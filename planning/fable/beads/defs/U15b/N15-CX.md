---
key: "N15-CX"
title: "registry: Sinew counterexamples CX-135 to CX-143 as negative controls, plus four from coupled transmissions"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "sinew", "frob-actuate", "frob-conformance", "solid", "registry", "test"]
parent: "WP-09"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N15-KA", "N15-REF", "N15a-12", "N15a-14", "N15a-15", "N15a-16", "N15a-24", "N15b-05", "N15b-06", "N15b-07", "N15b-10", "N15b-11", "N15b-12", "N15b-13", "N15b-30", "N15b-32", "N15b-33", "N15b-34", "N15b-35", "N15b-37", "N15b-39", "N15b-40", "N15b-41"]
gates: []
plan_refs: ["§15.7", "§15.8", "§15.9", "§15.10", "§15.11", "§15.12", "§15.15", "Appendix N.12", "§0.7", "§38"]
---
## Background

A plan rule binds every chapter: where a plausible shortcut is wrong, the smallest hand-checkable case that
breaks it is written down, and that case enters the counterexample catalogue and the negative-control registry
of the verification program (crate `frob-conformance`, seed bead BD-025). The reason is that a gate nothing can
fail validates nothing: a check is only trusted once a planted defect has been shown to trip it. For Sinew
(actuators, transmissions, power; crate `frob-actuate`) the catalogue holds nine counterexamples, CX-135 to
CX-143. The late subsection on coupled transmissions (§15.15) states four more shortcuts that the printed
catalogue does not number; this bead enters them too, with numbers from a block of the allocation ledger.

Each entry states the tempting shortcut, the case with its numbers, the rule it protects, and the planted
defect that realizes the shortcut, with the bead that plants it.

## Scope

In: the nine entries CX-135 to CX-143 and the four late entries in the counterexample catalogue
`crates/frob-conformance/registry/counterexamples.toml`, with the substance each needs to become a control (rule
identifiers, kind of control, the planted-defect identifiers, the expected observation); the block request for
the four late entries; a host test that recomputes every number in the tables.

Out, with the bead that has it: the catalogue's record schema and loader, and the seeding tool that turns
entries into rows of `registry/controls.toml`, allocates their `NC-` identifiers, keeps the seed map, resolves
rules that name no gate and proves completeness by the join (N38b-05); this bead writes no row of
`controls.toml` by hand. Running controls and writing `last_seen` (N38b-04). The number blocks (N38a-40).
Building each planted defect in the real component, and the test that must go red when it is on: the planting
bead named in each row. The correct mechanisms are owned by the beads named in the last column of the first
table; the known-answer registry is N15-KA.

## Technical approach

**Entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Planted shortcut; caught by |
|---|---|---|---|---|
| CX-135 | One efficiency multiplier describes a gear in all four quadrants | η = 0.8, N = 10, the load driving the joint with 10 N·m at 1 rad/s: 10 W in. The single-multiplier law τ_joint = η·N·τ_motor gives τ_motor = 1.25 N·m at 10 rad/s, so 12.5 W arrives at the motor and 2.5 W came from nowhere. The four-quadrant law gives 0.8 N·m and 8 W, with 2 W lost | Four-quadrant loss with η_f and η_b; INV-SINEW-01; FROB-REF-130; GATE-SINEW-08 | The transmission applies η in every quadrant; BD-074 (the card check refuses; the passivity battery) |
| CX-136 | Copper resistance can be held constant in the thermal check | Card with R₀ = 0.20 Ω, α = 0.00393 K⁻¹, G = 0.80 W/K, C = 120 J/K, ambient 25 °C, limit 155 °C, at a constant 20 A. Constant resistance: rise 80/0.80 = 100 K, 125 °C, inside the limit, which is never reached. With R(T): s = 0.314 W/K, rise 80/(0.80 − 0.314) = 164.7 K, 190 °C, outside the limit, reached at 385 s. The largest continuous current falls from 22.8 A to 18.6 A, and above I_crit = 31.9 A there is no equilibrium at all | ADR-SINEW-04 (the static check uses the corrected law); FROB-REF-132; GATE-SINEW-05 | α set to zero inside the static check, and α ignored in the stepped L2 thermal; N15a-14 (static thermal library), BD-103 (stage 6 admits the 20 A design), BD-072 (stepped 20 A row) |
| CX-137 | Under torque holding the runaway threshold can be estimated from the slope of the loss curve at T₀, and the continuous rating leaves margin below it | Same card with α_B = −0.0011 K⁻¹. The slope estimate √(G/(R₀·(α − 2·α_B))) gives 25.5 A; the loss curve is convex and the equilibrium vanishes at a tangency at 16.5 A cold-equivalent. The limit is reached at 15.9 A, so the continuous rating and the runaway threshold are four percent apart | `actuator.thermal` solves the tangency and reports the margin to it; GATE-SINEW-05 (tangency within 1%) | The tangency solve replaced by the slope formula; N15a-14 (tangency solve), BD-072 |
| CX-138 | Episodes that start at ambient temperature test the thermal limits of a design | In 20 s from cold at 20 A the winding of the card rises by 164.7·(1 − e^(−20/247)) = 12.8 K, against an equilibrium rise of 164.7 K. A policy trained and evaluated on such episodes never meets derating | ADR-SINEW-08: thermal state is initialized from a declared duty history (`cold`, `soaked(T)`, `steady(task)`); evaluation defaults to `steady(task)` | Evaluation always starts cold; N15a-15 (the declared duty history and the evaluation default `steady(task)`, which reaches derating inside the episode); BD-071 only consumes the resolved initial temperatures |
| CX-139 | A torque-speed envelope without the voltage ellipse is good enough above base speed | Card with λ = 0.010 V·s, L = 100 µH, I_max = 40 A, V_bus = 48 V: base speed 3,151 rad/s (electrical), no-load speed 3,394 rad/s. At 1.5 times base speed the motor delivers 59% of peak torque with field weakening (i_d = −32.2 A, i_q = 23.7 A) and none without it, because that speed is above the no-load speed | The dq voltage limit at L2; `actuator.envelope`; GATE-SINEW-06; the L1 flag when L_q·I_max exceeds one tenth of the flux linkage | The voltage ellipse dropped (full torque at every speed); N15a-16 (static envelope library), BD-073 |
| CX-140 | Reflected rotor inertia is diagonal in joint space | Symmetric differential with q₁ = (θ₁ + θ₂)/2 and q₂ = (θ₁ − θ₂)/2: the reflected inertia T⁻ᵀ·diag(J_r)·T⁻¹ is [[J₁ + J₂, J₁ − J₂], [J₁ − J₂, J₁ + J₂]]. Unequal rotors couple the two joints | Coupling transmissions as a matrix transformer; the known answer of §15.9 | Off-diagonal entries dropped; BD-074, N15b-11 |
| CX-141 | A bus under load can always deliver the commanded power at a lower voltage (or: bus feasibility is a current limit) | Actuators under current control draw power. A bus with E = 48 V and R = 0.08 Ω can deliver at most E²/(4·R) = 7,200 W; at 1,800 W it sits at 44.8 V and 40.2 A. A demand above the ceiling has no operating point: the voltage collapses and the step records a brownout event; it never completes at the requested power | FROB-REF-133 at stage 6; the brownout event; GATE-SINEW-07 (100 of 100 trials, never a completed step); INV-SINEW-04 and FROB-REF-135 for exhaustion, which no kernel clips | Demand clipped to the ceiling and the step completed; state of charge clamped at zero while power is still supplied; BD-075, N15b-03, BD-151 |
| CX-142 | Losses that flow into thermal nodes are also listed as energy that left the system | Copper, iron, friction and gear losses are stored in thermal nodes; the energy that leaves is mechanical work at the joint and heat to ambient. Counting stored heat again as a loss produces a false defect in the balance equal to that heat | The identity of §15.11 (decrease of chemical energy = work out + increase of stored energies + heat to ambient + residual); INV-SINEW-02; GATE-SINEW-02 | Heat entering a node booked as stored and as lost; N15b-05, BD-076 |
| CX-143 | Agreement between the referee and the fast tiers is evidence that the actuator card is right | Every tier reads the same card. If the card is wrong, the referee, the CPU batch tier and both GPU tiers are wrong together, and their agreement says nothing about it | Randomization over the card's stated uncertainty; the `card-conditional` label of A-16; refusal of verdicts on `assumed` fields under `--strict-data`; measurement in §30 as the only evidence about the card | A cross-tier agreement raises the evidence class of a card-derived result; N15b-06, N15b-07 |

**Late entries** (§15.15, stated after the catalogue was compiled; cited by the tags CX-SINEW-a to -d until
N38a-40 issues their block, with the working name in `slug`). The fixture is the known answer of §15.15: one
base-mounted rotor with θ = 10·(q₁ + q₂), torque limit 0.1 N·m and inertia 0.01 kg·m².

