//! Content-addressed identities and domain-separated BLAKE3 hashing (BD-005).
//!
//! Exactly two hashing entry points exist in `frob-core`: [`hash_blob`] (plain BLAKE3)
//! and [`identity`] (BLAKE3 derive-key mode with [`SchemaName`] as context and
//! [`CanonicalBytes`] as key material). Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdError {
    InvalidHex { input: String },
    InvalidPrefix { expected: &'static str, input: String },
    InvalidSchemaName { input: String, reason: &'static str },
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHex { input } => write!(f, "invalid BLAKE3 digest `{input}`: expected 64 lower-case hex digits"),
            Self::InvalidPrefix { expected, input } => write!(f, "invalid identifier prefix in `{input}`: expected `{expected}`"),
            Self::InvalidSchemaName { input, reason } => write!(f, "invalid schema name `{input}`: {reason}"),
        }
    }
}
impl std::error::Error for IdError {}

/// A 32-byte BLAKE3 digest rendered as 64 lower-case hexadecimal digits.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blake3([u8; Self::LEN]);

impl Blake3 {
    pub const LEN: usize = 32;
    #[must_use] pub const fn from_bytes(bytes: [u8; Self::LEN]) -> Self { Self(bytes) }
    #[must_use] pub const fn as_bytes(&self) -> &[u8; Self::LEN] { &self.0 }

    #[must_use]
    pub fn to_hex(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut out = String::with_capacity(64);
        for &b in &self.0 {
            out.push(HEX[usize::from(b >> 4)] as char);
            out.push(HEX[usize::from(b & 0x0f)] as char);
        }
        out
    }

    pub fn from_hex(s: &str) -> Result<Self, IdError> {
        let raw = s.as_bytes();
        if raw.len() != 64 { return Err(IdError::InvalidHex { input: s.to_owned() }); }
        let mut out = [0u8; Self::LEN];
        for (i, dst) in out.iter_mut().enumerate() {
            let hi = decode_lower_hex(raw[i * 2]).ok_or_else(|| IdError::InvalidHex { input: s.to_owned() })?;
            let lo = decode_lower_hex(raw[i * 2 + 1]).ok_or_else(|| IdError::InvalidHex { input: s.to_owned() })?;
            *dst = (hi << 4) | lo;
        }
        Ok(Self(out))
    }
}

const fn decode_lower_hex(b: u8) -> Option<u8> {
    match b { b'0'..=b'9' => Some(b - b'0'), b'a'..=b'f' => Some(b - b'a' + 10), _ => None }
}

impl fmt::Display for Blake3 { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.to_hex()) } }
impl fmt::Debug for Blake3 { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Blake3({})", self.to_hex()) } }
impl FromStr for Blake3 { type Err = IdError; fn from_str(s: &str) -> Result<Self, Self::Err> { Self::from_hex(s) } }

/// Validated schema name printing as `frob.<name>/<major>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaName { name: String, major: u32 }

impl SchemaName {
    pub fn new(name: impl Into<String>, major: u32) -> Result<Self, IdError> {
        let name = name.into();
        if major == 0 {
            return Err(IdError::InvalidSchemaName { input: format!("frob.{name}/{major}"), reason: "major version must be >= 1" });
        }
        validate_schema_ident(&name)?;
        Ok(Self { name, major })
    }

    pub fn parse(s: &str) -> Result<Self, IdError> {
        let rest = s.strip_prefix("frob.").ok_or_else(|| IdError::InvalidSchemaName { input: s.to_owned(), reason: "must start with `frob.`" })?;
        let (name, major_s) = rest.split_once('/').ok_or_else(|| IdError::InvalidSchemaName { input: s.to_owned(), reason: "missing `/<major>`" })?;
        if major_s.is_empty() || !major_s.bytes().all(|b| b.is_ascii_digit()) || (major_s.len() > 1 && major_s.starts_with('0')) {
            return Err(IdError::InvalidSchemaName { input: s.to_owned(), reason: "major must be canonical positive decimal" });
        }
        let major: u32 = major_s.parse().map_err(|_| IdError::InvalidSchemaName { input: s.to_owned(), reason: "major out of range" })?;
        Self::new(name, major)
    }

    #[must_use] pub fn name(&self) -> &str { &self.name }
    #[must_use] pub const fn major(&self) -> u32 { self.major }
    #[must_use] pub fn domain_string(&self) -> String { format!("frob.{}/{}", self.name, self.major) }
}

fn validate_schema_ident(name: &str) -> Result<(), IdError> {
    let err = |reason: &'static str| IdError::InvalidSchemaName { input: name.to_owned(), reason };
    let b = name.as_bytes();
    if b.is_empty() { return Err(err("empty schema name")); }
    if !b[0].is_ascii_lowercase() { return Err(err("must start with lower-case ASCII")); }
    if matches!(b[b.len() - 1], b'.' | b'-') { return Err(err("cannot end with `.` or `-`")); }
    let mut prev = 0u8;
    for &c in b {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'.' || c == b'-') {
            return Err(err("only lower-case ASCII, digits, `.` and `-` permitted"));
        }
        if c == b'.' && prev == b'.' { return Err(err("cannot contain `..`")); }
        prev = c;
    }
    Ok(())
}

