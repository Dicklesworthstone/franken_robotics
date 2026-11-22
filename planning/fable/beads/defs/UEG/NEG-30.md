---
key: "NEG-30"
title: "zoo: the `zoo/` directory of pinned archetype instances, `frob.zoo.expect/1` and its conformance test"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "gauntlet", "frob-conformance", "frob-design", "solid", "test", "registry"]
parent: "WP-38"
deps: ["BD-109", "BD-054", "BD-055", "BD-056", "BD-062", "BD-110", "N14a-21", "N14a-26", "BD-025"]
needs: []
related: ["NEG-31", "BD-246", "N14b-06", "N14b-10", "N14b-18", "BD-102"]
gates: []
plan_refs: ["Appendix G.4", "§14.10", "§14.11", "§14.14", "§14.15 GATE-SKEL-03", "§13.4"]
---
## Background

The project ships eighteen robot archetypes: parameterized designs in `assets/archetypes/`, listed by the verb
`design.templates`. Each exists to exercise named mechanisms and has a known answer that fails if the
mechanism is wrong, so an archetype is a test fixture as much as a starting point for an agent. The templates
themselves are written by the design-language beads (four at M1, fourteen at M3). What those beads do not fix
is a per-archetype golden: the exact instance, its identity, and everything preflight must say about it. The
zoo is that golden. For each archetype it holds the pinned default instance and an expectation file; a
conformance test recomputes every expectation. Eighteen beads (`ZOO-<name>`) each author one pair of files;
this bead builds what they share.

Why a second directory beside `assets/archetypes/`. A template is parameterized and evolves (a new lever, a
new preset); a golden must not move silently. The zoo instance is the output of
`frob design new --template <name>` at default parameters, checked in with its `DesignId`. If a template
change alters the default instance, the zoo test fails and the change is either reverted or recorded as a
dated amendment that regenerates the expectation. Rejected alternative: asserting on the template directly,
which lets a default drift without any hash changing in a reviewed file.

## Scope

In: the directory layout; the expectation file format; the generic test that reads one archetype's pair and
checks it; the rule for regenerating an instance; the catalogue-level checks (all eighteen present, numbering,
milestone availability). Out: the templates and `design.templates` (BD-109, N14b-02, N14b-03, N14b-04); the
catalogue-wide gate measurement (BD-246); preflight itself (BD-110, N14b-10); the end-to-end script (NEG-31).

## Technical approach

**Layout (DESIGN).**

```
zoo/<name>.fbot            # canonical frob.design/1 JSON: the default instance, constructors unexpanded
zoo/<name>.expect.toml     # frob.zoo.expect/1: everything below
zoo/<name>.variants/       # optional named variants used as negative controls (one .fbot + .expect.toml each)
zoo/INDEX.toml             # number, name, template path, first_milestone, DesignId, expectation digest
```

**Expectation file `frob.zoo.expect/1`.**

```toml
schema = "frob.zoo.expect/1"
name = "quad-mammal"   number = 1   template = "assets/archetypes/quad-mammal"   first_milestone = "M1"
design_id = "<hash>"                     # of zoo/<name>.fbot; pinned; identical on Linux and macOS
[morphology] layout = "<prose>"  actuated = 12  n_v = 18  bodies = 13  end_effectors = 4  floating_base = true
[symmetry]   gamma_mech = "C2xC2"  generators = ["mirror-xz", "mirror-yz"]  gamma_obs_default = "C2xC2"
[[symmetry.breaker]] edit = "add-sensor forward-lidar"  gamma_obs = "C2"  breaker = "sensor:lidar-front"
[[preflight]] stage = 1  status = "pass"  reports = { fills = 13, contact_laws = ["foot-patch"] }
[[preflight]] stage = 4  status = "pass"  reports = { growth_vector = "n/a (legged)" }
[[known_answer]] id = "KA-096"  name = "contact_strata"  value = "27"  tol_kind = "count"
[[control]] name = "<name>"  variant = "variants/<file>"  expect_refusal = "FROB-REF-109"
[reference_robots] names = ["go1", "go2", "a1", "anymal-b", "anymal-c", "spot"]   # keys of assets/robots/
```

