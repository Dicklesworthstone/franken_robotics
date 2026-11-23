---
key: "SPEC-C-31"
title: "spec C-31 core: safety in three separated tiers, and tier 1, the energy budget at a guided boundary"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "spine", "spec", "frontier", "reflex", "safety", "frob-shield", "test"]
parent: "WP-65"
deps: ["N10a-01", "STD-TEST", "STD-E2E"]
needs: []
related: ["BD-132", "BD-092", "BD-208", "BD-338", "BD-339", "BD-340", "N10c-02", "N10c-03", "N10-CX", "SPEC-C-38", "N10c-10", "CARD-T-28", "BD-337"]
gates: ["GATE-SHIELD-01"]
plan_refs: ["§10 C-31", "§22.11", "§12.3 T-28", "§12.4", "§12.7", "§38.7 MR-22", "Appendix D.7", "Appendix E GB-26", "Appendix I.15", "Appendix M.9", "Appendix N.7"]
---
## Background

Construction C-31 of the FrankenRobotics spine is titled "safety is an energy budget first, a filter second". It
defines three safety tiers with strict domain separation: (1) an energy budget at a guided boundary; (2) tube
inclusion for small subsystems of at most four reduced states; (3) runtime filters for the whole body. Tier [F] as
a whole; the inequality of tier 1 is Solid mathematics; the tube tables of tier 2 are Moonshot, at M8. Owner: the
Reflex subsystem, crate `frob-shield`. It replaces online verification of every contact and an unfiltered policy
during exploration.

This bead is the core specification: the separation of the three tiers and the whole of tier 1. Tier 3 is bead
N10c-02 and tier 2 is bead N10c-03; both extend the contract file this bead creates.

Why tier 1 comes first. Certifying a contact sequence is hard; bounding the energy that can reach a boundary is
not. Passive contact, with any friction coefficient μ ≥ 0, any restitution between 0 and 1 and any sequence of
impacts, can dissipate or redistribute mechanical energy and cannot create it. So if everything that could deliver
energy to a boundary is bounded, and the absorber there can take that much, the stop holds for every contact
sequence, and no contact model appears in the argument. The budget contains no term that depends on the policy,
so its evidence survives every retraining, and its monitor is one integrator and one comparison per tick. Rejected
alternatives: a monitor that only observes (two friction regimes can have identical histories before first slip,
so the breach is seen too late: the claim must be enforced by cutting drive power, not only watched); budgeting
payload kinetic energy alone (the rotor counterexample below); treating a simulated budget as a statement about a
physical robot.

Terms. *Guide*: a physical arrangement that puts an absorber on every forward path to the boundary (a stop, a
bumper, a door or drawer at the end of its travel, a lift platform, a compliant cover that may touch a person).
*Absorber*: the element that takes the delivered energy; W_abs,min is the lower envelope of the work it takes
before its limit. *Load path*: the bodies and ports that can deliver energy to the boundary. *Port graph*: the
typed-port wiring of the design (construction C-12). *Cutoff delay*: detection plus transport plus driver plus
brake delay. Proof classes: Proved, Certified (re-checked by the offline auditor `frob-verify` in exact
arithmetic), Tested, Estimated. Status today: HYPOTHESIS.

## Scope

In: `docs/math/C-31.md` part A (tier separation) and part B (tier 1); law tests in
`crates/frob-shield/tests/laws_c31_budget.rs`; registry entries for KA-153, KA-154, relation MR-22, theorem card
T-28's statement as a contract clause, and the controls seeded from CX-076.

Out: the `.fenergy` format, the exact-integer monitor kernel `K-energy-monitor`, the proof kind `energy-budget`
and the verbs `shield.budget` and `shield.check` (BD-337); the auditor's checker for that kind (BD-132, BD-337);
tier 3 (N10c-02; mechanism BD-338, BD-339, BD-340); tier 2 (N10c-03; mechanism BD-440); recording GATE-SHIELD-01
and deciding KILL-ENERGY (BD-337).

