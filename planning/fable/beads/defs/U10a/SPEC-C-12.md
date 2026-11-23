---
key: "SPEC-C-12"
title: "frob-design: C-12 specified — typed ports, power-conserving wiring, five functors; contract and law tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "skeleton", "frob-design", "frob-model", "frontier", "spine", "spec", "test"]
parent: "WP-07"
deps: ["BD-025", "N10a-01", "STD-TEST", "STD-E2E"]
needs: ["§10 C-38: port-Hamiltonian actuator components, the target of the dynamic functor (SPEC-C-38, another unit)", "§29.9: the energy ledger, which reads the port structure at run time (BD-092)"]
related: ["BD-056", "BD-060", "BD-061", "BD-062", "BD-063", "BD-076", "BD-077", "N10a-11", "N10a-19", "SPEC-C-02", "SPEC-C-09", "CARD-T-15", "CARD-T-29", "N10-KA1", "BD-054"]
gates: ["GATE-SKEL-02", "GATE-SKEL-06", "GATE-SINEW-02", "GATE-CHRON-03"]
plan_refs: ["§10 C-12", "§9 D-11", "§14.6", "§14.14", "§12.3 T-15 T-29", "§38.7", "Appendix C.1", "Appendix D.4", "Appendix M KA-075 KA-076 KA-078 KA-079", "Appendix N CX-037"]
---
## Background

Construction C-12 says what a robot design is: a composition of components over typed ports. It replaces a
monolithic description file plus a sizing spreadsheet. The construction is deliberately thin: one data
structure (a wiring diagram) and five functors that give it meaning, not a framework. A category-theoretic
framework beyond five functors was considered and rejected under the no-ornamental-mathematics rule.

Payoff:

- Editing a subterm recomputes only that subterm's meanings, by content-addressed memoization.
- "Does the design close?" is answered before simulation with either the minimal mass and power or a proof of
  infeasibility (the budget functor; specified in N10a-11).
- The energy ledger is the port structure read at run time.
- Gears are transformers and motors are gyrators, which is standard bond-graph algebra.

Tier: Frontier, kept thin. Pilot: N10a-19 (a 64-edit sequence with four invalidation controls). Fallback named
in advance: whole-model recompilation on every edit.

Foundations (FACT, FrankenSim `483d09c682f2`, 2026-11-21; existence is not admission):
`fs_ports::component::Component` (contract v0, not frozen) with `ports()`, `class()`, `stored_energy_j()`;
`fs-system::SystemBuilder`; `fs-mbd` already wraps a rigid body as a Component; `fs-phs::interconnect_mapped`
exists, constrained Dirac structures are deferred there and no passivity proof generator exists; FrankenSim's
operating rule since 2026-11-09 is "Compose, do not enumerate".

This bead specifies ports, power-conserving interconnection, the wiring operad, the five functors and their
composition laws, the energy balance and the gear law. N10a-11 specifies the budget fixed point, the
time-correlation rule for port contracts and the invalidation contract.

## Scope

In: `docs/math/C-12.md` (sections on ports, wiring, functors, energy balance); the law suite
`crates/frob-design/tests/laws_c12.rs`; functor-law fixtures on a small modular design; port-sign and
algebraic-loop refusals; the register row; tolerances and controls for KA-075, KA-076 and KA-078; the
statements of T-15 and T-29.

Out: the term model, constructors and module format (BD-054, BD-056, BD-060); the functors' implementations
(BD-061, BD-062, BD-063); the port energy-balance check on the referee (BD-076); the budget functor, shared
resources and invalidation (N10a-11); the pilot (N10a-19).

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-054 (the design term model
of `frob-design`): that bead is listed in `related` and is not a prerequisite. The law suite is generic over its
subject trait and closes on the exact reference model and the mutants in its own test support. If `frob-design`
does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

**Ports.** A component exposes typed ports. Each port carries a pair of conjugate variables, an effort e and a
flow f, whose product is power in watts, or a typed signal:

| Port | Carries | Power |
|---|---|---|
| Mechanical | a twist and a wrench at a frame | ⟨wrench, twist⟩ = τ·ω + f·v |
| Shaft | (ω, τ): angular rate and torque about one axis | τ·ω |
| Electrical | (V, I) | V·I |
| Thermal | (T, Q̇): temperature and heat flow | Q̇ (a thermal port carries heat flow as power; temperature is its potential) |
| Signal | a typed message with a rate and a latency bound | none |

Sign convention fixed by the contract: power is positive into the component at every port.

**Interconnection.** Interconnection is power-conserving (a Dirac structure): the port powers of a composition
sum to zero identically, Σ_ports eₖ·fₖ = 0 over the internal ports that a wiring joins. An algebraic loop (a
cycle of instantaneous dependencies with no state or delay on it) is refused.

**Elementary two-ports** (bond-graph algebra):

- *Transformer* (a gear of ratio r): f₂ = r·f₁ and e₁ = −r·e₂. Then e₁·f₁ + e₂·f₂ = −r·e₂·f₁ + e₂·r·f₁ = 0.
  This is the rational gear law (theorem card T-15): a polynomial identity after substitution; and a
  composition of power-preserving port maps preserves power.
