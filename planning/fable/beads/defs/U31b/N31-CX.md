---
key: "N31-CX"
title: "Counterexample registry of Foundry (§31): CX-336 to CX-344 and six late cases, with controls and gates"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "foundry", "frob-foundry", "frob-conformance", "frontier", "registry", "test"]
parent: "WP-70"
deps: ["BD-025", "STD-TEST"]
needs: ["Appendix M: the chapter's known-answer registry `N31-KA` (KA-737 to KA-754), written by the unit that owns the first subsections of the chapter; rows here cite its numbers and must agree with it value for value", "§38.11: the record format and the review rule of the negative-control registry `frob-conformance/registry/controls.toml` (conformance unit)"]
related: ["BD-400", "BD-402", "BD-404", "BD-406", "BD-408", "BD-423", "BD-438", "N31b-05", "N31b-34", "N31b-37", "N31b-43", "N31-REF"]
gates: []
plan_refs: ["Appendix N.28", "§31.6", "§31.8", "§31.9", "§31.11", "§31.12", "§31.13", "§31.14", "§31.18", "§31.21", "§31.22", "§31.23", "§38.11", "§10 C-35"]
---
## Background

A counterexample, in this program, is a hand-checkable case in which a plausible shortcut gives a wrong answer.
The catalogue of counterexamples seeds the negative-control registry: every entry becomes at least one planted
defect that a named gate must catch, on the rule that a gate nothing can fail validates nothing. A control is a
defect and not a boundary: it is the build or the input in which the protective rule was removed, and the gate
must fail on it.

Foundry is the subsystem that turns a qualified robot design into something that could be built: it extracts
load cases from simulation telemetry, sizes and optimizes links, checks stress, buckling, modes and cumulative
fatigue damage, feeds mass and inertia back into the model, checks manufacturability, allocates tolerances and
writes a build packet (`.fbuild`). Two Moonshot items sit beside it: export of fixed-point controller code and
lifecycle compression. Structural shortcuts are unusually dangerous here because several of them are standard
practice in spreadsheets (adding channel peaks, counting cycles per test run, evaluating a law at an average
condition) and give answers that look reasonable.

This bead is the registry of Foundry's counterexamples: the nine catalogue entries CX-336 to CX-344, and six
further cases stated in the chapter that have no catalogue number yet. Tier: the registry itself is plain
bookkeeping; the mechanisms it protects are Frontier [F] and, for CX-342 to CX-344, Moonshot [M].

Design reason for keeping the numbers in the registry: the cases are small on purpose, so that a reviewer can
recompute them by hand, and so that a test that passes by accident can be told from one that checks the right
thing. Two catalogue entries are printed without a numeric case (CX-337, CX-339); this bead supplies one for
each, labelled DESIGN, so that the control is executable.

## Scope

