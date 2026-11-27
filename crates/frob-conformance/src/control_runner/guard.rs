#![deny(unsafe_code)]

//! Mutation-feature guard (`N38b-04`, `tests/mutation_guard.rs`).
//!
//! Enforces across workspace crates and `controls.toml`:
//! 1. Every mutation feature named by a control in `controls.toml` (whatever its
//!    spelling: `cx-262`, `nc-<nnnn>`, etc.) is declared in its owning crate's
//!    `Cargo.toml` and implies that crate's `conformance` feature.
//! 2. No mutation feature (and no `conformance` feature) is enabled by `[features].default`.
//! 3. Every crate that declares mutation features implying `conformance` (and, under
//!    strict check, every crate declaring `conformance`) carries a release-profile
//!    `compile_error!` guard (`not(debug_assertions)`).
//! 4. Product binaries (`frob`, `frobd`, `frob-mcp`, `frob-verify`) carry neither a
//!    `conformance` feature nor any mutation feature.
//! 5. A release-profile build with a mutation feature enabled fails to compile.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use frob_core::registry::toml::{self, Value as TomlValue};
use frob_core::testkit::registry::{ControlKind, Controls};

use super::detect_annex_workspace;
use super::mutation::resolve_crate_and_feature;

/// Product binaries forbidden from carrying `conformance` or any mutation feature (`N38b-04`).
pub const PRODUCT_BINARIES: &[&str] = &["frob", "frobd", "frob-mcp", "frob-verify"];

/// One violation reported by [`check_mutation_guard`], always naming both crate and feature.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GuardViolation {
    pub crate_name: String,
    pub feature: String,
    pub rule: String,
    pub detail: String,
}

impl fmt::Display for GuardViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] crate `{}` feature `{}`: {}",
            self.rule, self.crate_name, self.feature, self.detail
        )
    }
}

/// Summary report returned by [`check_mutation_guard`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationGuardReport {
    pub controls_checked: usize,
    pub crates_checked: usize,
    pub product_binaries_checked: usize,
    pub violations: Vec<GuardViolation>,
}

