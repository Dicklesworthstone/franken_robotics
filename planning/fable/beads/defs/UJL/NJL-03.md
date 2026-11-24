---
key: "NJL-03"
title: "frob-verbs registry: the 80 verbs the mechanical inventory missed; the 243-row reconciliation, count checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "frob-verbs", "frob-conformance", "solid", "registry", "verb", "test", "gate"]
parent: "WP-06"
deps: ["BD-022", "N34-08", "NJL-01", "NJL-02", "NJL-04", "NJLx-02", "STD-E2E"]
needs: []
related: ["NJL-05", "STD-TEST", "VERBS-actuator", "VERBS-algebra", "VERBS-analyze", "VERBS-artifact", "VERBS-autopsy", "VERBS-contact", "VERBS-control", "VERBS-cores", "VERBS-design", "VERBS-eval", "VERBS-evidence", "VERBS-experiment", "VERBS-explain", "VERBS-fidelity", "VERBS-fleet", "VERBS-foundry", "VERBS-gait", "VERBS-ground", "VERBS-job", "VERBS-mission", "VERBS-module", "VERBS-nemesis", "VERBS-operator", "VERBS-opt", "VERBS-phylum", "VERBS-policy", "VERBS-predict", "VERBS-prove", "VERBS-report", "VERBS-result", "VERBS-run", "VERBS-scene", "VERBS-sense", "VERBS-session", "VERBS-shield", "VERBS-sim", "VERBS-skill", "VERBS-solve", "VERBS-spark", "VERBS-store", "VERBS-study", "VERBS-task", "VERBS-telemetry", "VERBS-train"]
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
are registered, that the registry holds exactly the 243 verbs the specification defines, plus the rows that a
`VERBS-<cluster>` bead registers by a stated DESIGN decision, each recorded as an addition with its owning bead
(one today, `artifact.import` of VERBS-artifact: 244), and no others, and that a verb a crate defines cannot be
missing from the registry again.

Why this matters (DESIGN): a verb that a chapter defines and the registry lacks does not exist on any surface;
no test of the mechanism notices, because the mechanism's own tests call the handler directly. The rule that a
capability is implemented only when it is reachable through `frob --json` makes the registry the place where
that omission must be caught. Rejected: trusting the scan (it already missed a third of the surface once);
a hand-kept list in documentation (it would be the second copy of a fact).

## Scope

