//! Exclusive critical-path phase attribution algorithm (bead `N39a-23`, §39.6).
//!
//! Assigns every millisecond instant of a campaign window `[t0_ms, t_stop_ms)`
//! to exactly one of the thirteen phases plus `unattributed`, so that
//! `sum(phases) + unattributed == T` (or `H` for a censored / cap-breached run)
//! holds exactly in integer milliseconds.
//!
//! Algorithm: O(n log n) event sweep with lazy binary heaps and precomputed
//! disjoint per-job phase spans. Pure, input-order invariant, determinism class
//! D0, zero `unsafe`, zero floating-point arithmetic, prints nothing (rule L-7).

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

use frob_cert::json::{JsonValue, serialize_canonical};

use crate::tenfold::ids::{Millis, Phase};
use crate::tenfold::phase::interval::{Interval, IntervalKind};
use crate::tenfold::refusal::TenfoldRefusal;

pub const MODULE_OWNER: &str = "N39a-23";

/// Flag attached to an [`ExclusiveTable`] when a cap-breached run charges its
/// unused remainder `H - (breach - t0)` to `wait` (`N39a-23` §6).
pub const FLAG_CENSORED_REMAINDER: &str = "censored-remainder";

/// Campaign attribution window on the harness clock (integer milliseconds).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributionWindow {
    /// Start of the campaign clock (`t0`), in milliseconds.
    pub t0_ms: u64,
    /// Stop of the campaign clock (`t_stop`): acceptance time, `t0 + H`, or cap-breach time.
    pub t_stop_ms: u64,
    /// Optional stratum horizon `H` (in ms) for a run censored by a cap breach before `t0 + H`.
    /// When `Some(h_ms)`, `[t0_ms, t_stop_ms)` is attributed normally and the unused remainder
    /// `h_ms - (t_stop_ms - t0_ms)` is charged to `wait` with flag `censored-remainder`.
    pub cap_breach_horizon_ms: Option<u64>,
}

impl AttributionWindow {
    /// Standard window `[t0_ms, t_stop_ms)` (accepted run or deadline-censored run at `t0 + H`).
    #[must_use]
    pub const fn new(t0_ms: u64, t_stop_ms: u64) -> Self {
        Self {
            t0_ms,
            t_stop_ms,
            cap_breach_horizon_ms: None,
        }
    }

    /// Cap-breach window: attributes `[t0_ms, breach_ms)` and charges the remainder
    /// `horizon_ms - (breach_ms - t0_ms)` to `wait` with flag `censored-remainder`.
    #[must_use]
    pub const fn cap_breach(t0_ms: u64, breach_ms: u64, horizon_ms: u64) -> Self {
        Self {
            t0_ms,
            t_stop_ms: breach_ms,
            cap_breach_horizon_ms: Some(horizon_ms),
        }
    }
}

/// Exclusive phase attribution table for one campaign (`N39a-23` §4, §6).
///
/// Always satisfies `sum(phases_ms) == clock_ms` exactly in integer milliseconds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExclusiveTable {
    pub t0_ms: u64,
    pub t_stop_ms: u64,
    /// Total charged clock `T` (or `H` for censored/cap-breached runs) in milliseconds.
    pub clock_ms: u64,
    /// Unused remainder `H - (breach - t0)` charged to `wait` on a cap-breached run (`0` otherwise).
    pub censored_remainder_ms: u64,
    /// Flags on the table (contains `"censored-remainder"` iff `censored_remainder_ms > 0`).
    pub flags: Vec<String>,
    /// Exact duration in milliseconds for each bucket in [`Phase::ALL`] order.
    pub phases_ms: [u64; 14],
}

impl ExclusiveTable {
    /// Exact attributed duration for `phase` as [`Millis`].
    #[must_use]
    pub const fn get(&self, phase: Phase) -> Millis {
        Millis(self.phases_ms[phase.index()])
    }

    /// Exact attributed duration for `phase` in integer milliseconds.
    #[must_use]
    pub const fn ms(&self, phase: Phase) -> u64 {
        self.phases_ms[phase.index()]
    }

    /// Exact sum of all fourteen buckets in milliseconds (always equals `self.clock_ms`).
    #[must_use]
    pub fn sum_ms(&self) -> u64 {
        self.phases_ms.iter().copied().sum()
    }

