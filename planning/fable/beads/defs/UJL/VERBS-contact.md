---
key: "VERBS-contact"
title: "frob-verbs registry: cluster `contact`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "traction", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-167", "BD-168", "BD-233", "BD-366", "BD-367", "N18b-30", "BD-137", "BD-222", "BD-312", "BD-376", "NJL-03", "NJL-04", "NJL-05"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§18.13", "§18.14", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 243 verbs
in 44 clusters (namespaces).

This bead is the registry of record for the cluster `contact`. The cluster covers contact laws and audits: the
admitted laws per tier and profile, a law, cone or film evaluated at a contact state, static contact maxima against
capacities, the tunnelling audit, clearance with a certificate, soil sinkage and thrust, and the contact response
batch. It has seven verbs (4 `read`, 3 `run`); the handlers live in `frob-contact`. The bead carries every row
completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match the list;
every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the seven registry entries of cluster `contact`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `contact.laws` | read | M1 | S | The admitted contact laws per tier and profile | §18.13 |
| `contact.probe` | read | M1 | S | A law, cone or film evaluated at a contact state | §18.13 |
| `contact.capacity` | read | M2 | S | Static contact maxima against capacities | §18.13 |
| `contact.audit` | run | M2 | S | The tunnelling audit | §18.13 |
| `contact.clearance` | run | M3 | S | `clear` with a certificate, `penetrates` with a witness, or `unknown` | §18.13 |
| `contact.soil` | read | M4 | S | Sinkage and thrust on a soil card | §18.13 |
| `contact.response` | run | M4 | F | The contact response batch | §18.13 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *known answer*: a Known answer where a registered row is recovered within its tolerance; class Tested.
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.

**`contact.laws`** (MCP tool `contact_laws`; owner `frob-contact`)
- Inputs: optional tier and profile.
- Outputs: the admitted laws per tier and profile, their parameters by class, the cards in use. Evidence:
  *pass-through*.
- Chapter refusals: `FROB-REF-246` (law not admitted: A law requested on a tier or profile where its gate has not
  passed).
- Implemented by: BD-167.

**`contact.probe`** (MCP tool `contact_probe`; owner `frob-contact`)
- Inputs: law, cone or film; a contact state.
- Outputs: impulse, regime, μ_eff, Hersey number, validity flags. Evidence: *static*.
- Chapter refusals: `FROB-REF-240` (invalid cone: μ < 0, r_eff < 0 or a non-positive anisotropy); `FROB-REF-242`
  (film law out of domain: Π or h/σ outside the card's calibrated range under `--strict-data`); `FROB-REF-243` (soil
  or tyre law out of range: Pressure, slip or sinkage outside the card's validity range).
- Implemented by: BD-167.

**`contact.capacity`** (MCP tool `contact_capacity`; owner `frob-contact`)
- Inputs: design.
- Outputs: static contact maxima against lockstep capacities and the referee's cap, with the patch reductions that
  apply. Evidence: *static*.
- Chapter refusals: none of its own.
- Implemented by: BD-168.

**`contact.audit`** (MCP tool `contact_audit`; owner `frob-contact`)
- Inputs: run or scene class; sampled windows.
- Outputs: missed contacts per 10⁶ world-steps, by scene class, from certified collision detection. Evidence: *known
  answer*.
- Chapter refusals: none of its own.
- Implemented by: BD-168.

**`contact.clearance`** (MCP tool `contact_clearance`; owner `frob-contact`)
- Inputs: body sweep; obstacles.
- Outputs: `clear` with a certificate, `penetrates` with a witness, or `unknown` with what to refine. Evidence:
  *certificate*.
- Chapter refusals: `FROB-REF-320` (thin feature: Scene geometry thinner than two fine voxels with no thin route);
  `FROB-REF-321` (no enclosure: A clearance certificate requested with no registered error bounds for a body or
  scene).
- Implemented by: BD-233.

**`contact.soil`** (MCP tool `contact_soil`; owner `frob-contact`)
- Inputs: wheel or track; soil card; load.
- Outputs: sinkage and thrust with validity. Evidence: *static*.
- Chapter refusals: `FROB-REF-243` (soil or tyre law out of range: Pressure, slip or sinkage outside the card's
  validity range).
- Implemented by: BD-366.

**`contact.response`** (MCP tool `contact_response`; owner `frob-contact`)
- Inputs: a fixed active set; columns.
- Outputs: the response batch with per-column error bounds and mode-guard reroutes. Evidence: *screening*.
- Chapter refusals: none of its own.
- Implemented by: BD-367.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `contact.laws` | `contact:read` | `frob.contact.laws.out/1` | `contact` | `frob contact laws --tier referee --json` |
| `contact.probe` | `contact:read` | `frob.contact.probe.out/1` | `contact` | `frob contact probe --law foot-patch --interface rubber-60a/tile --normal 80N --slip 0.02m/s --json` |
| `contact.capacity` | `contact:read` | `frob.contact.capacity.out/1` | `contact` | `frob contact capacity quad-a --json` |
| `contact.audit` | `contact:run` | `frob.contact.audit.out/1` | `contact` | `frob contact audit r:19e2 --key ca-1 --json` |
| `contact.clearance` | `contact:run` | `frob.contact.clearance.out/1` | `contact`, `places-check` | `frob contact clearance --design quad-a --scene s:kitchen-17 --key cc-1 --json` |
| `contact.soil` | `contact:read` | `frob.contact.soil.out/1` | `contact` | `frob contact soil --wheel w-75 --soil sand-dry --load 40N --json` |
| `contact.response` | `contact:run` | `frob.contact.response.out/1` | `contact` | `frob contact response r:19e2 --world 311 --key cr-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "contact.audit"
cluster = "contact"
kind = "run"
tier = "S"
capabilities = ["contact:run"]
input_schema = "frob.contact.audit.in/1"
output_schema = "frob.contact.audit.out/1"
refusals = []
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M2"
task_clusters = ["contact"]
examples = ["frob contact audit r:19e2 --key ca-1 --json"]
owner = "frob-contact"
purpose = "The tunnelling audit"
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

- Seven verbs. A law requested on a tier or profile where its gate has not passed is refused (`FROB-REF-246`).
- `contact.clearance` returns `clear` only with a certificate; without registered error bounds it refuses a
  certificate and offers `unknown` (`FROB-REF-321`).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "contact"` (4 `read`, 3 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster contact` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_contact.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/contact/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind, and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_contact` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `contact.response` is `run` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`contact.laws`: 246; `contact.probe`: 240, 242, 243;
  `contact.capacity`: none); the appended codes equal the rule for its kind.
- `mcp_names_one_to_one`: the tool names are the mapped names (`contact_laws`, `contact_probe`, `contact_capacity`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the five common properties and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_contact.sh` (header: `bead=VERBS-contact milestone=M1 labels=tether,verbs,verb:contact.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance contact`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
contact` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster contact`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_contact/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (4 `read`, 3 `run`); the tables above. KA-755 (the exit-code table on
  malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's
  tables; exact) is exercised for every verb by steps (6) and (7).
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
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M2, M3, M4. The
  per-milestone surface beads (BD-137, BD-222, BD-312, BD-376) re-run this script as verbs land; that is how
  GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §18.13, §18.14, §32.4, §32.6, §32.12.3, §32.18,
§32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