impl MutationGuardReport {
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.violations.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedDep {
    /// Dependency key in `[dependencies]` / `[build-dependencies]` / `[target.<cfg>.*]`.
    key: String,
    /// Actual package name (`package = "..."` when renamed, otherwise `key`).
    package: String,
    /// Whether Cargo enables the dependency's `default` feature (default `true`).
    default_features: bool,
    /// Whether the dependency is optional (default `false`).
    optional: bool,
    /// Explicit features enabled on this dependency (`features = [...]`).
    features: Vec<String>,
}

#[derive(Debug, Clone)]
struct ParsedCrateManifest {
    name: String,
    dir: PathBuf,
    features: BTreeMap<String, Vec<String>>,
    bins: Vec<String>,
    deps: Vec<ParsedDep>,
}

/// Audit all mutation controls, crate `[features]` tables, release-profile
/// `compile_error!` guards, and product binaries under `workspace_root`.
///
/// When `strict_all_conformance_crates` is `true`, **every** crate declaring a
/// `conformance` feature must carry a release-profile `compile_error!` guard.
/// When `false`, crates with mutation controls or features implying `conformance`
/// (and `frob-conformance` itself) must carry the guard while pure admission-probe
/// crates with no mutation features implying `conformance` are exempted.
pub fn check_mutation_guard(
    workspace_root: &Path,
    strict_all_conformance_crates: bool,
) -> Result<MutationGuardReport, String> {
    let (controls, _) = Controls::load(workspace_root)
        .map_err(|e| format!("load controls.toml for mutation_guard: {e}"))?;

    let manifests = load_workspace_manifests(workspace_root)?;
    let by_name: BTreeMap<String, &ParsedCrateManifest> =
        manifests.iter().map(|m| (m.name.clone(), m)).collect();

    let mut violations: Vec<GuardViolation> = Vec::new();
    let mut controls_checked = 0usize;
    let mut control_crate_features: BTreeSet<(String, String)> = BTreeSet::new();
    let mut control_feature_names: BTreeSet<String> = BTreeSet::new();
    let mut conformance_mutation_features: BTreeSet<(String, String)> = BTreeSet::new();
    let mut all_mutation_features: BTreeSet<String> = BTreeSet::new();

    // 1. Check every `mutation` control in `controls.toml`.
    for ctrl in controls.all() {
        if ctrl.kind != ControlKind::Mutation {
            continue;
        }
        if detect_annex_workspace(&ctrl.target).is_some() {
            // Annex workspace features are enforced by the annex workspace's own lint (N21c-30).
            continue;
        }
        if let Some(raw_feat) = ctrl.feature.as_deref().map(str::trim)
            && !raw_feat.is_empty()
        {
            control_feature_names.insert(raw_feat.to_owned());
            all_mutation_features.insert(raw_feat.to_owned());
        }
        let Ok((crate_name, feat_name)) = resolve_crate_and_feature(ctrl) else {
            // Unbound placeholder targets (e.g. `N24b-01:...`) belong to future milestones.
            continue;
        };
        control_crate_features.insert((crate_name.clone(), feat_name.clone()));
        control_feature_names.insert(feat_name.clone());
        all_mutation_features.insert(feat_name.clone());

        let Some(manifest) = by_name.get(&crate_name) else {
            // Owning crate does not exist yet on disk (future milestone M1+ crate).
            continue;
        };
        controls_checked += 1;

        if !manifest.features.contains_key(&feat_name) {
            violations.push(GuardViolation {
                crate_name: crate_name.clone(),
                feature: feat_name.clone(),
                rule: "feature-missing".to_owned(),
                detail: format!(
                    "control `{}` names feature `{feat_name}` which is not declared in `{}/Cargo.toml`",
                    ctrl.id,
                    manifest.dir.display()
                ),
            });
            continue;
        }

        // Feature must transitively imply `conformance` within its declaring crate.
        let closure = local_feature_closure(&manifest.features, &feat_name);
        if !closure.contains("conformance") {
            violations.push(GuardViolation {
                crate_name: crate_name.clone(),
                feature: feat_name.clone(),
                rule: "missing-conformance-implication".to_owned(),
                detail: format!(
                    "mutation feature `{feat_name}` (control `{}`) must imply `conformance`",
                    ctrl.id
                ),
            });
        }

        // Feature must not be enabled by `default`.
        let default_closure = local_feature_closure(&manifest.features, "default");
        if default_closure.contains(&feat_name) {
            violations.push(GuardViolation {
                crate_name: crate_name.clone(),
                feature: feat_name.clone(),
                rule: "mutation-in-default".to_owned(),
                detail: format!(
                    "mutation feature `{feat_name}` (control `{}`) must not be enabled by `default`",
                    ctrl.id
                ),
            });
        }

        // Owning crate must carry the release-profile compile_error! guard.
        if !crate_has_release_guard(&manifest.dir) {
            violations.push(GuardViolation {
                crate_name: crate_name.clone(),
                feature: feat_name.clone(),
                rule: "missing-release-guard".to_owned(),
                detail: format!(
                    "crate `{crate_name}` declares mutation feature `{feat_name}` without a release-profile `compile_error!` guard"
                ),
            });
        }
    }

    // 2. Audit every workspace crate declaring `conformance` or mutation features implying it,
    //    and collect the complete workspace-wide mutation feature set before auditing product binaries.
    for manifest in &manifests {
        let default_closure = local_feature_closure(&manifest.features, "default");
        if manifest.features.contains_key("conformance") {
            if default_closure.contains("conformance") {
                violations.push(GuardViolation {
                    crate_name: manifest.name.clone(),
                    feature: "conformance".to_owned(),
                    rule: "conformance-in-default".to_owned(),
                    detail: format!(
                        "crate `{}` enables `conformance` in `[features].default`",
                        manifest.name
                    ),
                });
            }

            let implying_conformance: Vec<&String> = manifest
                .features
                .keys()
                .filter(|k| {
                    k.as_str() != "conformance"
                        && k.as_str() != "default"
                        && local_feature_closure(&manifest.features, k).contains("conformance")
                })
                .collect();

            for mf in &implying_conformance {
                conformance_mutation_features.insert((manifest.name.clone(), (*mf).clone()));
                all_mutation_features.insert((*mf).clone());
                if default_closure.contains(mf.as_str()) {
                    violations.push(GuardViolation {
                        crate_name: manifest.name.clone(),
                        feature: (*mf).clone(),
                        rule: "mutation-in-default".to_owned(),
                        detail: format!(
                            "crate `{}` enables mutation feature `{mf}` in `[features].default`",
                            manifest.name
                        ),
                    });
                }
            }

            let needs_guard = strict_all_conformance_crates
                || !implying_conformance.is_empty()
                || manifest.name == "frob-conformance";
            if needs_guard && !crate_has_release_guard(&manifest.dir) {
                violations.push(GuardViolation {
                    crate_name: manifest.name.clone(),
                    feature: "conformance".to_owned(),
                    rule: "missing-release-guard".to_owned(),
                    detail: format!(
                        "crate `{}` declares `conformance` without `#[cfg(all(feature = \"conformance\", not(debug_assertions)))] compile_error!(...)`",
                        manifest.name
                    ),
                });
            }
        }
    }

    // 3. Check product binaries (`frob`, `frobd`, `frob-mcp`, `frob-verify`):
    //    resolve the features each product binary actually builds with (its default
    //    features and transitive dependency features) and require that its build carries
    //    neither `conformance` nor any control-named or conformance-implying mutation feature.
    let mut product_binaries_checked = 0usize;
    for manifest in &manifests {
        let matched_bins: Vec<&str> = manifest
            .bins
            .iter()
            .map(String::as_str)
            .filter(|b| PRODUCT_BINARIES.contains(b))
            .collect();
        if !matched_bins.is_empty() || PRODUCT_BINARIES.contains(&manifest.name.as_str()) {
            product_binaries_checked += matched_bins.len().max(1);
            let resolved = resolve_default_build_features(manifest, &by_name);
            for (owner_crate, feat) in &resolved {
                let display_feat = if owner_crate == &manifest.name {
                    feat.clone()
                } else {
                    format!("{owner_crate}/{feat}")
                };
                if feat == "conformance" {
                    violations.push(GuardViolation {
                        crate_name: manifest.name.clone(),
                        feature: display_feat,
                        rule: "product-binary-conformance".to_owned(),
                        detail: format!(
                            "product binary crate `{}` default build carries `conformance` (from `{owner_crate}`)",
                            manifest.name
                        ),
                    });
                } else {
                    let is_mutation = control_crate_features
                        .contains(&(owner_crate.clone(), feat.clone()))
                        || conformance_mutation_features
                            .contains(&(owner_crate.clone(), feat.clone()))
                        || control_feature_names.contains(feat.as_str())
                        || (!by_name.contains_key(owner_crate)
                            && all_mutation_features.contains(feat.as_str()));
                    if is_mutation {
                        violations.push(GuardViolation {
                            crate_name: manifest.name.clone(),
                            feature: display_feat,
                            rule: "product-binary-mutation-feature".to_owned(),
                            detail: format!(
                                "product binary crate `{}` default build carries mutation feature `{feat}` (from `{owner_crate}`)",
                                manifest.name
                            ),
                        });
                    }
                }
            }
        }
    }

    violations.sort();
    violations.dedup();
    Ok(MutationGuardReport {
        controls_checked,
        crates_checked: manifests.len(),
        product_binaries_checked,
        violations,
    })
}

/// Compile-fail verification: build `package` under `--release --features <feature>`
/// via PATH `cargo` (isolated `CARGO_TARGET_DIR`) and require compilation to fail with
/// a `compile_error!` diagnostic.
pub fn verify_release_build_fails(
    manifest_path: &Path,
    package: Option<&str>,
    feature: &str,
    target_dir: &Path,
    offline: bool,
) -> Result<String, String> {
    let mut cmd = Command::new("cargo");
    cmd.arg("check")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--release")
        .arg("--features")
        .arg(feature);
    if let Some(pkg) = package {
        cmd.arg("-p").arg(pkg);
    }
    if offline {
        cmd.arg("--offline");
    }
    cmd.env("CARGO_TARGET_DIR", target_dir)
        .env_remove("CARGO_BUILD_BUILD_DIR");

    let output = cmd
        .output()
        .map_err(|e| format!("failed to invoke cargo for release compile-fail check: {e}"))?;
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    if output.status.success() {
        return Err(format!(
            "release build with `--features {feature}` unexpectedly succeeded (must fail via compile_error!)"
        ));
    }
    if !stderr.contains("error:") {
        return Err(format!(
            "release build failed without rustc error diagnostic:\n{stderr}"
        ));
    }
    Ok(stderr)
}

/// Local transitive feature closure within a single crate's `[features]` table.
///
/// Only follows local feature edges (omitting `dep:<key>`, `<key>/<feat>`, and `<key>?/<feat>`),
/// so forwarding a dependency's feature cannot be mistaken for implying a same-named local feature.
fn local_feature_closure(features: &BTreeMap<String, Vec<String>>, root: &str) -> BTreeSet<String> {
    let mut visited = BTreeSet::new();
    if !features.contains_key(root) {
        return visited;
    }
    let mut stack = vec![root.to_owned()];
    while let Some(cur) = stack.pop() {
        if !visited.insert(cur.clone()) {
            continue;
        }
        if let Some(children) = features.get(&cur) {
            for ch in children {
                if ch.contains('/') || ch.starts_with("dep:") {
                    continue;
                }
                if !visited.contains(ch.as_str()) {
                    stack.push(ch.clone());
                }
            }
        }
    }
    visited
}

/// Resolve the `(crate_name, feature_name)` set enabled by a product binary crate's
/// default build across workspace manifests, matching Cargo feature resolution:
/// `default`, `default-features = false`, `optional = true`, `package = "..."` renames,
/// `dep:<key>`, `<key>/<feat>`, weak `<key>?/<feat>`, and implicit optional-dep features.
fn resolve_default_build_features(
    root: &ParsedCrateManifest,
    by_name: &BTreeMap<String, &ParsedCrateManifest>,
) -> BTreeSet<(String, String)> {
    let mut resolver = FeatureResolver {
        by_name,
        active_crates: BTreeSet::new(),
        active_dep_keys: BTreeSet::new(),
        enabled_features: BTreeSet::new(),
        pending_weak: BTreeMap::new(),
        crate_queue: Vec::new(),
        feat_queue: Vec::new(),
    };
    resolver.activate_crate(&root.name, true);
    resolver.drain();
    resolver.enabled_features
}

struct FeatureResolver<'a> {
    by_name: &'a BTreeMap<String, &'a ParsedCrateManifest>,
    active_crates: BTreeSet<String>,
    active_dep_keys: BTreeSet<(String, String)>,
    enabled_features: BTreeSet<(String, String)>,
    pending_weak: BTreeMap<(String, String), BTreeSet<String>>,
    crate_queue: Vec<String>,
    feat_queue: Vec<(String, String)>,
}

