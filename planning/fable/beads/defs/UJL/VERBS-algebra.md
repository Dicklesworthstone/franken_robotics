---
key: "VERBS-algebra"
title: "frob-verbs registry: cluster `algebra`, two rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "marrow", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§13.5", "§13.6", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `algebra`. The cluster covers finite-group computations used by
symmetry analysis: isotypic decomposition of a representation, and orbit counts with canonical representatives. It
has two verbs (2 `read`); the handlers live in `frob-algebra`. The bead carries every row completely, with the three
checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an
end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

Kinds: `read` returns stored or cheaply computed facts and changes nothing; `run` spends compute, takes an
idempotency key and a budget, and after a bounded synchronous wait returns a job handle with exit code 8; `mutate`
changes a named object in the caller's namespace and takes an idempotency key and an expected parent; `admin`
cancels, stops, evicts or deletes and needs a capability scope. Tiers: S Solid, F Frontier (with a pilot and a kill
gate), M Moonshot; `S/F` is a Solid core with Frontier extensions.

DESIGN: a registry bead per cluster exists beside the implementing beads because a row missing from the registry,
spelled twice, or served with different refusal lists on the CLI and over MCP is a surface defect that no mechanism
test catches. Rejected: rows left to the implementing beads alone (nothing checks their union against the
inventory); one bead for all the rows (too large to review).

## Scope

In: the two registry entries of cluster `algebra`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `algebra.irreps` | read | M1 | S/F | Isotypic decomposition and block types | §13.5 |
| `algebra.orbits` | read | M1 | S | Orbit counts and one canonical representative per orbit | §13.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.

**`algebra.irreps`** (MCP tool `algebra_irreps`; owner `frob-algebra`)
- Inputs: a group source, one of: a family positionally or by `--family` (C1 to C12, D1 to D12, each optionally
  `xC2`, or S4, with its standard action); explicit signed-permutation `generators`; or a design and a task
  (`--design <id> --task <task>`: the stored Γ_obs; `--group mech` gives Γ_mech, labelled as not licensed for
  weight tying). Two channel spaces `--in` and `--out` (explicit signed permutations, `natural`, `permutation:n`,
  `regular`, `regular*k`, or a named channel space of the bound model: joint positions, joint velocities,
  actions, an observation term), or `--rep` for one space. From M4, under the feature `frontier-wreath` (N13-26,
  Frontier, pilot N13-25), `--nested` with a design and a task: nested weight tying by the wreath product K ≀ H
  (each limb's own group K, the hub's group H on the slots X), computed from generators and never enumerated.
