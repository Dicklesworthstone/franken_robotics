---
key: "NJL-01"
title: "registries/verbs.toml: the entry schema of one verb, the four kinds, the tool-name map and the registry lints"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-005", "BD-022", "STD-E2E"]
needs: []
related: ["BD-023", "BD-137", "BD-138", "BD-004", "NJL-02", "NJL-03", "NJL-04", "NJL-05", "STD-TEST"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (preamble, J.1)", "§32.2", "§32.3 (ADR-TETHER-01, -02, -09, -11)", "§32.6", "§32.8.2", "§32.12.3", "§32.12.5", "§32.19 (INV-TETHER-01, -07)", "§34.6"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through one command-line binary `frob`, a
daemon `frobd` and an MCP server `frob-mcp` (MCP: the Model Context Protocol, by which an agent harness lists and
calls tools). Every capability is a **verb**, written `cluster.verb` (`design.check`, `telemetry.inspect-pixel`).
All verbs are declared once, in `registries/verbs.toml`, owned by crate `frob-verbs` (layer 6, tier Solid). The CLI
subcommands, the daemon protocol, the MCP tool definitions, the input and output schemas, the refusal table and
the agent guide (`frob robot-docs`) are generated from that file, so the surfaces cannot drift. Gate
GATE-TETHER-02 (TARGET, claim Known answer, M1) states it: both surfaces are generated from the registry and
their parity diff is empty; every verb has a schema, an example and a refusal list; the served MCP `tools/list`
equals the registry under the name map.

This bead fixes the registry's data model: the fields of one entry, the meaning of the four kinds, the name rules,
and the lints that make a malformed registry fail the build. The seed bead BD-022 builds the loader and the first
four verbs; this bead is the contract every later row is written against, and every `VERBS-<cluster>` registry
bead depends on it.

Decisions that govern it, with their reasons:

- **ADR-TETHER-01.** The registry is compiled into a static table in `frob-verbs` by its build script. Neither
  `frob-cli` nor `frob-mcp` contains per-verb code, with one exception (the MCP half of `session.focus`, which
  edits the visible tool set). Reason: parity by construction; the gate then checks the served surfaces, not the
  generator's intentions. Rejected: hand-written clap commands and hand-written `#[tool]` handlers per verb (two
  lists of 243 names that drift); a registry read at run time (a deployment could then serve a surface the tests
  never saw).
- **ADR-TETHER-02.** `frob-verbs` has a feature `registry-only` with no dependency on any physics crate or on the
  FrankenSim airlock crate `frob-fsim`. Reason: it is what the first fallback of kill gate KILL-MCP links (the MCP
  server built as its own workspace must still share the table, the schemas and the envelope types).
- **ADR-TETHER-09.** Integers that can exceed 2⁵³ − 1 travel as decimal strings; counts are range-checked numbers.
  Reason: a client that decodes JSON numbers as IEEE doubles silently changes larger values (2⁵³ + 1 =
  9,007,199,254,740,993 decodes to 9,007,199,254,740,992).
- **ADR-TETHER-11.** Mutating and job-submitting verbs bind immutable identifiers; a mutable name is resolved once,
  before the request is digested, and the resolution is echoed in the envelope's `bound` block. Reason: otherwise a
  retry after a name has moved would mean a different request under the same idempotency key.

Terms. *Envelope*: the single JSON object (`frob.envelope/1`) every verb writes with `--json`. *Refusal*: a typed
failure with a stable code `FROB-REF-nnn`, ranked repairs and an exit code. *Task cluster*: a set of at most
seven tools an agent sees at a time (bead NJL-05); it differs from a registry cluster, which is a namespace.
*Tier*: S (Solid), F (Frontier) or M (Moonshot), the maturity class of the mechanism behind the verb.

## Scope

