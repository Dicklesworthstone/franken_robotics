//! Integration and unit tests for `xtask::crate_map` (bead N34-02).
//!
//! Synthetic workspace fixtures are written under
//! `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-CopperDove/` and never deleted.

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use xtask::crate_map::{
    CRATE_MAP, CrateRow, Layer, NON_RUST_CAPS, RULE_METADATA_MISMATCH, RULE_UNLISTED_CRATE,
    Reason, TOMBSTONES, TableError, Tier, Tombstone, WorkspaceMember, counted_rows,
    discover_workspace_members, frontier_or_moonshot_only_core_cap_kloc, layer_cap_kloc, lookup,
    total_cap_kloc, trigger_cap_kloc, validate, validate_table, validate_with_table,
};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn make_scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let actor = env::var("FROB_SCRATCH_ACTOR").unwrap_or_else(|_| "CopperDove".to_owned());
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let dir = base
        .join(format!("frob-{actor}"))
        .join(format!("xtask-crate-map-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn write_member(root: &Path, rel_dir: &str, name: &str, metadata_section: &str) -> WorkspaceMember {
    let dir = root.join(rel_dir);
    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir).expect("create synthetic member dir");
    let manifest_path = dir.join("Cargo.toml");
    let content = format!(
        "[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n{metadata_section}\n"
    );
    fs::write(&manifest_path, content).expect("write synthetic Cargo.toml");
    fs::write(src_dir.join("lib.rs"), "").expect("write synthetic lib.rs");
    WorkspaceMember::from_manifest(root, &manifest_path).expect("parse synthetic member")
}

fn write_workspace_root(root: &Path, members: &[&str]) {
    let quoted: Vec<String> = members.iter().map(|m| format!("\"{m}\"")).collect();
    let toml = format!("[workspace]\nresolver = \"3\"\nmembers = [{}]\n", quoted.join(", "));
    fs::write(root.join("Cargo.toml"), toml).expect("write synthetic workspace Cargo.toml");
}

#[test]
fn table_has_43_rows_and_561_kloc() {
    assert_eq!(validate_table(CRATE_MAP, TOMBSTONES), Ok(()));
    let counted: Vec<&CrateRow> = counted_rows().collect();
    assert_eq!(counted.len(), 43);
    assert_eq!(counted.iter().filter(|r| matches!(r.layer, Layer::Core(_))).count(), 39);
    assert_eq!(counted.iter().filter(|r| matches!(r.layer, Layer::Annex)).count(), 4);
    assert_eq!(
        counted.iter().filter(|r| matches!(r.layer, Layer::Core(_)) && r.dir.starts_with("crates/")).count(),
        38
    );
    assert_eq!(lookup("frob-mcp").expect("frob-mcp row").dir, "surface/frob-mcp");
    assert_eq!(total_cap_kloc(), 561);
}

#[test]
fn layer_sums_match() {
    let expected = [
        (Layer::Core(0), 28),
        (Layer::Core(1), 40),
        (Layer::Core(2), 84),
        (Layer::Core(3), 92),
        (Layer::Core(4), 98),
        (Layer::Core(5), 78),
        (Layer::Core(6), 62),
        (Layer::Core(7), 47),
        (Layer::Annex, 32),
    ];
    let mut grand_total = 0u32;
    for (layer, expected_sum) in expected {
        let actual = layer_cap_kloc(layer);
        assert_eq!(actual, expected_sum, "cap sum mismatch for layer {layer}");
        grand_total += actual;
    }
    assert_eq!(grand_total, 561);
}

#[test]
fn frontier_only_sum_is_74() {
    let fm_only: Vec<&str> = counted_rows()
        .filter(|r| matches!(r.layer, Layer::Core(_)) && r.is_frontier_or_moonshot_only())
        .map(|r| r.name)
        .collect();
    assert_eq!(
        fm_only,
        vec![
            "frob-gait",
            "frob-shield",
            "frob-ground",
            "frob-synth",
            "frob-foundry",
            "frob-lifecycle",
            "frob-mission",
            "frob-tui",
        ]
    );
    let fm_sum = frontier_or_moonshot_only_core_cap_kloc();
    assert_eq!(fm_sum, 74);
    assert_eq!(total_cap_kloc() - fm_sum, 487);
}

#[test]
fn trigger_is_645() {
    assert_eq!(trigger_cap_kloc(total_cap_kloc()), 645);
}

