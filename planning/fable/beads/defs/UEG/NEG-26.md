---
key: "NEG-26"
title: "scene asset source `objaverse-xl` (per-object licences): not used by default; one licence entry per object"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["N19b-02", "BD-305", "BD-307"]
needs: []
related: ["NEG-19", "NEG-25", "NEG-16"]
gates: []
plan_refs: ["Appendix G.2 (row 10)", "§19.9", "§37", "FROB-REF-310"]
---
## Background

Objaverse-XL is a published index of a very large number of 3D objects gathered from many hosts. It has no
single licence: each object carries the licence its uploader chose (FACT from the research pass of
2026-11-21; re-checked at admission), which may be permissive, attribution-requiring, non-commercial,
restricted in other ways, or absent. Its row in the asset table reads "not used by default". A source with a
licence per object cannot be admitted as a whole. It can only be admitted object by object, and the unit of
the licence manifest is already the asset, so the mechanism exists; what this bead fixes is the policy:
nothing from this source enters the default pipeline, every object a user imports carries its own entry with
its own class, an object without a clear licence is `unknown`, and `unknown` is the most restrictive class.

## Scope

In: the register row; the per-object import path and its licence-resolution rule; the mixed-pack behaviour;
the importer-safety note for uncurated meshes. Out: the licence machinery (N19b-02); private store (BD-305);
importer limits and fuzzing (BD-307).

## Technical approach

**Register row.** `key = "objaverse-xl"`, `licence = "per-object"`, `class = "per-object"` (resolved at
import; never used as a class of a pack), `default_pipeline = false`, `redistribute = "per-object"`,
`kind = "object-index"`, `use = "not used by default"`.

**Per-object import.** `frob scene import --source objaverse-xl --user-supplied --object-licence <SPDX or
"unknown"> --object-origin <URL> <file> --json`. The user states the object's licence and origin; the import
records both, with the date, in the licence entry. Resolution of the class from the stated licence:

| Stated licence | Class | Redistribution |
|---|---|---|
| A public-domain dedication or a permissive licence without attribution duty | `redistributable` | Allowed |
| An attribution licence (for example CC BY 4.0) | `attribution` | Allowed with notice; the user supplies the attribution text or the import refuses |
| Any non-commercial or no-derivatives licence | `research-only` | Forbidden |
| A bespoke or platform-specific licence | `proprietary` | Forbidden |
| None stated, or "unknown" | `unknown` | Forbidden |

A no-derivatives term is treated as forbidding the import's own collision simplification from being
redistributed, hence `research-only`. The project does not verify the user's statement against the origin;
the entry's `verified` flag is false and exports that rely on it carry the standard no-authorization flags.

**Mixed packs.** A pack's class is the join of its entries, so one `unknown` object makes the whole pack
unexportable (FROB-REF-310 names the object). `frob scene show --json` lists the entries sorted by class so
that the user sees which object blocks an export and can remove it.

**Safety of uncurated meshes.** Objects from an open index are hostile input until proven otherwise: the
import runs under the importer's declared limits (file size, vertex and face counts, nesting, embedded
resources refused, external references not followed), never executes embedded scripts, and treats embedded
text (names, descriptions) as untrusted data marked as such wherever it is echoed.

**No bulk path.** There is no command that imports by query or by list from the index; each object is an
explicit act with its own stated licence. No selection list of this source is shipped.

## Acceptance criteria

- [ ] The register row exists with `default_pipeline = false` and per-object resolution.
- [ ] Each row of the resolution table is exercised by a `synthetic` object and yields the stated class and
      redistribution.
- [ ] A pack with one `unknown` object is refused for export with the object named; removing it allows the
      export of the rest with their notices.
- [ ] An import without a stated licence defaults to `unknown`; an attribution licence without attribution
      text is refused.
- [ ] Oversized and reference-following fixtures are refused by the importer limits; embedded text is marked
      untrusted on output.

## Test plan

### Unit tests

- `asset_source_objaverse_row_valid`.
- `objaverse_class_resolution_table`: five cases.
- `objaverse_mixed_pack_blocked_by_unknown`, `objaverse_mixed_pack_exports_after_removal`.
- `objaverse_attribution_text_required`.
- `objaverse_no_bulk_import_command`: the verb registry has no query or list form for this source.
- `objaverse_importer_limits_and_untrusted_text`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (NEG-16), scene-asset loop at M3: five `synthetic` objects with the five
stated licences, a mixed pack, its refused export, removal of the blocker and a successful export; log lines
carry object hashes, stated licence, class, `verified = false`, refusal codes.

### Known answers and negative controls

The join order of licence classes and the rule that `unknown` is most restrictive. Controls: an object with
no stated licence classed as redistributable; a bulk import path; embedded text echoed without its untrusted
mark. Determinism: resolution is a pure function of the stated licence.

## Considerations

- The stated licence is the user's assertion; an index's metadata can be wrong, and the project does not
  launder that uncertainty into a verified field.
- If a curated, uniformly licensed subset is ever wanted, it is a new source with its own row and admission
  reading, not a flag on this one.

## References

Appendix G.2 (row 10); §19.9; §37; FROB-REF-310.