Rules: numbers are strings parsed exactly (as for worked cards); every `[[preflight]]` row names a stage 1 to
8, a status (`pass`, `finding`, `refused`, `undecided`, `not-applicable`) and the values the stage must
report; stages 7 and 8 need a task and a scene and are present only where the archetype's bead supplies them;
every archetype has at least one `[[control]]` variant that must be refused or must change a reported value.

**The generic test** (`zoo_<name>` in `crates/frob-conformance/tests/zoo.rs`, one function per archetype
generated from `INDEX.toml`):

1. Parse `zoo/<name>.fbot`; recompute `DesignId`; compare with the pinned value.
2. Instantiate the template at defaults in process; assert the result's `DesignId` equals the zoo instance's
   (the zoo is the template's default, not a fork).
3. Compile; compare the `[morphology]` counts with the compiled model.
4. Run the symmetry analysis; compare Γ_mech and the default Γ_obs; apply each `breaker` edit and compare.
5. Run `design.check` stages 1 to 6 with `--all`; compare each stage's status and reported values.
6. Evaluate each known answer through the mechanism its archetype bead names; compare under its tolerance.
7. For each control variant: compile or check and assert the stated refusal code or value change.

Before `first_milestone` the function reports `not run`. Evidence of the test: claim Known answer, class
Tested, tier G1 · host for static rows and G1 · referee where a row needs dynamics.

**Regeneration rule.** `zoo/<name>.fbot` is written only by an explicit developer command that instantiates
the template and stages both the instance and a changelog line in `zoo/INDEX.toml` (date, reason, old and new
`DesignId`). CI fails if `design_id` changes without a new changelog line.

**Catalogue checks.** Exactly eighteen entries numbered 1 to 18 with the names on file; each name appears in
`design.templates`; `available` in the verb's output equals "first_milestone reached".

## Acceptance criteria

- [ ] The schema, the loader and the generic test exist; with no archetype files the catalogue check lists
      the eighteen missing names.
- [ ] A `synthetic` archetype pair exercises every rule: a changed instance without a changelog line, a
      missing control, a preflight row with an unknown stage, and a float literal are each rejected.
- [ ] `zoo/INDEX.toml` is complete at M3 with eighteen pinned `DesignId` values identical on both platforms.

## Test plan

### Unit tests

- `zoo_expect_schema`, `zoo_expect_rejects_float_literal`, `zoo_expect_requires_control`.
- `zoo_index_has_eighteen_names`: the names and numbers on file (1 `quad-mammal` … 18 `balancer-2w`).
- `zoo_instance_equals_template_default`: on the `synthetic` archetype.
- `zoo_design_id_change_needs_changelog`.
- `zoo_not_run_before_milestone`.

### End-to-end script and logging

`scripts/e2e/zoo_goldens.sh` (NEG-31) drives the same checks through the `frob` binary. This bead supplies
the JSON status line per archetype that the script reads.

### Known answers and negative controls

The values live in the eighteen expectation files. This bead's controls are the format violations above and a
planted template drift (a default parameter changed by 1 mm) that must fail step 2. Determinism: `DesignId`
is content-addressed and platform-independent; the test output is byte-identical between runs.

## Considerations

- The instruction for this unit names `zoo/<name>.fbot`; the specification's layout names `assets/archetypes/`
  for the shipped templates. Both exist here with distinct roles, and step 2 ties them together.
- Presets (a template bound to approximate a reference robot) are not zoo instances; their mass rows belong to
  the catalogue gate.
- The eighteen archetype beads may be worked in parallel once this bead is done; they conflict only in
  `zoo/INDEX.toml`, whose rows are keyed by name.

## References

Appendix G.4; §14.10; §14.11; §14.14; §14.15 (GATE-SKEL-03); §13.4.
