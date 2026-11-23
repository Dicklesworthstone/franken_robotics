---
key: "SPEC-C-30"
title: "spec C-30: requirements as temporal logic over unit-normalized predicates: contract, law tests, registry rows"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "solid", "arena", "frob-spec", "test"]
parent: "WP-41"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-122", "BD-194", "BD-255", "BD-257", "N10-CX", "SPEC-C-31", "N10c-10", "BD-121"]
gates: ["GATE-ARENA-03", "GATE-CORTEX-10"]
plan_refs: ["§10 C-30", "§24.5", "§24.6", "§38.7 MR-19 MR-20", "Appendix D.7", "Appendix I.8", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-30 of the FrankenRobotics spine: a requirement is a formula of signal temporal logic over typed
signals, and every atom is normalized by a declared scale of its own dimension, so the robustness of a formula is a
pure number that does not depend on the unit system. Tier Solid [S]. Owner: the Arena subsystem (tasks,
requirements, fixtures, qualification), crate `frob-spec`. No sibling library provides it (FACT, observed
2026-11-21).

It replaces success flags hand-coded per task and rewards standing in for requirements. Payoff: one artifact is
the acceptance predicate of a mission query (construction C-17), the falsification score of the rare-failure search
(C-27) and a runtime monitor (C-31), and mission verdicts stay separate from shaped rewards. Rejected alternatives:
(1) a minimum of raw margins, which changes its binding atom with the unit system (the counterexample below);
(2) evaluating "always" over whatever samples exist when an episode ends early, which reports a requirement as met
that was never tested; (3) using the requirement margin as the training reward by default: FACT (source register)
requirements do not replace shaped rewards in training, so that use exists only through a pilot with a kill gate.

Terms. *Signal*: a telemetry channel with a unit, a channel kind and a clock. *Robustness* or *margin* ρ: a
dimensionless number whose sign agrees with the Boolean value away from zero. *Binding atom*: the atom whose margin
equals the formula's margin; it is what an agent acts on. *Reach*: the sum of a formula's nested window ends.
*Tick*: an index on the requirement's clock, a rational period with a phase. Proof classes: Proved (sealed Lean
capsule), Certified (instance certificate re-checked by the offline auditor), Tested (property or known-answer test
with a negative control), Estimated. Status today: HYPOTHESIS.

## Scope

In: `docs/math/C-30.md`; law tests in `crates/frob-spec/tests/laws_c30.rs`; registry entries for KA-149 to KA-152,
relations MR-19 and MR-20, and the controls seeded from CX-072 and CX-073.

Out: the requirement language implementation and its three evaluators (BD-121); the task file and `task.check`,
`task.verdict` (BD-122); verdicts on batched tiers and `frob.verdict/1` on devices (BD-255); the reward-term pilot
and KILL-SPEC-REWARD (BD-257); the refusal codes of the language (owned by the Arena chapter); recording
GATE-ARENA-03 and GATE-CORTEX-10.

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-30: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`task.check`; also `task.verdict`), `failing_test`
(a test of `frob-spec::laws_c30` with its registered control), `payoff.gate` (GATE-ARENA-03), `statements`,
`known_answers` (KA-149 to KA-152), `cards` (none), `gates`, `pilot` left empty (the construction is Solid) and
`generic_comparator` only if this bead claims an advantage over a named generic method. Doctrine D-2 (no
ornamental mathematics) is checked on these fields: a construction with no verb or no failing test is moved to the
parked register by a decision record. The contract `docs/math/C-30.md` has the ten sections of the template
`docs/math/TEMPLATE.md` of the same bead, in the template's order (object and definitions; what it replaces and
the payoff; conventions; statements; formulas; fixtures and known answers; failure cases, refusals and retirement;
non-claims; foundations; implementation map); content that this bead lists under other headings goes into the
template section that matches. Statement identifiers are entered in the contract and in the register with the
construction prefix (`C-30.S1` and so on, as the template requires); the tables of this bead and of its part beads
abbreviate them.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-121 (the requirement
language core, which creates `frob-spec`, implements this construction and depends on this bead): that bead is
listed in `related` and is not a prerequisite. The law suite is generic over its subject trait and closes on the
exact reference model and the mutants in its own test support. If `frob-spec` does not exist when this bead
starts, the suite and its support files are written under `crates/frob-conformance/tests/` with the same file
names (that crate exists from BD-025) and the commands below read `-p frob-conformance`; the bead that implements
the construction moves them into the owning crate, binds its production types to the subject trait and runs the
unchanged suite as part of its own acceptance. A criterion or script step below that names production code or a
verb of an implementing bead is owed at that bead's acceptance and is logged `pending` here until then.

