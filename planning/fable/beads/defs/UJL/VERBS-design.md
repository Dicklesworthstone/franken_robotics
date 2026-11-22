---
key: "VERBS-design"
title: "frob-verbs registry: cluster `design`, twelve rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "skeleton", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-343", "BD-113", "BD-110", "BD-244", "BD-058", "N28b-01", "BD-059", "BD-109", "N14a-21", "BD-065", "BD-066", "N14a-25", "BD-057", "BD-137", "BD-376", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§14.4", "§14.7", "§14.8", "§14.10", "§14.11", "§14.12", "§28.7", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `design`. The cluster covers the design language: creating a
design from a template, a file or a mission cell, typed edits and patches with an expected parent, the eight-stage
preflight `design.check`, lineage (diff, blame, bisect), import and export with loss reports, and the two views of
the canonical term. It has twelve verbs (7 `read`, 1 `run`, 4 `mutate`); the handlers live in `frob-analyze`,
`frob-design`, `frob-io`, `frob-opt`. The bead carries every row completely, with the three checks that keep the
rows true: the generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema
printed by `frob <verb> --help --json` is stable.

Kinds: `read` returns stored or cheaply computed facts and changes nothing; `run` spends compute, takes an
idempotency key and a budget, and after a bounded synchronous wait returns a job handle with exit code 8; `mutate`
changes a named object in the caller's namespace and takes an idempotency key and an expected parent; `admin`
cancels, stops, evicts or deletes and needs a capability scope. Tiers: S Solid, F Frontier (with a pilot and a kill
gate), M Moonshot; `S/F` is a Solid core with Frontier extensions.

DESIGN: a registry bead per cluster exists beside the implementing beads because a row missing from the registry,
spelled twice, or served with different refusal lists on the CLI and over MCP is a surface defect that no mechanism
test catches. Rejected: rows left to the implementing beads alone (nothing checks their union against the
inventory); one bead for all 243 rows (too large to review).

## Scope

In: the twelve registry entries of cluster `design`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `design.bisect` | run | M4 | S | Find the edit in a lineage that changed a verdict, with an anytime-valid comparison at each step | §28.7 |
| `design.blame` | read | M1 | S | The edit that last changed a field or a derived quantity | §28.7 |
| `design.check` | read | M1 | S | The eight preflight stages and the analyses marked for `check`; findings with ranked repairs | §14.11, §28.4 |
| `design.diff` | read | M1 | S | Structural and parameter difference of two versions, with the symmetry effect | §28.7 |
| `design.edit` | mutate | M1 | S | One typed edit; returns the new `DesignId`, the symmetry effect and the invalidation set | §14.7 |
| `design.patch` | mutate | M1 | S | Several typed edits against a named parent, with locks and required analyses | §14.8 |
| `design.templates` | read | M1 | S | The eighteen archetypes and their parameters | §14.10 |
| `design.new` | mutate | M1 | S | Create a design from a template, a file or a mission cell | §14, §32.6 |
| `design.show` | read | M1 | S | Show a design and its compiled sections | §14, §32.6 |
| `design.import` | mutate | M1 | S | Import a robot description with a loss report | §14.12, §32.6 |
| `design.export` | read | M1 | S | Export a design with the reverse loss report | §14.12, §32.6 |
| `design.view` | read | M1 | S | Convert between the JSON and S-expression views | §14.4, §32.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.

**`design.bisect`** (MCP tool `design_bisect`; owner `frob-opt`)
- Inputs: lineage (two versions); verdict predicate; study level.
- Outputs: the edit that changed the verdict, with the anytime-valid comparison at each step; `first-change` only
  with a prefix scan or a declared monotone predicate. Evidence: *study*.
- Chapter refusals: `FROB-REF-550` (`first-change` asked of bisect without a prefix scan or a declared monotone
  predicate).
- Implemented by: BD-343.

**`design.blame`** (MCP tool `design_blame`; owner `frob-opt`)
- Inputs: design; field path or derived quantity.
- Outputs: the edit that last changed it, with author, parent and patch identifier. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-113.

**`design.check`** (MCP tool `design_check`; owner `frob-analyze`)
- Inputs: design; optional scene, budget or skill for the bound analyses.
- Outputs: per-stage results of the eight preflight stages; findings with code, severity and ranked repairs as typed
  edits; `closure`, `symmetry`, timing per stage. Evidence: *static*.
- Chapter refusals: `FROB-REF-101` (port type mismatch; joint class and screw basis disagree); `FROB-REF-104`
  (inertia not physically consistent (the smallest eigenvalue of the pseudo-inertia is reported)); `FROB-REF-107`
  (empty joint range); `FROB-REF-110` (a closure that over-constrains the mechanism (mobility below one)).
