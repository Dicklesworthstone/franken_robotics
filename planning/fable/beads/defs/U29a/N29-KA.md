---
key: "N29-KA"
title: "Chronicle known-answer registry: KA-707 to KA-720 with truths, tolerances, tiers and negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "chronicle", "frob-conformance", "solid", "registry", "test"]
parent: "WP-13"
deps: ["STD-TEST", "STD-E2E"]
needs: ["§38.6: the known-answer registry file format and runner of the verification harness (seed BD-146 wires the M1 rows, BD-223 the M2 rows)"]
related: ["BD-091", "BD-092", "BD-095", "BD-203", "BD-204", "BD-206", "BD-207", "BD-208", "BD-264", "N29a-04", "N29a-06", "N29a-12", "N29a-13", "N29a-16", "N29a-19", "N29a-21", "N29a-22"]
gates: []
plan_refs: ["§29.19", "Appendix M.28", "§38.6", "§29.20"]
---
## Background

A known answer is a case whose truth is fixed before the implementation exists: an analytic value, a counted
quantity, or a behaviour that follows from the design by argument. The record subsystem (Chronicle, crate
`frob-chronicle`) is tested against fourteen of them, numbered KA-707 to KA-720 in the project's known-answer
suite. Each has a *negative control*: a planted defect or a tempting shortcut that the test must catch. The
project's rule is that a gate nothing can fail validates nothing, so a known answer with no caught control does not
count as passed.

This bead is the registry of those fourteen rows. It carries each row in full so that the mechanism beads can cite
a number and a value, and it delivers the registry entries (data), not the mechanisms. Tier names: G0 laws, G1
known answers, G4 chaos, G5 determinism; the site after the tier says where the test runs (`host` = a test binary
in exact or double-precision arithmetic; `referee` = the double-precision reference tier; `lockstep` = the
throughput tiers; `daemon` = the job daemon under the deterministic simulation runtime; `all tiers` = referee,
`batch-cpu`, CUDA and Metal).

## Scope

In: the fourteen registry rows as `registries/known-answers/chronicle.toml` entries; for each, the fixture
identifier, the truth, the tolerance, the tier and site, the negative control, and the bead that implements the
test. Out: the tests themselves (the mechanism and gate beads named in the last column); the counterexample
registry (`N29-CX`) and the refusal registry (`N29-REF`), which another unit writes.

## Technical approach

One TOML table per row (DESIGN): `id`, `section`, `case`, `truth`, `tolerance`, `tier`, `site`,
`negative_control`, `test` (crate path and test name), `fixture`, `derivation` (path of the derivation file under
`docs/derivations/`), `gate`. The runner refuses to mark a row `pass` unless its negative control was caught in
the same run.

| KA | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Carried by |
|---|---|---|---|---|---|---|
| KA-707 | Energy ledger of a damped pendulum. Fixture (DESIGN numbers; the suite names the case without numbers): point mass 1 kg on a massless 1 m rod, g = 9.81 m/s², viscous joint damping 0.5 N·m·s/rad, released at rest from 60°, run 60 s | Equal to the hand calculation: Δ potential = −m·g·l·(1 − cos 60°) = −4.905 J; Δ kinetic = 0 (below 10⁻¹² J at 60 s); joint damping + numerical residual = 4.905 J; ledger residual below 10⁻⁹ relative (GATE-CHRON-03) | 10⁻⁹ relative | G1 · referee | A sign error seeded at one port is localized to that port | BD-092 |
| KA-708 | Energy per task: design A succeeds once for 10 J and fails once for 20 J; design B succeeds twice for 12 J each | Mean over completed episodes: A 10 J, B 12 J. Total over all attempts divided by completions: A 30 J, B 12 J. The first figure alone ranks them wrongly | exact | G1 · host | A report that prints only the conditional mean | BD-092 |
| KA-709 | Created energy: a body bouncing on a stiff penalty contact at a step near the stability limit; the same drop under the convex contact law at the default step | The bounce raises `energy-created` and `contact-active`; the convex law raises neither | exact (flags) | G1 · referee, lockstep | The audit switched off: the bouncing policy wins the comparison | N29a-22 |
| KA-710 | Cone excess: a block pushed along the diagonal of a pyramidal (four-sided) friction cone | Friction utilization ‖f_t‖/(μ·f_n) reaches √2 ≈ 1.41421; `cone-excess` is raised | to set (DESIGN: 10⁻³ relative on utilization) | G1 · lockstep | A circular-cone run must not be flagged | N29a-22 |
| KA-711 | Chunk protocol | Two identical publishes yield one row and one receipt; a crash after commit returns the original receipt | exact | G4 · daemon | A late lease that writes; overlapping ranges that commit; a reset at tick zero that collides | BD-091, N29a-06 |
| KA-712 | Pagination: 10⁴ rows read in pages during concurrent appends | Each row visible at the pin appears exactly once | exact | G4 · daemon | A query ordered by timestamp alone repeats or skips equal-time samples | N29a-04, N29a-06 |
| KA-713 | Sampling of traced worlds by address | The traced set is identical under 100 slot assignments and shard counts | exact | G5 · all tiers | Arrival-order reservoir sampling gives different sets | N29a-12, N29a-16 |
| KA-714 | Integer histogram | Counts identical under 100 permutations and shard splits, on every backend; integer bin counts 1, 1, 1 for the streams 1, 2, 3 and 3, 2, 1 | exact | G5 · all tiers | An order-dependent sketch: a compactor of capacity two that keeps the lower of each sorted pair gives medians 1 and 2 for the two orders | N29a-13, N29a-14, N29a-16 |
| KA-715 | Downsampling: an impulse train summed over a window | Momentum transfer is conserved (the window sum equals the sum of the impulses) | to set (exact in quantized integers) | G1 · host | The same train averaged | BD-090 (another unit) |
| KA-716 | Fork | The empty intervention replays bit for bit; a setpoint change at tick t reuses the prefix to the snapshot before t | exact (digest) | G5 · all tiers | The 60 mm wheel's contact warm start grafted onto the 75 mm wheel (refused, FROB-REF-582) | BD-205 (another unit); BD-203 for the replay half |
| KA-717 | First divergence: traces that differ at ticks 1 and 5 and agree at ticks 2 to 4 | Tick 1 | exact | G1 · host | Midpoint search on the pointwise predicate "tick k differs" reports tick 5 | BD-206, N29a-21 |
| KA-718 | Teleport | A planted single-kernel divergence is named by step and stage | exact | G1 · all tiers | None stated in the suite. Control added here (DESIGN): a localizer that reports the stage in which the digest *first printed* differs, on a plant in stage 6, must be caught naming stage 9 | BD-204 |
| KA-719 | Tier correction (the estimator Zᵢ = mᵢ + Aᵢ·(Y_ref,i − mᵢ)/pᵢ, θ̂ = mean of Zᵢ) | With every address audited (pᵢ = 1) it equals the plain referee mean exactly; with a prediction off by a constant it is still unbiased | exact for the first; inside the interval for the second | G0 · host | Audit bits chosen after seeing fast-tier failures, without recorded probabilities | BD-264 (another unit) |
| KA-720 | Artifact graph | Every gate row resolves to evidence and traces | exact | G1 · host | A claim printed after its evidence was collected | BD-095, N29a-10, BD-209 |

