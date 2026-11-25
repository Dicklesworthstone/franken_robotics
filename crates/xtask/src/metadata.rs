//! Declared-edge and resolved-graph `cargo metadata` readers for `xtask`
//! (beads N34p4-01 and BD-003).
//!
//! Invokes `cargo metadata` through `PATH` (never an absolute cargo path) with
//! `--locked --offline`, admits raw JSON bytes via `frob_core::json::admit`
//! (N34-17), and extracts workspace members, `[package.metadata.frob]` tables,
//! declared direct edges, resolved package `links` / build-deps, and the
//! resolved dependency graph. Compiles nothing and touches no network.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use frob_core::json::{Json, Limits, admit};

use crate::crate_map::{Layer, MAX_CORE_LAYER, ManifestMetadata, WorkspaceMember};

/// Exact repair hint mandated when an offline `cargo metadata` read needs the network.
pub const OFFLINE_FETCH_REPAIR: &str = "run `cargo fetch` once through the build shim";

/// Kind of Cargo workspace in the FrankenRobotics repository (shared with N34p4-02).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkspaceKind {
    Core,
    Annex,
    Link,
    Retarget,
}

impl WorkspaceKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Annex => "annex",
            Self::Link => "link",
            Self::Retarget => "retarget",
        }
    }
}

/// One workspace root passed to `xtask` checks (populated from `workspaces.rs` in N34p4-02).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRoot {
    pub name: String,
    /// Repository-relative directory (`""` for the repository root workspace).
    pub rel_dir: String,
    /// Filesystem directory containing this workspace's `Cargo.toml`.
    pub root_dir: PathBuf,
    pub kind: WorkspaceKind,
    /// Exempt thin-binary package names under `apps/` passed to `crate_map::validate`.
    pub link_packages: Vec<String>,
}

impl WorkspaceRoot {
    #[must_use]
    pub fn core(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            name: "core".to_owned(),
            rel_dir: String::new(),
            root_dir: repo_root.into(),
            kind: WorkspaceKind::Core,
            link_packages: Vec::new(),
        }
    }

    #[must_use]
    pub fn annex(
        name: impl Into<String>,
        rel_dir: impl Into<String>,
        root_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            name: name.into(),
            rel_dir: normalize_rel_str(&rel_dir.into()),
            root_dir: root_dir.into(),
            kind: WorkspaceKind::Annex,
            link_packages: Vec::new(),
        }
    }

    #[must_use]
    pub fn link(
        name: impl Into<String>,
        rel_dir: impl Into<String>,
        root_dir: impl Into<PathBuf>,
        link_packages: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            rel_dir: normalize_rel_str(&rel_dir.into()),
            root_dir: root_dir.into(),
            kind: WorkspaceKind::Link,
            link_packages,
        }
    }

    #[must_use]
    pub fn retarget(
        name: impl Into<String>,
        rel_dir: impl Into<String>,
        root_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            name: name.into(),
            rel_dir: normalize_rel_str(&rel_dir.into()),
            root_dir: root_dir.into(),
            kind: WorkspaceKind::Retarget,
            link_packages: Vec::new(),
        }
    }

    #[must_use]
    pub fn manifest_path(&self) -> PathBuf {
        self.root_dir.join("Cargo.toml")
    }

    #[must_use]
    pub fn has_manifest(&self) -> bool {
        self.manifest_path().is_file()
    }
}

/// Cargo dependency edge kind (`normal`, `dev`, or `build`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DepKind {
    Normal,
    Dev,
    Build,
}

impl DepKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Dev => "dev",
            Self::Build => "build",
        }
    }
}

impl fmt::Display for DepKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One declared direct dependency edge from a package manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredDep {
    /// Actual Cargo package name (even when renamed via `package = "..."`).
    pub name: String,
    pub rename: Option<String>,
    pub kind: DepKind,
    pub req: String,
    pub source: Option<String>,
    pub path: Option<PathBuf>,
    /// Repository-relative target directory when `path` lies inside `repo_root`.
    pub rel_path: Option<String>,
    /// Relative path from `repo_root` when `path` lies outside `repo_root` (e.g. `../frankensim/crates/fs-ga`).
    pub sibling_rel_path: Option<String>,
    pub uses_default_features: bool,
    pub features: Vec<String>,
    pub optional: bool,
    pub target: Option<String>,
}

