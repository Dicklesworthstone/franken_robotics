//! Foundation ledger validation for `franken-stack.lock` (bead `N33a-02`).
//!
//! Enforces the closed 18-foundation ledger invariants over a parsed
//! [`StackLock`] (`N33a-03`): exact foundation ID set, four-form `confined_to`
//! token validation against `crate_map::CRATE_MAP` (`N34-02`), critical-path
//! closure (`FD-FSIM`, `FD-ASUP`, `FD-FSQL`), pattern-only path/version
//! exclusion (`FD-FJAX`, `FD-FFW`, `FD-FSS`), required `first_needed` and four
//! honest `note` rows, and posture-to-graph edge checks through
//! [`crate::stack::posture::edge_allowed`] (`N33a-01`).
//!
//! Pure, deterministic (class D0), zero `unsafe`, prints nothing (rule L-7).

use std::collections::BTreeSet;
use std::fmt;

use crate::crate_map::{CRATE_MAP, Layer};
use crate::stack::lock::{Foundation, StackLock};
use crate::stack::posture::{BEDROCK_DIRECT_CRATES, DepSpec, EdgeKind, Posture, Tier, edge_allowed};

/// Complete closed set of the eighteen foundation identifiers (`§33.4`, `N33a-02`),
/// sorted canonically.
pub const EXPECTED_FOUNDATION_IDS: &[&str] = &[
    "FD-ASUP",
    "FD-CUDA",
    "FD-F3D",
    "FD-FDGR",
    "FD-FFW",
    "FD-FJAX",
    "FD-FKER",
    "FD-FLN",
    "FD-FMCP",
    "FD-FNP",
    "FD-FSCI",
    "FD-FSIM",
    "FD-FSQL",
    "FD-FSS",
    "FD-FSYM",
    "FD-FT",
    "FD-FTUI",
    "FD-FTX",
];

/// The three M1 critical-path foundations (`§33.4`, `N33a-02`).
pub const CRITICAL_PATH_IDS: &[&str] = &["FD-ASUP", "FD-FSIM", "FD-FSQL"];

/// Pattern-only foundation rows that must carry neither `path` nor `version`.
pub const PATTERN_ONLY_IDS: &[&str] = &["FD-FFW", "FD-FJAX", "FD-FSS"];

/// Foundation rows required to carry the four "honest notes" (`§33.4`).
pub const HONEST_NOTE_IDS: &[&str] = &["FD-F3D", "FD-FNP", "FD-FSYM", "FD-FT"];

/// Foundation rows permitted `admission_gate = "none"`; all others must name an `INT-` gate.
pub const GATE_NONE_IDS: &[&str] = &["FD-FFW", "FD-FSS"];

/// Structured validation error against the eighteen-foundation ledger contract (`N33a-02`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerError {
    pub row: String,
    pub field: String,
    pub message: String,
}

impl LedgerError {
    #[must_use]
    pub fn new(
        row: impl Into<String>,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            row: row.into(),
            field: field.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "row `{}` field `{}`: {}", self.row, self.field, self.message)
    }
}

impl std::error::Error for LedgerError {}

/// Validate a single `confined_to` token on `(row_id, posture)` against the four
/// closed token forms of `N33a-02` and `CRATE_MAP`:
/// 1. Core crate name (under `crates/`) or prefixed workspace crate dir (`spark/...`, `derive/...`, `surface/...`, `bindings/...`).
/// 2. `"bedrock-direct"` — permitted only on `FD-FSIM`.
/// 3. `"any"` — permitted only on `FD-ASUP`.
/// 4. Repository-relative directory ending in `/` — permitted only on `Subprocess` or `Pattern` uses.
pub fn validate_confinement_token(
    row_id: &str,
    posture: Posture,
    token: &str,
) -> Result<(), LedgerError> {
    if token.is_empty() || token.trim() != token {
        return Err(LedgerError::new(
            row_id,
            "confined_to",
            format!("invalid empty or whitespace-padded token `{token}`"),
        ));
    }

    if token == "bedrock-direct" {
        if row_id == "FD-FSIM" {
            return Ok(());
        }
        return Err(LedgerError::new(
            row_id,
            "confined_to",
            "`bedrock-direct` token is permitted only on `FD-FSIM`",
        ));
    }

    if token == "any" {
        if row_id == "FD-ASUP" {
            return Ok(());
        }
        return Err(LedgerError::new(
            row_id,
            "confined_to",
            "`any` token is permitted only on `FD-ASUP`",
        ));
    }

    if token.ends_with('/') {
        if !matches!(posture, Posture::Subprocess | Posture::Pattern) {
            return Err(LedgerError::new(
                row_id,
                "confined_to",
                format!(
                    "directory token `{token}` is permitted only on `subprocess` or `pattern` uses, not `{posture}`"
                ),
            ));
        }
        if is_valid_repo_rel_dir_token(token) {
            return Ok(());
        }
        return Err(LedgerError::new(
            row_id,
            "confined_to",
            format!("directory token `{token}` must be a clean repository-relative path ending in `/`"),
        ));
    }

    if is_known_crate_token(token) {
        return Ok(());
    }

    Err(LedgerError::new(
        row_id,
        "confined_to",
        format!("unknown crate token `{token}` (not in CRATE_MAP)"),
    ))
}

