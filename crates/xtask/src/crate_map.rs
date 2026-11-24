//! Closed crate map and ADR-WS-01 enforcement for FrankenRobotics (bead N34-02).
//!
//! Holds the compiled list of 43 counted crates across seven workspaces (39 core
//! crates across layers 0..=7, 4 annex crates), conditional rows (`frob-units`),
//! tombstones, non-Rust scope caps, `[package.metadata.frob]` reader, and the
//! closed-list / metadata validator.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Number of original counted crates in the closed map (39 core + 4 annex).
pub const ORIGINAL_CRATE_COUNT: usize = 43;
/// Highest core layer number (`Core(0)` through `Core(7)`).
pub const MAX_CORE_LAYER: u8 = 7;
/// Scope-cap review trigger percentage (115% -> `floor(cap * 115 / 100)`).
pub const CAP_TRIGGER_PERCENT: u32 = 115;
/// Rule identifier for ADR-WS-01 unlisted workspace member findings.
pub const RULE_UNLISTED_CRATE: &str = "ADR-WS-01.unlisted-crate";
/// Rule identifier for missing or mismatched `[package.metadata.frob]` metadata.
pub const RULE_METADATA_MISMATCH: &str = "map.metadata-mismatch";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layer {
    Core(u8),
    Annex,
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(n) => write!(f, "{n}"),
            Self::Annex => f.write_str("annex"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    Solid,
    Frontier,
    Moonshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reason {
    Original,
    Dependency,
    DeviceTarget,
    ProofBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrateRow {
    pub name: &'static str,
    pub dir: &'static str,
    pub layer: Layer,
    /// 1-based intra-layer dependency rank for counted rows (`0` for conditional `frob-units`).
    pub rank: u8,
    pub tiers: &'static [Tier],
    pub chapters: &'static [&'static str],
    /// Scope cap in thousands of lines of Rust including tests.
    pub cap_kloc: u16,
    pub reason: Reason,
    pub adr: Option<&'static str>,
    pub conditional: Option<&'static str>,
}

impl CrateRow {
    #[must_use]
    pub const fn is_counted(&self) -> bool {
        self.conditional.is_none()
    }

    #[must_use]
    pub fn is_frontier_or_moonshot_only(&self) -> bool {
        !self.tiers.is_empty() && !self.tiers.contains(&Tier::Solid)
    }

    /// True when `self` sits strictly below `other` in the L-1 / L-3 dependency order.
    #[must_use]
    pub const fn is_below(&self, other: &Self) -> bool {
        match (self.layer, other.layer) {
            (Layer::Core(a), Layer::Core(b)) => a < b || (a == b && self.rank < other.rank),
            (Layer::Core(_), Layer::Annex) => true,
            (Layer::Annex, Layer::Core(_) | Layer::Annex) => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tombstone {
    pub name: &'static str,
    pub adr: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonRustCap {
    pub name: &'static str,
    pub dir: &'static str,
    pub language: &'static str,
    pub cap_kloc: Option<u16>,
}

const S: &[Tier] = &[Tier::Solid];
const F: &[Tier] = &[Tier::Frontier];
const M: &[Tier] = &[Tier::Moonshot];
const SF: &[Tier] = &[Tier::Solid, Tier::Frontier];
const FM: &[Tier] = &[Tier::Frontier, Tier::Moonshot];

macro_rules! orig {
    ($name:literal, $dir:literal, $layer:expr, $rank:literal, $tiers:expr, $ch:expr, $cap:literal) => {
        CrateRow {
            name: $name,
            dir: $dir,
            layer: $layer,
            rank: $rank,
            tiers: $tiers,
            chapters: $ch,
            cap_kloc: $cap,
            reason: Reason::Original,
            adr: None,
            conditional: None,
        }
    };
}

/// Authoritative closed crate map: 43 original counted rows followed by conditional rows.
pub const CRATE_MAP: &[CrateRow] = &[
    // Layer 0 (2 crates, 28 kLOC)
    orig!("frob-core", "crates/frob-core", Layer::Core(0), 1, S, &["§8", "§12", "§35"], 8),
    orig!("frob-fsim", "crates/frob-fsim", Layer::Core(0), 2, S, &["§33"], 20),
    // Layer 1 (3 crates, 40 kLOC)
    orig!("frob-lie", "crates/frob-lie", Layer::Core(1), 1, S, &["§13"], 12),
    orig!("frob-algebra", "crates/frob-algebra", Layer::Core(1), 2, S, &["§13"], 14),
    orig!("frob-kir", "crates/frob-kir", Layer::Core(1), 3, SF, &["§21"], 14),
    // Layer 2 (5 crates, 84 kLOC)
    orig!("frob-actuate", "crates/frob-actuate", Layer::Core(2), 1, S, &["§15"], 12),
    orig!("frob-design", "crates/frob-design", Layer::Core(2), 2, S, &["§14"], 18),
    orig!("frob-model", "crates/frob-model", Layer::Core(2), 3, S, &["§14"], 16),
    orig!("frob-io", "crates/frob-io", Layer::Core(2), 4, S, &["§14"], 10),
    orig!("frob-scene", "crates/frob-scene", Layer::Core(2), 5, SF, &["§19"], 28),
    // Layer 3 (5 crates, 92 kLOC)
    orig!("frob-kernels", "crates/frob-kernels", Layer::Core(3), 1, S, &["§21"], 26),
    orig!("frob-contact", "crates/frob-contact", Layer::Core(3), 2, SF, &["§18"], 22),
    orig!("frob-dyn", "crates/frob-dyn", Layer::Core(3), 3, SF, &["§16"], 20),
    orig!("frob-phylum", "crates/frob-phylum", Layer::Core(3), 4, SF, &["§17"], 10),
    orig!("frob-sense", "crates/frob-sense", Layer::Core(3), 5, S, &["§20"], 14),
    // Layer 4 (7 crates, 98 kLOC)
    orig!("frob-spec", "crates/frob-spec", Layer::Core(4), 1, S, &["§24", "§25"], 8),
    orig!("frob-control", "crates/frob-control", Layer::Core(4), 2, SF, &["§22"], 16),
    orig!("frob-gait", "crates/frob-gait", Layer::Core(4), 3, F, &["§22"], 8),
    orig!("frob-analyze", "crates/frob-analyze", Layer::Core(4), 4, S, &["§14", "§28"], 20),
    orig!("frob-learn", "crates/frob-learn", Layer::Core(4), 5, SF, &["§23"], 18),
    orig!("frob-arena", "crates/frob-arena", Layer::Core(4), 6, S, &["§24"], 14),
    orig!("frob-opt", "crates/frob-opt", Layer::Core(4), 7, SF, &["§28"], 14),
    // Layer 5 (8 crates, 78 kLOC)
    orig!("frob-shield", "crates/frob-shield", Layer::Core(5), 1, F, &["§22"], 8),
    orig!("frob-autopsy", "crates/frob-autopsy", Layer::Core(5), 2, S, &["§27"], 8),
    orig!("frob-nemesis", "crates/frob-nemesis", Layer::Core(5), 3, SF, &["§26"], 10),
    orig!("frob-ground", "crates/frob-ground", Layer::Core(5), 4, F, &["§30"], 10),
    orig!("frob-synth", "crates/frob-synth", Layer::Core(5), 5, FM, &["§25", "§28"], 10),
    orig!("frob-foundry", "crates/frob-foundry", Layer::Core(5), 6, F, &["§31"], 12),
    orig!("frob-lifecycle", "crates/frob-lifecycle", Layer::Core(5), 7, M, &["§31"], 6),
    orig!("frob-mission", "crates/frob-mission", Layer::Core(5), 8, F, &["§25"], 14),
    // Layer 6 (4 crates, 62 kLOC)
    orig!("frob-cert", "crates/frob-cert", Layer::Core(6), 1, S, &["§12"], 12),
    orig!("frob-chronicle", "crates/frob-chronicle", Layer::Core(6), 2, S, &["§29"], 18),
    orig!("frob-lemma", "crates/frob-lemma", Layer::Core(6), 3, SF, &["§12"], 10),
    orig!("frob-verbs", "crates/frob-verbs", Layer::Core(6), 4, S, &["§32", "§28"], 22),
    // Layer 7 (5 crates, 47 kLOC)
    orig!("frob-mcp", "surface/frob-mcp", Layer::Core(7), 1, S, &["§32"], 6),
    orig!("frob-cli", "crates/frob-cli", Layer::Core(7), 2, S, &["§32"], 8),
    orig!("frob-tui", "crates/frob-tui", Layer::Core(7), 3, F, &["§32"], 6),
    orig!("frob-conformance", "crates/frob-conformance", Layer::Core(7), 4, S, &["§38", "§39"], 24),
    orig!("xtask", "crates/xtask", Layer::Core(7), 5, S, &["§34"], 3),
    // Annexes (4 crates, 32 kLOC)
    orig!("frob-spark-cuda", "spark/frob-spark-cuda", Layer::Annex, 1, S, &["§21"], 8),
    orig!("frob-spark-metal", "spark/frob-spark-metal", Layer::Annex, 2, S, &["§21"], 7),
    orig!("frob-derive", "derive/frob-derive", Layer::Annex, 3, F, &["§21", "§12"], 14),
    orig!("frob-gymnasium", "bindings/frob-gymnasium", Layer::Annex, 4, S, &["§23"], 3),
    // Conditional row (not counted in the 43-crate / 561 kLOC totals; Rule 5)
    CrateRow {
        name: "frob-units",
        dir: "crates/frob-units",
        layer: Layer::Core(0),
        rank: 0,
        tiers: S,
        chapters: &["§33"],
        cap_kloc: 3,
        reason: Reason::Original,
        adr: None,
        conditional: Some("INT-TOOLCHAIN-01"),
    },
];

/// Retired crate names that may never be re-introduced (Rule 4).
pub const TOMBSTONES: &[Tombstone] = &[];

/// Non-Rust scope caps held beside the Rust crate table (Rule 6).
pub const NON_RUST_CAPS: &[NonRustCap] = &[
    NonRustCap { name: "shared-dialect-kernels", dir: "kernels", language: "dialect", cap_kloc: Some(20) },
    NonRustCap { name: "viewer", dir: "viewer", language: "javascript", cap_kloc: Some(12) },
    NonRustCap { name: "gymnasium-python", dir: "bindings/frob-gymnasium", language: "python", cap_kloc: Some(1) },
    NonRustCap { name: "lean", dir: "lean", language: "lean", cap_kloc: None },
];

pub fn counted_rows() -> impl Iterator<Item = &'static CrateRow> {
    CRATE_MAP.iter().filter(|row| row.is_counted())
}

#[must_use]
pub fn lookup(name: &str) -> Option<&'static CrateRow> {
    lookup_in(CRATE_MAP, name)
}

#[must_use]
pub fn lookup_in<'a>(rows: &'a [CrateRow], name: &str) -> Option<&'a CrateRow> {
    rows.iter().find(|row| row.name == name)
}

#[must_use]
pub fn layer_cap_kloc(layer: Layer) -> u32 {
    counted_rows()
        .filter(|row| row.layer == layer)
        .map(|row| u32::from(row.cap_kloc))
        .sum()
}

#[must_use]
pub fn total_cap_kloc() -> u32 {
    counted_rows().map(|row| u32::from(row.cap_kloc)).sum()
}

#[must_use]
pub fn frontier_or_moonshot_only_core_cap_kloc() -> u32 {
    counted_rows()
        .filter(|row| matches!(row.layer, Layer::Core(_)) && row.is_frontier_or_moonshot_only())
        .map(|row| u32::from(row.cap_kloc))
        .sum()
}

#[must_use]
pub const fn trigger_cap_kloc(cap_kloc: u32) -> u32 {
    (cap_kloc * CAP_TRIGGER_PERCENT) / 100
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableError {
    DuplicateName(String),
    DuplicateDir(String),
    InvalidLayerNumber { name: String, layer: u8 },
    DuplicateRankInLayer { layer: Layer, rank: u8 },
    NonContiguousRanks { layer: Layer, observed: Vec<u8>, expected: Vec<u8> },
    OriginalWithAdr { name: String, adr: String },
    AdditionWithoutAdr { name: String },
    AdditionWithOriginalReason { name: String },
    TombstonedName { name: String, adr: String },
    EmptyTiers { name: String },
    EmptyChapters { name: String },
    EmptyConditionalGate { name: String },
    ZeroCap { name: String },
}

/// Validate structural and ADR-WS-01 invariants of a crate table and tombstone list.
pub fn validate_table(rows: &[CrateRow], tombstones: &[Tombstone]) -> Result<(), Vec<TableError>> {
    let mut errors = Vec::new();
    let mut seen_names = BTreeSet::new();
    let mut seen_dirs = BTreeSet::new();
    let mut all_layer_ranks: BTreeSet<(Layer, u8)> = BTreeSet::new();
    let mut counted_ranks_by_layer: BTreeMap<Layer, Vec<u8>> = BTreeMap::new();
    let tomb_map: BTreeMap<&str, &str> = tombstones.iter().map(|t| (t.name, t.adr)).collect();

    let mut counted_idx = 0usize;
    for row in rows {
        if !seen_names.insert(row.name) {
            errors.push(TableError::DuplicateName(row.name.to_owned()));
        }
        if !seen_dirs.insert(row.dir) {
            errors.push(TableError::DuplicateDir(row.dir.to_owned()));
        }
        if let Some(&adr) = tomb_map.get(row.name) {
            errors.push(TableError::TombstonedName {
                name: row.name.to_owned(),
                adr: adr.to_owned(),
            });
        }
        if let Layer::Core(n) = row.layer
            && n > MAX_CORE_LAYER
        {
            errors.push(TableError::InvalidLayerNumber { name: row.name.to_owned(), layer: n });
        }
        if row.tiers.is_empty() {
            errors.push(TableError::EmptyTiers { name: row.name.to_owned() });
        }
        if row.chapters.is_empty() {
            errors.push(TableError::EmptyChapters { name: row.name.to_owned() });
        }
        if row.cap_kloc == 0 {
            errors.push(TableError::ZeroCap { name: row.name.to_owned() });
        }
        if let Some(gate) = row.conditional
            && gate.trim().is_empty()
        {
            errors.push(TableError::EmptyConditionalGate { name: row.name.to_owned() });
        }
        match (row.reason, row.adr) {
            (Reason::Original, Some(adr)) => {
                errors.push(TableError::OriginalWithAdr {
                    name: row.name.to_owned(),
                    adr: adr.to_owned(),
                });
            }
            (Reason::Dependency | Reason::DeviceTarget | Reason::ProofBoundary, None) => {
                errors.push(TableError::AdditionWithoutAdr { name: row.name.to_owned() });
            }
            (Reason::Dependency | Reason::DeviceTarget | Reason::ProofBoundary, Some(adr))
                if adr.trim().is_empty() =>
            {
                errors.push(TableError::AdditionWithoutAdr { name: row.name.to_owned() });
            }
            _ => {}
        }
        if !all_layer_ranks.insert((row.layer, row.rank)) {
            errors.push(TableError::DuplicateRankInLayer { layer: row.layer, rank: row.rank });
        }
        if row.is_counted() {
            if counted_idx >= ORIGINAL_CRATE_COUNT && row.reason == Reason::Original {
                errors.push(TableError::AdditionWithOriginalReason { name: row.name.to_owned() });
            }
            counted_idx += 1;
            counted_ranks_by_layer.entry(row.layer).or_default().push(row.rank);
        }
    }

    for (layer, mut observed) in counted_ranks_by_layer {
        observed.sort_unstable();
        let expected: Vec<u8> = match u8::try_from(observed.len()) {
            Ok(max_rank) => (1..=max_rank).collect(),
            Err(_) => (1..=u8::MAX).collect(),
        };
        if observed != expected {
            errors.push(TableError::NonContiguousRanks { layer, observed, expected });
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestMetadata {
    pub layer: Layer,
    pub rank: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetadataError {
    Io(String),
    MissingTable,
    MissingField(&'static str),
    InvalidField { field: &'static str, detail: String },
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "manifest I/O error: {msg}"),
            Self::MissingTable => f.write_str("missing [package.metadata.frob] table"),
            Self::MissingField(field) => write!(f, "missing required key `{field}` in [package.metadata.frob]"),
            Self::InvalidField { field, detail } => write!(f, "invalid `{field}` in [package.metadata.frob]: {detail}"),
        }
    }
}

pub fn read_manifest_metadata(manifest_path: &Path) -> Result<ManifestMetadata, MetadataError> {
    let text = fs::read_to_string(manifest_path).map_err(|e| MetadataError::Io(e.to_string()))?;
    parse_manifest_metadata(&text)
}

/// Parse `[package.metadata.frob]` from a TOML manifest (table, inline table, or dotted keys).
pub fn parse_manifest_metadata(toml_text: &str) -> Result<ManifestMetadata, MetadataError> {
    let mut section = String::new();
    let mut saw_frob_table = false;
    let mut layer_val: Option<Layer> = None;
    let mut rank_val: Option<u8> = None;

    for raw_line in toml_text.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            if let Some(header) = parse_table_header(line) {
                section = header;
                if section == "package.metadata.frob" {
                    saw_frob_table = true;
                }
            }
            continue;
        }
        let Some((k, v)) = split_kv(line) else {
            continue;
        };
        let full_key = if section.is_empty() {
            normalize_dotted_key(k)
        } else {
            format!("{section}.{}", normalize_dotted_key(k))
        };
        if full_key == "package.metadata.frob" {
            saw_frob_table = true;
            parse_inline_frob_table(v, &mut layer_val, &mut rank_val)?;
        } else if let Some(leaf) = full_key.strip_prefix("package.metadata.frob.") {
            saw_frob_table = true;
            apply_frob_kv(leaf, v, &mut layer_val, &mut rank_val)?;
        }
    }

    if !saw_frob_table {
        return Err(MetadataError::MissingTable);
    }
    Ok(ManifestMetadata {
        layer: layer_val.ok_or(MetadataError::MissingField("layer"))?,
        rank: rank_val.ok_or(MetadataError::MissingField("rank"))?,
    })
}

pub fn read_package_name(manifest_path: &Path) -> Result<String, MetadataError> {
    let text = fs::read_to_string(manifest_path).map_err(|e| MetadataError::Io(e.to_string()))?;
    parse_package_name(&text)
}

pub fn parse_package_name(toml_text: &str) -> Result<String, MetadataError> {
    let mut section = String::new();
    for raw_line in toml_text.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            if let Some(header) = parse_table_header(line) {
                section = header;
            }
            continue;
        }
        if section == "package"
            && let Some((k, v)) = split_kv(line)
            && normalize_dotted_key(k) == "name"
        {
            return parse_toml_string(v).ok_or_else(|| MetadataError::InvalidField {
                field: "package.name",
                detail: format!("expected quoted string, got {v}"),
            });
        }
    }
    Err(MetadataError::MissingField("package.name"))
}

fn strip_toml_comment(line: &str) -> &str {
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    for (idx, ch) in line.char_indices() {
        if in_double {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double = false;
            }
        } else if in_single {
            if ch == '\'' {
                in_single = false;
            }
        } else {
            match ch {
                '"' => in_double = true,
                '\'' => in_single = true,
                '#' => return line.split_at(idx).0,
                _ => {}
            }
        }
    }
    line
}

fn parse_table_header(line: &str) -> Option<String> {
    if let Some(inner) = line.strip_prefix("[[").and_then(|s| s.strip_suffix("]]")) {
        return Some(format!("[[{}]]", normalize_dotted_key(inner.trim())));
    }
    if let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        return Some(normalize_dotted_key(inner.trim()));
    }
    None
}

