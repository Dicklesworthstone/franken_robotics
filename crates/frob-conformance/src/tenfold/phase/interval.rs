//! Raw wall-clock interval record (`intervals.ndjson`) and turn-phase rule
//! for ten-fold phase attribution (bead `N39a-23`, §39.6).
//!
//! Times (`start_ms`, `end_ms`, `cpu_ms`, `learner_ms`) and large token counts
//! (`tokens_in`, `tokens_out`) travel as canonical unsigned base-10 integer
//! strings (`^(0|[1-9][0-9]*)$`); small counts (`id`, `awaits`, `devices`,
//! `cores`, `tool_calls`) travel as JSON integers within `0..=2^53 - 1`.
//! Strictly refuses any phase outside the thirteen registered phases at read
//! time, duplicate keys, unknown fields, duplicate interval ids, and inverted
//! intervals (`end_ms < start_ms`). Pure (D0), zero `unsafe`, zero floats,
//! prints nothing (rule L-7).

use std::fmt;

use frob_cert::json::{JsonValue, serialize_canonical};

use crate::tenfold::canon::parse_json_strict;
use crate::tenfold::ids::Phase;
use crate::tenfold::refusal::TenfoldRefusal;

pub const MODULE_OWNER: &str = "N39a-23";

/// The five closed raw interval kinds (`N39a-23` §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntervalKind {
    Turn,
    Block,
    Work,
    Harness,
    Adjudication,
}

impl IntervalKind {
    pub const ALL: [Self; 5] = [
        Self::Turn,
        Self::Block,
        Self::Work,
        Self::Harness,
        Self::Adjudication,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Turn => "turn",
            Self::Block => "block",
            Self::Work => "work",
            Self::Harness => "harness",
            Self::Adjudication => "adjudication",
        }
    }

    pub fn parse(raw: &str, pointer: &str) -> Result<Self, TenfoldRefusal> {
        match raw {
            "turn" => Ok(Self::Turn),
            "block" => Ok(Self::Block),
            "work" => Ok(Self::Work),
            "harness" => Ok(Self::Harness),
            "adjudication" => Ok(Self::Adjudication),
            other => Err(TenfoldRefusal::unknown_field(
                pointer,
                format!("{other:?}"),
                "interval kind in {turn, block, work, harness, adjudication}",
            )),
        }
    }
}

impl fmt::Display for IntervalKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Compute the phase of an agent `turn` from the phases/tags of the tool calls
/// it emitted (`N39a-23` §3).
///
/// - `Phase::Author` if at least one tool call was emitted and every one carries `Phase::Author`;
/// - `Phase::Diagnose` if at least one tool call was emitted and every one carries `Phase::Diagnose`;
/// - `Phase::Propose` otherwise (mixed tags, any other phase tag, or zero tool calls / final answer).
#[must_use]
pub fn turn_phase(tool_tags: &[Phase]) -> Phase {
    if tool_tags.is_empty() {
        return Phase::Propose;
    }
    if tool_tags.iter().all(|&p| p == Phase::Author) {
        return Phase::Author;
    }
    if tool_tags.iter().all(|&p| p == Phase::Diagnose) {
        return Phase::Diagnose;
    }
    Phase::Propose
}

/// Optional compute-resource telemetry attached to a `work` interval (`N39a-23` §2, consumed by `N39a-24`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResourceInfo {
    /// Device indices held by the interval (e.g. `[0]`).
    pub devices: Vec<u32>,
    /// Pinned CPU core count, if known.
    pub cores: Option<u32>,
    /// Measured CPU time of the interval's process tree in milliseconds, if available.
    pub cpu_ms: Option<u64>,
    /// Learner time in milliseconds (populated for `train` when learner timers exist).
    pub learner_ms: Option<u64>,
}

impl ResourceInfo {
    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        let mut entries = Vec::with_capacity(4);
        if let Some(cores) = self.cores {
            entries.push(("cores".to_owned(), JsonValue::Int(i64::from(cores))));
        }
        if let Some(cpu_ms) = self.cpu_ms {
            entries.push(("cpu_ms".to_owned(), JsonValue::Str(cpu_ms.to_string())));
        }
        entries.push((
            "devices".to_owned(),
            JsonValue::Arr(
                self.devices
                    .iter()
                    .map(|&d| JsonValue::Int(i64::from(d)))
                    .collect(),
            ),
        ));
        if let Some(learner_ms) = self.learner_ms {
            entries.push((
                "learner_ms".to_owned(),
                JsonValue::Str(learner_ms.to_string()),
            ));
        }
        JsonValue::Obj(entries)
    }
}