impl FeatureResolver<'_> {
    fn enable_feature(&mut self, crate_name: &str, feat: &str) {
        let pair = (crate_name.to_owned(), feat.to_owned());
        if self.enabled_features.insert(pair.clone()) {
            self.feat_queue.push(pair);
        }
    }

    fn activate_crate(&mut self, crate_name: &str, with_default: bool) {
        if self.active_crates.insert(crate_name.to_owned()) {
            self.crate_queue.push(crate_name.to_owned());
        }
        if with_default
            && let Some(m) = self.by_name.get(crate_name)
            && m.features.contains_key("default")
        {
            self.enable_feature(crate_name, "default");
        }
    }

    fn activate_dep_key(&mut self, parent: &str, dep_key: &str) {
        let key_pair = (parent.to_owned(), dep_key.to_owned());
        if !self.active_dep_keys.insert(key_pair.clone()) {
            return;
        }
        let Some(parent_manifest) = self.by_name.get(parent) else {
            return;
        };
        let matching: Vec<&ParsedDep> = parent_manifest
            .deps
            .iter()
            .filter(|d| d.key == dep_key)
            .collect();
        for dep in &matching {
            self.activate_crate(&dep.package, dep.default_features);
            for f in &dep.features {
                self.enable_feature(&dep.package, f);
            }
        }
        if let Some(weak_feats) = self.pending_weak.get(&key_pair).cloned() {
            for dep in &matching {
                for wf in &weak_feats {
                    self.enable_feature(&dep.package, wf);
                }
            }
        }
    }

    fn drain(&mut self) {
        while !self.crate_queue.is_empty() || !self.feat_queue.is_empty() {
            while let Some(cname) = self.crate_queue.pop() {
                if let Some(manifest) = self.by_name.get(&cname).copied() {
                    for dep in &manifest.deps {
                        if !dep.optional {
                            self.activate_dep_key(&cname, &dep.key);
                        }
                    }
                }
            }

            while let Some((cname, fname)) = self.feat_queue.pop() {
                let Some(manifest) = self.by_name.get(&cname).copied() else {
                    continue;
                };
                let Some(items) = manifest.features.get(&fname) else {
                    continue;
                };
                for item in items {
                    if let Some(dep_key) = item.strip_prefix("dep:") {
                        self.activate_dep_key(&cname, dep_key);
                    } else if let Some((dep_key, dep_feat)) = item.split_once("?/") {
                        let key_pair = (cname.clone(), dep_key.to_owned());
                        self.pending_weak
                            .entry(key_pair.clone())
                            .or_default()
                            .insert(dep_feat.to_owned());
                        if self.active_dep_keys.contains(&key_pair) {
                            for dep in manifest.deps.iter().filter(|d| d.key == dep_key) {
                                self.enable_feature(&dep.package, dep_feat);
                            }
                        }
                    } else if let Some((dep_key, dep_feat)) = item.split_once('/') {
                        self.activate_dep_key(&cname, dep_key);
                        let mut matched_any = false;
                        for dep in manifest.deps.iter().filter(|d| d.key == dep_key) {
                            matched_any = true;
                            self.enable_feature(&dep.package, dep_feat);
                        }
                        if !matched_any {
                            self.activate_crate(dep_key, false);
                            self.enable_feature(dep_key, dep_feat);
                        }
                    } else if manifest.features.contains_key(item) {
                        self.enable_feature(&cname, item);
                    } else if manifest.deps.iter().any(|d| d.optional && d.key == *item) {
                        self.activate_dep_key(&cname, item);
                    } else {
                        self.enable_feature(&cname, item);
                    }
                }
            }
        }
    }
}

