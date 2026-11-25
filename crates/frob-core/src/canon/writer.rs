//! Canonical JSON writer from typed terms with explicit Unicode code-point sorting
//! (bead N34-18).
//!
//! Rules enforced:
//! 1. UTF-8, no BOM, no insignificant whitespace, no trailing newline.
//! 2. Object members are buffered per object, duplicate names are refused with
//!    [`CanonError::DuplicateMember`], and members are emitted sorted explicitly by
//!    Unicode code point (UTF-8 byte order; U+FFFF precedes U+10000).
//! 3. Minimal string escapes only: `\"`, `\\`, and lower-case `\u00xx` for code points
//!    `< 0x20`; U+007F and all non-ASCII scalars are written raw UTF-8.
//! 4. Exact number encodings (`int53`, `u64str`, `i64str`, `decimal`, `rational_pair`,
//!    `rational_slash`, `f64_shortest`).
//! 5. Arrays preserve insertion order.
//! 6. [`CanonWriter::finish`] is one of exactly two constructors of [`CanonicalBytes`]
//!    (the other is [`Admitted::into_canonical`]).
//!
//! Independent from `frob-cert`; std-only; prints nothing (L-7); zero `unsafe`; determinism D0.

use std::collections::BTreeSet;
use std::fmt;
use std::io::Write as _;
use std::marker::PhantomData;

use crate::id::{Blake3, CanonicalBytes, SchemaName, identity};
use crate::json::{Admitted, DEFAULT_MAX_DEPTH, INT53_MAX, INT53_MIN, Json, Spanned};

use super::number::{CanonInt, ExactDecimal, format_f64_shortest_into};

/// Error returned while encoding a typed term or checking canonical JSON bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonError {
    /// Two members in the same JSON object carried the exact same decoded name.
    DuplicateMember { name: String },
    /// Integer passed to `int53` lies outside `-(2^53 - 1)..=(2^53 - 1)`.
    Int53OutOfRange { value: i64 },
    /// Exact decimal string is malformed or exceeds the normalization cap.
    InvalidDecimal { input: String, reason: &'static str },
    /// Exact integer string does not match `^(0|-?[1-9][0-9]*)$`.
    InvalidInteger { input: String, reason: &'static str },
    /// Rational denominator was `<= 0`.
    NonPositiveDenominator,
    /// Rational `(p, q)` was not already reduced (`gcd(|p|, q) != 1`); never silently reduced.
    UnreducedRational,
    /// Binary64 value was `NaN`, `+∞`, or `-∞`.
    NonFiniteFloat,
    /// Shortest binary64 re-parse verification failed to match input bits.
    FloatRoundTripFailed,
    /// The canonical binary64 spelling would change an untyped number's exact decimal value.
    InexactUntypedNumber,
    /// Opening another container would exceed the reader's nesting limit.
    DepthLimit,
    /// A value callback returned `Ok(())` without emitting a value, or `finish` was called
    /// without a completed root value.
    IncompleteValue,
    /// `CanonWriter::value` or `write_term` was called more than once on the same writer.
    WriterAlreadyUsed,
    /// Admitted input bytes did not match their canonical re-serialization byte-for-byte.
    NotCanonical {
        first_diff_offset: usize,
        raw_len: usize,
        canonical_len: usize,
    },
}

impl fmt::Display for CanonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateMember { name } => {
                write!(f, "duplicate canonical JSON object member {name:?}")
            }
            Self::Int53OutOfRange { value } => {
                write!(
                    f,
                    "integer {value} lies outside exact int53 bounds ({INT53_MIN}..={INT53_MAX})"
                )
            }
            Self::InvalidDecimal { input, reason } => {
                write!(f, "invalid exact decimal {input:?}: {reason}")
            }
            Self::InvalidInteger { input, reason } => {
                write!(f, "invalid exact integer {input:?}: {reason}")
            }
            Self::NonPositiveDenominator => {
                f.write_str("rational denominator must be strictly positive (q > 0)")
            }
            Self::UnreducedRational => {
                f.write_str("rational must already be reduced (gcd(|p|, q) == 1)")
            }
            Self::NonFiniteFloat => f.write_str(
                "non-finite f64 (NaN or infinity) is not representable in canonical JSON",
            ),
            Self::FloatRoundTripFailed => {
                f.write_str("f64 shortest decimal failed bit-exact re-parse verification")
            }
            Self::InexactUntypedNumber => {
                f.write_str("canonicalizing this untyped number requires an explicit number policy")
            }
            Self::DepthLimit => write!(
                f,
                "canonical JSON exceeds {DEFAULT_MAX_DEPTH} nested containers"
            ),
            Self::IncompleteValue => {
                f.write_str("canonical writer value was not completely written")
            }
            Self::WriterAlreadyUsed => {
                f.write_str("canonical writer already consumed its single root value")
            }
            Self::NotCanonical {
                first_diff_offset,
                raw_len,
                canonical_len,
            } => write!(
                f,
                "admitted JSON is not canonical (first byte difference at offset {first_diff_offset}; raw len {raw_len}, canonical len {canonical_len})"
            ),
        }
    }
}

