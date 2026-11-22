---
key: "N28-KA"
title: "registry: the known answers of the design studio (KA-680 to KA-706), with tolerances, tiers and negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "atelier", "frob-conformance", "frob-analyze", "frob-opt", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: ["§38: the known-answer registry file format of `frob-conformance` and its loader, if it differs from the one BD-025 creates"]
related: ["BD-146", "BD-318", "BD-377", "BD-042", "BD-041", "BD-043"]
gates: ["GATE-ATEL-02", "GATE-ATEL-05"]
plan_refs: ["§28.22", "Appendix M (M.27)", "§28.4", "§28.6", "§28.7", "§28.9", "§28.11", "§28.12", "§28.13", "§28.15", "§28.19", "§38"]
---
## Background

A known answer is a case whose truth is fixed independently of the code under test: by hand arithmetic, by exact
enumeration, or by a pilot executed before any implementation and filed with its reproduction bundle. The design
studio (the subsystem called Atelier: the loop in which a design is checked, given a controller, simulated,
explained, edited and compared, in the crates `frob-analyze` and `frob-opt`) has 27 such cases. This bead puts all
of them into the conformance registry as data, each with its tolerance, the verification tier it runs at, and the
negative control that must fail, so that every mechanism bead of the studio cites a registered row instead of
restating a number that could drift.

Why a registry and not numbers scattered in tests: the project's rule is that a known-answer test passes against a
pinned oracle that was never regenerated from the candidate, and that a gate nothing can fail validates nothing.
A registry row carries its provenance (hand derivation, exact enumeration, or plan-time pilot) and its planted
failure, and the exit review of a milestone reads the registry to confirm that every gate has a caught negative
control. Rejected alternative: inline constants per test (the same case appears in three chapters under three
numbers, and a regenerated constant silently becomes the oracle).

Verification tiers used below: G0 pure arithmetic or table checks; G1 known answer against a pinned oracle. "host"
means the CPU with exact or double arithmetic; "referee" the high-fidelity CPU physics tier; "lockstep" the batched
tiers; "auditor" the offline certificate checker.

## Scope

In: the 27 rows below as registry entries (`registries/known_answers/atelier.toml`, DESIGN path), the fixture
files they need that no other bead builds, the test harness hooks that let a mechanism bead bind a test to a row,
and the negative-control entries. Out: the mechanisms themselves (each row names the bead that implements what it
tests); the counterexample registry of the chapter (key `N28-CX`, another unit) and its refusal registry
(`N28-REF`).

## Technical approach

**Row schema (DESIGN).** `id`, `stated_in`, `case`, `truth` (a value with units, or a verdict or label),
`tolerance` (`exact`, `exact-rational`, `printed-digits`, or a bound), `tier`, `oracle` (`hand`, `enumeration`,
`pilot:<bundle>`), `negative_control` (a mutant identifier and the expected failure), `equivalent_to` (a row of
another chapter that states the same case), `owner_bead`, `first_milestone`. A row whose oracle is a pilot stores
the SHA-256 of the pilot bundle; the registry loader refuses a row whose oracle hash changed.

**The rows.** "≡" marks a row that another chapter's registry also carries; both must agree, and a test binds to
either.