fn crate_has_release_guard(crate_dir: &Path) -> bool {
    let src_dir = crate_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    for path in files {
        let Ok(src) = fs::read_to_string(&path) else {
            continue;
        };
        if src.contains("compile_error!")
            && src.contains("not(debug_assertions)")
            && (src.contains("feature = \"conformance\"")
                || src.contains("feature = \"gauntlet-mutations\""))
        {
            return true;
        }
    }
    false
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect_rs_files(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

fn load_workspace_manifests(workspace_root: &Path) -> Result<Vec<ParsedCrateManifest>, String> {
    let mut out = Vec::new();
    let crates_dir = workspace_root.join("crates");
    if crates_dir.is_dir() {
        let rd = fs::read_dir(&crates_dir)
            .map_err(|e| format!("read `{}`: {e}", crates_dir.display()))?;
        let mut dirs: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
        dirs.sort();
        for dir in dirs {
            let mpath = dir.join("Cargo.toml");
            if mpath.is_file() {
                out.push(parse_crate_manifest(&mpath, &dir)?);
            }
        }
    }
    // Also support single-crate scratch workspaces whose Cargo.toml sits at workspace_root.
    let root_m = workspace_root.join("Cargo.toml");
    if out.is_empty() && root_m.is_file() {
        let raw =
            fs::read_to_string(&root_m).map_err(|e| format!("read `{}`: {e}", root_m.display()))?;
        if raw.contains("[package]") {
            out.push(parse_crate_manifest(&root_m, workspace_root)?);
        }
    }
    Ok(out)
}

fn parse_crate_manifest(
    manifest_path: &Path,
    crate_dir: &Path,
) -> Result<ParsedCrateManifest, String> {
    let raw = fs::read_to_string(manifest_path)
        .map_err(|e| format!("read `{}`: {e}", manifest_path.display()))?;
    let parsed = toml::parse(raw.as_bytes()).ok();

    if let Some(tbl) = parsed {
        let name = tbl
            .get("package")
            .and_then(TomlValue::as_table)
            .and_then(|p| p.get("name"))
            .and_then(TomlValue::as_str)
            .map(str::to_owned)
            .unwrap_or_else(|| {
                crate_dir
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_owned()
            });

        let mut features = BTreeMap::new();
        if let Some(ftbl) = tbl.get("features").and_then(TomlValue::as_table) {
            for (k, v) in ftbl.iter() {
                let items = v
                    .value
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|sp| sp.value.as_str().map(str::to_owned))
                            .collect()
                    })
                    .unwrap_or_default();
                features.insert(k.to_owned(), items);
            }
        }

        let autobins = tbl
            .get("package")
            .and_then(TomlValue::as_table)
            .and_then(|p| p.get("autobins"))
            .and_then(TomlValue::as_bool)
            .unwrap_or(true);

        let mut bins = Vec::new();
        if let Some(barr) = tbl.get("bin").and_then(TomlValue::as_array) {
            for sp in barr {
                if let Some(bname) = sp
                    .value
                    .as_table()
                    .and_then(|t| t.get("name"))
                    .and_then(TomlValue::as_str)
                {
                    bins.push(bname.to_owned());
                }
            }
        }
        if autobins {
            collect_autobins(crate_dir, &mut bins);
        }

        let mut deps = Vec::new();
        for dep_sec in ["dependencies", "build-dependencies"] {
            if let Some(dtbl) = tbl.get(dep_sec).and_then(TomlValue::as_table) {
                collect_dep_table(dtbl, &mut deps);
            }
        }
        if let Some(targets) = tbl.get("target").and_then(TomlValue::as_table) {
            for (_, tval) in targets.iter() {
                if let Some(ttbl) = tval.value.as_table() {
                    for dep_sec in ["dependencies", "build-dependencies"] {
                        if let Some(dtbl) = ttbl.get(dep_sec).and_then(TomlValue::as_table) {
                            collect_dep_table(dtbl, &mut deps);
                        }
                    }
                }
            }
        }

        return Ok(ParsedCrateManifest {
            name,
            dir: crate_dir.to_path_buf(),
            features,
            bins,
            deps,
        });
    }

    // Fallback line parser for Cargo.toml files that use full TOML constructs (e.g. dotted keys)
    // outside the strict registry TOML subset.
    Ok(parse_crate_manifest_fallback(&raw, crate_dir))
}