#[test]
fn ranks_are_contiguous_per_layer() {
    let layers = [
        Layer::Core(0),
        Layer::Core(1),
        Layer::Core(2),
        Layer::Core(3),
        Layer::Core(4),
        Layer::Core(5),
        Layer::Core(6),
        Layer::Core(7),
        Layer::Annex,
    ];
    for layer in layers {
        let mut ranks: Vec<u8> = counted_rows().filter(|r| r.layer == layer).map(|r| r.rank).collect();
        ranks.sort_unstable();
        let max_rank = u8::try_from(ranks.len()).expect("layer size fits in u8");
        let expected: Vec<u8> = (1..=max_rank).collect();
        assert_eq!(ranks, expected, "non-contiguous ranks in layer {layer}");
    }

    // Negative control: a table with a duplicated rank must fail validate_table.
    let mut bad = CRATE_MAP.to_vec();
    let idx = bad.iter().position(|r| r.name == "frob-dyn").expect("frob-dyn row");
    bad[idx].rank = 2; // duplicates frob-contact (3.2)
    let errs = validate_table(&bad, TOMBSTONES).expect_err("duplicated rank must fail");
    assert!(
        errs.iter().any(|e| matches!(e, TableError::DuplicateRankInLayer { layer: Layer::Core(3), rank: 2 })),
        "expected DuplicateRankInLayer, got {errs:?}"
    );
    assert!(
        errs.iter().any(|e| matches!(e, TableError::NonContiguousRanks { layer: Layer::Core(3), .. })),
        "expected NonContiguousRanks, got {errs:?}"
    );
}

#[test]
fn names_are_unique() {
    let mut seen = BTreeSet::new();
    for row in CRATE_MAP {
        assert!(seen.insert(row.name), "duplicate crate name: {}", row.name);
    }
}

#[test]
fn dirs_are_unique() {
    let mut seen = BTreeSet::new();
    for row in CRATE_MAP {
        assert!(seen.insert(row.dir), "duplicate crate dir: {}", row.dir);
    }
}

#[test]
fn intra_layer_order_examples() {
    let arena = lookup("frob-arena").expect("frob-arena");
    let opt = lookup("frob-opt").expect("frob-opt");
    assert_eq!((arena.layer, arena.rank), (Layer::Core(4), 6));
    assert_eq!((opt.layer, opt.rank), (Layer::Core(4), 7));
    assert!(arena.is_below(opt) && !opt.is_below(arena));

    let contact = lookup("frob-contact").expect("frob-contact");
    let dyn_crate = lookup("frob-dyn").expect("frob-dyn");
    assert_eq!((contact.layer, contact.rank), (Layer::Core(3), 2));
    assert_eq!((dyn_crate.layer, dyn_crate.rank), (Layer::Core(3), 3));
    assert!(contact.is_below(dyn_crate) && !dyn_crate.is_below(contact));

    let cert = lookup("frob-cert").expect("frob-cert");
    let verbs = lookup("frob-verbs").expect("frob-verbs");
    assert_eq!((cert.layer, cert.rank), (Layer::Core(6), 1));
    assert_eq!((verbs.layer, verbs.rank), (Layer::Core(6), 4));
    assert!(cert.is_below(verbs) && !verbs.is_below(cert));
}

#[test]
fn unlisted_member_is_finding() {
    let root = make_scratch_dir("unlisted");
    write_workspace_root(&root, &["crates/*"]);
    write_member(&root, "crates/frob-core", "frob-core", "[package.metadata.frob]\nlayer = 0\nrank = 1");
    write_member(&root, "crates/frob-extra", "frob-extra", "[package.metadata.frob]\nlayer = 2\nrank = 6");

    let members = discover_workspace_members(&root).expect("discover members");
    let findings = validate(&members, &[]);
    assert_eq!(findings.len(), 1, "expected 1 finding, got {findings:?}");
    assert_eq!(findings[0].rule, RULE_UNLISTED_CRATE);
    assert_eq!(findings[0].subject, "frob-extra");
    assert_eq!(findings[0].path, "crates/frob-extra/Cargo.toml");
}

#[test]
fn metadata_mismatch_is_finding() {
    let root = make_scratch_dir("meta-mismatch");
    write_workspace_root(&root, &["crates/*"]);
    write_member(&root, "crates/frob-contact", "frob-contact", "[package.metadata.frob]\nlayer = 3\nrank = 3");
    write_member(&root, "crates/frob-core", "frob-core", "[package.metadata.frob]\nlayer = 1\nrank = 1");

    let members = discover_workspace_members(&root).expect("discover members");
    let findings = validate(&members, &[]);
    assert_eq!(findings.len(), 2, "expected 2 findings, got {findings:?}");
    assert!(findings.iter().all(|f| f.rule == RULE_METADATA_MISMATCH));
    assert_eq!(findings[0].subject, "frob-contact");
    assert_eq!(findings[1].subject, "frob-core");
}

