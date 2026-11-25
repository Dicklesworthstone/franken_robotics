#![deny(unsafe_code)]

//! Registration-before-run validation rules (Rules 1–6), all twelve pilot loader
//! reasons (`N38-REF`), effective `void` computation, and frozen
//! `sampling_episodes_per_design` lookup (`N38a-09`).
//!
//! Determinism class D0; prints nothing (rule L-7); zero `unsafe`.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use frob_core::id::hash_blob;

use crate::pilots::schema::{
    PILOTS_PATH, PilotLoadError, PilotRecord, PilotRegister, PilotStatus, PilotTier,
    is_plan_time_ledger_id, is_valid_pilot_id, parse_register_raw,
};
use crate::reasons::HarnessReason;

/// All twelve pilot register loader reasons (`N38a-09`, `N38-REF`).
pub const PILOT_LOAD_REASONS: &[HarnessReason] = &[
    HarnessReason::PilotIdForm,
    HarnessReason::Reused,
    HarnessReason::Removed,
    HarnessReason::PilotPartMissing,
    HarnessReason::PilotCannotFail,
    HarnessReason::ResultProtocolMismatch,
    HarnessReason::ResultBeforeRegistration,
    HarnessReason::PilotReceiptUncommitted,
    HarnessReason::FallbackTierNotLower,
    HarnessReason::FallbackAlsoUnknown,
    HarnessReason::TierSWithoutRequiredBy,
    HarnessReason::PlanTimeRecordUnknown,
];

/// Options governing cross-file and historical checks when loading `pilots.toml`.
#[derive(Debug, Clone, Default)]
pub struct LoadOptions<'a> {
    /// Workspace root used to verify committed receipt files (`receipts/pilots/<hash>.json`)
    /// and protocol file digests (Rule 6 `void`).
    pub root: Option<&'a Path>,
    /// Previous commit / baseline snapshot for append-only (`removed` / `reused`)
    /// and frozen-field checks.
    pub previous: Option<&'a PilotRegister>,
    /// When `true` (default for [`load_register`]), verify every result's receipt
    /// exists byte-identical at `receipts/pilots/<hash>.json` under `root`.
    pub check_receipts: bool,
}

impl<'a> LoadOptions<'a> {
    /// Full workspace validation options against `root`.
    #[must_use]
    pub const fn for_root(root: &'a Path) -> Self {
        Self {
            root: Some(root),
            previous: None,
            check_receipts: true,
        }
    }

    /// Attach a previous snapshot for append-only (`removed` / `reused`) validation.
    #[must_use]
    pub const fn with_previous(mut self, previous: &'a PilotRegister) -> Self {
        self.previous = Some(previous);
        self
    }

    /// Toggle receipt-file existence and BLAKE3 verification (always `true` on real loads).
    #[must_use]
    pub const fn with_receipt_checks(mut self, check_receipts: bool) -> Self {
        self.check_receipts = check_receipts;
        self
    }
}

/// Load and validate `crates/frob-conformance/registry/pilots.toml` under `root`.
pub fn load_register(root: &Path) -> Result<PilotRegister, PilotLoadError> {
    load_register_with_options(root, LoadOptions::for_root(root))
}

/// Load and validate `pilots.toml` under `root` with custom [`LoadOptions`].
pub fn load_register_with_options(
    root: &Path,
    opts: LoadOptions<'_>,
) -> Result<PilotRegister, PilotLoadError> {
    let path = root.join(PILOTS_PATH);
    let bytes = fs::read(&path)
        .map_err(|e| PilotLoadError::parse(PILOTS_PATH, None, format!("read failed: {e}")))?;
    parse_and_validate(&bytes, PILOTS_PATH, opts)
}

/// Parse and validate `pilots.toml` bytes with explicit [`LoadOptions`].
pub fn parse_and_validate(
    bytes: &[u8],
    file: &str,
    opts: LoadOptions<'_>,
) -> Result<PilotRegister, PilotLoadError> {
    let reg = parse_register_raw(bytes, file)?;
    validate_register(&reg, file, opts)?;
    Ok(reg)
}