fn collect_dep_table(dtbl: &toml::Table, deps: &mut Vec<ParsedDep>) {
    for (dep_name, dep_val) in dtbl.iter() {
        let mut dep = ParsedDep {
            key: dep_name.to_owned(),
            package: dep_name.to_owned(),
            default_features: true,
            optional: false,
            features: Vec::new(),
        };
        if let Some(inner) = dep_val.value.as_table() {
            if let Some(pkg) = inner.get("package").and_then(TomlValue::as_str) {
                dep.package = pkg.to_owned();
            }
            if let Some(df) = inner
                .get("default-features")
                .or_else(|| inner.get("default_features"))
                .and_then(TomlValue::as_bool)
            {
                dep.default_features = df;
            }
            if let Some(opt) = inner.get("optional").and_then(TomlValue::as_bool) {
                dep.optional = opt;
            }
            if let Some(farr) = inner.get("features").and_then(TomlValue::as_array) {
                dep.features = farr
                    .iter()
                    .filter_map(|sp| sp.value.as_str().map(str::to_owned))
                    .collect();
            }
        }
        deps.push(dep);
    }
}

fn collect_autobins(crate_dir: &Path, bins: &mut Vec<String>) {
    let bin_dir = crate_dir.join("src/bin");
    let Ok(rd) = fs::read_dir(&bin_dir) else {
        return;
    };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let discovered = if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("rs") {
            p.file_stem().and_then(|s| s.to_str()).map(str::to_owned)
        } else if p.is_dir() && p.join("main.rs").is_file() {
            p.file_name().and_then(|s| s.to_str()).map(str::to_owned)
        } else {
            None
        };
        if let Some(bname) = discovered
            && !bins.iter().any(|existing| existing == &bname)
        {
            bins.push(bname);
        }
    }
}