impl fmt::Display for SchemaName { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "frob.{}/{}", self.name, self.major) } }
impl FromStr for SchemaName { type Err = IdError; fn from_str(s: &str) -> Result<Self, Self::Err> { Self::parse(s) } }

/// Opaque canonical byte sequence.
///
/// Exactly two producers construct `CanonicalBytes`: [`crate::canon::CanonWriter::finish`]
/// and [`crate::json::Admitted::into_canonical`]. No public struct literal or `From<Vec<u8>>`
/// conversion exists.
///
/// ```compile_fail,E0451
/// use frob_core::id::CanonicalBytes;
/// let _ = CanonicalBytes { bytes: vec![1, 2, 3] };
/// ```
///
/// ```compile_fail,E0277
/// use frob_core::id::CanonicalBytes;
/// let _: CanonicalBytes = Vec::<u8>::new().into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalBytes { bytes: Vec<u8> }

impl CanonicalBytes {
    #[must_use] pub fn as_bytes(&self) -> &[u8] { &self.bytes }
    #[must_use] pub fn as_str(&self) -> &str {
        match std::str::from_utf8(&self.bytes) {
            Ok(s) => s,
            Err(_) => "",
        }
    }
    #[must_use] pub fn into_bytes(self) -> Vec<u8> { self.bytes }
    #[must_use] pub fn len(&self) -> usize { self.bytes.len() }
    #[must_use] pub fn is_empty(&self) -> bool { self.bytes.is_empty() }
    pub(crate) const fn from_canonical_unchecked(bytes: Vec<u8>) -> Self { Self { bytes } }
    #[cfg(test)] pub(crate) fn from_test_bytes(bytes: Vec<u8>) -> Self { Self { bytes } }
}

/// Plain BLAKE3 hash of raw blob bytes (matches standard BLAKE3 tools).
#[must_use]
pub fn hash_blob(bytes: &[u8]) -> Blake3 {
    Blake3::from_bytes(*fs_blake3::hash_bytes(bytes).as_bytes())
}

/// Domain-separated artifact identity: BLAKE3 derive-key mode under `schema` (`frob.<name>/<major>`).
#[must_use]
pub fn identity(schema: &SchemaName, canonical: &CanonicalBytes) -> Blake3 {
    Blake3::from_bytes(*fs_blake3::hash_domain(&schema.domain_string(), canonical.as_bytes()).as_bytes())
}

macro_rules! define_typed_id {
    ($($name:ident => $prefix:literal),+ $(,)?) => {$(
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(Blake3);
        impl $name {
            pub const PREFIX: &'static str = $prefix;
            #[must_use] pub const fn from_digest(digest: Blake3) -> Self { Self(digest) }
            #[must_use] pub const fn digest(self) -> Blake3 { self.0 }
            pub fn parse(s: &str) -> Result<Self, IdError> {
                let hex = s.strip_prefix(Self::PREFIX).ok_or_else(|| IdError::InvalidPrefix { expected: Self::PREFIX, input: s.to_owned() })?;
                Ok(Self(Blake3::from_hex(hex)?))
            }
        }
        impl fmt::Display for $name { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}{}", Self::PREFIX, self.0) } }
        impl fmt::Debug for $name { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}({}{})", stringify!($name), Self::PREFIX, self.0) } }
        impl FromStr for $name { type Err = IdError; fn from_str(s: &str) -> Result<Self, Self::Err> { Self::parse(s) } }
    )+};
}

define_typed_id! { DesignId => "d:", ModelId => "m:", SceneId => "s:", KernelId => "k:", PolicyId => "p:", EpisodeId => "e:" }

/// Closed union of the six prefixed artifact identifiers owned by `frob-core`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypedId { Design(DesignId), Model(ModelId), Scene(SceneId), Kernel(KernelId), Policy(PolicyId), Episode(EpisodeId) }

impl TypedId {
    pub fn parse(s: &str) -> Result<Self, IdError> {
        if s.starts_with(DesignId::PREFIX) { Ok(Self::Design(DesignId::parse(s)?)) }
        else if s.starts_with(ModelId::PREFIX) { Ok(Self::Model(ModelId::parse(s)?)) }
        else if s.starts_with(SceneId::PREFIX) { Ok(Self::Scene(SceneId::parse(s)?)) }
        else if s.starts_with(KernelId::PREFIX) { Ok(Self::Kernel(KernelId::parse(s)?)) }
        else if s.starts_with(PolicyId::PREFIX) { Ok(Self::Policy(PolicyId::parse(s)?)) }
        else if s.starts_with(EpisodeId::PREFIX) { Ok(Self::Episode(EpisodeId::parse(s)?)) }
        else { Err(IdError::InvalidPrefix { expected: "one of d:, m:, s:, k:, p:, e:", input: s.to_owned() }) }
    }
}