In: the registry rows; one control record per row in the negative-control registry; the two supplied numeric
cases; the check that every row has a catching gate and an owning bead.
Out: the protected mechanisms (the beads named per row); the known-answer registry `N31-KA`; running the
controls (each gate's bead and the battery N31b-05).

## Technical approach

**Table 1: catalogue entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Gate that must catch the control | Planted as; owner |
|---|---|---|---|---|---|
| CX-336 | Assemble the peak of every load channel into one design case | A simply supported span L carries P at L/3 at one instant and P at 2L/3 at another. The mid-span moment is P·L/6 each time. The assembled case gives P·L/3, a twofold over-design. With the second load of opposite sign the assembled case gives 0 while each real case gives ±P·L/6: unsafe (known answer KA-740) | A load case is one instant of one episode; peaks from different instants are never assembled (ADR-FOUNDRY-03, INV-FOUNDRY-02); refusal FROB-REF-167 | GATE-FOUNDRY-02 | Mutation `assemble-peaks` in the envelope-case builder: per-channel maxima joined into one case. BD-400 |
| CX-337 | Symmetric limbs carry equal loads | The chapter gives no numbers. Supplied case (DESIGN, synthetic): a four-legged stance, body weight 200 N on the centreline, payload 50 N offset 0.05 m to the left, track width 0.30 m. Left side carries 100 + 50·(½ + 0.05/0.30) = 133.33 N and the right 116.67 N: 66.67 N against 58.33 N per leg, 14.3% apart | Each limb's history is transformed into the canonical limb frame and the cases are unioned (ADR-FOUNDRY-04); under a mirror, forces transform as polar vectors and moments as axial ones | GATE-FOUNDRY-02 | Mutation `assume-equal-limbs`: one limb's cases copied to its mirror image. BD-400 |
| CX-338 | Count rainflow cycles per episode and add the results | Sequence −2, 1, −3, 5, −1, 3, −4, 4, −2 in units of 40 MPa, curve N(S) = 10⁶·(100 MPa/S)³ on amplitude, no mean-stress correction. Whole history: ranges 3, 4, 6, 8, 9 with counts 0.5, 1.5, 0.5, 1.0, 0.5; damage 8.752 × 10⁻⁶. Split after the fifth point (the fifth point closes the first window and opens the second) and finish each window: half cycles 3, 4, 8, 6 and 4, 7, 8, 6; the range-9 half cycle is lost; damage 7.816 × 10⁻⁶, 10.7% low (KA-742) | Residues are carried between episodes and `finish` is called once (metamorphic relation MR-34) | GATE-FOUNDRY-03 | Mutation `finish-per-window`. BD-402 |
| CX-339 | Feed unsigned von Mises stress to a cycle counter | The chapter gives no numbers. Supplied case (DESIGN, synthetic): n fully reversed cycles of amplitude S. Signed history: n cycles of range 2·S, damage n/N(S). The magnitude of the same history: 2·n cycles of range S about mean S/2, amplitude S/2, damage 2·n/N(S/2) = 2·n/(8·N(S)) with exponent 3 and no mean correction: one quarter of the true damage | Signed stress on a gauge plane only; an unsigned equivalent stress is refused at the counter's input | GATE-FOUNDRY-03 | Planted input `unsigned-von-mises` offered to the spectrum builder. BD-402 |
| CX-340 | A zero first-order sensitivity means the feature may be loosened | q(x) = x² at nominal zero: the sensitivity is zero and a first-order allocator loosens without limit, while the deviation at tolerance t is t² (KA-745, control) | Zero or unknown sensitivity is refused (FROB-REF-165) and routed to the band-extreme check, then to worst-case boxes | GATE-FOUNDRY-08 | Planted feature `quadratic-at-zero`. BD-406 |
| CX-341 (same case as the shared-bus counterexample of the design studio and the co-design lane) | Prune a design whose component peaks add past the bus limit | Two gripper fingers each need one powered tick and the bus powers one finger per tick. With a one-tick deadline no schedule works; with two ticks exactly two do; the motor limits are identical in both cases. Adding both peaks prunes the feasible two-tick design | A prune is rejected if any discarded design is independently feasible under the same mission; a failed simulation removes a whole part family only under an algebraic bound | GATE-FOUNDRY-10 (cross-consistency of the parts list after snapping) | Mutation `additive-peak-prune` in the parts-list snap and prune step. BD-408 |
| CX-342 | Q15 multiplication cannot overflow | (−1)·(−1) is 0x8000·0x8000 = 0x40000000, which shifts right by 15 to 0x8000 and reads as −1; the negation of −32768 overflows (KA-749) | Fixed-point obligations decided by bit-vector queries; FROB-REF-827 | GATE-FOUNDRY-11 | Planted controller `q15-mul-unguarded` and `q15-neg-unguarded`. BD-438, pilot N31b-10 |
| CX-343 | An integrator without a clamp is safe if each step is small | Adding 400 counts per tick from zero overflows sixteen bits at tick 82 (400 × 81 = 32,400; 400 × 82 = 32,800 > 32,767); without a clamp the one-step (inductive) query returns the state 32,400 at once | Stateful elements need an inductive range invariant: one base query and one step query | GATE-FOUNDRY-11 | Planted controller `pi-no-clamp`. BD-438, pilot N31b-10 |
| CX-344 | Evaluate a life law at the average condition | Insulation life halves per 10 K. Half the time at T_ref and half at T_ref + 20 K consumes life at (1 + 4)/2 = 2.5/τ_ref. The law at the mean temperature T_ref + 10 K gives 2.0/τ_ref and overstates service duration by 25% (KA-752) | Micro windows average the rate, never the input | GATE-FOUNDRY-12 | Mutation `average-input-then-law`. BD-423, pilot N31b-11 |

**Table 2: cases stated in the chapter without a catalogue number.** They are entered in the negative-control
registry now; a CX number is requested from the catalogue owner (numbers are never reused or renumbered; new
rows append).

| Local id | The tempting shortcut | The case | The rule it protects | Gate | Planted as; owner |
|---|---|---|---|---|---|
| L31-1 | A mechanical pass of a packet is a review | A two-part bracket whose second fastener sits behind the first part once it is mounted. Each part passes the per-part tool-access check, the packet passes pre-review items M1 to M6, and the assembly cannot be completed in the listed order | Rubric item R2 (assembly and tool access) belongs to a human; a mechanical pass is never rendered as `review: accept` | GATE-FOUNDRY-13 | Fixture `bracket-enclosed-fastener`; the control is a renderer that prints `review: accept` after M1 to M6. N31b-32, N31b-33 |
| L31-2 | A tier bias that is negligible in a task score is negligible in a damage sum | Damage scales as stress range to the power m of the stress-life law. A load 10% high gives 1.1⁵ = 1.61 times the damage at m = 5 and 1.1¹⁰ = 2.59 times at m = 10 | Envelope cases are re-extracted on the referee before they enter a packet; spectra are rescaled to the referee's range | GATE-FOUNDRY-14 | Planted fast-tier artifact: a stiffness spike that doubles one peak on the fast tier only. N31b-35, N31b-37 |
| L31-3 | A screening score may take a declared wall at face value | A cantilevered tube 0.3 m long, 600 N across the tip, S_y = 276 MPa, safety factor 2, mean radius 15 mm needs t ≥ 1.85 mm and 0.141 kg. A design declaring the segment at 0.05 kg is under-built by a factor of 2.8 and its swing inertia is understated by the same factor, so it scores better than its buildable version | Member sizing as a validity check at preflight and after first rollouts; promotion blocked for `under-built` links | GATE-FOUNDRY-15 | Planted design `quad-thin-shank`. N31b-39, N31b-40 |
| L31-4 | Fresh random worlds per iteration are a fair test of loop convergence | With fresh worlds each iteration the sampling noise of the load envelope sets a floor above the 2% test and the test cannot pass; the iteration map is deterministic only when the campaign reuses its world addresses | The load campaign is paired across loop iterations by world address | GATE-FOUNDRY-06 | Mutation `resample-worlds-each-iteration`. BD-404 |
| L31-5 | Re-select actuators inside the structure loop | Structure gain λ = 0.15 and actuator-spiral gain κ = 0.59 in one loop give about 0.74; five iterations contract the first step only to 0.74⁴ = 0.30 of itself, and a discrete catalogue can cycle between sizes | Actuator cards are held fixed inside the structure loop (ADR-FOUNDRY-07) | GATE-FOUNDRY-06 (cycling detected) | Planted two-size cycle. BD-404 |
| L31-6 | A surface file needs no unit statement | A surface written in millimetres and read as metres has 10⁹ times the volume | Cross-export identity (pre-review item M2) reports the ratio and names the file | GATE-FOUNDRY-13 | Planted export `stl-in-millimetres`. N31b-32 |

Related entries owned by the constructions registry, cited by Foundry's tests and not duplicated here: the
residue case "history 0, 10, 0 split into the windows [0, 10] and [10, 0] holds one full cycle of range 10; a
counter that drops residues counts none" and the rare-overload case "with exponent 3, 10⁶ cycles at a life of
2 × 10⁶ give damage 0.5 and 10³ cycles at five times the amplitude (life 16,000) add 0.0625: 0.1% of the cycles
carry 11% of the damage".

**Control records.** For each row one record in `frob-conformance/registry/controls.toml` (DESIGN, the field
set of the negative-control registry): `id` (stable, never reused), `seed` (the CX number or the local id),
`gate` (every gate that must catch it), `kind` (`mutation`, `planted-input`, `planted-design`), `target` (the
module path, for example `frob-foundry::loads::envelope`), `defect` (one sentence), `expect` (the observable
that proves the catch, with its number, for example "damage 7.816e-6 where 8.752e-6 is true; the row fails"),
and `last_seen`. A mutation is compiled only under the test-only feature `planted-defects` of `frob-foundry`
and can never be enabled in a release build (checked by a build-graph test).

## Acceptance criteria

- [ ] Fifteen control records exist (nine catalogue, six late), each naming at least one gate and one owning bead that exists.
- [ ] CX-337 and CX-339 each have a numeric case on file under `docs/derivations/foundry/` (133.33 N against 116.67 N; one quarter of the true damage), marked DESIGN.
- [ ] Every control has been observed to fail its gate at least once (`last_seen.caught = true`), recorded by the battery or the gate's own script.
- [ ] A registry self-test fails when a row is removed, when a row names a missing gate, or when a planted mutation is reachable without the `planted-defects` feature.
- [ ] The six late cases are submitted for catalogue numbers and the request is on file.

## Test plan

### Unit tests
`cargo test -p frob-conformance --test foundry_cx`:
- `cx336_assembled_peaks`: with P = 90 N and L = 0.6 m the per-instant moment is 9 N·m, the assembled case gives 18 N·m, and with opposite signs 0.
- `cx337_offset_payload`: 133.33 N and 116.67 N by side; 66.67 N and 58.33 N per leg.
- `cx338_window_split`: whole-history sum of count × range³ = 1,094 and damage 8.752 × 10⁻⁶; split sum 977 and damage 7.816 × 10⁻⁶; ratio 0.8931.
- `cx339_unsigned_quarter_damage`: ratio of unsigned to signed damage exactly ¼ at exponent 3.
- `cx340_quadratic_zero_sensitivity`; `cx341_two_finger_bus` (zero schedules at one tick, two at two ticks).
- `cx342_q15_products`: 0x8000 × 0x8000 >> 15 = 0x8000; `cx343_integrator_tick_82`; `cx344_rate_average` (2.5 against 2.0).
- `late_cases_numbers`: 1.1⁵ = 1.61051; 1.1¹⁰ = 2.5937; 0.141/0.05 = 2.8; 0.74⁴ = 0.2999; 10⁹.
- `registry_rows_complete`; `planted_defects_feature_off_in_release`.

### End-to-end script and logging
`scripts/e2e/foundry_cx_registry.sh`: runs `cargo test -p frob-conformance --test foundry_cx`, then for each
control with a verb-level form runs the gate's command with the control enabled and asserts the documented
failure (for example `frob foundry loads --fixture span-two-instants --json` with the `assemble-peaks` build
returns exit 3 and `FROB-REF-167`). Logs per STD-E2E to `target/e2e-logs/foundry_cx_registry/`: per control its
id, seed, gate, the expected and observed values, `caught`; the `summary` line counts controls caught and lists
the gates touched. The script exits non-zero if any control is not caught.

### Known answers and negative controls
The values in the two tables are the known answers of this bead. The registry's own negative controls: a row
with its gate deleted, a control whose expected value was edited to the mutated value (the self-test must flag
a control that passes its gate), and a release build with a planted mutation linked. Determinism: all cases
are exact integer or rational arithmetic and are identical on every machine.

## Considerations

- The catalogue prints "no numeric case is given" for CX-337 and CX-339. The supplied cases are DESIGN choices; if the catalogue owner prefers other numbers the derivation file is the place to change them, before the first gate run.
- CX-341 is the same case as the shared-bus counterexample used by the design studio's sizing analysis and by the co-design lane; one control is registered and attached to every gate that must catch it.
- A control that has never failed its gate in any recorded run is suspect and is reviewed at each milestone exit.
- The three Moonshot rows (CX-342 to CX-344) are exercised first by their pilots; they bind a gate only if the corresponding epic is funded.

## References

Appendix N (N.28), §31.6, §31.8, §31.9, §31.11, §31.12, §31.13, §31.14, §31.21 to §31.23; KA-740, KA-742, KA-745, KA-749, KA-752; MR-34; ADR-FOUNDRY-03, -04, -07; C-35.
