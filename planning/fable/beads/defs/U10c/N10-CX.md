---
key: "N10-CX"
title: "registry: the counterexamples of the constructions (CX-019 to CX-097) seeded into the negative-control registry"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "registry", "gauntlet", "solid", "frob-conformance", "test"]
parent: "WP-14"
deps: ["BD-025", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N10c-01", "N10c-02", "N10c-03", "N10c-04", "N10c-05", "N10c-06", "N10c-07", "N10c-08", "N10c-09", "N13-18", "SPEC-C-28", "SPEC-C-29", "SPEC-C-30", "SPEC-C-31", "SPEC-C-32", "SPEC-C-33", "SPEC-C-34", "SPEC-C-35", "SPEC-C-36", "SPEC-C-37", "SPEC-C-38", "SPEC-C-39", "SPEC-C-40"]
gates: ["GATE-GAUNT-01"]
plan_refs: ["Appendix N.1", "Appendix N.7", "Appendix N.35", "§38.1", "§38.11", "§10"]
---
## Background

FrankenRobotics has a standing rule: a plausible shortcut ships with the smallest hand-checkable case that breaks
it. Every such case stated by a chapter is an entry of the counterexample catalogue, numbered CX-001 onward,
append-only: a number is never reused and never renumbered. Each entry is the seed of at least one record in the
negative-control registry: the shortcut is planted as a mutation or as an input, and the gates of the rule it
protects must catch it. The reason is the program's rule that a gate nothing can fail validates nothing: a gate
with no control in the registry cannot record a pass.

This bead is the registry of the 79 counterexamples stated by the forty constructions of the mathematical spine
(chapter 10): CX-019 to CX-097. It carries every entry in full (the shortcut, the case with its numbers, the rule
it protects) so that no reader needs the plan, and enters each in the counterexample catalogue
`frob-conformance/registry/counterexamples.toml` with its kind of control and its plant, from which the seeding
tool of N38b-05 derives one control per entry in `frob-conformance/registry/controls.toml`. The planted defect
itself (the mutant code or the planted input) is written by the bead named in the "Planted by" column; this bead
writes the entry, names the planting bead and the plant, and checks that nothing is lost.

Terms. *Control*: a defect, not a boundary: "the request is refused at the limit" tests the limit, and the control
is the build or input in which the limit was removed. *Control kinds*: `mutation` (a test build in which the
shortcut replaces the rule), `planted-input` (a request that asks for the unsound conclusion and must be refused),
`planted-design` (a design the shortcut would accept or discard wrongly), `planted-world` (a world or world pair
that separates the shortcut from the rule), `a-a` (a wrong experimental procedure run against itself or a scripted
adversary). *Specification bead*: the bead `SPEC-C-nn` that writes construction C-nn as a contract with law
tests.

## Scope

In: the 79 entries below as data in the catalogue, in N38b-05's record schema; the consistency checks of the
test plan; the map from each entry to the bead that plants it and to its plant identifier.

Out: the catalogue's record schema and loader, and the seeding tool that turns entries into controls, allocates
their `NC-` identifiers, keeps the seed map, resolves rules that name no gate and proves completeness by the
join (N38b-05; this bead writes no row of `controls.toml` by hand); the mutant code and planted inputs themselves
(the specification beads of C-01 to C-40, their part beads, and N13-18 for CX-020); the known-answer registry of
chapter 10 (N10-KA1 to N10-KA3); the harness, the tier runners, the control record format and the registry
check (BD-025); running controls (N38b-04); wiring of controls into the M1 batteries (BD-146); entries of other
chapters that reuse a fixture of chapter 10 (they keep their own entries, because the registry holds one control
per protected rule).

## Technical approach

### 1. Catalogue record (the schema of N38b-05, shared with every chapter's registry bead)

Each row of the table in section 4 is one entry of `frob-conformance/registry/counterexamples.toml` with the
fields of N38b-05, filled from the columns: `id` (the CX number, append-only); `shortcut`, `case` and `rule` (the
second to fourth columns, verbatim; the expected observation stays in `case`, from which the seeding tool writes
the control's `expect`); `defined_in` (the "Stated in" column); `same_as` (CX-010 for CX-044, else empty);
`numeric` and `plan_case` (true and `numbers` for every entry; CX-034's case is supplied in its row from the gait
fixture of §22.9.2, recorded in `case_supplied_by`); `label` (DESIGN, or FACT with its source for the five
entries named in the notes, which carry `rerun = false` until re-run here); `control_kind` (the "Control kind"
column); `plant`; and `rule_ids` (the construction C-nn, the identifiers in the rule cell such as T-nn, D-nn,
GB-nn, BET-nn and FROB-REF-nnn, and the gates of the last column).

`plant` (DESIGN, so that every runnable entry has one): for the rows that name mutant or input identifiers
(CX-067 to CX-097, CX-020), those identifiers, prefixed by the planting bead (for example
`SPEC-C-30:raw_minimum`, `N13-18:M2`); for the other rows, `<planting bead>:cx-<nnn>` (for example
`SPEC-C-01:cx-019`), the name under which the planting bead must build the plant (a Cargo feature `cx-nnn` for a
mutation, a fixture `cx-nnn` for the other kinds) and may refine, keeping the CX number in it.

The seeding tool writes one control per entry in BD-025's record format (`id`, `seed` = the CX number, `gate`,
`kind`, `target`, `defect`, `expect`, `state`, `last_seen`). DESIGN, correcting an earlier draft of this bead
that wrote control records straight into `controls.toml`: that file is written only by the tool, so control
identifiers are allocated once and never rewritten.

### 2. Rules of the registry that this bead must respect

1. A gate with no control in the registry cannot record a pass.
2. A control is run every time its gate is run; a gate run in which a control was not caught is a failure of the
   gate, whatever the passing test says.
3. A control is a defect, not a boundary.
4. A control that has never failed its gate in any recorded run is itself suspect and is reviewed at each
   milestone exit: either the defect cannot be planted as described or the gate is not looking.
5. A false certificate, a false promotion or a withdrawn result found in use becomes a permanent control with the
   counterexample that exposed it.
6. An entry whose rule has no gate is a finding about the plan, reported at the next milestone exit: either the
   rule gains a gate or the entry is withdrawn with a recorded reason.
7. A case that is FACT from a planning pilot or from a sibling's source is re-run in this repository before its
   control is runnable: until then the entry has `rerun = false` and the seeding writes its control with
   `state = "obligation"`.
8. An entry with no numeric case is an obligation on its chapter: a control needs a concrete planted defect or
   input.

### 3. Procedure

1. Enter the 79 entries of section 4 in `registry/counterexamples.toml` with the fields of section 1.
2. Run N38b-05's seeding tool: one control per entry, gates resolved from `rule_ids` (directly for the gates of
   the last column; through `registry/rule_gates.toml` for the construction and rule identifiers). An entry whose
   last column says pending resolves to no gate until its planting bead maps the construction to its gates in
   `rule_gates.toml`; the tool writes it to `registry/seed_findings.toml` with `gate-added` requested.