#[test]
fn missing_metadata_is_finding() {
    let root = make_scratch_dir("meta-missing");
    write_workspace_root(&root, &["crates/*"]);
    write_member(&root, "crates/frob-core", "frob-core", "");
    write_member(&root, "crates/frob-fsim", "frob-fsim", "[package.metadata.frob]\nlayer = 0");

    let members = discover_workspace_members(&root).expect("discover members");
    let findings = validate(&members, &[]);
    assert_eq!(findings.len(), 2, "expected 2 findings, got {findings:?}");
    assert!(findings.iter().all(|f| f.rule == RULE_METADATA_MISMATCH));
    let subjects: Vec<&str> = findings.iter().map(|f| f.subject.as_str()).collect();
    assert_eq!(subjects, vec!["frob-core", "frob-fsim"]);
}

#[test]
fn addition_requires_reason_and_adr() {
    // 1. Original row carrying Some(adr) must fail.
    let mut orig_with_adr = CRATE_MAP.to_vec();
    orig_with_adr[0].adr = Some("ADR-WS-99");
    let errs1 = validate_table(&orig_with_adr, TOMBSTONES).expect_err("Original with Some(adr) must fail");
    assert!(errs1.iter().any(|e| matches!(e, TableError::OriginalWithAdr { name, .. } if name == "frob-core")));

    // 2. Addition beyond original 43 with Reason::Dependency but adr: None must fail.
    let mut add_no_adr = CRATE_MAP.to_vec();
    add_no_adr.push(CrateRow {
        name: "frob-new",
        dir: "crates/frob-new",
        layer: Layer::Core(2),
        rank: 6,
        tiers: &[Tier::Solid],
        chapters: &["§14"],
        cap_kloc: 5,
        reason: Reason::Dependency,
        adr: None,
        conditional: None,
    });
    let errs2 = validate_table(&add_no_adr, TOMBSTONES).expect_err("non-Original without ADR must fail");
    assert!(errs2.iter().any(|e| matches!(e, TableError::AdditionWithoutAdr { name } if name == "frob-new")));

    // 3. Addition beyond original 43 with Reason::Original and adr: None must fail.
    let mut add_orig = CRATE_MAP.to_vec();
    add_orig.push(CrateRow {
        name: "frob-sneaky",
        dir: "crates/frob-sneaky",
        layer: Layer::Core(2),
        rank: 6,
        tiers: &[Tier::Solid],
        chapters: &["§14"],
        cap_kloc: 5,
        reason: Reason::Original,
        adr: None,
        conditional: None,
    });
    let errs3 = validate_table(&add_orig, TOMBSTONES).expect_err("44th counted row with Original must fail");
    assert!(errs3.iter().any(|e| matches!(e, TableError::AdditionWithOriginalReason { name } if name == "frob-sneaky")));

    // 4. Proper addition citing a valid reason and ADR passes.
    let mut add_valid = CRATE_MAP.to_vec();
    add_valid.push(CrateRow {
        name: "frob-new-ok",
        dir: "crates/frob-new-ok",
        layer: Layer::Core(2),
        rank: 6,
        tiers: &[Tier::Solid],
        chapters: &["§14"],
        cap_kloc: 5,
        reason: Reason::ProofBoundary,
        adr: Some("ADR-WS-99"),
        conditional: None,
    });
    assert_eq!(validate_table(&add_valid, TOMBSTONES), Ok(()));
}

#[test]
fn tombstoned_name_cannot_return() {
    let tombstones = [Tombstone { name: "frob-lifecycle", adr: "ADR-WS-42" }];
    let errs = validate_table(CRATE_MAP, &tombstones).expect_err("tombstoned crate in table must fail");
    assert!(errs.iter().any(|e| matches!(
        e,
        TableError::TombstonedName { name, adr } if name == "frob-lifecycle" && adr == "ADR-WS-42"
    )));

    let root = make_scratch_dir("tombstone-member");
    let member = write_member(&root, "crates/frob-lifecycle", "frob-lifecycle", "[package.metadata.frob]\nlayer = 5\nrank = 7");
    let rows_without: Vec<CrateRow> = CRATE_MAP.iter().copied().filter(|r| r.name != "frob-lifecycle").collect();
    let findings = validate_with_table(&rows_without, &tombstones, &[member], &[]);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule, RULE_UNLISTED_CRATE);
    assert!(findings[0].message.contains("ADR-WS-42"));
}

