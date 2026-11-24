---
key: "N19-REF"
title: "registry: Habitat refusal codes FROB-REF-301 to -317 with meanings, triggers, ranked repairs and exit codes"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "habitat", "frob-scene", "solid", "registry", "test", "review-notes-open"]
parent: "WP-17"
deps: ["BD-005", "N35x-02", "STD-E2E", "STD-TEST"]
needs: []
related: ["N19a-46", "N19c-05", "N19c-10", "N19c-27", "N19c-29", "N19c-30", "N19c-42", "N24a-31", "N32ax-01", "N33-REF", "N38a-33"]
gates: []
plan_refs: ["§19.16", "Appendix O.3", "§32.4", "§24.14", "§35"]
---
## Background

A **refusal** is the answer a verb gives when it will not compute what was asked: a stable code, a message, the
offending value with what was required, and **ranked repairs** (the typed changes that would make the request
admissible, best first). Refusals are not errors: the rule of the project is that a tool which cannot answer says so
with a reason and a repair instead of returning a plausible number. Codes are never reused; the machine-readable
registry is `registries/refusals.toml`, loaded by `frob-core`, and an unknown code is a load-time error.

Habitat (crate `frob-scene`, the subsystem that owns every part of a world that is not the robot) defines seventeen
codes, `FROB-REF-301` to `FROB-REF-317`. They are spread over many mechanism beads written by three drafting units.
This bead is the complete list, so that none is lost, and the loader check that every row is bound to a test. The
`3xx` block is shared: `FROB-REF-320` and `-321` belong to Traction (thin features, missing enclosures) and
`FROB-REF-360` to `-362` to Nemesis (legality profile); they are not rows here. `FROB-REF-502` belongs to Arena and
"keeps its meaning" in Habitat: it is listed at the end because the scene split supplies the data it checks.

Exit codes (one per refusal class, FACT of the surface contract): every `1xx` to `5xx` refusal exits with code 3
("refused input or question"), and the envelope has `ok: false` with a `refusal` block. Exit 1 is never emitted on
purpose. A capability that is not implemented or not in scope exits 5; an integrity fault (a recomputed identity
that differs from the stated one) exits 9. So all seventeen rows below exit 3.

## Scope

In: the seventeen rows as registry data with meaning, trigger, the fields of the refusal block, ranked repairs, exit
code and the bead that raises each; the binding check; the list of refusals the chapter implies and does not number,
and the full rows of the last five of them under their ledger numbers.
Out: raising the refusals (each row names its bead); the known-answer registry (N19-KA) and the counterexample
registry (N19-CX).

## Technical approach

1. **File.** Rows are added to `registries/refusals.toml` (owner `frob-core`; DESIGN field names): `code`, `class`
   (`scene`), `message`, `trigger`, `block` (the keys the refusal block must carry: at least `pointer`, `observed`,
   `required`, `unit`), `repairs` (ordered), `exit`, `raised_by` (crate and function), `test` and `bead`. The registry-wide check of N35-33 reads `raised_by` as
bead keys (a bead key or the word `unbound`), so in the merged registry the bead keys of the "Raised by" column
are written to `raised_by` and the crate and function to `raised_at`; `bead` is kept as written.
2. **Binding check** (`crates/frob-conformance/tests/refusals_habitat.rs`): every row has a test that provokes it
   through the public verb and asserts the code, exit 3, the block keys and that the first repair, applied, makes
   the same request admissible (a repair that does not repair is a defect). A row whose mechanism has not landed is
   `pending: <bead>` and is reported, not failed, until that bead's milestone.
3. **Rows.**

