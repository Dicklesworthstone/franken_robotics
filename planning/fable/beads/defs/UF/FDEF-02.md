---
key: "FDEF-02"
title: "defect report, frankensim `fs-mbd::planar_drive`: near-straight motion at a large heading returns zero translation"
type: "task"
priority: 2
milestone: "M0"
labels: ["m0", "suite", "traction", "upstream", "frankensim", "solid", "test"]
parent: "WP-37"
deps: ["BD-033", "BD-037", "STD-E2E"]
needs: []
related: ["FD-FSIM-11", "BD-025", "BD-117"]
gates: ["INT-FSIM-05"]
plan_refs: ["Appendix F.2 row 2", "§33.11", "§18.9", "§33.10", "Appendix F.3", "§0.1"]
---
## Background

`fs-mbd::planar_drive` in FrankenSim integrates a prescribed-speed differential drive. This project wants it as
a known-answer source for wheeled bases: an independent integrator against which the rolling of a simulated
base is checked. A defect was reported in it: near-straight motion at a large heading returns zero translation.
The plan's rule: a reported defect in a foundation is a **FACT to recheck**; it becomes an admission test here
first (INT-FSIM-05, BD-033) and an upstream work item there second (FD-FSIM-11). This bead is the defect report
between the two, with a reproduction that runs inside FrankenSim without this repository.

The mechanism was confirmed by reading the source on 2026-11-21 at commit `483d09c682f2` and by the rounding
arithmetic below; the run was not repeated for the plan. The reported result stays a report until the admission
test records it at the pin.

## Scope

In: the report (`tests/upstream/FD-FSIM-11/REPORT.md`, also the body of the mirrored bead in FrankenSim's
tracker); the standalone reproduction; the observed values copied from INT-FSIM-05's record; the guard kept
here. Out: the admission test and the independent formula's implementation in `frob-conformance` (BD-033); the
fix (FD-FSIM-11).

## Technical approach

**The defect.** `frankensim/crates/fs-mbd/src/planar_drive.rs:117–133`. The step takes its straight branch only
when |ω| ≤ 10⁻¹² rad/s, where ω is the yaw rate. Otherwise it forms θ′ = θ + ω·Δt and returns the arc
displacement R·(sin θ′ − sin θ) and −R·(cos θ′ − cos θ), with R = v/ω the turning radius, v the forward speed
and θ the heading. The threshold is on the rate, but what matters is whether the increment ω·Δt survives the
addition to θ in double precision.

**Smallest reproducing case, with the arithmetic.** θ = 32 rad; wheel speeds 1 and 1 + 6·10⁻¹³ m/s; track
0.5 m; Δt = 1/400 s; 400 steps.

- ω = (6·10⁻¹³ m/s)/(0.5 m) = 1.2·10⁻¹² rad/s, just above the 10⁻¹² threshold, so the curved branch is taken.
- ω·Δt = 3·10⁻¹⁵ rad.
- The spacing of doubles in [32, 64) is 2⁻⁴⁷, and half of it is 2⁻⁴⁸ ≈ 3.55·10⁻¹⁵. Since 3·10⁻¹⁵ is below that,
  θ′ rounds to θ, both differences are exactly zero, and the step returns zero translation. Both wheels advance
  and the chassis does not move.
- Over 400 steps (one second) the true displacement is one metre along the heading:
  (cos 32, sin 32) m ≈ (0.83422336, 0.55142668) m.

**The independent formula (the oracle that replaces it).** Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)),
with δ = ω·Δt the heading increment, sinc(x) = sin(x)/x and sinc(0) = 1. It has no branch on ω and no
difference of nearly equal trigonometric values. Hand-checkable case: unit speed for unit time through a half
turn from θ = 0 (v·Δt = 1 m, δ = π) gives the chord (0, 2/π) m ≈ (0, 0.63661977) m.

**The reproduction (standalone).** One Rust test file depending only on `fs-mbd` at the pinned commit, suitable
for that crate's `tests/` directory. It calls the public planar-drive step 400 times with the inputs above,
prints the accumulated displacement beside the closed form, and also prints the three intermediate numbers
(ω, ω·Δt, and whether θ + ω·Δt == θ in `f64`) so that a reader sees the mechanism and not only the symptom.

**Proposed fix (carried by FD-FSIM-11).** Either choose the straight branch on the heading increment relative
to the heading's rounding unit (take it when |ω·Δt| is below a small multiple of the unit in the last place of
θ), or replace both branches by the half-angle chord form above. The second is preferred, because it removes
the branch. Additive form: a corrected step beside the present one, with the present one documented; correcting
in place is FrankenSim's decision under its own rules.

**Admission test and pass condition (INT-FSIM-05, run by BD-033).** Against the chord formula, to 10⁻¹²: exact
straight travel; the near-zero-curvature case above; wrapped against unwrapped headings (θ and θ + 2π·k give the
same displacement); incremental against one-shot integration (400 steps of 1/400 s against one step of 1 s).
The seed bead's done-condition adds: the record states whether zero translation reproduces at the pin; the
independent formula returns the chord (0, 2/π) m and one metre over 400 steps at heading 32 rad.

