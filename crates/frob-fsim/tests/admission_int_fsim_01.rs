#![forbid(unsafe_code)]

//! Admission integration tests for `INT-FSIM-01` (`BD-006`, ADR-SUITE-01).
//!
//! Covers all four named tests from `BD-006`'s Test Plan:
//! - `links_five_crates`
//! - `build_identity_is_set`
//! - `one_asupersync_in_metadata`
//! - `closure_counts_recorded`

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use frob_core::json::{self, Json, Limits};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/frob-fsim must sit two levels below workspace root")
        .to_path_buf()
}

fn run_cargo_metadata(root: &Path) -> Vec<u8> {
    // Always invoke `cargo` via PATH (never by absolute path).
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--locked", "--offline"])
        .current_dir(root)
        .output()
        .expect("failed to execute `cargo metadata`");
    assert!(
        output.status.success(),
        "`cargo metadata` exited with {}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn is_frankensim_manifest(manifest_path: &str) -> bool {
    let normalized = manifest_path.replace('\\', "/");
    normalized.contains("/frankensim/crates/")
}

fn count_fsim_crates_in_closure(meta: &Json<'_>) -> (usize, BTreeSet<String>) {
    let packages = meta
        .get("packages")
        .and_then(|v| v.as_array())
        .expect("cargo metadata must include `packages` array");

    let mut id_to_pkg: BTreeMap<&str, (&str, &str)> = BTreeMap::new();
    let mut frob_fsim_id: Option<&str> = None;
    for pkg in packages {
        let id = pkg
            .get("id")
            .and_then(|v| v.as_str())
            .expect("package missing string `id`");
        let name = pkg
            .get("name")
            .and_then(|v| v.as_str())
            .expect("package missing string `name`");
        let manifest_path = pkg
            .get("manifest_path")
            .and_then(|v| v.as_str())
            .expect("package missing string `manifest_path`");
        id_to_pkg.insert(id, (name, manifest_path));
        if name == "frob-fsim" {
            frob_fsim_id = Some(id);
        }
    }
    let root_id = frob_fsim_id.expect("workspace metadata must contain `frob-fsim`");

    let nodes = meta
        .get("resolve")
        .and_then(|v| v.get("nodes"))
        .and_then(|v| v.as_array())
        .expect("cargo metadata must include `resolve.nodes`");
    let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for node in nodes {
        let id = node
            .get("id")
            .and_then(|v| v.as_str())
            .expect("resolve node missing `id`");
        let deps = node
            .get("dependencies")
            .and_then(|v| v.as_array())
            .expect("resolve node missing `dependencies`");
        let mut children = Vec::with_capacity(deps.len());
        for dep in deps {
            if let Some(dep_id) = dep.as_str() {
                children.push(dep_id);
            }
        }
        adj.insert(id, children);
    }

    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    seen.insert(root_id);
    queue.push_back(root_id);

    while let Some(cur) = queue.pop_front() {
        if let Some(children) = adj.get(cur) {
            for &child in children {
                if seen.insert(child) {
                    queue.push_back(child);
                }
            }
        }
    }

    let mut fsim_names = BTreeSet::new();
    for id in seen {
        if let Some(&(name, manifest_path)) = id_to_pkg.get(id)
            && is_frankensim_manifest(manifest_path)
        {
            fsim_names.insert(name.to_owned());
        }
    }
    (fsim_names.len(), fsim_names)
}

fn extract_fsim_crate_count(doc: &Json<'_>) -> Option<usize> {
    let measured = doc.get("measured")?.as_array()?;
    for entry in measured {
        let name = entry.get("name").and_then(|v| v.as_str())?;
        if name == "fsim_crates_compiled" {
            let val = entry.get("value")?.as_int53().ok()?;
            return usize::try_from(val).ok();
        }
    }
    None
}

fn find_recorded_fsim_crate_count(root: &Path) -> (PathBuf, usize) {
    // Prefer an official `frob.admission/1` record under `receipts/admissions/INT-FSIM-01/`
    // when present (taking the newest by `recorded.date` then filename); otherwise fall back
    // to the checked-in `INT-FSIM-01` measurement input file.
    let admissions_dir = root.join("receipts/admissions/INT-FSIM-01");
    if admissions_dir.is_dir() {
        let mut candidates: Vec<(String, PathBuf, usize)> = Vec::new();
        if let Ok(entries) = fs::read_dir(&admissions_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                let Ok(bytes) = fs::read(&path) else {
                    continue;
                };
                let Ok(admitted) = json::admit(&bytes, &Limits::default()) else {
                    continue;
                };
                let doc = admitted.value();
                if let Some(count) = extract_fsim_crate_count(doc) {
                    let date = doc
                        .get("recorded")
                        .and_then(|r| r.get("date"))
                        .and_then(|d| d.as_str())
                        .unwrap_or("")
                        .to_owned();
                    candidates.push((date, path, count));
                }
            }
        }
        candidates.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        if let Some((_, path, count)) = candidates.pop() {
            return (path, count);
        }
    }

    let input_path = root.join("tests/admission/int_fsim_01_input.json");
    let bytes = fs::read(&input_path).unwrap_or_else(|e| {
        panic!(
            "missing recorded INT-FSIM-01 measurement file `{}`: {e}",
            input_path.display()
        )
    });
    let admitted = json::admit(&bytes, &Limits::default()).unwrap_or_else(|e| {
        panic!("invalid JSON in `{}`: {e}", input_path.display())
    });
    let count = extract_fsim_crate_count(admitted.value()).unwrap_or_else(|| {
        panic!(
            "`{}` must contain a `measured` entry named `fsim_crates_compiled`",
            input_path.display()
        )
    });
    (input_path, count)
}

