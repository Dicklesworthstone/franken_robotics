//! `xtask check-deps` extensions for `N34-03`:
//! 1. One `asupersync` package per binary (`asup.duplicate`, INV-SUITE-02), with
//!    per-binary `cargo tree` precision confirmation and per-binary report facts.
//! 2. Airlock feature group enforcement (`fsim.group-not-allowed`, `fsim.default-set`,
//!    `fsim.optional-default-on`, INV-SUITE-01), reading allow-lists dynamically from
//!    `crates/frob-fsim/contract.toml` (never compiled into `xtask`).
//! 3. Tier features off by default (`tier.default-on`, `tier.unnamed`, L-12, INV-WS-04).
//! 4. All-features resolution (`resolve.all-features`).
//!
//! Determinism class D0: findings and facts are sorted deterministically, zero `unsafe`,
//! prints nothing (rule L-7), and reads all TOML exclusively via `frob_core::registry::toml`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{self, Table, Value};

use crate::crate_map::{CrateRow, Finding, Tier, lookup_in};
use crate::metadata::{
    DeclaredDep, DepKind, PackageMeta, ResolvedPackage, ResolvedWorkspace, WorkspaceKind,
    WorkspaceRoot, normalize_rel_path, run_cargo_tree_package_depth,
};
use crate::report::FactValue;

/// Rule identifier for more than one `asupersync` package in a binary's normal+build closure (INV-SUITE-02).
pub const RULE_ASUP_DUPLICATE: &str = "asup.duplicate";
/// Rule identifier for a crate enabling an airlock feature group it is not listed for (INV-SUITE-01).
pub const RULE_FSIM_GROUP_NOT_ALLOWED: &str = "fsim.group-not-allowed";
/// Rule identifier for `frob-fsim`'s default group set differing from `{referee, geometry, ledger}` (INV-SUITE-01).
pub const RULE_FSIM_DEFAULT_SET: &str = "fsim.default-set";
/// Rule identifier for a listed consumer enabling an optional airlock group by default (INV-SUITE-01).
pub const RULE_FSIM_OPTIONAL_DEFAULT_ON: &str = "fsim.optional-default-on";
/// Rule identifier for a `frontier-*` or `moonshot-*` feature enabled by default or leaking into core builds (L-12, INV-WS-04).
pub const RULE_TIER_DEFAULT_ON: &str = "tier.default-on";
/// Rule identifier for a tier-F/M feature in `contract.toml` lacking the `frontier-`/`moonshot-` prefix (L-12, INV-WS-04).
pub const RULE_TIER_UNNAMED: &str = "tier.unnamed";
/// Rule identifier for a workspace failing `--all-features` offline resolution (`N34-03`).
pub const RULE_RESOLVE_ALL_FEATURES: &str = "resolve.all-features";

/// Canonical default airlock groups of `frob-fsim` (excluding non-group build-mode features such as `mode-inproc`).
const CANONICAL_DEFAULT_AIRLOCK_GROUPS: [&str; 3] = ["geometry", "ledger", "referee"];

/// One parsed `[features.<group>]` record read directly from `crates/frob-fsim/contract.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AirlockGroupRecord {
    pub name: String,
    pub default: bool,
    pub dev_only: bool,
    pub consumers: Vec<String>,
    pub test_consumers: Vec<String>,
    pub parent_group: Option<String>,
}

/// One resolved `asupersync` package reachable from a binary target.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsupInstance {
    pub version: String,
    pub source: String,
    pub path: Vec<String>,
}

/// Per-binary `asupersync` summary fact recorded in `check-deps` report `facts["binaries"]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryAsupFact {
    pub workspace: String,
    pub package: String,
    pub binary: String,
    pub asupersync_version: String,
    pub asupersync_source: String,
    pub packages: Vec<AsupInstance>,
}

impl BinaryAsupFact {
    #[must_use]
    pub fn to_fact_value(&self) -> FactValue {
        let mut obj = BTreeMap::new();
        obj.insert("asupersync".to_owned(), FactValue::str(&self.asupersync_version));
        obj.insert(
            "asupersync_source".to_owned(),
            FactValue::str(&self.asupersync_source),
        );
        obj.insert("binary".to_owned(), FactValue::str(&self.binary));
        obj.insert("package".to_owned(), FactValue::str(&self.package));
        let pkgs = self
            .packages
            .iter()
            .map(|inst| {
                let mut m = BTreeMap::new();
                m.insert("path".to_owned(), FactValue::str_array(inst.path.clone()));
                m.insert("source".to_owned(), FactValue::str(&inst.source));
                m.insert("version".to_owned(), FactValue::str(&inst.version));
                FactValue::Object(m)
            })
            .collect();
        obj.insert("packages".to_owned(), FactValue::Array(pkgs));
        obj.insert("workspace".to_owned(), FactValue::str(&self.workspace));
        FactValue::Object(obj)
    }
}

// ============================================================================
// 1. Per-binary `asupersync` closure and `cargo tree` confirmation (`asup.duplicate`)
// ============================================================================

