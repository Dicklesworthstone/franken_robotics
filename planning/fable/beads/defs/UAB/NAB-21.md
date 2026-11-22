---
key: "NAB-21"
title: "registries/ev_matrix.toml: the Moonshot queue: 21 waiting items, order of entry and the cap of four"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record", "test"]
parent: "EPIC-M0"
deps: ["NAB-17", "NAB-19"]
needs: ["§0.7, §38.5: the pilot ledger, which counts running Moonshot pilots and refuses a fifth (N00-07); this bead supplies the order in which a freed slot is filled", "§43: the milestone register, which governs where the queue's default order and a milestone's content differ (N40-06)"]
related: ["NAB-22", "NAB-29", "NAB-31", "NAB-50", "N00-07", "N40-06", "BD-416", "BD-418", "BD-419", "BD-423", "BD-425", "BD-426", "BD-435", "BD-436", "BD-437", "BD-438", "BD-439", "BD-440", "BD-441"]
gates: []
plan_refs: ["Appendix C.1.4", "§9 (D-6, D-10)", "§43"]
---
## Background

A Moonshot, in FrankenRobotics, is a synthesis this project proposes itself: a fused per-world GPU kernel,
coupled fluid and cloth islands beside one world, a rotor port for flight, tactile fingertip pads, firmware
export, numerical continuation for linkage synthesis. Each could make the system unlike anything else; none is
needed for the system to work. A doctrine rule (D-6) therefore **caps the Moonshot budget: at most four [M]
items are active at once**, where an item is *active from the funding of its pilot until its expiry gate*. The
catalogue of admitted Moonshots is longer than four, so the rest wait in a queue.

Three standing facts about every item in the queue: nothing in it is on the critical path of the milestone
plan; every item sits behind a feature flag; and none is funded before its paper-scale pilot has run and its
ceiling has been stated (doctrine rule D-10). A pilot is small enough to run on a CPU in about a day, is
registered before it runs, and has an outcome that is not known in advance; a pilot that cannot fail is not a
pilot.

This bead carries the queue as data, `[[moonshot_queue]]` records in `registries/ev_matrix.toml` (schema
`frob.evmatrix/1`, NAB-17), and the function that says which item enters a freed slot. The cap itself is
enforced by the pilot ledger (a fifth running Moonshot is refused there); the queue supplies the order.
Tier: the items are Moonshot and each is backlog whatever its milestone; the queue itself is Solid bookkeeping
that the matrix lints and the milestone sweeps read, so it lands with them at M0.

## Scope

In: 21 queue records, the order rule, the slot-freeing rule, the next-entry function, the four notes, the
cross-checks with the Frontier and Moonshot table (NAB-19). Out: the pilots and the mechanisms (their own
beads and epics); the cap check (N00-07); the scores (NAB-19).

## Technical approach

**1. Record.**

```toml
[[moonshot_queue]]
id        = "MQ-12"
order     = 12                          # the default order of entry
item      = "Lifecycle compression"
where     = ["§31.14"]
pilot     = "A 100-hour reference of 1.8 × 10⁸ steps compressed at least 50-fold with at most 5% error on every slow state"
pilot_gate = "GATE-FOUNDRY-12"
kill_gate = "KILL-LIFECYCLE"            # "" when the fallback has no kill gate
fallback  = "aged variants at hand-specified wear states"
milestone = "M6"                        # earliest milestone at which the item may be funded
milestone_note = "work lands with M6; the gate is recorded at M8"   # the queue's wording, kept verbatim
register_milestone = "M6"               # where the gate register places the item's gate and kill gate
waits_for = []                          # other queue ids that must finish first
frontier_part_first = false             # true when the [M] part is funded only after the [F] part passed its gate
family    = ["EV-F54"]                  # rows of the matrix this item belongs to; [] when it has none
state     = "queued"                    # queued | active | passed | killed | falsified | deferred
```

**2. The order of entry into a free slot** (a DESIGN default; the milestone plan governs where it differs):