- *Gyrator* (an ideal motor of constant k): e₂ = k·f₁ and e₁ = −k·f₂, so e₁·f₁ + e₂·f₂ = −k·f₂·f₁ + k·f₁·f₂ = 0.

**Designs as terms.** Designs are terms of an operad of wiring diagrams: a term is a component, or a wiring of
sub-terms that joins compatible ports and exposes the rest. Composition is substitution of a term into a slot
of a wiring. A reusable subassembly is a module file (`.fmodule`) referenced by content hash, carrying its port signature,
its local inertia and passivity results, and a boundary descriptor.

**Five functors** (each maps terms to a meaning and must respect composition):

| Functor | Target | What composition becomes |
|---|---|---|
| Kinematic | the frame groupoid: a tree of frames and joint maps | composition of transforms along the attach path |
| Dynamic | port-Hamiltonian components | power-conserving interconnection of the components' ports |
| Geometric | collision Regions | union of placed Regions |
| Budget | monotone maps between resource and functionality posets | composition and feedback of monotone maps |
| Symmetry | groups | the group of the composite computed from the parts' groups and the wiring's automorphisms |

Functor law: F(compose(w; t₁, …, tₙ)) = F(w)(F(t₁), …, F(tₙ)) for every wiring w and sub-terms tᵢ.

**Energy balance.** At every step and for every mechanism:
supplied = change in stored + dissipated + exported, with the numerical residual on its own line. Heat held in
a thermal node is stored energy, not loss.

**Honest limits** (those that concern this half; the rest are in N10a-11).

1. Reuse across an edit is licensed by an implication, not by a matching hash.
2. The budget functor applies in the monotone fragment only.
3. The construction is a data structure and five functors; it claims no new theorem.

**Statements.**

| Id | Statement | Class | Route |
|---|---|---|---|
| C-12.S1 | Each of the five functors commutes with composition | Tested | Property test on random terms and substitutions |
| C-12.S2 | Wiring conserves power: the port powers of a composed design sum to zero identically (T-29, first half) | Certified | Linear identity on the wiring's incidence equations |
| C-12.S3 | The wiring graph has no algebraic loop, or the design is refused (T-29, second half) | Certified | Graph check, recomputed by the checker |
| C-12.S4 | Rational gear law: f₂ = r·f₁ and e₁ = −r·e₂ imply e₁·f₁ + e₂·f₂ = 0; compositions of power-preserving maps preserve power (T-15) | Certified; Proved in general | Polynomial identity after substitution; ring arithmetic in Lean |
| C-12.S5 | The energy balance closes every step: supplied = stored change + dissipated + exported, to tolerance | Tested | Referee and batched runs |
| C-12.S6 | A reversed port sign is refused | Tested | Planted design |
| C-12.S7 | A design built with a symmetry constructor equals its hand-expanded form under every functor | Tested | Metamorphic test |
| C-12.S8 | Editing a subterm changes only the meanings whose inputs changed; incremental compilation equals full compilation bit for bit | Tested | Random edit sequences |

**Fixtures** (DESIGN).

- *Two-stage gear train.* Ratios r₁ = 3 and r₂ = 5/2 in series, rational port values: input (e, f) = (2, 7);
  after stage 1 the output flow is 21 and effort −2/3 (from e₁ = −r·e₂); the composite ratio is 15/2 and the
  total port power is zero exactly.
- *Motor–gear–link chain.* A gyrator of constant k = 1/2 feeding a transformer of ratio 4 feeding a rotational
  inertia: the dynamic functor's composite has stored energy equal to the inertia's alone, and electrical
  power in equals mechanical power out plus the rate of stored energy, exactly in rationals.
- *Modular quadruped.* Four leg modules attached to a torso by `mirror` twice; used for S1, S7 and S8.
- *Algebraic loop.* Two ideal transformers wired output-to-input in a ring with no storage element.

**Refusals.** FROB-REF-108: power sign mismatch, copied power port or algebraic loop (repairs: flip the port
orientation; insert the missing storage or compliance element; remove the duplicate connection).
FROB-REF-101: port type mismatch. FROB-REF-114: module digest or port signature mismatch on import or
substitution.

**What it does not claim.** Passivity of a composite beyond the stored local results of its modules; closure
of a design (the budget half); correctness of any component's internal physics.

**Implementation map.** Crates `frob-design` (terms, wiring) and `frob-model` (functors). Verbs:
`design.check`, `design.view`, `module.check`, `module.substitute`, and the energy reports
(`telemetry.energy`). Gates: GATE-SKEL-02 (TARGET, M1: functor laws and incremental-equals-full compilation
hold on 10⁴ random designs and edits); GATE-SKEL-06 (substitution controls; M1); GATE-SINEW-02 (port energy
balance closes to 10⁻⁹ relative on the referee and 10⁻⁴ relative per episode on single-precision tiers; M1
and M2); GATE-CHRON-03 (the energy ledger closes: residual below 10⁻⁹ relative on the referee and 10⁻³ on
single-precision tiers; M1 and M2). Cards: T-15, T-29 (and T-12 in N10a-11).

