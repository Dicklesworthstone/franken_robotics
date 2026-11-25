//! Compendium registry loader, resolver, and `docs/math/compendium.md` renderer (`N12-41`).
//!
//! Shared across all ten compendium sections (`D.1` through `D.10`). Uses
//! `frob_core::registry::toml` (strict TOML subset + `Fields`) as its single TOML reader.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::registry::toml::{self, Fields, Spanned, Value};

/// Authoritative section list and plan row counts (`N12-41` step 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionSpec {
    pub id: &'static str,
    pub slug: &'static str,
    pub toml_file: &'static str,
    pub oracle_rel: &'static str,
    pub test_rel: &'static str,
    pub expected_rows: usize,
}

pub const COMPENDIUM_SECTIONS: &[SectionSpec] = &[
    SectionSpec { id: "D.1", slug: "d1", toml_file: "d1.toml", oracle_rel: "crates/frob-conformance/src/compendium/d1.rs", test_rel: "crates/frob-conformance/tests/compendium_d1.rs", expected_rows: 21 },
    SectionSpec { id: "D.2", slug: "d2", toml_file: "d2.toml", oracle_rel: "crates/frob-conformance/src/compendium/d2.rs", test_rel: "crates/frob-conformance/tests/compendium_d2.rs", expected_rows: 12 },
    SectionSpec { id: "D.3", slug: "d3", toml_file: "d3.toml", oracle_rel: "crates/frob-conformance/src/compendium/d3.rs", test_rel: "crates/frob-conformance/tests/compendium_d3.rs", expected_rows: 10 },
    SectionSpec { id: "D.4", slug: "d4", toml_file: "d4.toml", oracle_rel: "crates/frob-conformance/src/compendium/d4.rs", test_rel: "crates/frob-conformance/tests/compendium_d4.rs", expected_rows: 14 },
    SectionSpec { id: "D.5", slug: "d5", toml_file: "d5.toml", oracle_rel: "crates/frob-conformance/src/compendium/d5.rs", test_rel: "crates/frob-conformance/tests/compendium_d5.rs", expected_rows: 10 },
    SectionSpec { id: "D.6", slug: "d6", toml_file: "d6.toml", oracle_rel: "crates/frob-conformance/src/compendium/d6.rs", test_rel: "crates/frob-conformance/tests/compendium_d6.rs", expected_rows: 15 },
    SectionSpec { id: "D.7", slug: "d7", toml_file: "d7.toml", oracle_rel: "crates/frob-conformance/src/compendium/d7.rs", test_rel: "crates/frob-conformance/tests/compendium_d7.rs", expected_rows: 7 },
    SectionSpec { id: "D.8", slug: "d8", toml_file: "d8.toml", oracle_rel: "crates/frob-conformance/src/compendium/d8.rs", test_rel: "crates/frob-conformance/tests/compendium_d8.rs", expected_rows: 19 },
    SectionSpec { id: "D.9", slug: "d9", toml_file: "d9.toml", oracle_rel: "crates/frob-conformance/src/compendium/d9.rs", test_rel: "crates/frob-conformance/tests/compendium_d9.rs", expected_rows: 14 },
    SectionSpec { id: "D.10", slug: "d10", toml_file: "d10.toml", oracle_rel: "crates/frob-conformance/src/compendium/d10.rs", test_rel: "crates/frob-conformance/tests/compendium_d10.rs", expected_rows: 4 },
];

