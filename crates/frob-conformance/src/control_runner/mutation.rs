#![deny(unsafe_code)]

//! Mutation-control routing, manifest feature verification, and `cargo test`
//! invocation (`N38b-04`).
//!
//! Enforces:
//! - A `mutation` control is built with **exactly** the feature its record names
//!   (`cx-262` for a seeded control, `nc-<nnnn>` for one registered directly),
//!   one control per build.
//! - A control whose target is a law suite (`crates/<crate>/tests/laws_c<nn>.rs`)
//!   is built in its owning crate via `cargo test -p <crate> --features <feature> --test laws_c<nn>`
//!   (without `gauntlet-mutations`, except when staged inside `frob-conformance`).
//! - A control whose target names an annex workspace is routed through `DRIVERS`
//!   (or reported `stale` if none is registered).
//! - Always invokes `cargo` through PATH (`rch` build offload), never by absolute path.

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use frob_core::testkit::registry::Control;

use super::{ControlEval, detect_annex_workspace};

/// Resolved execution route for one `mutation` control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationRoute {
    /// Build a tier target of `frob-conformance` with `--features gauntlet-mutations,<crate>/<feature>`.
    WorkspaceTier {
        crate_name: String,
        feature: String,
        tier_target: String,
        gate_filter: String,
    },
    /// Build a law suite `tests/laws_c<nn>.rs` in its owning crate without `gauntlet-mutations`
    /// (unless `crate_name == "frob-conformance"`).
    LawSuite {
        crate_name: String,
        feature: String,
        test_target: String,
        test_name: Option<String>,
    },
    /// Target belongs to an annex workspace dispatched through `control_runner::DRIVERS`.
    Annex { workspace: String, target: String },
    /// M0 host stand-in inside `frob-conformance` before the owning M1 crate exists.
    HostStandIn { crate_name: String, feature: String },
}

/// Parsed law-suite target descriptor (`crates/<crate>/tests/laws_c<nn>.rs[::test_fn]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LawSuiteSpec {
    pub crate_name: String,
    pub test_target: String,
    pub test_name: Option<String>,
}

/// Parse a law-suite target path or module reference into `(crate_name, test_target, test_name)`.
///
/// Accepts:
/// - `crates/<crate>/tests/laws_c<nn>.rs`
/// - `crates/<crate>/tests/laws_c<nn>.rs::<test_fn>`
/// - `<crate>::tests::laws_c<nn>` or `<crate>::tests::laws_c<nn>::<test_fn>`
#[must_use]
pub fn parse_law_suite_target(target: &str) -> Option<LawSuiteSpec> {
    let trimmed = target.trim();

    if let Some(rest) = trimmed.strip_prefix("crates/") {
        let (path_part, test_name) = match rest.split_once("::") {
            Some((p, t)) => (
                p.trim(),
                Some(t.trim().to_owned()).filter(|s| !s.is_empty()),
            ),
            None => (rest, None),
        };
        let (crate_dir, tail) = path_part.split_once("/tests/")?;
        let stem = tail.strip_suffix(".rs")?;
        if is_law_suite_stem(stem) && !crate_dir.is_empty() {
            return Some(LawSuiteSpec {
                crate_name: crate_dir.replace('_', "-"),
                test_target: stem.to_owned(),
                test_name,
            });
        }
        return None;
    }

    let segs: Vec<&str> = trimmed.split("::").collect();
    if segs.len() >= 3 && segs[1] == "tests" && is_law_suite_stem(segs[2]) {
        let test_name = if segs.len() >= 4 {
            Some(segs[3..].join("::"))
        } else {
            None
        };
        return Some(LawSuiteSpec {
            crate_name: segs[0].trim().replace('_', "-"),
            test_target: segs[2].to_owned(),
            test_name,
        });
    }

    None
}

fn is_law_suite_stem(stem: &str) -> bool {
    let Some(digits) = stem.strip_prefix("laws_c") else {
        return false;
    };
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
}

/// Resolve the exact `(crate_name, feature_name)` pair from a control's `target` and `feature`.
///
/// Preserves the exact feature spelling written in `control.feature` (e.g. `cx-262`
/// or `nc-0399`); never rewrites a plant feature name to `nc-<nnnn>`.
pub fn resolve_crate_and_feature(control: &Control) -> Result<(String, String), String> {
    let raw_feat = control
        .feature
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("mutation control `{}` lacks `feature` field", control.id))?;

    if let Some((c, f)) = raw_feat.split_once('/') {
        let cn = c.trim().replace('_', "-");
        let fnm = f.trim().to_owned();
        if cn.is_empty() || fnm.is_empty() {
            return Err(format!(
                "malformed crate/feature `{raw_feat}` on control `{}`",
                control.id
            ));
        }
        return Ok((cn, fnm));
    }

    if let Some(law) = parse_law_suite_target(&control.target) {
        return Ok((law.crate_name, raw_feat.to_owned()));
    }

    let target_crate = control
        .target
        .split("::")
        .next()
        .unwrap_or("")
        .trim()
        .replace('_', "-");
    if target_crate.is_empty() || target_crate.contains(':') {
        return Err(format!(
            "cannot infer owning crate from target `{}` on control `{}`",
            control.target, control.id
        ));
    }
    Ok((target_crate, raw_feat.to_owned()))
}

