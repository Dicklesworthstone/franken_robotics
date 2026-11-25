//! Bundle schema `frob.cert/1` (`FMT-FROBCERT`), format rules R1–R5, generic
//! rejection rules 1–6, and Merkle-root computation for `frob-cert` (N12-03).
//!
//! Strictly `std`-only, `#![forbid(unsafe_code)]`, zero floating-point types,
//! and shares no source code with any proposer crate.

use std::collections::BTreeSet;
use std::fmt;

use crate::blake3;
use crate::json::{JsonValue, parse_canonical, parse_non_canonical, serialize_canonical};
use crate::kinds::registry::{
    CheckVerdict, Class, KindRow, KindStatus, Scope, check_class_cap, dispatch, lookup,
    lookup_version, scope_for_claim_kind,
};
use crate::kinds::rules::RuleId;
use crate::limits::{COMPILED_LIMITS, Limits};
use crate::rational::Rational;

/// Normative bundle schema version string (`FMT-FROBCERT`, N12-03).
pub const BUNDLE_SCHEMA_VERSION: &str = "frob.cert/1";

/// Allowed top-level keys of a `frob.cert/1` bundle (closed set; strictly sorted).
const TOP_LEVEL_KEYS: &[&str] = &[
    "assumptions",
    "authorization",
    "checker",
    "claim",
    "definitions",
    "domain",
    "expected_verdict",
    "lineage",
    "plant",
    "proof",
    "query",
    "review",
    "schema",
];

/// Required top-level keys checked in Rule 1 (`authorization` is checked in Rule 2
/// so absence reports `flags.missing`; `review` is checked per `claim.kind` so
/// absence on mission claims reports `schema.review-missing`).
const REQUIRED_RULE1_KEYS: &[&str] = &[
    "assumptions",
    "checker",
    "claim",
    "definitions",
    "domain",
    "expected_verdict",
    "lineage",
    "plant",
    "proof",
    "query",
];

/// Exact three no-authorization flags (`INV-LEMMA-03`, `FROB-REF-833`).
pub const AUTHORIZATION_FLAGS: &[&str] = &[
    "execution_authorized",
    "hardware_safety_signoff",
    "sim_to_real_certified",
];

/// Mission claim kinds that require the `review` marker (OPEN-AUGUR-8).
pub const MISSION_CLAIM_KINDS: &[&str] = &[
    "family-inclusion",
    "family-exclusion",
    "partition",
];

/// Non-SI unit strings rejected when `definitions.units == "SI"`.
const NON_SI_UNITS: &[&str] = &[
    "in", "ft", "yd", "mi", "lbf", "lb", "oz", "ft*lbf", "ft-lbf", "in*lbf", "psi", "deg",
];

/// Structured rejection produced by schema or generic rule validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleRejection {
    pub rule: RuleId,
    pub refusal: Option<&'static str>,
    pub path: String,
    pub message: String,
}

impl RuleRejection {
    pub fn new(rule: RuleId, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            rule,
            refusal: rule.refusal_code(),
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for RuleRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.refusal {
            write!(f, "{} ({}) at {}: {}", self.rule, code, self.path, self.message)
        } else {
            write!(f, "{} at {}: {}", self.rule, self.path, self.message)
        }
    }
}

impl std::error::Error for RuleRejection {}

/// The three mandatory no-authorization flags (Rust field `flags`, JSON key `authorization`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoAuthorization {
    pub execution_authorized: bool,
    pub hardware_safety_signoff: bool,
    pub sim_to_real_certified: bool,
}

/// Optional (required on mission claims) OPEN-AUGUR-8 review marker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewMarker {
    pub equations_reviewed: bool,
    pub equations_digest: Option<String>,
    pub review_record: Option<String>,
    pub scope: Option<String>,
}

/// Format rule R3: attained minimum with matching lower bound, or open threshold with candidate + gap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostBounds {
    pub attained: bool,
    pub strict: bool,
    pub unit: String,
    pub incumbent: Option<Rational>,
    pub lower_bound: Option<Rational>,
    pub bound: Option<Rational>,
    pub candidate: Option<Rational>,
    pub gap: Option<Rational>,
}

/// Format rule R4: unresolved probability mass under a caller-supplied weighting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedMass {
    pub weighting: JsonValue,
    pub value: JsonValue,
}

/// Claim statement block (field 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimStatement {
    pub kind: String,
    pub class: Class,
    pub quantifiers: Vec<String>,
    pub horizon: String,
    pub cost_bounds: Option<CostBounds>,
    pub unresolved_mass: Option<UnresolvedMass>,
}

/// Embedded query digest and canonical bytes (field 3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedQuery {
    pub blake3: String,
    pub bytes: String,
}

/// Units, coordinate frame, clocks, and ordered ports (field 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definitions {
    pub units: String,
    pub frame: String,
    pub clocks: Vec<JsonValue>,
    pub ports_ordered: Vec<String>,
}

/// Assumption polarity under which a certificate survives edits (field 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Polarity {
    Grow,
    Shrink,
}

impl Polarity {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "grow" => Some(Self::Grow),
            "shrink" => Some(Self::Shrink),
            _ => None,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Grow => "grow",
            Self::Shrink => "shrink",
        }
    }
}

/// Single assumption record (field 5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assumption {
    pub id: String,
    pub value: Option<JsonValue>,
    pub blake3: Option<String>,
    pub survives: Polarity,
    pub class: Option<Class>,
}

/// Model identity and policy-class digest (field 6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlantRef {
    pub model: String,
    pub policy_class: String,
    pub model_class: Option<Class>,
    pub input_classes: Vec<(String, Class)>,
}

/// Multi-coordinate rational box cell with optional end-closure markers (field 7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainCell {
    pub lo: Vec<Rational>,
    pub hi: Vec<Rational>,
    pub closed: Option<Vec<(bool, bool)>>,
}

/// Claimed domain chart and cell (field 7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Domain {
    pub chart: String,
    pub cell: DomainCell,
}

/// Proof block (field 8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleProof {
    pub kind: String,
    pub version: u32,
    pub primitive_equations: Vec<String>,
    pub payload: JsonValue,
    pub side_conditions: Vec<JsonValue>,
    pub required_side_conditions: Vec<String>,
}

/// Target checker reference (field 9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckerRef {
    pub name: String,
    pub build: String,
}

/// Expected verdict recorded in the bundle (field 10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedVerdict {
    Accept,
    Reject,
}

/// Lineage block carrying set Merkle root and qualification `alpha_spend` (field 11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lineage {
    pub merkle_root: String,
    pub alpha_spend: Vec<JsonValue>,
}

/// Validated `.frobcert` (`frob.cert/1`) bundle (Appendix I.4, N12-03).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bundle {
    pub schema: String,
    pub claim: ClaimStatement,
    pub query: EmbeddedQuery,
    pub definitions: Definitions,
    pub assumptions: Vec<Assumption>,
    pub plant: PlantRef,
    pub domain: Domain,
    pub proof: BundleProof,
    pub checker: CheckerRef,
    pub expected_verdict: ExpectedVerdict,
    pub lineage: Lineage,
    pub review: Option<ReviewMarker>,
    pub flags: NoAuthorization,
}

/// Outcome of passing schema and all six generic rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedBundle {
    pub bundle: Bundle,
    /// Matched proof-kind row (only `None` under the `shortcut-accept-on-hashes` negative control).
    pub kind_row: Option<&'static KindRow>,
    pub scope: Option<Scope>,
}

/// Recomputed Merkle tree details for a set of one or more `frob.cert/1` bundles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleTreeSummary {
    pub root: [u8; 32],
    pub root_hex: String,
    pub level_widths: Vec<usize>,
    pub leaf_digests_sorted: Vec<[u8; 32]>,
}

/// Result of auditing a set of one or more `frob.cert/1` bundles together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetAuditSummary {
    pub merkle_root: Option<String>,
    pub merkle_recomputed: bool,
    pub level_widths: Vec<usize>,
    pub class: Option<Class>,
    pub weakest_entry: Option<usize>,
    pub rejection: Option<RuleRejection>,
}

/// Returns `true` iff `s` is exactly 64 lowercase ASCII hexadecimal characters.
pub fn is_blake3_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// Compute the Merkle leaf digest of a parsed `frob.cert/1` JSON object:
/// BLAKE3 of the canonical JSON bytes with `lineage.merkle_root` set to `""`.
pub fn bundle_leaf_digest(val: &JsonValue) -> Result<[u8; 32], RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "$", "bundle root must be an object")
    })?;
    let mut cloned = Vec::with_capacity(entries.len());
    let mut saw_lineage = false;
    for (k, v) in entries {
        if k == "lineage" {
            saw_lineage = true;
            let lin_entries = v.as_obj().ok_or_else(|| {
                RuleRejection::new(RuleId::SchemaFieldType, "lineage", "lineage must be an object")
            })?;
            let mut lin_out = Vec::with_capacity(lin_entries.len());
            let mut saw_root = false;
            for (lk, lv) in lin_entries {
                if lk == "merkle_root" {
                    saw_root = true;
                    lin_out.push((lk.clone(), JsonValue::Str(String::new())));
                } else {
                    lin_out.push((lk.clone(), lv.clone()));
                }
            }
            if !saw_root {
                return Err(RuleRejection::new(
                    RuleId::SchemaMissingField,
                    "lineage.merkle_root",
                    "lineage is missing merkle_root",
                ));
            }
            cloned.push((k.clone(), JsonValue::Obj(lin_out)));
        } else {
            cloned.push((k.clone(), v.clone()));
        }
    }
    if !saw_lineage {
        return Err(RuleRejection::new(
            RuleId::SchemaMissingField,
            "lineage",
            "bundle is missing lineage",
        ));
    }
    let canonical = serialize_canonical(&JsonValue::Obj(cloned));
    Ok(blake3::hash(canonical.as_bytes()))
}

