---
key: "NEG-22"
title: "scene asset source `hssd` (CC BY-NC 4.0): user-supplied import only, never redistributed"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["BD-305", "N19a-05", "N19a-46", "N19b-02", "N37-18", "NAB-13"]
needs: []
related: ["NEGp1-51"]
gates: []
plan_refs: ["Appendix G.2 (row 6)", "§19.9", "§37", "FROB-REF-310", "FROB-REF-731", "FROB-REF-733", "Appendix H.3"]
---
## Background

HSSD is a published collection of furnished synthetic home scenes built for embodied-AI research. It is on
file under CC BY-NC 4.0 (FACT from the research pass of 2026-11-21; the dataset's page is in the source
register and is re-read at admission). The non-commercial term decides its role: the project never ships it,
never includes it in a pack that leaves the user's machine, and never draws from it in the default pipeline.
What the project does support is a user who has obtained the collection themselves under its terms and wants
to evaluate a robot in those homes: a user-supplied import. The scenes then live in the user's private store,
carry a licence entry of class `research-only` with redistribution forbidden, and every derived artifact
inherits that class, so a pack that would export them is refused at pack time (FROB-REF-310) and any other
export whose lineage reaches them is refused at the export door (FROB-REF-731 or FROB-REF-733), one code per
request as N19b-02 divides them.

## Scope

