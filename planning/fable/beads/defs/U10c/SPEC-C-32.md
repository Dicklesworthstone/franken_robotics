---
key: "SPEC-C-32"
title: "spec C-32 core: dual witnesses of infeasibility and the stance family in inverse geometry, exact specimen"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "frontier", "augur", "frob-mission", "frob-cert", "test"]
parent: "WP-42"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-042", "BD-115", "BD-130", "BD-132", "BD-133", "BD-382", "N10c-04", "N10-CX", "SPEC-C-40", "N10c-10", "CARD-T-21", "CARD-T-22", "BD-128", "BD-026", "N10a-02"]
gates: ["GATE-AUGUR-02", "GATE-LEMMA-03"]
plan_refs: ["§10 C-32", "§25.13", "§25.5", "§12.3 T-21 T-22", "§12.4", "§12.7", "Appendix D.6", "Appendix E GB-31", "Appendix I.4", "Appendix M.8", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-32 of the FrankenRobotics spine is "region certificates and dual witnesses of infeasibility". It
supplies two kinds of evidence about a whole family of designs: an outward-rounded enclosure that decides one
constraint on a box of design parameters, and a dual vector that proves a relaxation infeasible. Tier Frontier
[F]. Owner: the Augur subsystem (mission queries and inverse design), crates `frob-mission` (which proposes) and
`frob-cert` (the offline auditor `frob-verify`, which re-checks with the standard library alone).

This bead is the core specification: the dual witnesses and the planar stance family that is their worked
specimen, needed by the first mission slice at M1. The enclosure side (region certificates by interval, affine and
Taylor arithmetic, and the limits of paving) is bead N10c-04 at M3, which extends the same contract file.

What it replaces: a static margin computed for one design, and dense sampling presented as a universal claim.
Payoff: a whole family of stance dimensions and force ratings from one solve, with a boundary design and its force
witnesses, or the load vertex that defeats it; and a refusal that cites a separating vector tells the agent which
constraint binds. The same dual object is the separating witness returned when a task wrench lies outside a body's
attainable set, so a skill is refused before any training.

Design reasoning and rejected shortcuts. (1) A support-polygon test with a total-capacity check is no substitute
for the exact model: FACT (planning pilot, to be re-run in this repository) it accepted 1,596 bodies the exact
model rejects; the first example is in the test plan. (2) A result must state its authority model: an inclusion
proved for independently commanded forces is false for a passive suspension. (3) No speed is claimed for the
specialized projection: FACT (recorded pilot, to be re-run) a lifted robust linear program and the projection each
reached the optimum in one solve, so the lifted program is the first implementation and a specialized kernel must
be earned with real timings. (4) Only a strict conic witness is accepted; a non-strict one yields `unknown`.

Terms. *Witness*: explicit forces that satisfy the primitive equations at a load vertex. *Dual witness*: a vector
that proves no such forces exist. *Authority model*: the declared law by which forces can be commanded
(`independent-normal-force`, `passive-equal-spring`). *Load vertex*: a corner (W, P_x, P_y) of the payload
envelope, W the vertical load and P the first moment of the load about the footprint centre. Proof classes:
Proved (sealed Lean capsule), Certified (instance certificate re-checked by the auditor in exact rationals),
Tested, Estimated. Status today: HYPOTHESIS.

## Scope

In: `docs/math/C-32.md` parts A (dual witnesses) and B (stance family and specimen); law tests in
`crates/frob-mission/tests/laws_c32.rs`, with the auditor-side rows bound in `crates/frob-cert/tests/`; registry
entries for KA-157 to KA-160, KA-033, the statements of theorem cards T-21 and T-22 as contract clauses, and the
controls seeded from CX-077, CX-078, CX-079 and CX-016.

Out: the stance-family solver and the verb path (`mission.synthesize`, `mission.refute`; BD-130, BD-135); the
checkers for `lp-primal` and `lp-farkas` (BD-132); bundle writing (BD-133); the cone-program solver that produces
duals (BD-115); the Lean statement of Farkas soundness and the reflectors (BD-382); region certificates by
enclosure (N10c-04); recording GATE-AUGUR-02 and GATE-LEMMA-03.

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-32: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`mission.refute`; also `mission.synthesize`,
`prove.bundle`, `prove.verify`), `failing_test` (a test of `frob-mission::laws_c32` with its registered control),
`payoff.gate` (GATE-AUGUR-02), `statements`, `known_answers` (KA-157 to KA-160), `cards` (T-21, T-22), `gates`,
`pilot` (the record `PILOT-C-32` of the pilot register, required because the tier is Frontier) and
`generic_comparator` (the off-the-shelf method named in the pilot paragraph of this bead). Doctrine D-2 (no
ornamental mathematics) is checked on these fields: a construction with no verb or no failing test is moved to the
parked register by a decision record. The part bead N10c-04 adds their statements and tests to the same row. The
contract `docs/math/C-32.md` has the ten sections of the template `docs/math/TEMPLATE.md` of the same bead, in the
template's order (object and definitions; what it replaces and the payoff; conventions; statements; formulas;
fixtures and known answers; failure cases, refusals and retirement; non-claims; foundations; implementation map);
content that this bead lists under other headings goes into the template section that matches. Statement
identifiers are entered in the contract and in the register with the construction prefix (`C-32.S1` and so on, as
the template requires); the tables of this bead and of its part beads abbreviate them.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-128 (the `frob-mission`
core) or BD-026 (the auditor crate `frob-cert`): those beads are listed in `related` and are not prerequisites.
The law suite is generic over its subject trait and closes on the exact reference model and the mutants in its own
test support. If `frob-mission` or `frob-cert` do not exist when this bead starts, the suite and its support files
are written under `crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and
the commands below read `-p frob-conformance`; the bead that implements the construction moves them into the
owning crate, binds its production types to the subject trait and runs the unchanged suite as part of its own
acceptance. A criterion or script step below that names production code or a verb of an implementing bead is owed
at that bead's acceptance and is logged `pending` here until then.

## Technical approach

### A. Dual witnesses

| Id | Statement | Class |
|---|---|---|
| A1 | Farkas: the linear relaxation {x ≥ 0, M·x = r} is infeasible if some y has Mᵀy ≥ 0 and rᵀy < 0. Reason: for a feasible x, rᵀy = xᵀ(Mᵀy) ≥ 0. Checking a given (M, r, y) is two exact rational matrix–vector products. | General statement: Proved once its Lean capsule is sealed (theorem card T-21, [F]); instance: Certified (proof kind `lp-farkas`) |
| A2 | Conic: for {x ∈ 𝒦, M·x = r} the witness is y with Mᵀy ∈ 𝒦* (the dual cone) and rᵀy < 0. Only a strict witness is accepted; where none exists the verdict is `unknown`. | Certified per instance (proof kind `conic-farkas`) |
| A3 | The same dual object is the separating witness returned when a requested task wrench lies outside a body's attainable set. | Structural; Tested |
| A4 | An infeasible sample point does not eliminate its cell; an exclusion needs a dual that covers the cell (for example a parametric dual with a sign proof). | Rule, Tested by the excluded-cell case |

### B. The stance family

Planar contact template: points qᵢ are mapped to feet pᵢ = A·qᵢ + d; the total vertical load is W and its first
moment is P. Write B = A⁻¹ and e = −B·d. From Σ pᵢ·fᵢ = P it follows that

```text
Σ fᵢ = W        Σ qᵢ·fᵢ = B·P + e·W        0 ≤ fᵢ ≤ cᵢ
```

which is linear in the inverse geometry (B, e), the ratings c and the forces f, when the mission vertices (W, P)
are fixed independently of the design.

| Id | Statement | Class |
|---|---|---|
| B1 | The identity Σ qᵢ·fᵢ = B·P + e·W holds for every invertible A. | Tested (exact rationals); a polynomial identity |
| B2 | The feasible family is the projection of a polyhedron. *Lifted form*: one force vector per mission vertex with shared (B, e, c), a robust linear program. *Projected form*: with generators gᵢ = (1, qᵢ) of the wrench zonotope Σ [0, cᵢ]·gᵢ, for each pair with n = g_j × g_k ≠ 0 and each mission vertex w, Σᵢ min(0, cᵢ·n·gᵢ) ≤ n·w ≤ Σᵢ max(0, cᵢ·n·gᵢ); at most 2·V·C(m, 2) inequalities for m contacts and V load vertices. | Tested (the two forms agree on the validation grid) |
| B3 | Specimen (theorem card T-22): 400 N on four point feet at (±a, ±b) with a = 1/3 m; centre of mass anywhere in a rectangle of half-sizes 150 mm and 75 mm; each foot commanding any force in [0, 160] N; the allocator observes the load before allocating. The minimum half-width is b = 3/14 m. | Certified (witness replay and Farkas check in exact rationals) |
| B4 | For a fixed design the set of balanced wrenches is convex, so feasibility at the four vertices covers the rectangle: the bilinear interpolant of the four vertex witnesses is a valid allocation law. No single affine allocation law attains b = 3/14 m; the affine class needs b = 1/2 m. | Certified for the vertices; Tested for the affine statement |
| B5 | Authority: the same body is inner under `independent-normal-force` and excluded under `passive-equal-spring`; every typed result states its authority model and the assumption travels with the certificate. | Tested |
| B6 | Sign and type: with +z up, the support torque of vertical foot forces is (P_y, −P_x, 0); a first moment passed where a torque is expected is rejected. | Tested (auditor type rule) |

Derivation of B3, to be reproduced in the contract. Order the feet [++, +−, −+, −−]; write u = 1/a, v = 1/b. The
envelope is the hull of the vertices (W, P_x, P_y) = (400, ±60, ±30) N and N·m. Put r = (W + u·P_x)/2, the total on
the two +x feet, and s = (W + v·P_y)/2, the total on the two +y feet. With t = f₊₊ the solutions are
f = (t, r − t, s − t, W − r − s + t), and a vertex is feasible exactly when

```text
max{ 0, r − c₊₋, s − c₋₊, r + s − W }  ≤  min{ c₊₊, r, s, c₋₋ + r + s − W }.
```

At (400, 60, 30) with u = 3 and ratings 160 N: r = 290, s = 200 + 15·v, and the binding pair is t ≥ 90 + 15·v
(from f₋₋ ≥ 0) against t ≤ 160. So f₊₊ − f₋₋ = 90 + 15/b, and the vertex is feasible exactly when v ≤ 14/3.

| Vertex P (N·m) | Forces [++, +−, −+, −−] at b = 3/14 m (N) |
|---|---|
| (60, 30) | (160, 130, 110, 0) |
| (60, −30) | (130, 160, 0, 110) |
| (−60, 30) | (110, 0, 160, 130) |
| (−60, −30) | (0, 110, 130, 160) |

Inner cell: v ∈ [2, 14/3] with the affine-in-v witness (160, 130, 40 + 15·v, 70 − 15·v) at vertex (60, 30) and its
reflections; checking the two ends checks the cell. Excluded cell: v ∈ (14/3, 6], by the multipliers (0, 1/2, 1/2)
on (force balance, x-moment divided by a, y-moment divided by b), which give f₊₊ − f₋₋ = 90 + 15·v > 160. At
b = 1/5 m this is 165 N against f₊₊ ≤ 160 N and f₋₋ ≥ 0; in standard Farkas form, with the rating row of the ++
foot appended, y = (0, −1/2, −1/2, 1) gives Mᵀy ≥ 0 and rᵀy = −90 − 75 + 160 = −5 < 0. Authority case at b = 1/4 m:
f₊₊ may be anything in [150, 160] N and the active allocation (155, 135, 105, 5) N balances vertex (60, 30); four
equal passive springs add f₊₊ + f₋₋ = f₊₋ + f₋₊ and give (175, 115, 85, 25) N, 15 N over the rating. Fixture sizes:
lifted form 22 variables, 12 equalities, 16 inequalities; projected form 6 variables, 32 distinct inequalities.

### Conventions, failure cases and non-claims

- A boundary design has zero reserve on two constraints; a positive force floor for continuous contact is a
  different, declared query, and an unreported force floor is a rejected certificate.
- A static projection certifies neither run-up nor landing nor recovery. General lateral friction, contact switches
  and coupled torque limits need other owners. A body mass that changes with the design breaks convexity.
- Every field of a bundle is typed with frame, origin, sign convention and unit; the primitive equations travel in
  the bundle. The four registered mutations of the specimen bundle are each rejected: a swapped foot order with
  unchanged forces; an unreported positive force floor; a passive-authority tag; (60, 30) passed as the torque.
- Foundations (FACT, observed 2026-11-21): FrankenSymPy has exact rational linear algebra with certificates;
  `fs-sos` exists; the published basis is static-equilibrium projection and actuation-aware feasible regions.

### Law-test layout, registry, implementers and gates

`crates/frob-mission/tests/laws_c32.rs` with `tests/support/c32_ref.rs`: the one-parameter interval test, the
zonotope support test, the Farkas check and a tiny exact feasibility oracle, all in exact rationals. The suite is
generic over a trait `C32Subject` (decide a vertex; return a witness or a dual). Auditor rows are bound in
`crates/frob-cert/tests/` by feeding the reference witnesses and duals to the checkers of BD-132; the producer
never calls the auditor's decision code to build a witness. Mutants in `tests/support/c32_mutants.rs`. Registry:
KA-157 to KA-160 and KA-033 with derivation files; controls from CX-077, CX-078, CX-079, CX-016.

Implementers: `frob-mission` (BD-130), `frob-cert` (BD-132), `frob-lemma` (bundles, BD-133). Gates: GATE-AUGUR-02
at M1 (boundary exactly 3/14 m; four vertex witnesses; the dual (0, 1/2, 1/2) for the excluded cell; 11,520 of
11,520 grid states agree with an independent zonotope test; lifted and projected forms agree; the passive-spring
amendment withdraws the cell at b = 1/4 m) and GATE-LEMMA-03 at M1 (the four stance mutations rejected). The
specimen bundle is also part of GATE-LEMMA-04 and GATE-AUGUR-05.

## Acceptance criteria

- [ ] `docs/math/C-32.md` exists with parts A and B: A1 to A4, B1 to B6, the full derivation, the vertex table,
      both cells, the authority case, the fixture sizes, the conventions and the non-claims.
- [ ] `cargo test -p frob-mission --test laws_c32` passes on the reference model, and on the production stance
      solver if BD-130 has landed; the auditor rows pass in `frob-cert` if BD-132 has landed.
- [ ] Every mutant below fails its named law.
- [ ] KA-157 to KA-160 and KA-033 are registered with derivation files; the counts 2,254, 114 and 1,596 are
      recomputed in this repository and recorded as re-run.
- [ ] Controls from CX-077, CX-078, CX-079 and CX-016 are attached to GATE-AUGUR-02 and GATE-LEMMA-03.
- [ ] `scripts/e2e/spec_c32.sh` exits 0.
- [ ] Row C-32 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `farkas_soundness_property`: 10⁴ seeded rational systems with at most 4 variables; whenever a y passes the
  check, the exact oracle finds no feasible x; a y with one sign flipped is rejected.
- `conic_strict_and_weak`: second-order cone in ℝ³; {x₀ = −1} has the strict witness y = 1; {x₀ = x₁, x₂ = 1} is
  infeasible with no strict witness and returns `unknown`.
- `template_identity`: 10⁴ seeded invertible A, d, qᵢ, fᵢ: Σ qᵢ·fᵢ equals B·P + e·W exactly.
- `ka_157_stance_boundary`: minimum half-width 3/14 m; the four vertex witnesses satisfy Σ f = 400,
  a·(f₊₊ + f₊₋ − f₋₊ − f₋₋) = P_x and b·(f₊₊ − f₊₋ + f₋₊ − f₋₋) = P_y exactly.
- `inner_cell_affine_witness`: (160, 130, 40 + 15·v, 70 − 15·v) is feasible at v = 2 and v = 14/3.
- `ka_158_dual_witness`: at b = 1/5 m the combination gives 165 N; the standard-form y has Mᵀy ≥ 0 and rᵀy = −5;
  the perturbed multipliers (0, 1/2, 2/5) fail Mᵀy ≥ 0 and are rejected.
- `ka_159_authority`: at b = 1/4 m active allocation (155, 135, 105, 5) passes; equal springs give
  (175, 115, 85, 25) and exceed 160 N.
- `ka_160_support_torque_sign`: forces (160, 130, 110, 0) at b = 3/14 m give support torque (30, −60, 0) N·m; a
  packet carrying (60, 30) in a torque field is rejected.
- `validation_grid`: u, v in {2, 3, 4} per metre and four ratings in {0, 80, 160, 240} N give 2,304 bodies and,
  with the four vertices and their centroid, 11,520 states: the interval test and the zonotope test agree on all;
  2,254 states are feasible; 114 bodies cover the mission.
- `support_polygon_screen_is_wrong`: the screen "centre of mass inside the support polygon and total capacity at
  least the weight" accepts 1,596 bodies the exact model rejects; first example u = v = 2, ratings
  (80, 80, 80, 160) N, vertex (60, 30), where t would need to be at least 180 N and at most 80 N.
- `affine_law_needs_half_metre`: an affine allocation law in the load exists at v = 2 (f₊₊ = 100 + 45·x + 15·y
  with x = P_x/60, y = P_y/30) and none exists for v > 2; the bilinear interpolant works at v = 14/3.
- `ka_033_lifted_equals_projected`: the lifted program and the projection return the same optimum 3/14 m.

### End-to-end script and logging

`scripts/e2e/spec_c32.sh`: `frob --json doctor`; `cargo test -p frob-mission --test laws_c32`; `cargo test -p
frob-conformance --test g1 -- KA-157 KA-158 KA-159 KA-160`; `cargo test -p frob-conformance --test g2 -- KA-033`;
`frob --json prove list` filtered to cards T-21 and T-22; when bundles exist, `frob-verify` on the specimen bundle
and on its four mutations, expecting accept and four rejects. Log to `target/e2e-logs/spec_c32/<utc>.ndjson` with
a transcript; each line has `ts`, `script`, `bead`, `step`, `event`, command line, exit code, wall time, the
evidence block (claim Known answer, class, tier G1 auditor or host, determinism D0, seed), forces and multipliers
as rationals against expected, the three grid counts, and on failure the first differing state with its body
index. The `summary` line counts assertions and names GATE-AUGUR-02 and GATE-LEMMA-03.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-157 | f₊₊ − f₋₋ = 90 + 15/b; minimum half-width 3/14 m; forces (160, 130, 110, 0) N | exact (rationals) | G1 auditor | The four bundle mutations, each rejected |
| KA-158 | At b = 1/5 m the dual gives f₊₊ − f₋₋ = 165 N against 160 N | exact | G1 auditor | A perturbed multiplier is rejected |
| KA-159 | At b = 1/4 m active (155, 135, 105, 5) N; passive (175, 115, 85, 25) N | exact | G1 host | Mutant `authority_ignored`: carries the inclusion to the passive suspension |
| KA-160 | Support torque (P_y, −P_x, 0); (30, −60, 0) N·m | exact | G1 auditor | A packet that passes (60, 30) as the torque |
| KA-033 | Lifted program and projection reach the same optimum in one solve each (FACT, recorded pilot; re-run here) | exact | G2 host | Mutant `projection_drops_a_pair` (new): omits one generator pair |

Other mutants: `support_polygon_screen`; `weak_conic_witness_accepted`; `sample_point_excludes_cell`.
Counterexamples carried: CX-077 (first moment passed as torque), CX-078 (authority), CX-079 (support polygon plus
capacity), CX-016 (a specialized method assumed to beat the generic one). Determinism: exact rationals, twice on
one machine, and bundle bytes identical across two hosts when bundles exist.

## Considerations

- Open question OPEN-AUGUR-2 (linear programs exactly over ℚ, or in floating point with rational reconstruction):
  default floating point in the cone solver, basis reconstruction over ℚ, exact check; a failed reconstruction
  leaves the cell unresolved. Owner the Augur lead; decided at GATE-AUGUR-02 from the reconstruction failure rate.
- Kill gates that bear on the mechanism, not on this contract: KILL-FAMILYCERT and KILL-AUGUR-TIE (a tie with the
  generic comparator ships the generic method).
- The counts 2,254, 114 and 1,596 are FACT from a planning pilot; this bead's grid test is their re-run, and a
  disagreement is recorded, not hidden.
- The Farkas vector is printed in two conventions (multipliers on the equalities, and standard form); the contract
  states both so that a bundle and a hand check cannot disagree by a sign.

## References

Plan §10 C-32; §25.13; §25.5; theorem cards T-21, T-22; proof kinds `lp-primal`, `lp-farkas`, `conic-farkas`;
Appendix D.6 "Farkas witness", "Stance family"; GB-31; KA-157 to KA-160, KA-033; CX-077, CX-078, CX-079, CX-016;
GATE-AUGUR-02, -05; GATE-LEMMA-03, -04; ADR-AUGUR-03; D-17; OPEN-AUGUR-2; BET-19, BET-20; C-13, C-17.