In: the TOML schema of one `[[verb]]` entry and of the registry file; the Rust type the build script emits; the
four kinds and the common properties each adds; verbs with two kinds; name grammar, the CLI path and the MCP
tool-name map; schema identifiers; the build-time lints; the per-surface wait default; the behaviour of a
registered verb that is not implemented yet. Out: the loader skeleton, the envelope and the exit codes (BD-022);
the rows themselves (the `VERBS-<cluster>` beads and NJL-03); the conformance harness that compares served
surfaces with the registry (NJL-02); the alias rule (NJL-04); task-cluster assignment (NJL-05); idempotency
transactions, pagination and the bounded wait as mechanisms (the Tether seed beads BD-211 and BD-298).

## Technical approach

**One entry (the form every row takes).**

```toml
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
```

| Field | Type | Rule |
|---|---|---|
| `name` | string | Unique. Grammar below. The name its owner chapter gives the verb; no second spelling exists (NJL-04). |
| `cluster` | string | The registry namespace. Equals the part of `name` before the dot; the four operator verbs (`doctor`, `capabilities`, `robot-docs`, `schema`) have no dot and `cluster = "operator"`. |
| `kind` | `read` \| `run` \| `mutate` \| `admin` | See the kinds table. |
| `tier` | `S` \| `F` \| `M` | Maturity of the mechanism. An `F` or `M` verb is listed by `frob capabilities` with its pilot and kill gate. |
| `capabilities` | list of scopes | Scopes of the session token the verb needs. Every scope must exist in the scope registry; a call without one is `FROB-REF-720`, exit 5. |
| `input_schema`, `output_schema` | schema identifiers | `frob.<cluster>.<verb>.in/1` and `.out/1` (operator verbs: `frob.<verb>.in/1`), or a named artifact schema where the verb returns an artifact (`frob.verdict/1`, `frob.plan/1`). The trailing integer is the major version. |
| `refusals` | list of codes | The chapter-specific refusals the verb may raise. Each must exist in `registries/refusals.toml`. Surface-wide refusals are appended by the generator by kind and are not repeated per row (table below). |
| `idempotent` | bool | `true` when repeating the call changes nothing (every `read`; an `admin` cancel or stop). For `run` and `mutate` it is `false` and the generator adds the required `idempotency_key`: a retry with the same key and the same resolved content returns the stored receipt. |
| `cost_model` | string | `static: <bound>` for a verb with a stated bound, or the name of the estimator `--estimate` uses. Never empty. |
| `first_milestone` | `M0`…`M8` | The milestone at which the verb, possibly in a reduced form, first answers. Before that it answers `FROB-REF-724`. |
| `task_clusters` | list | The task clusters that list the tool. At least one; `["base"]` for the three always-listed base tools. |
| `examples` | list | At least one complete command line that the end-to-end script of the cluster runs verbatim. |
| `purpose`, `use_when` | strings | DESIGN additions (the appendix shows no field for them, but the generated tool description needs one sentence of purpose and when to use the tool). `purpose` ≤ 200 characters. |
| `owner` | crate name | DESIGN addition: the crate whose handler answers. Used by the layer check (a handler lives in the subsystem crate, never in `frob-cli` or `frob-mcp`). |
| `wait` | table, `run` only | The per-surface default of the bounded synchronous wait: `{ cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`. A registry field so that the parity check knows the one sanctioned per-surface difference. |
| `actions` | list of tables | Only for a verb with two kinds (below). |

**The four kinds.**

| Kind | Meaning | Common input properties the generator adds | Surface-wide refusals appended |
|---|---|---|---|
| `read` | Returns stored or cheaply computed facts and changes nothing | Paginated reads: `cursor`, `limit`, `follow` | 720, 723, 724, 802, 808; paginated: 806, 807 |
| `run` | Spends compute; takes an idempotency key and a budget; accepts the bounded synchronous wait and otherwise returns a job handle with exit code 8 | `sync_wait_ms` (0 to 30,000; MCP default 5,000; CLI default: wait to completion), `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave_running` default, or `cancel_job`) | 701, 702, 703, 704, 720, 723, 724, 802, 803, 805, 808 |
| `mutate` | Creates or changes a named object in the caller's namespace; takes an idempotency key and an expected parent | `idempotency_key`, `expected_parent` | 720, 723, 724, 725, 802, 803, 804, 805, 808 |
| `admin` | Cancels, stops, evicts or deletes; needs a capability scope | none | 720, 723, 724, 802 |

