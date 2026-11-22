---
key: "NEG-29"
title: "frob-conformance: every hazard is covered by a requirement, a shield or a refusal: the cover map and its check"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "arena", "safety", "frob-conformance", "solid", "test", "e2e", "registry"]
parent: "WP-23"
deps: ["NEG-28", "BD-227", "BD-121", "BD-253", "BD-244", "N19a-38", "STD-TEST", "STD-E2E"]
needs: ["§24: the registry of requirement templates per task family (the names a task can state and a monitor can decide), as the target of the R codes", "§22.12: the registry of shield kinds available at M3 and M4 (stopping-distance, geofence, energy-budget), as the target of the S codes"]
related: ["BD-338", "BD-337", "BD-270", "BD-278", "BD-254", "N14b-12", "N14b-13", "GB-33"]
gates: []
plan_refs: ["Appendix G.3", "§19.8", "§24", "§22.11", "§22.12", "§14.11", "§28"]
---
## Background

A hazard list is only useful if each hazard can make something fail visibly. The registries say what feature
of a place provokes which robot failure; nothing so far guarantees that the failure would be noticed. A
hazard whose failure no requirement measures, no shield guards against and no preflight check refuses is
decoration: a scene can contain it, a robot can fail on it, and every scorecard stays green. This bead closes
that gap with one rule and one check. The rule: each of the 59 hazards is covered by at least one of three
things, namely a requirement template that a task can state and a monitor can decide, a runtime shield, or a
refusal code raised at preflight or admission. The check: a registry maps hazards to their covers, and a
conformance test fails when a hazard has none, when a cover does not exist, or when a cover exists and cannot
actually be triggered by the hazard's own fixture. It is the same principle the project applies to gates (a
gate that nothing can fail validates nothing), applied to hazards.

## Scope

