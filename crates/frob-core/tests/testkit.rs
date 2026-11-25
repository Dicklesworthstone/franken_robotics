#![cfg(feature = "test-support")]
#![deny(unsafe_code)]

//! Unit tests for `frob_core::testkit` (`STD-TEST`).
//!
//! All scratch files are written under `${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BronzePelican/`
//! and never deleted (COMMON.md 9b).

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::testkit::det::{
    self, CheckpointDigest, DetClass, Lane, LaneRun, check_d3_decisions, compare_digest_pairs,
};
use frob_core::testkit::gate::{GateOutcome, GateReport, NotEligibleReason, Threshold};
use frob_core::testkit::prop::{self, Gen, Runner, base_seed, blake3_hash, mix};
use frob_core::testkit::registry::{
    self, CompareKind, ControlKind, ControlObservation, ControlOutcome, ControlState, Controls,
    Counterexamples, KaStatus, KnownAnswers, ObligationReason, Observed, PlanCase, Plant,
    Relations, Safe, Tolerance, assert_ka, control, control_expect,
};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn scratch_dir(label: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let n = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let dir = base
        .join("frob-BronzePelican")
        .join(format!("testkit-{label}-{ts}-{pid}-{n}"));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn hex32(bytes: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in bytes {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
    }
    s
}

// ============================================================================
// Property tests (STD-TEST §4)
// ============================================================================

#[test]
fn prop_draw_is_address_function() {
    // Official BLAKE3 vectors verify seed derivation matches standard BLAKE3 bit-for-bit.
    assert_eq!(
        hex32(&blake3_hash(b"")),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );
    assert_eq!(
        hex32(&blake3_hash(b"abc")),
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"
    );

    // Draw (seed 7, case 3, stream 1, index 0) is unchanged when stream 2 is added.
    let mut g_only_s1 = Gen::new(7, 3);
    let draw_before = g_only_s1.next_u64(1);

    let mut g_with_s2 = Gen::new(7, 3);
    let _s2_a = g_with_s2.next_u64(2);
    let _s2_b = g_with_s2.next_u64(2);
    let draw_after = g_with_s2.next_u64(1);

    assert_eq!(draw_before, draw_after);
    assert_eq!(draw_before, mix(7, 3, 1, 0));

    // Metamorphic relation on the kit: permuting case order changes no drawn input.
    let seed = base_seed("frob-core", "prop_draw_is_address_function", 0);
    let forward: Vec<u64> = (0u64..16).map(|c| Gen::new(seed, c).next_u64(1)).collect();
    let mut reverse: Vec<u64> = (0u64..16).rev().map(|c| Gen::new(seed, c).next_u64(1)).collect();
    reverse.reverse();
    assert_eq!(forward, reverse);
}

#[test]
fn prop_shrinks_to_minimal() {
    let runner = Runner::new("frob-core", "prop_shrinks_to_minimal", 64).env_seed(42);
    let failure = runner
        .run_shrink(
            &[],
            |g| g.u64_range(0, 1_000, 10_000_000),
            |&x| if x < 1_000 { Ok(()) } else { Err(format!("{x} >= 1000")) },
        )
        .expect_err("property x < 1000 must fail");
    assert_eq!(failure.shrunk, 1_000);
    assert!(failure.shrink_steps <= prop::MAX_SHRINK_STEPS);
}

#[test]
fn prop_vector_shrink() {
    let (shrunk, steps) = prop::shrink(vec![11u64, 3, 7, 99, 42, 7, 5], |v| !v.contains(&7));
    assert_eq!(shrunk, vec![7u64]);
    assert!(steps <= prop::MAX_SHRINK_STEPS);
}

#[test]
fn prop_replay_line_reproduces() {
    let dir = scratch_dir("prop-replay");
    let reg_file = dir.join("prop_replay_line_reproduces.txt");

    let runner = Runner::new("frob-core", "prop_replay_line_reproduces", 256)
        .env_seed(17)
        .regressions_path(&reg_file);
    let first = runner
        .run_shrink(
            &[],
            |g| g.u64_range(0, 0, 1_000),
            |&x| if x % 23 != 0 || x == 0 { Ok(()) } else { Err(format!("divisible by 23: {x}")) },
        )
        .expect_err("must find a failing case");

    assert!(first.replay_cmd.contains(&format!("FROB_TEST_SEED={}", first.env_seed)));
    assert!(first.replay_cmd.contains(&format!("FROB_PROP_CASE={}", first.case)));
    assert!(first.replay_cmd.contains("cargo test -p frob-core prop_replay_line_reproduces"));

    // Parse printed seed and case from replay_cmd and replay directly.
    let mut parsed_seed = None;
    let mut parsed_case = None;
    for tok in first.replay_cmd.split_whitespace() {
        if let Some(v) = tok.strip_prefix("FROB_TEST_SEED=") {
            parsed_seed = Some(v.parse::<u64>().expect("seed u64"));
        } else if let Some(v) = tok.strip_prefix("FROB_PROP_CASE=") {
            parsed_case = Some(v.parse::<u64>().expect("case u64"));
        }
    }
    let replay_runner = Runner::new("frob-core", "prop_replay_line_reproduces", 256)
        .env_seed(parsed_seed.expect("printed seed"))
        .only_case(parsed_case.expect("printed case"));
    let second = replay_runner
        .run_shrink(
            &[],
            |g| g.u64_range(0, 0, 1_000),
            |&x| if x % 23 != 0 || x == 0 { Ok(()) } else { Err(format!("divisible by 23: {x}")) },
        )
        .expect_err("replay must reproduce failure");
    assert_eq!(second.original, first.original);
    assert_eq!(second.case, first.case);
    assert_eq!(second.seed, first.seed);

    // Regression file was written and replays first on a subsequent run (even with 0 random cases).
    let regs = prop::read_regressions(&reg_file);
    assert_eq!(regs, vec![(first.env_seed, first.case)]);
    let replayed = Runner::new("frob-core", "prop_replay_line_reproduces", 0)
        .env_seed(999)
        .regressions_path(&reg_file)
        .run_shrink(
            &[],
            |g| g.u64_range(0, 0, 1_000),
            |&x| if x % 23 != 0 || x == 0 { Ok(()) } else { Err(format!("divisible by 23: {x}")) },
        )
        .expect_err("regression must replay first even when cases=0");
    assert_eq!(replayed.original, first.original);

    // Also verify float & tuple shrinkers make progress cleanly.
    let (sf, _) = prop::shrink(42.75_f64, |&x| x <= 1.0);
    assert!(sf <= 42.75 && sf > 1.0);
    let (st, _) = prop::shrink((50u64, 50u64), |&(a, b)| a + b < 10);
    assert_eq!(st.0 + st.1, 10);
}

// ============================================================================
// Known-answer loader & exact tolerance comparisons (STD-TEST §5)
// ============================================================================

fn parse_one_ka(extra_body: &str) -> registry::Row {
    let src = format!(
        "schema = \"frob.gauntlet.known-answers/1\"\n\n[[ka]]\n{extra_body}\n"
    );
    let ka = KnownAnswers::parse_str(&src, "synthetic-ka.toml").expect("parse synthetic ka");
    ka.all()[0].clone()
}

#[test]
fn ka_tolerance_kinds() {
    // Exact: rejects 344 against 345, accepts 345.
    let exact_row = parse_one_ka(
        r#"
id = "KA-501"
section = "§20.16"
owner = "GATE-SENS-13"
fixture = "docking-grid"
case = "synthetic exact integer"
truth = { kind = "int", value = 345 }
tolerance = { kind = "exact" }
tier = "G1"
sites = ["host"]
control = { text = "wrong count", nc = ["NC-0412"] }
derivation = "docs/derivations/docking-fixture.md"
label = "DESIGN"
"#,
    );
    assert_eq!(assert_ka(&exact_row, "host", 345i64).status, KaStatus::Pass);
    assert_eq!(assert_ka(&exact_row, "host", 344i64).status, KaStatus::Fail);

    // Docking grid counts (344 admitted, 217 excluded, 9 boundary).
    let counts_row = parse_one_ka(
        r#"
id = "KA-501"
section = "§20.16"
owner = "GATE-SENS-13"
fixture = "docking-grid"
case = "561 docking cells"
truth = { kind = "counts", value = { admitted = 344, excluded = 217, boundary = 9 } }
tolerance = { kind = "exact" }
tier = "G1"
sites = ["host"]
control = { text = "f32 boundary drift", nc = ["NC-0412"] }
derivation = "docs/derivations/docking-fixture.md"
label = "DESIGN"
"#,
    );
    let mut counts = BTreeMap::new();
    counts.insert("admitted".to_owned(), 344);
    counts.insert("excluded".to_owned(), 217);
    counts.insert("boundary".to_owned(), 9);
    assert_eq!(assert_ka(&counts_row, "host", Observed::Counts(counts)).status, KaStatus::Pass);

    // Abs(0.1): accepts 33.49 against 33.52, rejects 34.08.
    let abs_row = parse_one_ka(
        r#"
id = "KA-100"
section = "§15"
fixture = "abs-check"
case = "synthetic abs"
truth = { kind = "real", value = "33.52" }
tolerance = { kind = "abs", value = "0.1" }
tier = "G1"
sites = ["host"]
control = { text = "planted drift", nc = ["NC-0001"] }
derivation = "docs/derivations/abs.md"
label = "DESIGN"
"#,
    );
    assert_eq!(assert_ka(&abs_row, "host", 33.49_f64).status, KaStatus::Pass);
    assert_eq!(assert_ka(&abs_row, "host", 34.08_f64).status, KaStatus::Fail);

    // Rel(0.02): accepts 0.985 against 1, rejects 0.975.
    // Exact binary comparison: 0.98_f64 lies strictly below 98/100 and is rejected under Rel(0.02).
    let rel02_row = parse_one_ka(
        r#"
id = "KA-101"
section = "§15"
fixture = "rel-check"
case = "synthetic rel 0.02"
truth = { kind = "real", value = "1" }
tolerance = { kind = "rel", value = "0.02" }
tier = "G1"
sites = ["host"]
control = { text = "planted drift", nc = ["NC-0001"] }
derivation = "docs/derivations/rel.md"
label = "DESIGN"
"#,
    );
    assert_eq!(assert_ka(&rel02_row, "host", 0.985_f64).status, KaStatus::Pass);
    assert_eq!(assert_ka(&rel02_row, "host", 0.975_f64).status, KaStatus::Fail);
    assert_eq!(
        assert_ka(&rel02_row, "host", 0.98_f64).status,
        KaStatus::Fail,
        "f64 nearest 0.98 lies strictly below 0.98 and must be rejected under Rel(0.02)"
    );
    // Exact decimal "0.98" hits the inclusive boundary and is accepted.
    assert_eq!(assert_ka(&rel02_row, "host", "0.98").status, KaStatus::Pass);

    // Rel(0.25): accepts 0.75_f64 against 1 (0.75 is exact in binary -> inclusive boundary).
    let rel25_row = parse_one_ka(
        r#"
id = "KA-102"
section = "§15"
fixture = "rel-quarter"
case = "synthetic rel 0.25"
truth = { kind = "real", value = "1" }
tolerance = { kind = "rel", value = "0.25" }
tier = "G1"
sites = ["host"]
control = { text = "planted drift", nc = ["NC-0001"] }
derivation = "docs/derivations/rel.md"
label = "DESIGN"
"#,
    );
    assert_eq!(assert_ka(&rel25_row, "host", 0.75_f64).status, KaStatus::Pass);

    // PrintedDigits(4): accepts 0.9207 against 0.92074 (half-ULP = 0.00005), rejects 0.92076.
    let printed_row = parse_one_ka(
        r#"
id = "KA-262"
section = "§24"
fixture = "wheel-loop"
case = "wheel-loop spectral radius"
truth = { kind = "real", value = "0.92074" }
tolerance = { kind = "printed-digits", digits = 4 }
tier = "G1"
sites = ["host"]
control = { text = "explicit Euler mutant 1.497", nc = ["NC-0412"] }
derivation = "docs/derivations/wheel-loop.md"
label = "DESIGN"
"#,
    );
    assert_eq!(assert_ka(&printed_row, "host", 0.9207_f64).status, KaStatus::Pass);
    assert_eq!(assert_ka(&printed_row, "host", 1.497_f64).status, KaStatus::Fail);
}

#[test]
fn ka_obligation_states() {
    let to_set_row = parse_one_ka(
        r#"
id = "KA-200"
section = "§16"
fixture = "to-set-fixture"
case = "synthetic to-set"
truth = { kind = "real", value = "1.0" }
tolerance = { kind = "to-set" }
tier = "G1"
sites = ["host"]
control = { text = "mutant", nc = ["NC-0001"] }
derivation = "docs/derivations/x.md"
label = "DESIGN"
"#,
    );
    assert_eq!(
        assert_ka(&to_set_row, "host", 1.0_f64).status,
        KaStatus::Obligation(ObligationReason::Tolerance)
    );

    let none_stated_row = parse_one_ka(
        r#"
id = "KA-201"
section = "§16"
fixture = "no-ctrl-fixture"
case = "synthetic none stated"
truth = { kind = "int", value = 10 }
tolerance = { kind = "exact" }
tier = "G1"
sites = ["host"]
control = { text = "none stated", nc = [] }
derivation = "docs/derivations/x.md"
label = "DESIGN"
"#,
    );
    assert_eq!(
        assert_ka(&none_stated_row, "host", 10i64).status,
        KaStatus::Obligation(ObligationReason::Control)
    );

    let text_but_empty_nc = parse_one_ka(
        r#"
id = "KA-202"
section = "§16"
fixture = "empty-nc-fixture"
case = "synthetic text without nc"
truth = { kind = "int", value = 10 }
tolerance = { kind = "exact" }
tier = "G1"
sites = ["host"]
control = { text = "wrong sign breaks count", nc = [] }
derivation = "docs/derivations/x.md"
label = "DESIGN"
"#,
    );
    assert_eq!(
        assert_ka(&text_but_empty_nc, "host", 10i64).status,
        KaStatus::Obligation(ObligationReason::Control)
    );
}

#[test]
fn ka_row_full_fields() {
    // Synthetic registry string testing every N38a-12 field + Review Notes 10 & 11
    // (safe, control.removed_check, and late-row empty id resolved by slug).
    let src = r#"
schema = "frob.gauntlet.known-answers/1"

[[ka]]
id = "KA-664"
section = "§26.4"
owner = "GATE-NEMESIS-04"
fixture = "shared-case"
same_as = "KA-010"
case = "counting threshold"
truth = { kind = "int", value = 29957 }
tolerance = { kind = "exact" }
tier = "G1"
sites = ["host", "cuda"]
control = { text = "boundary bypass", nc = ["NC-0412"], removed_check = "count floor guard" }
derivation = "docs/derivations/counting.md"
label = "DESIGN"
due = "M2"
printed = "29956"
slug = "nemesis-count-safe"
safe = "higher"

[[ka]]
id = ""
section = "§23.7"
owner = "GATE-CORTEX-09"
fixture = "offpolicy-fixture"
same_as = ""
case = "provisional unnumbered late row"
truth = { kind = "int", value = 42 }
tolerance = { kind = "exact" }
tier = "G1"
sites = ["host"]
control = { text = "wrong importance weight", nc = ["NC-0412"] }
derivation = "docs/derivations/offpolicy.md"
label = "DESIGN"
due = "M2"
printed = ""
slug = "offpolicy-01"
safe = "lower"
"#;
    let ka = KnownAnswers::parse_str(src, "synthetic-full.toml").expect("parse full KA rows");
    let row = ka.get("KA-664").expect("KA-664 present");
    assert_eq!(row.owner, "GATE-NEMESIS-04");
    assert_eq!(row.same_as, "KA-010");
    assert_eq!(row.sites, vec!["host", "cuda"]);
    assert_eq!(row.printed, "29956");
    assert_eq!(row.slug, "nemesis-count-safe");
    assert_eq!(row.safe, Some(Safe::Higher));
    assert_eq!(row.control.removed_check.as_deref(), Some("count floor guard"));
    let res = assert_ka(row, "cuda", 29_957i64);
    assert_eq!(res.site, "cuda");
    assert_eq!(res.status, KaStatus::Pass);

    // Late row with empty id resolved by slug (Review Note 11).
    let late = ka.get("offpolicy-01").expect("resolve late row by slug");
    assert_eq!(late.id, "");
    assert_eq!(late.slug, "offpolicy-01");
    assert_eq!(late.safe, Some(Safe::Lower));

    // Retired singular `site` must be refused as an unknown key naming the row.
    let bad_site = r#"
[[ka]]
id = "KA-011"
section = "§10"
fixture = "bad-site"
case = "retired singular site"
truth = { kind = "int", value = 1 }
tolerance = { kind = "exact" }
tier = "G1"
sites = ["host"]
site = "host"
control = { text = "none stated", nc = [] }
derivation = "docs/derivations/x.md"
label = "DESIGN"
"#;
    let err = KnownAnswers::parse_str(bad_site, "bad-site.toml").expect_err("retired `site` must fail");
    assert!(err.to_string().contains("KA-011"), "error must name row: {err}");
    assert!(err.to_string().contains("site"), "error must name unknown key: {err}");

    // Empty id + empty slug must fail; invalid safe value must fail.
    let bad_empty = bad_site.replace("id = \"KA-011\"", "id = \"\"").replace("site = \"host\"\n", "");
    assert!(KnownAnswers::parse_str(&bad_empty, "bad-empty.toml").is_err());

    let bad_safe = bad_site.replace("site = \"host\"", "safe = \"up\"");
    let safe_err = KnownAnswers::parse_str(&bad_safe, "bad-safe.toml").expect_err("invalid safe must fail");
    assert!(safe_err.to_string().contains("KA-011"));
}

// ============================================================================
// Controls loader & control() outcomes (STD-TEST §5)
// ============================================================================

#[test]
fn controls_carry_every_field() {
    let src = r#"
schema = "frob.gauntlet.controls/1"

[[control]]
id = "NC-0001"
seed = "CX-262"
gate = ["GATE-A-01", "GATE-B-02"]
kind = "planted-input"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler order"
expect = "spectral radius 1.497"
last_seen = { caught = true, session = "sess-01", date = "2026-11-24" }
feature = "cx-262"
fixture = "fixtures/wheel_loop.toml"
due = "M1"
permanent = true
state = "obligation"
removed_check = "film thickness limit"

[[control]]
id = "NC-0002"
seed = "KA-501"
gate = ["GATE-A-01"]
kind = "mutation"
target = "frob-sense::docking"
defect = "f32 threshold"
expect = "docking grid fails"
"#;
    let controls = Controls::parse_str(src, "synthetic-controls.toml").expect("parse controls");
    assert_eq!(controls.all().len(), 2);

    let c1 = controls.get("NC-0001").expect("NC-0001");
    assert_eq!(c1.last_seen.as_ref().map(|l| l.caught), Some(true));
    assert_eq!(c1.last_seen.as_ref().map(|l| l.session.as_str()), Some("sess-01"));
    assert_eq!(c1.last_seen.as_ref().map(|l| l.date.as_str()), Some("2026-11-24"));
    assert_eq!(c1.feature.as_deref(), Some("cx-262"));
    assert_eq!(c1.fixture.as_deref(), Some("fixtures/wheel_loop.toml"));
    assert_eq!(c1.due.as_deref(), Some("M1"));
    assert!(c1.permanent);
    assert_eq!(c1.state, ControlState::Obligation);
    assert_eq!(c1.removed_check.as_deref(), Some("film thickness limit"));

    let c2 = controls.get("NC-0002").expect("NC-0002");
    assert_eq!(c2.last_seen, None);
    assert!(!c2.permanent);
    assert_eq!(c2.state, ControlState::Runnable);
    assert_eq!(c2.removed_check, None);

    let for_a: Vec<&str> = controls.for_gate("GATE-A-01").iter().map(|c| c.id.as_str()).collect();
    let for_b: Vec<&str> = controls.for_gate("GATE-B-02").iter().map(|c| c.id.as_str()).collect();
    assert_eq!(for_a, vec!["NC-0001", "NC-0002"]);
    assert_eq!(for_b, vec!["NC-0001"]);
}

#[test]
fn control_outcomes() {
    let src = r#"
[[control]]
id = "NC-0412"
seed = "CX-262"
gate = ["GATE-ARENA-04"]
kind = "mutation"
target = "frob-actuate::stage::integrate"
defect = "explicit Euler"
expect = "spectral radius 1.497"
feature = "cx-262"

[[control]]
id = "NC-0413"
seed = "CX-301"
gate = ["GATE-ATEL-01"]
kind = "planted-input"
target = "CX-301"
defect = "word-only obligation"
expect = "refused"
state = "obligation"
"#;
    let controls = Controls::parse_str(src, "outcomes.toml").expect("parse");
    let runnable = controls.get("NC-0412").expect("NC-0412");
    let obligation = controls.get("NC-0413").expect("NC-0413");

    assert_eq!(control(runnable, || ControlOutcome::Caught), ControlOutcome::Caught);
    assert_eq!(control(runnable, || ControlOutcome::Missed), ControlOutcome::Missed);
    assert_eq!(control(runnable, || ControlOutcome::Stale), ControlOutcome::Stale);

    // Failure matching `expect` is Caught; failure for another reason is Missed (caught-unexpected).
    assert_eq!(
        control_expect(runnable, || ControlObservation::FailedWith("wheel-loop spectral radius 1.497".into())),
        ControlOutcome::Caught
    );
    assert_eq!(
        control_expect(runnable, || ControlObservation::FailedWith("unrelated panic: division by zero".into())),
        ControlOutcome::Missed
    );
    assert_eq!(
        control_expect(runnable, || ControlObservation::Stale("plant not built".into())),
        ControlOutcome::Stale
    );

    // Obligation record MUST NOT invoke closure (panicking closure proves it).
    let out = control(obligation, || panic!("obligation closure must never run"));
    assert_eq!(out, ControlOutcome::Obligation);
    let out2 = control_expect(obligation, || panic!("obligation expect closure must never run"));
    assert_eq!(out2, ControlOutcome::Obligation);
}

// ============================================================================
// Relations & Counterexamples loaders (STD-TEST §5 + Review Note 10)
// ============================================================================

#[test]
fn relation_mutation_table_or_array() {
    let src = r#"
schema = "frob.gauntlet.relations/1"

[[relation]]
id = "MR-C01"
statement = "Carried sorting stability relation"
source = ["C-01"]
sites = ["host"]
compare = "leading-order"
bound = "O(dt^2)"
mutation = { name = "unstable", text = "unstable tie comparator", control = "NC-0100", feature = "nc-0100" }
entry = "mr:MR-C01"
owner = "§38"
due = "M2"
generated_from = "C-21.S3"

[[relation]]
id = "MR-01"
statement = "Two-mutation relation"
source = ["C-21"]
sites = ["strict", "cuda"]
compare = "bits"
bound = ""
mutation = [
  { name = "m1", text = "scale by 10", control = "NC-0101", feature = "nc-0101" },
  { name = "m2", text = "drop sign", control = "NC-0102", feature = "nc-0102" },
]
entry = "mr:MR-01"
owner = "§38"
due = "M2"
"#;
    let rels = Relations::parse_str(src, "synthetic-relations.toml").expect("parse relations");
    let c01 = rels.get("MR-C01").expect("MR-C01");
    assert_eq!(c01.mutations.len(), 1);
    assert_eq!(c01.compare, CompareKind::LeadingOrder);
    assert_eq!(c01.generated_from.as_deref(), Some("C-21.S3"));
    assert_eq!(format!("mr_{}_", c01.id.strip_prefix("MR-").unwrap().to_ascii_lowercase()), "mr_c01_");

    let mr01 = rels.get("MR-01").expect("MR-01");
    assert_eq!(mr01.mutations.len(), 2);
    assert_eq!(mr01.compare, CompareKind::Bits);
    assert_eq!(mr01.generated_from, None);
}

#[test]
fn counterexample_fields() {
    let src = r#"
schema = "frob.gauntlet.counterexamples/1"

[[cx]]
id = "CX-019"
shortcut = "Ignore Herve subgroup order"
case = "Sarrus 6R linkage"
numeric = true
plan_case = "numbers"
case_supplied_by = ""
rule = "C-01 subgroup mobility"
rule_ids = ["C-01", "GATE-MARROW-01"]
defined_in = "§13"
same_as = ""
label = "DESIGN"
control_kind = "mutation"
plant = "N13-18:M2"
due = "M2"

[[cx]]
id = "CX-020"
shortcut = "Same plant in table form"
case = "Sarrus 6R linkage"
numeric = true
plan_case = "numbers"
rule = "C-01 subgroup mobility"
rule_ids = ["C-01"]
defined_in = "§13"
label = "FACT (pilot)"
rerun = false
rerun_receipt = ""
control_kind = "planted-input"
plant = [{ id = "N13-18:M2" }]

[[cx]]
id = "CX-176"
shortcut = "Boundary restatement with all nine plant fields"
case = "4 mm film refused"
numeric = true
plan_case = "numbers"
rule = "Film check"
rule_ids = ["GATE-TRAC-04"]
defined_in = "§18"
label = "DESIGN"
control_kind = "planted-input"
plant = [{ id = "N18-CX:cx-176", kind = "planted-input", target = "frob-contact::film", defect = "disable thickness limit", expect = "FROB-REF-301", gates = ["GATE-TRAC-04"], feature = "cx-176", fixture = "fixtures/film_4mm.toml", removed_check = "film thickness limit" }]
slug = "trac-film-limit"
"#;
    let cxs = Counterexamples::parse_str(src, "synthetic-cx.toml").expect("parse counterexamples");
    let c19 = cxs.get("CX-019").expect("CX-019");
    let c20 = cxs.get("CX-020").expect("CX-020");
    let c176 = cxs.get("CX-176").expect("CX-176");

    assert_eq!(c19.rerun, None);
    assert_eq!(c19.due.as_deref(), Some("M2"));
    assert_eq!(c20.rerun, Some(false));
    assert_eq!(c20.due, None);
    assert_eq!(c19.plant, c20.plant);
    assert_eq!(c19.plant, vec![Plant { id: "N13-18:M2".to_owned(), ..Plant::default() }]);

    assert_eq!(c176.plan_case, PlanCase::Numbers);
    assert_eq!(c176.slug.as_deref(), Some("trac-film-limit"));
    assert_eq!(c176.plant.len(), 1);
    let p = &c176.plant[0];
    assert_eq!(p.id, "N18-CX:cx-176");
    assert_eq!(p.kind, Some(ControlKind::PlantedInput));
    assert_eq!(p.target.as_deref(), Some("frob-contact::film"));
    assert_eq!(p.defect.as_deref(), Some("disable thickness limit"));
    assert_eq!(p.expect.as_deref(), Some("FROB-REF-301"));
    assert_eq!(p.gates.as_deref(), Some(&["GATE-TRAC-04".to_owned()][..]));
    assert_eq!(p.feature.as_deref(), Some("cx-176"));
    assert_eq!(p.fixture.as_deref(), Some("fixtures/film_4mm.toml"));
    assert_eq!(p.removed_check.as_deref(), Some("film thickness limit"));

    // Plant table missing required `id` must be a load error naming the row.
    let missing_id = r#"
[[cx]]
id = "CX-099"
shortcut = "bad plant"
case = "case"
numeric = true
plan_case = "numbers"
rule = "rule"
rule_ids = ["GATE-X-01"]
defined_in = "§10"
label = "DESIGN"
control_kind = "mutation"
plant = [{ target = "frob-core::x" }]
"#;
    let err = Counterexamples::parse_str(missing_id, "missing-id.toml").expect_err("missing plant id must fail");
    assert!(err.to_string().contains("CX-099"), "error must name row: {err}");
}

#[test]
fn registry_of_record_loads() {
    let ws = registry::root().expect("workspace root with franken-stack.lock");

    // Verify logical aliases map to the canonical conformance registry paths.
    assert_eq!(
        registry::resolve_path(&ws, "registries/known_answers.toml"),
        ws.join(registry::KNOWN_ANSWERS_PATH)
    );
    assert_eq!(
        registry::resolve_path(&ws, "registries/negative_controls.toml"),
        ws.join(registry::CONTROLS_PATH)
    );

    let (ka, ka_exists) = KnownAnswers::load(&ws).expect("load known_answers.toml");
    let (ctrls, ctrl_exists) = Controls::load(&ws).expect("load controls.toml");
    let (rels, rel_exists) = Relations::load(&ws).expect("load relations.toml");
    let (cxs, cx_exists) = Counterexamples::load(&ws).expect("load counterexamples.toml");

    let ka_obl_tol = ka.all().iter().filter(|r| matches!(r.tolerance, Tolerance::ToSet)).count();
    let ka_obl_ctrl = ka
        .all()
        .iter()
        .filter(|r| r.control.text.trim().eq_ignore_ascii_case("none stated") || r.control.nc.is_empty())
        .count();
    let ctrl_obl = ctrls.all().iter().filter(|c| c.state == ControlState::Obligation).count();

    println!(
        "registry:known_answers rows={} obligation_tolerance={} obligation_control={} present={}",
        ka.all().len(), ka_obl_tol, ka_obl_ctrl, ka_exists
    );
    println!(
        "registry:controls rows={} obligation={} present={}",
        ctrls.all().len(), ctrl_obl, ctrl_exists
    );
    println!("registry:relations rows={} present={}", rels.all().len(), rel_exists);
    println!("registry:counterexamples rows={} present={}", cxs.all().len(), cx_exists);
}

#[test]
fn registry_append_only() {
    let ka_row = |id: &str| {
        format!(
            "[[ka]]\nid = \"{id}\"\nsection = \"§1\"\nfixture = \"f\"\ncase = \"c\"\ntruth = {{ kind = \"int\", value = 1 }}\ntolerance = {{ kind = \"exact\" }}\ntier = \"G1\"\nsites = [\"host\"]\ncontrol = {{ text = \"t\", nc = [\"NC-0001\"] }}\nderivation = \"d.md\"\nlabel = \"DESIGN\"\n"
        )
    };
    assert!(KnownAnswers::parse_str(&format!("{}{}", ka_row("KA-010"), ka_row("KA-010")), "ka.toml").is_err());
    assert!(KnownAnswers::parse_str(&format!("{}{}", ka_row("KA-012"), ka_row("KA-011")), "ka.toml").is_err());

    let cx_row = |id: &str| {
        format!(
            "[[cx]]\nid = \"{id}\"\nshortcut = \"s\"\ncase = \"c\"\nnumeric = true\nplan_case = \"numbers\"\nrule = \"r\"\nrule_ids = [\"G\"]\ndefined_in = \"§1\"\nlabel = \"DESIGN\"\ncontrol_kind = \"mutation\"\n"
        )
    };
    assert!(Counterexamples::parse_str(&format!("{}{}", cx_row("CX-010"), cx_row("CX-010")), "cx.toml").is_err());
    assert!(Counterexamples::parse_str(&format!("{}{}", cx_row("CX-012"), cx_row("CX-011")), "cx.toml").is_err());

    let nc_row = |id: &str| {
        format!(
            "[[control]]\nid = \"{id}\"\nseed = \"CX-010\"\ngate = [\"GATE-A-01\"]\nkind = \"mutation\"\ntarget = \"t\"\ndefect = \"d\"\nexpect = \"e\"\n"
        )
    };
    assert!(Controls::parse_str(&format!("{}{}", nc_row("NC-0010"), nc_row("NC-0010")), "nc.toml").is_err());
    assert!(Controls::parse_str(&format!("{}{}", nc_row("NC-0012"), nc_row("NC-0011")), "nc.toml").is_err());

    let mr_row = |id: &str| {
        format!(
            "[[relation]]\nid = \"{id}\"\nstatement = \"s\"\nsource = [\"C-01\"]\nsites = [\"host\"]\ncompare = \"exact\"\nmutation = {{ name = \"m\", text = \"t\", control = \"NC-0001\", feature = \"nc-0001\" }}\nentry = \"mr:{id}\"\n"
        )
    };
    assert!(Relations::parse_str(&format!("{}{}", mr_row("MR-02"), mr_row("MR-02")), "mr.toml").is_err());
    assert!(Relations::parse_str(&format!("{}{}", mr_row("MR-03"), mr_row("MR-02")), "mr.toml").is_err());
    assert!(Relations::parse_str(&format!("{}{}", mr_row("MR-C03"), mr_row("MR-C02")), "mr.toml").is_err());
}

// ============================================================================
// GateReport & GateLine (STD-TEST §8)
// ============================================================================

#[test]
fn gate_report_no_control() {
    let line = GateReport::new("GATE-ARENA-04")
        .measured("err", "0.001", "m", Threshold::le("0.01"))
        .finish();
    assert_eq!(line.outcome(), &GateOutcome::NotEligible(NotEligibleReason::NoControl));
    assert_eq!(line.to_string(), "not-eligible: no control caught in this run");
    assert!(!line.is_pass());
}

#[test]
fn gate_report_only_obligations() {
    let line = GateReport::new("GATE-ARENA-04")
        .measured("err", "0.001", "m", Threshold::le("0.01"))
        .control("NC-0413", ControlOutcome::Obligation)
        .finish();
    assert_eq!(line.outcome(), &GateOutcome::NotEligible(NotEligibleReason::NoControl));
    assert_eq!(line.to_string(), "not-eligible: no control caught in this run");
    assert!(!line.is_pass());
}

#[test]
fn gate_report_stale_control() {
    // Stale control blocks pass even when another control was caught.
    let line = GateReport::new("GATE-ARENA-04")
        .measured("err", "0.001", "m", Threshold::le("0.01"))
        .control("NC-0412", ControlOutcome::Caught)
        .control("NC-0414", ControlOutcome::Stale)
        .finish();
    assert_eq!(
        line.outcome(),
        &GateOutcome::NotEligible(NotEligibleReason::StaleControl("NC-0414".to_owned()))
    );
    assert_eq!(line.to_string(), "not-eligible: control NC-0414 stale");
    assert!(!line.is_pass());
}

#[test]
fn gate_report_missed_control() {
    // Missed + passing measurements -> fail; missed + stale -> fail (precedence).
    let line = GateReport::new("GATE-ARENA-04")
        .measured("err", "0.001", "m", Threshold::le("0.01"))
        .control("NC-0412", ControlOutcome::Missed)
        .control("NC-0414", ControlOutcome::Stale)
        .finish();
    assert_eq!(line.outcome(), &GateOutcome::Fail);
    assert_eq!(line.to_string(), "fail");
    assert!(!line.is_pass());
}

#[test]
fn gate_report_pass() {
    let ev_dir = scratch_dir("gate-pass-evidence");
    let line = GateReport::new("GATE-ARENA-04")
        .evidence_dir(&ev_dir)
        .measured("err", "0.001", "m", Threshold::le("0.01"))
        .control("NC-0412", ControlOutcome::Caught)
        .finish();
    assert_eq!(line.outcome(), &GateOutcome::Pass);
    assert_eq!(line.to_string(), "pass");
    assert!(line.is_pass());

    // Evidence file has both the caught NC line and the gate pass line.
    let ndjson = fs::read_to_string(ev_dir.join("frob-core.ndjson")).expect("evidence file");
    assert!(ndjson.contains("\"kind\":\"nc\"") && ndjson.contains("\"status\":\"caught\""));
    assert!(ndjson.contains("\"kind\":\"gate\"") && ndjson.contains("\"status\":\"pass\""));
}

#[test]
fn gate_report_no_claim() {
    let ev_dir = scratch_dir("gate-no-claim-evidence");
    let line = GateReport::new("GATE-REAL-01")
        .evidence_dir(&ev_dir)
        .no_claim("no-dataset-admitted")
        .finish();
    assert_eq!(line.outcome(), &GateOutcome::NoClaim("no-dataset-admitted".to_owned()));
    assert_eq!(line.to_string(), "no-claim: no-dataset-admitted");
    assert!(line.is_no_claim());
    assert!(!line.is_pass());

    // Evidence line has status no-claim, reason set, no control line, and observed null.
    let ndjson = fs::read_to_string(ev_dir.join("frob-core.ndjson")).expect("evidence file");
    let lines: Vec<&str> = ndjson.lines().collect();
    assert_eq!(lines.len(), 1, "no-claim must emit no control line: {ndjson}");
    assert!(lines[0].contains("\"status\":\"no-claim\""));
    assert!(lines[0].contains("\"reason\":\"no-dataset-admitted\""));
    assert!(lines[0].contains("\"observed\":null"));

    // Even if measurements or controls are also attached alongside no_claim, it never prints pass.
    let mixed = GateReport::new("GATE-REAL-01")
        .measured("err", "0", "m", Threshold::le("1"))
        .control("NC-0412", ControlOutcome::Caught)
        .no_claim("no-dataset-admitted")
        .finish();
    assert!(!mixed.is_pass());
    assert_ne!(mixed.to_string(), "pass");
}

#[test]
fn gate_line_has_no_constructor() {
    // Compile-fail proof using the live rustc + already-built frob_core rlib.
    // Supports both classic Cargo layout (`<profile>/deps/`) and nightly
    // build-dir layout (`<profile>/build/<crate>/<unit>/out/`).
    let exe = env::current_exe().expect("current_exe");
    let exe_dir = exe.parent().expect("exe parent dir");
    let scratch = scratch_dir("gate-line-compile-fail");

    // Locate `<target>/<profile>` from either `.../deps/<exe>` or `.../build/<crate>/<unit>/out/<exe>`.
    let mut profile_dir = exe_dir.to_path_buf();
    for anc in exe_dir.ancestors() {
        let name = anc.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if name == "deps" || name == "build" {
            if let Some(parent) = anc.parent() {
                profile_dir = parent.to_path_buf();
            }
            break;
        }
    }

    let mut dep_dirs: Vec<PathBuf> = Vec::new();
    let mut push_if_has_artifacts = |dir: PathBuf| {
        if let Ok(rd) = fs::read_dir(&dir) {
            let has_lib = rd.flatten().any(|e| {
                let p = e.path();
                matches!(
                    p.extension().and_then(|s| s.to_str()),
                    Some("rlib" | "rmeta" | "so" | "dylib")
                )
            });
            if has_lib {
                dep_dirs.push(dir);
            }
        }
    };
    push_if_has_artifacts(exe_dir.to_path_buf());
    push_if_has_artifacts(profile_dir.join("deps"));
    let build_root = profile_dir.join("build");
    if build_root.is_dir()
        && let Ok(crates_rd) = fs::read_dir(&build_root)
    {
        for crate_ent in crates_rd.flatten() {
            let crate_path = crate_ent.path();
            if !crate_path.is_dir() {
                continue;
            }
            if let Ok(units_rd) = fs::read_dir(&crate_path) {
                for unit_ent in units_rd.flatten() {
                    let out_dir = unit_ent.path().join("out");
                    if out_dir.is_dir() {
                        push_if_has_artifacts(out_dir);
                    }
                }
            }
        }
    }
    dep_dirs.sort();
    dep_dirs.dedup();

    let mut candidates: Vec<PathBuf> = Vec::new();
    for dir in &dep_dirs {
        if let Ok(rd) = fs::read_dir(dir) {
            for entry in rd.flatten() {
                let p = entry.path();
                let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if (fname.starts_with("libfrob_core-") || fname == "libfrob_core.rlib")
                    && fname.ends_with(".rlib")
                {
                    candidates.push(p);
                }
            }
        }
    }
    candidates.sort();
    candidates.dedup();

    // 1. Positive control probe: locate the exact libfrob_core-*.rlib compiled with test-support.
    let probe_rs = scratch.join("probe.rs");
    fs::write(
        &probe_rs,
        "pub fn probe() -> String { frob_core::testkit::GateReport::new(\"GATE-PROBE\").finish().to_string() }\n",
    )
    .expect("write probe");

    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let mut matched_rlib: Option<PathBuf> = None;
    let mut probe_diags = Vec::new();
    for p in &candidates {
        let probe_out = scratch.join("libprobe.rlib");
        let mut cmd = Command::new(&rustc);
        cmd.args(["--crate-type", "lib", "--edition", "2024", "--extern"])
            .arg(format!("frob_core={}", p.display()));
        for d in &dep_dirs {
            cmd.arg("-L").arg(format!("dependency={}", d.display()));
        }
        let status = cmd
            .arg(&probe_rs)
            .arg("-o")
            .arg(&probe_out)
            .output()
            .expect("run rustc probe");
        if status.status.success() {
            matched_rlib = Some(p.clone());
            break;
        }
        probe_diags.push(format!(
            "{} (exit {:?}): {}",
            p.display(),
            status.status.code(),
            String::from_utf8_lossy(&status.stderr).trim()
        ));
    }
    let rlib = matched_rlib.unwrap_or_else(|| {
        panic!(
            "must find compiled libfrob_core-*.rlib with test-support under {} (rustc={rustc}; dep_dirs={}; tried={probe_diags:?})",
            profile_dir.display(),
            dep_dirs.len()
        )
    });

    // 2. Negative compile-fail check: constructing GateLine by hand MUST fail to compile.
    let bad_rs = scratch.join("bad_gate_line.rs");
    fs::write(
        &bad_rs,
        "use frob_core::testkit::{GateLine, GateOutcome};\n\
         pub fn forge() -> GateLine {\n\
             GateLine { gate: \"GATE-FORGED-01\".to_owned(), outcome: GateOutcome::Pass, _sealed: () }\n\
         }\n",
    )
    .expect("write bad_gate_line.rs");
    let bad_out = scratch.join("libbad.rlib");
    let mut bad_cmd = Command::new(&rustc);
    bad_cmd
        .args(["--crate-type", "lib", "--edition", "2024", "--extern"])
        .arg(format!("frob_core={}", rlib.display()));
    for d in &dep_dirs {
        bad_cmd.arg("-L").arg(format!("dependency={}", d.display()));
    }
    let res = bad_cmd
        .arg(&bad_rs)
        .arg("-o")
        .arg(&bad_out)
        .output()
        .expect("run rustc compile-fail check");
    assert!(
        !res.status.success(),
        "forging GateLine by hand must fail to compile (stdout={}, stderr={})",
        String::from_utf8_lossy(&res.stdout),
        String::from_utf8_lossy(&res.stderr)
    );
    let stderr = String::from_utf8_lossy(&res.stderr);
    assert!(
        stderr.contains("private") || stderr.contains("E0451"),
        "expected private-field compile error, got: {stderr}"
    );
}

// ============================================================================
// Determinism lanes (STD-TEST §6)
// ============================================================================

#[test]
fn det_twice_catches_time_seeded_value() {
    // Passing deterministic example: pure function of fixed seed.
    let pass_res = det::lane(Lane::Twice, DetClass::D0, |_| {
        let digest = blake3_hash(b"deterministic-referee-state-v1");
        vec![CheckpointDigest::new(0, 10, "state_digest", digest)]
    });
    assert!(pass_res.passed, "deterministic twice lane must pass: {}", pass_res.message);

    // Planted negative control: wall-clock / run-index dependent value fails lane `twice`.
    let mut counter = 0u64;
    let fail_res = det::lane(Lane::Twice, DetClass::D0, |_| {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0u128, |d| d.as_nanos())
            ^ u128::from(counter);
        counter += 1;
        let digest = blake3_hash(&nanos.to_le_bytes());
        vec![CheckpointDigest::new(0, 10, "wall_clock_state", digest)]
    });
    assert!(!fail_res.passed, "time-seeded function must fail lane `twice`");
    let diff = fail_res.first_diff.expect("first_diff recorded");
    assert_eq!(diff.checkpoint, 0);
    assert_eq!(diff.quantity, "wall_clock_state");
}

