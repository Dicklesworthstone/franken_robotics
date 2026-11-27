//! Ten-fold harness structured refusals (bead `N39a-01`, `N39-REF` shape).
//!
//! Lives inside the scorer boundary (`BOUNDARY_FILES`): imports strictly `std`
//! and `frob_cert`. Zero `unsafe`, prints nothing (rule L-7), determinism D0.

use std::fmt;

use frob_cert::json::{JsonValue, serialize_canonical};

/// Exit code for structural/schema/argument refusals.
pub const EXIT_REFUSAL_INPUT: u8 = 3;

/// Exit code for integrity refusals (`record-hash-mismatch`, `store-overwrite`).
pub const EXIT_REFUSAL_INTEGRITY: u8 = 9;

/// Closed harness refusal reasons defined in `N39a-01` (bound to `N39-REF`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RefusalReason {
    RecordUnknownField,
    RecordMissingField,
    RecordDuplicateKey,
    RecordNoncanonicalNumber,
    RecordUnknownSchema,
    RecordHashMismatch,
    StoreOverwrite,
}

impl RefusalReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordUnknownField => "record-unknown-field",
            Self::RecordMissingField => "record-missing-field",
            Self::RecordDuplicateKey => "record-duplicate-key",
            Self::RecordNoncanonicalNumber => "record-noncanonical-number",
            Self::RecordUnknownSchema => "record-unknown-schema",
            Self::RecordHashMismatch => "record-hash-mismatch",
            Self::StoreOverwrite => "store-overwrite",
        }
    }

    #[must_use]
    pub const fn exit_code(self) -> u8 {
        match self {
            Self::RecordUnknownField
            | Self::RecordMissingField
            | Self::RecordDuplicateKey
            | Self::RecordNoncanonicalNumber
            | Self::RecordUnknownSchema => EXIT_REFUSAL_INPUT,
            Self::RecordHashMismatch | Self::StoreOverwrite => EXIT_REFUSAL_INTEGRITY,
        }
    }
}

impl fmt::Display for RefusalReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Structured refusal payload emitted in the `N39-REF` JSON envelope:
/// `{"refusal":{"observed":...,"pointer":...,"reason":...,"repairs":[...],"required":...}}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenfoldRefusal {
    pub reason: RefusalReason,
    pub pointer: String,
    pub observed: String,
    pub required: String,
    pub repairs: Vec<String>,
}

impl TenfoldRefusal {
    #[must_use]
    pub fn new(
        reason: RefusalReason,
        pointer: impl Into<String>,
        observed: impl Into<String>,
        required: impl Into<String>,
        repairs: Vec<String>,
    ) -> Self {
        Self {
            reason,
            pointer: pointer.into(),
            observed: observed.into(),
            required: required.into(),
            repairs,
        }
    }

    #[must_use]
    pub const fn exit_code(&self) -> u8 {
        self.reason.exit_code()
    }

    #[must_use]
    pub fn unknown_field(
        pointer: impl Into<String>,
        observed: impl Into<String>,
        required: impl Into<String>,
    ) -> Self {
        Self::new(
            RefusalReason::RecordUnknownField,
            pointer,
            observed,
            required,
            vec![
                "Remove the unrecognized field or verify its spelling against the record schema"
                    .to_owned(),
            ],
        )
    }

    #[must_use]
    pub fn missing_field(pointer: impl Into<String>, required: impl Into<String>) -> Self {
        Self::new(
            RefusalReason::RecordMissingField,
            pointer,
            "absent",
            required,
            vec!["Supply the required field in the record payload".to_owned()],
        )
    }

    #[must_use]
    pub fn duplicate_key(pointer: impl Into<String>, observed: impl Into<String>) -> Self {
        Self::new(
            RefusalReason::RecordDuplicateKey,
            pointer,
            observed,
            "unique object keys at every nesting level",
            vec!["Remove the duplicate JSON object key before re-submitting".to_owned()],
        )
    }

    #[must_use]
    pub fn noncanonical_number(
        pointer: impl Into<String>,
        observed: impl Into<String>,
        required: impl Into<String>,
        repairs: Vec<String>,
    ) -> Self {
        Self::new(
            RefusalReason::RecordNoncanonicalNumber,
            pointer,
            observed,
            required,
            repairs,
        )
    }

    #[must_use]
    pub fn unknown_schema(
        pointer: impl Into<String>,
        observed: impl Into<String>,
        required: impl Into<String>,
    ) -> Self {
        Self::new(
            RefusalReason::RecordUnknownSchema,
            pointer,
            observed,
            required,
            vec!["Use one of the five registered frob.tenfold.*/1 harness schemas".to_owned()],
        )
    }

    #[must_use]
    pub fn hash_mismatch(
        pointer: impl Into<String>,
        observed: impl Into<String>,
        required: impl Into<String>,
        repairs: Vec<String>,
    ) -> Self {
        Self::new(
            RefusalReason::RecordHashMismatch,
            pointer,
            observed,
            required,
            repairs,
        )
    }

    #[must_use]
    pub fn store_overwrite(
        path: impl Into<String>,
        observed: impl Into<String>,
        required: impl Into<String>,
    ) -> Self {
        Self::new(
            RefusalReason::StoreOverwrite,
            path,
            observed,
            required,
            vec![
                "Never overwrite an existing tenfold store artifact with different bytes"
                    .to_owned(),
            ],
        )
    }

    /// Render the refusal as canonical JSON bytes matching the `N39-REF` shape.
    #[must_use]
    pub fn to_canonical_json(&self) -> String {
        let repairs_arr: Vec<JsonValue> = self
            .repairs
            .iter()
            .map(|r| JsonValue::Str(r.clone()))
            .collect();
        let inner = JsonValue::Obj(vec![
            ("observed".to_owned(), JsonValue::Str(self.observed.clone())),
            ("pointer".to_owned(), JsonValue::Str(self.pointer.clone())),
            (
                "reason".to_owned(),
                JsonValue::Str(self.reason.as_str().to_owned()),
            ),
            ("repairs".to_owned(), JsonValue::Arr(repairs_arr)),
            ("required".to_owned(), JsonValue::Str(self.required.clone())),
        ]);
        let envelope = JsonValue::Obj(vec![("refusal".to_owned(), inner)]);
        serialize_canonical(&envelope)
    }
}

impl fmt::Display for TenfoldRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {} (observed: {}, required: {})",
            self.reason, self.pointer, self.observed, self.required
        )
    }
}

impl std::error::Error for TenfoldRefusal {}