impl std::error::Error for CanonError {}

/// A typed term that writes itself into deterministic canonical JSON.
pub trait Canonical {
    /// Emit this term into `w`.
    fn write(&self, w: Value<'_>) -> Result<(), CanonError>;
}

impl<T: Canonical + ?Sized> Canonical for &T {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        (**self).write(w)
    }
}

impl Canonical for bool {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.bool(*self);
        Ok(())
    }
}

impl Canonical for str {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.str(self);
        Ok(())
    }
}

impl Canonical for String {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.str(self);
        Ok(())
    }
}

impl Canonical for ExactDecimal {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.decimal(self);
        Ok(())
    }
}

impl Canonical for Json<'_> {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        write_json_value(self, w)
    }
}

impl Canonical for Spanned<'_> {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        write_json_value(&self.value, w)
    }
}

impl Canonical for Admitted<'_> {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        write_json_value(self.value(), w)
    }
}

/// Deterministic canonical JSON writer producing [`CanonicalBytes`].
#[derive(Debug, Default)]
pub struct CanonWriter {
    buf: Vec<u8>,
    started: bool,
    complete: bool,
}

impl CanonWriter {
    /// Create an empty canonical JSON writer.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buf: Vec::new(),
            started: false,
            complete: false,
        }
    }

    /// Create an empty canonical JSON writer with an initial output buffer capacity in bytes.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            started: false,
            complete: false,
        }
    }

    /// Obtain the single root [`Value`] handle for writing one JSON value.
    pub fn value(&mut self) -> Result<Value<'_>, CanonError> {
        if self.started {
            return Err(CanonError::WriterAlreadyUsed);
        }
        self.started = true;
        Ok(Value {
            buf: &mut self.buf,
            written: &mut self.complete,
            remaining_depth: DEFAULT_MAX_DEPTH,
        })
    }

    /// Write a [`Canonical`] term as the root value of this writer.
    pub fn write_term<T: Canonical + ?Sized>(&mut self, term: &T) -> Result<(), CanonError> {
        self.value()?.term(term)?;
        if !self.complete {
            return Err(CanonError::IncompleteValue);
        }
        Ok(())
    }

    /// Finish writing and yield [`CanonicalBytes`].
    ///
    /// Fails closed with [`CanonError::IncompleteValue`] if no root value completed cleanly.
    pub fn finish(self) -> Result<CanonicalBytes, CanonError> {
        if !self.complete {
            return Err(CanonError::IncompleteValue);
        }
        Ok(CanonicalBytes::from_canonical_unchecked(self.buf))
    }
}

/// Single-use handle for writing exactly one canonical JSON value.
pub struct Value<'w> {
    buf: &'w mut Vec<u8>,
    written: &'w mut bool,
    remaining_depth: u16,
}

impl<'w> Value<'w> {
    /// Write JSON `null`.
    pub fn null(self) {
        self.buf.extend_from_slice(b"null");
        *self.written = true;
    }

    /// Write JSON `true` or `false`.
    pub fn bool(self, b: bool) {
        self.buf
            .extend_from_slice(if b { b"true" } else { b"false" });
        *self.written = true;
    }