| Code | Refusal (meaning) | Trigger | Ranked repairs | Exit | Raised by |
|---|---|---|---|---|---|
| `FROB-REF-301` | Source geometry is not watertight or not consistently oriented | Compiler stage 4: the manifold or orientation certificate of a part tagged collidable fails. The scene is refused, never patched silently | 1. Request explicit repair with receipts (each closed hole is recorded, marked `derived: hole-fill`, and its neighbourhood becomes unknown). 2. Supply a closed model | 3 | N19a-13 |
| `FROB-REF-302` | The transform class of an import is not established: a reflection or affine map is not excluded, or winding disagrees with the determinant | Stage 2 or acceptance: fewer than four non-coplanar correspondences for a pipeline that admits a parity change; LᵀL ≠ c²·I or det L ≤ 0 within tolerance; c disagrees with the scale witness; tetrahedron volume under 1% of the cube of the largest separation; negative signed volume after baking; the glTF camera check fails | 1. Supply a fourth non-coplanar landmark. 2. Declare the source handedness. 3. Fix winding | 3 | N19a-11, N19a-21 |
| `FROB-REF-303` | A decode limit is exceeded or a reference does not resolve | Stage 1 or any pack read: bytes, instance nesting depth, polygon, texture or material count over the import profile; a relative reference leaves the pack; a blob is missing; a stored value lies outside its stated range; a required manifest member is missing or has the wrong type; a name a table refers to (a region, a route node) does not resolve, or a table's length differs from the count the manifest states (DESIGN: 303 is the pack reader's one code for "this pack cannot be decoded as written"; `observed` says which case) | 1. Raise the limit in the import profile. 2. Supply the asset | 3 | N19a-10, BD-162, N19a-04 |
| `FROB-REF-304` | Scale authority is below what the task requires | A metric task is bound to a scene whose scale label is below the task's requirement (`relative-only` for any metric task; `estimated` where `witnessed` is required) | 1. Add a scale witness. 2. Choose a non-metric task | 3 | N19c-02; `mission.new --from-scene` (N25c-29, M8: a surveyed scene that is `relative-only` or under-scaled, bound to a metric task) |
| `FROB-REF-305` | A required route, spawn or contact lies in an unknown region | A spawn or goal in or adjacent to unknown space; a task that requires known traversability on a route through unknown space; a required contact surface that is unknown | 1. Emit a probe request for the named box (`scene.probe-request`). 2. Choose another route | 3 | N19a-04 (a declared spawn or goal region in or beside unknown space, at pack validation, M2), N19c-03 |
| `FROB-REF-306` | A feature below voxel scale has no overlay, or an overlay index cell is over capacity | Stage 6: a part tagged sharp or thinner than two fine voxels cannot be expressed by an overlay primitive; or a 0.64 m index cell needs more than eight overlay slots | 1. Add an overlay. 2. Bake at a finer level. 3. Raise the capacity | 3 | N19a-17 |
| `FROB-REF-307` | A film is outside the validity of the film model | A film deeper than h_max = (3ν·Δx²/(4·g·Δt))^(1/3) (2.5 mm for water at Δx = 2 cm, Δt = 2 ms; 10 mm for a 65 mm²/s oil), or otherwise outside the stated validity domain | 1. Sub-cycle. 2. Treat the liquid as a pooled obstacle. 3. Use a static patch | 3 | N19b-08 |
| `FROB-REF-308` | A clearance query is outside the domain of the complex: the footprint changes shape along the route, a height or step capability is missing, or a kinematic label is requested for a base that cannot reverse | `scene.fit` or analysis A-18 on such a query | 1. Supply the envelope. 2. Split the route by posture. 3. Accept `geometric-only` | 3 | N19b-30; also raised by A-18 when a footprint that changes shape cannot be split by declared postures (N19b-34) |
| `FROB-REF-309` | A run is requested on a route that has a standing impossibility cut for this footprint | Run admission finds a certified cut on file for (scene, route, footprint) | 1. Read the cut. 2. Edit the design. 3. Run `scene.retrofit`. 4. Pass an explicit diagnostic override | 3 | N19b-32 (the standing cut); refused at run admission through the precondition table of the verb layer (N32ax-01) |
| `FROB-REF-310` | A pack or report would export a research-only or private asset | Pack time or export: the licence join or the private mark of an included asset forbids it | 1. Remove the asset. 2. Keep the pack local | 3 | N19b-02 |
| `FROB-REF-311` | A scene group straddles a split | `scene.split` (or a split read) finds members of one group in two parts | 1. Move the whole group | 3 | BD-235 |
| `FROB-REF-312` | A realism claim cites a scene with `assumed` dimensions | A report claims a scene is representative of real buildings while `dimensions_assumed` > 0 | 1. Record a norm or a measurement. 2. Drop the claim | 3 | N19a-39 |
| `FROB-REF-313` | A retrofit violates a constraint or an install class the mission does not permit | A kit breaks a catalog constraint or uses a forbidden install class | 1. Choose another amendment. 2. Widen the permitted class | 3 | N19b-38 |
| `FROB-REF-314` | World-bank admission would exceed the device cap | The estimated bytes of a pack plus the bytes already resident exceed the cap (default 85% of the device's measured working-set limit) | 1. Evict. 2. Lower the resolution. 3. Move the job to the large-memory machine | 3 | BD-236 |
| `FROB-REF-315` | A movable object has no declared behaviour | A part that can move declares none of static, articulated, free, prescribed | 1. Declare one | 3 | N19a-08 |
| `FROB-REF-316` | A surface region has no optical card | Stage 7: a region with no optical card (never a silent `diffuse`) | 1. Assign one, with provenance `assumed` if necessary | 3 | N19a-06 |
| `FROB-REF-317` | Collision-to-visual deviation exceeds the pack threshold with no waiver | Acceptance: a collidable region's deviation above the pack threshold | 1. Correct the collision model. 2. Tag the part `optical-only`. 3. Waive with a reason | 3 | N19a-23 |

   Carried from Arena, not a Habitat code: `FROB-REF-502`, "an evaluation reuses a scene from the policy's training
   manifest"; trigger: a scored evaluation scene whose identifier (or, by the group rule, whose group) appears in
   the training manifest; exit 3; raised at scoring (N24a-31).
4. **Refusals implied and not numbered.** Fifteen situations are said to be refused, in the chapter or in the
   beads that implement it, and have no code. This registry lists each by a stable slug, with its meaning, so that
   the gap stays visible and a raising bead has a name to cite. Every number comes from the allocation ledger
   `registries/refusal_allocations.toml` (N35-33, N35x-02); no mechanism bead invents a number of its own. The rows
   of the first ten (message, trigger, block fields, ranked repairs, exit) are fixed and entered by N19c-42, which
   depends on this bead; the rows of the last five, whose ledger rows are N35x-02's, are fixed below and entered by
   this bead under the numbers the ledger assigns them.

| Slug | Meaning | Family and exit | Raised by |
|---|---|---|---|
| `scene-pack-not-accepted` | A run on a pack whose acceptance report is incomplete or failed | `3xx`, 3 (proposed 318) | BD-228 |
| `scene-variant-not-resident` | A reset or schedule that names a variant not resident on the job's device | `3xx`, 3 (proposed 319) | N19c-22 |
| `field-reuse-preconditions` | Far-field reuse required under determinism class D3 or D4, or for a field kind with no stated stencil | `3xx`, 3 (proposed 322) | N19c-34, N19c-35 |
| `scene-pack-identity-mismatch` | A scene pack or a clearance complex whose recomputed identity, or a blob's hash, differs from the stated one | `9xx`, 9 (an integrity fault; number from the ledger) | BD-162, N19a-20, N19c-23, N19b-27 |
| `scene-split-group-unmatched` | A group that matches no rule of a split without a default part | `3xx`, 3 (proposed 324) | BD-235 |
| `scene-split-group-cited` | A move of a group that a frozen study or an open qualification request scored on | `3xx`, 3 (proposed 325) | BD-235 |
| `scene-split-count-mismatch` | A split whose stored counts differ from a recount | `9xx`, 9 (an integrity fault; number from the ledger) | BD-235, N19c-13 |
| `spawn-region-no-admissible-pose` | Compile stage 9 finds a spawn region with no pose at which the nominal footprint is free of interference and stable (repairs: move or enlarge the spawn region; remove or move the obstruction named most often; declare the nominal footprint with provenance) | `3xx`, 3 (proposed 327) | N19a-19 |
| `retrofit-cost-on-assumed-prices` | A cost verdict requested under `--strict-data` while a price in the comparison has provenance `assumed` | `3xx`, 3 (number from the ledger) | N19b-38, N19b-40, N19b-43 |
| `actor-subject-claim` | A claim about a scripted actor was requested: a requirement atom, metric, scorecard column or report sentence whose subject is a kinematic actor; retry `after-change`; `pointer` to the atom, metric or column, with the actor's identifier (repairs: state the claim about the robot under the declared script; drop the actor from the subject) | `3xx`, 3 (number from the ledger) | N38a-33 |
| `hazard-registry-invalid` | A verb that needs the project's hazard registry (`scene.hazards`, `scene.gen` for an interior family, a task check resolving `registry:hazards`) finds that it fails to load (a field, unit or range error); no partial registry is served | `3xx`, 3 (number from the ledger) | BD-227 |
| `scan-profile-absent` | A consumer (clearance query, route, run, analysis, still) would read the geometry of a scan pack imported without its scan profile | `3xx`, 3 (number from the ledger) | NEG-23 |
| `asset-source-not-default-pipeline` | A draw or import from an asset-source row with `default_pipeline = false` (research-only, per-object, not read, or prior art) without `--user-supplied`; nothing is written | `3xx`, 3 (number from the ledger) | N19b-02 |
| `scene-edit-invalid` | A derived `scene.compile` (`--base`, `--edits`) whose edit list cannot be applied to its base: a target that does not resolve, a parameter outside its kind's range, two edits setting one field differently, or an edit changing an aspect it does not declare | `3xx`, 3 (number from the ledger) | N19b-44 |
| `import-articulation-interpenetrates` | An articulated import (the `articulated-object` profile of NEG-24, the `kitchen-fixture` profile of NEG-20) whose kinematic sweep of a joint, from its lower to its upper limit and back in steps of at most 1° (revolute) or 5 mm (prismatic), finds a part penetrating its own fixture, frame or another part deeper than the recorded fitting or decomposition error; the part and the joint value are named and nothing is imported. DESIGN (N35x-02): a scene asset refused at import, not a design's self-collision (`FROB-REF-123`) | `3xx`, 3 (number from the ledger) | NEG-24, NEG-20 |

   Two of the ten are integrity faults. The registry's exit rule gives exit 9 only to `9xx` (and the listed
   integrity codes of `8xx`), so those two are `9xx` rows and not scene-family numbers. 323 and 326, once proposed
   for them, are withdrawn proposals in the ledger's `retired` list (N35-33's field; N35x-02 enters them), cited
   by no bead.

   **Rows of the last five** (entered here; each `3xx`, exit 3, retry `after-change`, `defined_in` as shown;
   raised behind the named constant shown; the ledger's `interim` field records the existing code under which a
   build that predates the allocation run raises it, with `detail` equal to the slug):

