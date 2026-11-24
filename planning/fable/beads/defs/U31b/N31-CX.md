---
key: "N31-CX"
title: "Counterexample registry of Foundry (§31): CX-336 to CX-344 and six late cases, with controls and gates"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "foundry", "frob-foundry", "frob-conformance", "frontier", "registry", "test"]
parent: "WP-70"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N31-KA", "N31-REF", "N31a-01", "N31a-10", "N31a-11", "N31a-21", "N31a-33", "N31a-39", "N31a-45", "N31a-47", "N31a-48", "N31a-52", "N31a-53", "N31a-55", "N31a-56", "N31a-57", "N31a-58", "N31a-59", "N31b-01", "N31b-04", "N31b-05", "N31b-10", "N31b-11", "N31b-32", "N31b-33", "N31b-34", "N31b-35", "N31b-36", "N31b-37", "N31b-39", "N31b-40", "N31b-43", "N38b-04"]
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
further cases stated in the chapter's late subsections that have no catalogue number yet. Tier: the registry
itself is plain bookkeeping; the mechanisms it protects are Frontier [F] and, for CX-342 to CX-344, Moonshot [M].

Design reason for keeping the numbers in the registry: the cases are small on purpose, so that a reviewer can
recompute them by hand, and so that a test that passes by accident can be told from one that checks the right
thing. Two catalogue entries are printed without a numeric case (CX-337, CX-339); this bead supplies one for
each, labelled DESIGN, so that the control is executable.

## Scope

In: the fifteen entries of the catalogue `crates/frob-conformance/registry/counterexamples.toml` (nine numbered,
six late), each with the substance a control needs (kind, plant, target, defect, expected observation, the gates
among `rule_ids`); the two supplied numeric cases with their derivation files; the block request for the six late
numbers; a host test that recomputes every number; the registry script.
Out, with the bead that has it: the catalogue's record schema and loader, and the seeding tool that writes
`crates/frob-conformance/registry/controls.toml`, allocates `NC-` numbers, keeps the seed map and proves
completeness by the join (N38b-05). This bead writes no row of `controls.toml`: its controls appear there when the
seeding is re-run over its entries, which is append-only and idempotent. The control runner (N38b-04). The number
blocks (N38a-40). The protected mechanisms and their planted variants (the beads named per row); the known-answer
registry `N31-KA`; running the controls (each gate's bead and the battery N31b-05).

## Technical approach

**Data.** Each row is one entry in the catalogue's record schema (N38b-05): `id` (the CX number, or the tag for a
late entry until its block is issued), `shortcut`, `case`, `numeric` (true for all fifteen), `plan_case`
(`"words"` for CX-337 and CX-339, which are two of the catalogue's seventeen word-only entries; `"numbers"` for the
others), `case_supplied_by` (for CX-337 and CX-339: `"N31-CX: docs/derivations/foundry/cx337.md"` and
`"N31-CX: docs/derivations/foundry/cx339.md"`; empty otherwise), `rule`, `rule_ids` (the gates of the gate column
with the decisions, invariants, relations and refusal codes of the rule column), `defined_in`, `same_as` (empty for
all nine: the catalogue marks none of them ≡, so the plan-time count of 13 `same_as` entries is unchanged),
`label` (DESIGN), `control_kind`, `plant` and, for a late entry, `slug`. The control column gives the control's
kind; plant; target; defect and expected observation: its first word is `control_kind`, the identifier in
backquotes is the name of the entry's plant, whose `id` is `<planting bead>:<identifier>` as N38b-05 requires,
the planting bead being the first bead of the "Planted and caught by" column (`BD-400:assemble-peaks`,
`BD-400:assume-equal-limbs`, `BD-402:finish-per-window`, `BD-402:unsigned-von-mises`, `BD-406:quadratic-at-zero`,
`BD-408:additive-peak-prune`, `N31a-45:q15-mul-unguarded` and `N31a-45:q15-neg-unguarded`, `N31a-45:pi-no-clamp`,
`N31a-47:average-input-then-law`; for the late entries `N31b-32:bracket-enclosed-fastener`,
`N31b-35:fast-tier-stiffness-spike`, `N31b-39:quad-thin-shank`, `BD-404:resample-worlds-each-iteration`,
`BD-404:two-size-cycle` and `N31b-32:stl-in-millimetres`); the name after the colon stays the Cargo feature or the
fixture. The target, defect and expected observation are the texts the seeding tool
copies into the control's `target`, `defect` and `expect` (BD-025 requires `defect` and `expect` non-empty). A
`mutation` plant is a Cargo feature of the owning crate (`frob-foundry` or `frob-lifecycle`), written by the
owning mechanism bead and compiled only under the `gauntlet-mutations` umbrella (BD-025); a build with such a
feature but without the umbrella, or under the release profile, is a compile error, and product binaries are
checked to carry none (`cargo tree -e features`). Other kinds name a fixture under
`crates/frob-conformance/fixtures/foundry/` with a `synthetic` kind header. Every gate among an entry's
`rule_ids` must fail with the plant enabled.