#[test]
fn det_workers_catches_arrival_order_sum() {
    // Values chosen so non-associative f64 arrival-order sum differs between 1 worker and 8 workers.
    let terms: [f64; 16] = [
        1e16, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        -1e16, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];

    // Passing example: canonical index-order reduction (independent of worker count).
    let pass_res = det::lane(Lane::Workers, DetClass::D2, |_| {
        let sum: f64 = terms.iter().copied().sum();
        vec![CheckpointDigest::new(0, 1, "canonical_sum", blake3_hash(&sum.to_bits().to_le_bytes()))]
    });
    assert!(pass_res.passed, "canonical reduction must pass workers lane");

    // Planted negative control: arrival-order reduction by chunk/worker count fails 1 vs 8.
    let fail_res = det::lane(Lane::Workers, DetClass::D2, |run| {
        let workers = match run { LaneRun::Workers(w) => w, _ => 1 };
        let chunk = terms.len() / workers;
        let mut partials = Vec::new();
        for w in 0..workers {
            let s: f64 = terms[w * chunk..(w + 1) * chunk].iter().copied().sum();
            partials.push(s);
        }
        // Reverse arrival order of worker completions when workers > 1.
        let total: f64 = if workers > 1 {
            partials.into_iter().rev().sum()
        } else {
            partials.into_iter().sum()
        };
        vec![CheckpointDigest::new(0, 1, "arrival_order_sum", blake3_hash(&total.to_bits().to_le_bytes()))]
    });
    assert!(!fail_res.passed, "arrival-order sum must fail lane `workers`");
    let diff = fail_res.first_diff.expect("first_diff recorded");
    assert_eq!(diff.quantity, "arrival_order_sum");
}

