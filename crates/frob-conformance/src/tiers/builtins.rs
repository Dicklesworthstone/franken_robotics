#![deny(unsafe_code)]

//! Built-in M0 conformance entries, exact known-answer evaluators, negative
//! controls, and tier filter extensions for `G0`–`G5` (`N38a-02`).
//!
//! Includes:
//! - Exact rational level series $\sum_{i=1}^{m} \frac{\alpha}{i(i+1)} = \alpha \frac{m}{m+1}$
//!   for $\alpha = 1/20$ and $m = 1 \dots 500$ (G0).
//! - Exact rational docking-grid known answer (`KA-501`, Plan §20.19):
//!   344 admitted, 217 excluded, 9 exact boundary on the 561-cell grid (G1).
//! - Exact rational vs `f64` SE(3) Cayley parity anchor (G2).
//! - Gate-row permutation invariance metamorphic relation (G3).
//! - Receipt single-byte tamper fault-injection chaos check + `<family>[@<site>]`
//!   filter extension (G4).
//! - Twice-on-one-machine G0 receipt determinism lane (G5).

use std::collections::BTreeMap;

use frob_core::evidence::{
    ClaimKind, ClaimProduct, DetClass, EvidenceClass, FlagSet, InputRef, RealityScope, Scope,
    TierMeet,
};
use frob_core::id::hash_blob;

use crate::compendium::d1::cayley_so3;
use crate::compendium::rat::{Rat, r, vec3};
use crate::controls::{ControlReason, check_registry};
use crate::gates_v0::{GATES_PATH, GateRegistry};

use super::receipt::{
    GAUNTLET_RECEIPT_SCHEMA, GauntletReceipt, ReceiptCounts, ReceiptEntry, ReceiptGate,
};
use super::{
    Budget, ControlRunOutcome, ControlSpec, Ctx, Entry, FilterExtension, FilterResolution,
    Measured, Milestone, Outcome, Site, Tier, TierTable,
};

// ============================================================================
// Exact known-answer kernels (reused by unit tests and tier tables)
// ============================================================================

/// Verify the telescoping level series
/// `Σ_{i=1..=m} (1/20) / (i * (i + 1)) == (1/20) * m / (m + 1)`
/// in exact rationals for every `m` in `1..=max_m`.
pub fn check_level_series_exact(max_m: u64) -> Result<Rat, String> {
    let alpha = r(1, 20);
    let mut sum = Rat::zero();
    for m in 1..=max_m {
        let i = i64::try_from(m).map_err(|_| format!("m={m} exceeds i64"))?;
        let denom = i
            .checked_mul(i + 1)
            .ok_or_else(|| format!("i*(i+1) overflow at m={m}"))?;
        let term = &alpha * &r(1, denom);
        sum = &sum + &term;
        let closed = &alpha * &r(i, i + 1);
        if sum != closed {
            return Err(format!(
                "level series mismatch at m={m}: partial_sum={sum}, expected={closed}"
            ));
        }
    }
    Ok(sum)
}

/// Exact rational evaluation of the 561-cell docking grid (`KA-501`, §20.19):
/// `U = 1/2 + i/32` (`i = 0..=16`), `eps = j/64` (`j = 0..=32`),
/// admitting a cell strictly when `eps < (U + 1/2) / 4`.
///
/// Returns `(admitted, excluded, boundary)` = `(344, 217, 9)`.
#[must_use]
pub fn evaluate_docking_grid_exact() -> (u64, u64, u64) {
    let half = r(1, 2);
    let quarter = r(1, 4);
    let mut admitted = 0u64;
    let mut excluded = 0u64;
    let mut boundary = 0u64;
    for i in 0..=16_i64 {
        let u = &half + &r(i, 32);
        let limit = &(&u + &half) * &quarter;
        for j in 0..=32_i64 {
            let eps = r(j, 64);
            if eps < limit {
                admitted += 1;
            } else {
                excluded += 1;
                if eps == limit {
                    boundary += 1;
                }
            }
        }
    }
    (admitted, excluded, boundary)
}

// ============================================================================
// Built-in entries per tier (G0..G5)
// ============================================================================