**Table 1: catalogue entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Gate that must catch the control | Control (kind; plant; target; defect; expected observation) | Planted and caught by |
|---|---|---|---|---|---|---|
| CX-336 | Assemble the peak of every load channel into one design case | A simply supported span L carries P at L/3 at one instant and P at 2L/3 at another. The mid-span moment is P·L/6 each time. The assembled case gives P·L/3, a twofold over-design. With the second load of opposite sign the assembled case gives 0 while each real case gives ±P·L/6: unsafe (known answer KA-740) | A load case is one instant of one episode; peaks from different instants are never assembled (ADR-FOUNDRY-03, INV-FOUNDRY-02); refusal FROB-REF-167 | GATE-FOUNDRY-02 | mutation; `assemble-peaks`; `frob-foundry::loads::envelope`; per-channel maxima joined into one case; mid-span moment P·L/3 (18 N·m at P = 90 N, L = 0.6 m) where 9 N·m per instant is true, 0 with opposite signs, and FROB-REF-167 does not fire | BD-400; M5 |
| CX-337 | Symmetric limbs carry equal loads | The chapter gives no numbers. Supplied case (DESIGN, synthetic): a four-legged stance, body weight 200 N on the centreline, payload 50 N offset 0.05 m to the left, track width 0.30 m. Left side carries 100 + 50·(½ + 0.05/0.30) = 133.33 N and the right 116.67 N: 66.67 N against 58.33 N per leg, 14.3% apart | Each limb's history is transformed into the canonical limb frame and the cases are unioned (ADR-FOUNDRY-04); under a mirror, forces transform as polar vectors and moments as axial ones | GATE-FOUNDRY-02 | mutation; `assume-equal-limbs`; `frob-foundry::loads::symmetry`; one limb's cases copied to its mirror image; the right legs are sized for 66.67 N where 58.33 N is true and the left legs for 58.33 N where 66.67 N is true | BD-400; M5 |
| CX-338 | Count rainflow cycles per episode and add the results | Sequence −2, 1, −3, 5, −1, 3, −4, 4, −2 in units of 40 MPa, curve N(S) = 10⁶·(100 MPa/S)³ on amplitude, no mean-stress correction. Whole history: ranges 3, 4, 6, 8, 9 with counts 0.5, 1.5, 0.5, 1.0, 0.5; damage 8.752 × 10⁻⁶. Split after the fifth point (the fifth point closes the first window and opens the second) and finish each window: half cycles 3, 4, 8, 6 and 4, 7, 8, 6; the range-9 half cycle is lost; damage 7.816 × 10⁻⁶, 10.7% low (KA-742) | Residues are carried between episodes and `finish` is called once (metamorphic relation MR-34) | GATE-FOUNDRY-03 | mutation; `finish-per-window`; `frob-foundry::check::fatigue`; `finish` called per window; damage 7.816e-6 where 8.752e-6 is true; the row fails | BD-402; M5 |
| CX-339 | Feed unsigned von Mises stress to a cycle counter | The chapter gives no numbers. Supplied case (DESIGN, synthetic): n fully reversed cycles of amplitude S. Signed history: n cycles of range 2·S, damage n/N(S). The magnitude of the same history: 2·n cycles of range S about mean S/2, amplitude S/2, damage 2·n/N(S/2) = 2·n/(8·N(S)) with exponent 3 and no mean correction: one quarter of the true damage | Signed stress on a gauge plane only; an unsigned equivalent stress is refused at the counter's input | GATE-FOUNDRY-03 | planted-input; `unsigned-von-mises`; the spectrum builder of `frob-foundry::check::fatigue`; an unsigned equivalent stress offered to the counter with the input check removed; damage one quarter of the signed value, where the real builder refuses the input | BD-402; M5 |
| CX-340 | A zero first-order sensitivity means the feature may be loosened | q(x) = x² at nominal zero: the sensitivity is zero and a first-order allocator loosens without limit, while the deviation at tolerance t is t² (KA-745, control) | Zero or unknown sensitivity is refused (FROB-REF-165) and routed to the band-extreme check, then to worst-case boxes | GATE-FOUNDRY-08 | planted-design; `quadratic-at-zero`; `foundry.tolerance`; a feature whose response is x² at nominal zero, offered with the band-extreme check removed; the allocator loosens the tolerance without limit and FROB-REF-165 does not fire | BD-406; M5 |
| CX-341 | Prune a design whose component peaks add past the bus limit | Two gripper fingers each need one powered tick and the bus powers one finger per tick. With a one-tick deadline no schedule works; with two ticks exactly two do; the motor limits are identical in both cases. Adding both peaks prunes the feasible two-tick design | A prune is rejected if any discarded design is independently feasible under the same mission; a failed simulation removes a whole part family only under an algebraic bound | GATE-FOUNDRY-10 (cross-consistency of the parts list after snapping) | mutation; `additive-peak-prune`; the parts-list snap and prune step of the packet writer; peaks added across the bus; the feasible two-tick design is pruned and the parts list loses its family | BD-408; M5 |
| CX-342 | Q15 multiplication cannot overflow | (−1)·(−1) is 0x8000·0x8000 = 0x40000000, which shifts right by 15 to 0x8000 and reads as −1; the negation of −32768 overflows (KA-749) | Fixed-point obligations decided by bit-vector queries; FROB-REF-827 | GATE-FOUNDRY-11 | planted-design; `q15-mul-unguarded` and `q15-neg-unguarded` (two plants, one control each); the overflow queries of the firmware export; a controller with an unguarded Q15 multiply and an unguarded negation offered for export; each must be refused with FROB-REF-827 and its counterexample (0x8000 × 0x8000, −(−32768)); with the query removed both are exported | N31a-45, after the pilot N31b-10; M8 |
| CX-343 | An integrator without a clamp is safe if each step is small | Adding 400 counts per tick from zero overflows sixteen bits at tick 82 (400 × 81 = 32,400; 400 × 82 = 32,800 > 32,767); without a clamp the one-step (inductive) query returns the state 32,400 at once | Stateful elements need an inductive range invariant: one base query and one step query | GATE-FOUNDRY-11 | planted-design; `pi-no-clamp`; the state-invariant queries of the firmware export; a proportional–integral controller without a clamp; the step query must return the state 32,400; with the step query removed the controller is certified | N31a-45, after the pilot N31b-10; M8 |
| CX-344 | Evaluate a life law at the average condition | Insulation life halves per 10 K. Half the time at T_ref and half at T_ref + 20 K consumes life at (1 + 4)/2 = 2.5/τ_ref. The law at the mean temperature T_ref + 10 K gives 2.0/τ_ref and overstates service duration by 25% (KA-752) | Micro windows average the rate, never the input | GATE-FOUNDRY-12 | mutation; `average-input-then-law`; `frob-lifecycle` micro windows; the law evaluated at the averaged temperature; consumption 2.0/τ_ref where 2.5/τ_ref is true | N31a-47 (micro windows) with the insulation law of N31a-48, after the pilot N31b-11; M6 |

