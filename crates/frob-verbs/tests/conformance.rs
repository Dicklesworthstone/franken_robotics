//! Crate-level conformance checks for `frob-verbs` (N32a-02, N34-08 rule 3, ADR-TETHER-02).

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::registry::refusals::RefusalRegistry;
use frob_core::registry::toml::{self, Fields};
use frob_verbs::registry::{REGISTRY_DIGEST, REGISTRY_DIGEST_HEX, TASK_CLUSTERS, VERBS};

const CARGO_TOML: &str = include_str!("../Cargo.toml");

const CONTRACT_MD: &str = include_str!("../CONTRACT.md");
const CONTRACT_TOML: &str = include_str!("../contract.toml");
const REQUIRED_SECTIONS: &[&str] = &[
    "Purpose",
    "Inputs",
    "Outputs",
    "Invariants",
    "Refusals",
    "Determinism class",
    "Evidence class of outputs",
    "Validity domain",
    "Oracles",
    "Known gaps",
];

#[test]
fn contract_md_has_all_ten_non_empty_sections_and_specimen_example() {
    let mut sections: Vec<(&str, String)> = Vec::new();
    let mut current: Option<(&str, String)> = None;
    for line in CONTRACT_MD.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(prev) = current.take() {
                sections.push(prev);
            }
            current = Some((heading.trim(), String::new()));
        } else if let Some((_, ref mut body)) = current {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(prev) = current {
        sections.push(prev);
    }

    for &req in REQUIRED_SECTIONS {
        let found = sections
            .iter()
            .find(|(h, _)| *h == req)
            .unwrap_or_else(|| panic!("CONTRACT.md missing required heading `## {req}`"));
        assert!(
            !found.1.trim().is_empty(),
            "CONTRACT.md section `## {req}` must be non-empty"
        );
    }
    let oracles = &sections.iter().find(|(h, _)| *h == "Oracles").unwrap().1;
    assert!(
        oracles.contains("docs/testing/CONVENTIONS.md"),
        "CONTRACT.md Oracles section must cite docs/testing/CONVENTIONS.md"
    );
    assert!(
        CONTRACT_MD.contains("schema = \"frob.verbs/1\"")
            && CONTRACT_MD.contains("name = \"design.check\""),
        "CONTRACT.md must document the registry format with one valid example"
    );
}