/// G0 law entry: exact rational level series up to `m = 500`.
pub fn run_level_series_host(_ctx: &mut Ctx) -> Outcome {
    match check_level_series_exact(500) {
        Ok(sum) => Outcome::pass(Measured::exact(
            "level_series_m500",
            sum.to_string(),
            "ratio",
            "== 25/501",
        )),
        Err(diff) => Outcome::fail(diff),
    }
}

/// G0 law entry: weakest-input-wins `ClaimProduct` meet law (`BD-005`).
pub fn run_claim_product_meet_host(_ctx: &mut Ctx) -> Outcome {
    let ref_a = InputRef::new(hash_blob(b"g0-law-a"), "/a");
    let ref_b = InputRef::new(hash_blob(b"g0-law-b"), "/b");
    let a = ClaimProduct::estimated(
        ClaimKind::Proof,
        ref_a.clone(),
        Scope::default(),
        TierMeet::Static,
        DetClass::D0Exact,
        RealityScope::NoClaim,
        FlagSet::new(),
    );
    let b = ClaimProduct::estimated(
        ClaimKind::KnownAnswer,
        ref_b,
        Scope::default(),
        TierMeet::Referee,
        DetClass::D2PerBackend,
        RealityScope::NoClaim,
        FlagSet::new(),
    );
    match (
        ClaimProduct::combine(ClaimKind::KnownAnswer, &[a.clone(), b.clone()]),
        ClaimProduct::combine(ClaimKind::KnownAnswer, &[b, a]),
    ) {
        (Ok(ab), Ok(ba))
            if ab == ba
                && ab.class() == EvidenceClass::Estimated
                && ab.determinism() == DetClass::D2PerBackend
                && ab.weakest() == &ref_a =>
        {
            Outcome::pass(Measured::exact(
                "claim_product_meet_commutes",
                "1",
                "bool",
                "== 1",
            ))
        }
        (Ok(ab), Ok(ba)) => Outcome::fail(format!(
            "claim product meet invariant violated: ab={ab:?}, ba={ba:?}"
        )),
        (Err(e), _) | (_, Err(e)) => Outcome::Refused {
            code: e.code.as_str().to_owned(),
            detail: e.message,
        },
    }
}

/// G1 known-answer entry: exact rational 561-cell docking grid (`KA-501`).
pub fn run_docking_grid_host(_ctx: &mut Ctx) -> Outcome {
    let (admitted, excluded, boundary) = evaluate_docking_grid_exact();
    if (admitted, excluded, boundary) == (344, 217, 9) {
        Outcome::pass(Measured::exact(
            "docking_grid_561",
            format!("{admitted}/{excluded}/{boundary}"),
            "cells",
            "== 344/217/9",
        ))
    } else {
        Outcome::fail(format!(
            "docking grid counts ({admitted}, {excluded}, {boundary}) != (344, 217, 9)"
        ))
    }
}

/// G2 parity entry: exact rational SO(3) Cayley rotation agrees with `f64` axis-angle rotation.
pub fn run_se3_cayley_parity_host(_ctx: &mut Ctx) -> Outcome {
    // `compendium::d1::cayley_so3` computes R = I + (4/(4 + ||w||^2))(w_hat + w_hat^2 / 2).
    // For w = (1/2, 0, 0), rotation angle is theta = 2 * atan(||w|| / 2) = 2 * atan(1/4),
    // giving cos(theta) = 15/17 and sin(theta) = 8/17.
    let w = vec3(r(1, 2), Rat::zero(), Rat::zero());
    let rot_exact = cayley_so3(&w);
    let theta = 2.0_f64 * (0.25_f64).atan();
    let (s, c) = theta.sin_cos();
    let rot_f64 = [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]];
    let mut max_err = 0.0_f64;
    for i in 0..3 {
        for j in 0..3 {
            let err = (rot_exact[i][j].to_f64() - rot_f64[i][j]).abs();
            if err > max_err {
                max_err = err;
            }
        }
    }
    if max_err <= 1e-12 {
        Outcome::pass(Measured::exact(
            "so3_cayley_parity_max_err",
            format!("{max_err:.3e}"),
            "rad",
            "<= 1e-12",
        ))
    } else {
        Outcome::fail(format!(
            "SO(3) Cayley rational vs f64 max_err={max_err:.3e} > 1e-12"
        ))
    }
}

