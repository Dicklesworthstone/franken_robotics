---
key: "NEGp1-53"
title: "zoo M4 rows: tracked-flipper's simulated pivot scrub on the belt law; wheel-leg's braked hold and free roll on the hybrid laws"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "skeleton", "traction", "frob-design", "frontier", "test", "e2e", "zoo"]
parent: "WP-38"
deps: ["BD-365", "N14b-07", "N14b-09", "N14bp2-02", "N18b-06", "NEG-31", "STD-E2E", "ZOO-tracked-flipper", "ZOO-wheel-leg"]
needs: []
related: ["BD-245", "BD-247", "N14b-06", "NEG-30", "NS-08"]
gates: []
plan_refs: ["Appendix G.4 rows 12 and 13", "§14.10", "§18.9", "§14.13 (rejection fixtures, track and foot)", "Appendix M KA-282, KA-418"]
---
## Background

Two archetypes of the catalogue of eighteen have contact laws that close on every tier only at M4: the
`tracked-flipper` (law `belt`, Frontier: a contact surface with a prescribed tangential velocity, an anisotropic
friction ellipse and distributed patches) and the `wheel-leg` (two laws per end effector, `wheel-rolling` and
`pad`, with a switching rule on the brake state). Both compile, pass preflight and have their static and
closed-form known answers at M3 in their zoo beads (ZOO-tracked-flipper, ZOO-wheel-leg). Their simulated rows
need the M4 laws (belt tracks in N18b-06, the tyre, track and hybrid laws on all tiers in BD-365), so they
cannot be acceptance criteria of M3 beads: an M3 bead that waits on an M4 mechanism either never closes or
closes with the rows unchecked.