fn normalize_dotted_key(raw: &str) -> String {
    let mut out = String::new();
    for (i, seg) in raw.split('.').enumerate() {
        if i > 0 {
            out.push('.');
        }
        let s = seg.trim();
        match parse_toml_string(s) {
            Some(unquoted) => out.push_str(&unquoted),
            None => out.push_str(s),
        }
    }
    out
}

fn split_kv(line: &str) -> Option<(&str, &str)> {
    let (lhs, rhs) = line.split_once('=')?;
    Some((lhs.trim(), rhs.trim()))
}

fn parse_toml_string(raw: &str) -> Option<String> {
    let s = raw.trim();
    if let Some(inner) = s.strip_prefix('"').and_then(|t| t.strip_suffix('"')) {
        let mut out = String::with_capacity(inner.len());
        let mut chars = inner.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next()? {
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    other => {
                        out.push('\\');
                        out.push(other);
                    }
                }
            } else {
                out.push(c);
            }
        }
        return Some(out);
    }
    s.strip_prefix('\'')
        .and_then(|t| t.strip_suffix('\''))
        .map(str::to_owned)
}

fn parse_u8_value(field: &'static str, raw: &str) -> Result<u8, MetadataError> {
    let cleaned = raw.trim().replace('_', "");
    if cleaned.is_empty() || !cleaned.bytes().all(|b| b.is_ascii_digit()) {
        return Err(MetadataError::InvalidField {
            field,
            detail: format!("expected non-negative integer, got `{raw}`"),
        });
    }
    cleaned.parse::<u8>().map_err(|e| MetadataError::InvalidField {
        field,
        detail: format!("integer `{raw}` out of u8 range: {e}"),
    })
}