fn is_valid_repo_rel_dir_token(token: &str) -> bool {
    let Some(stem) = token.strip_suffix('/') else {
        return false;
    };
    if stem.is_empty()
        || stem.starts_with('/')
        || stem.starts_with('~')
        || stem.contains('\\')
        || stem.contains("://")
    {
        return false;
    }
    stem.split('/')
        .all(|seg| !seg.is_empty() && seg != "." && seg != "..")
}

fn is_known_crate_token(token: &str) -> bool {
    if token.contains('/') {
        // Annex crates (`spark/...`, `derive/...`, `bindings/...`) and `surface/frob-mcp`.
        CRATE_MAP
            .iter()
            .any(|r| r.dir == token && !r.dir.starts_with("crates/"))
    } else {
        // Core crates living under `crates/<name>`.
        CRATE_MAP
            .iter()
            .any(|r| r.name == token && r.layer != Layer::Annex && r.dir.starts_with("crates/"))
    }
}

/// Validate all confinement tokens and non-empty confinement rules across `lock`.
pub fn validate_confinement(lock: &StackLock) -> Result<(), Vec<LedgerError>> {
    let mut errors = Vec::new();
    for row in &lock.foundations {
        let id = row.id.as_str();
        for use_ in &row.uses {
            if matches!(
                use_.posture,
                Posture::Consume | Posture::ConsumePath | Posture::BuildTime
            ) && use_.confined_to.is_empty()
            {
                errors.push(LedgerError::new(
                    id,
                    "confined_to",
                    format!(
                        "`{}` use requires a non-empty `confined_to` list",
                        use_.posture
                    ),
                ));
            }
            for token in &use_.confined_to {
                if let Err(err) = validate_confinement_token(id, use_.posture, token) {
                    errors.push(err);
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate a parsed [`StackLock`] against all `N33a-02` eighteen-foundation ledger invariants.
pub fn validate_ledger(lock: &StackLock) -> Result<(), Vec<LedgerError>> {
    let mut errors = Vec::new();

    // 1. Exact eighteen foundation IDs.
    let expected: BTreeSet<&str> = EXPECTED_FOUNDATION_IDS.iter().copied().collect();
    let observed: BTreeSet<&str> = lock.foundations.iter().map(|f| f.id.as_str()).collect();
    for missing in expected.difference(&observed) {
        errors.push(LedgerError::new(
            *missing,
            "id",
            format!("missing required foundation row `{missing}`"),
        ));
    }
    for extra in observed.difference(&expected) {
        errors.push(LedgerError::new(
            *extra,
            "id",
            format!("unexpected foundation row `{extra}` (ledger holds exactly eighteen rows without an ADR)"),
        ));
    }

    // 2. Per-row invariants.
    let mut observed_critical = BTreeSet::new();
    for row in &lock.foundations {
        let id = row.id.as_str();

        if row.fallback.trim().is_empty() {
            errors.push(LedgerError::new(
                id,
                "fallback",
                "fallback must be non-empty or explicit \"none\"",
            ));
        }

        if GATE_NONE_IDS.contains(&id) {
            if !row.admission_gate.is_none() {
                errors.push(LedgerError::new(
                    id,
                    "admission_gate",
                    format!("expected `admission_gate = \"none\"`, found `{}`", row.admission_gate),
                ));
            }
        } else if row.admission_gate.is_none() {
            errors.push(LedgerError::new(
                id,
                "admission_gate",
                "non-exempt row must name an `INT-` admission gate",
            ));
        }

        if row.first_needed.is_none() {
            errors.push(LedgerError::new(
                id,
                "first_needed",
                "every foundation row must declare `first_needed`",
            ));
        }

        if row.critical_path {
            observed_critical.insert(id);
            let all_non_solid = !row.uses.is_empty()
                && row
                    .uses
                    .iter()
                    .all(|u| matches!(u.tier, Tier::Frontier | Tier::Moonshot));
            if all_non_solid {
                errors.push(LedgerError::new(
                    id,
                    "critical_path",
                    "`critical_path = true` is forbidden on [F]/[M]-only foundations",
                ));
            }
        }

        if PATTERN_ONLY_IDS.contains(&id) {
            if row.path.is_some() {
                errors.push(LedgerError::new(
                    id,
                    "path",
                    "pattern-only foundation must not declare `path`",
                ));
            }
            if row.version.is_some() {
                errors.push(LedgerError::new(
                    id,
                    "version",
                    "pattern-only foundation must not declare `version`",
                ));
            }
        }

        if HONEST_NOTE_IDS.contains(&id)
            && row
                .note
                .as_deref()
                .is_none_or(|n| n.trim().is_empty())
        {
            errors.push(LedgerError::new(
                id,
                "note",
                "required honest `note` is missing or empty",
            ));
        }
    }

    let expected_critical: BTreeSet<&str> = CRITICAL_PATH_IDS.iter().copied().collect();
    if observed_critical != expected_critical {
        errors.push(LedgerError::new(
            "<ledger>",
            "critical_path",
            format!(
                "`critical_path` set {observed_critical:?} does not match expected {expected_critical:?}"
            ),
        ));
    }

    if let Err(mut conf_errs) = validate_confinement(lock) {
        errors.append(&mut conf_errs);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Evaluate whether any declared use of `row` permits a Cargo edge `(from -> dep_name, kind, spec)`
/// strictly through `N33a-01`'s [`edge_allowed`].
#[must_use]
pub fn foundation_allows_edge(
    row: &Foundation,
    from: &str,
    dep_name: &str,
    kind: EdgeKind,
    spec: &DepSpec,
) -> bool {
    let mut full_spec = spec.clone();
    if full_spec.row_version.is_none()
        && let Some(v) = &row.version
    {
        full_spec = full_spec.with_row_version(v.as_str());
    }
    if full_spec.row_path.is_none()
        && let Some(p) = &row.path
    {
        full_spec = full_spec.with_row_path(p.as_str());
    }

    let is_bedrock = row.id.as_str() == "FD-FSIM" && BEDROCK_DIRECT_CRATES.contains(&dep_name);
    row.uses.iter().any(|u| {
        let effective_from = if is_bedrock && u.confined_to.iter().any(|c| c == "bedrock-direct") {
            "bedrock-direct"
        } else {
            from
        };
        edge_allowed(u, effective_from, kind, &full_spec)
    })
}

// ============================================================================
// Unit tests (`cargo test -p xtask stack::ledger`)
// ============================================================================

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::*;
    use crate::stack::lock::{self, parse_bytes};
    use crate::stack::posture::Milestone;

    fn repo_lock_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../franken-stack.lock")
    }

    fn load_repo_lock() -> StackLock {
        lock::load(&repo_lock_path()).expect("committed franken-stack.lock must parse under N33a-03")
    }

    #[test]
    fn ledger_has_eighteen_rows() {
        let lock = load_repo_lock();
        assert_eq!(lock.foundations.len(), 18);

        let observed: BTreeSet<&str> = lock.foundations.iter().map(|f| f.id.as_str()).collect();
        let expected: BTreeSet<&str> = EXPECTED_FOUNDATION_IDS.iter().copied().collect();
        assert_eq!(observed, expected);

        validate_ledger(&lock).expect("committed 18-row ledger passes full validation");

        // Negative control: a nineteenth row for an unadmitted sibling (`franken_networkx`) must fail.
        let raw = std::fs::read_to_string(repo_lock_path()).expect("read franken-stack.lock");
        let with_nineteenth = format!(
            "{raw}\n\
             [[foundation]]\n\
             id = \"FD-FNX\"\n\
             name = \"franken_networkx\"\n\
             posture = \"pattern\"\n\
             confined_to = []\n\
             admission_gate = \"none\"\n\
             fallback = \"none\"\n\
             first_needed = \"M2\"\n"
        );
        let parsed = parse_bytes(with_nineteenth.as_bytes()).expect("19th row parses syntactically");
        let errs = validate_ledger(&parsed).expect_err("19th row must fail 18-row ledger check");
        assert!(
            errs.iter().any(|e| e.row == "FD-FNX" && e.field == "id"),
            "expected FD-FNX rejection, got {errs:?}"
        );
    }

    #[test]
    fn ledger_every_row_has_fallback_and_gate_or_none() {
        let lock = load_repo_lock();
        for row in &lock.foundations {
            let id = row.id.as_str();
            assert!(
                !row.fallback.trim().is_empty(),
                "row `{id}` must have non-empty fallback or explicit \"none\""
            );
            if id == "FD-FFW" || id == "FD-FSS" {
                assert!(
                    row.admission_gate.is_none(),
                    "row `{id}` must have admission_gate = \"none\""
                );
            } else {
                assert!(
                    row.admission_gate.as_str().starts_with("INT-"),
                    "row `{id}` must name an INT- admission gate, got `{}`",
                    row.admission_gate
                );
            }
        }

        // Verify the four honest notes are present and mention their key facts.
        let fnp = lock.foundation("FD-FNP").expect("FD-FNP");
        assert!(fnp.note.as_deref().is_some_and(|n| n.contains("f64") && n.contains("stride")));

        let fsym = lock.foundation("FD-FSYM").expect("FD-FSYM");
        assert!(fsym.note.as_deref().is_some_and(|n| n.contains("franken_numpy-pin") && n.contains("derive/")));

        let ft = lock.foundation("FD-FT").expect("FD-FT");
        assert!(ft.note.as_deref().is_some_and(|n| n.contains("Device::Metal") && n.contains("CPU")));

        let f3d = lock.foundation("FD-F3D").expect("FD-F3D");
        assert!(f3d.note.as_deref().is_some_and(|n| n.contains("self-contained") && n.contains("HTML")));
    }

    #[test]
    fn ledger_confinement_matches_crate_map() {
        let lock = load_repo_lock();
        validate_confinement(&lock).expect("all confined_to tokens match the four forms and CRATE_MAP");

        // Negative control 1: unknown crate name is refused.
        assert!(
            validate_confinement_token("FD-FSQL", Posture::Consume, "frob-nonexistent").is_err(),
            "unknown crate token must be refused"
        );

        // Negative control 2: `any` on FD-FT is refused (only FD-ASUP permits `any`).
        assert!(
            validate_confinement_token("FD-FT", Posture::Consume, "any").is_err(),
            "`any` on FD-FT must be refused"
        );
        assert!(
            validate_confinement_token("FD-ASUP", Posture::Consume, "any").is_ok(),
            "`any` on FD-ASUP must be admitted"
        );

        // Negative control 3: directory token on a Consume use is refused.
        assert!(
            validate_confinement_token("FD-F3D", Posture::Consume, "viewer/").is_err(),
            "directory token on Consume use must be refused"
        );
        assert!(
            validate_confinement_token("FD-F3D", Posture::Subprocess, "viewer/").is_ok(),
            "directory token on Subprocess use must be admitted"
        );

        // Negative control 4: `posture = "consume"` with empty `confined_to` is refused.
        let empty_consume = parse_bytes(
            b"toolchain = \"nightly-2026-08-31\"\n\n\
              [[foundation]]\n\
              id = \"FD-TEST\"\n\
              name = \"test\"\n\
              posture = \"consume\"\n\
              version = \"=0.1.0\"\n\
              confined_to = []\n\
              admission_gate = \"INT-TEST-01\"\n\
              fallback = \"none\"\n",
        )
        .expect("parses syntactically");
        let errs = validate_confinement(&empty_consume)
            .expect_err("consume with empty confined_to must be refused");
        assert!(errs.iter().any(|e| e.row == "FD-TEST" && e.field == "confined_to"));
    }

    #[test]
    fn ledger_critical_path_is_three() {
        let lock = load_repo_lock();
        let critical: BTreeSet<&str> = lock
            .foundations
            .iter()
            .filter(|f| f.critical_path)
            .map(|f| f.id.as_str())
            .collect();
        let expected: BTreeSet<&str> = ["FD-ASUP", "FD-FSIM", "FD-FSQL"].into_iter().collect();
        assert_eq!(critical, expected);

        // No row whose uses are [F] or [M] carries critical_path = true.
        for row in &lock.foundations {
            if row.uses.iter().all(|u| matches!(u.tier, Tier::Frontier | Tier::Moonshot)) {
                assert!(
                    !row.critical_path,
                    "Frontier/Moonshot row `{}` must not be critical_path",
                    row.id
                );
            }
        }
    }

    #[test]
    fn ledger_pattern_rows_have_no_path_or_version() {
        let lock = load_repo_lock();
        for id in PATTERN_ONLY_IDS {
            let Some(row) = lock.foundation(id) else {
                assert!(lock.foundation(id).is_some(), "missing {id}");
                continue;
            };
            assert!(
                row.path.is_none(),
                "pattern row `{id}` must not have `path`, got {:?}",
                row.path
            );
            assert!(
                row.version.is_none(),
                "pattern row `{id}` must not have `version`, got {:?}",
                row.version
            );
            assert!(
                row.uses.iter().all(|u| u.posture == Posture::Pattern),
                "row `{id}` must be pattern-only"
            );
        }
    }

    #[test]
    fn ledger_multi_use_rows() {
        let lock = load_repo_lock();

        // FD-FSQL parses to three uses: consume, pattern, oracle.
        let fsql = lock.foundation("FD-FSQL").expect("FD-FSQL");
        let fsql_postures: Vec<Posture> = fsql.uses.iter().map(|u| u.posture).collect();
        assert_eq!(
            fsql_postures,
            vec![Posture::Consume, Posture::Pattern, Posture::Oracle]
        );

        // FD-FT parses to two uses with different `first_needed` (M0 for Metal, M2 for CPU trainer).
        let ft = lock.foundation("FD-FT").expect("FD-FT");
        assert_eq!(ft.uses.len(), 2);
        assert_eq!(ft.uses[0].first_needed, Some(Milestone::M0));
        assert_eq!(ft.uses[1].first_needed, Some(Milestone::M2));
        assert_ne!(ft.uses[0].first_needed, ft.uses[1].first_needed);
    }

    #[test]
    fn ledger_rows_refuse_forbidden_edges() {
        let lock = load_repo_lock();

        // 1. Refuse `frob-dyn` -> `fs-mbd` (volatile FrankenSim outside frob-fsim).
        let fsim = lock.foundation("FD-FSIM").expect("FD-FSIM");
        assert!(
            !foundation_allows_edge(
                fsim,
                "frob-dyn",
                "fs-mbd",
                EdgeKind::Normal,
                &DepSpec::path("../frankensim/crates/fs-mbd", false),
            ),
            "frob-dyn -> fs-mbd must be refused"
        );
        // Positive contrast: bedrock-direct `fs-ga` from `frob-lie` is admitted.
        assert!(
            foundation_allows_edge(
                fsim,
                "frob-lie",
                "fs-ga",
                EdgeKind::Normal,
                &DepSpec::path("../frankensim/crates/fs-ga", false),
            ),
            "frob-lie -> fs-ga (bedrock-direct) must be admitted"
        );

        // 2. Refuse `frob-core` -> `franken-kernel` (normal edge; Pattern + dev-only Oracle).
        let fker = lock.foundation("FD-FKER").expect("FD-FKER");
        assert!(
            !foundation_allows_edge(
                fker,
                "frob-core",
                "franken-kernel",
                EdgeKind::Normal,
                &DepSpec::path("../asupersync/franken_kernel", false),
            ),
            "frob-core -> franken-kernel (normal) must be refused"
        );
        // Positive: `frob-conformance` -> `franken-kernel` as a dev-dependency is admitted.
        assert!(
            foundation_allows_edge(
                fker,
                "frob-conformance",
                "franken-kernel",
                EdgeKind::Dev,
                &DepSpec::path("../asupersync/franken_kernel", false),
            ),
            "frob-conformance -> franken-kernel (dev) must be admitted"
        );

        // 3. Refuse any core crate -> `ftx-drape` (FD-FTX is Subprocess only, never a Cargo dep).
        let ftx = lock.foundation("FD-FTX").expect("FD-FTX");
        for (crate_name, kind) in [
            ("frob-chronicle", EdgeKind::Normal),
            ("frob-conformance", EdgeKind::Dev),
            ("frob-dyn", EdgeKind::Normal),
        ] {
            assert!(
                !foundation_allows_edge(
                    ftx,
                    crate_name,
                    "ftx-drape",
                    kind,
                    &DepSpec::registry("=0.1.0", false),
                ),
                "{crate_name} -> ftx-drape ({kind}) must be refused"
            );
        }

        // 4. Refuse any core crate -> `f3d-runtime` (FD-F3D is Pattern + Subprocess only).
        let f3d = lock.foundation("FD-F3D").expect("FD-F3D");
        for (crate_name, kind) in [
            ("frob-scene", EdgeKind::Normal),
            ("frob-conformance", EdgeKind::Dev),
        ] {
            assert!(
                !foundation_allows_edge(
                    f3d,
                    crate_name,
                    "f3d-runtime",
                    kind,
                    &DepSpec::registry("=0.1.0", false),
                ),
                "{crate_name} -> f3d-runtime ({kind}) must be refused"
            );
        }

        // 5. Admit `frob-chronicle` -> `fsqlite` (FD-FSQL) with exact requirement and default-features off.
        let fsql = lock.foundation("FD-FSQL").expect("FD-FSQL");
        assert!(
            foundation_allows_edge(
                fsql,
                "frob-chronicle",
                "fsqlite",
                EdgeKind::Normal,
                &DepSpec::registry("=0.5.1", false),
            ),
            "frob-chronicle -> fsqlite (=0.5.1, default-features=false) must be admitted"
        );
    }

    #[test]
    fn ledger_first_needed_complete() {
        let lock = load_repo_lock();
        let expected_milestones = [
            ("FD-FSIM", Milestone::M0),
            ("FD-ASUP", Milestone::M0),
            ("FD-FSQL", Milestone::M0),
            ("FD-FT", Milestone::M0),
            ("FD-CUDA", Milestone::M0),
            ("FD-FNP", Milestone::M0),
            ("FD-FSCI", Milestone::M1),
            ("FD-FSYM", Milestone::M0),
            ("FD-FLN", Milestone::M5),
            ("FD-FMCP", Milestone::M0),
            ("FD-FTUI", Milestone::M3),
            ("FD-F3D", Milestone::M3),
            ("FD-FDGR", Milestone::M3),
            ("FD-FJAX", Milestone::M2),
            ("FD-FKER", Milestone::M1),
            ("FD-FTX", Milestone::M6),
            ("FD-FFW", Milestone::M8),
            ("FD-FSS", Milestone::M5),
        ];

        assert_eq!(expected_milestones.len(), 18);
        for (id, expected_m) in expected_milestones {
            let Some(row) = lock.foundation(id) else {
                assert!(lock.foundation(id).is_some(), "missing {id}");
                continue;
            };
            assert_eq!(
                row.first_needed,
                Some(expected_m),
                "row `{id}` expected first_needed = {expected_m}, got {:?}",
                row.first_needed
            );
        }
    }

    #[test]
    fn ledger_row_permutation_and_canonical_stability() {
        let raw = std::fs::read_to_string(repo_lock_path()).expect("read franken-stack.lock");
        let parsed1 = parse_bytes(raw.as_bytes()).expect("parse 1");
        let parsed2 = parse_bytes(raw.as_bytes()).expect("parse 2");
        assert_eq!(parsed1, parsed2);
        assert_eq!(parsed1.canonical_bytes(), parsed2.canonical_bytes());
        assert_eq!(parsed1.digest(), parsed2.digest());

        // Split on `[[foundation]]` blocks, reverse the 18 rows, and re-parse (metamorphic relation).
        let parts: Vec<&str> = raw.split("\n[[foundation]]\n").collect();
        assert_eq!(parts.len(), 19, "header + 18 [[foundation]] blocks");
        let header = parts[0];
        let mut permuted = String::from(header);
        for block in parts[1..].iter().rev() {
            permuted.push_str("\n[[foundation]]\n");
            permuted.push_str(block);
        }

        let parsed_perm = parse_bytes(permuted.as_bytes()).expect("permuted lock parses");
        assert_eq!(parsed_perm, parsed1);
        assert_eq!(parsed_perm.canonical_bytes(), parsed1.canonical_bytes());
        assert_eq!(parsed_perm.digest(), parsed1.digest());
        validate_ledger(&parsed_perm).expect("permuted lock passes ledger validation");
    }
}
