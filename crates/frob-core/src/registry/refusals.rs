//! Refusal-registry loader (`registries/refusals.toml`) on `registry::toml` (BD-005).
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::refusal::{RefusalCode, RetryPolicy};
use crate::registry::toml::{self, Fields, Spanned, Table, TomlError, Value};

pub const EMBEDDED_REFUSALS_TOML: &str = include_str!("../../../../registries/refusals.toml");
pub const CODE_GEN_MARKER: &str = "// @generated from registries/refusals.toml by frob_core::registry::refusals::render_refusal_code_enum (N34p4-03 check-generated); do not edit by hand.";

const FIXED_EXIT_TABLE: &[(&str, u8)] = &[
    ("1xx", 3),
    ("2xx", 3),
    ("3xx", 3),
    ("4xx", 3),
    ("5xx", 3),
    ("6xx", 6),
    ("70x", 4),
    ("72x", 5),
    ("73x", 5),
    ("8xx", 3),
    ("9xx", 9),
];
const OPTIONAL_7XX_DECADES: &[&str] = &["71x", "74x", "75x", "76x", "77x", "78x", "79x"];
const VALID_REFUSAL_EXITS: &[u8] = &[3, 4, 5, 6, 7, 9];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusalRegistryError {
    Toml(TomlError),
    InvalidSchema {
        observed: String,
    },
    InvalidRefusalCode {
        code: String,
        line: u32,
        column: u32,
    },
    DuplicateRefusalCode {
        code: String,
        line: u32,
        column: u32,
    },
    UnknownRefusalCode {
        code: String,
        context: String,
    },
    InvalidRetry {
        code: String,
        value: String,
        line: u32,
        column: u32,
    },
    ExitMismatch {
        row: String,
        rule_entry: String,
        observed: u8,
        expected: Option<u8>,
        reason: String,
    },
    OverlappingExceptionList {
        code: String,
        first_list: &'static str,
        second_list: &'static str,
    },
    ExceptionCodeMissing {
        list: &'static str,
        code: String,
    },
    EmptyRaisedBy {
        code: String,
    },
    EmptyRepairs {
        code: String,
    },
}

impl From<TomlError> for RefusalRegistryError {
    fn from(e: TomlError) -> Self {
        Self::Toml(e)
    }
}

impl fmt::Display for RefusalRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml(e) => fmt::Display::fmt(e, f),
            Self::InvalidSchema { observed } => {
                write!(f, "invalid refusal registry schema `{observed}`")
            }
            Self::InvalidRefusalCode { code, line, column } => {
                write!(f, "{line}:{column}: invalid refusal code `{code}`")
            }
            Self::DuplicateRefusalCode { code, line, column } => {
                write!(f, "{line}:{column}: duplicate refusal code `{code}`")
            }
            Self::UnknownRefusalCode { code, context } => {
                write!(f, "unknown refusal code `{code}` in {context}")
            }
            Self::InvalidRetry {
                code,
                value,
                line,
                column,
            } => write!(f, "{line}:{column}: invalid retry `{value}` for `{code}`"),
            Self::ExitMismatch {
                row,
                rule_entry,
                observed,
                expected,
                reason,
            } => write!(
                f,
                "exit mismatch for `{row}` under `{rule_entry}`: observed {observed}, expected {expected:?} ({reason})"
            ),
            Self::OverlappingExceptionList {
                code,
                first_list,
                second_list,
            } => write!(
                f,
                "code `{code}` in both `{first_list}` and `{second_list}`"
            ),
            Self::ExceptionCodeMissing { list, code } => {
                write!(f, "exception list `{list}` names missing code `{code}`")
            }
            Self::EmptyRaisedBy { code } => write!(f, "refusal `{code}` has empty `raised_by`"),
            Self::EmptyRepairs { code } => write!(f, "refusal `{code}` has empty `repairs`"),
        }
    }
}
impl std::error::Error for RefusalRegistryError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExitRule {
    pub ranges: BTreeMap<String, u8>,
    pub conflict: BTreeSet<u16>,
    pub integrity: BTreeSet<u16>,
    pub capability: BTreeSet<u16>,
}