fn parse_layer_value(raw: &str) -> Result<Layer, MetadataError> {
    if let Some(s) = parse_toml_string(raw) {
        if s == "annex" {
            return Ok(Layer::Annex);
        }
        return Err(MetadataError::InvalidField {
            field: "layer",
            detail: format!("expected integer 0..={MAX_CORE_LAYER} or \"annex\", got \"{s}\""),
        });
    }
    let n = parse_u8_value("layer", raw)?;
    if n > MAX_CORE_LAYER {
        return Err(MetadataError::InvalidField {
            field: "layer",
            detail: format!("core layer {n} exceeds maximum {MAX_CORE_LAYER}"),
        });
    }
    Ok(Layer::Core(n))
}

fn apply_frob_kv(
    key: &str,
    val: &str,
    layer_val: &mut Option<Layer>,
    rank_val: &mut Option<u8>,
) -> Result<(), MetadataError> {
    match key {
        "layer" => {
            if layer_val.is_some() {
                return Err(MetadataError::InvalidField {
                    field: "layer",
                    detail: "duplicate `layer` key".to_owned(),
                });
            }
            *layer_val = Some(parse_layer_value(val)?);
        }
        "rank" => {
            if rank_val.is_some() {
                return Err(MetadataError::InvalidField {
                    field: "rank",
                    detail: "duplicate `rank` key".to_owned(),
                });
            }
            *rank_val = Some(parse_u8_value("rank", val)?);
        }
        _ => {}
    }
    Ok(())
}