Meanings of the appended codes: 701 the estimate's upper end exceeds the remaining budget (exit 4); 702 budget
exhausted during a run, stopped at a safe boundary, nothing partial published (4); 703 admission guard, `retry:
backoff` (4); 704 a campaign-class job without an approved compute envelope (4); 720 the token lacks a scope (5);
723 token invalid or expired (5); 724 the verb is not implemented in this build (5); 725 a write outside the
caller's namespace (5); 802 arguments fail the schema, including an unknown parameter or an out-of-range
`sync_wait_ms` (3); 803 idempotency key reused with different resolved content (7); 804 stale parent (7); 805 a
mutating or job-submitting verb without an idempotency key (3); 806 cursor does not match the query (3); 807
cursor expired (3); 808 an integer outside ±(2⁵³ − 1) in a number-typed field (3).

**Verbs with two kinds (DESIGN).** Three inventory rows print two kinds: `nemesis.hardset` (read, run),
`scene.bank` (read, admin), `spark.cache` (read, admin). An entry has one `kind`, the kind of its default
action, and an `actions` list for the others: `actions = [{ name = "evict", kind = "admin", capabilities =
["spark:admin"] }]`. The invoked action is the input property `action`; the common properties, the scope and
the appended refusals follow the kind of that action. The MCP annotation `read_only` is set only when every
action is `read`. Rejected: two registry names per verb (it would add a spelling the chapters do not use).

**Names.**
1. Grammar: `segment ::= [a-z][a-z0-9]*("-"[a-z0-9]+)*`; a cluster verb is `segment "." segment`; an operator
   verb is one `segment`. No registry name contains an underscore.
2. CLI path: the segments in order, `frob design check …`, `frob telemetry inspect-pixel …`, `frob doctor`.
3. MCP tool name: the verb name with `.` and `-` replaced by `_` (`telemetry.inspect-pixel` →
   `telemetry_inspect_pixel`).
4. The map must be one-to-one **on the registered set**, checked on the image, not argued from the grammar.
   Counterexample: the operator verb `robot-docs` and a hypothetical cluster verb `robot.docs` both map to
   `robot_docs`; neither contains an underscore.

**Emitted type (DESIGN sketch; fixes intent, not signatures).**

```rust
pub struct VerbEntry {
    pub name: &'static str, pub cluster: &'static str, pub kind: Kind, pub tier: Tier,
    pub capabilities: &'static [Scope], pub input_schema: SchemaId, pub output_schema: SchemaId,
    pub refusals: &'static [RefusalCode], pub idempotent: bool, pub cost_model: &'static str,
    pub first_milestone: Milestone, pub task_clusters: &'static [&'static str],
    pub examples: &'static [&'static str], pub purpose: &'static str, pub use_when: &'static str,
    pub owner: &'static str, pub wait: Option<WaitDefault>, pub actions: &'static [Action],
}
impl VerbEntry {
    pub fn tool_name(&self) -> String;                   // the name map
    pub fn cli_path(&self) -> Vec<&'static str>;
    pub fn short_doc(&self) -> String;                   // generated, at most 800 characters (TARGET)
    pub fn input_schema_with_common_fields(&self) -> Schema;
    pub fn envelope_schema(&self) -> Schema;             // frob.envelope/1 with `result` specialized
    pub fn tags(&self) -> Vec<String>;                   // cluster:<c>, kind:<k>, task:<t>…, tier:<S|F|M>
    pub fn all_refusals(&self) -> Vec<RefusalCode>;      // chapter list plus the appended surface-wide codes
}
pub static REGISTRY: &[VerbEntry];                       // sorted by name; emitted by build.rs
```