/// Compute the Merkle leaf digest directly from canonical bundle bytes.
pub fn bundle_bytes_leaf_digest(bytes: &[u8]) -> Result<[u8; 32], RuleRejection> {
    let val = parse_canonical(bytes, &COMPILED_LIMITS).map_err(json_err_to_rejection)?;
    bundle_leaf_digest(&val)
}

/// Compute the deterministic Merkle root over a non-empty list of 32-byte leaf digests.
///
/// Rules (N12-03):
/// - Leaves are sorted in ascending 32-byte lexicographic order.
/// - Duplicate leaves are rejected with `hash.merkle-duplicate`.
/// - Inner node = `BLAKE3(0x01 || left_32 || right_32)`.
/// - An odd-length level carries its last node up unchanged.
/// - A single-leaf set's root is its sole leaf digest.
pub fn compute_merkle_root(leaves: &[[u8; 32]]) -> Result<MerkleTreeSummary, RuleRejection> {
    if leaves.is_empty() {
        return Err(RuleRejection::new(
            RuleId::SchemaMissingField,
            "lineage.merkle_root",
            "cannot compute Merkle root of an empty bundle set",
        ));
    }
    let mut sorted = leaves.to_vec();
    sorted.sort_unstable();
    for pair in sorted.windows(2) {
        if pair[0] == pair[1] {
            return Err(RuleRejection::new(
                RuleId::HashMerkleDuplicate,
                "lineage.merkle_root",
                format!(
                    "duplicate bundle leaf digest {} in set",
                    blake3::to_hex(&pair[0])
                ),
            ));
        }
    }

    let mut level = sorted.clone();
    let mut level_widths = vec![level.len()];
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i + 1 < level.len() {
            let mut buf = [0u8; 65];
            buf[0] = 0x01;
            buf[1..33].copy_from_slice(&level[i]);
            buf[33..65].copy_from_slice(&level[i + 1]);
            next.push(blake3::hash(&buf));
            i += 2;
        }
        if level.len() % 2 == 1 {
            next.push(level[level.len() - 1]);
        }
        level = next;
        level_widths.push(level.len());
    }

    let root = level[0];
    Ok(MerkleTreeSummary {
        root,
        root_hex: blake3::to_hex(&root),
        level_widths,
        leaf_digests_sorted: sorted,
    })
}

/// Compute the weakest evidence class and index of the first weakest member in a set.
pub fn set_weakest_class(classes: &[Class]) -> Option<(Class, usize)> {
    let mut best: Option<(Class, usize)> = None;
    for (idx, &cls) in classes.iter().enumerate() {
        match best {
            None => best = Some((cls, idx)),
            Some((cur, _)) if cls < cur => best = Some((cls, idx)),
            _ => {}
        }
    }
    best
}

/// Recompute the Merkle root of a set of canonical `frob.cert/1` bundle byte slices,
/// check that every member's `lineage.merkle_root` matches, and compute the set's
/// weakest class.
///
/// When `enforce_single_root` is `false` and `bundles.len() == 1`, the Merkle root
/// is NOT recomputed against `lineage.merkle_root` (matching N12-03: given one bundle
/// alone, the auditor reports that the root was not recomputed).
pub fn verify_bundle_set_bytes(
    bundles: &[&[u8]],
    enforce_single_root: bool,
) -> Result<SetAuditSummary, RuleRejection> {
    if bundles.is_empty() {
        return Err(RuleRejection::new(
            RuleId::SchemaMissingField,
            "$",
            "bundle set is empty",
        ));
    }

    let mut parsed_vals = Vec::with_capacity(bundles.len());
    let mut classes = Vec::with_capacity(bundles.len());
    let mut declared_roots = Vec::with_capacity(bundles.len());

    for (idx, bytes) in bundles.iter().enumerate() {
        let val = parse_canonical(bytes, &COMPILED_LIMITS).map_err(json_err_to_rejection)?;
        let validated = validate_bundle_value(&val, None, &COMPILED_LIMITS)?;
        classes.push(validated.bundle.claim.class);
        declared_roots.push((idx, validated.bundle.lineage.merkle_root));
        parsed_vals.push(val);
    }

    let (weak_class, weak_idx) = set_weakest_class(&classes)
        .map(|(c, i)| (Some(c), Some(i)))
        .unwrap_or((None, None));

    if bundles.len() == 1 && !enforce_single_root {
        return Ok(SetAuditSummary {
            merkle_root: Some(declared_roots[0].1.clone()),
            merkle_recomputed: false,
            level_widths: vec![1],
            class: weak_class,
            weakest_entry: weak_idx,
            rejection: None,
        });
    }

    let mut leaves = Vec::with_capacity(parsed_vals.len());
    for val in &parsed_vals {
        leaves.push(bundle_leaf_digest(val)?);
    }
    let tree = compute_merkle_root(&leaves)?;

    let mut mismatch = None;
    for (idx, decl) in &declared_roots {
        if decl != &tree.root_hex {
            mismatch = Some(RuleRejection::new(
                RuleId::HashMerkleRoot,
                format!("bundles[{idx}].lineage.merkle_root"),
                format!(
                    "declared merkle_root {decl:?} does not match recomputed root {:?}",
                    tree.root_hex
                ),
            ));
            break;
        }
    }

    Ok(SetAuditSummary {
        merkle_root: Some(tree.root_hex),
        merkle_recomputed: true,
        level_widths: tree.level_widths,
        class: weak_class,
        weakest_entry: weak_idx,
        rejection: mismatch,
    })
}

/// Parse canonical bytes and validate a `frob.cert/1` bundle through all six generic rules.
pub fn validate_bundle_bytes(
    bytes: &[u8],
    offered_query_bytes: Option<&[u8]>,
    limits: &Limits,
) -> Result<ValidatedBundle, RuleRejection> {
    let val = parse_canonical(bytes, limits).map_err(json_err_to_rejection)?;
    validate_bundle_value(&val, offered_query_bytes, limits)
}