In: the cover vocabulary; the cover map `registries/hazards/coverage.toml`; the three-part check; the
per-hazard trigger fixtures; the report of uncovered hazards per battery. Out: the hazard rows themselves
(NEG-28 and the scene unit's registries); the requirement language and task families (BD-121, BD-253); shields
(BD-338, BD-337); preflight stages 7 and 8 (BD-244).

## Technical approach

**Cover vocabulary (DESIGN).**

| Code | Kind | Meaning | Decided by |
|---|---|---|---|
| R:pass | requirement | A named passage (door, lift, lane) is completed within a deadline | task verdict |
| R:progress | requirement | No stall longer than a stated time while a motion command is active | task verdict |
| R:pitch | requirement | Pitch and roll excursion stay below a bound (includes ride-up and tip) | task verdict |
| R:clear | requirement | Signed clearance of body, underslung load, toe and raised load stays above zero | task verdict |
| R:fall | requirement | No fall (body height and attitude inside a band) | task verdict |
| R:slip | requirement | Load-weighted slip distance stays below a bound | task verdict |
| R:drop | requirement | Vertical drop of a wheel or castor stays below a bound | task verdict |
| R:odom | requirement | Yaw and odometry error against truth stay below a bound | task verdict |
| R:sep | requirement | Minimum separation from every moving actor stays above a bound | task verdict |
| R:nocoll | requirement | No collision with fixed geometry; displacement of pushed movable objects below a bound | task verdict |
| R:energy | requirement | Energy per metre and winding temperature stay below bounds | task verdict |
| R:zone | requirement | The footprint is never inside a keep-out region | task verdict |
| R:place | requirement | Placement error on a moving target below a bound | task verdict |
| R:hold | requirement | A carried container is not spilled and a grasp is not lost (acceleration and slosh bounds) | task verdict |
| R:sense | requirement | Perception stays usable: obstacle detected before the stopping distance; no stop on a phantom; dropout handled | task verdict |
| R:loc | requirement | The pose estimate stays consistent with truth (no jump to an aliased place) | task verdict |
| S:stop | shield | Stopping-distance shield with a backup controller | shield report |
| S:zone | shield | Geofence shield on keep-out regions | shield report |
| S:energy | shield | Energy-budget stop for a guided contact (a door or leaf returning onto the robot) | shield report |
| X:119 | refusal | No admissible route or reach in the scene (preflight stage 7) | preflight |
| X:120 | refusal | Observation alias under every admitted probe (preflight stage 8) | preflight |
| X:131 | refusal | A required operating point outside the torque, speed and voltage envelope | preflight |
| X:132 | refusal | No stable thermal equilibrium at a required operating point, or one above a limit | preflight |
| X:242 | refusal | Film law out of domain under strict data | admission |

FROB-REF-312 (a realism claim on a scene with `assumed` dimensions) applies to every hazard and is not a
cover: it guards a claim about the scene, not a failure of the robot.

**Cover map.** `registries/hazards/coverage.toml`: one entry per hazard (archetype, name) with a non-empty
list of codes, and for each code the fixture that triggers it. Plan-time content: the "Covered by" column of
the consolidated hazard table. Summary of that content: 59 of 59 hazards have at least one cover, 109 (hazard, code)
pairs in all; 53 hazards have a requirement; 8 have a shield (rows 9, 11, 30, 32, 37, 48, 54 and 56: five
stopping shields, one geofence, two energy stops); 24 have a refusal (X:119 twelve times, X:131 seven, X:120
three, X:242 twice, X:132 once); six are covered by a refusal alone (hallway, right-angle corridor, galley,
warehouse aisle, end cap, checkout lane: pure passability hazards, where the right behaviour is to be refused
before simulating).

**The check, in three parts.**

1. *Non-empty.* Every hazard of the index has an entry with at least one code; every entry names a hazard
   that exists.
2. *Covers exist.* Each R code resolves to a requirement template registered for at least one task family
   that can run on the hazard's archetype; each S code resolves to a shield kind in the shield registry; each
   X code resolves to a refusal in the refusal registry. Before the target's milestone the resolution is
   `pending(Mn)` and the hazard counts as covered only by its already-landed covers; a hazard with all covers
   pending is reported `uncovered until Mn`, never as covered.
3. *Covers can fire.* For each (hazard, code) pair, a trigger fixture: a minimal scene drawn from the
   hazard's row at the severe end of its range, a robot chosen to fail on it, and the expectation that the
   cover fires (the requirement's verdict is `violated`, the shield intervenes, or the refusal is raised).
   And a paired control at the benign end of the range, or with the hazard removed, on which it does not
   fire. Fixtures are `synthetic` and live in `crates/frob-conformance/fixtures/hazard-covers/`.

**Battery report.** For a battery (a set of scenes and tasks), `uncovered_hazards(battery)` lists every hazard
present in its scenes for which no task of the battery states a covering requirement and no shield or
refusal applies; the scorecard prints the count. The incidence export of the scene unit supplies the
hazard-by-scene matrix.

## Acceptance criteria

- [ ] The cover map has 59 entries; part 1 of the check passes and fails on a planted empty entry and on a
      planted entry for a hazard that does not exist.
- [ ] Part 2 resolves every code or reports it `pending(Mn)`; a planted unknown code fails.
- [ ] Part 3 has one trigger fixture and one control per (hazard, code) pair whose target has landed; each
      trigger fires and each control does not.
- [ ] The `standard` battery's uncovered-hazard count is printed on its scorecard; a planted battery with a
      wet-floor scene and no slip requirement reports that hazard uncovered.
- [ ] A hazard added to a registry without a cover fails the build.

## Test plan

### Unit tests

- `hazard_cover_map_has_59_entries`, `hazard_cover_nonempty`, `hazard_cover_names_exist`.
- `hazard_cover_codes_resolve`: R, S and X targets; `hazard_cover_pending_is_not_covered`.
- `hazard_cover_trigger_fires` and `hazard_cover_control_silent`: parameterized over the (hazard, code) pairs;
  examples: threshold 25 mm with a 40 mm wheel → R:progress violated, 5 mm → satisfied; checkout lane 0.6 m
  with a 0.7 m footprint → X:119 with the lane named, 0.9 m → admitted; greasy floor 100 mPa·s under a
  0.3 s stance → R:slip violated, dry patch → satisfied; repeated corridor with one beacon → X:120 with the
  two worlds, with a second beacon → admitted; forklift actor crossing at 2 m → S:stop intervenes, no actor →
  silent.
- `uncovered_hazards_report`: the planted battery.
- `new_hazard_without_cover_fails`.

### End-to-end script and logging

`scripts/e2e/hazard_covers.sh`: for each archetype, `frob scene gen` a trigger scene per hazard from the
registry (severe end) and its control, then `frob design check --task … --scene … --stages 7-8` for X covers,
`frob eval run` with the covering requirement for R covers, and `frob shield check` for S covers; finally
`frob eval scorecard` on the `standard` battery to read the uncovered-hazard count. Sources
`scripts/e2e/lib.sh`; logs per (hazard, code): the scene's identity, the drawn dimension with its `assumed`
provenance, the command, the verdict, refusal or intervention, the evidence block, and whether trigger and
control behaved as expected; the summary gives hazards covered, pending and uncovered, pairs passed and
failed. No private-scene data: all scenes are generated.

### Known answers and negative controls

The invariant "59 of 59 covered" and the six refusal-only hazards. Negative controls: the paired control
scenes (a cover that fires on the benign scene is as wrong as one that never fires); a planted requirement
template that always returns `satisfied` (its trigger fixture fails); a planted hazard with no cover.
Metamorphic: moving a hazard's dimension from the benign to the severe end flips the cover's outcome exactly
once along the way for monotone hazards (thresholds, widths, viscosities). Determinism: fixtures are seeded;
verdicts identical twice on one machine.

## Considerations

- Coverage here means "a failure would be noticed", not "the robot is safe": a covered hazard can still
  defeat a design, which is the purpose.
- The cover map is data and will change as task families and shields land; the check keeps the invariant
  while it changes.
- Three hazards have only a Frontier representation; until it lands their trigger fixtures are `not run` and
  they are reported `not represented`, which the summary shows separately from `uncovered`.

## References

Appendix G.3; §19.8; §24; §22.11; §22.12; §14.11; §28.