| Tag (`slug`) | The tempting shortcut | The smallest case that breaks it | The rule it protects | Planted shortcut; caught by |
|---|---|---|---|---|
| CX-SINEW-a (`coupled-torque-box`) | The feasible joint torques are the box of per-joint limits | The true set is (t, t) with abs(t) ≤ 1 N·m; the box of per-joint limits of 1 N·m admits (1, −1) N·m, which no motor torque produces | §15.15 rule 1: the feasible joint-torque set is the image Cᵀ·B; GATE-SINEW-10; FROB-REF-131 | The box of per-joint limits; N15b-10 |
| CX-SINEW-b (`shared-rotor-diagonal`) | A shared rotor's inertia reflects diagonally | True reflected inertia [[1, 1], [1, 1]] kg·m²; rotor energy 0 J at joint rates (1, −1) rad/s and 2 J at (1, 1) rad/s; the diagonal form diag(1, 1) reports 1 J twice | §15.15 rule 2: rotor inertia is pulled back once, as Cᵀ·diag(J_r)·C; GATE-SINEW-10 | Off-diagonal entries dropped; N15b-11 |
| CX-SINEW-c (`power-balance-sign-blind`) | A power balance detects a reversed gear | Two unit scalar stores, efforts (2, 3), coefficient 4: one sign convention gives rates (12, −8), the other (−12, 8); both satisfy 2·12 + 3·(−8) = 0 | The signed motion witness (§15.15, DESIGN); the port-sign refusal of §14.6, FROB-REF-108; GATE-SINEW-10 | The ratio negated; N15b-12 (signed motion witness) |
| CX-SINEW-d (`linkage-constant-ratio`) | A linkage transmission can be modelled by a constant ratio | A coupling that varies with configuration contributes velocity-product terms from ½·q̇ᵀ·Cᵀ·J_r·C·q̇; the constant-ratio shortcut omits them | §15.15 rule 3: the inertial bias of a configuration-dependent coupling; GATE-SINEW-10 | The coupling frozen at its current value (bias zero); N15b-11 |

**Related entries owned by other chapters, cited by Sinew tests:** CX-128 (a contract that carries only peak
demands can admit and cannot exclude: two fingers on a one-unit bus), CX-172 (backlash as a dead zone on the
torque command produces no impact at gap closure), CX-303 (adding peak currents on a shared bus).

**Catalogue record.** Each row of the two tables is one entry of `registry/counterexamples.toml` in the record
schema of N38b-05: `id` (the CX number, append-only; for a late entry, empty until its number is issued),
`shortcut`, `case` and `rule` (the second to fourth columns, verbatim; the expected observation stays in `case`,
from which the seeding tool writes the control's `expect`), `numeric` and `plan_case` (true and `numbers` for all
thirteen), `rule_ids`, `defined_in` (§15.7 to §15.12 for the numbered entries, §15.15 for the late ones),
`label`, `control_kind`, `plant` and, for a late entry, `slug`. `plant` names each planted defect as
`<planting bead>:<name>`, the name being the Cargo feature under which that bead builds the shortcut in the real
component, never a mock (the plant names below are the feature names the planting beads already use). The
seeding tool writes one control per plant in BD-025's record format (`id`, `seed` = the CX number, `gate`,
`kind`, `target`, `defect`, `expect`, `feature`, `state`, `last_seen`).