/// Resolve the execution route for a `mutation` control.
pub fn resolve_mutation_route(
    workspace_root: &Path,
    control: &Control,
    gate_id: &str,
    tier_target: &str,
    allow_m0_host_standin: bool,
) -> Result<MutationRoute, String> {
    if let Some(ws) = detect_annex_workspace(&control.target) {
        return Ok(MutationRoute::Annex {
            workspace: ws,
            target: control.target.clone(),
        });
    }

    let (crate_name, feature) = resolve_crate_and_feature(control)?;

    if let Some(law) = parse_law_suite_target(&control.target) {
        return Ok(MutationRoute::LawSuite {
            crate_name: law.crate_name,
            feature,
            test_target: law.test_target,
            test_name: law.test_name,
        });
    }

    // M0 host stand-in for NC-0412 (`cx-262`) before `crates/frob-actuate` exists.
    if allow_m0_host_standin
        && control.id == "NC-0412"
        && crate_name == "frob-actuate"
        && feature == "cx-262"
        && find_crate_manifest(workspace_root, "frob-actuate").is_none()
        && find_crate_manifest(workspace_root, "frob-conformance").is_some()
    {
        return Ok(MutationRoute::HostStandIn {
            crate_name: "frob-conformance".to_owned(),
            feature,
        });
    }

    Ok(MutationRoute::WorkspaceTier {
        crate_name,
        feature,
        tier_target: tier_target.to_owned(),
        gate_filter: gate_id.to_owned(),
    })
}

/// Check that `crates/<crate_name>/Cargo.toml` exists under `workspace_root` and
/// declares the exact feature key `exact_feature` under `[features]`.
pub fn check_manifest_declares_feature(
    workspace_root: &Path,
    crate_name: &str,
    exact_feature: &str,
) -> Result<(), String> {
    let manifest_path = find_crate_manifest(workspace_root, crate_name).ok_or_else(|| {
        format!(
            "crate `{crate_name}` manifest not found under `{}` for mutation feature `{exact_feature}`",
            workspace_root.display()
        )
    })?;
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("failed to read `{}`: {e}", manifest_path.display()))?;
    let declared = parse_manifest_feature_names(&content);
    if declared.contains(exact_feature) {
        Ok(())
    } else {
        Err(format!(
            "mutation feature `{exact_feature}` is not declared in `{}` (declared: {:?})",
            manifest_path.display(),
            declared
        ))
    }
}

/// Inspect the exact `cargo` argument vector for `route` (one control per build).
#[must_use]
pub fn inspect_cargo_args(
    workspace_root: &Path,
    route: &MutationRoute,
    offline: bool,
) -> Vec<String> {
    let manifest = workspace_root.join("Cargo.toml");
    let mut args = vec![
        "test".to_owned(),
        "--manifest-path".to_owned(),
        manifest.to_string_lossy().into_owned(),
    ];
    if offline {
        args.push("--offline".to_owned());
    }
    match route {
        MutationRoute::WorkspaceTier {
            crate_name,
            feature,
            tier_target,
            gate_filter,
        } => {
            args.push("-p".to_owned());
            args.push("frob-conformance".to_owned());
            args.push("--features".to_owned());
            if crate_name == "frob-conformance" {
                args.push(format!("gauntlet-mutations,{feature}"));
            } else {
                args.push(format!("gauntlet-mutations,{crate_name}/{feature}"));
            }
            args.push("--test".to_owned());
            args.push(tier_target.clone());
            if !gate_filter.is_empty() {
                args.push("--".to_owned());
                args.push(gate_filter.clone());
            }
        }
        MutationRoute::LawSuite {
            crate_name,
            feature,
            test_target,
            test_name,
        } => {
            args.push("-p".to_owned());
            args.push(crate_name.clone());
            args.push("--features".to_owned());
            if crate_name == "frob-conformance" {
                args.push(format!("gauntlet-mutations,{feature}"));
            } else {
                args.push(feature.clone());
            }
            args.push("--test".to_owned());
            args.push(test_target.clone());
            if let Some(tn) = test_name {
                args.push("--".to_owned());
                args.push(tn.clone());
            }
        }
        MutationRoute::HostStandIn {
            crate_name,
            feature,
        } => {
            args.push("-p".to_owned());
            args.push(crate_name.clone());
            args.push("--features".to_owned());
            args.push(format!("gauntlet-mutations,{feature}"));
            args.push("--test".to_owned());
            args.push("control_runner".to_owned());
            args.push("--".to_owned());
            args.push("wheel_loop_host_standin_row".to_owned());
        }
        MutationRoute::Annex { .. } => {}
    }
    args
}