Register row (the shared rule of N10a-01). The construction register
`frob-conformance/registry/constructions.toml` is created by N10a-01 with forty typed rows whose obligation fields
are empty; this bead fills row C-31: `rust_type` (the subject trait of the law suite below, replaced by the
production type when the implementing bead binds it), `verb` (`shield.budget`; also `shield.check`, `shield.fit`,
`shield.report`), `failing_test` (a test of `frob-shield::laws_c31_budget` with its registered control),
`payoff.gate` (GATE-SHIELD-01), `statements`, `known_answers` (KA-153 to KA-156), `cards` (T-28), `gates`, `pilot`
(the record `PILOT-C-31` of the pilot register, required because the tier is Frontier) and `generic_comparator`
(the off-the-shelf method named in the pilot paragraph of this bead). Doctrine D-2 (no ornamental mathematics) is
checked on these fields: a construction with no verb or no failing test is moved to the parked register by a
decision record. The part beads N10c-02 and N10c-03 add their statements and tests to the same row. The contract
`docs/math/C-31.md` has the ten sections of the template `docs/math/TEMPLATE.md` of the same bead, in the
template's order (object and definitions; what it replaces and the payoff; conventions; statements; formulas;
fixtures and known answers; failure cases, refusals and retirement; non-claims; foundations; implementation map);
content that this bead lists under other headings goes into the template section that matches. Statement
identifiers are entered in the contract and in the register with the construction prefix (`C-31.S1` and so on, as
the template requires); the tables of this bead and of its part beads abbreviate them.

Order of work (DESIGN; the specification-first rule stated in section 6 of the construction register bead
N10a-01). This bead is written before the code it specifies and does not depend on BD-337 (`shield.budget`, which
creates `frob-shield` and implements tier 1): that bead is listed in `related` and is not a prerequisite. The law
suite is generic over its subject trait and closes on the exact reference model and the mutants in its own test
support. If `frob-shield` does not exist when this bead starts, the suite and its support files are written under
`crates/frob-conformance/tests/` with the same file names (that crate exists from BD-025) and the commands below
read `-p frob-conformance`; the bead that implements the construction moves them into the owning crate, binds its
production types to the subject trait and runs the unchanged suite as part of its own acceptance. A criterion or
script step below that names production code or a verb of an implementing bead is owed at that bead's acceptance
and is logged `pending` here until then.

## Technical approach

### A. Tier separation (contract part A)

1. *Energy budget at a guided boundary.* Where a guide puts an absorber on every forward path, the energy that can
   reach the absorber is bounded by its sources, whatever the contact sequence. Class of the claim: exact
   arithmetic, capped by its weakest input.
2. *Tube inclusion for small subsystems.* For a guard-ambiguous subsystem of at most four reduced states (a
   threshold crossing, a tray, braking on mixed tiles, a door pinch zone), a table of mode-indexed cells with one
   common safe action per cell; at run time the whole belief set is checked and the answer is covered or uncovered.
   Abstraction-based reachability; the reduction to four states is a monitored assumption, not a proof.
3. *Runtime filters for the whole body.* A policy action is projected onto barrier rows inside the actuator box,
   with quantifier order ∃u ∀(w, e). Class Tested: no full-order certificate exists.

A claim of one tier is never presented under another tier's name or class. Tier 1 covers stops and guided contacts,
not tip-over or loss of balance, where energy is conserved and redistributed. Passivity establishes neither
collision avoidance nor fall avoidance.

### B. Tier 1: the budget (contract part B)

```text
½·m_cap·v² + ∫ max(P_elec, 0) dt + E_stored + E_gravity + E_other + E_reserve  ≤  W_abs,min
```

| Symbol | Meaning |
|---|---|
| m_cap | Upper bound on the effective moving mass on the load path, including the reflected inertia of rotors, gears and wheels: N²·J_r/r² for a rotor of inertia J_r behind a ratio N at a wheel of radius r |
| v | Bound on its speed from the encoders, inflated by the sensing error |
| ∫ max(P_elec, 0) dt | The worst-case drive input until the cutoff acts; bounded by P_max·Δt_cut |
| E_stored | Energy in springs, preloads and compliance that can be released onto the boundary |
| E_gravity | Work of the weight over the absorber stroke and any drop on the path |
| E_other | Work of every other actuator or uncut source on the load path; E_gravity + E_other is the external work E_ext |
| E_reserve | A declared margin, zero unless stated; at least the energy ledger's residual bound over the window |
| W_abs,min | Lower work envelope of the absorber before bottoming or a stated force limit |