**Table 2: cases stated in the late subsections without a catalogue number.** They are appended to the catalogue,
which is append-only. A bead never invents a number above the plan-time maximum (CX-384): the numbers come from the
allocation ledger `crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`, N38a-40).
This bead files one `[[block]]` request with `registry = "cx"`, `count = 6`, `owner = "N31-CX"`,
`section = "§31.21, §31.22, §31.23"`; the allocator fills `first` and `last`, and the six tags are mapped to the
block's numbers in table order in one commit. Until then the tag is the entry's name; afterwards it stays in
`slug`. A tag still used as an identifier in a registry, a test name or a derivation file at this bead's milestone
(M5) fails the ledger check `provisional-tag-past-due`.

| Tag | The tempting shortcut | The case | The rule it protects | Gate | Control (kind; plant; target; defect; expected observation) | Planted and caught by |
|---|---|---|---|---|---|---|
| L31-1 | A mechanical pass of a packet is a review | A two-part bracket whose second fastener sits behind the first part once it is mounted. Each part passes the per-part tool-access check, the packet passes pre-review items M1 to M6, and the assembly cannot be completed in the listed order | Rubric item R2 (assembly and tool access) belongs to a human; a mechanical pass is never rendered as `review: accept` | GATE-FOUNDRY-13 | planted-design; `bracket-enclosed-fastener`; `foundry.check` and its review-record import; a renderer that prints `review: accept` after M1 to M6 with no human record; the packet is reported accepted although R2 fails | N31b-32 with N31b-33; M5 |
| L31-2 | A tier bias that is negligible in a task score is negligible in a damage sum | Damage scales as stress range to the power m of the stress-life law. A load 10% high gives 1.1⁵ = 1.61 times the damage at m = 5 and 1.1¹⁰ = 2.59 times at m = 10 | Envelope cases are re-extracted on the referee before they enter a packet; spectra are rescaled to the referee's range | GATE-FOUNDRY-14 | planted-world; `fast-tier-stiffness-spike`; the referee re-extraction of envelope cases; a stiffness spike that doubles one peak on the fast tier only, with re-extraction removed; the doubled peak enters the packet unrelabelled | N31b-35 with the gate script N31b-37; M5 |
| L31-3 | A screening score may take a declared wall at face value | A cantilevered tube 0.3 m long, 600 N across the tip, S_y = 276 MPa, safety factor 2, mean radius 15 mm needs t ≥ 180/(138 × 10⁶ × π × 0.015²) = 1.85 mm and 0.141 kg at 2,700 kg/m³. A design declaring the segment at 0.05 kg is under-built by a factor of 2.8 and its swing inertia is understated by the same factor, so it scores better than its buildable version | Member sizing as a validity check at preflight and after first rollouts; promotion blocked for `under-built` links | GATE-FOUNDRY-15 | planted-design; `quad-thin-shank`; the stage-6 member-sizing check; a design declaring the 0.05 kg segment, with the sizing check removed; the design is promoted with no `under-built` finding | N31b-39 with N31b-40; M3 |
| L31-4 | Fresh random worlds per iteration are a fair test of loop convergence | With fresh worlds each iteration the sampling noise of the load envelope sets a floor above the 2% test and the test cannot pass; the iteration map is deterministic only when the campaign reuses its world addresses | The load campaign is paired across loop iterations by world address | GATE-FOUNDRY-06 | mutation; `resample-worlds-each-iteration`; the loads–mass loop of `frob-foundry`; fresh world addresses per iteration; the convergence test never meets 2% and the loop reports noise as non-convergence | BD-404; M5 |
| L31-5 | Re-select actuators inside the structure loop | Structure gain λ = 0.15 and actuator-spiral gain κ = 0.59 in one loop give about 0.74; five iterations contract the first step only to 0.74⁴ = 0.30 of itself, and a discrete catalogue can cycle between sizes | Actuator cards are held fixed inside the structure loop (ADR-FOUNDRY-07) | GATE-FOUNDRY-06 (cycling detected) | planted-design; `two-size-cycle`; the loads–mass loop; a catalogue with two sizes that alternate when actuators are re-selected inside the loop; the cycle must be detected and refused; with re-selection allowed and cycle detection removed the loop runs to its iteration cap | BD-404; M5 |
| L31-6 | A surface file needs no unit statement | A surface written in millimetres and read as metres has 10⁹ times the volume | Cross-export identity (pre-review item M2) reports the ratio and names the file | GATE-FOUNDRY-13 | planted-input; `stl-in-millimetres`; pre-review item M2 of `foundry.check`; an exported surface in millimetres with no unit statement; M2 must report the 10⁹ ratio and name the file; with M2 removed the packet passes | N31b-32; M5 |

