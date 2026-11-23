---
key: "SPEC-C-34"
title: "spec C-34 core: linkage synthesis, lane 1 exact and bounded: dyads, capped templates, circuit certificate"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "spine", "spec", "frontier", "augur", "atelier", "frob-synth", "test"]
parent: "WP-67"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-060", "BD-105", "BD-132", "BD-361", "N10c-05", "N10-CX", "N10c-10", "BD-360"]
gates: ["GATE-AUGUR-17"]
plan_refs: ["§10 C-34", "§10 C-11", "§25.21.3", "§12.4", "Appendix D.6", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-34 of the FrankenRobotics spine is "linkage synthesis: an exact bounded lane and a continuation
lane". It turns the algebra of kinematics (construction C-11: forward kinematics is a polynomial map, so solution
sets are real algebraic varieties) into a design tool for parallel legs, ankles and steering linkages. It has two
lanes. This bead is the core specification and covers lane 1, exact and bounded, tier Frontier [F]. Lane 2,
numerical continuation, tier Moonshot [M], is bead N10c-05, which extends the same contract file. Owner: the Augur
subsystem's constructive lanes, crate `frob-synth`.

What it replaces: random mutation of link lengths, and multistart numerical closure with no statement of
branches. Payoff: constructive design, with a certificate that catches circuit and branch defects, a real class
of error in linkage design. The key fact behind the design: **a root of the synthesis equations is not a
mechanism.** A linkage can satisfy every precision-pose equation and still be unable to move between its poses
without disassembly, because the poses lie on different assembly circuits. Three-pose synthesis has textbook
closed forms, so the value of lane 1 is the certificate and not the solve; no speed claim is made against a
competent baseline.

Bounds are facts of the symbolic sibling, not choices. FACT (`frankensympy/crates/fsym-solvers/src/real_rur.rs`,
`fsym-polys/src/groebner.rs`, observed 2026-11-21 at `29f847c1`): the real solver is capped at 8 variables, 64
equations, 4,096 terms, total degree 256, 4,096-bit coefficients, 2 × 10⁸ work units and 512 MiB, for
zero-dimensional ideals only; Gröbner bases are capped at 1,024 basis polynomials and 10⁶ pending pairs; there is
no homotopy solver. Unbounded elimination for a whole robot is therefore not attempted. Rejected alternative:
truncating a system that exceeds the caps; it is refused instead.

Terms. *Dyad*: a two-link chain with a fixed pivot O and a moving pivot P. *Precision pose*: a pose the mechanism
must reach exactly. *Assembly circuit*: a connected component of the configuration set of a closed chain.
*Branch point*: a configuration where the loop-closure Jacobian is singular. *Saturation*: adjoining
s·d(x) − 1 = 0 to exclude roots where d vanishes. Proof classes: Proved, Certified (re-checked by the offline
auditor in exact or interval arithmetic), Tested, Estimated; status HYPOTHESIS.

## Scope

In: `docs/math/C-34.md` part A (lane 1); law tests in `crates/frob-synth/tests/laws_c34_exact.rs`; registry
entries for KA-164 and KA-165 and the control seeded from CX-082.

