#![deny(unsafe_code)]

//! Plan-time and dated-revision gate count expectation loader and meta-assertion (`N38b-18`).
//!
//! Reads `crates/frob-conformance/registry/gate_counts.toml` (`frob.gate_counts/1`) exclusively
//! through `frob_core::registry::toml` (zero dependencies outside `std` and `frob-core`).

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::registry::toml::{self, Fields, Spanned, Table, Value};

use crate::register::row::{BaseDocument, BaseRowError, KindCounts};

/// Expected schema identifier for `registry/gate_counts.toml`.
pub const GATE_COUNTS_SCHEMA: &str = "frob.gate_counts/1";

/// Workspace-relative path of the committed gate-count expectation file.
pub const GATE_COUNTS_PATH: &str = "crates/frob-conformance/registry/gate_counts.toml";

/// First-inventory plan-time counts (421 = 278 convergence + 111 kill + 32 admission),
/// plus the 112-vs-111 kill-gate document note and the multi-part gate counts (40 convergence
/// + 3 admission per `N40-02` step 5 / round-4 review note #1198).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanInventory {
    pub convergence: usize,
    pub kill: usize,
    pub admission: usize,
    pub total: usize,
    pub kill_intro_count: usize,
    pub kill_intro_note: String,
    pub multi_part_convergence: usize,
    pub multi_part_admission: usize,
    pub multi_part_note: String,
}

impl PlanInventory {
    /// Report the known document count delta between the register intro (112) and the
    /// first-inventory table (111) without silently choosing between them (`N38b-18`).
    #[must_use]
    pub fn kill_count_delta_report(&self) -> String {
        let diff = self.kill_intro_count.abs_diff(self.kill);
        format!(
            "plan inventory kill={} vs register intro kill={} (delta={}): {}",
            self.kill, self.kill_intro_count, diff, self.kill_intro_note
        )
    }
}

/// One dated revision of the expected committed `registries/gates.toml` counts and IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountRevision {
    pub date: String,
    pub note: String,
    pub convergence: usize,
    pub kill: usize,
    pub admission: usize,
    pub program: usize,
    pub total: usize,
    pub ids: Vec<String>,
}

impl CountRevision {
    #[must_use]
    pub const fn expected_counts(&self) -> KindCounts {
        KindCounts {
            convergence: self.convergence,
            kill: self.kill,
            admission: self.admission,
            program: self.program,
            total: self.total,
        }
    }
}

/// Result of comparing a [`BaseDocument`] against an expectation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountReport {
    pub expected: KindCounts,
    pub observed: KindCounts,
    pub revision_date: String,
}

/// Detailed mismatch reported when counts or gate identifiers differ from the expectation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountMismatch {
    pub expected: KindCounts,
    pub observed: KindCounts,
    pub extra_ids: Vec<String>,
    pub missing_ids: Vec<String>,
}

impl fmt::Display for CountMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "gate count mismatch: expected total={} (conv={}, kill={}, adm={}, prog={}), observed total={} (conv={}, kill={}, adm={}, prog={}); extra_ids={:?}; missing_ids={:?}",
            self.expected.total,
            self.expected.convergence,
            self.expected.kill,
            self.expected.admission,
            self.expected.program,
            self.observed.total,
            self.observed.convergence,
            self.observed.kill,
            self.observed.admission,
            self.observed.program,
            self.extra_ids,
            self.missing_ids,
        )
    }
}

impl std::error::Error for CountMismatch {}

/// Parsed `registry/gate_counts.toml` expectation file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateCountsExpectation {
    pub schema: String,
    pub plan_inventory: PlanInventory,
    pub revisions: Vec<CountRevision>,
}

impl GateCountsExpectation {
    /// Load from disk.
    pub fn load(path: &Path) -> Result<Self, BaseRowError> {
        let label = path.to_string_lossy();
        let bytes = fs::read(path)
            .map_err(|e| BaseRowError::new(&label, None, format!("read failed: {e}")))?;
        Self::parse_bytes(&bytes, &label)
    }

