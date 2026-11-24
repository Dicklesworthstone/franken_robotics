---
key: "NEG-31"
title: "e2e: `scripts/e2e/zoo_goldens.sh`: every zoo archetype instantiated, preflighted and compared with its golden"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "gauntlet", "frob-design", "solid", "test", "e2e"]
parent: "WP-38"
deps: ["BD-022", "BD-102", "BD-109", "BD-110", "NEG-30", "STD-E2E", "ZOO-diff-drive", "ZOO-humanoid", "ZOO-mecanum", "ZOO-quad-mammal"]
needs: []
related: ["ZOO-ackermann", "ZOO-balancer-2w", "ZOO-biped-point", "ZOO-hexapod-radial", "ZOO-mobile-manipulator", "ZOO-octopod", "ZOO-omni-3", "ZOO-quad-sprawl", "ZOO-rocker-bogie", "ZOO-snake-crawler", "ZOO-swerve-4", "ZOO-tracked-flipper", "ZOO-tri-wheel-stair", "ZOO-wheel-leg"]
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

In: the script, its per-archetype loop, assertions and log. This script is the only place where an archetype's
default instance is compared with its goldens from the user's side (pinned `DesignId`, byte-identity,
morphology counts, groups and breakers, stage values, known answers, control variants). One source per value
(DESIGN, the same statement in N14b-06): those goldens live in the zoo's expectation files and are pinned
nowhere else.

Out: the expectation files (the eighteen `ZOO-…` beads); the loader (NEG-30); the archetype-catalogue script
`scripts/e2e/skel_archetypes.sh` (N14b-06, which depends on this bead, runs this script as its first stage and
adds the template listing, presets and sensor options, the two views, the controller class, the standing rows,
the mass window of GATE-SKEL-07 and its four cross-cutting controls); the gate record: there is one record of
GATE-SKEL-03 per milestone, written from the log of `skel_archetypes.sh`, which carries this script's summary;
at M3 BD-246 runs `skel_archetypes.sh` and enters GATE-SKEL-07's M3 part, and the M3 part of GATE-SKEL-03 is
entered by BD-247 once the standing and rolling rows exist; this bead writes none.

## Technical approach

**Script.** `scripts/e2e/zoo_goldens.sh [--milestone Mn] [--only <name>]`; plain shell; sources
`scripts/e2e/lib.sh`; idempotent; works in a scratch workspace under `target/`.

**Per archetype (from `zoo/INDEX.toml`, skipping with a logged `not run` those whose `first_milestone` is
later than `--milestone`):**

1. `frob design templates --name <name> --json`: assert `available`, the number and the parameter list.
2. `frob design new zoo-<name> --template <name> --key zoo-<name>-<run> --json` (`design.new` takes the new
   design's name and an idempotency key and has no output-file option, N14a-21): assert the returned `DesignId`
   equals the pinned one; then `frob design show <DesignId> --section term --json` and assert its canonical
   bytes are byte-identical to `zoo/<name>.fbot`. Every later step addresses the instance by this immutable
   `DesignId`, so the breaker edits of step 4 (new versions of `zoo-<name>`) change nothing the other steps read.
3. `frob design show <DesignId> --section summary --section channels --json`: assert the `[morphology]`
   counts (bodies, actuated coordinates, n_v, end effectors, floating base).
4. `frob analyze symmetry <DesignId> --json`: assert Γ_mech, default Γ_obs and generators; then for each
   `breaker`, `frob design edit zoo-<name> <breaker edit> --expected-parent <DesignId> --key
   zoo-<name>-brk-<i> --json` followed by the symmetry verb on the returned version, asserting the reduced group
   and the named breaker. A `[symmetry]` table or breaker row marked `blocked` is logged `blocked: <reason>` with
   the verb's output beside it and is not asserted.
5. `frob design check <DesignId> --stages 1-6 --all --json`: assert each stage's status, note and reported
   values; assert zero refusals at defaults (a refusal on a stage whose row is marked `blocked` is logged with
   that row, `blocked: <reason>`, and is neither asserted nor counted); record per-stage wall time against its budget (100, 50, 400, 200,
   150, 755 ms at the 95th percentile on the reference host, stage 6 as revised by N14bx-01 for a request with
   nothing bound; a miss is logged as a finding, not a failure of this script).
6. Known answers that have a verb surface (orbit and strata counts through `frob algebra orbits`; growth
   vectors through stage 4's report; mass and inertia rows through `design show --section mass`).
7. Each control variant: run the stated command and assert the stated refusal code and exit status, or the
   stated change in a reported value.
8. Determinism: steps 2 to 5 a second time under the name `zoo-<name>-2` with its own key (a repeated key
   would only replay the stored answer); `DesignId`, `ModelId` and the stage report digests identical.

**Logging.** Newline-delimited JSON at `target/e2e-logs/zoo_goldens/<utc>.ndjson` and a transcript. Every
line: `ts`, `script`, `bead` (the archetype's `ZOO-…` key), `step`, `event`, the exact command, exit code, wall
time, the envelope's `evidence` block (claim Known answer, class Tested, tier static or referee, determinism
D0, `DesignId`, `ModelId`), the expectation name, the measured value and the expected value with tolerance
kind, refusal code and repairs where one is expected, and on failure the first differing field and the paths
of the instance, the expectation file and the raw output. The `summary` line: archetypes passed, failed and
not run; expectations passed and failed, and known answers and symmetry rows `blocked` counted apart by reason
(`derivation`, `admission`, `milestone`, `open` with the bead named; NEG-30's field), never as passed; controls caught; stage-time findings; gates touched
(`GATE-SKEL-03`). All fixtures are shipped designs; no private data.

## Acceptance criteria

- [ ] At M1 the script passes for `quad-mammal`, `humanoid`, `diff-drive` and `mecanum` and logs fourteen
      `not run`. The M3 rows are not this M1 bead's to close: each of the fourteen M3 zoo beads depends on this
      script and passes its own rows at M3 (the full M3 run is the sum of those beads' acceptance, and N14b-06's
      M3 gate run reads its summary); the M4 rows of `tracked-flipper` and `wheel-leg` are NEGp1-53's.
- [ ] A planted change of one expectation value, a planted template drift and a planted removal of a control
      variant each make it exit non-zero with the archetype and row named.
- [ ] The instance created by `design new`, read back with `design show --section term`, is byte-identical to the
      zoo file on Linux and macOS.
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