| Slug | `defined_in` | Block | Ranked repairs | Constant; ledger interim |
|---|---|---|---|---|
| `hazard-registry-invalid` | §19.8 | `pointer` the file, the slug and the field | 1. Correct the row. 2. Restore the registry from the repository | `HAZARD_REGISTRY_INVALID` (BD-227); `FROB-REF-802` |
| `scan-profile-absent` | §19.9 | `pointer` the pack and `provenance.scan_profile` | 1. Re-import the scan with the scan profile (from M5: `--kind place --profile hm3d-scan`). 2. Use another scene | `SCAN_PROFILE_ABSENT` (NEG-23); `FROB-REF-305` |
| `asset-source-not-default-pipeline` | §19.9 | `pointer` `/source`; `observed` the source key, its class and `read`; `required` a row with `default_pipeline = true`, or `--user-supplied` | 1. Re-run with `--user-supplied` (with `--licence-ack` for a `research-only` or `proprietary` row), accepting that the pack can never be exported. 2. Choose a default-pipeline source | `ASSET_SOURCE_NOT_DEFAULT` (N19b-02); `FROB-REF-802` |
| `scene-edit-invalid` | §19.2 (ADR-HAB-09, typed scene edits) | `detail` the variant (`UnresolvedTarget`, `ParamOutOfRange`, `Conflict`, `UndeclaredAspect`); `pointer` the offending edit; `observed` and `required` the value and the kind's range where one applies | 1. Name a target that resolves in the base (the nearest labels are listed). 2. Bring the parameter into its kind's range. 3. Drop one of the conflicting edits. 4. Correct the edit's declared aspects | `SCENE_EDIT_INVALID` (N19b-44); none |
| `import-articulation-interpenetrates` | §19.9 | `pointer` the penetrating part and the swept joint; `observed` the penetration depth and the joint value at the deepest step; `required` the recorded fitting or decomposition error; `unit` `m` for the depth (the joint value in rad for a revolute joint, m for a prismatic one) | 1. Correct the named joint's limits, or the named part's collision geometry, in the source asset and re-import. 2. Import another asset. (DESIGN: NEG-24 and NEG-20 state the refusal and what it names but no repair; these are the two changes open to the caller, and the first, applied, makes the same import admissible) | `IMPORT_ARTICULATION_INTERPENETRATES` (NEG-24, NEG-20); none |