fn parse_inline_frob_table(
    raw: &str,
    layer_val: &mut Option<Layer>,
    rank_val: &mut Option<u8>,
) -> Result<(), MetadataError> {
    let trimmed = raw.trim();
    let Some(body) = trimmed.strip_prefix('{').and_then(|t| t.strip_suffix('}')) else {
        return Err(MetadataError::InvalidField {
            field: "frob",
            detail: format!("expected inline table, got `{raw}`"),
        });
    };
    for part in split_inline_entries(body) {
        let entry = part.trim();
        if entry.is_empty() {
            continue;
        }
        let Some((k, v)) = split_kv(entry) else {
            return Err(MetadataError::InvalidField {
                field: "frob",
                detail: format!("malformed inline table entry `{entry}`"),
            });
        };
        apply_frob_kv(&normalize_dotted_key(k), v, layer_val, rank_val)?;
    }
    Ok(())
}

fn split_inline_entries(body: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut rest = body;
    let (mut in_double, mut in_single, mut escaped) = (false, false, false);
    let mut offset = 0usize;
    for (idx, ch) in body.char_indices() {
        if in_double {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_double = false;
            }
        } else if in_single {
            if ch == '\'' {
                in_single = false;
            }
        } else {
            match ch {
                '"' => in_double = true,
                '\'' => in_single = true,
                ',' => {
                    let (head, tail) = rest.split_at(idx - offset);
                    parts.push(head);
                    rest = tail.strip_prefix(',').unwrap_or(tail);
                    offset = idx + 1;
                }
                _ => {}
            }
        }
    }
    parts.push(rest);
    parts
}

