//! Deterministic Rust code generator for `$OUT_DIR/verbs_table.rs` (bead N32a-02).
//!
//! Emits:
//! - `pub static VERBS: [VerbEntry; N]` sorted strictly by `name` (UTF-8 / code point order)
//! - `pub static TASK_CLUSTERS: [TaskCluster; K]` sorted by `name` (empty at M0)
//! - `pub static SCHEMAS: [(SchemaId, &str); S]` sorted by schema identifier, canonical JSON
//! - `pub const REGISTRY_DIGEST: [u8; 32]` (BLAKE3 over canonical sorted entries)
//! - `pub const REGISTRY_DIGEST_HEX: &str`
//!
//! Pure function of input registry + schemas + `conformance` flag (determinism class D0;
//! emits zero timestamps and zero machine paths).

use std::fmt::Write as _;

use frob_core::id::Blake3;

use super::entry::{
    Kind, Milestone, OwnedAction, OwnedParamKind, OwnedTaskCluster, OwnedVerbEntry, ParsedRegistry,
    Tier,
};
use super::lint::{LintContext, LintError, LintReport, lint_registry};

/// Header marker placed on line 1 of `$OUT_DIR/verbs_table.rs`.
pub const GENERATED_TABLE_MARKER: &str = "// @generated from registries/verbs.toml by frob-verbs/build.rs (N32a-02); do not edit by hand.";

/// Complete output produced by compiling a validated verb registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedTable {
    pub source: String,
    pub digest: [u8; 32],
    pub digest_hex: String,
    pub entry_count: usize,
    pub cluster_count: usize,
    pub largest_task_cluster: usize,
    pub warnings: Vec<String>,
}

/// Lint `reg` under `ctx` and render deterministic `$OUT_DIR/verbs_table.rs` source.
pub fn compile_registry(
    reg: &ParsedRegistry,
    ctx: &LintContext,
) -> Result<GeneratedTable, LintError> {
    let report = lint_registry(reg, ctx)?;
    render_validated_registry(reg, ctx, report)
}

/// Parse `verbs.toml` bytes, lint under `ctx`, and render deterministic `$OUT_DIR/verbs_table.rs`.
pub fn compile_registry_bytes(
    bytes: &[u8],
    ctx: &LintContext,
) -> Result<GeneratedTable, LintError> {
    let (reg, report) = super::lint::parse_and_lint(bytes, ctx)?;
    render_validated_registry(&reg, ctx, report)
}

fn render_validated_registry(
    reg: &ParsedRegistry,
    ctx: &LintContext,
    report: LintReport,
) -> Result<GeneratedTable, LintError> {
    let mut sorted_verbs: Vec<&OwnedVerbEntry> = reg.verbs.iter().collect();
    sorted_verbs.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));

    let mut sorted_clusters: Vec<&OwnedTaskCluster> = reg.task_clusters.iter().collect();
    sorted_clusters.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));

    let digest = reg.registry_digest().map_err(|e| {
        LintError::new(
            "<digest>",
            "RL-13",
            "canonical-digest",
            "REGISTRY_DIGEST",
            e.to_string(),
        )
    })?;
    let digest_hex = Blake3::from_bytes(digest).to_hex();

    let mut out = String::with_capacity(1024 + sorted_verbs.len() * 512);
    let _ = writeln!(out, "{GENERATED_TABLE_MARKER}");
    out.push('\n');

    // VERBS static array
    let _ = writeln!(
        out,
        "/// All registered verbs sorted strictly by `name` for binary search."
    );
    let _ = writeln!(
        out,
        "pub static VERBS: [VerbEntry; {}] = [",
        sorted_verbs.len()
    );
    for entry in &sorted_verbs {
        emit_verb_entry(&mut out, entry, ctx)?;
    }
    out.push_str("];\n\n");

    // TASK_CLUSTERS static array
    let _ = writeln!(
        out,
        "/// Declared task clusters (empty at M0; populated by NJL-05 at M1)."
    );
    let _ = writeln!(
        out,
        "pub static TASK_CLUSTERS: [TaskCluster; {}] = [",
        sorted_clusters.len()
    );
    for tc in &sorted_clusters {
        emit_task_cluster(&mut out, tc);
    }
    out.push_str("];\n\n");

    // SCHEMAS static array (BTreeMap guarantees sorted order by schema identifier)
    let _ = writeln!(
        out,
        "/// Embedded canonical JSON schemas sorted by schema identifier."
    );
    let _ = writeln!(
        out,
        "pub static SCHEMAS: [(SchemaId, &str); {}] = [",
        report.embedded_schemas.len()
    );
    for (id, canonical_json) in &report.embedded_schemas {
        let _ = writeln!(
            out,
            "    (SchemaId({}), {}),",
            rust_str(id),
            rust_str(canonical_json)
        );
    }
    out.push_str("];\n\n");

    // REGISTRY_DIGEST bytes & hex
    let _ = writeln!(
        out,
        "/// Domain-separated BLAKE3 digest of the canonical serialization of all sorted verb entries."
    );
    out.push_str("pub const REGISTRY_DIGEST: [u8; 32] = [\n");
    for chunk in digest.chunks(8) {
        out.push_str("    ");
        for (i, &b) in chunk.iter().enumerate() {
            if i > 0 {
                out.push(' ');
            }
            let _ = write!(out, "0x{b:02x},");
        }
        out.push('\n');
    }
    out.push_str("];\n\n");
    let _ = writeln!(
        out,
        "/// Lower-case hexadecimal spelling of [`REGISTRY_DIGEST`]."
    );
    let _ = writeln!(
        out,
        "pub const REGISTRY_DIGEST_HEX: &str = {};",
        rust_str(&digest_hex)
    );

    Ok(GeneratedTable {
        source: out,
        digest,
        digest_hex,
        entry_count: sorted_verbs.len(),
        cluster_count: report.cluster_count,
        largest_task_cluster: report.largest_task_cluster,
        warnings: report.warnings,
    })
}

