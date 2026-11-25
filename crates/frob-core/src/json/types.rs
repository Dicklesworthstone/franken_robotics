//! Core types, resource limits, fault taxonomy, envelope identifier validation,
//! and typed number accessors for `frob_core::json` (bead N34-17).

use std::fmt;
use std::ops::Deref;

use crate::canon::CanonError;
use crate::id::CanonicalBytes;

/// Maximum exact integer magnitude in IEEE-754 binary64 (`2^53 - 1`).
pub const INT53_MAX: i64 = 9_007_199_254_740_991;
/// Minimum exact integer in IEEE-754 binary64 (`-(2^53 - 1)`).
pub const INT53_MIN: i64 = -INT53_MAX;
/// Default maximum input size in bytes (64 MiB).
pub const DEFAULT_MAX_BYTES: usize = 64 * 1024 * 1024;
/// Default maximum container nesting depth.
pub const DEFAULT_MAX_DEPTH: u16 = 64;
/// Default maximum number lexeme length in characters.
pub const DEFAULT_MAX_NUMBER_LEN: usize = 1024;

/// Half-open byte span `[start, end)` in the admitted raw input slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Construct a half-open byte span `[start, end)`.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Length of the span in bytes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Return `true` when the span covers zero bytes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

/// Resource bounds checked while scanning raw bytes, before proportional allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_depth: u16,
    pub max_string: usize,
    pub max_number_len: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_bytes: DEFAULT_MAX_BYTES,
            max_depth: DEFAULT_MAX_DEPTH,
            max_string: DEFAULT_MAX_BYTES,
            max_number_len: DEFAULT_MAX_NUMBER_LEN,
        }
    }
}

/// Classification of raw-byte JSON admission, typed-number, and identifier faults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaultKind {
    TooLarge,
    NotUtf8,
    Bom,
    TrailingData,
    ControlInString,
    Syntax,
    NumberForm,
    TooDeep,
    StringTooLong,
    NumberTooLong,
    LoneSurrogate,
    DuplicateMember,
    Int53Range,
    IntegerStringForm,
    NumberNotFinite,
    WrongType,
    IdentifierEmpty,
    IdentifierNotPrintable,
}

impl FaultKind {
    /// Canonical hyphenated fault slug.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::TooLarge => "too-large",
            Self::NotUtf8 => "not-utf8",
            Self::Bom => "bom",
            Self::TrailingData => "trailing-data",
            Self::ControlInString => "control-in-string",
            Self::Syntax => "syntax",
            Self::NumberForm => "number-form",
            Self::TooDeep => "too-deep",
            Self::StringTooLong => "string-too-long",
            Self::NumberTooLong => "number-too-long",
            Self::LoneSurrogate => "lone-surrogate",
            Self::DuplicateMember => "duplicate-member",
            Self::Int53Range => "int53-range",
            Self::IntegerStringForm => "integer-string-form",
            Self::NumberNotFinite => "number-not-finite",
            Self::WrongType => "wrong-type",
            Self::IdentifierEmpty => "identifier-empty",
            Self::IdentifierNotPrintable => "identifier-not-printable",
        }
    }
}

impl fmt::Display for FaultKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.slug())
    }
}

/// Detail attached to [`FaultKind::DuplicateMember`]: decoded member name and both key byte offsets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateMemberInfo {
    pub name: String,
    pub first_offset: usize,
    pub second_offset: usize,
}

/// Deterministic fault returned by JSON admission, typed number decoding, or identifier validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionFault {
    pub kind: FaultKind,
    pub offset: usize,
    pub pointer: String,
    pub message: String,
    pub duplicate: Option<DuplicateMemberInfo>,
    pub code_point: Option<u32>,
}

impl AdmissionFault {
    /// Construct a general admission fault at `offset` and JSON Pointer `pointer`.
    #[must_use]
    pub fn new(
        kind: FaultKind,
        offset: usize,
        pointer: String,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            offset,
            pointer,
            message: message.into(),
            duplicate: None,
            code_point: None,
        }
    }

    /// Construct a `duplicate-member` fault naming the decoded key and both key offsets.
    #[must_use]
    pub fn duplicate_member(
        name: String,
        first_offset: usize,
        second_offset: usize,
        pointer: String,
    ) -> Self {
        let message = format!(
            "duplicate object member {name:?} at byte offsets {first_offset} and {second_offset}"
        );
        Self {
            kind: FaultKind::DuplicateMember,
            offset: second_offset,
            pointer,
            message,
            duplicate: Some(DuplicateMemberInfo {
                name,
                first_offset,
                second_offset,
            }),
            code_point: None,
        }
    }
}