In: the table of added verbs; the count known answers of the inventory subset and of the whole registry; the
baseline list `fixtures/golden/verbs/baseline-243.txt` and the staged check of the registry against it; the
record of rows registered by DESIGN; the per-cluster reconciliation table; the contract cross-check that replaces
the scan; the status of the MCP-only
compatibility tool `verb_invoke`; the handling of the one row whose registration is an open owner decision.
Out: the full rows (each cluster's `VERBS-<cluster>` bead carries them); the entry schema (NJL-01); the parity
harness (NJL-02); the alias table (NJL-04); and what the per-milestone surface beads do with the rows once
their handlers exist (BD-137 for the M1 set, then BD-222, BD-312, BD-376 and BD-409): the schema files'
content, the handler bindings, the golden envelopes, the examples made to run, the capability status of each
verb, and the record of GATE-TETHER-02 for each milestone's set. Those beads enter no row: BD-137 depends on
this bead, reads its count rows and its inventory record into the gate record, and writes that record. This
bead and the cluster beads are the only places a row is entered.

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
default, used by every count below: `fidelity.rank` is **not** registered; the specification's list holds 243
verbs. If the owner chooses to register it, it is one more addition in the sense defined below: every total
rises by one (245 with `artifact.import`), the `fidelity` cluster has six and `run` verbs one more; the tests
read the decision from the registries themselves (the name is either a row of the retired-name table of bead
NJL-04 or a row of the registry with its `[[addition]]` record, never both), so the change is the row, its
record and one retired-name line, and no test is edited by hand.

**Count known answers.**

| Set | Verbs | Clusters | read | run | mutate | admin | two kinds | By first milestone |
|---|---|---|---|---|---|---|---|---|
| Inventory subset (FACT about the inventory table) | 163 | 34 | 73 | 73 | 11 | 3 | 3 | M0 11, M1 33, M2 31, M3 39, M4 35, M5 11, M8 3 |
| Added by this bead | 80 | 10 new, 14 extended | 63 | 10 | 4 | 3 | 0 | M0 5, M1 39, M2 10, M3 14, M4 11, M6 1 |
| Whole registry as the specification lists it (the baseline) | 243 | 44 | 136 | 83 | 15 | 6 | 3 | M0 16, M1 72, M2 41, M3 53, M4 46, M5 11, M6 1, M8 3 |
| Registered by DESIGN in a cluster bead (`artifact.import`, VERBS-artifact) | 1 | `artifact` extended | 0 | 1 | 0 | 0 | 0 | M2 1 |
| Complete registry (baseline plus additions) | 244 | 44 | 136 | 84 | 15 | 6 | 3 | M0 16, M1 72, M2 42, M3 53, M4 46, M5 11, M6 1, M8 3 |

The three verbs with two kinds are `nemesis.hardset` (read, run), `scene.bank` (read, admin) and `spark.cache`
(read, admin): the rows the specification prints with two kinds, each `read` by default. The kind columns count
kinds as the specification prints them; a cluster bead that has since declared an `actions` list on a further
row (NJL-01 names `session.show`, `sense.symmetry`, `design.diff`, `ground.band` and `nemesis.audit`) leaves
that row's default kind, and so these columns, unchanged. The ten new clusters are `operator`, `artifact`,
`algebra`, `actuator`, `fidelity`, `contact`, `fleet`, `cores`, `evidence` and `store`. Milestones of the analysis family used in the second row: M1 for A-01,
A-02, A-05 to A-08, A-10 to A-13, A-17, A-20 to A-23; M3 for A-09, A-14, A-16, A-18; M4 for A-04, A-15, A-19,
A-24 to A-26.

**Rows registered by DESIGN (additions).** A row beyond the specification's 243 is entered only by a
`VERBS-<cluster>` bead that states the DESIGN decision with its reason, and it is recorded in the registry file
as an `[[addition]]` record (`name`, `added_by`, `defined_in`; a top-level table that N32a-02's strict reader
admits from M0), written in the same change as the row. There is one today: `artifact.import` (kind `run`,
first milestone M2, cluster `artifact`, owner `frob-chronicle`; `added_by = "VERBS-artifact"`, `defined_in =
"§21.17"`; implemented by N21c-08). It admits a cross-machine transfer bundle on the receiving machine: the
specification gives the transfer rule and no command that admits a bundle, and its count of 243 is the sum of
its own tables, not a rule that forbids a row. With it the `artifact` cluster holds four rows, `run` verbs are
84 and 42 verbs have first milestone M2. If the owner keeps the specification's count, the row and its record
are removed in one change and every total falls back to the baseline. The three late analysis verbs of NJLx-51
(M3) are additions of the same kind and take the complete registry to 247.

**Staging and the baseline list (DESIGN; the rule NJL-05 states for task clusters and BD-137 for parity).**
Rows enter the registry with their cluster's `VERBS-<cluster>` bead, from M0 to M4, so the registry is complete
only when the last cluster first registered at M4 has landed, while this bead closes at M1. Its counts are
therefore asserted on a committed list, and the registry is checked against that list by a staged rule:
1. This bead commits `fixtures/golden/verbs/baseline-243.txt`: one line per verb of the specification, sorted by
   name, with the fields `name cluster kind first_milestone source`, where `kind` is the default kind, followed
   by `+` and the second kind for the three rows printed with two (`read+run`, `read+admin`), and `source` is
   `inventory` or `added`. It has 243 lines; `fidelity.rank` is absent under the default decision. The first
   three count rows and the per-cluster table are computed from this file. It is also the baseline of NJLx-02's
   rows JL-01 to JL-04 and of NJLx-51's lint `unrecorded-addition`.
2. At every milestone: every registered row is a line of the list with the same cluster, default kind and first
   milestone (else `baseline-mismatch`, naming the field), or has an `[[addition]]` record (else
   `not-in-baseline`); no `[[addition]]` record names a baseline name or a name without a row.
3. A cluster's rows are complete when its cluster bead closes (that bead's `rows_present` test). The statement
   about the whole registry (its names equal the list plus the additions: 244 rows in 44 clusters, with the
   kinds and the histogram of the last count row) is evaluated by the same tests from the first build in which
   every one of the 44 clusters has a row; before that it is reported `not-due` with the clusters still
   unregistered, never as a pass.