pub const COMPENDIUM_DOC_MARKER: &str = "<!-- @generated from registries/compendium/d*.toml by frob-conformance example compendium_doc (N12-41); do not edit by hand. -->";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompendiumRow {
    pub name: String,
    pub statement: String,
    pub validity: String,
    pub used_in: Vec<String>,
    pub oracle: String,
    pub tests: Vec<String>,
    pub negative_control: String,
    pub carried_by: Vec<String>,
    pub evidence_class: Option<String>,
    pub arithmetic: String,
    pub tolerance: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompendiumSection {
    pub section: String,
    pub title: String,
    pub expected_rows: usize,
    pub rows: Vec<CompendiumRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryFinding {
    pub section: String,
    pub row: String,
    pub rule: String,
    pub detail: String,
}

fn str_list(key: &str, items: Vec<Spanned<Value>>) -> Result<Vec<String>, String> {
    items
        .into_iter()
        .map(|sp| match sp.value {
            Value::Str(s) => Ok(s),
            other => Err(format!("{}:{}: `{key}` element expected string, found {}", sp.line, sp.column, other.type_name())),
        })
        .collect()
}

/// Parse one section file `registries/compendium/d<N>.toml` strictly via `frob_core::registry::toml`.
pub fn parse_section_bytes(bytes: &[u8]) -> Result<CompendiumSection, String> {
    let root = toml::parse(bytes).map_err(|e| e.to_string())?;
    let mut top = Fields::new(root, "<section>");
    let schema = top.require_str("schema").map_err(|e| e.to_string())?;
    if schema != "frob.compendium.section/1" {
        return Err(format!("unexpected schema `{schema}`, expected `frob.compendium.section/1`"));
    }
    let section = top.require_str("section").map_err(|e| e.to_string())?;
    let title = top.require_str("title").map_err(|e| e.to_string())?;
    let expected_raw = top.require_int("expected_rows").map_err(|e| e.to_string())?;
    let expected_rows = usize::try_from(expected_raw)
        .map_err(|_| format!("invalid expected_rows `{expected_raw}`"))?;
    let row_vals = top.require_array("row").map_err(|e| e.to_string())?;
    top.finish().map_err(|e| e.to_string())?;

    let mut rows = Vec::with_capacity(row_vals.len());
    for (idx, sp) in row_vals.into_iter().enumerate() {
        let Value::Table(tbl) = sp.value else {
            return Err(format!("{}:{}: row[{idx}] must be a table", sp.line, sp.column));
        };
        let mut rf = Fields::new(tbl, format!("row[{idx}]"));
        let name = rf.require_str("name").map_err(|e| e.to_string())?;
        let statement = rf.require_str("statement").map_err(|e| e.to_string())?;
        let validity = rf.require_str("validity").map_err(|e| e.to_string())?;
        let used_in = str_list("used_in", rf.require_array("used_in").map_err(|e| e.to_string())?)?;
        let oracle = rf.require_str("oracle").map_err(|e| e.to_string())?;
        let tests = str_list("tests", rf.require_array("tests").map_err(|e| e.to_string())?)?;
        let negative_control = rf.require_str("negative_control").map_err(|e| e.to_string())?;
        let carried_by = str_list("carried_by", rf.require_array("carried_by").map_err(|e| e.to_string())?)?;
        let evidence_class = rf.opt_str("evidence_class").map_err(|e| e.to_string())?;
        let arithmetic = rf.opt_str("arithmetic").map_err(|e| e.to_string())?.unwrap_or_else(|| "exact-rational".to_owned());
        let tolerance = rf.opt_str("tolerance").map_err(|e| e.to_string())?;
        rf.finish().map_err(|e| e.to_string())?;

        rows.push(CompendiumRow {
            name,
            statement,
            validity,
            used_in,
            oracle,
            tests,
            negative_control,
            carried_by,
            evidence_class,
            arithmetic,
            tolerance,
        });
    }
    Ok(CompendiumSection { section, title, expected_rows, rows })
}

/// Load all present compendium section files in canonical order (`D.1`..`D.10`).
pub fn load_present_sections(root: &Path) -> Result<Vec<(SectionSpec, CompendiumSection)>, String> {
    let dir = root.join("registries/compendium");
    let mut out = Vec::new();
    for &spec in COMPENDIUM_SECTIONS {
        let path = dir.join(spec.toml_file);
        if path.is_file() {
            let bytes = fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
            let parsed = parse_section_bytes(&bytes)
                .map_err(|e| format!("{}: {e}", path.display()))?;
            out.push((spec, parsed));
        }
    }
    Ok(out)
}

fn collect_defined_fns(src: &str, require_pub: bool) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for raw in src.lines() {
        let line = raw.trim();
        let rest = if require_pub {
            line.strip_prefix("pub fn ")
                .or_else(|| line.strip_prefix("pub const fn "))
        } else {
            line.strip_prefix("fn ")
                .or_else(|| line.strip_prefix("pub fn "))
        };
        if let Some(after) = rest {
            let name: String = after
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty() {
                out.insert(name);
            }
        }
    }
    out
}

/// Validate all present `registries/compendium/d*.toml` files against their oracle and test sources.
pub fn validate_workspace(root: &Path) -> Result<Vec<RegistryFinding>, String> {
    let present = load_present_sections(root)?;
    let mut findings = Vec::new();
    if present.is_empty() {
        findings.push(RegistryFinding {
            section: "<all>".into(),
            row: String::new(),
            rule: "compendium.empty".into(),
            detail: "no registries/compendium/d*.toml files found".into(),
        });
        return Ok(findings);
    }

    for (spec, sec) in present {
        if sec.section != spec.id {
            findings.push(RegistryFinding {
                section: spec.id.into(),
                row: String::new(),
                rule: "compendium.section-id".into(),
                detail: format!("file {} declares section `{}`, expected `{}`", spec.toml_file, sec.section, spec.id),
            });
        }
        if sec.rows.len() != spec.expected_rows || sec.expected_rows != spec.expected_rows {
            findings.push(RegistryFinding {
                section: spec.id.into(),
                row: String::new(),
                rule: "compendium.row-count".into(),
                detail: format!("found {} rows, expected {}", sec.rows.len(), spec.expected_rows),
            });
        }
        let oracle_path = root.join(spec.oracle_rel);
        let oracle_fns = match fs::read_to_string(&oracle_path) {
            Ok(txt) => collect_defined_fns(&txt, true),
            Err(e) => {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: String::new(),
                    rule: "compendium.oracle-file-missing".into(),
                    detail: format!("cannot read {}: {e}", oracle_path.display()),
                });
                BTreeSet::new()
            }
        };
        let test_path = root.join(spec.test_rel);
        let test_fns = match fs::read_to_string(&test_path) {
            Ok(txt) => collect_defined_fns(&txt, false),
            Err(e) => {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: String::new(),
                    rule: "compendium.test-file-missing".into(),
                    detail: format!("cannot read {}: {e}", test_path.display()),
                });
                BTreeSet::new()
            }
        };

        let expected_prefix = format!("frob_conformance::compendium::{}::", spec.slug);
        let mut seen_names = BTreeSet::new();
        for row in &sec.rows {
            if !seen_names.insert(row.name.clone()) {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: row.name.clone(),
                    rule: "compendium.duplicate-row".into(),
                    detail: format!("duplicate row name `{}`", row.name),
                });
            }
            match row.oracle.strip_prefix(&expected_prefix) {
                Some(fn_name) if !fn_name.is_empty() && oracle_fns.contains(fn_name) => {}
                _ => findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: row.name.clone(),
                    rule: "compendium.oracle-unresolved".into(),
                    detail: format!("oracle `{}` does not resolve to a `pub fn` in {}", row.oracle, spec.oracle_rel),
                }),
            }
            if row.tests.is_empty() {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: row.name.clone(),
                    rule: "compendium.no-test".into(),
                    detail: "row lists zero unit tests".into(),
                });
            }
            for t in &row.tests {
                if !test_fns.contains(t) {
                    findings.push(RegistryFinding {
                        section: spec.id.into(),
                        row: row.name.clone(),
                        rule: "compendium.test-unresolved".into(),
                        detail: format!("test `{t}` not found in {}", spec.test_rel),
                    });
                }
            }
            if row.negative_control.trim().is_empty() {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: row.name.clone(),
                    rule: "compendium.missing-negative-control".into(),
                    detail: "row has empty negative_control".into(),
                });
            } else if !test_fns.contains(&row.negative_control) {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: row.name.clone(),
                    rule: "compendium.negative-control-unresolved".into(),
                    detail: format!("negative control `{}` not found in {}", row.negative_control, spec.test_rel),
                });
            }
            if row.carried_by.is_empty() {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: row.name.clone(),
                    rule: "compendium.carried-by-empty".into(),
                    detail: "row must list carrying specification beads or `compendium-only`".into(),
                });
            }
            if let Some(ec) = &row.evidence_class
                && !matches!(ec.as_str(), "Proved" | "Certified" | "Tested" | "Estimated")
            {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: row.name.clone(),
                    rule: "compendium.invalid-evidence-class".into(),
                    detail: format!("unknown evidence_class `{ec}`"),
                });
            }
            if row.arithmetic == "tolerance" && row.tolerance.as_deref().is_none_or(|t| t.trim().is_empty()) {
                findings.push(RegistryFinding {
                    section: spec.id.into(),
                    row: row.name.clone(),
                    rule: "compendium.missing-tolerance".into(),
                    detail: "tolerance-class row must declare a dimensioned tolerance".into(),
                });
            }
        }
    }
    Ok(findings)
}