## Acceptance criteria

- [ ] `docs/math/C-12.md` exists with the port table and sign convention, the interconnection rule, both
      two-ports with their identities, the operad, the five functors with the functor law, the energy
      balance, the eight statements and the fixtures.
- [ ] `cargo test -p frob-design --test laws_c12` passes on the reference term model and on the production
      functors as they land.
- [ ] Each functor commutes with composition on 10⁴ random terms and substitutions.
- [ ] The gear-train and motor–gear–link fixtures balance exactly in rational arithmetic.
- [ ] The reversed-port design and the algebraic-loop design are refused with FROB-REF-108.
- [ ] A `mirror`-built quadruped equals its hand-expanded form under all five functors (identical model
      identity).
- [ ] KA-075, KA-076 and KA-078 registered with tolerances and controls set; the register row of C-12 filled.

## Test plan

### Unit tests

- `s1_functor_laws_{kinematic,dynamic,geometric,budget,symmetry}`: random terms of depth up to four over a
  catalogue of six leaf components; F(compose) = F(w)(F(tᵢ)) compared by canonical bytes.
- `s2_wiring_power_zero`: random wirings; Σ eₖ·fₖ over joined ports is the zero polynomial.
- `s3_algebraic_loop_refused`: the ring of transformers; and the same ring with one inertia inserted is
  accepted.
- `s4_gear_law`: the identity for symbolic r; the two-stage fixture numerically in rationals.
- `s4_gyrator_law`: the identity for symbolic k.
- `s5_energy_balance_exact`: motor–gear–link chain stepped by an exact rational integrator for ten steps;
  residual exactly zero; then on the referee to 10⁻⁹ relative.
- `s6_reversed_port_refused`: one port orientation flipped.
- `s7_mirror_equals_expanded`: the modular quadruped.
- `s8_incremental_equals_full`: 10³ random edit sequences of length eight; byte-equal products.

### End-to-end script and logging

`scripts/e2e/spec_c12.sh`. Sources the STD-E2E helper. Steps: law suite; law suite per control; G0 and G1 rows
KA-075, KA-076, KA-078; `frob --json design check fixtures/ports/reversed_sign.fbot` (expects FROB-REF-108
with ranked repairs); `frob --json design check fixtures/ports/algebraic_loop.fbot` (same code); `frob --json
sim run --tier referee` on the motor–gear–link fixture followed by `frob --json telemetry energy`, asserting
the four ledger terms and the residual against 10⁻⁹ relative. Log `target/e2e-logs/spec_c12/<utc>.ndjson`
with transcript: commands, exit codes, refusal codes and repairs, ledger terms with units, the evidence block,
and on failure the first functor and term whose images differed. Final `summary` line.

### Known answers and negative controls

- KA-075 functor laws: exact; G0 · host. Control (set here): the kinematic functor applied to an attachment
  without composing the attached part's mount transform.
- KA-076 energy balance of a composed design: tolerance set to 10⁻⁹ relative on the referee and 10⁻⁴ relative
  per episode on single-precision tiers (the port-balance gate's figures); G1 · all tiers. Control: a reversed
  port sign, which must be refused.
- KA-078 rational gear law: exact; G0 · host. Control (set here): the law written with e₁ = +r·e₂, which gives
  port power 2·r·e₂·f₁ instead of zero.
- CX-037 (reuse because the sub-term's hash still matches) is carried with the invalidation half in N10a-11.
- Further controls: heat entering a thermal node booked as both stored and lost (the ledger shows a deficit
  equal to that heat); a duplicated power connection.
- Metamorphic relations: a design built with `mirror` equals its hand-expanded form; an edit followed by its
  inverse returns the same design identity; incremental recomputation equals full recomputation on random
  edit sequences (mutation: an input missing from a key).
- Determinism: all functor outputs are D0; identical bytes on Linux and macOS.

## Considerations

- Kill gate for substitution, named in advance (KILL-SKEL-SUBST): if the substitution contract cannot be made
  sound, the fallback is whole-model recompilation on every edit; the functor laws still hold.
- Risk: the wiring formalism grows into a framework. The scope is one data structure and five functors; a
  sixth functor needs an architecture decision record and its own failing test.
- The upstream component contract is v0 and not frozen; this contract's port table is defined independently of
  it and mapped at the airlock.
- Ceiling stated before measurement: time from an edit to its derived meanings, a small share of a design loop
  once a replay takes seconds.

## References

Plan §10 C-12; §9 D-11; §14.6, §14.14 (INV-SKEL-02); §12.3 (T-15, T-29); §38.7; Appendix C.1 (kept thin; a
larger framework rejected); Appendix D.4 (rational gear law, energy balance); Appendix M KA-075, KA-076,
KA-078, KA-079; Appendix N CX-037. Pilot: N10a-19.