## Acceptance criteria

- [ ] `registries/refusals.toml` contains exactly the seventeen Habitat rows 301 to 317 with the fields above, and a full row for each of the last five slugs of step 4 that the ledger has assigned (its number, `name` the slug, the block and repairs of the table); the loader rejects a duplicate or unknown code.
- [ ] Every row is bound to a provoking test or marked `pending: <bead>`; at each milestone exit no row whose bead's milestone has passed is pending.
- [ ] For every bound row the test asserts exit code 3, `ok: false`, the code, the block keys, and that applying the first repair makes the request admissible.
- [ ] The refusal table that `frob --json schema` serves (there is no `refusals` verb) lists the seventeen rows; each of the fifteen unnumbered refusals appears in the gap report by slug until its row is entered (N19c-42 for the first ten, this bead for the last five).
- [ ] Negative control registered: a build in which one mechanism returns a result where its row says "refuse" fails the binding test.

## Test plan

### Unit tests
- `habitat_refusal_rows_are_seventeen`: loads the registry, asserts the code set 301..317 and exit 3 for each.
- `every_row_bound_or_pending`: each `test` symbol exists or the row is pending with a bead key.
- `repair_order_is_stable`: the repairs of each row equal the table above, in order (a reordering is a contract change).
- `first_repair_repairs` (one per landed row): for example `FROB-REF-311` on a planted mirrored pair, then "move the
  whole group" applied, then the same `scene.split` succeeds; `FROB-REF-314` on an over-cap admission, then one
  eviction, then admission succeeds.
