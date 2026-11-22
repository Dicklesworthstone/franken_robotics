---
key: "NEG-02"
title: "e2e: `scripts/e2e/worked_cards.sh`: all 41 worked cards recomputed, through verbs where a surface exists"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "gauntlet", "frob-conformance", "solid", "test", "e2e", "card"]
parent: "WP-14"
deps: ["STD-E2E", "NEG-01", "NEG-03", "BD-022", "GB-01", "GB-08", "GB-09", "GB-25"]
needs: []
related: ["BD-146", "BD-223", "BD-318", "BD-377", "BD-411"]
gates: []
plan_refs: ["Appendix E", "§38", "§0.2"]
---
## Background

The 41 worked cards are goldens: one file each in `registries/cards/`, recomputed by tests in two lanes (a
reference lane that evaluates the card's equation independently, and a mechanism lane that asks the owning
crate, and where one exists the public verb, for the same numbers). A user or reviewer needs one command that
answers "do all the cards the project relies on still reproduce, and through the real binary?", with a log
that shows every measured value beside its golden. This bead is that script. It is one end-to-end test for the
card capability as a whole, not one per card; each card bead contributes its row to the script's table.

The cards land at different milestones (four at M0, twelve by M1, the rest with their mechanisms up to M5).
The script therefore runs at every milestone and distinguishes three states per card and lane: `pass`, `fail`
and `not run` (the mechanism's milestone has not been reached). `not run` is never counted as a pass, and the
summary prints all three counts.

## Scope

In: the script, its surface table, its assertions and its log. Out: the cards and their tests (GB-01 to
GB-41); the loader (NEG-01); the stale-value scan (NEG-03), which the script invokes as a step.

## Technical approach

**Script.** `scripts/e2e/worked_cards.sh [--milestone Mn] [--card GB-nn] [--seed S]`. Plain shell; sources
`scripts/e2e/lib.sh` (the shared helper of the end-to-end harness); idempotent; writes nothing outside
`target/`. Seeds for the cards' Monte Carlo checks are read from the card files and echoed.

**Steps.**

1. `frob doctor --json`: record toolchain, pins and build identity; abort on a dirty stack lock.
2. Registry step: `cargo test -p frob-conformance --test cards cards_registry_complete`. At a milestone before
   all cards exist the lint is run with `--expect-through Mn` and must report exactly the cards due by then.
3. Reference lane: `cargo test -p frob-conformance --test cards -- reference controls --format terse`, then the
   status dump (`cards_status`, a test that prints one JSON line per card from the loader's `status()`).
4. Mechanism lane: the same for `mechanism`.
5. Surface lane: for each card whose file has a `[surface]` block, run the named `frob … --json` command with
   the card's inputs and compare the mapped fields with the goldens under the card's tolerances. The table at
   plan time:

   | Card | Verb | Card | Verb |
   |---|---|---|---|
   | GB-02 | `algebra.irreps` | GB-21 | `contact.probe` (film) |
   | GB-03 | `design.check --stages 6` | GB-23 | `sense.schedule` |
   | GB-04 | `spark.probe` (records rounding mode only) | GB-24 | `mission.synthesize`, `task.fixture` |
   | GB-05 | `design.check --stages 4` | GB-25 | `spark.probe`, `prove.kernel` |
   | GB-06 | `study.plan` | GB-26 | `shield.budget`, then `frob-verify` |
   | GB-07 | `scene.compile`, `scene.show` | GB-27 | `gait.orbit` |
   | GB-10 | `design.show`, `design.check --stages 1` | GB-29 | `eval.ledger` |
   | GB-11 | `actuator.thermal` | GB-30 | `eval.preanalyze`, `study.plan` |
   | GB-12 | `actuator.envelope` | GB-31 | `mission.synthesize`, `prove.verify` |
   | GB-13 | `actuator.bus` | GB-32 | `nemesis.bound` |
   | GB-14 | `cores.show`, `eval.promote` | GB-33 | `nemesis.cover` |
   | GB-15 | `eval.promote` | GB-34 | `autopsy.attribute` |
   | GB-16 | `sim.estimate` | GB-36 | `experiment.suggest` |
   | GB-17 | `phylum.plan` | GB-37 | `foundry.tolerance` |
   | GB-19 | `algebra.orbits`, `phylum.quotient` | GB-39 | `cores.show` |
   | GB-20 | `contact.probe` | GB-41 | `sim.estimate` |
   | GB-22 | `sim.run` (expects FROB-REF-245) | GB-01, -08, -09, -18, -28, -35, -38, -40 | no verb: `surface: none` |

6. Refusal checks: the cards that name a refusal are driven to it and the exit code, refusal code and repairs
   are asserted (FROB-REF-105, -116, -131, -132, -133, -212 through the test lane, -242, -245, -260, -263,
   -532, -602, -741, -866 as their milestones arrive).
7. Stale-value step: run the scan of NEG-03 and require zero hits.
8. Determinism: steps 3 to 5 are run twice; the status dumps and the surface outputs' digests must be
   byte-identical on one machine. Across platforms the dump is compared by value under each card's tolerance,
   and cards of tolerance kind `exact`, `count` and `bits` must match byte for byte.

**Assertions.** No card `fail`; every card due by the milestone has lane A `pass`; every card whose
`first_milestone` has arrived has lane B `pass`; every control `caught`; a card with a pending FACT input is
listed under `not citable` and does not fail the script.

**Logging.** Newline-delimited JSON at `target/e2e-logs/worked_cards/<utc>.ndjson` with a transcript beside it.
Every line: `ts`, `script`, `bead` (the card's key), `step`, `event` (`start`, `cmd`, `assert`, `metric`,
`refusal`, `end`), the exact command line, exit code, wall time, the envelope's `evidence` block (claim
Known answer, class, tier, determinism, identities, seed), the output name, measured value with unit, golden,
tolerance kind and bound, lane, and on failure the first differing value and the path of the card file and of
the raw verb output. The `summary` line gives cards by state per lane, controls caught, cards not citable,
assertions passed and failed, refusals observed, and `gates: []` (cards touch no gate directly). No
private-scene data can occur: every fixture is `synthetic`.

## Acceptance criteria

- [ ] At M0 the script passes with GB-01, GB-08, GB-09 and GB-25 `pass` in both lanes and 37 cards `not run`
      (reported, not hidden).
- [ ] At each later milestone the set of cards with lane B `pass` equals the set whose `first_milestone` has
      arrived; at M5 all 41 pass both lanes.
- [ ] A planted wrong golden (one digit changed in any card) makes the script exit non-zero and the log names
      the card, the output, the measured and golden values.
- [ ] A planted `skip` of a due card is reported as a failure of the registry step.
- [ ] Two consecutive runs produce identical status dumps.

## Test plan

### Unit tests

- `e2e_worked_cards_surface_table_complete`: every card file with a `[surface]` block has a row in the script,
  and every row names a verb present in the verb registry.
- `e2e_worked_cards_not_run_is_not_pass`: the summary arithmetic counts `not run` separately.
- `e2e_worked_cards_log_schema`: a recorded log validates against the log schema of the shared helper.

### End-to-end script and logging

This bead is the script. Its own negative controls are run by `scripts/e2e/worked_cards.sh --self-test`: a
scratch copy of `registries/cards/` with (a) one golden altered, (b) one control removed, (c) one card file
deleted; each must make the corresponding step fail.

### Known answers and negative controls

The script carries no numbers of its own: goldens live in the card files. Controls: the three self-test
mutations; metamorphic: running a single card with `--card` yields the same lines for that card as the full
run.

## Considerations

- The surface table changes when a verb is renamed; the unit test ties it to the verb registry so that drift
  fails fast.
- Runtime: the reference lane includes two enumerations (10⁶ colourings, 735,471 subsets) and several seeded
  simulations; the whole script is budgeted at under two minutes on the reference host, and the log records
  wall time per card so that a regression is visible.

## References

Appendix E; §38 (tiers and negative controls); §0.2 (what "implemented" means).
