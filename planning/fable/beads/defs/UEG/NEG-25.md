---
key: "NEG-25"
title: "scene asset source `behavior-1k` (its own agreement): not used; supported only as a user-supplied import"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["BD-305", "N19a-46", "N19b-02", "N37-18", "NEG-22"]
needs: []
related: ["NEG-26", "NEGp1-51"]
gates: []
plan_refs: ["Appendix G.2 (row 9)", "§19.9", "§37", "FROB-REF-310", "FROB-REF-731", "FROB-REF-733"]
---
## Background

BEHAVIOR-1K is a published benchmark of household activities with a large asset set, distributed under its
own agreement (FACT from the research pass of 2026-11-21; re-checked at admission). Its row in the asset
table reads "not used; supported only as a user-supplied import". The distinction from the other research-
only collections is deliberate. For those, the project names a use (a user evaluating in furnished homes, in
scanned buildings, with articulated objects). For this one it names none: the project does not use it, does
not test against it, and has not designed an import profile around its content. The only commitment is that
a user who holds it under its agreement is not blocked: the generic user-supplied import path works, with
the most restrictive handling. A bespoke agreement, as opposed to a standard public licence, cannot be
classified by an identifier, so its terms are whatever the user's copy says and the project's classification
is conservative.

## Scope

In: the register row; the generic user-supplied path applied to this source; containment under class
`proprietary`; the "not used" checks. Out: the licence machinery (N19b-02); private store and purge (BD-305);
the research-only containment rules stated in full by the synthetic-home source's bead (NEG-22).

## Technical approach

**Register row.** `key = "behavior-1k"`, `licence = "custom-agreement"`, `class = "proprietary"` (a bespoke
agreement is classed above `research-only` in the join order redistributable < attribution < research-only <
proprietary < unknown, because the project has not mapped its terms to a standard class),
`default_pipeline = false`, `redistribute = "never"`, `kind = "benchmark-asset-set"`,
`use = "not used; user-supplied import only"`, `profile = "generic"`.

**Generic user-supplied import.** `frob scene import --kind asset <asset-or-scene> --source behavior-1k
--user-supplied --licence-ack <path-to-user's-copy-of-the-agreement> --profile generic --key <k> --json` (the
verb and its inputs are N19a-46's; a `proprietary` row requires `--user-supplied` with `--licence-ack`). The
generic profile: decode supported
geometry formats; treat everything as static geometry unless the file carries a joint description the
generic reader understands; `assumed` materials; N19a-46's scale declaration (the unit the format states, or the
caller's `--unit` recorded as `user-declared`; a file with neither is refused, not guessed; a declaration, not a
scale label: from M5 it maps to `estimated`, never `witnessed`, N19c-02); a loss report that
lists every unsupported element; licence entries with class `proprietary`, redistribution forbidden, and the
hash of the agreement copy the user supplied; pack marked `private`.

**Containment.** Identical in mechanism to research-only sources and one step stricter in effect: because
the class is `proprietary`, any artifact whose licence join includes it is refused for export, one code per
request as N19b-02 divides them: a pack or scene variant by the pack export check with FROB-REF-310 (exit 3);
every other artifact (a still, a report, a replay, a run bundle, a scorecard) at the export door of N37-18,
exit 5, with FROB-REF-731 while the private mark applies and FROB-REF-733 for the licence set in a session
holding `private:export` and the flag. Aggregate results are refused like the rest: an earlier draft of this
bead allowed a scorecard without geometry under a logged user override, which the export door has no path for
and the plan does not state; it is not built here or anywhere (N37-18's table has no aggregate exception, and
any such override would be a change to the door, never for geometry, stills or packs).

**"Not used" checks.**

1. `behavior1k_absent_from_tree`: no file, fixture, selection list or documentation example in the repository
   derives from the source.
2. `behavior1k_no_profile`: no import profile other than `generic` names the source; adding one requires a
   new bead and an admission reading.
3. `behavior1k_not_in_any_battery`: no registered battery, task family or scorecard references it.

**Stand-in.** A single static object authored in the repository, tagged `behavior-1k` in a scratch register.

## Acceptance criteria

- [ ] The register row exists with class `proprietary`, `default_pipeline = false`, `use` as stated.
- [ ] The three "not used" checks pass and each fails on its planted violation.
- [ ] A user-supplied import of the stand-in works through the generic profile and yields a private pack.
- [ ] Exports of the pack and of a variant are refused with FROB-REF-310 (exit 3); a still, a report and a
      scorecard over runs that used it are refused at the door with FROB-REF-731 (exit 5), or FROB-REF-733 with
      `private:export` and the flag; each request gets exactly one code.

## Test plan

### Unit tests

- `asset_source_behavior1k_row_valid`.
- `behavior1k_absent_from_tree`, `behavior1k_no_profile`, `behavior1k_not_in_any_battery`.
- `behavior1k_generic_import_private_proprietary`.
- `behavior1k_pack_export_refused_310`, `behavior1k_variant_export_refused_310`.
- `behavior1k_still_report_scorecard_refused_at_door`: 731 by default, 733 with `private:export` and the flag.
- `behavior1k_no_aggregate_bypass`: no flag or option of `scene` or `report` verbs exports a scorecard whose
  lineage reaches the import.
- `licence_join_order`: the join of {attribution, proprietary} is proprietary; of {proprietary, unknown} is
  unknown.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M3 scene-asset loop, owned and run by NEGp1-51, which depends on this
bead; this bead lists the steps of its row), on the stand-in, every `run` verb call with its own `--key`: the
absence scan; the import without `--user-supplied` (`--key al-b1k-1`; refused by the default-pipeline filter);
the generic import as written above (`--key al-b1k-2`; a private pack, class `proprietary`); the pack, variant,
still, report and scorecard export attempts, each `frob --json artifact export <ref> --out exports/ --key
al-b1k-x<n>` (a distinct key per attempt), with and without `private:export` and the flag. Log lines carry `ts`, `script`, `bead: "NEG-25"`, `step`, `event`, the
exact command, exit code, wall time, the `evidence` block, class, each refusal code with its exit code and
raising site, and the licence join of each refused artifact.

### Known answers and negative controls

The join order of licence classes. Controls: the source added to a battery; a bespoke profile added without
admission; an override that leaks a still. Determinism: refusals and scans are deterministic.

## Considerations

- Classing a bespoke agreement as `proprietary` may be stricter than its text; relaxing it requires reading
  the agreement and recording the mapping, which is an admission task nobody has asked for.
- "Supported" here means "not obstructed", not "tested for quality": the generic profile's loss report may be
  long.

## References

Appendix G.2 (row 9); §19.9; §37; FROB-REF-310, FROB-REF-731, FROB-REF-733.