    /// Write a JSON string with N34-18 Rule 3 minimal escaping.
    pub fn str(self, s: &str) {
        write_canonical_string(s, self.buf);
        *self.written = true;
    }

    /// Write an exact integer JSON number in `-(2^53 - 1)..=(2^53 - 1)`.
    pub fn int53(self, n: i64) -> Result<(), CanonError> {
        if !(INT53_MIN..=INT53_MAX).contains(&n) {
            return Err(CanonError::Int53OutOfRange { value: n });
        }
        let _ = write!(self.buf, "{n}");
        *self.written = true;
        Ok(())
    }

    /// Write an unsigned 64-bit integer as a canonical base-10 JSON string.
    pub fn u64str(self, n: u64) {
        self.buf.push(b'"');
        let _ = write!(self.buf, "{n}");
        self.buf.push(b'"');
        *self.written = true;
    }

    /// Write a signed 64-bit integer as a canonical base-10 JSON string.
    pub fn i64str(self, n: i64) {
        self.buf.push(b'"');
        let _ = write!(self.buf, "{n}");
        self.buf.push(b'"');
        *self.written = true;
    }

    /// Write an [`ExactDecimal`] as a JSON string holding its shortest exact plain decimal.
    pub fn decimal(self, d: &ExactDecimal) {
        self.str(d.as_str());
    }

    /// Write a reduced rational `(p, q)` (`q > 0`, `gcd(|p|, q) == 1`) as `["p","q"]`.
    ///
    /// Never silently reduces: non-positive denominators and unreduced pairs return an error.
    pub fn rational_pair(self, p: &CanonInt, q: &CanonInt) -> Result<(), CanonError> {
        validate_reduced_rational(p, q)?;
        self.array(|array| {
            array.push_str(&p.to_decimal_string());
            array.push_str(&q.to_decimal_string());
            Ok(())
        })
    }

    /// Write a reduced rational `(p, q)` (`q > 0`, `gcd(|p|, q) == 1`) as `"p/q"`, or `"p"` when `q == 1`.
    ///
    /// Never silently reduces: non-positive denominators and unreduced pairs return an error.
    pub fn rational_slash(self, p: &CanonInt, q: &CanonInt) -> Result<(), CanonError> {
        validate_reduced_rational(p, q)?;
        let p_str = p.to_decimal_string();
        self.buf.push(b'"');
        self.buf.extend_from_slice(p_str.as_bytes());
        if !q.is_one() {
            let q_str = q.to_decimal_string();
            self.buf.push(b'/');
            self.buf.extend_from_slice(q_str.as_bytes());
        }
        self.buf.push(b'"');
        *self.written = true;
        Ok(())
    }

    /// Write a finite IEEE-754 binary64 using its shortest round-trip decimal digits under
    /// N34-18 Rule 4, verified by re-parsing before emission.
    pub fn f64_shortest(self, x: f64) -> Result<(), CanonError> {
        format_f64_shortest_into(x, self.buf)?;
        *self.written = true;
        Ok(())
    }

    fn untyped_number(self, lexeme: &str, x: f64) -> Result<(), CanonError> {
        let start = self.buf.len();
        format_f64_shortest_into(x, self.buf)?;
        let encoded = std::str::from_utf8(&self.buf[start..])
            .map_err(|_| CanonError::FloatRoundTripFailed)?;
        let same_value = lexeme == encoded
            || ExactDecimal::parse(lexeme)
                .ok()
                .zip(ExactDecimal::parse(encoded).ok())
                .is_some_and(|(input, output)| input == output);
        if !same_value {
            self.buf.truncate(start);
            return Err(CanonError::InexactUntypedNumber);
        }
        *self.written = true;
        Ok(())
    }