1. An item whose consumers land at an earlier milestone goes first.
2. Within a milestone the item with the cheaper pilot goes first, because a cheap pilot that fails frees its
   slot soonest.
3. An item that depends on another waits for it.

**3. When a slot is freed.** When the item passes its gate; when its kill gate fires and the fallback is
recorded; or when the falsifier of its pilot holds.

**4. The next-entry function.** `next_entries(queue, now_milestone, active_count) -> Vec<Id>`: among items with
`state = "queued"`, keep those whose `milestone` has been reached, whose `waits_for` items all have state
`passed`, and (where `frontier_part_first`) whose Frontier part has a recorded pass; sort by `order`; return the
first `4 − active_count`. The function is pure, D0, and is called by the pilot ledger's funding check and by
the milestone-exit sweeps.

**5. Four notes that qualify the table.**

1. Pilot sizes and costs in the table are TARGETs and estimates taken from the sections that own the items.
2. Kernel reflection is tiered [F] and is **not** counted against the cap; if its kill gate (KILL-REFLECT) fires
   it moves to the parked table of the matrix.
3. Payload slosh by an equivalent mechanical model, robot-advected films and articulated cord and rug proxies
   are [F] and are likewise outside the cap; resolved free-surface fluid, cloth and rods exist only inside
   item 2.
4. For items 9, 10 and 11 the [M] part is funded only after the [F] part of the same section has passed its own
   gate (`frontier_part_first = true`).

**6. Dependencies between items** (`waits_for`): item 8 (the continuum arm: a rod as a generalized joint) waits
for item 2 and is not built without it; item 15 (Study-quadric mechanism synthesis) waits for item 14 and is
piloted with it. Item 21 (general Lean theorems on Lie groups, theorem card T-13) has no pilot possible while
the proof library's modules are blocked; its state is `deferred` and it is not scheduled.

**The queue.**