fn esc_md(s: &str) -> String {
    s.replace('|', "\\|")
}

/// Deterministically render `docs/math/compendium.md` from all present section TOML files.
pub fn render_compendium_doc(root: &Path) -> Result<String, String> {
    let present = load_present_sections(root)?;
    let mut out = String::new();
    out.push_str(COMPENDIUM_DOC_MARKER);
    out.push_str("\n# FrankenRobotics Mathematics Compendium\n\n");
    out.push_str("## Conventions (§11)\n\n");
    out.push_str("- `g = (R, p) = a_from_b` is a rigid transform acting as `x_a = R·x_b + p`.\n");
    out.push_str("- `ξ = (ω, v)` is a twist (angular part first, body-frame by default).\n");
    out.push_str("- `w = (τ, f)` is a wrench (torque first); `μ = (π, ℓ)` is a momentum.\n");
    out.push_str("- Algebra coadjoint is the dual map `ad*_ξ = ad_ξᵀ` (no minus sign); group coadjoint is `Ad_g⁻ᵀ`; Euler–Poincaré reads `μ̇ = ad*_ξ μ + w`.\n");
    out.push_str("- Exact statements use arbitrary-precision rational arithmetic over `ℚ`; non-rational rows declare explicit dimensioned tolerances.\n\n");

    for (spec, sec) in &present {
        out.push_str(&format!("## {} {} ({} rows)\n\n", spec.id, sec.title, sec.rows.len()));
        out.push_str("| Row | Statement | Used in | Validity | Oracle | Tests | Negative control | Carried by | Arithmetic |\n");
        out.push_str("|---|---|---|---|---|---|---|---|---|\n");
        let mut compendium_only = Vec::new();
        let mut estimated_rows = Vec::new();
        for row in &sec.rows {
            let used = row.used_in.join(", ");
            let tests = row.tests.iter().map(|t| format!("`{t}`")).collect::<Vec<_>>().join(", ");
            let carried = row.carried_by.join(", ");
            if row.carried_by.iter().any(|c| c == "compendium-only") {
                compendium_only.push(row.name.as_str());
            }
            if row.evidence_class.as_deref() == Some("Estimated") {
                estimated_rows.push(row.name.as_str());
            }
            let arith = match &row.tolerance {
                Some(tol) => format!("{} ({tol})", row.arithmetic),
                None => row.arithmetic.clone(),
            };
            out.push_str(&format!(
                "| {} | {} | {} | {} | `{}` | {} | `{}` | {} | {} |\n",
                esc_md(&row.name),
                esc_md(&row.statement),
                esc_md(&used),
                esc_md(&row.validity),
                esc_md(&row.oracle),
                tests,
                esc_md(&row.negative_control),
                esc_md(&carried),
                esc_md(&arith),
            ));
        }
        out.push('\n');
        if !compendium_only.is_empty() {
            out.push_str(&format!(
                "Rows marked `compendium-only` in {}: {}.\n\n",
                spec.id,
                compendium_only.join(", ")
            ));
        }
        if !estimated_rows.is_empty() {
            out.push_str(&format!(
                "Rows marked `Estimated` in {}: {}.\n\n",
                spec.id,
                estimated_rows.join(", ")
            ));
        }
    }

    let missing: Vec<&str> = COMPENDIUM_SECTIONS
        .iter()
        .filter(|spec| !present.iter().any(|(p, _)| p.id == spec.id))
        .map(|spec| spec.id)
        .collect();
    if !missing.is_empty() {
        out.push_str(&format!(
            "## Pending sections\n\nSections not yet landed: {}.\n",
            missing.join(", ")
        ));
    }
    Ok(out)
}

/// Locate the workspace root containing `franken-stack.lock`.
pub fn workspace_root() -> Result<PathBuf, String> {
    frob_core::testkit::registry::root().map_err(|e| e.to_string())
}