**Reconciliation by cluster** (each cluster's rows are carried by the bead `VERBS-<cluster>`).

| Cluster | Inventory rows | Added rows | Total | First milestone | Owner crates | Registry bead |
|---|---|---|---|---|---|---|
| `analyze` | 1 | 25 | 26 | M1 | `frob-analyze` | VERBS-analyze |
| `autopsy` | 5 | 0 | 5 | M3 | `frob-autopsy` | VERBS-autopsy |
| `control` | 4 | 3 | 7 | M1 | `frob-control` | VERBS-control |
| `design` | 7 | 5 | 12 | M1 | `frob-analyze`, `frob-design`, `frob-io`, `frob-opt` | VERBS-design |
| `eval` | 9 | 1 | 10 | M1 | `frob-arena`, `frob-learn`, `frob-opt` | VERBS-eval |
| `experiment` | 1 | 0 | 1 | M4 | `frob-mission` | VERBS-experiment |
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

The table reconciles the specification's rows. With the addition `artifact.import` the `artifact` cluster holds
four rows and the complete registry 244.

**The cross-check that replaces the scan (DESIGN).** Each crate's `contract.toml` (the machine-readable part
of its `CONTRACT.md`) gains a list `verbs = ["design.edit", …]` of the verbs whose handlers it owns. A test in
`frob-conformance`, `verbs_contract_reconciliation`, asserts both directions: every verb in any crate's list is
a registry row whose `owner` is that crate; every registry row is in exactly one crate's list. A verb defined
by a crate and absent from the registry (the defect this bead exists for) fails with `defined-not-registered`;
a registry row no crate claims (the `policy.rs` kind of mistake, which the name grammar cannot catch, because
`policy.rs` is a well-formed name) fails with `registered-not-defined`. The tool subcommand is
`frob-surface-diff inventory`, which prints the record `frob.verbs.inventory/1`: `{registry, in_contracts,
defined_not_registered[], registered_not_defined[], owner_mismatch[], reserved_names[], not_in_baseline[],
baseline_mismatch[], additions[], unregistered_clusters[], fidelity_rank: "retired" | "registered"}`
(`registry` is the number of rows the file holds at that build; `additions` lists the `[[addition]]` records;
`unregistered_clusters` is empty from the build in which the registry is complete). `reserved_names` lists every row whose MCP tool name equals a name the MCP surface
reserves for itself (today one: `verb_invoke`).

**`verb_invoke`.** The MCP surface offers one compatibility tool, `verb_invoke {verb, arguments}`, validated
against the named verb's schema, for clients that never re-list tools. It exists on MCP only and is not a
registry verb: the registry must hold no row for it, the name map must not produce `verb_invoke` from any row
(a row `verb.invoke` would; `frob-surface-diff inventory` fails it as `reserved-name`, and the record lists it
under `reserved_names[]`), and the parity harness lists it as a declared difference, not an exemption (NJL-02's
closed list).
DESIGN, one rule shared with BD-137: the surface gate is measured with the compatibility flag off, and there
the harness's exemption list has exactly one entry, the unknown-verb case (the one difference between the
surfaces that the surface invariant permits). `verb_invoke` is a second entry of the harness's closed list
that applies only to the run with the flag on, which this bead makes: the served list must then be the
registry's tools plus `verb_invoke` and nothing else. Reason: a gate record that counted two exemptions would
depend on a launch flag.

## Acceptance criteria

- [ ] `fixtures/golden/verbs/baseline-243.txt` holds 243 lines in the stated form; every verb of the
      added-verbs table and every one of the 26 analysis verbs is a line of it with the kind and first milestone
      of its cluster bead; `fidelity.rank` is absent under the default decision.
- [ ] The first three count rows hold exactly on the list: 243 verbs in 44 clusters with 136 `read`, 83 `run`,
      15 `mutate`, 6 `admin` and 3 two-kind rows, and the milestone histogram of the third row; the inventory
      subset of 163 lines reproduces the first count row; the per-cluster table holds for all 44 clusters.
- [ ] Staged check, at M1 and at every later milestone: every registered row is a line of the list with the
      same cluster, default kind and first milestone, or has an `[[addition]]` record; `artifact.import` is
      registered with its record (`added_by = "VERBS-artifact"`) and the `artifact` cluster has four rows.
