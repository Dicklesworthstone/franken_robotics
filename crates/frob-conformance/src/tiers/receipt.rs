#![deny(unsafe_code)]

//! Canonical hash-named Gauntlet receipts (`frob.gauntlet.receipt/1`, `N38a-02`).
//!
//! Serializes exclusively through `frob_core::canon` (`N34-18`), parses strictly
//! through `frob_core::json::admit` (`N34-17`), and content-addresses every
//! receipt by the BLAKE3 digest of its canonical bytes (`frob_core::id::hash_blob`).
//!
//! Every receipt embeds `stack_receipt` (`sr:` form of `BUILT_UNDER.receipt`),
//! `stack_dev` (`BUILT_UNDER.dev`), and `build_identity` (`BUILD_IDENTITY_MODE`:
//! `"dev-salt"` or `"depgraph-receipt"`). The reader rejects any missing,
//! malformed, or tampered field.

use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, File};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use frob_core::canon::{self, CanonError, Canonical, CanonicalBytes, Value};
use frob_core::evidence::DetClass;
use frob_core::id::hash_blob;
use frob_core::json::{self, Json, Limits};
use frob_core::stack::StackReceiptHash;
use frob_core::stack::build_identity::BuildIdentityMode;

use super::{ControlRunOutcome, ReducedCount, Site, Tier};

/// Schema identifier for Gauntlet tier receipts.
pub const GAUNTLET_RECEIPT_SCHEMA: &str = "frob.gauntlet.receipt/1";

/// Maximum exact JSON integer representable in `frob_core::canon::int53`.
const INT53_MAX_U64: u64 = (1u64 << 53) - 1;

/// Quantitative measurement recorded on one receipt entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptMeasured {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub threshold: String,
}

impl Canonical for ReceiptMeasured {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("name", &self.name)?;
            o.str("threshold", &self.threshold)?;
            o.str("unit", &self.unit)?;
            o.str("value", &self.value)?;
            Ok(())
        })
    }
}

impl Canonical for ReducedCount {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        let count = u64_to_int53(self.count)?;
        let target = u64_to_int53(self.target)?;
        w.object(|o| {
            o.int53("count", count)?;
            o.int53("target", target)?;
            Ok(())
        })
    }
}

/// One entry execution record inside `frob.gauntlet.receipt/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptEntry {
    pub id: String,
    pub site: Site,
    pub outcome: String,
    pub measured: Option<ReceiptMeasured>,
    pub first_diff: Option<String>,
    pub determinism: Option<DetClass>,
    pub synthetic: bool,
    /// Present ONLY when run below `target_count`; omitted on full-count runs.
    pub reduced: Option<ReducedCount>,
}

impl Canonical for ReceiptEntry {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            if let Some(det) = self.determinism {
                o.str("determinism", det.as_str())?;
            }
            if let Some(ref fd) = self.first_diff {
                o.str("first_diff", fd)?;
            }
            o.str("id", &self.id)?;
            if let Some(ref m) = self.measured {
                o.member_term("measured", m)?;
            }
            o.str("outcome", &self.outcome)?;
            if let Some(red) = self.reduced {
                o.member_term("reduced", &red)?;
            }
            o.str("site", self.site.as_str())?;
            o.bool("synthetic", self.synthetic)?;
            Ok(())
        })
    }
}

/// One same-run negative-control execution record inside `frob.gauntlet.receipt/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptControl {
    pub id: String,
    pub gate: String,
    pub outcome: ControlRunOutcome,
}

impl Canonical for ReceiptControl {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("gate", &self.gate)?;
            o.str("id", &self.id)?;
            o.str("outcome", self.outcome.as_str())?;
            Ok(())
        })
    }
}

/// One gate verdict line inside `frob.gauntlet.receipt/1`.
///
/// `line` is strictly one of `"pass"`, `"fail"`, `"not-eligible"`, or
/// `"no-claim: <reason>"`. `reduced` is omitted unless `Some(true)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptGate {
    pub id: String,
    pub line: String,
    /// Present ONLY as `true` when any entry for this gate ran at a reduced count.
    pub reduced: Option<bool>,
}

impl Canonical for ReceiptGate {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("id", &self.id)?;
            o.str("line", &self.line)?;
            if let Some(true) = self.reduced {
                o.bool("reduced", true)?;
            }
            Ok(())
        })
    }
}

/// Summary counts over the entries executed in a tier run.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReceiptCounts {
    pub pass: u64,
    pub fail: u64,
    pub refused: u64,
    pub skipped: u64,
    pub obligations: u64,
}