| Entry | `control_kind` | `plant` | `rule_ids` | `label` |
|---|---|---|---|---|
| CX-135 | mutation | `BD-074:mutant-gear-single-eta` | INV-SINEW-01, FROB-REF-130, GATE-SINEW-08 | DESIGN |
| CX-136 | mutation | `N15a-14:mutant-thermal-const-r`, `BD-072:mutant-l2-const-r` | ADR-SINEW-04, FROB-REF-132, GATE-SINEW-05 | DESIGN |
| CX-137 | mutation | `N15a-14:mutant-thermal-slope-estimate` | GATE-SINEW-05, §15.7 | DESIGN |
| CX-138 | mutation | `N15a-15:mutant-thermal-init-cold` | ADR-SINEW-08 | DESIGN |
| CX-139 | mutation | `N15a-16:mutant-envelope-no-ellipse` | GATE-SINEW-06, §15.8 | DESIGN |
| CX-140 | mutation | `BD-074:mutant-reflect-diagonal` | §15.9 | DESIGN |
| CX-141 | mutation | `BD-075:mutant-bus-clip-demand`, `N15b-03:mutant-battery-clip-soc` | FROB-REF-133, GATE-SINEW-07, INV-SINEW-04, FROB-REF-135 | DESIGN |
| CX-142 | mutation | `N15b-05:mutant-ledger-double-heat` | INV-SINEW-02, GATE-SINEW-02, §15.11 | DESIGN |
| CX-143 | mutation | `N15b-06:mutant-agreement-raises-class` | §15.12 | DESIGN |
| `coupled-torque-box` | mutation | `N15b-10:mutant-torque-box` | GATE-SINEW-10, FROB-REF-131 | DESIGN |
| `shared-rotor-diagonal` | mutation | `N15b-11:mutant-rotor-diagonal` | GATE-SINEW-10 | DESIGN |
| `power-balance-sign-blind` | mutation | `N15b-12:mutant-ratio-negated` | GATE-SINEW-10, FROB-REF-108 | FACT: `frankensim/crates/fs-phs/src/interconnection.rs:91` at `483d09c682f2` (reported, not re-run); `rerun = false` until reproduced in this repository with a receipt |
| `linkage-constant-ratio` | mutation | `N15b-11:mutant-coupling-frozen` | GATE-SINEW-10 | DESIGN |

A FACT entry's control is written by the seeding tool as an `obligation` (not runnable, makes no gate eligible)
until `rerun = true` with a receipt (N38b-05). Rules that name no gate (CX-138, CX-140, CX-143) are resolved to
gates by the seeding tool through `registry/rule_gates.toml`.

**Numbers for the late entries.** This bead files one request in the allocation ledger
`crates/frob-conformance/registry/allocations.toml` (N38a-40): `[[block]]` with `registry = "cx"`, `count = 4`,
`owner = "N15-CX"`, `section = "§15.15"`; the allocator fills `first` and `last`. When the block is issued,
CX-SINEW-a to -d are mapped to its numbers in table order in one commit; a tag still cited in a registry file,
a test name or a derivation file at the M1 exit fails (`provisional-tag-past-due`).

**Completeness check.** The join of N38b-05 run over this chapter: each of the nine numbered entries and the four
late ones is present with every field filled and has a control per plant after seeding, and each control
resolves to at least one gate; the chapter count is 9, as the catalogue's table states, plus 4 appended.

A control is a defect, not a boundary: "the request is refused at the limit" tests the limit; the control is the
build or input in which the limit was removed.

## Acceptance criteria

- [ ] Nine entries CX-135 to CX-143 and the four late entries exist in `registry/counterexamples.toml` with every
      field of N38b-05's schema filled (the late entries by tag until N38a-40 issues their block, then by
      number, with the working name in `slug`); re-running the seeding tool gives one control per plant
      (fifteen), each with at least one gate after seeding; no row of `controls.toml` was written by hand.
- [ ] Every entry carries the `control_kind`, `plant`, `rule_ids` and `label` of the table above;
      `power-balance-sign-blind` is FACT with its source and `rerun = false` until reproduced.
- [ ] Every numeric value in the tables is reproduced by a host test that computes it.
- [ ] The block request `registry = "cx"`, `count = 4`, `owner = "N15-CX"` is committed with this bead, and no
      tag remains outside `slug` at the M1 exit.
- [ ] `scripts/e2e/sinew_cx_registry.sh` exits 0.

## Test plan

### Unit tests

All run as `FROB_TEST_SEED=0 cargo test -p frob-conformance --test g0_laws -- registry::sinew_cx` (host
arithmetic, exact where rational):
- `cx135_gear`: 1.25 N·m and 12.5 W against 0.8 N·m, 8 W and 2 W lost; 2.5 W created.
- `cx136_thermal`: s = 400·0.20·0.00393 = 0.3144 W/K; rises 100 K and 164.7 K; 385 s to the limit
  (τ = 120/(0.80 − 0.3144) = 247.1 s); 22.8 A and 22.8/√(1 + 0.00393·130) = 18.6 A; I_crit = 31.9 A.
- `cx137_slope`: √(0.80/(0.20·(0.00393 + 0.0022))) = 25.5 A.
- `cx138_cold_start`: 164.7·(1 − e^(−20/247)) = 12.8 K.
- `cx139_envelope`: V_lim = 48/√2 = 33.94 V; base speed 3,151 rad/s; no-load 3,394 rad/s; at 1.5 times base
  speed i_d = −32.2 A, i_q = 23.7 A, 59% of peak torque.