- Implemented by: BD-110, BD-244.

**`design.diff`** (MCP tool `design_diff`; owner `frob-opt`)
- Inputs: two design versions.
- Outputs: structural and parameter differences as typed edits, with the symmetry effect. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-113.

**`design.edit`** (MCP tool `design_edit`; owner `frob-design`)
- Inputs: design; one typed edit with arguments; expected parent.
- Outputs: the new `DesignId`, its parent, the symmetry effect (`preserved`, `broken`, `restored`), the invalidation
  set. Evidence: *static*.
- Chapter refusals: `FROB-REF-102` (unit or dimension error); `FROB-REF-103` (unknown, misspelt or duplicate field);
  `FROB-REF-109` (declared symmetry differs from computed symmetry); `FROB-REF-111` (a lever outside its bounds; an
  edit to a locked field).
- Implemented by: BD-058, N28b-01.

**`design.patch`** (MCP tool `design_patch`; owner `frob-design`)
- Inputs: `frob.design.patch/1`: several typed edits against a named parent, with locks and required analyses.
- Outputs: the new `DesignId`, per-edit results, the invalidation set; all or nothing. Evidence: *static*.
- Chapter refusals: `FROB-REF-103` (unknown, misspelt or duplicate field); `FROB-REF-111` (a lever outside its
  bounds; an edit to a locked field); `FROB-REF-112` (stale parent in a patch).
- Implemented by: BD-059.

**`design.templates`** (MCP tool `design_templates`; owner `frob-design`)
- Inputs: optional archetype filter.
- Outputs: the eighteen archetypes with their parameters, bounds and defaults. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-109.

**`design.new`** (MCP tool `design_new`; owner `frob-design`)
- Inputs: name; a template with parameters, a file, or a mission cell (`--from-cell <mission>/<cell>`).
- Outputs: `DesignId`, the parameter point (the cell centre for a mission cell), the computed symmetry groups.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-102` (unit or dimension error); `FROB-REF-103` (unknown, misspelt or duplicate field);
  `FROB-REF-125` (unsupported schema version).
- Implemented by: N14a-21.

**`design.show`** (MCP tool `design_show`; owner `frob-design`)
- Inputs: design; optional section.
- Outputs: the design's canonical term and compiled sections (model identity, mass roll-up, symmetry, levers,
  locks). Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: N14a-21.

**`design.import`** (MCP tool `design_import`; owner `frob-io`)
- Inputs: a URDF or MJCF document; `--strict`.
- Outputs: `DesignId` and a loss report: every element dropped, approximated or defaulted. Evidence: *static*.
- Chapter refusals: `FROB-REF-103` (unknown, misspelt or duplicate field); `FROB-REF-122` (an unsupported element in
  an import under `--strict`); `FROB-REF-125` (unsupported schema version); `FROB-REF-832` (an import over its size,
  depth or polygon limit).
- Implemented by: BD-065, BD-066.

**`design.export`** (MCP tool `design_export`; owner `frob-io`)
- Inputs: design; target format (URDF, MJCF).
- Outputs: the exported document under the session's export root and the reverse loss report. Evidence:
  *pass-through*.
- Chapter refusals: `FROB-REF-122` (an unsupported element in an import under `--strict`); `FROB-REF-733` (the
  licence set of the artifact forbids this export).
- Implemented by: N14a-25, BD-066.

**`design.view`** (MCP tool `design_view`; owner `frob-design`)
- Inputs: design or document; `--as sexpr` or `--as json`.
- Outputs: the other view of the same canonical term; the round trip is the identity on canonical forms. Evidence:
  *pass-through*.
- Chapter refusals: `FROB-REF-103` (unknown, misspelt or duplicate field); `FROB-REF-125` (unsupported schema
  version); `FROB-REF-892` (`view-is-not-identity`: A view is offered where an identity is required: to hash, to
  sign, to cite as evidence, or as a cache key).
- Implemented by: BD-057.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `design.bisect` | `design:run` | `frob.design.bisect.out/1` | `search` | `frob design bisect quad-a --from d:41aa --to d:7c03 --task stairs-0.4 --key bis-1 --json` |
| `design.blame` | `design:read` | `frob.design.blame.out/1` | `search` | `frob design blame quad-a /limbs/of/of/chain/2/transmission/belt/ratio --json` |
| `design.check` | `design:read` | `frob.design.check.out/1` | `author`, `places` | `frob design check quad-a --json` |
| `design.diff` | `design:read` | `frob.design.diff.out/1` | `author` | `frob design diff quad-a@prev quad-a --json` |
| `design.edit` | `design:write` | `frob.design.edit.out/1` | `author`, `repair` | `frob design edit quad-a set-param foot.radius 0.030 m --expected-parent d:7a3e --key ed-1 --json` |
| `design.patch` | `design:write` | `frob.design.patch.out/1` | `modules` | `frob design patch quad-a patch.json --key pat-1 --json` |
| `design.templates` | `design:read` | `frob.design.templates.out/1` | `orient` | `frob design templates --json` |
| `design.new` | `design:write` | `frob.design.new.out/1` | `author` | `frob design new quad-a --template quadruped-12dof --key des-1 --json` |
| `design.show` | `design:read` | `frob.design.show.out/1` | `author` | `frob design show quad-a --json` |
| `design.import` | `design:write` | `frob.design.import.out/1` | `author` | `frob design import arm.urdf --name arm-a --key imp-1 --json` |
| `design.export` | `design:read` | `frob.design.export.out/1` | `modules` | `frob design export quad-a --as urdf --json` |
| `design.view` | `design:read` | `frob.design.view.out/1` | `modules` | `frob design view quad-a --as sexpr --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. Stated bounds: `design.check`: static: under 2 s for
template-sized designs.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "design.bisect"
cluster = "design"
kind = "run"
tier = "S"
capabilities = ["design:run"]
input_schema = "frob.design.bisect.in/1"
output_schema = "frob.design.bisect.out/1"
refusals = ["FROB-REF-550"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["search"]
examples = ["frob design bisect quad-a --from d:41aa --to d:7c03 --task stairs-0.4 --key bis-1 --json"]
owner = "frob-opt"
purpose = "Find the edit in a lineage that changed a verdict, with an anytime-valid comparison at each step"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel`; `mutate`: `idempotency_key`,
   `expected_parent`) and appends the surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid),
   724 (not implemented in this build) and 802 (schema failure, including an unknown parameter) for every verb; 701
   to 704 (budget and admission), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`; 806 and 807 for paginated reads; 808 for an out-of-range integer.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1). Until its own milestone a verb answers one envelope
   with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent.