/// Validate an already-parsed [`PilotRegister`] against all registration rules
/// (Rules 1–6) and all twelve load reasons.
pub fn validate_register(
    reg: &PilotRegister,
    file: &str,
    opts: LoadOptions<'_>,
) -> Result<(), PilotLoadError> {
    // Pass 1: identifier grammar, uniqueness (`reused`), and append-only vs `previous` (`removed`).
    let mut ids_present: BTreeSet<&str> = BTreeSet::new();
    let mut mech_tiers: BTreeMap<&str, PilotTier> = BTreeMap::new();
    for rec in &reg.pilots {
        if !is_valid_pilot_id(&rec.id) {
            return Err(PilotLoadError::with_reason(
                file,
                Some(rec.id.clone()),
                HarnessReason::PilotIdForm,
                format!("pilot identifier `{}` violates grammar", rec.id),
            ));
        }
        if !ids_present.insert(rec.id.as_str()) {
            return Err(PilotLoadError::with_reason(
                file,
                Some(rec.id.clone()),
                HarnessReason::Reused,
                format!("duplicate or reused pilot identifier `{}`", rec.id),
            ));
        }
        mech_tiers.insert(rec.mechanism.as_str(), rec.tier);
    }

    if let Some(prev) = opts.previous {
        for old in &prev.pilots {
            match reg.get(&old.id) {
                None => {
                    return Err(PilotLoadError::with_reason(
                        file,
                        Some(old.id.clone()),
                        HarnessReason::Removed,
                        format!("append-only pilot identifier `{}` was removed", old.id),
                    ));
                }
                Some(curr) => {
                    // Reusing an existing identifier for a different mechanism violates append-only identity.
                    if curr.mechanism != old.mechanism {
                        return Err(PilotLoadError::with_reason(
                            file,
                            Some(old.id.clone()),
                            HarnessReason::Reused,
                            format!(
                                "pilot identifier `{}` was reused for mechanism `{}` (previously `{}`)",
                                old.id, curr.mechanism, old.mechanism
                            ),
                        ));
                    }
                }
            }
        }
    }

    // Pass 2: per-record rules (Rules 2, 2a, 3, 4, 5, 5a, and `plan_time_records`).
    for rec in &reg.pilots {
        validate_record(rec, file, &ids_present, &mech_tiers, &opts)?;
    }

    Ok(())
}