    /// Parse bytes strictly via `frob_core::registry::toml`.
    pub fn parse_bytes(bytes: &[u8], file: &str) -> Result<Self, BaseRowError> {
        let root = toml::parse(bytes).map_err(|e| BaseRowError::new(file, None, e.to_string()))?;
        let mut top = Fields::new(root, "gate_counts");
        let err = |e: toml::TomlError| BaseRowError::new(file, None, e.to_string());

        let schema = top.require_str("schema").map_err(err)?;
        if schema != GATE_COUNTS_SCHEMA {
            return Err(BaseRowError::new(
                file,
                None,
                format!("unexpected schema `{schema}` (expected `{GATE_COUNTS_SCHEMA}`)"),
            ));
        }

        let plan_tbl = top.require_table("plan_inventory").map_err(err)?;
        let plan_inventory = parse_plan_inventory(plan_tbl, file)?;

        let rev_arr = top.require_array("revision").map_err(err)?;
        if rev_arr.is_empty() {
            return Err(BaseRowError::new(
                file,
                None,
                "`revision` must contain at least one dated entry",
            ));
        }
        let mut revisions = Vec::with_capacity(rev_arr.len());
        for (idx, sp) in rev_arr.into_iter().enumerate() {
            let tbl = match sp.value {
                Value::Table(t) => t,
                other => {
                    return Err(BaseRowError::new(
                        file,
                        None,
                        format!(
                            "revision[{idx}] expected table, found {}",
                            other.type_name()
                        ),
                    ));
                }
            };
            revisions.push(parse_revision(tbl, idx, file)?);
        }

        top.finish().map_err(err)?;
        Ok(Self {
            schema,
            plan_inventory,
            revisions,
        })
    }

    /// Latest dated revision entry.
    #[must_use]
    pub fn latest_revision(&self) -> Option<&CountRevision> {
        self.revisions.last()
    }

    /// Assert `doc` matches the latest dated revision in both per-kind counts and exact ID set.
    pub fn check_latest(&self, doc: &BaseDocument) -> Result<CountReport, CountMismatch> {
        let rev = self
            .latest_revision()
            .expect("validated non-empty revisions");
        compare_counts_and_ids(doc, rev.expected_counts(), Some(&rev.ids), &rev.date)
    }

    /// Assert `doc` matches the first-inventory plan-time counts (`278 + 111 + 32 = 421`).
    pub fn check_plan_inventory(&self, doc: &BaseDocument) -> Result<CountReport, CountMismatch> {
        let expected = KindCounts {
            convergence: self.plan_inventory.convergence,
            kill: self.plan_inventory.kill,
            admission: self.plan_inventory.admission,
            program: 0,
            total: self.plan_inventory.total,
        };
        compare_counts_and_ids(doc, expected, None, "plan-inventory")
    }
}

fn compare_counts_and_ids(
    doc: &BaseDocument,
    expected: KindCounts,
    expected_ids: Option<&[String]>,
    revision_date: &str,
) -> Result<CountReport, CountMismatch> {
    let observed = doc.counts_by_kind();
    let mut extra_ids = Vec::new();
    let mut missing_ids = Vec::new();

    if let Some(exp_list) = expected_ids {
        let exp_set: BTreeSet<&str> = exp_list.iter().map(String::as_str).collect();
        let obs_set: BTreeSet<&str> = doc.gates.iter().map(|g| g.id.as_str()).collect();
        for &id in obs_set.difference(&exp_set) {
            extra_ids.push(id.to_owned());
        }
        for &id in exp_set.difference(&obs_set) {
            missing_ids.push(id.to_owned());
        }
    }

    if observed != expected || !extra_ids.is_empty() || !missing_ids.is_empty() {
        return Err(CountMismatch {
            expected,
            observed,
            extra_ids,
            missing_ids,
        });
    }

    Ok(CountReport {
        expected,
        observed,
        revision_date: revision_date.to_owned(),
    })
}

fn non_neg_usize(v: i64, file: &str, field: &str) -> Result<usize, BaseRowError> {
    usize::try_from(v).map_err(|_| {
        BaseRowError::new(
            file,
            None,
            format!("`{field}` must be a non-negative integer, got {v}"),
        )
    })
}

