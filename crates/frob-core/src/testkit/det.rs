//! Determinism lanes and D3 decision-equivalence checks (STD-TEST §6).

use std::fmt;
use crate::testkit::evidence::{EvidenceKind, EvidenceRecord, EvidenceStatus, emit_if_configured};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lane { Twice, Workers, Profiles, Platforms, Backends }
impl Lane {
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self { Self::Twice => "twice", Self::Workers => "workers", Self::Profiles => "profiles", Self::Platforms => "platforms", Self::Backends => "backends" }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DetClass { D0 = 0, D1 = 1, D2 = 2, D3 = 3, D4 = 4 }
impl DetClass {
    #[must_use] pub const fn as_str(self) -> &'static str {
        match self { Self::D0 => "D0", Self::D1 => "D1", Self::D2 => "D2", Self::D3 => "D3", Self::D4 => "D4" }
    }
    #[must_use] pub const fn weakest(a: Self, b: Self) -> Self { if (a as u8) >= (b as u8) { a } else { b } }
}
impl fmt::Display for DetClass { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) } }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneRun { Run(usize), Workers(usize), Profile(&'static str), Platform(&'static str), Backend(&'static str) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointDigest { pub checkpoint: usize, pub step: u64, pub quantity: String, pub digest: [u8; 32] }
impl CheckpointDigest {
    #[must_use] pub fn new(checkpoint: usize, step: u64, quantity: impl Into<String>, digest: [u8; 32]) -> Self {
        Self { checkpoint, step, quantity: quantity.into(), digest }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestDiff { pub checkpoint: usize, pub step: u64, pub quantity: String, pub lhs: [u8; 32], pub rhs: [u8; 32] }

#[derive(Debug, Clone, PartialEq)]
pub struct D3DecisionResult {
    pub total: usize, pub all_within_envelope: bool, pub discordant_count: usize,
    pub boundary_sensitive_count: usize, pub net_diff: usize,
    pub upper_95_discordant: f64, pub upper_95_net: f64, pub passed: bool, pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LaneResult {
    pub lane: Lane, pub class: DetClass, pub passed: bool,
    pub first_diff: Option<DigestDiff>, pub d3: Option<D3DecisionResult>, pub message: String,
}

#[must_use]
pub fn compare_digest_pairs(lane: Lane, class: DetClass, lhs: &[CheckpointDigest], rhs: &[CheckpointDigest]) -> LaneResult {
    if class == DetClass::D4 {
        return finish_lane(LaneResult { lane, class, passed: true, first_diff: None, d3: None, message: format!("lane `{}` class D4 (not asserted)", lane.as_str()) });
    }
    let common = lhs.len().min(rhs.len());
    for i in 0..common {
        let (a, b) = (&lhs[i], &rhs[i]);
        if a.checkpoint != b.checkpoint || a.step != b.step || a.quantity != b.quantity || a.digest != b.digest {
            let diff = DigestDiff { checkpoint: a.checkpoint, step: a.step, quantity: a.quantity.clone(), lhs: a.digest, rhs: b.digest };
            let message = format!("lane `{}` ({class}) mismatch at checkpoint {} step {} quantity `{}`", lane.as_str(), diff.checkpoint, diff.step, diff.quantity);
            return finish_lane(LaneResult { lane, class, passed: false, first_diff: Some(diff), d3: None, message });
        }
    }
    if lhs.len() != rhs.len() {
        let extra = if lhs.len() > common { &lhs[common] } else { &rhs[common] };
        let diff = DigestDiff { checkpoint: extra.checkpoint, step: extra.step, quantity: extra.quantity.clone(), lhs: lhs.get(common).map_or([0; 32], |x| x.digest), rhs: rhs.get(common).map_or([0; 32], |x| x.digest) };
        let message = format!("lane `{}` ({class}) count mismatch ({} vs {}) at checkpoint {} step {} quantity `{}`", lane.as_str(), lhs.len(), rhs.len(), diff.checkpoint, diff.step, diff.quantity);
        return finish_lane(LaneResult { lane, class, passed: false, first_diff: Some(diff), d3: None, message });
    }
    finish_lane(LaneResult { lane, class, passed: true, first_diff: None, d3: None, message: format!("lane `{}` ({class}) matched {} checkpoints", lane.as_str(), lhs.len()) })
}

pub fn lane<F: FnMut(LaneRun) -> Vec<CheckpointDigest>>(lane: Lane, class: DetClass, mut run: F) -> LaneResult {
    if class == DetClass::D4 { let _ = run(LaneRun::Run(0)); return compare_digest_pairs(lane, class, &[], &[]); }
    match lane {
        Lane::Twice => { let (a, b) = (run(LaneRun::Run(0)), run(LaneRun::Run(1))); compare_digest_pairs(lane, class, &a, &b) }
        Lane::Workers => {
            let w1 = run(LaneRun::Workers(1));
            for w in [2, 8] { let r = compare_digest_pairs(lane, class, &w1, &run(LaneRun::Workers(w))); if !r.passed { return r; } }
            compare_digest_pairs(lane, class, &w1, &w1)
        }
        Lane::Profiles => { let (a, b) = (run(LaneRun::Profile("debug")), run(LaneRun::Profile("release"))); compare_digest_pairs(lane, class, &a, &b) }
        Lane::Platforms => { let (a, b) = (run(LaneRun::Platform("arm64-macos")), run(LaneRun::Platform("x86_64-linux"))); compare_digest_pairs(lane, class, &a, &b) }
        Lane::Backends => { let (a, b) = (run(LaneRun::Backend("batch-cpu")), run(LaneRun::Backend("gpu"))); compare_digest_pairs(lane, class, &a, &b) }
    }
}

#[must_use]
pub fn exact_binomial_upper_95(k: usize, n: usize) -> f64 {
    if n == 0 || k >= n { return 1.0; }
    if k == 0 { return 1.0 - 0.05_f64.powf(1.0 / (n as f64)); }
    let mut lf = vec![0.0_f64; n + 1];
    for i in 1..=n { lf[i] = lf[i - 1] + (i as f64).ln(); }
    let (mut lo, mut hi) = ((k as f64) / (n as f64), 1.0_f64);
    for _ in 0..64 {
        let mid = 0.5 * (lo + hi);
        let (lp, lq) = (mid.ln(), (1.0 - mid).ln());
        let cdf: f64 = (0..=k).map(|i| (lf[n] - lf[i] - lf[n - i] + (i as f64) * lp + ((n - i) as f64) * lq).exp()).sum();
        if cdf > 0.05 { lo = mid; } else { hi = mid; }
    }
    hi
}

#[must_use]
pub fn check_d3_decisions(pairs: &[(f64, f64)], envelope: f64, threshold: f64) -> D3DecisionResult {
    let total = pairs.len();
    let (mut within, mut n10, mut n01, mut boundary) = (true, 0usize, 0usize, 0usize);
    for &(a, b) in pairs {
        if !a.is_finite() || !b.is_finite() || (a - b).abs() > envelope { within = false; }
        if (a - threshold).abs() < envelope || (b - threshold).abs() < envelope { boundary += 1; }
        match (a >= threshold, b >= threshold) { (true, false) => n10 += 1, (false, true) => n01 += 1, _ => {} }
    }
    let (discordant_count, net_diff) = (n10 + n01, n10.abs_diff(n01));
    let upper_95_discordant = exact_binomial_upper_95(discordant_count, total);
    let upper_95_net = if discordant_count == 0 { 0.0 } else { exact_binomial_upper_95(net_diff, total) };
    let passed = within && total > 0 && upper_95_discordant <= 0.01 && upper_95_net <= 0.005;
    let message = format!("D3 (n={total}): within={within}, discordant={discordant_count} (boundary={boundary}, u95={upper_95_discordant:.6}), net={net_diff} (u95={upper_95_net:.6})");
    let mut ev = EvidenceRecord::new(EvidenceKind::Det, "backends-d3", if passed { EvidenceStatus::Pass } else { EvidenceStatus::Fail });
    ev.determinism = Some("D3".to_owned()); ev.observed = Some(format!("discordant={discordant_count}/{total}")); ev.expected = Some("u95<=0.01,net<=0.005".to_owned());
    emit_if_configured(&ev);
    D3DecisionResult { total, all_within_envelope: within, discordant_count, boundary_sensitive_count: boundary, net_diff, upper_95_discordant, upper_95_net, passed, message }
}

fn finish_lane(res: LaneResult) -> LaneResult {
    let st = if res.class == DetClass::D4 { EvidenceStatus::Skip } else if res.passed { EvidenceStatus::Pass } else { EvidenceStatus::Fail };
    let mut ev = EvidenceRecord::new(EvidenceKind::Det, res.lane.as_str(), st);
    ev.determinism = Some(res.class.as_str().to_owned());
    emit_if_configured(&ev);
    res
}
