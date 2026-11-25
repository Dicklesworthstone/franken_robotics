//! Owned JSON value tree used for verb results, schema documents and the reader (N32a-03).
//!
//! Built only from bytes admitted by `frob_core::json::admit` (strict RFC 8259, duplicate
//! members refused) and written back only through `frob_core::canon` (sorted members,
//! minimal escapes, numbers as `int53` when exact and as shortest binary64 otherwise).
//! Number lexemes are kept verbatim until they are written. Determinism D0; prints nothing.

use frob_core::canon::{self, CanonError, Canonical, CanonicalBytes, Value};
use frob_core::json::{AdmissionFault, Json, Limits, admit};

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
            (Self::Num(_), Self::Num(_)) => match (self.as_f64(), other.as_f64()) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            },
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
            Self::Num(lexeme) => {
                let node = Json::Num(lexeme.as_str());
                if let Ok(n) = node.as_int53() {
                    w.int53(n)
                } else {
                    let (_, x) = node
                        .as_f64_lexeme()
                        .map_err(|_| CanonError::NonFiniteFloat)?;
                    w.f64_shortest(x)
                }
            }
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