- [ ] Complete registry: from the first build in which all 44 clusters have a row, the registry's names equal
      the list plus the additions and the last count row holds on the registry itself (244 verbs, 136 `read`,
      84 `run`, 15 `mutate`, 6 `admin`, 3 printed with two kinds, M2 42); before that build the check reports
      `not-due` naming the unregistered clusters, never a pass.
- [ ] `frob-surface-diff inventory` exits 0: both reconciliation lists, `not_in_baseline` and
      `baseline_mismatch` are empty and no owner mismatches exist.
- [ ] Negative controls, each caught and named: a removed added row (`defined-not-registered`); a planted row
      `policy.rs` (`registered-not-defined`, and `not-in-baseline`); a planted row `fidelity.rank` while the
      name is still in the retired-name table (`retired-name`); a planted row `verb.invoke` (`reserved-name`);
      `artifact.import` with its `[[addition]]` record removed (`not-in-baseline`); a baseline row entered with
      another kind (`baseline-mismatch`).
- [ ] `frob robot-docs --json` and `frob capabilities --json` each list exactly the rows the registry holds at
      that build (244 once it is complete); the MCP `tools/list` with every cluster enabled lists the same
      tools, plus `verb_invoke` when the compatibility flag is on.
- [ ] GATE-TETHER-02 (TARGET; claim Known answer; M1): the parity record over the verbs due at M1 is empty apart
      from the closed exemptions; the measured counts are logged against the count rows of this bead (the
      registered rows against the list restricted to the registered clusters plus the additions). This bead
      measures the count and inventory part of the gate and stores the record `frob.verbs.inventory/1`; the
      gate's row for the M1 set is written once, by BD-137, which cites that record.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test registry_counts` and `cargo test -p frob-conformance --test
