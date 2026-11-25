//! Stack identities on results: resolution receipt hash (`sr:`), stack-lock digest (`sl:`),
//! build-time embedded [`BUILT_UNDER`], FrankenSim build-identity mode
//! ([`BUILD_IDENTITY_MODE`] and [`build_identity`]), same-stack predicate,
//! `FROB-REF-890` refusal constructor, and the airlock [`StackKeyed`] trait
//! (`N33a-06`, `N33a-08`, §33.2, §33.3, §33.5, §33.16).
//!
//! Determinism class D0; prints nothing (L-7); zero `unsafe`; canonical JSON via
//! [`crate::canon`].

#[cfg(all(
    feature = "nc-stack-compare-lock",
    any(not(feature = "gauntlet-mutations"), not(debug_assertions))
))]
compile_error!(
    "`nc-stack-compare-lock` requires a debug build with `--features gauntlet-mutations`"
);

#[path = "stack_build_identity.rs"]
pub mod build_identity;

pub use build_identity::BUILD_IDENTITY_MODE;

use std::fmt;
use std::str::FromStr;

use crate::canon::{self, CanonError, Canonical, Object, Value};
use crate::id::{Blake3, CanonicalBytes, IdError};
use crate::refusal::{Refusal, RefusalCode, Repair, RetryPolicy};

/// BLAKE3 hash of the canonical resolution receipt (`frob.stack-receipt/1`), printed as
/// `sr:<64 lower-case hex digits>`.
///
/// Distinct Rust type from [`StackLockDigest`] with no constructor from an arbitrary string
/// outside [`Self::parse`]:
///
/// ```compile_fail,E0308
/// use frob_core::stack::{BUILT_UNDER, StackLockDigest};
/// let _wrong: StackLockDigest = BUILT_UNDER.receipt;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackReceiptHash([u8; 32]);

impl StackReceiptHash {
    /// Required text prefix for resolution receipt hashes.
    pub const PREFIX: &'static str = "sr:";

    /// All-zero receipt hash (used only with `dev: true` when `receipts/HEAD` is absent).
    pub const ZERO: Self = Self([0u8; 32]);

    /// Construct from 32 raw digest bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Construct from a [`Blake3`] digest.
    #[must_use]
    pub const fn from_digest(digest: Blake3) -> Self {
        Self(*digest.as_bytes())
    }

    /// Borrow the 32 raw bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Return the underlying [`Blake3`] digest.
    #[must_use]
    pub const fn digest(self) -> Blake3 {
        Blake3::from_bytes(self.0)
    }

    /// Render as 64 bare lower-case hex digits (without the `sr:` prefix).
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.digest().to_hex()
    }

    /// Parse `sr:<64 lower-case hex digits>`. Rejects `sl:` and bare or malformed hex.
    pub fn parse(s: &str) -> Result<Self, IdError> {
        let hex = s.strip_prefix(Self::PREFIX).ok_or_else(|| IdError::InvalidPrefix {
            expected: Self::PREFIX,
            input: s.to_owned(),
        })?;
        Ok(Self::from_digest(Blake3::from_hex(hex)?))
    }

    pub(crate) const fn from_const_hex(hex: &str) -> Self {
        Self(parse_const_hex_32(hex))
    }
}

impl fmt::Display for StackReceiptHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.digest())
    }
}

impl fmt::Debug for StackReceiptHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StackReceiptHash({}{})", Self::PREFIX, self.digest())
    }
}

impl FromStr for StackReceiptHash {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for StackReceiptHash {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.str(&self.to_string());
        Ok(())
    }
}

/// BLAKE3 digest over the canonical bytes of `franken-stack.lock` (`frob.stack-lock/1`),
/// printed as `sl:<64 lower-case hex digits>`.
///
/// Distinct Rust type from [`StackReceiptHash`] with no constructor from an arbitrary string
/// outside [`Self::parse`]:
///
/// ```compile_fail,E0308
/// use frob_core::stack::{BUILT_UNDER, StackReceiptHash};
/// let _wrong: StackReceiptHash = BUILT_UNDER.lock;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackLockDigest([u8; 32]);