impl Canonical for ReceiptCounts {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        let fail = u64_to_int53(self.fail)?;
        let obligations = u64_to_int53(self.obligations)?;
        let pass = u64_to_int53(self.pass)?;
        let refused = u64_to_int53(self.refused)?;
        let skipped = u64_to_int53(self.skipped)?;
        w.object(|o| {
            o.int53("fail", fail)?;
            o.int53("obligations", obligations)?;
            o.int53("pass", pass)?;
            o.int53("refused", refused)?;
            o.int53("skipped", skipped)?;
            Ok(())
        })
    }
}

/// Complete typed `frob.gauntlet.receipt/1` tier receipt (`N38a-02`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GauntletReceipt {
    pub schema: String,
    pub tier: Tier,
    pub filter: String,
    pub commit: String,
    pub dirty: bool,
    pub toolchain: String,
    pub fingerprint: String,
    pub seed: u64,
    pub profile: String,
    pub started: String,
    pub wall_ms: u64,
    pub entries: Vec<ReceiptEntry>,
    pub controls: Vec<ReceiptControl>,
    pub gates: Vec<ReceiptGate>,
    pub counts: ReceiptCounts,
    pub session: String,
    pub stack_receipt: String,
    pub stack_dev: bool,
    pub build_identity: String,
}

impl Canonical for GauntletReceipt {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        let seed = u64_to_int53(self.seed)?;
        let wall_ms = u64_to_int53(self.wall_ms)?;
        w.object(|o| {
            o.str("build_identity", &self.build_identity)?;
            o.str("commit", &self.commit)?;
            o.array("controls", |a| {
                for c in &self.controls {
                    a.push_term(c)?;
                }
                Ok(())
            })?;
            o.member_term("counts", &self.counts)?;
            o.bool("dirty", self.dirty)?;
            o.array("entries", |a| {
                for e in &self.entries {
                    a.push_term(e)?;
                }
                Ok(())
            })?;
            o.str("filter", &self.filter)?;
            o.str("fingerprint", &self.fingerprint)?;
            o.array("gates", |a| {
                for g in &self.gates {
                    a.push_term(g)?;
                }
                Ok(())
            })?;
            o.str("profile", &self.profile)?;
            o.str("schema", &self.schema)?;
            o.int53("seed", seed)?;
            o.str("session", &self.session)?;
            o.bool("stack_dev", self.stack_dev)?;
            o.str("stack_receipt", &self.stack_receipt)?;
            o.str("started", &self.started)?;
            o.str("tier", self.tier.code())?;
            o.str("toolchain", &self.toolchain)?;
            o.int53("wall_ms", wall_ms)?;
            Ok(())
        })
    }
}

impl GauntletReceipt {
    /// Serialize this receipt to canonical JSON bytes via `frob_core::canon`.
    pub fn to_canonical_bytes(&self) -> Result<CanonicalBytes, ReceiptError> {
        validate_receipt_invariants(self)?;
        canon::to_canonical(self).map_err(|e| ReceiptError::Canon(e.to_string()))
    }

    /// Lowercase 64-hex BLAKE3 digest of the canonical receipt bytes.
    pub fn digest_hex(&self) -> Result<String, ReceiptError> {
        let bytes = self.to_canonical_bytes()?;
        Ok(hash_blob(bytes.as_bytes()).to_hex())
    }

    /// Return a copy with run-ephemeral fields (`started`, `wall_ms`, `session`)
    /// normalized so two runs of the same tier/filter/seed/commit can be compared
    /// byte-for-byte (`N38a-02` determinism lane).
    #[must_use]
    pub fn without_run_ephemera(&self) -> Self {
        let mut copy = self.clone();
        copy.started.clear();
        copy.wall_ms = 0;
        copy.session.clear();
        copy
    }

    /// Parse and strictly validate canonical JSON bytes into a [`GauntletReceipt`].
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, ReceiptError> {
        let admitted = json::admit(bytes, &Limits::default())
            .map_err(|e| ReceiptError::Json(e.to_string()))?;
        let receipt = decode_receipt_json(admitted.value())?;
        let _ = admitted
            .into_canonical()
            .map_err(|e| ReceiptError::Canon(e.to_string()))?;
        validate_receipt_invariants(&receipt)?;
        let reencoded = receipt.to_canonical_bytes()?;
        if reencoded.as_bytes() != bytes {
            return Err(ReceiptError::NonCanonical(
                "re-serialized canonical receipt bytes differ from input".to_owned(),
            ));
        }
        Ok(receipt)
    }
}

