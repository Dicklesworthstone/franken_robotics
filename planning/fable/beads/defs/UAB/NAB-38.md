---
key: "NAB-38"
title: "killed-idea register KI-47 to KI-56: algebra, synthesis, proofs, certificates (ornamental labels)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "lemma", "marrow", "augur", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-32", "NAB-02"]
needs: ["Appendix O: refusal code FROB-REF-214 (a quaternionic constituent is refused, because meeting one means the group computation is wrong) in the refusal registry", "§13.5: the equivariant blocks with the Reynolds operator kept as their test oracle (BD-325)", "§12.1, §12.5: the stated limits of the proof layer and the admission of the proof checker (BD-380)", "§0.8: hypothesis rows H-17, H-19 and H-20 (N00-12)"]
related: ["NAB-37", "NAB-39", "NAB-20", "NAB-26", "NAB-28", "NAB-31", "NAB-15", "N12-30", "N25a-18", "N25e-10", "BD-048", "BD-049", "BD-131", "BD-325", "BD-339", "BD-360", "BD-361", "BD-362", "BD-382", "BD-437"]
gates: []
plan_refs: ["Appendix C.2.6", "§0.8 (H-17, H-19, H-20)", "§12.5", "§13.5", "§25.21", "§25.22"]
---
## Background

FrankenRobotics keeps every idea that was proposed and then refuted or deflated in a register
(`registries/killed_ideas.toml`, rendered as `docs/decisions/killed-ideas.md`; schema and revive lint in
NAB-32), each with the strong form as proposed, the refutation with its numbers, what survives, and the
condition under which it would be revived. A proposal that matches an entry must cite it.

This bead carries the sixth group, ten entries about **algebra, synthesis, proofs and certificates**. The
project uses real mathematics (Lie groups, finite group representations, exact polynomial solving, interval
enclosures, a proof kernel) and is correspondingly exposed to mathematics used as decoration, and to proof
claims that outrun what the tools can decide. The entries here are of four kinds: capability claims that the
tools cannot support (root certificates checked inside a proof kernel in half a second; soundness theorems
"proved once at M1"; whole-robot synthesis by elimination); statements that are simply false (posynomial
feasibility by integer-linear arithmetic; "Reynolds averaging mishandles complex irreducibles"); structure
that does not occur (quaternionic irreducibles among the groups in scope); and vocabulary that names a richer
theory without using its theorems.

Reading conventions: a number inside a *strong form* is the proposal's own; FACT figures (from sibling code,
the literature, or executed pilots) have source-register records and are re-read or re-run before a gate
cites them; "DESIGN arithmetic" can be redone by hand; ranges in *survives* lines are HYPOTHESES until the
named gate records a value; thresholds are TARGETs. Tier of the bead: Solid (a decision record).

## Scope

In: records `KI-47` to `KI-56` with calculations, keywords, carriers and revival conditions; the eight
ornamental labels of KI-50 as `[[ornamental]]` records consumed by the glossary lint. Out: the schema, harness
and lint (NAB-32); the glossary lint itself (NAB-02); the surviving mechanisms (their carriers).

## Technical approach

**Transcription rule.** As in NAB-33.

**Machine-checked arithmetic of this group.**

| Record | Expression | Value |
|---|---|---|
| KI-47 | solutions of the nine-point four-bar problem: 3 cognates × 1,442 triples; paths of the best multihomogeneous homotopy | 4,326; 286,720 |
| KI-47 | the α-test threshold (13 − 3√17)/4 | 0.157671 |
| KI-48 | Frobenius–Schur indicator ι(χ) = (1/\|Γ\|)·Σ_g χ(g²) of the two-dimensional character of the quaternion group of order 8: (2 + 2 + 6·(−2))/8 | −1 (quaternionic type) |
| KI-48 | the indicator of a non-trivial character of C₃: (1 + ω² + ω)/3 with ω = e^(2πi/3) | 0 (complex type) |
| KI-54 | strata of k contacts with three states: 3ᵏ at k = 4 | 81 |
| KI-54 | strata of four feet up to the symmetry group C₂ × C₂, by orbit counting: (3⁴ + 3·3²)/4 | 27 |
| KI-56 | ceiling of a zero-cost solver in the solver pilot: 842.8 µs ÷ 575.7 µs | 1.46 |
| KI-56 | the residual the solver does not touch: 610.1 µs − 34.4 µs | 575.7 µs |