#[test]
fn det_backends_d3_decision() {
    // Passing D3 example: 300 concordant worlds inside envelope -> 1 - 0.05^(1/300) <= 0.01.
    let concordant: Vec<(f64, f64)> = (0..300)
        .map(|i| {
            let base = if i % 2 == 0 { 1.25 } else { 0.75 };
            (base, base + 1e-6)
        })
        .collect();
    let pass_res = check_d3_decisions(&concordant, 1e-4, 1.0);
    assert!(pass_res.passed, "300 concordant D3 worlds must pass: {}", pass_res.message);
    assert_eq!(pass_res.discordant_count, 0);
    assert!(pass_res.upper_95_discordant <= 0.01);

    // Negative control: 1.0000001 vs 0.9999999 around threshold 1.0 is inside envelope,
    // yet reported as a discordant decision and fails D3 decision equivalence.
    let discordant_res = check_d3_decisions(&[(1.000_000_1, 0.999_999_9)], 1e-4, 1.0);
    assert!(discordant_res.all_within_envelope, "values are inside envelope 1e-4");
    assert_eq!(discordant_res.discordant_count, 1);
    assert_eq!(discordant_res.boundary_sensitive_count, 1);
    assert!(!discordant_res.passed);

    // Weakest-input class rule: never raises a class.
    assert_eq!(DetClass::weakest(DetClass::D0, DetClass::D3), DetClass::D3);
    assert_eq!(DetClass::weakest(DetClass::D4, DetClass::D1), DetClass::D4);
}

