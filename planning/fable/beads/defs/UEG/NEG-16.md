---
key: "NEG-16"
title: "e2e: `scripts/e2e/assets_licences.sh`: reference robots and scene-asset sources validated and licence-checked"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "habitat", "safety", "frob-io", "solid", "test", "e2e"]
parent: "WP-08"
deps: ["STD-E2E", "NEG-15", "BD-066", "BD-022", "ROBOT-go2", "ROBOT-g1", "ROBOT-fsim-g1", "ROBOT-stretch3", "ROBOT-tiago"]
needs: []
related: ["N14b-24", "N19b-02", "NEG-17", "NEG-27", "BD-305", "BD-303"]
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

In: the script, its loops over `assets/robots/` and over the scene-asset source register, its assertions and
log. Out: the manifests and goldens (the `ROBOT-…` beads and the scene-asset beads); the shared checks
(NEG-15); the scene-side licence manifest, register and pack-time refusal (owned by the scene unit).

## Technical approach

**Script.** `scripts/e2e/assets_licences.sh [--milestone Mn] [--only <name>] [--offline]`; plain shell;
sources `scripts/e2e/lib.sh`; idempotent; works in a scratch directory under `target/`; never fetches unless
`--fetch` is given (default is offline against vendored files and recorded hashes).

**Robot loop (from `assets/robots/INDEX.toml`; entries whose `first_milestone` is later are logged
`not run`).**

1. Licence check: hash of `LICENSE` against the manifest; identifier on the admitted list; NOTICE present
   where required; `mark` recorded.
2. Import: `frob design import --format mjcf … --json` for file-based robots; for the two sibling-catalogue
   robots, `frob design new --from-sibling <builder> --json` through the airlock. Assert the `DesignId` equals
   the pinned one and the loss report equals its golden (`--strict` must fail exactly when the golden loss
   report is non-empty).
3. Validation: counts, exact inertia consistency, joint limits, total mass; compare `validation.json`.
4. Role rules: a parity-only robot is absent from the mass-gate table; a `reduced` profile is refused as a
   parity reference; no artifact produced in the loop carries claim Reality.
5. Export probe: `frob design export … --json` of each import and a `synthetic` report citing it must carry
   the licence entry and the three no-authorization flags, all false.

**Scene-asset loop (from M3; from the asset-source register).** For each source: its class (`default`,
`offline-generator`, `reference-only`, `user-supplied-only`, `not-used`, `not-read`); for redistributable
classes the licence hash and attribution text; for `user-supplied-only` a `synthetic` stand-in asset tagged
with that source's licence class is put in a scratch pack and `frob scene compile` then an export are
attempted: the export must be refused with FROB-REF-310 (a pack or report would export a research-only or
private asset) and exit non-zero; the starter pack is scanned and must contain only synthetic and
redistributable fixtures; for `not-read` sources the script asserts that no file from them exists in the tree
and that the default pipeline lists them as excluded.

**Negative controls built into the run (`--self-test`).** A scratch copy with (a) one byte changed in a
`LICENSE`; (b) a robot's `licence` set to `not-read`; (c) a parity-only robot inserted into the mass table;
(d) a research-only stand-in placed in the starter pack. Expected: (a) fail; (b) the robot is skipped and
listed; (c) fail; (d) FROB-REF-310.

**Logging.** Newline-delimited JSON at `target/e2e-logs/assets_licences/<utc>.ndjson` with a transcript. Every
line: `ts`, `script`, `bead` (the asset's key), `step`, `event`, the exact command, exit code, wall time, the
envelope's `evidence` block (claim Parity or Known answer, class Tested, tier static, determinism D0,
identities), source collection and commit, licence identifier, text hash, mark, role, counts, total mass in
kg, `DesignId`, loss-report digest, refusal code where expected, and on failure the first differing field and
the paths needed to reproduce. The `summary` line: assets passed, failed, skipped (`not-read`) and not run;
licence classes seen; refusals observed; `gates: []`. Logs contain identifiers and hashes, never geometry.

## Acceptance criteria

- [ ] At M1 the script passes for the robots due at M1 and logs the rest `not run`; at M3 it passes for all
      nineteen and for every scene-asset source.
- [ ] The four self-test mutations produce the four expected outcomes.
- [ ] Offline mode needs no network; `--fetch` verifies every fetched file against recorded hashes and
      refuses on a mismatch.
- [ ] No log line contains mesh data or scene geometry.

## Test plan

### Unit tests

- `e2e_assets_index_covers_directories`: every directory under `assets/robots/` is in the index and vice
  versa.
- `e2e_assets_skip_is_not_pass`: `not-read` and `not run` are counted separately from passes.
- `e2e_assets_log_schema`; `e2e_assets_log_has_no_geometry` (a scan for numeric arrays longer than a bound).

### End-to-end script and logging

This bead is the script; `--self-test` is its own negative-control run.

### Known answers and negative controls

No numeric known answers: the goldens are the assets' own files. Controls: the four mutations. Metamorphic:
`--only <name>` yields the same lines for that asset as the full run. Determinism: two runs produce identical
logs apart from timestamps and wall times.

## Considerations

- The verb form for constructing a design from a sibling builder is this bead's assumption about the surface;
  if the verb registry names it differently the script follows the registry and the unit test that ties
  commands to the registry catches the drift.
- Licence checking here is a consistency check against recorded readings; it is not legal advice, and the
  no-authorization flags on every export say so.

## References

Appendix G (preamble), G.1, G.2; §19.9; §37.