Notes on these numbers. The indicator's meaning: ι = +1 real type, 0 complex type, −1 quaternionic type. Every
irreducible representation of C₂, C₂ × C₂, the dihedral groups and S₄ is of real type, and Cₙ with n ≥ 3 adds
complex type only; the smallest group with a quaternionic irreducible is the quaternion group of order 8,
which is not a point group. The orbit count of KI-54 uses the fact that the identity fixes all 3⁴ labelings
and each of the three non-identity elements of C₂ × C₂ (acting on four feet as products of two transpositions)
fixes 3² of them. The measured ratios of KI-56 (1.345 warm, 1.088 cold) are medians of per-fixture ratios and
are not the ratio of the medians.

FACT figures and their sources: sibling-code facts (the exact real solver is capped at 8 variables, 64
equations, degree 256 and 4,096-bit coefficients, on zero-dimensional ideals only; no sibling has a homotopy
solver; the proof tool implements a decision-tree tactic and linear integer arithmetic, its other tactics are
type stubs, and the library modules on Lie groups, matrix exponentials and representation theory are blocked;
the semidefinite solver costs O(N⁴) for an N × N Gram block) are local sources re-read at the pinned commits;
the nine-point counts are standard references to confirm; the executed pilots of KI-56 (standard exact
methods returned the same complete answers on every fixture) are plan-time checks to re-run.

**KI-50, ornamental labels, as data.** Each label becomes an `[[ornamental]]` record (`label`, `plain`,
`allowed_if`) read by the glossary lint, so that the rule "a term from a richer theory is used only where a
theorem of that theory is used" is enforced on contracts and verb descriptions:

| Label | Plain name to use | Where the plain mechanism lives |
|---|---|---|
| decorated cospan; sub-PROP | composition over typed ports | the module port contract (construction C-12) |
| Futamura projection | build-time partial evaluation | the kernel derivation toolchain (C-14) |
| Pearl-causal model | interventions on a forkable simulator | counterfactual debugging (C-29) |
| Wasserstein robustness | a worst case over a bounded set | mission queries (C-17) |
| cellular sheaf | a least-squares pose graph | sensor calibration |
| conformal geometric algebra | primitive intersection tests | collision primitives |
| max-plus cycle mean, for balancing two machines | a role table for two machines | the fleet scheduler |

(Max-plus algebra *is* used, with its eigenvalue theorem, for gait and task timing; that use is not ornamental
and its label is allowed there.)

**Keywords for the revive lint.** KI-47: GPU homotopy continuation, α-theory inside the proof kernel. KI-48:
quaternionic block, quaternionic irreducible. KI-49: Reynolds operator is wrong, typed blocks fix Reynolds.
KI-50: the labels above. KI-51: proved once, verified at M1, general soundness theorem. KI-52: all real
branches of the whole robot, synthesis by elimination at robot scale. KI-53: posynomial feasibility by linear
arithmetic, sample inside inner ellipsoid. KI-54: Conley index for contact modes, cylindrical decomposition of
reachability. KI-55: self-validating monitor bound, bound valid under shift. KI-56: family query no existing
tool has, specialized solver ten-fold.

**The entries.**

**KI-47. Continuation on the GPU, with root certificates checked inside a proof kernel**
- *Strong form.* Track homotopy paths on the GPU and certify thousands of roots by α-theory inside a proof kernel
  in half a second.
- *Refutation.* Path tracking needs double precision and Metal has none. FACT (literature): the nine-point
  four-bar problem has 1,442 cognate triples, 3 × 1,442 = 4,326 solutions, found among 286,720 paths of the best
  multihomogeneous homotopy, and that one-time solve was priced at zero. Exact arithmetic on thousands of roots
  inside a proof kernel in half a second has no basis, and an α-test proves a nearby nonsingular root and nothing
  about completeness.
- *Survives.* The CPU lane [M]: double precision, a monodromy start with a trace test, and the α-test
  α(f, z) < (13 − 3√17)/4 ≈ 0.157671 evaluated by the exact rational checker outside any proof kernel, with the
  labels `Complete` and `Partial` (C-34; §25.21.4; GATE-AUGUR-22; KILL-SYNTH-CONT).