/// One workspace member package loaded from `cargo metadata --no-deps`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageMeta {
    pub id: String,
    pub name: String,
    pub manifest_path: PathBuf,
    /// Package directory relative to `repo_root` (POSIX slashes, no leading `./`).
    pub rel_dir: String,
    /// Manifest path relative to `repo_root` (e.g. `crates/frob-core/Cargo.toml`).
    pub rel_manifest: String,
    pub frob_metadata: Option<Result<ManifestMetadata, String>>,
    pub dependencies: Vec<DeclaredDep>,
}

impl PackageMeta {
    #[must_use]
    pub fn crate_dir(&self) -> &Path {
        self.manifest_path.parent().unwrap_or(Path::new("."))
    }

    #[must_use]
    pub fn to_workspace_member(&self) -> WorkspaceMember {
        WorkspaceMember {
            name: self.name.clone(),
            rel_dir: self.rel_dir.clone(),
            manifest_path: self.manifest_path.clone(),
        }
    }
}

/// Declared workspace metadata read by one `--no-deps --locked --offline` invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredWorkspace {
    pub workspace_root: PathBuf,
    pub members: Vec<PackageMeta>,
}

/// One package in the resolved `cargo metadata --locked --offline` closure (BD-003).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub source: Option<String>,
    pub links: Option<String>,
    pub manifest_path: PathBuf,
    pub rel_dir: Option<String>,
    pub sibling_rel_path: Option<String>,
    pub dependencies: Vec<DeclaredDep>,
}

impl ResolvedPackage {
    #[must_use]
    pub fn is_registry(&self) -> bool {
        self.source
            .as_deref()
            .is_some_and(|s| s.starts_with("registry+"))
    }
}

/// One resolved dependency edge on a [`ResolveNode`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveDep {
    pub pkg: String,
    pub name: String,
    pub dep_kinds: Vec<DepKind>,
}

/// One node in the `resolve.nodes` graph of `cargo metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveNode {
    pub id: String,
    pub dependencies: Vec<String>,
    pub deps: Vec<ResolveDep>,
    pub features: Vec<String>,
}

/// Resolved workspace metadata read by one `--locked --offline` invocation (BD-003).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedWorkspace {
    pub workspace_root: PathBuf,
    pub workspace_members: BTreeSet<String>,
    pub packages: BTreeMap<String, ResolvedPackage>,
    pub nodes: BTreeMap<String, ResolveNode>,
}

/// Failure loading or parsing `cargo metadata`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetadataLoadError {
    MissingManifest(PathBuf),
    OfflineNeedsFetch { detail: String },
    CargoFailed { detail: String },
    SpawnFailed(String),
    JsonAdmission(String),
    Schema(String),
}

impl MetadataLoadError {
    /// True when the error should exit 3 (usage/offline-cache error) rather than a tool fault.
    #[must_use]
    pub const fn is_usage_error(&self) -> bool {
        matches!(self, Self::MissingManifest(_) | Self::OfflineNeedsFetch { .. })
    }

    #[must_use]
    pub fn repair(&self) -> Option<&'static str> {
        match self {
            Self::OfflineNeedsFetch { .. } => Some(OFFLINE_FETCH_REPAIR),
            _ => None,
        }
    }
}

impl fmt::Display for MetadataLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingManifest(p) => write!(f, "workspace manifest not found: {}", p.display()),
            Self::OfflineNeedsFetch { detail } => {
                write!(f, "cargo metadata requires network/cache ({OFFLINE_FETCH_REPAIR}): {detail}")
            }
            Self::CargoFailed { detail } => write!(f, "cargo metadata failed: {detail}"),
            Self::SpawnFailed(msg) => write!(f, "failed to invoke `cargo metadata`: {msg}"),
            Self::JsonAdmission(msg) => write!(f, "cargo metadata JSON admission failed: {msg}"),
            Self::Schema(msg) => write!(f, "unexpected cargo metadata JSON shape: {msg}"),
        }
    }
}

impl std::error::Error for MetadataLoadError {}

/// Run `cargo metadata --format-version 1 --no-deps --locked --offline` for `ws`
/// and parse the result through `frob_core::json::admit`.
pub fn load_declared_metadata(
    repo_root: &Path,
    ws: &WorkspaceRoot,
) -> Result<DeclaredWorkspace, MetadataLoadError> {
    let stdout = run_cargo_metadata(ws, true, false)?;
    parse_declared_metadata_json(repo_root, &stdout)
}

