---
key: "NJL-03"
title: "frob-verbs registry: the 80 verbs the mechanical inventory missed, and the 243-row reconciliation with its count checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "frob-verbs", "frob-conformance", "solid", "registry", "verb", "test", "gate"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["N16b-27", "N28a-05", "BD-137", "BD-138", "NJL-04", "NJL-05", "STD-TEST"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.2 counts, J.3)", "§32.6", "§32.12.4", "§32.17", "§28.4", "§16.13", "§34.5"]
---
## Background

Every capability of FrankenRobotics (a robotics simulation and design system) is a verb in one registry,
`registries/verbs.toml` (crate `frob-verbs`), from which the command line `frob`, the MCP server `frob-mcp`, the
schemas and the agent guide are generated. The registry was first filled from a **mechanical inventory**: a
scan of the specification for dotted names of the form `word.word`. That scan found 169 names. One, `policy.rs`,
is a source file name in a sibling library and not a verb. Five more were second names for a mechanism that an
owner chapter registers under another name, or had no owner, and are not registered (the one-name rule, bead
NJL-04). The other 163 are verbs, in 34 clusters.

Because the scan matched one spelling, it missed three groups of verbs that the subsystem chapters define:
(1) the four operator verbs, which have no dot (`doctor`, `capabilities`, `robot-docs`, `schema`); (2) verbs that
chapters introduce in a cluster's verb list or table without writing the dotted form in running text
(`session.close`, `job.status`, `design.new`, the `actuator`, `contact` and `fidelity` clusters, and others);
(3) the family `analyze.<name>`, one verb per static analysis A-01 to A-26, of which the scan saw only
`analyze.symmetry`. All of them are registry entries. This bead owns the union: it makes sure the 80 added rows
are registered, that the registry holds exactly the 243 verbs the specification defines and no others, and that
a verb a crate defines cannot be missing from the registry again.

Why this matters (DESIGN): a verb that a chapter defines and the registry lacks does not exist on any surface;
no test of the mechanism notices, because the mechanism's own tests call the handler directly. The rule that a
capability is implemented only when it is reachable through `frob --json` makes the registry the place where
that omission must be caught. Rejected: trusting the scan (it already missed a third of the surface once);
a hand-kept list in documentation (it would be the second copy of a fact).

## Scope

