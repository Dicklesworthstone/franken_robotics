#![deny(unsafe_code)]

//! Conformance unit tests for the independent `frob.design/1` reference
//! canonicalizer and golden corpus (`N14a-26`).
//!
//! Run as `cargo test -p frob-conformance --test design_golden`.
//! Never deletes files or directories (COMMON.md 9b).

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_conformance::design_golden::{
    CORPUS_REL_DIR, CaseExpect, CanonOutcome, ORACLE_REL_DIR, check, check_manifest,
    compute_oracle_blake3, invoke_reference, load_manifest, verify_manifest_integrity,
};
use frob_core::testkit::registry;

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn workspace_root() -> PathBuf {
    if let Some(ov) = env::var_os("FROB_DESIGN_GOLDEN_ROOT") {
        return PathBuf::from(ov);
    }
    registry::root().expect("locate workspace root via franken-stack.lock")
}

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
        .join("frob-BronzePelican")
        .join(format!("design_golden-{label}-{nanos}-{seq}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn copy_corpus_to_scratch(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create scratch corpus dir");
    for entry in fs::read_dir(src).expect("read corpus dir").flatten() {
        let p = entry.path();
        if p.is_file() {
            let fname = entry.file_name();
            fs::copy(&p, dst.join(fname)).expect("copy corpus file to scratch");
        }
    }
}

#[test]
fn harness_accepts_reference() {
    let root = workspace_root();
    let summary = check(&root, |bytes| invoke_reference(&root, "ref", bytes)).unwrap_or_else(|ms| {
        let lines: Vec<String> = ms.iter().map(ToString::to_string).collect();
        panic!(
            "reference canonicalizer disagreed with golden corpus:\n{}",
            lines.join("\n")
        );
    });
    assert!(summary.total >= 40, "expected >= 40 corpus cases, got {}", summary.total);
    assert_eq!(summary.accepted, 41);
    assert_eq!(summary.refused, 6);
    println!(
        "design_golden: total={} accepted={} refused={} oracle_blake3={} manifest_blake3={}",
        summary.total,
        summary.accepted,
        summary.refused,
        summary.oracle_blake3,
        summary.manifest_blake3
    );
}

#[test]
fn harness_rejects_planted() {
    let root = workspace_root();

    // 1. UTF-16 code-unit key order: fails ONLY on the two distinguishing key_order files.
    let u16_errs = check(&root, |bytes| invoke_reference(&root, "utf16-keys", bytes))
        .expect_err("utf16-keys mutant must fail");
    let u16_ids: BTreeSet<&str> = u16_errs.iter().map(|m| m.case_id.as_str()).collect();
    let u16_groups: BTreeSet<&str> = u16_errs.iter().map(|m| m.group.as_str()).collect();
    assert_eq!(
        u16_ids,
        BTreeSet::from(["07_keys_codepoint_vs_utf16_a", "08_keys_codepoint_vs_utf16_b"]),
        "utf16-keys must fail only on the two distinguishing key_order files: {u16_errs:?}"
    );
    assert_eq!(u16_groups, BTreeSet::from(["key_order"]));
    for m in &u16_errs {
        assert!(m.first_diff_offset.is_some(), "mismatch must report byte offset: {m}");
    }

    // 2. binary64 with shortest round-trip printing: fails on 1/3 m and 0.30000000000000001 m
    //    (plus worked design quad-tray which has 1/7 m), while 29.97 Hz survives and no other
    //    focused group fails.
    let rt_errs = check(&root, |bytes| invoke_reference(&root, "bin64-roundtrip", bytes))
        .expect_err("bin64-roundtrip mutant must fail");
    let rt_ids: BTreeSet<&str> = rt_errs.iter().map(|m| m.case_id.as_str()).collect();
    let rt_groups: BTreeSet<&str> = rt_errs.iter().map(|m| m.group.as_str()).collect();
    assert!(rt_ids.contains("12_qty_one_third_m"), "must catch 1/3 m: {rt_ids:?}");
    assert!(
        rt_ids.contains("19_qty_0_30000000000000001m"),
        "must catch 0.30000000000000001 m: {rt_ids:?}"
    );
    assert!(
        !rt_ids.contains("18_qty_4000usd_29_97hz"),
        "29.97 Hz survives shortest round-trip printing (caught by bin64-exact): {rt_ids:?}"
    );
    assert!(
        rt_groups.is_subset(&BTreeSet::from(["quantities", "worked_designs"])),
        "bin64-roundtrip must fail on no unrelated group: {rt_groups:?}"
    );
    for m in &rt_errs {
        assert!(m.first_diff_offset.is_some(), "mismatch must report byte offset: {m}");
    }

    // 3. binary64 with exact double printing: catches 29.97 Hz and non-dyadic decimals in quantities
    //    (and worked_designs), and fails on no unrelated group.
    let ex_errs = check(&root, |bytes| invoke_reference(&root, "bin64-exact", bytes))
        .expect_err("bin64-exact mutant must fail");
    let ex_ids: BTreeSet<&str> = ex_errs.iter().map(|m| m.case_id.as_str()).collect();
    let ex_groups: BTreeSet<&str> = ex_errs.iter().map(|m| m.group.as_str()).collect();
    assert!(
        ex_ids.contains("18_qty_4000usd_29_97hz"),
        "bin64-exact must catch 29.97 Hz: {ex_ids:?}"
    );
    assert!(
        ex_groups.is_subset(&BTreeSet::from(["quantities", "worked_designs"])),
        "bin64-exact must fail on no unrelated group: {ex_groups:?}"
    );
    for m in &ex_errs {
        assert!(m.first_diff_offset.is_some(), "mismatch must report byte offset: {m}");
    }

    // 4. defaults not written out: fails on the 5 omitted-default files in defaults (and authored
    //    worked_designs), and on no unrelated group.
    let nd_errs = check(&root, |bytes| invoke_reference(&root, "no-defaults", bytes))
        .expect_err("no-defaults mutant must fail");
    let nd_ids: BTreeSet<&str> = nd_errs.iter().map(|m| m.case_id.as_str()).collect();
    let nd_groups: BTreeSet<&str> = nd_errs.iter().map(|m| m.group.as_str()).collect();
    for expected in [
        "28_def_origin_omitted",
        "30_def_collision_omitted",
        "32_def_members_omitted",
        "34_def_removable_omitted",
        "36_def_locked_omitted",
    ] {
        assert!(
            nd_ids.contains(expected),
            "no-defaults must catch {expected}: {nd_ids:?}"
        );
    }
    assert!(
        nd_groups.is_subset(&BTreeSet::from(["defaults", "worked_designs"])),
        "no-defaults must fail on no unrelated group: {nd_groups:?}"
    );
    for m in &nd_errs {
        assert!(m.first_diff_offset.is_some(), "mismatch must report byte offset: {m}");
    }
}

