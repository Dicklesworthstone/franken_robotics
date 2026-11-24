# Polishing round 4, slice F-U27a

9 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| BD-205 | M2 | feature | WP-51 | yes | verb run.fork: forks from snapshots by indexed copy, classified into the five classes before any restore | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/BD-205.md` |
| BD-281 | M3 | feature | WP-56 | yes | frob-autopsy: coalition experiments on a fork budget: the value table, pair interactions and the Shapley trigger | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/BD-281.md` |
| BD-283 | M3 | feature | WP-56 | yes | verb autopsy.rerun: design-parameter and structural edits re-run from the episode start on paired addresses | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/BD-283.md` |
| BD-375 | M4 | task | WP-56 | yes | GATE-AUTOPSY-08: agents with Autopsy against logs and explain alone on the planted-defect designs, run and recorded | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/BD-375.md` |
| N27a-02 | M2 | task | WP-51 | yes | frob-chronicle: the fork classifier: five classes, the earliest valid fork tick and the forcing fields | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-02.md` |
| N27a-05 | M2 | task | WP-51 | yes | frob-chronicle: restore handling per fork class: bitwise, cold contact solve, re-initialized queues and filters | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-05.md` |
| N27a-07 | M2 | task | WP-51 | yes | The fork-classification catalogue of 41 pairs, and the measurement of GATE-AUTOPSY-02 and GATE-CHRON-05 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-07.md` |
| N27a-08 | M2 | task | WP-51 | yes | End-to-end script for run.fork: fork point, classify-only, one child per class, refusals, untouched parent | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-08.md` |
| N27a-16 | M3 | task | WP-56 | yes | The thermal re-gear fixture: closed form and simulated realization of the fork-versus-rerun counterexample | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-16.md` |

## Addressed to your beads

- `BD-205`: request from round 4, slice H-verbs-7: VERBS-run now declares run.fork's `--children` plan as the params entry `children = "artifact-input<frob.fork.children/1>"` (891, 734 and 828 appended). As the bead that defines the child plan `frob.fork.children/1`, enter its row in `registries/formats.toml` (N34-27's rule fmt.param-without-row) and state the parameter kind in the verb section.
- `BD-283`: mechanical check of the conventions (can be a false alarm): [C5-key] 1 complete command(s) of a run verb in a code block carry no `--key` (convention 5), e.g. `frob autopsy rerun <run> ( --edit '<typed design edit>' | --do '<factor> := <value>' )`
- `BD-283`: request from round 4, slice H-verbs-1: Your refusal table copies design.edit's thirteen codes. VERBS-design says the code of the ledger slug sized-fill-unsatisfiable (a resolve-sized edit no stock wall carries; N31b-41, M3) joins design.edit's list under its assigned number; VERBS-autopsy now states that it joins autopsy.rerun's row in the same change. Add the same line to your table row for design.edit's codes.