/// Evaluate the one-`asupersync`-per-binary rule (`INV-SUITE-02`) for every binary
/// target in `ws` over both default-feature and `--all-features` resolved graphs.
///
/// Any candidate duplicate (`> 1` `asupersync` package in the workspace resolve walk)
/// is confirmed for that package alone via `cargo tree -p <pkg> -e normal,build --prefix depth`
/// so workspace-wide feature unification never produces false positives. Fail-closed: if
/// confirmation cannot run, the candidate is reported with `confirmed: false`.
pub fn evaluate_binary_asupersync(
    ws: &WorkspaceRoot,
    declared_members: &[PackageMeta],
    resolved_default: &ResolvedWorkspace,
    resolved_all: Option<&ResolvedWorkspace>,
    findings: &mut Vec<Finding>,
) -> Vec<BinaryAsupFact> {
    let members_by_id: BTreeMap<&str, &PackageMeta> = declared_members
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect();

    let mut binaries: Vec<(&str, &ResolvedPackage, String)> = Vec::new();
    for member_id in &resolved_default.workspace_members {
        if let Some(pkg) = resolved_default.packages.get(member_id) {
            for bin_name in &pkg.bin_targets {
                binaries.push((member_id.as_str(), pkg, bin_name.clone()));
            }
        }
    }
    binaries.sort_by(|a, b| {
        a.2.cmp(&b.2)
            .then_with(|| a.1.name.cmp(&b.1.name))
            .then_with(|| a.0.cmp(b.0))
    });

    // Cache per-package `cargo tree` confirmation results so multi-bin packages invoke it once.
    let mut tree_cache: BTreeMap<(String, bool), Result<Vec<AsupInstance>, String>> = BTreeMap::new();
    let mut binary_facts = Vec::with_capacity(binaries.len());

    for (pkg_id, pkg, bin_name) in binaries {
        let rel_manifest = members_by_id
            .get(pkg_id)
            .map(|m| m.rel_manifest.clone())
            .unwrap_or_else(|| {
                pkg.rel_dir.as_deref().map_or_else(
                    || "Cargo.toml".to_owned(),
                    |d| {
                        if d.is_empty() {
                            "Cargo.toml".to_owned()
                        } else {
                            format!("{d}/Cargo.toml")
                        }
                    },
                )
            });

        let default_asups = collect_binary_asupersync_closure(resolved_default, pkg_id);
        let mut reported_duplicate = false;
        let confirmed_default: Vec<AsupInstance> = if default_asups.len() > 1 {
            let tree_res = tree_cache
                .entry((pkg.name.clone(), false))
                .or_insert_with(|| confirm_binary_asupersync_via_tree(ws, &pkg.name, false));
            match tree_res {
                Ok(confirmed) if confirmed.len() > 1 => {
                    reported_duplicate = true;
                    findings.push(build_asup_duplicate_finding(
                        ws,
                        &pkg.name,
                        &bin_name,
                        &rel_manifest,
                        confirmed,
                        "default",
                        true,
                        None,
                    ));
                    confirmed.clone()
                }
                Ok(confirmed) => {
                    // Cleared by per-package confirmation: candidate came only from
                    // workspace-wide feature unification of another member.
                    confirmed.clone()
                }
                Err(err_msg) => {
                    // Fail-closed: confirmation could not run.
                    reported_duplicate = true;
                    findings.push(build_asup_duplicate_finding(
                        ws,
                        &pkg.name,
                        &bin_name,
                        &rel_manifest,
                        &default_asups,
                        "default",
                        false,
                        Some(err_msg),
                    ));
                    default_asups.clone()
                }
            }
        } else {
            default_asups
        };

        // Also check the `--all-features` closure for binary-scoped duplicates that only
        // appear when optional features of this binary are turned on.
        if !reported_duplicate
            && let Some(ra) = resolved_all
        {
            let all_asups = collect_binary_asupersync_closure(ra, pkg_id);
            if all_asups.len() > 1 {
                let tree_res = tree_cache
                    .entry((pkg.name.clone(), true))
                    .or_insert_with(|| confirm_binary_asupersync_via_tree(ws, &pkg.name, true));
                match tree_res {
                    Ok(confirmed) if confirmed.len() > 1 => {
                        findings.push(build_asup_duplicate_finding(
                            ws,
                            &pkg.name,
                            &bin_name,
                            &rel_manifest,
                            confirmed,
                            "all-features",
                            true,
                            None,
                        ));
                    }
                    Ok(_) => {
                        // Cleared by per-package `--all-features` confirmation.
                    }
                    Err(err_msg) => {
                        findings.push(build_asup_duplicate_finding(
                            ws,
                            &pkg.name,
                            &bin_name,
                            &rel_manifest,
                            &all_asups,
                            "all-features",
                            false,
                            Some(err_msg),
                        ));
                    }
                }
            }
        }

        let (ver, src) = match confirmed_default.as_slice() {
            [] => ("none".to_owned(), "none".to_owned()),
            [single] => (single.version.clone(), single.source.clone()),
            many => (
                many.iter()
                    .map(|m| m.version.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
                many.iter()
                    .map(|m| m.source.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        };

        binary_facts.push(BinaryAsupFact {
            workspace: ws.name.clone(),
            package: pkg.name.clone(),
            binary: bin_name,
            asupersync_version: ver,
            asupersync_source: src,
            packages: confirmed_default,
        });
    }

    binary_facts
}

/// Deterministic BFS from `root_pkg_id` over `Normal` and `Build` edges only, collecting
/// every distinct `asupersync` package ID with one shortest dependency path each.
#[must_use]
pub fn collect_binary_asupersync_closure(
    resolved: &ResolvedWorkspace,
    root_pkg_id: &str,
) -> Vec<AsupInstance> {
    let parent = bfs_normal_build_parents(resolved, root_pkg_id);
    let mut out = Vec::new();
    let mut reachable: Vec<(&str, &ResolvedPackage)> = parent
        .keys()
        .filter_map(|id| resolved.packages.get(*id).map(|p| (*id, p)))
        .filter(|(_, p)| p.name == "asupersync")
        .collect();
    reachable.sort_by(|a, b| {
        a.1.version
            .cmp(&b.1.version)
            .then_with(|| a.1.source.cmp(&b.1.source))
            .then_with(|| a.0.cmp(b.0))
    });

    for (pkg_id, pkg) in reachable {
        let path = reconstruct_path_from_parents(pkg_id, &parent, resolved);
        let source = format_package_source(pkg);
        out.push(AsupInstance {
            version: pkg.version.clone(),
            source,
            path,
        });
    }
    out
}

fn bfs_normal_build_parents<'a>(
    resolved: &'a ResolvedWorkspace,
    root_pkg_id: &'a str,
) -> BTreeMap<&'a str, Option<&'a str>> {
    let mut parent: BTreeMap<&'a str, Option<&'a str>> = BTreeMap::new();
    let mut queue: VecDeque<&'a str> = VecDeque::new();
    parent.insert(root_pkg_id, None);
    queue.push_back(root_pkg_id);

    while let Some(cur) = queue.pop_front() {
        let Some(node) = resolved.nodes.get(cur) else {
            continue;
        };
        let mut children: Vec<&'a str> = if !node.deps.is_empty() {
            node.deps
                .iter()
                .filter(|d| {
                    d.dep_kinds
                        .iter()
                        .any(|k| matches!(k, DepKind::Normal | DepKind::Build))
                })
                .map(|d| d.pkg.as_str())
                .collect()
        } else {
            node.dependencies.iter().map(String::as_str).collect()
        };
        children.sort_by(|a, b| {
            let na = resolved.packages.get(*a).map_or(*a, |p| p.name.as_str());
            let nb = resolved.packages.get(*b).map_or(*b, |p| p.name.as_str());
            na.cmp(nb).then_with(|| a.cmp(b))
        });
        children.dedup();
        for child in children {
            if !parent.contains_key(child) {
                parent.insert(child, Some(cur));
                queue.push_back(child);
            }
        }
    }
    parent
}

fn reconstruct_path_from_parents(
    target_id: &str,
    parent: &BTreeMap<&str, Option<&str>>,
    resolved: &ResolvedWorkspace,
) -> Vec<String> {
    let mut chain = Vec::new();
    let mut cursor = Some(target_id);
    while let Some(id) = cursor {
        let label = resolved
            .packages
            .get(id)
            .map_or_else(|| id.to_owned(), |p| p.name.clone());
        chain.push(label);
        cursor = parent.get(id).copied().flatten();
    }
    chain.reverse();
    chain
}

fn format_package_source(pkg: &ResolvedPackage) -> String {
    if let Some(src) = pkg.source.as_deref() {
        return src.to_owned();
    }
    if let Some(sib) = pkg.sibling_rel_path.as_deref() {
        return format!("path:{sib}");
    }
    if let Some(rel) = pkg.rel_dir.as_deref() {
        return format!("path:{rel}");
    }
    let dir = pkg
        .manifest_path
        .parent()
        .map(normalize_rel_path)
        .unwrap_or_else(|| "<path>".to_owned());
    format!("path:{dir}")
}

fn confirm_binary_asupersync_via_tree(
    ws: &WorkspaceRoot,
    pkg_name: &str,
    all_features: bool,
) -> Result<Vec<AsupInstance>, String> {
    let stdout = run_cargo_tree_package_depth(ws, pkg_name, all_features)?;
    Ok(parse_cargo_tree_depth_asupersync(&stdout))
}

/// Parse `cargo tree --prefix depth` stdout and collect every distinct `asupersync`
/// package `(version, source)` with its first observed dependency chain.
#[must_use]
pub fn parse_cargo_tree_depth_asupersync(stdout: &str) -> Vec<AsupInstance> {
    let mut stack: Vec<(usize, String)> = Vec::new();
    let mut found: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();

    for raw_line in stdout.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() {
            continue;
        }
        let digit_len = line.bytes().take_while(u8::is_ascii_digit).count();
        if digit_len == 0 {
            // Section headers such as `[build-dependencies]` carry no depth prefix.
            continue;
        }
        let Ok(depth) = line[..digit_len].parse::<usize>() else {
            continue;
        };
        let rest = line[digit_len..].trim();
        let cleaned = rest
            .strip_suffix(" (*)")
            .unwrap_or(rest)
            .trim();
        let cleaned = cleaned
            .strip_suffix(" (proc-macro)")
            .unwrap_or(cleaned)
            .trim();

        let mut parts = cleaned.splitn(3, ' ');
        let Some(pkg_name) = parts.next() else {
            continue;
        };
        let ver_tok = parts.next().unwrap_or("");
        let version = ver_tok.strip_prefix('v').unwrap_or(ver_tok).to_owned();
        let tail = parts.next().unwrap_or("").trim();

        while stack.last().is_some_and(|(d, _)| *d >= depth) {
            stack.pop();
        }
        stack.push((depth, pkg_name.to_owned()));

        if pkg_name == "asupersync" {
            let source = if tail.starts_with('(') && tail.ends_with(')') {
                let inner = &tail[1..tail.len() - 1];
                if inner.starts_with("http://")
                    || inner.starts_with("https://")
                    || inner.starts_with("git+")
                    || inner.starts_with("registry+")
                {
                    inner.to_owned()
                } else {
                    format!("path:{inner}")
                }
            } else {
                "registry".to_owned()
            };
            let path: Vec<String> = stack.iter().map(|(_, n)| n.clone()).collect();
            found.entry((version, source)).or_insert(path);
        }
    }

    found
        .into_iter()
        .map(|((version, source), path)| AsupInstance {
            version,
            source,
            path,
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn build_asup_duplicate_finding(
    ws: &WorkspaceRoot,
    pkg_name: &str,
    bin_name: &str,
    rel_manifest: &str,
    instances: &[AsupInstance],
    mode: &str,
    confirmed: bool,
    confirm_error: Option<&str>,
) -> Finding {
    let mut details = Vec::with_capacity(instances.len());
    for inst in instances {
        details.push(format!(
            "asupersync {} [{}] via `{}`",
            inst.version,
            inst.source,
            inst.path.join(" -> ")
        ));
    }
    let err_suffix = confirm_error
        .map(|e| format!("; confirmation error: {e}"))
        .unwrap_or_default();
    Finding {
        rule: RULE_ASUP_DUPLICATE,
        subject: format!("{}:{bin_name} ({pkg_name})", ws.name),
        path: rel_manifest.to_owned(),
        line: None,
        message: format!(
            "binary `{bin_name}` (package `{pkg_name}`, workspace `{}`, mode `{mode}`, confirmed: {confirmed}) links {} distinct `asupersync` packages (INV-SUITE-02): {}{err_suffix}",
            ws.name,
            instances.len(),
            details.join("; ")
        ),
        repair: format!(
            "align all dependencies of `{pkg_name}` onto the single `asupersync` package/source pinned in franken-stack.lock so `{bin_name}` links at most one `asupersync` runtime"
        ),
    }
}

// ============================================================================
// 2. Airlock feature groups (`fsim.group-not-allowed`, `fsim.default-set`,
//    `fsim.optional-default-on`, INV-SUITE-01)
// ============================================================================

/// Load the airlock group records from `crates/frob-fsim/contract.toml` strictly via
/// `frob_core::registry::toml`. Never hardcodes any consumer allow-list in `xtask`.
pub fn load_airlock_groups(
    contract_path: &Path,
) -> Result<BTreeMap<String, AirlockGroupRecord>, String> {
    let bytes = fs::read(contract_path)
        .map_err(|e| format!("failed to read `{}`: {e}", contract_path.display()))?;
    parse_airlock_groups_bytes(&bytes, &contract_path.display().to_string())
}

/// Strictly parse `[features.*]` airlock group records from `frob-fsim/contract.toml` bytes.
pub fn parse_airlock_groups_bytes(
    bytes: &[u8],
    source_label: &str,
) -> Result<BTreeMap<String, AirlockGroupRecord>, String> {
    let root = toml::parse(bytes)
        .map_err(|e| format!("{source_label}: TOML parse error: {e}"))?;
    let Some(Value::Table(features_tbl)) = root.get("features") else {
        return Ok(BTreeMap::new());
    };

    let mut groups = BTreeMap::new();
    for (group_name, sp) in features_tbl.iter() {
        let Value::Table(gtbl) = &sp.value else {
            return Err(format!(
                "{source_label}: `[features.{group_name}]` must be a table"
            ));
        };
        let default = gtbl
            .get("default")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let dev_only = gtbl
            .get("dev_only")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let consumers = extract_str_array(gtbl, "consumers", source_label, group_name)?;
        let test_consumers = extract_str_array(gtbl, "test_consumers", source_label, group_name)?;
        let parent_group = gtbl
            .get("parent_group")
            .and_then(Value::as_str)
            .map(str::to_owned);

        groups.insert(
            group_name.clone(),
            AirlockGroupRecord {
                name: group_name.clone(),
                default,
                dev_only,
                consumers,
                test_consumers,
                parent_group,
            },
        );
    }
    Ok(groups)
}

fn extract_str_array(
    tbl: &Table,
    key: &str,
    source_label: &str,
    ctx: &str,
) -> Result<Vec<String>, String> {
    let Some(val) = tbl.get(key) else {
        return Ok(Vec::new());
    };
    let Some(arr) = val.as_array() else {
        return Err(format!(
            "{source_label}: `[features.{ctx}].{key}` expected array, found {}",
            val.type_name()
        ));
    };
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let Some(s) = item.value.as_str() else {
            return Err(format!(
                "{source_label}: `[features.{ctx}].{key}` element expected string, found {}",
                item.value.type_name()
            ));
        };
        out.push(s.to_owned());
    }
    Ok(out)
}

/// Locate `frob-fsim/contract.toml` for this workspace or repository root.
#[must_use]
pub fn locate_frob_fsim_contract(
    repo_root: &Path,
    declared_members: &[PackageMeta],
) -> Option<PathBuf> {
    if let Some(pkg) = declared_members.iter().find(|m| m.name == "frob-fsim") {
        let p = pkg.crate_dir().join("contract.toml");
        if p.is_file() {
            return Some(p);
        }
    }
    let fallback = repo_root.join("crates/frob-fsim/contract.toml");
    fallback.is_file().then_some(fallback)
}

/// Evaluate airlock feature-group rules (`fsim.default-set`, `fsim.group-not-allowed`,
/// `fsim.optional-default-on`) over one workspace and return a sorted per-crate map of
/// enabled airlock groups for report `facts`.
pub fn evaluate_airlock_groups(
    repo_root: &Path,
    declared_members: &[PackageMeta],
    resolved_default: &ResolvedWorkspace,
    findings: &mut Vec<Finding>,
) -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut enabled_by_crate: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let Some(contract_path) = locate_frob_fsim_contract(repo_root, declared_members) else {
        return Ok(enabled_by_crate);
    };
    let groups = load_airlock_groups(&contract_path)?;
    if groups.is_empty() {
        return Ok(enabled_by_crate);
    }

    let contract_rel = contract_path
        .strip_prefix(repo_root)
        .ok()
        .map(normalize_rel_path)
        .unwrap_or_else(|| normalize_rel_path(&contract_path));

    // Rule 3b: `fsim.default-set` on `frob-fsim` itself.
    let mut default_set_bad_groups: BTreeSet<String> = BTreeSet::new();
    if let Some(fsim_pkg) = declared_members.iter().find(|m| m.name == "frob-fsim") {
        check_fsim_default_set(
            fsim_pkg,
            &contract_rel,
            &groups,
            &mut default_set_bad_groups,
            findings,
        );
    }

    // Rule 3a & 3c: every consumer member's `frob-fsim` dependencies and feature forwardings.
    let mut attributed_optional_groups: BTreeSet<String> = BTreeSet::new();
    for member in declared_members {
        if member.name == "frob-fsim" {
            continue;
        }
        let enabled = check_member_airlock_usage(
            &member.name,
            &member.rel_manifest,
            &member.dependencies,
            &member.features,
            &groups,
            &mut attributed_optional_groups,
            findings,
        );
        if !enabled.is_empty() {
            enabled_by_crate.insert(member.name.clone(), enabled);
        }
    }

    // Belt-and-suspenders check on the default-feature resolved feature set of `frob-fsim`:
    // if an off-by-default non-dev-only group is active in default resolution and wasn't
    // already attributed to a specific member or `fsim.default-set`, report `fsim.optional-default-on`.
    if let Some((fsim_id, fsim_pkg)) = resolved_default
        .packages
        .iter()
        .find(|(_, p)| p.name == "frob-fsim")
        && let Some(node) = resolved_default.nodes.get(fsim_id)
    {
        let fsim_manifest = fsim_pkg
            .rel_dir
            .as_deref()
            .map_or_else(|| "crates/frob-fsim/Cargo.toml".to_owned(), |d| format!("{d}/Cargo.toml"));
        for feat in &node.features {
            if let Some(rec) = groups.get(feat)
                && !rec.default
                && !rec.dev_only
                && !default_set_bad_groups.contains(feat)
                && !attributed_optional_groups.contains(feat)
            {
                findings.push(Finding {
                    rule: RULE_FSIM_OPTIONAL_DEFAULT_ON,
                    subject: format!("frob-fsim [{feat}]"),
                    path: fsim_manifest.clone(),
                    line: None,
                    message: format!(
                        "optional airlock group `{feat}` is active in the workspace default-feature resolution of `frob-fsim` (INV-SUITE-01)"
                    ),
                    repair: format!(
                        "keep `{feat}` off by default and request `frob-fsim/{feat}` only through an opt-in non-default consumer feature"
                    ),
                });
            }
        }
    }

    Ok(enabled_by_crate)
}

/// Evaluate a standalone manifest file (such as N33a-11's planted fixture
/// `crates/frob-fsim/tests/fixtures/frob_dyn_enables_structure/Cargo.toml` read in place,
/// never copied) directly against parsed `crates/frob-fsim/contract.toml` group records.
pub fn check_airlock_manifest_file(
    manifest_path: &Path,
    rel_manifest: &str,
    groups: &BTreeMap<String, AirlockGroupRecord>,
) -> Result<Vec<Finding>, String> {
    let bytes = fs::read(manifest_path)
        .map_err(|e| format!("failed to read `{}`: {e}", manifest_path.display()))?;
    let root = toml::parse(&bytes)
        .map_err(|e| format!("{}: TOML parse error: {e}", manifest_path.display()))?;

    let pkg_tbl = root
        .get("package")
        .and_then(Value::as_table)
        .ok_or_else(|| format!("{}: missing `[package]` table", manifest_path.display()))?;
    let pkg_name = pkg_tbl
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{}: missing `[package].name`", manifest_path.display()))?
        .to_owned();

    let mut deps = Vec::new();
    for (section, kind) in [
        ("dependencies", DepKind::Normal),
        ("dev-dependencies", DepKind::Dev),
        ("build-dependencies", DepKind::Build),
    ] {
        if let Some(tbl) = root.get(section).and_then(Value::as_table) {
            parse_manifest_dep_section(tbl, kind, &mut deps);
        }
    }

    let mut features = BTreeMap::new();
    if let Some(ftbl) = root.get("features").and_then(Value::as_table) {
        for (fname, sp) in ftbl.iter() {
            if let Some(arr) = sp.value.as_array() {
                let items = arr
                    .iter()
                    .filter_map(|i| i.value.as_str().map(str::to_owned))
                    .collect();
                features.insert(fname.clone(), items);
            }
        }
    }

    let mut findings = Vec::new();
    let mut attributed = BTreeSet::new();
    check_member_airlock_usage(
        &pkg_name,
        rel_manifest,
        &deps,
        &features,
        groups,
        &mut attributed,
        &mut findings,
    );
    findings.sort();
    findings.dedup();
    Ok(findings)
}

fn parse_manifest_dep_section(tbl: &Table, kind: DepKind, out: &mut Vec<DeclaredDep>) {
    for (dep_key, sp) in tbl.iter() {
        let mut actual_name = dep_key.clone();
        let mut rename = None;
        let mut uses_default_features = true;
        let mut optional = false;
        let mut features = Vec::new();
        let mut req = "*".to_owned();
        let mut path = None;

        match &sp.value {
            Value::Str(s) => {
                req = s.clone();
            }
            Value::Table(dt) => {
                if let Some(pkg) = dt.get("package").and_then(Value::as_str) {
                    actual_name = pkg.to_owned();
                    rename = Some(dep_key.clone());
                }
                if let Some(v) = dt.get("version").and_then(Value::as_str) {
                    req = v.to_owned();
                }
                if let Some(p) = dt.get("path").and_then(Value::as_str) {
                    path = Some(PathBuf::from(p));
                }
                if let Some(df) = dt.get("default-features").and_then(Value::as_bool) {
                    uses_default_features = df;
                }
                if let Some(opt) = dt.get("optional").and_then(Value::as_bool) {
                    optional = opt;
                }
                if let Some(arr) = dt.get("features").and_then(Value::as_array) {
                    for item in arr {
                        if let Some(s) = item.value.as_str() {
                            features.push(s.to_owned());
                        }
                    }
                }
            }
            _ => continue,
        }

        out.push(DeclaredDep {
            name: actual_name,
            rename,
            kind,
            req,
            source: None,
            path,
            rel_path: None,
            sibling_rel_path: None,
            uses_default_features,
            features,
            optional,
            target: None,
        });
    }
}

fn check_fsim_default_set(
    fsim_pkg: &PackageMeta,
    contract_rel: &str,
    groups: &BTreeMap<String, AirlockGroupRecord>,
    bad_groups: &mut BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    let expected_canonical: BTreeSet<String> = CANONICAL_DEFAULT_AIRLOCK_GROUPS
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    let contract_defaults: BTreeSet<String> = groups
        .values()
        .filter(|g| g.default)
        .map(|g| g.name.clone())
        .collect();

    // Expand `default` transitively inside `frob-fsim`, keeping only entries that are
    // registered airlock groups (non-group build modes like `mode-inproc` are not counted).
    let default_closure = expand_local_feature_closure(&fsim_pkg.features, "default");
    let manifest_default_groups: BTreeSet<String> = default_closure
        .keys()
        .filter(|f| groups.contains_key(*f))
        .cloned()
        .collect();

    if contract_defaults != expected_canonical {
        for g in contract_defaults.difference(&expected_canonical) {
            bad_groups.insert(g.clone());
        }
        findings.push(Finding {
            rule: RULE_FSIM_DEFAULT_SET,
            subject: "frob-fsim (contract.toml)".to_owned(),
            path: contract_rel.to_owned(),
            line: None,
            message: format!(
                "`frob-fsim/contract.toml` marks default airlock groups {:?}, expected exactly {:?} (INV-SUITE-01)",
                contract_defaults.iter().collect::<Vec<_>>(),
                CANONICAL_DEFAULT_AIRLOCK_GROUPS
            ),
            repair: "set `default = true` exclusively on `referee`, `geometry`, and `ledger` in crates/frob-fsim/contract.toml".to_owned(),
        });
    }

    if manifest_default_groups != contract_defaults {
        for g in manifest_default_groups.difference(&contract_defaults) {
            bad_groups.insert(g.clone());
        }
        findings.push(Finding {
            rule: RULE_FSIM_DEFAULT_SET,
            subject: "frob-fsim (default)".to_owned(),
            path: fsim_pkg.rel_manifest.clone(),
            line: None,
            message: format!(
                "`frob-fsim` manifest `default` enables airlock groups {:?}, expected exactly {:?} (non-group features such as `mode-inproc` are excluded; INV-SUITE-01)",
                manifest_default_groups.iter().collect::<Vec<_>>(),
                contract_defaults.iter().collect::<Vec<_>>()
            ),
            repair: format!(
                "update `[features].default` in `{}` so its airlock groups are exactly {:?}",
                fsim_pkg.rel_manifest,
                contract_defaults.iter().collect::<Vec<_>>()
            ),
        });
    }
}

#[derive(Default)]
struct GroupUsage {
    via_normal_or_build: bool,
    via_dev: bool,
    plain_normal_or_build: bool,
    default_feature_chains: Vec<Vec<String>>,
}

fn check_member_airlock_usage(
    member_name: &str,
    rel_manifest: &str,
    deps: &[DeclaredDep],
    features: &BTreeMap<String, Vec<String>>,
    groups: &BTreeMap<String, AirlockGroupRecord>,
    attributed_optional_groups: &mut BTreeSet<String>,
    findings: &mut Vec<Finding>,
) -> Vec<String> {
    let fsim_deps: Vec<&DeclaredDep> = deps.iter().filter(|d| d.name == "frob-fsim").collect();
    if fsim_deps.is_empty() {
        return Vec::new();
    }

    let default_groups: BTreeSet<String> = groups
        .values()
        .filter(|g| g.default)
        .map(|g| g.name.clone())
        .collect();

    let mut aliases: BTreeMap<String, (bool, bool, bool)> = BTreeMap::new();
    // alias -> (has_normal_or_build, has_dev, active_by_default)
    let mut usage: BTreeMap<String, GroupUsage> = BTreeMap::new();

    let default_closure = expand_local_feature_closure(features, "default");

    for dep in &fsim_deps {
        let alias = dep.rename.as_deref().unwrap_or(&dep.name).to_owned();
        let is_dev = dep.kind == DepKind::Dev;
        let is_plain_active = !is_dev && !dep.optional;
        let opt_enabled_by_default = !is_dev
            && dep.optional
            && (default_closure.contains_key(&alias)
                || default_closure.values().any(|chain| {
                    chain.last().is_some_and(|f| {
                        features.get(f).is_some_and(|items| {
                            items.iter().any(|it| {
                                it == &alias
                                    || it == &format!("dep:{alias}")
                                    || it.starts_with(&format!("{alias}/"))
                            })
                        })
                    })
                }));

        let entry = aliases.entry(alias).or_insert((false, false, false));
        if is_dev {
            entry.1 = true;
        } else {
            entry.0 = true;
        }
        if is_plain_active || opt_enabled_by_default {
            entry.2 = true;
        }

        // Explicit `features = [...]` on the dependency declaration.
        for feat in &dep.features {
            if groups.contains_key(feat) {
                let u = usage.entry(feat.clone()).or_default();
                if is_dev {
                    u.via_dev = true;
                } else {
                    u.via_normal_or_build = true;
                    if !dep.optional {
                        u.plain_normal_or_build = true;
                    } else if opt_enabled_by_default {
                        u.default_feature_chains
                            .push(vec!["default".to_owned(), format!("dep:{}", dep.name)]);
                    }
                }
            }
        }

        // Implicit default groups when `default-features = true` (Cargo's default).
        if dep.uses_default_features {
            for dg in &default_groups {
                let u = usage.entry(dg.clone()).or_default();
                if is_dev {
                    u.via_dev = true;
                } else {
                    u.via_normal_or_build = true;
                }
            }
        }
    }

    // Feature-table forwarding (`<alias>/<group>` or `<alias>?/<group>`).
    for (feat_name, items) in features {
        for item in items {
            let Some((dep_key, weak, target_feat)) = split_dep_feature_ref(item) else {
                continue;
            };
            let Some(&(has_nb, has_dev, active_by_default)) = aliases.get(dep_key) else {
                continue;
            };
            if !groups.contains_key(target_feat) {
                continue;
            }
            let u = usage.entry(target_feat.to_owned()).or_default();
            if has_nb {
                u.via_normal_or_build = true;
            }
            if has_dev {
                u.via_dev = true;
            }
            if let Some(chain) = default_closure.get(feat_name) {
                // Non-weak `<alias>/<group>` activates `<alias>` even if optional; weak `<alias>?/<group>`
                // activates the group by default only if `<alias>` is active by default.
                if has_nb && (!weak || active_by_default) {
                    u.default_feature_chains.push(chain.clone());
                }
            }
        }
    }

    let mut enabled_groups = Vec::with_capacity(usage.len());
    for (group_name, u) in usage {
        enabled_groups.push(group_name.clone());
        let Some(rec) = groups.get(&group_name) else {
            continue;
        };

        let in_consumers = rec.consumers.iter().any(|c| c == member_name);
        let in_test_consumers = rec.test_consumers.iter().any(|c| c == member_name);
        let allowed = if rec.dev_only {
            !u.via_normal_or_build && u.via_dev && (in_test_consumers || in_consumers)
        } else {
            in_consumers || in_test_consumers
        };

        if !allowed {
            attributed_optional_groups.insert(group_name.clone());
            let allowed_desc = if rec.dev_only {
                format!(
                    "dev-dependencies of {:?} only (dev_only = true)",
                    rec.test_consumers
                )
            } else {
                let mut combined = rec.consumers.clone();
                for tc in &rec.test_consumers {
                    if !combined.contains(tc) {
                        combined.push(tc.clone());
                    }
                }
                format!("{combined:?}")
            };
            findings.push(Finding {
                rule: RULE_FSIM_GROUP_NOT_ALLOWED,
                subject: format!("{member_name} -> frob-fsim/{group_name}"),
                path: rel_manifest.to_owned(),
                line: None,
                message: format!(
                    "crate `{member_name}` enables airlock feature group `{group_name}` of `frob-fsim`, which is allowed only for {allowed_desc} per crates/frob-fsim/contract.toml (INV-SUITE-01)"
                ),
                repair: format!(
                    "remove `frob-fsim/{group_name}` from `{rel_manifest}` or record an ADR/admission update in crates/frob-fsim/contract.toml"
                ),
            });
            // One finding per defect: never also report `fsim.optional-default-on` for an unlisted crate.
            continue;
        }

        // Rule 3c: `fsim.optional-default-on` — listed consumer turning an off-by-default,
        // non-dev-only group on for every default build.
        if !rec.default && !rec.dev_only {
            if u.plain_normal_or_build {
                attributed_optional_groups.insert(group_name.clone());
                findings.push(Finding {
                    rule: RULE_FSIM_OPTIONAL_DEFAULT_ON,
                    subject: format!("{member_name} -> frob-fsim/{group_name}"),
                    path: rel_manifest.to_owned(),
                    line: None,
                    message: format!(
                        "listed consumer `{member_name}` enables optional airlock group `{group_name}` in the `features` list of its plain declaration on `frob-fsim` (INV-SUITE-01)"
                    ),
                    repair: format!(
                        "move `{group_name}` out of the plain `frob-fsim` dependency in `{rel_manifest}` behind an opt-in non-default feature (e.g. `airlock-{group_name} = [\"frob-fsim/{group_name}\"]`)"
                    ),
                });
            } else if !u.default_feature_chains.is_empty() {
                attributed_optional_groups.insert(group_name.clone());
                let mut chains = u.default_feature_chains;
                chains.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
                let chain = &chains[0];
                let leaf_feat = chain.last().map(String::as_str).unwrap_or("default");
                let chain_str = chain.join(" -> ");
                findings.push(Finding {
                    rule: RULE_FSIM_OPTIONAL_DEFAULT_ON,
                    subject: format!("{member_name} -> frob-fsim/{group_name} ({leaf_feat})"),
                    path: rel_manifest.to_owned(),
                    line: None,
                    message: format!(
                        "listed consumer `{member_name}` enables optional airlock group `{group_name}` by default through feature `{leaf_feat}` (chain `{chain_str} -> frob-fsim/{group_name}`; INV-SUITE-01)"
                    ),
                    repair: format!(
                        "remove `{leaf_feat}` (or its parent in `{chain_str}`) from `[features].default` in `{rel_manifest}` so optional group `{group_name}` stays off by default"
                    ),
                });
            } else if u.via_dev && !u.via_normal_or_build {
                // Legitimate dev-only usage in a listed consumer/test_consumer activates the
                // feature in `cargo metadata`'s unified workspace resolve; do not misreport it
                // as a default normal-build leak.
                attributed_optional_groups.insert(group_name.clone());
            }
        }
    }

    enabled_groups.sort();
    enabled_groups.dedup();
    enabled_groups
}

/// Expand all local features reachable from `start_feature` (inclusive) via deterministic BFS,
/// returning `feature_name -> shortest chain starting at start_feature`.
#[must_use]
pub fn expand_local_feature_closure(
    features: &BTreeMap<String, Vec<String>>,
    start_feature: &str,
) -> BTreeMap<String, Vec<String>> {
    let mut chains: BTreeMap<String, Vec<String>> = BTreeMap::new();
    if !features.contains_key(start_feature) {
        return chains;
    }
    let mut queue: VecDeque<String> = VecDeque::new();
    chains.insert(start_feature.to_owned(), vec![start_feature.to_owned()]);
    queue.push_back(start_feature.to_owned());

    while let Some(cur) = queue.pop_front() {
        let cur_chain = chains.get(&cur).cloned().unwrap_or_default();
        let Some(items) = features.get(&cur) else {
            continue;
        };
        let mut sorted_items: Vec<&str> = items.iter().map(String::as_str).collect();
        sorted_items.sort();
        for item in sorted_items {
            if item.contains('/') || item.starts_with("dep:") {
                continue;
            }
            if features.contains_key(item) && !chains.contains_key(item) {
                let mut next_chain = cur_chain.clone();
                next_chain.push(item.to_owned());
                chains.insert(item.to_owned(), next_chain);
                queue.push_back(item.to_owned());
            }
        }
    }
    chains
}

/// Parse `<dep>/<feat>` or `<dep>?/<feat>` into `(dep_key, is_weak, feat_name)`.
#[must_use]
pub fn split_dep_feature_ref(item: &str) -> Option<(&str, bool, &str)> {
    if let Some((dep, feat)) = item.split_once("?/") {
        if !dep.is_empty() && !feat.is_empty() {
            return Some((dep, true, feat));
        }
        return None;
    }
    if let Some((dep, feat)) = item.split_once('/')
        && !dep.is_empty()
        && !feat.is_empty()
    {
        return Some((dep, false, feat));
    }
    None
}

// ============================================================================
// 3. Tier features (L-12, INV-WS-04): `tier.default-on` and `tier.unnamed`
// ============================================================================

/// Return the tier prefix (`"frontier-"` or `"moonshot-"`) if `feat` is a tier feature.
#[must_use]
pub fn tier_feature_prefix(feat: &str) -> Option<&'static str> {
    if feat.starts_with("frontier-") {
        Some("frontier-")
    } else if feat.starts_with("moonshot-") {
        Some("moonshot-")
    } else {
        None
    }
}