#[test]
fn utf16_vs_codepoint_files_differ_and_stored_is_codepoint() {
    let root = workspace_root();
    let corpus = root.join(CORPUS_REL_DIR);
    for (fbot, canon, cp_first, cp_second) in [
        (
            "07_keys_codepoint_vs_utf16_a.fbot",
            "07_keys_codepoint_vs_utf16_a.canon",
            "\u{E000}",
            "\u{10000}",
        ),
        (
            "08_keys_codepoint_vs_utf16_b.fbot",
            "08_keys_codepoint_vs_utf16_b.canon",
            "\u{FF01}",
            "\u{1F916}",
        ),
    ] {
        let raw = fs::read(corpus.join(fbot)).expect("read distinguishing input");
        let stored = fs::read(corpus.join(canon)).expect("read stored expectation");
        let CanonOutcome::Accepted {
            canonical_bytes: cp_bytes,
            ..
        } = invoke_reference(&root, "ref", &raw)
        else {
            panic!("ref must accept {fbot}");
        };
        let CanonOutcome::Accepted {
            canonical_bytes: u16_bytes,
            ..
        } = invoke_reference(&root, "utf16-keys", &raw)
        else {
            panic!("utf16-keys must emit bytes for {fbot}");
        };
        assert_ne!(cp_bytes, u16_bytes, "{fbot} must differ between code-point and UTF-16 order");
        assert_eq!(stored, cp_bytes, "{canon} must store the code-point ordered bytes");

        let stored_str = String::from_utf8(stored).expect("valid UTF-8");
        let i1 = stored_str.find(cp_first).expect("first code point present");
        let i2 = stored_str.find(cp_second).expect("second code point present");
        assert!(i1 < i2, "{canon} must sort {cp_first:?} before {cp_second:?}");
    }
}

#[test]
fn defaults_pairs_share_one_identity() {
    let root = workspace_root();
    let manifest = load_manifest(&root.join(CORPUS_REL_DIR).join("manifest.toml"))
        .expect("load manifest.toml");
    let by_id: BTreeMap<&str, _> = manifest.cases.iter().map(|c| (c.id.as_str(), c)).collect();
    let pairs = [
        ("28_def_origin_omitted", "29_def_origin_explicit"),
        ("30_def_collision_omitted", "31_def_collision_explicit"),
        ("32_def_members_omitted", "33_def_members_explicit"),
        ("34_def_removable_omitted", "35_def_removable_explicit"),
        ("36_def_locked_omitted", "37_def_locked_explicit"),
    ];
    for (omitted, explicit) in pairs {
        let a = by_id[omitted];
        let b = by_id[explicit];
        assert_eq!(a.group, "defaults");
        assert_eq!(b.group, "defaults");
        assert_eq!(
            a.design_id, b.design_id,
            "defaults pair {omitted} vs {explicit} must share one DesignId"
        );
        assert_eq!(
            a.canon_blake3, b.canon_blake3,
            "defaults pair {omitted} vs {explicit} must share identical canonical bytes"
        );
    }
}