- `no_reuse_of_codes`: the registry history file shows no code with two meanings.
- `unnumbered_slugs_are_fifteen`: the gap report lists exactly the fifteen slugs of step 4 while none of their rows is entered, each with its family, and drops each slug when its row is entered; a planted sixteenth refusal raised in `frob-scene` with no row and no slug fails the report (negative control).
- `late_five_rows_entered`: for each of `hazard-registry-invalid`, `scan-profile-absent`, `asset-source-not-default-pipeline`, `scene-edit-invalid` and `import-articulation-interpenetrates` whose ledger row is `assigned`, the registry holds a row with that number, family `3xx`, exit 3, the block keys and the repairs of the table in order.

### End-to-end script and logging
`scripts/e2e/habitat_refusals.sh` sources the STD-E2E helper and walks the landed rows through the real `frob` binary
on synthetic fixtures (an open mesh for 301, the reflected room for 302, an over-limit glTF for 303, a `relative-only`
pack under a metric task for 304, a goal inside the unknown mask for 305, nine overlays in one index cell for 306, a
straddling group for 311, an over-cap admission for 314, an undeclared movable for 315, a region without an optical
card for 316). Each step logs `event: refusal` with the exact command, exit code, wall time, the refusal block
(`code`, `pointer`, `observed`, `required`, `unit`, `repairs`), then applies repair 1 and logs the passing rerun.
Output goes to `target/e2e-logs/habitat_refusals/<utc>.ndjson` with a transcript; the `summary` line counts rows
exercised, rows pending and assertions passed or failed. No private-scene data is logged.

### Known answers and negative controls
- Known answers used as provoking fixtures: KA-429 (reflected room, `FROB-REF-302`), KA-444 (a 4 mm water film,
  `FROB-REF-307`), KA-456 (mirrored layout across a split, `FROB-REF-311`).
- Negative controls: a planted mechanism that clamps instead of refusing (303), that splits by `SceneId` alone (311),
  that admits over the cap (314); each must make its row's test fail.
- Determinism: the refusal block is byte-identical twice on one machine and across platforms (D0: it is data).

## Considerations

- Meanings and repairs are governed by the chapter text; Appendix O is a compilation. Appendix O prints 311 as "A scene
  group straddles a split: Move the whole group", which folds the repair into the meaning; the row above separates them.
- 303 is used for decode limits, for a stored value outside its stated range and for a manifest that lacks a required
  member or whose tables do not match their counts (a DESIGN choice of the manifest bead: one code for "this pack
  cannot be decoded as written", told apart by `observed`); if the registry owner prefers a separate code, only the
  manifest reader changes.
- The acceptance-incomplete case matters most of the fifteen gaps: a pack that failed acceptance must not be runnable.
- Security: refusal blocks echo imported text only under the untrusted-text mark; paths are never echoed for private packs.

## References

§19.16 (table and the note on 502), Appendix O.3, §32.4 (exit codes), §24.14 (FROB-REF-502), §19.5, §19.15, §19.21.
