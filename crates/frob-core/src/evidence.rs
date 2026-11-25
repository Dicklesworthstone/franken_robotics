//! Evidence classes, determinism classes, physics-tier meets, and the weakest-input-wins
//! [`ClaimProduct`] (BD-005, INV-NUM-06, INV-LEMMA-02).
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::collections::BTreeSet;
use std::fmt;
use std::marker::PhantomData;

use crate::auth::NoAuthorization;
use crate::id::Blake3;
use crate::refusal::{Refusal, RefusalCode, Repair, RetryPolicy};

/// Four evidence classes ordered weakest-first (`Estimated < Tested < Certified < Proved`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceClass { Estimated, Tested, Certified, Proved }

impl EvidenceClass {
    #[must_use] pub const fn meet(self, other: Self) -> Self { if (self as u8) <= (other as u8) { self } else { other } }
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self { Self::Estimated => "Estimated", Self::Tested => "Tested", Self::Certified => "Certified", Self::Proved => "Proved" }
    }
}
impl fmt::Display for EvidenceClass { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) } }

/// Exactly one of the seven claims of §8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClaimKind { Proof, KnownAnswer, Parity, Fidelity, DesignComparison, Reality, Performance }

impl ClaimKind {
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proof => "proof", Self::KnownAnswer => "known-answer", Self::Parity => "parity",
            Self::Fidelity => "fidelity", Self::DesignComparison => "design-comparison",
            Self::Reality => "reality", Self::Performance => "performance",
        }
    }
}
impl fmt::Display for ClaimKind { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) } }

pub enum ProofClaim {}
pub enum KnownAnswerClaim {}
pub enum ParityClaim {}
pub enum FidelityClaim {}
pub enum DesignComparisonClaim {}
pub enum RealityClaim {}
pub enum PerformanceClaim {}

/// Determinism classes ordered weakest-first (`D4 < D3 < D2 < D1 < D0`), so `min` is the meet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DetClass { D4Unspecified, D3Tolerance, D2PerBackend, D1CrossBackend, D0Exact }

impl DetClass {
    #[must_use] pub const fn meet(self, other: Self) -> Self { if (self as u8) <= (other as u8) { self } else { other } }
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self { Self::D4Unspecified => "D4", Self::D3Tolerance => "D3", Self::D2PerBackend => "D2", Self::D1CrossBackend => "D1", Self::D0Exact => "D0" }
    }
    #[must_use] pub fn parse(s: &str) -> Option<Self> {
        match s { "D0" => Some(Self::D0Exact), "D1" => Some(Self::D1CrossBackend), "D2" => Some(Self::D2PerBackend), "D3" => Some(Self::D3Tolerance), "D4" => Some(Self::D4Unspecified), _ => None }
    }
}
impl fmt::Display for DetClass { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) } }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FastTier { BatchCpu, GpuCuda, GpuMetal }

impl FastTier {
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self { Self::BatchCpu => "batch-cpu", Self::GpuCuda => "gpu-cuda", Self::GpuMetal => "gpu-metal" }
    }
}
impl fmt::Display for FastTier { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) } }

/// Physics tier meet lattice: `Static` > `Referee` > mutually incomparable fast tiers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TierMeet { Static, Referee, Fast(BTreeSet<FastTier>) }

impl TierMeet {
    #[must_use] pub fn fast(tier: FastTier) -> Self { let mut s = BTreeSet::new(); s.insert(tier); Self::Fast(s) }
    #[must_use] pub const fn is_provisional(&self) -> bool { matches!(self, Self::Fast(_)) }
    #[must_use]
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Static, x) | (x, Self::Static) => x.clone(),
            (Self::Referee, Self::Referee) => Self::Referee,
            (Self::Referee, Self::Fast(s)) | (Self::Fast(s), Self::Referee) => Self::Fast(s.clone()),
            (Self::Fast(a), Self::Fast(b)) => { let mut m = a.clone(); m.extend(b.iter().copied()); Self::Fast(m) }
        }
    }
}