| KA | Case | Truth | Tolerance | Tier | Negative control that must fail | Owner bead, milestone |
|---|---|---|---|---|---|---|
| KA-680 | A-03 symmetry: a quadruped built symmetric fore-aft and left-right, with one forward lidar | Four-element mechanical group C₂ × C₂; two-element observation-preserving subgroup C₂; breaker = the lidar mount | exact (group) | G1 · host | A policy tied under the mechanical group is refused | BD-102 (symmetry); checked from the studio by N28a-11; M1 |
| KA-681 | A-07, A-09: a mecanum base with misaligned (all-parallel) rollers | Curvature of the connection is full rank (spanning); the growth vector shows a rank drop: (2, 3) where (3) is required | exact (rank) | G1 · host | A curvature-only check passes the defective base | BD-106; M1 |
| KA-682 | A-08: stance enumeration over 2,304 bodies × 5 load states | 11,520 of 11,520 classifications agree with the exact classifier; equal passive springs at half-width 1/4 m give (175, 115, 85, 25) N | exact | G1 · host | The support-polygon-plus-total-capacity screen accepts 1,596 bodies the exact classifier rejects | N28a-06; M1 |
| KA-683 ≡ KA-113 | A-17: seven readings, each right with probability 0.75, majority vote | 3807/4096 ≈ 0.929 when errors are independent; 0.75 with a fully shared error | exact (rational) | G1 · host | A redundancy score computed from per-reading accuracy | BD-120; N28a-11; M1 |
| KA-684 ≡ KA-154 | A-20: 8 kg body, 12 mm rise, 0.90 J budget | 8 × 9.81 × 0.012 = 0.94176 J excludes the passive crossing; 0.05 J of approach kinetic energy removes the exclusion | exact to the printed digits | G1 · host | An exclusion issued with approach energy unbound | N28a-07; M1 |
| KA-685 | A-21: a packet arriving at tick 1 serves decisions committing at ticks 2 and 4; arrival delayed to tick 2, then to tick 3 | The answer changes only at the second delay (tick 3) and only for the first decision | exact | G1 · host | A check that compares content and ignores arrival | N28a-08; M1 |
| KA-686 ≡ KA-610 | A-22: 75 mm wheel, total reflected inertia 0.00107890625 kg·m², current target held for 10 ms | One-tick spectral radius 1.4820 with target-current feedforward, 0.9207 with measured-current feedforward | to the printed digits | G1 · host | The instantaneous-current model gives 2.6917 for both and misses the stable case | N28a-09; M1 |
| KA-687 | A-23: two fingers on one bus, each needing one powered tick, the bus powering one finger per tick | Zero joint schedules at a one-tick deadline; exactly two at a two-tick deadline | exact (count) | G1 · host | The additive-peak rule rejects the feasible two-tick design | N28a-10; M1 |
| KA-688 | A-25: drawbar force F = min(k_t·I/r, μ·N), k_t = 0.2 N·m/A, r = 0.06 m | At 1 A and 2 A under 30 N and 40 N: 3.33 N and 6.67 N for μ = 0.4 and for μ = 0.8, so zero sensitivity to μ; at 3 A under 10 N: 4 N against 8 N | to the printed digits | G1 · host | A fit that reports μ from the first two runs | N28a-13; M4 |
| KA-689 | A-18: a doorway narrower than every orientation of the footprint; the same doorway widened past the footprint | An impossibility cut that names the doorway; a witness path when widened | exact (verdict) | G1 · host | A clearance test on a bounding ball for a rectangular footprint | BD-233 (clearance complexes); gate row read by BD-348; M3 |
| KA-690 | A-24: a task wrench outside the attainable set | A dual vector that separates it from every admissible actuation | exact (rational re-check) | G1 · host, auditor | An infeasible wrench reported as `not-synthesizable` with no witness | BD-348; M4 |
| KA-691 | A-26: a recorded run with one planted contact-mode switch | Two regions, with the boundary at the switch tick | exact | G1 · referee | A gradient reported as trustworthy across the switch | N28a-16; M4 |
| KA-692 | A-19: a finalist whose advantage comes from diagonal traction on a pyramidal friction cone (up to √2·μ) | Typed `model-conditional` | exact (label) | G1 · lockstep, referee | The same campaign with the rank-robustness stage skipped reports it as the winner | BD-346; exercised by BD-352; M4 |
| KA-693 ≡ KA-612 | Fixed-n rule at n = 64, margin 0.05, two-sided 97.5% Clopper–Pearson on each discordant probability | 2,145 count pairs: 521 prefer, 521 inferior, 1,103 inconclusive, 0 ties; the first tie at n = 86 (only for (0, 0), among 3,828 pairs: 1,031, 1,031, 1, 1,765) | exact | G0 · host | A study that declares "tie" reachable at n = 64 | BD-124; N28a-19; M1 |
| KA-694 | Zero discordances in 64 pairs | One-sided 95% bound 1 − 0.05^(1/64) = 0.04573 | to the printed digits | G0 · host | none stated | N28a-19; M1 |
| KA-695 | DC-01 promotion table at p₀ = 0.05, p₁ = 0.30, α⁺ = 0.01 | 16 clean pairs promote, or 22 pairs with one exceedance | exact | G0 · host | An envelope changed after the first outcome | BD-263 / BD-309; cited by BD-349; M3 |
| KA-696 | Conformal margin at n = 39, α = 0.05 | The 38th smallest residual (index ⌈40 × 0.95⌉ = 38) | exact (index) | G0 · host | Calibration from survivors only | BD-329; M4 |
| KA-697 | Cascade stage 1: ten leg types on six slots, dihedral symmetry of order 12; then only a sagittal mirror | (10⁶ + 2·10 + 2·10² + 10³ + 3·10⁴ + 3·10³)/12 = 86,185 representatives; (10⁶ + 10³)/2 = 500,500 | exact (count) | G1 · host | Deduplication under the mechanical group when a sensor breaks it | BD-285; exercised by BD-352; M4 |
| KA-698 ≡ KA-127 | Deduplication digest on the ballast family: eight slugs in 24 collinear slots | C(24, 8) = 735,471 placements give 24,546 classes | exact (count) | G1 · host | Two placements with equal first moment and different second moment merged | BD-285; M3 |
| KA-699 | Revalidation, four-state camera fixture: decisions commit at ticks 2 and 4, packet at tick 1; states: initial; a display label renamed; delivery delayed to tick 2; delivery delayed to tick 3 | 2, 0, 0, 1 scene-pair evaluations and 1, 0, 0, 1 verdict recomputations, each verdict equal to a fresh replay | exact | G1 · host | The key without delivery time keeps a stale promotion | BD-292; re-run by BD-347; M3 |
| KA-700 | Revalidation, widening the doorway: a cost-9 cart certified minimal while a cost-5 relay is excluded by the doorway | The cart stays feasible; its optimality is revoked | exact (verdict) | G1 · host | An optimum kept after a cheaper rival became feasible | BD-292; BD-347; M3 |
| KA-701 | Revalidation, enlarging the hypothesis support of the docking fixture (gains {1, 2}) by gain 33/32 | Indistinguishable from gain 1 at error radius 1/64; every one of the 34,590 positive decisions on each test grid is withdrawn | exact (count) | G1 · host | A hash match taken as verdict transfer | BD-347; N28a-23; M4 |
| KA-702 | Bisect: outcomes along six versions are pass, fail, pass, pass, pass, fail | An `a-change` at the last edit; no `first-change` without a scan of the prefix | exact (label) | G1 · host | `first-change` reported at the last edit | BD-343; M4 |
| KA-703 | DC-09 worked queue: one device at utilization 0.7; 90% of jobs take 10 s, 10% take 30 min | Waits of about 2,000 s first-come-first-served (λ·E[S²]/(2(1 − ρ)) = 2,000.6 s), 621 s with non-preemptive priority, 0.2 s plus one quantum with preempt-resume | to the printed digits | G1 · host | none stated | BD-309 (other unit); M3 |
| KA-704 | DC-10 coverage certificate: 40 modes of total weight 100; greedy gains 31, 22, 14, 9, 6, 4 | Six gains cover 86 of 100; the six largest remaining gains sum to 12, so the bound is 98 | exact | G1 · host | A certificate computed from stale gains | BD-309 (other unit); M3 |
| KA-705 | DC-11, `f32` regrouping with a = 10⁸, b = −10⁸, c = 1 | (a + b) + c = 1 and a + (b + c) = 0 | exact | G0 · host | A reassociation admitted in the bit-exact rewrite class | N28a-28; M4 |
| KA-706 | Screening-audit bound 1 − δ^(1/m) | No top-k design among 30 audited discards bounds the false-discard rate at 9.5% with 95% confidence (1 − 0.05^(1/30) = 0.0950); 59 audited discards bound it at 5.0% (0.0495) | to the printed digits | G0 · host | none stated | BD-345; cited by BD-352; M4 |

