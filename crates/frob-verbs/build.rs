//! Build script for `frob-verbs` (bead N32a-02, ADR-TETHER-01).
//!
//! Reads `registries/verbs.toml` (or `FROB_VERBS_REGISTRY` in negative-control lanes)
//! together with referenced schema files under `schemas/` (or `FROB_VERBS_SCHEMAS_DIR`),
//! validates all build-time lints (`RL-01`..`RL-24`), and emits deterministic
//! `$OUT_DIR/verbs_table.rs` (`VERBS`, `TASK_CLUSTERS`, `SCHEMAS`, `REGISTRY_DIGEST`).
//!
//! Checked for byte-identity and `FROB_VERBS_REGISTRY` rejection by `xtask check-generated`
//! (N34p4-03 / BD-022).

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[path = "src/registry/engine.rs"]
mod engine;
#[path = "src/envelope/regex.rs"]
mod regex;
#[path = "src/envelope/tree.rs"]
mod tree;
#[path = "src/envelope/validate.rs"]
mod validate;

mod envelope {
    pub(crate) use super::{tree::Tree, validate::Schema};
}

use engine::codegen::compile_registry;
use engine::entry::ParsedRegistry;
use engine::lint::{LintContext, LintError};
use engine::schema_id::schema_file_name;

const RELEASE_FORBIDDEN_FEATURES: &[(&str, &str)] = &[
    ("CARGO_FEATURE_CONFORMANCE", "conformance"),
    ("CARGO_FEATURE_GAUNTLET_MUTATIONS", "gauntlet-mutations"),
    ("CARGO_FEATURE_NC_BARE_UNKNOWN_VERB", "nc-bare-unknown-verb"),
    ("CARGO_FEATURE_NC_SECOND_ENVELOPE", "nc-second-envelope"),
    ("CARGO_FEATURE_NC_OMIT_805", "nc-omit-805"),
    ("CARGO_FEATURE_NC_KEY_IN_REQUIRED", "nc-key-in-required"),
    ("CARGO_FEATURE_NC_APPEND_990", "nc-append-990"),
];

fn main() {
    println!("cargo:rerun-if-env-changed=FROB_VERBS_REGISTRY");
    println!("cargo:rerun-if-env-changed=FROB_VERBS_SCHEMAS_DIR");
    println!("cargo:rerun-if-env-changed=FROB_VERBS_SCOPES");

    let conformance = env::var_os("CARGO_FEATURE_CONFORMANCE").is_some();
    // Cargo classifies PROFILE by inheritance, independent of debug assertions.
    if env::var("PROFILE").as_deref() == Ok("release") {
        let mut forbidden: BTreeSet<String> = RELEASE_FORBIDDEN_FEATURES
            .iter()
            .filter_map(|&(env_var, feature)| env::var_os(env_var).map(|_| feature.to_owned()))
            .collect();
        for (key, _) in env::vars() {
            if let Some(rest) = key.strip_prefix("CARGO_FEATURE_")
                && (rest.starts_with("NC_")
                    || rest.starts_with("CX_")
                    || rest.starts_with("MUTANT_"))
            {
                forbidden.insert(rest.to_ascii_lowercase().replace('_', "-"));
            }
        }
        if !forbidden.is_empty() {
            let list: Vec<&str> = forbidden.iter().map(String::as_str).collect();
            eprintln!(
                "frob-verbs build failed: feature(s) `{}` forbidden in release profiles",
                list.join("`, `")
            );
            process::exit(1);
        }
    }

    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set by Cargo"),
    );
    let repo_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("crates/frob-verbs must live two levels below repository root")
        .to_path_buf();

    let override_env = env::var_os("FROB_VERBS_REGISTRY");
    let built_with_override = override_env.is_some();
    println!(
        "cargo:rustc-env=FROB_VERBS_BUILT_WITH_OVERRIDE={}",
        if built_with_override { "1" } else { "0" }
    );

    let registry_path = override_env
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root.join("registries/verbs.toml"));
    let schemas_dir = env::var_os("FROB_VERBS_SCHEMAS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root.join("schemas"));
    let scopes_path = env::var_os("FROB_VERBS_SCOPES")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root.join("registries/scopes.toml"));
    let refusals_path = repo_root.join("registries/refusals.toml");

    println!("cargo:rerun-if-changed={}", registry_path.display());
    println!("cargo:rerun-if-changed={}", refusals_path.display());

    let registry_bytes = match fs::read(&registry_path) {
        Ok(b) => b,
        Err(err) => {
            eprintln!(
                "frob-verbs build failed: cannot read registry `{}`: {err}",
                registry_path.display()
            );
            process::exit(1);
        }
    };

    let parsed = match ParsedRegistry::parse_toml(&registry_bytes) {
        Ok(reg) => reg,
        Err(err) => {
            let lint_err = LintError::from(err);
            eprintln!("frob-verbs build failed: {lint_err}");
            process::exit(1);
        }
    };

    let mut schemas = BTreeMap::new();
    for entry in &parsed.verbs {
        for schema_id in [&entry.input_schema, &entry.output_schema] {
            let fname = schema_file_name(schema_id);
            if schemas.contains_key(&fname) {
                continue;
            }
            let source = schemas_dir.join("verbs").join(&fname);
            let full = if source.is_file() {
                source
            } else {
                schemas_dir.join(&fname)
            };
            println!("cargo:rerun-if-changed={}", full.display());
            if let Ok(bytes) = fs::read(&full) {
                schemas.insert(fname, bytes);
            }
        }
    }

    let scopes = load_optional_scopes(&scopes_path);
    let ctx = match LintContext::with_embedded_refusals(schemas, scopes, conformance) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("frob-verbs build failed: {err}");
            process::exit(1);
        }
    };

    let generated = match compile_registry(&parsed, &ctx) {
        Ok(g) => g,
        Err(err) => {
            eprintln!("frob-verbs build failed: {err}");
            process::exit(1);
        }
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo"));
    let out_file = out_dir.join("verbs_table.rs");
    if let Err(err) = fs::write(&out_file, &generated.source) {
        eprintln!(
            "frob-verbs build failed: cannot write `{}`: {err}",
            out_file.display()
        );
        process::exit(1);
    }
}

fn load_optional_scopes(path: &Path) -> Option<BTreeSet<String>> {
    if !path.is_file() {
        return None;
    }
    println!("cargo:rerun-if-changed={}", path.display());
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(err) => {
            eprintln!(
                "frob-verbs build failed: cannot read scope catalogue `{}`: {err}",
                path.display()
            );
            process::exit(1);
        }
    };
    let root = match frob_core::registry::toml::parse(&bytes) {
        Ok(t) => t,
        Err(err) => {
            eprintln!(
                "frob-verbs build failed: invalid scope catalogue `{}`: {err}",
                path.display()
            );
            process::exit(1);
        }
    };
    let mut scopes = BTreeSet::new();
    if let Some(arr) = root.get("scope").and_then(|v| v.as_array()) {
        for item in arr {
            if let Some(name) = item
                .value
                .as_table()
                .and_then(|t| t.get("name"))
                .and_then(|v| v.as_str())
            {
                scopes.insert(name.to_owned());
            }
        }
    }
    Some(scopes)
}