- *Revive only if.* Double precision on every backend in the run, and a measured need the CPU lane does not meet. The CPU lane is decided at GATE-AUGUR-22.
- *Carried by.* BD-437, N25d-23 to N25d-25.

**KI-48. Quaternionic irreducibles**
- *Strong form.* Equivariant layers must implement the real, complex and quaternionic types of irreducible
  representation.
- *Refutation.* The type is read from the Frobenius–Schur indicator ι(χ) = |Γ|⁻¹·Σ_g χ(g²), and quaternionic type
  means ι = −1. The smallest group with such a representation is the quaternion group of order 8, whose
  two-dimensional character gives (2 + 2 + 6·(−2))/8 = −1; it is not a point group. Every irreducible of C₂,
  C₂ × C₂, the dihedral groups and S₄ is of real type, and Cₙ with n ≥ 3 adds complex type only: for C₃,
  (1 + ω² + ω)/3 = 0.
- *Survives.* Real and complex blocks only (§13.5; ADR-MARROW-07). A quaternionic constituent is refused
  (`FROB-REF-214`), because meeting one means the group computation is wrong.
- *Revive only if.* A group in scope with a Frobenius–Schur indicator of −1. None of the point groups in scope has one, and meeting one is treated as a defect of the group computation.
- *Carried by.* BD-048, BD-325, N13-24.

**KI-49. "Reynolds averaging mishandles complex irreducibles"**
- *Strong form.* Typed irreducible blocks correct an error that Reynolds averaging makes on complex-type
  representations.
- *Refutation.* The Reynolds operator W ↦ |Γ|⁻¹·Σ_g ρ_out(g)·W·ρ_in(g)⁻¹ is the exact projector onto the
  equivariant maps for any real representation. It is more expensive, not wrong.
- *Survives.* The blocks as a parameterization, with the Reynolds operator kept as their test oracle (§13.5;
  GATE-MARROW-04; KILL-MARROW-BLOCKS).
- *Revive only if.* None. The statement was false: the Reynolds operator is the exact projector.
- *Carried by.* BD-325, N10a-10.

**KI-50. Ornamental labels**
- *Strong form.* Mechanisms named as decorated cospans, a sub-PROP, Futamura projections, Pearl-causal models,
  Wasserstein robustness, cellular sheaves or conformal geometric algebra; max-plus cycle means to balance two
  machines.
- *Refutation.* D-2 binds vocabulary: a term from a richer theory is used only where a theorem of that theory is
  used. In each case no such theorem was invoked, and the mechanism underneath is an ordinary one.
- *Survives.* Each mechanism under its plain name: composition over typed ports (C-12), build-time partial
  evaluation (C-14), interventions on a forkable simulator (C-29), a worst case over a bounded set (C-17), a
  least-squares pose graph (§20.14), primitive intersection tests (§18.3), a role table for two machines
  (§21.17).
- *Revive only if.* A mechanism that invokes a theorem of the richer theory may take that theory's name. The glossary lint and the plain-names review rule keep the vocabulary bound.
- *Carried by.* NAB-02, N00-08.

**KI-51. "Proved once at M1"**
- *Strong form.* The soundness theorems of the proof layer are proved once at M1 and reused. The obligations
  include identical trajectories under a quotient, equivalence of fixed-point firmware, and a quantified barrier
  condition for high-dimensional dynamics.
- *Refutation.* FACT (sibling code): the proof tool implements a decision-tree tactic and linear integer
  arithmetic; its other tactics are type stubs, and the library modules on Lie groups, matrix exponentials and
  representation theory are blocked. The listed obligations are not statements those checkers can decide, and no
  route to the theorems without tactics was given.
- *Survives.* Four evidence classes, with instance certificates checked in exact arithmetic (§12.2, §12.4).
  Reflection as an [F] pilot at M5 (GATE-LEMMA-05; KILL-REFLECT). The limits stated in §12.5. The word
  "verified" is not used for results.
- *Revive only if.* The proof tool gains the tactics, and the blocked library modules are unblocked, in its own repository; then the reflection pilot (GATE-LEMMA-05).
- *Carried by.* BD-380, BD-381, BD-382.

**KI-52. From mission to design by solving, at whole-robot scale** (H-17, NARROWED)
- *Strong form.* Polynomial synthesis returns all real branches of a whole-robot design problem, and three
  solvers produce candidate designs for a mission in minutes.