    /// Write a JSON array preserving element insertion order (N34-18 Rule 5).
    pub fn array(
        self,
        f: impl FnOnce(&mut Array<'_>) -> Result<(), CanonError>,
    ) -> Result<(), CanonError> {
        let remaining_depth = self
            .remaining_depth
            .checked_sub(1)
            .ok_or(CanonError::DepthLimit)?;
        let start = self.buf.len();
        self.buf.push(b'[');
        let mut arr = Array {
            buf: self.buf,
            count: 0,
            remaining_depth,
        };
        if let Err(err) = f(&mut arr) {
            self.buf.truncate(start);
            return Err(err);
        }
        self.buf.push(b']');
        *self.written = true;
        Ok(())
    }

    /// Write a JSON object, buffering members, refusing duplicate names, and emitting
    /// members sorted explicitly by Unicode code point (N34-18 Rule 2).
    pub fn object(
        self,
        f: impl FnOnce(&mut Object<'_>) -> Result<(), CanonError>,
    ) -> Result<(), CanonError> {
        let remaining_depth = self
            .remaining_depth
            .checked_sub(1)
            .ok_or(CanonError::DepthLimit)?;
        let mut obj = Object {
            members: Vec::new(),
            seen: BTreeSet::new(),
            _marker: PhantomData,
            remaining_depth,
        };
        f(&mut obj)?;

        #[cfg(not(any(feature = "writer-no-key-sort", feature = "utf16-sort")))]
        obj.members
            .sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));

        #[cfg(feature = "utf16-sort")]
        obj.members
            .sort_by(|a, b| a.0.encode_utf16().cmp(b.0.encode_utf16()));

        // Under `feature = "writer-no-key-sort"`, members are intentionally left in insertion order.

        self.buf.push(b'{');
        for (idx, (name, value_bytes)) in obj.members.into_iter().enumerate() {
            if idx > 0 {
                self.buf.push(b',');
            }
            write_canonical_string(&name, self.buf);
            self.buf.push(b':');
            self.buf.extend_from_slice(&value_bytes);
        }
        self.buf.push(b'}');
        *self.written = true;
        Ok(())
    }

    /// Write any [`Canonical`] term into this value position.
    pub fn term<T: Canonical + ?Sized>(self, term: &T) -> Result<(), CanonError> {
        let result = term.write(Value {
            buf: self.buf,
            written: &mut *self.written,
            remaining_depth: self.remaining_depth,
        });
        if result.is_err() {
            *self.written = false;
        }
        result
    }
}

/// Ordered canonical JSON array builder (N34-18 Rule 5).
pub struct Array<'w> {
    buf: &'w mut Vec<u8>,
    count: usize,
    remaining_depth: u16,
}