#[test]
fn contract_toml_validates_against_strict_toml_and_refusal_registry() {
    let reg = RefusalRegistry::load().expect("embedded refusals.toml must load");
    let root = toml::parse(CONTRACT_TOML.as_bytes()).expect("contract.toml must parse");
    let mut top = Fields::new(root, "contract.toml");

    let mut c = Fields::new(top.require_table("crate").unwrap(), "crate");
    assert_eq!(c.require_str("name").unwrap(), "frob-verbs");
    assert_eq!(c.require_int("layer").unwrap(), 6);
    assert_eq!(c.require_int("rank").unwrap(), 4);
    assert_eq!(c.require_str("plane").unwrap(), "decision");
    assert_eq!(c.require_str("determinism").unwrap(), "D0");
    assert_eq!(c.require_str("evidence").unwrap(), "Tested");
    let gates = c.require_array("gates").unwrap();
    assert_eq!(gates.len(), 1);
    assert_eq!(gates[0].value.as_str(), Some("GATE-TETHER-02"));
    for sp in c.require_array("refusals").unwrap() {
        let code = sp.value.as_str().expect("refusal code must be string");
        reg.lookup(code, "frob-verbs/contract.toml")
            .unwrap_or_else(|e| panic!("contract.toml refusal `{code}` not in registry: {e}"));
    }
    c.finish().unwrap();

    let mut budgets = Fields::new(top.require_table("budgets").unwrap(), "budgets");
    assert!(budgets.require_int("wall_ms").unwrap() > 0);
    assert!(budgets.require_int("memory_mib").unwrap() > 0);
    budgets.finish().unwrap();

    let mut safe = Fields::new(top.require_table("safe_mode").unwrap(), "safe_mode");
    assert_eq!(safe.require_str("on_budget").unwrap(), "reference-path");
    assert_eq!(
        safe.require_str("on_calibration").unwrap(),
        "reference-path"
    );
    safe.finish().unwrap();

    let mut feats = Fields::new(top.require_table("features").unwrap(), "features");
    for name in [
        "registry-only",
        "handlers",
        "conformance",
        "gauntlet-mutations",
        "nc-bare-unknown-verb",
        "nc-second-envelope",
        "nc-omit-805",
        "nc-key-in-required",
        "nc-append-990",
    ] {
        let mut f = Fields::new(feats.require_table(name).unwrap(), name);
        assert_eq!(f.require_str("tier").unwrap(), "S");
        f.finish().unwrap();
    }
    feats.finish().unwrap();

    // `[api.float_ok]`: every envelope item that carries a binary64 states why (N32a-03).
    let mut api = Fields::new(top.require_table("api").unwrap(), "api");
    let mut float_ok = Fields::new(api.require_table("float_ok").unwrap(), "api.float_ok");
    for sig in [
        "envelope::Cost",
        "envelope::Budget::with",
        "envelope::Eta",
        "envelope::Tree::as_f64",
    ] {
        let reason = float_ok.require_str(sig).unwrap();
        assert!(
            !reason.trim().is_empty(),
            "`[api.float_ok].\"{sig}\"` needs a reason"
        );
    }
    float_ok.finish().unwrap();
    api.finish().unwrap();
    top.finish().unwrap();

    // Verify generated static table symbols are live and consistent.
    assert_eq!(REGISTRY_DIGEST_HEX.len(), 64);
    assert_eq!(REGISTRY_DIGEST.len(), 32);
    let _ = (VERBS.len(), TASK_CLUSTERS.len());
}

#[test]
fn registry_only_has_no_physics() {
    // ADR-TETHER-02: `--no-default-features --features registry-only` must carry no
    // `frob-fsim`, no FrankenSim physics crate, and no subsystem crate.
    // Note: `frob-core` unconditionally depends on layer-0 non-physics utilities
    // `fs-blake3` and `fs-qty`; those two are explicitly distinguished from physics crates.
    let output = Command::new("cargo")
        .args([
            "tree",
            "-p",
            "frob-verbs",
            "--no-default-features",
            "--features",
            "registry-only",
            "-e",
            "normal",
            "--locked",
            "--offline",
        ])
        .output()
        .expect("cargo tree must execute via PATH");
    assert!(
        output.status.success(),
        "cargo tree failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages: Vec<String> = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim_start_matches(|c: char| !c.is_ascii_alphanumeric());
        if let Some(pkg) = trimmed.split_whitespace().next() {
            packages.push(pkg.to_owned());
        }
    }
    assert!(
        packages.iter().any(|p| p == "frob-verbs"),
        "expected frob-verbs in cargo tree output:\n{stdout}"
    );
    for pkg in &packages {
        assert_ne!(
            pkg, "frob-fsim",
            "registry-only must never link frob-fsim:\n{stdout}"
        );
        if pkg.starts_with("frob-") {
            assert!(
                matches!(pkg.as_str(), "frob-verbs" | "frob-core"),
                "registry-only linked unexpected subsystem crate `{pkg}`:\n{stdout}"
            );
        }
        if pkg.starts_with("fs-") {
            assert!(
                matches!(pkg.as_str(), "fs-blake3" | "fs-qty"),
                "registry-only linked unexpected FrankenSim physics crate `{pkg}`:\n{stdout}"
            );
        }
    }
}

const RELEASE_FORBIDDEN_FEATURES: &[&str] = &[
    "conformance",
    "gauntlet-mutations",
    "nc-bare-unknown-verb",
    "nc-second-envelope",
    "nc-omit-805",
    "nc-key-in-required",
    "nc-append-990",
];

const NC_FEATURES: &[&str] = &[
    "nc-bare-unknown-verb",
    "nc-second-envelope",
    "nc-omit-805",
    "nc-key-in-required",
    "nc-append-990",
];

