---
key: "NEGp1-51"
title: "e2e: the M3 run of `assets_licences.sh`: all nineteen robots, the M3 scene-asset sources, and the 310/731/733 division end to end"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "frob-io", "solid", "test", "e2e"]
parent: "WP-08"
deps: ["BD-305", "N19a-46", "N19b-02", "N37-17", "N37-18", "NEG-16", "NEG-17", "NEG-21", "NEG-22", "NEG-23", "NEG-24", "NEG-25", "NEG-26", "NEG-27", "ROBOT-cassie", "ROBOT-iiwa7", "STD-E2E"]
needs: []
related: ["NEG-15", "NEGp1-54"]
gates: []
plan_refs: ["Appendix G (preamble)", "Appendix G.1", "Appendix G.2", "§19.9", "§37", "FROB-REF-310", "FROB-REF-731", "FROB-REF-733"]
---
## Background

`scripts/e2e/assets_licences.sh` (NEG-16) answers one question for a user or reviewer: is every asset in the
tree what its manifest says, under the licence it says, validated, and would any export of it break a term?
NEG-16 writes the script and runs it at M1 over the seventeen reference robots due then. Everything else it was
asked to cover needs M3 machinery: the two M3 robots (`cassie`, `iiwa7`), the scene-asset source register and
the pack export check with FROB-REF-310 (N19b-02), the import verb (N19a-46), the private root (BD-305), the
licence join over lineage (N37-17) and the one export door with FROB-REF-733 and FROB-REF-731 (N37-18). An M1
bead cannot depend on those, so this bead carries the M3 run. It was written in polishing round 1, when the
M3 promise was found inside the M1 bead and the ten M3 scene-asset rows (NEG-17, NEG-21 to NEG-27) had no
bead running their end-to-end steps.