/// One raw interval row of `intervals.ndjson` (`N39a-23` §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval {
    pub id: u64,
    pub kind: IntervalKind,
    /// One of the 13 phases (`Some(p)` where `p.is_phase()`) or `None`.
    /// Required (`Some`) for `Turn` and `Work`; `None` for `Block`; `None` or
    /// `Some(Wait)` for `Harness`; `None` or `Some(Adjudicate)` for `Adjudication`.
    pub phase: Option<Phase>,
    /// Inclusive start timestamp on the harness clock in integer milliseconds.
    pub start_ms: u64,
    /// Exclusive end timestamp on the harness clock in integer milliseconds (`end_ms >= start_ms`).
    pub end_ms: u64,
    /// For `Block`: interval IDs of `work`, `harness`, and `adjudication` intervals awaited.
    pub awaits: Vec<u64>,
    /// For `Work`: optional compute resource accounting block.
    pub resource: Option<ResourceInfo>,
    /// For `Turn`: optional input token count.
    pub tokens_in: Option<u64>,
    /// For `Turn`: optional output token count.
    pub tokens_out: Option<u64>,
    /// For `Turn`: optional emitted tool-call count.
    pub tool_calls: Option<u64>,
    /// Optional process or logical job identity (shared across sub-phases of one multi-phase job).
    pub proc: Option<String>,
    /// Optional tag-map rule identifier that set `phase`.
    pub tag_rule: Option<String>,
}

impl Interval {
    /// Construct a `turn` interval.
    #[must_use]
    pub fn turn(id: u64, phase: Phase, start_ms: u64, end_ms: u64) -> Self {
        Self {
            id,
            kind: IntervalKind::Turn,
            phase: Some(phase),
            start_ms,
            end_ms,
            awaits: Vec::new(),
            resource: None,
            tokens_in: None,
            tokens_out: None,
            tool_calls: None,
            proc: None,
            tag_rule: None,
        }
    }

    /// Construct a `block` interval awaiting `awaits`.
    #[must_use]
    pub fn block(id: u64, start_ms: u64, end_ms: u64, awaits: Vec<u64>) -> Self {
        Self {
            id,
            kind: IntervalKind::Block,
            phase: None,
            start_ms,
            end_ms,
            awaits,
            resource: None,
            tokens_in: None,
            tokens_out: None,
            tool_calls: None,
            proc: None,
            tag_rule: None,
        }
    }

    /// Construct a `work` interval with an optional logical job `proc` identity.
    #[must_use]
    pub fn work(
        id: u64,
        phase: Phase,
        start_ms: u64,
        end_ms: u64,
        proc: Option<String>,
    ) -> Self {
        Self {
            id,
            kind: IntervalKind::Work,
            phase: Some(phase),
            start_ms,
            end_ms,
            awaits: Vec::new(),
            resource: None,
            tokens_in: None,
            tokens_out: None,
            tool_calls: None,
            proc,
            tag_rule: None,
        }
    }

    /// Construct a `harness` service interval.
    #[must_use]
    pub fn harness(id: u64, start_ms: u64, end_ms: u64) -> Self {
        Self {
            id,
            kind: IntervalKind::Harness,
            phase: None,
            start_ms,
            end_ms,
            awaits: Vec::new(),
            resource: None,
            tokens_in: None,
            tokens_out: None,
            tool_calls: None,
            proc: None,
            tag_rule: None,
        }
    }

    /// Construct an `adjudication` interval.
    #[must_use]
    pub fn adjudication(id: u64, start_ms: u64, end_ms: u64) -> Self {
        Self {
            id,
            kind: IntervalKind::Adjudication,
            phase: None,
            start_ms,
            end_ms,
            awaits: Vec::new(),
            resource: None,
            tokens_in: None,
            tokens_out: None,
            tool_calls: None,
            proc: None,
            tag_rule: None,
        }
    }

