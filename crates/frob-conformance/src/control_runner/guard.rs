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

#[derive(Debug, Clone)]
struct ParsedCrateManifest {
    name: String,
    dir: PathBuf,
    features: BTreeMap<String, Vec<String>>,
    bins: Vec<String>,
    dep_features: Vec<(String, Vec<String>)>,
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
        let Ok((crate_name, feat_name)) = resolve_crate_and_feature(ctrl) else {
            // Unbound placeholder targets (e.g. `N24b-01:...`) belong to future milestones.
            continue;
        };
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

        // Feature must transitively imply `conformance`.
        let closure = feature_closure(&manifest.features, &feat_name);
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
        let default_closure = feature_closure(&manifest.features, "default");
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

    // 2. Audit every workspace crate declaring `conformance` or mutation features implying it.
    let mut product_binaries_checked = 0usize;
    for manifest in &manifests {
        let default_closure = feature_closure(&manifest.features, "default");
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
                        && feature_closure(&manifest.features, k).contains("conformance")
                })
                .collect();

            for mf in &implying_conformance {
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

        // 3. Check product binaries (`frob`, `frobd`, `frob-mcp`, `frob-verify`).
        let matched_bins: Vec<&str> = manifest
            .bins
            .iter()
            .map(String::as_str)
            .filter(|b| PRODUCT_BINARIES.contains(b))
            .collect();
        if !matched_bins.is_empty() || PRODUCT_BINARIES.contains(&manifest.name.as_str()) {
            product_binaries_checked += matched_bins.len().max(1);
            if manifest.features.contains_key("conformance") {
                violations.push(GuardViolation {
                    crate_name: manifest.name.clone(),
                    feature: "conformance".to_owned(),
                    rule: "product-binary-conformance".to_owned(),
                    detail: format!(
                        "product binary crate `{}` must not declare or carry `conformance`",
                        manifest.name
                    ),
                });
            }
            for (dep_name, dep_feats) in &manifest.dep_features {
                for df in dep_feats {
                    let bare = df.split_once('/').map_or(df.as_str(), |(_, f)| f);
                    if bare == "conformance"
                        || bare == "gauntlet-mutations"
                        || all_mutation_features.contains(bare)
                    {
                        violations.push(GuardViolation {
                            crate_name: manifest.name.clone(),
                            feature: format!("{dep_name}/{bare}"),
                            rule: "product-binary-mutation-feature".to_owned(),
                            detail: format!(
                                "product binary crate `{}` enables forbidden feature `{bare}` on dependency `{dep_name}`",
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
    cmd.env("CARGO_TARGET_DIR", target_dir);

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

fn feature_closure(features: &BTreeMap<String, Vec<String>>, root: &str) -> BTreeSet<String> {
    let mut visited = BTreeSet::new();
    let mut stack = vec![root.to_owned()];
    while let Some(cur) = stack.pop() {
        if !visited.insert(cur.clone()) {
            continue;
        }
        if let Some(children) = features.get(&cur) {
            for ch in children {
                let target = ch.split_once('/').map_or(ch.as_str(), |(_, f)| f);
                if !visited.contains(target) {
                    stack.push(target.to_owned());
                }
            }
        }
    }
    visited
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

        let mut dep_features = Vec::new();
        for dep_sec in ["dependencies", "build-dependencies"] {
            if let Some(dtbl) = tbl.get(dep_sec).and_then(TomlValue::as_table) {
                for (dep_name, dep_val) in dtbl.iter() {
                    if let Some(inner) = dep_val.value.as_table()
                        && let Some(farr) = inner.get("features").and_then(TomlValue::as_array)
                    {
                        let fs_list: Vec<String> = farr
                            .iter()
                            .filter_map(|sp| sp.value.as_str().map(str::to_owned))
                            .collect();
                        if !fs_list.is_empty() {
                            dep_features.push((dep_name.to_owned(), fs_list));
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
            dep_features,
        });
    }

    // Fallback line parser for Cargo.toml files that use full TOML constructs (e.g. dotted keys)
    // outside the strict registry TOML subset.
    Ok(parse_crate_manifest_fallback(&raw, crate_dir))
}

fn parse_crate_manifest_fallback(raw: &str, crate_dir: &Path) -> ParsedCrateManifest {
    let mut name = crate_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_owned();
    let mut features: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut bins = Vec::new();
    let mut dep_features = Vec::new();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Sec {
        None,
        Package,
        Features,
        Bin,
        Deps,
    }
    let mut sec = Sec::None;
    let mut pending_feat: Option<(String, String)> = None;

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
        if line.starts_with('[') && line.ends_with(']') {
            sec = match line {
                "[package]" => Sec::Package,
                "[features]" => Sec::Features,
                "[[bin]]" => Sec::Bin,
                "[dependencies]" | "[build-dependencies]" => Sec::Deps,
                _ => Sec::None,
            };
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let key = k.trim().trim_matches('"').trim_matches('\'');
        let val = v.trim();
        match sec {
            Sec::Package if key == "name" => {
                name = val.trim_matches('"').trim_matches('\'').to_owned();
            }
            Sec::Bin if key == "name" => {
                bins.push(val.trim_matches('"').trim_matches('\'').to_owned());
            }
            Sec::Features => {
                if val.contains('[') && !val.contains(']') {
                    pending_feat = Some((key.to_owned(), val.to_owned()));
                } else {
                    features.insert(key.to_owned(), parse_bracket_strings(val));
                }
            }
            Sec::Deps => {
                if let Some(idx) = val.find("features")
                    && let Some(sub) = val[idx..].find('[')
                {
                    let slice = &val[idx + sub..];
                    let list = parse_bracket_strings(slice);
                    if !list.is_empty() {
                        dep_features.push((key.to_owned(), list));
                    }
                }
            }
            _ => {}
        }
    }

    ParsedCrateManifest {
        name,
        dir: crate_dir.to_path_buf(),
        features,
        bins,
        dep_features,
    }
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