## Technical approach

### 1. Definitions and semantics (contract sections 1 and 2)

An atom is ψ(x) ≥ 0 (or ψ(x) > 0), where ψ has a physical dimension and a declared positive scale s of the same
dimension. In the language it is a comparison of two expressions of one dimension with a scale; an atom over an
integer-valued signal is declared exact and has margin +1 or −1. Margins at tick t:

| Formula at tick t | Margin ρ |
|---|---|
| Atom with expression e ≥ 0 or e > 0 and scale s | e(t)/s |
| ¬φ | −ρ(φ, t) |
| φ ∧ ψ ; φ ∨ ψ | min ; max of the two margins |
| G_[a,b] φ (always) ; F_[a,b] φ (eventually) | min ; max of ρ(φ, t′) over t′ in [t + a, t + b] |
| φ U_[a,b] ψ (until) | max over t′ in [t + a, t + b] of min( ρ(ψ, t′), min of ρ(φ, t″) over t″ in [t, t′) ) |
| H_[a,b] φ (historically) ; O_[a,b] φ (once) | min ; max of ρ(φ, t′) over t′ in [t − b, t − a] |
| φ S_[a,b] ψ (since) | max over t′ in [t − b, t − a] of min( ρ(ψ, t′), min of ρ(φ, t″) over t″ in (t′, t] ) |

A window [a, b] denotes the ticks j with a ≤ j·T ≤ b for period T, compared in exact rational arithmetic.

| Id | Statement | Class |
|---|---|---|
| S1 | Soundness: ρ > 0 implies the formula holds; ρ < 0 implies it fails; ρ = 0 decides nothing, and the atom's declared comparator (strict, non-strict or equality) decides, in exact arithmetic where the inputs are exact. A uniform positive margin is a separate, stronger request. | Tested (property over random formulas and traces) |
| S2 | Unit invariance: because ρ = ψ/s with s of the dimension of ψ, a change of units leaves every normalized margin and the binding atom unchanged. | Tested (exact rationals) |
| S3 | The past-time operators H, O and S depend only on the past and are evaluated online by a streaming monotonic deque in amortized constant time with at most three comparisons per sample per operator node, so they may feed rewards and runtime monitors. | Tested (equality with brute force; comparison count) |
| S4 | Future-time operators yield verdicts offline over completed traces. A formula with future horizon b is decided at delay b. A window that the trace does not cover yields `undetermined`, never a truncated minimum. | Tested |
| S5 | A bounded-future formula is rewritten as a past-time formula evaluated at its horizon: G_[0,T] φ at time 0 equals H_[0,T] φ at time T; likewise F_[0,T] φ at 0 equals O_[0,T] φ at T, and G_[a,b] φ at t equals H_[0,b−a] φ at t + b. | Tested (bit equality) |
| S6 | Minimum, maximum and negation are exact in floating point, so every evaluator returns identical bits for identical atom values, after negative zero is canonicalized; a NaN marks the world `Faulted`. | Tested |
| S7 | A verdict already fixed by a prefix (an always-clause violated, an until-clause met) is reported as decided at the tick that fixed it. | Tested |

### 2. Algorithms the contract fixes

Streaming sliding minimum over a window of W ticks (maximum is symmetric), monotonic-deque form:
1. Keep a deque of (tick, value) pairs whose values increase from front to back.
2. On a new sample (i, v): pop from the back while the back value is at least v; push (i, v).
3. Pop from the front while the front tick is older than i − W + 1.
4. The window minimum is the front value. Each sample is pushed once and popped at most once.

Block-decomposition form for kernels with no data-dependent control flow: per block of W samples keep the prefix
extremum from the block start and the suffix extremum to the block end; the window value is the extremum of one
suffix and one prefix value. Memory is 2·W values per operator node and world, with at most three comparisons per
sample. The two forms compute the same function and must return identical bits.

Offline evaluation: a two-pass evaluator on the completed trace; the reach is computed statically.

### 3. Conventions, failure cases and non-claims

- Verdict values: `satisfied`, `violated`, `undetermined`, `invalid` (a numerical fault). The verdict is a pure
  function of the trace and the task identity.
- Memory: a 600 s future window at 50 Hz needs 30,000 samples. The online window capacity is 4,096 requirement
  ticks (TARGET of the owning chapter); a longer window is offline only.
