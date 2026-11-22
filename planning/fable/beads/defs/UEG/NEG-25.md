---
key: "NEG-25"
title: "scene asset source `behavior-1k` (its own agreement): not used; supported only as a user-supplied import"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["N19b-02", "BD-305", "NEG-22"]
needs: []
related: ["NEG-23", "NEG-24", "NEG-26", "NEG-16"]
gates: []
plan_refs: ["Appendix G.2 (row 9)", "§19.9", "§37", "FROB-REF-310"]
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

**Generic user-supplied import.** `frob scene import --source behavior-1k --user-supplied --licence-ack
<path-to-user's-copy-of-the-agreement> <asset-or-scene> --json`. The generic profile: decode supported
geometry formats; treat everything as static geometry unless the file carries a joint description the
generic reader understands; `assumed` materials; scale label `user-declared` or refusal; a loss report that
lists every unsupported element; licence entries with class `proprietary`, redistribution forbidden, and the
hash of the agreement copy the user supplied; pack marked `private`.

**Containment.** Identical in mechanism to research-only sources and one step stricter in effect: because
the class is `proprietary`, any artifact whose licence join includes it is refused for export (FROB-REF-310)
including aggregate results unless the user sets an explicit, logged override for aggregates only (a
scorecard without geometry); the override is recorded in the ledger with the user's capability and never
applies to geometry, stills or packs.

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
- [ ] Every export of geometry, stills or packs is refused with FROB-REF-310; an aggregate export needs the
      explicit override and records it.

## Test plan

### Unit tests

- `asset_source_behavior1k_row_valid`.
- `behavior1k_absent_from_tree`, `behavior1k_no_profile`, `behavior1k_not_in_any_battery`.
- `behavior1k_generic_import_private_proprietary`.
- `behavior1k_export_refused_310`.
- `behavior1k_aggregate_needs_logged_override`.
- `licence_join_order`: the join of {attribution, proprietary} is proprietary; of {proprietary, unknown} is
  unknown.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop at M3, on the stand-in: the absence scan, the
generic import, the export attempts with and without the aggregate override; log lines carry class, refusal
codes and the override record.

### Known answers and negative controls

The join order of licence classes. Controls: the source added to a battery; a bespoke profile added without
admission; an override that leaks a still. Determinism: refusals and scans are deterministic.

## Considerations

- Classing a bespoke agreement as `proprietary` may be stricter than its text; relaxing it requires reading
  the agreement and recording the mapping, which is an admission task nobody has asked for.
- "Supported" here means "not obstructed", not "tested for quality": the generic profile's loss report may be
  long.

## References

Appendix G.2 (row 9); §19.9; §37; FROB-REF-310.
