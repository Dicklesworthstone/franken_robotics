---
key: "NEG-26"
title: "scene asset source `objaverse-xl` (per-object licences): not used by default; one licence entry per object"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "safety", "frob-scene", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["BD-305", "BD-307", "N19a-46", "N19b-02", "N37-18"]
needs: []
related: ["NEGp1-51"]
gates: []
plan_refs: ["Appendix G.2 (row 10)", "§19.9", "§37", "FROB-REF-310", "FROB-REF-731", "FROB-REF-733"]
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

**Per-object import.** `frob scene import --kind asset <file> --source objaverse-xl --user-supplied
--object-licence <SPDX or "unknown"> --object-origin <URL> [--attribution <text>] --profile asset-default
--behaviour <b> --key <k> --json` (the verb and its inputs are N19a-46's; the three per-object inputs are
admitted only for a row whose class is `per-object`). The user states the object's licence and origin; the import
records both, with the date, in the licence entry. Resolution of the class from the stated licence:

| Stated licence | Class | Redistribution |
|---|---|---|
| A public-domain dedication or a permissive licence without attribution duty | `redistributable` | Allowed |
| An attribution licence (for example CC BY 4.0) | `attribution` | Allowed with notice; the user supplies the attribution text or the import refuses (`FROB-REF-802` naming `--attribution`, exit 3, nothing written) |
| Any non-commercial or no-derivatives licence | `research-only` | Forbidden |
| A bespoke or platform-specific licence | `proprietary` | Forbidden |
| None stated, or "unknown" | `unknown` | Forbidden |

A no-derivatives term is treated as forbidding the import's own collision simplification from being
redistributed, hence `research-only`. The project does not verify the user's statement against the origin;
the entry's `verified` flag is false and exports that rely on it carry the standard no-authorization flags.

**Mixed packs.** A pack's class is the join of its entries, so one `unknown` object makes the whole pack
unexportable: the pack export check refuses with FROB-REF-310 naming the object (exit 3). Anything else whose
lineage reaches that object (a still, a report, a run bundle) is refused at the export door of N37-18, exit 5,
naming the lineage path to it. An object of class `unknown`, `research-only` or `proprietary` is imported only into
the private root (N19a-46, step 1), so the door, which checks the private mark before the licence set, answers
FROB-REF-731 by default and FROB-REF-733 (the licence set) only in a session holding `private:export` and the
flag; one code per request, as N19b-02 divides them. `frob scene show --json` lists the entries sorted by class so
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
- [ ] A mixed pack of the `redistributable`, the `attribution` and the `unknown` object is refused for export
      with FROB-REF-310 and the `unknown` object named, and a still of a scene containing it with FROB-REF-731 at
      the door (FROB-REF-733 in a session holding `private:export` and the flag), exit 5; removing the
      `unknown` object allows the export of the other two with the attribution notice (a pack that also held
      the `research-only` or `proprietary` object would stay refused).
- [ ] An import without a stated licence defaults to `unknown`; an attribution licence without attribution
      text is refused.
- [ ] Oversized and reference-following fixtures are refused by the importer limits; embedded text is marked
      untrusted on output.

## Test plan

### Unit tests

- `asset_source_objaverse_row_valid`.
- `objaverse_class_resolution_table`: five cases.
- `objaverse_mixed_pack_blocked_by_unknown` (310, exit 3), `objaverse_still_blocked_at_door` (731, exit 5; 733
  with `private:export` and the flag; the `unknown` object's entry lies under the private root),
  `objaverse_mixed_pack_exports_after_removal`.
- `objaverse_attribution_text_required`.
- `objaverse_no_bulk_import_command`: the verb registry has no query or list form for this source.
- `objaverse_importer_limits_and_untrusted_text`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M3 scene-asset loop, owned and run by NEGp1-51, which depends on this
bead; this bead lists the steps of its row), every `run` verb call with its own `--key`: five `synthetic`
objects imported with the five stated licences (the attribution one once without `--attribution`, refused, and
once with it), a mixed pack of the `redistributable`, `attribution` and `unknown` objects, its refused export
(`FROB-REF-310`, exit 3, the `unknown` object named), a still
of a run in the mixed scene (`FROB-REF-731` at the door, exit 5, the `unknown` object lying under the private
root; `FROB-REF-733` in a session holding `private:export` and the flag), removal of the blocker and a successful export with the notices. Log lines carry `ts`, `script`,
`bead: "NEG-26"`, `step`, `event`, the exact command, exit code, wall time, the `evidence` block, object
hashes, stated licence, class, `verified = false`, each refusal code with its exit code and raising site.

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

Appendix G.2 (row 10); §19.9; §37; FROB-REF-310, FROB-REF-731, FROB-REF-733.
