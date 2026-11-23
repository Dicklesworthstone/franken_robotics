---
key: "SPEC-C-10"
title: "frob-gait: C-10 specified — gaits as (H, K) subgroup pairs, max-plus footfall timing; contract, law tests"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "reflex", "frob-gait", "frob-algebra", "frontier", "spine", "spec", "test"]
parent: "WP-64"
deps: ["BD-025", "N10a-01", "SPEC-C-09", "STD-TEST", "STD-E2E"]
needs: ["§10 C-16: hybrid orbits, saltation and the coupled solve for simultaneous touchdowns (SPEC-C-16, another unit)", "§33: the airlock entry for fs-tropical (max_cycle_mean, critical_circuit) and fs-spectral::floquet", "§22.9: the bead that creates crate frob-gait (BD-332 or BD-333 lands first; the law suite is created with the crate skeleton if neither has)"]
related: ["BD-332", "BD-333", "BD-335", "N10a-17", "CARD-T-25", "N10-KA1", "BD-049"]
gates: ["GATE-ATEL-02", "GATE-GAIT-01", "GATE-GAIT-03"]
plan_refs: ["§10 C-10", "§13.4", "§13.6", "§22.9", "§28.4 A-04", "§12.3 T-25", "Appendix D.2", "Appendix D.5", "Appendix M KA-072 KA-073", "Appendix N CX-034 CX-035"]
---
## Background

Construction C-10 replaces hand-tuned phase-offset tables per robot with two pieces of algebra. First, the
admissible gait types of a symmetric system are enumerated from its symmetry group: a periodic gait has a
group H of spatio-temporal symmetries (elements that map the orbit to itself up to a phase shift) and a normal
subgroup K of purely spatial symmetries, and H/K is cyclic (Buono and Golubitsky). Second, a footfall schedule
is a max-plus linear system whose eigenvalue is the stride period and whose eigenvector gives the steady phase
offsets (Lopes et al.).

Payoff:

- A gait atlas (analysis A-04): every (H, K) pair of the design's leg-permutation group, with phase relations,
  each usable as a clock for the baseline controller and as a reference for reward shaping.
- Periodic-orbit search restricted to symmetric trajectories has lower dimension: shooting on [0, T/m] inside
  the fixed-point subspace Fix(K) with the twisted boundary condition x(T/m) = h·x(0), where h generates H/K
  of order m and T is the period.
- Max-plus synthesis gives schedules with a guaranteed steady state and a bound on transition time between
  gaits.
- Orbits continued in design parameters give a screening signal and a warm start.

Tier: Frontier. Pilot: N10a-17. Fallback named in advance: the atlas as phase patterns and controller clocks,
without orbits.

Foundations (FACT, FrankenSim `483d09c682f2`, 2026-11-21; existence is not admission): `fs-tropical` has
`max_cycle_mean` and `critical_circuit`; `fs-spectral::floquet` analyses a supplied monodromy, including
non-normal transient growth. Nothing else exists.

## Scope

In: `docs/math/C-10.md`; the law suite `crates/frob-gait/tests/laws_c10.rs` (exhaustive enumeration over
subgroup lattices; max-plus eigenproblem on exact rationals); the quadruped atlas fixtures under two groups;
the trot schedule fixture; the register row; tolerances and controls for KA-072 and KA-073; the statement of
T-25.