**Lints (each fails the build of `frob-verbs` with a named error; a registry that loads is a registry that
passed all of them).** `unknown-field` (a misspelt field is a refusal, never a default); `duplicate-name`;
`name-grammar`; `cluster-mismatch`; `tool-name-collision`; `kind`; `tier`; `milestone`; `unknown-refusal`;
`unknown-scope`; `schema-id-form`; `empty-examples`; `example-not-this-verb` (the example's command path must
be the verb's CLI path); `empty-task-clusters`; `empty-cost-model`; `run-without-wait`; `wait-on-non-run`;
`mcp-wait-out-of-range` (default 0 to max, max ≤ 30,000); `owner-is-surface` (the owner may not be `frob-cli` or
`surface/frob-mcp`); `integer-type` (a schema may use only `int53`, `u64str`, `i64str`; any other integer type
fails); `purpose-too-long`; `description-too-long` (the generated description above 800 characters).

**A registered verb that is not implemented yet.** Rows are entered when their cluster is first registered,
whatever their `first_milestone`. Until its milestone, a verb answers one envelope with `ok: false`,
`FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent. Reason: the agent
guide and the tool list are generated once and stay stable across milestones.

**Files.** `registries/verbs.toml`; `crates/frob-verbs/build.rs` (parse, lint, emit `registry_table.rs` into
`OUT_DIR`); `crates/frob-verbs/src/registry/{entry,lint,names,schema_id}.rs`; `schemas/frob.verbs.entry.1.json`
(generated; the schema of one entry, printed by `frob schema frob.verbs.entry/1 --json`). The build script reads
the path in `FROB_VERBS_REGISTRY` when it is set; that override exists for the negative-control lane only, and the
generated-file check fails if a release build was made with it.

## Acceptance criteria

- [ ] The specimen entry above parses to the type; every field of the table is read; `frob schema
      frob.verbs.entry/1 --json` prints the entry schema with a minimal valid example.
- [ ] Each of the 22 lints fires on its planted defect and names it (negative controls, 22 of 22); the clean
      registry builds.
- [ ] `tool-name-collision` catches `robot-docs` against a planted `robot.docs`.
- [ ] The generator appends exactly the surface-wide codes of the kinds table to each verb's refusal list, and
      `frob schema <verb> --json` shows chapter codes and appended codes separately.
- [ ] A `run` entry carries the wait default; the generated MCP input schema has `sync_wait_ms` with default
      5,000 and maximum 30,000; a value of 30,001 is `FROB-REF-802`, exit 3.
- [ ] A two-kind verb exposes its `action` property; calling the `admin` action without its scope is
      `FROB-REF-720`, exit 5, and the `read` action still works.
- [ ] A registered, unimplemented verb answers one envelope, `FROB-REF-724`, exit 5, on the CLI and over MCP.
- [ ] `crates/frob-verbs/CONTRACT.md` (ten sections) states the entry schema, the kinds table and the lints.
- [ ] With feature `registry-only`, `cargo tree -p frob-verbs` shows no physics crate and no `frob-fsim`.
- [ ] The emitted table and the generated schemas are byte-identical across two builds, debug and release, Linux
      and macOS (determinism class D0).

## Test plan

### Unit tests

`cargo test -p frob-verbs --test registry_schema` (no mocks; fixture registries are TOML strings labelled
`synthetic`):
- `entry_specimen_parses`: the `design.check` entry above → kind `read`, tier `S`, four refusals, two task
  clusters, tool name `design_check`, CLI path `["design", "check"]`.
- `name_map_examples`: `telemetry.inspect-pixel` → `telemetry_inspect_pixel`; `robot-docs` → `robot_docs`;
  `eval.rank-agreement` → `eval_rank_agreement`.
- `name_map_injective_on_registry`: the image of the registered set has as many elements as the set.
- `lint_<name>` for each of the 22 lints: one planted registry each; the error names the lint and the row.
- `appended_refusals_by_kind`: a `read` gets {720, 723, 724, 802, 808}; a paginated `read` also {806, 807}; a
  `run` gets {701, 702, 703, 704, 720, 723, 724, 802, 803, 805, 808}; a `mutate` gets {720, 723, 724, 725, 802,
  803, 804, 805, 808}; an `admin` gets {720, 723, 724, 802}.
- `common_properties_by_kind`: the generated input schema of a `run` has exactly the five added properties, of a
  `mutate` the two, of a `read` none.
- `sync_wait_bounds`: 0, 5,000 and 30,000 validate; −1 and 30,001 are refused with code 802.
- `two_kind_actions`: `spark.cache` with action `evict` requires `spark:admin`; `read_only` annotation false.
- `unimplemented_answers_724`: an entry with `first_milestone` above the build's milestone → code 724, exit 5.
- `schema_integer_types_only`: a schema with a bare 64-bit integer fails `integer-type` (KA-756's control).
- Property test `prop_entry_roundtrip`: a generated valid entry serializes to TOML and parses back equal.

### End-to-end script and logging

`scripts/e2e/verbs_registry_schema.sh` (header: `bead=NJL-01 milestone=M0 labels=tether,verbs,registry
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp plants=collision,unknown-field,bare-int`). Steps: (1) `frob schema
frob.verbs.entry/1 --json`, assert the example validates; (2) `frob capabilities --json`, assert every listed
verb has kind, tier, milestone and at least one example; (3) for one verb of each kind, `frob <verb> --help
--json` and assert the common properties of its kind; (4) over MCP (in-memory transport and stdio), `tools/list`
and assert each tool name is the mapped name of exactly one registry verb; (5) a registered unimplemented verb →
`e2e_expect_refusal FROB-REF-724 5`; (6) `sync_wait_ms=30001` → `e2e_expect_refusal FROB-REF-802 3`; (7) the
negative-control lane: for each plant, build `frob-verbs` with `FROB_VERBS_REGISTRY` pointing at the planted
file and `e2e_expect_caught` the named lint. Logs go to `target/e2e-logs/verbs_registry_schema/<utc>.ndjson`
through the shared helper: every command with exit code and wall time, the envelope's evidence block, each
assertion with observed and expected values, each refusal with its code, and the final `summary`.

### Known answers and negative controls

- KA-756 (2⁵³ + 1 as `int53` is refused with `FROB-REF-808`, preserved exactly as `u64str`; exact; control: a
  schema with a bare integer type fails the schema lint). CX-348 (send 64-bit integers as JSON numbers) is the
  counterexample behind the `integer-type` lint.
- Known answer of this bead: the name map of the three examples above, and the collision of `robot-docs` with
  `robot.docs`.
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; no verb, flag, schema or
  refusal exists on one surface and not the other, except per-surface defaults declared in the registry and the
  unknown-verb case) as the property `name_map_injective_on_registry` plus the parity harness of NJL-02;
  INV-TETHER-07 (no integer travels as a JSON number unless its schema type is `int53` and its value is within
  ±(2⁵³ − 1)) as `schema_integer_types_only`.
- Negative controls: the 22 planted registries; a build with the override variable set must fail the
  generated-file check.
- Determinism lanes: twice on one machine; debug and release; Linux and macOS; class D0 (byte equality of the
  emitted table and schemas).

## Considerations

- The fields `purpose`, `use_when`, `owner`, `wait` and `actions` are DESIGN additions to the entry printed in the
  appendix; they are confined to what the generated surfaces already need. If the Tether owner spells them
  differently in BD-022, this bead's lints follow that spelling.
- Capability scope names in rows are proposals until the scope registry exists (the capability bead BD-258);
  `unknown-scope` is therefore a warning until M3 and an error after.
- The registry is a build input of every binary; a lint that is slow slows every build. Budget: the whole
  build script under 1 s for 300 entries.
- OPEN-TETHER-3 (owner: Tether lead; blocks GATE-TETHER-13): the default and maximum of `sync_wait_ms` against
  measured agent turn time. Fail-closed default 5,000 and 30,000; they are registry values, so a change is one
  edit.
- If kill gate KILL-MCP fires (INT-FMCP-01 cannot pass within the M0 budget), the registry and this bead are
  unchanged: the socket fallback links `frob-verbs` with `registry-only`.

## References

Appendix J (preamble, J.1), §32.2, §32.3, §32.6, §32.8.2, §32.12.3, §32.12.5, §32.18, §32.19, §34.6;
GATE-TETHER-02; ADR-TETHER-01, -02, -09, -11; INV-TETHER-01, -07; KA-756; CX-348; OPEN-TETHER-3.