fn parse_plan_inventory(tbl: Table, file: &str) -> Result<PlanInventory, BaseRowError> {
    let mut f = Fields::new(tbl, "plan_inventory");
    let err = |e: toml::TomlError| BaseRowError::new(file, None, e.to_string());

    let convergence = non_neg_usize(
        f.require_int("convergence").map_err(err)?,
        file,
        "convergence",
    )?;
    let kill = non_neg_usize(f.require_int("kill").map_err(err)?, file, "kill")?;
    let admission = non_neg_usize(f.require_int("admission").map_err(err)?, file, "admission")?;
    let total = non_neg_usize(f.require_int("total").map_err(err)?, file, "total")?;
    if convergence + kill + admission != total {
        return Err(BaseRowError::new(
            file,
            None,
            format!(
                "plan_inventory total={total} does not equal convergence({convergence})+kill({kill})+admission({admission})"
            ),
        ));
    }
    let kill_intro_count = non_neg_usize(
        f.require_int("kill_intro_count").map_err(err)?,
        file,
        "kill_intro_count",
    )?;
    let kill_intro_note = f.require_str("kill_intro_note").map_err(err)?;
    let multi_part_convergence = non_neg_usize(
        f.require_int("multi_part_convergence").map_err(err)?,
        file,
        "multi_part_convergence",
    )?;
    let multi_part_admission = non_neg_usize(
        f.require_int("multi_part_admission").map_err(err)?,
        file,
        "multi_part_admission",
    )?;
    let multi_part_note = f.require_str("multi_part_note").map_err(err)?;
    f.finish().map_err(err)?;

    Ok(PlanInventory {
        convergence,
        kill,
        admission,
        total,
        kill_intro_count,
        kill_intro_note,
        multi_part_convergence,
        multi_part_admission,
        multi_part_note,
    })
}

fn parse_revision(tbl: Table, idx: usize, file: &str) -> Result<CountRevision, BaseRowError> {
    let mut f = Fields::new(tbl, format!("revision[{idx}]"));
    let err = |e: toml::TomlError| BaseRowError::new(file, None, e.to_string());

    let date = f.require_str("date").map_err(err)?;
    if date.trim().is_empty() {
        return Err(BaseRowError::new(
            file,
            None,
            "revision `date` must be non-empty",
        ));
    }
    let note = f.require_str("note").map_err(err)?;
    let convergence = non_neg_usize(
        f.require_int("convergence").map_err(err)?,
        file,
        "convergence",
    )?;
    let kill = non_neg_usize(f.require_int("kill").map_err(err)?, file, "kill")?;
    let admission = non_neg_usize(f.require_int("admission").map_err(err)?, file, "admission")?;
    let program = non_neg_usize(f.require_int("program").map_err(err)?, file, "program")?;
    let total = non_neg_usize(f.require_int("total").map_err(err)?, file, "total")?;
    if convergence + kill + admission + program != total {
        return Err(BaseRowError::new(
            file,
            None,
            format!("revision[{idx}] total={total} does not equal sum of kind counts"),
        ));
    }
    let raw_ids = f.require_array("ids").map_err(err)?;
    let ids = extract_ids(raw_ids, file, idx)?;
    if ids.len() != total {
        return Err(BaseRowError::new(
            file,
            None,
            format!(
                "revision[{idx}] ids.len()={} does not equal total={total}",
                ids.len()
            ),
        ));
    }
    f.finish().map_err(err)?;

    Ok(CountRevision {
        date,
        note,
        convergence,
        kill,
        admission,
        program,
        total,
        ids,
    })
}

fn extract_ids(
    items: Vec<Spanned<Value>>,
    file: &str,
    idx: usize,
) -> Result<Vec<String>, BaseRowError> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::with_capacity(items.len());
    for sp in items {
        let s = match sp.value {
            Value::Str(s) => s,
            other => {
                return Err(BaseRowError::new(
                    file,
                    None,
                    format!(
                        "revision[{idx}].ids expected string, found {}",
                        other.type_name()
                    ),
                ));
            }
        };
        if !seen.insert(s.clone()) {
            return Err(BaseRowError::new(
                file,
                None,
                format!("revision[{idx}].ids contains duplicate `{s}`"),
            ));
        }
        out.push(s);
    }
    Ok(out)
}