/// One workspace member package inspected by [`validate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceMember {
    pub name: String,
    pub rel_dir: String,
    pub manifest_path: PathBuf,
}

impl WorkspaceMember {
    pub fn from_manifest(workspace_root: &Path, manifest_path: &Path) -> Result<Self, MetadataError> {
        let name = read_package_name(manifest_path)?;
        let parent = manifest_path.parent().unwrap_or(workspace_root);
        let rel = parent.strip_prefix(workspace_root).unwrap_or(parent);
        Ok(Self {
            name,
            rel_dir: normalize_rel_dir(rel),
            manifest_path: manifest_path.to_path_buf(),
        })
    }

    fn rel_manifest_path(&self) -> String {
        if self.rel_dir.is_empty() || self.rel_dir == "." {
            "Cargo.toml".to_owned()
        } else {
            format!("{}/Cargo.toml", self.rel_dir)
        }
    }
}

fn normalize_rel_dir(path: &Path) -> String {
    let raw = path.to_string_lossy().replace('\\', "/");
    raw.trim_start_matches("./").trim_end_matches('/').to_owned()
}

fn is_under_apps(rel_dir: &str) -> bool {
    rel_dir == "apps" || rel_dir.starts_with("apps/")
}

/// Finding emitted by [`validate`] and [`validate_with_table`], matching `frob.xtask.report/1`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    pub rule: &'static str,
    pub subject: String,
    pub path: String,
    pub line: Option<u32>,
    pub message: String,
    pub repair: String,
}

