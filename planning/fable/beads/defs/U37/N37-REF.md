---
key: "N37-REF"
title: "registries/refusals.toml: safety refusal codes 731, 733, 734, 816, 819, 825, 828, 832 with triggers and repairs"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "safety", "tether", "frob-core", "frob-verbs", "solid", "registry", "test"]
parent: "WP-58"
deps: ["BD-005", "BD-022"]
needs: ["Appendix O: the consolidated refusal registry `registries/refusals.toml` and its loader, in which these rows live beside every other chapter's (the registry is loaded by the foundation crate; an unknown code is a load-time error)"]
related: ["N37-KA", "N37-CX", "N37-01", "N37-03", "N37-05", "N37-07", "N37-11", "N37-18", "N37-23", "N37-26", "N37-28", "BD-305", "BD-261"]
gates: []
plan_refs: ["§37.12", "Appendix O.7", "Appendix O.8", "§32.4", "§32.5", "§35.11"]
---
## Background

Every refusal of the system has a stable code, a message and ranked repairs, and belongs to a family: `7xx`
budget and capability, `8xx` evidence and protocol (the other families are design, numerics, scene, control,
learning, device and internal). Three standing rules apply to all of them: refuse, never truncate; a budget
stop publishes nothing partial as a result; a misspelt parameter is a refusal, not a default. Codes are never
reused, and a code that is not in the registry does not compile (the code enumeration is generated from the
registry) or, when it arrives serialized from another process, fails at load.

The safety chapter defines eight codes. Its own table lists a ninth row, for a confirmation world read in a
discovery session: that refusal is `FROB-REF-840`, owned by the qualification chapter, and the safety
chapter's earlier code `FROB-REF-730` is retired in its favour (it must not exist in the registry; it is
never reused).

The chapter's table gives each code a one-line meaning and an exit code and no repairs. A refusal without a
repair teaches an agent nothing, so this bead completes each row with its trigger, its observed and required
fields and ranked repairs (the repairs are DESIGN, written by the beads that raise each code and gathered
here).

Exit codes of the binaries, for reference: 3 for a refused input or question (families 1xx to 5xx and 8xx
other than conflicts); 4 for budget; 5 for a capability not implemented, not admitted or outside the session
token's scopes (72x and 73x); 6 for a device; 7 for a conflict; 9 for indeterminate and integrity faults.
On MCP a refusal is a tool error whose structured content is the same envelope.

## Scope

In: the eight registry rows in full; the note on 840 and 730; the map from each code to the verbs that list
it and the tests that raise it; the relation of each code to its neighbours owned by other chapters; the
loader and coverage tests. Out: raising the refusals (each mechanism bead); the registry loader and the
generated enumeration (foundation crate); the other chapters' rows.

## Technical approach

1. **Row format** (`registries/refusals.toml`; the fields the loader requires):
   ```toml
   [[refusal]]
   code = "FROB-REF-734"
   family = "7xx"
   exit = 5
   name = "path-outside-roots"
   message = "A path outside the session's roots"
   retry = "never"                 # never | after-change | backoff
   owner = "safety"
   observed = ["empty", "control-character", "absolute", "parent-component", "backslash", "too-long",
               "symlink-escape", "symlink-in-write-path", "not-regular", "hard-linked-target", "root-unset"]
   repairs = ["pass a relative path beneath the named root",
              "refer to the object by identifier or digest (artifact.read, artifact.export)",
              "ask the owner to configure the root"]
   ```
2. **The eight rows (complete).**