verbs_contract_reconciliation` (the real registry and the real contracts; planted registries are TOML strings
labelled `synthetic`):
- `baseline_total_243` (on the list): 243 lines, 44 distinct clusters, sorted, no duplicate name.
- `baseline_kinds` (on the list): 136, 83, 15, 6 with one kind and three lines with two (`nemesis.hardset`,
  `scene.bank`, `spark.cache`, each `read` first).
- `baseline_milestones` (on the list): M0 16, M1 72, M2 41, M3 53, M4 46, M5 11, M6 1, M8 3; no line at M7.
- `inventory_subset_counts`: the 163 lines with source `inventory` give 73, 73, 11, 3 and 3, in 34 clusters,
  with M0 11, M1 33, M2 31, M3 39, M4 35, M5 11, M8 3.
- `added_rows_present`: each of the 80 added names is a line with source `added`; the operator verbs have
  cluster `operator` and no dot.
- `analyze_family_26`: 26 lines in cluster `analyze`, all `read`, names as listed; A-19 and A-26 are not in the
  `check` set.
- `cluster_table`: the per-cluster counts of the list equal the table of this bead.
- `registry_is_list_restricted` (on the real registry, at every milestone): every row is a line with the same
  cluster, default kind and first milestone, or has an `[[addition]]` record; the three lines with two kinds
  that are registered carry an `actions` list; a `synthetic` registry with a row absent from both fails
  `not-in-baseline`, and one with `design.check` entered as `run` fails `baseline-mismatch` naming `kind`.
- `additions_recorded`: the records are exactly `artifact.import` (`added_by = "VERBS-artifact"`), plus
  `fidelity.rank` when it is a registry row and no longer a retired name, plus NJLx-51's three once they are
  registered; a record whose name is a baseline name, or has no row, fails.
- `registry_complete_or_not_due`: with a row in each of the 44 clusters, the registry's names equal the list
  plus the additions and the counts are 244, 136, 84, 15, 6 and 3 printed with two kinds, with M2 42 (one more
  of each total per further addition); on a `synthetic` registry lacking the `fleet` rows the result is
  `not-due` naming `fleet`, and the summary counts it apart from the passes.
- `verb_invoke_not_a_row`: no row maps to the tool name `verb_invoke`; a `synthetic` registry with a row
  `verb.invoke` fails `inventory` with `reserved-name`.
- `reconciliation_both_directions`, `reconciliation_owner_matches`: on the real tree; then one planted defect
  each for `defined-not-registered`, `registered-not-defined` and `owner-mismatch`.

### End-to-end script and logging

`scripts/e2e/verbs_inventory.sh` (header: `bead=NJL-03 milestone=M1 labels=tether,verbs,registry
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp plants=drop-row,policy-rs,fidelity-rank,unrecorded-row`). Steps: (1)
`frob capabilities --json`, count verbs and clusters and assert that the names are the list restricted to the
registered clusters plus the registered additions (244 and 44 once the registry is complete; before that the
expected count is computed from the list and logged with the unregistered clusters); (2) `frob robot-docs
--json`, assert the same set of names; (3) MCP `tools/list` with every cluster enabled, assert the same tools
under the inverse name map; (4) for each of the ten new clusters that is registered, `frob robot-docs --cluster
<task cluster> --json` lists its verbs; (5) `frob-surface-diff inventory`, assert the reconciliation lists,
`not_in_baseline` and `baseline_mismatch` empty and `additions` equal to the expected records; (6) one call of an operator verb (`frob doctor
--json`, exit 0 or 2, one envelope) and one of an added verb not yet implemented (`e2e_expect_refusal
FROB-REF-724 5`); (7) the plants, each through `e2e_expect_caught` with the named error. The log
(`target/e2e-logs/verbs_inventory/<utc>.ndjson`, written through the shared helper) records every command with
exit code and wall time, the metrics `baseline_lines` (== 243), `registry_verbs` (== the expected count of
step (1); 244 when complete), `registry_clusters` (== the registered clusters; 44 when complete), `additions`
(== 1 at M1), `defined_not_registered` (== 0), `registered_not_defined` (== 0), `not_in_baseline` (== 0) tagged
with the gate, each control with
`caught: true|false`, the first differing name on failure, and the final `summary`.

### Known answers and negative controls

- The count rows and the per-cluster table are the known answers of this bead (exact; tier G0 on the host).
  Their numbered rows are JL-01 (243 baseline verbs in 44 clusters, and the committed registry as baseline plus
  additions), JL-02 (kinds), JL-03 (milestones) and JL-04 (163 + 80) of the Appendix J and L block that NJLx-02
  registers (a dependency); this bead's tests cite those numbers once the block is issued.
- KA-755 (the exit-code table of the surface) is exercised by step (6).
- Negative controls: the six planted registries of the acceptance criteria.
- Metamorphic relation: the sum of the per-cluster counts equals the total, and the sum of the per-kind counts
  plus the two-kind rows equals the same total, on the list (243) and on the complete registry (244).
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
  the not-implemented refusal until its bead lands, so no count depends on the disagreement. The rows that
  answer later than their field are NJL-02's closed list `LATE_ROWS`, the one place where the capability-level
  coverage check accepts `FROB-REF-724` past a row's `first_milestone`; a late row missing from that list fails
  the check by name.
- DESIGN, the same rule in the other direction (shared with BD-137): where a seed bead builds a reduced form
  before the appendix's milestone (at M1: the CPU forms of `study.plan`, `study.submit`, `study.status` and
  `eval.compare`, the CPU sensing verbs `sense.profile`, `sense.schedule`, `sense.causal-test`,
  `sense.observability` and `sense.alias`, and `mission.distinguish`), the row keeps the appendix's
  `first_milestone` and the verb is implemented early: it answers, and `frob capabilities` lists it as
  implemented with its restriction named. Reason: the milestone histogram above is a known answer about the
  appendix; if the field followed delivery, ten rows would move and the counts would describe the order of
  work. Whether a verb answers is decided by its handler being in the build, never by comparing the build's
  milestone with the field.
- No verb is registered at M7 (the tournament milestone adds no surface), which the milestone test pins.
- At M3 NJLx-51 registers three late analysis verbs as further additions: the complete registry then holds 247
  (139 `read`, 84 `run`, M3 55, M4 47, 84 outside the inventory, `analyze` 29), the totals of the rows the file
  holds move into the registry's `[counts]` table, and the tests here go on comparing the registry with the
  list plus the `[[addition]]` records; the baseline rows of this bead are not rewritten.
- The whole-registry statement becomes evaluable only when the last cluster bead of M4 lands, after this bead
  has closed. It is a test of the ordinary test run, so it turns from `not-due` to a pass or a failure in that
  change without anyone re-opening this bead; the registry-completeness test of the last verb-set bead (BD-409,
  M5) asserts the same reference, the list plus the additions, against the complete registry.

## References

Appendix J (J.2 counts, J.3), §32.6, §32.12.4, §32.17, §28.4, §16.13, §34.5; GATE-TETHER-02; INV-TETHER-01;
KA-755.
