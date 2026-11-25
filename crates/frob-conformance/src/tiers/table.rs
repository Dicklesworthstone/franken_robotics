#![deny(unsafe_code)]

//! Validated static entry tables per tier (`N38a-02`).
//!
//! Enforces single-tier, single-site registration: rejects duplicate entry IDs,
//! wrong-tier entries, missing/mismatched `@<site>` ID suffixes, and cross-tier
//! duplicate registrations.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::{ControlSpec, Entry, FilterExtension, Site, Tier};

/// Error returned when constructing or cross-checking a [`TierTable`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableError {
    EmptyEntryId,
    EntryWrongTier {
        id: String,
        expected: Tier,
        found: Tier,
    },
    DuplicateEntryId {
        id: String,
    },
    SiteSuffixMismatch {
        id: String,
        declared: Site,
        suffix: String,
    },
    EntryRegisteredInTwoTiers {
        id: String,
        first_tier: Tier,
        second_tier: Tier,
    },
    DuplicateExtensionPrefix {
        prefix: String,
    },
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEntryId => f.write_str("tier entry id must not be empty"),
            Self::EntryWrongTier {
                id,
                expected,
                found,
            } => write!(
                f,
                "entry `{id}` declares tier `{found}` inside `{expected}` table"
            ),
            Self::DuplicateEntryId { id } => {
                write!(f, "duplicate entry id `{id}` in tier table")
            }
            Self::SiteSuffixMismatch {
                id,
                declared,
                suffix,
            } => write!(
                f,
                "entry `{id}` has site suffix `@{suffix}` which disagrees with declared site `{declared}`"
            ),
            Self::EntryRegisteredInTwoTiers {
                id,
                first_tier,
                second_tier,
            } => write!(
                f,
                "entry `{id}` is registered in two tiers (`{first_tier}` and `{second_tier}`)"
            ),
            Self::DuplicateExtensionPrefix { prefix } => {
                write!(f, "duplicate filter extension prefix `{prefix}`")
            }
        }
    }
}

impl std::error::Error for TableError {}

/// Validated collection of [`Entry`] items, same-run [`ControlSpec`]s, and
/// tier-specific [`FilterExtension`]s for one [`Tier`].
#[derive(Clone)]
pub struct TierTable {
    tier: Tier,
    entries: Vec<Entry>,
    controls: Vec<ControlSpec>,
    extensions: Vec<FilterExtension>,
}

impl fmt::Debug for TierTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TierTable")
            .field("tier", &self.tier)
            .field("entries", &self.entries.len())
            .field("controls", &self.controls.len())
            .field("extensions", &self.extensions.len())
            .finish()
    }
}

impl TierTable {
    /// Construct and validate a tier table from static entries.
    pub fn new(tier: Tier, entries: &[Entry]) -> Result<Self, TableError> {
        let mut seen = BTreeSet::new();
        for entry in entries {
            validate_entry(tier, entry)?;
            if !seen.insert(entry.id) {
                return Err(TableError::DuplicateEntryId {
                    id: entry.id.to_owned(),
                });
            }
        }
        Ok(Self {
            tier,
            entries: entries.to_vec(),
            controls: Vec::new(),
            extensions: Vec::new(),
        })
    }

    /// Attach runnable negative controls to execute in the same tier run.
    #[must_use]
    pub fn with_controls(mut self, controls: &[ControlSpec]) -> Self {
        self.controls.extend_from_slice(controls);
        self
    }

    /// Register tier-specific filter extensions (`N38a-02` round-4 review notes 1 & 3),
    /// e.g. `battery:<name>` on `g1_known_answers` or `<family>[@<site>]` on `g4_chaos`.
    pub fn with_extensions(mut self, extensions: &[FilterExtension]) -> Result<Self, TableError> {
        let mut prefixes: BTreeSet<&str> = self.extensions.iter().map(|e| e.prefix).collect();
        for ext in extensions {
            if !ext.prefix.is_empty() && !prefixes.insert(ext.prefix) {
                return Err(TableError::DuplicateExtensionPrefix {
                    prefix: ext.prefix.to_owned(),
                });
            }
            self.extensions.push(*ext);
        }
        Ok(self)
    }

    #[must_use]
    pub const fn tier(&self) -> Tier {
        self.tier
    }

    #[must_use]
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    #[must_use]
    pub fn controls(&self) -> &[ControlSpec] {
        &self.controls
    }

    #[must_use]
    pub fn extensions(&self) -> &[FilterExtension] {
        &self.extensions
    }

    /// Look up an entry by its exact identifier.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id == id)
    }
}

/// Validate a single [`Entry`] against its owning [`Tier`].
pub fn validate_entry(expected_tier: Tier, entry: &Entry) -> Result<(), TableError> {
    let trimmed = entry.id.trim();
    if trimmed.is_empty() || trimmed != entry.id {
        return Err(TableError::EmptyEntryId);
    }
    if entry.tier != expected_tier {
        return Err(TableError::EntryWrongTier {
            id: entry.id.to_owned(),
            expected: expected_tier,
            found: entry.tier,
        });
    }
    // If the entry ID carries an `@<suffix>`, require exactly one `@` and require
    // `<suffix>` to match `entry.site.as_str()`.
    if let Some((head, suffix)) = entry.id.split_once('@') {
        if head.is_empty() || suffix.contains('@') || suffix != entry.site.as_str() {
            return Err(TableError::SiteSuffixMismatch {
                id: entry.id.to_owned(),
                declared: entry.site,
                suffix: suffix.to_owned(),
            });
        }
    }
    Ok(())
}

/// Verify that no entry identifier appears in more than one [`TierTable`].
pub fn validate_disjoint_tiers(tables: &[&TierTable]) -> Result<(), TableError> {
    let mut owner: BTreeMap<&str, Tier> = BTreeMap::new();
    for table in tables {
        for entry in table.entries() {
            if let Some(&prev_tier) = owner.get(entry.id) {
                if prev_tier != table.tier() {
                    return Err(TableError::EntryRegisteredInTwoTiers {
                        id: entry.id.to_owned(),
                        first_tier: prev_tier,
                        second_tier: table.tier(),
                    });
                }
                return Err(TableError::DuplicateEntryId {
                    id: entry.id.to_owned(),
                });
            }
            owner.insert(entry.id, table.tier());
        }
    }
    Ok(())
}