#[test]
fn det_pair_helper_reports_first_diff() {
    let make = |mutate_cp3: bool| -> Vec<CheckpointDigest> {
        (0..5)
            .map(|cp| {
                let tag = if mutate_cp3 && cp == 3 {
                    format!("cp-{cp}-mutated")
                } else {
                    format!("cp-{cp}")
                };
                let qty = if cp == 3 { "joint_torque" } else { "state_hash" };
                CheckpointDigest::new(cp, (cp as u64) * 10, qty, blake3_hash(tag.as_bytes()))
            })
            .collect()
    };

    let clean_a = make(false);
    let clean_b = make(false);
    let ok_profiles = compare_digest_pairs(Lane::Profiles, DetClass::D0, &clean_a, &clean_b);
    assert!(ok_profiles.passed);
    assert!(ok_profiles.first_diff.is_none());

    let bad_b = make(true);
    let bad_platforms = compare_digest_pairs(Lane::Platforms, DetClass::D0, &clean_a, &bad_b);
    assert!(!bad_platforms.passed);
    let diff = bad_platforms.first_diff.expect("first_diff at checkpoint 3");
    assert_eq!(diff.checkpoint, 3);
    assert_eq!(diff.step, 30);
    assert_eq!(diff.quantity, "joint_torque");
    assert!(bad_platforms.message.contains("checkpoint 3"));
    assert!(bad_platforms.message.contains("joint_torque"));
}