Out: the synthesis strategy `linkage` through the derivation subprocess and the `.fmodule` output (BD-361); the
derived leg inverse-kinematics kernel and admission of the symbolic sibling's solver (BD-105); the auditor's
`interval-box` and `partition` checkers (BD-132 and successors); lane 2 (N10c-05, BD-437); recording the linkage
rows of GATE-AUGUR-17 and deciding KILL-SYNTH-LINKAGE (BD-361).

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-34: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`mission.synthesize` with strategy `linkage`;
`continuation` for lane 2), `failing_test` (a test of `frob-synth::laws_c34_exact` with its registered control),
`payoff.gate` (GATE-AUGUR-17), `statements`, `known_answers` (KA-164 to KA-167), `cards` (T-34), `gates`, `pilot`
(the record `PILOT-C-34` of the pilot register, required because the tier is Frontier) and `generic_comparator`
(the off-the-shelf method named in the pilot paragraph of this bead). Doctrine D-2 (no ornamental mathematics) is
checked on these fields: a construction with no verb or no failing test is moved to the parked register by a
decision record. The part bead N10c-05 adds their statements and tests to the same row. The contract
`docs/math/C-34.md` follows the ten-section template `docs/math/TEMPLATE.md` of the same bead.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-360 (the co-design lane, the
first seed in `frob-synth`): that bead is listed in `related` and is not a prerequisite. The law suite is generic
over its subject trait and closes on the exact reference model and the mutants in its own test support. If
`frob-synth` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### 1. Statements of lane 1

| Id | Statement | Class |
|---|---|---|
| L1 | Three-pose planar dyad: a moving pivot with positions P₁, P₂, P₃ in the three poses has its fixed pivot O at their circumcentre, the solution of the 2×2 rational system 2·(Pᵢ − P₁)·O = ‖Pᵢ‖² − ‖P₁‖² for i = 2, 3 (obtained by eliminating the crank radius from ‖Pᵢ − O‖² = ‖P₁ − O‖²). Degenerate cases are split by the rank of its determinant: rank 2, one pivot; rank 1 (distinct collinear positions), no finite pivot, reported as degenerate; rank 0, coincident positions. | Certified by substitution in exact rationals |
| L2 | Five-pose Burmester problems and four- and five-bar templates are posed as loop-closure or Study-quadric polynomials and solved inside the caps of the real solver, with a saturation equation s·d(x) − 1 = 0 that excludes zero-length and collinear roots. Only zero-dimensional systems inside the caps are submitted; a system outside them is refused, not truncated. | Tested; each isolated root is Certified by interval contraction |
| L3 | Circuit and branch certificate, for every candidate: real roots isolated by interval contraction; one assembly circuit that traverses all target poses in order; a loop-Jacobian determinant of constant sign along the path, so no branch point is crossed; no self-collision; the required reach. A tolerance box around the dimensions carries the certificate to as-built units. | Certified (proof kinds `interval-box` and `partition`) |
| L4 | For a planar four-bar with ground pivots O₂, O₄, crank pivot A and coupler point B, the branch indicator σ = (B − A) × (O₄ − A) vanishes exactly when A, B and O₄ are collinear, which happens only when the distance ‖A − O₄‖ equals the sum or the difference of the coupler and rocker lengths. If that distance never reaches either value over the crank range, σ has constant sign on each assembly circuit and the two circuits are disjoint. | Tested; per instance Certified by enclosure over the crank interval |
| L5 | Grashof condition: with shortest link s, longest l and the others p, q, s + l ≤ p + q lets the shortest link turn fully. | Textbook; Tested |
| L6 | A candidate whose target poses lie on different circuits is rejected, although it satisfies closure at every pose. | Tested (the circuit defect) |

### 2. Procedure the contract fixes

1. Solve the dyad or template system exactly (L1, L2); refuse a system outside the caps.
2. For each real root, isolate it in a box by interval contraction.
3. Evaluate the branch indicator at every target pose; reject if the signs differ.
4. Enclose the branch indicator over the path between consecutive poses, subdividing the driving interval until
   every sub-interval has an enclosure that excludes zero, or reject.
5. Check pose order, self-collision and reach on the same sub-intervals; repeat on the tolerance box.
6. Emit the linkage with its certificates; label the lane's result by what was certified.

### 3. Worked fixtures (hand-checkable)

- Circumcentre A: P = (1, 0), (0, 1), (−1, 0) give O = (0, 0) and radius 1. Circumcentre B (the owning chapter's
  row): P = (0, 0), (2, 0), (0, 2) give O = (1, 1) and radius² = 2. Collinear positions give determinant zero.