| Id (entry order) | Item | Where | Pilot before funding | Kill gate or fallback | Milestone |
|---|---|---|---|---|---|
| MQ-01 | Fused per-world step kernel | §21.18 | Fuse kinematics, dynamics terms and one Newton iteration for the reference quadruped; compare with the staged kernels at 8,192 worlds with digests unchanged; one week | KILL-MEGA; the staged step plan | After the M2 profile |
| MQ-02 | Coupled multiphysics islands: free-surface fluid, implicit cloth, Cosserat rod | §19.10.6, §19.10.7 | Proxy adequacy on an incumbent engine: three scenes, 200 episodes each, proxies against full models (GATE-HAB-09) | KILL-LIVING; the Solid tier and the Frontier proxies | After M6; the pilot needs no new solver and may run from M3 |
| MQ-03 | Rotor port for flight, with the aerial-terrestrial hybrid variant | §14.10, §15.4 | Hover by hand: thrust equals weight, ideal induced power T^(3/2)/√(2·ρ·A), and the energy ledger closes | KILL-SINEW-ROTOR; port and variant withdrawn | M6 |
| MQ-04 | Tactile hands: micro-patch contact pads | §14.10, §18.17 | A fingertip pad as 2 × 2 and 4 × 4 four-dimensional contacts on the `strict` profile, against one patch | One patch per pad | M6 |
| MQ-05 | Persistent ruts and footprints | §18.10, §18.17 | The leak fixture on `batch-cpu`: a rut made in one episode has no effect on another | Off; terrain resets each episode | M6 |
| MQ-06 | Link-chain tracks | §18.9, §18.17 | A chain of 40 links on the referee; falsifier, a contact count beyond every admitted capacity | The belt model | M6 |
| MQ-07 | Hydraulic actuation (actuator level L4) | §15.4, §15.14 | A single cylinder through an orifice: steady speed against the closed form, with the port balance closed | Electric levels L1 to L3 only | M6 at the earliest |
| MQ-08 | Continuum arm: a rod as a generalized joint | §14.10 | Waits for the rod of item 2 | Not built without item 2 | After item 2 |
| MQ-09 | Shape-space atlas for locomotion by geometric phase | §25.19 | Differential-drive and crawler fixtures (GATE-AUGUR-20); funded only after one proposed gait is admitted by a full referee rollout | KILL-AUGUR-PHASE; A-09 stays a forward diagnostic | M6 |
| MQ-10 | Symmetry at the instant of contact, amortized per family | §25.20 | The mirrored landing and its four asymmetric mutations (GATE-AUGUR-21) | KILL-AUGUR-SYMEVENT; per-event finite differences | M6 |
| MQ-11 | Label minimization from contact for a shared controller | §25.18 | A three-variant gripper fixture: inner and outer command sets derived from contact | KILL-AUGUR-LABELS; supplied sets through a conventional solver | After M4 |
| MQ-12 | Lifecycle compression | §31.14 | A 100-hour reference of 1.8 × 10⁸ steps compressed at least 50-fold with at most 5% error on every slow state (GATE-FOUNDRY-12) | KILL-LIFECYCLE; aged variants at hand-specified wear states | Work lands with M6; the gate is recorded at M8 |
| MQ-13 | Firmware export of fixed-point controllers, with the reserved fixed-point kernel types and theorem card T-14 | §21.7, §31.13 | The fixed-point cases of §31.18, 10⁶ vectors bit-equal (GATE-FOUNDRY-11) | KILL-FIRMWARE; parameters, Q formats and the test-vector manifest only | M8 |
| MQ-14 | Numerical continuation for linkage synthesis, on the CPU | C-34; §25.21.4 | Stewart–Gough forward kinematics: 40 certified roots on ten generic parameter sets (GATE-AUGUR-22) | KILL-SYNTH-CONT; the exact lane with multistart recovery, labeled `Partial` | M8 |
| MQ-15 | Study-quadric mechanism synthesis | C-11 | Piloted with item 14 | Numerical inverse kinematics with multistart, labeled incomplete | After item 14 |
| MQ-16 | Skill transport through a physical interface | §25.11 | The tray interface, three treatments, pre-registered (GATE-AUGUR-23) | KILL-AUGUR-TRANSPORT; family policies and inheritance of §23 | M8 |
| MQ-17 | Skill transport by a learned residual | §22.10 | 24 target bodies of one class, three treatments at equal compute, about 40 GPU-hours (GATE-REFLEX-09) | KILL-SKILL; the exact layer and its witnesses | M8 |
| MQ-18 | Reachable-set tables for small subsystems | §22.12.5 | Braking across two floor materials: two states and a mode (GATE-SHIELD-05) | KILL-TUBE; reduced-order barriers | M8 |
| MQ-19 | Geometric-algebra policy on the kinematic tree | C-24; §23.14 | Three arms with three seeds each, about 90 GPU-hours in an owner-approved envelope (GATE-CORTEX-17) | KILL-GATREE; the plain morphology transformer | M8 |
| MQ-20 | Facility co-design and multi-robot handoff | §25.17 | The acceptance fixture: two strategies discriminated and four seeded failures recovered (GATE-AUGUR-24) | KILL-AUGUR-FACILITY; single-robot missions with priced world edits | M8 |
| MQ-21 | General Lean theorems on Lie groups (T-13) | §12.3 | None possible while the modules are blocked (§12.1) | Stays deferred | Not scheduled |

## Acceptance criteria

- [ ] 21 `[[moonshot_queue]]` records `MQ-01` … `MQ-21` exist with the pilot, kill gate or fallback, and
      milestone of the table; `order` is 1 to 21 without gaps.
- [ ] `waits_for` is set for MQ-08 (on MQ-02) and MQ-15 (on MQ-14); `frontier_part_first` for MQ-09, MQ-10 and
      MQ-11; MQ-21 is `deferred`.
- [ ] Every kill gate named exists in the gate register; the seven items with no kill gate (MQ-04 tactile
      hands, MQ-05 persistent ruts, MQ-06 link-chain tracks, MQ-07 hydraulic actuation, MQ-08 the continuum
      arm, MQ-15 Study-quadric synthesis, MQ-21 general Lean theorems) each have a non-empty `fallback`.