Out: the atlas and its verb (BD-332); orbit computation, return maps and Floquet multipliers (BD-333,
construction C-16); continuation in morphology (BD-335); the pilot (N10a-17); the subgroup-lattice code itself
(BD-049), which this suite calls and independently re-derives.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-049 (orbit counts and
subgroup lattices in `frob-algebra`): that bead is listed in `related` and is not a prerequisite. The law suite is
generic over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-gait` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then. Until BD-049 lands, the calls the suite makes into the production
subgroup lattice run on the independent re-derivation in its test support.

## Technical approach

**Which group.** The ambient group for the body is the leg-permutation part of Γ_obs (the
observation-preserving symmetry group of the design and task), intersected with the isotropy subgroup of the
commanded velocity. That group is small: a quadruped commanded forward keeps only the left-right mirror C₂.
Under C₂, trot, pace and walk share the pair (C₂, 1), and bound and pronk share (C₂, C₂). The six primary
gaits are separated only by the symmetry group of a **declared leg-clock network**, ℤ₄ × ℤ₂ for the eight-cell
quadruped network of Golubitsky et al.; that group contains permutations that no isometry of the body
realizes, so it is a declared symmetry of the controller's coupling and not a subgroup of Γ_obs. Under
ℤ₄ × ℤ₂, trot, pace, bound, walk, pronk and the jump are distinct (H, K) pairs with prescribed phase lags.

**Enumeration rule** (the statement of theorem card T-25). The listed (H, K) pairs are exactly the pairs of
subgroups of the ambient group with K normal in H and H/K cyclic. A generator of H/K ≅ ℤ_m acts as a phase
shift of 1/m of the period; elements of K act with zero phase shift.

**The ℤ₄ × ℤ₂ table.** Write ω for the generator of ℤ₄ (it advances along each side of the network, hind to
fore) and κ for the generator of ℤ₂ (it exchanges left and right). Legs: left hind LH, right hind RH, left
fore LF, right fore RF. The group has eight subgroups: {e}; ⟨ω²⟩, ⟨κ⟩, ⟨κω²⟩ of order 2; ⟨ω⟩, ⟨κω⟩, ⟨ω², κ⟩ of
order 4; and the whole group. With H the whole group, the quotient is cyclic for exactly six choices of K
(DESIGN arithmetic by exhaustive check; the phase table is FACT from the literature, Golubitsky, Stewart,
Buono and Collins 1999, to be re-read from the primary source before a gate cites it, and is re-derived by
the enumeration):

| Gait | K | H/K | Phase of (LH, RH, LF, RF) in periods |
|---|---|---|---|
| pronk | ⟨ω, κ⟩ | trivial | (0, 0, 0, 0) |
| pace | ⟨ω⟩ | ℤ₂, κ ↦ ½ | (0, ½, 0, ½) |
| bound | ⟨ω², κ⟩ | ℤ₂, ω ↦ ½ | (0, 0, ½, ½) |
| trot | ⟨κω⟩ | ℤ₂, ω ↦ ½ and κ ↦ ½ | (0, ½, ½, 0) |
| jump | ⟨κ⟩ | ℤ₄, ω ↦ ¼ | (0, 0, ¼, ¼) |
| walk | ⟨κω²⟩ | ℤ₄, ω ↦ ¼ and κ ↦ ½ | (0, ½, ¼, ¾) |

The two remaining choices of K fail the rule: ⟨ω²⟩ gives quotient ℤ₂ × ℤ₂ and {e} gives ℤ₄ × ℤ₂, neither
cyclic. Over all H the group has 23 admissible pairs: 6 with H the whole group, 3 each for ⟨ω⟩ and ⟨κω⟩, 4 for
⟨ω², κ⟩, 2 each for the three subgroups of order 2, and 1 for the trivial group.

**The body-group table.** With the body group C₂ = {e, κ} alone the admissible pairs are (C₂, C₂), (C₂, 1) and
(1, 1), and the atlas reports trot, pace and walk as one class (left and right half a period apart) and bound
and pronk as one class (left and right in phase).

**Max-plus timing.** In the max-plus algebra a ⊕ b = max(a, b) and a ⊗ b = a + b, with ε = −∞ the zero. A
footfall schedule is x(k + 1) = A ⊗ x(k), where x(k) collects event times of stride k. The eigenproblem
A ⊗ x = λ ⊗ x has, for irreducible A, a unique eigenvalue λ equal to the maximum cycle mean of A (the largest
average weight of a circuit in the precedence graph). λ is the stride period; an eigenvector gives the steady
phase offsets.

**Trot schedule fixture** (DESIGN constants, set here because the construction states the test without
numbers; exact rationals). Two leg groups (the diagonals), touchdown times t₁, t₂. Flight time τ_f = 1/5 s,
double-stance time τ_Δ = 1/20 s, minimum stance time τ_g = 3/10 s. Events: a group lifts off no earlier than
τ_g after its touchdown and no earlier than τ_Δ after the other group's touchdown, and touches down τ_f after
lift-off. Eliminating lift-offs:
t₁(k + 1) = max(t₁(k) + τ_g + τ_f, t₂(k) + τ_Δ + τ_f);
t₂(k + 1) = max(t₂(k) + τ_g + τ_f, t₁(k + 1) + τ_Δ + τ_f).
So A = [[τ_g + τ_f, τ_Δ + τ_f], [τ_g + 2·τ_f + τ_Δ, max(τ_g + τ_f, 2·τ_Δ + 2·τ_f)]] = [[1/2, 1/4], [3/4, 1/2]].
Circuits: the two self-loops of weight 1/2 and the two-cycle of mean (1/4 + 3/4)/2 = 1/2. Hand result: stride
period λ = max(τ_g + τ_f, 2·(τ_f + τ_Δ)) = 1/2 s; eigenvector x = (0, 1/4), the second group half a stride
after the first; duty factor τ_g/λ = 3/5.

Variant showing a failure case: τ_g = 2/5 s gives A = [[3/5, 1/4], [17/20, 3/5]], λ = 3/5, and the eigenvector
is not unique: x = (0, x₂) for every x₂ ∈ [1/4, 7/20]. The critical graph has two components, and the phase
offset is then not determined by timing alone. The contract states that the eigenvector is unique up to an
additive constant only when the critical graph is strongly connected, and the atlas reports the interval
otherwise.

**Honest limits.**

1. The enumeration lists admissible symmetry types, not realized gaits.
2. An orbit computed inside Fix(K) can hide unstable multipliers transverse to it, so the monodromy is always
   computed on the full space.
3. Trot, bound and pronk have simultaneous touchdowns by construction; simultaneity is handled by a coupled
   solve and is never a reason to refuse a gait.
4. FACT (source register, to be re-read): published full-order orbit solves take about 4 to 260 s of CPU at 20
   to 68 states, so an orbit is a screening signal for periodic gaits on regular ground and not a
   referee-ready controller.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-10.S1 | The listed (H, K) pairs are exactly the pairs with K normal in H and H/K cyclic (T-25) | Certified | Exhaustive enumeration of the subgroup lattice, re-derived by the checker |
| C-10.S2 | Under ℤ₄ × ℤ₂ the six primary gaits are the six pairs with H the whole group, with the phase table above | Certified (pairs); FACT to re-read (names against the literature) | Enumeration; phase from the image of ω and κ in H/K |
| C-10.S3 | Under the body group C₂ the pairs are (C₂, C₂), (C₂, 1), (1, 1), and trot, pace and walk are one class | Certified | Enumeration |
| C-10.S4 | The max-plus eigenvalue of the trot schedule equals the stride period computed by hand, and the eigenvector gives the steady phase offsets | Certified | Exact rational maximum cycle mean and eigenvector check A ⊗ x = λ ⊗ x |
| C-10.S5 | Shooting on [0, T/m] in Fix(K) with x(T/m) = h·x(0) reproduces the full-period orbit | Tested | On a known periodic fixture |
| C-10.S6 | A transverse instability hidden by restriction to Fix(K) is found by the full-space monodromy | Tested | Planted fixture |
| C-10.S7 | A gait with simultaneous touchdowns is never refused for simultaneity alone | Tested | Trot, bound, pronk fixtures |

**What it does not claim.** That a listed gait exists as a stable orbit for a given body; that a max-plus
schedule is dynamically feasible. Ceiling stated before measurement: the cost of one orbit is between about 1%
and 63% of a 410 s quadruped training run (4 s and 260 s; FACT, source register), so the atlas replaces
training only as a screen. Falsifier of the pilot (TARGET): orbit ranking of ten design variants disagrees with
rollout ranking at Kendall τ below 0.5.

**Implementation map.** Crates `frob-algebra` (lattice, pairs) and `frob-gait` (atlas, schedules, orbits);
max-plus matrices come from `fs-tropical` through the airlock. Verb: `gait.atlas` (enumeration of gaits as
(H, K) pairs with phase lags). Gates: the A-04 row of GATE-ATEL-02 (M4); GATE-GAIT-01 (the planted transverse
instability is found and the restricted computation is marked incomplete; M4); GATE-GAIT-03 (Kendall τ of at
least 0.5 between orbit cost of transport and specialists on the calibration set; M4). Card: T-25. Kill gates
that concern its consumers: KILL-ATLAS, KILL-GAIT.

## Acceptance criteria

- [ ] `docs/math/C-10.md` exists with the group choice, the enumeration rule, both tables, the max-plus
      definitions, the trot fixture and its variant, the four limits and the seven statements.
- [ ] `cargo test -p frob-gait --test laws_c10` passes on the exact reference.
- [ ] ℤ₄ × ℤ₂: 8 subgroups, 23 admissible pairs, 6 with H the whole group; the phase table reproduces.
- [ ] C₂: exactly three pairs; C₂ × C₂: exactly eleven pairs.
- [ ] Trot schedule: λ = 1/2 s exactly and x = (0, 1/4); the variant reports the interval [1/4, 7/20].
- [ ] The planted transverse instability is reported and the restricted result is marked incomplete.
- [ ] KA-072 and KA-073 registered with tolerance `exact` and controls set; the register row of C-10 filled.

## Test plan

### Unit tests

- `s1_enumeration_complete`: for C₂, C₂ × C₂, ℤ₄ × ℤ₂, D₃, D₆: the pairs produced equal the pairs found by
  brute force over all subsets closed under the group law; counts 3, 11, 23 for the first three.
- `s2_primary_gaits`: the six K subgroups and their phase vectors.
- `s3_body_group_classes`: trot, pace and walk map to (C₂, 1); bound and pronk to (C₂, C₂).
- `s4_trot_eigen`: A = [[1/2, 1/4], [3/4, 1/2]] → λ = 1/2, x = (0, 1/4), verified by A ⊗ x = λ ⊗ x in
  rationals; `s4_nonunique_eigenvector`: the variant.
- `s4_agrees_with_fs_tropical`: the airlock's `max_cycle_mean` returns the same λ on both matrices.
- `s5_twisted_shooting`: a planar oscillator pair with exchange symmetry; half-period shooting with the swap
  as h reproduces the full orbit within 10⁻⁹.
- `s6_hidden_transverse_instability`: two identical unstable-in-antiphase oscillators; the restricted
  monodromy is stable, the full one has a multiplier outside the unit circle.
- `s7_simultaneity_not_refused`: trot schedule with exactly simultaneous diagonal touchdowns is accepted.

### End-to-end script and logging

`scripts/e2e/spec_c10.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G1 rows KA-072
and KA-073; `frob --json gait atlas --design templates/quad-mammal --network z4xz2` asserting six primary
gaits with their phase vectors, and the same without a declared network asserting three pairs and the merged
classes; evidence block class `Certified`, determinism D0. Log `target/e2e-logs/spec_c10/<utc>.ndjson` with
transcript: commands, exit codes, pair counts against expected, λ with unit against 1/2 s, and on failure the
first pair missing or extra. Final `summary` line.