Related entries owned by the constructions registry, cited by Foundry's tests and not duplicated here: the
residue case "history 0, 10, 0 split into the windows [0, 10] and [10, 0] holds one full cycle of range 10; a
counter that drops residues counts none" and the rare-overload case "with exponent 3, 10⁶ cycles at a life of
2 × 10⁶ give damage 0.5 and 10³ cycles at five times the amplitude (life 16,000) add 0.0625: 0.1% of the cycles
carry 11% of the damage". CX-341 uses the same two-finger bus fixture as the studio's CX-303 and protects a
different rule (the parts-list prune of §31.12); the catalogue does not mark it ≡, so each keeps its own entry and
its own control, attached to its own gates.

**What the seeding tool writes from an entry** (the record of BD-025, for orientation): `id` (`NC-` plus four
digits, assigned by the tool), `seed` (the CX number; for a late entry its issued number), `gate`, `kind`,
`target`, `defect`, `expect`, `last_seen` (written by the control runner, never by hand), and `feature` for a
`mutation` or `fixture` for the other kinds. Until the owning bead has written its branch or fixture the runner
reports the control `stale`; `caught` and `last_seen` are written by the session that records the gate, with the
run in `registry/control_history.ndjson`, never by this bead.

## Acceptance criteria

- [ ] `counterexamples.toml` holds the nine entries CX-336 to CX-344 and the six late entries, each with
      `control_kind`, a `plant` and its gates among `rule_ids`, every planting bead in the last column live in the
      bead index. After the seeding tool of N38b-05 is re-run, each entry has exactly one control in
      `controls.toml` whose `seed` names it (CX-342 has two, one per plant), the tool's second run changes no byte,
      the join passes, and no entry ends in the seeding tool's findings file. No record of `controls.toml` is
      written by this bead.