/// Run `cargo metadata --format-version 1 --locked --offline` (resolved closure) for `ws`
/// and parse the result through `frob_core::json::admit`.
pub fn load_resolved_metadata(
    repo_root: &Path,
    ws: &WorkspaceRoot,
) -> Result<ResolvedWorkspace, MetadataLoadError> {
    let stdout = run_cargo_metadata(ws, false, false)?;
    parse_resolved_metadata_json(repo_root, &stdout)
}

/// Run `cargo metadata --format-version 1 --all-features --locked --offline`
/// (resolved closure with every optional feature group enabled) for `ws`
/// and parse the result through `frob_core::json::admit` (N33c-09).
pub fn load_resolved_metadata_all_features(
    repo_root: &Path,
    ws: &WorkspaceRoot,
) -> Result<ResolvedWorkspace, MetadataLoadError> {
    let stdout = run_cargo_metadata(ws, false, true)?;
    parse_resolved_metadata_json(repo_root, &stdout)
}

fn run_cargo_metadata(
    ws: &WorkspaceRoot,
    no_deps: bool,
    all_features: bool,
) -> Result<Vec<u8>, MetadataLoadError> {
    let manifest_path = ws.manifest_path();
    if !manifest_path.is_file() {
        return Err(MetadataLoadError::MissingManifest(manifest_path));
    }

    let mut cmd = Command::new("cargo");
    cmd.current_dir(&ws.root_dir);
    cmd.args(["metadata", "--format-version", "1"]);
    if no_deps {
        cmd.arg("--no-deps");
    }
    if all_features {
        cmd.arg("--all-features");
    }
    cmd.args(["--locked", "--offline", "--manifest-path"])
        .arg(&manifest_path);

    let output = cmd
        .output()
        .map_err(|e| MetadataLoadError::SpawnFailed(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        if looks_like_offline_or_network_error(&stderr) {
            return Err(MetadataLoadError::OfflineNeedsFetch { detail: stderr });
        }
        return Err(MetadataLoadError::CargoFailed { detail: stderr });
    }

    Ok(output.stdout)
}

/// Detect Cargo stderr diagnostics indicating an offline/network cache miss or missing lockfile.
#[must_use]
pub fn looks_like_offline_or_network_error(stderr: &str) -> bool {
    let lower = stderr.to_ascii_lowercase();
    let needles = [
        "without network",
        "--offline",
        "offline mode",
        "not found in cache",
        "attempting to make an http",
        "failed to download",
        "could not resolve host",
        "no matching package named",
        "failed to select a version",
        "failed to get `",
        "lock file",
        "cannot create the lock file",
        "needs to be updated but --locked was passed",
        "cannot update the lock file",
        "unable to update",
    ];
    needles.iter().any(|n| lower.contains(n))
}

/// Parse raw `cargo metadata --format-version 1 --no-deps` JSON bytes through `frob_core::json::admit`.
pub fn parse_declared_metadata_json(
    repo_root: &Path,
    raw_json: &[u8],
) -> Result<DeclaredWorkspace, MetadataLoadError> {
    let admitted = admit(raw_json, &Limits::default())
        .map_err(|e| MetadataLoadError::JsonAdmission(e.to_string()))?;
    let root_obj = admitted.value();

    let ws_root_str = root_obj
        .get("workspace_root")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MetadataLoadError::Schema("missing string `workspace_root`".to_owned()))?;
    let workspace_root = PathBuf::from(ws_root_str);

    let member_ids = parse_workspace_member_ids(root_obj)?;

    let packages_arr = root_obj
        .get("packages")
        .and_then(|v| v.as_array())
        .ok_or_else(|| MetadataLoadError::Schema("missing array `packages`".to_owned()))?;

    let canon_repo = canonical_or_clean(repo_root);
    let mut members = Vec::new();
    for pkg_node in packages_arr {
        let id = pkg_node
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MetadataLoadError::Schema("package missing string `id`".to_owned()))?;
        if !member_ids.contains(id) {
            continue;
        }
        let name = pkg_node
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MetadataLoadError::Schema("package missing string `name`".to_owned()))?
            .to_owned();
        let manifest_str = pkg_node
            .get("manifest_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MetadataLoadError::Schema("package missing string `manifest_path`".to_owned()))?;
        let manifest_path = PathBuf::from(manifest_str);
        let pkg_dir = manifest_path.parent().unwrap_or(Path::new("."));
        let rel_dir = relative_dir_inside(&canon_repo, pkg_dir)
            .unwrap_or_else(|| normalize_rel_path(pkg_dir));
        let rel_manifest = if rel_dir.is_empty() {
            "Cargo.toml".to_owned()
        } else {
            format!("{rel_dir}/Cargo.toml")
        };

        let frob_metadata = parse_frob_metadata_json(pkg_node.get("metadata").map(|s| &s.value));
        let dependencies = parse_deps_array(&canon_repo, pkg_node)?;

        members.push(PackageMeta {
            id: id.to_owned(),
            name,
            manifest_path,
            rel_dir,
            rel_manifest,
            frob_metadata,
            dependencies,
        });
    }

    members.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.rel_dir.cmp(&b.rel_dir)));
    Ok(DeclaredWorkspace {
        workspace_root,
        members,
    })
}