#[test]
fn refusals_have_no_canonical_form() {
    let root = workspace_root();
    let corpus = root.join(CORPUS_REL_DIR);
    let manifest = load_manifest(&corpus.join("manifest.toml")).expect("load manifest.toml");
    let refusals: Vec<_> = manifest
        .cases
        .iter()
        .filter(|c| c.expect == CaseExpect::Refuse)
        .collect();
    assert_eq!(refusals.len(), 6, "expected 6 refusal cases");
    for c in refusals {
        assert!(c.canon.is_empty());
        assert!(c.design_id.is_empty());
        let raw = fs::read(corpus.join(&c.input)).expect("read refusal input");
        match invoke_reference(&root, "ref", &raw) {
            CanonOutcome::Accepted { design_id, .. } => {
                panic!("refusal case `{}` was accepted as `{design_id}`", c.id);
            }
            CanonOutcome::Refused { reason_class, .. } => {
                assert_eq!(
                    reason_class, c.reason,
                    "refusal case `{}` reason mismatch",
                    c.id
                );
            }
        }
    }
}

#[test]
fn manifest_is_complete() {
    let root = workspace_root();
    let manifest = verify_manifest_integrity(&root).unwrap_or_else(|errs| {
        panic!("manifest integrity failed:\n{}", errs.join("\n"));
    });
    assert_eq!(manifest.cases.len(), 47);
    let mut by_group: BTreeMap<&str, usize> = BTreeMap::new();
    for c in &manifest.cases {
        *by_group.entry(c.group.as_str()).or_insert(0) += 1;
    }
    assert_eq!(by_group["worked_designs"], 2);
    assert_eq!(by_group["key_order"], 6);
    assert_eq!(by_group["quantities"], 11);
    assert_eq!(by_group["annotated"], 4);
    assert_eq!(by_group["wide_values"], 4);
    assert_eq!(by_group["defaults"], 10);
    assert_eq!(by_group["constructors"], 4);
    assert_eq!(by_group["refusals"], 6);
}

#[test]
fn expectations_from_reference_only() {
    let root = workspace_root();
    let manifest = load_manifest(&root.join(CORPUS_REL_DIR).join("manifest.toml"))
        .expect("load manifest.toml");
    let live_oracle_hash = compute_oracle_blake3(&root).expect("hash reference scripts");
    assert_eq!(
        manifest.oracle_blake3, live_oracle_hash,
        "manifest oracle_blake3 must equal hash of reference scripts in tree"
    );
}

#[test]
fn planted_expected_byte_flip_caught() {
    let root = workspace_root();
    let src_corpus = root.join(CORPUS_REL_DIR);
    let dst_corpus = scratch_dir("byte_flip");
    copy_corpus_to_scratch(&src_corpus, &dst_corpus);

    // Flip one byte at a known offset in 10_qty_0_42m.canon.
    let target_name = "10_qty_0_42m.canon";
    let target_path = dst_corpus.join(target_name);
    let mut bytes = fs::read(&target_path).expect("read copied canon");
    let flip_offset = 10usize;
    assert!(flip_offset < bytes.len());
    bytes[flip_offset] = if bytes[flip_offset] == b'X' { b'Y' } else { b'X' };
    fs::write(&target_path, &bytes).expect("write flipped canon");

    let manifest = load_manifest(&dst_corpus.join("manifest.toml")).expect("load manifest");
    let errs = check_manifest(&manifest, &dst_corpus, |inp| {
        invoke_reference(&root, "ref", inp)
    })
    .expect_err("harness must catch flipped expected byte");
    let hit = errs
        .iter()
        .find(|m| m.file == target_name)
        .unwrap_or_else(|| panic!("expected mismatch naming `{target_name}`, got {errs:?}"));
    assert_eq!(
        hit.first_diff_offset,
        Some(flip_offset),
        "harness must report exact byte offset {flip_offset}: {hit}"
    );
}

#[test]
fn reference_selftest_passes() {
    let root = workspace_root();
    let script = root.join(ORACLE_REL_DIR).join("selftest.sh");
    let out = Command::new("bash")
        .arg(&script)
        .output()
        .expect("run tools/oracle/design_canon/selftest.sh");
    assert!(
        out.status.success(),
        "selftest.sh failed (exit {:?}):\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