- *Refutation.* FACT (sibling code): the exact real solver is capped at 8 variables, 64 equations, degree 256 and
  4,096-bit coefficients, on zero-dimensional ideals only, and no sibling has a homotopy solver. The nine-point
  problem of KI-47 alone has 4,326 solutions.
- *Survives.* A bounded exact lane [F] (KILL-SYNTH-LINKAGE). Trajectory optimization with design variables as
  local refinement: about 12 degrees of freedom, up to 17 parameters, 4 to 10 minutes (KILL-SYNTH-TRAJOPT).
  Monotone co-design for actuator, battery and compute selection, with no measured timing. Continuation [M] on
  the CPU (§25.21). The pass rate of synthesized designs under independent closed-loop evaluation is unknown, so
  every synthesized candidate passes the ordinary ladder.
- *Revive only if.* Raised caps in the exact real solver (8 variables, 64 equations, degree 256, 4,096-bit coefficients) or an admitted homotopy solver; and a measured pass rate of synthesized designs under independent closed-loop evaluation.
- *Carried by.* BD-360, BD-361, BD-362, BD-437.

**KI-53. Posynomial feasibility by integer-linear arithmetic, and sampling only inside a certified inner ellipsoid**
- *Strong form.* The substitution y = ln θ makes posynomial constraints linear, so an integer-linear decision
  procedure settles feasibility; and candidates are sampled only inside a certified inner ellipsoid, for a
  seventy-fold yield.
- *Refutation.* The logarithm of a posynomial is a log-sum-exp, which is convex and not linear, and the procedure
  is integer-linear. An inner ellipsoid cuts off the feasibility boundary, where optima sit, and misses
  disconnected components.
- *Survives.* Monomial screens and proved interval exclusion boxes; certificate-backed local boxes, with the
  unresolved remainder reported (§25.6, §25.7).
- *Revive only if.* None. The logarithm of a posynomial is convex and not linear. Monomial screens in exponent coordinates are the admissible form (GATE-AUGUR-27).
- *Carried by.* BD-358, N25e-10 to N25e-12.

**KI-54. Topological and exact reachability proofs over contact modes**
- *Strong form.* A Conley-index or cylindrical-decomposition argument proves reachability across all contact
  modes.
- *Refutation.* DESIGN arithmetic: k contacts with three states have 3ᵏ strata, 81 for four feet, and
  cylindrical decomposition is doubly exponential beyond four or five variables.
- *Survives.* Mode strata counted up to symmetry: (3⁴ + 3·3²)/4 = 27 for four feet under C₂ × C₂ (§13.6).
  Max-plus timing for schedules (C-10).
- *Revive only if.* None at four contacts or more. Strata are counted up to symmetry.
- *Carried by.* BD-049.

**KI-55. A probability bound for runtime monitors, "valid under shift"**
- *Strong form.* Breach probability is bounded by r_breach·δ₁ + r_alarm·δ₂ + δ₃ over 14 strata, and the bound
  survives distribution shift.
- *Refutation.* The first term counts breaches twice. A monitor cannot label its own silent breaches. The strata
  number 27 (KI-54), not 14. A shift moves δ₁.
- *Survives.* A conditional bound with breach labels that are independent of the monitor, and shifted worlds
  reported separately (§22.12; GATE-SHIELD-03).
- *Revive only if.* Breach labels drawn from hidden state independently of the monitor (GATE-SHIELD-03). A bound that validates itself is not revived.
- *Carried by.* BD-339, BD-340.

**KI-56. A new family query as a capability, and a faster solver as the lever**
- *Strong form.* Family certificates and minimum-tag queries are a capability no existing tool has, and a
  specialized solver for them gives a ten-fold decision.
- *Refutation.* FACT (executed pilots): standard exact methods (projection, Fourier–Motzkin elimination,
  satisfiability modulo theories, belief-game elimination) returned the same complete answers on every fixture.
  A specialized solver measured 1.345 warm and 1.088 cold against a composed one, and its batch held a residual of
  575.7 µs that the solver does not touch (parsing, assembly, checking, serialization and harness, not separately
  timed), so no solver could have shown more than about 1.46 (842.8 ÷ 575.7).
