---
key: "N14-CX"
title: "Counterexample registry of Skeleton (§14): CX-125 to CX-134 and three late cases, seeding negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025"]
needs: ["§38.11: the negative-control registry `frob-conformance/registry/controls.toml` and its record schema"]
related: ["N14-REF", "N14b-18", "N14b-25", "N14b-26", "BD-146"]
gates: []
plan_refs: ["Appendix N.11", "§14.5", "§14.6", "§14.10", "§14.11", "§14.14", "§14.16", "§14.17", "§38.11", "§0.7"]
---
## Background

The plan treats counterexamples as first-class: where a plausible shortcut is wrong, it gives the smallest
hand-checkable case that breaks it, and the case enters the counterexample catalogue and the negative-control
registry. A rule that ships without such a case has not been thought through. The registry's rule is that each
catalogue entry becomes at least one control: a planted defect, attached to the gates of the rule it protects,
that the gate must catch every time it runs. A gate with no control cannot record a pass, and a control that has
never failed its gate in any recorded run is itself suspect.

This bead is the Skeleton chapter's part of that catalogue: ten numbered counterexamples and three that the
chapter's late subsections state without a number. For each it records the shortcut, the case with its numbers,
the rule it protects, and the control that plants the shortcut, with the gate and the bead that carries it.

## Scope

In: the thirteen entries as registry records; the mapping of each to a control, a gate and a carrying bead; a
test that each control exists and has been seen to fail. Out: implementing the controls (each is planted by the
bead named in its row).

## Technical approach

**Entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control (planted shortcut), gate, carried by |
|---|---|---|---|---|
| CX-125 | A body with no stated fill can default to solid | Torso box 0.42 × 0.20 × 0.10 m in an alloy at 2,700 kg/m³: solid it weighs 22.7 kg; as a 2.5 mm shell it weighs 1.92 kg. The solid reading is 11.8 times too heavy, and through the mass spiral (loop gain 0.59, 2.4 kg of robot per kilogram of structure) a 20.8 kg error becomes about 50 kg of robot, with every actuator sized for it | No default fill; every body's mass comes from a Region, a fill and a material, or is declared; FROB-REF-105; the solid-fill advisory above one quarter of `mass_max` | A parser that defaults a missing fill to `solid`; GATE-SKEL-04, GATE-SKEL-07; the design-schema and geometry beads, and the defect corpus (code 105) |
| CX-126 | Rescaling the density to hit the right mass repairs the inertia | The same box about its vertical axis: radius of gyration squared (0.42² + 0.20²)/12 = 0.0180 m² as a solid and 0.0236 m² as a shell, 31% larger, because a shell keeps its mass at the walls | The fill rule (shell moments as the difference of two solids); the shell-moment proof obligation; GATE-SKEL-07 | A shell computed as a solid at rescaled density; GATE-SKEL-07; the geometry bead and the invariant battery |
| CX-127 | A declared inertia that passes pseudo-inertia positivity is physically possible for its part | A 1 kg part with principal moments 0.008 kg·m² each, declared inside a sphere of radius 0.10 m: the mass-weighted mean squared distance tr(I_C)/(2m) is 0.012 m², above R² = 0.010 m². No body inside the sphere has that inertia; a thin shell of that radius meets the bound with equality | The mass-support bound ‖c − o‖² + tr(I_C)/(2m) ≤ R² and tr(P·Q) ≥ 0 at stage 2; FROB-REF-106 | A stage 2 with the mass-support test removed; GATE-SKEL-04; the declared-bodies bead and the defect corpus (code 106). The same fixture appears in the algebra chapter's catalogue |
| CX-128 | A contract that carries peak demands can exclude a design (add the peaks and compare with the limit) | Two fingers share a bus limited to one unit of current, each with peak demand one unit. Adding peaks gives two units and prunes the design; one finger draws at the first tick and the other at the second, so the schedule is feasible | A `bus_demand` without a time-resolved profile is usable for admission only; the bus analysis runs in full before any electrical exclusion; GATE-SKEL-06 | A sum-of-peaks exclusion rule; GATE-SKEL-06 and GATE-SKEL-09; the substitution bead and the composed screen |
| CX-129 | A limb's port inertia is positive definite, so it can be inverted at the port and can stand for a limb in contact | Eliminating a joint with axis s gives I − I·s·(sᵀI·s)⁻¹·sᵀI, which sends s to zero; with I the identity and s the first basis vector the packet is diag(0, 1, 1, 1, 1, 1). A chain of massless parents is singular, and contact rows couple every limb through the base | The positive-semidefinite type of the limb port packet; its proof obligation; the three consumer rules; stage 2 refuses massless chains; KILL-SKEL-PORT | A packet typed positive definite, or inverted at the port; the packet's proof obligation at M5; the module bead and the tree-operator pilot |
| CX-130 | Swapping one leaf module leaves the robot's verdicts and rollouts valid | A leaf-local substitution reuses siblings' subtrees, kinematics kernels and local analyses, and still invalidates static stability, the baseline controller, every rollout, every verdict and every whole-robot certificate, because ground contact couples every limb through the base; an edit to the shared bus or sink is global | The four conditions of leaf-local substitution; the invalidation invariants; GATE-SKEL-06 (the mirror swap reuses, the shared-bus edit invalidates, a reversed port sign refuses) | A substitution that keeps a verdict or a rollout; a shared-bus edit classed leaf-local; GATE-SKEL-06; the substitution bead |
| CX-131 | A connection-curvature (bracket) test decides whether a wheel layout can produce the required motions | A mecanum base with all four roller axes parallel: the twist-to-wheel map drops from rank 3 to rank 2, the base slides freely along one direction, and the growth vector is (2, 3) where an omnidirectional base must have (3). The curvature test passes, as it passes a differential drive, because the missing direction is still reached by a bracket | Stage 4 runs the growth vector on wheeled and tracked bases and curvature on shape-changers; FROB-REF-118 | Stage 4 with the two branches swapped; GATE-SKEL-04; the stage-4 bead and the defect corpus (code 118) |
| CX-132 | A full-rank local information matrix means the sensors can tell the admitted worlds apart | Range beacons at (−1, 0) and (1, 0). A robot at (0, 1) and a robot at (0, −1) both read √2 from each, and the local information matrix is the identity at both points; a task that requires moving toward the positive side needs opposite actions in the two worlds. A third beacon at (0, 2) reads 1 and 3 and separates them when the range error is below 1 | The stage-8 alias search with its two labels; FROB-REF-120; the bound (1 + TV)/2; KILL-SKEL-ALIAS | A stage 8 that passes on a full-rank local test; GATE-SKEL-04 (stages 7 and 8); the stage-8 bead and the planted cases |
| CX-133 | A symmetry the designer declares can be used by consumers | `quad-mammal` with a forward lidar and `assert_obs: C2xC2`: the computed Γ_obs is C₂, and the design is refused (FROB-REF-109) with the lidar mount as breaker | A declared symmetry never replaces a computed one, and no consumer reads the declaration | A consumer that reads the assertion; a compiler that accepts a false one; GATE-SKEL-04, GATE-MARROW-03; the symmetry bead, the invariant battery and the defect corpus (code 109) |
| CX-134 | A wheel edit can be judged by one of its two effects at a threshold | At a 0.012 m lip, raising the wheel radius from 0.060 m to 0.075 m lengthens the lever arm √(2·r·h − h²) from 36.0 mm to 40.7 mm while the horizontal push over axle load falls from 0.750 to 0.646 | The raised-threshold oracle of the `diff-drive` archetype | A fixture that checks only the lever arm; GATE-SKEL-03; the first-archetypes bead |

