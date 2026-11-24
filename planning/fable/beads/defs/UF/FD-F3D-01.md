---
key: "FD-F3D-01"
title: "upstream franken_threed: exact pin `serde_json = \"=1.0.151\"` relaxed (the same change as in franken_textiles)"
type: "task"
priority: 3
milestone: "M3"
labels: ["m3", "suite", "tether", "upstream", "franken_threed", "frontier", "review-notes-open"]
parent: "WP-52"
deps: ["BD-037", "N33b-04", "N33c-24", "STD-E2E"]
needs: []
related: ["FD-FTX-01", "N19c-39", "N33c-09", "N33c-38", "N33c-46", "N33c-47", "N41-02", "NF-12"]
gates: ["INT-F3D-01", "GATE-WS-06"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§33.17", "§33.15", "§33.13", "§33.4", "§34.14"]
---
## Background

The viewer of FrankenRobotics is an ordinary Three.js page (r186, vendored with its hash; an exported replay
fetches nothing). The sibling project franken_threed contributes two things to it. Its
`DEPENDENCY_ADMISSION.md` discipline is already adopted for every foundation. And it offers an optional
renderer route: an ingest command that routes an ordinary application's WebGPU renderer to a new backend for
an admitted feature subset (FACT, `franken_threed/README.md`, "Honest status": Three.js r186 is pinned; pixel
parity holds for ten scenes on a software rasterizer; no hardware run and no speedup have been measured;
single-file packaging is not built). Its crates: `f3d-core` (10,688 lines), `f3d-math` (19,100), `f3d-graph`
(7,507), `f3d-runtime` (31,564). Posture: Pattern now; Subprocess when the viewer's feature subset is admitted
there. Tier: Frontier. Rendering is outside every claim of the product.

FACT: franken_threed pins `serde_json = "=1.0.151"` exactly, while this project's graph resolves 1.0.154
(through FrankenSim and frankenscipy; fastmcp_rust pins `=1.0.154`). It also pins asupersync `=0.5.0`, which
never enters a graph here. So its crates cannot share a Cargo graph with the core as they are: the `f3d-*`
crates are used with their `serde` feature off, and the renderer route is a subprocess. Upstream register
priority: **P3**.

Rejected: a local patch of the sibling's manifest; vendoring `f3d-math` (a lift of 19 thousand lines for a
handful of functions).

## Scope

