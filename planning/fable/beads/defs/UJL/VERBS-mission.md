---
key: "VERBS-mission"
title: "frob-verbs registry: cluster `mission`, nine rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "augur", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-135", "BD-291", "BD-290", "BD-292", "N25a-16", "BD-293", "BD-436", "N25c-02", "BD-126", "N24b-26", "BD-137", "BD-376", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§25.3", "§25.3.1", "§25.3.2", "§25.8", "§25.9", "§25.11", "§25.14", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `mission`. The cluster covers mission queries and inverse
design: a quantified statement about a design family, a world family, an observation channel and a policy class,
answered with inner and outer design sets, witnesses, exclusions, unresolved cells and a certificate bundle. It has
nine verbs (2 `read`, 6 `run`, 1 `mutate`); the handlers live in `frob-mission`, `frob-spec`. The bead carries every
row completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match the
list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the nine registry entries of cluster `mission`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `mission.distinguish` | run | M4 | F | The smallest admissible intervention or instrument that separates alternatives | §25.14 |
| `mission.refute` | run | M1 | F | A scoped impossibility bound, or a witness defeating one design and controller; the smallest relaxing change | §25.3.1, §25.9 |
| `mission.revalidate` | run | M4 | F | Surviving certificates, invalidated regions, reruns and their cost after an edit (one engine with `result.revalidate`) | §25.8 |
| `mission.separate` | run | M4 | F | A minimized, admissible pair of worlds in which a design ordering or a diagnosis changes | §25.9 |
| `mission.show` | read | M1 | S | A stored answer by set (`inner`, `outer`, `excluded`, `unresolved`), ordered and paged | §32.6 |
| `mission.synthesize` | run | M1 | F | Inner and outer design sets with witnesses, exclusions, unresolved cells and cost bounds | §25.3 |
| `mission.transport` | run | M8 | F/M | Move a certified claim through a map between task interfaces with a bounded error, or return a counterexample naming the lost mode [F]; static core at M4. A skill artifact with a learned residual is `skill.transport` | §25.3.1, §25.11 |
| `mission.new` | mutate | M1 | S | Create a mission query from a document or a template | §25.3.2, §32.6 |
| `mission.check` | read | M1 | S | Admit a mission query: refuse an empty or inconsistent domain and echo the quantifier order | §25.3.2, §32.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.

**`mission.distinguish`** (MCP tool `mission_distinguish`; owner `frob-mission`)
- Inputs: mission; alternatives (designs, diagnoses or laws).
- Outputs: the smallest admissible intervention or instrument whose readings separate the alternatives, or two
  witness mechanisms with opposing margins. Evidence: *certificate*.
- Chapter refusals: `FROB-REF-847` (an observation has no arrival instant, or a policy uses information before it
  arrives); `FROB-REF-859` (an experiment or world edit cannot be realized through the declared interfaces, or does
  not declare the causal path it changes).
- Implemented by: BD-135, BD-291.

**`mission.refute`** (MCP tool `mission_refute`; owner `frob-mission`)
- Inputs: mission; a cell, or one design and controller.
- Outputs: a scoped impossibility bound with its proof, or a witness that defeats the design and controller; the
  binding constraint; the smallest relaxing change. Evidence: *certificate*.
