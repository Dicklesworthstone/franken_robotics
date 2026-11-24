---
key: "N34-REF"
title: "Workspace refusal-code registry: FROB-REF-891, -892, -893 (format version, view is not identity, owner bypass)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "frob-core", "frob-verbs", "solid", "registry"]
parent: "WP-06"
deps: ["BD-005", "N35-33"]
needs: []
related: ["N34-19", "N34-23", "N34-24", "N34p3-01", "N35x-01", "N35x-02"]
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
three rows in the registry with trigger, pointer, ranked repairs and exit code, names the mechanism that raises
each and the fixture that triggers it, and records how two admission faults that have no code of their own are
reported. Tier: Solid.

DESIGN (order): this is a registry bead and comes before the mechanisms that raise its codes. The version-gate
mapper (N34-19) and the verb layer's artifact-input stage (N34-24) name these codes through the refusal-code type
generated from the registry, in which an unknown code is a load-time error, so both depend on this bead; an
earlier draft depended on N34-19 instead and asserted the mechanisms' behaviour here, which put the rows after
the code that needs them. The rows land at M0, not with their M1 mechanisms: the generator NJL-01 appends 891
to 893 to verb rows registered at M0 (its variations 7 and 8, from a parameter's kind), and every code a row lists
must resolve in `registries/refusals.toml` (RL-06).

## Scope

In: the three rows in `registries/refusals.toml`; one triggering fixture per code, as data; the interim mapping
for the two uncoded admission faults and for the uncoded fixture-reference fault.
Out: the mechanisms that raise the codes (N34-19 for the version gate, N34-24 for the verb boundary), with the
tests that run each registry fixture through its mechanism, the source scan that only the named mechanisms raise
these codes, the surface-parity check and the declared refusal lists of the M1 verbs (all N34-24, which depends
on this bead); the user-side run for every landed format (N34-23, step 7); refusal codes of other chapters that
sit beside these (the design chapter's own version and field codes, the envelope's schema and integer codes,
the stack-mismatch code of the suite-consumption chapter); the allocation ledger (N35-33).

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
which already exists. The two faults are filed in the allocation ledger `registries/refusal_allocations.toml`
as the requests `duplicate-member-name` and `identifier-not-printable` (N35x-02), family `8xx`, held under the
`8xx` capacity decision (N35x-01) on exactly these interim codes. When the ledger assigns numbers, the `detail`
rows here are replaced by the assigned codes through the ledger's hand-off table, and no code is reused. The
three codes of this bead are printed in Appendix O.8 and therefore need no ledger row (N35-33's count check).

**A third uncoded fault: an unresolved fixture reference.** A `@fixture:` reference that the resolver of
N34p3-01 (M0) cannot turn into one file is refused before the request digest, so no key is bound, no job exists
and nothing is charged. The chapter mints no code for it. It is the ledger request `fixture-reference-unresolved`
(family `8xx`, no proposal, filed in N35x-02's second round for this registry and held under N35x-01), and its
`detail` row here is: interim FROB-REF-802 (arguments fail the schema), `detail = "fixture-reference-unresolved"`,
exit 3, `retry: after-change`, `pointer` the parameter, `observed` one of `fixture-name-invalid`,
`fixtures-root-absent`, `fixture-unknown:<name>`, `fixture-outside-root:<name>` (a symbolic link that leaves the
root), `fixture-ambiguous:<name>:<count>` (more than one candidate where one is required) or
`fixture-empty:<name>` (a directory with no file of the format); ranked repairs: 1. pass the reference of the file
itself; 2. pass a path. Raised by the resolver of N34p3-01, which owns the reference grammar (N34-24 changes
only how a fixture file's format is decided, at M1).

**Neighbouring code, for orientation only.** FROB-REF-890 (`stack-mismatch`: evidence recorded under another
stack receipt is offered as same-stack evidence without revalidation) is defined by the suite-consumption
chapter and is not one of this chapter's three.

## Acceptance criteria

- [ ] The three rows load from `registries/refusals.toml`; the generated refusal-code enum contains
      `FROB-REF-891`, `-892`, `-893`; each row's `exit` is 3 and its `retry` is `after-change`.
- [ ] Each row names a fixture that exists and passes raw-byte admission (so that it reaches the stage that
      must refuse it): an artifact with `schema` major raised by one, and one with an unknown field (891); a
      view, and a re-spelled JSON, for an identity-required parameter (892); a registered-extension file for a
      foreign-file parameter, and JSON with a registered `schema` member (893). Each fixture carries a sidecar
      naming its code, the parameter kind it is meant for and the expected `observed` value, which the tests of
      N34-24 read.
- [ ] Each row's `raised_by` names the mechanism (891: the version-gate mapper of N34-19 and the owners'
      readers; 892 and 893: the artifact-input stage of N34-24); the registry check refuses a row whose
      `raised_by` is empty.
- [ ] The registry's `detail` table lists the two uncoded admission faults and `fixture-reference-unresolved`
      with their interim codes, their `detail` values and their ledger request slugs; the fixture-reference row
      names the six `observed` forms and the two ranked repairs.
- [ ] The rows obey the family exit rule of BD-005 (a planted copy of 891 with exit 4 fails to load).

## Test plan

### Unit tests

`cargo test -p frob-core refusal_registry_workspace`: `rows_891_892_893_present`; `exit_and_retry_values`;
`repairs_are_ranked_and_nonempty`; `detail_mapping_for_uncoded_faults` (with the ledger slugs; the three
faults, `fixture-reference-unresolved` under 802 with its six `observed` forms, and no `detail` row whose
interim is a held or proposed number);
`raised_by_nonempty`; `family_exit_rule_holds` (the planted exit-4 copy fails); `fixtures_exist_and_admit` (the
six fixtures exist, pass `frob_core::json::admit` or are a registered view, and each sidecar names one of the
three codes). The fixtures are run through their mechanisms by N34-24's `registry_fixtures_raise_their_codes`
and its source scan `only_named_mechanisms_raise_these_codes`.

### End-to-end script and logging

`scripts/e2e/format_registry.sh` (bead N34-23), step 7, triggers the three codes for every landed format on the
CLI and through `frob-mcp`, and logs for each the command, exit code 3, the code, `pointer`, `observed`, the
first repair, and whether a job was created (never), in the structured format of STD-E2E as `refusal` events.

### Known answers and negative controls

Known answers: exit 3 and `retry: after-change` for all three. Counterexamples behind the codes: CX-361 (891),
CX-360 (892), CX-358 and the one-owner rule (893). Negative controls of the rows: the exit-4 copy; a row with an
empty repair list; a fixture sidecar naming a code that is not one of the three. Negative controls of the
mechanisms against these fixtures, registered here and run by N34-24: a registry copy with code 892 removed
must make the verb layer fail to load (an unknown code is a load-time error); a verb-layer build that maps a
view to a silent conversion must fail the 892 fixture; a build that reports the owner bypass as a generic schema
error must fail the 893 fixture. Determinism: the refusal envelope's fields are identical twice on one machine and
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
`fixture-reference-unresolved` (N34p3-01; N35x-02); CX-358, CX-360, CX-361.