/// Evaluate rule L-12 (`tier.default-on` and `tier.unnamed`, INV-WS-04) across a workspace.
pub fn evaluate_tier_features(
    ws: &WorkspaceRoot,
    declared_members: &[PackageMeta],
    resolved_default: &ResolvedWorkspace,
    crate_map: &[CrateRow],
    findings: &mut Vec<Finding>,
) {
    let mut reported_pkg_feats: BTreeSet<(String, String)> = BTreeSet::new();

    for member in declared_members {
        let default_closure = expand_local_feature_closure(&member.features, "default");
        let is_binary_crate = !member.bin_targets.is_empty();

        // Condition 1: any `frontier-*` / `moonshot-*` feature in `default` (directly or transitively).
        for (feat_name, chain) in &default_closure {
            if tier_feature_prefix(feat_name).is_some() {
                reported_pkg_feats.insert((member.name.clone(), feat_name.clone()));
                findings.push(Finding {
                    rule: RULE_TIER_DEFAULT_ON,
                    subject: format!("{} [{feat_name}]", member.name),
                    path: member.rel_manifest.clone(),
                    line: None,
                    message: format!(
                        "tier feature `{feat_name}` is enabled by `default` in `{}` (chain `{}`; rule L-12, INV-WS-04)",
                        member.name,
                        chain.join(" -> ")
                    ),
                    repair: format!(
                        "remove `{feat_name}` from `[features].default` in `{}` so Frontier/Moonshot features stay off by default",
                        member.rel_manifest
                    ),
                });
            }
            if let Some(items) = member.features.get(feat_name) {
                for item in items {
                    if let Some((dep_key, _, target_feat)) = split_dep_feature_ref(item)
                        && tier_feature_prefix(target_feat).is_some()
                    {
                        let target_pkg = resolve_dep_alias_name(&member.dependencies, dep_key);
                        reported_pkg_feats.insert((target_pkg.clone(), target_feat.to_owned()));
                        findings.push(Finding {
                            rule: RULE_TIER_DEFAULT_ON,
                            subject: format!("{} -> {target_pkg}/{target_feat}", member.name),
                            path: member.rel_manifest.clone(),
                            line: None,
                            message: format!(
                                "default feature chain `{}` in `{}` enables tier feature `{target_pkg}/{target_feat}` (rule L-12, INV-WS-04)",
                                chain.join(" -> "),
                                member.name
                            ),
                            repair: format!(
                                "gate `{target_pkg}/{target_feat}` behind an opt-in non-default tier feature in `{}`",
                                member.rel_manifest
                            ),
                        });
                    }
                }
            }
        }

        // Condition 2: inside the core workspace, a dependency declaration enables a
        // `frontier-*` / `moonshot-*` feature without an explicit feature of the same tier
        // on the depending crate (binaries may also opt in via a non-default feature).
        if ws.kind == WorkspaceKind::Core {
            for dep in &member.dependencies {
                if dep.kind == DepKind::Dev || dep.optional {
                    continue;
                }
                for feat in &dep.features {
                    if tier_feature_prefix(feat).is_some() {
                        reported_pkg_feats.insert((dep.name.clone(), feat.clone()));
                        findings.push(Finding {
                            rule: RULE_TIER_DEFAULT_ON,
                            subject: format!("{} -> {}/{feat}", member.name, dep.name),
                            path: member.rel_manifest.clone(),
                            line: None,
                            message: format!(
                                "core crate `{}` enables tier feature `{}/{feat}` directly on a {} dependency without an explicit same-tier feature (rule L-12, INV-WS-04)",
                                member.name, dep.name, dep.kind
                            ),
                            repair: format!(
                                "remove `{feat}` from the plain dependency on `{}` in `{}` and gate it behind a non-default feature of the same tier",
                                dep.name, member.rel_manifest
                            ),
                        });
                    }
                }
            }

            for (feat_name, items) in &member.features {
                if feat_name == "default" || default_closure.contains_key(feat_name) {
                    continue;
                }
                for item in items {
                    if let Some((dep_key, _, target_feat)) = split_dep_feature_ref(item)
                        && let Some(req_prefix) = tier_feature_prefix(target_feat)
                    {
                        let same_tier = feat_name.starts_with(req_prefix);
                        if !same_tier && !is_binary_crate {
                            let target_pkg = resolve_dep_alias_name(&member.dependencies, dep_key);
                            reported_pkg_feats.insert((target_pkg.clone(), target_feat.to_owned()));
                            findings.push(Finding {
                                rule: RULE_TIER_DEFAULT_ON,
                                subject: format!(
                                    "{} [{feat_name}] -> {target_pkg}/{target_feat}",
                                    member.name
                                ),
                                path: member.rel_manifest.clone(),
                                line: None,
                                message: format!(
                                    "core crate `{}` enables tier feature `{target_pkg}/{target_feat}` through feature `{feat_name}` which lacks the `{req_prefix}` tier prefix (rule L-12, INV-WS-04)",
                                    member.name
                                ),
                                repair: format!(
                                    "rename `{feat_name}` to `{req_prefix}...` in `{}` or move the opt-in to a binary's non-default feature",
                                    member.rel_manifest
                                ),
                            });
                        }
                    }
                }
            }
        }

        // Rule `tier.unnamed`: a crate whose crate-map tier includes Frontier or Moonshot
        // has a `contract.toml` feature of tier F or M lacking the required prefix.
        check_member_tier_unnamed(member, crate_map, findings);
    }

    // Condition 3: any `frontier-*` / `moonshot-*` feature active in the default-feature
    // resolution of any binary target.
    let members_by_id: BTreeMap<&str, &PackageMeta> = declared_members
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect();
    for member_id in &resolved_default.workspace_members {
        let Some(bin_pkg) = resolved_default.packages.get(member_id) else {
            continue;
        };
        if bin_pkg.bin_targets.is_empty() {
            continue;
        }
        let parent = bfs_normal_build_parents(resolved_default, member_id);
        let rel_manifest = members_by_id
            .get(member_id.as_str())
            .map(|m| m.rel_manifest.clone())
            .unwrap_or_else(|| "Cargo.toml".to_owned());

        for (&node_id, _) in &parent {
            let Some(node) = resolved_default.nodes.get(node_id) else {
                continue;
            };
            let Some(target_pkg) = resolved_default.packages.get(node_id) else {
                continue;
            };
            for feat in &node.features {
                if tier_feature_prefix(feat).is_some()
                    && !reported_pkg_feats.contains(&(target_pkg.name.clone(), feat.clone()))
                {
                    let path = reconstruct_path_from_parents(node_id, &parent, resolved_default);
                    for bin_name in &bin_pkg.bin_targets {
                        findings.push(Finding {
                            rule: RULE_TIER_DEFAULT_ON,
                            subject: format!("{bin_name} -> {} [{feat}]", target_pkg.name),
                            path: rel_manifest.clone(),
                            line: None,
                            message: format!(
                                "binary `{bin_name}` enables tier feature `{} [{feat}]` in its default-feature resolution via `{}` (rule L-12, INV-WS-04)",
                                target_pkg.name,
                                path.join(" -> ")
                            ),
                            repair: format!(
                                "disable `{feat}` by default along `{}` so default binary builds contain only Solid features",
                                path.join(" -> ")
                            ),
                        });
                    }
                }
            }
        }
    }
}

