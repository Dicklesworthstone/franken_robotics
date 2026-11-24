---
key: "NEG-02"
title: "e2e: `scripts/e2e/worked_cards.sh`: all 41 worked cards recomputed, through verbs where a surface exists"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "gauntlet", "frob-conformance", "solid", "test", "e2e", "card", "review-notes-open"]
parent: "WP-14"
deps: ["BD-022", "GB-01", "GB-08", "GB-09", "GB-18", "GB-25", "NEG-01", "NEG-03", "STD-E2E"]
needs: []
related: []
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

The cards land at different milestones (five at M0; thirteen more at M1, eighteen in all; then 8, 8, 5 and 2
at M2 to M5, as `registries/cards/INDEX.toml` of NEG-01 lists them). The script therefore runs at every
milestone and distinguishes three states per card, lane, lane B check and surface call: `pass`, `fail` and `not
run` (the milestone it waits for has not been reached). `not run` is never counted as a pass, and the summary
prints all three counts. This bead depends on the five M0 cards, so that the script is proven on real cards when
it closes; every later card plugs its rows in (its `[[surface]]` entries, its refusals) and its own acceptance
includes passing in this script at its milestone. No card bead depends on this one.

## Scope

In: the script, its surface table, its assertions and its log. Out: the cards and their tests (GB-01 to
GB-41); the loader (NEG-01); the stale-value scan (NEG-03), which the script invokes as a step.

## Technical approach

**Script.** `scripts/e2e/worked_cards.sh [--milestone Mn] [--card GB-nn] [--seed S]`. Plain shell; sources
`scripts/e2e/lib.sh` (the shared helper of the end-to-end harness); idempotent; writes nothing outside
`target/`. Seeds for the cards' Monte Carlo checks are read from the card files and echoed.

**Steps.**

1. `frob doctor --json`: record toolchain, pins and build identity; abort on a dirty stack lock.
2. Registry step: `FROB_CARDS_EXPECT_THROUGH=Mn cargo test -p frob-conformance --test cards
   cards_registry_complete` with Mn the milestone under test (NEG-01's lint): it fails if a card whose index
   milestone is Mn or earlier is missing, and reports the later ones as pending.
3. Reference lane: `cargo test -p frob-conformance --test cards -- reference controls --format terse`, then the
   status dump (`cards_status`, a test that prints one JSON line per card from the loader's `status()`).
4. Mechanism lane: the same for `mechanism`.
5. Surface lane: for each `[[surface]]` entry of each card file whose `first_milestone` has arrived, run the
   named `frob … --json` command with the card's inputs and compare the mapped fields with the goldens under the
   card's tolerances (or, for an entry with `expect`, the exit code and refusal code). The table at plan time
   (each card bead states the exact command lines):

   | Card | Verb | Card | Verb |
   |---|---|---|---|
   | GB-02 | `algebra.irreps` | GB-22 | `sim.run` (expects FROB-REF-245) |
   | GB-03 | `analyze.closure`, `design.check --stages 6` | GB-23 | `sense.schedule` (`lockstep`, `referee`) |
   | GB-04 | `spark.probe` (records rounding mode only) | GB-24 | `mission.synthesize`, `task.fixture` |
   | GB-05 | `analyze.maneuverability`, `design.check --stages 4` | GB-25 | `spark.probe`, `prove.kernel` (M2, M4 checks staged) |
   | GB-06 | `study.plan` | GB-26 | `shield.budget`, then `frob-verify` |
   | GB-07 | `scene.compile`, `scene.show` | GB-27 | `gait.orbit` |
   | GB-10 | `design.show --section mass`, `design.check` | GB-28 | `train.family` (expects FROB-REF-761, M4) |
   | GB-11 | `actuator.thermal` | GB-29 | `eval.ledger` |
   | GB-12 | `actuator.envelope` | GB-30 | `eval.preanalyze`, `study.plan` |
   | GB-13 | `actuator.bus` | GB-31 | `mission.synthesize`, `prove.verify` |
   | GB-14 | `cores.show`, `eval.promote` | GB-32 | `nemesis.bound` |
   | GB-15 | `eval.promote` | GB-33 | `nemesis.cover` |
   | GB-16 | `sim.estimate` | GB-34 | `autopsy.attribute` |
   | GB-17 | `phylum.plan` | GB-35 | `opt.cascade` (M4) |
   | GB-19 | `algebra.orbits`, `phylum.quotient` | GB-36 | `experiment.suggest` |
   | GB-20 | `contact.probe` (cone) | GB-37 | `foundry.tolerance` |
   | GB-21 | `contact.probe` (film) | GB-38 | `foundry.optimize --loop` (M5) |
   | GB-39 | `cores.show` | GB-41 | `sim.estimate`, `sim.run` (expects FROB-REF-602) |
   | GB-01, -08, -09, -18, -40 | no verb: no `[[surface]]` entry; the surface step is the test binary's JSON status line (GB-18 and GB-40: the performance harness has no verb by design) | | |

6. Refusal checks: the cards that name a refusal are driven to it and the exit code, refusal code and repairs
   are asserted as their milestones arrive: through a verb, FROB-REF-105, -116, -131, -132, -133, -240, -245,
   -532 and -565 (exit code 3), -741 and -761 (exit code 4), -602 (exit code 6), and -165, -242, -260, -263 and
   -802 as each card bead states; through the test lane, FROB-REF-212, -740, -862 and -866. Refusal entries
   are data (`expect` in the card files), so a card bead that adds one adds no code to the script.
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

- [ ] With `--milestone M0` the script passes with GB-01, GB-08, GB-09, GB-18 and GB-25 `pass` in lane A and in
      every lane B check whose milestone is M0, GB-25's M2 and M4 checks `not run (M2)` and `not run (M4)`, and
      36 cards `not run` (reported, not hidden).
- [ ] The milestone rule holds in the script's own arithmetic (unit test below): at milestone Mn the cards
      required to pass lane A are those of the index through Mn (5, 18, 26, 34, 39, 41), and a lane B check or
      surface call is required to pass exactly when its `first_milestone` has arrived. Passing at M1 to M5 is
      asserted by the card beads of those milestones, each through this script.
- [ ] A planted wrong golden (one digit changed in any card) makes the script exit non-zero and the log names
      the card, the output, the measured and golden values.
- [ ] A planted `skip` of a due card is reported as a failure of the registry step.
- [ ] Two consecutive runs produce identical status dumps.

## Test plan

### Unit tests

- `e2e_worked_cards_surface_table_complete`: every `[[surface]]` entry of every card file names a verb present
  in the verb registry, with that verb's registered first milestone not later than the entry's
  `first_milestone`.
- `e2e_worked_cards_due_sets`: from `registries/cards/INDEX.toml`, the due sets at M0 to M5 have 5, 18, 26, 34,
  39 and 41 cards.
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