The same inequality in the owning chapter's letters: T_ub + E_drive + E_ext + E_stored + E_reserve ≤ W_abs.

| Id | Statement | Class |
|---|---|---|
| B1 | If every source on the declared load path is bounded as above and the absorber is on every forward path, the energy delivered to the boundary during a stop does not exceed W_abs,min, for every contact sequence and every friction coefficient μ ≥ 0, every restitution in [0, 1] and every contact law. | Certified for the arithmetic of an instance (theorem card T-28, proof kind `energy-budget`); the claim's class is that of its weakest input, so Estimated when any input is a card value marked `assumed`; Tested as a negative-control sweep |
| B2 | The longest admissible cutoff delay is Δt_cut,max = (W_abs − T_ub − E_ext − E_stored − E_reserve)/P_max. | Certified (exact arithmetic) |
| B3 | The budget has a term for every source on the declared load path: every port of the port graph that can inject power is in the budget with a bound, or is cut by the monitor. A budget with an unlisted source is refused. | Certified (structural check against the port graph; T-28) |
| B4 | Exclusion reading: if a mandatory crossing needs a potential rise greater than the initial energy plus all allowed external work, no controller succeeds, clairvoyant or not. Approach kinetic energy is a term of that budget, not a footnote. | Certified (exact arithmetic) |
| B5 | The budget is unchanged by retraining and by any passive contact parameters (relation MR-22). | Tested |
| B6 | In simulation the claim is exact arithmetic over Estimated inputs and therefore Estimated; a claim about a physical robot is a Reality claim with a recorded no-claim until bench data for the absorber and the drive are imported. | Rule of the claim product |

Monitor (carried so the mechanism matches the contract): one accumulator and one comparison per control tick,
evaluated in exact integers at declared unit scales (for example 10⁻⁶ J and 10⁻⁶ s); it outputs the margin
W_abs − E_deliv; the budget is admitted when E_deliv ≤ W_abs, and the monitor keeps drive power only while the
margin is positive: at zero margin it cuts drive power and hands over to the backup controller.

Premises, each checked or declared: (1) sources are complete (B3); (2) mass caps and speed errors are card values
with provenance; (3) the cutoff acts within Δt_cut, and if the packet-delay distribution has a tail beyond it the
claim becomes a probability and the artifact says so; (4) the absorber is on every forward path, from a declared
guide or from the clearance analysis; without it the claim covers only contacts made through the absorber.

Not supported: peak force, pressure or injury thresholds (except at the absorber when W_abs was defined up to a
force limit); quasi-static clamping after the stop; contacts that bypass the absorber and sources outside the port
graph (a second robot, a falling object, a person pushing); regulatory conformity or permission to operate near
people; anything about the simulator's contact law, which is used the other way round, as a negative control.

Artifact (DESIGN sketch): `EnergyBudget { boundary, load_path, unit_scale, terms { kinetic_ub, drive, external[],
stored[], reserve }, absorber { card, w_abs, provenance }, premises { sources_complete, absorber_on_path }, claim }`
with `admits(speed_reading) -> bool` and `longest_cutoff_delay(speed_reading) -> Qty`; file `.fenergy`, schema
`frob.energy/1`, quantities as decimal strings so no binary rounding enters the predicate; the three
no-authorization flags, all false.

### Law-test layout and registry

`crates/frob-shield/tests/laws_c31_budget.rs` with `tests/support/c31_ref.rs`: the budget evaluated in integer
micro-joules and microseconds; a port-graph completeness check on a small explicit graph; and a *synthetic*
one-dimensional contact model (masses on a line, impacts with restitution in [0, 1], Coulomb friction with μ ≥ 0, a
drive bounded by P_max until the cutoff, an absorber at the end) that integrates energy in exact rationals. The
suite is generic over a trait `C31BudgetSubject`; mutants in `tests/support/c31_mutants.rs`. Registry: KA-153 and
KA-154 with derivation files; MR-22; controls seeded from CX-076; the auditor row "energy budget without its
gravity term gives `reject`" (KA-213) is bound to the same mutant.