/// Validate workspace members against the compiled [`CRATE_MAP`] and [`TOMBSTONES`].
///
/// `link_packages` lists the exempt thin-binary package names allowed under `apps/`
/// (for example `&["frob", "frobd"]`). Rows whose directories do not exist yet are
/// not findings. Returned findings are deterministically sorted.
#[must_use]
pub fn validate(workspace_members: &[WorkspaceMember], link_packages: &[&str]) -> Vec<Finding> {
    validate_with_table(CRATE_MAP, TOMBSTONES, workspace_members, link_packages)
}

/// Validate workspace members against an explicit table and tombstone list.
#[must_use]
pub fn validate_with_table(
    rows: &[CrateRow],
    tombstones: &[Tombstone],
    workspace_members: &[WorkspaceMember],
    link_packages: &[&str],
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let tomb_map: BTreeMap<&str, &str> = tombstones.iter().map(|t| (t.name, t.adr)).collect();

    for member in workspace_members {
        let rel_manifest = member.rel_manifest_path();

        // Rule 7: under apps/, only link_packages are exempt; any other package is unlisted.
        if is_under_apps(&member.rel_dir) {
            if link_packages.contains(&member.name.as_str()) {
                continue;
            }
            findings.push(Finding {
                rule: RULE_UNLISTED_CRATE,
                subject: member.name.clone(),
                path: rel_manifest,
                line: None,
                message: format!(
                    "package `{}` under apps/ is not in the exempt link_packages list (ADR-WS-01)",
                    member.name
                ),
                repair: "remove the unlisted apps/ package or add it to the workspace link_packages table".to_owned(),
            });
            continue;
        }

        if let Some(&adr) = tomb_map.get(member.name.as_str()) {
            findings.push(Finding {
                rule: RULE_UNLISTED_CRATE,
                subject: member.name.clone(),
                path: rel_manifest,
                line: None,
                message: format!("crate `{}` is tombstoned by {adr} and may never be reused", member.name),
                repair: format!("remove crate `{}` or choose a new name with an ADR-WS-01 record", member.name),
            });
            continue;
        }

        let Some(row) = lookup_in(rows, &member.name) else {
            findings.push(Finding {
                rule: RULE_UNLISTED_CRATE,
                subject: member.name.clone(),
                path: rel_manifest,
                line: None,
                message: format!("workspace member `{}` has no row in CRATE_MAP (ADR-WS-01)", member.name),
                repair: "remove the unlisted crate, or add a row to crates/xtask/src/crate_map.rs citing an ADR-WS-01 decision".to_owned(),
            });
            continue;
        };

        if row.conditional.is_some() && !member.manifest_path.parent().is_some_and(Path::is_dir) {
            findings.push(Finding {
                rule: RULE_UNLISTED_CRATE,
                subject: member.name.clone(),
                path: rel_manifest,
                line: None,
                message: format!(
                    "conditional crate `{}` is only accepted when its directory exists on disk",
                    member.name
                ),
                repair: "remove the phantom conditional member entry".to_owned(),
            });
            continue;
        }

        match read_manifest_metadata(&member.manifest_path) {
            Ok(meta) if meta.layer == row.layer && meta.rank == row.rank => {}
            Ok(meta) => {
                findings.push(Finding {
                    rule: RULE_METADATA_MISMATCH,
                    subject: member.name.clone(),
                    path: rel_manifest,
                    line: None,
                    message: format!(
                        "manifest [package.metadata.frob] declares layer={}, rank={}, but CRATE_MAP specifies layer={}, rank={}",
                        meta.layer, meta.rank, row.layer, row.rank
                    ),
                    repair: format!(
                        "set [package.metadata.frob] to layer = {}, rank = {} in Cargo.toml",
                        format_toml_layer(row.layer),
                        row.rank
                    ),
                });
            }
            Err(err) => {
                findings.push(Finding {
                    rule: RULE_METADATA_MISMATCH,
                    subject: member.name.clone(),
                    path: rel_manifest,
                    line: None,
                    message: format!("failed to read [package.metadata.frob] for `{}`: {err}", member.name),
                    repair: format!(
                        "add [package.metadata.frob] with layer = {} and rank = {} to Cargo.toml",
                        format_toml_layer(row.layer),
                        row.rank
                    ),
                });
            }
        }
    }

    findings.sort();
    findings
}

