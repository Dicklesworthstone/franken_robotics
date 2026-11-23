---
key: "SPEC-C-35"
title: "spec C-35 core: structure and life, and topology optimization of a link from recorded load envelopes"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "spine", "spec", "frontier", "foundry", "frob-foundry", "test"]
parent: "WP-70"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-401", "BD-402", "BD-403", "BD-404", "BD-406", "N10c-06", "N10c-07", "N10-CX", "N10c-10", "BD-400"]
gates: ["GATE-FOUNDRY-04", "GATE-FOUNDRY-05"]
plan_refs: ["§10 C-35", "§31.5", "§31.6", "§31.7", "§31.9", "Appendix D.10", "Appendix M.9"]
---
## Background

Construction C-35 of the FrankenRobotics spine, "structure and life: topology, fatigue, tolerance", is three
computations that turn simulated loads into a part: (1) topology optimization of a link; (2) cumulative fatigue
damage; (3) tolerance allocation. Tier Frontier [F]. Owner: the Foundry subsystem (structure, manufacture,
lifecycle), crate `frob-foundry`. It replaces links modelled as boxes with a density and safety factors chosen by
habit. The payoff is a build packet whose parts are sized by the loads the robot met on its missions, with the
life and the tolerance cost stated.

This bead is the core specification: the common frame of the three computations and the whole of computation 1.
Fatigue is bead N10c-06 and tolerance allocation is bead N10c-07; both extend the contract file created here.

What this construction is not: a source of speed. FACT (source register, to be re-read): topology optimization
makes links 7 to 30% lighter than engineered parts, which is about 2 to 8% of robot mass on the one quadruped
with a published mass breakdown; it improves the output and not the time to a decision, so it is not counted in
the ten-fold thesis. The decisive design choice is where the loads come from: load cases are the envelope of
interface wrenches recorded by the energy ledger over qualified missions, not an assumed static load. A structural
result holds for the load envelope it was given, and an edit that changes the envelope invalidates it: reuse
across an edit is licensed by a recorded implication, never by a matching hash. Rejected alternative: three
dimensional optimization for every design in a campaign; it is for finalists only, with a beam-sizing rung as the
engineered baseline and the fallback.

Foundations (FACT, FrankenSim `483d09c682f2`, observed 2026-11-21): `fs-topopt` (about 69k lines) and `fs-topols`;
the rainflow counter `RainflowMiner` of `fs-material`, which `fs-solid::fatigue` imports; `fs-toleralloc` (about
9.6k lines). A foundation that exists is not a foundation that is admitted; admission is a separate bead.

Terms. *Density* ρ_e: a design variable per mesh element in [ρ_min, 1]. *Compliance* c = FᵀU: twice the strain
energy under load F with displacement U; smaller is stiffer. *Load envelope*: the set of interface wrench cases
extracted from telemetry. *Interface wrench*: the wrench a link exchanges at a port. Proof classes: Proved,
Certified, Tested, Estimated; status HYPOTHESIS. Every structural number produced by the mechanism is Estimated
and enters a claim product as its weakest input.

## Scope

In: `docs/math/C-35.md` part A (frame) and part B (topology optimization); law tests in
`crates/frob-foundry/tests/laws_c35_topology.rs`; registry entry for KA-168 and the pilot record of the whole
construction.