/// Structured error returned when writing, reading, or verifying a Gauntlet receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptError {
    Io(String),
    Json(String),
    Canon(String),
    NonCanonical(String),
    HashMismatch { expected: String, actual: String },
    InvalidFilename(String),
    InvalidField { field: String, detail: String },
}

impl fmt::Display for ReceiptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(m) => write!(f, "receipt I/O error: {m}"),
            Self::Json(m) => write!(f, "receipt JSON error: {m}"),
            Self::Canon(m) => write!(f, "receipt canonicalization error: {m}"),
            Self::NonCanonical(m) => write!(f, "non-canonical receipt: {m}"),
            Self::HashMismatch { expected, actual } => write!(
                f,
                "reason=receipt-hash-mismatch: receipt BLAKE3 `{actual}` does not match filename `{expected}`"
            ),
            Self::InvalidFilename(m) => write!(f, "invalid receipt filename: {m}"),
            Self::InvalidField { field, detail } => {
                write!(f, "invalid receipt field `{field}`: {detail}")
            }
        }
    }
}

impl std::error::Error for ReceiptError {}

/// Write `receipt` atomically as `<dir>/<blake3>.json` (write temp, sync, rename;
/// never deletes files). Returns `(path, blake3_hex)`.
pub fn write_receipt(
    dir: &Path,
    receipt: &GauntletReceipt,
) -> Result<(PathBuf, String), ReceiptError> {
    let canon = receipt.to_canonical_bytes()?;
    let hash = hash_blob(canon.as_bytes()).to_hex();
    fs::create_dir_all(dir)
        .map_err(|e| ReceiptError::Io(format!("create_dir_all `{}`: {e}", dir.display())))?;
    let dest = dir.join(format!("{hash}.json"));
    if dest.exists() {
        let existing = fs::read(&dest)
            .map_err(|e| ReceiptError::Io(format!("read existing `{}`: {e}", dest.display())))?;
        let actual = hash_blob(&existing).to_hex();
        if existing == canon.as_bytes() && actual == hash {
            return Ok((dest, hash));
        }
        return Err(ReceiptError::HashMismatch {
            expected: hash,
            actual,
        });
    }

    static TMP_SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp = dir.join(format!(
        ".gauntlet-receipt-{}-{seq}.tmp",
        std::process::id()
    ));
    let mut f = File::create(&tmp)
        .map_err(|e| ReceiptError::Io(format!("create temp `{}`: {e}", tmp.display())))?;
    f.write_all(canon.as_bytes())
        .and_then(|()| f.sync_all())
        .map_err(|e| ReceiptError::Io(format!("write/sync temp `{}`: {e}", tmp.display())))?;
    drop(f);
    fs::rename(&tmp, &dest).map_err(|e| {
        ReceiptError::Io(format!(
            "rename `{}` -> `{}`: {e}",
            tmp.display(),
            dest.display()
        ))
    })?;
    Ok((dest, hash))
}

/// Read `<64-hex>.json`, verify that its BLAKE3 hash equals its filename stem,
/// verify strict canonicality, and validate all required fields (including
/// `stack_receipt`, `stack_dev`, and `build_identity`).
pub fn read_and_verify_receipt(path: &Path) -> Result<(GauntletReceipt, String), ReceiptError> {
    let fname = path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| ReceiptError::InvalidFilename("non-UTF-8 receipt filename".to_owned()))?;
    let stem = fname.strip_suffix(".json").ok_or_else(|| {
        ReceiptError::InvalidFilename(format!("receipt filename `{fname}` must end with `.json`"))
    })?;
    if !is_lower_hex_64(stem) {
        return Err(ReceiptError::InvalidFilename(format!(
            "receipt filename stem `{stem}` must be 64 lower-case hex digits"
        )));
    }
    let bytes =
        fs::read(path).map_err(|e| ReceiptError::Io(format!("read `{}`: {e}", path.display())))?;
    let actual = hash_blob(&bytes).to_hex();
    if actual != stem {
        return Err(ReceiptError::HashMismatch {
            expected: stem.to_owned(),
            actual,
        });
    }
    let receipt = GauntletReceipt::from_canonical_bytes(&bytes)?;
    Ok((receipt, actual))
}

