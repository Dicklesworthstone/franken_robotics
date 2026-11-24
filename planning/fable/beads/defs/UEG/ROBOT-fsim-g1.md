---
key: "ROBOT-fsim-g1"
title: "assets/robots/fsim-g1: the sibling library's G1 builder (30 links, 31.96314202 kg) as profile `g1-reduced-30`"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "suite", "frob-fsim", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["BD-007", "N14b-33", "N14b-34", "N33c-41", "NEG-15", "NEG-32", "ROBOT-g1"]
needs: []
related: ["ROBOT-iiwa7", "ZOO-humanoid"]
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

In: the asset directory `assets/robots/fsim-g1/`: the manifest with `role = "sibling-catalogue"`, the
provenance record, the lowered design `import.fbot` carrying the `reduced` block that the importer wrote from
the profile, the validation golden, and the recorded M0 digest; the asset-level validation, which re-checks
the roll-up and the omitted mass against the reference on file.

Out, with the bead that has each:
- the reduced-model mechanism: the schema of the `reduced` block, the refusal of mass dropped at import, the
  `--reduced` option, `evidence.reduced` on envelopes, and the profile file
  `assets/profiles/g1-reduced-30.toml` (retained link names, omitted links with their masses, provenance):
  N14b-34. This asset names that profile and defines neither the profile nor the block;
- the admission probe of the builder through the airlock at M0, its omission list as read at the pin and the
  record of the roll-up for GATE-WS-09: N33c-41. This asset cites that record and repeats the roll-up only
  as its own validation step;
- the airlock and the referee smoke test (BD-007, BD-024); the lowering of a catalogue builder's model to a
  design term, which is the `fsim-catalog` format of `design.import` (NEG-32); the 38-link reference
  (ROBOT-g1); the upstream work item itself (NF-07).

## Technical approach

**Source record.** Not a file import: the model is constructed by the sibling's builder at the pinned sibling
commit recorded in the stack lock. The manifest stores the sibling repository name, the commit, the source
path and line range, the builder's function name, and `licence = the suite's` with its text hash. No `source/`
directory is vendored; the asset is regenerated from the pin by calling the builder through the airlock and
lowering the result to a design term: `frob design import --format fsim-catalog --builder g1 --name fsim-g1
--reduced g1-reduced-30 --key asset-fsim-g1-<sibling commit> --json` (NEG-32; NEG-15's import recipe), whose
design's `term` section is `import.fbot` (the verb writes no file).

**Facts (FACT, re-read at the pin by the admission test).** 29 actuated degrees of freedom; 30 links; total
mass 31.96314202 kg; n_v = 35.

**The `reduced` block.** The block is N14b-34's and is written into `import.fbot` by the importer when it is
given `--reduced g1-reduced-30`; this asset authors no block of its own. In that bead's schema it reads

```json
"reduced": { "source": "b3:…", "profile": "g1-reduced-30",
             "omitted": [ {"id": "head_link", "mass": "… kg"}, … ],
             "omitted_mass": "1.377 kg" }
```

with `source` the hash of the full 38-link source document (the source of ROBOT-g1), `omitted` the links the
profile lists with their masses (the names come from ROBOT-g1's comparison record), and `omitted_mass` their
exact sum. An earlier draft of this asset showed a block with other field names (`reference`,
`dropped_mass_kg`, `dropped_links`, `reason`); that form is withdrawn, because the plan's record is
`reduced: { omitted, omitted_mass }` and one schema is defined, in N14b-34. The reason for the reduction
("sibling builder omits fixed attachments with nonzero mass") is kept where provenance belongs: in the
profile file's provenance statement and in this asset's manifest field `reduction_reason`. With the block
present the design compiles and every report on it prints the profile name; without it the import refuses
(mass dropped at import).

**Validation.** (1) Roll-up: the sum of the builder's 30 link masses equals 31.96314202 kg exactly (the known
answer of the admission probe N33c-41, whose record at the pin this asset cites by receipt hash).
(2) `omitted_mass` equals the reference's mass on file minus this roll-up (33.34014202 − 31.96314202 =
1.377 kg; at M0 the probe could only record this figure as reported, and here it is computed from ROBOT-g1's
asset on file). (3) Inertia consistency per link. (4) Kinematic agreement with the 38-link import after
folding: joint axes and link frames agree; only masses and inertias of the bodies that carried the omitted
attachments differ. The differing bodies are exactly the retained parents of the links named in `omitted`
(the omitted links themselves exist only in the 38-link import, where the fold merged them into those
parents), and each parent's mass difference equals the summed mass of its omitted children. (5) The M0 digest of the referee smoke run on
this model is recorded beside the asset with the sibling commit.

**Use.** The M0 referee smoke test and its digest; a cheap humanoid fixture for kernels and performance rows
(labelled with the profile); the admission record for the foundation-defect row about the catalogue. It is
excluded from the humanoid mass row by rule.

## Acceptance criteria

- [ ] The manifest records sibling commit, path, lines and builder; the roll-up equals 31.96314202 kg at the
      pin and agrees with N33c-41's recorded probe; the `reduced` block in `import.fbot` validates against
      N14b-34's schema and equals what the profile `assets/profiles/g1-reduced-30.toml` prescribes.
- [ ] Offered as the reference for the `humanoid@g1` mass row, the asset is refused with the profile named.
- [ ] Kinematics agree with the folded 38-link import and the mass differences are confined to the retained
      parents of the omitted links, each equal to the summed mass of its omitted children.
- [ ] When the upstream item lands, the switch-over is a recorded event: new pin, new roll-up, the `reduced`
      block removed or shrunk, goldens regenerated.

## Test plan

### Unit tests

- `robot_fsim_g1_rollup`: 30 links, 31.96314202 kg.
- `robot_fsim_g1_reduced_block_required`: without the block the lowering refuses.
- `robot_fsim_g1_omitted_mass_matches_reference`: 1.377 kg, as the exact difference of the two masses on file.
- `robot_fsim_g1_block_is_the_profiles`: the block in `import.fbot` has exactly the fields of N14b-34's schema
  and the omitted list of the profile file.
- `robot_fsim_g1_kinematics_match_folded_reference`.
- `robot_fsim_g1_not_a_parity_reference`: the mass-gate loader rejects a `reduced` profile.
- `robot_fsim_g1_provenance_complete`: commit, path, lines, builder present and consistent with the stack lock.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): builds the model through the airlock, lowers it, runs validation;
`frob --json sim run g1 --tier referee --seconds 2 --key asset-fsim-g1-smoke-<sibling commit>` (BD-024's M0 smoke
call on the catalogue model, with a key of its own) reproduces the recorded digest; logs sibling commit, roll-up to eight
decimals, the profile name on every evidence block, and the digest.

### Known answers and negative controls

The admission known answer, held by N33c-41 and re-checked here: the roll-up of 31.96314202 kg, recorded at
the pin with its result. Controls: the reduced model used as the parity reference (refused); the `reduced`
block's `omitted_mass` altered by one gram (validation fails). Determinism: the digest is pinned per sibling commit and build identity.

## Considerations

- A sibling's newer clean head is never rolled back locally; adopting a change goes through the switch-over
  rule.
- The profile label must reach every surface: a throughput figure measured on the reduced humanoid says so.

## References

Appendix G.1; §14.10; §14.17; §33 (admission of the G1 catalogue); GATE-WS-09; Appendix F.2.