impl StackLockDigest {
    /// Required text prefix for stack-lock digests.
    pub const PREFIX: &'static str = "sl:";

    /// All-zero lock digest (used only with `dev: true` when `receipts/LOCK_DIGEST` is absent).
    pub const ZERO: Self = Self([0u8; 32]);

    /// Construct from 32 raw digest bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Construct from a [`Blake3`] digest.
    #[must_use]
    pub const fn from_digest(digest: Blake3) -> Self {
        Self(*digest.as_bytes())
    }

    /// Borrow the 32 raw bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Return the underlying [`Blake3`] digest.
    #[must_use]
    pub const fn digest(self) -> Blake3 {
        Blake3::from_bytes(self.0)
    }

    /// Render as 64 bare lower-case hex digits (without the `sl:` prefix).
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.digest().to_hex()
    }

    /// Parse `sl:<64 lower-case hex digits>`. Rejects `sr:` and bare or malformed hex.
    pub fn parse(s: &str) -> Result<Self, IdError> {
        let hex = s.strip_prefix(Self::PREFIX).ok_or_else(|| IdError::InvalidPrefix {
            expected: Self::PREFIX,
            input: s.to_owned(),
        })?;
        Ok(Self::from_digest(Blake3::from_hex(hex)?))
    }

    pub(crate) const fn from_const_hex(hex: &str) -> Self {
        Self(parse_const_hex_32(hex))
    }
}

impl fmt::Display for StackLockDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.digest())
    }
}

impl fmt::Debug for StackLockDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StackLockDigest({}{})", Self::PREFIX, self.digest())
    }
}

impl FromStr for StackLockDigest {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Canonical for StackLockDigest {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.str(&self.to_string());
        Ok(())
    }
}

/// Complete stack identity embedded in every FrankenRobotics binary and result (`N33a-06`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackIdentity {
    /// Wide identity: hash of the resolved `frob.stack-receipt/1` JSON.
    pub receipt: StackReceiptHash,
    /// Narrow identity: digest of canonical `franken-stack.lock` (referee airlock cache key).
    pub lock: StackLockDigest,
    /// Development build flag (`FROB_STACK_DEV=1`): never compares same-stack, refused by gate PASS.
    pub dev: bool,
}

impl StackIdentity {
    /// Construct a stack identity from its typed components.
    #[must_use]
    pub const fn new(receipt: StackReceiptHash, lock: StackLockDigest, dev: bool) -> Self {
        Self { receipt, lock, dev }
    }

    /// Serialize this identity to canonical JSON bytes via [`crate::canon::to_canonical`].
    pub fn to_canonical_json(&self) -> Result<CanonicalBytes, CanonError> {
        canon::to_canonical(self)
    }

    /// Write the envelope `evidence` stack fields (`stack_receipt`, and `stack_dev: true`
    /// only when `self.dev` is set) into a canonical JSON object (`N33a-06`, `N32a-03`).
    pub fn write_envelope_evidence_fields(&self, obj: &mut Object<'_>) -> Result<(), CanonError> {
        if self.dev {
            obj.bool("stack_dev", true)?;
        }
        obj.member_term("stack_receipt", &self.receipt)?;
        Ok(())
    }
}

impl Canonical for StackIdentity {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.bool("dev", self.dev)?;
            o.member_term("lock", &self.lock)?;
            o.member_term("receipt", &self.receipt)?;
            Ok(())
        })
    }
}

/// Compile-time stack identity embedded by `crates/frob-core/build.rs` from `receipts/HEAD`,
/// `receipts/LOCK_DIGEST`, and `FROB_STACK_DEV`.
pub const BUILT_UNDER: StackIdentity = StackIdentity {
    receipt: StackReceiptHash::from_const_hex(env!("FROB_STACK_RECEIPT_HEX")),
    lock: StackLockDigest::from_const_hex(env!("FROB_STACK_LOCK_HEX")),
    dev: const_str_eq(env!("FROB_STACK_DEV_FLAG"), "true"),
};