**Fixtures this bead builds** (the others come from the owner beads): the six-version bisect lineage of KA-702 as
a `synthetic` lineage with scripted outcomes; the arithmetic rows KA-694, KA-696, KA-703, KA-704, KA-705, KA-706
as pure functions with their expected values; the two-finger bus instance of KA-687 as a bus-contract file.

**Binding.** A test binds with `known_answer!("KA-693")` (DESIGN macro in `frob-conformance`), which records the
row id, the measured value and the comparison under the row's tolerance in the bead log; a test that asserts a
value not equal to the registry's fails at compile-time registration. Rows are D0 (exact) data.

## Acceptance criteria

- [ ] All 27 rows KA-680 to KA-706 load from the registry; an unknown id or a changed oracle hash is a load error.
- [ ] Each row names its owner bead and milestone; `frob-conformance` lists, per milestone, the rows not yet bound
      to a passing test (the honest-status input).
- [ ] The 24 rows with a stated negative control each have a registered mutant; the three rows with "none stated"
      (KA-694, KA-703, KA-706) are marked `no-control` with the reason "pure arithmetic".
- [ ] The arithmetic rows this bead owns pass: 0.04573; index 38; 2,000.6, 621 and 0.2 s; 86 and 98; 1 and 0;
      0.0950 and 0.0495; 86,185 and 500,500.