fn parse_dep_section_header(inner: &str) -> Option<Option<String>> {
    let tail = if let Some(after_target) = inner.strip_prefix("target.") {
        let bytes = after_target.as_bytes();
        let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
        let mut dot_pos = None;
        for (i, &b) in bytes.iter().enumerate() {
            if esc {
                esc = false;
                continue;
            }
            if in_dq {
                if b == b'\\' {
                    esc = true;
                } else if b == b'"' {
                    in_dq = false;
                }
            } else if in_sq {
                if b == b'\'' {
                    in_sq = false;
                }
            } else if b == b'"' {
                in_dq = true;
            } else if b == b'\'' {
                in_sq = true;
            } else if b == b'.' {
                let rest = &after_target[i + 1..];
                if rest == "dependencies"
                    || rest == "build-dependencies"
                    || rest.starts_with("dependencies.")
                    || rest.starts_with("build-dependencies.")
                {
                    dot_pos = Some(i);
                    break;
                }
            }
        }
        &after_target[dot_pos? + 1..]
    } else {
        inner
    };

    if tail == "dependencies" || tail == "build-dependencies" {
        return Some(None);
    }
    let dep = tail
        .strip_prefix("dependencies.")
        .or_else(|| tail.strip_prefix("build-dependencies."))?;
    let clean = dep.trim().trim_matches('"').trim_matches('\'');
    if clean.is_empty() {
        None
    } else {
        Some(Some(clean.to_owned()))
    }
}

