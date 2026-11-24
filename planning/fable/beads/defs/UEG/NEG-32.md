---
key: "NEG-32"
title: "frob-io: sibling-catalogue robots as designs: a catalog builder lowered through the airlock to a design term"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "suite", "frob-io", "frob-fsim", "solid", "asset", "verb"]
parent: "WP-08"
deps: ["BD-007", "BD-033", "BD-054", "BD-061", "BD-065", "N14a-04", "N14a-13", "N14b-20", "N14b-34", "N33c-41", "VERBS-design"]
needs: []
related: ["NF-07", "ROBOT-g1"]
gates: []
plan_refs: ["Appendix G.1 (last row)", "§14.12", "§14.17", "§33 (INT-FSIM-05; foundation-fact row 9)", "Appendix J (design.import)", "Appendix H.2"]
---
## Background

Two of the nineteen reference robots are not files. The table of reference robots lists, beside the seventeen
models taken from a public model collection, "FrankenSim (`fs-mbd::robot_models`), with provenance", under the
suite's own licence: a Unitree G1 with 29 degrees of freedom, serving the `humanoid` archetype, and a KUKA
iiwa 7, the arm of `mobile-manipulator`. FrankenSim is the sibling physics library; its multibody crate ships
a robot catalogue whose entries are *builder functions* that construct a model in memory. FACT (read in the
sibling's source on 2026-11-21 at commit `483d09c682f2`): the builders live in
`frankensim/crates/fs-mbd/src/robot_models.rs` (1,488 lines); the G1 builder (lines 398 to 403) keeps 30 links
and omits fixed attachments with nonzero mass, so its masses sum to 31.96314202 kg against 33.34014202 kg for
the 38-link public source.

The project reaches FrankenSim only through the airlock crate `frob-fsim`, the one crate allowed to name the
sibling's types. At M0 the airlock hands out a catalogue model as an opaque referee handle
(`catalog_model(which) -> RefereeModel`) and a few facts about it (link count, coordinate layout, total mass).
That is enough to step the model on the referee and pin a state digest. It is not enough for a reference
robot. A reference robot is an *asset*: a directory with a manifest, an imported design with a pinned
`DesignId`, a loss report and a validation golden, so that the archetype mass rows, the analyses and the
presets can read it like any other design. Every file-based robot gets there through `design.import`. The two
catalogue robots have no file to import, and the verb that creates designs (`design.new`) accepts a template,
a file or a mission cell and nothing else.

This bead closes that gap once, for both robots: it lowers a catalogue builder's model to a design term. It is
the reverse of the compiler's referee lowering (design to referee model) and is checked against it.

DESIGN decisions and the alternatives rejected:

- **A format of `design.import`, not a new verb and not an option of `design.new`.** The operation is an
  import: an outside description becomes a design with a loss report. Rejected: `design new --from-sibling`
  (no loss report, no `--strict`, a second door for outside data); a hand-written `.fbot` copy of the
  builder's numbers (drifts from the sibling at the next pin move, and a typed copy is exactly what the asset
  manifest forbids); using the opaque referee handle as the asset (no `DesignId`, invisible to preflight,
  analyses and the mass gate).
- **Nothing is invented.** The builder states a kinematic tree and mass properties. It states no actuator
  card, no transmission, no sensor, no contact law and no collision geometry that the design language can
  cite. The lowering writes what the builder states and reports the rest.
- **A catalogue builder is not an oracle because it exists.** The lowering refuses a builder that has no
  admission record.

Tier: Solid. Subsystems: Skeleton (the importer) and Suite (the airlock accessor).

## Scope

In: the read-only structural accessor of the airlock; the lowering from a catalogue model to a design term;
the `fsim-catalog` format of the verb `design.import` with its loss report; the coordinate map between the
builder's layout and the compiled design; the round-trip comparison on the referee; refusals; tests.

Out: the airlock trait, the built-in G1 handle and the state digest (BD-007); the admission of the builders
(BD-033 for the catalogue smoke of both builders; N33c-41 for the G1 mass roll-up); the `reduced` block and
the profile `g1-reduced-30` (N14b-34); the loss-report schema and the field-coverage diff (N14b-20); the
asset directories, manifests and validation of the two robots (ROBOT-fsim-g1, ROBOT-iiwa7, on the layout of
NEG-15); the end-to-end script that drives this format (NEG-16); the upstream item that folds or declares the
G1's omitted masses (NF-07).

## Technical approach