- [ ] CX-337 and CX-339 carry `plan_case = "words"`, `numeric = true` and `case_supplied_by` naming their
      derivation files under `docs/derivations/foundry/` (133.33 N against 116.67 N; one quarter of the true
      damage), marked DESIGN; the word-only count of N38b-05 stays 17.
- [ ] The six late entries carry numbers from a block issued to N31-CX in `allocations.toml`, in table order, with
      their tags in `slug`; `registry::alloc::validate` passes and no tag remains as an identifier at M5.
- [ ] The registry test fails when an entry is removed, when an entry names a missing gate or a planting bead
      absent from the index, or when an entry's numbers differ from the recomputation.
- [ ] `scripts/e2e/foundry_cx_registry.sh` passes and rejects each of its planted registry defects. Observing
      every control `caught` is a condition of each gate's own record (the gate scripts and the battery N31b-05),
      not of this bead, so the registry can close before the mechanisms exist.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test foundry_cx` (real registry files and scratch copies, no mocks):
- `cx336_assembled_peaks`: with P = 90 N and L = 0.6 m the per-instant moment is 9 N·m, the assembled case gives
  18 N·m, and with opposite signs 0.
- `cx337_offset_payload`: 133.33 N and 116.67 N by side; 66.67 N and 58.33 N per leg; ratio 1.143.
- `cx338_window_split`: whole-history sum of count × range³ = 1,094 and damage 8.752 × 10⁻⁶; split sum 977 and
  damage 7.816 × 10⁻⁶; ratio 0.8931.
- `cx339_unsigned_quarter_damage`: ratio of unsigned to signed damage exactly ¼ at exponent 3.
- `cx340_quadratic_zero_sensitivity` (sensitivity 0, deviation t² at t = 0.1 is 0.01); `cx341_two_finger_bus`
  (zero schedules at one tick, two at two ticks).
- `cx342_q15_products`: 0x8000 × 0x8000 >> 15 = 0x8000; −(−32768) not representable in i16;
  `cx343_integrator_tick_82` (32,400 at tick 81, 32,800 > 32,767 at tick 82); `cx344_rate_average` (2.5 against
  2.0; ratio 1.25).
- `late_cases_numbers`: 1.1⁵ = 1.61051; 1.1¹⁰ = 2.5937; t = 1.845 mm and 0.1409 kg, 0.141/0.05 = 2.8;
  0.15 + 0.59 = 0.74 and 0.74⁴ = 0.2999; 1000³ = 10⁹.
- `registry_rows_complete` (fifteen entries by tag or number, each with a gate in the gate register and a live
  planting bead); `block_request_of_six`; `word_only_supplied` (CX-337, CX-339 by `plan_case`, with
  `case_supplied_by` filled).

### End-to-end script and logging

`scripts/e2e/foundry_cx_registry.sh` (header `bead=N31-CX milestone=M5 labels=foundry,registry
plants=drop-entry,unknown-gate,expect-equals-truth,stale-tag`): runs `cargo test -p frob-conformance --test
foundry_cx`; re-runs the seeding tool (`cargo test -p frob-conformance --test seed_controls -- --ignored seed`)
and the join (`--test seed_join`) on a scratch copy of `registry/` and asserts by digest that a second seeding
run is a no-op; then plants on scratch copies, one at a time, an entry with its gate deleted, an entry naming a
gate absent from the register, an entry whose expected observation equals the true value (a control that plants
nothing), and a tag left as an identifier at M5, and asserts each is rejected. For each control whose planting
bead has landed it runs the gate's tier target with the plant off and on, in the form of ADR-GAUNT-01:
`FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier from the gate record> -- <gate>`, the second
time with `--features gauntlet-mutations,<crate>/<feature>`, and asserts pass then fail. It calls no `frob`
verb of a Foundry mechanism (those beads are not its dependencies): the verb-level catch, for example
`foundry.loads` on the two-instant span returning `FROB-REF-167` under the `assemble-peaks` build, is asserted by
the gate's own script (BD-400's). Controls whose planting bead is open are logged `stale` with that bead and do not
hold this bead open; the catch that counts is recorded by each gate's own script. Logs per STD-E2E
to `target/e2e-logs/foundry_cx_registry/<utc-timestamp>.ndjson` with a transcript: every line has `ts`, `script`,
`bead`, `step`, `event`, the exact command line (with its feature list), exit code, wall time and the evidence
block (claim Known answer, class Tested, determinism D0); per control its CX number or tag, its `NC-` identifier,
plant, gate, planting bead, the expected and observed values and the status (`caught`, `missed`, `stale`); on a
miss the first differing observable and the mutant build's feature list to reproduce; the digests of the catalogue
and of the scratch `controls.toml`. The `summary` line counts controls caught, stale and missed and lists the
gates touched. The script exits non-zero on any `missed` or `caught-unexpected`, and on any `stale` control whose
planting bead is closed.

### Known answers and negative controls

The values in the two tables are the known answers of this bead (KA-740, KA-742, KA-745, KA-749, KA-752 of
N31-KA for the catalogue rows). The registry's own negative controls: an entry with its gate deleted, an entry
whose expected value equals the true value (the self-test must flag a control that plants nothing), a release
build with a planted mutation linked (a compile error), and a stale tag. Determinism: all cases are exact integer
or rational arithmetic and are identical on every machine.

## Considerations

- The catalogue prints "no numeric case is given" for CX-337 and CX-339. The supplied cases are DESIGN choices; if
  another case is preferred the derivation file is the place to change it, before the first gate run.
- A control that has never failed its gate in any recorded run is suspect and is reviewed at each milestone exit.
- The three Moonshot rows (CX-342 to CX-344) are exercised first by their pilots; they bind a gate only if the
  corresponding work is funded. If KILL-FIRMWARE or KILL-LIFECYCLE fires, their controls are withdrawn with the
  mechanism, with a recorded reason, and their entries stay in the catalogue.
- Earlier drafts named the epics BD-438 and BD-423 as owners; an epic carries no work, so the planting beads are
  the task beads that build the overflow queries (N31a-45) and the micro windows (N31a-47, N31a-48).

## References

Appendix N (N.28), §31.6, §31.8, §31.9, §31.11, §31.12, §31.13, §31.14, §31.21 to §31.23; KA-740, KA-742,
KA-745, KA-749, KA-752; MR-34; ADR-FOUNDRY-03, -04, -07; C-35.
