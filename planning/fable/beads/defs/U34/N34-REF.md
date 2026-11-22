---
key: "N34-REF"
title: "Workspace refusal-code registry: FROB-REF-891, -892, -893 (format version, view is not identity, owner bypass)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "workspace", "frob-core", "frob-verbs", "solid", "registry"]
parent: "EPIC-M1"
deps: ["BD-005", "BD-022", "N34-19"]
needs: []
related: ["N34-KA", "N34-CX", "N34-20", "N34-24", "N34-23", "BD-137", "BD-141", "BD-054"]
gates: []
plan_refs: ["§34.9 (the refusal table)", "§34.14", "Appendix O.8", "§32.4 (exit codes)", "§35.11", "§14.13"]
---
## Background

Every refusal of FrankenRobotics has a stable code, a message and ranked repairs. A refusal is not an error
message: it is a typed answer that says what was at fault, what was observed against what was required, which
claims are affected, whether a retry can help, and what to do next, so that an agent can repair without
guessing. The machine-readable registry is `registries/refusals.toml`; an unknown code is a load-time error and
codes are never reused. The envelope's `refusal` block carries `code`, `message`, `pointer` (the field or part
at fault), `observed`, `required`, `unit`, `affected_claims`, `retry` (`never`, `after-change`, `backoff`) and
ranked `repairs`. Three standing rules apply: refuse, never truncate; a budget stop publishes nothing partial as
a result; a misspelt parameter is a refusal, not a default.

The workspace chapter defines three codes, all in the `8xx` family (evidence and protocol), all about the format
registry: thirteen artifact formats, each with one owner crate and one canonical encoding, where an artifact's
identity is the hash of its canonical bytes and a view is never an identity. Exit code for all three: 3 (a
refused input or question; the `8xx` codes other than conflicts and integrity faults exit 3). This bead puts the
three rows in the registry with trigger, pointer, ranked repairs and exit code, binds each to the mechanism that
raises it and the test that exercises it, and records how two admission faults that have no code of their own
are reported. Tier: Solid.

## Scope

In: the three rows in `registries/refusals.toml`; one triggering fixture per code; the check that each code is
raised by the mechanism named; the interim mapping for the two uncoded admission faults.
Out: the mechanisms that raise the codes (N34-19 for the version gate, N34-24 for the verb boundary); refusal
codes of other chapters that sit beside these (the design chapter's own version and field codes, the envelope's
schema and integer codes, the stack-mismatch code of the suite-consumption chapter).

## Technical approach

| Code | Name | Meaning | Trigger (who raises it) | Pointer | Ranked repairs | Retry | Exit |
|---|---|---|---|---|---|---|---|
| FROB-REF-891 | `format-version-unsupported` | Unknown major version, or an unknown required field | The version gate that every owner's reader calls before interpreting any field: the artifact's `schema` member names a major version this build does not read; or the artifact uses a field the reader's schema does not list (a reader cannot know an unlisted field to be optional, so it refuses and never parses on a best-effort basis). Raised with this code by every owner whose chapter defines no code of its own for the condition; the design owner raises its own codes (FROB-REF-125 for the version, FROB-REF-103 for an unknown field) | The `schema` member, or the JSON Pointer of the unknown field | 1. Migrate the artifact with the owner's migration edit, which produces a new artifact with a link to the old one (the old identity stays valid for citing old evidence). 2. Use a build that reads this schema revision; `observed` and `required` name the found and supported versions. 3. If the field is a misspelling, correct it | `after-change` | 3 |
| FROB-REF-892 | `view-is-not-identity` | A view is offered where an identity is required: to hash, to sign, to cite as evidence, or as a cache key | The artifact-input stage of the verb layer, for a parameter of the identity-required kind: the argument is a registered view (for example the S-expression text of a design, an array-file export of a run, a bill of materials), or JSON that is not byte-for-byte canonical, or a reference that resolves to a stored view. In code the same rule is a missing method: a view type has no identity | The parameter; `observed` says `view:<kind>` or `non-canonical-spelling` | 1. Convert with the owner's view verb (for a design, `frob design view --as json`) and pass the canonical file or its identity. 2. Pass the identity printed in the `bound` field of the envelope that produced the artifact. There is no option to hash the view | `after-change` | 3 |
| FROB-REF-893 | `format-owner-bypass` | A file with a registered extension reaches a verb through a path that did not go through its owner's reader | The artifact-input stage of the verb layer: a file with a registered extension, or JSON whose top-level `schema` member is a registered schema name, arrives in a foreign-file parameter or as an opaque blob reference; and any internal call that would hand raw bytes of a registered format to a verb executor | The parameter and the file; `observed` names the registered format | 1. Pass the file to the parameter of its own format, so that its owner's reader admits it. 2. If a conversion was intended, use the owner's import or view verb. 3. If the file is genuinely foreign and merely shares an extension, rename it | `after-change` | 3 |

Each row in `registries/refusals.toml` carries `code`, `name`, `message`, `defined_in`, `raised_by` (crate and
function group), `pointer_kind`, `repairs` (ordered), `retry`, `exit`, `fixture` (path of a triggering input) and
`affected_claims`. Affected claims: every claim kind that would have consumed the artifact (a design that cannot
be read supports no Known answer, Fidelity or Design comparison result; a view offered for citation would have
supported a Proof or a Design comparison). `affected_claims` is filled per occurrence from the verb's declared
outputs.