impl fmt::Display for AdmissionFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at byte {} (pointer '{}'): {}",
            self.kind, self.offset, self.pointer, self.message
        )
    }
}

impl std::error::Error for AdmissionFault {}

fn fault_without_pointer(kind: FaultKind, offset: usize, message: &'static str) -> AdmissionFault {
    AdmissionFault::new(kind, offset, String::new(), message)
}

/// One object member in source order, pairing decoded name, key byte span, and spanned value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member<'a> {
    pub name: String,
    pub name_span: Span,
    pub value: Spanned<'a>,
}

/// An admitted JSON value paired with its exact source byte span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<'a> {
    pub span: Span,
    pub value: Json<'a>,
}

impl<'a> Deref for Spanned<'a> {
    type Target = Json<'a>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Spanned<'a> {
    /// Pair a JSON value with its source byte span.
    #[must_use]
    pub const fn new(span: Span, value: Json<'a>) -> Self {
        Self { span, value }
    }

    /// Decode an exact integer in `-(2^53 - 1)..=(2^53 - 1)` (no fraction, no exponent, not `-0`).
    pub fn as_int53(&self) -> Result<i64, AdmissionFault> {
        self.value.as_int53_at(self.span.start)
    }

    /// Decode a canonical unsigned 64-bit integer string matching `^(0|[1-9][0-9]{0,19})$`.
    pub fn as_u64str(&self) -> Result<u64, AdmissionFault> {
        self.value.as_u64str_at(self.span.start)
    }

    /// Decode a canonical signed 64-bit integer string matching `^(0|-?[1-9][0-9]{0,19})$`.
    pub fn as_i64str(&self) -> Result<i64, AdmissionFault> {
        self.value.as_i64str_at(self.span.start)
    }

    /// Return the verbatim number lexeme paired with its correctly rounded finite IEEE-754 `f64`.
    pub fn as_f64_lexeme(&self) -> Result<(&'a str, f64), AdmissionFault> {
        self.value.as_f64_lexeme_at(self.span.start)
    }
}

/// Admitted JSON AST node.
///
/// Numbers keep their source lexeme (`&'a str`) verbatim and are never rounded to floats
/// during admission. Object members preserve source order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Json<'a> {
    Null,
    Bool(bool),
    Num(&'a str),
    Str(String),
    Arr(Vec<Spanned<'a>>),
    Obj(Vec<Member<'a>>),
}

impl<'a> Json<'a> {
    /// Return `true` if this value is JSON `null`.
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Return the boolean payload if this value is a JSON boolean.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    /// Return the verbatim source number lexeme if this value is a JSON number.
    #[must_use]
    pub const fn as_num_lexeme(&self) -> Option<&'a str> {
        if let Self::Num(s) = self {
            Some(*s)
        } else {
            None
        }
    }

    /// Return the decoded UTF-8 string if this value is a JSON string.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        if let Self::Str(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }

    /// Return the element slice if this value is a JSON array.
    #[must_use]
    pub fn as_array(&self) -> Option<&[Spanned<'a>]> {
        if let Self::Arr(items) = self {
            Some(items.as_slice())
        } else {
            None
        }
    }

    /// Return the member slice (in source order) if this value is a JSON object.
    #[must_use]
    pub fn as_object(&self) -> Option<&[Member<'a>]> {
        if let Self::Obj(members) = self {
            Some(members.as_slice())
        } else {
            None
        }
    }

    /// Look up an object member by exact decoded name (by code point, no normalization).
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Spanned<'a>> {
        self.as_object()?
            .iter()
            .find(|member| member.name == name)
            .map(|member| &member.value)
    }

    /// Decode an exact integer in `-(2^53 - 1)..=(2^53 - 1)` (no fraction, no exponent, not `-0`).
    pub fn as_int53(&self) -> Result<i64, AdmissionFault> {
        self.as_int53_at(0)
    }

    pub(crate) fn as_int53_at(&self, offset: usize) -> Result<i64, AdmissionFault> {
        let Self::Num(lexeme) = self else {
            return Err(fault_without_pointer(
                FaultKind::WrongType,
                offset,
                "expected JSON number for int53",
            ));
        };
        if *lexeme == "-0" || lexeme.contains(['.', 'e', 'E']) {
            return Err(fault_without_pointer(
                FaultKind::Int53Range,
                offset,
                "number lexeme is not an int53 integer",
            ));
        }
        let value = lexeme.parse::<i64>().map_err(|_| {
            fault_without_pointer(
                FaultKind::Int53Range,
                offset,
                "integer overflows int53 bounds",
            )
        })?;
        if !(INT53_MIN..=INT53_MAX).contains(&value) {
            return Err(fault_without_pointer(
                FaultKind::Int53Range,
                offset,
                "integer outside -(2^53 - 1)..=(2^53 - 1)",
            ));
        }
        Ok(value)
    }

    /// Decode a canonical unsigned 64-bit integer string matching `^(0|[1-9][0-9]{0,19})$`.
    pub fn as_u64str(&self) -> Result<u64, AdmissionFault> {
        self.as_u64str_at(0)
    }

    pub(crate) fn as_u64str_at(&self, offset: usize) -> Result<u64, AdmissionFault> {
        let Self::Str(s) = self else {
            return Err(fault_without_pointer(
                FaultKind::WrongType,
                offset,
                "expected JSON string for u64str",
            ));
        };
        if !matches_uint_digits(s.as_bytes(), false) {
            return Err(fault_without_pointer(
                FaultKind::IntegerStringForm,
                offset,
                "u64str must match ^(0|[1-9][0-9]{0,19})$",
            ));
        }
        s.parse::<u64>().map_err(|_| {
            fault_without_pointer(
                FaultKind::IntegerStringForm,
                offset,
                "u64str exceeds u64::MAX",
            )
        })
    }

    /// Decode a canonical signed 64-bit integer string matching `^(0|-?[1-9][0-9]{0,19})$`.
    pub fn as_i64str(&self) -> Result<i64, AdmissionFault> {
        self.as_i64str_at(0)
    }

    pub(crate) fn as_i64str_at(&self, offset: usize) -> Result<i64, AdmissionFault> {
        let Self::Str(s) = self else {
            return Err(fault_without_pointer(
                FaultKind::WrongType,
                offset,
                "expected JSON string for i64str",
            ));
        };
        let (negative, digits) = s
            .strip_prefix('-')
            .map_or((false, s.as_str()), |rest| (true, rest));
        if !matches_uint_digits(digits.as_bytes(), negative) {
            return Err(fault_without_pointer(
                FaultKind::IntegerStringForm,
                offset,
                "i64str must match ^(0|-?[1-9][0-9]{0,19})$",
            ));
        }
        s.parse::<i64>().map_err(|_| {
            fault_without_pointer(
                FaultKind::IntegerStringForm,
                offset,
                "i64str outside i64::MIN..=i64::MAX",
            )
        })
    }

    /// Return the verbatim number lexeme paired with its correctly rounded finite IEEE-754 `f64`.
    pub fn as_f64_lexeme(&self) -> Result<(&'a str, f64), AdmissionFault> {
        self.as_f64_lexeme_at(0)
    }

    pub(crate) fn as_f64_lexeme_at(&self, offset: usize) -> Result<(&'a str, f64), AdmissionFault> {
        let Self::Num(lexeme) = self else {
            return Err(fault_without_pointer(
                FaultKind::WrongType,
                offset,
                "expected JSON number for f64 lexeme",
            ));
        };
        let value = lexeme.parse::<f64>().map_err(|_| {
            fault_without_pointer(
                FaultKind::NumberForm,
                offset,
                "failed to parse number lexeme as f64",
            )
        })?;
        if !value.is_finite() {
            return Err(fault_without_pointer(
                FaultKind::NumberNotFinite,
                offset,
                "number lexeme overflows to non-finite IEEE-754 binary64",
            ));
        }
        Ok((*lexeme, value))
    }

    /// Structural value equality preserving object member source order (ignoring spans).
    #[must_use]
    pub fn eq_ordered(&self, other: &Json<'_>) -> bool {
        eq_tree(self, other, false)
    }

    /// Structural value equality ignoring object member order and spans (KA-781 respelling).
    #[must_use]
    pub fn eq_unordered(&self, other: &Json<'_>) -> bool {
        eq_tree(self, other, true)
    }
}

fn matches_uint_digits(bytes: &[u8], negative: bool) -> bool {
    match bytes {
        [b'0'] => !negative,
        [b'1'..=b'9', rest @ ..] => rest.len() <= 19 && rest.iter().all(u8::is_ascii_digit),
        _ => false,
    }
}

fn eq_tree(a: &Json<'_>, b: &Json<'_>, unordered: bool) -> bool {
    match (a, b) {
        (Json::Null, Json::Null) => true,
        (Json::Bool(x), Json::Bool(y)) => x == y,
        (Json::Num(x), Json::Num(y)) => x == y,
        (Json::Str(x), Json::Str(y)) => x == y,
        (Json::Arr(x), Json::Arr(y)) => {
            x.len() == y.len()
                && x.iter()
                    .zip(y)
                    .all(|(l, r)| eq_tree(&l.value, &r.value, unordered))
        }
        (Json::Obj(x), Json::Obj(y)) if x.len() == y.len() => {
            if !unordered {
                return x.iter().zip(y).all(|(l, r)| {
                    l.name == r.name && eq_tree(&l.value.value, &r.value.value, false)
                });
            }
            let mut sx: Vec<&Member<'_>> = x.iter().collect();
            let mut sy: Vec<&Member<'_>> = y.iter().collect();
            sx.sort_by(|l, r| l.name.cmp(&r.name));
            sy.sort_by(|l, r| l.name.cmp(&r.name));
            sx.iter()
                .zip(sy)
                .all(|(l, r)| l.name == r.name && eq_tree(&l.value.value, &r.value.value, true))
        }
        _ => false,
    }
}

/// Admitted JSON tree paired with the untouched input byte slice.
///
/// [`Admitted::into_canonical`] (N34-18) is one of exactly two constructors of
/// [`CanonicalBytes`], accepting the admitted bytes only when they equal their own
/// canonical re-serialization byte-for-byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admitted<'a> {
    root: Spanned<'a>,
    bytes: &'a [u8],
}