    /// Validate structural and phase invariants of an in-memory [`Interval`].
    pub fn validate(&self) -> Result<(), TenfoldRefusal> {
        if self.end_ms < self.start_ms {
            return Err(TenfoldRefusal::noncanonical_number(
                "/end_ms",
                format!("end_ms={} < start_ms={}", self.end_ms, self.start_ms),
                "end_ms >= start_ms",
                vec!["Ensure every interval has end_ms >= start_ms on the harness clock".to_owned()],
            ));
        }
        if let Some(p) = self.phase
            && !p.is_phase()
        {
            return Err(TenfoldRefusal::unknown_field(
                "/phase",
                format!("{:?}", p.as_str()),
                "one of the thirteen phases (unattributed is not a phase)",
            ));
        }
        match self.kind {
            IntervalKind::Turn | IntervalKind::Work => {
                if self.phase.is_none() {
                    return Err(TenfoldRefusal::missing_field(
                        "/phase",
                        format!("one of the thirteen phases for kind `{}`", self.kind),
                    ));
                }
            }
            IntervalKind::Block => {
                if let Some(p) = self.phase {
                    return Err(TenfoldRefusal::noncanonical_number(
                        "/phase",
                        format!("{:?}", p.as_str()),
                        "null for kind `block`",
                        vec!["Set `phase` to null on `block` intervals".to_owned()],
                    ));
                }
            }
            IntervalKind::Harness => {
                if let Some(p) = self.phase
                    && p != Phase::Wait
                {
                    return Err(TenfoldRefusal::noncanonical_number(
                        "/phase",
                        format!("{:?}", p.as_str()),
                        "null or \"wait\" for kind `harness`",
                        vec!["Set `phase` to null or \"wait\" on `harness` intervals".to_owned()],
                    ));
                }
            }
            IntervalKind::Adjudication => {
                if let Some(p) = self.phase
                    && p != Phase::Adjudicate
                {
                    return Err(TenfoldRefusal::noncanonical_number(
                        "/phase",
                        format!("{:?}", p.as_str()),
                        "null or \"adjudicate\" for kind `adjudication`",
                        vec![
                            "Set `phase` to null or \"adjudicate\" on `adjudication` intervals"
                                .to_owned(),
                        ],
                    ));
                }
            }
        }
        Ok(())
    }

    /// Effective phase of an owned interval (`adjudicate` for `Adjudication`,
    /// `wait` for `Harness`, the interval's phase for `Turn`/`Work`).
    #[must_use]
    pub fn effective_phase(&self) -> Option<Phase> {
        match self.kind {
            IntervalKind::Turn | IntervalKind::Work => self.phase,
            IntervalKind::Harness => Some(Phase::Wait),
            IntervalKind::Adjudication => Some(Phase::Adjudicate),
            IntervalKind::Block => None,
        }
    }

    /// Render this interval as a canonical [`JsonValue`].
    #[must_use]
    pub fn to_json_value(&self) -> JsonValue {
        let mut entries: Vec<(String, JsonValue)> = Vec::with_capacity(12);
        if self.kind == IntervalKind::Block || !self.awaits.is_empty() {
            entries.push((
                "awaits".to_owned(),
                JsonValue::Arr(
                    self.awaits
                        .iter()
                        .map(|&id| JsonValue::Int(id as i64))
                        .collect(),
                ),
            ));
        }
        entries.push(("end_ms".to_owned(), JsonValue::Str(self.end_ms.to_string())));
        entries.push(("id".to_owned(), JsonValue::Int(self.id as i64)));
        entries.push((
            "kind".to_owned(),
            JsonValue::Str(self.kind.as_str().to_owned()),
        ));
        entries.push((
            "phase".to_owned(),
            match self.phase {
                Some(p) => JsonValue::Str(p.as_str().to_owned()),
                None => JsonValue::Null,
            },
        ));
        if let Some(ref proc_id) = self.proc {
            entries.push(("proc".to_owned(), JsonValue::Str(proc_id.clone())));
        }
        if let Some(ref res) = self.resource {
            entries.push(("resource".to_owned(), res.to_json_value()));
        }
        entries.push((
            "start_ms".to_owned(),
            JsonValue::Str(self.start_ms.to_string()),
        ));
        if let Some(ref rule) = self.tag_rule {
            entries.push(("tag_rule".to_owned(), JsonValue::Str(rule.clone())));
        }
        if let Some(t_in) = self.tokens_in {
            entries.push(("tokens_in".to_owned(), JsonValue::Str(t_in.to_string())));
        }
        if let Some(t_out) = self.tokens_out {
            entries.push(("tokens_out".to_owned(), JsonValue::Str(t_out.to_string())));
        }
        if let Some(tc) = self.tool_calls {
            entries.push(("tool_calls".to_owned(), JsonValue::Int(tc as i64)));
        }
        JsonValue::Obj(entries)
    }
}