/// Render a human-readable summary for stdout, printing each gate's line and
/// `reduced: <count> of <target>` whenever an entry ran at a reduced count.
#[must_use]
pub fn format_text_summary(receipt: &GauntletReceipt) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "tier={} filter={} seed={} stack_receipt={} stack_dev={} build_identity={}\n",
        receipt.tier.code(),
        receipt.filter,
        receipt.seed,
        receipt.stack_receipt,
        receipt.stack_dev,
        receipt.build_identity,
    ));
    for e in &receipt.entries {
        out.push_str(&format!(
            "  entry {} [{}]: {}",
            e.id,
            e.site.as_str(),
            e.outcome
        ));
        if let Some(red) = e.reduced {
            out.push_str(&format!(" (reduced: {} of {})", red.count, red.target));
        }
        if let Some(ref diff) = e.first_diff {
            out.push_str(&format!(" diff={diff}"));
        }
        out.push('\n');
    }
    for c in &receipt.controls {
        out.push_str(&format!(
            "  control {} ({}): {}\n",
            c.id,
            c.gate,
            c.outcome.as_str()
        ));
    }
    for g in &receipt.gates {
        out.push_str(&format!("  gate {}: {}", g.id, g.line));
        if g.reduced == Some(true) {
            if let Some(red) = receipt.entries.iter().find_map(|e| e.reduced) {
                out.push_str(&format!(" [reduced: {} of {}]", red.count, red.target));
            } else {
                out.push_str(" [reduced]");
            }
        }
        out.push('\n');
    }
    out.push_str(&format!(
        "counts: pass={} fail={} refused={} skipped={} obligations={}\n",
        receipt.counts.pass,
        receipt.counts.fail,
        receipt.counts.refused,
        receipt.counts.skipped,
        receipt.counts.obligations,
    ));
    out
}

fn validate_receipt_invariants(r: &GauntletReceipt) -> Result<(), ReceiptError> {
    if r.schema != GAUNTLET_RECEIPT_SCHEMA {
        return Err(ReceiptError::InvalidField {
            field: "schema".to_owned(),
            detail: format!("expected `{GAUNTLET_RECEIPT_SCHEMA}`, got `{}`", r.schema),
        });
    }
    StackReceiptHash::parse(&r.stack_receipt).map_err(|e| ReceiptError::InvalidField {
        field: "stack_receipt".to_owned(),
        detail: format!("must be `sr:<64 lower-case hex>`: {e}"),
    })?;
    if BuildIdentityMode::parse(&r.build_identity).is_none() {
        return Err(ReceiptError::InvalidField {
            field: "build_identity".to_owned(),
            detail: format!(
                "must be `dev-salt` or `depgraph-receipt`, got `{}`",
                r.build_identity
            ),
        });
    }
    for e in &r.entries {
        if !matches!(
            e.outcome.as_str(),
            "pass" | "fail" | "refused" | "skipped" | "no-claim" | "obligation"
        ) {
            return Err(ReceiptError::InvalidField {
                field: format!("entries[{}].outcome", e.id),
                detail: format!("unknown entry outcome `{}`", e.outcome),
            });
        }
        if let Some(red) = e.reduced
            && red.count >= red.target
        {
            return Err(ReceiptError::InvalidField {
                field: format!("entries[{}].reduced", e.id),
                detail: format!(
                    "reduced count {} must be < target {}",
                    red.count, red.target
                ),
            });
        }
    }
    for g in &r.gates {
        let valid_line = matches!(g.line.as_str(), "pass" | "fail" | "not-eligible")
            || g.line
                .strip_prefix("no-claim: ")
                .is_some_and(|rest| !rest.trim().is_empty());
        if !valid_line {
            return Err(ReceiptError::InvalidField {
                field: format!("gates[{}].line", g.id),
                detail: format!("invalid gate line `{}`", g.line),
            });
        }
        if g.reduced == Some(false) {
            return Err(ReceiptError::InvalidField {
                field: format!("gates[{}].reduced", g.id),
                detail: "reduced must be omitted when false".to_owned(),
            });
        }
    }
    Ok(())
}