/// Parse raw `cargo metadata --format-version 1` (resolved) JSON bytes through `frob_core::json::admit`.
pub fn parse_resolved_metadata_json(
    repo_root: &Path,
    raw_json: &[u8],
) -> Result<ResolvedWorkspace, MetadataLoadError> {
    let admitted = admit(raw_json, &Limits::default())
        .map_err(|e| MetadataLoadError::JsonAdmission(e.to_string()))?;
    let root_obj = admitted.value();

    let ws_root_str = root_obj
        .get("workspace_root")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MetadataLoadError::Schema("missing string `workspace_root`".to_owned()))?;
    let workspace_root = PathBuf::from(ws_root_str);

    let workspace_members = parse_workspace_member_ids(root_obj)?;
    let packages_arr = root_obj
        .get("packages")
        .and_then(|v| v.as_array())
        .ok_or_else(|| MetadataLoadError::Schema("missing array `packages`".to_owned()))?;

    let canon_repo = canonical_or_clean(repo_root);
    let mut packages = BTreeMap::new();
    for pkg_node in packages_arr {
        let id = pkg_node
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MetadataLoadError::Schema("package missing string `id`".to_owned()))?
            .to_owned();
        let name = pkg_node
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MetadataLoadError::Schema("package missing string `name`".to_owned()))?
            .to_owned();
        let version = pkg_node
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();
        let source = pkg_node
            .get("source")
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        let links = pkg_node
            .get("links")
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        let manifest_str = pkg_node
            .get("manifest_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MetadataLoadError::Schema("package missing string `manifest_path`".to_owned()))?;
        let manifest_path = PathBuf::from(manifest_str);
        let pkg_dir = manifest_path.parent().unwrap_or(Path::new("."));
        let rel_dir = relative_dir_inside(&canon_repo, pkg_dir);
        let sibling_rel_path = if rel_dir.is_none() && source.is_none() {
            sibling_relative_path(&canon_repo, pkg_dir)
        } else {
            None
        };
        let dependencies = parse_deps_array(&canon_repo, pkg_node)?;

        packages.insert(
            id.clone(),
            ResolvedPackage {
                id,
                name,
                version,
                source,
                links,
                manifest_path,
                rel_dir,
                sibling_rel_path,
                dependencies,
            },
        );
    }

    let mut nodes = BTreeMap::new();
    if let Some(resolve_obj) = root_obj.get("resolve")
        && !resolve_obj.is_null()
        && let Some(nodes_arr) = resolve_obj.get("nodes").and_then(|v| v.as_array())
    {
        for node in nodes_arr {
            let id = node
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| MetadataLoadError::Schema("resolve node missing `id`".to_owned()))?
                .to_owned();
            let mut dependencies = Vec::new();
            if let Some(arr) = node.get("dependencies").and_then(|v| v.as_array()) {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        dependencies.push(s.to_owned());
                    }
                }
            }
            let mut deps = Vec::new();
            if let Some(arr) = node.get("deps").and_then(|v| v.as_array()) {
                for dep in arr {
                    let pkg = dep
                        .get("pkg")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            MetadataLoadError::Schema("resolve dep missing `pkg`".to_owned())
                        })?
                        .to_owned();
                    let name = dep
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_owned();
                    let mut dep_kinds = Vec::new();
                    if let Some(kinds_arr) = dep.get("dep_kinds").and_then(|v| v.as_array()) {
                        for k in kinds_arr {
                            dep_kinds.push(parse_dep_kind_value(
                                k.get("kind").map(|v| &v.value),
                                &name,
                            )?);
                        }
                    }
                    if dep_kinds.is_empty() {
                        dep_kinds.push(DepKind::Normal);
                    }
                    deps.push(ResolveDep {
                        pkg,
                        name,
                        dep_kinds,
                    });
                }
            }
            let mut features = Vec::new();
            if let Some(arr) = node.get("features").and_then(|v| v.as_array()) {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        features.push(s.to_owned());
                    }
                }
            }
            nodes.insert(
                id.clone(),
                ResolveNode {
                    id,
                    dependencies,
                    deps,
                    features,
                },
            );
        }
    }

    Ok(ResolvedWorkspace {
        workspace_root,
        workspace_members,
        packages,
        nodes,
    })
}