- [ ] `next_entries` returns the known answers of the test plan.
- [ ] Every row of the Frontier and Moonshot table whose decision says "queued" (or "admitted") points to a
      queue record, and the lint `ev.queue-mismatch` passes.
- [ ] With four Moonshot pilots `running` in the pilot ledger, `next_entries` returns nothing and a fifth
      funding request is refused there.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ev_matrix.rs`, module `moonshot_queue`:

- `twenty_one_items_in_order`.
- `next_entries_at_m2_profile`: at `now = M2` (after the profile), nothing active → `[MQ-01]` only (the other
  items' milestones are not reached).
- `next_entries_at_m6`: at `now = M6`, MQ-01 passed, nothing active → `[MQ-02, MQ-03, MQ-04, MQ-05]`.
- `next_entries_respects_cap`: same, with two active → the first two of that list.
- `continuum_arm_waits_for_islands`: at M6 with MQ-02 `queued`, MQ-08 is never returned; with MQ-02 `killed`,
  MQ-08 is still not returned (it is not built without item 2) and its state becomes `killed` in the sweep.
- `frontier_part_first`: MQ-09 is returned only when the geometric-phase gate on kinematic fixtures
  (GATE-AUGUR-20) has a recorded pass.
- `lean_theorems_never_returned`.
- `slot_freed_on_three_events`: passing, a fired kill gate with its fallback recorded, a falsified pilot.
- `lifecycle_reference_arithmetic`: 100 hours at a 2 ms step is 100 × 3,600 ÷ 0.002 = 1.8 × 10⁸ steps; at least
  50-fold compression means at most 3.6 × 10⁶ macro-steps.
- `hover_pilot_formula_is_stored`: the rotor item's pilot text contains "T^(3/2)/√(2·ρ·A)".

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) prints the queue with states, runs `next_entries` for each milestone
M0 to M8 on the plan-time state and logs the result per milestone as an `assert` with the expected list.
Logged per STD-E2E with the digest of the registry.

### Known answers and negative controls

Known answers: 21 items; by earliest milestone: one after the M2 profile (MQ-01), one after M6 whose pilot may
run from M3 (MQ-02), seven at M6 (MQ-03 to MQ-07, MQ-09, MQ-10), one after item 2 (MQ-08), one after M4
(MQ-11), one landing with M6 (MQ-12; the queue's wording adds that its gate is recorded at M8, and the gate
register places gate and kill gate at M6), seven at M8 (MQ-13, MQ-14, MQ-16 to MQ-20),
one after item 14 (MQ-15), one not scheduled (MQ-21). Pilot numbers to keep exact: 8,192 worlds and one week
(MQ-01); three scenes of 200 episodes each (MQ-02); 2 × 2 and 4 × 4 contacts (MQ-04); a chain of 40 links
(MQ-06); 40 certified roots on ten generic parameter sets (MQ-14); 24 target bodies, three treatments, about 40
GPU-hours (MQ-17); three arms with three seeds each, about 90 GPU-hours (MQ-19); 10⁶ bit-equal vectors (MQ-13).
Negative controls: a fifth active Moonshot; MQ-08 funded while MQ-02 is queued; a queue item with neither kill
gate nor fallback; an item moved ahead of a cheaper pilot at the same milestone without an ADR.

## Considerations

- The queue is not a roadmap. An item's presence means only that it may be piloted when a slot is free.
- The GPU-hour figures of two pilots exceed the default machine envelope (16 GiB, 8 cores, one hour, one
  device); those pilots also need the owner's written approval of a compute class before they start.
- If the M2 profile makes item 1 pointless (the step is not bound by what fusion removes), the item is killed
  from the queue without a pilot and its slot is never taken; that is a legitimate use of the falsifier.
- One discrepancy in the planning record concerned item 12: one chapter recorded its gate and kill gate at M8
  while the gate register places both at M6. The register governs: the work lands with M6.

## References

Appendix C.1.4; doctrine rules D-6 and D-10; GOAL-43; §43.