/// Same-stack predicate (`N33a-06`, INV-SUITE-05).
///
/// Returns `true` iff neither identity is marked `dev` and both carry the exact same
/// resolution receipt hash. Equal lock digests alone are never sufficient.
#[must_use]
pub const fn same_stack(a: &StackIdentity, b: &StackIdentity) -> bool {
    #[cfg(feature = "nc-stack-compare-lock")]
    {
        !a.dev && !b.dev && bytes_eq(&a.lock.0, &b.lock.0)
    }
    #[cfg(not(feature = "nc-stack-compare-lock"))]
    {
        !a.dev && !b.dev && bytes_eq(&a.receipt.0, &b.receipt.0)
    }
}

/// Enforce the same-stack rule (`FROB-REF-890`, exit 3, `retry: after-change`) whenever
/// two results are compared, pooled, substituted, promoted, or served from a cache.
pub fn require_same_stack(
    a: &StackIdentity,
    b: &StackIdentity,
    what: &str,
) -> Result<(), Refusal> {
    if same_stack(a, b) {
        return Ok(());
    }

    let reason = if a.dev && b.dev {
        format!("cannot treat `{what}` as same-stack evidence: both stacks are dev builds")
    } else if a.dev {
        format!("cannot treat `{what}` as same-stack evidence: observed stack is a dev build")
    } else if b.dev {
        format!("cannot treat `{what}` as same-stack evidence: required stack is a dev build")
    } else {
        format!("cannot treat `{what}` as same-stack evidence: resolution receipt hashes differ")
    };

    let repairs = vec![
        Repair::with_detail(
            "result.revalidate",
            "Revalidate the older evidence under the current stack",
        ),
        Repair::with_detail(
            "rerun",
            "Re-run the computation under the current stack",
        ),
        Repair::with_detail(
            "present-separately",
            "Present the two as separate results, each labelled with its stack",
        ),
    ];

    let refusal = Refusal::new(
        RefusalCode::FrobRef890,
        reason,
        "/evidence/stack_receipt",
        a.receipt.to_string(),
        b.receipt.to_string(),
        "stack-receipt",
        RetryPolicy::AfterChange,
        repairs,
    )
    .with_detail(format!(
        "what={what}; observed_dev={}; required_dev={}; observed_lock={}; required_lock={}",
        a.dev, b.dev, a.lock, b.lock
    ));

    Err(refusal)
}

/// Serialize a [`Refusal`] to [`CanonicalBytes`] using [`crate::canon`] (`N34-18`, `N33a-06`).
pub fn refusal_to_canonical_json(refusal: &Refusal) -> Result<CanonicalBytes, CanonError> {
    canon::to_canonical(&CanonicalRefusal(refusal))
}

struct CanonicalRefusal<'a>(&'a Refusal);