fn parse_workspace_member_ids(root_obj: &Json<'_>) -> Result<BTreeSet<String>, MetadataLoadError> {
    let member_ids_arr = root_obj
        .get("workspace_members")
        .and_then(|v| v.as_array())
        .ok_or_else(|| MetadataLoadError::Schema("missing array `workspace_members`".to_owned()))?;
    let mut member_ids = BTreeSet::new();
    for item in member_ids_arr {
        let id = item
            .as_str()
            .ok_or_else(|| MetadataLoadError::Schema("non-string entry in `workspace_members`".to_owned()))?;
        member_ids.insert(id.to_owned());
    }
    Ok(member_ids)
}

fn parse_deps_array(
    canon_repo: &Path,
    pkg_node: &Json<'_>,
) -> Result<Vec<DeclaredDep>, MetadataLoadError> {
    let deps_arr = pkg_node
        .get("dependencies")
        .and_then(|v| v.as_array())
        .ok_or_else(|| MetadataLoadError::Schema("package missing array `dependencies`".to_owned()))?;
    let mut dependencies = Vec::with_capacity(deps_arr.len());
    for dep_node in deps_arr {
        dependencies.push(parse_dep_json(canon_repo, dep_node)?);
    }
    Ok(dependencies)
}

fn parse_dep_kind_value(
    kind_node: Option<&Json<'_>>,
    dep_name: &str,
) -> Result<DepKind, MetadataLoadError> {
    match kind_node {
        None | Some(Json::Null) => Ok(DepKind::Normal),
        Some(Json::Str(s)) if s == "dev" => Ok(DepKind::Dev),
        Some(Json::Str(s)) if s == "build" => Ok(DepKind::Build),
        Some(Json::Str(other)) => Err(MetadataLoadError::Schema(format!(
            "unknown dependency kind `{other}` for `{dep_name}`"
        ))),
        Some(_) => Err(MetadataLoadError::Schema(format!(
            "non-string dependency kind for `{dep_name}`"
        ))),
    }
}

fn parse_dep_json(canon_repo: &Path, dep_node: &Json<'_>) -> Result<DeclaredDep, MetadataLoadError> {
    let name = dep_node
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MetadataLoadError::Schema("dependency missing string `name`".to_owned()))?
        .to_owned();
    let rename = dep_node
        .get("rename")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let kind = parse_dep_kind_value(dep_node.get("kind").map(|v| &v.value), &name)?;
    let req = dep_node
        .get("req")
        .and_then(|v| v.as_str())
        .unwrap_or("*")
        .to_owned();
    let source = dep_node
        .get("source")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let path = dep_node
        .get("path")
        .and_then(|v| v.as_str())
        .map(PathBuf::from);
    let rel_path = path
        .as_deref()
        .and_then(|p| relative_dir_inside(canon_repo, p));
    let sibling_rel_path = if rel_path.is_none() {
        path.as_deref()
            .and_then(|p| sibling_relative_path(canon_repo, p))
    } else {
        None
    };
    let uses_default_features = dep_node
        .get("uses_default_features")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let optional = dep_node
        .get("optional")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let target = dep_node
        .get("target")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let mut features = Vec::new();
    if let Some(arr) = dep_node.get("features").and_then(|v| v.as_array()) {
        for item in arr {
            if let Some(s) = item.as_str() {
                features.push(s.to_owned());
            }
        }
    }

    Ok(DeclaredDep {
        name,
        rename,
        kind,
        req,
        source,
        path,
        rel_path,
        sibling_rel_path,
        uses_default_features,
        features,
        optional,
        target,
    })
}