fn format_toml_layer(layer: Layer) -> String {
    match layer {
        Layer::Core(n) => n.to_string(),
        Layer::Annex => "\"annex\"".to_owned(),
    }
}

/// Discover Cargo workspace member manifests under `workspace_root` from `[workspace]`.
pub fn discover_workspace_members(workspace_root: &Path) -> Result<Vec<WorkspaceMember>, MetadataError> {
    let root_manifest = workspace_root.join("Cargo.toml");
    let text = fs::read_to_string(&root_manifest).map_err(|e| MetadataError::Io(e.to_string()))?;
    let members_pats = parse_workspace_array(&text, "members");
    let excluded: BTreeSet<String> = parse_workspace_array(&text, "exclude")
        .into_iter()
        .map(|s| normalize_rel_dir(Path::new(&s)))
        .collect();

    let mut manifest_paths = Vec::new();
    for pat in members_pats {
        if let Some(prefix) = pat.strip_suffix("/*") {
            let dir = workspace_root.join(prefix);
            if !dir.is_dir() {
                continue;
            }
            let entries = fs::read_dir(&dir).map_err(|e| MetadataError::Io(e.to_string()))?;
            for entry in entries {
                let entry = entry.map_err(|e| MetadataError::Io(e.to_string()))?;
                let candidate = entry.path().join("Cargo.toml");
                if candidate.is_file() {
                    let rel = entry
                        .path()
                        .strip_prefix(workspace_root)
                        .map(normalize_rel_dir)
                        .unwrap_or_default();
                    if !excluded.contains(&rel) {
                        manifest_paths.push(candidate);
                    }
                }
            }
        } else {
            let rel = normalize_rel_dir(Path::new(&pat));
            if excluded.contains(&rel) {
                continue;
            }
            let candidate = workspace_root.join(&pat).join("Cargo.toml");
            if candidate.is_file() {
                manifest_paths.push(candidate);
            }
        }
    }

    manifest_paths.sort();
    manifest_paths.dedup();
    manifest_paths
        .into_iter()
        .map(|p| WorkspaceMember::from_manifest(workspace_root, &p))
        .collect()
}

