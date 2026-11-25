//! Schema identifiers and `schemas/` file-path mapping (bead N32a-02, rule RL-05).
//!
//! Grammar: `^frob(\.[a-z0-9-]+)+/[1-9][0-9]*$` (with major version `u32 >= 1`).
//! File mapping: replace `/` with `.` and append `.json` under `schemas/`, e.g.
//! `frob.design.check.in/1` → `schemas/frob.design.check.in.1.json`.

use std::fmt;
use std::str::FromStr;

/// Error returned when parsing a [`SchemaId`] or [`OwnedSchemaId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaIdError {
    pub input: String,
    pub reason: &'static str,
}

impl fmt::Display for SchemaIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid schema identifier {:?}: {}",
            self.input, self.reason
        )
    }
}

impl std::error::Error for SchemaIdError {}

/// Static schema identifier stored in generated `VERBS` and `SCHEMAS` tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaId(pub &'static str);

impl SchemaId {
    /// Borrow the raw schema identifier string (`frob.<...>/<major>`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Return the schema's file name (`<id with / replaced by .>.json`).
    #[must_use]
    pub fn file_name(self) -> String {
        schema_file_name(self.0)
    }

    /// Return the repository-relative schema path (`schemas/<file_name>`).
    #[must_use]
    pub fn relative_path(self) -> String {
        schema_relative_path(self.0)
    }

    /// Parse the trailing major version integer if well-formed.
    #[must_use]
    pub fn major(self) -> Option<u32> {
        parse_schema_id_major(self.0).ok()
    }
}

impl fmt::Display for SchemaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Validated owned schema identifier matching `^frob(\.[a-z0-9-]+)+/[1-9][0-9]*$`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedSchemaId {
    raw: String,
    major: u32,
}

impl OwnedSchemaId {
    /// Validate and construct an owned schema identifier.
    pub fn parse(s: &str) -> Result<Self, SchemaIdError> {
        let major = parse_schema_id_major(s)?;
        Ok(Self {
            raw: s.to_owned(),
            major,
        })
    }

    /// Borrow the canonical schema identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Return the parsed major version (`>= 1`).
    #[must_use]
    pub const fn major(&self) -> u32 {
        self.major
    }

    /// Return `<id with / replaced by .>.json`.
    #[must_use]
    pub fn file_name(&self) -> String {
        schema_file_name(&self.raw)
    }

    /// Return `schemas/<file_name>`.
    #[must_use]
    pub fn relative_path(&self) -> String {
        schema_relative_path(&self.raw)
    }
}

impl fmt::Display for OwnedSchemaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.raw)
    }
}

impl FromStr for OwnedSchemaId {
    type Err = SchemaIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// Validate `s` against `^frob(\.[a-z0-9-]+)+/[1-9][0-9]*$` and return its major version.
pub fn parse_schema_id_major(s: &str) -> Result<u32, SchemaIdError> {
    let err = |reason: &'static str| SchemaIdError {
        input: s.to_owned(),
        reason,
    };

    let Some(rest) = s.strip_prefix("frob.") else {
        return Err(err("schema identifier must start with `frob.`"));
    };
    let Some((path, ver_str)) = rest.split_once('/') else {
        return Err(err("schema identifier must end with `/<major>`"));
    };
    if ver_str.contains('/') {
        return Err(err("schema identifier may contain only one `/`"));
    }
    if path.is_empty() {
        return Err(err(
            "schema identifier must contain at least one segment after `frob.`",
        ));
    }
    for seg in path.split('.') {
        if seg.is_empty() {
            return Err(err("empty segment in schema identifier"));
        }
        if !seg
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(err(
                "schema identifier segments may contain only `[a-z0-9-]`",
            ));
        }
    }
    if ver_str.is_empty()
        || !ver_str.bytes().all(|b| b.is_ascii_digit())
        || (ver_str.len() > 1 && ver_str.starts_with('0'))
    {
        return Err(err(
            "schema major version must be a canonical positive decimal integer",
        ));
    }
    let major: u32 = ver_str
        .parse()
        .map_err(|_| err("schema major version out of u32 range"))?;
    if major == 0 {
        return Err(err("schema major version must be >= 1"));
    }
    Ok(major)
}

/// Convert a schema identifier to its file name (`/` replaced by `.`, plus `.json`).
#[must_use]
pub fn schema_file_name(id: &str) -> String {
    format!("{}.json", id.replace('/', "."))
}

/// Convert a schema identifier to its repository-relative path under `schemas/`.
#[must_use]
pub fn schema_relative_path(id: &str) -> String {
    format!("schemas/{}", schema_file_name(id))
}