/// Validate an already-parsed canonical JSON tree against `frob.cert/1` and all
/// six generic rejection rules in exact precedence order (N12-03).
pub fn validate_bundle_value(
    val: &JsonValue,
    offered_query_bytes: Option<&[u8]>,
    limits: &Limits,
) -> Result<ValidatedBundle, RuleRejection> {
    // =========================================================================
    // RULE 1: Structure, schema (`schema.*`), numeric format (R5), and
    //         recomputable hashes (`hash.*`)
    // =========================================================================
    let root_entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "$", "bundle root must be a JSON object")
    })?;

    for (k, _) in root_entries {
        if !TOP_LEVEL_KEYS.contains(&k.as_str()) {
            return Err(RuleRejection::new(
                RuleId::SchemaUnknownField,
                k.as_str(),
                format!("unknown top-level field {k:?} in frob.cert/1"),
            ));
        }
    }

    // 1.1 schema version string
    let schema_str = match val.get("schema") {
        None => {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                "schema",
                "missing required field 'schema'",
            ));
        }
        Some(v) => v.as_str().ok_or_else(|| {
            RuleRejection::new(RuleId::SchemaFieldType, "schema", "'schema' must be a string")
        })?,
    };
    if schema_str != BUNDLE_SCHEMA_VERSION {
        return Err(RuleRejection::new(
            RuleId::SchemaUnknown,
            "schema",
            format!("unsupported bundle schema {schema_str:?} (expected {BUNDLE_SCHEMA_VERSION:?})"),
        ));
    }

    // 1.2 required top-level fields (authorization checked in Rule 2; review checked below)
    for &req in REQUIRED_RULE1_KEYS {
        if val.get(req).is_none() {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                req,
                format!("missing required top-level field {req:?}"),
            ));
        }
    }

    // 1.3 claim
    let claim_val = val.get("claim").unwrap();
    let claim_basic = parse_claim_structure(claim_val)?;

    // 1.4 review (OPEN-AUGUR-8: required for mission claims, optional otherwise)
    let review = parse_review_marker(val.get("review"), &claim_basic.kind)?;

    // 1.5 query
    let query = parse_embedded_query(val.get("query").unwrap())?;

    // 1.6 definitions
    let definitions = parse_definitions(val.get("definitions").unwrap())?;

    // 1.7 assumptions
    let assumptions = parse_assumptions(val.get("assumptions").unwrap())?;

    // 1.8 plant
    let plant = parse_plant(val.get("plant").unwrap())?;

    // 1.9 domain (including R5 exact rational validation on cell bounds)
    let domain = parse_domain(val.get("domain").unwrap(), "domain")?;

    // 1.10 proof (kind, version required; primitive_equations presence deferred to Rule 5 R2)
    let (proof, raw_primitive_eqs) = parse_proof_structure(val.get("proof").unwrap())?;

    // 1.11 checker
    let checker = parse_checker(val.get("checker").unwrap())?;

    // 1.12 expected_verdict
    let expected_verdict = parse_expected_verdict(val.get("expected_verdict").unwrap())?;

    // 1.13 lineage
    let lineage = parse_lineage(val.get("lineage").unwrap())?;

    // 1.14 R5 interval declarations & exact rationals inside proof.payload and claim
    validate_r5_intervals_in_value(&proof.payload, "proof.payload")?;
    if let Some(cb_val) = claim_val.get("cost_bounds") {
        validate_r5_intervals_in_value(cb_val, "claim.cost_bounds")?;
    }
    if let Some(um_val) = claim_val.get("unresolved_mass") {
        validate_r5_intervals_in_value(um_val, "claim.unresolved_mass")?;
    }

    // 1.15 Recomputable hashes (`hash.mismatch`, `hash.equations-digest`)
    let actual_query_hash = blake3::hash_hex(query.bytes.as_bytes());
    if actual_query_hash != query.blake3 {
        return Err(RuleRejection::new(
            RuleId::HashMismatch,
            "query.blake3",
            format!(
                "query.blake3 {:?} does not match BLAKE3(query.bytes) {:?}",
                query.blake3, actual_query_hash
            ),
        ));
    }

    if let Some(rev) = &review {
        if rev.equations_reviewed {
            let eq_val = raw_primitive_eqs.ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SideEquationsMissing,
                    "proof.primitive_equations",
                    "equations_reviewed is true but proof.primitive_equations is absent",
                )
            })?;
            let eq_canon = serialize_canonical(eq_val);
            let expected_eq_hash = blake3::hash_hex(eq_canon.as_bytes());
            let declared_eq_hash = rev.equations_digest.as_deref().unwrap_or("");
            if declared_eq_hash != expected_eq_hash {
                return Err(RuleRejection::new(
                    RuleId::HashEquationsDigest,
                    "review.equations_digest",
                    format!(
                        "review.equations_digest {declared_eq_hash:?} does not match BLAKE3(primitive_equations) {expected_eq_hash:?}"
                    ),
                ));
            }
        }
    }

    // =========================================================================
    // RULE 2: Flags (`flags.missing`, `flags.true`; FROB-REF-833)
    //         Runs strictly BEFORE Rule 3 (Kind).
    // =========================================================================
    let flags = validate_authorization_flags(val.get("authorization"))?;

    // =========================================================================
    // RULE 3: Kind registry lookup (`kind.*`; FROB-REF-823)
    //         Not-implemented kinds STILL flow through Rules 4–6.
    // =========================================================================
    let kind_row = match lookup(&proof.kind) {
        None => {
            #[cfg(feature = "shortcut-accept-on-hashes")]
            {
                let bundle = Bundle {
                    schema: schema_str.to_string(),
                    claim: claim_basic,
                    query,
                    definitions,
                    assumptions,
                    plant,
                    domain,
                    proof,
                    checker,
                    expected_verdict,
                    lineage,
                    review,
                    flags,
                };
                return Ok(ValidatedBundle {
                    bundle,
                    kind_row: None,
                    scope: None,
                });
            }
            #[cfg(not(feature = "shortcut-accept-on-hashes"))]
            {
                return Err(RuleRejection::new(
                    RuleId::KindUnknown,
                    "proof.kind",
                    format!("unknown proof kind {:?} in closed registry", proof.kind),
                ));
            }
        }
        Some(_) => match lookup_version(&proof.kind, proof.version) {
            Some(row) => row,
            None => {
                return Err(RuleRejection::new(
                    RuleId::KindVersionUnknown,
                    "proof.version",
                    format!(
                        "unknown version {} for proof kind {:?} in closed registry",
                        proof.version, proof.kind
                    ),
                ));
            }
        },
    };

    let scope = match scope_for_claim_kind(&claim_basic.kind) {
        Ok(s) => s,
        Err(CheckVerdict::Reject { rule, message, .. }) => {
            return Err(RuleRejection::new(rule, "claim.kind", message));
        }
        Err(_) => unreachable!(),
    };

    if !kind_row.allows_scope(scope) {
        return Err(RuleRejection::new(
            RuleId::KindOutOfScope,
            "proof.kind",
            format!(
                "proof kind {:?} v{} is not permitted under scope {}",
                proof.kind,
                proof.version,
                scope.as_str()
            ),
        ));
    }

    if kind_row.status() == KindStatus::DisabledPendingCause {
        return Err(RuleRejection::new(
            RuleId::KindDisabled,
            "proof.kind",
            format!("proof kind {:?} v{} is disabled-pending-cause", proof.kind, proof.version),
        ));
    }

    // =========================================================================
    // RULE 4: Query match (`query.prefix`, `query.domain`, `query.assumptions`,
    //         `query.policy-class`; FROB-REF-826)
    // =========================================================================
    let q_bytes = offered_query_bytes.unwrap_or_else(|| query.bytes.as_bytes());
    validate_query_match(q_bytes, &claim_basic, &domain, &assumptions, &plant, limits)?;

    // =========================================================================
    // RULE 5: Side conditions, typed fields, and format rules R1–R4
    //         (`side.equations-missing`, `side.missing`, `field.type`,
    //          `claim.minimum-unbounded`, `claim.mass-without-weighting`)
    // =========================================================================
    // R2: Primitive equations travel in the bundle.
    if proof.primitive_equations.is_empty()
        || proof.primitive_equations.iter().any(|eq| eq.trim().is_empty())
    {
        return Err(RuleRejection::new(
            RuleId::SideEquationsMissing,
            "proof.primitive_equations",
            "proof.primitive_equations must be a non-empty list of non-empty equations (R2)",
        ));
    }

    // Side conditions required by the proof kind or declared on the proof.
    validate_side_conditions(kind_row, &proof)?;

    // R1: Typed fields in proof.payload (frame, origin, sign, units, torque vs first moment).
    validate_typed_fields(&proof.payload, &definitions)?;

    // R3: Minimum with matching lower bound, or open threshold with candidate + gap.
    let cost_bounds = validate_cost_bounds_opt(
        claim_val.get("cost_bounds").or_else(|| proof.payload.get("cost_bounds")),
        if claim_val.get("cost_bounds").is_some() {
            "claim.cost_bounds"
        } else {
            "proof.payload.cost_bounds"
        },
    )?;

    // R4: Probability mass of unresolved set requires supplied weighting.
    let unresolved_mass = validate_unresolved_mass_opt(
        claim_val
            .get("unresolved_mass")
            .or_else(|| proof.payload.get("unresolved_mass")),
        if claim_val.get("unresolved_mass").is_some() {
            "claim.unresolved_mass"
        } else {
            "proof.payload.unresolved_mass"
        },
    )?;

    let mut claim = claim_basic;
    claim.cost_bounds = cost_bounds;
    claim.unresolved_mass = unresolved_mass;

    // =========================================================================
    // RULE 6: Class rule (`class.above-kind-cap`, `class.above-input`,
    //         `class.no-receipt`; FROB-REF-834)
    // =========================================================================
    if let Err(CheckVerdict::Reject { rule, message, .. }) = check_class_cap(kind_row, claim.class) {
        return Err(RuleRejection::new(rule, "claim.class", message));
    }

    for (idx, a) in assumptions.iter().enumerate() {
        if let Some(input_class) = a.class {
            if claim.class > input_class {
                return Err(RuleRejection::new(
                    RuleId::ClassAboveInput,
                    format!("assumptions[{idx}].class"),
                    format!(
                        "claim class {} exceeds assumption {:?} class {}",
                        claim.class, a.id, input_class
                    ),
                ));
            }
        }
    }
    if let Some(mc) = plant.model_class {
        if claim.class > mc {
            return Err(RuleRejection::new(
                RuleId::ClassAboveInput,
                "plant.model_class",
                format!("claim class {} exceeds plant model class {}", claim.class, mc),
            ));
        }
    }
    for (inp_id, ic) in &plant.input_classes {
        if claim.class > *ic {
            return Err(RuleRejection::new(
                RuleId::ClassAboveInput,
                format!("plant.inputs.{inp_id}"),
                format!(
                    "claim class {} exceeds plant input {inp_id:?} class {}",
                    claim.class, ic
                ),
            ));
        }
    }

    if matches!(claim.class, Class::Proved | Class::Certified) {
        if checker.name.trim().is_empty() {
            return Err(RuleRejection::new(
                RuleId::ClassNoReceipt,
                "checker.name",
                format!("class {} requires a non-empty checker identity", claim.class),
            ));
        }
        if checker.build.trim().is_empty() {
            return Err(RuleRejection::new(
                RuleId::ClassNoReceipt,
                "checker.build",
                format!("class {} requires a non-empty checker build identity", claim.class),
            ));
        }
        if lineage.merkle_root.is_empty() {
            return Err(RuleRejection::new(
                RuleId::ClassNoReceipt,
                "lineage.merkle_root",
                format!("class {} requires a certificate hash (lineage.merkle_root)", claim.class),
            ));
        }
    }

    let bundle = Bundle {
        schema: schema_str.to_string(),
        claim,
        query,
        definitions,
        assumptions,
        plant,
        domain,
        proof,
        checker,
        expected_verdict,
        lineage,
        review,
        flags,
    };

    Ok(ValidatedBundle {
        bundle,
        kind_row: Some(kind_row),
        scope: Some(scope),
    })
}