fn emit_verb_entry(
    out: &mut String,
    entry: &OwnedVerbEntry,
    ctx: &LintContext,
) -> Result<(), LintError> {
    let kind = Kind::parse(&entry.kind_raw).ok_or_else(|| {
        LintError::new(
            &entry.name,
            "RL-08",
            "kind",
            &entry.kind_raw,
            "invalid kind",
        )
    })?;
    let tier = Tier::parse(&entry.tier_raw).ok_or_else(|| {
        LintError::new(
            &entry.name,
            "RL-12",
            "tier",
            &entry.tier_raw,
            "invalid tier",
        )
    })?;
    let milestone = Milestone::parse(&entry.first_milestone_raw).ok_or_else(|| {
        LintError::new(
            &entry.name,
            "RL-12",
            "milestone",
            &entry.first_milestone_raw,
            "invalid milestone",
        )
    })?;

    out.push_str("    VerbEntry {\n");
    let _ = writeln!(out, "        name: {},", rust_str(&entry.name));
    let _ = writeln!(out, "        cluster: {},", rust_str(&entry.cluster));
    let _ = writeln!(out, "        kind: {},", kind_variant(kind));
    let _ = writeln!(out, "        tier: {},", tier_variant(tier));
    let _ = writeln!(
        out,
        "        capabilities: {},",
        emit_scopes(&entry.capabilities)
    );
    let _ = writeln!(
        out,
        "        input_schema: SchemaId({}),",
        rust_str(&entry.input_schema)
    );
    let _ = writeln!(
        out,
        "        output_schema: SchemaId({}),",
        rust_str(&entry.output_schema)
    );

    // Emit RefusalCode variants sorted by numeric code for stability.
    let mut ref_nums: Vec<u16> = Vec::with_capacity(entry.refusals.len());
    for code in &entry.refusals {
        let row = ctx.refusals.lookup(code, &entry.name).map_err(|e| {
            LintError::new(&entry.name, "RL-06", "unknown-refusal", code, e.to_string())
        })?;
        ref_nums.push(row.number);
    }
    out.push_str("        refusals: &[");
    for (idx, num) in ref_nums.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "frob_core::refusal::RefusalCode::FrobRef{num}");
    }
    out.push_str("],\n");

    let _ = writeln!(out, "        idempotent: {},", entry.idempotent);
    let _ = writeln!(out, "        cost_model: {},", rust_str(&entry.cost_model));
    let _ = writeln!(
        out,
        "        first_milestone: {},",
        milestone_variant(milestone)
    );
    let _ = writeln!(
        out,
        "        task_clusters: {},",
        emit_str_slice(&entry.task_clusters)
    );
    let _ = writeln!(
        out,
        "        examples: {},",
        emit_str_slice(&entry.examples)
    );
    let _ = writeln!(out, "        purpose: {},", rust_str(&entry.purpose));
    let _ = writeln!(out, "        use_when: {},", rust_str(&entry.use_when));
    let _ = writeln!(out, "        owner: {},", rust_str(&entry.owner));

    match &entry.wait {
        Some(w) => {
            let _ = writeln!(
                out,
                "        wait: Some(WaitDefault {{ cli: {}, mcp_ms: {}, mcp_max_ms: {} }}),",
                rust_str(&w.cli),
                w.mcp_ms,
                w.mcp_max_ms
            );
        }
        None => out.push_str("        wait: None,\n"),
    }

    let mut sorted_actions: Vec<&OwnedAction> = entry.actions.iter().collect();
    sorted_actions.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));
    if sorted_actions.is_empty() {
        out.push_str("        actions: &[],\n");
    } else {
        out.push_str("        actions: &[\n");
        for act in sorted_actions {
            let act_kind = Kind::parse(&act.kind_raw).ok_or_else(|| {
                LintError::new(
                    &entry.name,
                    "RL-08",
                    "kind",
                    &act.kind_raw,
                    "invalid action kind",
                )
            })?;
            let _ = writeln!(
                out,
                "            Action {{ name: {}, kind: {}, capabilities: {}, paginated: {}, destructive: {} }},",
                rust_str(&act.name),
                kind_variant(act_kind),
                emit_scopes(&act.capabilities),
                act.paginated,
                act.effective_destructive()
            );
        }
        out.push_str("        ],\n");
    }

    let _ = writeln!(out, "        schema_version: {},", entry.schema_version);
    let _ = writeln!(out, "        paginated: {},", entry.paginated);
    let _ = writeln!(out, "        exports: {},", entry.exports);
    let _ = writeln!(out, "        estimable: {},", entry.estimable);
    let _ = writeln!(
        out,
        "        destructive: {},",
        entry.effective_destructive()
    );
    let _ = writeln!(out, "        public: {},", entry.public);
    let _ = writeln!(out, "        key_optional: {},", entry.key_optional);

    if entry.option_capabilities.is_empty() {
        out.push_str("        option_capabilities: &[],\n");
    } else {
        out.push_str("        option_capabilities: &[\n");
        for (opt, scopes) in &entry.option_capabilities {
            let _ = writeln!(
                out,
                "            ({}, {}),",
                rust_str(opt),
                emit_scopes(scopes)
            );
        }
        out.push_str("        ],\n");
    }

    match entry.spends_physics {
        Some(b) => {
            let _ = writeln!(out, "        spends_physics: Some({b}),");
        }
        None => out.push_str("        spends_physics: None,\n"),
    }
    let _ = writeln!(out, "        released: {},", entry.released);

    if entry.params.is_empty() {
        out.push_str("        params: &[],\n");
    } else {
        out.push_str("        params: &[\n");
        for (k, pk) in &entry.params {
            let _ = writeln!(
                out,
                "            ({}, {}),",
                rust_str(k),
                emit_param_kind(pk)
            );
        }
        out.push_str("        ],\n");
    }

    let _ = writeln!(
        out,
        "        conformance_only: {},",
        emit_str_slice(&entry.conformance_only)
    );
    out.push_str("    },\n");
    Ok(())
}