fn decode_receipt_json(val: &Json<'_>) -> Result<GauntletReceipt, ReceiptError> {
    let obj = val.as_object().ok_or_else(|| ReceiptError::InvalidField {
        field: "<root>".to_owned(),
        detail: "receipt root must be a JSON object".to_owned(),
    })?;
    const TOP_KEYS: &[&str] = &[
        "build_identity",
        "commit",
        "controls",
        "counts",
        "dirty",
        "entries",
        "filter",
        "fingerprint",
        "gates",
        "profile",
        "schema",
        "seed",
        "session",
        "stack_dev",
        "stack_receipt",
        "started",
        "tier",
        "toolchain",
        "wall_ms",
    ];
    for m in obj {
        if !TOP_KEYS.contains(&m.name.as_str()) {
            return Err(ReceiptError::InvalidField {
                field: m.name.clone(),
                detail: "unknown top-level receipt field".to_owned(),
            });
        }
    }

    let schema = req_str(val, "schema")?;
    let tier_raw = req_str(val, "tier")?;
    let tier = Tier::parse(&tier_raw).ok_or_else(|| ReceiptError::InvalidField {
        field: "tier".to_owned(),
        detail: format!("unknown tier `{tier_raw}`"),
    })?;
    let filter = req_str(val, "filter")?;
    let commit = req_str(val, "commit")?;
    let dirty = req_bool(val, "dirty")?;
    let toolchain = req_str(val, "toolchain")?;
    let fingerprint = req_str(val, "fingerprint")?;
    let seed = req_u64(val, "seed")?;
    let profile = req_str(val, "profile")?;
    let started = req_str(val, "started")?;
    let wall_ms = req_u64(val, "wall_ms")?;
    let session = req_str(val, "session")?;
    let stack_receipt = req_str(val, "stack_receipt")?;
    let stack_dev = req_bool(val, "stack_dev")?;
    let build_identity = req_str(val, "build_identity")?;

    let counts_val = val
        .get("counts")
        .ok_or_else(|| ReceiptError::InvalidField {
            field: "counts".to_owned(),
            detail: "missing required field".to_owned(),
        })?;
    check_allowed_keys(
        counts_val,
        "counts",
        &["fail", "obligations", "pass", "refused", "skipped"],
    )?;
    let counts = ReceiptCounts {
        pass: req_u64(counts_val, "pass")?,
        fail: req_u64(counts_val, "fail")?,
        refused: req_u64(counts_val, "refused")?,
        skipped: req_u64(counts_val, "skipped")?,
        obligations: req_u64(counts_val, "obligations")?,
    };

    let entries_arr = val
        .get("entries")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ReceiptError::InvalidField {
            field: "entries".to_owned(),
            detail: "missing or non-array field".to_owned(),
        })?;
    let mut entries = Vec::with_capacity(entries_arr.len());
    for (idx, ev) in entries_arr.iter().enumerate() {
        let label = format!("entries[{idx}]");
        check_allowed_keys(
            ev,
            &label,
            &[
                "determinism",
                "first_diff",
                "id",
                "measured",
                "outcome",
                "reduced",
                "site",
                "synthetic",
            ],
        )?;
        let id = req_str(ev, "id")?;
        let site_raw = req_str(ev, "site")?;
        let site = Site::parse(&site_raw).ok_or_else(|| ReceiptError::InvalidField {
            field: format!("{label}.site"),
            detail: format!("unknown site `{site_raw}`"),
        })?;
        let outcome = req_str(ev, "outcome")?;
        let synthetic = req_bool(ev, "synthetic")?;
        let first_diff = opt_str(ev, "first_diff")?;
        let determinism = match opt_str(ev, "determinism")? {
            Some(d) => Some(
                DetClass::parse(&d).ok_or_else(|| ReceiptError::InvalidField {
                    field: format!("{label}.determinism"),
                    detail: format!("unknown determinism class `{d}`"),
                })?,
            ),
            None => None,
        };
        let measured = match ev.get("measured") {
            Some(mv) => {
                check_allowed_keys(
                    mv,
                    &format!("{label}.measured"),
                    &["name", "threshold", "unit", "value"],
                )?;
                Some(ReceiptMeasured {
                    name: req_str(mv, "name")?,
                    value: req_str(mv, "value")?,
                    unit: req_str(mv, "unit")?,
                    threshold: req_str(mv, "threshold")?,
                })
            }
            None => None,
        };
        let reduced = match ev.get("reduced") {
            Some(rv) => {
                check_allowed_keys(rv, &format!("{label}.reduced"), &["count", "target"])?;
                Some(ReducedCount {
                    count: req_u64(rv, "count")?,
                    target: req_u64(rv, "target")?,
                })
            }
            None => None,
        };
        entries.push(ReceiptEntry {
            id,
            site,
            outcome,
            measured,
            first_diff,
            determinism,
            synthetic,
            reduced,
        });
    }

    let controls_arr = val
        .get("controls")
        .and_then(|v| v.as_array())
        .ok_or_else(|| ReceiptError::InvalidField {
            field: "controls".to_owned(),
            detail: "missing or non-array field".to_owned(),
        })?;
    let mut controls = Vec::with_capacity(controls_arr.len());
    for (idx, cv) in controls_arr.iter().enumerate() {
        let label = format!("controls[{idx}]");
        check_allowed_keys(cv, &label, &["gate", "id", "outcome"])?;
        let id = req_str(cv, "id")?;
        let gate = req_str(cv, "gate")?;
        let out_raw = req_str(cv, "outcome")?;
        let outcome =
            ControlRunOutcome::parse(&out_raw).ok_or_else(|| ReceiptError::InvalidField {
                field: format!("{label}.outcome"),
                detail: format!("unknown control outcome `{out_raw}`"),
            })?;
        controls.push(ReceiptControl { id, gate, outcome });
    }

    let gates_arr =
        val.get("gates")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ReceiptError::InvalidField {
                field: "gates".to_owned(),
                detail: "missing or non-array field".to_owned(),
            })?;
    let mut gates = Vec::with_capacity(gates_arr.len());
    for (idx, gv) in gates_arr.iter().enumerate() {
        let label = format!("gates[{idx}]");
        check_allowed_keys(gv, &label, &["id", "line", "reduced"])?;
        let id = req_str(gv, "id")?;
        let line = req_str(gv, "line")?;
        let reduced = match gv.get("reduced") {
            Some(rv) => {
                let b = rv.as_bool().ok_or_else(|| ReceiptError::InvalidField {
                    field: format!("{label}.reduced"),
                    detail: "must be boolean".to_owned(),
                })?;
                Some(b)
            }
            None => None,
        };
        gates.push(ReceiptGate { id, line, reduced });
    }

    Ok(GauntletReceipt {
        schema,
        tier,
        filter,
        commit,
        dirty,
        toolchain,
        fingerprint,
        seed,
        profile,
        started,
        wall_ms,
        entries,
        controls,
        gates,
        counts,
        session,
        stack_receipt,
        stack_dev,
        build_identity,
    })
}