fn parse_crate_manifest_fallback(raw: &str, crate_dir: &Path) -> ParsedCrateManifest {
    let mut name = crate_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_owned();
    let mut autobins = true;
    let mut features: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut bins = Vec::new();
    let mut deps: Vec<ParsedDep> = Vec::new();

    #[derive(Clone, PartialEq, Eq)]
    enum Sec {
        None,
        Package,
        Features,
        Bin,
        Deps,
        NamedDep(usize),
    }
    let mut sec = Sec::None;
    let mut pending_feat: Option<(String, String)> = None;
    let mut pending_inline_dep: Option<(String, String, i32)> = None;
    let mut pending_named_dep_feat: Option<(usize, String)> = None;
    let mut current_dep_section_start: usize = 0;

    for raw_line in raw.lines() {
        let line = super::mutation::strip_toml_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if let Some((ref fk, ref mut acc)) = pending_feat {
            acc.push(' ');
            acc.push_str(line);
            if line.contains(']') {
                features.insert(fk.clone(), parse_bracket_strings(acc));
                pending_feat = None;
            }
            continue;
        }
        if let Some((ref dep_key, ref mut acc, ref mut depth)) = pending_inline_dep {
            acc.push(' ');
            acc.push_str(line);
            *depth += inline_brace_delta(line);
            if *depth <= 0 {
                deps.push(parse_inline_dep(dep_key, acc));
                pending_inline_dep = None;
            }
            continue;
        }
        if let Some((dep_idx, ref mut acc)) = pending_named_dep_feat {
            acc.push(' ');
            acc.push_str(line);
            if line.contains(']') {
                deps[dep_idx].features = parse_bracket_strings(acc);
                pending_named_dep_feat = None;
            }
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            sec = match line {
                "[package]" => Sec::Package,
                "[features]" => Sec::Features,
                "[[bin]]" => Sec::Bin,
                _ => {
                    let inner = line[1..line.len() - 1].trim();
                    match parse_dep_section_header(inner) {
                        Some(None) => {
                            current_dep_section_start = deps.len();
                            Sec::Deps
                        }
                        Some(Some(dep_key)) => {
                            let idx = deps.len();
                            deps.push(ParsedDep {
                                key: dep_key.clone(),
                                package: dep_key,
                                default_features: true,
                                optional: false,
                                features: Vec::new(),
                            });
                            Sec::NamedDep(idx)
                        }
                        None => Sec::None,
                    }
                }
            };
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let key = unquote_toml_atom(k);
        let val = v.trim();
        match &sec {
            Sec::Package if key == "name" => {
                name = unquote_toml_atom(val).to_owned();
            }
            Sec::Package if key == "autobins" => {
                autobins = val != "false";
            }
            Sec::Bin if key == "name" => {
                bins.push(unquote_toml_atom(val).to_owned());
            }
            Sec::Features => {
                if val.contains('[') && !val.contains(']') {
                    pending_feat = Some((key.to_owned(), val.to_owned()));
                } else {
                    features.insert(key.to_owned(), parse_bracket_strings(val));
                }
            }
            Sec::Deps => {
                if let Some((dep_part, field_part)) = split_dotted_dep_key(k.trim()) {
                    let dep_idx = deps[current_dep_section_start..]
                        .iter()
                        .position(|d| d.key == dep_part)
                        .map(|off| current_dep_section_start + off)
                        .unwrap_or_else(|| {
                            let idx = deps.len();
                            deps.push(ParsedDep {
                                key: dep_part.to_owned(),
                                package: dep_part.to_owned(),
                                default_features: true,
                                optional: false,
                                features: Vec::new(),
                            });
                            idx
                        });
                    apply_dep_field(
                        &mut deps[dep_idx],
                        dep_idx,
                        field_part,
                        val,
                        &mut pending_named_dep_feat,
                    );
                } else if val.starts_with('{') {
                    let depth = inline_brace_delta(val);
                    if depth <= 0 {
                        deps.push(parse_inline_dep(key, val));
                    } else {
                        pending_inline_dep = Some((key.to_owned(), val.to_owned(), depth));
                    }
                } else {
                    deps.push(ParsedDep {
                        key: key.to_owned(),
                        package: key.to_owned(),
                        default_features: true,
                        optional: false,
                        features: Vec::new(),
                    });
                }
            }
            &Sec::NamedDep(dep_idx) => {
                apply_dep_field(
                    &mut deps[dep_idx],
                    dep_idx,
                    key,
                    val,
                    &mut pending_named_dep_feat,
                );
            }
            _ => {}
        }
    }

    if autobins {
        collect_autobins(crate_dir, &mut bins);
    }

    ParsedCrateManifest {
        name,
        dir: crate_dir.to_path_buf(),
        features,
        bins,
        deps,
    }
}

fn unquote_toml_atom(s: &str) -> &str {
    let trimmed = s.trim();
    if trimmed.len() >= 2
        && ((trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\'')))
    {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    }
}

