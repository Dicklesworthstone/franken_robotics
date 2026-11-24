---
key: "N28-KA"
title: "registry: the known answers of the design studio (KA-680 to KA-706), with tolerances, tiers and negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "atelier", "frob-conformance", "frob-analyze", "frob-opt", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "N38a-12", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N28ap1-02", "N28a-06", "N28a-07", "N28a-08", "N28a-09", "N28a-10", "N28a-11", "N28a-13", "N28a-16", "N28a-18", "N28a-19", "N28a-23", "N28a-27", "N28a-28", "N28b-02", "N28b-03", "N28b-14", "N28b-15", "N28b-16", "N28b-18", "N28b-19", "N28b-26", "N28b-27", "N28b-28", "N28b-34", "N28b-35", "N28b-40", "N28b-41", "N28b-42", "N38a-12"]
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

In: the 27 rows below as records of the suite's one known-answer file
`crates/frob-conformance/registry/known_answers.toml` (DESIGN correction: an earlier draft named a chapter file
`registries/known_answers/atelier.toml`, which N38a-12's one-file rule refuses with `ka-second-file`), with their
derivation files; the fixture files they need that no other bead builds; the test harness hooks that let a
mechanism bead bind a test to a row. Out: the mechanisms themselves (each row names the bead that implements what
it tests); the row schema, loader and eligibility rules (N38a-12); the `NC-` records of the stated controls,
derived by the seeding tool of N38b-05 from the control cells; the controls owed by the three rows that state
none (N38b-33 proposes them, and N28cp1-02 registers KA-703's); the counterexample registry of the chapter (key
`N28-CX`, another unit) and its refusal registry (`N28-REF`).

## Technical approach

**Row schema.** The record is N38a-12's: `id`, `section` (the "stated in" analysis or core, with `owner` = the
gate), `fixture`, `same_as`, `case`, `truth` (a value with units, or a verdict or label), `tolerance` (`exact` for
counts, rationals, verdicts and labels; `printed-digits` for "to the printed digits"; `abs` or `rel` for a
bound), `tier`, `sites`, `control` (`text` = the mutant and its expected failure; `nc` filled by the seeding
tool), `derivation`, `label`, `due` (the first milestone of the table), `printed`. The columns of an earlier draft
map as: `oracle` (`hand`, `enumeration`, `pilot:<bundle>`) → `label` (DESIGN for hand and enumeration, FACT for a
pilot) with the bundle named in the derivation file; `equivalent_to` → `same_as` with the earlier row's
`fixture` (the five ≡ rows below are the later rows, so each carries `same_as` = the earlier KA); `owner_bead` →
the derivation file's front matter, which lists for each KA the test that checks it and its owner bead. A row
whose oracle is a pilot stores the SHA-256 of the pilot bundle in its derivation file; this chapter's check
refuses a row whose bundle hash changed.