**1. Structural accessor in the airlock (DESIGN; `frob_fsim::referee`, feature `referee`).**

```rust
pub enum CatalogRobot { G1, Iiwa7 }
pub struct CatalogTree { pub base: BaseKind, pub links: Vec<CatalogLink>, pub source: CatalogSource }
pub enum BaseKind { Floating, Fixed }
pub struct CatalogLink {
    pub name: String, pub parent: Option<u32>,          // index into `links`; None for the root
    pub joint: JointKind,                               // Revolute | Prismatic | Fixed
    pub axis: [f64; 3],                                 // unit axis in the joint frame
    pub origin: Pose,                                   // parent link frame -> joint frame at q = 0 (m, unit quaternion)
    pub limits: Option<JointLimits>,                    // lower, upper (rad or m); velocity, effort if stated
    pub mass_kg: f64, pub com_m: [f64; 3], pub inertia_com: [f64; 6],   // xx, yy, zz, xy, xz, yz about the centre of mass
}
pub struct CatalogSource { pub repository: String, pub commit: String, pub path: String, pub builder: String }
fn catalog_tree(&self, which: CatalogRobot) -> Result<CatalogTree, RefereeRefusal>;
```

The accessor reads FrankenSim's *public* accessors at the pinned commit and copies values; it holds no
sibling type in its result. If a quantity above has no public accessor at the pin, `catalog_tree` returns a
refusal naming the quantity, and the fallback of Considerations applies. No private field is reached and no
sibling source is copied into this repository.

**2. Lowering (`frob_io::catalog::lower`), numbered.**

1. *Admission.* Look up the admission record of the builder (written by the catalogue smoke of INT-FSIM-05
   and, for the G1, by the roll-up probe) for the pinned commit. Absent or for another commit: refuse. The
   result row of any caller is `blocked: admission`, never `pass`.
2. *Read.* `catalog_tree(which)`. Check: exactly one root; parents precede children; every axis has unit
   norm within 10⁻¹²; every quaternion has unit norm within 10⁻¹²; every mass is finite and ≥ 0.
3. *Numbers.* Every `f64` is written as its shortest decimal string that parses back to the same bits. The
   design language stores numbers as exact decimals, so the round trip `f64 → decimal → f64` is the identity
   and the design's parameters are the builder's, bit for bit.
4. *Bodies.* One body per link, `fill: declared` with `mass_properties` (mass, centre of mass, six inertia
   entries about the centre of mass). Each field carries its provenance and its source (sibling repository,
   commit, path, builder), as the file importers do with file and line: `datasheet` where the sibling states
   where its numbers come from, `assumed` where it states nothing at the pin. No Region, no collision and no
   visual geometry is written.
5. *Joints.* One joint per non-root link: class from `JointKind` (`R`, `P` or `fixed`), the screw axis and
   origin from the tree, limits where stated (a joint whose limits the builder does not state is written
   `unbounded` and listed in the loss report as `default-filled`). Fixed joints are kept as fixed joints;
   they are folded only on request (`--fold-fixed`, the folding rule of the import chapter).
   *The base.* The design format has no field for a robot bolted to the world: the compiler gives every
   root a `free` joint (six velocity coordinates) and a fixed-base mechanism is anchored by the task or the
   scene. A `Floating` builder therefore maps exactly. A `Fixed` builder (the arm) is lowered with the
   same free root, and the loss report carries one `no-exact-equivalent` entry, "fixed base: anchor by task
   or scene". The arm's own seven coordinates are unchanged; alone and unanchored, the compiled design has
   n_v = 13.
6. *What the builder does not state.* Actuators, transmissions, sensors, end effectors, contact laws, the
   power bus, budgets and symmetry assertions are absent from the term where the schema allows absence. The
   design language fills nothing by default, so preflight stages that need one of them report `not checked`
   on this design; they do not pass.
7. *Reduced models.* If the admission record marks the builder as reduced (the G1: bodies with nonzero mass
   omitted), the lowering refuses unless `--reduced <profile>` names a profile whose retained links equal the
   tree's links. It then writes the `reduced` block from the profile. This is the same refusal a file import
   raises when mass is dropped (N14b-34's refusal, exit 3, payload each dropped body with its mass and the
   total): the ledger row `import-mass-dropped` of N35x-02 (family `1xx`, proposal 127 on file, no number
   assigned), raised under its slug until the ledger assigns the number, as the `design.import` row of
   VERBS-design states; tests assert the slug (then the assigned code), never a proposed number.
