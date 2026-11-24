---
key: "NEG-30"
title: "zoo: the `zoo/` directory of pinned archetype instances, `frob.zoo.expect/1` and its conformance test"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "gauntlet", "frob-conformance", "frob-design", "solid", "test", "registry"]
parent: "WP-38"
deps: ["BD-025", "BD-054", "BD-055", "BD-056", "BD-062", "BD-109", "BD-110", "N14a-21", "N14a-26"]
needs: []
related: []
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
milestone availability). Out, with the bead that has it: the templates and `design.templates` (BD-109, N14b-02,
N14b-03, N14b-04); the catalogue-wide gate measurement at M3 (BD-246: the catalogue table of Γ_mech as the known
answer KA-270, which the eighteen expectation files must equal; the 24 planted asymmetries, which it stores as
variants in this bead's format, `zoo/<name>.variants/asym-<kind>.fbot` with `<kind>` one of `sensor`, `payload`,
`sole`, `gear`, on the six archetypes `quad-mammal`, `hexapod-radial`, `humanoid`, `diff-drive`, `mecanum` and
`swerve-4`; the mass rows of presets; and the one gate record it enters, GATE-SKEL-07's M3 part). The M3 part of
GATE-SKEL-03 is entered by BD-247 once the standing and rolling rows exist, and that of GATE-MARROW-03 by N13-22. The expectation files of this bead are where each archetype's group is "on file": BD-246 reads
`[symmetry]` from them and keeps no second per-archetype record. Preflight itself (BD-110, N14b-10); the
end-to-end script (NEG-31).

## Technical approach

**Layout (DESIGN).**

```
zoo/<name>.fbot            # canonical frob.design/1 JSON: the default instance, constructors unexpanded
zoo/<name>.expect.toml     # frob.zoo.expect/1: everything below
zoo/<name>.variants/       # named variants used as negative controls (one .fbot + .expect.toml each); the 24
                           # planted asymmetries of the catalogue gate (BD-246) are stored here as
                           # asym-<kind>.fbot, kind sensor|payload|sole|gear, on six archetypes
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
[[preflight]] stage = 1  status = "passed"  reports = { fills = 13, contact_laws = ["foot-patch"] }
[[preflight]] stage = 4  status = "passed"  note = "not-applicable"  reports = { growth_vector = "n/a (legged)" }
[[known_answer]] id = "KA-096"  name = "contact_strata"  value = "27"  tol_kind = "count"
[[control]] name = "<name>"  variant = "variants/<file>"  expect_refusal = "FROB-REF-109"
[reference_robots] names = ["go1", "go2", "a1", "anymal-b", "anymal-c", "spot"]   # keys of assets/robots/
```

(Keys are shown several to a line for space; the file has one key per line, as the strict TOML-subset reader of
`frob-core` requires.) Rules: numbers are strings parsed exactly (as for worked cards); every `[[preflight]]` row names a stage 1 to
8, a status and the values the stage must report. The status is one of the four words `design.check` itself
prints for a stage, as the staging contract fixes them (N14b-10): `passed`, `refused`, `undecided`, `not-run`.
An earlier draft of this format had its own five words (`pass`, `finding`, `refused`, `undecided`,
`not-applicable`), which could not be compared with the verb's output; what the two extra words meant is
carried by fields of the row: a stage that passes while reporting findings is `passed` with
`findings = ["<finding id>", …]`; a stage that runs and reports that its check does not apply to this base
is `passed` with the stage's own note, `note = "not-applicable"` (the stage-4 rule for a legged base or an arm,
N14b-11, which prints that note); and a stage that is not run at all is `not-run` with `reason = "<text>"`; stages 7 and 8 need a task and a scene and are present only where the archetype's bead supplies them;
every archetype has at least one `[[control]]` variant that must be refused or must change a reported value.
A `[[known_answer]]` row whose value cannot yet be evaluated carries `blocked = "<reason>"` with a reason from a
closed list (`derivation`: its derivation file is not on record, as for `balancer-2w`'s pole; `admission`: its
oracle is a sibling module not yet admitted, as for `tri-wheel-stair`'s stepping oracle; `milestone`: its
mechanism lands at a later milestone, named in the row as `due = "M4"`, as for the simulated rows of
`tracked-flipper` and `wheel-leg`, which NEGp1-53 evaluates at M4; `open`: the value on file is contested by an
open question of the archetype's template bead, named in the row as `open = "<bead key>"`, as for the group rows
of `snake-crawler` while N14bp1-01's question is undecided); the generic test
reports such a row as `blocked: <reason>`, never as passed, and a gate row that cites it is not supported by it.
The same field, with the same list, may stand on the `[symmetry]` table, on a `[[symmetry.breaker]]` row and on a
`[[preflight]]` row; step 4 or step 5 then reports that comparison `blocked: <reason>` with the observed output
beside it (a refusal on a blocked stage is logged with the row and not counted as a refusal of the defaults) and
evaluates the rest. A `milestone` reason without `due`, or
an `open` reason without `open`, is rejected.
The field is removed by a dated amendment when the blocker clears, and an unknown reason is rejected.

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
- [ ] `zoo/INDEX.toml` exists with the eighteen names, numbers and first milestones; each row's `DesignId` and
      expectation digest are filled by its `ZOO-<name>` bead (four at M1, fourteen at M3), identical on both
      platforms, and a row not yet filled is reported `missing` by the catalogue check, never as passed.
- [ ] A row's `note` is compared with the note the stage prints: a `synthetic` legged archetype whose stage-4 row
      omits `note = "not-applicable"` fails step 5, and one that writes `not-run` for stage 4 fails it too.

## Test plan

### Unit tests

- `zoo_expect_schema`, `zoo_expect_rejects_float_literal`, `zoo_expect_requires_control`.
- `zoo_preflight_status_words`: `passed`, `refused`, `undecided` and `not-run` load; `pass`, `finding` and
  `not-applicable` are rejected as unknown statuses; `note = "not-applicable"` loads on a `passed` row.
- `zoo_index_rows_unfilled_reported_missing`: the skeleton index with eighteen names and no `DesignId` reports
  eighteen `missing`, zero passed.
- `zoo_index_has_eighteen_names`: the names and numbers on file (1 `quad-mammal` … 18 `balancer-2w`).
- `zoo_instance_equals_template_default`: on the `synthetic` archetype.
- `zoo_design_id_change_needs_changelog`.
- `zoo_not_run_before_milestone`.
- `zoo_known_answer_blocked_never_passes`: a `synthetic` row with `blocked = "derivation"` is reported
  `blocked: derivation` and counted neither as passed nor as failed; `blocked = "later"` (not on the list) is
  rejected; a `synthetic` gate summary that cites the blocked row reports it unsupported.
- `zoo_blocked_open_on_symmetry`: a `synthetic` `[symmetry]` table with `blocked = "open"` and `open =
  "N14bp1-01"` loads, step 4 reports `gamma_mech` and `gamma_obs_default` `blocked: open` while the other steps
  are evaluated; a stage-5 `[[preflight]]` row so marked whose stage refuses (FROB-REF-109) is reported
  `blocked: open` with the code and is not counted as a refusal; the same table without the `open` key, and a `milestone` row without `due`, are rejected.

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