This bead carries those rows at M4. The two zoo beads write them in their expectation files with
`blocked = "milestone"` and `due = "M4"` (NEG-30's field for a known answer whose mechanism lands later); this
bead evaluates them, removes the `blocked` field by a dated amendment, and adds the M4 step of the zoo script
for the two archetypes. It was split from the two zoo beads in polishing round 1; every row and test those
beads listed for M4 is here, none is dropped.

What the rows protect. A tracked base cannot yaw under exact lateral no-slip, so every turn in place
dissipates scrub work; a model in which it turns for free is wrong in a way that undersizes track motors and
hides the thermal duty of a long climb (KA-418, KA-282). A wheel-leg robot whose simulator binds one law per
end effector, or applies rolling contact to a braked wheel, rolls down a slope it should hold.

## Scope

In:
- The M4 rows of `zoo/tracked-flipper` and `zoo/wheel-leg` expectation files: evaluated, with the `blocked`
  field removed by a dated amendment line in `zoo/INDEX.toml` (no `DesignId` changes).
- Three `synthetic` task documents this bead registers: `fixtures/tasks/pivot-90.ftask` (a 90° turn in place on
  a flat frictional floor), `fixtures/tasks/ramp-5pct-hold.ftask` and `fixtures/tasks/ramp-5pct-roll.ftask` (a 5%
  grade, brakes engaged or released). Each is marked `synthetic` and is never an Arena task; a run names it by
  path, `--task fixtures/tasks/<name>.ftask` (there is no `fixture:` spelling of `--task`).
- The tests below and the M4 step of `scripts/e2e/zoo_goldens.sh` for the two archetypes.

Out, with the bead that has it: the `belt` law and its scrub (N18b-06, BD-365; the M3 form in BD-245 is the
named fallback); the hybrid laws and the switching event (BD-365); the two-law contact field, its typing, the brake command
channel and the two bindings per wheel (N14bp2-02); the skid-steer baseline (N14b-07) and the
hybrid mode-graph controller (N14b-09); the static and closed-form rows and the controls evaluated at M3 (the
two zoo beads); tracks on rubble (NS-08); the standing rows of the catalogue gate (BD-247 and the catalogue
script of N14b-06).

## Technical approach

**Tracked pivot.** The expected value is computed, not copied: the scrub moment of a uniformly loaded track pair
is M = μ_lat·W·ℓ/4 and a pivot through φ dissipates E = M·φ, with W the compiled weight on the tracks, ℓ the
compiled contact length of the main tracks and μ_lat the lateral friction of the track's interface card. At the
template's defaults as ZOO-tracked-flipper repeats them (W = 300 N, ℓ = 0.50 m, μ = 0.8) this is 30 N·m and
47.1239 J for φ = π/2. If the compiled values differ from those defaults, the test fails and names both, since
the zoo's fixture must change by a dated amendment and never silently.

Run: `frob sim run <DesignId> --tier batch-cpu --profile strict --task fixtures/tasks/pivot-90.ftask --seed 1
--key zoo-tf-pivot-strict-<run> --json` (the `DesignId` of the zoo instance that the zoo script's step 2 created
as `zoo-tracked-flipper`; `sim.run` is a `run` verb and refuses a call without `--key` with FROB-REF-805) under
the skid-steer baseline (opposite track speeds until the yaw reaches 90°, then stop), then `frob telemetry energy
<run> --json`, which reports dissipated energy by channel. The `strict` profile is the reference value, as it is
the adjudicator of BD-365: the referee cannot run the tracked base, whose contact set exceeds the referee's
21-contact cap in normal driving, and refuses it by name with FROB-REF-721, exit 5, no run stored (BD-365,
N18b-06, N18b-20); that refusal is asserted once (`--tier referee`, key `zoo-tf-pivot-ref-<run>`), never recorded
as a value. The scrub channel (lateral friction dissipation at the track patches) must be strictly positive and
within 10% of the closed form. The same task runs on every tier on which BD-365 registers `belt` (key
`zoo-tf-pivot-<tier>-<run>`), and each tier's value is recorded against the `strict` value under that tier's
class tolerance. The zero-scrub variant (`variants/zero-scrub.fbot`, tracks as
rows of ideal rolling wheels) runs the same pivot: its scrub work is below 1% of the closed form, and the row
for that variant must report the change and name KA-282's rejection fixture.

**Wheel-leg slope fixtures.** Grade 5%: θ = atan 0.05 = 2.862°; a braked wheel holds when the `pad` law's
friction exceeds tan θ = 0.05, which the default card does with a wide margin. (a) Braked hold: brakes engaged,
the mode-graph controller in its stand mode; over 5 s the base moves less than 1 mm along the slope (DESIGN
threshold) and the active law at all four contacts is `pad` at every step. (b) Free roll: brakes released, legs
held in the stand pose, no drive torque; after 2 s the displacement is at least half of the frictionless
rolling closed form s = ½·a·t² with a = g·sin θ·m/(m + Σ I_w/r_w²), computed from the compiled mass, wheel
inertias and radii (the factor one half allows for rolling resistance and leg compliance, DESIGN). (c) Control
(b) of ZOO-wheel-leg: a build that binds `wheel-rolling` unconditionally rolls under fixture (a), and the
braked-hold row catches it. (d) Switch instant: releasing the brake mid-run produces exactly one law switch per
contact, at the step boundary on lockstep tiers and at the event time on the referee, recorded with the tier's
profile. The brakes are N14bp2-02's product: each wheel carries two bindings, role `primary` with law
`wheel-rolling` and role `braked` with law `pad`, and one contact switch on the command channel `brake:<joint id>`
of its own axle joint (engaged or released; reset value released), which the task document sets per fixture.
Each fixture runs as `frob sim run <DesignId of zoo-wheel-leg> --tier <tier> --task
fixtures/tasks/ramp-5pct-hold.ftask` (or `ramp-5pct-roll.ftask`) `--seed 1 --key zoo-wl-<fixture>-<tier>-<run>
--json`, read back with `frob telemetry query <run> --stream contact --json` for the role and law in force per
contact per step and the `law-switch` events (BD-365), and with the run's base-pose telemetry for the
displacement.

**Amendment.** When every row passes, the two expectation files lose `blocked = "milestone"` for these rows and
`zoo/INDEX.toml` gains one dated line per archetype naming this bead; a row that fails stays blocked and the
bead does not close.

## Acceptance criteria

- [ ] On `batch-cpu` under the `strict` profile, the simulated 90° pivot of `tracked-flipper` dissipates scrub
      work that is positive and within 10% of the closed form computed from compiled values (47.1239 J at the
      defaults); every tier that registers `belt` has its value recorded against the `strict` value under its
      class tolerance; the same pivot on the referee is refused with FROB-REF-721 (exit 5), no run stored.
- [ ] The zero-scrub variant's simulated pivot dissipates under 1% of the closed form and the row reports it as
      KA-282's rejection fixture.
- [ ] `wheel-leg` on the 5% grade: braked, under 1 mm in 5 s with `pad` active on all four contacts; released,
      at least half the closed-form rolling distance in 2 s; the unconditional-rolling build is caught; one switch
      event per contact at the documented instant.
- [ ] The two expectation files carry no `blocked = "milestone"` row due at M4, with dated amendment lines, and
      `scripts/e2e/zoo_goldens.sh --milestone M4` passes for both archetypes.

## Test plan

### Unit tests

- `zoo_tracked_pivot_closed_form_from_compiled`: μ_lat·W·ℓ/4·π/2 from compiled weight, contact length and card
  friction; 47.1239 J at the defaults, rel 10⁻⁹; a planted 10% change of ℓ without an amendment fails.
- `zoo_tracked_pivot_scrub_positive`: `batch-cpu --profile strict`, 90° pivot; scrub channel > 0 and within 10%
  of the closed form.
- `zoo_tracked_pivot_per_tier`: each tier registering `belt` within its class tolerance of the `strict` value.
- `zoo_tracked_pivot_referee_refused_721`: the pivot with `--tier referee` exits 5 with FROB-REF-721 naming the
  contact cap, and no run is stored.
- `zoo_tracked_zero_scrub_caught_in_sim`: under 1% of the closed form, reported as KA-282's fixture.
- `zoo_wheel_leg_braked_holds_on_slope`: under 1 mm in 5 s; role `braked` and law `pad` in force at every step
  on all contacts, read from the contact stream.
- `zoo_wheel_leg_free_wheel_rolls`: at least half of ½·a·t² at t = 2 s.
- `zoo_wheel_leg_unconditional_rolling_caught`: control (b) moves more than the hold threshold and the test
  asserts the catch.
- `zoo_wheel_leg_switch_instant_recorded`: one switch per contact at the step boundary (lockstep tier) and at
  the event time (referee).
- `zoo_m4_rows_unblocked`: in an M4 build the loader finds no row of these two archetypes with
  `blocked = "milestone"` and `due` at or before M4.

### End-to-end script and logging

The M4 step of `scripts/e2e/zoo_goldens.sh` (NEG-31), run as `--milestone M4` and also as
`--only tracked-flipper,wheel-leg` for development (a gate row is never recorded from an `--only` run). It
sources the shared helper of STD-E2E and writes `target/e2e-logs/zoo_goldens/<utc-timestamp>.ndjson`. Every line
has `ts`, `script`, `bead: "NEGp1-53"`, `archetype`, `step`, `event` (`start`, `cmd`, `assert`, `metric`,
`caught`, `end`), the exact command line, exit code, wall time and the envelope's `evidence` block (claim,
class, tier, determinism, identities, seed). Metric lines carry: the scrub work in joules against its closed
form and window, the energy by channel, the per-tier values; the slope displacement in millimetres against the
threshold, the active law per contact as counts per law over the run, the switch events with their instants;
and for each control the expected and observed value. On failure a line gives the first differing value and the
paths of the run bundle and the telemetry file. The final `summary` line gives rows passed, failed and still
blocked, and controls caught.