fn emit_task_cluster(out: &mut String, tc: &OwnedTaskCluster) {
    let _ = writeln!(
        out,
        "    TaskCluster {{ name: {}, activity: {}, default: {}, tools: {} }},",
        rust_str(&tc.name),
        rust_str(&tc.activity),
        tc.default,
        emit_str_slice(&tc.tools)
    );
}

fn emit_scopes(scopes: &[String]) -> String {
    let mut out = String::from("&[");
    for (i, s) in scopes.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "Scope({})", rust_str(s));
    }
    out.push(']');
    out
}

fn emit_str_slice(items: &[String]) -> String {
    let mut out = String::from("&[");
    for (i, s) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&rust_str(s));
    }
    out.push(']');
    out
}

fn emit_param_kind(pk: &OwnedParamKind) -> String {
    match pk {
        OwnedParamKind::ArtifactInput(f) => format!("ParamKind::ArtifactInput({})", rust_str(f)),
        OwnedParamKind::ArtifactRef(f) => format!("ParamKind::ArtifactRef({})", rust_str(f)),
        OwnedParamKind::ArtifactIdentity(f) => {
            format!("ParamKind::ArtifactIdentity({})", rust_str(f))
        }
        OwnedParamKind::ForeignFile => String::from("ParamKind::ForeignFile"),
    }
}

const fn kind_variant(kind: Kind) -> &'static str {
    match kind {
        Kind::Read => "Kind::Read",
        Kind::Run => "Kind::Run",
        Kind::Mutate => "Kind::Mutate",
        Kind::Admin => "Kind::Admin",
    }
}

const fn tier_variant(tier: Tier) -> &'static str {
    match tier {
        Tier::S => "Tier::S",
        Tier::F => "Tier::F",
        Tier::M => "Tier::M",
    }
}

const fn milestone_variant(m: Milestone) -> &'static str {
    match m {
        Milestone::M0 => "Milestone::M0",
        Milestone::M1 => "Milestone::M1",
        Milestone::M2 => "Milestone::M2",
        Milestone::M3 => "Milestone::M3",
        Milestone::M4 => "Milestone::M4",
        Milestone::M5 => "Milestone::M5",
        Milestone::M6 => "Milestone::M6",
        Milestone::M7 => "Milestone::M7",
        Milestone::M8 => "Milestone::M8",
    }
}

/// Emit an unambiguous Rust string literal with ASCII/control escapes for deterministic source.
fn rust_str(s: &str) -> String {
    format!("{s:?}")
}