### Pilot, implementers and gates

Pilot (run by BD-337): the tray-cart stop fixture. Falsifier (TARGET): any accepted budget with an omitted source.
Ceiling: safety evidence; no speed is claimed. Fallback: tier 3 drops to monitoring only and tier 1 stands on its
own. Implementer: `frob-shield` (BD-337), with the auditor's checker in `frob-cert`. Gate: GATE-SHIELD-01 at M4
(the worked card reproduces; no exceedance over 10⁴ randomized contact sequences with μ in [0, 5], restitution in
[0, 1] and every contact law; a budget with a planted unlisted spring is refused; a payload-only budget shows an
exceedance). Kill gate KILL-ENERGY (M4): an exceedance under the premises survives one repair cycle; then the
claim is withdrawn and the monitor remains as an alarm of class Estimated.

## Acceptance criteria

- [ ] `docs/math/C-31.md` exists with part A and part B: the inequality with every symbol defined, B1 to B6, the
      monitor rule, the four premises, the unsupported uses, the artifact sketch and the pilot.
- [ ] `cargo test -p frob-shield --test laws_c31_budget` passes on the reference model, and on the production
      budget of BD-337.
- [ ] Every mutant below fails its named law.
- [ ] KA-153 and KA-154 are registered with derivation files; the derivation of KA-153 records the gravity-term
      discrepancy described under Considerations.
- [ ] MR-22 and the controls from CX-076 are registered and attached to GATE-SHIELD-01.
- [ ] `scripts/e2e/spec_c31_budget.sh` exits 0.
- [ ] Row C-31 of `frob-conformance/registry/constructions.toml` has its obligation fields filled as the
      Scope states, and `cargo test -p frob-conformance --test g0_doctrine -- d2` reports no failed
      obligation for the row (fields not yet due are reported `pending` under the grace rule of N10a-01).

## Test plan

### Unit tests

- `ka_153_budget_arithmetic`: in micro-joules, 8 kg at 0.4 m/s is 640,000; 20 W over 25,000 µs is 500,000;
  external work 250,000; stored 100,000; total 1,490,000 against 2,000,000 (50 N over 40 mm): admitted. Longest
  cutoff delay (2,000,000 − 640,000 − 250,000 − 100,000)/20 W = 50,500 µs; with reserve 100,000 it is 45,500 µs.
- `equality_is_admitted_and_cuts`: at E_deliv equal to W_abs the budget is admitted and the monitor margin is 0,
  which cuts drive power.
- `omitted_gravity_rejected`: the port graph lists gravity over the stroke and the budget omits it: rejected.
- `planted_unlisted_spring_rejected`: a spring port present in the graph and absent from the terms: rejected.
- `ka_154_exclusion`: lifting 8 kg over a 12 mm sill needs 8 × 9.81 × 0.012 = 0.94176 J; against a 0.90 J budget
  the family is excluded for wheel radii 0.02, 0.05 and 0.10 m alike; with 0.05 J of approach kinetic energy the
  budget is 0.95 J and the exclusion is withdrawn.
- `reflected_rotor_inertia`: J_r = 5 × 10⁻⁵ kg·m², N = 50, r = 0.05 m gives 50 kg of effective mass; a payload-only
  budget (8 kg, 0.64 J) is admitted and the full budget (58 kg, 4.64 J of kinetic energy) is not.
- `speed_error_inflates`: a reading of 0.40 m/s with error 0.01 m/s gives ½·8·0.41² = 0.6724 J, rounded outward.
- `mr_22_contact_independence`: 10⁴ seeded contact sequences on the synthetic model with μ in [0, 5] and
  restitution in [0, 1]: delivered energy never exceeds the budget's left side; replacing the "policy" (the drive
  schedule inside its power bound) leaves the budget unchanged.
- `claim_class_follows_weakest_input`: an absorber card marked `assumed` gives class Estimated, reality no-claim.

