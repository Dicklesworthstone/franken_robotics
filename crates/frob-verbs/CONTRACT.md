# `frob-verbs` Contract

`frob-verbs` is layer 6 (rank 4, tier Solid, cap 22,000 lines with tests) of the FrankenRobotics core workspace.

## Purpose

Compile `registries/verbs.toml` (schema `frob.verbs/1`) once at build time into the single authoritative static verb table (`VERBS`, `TASK_CLUSTERS`, `SCHEMAS`, `REGISTRY_DIGEST`) consumed by both `frob-cli` and `surface/frob-mcp` (ADR-TETHER-01), enforcing build-time registry lints `RL-01` through `RL-24` so neither surface can drift or admit malformed entries. Under `--no-default-features --features registry-only` (ADR-TETHER-02), expose the registry table, name map, schema identifiers, and strict parser/linter without linking `frob-fsim` or any physics or subsystem crate.

### Registry Format (`frob.verbs/1`) and Specimen Entry

Top-level tables admitted at M0 (closed schema; any other top-level key fails `RL-13` `unknown-field`): `schema = "frob.verbs/1"`, `[[verb]]`, `[[task_cluster]]`, `[[retired_name]]` (requiring `name`, `use`, `reason`), `[counts]`, and `[[addition]]` (`[[admission_check]]` is added by N32ax-01 at M2).

```toml
schema = "frob.verbs/1"

[[verb]]
name = "design.check"
cluster = "design"
kind = "read"
tier = "S"
capabilities = ["design:read"]
input_schema = "frob.design.check.in/1"
output_schema = "frob.design.check.out/1"
refusals = ["FROB-REF-101", "FROB-REF-104", "FROB-REF-107", "FROB-REF-110"]
idempotent = true
cost_model = "static: under 2 s for template-sized designs"
first_milestone = "M1"
task_clusters = ["author", "places"]
examples = ["frob design check quad-a --json"]
purpose = "Run the preflight stages and the analyses marked for check; return findings with ranked repairs."
use_when = "After any edit and before any simulation."
owner = "frob-design"
schema_version = 1
paginated = false
exports = false
estimable = false
```

Later-bead entry fields remain refused as `unknown-field` until their owning bead lands: `needs_groups` (N33c-32, M0), `memo` (BD-111, M1), `reports` (BD-139, M1), `threats` (N37-01, M1), `uses` (N35-07, M2), `consumes` and `claim_kinds` (BD-308, M3), `consumes_evidence` (N37-26, M3, scoped to non-pass-through/non-screening verbs plus `accepts = "verdict"` parameters), `max_wall_ms` (N37-13, M3), `same_engine_as` (N28c-29, M3), and `engines` (N32ax-02, M4).

## Inputs