| Code | Name (DESIGN) | Meaning | Trigger | `observed` values | Ranked repairs | Retry | Exit | Raised by |
|---|---|---|---|---|---|---|---|---|
| FROB-REF-731 | `private-without-scope-or-flag` | A private object, or anything derived from one, without the private scope and flag | A direct read of a private-marked object without `private:read`; an export of one without `private:export`, without the per-call flag, or with the owner key off; a job whose inputs include a private scan submitted away from the machine that holds it; sending a private-marked object across machines | `missing-scope`, `missing-flag`, `export-disabled-by-owner`, `not-on-holder` | 1. keep the result local and read it through verbs under `private:read`, running the job on the holder; 2. re-derive from a non-private proxy of the place if the result must be shared; 3. ask the owner for `private:export` and pass the flag | after-change | 5 | BD-305, N37-18 (export door), N37-14 |
| FROB-REF-733 | `licence-forbids-export` | The licence set of the artifact forbids this export | An export of any kind whose licence class is `research-only`, `proprietary` or `unknown` (unknown is the most restrictive); an attribution entry with no notice text | The class and the offending entries (source, licence, class), with the lineage path to each root asset | 1. replace or remove the named asset and re-derive; 2. for `unknown`, record the asset's licence and source so it is classified; 3. keep the artifact local | after-change | 5 | N37-18 |
| FROB-REF-734 | `path-outside-roots` | A path outside the session's roots | A path argument that is empty, absolute, contains a parent component, a backslash or a control character, is over length, resolves through a link that leaves the root, has a link in a write path, targets a non-regular file or a hard-linked file for replacement, or names a root that is not configured or not in the session's scopes | As in the row example above | As in the row example above | never (lexical); after-change (others) | 5 | N37-11, BD-258, BD-305 |
| FROB-REF-816 | `authorization-flags-missing-or-not-false` | An artifact whose authorization flags are missing or not `false` | A reader of one of this system's artifact kinds finds the flags object absent, a field absent, a field `true`, a field that is not the literal false, or an extra field; a view without its sidecar offered as an artifact of this system | `missing:<field>`, `true:<field>`, `not-false:<field>`, `extra:<field>`, `no-sidecar` | 1. re-export the artifact from its source with this tool; 2. import a foreign file through the importer for foreign data; 3. none: no input makes this system accept a true flag | never | 3 | N37-23, BD-303 |
| FROB-REF-819 | `cited-above-claim-product` | A result cited above its claim product | A consumer declares a minimum (claim kind, class, standing, tier, reality, conditions) for evidence it is offered and the offered result's product is below it in some member, where no more specific code applies; a report sentence that would state a result more strongly than its product | The product's value in the failing member; `weakest` input | 1. raise the weakest input (promote a provisional result; re-run on the referee; measure an assumed field); 2. cite the result for what it supports, in the permitted wording; 3. use a consumer that accepts the product | after-change | 3 | N37-26, BD-308 |
| FROB-REF-825 | `d4-offered-as-evidence` | A result of class D4 offered as evidence | A stored result whose determinism class is D4 or unrecorded is offered to paired statistics, promotion, qualification, a certificate or a build packet | `D4` or `unrecorded`, and the profile name | 1. re-evaluate the policy under the deterministic device profile (D2) or stricter and offer that result; 2. re-run the training under the default deterministic profile; 3. none: a class is never raised by relabeling | after-change | 3 | N37-28 |
| FROB-REF-828 | `code-payload-or-weight-format` | A payload that carries code, or a weight format outside the allowlist | A file or blob argument whose leading bytes match an executable, script, pickle, archive or bitcode signature; a weight file that is not the plain tensor layout, has unaccounted bytes, a non-finite value, an unknown metadata key, or an architecture family or layout not on the allowlist | The kind (`elf`, `mach-o`, `pe`, `wasm-module`, `script`, `pickle`, `zip`, `bitcode`, `hdf5`, `array-file`), or `not-plain-layout`, `unaccounted-bytes`, `non-finite-value`, `unknown-metadata-key`, `family-not-allowlisted`, `layout-mismatch` | 1. submit the data form (an expression of the requirement language; a typed edit; tensors by name in the plain container with the policy manifest); 2. use an allowlisted family or ask the learning lead to add one; 3. for a kernel or a custom term: change the model or the derivation input; custom code is not admitted | never (code); after-change (weights) | 3 | N37-03, N37-07 |
| FROB-REF-832 | `import-over-limit` | An import over its size, depth or polygon limit | A design import, a mesh or a weight file that exceeds a limit of the import profile in force (bytes, nesting depth, elements, attributes, children, attribute bytes, includes, polygons, meshes, tensors, header bytes) | `limit` (the registry key), the observed value, the limit, the profile name and digest | 1. reduce the input (decimate, prune, split); 2. expand macros outside and remove generated duplication; 3. ask the owner to select a larger import profile | after-change | 3 | N37-05, N37-07, BD-307 |

3. **The ninth row of the chapter's table.** `FROB-REF-840` (owner: the qualification chapter): a
   confirmation world or run is not readable in a discovery session: sealed confirmation data asked for
   before close. Exit 5. The safety chapter's `FROB-REF-730` is retired in its favour: the registry contains
   no row 730, and a test asserts it (retired codes are listed in a `retired` table so they can never be
   reassigned).
4. **Neighbours owned elsewhere (so that each situation has exactly one code).**