8. *Canonical form and identity.* The term goes through the seven canonical-form rules and receives its
   `DesignId`. The identity depends on the sibling commit only through the numbers read.
9. *Coordinate map.* `coord_map.json`: for each generalized coordinate of the builder's layout, the index of
   the same coordinate in the compiled design's layout, and the base parameterization of each side. States
   and torques cross between the two models only through this map.

**3. Loss report.** `frob.lossreport/1` with `source.format = "fsim-catalog"`, `source.hash` the hash of the
canonical serialization of the `CatalogTree`, and `source.file` the builder's path and name. Entries, using
the existing classes: `not-checked` for mass support (no mass Region exists); `dropped` for any quantity the
accessor exposes and the term has no place for; `default-filled` for limits written `unbounded`;
`no-exact-equivalent` for a fixed base; `omitted` for each body of a reduced profile, with its mass.
The field-coverage diff runs over the tree's fields: `unaccounted` must be zero. With `--strict` a non-empty
report fails the import, as for every importer.

**4. Verb surface.** `frob design import --format fsim-catalog --builder <g1|iiwa7> --name <name>
[--reduced <profile>] [--fold-fixed] [--strict] --key <key> --json`. Output schema `frob.design.import.out/1`
(unchanged): the `DesignId`, the loss report, and in `evidence` the sibling repository, commit, builder and,
for a reduced model, `reduced = { profile, omitted_mass }`. Kind `mutate`, scope `design:write`, evidence
*static*, determinism class D0 (the term is a pure function of the tree). The registry row of
`design.import` gains the format value and the option `--builder`; the registry's parity and
schema-snapshot checks then cover them. An unknown builder name is refused as a malformed request
(FROB-REF-802, exit 3) and the refusal lists the admitted names. `frob-io` reaches the builders only through
`frob-fsim` (feature `fsim-catalog`); without the feature the format is absent from `capabilities` and the
verb answers with the capability refusal.

**5. Round-trip comparison (the check that the lowering means what the builder means).** Compile the lowered
design, lower it to the referee with the compiler's own lowering, and compare with the builder's handle:

- total mass equal as an exact decimal sum;
- link count and joint coordinate count equal; `nq` and `nv` equal for a floating-base builder, and equal
  after removing the six root coordinates for a fixed-base builder;
- every link's mass, centre of mass and inertia bit-identical to the tree's;
- forward kinematics: at 100 seeded random configurations inside the joint limits, the pose of every link
  frame relative to the root agrees within 10⁻¹² m and 10⁻¹² rad;
- forward dynamics, floating-base builders: at the catalogue reference state and at the same 100 states,
  with zero torques and with seeded random torques, accelerations agree within 10⁻¹⁰ relative (TARGET; both
  sides run the same articulated algorithm on bit-identical parameters, so the expected difference is zero
  where the tree order is preserved; a nonzero difference is logged with the first differing coordinate);
- forward dynamics, fixed-base builders: the same comparison with the design's root anchored by a scene
  anchor on the referee. Until an anchor is available at the milestone this row is `not run`, never `pass`,
  and the rows above stand.

A lowering that passes its own unit tests and fails this comparison is wrong.

**6. Ledger.** The import records the sibling commit, the stack-lock digest, the builder, the `DesignId`, the
loss-report hash and the round-trip maxima.

## Acceptance criteria

- [ ] `frob design import --format fsim-catalog --builder g1 --name fsim-g1 --reduced g1-reduced-30 --key
      cat-g1-1 --json` returns a design with 30 bodies, 29 actuated coordinates, n_v = 35 and total mass
      31.96314202 kg (exact decimal sum); the same call repeated with the same key is an idempotent replay.
- [ ] The same command without `--reduced` (key `cat-g1-2`) is refused, naming the profile and the omitted mass
      on record.
- [ ] `--builder iiwa7` returns a design whose counts equal the admission record's (expected, the builder at
      the pin governs: 7 revolute joints, 8 bodies) with a free root and the `no-exact-equivalent` entry for
      the fixed base.
- [ ] Both `DesignId`s are identical twice on one machine, across worker counts, in debug and release, and
      on Linux and macOS, for one sibling commit.
- [ ] The round-trip comparison passes for both robots and its maxima are recorded.
- [ ] The lowered designs contain no Region, no actuator card and no contact law; preflight stages that need
      them report `not checked`.