- `cx140_differential`: the reflected inertia equals [[J₁ + J₂, J₁ − J₂], [J₁ − J₂, J₁ + J₂]] for symbolic J₁, J₂
  (exact rationals at J₁ = 2, J₂ = 1).
- `cx141_bus`: E²/(4·R) = 7,200 W; at 1,800 W, I = (48 − √1,728)/0.16 = 40.2 A and V = 44.8 V.
- `late_torque_box`: generator (1, 1) N·m; (1, −1) outside the image; `late_rotor_energy`: [[1, 1], [1, 1]],
  0 J and 2 J against 1 J twice; `late_sign`: rates (12, −8) and (−12, 8), both with 2·12 + 3·(−8) = 0.
- `cx_registry_sinew_complete` (the chapter's slice of the join: thirteen entries, every field, one control per
  plant after seeding); `cx_entries_have_kind_and_plant` (every `mutation` has a non-empty plant naming its
  planting bead); `cx_sinew_block_request`; `cx_late_tags_mapped` (in milestone-exit mode, fails if a tag is
  still cited outside `slug`).

### End-to-end script and logging

`scripts/e2e/sinew_cx_registry.sh` (sources the STD-E2E helper; idempotent; it calls only cargo targets of its
dependencies): runs the unit tests above; runs N38b-05's seeding tool,
`FROB_TEST_SEED=0 cargo test -p frob-conformance --test seed_controls -- --ignored seed`, twice, asserting that the
second run leaves the digest of the committed `controls.toml` unchanged; runs the join,
`cargo test -p frob-conformance --test seed_join`; logs per entry the id or tag, the NCs allocated, kind,
plants, gates and each control's state (`runnable`, `obligation`, or `stale` while its planting bead has not
built the plant); reads `registry/control_history.ndjson` and lists, per control, its last recorded outcome or
`never run`. Running a planted variant against its gate is the control runner's (N38b-04), invoked by the
planting beads' own scripts, and is not repeated here. NDJSON at `target/e2e-logs/sinew_cx_registry/<utc>.ndjson`
with a transcript: `ts`, `script`, `bead` (`N15-CX`), `step`, `event`, the command line, exit code and wall time;
one `metric` line per control (id, seed, gate, state, last outcome); the digest of `controls.toml` before and
after the second seeding; on failure the first missing entry or unresolved gate with the registry path. The
script writes no field of `controls.toml`. Summary: entries, controls, controls runnable, controls stale with
the planting bead named, controls never seen to fail.

### Known answers and negative controls

The numbers in the tables are the known answers KA-320 (CX-135), KA-305 to KA-309 and KA-313 (CX-136), KA-310
(CX-137), KA-312 (CX-138), KA-315 to KA-318 (CX-139), KA-322 (CX-140), KA-324 to KA-326 (CX-141), KA-295
(CX-142), and the one-rotor table of GATE-SINEW-10 (the late entries). Controls on the catalogue itself, each on
a scratch copy under `target/e2e-scratch/sinew_cx/`, never the committed file: an entry deleted (the chapter
count fails); a `mutation` entry with an empty `plant` (`cx_entries_have_kind_and_plant` fails); a late tag left
past the M1 exit (`cx_late_tags_mapped` fails); and, after seeding, a control whose `expect` describes a
boundary instead of a defect, which BD-025's loader rejects with `boundary-not-control`.

## Considerations

- A mutant in production arithmetic is a hazard if it can ship: each plant is a per-control mutation feature,
  never in the `gauntlet-mutations` umbrella and absent from the product binaries (BD-025, N38b-04's
  `cargo tree -e features` check).
- Tolerances marked "to set" in the known-answer registry do not weaken a control: each control asserts a
  difference far above any tolerance (2.5 W created; 1 J against 0 J; a refusal against an admission).
- Owed by the planting beads, not by this registry: every control has `last_seen.caught = true` (written by the
  control runner of N38b-04) from a recorded run before the gate it is attached to records a pass; a control
  never seen to fail is listed for review at the milestone exit.

## References

Plan Appendix N.12; §15.7 to §15.12, §15.15; §0.7 (counterexamples are first-class); §38 (negative-control
registry). ADR-SINEW-04, ADR-SINEW-08; INV-SINEW-01, -02, -04; GATE-SINEW-02, -05, -06, -07, -08, -10.