fn split_dotted_dep_key(raw_key: &str) -> Option<(&str, &str)> {
    let bytes = raw_key.as_bytes();
    let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
    for (i, &b) in bytes.iter().enumerate() {
        if esc {
            esc = false;
            continue;
        }
        if in_dq {
            if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_dq = false;
            }
        } else if in_sq {
            if b == b'\'' {
                in_sq = false;
            }
        } else if b == b'"' {
            in_dq = true;
        } else if b == b'\'' {
            in_sq = true;
        } else if b == b'.' {
            let dep = unquote_toml_atom(&raw_key[..i]);
            let field = unquote_toml_atom(&raw_key[i + 1..]);
            if !dep.is_empty() && !field.is_empty() {
                return Some((dep, field));
            }
            return None;
        }
    }
    None
}

fn apply_dep_field(
    dep: &mut ParsedDep,
    dep_idx: usize,
    field: &str,
    val: &str,
    pending_feat: &mut Option<(usize, String)>,
) {
    match field {
        "package" => dep.package = unquote_toml_atom(val).to_owned(),
        "default-features" | "default_features" => dep.default_features = val != "false",
        "optional" => dep.optional = val == "true",
        "features" => {
            if val.contains('[') && !val.contains(']') {
                *pending_feat = Some((dep_idx, val.to_owned()));
            } else {
                dep.features = parse_bracket_strings(val);
            }
        }
        _ => {}
    }
}

fn inline_brace_delta(s: &str) -> i32 {
    let mut delta = 0i32;
    let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
    for b in s.bytes() {
        if esc {
            esc = false;
            continue;
        }
        if in_dq {
            if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_dq = false;
            }
        } else if in_sq {
            if b == b'\'' {
                in_sq = false;
            }
        } else if b == b'"' {
            in_dq = true;
        } else if b == b'\'' {
            in_sq = true;
        } else if b == b'{' {
            delta += 1;
        } else if b == b'}' {
            delta -= 1;
        }
    }
    delta
}

fn parse_inline_dep(dep_key: &str, raw_table: &str) -> ParsedDep {
    let mut dep = ParsedDep {
        key: dep_key.to_owned(),
        package: dep_key.to_owned(),
        default_features: true,
        optional: false,
        features: Vec::new(),
    };
    let trimmed = raw_table.trim();
    let Some(open) = trimmed.find('{') else {
        return dep;
    };
    let close = trimmed.rfind('}').unwrap_or(trimmed.len());
    if close <= open + 1 {
        return dep;
    }
    let inner = &trimmed[open + 1..close];

    // Split top-level comma-separated entries respecting strings, brackets, and nested braces.
    let bytes = inner.as_bytes();
    let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
    let (mut bracket_depth, mut brace_depth) = (0usize, 0usize);
    let mut entry_start = 0usize;
    let mut entries = Vec::new();
    for (i, &b) in bytes.iter().enumerate() {
        if esc {
            esc = false;
            continue;
        }
        if in_dq {
            if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_dq = false;
            }
        } else if in_sq {
            if b == b'\'' {
                in_sq = false;
            }
        } else {
            match b {
                b'"' => in_dq = true,
                b'\'' => in_sq = true,
                b'[' => bracket_depth += 1,
                b']' => bracket_depth = bracket_depth.saturating_sub(1),
                b'{' => brace_depth += 1,
                b'}' => brace_depth = brace_depth.saturating_sub(1),
                b',' if bracket_depth == 0 && brace_depth == 0 => {
                    entries.push(&inner[entry_start..i]);
                    entry_start = i + 1;
                }
                _ => {}
            }
        }
    }
    if entry_start < inner.len() {
        entries.push(&inner[entry_start..]);
    }

    let mut unused_pending = None;
    for entry in entries {
        let Some((k, v)) = entry.split_once('=') else {
            continue;
        };
        let field = unquote_toml_atom(k);
        apply_dep_field(&mut dep, 0, field, v.trim(), &mut unused_pending);
    }
    dep
}

fn parse_bracket_strings(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(start) = s.find('[') else {
        return out;
    };
    let rest = &s[start + 1..];
    let end = rest.find(']').unwrap_or(rest.len());
    let inner = &rest[..end];
    for part in inner.split(',') {
        let tok = part.trim().trim_matches('"').trim_matches('\'').trim();
        if !tok.is_empty() {
            out.push(tok.to_owned());
        }
    }
    out
}