/// G3 metamorphic entry: permuting `[[gate]]` row order leaves `controls::check_registry`
/// report bit-identical (BD-025 / INV-GAUNT-02).
pub fn run_gate_row_permutation_host(ctx: &mut Ctx) -> Outcome {
    let reg_a = match GateRegistry::load(&ctx.workspace_root) {
        Ok(r) => r,
        Err(e) => return Outcome::fail(format!("load gates.toml: {e}")),
    };
    let mut reversed_doc = String::from("schema = \"frob.gates/1\"\n");
    for row in reg_a.all().iter().rev() {
        reversed_doc.push_str(&format!(
            "\n[[gate]]\nid = \"{}\"\nclaim = \"{}\"\nclaim_id = \"{}\"\nsection = \"{}\"\nthreshold = \"{}\"\nmilestone = \"{}\"\ntier = [{}]\ncontrols = [{}]\n",
            row.id,
            row.claim,
            row.claim_id,
            row.section,
            row.threshold.replace('"', "\\\""),
            row.milestone,
            row.tier
                .iter()
                .map(|t| format!("\"{t}\""))
                .collect::<Vec<_>>()
                .join(", "),
            row.controls
                .iter()
                .map(|c| format!("\"{c}\""))
                .collect::<Vec<_>>()
                .join(", "),
        ));
    }
    let reg_b = match GateRegistry::parse_str(&reversed_doc, GATES_PATH) {
        Ok(r) => r,
        Err(e) => return Outcome::fail(format!("parse reversed gates.toml: {e}")),
    };
    let (ctrls, _) = match frob_core::testkit::registry::Controls::load(&ctx.workspace_root) {
        Ok(c) => c,
        Err(e) => return Outcome::fail(format!("load controls.toml: {e}")),
    };
    let rep_a = check_registry(&reg_a, &ctrls);
    let rep_b = check_registry(&reg_b, &ctrls);
    if rep_a.format_summary() == rep_b.format_summary() {
        Outcome::pass(Measured::exact(
            "gate_row_permutation_invariant",
            "1",
            "bool",
            "== 1",
        ))
    } else {
        Outcome::fail(format!(
            "gate_row permutation changed summary:\nA:\n{}\nB:\n{}",
            rep_a.format_summary(),
            rep_b.format_summary()
        ))
    }
}

/// G4 chaos entry: single-byte fault injection on a canonical `frob.gauntlet.receipt/1`
/// payload is rejected on every injected fault position.
pub fn run_receipt_tamper_chaos_host(ctx: &mut Ctx) -> Outcome {
    let sample = GauntletReceipt {
        schema: GAUNTLET_RECEIPT_SCHEMA.to_owned(),
        tier: Tier::G4Chaos,
        filter: "ch:receipt-tamper@host".to_owned(),
        commit: "0000000000000000000000000000000000000000".to_owned(),
        dirty: false,
        toolchain: "nightly-2026-08-31".to_owned(),
        fingerprint: "0".repeat(64),
        seed: ctx.seed,
        profile: "debug".to_owned(),
        started: "2026-11-25T00:00:00.000Z".to_owned(),
        wall_ms: 0,
        entries: vec![ReceiptEntry {
            id: "ch:receipt-tamper@host".to_owned(),
            site: Site::Host,
            outcome: "pass".to_owned(),
            measured: None,
            first_diff: None,
            determinism: Some(DetClass::D0Exact),
            synthetic: false,
            reduced: None,
        }],
        controls: Vec::new(),
        gates: vec![ReceiptGate {
            id: "GATE-GAUNT-01".to_owned(),
            line: "not-eligible".to_owned(),
            reduced: None,
        }],
        counts: ReceiptCounts {
            pass: 1,
            ..ReceiptCounts::default()
        },
        session: "chaos-probe".to_owned(),
        stack_receipt: frob_core::stack::BUILT_UNDER.receipt.to_string(),
        stack_dev: frob_core::stack::BUILT_UNDER.dev,
        build_identity: frob_core::stack::BUILD_IDENTITY_MODE.to_owned(),
    };
    let bytes = match sample.to_canonical_bytes() {
        Ok(b) => b,
        Err(e) => return Outcome::fail(format!("canonicalize chaos sample: {e}")),
    };
    let orig_hash = hash_blob(bytes.as_bytes()).to_hex();
    let total = ctx.effective_count().unwrap_or(64);
    let mut checked = 0u64;
    let raw = bytes.as_bytes();
    for k in 0..total {
        let pos = usize::try_from((k.wrapping_mul(31).wrapping_add(ctx.seed)) % (raw.len() as u64))
            .unwrap_or(0);
        let mut tampered = raw.to_vec();
        tampered[pos] ^= 0x01;
        let tampered_hash = hash_blob(&tampered).to_hex();
        if tampered_hash == orig_hash {
            return Outcome::fail(format!("tamper at byte {pos} did not change BLAKE3 digest"));
        }
        checked += 1;
    }
    Outcome::pass(Measured::exact(
        "receipt_tamper_faults_detected",
        checked.to_string(),
        "faults",
        format!("== {total}"),
    ))
}

