#![deny(unsafe_code)]

//! Deterministic test selection across the four core filter forms plus
//! tier-registered filter extensions (`N38a-02`, round-4 review notes 1 & 3).
//!
//! Core filter forms:
//! 1. Gate: `GATE-...`, `INT-...`, `KILL-...`, `PROG-...`, or `gate:<id>`
//! 2. Row: `KA-...`, `MR-...`, `CX-...`, `NC-...`, `law:...`, `ka:...`, etc., or `row:<id>`
//! 3. Site: `@host`, `@auditor`, `@referee`, `@strict`, `@lockstep`, `@cuda`, `@metal`, `@daemon`
//! 4. Milestone: `due:M0`..`due:M8`
//!
//! Extension point:
//! Tier beads register [`FilterExtension`]s on their [`TierTable`] (for example
//! `battery:<name>` on `g1_known_answers` in `N38a-22`/`BD-146`, or
//! `<family>[@<site>]` with `--count N` on `g4_chaos` in `N38a-23`).
//! Every filter form enforces the same fail-closed rule: an empty selection is
//! an error, never a pass.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::{Entry, Milestone, RunOptions, Site, TierTable};

/// Result returned by a tier-registered [`FilterExtension`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterResolution {
    /// Selected entry IDs (must be non-empty; empty returns `SelectError::EmptySelection`).
    pub entry_ids: Vec<String>,
    /// Optional reduced count override parsed from extension args (e.g. `--count N`).
    pub count_override: Option<u64>,
}

/// Resolver function signature for a tier-registered filter extension.
pub type FilterResolverFn =
    fn(arg: &str, extra_args: &[String], table: &TierTable) -> Result<FilterResolution, String>;

/// Tier-level filter extension (`N38a-02` round-4 review notes 1 & 3).
///
/// - When `prefix` is non-empty (e.g. `"battery:"`), matches filters starting
///   with `prefix` and passes the remainder after `prefix` to `resolve`.
/// - When `prefix` is empty (`""`), acts as a tier-specific fallback matcher
///   (for example G4 `<family>[@<site>]`) invoked only when no core form or
///   prefixed extension matched.
#[derive(Clone, Copy)]
pub struct FilterExtension {
    pub name: &'static str,
    pub prefix: &'static str,
    pub resolve: FilterResolverFn,
}

impl fmt::Debug for FilterExtension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterExtension")
            .field("name", &self.name)
            .field("prefix", &self.prefix)
            .finish()
    }
}

/// Parsed filter token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    Gate(String),
    Row(String),
    Site(Site),
    Milestone(Milestone),
    Custom {
        extension: &'static str,
        raw: String,
    },
}

impl Filter {
    /// Canonical string label recorded in the receipt's `filter` field.
    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::Gate(g) => g.clone(),
            Self::Row(r) => r.clone(),
            Self::Site(s) => format!("@{}", s.as_str()),
            Self::Milestone(m) => format!("due:{}", m.as_str()),
            Self::Custom { raw, .. } => raw.clone(),
        }
    }
}

/// Selection error (empty selection or invalid filter; never treated as pass).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectError {
    InvalidFilter { filter: String, detail: String },
    EmptySelection { filter: String },
    ExtensionError { extension: String, detail: String },
    UnknownEntryIdFromExtension { extension: String, id: String },
}

impl fmt::Display for SelectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFilter { filter, detail } => {
                write!(f, "invalid tier filter `{filter}`: {detail}")
            }
            Self::EmptySelection { filter } => {
                write!(
                    f,
                    "empty selection for filter `{filter}` (an empty selection is an error, not a pass)"
                )
            }
            Self::ExtensionError { extension, detail } => {
                write!(f, "filter extension `{extension}` failed: {detail}")
            }
            Self::UnknownEntryIdFromExtension { extension, id } => {
                write!(
                    f,
                    "filter extension `{extension}` returned unknown entry id `{id}`"
                )
            }
        }
    }
}

impl std::error::Error for SelectError {}

/// Selected entries ordered deterministically (weakest site first within each row).
#[derive(Debug, Clone)]
pub struct SelectedEntries {
    pub filter_label: String,
    pub entries: Vec<Entry>,
    pub count_override: Option<u64>,
}

/// Extract the logical row key of an entry ID (everything before `@<site>`).
#[must_use]
pub fn row_key(entry_id: &str) -> &str {
    entry_id.split_once('@').map_or(entry_id, |(head, _)| head)
}