fn parse_workspace_array(toml_text: &str, key: &str) -> Vec<String> {
    let mut section = String::new();
    let mut collecting = false;
    let mut buf = String::new();

    for raw_line in toml_text.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && !collecting {
            if let Some(header) = parse_table_header(line) {
                section = header;
            }
            continue;
        }
        if section != "workspace" {
            continue;
        }
        if !collecting {
            if let Some((k, v)) = split_kv(line)
                && normalize_dotted_key(k) == key
            {
                buf.push_str(v);
                if v.contains(']') {
                    break;
                }
                collecting = true;
            }
        } else {
            buf.push(' ');
            buf.push_str(line);
            if line.contains(']') {
                break;
            }
        }
    }

    let inner = buf
        .trim()
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or("");
    split_inline_entries(inner)
        .into_iter()
        .filter_map(parse_toml_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        CRATE_MAP, Layer, ManifestMetadata, TOMBSTONES, parse_manifest_metadata, validate_table,
    };

    #[test]
    fn crate_map_table_passes_internal_invariants() {
        assert_eq!(validate_table(CRATE_MAP, TOMBSTONES), Ok(()));
    }

    #[test]
    fn parse_table_inline_and_dotted_metadata_forms() {
        let standard = "[package]\nname = \"frob-contact\"\n[package.metadata.frob]\nlayer = 3 # compute\nrank = 2\n";
        assert_eq!(
            parse_manifest_metadata(standard),
            Ok(ManifestMetadata { layer: Layer::Core(3), rank: 2 })
        );
        let inline = "[package]\nname = \"frob-spark-cuda\"\n[package.metadata]\nfrob = { layer = \"annex\", rank = 1 }\n";
        assert_eq!(
            parse_manifest_metadata(inline),
            Ok(ManifestMetadata { layer: Layer::Annex, rank: 1 })
        );
        let dotted = "[package]\nname = \"frob-core\"\n[package.metadata]\nfrob.layer = 0\nfrob.rank = 1\n";
        assert_eq!(
            parse_manifest_metadata(dotted),
            Ok(ManifestMetadata { layer: Layer::Core(0), rank: 1 })
        );
    }
}
