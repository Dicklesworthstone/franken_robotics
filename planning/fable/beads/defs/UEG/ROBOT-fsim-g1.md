---
key: "ROBOT-fsim-g1"
title: "assets/robots/fsim-g1: the sibling library's G1 builder (30 links, 31.96314202 kg) as profile `g1-reduced-30`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "suite", "frob-fsim", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["NEG-15", "BD-007", "N14b-34", "N14b-33", "ROBOT-g1"]
needs: ["Appendix F: the upstream work item that folds or declares the omitted masses of the sibling's G1 catalogue, by its identifier"]
related: ["ZOO-humanoid", "BD-024", "BD-032", "N33a-09", "BD-061"]
gates: []
plan_refs: ["Appendix G.1 (last row)", "§14.10", "§14.17", "§33 (foundation-defect admission, row 9)", "GATE-WS-09", "Appendix F.2"]
---
## Background

The sibling physics library (FrankenSim) ships a robot catalogue in its multibody crate, and one entry is a
Unitree G1 with 29 degrees of freedom. The project consumes it through the airlock from M0: the first referee
smoke test steps this model and pins its state digest. It is on file "with provenance, under the suite's
licence". One fact about it must travel with every use. FACT (read in the sibling's source on 2026-11-21, at
the recorded commit; `crates/fs-mbd/src/robot_models.rs`, lines 398 to 403): the builder keeps 30 links and
omits fixed attachments with nonzero mass; its masses sum to 31.96314202 kg, against 33.34014202 kg for the
38-link source. The builder's model is therefore a reduced model. The design language refuses mass dropped at
import unless a `reduced` block declares what was dropped, so this asset is admitted as the declared profile
`g1-reduced-30`, usable as a fast fixture and as the M0 smoke model, and never as the parity reference for the
humanoid mass row. An upstream work item asks the sibling to fold or declare the omitted masses; it is done in
the sibling's repository under its rules, and the interim posture here holds until it lands.

## Scope

In: `assets/robots/fsim-g1/` (manifest with `role = "sibling-catalogue"`, the provenance record, the `reduced`
block, validation golden); the admission-test record of the roll-up. Out: the airlock and the referee smoke
test (BD-007, BD-024); the reduced-model mechanism (N14b-34); the 38-link reference (ROBOT-g1); the upstream
work item itself.

## Technical approach

**Source record.** Not a file import: the model is constructed by the sibling's builder at the pinned sibling
commit recorded in the stack lock. The manifest stores the sibling repository name, the commit, the source
path and line range, the builder's function name, and `licence = the suite's` with its text hash. No `source/`
directory is vendored; the asset is regenerated from the pin by calling the builder through the airlock and
lowering the result to a design term.

**Facts (FACT, re-read at the pin by the admission test).** 29 actuated degrees of freedom; 30 links; total
mass 31.96314202 kg; n_v = 35.

**The `reduced` block (DESIGN).** `reduced = { profile = "g1-reduced-30", reference = "g1", dropped_mass_kg =
"1.37700000", dropped_links = [<names, from ROBOT-g1's comparison record>], reason = "sibling builder omits
fixed attachments with nonzero mass" }`. With the block present the design compiles and every report on it
prints the profile name; without it the import refuses (mass dropped at import).

**Validation.** (1) Roll-up: the sum of the builder's 30 link masses equals 31.96314202 kg exactly (the
admission test's known answer). (2) `dropped_mass_kg` equals the reference's mass on file minus this roll-up.
(3) Inertia consistency per link. (4) Kinematic agreement with the 38-link import after folding: joint axes
and link frames agree; only masses and inertias of the bodies that carried the omitted attachments differ, and
the differing bodies are exactly those named in `dropped_links`. (5) The M0 digest of the referee smoke run on
this model is recorded beside the asset with the sibling commit.

**Use.** The M0 referee smoke test and its digest; a cheap humanoid fixture for kernels and performance rows
(labelled with the profile); the admission record for the foundation-defect row about the catalogue. It is
excluded from the humanoid mass row by rule.

## Acceptance criteria

- [ ] The manifest records sibling commit, path, lines and builder; the roll-up equals 31.96314202 kg at the
      pin; the `reduced` block is complete.
- [ ] Offered as the reference for the `humanoid@g1` mass row, the asset is refused with the profile named.
- [ ] Kinematics agree with the folded 38-link import and the mass differences are confined to the listed
      bodies.
- [ ] When the upstream item lands, the switch-over is a recorded event: new pin, new roll-up, the `reduced`
      block removed or shrunk, goldens regenerated.

## Test plan

### Unit tests

- `robot_fsim_g1_rollup`: 30 links, 31.96314202 kg.
- `robot_fsim_g1_reduced_block_required`: without the block the lowering refuses.
- `robot_fsim_g1_dropped_mass_matches_reference`: 1.37700000 kg.
- `robot_fsim_g1_kinematics_match_folded_reference`.
- `robot_fsim_g1_not_a_parity_reference`: the mass-gate loader rejects a `reduced` profile.
- `robot_fsim_g1_provenance_complete`: commit, path, lines, builder present and consistent with the stack lock.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): builds the model through the airlock, lowers it, runs validation;
`frob sim run --tier referee` on it reproduces the recorded digest; logs sibling commit, roll-up to eight
decimals, the profile name on every evidence block, and the digest.

### Known answers and negative controls

The admission known answer: the roll-up of 31.96314202 kg, recorded at the pin with its result. Controls: the
reduced model used as the parity reference (refused); the `reduced` block's dropped mass altered by one gram
(validation fails). Determinism: the digest is pinned per sibling commit and build identity.

## Considerations

- A sibling's newer clean head is never rolled back locally; adopting a change goes through the switch-over
  rule.
- The profile label must reach every surface: a throughput figure measured on the reduced humanoid says so.

## References

Appendix G.1; §14.10; §14.17; §33 (admission of the G1 catalogue); GATE-WS-09; Appendix F.2.