fn parse_manifest_features(raw: &str) -> Vec<(String, Vec<String>)> {
    let mut in_features = false;
    let mut pending: Option<(String, String)> = None;
    let mut out = Vec::new();
    for raw_line in raw.lines() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if let Some((ref key, ref mut acc)) = pending {
            acc.push(' ');
            acc.push_str(line);
            if line.contains(']') {
                out.push((key.clone(), parse_bracket_list(acc)));
                pending = None;
            }
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_features = line == "[features]";
            continue;
        }
        if !in_features {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let key = k.trim().trim_matches('"').trim_matches('\'').to_owned();
        let val = v.trim();
        if val.contains('[') && !val.contains(']') {
            pending = Some((key, val.to_owned()));
        } else {
            out.push((key, parse_bracket_list(val)));
        }
    }
    out
}

fn parse_bracket_list(val: &str) -> Vec<String> {
    let Some(start) = val.find('[') else {
        return Vec::new();
    };
    let rest = &val[start + 1..];
    let end = rest.find(']').unwrap_or(rest.len());
    rest[..end]
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim_matches('\'').trim())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}

#[test]
fn nc_features_imply_gauntlet_mutations() {
    let features = parse_manifest_features(CARGO_TOML);
    for &nc in NC_FEATURES {
        let deps = features
            .iter()
            .find(|(name, _)| name == nc)
            .map(|(_, deps)| deps)
            .unwrap_or_else(|| panic!("frob-verbs/Cargo.toml missing `[features].{nc}`"));
        assert!(
            deps.iter().any(|d| d == "gauntlet-mutations"),
            "frob-verbs planted-defect feature `{nc}` must imply `gauntlet-mutations`, got {deps:?}"
        );
    }
    for (name, deps) in &features {
        if name.starts_with("nc-") {
            assert!(
                deps.iter().any(|d| d == "gauntlet-mutations"),
                "every `nc-*` feature in frob-verbs/Cargo.toml must imply `gauntlet-mutations`; `{name}` has {deps:?}"
            );
        }
        if matches!(name.as_str(), "default" | "registry-only") {
            for &forbidden in RELEASE_FORBIDDEN_FEATURES {
                assert!(
                    !deps.iter().any(|d| d == forbidden),
                    "`{name}` must not enable release-forbidden feature `{forbidden}`"
                );
            }
        }
    }
}

#[test]
fn release_profile_refuses_conformance_and_mutation_features() {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("crates/frob-verbs sits two levels below workspace root");
    let workspace_manifest = repo_root.join("Cargo.toml");

    let base = env::var_os("CARGO_TARGET_TMPDIR")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let target_dir = base.join(format!(
        "frob-verbs-release-guard-{}-{nanos}-{}",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&target_dir).expect("create isolated target dir for release guard test");

    for &feature in RELEASE_FORBIDDEN_FEATURES {
        let output = Command::new("cargo")
            .args([
                "check",
                "--manifest-path",
                workspace_manifest.to_str().expect("utf8 manifest path"),
                "--offline",
                "--locked",
                "-j",
                "1",
                "--release",
                "--config",
                "profile.release.debug-assertions=true",
                "--no-default-features",
                "--features",
                &format!("registry-only,{feature}"),
                "-p",
                "frob-verbs",
            ])
            .current_dir(repo_root)
            .env("CARGO_TARGET_DIR", &target_dir)
            .env_remove("CARGO_BUILD_BUILD_DIR")
            .output()
            .expect("invoke cargo check for frob-verbs release-profile guard");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !output.status.success(),
            "release profile with debug-assertions=true and `--features {feature}` MUST fail in build.rs; stderr:\n{stderr}"
        );
        assert!(
            stderr.contains("frob-verbs build failed:")
                && stderr.contains(feature)
                && stderr.contains("release"),
            "release profile refusal for `{feature}` must come from build.rs and name the feature; stderr:\n{stderr}"
        );
    }
}