- Margins are comparable across atoms only through their declared scales, which are judgments recorded in the
  task file. Requirements are the acceptance metric and a falsification score; they are not claimed as a
  replacement for shaped rewards.
- A verdict on a sampled trace says nothing about the time between samples.
- Refusals the contract relies on (codes owned by the Arena chapter): an atom with no scale or mixed dimensions; a
  missing reduction for a faster clock (FROB-REF-525); a window bound that is not a multiple of the period without
  declared rounding (FROB-REF-538); an episode horizon shorter than the reach (FROB-REF-526); an online window
  above capacity (FROB-REF-528).
- Type sketch (DESIGN): `Atom { id, cmp: Gt | Ge | Lt | Le | Eq, lhs, rhs, scale: Qty | Exact }`; `Past { Atom,
  And, Or, Not, Historically, Once, Since }`; `Future { Past, And, Or, Not, Always, Eventually, Until }`;
  `Requirement { clock, atoms, online: Vec<(Ident, Past)>, verdict: (Ident, Future), examples }`;
  `trait Monitor { fn margin(&self, tick) -> Margin; fn decided_at(&self) -> Option<Tick>; }`.

### 4. Law-test layout and registry

`crates/frob-spec/tests/laws_c30.rs`; reference model in `tests/support/c30_ref.rs`: the margin table evaluated by
brute force over explicit tick ranges, in exact rationals for the unit laws and in `f64` for the bit-equality laws.
The suite is generic over a trait `C30Subject` (evaluate a formula at a tick; stream a past-time formula; report
the verdict and the binding atom) and binds the three production evaluators of `frob-spec` (offline two-pass,
monotonic deque, block decomposition). Mutants in `tests/support/c30_mutants.rs`. Registry: KA-149 to KA-152 with
derivation files; MR-19 (a change of units leaves normalized robustness and the binding atom unchanged) and MR-20
(streaming past-time robustness equals offline evaluation; G over [0, T] at 0 equals H over [0, T] at T) in the
relation registry; controls seeded from CX-072 and CX-073.

### 5. Implementers and gates

Implementer: `frob-spec` (BD-121, BD-122), with kernels for online monitors on batched tiers (BD-255). Gates:
GATE-ARENA-03 at M1 (the three evaluators agree bit for bit on 10⁶ random traces per operator, including multi-rate
signals and windows with rounding; every requirement-language refusal is triggered by its planted task file) and
GATE-CORTEX-10 at M3 (streaming past-time robustness equals offline evaluation on 10⁴ random traces; the mixed-unit
fixture binds on the clearance in both unit systems; a future-time online reward is refused). Kill gate
KILL-SPEC-REWARD (M3) concerns only the reward use: if the arm with the added requirement term does not improve
success over the shaped reward across five seeds, requirements stay acceptance metrics and search scores only.

## Acceptance criteria

- [ ] `docs/math/C-30.md` exists with the margin table, S1 to S7, both streaming algorithms, the conventions, the
      refusals relied on and the non-claims; every symbol is defined.
- [ ] `cargo test -p frob-spec --test laws_c30` passes on the reference model and on each of the three production
      evaluators.
- [ ] Every mutant below fails its named law.
- [ ] KA-149 to KA-152 are registered with derivation files; KA-150 and KA-152, printed with no control, have the
      controls named below.
- [ ] MR-19 and MR-20 are registered; controls from CX-072 and CX-073 are attached to GATE-ARENA-03 and
      GATE-CORTEX-10.
- [ ] `scripts/e2e/spec_c30.sh` exits 0.
- [ ] Row C-30 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `ka_149_unit_normalization`: clearance 0.02 m and thermal margin 15 K with scales 0.10 m and 10 K give 0.2 and
  1.5; the binding atom is the clearance in metres and in millimetres. The raw minimum is 0.02 in (m, K) and 15 in
  (mm, K): its binding atom changes.
- `unit_invariance_property`: 10⁴ seeded conjunctions and disjunctions of atoms under random rescalings by
  powers of ten, in rationals; margins and binding atom unchanged.
- `ka_150_streaming_equals_bruteforce`: 10⁴ seeded random signals, lengths up to 4,096, W in {1, 2, 3, 7, 64,
  1000}, with ties and negative zeros; deque output equals the brute-force window minimum bit for bit; comparisons
  per sample at most 3 amortized.
- `block_equals_deque`: the block-decomposition form equals the deque form bit for bit and uses 2·W values.
- `ka_151_truncation_undetermined`: G_[0, 45 s] on an all-positive trace ending at 30 s returns `undetermined`
  with the reason that the window is not covered.
