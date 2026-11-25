// @generated from registries/refusals.toml by frob_core::registry::refusals::render_refusal_code_enum (N34p4-03 check-generated); do not edit by hand.

use std::fmt;

/// Closed refusal code enum generated from `registries/refusals.toml`.
///
/// Every variant corresponds to one registered `[[refusal]]` row; an unregistered
/// code cannot compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum RefusalCode {
    /// FROB-REF-247 (§35.11): Determinism class not attainable on requested placement (e.g. bit-equality replay on a D3 device family, or fork on a family without a current batch-independence record); admission-time check (distinct from snapshot restore across profiles FROB-REF-894); pointer verb and placement; observed attained class and record; required class and use
    FrobRef247 = 247,
    /// FROB-REF-248 (§35.11): Solve ill-conditioned beyond precision profile (kappa_hat * u above frozen OPEN-NUM-4 limit, default 0.1) under refuse policy; run-time measurement (distinct from admission-time interface-card estimate FROB-REF-244); pointer world address and step; observed kappa_hat, lower bound kappa_hat * u, and precision profile; required kappa_hat * u <= limit
    FrobRef248 = 248,
    /// FROB-REF-249 (§35.11): Anchored offset left its range in one step (magnitude >= 16 m after integration) or a cell index left scene bounds; pointer world address, body and axis; observed offset or cell index; required magnitude below 16 m and index inside declared bounds
    FrobRef249 = 249,
    /// FROB-REF-250 (§35.11): Kernel is not dimensionally homogeneous (build time): sum of terms with different base-dimension exponent vectors or square root of an odd exponent vector (theorem card T-09); pointer kernel, statement and both exponent vectors; required equal exponent vectors
    FrobRef250 = 250,
    /// FROB-REF-251 (§35.11): A banned operation appears in kernel source (build time): a fast-math relaxation, a fused multiply-add (contracted or explicit), a built-in transcendental or reciprocal square root, a floating-point atomic; pointer file, line and token; observed ban category
    FrobRef251 = 251,
    /// FROB-REF-252 (§35.11): Strict predicate in floating point without an exact form or certified guard (build time); pointer kernel and comparison; observed predicate class; required exact or guarded
    FrobRef252 = 252,
    /// FROB-REF-561 (§35.11): Paired statistic requested with a D4 or unrecorded arm, or paired across device families without D1, at the numerics licence check (licence::check, N35-07). On eval.compare, pairing::admit first raises FROB-REF-542 for a D4 arm, different classes, or a D3 pair lacking decision equivalence; 561 covers an unrecorded class or an exact pairing the placement cannot license. On eval.detect, 561 covers a D4 or unrecorded arm. On eval.promote, 561 covers a fast-tier result and its re-evaluation paired across device families without D1, or arms that cannot be paired. On study.compare and eval.promote the stored-result evidence boundary raises FROB-REF-825 for D4 or unrecorded evidence; Arena study::admit raises FROB-REF-533. Pointer arm; observed class/profile or both device families; required both arms D2 on one family with current batch-independence record, D1 across families, or D3 with decision-equivalence record
    FrobRef561 = 561,
    /// FROB-REF-562 (§35.11): Attribution or counterfactual fork requested below class D2 (source run class D3, D4 or unrecorded, or device family batch-independence record not current) at fork or re-run admission (distinct from attribution-time fork-set failure FROB-REF-584); pointer source run; observed class and record age; required D2 with GATE-SPARK-15 current for the family
    FrobRef562 = 562,
    /// FROB-REF-701 (§32.18): The estimate's upper end exceeds the remaining session budget at admission (pointer /budget/<unit>; observed upper estimate, required remaining budget; due M2)
    FrobRef701 = 701,
    /// FROB-REF-702 (§32.18): Budget exhausted during a run at a kernel-batch boundary; unpublished state is discarded, nothing partial is published as a result, and the acknowledgement names the last committed tick per world (pointer /budget/<unit>; due M1)
    FrobRef702 = 702,
    /// FROB-REF-703 (§32.18): Admission guard triggered: device utilization >= 0.8 or shape-class queue already holds 64 jobs; carries poll_after_ms (pointer /queue; due M3)
    FrobRef703 = 703,
    /// FROB-REF-704 (§32.18): A campaign-class job has no approved compute envelope for its scope (launcher token, cited envelope_scope, verb-derived scope, or adhoc), raised at daemon submission and at dispatch or resume on lapsed approval (pointer /envelope_scope; due M3)
    FrobRef704 = 704,
    /// FROB-REF-720 (§32.18): The session token lacks a capability scope required by the verb's registry entry, checked before the handler runs so no partial data exists (pointer /session/scopes; due M2)
    FrobRef720 = 720,
    /// FROB-REF-723 (§32.18): Session token invalid, expired, closed, unverifiable without a reachable daemon, missing on a non-public daemon call, or widening attenuation beyond parent or root grant (pointer /session/token; due M2)
    FrobRef723 = 723,
    /// FROB-REF-724 (§32.18): The verb or an explicitly named mechanism is not in this build: reason in {not-linked, feature-not-built, killed: <gate>} with mechanism and declared_tier when a mechanism is named (pointer /verb or /mechanism; due M0)
    FrobRef724 = 724,
    /// FROB-REF-725 (§32.18): A mutate verb targets a mutable name or lineage whose owning namespace is not the session token's namespace (pointer /lineage; due M3)
    FrobRef725 = 725,
    /// FROB-REF-726 (§32.18): Mechanism is built but not admitted for the run's declared purpose, tier, or inputs (mechanism-not-admitted): reason in {island-in-training, tier-not-eligible, purpose-not-eligible, card-provenance}; purpose check precedes build check; carries mechanism, reason, declared_tier, run_purpose, decision ADR path (for purpose-not-eligible), and field/provenance_found/provenances_admitted (for card-provenance) (pointer /mechanism or /actuator_level; due M3, card-provenance M6)
    FrobRef726 = 726,
    /// FROB-REF-761 (§35.11): Job exceeds compute envelope in force for its scope (scope order: frob-launch launcher token, cited --envelope-scope, verb-derived scope e.g. train.family, else adhoc at default E0: 16 GiB, 8 cores, 1 hour, 1 device). Triggers: demand exceeds class or approved per-run/cumulative limit across (1) memory, (2) cores, (3) devices, (4) wall-time interval upper end, (5) cumulative device-hours, or (6) approved scope limit; approved pretraining E3 without written ceiling and kill rule; child run verb exceeding launcher token declaration; cited envelope_scope contradicting launcher token (pointer /envelope_scope; observed cited scope; required token scope; repairs in order: drop the citation so the token scope applies, or run under a launcher of the cited scope). Raised before allocation after FROB-REF-602 and FROB-REF-704 (daemon campaign without approved envelope), before FROB-REF-701 (session budget); pointer limit crossed; observed demand vector and sim.estimate class; required class in force and limits
    FrobRef761 = 761,
    /// FROB-REF-790 (§33.16): This binary was built without the airlock feature group the verb needs
    FrobRef790 = 790,
    /// FROB-REF-801 (§32.18): Unknown verb on the CLI (names no registry entry; MCP answers an unknown tool with JSON-RPC -32602; pointer /verb; due M0)
    FrobRef801 = 801,
    /// FROB-REF-802 (§32.18): Arguments fail the schema, including an unknown parameter or a sync_wait_ms outside 0..=30000 (pointer /sync_wait_ms or offending parameter; nothing charged; due M0)
    FrobRef802 = 802,
    /// FROB-REF-803 (§32.18): Idempotency key reused with different resolved canonical request content for (principal, verb, key); both resolutions are shown (pointer /idempotency_key; due M1)
    FrobRef803 = 803,
    /// FROB-REF-804 (§32.18): Stale parent (expected_parent is not lineage head) or stale field value in design.edit two-value form (set-param <field> <expected> <new>); payload carries head, field, expected, and current (pointer /expected_parent; due M1)
    FrobRef804 = 804,
    /// FROB-REF-805 (§32.18): A mutate verb or job-submitting run verb arrived without an idempotency key, refused before anything is charged (pointer /idempotency_key; due M1)
    FrobRef805 = 805,
    /// FROB-REF-806 (§32.18): Pagination cursor does not match the query digest (object, filters, ordering) or schema version, fails authentication tag verification, or was issued to another principal (pointer /cursor; due M1)
    FrobRef806 = 806,
    /// FROB-REF-807 (§32.18): Pagination cursor expired because the frontier pinned in the cursor has been garbage-collected (pointer /cursor; due M1)
    FrobRef807 = 807,
    /// FROB-REF-808 (§32.18): An integer outside +/-(2^53 - 1) (+/-9007199254740991) in an int53 number-typed input field; input validation only (serializer out-of-range int53 is FROB-REF-990; due M1)
    FrobRef808 = 808,
    /// FROB-REF-809 (§32.18): Plan not found or bound to a superseded identity (design, scene, task, backend, or stack) in study.submit (pointer /plan_id; due M1)
    FrobRef809 = 809,
    /// FROB-REF-810 (§32.18): Chunk conflict: different bytes offered for an already-committed extent; job fails as indeterminate with both content hashes attached (pointer /chunk; due M1)
    FrobRef810 = 810,
    /// FROB-REF-816 (§37.12): An artifact whose authorization flags are missing or not false
    FrobRef816 = 816,
    /// FROB-REF-861 (§35.11): Number observed during a fast-training run (learning curve or training return) cited as evidence, or a policy trained under fast-training offered for a claim without re-evaluation under device-replay or stricter (distinct from frob-learn consumer code FROB-REF-867); pointer cited value and run or policy; observed trained_under: D4; required re-evaluation record for the same PolicyId
    FrobRef861 = 861,
    /// FROB-REF-862 (§35.11): Performance figure without a complete disclosure block. Triggers: (1) required field absent; (2) na used where applicability table forbids it; (3) energy is zero instead of a value or named gap; (4) step rate without its decimation; (5) stale fingerprint digest; (6) ratio block without comparator revision or fewer than ten pairs; (7) parity-defect field missing while a defect is open. Interim trigger: rates violate unit ordering raised with detail = "rates-violate-unit-ordering" and pointer rates.ordering (N36-REF ledger request rates-violate-unit-ordering). Pointer first missing or inadmissible field inside the figure's block; observed missing/inadmissible fields by name; required complete block
    FrobRef862 = 862,
    /// FROB-REF-863 (§35.11): Performance ratio without a paired lower bound. Triggers: (1) ratio stated with no campaign; (2) campaign of fewer than ten pairs; (3) arms whose units or decimations are not comparable; (4) void campaign where an arm failed; (5) threshold judged on mean instead of bound. Pointer ratio field; observed pair count and whether a bound was computed; required at least ten pairs and the one-sided 95% bound exp(l_bar - t * s / sqrt(n))
    FrobRef863 = 863,
    /// FROB-REF-864 (§35.11): Device result cited without a current CPU-versus-device check for its kernels on that device family, driver and compiler (distinct from run-time refusal FROB-REF-605 which refuses to run on the device); pointer result; observed stale or missing check rows with fingerprint digests; required current check for every kernel in the step plan
    FrobRef864 = 864,
    /// FROB-REF-865 (§35.11): Summary statistic that omits what it excluded (worlds marked Degraded or Faulted, unscored pairs, masked samples); pointer statistic; observed inputs counted against denominator; required excluded table with counts by reason
    FrobRef865 = 865,
    /// FROB-REF-866 (§35.11): Speed experiment requested without its computed ceiling, or with a ceiling below its goal. Triggers: (1) campaign of kind optimization or speed-experiment with no ceiling record; (2) ceiling filed after first measurement; (3) ceiling computed on cold or uncached baseline; (4) ceiling below goal; (5) pilot whose falsifier is a speed ratio with no ceiling. Pointer experiment's ceiling field, or goal field when ceiling is below it; observed ceiling (absent or value, when, baseline) and goal; required ceiling present, filed before first measurement, computed on warm cached baseline, and at least the goal
    FrobRef866 = 866,
    /// FROB-REF-881 (§25.24): Evidence or a certificate establishes a different quantifier prefix than the query's, including clairvoyant evidence offered for a robust claim
    FrobRef881 = 881,
    /// FROB-REF-890 (§33.16): Evidence recorded under another stack receipt is offered as same-stack evidence without revalidation
    FrobRef890 = 890,
    /// FROB-REF-891 (§34.9): Format version unsupported (format-version-unsupported): unknown schema major version or unknown required field; pointer is the schema member or JSON Pointer of the unknown field; fixture crates/frob-core/tests/fixtures/workspace_refusals/ref891_major_raised.json (and ref891_unknown_field.json); affected_claims filled per occurrence from the verb's declared outputs (CX-361)
    FrobRef891 = 891,
    /// FROB-REF-892 (§34.9): View is not identity (view-is-not-identity): a registered view, non-canonical JSON, or stored-view reference is offered where an identity is required; pointer is the parameter with observed view:<kind> or non-canonical-spelling; fixture crates/frob-core/tests/fixtures/workspace_refusals/ref892_view_sexp.sexp (and ref892_noncanonical.json); affected_claims filled per occurrence from the verb's declared outputs (CX-360)
    FrobRef892 = 892,
    /// FROB-REF-893 (§34.9): Format owner bypass (format-owner-bypass): a file with a registered extension or JSON with a registered top-level schema reaches a verb outside its owner's reader; pointer is the parameter and file with observed naming the registered format; fixture crates/frob-core/tests/fixtures/workspace_refusals/ref893_registered_ext.fbot (and ref893_registered_schema.json); affected_claims filled per occurrence from the verb's declared outputs (CX-358)
    FrobRef893 = 893,
    /// FROB-REF-990 (§32.4): Internal fault (internal-fault): handler panicked, returned an error mapping to no refusal code, or serializer refused an out-of-range int53; one envelope is still written with pointer naming the verb and observed holding the diagnostic file path; never listed in any verb's refusals (RL-06; due M0)
    FrobRef990 = 990,
}