**Late cases (stated in the chapter's late subsections; not numbered in the catalogue; to receive numbers when
the catalogue is next regenerated).**

| Provisional id | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control, gate, carried by |
|---|---|---|---|---|
| CX-14.16-a | A capability-screen result holds for any stance | Four one-joint legs that can each press 40 N under a centred 16 kg body (156.96 N) are admitted with four feet down, at 39.24 N per foot. In a trot two diagonal feet carry 78.48 N each against 40 N. A screen that reuses the four-foot result for a trotting requirement admits a robot that cannot trot | The stance set is part of the query and of the result | A screen that ignores the stance set; GATE-SKEL-09; the composed-screen beads |
| CX-14.17-a | Two inertias about their own centres can be added when a welded link is merged into its parent | A parent of 2 kg with inertia 0.02·I kg·m² about its origin and a welded child of 1 kg with 0.01·I about its own centre at (0.3, 0, 0) m. The fold has inertia diag(0.03, 0.12, 0.12) kg·m² about the parent origin; adding without the offset term gives 0.03·I, wrong by 0.09 kg·m² on two axes | The fold by the pseudo-inertia sum 𝖯_p + G·𝖯_c·Gᵀ | The no-offset mutant; GATE-SKEL-11; the fold bead |
| CX-14.17-b | A model that omits small welded parts is the same robot | A 30-link humanoid model of 31.96314202 kg against its 38-link source of 33.34014202 kg: 313.5584 N against 327.0668 N at 9.81 m/s². A static support rated at 320 N passes on the reduction and fails on the robot. Dropping the child in the hand case gives 2 kg where 3 kg is right | A model that omits mass is a different robot and is named as one; an undeclared drop of mass at import is refused | An importer that drops massive links silently; a result that omits the reduction; GATE-SKEL-11; the reduced-model bead |

