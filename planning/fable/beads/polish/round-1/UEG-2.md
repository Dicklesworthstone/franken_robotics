# Polishing round 1, slice UEG-2

55 beads (UEG-2).

| Key | Milestone | Type | Parent | Title | File |
|---|---|---|---|---|---|
| NEG-15 | M1 | task | WP-08 | assets/robots: layout, manifest `frob.robot.asset/1`, licence check and validation for reference robots | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-15.md` |
| NEG-16 | M1 | task | WP-08 | e2e: `scripts/e2e/assets_licences.sh`: reference robots and scene-asset sources validated and licence-checked | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-16.md` |
| NEG-17 | M3 | task | WP-23 | scene asset source `frob-scene`: the project's own generators as default source; a pure starter pack | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-17.md` |
| NEG-18 | M5 | task | WP-32 | scene asset source `infinigen-indoors` (BSD-3): offline external generator, never in the production graph | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-18.md` |
| NEG-19 | M5 | task | WP-32 | scene asset source `google-scanned-objects` (CC-BY 4.0): household clutter, attribution in every export | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-19.md` |
| NEG-20 | M5 | task | WP-32 | scene asset source `robocasa` (code MIT, assets CC BY 4.0): kitchen fixtures and layouts, assets only | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-20.md` |
| NEG-21 | M3 | task | WP-23 | scene asset source `procthor` (code Apache-2.0; dataset licence not verified): generator reference only | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-21.md` |
| NEG-22 | M3 | task | WP-23 | scene asset source `hssd` (CC BY-NC 4.0): user-supplied import only, never redistributed | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-22.md` |
| NEG-23 | M3 | task | WP-23 | scene asset source `hm3d` (academic, non-commercial): user-supplied import of scanned buildings only | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-23.md` |
| NEG-24 | M3 | task | WP-23 | scene asset source `partnet-mobility` (non-commercial research): user-supplied articulated objects only | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-24.md` |
| NEG-25 | M3 | task | WP-23 | scene asset source `behavior-1k` (its own agreement): not used; supported only as a user-supplied import | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-25.md` |
| NEG-26 | M3 | task | WP-23 | scene asset source `objaverse-xl` (per-object licences): not used by default; one licence entry per object | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-26.md` |
| NEG-27 | M3 | task | WP-23 | prior-art register for interior generation (retail, warehouse; not read) and the no-novelty claim check | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-27.md` |
| NEG-28 | M3 | task | WP-23 | registry: the consolidated hazard list: 59 hazards in seven archetypes, every dimension `assumed` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-28.md` |
| NEG-29 | M3 | task | WP-23 | frob-conformance: every hazard is covered by a requirement, a shield or a refusal: the cover map and its check | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-29.md` |
| NEG-30 | M1 | task | WP-38 | zoo: the `zoo/` directory of pinned archetype instances, `frob.zoo.expect/1` and its conformance test | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-30.md` |
| NEG-31 | M1 | task | WP-38 | e2e: `scripts/e2e/zoo_goldens.sh`: every zoo archetype instantiated, preflighted and compared with its golden | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-31.md` |
| NEG-32 | M1 | task | WP-08 | frob-io: sibling-catalogue robots as designs: a catalog builder lowered through the airlock to a design term | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-32.md` |
| ROBOT-a1 | M1 | task | WP-08 | assets/robots/a1: Unitree A1 from MuJoCo Menagerie (BSD-3): parity fixture and baseline for `quad-mammal` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-a1.md` |
| ROBOT-anymal-b | M1 | task | WP-08 | assets/robots/anymal-b: ANYmal B from MuJoCo Menagerie (BSD-3): mid-size baseline for `quad-mammal` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-anymal-b.md` |
| ROBOT-anymal-c | M1 | task | WP-08 | assets/robots/anymal-c: ANYmal C from MuJoCo Menagerie (BSD-3): heaviest baseline for `quad-mammal` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-anymal-c.md` |
| ROBOT-apollo | M1 | task | WP-08 | assets/robots/apollo: Apptronik Apollo from MuJoCo Menagerie (Apache-2.0): import parity only; size stress | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-apollo.md` |
| ROBOT-barkour | M1 | task | WP-08 | assets/robots/barkour: Barkour from MuJoCo Menagerie (Apache-2.0): `quad-mammal`, import parity only | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-barkour.md` |
| ROBOT-booster-t1 | M1 | task | WP-08 | assets/robots/booster-t1: Booster T1 from MuJoCo Menagerie (Apache-2.0): `humanoid`, import parity only | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-booster-t1.md` |
| ROBOT-cassie | M3 | task | WP-38 | assets/robots/cassie: Cassie from MuJoCo Menagerie (MIT): closed-loop legs, reference for `biped-point` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-cassie.md` |
| ROBOT-fourier-n1 | M1 | task | WP-08 | assets/robots/fourier-n1: Fourier N1 from MuJoCo Menagerie (Apache-2.0): import parity only; conventions | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-fourier-n1.md` |
| ROBOT-fsim-g1 | M1 | task | WP-08 | assets/robots/fsim-g1: the sibling library's G1 builder (30 links, 31.96314202 kg) as profile `g1-reduced-30` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-fsim-g1.md` |
| ROBOT-g1 | M1 | task | WP-08 | assets/robots/g1: Unitree G1 from MuJoCo Menagerie (BSD-3): the 38-link parity reference, 33.34014202 kg | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-g1.md` |
| ROBOT-go1 | M1 | task | WP-08 | assets/robots/go1: Unitree Go1 from MuJoCo Menagerie (BSD-3): parity fixture and baseline for `quad-mammal` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-go1.md` |
| ROBOT-go2 | M1 | task | WP-08 | assets/robots/go2: Unitree Go2 from MuJoCo Menagerie (BSD-3): the default baseline for `quad-mammal` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-go2.md` |
| ROBOT-h1 | M1 | task | WP-08 | assets/robots/h1: Unitree H1 from MuJoCo Menagerie (BSD-3): full-size parity fixture and baseline for `humanoid` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-h1.md` |
| ROBOT-iiwa7 | M3 | task | WP-38 | assets/robots/iiwa7: the sibling library's KUKA iiwa 7 builder: the arm of `mobile-manipulator` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-iiwa7.md` |
| ROBOT-op3 | M1 | task | WP-08 | assets/robots/op3: OP3 from MuJoCo Menagerie (Apache-2.0): import parity only; the small-scale fixture | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-op3.md` |
| ROBOT-spot | M1 | task | WP-08 | assets/robots/spot: Spot from MuJoCo Menagerie (BSD-3): parity fixture and baseline for `quad-mammal`, legs only | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-spot.md` |
| ROBOT-stretch3 | M1 | task | WP-08 | assets/robots/stretch3: Stretch 3 from MuJoCo Menagerie (Apache-2.0): `diff-drive` base, `mobile-manipulator` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-stretch3.md` |
| ROBOT-talos | M1 | task | WP-08 | assets/robots/talos: TALOS from MuJoCo Menagerie (Apache-2.0): import parity only; heavy, many joints | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-talos.md` |
| ROBOT-tiago | M1 | task | WP-08 | assets/robots/tiago: TIAGo from MuJoCo Menagerie (Apache-2.0): omni base for `mecanum`, `mobile-manipulator` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ROBOT-tiago.md` |
| ZOO-ackermann | M3 | task | WP-38 | zoo/ackermann.fbot: archetype 8, car-like base with a steering linkage: C₂, growth vector (2, 3, 4), L/tan δ | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-ackermann.md` |
| ZOO-balancer-2w | M3 | task | WP-38 | zoo/balancer-2w.fbot: archetype 18, two-wheeled inverted pendulum: C₂, the open-loop pole in closed form | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-balancer-2w.md` |
| ZOO-biped-point | M3 | task | WP-38 | zoo/biped-point.fbot: archetype 5, point-foot biped: C₂, no static stability reported and not refused | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-biped-point.md` |
| ZOO-diff-drive | M1 | task | WP-38 | zoo/diff-drive.fbot: archetype 7, two driven wheels and a caster: growth vector (2, 3), chord step, threshold | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-diff-drive.md` |
| ZOO-hexapod-radial | M3 | task | WP-38 | zoo/hexapod-radial.fbot: archetype 3, radial hexapod: D₆ (C₆ handed), 86,185 orbits, tripod gait as (H, K) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-hexapod-radial.md` |
| ZOO-humanoid | M1 | task | WP-38 | zoo/humanoid.fbot: archetype 6, humanoid with arms and waist: 29 actuated, n_v = 35, C₂, coupled transmissions | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-humanoid.md` |
| ZOO-mecanum | M1 | task | WP-38 | zoo/mecanum.fbot: archetype 9, four mecanum wheels: C₂ × C₂, growth vector (3), parallel rollers are rank 2 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-mecanum.md` |
| ZOO-mobile-manipulator | M3 | task | WP-38 | zoo/mobile-manipulator.fbot: archetype 16, seven-joint arm on a wheeled base: trivial symmetry, workspace | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-mobile-manipulator.md` |
| ZOO-octopod | M3 | task | WP-38 | zoo/octopod.fbot: archetype 4, eight-legged body: D₈ radial or C₂ spider, contact capacity, the 36 bucket | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-octopod.md` |
| ZOO-omni-3 | M3 | task | WP-38 | zoo/omni-3.fbot: archetype 10, three omni wheels at 120°: D₃, growth vector (3), the 3 × 3 wheel map | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-omni-3.md` |
| ZOO-quad-mammal | M1 | task | WP-38 | zoo/quad-mammal.fbot: archetype 1, dog-like quadruped: C₂ × C₂, 27 contact strata, Γ_obs = C₂ with a lidar | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-quad-mammal.md` |
| ZOO-quad-sprawl | M3 | task | WP-38 | zoo/quad-sprawl.fbot: archetype 2, sprawled quadruped: stable on a wide stance, refused thermally at `@hot` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-quad-sprawl.md` |
| ZOO-rocker-bogie | M3 | task | WP-38 | zoo/rocker-bogie.fbot: archetype 15, six wheels on a passive rocker-bogie: C₂, differential, mobility 3 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-rocker-bogie.md` |
| ZOO-snake-crawler | M3 | task | WP-38 | zoo/snake-crawler.fbot: archetype 17, planar chain with passive wheels: C₂ by reversal, locomotion as holonomy | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-snake-crawler.md` |
| ZOO-swerve-4 | M3 | task | WP-38 | zoo/swerve-4.fbot: archetype 11, four steer-and-drive modules: D₄, nested symmetry, scrub torque, singular pose | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-swerve-4.md` |
| ZOO-tracked-flipper | M3 | task | WP-38 | zoo/tracked-flipper.fbot: archetype 12, two tracks with four flippers: C₂ × C₂, belt law, scrub work in a pivot | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-tracked-flipper.md` |
| ZOO-tri-wheel-stair | M3 | task | WP-38 | zoo/tri-wheel-stair.fbot: archetype 14, two tri-wheel clusters: C₂ outside, C₃ or D₃ inside, riser and tread | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-tri-wheel-stair.md` |
| ZOO-wheel-leg | M3 | task | WP-38 | zoo/wheel-leg.fbot: archetype 13, quadruped with driven wheels as feet: C₂ × C₂, two contact laws per foot | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-wheel-leg.md` |

## Addressed to your beads

- `NEG-21`: left by the reconciliation pass (dup-10): N19b-02: DESIGN recorded in N19b-02 and N37-18 (the plan words FROB-REF-310 'a pack or report would export …' in §19.16 and defines FROB-REF-733 and -731 in §37.12 without dividing them): 310 is raised only by ScenePack::export_check when the exported artifact is itself a scene pack (exit 3); a report or any other derived artifact is refused at the export door with 733 (licence) or 731 (private), exit 5. N19b-02 no longer has the report verbs call the pack check. Beads outside this packet that still expect 310 for stills or reports (NEG-21 to NEG-26, N41-35) need the same correction
- `NEG-21`: request from round 1, slice U19b: Per the reconciliation DESIGN in N19b-02/N37-18, FROB-REF-310 is raised only by ScenePack::export_check for a scene pack; 'any export containing it is refused with FROB-REF-310' must say: a scene pack or variant -> 310 (exit 3); a report, still, replay or other derived artifact -> FROB-REF-733 at the export door (exit 5).
- `NEG-22`: request from round 1, slice U19b: Same 310/733 division: line 67 expects 310 for a non-pack export; derived artifacts are refused at the export door with FROB-REF-733 (licence) or -731 (private), exit 5.
- `NEG-23`: request from round 1, slice U19b: Same 310/733/731 division: 'every export path is refused with FROB-REF-310' is wrong for stills, reports and replays (733 or 731, exit 5); only the pack and its variants give 310.
- `NEG-24`: request from round 1, slice U19b: Same division: 'stills and reports containing them refused with FROB-REF-310' should be FROB-REF-733 at the export door (exit 5); 310 only for the pack and variants.
- `NEG-25`: request from round 1, slice U19b: Same division: 'every export of geometry, stills or packs is refused with FROB-REF-310' -> packs 310 (exit 3); geometry and stills 733 (exit 5).
- `NEG-26`: left by the reconciliation pass (dup-10): N19b-02: DESIGN recorded in N19b-02 and N37-18 (the plan words FROB-REF-310 'a pack or report would export …' in §19.16 and defines FROB-REF-733 and -731 in §37.12 without dividing them): 310 is raised only by ScenePack::export_check when the exported artifact is itself a scene pack (exit 3); a report or any other derived artifact is refused at the export door with 733 (licence) or 731 (private), exit 5. N19b-02 no longer has the report verbs call the pack check. Beads outside this packet that still expect 310 for stills or reports (NEG-21 to NEG-26, N41-35) need the same correction
- `NEG-26`: request from round 1, slice U19b: Check the 310 expectation is limited to scene-pack exports (N19b-02 DESIGN); derived artifacts get FROB-REF-733.
- `ZOO-hexapod-radial`: request from round 1, slice U24c: add dep N24c-16 (OPEN-ARENA-2): its reference-robot row is `none` and the mass check is `not applicable`
- `ZOO-snake-crawler`: request from round 1, slice U14b: Scope Out names the template as N14b-04; it is now N14bp1-01 (split from N14b-04): update the reference and add dep N14bp1-01

## Graph findings about your beads

- `NEG-17`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-18`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-19`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-20`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-21`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-23`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-24`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-25`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-26`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
- `NEG-27`: a registry bead that nothing depends on: the mechanism and test beads that use its rows should depend on it
