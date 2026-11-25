#![deny(unsafe_code)]

//! Printout regeneration tests for `docs/delivery/pilots.md` (`N38a-09`).
//!
//! Verifies that `docs/delivery/pilots.md` matches a fresh D0 generation from
//! `crates/frob-conformance/registry/pilots.toml` byte for byte, and that a
//! hand-edited printout is caught. Never deletes any file or directory.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::pilots::{
    PILOTS_PATH, PILOTS_PRINTOUT_PATH, check_printout_matches, load_register,
    render_pilots_markdown, workspace_root,
};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = base
        .join("frob-WindyDog")
        .join(format!("pilots_printout-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

#[test]
fn pilots_printout_matches_committed_file() {
    let root = workspace_root();
    check_printout_matches(&root).expect("docs/delivery/pilots.md must match fresh generation");
}

#[test]
fn pilots_printout_detects_hand_edit() {
    let root = workspace_root();
    let scratch = scratch_dir("hand_edit");
    fs::create_dir_all(scratch.join("crates/frob-conformance/registry"))
        .expect("create registry dir");
    fs::create_dir_all(scratch.join("docs/delivery")).expect("create delivery dir");
    fs::copy(root.join(PILOTS_PATH), scratch.join(PILOTS_PATH)).expect("copy pilots.toml");

    let reg = load_register(&scratch).expect("load copied register");
    let valid = render_pilots_markdown(&reg, Some(&scratch), None);
    let edited = format!("{valid}\n<!-- hand edit -->\n");
    fs::write(scratch.join(PILOTS_PRINTOUT_PATH), edited).expect("write edited printout");

    let err = check_printout_matches(&scratch).expect_err("hand-edited printout must fail");
    assert!(
        err.contains(PILOTS_PRINTOUT_PATH),
        "error must name printout path: {err}"
    );
}

#[test]
fn pilots_printout_d0_deterministic() {
    let root = workspace_root();
    let reg = load_register(&root).expect("load real pilots.toml");
    let first = render_pilots_markdown(&reg, Some(&root), None);
    let second = render_pilots_markdown(&reg, Some(&root), None);
    assert_eq!(first, second, "printout rendering must be D0 bit-identical");
}