impl fmt::Display for TierMeet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static => f.write_str("static"),
            Self::Referee => f.write_str("referee"),
            Self::Fast(set) => {
                for (i, t) in set.iter().enumerate() { if i > 0 { f.write_str("+")?; } f.write_str(t.as_str())?; }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Quantifier { ForAll, Exists, Prob }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuantifierBinder { pub quantifier: Quantifier, pub variable: String }

impl QuantifierBinder {
    #[must_use] pub fn for_all(v: impl Into<String>) -> Self { Self { quantifier: Quantifier::ForAll, variable: v.into() } }
    #[must_use] pub fn exists(v: impl Into<String>) -> Self { Self { quantifier: Quantifier::Exists, variable: v.into() } }
    #[must_use] pub fn prob(v: impl Into<String>) -> Self { Self { quantifier: Quantifier::Prob, variable: v.into() } }
}

impl fmt::Display for QuantifierBinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.quantifier { Quantifier::ForAll => "∀", Quantifier::Exists => "∃", Quantifier::Prob => "ℙ" };
        write!(f, "{s}{}", self.variable)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuantifierPrefix(pub Vec<QuantifierBinder>);

impl fmt::Display for QuantifierPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() { return f.write_str("none"); }
        for (i, b) in self.0.iter().enumerate() { if i > 0 { f.write_str(" ")?; } fmt::Display::fmt(b, f)?; }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeDomain { #[default] Unrestricted, Named(String), Empty }

impl ScopeDomain {
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Unrestricted, x) | (x, Self::Unrestricted) => x.clone(),
            (Self::Named(a), Self::Named(b)) if a == b => Self::Named(a.clone()),
            _ => Self::Empty,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope {
    pub design_family: ScopeDomain,
    pub world_family: ScopeDomain,
    pub quantifiers: QuantifierPrefix,
    pub horizon_steps: Option<u64>,
    pub validity_region: ScopeDomain,
}

impl Scope {
    pub fn intersect(&self, other: &Self) -> Result<Self, Refusal> {
        if self.quantifiers != other.quantifiers {
            let (low, high) = if self.quantifiers <= other.quantifiers { (&self.quantifiers, &other.quantifiers) } else { (&other.quantifiers, &self.quantifiers) };
            return Err(quantifier_refusal(low, high));
        }
        let horizon_steps = match (self.horizon_steps, other.horizon_steps) { (None, x) | (x, None) => x, (Some(a), Some(b)) => Some(a.min(b)) };
        Ok(Self {
            design_family: self.design_family.intersect(&other.design_family),
            world_family: self.world_family.intersect(&other.world_family),
            quantifiers: self.quantifiers.clone(),
            horizon_steps,
            validity_region: self.validity_region.intersect(&other.validity_region),
        })
    }
}

fn quantifier_refusal(low: &QuantifierPrefix, high: &QuantifierPrefix) -> Refusal {
    Refusal::new(
        RefusalCode::FrobRef881, "cannot combine parts with mismatched quantifier prefixes",
        "/scope/quantifiers", high.to_string(), low.to_string(), "", RetryPolicy::AfterChange,
        vec![Repair::new("Supply evidence establishing the same quantifier prefix"), Repair::new("Re-run verification under the required quantifier order")],
    )
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RealityScope { #[default] NoClaim, CalibratedFamily(String) }

impl RealityScope {
    #[must_use] pub fn meet(&self, other: &Self) -> Self {
        match (self, other) { (Self::CalibratedFamily(a), Self::CalibratedFamily(b)) if a == b => Self::CalibratedFamily(a.clone()), _ => Self::NoClaim }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConditionFlag { TierSensitive, ControllerSensitive, ModelConditional }

impl ConditionFlag {
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self { Self::TierSensitive => "tier-sensitive", Self::ControllerSensitive => "controller-sensitive", Self::ModelConditional => "model-conditional" }
    }
}
impl fmt::Display for ConditionFlag { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) } }

/// Monotone set of [`ConditionFlag`]s: supports insertion and union, never removal.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlagSet { flags: BTreeSet<ConditionFlag> }

impl FlagSet {
    #[must_use] pub const fn new() -> Self { Self { flags: BTreeSet::new() } }
    #[must_use] pub fn from_flags(iter: impl IntoIterator<Item = ConditionFlag>) -> Self { Self { flags: iter.into_iter().collect() } }
    pub fn insert(&mut self, flag: ConditionFlag) { self.flags.insert(flag); }
    #[must_use] pub fn union(&self, other: &Self) -> Self { let mut f = self.flags.clone(); f.extend(other.flags.iter().copied()); Self { flags: f } }
    #[must_use] pub fn contains(&self, flag: ConditionFlag) -> bool { self.flags.contains(&flag) }
    #[must_use] pub fn is_superset(&self, other: &Self) -> bool { self.flags.is_superset(&other.flags) }
    #[must_use] pub fn len(&self) -> usize { self.flags.len() }
    #[must_use] pub fn is_empty(&self) -> bool { self.flags.is_empty() }
    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, ConditionFlag> { self.flags.iter() }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputRef { pub digest: Blake3, pub field_path: String }

impl InputRef {
    #[must_use] pub fn new(digest: Blake3, field_path: impl Into<String>) -> Self { Self { digest, field_path: field_path.into() } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssumptionPolarity { Conservative, Optimistic, Exact }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Assumption { pub name: String, pub polarity: AssumptionPolarity }

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssumptionSet(pub BTreeSet<Assumption>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiptId(pub Blake3);

/// Typed leaf evidence for one claim kind `K`. All fields private; no `From<ClaimProduct>`,
/// no setter of `class`, no public constructor taking an [`EvidenceClass`].
///
/// ```compile_fail,E0277
/// use frob_core::evidence::{ClaimKind, ClaimProduct, DetClass, Evidence, FlagSet, InputRef, ProofClaim, RealityScope, Scope, TierMeet};
/// use frob_core::id::hash_blob;
/// let leaf = ClaimProduct::estimated(ClaimKind::Proof, InputRef::new(hash_blob(b"a"), "/x"), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
/// let _: Evidence<ProofClaim> = Evidence::from(leaf);
/// ```
///
/// ```compile_fail,E0616
/// use frob_core::evidence::{AssumptionSet, Evidence, EvidenceClass, ProofClaim, Scope};
/// let mut ev = Evidence::<ProofClaim>::estimated(Scope::default(), AssumptionSet::default(), vec![]);
/// ev.class = EvidenceClass::Proved;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence<K> {
    class: EvidenceClass,
    scope: Scope,
    assumptions: AssumptionSet,
    receipts: Vec<ReceiptId>,
    flags: NoAuthorization,
    kind: PhantomData<K>,
}

impl<K> Evidence<K> {
    fn new_internal(class: EvidenceClass, scope: Scope, assumptions: AssumptionSet, receipts: Vec<ReceiptId>) -> Self {
        Self { class, scope, assumptions, receipts, flags: NoAuthorization::VALUE, kind: PhantomData }
    }
    #[must_use] pub fn estimated(scope: Scope, assumptions: AssumptionSet, receipts: Vec<ReceiptId>) -> Self { Self::new_internal(EvidenceClass::Estimated, scope, assumptions, receipts) }
    #[doc(hidden)] #[deprecated(note = "doorway: callable only by frob-conformance harness and frob-arena study runner (N34-07)")]
    #[must_use] pub fn doorway_tested(scope: Scope, assumptions: AssumptionSet, receipts: Vec<ReceiptId>) -> Self { Self::new_internal(EvidenceClass::Tested, scope, assumptions, receipts) }
    #[must_use] pub const fn class(&self) -> EvidenceClass { self.class }
    #[must_use] pub const fn scope(&self) -> &Scope { &self.scope }
    #[must_use] pub const fn assumptions(&self) -> &AssumptionSet { &self.assumptions }
    #[must_use] pub fn receipts(&self) -> &[ReceiptId] { &self.receipts }
    #[must_use] pub const fn flags(&self) -> NoAuthorization { self.flags }
}

impl Evidence<ProofClaim> {
    #[doc(hidden)] #[deprecated(note = "doorway: callable only by frob-lemma issuing module (N34-07)")]
    #[must_use] pub fn doorway_certified(scope: Scope, assumptions: AssumptionSet, receipts: Vec<ReceiptId>) -> Self { Self::new_internal(EvidenceClass::Certified, scope, assumptions, receipts) }
    #[doc(hidden)] #[deprecated(note = "doorway: callable only by frob-lemma issuing module (N34-07)")]
    #[must_use] pub fn doorway_proved(scope: Scope, assumptions: AssumptionSet, receipts: Vec<ReceiptId>) -> Self { Self::new_internal(EvidenceClass::Proved, scope, assumptions, receipts) }
}

/// Weakest-input-wins claim product carried across every FrankenRobotics boundary.
///
/// ```compile_fail,E0599
/// use frob_core::evidence::{ClaimKind, ClaimProduct, DetClass, EvidenceClass, FlagSet, InputRef, RealityScope, Scope, TierMeet};
/// use frob_core::id::hash_blob;
/// let mut p = ClaimProduct::estimated(ClaimKind::KnownAnswer, InputRef::new(hash_blob(b"a"), "/a"), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
/// p.set_class(EvidenceClass::Proved);
/// ```
///
/// ```compile_fail,E0616
/// use frob_core::evidence::{ClaimKind, ClaimProduct, DetClass, EvidenceClass, FlagSet, InputRef, RealityScope, Scope, TierMeet};
/// use frob_core::id::hash_blob;
/// let mut p = ClaimProduct::estimated(ClaimKind::KnownAnswer, InputRef::new(hash_blob(b"a"), "/a"), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
/// p.class = EvidenceClass::Proved;
/// ```
///
/// ```compile_fail,E0599
/// use frob_core::evidence::{ClaimKind, ClaimProduct, EvidenceClass};
/// let _ = ClaimProduct::new(ClaimKind::Proof, EvidenceClass::Proved);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimProduct {
    kind: ClaimKind,
    class: EvidenceClass,
    weakest: InputRef,
    scope: Scope,
    tier: TierMeet,
    determinism: DetClass,
    reality: RealityScope,
    conditions: FlagSet,
    inputs: Vec<InputRef>,
    authorization: NoAuthorization,
}

impl ClaimProduct {
    #[allow(clippy::too_many_arguments)]
    fn leaf_with_class(
        kind: ClaimKind, class: EvidenceClass, input: InputRef, scope: Scope,
        tier: TierMeet, determinism: DetClass, reality: RealityScope, conditions: FlagSet,
    ) -> Self {
        Self { kind, class, weakest: input.clone(), scope, tier, determinism, reality, conditions, inputs: vec![input], authorization: NoAuthorization::VALUE }
    }

    #[must_use]
    pub fn estimated(kind: ClaimKind, input: InputRef, scope: Scope, tier: TierMeet, determinism: DetClass, reality: RealityScope, conditions: FlagSet) -> Self {
        Self::leaf_with_class(kind, EvidenceClass::Estimated, input, scope, tier, determinism, reality, conditions)
    }

    #[doc(hidden)] #[deprecated(note = "doorway: callable only by frob-conformance harness and frob-arena study runner (N34-07)")]
    #[must_use]
    pub fn tested(kind: ClaimKind, input: InputRef, scope: Scope, tier: TierMeet, determinism: DetClass, reality: RealityScope, conditions: FlagSet) -> Self {
        Self::leaf_with_class(kind, EvidenceClass::Tested, input, scope, tier, determinism, reality, conditions)
    }

    #[doc(hidden)] #[deprecated(note = "doorway: callable only by frob-lemma issuing module (N34-07)")]
    #[must_use]
    pub fn proof_certified(input: InputRef, scope: Scope, tier: TierMeet, determinism: DetClass, reality: RealityScope, conditions: FlagSet) -> Self {
        Self::leaf_with_class(ClaimKind::Proof, EvidenceClass::Certified, input, scope, tier, determinism, reality, conditions)
    }

    #[doc(hidden)] #[deprecated(note = "doorway: callable only by frob-lemma issuing module (N34-07)")]
    #[must_use]
    pub fn proof_proved(input: InputRef, scope: Scope, tier: TierMeet, determinism: DetClass, reality: RealityScope, conditions: FlagSet) -> Self {
        Self::leaf_with_class(ClaimKind::Proof, EvidenceClass::Proved, input, scope, tier, determinism, reality, conditions)
    }

    /// Combine parts by weakest-input-wins (D0, permutation-invariant).
    pub fn combine(kind: ClaimKind, parts: &[Self]) -> Result<Self, Refusal> {
        let Some(first) = parts.first() else {
            return Err(Refusal::new(
                RefusalCode::FrobRef802, "ClaimProduct::combine requires at least one part", "/parts",
                "0 parts", "at least 1 ClaimProduct part", "", RetryPolicy::AfterChange,
                vec![Repair::new("Construct leaf products first"), Repair::new("Pass a non-empty slice to combine")],
            ));
        };
        let (mut raw_min, mut det, mut tier, mut scope, mut reality, mut conds) = (
            first.class, first.determinism, first.tier.clone(), first.scope.clone(), first.reality.clone(), first.conditions.clone(),
        );
        let (mut input_set, mut quants) = (BTreeSet::new(), BTreeSet::new());
        for p in parts {
            raw_min = raw_min.meet(p.class);
            det = det.meet(p.determinism);
            tier = tier.meet(&p.tier);
            reality = reality.meet(&p.reality);
            conds = conds.union(&p.conditions);
            quants.insert(&p.scope.quantifiers);
            input_set.insert(p.weakest.clone());
            input_set.extend(p.inputs.iter().cloned());
        }
        if quants.len() > 1 {
            let mut it = quants.into_iter();
            if let (Some(low), Some(high)) = (it.next(), it.next()) {
                return Err(quantifier_refusal(low, high));
            }
        }
        for p in &parts[1..] { scope = scope.intersect(&p.scope)?; }
        let weakest = parts.iter().filter(|p| p.class == raw_min).map(|p| &p.weakest).min().cloned().unwrap_or_else(|| first.weakest.clone());
        let class = if kind == ClaimKind::Proof { raw_min } else { raw_min.meet(EvidenceClass::Tested) };
        Ok(Self { kind, class, weakest, scope, tier, determinism: det, reality, conditions: conds, inputs: input_set.into_iter().collect(), authorization: NoAuthorization::VALUE })
    }

    #[must_use] pub const fn kind(&self) -> ClaimKind { self.kind }
    #[must_use] pub const fn class(&self) -> EvidenceClass { self.class }
    #[must_use] pub const fn weakest(&self) -> &InputRef { &self.weakest }
    #[must_use] pub const fn scope(&self) -> &Scope { &self.scope }
    #[must_use] pub const fn tier(&self) -> &TierMeet { &self.tier }
    #[must_use] pub const fn determinism(&self) -> DetClass { self.determinism }
    #[must_use] pub const fn reality(&self) -> &RealityScope { &self.reality }
    #[must_use] pub const fn conditions(&self) -> &FlagSet { &self.conditions }
    #[must_use] pub fn inputs(&self) -> &[InputRef] { &self.inputs }
    #[must_use] pub const fn authorization(&self) -> NoAuthorization { self.authorization }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::hash_blob;

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 { self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15); let mut z = self.0; z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9); z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB); z ^ (z >> 31) }
        fn idx(&mut self, n: usize) -> usize { (self.next() as usize) % n }
        fn shuffle<T>(&mut self, s: &mut [T]) { for i in (1..s.len()).rev() { let j = self.idx(i + 1); s.swap(i, j); } }
    }

    fn rand_part(r: &mut Rng, q: &QuantifierPrefix) -> ClaimProduct {
        const KINDS: [ClaimKind; 7] = [ClaimKind::Proof, ClaimKind::KnownAnswer, ClaimKind::Parity, ClaimKind::Fidelity, ClaimKind::DesignComparison, ClaimKind::Reality, ClaimKind::Performance];
        const CLASSES: [EvidenceClass; 4] = [EvidenceClass::Estimated, EvidenceClass::Tested, EvidenceClass::Certified, EvidenceClass::Proved];
        const DETS: [DetClass; 5] = [DetClass::D4Unspecified, DetClass::D3Tolerance, DetClass::D2PerBackend, DetClass::D1CrossBackend, DetClass::D0Exact];
        let kind = KINDS[r.idx(7)];
        let raw = CLASSES[r.idx(4)];
        let class = if kind == ClaimKind::Proof { raw } else { raw.meet(EvidenceClass::Tested) };
        let tier = match r.idx(5) { 0 => TierMeet::Static, 1 => TierMeet::Referee, 2 => TierMeet::fast(FastTier::BatchCpu), 3 => TierMeet::fast(FastTier::GpuCuda), _ => TierMeet::fast(FastTier::GpuMetal) };
        let reality = match r.idx(3) { 0 => RealityScope::NoClaim, 1 => RealityScope::CalibratedFamily("fam-a".into()), _ => RealityScope::CalibratedFamily("fam-b".into()) };
        let b = r.next();
        let mut conds = FlagSet::new();
        if (b & 1) != 0 { conds.insert(ConditionFlag::TierSensitive); }
        if (b & 2) != 0 { conds.insert(ConditionFlag::ControllerSensitive); }
        if (b & 4) != 0 { conds.insert(ConditionFlag::ModelConditional); }
        let inp = InputRef::new(hash_blob(&r.next().to_le_bytes()), format!("/f/{}", r.idx(8)));
        let scope = Scope { design_family: if (b & 8) != 0 { ScopeDomain::Unrestricted } else { ScopeDomain::Named("quad".into()) }, world_family: ScopeDomain::Named("flat".into()), quantifiers: q.clone(), horizon_steps: ((b & 16) != 0).then(|| (r.next() % 500) + 1), validity_region: ScopeDomain::Unrestricted };
        ClaimProduct::leaf_with_class(kind, class, inp, scope, tier, DETS[r.idx(5)], reality, conds)
    }

    #[test]
    fn class_order() {
        assert!(EvidenceClass::Estimated < EvidenceClass::Tested && EvidenceClass::Tested < EvidenceClass::Certified && EvidenceClass::Certified < EvidenceClass::Proved);
        assert!(DetClass::D4Unspecified < DetClass::D3Tolerance && DetClass::D3Tolerance < DetClass::D2PerBackend && DetClass::D2PerBackend < DetClass::D1CrossBackend && DetClass::D1CrossBackend < DetClass::D0Exact);
    }

    #[test]
    fn combine_worked_case() {
        let k = ClaimProduct::leaf_with_class(ClaimKind::Proof, EvidenceClass::Certified, InputRef::new(hash_blob(b"k"), "/k"), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
        let fref = InputRef::new(hash_blob(b"mu"), "/card/mu");
        let card = ClaimProduct::estimated(ClaimKind::KnownAnswer, fref.clone(), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::from_flags([ConditionFlag::ModelConditional]));
        let study = ClaimProduct::leaf_with_class(ClaimKind::DesignComparison, EvidenceClass::Tested, InputRef::new(hash_blob(b"s"), "/s"), Scope::default(), TierMeet::fast(FastTier::GpuCuda), DetClass::D2PerBackend, RealityScope::NoClaim, FlagSet::from_flags([ConditionFlag::TierSensitive]));
        let p = ClaimProduct::combine(ClaimKind::DesignComparison, &[k, card, study]).unwrap();
        assert_eq!((p.kind(), p.class(), p.weakest(), p.determinism(), p.tier(), p.reality(), p.authorization()), (ClaimKind::DesignComparison, EvidenceClass::Estimated, &fref, DetClass::D2PerBackend, &TierMeet::fast(FastTier::GpuCuda), &RealityScope::NoClaim, NoAuthorization::VALUE));
        assert!(p.conditions().contains(ConditionFlag::ModelConditional) && p.conditions().contains(ConditionFlag::TierSensitive));
    }

    #[test]
    fn combine_caps_non_proof_at_tested() {
        let a = ClaimProduct::leaf_with_class(ClaimKind::Proof, EvidenceClass::Certified, InputRef::new(hash_blob(b"a"), "/a"), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
        let b = ClaimProduct::leaf_with_class(ClaimKind::Proof, EvidenceClass::Certified, InputRef::new(hash_blob(b"b"), "/b"), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
        assert_eq!(ClaimProduct::combine(ClaimKind::KnownAnswer, &[a.clone(), b.clone()]).unwrap().class(), EvidenceClass::Tested);
        assert_eq!(ClaimProduct::combine(ClaimKind::Proof, &[a, b]).unwrap().class(), EvidenceClass::Certified);
    }

    #[test]
    fn combine_exact_over_estimated_is_estimated() {
        let exact = ClaimProduct::leaf_with_class(ClaimKind::Proof, EvidenceClass::Proved, InputRef::new(hash_blob(b"l"), "/l"), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
        let mref = InputRef::new(hash_blob(b"mat"), "/mat/e");
        let card = ClaimProduct::estimated(ClaimKind::KnownAnswer, mref.clone(), Scope::default(), TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
        let c = ClaimProduct::combine(ClaimKind::Proof, &[exact, card]).unwrap();
        assert_eq!((c.class(), c.weakest()), (EvidenceClass::Estimated, &mref));
    }

    #[test]
    fn combine_quantifier_mismatch_refuses() {
        let a = ClaimProduct::estimated(ClaimKind::Proof, InputRef::new(hash_blob(b"a"), "/a"), Scope { quantifiers: QuantifierPrefix(vec![QuantifierBinder::for_all("world"), QuantifierBinder::exists("policy")]), ..Scope::default() }, TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
        let b = ClaimProduct::estimated(ClaimKind::Proof, InputRef::new(hash_blob(b"b"), "/b"), Scope { quantifiers: QuantifierPrefix(vec![QuantifierBinder::exists("policy"), QuantifierBinder::for_all("world")]), ..Scope::default() }, TierMeet::Static, DetClass::D0Exact, RealityScope::NoClaim, FlagSet::new());
        let (e1, e2) = (ClaimProduct::combine(ClaimKind::Proof, &[a.clone(), b.clone()]).unwrap_err(), ClaimProduct::combine(ClaimKind::Proof, &[b, a]).unwrap_err());
        assert_eq!(e1.code, RefusalCode::FrobRef881);
        assert_eq!(e1, e2);
        assert_eq!(ClaimProduct::combine(ClaimKind::KnownAnswer, &[]).unwrap_err().code, RefusalCode::FrobRef802);
    }

    #[test]
    fn combine_is_order_independent() {
        const KINDS: [ClaimKind; 7] = [ClaimKind::Proof, ClaimKind::KnownAnswer, ClaimKind::Parity, ClaimKind::Fidelity, ClaimKind::DesignComparison, ClaimKind::Reality, ClaimKind::Performance];
        let mut r = Rng(0xBD00_5E11_D000_0001);
        let q = QuantifierPrefix(vec![QuantifierBinder::exists("policy"), QuantifierBinder::for_all("world")]);
        for _ in 0..10_000 {
            let mut parts: Vec<_> = (0..1 + r.idx(6)).map(|_| rand_part(&mut r, &q)).collect();
            let kind = KINDS[r.idx(7)];
            let raw_min = parts.iter().map(ClaimProduct::class).min().unwrap();
            let exp_class = if kind == ClaimKind::Proof { raw_min } else { raw_min.meet(EvidenceClass::Tested) };
            let exp_det = parts.iter().map(ClaimProduct::determinism).min().unwrap();
            let p1 = ClaimProduct::combine(kind, &parts).unwrap();
            assert_eq!((p1.class(), p1.determinism()), (exp_class, exp_det));
            for p in &parts { assert!(p1.conditions().is_superset(p.conditions())); }
            r.shuffle(&mut parts);
            assert_eq!(p1, ClaimProduct::combine(kind, &parts).unwrap());
        }
    }

    #[test]
    fn conditions_cannot_be_dropped() {
        let mut r = Rng(0xC0AD_1710_A500_0002);
        let q = QuantifierPrefix::default();
        for _ in 0..1_000 {
            let parts: Vec<_> = (0..1 + r.idx(5)).map(|_| rand_part(&mut r, &q)).collect();
            let prod = ClaimProduct::combine(ClaimKind::Fidelity, &parts).unwrap();
            for p in &parts { for f in p.conditions().iter() { assert!(prod.conditions().contains(*f)); } }
        }
    }

    #[test]
    fn tier_meet_of_two_fast_tiers_is_both() {
        let (cpu, metal) = (TierMeet::fast(FastTier::BatchCpu), TierMeet::fast(FastTier::GpuMetal));
        let both = cpu.meet(&metal);
        assert_eq!(both, metal.meet(&cpu));
        assert_eq!(both.to_string(), "batch-cpu+gpu-metal");
        assert!(both.is_provisional());
        assert_eq!(TierMeet::Referee.meet(&metal), metal);
        assert!(TierMeet::Referee.meet(&metal).is_provisional());
    }

    #[test]
    fn static_is_neutral() {
        for t in [TierMeet::Static, TierMeet::Referee, TierMeet::fast(FastTier::BatchCpu), TierMeet::fast(FastTier::GpuCuda), TierMeet::fast(FastTier::GpuMetal)] {
            assert_eq!(TierMeet::Static.meet(&t), t);
            assert_eq!(t.meet(&TierMeet::Static), t);
        }
    }
}
