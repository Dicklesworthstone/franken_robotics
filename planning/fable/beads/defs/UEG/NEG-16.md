---
key: "NEG-16"
title: "e2e: `scripts/e2e/assets_licences.sh`: reference robots and scene-asset sources validated and licence-checked"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "habitat", "safety", "frob-io", "solid", "test", "e2e"]
parent: "WP-08"
deps: ["BD-022", "BD-066", "NEG-15", "NEG-32", "ROBOT-a1", "ROBOT-anymal-b", "ROBOT-anymal-c", "ROBOT-apollo", "ROBOT-barkour", "ROBOT-booster-t1", "ROBOT-fourier-n1", "ROBOT-fsim-g1", "ROBOT-g1", "ROBOT-go1", "ROBOT-go2", "ROBOT-h1", "ROBOT-op3", "ROBOT-spot", "ROBOT-stretch3", "ROBOT-talos", "ROBOT-tiago", "STD-E2E"]
needs: []
related: ["NEGp1-51", "NEGp1-54", "NEG-17", "NEG-18", "NEG-19", "NEG-20", "NEG-21", "NEG-22", "NEG-23", "NEG-24", "NEG-25", "NEG-26", "NEG-27"]
gates: []
plan_refs: ["Appendix G (preamble)", "Appendix G.1", "Appendix G.2", "§19.9", "§37"]
---
## Background

The project redistributes some third-party material (reference robot models under BSD-3, MIT and Apache-2.0;
later, scene assets under BSD-3, CC BY 4.0 and MIT) and must never redistribute other material (research-only
scene collections that users may supply themselves). Licences on file are facts from one research pass and
are re-checked at admission; an asset whose licence was not read stays outside the default pipeline. A user or
reviewer needs one command that answers: is every asset in the tree what its manifest says, under the licence
it says, validated, and would any export of it break a term? This script is that command. It is the
end-to-end test of the asset-and-licence capability; kinematic parity of robot imports against the oracle is
measured by the import-and-export script of the design-language unit and is not repeated here.

## Scope

In: the script, its loops over `assets/robots/` and over the scene-asset source register (the loop's frame:
reading the register, the per-class dispatch, and logging every source `not run` before its milestone), its
assertions and log, and the M1 run over the seventeen robots due at M1 (every reference robot except
`cassie` and `iiwa7`, which land at M3), each a dependency. Out: the M3 run, which adds `cassie`, `iiwa7`,
every scene-asset source due at M3 and the fourth self-test mutation, and needs the M3 pack check, export door
and import verb (NEGp1-51, which depends on this bead); the M5 sources and the scan profile (NEGp1-54); the manifests and goldens (the `ROBOT-…` beads and the scene-asset beads); the shared checks
(NEG-15); the scene-side licence manifest, register and pack-time refusal (owned by the scene unit).

## Technical approach

**Script.** `scripts/e2e/assets_licences.sh [--milestone Mn] [--only <name>] [--offline]`; plain shell;
sources `scripts/e2e/lib.sh`; idempotent; works in a scratch directory under `target/`; never fetches unless
`--fetch` is given (default is offline against vendored files and recorded hashes).

**Robot loop (from `assets/robots/INDEX.toml`; entries whose `first_milestone` is later are logged
`not run`).**

0. Class rows, once before the loop: NEG-15's index check `robot_index_check` (run through NEG-15's tests named
   `robot_index_…`, logged as one command with its exit code) on the index under test: eighteen class rows, each
   logged with its archetype and either its reference robots, its pending robots with their milestone, or
   `reference: none` with its reason, its revisit condition and its decision identifier where one is recorded.
   At M1 the run expects five classes with a reference on file, `biped-point` pending (cassie, M3) and twelve
   `none`.
1. Licence check: hash of `LICENSE` against the manifest; identifier on the admitted list; NOTICE present
   where required; `mark` recorded.
