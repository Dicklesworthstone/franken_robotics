---
key: "FDEF-06"
title: "defect report, frankensim `gaussian_action`: correct marginals, wrong joint law (sign agreement 0.6655)"
type: "task"
priority: 2
milestone: "M2"
labels: ["m2", "suite", "cortex", "upstream", "frankensim", "solid", "test"]
parent: "WP-48"
deps: ["BD-034", "BD-037", "N23b-30", "N33c-37", "STD-E2E"]
needs: []
related: ["N41-38"]
gates: ["GATE-CORTEX-18", "GATE-WS-06"]
plan_refs: ["Appendix F.2 row 6", "§33.17", "§23.25", "§23.6", "§41", "Appendix F.3"]
---
## Background

The suite's only PPO trainer, FrankenSim's `fs-g1-train`, samples actions with `gaussian_action`. Each channel
of the action has the right marginal distribution: a standard normal scaled and shifted. The channels are not
independent. Adjacent channels agree in sign with probability 0.6655, where independence gives ½. The trainer
then scores these samples with a product of marginal densities, so its likelihood ratio is computed for a random
variable that was not the one sampled, and exploration is correlated across joints.

This project does not link that sampler. The defect still matters here for two reasons. It is the reason for
rule P-12 of this project's PPO battery (the joint law of the sampled action equals the law the likelihood
computation assumes; marginal checks do not establish it), and a planted copy of the sibling's sampler is that
rule's negative control. And the sibling trainer cannot serve as a comparison baseline until it is fixed. The
plan's rule applies: a FACT to recheck, an admission test here first (the joint-law battery, GATE-CORTEX-18;
the planted sampler is rejected, GATE-WS-06), an upstream work item there second (FD-FSIM-32).

Read in source on 2026-11-21 at `483d09c682f2`: the mechanism is confirmed by reading
(`frankensim/crates/fs-g1-train/src/ppo.rs:172–189`); the fraction is arithmetic on the two constants in the
source and was not executed for the plan.

## Scope

In: the report (`tests/upstream/FD-FSIM-32/REPORT.md`, also the body of the mirrored bead); the standalone
reproduction; the battery's recorded statistics; the statement of the guard. Out: the sampler of this project
and its manifest, the battery and the refusal in `train.start` (owned by the learning chapter's beads); the
three PPO-update errors (FDEF-01); the upstream fix (FD-FSIM-32).

## Technical approach

**The mechanism.** `gaussian_action` draws each channel from `gaussian_sample`, a Box–Muller cosine over a
counter. The counter advances by the constant 0x9E3779B97F4A7C15 per draw, and the phase uniform is taken from
the top 53 bits of the counter multiplied by 0x94D049BB133111EB. The phase is therefore a linear function of the
counter modulo 1, and consecutive draws differ in phase by a fixed fraction of a turn:

  (0x9E3779B97F4A7C15 · 0x94D049BB133111EB mod 2⁶⁴) / 2⁶⁴ = 0.167245 of a turn, an angle φ = 0.334490·π.

For a uniform angle θ, cos θ and cos(θ + φ) have the same sign with probability 1 − φ/π. So two adjacent
channels agree in sign with probability 1 − 0.334490 = 0.665510, where independence gives 0.5. Each channel
alone is positive half the time, so every marginal test passes.

**Reported observation.** In 100,000 two-channel calls per seed, the agreement counts are 66,552, 66,552 and
66,551 (three seeds). Under a product law the agreement frequency over N decisions has mean ½ and standard
deviation ½/√N, which is 0.00158 at N = 100,000; the observed 0.6655 is about 105 standard deviations away.

**The reproduction (standalone).** One Rust test depending only on `fs-g1-train` at the pinned commit. It calls
the public sampler N = 100,000 times with two action channels for each of three seeds, counts sign agreements
of the standardized noise, and prints the frequency beside 0.665510 and beside ½ ± 3·(½/√N). It also prints
each channel's own positive fraction, to show that the marginals are correct. A second section recomputes the
phase step from the two constants in integer arithmetic and prints 0.167245.

**Proposed fix (carried by FD-FSIM-32; register priority P2).** Draw action channels independently: each
Box–Muller pair consumes two uniforms from distinct counters (or a counter-based generator keyed per channel),
and no draw is a function of a neighbouring draw's counter. Additive form: a corrected sampler beside the
present one, with the present one documented; whether the present one is corrected in place is FrankenSim's
decision, since its exported checkpoints were trained with it.

**Admission test and pass condition.** The joint-law battery (rule P-12, GATE-CORTEX-18), run in
`frob-conformance` against a sampler's manifest, with four statistics over N decisions: the sign-agreement
frequency of every pair of channels; the same for one channel at consecutive decisions; the same for one
channel in adjacent worlds; and the empirical covariance of the standardized noise. GATE-CORTEX-18's terms: for
action dimensions 2, 12 and 29, N = 10⁵ decisions and 64 worlds, every channel-pair, consecutive-decision and
adjacent-world agreement lies in ½ ± 0.005, every off-diagonal entry of the standardized covariance is below
4/√N = 0.01265 in magnitude, and the planted linear-phase sampler is rejected with an agreement of
0.6655 ± 0.005. Program-level signal threshold: a pairwise sign-agreement frequency outside ½ ± 0.005. Pass
conditions touching this report: the
planted copy of the sibling's sampler is rejected (GATE-WS-06, sampler row, at M2); this project's sampler
passes.

**The guard this repository keeps meanwhile.** `fs-g1-train` is not linked. A planted copy of its sampler (the
two constants and the Box–Muller cosine, about twenty lines, held in `frob-conformance` as a negative control
and labelled as such) is the battery's negative control. This project's own draws are counter-based outputs
keyed by `(WorldAddr, decision index, stream id, channel)`, and `train.start` refuses a sampler that has no
qualification receipt for the pinned kernel.

**Procedure.** (1) Copy the battery's recorded statistics for the planted sampler and the commit into the
report (`observed_at`). (2) Run the reproduction in a scratch clone at the pin. (3) If the frequency at the pin
is within ½ ± 0.005, the report says "not reproduced at `<rev>`" and FD-FSIM-32 is closed as not needed; the
planted copy stays as the negative control, since it is this project's own code. (4) File the report as the
mirrored bead of FD-FSIM-32 in FrankenSim's tracker under its `AGENTS.md`; store the identifier in the registry.

## Acceptance criteria

- [ ] `tests/upstream/FD-FSIM-32/REPORT.md` has the sections Defect, Evidence (file, lines, commit, the two constants), Mechanism with the arithmetic, Expected and observed, Proposed fix, Interim posture, and `observed_at`.
- [ ] The reproduction compiles against `fs-g1-train` alone at the pin and prints, per seed, the agreement frequency, each channel's positive fraction and the recomputed phase step 0.167245.
- [ ] The report is filed in FrankenSim's tracker; its identifier is the `mirror` of FD-FSIM-32's registry row.
- [ ] The planted sampler in `frob-conformance` is rejected by the battery in 100 of 100 runs, and no crate of the core workspace depends on `fs-g1-train`.

## Test plan

### Unit tests
- `phase_step_from_constants`: integer arithmetic on the two constants modulo 2⁶⁴; expected fraction 0.167245 to six places and predicted agreement 0.665510.
- `repro_sign_agreement_three_seeds`: N = 100,000 per seed; expected (if the defect is real) a frequency within 0.005 of 0.6655; expected for a correct sampler ½ ± 3·0.00158.
- `repro_marginals_are_fine`: each channel positive in ½ ± 0.005 of calls, mean within 0.01 of 0 and variance within 0.02 of 1 for the standardized noise.
- `planted_sampler_matches_sibling`: from one starting counter, the planted copy and the sibling's sampler produce bit-identical phase uniforms, and therefore identical sign patterns, over the first 1,000 two-channel draws (so that the negative control really has the sibling's joint law). The radius uniforms are not compared: the plant takes its radius from an independent addressed draw by design (N33c-37), so that only the joint law is defective.