| Situation | Code | Owner |
|---|---|---|
| The token lacks a scope; a token is invalid or an attenuation widens; a write outside the namespace | 720; 723; 725 | Surface chapter |
| A scene pack that would export a research-only or private asset, at pack time | 310 | Scene chapter |
| A scene source over a decode limit | 303 | Scene chapter |
| A transfer class that may not cross machines | 618 | Device chapter |
| A flag missing or true inside a certificate bundle, found by the auditor | 833 | Certificate chapter |
| A build packet without the flags, or any attempt to produce a true flag | 860 | Manufacturing chapter |
| A result presented above the class of its weakest input inside a bundle | 834 | Certificate chapter |
| A label other than `established` cited as established | 820 | Dynamics chapter (trust ladder) |
| A result cited as qualified without a promoting receipt | 845 | Qualification chapter |
| A paired statistic planned with a D4 or unrecorded arm; attribution below D2; a `fast-training` number cited | 561; 562; 861 | Numerics chapter |
| A `model-conditional` result offered for promotion, build or prototype budget | 830 | Learning chapter |
| A kernel cache entry corrupt and recompilation unavailable | 612 | Device chapter |

5. **Coverage tests.** (a) Loader: the eight rows load; `exit` obeys the family rule (73x → 5; 8xx → 3); each
   row has a message, a retry class and at least one repair or an explicit "none". (b) Verb lists: every verb
   that can raise one of the eight lists it in its registry `refusals` (computed for the dispatcher-level
   ones: every verb with a path argument lists 734; every exporting verb lists 731, 733 and 816; every verb
   with a `consumes_evidence` argument lists 819 and 825; every importing verb lists 828 and 832). (c) Reach:
   for each code, at least one end-to-end script step produces it through the real binary (the map is the
   last column above); a code with no producing test fails the coverage test. (d) Family parity: the same
   envelope and the same exit code on the CLI and as the MCP tool error's structured content.

## Acceptance criteria

- [ ] `registries/refusals.toml` contains the eight rows with code, family, exit, name, message, retry,
      owner, observed values and ranked repairs, and loads without error.
- [ ] Exit codes: 5 for 731, 733 and 734; 3 for 816, 819, 825, 828 and 832.
- [ ] `FROB-REF-730` is absent and listed as retired; `FROB-REF-840` is present under its owner.
- [ ] Every verb's `refusals` list contains the safety codes its arguments can raise (the computed lists
      match the registry).
- [ ] Each of the eight codes is produced by at least one end-to-end script step and one unit test.
- [ ] No refusal message of the eight interpolates imported text or a host path (messages are registered
      templates over identifiers and numbers).

## Test plan

### Unit tests
`cargo test -p frob-core --test refusal_rows_safe -p frob-verbs --test refusal_lists_safe`:
`eight_rows_load`; `exit_family_rule_safe` (a 73x row with exit 3 fails; an 8xx row with exit 5 fails);
`retired_730_absent_and_reserved`; `every_row_has_repairs_or_explicit_none`;
`path_verbs_list_734`; `exporting_verbs_list_731_733_816`; `evidence_consumers_list_819_825`;
`importing_verbs_list_828_832`; `messages_are_templates_without_untrusted_text`;
`each_code_has_a_producing_test` (reads the test inventory).

### End-to-end script and logging
None of its own. The producing steps are in the safety scripts: 731 and 733 in `safe_private_licence.sh`; 734
in `safe_path_jail.sh`; 816 in `safe_flags.sh`; 819 in `safe_claim_boundary.sh`; 825 in `safe_d4.sh`; 828 and
832 in `safe_importers.sh`. Each logs, per STD-E2E, a `refusal` line with the code, `observed`, `required`,
`unit` where applicable, the retry class and the first repair, and its `summary` counts refusals by code. A
conformance report (`frob-conformance refusals report --chapter safe`) prints, per code, the last script run
that produced it.

### Known answers and negative controls
Known answers: the exit code per code (5, 5, 5, 3, 3, 3, 3, 3 in code order). Negative controls: a registry
copy with 734 at exit 3 fails the family rule; a copy that reintroduces 730 fails the retired-code test; a
verb row with a path argument and no 734 in its list fails the list test. Determinism: refusals are D0; the
same request gives the same code, fields and repairs on both surfaces and both platforms.

## Considerations

- Neighbouring codes overlap by design where two chapters each defined one (819 with 820 and 834; 825 with
  561 and 861; 733 with 310; 832 with 303; 816 with 833 and 860). The precedence stated in the raising beads
  gives every request exactly one code; this bead is where the precedence is read in one place, and a new
  overlap must be added to the table of step 4 before its code is registered.
- Repairs are advice built from identifiers; none is a command that weakens a mechanism, and none exists that
  makes a `true` flag, a code payload or a D4 result acceptable.
- A refusal must not leak what it protects: 731 for a direct read confirms existence only to a caller that
  already holds the identifier; 734 never prints a host path; 840 names the campaign and nothing about a
  world.

## References

§37.12 (the chapter's refusal table), Appendix O.7 and O.8 (rows 731, 733, 734, 816, 819, 825, 828, 832, 840),
§32.4 (exit codes), §32.5 (failure classes), §35.11 (families and standing rules).