/// Dispatch a validated bundle to its kind checker (step after generic rules 1–6).
pub fn dispatch_validated(validated: &ValidatedBundle) -> crate::Verdict {
    let (Some(row), Some(scope)) = (validated.kind_row, validated.scope) else {
        // Only reachable under `shortcut-accept-on-hashes` negative-control build.
        return crate::Verdict::Accept;
    };
    match dispatch(row.name(), row.version(), scope) {
        Ok(_checker) => crate::Verdict::Accept,
        Err(CheckVerdict::Accept) => crate::Verdict::Accept,
        Err(CheckVerdict::Reject {
            rule,
            refusal,
            message,
            ..
        }) => crate::Verdict::Reject {
            rule: rule.as_str(),
            refusal,
            path: "proof.kind".to_string(),
            message,
        },
        Err(CheckVerdict::Unsupported { kind, message, .. }) => {
            crate::Verdict::Unsupported { kind, message }
        }
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn json_err_to_rejection(err: crate::json::JsonError) -> RuleRejection {
    let rule = RuleId::parse(err.rule).unwrap_or(RuleId::JsonSyntax);
    RuleRejection {
        rule,
        refusal: err.refusal.or_else(|| rule.refusal_code()),
        path: err.path,
        message: err.message,
    }
}

fn check_closed_keys(
    entries: &[(String, JsonValue)],
    allowed: &[&str],
    prefix: &str,
) -> Result<(), RuleRejection> {
    for (k, _) in entries {
        if !allowed.contains(&k.as_str()) {
            return Err(RuleRejection::new(
                RuleId::SchemaUnknownField,
                format!("{prefix}.{k}"),
                format!("unknown field {k:?} under {prefix}"),
            ));
        }
    }
    Ok(())
}

fn req_str<'a>(obj: &'a JsonValue, key: &str, path: &str) -> Result<&'a str, RuleRejection> {
    match obj.get(key) {
        None => Err(RuleRejection::new(
            RuleId::SchemaMissingField,
            path,
            format!("missing required field {path:?}"),
        )),
        Some(v) => v.as_str().ok_or_else(|| {
            RuleRejection::new(RuleId::SchemaFieldType, path, format!("{path:?} must be a string"))
        }),
    }
}

fn req_non_empty_str<'a>(
    obj: &'a JsonValue,
    key: &str,
    path: &str,
) -> Result<&'a str, RuleRejection> {
    let s = req_str(obj, key, path)?;
    if s.trim().is_empty() {
        return Err(RuleRejection::new(
            RuleId::SchemaFieldType,
            path,
            format!("{path:?} must be a non-empty string"),
        ));
    }
    Ok(s)
}

fn req_blake3<'a>(obj: &'a JsonValue, key: &str, path: &str) -> Result<&'a str, RuleRejection> {
    let s = req_str(obj, key, path)?;
    if !is_blake3_hex(s) {
        return Err(RuleRejection::new(
            RuleId::SchemaFieldType,
            path,
            format!("{path:?} must be a 64-character lowercase hex BLAKE3 digest"),
        ));
    }
    Ok(s)
}

fn parse_claim_structure(val: &JsonValue) -> Result<ClaimStatement, RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "claim", "'claim' must be an object")
    })?;
    check_closed_keys(
        entries,
        &["class", "cost_bounds", "horizon", "kind", "quantifiers", "unresolved_mass"],
        "claim",
    )?;
    let kind = req_non_empty_str(val, "kind", "claim.kind")?.to_string();
    let class_str = req_str(val, "class", "claim.class")?;
    let class = Class::parse(class_str).ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaFieldType,
            "claim.class",
            format!("invalid claim.class {class_str:?} (expected Proved|Certified|Tested|Estimated)"),
        )
    })?;
    let q_arr = match val.get("quantifiers") {
        None => {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                "claim.quantifiers",
                "missing required field 'claim.quantifiers'",
            ));
        }
        Some(v) => v.as_arr().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                "claim.quantifiers",
                "'claim.quantifiers' must be an array of strings",
            )
        })?,
    };
    let mut quantifiers = Vec::with_capacity(q_arr.len());
    for (i, item) in q_arr.iter().enumerate() {
        let s = item.as_str().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("claim.quantifiers[{i}]"),
                "quantifier entry must be a string",
            )
        })?;
        if s.trim().is_empty() {
            return Err(RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("claim.quantifiers[{i}]"),
                "quantifier entry must be non-empty",
            ));
        }
        quantifiers.push(s.to_string());
    }
    let horizon = req_non_empty_str(val, "horizon", "claim.horizon")?.to_string();

    Ok(ClaimStatement {
        kind,
        class,
        quantifiers,
        horizon,
        cost_bounds: None,
        unresolved_mass: None,
    })
}

fn parse_review_marker(
    opt: Option<&JsonValue>,
    claim_kind: &str,
) -> Result<Option<ReviewMarker>, RuleRejection> {
    let is_mission = MISSION_CLAIM_KINDS.contains(&claim_kind);
    let Some(val) = opt else {
        if is_mission {
            return Err(RuleRejection::new(
                RuleId::SchemaReviewMissing,
                "review",
                format!(
                    "mission claim kind {claim_kind:?} requires the 'review' object (OPEN-AUGUR-8)"
                ),
            ));
        }
        return Ok(None);
    };
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "review", "'review' must be an object")
    })?;
    check_closed_keys(
        entries,
        &["equations_digest", "equations_reviewed", "review_record", "scope"],
        "review",
    )?;
    let equations_reviewed = match val.get("equations_reviewed") {
        None => {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                "review.equations_reviewed",
                "missing required boolean 'review.equations_reviewed'",
            ));
        }
        Some(v) => v.as_bool().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                "review.equations_reviewed",
                "'review.equations_reviewed' must be a boolean",
            )
        })?,
    };
    if equations_reviewed {
        let eq_dig = req_blake3(val, "equations_digest", "review.equations_digest")?.to_string();
        let rec = req_non_empty_str(val, "review_record", "review.review_record")?.to_string();
        let scope = req_non_empty_str(val, "scope", "review.scope")?.to_string();
        Ok(Some(ReviewMarker {
            equations_reviewed: true,
            equations_digest: Some(eq_dig),
            review_record: Some(rec),
            scope: Some(scope),
        }))
    } else {
        let eq_dig = match val.get("equations_digest") {
            None => None,
            Some(v) => {
                let s = v.as_str().ok_or_else(|| {
                    RuleRejection::new(
                        RuleId::SchemaFieldType,
                        "review.equations_digest",
                        "'review.equations_digest' must be a string",
                    )
                })?;
                if !s.is_empty() && !is_blake3_hex(s) {
                    return Err(RuleRejection::new(
                        RuleId::SchemaFieldType,
                        "review.equations_digest",
                        "invalid BLAKE3 hex in review.equations_digest",
                    ));
                }
                Some(s.to_string())
            }
        };
        let rec = match val.get("review_record") {
            None => None,
            Some(v) => Some(
                v.as_str()
                    .ok_or_else(|| {
                        RuleRejection::new(
                            RuleId::SchemaFieldType,
                            "review.review_record",
                            "'review.review_record' must be a string",
                        )
                    })?
                    .to_string(),
            ),
        };
        let scope = match val.get("scope") {
            None => None,
            Some(v) => Some(
                v.as_str()
                    .ok_or_else(|| {
                        RuleRejection::new(
                            RuleId::SchemaFieldType,
                            "review.scope",
                            "'review.scope' must be a string",
                        )
                    })?
                    .to_string(),
            ),
        };
        Ok(Some(ReviewMarker {
            equations_reviewed: false,
            equations_digest: eq_dig,
            review_record: rec,
            scope,
        }))
    }
}

fn parse_embedded_query(val: &JsonValue) -> Result<EmbeddedQuery, RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "query", "'query' must be an object")
    })?;
    check_closed_keys(entries, &["blake3", "bytes"], "query")?;
    let blake3_hex = req_blake3(val, "blake3", "query.blake3")?.to_string();
    let bytes = req_non_empty_str(val, "bytes", "query.bytes")?.to_string();
    Ok(EmbeddedQuery {
        blake3: blake3_hex,
        bytes,
    })
}

fn parse_definitions(val: &JsonValue) -> Result<Definitions, RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaFieldType,
            "definitions",
            "'definitions' must be an object",
        )
    })?;
    check_closed_keys(
        entries,
        &[
            "clocks",
            "frame",
            "handedness",
            "origin",
            "ports_ordered",
            "sign_convention",
            "units",
        ],
        "definitions",
    )?;
    let units = req_non_empty_str(val, "units", "definitions.units")?.to_string();
    let frame = req_non_empty_str(val, "frame", "definitions.frame")?.to_string();
    let clocks = match val.get("clocks") {
        None => {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                "definitions.clocks",
                "missing required field 'definitions.clocks'",
            ));
        }
        Some(v) => v
            .as_arr()
            .ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SchemaFieldType,
                    "definitions.clocks",
                    "'definitions.clocks' must be an array",
                )
            })?
            .to_vec(),
    };
    let ports_arr = match val.get("ports_ordered") {
        None => {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                "definitions.ports_ordered",
                "missing required field 'definitions.ports_ordered'",
            ));
        }
        Some(v) => v.as_arr().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                "definitions.ports_ordered",
                "'definitions.ports_ordered' must be an array of strings",
            )
        })?,
    };
    let mut ports_ordered = Vec::with_capacity(ports_arr.len());
    let mut seen_ports = BTreeSet::new();
    for (i, item) in ports_arr.iter().enumerate() {
        let p = item.as_str().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("definitions.ports_ordered[{i}]"),
                "port name must be a string",
            )
        })?;
        if p.trim().is_empty() || !seen_ports.insert(p) {
            return Err(RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("definitions.ports_ordered[{i}]"),
                format!("invalid or duplicate port name {p:?}"),
            ));
        }
        ports_ordered.push(p.to_string());
    }
    Ok(Definitions {
        units,
        frame,
        clocks,
        ports_ordered,
    })
}