impl ExitRule {
    pub fn expected_exit(&self, num: u16) -> Result<(u8, String), RefusalRegistryError> {
        if self.conflict.contains(&num) {
            return Ok((7, "exit_rule.conflict".into()));
        }
        if self.integrity.contains(&num) {
            return Ok((9, "exit_rule.integrity".into()));
        }
        if self.capability.contains(&num) {
            return Ok((5, "exit_rule.capability".into()));
        }
        let key = if (700..=799).contains(&num) {
            format!("7{}x", (num - 700) / 10)
        } else {
            format!("{}xx", num / 100)
        };
        self.ranges
            .get(&key)
            .copied()
            .map(|e| (e, format!("exit_rule.{key}")))
            .ok_or_else(|| RefusalRegistryError::ExitMismatch {
                row: format_code(num),
                rule_entry: format!("exit_rule.{key}"),
                observed: 0,
                expected: None,
                reason: format!("no `[exit_rule]` entry covers `{key}`"),
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Milestone {
    M0,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
}

impl Milestone {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M0 => "M0",
            Self::M1 => "M1",
            Self::M2 => "M2",
            Self::M3 => "M3",
            Self::M4 => "M4",
            Self::M5 => "M5",
            Self::M6 => "M6",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "M0" => Some(Self::M0),
            "M1" => Some(Self::M1),
            "M2" => Some(Self::M2),
            "M3" => Some(Self::M3),
            "M4" => Some(Self::M4),
            "M5" => Some(Self::M5),
            "M6" => Some(Self::M6),
            _ => None,
        }
    }
}

impl fmt::Display for Milestone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Raising-site verification record for milestone-staged refusal checks (`N21-REF`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RaisingSiteRecord {
    pub first_milestone: Milestone,
    pub has_raising_test: bool,
    pub ran_in_current_build: bool,
}

impl RaisingSiteRecord {
    #[must_use]
    pub const fn verified(first_milestone: Milestone) -> Self {
        Self {
            first_milestone,
            has_raising_test: true,
            ran_in_current_build: true,
        }
    }
    #[must_use]
    pub const fn missing(first_milestone: Milestone) -> Self {
        Self {
            first_milestone,
            has_raising_test: false,
            ran_in_current_build: false,
        }
    }
    #[must_use]
    pub const fn unexecuted(first_milestone: Milestone) -> Self {
        Self {
            first_milestone,
            has_raising_test: true,
            ran_in_current_build: false,
        }
    }
}

/// Result of checking refusal rows against a build milestone (`N21-REF`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DueSiteReport {
    pub verified: Vec<String>,
    pub pending: Vec<(String, Milestone)>,
}

impl DueSiteReport {
    #[must_use]
    pub fn pending_label(&self, code: &str) -> Option<String> {
        self.pending
            .iter()
            .find(|(c, _)| c == code)
            .map(|(_, m)| format!("pending ({m})"))
    }
}

/// Failure when a refusal code due at or before the build milestone lacks a verified raising test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DueSiteError {
    UnknownCode {
        code: String,
    },
    MissingRaisingTest {
        code: String,
        due: Milestone,
        current: Milestone,
    },
    UnexecutedRaisingTest {
        code: String,
        due: Milestone,
        current: Milestone,
    },
}

impl fmt::Display for DueSiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCode { code } => {
                write!(f, "due-site check names unregistered refusal `{code}`")
            }
            Self::MissingRaisingTest { code, due, current } => {
                write!(
                    f,
                    "refusal `{code}` (due {due}) is missing its raising test at {current}"
                )
            }
            Self::UnexecutedRaisingTest { code, due, current } => {
                write!(
                    f,
                    "refusal `{code}` (due {due}) has no recorded raising-test run in build {current}"
                )
            }
        }
    }
}
impl std::error::Error for DueSiteError {}

/// Trigger selector for `FROB-REF-601` structured repair order (`N21-REF`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Spark601Trigger {
    Device,
    Daemon,
}

/// Slab-class selector for `FROB-REF-602` structured repair order (`N21-REF`, `N36-REF`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Spark602SlabClass {
    Worlds,
    Rollout,
    Scenes,
    Frames,
    Adjoint,
}

impl Spark602SlabClass {
    pub const ALL: &'static [Self] = &[
        Self::Worlds,
        Self::Rollout,
        Self::Scenes,
        Self::Frames,
        Self::Adjoint,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Worlds => "worlds",
            Self::Rollout => "rollout",
            Self::Scenes => "scenes",
            Self::Frames => "frames",
            Self::Adjoint => "adjoint",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusalRow {
    pub code: String,
    pub number: u16,
    pub meaning: String,
    pub defined_in: String,
    pub exit: u8,
    pub retry: RetryPolicy,
    pub repairs: Vec<String>,
    pub raised_by: Vec<String>,
    pub related: Vec<String>,
}

impl RefusalRow {
    /// Extract the first due milestone declared as `(due Mn)` or `(due Mn/Mk)` at the end of `meaning`.
    #[must_use]
    pub fn due_milestone(&self) -> Option<Milestone> {
        let tail = self.meaning.rsplit_once("(due ")?.1;
        let inner = tail.strip_suffix(')')?;
        let first = inner.split('/').next()?;
        Milestone::parse(first)
    }

    /// Structured per-trigger repair order for `FROB-REF-601` (`N21-REF`).
    #[must_use]
    pub fn spark_601_repairs(&self, trigger: Spark601Trigger) -> Option<Vec<String>> {
        if self.number != 601 {
            return None;
        }
        match trigger {
            Spark601Trigger::Device => {
                let items: Vec<String> = self
                    .repairs
                    .iter()
                    .filter(|s| !s.contains("pointer: daemon"))
                    .cloned()
                    .collect();
                (!items.is_empty()).then_some(items)
            }
            Spark601Trigger::Daemon => {
                let raw = self
                    .repairs
                    .iter()
                    .find(|s| s.contains("pointer: daemon"))?;
                let after = raw.split_once("first ")?.1;
                let (first, second) = after.split_once(", then ")?;
                Some(vec![first.trim().to_owned(), second.trim().to_owned()])
            }
        }
    }