In: the table of added verbs; the count known answers of the inventory subset and of the whole registry; the
per-cluster reconciliation table; the contract cross-check that replaces the scan; the status of the MCP-only
compatibility tool `verb_invoke`; the handling of the one row whose registration is an open owner decision.
Out: the full rows (each cluster's `VERBS-<cluster>` bead carries them); the entry schema (NJL-01); the parity
harness (NJL-02); the alias table (NJL-04).

## Technical approach

**The added verbs, as the specification groups them.** 56 names are printed and the family adds 25 beyond
`analyze.symmetry`.

| Verb | Kind | First milestone | Purpose | Defined in |
|---|---|---|---|---|
| `doctor`, `capabilities`, `robot-docs`, `schema` | read | M0 | Operator verbs: toolchains, pins and devices; what is implemented; the agent guide; a schema with a minimal example | §32.6, §32.17 |
| `session.close` | mutate | M0 | Close a session and release its reservations | §32.6 |
| `job.status`, `job.wait` | read | M2 | Job state; a bounded wait on a job | §32.10, §32.12.5 |
| `artifact.read`, `artifact.list`, `artifact.export` | read, read, run | M1 | Read by digest and byte range; list; export under the session's export root | §32.6, §29 |
| `design.new`, `design.show`, `design.import`, `design.export`, `design.view` | mutate, read, mutate, read, read | M1 | Create from a template or a mission cell; show; import and export with a loss report; convert between the JSON and S-expression views | §14, §32.6 |
| `analyze.<name>` | read | M1 to M4 | One verb per analysis A-01 to A-26 (`analyze.symmetry` is A-03); A-19 and A-26 read runs and never join `check` | §28.4 |
| `algebra.irreps`, `algebra.orbits` | read | M1 | Isotypic decomposition and block types; orbit counts and one canonical representative per orbit | §13.5, §13.6 |
| `actuator.list`, `actuator.show`, `actuator.check`, `actuator.envelope`, `actuator.thermal`, `actuator.bus` | read | M1 | Cards and their provenance; admission checks; the torque–speed envelope; the thermal steady state and runaway; the shared bus ceiling | §15 |
| `control.synth`, `control.show` | run, read | M1 | Synthesize the baseline controller for a morphology class, with three outcomes; show it | §22.4 |
| `fidelity.ledger`, `fidelity.energy`, `fidelity.coverage` | read | M2 | Ledger rows and frozen envelopes; created-energy totals; both coverage figures | §16.13 |
| `fidelity.rank` | run | M3 | The rank-agreement record for a design set and a tier pair, with the permitted use per stratum (between controller rungs the record is `eval.rank-agreement`) | §16.9 |
| `fidelity.modelform` | run | M3 | `model-robust` or `model-conditional` for a ranking (the rank-agreement record for a tier pair is `eval.rank-agreement`) | §16.9, §16.13 |
| `fidelity.shadow` | admin | M3 | Start, stop and configure the background referee for a job | §16.13 |
| `contact.laws`, `contact.probe` | read | M1 | The admitted laws per tier and profile; a law, cone or film evaluated at a contact state | §18.13 |
| `contact.capacity`, `contact.audit` | read, run | M2 | Static contact maxima against capacities; the tunnelling audit | §18.13 |
| `contact.clearance` | run | M3 | `clear` with a certificate, `penetrates` with a witness, or `unknown` | §18.13 |
| `contact.soil`, `contact.response` | read, run | M4 | Sinkage and thrust on a soil card; the response batch [F] | §18.13 |
| `fleet.status`, `fleet.plan` | read | M4 | Roles and queues of the two machines; the assignment by role affinity and estimated work | §21.17 |
| `run.list` | read | M1 | Runs in the caller's namespace | §32.6 |
| `study.cancel` | admin | M2 | Cancel a submitted study | §32.6 |
| `mission.new`, `mission.check` | mutate, read | M1 | Create a query from a document or a template; admit it, refuse an empty or inconsistent domain, echo the quantifier order | §32.6 |
| `result.show` | read | M4 | A stored result with its claim product and dependencies | §32.6 |
| `report.run`, `report.mission` | read | M3 | The evidence package of a run; of a mission answer | §32.6 |
| `cores.show` | read | M3 | A decision core's table, loss model, obligations and fallback | §28.19 |
| `evidence.trace` | read | M1 | The evidence closure of a claim identifier | §29.13 |
| `store.status`, `store.collect` | read, admin | M2 | Store size, pins and retention; a collection cycle with tombstones | §29.16 |
| `module.characterize` | run | M3 | Characterize one module's capability table for the composed quasi-static screen | §14.16 |
| `sense.visibility` | run | M3 | Visibility of each sensor along a nominal motion, with the robot's own limbs and payload as occluders | §20.24 |
| `control.lift` | run | M6 | Fit and calibrate a lifted linear planning model for one soft-soil regime | §22.20 |
| `task.audit` | run | M3 | Attack a task before any candidate is scored: foil bodies and intent invariants | §24.20 |
| `eval.items` | read | M3 | The item bank of a battery and the sampling weights a close comparison used | §24.21 |

**The 26 analysis verbs** (the names are DESIGN defaults, except `analyze.symmetry`; they are the spellings the
generator of the analysis registry uses, bead N28a-05): A-01 `analyze.mobility`, A-02 `analyze.mass`, A-03
`analyze.symmetry`, A-04 `analyze.gait`, A-05 `analyze.workspace`, A-06 `analyze.singularities`, A-07
`analyze.maneuverability`, A-08 `analyze.stability`, A-09 `analyze.curvature`, A-10 `analyze.closure`, A-11
`analyze.envelope`, A-12 `analyze.modes`, A-13 `analyze.self-collision`, A-14 `analyze.coverage`, A-15
`analyze.structure`, A-16 `analyze.sensitivity`, A-17 `analyze.observability`, A-18 `analyze.route`, A-19
`analyze.rank-robustness`, A-20 `analyze.energy-budget`, A-21 `analyze.deadlines`, A-22
`analyze.loop-stability`, A-23 `analyze.bus-schedule`, A-24 `analyze.wrench`, A-25 `analyze.identifiability`,
A-26 `analyze.regimes`. A-19 and A-26 read recorded runs and never join `design.check`.

**The count, and the one open row.** The specification states: "55 named verbs and the `analyze` family of 26,
of which one is in the inventory; the registry therefore holds 243 verbs (163 + 55 + 25)". Its table prints 56
names. The difference is `fidelity.rank` (the rank-agreement record for a design set and a tier pair): the
one-name table of the same appendix registers that record, "for rungs and for tier pairs", as
`eval.rank-agreement`, and the task-cluster assignment speaks of the `fidelity` cluster's "five verbs". The
Kinesis chapter's own verb table counts six. This is an owner decision, recorded with bead N16b-27. Fail-closed
default, used by every count below: `fidelity.rank` is **not** registered; the registry holds 243 verbs. If the
owner chooses to register it, the expected total becomes 244, the `fidelity` cluster six, and `run` verbs 84;
the tests read the decision from the registries themselves (the name is either a row of the retired-name
table of bead NJL-04 or a row of the registry, never both), so the change is two lines and no test is edited
by hand.

**Count known answers.**

| Set | Verbs | Clusters | read | run | mutate | admin | two kinds | By first milestone |
|---|---|---|---|---|---|---|---|---|
| Inventory subset (FACT about the inventory table) | 163 | 34 | 73 | 73 | 11 | 3 | 3 | M0 11, M1 33, M2 31, M3 39, M4 35, M5 11, M8 3 |
| Added by this bead | 80 | 10 new, 14 extended | 63 | 10 | 4 | 3 | 0 | M0 5, M1 39, M2 10, M3 14, M4 11, M6 1 |
| Whole registry | 243 | 44 | 136 | 83 | 15 | 6 | 3 | M0 16, M1 72, M2 41, M3 53, M4 46, M5 11, M6 1, M8 3 |

The three verbs with two kinds are `nemesis.hardset` (read, run), `scene.bank` (read, admin) and `spark.cache`
(read, admin). The ten new clusters are `operator`, `artifact`, `algebra`, `actuator`, `fidelity`, `contact`,
`fleet`, `cores`, `evidence` and `store`. Milestones of the analysis family used in the second row: M1 for A-01,
A-02, A-05 to A-08, A-10 to A-13, A-17, A-20 to A-23; M3 for A-09, A-14, A-16, A-18; M4 for A-04, A-15, A-19,
A-24 to A-26.

**Reconciliation by cluster** (each cluster's rows are carried by the bead `VERBS-<cluster>`).

| Cluster | Inventory rows | Added rows | Total | First milestone | Owner crates | Registry bead |
|---|---|---|---|---|---|---|
| `analyze` | 1 | 25 | 26 | M1 | `frob-analyze` | VERBS-analyze |
| `autopsy` | 5 | 0 | 5 | M3 | `frob-autopsy` | VERBS-autopsy |
| `control` | 4 | 3 | 7 | M1 | `frob-control` | VERBS-control |
| `design` | 7 | 5 | 12 | M1 | `frob-analyze`, `frob-design`, `frob-io`, `frob-opt` | VERBS-design |
| `eval` | 9 | 1 | 10 | M1 | `frob-arena`, `frob-dyn`, `frob-learn`, `frob-opt` | VERBS-eval |
| `experiment` | 1 | 0 | 1 | M4 | `frob-opt` | VERBS-experiment |
| `explain` | 3 | 0 | 3 | M1 | `frob-chronicle`, `frob-opt` | VERBS-explain |
| `foundry` | 7 | 0 | 7 | M4 | `frob-foundry`, `frob-lifecycle` | VERBS-foundry |
| `gait` | 4 | 0 | 4 | M4 | `frob-gait` | VERBS-gait |
| `ground` | 7 | 0 | 7 | M1 | `frob-ground` | VERBS-ground |
| `job` | 3 | 2 | 5 | M2 | `frob-verbs` | VERBS-job |
| `mission` | 7 | 2 | 9 | M1 | `frob-mission`, `frob-spec` | VERBS-mission |
| `module` | 4 | 1 | 5 | M1 | `frob-analyze`, `frob-design` | VERBS-module |
| `nemesis` | 7 | 0 | 7 | M3 | `frob-nemesis` | VERBS-nemesis |
| `opt` | 4 | 0 | 4 | M4 | `frob-opt` | VERBS-opt |
| `phylum` | 7 | 0 | 7 | M2 | `frob-phylum` | VERBS-phylum |
| `policy` | 1 | 0 | 1 | M4 | `frob-learn` | VERBS-policy |
| `predict` | 2 | 0 | 2 | M4 | `frob-opt` | VERBS-predict |
| `prove` | 5 | 0 | 5 | M0 | `frob-cert`, `frob-lemma` | VERBS-prove |
| `report` | 2 | 2 | 4 | M3 | `frob-opt` | VERBS-report |
| `result` | 1 | 1 | 2 | M4 | `frob-verbs` | VERBS-result |
| `run` | 3 | 1 | 4 | M1 | `frob-chronicle` | VERBS-run |
| `scene` | 13 | 0 | 13 | M2 | `frob-scene` | VERBS-scene |
| `sense` | 7 | 1 | 8 | M2 | `frob-sense` | VERBS-sense |
| `session` | 3 | 1 | 4 | M0 | `frob-verbs` | VERBS-session |
| `shield` | 4 | 0 | 4 | M3 | `frob-shield` | VERBS-shield |
| `sim` | 3 | 0 | 3 | M0 | `frob-chronicle`, `frob-dyn` | VERBS-sim |
| `skill` | 4 | 0 | 4 | M4 | `frob-control` | VERBS-skill |
| `solve` | 1 | 0 | 1 | M4 | `frob-verbs` | VERBS-solve |
| `spark` | 7 | 0 | 7 | M0 | `frob-cli`, `frob-kernels` | VERBS-spark |
| `study` | 4 | 1 | 5 | M2 | `frob-arena`, `frob-opt` | VERBS-study |
| `task` | 7 | 1 | 8 | M1 | `frob-arena`, `frob-spec` | VERBS-task |
| `telemetry` | 9 | 0 | 9 | M1 | `frob-chronicle` | VERBS-telemetry |
| `train` | 7 | 0 | 7 | M2 | `frob-learn` | VERBS-train |
| `operator` | 0 | 4 | 4 | M0 | `frob-verbs` | VERBS-operator |
| `artifact` | 0 | 3 | 3 | M1 | `frob-chronicle` | VERBS-artifact |
| `algebra` | 0 | 2 | 2 | M1 | `frob-algebra` | VERBS-algebra |
| `actuator` | 0 | 6 | 6 | M1 | `frob-actuate` | VERBS-actuator |
| `fidelity` | 0 | 5 | 5 | M2 | `frob-dyn` | VERBS-fidelity |
| `contact` | 0 | 7 | 7 | M1 | `frob-contact` | VERBS-contact |
| `fleet` | 0 | 2 | 2 | M4 | `frob-cli` | VERBS-fleet |
| `cores` | 0 | 1 | 1 | M3 | `frob-opt` | VERBS-cores |
| `evidence` | 0 | 1 | 1 | M1 | `frob-chronicle` | VERBS-evidence |
| `store` | 0 | 2 | 2 | M2 | `frob-chronicle` | VERBS-store |
| **Total** | **163** | **80** | **243** | | | 44 beads |

**The cross-check that replaces the scan (DESIGN).** Each crate's `contract.toml` (the machine-readable part
of its `CONTRACT.md`) gains a list `verbs = ["design.edit", …]` of the verbs whose handlers it owns. A test in
`frob-conformance`, `verbs_contract_reconciliation`, asserts both directions: every verb in any crate's list is
a registry row whose `owner` is that crate; every registry row is in exactly one crate's list. A verb defined
by a crate and absent from the registry (the defect this bead exists for) fails with `defined-not-registered`;
a registry row no crate claims (the `policy.rs` kind of mistake, which the name grammar cannot catch, because
`policy.rs` is a well-formed name) fails with `registered-not-defined`. The tool subcommand is
`frob-surface-diff inventory`, which prints the record `frob.verbs.inventory/1`: `{registry, in_contracts,
defined_not_registered[], registered_not_defined[], owner_mismatch[], fidelity_rank: "retired" | "registered"}`.

**`verb_invoke`.** The MCP surface offers one compatibility tool, `verb_invoke {verb, arguments}`, validated
against the named verb's schema, for clients that never re-list tools. It exists on MCP only and is not a
registry verb: the registry must hold no row for it, the name map must not produce `verb_invoke` from any row,
and the parity harness lists it as an exemption.

## Acceptance criteria

- [ ] Every verb of the added-verbs table and every one of the 26 analysis verbs is a registry row with the
      kind and first milestone of its cluster bead; `fidelity.rank` is absent under the default decision.
- [ ] The three count rows above hold exactly: the registry test prints 243 verbs in 44 clusters with 136
      `read`, 83 `run`, 15 `mutate`, 6 `admin` and 3 two-kind rows, and the milestone histogram of the third row.
- [ ] The inventory subset of 163 rows reproduces the first count row.
- [ ] The per-cluster table holds for all 44 clusters.
- [ ] `frob-surface-diff inventory` exits 0: both reconciliation lists are empty and no owner mismatches exist.
- [ ] Negative controls, each caught and named: a removed added row (`defined-not-registered`); a planted row
      `policy.rs` (`registered-not-defined`); a planted row `fidelity.rank` while the name is still in
      the retired-name table (`retired-name`); a planted row `verb.invoke` (`reserved-name`).
- [ ] `frob robot-docs --json` and `frob capabilities --json` each list 243 verbs; the MCP `tools/list` with
      every cluster enabled lists 243 tools plus `verb_invoke` when the compatibility flag is on.
- [ ] GATE-TETHER-02 (TARGET; claim Known answer; M1): the parity record over the verbs due at M1 is empty apart
      from the closed exemptions; the measured counts are logged against the count rows of this bead.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test registry_counts` and `cargo test -p frob-conformance --test
verbs_contract_reconciliation` (the real registry and the real contracts; planted registries are TOML strings
labelled `synthetic`):
- `registry_total_243`: 243 rows, 44 distinct clusters (244 and six `fidelity` rows when `fidelity.rank` is a
  registry row and no longer a retired name).
- `registry_kinds`: 136, 83, 15, 6 by default kind, and exactly three rows with an `actions` list.
- `registry_milestones`: M0 16, M1 72, M2 41, M3 53, M4 46, M5 11, M6 1, M8 3; no row at M7.
- `inventory_subset_counts`: the 163 named inventory rows give 73, 73, 11, 3 and 3, in 34 clusters, with M0 11,
  M1 33, M2 31, M3 39, M4 35, M5 11, M8 3.
- `added_rows_present`: each of the 80 added names resolves; the operator verbs have `cluster = "operator"` and
  no dot.
- `analyze_family_26`: 26 rows in cluster `analyze`, all `read`, names as listed; A-19 and A-26 are not in the
  `check` set.
- `cluster_table`: the per-cluster counts equal the table of this bead.
- `verb_invoke_not_a_row`: no row maps to the tool name `verb_invoke`.
- `reconciliation_both_directions`, `reconciliation_owner_matches`: on the real tree; then one planted defect
  each for `defined-not-registered`, `registered-not-defined` and `owner-mismatch`.

### End-to-end script and logging

`scripts/e2e/verbs_inventory.sh` (header: `bead=NJL-03 milestone=M1 labels=tether,verbs,registry
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp plants=drop-row,policy-rs,fidelity-rank`). Steps: (1) `frob
capabilities --json`, count verbs and clusters and assert 243 and 44; (2) `frob robot-docs --json`, assert the
same set of names; (3) MCP `tools/list` with every cluster enabled, assert 243 tools under the inverse name map;
(4) for each of the ten new clusters, `frob robot-docs --cluster <task cluster> --json` lists its verbs;
(5) `frob-surface-diff inventory`, assert the three lists empty; (6) one call of an operator verb (`frob doctor
--json`, exit 0 or 2, one envelope) and one of an added verb not yet implemented (`e2e_expect_refusal
FROB-REF-724 5`); (7) the plants, each through `e2e_expect_caught` with the named error. The log
(`target/e2e-logs/verbs_inventory/<utc>.ndjson`, written through the shared helper) records every command with
exit code and wall time, the metrics `registry_verbs` (== 243), `registry_clusters` (== 44),
`defined_not_registered` (== 0), `registered_not_defined` (== 0) tagged with the gate, each control with
`caught: true|false`, the first differing name on failure, and the final `summary`.

### Known answers and negative controls

- The three count rows and the per-cluster table are the known answers of this bead (exact; tier G0 on the host).
  They have no number in the known-answer suite; the unit's coverage file asks the lead to assign one.
- KA-755 (the exit-code table of the surface) is exercised by step (6).
- Negative controls: the four planted registries of the acceptance criteria.
- Metamorphic relation: the sum of the per-cluster counts equals the registry total, and the sum of the
  per-kind counts plus the two-kind rows equals the same total.
- Determinism lanes: the counts and the inventory record are identical twice on one machine, in debug and
  release, on Linux and macOS (class D0).

## Considerations

- The decision on `fidelity.rank` blocks nothing: the executor is the same under either name (the bead for rank
  agreement on the 40-design set), and the default costs one row to reverse.
- The contract list adds a field to `contract.toml`; if the workspace owner prefers the list to be generated
  from handler registrations, the test reads the generated list instead and the two failure names stay.
- Milestone disagreements between the inventory table and the seed beads are listed in the cluster beads
  (`foundry.size` and `foundry.loads` at M4 against seed beads at M5; `foundry.lifecycle` at M8 against an M6
  epic; `fidelity.shadow` at M3 against an M2 seed bead; the `session` cluster at M0 against an M1 seed bead;
  `ground.ingest` at M1 against an M5 seed bead). The registry carries the inventory's milestone; a verb answers
  the not-implemented refusal until its bead lands, so no test depends on the disagreement.
- No verb is registered at M7 (the tournament milestone adds no surface), which the milestone test pins.

## References

Appendix J (J.2 counts, J.3), §32.6, §32.12.4, §32.17, §28.4, §16.13, §34.5; GATE-TETHER-02; INV-TETHER-01;
KA-755.