Derivation files this registry requires (one page each, written by the carrying bead):
`chronicle-pendulum-ledger.md` (KA-707), `chronicle-energy-per-task.md` (KA-708),
`chronicle-pyramid-diagonal.md` (KA-710: a pyramid with |f_x| ≤ μ·f_n and |f_y| ≤ μ·f_n admits
(μ·f_n, μ·f_n), whose norm is √2·μ·f_n), `chronicle-k-smallest-uniform.md` (KA-713),
`chronicle-compactor.md` (KA-714: the stream 1, 2, 3 compacts to {1 with weight 2, 3}, median 1; the stream
3, 2, 1 compacts to {2 with weight 2, 1}, median 2), `first-divergence-monotonicity.md` (KA-717).

## Acceptance criteria

- [ ] `registries/known-answers/chronicle.toml` holds exactly fourteen rows, KA-707 to KA-720, each with every
      field of the schema; the registry schema check passes.
- [ ] Every row names a test that exists and a negative control registered in the negative-control registry; a
      row whose control is `none stated` in the suite (KA-718) carries the control added above and says so.
- [ ] The six derivation files exist and each reproduces its truth by hand arithmetic.
- [ ] A row marked `pass` without a caught control in the same run is refused by the runner (planted).
- [ ] Rows owned by another unit's bead (KA-715, KA-716 fork half, KA-719) point at that bead's test and are not
      duplicated.

## Test plan

### Unit tests

`cargo test -p frob-conformance known_answers::chronicle`:
- `registry_has_fourteen_rows`: ids are exactly KA-707..KA-720, no gaps, no duplicates.
- `every_row_has_test_and_control`: each `test` path resolves to a registered test; each `negative_control`
  resolves to a registry entry.
- `ka708_arithmetic`: (10)/1 = 10 and (10 + 20)/1 = 30; (12 + 12)/2 = 12 both ways.
- `ka710_arithmetic`: ‖(1, 1)‖ = √2; utilization 1.41421356 within 10⁻⁸ of √2.
- `ka714_compactor`: the capacity-two compactor on 1, 2, 3 and on 3, 2, 1 yields medians 1 and 2; integer bins
  yield counts (1, 1, 1) both ways.
- `ka707_hand_value`: 1 × 9.81 × 1 × (1 − 0.5) = 4.905.
- `pass_without_control_refused` (planted): a result record with `verdict = pass` and an empty `controls_caught`
  list is rejected.

### End-to-end script and logging

`scripts/e2e/chronicle_known_answers.sh` runs the harness the way a gate recorder does, `cargo test -p
frob-conformance --test g1 -- KA-707` through `KA-720` (one invocation per row, at the tier the row names), reads
the hash-named receipt each run writes, and asserts that each of the fourteen rows reports `pass` with its
control caught or `not run` with a reason (rows of a later milestone). It sources the shared helper and writes
`target/e2e-logs/chronicle_known_answers/<utc>.ndjson`: one `assert` line per row with the measured value, the
truth, the tolerance, the tier and site, the control's outcome and the evidence block; a `summary` line with
counts passed, failed and not run.

### Known answers and negative controls

This bead is the registry; its own negative control is the planted `pass` without a caught control. Determinism:
the registry file is canonical TOML and its digest is recorded in the gate log, so a silent edit of a truth
changes the digest.

## Considerations

- KA-707's numbers are a DESIGN choice of this registry (the suite names the case only); a change of fixture
  constants after the first recorded pass changes the fixture's identity and must be recorded as such.
- KA-710's tolerance is "to set": the value above is the proposal and is frozen with the first recorded run.
- Rows at milestone M2 (KA-709, -710, -713, -714, -716 to -718) are `not run` at the M1 exit by design.

## References

§29.19; Appendix M.28 (KA-707 to KA-720); §38.6; GATE-CHRON-03, -04, -06, -07, -08, -10.