Out: load-case extraction (`foundry.loads`, BD-400); beam-rung sizing and the structural margin (BD-401); the
topology optimization mechanism and `foundry.optimize` (BD-403); inertia fed back into the model with its
convergence test (BD-404); fatigue (N10c-06, BD-402); tolerance (N10c-07, BD-406); recording GATE-FOUNDRY-04 and
-05 and deciding KILL-TOPOPT (BD-403).

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-35: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`foundry.optimize`; also `foundry.check`,
`foundry.tolerance`), `failing_test` (a test of `frob-foundry::laws_c35_topology` with its registered control),
`payoff.gate` (GATE-FOUNDRY-04), `statements`, `known_answers` (KA-168 to KA-171), `cards` (none), `gates`,
`pilot` (the record `PILOT-C-35` of the pilot register, required because the tier is Frontier) and
`generic_comparator` (the off-the-shelf method named in the pilot paragraph of this bead). Doctrine D-2 (no
ornamental mathematics) is checked on these fields: a construction with no verb or no failing test is moved to the
parked register by a decision record. The part beads N10c-06 and N10c-07 add their statements and tests to the
same row. The contract `docs/math/C-35.md` has the ten sections of the template `docs/math/TEMPLATE.md` of the
same bead, in the template's order (object and definitions; what it replaces and the payoff; conventions;
statements; formulas; fixtures and known answers; failure cases, refusals and retirement; non-claims; foundations;
implementation map); content that this bead lists under other headings goes into the template section that
matches. Statement identifiers are entered in the contract and in the register with the construction prefix
(`C-35.S1` and so on, as the template requires); the tables of this bead and of its part beads abbreviate them.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-400 (load cases and spectra,
the first seed in `frob-foundry`, which depends on this bead): that bead is listed in `related` and is not a
prerequisite. The law suite is generic over its subject trait and closes on the exact reference model and the
mutants in its own test support. If `frob-foundry` does not exist when this bead starts, the suite and its support
files are written under `crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025)
and the commands below read `-p frob-conformance`; the bead that implements the construction moves them into the
owning crate, binds its production types to the subject trait and runs the unchanged suite as part of its own
acceptance. A criterion or script step below that names production code or a verb of an implementing bead is owed
at that bead's acceptance and is logged `pending` here until then.

## Technical approach

### A. Frame of the three computations

1. Loads come from recorded missions: the envelope of interface wrenches from the energy ledger, with the
   free-body residual of each case checked.
2. A link is sized or optimized against that envelope (part B).
3. Stress histories at gauges are reduced to cycles and to damage (bead N10c-06).
4. Tolerances are allocated from sensitivities of functional dimensions (bead N10c-07).
5. Mass and inertia are written back into the model as a typed edit, and the loop repeats until it converges;
   every result is keyed by the digest of the envelope it used.

### B. Topology optimization of a link

Densities ρ_e ∈ [ρ_min, 1] on a mesh give the stiffness K(ρ) = Σ_e ρ_eᵖ·K_e, with K_e the stiffness of element e at
full density and penalization exponent p = 3 (DESIGN). The problem is

```text
minimize   c(ρ) = FᵀU
subject to K(ρ)·U = F,   Σ_e ρ_e·v_e ≤ V*,   ρ_min ≤ ρ_e ≤ 1
```

with v_e the element volume and V* the volume allowed.

| Id | Statement | Class |
|---|---|---|
| T1 | Sensitivity: ∂c/∂ρ_e = −p·ρ_eᵖ⁻¹·u_eᵀ·K_e·u_e, with u_e the displacement of element e. Derivation: K·dU = −dK·U, so dc = Fᵀ·dU = −Uᵀ·dK·U. | Tested (exact on a series-bar fixture; finite differences on small meshes) |
| T2 | No extra solve is needed for the sensitivity, because the compliance problem is self-adjoint: the adjoint equals the displacement. | Tested (solve count) |
| T3 | ∂c/∂ρ_e ≤ 0: adding material never increases compliance. | Tested |
| T4 | A density filter fixes the length scale. By DESIGN of this contract the filter is the linear hat filter ρ̃_e = Σ_j w_ej·ρ_j / Σ_j w_ej with w_ej = max(0, R − dist(e, j)), and sensitivities are chained through it. | Tested (weights sum to one; a uniform field is unchanged) |
| T5 | With several load cases the objective is a declared combination of per-case compliances and its sensitivity is the same combination of per-case sensitivities. | Tested |
| T6 | Known answer for sizing: for a load at the apex of a symmetric two-bar truss the volume at fixed stress is proportional to 1/(sin θ·cos θ), least at 45°. Derivation with half-span w, bar angle θ to the base line and apex load F along the axis of symmetry: bar force F/(2·sin θ), bar length w/cos θ, volume F·w/(σ·sin θ·cos θ). | Tested (closed form) |
| T7 | A structural result is valid for its load envelope only; a changed envelope digest invalidates it. | Rule (reuse needs a recorded implication) |

Formulation carried from the owning chapter so the contract and the mechanism agree: the design region is the
keep-in envelope minus swept volumes of neighbouring bodies minus actuator, bearing and cable keep-outs, with port
bosses frozen solid; where a sibling driver takes compliance at fixed volume, an outer bisection on volume
fraction finds the lightest feasible design; the surface is the level one half of the projected density;
smoothing may change volume by at most 1% (TARGET) and is always followed by an independent check on a
body-fitted mesh.

### Conventions, failure cases and non-claims

- p = 3 is DESIGN. Intermediate densities are a relaxation, not a material; the extracted surface is what is
  checked.
- Not claimed: any gain in time to a decision; optimality of the extracted part (a local optimum of a relaxed
  problem); validity for any load outside the envelope.
- The independent check uses a different discretization from the optimizer; because the sibling optimizer depends
  on the sibling solid solver, the independence is of mesh and formulation, not of codebase, and the cantilever
  and two-bar known answers anchor both paths.

### Law-test layout, registry, pilot and gates

`crates/frob-foundry/tests/laws_c35_topology.rs` with `tests/support/c35_ref.rs`: a series-bar model in exact
rationals; a small truss solver in `f64` for finite-difference checks; the two-bar volume function; the hat
filter. Generic over a trait `C35TopologySubject` (compliance, sensitivity, solve count); mutants in
`tests/support/c35_mutants.rs`. Registry: KA-168 with a derivation file; the construction's pilot in the pilot
register.

Pilot of the construction (run by BD-403 with BD-402 and BD-406): one link of a reference quadruped through all
three computations. Falsifier (TARGET): a mass saving below 7% at equal stiffness and peak stress, or a damage
figure that changes by more than 5% when the window length is halved. Ceiling: link mass fraction times saving,
0.27 × 0.30 ≈ 8% of robot mass on the quadruped with a published breakdown. Fallback: catalogue sections with a
beam check. Gates: GATE-FOUNDRY-04 at M5 (two-bar compliance within 10% and bar angle within 5°; the extracted
surface meshes and passes the mesh audit on 20 of 20 fixtures) and GATE-FOUNDRY-05 at M5 (the stage-2 pilot:
saving at least 7%, the kill threshold, with 15% the target; the convergence test passes within five iterations;
coarse solve at most 120 s per link class). Kill gate KILL-TOPOPT (M5): GATE-FOUNDRY-05 fails (saving under 7%, no
convergence, or over budget); then beam-rung sizing supplies mass and inertia feedback and three-dimensional
optimization becomes an offline option or is dropped.

## Acceptance criteria

- [ ] `docs/math/C-35.md` exists with part A and part B: the problem statement with every symbol defined, T1 to
      T7 with derivations, the carried formulation, the conventions and the non-claims, and the FACT lines with
      pins and dates.
- [ ] `cargo test -p frob-foundry --test laws_c35_topology` passes on the reference model, and on the production
      optimizer's sensitivity if BD-403 has landed.
- [ ] Every mutant below fails its named law.
- [ ] KA-168 is registered with a derivation file, a stated tolerance and the control named below (the row was
      printed with neither).
- [ ] The pilot record is in the pilot register.
- [ ] `scripts/e2e/spec_c35_topology.sh` exits 0.
- [ ] Row C-35 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `series_bar_sensitivity_exact`: four elements in series with stiffness ρ_e³·k₀ under end load F: compliance
  F²·Σ 1/(ρ_e³·k₀); the formula of T1 gives −3·F²/(ρ_e⁴·k₀) for each element, equal to the derivative of the closed
  form, in exact rationals for ρ = (1, 1/2, 3/4, 1/4).
- `sensitivity_matches_finite_difference`: 10² seeded small trusses; central differences agree with T1 to a
  relative 10⁻⁶.
- `sensitivity_nonpositive`: every component is at most zero.
- `one_solve_per_load_case`: the subject performs one linear solve per load case per evaluation.
- `ka_168_two_bar_truss`: V(θ) = 1/(sin θ·cos θ) in units of F·w/σ is 2 at 45°, 4/√3 ≈ 2.309 at 30° and at 60°,
  and is minimal at 45° over a 1° scan.
- `hat_filter_properties`: weights sum to one; a uniform field is unchanged; a single-element spike spreads over
  radius R.
- `multi_load_additivity`: the sensitivity of a weighted sum of two cases is the weighted sum of sensitivities.
- `envelope_digest_invalidates`: a result presented with a different envelope digest is refused.
- `ceiling_arithmetic`: 0.27 × 0.30 = 0.081, and 0.27 × 0.07 = 0.019, matching the quoted 2 to 8% of robot mass.

### End-to-end script and logging

`scripts/e2e/spec_c35_topology.sh`: `frob --json doctor`; `cargo test -p frob-foundry --test laws_c35_topology`;
`cargo test -p frob-conformance --test g1 -- KA-168`; file and registry checks. Log to
`target/e2e-logs/spec_c35_topology/<utc>.ndjson` with a transcript; each line has `ts`, `script`, `bead`, `step`,
`event`, command line, exit code, wall time, the evidence block (claim Known answer, class Tested, tier G1 host,
determinism D0 for rational rows, seed), sensitivities against the closed form with the relative error and its
threshold, and on failure the first differing element. The `summary` line counts assertions and names
GATE-FOUNDRY-04 and GATE-FOUNDRY-05.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-168 | Two-bar truss: volume at fixed stress proportional to 1/(sin θ·cos θ), least at 45° | closed form exact; for an optimizer, bar angle within 5° and compliance within 10% (the tolerance of GATE-FOUNDRY-04) | G1 host | Mutant `sensitivity_sign_flipped` (new): the optimizer moves away from 45° |

Other mutants: `penalization_dropped` (uses p = 1 in the sensitivity); `extra_adjoint_solve` (violates the solve
count); `stale_envelope_reused`. Counterexamples of this construction's other parts (CX-084, CX-085) are carried by
N10c-06. Determinism: rational rows exact; `f64` rows twice on one machine.

## Considerations

- Budgets of the owning chapter (TARGET, unmeasured): at most 120 s per unique link class for the coarse solve and
  10 minutes for a finalist solve on the CPU cores of the larger machine; a symmetric quadruped has three to five
  link classes.
- The loop of loads, mass and inertia is a fixed-point iteration with its own convergence test (BD-404); this
  contract only requires that results be keyed by envelope digest so the loop cannot reuse a stale structure.
- The saving range is a literature FACT to be re-read; the gate's 7% is a kill threshold, not a prediction.
- Licence and supply chain: the Foundry feature group adds sibling crates; the contract names them as foundations
  only.

## References

Plan §10 C-35; §31.5 to §31.7, §31.9; Appendix D.10 "Compliance sensitivity", "Two-bar truss"; KA-168;
GATE-FOUNDRY-01, -04, -05; KILL-TOPOPT; BET-29; D-11; A-15; BD-403.