fn parse_assumptions(val: &JsonValue) -> Result<Vec<Assumption>, RuleRejection> {
    let arr = val.as_arr().ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaFieldType,
            "assumptions",
            "'assumptions' must be an array",
        )
    })?;
    let mut out = Vec::with_capacity(arr.len());
    let mut seen_ids = BTreeSet::new();
    for (i, item) in arr.iter().enumerate() {
        let prefix = format!("assumptions[{i}]");
        let entries = item.as_obj().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                &prefix,
                "assumption entry must be an object",
            )
        })?;
        check_closed_keys(entries, &["blake3", "class", "id", "survives", "value"], &prefix)?;
        let id = req_non_empty_str(item, "id", &format!("{prefix}.id"))?.to_string();
        if !seen_ids.insert(id.clone()) {
            return Err(RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("{prefix}.id"),
                format!("duplicate assumption id {id:?}"),
            ));
        }
        let surv_str = req_str(item, "survives", &format!("{prefix}.survives"))?;
        let survives = Polarity::parse(surv_str).ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("{prefix}.survives"),
                format!("invalid survives polarity {surv_str:?} (expected grow|shrink)"),
            )
        })?;
        let value = match item.get("value") {
            None => None,
            Some(JsonValue::Null) => {
                return Err(RuleRejection::new(
                    RuleId::SchemaFieldType,
                    format!("{prefix}.value"),
                    "assumption value must not be null",
                ));
            }
            Some(v) => Some(v.clone()),
        };
        let b3 = match item.get("blake3") {
            None => None,
            Some(_) => Some(req_blake3(item, "blake3", &format!("{prefix}.blake3"))?.to_string()),
        };
        if value.is_none() && b3.is_none() {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                &prefix,
                "assumption must carry at least one of 'value' or 'blake3'",
            ));
        }
        let class = match item.get("class") {
            None => None,
            Some(v) => {
                let cs = v.as_str().ok_or_else(|| {
                    RuleRejection::new(
                        RuleId::SchemaFieldType,
                        format!("{prefix}.class"),
                        "assumption class must be a string",
                    )
                })?;
                let c = Class::parse(cs).ok_or_else(|| {
                    RuleRejection::new(
                        RuleId::SchemaFieldType,
                        format!("{prefix}.class"),
                        format!("invalid assumption class {cs:?}"),
                    )
                })?;
                Some(c)
            }
        };
        out.push(Assumption {
            id,
            value,
            blake3: b3,
            survives,
            class,
        });
    }
    Ok(out)
}

fn parse_plant(val: &JsonValue) -> Result<PlantRef, RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "plant", "'plant' must be an object")
    })?;
    check_closed_keys(entries, &["inputs", "model", "model_class", "policy_class"], "plant")?;
    let model = req_non_empty_str(val, "model", "plant.model")?.to_string();
    let policy_class = req_blake3(val, "policy_class", "plant.policy_class")?.to_string();
    let model_class = match val.get("model_class") {
        None => None,
        Some(v) => {
            let cs = v.as_str().ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SchemaFieldType,
                    "plant.model_class",
                    "plant.model_class must be a string",
                )
            })?;
            Some(Class::parse(cs).ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SchemaFieldType,
                    "plant.model_class",
                    format!("invalid plant.model_class {cs:?}"),
                )
            })?)
        }
    };
    let mut input_classes = Vec::new();
    if let Some(inps) = val.get("inputs") {
        let arr = inps.as_arr().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                "plant.inputs",
                "plant.inputs must be an array",
            )
        })?;
        for (i, item) in arr.iter().enumerate() {
            let pfx = format!("plant.inputs[{i}]");
            let id = req_non_empty_str(item, "id", &format!("{pfx}.id"))?.to_string();
            let cs = req_str(item, "class", &format!("{pfx}.class"))?;
            let c = Class::parse(cs).ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SchemaFieldType,
                    format!("{pfx}.class"),
                    format!("invalid input class {cs:?}"),
                )
            })?;
            input_classes.push((id, c));
        }
    }
    Ok(PlantRef {
        model,
        policy_class,
        model_class,
        input_classes,
    })
}

fn parse_rational_value(val: &JsonValue, path: &str) -> Result<Rational, RuleRejection> {
    match val {
        JsonValue::Arr(_) => val.parse_rational_pair().map_err(|e| {
            RuleRejection::new(RuleId::NumRationalForm, path, e.to_string())
        }),
        JsonValue::Str(_) => val.parse_rational_slash().map_err(|e| {
            RuleRejection::new(RuleId::NumRationalForm, path, e.to_string())
        }),
        _ => Err(RuleRejection::new(
            RuleId::NumRationalForm,
            path,
            "expected exact rational pair [\"p\",\"q\"] or strict rational string",
        )),
    }
}

fn parse_rational_coord_list(val: &JsonValue, path: &str) -> Result<Vec<Rational>, RuleRejection> {
    let arr = val.as_arr().ok_or_else(|| {
        RuleRejection::new(
            RuleId::NumRationalForm,
            path,
            "expected rational pair [\"p\",\"q\"] or list of rational pairs",
        )
    })?;
    if arr.is_empty() {
        return Err(RuleRejection::new(
            RuleId::SchemaFieldType,
            path,
            "cell bound cannot be empty",
        ));
    }
    if arr[0].as_str().is_some() {
        // 1D rational pair ["p", "q"]
        return Ok(vec![parse_rational_value(val, path)?]);
    }
    // Multi-coordinate list [["p1","q1"], ["p2","q2"], ...]
    let mut out = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        out.push(parse_rational_value(item, &format!("{path}[{i}]"))?);
    }
    Ok(out)
}

fn parse_domain(val: &JsonValue, path: &str) -> Result<Domain, RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, path, "'domain' must be an object")
    })?;
    check_closed_keys(entries, &["cell", "chart", "closed", "unit"], path)?;
    let chart = req_non_empty_str(val, "chart", &format!("{path}.chart"))?.to_string();
    let cell_val = val.get("cell").ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaMissingField,
            format!("{path}.cell"),
            "missing required field 'domain.cell'",
        )
    })?;
    let cell_entries = cell_val.as_obj().ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaFieldType,
            format!("{path}.cell"),
            "'domain.cell' must be an object",
        )
    })?;
    check_closed_keys(
        cell_entries,
        &["closed", "hi", "hi_closed", "id", "lo", "lo_closed", "open_ends", "topology"],
        &format!("{path}.cell"),
    )?;
    let lo_val = cell_val.get("lo").ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaMissingField,
            format!("{path}.cell.lo"),
            "missing required field 'domain.cell.lo'",
        )
    })?;
    let hi_val = cell_val.get("hi").ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaMissingField,
            format!("{path}.cell.hi"),
            "missing required field 'domain.cell.hi'",
        )
    })?;
    let lo = parse_rational_coord_list(lo_val, &format!("{path}.cell.lo"))?;
    let hi = parse_rational_coord_list(hi_val, &format!("{path}.cell.hi"))?;
    if lo.len() != hi.len() {
        return Err(RuleRejection::new(
            RuleId::SchemaFieldType,
            format!("{path}.cell"),
            format!("domain.cell.lo dim {} != hi dim {}", lo.len(), hi.len()),
        ));
    }
    for (i, (l, h)) in lo.iter().zip(hi.iter()).enumerate() {
        if l > h {
            return Err(RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("{path}.cell"),
                format!("domain.cell lower bound exceeds upper bound at coord {i}"),
            ));
        }
    }
    let closed = parse_closed_ends(cell_val.get("closed"), lo.len(), &format!("{path}.cell.closed"))?;
    Ok(Domain {
        chart,
        cell: DomainCell { lo, hi, closed },
    })
}

fn parse_closed_ends(
    opt: Option<&JsonValue>,
    dim: usize,
    path: &str,
) -> Result<Option<Vec<(bool, bool)>>, RuleRejection> {
    let Some(v) = opt else {
        return Ok(None);
    };
    let arr = v.as_arr().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, path, "'closed' must be an array")
    })?;
    if arr.len() == 2 && arr[0].as_bool().is_some() && arr[1].as_bool().is_some() {
        if dim != 1 {
            return Err(RuleRejection::new(
                RuleId::SchemaFieldType,
                path,
                format!("1D closed marker does not match {dim}D cell"),
            ));
        }
        return Ok(Some(vec![(arr[0].as_bool().unwrap(), arr[1].as_bool().unwrap())]));
    }
    if arr.len() != dim {
        return Err(RuleRejection::new(
            RuleId::SchemaFieldType,
            path,
            format!("closed array length {} does not match cell dimension {dim}", arr.len()),
        ));
    }
    let mut out = Vec::with_capacity(dim);
    for (i, item) in arr.iter().enumerate() {
        let pair = item.as_arr().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("{path}[{i}]"),
                "expected [lo_closed, hi_closed] booleans",
            )
        })?;
        if pair.len() != 2 {
            return Err(RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("{path}[{i}]"),
                "expected 2 booleans for [lo_closed, hi_closed]",
            ));
        }
        let a = pair[0].as_bool().ok_or_else(|| {
            RuleRejection::new(RuleId::SchemaFieldType, format!("{path}[{i}][0]"), "expected bool")
        })?;
        let b = pair[1].as_bool().ok_or_else(|| {
            RuleRejection::new(RuleId::SchemaFieldType, format!("{path}[{i}][1]"), "expected bool")
        })?;
        out.push((a, b));
    }
    Ok(Some(out))
}