fn validate_record(
    rec: &PilotRecord,
    file: &str,
    ids_present: &BTreeSet<&str>,
    mech_tiers: &BTreeMap<&str, PilotTier>,
    opts: &LoadOptions<'_>,
) -> Result<(), PilotLoadError> {
    // Rule 5a: tier = "S" requires non-empty `required_by`.
    if rec.tier == PilotTier::S
        && rec
            .required_by
            .as_deref()
            .map(str::trim)
            .is_none_or(str::is_empty)
    {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::TierSWithoutRequiredBy,
            "record with `tier = \"S\"` must declare non-empty `required_by` naming the chapter subsection",
        ));
    }

    // Rule 3 & Rule 4: four mandatory parts (`falsifier`, `ceiling`, `fallback.ships`, `fallback.kill_gate`)
    // and refusal of pilots that cannot fail.
    validate_falsifier(rec, file)?;
    validate_ceiling(rec, file)?;
    validate_fallback(rec, file, ids_present, mech_tiers)?;

    // `plan_time_records`: every entry must name an existing plan-time ledger record (`PILOT-PT-*`).
    for pt_id in &rec.plan_time_records {
        if !is_plan_time_ledger_id(pt_id) || !ids_present.contains(pt_id.as_str()) {
            return Err(PilotLoadError::with_reason(
                file,
                Some(rec.id.clone()),
                HarnessReason::PlanTimeRecordUnknown,
                format!(
                    "`plan_time_records` references `{pt_id}`, which is not an existing plan-time ledger record"
                ),
            ));
        }
    }

    // Rule 2 & Rule 2a: validate every appended `[[pilot.result]]`.
    let reg_hash = rec.protocol.normalized_hash();
    for (ridx, res) in rec.results.iter().enumerate() {
        let res_hash = res.normalized_protocol_hash();
        match (reg_hash, res_hash) {
            (Some(expected), Some(observed)) if expected == observed => {}
            _ => {
                return Err(PilotLoadError::with_reason(
                    file,
                    Some(rec.id.clone()),
                    HarnessReason::ResultProtocolMismatch,
                    format!(
                        "result[{ridx}] `protocol_blake3` `{}` does not match registered `protocol.blake3` `{}`",
                        res.protocol_blake3, rec.protocol.blake3
                    ),
                ));
            }
        }

        // Result date and (if present) ledger_seq must follow registration.
        if rec.protocol.frozen.trim().is_empty()
            || compare_dates(&res.date, &rec.protocol.frozen) == Ordering::Less
        {
            return Err(PilotLoadError::with_reason(
                file,
                Some(rec.id.clone()),
                HarnessReason::ResultBeforeRegistration,
                format!(
                    "result[{ridx}] date `{}` predates protocol registration `{}`",
                    res.date, rec.protocol.frozen
                ),
            ));
        }
        if let Some(ref res_seq) = res.ledger_seq {
            if rec.protocol.ledger_seq.trim().is_empty()
                || compare_ledger_seq(res_seq, &rec.protocol.ledger_seq) != Ordering::Greater
            {
                return Err(PilotLoadError::with_reason(
                    file,
                    Some(rec.id.clone()),
                    HarnessReason::ResultBeforeRegistration,
                    format!(
                        "result[{ridx}] ledger_seq `{res_seq}` does not follow registration ledger_seq `{}`",
                        rec.protocol.ledger_seq
                    ),
                ));
            }
        }

        // Rule 2a: committed receipt byte-identical at `receipts/pilots/<hash>.json`.
        let Some(expected_hash) = res.receipt_hash() else {
            return Err(PilotLoadError::with_reason(
                file,
                Some(rec.id.clone()),
                HarnessReason::PilotReceiptUncommitted,
                format!(
                    "result[{ridx}] receipt `{}` is not of the form `blake3:<64-lower-hex>`",
                    res.receipt
                ),
            ));
        };
        if opts.check_receipts
            && let Some(root) = opts.root
        {
            let rel = format!("receipts/pilots/{expected_hash}.json");
            let full = root.join(&rel);
            let bytes = fs::read(&full).map_err(|_| {
                PilotLoadError::with_reason(
                    file,
                    Some(rec.id.clone()),
                    HarnessReason::PilotReceiptUncommitted,
                    format!("result[{ridx}] committed receipt absent at `{rel}`"),
                )
            })?;
            let actual_hash = hash_blob(&bytes).to_hex();
            if actual_hash != expected_hash {
                return Err(PilotLoadError::with_reason(
                    file,
                    Some(rec.id.clone()),
                    HarnessReason::PilotReceiptUncommitted,
                    format!(
                        "result[{ridx}] receipt `{rel}` hashes to `{actual_hash}`, expected `{expected_hash}`"
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn validate_falsifier(rec: &PilotRecord, file: &str) -> Result<(), PilotLoadError> {
    let raw = rec.falsifier.trim();
    if raw.is_empty() {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::PilotCannotFail,
            "`falsifier` is empty (a pilot that cannot fail is not a pilot)",
        ));
    }

    let low = raw.to_ascii_lowercase();
    let tautology = matches!(
        low.as_str(),
        "none"
            | "n/a"
            | "na"
            | "null"
            | "never"
            | "cannot fail"
            | "can't fail"
            | "always passes"
            | "always pass"
            | "always succeeds"
            | "always true"
            | "no falsifier"
            | "none possible"
            | "certain"
    ) || low.contains("cannot fail")
        || low.contains("can't fail")
        || low.contains("never fails")
        || low.contains("impossible to fail");

    if tautology {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::PilotCannotFail,
            format!(
                "`falsifier` `{raw}` cannot fail (expected an observable failure condition with a number or exact count)"
            ),
        ));
    }

    let has_digit = raw.bytes().any(|b| b.is_ascii_digit());
    let has_exact_count_word = has_word(&low, "zero")
        || has_word(&low, "one")
        || has_word(&low, "two")
        || has_word(&low, "three")
        || has_word(&low, "any")
        || has_word(&low, "every")
        || has_word(&low, "no");
    let has_comparison_or_predicate = raw.contains('<')
        || raw.contains('>')
        || raw.contains('=')
        || raw.contains('%')
        || raw.contains('×')
        || low.contains("disagree")
        || low.contains("differ")
        || low.contains("mismatch")
        || low.contains("below")
        || low.contains("above")
        || low.contains("under")
        || low.contains("over")
        || low.contains("exceed")
        || low.contains("less")
        || low.contains("greater")
        || low.contains("worse")
        || low.contains("slower")
        || low.contains("fail")
        || low.contains("miss")
        || low.contains("violat")
        || low.contains("diverge")
        || low.contains("reproduce")
        || low.contains("at least")
        || low.contains("at most");

    if !(has_digit || (has_exact_count_word && has_comparison_or_predicate)) {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::PilotCannotFail,
            format!(
                "`falsifier` `{raw}` does not state a measurable number or exact-count comparison that can fail"
            ),
        ));
    }
    Ok(())
}