In: the register row; the user-supplied import profile for whole furnished home scenes; the containment
checks; a `synthetic` stand-in used by tests (the project's tests never contain real HSSD data). Out: the
licence manifest, join and pack-time refusal (N19b-02); the private store, path jail and purge (BD-305);
train and test hygiene for scene groups (BD-235).

## Technical approach

**Register row.** `key = "hssd"`, `licence = "CC-BY-NC-4.0"`, `class = "research-only"`,
`default_pipeline = false`, `redistribute = "never"`, `kind = "scene-collection"`,
`use = "user-supplied import only"`, `read` set by the admission reading (the licence text's hash and date).

**Import.** `frob scene import --kind asset <scene> --source hssd --user-supplied --licence-ack
<path-to-user's-copy-of-terms> --profile scene-collection --key <k> --json` (the verb and its inputs are
N19a-46's: `kind` is required, a `run` verb takes an idempotency key, and `--user-supplied` with
`--licence-ack` is what a research-only row requires; this bead adds the import profile `scene-collection`,
which admits a directory holding one furnished scene with its object files and raises the decode limits of
`asset-default` to the profile's own, recorded in `registries/import-profiles.toml`). Without `--user-supplied`
the default-pipeline filter refuses; without `--licence-ack` the request is `FROB-REF-802`. The import: decodes the scene
(structure and objects); compiles it to a private pack under the user's private root; writes one licence
entry per asset with `source = "hssd"`, class `research-only`, `redistribution = Forbidden`; marks the pack
`private`; assigns interface and optical classes (`assumed`); records N19a-46's scale declaration in the loss
report (the unit the scene's format states, or the caller's `--unit` recorded as declared by the caller; a scene
that states no unit and is given none is refused, not guessed); emits a loss report. A declaration is not a
measurement: from M5 it maps to the scale label `estimated` with the declared accuracy (±5% when none is
declared), never to `witnessed` (N19c-02), so a metric task bound to the pack is refused with FROB-REF-304.

**Containment checks.**

1. Export of the pack or of any scene variant derived from it: refused by the pack export check with
   FROB-REF-310 naming the assets and the source, exit 3. Repairs: remove the asset; keep the pack local.
2. Every other export: a still, report, replay, run bundle or scorecard whose lineage reaches it is refused at the export door of
N37-18, exit 5, with FROB-REF-731 while the private mark applies (the door checks the mark before the licence set)
and with FROB-REF-733 for the licence set in a session holding `private:export` and the flag; the refusal carries the lineage path to the root asset. A still
   rendered from the scene is itself a derived asset of class `research-only`. Aggregated results (a scorecard,
   a verdict) are no exception: the door's table refuses a scorecard whose runs touched the scene (N37-18), and
   this bead adds no aggregate path of its own (a request to N37-18 asks whether one is wanted).
3. Logs and telemetry never contain the scene's geometry (the general rule for private scenes).
4. `frob scene purge <pack>` is a dry run by default (the lineage closure of the pack and its digest, nothing
   removed); `frob scene purge <pack> --dry-run=false --confirm <digest>` removes the pack and everything derived
   from it, leaving tombstones (N37-19's spellings in VERBS-scene: an `admin` verb, no key, `private:read` needed).
5. The scenes of one home, and its edited or mirrored variants, form one group for train and test splits.

**Stand-in.** `tests/fixtures/synthetic/hssd-like/`: a small furnished room authored in this repository,
tagged with source key `hssd` in a scratch register so that tests exercise the research-only path without any
real data.

## Acceptance criteria

- [ ] The register row exists with class `research-only` and `redistribute = "never"`.
- [ ] A default-mode import is refused; a user-supplied import of the stand-in yields a private pack whose
      entries forbid redistribution.
- [ ] Export of the pack and of a derived variant are refused with FROB-REF-310 (exit 3); a still, a report
      containing a still and a scorecard over runs in the scene are refused at the door with FROB-REF-731
      (exit 5), and with FROB-REF-733 in a session holding `private:export` and the flag; each request gets
      exactly one code.
- [ ] No real HSSD file exists in the repository or in any test fixture (scan by source key and by the
      absence of that host's file naming pattern).

## Test plan

### Unit tests

- `asset_source_hssd_row_valid`.
- `hssd_default_import_refused`, `hssd_user_supplied_import_is_private_research_only`.
- `hssd_export_pack_refused_310`, `hssd_export_variant_refused_310`: exit 3, raised by the pack check.
- `hssd_export_still_refused_731_at_door`, `hssd_export_report_refused_731_at_door`,
  `hssd_scorecard_refused_at_door`: exit 5; with `private:export` and the flag the same three get 733.
- `hssd_one_code_per_request`: no request yields both 310 and a door code.
- `hssd_purge_leaves_tombstones_only`.
- `hssd_no_real_data_in_tree`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M3 scene-asset loop, owned and run by NEGp1-51, which depends on this
bead; this bead lists the steps of its row), on the stand-in, every `run` verb call with its own `--key`: the
import without `--user-supplied` (`--key al-hssd-1`; refused by the default-pipeline filter); the user-supplied
import as written above (`--key al-hssd-2`; a private pack under the per-run private root, entries
`research-only` and `Forbidden`, the acknowledgement's hash in each entry); `frob --json artifact export <pack>
--out exports/ --key al-hssd-3` and the same (`--key al-hssd-3v`) for a variant derived from the pack by a typed
edit (`frob --json scene compile --base <pack> --edits fixtures/hab/edits/threshold_ramp.json --key al-hssd-v`,
N19b-44's derived form) (`FROB-REF-310`, exit 3, each); after
`e2e_scene_ready <pack>` and a referee run in the scene (`frob --json sim run quad-a --scene <pack> --tier
referee --seconds 2 --key al-hssd-run`), the export of a still and of the run's report (`FROB-REF-731`, exit 5,
each; and `FROB-REF-733` in a session holding `private:export` with the flag); `frob --json scene purge <pack>`
(the dry run: closure and digest, nothing removed), then `frob --json scene purge <pack> --dry-run=false --confirm
<digest>` (tombstones only; the closure holds the variant and the run's artifacts). Log lines carry `ts`, `script`, `bead: "NEG-22"`, `step`, `event`, the exact command (the
acknowledgement path redacted as a private path), exit code, wall time, the `evidence` block, the source key,
class, each refusal code with its exit code and raising site, and no geometry: the harness's linter checks the
last.

### Known answers and negative controls

The safety gate's licence rule (the licence set of a derived artifact equals the join of its inputs'; a
research-only asset cannot enter an exported report or pack). Controls: a build whose export check ignores
derived variants; a still exported through the report path (it must still reach the door and be refused); a
build that raises 310 for a report (the wrong raising site). Determinism: refusals are deterministic.

## Considerations

- The user is responsible for holding the collection lawfully; the acknowledgement path records what they
  declared, and the no-authorization flags on every artifact say that the tool grants nothing.
- Non-commercial terms bind the user's use too; the project takes no position on what counts as commercial.

## References

Appendix G.2 (row 6); §19.9; §37; FROB-REF-310, FROB-REF-731, FROB-REF-733; Appendix H.3.