impl<'a> Admitted<'a> {
    pub(crate) const fn new(root: Spanned<'a>, bytes: &'a [u8]) -> Self {
        Self { root, bytes }
    }

    /// Root value paired with its exact source byte span.
    #[must_use]
    pub const fn root(&self) -> &Spanned<'a> {
        &self.root
    }

    /// Root JSON value without its outer span wrapper.
    #[must_use]
    pub const fn value(&self) -> &Json<'a> {
        &self.root.value
    }

    /// Source byte span of the root value within [`Self::bytes`].
    #[must_use]
    pub const fn span(&self) -> Span {
        self.root.span
    }

    /// Untouched raw input bytes admitted by [`crate::json::admit`].
    #[must_use]
    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Deconstruct into the spanned root tree and raw input byte slice.
    #[must_use]
    pub fn into_parts(self) -> (Spanned<'a>, &'a [u8]) {
        (self.root, self.bytes)
    }

    /// Verify that the admitted raw bytes are already in canonical JSON form and upgrade
    /// them into [`CanonicalBytes`].
    ///
    /// Re-serializes the admitted tree through [`crate::canon::CanonWriter`] and requires
    /// byte-for-byte equality with [`Self::bytes`]. Any difference in member order,
    /// whitespace, string escaping, or number layout returns [`CanonError::NotCanonical`].
    pub fn into_canonical(self) -> Result<CanonicalBytes, CanonError> {
        #[cfg(feature = "identity-on-raw-bytes")]
        {
            Ok(CanonicalBytes::from_canonical_unchecked(
                self.bytes.to_vec(),
            ))
        }

        #[cfg(not(feature = "identity-on-raw-bytes"))]
        {
            let canonical = crate::canon::write_json(self.value())?;
            let canon_bytes = canonical.as_bytes();
            if canon_bytes == self.bytes {
                return Ok(canonical);
            }
            let first_diff_offset = self
                .bytes
                .iter()
                .zip(canon_bytes.iter())
                .position(|(a, b)| a != b)
                .unwrap_or_else(|| self.bytes.len().min(canon_bytes.len()));
            Err(CanonError::NotCanonical {
                first_diff_offset,
                raw_len: self.bytes.len(),
                canonical_len: canon_bytes.len(),
            })
        }
    }
}

/// Validate an envelope identifier: non-empty string with no code point `< 32` or `127`.
///
/// Every other Unicode scalar sequence is accepted, ensuring identifiers survive a round trip
/// through shell variables without control-character corruption.
pub fn validate_identifier(s: &str) -> Result<(), AdmissionFault> {
    if s.is_empty() {
        return Err(fault_without_pointer(
            FaultKind::IdentifierEmpty,
            0,
            "identifier must not be empty",
        ));
    }
    for (byte_offset, ch) in s.char_indices() {
        let code_point = ch as u32;
        if code_point < 32 || code_point == 127 {
            let mut fault = fault_without_pointer(
                FaultKind::IdentifierNotPrintable,
                byte_offset,
                "identifier contains non-printable code point",
            );
            fault.code_point = Some(code_point);
            return Err(fault);
        }
    }
    Ok(())
}