/// Strictly parse a canonical unsigned base-10 integer string (`^(0|[1-9][0-9]*)$`).
fn parse_canonical_u64_str(val: &JsonValue, pointer: &str) -> Result<u64, TenfoldRefusal> {
    let Some(raw) = val.as_str() else {
        return Err(TenfoldRefusal::noncanonical_number(
            pointer,
            format!("{val:?}"),
            "canonical unsigned integer string matching ^(0|[1-9][0-9]*)$",
            vec!["Encode millisecond timestamps and token counts as canonical base-10 strings".to_owned()],
        ));
    };
    if raw.is_empty()
        || !raw.bytes().all(|b| b.is_ascii_digit())
        || (raw.len() > 1 && raw.starts_with('0'))
    {
        return Err(TenfoldRefusal::noncanonical_number(
            pointer,
            format!("{raw:?}"),
            "canonical unsigned integer string matching ^(0|[1-9][0-9]*)$",
            vec!["Use non-negative base-10 integer digits without leading zeros or signs".to_owned()],
        ));
    }
    raw.parse::<u64>().map_err(|_| {
        TenfoldRefusal::noncanonical_number(
            pointer,
            format!("{raw:?}"),
            "unsigned 64-bit integer string",
            vec!["Ensure integer string fits within u64".to_owned()],
        )
    })
}

fn parse_nonneg_u64_int(val: &JsonValue, pointer: &str) -> Result<u64, TenfoldRefusal> {
    match val.as_i64() {
        Some(n) if n >= 0 => Ok(n as u64),
        _ => Err(TenfoldRefusal::noncanonical_number(
            pointer,
            format!("{val:?}"),
            "non-negative integer within 0..=2^53-1",
            vec!["Encode interval ids and small counts as non-negative JSON integers".to_owned()],
        )),
    }
}

fn parse_nonneg_u32_int(val: &JsonValue, pointer: &str) -> Result<u32, TenfoldRefusal> {
    match val.as_i64() {
        Some(n) if n >= 0 && n <= i64::from(u32::MAX) => Ok(n as u32),
        _ => Err(TenfoldRefusal::noncanonical_number(
            pointer,
            format!("{val:?}"),
            "non-negative 32-bit integer",
            vec!["Supply a non-negative integer fitting in u32".to_owned()],
        )),
    }
}

fn parse_resource_obj(val: &JsonValue) -> Result<ResourceInfo, TenfoldRefusal> {
    let Some(entries) = val.as_obj() else {
        return Err(TenfoldRefusal::noncanonical_number(
            "/resource",
            format!("{val:?}"),
            "JSON object for `resource`",
            vec!["Encode `resource` as a JSON object with devices/cores/cpu_ms/learner_ms".to_owned()],
        ));
    };
    let mut info = ResourceInfo::default();
    for (k, v) in entries {
        match k.as_str() {
            "devices" => {
                let Some(arr) = v.as_arr() else {
                    return Err(TenfoldRefusal::noncanonical_number(
                        "/resource/devices",
                        format!("{v:?}"),
                        "array of non-negative device indices",
                        vec!["Encode `resource.devices` as an array of device indices".to_owned()],
                    ));
                };
                info.devices.reserve(arr.len());
                for (i, item) in arr.iter().enumerate() {
                    info.devices
                        .push(parse_nonneg_u32_int(item, &format!("/resource/devices/{i}"))?);
                }
            }
            "cores" => {
                if !matches!(v, JsonValue::Null) {
                    info.cores = Some(parse_nonneg_u32_int(v, "/resource/cores")?);
                }
            }
            "cpu_ms" => {
                if !matches!(v, JsonValue::Null) {
                    info.cpu_ms = Some(parse_canonical_u64_str(v, "/resource/cpu_ms")?);
                }
            }
            "learner_ms" => {
                if !matches!(v, JsonValue::Null) {
                    info.learner_ms = Some(parse_canonical_u64_str(v, "/resource/learner_ms")?);
                }
            }
            other => {
                return Err(TenfoldRefusal::unknown_field(
                    format!("/resource/{other}"),
                    format!("{other:?}"),
                    "key in [devices, cores, cpu_ms, learner_ms]",
                ));
            }
        }
    }
    Ok(info)
}