fn parse_proof_structure<'a>(
    val: &'a JsonValue,
) -> Result<(BundleProof, Option<&'a JsonValue>), RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "proof", "'proof' must be an object")
    })?;
    check_closed_keys(
        entries,
        &[
            "equations_dag",
            "kind",
            "payload",
            "primitive_equations",
            "receipt",
            "required_side_conditions",
            "side_conditions",
            "version",
        ],
        "proof",
    )?;
    let kind = req_non_empty_str(val, "kind", "proof.kind")?.to_string();
    let version = match val.get("version") {
        None => {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                "proof.version",
                "missing required integer field 'proof.version'",
            ));
        }
        Some(v) => match v.as_i64() {
            Some(n) if n >= 1 && n <= i64::from(u32::MAX) => n as u32,
            _ => {
                return Err(RuleRejection::new(
                    RuleId::SchemaFieldType,
                    "proof.version",
                    "'proof.version' must be a positive integer >= 1",
                ));
            }
        },
    };
    let raw_eqs = val.get("primitive_equations");
    let mut primitive_equations = Vec::new();
    if let Some(eq_v) = raw_eqs {
        let arr = eq_v.as_arr().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                "proof.primitive_equations",
                "'proof.primitive_equations' must be an array of strings",
            )
        })?;
        for (i, item) in arr.iter().enumerate() {
            let s = item.as_str().ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SchemaFieldType,
                    format!("proof.primitive_equations[{i}]"),
                    "primitive equation entry must be a string",
                )
            })?;
            primitive_equations.push(s.to_string());
        }
    }
    let payload = match val.get("payload") {
        None => {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                "proof.payload",
                "missing required object 'proof.payload'",
            ));
        }
        Some(v) if v.as_obj().is_some() => v.clone(),
        Some(_) => {
            return Err(RuleRejection::new(
                RuleId::SchemaFieldType,
                "proof.payload",
                "'proof.payload' must be a JSON object",
            ));
        }
    };
    let side_conditions = match val.get("side_conditions") {
        None => Vec::new(),
        Some(v) => v
            .as_arr()
            .ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SchemaFieldType,
                    "proof.side_conditions",
                    "'proof.side_conditions' must be an array",
                )
            })?
            .to_vec(),
    };
    let mut required_side_conditions = Vec::new();
    if let Some(req_sc) = val.get("required_side_conditions") {
        let arr = req_sc.as_arr().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                "proof.required_side_conditions",
                "'proof.required_side_conditions' must be an array of strings",
            )
        })?;
        for (i, item) in arr.iter().enumerate() {
            let s = item.as_str().ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SchemaFieldType,
                    format!("proof.required_side_conditions[{i}]"),
                    "required side condition entry must be a string",
                )
            })?;
            required_side_conditions.push(s.to_string());
        }
    }
    Ok((
        BundleProof {
            kind,
            version,
            primitive_equations,
            payload,
            side_conditions,
            required_side_conditions,
        },
        raw_eqs,
    ))
}

fn parse_checker(val: &JsonValue) -> Result<CheckerRef, RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "checker", "'checker' must be an object")
    })?;
    check_closed_keys(entries, &["build", "name", "registry", "version"], "checker")?;
    let name = req_str(val, "name", "checker.name")?.to_string();
    let build = req_str(val, "build", "checker.build")?.to_string();
    Ok(CheckerRef { name, build })
}

fn parse_expected_verdict(val: &JsonValue) -> Result<ExpectedVerdict, RuleRejection> {
    let s = val.as_str().ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaFieldType,
            "expected_verdict",
            "'expected_verdict' must be a string",
        )
    })?;
    match s {
        "accept" => Ok(ExpectedVerdict::Accept),
        "reject" => Ok(ExpectedVerdict::Reject),
        _ => Err(RuleRejection::new(
            RuleId::SchemaFieldType,
            "expected_verdict",
            format!("invalid expected_verdict {s:?} (expected accept|reject)"),
        )),
    }
}

fn parse_lineage(val: &JsonValue) -> Result<Lineage, RuleRejection> {
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(RuleId::SchemaFieldType, "lineage", "'lineage' must be an object")
    })?;
    check_closed_keys(
        entries,
        &["alpha_spend", "merkle_root", "parent_hashes", "receipt_hash"],
        "lineage",
    )?;
    let merkle_root = req_str(val, "merkle_root", "lineage.merkle_root")?.to_string();
    if !merkle_root.is_empty() && !is_blake3_hex(&merkle_root) {
        return Err(RuleRejection::new(
            RuleId::SchemaFieldType,
            "lineage.merkle_root",
            "lineage.merkle_root must be empty or a 64-character lowercase hex BLAKE3 digest",
        ));
    }
    let alpha_spend = match val.get("alpha_spend") {
        None => {
            return Err(RuleRejection::new(
                RuleId::SchemaMissingField,
                "lineage.alpha_spend",
                "missing required field 'lineage.alpha_spend'",
            ));
        }
        Some(v) => v
            .as_arr()
            .ok_or_else(|| {
                RuleRejection::new(
                    RuleId::SchemaFieldType,
                    "lineage.alpha_spend",
                    "'lineage.alpha_spend' must be an array",
                )
            })?
            .to_vec(),
    };
    Ok(Lineage {
        merkle_root,
        alpha_spend,
    })
}

fn validate_authorization_flags(opt: Option<&JsonValue>) -> Result<NoAuthorization, RuleRejection> {
    let val = opt.ok_or_else(|| {
        RuleRejection::new(
            RuleId::FlagsMissing,
            "authorization",
            "missing required 'authorization' object (absence is never read as false)",
        )
    })?;
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaFieldType,
            "authorization",
            "'authorization' must be a JSON object",
        )
    })?;
    check_closed_keys(entries, AUTHORIZATION_FLAGS, "authorization")?;

    let mut parsed = [false; 3];
    for (idx, &flag) in AUTHORIZATION_FLAGS.iter().enumerate() {
        let path = format!("authorization.{flag}");
        let Some(fv) = val.get(flag) else {
            return Err(RuleRejection::new(
                RuleId::FlagsMissing,
                path,
                format!("missing required no-authorization flag {flag:?}"),
            ));
        };
        let b = fv.as_bool().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                &path,
                format!("authorization flag {flag:?} must be boolean false"),
            )
        })?;
        if b {
            return Err(RuleRejection::new(
                RuleId::FlagsTrue,
                path,
                format!("authorization flag {flag:?} is true; no code path authorizes execution or hardware"),
            ));
        }
        parsed[idx] = b;
    }

    Ok(NoAuthorization {
        execution_authorized: parsed[0],
        hardware_safety_signoff: parsed[1],
        sim_to_real_certified: parsed[2],
    })
}