impl Canonical for CanonicalRefusal<'_> {
    fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
        let r = self.0;
        w.object(|o| {
            o.array("affected_claims", |a| {
                for c in &r.affected_claims {
                    a.push_str(c);
                }
                Ok(())
            })?;
            o.str("code", r.code.as_str())?;
            if let Some(ref d) = r.detail {
                o.str("detail", d)?;
            }
            o.str("message", &r.message)?;
            o.str("observed", &r.observed)?;
            o.str("pointer", &r.pointer)?;
            o.array("repairs", |a| {
                for rep in &r.repairs {
                    a.push_object(|ro| {
                        ro.str("action", &rep.action)?;
                        if let Some(ref rd) = rep.detail {
                            ro.str("detail", rd)?;
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            o.str("required", &r.required)?;
            o.str("retry", r.retry.as_str())?;
            o.str("unit", &r.unit)?;
            Ok(())
        })
    }
}

/// Key-component trait implemented by airlock adapters (`frob-fsim`) so every memoized
/// referee result includes the stack-lock digest in its cache key (`N33a-06`, §33.3).
pub trait StackKeyed {
    /// Stack-lock digest component mixed into the derived memo key.
    fn stack_key_component(&self) -> StackLockDigest;
}

impl StackKeyed for StackIdentity {
    fn stack_key_component(&self) -> StackLockDigest {
        self.lock
    }
}

impl StackKeyed for StackLockDigest {
    fn stack_key_component(&self) -> StackLockDigest {
        *self
    }
}

const fn bytes_eq(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut i = 0;
    while i < 32 {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn const_str_eq(a: &str, b: &str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    if ab.len() != bb.len() {
        return false;
    }
    let mut i = 0;
    while i < ab.len() {
        if ab[i] != bb[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn const_hex_nibble(b: u8) -> u8 {
    assert!(
        matches!(b, b'0'..=b'9' | b'a'..=b'f'),
        "build.rs must emit 64 lower-case hex digits"
    );
    if b <= b'9' {
        b - b'0'
    } else {
        b - b'a' + 10
    }
}

const fn parse_const_hex_32(s: &str) -> [u8; 32] {
    let raw = s.as_bytes();
    assert!(raw.len() == 64, "build.rs must emit 64 hex digits");
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        let hi = const_hex_nibble(raw[i * 2]);
        let lo = const_hex_nibble(raw[i * 2 + 1]);
        out[i] = (hi << 4) | lo;
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const HEX_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HEX_B: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    const LOCK_A: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    fn id(receipt_hex: &str, lock_hex: &str, dev: bool) -> StackIdentity {
        StackIdentity {
            receipt: StackReceiptHash::parse(&format!("sr:{receipt_hex}")).unwrap(),
            lock: StackLockDigest::parse(&format!("sl:{lock_hex}")).unwrap(),
            dev,
        }
    }

    #[test]
    fn same_stack_reflexive_on_release_identity() {
        let a = id(HEX_A, LOCK_A, false);
        assert!(same_stack(&a, &a));
        assert!(require_same_stack(&a, &a, "comparison").is_ok());
    }

    #[test]
    fn same_stack_false_on_different_receipt() {
        // Equal lock digests, receipts differing in one hex digit: must be false.
        // Fails under `--features nc-stack-compare-lock` negative control.
        let a = id(HEX_A, LOCK_A, false);
        let b = id(HEX_B, LOCK_A, false);
        assert_eq!(a.lock, b.lock);
        assert_ne!(a.receipt, b.receipt);
        assert!(!same_stack(&a, &b));
        assert!(require_same_stack(&a, &b, "paired-arm").is_err());
    }

    #[test]
    fn same_stack_false_when_dev() {
        let rel = id(HEX_A, LOCK_A, false);
        let dev = id(HEX_A, LOCK_A, true);
        assert!(!same_stack(&rel, &dev));
        assert!(!same_stack(&dev, &rel));
        assert!(!same_stack(&dev, &dev));
        assert_eq!(
            require_same_stack(&dev, &rel, "memo-hit").unwrap_err().code,
            RefusalCode::FrobRef890
        );
        assert_eq!(
            require_same_stack(&rel, &dev, "memo-hit").unwrap_err().code,
            RefusalCode::FrobRef890
        );
    }

    #[test]
    fn require_same_stack_refuses_890() {
        let a = id(HEX_A, LOCK_A, false);
        let b = id(HEX_B, LOCK_A, false);
        let refusal = require_same_stack(&a, &b, "ledger-pool").unwrap_err();

        assert_eq!(refusal.code, RefusalCode::FrobRef890);
        assert_eq!(refusal.code.number(), 890);
        assert_eq!(refusal.observed, format!("sr:{HEX_A}"));
        assert_eq!(refusal.required, format!("sr:{HEX_B}"));
        assert_eq!(refusal.retry, RetryPolicy::AfterChange);
        assert_eq!(refusal.repairs.len(), 3);
        assert_eq!(refusal.repairs[0].action, "result.revalidate");
        assert_eq!(refusal.repairs[1].action, "rerun");
        assert_eq!(refusal.repairs[2].action, "present-separately");

        // Canonical serialization via frob_core::canon matches Refusal::to_json and roundtrips.
        let canon_bytes = refusal_to_canonical_json(&refusal).expect("canonical refusal");
        assert_eq!(canon_bytes.as_str(), refusal.to_json());
        let parsed = Refusal::from_json(canon_bytes.as_str()).expect("roundtrip refusal");
        assert_eq!(parsed, refusal);
    }

    #[test]
    fn text_form_roundtrip() {
        let sr_text = format!("sr:{HEX_A}");
        let sl_text = format!("sl:{LOCK_A}");
        let sr = StackReceiptHash::parse(&sr_text).expect("valid sr:");
        let sl = StackLockDigest::parse(&sl_text).expect("valid sl:");
        assert_eq!(sr.to_string(), sr_text);
        assert_eq!(sl.to_string(), sl_text);
        assert_eq!(sr.to_hex(), HEX_A);
        assert_eq!(sl.to_hex(), LOCK_A);
        assert_eq!(sr_text.parse::<StackReceiptHash>().unwrap(), sr);
        assert_eq!(sl_text.parse::<StackLockDigest>().unwrap(), sl);

        // 63-digit string refused.
        let short_sr = format!("sr:{}", &HEX_A[..63]);
        let short_sl = format!("sl:{}", &LOCK_A[..63]);
        assert!(StackReceiptHash::parse(&short_sr).is_err());
        assert!(StackLockDigest::parse(&short_sl).is_err());

        // Cross-prefix and bare hex refused.
        assert!(StackReceiptHash::parse(&sl_text).is_err());
        assert!(StackLockDigest::parse(&sr_text).is_err());
        assert!(StackReceiptHash::parse(HEX_A).is_err());
        assert!(StackLockDigest::parse(LOCK_A).is_err());

        // Uppercase hex refused.
        assert!(StackReceiptHash::parse(&format!("sr:{}", HEX_A.to_ascii_uppercase())).is_err());
        assert!(StackLockDigest::parse(&format!("sl:{}", LOCK_A.to_ascii_uppercase())).is_err());
    }

    #[test]
    fn stack_identity_canonical_json_via_canon() {
        let a = id(HEX_A, LOCK_A, false);
        let bytes = a.to_canonical_json().expect("canon stack identity");
        assert_eq!(
            bytes.as_str(),
            format!("{{\"dev\":false,\"lock\":\"sl:{LOCK_A}\",\"receipt\":\"sr:{HEX_A}\"}}")
        );
        assert_eq!(a.stack_key_component(), a.lock);
        assert_eq!(a.lock.stack_key_component(), a.lock);

        let env_clean = canon::to_canonical(&EnvHelper(a)).expect("env clean");
        assert_eq!(env_clean.as_str(), format!("{{\"stack_receipt\":\"sr:{HEX_A}\"}}"));
        let env_dev = canon::to_canonical(&EnvHelper(id(HEX_A, LOCK_A, true))).expect("env dev");
        assert_eq!(
            env_dev.as_str(),
            format!("{{\"stack_dev\":true,\"stack_receipt\":\"sr:{HEX_A}\"}}")
        );
    }

    struct EnvHelper(StackIdentity);
    impl Canonical for EnvHelper {
        fn write(&self, w: Value<'_>) -> Result<(), CanonError> {
            w.object(|o| self.0.write_envelope_evidence_fields(o))
        }
    }

    #[test]
    fn built_under_matches_committed_receipt_files() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("workspace root");
        let head = std::fs::read_to_string(root.join("receipts/HEAD"))
            .expect("receipts/HEAD present")
            .trim()
            .to_owned();
        let lock = std::fs::read_to_string(root.join("receipts/LOCK_DIGEST"))
            .expect("receipts/LOCK_DIGEST present")
            .trim()
            .to_owned();
        assert_eq!(BUILT_UNDER.receipt.to_hex(), head);
        assert_eq!(BUILT_UNDER.lock.to_hex(), lock);
    }
}