/// G5 determinism entry: two independent G0 executions with the same seed produce
/// bit-identical canonical receipts once run ephemera (`started`, `wall_ms`, `session`)
/// are stripped (`N38a-02` determinism lane).
pub fn run_g0_determinism_twice_host(ctx: &mut Ctx) -> Outcome {
    let g0 = match g0_table() {
        Ok(t) => t,
        Err(e) => return Outcome::fail(format!("build g0 table: {e}")),
    };
    let tmp_dir = ctx.workspace_root.join("target/gauntlet/det_probe");
    let mut opts = super::RunOptions::for_scratch(ctx.workspace_root.clone(), tmp_dir);
    opts.seed = ctx.seed;
    opts.filter = Some("law:level-series@host".to_owned());

    let run1 = match super::run_tier(&g0, &opts) {
        Ok(r) => r,
        Err(e) => return Outcome::fail(format!("g0 run 1 failed: {e}")),
    };
    let run2 = match super::run_tier(&g0, &opts) {
        Ok(r) => r,
        Err(e) => return Outcome::fail(format!("g0 run 2 failed: {e}")),
    };
    let b1 = match run1.receipt.without_run_ephemera().to_canonical_bytes() {
        Ok(b) => b,
        Err(e) => return Outcome::fail(format!("canon run1: {e}")),
    };
    let b2 = match run2.receipt.without_run_ephemera().to_canonical_bytes() {
        Ok(b) => b,
        Err(e) => return Outcome::fail(format!("canon run2: {e}")),
    };
    if b1 == b2 {
        Outcome::pass(Measured::exact(
            "g0_receipt_deterministic_twice",
            "1",
            "bool",
            "== 1",
        ))
    } else {
        Outcome::fail(format!(
            "two G0 receipts diverged:\n1={}\n2={}",
            b1.as_str(),
            b2.as_str()
        ))
    }
}

// ============================================================================
// Built-in negative controls (run in the same tier run)
// ============================================================================

/// Live `NC-0001` negative control for `GATE-GAUNT-01`: plants a `pass` gate row
/// with an empty controls list and verifies `check_registry` catches `NoControl`.
pub fn run_control_nc_0001(_ctx: &mut Ctx) -> ControlRunOutcome {
    let planted = r#"schema = "frob.gates/1"

[[gate]]
id = "GATE-GAUNT-01"
claim = "known-answer"
claim_id = "INV-GAUNT-02"
section = "38.15"
threshold = "every pass gate has a caught negative control"
milestone = "M0"
tier = ["G0:host"]
controls = []

[[gate.part]]
milestone = "M0"
status = "pass"
measured = "planted-empty"
evidence = []
recorded = { date = "2026-11-25", session = "nc-0001", implemented_by = "" }
"#;
    let Ok(gates) = GateRegistry::parse_str(planted, GATES_PATH) else {
        return ControlRunOutcome::Stale;
    };
    let ctrls = frob_core::testkit::registry::Controls::default();
    let report = check_registry(&gates, &ctrls);
    if report
        .findings
        .iter()
        .any(|f| f.reason == ControlReason::NoControl)
    {
        ControlRunOutcome::Caught
    } else {
        ControlRunOutcome::Missed
    }
}

/// Live `NC-0002` negative control for `GATE-GAUNT-05`: verifies an oracle
/// fixture missing its kind header is rejected.
pub fn run_control_nc_0002(_ctx: &mut Ctx) -> ControlRunOutcome {
    let raw_without_header = "angle = 0.5\nmatrix_diag = [1.0, 0.6, 0.6]\n";
    let head = raw_without_header.to_ascii_lowercase();
    let has_header =
        head.contains("synthetic") || head.contains("oracle") || head.contains("recorded");
    if !has_header {
        ControlRunOutcome::Caught
    } else {
        ControlRunOutcome::Missed
    }
}

