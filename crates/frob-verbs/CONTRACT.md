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
- Cargo feature flags `registry-only`, `handlers` (default), and `conformance` (debug/test builds only); `nc-bare-unknown-verb` and `nc-second-envelope` are planted defects for `GATE-TETHER-03`'s negative controls and are never enabled in a build.
- Envelope (N32a-03): a verb handler's outcome (a result with its `frob_core::evidence::ClaimProduct`, a `frob_core::refusal::Refusal` or a code with its fields, or a job report), the verb's output schema, and the caller's writer.

## Outputs

- `$OUT_DIR/verbs_table.rs` included by `frob_verbs::registry`:
  - `VERBS: [VerbEntry; N]` sorted strictly by `name` (UTF-8 / code-point order) for `O(log N)` binary search (`lookup`, `lookup_tool`).
  - `TASK_CLUSTERS: [TaskCluster; K]` (empty at M0; populated by NJL-05 at M1).
  - `SCHEMAS: [(SchemaId, &str); S]` carrying canonical JSON schema text (with `conformance_only` properties stripped whenever `feature = "conformance"` is disabled).
  - `REGISTRY_DIGEST: [u8; 32]` and `REGISTRY_DIGEST_HEX: &str`: domain-separated BLAKE3 digest over the canonical JSON serialization of sorted entries under `frob.verbs/1`.
- Pure parser, linter, and deterministic code generator (`frob_verbs::registry::{entry, lint, names, schema_id, codegen}`) shared directly with `build.rs`.
- `frob_verbs::envelope` (N32a-03): `frob.envelope/1` and `frob.progress/1` (hand-written schemas in `crates/frob-verbs/schemas/`), the `Envelope` builder and canonical serializer, `ExitCode`, the reader `read_envelope`, the per-verb schema `envelope_schema(entry)`, the output guard `serve`, and the stream checker `check_stream`.

### Envelope (`frob.envelope/1`)

Every verb answers with exactly one envelope; a long job writes zero or more `frob.progress/1` lines (`{"done","job","schema","state","total","unit"}`) before it, and the envelope is always the last line. Serialization is canonical (sorted members, minimal escapes, exact numbers), so equal envelopes are equal bytes.

| Field | Rule |
|---|---|
| `schema`, `verb`, `ok`, `exit` | Always present. `exit` equals the process exit code on the CLI; `ok` is true exactly for exits 0, 2 and 8. |
| `result` | Present exactly for exits 0 and 2; its schema is the verb's output schema (`envelope_schema(entry)`). |
| `evidence` | Always present with `stack_receipt` (`sr:` form of `BUILT_UNDER`) and `stack_dev: true` exactly on a dev identity. Beside a result it also carries the serialized claim product: `class`, `tier`, `model_id`, `determinism` (printed as the class, `D0`..`D4`), `claims`, `reality`, and where they apply `seed` (decimal string), `weakest`, `conditions`. Built only from a `ClaimProduct`. |
| `bound` | Mutable names resolved to immutable identifiers; `fixtures` (array of `{param, reference, path, digest}`) after the Resolve step, on refusals of later stages too. |
| `refusal` | Exactly when `ok` is false: `code` (`FROB-REF-[1-9][0-9]{2,3}`, compared as a whole token), `message`, `pointer`, `observed`, `required`, `unit`, `affected_claims`, `retry`, ranked `repairs`, and `detail` only when set (an interim raise). |
| `job` | Exit 8 (required), job verbs, and a budget stop's acknowledgement: `id`, `state`, `progress {done, total, unit}` with `done <= total` and `total >= 1`, `eta_s {point, lo, hi}`, `poll_after_ms`, `last_committed`. |
| `cursor`, `frontier` | Paginated results only; `cursor` requires `frontier`. |
| `idempotent_replay` | `true` when the envelope is the stored answer to an earlier request with the same key. |
| `authorization` | Exporting verbs (required on their results): `execution_authorized`, `hardware_safety_signoff`, `sim_to_real_certified`, plus `fabrication_released` for a build packet; all constant `false`. |
| `cost`, `budget` | `cost {wall_ms, cpu_core_s, gpu_s}`; `budget` maps `<unit>_left` to non-negative amounts. `cost.wall_ms` is the only wall-clock field. |
| `warnings`, `next_actions` | Always present arrays: `{code, message}` notes that do not change `exit`; ranked `{command, why}`. |
| `delta` | After a run: what changed since the compared run (object; field names DESIGN). |

| Exit | Meaning | Refusal codes (N35-33 `[exit_rule]`, read from the registry) | Failure class |
|---|---|---|---|
| 0 | Completed, nothing to flag | | none |
| 2 | Completed with findings or flags | | none |
| 3 | Refused input or question | `1xx`-`5xx`, `8xx` | domain |
| 4 | Budget refused at admission or stopped; nothing partial published | `70x`, `74x`, `76x` | domain |
| 5 | Capability not implemented, not admitted, or outside the token's scopes | `72x`, `73x`, `79x`; 840 | domain |
| 6 | Backend or device unavailable | `6xx` | domain |
| 7 | Conflict: key reuse with different content, stale parent, lineage | 112, 803, 804, 809 | domain |
| 8 | Accepted and still running; the envelope carries a job handle | | none |
| 9 | Indeterminate: never retry blindly, recover by key; integrity faults | 810, `9xx` (990 `internal-fault`) | transport |

Exit 1 is not representable (`ExitCode` has the nine values above and no conversion from an integer); an unhandled abort is its only source.

## Invariants