2. Import by NEG-15's import recipe: `frob design import assets/robots/<name>/source/<entry> --format mjcf
   --name <name> --key asset-<name>-<source hash> --json` for file-based robots; for the two sibling-catalogue
   robots, `frob design import --format fsim-catalog --builder <g1|iiwa7> --name <name> --key
   asset-<name>-<sibling commit> --json` through the airlock (the format of NEG-32; the G1 with `--reduced
   g1-reduced-30`). Assert the `DesignId` equals the pinned one, `frob design show <DesignId> --section term
   --json` equals `import.fbot` byte for byte, a second call with the same key is an idempotent replay, and
   the loss report equals its golden (`--strict` must fail exactly when the golden loss report is non-empty).
3. Validation: counts, exact inertia consistency, joint limits, total mass; compare `validation.json`.
4. Role rules: a parity-only robot is absent from the mass-gate table; a `reduced` profile is refused as a
   parity reference; no artifact produced in the loop carries claim Reality.
5. Export probe: `frob design export <DesignId> --format urdf --json` of each import must carry the three
   no-authorization flags, all false, and its reverse loss report, and the robot's manifest must carry its
   licence entry in the form the licence join reads (NEG-15 rule 5). That the export itself and a `synthetic`
   report citing the robot carry the licence entry and notices needs the join over lineage and the export door
   (N37-17, N37-18, M3): at M1 that assertion is logged `not run` with its milestone, and NEGp1-51 runs it.

**Scene-asset loop (from M3; from the asset-source register; its assertions are run by NEGp1-51 at M3 and by
NEGp1-54 at M5, and at M1 every source is logged `not run` with its milestone).** For each source: its class (`default`,
`offline-generator`, `reference-only`, `user-supplied-only`, `not-used`, `not-read`); for redistributable
classes the licence hash and attribution text; for `user-supplied-only` a `synthetic` stand-in asset tagged
with that source's licence class is put in a scratch pack and `frob scene compile` then an export are
attempted: the pack export must be refused with FROB-REF-310 by the pack export check (exit 3), and a still
or report of the same stand-in must be refused at the export door of N37-18 with FROB-REF-731 (under the
private root) or FROB-REF-733 (licence set), exit 5: one code per request, the division DESIGNed in N19b-02; the starter pack is scanned and must contain only synthetic and
redistributable fixtures; for `not-read` sources the script asserts that no file from them exists in the tree
and that the default pipeline lists them as excluded.

**Negative controls built into the run (`--self-test`).** A scratch copy with (a) one byte changed in a
`LICENSE`; (b) a robot's `licence` set to `not-read`; (c) a parity-only robot inserted into the mass table;
(d) a research-only stand-in placed in the starter pack. Expected: (a) fail; (b) the robot is skipped and
listed; (c) fail; (d) FROB-REF-310. Mutation (d) needs the pack export check (M3): at M1 it is logged
`not run` with that reason, never as caught, and NEGp1-51 runs it.

**Logging.** Newline-delimited JSON at `target/e2e-logs/assets_licences/<utc>.ndjson` with a transcript. Every
line: `ts`, `script`, `bead` (the asset's key), `step`, `event`, the exact command, exit code, wall time, the
envelope's `evidence` block (claim Parity or Known answer, class Tested, tier static, determinism D0,
identities), source collection and commit, licence identifier, text hash, mark, role, counts, total mass in
kg, `DesignId`, loss-report digest, refusal code where expected, and on failure the first differing field and
the paths needed to reproduce. The `summary` line: assets passed, failed, skipped (`not-read`) and not run;
archetype classes with a reference, pending and `none`; licence classes seen; refusals observed; `gates: []`. Logs contain identifiers and hashes, never geometry.

## Acceptance criteria

- [ ] At M1 the script passes for the seventeen robots due at M1 and logs `cassie`, `iiwa7`, every
      scene-asset source and the licence-entry-on-export assertion of step 5 `not run` with its milestone; the
      M3 and M5 runs are NEGp1-51's and NEGp1-54's.
- [ ] Self-test mutations (a) to (c) produce their expected outcomes at M1, and (d) is logged `not run`.
- [ ] Step 0 logs the eighteen class rows (five with a reference on file, one pending, twelve `none` at M1), and
      a scratch index whose `hexapod-radial` row has `reference = "none"` and no revisit condition fails the run
      at step 0, naming the row.
- [ ] Offline mode needs no network; `--fetch` verifies every fetched file against recorded hashes and
      refuses on a mismatch.
- [ ] No log line contains mesh data or scene geometry.

## Test plan

### Unit tests

- `e2e_assets_index_covers_directories`: every directory under `assets/robots/` is in the index and vice
  versa.
- `e2e_assets_skip_is_not_pass`: `not-read` and `not run` are counted separately from passes.
- `e2e_assets_log_schema`; `e2e_assets_log_has_no_geometry` (a scan for numeric arrays longer than a bound).
- `e2e_assets_milestone_gate`: with `--milestone M1` the M3 robots, every scene-asset source and mutation (d)
  are logged `not run` with their milestone and counted apart from passes; with a planted entry whose
  `first_milestone` is M1 but whose manifest is missing the run fails.
- `e2e_assets_robot_deps_match_index`: the robots due at M1 in `assets/robots/INDEX.toml` are exactly the
  seventeen this bead depends on.

### End-to-end script and logging

This bead is the script; `--self-test` is its own negative-control run.

### Known answers and negative controls

No numeric known answers: the goldens are the assets' own files. Controls: the four mutations. Metamorphic:
`--only <name>` yields the same lines for that asset as the full run. Determinism: two runs produce identical
logs apart from timestamps and wall times.

## Considerations

- The verb form for constructing a design from a sibling builder is the `fsim-catalog` format of
  `design.import` (NEG-32), which adds the format to the verb registry row; the unit test that ties the
  script's commands to the registry catches any drift.
- Licence checking here is a consistency check against recorded readings; it is not legal advice, and the
  no-authorization flags on every export say so.

## References

Appendix G (preamble), G.1, G.2; §19.9; §37.
