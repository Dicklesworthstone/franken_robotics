---
key: "NEG-31"
title: "e2e: `scripts/e2e/zoo_goldens.sh`: every zoo archetype instantiated, preflighted and compared with its golden"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "gauntlet", "frob-design", "solid", "test", "e2e"]
parent: "WP-38"
deps: ["STD-E2E", "NEG-30", "BD-109", "BD-110", "BD-102", "BD-022", "ZOO-quad-mammal", "ZOO-humanoid", "ZOO-diff-drive", "ZOO-mecanum"]
needs: []
related: ["N14b-06", "N14b-19", "BD-246", "BD-247"]
gates: ["GATE-SKEL-03"]
plan_refs: ["Appendix G.4", "§14.10", "§14.11", "§14.15 GATE-SKEL-03"]
---
## Background

The zoo holds one pinned instance and one expectation file per archetype (eighteen in all: four at M1,
fourteen at M3). The conformance test checks them in process. This script checks the same expectations from
the user's side, through the real `frob` binary, so that the verbs an agent actually calls (`design.templates`,
`design.new`, `design.show`, `design.check`, `analyze.symmetry`, `algebra.orbits`) are shown to return the
goldens. It differs from the archetype-catalogue script of the design-language unit, which walks the template
catalogue and the standing rows of the baseline controllers: this one is driven by the zoo's expectation
files and asserts every row in them, including the negative-control variants. It contributes rows to the gate
that says each archetype compiles, passes stages 1 to 6 with no refusal and has its symmetry groups on file
(GATE-SKEL-03; the standing and rolling clause is measured elsewhere).

## Scope

In: the script, its per-archetype loop, assertions and log. Out: the expectation files (the eighteen `ZOO-…`
beads); the loader (NEG-30); the catalogue gate record (BD-246).

## Technical approach

**Script.** `scripts/e2e/zoo_goldens.sh [--milestone Mn] [--only <name>]`; plain shell; sources
`scripts/e2e/lib.sh`; idempotent; works in a scratch workspace under `target/`.

**Per archetype (from `zoo/INDEX.toml`, skipping with a logged `not run` those whose `first_milestone` is
later than `--milestone`):**

1. `frob design templates --name <name> --json`: assert `available`, the number and the parameter list.
2. `frob design new --template <name> --out $W/<name>.fbot --json`: assert the returned `DesignId` equals the
   pinned one and the file is byte-identical to `zoo/<name>.fbot`.
3. `frob design show $W/<name>.fbot --section model --json`: assert `[morphology]` counts.
4. `frob analyze symmetry $W/<name>.fbot --json`: assert Γ_mech, default Γ_obs and generators; then for each
   `breaker`, `frob design edit … --json` followed by the symmetry verb, asserting the reduced group and the
   named breaker.
5. `frob design check $W/<name>.fbot --stages 1-6 --all --json`: assert each stage's status and reported
   values; assert zero refusals at defaults; record per-stage wall time against its budget (100, 50, 400, 200,
   150, 600 ms at the 95th percentile on the reference host; a miss is logged as a finding, not a failure of
   this script).
6. Known answers that have a verb surface (orbit and strata counts through `frob algebra orbits`; growth
   vectors through stage 4's report; mass and inertia rows through `design show --section mass`).
7. Each control variant: run the stated command and assert the stated refusal code and exit status, or the
   stated change in a reported value.
8. Determinism: steps 2 to 5 twice; `DesignId`, `ModelId` and the stage report digests identical.

**Logging.** Newline-delimited JSON at `target/e2e-logs/zoo_goldens/<utc>.ndjson` and a transcript. Every
line: `ts`, `script`, `bead` (the archetype's `ZOO-…` key), `step`, `event`, the exact command, exit code, wall
time, the envelope's `evidence` block (claim Known answer, class Tested, tier static or referee, determinism
D0, `DesignId`, `ModelId`), the expectation name, the measured value and the expected value with tolerance
kind, refusal code and repairs where one is expected, and on failure the first differing field and the paths
of the instance, the expectation file and the raw output. The `summary` line: archetypes passed, failed and
not run; expectations passed and failed; controls caught; stage-time findings; gates touched
(`GATE-SKEL-03`). All fixtures are shipped designs; no private data.

## Acceptance criteria

- [ ] At M1 the script passes for `quad-mammal`, `humanoid`, `diff-drive` and `mecanum` and logs fourteen
      `not run`; at M3 it passes for all eighteen.
- [ ] A planted change of one expectation value, a planted template drift and a planted removal of a control
      variant each make it exit non-zero with the archetype and row named.
- [ ] The instance written by `design new` is byte-identical to the zoo file on Linux and macOS.
- [ ] The summary's counts equal the number of rows in the expectation files due at the milestone.

## Test plan

### Unit tests

- `e2e_zoo_rows_cover_expectations`: every row kind of `frob.zoo.expect/1` has an assertion in the script (a
  table-driven check of the script's handlers against the schema).
- `e2e_zoo_not_run_is_not_pass`.
- `e2e_zoo_log_schema`.

### End-to-end script and logging

This bead is the script. `--self-test` runs it against a scratch copy of `zoo/` with the three planted
defects of the acceptance criteria and expects three failures.

### Known answers and negative controls

All values come from the expectation files. Script-level controls: the three planted defects. Metamorphic:
`--only <name>` yields the same lines for that archetype as the full run. Determinism: identities and report
digests equal twice on one machine and across the two platforms.

## Considerations

- Stage 7 and 8 rows (place and information) appear only for archetypes whose bead supplies a task and scene;
  they are logged `not run` until those stages land at M3.
- Tracked and hybrid archetypes compile and pass preflight at M3 and stand under their controllers only at M4;
  this script asserts nothing dynamic about them before then.

## References

Appendix G.4; §14.10; §14.11; §14.15 (GATE-SKEL-03).