fn resolve_dep_alias_name(deps: &[DeclaredDep], dep_key: &str) -> String {
    deps.iter()
        .find(|d| d.rename.as_deref() == Some(dep_key) || d.name == dep_key)
        .map_or_else(|| dep_key.to_owned(), |d| d.name.clone())
}

fn check_member_tier_unnamed(
    member: &PackageMeta,
    crate_map: &[CrateRow],
    findings: &mut Vec<Finding>,
) {
    let Some(row) = lookup_in(crate_map, &member.name) else {
        return;
    };
    let has_fm_tier = row
        .tiers
        .iter()
        .any(|t| matches!(t, Tier::Frontier | Tier::Moonshot));
    if !has_fm_tier {
        return;
    }

    let contract_path = member.crate_dir().join("contract.toml");
    let Ok(bytes) = fs::read(&contract_path) else {
        return;
    };
    let Ok(root) = toml::parse(&bytes) else {
        return;
    };
    let Some(Value::Table(features_tbl)) = root.get("features") else {
        return;
    };

    let rel_contract = if member.rel_dir.is_empty() {
        "contract.toml".to_owned()
    } else {
        format!("{}/contract.toml", member.rel_dir)
    };

    for (fname, sp) in features_tbl.iter() {
        let Value::Table(ftbl) = &sp.value else {
            continue;
        };
        let Some(tier) = ftbl.get("tier").and_then(Value::as_str) else {
            continue;
        };
        if tier == "F" && !fname.starts_with("frontier-") {
            findings.push(Finding {
                rule: RULE_TIER_UNNAMED,
                subject: format!("{} [{fname}]", member.name),
                path: rel_contract.clone(),
                line: Some(sp.line),
                message: format!(
                    "tier-F feature `{fname}` in `{}` lacks the required `frontier-` prefix (rule L-12, INV-WS-04)",
                    member.name
                ),
                repair: format!(
                    "rename `{fname}` to `frontier-{fname}` in both `{}` and `{rel_contract}`",
                    member.rel_manifest
                ),
            });
        } else if tier == "M" && !fname.starts_with("moonshot-") {
            findings.push(Finding {
                rule: RULE_TIER_UNNAMED,
                subject: format!("{} [{fname}]", member.name),
                path: rel_contract.clone(),
                line: Some(sp.line),
                message: format!(
                    "tier-M feature `{fname}` in `{}` lacks the required `moonshot-` prefix (rule L-12, INV-WS-04)",
                    member.name
                ),
                repair: format!(
                    "rename `{fname}` to `moonshot-{fname}` in both `{}` and `{rel_contract}`",
                    member.rel_manifest
                ),
            });
        }
    }
}