**Where stated and owner (the record's `section` and `owner`).** `section` is Appendix M.27's "Where stated",
each with §28.22 (the chapter's list) except KA-701 and KA-706, which M.27 states once: §28.4 for KA-680 to
KA-692; §28.6 for KA-693 and KA-694; §28.9 for KA-696; §28.11 for KA-697; §28.15 for KA-698; §28.13 for KA-699
to KA-701; §28.7 for KA-702; §28.19 for KA-695 and KA-703 to KA-705; §28.12 for KA-706. `owner` is the gate whose
text the row serves (the register, §28.23): GATE-ATEL-02 (analyses A-01 to A-14) for KA-680 to KA-682;
GATE-ATEL-05 (A-15 to A-26) for KA-683 to KA-691; GATE-ATEL-10 (A-19 by name) for KA-692; GATE-ATEL-06 (the
fixed-n planner) for KA-693 and KA-694; GATE-ATEL-08 (the calibrated layer) for KA-696; GATE-ATEL-16 (the cascade
and its stage controls) for KA-697 and KA-698; GATE-ATEL-11 (revalidation) for KA-699 to KA-701; GATE-ATEL-04
(bisect) for KA-702; GATE-ATEL-13 (decision cores DC-01, DC-09, DC-10, DC-11) for KA-695 and KA-703 to KA-705;
GATE-ATEL-09 (screening audits) for KA-706. `label` is FACT for the four pilot truths (KA-682, KA-686, KA-699 and
KA-701's 34,590), each with its bundle as source, and DESIGN for the rest.

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
| KA-690 | A-24: a task wrench outside the attainable set | A dual vector that separates it from every admissible actuation | exact (rational re-check) | G1 · host, auditor | An infeasible wrench reported as `not-synthesizable` with no witness | N28ap1-02 (A-24); gate row read by BD-348; M4 |
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

**Binding.** A row is bound to code by its runner entry `ka:<id>@<site>` in the tier target (N38a-13, rule 1),
entered by the owner bead with the function that computes the observed value; a unit test named `ka_<nnn>_…`
(STD-TEST) in the owning crate is a second, developer-level binding. Both compare through the test kit,
`testkit::assert_ka(&KnownAnswers::get("KA-693"), observed)`, which reads the truth and tolerance from the row, so
no test carries the value as a constant of its own, and which reports `obligation`, never `pass`, for a row whose
control is `none stated`. (DESIGN correction: an earlier draft defined a `known_answer!` macro of its own with
compile-time registration, a second binding mechanism beside the test kit's; it is withdrawn.) Rows are D0
(exact) data.

## Acceptance criteria

- [ ] All 27 rows KA-680 to KA-706 load from the registry; an unknown id or a changed oracle hash is a load error.
- [ ] Each row names its owner bead and milestone; `frob-conformance` lists, per milestone, the rows not yet bound
      to a passing test (the honest-status input).
- [ ] All 27 rows are records of `known_answers.toml`, valid under the loader of N38a-12; no chapter file exists.
- [ ] After the seeding tool of N38b-05 is re-run, the 24 rows with a stated negative control each have their
      `NC-` identifier in `control.nc`; the three rows with "none stated" (KA-694, KA-703, KA-706) carry that text
      and are reported `obligation: control` (never `eligible`, never exempted as "pure arithmetic") until the
      controls owed by N38b-33 (n + 1 in the exponent; first-come-first-served in place of shortest work; the
      bound taken as 1/m) and N28cp1-02's control for KA-703 are registered.
- [ ] The arithmetic rows this bead owns pass: 0.04573; index 38; 2,000.6, 621 and 0.2 s; 86 and 98; 1 and 0;
      0.0950 and 0.0495; 86,185 and 500,500.
- [ ] The five equivalences (KA-683 ≡ KA-113, KA-684 ≡ KA-154, KA-686 ≡ KA-610, KA-693 ≡ KA-612, KA-698 ≡ KA-127)
      are checked: both rows hold the same truth, and a divergence is a registry error.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ka_registry_atelier.rs`, run as `cargo test -p frob-conformance --test
ka_registry_atelier` (the file `tests/ka_atelier.rs` is the chapter battery's, created by N28cp1-01, which depends
on this bead; the registry's checks take a file and a target of their own):
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

`scripts/e2e/atelier_ka_registry.sh` (header `bead=N28-KA milestone=M1 labels=atelier,registry gates=`; the
script `atelier_known_answers.sh` is the chapter battery's, created by N28cp1-01 with its phases `m1`, `m3` and
`m4`, and this bead writes no line of it) runs `cargo test -p frob-conformance --test ka_registry_atelier` and
`cargo test -p frob-conformance --test registry_check` (there is no `frob conformance` command, ADR-GAUNT-01;
BD-025's lint refuses one in a script), asserts 27 rows for §28 with their eligibility, and prints the bound and
unbound rows per milestone as the tier targets report them (`FROB_TEST_SEED=<seed> cargo test -p frob-conformance
--test <tier> -- <row>`, `<tier>` = `g0_laws` or `g1_known_answers` as the row's tier says; a row is `unbound`
until its owner bead binds it, never passed, and the script does not fail on it, so this bead's close waits on no
owner bead). It sources the shared helper of STD-E2E and writes NDJSON to
`target/e2e-logs/atelier_ka_registry/<utc>.ndjson` with, per row, `ka`, `truth`, `measured`, `tolerance`,
`tier`, `negative_control` and `caught` (true, false or `not-registered`), and a `summary` line with counts.
Every line also carries `ts`, `script`, `bead`, `step` (`unit`, `list`, `row`), `event`, the exact command line,
exit code and wall time; on failure the first row whose measured value differs from its truth, with both values
and units, and the registry path and oracle hash needed to reproduce.

### Known answers and negative controls

The rows are the known answers. Registry-level controls: a row with its truth edited (the arithmetic rows'
recomputation and, for a pilot row, the bundle hash check catch it); a row due at M1 with no runner entry
(N38a-13 reports it `unbound`, which fails the gate); a mutant registered for a row whose test still passes under
the mutant (the run reports `control-missed` and the row counts as failed).
Determinism: the registry and its listing are byte-identical across runs and platforms.

## Considerations

- Several truths are FACTs from plan-time pilots (the stance enumeration, the wheel loop, the camera fixture, the
  docking count 34,590). Each is re-derived from its reproduction bundle before a gate cites it (seed beads
  BD-041, BD-042, BD-043 re-run the pilots at M0). The rows carry `label = "FACT"` with the bundle as source and
  N38a-12's note "to be re-run or re-read before a gate cites it"; a gate does not cite such a row until the
  re-run is on record (DESIGN correction: an earlier draft used a label `pilot-unverified`, which is not a value
  of the schema).
- KA-703 and KA-704 belong to cores another unit implements; they are registered here because this bead owns the
  chapter's registry, and their tests bind when those cores land.
- The tolerance "to the printed digits" means agreement after rounding to the digits shown; it is not a relative
  tolerance.
- A known answer scoped to a tier says nothing about another tier; KA-692 must hold on both lockstep and referee.

## References

§28.22, Appendix M.27 (KA-680 to KA-706); §38 (negative-control registry); GATE-ATEL-02, GATE-ATEL-05; BET-05.