- Chapter refusals: `FROB-REF-853` (a minimum is claimed without a matching lower bound, or an unattained infimum is
  reported as attained); `FROB-REF-868` (the `.fmission` file is not valid: unknown field, missing unit, misspelt
  parameter, number not an exact string); `FROB-REF-874` (the world family is empty, or its scope changed after
  candidates were tested); `FROB-REF-881` (evidence or a certificate establishes a different quantifier prefix than
  the query's, including clairvoyant evidence offered for a robust claim).
- Implemented by: BD-135, BD-290.

**`mission.revalidate`** (MCP tool `mission_revalidate`; owner `frob-mission`)
- Inputs: mission answer; the edit (mission, design family, card or world).
- Outputs: surviving certificates, invalidated regions, required reruns and their cost (one engine with
  `result.revalidate`). Evidence: *certificate*.
- Chapter refusals: `FROB-REF-835` (a verdict is reused after a change without a recorded implication (D-11));
  `FROB-REF-854` (a classifier, screen or certificate is reused outside its eligibility: support, deadline, error
  units or tolerance changed).
- Implemented by: BD-292, N25a-16.

**`mission.separate`** (MCP tool `mission_separate`; owner `frob-mission`)
- Inputs: two designs or two diagnoses; world family.
- Outputs: a minimized, admissible, executable pair of worlds in which the ordering or diagnosis changes, with the
  edit that removes the effect. Evidence: *run*.
- Chapter refusals: `FROB-REF-360` (a world or edit lies outside the legality profile; the violated bound is named);
  `FROB-REF-362` (an edit does not declare the causal paths it changes); `FROB-REF-859` (an experiment or world edit
  cannot be realized through the declared interfaces, or does not declare the causal path it changes).
- Implemented by: BD-293.

**`mission.show`** (MCP tool `mission_show`; owner `frob-mission`)
- Inputs: mission; answer; `--set` (`inner`, `outer`, `excluded`, `unresolved`); order; limit; cursor.
- Outputs: the cells of the set, ordered and paged, with measure and proofs. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-135.

**`mission.synthesize`** (MCP tool `mission_synthesize`; owner `frob-mission`)
- Inputs: mission; `--strategy`; budget; `--estimate`.
- Outputs: inner cells with witnesses, excluded cells with proofs, unresolved cells, cost bounds, the next test, the
  certificate bundle root. Evidence: *certificate*.
- Chapter refusals: `FROB-REF-846` (a chance constraint lacks its population, event or confidence, or marginal stage
  rates are multiplied without a dependence assumption); `FROB-REF-847` (an observation has no arrival instant, or a
  policy uses information before it arrives); `FROB-REF-853` (a minimum is claimed without a matching lower bound,
  or an unattained infimum is reported as attained); `FROB-REF-855` (evidence profiles are mixed inside one cell, or
  a probability mass is requested without a supplied weighting); `FROB-REF-856` (a region certificate is requested
  over more parameters than the admitted limit, or a joint paving is requested); `FROB-REF-857` (a synthesis system
  is outside the symbolic caps or is not zero-dimensional); `FROB-REF-868` (the `.fmission` file is not valid:
  unknown field, missing unit, misspelt parameter, number not an exact string); `FROB-REF-874` (the world family is
  empty, or its scope changed after candidates were tested); `FROB-REF-877` (the predicates conflict with the world
  family for every design); `FROB-REF-884` (no quantifier prefix, or a hidden parameter with no persistence
  declaration); `FROB-REF-887` (a policy is indexed by a parameter the controller cannot observe).
- Implemented by: BD-135.

**`mission.transport`** (MCP tool `mission_transport`; owner `frob-mission`)
- Inputs: a certified claim; a map between task interfaces.
- Outputs: the transported claim with a bounded error, or a counterexample naming the contact mode where authority
  is lost; static core at M4. Evidence: *certificate*.
- Chapter refusals: `FROB-REF-444` (a transported skill does not refine its specification); `FROB-REF-852` (a
  monotone or symmetry transport lacks a side condition); `FROB-REF-858` (stage composition fails: an exit set not
  inside the next entry set, a clock mismatch, or a resource spent twice).
- Implemented by: BD-436, N25c-02.

**`mission.new`** (MCP tool `mission_new`; owner `frob-spec`)
- Inputs: a `.fmission` document or a template with parameters.
- Outputs: `MissionId` and the identities every mutable name was bound to. Evidence: *static*.
- Chapter refusals: `FROB-REF-868` (the `.fmission` file is not valid: unknown field, missing unit, misspelt
  parameter, number not an exact string).
- Implemented by: BD-126.

**`mission.check`** (MCP tool `mission_check`; owner `frob-spec`)
- Inputs: mission.
- Outputs: domain `nonempty` or the refusal; topologies; free parameters; the quantifier order echoed; the fragment
  (whether an exact answer is available); horizon. Evidence: *static*.
- Chapter refusals: `FROB-REF-868` (the `.fmission` file is not valid: unknown field, missing unit, misspelt
  parameter, number not an exact string); `FROB-REF-874` (the world family is empty, or its scope changed after
  candidates were tested); `FROB-REF-877` (the predicates conflict with the world family for every design);
  `FROB-REF-884` (no quantifier prefix, or a hidden parameter with no persistence declaration).
- Implemented by: BD-126.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `mission.distinguish` | `mission:run` | `frob.mission.distinguish.out/1` | `mission` | `frob mission distinguish q:5b1e --between c:0204,c:0007 --key dis-1 --json` |
| `mission.refute` | `mission:run` | `frob.mission.refute.out/1` | `mission` | `frob mission refute q:3c9d --design v=9,r=5/2 --json` |
| `mission.revalidate` | `mission:run` | `frob.mission.revalidate.out/1` | `transfer` | `frob mission revalidate q:5b1e --since a:0e51 --key mrv-1 --json` |
| `mission.separate` | `mission:run` | `frob.mission.separate.out/1` | `attack` | `frob mission separate q:5b1e --designs d:a3f1,d:b77e --key sep-1 --json` |
| `mission.show` | `mission:read` | `frob.mission.show.out/1` | `mission`, `transfer` | `frob mission show q:3c9d --answer a:0e51 --set outer --json` |
| `mission.synthesize` | `mission:run` | `frob.mission.synthesize.out/1` | `mission` | `frob mission synthesize q:3c9d --budget cpu_core_s=60 --key syn-1 --sync-wait-ms 2000 --json` |
| `mission.transport` | `mission:run` | `frob.mission.transport.out/1` | `transfer` | `frob mission transport a:8b27 --to carrier-b --key tr-1 --json` |
| `mission.new` | `mission:write` | `frob.mission.new.out/1` | `mission` | `frob mission new tray-stance.fmission --key mis-1 --json` |
| `mission.check` | `mission:read` | `frob.mission.check.out/1` | `mission` | `frob mission check q:3c9d --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "mission.distinguish"
cluster = "mission"
kind = "run"
tier = "F"
capabilities = ["mission:run"]
input_schema = "frob.mission.distinguish.in/1"
output_schema = "frob.mission.distinguish.out/1"
refusals = ["FROB-REF-847", "FROB-REF-859"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["mission"]
examples = ["frob mission distinguish q:5b1e --between c:0204,c:0007 --key dis-1 --json"]
owner = "frob-mission"
purpose = "The smallest admissible intervention or instrument that separates alternatives"
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

- Nine verbs: the six of the mission chapter (`synthesize`, `refute`, `distinguish`, `separate`, `transport`,
  `revalidate`), `show` for stored answers, and `new` and `check` for admission.
- A point query is `mission.synthesize` with every design parameter bound; an existence query returns `true` with a
  witness, `false` with an exclusion of the whole domain, or `unknown` with the reason. Binding the body is a
  special case, not a different verb.
- The inner set holds only cells with a constructive witness; the outer set is the domain minus the excluded cells
  and is not listed as cells of its own. A minimum is stated only with a matching lower bound (`FROB-REF-853`).
- Two verbs transport and their scopes differ: `mission.transport` moves a certified claim through an interface map
  with a bounded error (Frontier; static core at M4; beyond it Moonshot, M8); `skill.transport` moves a skill
  artifact with a learned residual (Moonshot).
- `mission.revalidate` and `result.revalidate` are two verbs over one engine.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly nine entries with `cluster = "mission"` (2 `read`, 6 `run`, 1 `mutate`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster mission` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 9, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_mission.sh` runs each verb of the cluster (nine), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/mission/<verb>.help.json` byte for byte for each of
      the nine, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_mission` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 9 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `mission.check` is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`mission.distinguish`: 847, 859; `mission.refute`: 853,
  868, 874, 881; `mission.revalidate`: 835, 854); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`mission_distinguish`, `mission_refute`,
  `mission_revalidate`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; a verb
  whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_mission.sh` (header: `bead=VERBS-mission milestone=M1 labels=tether,verbs,verb:mission.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance mission`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the nine verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
mission` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster mission`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_mission/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 9
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 9 verbs (2 `read`, 6 `run`, 1 `mutate`); the tables above. KA-755 (the exit-code
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

- Every `run` verb accepts `--estimate`, which returns the cost of admission, lowering, proposing and checking
  separately and runs nothing; checker time is a budget line of its own.
- Kill gate KILL-AUGUR-FALSECERT (a false certificate is accepted): the false-certificate procedure disables the
  proof kind, withdraws bundles and downgrades answers; the rows stay registered.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M4, M8. The
  per-milestone surface beads (BD-137, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is
  re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §25.3, §25.3.1, §25.3.2,
§25.8, §25.9, §25.11, §25.14, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