- `registries/verbs.toml` (or the negative-control override `FROB_VERBS_REGISTRY`, rejected in release verification by `N34p4-03`'s `xtask check-generated`).
- Referenced JSON Schema files under `schemas/` (`<schema id with / replaced by .>.json`, or `FROB_VERBS_SCHEMAS_DIR`).
- `registries/refusals.toml` through `frob_core::registry::refusals::RefusalRegistry`.
- Optional `registries/scopes.toml` (warning-only for `RL-15` until M2 `N32b-01` lands it, error thereafter).
- Cargo feature flags `registry-only`, `handlers` (default), and `conformance` (debug/test builds only).

## Outputs

- `$OUT_DIR/verbs_table.rs` included by `frob_verbs::registry`:
  - `VERBS: [VerbEntry; N]` sorted strictly by `name` (UTF-8 / code-point order) for `O(log N)` binary search (`lookup`, `lookup_tool`).
  - `TASK_CLUSTERS: [TaskCluster; K]` (empty at M0; populated by NJL-05 at M1).
  - `SCHEMAS: [(SchemaId, &str); S]` carrying canonical JSON schema text (with `conformance_only` properties stripped whenever `feature = "conformance"` is disabled).
  - `REGISTRY_DIGEST: [u8; 32]` and `REGISTRY_DIGEST_HEX: &str`: domain-separated BLAKE3 digest over the canonical JSON serialization of sorted entries under `frob.verbs/1`.
- Pure parser, linter, and deterministic code generator (`frob_verbs::registry::{entry, lint, names, schema_id, codegen}`) shared directly with `build.rs`.

## Invariants

- **Zero `unsafe`, zero stdout/stderr printing in library code (L-7), zero `serde`, zero async runtime.**
- **Parity by construction (ADR-TETHER-01):** One build-time static table governs both CLI and MCP surfaces; no per-verb code belongs in surface crates.
- **Closed schema (`RL-13`):** Unknown top-level keys, entry fields, action members, wait members, or later-bead fields fail the build immediately rather than defaulting.
- **One-to-one MCP tool-name map (`RL-01`, `RL-02`):** Verb names contain no underscores, match segment grammar `^[a-z][a-z0-9]*(-[a-z0-9]+)*(\.[a-z][a-z0-9]*(-[a-z0-9]+)*)?$`, and map injectively (`.` and `-` → `_`) across all registered entries and the three base tools (`session_focus`, `job_wait`, `job_cancel`).
- **Task-cluster cap (`RL-04`):** Every verb belongs to at least one task cluster; no task cluster exceeds 7 non-base tools.
- **Conformance isolation (`RL-24`):** `feature = "conformance"` is a compile error in release builds; without it, `conformance_only` properties are removed from embedded `SCHEMAS` while `REGISTRY_DIGEST` remains invariant.
- **Registry-only boundary (ADR-TETHER-02):** `--no-default-features --features registry-only` links only `frob-core` (plus `frob-core`'s own layer-0 utilities `fs-blake3` and `fs-qty`), with zero `frob-fsim`, zero FrankenSim physics crates, and zero subsystem crates.

## Refusals

- Build-time failures only in N32a-02 (all `RL-01`..`RL-24` violations fail `build.rs` with non-zero exit naming the offending entry, lint ID, slug, and value).
- `VerbEntry::resolve_action` returns `FROB-REF-802` (exit 3) when passed an undeclared action name (including naming an unnamed default action, per NJL-01 action rule 1).
- `FROB-REF-990` is explicitly forbidden in any chapter `refusals` list (`RL-06` `internal-fault-not-listable`).

## Determinism class

`D0` (Exact). `verbs_table.rs` and `REGISTRY_DIGEST` are pure functions of the sorted registry entries and canonicalized schema files with zero timestamps, zero host paths, and permutation invariance over entry order in `registries/verbs.toml`.

## Evidence class of outputs

`Tested`. The build script, lints, round-trip serialization, permutation invariance, and planted negative controls are verified by unit tests and `scripts/e2e/tether_registry.sh` (generation half of `GATE-TETHER-02`).

## Validity domain

- `registries/verbs.toml`: UTF-8 without BOM inside the strict `frob-core::registry::toml` subset (`<= 16 MiB`, depth `<= 64`).
- `purpose`: `1..=200` Unicode scalar characters.
- `wait` (required iff default kind or any action is `run`): `cli == "to-completion"`, `0 <= mcp_ms <= mcp_max_ms <= 30_000`.
- `schema_id`: `^frob(\.[a-z0-9-]+)+/[1-9][0-9]*$` with major matching `schema_version`.
- Build budget: `< 1 s` for 300 entries.

## Oracles

- Known-answer name mappings (`design.check` → `design_check`, `telemetry.inspect-pixel` → `telemetry_inspect_pixel`, `robot-docs` → `robot_docs`) and base-tool set `{session_focus, job_wait, job_cancel}`.
- Planted negative controls for `RL-01`..`RL-24` (unit suite `cargo test -p frob-verbs registry` and `scripts/e2e/tether_registry.sh` via `FROB_VERBS_REGISTRY`).
- Byte-identical regeneration and entry-order permutation invariance (`D0`).
- Unit-test conventions (`STD-TEST`): [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, and determinism lanes.

## Known gaps

- `registries/verbs.toml` begins schema-valid with zero `[[verb]]` rows at N32a-02; M0 operator verb entries (`doctor`, `capabilities`, `schema`, `robot-docs`) land in BD-022 and milestone verb rows in BD-137..BD-409.
- Kind-based derivation (`derive.rs`: `common_properties`, `short_doc`, `all_refusals`, `tags`, `annotations`) and `RL-11` (`description-too-long`) land in NJL-01.
- Envelope types (`frob.envelope/1`) land in N32a-03; dispatch service lands in N32a-04; `RL-INT` (`integer-type`) lands in N32a-07; `xtask check-generated` verb-table rule (`N34p4-03` check) is wired in BD-022.
- `frob-core` unconditionally links layer-0 utilities `fs-blake3` and `fs-qty` from FrankenSim, so `cargo tree -p frob-verbs --no-default-features --features registry-only -e normal` carries those two non-physics utilities transitively through `frob-core` and zero physics or subsystem crates.
