---
key: "NFx-01"
title: "upstream register: late rows FD-FSIM-39 to -42 and FD-FSYM-06, FD-FMCP-02 reserved; a count that follows the lists"
type: "task"
priority: 1
milestone: "M0"
labels: ["m0", "suite", "workspace", "upstream", "registry", "test", "solid"]
parent: "WP-37"
deps: ["BD-037", "N33c-47"]
needs: []
related: []
gates: []
plan_refs: ["Appendix F.1", "Appendix F", "§33.8", "§33.13", "§33.18", "§22.20", "§31.10", "§12.8"]
---
## Background

Every sibling repository FrankenRobotics consumes belongs to the project owner, so a limitation, defect or
mismatch in a sibling is work this program does in the sibling's repository, under that repository's own
`AGENTS.md`, gates and conventions. Each such change is an **upstream work item**: a bead here with the label
`upstream`, a mirrored bead in the sibling's tracker, and a row in `registries/upstream.toml` (schema
`frob.upstream/1`) holding the change, a priority, the interim posture here, the switch-over rule and one
reproducible failing fixture. The interim posture and the switch-over rule exist so that no milestone waits on a
sibling change landing.

The register is written by two beads. BD-037 defines the format and writes the 53 rows of the plan's table
(`FD-FSIM-01` to `-33` and the others). N33c-47 writes six more for the later foundation facts (`FD-FSIM-34` to
`-38`, `FD-F3D-02`, carried by NF-07 to NF-12), which brings the register to 59. Each of the two counts its own
identifiers from a constant and accepts a well-formed row it does not know.

That leaves work items that are beads in this graph, do work in a sibling, and have no row and no identifier:

- the input-affine (bilinear) EDMD fit in `fs-surrogate`, which the control chapter calls an upstream change
  without numbering it (bead N22c-22);
