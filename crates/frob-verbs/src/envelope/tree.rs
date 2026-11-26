//! Owned JSON value tree used for verb results, schema documents and the reader (N32a-03).
//!
//! Built only from bytes admitted by `frob_core::json::admit` (strict RFC 8259, duplicate
//! members refused) and written back only through `frob_core::canon` (sorted members,
//! minimal escapes, and refusal when canonicalizing an untyped number would change its value).
//! Number lexemes are kept verbatim until they are written. Determinism D0; prints nothing.

use std::cmp::Ordering;

use frob_core::canon::{self, CanonError, Canonical, CanonicalBytes, Value};
use frob_core::json::{AdmissionFault, Json, Limits, admit};

/// Exact decimal comparison without expanding a potentially very large exponent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExactNumber {
    sign: i8,
    digits: String,
    exponent: Vec<i8>,
    point: i64,
}

impl ExactNumber {
    pub(super) fn parse(lexeme: &str) -> Option<Self> {
        let (sign, body) = lexeme.strip_prefix('-').map_or((1, lexeme), |s| (-1, s));
        let (mantissa, exponent) = body.split_once(['e', 'E']).unwrap_or((body, "0"));
        let exponent_sign = if exponent.starts_with('-') { -1 } else { 1 };
        let exponent = exponent.strip_prefix(['-', '+']).unwrap_or(exponent);
        let (integer, fraction) = match mantissa.split_once('.') {
            Some((_, "")) => return None,
            Some(parts) => parts,
            None => (mantissa, ""),
        };
        if integer.is_empty()
            || (integer.len() > 1 && integer.starts_with('0'))
            || exponent.is_empty()
            || !integer
                .bytes()
                .chain(fraction.bytes())
                .chain(exponent.bytes())
                .all(|b| b.is_ascii_digit())
        {
            return None;
        }
        let mantissa = format!("{integer}{fraction}");
        let significant = mantissa.trim_start_matches('0');
        Some(Self {
            sign: if significant.is_empty() { 0 } else { sign },
            digits: significant.trim_end_matches('0').to_owned(),
            exponent: exponent
                .bytes()
                .map(|b| exponent_sign * (b - b'0') as i8)
                .collect(),
            point: i64::try_from(integer.len()).ok()?
                - i64::try_from(mantissa.len() - significant.len()).ok()?,
        })
    }

    pub(super) fn compare(&self, other: &Self) -> Ordering {
        if self.sign == 0 || self.sign != other.sign {
            return self.sign.cmp(&other.sign);
        }
        let width = self.exponent.len().max(other.exponent.len());
        // Subtract aligned exponent digits. Saturation cannot affect the ordering:
        // once the difference is that large, a mantissa's point offset cannot reverse it.
        let delta = self
            .padded_exponent(width)
            .zip(other.padded_exponent(width))
            .fold(0i64, |delta, (a, b)| {
                delta.saturating_mul(10).saturating_add(i64::from(a - b))
            });
        let magnitude = delta
            .cmp(&other.point.saturating_sub(self.point))
            .then_with(|| self.digits.cmp(&other.digits));
        if self.sign < 0 {
            magnitude.reverse()
        } else {
            magnitude
        }
    }

    fn padded_exponent(&self, width: usize) -> impl Iterator<Item = i8> + '_ {
        std::iter::repeat_n(0, width - self.exponent.len()).chain(self.exponent.iter().copied())
    }
}

/// Owned JSON value. Object members keep their source order; the canonical writer sorts
/// them when the tree is serialized.
#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Null,
    Bool(bool),
    /// Verbatim RFC 8259 number lexeme.
    Num(String),
    Str(String),
    Arr(Vec<Tree>),
    Obj(Vec<(String, Tree)>),
}

impl Tree {
    /// Admit `bytes` (strict JSON, duplicate members refused) and convert to an owned tree.
    pub fn parse(bytes: &[u8]) -> Result<Self, AdmissionFault> {
        let admitted = admit(bytes, &Limits::default())?;
        Ok(Self::from_json(admitted.value()))
    }

    /// Convert an admitted JSON node into an owned tree.
    #[must_use]
    pub fn from_json(node: &Json<'_>) -> Self {
        match node {
            Json::Null => Self::Null,
            Json::Bool(b) => Self::Bool(*b),
            Json::Num(lexeme) => Self::Num((*lexeme).to_owned()),
            Json::Str(s) => Self::Str(s.clone()),
            Json::Arr(items) => {
                Self::Arr(items.iter().map(|i| Self::from_json(&i.value)).collect())
            }
            Json::Obj(members) => Self::Obj(
                members
                    .iter()
                    .map(|m| (m.name.clone(), Self::from_json(&m.value.value)))
                    .collect(),
            ),
        }
    }

    /// Serialize through the canonical writer (N34-18).
    pub fn to_canonical(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }

    /// Member lookup by exact name (objects only).
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Self> {
        match self {
            Self::Obj(members) => members.iter().find(|(k, _)| k == name).map(|(_, v)| v),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Exact integer in the `int53` range (no fraction, no exponent).
    #[must_use]
    pub fn as_int53(&self) -> Option<i64> {
        match self {
            Self::Num(lexeme) => Json::Num(lexeme.as_str()).as_int53().ok(),
            _ => None,
        }
    }

    /// Nearest binary64 of a number lexeme (finite values only).
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Num(lexeme) => Json::Num(lexeme.as_str())
                .as_f64_lexeme()
                .ok()
                .map(|(_, x)| x),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Arr(items) => Some(items),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_object(&self) -> Option<&[(String, Self)]> {
        match self {
            Self::Obj(members) => Some(members),
            _ => None,
        }
    }

    /// JSON Schema value equality: numbers compare by value, objects ignore member order.
    #[must_use]
    pub fn same_value(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Num(a), Self::Num(b)) => ExactNumber::parse(a)
                .zip(ExactNumber::parse(b))
                .is_some_and(|(a, b)| a.compare(&b).is_eq()),
            (Self::Str(a), Self::Str(b)) => a == b,
            (Self::Arr(a), Self::Arr(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.same_value(y))
            }
            (Self::Obj(a), Self::Obj(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .all(|(k, v)| b.iter().any(|(k2, v2)| k == k2 && v.same_value(v2)))
            }
            _ => false,
        }
    }

    /// JSON type name of this value (`integer` is reported as `number`).
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "boolean",
            Self::Num(_) => "number",
            Self::Str(_) => "string",
            Self::Arr(_) => "array",
            Self::Obj(_) => "object",
        }
    }
}

impl Canonical for Tree {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        match self {
            Self::Null => {
                w.null();
                Ok(())
            }
            Self::Bool(b) => {
                w.bool(*b);
                Ok(())
            }
            Self::Num(lexeme) => w.term(&Json::Num(lexeme)),
            Self::Str(s) => {
                w.str(s);
                Ok(())
            }
            Self::Arr(items) => w.array(|a| {
                for item in items {
                    a.push_term(item)?;
                }
                Ok(())
            }),
            Self::Obj(members) => w.object(|o| {
                for (name, value) in members {
                    o.member_term(name, value)?;
                }
                Ok(())
            }),
        }
    }
}