### Known answers and negative controls

KA-418 (a skid-steer base under exact lateral no-slip has zero yaw rate, so a turn in place costs scrub work);
KA-282 (rejection fixture, track: turning in place at zero scrub work on a frictional floor is caught, here in
simulation). Controls: the zero-scrub variant; the unconditional-rolling build; a planted contact-length change
without an amendment. Determinism: seeds named; the `strict` reference and the wheel-leg referee runs within
their class; per-tier values within each tier's class.

## Considerations

- The uniform-pressure closed form is an idealization; real pressure concentrates under road wheels, which is
  what the 10% window allows for. If N18b-06's Frontier belt misses its pilot and BD-365 runs on BD-245's M3
  form (its named fallback), the rows run on that form and every log line names which form produced the number.
- The hold threshold (1 mm in 5 s) and the rolling factor (one half) are DESIGN values; changing either is a
  dated amendment of this bead's fixtures.
- Risk: the tracked and hybrid standing rows of the catalogue gate share these laws; a failure here is reported
  to BD-247 and N14b-06 rather than worked around in the fixtures.

## References

Appendix G.4 rows 12 and 13; §14.10; §14.13; §18.9; KA-282, KA-418; ZOO-tracked-flipper; ZOO-wheel-leg;
BD-365; N18b-06.