- `ka_152_bounded_future_rewrite`: on 10⁴ random traces G_[0,T] φ at 0 equals H_[0,T] φ at T; F and O likewise;
  G_[a,b] φ at t equals H_[0,b−a] φ at t + b.
- `soundness_signs`: 10⁴ random formulas of depth at most 4: positive margin implies true, negative implies false.
- `comparator_decides_at_zero`: an atom with value exactly 0 is true under ≥ and false under >, margin 0 in both.
- `until_since_hand_trace`: φ = (1, 1, −1, 1), ψ = (−1, −1, 2, −1) at ticks 0 to 3: φ U_[0,3] ψ at 0 has margin
  1; the mirrored trace gives the same for S.
- `negation_duality`: ρ(¬G φ) equals ρ(F ¬φ) bit for bit.
- `exact_atom_margin`: an integer-valued atom has margin +1 or −1.
- `memory_arithmetic`: 600 s at 50 Hz is 30,000 samples.
- `decided_at_prefix`: an always-clause first violated at tick 8 reports `decided_at` 8.
- `nan_marks_faulted` and `negative_zero_canonical`.

### End-to-end script and logging

`scripts/e2e/spec_c30.sh`: `frob --json doctor`; `cargo test -p frob-spec --test laws_c30`; `cargo test -p
frob-conformance --test g1 -- KA-149 KA-150 KA-151`; `cargo test -p frob-conformance --test g3 -- KA-152 MR-19
MR-20`; file and registry checks. Log to `target/e2e-logs/spec_c30/<utc>.ndjson` with a transcript; each line has
`ts`, `script`, `bead`, `step`, `event`, command line, exit code, wall time, the evidence block (claim Known
answer, class Tested, tier, determinism D0 for rational rows and bit equality for `f64` rows, seed), measured and
expected margins with their binding atoms, and on failure the first differing tick and value with the trace file
path. The `summary` line counts assertions and names GATE-ARENA-03 and GATE-CORTEX-10.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-149 | Normalized values 0.2 and 1.5; clearance binds in every unit system | exact | G1 host | Mutant `raw_minimum`: 0.02 in (m, K), 15 in (mm, K) |
| KA-150 | The deque equals the brute-force window minimum on random signals | exact | G1 host | Mutant `deque_without_expiry` (new): step 3 omitted |
| KA-151 | An episode ending at 30 s under G_[0, 45 s] is `undetermined` | exact (verdict) | G1 host | Mutant `truncated_minimum`; mutant `pads_short_window` |
| KA-152 | G_[0,T] φ at 0 equals H_[0,T] φ at T | exact | G3 host | Mutant `off_by_one_window` (new): H over [1, T] |

Counterexamples carried: CX-072 (the robustness of a conjunction is not the minimum of raw margins), CX-073 (an
early-ended episode is not evaluated over the samples that exist). The owning chapter states the same two cases
with other numbers (clearance 0.02 m and thermal margin 3 K: raw minimum 0.02, in millimetres 3, normalized 0.2 and
0.3); both fixtures run. Determinism lanes: twice on one machine; debug and release; the production binding repeats
the bit-equality laws on each backend.

## Considerations

- Order against BD-121 (settled): BD-121 implements this construction and creates `frob-spec`, so it depends on
  this bead and this bead does not depend on it. The reference model and the suite are written first and are
  bound by BD-121 when the crate exists, as the Scope states.
- Guard band: the owning chapter decides atoms on a device only outside a band of 2⁻²¹ around zero and re-evaluates
  inside it on the CPU with the declared comparator. FACT (planning archive, to be re-run): an unguarded
  single-precision strict comparison on a 65,536-case boundary screen gave 452 false admissions and 23,438 false
  exclusions; with the guard and an exact fallback there were none. S1 is the mathematical reason the band exists.
- Open question OPEN-AUGUR-1 (whether mission predicates are written in this language or reference task terms by
  identifier; default: reference by identifier, one grammar) is owned by the Arena lead and does not block this
  bead.
- Scales are judgments; the contract requires that they be recorded, not that they be right.

## References

Plan §10 C-30; §24.5, §24.6; Appendix D.7 "Requirement robustness"; Appendix I.8; MR-19, MR-20; KA-149 to KA-152;
CX-072, CX-073; GATE-ARENA-03, GATE-CORTEX-10; PO-ARENA-04, INV-ARENA-01, INV-ARENA-02; KILL-SPEC-REWARD; BET-24;
C-17, C-27, C-31.
