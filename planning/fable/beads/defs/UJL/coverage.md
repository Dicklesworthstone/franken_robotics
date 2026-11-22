# Coverage of drafting unit UJL: Appendix J (verb registry) and Appendix L (worked artifacts)

Bead files in this directory: 62 (0 seed beads; 44 cluster registry beads `VERBS-<cluster>`; 9 artifact beads `ART-L1` to `ART-L9`; 9 new beads `NJL-01` to `NJL-09`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| Appendix J, preamble | The registry file, what is generated from it, the four kinds | NJL-01, NJL-02 |
| J.1 | One entry | NJL-01 (entry schema, kinds, name map, lints); every `VERBS-<cluster>` bead shows one entry in full |
| J.2 | Every verb of the mechanical inventory, by cluster (163 verbs, 34 clusters) | The 34 beads `VERBS-analyze` … `VERBS-train` (one per cluster); the count facts in NJL-03 |
| J.3 | Verbs the chapters define that the mechanical inventory does not hold (80 verbs) | NJL-03 (the table, the reconciliation, the counts); rows carried by `VERBS-operator`, `-artifact`, `-algebra`, `-actuator`, `-fidelity`, `-contact`, `-fleet`, `-cores`, `-evidence`, `-store` and by the 14 extended cluster beads; `verb_invoke` in NJL-02 and NJL-03 |
| J.4 | One name per verb | NJL-04 (the fifteen mechanisms, the retired-name table, the alias lint) |
| J.5 | What an agent sees: task clusters of at most seven tools | NJL-05 (assignment of 45 task clusters, lints, the 12 kB budget); NJL-06 (the MCP resource `frob://docs/clusters` and its test) |
| Appendix L, preamble | Nine artifacts for one story; specimens; exact arithmetic | NJL-07 (layout, layers, identity map, story-consistency check, blessing rule) |
| L.1 | A `.fbot` design: `quad-tray` | ART-L1 |
| L.2 | An `.fmodule`: the three-joint leg and its port contract | ART-L2 |
| L.3 | An `.ftask`: kitchen tray delivery | ART-L3 |
| L.4 | An `.fmission` query: which stances and knee ratios can carry the tray | ART-L4 |
| L.5 | An `.fenergy` budget: the stop at the pass counter | ART-L5 |
| L.6 | An `.fmcw`: a minimal counterexample world of three edits | ART-L6 |
| L.7 | A `.frobcert` bundle listing | ART-L7 |
| L.8 | A study plan | ART-L8 |
| L.9 | An agent session over MCP | ART-L9 (the scripted session in three hosting lanes), NJL-08 (transcript part A and the session driver), NJL-09 (transcript part B) |

## (b) Gates

Neither appendix defines a gate, a kill gate or an admission gate. Gates of other chapters that beads of this unit measure in part:

| Gate | Defined in | Beads of this unit | What they contribute |
|---|---|---|---|
| GATE-TETHER-02 (M1, re-measured per milestone) | §32.21, §40 | NJL-01, NJL-02, NJL-03, NJL-04, NJL-05 and the 44 `VERBS-<cluster>` beads | The parity harness; the per-cluster rows of the parity record; counts; the alias and task-cluster lints. The gate's status is recorded by the Tether surface seeds (BD-137, BD-222, BD-312, BD-376, BD-409) |
| GATE-TETHER-12 (M3) | §32.21, §40 | NJL-05, NJL-06, ART-L9 | The focus clause (at most seven tools plus the base three), the cluster-directory row of the resource clause, and the wait and cancellation clauses on the tools of the scripted session. The full gate is BD-298's |
| INT-FMCP-01 (M0) | §33.6 | NJL-02 | The measurement that the tools listed are the registry's; the admission itself is BD-023's |

Kill gates named in Considerations as fallbacks, none decided here: KILL-MCP, KILL-SOLVE, KILL-REPLAYPACK, KILL-SKEL-SEXPR, KILL-SKEL-SUBST, KILL-STATIC, KILL-ATLAS, KILL-GAIT, KILL-ENERGY, KILL-PHYLUM, KILL-FAMILY, KILL-REFLECT, KILL-FLEET, KILL-INGEST, KILL-AUGUR-FALSECERT, KILL-MCW, KILL-AUTOPSY. Open questions named with their fail-closed defaults: OPEN-TETHER-2, OPEN-TETHER-3, OPEN-TETHER-5, OPEN-NEMESIS-3, OPEN-CORTEX-1.

## (c) Counts

- Verbs carried: 243 (163 inventory rows, 80 added rows) in 44 registry clusters; 45 task clusters; 15 one-name mechanisms; 11 retired or false-positive names.
- Known answers of Appendix M cited with their values (Appendix M has no section for Appendix J or L): 9 (KA-090, KA-612, KA-693, KA-755, KA-756, KA-757, KA-760, KA-762, KA-763).
- Known answers new to this unit, stated inline and without an Appendix M number (the lead is asked to assign numbers): the registry counts (243; 136, 83, 15, 6, 3; the milestone histogram; the inventory subset) in NJL-03; the task-cluster counts in NJL-05; the canonical-form pointer table and derived facts of L.1 (ART-L1); the contract arithmetic of L.2 (ART-L2); the tick, reach and margin arithmetic and the eleven labelled traces of L.3 (ART-L3); the closed form, pavings, witnesses, dual values and the grid count 515 of 2,511 of L.4 (ART-L4); the integer energy terms and the two boundary pairs of L.5 (ART-L5); the shrink counts and the two delta-debugging cases of L.6 (ART-L6); the eight force vectors, the partition of thirteen cells and the receipt arithmetic of L.7 (ART-L7); the interval [0.0554, 0.4392] at (a, b) = (2, 19) of L.8 (ART-L8); the second-request level and bounds and the budget identities of L.9 (NJL-09).
- Counterexamples of Appendix N cited by number: 1 (CX-348). Counterexamples carried by statement (they belong to other chapters' registries): the mechanical symmetry group as the policy's symmetry (ART-L1); adding peak currents on a shared bus, and the semidefinite limb packet (ART-L2); composing margins without scales (ART-L3); reporting an incumbent as a minimum (ART-L4); a payload-only energy budget and the reflected rotor (ART-L5); a 1-minimal world taken as the smallest failure (ART-L6); a first moment passed as a torque (ART-L7); stopping a fixed-n study at a favourable look (ART-L8); retrying a held-out test until it passes (NJL-09); `robot-docs` against `robot.docs` under the name map (NJL-01).
- Refusal codes: neither appendix defines one. Codes of other chapters named in this unit's beads with their meanings: 244 distinct (lowest FROB-REF-101, highest FROB-REF-899).

## (d) Needs from other slices

- `result.show`, `study.cancel` and `run.export` are named by no seed bead title. Beads drafted by other units while this unit was being written carry them: BD-376 (its drafted title names `result.show`), N32a-13 (`study.cancel`) and N29a-02 (`run.export`). The lead should confirm that those beads keep them.
- The scope registry (BD-258, unit U37): the scope names in the 243 rows are proposals of the form `<cluster>:<read|run|write|admin>` plus the named scopes of the surface.
- The owner decision on `fidelity.rank` (recorded with N16b-27, unit U16b). Every count in this unit uses its fail-closed default.
- Known-answer numbers for the unit's new known answers (list in (c)); the known-answer registry bead of the verification chapter.
- `contract.toml` gaining a `verbs = [...]` list per crate (NJL-03's cross-check); the workspace unit (U34) owns the contract schema.
- The repository layout table gaining `fixtures/golden/` (NJL-07); the workspace unit owns the layout.
- Generation of the CLI and MCP surfaces, `session.focus`, the bounded wait, resources and prompts: the Tether seeds BD-022, BD-023, BD-137, BD-138, BD-298 (unit U32a); this unit's beads depend on them by key.

## (e) Contradictions and things that could not be turned into work as written

1. J.3 says "55 named verbs" and its table prints 56. The extra name is `fidelity.rank`: J.4 registers that record as `eval.rank-agreement`, J.5 speaks of the `fidelity` cluster's "five verbs", and the Kinesis chapter counts six. Done: the fail-closed default of N16b-27 (not registered; 243 verbs); every count test follows the registries, so the owner's decision is a two-line change (NJL-03, NJL-04, VERBS-fidelity).
2. J.5 gives three analysis clusters of at most seven for 26 analyses (21 seats). Done: a fourth cluster, `analyze-evidence` (NJL-05).
3. J.5 describes `operate` as "the job, artifact, device and session verbs": ten verbs once the base tools are set aside. Done: seven in `operate`; `session.open` and `session.close` in `housekeeping`; `session.show` stays in `orient` (NJL-05).
4. The 28 task clusters the appendix names leave 61 verbs without a seat, while the lint requires every verb to sit in at least one. Done: 17 further clusters as a starting assignment (NJL-05).
5. Three inventory rows print two kinds (`nemesis.hardset`, `scene.bank`, `spark.cache`) while an entry has one `kind`. Done: one default kind plus an `actions` list (NJL-01).
6. The entry of J.1 has no field for the one-sentence purpose that the generated tool description needs, for the owner crate, or for the per-surface wait default that the J.1 text mentions. Done: DESIGN additions `purpose`, `use_when`, `owner`, `wait`, `actions` (NJL-01).
7. J.2 says five scanned names were not registered and does not print them. The planning record shows nine retired spellings and two file names. Done: all eleven are in the retired-name table (NJL-04). `policy.rs` is a well-formed verb name, so the name grammar cannot reject it; the contract cross-check does (NJL-03).
8. The verb tables of the Ground and Foundry chapters print refusal numbers that predate the refusal registry (840 to 850; 160 to 165, 260 to 262, 860 to 864) and collide with other chapters' codes. Done: mapped by meaning to the codes of the chapters' own refusal tables (VERBS-ground, VERBS-foundry).
9. Milestones in the inventory disagree with seed beads: `foundry.size` and `foundry.loads` M4 (seeds BD-401, BD-400 at M5); `foundry.lifecycle` M8 (epic BD-423 at M6); `fidelity.shadow` M3 (BD-214 at M2); the `session` cluster M0 (BD-138 at M1); `ground.ingest` M1 (BD-387 at M5); `shield.budget` and `shield.check` M3 (BD-337, BD-338 at M4); `study.plan` M2 (BD-124 at M1); `result.revalidate` M4 in J.2 and M1 for the mission slice in the surface chapter; `analyze.structure` inside "M1 to M4" (its analysis bead at M5). Done: the registry carries the inventory's milestone; a verb answers the not-implemented refusal until its bead lands; each cluster bead notes its case.
10. The surface's exit table names exits for 70x, 72x and 73x only; codes 740, 741, 760, 761, 790 and 791 have no stated exit. Not resolved here: the refusal registries of their chapters must state it. The cluster beads print the rule as the surface states it.
11. Some chapters call seven verbs "the limit" for their registry cluster, and one cluster bead of another unit lints it; the surface applies the limit of seven to task clusters only (registry clusters reach 26). Done: this unit lints task clusters only (NJL-05).
12. L.3: the specimen lists four labelled examples and says the mutation check passes; under the admission rule (a vacuous hard-limit atom is refused) four examples cannot distinguish six hard-limit atoms. Done: eleven synthetic traces in the resolved layer, the four-example case kept as a control (ART-L3).
13. L.5 spells the flags field of `claim` as `flags`; the owner chapter's sketch spells it `authorization`; both hold a prose placeholder. Done: default `authorization` with the three-flag object in the resolved layer (ART-L5).
14. L.2: how a `params` entry names its target is unspecified, and one abbreviation is printed for the module's reference hash and for its contract digest. Done: DESIGN proposals with fail-closed defaults (ART-L2).
15. L.7 lists the qualification receipt (`sample-coverage`) as class Tested; the certificate chapter says "at most Tested"; the mission chapter's table of proof kinds says Estimated. Done: Tested for a referee-tier qualification receipt, Estimated for a mission answer's empirical bound (ART-L7).
16. L.8 cites `FROB-REF-532` for an unreachable verdict; the studio chapter has `FROB-REF-555` for the same condition. Done: the script asserts 532; which layer raises which is left to the owners (ART-L8).
17. L.9: the introduction says focus changes eight times and the transcript has nine `session_focus` calls; the token's six scopes do not cover the tools called under a per-cluster scope rule; `forks_run: 113` matches three admitted interventions while one is refused; the first synthesis shows exit 0 and the refinement exit 2 although both carry a warning; `forced_by` names a slosh state that is a later epic. Done: the step file follows the transcript; scopes are computed from the registry and the difference is logged; the story values are classed `structural` (NJL-08, NJL-09, ART-L9).
18. L.6 says the original world had seven edits and lists three. Done: four decoy edits chosen by this unit, as fixture data (ART-L6).
19. The dynamic content of L.6, L.8 (result) and the second half of L.9 cannot be reproduced value for value by any real run. Done: three fixture layers and four field classes (NJL-07); exact fields are compared exactly, simulated outcomes by invariants; a skip is never a pass.
20. The repository layout has `tests/fixtures/oracle/` and no `fixtures/golden/`; the brief fixes `fixtures/golden/`. Done: NJL-07 adds the root and keeps golden fixtures apart from oracle captures.
21. Appendix M and Appendix N have no rows for Appendix J or L. Done: the unit's known answers are stated inline and listed in (c) for numbering.

## (f) Every verb, the registry bead that carries its row, and the bead expected to implement it

Status is as found on disk when this file was written. `drafted` names the unit whose directory holds the bead file; `seed, not drafted` names the unit that the seed index assigns.

| Verb | Kind | First milestone | Registry bead | Implementing bead(s) | Expected unit |
|---|---|---|---|---|---|
| `analyze.mobility` | read | M1 | VERBS-analyze | BD-101 (drafted), N28a-05 (drafted) | U28a |
| `analyze.mass` | read | M1 | VERBS-analyze | BD-100 (drafted), N28a-05 (drafted) | U28a |
| `analyze.symmetry` | read | M1 | VERBS-analyze | BD-102 (drafted), N28a-05 (drafted) | U20b, U28a |
| `analyze.gait` | read | M4 | VERBS-analyze | BD-332 (drafted), N28a-05 (drafted) | U28b, U28a |
| `analyze.workspace` | read | M1 | VERBS-analyze | BD-105 (drafted), N28a-05 (drafted) | U33a, U28a |
| `analyze.singularities` | read | M1 | VERBS-analyze | BD-105 (drafted), N28a-05 (drafted) | U33a, U28a |
| `analyze.maneuverability` | read | M1 | VERBS-analyze | BD-106 (drafted), N28a-05 (drafted) | U28a |
| `analyze.stability` | read | M1 | VERBS-analyze | BD-100 (drafted), N28a-05 (drafted) | U28a |
| `analyze.curvature` | read | M3 | VERBS-analyze | BD-243 (drafted), N28a-05 (drafted) | U33b, U28a |
| `analyze.closure` | read | M1 | VERBS-analyze | BD-103 (drafted), N28a-05 (drafted) | U15b, U28a |
| `analyze.envelope` | read | M1 | VERBS-analyze | BD-103 (drafted), N28a-05 (drafted) | U15b, U28a |
| `analyze.modes` | read | M1 | VERBS-analyze | BD-107 (drafted), N28a-05 (drafted) | U28a |
| `analyze.self-collision` | read | M1 | VERBS-analyze | BD-104 (drafted), N28a-05 (drafted) | U19a, U28a |
| `analyze.coverage` | read | M3 | VERBS-analyze | BD-243 (drafted), N28a-05 (drafted) | U33b, U28a |
| `analyze.structure` | read | M4 | VERBS-analyze | BD-401 (drafted), N28a-05 (drafted) | U33b, U28a |
| `analyze.sensitivity` | read | M3 | VERBS-analyze | BD-243 (drafted), N28a-05 (drafted) | U33b, U28a |
| `analyze.observability` | read | M1 | VERBS-analyze | BD-120 (drafted), N28a-05 (drafted) | U20a, U28a |
| `analyze.route` | read | M3 | VERBS-analyze | BD-233 (drafted), N28a-05 (drafted) | U19b, U28a |
| `analyze.rank-robustness` | read | M4 | VERBS-analyze | BD-346 (drafted), N28a-05 (drafted) | U28b, U28a |
| `analyze.energy-budget` | read | M1 | VERBS-analyze | BD-108 (drafted), N28a-05 (drafted) | U28a |
| `analyze.deadlines` | read | M1 | VERBS-analyze | BD-108 (drafted), N28a-05 (drafted) | U28a |
| `analyze.loop-stability` | read | M1 | VERBS-analyze | BD-108 (drafted), N28a-05 (drafted) | U28a |
| `analyze.bus-schedule` | read | M1 | VERBS-analyze | BD-108 (drafted), N28a-05 (drafted) | U28a |
| `analyze.wrench` | read | M4 | VERBS-analyze | BD-348 (drafted), N28a-05 (drafted) | U28a |
| `analyze.identifiability` | read | M4 | VERBS-analyze | BD-348 (drafted), N28a-05 (drafted) | U28a |
| `analyze.regimes` | read | M4 | VERBS-analyze | BD-348 (drafted), N28a-05 (drafted) | U28a |
| `autopsy.attribute` | read | M3 | VERBS-autopsy | BD-280 (drafted) | U27b |
| `autopsy.intervene` | run | M3 | VERBS-autopsy | BD-279 (drafted) | U27b |
| `autopsy.repair` | run | M3 | VERBS-autopsy | N27b-14 (drafted), BD-284 (drafted) | U27b |
| `autopsy.rerun` | run | M3 | VERBS-autopsy | BD-283 (drafted) | U27a |
| `autopsy.show` | read | M3 | VERBS-autopsy | BD-284 (drafted) | U27b |
| `control.estimate` | read | M1 | VERBS-control | BD-114 (drafted), N22a-04 (drafted) | U22a |
| `control.ladder` | read | M3 | VERBS-control | BD-330 (drafted), N22a-18 (drafted) | U22a |
| `control.mpc` | run | M3 | VERBS-control | BD-252 (drafted) | U22a |
| `control.symmetry` | read | M4 | VERBS-control | BD-327 (drafted) | U22b |
| `control.synth` | run | M1 | VERBS-control | BD-118 (drafted) | U22c |
| `control.show` | read | M1 | VERBS-control | BD-118 (drafted) | U22c |
| `control.lift` | run | M6 | VERBS-control | N22c-23 (drafted) | U22c |
| `design.bisect` | run | M4 | VERBS-design | BD-343 (drafted) | U28a |
| `design.blame` | read | M1 | VERBS-design | BD-113 (drafted) | U28a |
| `design.check` | read | M1 | VERBS-design | BD-110 (drafted), BD-244 (drafted) | U28a, U14b |
| `design.diff` | read | M1 | VERBS-design | BD-113 (drafted) | U28a |
| `design.edit` | mutate | M1 | VERBS-design | BD-058 (drafted), N28b-01 (drafted) | U41, U28b |
| `design.patch` | mutate | M1 | VERBS-design | BD-059 (drafted) | U14a |
| `design.templates` | read | M1 | VERBS-design | BD-109 (drafted) | U14b |
| `design.new` | mutate | M1 | VERBS-design | N14a-21 (drafted) | U14a |
| `design.show` | read | M1 | VERBS-design | N14a-21 (drafted) | U14a |
| `design.import` | mutate | M1 | VERBS-design | BD-065 (drafted), BD-066 (drafted) | U14b, U14a |
| `design.export` | read | M1 | VERBS-design | N14a-25 (drafted), BD-066 (drafted) | U14a |
| `design.view` | read | M1 | VERBS-design | BD-057 (drafted) | U14a |
| `eval.compare` | run | M3 | VERBS-eval | BD-262 (drafted), BD-124 (drafted) | U28a, U33b |
| `eval.detect` | run | M3 | VERBS-eval | BD-267 (drafted) | U23b |
| `eval.ledger` | read | M3 | VERBS-eval | N24b-15 (drafted), BD-259 (drafted) | U24b |
| `eval.preanalyze` | read | M1 | VERBS-eval | N24b-06 (drafted), BD-124 (drafted) | U24b, U33b |
| `eval.promote` | run | M3 | VERBS-eval | BD-263 (drafted) | U16b |
| `eval.qualify` | run | M3 | VERBS-eval | BD-259 (drafted) | U24b |
| `eval.rank-agreement` | run | M4 | VERBS-eval | BD-328 (drafted), BD-265 (drafted) | U23b, U16b |
| `eval.run` | run | M2 | VERBS-eval | N24c-04 (drafted), BD-125 (drafted) | U24c |
| `eval.scorecard` | read | M3 | VERBS-eval | BD-254 (drafted), N24c-04 (drafted) | U24b, U24c |
| `eval.items` | read | M3 | VERBS-eval | N24c-30 (drafted) | U24c |
| `experiment.suggest` | read | M4 | VERBS-experiment | BD-351 (drafted), N25c-18 (drafted) | U28a, U25c |
| `explain.compare` | read | M3 | VERBS-explain | N28a-17 (drafted), BD-262 (drafted) | U28a |
| `explain.design` | read | M1 | VERBS-explain | BD-112 (drafted) | U15a |
| `explain.run` | read | M1 | VERBS-explain | BD-112 (drafted) | U15a |
| `foundry.check` | run | M5 | VERBS-foundry | BD-405 (drafted) | U31b |
| `foundry.export` | run | M5 | VERBS-foundry | BD-408 (drafted) | U31b |
| `foundry.lifecycle` | run | M8 | VERBS-foundry | BD-423 (drafted), N31a-50 (drafted) | U31b, U31a |
| `foundry.loads` | run | M4 | VERBS-foundry | BD-400 (drafted) | U31b |
| `foundry.optimize` | run | M5 | VERBS-foundry | BD-403 (drafted) | U31b |
| `foundry.size` | read | M4 | VERBS-foundry | BD-401 (drafted) | U33b |
| `foundry.tolerance` | read | M5 | VERBS-foundry | BD-406 (drafted) | U31b |
| `gait.atlas` | read | M4 | VERBS-gait | BD-332 (drafted) | U28b |
| `gait.continue` | run | M4 | VERBS-gait | BD-335 (drafted) | U22a |
| `gait.gradient` | run | M4 | VERBS-gait | BD-334 (drafted) | U22a |
| `gait.orbit` | run | M4 | VERBS-gait | BD-333 (drafted) | U33b |
| `ground.band` | read | M5 | VERBS-ground | BD-393 (drafted) | U30b |
| `ground.excite` | read | M4 | VERBS-ground | BD-390 (drafted) | U30a |
| `ground.identify` | run | M5 | VERBS-ground | BD-391 (drafted) | U30b |
| `ground.ingest` | mutate | M1 | VERBS-ground | BD-387 (drafted) | U25a |
| `ground.kit` | run | M5 | VERBS-ground | BD-396 (drafted) | U30b |
| `ground.predict` | mutate | M5 | VERBS-ground | BD-392 (drafted) | U30b |
| `ground.score` | run | M5 | VERBS-ground | BD-392 (drafted), N30b-06 (drafted) | U30b |
| `job.cancel` | admin | M2 | VERBS-job | BD-210 (drafted), N29a-23 (drafted) | U29a |
| `job.find` | read | M2 | VERBS-job | BD-210 (drafted), N29a-23 (drafted) | U29a |
| `job.list` | read | M2 | VERBS-job | BD-210 (drafted), N29a-23 (drafted) | U29a |
| `job.status` | read | M2 | VERBS-job | BD-210 (drafted), N29a-23 (drafted) | U29a |
| `job.wait` | read | M2 | VERBS-job | BD-210 (drafted), N29a-23 (drafted), N32a-16 (drafted) | U29a, U32a |
| `mission.distinguish` | run | M4 | VERBS-mission | BD-135 (drafted), BD-291 (drafted) | U25a, U25c |
| `mission.refute` | run | M1 | VERBS-mission | BD-135 (drafted), BD-290 (drafted) | U25a, U25b |
| `mission.revalidate` | run | M4 | VERBS-mission | BD-292 (drafted), N25a-16 (drafted) | U25a |
| `mission.separate` | run | M4 | VERBS-mission | BD-293 (drafted) | U25b |
| `mission.show` | read | M1 | VERBS-mission | BD-135 (drafted) | U25a |
| `mission.synthesize` | run | M1 | VERBS-mission | BD-135 (drafted) | U25a |
| `mission.transport` | run | M8 | VERBS-mission | BD-436 (drafted), N25c-02 (drafted) | U25a, U25c |
| `mission.new` | mutate | M1 | VERBS-mission | BD-126 (drafted) | U24b |
| `mission.check` | read | M1 | VERBS-mission | BD-126 (drafted) | U24b |
| `module.check` | read | M1 | VERBS-module | BD-060 (drafted), N15b-02 (drafted) | U15b |
| `module.list` | read | M1 | VERBS-module | BD-060 (drafted), N15b-02 (drafted) | U15b |
| `module.show` | read | M1 | VERBS-module | BD-060 (drafted), N15b-02 (drafted) | U15b |
| `module.substitute` | mutate | M1 | VERBS-module | BD-077 (drafted) | U14a |
| `module.characterize` | run | M3 | VERBS-module | N14b-28 (drafted) | U14b |
| `nemesis.audit` | run | M4 | VERBS-nemesis | BD-345 (drafted), N26b-05 (drafted) | U28b, U26b |
| `nemesis.bound` | run | M3 | VERBS-nemesis | BD-276 (drafted) | U26a |
| `nemesis.cover` | read | M3 | VERBS-nemesis | BD-278 (drafted) | U26b |
| `nemesis.hardset` | read, run | M3 | VERBS-nemesis | BD-275 (drafted) | U15a |
| `nemesis.oracle` | run | M3 | VERBS-nemesis | BD-271 (drafted) | U26b |
| `nemesis.search` | run | M3 | VERBS-nemesis | BD-273 (drafted) | U26a |
| `nemesis.shrink` | run | M3 | VERBS-nemesis | BD-274 (drafted) | U26b |
| `opt.archive` | read | M4 | VERBS-opt | BD-342 (drafted) | U17b |
| `opt.cascade` | run | M4 | VERBS-opt | BD-352 (drafted) | U28a |
| `opt.start` | run | M4 | VERBS-opt | BD-341 (drafted), N28b-09 (drafted) | U41, U28b |
| `opt.status` | read | M4 | VERBS-opt | BD-341 (drafted) | U41 |
| `phylum.generate` | read | M3 | VERBS-phylum | BD-287 (drafted) | U17a |
| `phylum.gradient` | run | M4 | VERBS-phylum | BD-353 (drafted) | U17b |
| `phylum.map` | read | M4 | VERBS-phylum | BD-342 (drafted), N17b-18 (drafted) | U17b |
| `phylum.plan` | read | M2 | VERBS-phylum | BD-171 (drafted) | U21b |
| `phylum.quotient` | read | M3 | VERBS-phylum | BD-285 (drafted) | U17a |
| `phylum.reuse` | read | M3 | VERBS-phylum | BD-286 (drafted), N17b-14 (drafted) | U17b |
| `phylum.sweep` | run | M2 | VERBS-phylum | BD-184 (drafted), N17b-04 (drafted) | U17b |
| `policy.apply` | run | M4 | VERBS-policy | BD-322 (drafted), N23a-25 (drafted) | U23a |
| `predict.register` | mutate | M4 | VERBS-predict | BD-344 (drafted) | U41 |
| `predict.report` | read | M4 | VERBS-predict | BD-344 (drafted) | U41 |
| `prove.bundle` | run | M1 | VERBS-prove | BD-133 (drafted) | U25b |
| `prove.kernel` | read | M0 | VERBS-prove | BD-027 (drafted) | U25b |
| `prove.list` | read | M0 | VERBS-prove | BD-133 (drafted), N12-06 (drafted) | U25b, U12 |
| `prove.reflect` | run | M5 | VERBS-prove | BD-382 (drafted) | U12 |
| `prove.verify` | read | M0 | VERBS-prove | BD-133 (drafted), BD-027 (drafted) | U25b |
| `report.campaign` | read | M4 | VERBS-report | BD-352 (drafted), BD-302 (drafted) | U28a, U25e |
| `report.design` | read | M3 | VERBS-report | BD-302 (drafted) | U25e |
| `report.run` | read | M3 | VERBS-report | BD-302 (drafted) | U25e |
| `report.mission` | read | M3 | VERBS-report | BD-302 (drafted) | U25e |
| `result.revalidate` | run | M4 | VERBS-result | BD-292 (drafted), BD-347 (drafted), N28b-32 (drafted) | U25a, U28a, U28b |
| `result.show` | read | M4 | VERBS-result | BD-376 (drafted) | U32a |
| `run.export` | run | M1 | VERBS-run | N29a-02 (drafted), BD-097 (drafted) | U29a |
| `run.fork` | run | M2 | VERBS-run | BD-205 (drafted) | U27a |
| `run.show` | read | M1 | VERBS-run | BD-097 (drafted) | U29a |
| `run.list` | read | M1 | VERBS-run | BD-097 (drafted) | U29a |
| `scene.accept` | run | M3 | VERBS-scene | BD-228 (drafted) | U19a |
| `scene.bank` | read, admin | M3 | VERBS-scene | BD-236 (drafted) | U19c |
| `scene.compile` | run | M2 | VERBS-scene | BD-163 (drafted) | U19c |
| `scene.fit` | run | M3 | VERBS-scene | N19b-32 (drafted), BD-398 (drafted) | U19b, U33a |
| `scene.gen` | run | M2 | VERBS-scene | BD-165 (drafted) | U18a |
| `scene.hazards` | read | M3 | VERBS-scene | BD-227 (drafted) | U14a |
| `scene.import` | run | M3 | VERBS-scene | BD-398 (drafted) | U33a |
| `scene.list` | read | M2 | VERBS-scene | BD-163 (drafted) | U19c |
| `scene.probe-request` | read | M5 | VERBS-scene | BD-291 (drafted), N19c-05 (drafted) | U25c, U19c |
| `scene.purge` | admin | M3 | VERBS-scene | BD-305 (drafted), N37-19 (drafted) | U37 |
| `scene.retrofit` | run | M3 | VERBS-scene | BD-369 (drafted) | U19b |
| `scene.show` | read | M2 | VERBS-scene | BD-163 (drafted) | U19c |
| `scene.split` | mutate | M3 | VERBS-scene | BD-235 (drafted) | U19c |
| `sense.alias` | run | M3 | VERBS-sense | BD-240 (drafted), BD-134 (drafted) | U20a |
| `sense.calibrate` | run | M3 | VERBS-sense | BD-242 (drafted) | U20b |
| `sense.causal-test` | run | M2 | VERBS-sense | BD-119 (drafted) | U20b |
| `sense.observability` | run | M3 | VERBS-sense | BD-120 (drafted) | U20a |
| `sense.profile` | read | M2 | VERBS-sense | BD-099 (drafted) | U20b |
| `sense.schedule` | read | M2 | VERBS-sense | BD-078 (drafted) | U20a |
| `sense.symmetry` | read | M3 | VERBS-sense | BD-241 (drafted) | U20b |
| `sense.visibility` | run | M3 | VERBS-sense | N20b-33 (drafted) | U20b |
| `session.focus` | mutate | M0 | VERBS-session | BD-138 (drafted) | U32a |
| `session.open` | mutate | M0 | VERBS-session | BD-138 (drafted), N32b-04 (drafted), BD-258 (drafted) | U32a, U32b, U37 |
| `session.show` | read | M0 | VERBS-session | BD-138 (drafted), N32b-04 (drafted) | U32a, U32b |
| `session.close` | mutate | M0 | VERBS-session | BD-138 (drafted) | U32a |
| `shield.budget` | run | M3 | VERBS-shield | BD-337 (drafted) | U22b |
| `shield.check` | run | M3 | VERBS-shield | BD-338 (drafted) | U22b |
| `shield.fit` | run | M4 | VERBS-shield | BD-338 (drafted) | U22b |
| `shield.report` | read | M4 | VERBS-shield | BD-340 (drafted) | U22b |
| `sim.estimate` | read | M2 | VERBS-sim | BD-219 (drafted), N16b-26 (drafted) | U36, U16b |
| `sim.replay` | run | M2 | VERBS-sim | BD-203 (drafted), BD-204 (drafted) | U29a |
| `sim.run` | run | M0 | VERBS-sim | BD-024 (drafted), N16b-25 (drafted) | U29a, U16b |
| `skill.check` | run | M4 | VERBS-skill | BD-336 (drafted) | U22b |
| `skill.compile` | run | M4 | VERBS-skill | BD-336 (drafted) | U22b |
| `skill.show` | read | M4 | VERBS-skill | BD-336 (drafted) | U22b |
| `skill.transport` | run | M8 | VERBS-skill | BD-436 (drafted), N22b-07 (drafted) | U25a, U22b |
| `solve.run` | run | M4 | VERBS-solve | BD-372 (drafted), N32a-32 (drafted) | U32a |
| `spark.cache` | read, admin | M2 | VERBS-spark | BD-173 (drafted) | U21a |
| `spark.check` | run | M0 | VERBS-spark | BD-176 (drafted), N21b-32 (drafted) | U21b |
| `spark.devices` | read | M0 | VERBS-spark | N21c-04 (drafted) | U21c |
| `spark.fingerprint` | run | M0 | VERBS-spark | BD-029 (drafted) | U21a |
| `spark.place` | read | M2 | VERBS-spark | BD-178 (drafted) | U21a |
| `spark.precompile` | run | M2 | VERBS-spark | BD-173 (drafted) | U21a |
| `spark.probe` | run | M0 | VERBS-spark | BD-017 (drafted) | U21b |
| `study.compare` | read | M3 | VERBS-study | BD-262 (drafted), N28a-21 (drafted) | U28a |
| `study.plan` | mutate | M2 | VERBS-study | BD-124 (drafted), N33b-19 (drafted) | U33b |
| `study.status` | read | M2 | VERBS-study | BD-124 (drafted) | U33b |
| `study.submit` | run | M2 | VERBS-study | BD-124 (drafted) | U33b |
| `study.cancel` | admin | M2 | VERBS-study | N32a-13 (drafted) | U32a |
| `task.check` | read | M1 | VERBS-task | BD-122 (drafted), N24c-01 (drafted) | U24c |
| `task.fixture` | run | M1 | VERBS-task | BD-123 (drafted) | U24b |
| `task.list` | read | M1 | VERBS-task | BD-122 (drafted) | U24c |
| `task.new` | mutate | M1 | VERBS-task | BD-122 (drafted) | U24c |
| `task.settle` | run | M1 | VERBS-task | BD-195 (drafted), N24c-03 (drafted) | U24a, U24c |
| `task.show` | read | M1 | VERBS-task | BD-122 (drafted) | U24c |
| `task.verdict` | read | M1 | VERBS-task | BD-122 (drafted), N24c-02 (drafted) | U24c |
| `task.audit` | run | M3 | VERBS-task | N24c-22 (drafted) | U24c |
| `telemetry.divergence` | read | M2 | VERBS-telemetry | BD-206 (drafted) | U29a |
| `telemetry.energy` | read | M1 | VERBS-telemetry | BD-094 (drafted), N24a-37 (drafted) | U24a |
| `telemetry.export-html` | read | M2 | VERBS-telemetry | BD-191 (drafted), N24a-41 (drafted) | U24a |
| `telemetry.failures` | read | M1 | VERBS-telemetry | BD-094 (drafted) | U24a |
| `telemetry.histogram` | read | M2 | VERBS-telemetry | BD-207 (drafted), N29a-15 (drafted) | U29a |
| `telemetry.inspect-pixel` | read | M2 | VERBS-telemetry | BD-191 (drafted), N24a-40 (drafted) | U24a |
| `telemetry.query` | read | M1 | VERBS-telemetry | BD-094 (drafted), N24a-36 (drafted) | U24a |
| `telemetry.stills` | read | M2 | VERBS-telemetry | BD-191 (drafted) | U24a |
| `telemetry.summary` | read | M1 | VERBS-telemetry | BD-094 (drafted), N24a-35 (drafted) | U24a |
| `train.distill` | run | M4 | VERBS-train | BD-323 (drafted) | U23a |
| `train.export` | read | M2 | VERBS-train | BD-198 (drafted), N23a-13 (drafted) | U23a |
| `train.family` | run | M4 | VERBS-train | BD-320 (drafted) | U23b |
| `train.inherit` | run | M4 | VERBS-train | BD-322 (drafted) | U23a |
| `train.start` | run | M2 | VERBS-train | BD-198 (drafted), N23a-14 (drafted) | U23a |
| `train.status` | read | M2 | VERBS-train | BD-198 (drafted) | U23a |
| `train.stop` | admin | M2 | VERBS-train | BD-198 (drafted) | U23a |
| `doctor` | read | M0 | VERBS-operator | BD-022 (drafted) | U32a |
| `capabilities` | read | M0 | VERBS-operator | BD-022 (drafted) | U32a |
| `robot-docs` | read | M0 | VERBS-operator | BD-022 (drafted), BD-313 (drafted), N32a-26 (drafted) | U32a |
| `schema` | read | M0 | VERBS-operator | BD-022 (drafted) | U32a |
| `artifact.read` | read | M1 | VERBS-artifact | BD-097 (drafted) | U29a |
| `artifact.list` | read | M1 | VERBS-artifact | BD-097 (drafted) | U29a |
| `artifact.export` | run | M1 | VERBS-artifact | BD-097 (drafted) | U29a |
| `algebra.irreps` | read | M1 | VERBS-algebra | BD-048 (drafted), N13-14 (drafted) | U13 |
| `algebra.orbits` | read | M1 | VERBS-algebra | BD-049 (drafted) | U22a |
| `actuator.list` | read | M1 | VERBS-actuator | BD-069 (drafted) | U15a |
| `actuator.show` | read | M1 | VERBS-actuator | BD-069 (drafted) | U15a |
| `actuator.check` | read | M1 | VERBS-actuator | BD-069 (drafted) | U15a |
| `actuator.envelope` | read | M1 | VERBS-actuator | BD-073 (drafted) | U15a |
| `actuator.thermal` | read | M1 | VERBS-actuator | BD-072 (drafted) | U15a |
| `actuator.bus` | read | M1 | VERBS-actuator | BD-075 (drafted) | U15b |
| `fidelity.ledger` | read | M2 | VERBS-fidelity | BD-212 (drafted) | U16b |
| `fidelity.energy` | read | M2 | VERBS-fidelity | BD-208 (drafted), N29a-22 (drafted) | U29a |
| `fidelity.coverage` | read | M2 | VERBS-fidelity | BD-213 (drafted) | U16a |
| `fidelity.modelform` | run | M3 | VERBS-fidelity | BD-266 (drafted) | U16b |
| `fidelity.shadow` | admin | M3 | VERBS-fidelity | BD-214 (drafted) | U16b |
| `contact.laws` | read | M1 | VERBS-contact | BD-167 (drafted) | U18b |
| `contact.probe` | read | M1 | VERBS-contact | BD-167 (drafted) | U18b |
| `contact.capacity` | read | M2 | VERBS-contact | BD-168 (drafted) | U18b |
| `contact.audit` | run | M2 | VERBS-contact | BD-168 (drafted) | U18b |
| `contact.clearance` | run | M3 | VERBS-contact | BD-233 (drafted) | U19b |
| `contact.soil` | read | M4 | VERBS-contact | BD-366 (drafted) | U18b |
| `contact.response` | run | M4 | VERBS-contact | BD-367 (drafted) | U18a |
| `fleet.status` | read | M4 | VERBS-fleet | N21c-15 (drafted), BD-370 (drafted) | U21c |
| `fleet.plan` | read | M4 | VERBS-fleet | BD-370 (drafted) | U21c |
| `cores.show` | read | M3 | VERBS-cores | BD-309 (drafted) | U28c |
| `evidence.trace` | read | M1 | VERBS-evidence | BD-095 (drafted) | U29a |
| `store.status` | read | M2 | VERBS-store | BD-209 (drafted) | U29a |
| `store.collect` | admin | M2 | VERBS-store | BD-209 (drafted) | U29a |

## (g) Verbs for which no implementing bead exists yet

No bead at all (no seed title names the verb and no drafted bead carries it): none.

Three verbs have no seed bead of their own and are carried only by beads that other units added: `result.show` (BD-376, by its drafted title), `study.cancel` (N32a-13) and `run.export` (N29a-02).

A seed bead is named but its file was not drafted when this file was written (0 verbs):

| Verb | Seed bead(s) | Unit |
|---|---|---|