fn check_allowed_keys(val: &Json<'_>, parent: &str, allowed: &[&str]) -> Result<(), ReceiptError> {
    let obj = val.as_object().ok_or_else(|| ReceiptError::InvalidField {
        field: parent.to_owned(),
        detail: "expected JSON object".to_owned(),
    })?;
    for m in obj {
        if !allowed.contains(&m.name.as_str()) {
            return Err(ReceiptError::InvalidField {
                field: format!("{parent}.{}", m.name),
                detail: "unknown field".to_owned(),
            });
        }
    }
    Ok(())
}

fn req_str(val: &Json<'_>, key: &str) -> Result<String, ReceiptError> {
    val.get(key)
        .and_then(|v| v.as_str())
        .map(str::to_owned)
        .ok_or_else(|| ReceiptError::InvalidField {
            field: key.to_owned(),
            detail: "missing or non-string required field".to_owned(),
        })
}

fn opt_str(val: &Json<'_>, key: &str) -> Result<Option<String>, ReceiptError> {
    match val.get(key) {
        None => Ok(None),
        Some(v) => {
            v.as_str()
                .map(|s| Some(s.to_owned()))
                .ok_or_else(|| ReceiptError::InvalidField {
                    field: key.to_owned(),
                    detail: "must be string when present".to_owned(),
                })
        }
    }
}

fn req_bool(val: &Json<'_>, key: &str) -> Result<bool, ReceiptError> {
    val.get(key)
        .and_then(|v| v.as_bool())
        .ok_or_else(|| ReceiptError::InvalidField {
            field: key.to_owned(),
            detail: "missing or non-boolean required field".to_owned(),
        })
}

fn req_u64(val: &Json<'_>, key: &str) -> Result<u64, ReceiptError> {
    let v = val.get(key).ok_or_else(|| ReceiptError::InvalidField {
        field: key.to_owned(),
        detail: "missing required integer field".to_owned(),
    })?;
    let i = v.as_int53().map_err(|e| ReceiptError::InvalidField {
        field: key.to_owned(),
        detail: format!("invalid integer: {e}"),
    })?;
    u64::try_from(i).map_err(|_| ReceiptError::InvalidField {
        field: key.to_owned(),
        detail: format!("expected non-negative integer, got {i}"),
    })
}

fn u64_to_int53(v: u64) -> Result<i64, CanonError> {
    if v > INT53_MAX_U64 {
        return Err(CanonError::Int53OutOfRange { value: i64::MAX });
    }
    Ok(v as i64)
}

fn is_lower_hex_64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}
