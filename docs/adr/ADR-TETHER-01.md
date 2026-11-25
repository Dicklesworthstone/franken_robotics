# ADR-TETHER-01: Single Compiled Verb Registry and Generic Surfaces

- **ID:** `ADR-TETHER-01`
- **Status:** `accepted`
- **Governing bead:** `N32a-02`
- **First milestone:** `M0`
- **Plan references:** §32.2, §32.3, §32.12.4

## Decision

`registries/verbs.toml` (schema `frob.verbs/1`) defines every verb once, and the build script of `frob-verbs` compiles it into a single static table (`VERBS`, `TASK_CLUSTERS`, `SCHEMAS`, `REGISTRY_DIGEST`) in `$OUT_DIR/verbs_table.rs`. Neither `frob-cli` nor `surface/frob-mcp` contains per-verb command definitions, per-verb tool declarations, or per-verb handlers, with exactly one allowlisted exception: the MCP half of `session.focus`, which edits the client's visible tool set.

## Reason

Generating the CLI parser, MCP tool definitions, JSON schemas, refusal lists, and agent documentation from one compiled static table delivers surface parity by construction. The parity gate (`GATE-TETHER-02`) then verifies the served surfaces directly against the registry rather than trusting hand-maintained mirrors to stay aligned across ~243 verbs in ~44 clusters.

## Alternative excluded

Hand-written CLI subcommand enums (such as per-verb `clap` definitions) and hand-written MCP `#[tool]` handler wrappers in each adapter crate. That design duplicates every verb name, flag, schema, and refusal list across two surfaces and inevitably drifts.

## Enforced by

- `frob-verbs::registry::table_sorted_and_unique` (`crates/frob-verbs/src/registry/mod.rs`, active at M0): verifies the compiled static `VERBS` table is strictly ordered and unique.
- `verb_literal_lint_catches_plant` (pending at M0, `BD-022` for `frob-cli` and `BD-023` for `surface/frob-mcp`): fails if any verb-name literal appears in either surface crate outside the one allowlisted `session.focus` file.
- `GATE-TETHER-02` (pending at M0, `N32a-11`): parity harness diffing served CLI and MCP surfaces against `registries/verbs.toml`.

## Revisit when

- A second surface-local protocol mutation beyond `session.focus` requires per-verb handling inside a surface crate (requires a superseding ADR widening the explicit allowlist rather than adding ad-hoc exceptions).
- Build-script compilation time for `registries/verbs.toml` exceeds its 1 s budget for 300 entries.