### End-to-end script and logging

`scripts/e2e/spec_c31_budget.sh`: `frob --json doctor`; `cargo test -p frob-shield --test laws_c31_budget`;
`cargo test -p frob-conformance --test g1 -- KA-153 KA-154 KA-213`; `cargo test -p frob-conformance --test g3 --
MR-22`; `frob --json prove list` filtered to card T-28; file and registry checks. Log to
`target/e2e-logs/spec_c31_budget/<utc>.ndjson` with a transcript; each line has `ts`, `script`, `bead`, `step`,
`event`, command line, exit code, wall time, the evidence block (claim Known answer, class, tier G1 auditor or
host, determinism D0, seed), each term in micro-joules against its expected value, and on failure the first
differing term and the fixture path. The `summary` line counts assertions and names GATE-SHIELD-01.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-153 | 0.64 + 0.50 + 0.25 + 0.10 = 1.49 J against 2.00 J; longest cutoff delay 50.5 ms, or 45.5 ms with 0.10 J of reserve | exact | G1 auditor | Mutant `omits_gravity_term`: accepted where it must be rejected |
| KA-154 | 0.94176 J needed above a 0.90 J budget for every wheel radius; 0.05 J of approach energy removes the exclusion | exact | G1 auditor | Mutant `exclusion_without_approach_energy` |

Other mutants: `payload_only_mass`; `accepts_unlisted_source`; `contact_creates_energy` (a synthetic contact with
restitution above 1, which the sweep must expose). Counterexample carried: CX-076 (an energy budget may not leave
out small sources). Related case: the pre-slip aliasing fixture (two friction regimes with identical histories
before first slip) is named by the bets and has no numbers anywhere in the plan; a proposed numeric instance is
under Considerations. Determinism: integer arithmetic, twice on one machine, debug and release.

## Considerations

- Gravity-term discrepancy. The plan prints 8 × 9.81 × 0.040 × 0.08 = 0.25 J; the product is 0.251136 J. The known
  answer and the artifact sketch carry the declared bound 0.25 J, and KA-153 keeps the printed values. A checker
  that re-derives the bound from its stated basis would find it low by 0.001136 J. The derivation file must say
  which is meant; the fail-closed default is outward rounding of a declared bound (0.26 J, total 1.50 J, longest
  delay 50.0 ms), with the printed row listed for the registry's reconciliation table.
- Label note: an earlier wording of the test called the 0.25 J term stored energy and the 0.10 J term reserve. The
  settled labels are external work (gravity over the stroke) and stored energy (a series spring), with reserve a
  further 0.10 J that shortens the delay from 50.5 ms to 45.5 ms.
- The margin is thin by design of the example: a quarter of the absorber.
- Open question OPEN-SHIELD-1 (where absorber envelopes come from before bench data exist): default `assumed`
  cards and Estimated claims; owner the Sinew lead with Ground; it blocks any energy claim above Estimated.
- Proposed numeric instance for the pre-slip aliasing control (DESIGN, for the Reflex lead to confirm): the 8 kg
  cart at 0.4 m/s brakes 0.10 m before the stop on μ = 0.5 (stops in 0.08 m at 1 m/s²) or on μ = 0.05 (wheels lock,
  deceleration 0.4905 m/s², arrival at the stop with ½·8·(0.16 − 2·0.4905·0.10) = 0.248 J). Encoder histories are
  identical until the brakes are applied; a monitor that only observes cannot prevent the contact, and the budget
  holds in both worlds because 0.248 J is inside the 0.64 J kinetic term.
- Interactions: the energy terms are those of the energy ledger (construction C-38 and the Chronicle subsystem);
  premise 4 can come from a clearance complex (C-33).

## References

Plan §10 C-31; §22.11; theorem card T-28; proof kind `energy-budget`; Appendix D.7 "Energy budget"; GB-26;
Appendix I.15; MR-22; KA-153, KA-154, KA-213; CX-076, CX-012; GATE-SHIELD-01; PO-SHIELD-01; KILL-ENERGY; BET-25;
OPEN-SHIELD-1; D-5; C-12, C-33, C-38.
