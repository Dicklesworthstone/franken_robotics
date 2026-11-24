//! Compiled resource and size limits for `frob-cert` (BD-026).
//!
//! Every rejection caused by exceeding one of these limits returns rule
//! `limits.exceeded`; the table is printed in reports so any rejection for size
//! can be reproduced.

/// Default and compiled auditor limits (DESIGN, BD-026 item 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// Maximum bit length of any arbitrary-precision integer operand or result.
    pub max_bigint_bits: usize,
    /// Maximum number of numeric entries in one payload.
    pub max_numeric_entries: usize,
    /// Maximum nesting depth of JSON arrays/objects.
    pub max_json_depth: usize,
    /// Maximum bundle file size in bytes (64 MiB).
    pub max_file_bytes: usize,
}

/// The compiled limit table of `frob-cert`.
pub const COMPILED_LIMITS: Limits = Limits {
    max_bigint_bits: 16_384,
    max_numeric_entries: 1_000_000,
    max_json_depth: 64,
    max_file_bytes: 64 * 1024 * 1024,
};

/// Maximum JSON integer magnitude (`2^53 - 1`), exact across JSON/IEEE-754 integers.
pub const MAX_JSON_INT: i64 = (1_i64 << 53) - 1;
/// Minimum JSON integer value (`-(2^53 - 1)`).
pub const MIN_JSON_INT: i64 = -MAX_JSON_INT;

/// Stable rule identifiers emitted at the M0 floor of `frob-cert`.
pub const RULE_LIMITS_EXCEEDED: &str = "limits.exceeded";
pub const RULE_INPUT_UNREADABLE: &str = "input.unreadable";
pub const RULE_JSON_SYNTAX: &str = "json.syntax";
pub const RULE_JSON_BOM: &str = "json.bom";
pub const RULE_JSON_DUPLICATE_KEY: &str = "json.duplicate-key";
pub const RULE_JSON_KEY_ORDER: &str = "json.key-order";
pub const RULE_JSON_WHITESPACE: &str = "json.whitespace";
pub const RULE_JSON_ESCAPE: &str = "json.escape";
pub const RULE_NUM_FLOAT_IN_BUNDLE: &str = "num.float-in-bundle";
pub const RULE_NUM_INT_RANGE: &str = "num.int-range";
pub const RULE_NUM_INT_FORM: &str = "num.int-form";
pub const RULE_NUM_RATIONAL_FORM: &str = "num.rational-form";
pub const RULE_KIND_UNKNOWN: &str = "kind.unknown";

/// Stable refusal code for malformed certificate arithmetic (BD-026 item 2).
pub const REFUSAL_ARITHMETIC_MALFORMED: &str = "FROB-REF-849";

impl Limits {
    /// Format the compiled limit table as a stable human-readable summary line.
    pub fn summary_line(&self) -> String {
        format!(
            "limits: max_bigint_bits={} max_numeric_entries={} max_json_depth={} max_file_bytes={}",
            self.max_bigint_bits,
            self.max_numeric_entries,
            self.max_json_depth,
            self.max_file_bytes,
        )
    }
}