// ============================================================================
// 4. All-features resolution (`resolve.all-features`)
// ============================================================================

/// Construct a `resolve.all-features` finding from Cargo's offline resolution error,
/// extracting the offending package name and conflicting requirement summary.
#[must_use]
pub fn build_resolve_all_features_finding(ws: &WorkspaceRoot, detail: &str) -> Finding {
    let manifest_rel = if ws.rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        format!("{}/Cargo.toml", ws.rel_dir)
    };
    let offending_pkg = extract_conflicting_package_name(detail);
    let subject = match offending_pkg.as_deref() {
        Some(pkg) => format!("{} -> {pkg}", ws.name),
        None => ws.name.clone(),
    };
    let compact = detail
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("Blocking waiting for file lock"))
        .collect::<Vec<_>>()
        .join(" | ");

    Finding {
        rule: RULE_RESOLVE_ALL_FEATURES,
        subject,
        path: manifest_rel,
        line: None,
        message: format!(
            "offline `--all-features` resolution failed for workspace `{}`: {compact}",
            ws.name
        ),
        repair: match offending_pkg {
            Some(pkg) => format!(
                "reconcile the conflicting version/feature requirements on `{pkg}` across workspace `{}` and refresh its committed Cargo.lock offline",
                ws.name
            ),
            None => format!(
                "reconcile the conflicting optional-feature requirements in workspace `{}` and refresh its committed Cargo.lock offline",
                ws.name
            ),
        },
    }
}

fn extract_conflicting_package_name(stderr: &str) -> Option<String> {
    for needle in [
        "failed to select a version for `",
        "no matching package named `",
        "native library `",
        "Package `",
        "package `",
    ] {
        if let Some(idx) = stderr.find(needle) {
            let after = &stderr[idx + needle.len()..];
            if let Some(end) = after.find('`') {
                let raw = after[..end].trim();
                let name = raw.split_whitespace().next().unwrap_or(raw);
                if !name.is_empty() {
                    return Some(name.to_owned());
                }
            }
        }
    }
    None
}