    /// Return all fourteen exclusive buckets as a `BTreeMap<Phase, Millis>` compatible with `LedgerRecord`.
    #[must_use]
    pub fn phases_map(&self) -> BTreeMap<Phase, Millis> {
        let mut map = BTreeMap::new();
        for p in Phase::ALL {
            map.insert(p, self.get(p));
        }
        map
    }

    /// True if `flag` is set on this table (e.g. `FLAG_CENSORED_REMAINDER`).
    #[must_use]
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }

    /// Render the exclusive table as canonical JSON carrying the required
    /// no-authorization flags (`execution_authorized: false`, `reality: "no-claim"`).
    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        let mut phases_s = Vec::with_capacity(Phase::ALL.len());
        let mut phases_ms_obj = Vec::with_capacity(Phase::ALL.len());
        for p in Phase::ALL {
            let ms = self.get(p);
            phases_s.push((
                p.as_str().to_owned(),
                JsonValue::Str(ms.to_canonical_seconds()),
            ));
            phases_ms_obj.push((p.as_str().to_owned(), JsonValue::Str(ms.0.to_string())));
        }
        let flags_arr = self
            .flags
            .iter()
            .map(|f| JsonValue::Str(f.clone()))
            .collect();

        JsonValue::Obj(vec![
            (
                "censored_remainder_ms".to_owned(),
                JsonValue::Str(self.censored_remainder_ms.to_string()),
            ),
            (
                "clock_ms".to_owned(),
                JsonValue::Str(self.clock_ms.to_string()),
            ),
            (
                "clock_s".to_owned(),
                JsonValue::Str(Millis(self.clock_ms).to_canonical_seconds()),
            ),
            ("execution_authorized".to_owned(), JsonValue::Bool(false)),
            ("flags".to_owned(), JsonValue::Arr(flags_arr)),
            ("hardware_safety_signoff".to_owned(), JsonValue::Bool(false)),
            ("phases".to_owned(), JsonValue::Obj(phases_s)),
            ("phases_ms".to_owned(), JsonValue::Obj(phases_ms_obj)),
            ("reality".to_owned(), JsonValue::Str("no-claim".to_owned())),
            ("sim_to_real_certified".to_owned(), JsonValue::Bool(false)),
            ("t0_ms".to_owned(), JsonValue::Str(self.t0_ms.to_string())),
            (
                "t_stop_ms".to_owned(),
                JsonValue::Str(self.t_stop_ms.to_string()),
            ),
        ])
    }

    /// Serialize to unique canonical JSON string (D0 deterministic).
    #[must_use]
    pub fn to_canonical_json(&self) -> String {
        serialize_canonical(&self.to_json_value())
    }

    /// Serialize to unique canonical UTF-8 JSON bytes (D0 deterministic).
    #[must_use]
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        self.to_canonical_json().into_bytes()
    }
}

/// Failure diagnostic describing the attributed phase and covering intervals at a single instant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentDiagnosis {
    pub at_ms: u64,
    pub attributed_phase: Phase,
    pub rule: &'static str,
    pub covering_interval_ids: Vec<u64>,
}

// ============================================================================
// Internal O(n log n) sweep structures
// ============================================================================

#[derive(Debug, Clone, Copy)]
struct ClippedTurn {
    id: u64,
    raw_start_ms: u64,
    phase: Phase,
}