    /// Structured per-slab-class repair order for `FROB-REF-602` (`N21-REF`, `N36-REF`).
    #[must_use]
    pub fn spark_602_repairs(&self, class: Spark602SlabClass) -> Option<Vec<String>> {
        if self.number != 602 {
            return None;
        }
        let prefix = format!("{}: ", class.as_str());
        let raw = self.repairs.iter().find_map(|s| s.strip_prefix(&prefix))?;
        let items: Vec<String> = raw
            .split("; ")
            .map(|part| {
                let p = part.trim();
                match p.split_once(". ") {
                    Some((num, rest))
                        if !num.is_empty() && num.bytes().all(|b| b.is_ascii_digit()) =>
                    {
                        rest.trim().to_owned()
                    }
                    _ => p.to_owned(),
                }
            })
            .filter(|s| !s.is_empty())
            .collect();
        (!items.is_empty()).then_some(items)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetailRow {
    pub slug: String,
    pub detail: String,
    pub interim: Vec<String>,
    pub exit: u8,
    pub retry: RetryPolicy,
    pub pointer: String,
    pub observed: Vec<String>,
    pub repairs: Vec<String>,
    pub raised_by: Vec<String>,
    pub ledger: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusalRegistry {
    schema: String,
    exit_rule: ExitRule,
    rows: Vec<RefusalRow>,
    details: Vec<DetailRow>,
    by_number: BTreeMap<u16, usize>,
}

impl RefusalRegistry {
    pub fn load() -> Result<Self, RefusalRegistryError> {
        Self::from_bytes(EMBEDDED_REFUSALS_TOML.as_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, RefusalRegistryError> {
        let mut fields = Fields::new(toml::parse(bytes)?, "<root>");
        let schema = fields.require_str("schema")?;
        if schema != "frob.refusals/1" {
            return Err(fields.fail(RefusalRegistryError::InvalidSchema { observed: schema }));
        }
        let exit_rule_tbl = fields.require_table("exit_rule")?;
        let refusal_tables = fields.opt_array("refusal")?.unwrap_or_default();
        let detail_tables = fields.opt_array("detail")?.unwrap_or_default();
        fields.finish()?;
        let exit_rule = parse_exit_rule(exit_rule_tbl)?;

        let (mut rows, mut seen) = (Vec::with_capacity(refusal_tables.len()), BTreeSet::new());
        for (idx, sp) in refusal_tables.into_iter().enumerate() {
            let Value::Table(tbl) = sp.value else {
                return Err(TomlError::new(
                    sp.line,
                    sp.column,
                    toml::TomlErrorKind::TypeMismatch {
                        key: format!("refusal[{idx}]"),
                        expected: "table",
                        found: sp.value.type_name(),
                    },
                )
                .into());
            };
            let row = parse_refusal_row(tbl, &exit_rule, idx)?;
            if !seen.insert(row.number) {
                return Err(RefusalRegistryError::DuplicateRefusalCode {
                    code: row.code,
                    line: sp.line,
                    column: sp.column,
                });
            }
            rows.push(row);
        }
        for (list, set) in [
            ("conflict", &exit_rule.conflict),
            ("integrity", &exit_rule.integrity),
            ("capability", &exit_rule.capability),
        ] {
            for &n in set {
                if !seen.contains(&n) {
                    return Err(RefusalRegistryError::ExceptionCodeMissing {
                        list,
                        code: format_code(n),
                    });
                }
            }
        }
        let mut details = Vec::with_capacity(detail_tables.len());
        for (idx, sp) in detail_tables.into_iter().enumerate() {
            let Value::Table(tbl) = sp.value else {
                return Err(TomlError::new(
                    sp.line,
                    sp.column,
                    toml::TomlErrorKind::TypeMismatch {
                        key: format!("detail[{idx}]"),
                        expected: "table",
                        found: sp.value.type_name(),
                    },
                )
                .into());
            };
            let d = parse_detail_row(tbl, &seen, idx)?;
            details.push(d);
        }
        rows.sort_by_key(|r| (code_family(r.number), r.number));
        let by_number = rows
            .iter()
            .enumerate()
            .map(|(i, r)| (r.number, i))
            .collect();
        Ok(Self {
            schema,
            exit_rule,
            rows,
            details,
            by_number,
        })
    }

    pub fn load_with_verbs(
        refusals_bytes: &[u8],
        verbs_bytes: &[u8],
    ) -> Result<Self, RefusalRegistryError> {
        let reg = Self::from_bytes(refusals_bytes)?;
        reg.validate_verb_refusals(verbs_bytes)?;
        Ok(reg)
    }

    pub fn validate_verb_refusals(&self, verbs_bytes: &[u8]) -> Result<(), RefusalRegistryError> {
        let mut fields = Fields::new(toml::parse(verbs_bytes)?, "<verbs>");
        let _schema = fields.opt_str("schema")?;
        let verbs = fields.opt_array("verb")?.unwrap_or_default();
        fields.finish()?;
        for (idx, sp) in verbs.into_iter().enumerate() {
            let Value::Table(tbl) = sp.value else {
                return Err(TomlError::new(
                    sp.line,
                    sp.column,
                    toml::TomlErrorKind::TypeMismatch {
                        key: format!("verb[{idx}]"),
                        expected: "table",
                        found: sp.value.type_name(),
                    },
                )
                .into());
            };
            let mut vf = Fields::new(tbl, format!("verb[{idx}]"));
            let name = vf.require_str("name")?;
            let refs = vf
                .opt_array("refusals")?
                .map(|a| extract_str_arr("refusals", a))
                .transpose()
                .map_err(|e| vf.fail(e))?
                .unwrap_or_default();
            vf.finish()?;
            for code in refs {
                self.lookup(&code, &format!("verb `{name}`"))?;
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn schema(&self) -> &str {
        &self.schema
    }
    #[must_use]
    pub const fn exit_rule(&self) -> &ExitRule {
        &self.exit_rule
    }
    #[must_use]
    pub fn rows(&self) -> &[RefusalRow] {
        &self.rows
    }
    #[must_use]
    pub fn details(&self) -> &[DetailRow] {
        &self.details
    }
    #[must_use]
    pub fn row(&self, code: RefusalCode) -> Option<&RefusalRow> {
        self.by_number.get(&code.number()).map(|&i| &self.rows[i])
    }

    /// Enforce row-level completeness on production registries: every row must declare non-empty
    /// `raised_by` and non-empty `repairs` (N34-REF / N35-REF).
    pub fn validate_row_metadata(&self) -> Result<(), RefusalRegistryError> {
        for r in &self.rows {
            if r.raised_by.is_empty() {
                return Err(RefusalRegistryError::EmptyRaisedBy {
                    code: r.code.clone(),
                });
            }
            if r.repairs.is_empty() {
                return Err(RefusalRegistryError::EmptyRepairs {
                    code: r.code.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn lookup(&self, code: &str, context: &str) -> Result<&RefusalRow, RefusalRegistryError> {
        parse_code_number(code)
            .and_then(|n| self.by_number.get(&n))
            .map(|&i| &self.rows[i])
            .ok_or_else(|| RefusalRegistryError::UnknownRefusalCode {
                code: code.to_owned(),
                context: context.to_owned(),
            })
    }

    pub fn exit_for(&self, code: RefusalCode) -> Result<u8, RefusalRegistryError> {
        self.row(code)
            .map(|r| r.exit)
            .ok_or_else(|| RefusalRegistryError::UnknownRefusalCode {
                code: code.as_str().to_owned(),
                context: "exit_for".into(),
            })
    }

    /// Milestone-staged raising-site check (`N21-REF`): codes due at or before `current`
    /// must have a raising test that ran in the current build; codes due later are
    /// reported as `pending (Mn)` without failing the check.
    pub fn check_due_raising_sites(
        &self,
        current: Milestone,
        sites: &BTreeMap<String, RaisingSiteRecord>,
    ) -> Result<DueSiteReport, DueSiteError> {
        let mut verified = Vec::new();
        let mut pending = Vec::new();
        for (code, site) in sites {
            if self.lookup(code, "due-site-check").is_err() {
                return Err(DueSiteError::UnknownCode { code: code.clone() });
            }
            if site.first_milestone <= current {
                if !site.has_raising_test {
                    return Err(DueSiteError::MissingRaisingTest {
                        code: code.clone(),
                        due: site.first_milestone,
                        current,
                    });
                }
                if !site.ran_in_current_build {
                    return Err(DueSiteError::UnexecutedRaisingTest {
                        code: code.clone(),
                        due: site.first_milestone,
                        current,
                    });
                }
                verified.push(code.clone());
            } else {
                pending.push((code.clone(), site.first_milestone));
            }
        }
        Ok(DueSiteReport { verified, pending })
    }
}

/// Pure deterministic generator for `crates/frob-core/src/refusal/code_gen.rs` (N34p4-03).
#[must_use]
pub fn render_refusal_code_enum(reg: &RefusalRegistry) -> String {
    let mut out = format!("{CODE_GEN_MARKER}\n\nuse std::fmt;\n\n");
    out.push_str("/// Closed refusal code enum generated from `registries/refusals.toml`.\n///\n/// Every variant corresponds to one registered `[[refusal]]` row; an unregistered\n/// code cannot compile.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n#[repr(u16)]\npub enum RefusalCode {\n");
    for r in reg.rows() {
        out.push_str(&format!(
            "    /// {} ({}): {}\n    FrobRef{} = {},\n",
            r.code, r.defined_in, r.meaning, r.number, r.number
        ));
    }
    out.push_str("}\n\nimpl RefusalCode {\n    /// All registered refusal codes ordered by family, then numerically.\n    pub const ALL: &'static [Self] = &[\n");
    for r in reg.rows() {
        out.push_str(&format!("        Self::FrobRef{},\n", r.number));
    }
    out.push_str("    ];\n\n    /// Numeric code matching `FROB-REF-[1-9][0-9]{2,3}`.\n    #[must_use]\n    pub const fn number(self) -> u16 {\n        self as u16\n    }\n\n");
    out.push_str("    /// Family digit (`1..=9`).\n    #[must_use]\n    pub const fn family(self) -> u8 {\n        let n = self.number();\n        if n >= 1000 { (n / 1000) as u8 } else { (n / 100) as u8 }\n    }\n\n");
    out.push_str("    /// Canonical wire spelling `FROB-REF-[1-9][0-9]{2,3}`.\n    #[must_use]\n    pub const fn as_str(self) -> &'static str {\n        match self {\n");
    for r in reg.rows() {
        out.push_str(&format!(
            "            Self::FrobRef{} => \"{}\",\n",
            r.number, r.code
        ));
    }
    out.push_str("        }\n    }\n\n    /// Parse a canonical wire code `FROB-REF-[1-9][0-9]{2,3}`.\n    #[must_use]\n    pub fn parse(code: &str) -> Option<Self> {\n        match code {\n");
    for r in reg.rows() {
        out.push_str(&format!(
            "            \"{}\" => Some(Self::FrobRef{}),\n",
            r.code, r.number
        ));
    }
    out.push_str("            _ => None,\n        }\n    }\n}\n\nimpl fmt::Display for RefusalCode {\n    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n        f.write_str(self.as_str())\n    }\n}\n");
    out
}

fn parse_exit_rule(table: Table) -> Result<ExitRule, RefusalRegistryError> {
    let four_digit_keys: Vec<(String, u8)> = table
        .iter()
        .filter_map(|(k, _)| parse_four_digit_hundred_key(k).map(|fam| (k.clone(), fam)))
        .collect();
    let mut fields = Fields::new(table, "exit_rule");
    let mut ranges = BTreeMap::new();
    for &(k, fixed) in FIXED_EXIT_TABLE {
        if let Some(v) = fields.opt_int(k)? {
            let exit = checked_exit_u8(v, "exit_rule", k).map_err(|e| fields.fail(e))?;
            if exit != fixed {
                return Err(fields.fail(RefusalRegistryError::ExitMismatch {
                    row: "[exit_rule]".into(),
                    rule_entry: format!("exit_rule.{k}"),
                    observed: exit,
                    expected: Some(fixed),
                    reason: "contradicts fixed surface exit table".into(),
                }));
            }
            ranges.insert(k.to_owned(), exit);
        } else {
            ranges.insert(k.to_owned(), fixed);
        }
    }
    for &d in OPTIONAL_7XX_DECADES {
        if let Some(v) = fields.opt_int(d)? {
            let exit = checked_exit_u8(v, "exit_rule", d).map_err(|e| fields.fail(e))?;
            ranges.insert(d.to_owned(), exit);
        }
    }
    for (k, fam) in four_digit_keys {
        if let Some(v) = fields.opt_int(&k)? {
            let exit = checked_exit_u8(v, "exit_rule", &k).map_err(|e| fields.fail(e))?;
            if let Some(fixed) = fixed_family_exit(fam)
                && exit != fixed
            {
                return Err(fields.fail(RefusalRegistryError::ExitMismatch {
                    row: "[exit_rule]".into(),
                    rule_entry: format!("exit_rule.{k}"),
                    observed: exit,
                    expected: Some(fixed),
                    reason: "contradicts fixed surface exit table".into(),
                }));
            }
            ranges.insert(k, exit);
        }
    }
    let conflict = parse_exc_list(&mut fields, "conflict")?;
    let integrity = parse_exc_list(&mut fields, "integrity")?;
    let capability = parse_exc_list(&mut fields, "capability")?;
    fields.finish()?;
    for &(n1, s1, n2, s2) in &[
        ("conflict", &conflict, "integrity", &integrity),
        ("conflict", &conflict, "capability", &capability),
        ("integrity", &integrity, "capability", &capability),
    ] {
        for &c in s1 {
            if s2.contains(&c) {
                return Err(RefusalRegistryError::OverlappingExceptionList {
                    code: format_code(c),
                    first_list: n1,
                    second_list: n2,
                });
            }
        }
    }
    for (name, set, obs) in [
        ("conflict", &conflict, 7u8),
        ("capability", &capability, 5u8),
    ] {
        for &c in set {
            if code_family(c) == 9 {
                return Err(RefusalRegistryError::ExitMismatch {
                    row: format_code(c),
                    rule_entry: format!("exit_rule.{name}"),
                    observed: obs,
                    expected: Some(9),
                    reason: "9xx is fixed to exit 9".into(),
                });
            }
        }
    }
    Ok(ExitRule {
        ranges,
        conflict,
        integrity,
        capability,
    })
}

fn parse_exc_list(
    fields: &mut Fields,
    name: &'static str,
) -> Result<BTreeSet<u16>, RefusalRegistryError> {
    let mut set = BTreeSet::new();
    let Some(items) = fields.opt_array(name)? else {
        return Ok(set);
    };
    for sp in items {
        let num = match sp.value {
            Value::Int(n) if (100..=9999).contains(&n) => n as u16,
            Value::Int(n) => {
                return Err(fields.fail(RefusalRegistryError::InvalidRefusalCode {
                    code: n.to_string(),
                    line: sp.line,
                    column: sp.column,
                }));
            }
            Value::Str(s) => parse_code_number(&s).ok_or_else(|| {
                fields.fail(RefusalRegistryError::InvalidRefusalCode {
                    code: s,
                    line: sp.line,
                    column: sp.column,
                })
            })?,
            other => {
                return Err(fields.fail(
                    TomlError::new(
                        sp.line,
                        sp.column,
                        toml::TomlErrorKind::TypeMismatch {
                            key: format!("exit_rule.{name}"),
                            expected: "integer or code string",
                            found: other.type_name(),
                        },
                    )
                    .into(),
                ));
            }
        };
        set.insert(num);
    }
    Ok(set)
}

fn parse_refusal_row(
    table: Table,
    exit_rule: &ExitRule,
    idx: usize,
) -> Result<RefusalRow, RefusalRegistryError> {
    let (line, col) = table.span();
    let mut f = Fields::new(table, format!("refusal[{idx}]"));
    let code = f.require_str("code")?;
    let number = parse_code_number(&code).ok_or_else(|| {
        f.fail(RefusalRegistryError::InvalidRefusalCode {
            code: code.clone(),
            line,
            column: col,
        })
    })?;
    let (meaning, defined_in) = (f.require_str("meaning")?, f.require_str("defined_in")?);
    let exit_raw = f.require_int("exit")?;
    let exit = checked_exit_u8(exit_raw, &code, "exit").map_err(|e| f.fail(e))?;
    let retry_s = f.require_str("retry")?;
    let retry = RetryPolicy::parse(&retry_s).ok_or_else(|| {
        f.fail(RefusalRegistryError::InvalidRetry {
            code: code.clone(),
            value: retry_s,
            line,
            column: col,
        })
    })?;
    let repairs = f
        .opt_array("repairs")?
        .map(|a| extract_str_arr("repairs", a))
        .transpose()
        .map_err(|e| f.fail(e))?
        .unwrap_or_default();
    let raised_by = f
        .opt_array("raised_by")?
        .map(|a| extract_str_arr("raised_by", a))
        .transpose()
        .map_err(|e| f.fail(e))?
        .unwrap_or_default();
    let related = f
        .opt_array("related")?
        .map(|a| extract_str_arr("related", a))
        .transpose()
        .map_err(|e| f.fail(e))?
        .unwrap_or_default();
    f.finish()?;

    let (expected, rule_entry) = match exit_rule.expected_exit(number) {
        Ok(p) => p,
        Err(RefusalRegistryError::ExitMismatch {
            row,
            rule_entry,
            expected,
            reason,
            ..
        }) => {
            return Err(RefusalRegistryError::ExitMismatch {
                row,
                rule_entry,
                observed: exit,
                expected,
                reason,
            });
        }
        Err(e) => return Err(e),
    };
    if exit != expected {
        return Err(RefusalRegistryError::ExitMismatch {
            row: code,
            rule_entry,
            observed: exit,
            expected: Some(expected),
            reason: "row exit differs from rule".into(),
        });
    }
    if code_family(number) == 9 && retry != RetryPolicy::Never {
        return Err(RefusalRegistryError::ExitMismatch {
            row: code,
            rule_entry: "exit_rule.9xx".into(),
            observed: exit,
            expected: Some(9),
            reason: "9xx requires retry = \"never\"".into(),
        });
    }
    Ok(RefusalRow {
        code,
        number,
        meaning,
        defined_in,
        exit,
        retry,
        repairs,
        raised_by,
        related,
    })
}

fn parse_detail_row(
    table: Table,
    seen: &BTreeSet<u16>,
    idx: usize,
) -> Result<DetailRow, RefusalRegistryError> {
    let (line, col) = table.span();
    let mut f = Fields::new(table, format!("detail[{idx}]"));
    let slug = f.require_str("slug")?;
    let detail = f.require_str("detail")?;
    let interim = extract_str_arr("interim", f.require_array("interim")?).map_err(|e| f.fail(e))?;
    let exit =
        checked_exit_u8(f.require_int("exit")?, &slug, "detail.exit").map_err(|e| f.fail(e))?;
    let retry_s = f.require_str("retry")?;
    let retry = RetryPolicy::parse(&retry_s).ok_or_else(|| {
        f.fail(RefusalRegistryError::InvalidRetry {
            code: slug.clone(),
            value: retry_s,
            line,
            column: col,
        })
    })?;
    let pointer = f.require_str("pointer")?;
    let observed =
        extract_str_arr("observed", f.require_array("observed")?).map_err(|e| f.fail(e))?;
    let repairs = extract_str_arr("repairs", f.require_array("repairs")?).map_err(|e| f.fail(e))?;
    let raised_by =
        extract_str_arr("raised_by", f.require_array("raised_by")?).map_err(|e| f.fail(e))?;
    let ledger = f.require_str("ledger")?;
    f.finish()?;

    for code in &interim {
        if !parse_code_number(code).is_some_and(|n| seen.contains(&n)) {
            return Err(RefusalRegistryError::UnknownRefusalCode {
                code: code.clone(),
                context: format!("detail `{slug}`"),
            });
        }
    }
    Ok(DetailRow {
        slug,
        detail,
        interim,
        exit,
        retry,
        pointer,
        observed,
        repairs,
        raised_by,
        ledger,
    })
}

fn checked_exit_u8(val: i64, row: &str, rule_entry: &str) -> Result<u8, RefusalRegistryError> {
    if let Ok(u) = u8::try_from(val)
        && VALID_REFUSAL_EXITS.contains(&u)
    {
        return Ok(u);
    }
    Err(RefusalRegistryError::ExitMismatch {
        row: row.to_owned(),
        rule_entry: rule_entry.to_owned(),
        observed: u8::try_from(val).unwrap_or(u8::MAX),
        expected: None,
        reason: format!("exit `{val}` outside {{3,4,5,6,7,9}}"),
    })
}

fn extract_str_arr(
    key: &str,
    items: Vec<Spanned<Value>>,
) -> Result<Vec<String>, RefusalRegistryError> {
    items
        .into_iter()
        .map(|sp| match sp.value {
            Value::Str(s) => Ok(s),
            other => Err(TomlError::new(
                sp.line,
                sp.column,
                toml::TomlErrorKind::TypeMismatch {
                    key: key.to_owned(),
                    expected: "string",
                    found: other.type_name(),
                },
            )
            .into()),
        })
        .collect()
}

const fn code_family(num: u16) -> u8 {
    if num >= 1000 {
        (num / 1000) as u8
    } else {
        (num / 100) as u8
    }
}

const fn fixed_family_exit(family: u8) -> Option<u8> {
    match family {
        1..=5 | 8 => Some(3),
        6 => Some(6),
        9 => Some(9),
        _ => None,
    }
}

fn parse_four_digit_hundred_key(key: &str) -> Option<u8> {
    let b = key.as_bytes();
    (b.len() == 4 && matches!(b[0], b'1'..=b'9') && b[1].is_ascii_digit() && &b[2..] == b"xx")
        .then(|| b[0] - b'0')
}

fn parse_code_number(code: &str) -> Option<u16> {
    let d = code.strip_prefix("FROB-REF-")?;
    let b = d.as_bytes();
    if !matches!(b.len(), 3 | 4)
        || !matches!(b[0], b'1'..=b'9')
        || !b[1..].iter().all(u8::is_ascii_digit)
    {
        return None;
    }
    d.parse().ok()
}

fn format_code(num: u16) -> String {
    format!("FROB-REF-{num}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(code: &str, exit: u8, retry: &str) -> String {
        format!(
            "[[refusal]]\ncode = \"{code}\"\nmeaning = \"s {code}\"\ndefined_in = \"§0\"\nexit = {exit}\nretry = \"{retry}\"\nrepairs = [\"r1\", \"r2\"]\nraised_by = [\"BD-005\"]\n\n"
        )
    }
    fn hdr(extra: &str, conflict: &str, integrity: &str, cap: &str) -> String {
        format!(
            "schema = \"frob.refusals/1\"\n\n[exit_rule]\n\"1xx\" = 3\n\"2xx\" = 3\n\"3xx\" = 3\n\"4xx\" = 3\n\"5xx\" = 3\n\"6xx\" = 6\n\"70x\" = 4\n\"72x\" = 5\n\"73x\" = 5\n\"8xx\" = 3\n\"9xx\" = 9\n{extra}conflict = [{conflict}]\nintegrity = [{integrity}]\ncapability = [{cap}]\n\n"
        )
    }

    #[test]
    fn embedded_registry_loads_and_matches_generated_enum() {
        let reg = RefusalRegistry::load().expect("embedded registry loads");
        assert_eq!(reg.exit_for(RefusalCode::FrobRef802).unwrap(), 3);
        assert_eq!(reg.exit_for(RefusalCode::FrobRef816).unwrap(), 3);
        assert_eq!(reg.exit_for(RefusalCode::FrobRef881).unwrap(), 3);
        assert_eq!(
            include_str!("../refusal/code_gen.rs"),
            render_refusal_code_enum(&reg)
        );
    }

    #[test]
    fn unknown_refusal_code_in_verb_row_fails() {
        let verbs = b"schema = \"frob.verbs/1\"\n[[verb]]\nname = \"design.check\"\nrefusals = [\"FROB-REF-816\", \"FROB-REF-999\"]\n";
        let err =
            RefusalRegistry::load_with_verbs(EMBEDDED_REFUSALS_TOML.as_bytes(), verbs).unwrap_err();
        assert!(
            matches!(err, RefusalRegistryError::UnknownRefusalCode { ref code, .. } if code == "FROB-REF-999")
        );
    }

    #[test]
    fn registry_duplicate_code_fails() {
        let doc = format!(
            "{}{}{}",
            hdr("", "", "", ""),
            row("FROB-REF-816", 3, "after-change"),
            row("FROB-REF-816", 3, "after-change")
        );
        assert!(
            matches!(RefusalRegistry::from_bytes(doc.as_bytes()), Err(RefusalRegistryError::DuplicateRefusalCode { ref code, .. }) if code == "FROB-REF-816")
        );
    }

    #[test]
    fn registry_exit_family_rule() {
        let r74 = "\"74x\" = 4\n";
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr(r74, "", "", ""),
                    row("FROB-REF-701", 3, "after-change")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 3,
                expected: Some(4),
                ..
            })
        ));
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr(r74, "", "", ""),
                    row("FROB-REF-990", 3, "never")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 3,
                expected: Some(9),
                ..
            })
        ));
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr(r74, "", "", ""),
                    row("FROB-REF-990", 9, "after-change")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch { .. })
        ));
        assert!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr(r74, "", "", ""),
                    row("FROB-REF-990", 9, "never")
                )
                .as_bytes()
            )
            .is_ok()
        );
        assert!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr(r74, "", "", ""),
                    row("FROB-REF-740", 4, "after-change")
                )
                .as_bytes()
            )
            .is_ok()
        );
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr(r74, "", "", ""),
                    row("FROB-REF-740", 5, "after-change")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 5,
                expected: Some(4),
                ..
            })
        ));
        assert!(
            matches!(RefusalRegistry::from_bytes(format!("{}{}", hdr(r74, "", "", ""), row("FROB-REF-771", 4, "after-change")).as_bytes()), Err(RefusalRegistryError::ExitMismatch { ref rule_entry, expected: None, .. }) if rule_entry == "exit_rule.77x")
        );
        assert!(matches!(
            RefusalRegistry::from_bytes(
                hdr("", "", "", "")
                    .replace("\"9xx\" = 9", "\"9xx\" = 3")
                    .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 3,
                expected: Some(9),
                ..
            })
        ));
        assert!(matches!(
            RefusalRegistry::from_bytes(
                hdr("", "", "", "")
                    .replace("\"6xx\" = 6", "\"6xx\" = 4")
                    .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 4,
                expected: Some(6),
                ..
            })
        ));

        let exc = format!(
            "{}{}{}{}{}",
            hdr(r74, "112, 803", "810", "840"),
            row("FROB-REF-112", 7, "after-change"),
            row("FROB-REF-803", 7, "after-change"),
            row("FROB-REF-810", 9, "never"),
            row("FROB-REF-840", 5, "after-change")
        );
        let reg = RefusalRegistry::from_bytes(exc.as_bytes()).expect("exception lists load");
        assert_eq!(
            (
                reg.lookup("FROB-REF-112", "t").unwrap().exit,
                reg.lookup("FROB-REF-803", "t").unwrap().exit,
                reg.lookup("FROB-REF-810", "t").unwrap().exit,
                reg.lookup("FROB-REF-840", "t").unwrap().exit
            ),
            (7, 7, 9, 5)
        );

        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr("", "", "", ""),
                    row("FROB-REF-820", 7, "after-change")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 7,
                expected: Some(3),
                ..
            })
        ));
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr("", "112", "", ""),
                    row("FROB-REF-112", 3, "after-change")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 3,
                expected: Some(7),
                ..
            })
        ));
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr("", "", "", "840"),
                    row("FROB-REF-840", 3, "after-change")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 3,
                expected: Some(5),
                ..
            })
        ));
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr("", "", "", ""),
                    row("FROB-REF-820", 5, "after-change")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 5,
                expected: Some(3),
                ..
            })
        ));
        assert!(
            matches!(RefusalRegistry::from_bytes(format!("{}{}", hdr("", "840", "", "840"), row("FROB-REF-840", 5, "after-change")).as_bytes()), Err(RefusalRegistryError::OverlappingExceptionList { ref code, .. }) if code == "FROB-REF-840")
        );
        assert!(
            matches!(RefusalRegistry::from_bytes(hdr("", "803", "", "").as_bytes()), Err(RefusalRegistryError::ExceptionCodeMissing { list: "conflict", ref code }) if code == "FROB-REF-803")
        );
    }

    #[test]
    fn registry_unknown_row_key_is_error() {
        let doc = format!(
            "{}[[refusal]]\ncode = \"FROB-REF-816\"\nmeaning = \"m\"\ndefined_in = \"§37.12\"\nexit = 3\nexti = 3\nretry = \"after-change\"\n",
            hdr("", "", "", "")
        );
        match RefusalRegistry::from_bytes(doc.as_bytes()).unwrap_err() {
            RefusalRegistryError::Toml(te) => {
                assert!(te.line > 1 && te.column >= 1);
                assert!(
                    matches!(te.kind, toml::TomlErrorKind::UnknownKey { ref key, .. } if key == "exti")
                );
            }
            other => panic!("expected Toml UnknownKey for `exti`, got {other:?}"),
        }
    }

    #[test]
    fn four_digit_code_and_whole_token_rule() {
        let r81 = "\"81xx\" = 3\n";
        let doc = format!(
            "{}{}{}{}",
            hdr(r81, "", "", ""),
            row("FROB-REF-990", 9, "never"),
            row("FROB-REF-8100", 3, "after-change"),
            row("FROB-REF-816", 3, "after-change"),
        );
        let reg = RefusalRegistry::from_bytes(doc.as_bytes()).expect("8100 loads with 81xx = 3");
        let nums: Vec<u16> = reg.rows().iter().map(|r| r.number).collect();
        assert_eq!(
            nums,
            vec![816, 8100, 990],
            "sorted by family, then numerically"
        );
        assert_eq!(reg.lookup("FROB-REF-8100", "t").unwrap().exit, 3);
        let rendered = render_refusal_code_enum(&reg);
        let i816 = rendered.find("FrobRef816 = 816,").unwrap();
        let i8100 = rendered.find("FrobRef8100 = 8100,").unwrap();
        let i990 = rendered.find("FrobRef990 = 990,").unwrap();
        assert!(i816 < i8100 && i8100 < i990);

        // Whole-token comparison: FROB-REF-810 must never match FROB-REF-8100 or vice versa.
        assert!(
            matches!(reg.lookup("FROB-REF-810", "t"), Err(RefusalRegistryError::UnknownRefusalCode { ref code, .. }) if code == "FROB-REF-810")
        );
        let verb_810 =
            b"schema = \"frob.verbs/1\"\n[[verb]]\nname = \"v\"\nrefusals = [\"FROB-REF-810\"]\n";
        assert!(
            matches!(reg.validate_verb_refusals(verb_810), Err(RefusalRegistryError::UnknownRefusalCode { ref code, .. }) if code == "FROB-REF-810")
        );
        let only_810 = RefusalRegistry::from_bytes(
            format!(
                "{}{}",
                hdr(r81, "", "810", ""),
                row("FROB-REF-810", 9, "never")
            )
            .as_bytes(),
        )
        .unwrap();
        assert!(
            matches!(only_810.lookup("FROB-REF-8100", "t"), Err(RefusalRegistryError::UnknownRefusalCode { ref code, .. }) if code == "FROB-REF-8100")
        );

        // Missing 81xx rule, contradictory 81xx exit, wrong row exit, and exception list override.
        assert!(
            matches!(RefusalRegistry::from_bytes(format!("{}{}", hdr("", "", "", ""), row("FROB-REF-8100", 3, "after-change")).as_bytes()), Err(RefusalRegistryError::ExitMismatch { ref rule_entry, expected: None, .. }) if rule_entry == "exit_rule.81xx")
        );
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr("\"81xx\" = 6\n", "", "", ""),
                    row("FROB-REF-8100", 6, "after-change")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 6,
                expected: Some(3),
                ..
            })
        ));
        assert!(matches!(
            RefusalRegistry::from_bytes(
                format!(
                    "{}{}",
                    hdr(r81, "", "", ""),
                    row("FROB-REF-8100", 9, "never")
                )
                .as_bytes()
            ),
            Err(RefusalRegistryError::ExitMismatch {
                observed: 9,
                expected: Some(3),
                ..
            })
        ));
        let exc = RefusalRegistry::from_bytes(
            format!(
                "{}{}",
                hdr(r81, "8100", "", ""),
                row("FROB-REF-8100", 7, "after-change")
            )
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(exc.lookup("FROB-REF-8100", "t").unwrap().exit, 7);

        for bad in ["FROB-REF-81", "FROB-REF-0810", "FROB-REF-81000"] {
            assert!(
                matches!(
                    RefusalRegistry::from_bytes(
                        format!("{}{}", hdr(r81, "", "", ""), row(bad, 3, "after-change"))
                            .as_bytes()
                    ),
                    Err(RefusalRegistryError::InvalidRefusalCode { .. })
                ),
                "expected InvalidRefusalCode for {bad}"
            );
        }
    }
}