- [ ] The five equivalences (KA-683 ≡ KA-113, KA-684 ≡ KA-154, KA-686 ≡ KA-610, KA-693 ≡ KA-612, KA-698 ≡ KA-127)
      are checked: both rows hold the same truth, and a divergence is a registry error.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ka_atelier.rs`:
- `ka_rows_load_27`: exactly 27 rows with ids KA-680..KA-706, no gaps.
- `ka_694_zero_discordance_bound`: 1 − 0.05^(1/64) rounds to 0.04573.
- `ka_696_conformal_index`: ⌈(39 + 1)·0.95⌉ = 38; with n = 18 the index 19 exceeds n and the function returns
  `insufficient` (n ≥ 19 is required at α = 0.05).
- `ka_697_burnside`: both counts by the cycle-index formula and, for three leg types, by brute force (92).
- `ka_703_queue`: with λ = 0.7/189 s⁻¹, E[S] = 189 s, E[S²] = 324,090 s²: 2,000.6 s; priority wait
  λ·E[S²]/2 ÷ (1 − ρ₁) with ρ₁ = λ·9 = 0.0333: 620.9 s.
- `ka_704_greedy_bound`: 31 + 22 + 14 + 9 + 6 + 4 = 86; bound 86 + 12 = 98; ratio 86/98 ≥ 0.877.
- `ka_705_f32_regroup`: the two groupings in `f32` give exactly 1.0 and 0.0.
- `ka_706_audit_bound`: 0.0950 at m = 30, 0.0495 at m = 59, δ = 0.05.
- `equivalent_rows_agree`: the five ≡ pairs.
- `changed_oracle_hash_refused` (negative control on the registry itself).

### End-to-end script and logging

`scripts/e2e/atelier_known_answers.sh` runs `cargo test -p frob-conformance ka_atelier` and
`frob conformance list --chapter 28 --json`, asserts 27 rows, and prints the bound and unbound rows per
milestone. It sources the shared helper of STD-E2E and writes NDJSON to
`target/e2e-logs/atelier_known_answers/<utc>.ndjson` with, per row, `ka`, `truth`, `measured`, `tolerance`,
`tier`, `negative_control` and `caught` (true, false or `not-registered`), and a `summary` line with counts.

### Known answers and negative controls

The rows are the known answers. Registry-level controls: a row with its truth edited (the loader's hash check
catches it); a test bound to a row but asserting a different constant (registration fails); a mutant registered
for a row whose test still passes under the mutant (the conformance run reports `control-not-caught` and fails).
Determinism: the registry and its listing are byte-identical across runs and platforms.

## Considerations

- Several truths are FACTs from plan-time pilots (the stance enumeration, the wheel loop, the camera fixture, the
  docking count 34,590). Each is re-derived from its reproduction bundle before a gate cites it (seed beads
  BD-041, BD-042, BD-043 re-run the pilots at M0); until a re-run is on record the row is labelled
  `pilot-unverified` and a gate that reads it reports that label.
- KA-703 and KA-704 belong to cores another unit implements; they are registered here because this bead owns the
  chapter's registry, and their tests bind when those cores land.
- The tolerance "to the printed digits" means agreement after rounding to the digits shown; it is not a relative
  tolerance.
- A known answer scoped to a tier says nothing about another tier; KA-692 must hold on both lockstep and referee.

## References

§28.22, Appendix M.27 (KA-680 to KA-706); §38 (negative-control registry); GATE-ATEL-02, GATE-ATEL-05; BET-05.