**Counterexamples the three codes stand on.** For 891: a reader that ignores an unknown required field such as a
contact-law selector would simulate a different robot from the one described and report it under the same
identity. For 892: the S-expression and the JSON of one design are different bytes; a cache keyed on whichever
arrived first would hold two entries for one design and two verdicts a reader could cite as independent. For
893: a second reading of a design, by code other than its owner's reader, is a second opinion about which robot
the file describes.

**Two admission faults without a code.** The canonical reader refuses a member name that repeats after escape
decoding and an identifier that is empty or not printable, before a job is planned or a budget is charged. The
chapter names these refusals and mints no code for them. Fail-closed interim mapping, recorded in the registry
as `detail` values of existing codes, not as new rows: in request arguments both are reported under FROB-REF-802
(arguments fail the schema) with `observed` of `duplicate-member:<name>` or `identifier-not-printable:<code
point>`; in an artifact file a duplicate member is reported under the owner's code for an unknown, misspelt or
duplicate field (FROB-REF-103 for a design) or, for an owner with no such code, under FROB-REF-891 with
`observed` of `duplicate-member:<name>`. An integer outside ±(2⁵³ − 1) in a number-typed field is FROB-REF-808,
which already exists. If the registry's owner mints dedicated codes, the mapping rows are replaced and no code
is reused.

**Neighbouring code, for orientation only.** FROB-REF-890 (`stack-mismatch`: evidence recorded under another
stack receipt is offered as same-stack evidence without revalidation) is defined by the suite-consumption
chapter and is not one of this chapter's three.

## Acceptance criteria

- [ ] The three rows load from `registries/refusals.toml`; the generated refusal-code enum contains
      `FROB-REF-891`, `-892`, `-893`; each row's `exit` is 3 and its `retry` is `after-change`.
- [ ] Each row names a fixture, and running the fixture through `frob` returns that code with its ranked
      repairs: an artifact with `schema` major raised by one (891); a view given to an identity-required
      parameter (892); a registered-extension file given to a foreign-file parameter (893).
- [ ] Each code is raised by the mechanism named and by no other code path (a source scan for the enum variant:
      891 appears in the version-gate mapper and in owners' readers; 892 and 893 appear in the artifact-input
      stage only).
- [ ] The design owner's fixtures for the same conditions return its own codes, not 891, and the registry's
      `detail` table lists the two uncoded admission faults with their interim codes.
- [ ] The M1 verbs' declared refusal lists include 891, 892 and 893 wherever a verb takes an artifact, and a
      verb row naming a code absent from the registry fails to load.
- [ ] Both surfaces return the same refusal fields for the three fixtures.

## Test plan

### Unit tests

`cargo test -p frob-core refusal_registry_workspace`: `rows_891_892_893_present`; `exit_and_retry_values`;
`repairs_are_ranked_and_nonempty`; `detail_mapping_for_uncoded_faults`.
`cargo test -p frob-verbs refusal_fixtures_workspace`: `fixture_891_unknown_major`; `fixture_891_unknown_field`;
`fixture_892_view`; `fixture_892_respelled`; `fixture_893_foreign_parameter`; `fixture_893_blob_reference`;
`design_owner_uses_its_own_codes`; `only_named_mechanisms_raise_these_codes` (the source scan).

### End-to-end script and logging

`scripts/e2e/format_registry.sh` (bead N34-23), step 7, triggers the three codes for every landed format on the
CLI and through `frob-mcp`, and logs for each the command, exit code 3, the code, `pointer`, `observed`, the
first repair, and whether a job was created (never), in the structured format of STD-E2E as `refusal` events.

### Known answers and negative controls

Known answers: exit 3 and `retry: after-change` for all three. Counterexamples behind the codes: CX-361 (891),
CX-360 (892), CX-358 and the one-owner rule (893). Negative controls: a registry copy with code 892 removed must
make the verb layer fail to load (an unknown code is a load-time error); a verb-layer build that maps a view to
a silent conversion must fail the 892 fixture; a build that reports the owner bypass as a generic schema error
must fail the 893 fixture. Determinism: the refusal envelope's fields are identical twice on one machine and
across platforms.

## Considerations

- Two codes can describe an unsupported version (the design chapter's and the registry's). The rule is fixed
  here so that an agent sees one code per format: an owner with its own code uses it; every other owner uses
  891.
- Messages never include artifact content; they carry pointers, identities and version names, so a refusal
  about a private scene leaks nothing.
- If the S-expression view is withdrawn by its kill gate, 892 remains: export views of runs and build packets
  exist from M1 and M5.
- A fourth registry refusal would be a new row with the next free code in the family; none of these three is
  ever reassigned.

## References

§34.9 (the refusal table; registry rules 1 to 3), §34.14, Appendix O.8, §32.4, §35.11, §14.13; FROB-REF-891,
FROB-REF-892, FROB-REF-893; FROB-REF-103, FROB-REF-125, FROB-REF-802, FROB-REF-808, FROB-REF-890 (neighbours);
CX-358, CX-360, CX-361.