3. As each planting bead lands, it completes the entry's `plant` with the code path and feature or fixture it
   built and maps C-nn to its gates; the next seeding gives the control its `target`, gates and runnable state
   with the same NC number. A control still an obligation, or a finding still open, at the milestone of its
   construction's first gate is reported at that milestone's exit.
4. Entries marked "≡" share a fixture with an entry of another chapter (CX-044 ≡ CX-010, `same_as`); both
   entries and both controls are kept.

### 4. The entries

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Stated in | Control kind | Planted by | Gates |
|---|---|---|---|---|---|---|---|
| CX-019 | Compute mobility from the Grübler–Kutzbach count | Sarrus linkage: the count gives 0 and the mobility is 1. Bennett linkage: the count gives −2 and the mobility is 1 | Mobility comes from intersections and products in the subgroup lattice, cross-checked by the rank of the constraint Jacobian at exact rational sample points (A-01) | C-01 | mutation | SPEC-C-01 | pending: attached by the planting bead |
| CX-020 | Move every spatial quantity with one adjoint, so that a wrench is moved by the twist map | A twist moves by Ad_g; a wrench is a covector and moves by Ad_g⁻ᵀ (pushforward) or Ad_gᵀ (pullback). A single reversed adjoint must fail three independent fixtures: Ad(g·h) = Ad(g)·Ad(h), frame invariance of the power pairing, and the planar 2R fixture of C-04 (torques −3 N·m, power −0.3 W) | Three separately typed maps; a wrench cannot be moved by the twist map (T-04, T-05; §11.3) | C-02 (numeric statement of the maps in §11.3) | mutation | N13-18: `M2` (the group coadjoint defined as (Ad_g)ᵀ, Marrow's mutation matrix; SPEC-C-02 states the law) | GATE-MARROW-01, GATE-MARROW-04 |
| CX-021 | Accept an inertia because it passes the positivity and triangle tests | A centred 1 kg part with three principal moments of 0.008 kg·m² passes both, yet tr(I_C)/(2m) = 0.012 m² where a ball of radius 0.10 m allows 0.010 m². An isotropic thin shell of that radius, with moments 1/150 kg·m², is the equality case | The mass-support bound ‖c − o‖² + tr(I_C)/(2m) ≤ ρ² for a declared envelope | C-03 | planted-design | SPEC-C-03 | pending: attached by the planting bead |
| CX-022 | Derive inertia from geometry by filling every enclosure solid | An aluminium box of 0.4 × 0.3 × 0.2 m at 2,700 kg/m³ is 64.8 kg as a solid and 3.45 kg with a 2.5 mm wall, a ratio of 18.8 | A declared wall thickness makes a body a shell by the difference rule; a solid fill of a large enclosure raises a density warning (§11.5, FROB-REF-168) | C-03 | planted-design | SPEC-C-03 | pending: attached by the planting bead |
| CX-023 | Take the lower three rows of the space Jacobian as the point Jacobian | Planar 2R chain, links 0.4 m and 0.3 m, angles 0 and π/2, tool point (0.4, 0.3) m: the point Jacobian is [[−0.3, −0.3], [0.4, 0.0]] m/rad, a tool force (10, 0) N gives torques (−3, −3) N·m, and both power computations give −0.3 W. For a space column (ωᵢ, vᵢ) the point column is vᵢ + ωᵢ × p; the lower rows alone drop ωᵢ × p and fail the fixture | The space Jacobian pairs with the wrench (p × F, F) about the origin and the point Jacobian pairs with F (D-9) | C-04 | mutation | SPEC-C-04 | pending: attached by the planting bead |
| CX-024 | Treat a limb's articulated inertia at its attachment port as positive definite and solve the limbs independently | Eliminating a joint axis s gives I − I·s·(sᵀ·I·s)⁻¹·sᵀ·I; for I the identity and s the first axis the result is diag(0, 1, 1, 1, 1, 1): one direction is removed, so the packet is semidefinite | Multi-limb ground contact goes through the coupled base solve | C-04 | mutation | SPEC-C-04 | pending: attached by the planting bead |
| CX-025 | Use the Cayley update as if it were the exponential at any step size | The Cayley step rotates by 2·atan(‖ω‖·h/2) instead of ‖ω‖·h; it is second-order | Use is bounded by ‖ω‖·h ≤ 0.2 per substep and refused beyond (FROB-REF-212; GB-01) | C-05 | planted-input | SPEC-C-05 | pending: attached by the planting bead |
| CX-026 | Approximate the friction cone by a pyramid | The box abs(γₜ₁) ≤ μ·γₙ, abs(γₜ₂) ≤ μ·γₙ admits (μ·γₙ, μ·γₙ), of norm √2·μ·γₙ: 41% of traction that does not exist along the diagonals | The second-order cone is used; the planted pyramid exploit must be caught by the model-form check | C-06 | mutation | SPEC-C-06 | pending: attached by the planting bead |
| CX-027 | Read full bracket rank as reachability | ẋ = u with u ∈ [0, 1]: the rank is one, and negative x is still unreachable from zero | A bracket direction is infinitesimal and assumes reversible inputs; a finite mission needs an integrated witness | C-07 | planted-input | SPEC-C-07 | pending: attached by the planting bead |
| CX-028 | Apply ideal rolling to a four-wheel fixed-axis (skid-steer) base | With axles at ±a, lateral no-slip requires v_y + a·ω = 0 and v_y − a·ω = 0, which forces ω = 0: under the shortcut the base cannot yaw at all | Ideal rolling is a regime; yaw of such a base exists only under a separately qualified slip model | C-07 | planted-design | SPEC-C-07 | pending: attached by the planting bead |
| CX-029 | Use a sibling routine as an oracle because it exists | `fs-mbd` `planar_drive` (reported, to recheck): from heading 32 rad with wheel speeds 1 and 1 + 6·10⁻¹³ m/s on a 0.5 m track, 400 steps of 1/400 s return zero translation where the limit is about (0.8342, 0.5514) m | A foundation is qualified against an independent formula at admission before it serves as an oracle; a reported defect becomes an admission test and an upstream work item (§33; BET-04) | C-07; §1.7 | planted-input | SPEC-C-07 | pending: attached by the planting bead |
| CX-030 | Write the curvature of the mechanical connection with +[𝔸ᵢ, 𝔸ⱼ] | On the planar three-link floating body, the component formula with the bracket term's sign flipped fails the loop test against direct integration by an error of order one | The component formula and its sign are fixed once (§11.6) and carry a sign control | C-08 (formula in §11.6) | mutation | SPEC-C-08 | pending: attached by the planting bead |
| CX-031 | Reduce by the mechanical symmetry group Γ_mech where the observation-preserving group Γ_obs is required | A quadruped with a front-only lidar has Γ_mech = C₂ × C₂ and Γ_obs = C₂. Adding a one-sided payload removes the reflection from Γ_obs. A quadruped crossing a one-sided slippery patch must be allowed asymmetric actions | Exact reduction uses only Γ_obs intersected with the stabilizers of payload, boundary conditions and contact mode (D-14) | C-09 | planted-design | SPEC-C-09 | pending: attached by the planting bead |
| CX-032 | Treat angular velocity and torque as polar vectors under a mirror | Under the sagittal mirror P = diag(1, −1, 1) the axial image of a joint axis a is −P·a: a pitch axis along y keeps its sign, and roll and yaw axes change theirs. A table that flips all three, or none, fails the step-equivariance test | Polar vectors map by P and axial vectors by det(P)·P (§11.4; T-10) | C-09 | mutation | SPEC-C-09 | pending: attached by the planting bead |
| CX-033 | Conclude that a task is infeasible because the search over symmetry-tied deterministic policies failed | Two identical latches, exactly one must close. At a reflection-fixed observation a tied deterministic policy issues equal commands and both fail; either asymmetric choice succeeds, and a policy that draws one of the two uniformly is equivariant in law | A failure of tied-policy search excludes only the tied policy class, and the report says so and nothing more | C-09 | planted-input | SPEC-C-09 | pending: attached by the planting bead |
| CX-034 | Find a gait orbit inside Fix(K) and judge its stability there | An orbit computed inside Fix(K) can hide unstable multipliers transverse to it (C-10 states the rule; the smallest numeric case is given with the gait analysis of §22.9.2: ẋ = −x, ẏ = +y with the symmetry y ↦ −y and period T; on Fix(K) = {y = 0} the multiplier is e^(−T), the full-space multiplier of the antisymmetric block is e^(+T) > 1, so the restricted answer says stable for an orbit that is unstable) | The monodromy is always computed on the full space | C-10 | mutation | SPEC-C-10 | pending: attached by the planting bead |
| CX-035 | Refuse simultaneous impacts, or impose an order on them | Trot, bound and pronk have simultaneous touchdowns by construction; forcing an order can change the outgoing velocity and a gait's basin | Simultaneous events are resolved by one coupled solve and are never a reason to refuse a gait | C-10; C-16 | mutation | SPEC-C-10, SPEC-C-16 | pending: attached by the planting bead |
| CX-036 | Summarize a port by its peak demand and add the peaks | Two fingers on one unit-current bus have identical local summaries. A rule that adds their peak demands rejects a task that a two-tick schedule completes: no one-tick schedule is feasible and two two-tick schedules are | A port contract carries time correlation; the two-finger bus fixture must not be pruned, and two modules with identical local summaries on one current-limited bus are not merged (BET-31) | C-12 | planted-design | N10a-11 (the C-12 budget-and-reuse part of SPEC-C-12): `cx-036` | pending: attached by the planting bead |
| CX-037 | Reuse a result after an edit because the sub-term's hash still matches | Swapping a leaf keeps its siblings' local results, but an edit that touches a shared battery bus, a shared thermal sink, a cross-branch kinematic loop or base-coupled contact invalidates every global result, because a shared moving base couples all limbs through the contact operator. Controls: a mirror swap of a leaf reuses its siblings' results; an edit to the shared bus must invalidate | A hash identifies; only a recorded implication licenses skipping a recheck (D-11; FROB-REF-835) | C-12; D-11 | mutation | SPEC-C-12 | pending: attached by the planting bead |
| CX-038 | Score an estimator after fitting a scale | A 2 m estimate of a 1 m motion has zero residual after a factor of ½ | A metric gravity-referenced estimate is aligned with one translation and one yaw per trajectory and never with a fitted scale | C-13 | planted-input | SPEC-C-13 | pending: attached by the planting bead |
| CX-039 | File a reassociation as a bit-exact rewrite | In `f32` with a = 10⁸, b = −10⁸, c = 1: (a + b) + c is 1 and a + (b + c) is 0 | Rewrite classes: class E changes no bit and ships a replayable trace; class R (reassociation, distribution, fused multiply-add) makes a different kernel with a new identity; the trace checker rejects the misfiled rewrite (T-27) | C-14 | mutation | SPEC-C-14 | pending: attached by the planting bead |
| CX-040 | Freeze a fixed-n decision rule without listing the verdicts it can return | Two-sided exact Clopper–Pearson rule at n = 64 with a ±5 point indifference band: perfect concordance gives [−0.0662, +0.0662], so no outcome can return a tie. The recorded rule with 64 pairs could return three of its four verdicts; the fourth was first reachable at 86 pairs | A fixed-n study ships a pre-analysis of reachable verdicts (T-26); a study that needs an unreachable verdict is refused at admission | C-15; §0.7 | planted-input | SPEC-C-15 | pending: attached by the planting bead |
| CX-041 | Use the reset Jacobian as the derivative across an event | Bouncing ball, state (y, v), reset (y, −r·v), impact speed v⁻ < 0: the saltation matrix is [[−r, 0], [−(1 + r)·g/v⁻, −r]] with determinant r². The reset Jacobian diag(1, −r) has the wrong sign in the first entry, omits the coupling in the second row, and for r = 1 has determinant −1 where the elastic bounce preserves area with determinant +1 | Event derivatives use the saltation matrix and carry the label `saltation` (D-15) | C-16 | mutation | SPEC-C-16 | pending: attached by the planting bead |
| CX-042 | Transport a derivative across an event by a symmetry without checking that the relabeling commutes with the event | Two bouncing balls and the swap of their labels satisfy D h(x⁺)·Ξ_e(x⁻) = Ξ_{h·e}(h·x⁻)·D h(x⁻) when the restitutions are equal and violate it by terms proportional to r_L − r_R when they differ | A symmetry is used across an event only where the commuting identity holds | C-16 | planted-world | N10b-02 (the C-16 commuting-identity part of SPEC-C-16): `cx-042` | pending: attached by the planting bead |
| CX-043 | Return a derivative at a grazing event | A ball whose apex touches a ceiling: the guard is not crossed transversally, and the query returns `refused`, not a matrix | Refusal near grazing, at an ambiguous event order, across an active-set change or at frictional degeneracy (FROB-REF-263) | C-16 | planted-input | N10b-01 (the C-16 derivative-label part of SPEC-C-16): `cx-043` | pending: attached by the planting bead |
| CX-044 | Certify the convex hull of two feasible designs | Feet at (±1 − e, ±1) m, each capped at 0.5 N, carry a load W at the origin; the positive-x pair must supply W·(1 + e)/2. The designs (e, W) = (0, 1.9) and (0.9, 1) each need 0.95 N and are feasible; their midpoint (0.45, 1.45) needs 841/800 N from a pair capped at 1 N | An inner cell needs a witness for the whole cell; only a union of proved inner cells is inner; the midpoint must be excluded from every inner set (BET-20) | C-17 ≡ CX-010 | planted-design | N10b-04 (the C-17 proof-rule part of SPEC-C-17): `cx-044` | pending: attached by the planting bead |
| CX-045 | Accept one policy per world as evidence that one policy works for all worlds (∀w ∃π offered for ∃π ∀w) | Docking fixture with the packet arriving late, gains g ∈ {1, 2}: with first command p the remaining commands must be (1/g − 2p, p − 1/g), which differ by ½ between the gains. Each world alone has a successful open-loop command and no single command succeeds in both | Quantifiers are part of the claim; evidence for an easier quantifier order is rejected by type and by the auditor (D-12; §8.3 rule 8; FROB-REF-826) | C-17; C-19 | planted-input | SPEC-C-17 | pending: attached by the planting bead |
| CX-046 | Evaluate a strict requirement with a non-strict comparator, or by an infimum | At U = ¾ and ε = 5/16 the docking residual is exactly zero and the strict predicate fails: with p = 5/8 both worlds can report 15/16 while needing −¼ and −¾. On 0 < w ≤ 1 every realization has w > 0 although the infimum is 0 | Strict, non-strict and equality atoms are evaluated with their declared comparators; a uniform positive margin is a separate, stronger request; strict decisions are computed exactly (D-18) | C-17; C-19 | mutation | SPEC-C-17 | pending: attached by the planting bead |
| CX-047 | Multiply stage pass rates to get the mission pass rate | Among 20 equally likely worlds two stages each pass in 19. The mission passes in 18 worlds if the failing worlds differ and in 19 if they coincide | A chance constraint preserves the joint law across stages or allocates failure budgets that sum to the allowance; a product bound needs conditional success at every reached history | C-17 | mutation | SPEC-C-17 | pending: attached by the planting bead |
| CX-048 | Intersect per-world answers to obtain the joint answer | F(V) ⊆ ⋂ over w ∈ V of F({w}), and the gap is what the controller cannot observe in time: in the docking fixture with a late packet each of the worlds g = 1 and g = 2 is solvable alone and the joint query has no solution | The joint query is answered as stated; per-world evidence is not combined by intersection (D-9, D-12) | C-18 | mutation | SPEC-C-18 | pending: attached by the planting bead |
| CX-049 | Keep an inclusion when the world set grows | Docking family: enlarging W by one hidden gain 33/32 keeps every exclusion and forces a recheck of every inclusion. The inclusion at (U, ε) = (¾, 1/64) reverses, because the reading (65/64)·p is possible under gains 1 and 33/32 for every first command p; the least worst-case terminal error becomes 1/65 m | Verdicts have polarity: an inner certificate survives when W shrinks, Π grows or A weakens, and must be rechecked on the opposite moves; an exclusion is the reverse | C-18 | planted-world | SPEC-C-18 | pending: attached by the planting bead |
| CX-050 | Trust a cache key or a dependency record to mean the verdict still holds | A dependency graph with one edge omitted returns a promotion that fresh replay contradicts | Reuse needs a recorded implication; the stale-verdict control must be caught by the shadow recompute (D-11; BET-33) | C-18 | mutation | SPEC-C-18 | pending: attached by the planting bead |
| CX-051 | Take pairwise compatibility as joint compatibility | Commands (x, y) ∈ [0, 1]² with three variants requiring x ≥ 2/3, y ≥ 2/3 and x + y ≤ 1: every pair of sets intersects and the triple does not. Pairwise compatibility would have authorized one shared controller; two labels suffice and one is impossible | The label count is a colouring of the hypergraph whose edges are the inclusion-minimal collections with empty intersection | C-19 | planted-design | N10b-09 (the C-19 hypergraph-colouring part of SPEC-C-19): `cx-051` | pending: attached by the planting bead |
| CX-052 | Count repeated sensor reads as independent evidence | A binary probe correct with probability ¾ gives 27/32 with three independent reads, 459/512 with five and 3807/4096 with seven, and stays at ¾ after any number of reads that share one fault bit | Error dependence, not per-read accuracy, sets the value of redundancy | C-19 | mutation | SPEC-C-19 | pending: attached by the planting bead |
| CX-053 | Expect more training to overcome a sensing limit | Docking fixture: if the only informative packet arrives after the third command is committed, no causal policy docks in both worlds for any U and ε; the two worlds deliver identical packet histories up to the commitment deadline and need disjoint commands | An indistinguishability witness ends a policy search before it is paid for (`mission.refute`) | C-19 | planted-world | SPEC-C-19 | pending: attached by the planting bead |
| CX-054 | Specify sensor accuracy separately from actuator authority and the probing command | Docking fixture at U = ¾: the probe p = ¼ tolerates ε < 1/8 and the probe p = 5/8 tolerates ε < 5/16 on the same actuator; in general ε < (U + ½)/4 | Sensor, actuator and delivery time appear in one design inequality; the probe and the feedback are chosen together | C-19 | planted-design | N10b-08 (the C-19 docking-fixture part of SPEC-C-19): `cx-054` | pending: attached by the planting bead |
| CX-055 | Read a full-rank local observability Gramian as distinguishability | Range anchors at (−1, 0) and (1, 0) both read √2 from (0, 1) and from (0, −1). With unit variances the information matrix is the identity at both points, yet a controller that must pick the side fails. A third anchor at (0, 2) reads 1 or 3, and closed reading intervals of half-width ε separate exactly when ε < 1 | Every calibration and placement analysis also runs a global ambiguity search | C-20 | planted-world | SPEC-C-20 | pending: attached by the planting bead |
| CX-056 | Two designs whose geometric ratios match are dynamically similar, so a verdict moves between them | Scale a body by λ at constant density: mass × λ³, static joint torque × λ⁴. With a motor whose torque scales as λ³ the group τ_max/(m₀·g·ℓ₀) falls as 1/λ, so a design that stands at one size fails at twice the size. A fixed latency has delay group ∝ λ^(−½): it doubles at one quarter of the size | Every dimensionless group must match before a verdict is transported; differing groups are reported by name (C-21, D-11) | C-21 | planted-design | SPEC-C-21 | pending: attached by the planting bead |
| CX-057 | A physical constant may sit in a kernel as a pure number | Scale lengths × 4, times × 2, masses × 2 with gravity unchanged. A homogeneous kernel reproduces every significand bit and branch; a kernel with a hidden dimensional constant differs on its first step | Dimensional homogeneity of every kernel (T-09); the power-of-two similarity test | C-21 | mutation | SPEC-C-21 | pending: attached by the planting bead |
| CX-058 | Quotient a campaign by the mechanical symmetry group Γ_mech of the chassis | Ten leg types on a hexapod: dihedral symmetry of order 12 gives 86,185 classes. With a front-only lidar only the sagittal mirror survives and there are 500,500 classes; counting with Γ_mech merges designs that the closed loop distinguishes | The acting group of a campaign is Γ_obs, never Γ_mech alone (C-22, C-09) | C-22 | mutation | SPEC-C-22 | pending: attached by the planting bead |
| CX-059 | Designs that agree on some mission observables can be merged | Eight ballast slugs enter the dynamics through count, Σx and Σx². Slots {1, 4} and {2, 3} have equal count and equal Σx = 5 but Σx² = 17 and 13: they are different rigid bodies | A projection class requires equality over all active observables, and merged pairs must replay bit-identically (C-22) | C-22 | planted-design | N10b-12 (the C-22 projection-quotient part of SPEC-C-22): `cx-059` | pending: attached by the planting bead |
| CX-060 | A design-conditioned policy may read its tokens in file order | Exchange two legs in the description of one robot: the robot is unchanged and the actions change. FACT (source register): a published morphology-conditioned policy lost most of its return when the same robots were re-described in equivalent conventions | A family policy is natural under re-description, σ(d′) = φ·σ(d); permutation-equivariant tokens (C-23) | C-23 | mutation | SPEC-C-23 | pending: attached by the planting bead |
| CX-061 | More equivariance is better: make the policy equivariant under all rotations | A rotation about a horizontal axis does not preserve gravity. The rigid motions that do are rotations about the vertical, translations and reflections through vertical planes, and the layers are equivariant under those only | The gravity control of C-24: a horizontal-axis rotation must not be a symmetry of the policy | C-24 | mutation | SPEC-C-24 | pending: attached by the planting bead |
| CX-062 | Audit a small share of worlds on the high tier and correct by inverse weights; the cheap prediction only has to be roughly right | With Y ∈ {0, 1}, a prediction wrong by a constant ½ and audit probability 0.1, the added variance is (1 − p)/p·(Y − m)² = 9 × ¼ = 2.25, worse than auditing everything. The estimate stays unbiased; the cost is variance | Tier correction is funded only where the measured ρ supports it (kill below 0.7); p_min bounds the weights (C-25) | C-25 | planted-input | SPEC-C-25 | pending: attached by the planting bead |
| CX-063 | Calibrate the conformal margin on the designs that were promoted, since those are the ones that ran both rungs | On a fixture where the residual abs(y_hi − ĝ(y_lo)) correlates with the cheap score, q̂ computed from promoted designs only under-covers | The audit set is drawn by keyed randomness before any score exists and run at fixed n (C-26) | C-26 | a-a | SPEC-C-26 | pending: attached by the planting bead |
| CX-064 | A dozen audit designs are enough for a 95% conformal margin | The index is ⌈(n + 1)·(1 − α)⌉: n = 39 gives the 38th smallest residual; n = 18 gives 19, which exceeds n. Fewer than 19 audit designs cannot support α = 0.05 | Minimum audit-set size for a declared α (C-26) | C-26 | planted-input | SPEC-C-26 | pending: attached by the planting bead |
| CX-065 | A defensive mixture proposal makes certifying the absence of failures cheaper | With q = β·p + (1 − β)·q′ the weights are at most 1/β, and zero observed failures certify only p ≤ ln(1/δ)/(β·n). At δ = 0.05 and p* = 10⁻⁴ crude sampling needs about 3·10⁴ nominal episodes, and the mixture needs that many on its nominal share | Certification and exploration are two lanes that never mix (D-16); no sampler discounts certified absence (C-27) | C-27 | planted-input | SPEC-C-27 | pending: attached by the planting bead |
| CX-066 | Forking a world at a splitting level multiplies the evidence | A fork without fresh randomness is an identical copy: children that share the parent's streams reproduce the parent's trajectory bit for bit and add nothing | Child streams are keyed by child world addresses; duplicate children are detected (C-27, §11.1) | C-27 | mutation | SPEC-C-27 | pending: attached by the planting bead |
| CX-067 | The shrunk failing world is the smallest one, and its size measures distance to failure | Edit 1 (a grease patch) fails the design; edit 2 (a rug over the patch) masks it. {1} fails and {1, 2} passes, so failure is not monotone in the edit set. With eight edits and failure exactly when 3 and 6 are present the search returns {3, 6}, which is 1-minimal by construction and nothing more | A result is reported as 1-minimal under the tested edit relation; no distance to a boundary is claimed (C-28, §12.3) | C-28 | mutation | SPEC-C-28: `monotone_pruning`, `claims_distance` | GATE-NEMESIS-03 |
| CX-068 | Any world in which the design fails is a counterexample | A doorway narrower than the body fails every controller: it is a useless trap and says nothing about the design | A counterexample is kept only with a solvability witness from the candidate's own body, actuators and mission constraints (C-28) | C-28 | planted-world | SPEC-C-28: `keep_any_failure` | GATE-NEMESIS-01 |
| CX-069 | A large Shapley value identifies a necessary cause | Two interventions A and B. Case 1: either alone prevents the failure, v({A}) = v({B}) = v({A, B}) = 1. Case 2: only both together prevent it, v({A}) = v({B}) = 0, v({A, B}) = 1. Both give φ = (½, ½); case 1 has two but-for causes with PN = 1 and case 2 has none | A cause is named by the probability of necessity against the placebo flip rate; Shapley is a budgeted summary (C-29) | C-29 | mutation | N10c-01: `shapley_as_necessity` | GATE-AUTOPSY-04 |
| CX-070 | An intervention that flips the outcome is a cause | On a fixture where every last-place nudge of an unrelated state flips the outcome, any intervention appears to prevent the failure | The placebo fork: c is a cause only when PN(c) exceeds the placebo flip rate ρ₀; otherwise the report is `chaotic: unattributable` (C-29) | C-29 | mutation | N10c-01: `no_placebo` | GATE-AUTOPSY-05 |
| CX-071 | A design-parameter change can be tested by forking at a snapshot just before the failure | For a gear ratio or a mass, the thermal state, the estimator state and the policy's history at the snapshot would all differ under the changed design, and an intervention at a snapshot answers a question about the continuation only | Design-parameter interventions re-run from the episode start on the same streams; a fork is refused when the controller cannot adapt without retraining (C-29) | C-29 | planted-input | SPEC-C-29: `snapshot_fork_for_design_parameter` | GATE-AUTOPSY-02, GATE-AUTOPSY-06 |
| CX-072 | The robustness of a conjunction is the minimum of the raw margins | A clearance of 0.02 m and a thermal margin of 15 K: the minimum is 0.02 (clearance binds) in metres and kelvin, and 15 (the thermal margin binds) in millimetres and kelvin. With declared scales 0.10 m and 10 K the values are 0.2 and 1.5 and clearance binds in every unit system | Every atom has a declared scale of its own dimension; robustness is ρ = ψ/s (C-30) | C-30 | mutation | SPEC-C-30: `raw_minimum` | GATE-ARENA-03, GATE-CORTEX-10 |
| CX-073 | When an episode ends early, evaluate "always" over the samples that exist | An episode that ends at 30 s under G_[0, 45 s]: the minimum over 30 s can be positive although the requirement was never tested on the last 15 s | A window the trace does not cover yields `undetermined`, never a truncated minimum (C-30, §24.5) | C-30 | mutation | SPEC-C-30: `truncated_minimum`, `pads_short_window` | GATE-ARENA-03 |
| CX-074 | Project the action onto the barrier row, then clip to the actuator box | R = I, u_π = (1, 0), box [0, 1]², row u₁ + u₂ ≥ 1.5. Projection ignoring the box gives (1.25, 0.25), outside the box; clipping gives (1, 0.25), with u₁ + u₂ = 1.25 < 1.5. The breakpoint method gives (1, 0.5) | The one-row filter is solved exactly inside the box; multi-row solves carry a mandatory KKT check with a passive fallback (C-31) | C-31 | mutation | N10c-02: `project_then_clip` | GATE-SHIELD-02 |
| CX-075 | For every disturbance there is a safe action, so the filter is safe | A certificate proved in the order ∀w ∃u lets the action be chosen after the disturbance and the estimation error are seen, which the controller cannot do | The quantifier order is ∃u ∀(w, e); a certificate in the other order is rejected (C-31, FROB-REF-826) | C-31 | planted-input | N10c-02: `prefix_not_compared` | GATE-SHIELD-02 |
| CX-076 | An energy budget may leave out small sources | Lifting 8 kg over a 12 mm sill needs 8 × 9.81 × 0.012 = 0.94176 J, above a 0.90 J budget, so the family is excluded; 0.05 J of approach kinetic energy (0.95 J) removes the exclusion. In the stop budget (0.64 + 0.50 + 0.25 + 0.10 = 1.49 J of 2.00 J) a missing gravity term over the stroke makes the checker reject | A budget has a term for every source on the declared load path (T-28); the omitted-term control | C-31 | mutation | SPEC-C-31: `omits_gravity_term`, `exclusion_without_approach_energy` | GATE-SHIELD-01 |
| CX-077 | The first moment of the load can be passed where the support torque is expected | With +z up the support torque is (P_y, −P_x, 0). P = (60, 30) N·m with the stance forces gives (30, −60, 0) N·m; a packet that passes (60, 30) as the torque has the wrong components and sign | Every field of a bundle is typed with frame, origin, sign and unit (§12.4, FROB-REF-829) | C-32 | planted-input | SPEC-C-32: the packet with (60, 30) in a torque field | GATE-LEMMA-03 |
| CX-078 | An inclusion proved for independently commanded foot forces holds for the same robot on a passive suspension | Stance fixture at b = ¼ m with a 160 N cap: active allocation gives (155, 135, 105, 5) N, inside the cap; equal passive springs give (175, 115, 85, 25) N, above it | A typed result states its authority model; the assumption `authority` travels with the certificate (C-32) | C-32 | planted-input | SPEC-C-32: `authority_ignored` | GATE-AUGUR-02 |
| CX-079 | Centre of mass inside the support polygon and total foot capacity above the weight means the stance is feasible | Stance fixture at b = 1/5 m: the centre of mass (150 mm, 75 mm) lies inside the feet at (±1/3, ±1/5) m and total capacity is 4 × 160 = 640 N against 400 N, yet the moment equations give f₊₊ − f₋₋ = 165 N with f₊₊ ≤ 160 N and f₋₋ ≥ 0. FACT (planning pilot, to be re-run): the test accepted 1,596 bodies the exact model rejects | The stance family is decided by the exact model with per-foot bounds and a dual witness (C-32, T-22) | C-32 | planted-design | SPEC-C-32: `support_polygon_screen` | GATE-AUGUR-02 |
| CX-080 | A region certificate can be had by paving the whole design box | 10⁴ boxes are 13 bisections, less than one cut per axis in 20 dimensions: a uniform box keeps 0.64 of each side (0.41 in 10 dimensions) | Certificates are issued per constraint on sub-boxes of about six parameters or fewer and assembled by products, monotonicity and witnesses (C-32, C-17) | C-32 | planted-input | N10c-04: `joint_paving_allowed` | GATE-AUGUR-09 |
| CX-081 | Clearance can be tested with one scalar radius | A thin rod of length L turns a right-angle corner between corridors of width a exactly when L ≤ 2√2·a. The circumscribed-disc test, L ≤ a, refuses every rod between a and 2.83·a although each passes; the inscribed-disc test accepts rods longer than 2.83·a, which jam | Clearance is certified from both sides on complexes in SE(2), with a reported band (C-33) | C-33 | mutation | SPEC-C-33: `circumscribed_disc`, `inscribed_disc` | GATE-HAB-10 |
| CX-082 | A linkage that closes at every target pose reaches them all | Links 4 (ground), 1 (crank), 3.5 (coupler), 3 (rocker): 1 + 4 ≤ 3.5 + 3, so the crank turns fully and the two assembly circuits are disjoint. A candidate whose target poses lie on different circuits satisfies closure at each pose and cannot move between them without disassembly | The circuit and branch certificate of lane 1 (C-34) | C-34 | planted-design | SPEC-C-34: `closure_only` | GATE-AUGUR-17 |
| CX-083 | Every tracked root passed the α-test, so the solution set is complete | An α-test proves a nearby nonsingular root and nothing about completeness. The Stewart–Gough forward problem has 40 solutions: a run that certifies fewer, or loses a path, has not found them all | A result is `Complete` only when the certified count equals an independent generic count with no lost path; otherwise `Partial(certified, lost)` (C-34) | C-34 | mutation | N10c-05: `complete_from_alpha_alone` | GATE-AUGUR-22 |
| CX-084 | Rainflow counting can be run window by window | The history 0, 10, 0 split into the windows [0, 10] and [10, 0] holds one full cycle of range 10; a counter that drops residues at window ends counts none | The residue of open half-cycles is carried between windows (C-35) | C-35 | mutation | N10c-06: `drops_residue_at_window_end`, `close_per_window` | GATE-FOUNDRY-03 |
| CX-085 | Rare overload cycles can be dropped from a fatigue history | With m = 3, 10⁶ cycles at a life of 2·10⁶ give D = 0.5; 10³ cycles at five times the amplitude (life 16,000) add 0.0625: 0.1% of the cycles carry 11% of the damage | Load cases are the recorded envelope over qualified missions; the damage sum keeps every cycle (C-35) | C-35 | mutation | N10c-06: `drops_rare_cycles` | GATE-FOUNDRY-03 |
| CX-086 | Momentum conservation is the test of a step for any system with a symmetry | Chaplygin sleigh: v̇ = a·ω², ω̇ = −m·a·v·ω/(I + m·a²). The energy ½·m·v² + ½·(I + m·a²)·ω² is constant, since m·v·(a·ω²) − m·a·v·ω·ω = 0, while the direction of linear momentum turns | The momentum test of C-04 is not applied to nonholonomic systems; they are tested by energy (C-36) | C-36 | planted-input | N10c-08: the momentum check applied to the sleigh | GATE-TRAC-01 |
| CX-087 | If the variational step conserves momentum, its implicit solve is right | Evaluate step 2 with the differential at +hξ_k instead of −hξ_k (the other trivialization). Momentum stays exact, because the transport of step 3 conserves it for any ξ; the energy bound and reversibility are lost | The variational step is tested by bounded energy error (÷4 when h is halved) and by reversal, never by momentum alone (C-36) | C-36 | mutation | SPEC-C-36: `wrong_trivialization` | GATE-KIN-01 |
| CX-088 | Model every wheel as an ideal rolling constraint | Four fixed wheels as ideal rows return ω = 0 under any command: a skid-steer base cannot turn. With cone-bounded rows it turns with its lateral multipliers on their cones | Rolling rows carry traction limits; ideal rolling is a limit of the same solve (C-36, C-07) | C-36 | mutation | N10c-08: `all_wheels_ideal` | GATE-TRAC-01 |
| CX-089 | Sliding friction and torsional friction are independent rows with their own coefficients | At v_t = r·ω_n the coupled law gives a tangential friction magnitude of μ·λ_n/√2; the uncoupled model gives μ·λ_n. A spinning contact resists sliding less | One four-dimensional cone for sliding and spin (C-37) | C-37 | mutation | SPEC-C-37: `independent_rows` | GATE-TRAC-04 |
| CX-090 | Heat entering the winding is a loss, and the node's temperature rise is stored energy | A ledger that books the same heat as both stored and lost shows a deficit equal to that heat | H + U changes only by port power and by the heat that leaves to ambient; ledger closure in all three modes (C-38) | C-38 | mutation | SPEC-C-38: `heat_double_counted` | GATE-SINEW-02 |
| CX-091 | A passing second-law check shows the thermal model is right | The check has content only as a sign check on each assigned loss: it cannot fail for a correct model. Its one catch: a regenerative mode with a negated copper loss gives σ < 0 | The entropy balance is a derived report, used as a sign control and not as validation (C-38) | C-38 | planted-input | SPEC-C-38: the regenerative mode with a negated copper loss | GATE-SINEW-08 |
| CX-092 | Any measurement of the mechanism will identify it; fit a richer curve | Laws μ₁(v) = 0.5 and μ₂(v) = 0.3 + 0.4·v agree at 0.5 m/s and imply opposite designs at the mission speed of 1.5 m/s (0.5 against 0.9). A probe at 0.5 m/s separates nothing at any precision; a probe at 1.0 m/s reads 0.5 or 0.7 and separates them exactly when the reading error is below 0.1 | A probe is chosen so that the reading sets of the decision blocks are disjoint over the nuisance intervals (C-39) | C-39 | planted-input | N10c-09: `richer_curve_fit`, the probe at 0.5 m/s | GATE-AUGUR-11 |
| CX-093 | A probe that separates the admitted mechanisms settles the design decision | A third law that follows μ₁ up to 1.0 m/s and departs later defeats the 1.0 m/s probe. FACT (planning pilot, to be re-run): a probe synthesized for three admitted laws chose correctly in 675 of 675 in-class cases; a fourth law reversed all 225 held-out decisions; direct measurement at mission dwell was right in 900 of 900 | A probe certificate is conditional on the admitted set ℳ and says so; direct measurement at mission conditions is always an admitted probe (C-39) | C-39 | planted-world | N10c-09: `unconditional_certificate` | GATE-AUGUR-11 |
| CX-094 | Dominance can be checked by enclosing each design's outcomes separately and comparing | z ∈ [0, 1]; A succeeds when z ≥ 3/5 and B when z ≥ 2/5. Separate enclosures give {0, 1} for each and admit all four pairs, including (success, failure), which no shared z produces | Dominance is unreachability of (success, failure) in the product system on shared inputs (C-40) | C-40 | mutation | SPEC-C-40: `separate_enclosures` | GATE-AUGUR-16 |
| CX-095 | A product enclosure that covers each design's outcome set covers the joint outcomes | A product whose internal branches are wrongly tied reaches only (0, 0) and (1, 1). Its projections cover both marginal outcome sets, and it hides a reachable (1, 0) | The checker demands joint coverage; one mislabeled branch invalidates the certificate (C-40) | C-40 | mutation | SPEC-C-40: `marginal_coverage_only` | GATE-AUGUR-16 |
| CX-096 | Pair two designs by seed and tick, and the worlds match | Two tyres on different paths over a patterned floor meet different friction at the same tick. A product coupled by tick and not by position reports a false regression or hides a true one | Shared inputs are compiled at physical coordinates: one space-indexed field read at each robot's contact point (C-40) | C-40 | mutation | SPEC-C-40: `tick_indexed_field` | GATE-AUGUR-16 |
| CX-097 | "B never loses to A" means B meets the mission | In the same fixture neither design is guaranteed: A fails for z < 3/5 and B for z < 2/5, while the paired verdict of no loss holds on the whole interval | The answer carries the paired verdict and each design's absolute verdict separately (C-40) | C-40 | planted-input | SPEC-C-40: `paired_verdict_only` | GATE-AUGUR-16 |

Notes on the table. Entries CX-019 to CX-066 belong to constructions C-01 to C-27; their mutants are written by
those constructions' specification beads, which also name the gates, or, where a specification was split into
part beads, by the part bead named in the row that carries the entry (CX-036 N10a-11, CX-042 N10b-02, CX-043
N10b-01, CX-044 N10b-04, CX-051 N10b-09, CX-054 N10b-08, CX-059 N10b-12; each of them cites its entry, while
the specification bead does not). Entries CX-067 to CX-097 belong to C-28 to
C-40; the mutant names in the "Planted by" column are those of the law suites written for those constructions.
CX-029 is a reported defect of a sibling routine: it is an admission test and an upstream work item, and the
routine is not used as an oracle until it passes. CX-034 states its rule in words in its construction and points
to the gait analysis for its smallest numeric case; that case (§22.9.2, the same fixture as the Reflex entry
CX-237, which protects the gait obligation and keeps its own control) is carried in its row, with
`case_supplied_by = "§22.9.2 transverse-instability fixture (CX-237)"`, so the entry is runnable once SPEC-C-10
builds its plant; CX-034 is not one of the catalogue's seventeen word-only entries. CX-020 is the catalogue
entry of Marrow's mutation `M2` (N13-18: Ad* and its inverse exchanged, the group coadjoint defined as (Ad_g)ᵀ),
so the control seeded from it is the `M2` record that N13-18's runner finds under GATE-MARROW-01 and
GATE-MARROW-04 (the seeding tool processes catalogue entries first, and a known-answer row that names `M2` reuses
that control instead of allocating a second one). Cases labelled FACT from a planning
pilot or a publication (in CX-029, CX-040, CX-060, CX-079, CX-093) carry `rerun = false` and are re-run or re-read
here before their controls become runnable. Section marks inside the cells (for example a decision number or a refusal code)
are provenance: the cell itself states the rule.

## Acceptance criteria

- [ ] `registry/counterexamples.toml` holds the 79 entries CX-019 to CX-097 with every field of section 1;
      re-running N38b-05's seeding gives exactly one control per entry (79) and a second run changes no byte; no
      row of `controls.toml` was written by hand.
- [ ] No CX number is missing, duplicated or renumbered; the constructions' count in the catalogue loader is 79.
- [ ] Every entry's `plant` names its planting bead and a plant identifier; no runnable `mutation` entry has an
      empty plant.
- [ ] After seeding, the 31 controls of C-28 to C-40 and the control of CX-020 carry the gates of the last
      column (CX-020: GATE-MARROW-01 and GATE-MARROW-04, with plant `N13-18:M2`); every pending entry is in
      `seed_findings.toml` with `gate-added` requested until its construction maps to a gate.
- [ ] The five FACT entries (CX-029, CX-040, CX-060, CX-079, CX-093) carry `rerun = false` and seed as
      obligations; CX-034 carries its §22.9.2 case with `case_supplied_by` filled.
- [ ] A printout generated from the registry reproduces the first five columns of this table cell for cell (the
      registries are data and the tables are their printout).
- [ ] `scripts/e2e/registry_n10_cx.sh` exits 0.

## Test plan

### Unit tests

In `crates/frob-conformance/tests/`, under the laws-and-self-tests tier:
- `cx_chapter10_count`: exactly 79 entries in the range CX-019 to CX-097, none outside it with `defined_in` under
  chapter 10.
- `cx_ids_append_only`: the id set equals the frozen list; a removed or renumbered entry fails.
- `cx_entry_shape`: every entry has non-empty `shortcut`, `case`, `rule`, `rule_ids` and `plant`, a
  `control_kind` from the closed list, `rerun` present on the five FACT entries, and `case_supplied_by` filled
  on CX-034.
- `cx_kind_closed_list`: an unknown `control_kind` is rejected by the catalogue loader.
- `cx_planted_by_resolves`: every planting bead named in a `plant` exists in the bead graph export.
- `cx_seeded_once`: after seeding, exactly one control per entry with `seed` = its CX number and `kind` = its
  `control_kind`; a second seeding changes no byte of `controls.toml`.
- `cx_gates_exist`: every gate id in an entry's `rule_ids` is a row of the gate register `registries/gates.toml`;
  a misspelled gate (for example `GATE-AUGUR-016`) fails.
- `cx_gates_match_table`: after seeding, the gate lists of the controls of CX-067 to CX-097 and CX-020 equal the
  last column of the table above, entry by entry (for example CX-071 → {GATE-AUTOPSY-02, GATE-AUTOPSY-06},
  CX-072 → {GATE-ARENA-03, GATE-CORTEX-10}, CX-020 → {GATE-MARROW-01, GATE-MARROW-04}); a control that loses a
  gate, or a runnable control with an empty gate list, fails. This is the test that catches the "one gate
  removed" negative control below.
- `cx_kind_matches_table`: the `control_kind` of each of the 79 entries equals the "Control kind" column; an
  entry whose kind was changed from `mutation` to `planted-input` fails.
- `cx_fact_entries_are_obligations`: the controls of CX-029, CX-040, CX-060, CX-079 and CX-093 are `obligation`
  while `rerun = false`.
- `cx_printout_round_trip`: the generated table equals the stored table byte for byte.
- `cx_pending_report`: a pending entry (an open finding or an obligation) whose construction's milestone has
  passed is listed in the exit report.

### End-to-end script and logging

`scripts/e2e/registry_n10_cx.sh` (it calls only cargo targets of its dependencies, so no `frob` command): runs
`cargo test -p frob-conformance --test g0_laws -- cx_`; N38b-05's seeding tool, `cargo test -p frob-conformance
--test seed_controls -- --ignored seed`, twice (the second run must leave the digest of the committed
`controls.toml` unchanged), then its join, `cargo test -p frob-conformance --test seed_join`, for this chapter's
entries; regenerates the printout and compares it; lists obligations and open findings; then,
on five scratch copies of the catalogue under `target/e2e-scratch/registry_n10_cx/` (never the committed file),
plants the five negative controls below one at a time and asserts that each copy is rejected with the named
test. It sources the STD-E2E helper and writes `target/e2e-logs/registry_n10_cx/<utc>.ndjson` with a
transcript; each line has `ts`, `script`, `bead` (`N10-CX`), `step`, `event` (`start`, `cmd`, `assert`,
`metric`, `refusal`, `end`), the command line, exit code, wall time, the evidence block (claim Known answer,
class Tested, tier G0 host, determinism D0), the counts of entries and seeded controls against 79, of runnable
controls, obligations and open findings, the digest of `controls.toml` before and after the second seeding, and
on failure the first missing or duplicated entry with the registry path. The `summary` line counts assertions and names GATE-GAUNT-01.

### Known answers and negative controls

Known answers: the count 79; the range CX-019 to CX-097; the per-construction counts: C-01: 1, C-02: 1, C-03: 2,
C-04: 2, C-05: 1, C-06: 1, C-07: 3, C-08: 1, C-09: 3, C-10: 2, C-12: 2, C-13: 1, C-14: 1, C-15: 1, C-16: 3,
C-17: 4, C-18: 3, C-19: 4, C-20: 1, C-21: 2, C-22: 2, C-23: 1, C-24: 1, C-25: 1, C-26: 2, C-27: 2, C-28: 2,
C-29: 3, C-30: 2, C-31: 3, C-32: 4, C-33: 1, C-34: 2, C-35: 2, C-36: 3, C-37: 1, C-38: 2, C-39: 2, C-40: 4. C-11
has none. CX-035 is counted under C-10 and also names C-16. Negative controls for this bead: a catalogue with one
entry deleted (caught by `cx_chapter10_count` and `cx_ids_append_only`), with one entry duplicated (caught by the
catalogue loader's duplicate check and `cx_chapter10_count`), with one `control_kind` misspelled (caught by
`cx_kind_closed_list`), with one gate removed from the `rule_ids` of CX-072 (caught by `cx_gates_match_table`
after seeding), and with one gate id misspelled (caught by `cx_gates_exist`); each must fail its named test, and
the e2e script logs each planted copy, the test that failed and its first differing value.

## Considerations

- The catalogue is the authority and this table is its first printout; after loading, the table is regenerated,
  never edited by hand.
- A control that cannot be planted as described is a finding, not a pass; rule 4 reviews it at milestone exits.
- The pre-slip aliasing fixture (CX-012) belongs to the bets, not to chapter 10, and has no numbers anywhere in
  the plan; its numeric instance is owed by the energy-budget tier, and a proposal is recorded in the
  specification bead of construction C-31.
- The gate that counts rows with a missing tolerance or a missing control requires that count to be zero by M3
  for rows due by M3; seeding these records early is what lets the specification beads close that gap.
- Privacy: none of these fixtures uses a private scene.

## References

Plan Appendix N.1, N.7, N.35; §38.1, §38.11; GATE-GAUNT-01, GATE-GAUNT-02; INV-GAUNT-02, INV-GAUNT-07; PO-GAUNT-03;
ADR-GAUNT-02, ADR-GAUNT-03.