- Four-bar A: ground 4, crank 1, coupler 3.5, rocker 3. Grashof: 1 + 4 ≤ 3.5 + 3. With O₂ = (0, 0) and O₄ = (4, 0),
  ‖A − O₄‖² = 17 − 8·cos θ lies in [9, 25], so the distance lies in [3, 5] and never reaches 0.5 or 6.5: the crank
  turns fully and the two assembly circuits are disjoint.
- Four-bar B (the owning chapter's row): ground 4, crank 1, coupler 4, rocker 3; the distance never reaches 1 or 7.
  At crank angle 0 the closure equations give B = (11/3, ±√80/3); at crank angle π they give B = (11/5, ±12/5). A
  design whose precision poses are (crank 0, B above) and (crank π, B below) satisfies every synthesis equation
  and cannot be driven from one pose to the other without disassembly.

### 4. Conventions, failure cases and non-claims

- Certificates are checked by an exact rational or interval checker, never inside a proof kernel.
- Output of the mechanism: a linkage as a module file with its port contract, plus certificates of kinds
  `interval-box` and `partition` for circuit, branch, clearance and tolerance.
- Not claimed: completeness beyond the zero-dimensional systems submitted; any whole-robot elimination; any speed.

### 5. Law-test layout, registry, pilot and gate

`crates/frob-synth/tests/laws_c34_exact.rs` with `tests/support/c34_ref.rs`: the 2×2 rational solve with rank
split; four-bar closure by circle intersection with rational squared coordinates; the branch indicator and its
interval enclosure over crank sub-intervals; a cap check. Generic over a trait `C34ExactSubject`; mutants in
`tests/support/c34_mutants.rs`. Registry: KA-164 and KA-165 with derivation files (both circumcentre fixtures and
both four-bars, one fixture identity per physical case); the control from CX-082.

Pilot as the construction states it: 24 dyad and four-bar templates with planted circuit and branch defects;
falsifier: one defect certified as valid. The owning chapter's pilot is 240 template instances (three-pose dyads
with a lug catalogue and keep-in regions) against multi-start numerical optimization under the same checks;
falsifier: a certified linkage binds or changes circuit on the referee, or more than a quarter of the instances
fall outside the caps. Both are recorded in the pilot register; BD-361 runs them. Ceiling: better proposals for
one mechanism family at a time; the certificate, not speed. Fallback: multistart recovery on the CPU, labelled
`Partial`. Gate: the linkage rows of GATE-AUGUR-17 at M4 (the three-pose dyad and both circuit fixtures exact;
zero certified linkages change circuit on the referee across 240 instances). Kill gate KILL-SYNTH-LINKAGE (M4): a
certified linkage binds or changes circuit on the referee, or more than 25% of template instances exceed the
caps; fallback: multi-start numerical optimization with the same circuit and clearance check, classed Tested.

## Acceptance criteria

- [ ] `docs/math/C-34.md` exists with part A: L1 to L6, the six-step procedure, the caps with their source paths,
      the worked fixtures and the non-claims.
- [ ] `cargo test -p frob-synth --test laws_c34_exact` passes on the reference model, and on the production lane
      if BD-361 has landed.
- [ ] Every mutant below fails its named law.
- [ ] KA-164 and KA-165 are registered with derivation files; KA-164, printed with no control, has the control
      named below; the control from CX-082 is attached to GATE-AUGUR-17.
- [ ] Both pilot statements are in the pilot register with the difference between them noted.
- [ ] `scripts/e2e/spec_c34_exact.sh` exits 0.
- [ ] Row C-34 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `ka_164_circumcentre`: fixture A gives O = (0, 0), radius 1; fixture B gives O = (1, 1), radius² = 2; positions
  (0, 0), (1, 1), (2, 2) are reported degenerate with rank 1; three equal positions with rank 0.