- [ ] A builder with no admission record for the pinned commit is refused and the row is `blocked: admission`.
- [ ] The loss report validates, has `unaccounted = 0`, and `--strict` fails exactly when it is non-empty.
- [ ] The verb registry row lists the format; the format is reachable through `frob --json` and absent from
      `capabilities` when the feature is off.
- [ ] `CONTRACT.md` of `frob-io` states the format, its inputs, what it never writes, and its refusals.

## Test plan

### Unit tests

In `crates/frob-io/tests/catalog.rs` and `crates/frob-fsim/tests/catalog_tree.rs`; the real builders through
the real airlock, no mocks.

- `catalog_tree_g1_shape`: 30 links, one root, floating base. `catalog_tree_iiwa7_shape`: one root, fixed
  base, link count equal to the admission record's (expected 8).
- `catalog_tree_units_and_norms`: unit axes and quaternions within 10⁻¹².
- `catalog_decimal_round_trip`: every number of both trees survives `f64 → decimal → f64` bit for bit.
- `catalog_lower_g1_counts_and_mass`: 30, 29, 35, 31.96314202 kg.
- `catalog_lower_g1_requires_reduced`: refusal without the profile; with a profile that names 29 links,
  refusal naming the missing link.
- `catalog_lower_iiwa7_counts`: 7 joint coordinates; free root; the fixed-base loss-report entry present.
- `catalog_lower_invents_nothing`: no Region, actuator, sensor or contact field in either term.
- `catalog_round_trip_parameters`, `catalog_round_trip_kinematics`: both robots, 100 seeded configurations.
- `catalog_round_trip_dynamics_g1`: accelerations on 100 seeded states.
- `catalog_coord_map_is_a_permutation`, `catalog_design_id_pinned`.
- `catalog_lower_blocked_without_admission`.
- `catalog_loss_report_coverage_zero_unaccounted`, `catalog_strict_fails_on_nonempty_report`.

### End-to-end script and logging

Driven by `scripts/e2e/assets_licences.sh` (NEG-16), robot loop, for the two catalogue robots: the import
command above, comparison of the `DesignId` and loss report with the goldens of the asset directory, and
`frob --json sim run <DesignId of the lowered G1> --tier referee --seconds 2 --key cat-g1-run` (sim.run's
positional design, BD-024 at M0; a distinct key from any other run). Log lines carry the sibling commit, builder, `DesignId`,
counts, total mass to eight decimals, the profile name, the round-trip maxima with their threshold, refusal
codes, and the evidence block.

### Known answers and negative controls

Known answers: the G1 roll-up 31.96314202 kg over 30 links (the admission known answer), 1.37700000 kg below
the 38-link source; the counts of both builders as recorded at admission. Planted defects the tests must
catch: an axis perturbed by 1 mm in the lowering (the kinematic comparison fails and names the joint); one
link mass dropped (the roll-up differs and the import is refused as mass dropped); a coordinate map with two
entries swapped (accelerations disagree);
numbers written with seven significant digits in place of shortest round-trip decimals (the decimal test
fails). Metamorphic relation: renaming links in the tree changes names in the term and nothing in the
compiled model's dynamics. Determinism lanes: twice on one machine; across worker counts; debug and release;
Linux and macOS; class D0.

## Considerations

- **If the sibling exposes no structural accessors at the pin.** Fallback, named in advance: an upstream
  work item is filed by description (public read accessors for the catalogue models, in the sibling's
  repository and under its rules), and until it lands the two designs are transcribed once from the builder
  source at the pin into `.fbot` files with provenance `transcribed`, and the round-trip comparison of step 5
  becomes their admission test. The verb format stays absent from `capabilities` in that posture.
- A sibling's newer clean head is never rolled back locally. A changed builder changes the numbers, hence
  the `DesignId`; the two asset beads regenerate their goldens through the switch-over rule.
- When the upstream item on the G1's omitted masses lands, the admission record changes, the `reduced`
  requirement shrinks or disappears, and the profile is reviewed.
- Nothing lowered here is a Reality claim, and a lowered design is never the import-parity reference for a
  robot that has a fuller public source.
- Licence: the lowered design carries the suite's licence entry, like every artifact derived from the
  sibling's catalogue.
- The format is deliberately narrow (two admitted builders). A third catalogue entry needs its own admission
  record before the name is accepted.

## References

Appendix G.1 (last row); §14.12 (importers, loss report); §14.17 (reduced models); §33 (airlock, INT-FSIM-05,
foundation-fact row 9, GATE-WS-09); Appendix J (`design.import`).