fn validate_r5_intervals_in_value(val: &JsonValue, path: &str) -> Result<(), RuleRejection> {
    match val {
        JsonValue::Obj(entries) => {
            let is_explicit_interval = val
                .get("kind")
                .and_then(JsonValue::as_str)
                == Some("interval")
                || (val.get("lo").is_some() && val.get("hi").is_some());
            if is_explicit_interval {
                let impl_ok = val
                    .get("impl")
                    .and_then(JsonValue::as_str)
                    .is_some_and(|s| !s.trim().is_empty());
                let prec_ok = val
                    .get("precision")
                    .and_then(JsonValue::as_i64)
                    .is_some_and(|p| p >= 1);
                if !impl_ok || !prec_ok {
                    return Err(RuleRejection::new(
                        RuleId::NumIntervalDecl,
                        path,
                        "outward-rounded interval must declare non-empty 'impl' and positive integer 'precision' (R5)",
                    ));
                }
                let lo_v = val.get("lo").ok_or_else(|| {
                    RuleRejection::new(
                        RuleId::NumIntervalDecl,
                        format!("{path}.lo"),
                        "interval missing 'lo' bound",
                    )
                })?;
                let hi_v = val.get("hi").ok_or_else(|| {
                    RuleRejection::new(
                        RuleId::NumIntervalDecl,
                        format!("{path}.hi"),
                        "interval missing 'hi' bound",
                    )
                })?;
                let lo = parse_rational_value(lo_v, &format!("{path}.lo"))?;
                let hi = parse_rational_value(hi_v, &format!("{path}.hi"))?;
                if lo > hi {
                    return Err(RuleRejection::new(
                        RuleId::NumIntervalDecl,
                        path,
                        "interval lower bound exceeds upper bound",
                    ));
                }
                return Ok(());
            }
            for (k, v) in entries {
                let child_path = format!("{path}.{k}");
                if k == "forces_newton"
                    || k.starts_with("forces_newton_")
                    || k == "multipliers"
                {
                    validate_nested_rationals(v, &child_path)?;
                } else {
                    validate_r5_intervals_in_value(v, &child_path)?;
                }
            }
            Ok(())
        }
        JsonValue::Arr(items) => {
            for (i, item) in items.iter().enumerate() {
                validate_r5_intervals_in_value(item, &format!("{path}[{i}]"))?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_nested_rationals(val: &JsonValue, path: &str) -> Result<(), RuleRejection> {
    let arr = val.as_arr().ok_or_else(|| {
        RuleRejection::new(RuleId::NumRationalForm, path, "expected array of rationals")
    })?;
    if arr.len() == 2 && arr[0].as_str().is_some() && arr[1].as_str().is_some() {
        let _ = parse_rational_value(val, path)?;
        return Ok(());
    }
    for (i, item) in arr.iter().enumerate() {
        validate_nested_rationals(item, &format!("{path}[{i}]"))?;
    }
    Ok(())
}

fn validate_query_match(
    offered_query_bytes: &[u8],
    claim: &ClaimStatement,
    domain: &Domain,
    assumptions: &[Assumption],
    plant: &PlantRef,
    limits: &Limits,
) -> Result<(), RuleRejection> {
    let q_val = parse_non_canonical(offered_query_bytes, limits).map_err(|e| {
        RuleRejection::new(
            RuleId::QueryMismatch,
            "query.bytes",
            format!("offered query is not valid JSON: {e}"),
        )
    })?;
    if q_val.as_obj().is_none() {
        return Err(RuleRejection::new(
            RuleId::QueryMismatch,
            "query.bytes",
            "offered query must be a JSON object",
        ));
    }

    // 4a. Quantifier prefix (exact ordered sequence).
    let q_quantifiers = extract_query_quantifiers(&q_val)?;
    if q_quantifiers != claim.quantifiers {
        return Err(RuleRejection::new(
            RuleId::QueryPrefix,
            "claim.quantifiers",
            format!(
                "bundle quantifier prefix {:?} does not match offered query prefix {:?}",
                claim.quantifiers, q_quantifiers
            ),
        ));
    }

    // 4b. Domain (field-by-field: chart, exact rational cell bounds, end closures).
    let q_domain_val = q_val.get("domain").ok_or_else(|| {
        RuleRejection::new(
            RuleId::QueryDomain,
            "domain",
            "offered query is missing 'domain' block",
        )
    })?;
    let q_domain = parse_domain(q_domain_val, "offered_query.domain").map_err(|e| {
        RuleRejection::new(
            RuleId::QueryDomain,
            "domain",
            format!("offered query domain invalid: {}", e.message),
        )
    })?;
    if q_domain != *domain {
        return Err(RuleRejection::new(
            RuleId::QueryDomain,
            "domain",
            "bundle domain does not match offered query domain",
        ));
    }

    // 4c. Assumption set (order-independent set equality on id, survives, value/blake3).
    let q_assump_val = q_val.get("assumptions").ok_or_else(|| {
        RuleRejection::new(
            RuleId::QueryAssumptions,
            "assumptions",
            "offered query is missing 'assumptions' array",
        )
    })?;
    let q_assumptions = parse_assumptions(q_assump_val).map_err(|e| {
        RuleRejection::new(
            RuleId::QueryAssumptions,
            "assumptions",
            format!("offered query assumptions invalid: {}", e.message),
        )
    })?;
    if !assumption_sets_equal(assumptions, &q_assumptions) {
        return Err(RuleRejection::new(
            RuleId::QueryAssumptions,
            "assumptions",
            "bundle assumption set does not match offered query assumption set",
        ));
    }

    // 4d. Policy class digest (channel, timing with arrival instants, indexing, randomization).
    let _ = plant;
    #[cfg(not(feature = "nc-omit-policy-class"))]
    {
        let q_pc = q_val.get("policy_class").ok_or_else(|| {
            RuleRejection::new(
                RuleId::QueryPolicyClass,
                "plant.policy_class",
                "offered query is missing 'policy_class' section",
            )
        })?;
        let canonical_pc = serialize_canonical(q_pc);
        let recomputed_pc = blake3::hash_hex(canonical_pc.as_bytes());
        if plant.policy_class != recomputed_pc {
            return Err(RuleRejection::new(
                RuleId::QueryPolicyClass,
                "plant.policy_class",
                format!(
                    "bundle plant.policy_class {:?} does not match recomputed digest {:?} of offered query policy_class",
                    plant.policy_class, recomputed_pc
                ),
            ));
        }
    }

    Ok(())
}

fn extract_query_quantifiers(q_val: &JsonValue) -> Result<Vec<String>, RuleRejection> {
    let arr = q_val
        .get("quantifiers")
        .and_then(JsonValue::as_arr)
        .ok_or_else(|| {
            RuleRejection::new(
                RuleId::QueryPrefix,
                "claim.quantifiers",
                "offered query is missing 'quantifiers' array",
            )
        })?;
    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        if let Some(s) = item.as_str() {
            out.push(s.to_string());
        } else if let Some(q) = item.get("q").and_then(JsonValue::as_str) {
            let var = item.get("var").and_then(JsonValue::as_str).unwrap_or("");
            let scope = item.get("scope").and_then(JsonValue::as_str).unwrap_or("");
            let canonical = match (q, var, scope) {
                ("exists", "policy", "per-nominal-design") => {
                    "exists_causal_policy_per_design".to_string()
                }
                ("forall", "world", _) => "forall_admitted_worlds".to_string(),
                ("forall", "execution", _) => "forall_admitted_executions".to_string(),
                _ if scope.is_empty() => format!("{q}_{var}"),
                _ => format!("{q}_{var}_{scope}"),
            };
            out.push(canonical);
        } else {
            return Err(RuleRejection::new(
                RuleId::QueryPrefix,
                "claim.quantifiers",
                "offered query contains malformed quantifier entry",
            ));
        }
    }
    Ok(out)
}

fn assumption_sets_equal(a: &[Assumption], b: &[Assumption]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let key = |x: &Assumption| {
        let val_canon = x.value.as_ref().map(serialize_canonical);
        (x.id.clone(), x.survives, val_canon, x.blake3.clone())
    };
    let mut sa: Vec<_> = a.iter().map(key).collect();
    let mut sb: Vec<_> = b.iter().map(key).collect();
    sa.sort();
    sb.sort();
    sa == sb
}

fn validate_side_conditions(kind_row: &KindRow, proof: &BundleProof) -> Result<(), RuleRejection> {
    let kind_requires = !kind_row.requires_side_conditions().is_empty()
        || matches!(
            kind_row.name(),
            "partition" | "monotone-transport" | "symmetry-transport" | "composition"
        );
    if kind_requires && proof.side_conditions.is_empty() {
        return Err(RuleRejection::new(
            RuleId::SideMissing,
            "proof.side_conditions",
            format!(
                "proof kind {:?} requires side-condition sub-certificates",
                kind_row.name()
            ),
        ));
    }
    if !proof.required_side_conditions.is_empty() {
        let present: BTreeSet<&str> = proof
            .side_conditions
            .iter()
            .filter_map(|v| {
                v.as_str()
                    .or_else(|| v.get("id").and_then(JsonValue::as_str))
            })
            .collect();
        for req in &proof.required_side_conditions {
            if !present.contains(req.as_str()) {
                return Err(RuleRejection::new(
                    RuleId::SideMissing,
                    "proof.side_conditions",
                    format!("missing required side-condition sub-certificate {req:?}"),
                ));
            }
        }
    }
    Ok(())
}

fn validate_typed_fields(payload: &JsonValue, defs: &Definitions) -> Result<(), RuleRejection> {
    let Some(entries) = payload.as_obj() else {
        return Ok(());
    };

    // Check `forces_newton` port-count match against `definitions.ports_ordered`.
    if let Some(forces) = payload.get("forces_newton").and_then(JsonValue::as_arr) {
        if !defs.ports_ordered.is_empty() && forces.len() != defs.ports_ordered.len() {
            return Err(RuleRejection::new(
                RuleId::FieldType,
                "proof.payload.forces_newton",
                format!(
                    "forces_newton length {} does not match definitions.ports_ordered length {}",
                    forces.len(),
                    defs.ports_ordered.len()
                ),
            ));
        }
    }

    // Parse optional `first_moment` (2D planar load first moment (P_x, P_y) in N*m).
    let mut first_moment_2d: Option<[Rational; 2]> = None;
    if let Some(fm_val) = payload.get("first_moment") {
        let vec = validate_typed_physical_vector(
            fm_val,
            "proof.payload.first_moment",
            "first-moment",
            2,
            &["N*m", "N·m"],
            defs,
        )?;
        first_moment_2d = Some([vec[0].clone(), vec[1].clone()]);
    }

    // Validate `torque` (3D torque vector in N*m; under +z up, tau = (P_y, -P_x, 0), CX-108 / CX-077).
    if let Some(tq_val) = payload.get("torque") {
        let tq = validate_typed_physical_vector(
            tq_val,
            "proof.payload.torque",
            "torque",
            3,
            &["N*m", "N·m"],
            defs,
        )?;
        if let Some([px, py]) = &first_moment_2d {
            if defs.frame.contains("+z up") {
                let expected_tx = py.clone();
                let expected_ty = px.negate();
                let expected_tz = Rational::zero();
                if tq[0] != expected_tx || tq[1] != expected_ty || tq[2] != expected_tz {
                    return Err(RuleRejection::new(
                        RuleId::FieldType,
                        "proof.payload.torque",
                        "torque vector disagrees with first_moment under +z up sign convention: expected (P_y, -P_x, 0)",
                    ));
                }
            }
        }
    }

    // Validate any generic typed physical field carrying `"quantity"`.
    for (k, v) in entries {
        if k == "torque" || k == "first_moment" {
            continue;
        }
        if v.as_obj().is_some() && v.get("quantity").is_some() {
            validate_typed_field_metadata(v, &format!("proof.payload.{k}"), None, None, defs)?;
        }
    }
    Ok(())
}

fn validate_typed_field_metadata(
    val: &JsonValue,
    path: &str,
    expected_quantity: Option<&str>,
    allowed_units: Option<&[&str]>,
    defs: &Definitions,
) -> Result<(), RuleRejection> {
    if val.as_obj().is_none() {
        return Err(RuleRejection::new(
            RuleId::FieldType,
            path,
            format!("typed physical field {path:?} must be an object declaring quantity, frame, origin, sign, unit, and value"),
        ));
    }
    for req in ["quantity", "frame", "origin", "sign", "unit"] {
        match val.get(req).and_then(JsonValue::as_str) {
            Some(s) if !s.trim().is_empty() => {}
            _ => {
                return Err(RuleRejection::new(
                    RuleId::FieldType,
                    path,
                    format!("typed field {path:?} is missing non-empty string {req:?} (R1)"),
                ));
            }
        }
    }
    if val.get("value").is_none() {
        return Err(RuleRejection::new(
            RuleId::FieldType,
            path,
            format!("typed field {path:?} is missing 'value'"),
        ));
    }
    let qty = val.get("quantity").and_then(JsonValue::as_str).unwrap();
    if let Some(exp_q) = expected_quantity {
        if qty != exp_q {
            return Err(RuleRejection::new(
                RuleId::FieldType,
                path,
                format!("expected quantity {exp_q:?} at {path:?}, found {qty:?}"),
            ));
        }
    }
    let frame = val.get("frame").and_then(JsonValue::as_str).unwrap();
    if frame != defs.frame {
        return Err(RuleRejection::new(
            RuleId::FieldType,
            path,
            format!(
                "field frame {frame:?} does not match definitions.frame {:?}",
                defs.frame
            ),
        ));
    }
    let unit = val.get("unit").and_then(JsonValue::as_str).unwrap();
    if defs.units == "SI" && NON_SI_UNITS.contains(&unit) {
        return Err(RuleRejection::new(
            RuleId::FieldType,
            path,
            format!("non-SI unit {unit:?} is incompatible with definitions.units=\"SI\""),
        ));
    }
    if let Some(allowed) = allowed_units {
        if !allowed.contains(&unit) {
            return Err(RuleRejection::new(
                RuleId::FieldType,
                path,
                format!("unit {unit:?} is not permitted for {path:?} (expected one of {allowed:?})"),
            ));
        }
    }
    Ok(())
}

fn validate_typed_physical_vector(
    val: &JsonValue,
    path: &str,
    expected_quantity: &str,
    expected_dim: usize,
    allowed_units: &[&str],
    defs: &Definitions,
) -> Result<Vec<Rational>, RuleRejection> {
    validate_typed_field_metadata(val, path, Some(expected_quantity), Some(allowed_units), defs)?;
    let arr = val
        .get("value")
        .and_then(JsonValue::as_arr)
        .ok_or_else(|| {
            RuleRejection::new(
                RuleId::FieldType,
                path,
                format!("typed vector field {path:?} value must be a {expected_dim}-element rational vector"),
            )
        })?;
    if arr.len() != expected_dim {
        return Err(RuleRejection::new(
            RuleId::FieldType,
            path,
            format!(
                "typed field {path:?} ({expected_quantity}) expects {expected_dim} components, got {}",
                arr.len()
            ),
        ));
    }
    let mut out = Vec::with_capacity(expected_dim);
    for (i, item) in arr.iter().enumerate() {
        out.push(parse_rational_value(item, &format!("{path}.value[{i}]"))?);
    }
    Ok(out)
}

fn validate_cost_bounds_opt(
    opt: Option<&JsonValue>,
    path: &str,
) -> Result<Option<CostBounds>, RuleRejection> {
    let Some(val) = opt else {
        return Ok(None);
    };
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(
            RuleId::ClaimMinimumUnbounded,
            path,
            "cost_bounds must be an object",
        )
    })?;
    check_closed_keys(
        entries,
        &[
            "attained",
            "bound",
            "candidate",
            "gap",
            "incumbent",
            "lower_bound",
            "lower_bound_proof",
            "strict",
            "unit",
        ],
        path,
    )?;
    let attained = val
        .get("attained")
        .and_then(JsonValue::as_bool)
        .ok_or_else(|| {
            RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.attained"),
                "cost_bounds requires boolean 'attained' (R3)",
            )
        })?;
    let strict = match val.get("strict") {
        None => false,
        Some(v) => v.as_bool().ok_or_else(|| {
            RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("{path}.strict"),
                "'strict' must be boolean",
            )
        })?,
    };
    let unit = req_non_empty_str(val, "unit", &format!("{path}.unit"))?.to_string();

    if attained {
        if strict {
            return Err(RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                path,
                "an open/strict threshold cannot be claimed as attained (R3, CX-114)",
            ));
        }
        let inc_val = val.get("incumbent").ok_or_else(|| {
            RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.incumbent"),
                "attained minimum requires 'incumbent' value (R3)",
            )
        })?;
        let lb_val = val.get("lower_bound").ok_or_else(|| {
            RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.lower_bound"),
                "attained minimum requires a checked matching 'lower_bound' (R3)",
            )
        })?;
        let incumbent = parse_rational_value(inc_val, &format!("{path}.incumbent"))?;
        let lower_bound = parse_rational_value(lb_val, &format!("{path}.lower_bound"))?;
        if incumbent != lower_bound {
            return Err(RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                path,
                "attained minimum incumbent does not equal lower_bound (R3)",
            ));
        }
        let gap = match val.get("gap") {
            None => Some(Rational::zero()),
            Some(gv) => {
                let g = parse_rational_value(gv, &format!("{path}.gap"))?;
                if !g.is_zero() {
                    return Err(RuleRejection::new(
                        RuleId::ClaimMinimumUnbounded,
                        format!("{path}.gap"),
                        "attained minimum must have zero gap",
                    ));
                }
                Some(g)
            }
        };
        Ok(Some(CostBounds {
            attained: true,
            strict: false,
            unit,
            incumbent: Some(incumbent),
            lower_bound: Some(lower_bound),
            bound: None,
            candidate: None,
            gap,
        }))
    } else {
        if !strict {
            return Err(RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.strict"),
                "unattained open threshold must set strict=true (R3, CX-114)",
            ));
        }
        let bound_val = val.get("bound").ok_or_else(|| {
            RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.bound"),
                "open threshold requires 'bound' (R3)",
            )
        })?;
        let cand_val = val.get("candidate").ok_or_else(|| {
            RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.candidate"),
                "open threshold requires a feasible 'candidate' (R3)",
            )
        })?;
        let gap_val = val.get("gap").ok_or_else(|| {
            RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.gap"),
                "open threshold requires a positive 'gap' (R3)",
            )
        })?;
        let bound = parse_rational_value(bound_val, &format!("{path}.bound"))?;
        let candidate = parse_rational_value(cand_val, &format!("{path}.candidate"))?;
        let gap = parse_rational_value(gap_val, &format!("{path}.gap"))?;
        if !gap.is_positive() {
            return Err(RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.gap"),
                "open threshold gap must be strictly positive (R3)",
            ));
        }
        let diff = bound
            .sub(&candidate)
            .map_err(|e| RuleRejection::new(RuleId::LimitsExceeded, format!("{path}.gap"), e.to_string()))?
            .abs();
        if diff != gap {
            return Err(RuleRejection::new(
                RuleId::ClaimMinimumUnbounded,
                format!("{path}.gap"),
                "open threshold gap does not equal |bound - candidate| (R3)",
            ));
        }
        Ok(Some(CostBounds {
            attained: false,
            strict: true,
            unit,
            incumbent: None,
            lower_bound: None,
            bound: Some(bound),
            candidate: Some(candidate),
            gap: Some(gap),
        }))
    }
}