/// Strictly parse one JSON object [`JsonValue`] into an [`Interval`].
/// Single-pass over object keys (zero per-line `BTreeSet` allocation) for 10⁵-interval throughput.
pub fn parse_interval_value(val: &JsonValue) -> Result<Interval, TenfoldRefusal> {
    let Some(entries) = val.as_obj() else {
        return Err(TenfoldRefusal::noncanonical_number(
            "",
            "non-object interval line",
            "JSON object representing one raw interval",
            vec!["Each line of intervals.ndjson must be a single JSON object".to_owned()],
        ));
    };

    let mut id: Option<u64> = None;
    let mut kind: Option<IntervalKind> = None;
    let mut phase: Option<Phase> = None;
    let mut start_ms: Option<u64> = None;
    let mut end_ms: Option<u64> = None;
    let mut awaits: Vec<u64> = Vec::new();
    let mut resource: Option<ResourceInfo> = None;
    let mut tokens_in: Option<u64> = None;
    let mut tokens_out: Option<u64> = None;
    let mut tool_calls: Option<u64> = None;
    let mut proc_id: Option<String> = None;
    let mut tag_rule: Option<String> = None;

    for (k, v) in entries {
        match k.as_str() {
            "id" => id = Some(parse_nonneg_u64_int(v, "/id")?),
            "kind" => {
                let Some(s) = v.as_str() else {
                    return Err(TenfoldRefusal::noncanonical_number(
                        "/kind",
                        format!("{v:?}"),
                        "interval kind string",
                        vec!["Encode `kind` as one of turn, block, work, harness, adjudication".to_owned()],
                    ));
                };
                kind = Some(IntervalKind::parse(s, "/kind")?);
            }
            "phase" => match v {
                JsonValue::Null => phase = None,
                JsonValue::Str(s) => phase = Some(Phase::parse_phase(s, "/phase")?),
                other => {
                    return Err(TenfoldRefusal::noncanonical_number(
                        "/phase",
                        format!("{other:?}"),
                        "one of the thirteen phase strings or null",
                        vec!["Encode `phase` as one of the thirteen phases or null".to_owned()],
                    ));
                }
            },
            "start_ms" => start_ms = Some(parse_canonical_u64_str(v, "/start_ms")?),
            "end_ms" => end_ms = Some(parse_canonical_u64_str(v, "/end_ms")?),
            "awaits" => {
                if !matches!(v, JsonValue::Null) {
                    let Some(arr) = v.as_arr() else {
                        return Err(TenfoldRefusal::noncanonical_number(
                            "/awaits",
                            format!("{v:?}"),
                            "array of awaited interval integer ids",
                            vec!["Encode `awaits` as a JSON array of integer interval ids".to_owned()],
                        ));
                    };
                    awaits.reserve(arr.len());
                    for (i, item) in arr.iter().enumerate() {
                        awaits.push(parse_nonneg_u64_int(item, &format!("/awaits/{i}"))?);
                    }
                }
            }
            "resource" => {
                if !matches!(v, JsonValue::Null) {
                    resource = Some(parse_resource_obj(v)?);
                }
            }
            "tokens_in" => {
                if !matches!(v, JsonValue::Null) {
                    tokens_in = Some(parse_canonical_u64_str(v, "/tokens_in")?);
                }
            }
            "tokens_out" => {
                if !matches!(v, JsonValue::Null) {
                    tokens_out = Some(parse_canonical_u64_str(v, "/tokens_out")?);
                }
            }
            "tool_calls" => {
                if !matches!(v, JsonValue::Null) {
                    tool_calls = Some(parse_nonneg_u64_int(v, "/tool_calls")?);
                }
            }
            "proc" => match v {
                JsonValue::Null => proc_id = None,
                JsonValue::Str(s) => proc_id = Some(s.clone()),
                other => {
                    return Err(TenfoldRefusal::noncanonical_number(
                        "/proc",
                        format!("{other:?}"),
                        "string or null",
                        vec!["Encode `proc` as a string process/job identity or null".to_owned()],
                    ));
                }
            },
            "tag_rule" => match v {
                JsonValue::Null => tag_rule = None,
                JsonValue::Str(s) => tag_rule = Some(s.clone()),
                other => {
                    return Err(TenfoldRefusal::noncanonical_number(
                        "/tag_rule",
                        format!("{other:?}"),
                        "string or null",
                        vec!["Encode `tag_rule` as a string rule name or null".to_owned()],
                    ));
                }
            },
            other => {
                return Err(TenfoldRefusal::unknown_field(
                    format!("/{other}"),
                    format!("{other:?}"),
                    "key in [id, kind, phase, start_ms, end_ms, awaits, resource, tokens_in, tokens_out, tool_calls, proc, tag_rule]",
                ));
            }
        }
    }

    let interval = Interval {
        id: id.ok_or_else(|| TenfoldRefusal::missing_field("/id", "required field `id`"))?,
        kind: kind.ok_or_else(|| TenfoldRefusal::missing_field("/kind", "required field `kind`"))?,
        phase,
        start_ms: start_ms
            .ok_or_else(|| TenfoldRefusal::missing_field("/start_ms", "required field `start_ms`"))?,
        end_ms: end_ms
            .ok_or_else(|| TenfoldRefusal::missing_field("/end_ms", "required field `end_ms`"))?,
        awaits,
        resource,
        tokens_in,
        tokens_out,
        tool_calls,
        proc: proc_id,
        tag_rule,
    };
    interval.validate()?;
    Ok(interval)
}

