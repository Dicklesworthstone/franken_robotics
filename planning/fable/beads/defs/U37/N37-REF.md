---
key: "N37-REF"
title: "registries/refusals.toml: safety refusal codes 731, 733, 734, 816, 819, 825, 828, 832 with triggers and repairs"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "safety", "tether", "frob-core", "frob-verbs", "solid", "registry", "test"]
parent: "WP-58"
deps: ["BD-005", "BD-022", "NJL-01", "STD-E2E"]
needs: []
related: ["N24-REF", "N35-REF", "N35x-02", "N23-REF"]
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
generated enumeration (foundation crate); the other chapters' rows. Milestone M0 (DESIGN): the rows are data
entry that depends only on the foundation crate and the verb skeleton, and the claim registry (N00-25) raises
819 at M0, so every row is entered then; the list and reach tests are computed over whatever surface exists
and grow with it (step 5).

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
               "symlink-escape", "symlink-in-write-path", "not-regular", "hard-linked-target", "root-unset",
               "config-unprotected"]
   repairs = ["pass a relative path beneath the named root",
              "refer to the object by identifier or digest (artifact.read, artifact.export)",
              "ask the owner to configure the root"]
   ```
2. **The eight rows (complete).**

| Code | Name (DESIGN) | Meaning | Trigger | `observed` values | Ranked repairs | Retry | Exit | Raised by |
|---|---|---|---|---|---|---|---|---|
| FROB-REF-731 | `private-without-scope-or-flag` | A private object, or anything derived from one, without the private scope and flag | A direct read of a private-marked object without `private:read`; an export of one without `private:export`, without the per-call flag, or with the owner key off; a job whose inputs include a private scan submitted away from the machine that holds it; sending a private-marked object across machines | `missing-scope`, `missing-flag`, `export-disabled-by-owner`, `not-on-holder` | 1. keep the result local and read it through verbs under `private:read`, running the job on the holder; 2. re-derive from a non-private proxy of the place if the result must be shared; 3. ask the owner for `private:export` and pass the flag | after-change | 5 | BD-305, N37-18 (export door), N37-14, N37-19 (`scene.purge`, which lists it) |
| FROB-REF-733 | `licence-forbids-export` | The licence set of the artifact forbids this export | An export of any kind whose licence class is `research-only`, `proprietary` or `unknown` (unknown is the most restrictive); an attribution entry with no notice text | The class and the offending entries (source, licence, class), with the lineage path to each root asset | 1. replace or remove the named asset and re-derive; 2. for `unknown`, record the asset's licence and source so it is classified; 3. keep the artifact local | after-change | 5 | N37-18 |
| FROB-REF-734 | `path-outside-roots` | A path outside the session's roots | A path argument that is empty, absolute, contains a parent component, a backslash or a control character, is over length, resolves through a link that leaves the root, has a link in a write path, targets a non-regular file or a hard-linked file for replacement, or names a root that is not configured or not in the session's scopes; and, with pointer `safety.toml`, the owner-held configuration failing its protection checks in the no-daemon path (a link, not a regular file, another owner, a group or other write bit, a state root not 0700: `config-unprotected`, the message naming the failed check) | As in the row example above | As in the row example above (for `config-unprotected`: the owner restores mode 0600 and ownership; no argument works around it) | never (lexical); after-change (others) | 5 | N37-11, BD-258, BD-305, N37-02, N37-31 (the fallback posture without the admitted directory-handle crate: a write argument other than a fresh run-private directory, `root-unset`) |
| FROB-REF-816 | `authorization-flags-missing-or-not-false` | An artifact whose authorization flags are missing or not `false` | A reader of one of this system's artifact kinds finds the flags object absent, a field absent, a field `true`, a field that is not the literal false, or an extra field; a view without its sidecar offered as an artifact of this system | `missing:<field>`, `true:<field>`, `not-false:<field>`, `extra:<field>`, `no-sidecar` | 1. re-export the artifact from its source with this tool; 2. import a foreign file through the importer for foreign data; 3. none: no input makes this system accept a true flag | never | 3 | N37-23, N37p1-01 (checkpoint metadata key, still sidecar, replay header), BD-303, N37-07 (a checkpoint of this system without `frob.authorization/1`), N37-18 (the export door's first check, M3) |
| FROB-REF-819 | `cited-above-claim-product` | A result cited above its claim product | A consumer declares a minimum (claim kind, class, standing, tier, reality, conditions) for evidence it is offered and the offered result's product is below it in some member, where no more specific code applies; a report sentence that would state a result more strongly than its product | The product's value in the failing member; `weakest` input | 1. raise the weakest input (promote a provisional result; re-run on the referee; measure an assumed field); 2. cite the result for what it supports, in the permitted wording; 3. use a consumer that accepts the product | after-change | 3 | N00-25 (M0: a report citing a result for a claim kind outside its `evidence.claims`), N37-26, BD-308 |
| FROB-REF-825 | `d4-offered-as-evidence` | A result of class D4 offered as evidence | A stored result whose determinism class is D4 or unrecorded is offered to one of the five consumers of the evidence boundary: promotion, qualification, a certificate, a build packet, or paired statistics computed from stored results (a study assembled outside `eval.compare`, among them a stored study read by `study.compare`, N37-28; a D4 or unrecorded arm at the planning or admission of a paired comparison is answered by the layer that admits it, see step 4: at `eval.compare` 542 for a D4 arm and 561 for an unrecorded one, 533 at the Arena study admission, 561 at the licence check elsewhere) | `D4` or `unrecorded`, and the profile name | 1. re-evaluate the policy under the deterministic device profile (D2) or stricter and offer that result; 2. re-run the training under the default deterministic profile; 3. none: a class is never raised by relabeling | after-change | 3 | N37-28; N24b-16 (a D4 arm's evidence in a qualification request, M3) |
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
| A paired statistic planned or admitted with a D4 or unrecorded arm, a property of the execution being planned: at `eval.compare` (pairing admission `pairing::admit`, BD-262, then the numerics licence check); at the study admission of `study.plan` and the other Arena study verbs (`study::admit`); at the numerics chapter's licence check for any other paired statistic (N35-07) | At `eval.compare`: 542 for a D4 arm (`pairing::admit`), 561 for an arm of unrecorded class or a pairing demanded `exact` that the placement cannot license (the licence check that runs after it); 533 at the Arena study admission; 561 at the licence check elsewhere (one condition, three layers: the pair table of N35x-02, N28-REF's layer rule and N35-REF's row 561). Not a planning or admission case: a stored study whose arm results include a result of class D4 or unrecorded, read by `study.compare`, is this chapter's 825 (N37-28's evidence boundary on the paired-statistics path; VERBS-study lists 825 on `study.compare` and neither 542 nor 561) | Comparison chapter; task chapter; numerics chapter; this chapter for 825 |
| Attribution below D2; a `fast-training` number cited, or a policy trained under it offered with no stricter re-evaluation at a citation point outside `frob-learn` | 562; 861 | Numerics chapter |
| A D4-trained policy without re-evaluation cited, scored or attached to a gate by a `frob-learn` verb or record builder | 867 | Learning chapter |
| A `model-conditional` result offered for promotion, build or prototype budget | 830 | Learning chapter |
| A kernel cache entry corrupt and recompilation unavailable | 612 | Device chapter |

5. **Coverage tests.** (a) Loader: the eight rows load; `exit` obeys the family rule (73x → 5; 8xx → 3); each
   row has a message, a retry class and at least one repair or an explicit "none". (b) Verb lists: every verb
   that can raise one of the eight has it in its full refusal list, `all_refusals` (NJL-01: the chapter list
   plus the codes the generator appends by field), never checked against the chapter list alone. Computed for
   the dispatcher-level codes: every verb with a path argument lists 734 (appended by NJL-01 for each `params`
   entry of kind `artifact-input<F>`, `artifact-identity<F>` or `foreign-file`; named in the chapter list for
   a `frob.path/1` output argument such as an export's `out` or `sim.run --resolved-out`); every verb with a
   file or blob argument lists 828 (N37-03 sniffs every one before its reader; appended by NJL-01 for the
   same `params` kinds); only an importing verb, one named in a row of `registries/importers.toml` (N37-05 at
   M1, completed by BD-307), lists 832; every verb whose entry has `exports = true` lists 731, 733 and 816 in
   its chapter list (the place N37-18 and NJL-01 name for the export door's codes); every verb with a
   `consumes_evidence` declaration on an argument lists 819 and 825 (the declaration is N37-26's DESIGN
   addition to the verb entry; until it is in the entry schema of N32a-02 the test reads it from N37-26's
   declaration table). (c) Reach, milestone-aware: for each code whose earliest raising bead (last column
   above) and whose producing script's bead (the list under "End-to-end script") are both at or before the
   milestone under test, at least one end-to-end script step produces it through the real binary, and a code
   with no producing step then fails the coverage test; a code whose raising beads or producing script are
   all later is reported `pending` with that bead and its milestone, never `pass` (828 is raised from M1 by
   N37-03 and produced end to end from M2 by `safe_importers.sh`, N37-06, so at M1 it is `pending: N37-06
   (M2)` for reach and covered by N37-03's unit tests). By M3 every one of the eight is produced. (d) Family parity: the same envelope and the same exit code on the CLI and as the
   MCP tool error's structured content.

## Acceptance criteria

- [ ] `registries/refusals.toml` contains the eight rows with code, family, exit, name, message, retry,
      owner, observed values and ranked repairs, and loads without error.
- [ ] Exit codes: 5 for 731, 733 and 734; 3 for 816, 819, 825, 828 and 832.
- [ ] `FROB-REF-730` is absent and listed as retired; `FROB-REF-840` is present under its owner.
- [ ] Every verb's full list (`all_refusals`) contains the safety codes its arguments can raise (the
      computed lists of step 5(b) match the registry); a file argument without 828 and a non-importing verb
      listing 832 each fail the test.
- [ ] Each of the eight codes is produced by at least one unit test and, from the milestone of its earliest
      raising bead, by one end-to-end script step; before that milestone it is reported `pending` (at M0 the
      rows load, and 819 is `checked` once N00-25's unit test exists, `pending: N00-25` before; at M1 734 and
      816 are produced end to end (`safe_path_jail.sh` of N37-12, `safe_flags.sh` of N37-24); at M2 828 and 832
      (`safe_importers.sh` of N37-06); by M3 all eight are produced end to end).
- [ ] `scripts/e2e/safe_refusal_registry.sh` passes and its log names the eight rows, the list checks and the
      reach table.
- [ ] No refusal message of the eight interpolates imported text or a host path (messages are registered
      templates over identifiers and numbers).

## Test plan

### Unit tests
`cargo test -p frob-core --test refusal_rows_safe -p frob-verbs --test refusal_lists_safe`:
`eight_rows_load`; `exit_family_rule_safe` (a 73x row with exit 3 fails; an 8xx row of the eight with exit 5
fails; row 840, where present, loads with exit 5 under the exit rule's `capability` list, and a copy of it with
exit 3 fails);
`d4_arm_code_by_layer` (the neighbour table of step 4, kept as data beside the rows, maps at `eval.compare` a D4
arm to 542 and an unrecorded arm to 561, either arm to 533 at the Arena study admission and to 561 at the
licence check elsewhere, and a stored D4 result at the five consumers to 825, a stored study with a D4 or
unrecorded arm read by `study.compare` among them; a planted table that names 561 for a D4 arm at
`eval.compare`, or 542 for an unrecorded one, or 542 for a stored study's D4 arm at `study.compare`, fails);
`retired_730_absent_and_reserved`; `every_row_has_repairs_or_explicit_none`;
`path_verbs_list_734`; `exporting_verbs_list_731_733_816`; `evidence_consumers_list_819_825`;
`file_args_list_828` (a planted row with a `foreign-file` parameter whose `all_refusals` lacks 828 fails);
`only_importers_list_832` (a planted non-importing verb listing 832 fails); `lists_read_all_refusals` (a
row whose chapter list omits 734 but whose `params` appends it passes);
`messages_are_templates_without_untrusted_text`; `each_code_has_a_producing_test` (reads the test inventory
and the milestone of each raising bead; a code due at the milestone under test with no producer fails, a
later one is `pending`).

### End-to-end script and logging
The producing steps are in the safety scripts: 731 and 733 in `safe_private_licence.sh`; 734 in
`safe_path_jail.sh`; 816 in `safe_flags.sh`; 819 in `safe_claim_boundary.sh`; 825 in `safe_d4.sh`; 828 and 832
in `safe_importers.sh`. Each logs, per STD-E2E, a `refusal` line with the code, `observed`, `required`, `unit`
where applicable, the retry class and the first repair, and its `summary` counts refusals by code.

This bead's own script, `scripts/e2e/safe_refusal_registry.sh` (sources the STD-E2E helper; writes
`target/e2e-logs/safe_refusal_registry/<utc>.ndjson` and a transcript; no device time; idempotent), checks the
rows and the lists through the real binary: (1) `frob schema --refusals --json`: the eight rows are present
with the exits 5, 5, 5, 3, 3, 3, 3, 3, row 730 is absent and in `retired`, row 840 is present under its owner;
(2) `frob schema --verbs --json` against `frob schema --path-args --json` and `frob schema --exports --json`:
every verb with a path argument lists 734, every exporting verb lists 731, 733 and 816, every verb with a file
or blob argument lists 828, and 832 appears only on importing verbs (each list form is enabled when the bead
that adds it has landed, and logged `not-run: <bead> open` before that); (3) the reach table: for each code,
the newest log of the producing safety script under `target/e2e-logs/` and the step that produced it, or
`pending: <bead> (<milestone>)` while its raising beads are later than the milestone under test. Each `cmd` line
carries the exact command, exit code and wall time; one `assert` line per row and per list check with the
expected and found values; one `refusal-row` line per code with exit, retry, the first repair and `raised_by`;
a `summary` line counts rows present (threshold 8), list violations (threshold 0) and codes produced, pending
and missing. A planted registry copy with 734 at exit 3 run through step 1 must make the script fail.

### Known answers and negative controls
Known answers: the exit code per code (5, 5, 5, 3, 3, 3, 3, 3 in code order). Negative controls: a registry
copy with 734 at exit 3 fails the family rule; a copy that reintroduces 730 fails the retired-code test; a
verb row with a path argument and no 734 in its list fails the list test. Determinism: refusals are D0; the
same request gives the same code, fields and repairs on both surfaces and both platforms.

## Considerations

- Neighbouring codes overlap by design where two chapters each defined one (819 with 820 and 834; 825 with
  561, 861 and 867; 733 with 310; 832 with 303; 816 with 833 and 860). The precedence stated in the raising beads
  gives every request exactly one code; this bead is where the precedence is read in one place, and a new
  overlap must be added to the table of step 4 before its code is registered.
- The D4 codes are divided by raising site (DESIGN; the plan defines 561, 825, 861 and 867 in three chapters
  and does not divide them; the reason is that one request must get one code). The planning or admission of a
  paired comparison is, at `eval.compare`, 542 for a D4 arm (BD-262's `pairing::admit`) and 561 for an arm of
  unrecorded class or an `exact` pairing the placement cannot license (the licence check that runs after it,
  VERBS-eval); 533 at the Arena study admission; 561 at the numerics licence check elsewhere (N35x-02's pair
  table); 825 belongs to a stored D4 result offered to one
  of the five consumers above; 867 to a D4-trained policy cited at `frob-learn`'s own verbs and record builders;
  861 to a training-time number wherever it is cited and to such a policy at every other citation point. The
  numerics registry N35-REF (owner of 561 and 861) and the learning registry N23-REF (owner of 867) state the
  same division, and a change to one is a change to all three.
- 840 is an `8xx` code with exit 5 (the plan's safety table and its owner N24-REF). The exit rule of N35-33 maps
  `8xx` to 3 except the codes of its exception lists: `conflict` (7), `integrity` (9), and `capability` (5), the
  `8xx` codes that report a missing scope or capability, which holds 840. A row 840 with exit 3 therefore fails
  the loader's check 3, and the row with exit 5 loads.
- Repairs are advice built from identifiers; none is a command that weakens a mechanism, and none exists that
  makes a `true` flag, a code payload or a D4 result acceptable.
- A refusal must not leak what it protects: 731 for a direct read confirms existence only to a caller that
  already holds the identifier; 734 never prints a host path; 840 names the campaign and nothing about a
  world.

## References

§37.12 (the chapter's refusal table), Appendix O.7 and O.8 (rows 731, 733, 734, 816, 819, 825, 828, 832, 840),
§32.4 (exit codes), §32.5 (failure classes), §35.11 (families and standing rules).