- *Survives.* The query is kept where deriving the abstraction from a rich plant is cheaper than an expert doing
  it by hand. A tie with the generic comparator ships the generic method behind the same verbs (§25.22;
  KILL-AUGUR-TIE, KILL-FAMILYCERT; BET-14, BET-19).
- *Revive only if.* At the M3 partial gate, a family certificate decides a cell that the generic comparator leaves undecided at equal cost (KILL-FAMILYCERT, KILL-AUGUR-TIE).
- *Carried by.* BD-131, BD-140, N12-30, N25a-18.

## Acceptance criteria

- [ ] Records `KI-47` … `KI-56` exist with the three paragraphs verbatim, revival conditions, carriers and
      keywords; eight `[[ornamental]]` records exist (one per label; the first row of the table holds two) and
      the glossary lint reads them.
- [ ] The harness recomputes the eight calculation rows; the Frobenius–Schur indicators are computed by a test
      from the groups' character tables, and the orbit count by brute-force enumeration of 81 labelings.
- [ ] `FROB-REF-214` exists in the refusal registry with the stated meaning.
- [ ] The wording lint finds no use of "verified" for a result anywhere, and KI-51's record names that rule.
- [ ] The revive lint fails a planted contract sentence "the fusion is a cellular sheaf over the sensor graph"
      (through the glossary lint, `gloss.ornamental-label`) and a planted bead "Certify 4,326 roots by α-theory
      inside the Lean kernel on the GPU" (`ki.match-without-citation`, KI-47).
- [ ] KI-56's record states the tie rule: a tie with the generic comparator ships the generic method behind
      the same verbs.

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`, module `g6_algebra`:

- `ki47_counts_and_threshold`: 3 × 1,442 = 4,326; (13 − 3·√17)/4 = 0.1576708.
- `ki48_indicators`: for the quaternion group Q₈ (elements ±1, ±i, ±j, ±k) with the two-dimensional character
  χ(±1) = ±2 and 0 elsewhere, Σ χ(g²) = 2 + 2 + 6·(−2) = −8, ι = −1; for C₂, C₂ × C₂, D₄ and S₄ every irreducible
  has ι = +1; for C₃ the two non-trivial characters have ι = 0.
- `ki54_orbit_count`: enumerate the 3⁴ labelings of four feet with three states each (labelled 0, 1, 2); the
  number of orbits under C₂ × C₂ acting by the three
  double transpositions is 27; without symmetry 81; at twelve contacts 3¹² = 531,441.
- `ki56_ceiling`: 610.1 − 34.4 = 575.7; 842.8/575.7 = 1.4640; and 842.8/610.1 = 1.381 ≠ 1.345.
- `ornamental_records_load_into_glossary_lint`.
- `records_complete_and_verbatim` against `tests/fixtures/killed/g6.md`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) recomputes the calculations, runs the group-theory tests and the
planted cases. Logged per STD-E2E.

### Known answers and negative controls

Known answers: the table; 27 orbits; indicators −1, 0, +1 as stated. Negative controls: an "equivariant layer"
fixture whose group computation returns a constituent with ι = −1 for a dihedral group (the carrier must
refuse with `FROB-REF-214`; this bead supplies the indicator test); a monitor bound over "14 strata" (the
count is 27); a ceiling of 1.43 obtained as 1.345 ÷ 0.94 (a withdrawn derivation); the word "verified" in a
report template. Metamorphic check for `ki54_orbit_count`: the Burnside average of fixed points equals the
brute-force orbit count.

## Considerations

- KI-51 is a statement about tools at pinned commits. When the proof tool gains tactics and the blocked
  modules are unblocked upstream, the evidence has changed and the reflection pilot is the first step; the
  word "verified" stays out of results regardless.
- KI-52 leaves three working lanes (bounded exact linkages, trajectory optimization with design variables,
  monotone catalog selection) and one Moonshot (continuation on the CPU); the pass rate of synthesized designs
  under independent closed-loop evaluation is unknown, so every synthesized candidate passes the ordinary
  ladder of evaluation.
- KI-49 and KI-53 cannot be revived by measurement: they are false statements. Their records exist so the
  statements are not made again.

## References

Appendix C.2.6; §0.8 rows H-17, H-19, H-20; §12.5; §13.5; §25.21; §25.22; doctrine rule D-2; ADR-MARROW-07.