#[derive(Debug, Clone, Copy)]
struct ClippedBlock {
    id: u64,
    raw_start_ms: u64,
    owner: Option<BlockOwner>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BlockOwner {
    job_idx: usize,
    end_ms: u64,
    rep_id: u64,
}

#[derive(Debug, Clone, Copy)]
struct JobSpan {
    start_ms: u64,
    end_ms: u64,
    phase: Phase,
}

#[derive(Debug, Clone)]
struct LogicalJob {
    end_ms: u64,
    rep_id: u64,
    /// Disjoint, strictly sorted spans (`spans[i].end_ms <= spans[i+1].start_ms`).
    spans: Vec<JobSpan>,
}

impl LogicalJob {
    /// Resolve the phase covering elementary segment `[u, v)` in O(log k) (O(1) for single-interval jobs).
    #[inline]
    fn phase_at(&self, u: u64) -> Option<Phase> {
        if self.spans.len() == 1 {
            let s = self.spans[0];
            return (s.start_ms <= u && u < s.end_ms).then_some(s.phase);
        }
        let pos = self.spans.partition_point(|s| s.start_ms <= u);
        if pos > 0 {
            let s = self.spans[pos - 1];
            if u < s.end_ms {
                return Some(s.phase);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct RawJobPiece {
    id: u64,
    raw_start_ms: u64,
    start_ms: u64,
    end_ms: u64,
    phase: Phase,
}

/// Convert raw clipped sub-intervals of one logical job into strictly disjoint,
/// sorted [`JobSpan`]s in O(k log k). Overlapping sub-intervals within one `proc`
/// resolve deterministically by earliest `(raw_start_ms, id)`.
fn build_disjoint_spans(pieces: &mut [RawJobPiece]) -> Vec<JobSpan> {
    if pieces.len() == 1 {
        let p = pieces[0];
        return vec![JobSpan {
            start_ms: p.start_ms,
            end_ms: p.end_ms,
            phase: p.phase,
        }];
    }
    pieces.sort_unstable_by_key(|p| (p.start_ms, p.end_ms, p.id));
    let non_overlapping = pieces.windows(2).all(|w| w[0].end_ms <= w[1].start_ms);
    if non_overlapping {
        return pieces
            .iter()
            .map(|p| JobSpan {
                start_ms: p.start_ms,
                end_ms: p.end_ms,
                phase: p.phase,
            })
            .collect();
    }

    // Mini-sweep for overlapping pieces within the same logical `proc`.
    let mut events: Vec<(u64, bool, usize)> = Vec::with_capacity(pieces.len() * 2);
    for (idx, p) in pieces.iter().enumerate() {
        events.push((p.start_ms, true, idx));
        events.push((p.end_ms, false, idx));
    }
    events.sort_unstable_by_key(|e| e.0);

    let mut active = vec![false; pieces.len()];
    let mut heap: BinaryHeap<Reverse<(u64, u64, usize)>> = BinaryHeap::new();
    let mut spans: Vec<JobSpan> = Vec::with_capacity(pieces.len());

    let mut prev_t = events[0].0;
    let mut i = 0usize;
    while i < events.len() {
        let t = events[i].0;
        if prev_t < t {
            while let Some(&Reverse((_, _, idx))) = heap.peek() {
                if active[idx] {
                    let phase = pieces[idx].phase;
                    if let Some(last) = spans.last_mut()
                        && last.end_ms == prev_t
                        && last.phase == phase
                    {
                        last.end_ms = t;
                    } else {
                        spans.push(JobSpan {
                            start_ms: prev_t,
                            end_ms: t,
                            phase,
                        });
                    }
                    break;
                }
                heap.pop();
            }
        }
        while i < events.len() && events[i].0 == t {
            let (_, is_start, idx) = events[i];
            if is_start {
                active[idx] = true;
                heap.push(Reverse((pieces[idx].raw_start_ms, pieces[idx].id, idx)));
            } else {
                active[idx] = false;
            }
            i += 1;
        }
        prev_t = t;
    }
    spans
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventKind {
    TurnStart(usize),
    TurnEnd(usize),
    BlockStart(usize),
    BlockEnd(usize),
    AdjStart,
    AdjEnd,
    HarnessStart,
    HarnessEnd,
    Boundary,
}

#[derive(Debug, Clone, Copy)]
struct SweepEvent {
    t: u64,
    kind: EventKind,
}

/// Active turn key for min-heap (earliest `raw_start_ms`, tie -> lower `id`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TurnHeapEntry {
    raw_start_ms: u64,
    id: u64,
    idx: usize,
}

/// Active block key for max-heap (awaited owner with latest `end_ms`, tie -> lower `rep_id`;
/// blocks with an owner beat blocks without an owner; further ties -> earliest block `raw_start_ms`, lower `block.id`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BlockHeapEntry {
    has_owner: bool,
    owner_end_ms: u64,
    owner_rep_id_rev: Reverse<u64>,
    block_start_rev: Reverse<u64>,
    block_id_rev: Reverse<u64>,
    idx: usize,
}

/// Validate inputs and run the exclusive critical-path attribution algorithm (`N39a-23` §4, §6).
///
/// Guarantees:
/// - O(n log n) time complexity in `intervals.len()`.
/// - Input-order invariance (shuffling `intervals` produces an identical [`ExclusiveTable`]).
/// - Exact millisecond conservation: `table.sum_ms() == table.clock_ms`.
pub fn attribute_exclusive(
    intervals: &[Interval],
    window: AttributionWindow,
) -> Result<ExclusiveTable, TenfoldRefusal> {
    if window.t_stop_ms < window.t0_ms {
        return Err(TenfoldRefusal::noncanonical_number(
            "/stop_ms",
            format!("t_stop_ms={} < t0_ms={}", window.t_stop_ms, window.t0_ms),
            "t_stop_ms >= t0_ms",
            vec!["Ensure stop timestamp is greater than or equal to t0".to_owned()],
        ));
    }

    let elapsed_ms = window.t_stop_ms - window.t0_ms;
    let (clock_ms, censored_remainder_ms, flags) = match window.cap_breach_horizon_ms {
        Some(h_ms) => {
            if elapsed_ms > h_ms {
                return Err(TenfoldRefusal::noncanonical_number(
                    "/cap_breach_horizon_ms",
                    format!("breach elapsed {} > horizon {}", elapsed_ms, h_ms),
                    "cap_breach_horizon_ms >= (t_stop_ms - t0_ms)",
                    vec![
                        "Cap-breach horizon H must be at least breach_ms - t0_ms".to_owned(),
                    ],
                ));
            }
            let rem = h_ms - elapsed_ms;
            let f = if rem > 0 {
                vec![FLAG_CENSORED_REMAINDER.to_owned()]
            } else {
                Vec::new()
            };
            (h_ms, rem, f)
        }
        None => (elapsed_ms, 0, Vec::new()),
    };

    // Validate all intervals and verify unique interval IDs in O(n log n).
    let mut id_check: Vec<u64> = Vec::with_capacity(intervals.len());
    for iv in intervals {
        iv.validate()?;
        id_check.push(iv.id);
    }
    id_check.sort_unstable();
    for pair in id_check.windows(2) {
        if pair[0] == pair[1] {
            return Err(TenfoldRefusal::duplicate_key(
                "/id",
                format!("duplicate interval id {}", pair[0]),
            ));
        }
    }

    if elapsed_ms == 0 {
        let mut phases_ms = [0_u64; 14];
        phases_ms[Phase::Wait.index()] = censored_remainder_ms;
        return Ok(ExclusiveTable {
            t0_ms: window.t0_ms,
            t_stop_ms: window.t_stop_ms,
            clock_ms,
            censored_remainder_ms,
            flags,
            phases_ms,
        });
    }

    // Step 1: Clip intervals to [t0_ms, t_stop_ms) and group logical jobs for awaited owners.
    // Multi-phase `work` intervals sharing a non-empty `proc` form one logical job.
    let mut proc_groups: BTreeMap<&str, Vec<RawJobPiece>> = BTreeMap::new();
    let mut singleton_pieces: Vec<(u64, RawJobPiece)> = Vec::new();
    let mut proc_by_work_id: Vec<(u64, &str)> = Vec::new();

    let mut turns: Vec<ClippedTurn> = Vec::new();
    let mut raw_blocks: Vec<(usize, u64, u64)> = Vec::new(); // (orig_idx, cs, ce)
    let mut events: Vec<SweepEvent> = Vec::with_capacity(intervals.len().saturating_mul(2));

    for (orig_idx, iv) in intervals.iter().enumerate() {
        // Track proc membership for all `work` intervals (even if a sub-phase lies outside the
        // clip window) so `block.awaits` referencing an earlier sub-phase id still resolves the job.
        if iv.kind == IntervalKind::Work
            && let Some(ref p) = iv.proc
            && !p.is_empty()
        {
            proc_by_work_id.push((iv.id, p.as_str()));
        }

        let cs = iv.start_ms.max(window.t0_ms);
        let ce = iv.end_ms.min(window.t_stop_ms);
        if cs >= ce {
            continue;
        }

        match iv.kind {
            IntervalKind::Turn => {
                let phase = iv.phase.unwrap_or(Phase::Propose);
                let tidx = turns.len();
                turns.push(ClippedTurn {
                    id: iv.id,
                    raw_start_ms: iv.start_ms,
                    phase,
                });
                events.push(SweepEvent {
                    t: cs,
                    kind: EventKind::TurnStart(tidx),
                });
                events.push(SweepEvent {
                    t: ce,
                    kind: EventKind::TurnEnd(tidx),
                });
            }
            IntervalKind::Block => {
                raw_blocks.push((orig_idx, cs, ce));
            }
            IntervalKind::Work => {
                let phase = iv.phase.unwrap_or(Phase::Unattributed);
                let piece = RawJobPiece {
                    id: iv.id,
                    raw_start_ms: iv.start_ms,
                    start_ms: cs,
                    end_ms: ce,
                    phase,
                };
                if let Some(ref p) = iv.proc
                    && !p.is_empty()
                {
                    proc_groups.entry(p.as_str()).or_default().push(piece);
                } else {
                    singleton_pieces.push((iv.id, piece));
                }
                events.push(SweepEvent {
                    t: cs,
                    kind: EventKind::Boundary,
                });
                events.push(SweepEvent {
                    t: ce,
                    kind: EventKind::Boundary,
                });
            }
            IntervalKind::Harness => {
                let piece = RawJobPiece {
                    id: iv.id,
                    raw_start_ms: iv.start_ms,
                    start_ms: cs,
                    end_ms: ce,
                    phase: Phase::Wait,
                };
                singleton_pieces.push((iv.id, piece));
                events.push(SweepEvent {
                    t: cs,
                    kind: EventKind::HarnessStart,
                });
                events.push(SweepEvent {
                    t: ce,
                    kind: EventKind::HarnessEnd,
                });
            }
            IntervalKind::Adjudication => {
                let piece = RawJobPiece {
                    id: iv.id,
                    raw_start_ms: iv.start_ms,
                    start_ms: cs,
                    end_ms: ce,
                    phase: Phase::Adjudicate,
                };
                singleton_pieces.push((iv.id, piece));
                events.push(SweepEvent {
                    t: cs,
                    kind: EventKind::AdjStart,
                });
                events.push(SweepEvent {
                    t: ce,
                    kind: EventKind::AdjEnd,
                });
            }
        }
    }

    // Build LogicalJob table and id -> job_idx lookup sorted by id (O(n log n), zero HashMap).
    let mut jobs: Vec<LogicalJob> = Vec::with_capacity(proc_groups.len() + singleton_pieces.len());
    let mut id_to_job: Vec<(u64, usize)> =
        Vec::with_capacity(singleton_pieces.len() + proc_by_work_id.len());
    let mut proc_to_job: BTreeMap<&str, usize> = BTreeMap::new();

    for (proc_name, mut pieces) in proc_groups {
        let mut end_ms = 0_u64;
        let mut rep_id = u64::MAX;
        for p in &pieces {
            if p.end_ms > end_ms {
                end_ms = p.end_ms;
            }
            if p.id < rep_id {
                rep_id = p.id;
            }
        }
        let spans = build_disjoint_spans(&mut pieces);
        let jidx = jobs.len();
        jobs.push(LogicalJob {
            end_ms,
            rep_id,
            spans,
        });
        proc_to_job.insert(proc_name, jidx);
    }

    for (wid, proc_name) in proc_by_work_id {
        if let Some(&jidx) = proc_to_job.get(proc_name) {
            id_to_job.push((wid, jidx));
        }
    }

    for (iv_id, piece) in singleton_pieces {
        let jidx = jobs.len();
        jobs.push(LogicalJob {
            end_ms: piece.end_ms,
            rep_id: piece.id,
            spans: vec![JobSpan {
                start_ms: piece.start_ms,
                end_ms: piece.end_ms,
                phase: piece.phase,
            }],
        });
        id_to_job.push((iv_id, jidx));
    }

    id_to_job.sort_unstable_by_key(|&(id, _)| id);

    // Resolve each clipped block's awaited owner once in O(|awaits| log n).
    let mut blocks: Vec<ClippedBlock> = Vec::with_capacity(raw_blocks.len());
    for (orig_idx, cs, ce) in raw_blocks {
        let iv = &intervals[orig_idx];
        let mut best: Option<BlockOwner> = None;
        for &awaited_id in &iv.awaits {
            if let Ok(pos) = id_to_job.binary_search_by_key(&awaited_id, |&(id, _)| id) {
                let jidx = id_to_job[pos].1;
                let job = &jobs[jidx];
                let cand = BlockOwner {
                    job_idx: jidx,
                    end_ms: job.end_ms,
                    rep_id: job.rep_id,
                };
                let better = match best {
                    None => true,
                    Some(cur) => {
                        cand.end_ms > cur.end_ms
                            || (cand.end_ms == cur.end_ms && cand.rep_id < cur.rep_id)
                    }
                };
                if better {
                    best = Some(cand);
                }
            }
        }
        let bidx = blocks.len();
        blocks.push(ClippedBlock {
            id: iv.id,
            raw_start_ms: iv.start_ms,
            owner: best,
        });
        events.push(SweepEvent {
            t: cs,
            kind: EventKind::BlockStart(bidx),
        });
        events.push(SweepEvent {
            t: ce,
            kind: EventKind::BlockEnd(bidx),
        });
    }

    // Step 2: Sort sweep events by timestamp and attribute every elementary segment [prev_t, t).
    events.sort_unstable_by_key(|e| e.t);

    let mut turn_active = vec![false; turns.len()];
    let mut turn_heap: BinaryHeap<Reverse<TurnHeapEntry>> = BinaryHeap::new();
    let mut block_active = vec![false; blocks.len()];
    let mut block_heap: BinaryHeap<BlockHeapEntry> = BinaryHeap::new();
    let mut adj_active: i64 = 0;
    let mut harness_active: i64 = 0;

    let mut phases_ms = [0_u64; 14];
    let mut prev_t = window.t0_ms;

    let mut i = 0usize;
    while i < events.len() {
        let t = events[i].t;
        if prev_t < t {
            let (phase, _) = resolve_active_segment(
                prev_t,
                &turns,
                &turn_active,
                &mut turn_heap,
                &blocks,
                &block_active,
                &mut block_heap,
                &jobs,
                adj_active,
                harness_active,
            );
            phases_ms[phase.index()] += t - prev_t;
        }
        while i < events.len() && events[i].t == t {
            match events[i].kind {
                EventKind::TurnStart(idx) => {
                    turn_active[idx] = true;
                    turn_heap.push(Reverse(TurnHeapEntry {
                        raw_start_ms: turns[idx].raw_start_ms,
                        id: turns[idx].id,
                        idx,
                    }));
                }
                EventKind::TurnEnd(idx) => {
                    turn_active[idx] = false;
                }
                EventKind::BlockStart(idx) => {
                    block_active[idx] = true;
                    let b = blocks[idx];
                    let (has_owner, owner_end_ms, owner_rep_id) = match b.owner {
                        Some(o) => (true, o.end_ms, o.rep_id),
                        None => (false, 0, u64::MAX),
                    };
                    block_heap.push(BlockHeapEntry {
                        has_owner,
                        owner_end_ms,
                        owner_rep_id_rev: Reverse(owner_rep_id),
                        block_start_rev: Reverse(b.raw_start_ms),
                        block_id_rev: Reverse(b.id),
                        idx,
                    });
                }
                EventKind::BlockEnd(idx) => {
                    block_active[idx] = false;
                }
                EventKind::AdjStart => adj_active += 1,
                EventKind::AdjEnd => adj_active -= 1,
                EventKind::HarnessStart => harness_active += 1,
                EventKind::HarnessEnd => harness_active -= 1,
                EventKind::Boundary => {}
            }
            i += 1;
        }
        prev_t = t;
    }

    if prev_t < window.t_stop_ms {
        let (phase, _) = resolve_active_segment(
            prev_t,
            &turns,
            &turn_active,
            &mut turn_heap,
            &blocks,
            &block_active,
            &mut block_heap,
            &jobs,
            adj_active,
            harness_active,
        );
        phases_ms[phase.index()] += window.t_stop_ms - prev_t;
    }

    // Step 3: Cap-breach unused remainder goes to `wait` with flag `censored-remainder`.
    if censored_remainder_ms > 0 {
        phases_ms[Phase::Wait.index()] += censored_remainder_ms;
    }

    Ok(ExclusiveTable {
        t0_ms: window.t0_ms,
        t_stop_ms: window.t_stop_ms,
        clock_ms,
        censored_remainder_ms,
        flags,
        phases_ms,
    })
}

#[allow(clippy::too_many_arguments)]
fn resolve_active_segment(
    u: u64,
    turns: &[ClippedTurn],
    turn_active: &[bool],
    turn_heap: &mut BinaryHeap<Reverse<TurnHeapEntry>>,
    blocks: &[ClippedBlock],
    block_active: &[bool],
    block_heap: &mut BinaryHeap<BlockHeapEntry>,
    jobs: &[LogicalJob],
    adj_active: i64,
    harness_active: i64,
) -> (Phase, &'static str) {
    // Rule 2a: A `turn` covers [u, v) -> earliest raw_start_ms, tie -> lower id.
    while let Some(&Reverse(top)) = turn_heap.peek() {
        if turn_active[top.idx] {
            return (turns[top.idx].phase, "rule-2a:turn");
        }
        turn_heap.pop();
    }
    // Rule 2b: A `block` covers [u, v) -> awaited owner with latest end_ms, tie -> lower id.
    while let Some(&top) = block_heap.peek() {
        if block_active[top.idx] {
            let phase = match blocks[top.idx].owner {
                Some(owner) => jobs[owner.job_idx].phase_at(u).unwrap_or(Phase::Wait),
                None => Phase::Wait,
            };
            return (phase, "rule-2b:block-owner");
        }
        block_heap.pop();
    }
    // Rule 2c: Bare adjudication -> adjudicate; else bare harness -> wait.
    if adj_active > 0 {
        return (Phase::Adjudicate, "rule-2c:adjudication");
    }
    if harness_active > 0 {
        return (Phase::Wait, "rule-2c:harness");
    }
    // Rule 2d: Otherwise -> unattributed.
    (Phase::Unattributed, "rule-2d:unattributed")
}

/// Inspect a single instant `at_ms` inside `[window.t0_ms, window.t_stop_ms)`
/// for failure diagnostics, evaluating block owners against the full `window`
/// and returning the attributed phase, winning rule, and sorted IDs of all
/// intervals covering `at_ms`.
pub fn diagnose_instant(
    intervals: &[Interval],
    window: AttributionWindow,
    at_ms: u64,
) -> Result<SegmentDiagnosis, TenfoldRefusal> {
    if window.t_stop_ms < window.t0_ms {
        return Err(TenfoldRefusal::noncanonical_number(
            "/stop_ms",
            format!("t_stop_ms={} < t0_ms={}", window.t_stop_ms, window.t0_ms),
            "t_stop_ms >= t0_ms",
            vec!["Ensure stop timestamp is greater than or equal to t0".to_owned()],
        ));
    }
    if at_ms < window.t0_ms || at_ms >= window.t_stop_ms {
        return Err(TenfoldRefusal::noncanonical_number(
            "/at_ms",
            at_ms.to_string(),
            format!("instant in [{}..{})", window.t0_ms, window.t_stop_ms),
            vec!["Choose an instant strictly inside [t0_ms, t_stop_ms)".to_owned()],
        ));
    }
    for iv in intervals {
        iv.validate()?;
    }

    let mut covering: Vec<u64> = intervals
        .iter()
        .filter(|iv| {
            let cs = iv.start_ms.max(window.t0_ms);
            let ce = iv.end_ms.min(window.t_stop_ms);
            cs <= at_ms && at_ms < ce
        })
        .map(|iv| iv.id)
        .collect();
    covering.sort_unstable();

    // Rule 2a: turn covering at_ms (earliest raw_start_ms, tie -> lower id).
    if let Some(best_turn) = intervals
        .iter()
        .filter(|iv| {
            iv.kind == IntervalKind::Turn
                && iv.start_ms.max(window.t0_ms) <= at_ms
                && at_ms < iv.end_ms.min(window.t_stop_ms)
        })
        .min_by_key(|iv| (iv.start_ms, iv.id))
    {
        return Ok(SegmentDiagnosis {
            at_ms,
            attributed_phase: best_turn.phase.unwrap_or(Phase::Propose),
            rule: "rule-2a:turn",
            covering_interval_ids: covering,
        });
    }

    // Rule 2b: block covering at_ms -> evaluate full-window logical jobs.
    let active_blocks: Vec<&Interval> = intervals
        .iter()
        .filter(|iv| {
            iv.kind == IntervalKind::Block
                && iv.start_ms.max(window.t0_ms) <= at_ms
                && at_ms < iv.end_ms.min(window.t_stop_ms)
        })
        .collect();
    if !active_blocks.is_empty() {
        let mut proc_groups: BTreeMap<&str, Vec<RawJobPiece>> = BTreeMap::new();
        let mut singletons: Vec<(u64, RawJobPiece)> = Vec::new();
        let mut proc_by_id: Vec<(u64, &str)> = Vec::new();
        for iv in intervals {
            if iv.kind == IntervalKind::Work
                && let Some(ref p) = iv.proc
                && !p.is_empty()
            {
                proc_by_id.push((iv.id, p.as_str()));
            }
            let cs = iv.start_ms.max(window.t0_ms);
            let ce = iv.end_ms.min(window.t_stop_ms);
            if cs >= ce {
                continue;
            }
            let phase = match iv.kind {
                IntervalKind::Work => iv.phase.unwrap_or(Phase::Unattributed),
                IntervalKind::Harness => Phase::Wait,
                IntervalKind::Adjudication => Phase::Adjudicate,
                IntervalKind::Turn | IntervalKind::Block => continue,
            };
            let piece = RawJobPiece {
                id: iv.id,
                raw_start_ms: iv.start_ms,
                start_ms: cs,
                end_ms: ce,
                phase,
            };
            if iv.kind == IntervalKind::Work
                && let Some(ref p) = iv.proc
                && !p.is_empty()
            {
                proc_groups.entry(p.as_str()).or_default().push(piece);
            } else {
                singletons.push((iv.id, piece));
            }
        }
        let mut jobs: Vec<LogicalJob> = Vec::new();
        let mut id_to_job: Vec<(u64, usize)> = Vec::new();
        let mut proc_to_job: BTreeMap<&str, usize> = BTreeMap::new();
        for (pname, mut pieces) in proc_groups {
            let end_ms = pieces.iter().map(|p| p.end_ms).max().unwrap_or(0);
            let rep_id = pieces.iter().map(|p| p.id).min().unwrap_or(u64::MAX);
            let spans = build_disjoint_spans(&mut pieces);
            let jidx = jobs.len();
            jobs.push(LogicalJob {
                end_ms,
                rep_id,
                spans,
            });
            proc_to_job.insert(pname, jidx);
        }
        for (wid, pname) in proc_by_id {
            if let Some(&jidx) = proc_to_job.get(pname) {
                id_to_job.push((wid, jidx));
            }
        }
        for (iv_id, piece) in singletons {
            let jidx = jobs.len();
            jobs.push(LogicalJob {
                end_ms: piece.end_ms,
                rep_id: piece.id,
                spans: vec![JobSpan {
                    start_ms: piece.start_ms,
                    end_ms: piece.end_ms,
                    phase: piece.phase,
                }],
            });
            id_to_job.push((iv_id, jidx));
        }
        id_to_job.sort_unstable_by_key(|&(id, _)| id);

        let mut best: Option<BlockOwner> = None;
        for blk in active_blocks {
            for &aid in &blk.awaits {
                if let Ok(pos) = id_to_job.binary_search_by_key(&aid, |&(id, _)| id) {
                    let jidx = id_to_job[pos].1;
                    let job = &jobs[jidx];
                    let cand = BlockOwner {
                        job_idx: jidx,
                        end_ms: job.end_ms,
                        rep_id: job.rep_id,
                    };
                    let better = match best {
                        None => true,
                        Some(cur) => {
                            cand.end_ms > cur.end_ms
                                || (cand.end_ms == cur.end_ms && cand.rep_id < cur.rep_id)
                        }
                    };
                    if better {
                        best = Some(cand);
                    }
                }
            }
        }
        let phase = match best {
            Some(owner) => jobs[owner.job_idx].phase_at(at_ms).unwrap_or(Phase::Wait),
            None => Phase::Wait,
        };
        return Ok(SegmentDiagnosis {
            at_ms,
            attributed_phase: phase,
            rule: "rule-2b:block-owner",
            covering_interval_ids: covering,
        });
    }

    // Rule 2c & 2d
    let has_adj = intervals.iter().any(|iv| {
        iv.kind == IntervalKind::Adjudication
            && iv.start_ms.max(window.t0_ms) <= at_ms
            && at_ms < iv.end_ms.min(window.t_stop_ms)
    });
    if has_adj {
        return Ok(SegmentDiagnosis {
            at_ms,
            attributed_phase: Phase::Adjudicate,
            rule: "rule-2c:adjudication",
            covering_interval_ids: covering,
        });
    }
    let has_harness = intervals.iter().any(|iv| {
        iv.kind == IntervalKind::Harness
            && iv.start_ms.max(window.t0_ms) <= at_ms
            && at_ms < iv.end_ms.min(window.t_stop_ms)
    });
    if has_harness {
        return Ok(SegmentDiagnosis {
            at_ms,
            attributed_phase: Phase::Wait,
            rule: "rule-2c:harness",
            covering_interval_ids: covering,
        });
    }

    Ok(SegmentDiagnosis {
        at_ms,
        attributed_phase: Phase::Unattributed,
        rule: "rule-2d:unattributed",
        covering_interval_ids: covering,
    })
}