/// Extract the bare row stem by stripping an optional `<prefix>:` and `@<site>`.
/// E.g., `"ka:KA-501@host"` -> `"KA-501"`, `"mr:MR-06@strict"` -> `"MR-06"`,
/// `"law:level-series@host"` -> `"level-series"`.
#[must_use]
pub fn row_stem(entry_id: &str) -> &str {
    let key = row_key(entry_id);
    key.split_once(':').map_or(key, |(_, rest)| rest)
}

/// Parse a filter argument against the four core forms and any prefixed
/// extensions registered on `table`.
pub fn parse_filter(raw: &str, table: &TierTable) -> Result<Filter, SelectError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(SelectError::InvalidFilter {
            filter: raw.to_owned(),
            detail: "filter must not be empty".to_owned(),
        });
    }

    // 1. Site filter: `@<site>`.
    if let Some(site_str) = trimmed.strip_prefix('@') {
        let Some(site) = Site::parse(site_str) else {
            return Err(SelectError::InvalidFilter {
                filter: trimmed.to_owned(),
                detail: format!(
                    "unknown site `{site_str}` (expected one of: host, auditor, referee, strict, lockstep, cuda, metal, daemon)"
                ),
            });
        };
        return Ok(Filter::Site(site));
    }

    // 2. Milestone filter: `due:M<n>`.
    if let Some(m_str) = trimmed.strip_prefix("due:") {
        let Some(m) = Milestone::parse(m_str) else {
            return Err(SelectError::InvalidFilter {
                filter: trimmed.to_owned(),
                detail: format!("unknown milestone `{m_str}` (expected M0..M8)"),
            });
        };
        return Ok(Filter::Milestone(m));
    }

    // 3. Explicit `gate:` or `row:` prefix.
    if let Some(g) = trimmed.strip_prefix("gate:") {
        if g.is_empty() {
            return Err(SelectError::InvalidFilter {
                filter: trimmed.to_owned(),
                detail: "gate filter requires a non-empty identifier".to_owned(),
            });
        }
        return Ok(Filter::Gate(g.to_owned()));
    }
    if let Some(r) = trimmed.strip_prefix("row:") {
        if r.is_empty() {
            return Err(SelectError::InvalidFilter {
                filter: trimmed.to_owned(),
                detail: "row filter requires a non-empty identifier".to_owned(),
            });
        }
        return Ok(Filter::Row(r.to_owned()));
    }

    // 4. Registered prefixed extensions (e.g. `battery:<name>`).
    for ext in table.extensions() {
        if !ext.prefix.is_empty() && trimmed.starts_with(ext.prefix) {
            return Ok(Filter::Custom {
                extension: ext.name,
                raw: trimmed.to_owned(),
            });
        }
    }

    // 5. Implicit Gate identifier forms (`GATE-`, `INT-`, `KILL-`, `PROG-`).
    if is_gate_id_form(trimmed) {
        return Ok(Filter::Gate(trimmed.to_owned()));
    }

    // 6. Implicit Row identifier forms (`KA-`, `MR-`, `CX-`, `NC-`, `law:`, `ka:`,
    //    `mr:`, `cx:`, `nc:`, `par:`, `ch:`, `det:`, or exact entry id in table).
    if is_row_id_form(trimmed) || table.get(trimmed).is_some() {
        return Ok(Filter::Row(trimmed.to_owned()));
    }

    // 7. Registered unprefixed fallback extension (e.g. G4 `<family>[@<site>]`).
    if let Some(ext) = table.extensions().iter().find(|e| e.prefix.is_empty()) {
        return Ok(Filter::Custom {
            extension: ext.name,
            raw: trimmed.to_owned(),
        });
    }

    Err(SelectError::InvalidFilter {
        filter: trimmed.to_owned(),
        detail: "unrecognized filter form (expected gate, row, @site, due:M<n>, or a registered tier extension)".to_owned(),
    })
}