fn validate_ceiling(rec: &PilotRecord, file: &str) -> Result<(), PilotLoadError> {
    let val = rec.ceiling.value.trim();
    let stmt = rec.ceiling.statement.trim();
    let basis = rec.ceiling.basis.trim();
    if val.is_empty() || stmt.is_empty() || basis.is_empty() {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::PilotPartMissing,
            "mandatory part `ceiling` requires non-empty `statement`, `value`, and `basis` (or `value = \"none on speed\"` with a reason in `basis`)",
        ));
    }
    Ok(())
}

fn validate_fallback(
    rec: &PilotRecord,
    file: &str,
    ids_present: &BTreeSet<&str>,
    mech_tiers: &BTreeMap<&str, PilotTier>,
) -> Result<(), PilotLoadError> {
    let ships = rec.fallback.ships.trim();
    if ships.is_empty() {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::PilotPartMissing,
            "missing mandatory part `fallback.ships`",
        ));
    }

    let kill_gate = rec.fallback.kill_gate.trim();
    if kill_gate.is_empty() {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::PilotPartMissing,
            "missing mandatory part `fallback.kill_gate`",
        ));
    }

    // Round-4 Review Note 4 (slice S-14): `kill_gate = "none"` is admitted ONLY
    // on a `tier = "S"` record whose `required_by` subsection attaches no kill gate.
    if kill_gate == "none" {
        if rec.tier != PilotTier::S {
            return Err(PilotLoadError::with_reason(
                file,
                Some(rec.id.clone()),
                HarnessReason::PilotPartMissing,
                format!(
                    "`fallback.kill_gate = \"none\"` is only permitted on `tier = \"S\"` records (found tier `{}`)",
                    rec.tier
                ),
            ));
        }
    } else if kill_gate != "to-register"
        && !kill_gate.starts_with("KILL-")
        && !kill_gate.starts_with("GATE-")
    {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::PilotPartMissing,
            format!(
                "`fallback.kill_gate` `{kill_gate}` must name a gate identifier (`KILL-...` / `GATE-...`), `to-register`, or (for tier S only) `none`"
            ),
        ));
    }

    // Rule 5: `fallback.ships` alone must satisfy the lower-tier rule (never satisfied by `also`).
    let fallback_tier = infer_fallback_tier(rec, mech_tiers);
    if !rec.tier.admits_fallback_tier(fallback_tier) {
        return Err(PilotLoadError::with_reason(
            file,
            Some(rec.id.clone()),
            HarnessReason::FallbackTierNotLower,
            format!(
                "`fallback.ships` `{ships}` has tier `{fallback_tier}`, which is not lower than pilot tier `{}` (`fallback.also` never satisfies this rule)",
                rec.tier
            ),
        ));
    }

    // Rule 5 (`fallback.also`): every entry must name an existing pilot record.
    for (aidx, also) in rec.fallback.also.iter().enumerate() {
        if also.ships.trim().is_empty() {
            return Err(PilotLoadError::with_reason(
                file,
                Some(rec.id.clone()),
                HarnessReason::PilotPartMissing,
                format!("`fallback.also[{aidx}].ships` must be non-empty"),
            ));
        }
        if !ids_present.contains(also.pilot.as_str()) {
            return Err(PilotLoadError::with_reason(
                file,
                Some(rec.id.clone()),
                HarnessReason::FallbackAlsoUnknown,
                format!(
                    "`fallback.also[{aidx}]` names unknown pilot record `{}`",
                    also.pilot
                ),
            ));
        }
    }

    Ok(())
}

/// Determine the tier of `rec.fallback.ships`:
/// 1. Explicit `fallback.tier` when supplied.
/// 2. Tier of a matching mechanism in `pilots.toml`.
/// 3. Explicit inline tier tag (`[M]`/`[F]`/`[S]` or word `Moonshot`/`Frontier`/`Solid`).
/// 4. Defaults to `S` (Solid — existing baseline mechanism in the design).
fn infer_fallback_tier(rec: &PilotRecord, mech_tiers: &BTreeMap<&str, PilotTier>) -> PilotTier {
    if let Some(explicit) = rec.fallback.tier {
        return explicit;
    }
    let ships = rec.fallback.ships.trim();
    if let Some(&t) = mech_tiers.get(ships) {
        return t;
    }
    if ships.contains("[M]") || has_word(&ships.to_ascii_lowercase(), "moonshot") {
        return PilotTier::M;
    }
    if ships.contains("[F]") || has_word(&ships.to_ascii_lowercase(), "frontier") {
        return PilotTier::F;
    }
    PilotTier::S
}