- the two legs of the first toolchain train, which the plan names only inside the row of `FD-FMCP-01` ("the
  same toolchain train advances FrankenSim's pin and moves frankensympy"): NF-03 and NF-04. Both beads store a
  mirrored-bead identifier "in this item's row", and no bead writes that row;
- two items raised when gaps in the suite chapter were closed: an external differentiable stage in the design
  map of `fs-topopt` (NFx-02) and a command-line entry for the standalone checker `fs-checker` (N33bx-01);
- the conditional raw-frame hook in `fastmcp_rust` (NF-13), which writes its own row only if it is opened.

Nothing checked the register as a whole either: that every upstream bead has exactly one row, that a number is
never handed out twice by two beads writing rows at different times, and that the total equals what the row
writers say they wrote. This bead writes the missing rows and adds that check. Tier: Solid. Rejected: raising
the constant in BD-037's count test each time a row is added (the count would then follow whoever edited last,
and a forgotten row would pass); one row for the whole toolchain train (the schema has one `mirror` per row, and
each leg is mirrored in a different sibling's tracker).

## Scope

In: five register rows with their identifiers, fixtures and mirrored beads; the reservation of `FD-FMCP-02`;
the series file that holds reservations and tombstones; the register-wide consistency test and its counts.
Out: the format, loader, fixture rule and first 53 rows (BD-037); the six rows of the later foundation facts and
the probe links (N33c-47); the upstream changes themselves, their switch-overs and their status transitions
(N22c-22, NF-03, NF-04, NFx-02, N33bx-01, NF-13); standing interim-posture checks (N33b-08).

## Technical approach

**1. Identifiers (DESIGN).** The next free numbers of each sibling's series, fixed when the row is written and
never reused. `FD-FSIM-39` goes to the EDMD fit because N33c-47 already names that number for it; the others
follow in the order NF-03, NFx-02, N33bx-01. If a number has been taken by the time this bead runs, the affected
rows take the next free run in the same order and the owning beads are updated in the same change.

| Identifier | Bead | Repo | Change | P | Interim posture here | Switch-over rule |
|---|---|---|---|---|---|---|
| FD-FSIM-39 | N22c-22 | frankensim | `fs-surrogate`: an input-affine EDMD fit z⁺ = A·z + Σⱼ uⱼ·Bⱼ·z beside the autonomous one, a rollout that re-lifts under a supplied input sequence, and split-conformal bands under recorded inputs | P3 | The fit, the rollout and the input-conditioned scores live in `frob-control`; only the band quantile and the simultaneous adjustment come from the sibling | Differential test of local against upstream (coefficients to 10⁻¹⁰ relative, bands equal); then delete the local fit and re-run GATE-REFLEX-10 |
| FD-FSIM-40 | NF-03 | frankensim | Advance the toolchain pin from `nightly-2026-07-06` to T under candidate (a): the FrankenSim leg of the first toolchain train | P1 | T is (a) if FrankenSim's crates already compile on it from this repository; otherwise T is candidate (b) and the train to (a) waits | The toolchain train: T and the pin move in one change; re-run INT-TOOLCHAIN-01's matrix, INT-FSIM-01 and INT-FSIM-02 to -11 on the new receipt |
| FD-FSYM-06 | NF-04 | frankensympy | Move the toolchain pin from `nightly-2026-08-20` to T, its suite passing there | P1 | `derive/` builds on T against the `fsym-*` crates by path; if they do not compile on T, derivation runs as a subprocess in FrankenSymPy's workspace and generated kernels are checked in with digests | The toolchain train; re-run INT-FSYM-01 and regenerate the checked-in kernels bit for bit |
| FD-FSIM-41 | NFx-02 | frankensim | `fs-topopt`: an optional external differentiable stage in the design map, between the length-scale filter and the projection, with its transposed-Jacobian action in the pullback | P3 | The optimization runs without an in-job overhang filter; results carry `overhang: post-check only` and the post-extraction re-run loop handles overhang | Re-run the capability probe, then the shelf fixture with the filter in the job and the overhang rows of GATE-FOUNDRY-07 |
| FD-FSIM-42 | N33bx-01 | frankensim | `fs-checker`: a command-line entry (one package or one witness in, one canonical verdict out) with no new dependency | P3 | The `fs-checker` arm of the second-checker experiment is recorded `not run`; one auditor with its mutation suite | Run the out-of-process admission lane at the new pin, then the arm |

Priorities (DESIGN, by the register's rule: the value to this project and the order in this program's queue,
never a reason for a milestone to wait): the two train legs are P1 because the preferred toolchain depends on
them and candidate (b) keeps the plan alive without them; the three capability items are P3 because each has an
interim posture that takes the sibling off the trusted path.

**2. Each row** is written in BD-037's format: `id`, `repo`, `change`, `priority`, `interim_posture`,
`switch_over`, `fixture` (`tests/upstream/<bead key>/`, named by the bead key as BD-037 rules), `admission_test`
(`INT-TOOLCHAIN-01` for the two legs; empty for the others until their probes exist), `bead`, `mirror`,
`status = "open"`, `observed_at`. The mirrored bead is opened in the sibling's tracker under that sibling's
rules with the row as its body.

**3. Fixtures at registration.** This bead writes all five fixture directories, because each owning bead
depends on it and the row must never exist without something that fails at the pin. For the two train legs the
design is the one NF-03 and NF-04 state, which those beads then keep current: a matrix cell as a script with its
expected result (NF-03: the M1 closure on candidate (a); NF-04: `derive/frob-derive` with the `fsym-*` crates)
plus the assertion on the channel in the sibling's toolchain file at the pin. For NF-04 that channel must equal
T (it is `nightly-2026-08-20`, which is neither candidate). For NF-03 it must equal candidate (a), not T: if T is
candidate (b), FrankenSim's present channel already equals T and an assertion against T would pass at the pin.
The three capability items need an airlock group that is
admitted at a later milestone, so their fixture has two stages (DESIGN). Stage 1, written here: the
`#[ignore]`d `frob-conformance` test `upstream_<bead key>` asserts that the needed public entry exists in the
pinned sibling checkout, by the pattern list in `expected.toml` (for `FD-FSIM-39` an EDMD entry that takes an
input sequence in `crates/fs-surrogate/src/koopman.rs`; for `FD-FSIM-41` a stage slot on
`fs_topopt::pipeline::DesignPipeline`; for `FD-FSIM-42` a binary target in `crates/fs-checker/Cargo.toml`). It
fails at the pin because the entry is absent, and `expected.toml` says "source-level: evidence for a decision,
not a proof of absence". Stage 2, written by the owning upstream bead when its group is admitted: the
behavioural assertion replaces stage 1 in the same directory.

**4. Reservations and tombstones.** `crates/frob-conformance/tests/data/upstream_series.toml` (test data, read
only by the test below; DESIGN: `registries/upstream.toml` refuses unknown fields, so this does not go there):
`reserved = [{ id = "FD-FMCP-02", bead = "NF-13", condition = "opened only if the byte-level guard cannot front
the library's transports", opened = false }]` and `tombstones = []`. A tombstone is `{ id, bead, reason }`: an
identifier withdrawn after being written, or a reservation whose bead closed it as not needed, listed here and
never reissued. NF-13 writes its own row when it is opened and sets `opened = true` in the same change; if it
closes as not needed it moves the reservation to `tombstones`. This bead only holds the number for it.

**5. The register-wide test** (`frob-conformance`, `upstream_register_whole`). It takes its expectation from
the lists, never from a total typed into the test:
1. *One row per upstream bead.* Every bead in the exported bead data whose title begins `upstream <repository>`
   is named by exactly one row's `bead`, or by one unopened `reserved` entry or one tombstone and no row; an
   opened reservation names exactly one row with its `id` and `bead`. `<repository>` is matched
   against the repository names of the register's `repo` values, so the two register writers titled
   `upstream register: …` (N33c-47 and this bead) and BD-037 (`upstream: …`) are not items. Beads that carry a
   part or a later stage of a registered item (NFp1-51 and NFp1-52, stages 3 to 5 of FD-FT-01; NFp4-01, the
   sibling half of FD-ASUP-01) are titled by repository without the `upstream` prefix and are not rows: the item
   keeps its one row and that row's mirrored beads.
2. *No orphan rows.* Every row's `bead` exists in the exported bead data; every `fixture` directory exists.
3. *Series.* Per prefix (`FD-FSIM`, `FD-FSYM`, …) the identifiers in rows, unopened reservations and
   tombstones form the run 01 to n with no number missing and none twice (an opened reservation is counted by
   its row).
4. *Count follows the lists.* The number of rows equals the size of BD-037's list (53) plus N33c-47's (6) plus
   this bead's (5) plus the reservations that have been opened. Each list is the constant its owner keeps, so a
   new row passes only when exactly one owner lists it.
5. Counts by repository and by priority are printed, with the numbers of unopened reservations and of
   tombstones (the script's `reserved` metric is read from this line).

Determinism: the registry and the series file are data; the test is a pure function of them, class D0.

## Acceptance criteria

- [ ] `registries/upstream.toml` holds the five rows above with every required field; `bead` is N22c-22, NF-03, NF-04, NFx-02 and N33bx-01; the rows of BD-037 and N33c-47 are untouched.
- [ ] Each of the five has a fixture directory named by its bead key whose test fails at the pinned sibling commit, with `observed_at` recorded, and a mirrored bead whose identifier is in `mirror`.
- [ ] `FD-FMCP-02` is reserved for NF-13 and no row carries it until NF-13 is opened.
- [ ] The register-wide test passes: 64 rows; frankensim 42, frankensympy 6, frankentorch 3, franken_lean 2, franken_threed 2, nine other repositories or groups one each; P0 2, P1 10, P2 16, P3 36.
- [ ] Each negative control below fails the test with the rule named.
- [ ] No sibling file was edited by this bead; the count tests of BD-037 (53) and N33c-47 (59) still pass unchanged.

## Test plan

### Unit tests

`cargo test -p frob-conformance upstream_register_whole`:
- `every_upstream_bead_has_one_row`: on the real export; expected no finding.
- `no_orphan_rows`, `fixture_directories_exist`.
- `series_are_gapless`: `FD-FSIM` runs 01 to 42, `FD-FSYM` 01 to 06, `FD-F3D` 01 to 02, `FD-FMCP` 01 to 02 (the second reserved).
- `count_follows_owner_lists`: 53 + 6 + 5 + 0 = 64.
- `reserved_identifier_has_no_row`; `opened_reservation_counts_once` (`synthetic` copy in which NF-13's row exists and its reservation reads `opened = true`: 65 rows, P2 17); `closed_reservation_as_tombstone` (`synthetic` copy in which the reservation has moved to `tombstones`: 64 rows, the `FD-FMCP` series still 01 to 02, NF-13 accepted by rule 1).
- `late_fixtures_fail_at_pin`: each of the five `#[ignore]`d tests, run explicitly, fails with the assertion recorded in its `expected.toml`.

### End-to-end script and logging

Step `late_rows` of `scripts/e2e/upstream_registry.sh` (the script of BD-037): `frob doctor --section upstream
--json` must list 64 register rows; the one reservation is counted from the output of `cargo test -p
frob-conformance upstream_register_whole`, because the series file is test data that `doctor` does not read;
then the five fixtures are run and must fail with their recorded signatures. Logged through the shared helper as NDJSON under `target/e2e-logs/upstream_registry/`: one `cmd`
event per fixture with exit code and wall time, `metric` events `rows` (64), `reserved` (1), `by_repo` and
`by_priority`, one `assert` event per rule of the register-wide test, the sibling commit of each fixture, and
on failure the first offending identifier with the rule it broke.

### Known answers and negative controls

Known answers: 59 rows before this bead and 64 after; frankensim 38 + 4 = 42; frankensympy 5 + 1 = 6; P1
8 + 2 = 10; P3 33 + 3 = 36. Negative controls, each a `synthetic` copy of the registry or of the bead export:
a second row with `bead = "N22c-22"` (rule 1); a bead titled `upstream frankensim: …` with no row (rule 1); a
row whose `bead` is `N99-99` (rule 2); `FD-FSIM-41` removed so the series has a hole (rule 3); `FD-FSIM-39`
issued twice (rule 3); a sixth late row that no owner list names (rule 4); a fixture edited to pass at the pin
(BD-037's rule: "fixture no longer demonstrates the limit"). Metamorphic: reordering rows changes no finding and
no count. Lanes: the test output is byte-identical on both platforms.

## Considerations

- Two reconcilers of the bead graph reported the missing EDMD row; this is the one bead that writes it.
- Stage-1 fixtures are source-level and weak by construction. They exist so that a row is never written
  without something that fails at the pin; the owning bead replaces each with a behavioural test.
- A number written into a row is permanent. If N22c-22 is closed by its kill gate before the change is made,
  the row stays with `status = "open"` and a note, or moves to the tombstones if the owner withdraws the item.
- RISK-55 (collisions with other agents in a sibling) applies to the mirrored beads only: registration edits no
  sibling source file.
- Hygiene: the register is read by `frob doctor`, by the switch-over procedure and by this test. The series
  file is test data and has no other reader.

## References

Appendix F (the rule, F.1, F.3), §33.8, §33.13 (the toolchain train and its legs), §22.20, §31.10, §12.8;
RISK-55; BD-037, N33c-47.