impl<'w> Array<'w> {
    /// Append one element using a [`Value`] closure.
    pub fn push(
        &mut self,
        f: impl FnOnce(Value<'_>) -> Result<(), CanonError>,
    ) -> Result<(), CanonError> {
        let start = self.buf.len();
        if self.count > 0 {
            self.buf.push(b',');
        }
        let mut written = false;
        let res = f(Value {
            buf: self.buf,
            written: &mut written,
            remaining_depth: self.remaining_depth,
        });
        match res {
            Ok(()) if written => {
                self.count += 1;
                Ok(())
            }
            Ok(()) => {
                self.buf.truncate(start);
                Err(CanonError::IncompleteValue)
            }
            Err(err) => {
                self.buf.truncate(start);
                Err(err)
            }
        }
    }

    /// Append one [`Canonical`] term.
    pub fn push_term<T: Canonical + ?Sized>(&mut self, term: &T) -> Result<(), CanonError> {
        self.push(|v| term.write(v))
    }

    /// Append JSON `null`.
    pub fn push_null(&mut self) {
        if self.count > 0 {
            self.buf.push(b',');
        }
        self.buf.extend_from_slice(b"null");
        self.count += 1;
    }

    /// Append JSON boolean.
    pub fn push_bool(&mut self, b: bool) {
        if self.count > 0 {
            self.buf.push(b',');
        }
        self.buf
            .extend_from_slice(if b { b"true" } else { b"false" });
        self.count += 1;
    }

    /// Append a canonical JSON string.
    pub fn push_str(&mut self, s: &str) {
        if self.count > 0 {
            self.buf.push(b',');
        }
        write_canonical_string(s, self.buf);
        self.count += 1;
    }

    /// Append an exact `int53` JSON number.
    pub fn push_int53(&mut self, n: i64) -> Result<(), CanonError> {
        self.push(|v| v.int53(n))
    }

    /// Append a canonical unsigned 64-bit integer string.
    pub fn push_u64str(&mut self, n: u64) {
        if self.count > 0 {
            self.buf.push(b',');
        }
        self.buf.push(b'"');
        let _ = write!(self.buf, "{n}");
        self.buf.push(b'"');
        self.count += 1;
    }

    /// Append a canonical signed 64-bit integer string.
    pub fn push_i64str(&mut self, n: i64) {
        if self.count > 0 {
            self.buf.push(b',');
        }
        self.buf.push(b'"');
        let _ = write!(self.buf, "{n}");
        self.buf.push(b'"');
        self.count += 1;
    }

    /// Append an [`ExactDecimal`] string.
    pub fn push_decimal(&mut self, d: &ExactDecimal) {
        self.push_str(d.as_str());
    }

    /// Append a reduced rational pair `["p","q"]`.
    pub fn push_rational_pair(&mut self, p: &CanonInt, q: &CanonInt) -> Result<(), CanonError> {
        self.push(|v| v.rational_pair(p, q))
    }

    /// Append a reduced rational slash string `"p/q"` or `"p"`.
    pub fn push_rational_slash(&mut self, p: &CanonInt, q: &CanonInt) -> Result<(), CanonError> {
        self.push(|v| v.rational_slash(p, q))
    }

    /// Append a finite binary64 in shortest round-trip canonical form.
    pub fn push_f64_shortest(&mut self, x: f64) -> Result<(), CanonError> {
        self.push(|v| v.f64_shortest(x))
    }

    /// Append a nested JSON array.
    pub fn push_array(
        &mut self,
        f: impl FnOnce(&mut Array<'_>) -> Result<(), CanonError>,
    ) -> Result<(), CanonError> {
        self.push(|v| v.array(f))
    }

    /// Append a nested JSON object.
    pub fn push_object(
        &mut self,
        f: impl FnOnce(&mut Object<'_>) -> Result<(), CanonError>,
    ) -> Result<(), CanonError> {
        self.push(|v| v.object(f))
    }
}

/// Buffered canonical JSON object builder (N34-18 Rule 2).
///
/// Rejects duplicate member names immediately (by exact Unicode code points) and buffers
/// serialized member values until the object closes, where members are sorted explicitly by
/// Unicode code point (UTF-8 byte order).
pub struct Object<'w> {
    members: Vec<(String, Vec<u8>)>,
    seen: BTreeSet<String>,
    _marker: PhantomData<&'w mut Vec<u8>>,
    remaining_depth: u16,
}

impl<'w> Object<'w> {
    /// Insert a member under `name` using a [`Value`] closure.
    ///
    /// Fails with [`CanonError::DuplicateMember`] if `name` was already inserted in this object.
    pub fn member(
        &mut self,
        name: &str,
        f: impl FnOnce(Value<'_>) -> Result<(), CanonError>,
    ) -> Result<(), CanonError> {
        if self.seen.contains(name) {
            return Err(CanonError::DuplicateMember {
                name: name.to_owned(),
            });
        }
        let mut val_buf = Vec::new();
        let mut written = false;
        f(Value {
            buf: &mut val_buf,
            written: &mut written,
            remaining_depth: self.remaining_depth,
        })?;
        if !written {
            return Err(CanonError::IncompleteValue);
        }
        let owned = name.to_owned();
        self.seen.insert(owned.clone());
        self.members.push((owned, val_buf));
        Ok(())
    }

    /// Insert a [`Canonical`] term under `name`.
    pub fn member_term<T: Canonical + ?Sized>(
        &mut self,
        name: &str,
        term: &T,
    ) -> Result<(), CanonError> {
        self.member(name, |v| term.write(v))
    }

    /// Insert a `null` member under `name`.
    pub fn null(&mut self, name: &str) -> Result<(), CanonError> {
        self.member(name, |v| {
            v.null();
            Ok(())
        })
    }

    /// Insert a boolean member under `name`.
    pub fn bool(&mut self, name: &str, b: bool) -> Result<(), CanonError> {
        self.member(name, |v| {
            v.bool(b);
            Ok(())
        })
    }

    /// Insert a string member under `name`.
    pub fn str(&mut self, name: &str, s: &str) -> Result<(), CanonError> {
        self.member(name, |v| {
            v.str(s);
            Ok(())
        })
    }

    /// Insert an `int53` number member under `name`.
    pub fn int53(&mut self, name: &str, n: i64) -> Result<(), CanonError> {
        self.member(name, |v| v.int53(n))
    }

    /// Insert a `u64str` string member under `name`.
    pub fn u64str(&mut self, name: &str, n: u64) -> Result<(), CanonError> {
        self.member(name, |v| {
            v.u64str(n);
            Ok(())
        })
    }

    /// Insert an `i64str` string member under `name`.
    pub fn i64str(&mut self, name: &str, n: i64) -> Result<(), CanonError> {
        self.member(name, |v| {
            v.i64str(n);
            Ok(())
        })
    }

    /// Insert an [`ExactDecimal`] string member under `name`.
    pub fn decimal(&mut self, name: &str, d: &ExactDecimal) -> Result<(), CanonError> {
        self.member(name, |v| {
            v.decimal(d);
            Ok(())
        })
    }

    /// Insert a reduced rational pair `["p","q"]` member under `name`.
    pub fn rational_pair(
        &mut self,
        name: &str,
        p: &CanonInt,
        q: &CanonInt,
    ) -> Result<(), CanonError> {
        self.member(name, |v| v.rational_pair(p, q))
    }

    /// Insert a reduced rational slash string `"p/q"` (or `"p"`) member under `name`.
    pub fn rational_slash(
        &mut self,
        name: &str,
        p: &CanonInt,
        q: &CanonInt,
    ) -> Result<(), CanonError> {
        self.member(name, |v| v.rational_slash(p, q))
    }

    /// Insert a finite `f64_shortest` number member under `name`.
    pub fn f64_shortest(&mut self, name: &str, x: f64) -> Result<(), CanonError> {
        self.member(name, |v| v.f64_shortest(x))
    }

    /// Insert an array member under `name`.
    pub fn array(
        &mut self,
        name: &str,
        f: impl FnOnce(&mut Array<'_>) -> Result<(), CanonError>,
    ) -> Result<(), CanonError> {
        self.member(name, |v| v.array(f))
    }

    /// Insert an object member under `name`.
    pub fn object(
        &mut self,
        name: &str,
        f: impl FnOnce(&mut Object<'_>) -> Result<(), CanonError>,
    ) -> Result<(), CanonError> {
        self.member(name, |v| v.object(f))
    }
}

/// Serialize a [`Canonical`] term into [`CanonicalBytes`].
pub fn to_canonical<T: Canonical + ?Sized>(term: &T) -> Result<CanonicalBytes, CanonError> {
    let mut writer = CanonWriter::new();
    writer.write_term(term)?;
    writer.finish()
}

/// Serialize a [`Canonical`] term into [`CanonicalBytes`] and compute its domain-separated
/// [`Blake3`] identity under `schema`.
pub fn to_canonical_with_identity<T: Canonical + ?Sized>(
    schema: &SchemaName,
    term: &T,
) -> Result<(CanonicalBytes, Blake3), CanonError> {
    let bytes = to_canonical(term)?;
    let digest = identity(schema, &bytes);
    Ok((bytes, digest))
}

/// Compute the domain-separated [`Blake3`] identity of a [`Canonical`] term under `schema`.
pub fn canonical_identity<T: Canonical + ?Sized>(
    schema: &SchemaName,
    term: &T,
) -> Result<Blake3, CanonError> {
    let bytes = to_canonical(term)?;
    Ok(identity(schema, &bytes))
}

/// Serialize an admitted [`Json`] value tree through [`CanonWriter`].
///
/// Refuses a number if canonicalizing it would change its exact decimal value.
pub fn write_json(value: &Json<'_>) -> Result<CanonicalBytes, CanonError> {
    to_canonical(value)
}

/// Serialize an [`Admitted`] JSON document through [`CanonWriter`].
pub fn write_admitted(admitted: &Admitted<'_>) -> Result<CanonicalBytes, CanonError> {
    write_json(admitted.value())
}

fn validate_reduced_rational(p: &CanonInt, q: &CanonInt) -> Result<(), CanonError> {
    if !q.is_positive() {
        return Err(CanonError::NonPositiveDenominator);
    }
    if !p.is_coprime_with(q) {
        return Err(CanonError::UnreducedRational);
    }
    Ok(())
}

fn write_json_value(node: &Json<'_>, w: Value<'_>) -> Result<(), CanonError> {
    match node {
        Json::Null => {
            w.null();
            Ok(())
        }
        Json::Bool(b) => {
            w.bool(*b);
            Ok(())
        }
        Json::Str(s) => {
            w.str(s);
            Ok(())
        }
        Json::Num(_) => {
            if let Ok(n) = node.as_int53() {
                w.int53(n)
            } else {
                let (lexeme, x) = node
                    .as_f64_lexeme()
                    .map_err(|_| CanonError::NonFiniteFloat)?;
                w.untyped_number(lexeme, x)
            }
        }
        Json::Arr(items) => w.array(|a| {
            for item in items {
                a.push(|v| write_json_value(&item.value, v))?;
            }
            Ok(())
        }),
        Json::Obj(members) => w.object(|o| {
            for member in members {
                o.member(&member.name, |v| write_json_value(&member.value.value, v))?;
            }
            Ok(())
        }),
    }
}

/// Write a JSON string under N34-18 Rule 3 (minimal escapes only).
///
/// Escapes only `\"`, `\\`, and lower-case `\u00xx` for code points `< 0x20`.
/// U+007F and all non-ASCII scalars are emitted raw UTF-8 (unless the negative-control
/// plant feature `escape-all-non-ascii` is enabled).
pub(crate) fn write_canonical_string(s: &str, out: &mut Vec<u8>) {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    out.push(b'"');

    #[cfg(not(feature = "escape-all-non-ascii"))]
    {
        let bytes = s.as_bytes();
        let mut run_start = 0usize;
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'"' || b == b'\\' || b < 0x20 {
                if run_start < i {
                    out.extend_from_slice(&bytes[run_start..i]);
                }
                match b {
                    b'"' => out.extend_from_slice(b"\\\""),
                    b'\\' => out.extend_from_slice(b"\\\\"),
                    ctrl => {
                        out.extend_from_slice(&[
                            b'\\',
                            b'u',
                            b'0',
                            b'0',
                            HEX[usize::from(ctrl >> 4)],
                            HEX[usize::from(ctrl & 0x0f)],
                        ]);
                    }
                }
                run_start = i + 1;
            }
        }
        if run_start < bytes.len() {
            out.extend_from_slice(&bytes[run_start..]);
        }
    }

    #[cfg(feature = "escape-all-non-ascii")]
    {
        for ch in s.chars() {
            let cp = ch as u32;
            match ch {
                '"' => out.extend_from_slice(b"\\\""),
                '\\' => out.extend_from_slice(b"\\\\"),
                _ if cp < 0x20 => {
                    let b = cp as u8;
                    out.extend_from_slice(&[
                        b'\\',
                        b'u',
                        b'0',
                        b'0',
                        HEX[usize::from(b >> 4)],
                        HEX[usize::from(b & 0x0f)],
                    ]);
                }
                _ if cp < 0x80 => out.push(cp as u8),
                _ => {
                    let mut utf16 = [0u16; 2];
                    for unit in ch.encode_utf16(&mut utf16).iter().copied() {
                        out.extend_from_slice(&[
                            b'\\',
                            b'u',
                            HEX[usize::from((unit >> 12) & 0x0f)],
                            HEX[usize::from((unit >> 8) & 0x0f)],
                            HEX[usize::from((unit >> 4) & 0x0f)],
                            HEX[usize::from(unit & 0x0f)],
                        ]);
                    }
                }
            }
        }
    }

    out.push(b'"');
}