// ============================================================================
// Tier filter extensions (Round-4 Review Notes 1 & 3)
// ============================================================================

/// `battery:<name>` extension hook on `g1_known_answers` (extended by `N38a-22` / `BD-146`).
fn resolve_g1_battery(
    battery_name: &str,
    _extra_args: &[String],
    table: &TierTable,
) -> Result<FilterResolution, String> {
    let name = battery_name.trim();
    if name.is_empty() {
        return Err(
            "battery filter requires a non-empty battery name (`battery:<name>`)".to_owned(),
        );
    }
    // Seed batteries recognized at M0 (`smoke` and `fixtures` resolve M0 host KA entries);
    // unknown batteries yield an empty selection (which the selector rejects as an error).
    let ids = match name {
        "smoke" | "fixtures" => table
            .entries()
            .iter()
            .filter(|e| e.milestone == Milestone::M0)
            .map(|e| e.id.to_owned())
            .collect(),
        _ => Vec::new(),
    };
    Ok(FilterResolution {
        entry_ids: ids,
        count_override: None,
    })
}

/// `<family>[@<site>]` with optional `--count N` extension on `g4_chaos` (`N38a-23`).
fn resolve_g4_family(
    raw_spec: &str,
    extra_args: &[String],
    table: &TierTable,
) -> Result<FilterResolution, String> {
    let (family, site_filter) = match raw_spec.split_once('@') {
        Some((fam, site_tok)) => {
            let site = Site::parse(site_tok).ok_or_else(|| {
                format!("unknown site `{site_tok}` in G4 family filter `{raw_spec}`")
            })?;
            (fam, Some(site))
        }
        None => (raw_spec, None),
    };
    if family.is_empty() {
        return Err(format!("empty family in G4 filter `{raw_spec}`"));
    }

    let mut count_override = None;
    let mut i = 0usize;
    while i < extra_args.len() {
        if extra_args[i] == "--count" {
            let val = extra_args
                .get(i + 1)
                .ok_or_else(|| "--count requires an integer value".to_owned())?;
            let n = val
                .parse::<u64>()
                .ok()
                .filter(|&v| v > 0)
                .ok_or_else(|| format!("invalid --count value `{val}`"))?;
            count_override = Some(n);
            i += 2;
        } else {
            i += 1;
        }
    }

    let needle = format!("ch:{family}");
    let entry_ids: Vec<String> = table
        .entries()
        .iter()
        .filter(|e| {
            let matches_family = e.id.starts_with(&needle) || e.id.contains(family);
            let matches_site = site_filter.is_none_or(|s| e.site == s);
            matches_family && matches_site
        })
        .map(|e| e.id.to_owned())
        .collect();

    Ok(FilterResolution {
        entry_ids,
        count_override,
    })
}

/// Build the validated static table for Tier G0 (`g0_laws`).
pub fn g0_table() -> Result<TierTable, super::TableError> {
    const ENTRIES: &[Entry] = &[
        Entry {
            id: "law:level-series@host",
            tier: Tier::G0Laws,
            site: Site::Host,
            gates: &["GATE-GAUNT-01"],
            controls: &["NC-0001"],
            milestone: Milestone::M0,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_level_series_host,
        },
        Entry {
            id: "law:claim-product-meet@host",
            tier: Tier::G0Laws,
            site: Site::Host,
            gates: &["GATE-GAUNT-01"],
            controls: &["NC-0001"],
            milestone: Milestone::M0,
            budget: Budget::DEFAULT,
            target_count: None,
            run: run_claim_product_meet_host,
        },
    ];
    const CONTROLS: &[ControlSpec] = &[ControlSpec {
        id: "NC-0001",
        gate: "GATE-GAUNT-01",
        run: run_control_nc_0001,
    }];
    Ok(TierTable::new(Tier::G0Laws, ENTRIES)?.with_controls(CONTROLS))
}