/// Construct a `std::process::Command` invoking PATH `cargo` (never an absolute path)
/// with an isolated `CARGO_TARGET_DIR`.
#[must_use]
pub fn build_cargo_command(
    workspace_root: &Path,
    route: &MutationRoute,
    target_dir: Option<&Path>,
    offline: bool,
) -> Command {
    let args = inspect_cargo_args(workspace_root, route, offline);
    let mut cmd = Command::new("cargo");
    cmd.args(&args).current_dir(workspace_root);
    let isolated_target = target_dir.map(PathBuf::from).unwrap_or_else(|| {
        let base = env::var_os("RCH_TARGET_BASE")
            .or_else(|| env::var_os("TMPDIR"))
            .map(PathBuf::from)
            .unwrap_or_else(env::temp_dir);
        base.join("frob-control-runner")
    });
    cmd.env("CARGO_TARGET_DIR", isolated_target);
    cmd
}

/// Execute a single `mutation` control by building and running its route with `cargo test`.
#[must_use]
pub fn run_mutation_control(
    workspace_root: &Path,
    control: &Control,
    gate_id: &str,
    tier_target: &str,
    target_dir: Option<&Path>,
    offline: bool,
    allow_m0_host_standin: bool,
) -> ControlEval {
    let route = match resolve_mutation_route(
        workspace_root,
        control,
        gate_id,
        tier_target,
        allow_m0_host_standin,
    ) {
        Ok(r) => r,
        Err(why) => return ControlEval::Stale { detail: why },
    };

    match &route {
        MutationRoute::Annex { workspace, .. } => {
            return ControlEval::Stale {
                detail: format!("annex workspace `{workspace}` must run via DRIVERS"),
            };
        }
        MutationRoute::WorkspaceTier {
            crate_name,
            feature,
            ..
        }
        | MutationRoute::LawSuite {
            crate_name,
            feature,
            ..
        }
        | MutationRoute::HostStandIn {
            crate_name,
            feature,
        } => {
            if let Err(why) = check_manifest_declares_feature(workspace_root, crate_name, feature) {
                return ControlEval::Stale { detail: why };
            }
        }
    }

    let mut cmd = build_cargo_command(workspace_root, &route, target_dir, offline);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            return ControlEval::Stale {
                detail: format!("failed to spawn cargo for `{}`: {e}", control.id),
            };
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");

    if output.status.success() {
        return ControlEval::DidNotFail {
            detail: format!(
                "mutant build `{}` passed all entries under route {route:?}",
                control.id
            ),
        };
    }

    // Distinguish missing/uncompilable feature (stale) from test failure (caught / caught-unexpected).
    if stderr.contains("does not have feature")
        || stderr.contains("does not have these features")
        || stderr.contains("none of the selected packages contains these features")
    {
        return ControlEval::Stale {
            detail: stderr.trim().to_owned(),
        };
    }

    ControlEval::Failed {
        observed: combined.trim().to_owned(),
    }
}

/// Locate `crates/<crate_name>/Cargo.toml` (or workspace member manifest with `[package].name`).
#[must_use]
pub fn find_crate_manifest(workspace_root: &Path, crate_name: &str) -> Option<PathBuf> {
    let direct = workspace_root
        .join("crates")
        .join(crate_name)
        .join("Cargo.toml");
    if direct.is_file() {
        return Some(direct);
    }
    let root_manifest = workspace_root.join("Cargo.toml");
    if root_manifest.is_file()
        && let Ok(txt) = fs::read_to_string(&root_manifest)
        && parse_manifest_package_name(&txt).as_deref() == Some(crate_name)
    {
        return Some(root_manifest);
    }
    let crates_dir = workspace_root.join("crates");
    let rd = fs::read_dir(crates_dir).ok()?;
    let mut dirs: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    dirs.sort();
    for dir in dirs {
        let m = dir.join("Cargo.toml");
        if !m.is_file() {
            continue;
        }
        if let Ok(txt) = fs::read_to_string(&m)
            && parse_manifest_package_name(&txt).as_deref() == Some(crate_name)
        {
            return Some(m);
        }
    }
    None
}

pub(crate) fn parse_manifest_package_name(content: &str) -> Option<String> {
    let mut in_pkg = false;
    for raw in content.lines() {
        let line = strip_toml_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_pkg = line == "[package]";
            continue;
        }
        if in_pkg
            && let Some((k, v)) = line.split_once('=')
            && k.trim() == "name"
        {
            return Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
        }
    }
    None
}

pub(crate) fn parse_manifest_feature_names(content: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut in_features = false;
    for raw in content.lines() {
        let line = strip_toml_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_features = line == "[features]";
            continue;
        }
        if in_features && let Some((k, _)) = line.split_once('=') {
            let key = k.trim().trim_matches('"').trim_matches('\'');
            if !key.is_empty() {
                names.insert(key.to_owned());
            }
        }
    }
    names
}

pub(crate) fn strip_toml_comment(line: &str) -> &str {
    let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
    for (i, b) in line.bytes().enumerate() {
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
        } else if b == b'#' {
            return &line[..i];
        }
    }
    line
}