fn parse_frob_metadata_json(
    metadata_node: Option<&Json<'_>>,
) -> Option<Result<ManifestMetadata, String>> {
    let frob = metadata_node?.get("frob")?;
    if frob.is_null() {
        return None;
    }
    let layer = match frob.get("layer") {
        None => return Some(Err("missing `layer` in [package.metadata.frob]".to_owned())),
        Some(sp) => match &sp.value {
            Json::Str(s) if s == "annex" => Layer::Annex,
            Json::Str(s) => {
                return Some(Err(format!(
                    "invalid `layer` string `{s}` in [package.metadata.frob]"
                )));
            }
            Json::Num(_) => match sp.as_int53() {
                Ok(n) if (0..=i64::from(MAX_CORE_LAYER)).contains(&n) => {
                    Layer::Core(u8::try_from(n).unwrap_or(0))
                }
                Ok(n) => {
                    return Some(Err(format!(
                        "core layer {n} out of range 0..={MAX_CORE_LAYER}"
                    )));
                }
                Err(e) => return Some(Err(format!("invalid `layer` integer: {e}"))),
            },
            _ => return Some(Err("`layer` must be integer 0..=7 or \"annex\"".to_owned())),
        },
    };
    let rank = match frob.get("rank") {
        None => return Some(Err("missing `rank` in [package.metadata.frob]".to_owned())),
        Some(sp) => match sp.as_int53() {
            Ok(n) => match u8::try_from(n) {
                Ok(r) => r,
                Err(_) => return Some(Err(format!("`rank` {n} out of u8 range"))),
            },
            Err(e) => return Some(Err(format!("invalid `rank` integer: {e}"))),
        },
    };
    Some(Ok(ManifestMetadata { layer, rank }))
}

/// Canonicalize `path` if it exists on disk; otherwise resolve its longest existing
/// ancestor and append remaining normalized components (handles macOS `/var` -> `/private/var`).
#[must_use]
pub fn canonical_or_clean(path: &Path) -> PathBuf {
    if let Ok(c) = fs::canonicalize(path) {
        return c;
    }
    let cleaned = lexical_clean(path);
    let mut cur = cleaned.as_path();
    let mut tail = Vec::new();
    while let Some(parent) = cur.parent() {
        if let Some(name) = cur.file_name() {
            tail.push(name.to_os_string());
        }
        if let Ok(mut canon) = fs::canonicalize(parent) {
            for seg in tail.into_iter().rev() {
                canon.push(seg);
            }
            return lexical_clean(&canon);
        }
        cur = parent;
    }
    cleaned
}

fn lexical_clean(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in path.components() {
        match comp {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    out.push(comp.as_os_str());
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Return POSIX-normalized relative path if `target` lies inside `canon_repo`.
#[must_use]
pub fn relative_dir_inside(canon_repo: &Path, target: &Path) -> Option<String> {
    let canon_target = canonical_or_clean(target);
    let rel = canon_target.strip_prefix(canon_repo).ok()?;
    Some(normalize_rel_path(rel))
}

/// Compute a POSIX relative path (`../<sibling>/...`) from `canon_repo` to `target`
/// when `target` lies outside `canon_repo`. Returns `None` if `target` lies inside `canon_repo`.
#[must_use]
pub fn sibling_relative_path(canon_repo: &Path, target: &Path) -> Option<String> {
    let canon_target = canonical_or_clean(target);
    if canon_target.strip_prefix(canon_repo).is_ok() {
        return None;
    }
    let repo_comps: Vec<Component<'_>> = canon_repo.components().collect();
    let target_comps: Vec<Component<'_>> = canon_target.components().collect();
    let mut common = 0usize;
    while common < repo_comps.len()
        && common < target_comps.len()
        && repo_comps[common] == target_comps[common]
    {
        common += 1;
    }
    if common == 0 {
        return None;
    }
    let mut parts: Vec<String> = Vec::new();
    for _ in common..repo_comps.len() {
        parts.push("..".to_owned());
    }
    for comp in &target_comps[common..] {
        if let Component::Normal(s) = comp {
            parts.push(s.to_string_lossy().into_owned());
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("/"))
    }
}

#[must_use]
pub fn normalize_rel_path(path: &Path) -> String {
    let raw = path.to_string_lossy().replace('\\', "/");
    normalize_rel_str(&raw)
}

#[must_use]
pub fn normalize_rel_str(raw: &str) -> String {
    raw.trim_start_matches("./").trim_end_matches('/').to_owned()
}
