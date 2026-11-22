---
key: "N19-REF"
title: "registry: Habitat refusal codes FROB-REF-301 to -317 with meanings, triggers, ranked repairs and exit codes"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "habitat", "frob-scene", "solid", "registry", "test"]
parent: "WP-17"
deps: ["BD-005", "STD-TEST"]
needs: ["§19.5: a refusal code for running on a pack whose acceptance report is incomplete or failed (asked for by the scene.accept bead BD-228; none exists in 301 to 317)", "§19.21: a refusal code for far-field reuse requested under determinism class D3 or D4, or for a field kind whose contract states no stencil (the subsection says 'refused' and names no code)", "§19.15: a refusal code for a reset that asks for a scene variant not resident in the world's bound pool segment (FROB-REF-314 covers admission only)"]
related: ["N19-KA", "N19-CX", "BD-162", "BD-163", "BD-164", "BD-228", "BD-233", "BD-235", "BD-236", "BD-369", "BD-398", "N19c-02", "N19c-03", "N19c-12", "N19c-27"]
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
code and the bead that raises each; the binding check; the list of refusals the chapter implies and does not number.
Out: raising the refusals (each row names its bead); the known-answer registry (N19-KA) and the counterexample
registry (N19-CX).

## Technical approach

1. **File.** Rows are added to `registries/refusals.toml` (owner `frob-core`; DESIGN field names): `code`, `class`
   (`scene`), `message`, `trigger`, `block` (the keys the refusal block must carry: at least `pointer`, `observed`,
   `required`, `unit`), `repairs` (ordered), `exit`, `raised_by` (crate and function), `test` and `bead`.
2. **Binding check** (`crates/frob-conformance/tests/refusals_habitat.rs`): every row has a test that provokes it
   through the public verb and asserts the code, exit 3, the block keys and that the first repair, applied, makes
   the same request admissible (a repair that does not repair is a defect). A row whose mechanism has not landed is
   `pending: <bead>` and is reported, not failed, until that bead's milestone.
3. **Rows.**

| Code | Refusal (meaning) | Trigger | Ranked repairs | Exit | Raised by |
|---|---|---|---|---|---|
| `FROB-REF-301` | Source geometry is not watertight or not consistently oriented | Compiler stage 4: the manifold or orientation certificate of a part tagged collidable fails. The scene is refused, never patched silently | 1. Request explicit repair with receipts (each closed hole is recorded, marked `derived: hole-fill`, and its neighbourhood becomes unknown). 2. Supply a closed model | 3 | N19a-13 |
| `FROB-REF-302` | The transform class of an import is not established: a reflection or affine map is not excluded, or winding disagrees with the determinant | Stage 2 or acceptance: fewer than four non-coplanar correspondences for a pipeline that admits a parity change; LᵀL ≠ c²·I or det L ≤ 0 within tolerance; c disagrees with the scale witness; tetrahedron volume under 1% of the cube of the largest separation; negative signed volume after baking; the glTF camera check fails | 1. Supply a fourth non-coplanar landmark. 2. Declare the source handedness. 3. Fix winding | 3 | N19a-11, N19a-21 |
| `FROB-REF-303` | A decode limit is exceeded or a reference does not resolve | Stage 1 or any pack read: bytes, instance nesting depth, polygon, texture or material count over the import profile; a relative reference leaves the pack; a blob is missing; a stored value lies outside its stated range | 1. Raise the limit in the import profile. 2. Supply the asset | 3 | N19a-10, BD-162 |
| `FROB-REF-304` | Scale authority is below what the task requires | A metric task is bound to a scene whose scale label is below the task's requirement (`relative-only` for any metric task; `estimated` where `witnessed` is required) | 1. Add a scale witness. 2. Choose a non-metric task | 3 | N19c-02 |
| `FROB-REF-305` | A required route, spawn or contact lies in an unknown region | A spawn or goal in or adjacent to unknown space; a task that requires known traversability on a route through unknown space; a required contact surface that is unknown | 1. Emit a probe request for the named box (`scene.probe-request`). 2. Choose another route | 3 | N19c-03 |
| `FROB-REF-306` | A feature below voxel scale has no overlay, or an overlay index cell is over capacity | Stage 6: a part tagged sharp or thinner than two fine voxels cannot be expressed by an overlay primitive; or a 0.64 m index cell needs more than eight overlay slots | 1. Add an overlay. 2. Bake at a finer level. 3. Raise the capacity | 3 | N19a-17 |
| `FROB-REF-307` | A film is outside the validity of the film model | A film deeper than h_max = (3ν·Δx²/(4·g·Δt))^(1/3) (2.5 mm for water at Δx = 2 cm, Δt = 2 ms; 10 mm for a 65 mm²/s oil), or otherwise outside the stated validity domain | 1. Sub-cycle. 2. Treat the liquid as a pooled obstacle. 3. Use a static patch | 3 | N19b-08 |
| `FROB-REF-308` | A clearance query is outside the domain of the complex: the footprint changes shape along the route, a height or step capability is missing, or a kinematic label is requested for a base that cannot reverse | `scene.fit` or analysis A-18 on such a query | 1. Supply the envelope. 2. Split the route by posture. 3. Accept `geometric-only` | 3 | N19b-30 |
| `FROB-REF-309` | A run is requested on a route that has a standing impossibility cut for this footprint | Run admission finds a certified cut on file for (scene, route, footprint) | 1. Read the cut. 2. Edit the design. 3. Run `scene.retrofit`. 4. Pass an explicit diagnostic override | 3 | N19b-32 |
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
   the training manifest; exit 3; raised at scoring (N19c-12).
4. **Refusals implied and not numbered.** Three situations in the chapter are said to be refused and have no code
   (the `needs` field lists them). Until codes are assigned (318 and 319 are free in the block) the behaviour is
   fail-closed: the verb returns exit 5 "not admitted" with a named reason string, and this registry lists the three
   as `unnumbered` so the gap stays visible.

## Acceptance criteria

- [ ] `registries/refusals.toml` contains exactly the seventeen Habitat rows with the fields above; the loader rejects a duplicate or unknown code.
- [ ] Every row is bound to a provoking test or marked `pending: <bead>`; at each milestone exit no row whose bead's milestone has passed is pending.
- [ ] For every bound row the test asserts exit code 3, `ok: false`, the code, the block keys, and that applying the first repair makes the request admissible.
- [ ] `frob --json refusals --class scene` (or the registry dump) lists the rows; the three unnumbered refusals appear in the gap report.
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
- 303 is used both for decode limits and for a stored value outside its stated range (a DESIGN choice of the manifest
  bead); if the registry owner prefers a separate code, only the manifest reader changes.
- The acceptance-incomplete case matters most of the three gaps: a pack that failed acceptance must not be runnable.
- Security: refusal blocks echo imported text only under the untrusted-text mark; paths are never echoed for private packs.

## References

§19.16 (table and the note on 502), Appendix O.3, §32.4 (exit codes), §24.14 (FROB-REF-502), §19.5, §19.15, §19.21.