### End-to-end script and logging
`scripts/e2e/defect_fdef_06.sh`: runs the reproduction in `target/upstream-scratch/frankensim@<rev>`; runs the
battery of `frob-conformance` on the planted sampler and on this project's sampler (`FROB_TEST_SEED=<seed> cargo
test -p frob-conformance --features gauntlet-mutations --test g1_known_answers -- GATE-CORTEX-18`; expected: the
plant rejected, with the offending pair and its frequency in the rejection record, and this project's sampler
qualified); reads the registry row with
`frob doctor --json`.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/defect_fdef_06/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FDEF-06`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged. The metric lines carry the agreement frequency, N, the seed and the
band ½ ± 0.005.

### Known answers and negative controls
- Known answer: the agreement probability 1 − φ/π = 0.665510 for φ = 0.334490·π; the reported counts 66,552, 66,552 and 66,551 of 100,000.
- Negative control: a sampler with independent channels (two uniforms from distinct counters per draw) must pass the same statistic; if the reproduction also reports 0.6655 for it, the counting is wrong.
- Metamorphic relation: reordering the channels of a product-law sampler leaves every statistic unchanged; for the sibling's sampler the agreement of channels k and k + 2 is 1 − 2φ/π = 0.331020, a second prediction the reproduction checks.
- Determinism lanes: counts are exact integers and identical on every run and platform for a fixed seed.

## Considerations

- RISK-58 (high): a sampler with correct marginals and a wrong joint law makes the likelihood ratio wrong and correlates exploration across channels; its planted instance is GATE-WS-07's M2 row (N41-38); earliest signals are a pairwise sign-agreement frequency outside ½ ± 0.005 and a learning curve that changes when channels are reordered.
- The fraction 0.6655 is arithmetic on constants read from source, so it survives a re-read better than a measured number; it must still be recorded at the pin before anything cites it.
- No milestone waits for the fix: nothing here links the sibling sampler. If it never lands, the sibling trainer simply never becomes a comparison baseline.
- When it lands it is adopted only through its switch-over rule (re-run the joint-law battery against the corrected sampler; only then may it serve as a comparison baseline); neither a pin nor the toolchain moves during the ten-fold tournament.

## References

Appendix F.2 (row 6), F.3; §33.17 (row 6), §23.25 (rule P-12), §23.6; GATE-CORTEX-18, GATE-WS-06; FD-FSIM-32; RISK-58, RISK-24.