**The guard this repository keeps meanwhile.** Known answers for wheeled bases use the independent formula,
which lives in `frob-conformance`. `planar_drive` is not an oracle until INT-FSIM-05 passes: no fixture, known
answer or parity test may cite it, and the oracle registry of `frob-conformance` lists it as `not-admitted`
with this report as the reason.

**Procedure.** (1) Copy the observed displacement and the commit from INT-FSIM-05's record into the report
(`observed_at`). (2) If zero translation does not reproduce at the pin, the report says "not reproduced at
`<rev>`", FD-FSIM-11 is closed as not needed after INT-FSIM-05 passes in full, and `planar_drive` may be
admitted as an oracle. (3) Run the reproduction in a scratch clone at the pin. (4) File the report as the
mirrored bead of FD-FSIM-11 in FrankenSim's tracker and store its identifier in `registries/upstream.toml`.

## Acceptance criteria

- [ ] `tests/upstream/FD-FSIM-11/REPORT.md` has the sections Defect, Evidence (file, lines, commit), Smallest reproducing case with the rounding arithmetic, Expected and observed, Proposed fix, Interim posture, and `observed_at` with the recorded values.
- [ ] The reproduction compiles against `fs-mbd` alone at the pin and prints the accumulated displacement, the closed form (0.83422336, 0.55142668) m, and the three intermediate numbers.
- [ ] The report is filed in FrankenSim's tracker; its identifier is the `mirror` of FD-FSIM-11's registry row.
- [ ] The oracle registry of `frob-conformance` lists `planar_drive` as `not-admitted`, and a known answer that cites it is rejected by the registry loader.
- [ ] The chord formula in `frob-conformance` returns (0, 2/π) m for the half turn and one metre along heading 32 rad over 400 steps, each to 10⁻¹².

## Test plan

### Unit tests
- `repro_zero_translation_at_heading_32`: the inputs above; expected (correct) displacement (0.83422336, 0.55142668) m to 10⁻⁸; the observed value is compared with the report's `observed_at` entry.
- `repro_increment_is_absorbed`: asserts that 32.0 + 3e-15 == 32.0 in `f64` and that 2⁻⁴⁸ > 3·10⁻¹⁵ (the mechanism, independent of FrankenSim).
- `chord_half_turn`: v·Δt = 1, δ = π, θ = 0; expected (0, 2/π) to 10⁻¹².
- `chord_wrapped_equals_unwrapped`: θ = 0.584073464 and θ = 0.584073464 + 10π give displacements equal to 10⁻¹².
- `oracle_registry_rejects_planar_drive`: a planted known answer citing `fs-mbd::planar_drive` fails to load.

### End-to-end script and logging
`scripts/e2e/defect_fdef_02.sh`: clones FrankenSim's pinned commit into `target/upstream-scratch/frankensim@<rev>`,
runs the reproduction there, asserts that the status (reproduced or not) equals the report's, runs the chord
tests of `frob-conformance`, and reads the registry row back with `frob doctor --json`.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/defect_fdef_02/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FDEF-02`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- KA-775 (the near-zero-curvature case; truth one metre along the heading, tolerance 10⁻¹² for the formula) and KA-776 (the chord (0, 2/π) m). CX-352: "a branch on abs(ω) ≤ 10⁻¹² rad/s separates straight from curved motion". CX-182: "a library integrator for a differential drive can serve as the rolling oracle".
- Negative control: a local copy of the reported branch logic (threshold on ω, difference of sines), written in the test, must return exactly zero for the case; if it does not, the reproduction's reading of the mechanism is wrong.
- Metamorphic relations: displacement is invariant under θ → θ + 2π·k; n steps of Δt/n compose to one step of Δt to 10⁻¹².
- Determinism lanes: identical values twice on one machine and in debug and release; this Mac and the Linux host may differ in `sin` and `cos` by platform, which the tolerance of 10⁻¹² covers.

## Considerations

- RISK-24: a reported foundation defect relied on before it is rechecked. The guard (the registry entry `not-admitted`) is what prevents a wheeled-base known answer from quietly citing the defective integrator.
- The same file holds the G1 and iiwa7 catalog builders' neighbours (`robot_models.rs` is admitted with INT-FSIM-05); the report concerns `planar_drive` only.
- No milestone waits for the fix. If it never lands, the independent formula stays the oracle and nothing is lost but a second opinion.
- When the fix lands it is adopted only through its switch-over rule (re-run INT-FSIM-05; then admit `planar_drive` as an oracle beside the formula); neither a pin nor the toolchain moves during the ten-fold tournament.

## References

Appendix F.2 (row 2), F.3; §33.11 (row 2, the arithmetic), §18.9, §33.10; INT-FSIM-05; FD-FSIM-11; KA-775, KA-776; CX-352, CX-182; RISK-24.