#[test]
fn conditional_row_not_counted() {
    let units = lookup("frob-units").expect("frob-units conditional row");
    assert_eq!(units.conditional, Some("INT-TOOLCHAIN-01"));
    assert_eq!(units.layer, Layer::Core(0));
    assert_eq!(units.cap_kloc, 3);
    assert!(!units.is_counted());
    assert_eq!(counted_rows().count(), 43);
    assert_eq!(layer_cap_kloc(Layer::Core(0)), 28);
    assert_eq!(total_cap_kloc(), 561);

    // Case 1: frob-units absent -> no finding.
    let root_absent = make_scratch_dir("units-absent");
    write_workspace_root(&root_absent, &["crates/*"]);
    write_member(&root_absent, "crates/frob-core", "frob-core", "[package.metadata.frob]\nlayer = 0\nrank = 1");
    let members_absent = discover_workspace_members(&root_absent).expect("discover");
    assert!(validate(&members_absent, &[]).is_empty());

    // Case 2: frob-units present on disk with matching metadata -> accepted.
    let root_present = make_scratch_dir("units-present");
    write_workspace_root(&root_present, &["crates/*"]);
    write_member(&root_present, "crates/frob-core", "frob-core", "[package.metadata.frob]\nlayer = 0\nrank = 1");
    write_member(&root_present, "crates/frob-units", "frob-units", "[package.metadata.frob]\nlayer = 0\nrank = 0");
    let members_present = discover_workspace_members(&root_present).expect("discover");
    assert!(validate(&members_present, &[]).is_empty());

    // Case 3: frob-units present WITHOUT conditional row -> ADR-WS-01.unlisted-crate.
    let rows_without_conditional: Vec<CrateRow> = CRATE_MAP.iter().copied().filter(|r| r.name != "frob-units").collect();
    let findings = validate_with_table(&rows_without_conditional, TOMBSTONES, &members_present, &[]);
    assert_eq!(findings.len(), 1, "expected finding without conditional row: {findings:?}");
    assert_eq!(findings[0].rule, RULE_UNLISTED_CRATE);
    assert_eq!(findings[0].subject, "frob-units");
}

#[test]
fn apps_link_packages_exempt() {
    let root = make_scratch_dir("apps-link");
    write_workspace_root(&root, &["apps/*", "crates/*"]);
    write_member(&root, "apps/frob", "frob", "");
    write_member(&root, "apps/frobd", "frobd", "");
    write_member(&root, "apps/frob-extra", "frob-extra", "");
    // Package under apps/ masquerading as a listed core crate must still fail Rule 7.
    write_member(&root, "apps/frob-core", "frob-core", "[package.metadata.frob]\nlayer = 0\nrank = 1");
    // Package named "frob" outside apps/ must NOT be exempted.
    write_member(&root, "crates/frob", "frob", "");

    let members = discover_workspace_members(&root).expect("discover");
    let findings = validate(&members, &["frob", "frobd"]);
    assert_eq!(findings.len(), 3, "expected 3 findings, got {findings:?}");
    assert!(findings.iter().all(|f| f.rule == RULE_UNLISTED_CRATE));
    let paths: Vec<&str> = findings.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(
        paths,
        vec![
            "crates/frob/Cargo.toml",
            "apps/frob-core/Cargo.toml",
            "apps/frob-extra/Cargo.toml",
        ]
    );
}

#[test]
fn non_rust_caps_and_determinism() {
    assert_eq!(NON_RUST_CAPS.len(), 4);
    assert_eq!(NON_RUST_CAPS[0].cap_kloc, Some(20));
    assert_eq!(NON_RUST_CAPS[1].cap_kloc, Some(12));
    assert_eq!(NON_RUST_CAPS[2].cap_kloc, Some(1));
    assert_eq!(NON_RUST_CAPS[3].cap_kloc, None);

    let root = make_scratch_dir("det");
    let m1 = write_member(&root, "crates/frob-extra-b", "frob-extra-b", "");
    let m2 = write_member(&root, "crates/frob-extra-a", "frob-extra-a", "");
    let f_ab = validate(&[m1.clone(), m2.clone()], &[]);
    let f_ba = validate(&[m2, m1], &[]);
    assert_eq!(f_ab, f_ba);
    assert_eq!(f_ab.len(), 2);
}
