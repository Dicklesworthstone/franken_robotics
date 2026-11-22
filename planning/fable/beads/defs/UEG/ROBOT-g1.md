---
key: "ROBOT-g1"
title: "assets/robots/g1: Unitree G1 from MuJoCo Menagerie (BSD-3): the 38-link parity reference, 33.34014202 kg"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-io", "solid", "test", "asset", "robot"]
parent: "WP-08"
deps: ["NEG-15", "BD-066", "BD-067", "N14b-20", "N14b-33"]
needs: []
related: ["ZOO-humanoid", "ROBOT-fsim-g1", "N14b-34", "BD-109", "N14b-24", "BD-089", "BD-007", "ROBOT-h1"]
gates: []
plan_refs: ["Appendix G.1", "§14.10", "§14.15 GATE-SKEL-01, GATE-SKEL-07, GATE-SKEL-11", "§14.17", "Appendix F.2", "Appendix H.3"]
---
## Background

The Unitree G1 is the humanoid of the project's "H" workload and the reference for the `humanoid` archetype's
mass row. Two models of it are on file and must not be confused. This one is the public MuJoCo model of the
Menagerie collection (BSD-3; FACT from the research pass of 2026-11-21, re-checked at admission), whose link
masses sum to 33.34014202 kg over 38 links: it is the parity reference. The other is the sibling physics
library's own builder, which keeps 30 links summing to 31.96314202 kg and omits fixed attachments of nonzero
mass: it is a reduced profile and never the parity reference (its asset is a separate bead). The 38-link
figure is reported and is re-read from the primary source before a gate cites it. Neither model is the source
of a Reality claim.

## Scope

In: `assets/robots/g1/` (manifest, licence text, pinned source record, import, loss-report and validation
goldens); the mass on file for `humanoid@g1`; the comparison record against the sibling builder's roll-up.
Out: the shared layout and checks (NEG-15); the importer (BD-066); folding of fixed joints (N14b-33); reduced
models and the `g1-reduced-30` profile (N14b-34); the sibling asset (ROBOT-fsim-g1).

## Technical approach

**Source record.** MuJoCo Menagerie at the project's single pinned commit; directory expected `unitree_g1/`
(recalled, mark M: confirm at the pin; the directory may hold variants with and without hands, and the
manifest names the 29-degree-of-freedom entry). Licence BSD-3-Clause: vendoring allowed with notice, licence
text and the no-endorsement clause.

**Facts on file (reported; re-read at the pin).** 29 actuated degrees of freedom; 38 links including welded
attachments; total mass 33.34014202 kg; n_v = 35 after welded links are folded (30 moving bodies). Mass window
of the archetype gate: within 15%, that is 28.339 kg to 38.341 kg.

**Import path.** `frob design import --format mjcf assets/robots/g1/source/<entry> --out
assets/robots/g1/import.fbot --json`. The import keeps all 38 links; welded links are fixed joints in the
design term and are folded at compile by the pseudo-inertia sum (mass, first moment and second moment add in
the parent's frame), so the compiled model has 30 bodies and exactly the source's total mass. Expected
loss-report entries (HYPOTHESIS until read): visual meshes; fitted collision primitives; unmapped solver and
contact parameters; `armature`, `damping`, `frictionloss` to an `assumed` card; the ankle and waist, if the
public model writes them as plain serial joints, imported as such, with a note that the real robot's coupled
linkages are not in the file.

**Validation specifics.** (1) Mass conservation under folding: Σ of the 38 source masses equals the compiled
30-body total exactly in decimal arithmetic. (2) Comparison with the sibling builder's roll-up: the
difference, 33.34014202 − 31.96314202 = 1.37700000 kg, must equal the summed mass of the welded links the
builder omits, link by link; the record lists them. (3) Inertia consistency per folded body.

**Use.** The mass row of `humanoid@g1` (GATE-SKEL-07); import parity and contact-free dynamics parity; the
fold-equivalence battery (a folded model's dynamics equal the unfolded model's).

## Acceptance criteria

- [ ] `assets/robots/g1/` passes the licence check and validation; `mark = "P"`; the 38-link mass is the value
      computed at the pin, and a difference from 33.34014202 kg is recorded as a dated amendment of every row
      that used it.
- [ ] Folding conserves mass exactly; the compiled model has 30 bodies and n_v = 35.
- [ ] The 1.377 kg difference from the sibling builder is accounted for link by link.

## Test plan

### Unit tests

- `robot_g1_manifest_valid`, `robot_g1_licence_bsd3_matches`.
- `robot_g1_counts`: 38 links, 29 actuated, 30 folded bodies, n_v 35.
- `robot_g1_mass_38_links`: sum equals `mass_on_file`; window 28.339 to 38.341 kg computed from it.
- `robot_g1_fold_conserves_mass_and_first_moment`: exact decimal equality of mass and of the whole-body centre
  of mass at the reference pose before and after folding.
- `robot_g1_vs_sibling_omitted_links`: omitted masses sum to the difference.
- `robot_g1_loss_report_golden`, `robot_g1_design_id_pinned`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16): import, both checks, golden comparison, the mass comparison; logs
commit, entry file, link count, total mass to eight decimals, the omitted-link table and findings.

### Known answers and negative controls

The humanoid mass row (within 15% of the mass on file) and the import-parity row. Controls: an import that
drops welded links with mass (the total falls to the reduced figure and the importer must refuse unless a
`reduced` block declares it); the reduced model offered as parity reference. Determinism: exact decimal sums.

## Considerations

- Until the 38-link figure is re-read, every row that cites it carries that caveat.
- Recalled details (directory name, hand variants) are expectations; the reading at the pin governs.

## References

Appendix G.1; §14.10; §14.15; §14.17; Appendix F.2; Appendix H.3.