/// Strictly parse one NDJSON line (UTF-8 bytes of one JSON object) into an [`Interval`].
pub fn parse_interval_line(line: &[u8]) -> Result<Interval, TenfoldRefusal> {
    let val = parse_json_strict(line)?;
    parse_interval_value(&val)
}

/// Strictly parse an entire `intervals.ndjson` byte slice (one JSON object per
/// non-empty line), rejecting BOM, invalid UTF-8, unknown phases, unknown/duplicate
/// keys, invalid timestamps, and duplicate interval ids.
pub fn parse_intervals_bytes(bytes: &[u8]) -> Result<Vec<Interval>, TenfoldRefusal> {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err(TenfoldRefusal::noncanonical_number(
            "",
            "UTF-8 BOM",
            "UTF-8 NDJSON without BOM",
            vec!["Strip the UTF-8 byte-order mark from intervals.ndjson".to_owned()],
        ));
    }
    let text = std::str::from_utf8(bytes).map_err(|e| {
        TenfoldRefusal::noncanonical_number(
            "",
            format!("invalid UTF-8: {e}"),
            "valid UTF-8 NDJSON",
            vec!["Ensure intervals.ndjson is valid UTF-8".to_owned()],
        )
    })?;
    parse_intervals_ndjson(text)
}

/// Strictly parse an entire `intervals.ndjson` string into validated [`Interval`] rows.
pub fn parse_intervals_ndjson(ndjson: &str) -> Result<Vec<Interval>, TenfoldRefusal> {
    // Estimate capacity from line count to avoid repeated Vec reallocations on 10^5 rows.
    let approx_lines = ndjson.as_bytes().iter().filter(|&&b| b == b'\n').count() + 1;
    let mut intervals = Vec::with_capacity(approx_lines);
    let mut seen_ids: Vec<(u64, usize)> = Vec::with_capacity(approx_lines);

    for (idx, raw_line) in ndjson.lines().enumerate() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let iv = parse_interval_line(trimmed.as_bytes())?;
        seen_ids.push((iv.id, idx + 1));
        intervals.push(iv);
    }

    // Verify uniqueness of interval IDs in O(n log n) with contiguous cache-friendly sort.
    seen_ids.sort_unstable();
    for pair in seen_ids.windows(2) {
        if pair[0].0 == pair[1].0 {
            return Err(TenfoldRefusal::duplicate_key(
                "/id",
                format!(
                    "duplicate interval id {} (lines {} and {})",
                    pair[0].0, pair[0].1, pair[1].1
                ),
            ));
        }
    }

    Ok(intervals)
}

/// Serialize a slice of [`Interval`] rows into canonical NDJSON (one canonical JSON object per line, LF-terminated).
#[must_use]
pub fn write_intervals_ndjson(intervals: &[Interval]) -> String {
    let mut out = String::with_capacity(intervals.len().saturating_mul(96));
    for iv in intervals {
        out.push_str(&serialize_canonical(&iv.to_json_value()));
        out.push('\n');
    }
    out
}
