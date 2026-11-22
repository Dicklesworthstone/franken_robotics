---
key: "FD-FSIM-26"
title: "upstream frankensim: a mesh entry point to the faceted STEP exporter of `fs-io`"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "foundry", "upstream", "frankensim", "frontier"]
parent: "WP-71"
deps: ["BD-037", "BD-407", "NF-01", "STD-E2E"]
needs: []
related: ["BD-408", "BD-401", "FD-FSIM-30"]
gates: ["GATE-FOUNDRY-09"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§31.4", "§31.12", "§33.9", "§33.8"]
---
## Background

Foundry turns a qualified design into a build packet, and the packet carries each made part's geometry as STL,
3MF, GLB and STEP. FrankenSim's `fs-io` (about 22.3 thousand lines, layer L2) provides the STEP machinery:
bounded Part-21 parsing and canonical writing at the syntax level (`step`); strict decoding of a triangle-only
faceted solid (`step_faceted`); and deterministic re-emission of a decoded faceted solid with mandatory
self-parse and bit-exact replay (`step_faceted_export`). FACT (read 2026-11-21): **the faceted exporter accepts
only a solid produced by its own decoder; there is no entry point from a mesh.** Its default limits admit 10⁶
instances per file. Upstream register priority: **P3**.

This project therefore reaches the exporter by a detour that needs no upstream change: Foundry builds a
syntax-level `StepDocument` of the faceted-solid entity chain, writes it with `write_step`, parses and decodes
it with `decode_faceted_brep_with_limits`, and re-emits it with `export_decoded_faceted_brep`, which refuses
unless every coordinate bit and triangle index replays. It works and it is exact; it also means this project
hand-assembles STEP entities, which is the kind of code a format owner should own. Rejected: writing STEP text
directly without the decode-and-replay check (an unverified export); adopting a third-party CAD kernel (outside
the suite's dependency rules).

## Scope

In: in `fs-io`, an entry point that takes a triangle mesh and emits the same canonical faceted STEP; here,
replacement of the detour and a hash comparison of the packet fixtures. Out: bounded geometry export on this
side, with decimation and refusal (BD-407); the build packet (BD-408); curved surfaces, units entities and
application-protocol conformance, which the exporter does not claim.

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `frankensim` repository
(`../frankensim` beside this one; head `483d09c682f2` and toolchain channel `nightly-2026-07-06` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions. For FrankenSim those rules include its layer checks, its contracts, its unsafe-capsule discipline, its ban on
in-process foreign kernels in production paths and its ban on branches and worktrees (FACT:
`frankensim/AGENTS.md`, `frankensim/docs/ACCELERATOR_DOCTRINE.md`), so the work is committed on `main` in small
steps that each pass its gates. FrankenSim moves fast (FACT: 409 commits touched `crates/fs-mbd` in the thirty
days to 2026-11-21, and its checkout had 32 modified paths), so every file named in this bead is re-read at the
current head before it is edited.

1. Read that repository's `AGENTS.md` and its coordination rules first. They govern; where they differ from this
   bead, they win, and the difference is written into this bead's closing note.
2. Other agents work in the same repository. Before editing, reserve every file to be touched through Agent
   Mail and follow the repository's own coordination rules; release the reservations once the change has landed.
3. The change is additive (DESIGN): a new mode, entry point, type or feature. Existing behaviour is not changed,
   and any lossiness is self-described in the new item's documentation and return type.
4. It lands on that repository's `main` through its own tests and gates. Nothing in this repository patches,
   forks, vendors or edits the sibling, and no tool of this repository writes into its checkout (INV-SUITE-04).
5. The work is tracked twice: this bead, and a mirrored bead in the sibling's tracker whose identifier is stored
   in the `mirror` field of this item's row in `registries/upstream.toml` (schema `frob.upstream/1`). The row's
   `status` moves `open` → `landed` (merged on the sibling's `main`) → `adopted` (switch-over run here).

**The change (DESIGN).** `export_faceted_brep_from_mesh(vertices, triangles, limits)`:

1. validates the mesh: indices in range; no degenerate triangle; closed (every edge shared by exactly two
   triangles), manifold, consistently oriented; within `limits`;
2. builds the same decoded representation its own decoder would have produced for that solid, in a canonical
   order (vertices and triangles in the order given, so that the output is a function of the input alone);
3. emits through the existing exporter, including the mandatory self-parse and bit-exact replay of every
   coordinate bit and triangle index;
4. refuses by a typed value: an open shell, a non-manifold edge, an orientation conflict, or a mesh over the
   instance limit, each with the offending element.

Instance arithmetic, for the limit: a closed triangle mesh with F triangles and V points uses three instances
per triangle and one per point; for a surface without handles V = F/2 + 2, so the count is 3.5·F + 2. The
default limit of 10⁶ instances therefore admits 285,713 triangles; a tetrahedron (F = 4, V = 4) uses 16.

Additive: the decoder-fed exporter is unchanged.

**Interim posture here.** Faceted export under a stated triangle bound through the detour above. A larger part
is decimated under a stated Hausdorff tolerance and re-checked, or refused (`FROB-REF-824`). The file has no
curved surfaces and no units entity; the packet's manifest states metres.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSIM-26/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSIM-26`, compiled as the `frob-conformance` integration test `upstream_fd_fsim_26` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It calls the mesh entry through the airlock trait `MeshExchange` with a
tetrahedron (absent at the pin) and, when present, compares the bytes with the detour's output for the same mesh.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) run `cargo xtask check-stack`, which writes a new resolution receipt
(`frob.stack-receipt/1`) under `receipts/`; (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**What the switch-over re-runs.** The rule is "re-export the packet fixtures and compare hashes": every
geometry fixture of the build-packet tests is exported once through the detour and once through the mesh
entry, and the two files must have the same BLAKE3 hash, since both end in the same exporter with the same
canonical order. Then GATE-FOUNDRY-09 is re-run (tetrahedron round trip bit-exact; open shell and over-bound
parts refused; mass properties from exported geometry within 10⁻⁶ of the model). If any hash differs, the
difference is diagnosed before the detour is removed, because a changed STEP file changes the packet's hash.

**Fallback if the change cannot land.** The detour stays; it is exact and tested.

## Acceptance criteria

- [ ] In FrankenSim: the mesh entry exists, validates, refuses the four cases by typed values, and passes the self-parse and replay check on every accepted mesh.
- [ ] In FrankenSim: for a solid decoded from a file, exporting from its mesh and re-emitting from the decoded solid give identical bytes.
- [ ] Here: every packet fixture has identical hashes through both routes, recorded; only then is the detour removed from `frob-foundry`.
- [ ] Here: GATE-FOUNDRY-09 passes at the new pin; the over-bound refusal still carries `FROB-REF-824` on this side and the upstream reason beneath it.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSim (added, in `fs-io`):
- `tetrahedron_from_mesh_round_trips`: vertices (0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1) with outward-oriented faces; expected a file that decodes to the same four vertices and four triangles bit for bit, with 16 instances.
- `open_shell_is_refused`: the tetrahedron with one face removed; expected the typed refusal naming three boundary edges.
- `non_manifold_edge_is_refused`: two tetrahedra sharing one edge; expected the refusal naming that edge.
- `flipped_triangle_is_refused`: one face with reversed winding; expected the orientation refusal.
- `instance_limit_is_enforced`: a closed mesh of 285,714 triangles against the default limit; expected refusal; 285,712 accepted.
Here:
- `detour_and_mesh_entry_hash_equal`: for each packet fixture.
- `exported_mass_properties_match_model`: volume, centre of mass and inertia computed from the exported triangles within 10⁻⁶ relative of the model's (unit tetrahedron: volume 1/6, centroid (¼, ¼, ¼)).

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsim_26.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsim_26 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob foundry export --json` on the packet fixtures and compares
each geometry file's hash with the hash stored from the detour.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsim_26/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSIM-26`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the tetrahedron (16 instances, volume 1/6, centroid (¼, ¼, ¼)); the bound of 285,713 triangles at 10⁶ instances; a mesh written through `fs-io` reads back with the same hash (INT-FSIM-15, KA-770).
- Negative controls: an exporter that reorders vertices by a hash map's iteration order must fail the hash comparison on a second run; a STEP file above the stated bound must be quarantined with a receipt, not parsed (the negative control of INT-FSIM-15).
- Determinism lanes: byte-identical output twice on one machine and on both platforms (class D0).

## Considerations

- The build packet is addressed by the hash of its manifest, which holds the hash of every file; this is why "compare hashes" is the switch-over rule and not a visual check.
- Every import still passes `fs-io`'s quarantine; the mesh entry is an export path and adds no import surface.
- A build packet is not an authorization to fabricate: its `fabrication_released` flag is always false, whatever the geometry route.
- No milestone waits on this bead. Its milestone label says when its switch-over gate first exists, not that the
  milestone's exit depends on it. Until the switch-over has been run, the interim posture above is the design of
  record. M1 depends only on FrankenSim, asupersync and FrankenSQLite, and no upstream work item is on its
  critical path.
- If this change never lands, the program still ships on the interim posture, at the cost stated there. A change
  that does land is adopted only through its switch-over rule: never by a local patch, a fork or a vendored copy,
  and never by rolling a sibling's newer clean head back. Neither a pin nor T moves during the ten-fold
  tournament of M7.
- The priority in the upstream register states the item's value to this project and its place in this program's
  queue. The risks carried at program level are the cost and ordering of upstream work and collisions with other
  agents in the same sibling; the mitigations are file reservation, small commits and the sibling's own gates.

## References

Appendix F.1 (row FD-FSIM-26), F.3; §31.4 (`fs-io`), §31.12 (bounded export; the bound), §33.9 (INT-FSIM-15); GATE-FOUNDRY-09; FROB-REF-824; KA-770.