The run is also where the division of refusal codes is checked across the whole asset family, through the real
binary. The plan defines FROB-REF-310 in the scene chapter ("a pack or report would export a research-only or
private asset") and FROB-REF-731 and FROB-REF-733 in the safety chapter, and does not divide them; N19b-02
DESIGNs the division and N37-18 implements the door: 310 (exit 3) is raised only by the pack export check, for
a scene pack, a scene variant or a kit or starter pack containing scene packs; every other export (a still, a
report, a replay, a run bundle, a scorecard, a design, a policy) is decided at the door from the lineage join,
731 (exit 5) for the private mark, checked first, and 733 (exit 5) for the licence set. One request gets
exactly one code.

## Scope

In: the M3 steps of the script: the robot loop over all nineteen (adding `cassie` and `iiwa7`); the scene-asset
loop over every register row due at M3 (`frob-scene`; `procthor`; `hssd`; `hm3d` in its M3 form; `partnet-mobility`;
`behavior-1k`; `objaverse-xl`; the prior-art rows of NEG-27), each row's steps as its bead lists them; the
division matrix below; self-test mutation (d); the helper tests; the M3 log. Out: the script's frame, its M1
run and mutations (a) to (c) (NEG-16); the rows and their mechanisms (the row beads, N19b-02, N37-17, N37-18,
BD-305, N19a-46); the M5 sources and the HM3D scan profile (NEGp1-54, NEGp1-52).

## Technical approach

**Run.** `scripts/e2e/assets_licences.sh --milestone M3` (offline by default; `--fetch` only on request), in a
scratch directory under `target/`, with a fresh private root configured for the run and removed by
`scene.purge` at the end, in its dry-run and confirm form (the purge's tombstones are asserted).

**Robot loop at M3.** NEG-16's five steps for all nineteen robots; `cassie` and `iiwa7` now run, the latter
through the `fsim-catalog` import format (NEG-32). The export probe (step 5) now meets the door: an export of
each import carries its licence entry and notices from N37-18, the assertion NEG-16 logs `not run` at M1. The
two-licence fixture of ROBOT-barkour runs here for the same reason: a `synthetic` report citing `barkour`
(Apache-2.0, with its NOTICE) and `go1` (BSD-3-Clause) carries both licence entries, both notices and the
stricter of each obligation, and an export holding both designs carries both licence texts; the planted report that
drops the Apache-2.0 entry when the BSD-3 entry is present must fail.

**Scene-asset loop at M3.** For each register row due at M3, the steps its bead lists (absence scans,
refused default imports, user-supplied imports of the `synthetic` stand-ins, the purity check of the starter
pack, the claim lint), with the commands and keys written there: every import is `scene import --kind asset`
with the row's profile (`scene-collection` for `hssd`, NEG-22; `scan-store` for `hm3d`, NEG-23;
`articulated-object` with `--behaviour articulated` for `partnet-mobility`, NEG-24; `generic` for `behavior-1k`,
NEG-25; NEG-17 and NEG-21 list their own exact commands); every `run` verb call carries its own `--key`, distinct
per call; every export is `frob --json artifact export <ref> --out exports/ --key …`; NEG-23's fail-closed step
asserts the code behind `SCAN_PROFILE_ABSENT` (never a literal) on `scene.accept`, on `scene.fit` and on a run.
The private-root purge is `frob --json scene purge <pack>` (a dry run returning the closure and its digest)
followed by `frob --json scene purge <pack> --dry-run=false --confirm <digest>` (an `admin` verb: no key). Rows due at M5 (`infinigen-indoors`, `google-scanned-objects`, `robocasa`) are logged
`not run` with their milestone, counted apart from passes. The register check of N19b-02 runs first: thirteen
keys, the schema's fields, and no more rows with `default_pipeline = true` than the schema permits.

**Division matrix.** For each user-supplied stand-in (`hssd`, `hm3d`, `partnet-mobility`, `behavior-1k`, a
user-supplied `procthor` entry imported with no declared class (class `unknown`, `NotRead`; a declared class
would weaken the row and is refused with 802, NEG-21), and an `objaverse-xl` object of class `unknown`) and for each export kind,
one expected outcome:

| Export of | Session without `private:export` | Session with `private:export` and the flag |
|---|---|---|
| The pack | 310, exit 3 (pack check) | 310, exit 3 |
| A scene variant of it | 310, exit 3 | 310, exit 3 |
| A still of a scene containing it | 731, exit 5 (door) | 733, exit 5 (door) |
| A report containing such a still | 731, exit 5 | 733, exit 5 |
| A scorecard over runs, one of which used the scene | 731, exit 5 | 733, exit 5 |

An `objaverse-xl` object of class `unknown` is always imported under the private root (N19a-46, step 1), so its
door rows are those of the table (731 without the scope, 733 with it). For `hm3d` in its M3 form only the pack
row is reachable through the binary: the scan pack's reader refuses the compile, render and run that would make a
variant, a still, a report or a scorecard (NEG-23), so those four cells assert that refusal (the code behind
`SCAN_PROFILE_ABSENT`, exit 3) at the producing step and are run as door rows at M5 on the profiled scan
(NEGp1-54, step 4); they are logged apart from the cells that passed. For the
`frob-scene` starter pack and generated scenes, every export succeeds and carries its one-entry licence
manifest. Each refusal carries the lineage path to the root asset (door) or the offending entry (pack check);
each request yields exactly one code.

**Self-test mutation (d).** A research-only stand-in placed in the starter pack: the purity check fails and the
pack export refuses with 310 (exit 3).

## Acceptance criteria

- [ ] `--milestone M3` passes for all nineteen robots and every register row due at M3; the three M5 rows are
      logged `not run` with their milestone.
- [ ] Every robot's export carries its licence entry and notices; the barkour-and-go1 report carries both
      entries and both notices, and the planted report that drops the Apache-2.0 entry fails.
- [ ] Every cell of the division matrix produces its expected code and exit code, exactly one code per request,
      and every refusal names its lineage path or offending entry.
- [ ] Mutation (d) gives 310; mutations (a) to (c) still give NEG-16's outcomes.
- [ ] The run's private root is purged at the end and only tombstones remain; no log line contains geometry.

## Test plan

### Unit tests

Helper tests in `scripts/e2e/tests/`:
- `division_matrix_complete`: every (stand-in, export kind, session) cell has exactly one expected code.
- `exit_code_mapping`: 310 maps to exit 3, 731 and 733 to exit 5; a planted 310 for a still fails the run.
- `one_code_per_request`: a response carrying two refusal codes fails the run.
- `m3_rows_cover_register`: every register row due at M3 has steps in the loop, and every row due later is
  logged `not run`.
- `purge_leaves_tombstones_only`: after the run, the private root holds tombstones and nothing else.
- `two_licence_join_fixture`: the expected licence set of the barkour-and-go1 report is {Apache-2.0 with NOTICE,
  BSD-3-Clause}; a log in which either entry or notice is missing fails the run.

### End-to-end script and logging

This bead is the M3 run of the script. Log: `target/e2e-logs/assets_licences/<utc>.ndjson` with a transcript,
through the shared helper of STD-E2E. Every line: `ts`, `script`, `bead` (the row's or robot's key, or
`"NEGp1-51"` for the matrix), `step`, `event` (`start`, `cmd`, `assert`, `refusal`, `metric`, `end`), the exact
command, exit code, wall time, the `evidence` block (claim, class Tested, tier static, determinism D0,
identities), source key, licence class, the session's scope and flag, the export kind, the expected and
observed codes and exit codes, the raising site (`pack-check` or `door`), the lineage path, the licence join,
notice presence, and on failure the first differing field and the paths needed to reproduce. The `summary`
line: robots and rows passed, failed, skipped (`not-read`) and not run; matrix cells passed and failed; codes
observed by raising site; `gates: []`. Logs contain identifiers and hashes, never geometry.

### Known answers and negative controls

The matrix is the known answer (N19b-02's DESIGN, N37-18's table). Controls: mutation (d); a build that raises
310 for a report (wrong raising site, caught by `exit_code_mapping`); a still exported through the pack path
(must still reach the door). Determinism: two runs give identical logs apart from timestamps and wall times.

## Considerations

- The private root is configured per run so that a user's real private store is never touched by the script.
- N37-18's table has no aggregate exception; were the door ever changed to admit one, the scorecard row of the
  matrix would change by a dated amendment of this bead, not silently.

## References

Appendix G (preamble), G.1, G.2; §19.9; §37; FROB-REF-310, FROB-REF-731, FROB-REF-733; NEG-16; N19b-02; N37-18.