- **Zero `unsafe`, zero stdout/stderr printing in library code (L-7), zero `serde`, zero async runtime.**
- **Parity by construction (ADR-TETHER-01):** One build-time static table governs both CLI and MCP surfaces; no per-verb code belongs in surface crates.
- **Closed schema (`RL-13`):** Unknown top-level keys, entry fields, action members, wait members, or later-bead fields fail the build immediately rather than defaulting.
- **One-to-one MCP tool-name map (`RL-01`, `RL-02`):** Verb names contain no underscores, match segment grammar `^[a-z][a-z0-9]*(-[a-z0-9]+)*(\.[a-z][a-z0-9]*(-[a-z0-9]+)*)?$`, and map injectively (`.` and `-` → `_`) across all registered entries and the three base tools (`session_focus`, `job_wait`, `job_cancel`).
- **Task-cluster cap (`RL-04`):** Every verb belongs to at least one task cluster; no task cluster exceeds 7 non-base tools.
- **Conformance isolation (`RL-24`):** `feature = "conformance"` is a compile error in release builds; without it, `conformance_only` properties are removed from embedded `SCHEMAS` while `REGISTRY_DIGEST` remains invariant.
- **One envelope (INV-TETHER-02, N32a-03):** `serve` writes the handler's progress lines and then exactly one envelope; a panic, an error that maps to no code, or an envelope for another verb becomes `FROB-REF-990` (exit 9, `retry: never`, repairs: recover by key when the call carried one, then `frob doctor`); an unknown verb becomes `FROB-REF-801`.
- **Exit from the registry (N32a-03):** the exit of a refusal is the `exit` of its row in `registries/refusals.toml`; the envelope chooses no value and the reader refuses an envelope whose exit differs from the row.
- **Evidence only from a claim product (N32a-03):** no envelope entry point takes a class, tier, determinism or reality as text (compile-fail doctests in `envelope::evidence` and `envelope::exit`).
- **Registry-only boundary (ADR-TETHER-02):** `--no-default-features --features registry-only` links only `frob-core` (plus `frob-core`'s own layer-0 utilities `fs-blake3` and `fs-qty`), with zero `frob-fsim`, zero FrankenSim physics crates, and zero subsystem crates.

## Refusals

- Build-time failures only in N32a-02 (all `RL-01`..`RL-24` violations fail `build.rs` with non-zero exit naming the offending entry, lint ID, slug, and value).
- `VerbEntry::resolve_action` returns `FROB-REF-802` (exit 3) when passed an undeclared action name (including naming an unnamed default action, per NJL-01 action rule 1).
- `FROB-REF-990` is explicitly forbidden in any chapter `refusals` list (`RL-06` `internal-fault-not-listable`).
- Envelope (N32a-03): every refusal code carried in an envelope takes the exit of its registry row (`FROB-REF-802` → 3 today); `FROB-REF-816` when the reader finds authorization flags missing or not `false`; `FROB-REF-990` (exit 9) for a handler fault; `FROB-REF-801` (exit 3) for an unknown verb (the 990 envelope, exit 9, under a registry without 801's row).

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
- Envelope (N32a-03, `cargo test -p frob-verbs envelope`): golden envelopes in `tests/golden/envelope/` (completed, completed with findings, refused, deferred, paginated, idempotent replay, export, internal fault, and the plan's `design.check` example) compared byte for byte and round-tripped; the eighteen Tether codes and 990 against a synthetic mirror of N32-REF (`exit_table_exact`); the `crates/frob-conformance/fixtures/tether/exit_table.toml` rows (KA-755, `exit_fixture_rows`); the property `one_envelope_always` (10⁴ random handler programs); negative controls `control_bare_error_unknown_verb`, `control_second_envelope_serve` (fail under the `nc-*` features) and `control_wrong_exit_row_*`; end to end `scripts/e2e/tether_envelope_exit.sh`.
- Unit-test conventions (`STD-TEST`): [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, and determinism lanes.

## Known gaps

- `registries/verbs.toml` begins schema-valid with zero `[[verb]]` rows at N32a-02; M0 operator verb entries (`doctor`, `capabilities`, `schema`, `robot-docs`) land in BD-022 and milestone verb rows in BD-137..BD-409.
- Kind-based derivation (`derive.rs`: `common_properties`, `short_doc`, `all_refusals`, `tags`, `annotations`) and `RL-11` (`description-too-long`) land in NJL-01.
- Envelope (N32a-03): `112` (N14-REF) and `840` sit in N35-33's `conflict` and `capability` exception lists but are not yet rows of `registries/refusals.toml`, so the registry's lists hold only 803, 804, 809 and 810 and `Envelope::from_code` refuses 112 and 840 as unregistered until their rows land; the unit tests exercise both lists through the synthetic `MIRROR` registry. The `frob` binary and its argument parser (BD-022) do not exist, so `one_envelope_always` drives the output guard with random handler programs rather than random argument vectors, the flag-reordering metamorphic relation is not checked, and the e2e script records its CLI rows as not exercised. `GATE-TETHER-03` is not yet a row of `registries/gates.toml`. The schemas live in `crates/frob-verbs/schemas/` because `schemas/frob.*.json` is reserved for generated output.
- Dispatch service lands in N32a-04; `RL-INT` (`integer-type`) lands in N32a-07; `xtask check-generated` verb-table rule (`N34p4-03` check) is wired in BD-022.
- `frob-core` unconditionally links layer-0 utilities `fs-blake3` and `fs-qty` from FrankenSim, so `cargo tree -p frob-verbs --no-default-features --features registry-only -e normal` carries those two non-physics utilities transitively through `frob-core` and zero physics or subsystem crates.