**Registry records.** Each entry yields one or more records of the negative-control registry:

```toml
[[control]]
id        = "NC-…"                    # stable, never reused
seed      = "CX-131"                  # the catalogue entry it came from
gate      = ["GATE-SKEL-04"]          # every gate that must catch it
kind      = "mutation"                # mutation | planted-input | planted-design | planted-world | a-a
target    = "frob-analyze::preflight::stage4"
defect    = "curvature test used for a wheeled base"
expect    = "the parallel-roller mecanum passes stage 4; the gate fails"
last_seen = { caught = true, session = "…", date = "…" }
```

A control is a defect, not a boundary: "the request is refused at the limit" tests the limit; the control is the
build or input in which the limit was removed.

## Acceptance criteria

- [ ] The thirteen entries exist as records with `seed`, `gate`, `kind`, `target`, `defect` and `expect`, each
      attached to at least one gate.
- [ ] Every numeric value in the table is reproduced by a test that computes it (the masses, the gyration radii,
      0.012 against 0.010 m², the rank drop, the beacon readings, the lever arms and push ratios, the stance
      forces, the fold inertias, the two weights).
- [ ] Every control has `last_seen.caught = true` from a recorded run before the gate it is attached to records a
      pass; a control never seen to fail is listed for review at the milestone exit.
- [ ] The three late cases are flagged for numbering in the catalogue, and their provisional identifiers resolve
      to their records.
- [ ] The duplicate between CX-127 and the algebra chapter's entry is recorded as one fixture with two seeds.

## Test plan

### Unit tests

- `cx125_masses`: 0.0084 m³ × 2,700 = 22.68 kg; shell 0.000712125 m³ × 2,700 = 1.923 kg; ratio 11.8.
- `cx126_gyration`: 0.01803 m² and 0.02357 m²; ratio 1.307.
- `cx127_mass_support`: 3 × 0.008/(2 × 1) = 0.012 m² > 0.010 m²; a thin shell of radius 0.10 m and mass 1 kg has
  principal moments 2/3 × 0.01 = 1/150 kg·m² and meets the bound with equality.
- `cx128_two_ticks_feasible`: zero one-tick schedules and exactly two two-tick schedules.
- `cx129_packet`: identity inertia, s = e₁ → diag(0, 1, 1, 1, 1, 1).
- `cx131_rank`: all roller tangents equal → rank 2; null twist (−t, 1, 0).
- `cx132_beacons`: √2 and √2 in both worlds; 1 and 3 from (0, 2).
- `cx134_threshold`: 36.0 mm and 40.7 mm; 0.750 and 0.646.
- `late_stance`: 39.24 N and 78.48 N; `late_fold`: diag(0.03, 0.12, 0.12) against 0.03·I; `late_reduced`:
  313.5584 N and 327.0668 N against 320 N.
- `every_entry_has_control_and_gate`; `never_failed_controls_listed`.

### End-to-end script and logging

`scripts/e2e/skel_controls.sh`: for each control whose target is reachable from the outside, builds or selects
the planted variant, runs the owning gate's script against it and asserts that the script fails; then restores
the normal build and asserts that it passes. NDJSON per control: its identifier and seed, the gate, the command,
the exit code with and without the planted defect, the measured value that tripped, and the date written to
`last_seen`. Summary: controls run, controls caught, controls never seen to fail.

### Known answers and negative controls

- The known answers that share these fixtures: KA-261 (the torso card), KA-264 (the port packet), KA-265 (the
  raised threshold), KA-268 (parallel rollers), KA-271 (the two-finger bus), KA-272 and KA-273 (the mirror swap
  and the shared-bus edit), KA-274 (the beacons), KA-276 (the refused symmetry assertion).
- A control on the registry itself: a record whose `gate` list is empty, or whose `expect` describes a boundary
  instead of a defect, must be rejected by the registry's validator.

## Considerations

- The catalogue's printed form has 384 entries; this chapter's are CX-125 to CX-134. The three late cases come
  from subsections added after the catalogue was compiled, as do their gates; they bind all the same.
- One convention disagreement touches CX-125: the conventions chapter says that a body without a wall thickness
  is solid, subject to a mass-plausibility refusal, while this chapter says there is no default fill and a
  missing fill is refused. The stricter rule of this chapter governs designs; the control planted here is a
  parser that applies a solid default.
- Controls cost time on every gate run. They are kept small (one mutation each) and are never skipped: a gate
  run in which a control was not caught is a failure of the gate, whatever the passing test says.

## References

Appendix N.11 (CX-125 to CX-134), §14.5, §14.6, §14.10, §14.11, §14.14, §14.16, §14.17, §38.11 (the
negative-control registry and its five rules), §0.7.