/// Select and deterministically order entries from `table` under `opts`.
///
/// Within every logical row, entries are strictly ordered weakest-site-first
/// (`host < auditor < referee < strict < lockstep < cuda < metal < daemon`).
/// Two runs with the same seed, filter, and table always produce the exact
/// same order. An empty selection always returns `Err(SelectError::EmptySelection)`.
pub fn select_entries(
    table: &TierTable,
    opts: &RunOptions,
) -> Result<SelectedEntries, SelectError> {
    let mut count_override = opts.count_override;

    let (filter_label, matched): (String, Vec<Entry>) = match opts.filter.as_deref() {
        None => {
            let label = format!("due:{}", opts.milestone.as_str());
            let items: Vec<Entry> = table
                .entries()
                .iter()
                .copied()
                .filter(|e| e.milestone <= opts.milestone)
                .collect();
            (label, items)
        }
        Some(raw) => {
            let parsed = parse_filter(raw, table)?;
            let label = parsed.label();
            let items = match parsed {
                Filter::Gate(ref gate_id) => table
                    .entries()
                    .iter()
                    .copied()
                    .filter(|e| e.gates.contains(&gate_id.as_str()))
                    .collect(),
                Filter::Row(ref row_id) => table
                    .entries()
                    .iter()
                    .copied()
                    .filter(|e| {
                        e.id == row_id || row_key(e.id) == row_id || row_stem(e.id) == row_id
                    })
                    .collect(),
                Filter::Site(site) => table
                    .entries()
                    .iter()
                    .copied()
                    .filter(|e| e.site == site)
                    .collect(),
                Filter::Milestone(m) => table
                    .entries()
                    .iter()
                    .copied()
                    .filter(|e| e.milestone <= m)
                    .collect(),
                Filter::Custom {
                    extension,
                    raw: ref raw_tok,
                } => {
                    let Some(ext) = table.extensions().iter().find(|e| e.name == extension) else {
                        return Err(SelectError::ExtensionError {
                            extension: extension.to_owned(),
                            detail: "registered extension not found".to_owned(),
                        });
                    };
                    let arg = raw_tok.strip_prefix(ext.prefix).unwrap_or(raw_tok);
                    let res = (ext.resolve)(arg, &opts.extra_args, table).map_err(|detail| {
                        SelectError::ExtensionError {
                            extension: extension.to_owned(),
                            detail,
                        }
                    })?;
                    if let Some(ov) = res.count_override {
                        count_override = Some(ov);
                    }
                    let mut seen_ids = BTreeSet::new();
                    let mut out = Vec::new();
                    for id in res.entry_ids {
                        if !seen_ids.insert(id.clone()) {
                            continue;
                        }
                        let Some(entry) = table.get(&id) else {
                            return Err(SelectError::UnknownEntryIdFromExtension {
                                extension: extension.to_owned(),
                                id,
                            });
                        };
                        out.push(*entry);
                    }
                    out
                }
            };
            (label, items)
        }
    };

    if matched.is_empty() {
        return Err(SelectError::EmptySelection {
            filter: filter_label,
        });
    }

    let ordered = order_entries_weakest_site_first(matched);
    Ok(SelectedEntries {
        filter_label,
        entries: ordered,
        count_override,
    })
}

/// Group entries by logical row (`row_key`), sort rows deterministically by
/// `(row_key)` and order entries within every row strictly weakest-site-first
/// (`Site::rank()`).
fn order_entries_weakest_site_first(entries: Vec<Entry>) -> Vec<Entry> {
    let mut by_row: BTreeMap<&'static str, Vec<Entry>> = BTreeMap::new();
    for entry in entries {
        by_row.entry(row_key(entry.id)).or_default().push(entry);
    }
    let mut out = Vec::new();
    for (_, mut group) in by_row {
        group.sort_by(|a, b| {
            a.site
                .rank()
                .cmp(&b.site.rank())
                .then_with(|| a.id.cmp(b.id))
        });
        out.extend(group);
    }
    out
}

fn is_gate_id_form(s: &str) -> bool {
    s.starts_with("GATE-")
        || s.starts_with("INT-")
        || s.starts_with("KILL-")
        || s.starts_with("PROG-")
}

fn is_row_id_form(s: &str) -> bool {
    s.starts_with("KA-")
        || s.starts_with("MR-")
        || s.starts_with("CX-")
        || s.starts_with("NC-")
        || s.starts_with("law:")
        || s.starts_with("ka:")
        || s.starts_with("mr:")
        || s.starts_with("cx:")
        || s.starts_with("nc:")
        || s.starts_with("par:")
        || s.starts_with("ch:")
        || s.starts_with("det:")
}