**Notes for this cluster.**

- The cluster has twelve verbs, more than an agent should see at once; the task clusters `author`, `modules`,
  `search` and `repair` each list at most seven tools (bead NJL-05).
- Handlers live in four crates: `frob-design` (the term, edits, views), `frob-analyze` (`design.check`), `frob-opt`
  (lineage) and `frob-io` (import, export). The registry field `owner` records which.
- `design.edit` and `design.patch` are `mutate` verbs: they take an idempotency key and an expected parent; a stale
  parent is refused (`FROB-REF-804` at the surface, `FROB-REF-112` inside a patch). The envelope's `bound` block
  echoes the immutable `DesignId` every name resolved to.
- `design.check` returns exit 2 when it has findings and exit 0 when it has none; a refusal (exit 3) is reserved for
  a design the compiler cannot read. TARGET: under 2 s for a template-sized design.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly twelve entries with `cluster = "design"` (7 `read`, 1 `run`, 4
      `mutate`), each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no
      second spelling.
- [ ] `frob-surface-diff parity --cluster design` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 12, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_design.sh` runs each verb of the cluster (twelve), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/design/<verb>.help.json` byte for byte for each of
      the twelve, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_design` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 12 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `design.view` is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`design.bisect`: 550; `design.blame`: none;
  `design.check`: 101, 104, 107, 110); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`design_bisect`, `design_blame`, `design_check`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; a verb
  whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_design.sh` (header: `bead=VERBS-design milestone=M1 labels=tether,verbs,verb:design.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance design`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the twelve verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
design` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster design`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_design/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 12 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 12 verbs (7 `read`, 1 `run`, 4 `mutate`); the tables above. KA-755 (the exit-code
  table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the
  surface's tables; exact) is exercised for every verb by steps (6) and (7).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`). A
  verb answering with two envelopes or exit code 1 fails step (7).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Kill gate KILL-SKEL-SEXPR (the S-expression view cannot be kept lossless): fallback, JSON is the only syntax and
  `design.view` keeps its row with `--as json` only.
- `design.bisect` first answers at M4; `first-change` is refused without a prefix scan or a declared monotone
  predicate (`FROB-REF-550`), because verdicts along a lineage need not be monotone.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M4. The per-milestone
  surface beads (BD-137, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §14.4, §14.7, §14.8, §14.10,
§14.11, §14.12, §28.7, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