- Outputs: group order, generators, class sizes, the character table as exact strings; per space its constituents
  (character, degree, type `real` or `complex`, conjugate, multiplicity, real dimension); `free_parameters` of
  equivariant maps (dim Hom = Σᵢ cᵢ·mᵢ^out·mᵢ^in, cᵢ = 1 for real and 2 for complex type), the unconstrained and
  bias counts, the Reynolds rank cross-check, `group_kind` (`obs`, `mech`, `declared`, `family`); `next_actions`
  pointing to `analyze.symmetry` when a design is given without a task. With `--nested`: the flat count, the
  nested count, their ratio, the nested order by the formula |K|^|X|·|H|, and the two port conditions (every limb
  automorphism preserves the limb's attachment port; the hub action preserves the decorated ports), each with
  pass or fail and the field path of a failure; when a condition fails the flat group's count is returned with
  the failed condition named. Evidence: *static* (`Certified` when the orthogonality certificate is attached); a
  nested symmetry is `Tested` (N13-26's simulator equivariance test) unless a certificate establishes it.
- Chapter refusals: `FROB-REF-214` (a representation with a constituent of quaternionic type, or a symmetry group
  outside the supported families: order above 256, or generators that do not close inside the cap);
  `FROB-REF-215` (an exact host computation over its declared size; returned with the expression, the input sizes
  and the budget).
- Implemented by: BD-048, N13-14 (the verb); exercised end to end over the gate set by N13-20; extended by
  N13-26 (`--nested`, M4, Frontier: if its pilot N13-25 or its own equivariance test falsifies the mechanism, the
  option is dropped without replacement and flat Γ_obs tying is unaffected).

**`algebra.orbits`** (MCP tool `algebra_orbits`; owner `frob-algebra`; paginated over representatives)
- Inputs: a group source as for `algebra.irreps` (family positionally or by `--family`, generators, or a design
  and a task); the slots, by `--slots <n>` with an optional `--action <name>` (`sagittal` for the left-right
  mirror of paired slots) or by `--set <name>:<n>` (`feet`, `legs`, `wheels`; `feet` defaults to k = 3); `--types
  k`; optional `--representatives` with `limit` and `cursor`; optional `--lattice` (subgroups up to conjugacy and
  the (H, K) pairs with H/K cyclic); optional `--transitions`. The positional, `--family`, `--slots` and `--set`
  forms build the same input document, echoed in `bound`.
- Outputs: `raw` (kⁿ), `orbits` (Burnside: |Γ|⁻¹·Σ_g k^c(g), c(g) the number of cycles of g on the slots),
  `factor`, `bound` (|Γ_obs|), `group_order`, `group_kind`, `cycle_index`, a page of canonical representatives with
  `orbit_size`, and, for a group resolved from a design, a warning giving the count the mechanical group would
  give. Evidence: *static*.
- Chapter refusals: `FROB-REF-214` (a symmetry group outside the supported families, or closure beyond 256
  elements); `FROB-REF-215` (an exact host computation over its declared size: the numerator Σ_g k^c(g) over its
  declared 65,536 bits, checked before any arithmetic).
- Implemented by: BD-049; its conformance battery and end-to-end script are N13-17's.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `algebra.irreps` | `algebra:read` | `frob.algebra.irreps.out/1` | `symmetry` | `frob algebra irreps C2xC2 --rep permutation:4 --json` |
| `algebra.orbits` | `algebra:read` | `frob.algebra.orbits.out/1` | `symmetry` | `frob algebra orbits C2xC2 --set feet:4 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column; `paginated = true` where the block says paginated.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "algebra.irreps"
cluster = "algebra"
kind = "read"
tier = "S"
capabilities = ["algebra:read"]
input_schema = "frob.algebra.irreps.in/1"
output_schema = "frob.algebra.irreps.out/1"
refusals = ["FROB-REF-214", "FROB-REF-215"]
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M1"
task_clusters = ["symmetry"]
examples = ["frob algebra irreps C2xC2 --rep permutation:4 --json"]
owner = "frob-algebra"
purpose = "Isotypic decomposition and block types"
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved compute
   envelope scope the job cites; its schema, its place in the scope order and its refusals are NJL-01's);
   `mutate`: `idempotency_key`, `expected_parent`). `idempotency_key` is in no JSON-schema `required` array: the
   annotation `x-frob-key` (`"required"`, or `"optional"` on a `key_optional` entry) carries the rule, and a
   missing key is refused with 805 by the admission transaction on both surfaces. The generator appends the
   surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this
   build) and 802 (schema failure, including an unknown parameter) for every verb; 701 to 704 (budget and
   admission), 761 (above the compute envelope in force), 803 (key reused with different content) and 805
   (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. The other properties and codes are
   added by field, as the variation table of NJL-01 lists them (that table is the one list; its variations 10
   to 16, which later beads land, are read there and not copied here): 806 and 807 for paginated reads; 808 (an
   integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property and for no
   entry by kind (every `run` entry has one in `sync_wait_ms`, every paginated entry in `limit`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790
   for a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a
   path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`).
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- A representation with a constituent of quaternionic type, or a group outside the supported families, is refused
  (`FROB-REF-214`); an exact computation over its declared size is refused (`FROB-REF-215`) on both verbs.
- `algebra.orbits` is paginated (`paginated = true`: `cursor`, `limit`, `follow`, codes 806 and 807), because its
  representatives come in pages.
- `algebra.irreps` is printed `S/F`: a Solid core with one Frontier extension (`--nested`, M4). It is registered
  with `tier = "S"`, because the field takes one of S, F, M; `frob capabilities` lists the extension with its
  pilot N13-25 when the feature `frontier-wreath` is in the build. Until N13-26 lands, `--nested` is an unknown
  parameter (`FROB-REF-802`, exit 3); its arrival is an additive change to the input and output schemas.
- The two examples parse against the implementing beads' forms: `frob algebra irreps C2xC2 --rep permutation:4`
  (four real constituents, `free_parameters` 4; N13-14) and `frob algebra orbits C2xC2 --set feet:4` (27 contact
  strata; BD-049 accepts `--set` beside `--family D6 --slots 6 --types 10`, which N13-17 drives).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly two entries with `cluster = "algebra"` (2 `read`), each with every field
      of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster algebra` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 2, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_algebra.sh` runs each verb of the cluster (two), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/algebra/<verb>.help.json` byte for byte for each of
      the two, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_algebra` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 2 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `algebra.orbits` is `read` at M1), a row printed `S/F` being registered `S` (`algebra.irreps`).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`algebra.irreps`: 214, 215; `algebra.orbits`: 214,
  215); the appended codes equal the rule for its kind and fields (`algebra.orbits` also 806 and 807, and
  808 through its `int53` inputs `limit`, `slots` and `types`).
- `examples_known_answers`: the two registry examples, run through the real binary, return `free_parameters` 4 and
  `orbits` 27.
- `mcp_names_one_to_one`: the tool names are the mapped names (`algebra_irreps`, `algebra_orbits`), with no
  collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_algebra.sh` (header: `bead=VERBS-algebra milestone=M1
labels=tether,verbs,verb:algebra.irreps,verb:algebra.orbits gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd
plants=alias,missing-row,renamed-field`) sources the shared helper and `scripts/e2e/verbs_lib.sh` and calls
`verbs_cluster_conformance algebra`. Steps: (1) `frob capabilities --json` and `frob robot-docs --json` list the two
verbs with kind and milestone; (2) `frob-surface-diff parity --cluster algebra` over stdio and through `frobd
--mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster algebra`; (4) each verb's
registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob job wait`) or
`FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented verb the same
call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`,
`warnings` and `next_actions`; (6) per verb one call with a misspelt parameter (`e2e_expect_refusal FROB-REF-802
3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_algebra/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 2
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 2 verbs (2 `read`); the tables above. KA-755 (the exit-code table on malformed,
  unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's tables;
  exact) is exercised for every verb by steps (6) and (7).
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

- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-137) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §13.5, §13.6, §32.4, §32.6, §32.12.3, §32.18,
§32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