impl fmt::Display for TypedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Design(x) => fmt::Display::fmt(x, f), Self::Model(x) => fmt::Display::fmt(x, f),
            Self::Scene(x) => fmt::Display::fmt(x, f), Self::Kernel(x) => fmt::Display::fmt(x, f),
            Self::Policy(x) => fmt::Display::fmt(x, f), Self::Episode(x) => fmt::Display::fmt(x, f),
        }
    }
}
impl FromStr for TypedId { type Err = IdError; fn from_str(s: &str) -> Result<Self, Self::Err> { Self::parse(s) } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blake3_empty_blob() {
        assert_eq!(hash_blob(b"").to_hex(), "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262");
    }

    #[test]
    fn identity_differs_across_schema_names() {
        let (s1, s2, s3) = (SchemaName::new("design", 1).unwrap(), SchemaName::new("model", 1).unwrap(), SchemaName::new("design", 2).unwrap());
        let canon = CanonicalBytes::from_test_bytes(br#"{"a":1,"b":2}"#.to_vec());
        let (h1, h2, h3, blob) = (identity(&s1, &canon), identity(&s2, &canon), identity(&s3, &canon), hash_blob(canon.as_bytes()));
        assert_ne!(h1, h2); assert_ne!(h1, h3); assert_ne!(h2, h3); assert_ne!(h1, blob);
        assert_eq!(h1, identity(&s1, &canon));
    }

    #[test]
    fn schema_and_hex_validation() {
        let s = SchemaName::parse("frob.design-v1.card/1").unwrap();
        assert_eq!(s.to_string(), "frob.design-v1.card/1");
        for bad in ["design/1", "frob.Design/1", "frob.design/0", "frob.design/01", "frob./1", "frob.1d/1", "frob.a..b/1", "frob.a-/1", "frob.a_b/1"] {
            assert!(SchemaName::parse(bad).is_err(), "expected error for `{bad}`");
        }
        let d = hash_blob(b"lower");
        assert_eq!(Blake3::from_hex(&d.to_hex()).unwrap(), d);
        assert!(Blake3::from_hex(&d.to_hex().to_ascii_uppercase()).is_err());
        assert!(Blake3::from_hex(&d.to_hex()[..63]).is_err());
    }

    #[test]
    fn id_text_roundtrip() {
        let mut st: u64 = 0xBD00_51D0_2026_1124;
        let mut next = || { st = st.wrapping_add(0x9E37_79B9_7F4A_7C15); let mut z = st; z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9); z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB); z ^ (z >> 31) };
        for _ in 0..256 {
            let mut bytes = [0u8; 32];
            for c in bytes.chunks_exact_mut(8) { c.copy_from_slice(&next().to_le_bytes()); }
            let h = Blake3::from_bytes(bytes);
            let (d, m, s, k, p, e) = (DesignId::from_digest(h), ModelId::from_digest(h), SceneId::from_digest(h), KernelId::from_digest(h), PolicyId::from_digest(h), EpisodeId::from_digest(h));
            assert_eq!(DesignId::parse(&d.to_string()).unwrap(), d); assert_eq!(ModelId::parse(&m.to_string()).unwrap(), m);
            assert_eq!(SceneId::parse(&s.to_string()).unwrap(), s); assert_eq!(KernelId::parse(&k.to_string()).unwrap(), k);
            assert_eq!(PolicyId::parse(&p.to_string()).unwrap(), p); assert_eq!(EpisodeId::parse(&e.to_string()).unwrap(), e);
            assert_eq!(TypedId::parse(&d.to_string()).unwrap(), TypedId::Design(d));
            assert_eq!(TypedId::parse(&m.to_string()).unwrap(), TypedId::Model(m));
            assert_eq!(TypedId::parse(&s.to_string()).unwrap(), TypedId::Scene(s));
            assert_eq!(TypedId::parse(&k.to_string()).unwrap(), TypedId::Kernel(k));
            assert_eq!(TypedId::parse(&p.to_string()).unwrap(), TypedId::Policy(p));
            assert_eq!(TypedId::parse(&e.to_string()).unwrap(), TypedId::Episode(e));
            assert!(DesignId::parse(&m.to_string()).is_err()); assert!(ModelId::parse(&s.to_string()).is_err());
            assert!(SceneId::parse(&k.to_string()).is_err()); assert!(KernelId::parse(&p.to_string()).is_err());
            assert!(PolicyId::parse(&e.to_string()).is_err()); assert!(EpisodeId::parse(&d.to_string()).is_err());
            assert!(TypedId::parse(&format!("x:{h}")).is_err());
            assert!(TypedId::parse(&format!("clm:{h}")).is_err());
            assert!(TypedId::parse(&h.to_hex()).is_err());
        }
    }
}