impl RefusalCode {
    /// All registered refusal codes ordered by family, then numerically.
    pub const ALL: &'static [Self] = &[
        Self::FrobRef247,
        Self::FrobRef248,
        Self::FrobRef249,
        Self::FrobRef250,
        Self::FrobRef251,
        Self::FrobRef252,
        Self::FrobRef561,
        Self::FrobRef562,
        Self::FrobRef701,
        Self::FrobRef702,
        Self::FrobRef703,
        Self::FrobRef704,
        Self::FrobRef720,
        Self::FrobRef723,
        Self::FrobRef724,
        Self::FrobRef725,
        Self::FrobRef726,
        Self::FrobRef761,
        Self::FrobRef790,
        Self::FrobRef801,
        Self::FrobRef802,
        Self::FrobRef803,
        Self::FrobRef804,
        Self::FrobRef805,
        Self::FrobRef806,
        Self::FrobRef807,
        Self::FrobRef808,
        Self::FrobRef809,
        Self::FrobRef810,
        Self::FrobRef816,
        Self::FrobRef861,
        Self::FrobRef862,
        Self::FrobRef863,
        Self::FrobRef864,
        Self::FrobRef865,
        Self::FrobRef866,
        Self::FrobRef881,
        Self::FrobRef890,
        Self::FrobRef891,
        Self::FrobRef892,
        Self::FrobRef893,
        Self::FrobRef990,
    ];

    /// Numeric code matching `FROB-REF-[1-9][0-9]{2,3}`.
    #[must_use]
    pub const fn number(self) -> u16 {
        self as u16
    }

    /// Family digit (`1..=9`).
    #[must_use]
    pub const fn family(self) -> u8 {
        let n = self.number();
        if n >= 1000 { (n / 1000) as u8 } else { (n / 100) as u8 }
    }

    /// Canonical wire spelling `FROB-REF-[1-9][0-9]{2,3}`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrobRef247 => "FROB-REF-247",
            Self::FrobRef248 => "FROB-REF-248",
            Self::FrobRef249 => "FROB-REF-249",
            Self::FrobRef250 => "FROB-REF-250",
            Self::FrobRef251 => "FROB-REF-251",
            Self::FrobRef252 => "FROB-REF-252",
            Self::FrobRef561 => "FROB-REF-561",
            Self::FrobRef562 => "FROB-REF-562",
            Self::FrobRef701 => "FROB-REF-701",
            Self::FrobRef702 => "FROB-REF-702",
            Self::FrobRef703 => "FROB-REF-703",
            Self::FrobRef704 => "FROB-REF-704",
            Self::FrobRef720 => "FROB-REF-720",
            Self::FrobRef723 => "FROB-REF-723",
            Self::FrobRef724 => "FROB-REF-724",
            Self::FrobRef725 => "FROB-REF-725",
            Self::FrobRef726 => "FROB-REF-726",
            Self::FrobRef761 => "FROB-REF-761",
            Self::FrobRef790 => "FROB-REF-790",
            Self::FrobRef801 => "FROB-REF-801",
            Self::FrobRef802 => "FROB-REF-802",
            Self::FrobRef803 => "FROB-REF-803",
            Self::FrobRef804 => "FROB-REF-804",
            Self::FrobRef805 => "FROB-REF-805",
            Self::FrobRef806 => "FROB-REF-806",
            Self::FrobRef807 => "FROB-REF-807",
            Self::FrobRef808 => "FROB-REF-808",
            Self::FrobRef809 => "FROB-REF-809",
            Self::FrobRef810 => "FROB-REF-810",
            Self::FrobRef816 => "FROB-REF-816",
            Self::FrobRef861 => "FROB-REF-861",
            Self::FrobRef862 => "FROB-REF-862",
            Self::FrobRef863 => "FROB-REF-863",
            Self::FrobRef864 => "FROB-REF-864",
            Self::FrobRef865 => "FROB-REF-865",
            Self::FrobRef866 => "FROB-REF-866",
            Self::FrobRef881 => "FROB-REF-881",
            Self::FrobRef890 => "FROB-REF-890",
            Self::FrobRef891 => "FROB-REF-891",
            Self::FrobRef892 => "FROB-REF-892",
            Self::FrobRef893 => "FROB-REF-893",
            Self::FrobRef990 => "FROB-REF-990",
        }
    }

    /// Parse a canonical wire code `FROB-REF-[1-9][0-9]{2,3}`.
    #[must_use]
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "FROB-REF-247" => Some(Self::FrobRef247),
            "FROB-REF-248" => Some(Self::FrobRef248),
            "FROB-REF-249" => Some(Self::FrobRef249),
            "FROB-REF-250" => Some(Self::FrobRef250),
            "FROB-REF-251" => Some(Self::FrobRef251),
            "FROB-REF-252" => Some(Self::FrobRef252),
            "FROB-REF-561" => Some(Self::FrobRef561),
            "FROB-REF-562" => Some(Self::FrobRef562),
            "FROB-REF-701" => Some(Self::FrobRef701),
            "FROB-REF-702" => Some(Self::FrobRef702),
            "FROB-REF-703" => Some(Self::FrobRef703),
            "FROB-REF-704" => Some(Self::FrobRef704),
            "FROB-REF-720" => Some(Self::FrobRef720),
            "FROB-REF-723" => Some(Self::FrobRef723),
            "FROB-REF-724" => Some(Self::FrobRef724),
            "FROB-REF-725" => Some(Self::FrobRef725),
            "FROB-REF-726" => Some(Self::FrobRef726),
            "FROB-REF-761" => Some(Self::FrobRef761),
            "FROB-REF-790" => Some(Self::FrobRef790),
            "FROB-REF-801" => Some(Self::FrobRef801),
            "FROB-REF-802" => Some(Self::FrobRef802),
            "FROB-REF-803" => Some(Self::FrobRef803),
            "FROB-REF-804" => Some(Self::FrobRef804),
            "FROB-REF-805" => Some(Self::FrobRef805),
            "FROB-REF-806" => Some(Self::FrobRef806),
            "FROB-REF-807" => Some(Self::FrobRef807),
            "FROB-REF-808" => Some(Self::FrobRef808),
            "FROB-REF-809" => Some(Self::FrobRef809),
            "FROB-REF-810" => Some(Self::FrobRef810),
            "FROB-REF-816" => Some(Self::FrobRef816),
            "FROB-REF-861" => Some(Self::FrobRef861),
            "FROB-REF-862" => Some(Self::FrobRef862),
            "FROB-REF-863" => Some(Self::FrobRef863),
            "FROB-REF-864" => Some(Self::FrobRef864),
            "FROB-REF-865" => Some(Self::FrobRef865),
            "FROB-REF-866" => Some(Self::FrobRef866),
            "FROB-REF-881" => Some(Self::FrobRef881),
            "FROB-REF-890" => Some(Self::FrobRef890),
            "FROB-REF-891" => Some(Self::FrobRef891),
            "FROB-REF-892" => Some(Self::FrobRef892),
            "FROB-REF-893" => Some(Self::FrobRef893),
            "FROB-REF-990" => Some(Self::FrobRef990),
            _ => None,
        }
    }
}

impl fmt::Display for RefusalCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