#[test]
fn links_five_crates() {
    let results = frob_fsim::smoke_all_admitted();
    assert_eq!(results.len(), 5, "expected five admitted FrankenSim crates");

    let names: Vec<&str> = results.iter().map(|r| r.crate_name).collect();
    assert_eq!(
        names,
        frob_fsim::ADMITTED_FRANKENSIM_CRATES,
        "admitted crate list must match the five INT-FSIM-01 crates"
    );
    for res in results {
        assert!(
            res.passed,
            "smoke check failed for `{}` ({})",
            res.crate_name, res.item_path
        );
    }

    // Exercise each crate wrapper individually as well.
    assert!(frob_fsim::smoke_fs_contact().passed);
    assert!(frob_fsim::smoke_fs_ga().passed);
    assert!(frob_fsim::smoke_fs_mbd().passed);
    assert!(frob_fsim::smoke_fs_query().passed);
    assert!(frob_fsim::smoke_fs_time().passed);
}

#[test]
fn build_identity_is_set() {
    let identity = frob_fsim::build_identity().expect(
        "either FRANKENSIM_DEPGRAPH_RECEIPT or FRANKENSIM_DEPGRAPH_SALT must be set at compile time",
    );
    assert!(
        !identity.value.is_empty(),
        "build identity variable `{}` must not be empty",
        identity.env_var
    );
    println!(
        "INT-FSIM-01 build identity: {}={}",
        identity.env_var, identity.value
    );
}

#[test]
fn one_asupersync_in_metadata() {
    let root = workspace_root();
    let stdout = run_cargo_metadata(&root);
    let admitted = json::admit(&stdout, &Limits::default())
        .expect("cargo metadata output must be valid JSON");
    let packages = admitted
        .value()
        .get("packages")
        .and_then(|v| v.as_array())
        .expect("cargo metadata must contain `packages` array");

    let mut asupersync_pkgs: Vec<(&str, &str)> = Vec::new();
    for pkg in packages {
        if pkg.get("name").and_then(|v| v.as_str()) == Some("asupersync") {
            let ver = pkg.get("version").and_then(|v| v.as_str()).unwrap_or("?");
            let mp = pkg
                .get("manifest_path")
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            asupersync_pkgs.push((ver, mp));
        }
    }
    assert_eq!(
        asupersync_pkgs.len(),
        1,
        "expected exactly one `asupersync` package in workspace metadata, found: {asupersync_pkgs:?}"
    );
}

#[test]
fn closure_counts_recorded() {
    let root = workspace_root();
    let stdout = run_cargo_metadata(&root);
    let admitted = json::admit(&stdout, &Limits::default())
        .expect("cargo metadata output must be valid JSON");
    let (live_count, fsim_names) = count_fsim_crates_in_closure(admitted.value());

    for required in frob_fsim::ADMITTED_FRANKENSIM_CRATES {
        assert!(
            fsim_names.contains(required),
            "live FrankenSim closure missing admitted crate `{required}`"
        );
    }

    let (source_path, recorded_count) = find_recorded_fsim_crate_count(&root);
    assert_eq!(
        live_count,
        recorded_count,
        "live FrankenSim crate count in `frob-fsim` closure ({live_count}) disagrees with recorded `fsim_crates_compiled` ({recorded_count}) in `{}`",
        source_path.display()
    );
}