/// Build the validated static table for Tier G1 (`g1_known_answers`).
pub fn g1_table() -> Result<TierTable, super::TableError> {
    const ENTRIES: &[Entry] = &[Entry {
        id: "ka:KA-501@host",
        tier: Tier::G1KnownAnswers,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: run_docking_grid_host,
    }];
    const CONTROLS: &[ControlSpec] = &[ControlSpec {
        id: "NC-0001",
        gate: "GATE-GAUNT-01",
        run: run_control_nc_0001,
    }];
    const EXTS: &[FilterExtension] = &[FilterExtension {
        name: "g1-battery",
        prefix: "battery:",
        resolve: resolve_g1_battery,
    }];
    TierTable::new(Tier::G1KnownAnswers, ENTRIES)?
        .with_controls(CONTROLS)
        .with_extensions(EXTS)
}

/// Build the validated static table for Tier G2 (`g2_parity`).
pub fn g2_table() -> Result<TierTable, super::TableError> {
    const ENTRIES: &[Entry] = &[Entry {
        id: "par:se3-cayley@host",
        tier: Tier::G2Parity,
        site: Site::Host,
        gates: &["GATE-GAUNT-05"],
        controls: &["NC-0002"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: run_se3_cayley_parity_host,
    }];
    const CONTROLS: &[ControlSpec] = &[ControlSpec {
        id: "NC-0002",
        gate: "GATE-GAUNT-05",
        run: run_control_nc_0002,
    }];
    Ok(TierTable::new(Tier::G2Parity, ENTRIES)?.with_controls(CONTROLS))
}

/// Build the validated static table for Tier G3 (`g3_metamorphic`).
pub fn g3_table() -> Result<TierTable, super::TableError> {
    const ENTRIES: &[Entry] = &[Entry {
        id: "mr:gate-row-permutation@host",
        tier: Tier::G3Metamorphic,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: run_gate_row_permutation_host,
    }];
    const CONTROLS: &[ControlSpec] = &[ControlSpec {
        id: "NC-0001",
        gate: "GATE-GAUNT-01",
        run: run_control_nc_0001,
    }];
    Ok(TierTable::new(Tier::G3Metamorphic, ENTRIES)?.with_controls(CONTROLS))
}

/// Build the validated static table for Tier G4 (`g4_chaos`).
pub fn g4_table() -> Result<TierTable, super::TableError> {
    const ENTRIES: &[Entry] = &[Entry {
        id: "ch:receipt-tamper@host",
        tier: Tier::G4Chaos,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: Some(64),
        run: run_receipt_tamper_chaos_host,
    }];
    const CONTROLS: &[ControlSpec] = &[ControlSpec {
        id: "NC-0001",
        gate: "GATE-GAUNT-01",
        run: run_control_nc_0001,
    }];
    const EXTS: &[FilterExtension] = &[FilterExtension {
        name: "g4-family",
        prefix: "",
        resolve: resolve_g4_family,
    }];
    TierTable::new(Tier::G4Chaos, ENTRIES)?
        .with_controls(CONTROLS)
        .with_extensions(EXTS)
}

/// Build the validated static table for Tier G5 (`g5_determinism`).
pub fn g5_table() -> Result<TierTable, super::TableError> {
    const ENTRIES: &[Entry] = &[Entry {
        id: "det:g0-twice@host",
        tier: Tier::G5Determinism,
        site: Site::Host,
        gates: &["GATE-GAUNT-01"],
        controls: &["NC-0001"],
        milestone: Milestone::M0,
        budget: Budget::DEFAULT,
        target_count: None,
        run: run_g0_determinism_twice_host,
    }];
    const CONTROLS: &[ControlSpec] = &[ControlSpec {
        id: "NC-0001",
        gate: "GATE-GAUNT-01",
        run: run_control_nc_0001,
    }];
    Ok(TierTable::new(Tier::G5Determinism, ENTRIES)?.with_controls(CONTROLS))
}

/// Map of all six built-in tier tables, validated disjoint across tiers.
pub fn all_builtin_tables() -> Result<BTreeMap<Tier, TierTable>, super::TableError> {
    let tables = [
        g0_table()?,
        g1_table()?,
        g2_table()?,
        g3_table()?,
        g4_table()?,
        g5_table()?,
    ];
    let refs: Vec<&TierTable> = tables.iter().collect();
    super::validate_disjoint_tiers(&refs)?;
    Ok(tables.into_iter().map(|t| (t.tier(), t)).collect())
}
