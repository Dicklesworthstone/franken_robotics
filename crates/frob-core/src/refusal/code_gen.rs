// @generated from registries/refusals.toml by frob_core::registry::refusals::render_refusal_code_enum (N34p4-03 check-generated); do not edit by hand.

use std::fmt;

/// Closed refusal code enum generated from `registries/refusals.toml`.
///
/// Every variant corresponds to one registered `[[refusal]]` row; an unregistered
/// code cannot compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum RefusalCode {
    /// FROB-REF-251 (§35.11): A banned operation appears in kernel source (build time): a fast-math relaxation, a fused multiply-add (contracted or explicit), a built-in transcendental or reciprocal square root, a floating-point atomic
    FrobRef251 = 251,
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
    /// FROB-REF-881 (§25.24): Evidence or a certificate establishes a different quantifier prefix than the query's, including clairvoyant evidence offered for a robust claim
    FrobRef881 = 881,
    /// FROB-REF-890 (§33.16): Evidence recorded under another stack receipt is offered as same-stack evidence without revalidation
    FrobRef890 = 890,
    /// FROB-REF-990 (§32.4): Internal fault (internal-fault): handler panicked, returned an error mapping to no refusal code, or serializer refused an out-of-range int53; one envelope is still written with pointer naming the verb and observed holding the diagnostic file path; never listed in any verb's refusals (RL-06; due M0)
    FrobRef990 = 990,
}

impl RefusalCode {
    /// All registered refusal codes in ascending numeric order.
    pub const ALL: &'static [Self] = &[
        Self::FrobRef251,
        Self::FrobRef701,
        Self::FrobRef702,
        Self::FrobRef703,
        Self::FrobRef704,
        Self::FrobRef720,
        Self::FrobRef723,
        Self::FrobRef724,
        Self::FrobRef725,
        Self::FrobRef726,
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
        Self::FrobRef881,
        Self::FrobRef890,
        Self::FrobRef990,
    ];

    /// Numeric code in `100..=999`.
    #[must_use]
    pub const fn number(self) -> u16 {
        self as u16
    }

    /// Family digit (`1..=9`).
    #[must_use]
    pub const fn family(self) -> u8 {
        (self.number() / 100) as u8
    }

    /// Canonical wire spelling `FROB-REF-nnn`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrobRef251 => "FROB-REF-251",
            Self::FrobRef701 => "FROB-REF-701",
            Self::FrobRef702 => "FROB-REF-702",
            Self::FrobRef703 => "FROB-REF-703",
            Self::FrobRef704 => "FROB-REF-704",
            Self::FrobRef720 => "FROB-REF-720",
            Self::FrobRef723 => "FROB-REF-723",
            Self::FrobRef724 => "FROB-REF-724",
            Self::FrobRef725 => "FROB-REF-725",
            Self::FrobRef726 => "FROB-REF-726",
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
            Self::FrobRef881 => "FROB-REF-881",
            Self::FrobRef890 => "FROB-REF-890",
            Self::FrobRef990 => "FROB-REF-990",
        }
    }

    /// Parse a canonical wire code `FROB-REF-nnn`.
    #[must_use]
    pub fn parse(code: &str) -> Option<Self> {
        match code {
            "FROB-REF-251" => Some(Self::FrobRef251),
            "FROB-REF-701" => Some(Self::FrobRef701),
            "FROB-REF-702" => Some(Self::FrobRef702),
            "FROB-REF-703" => Some(Self::FrobRef703),
            "FROB-REF-704" => Some(Self::FrobRef704),
            "FROB-REF-720" => Some(Self::FrobRef720),
            "FROB-REF-723" => Some(Self::FrobRef723),
            "FROB-REF-724" => Some(Self::FrobRef724),
            "FROB-REF-725" => Some(Self::FrobRef725),
            "FROB-REF-726" => Some(Self::FrobRef726),
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
            "FROB-REF-881" => Some(Self::FrobRef881),
            "FROB-REF-890" => Some(Self::FrobRef890),
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