/// Compute a pilot record's **effective** status under Rule 5b and Rule 6 (`void`).
///
/// A record's effective status becomes [`PilotStatus::Void`] if:
/// - its stored `status` is `void`;
/// - its protocol file exists at `root.join(&rec.protocol.path)` and its BLAKE3
///   digest differs from `rec.protocol.blake3`;
/// - `rec.protocol.registered_sampling_episodes_per_design` differs from
///   `rec.protocol.sampling_episodes_per_design`;
/// - its protocol file declares `sampling_episodes_per_design = <n>` differing from
///   `rec.protocol.sampling_episodes_per_design`;
/// - or `previous` contains the same record with a different `protocol.blake3` or
///   a different `protocol.sampling_episodes_per_design`.
#[must_use]
pub fn effective_status(
    rec: &PilotRecord,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
) -> PilotStatus {
    if rec.status == PilotStatus::Void {
        return PilotStatus::Void;
    }

    if let (Some(curr), Some(reg_val)) = (
        rec.protocol.sampling_episodes_per_design,
        rec.protocol.registered_sampling_episodes_per_design,
    ) && curr != reg_val
    {
        return PilotStatus::Void;
    }

    if let Some(prev_reg) = previous
        && let Some(prev) = prev_reg.get(&rec.id)
    {
        if prev.is_registered() && prev.protocol.normalized_hash() != rec.protocol.normalized_hash()
        {
            return PilotStatus::Void;
        }
        if prev.protocol.sampling_episodes_per_design.is_some()
            && prev.protocol.sampling_episodes_per_design
                != rec.protocol.sampling_episodes_per_design
        {
            return PilotStatus::Void;
        }
    }

    if let Some(ws_root) = root
        && rec.is_registered()
        && !rec.protocol.path.trim().is_empty()
    {
        let proto_path = ws_root.join(&rec.protocol.path);
        if proto_path.is_file()
            && let Ok(bytes) = fs::read(&proto_path)
        {
            if let Some(expected_hex) = rec.protocol.normalized_hash() {
                let actual_hex = hash_blob(&bytes).to_hex();
                if actual_hex != expected_hex {
                    return PilotStatus::Void;
                }
            }
            if let Some(curr_ep) = rec.protocol.sampling_episodes_per_design
                && let Ok(text) = std::str::from_utf8(&bytes)
                && let Some(proto_ep) = parse_sampling_episodes_from_protocol(text)
                && proto_ep != curr_ep
            {
                return PilotStatus::Void;
            }
        }
    }

    rec.status
}

/// Rule 5b: return the frozen `protocol.sampling_episodes_per_design` of a
/// registered, non-void record, and `None` otherwise.
#[must_use]
pub fn sampling_allowance_in(
    reg: &PilotRegister,
    id: &str,
    root: Option<&Path>,
    previous: Option<&PilotRegister>,
) -> Option<u32> {
    let rec = reg.get(id)?;
    if !rec.is_registered() {
        return None;
    }
    if effective_status(rec, root, previous) == PilotStatus::Void {
        return None;
    }
    rec.protocol.sampling_episodes_per_design
}

fn parse_sampling_episodes_from_protocol(text: &str) -> Option<u32> {
    for line in text.lines() {
        let trimmed = line.trim();
        let rest = trimmed
            .strip_prefix("sampling_episodes_per_design")
            .map(str::trim_start)?;
        let val_raw = rest
            .strip_prefix('=')
            .or_else(|| rest.strip_prefix(':'))?
            .trim();
        let digits: String = val_raw.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty()
            && let Ok(v) = digits.parse::<u32>()
            && v >= 1
        {
            return Some(v);
        }
    }
    None
}

fn compare_dates(a: &str, b: &str) -> Ordering {
    a.trim().cmp(b.trim())
}

fn compare_ledger_seq(a: &str, b: &str) -> Ordering {
    let (at, bt) = (a.trim(), b.trim());
    match (at.parse::<u64>(), bt.parse::<u64>()) {
        (Ok(an), Ok(bn)) => an.cmp(&bn),
        _ => at.cmp(bt),
    }
}

fn has_word(haystack_lower: &str, needle_lower: &str) -> bool {
    let b = haystack_lower.as_bytes();
    let n = needle_lower.as_bytes();
    if n.is_empty() || b.len() < n.len() {
        return false;
    }
    let mut i = 0usize;
    while i + n.len() <= b.len() {
        if &b[i..i + n.len()] == n {
            let left_ok = i == 0 || !b[i - 1].is_ascii_alphanumeric();
            let right_ok = i + n.len() == b.len() || !b[i + n.len()].is_ascii_alphanumeric();
            if left_ok && right_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}