In: in franken_threed, the exact `serde_json` pin relaxed to admit 1.0.154; here, the re-run of INT-F3D-01.
Out: the dashboard and viewer overlays (BD-300); stills and pixel inspection (BD-191); the ray-parameter hazard
in `f3d-math` (NF-12); franken_threed's asupersync 0.5.0 pin, which is a separate obstacle and is not in the
register (noted below).

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `franken_threed` repository
(`../franken_threed` beside this one; head `bf7abe16367e` and toolchain channel `nightly-2026-08-31` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions.

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

**The change (DESIGN).** The workspace requirement `serde_json = "=1.0.151"` becomes a requirement that admits
1.0.151 and 1.0.154, such as `">=1.0.151, <1.1"`; franken_threed's own lockfile may stay where it is. If the
exact pin protects byte-for-byte outputs of its admission records or scene files, that is tested there first:
its golden outputs must be identical under 1.0.154, or the item stops with the finding recorded.

**Interim posture here.** `f3d-*` crates are used with their `serde` feature off, and only those that bring no
asupersync (the mathematics crates; HYPOTHESIS, recorded per crate at INT-F3D-01); the renderer route is a
subprocess; `check-deps` reports one `serde_json` version per binary. Where they are used: never in a workspace
that builds a `frob` binary (franken_threed is a never-dependency sibling there, and `check-deps` refuses an
`f3d-*` package in a core crate with `suite.forbidden-package`, N33c-09); only an admission-probe workspace under
`tests/admission/` may path-depend on an `f3d-*` crate (the ray-transform probe of N33c-46 is one). This item
does not change that rule: after adoption the probe workspace may turn `serde` on, and the core still links no
`f3d-*` crate.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-F3D-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-F3D-01`, compiled as the `frob-conformance` integration test `upstream_fd_f3d_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It runs `cargo metadata` on a scratch workspace in which `f3d-math` with its
`serde` feature **on** sits beside FrankenSim's crates, and expects one `serde_json`; at the pin Cargo reports
the conflict between `=1.0.151` and `>=1.0.154`. franken_threed has no row in `franken-stack.lock` at M0, so under
BD-037's fixture timing rule the fixture was written against franken_threed's head as read on the day,
`observed_at` names that commit and the row carries `pin_state = "unpinned"`. franken_threed is first admitted by
the pilot N33c-24 (a dependency), which records its pinned revision; this bead then re-runs the fixture at that
pin, records that commit in `observed_at` with the signature observed there, and clears `pin_state`.

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

**What the switch-over re-runs.** INT-F3D-01: ten reference frames of the replay page built through
franken_threed's route match the plain bundle within 1/255 per channel on at least 99.9% of pixels, and an
unadmitted feature fails explicitly. With the pin relaxed, the `f3d-*` crates that the admission-probe workspace
uses may enable their `serde` feature; the unified feature set of `serde_json` in production binaries (which
link no `f3d-*` crate) must stay as the stack lock records it. The renderer route remains a subprocess, because `f3d-runtime`'s asupersync 0.5.0 pin still
keeps it out of the graph; this item does not change that.

**Fallback if the change cannot land.** `serde` off on the `f3d-*` crates; the plain Three.js bundle is the
viewer in every case.

## Acceptance criteria

- [ ] In franken_threed: the requirement admits 1.0.154; its tests and golden outputs pass with both versions.
- [ ] Here: the fixture resolves with exactly one `serde_json` at the new pin; `check-deps` shows the same version and feature set for every production binary as before.
- [ ] Here: INT-F3D-01 passes (pixel agreement and the explicit failure of an unadmitted feature), with the measured share of agreeing pixels recorded against 99.9%.
- [ ] Here: no crate that requires asupersync 0.5.0 is in any graph of this project, at every pin.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet; its `pin_state` field, `unpinned` as BD-037 wrote it, is cleared once the fixture has been re-run at the pin N33c-24 admitted.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In franken_threed (added):
- `goldens_identical_under_both_serde_json_versions`: regenerates the repository's golden admission records and scene files twice, once with its lockfile at `serde_json` 1.0.151 and once at 1.0.154 (`cargo update -p serde_json --precise`), and compares them byte for byte; expected: identical. A difference stops the item with the finding recorded.
Here (`tests/upstream/FD-F3D-01/`):
- `resolution_with_f3d_math_serde_on`: the fixture.
- `one_serde_json_per_binary` and `feature_set_unchanged`: from `check-deps`, against the stack lock.
- `no_asupersync_0_5_0_in_any_graph`: `cargo tree -d` and a search of every lockfile.
- `replay_frames_match_plain_bundle`: ten reference frames; for each, the fraction of pixels whose three channels each differ by at most 1/255 is at least 0.999.
- `unadmitted_feature_fails_explicitly`: a replay using a feature outside the admitted subset; the route must fail with a named error and never render something else.

### End-to-end script and logging
`scripts/e2e/upstream_fd_f3d_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_f3d_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob telemetry export-html <run> --world <address> --out replay.html --json` on N33c-24's fixture episode (the run, world and output file the row requires; the file lands under the session's export root), builds
the page through the route as a subprocess and through the plain bundle, renders the ten frames headlessly
and logs the agreeing-pixel share per frame.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_f3d_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-F3D-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: a frame compared with itself agrees on 100% of pixels; one `serde_json` version per binary.
- Negative controls: a frame with a 2/255 offset on one channel across the image must fail the 1/255 test; the old exact pin in a scratch graph must fail resolution; the planted `ftx-drape`-in-core graph of GATE-WS-06 must still fail; an `f3d-math` dependency added to a core crate must still fail `check-deps` with `suite.forbidden-package` after adoption.
- Determinism lanes: headless frames are rendered on the software rasterizer so that comparisons are repeatable.

## Considerations

- franken_threed's asupersync `=0.5.0` pin is a second obstacle to using its runtime crate in process and has no item in the register; the dependency rules keep it out of every graph. If the renderer route is ever wanted in process, that pin needs its own upstream work item, modelled on FD-FTUI-01.
- Ceiling, stated: this route changes rendering cost only, and rendering is outside every claim of the ten-fold thesis.
- The self-contained HTML export is this project's own build step: franken_threed does not yet package a single self-contained page.
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

Appendix F.1 (row FD-F3D-01), F.3; §33.17 (row 7), §33.15 (franken_threed), §33.13 (consequence 3), §33.4 (FD-F3D), §34.14; INT-F3D-01; GATE-WS-06.