- `dyad_equidistance_property`: 10⁴ seeded non-collinear rational triples: ‖Pᵢ − O‖² are equal exactly.
- `saturation_excludes_degenerate_root`: x·(x − 1) = 0 with d(x) = x: the saturated system has the single root
  x = 1, s = 1.
- `ka_165_grashof_and_circuits`: four-bar A and four-bar B: the Grashof inequality holds; the squared distance
  ‖A − O₄‖² stays in [9, 25]; over 64 crank sub-intervals the enclosure of σ excludes zero on each circuit.
- `closure_points_exact`: for four-bar B the squared distances from B = (11/3, ±√80/3) and (11/5, ±12/5) to A and
  O₄ are 16 and 9 exactly (the y-coordinates enter only through their squares, 80/9 and 144/25).
- `cross_circuit_candidate_rejected`: (crank 0, B above) with (crank π, B below) is rejected; (crank 0, B above)
  with (crank π, B above) is accepted.
- `branch_point_on_path_rejected`: ground 4, crank 2, coupler 3, rocker 3: at crank angle π the distance is 6,
  the sum of coupler and rocker, so a path through π has no sign certificate and is rejected.
- `caps_refusal`: a system declared with 9 variables, with 65 equations or with degree 257 is refused.
- `tolerance_box_keeps_certificate`: with every length of four-bar A in a box of ±0.01 the distance range
  [2.98, 5.02] still excludes [0.48, 0.52] and [6.48, 6.52].

### End-to-end script and logging

`scripts/e2e/spec_c34_exact.sh`: `frob --json doctor`; `cargo test -p frob-synth --test laws_c34_exact`;
`cargo test -p frob-conformance --test g1 -- KA-164 KA-165`; file and registry checks. Log to
`target/e2e-logs/spec_c34_exact/<utc>.ndjson` with a transcript; each line has `ts`, `script`, `bead`, `step`,
`event`, command line, exit code, wall time, the evidence block (claim Known answer, class Tested, tier G1 host,
determinism D0, seed), pivots and distances as rationals against expected, the sign of σ per sub-interval, and on
failure the first sub-interval whose enclosure contains zero. The `summary` line counts assertions and names
GATE-AUGUR-17.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-164 | P = (1, 0), (0, 1), (−1, 0) give O = (0, 0), radius 1; collinear positions are degenerate | exact (rationals) | G1 host | Mutant `collinear_returns_pivot` (new): solves the singular system by least squares |
| KA-165 | Links 4, 1, 3.5, 3: 1 + 4 ≤ 3.5 + 3; the crank turns fully and the two assembly circuits are disjoint | exact | G1 host | A candidate whose target poses lie on different circuits must be rejected; mutant `closure_only` accepts it |

Other mutants: `sign_checked_at_poses_only` (skips step 4 and accepts the path through the branch point);
`truncates_over_cap`; `no_saturation`. Counterexample carried: CX-082 (a linkage that closes at every target pose
does not thereby reach them all). Determinism: exact arithmetic, twice on one machine.

## Considerations

- The construction's pilot (24 templates) and the owning chapter's (240 instances) differ in size and falsifier;
  the gate uses 240. This bead records both and does not reconcile them; the Augur lead owns the pilot.
- The derivation subprocess: the symbolic sibling is used at build time and through a subprocess, so the caps are
  enforced before submission and a timeout is a refusal, not a partial answer.
- If KILL-SYNTH-LINKAGE fires, the circuit and clearance checks of L3 stay in force for the numerical fallback;
  only the exact solve is replaced.
- Self-collision and reach use the same sub-interval enclosures; their geometry is the clearance machinery of the
  Skeleton and Habitat subsystems and is out of this contract.

## References

Plan §10 C-34 (lane 1), C-11; §25.21.3; proof kinds `interval-box`, `partition`; Appendix D.6 "Three-pose dyad";
KA-164, KA-165; CX-082; GATE-AUGUR-17; KILL-SYNTH-LINKAGE; BD-361.