fn validate_unresolved_mass_opt(
    opt: Option<&JsonValue>,
    path: &str,
) -> Result<Option<UnresolvedMass>, RuleRejection> {
    let Some(val) = opt else {
        return Ok(None);
    };
    let entries = val.as_obj().ok_or_else(|| {
        RuleRejection::new(
            RuleId::ClaimMassWithoutWeighting,
            path,
            "unresolved_mass must be an object declaring a caller-supplied 'weighting' and 'value' (R4)",
        )
    })?;
    check_closed_keys(entries, &["value", "weighting"], path)?;
    let weighting = val.get("weighting").ok_or_else(|| {
        RuleRejection::new(
            RuleId::ClaimMassWithoutWeighting,
            format!("{path}.weighting"),
            "unresolved_mass requires a supplied 'weighting' (R4)",
        )
    })?;
    let weighting_ok = match weighting {
        JsonValue::Str(s) => !s.trim().is_empty(),
        JsonValue::Obj(m) => !m.is_empty(),
        _ => false,
    };
    if !weighting_ok {
        return Err(RuleRejection::new(
            RuleId::ClaimMassWithoutWeighting,
            format!("{path}.weighting"),
            "unresolved_mass weighting must be a non-empty string or object (R4)",
        ));
    }
    let value = val.get("value").ok_or_else(|| {
        RuleRejection::new(
            RuleId::SchemaMissingField,
            format!("{path}.value"),
            "unresolved_mass requires 'value'",
        )
    })?;
    if value.as_obj().is_none() {
        let r = parse_rational_value(value, &format!("{path}.value"))?;
        if r.is_negative() || r > Rational::one() {
            return Err(RuleRejection::new(
                RuleId::SchemaFieldType,
                format!("{path}.value"),
                "unresolved_mass probability value must lie in [0, 1]",
            ));
        }
    }
    Ok(Some(UnresolvedMass {
        weighting: weighting.clone(),
        value: value.clone(),
    }))
}