### Known answers and negative controls

- KA-072 quadruped gait atlas: exact; G1 · host. Control (set here): the cyclic-quotient test replaced by an
  abelian-quotient test, which admits (whole group, ⟨ω²⟩) and reports seven pairs with H the whole group.
- KA-073 max-plus timing: exact; G1 · host. Control (set here): the eigenvalue computed as the largest entry
  of A, which gives 3/4 s instead of 1/2 s.
- CX-034 (stability judged inside Fix(K)); CX-035 (refusing or ordering simultaneous impacts).
- Further control: normality of K in H not checked (invisible for abelian groups, caught on D₃ and D₆).
- Determinism: exact, D0.

## Considerations

- The network group is declared by the controller, not derived from the body; the atlas output must label
  which group produced each pair, or a reader will take clock-network gaits for body symmetries.
- Risk: reading the atlas as a list of gaits the robot can perform. Limit 1 is printed with every atlas.
- Reducible schedules (a leg group that never waits for another) have several eigenvalues; the contract
  requires irreducibility for the uniqueness statement and reports components otherwise.
- The primary-gait names come from the literature and are to be re-read; the enumeration itself does not
  depend on them.

## References

Plan §10 C-10; §13.4, §13.6; §22.9; §28.4 (A-04); §12.3 (T-25); Appendix D.2 (return map), D.5 (H/K theorem,
max-plus eigenproblem); Appendix M KA-072, KA-073; Appendix N CX-034, CX-035. Pilot: N10a-17.
