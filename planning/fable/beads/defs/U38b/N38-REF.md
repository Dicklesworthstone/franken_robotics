---
key: "N38-REF"
title: "frob-conformance: refusal vocabulary of the Gauntlet: harness reasons and the existing codes its rules use"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-005", "BD-025", "STD-E2E", "STD-TEST"]
needs: []
related: ["N38a-05", "N38a-09", "N38a-10", "N40-01", "N38a-12", "N38a-14", "N38a-19", "N38a-36", "N38a-40", "N38b-03", "N38b-04", "N38b-05", "N38b-06", "N38b-07", "N38b-08", "N38b-18", "N38b-19", "N38b-20", "N38b-22"]
gates: ["GATE-GAUNT-01", "GATE-GAUNT-10"]
plan_refs: ["§38.10", "§38.11", "§38.13", "§38.14", "Appendix O", "§32.4", "§37.12"]
---
## Background

Every refusal of FrankenRobotics that an agent can meet through a verb has a stable code (`FROB-REF-nnn`), a
message and ranked repairs, registered in `registries/refusals.toml`; codes are never reused. The verification
subsystem (the Gauntlet, crate `frob-conformance`) is different in kind: by decision it has no agent verb, so
its chapter defines **no new public refusal code**. Its refusals are of two sorts, and this bead registers
both so that none is ad hoc:

1. *Harness reasons*: typed reason strings returned by the Gauntlet's own tools (the registry check, the
   control runner, the gate recorder, the seeding join, the registry diff, the close audit, the live-dependency
   helper). They are read by sessions and by end-to-end scripts, not by product users.
2. *Existing public codes* that the chapter's rules rely on, defined by other chapters, restated here with the
   place where the Gauntlet's rules invoke them.

DESIGN: harness reasons are a closed enumeration in one module with a registry file, so that a script can
match on them and a new reason needs a recorded addition. Rejected: free-text error messages (scripts would
match on prose); minting public codes for harness reasons (the public registry is for verbs).

## Scope

In: the enumeration and its registry file; meaning, trigger, ranked repairs and process exit of each harness
reason; the table of existing codes with exit codes; the test that every reason used anywhere in the crate is
registered. Out: defining or changing any public code (owning chapters).

## Technical approach

**Registry (`crates/frob-conformance/registry/reasons.toml`) and enumeration (`src/reasons.rs`).** One record
per reason: `id`, `tool`, `meaning`, `repairs` (ranked), `exit`. Tools that are Cargo tests fail the test
(process exit 101) and print `reason=<id>` on the first line of the failure; the recorder example exits 3 (a
refused request) and prints the same line; warnings exit 0.

**Harness reasons.**

| Reason | Tool | Trigger | Ranked repairs |
|---|---|---|---|
| `no-control` | Registry check | A gate with status `pass` and an empty control list | Register a control first; set the gate back to `not-run` |
| `dangling-control` | Registry check | A gate lists a control that does not exist | Fix the identifier; add the record |
| `one-way-link` | Registry check | A gate lists a control that does not name the gate, or the reverse | Add the missing direction |
| `uncaught-control` | Registry check | A `pass` row whose control's last recorded outcome is not caught | Re-run the gate; fix the test or the plant |
| `no-gate`, `unknown-gate` | Control loader | A control with an empty `gate` list, or naming a gate absent from `registries/gates.toml` | Name at least one existing gate; register the gate first |
| `kind-not-closed` | Control loader | A control whose `kind` is not one of `mutation`, `planted-input`, `planted-design`, `planted-world`, `a-a` | Choose one of the five kinds |
| `defect-or-expect-empty` | Control loader (BD-025, rule 4) | A control whose `defect` or `expect` is empty, or whose `expect` states no observable (a value, a refusal code or a failed row) | State the planted defect; state the observable the gate must show when it catches it (the value, the refusal code or the failed row) |
| `feature-missing`, `fixture-missing` | Control loader | A `mutation` naming no feature declared in its target crate; another kind naming no existing fixture with its kind header | Declare the feature; add the fixture with its `synthetic` header |
| `unknown-seed-form` | Control loader | A `seed` outside the admitted forms (CX, KA, MR, CERT, INV, PO, RISK, SAFE-X, GATE, or empty on a permanent control) | Seed the control from its catalogue entry, row or rule |
| `boundary-not-control` | Control loader | A control whose defect is the product's own refusal at a limit | Name the build or input in which the limit was removed |
| `cannot-fail` | Control loader | A control whose planted value lies on the safe side of its row | Choose a defect that errs in the unsafe direction |
| `missed` | Control runner | The gate's entries pass with the defect planted | Strengthen the gate's test; re-state the defect |
| `caught-unexpected` | Control runner | The gate fails for a reason other than the control's stated expectation | Correct the expectation or the plant |
| `stale` | Control runner | The defect cannot be planted (feature or fixture gone) | Re-plant; withdraw with a recorded reason |
| `not-eligible` | Tier runner, gate line | No control caught in this run, or a declared dependency without a live-dependency proof | Run the controls; add the proof |
| `suspect-undisposed` | Control review | A control never caught in recorded history, with no disposition at an exit | Record `replanted`, `gate-strengthened` or `withdrawn` |
| `permanent-control-removed` | Control loader | A permanent control deleted or detached from a gate | Restore it |
| `receipt-hash-mismatch`, `receipt-dirty-tree`, `receipt-commit-unknown` | Recorder | The receipt's bytes, tree state or commit cannot be trusted | Re-run the tier on a clean, known commit |
| `gate-line-not-eligible` | Recorder | The receipt's line for the gate is not `pass` or `fail` | Run the gate's controls in the same run |
| `pass-without-control`, `control-missing-from-receipt`, `control-not-caught` | Recorder | The same-run rule is not met | Register and run the control with the gate |
| `self-certified` | Recorder | The recording session implemented the capability | Record from a fresh-context session |
| `evidence-unresolved` | Recorder | An evidence item or claim identifier dangles or is tombstoned | Cite a live receipt; re-run |
| `receipt-stack-dev` | Recorder (N38b-19, step 2) | A receipt whose `stack_dev` is true (a result of a development stack identity), whatever its line's status | Re-run the tier from a binary built under a clean stack receipt |
| `dev-build-identity` | Recorder (N38b-19, step 2) | `may_cite(gate_class)` of N33a-08, called on the receipt's `build_identity`, refuses a `pass` of a Performance or Parity gate from a binary in mode `dev-salt` (a `fail` is recorded) | Rebuild under `depgraph-receipt` and re-run; or record the value without citing it as a pass |
| `receipt-set-invalid` | Recorder (N38b-19, receipts of a part) | Two receipts of one tier, or receipts of two commits, in one recording (the tier or the commits named) | Give one receipt per tier, all from one commit |
| `part-rows-dropped` | Recorder (N38b-19, recording a part again) | A re-recording whose receipts omit an entry that the replaced recording measured and that is still registered (the entry named) | Add the receipt of the dropped entry's tier |
| `receipt-not-of-record` | Recorder, step-7 hook `receipts_of_record` (N38b-19's hook list; the check is N32ax-03's `frob_core::registries::assert_of_record`) | An evidence receipt that fails `assert_of_record`: made under a scratch or snapshot registry root, or with a run-time registry in its fail-closed reading; nothing written | Re-run under the registries of record |
| `unmatched-seed`, `orphan-control`, `rule-without-gate` | Seeding join | A catalogue entry or stated control cell with no control; a control with no seed; a rule that resolves to no gate | Seed the entry; fix the seed; add a gate or withdraw the entry with a reason |
| `removed`, `reused`, `renumbered`, `out-of-order`, `gap` | Registry diff | An identifier family is not append-only | Restore the identifier; append instead |
| `untyped`, `foreign-test`, `no-output`, `self-closed`, `shared-reason`, `shared-output` | Close audit | A bead's close reason does not cite its own test and output from another session | Re-close with a typed reason and its own evidence |
| `burst` (warning) | Close audit | More than 5 closes by one session in 60 seconds | Written disposition by the exit reviewer |
| `stand-in-suspected`, `unbound-dependency` | Live-dependency helper | A perturbed input does not change the output, or changes it without changing the bound identities | Remove the stand-in; bind the input in the envelope |
| `views-differ` | Status views | A generated view differs from the committed file | Regenerate; never edit by hand |
| `reduced-not-recordable` | Recorder | A reduced-count receipt offered for a full-count threshold | Run the full count or record the reduced value as measured |
| `claim-evidence-mismatch` | Recorder (N38a-05) | A row's claim kind is stronger than the kind its evidence can carry: a `reality` row whose evidence includes a `ParityResult` of any status; exit 3, nothing written | Record the row with claim `parity`; cite evidence of the claimed kind; leave the gate `not-run` |
| `no-potency-record` | Recorder (N38a-36) | An analysis, screen or attribution method is cited with no potency record, or with one made under a stale corpus version; exit 3, nothing written | Run the potency check on the current corpus; cite a different method |
| `not-potent` | Recorder (N38a-36) | The cited method's current potency record has the verdict `not-potent`; exit 3, nothing written | Improve the method and re-run it on the held-out half; leave the gate `not-run` |
| `origin-label-mismatch` | Recorder, step-7 hook `origin_labels_admissible` (N38b-07, in N38b-19's hook list) | The claim product recomputed from an evidence item's inputs carries `origin-synthetic` or `origin-development-oracle` and the presented evidence block does not (the recorder's form of FROB-REF-819); exit 3, nothing written, `gates.toml` byte-identical | Cite the receipt as it was produced, with its labels; re-run on the real tier |
| `origin-not-admissible` | Recorder, step-7 hook (N38b-07) | An evidence item carries `origin-development-oracle` (for any gate), or carries `origin-synthetic` for a gate whose claim is `reality`; exit 3, nothing written | Re-run on the CPU referee; record a claim other than `reality` |
| `ka-second-file` | Known-answer loader (N38a-12) | A file other than `known_answers.toml` matches `known_answers*.toml` or `known-answers*.toml`, or is a `.toml` file under a directory named `known_answers` or `known-answers` | Move its rows into the one file; remove the second file |
| `boundary-only` (flag) | Known-answer loader (N38a-12) | A row whose `control.text` only restates a refusal at a limit; the load does not fail (exit 0) and the row stays `obligation: control` | Name the build or input in which the limit was removed and register it as the row's control |
| `relation-without-control`, `control-without-relation` | Control loader, relation registry (N38a-14) | A metamorphic relation with no mutation control in `controls.toml`, or a mutation control whose relation is gone (the map relation ↔ control is one-to-one) | Register the relation's mutation as a control; withdraw the control with a recorded reason, or restore the relation |
| `mutation-survived` | G3 runner (N38a-14, N38a-19) | A relation's mutant produced no violation in the scenarios tried (count reported) | Strengthen the relation's test; re-state the mutation |
| `mutation-stale` | G3 runner (N38a-14, N38a-19) | A relation's mutation cannot be planted as described (the code path no longer exists); never counted as caught; reviewed at the milestone exit | Re-state the mutation at the current code path; never drop it |
| `due-unbound` | G3 binding check (N38a-15 to N38a-18; fails the gate in N38a-19); tier-matrix check (N38a-03) | A relation due at the current milestone has no runner entry on a site it lists, or a due cell of the tier matrix has no runner entry | Bind the relation on every listed site; bind the cell to its leading test's entry |
| `unexpected-owner` | Pilot-ledger completeness check (N38a-10) | A pilot record names a re-run owner the ledger does not give it (rows 19 and 20 name N38a-11 only) | Keep the owners the ledger names; remove the extra owner |
| `unallocated-number` | Allocation validator `registry::alloc::validate` (N38a-40) | A KA or CX number above the plan-time maximum that lies in no block, or in a block owned by another bead's chapter | Request a block in `allocations.toml` and take the number from it; never type a number |
| `missing-number` | Allocation validator (N38a-40) | A hole in a numbering family outside a block whose status is `reserved` or `filling` | Add the row, or record the number as `void` in its block |
| `provisional-tag-past-due` | Allocation validator (N38a-40) | A provisional tag still used in a registry, a test name or a derivation file at its owner's milestone after the owner's block was issued | Map the tag to its block's number in table order, in one commit |
| `cx-count-mismatch` | Counterexample-catalogue validation `frob_conformance::counterexamples::validate` (N38b-05) | A chapter's count of entries, or the plan-time count, differs from N38b-05's count table | Add the missing entries or remove the extra ones; the count table changes only with the plan |
| `same-as-invalid` | Counterexample-catalogue validation (N38b-05) | `same_as` names a missing entry or a later one | Point `same_as` at an existing earlier entry |
| `case-supplier-invalid` | Counterexample-catalogue validation (N38b-05) | `case_supplied_by` names a missing entry, an earlier one, or one with `numeric = false` | Point `case_supplied_by` at an existing later entry with `numeric = true` |
| `fact-without-rerun` | Counterexample-catalogue validation (N38b-05) | A FACT entry with no `rerun` field | Record `rerun`, with `rerun_receipt` when it is true |
| `plant-missing` | Counterexample-catalogue validation (N38b-05) | A runnable entry with an empty `plant` | State the plant; until then the entry stays an obligation |
| `rerun-disagrees-with-pilot` | Counterexample-catalogue validation (N38b-05) | A pilot-derived FACT entry whose `rerun` / `rerun_receipt` pair disagrees with its PILOT record's status (`rerun = true` with that result's receipt exactly when the status is `reproduced`) | Copy the pair from the pilot register's status read (`pilots::rerun_status`); re-run the pilot if the case must be reproduced |
| `plant-changed` (finding) | Seeding tool (N38b-05), written to `registry/seed_findings.toml` | A plant's kind or target changed after its control exists, outside the one placeholder exception | Restore the text (disposition `restored`); append a plant with a new `id`, which gets a new control (disposition `new-plant`) |

The twenty-three reasons after `reduced-not-recordable` are emitted by the beads named in the tool column; each
record names that bead in its `tool` field, and every one of those beads depends on this bead. Their exits
follow the rule above (101 from a Cargo test, 3 from the recorder: so 3 for the five recorder reasons
`claim-evidence-mismatch`, `no-potency-record`, `not-potent`, `origin-label-mismatch`, `origin-not-admissible`;
101 for the allocation validator's three and for the six counterexample-validation reasons of N38b-05, which run
as Cargo tests), except the flag `boundary-only` and the seeding finding `plant-changed`, which exit 0 (a
`plant-changed` finding requires its disposition and is reported at the next milestone exit). `defect-or-expect-empty` is emitted by
the control loader of BD-025 (a dependency), like the other loader reasons, and exits 101.

**Reasons of the gate register's write validator and of the pilot register.** More tools of this crate
print reason strings that their beads say are registered here, and the scan of the acceptance criteria would
find them: the write-time validator of `registries/gates.toml` (N40-01, rule table 4), and the loader and the
funding answer of the pilot register `crates/frob-conformance/registry/pilots.toml` (N38a-09). The pilot loader's
two registry-diff reasons, `reused` and `removed`, are the registry-diff row above. The four validator reasons
that the recorder already uses (`pass-without-control`, `control-not-caught`, `self-certified`,
`evidence-unresolved`) are the rows above, with the validator added to their `tool`; the others are:

| Reason | Tool | Trigger | Ranked repairs |
|---|---|---|---|
| `target-as-measurement` | Gate-register validator (N40-01) | `measured` empty or byte-equal to `threshold` | Record the measured value |
| `claim-not-one-of-seven` | Gate-register validator | `claim` outside the seven, or two values | Apply rule 3 of the register (the claim of the governing part) |
| `no-claim-on-non-reality` | Gate-register validator; tier runner (N38a-02) | `no-claim` on a gate whose claim is not `reality`, or with no reason in `detail`; in the tier runner, an entry returning `NoClaim` for a gate whose claim is not `reality` (the run fails, naming the entry and the gate) | Use `not-run` or `fail`; on a Reality gate name the reason (`no-dataset-admitted`) |
| `blocked-without-question` | Gate-register validator | `blocked` with neither an `OPEN-…` identifier nor `claim-reading: <bead>` in `detail` | Name the question, or the chapter bead whose owner must confirm the claim reading |
| `killed-without-decision` | Gate-register validator | `killed` with no kill gate (or, for a `program` row, the bet whose written consequence is the fallback), fallback and ADR in `detail` | Record the kill-gate decision |
| `duplicate-id`, `reused-id` | Gate-register validator | A gate identifier appears twice, or a retired one returns | Restore the identifier; append a new one |
| `missing`, `unregistered`, `void` | Pilot funding check `pilots::fundable` (N38a-09) | An upper-tier mechanism of the package has no pilot record; a record without a frozen protocol; a record voided by a changed value | Register the record; freeze its protocol; register a new record |
| `registered`, `running` | Pilot funding check | The record has no result yet | Run the pilot; open the package for its fundable mechanisms only |
| `falsifier-holds`, `inconclusive` | Pilot funding check | The pilot's verdict is `falsifier-holds` (the fallback ships, printed with its kill gate), or `inconclusive` (the record states what would decide it) | Fund the fallback under its own name; run what the record says would make the pilot decisive |
| `kill-gate-to-register` | Pilot funding check | The record names a kill gate that the gate register does not yet hold | Register the kill gate in `registries/gates.toml` |
| `moonshot-cap` | Pilot register status change and funding check | A fifth Moonshot pilot or work package set `running` in one milestone | Wait until a running Moonshot closes; at most four run per milestone |
| `plan-time-differs`, `plan-time-not-run` | Pilot funding check | A linked plan-time ledger record (N38a-10) is `differs`, or is not yet run | Resolve or withdraw the linked record; run it |
| `pilot-id-form` | Pilot register loader (N38a-09) | A record identifier outside the grammar `^PILOT-(C-[0-9]{2}(-[a-z]+)?\|PT-([0-9]{2}\|S[0-9])\|[A-Z]{2,14}-[0-9]{2})$` (the record named) | Give the record an identifier of the grammar: `PILOT-C-<nn>` with an optional lower-case suffix, `PILOT-PT-<nn>` or `PILOT-PT-S<n>`, or `PILOT-<SET>-<nn>` |
| `pilot-part-missing` | Pilot register loader | One of the four mandatory parts absent or malformed at registration: a `falsifier` with no comparison with a number or an exact count, a `ceiling` that is neither a value nor `none on speed` with a reason, an empty `fallback.ships`, a `fallback.kill_gate` that is neither a gate of the register nor `to-register` with its proposed wording (the part named) | State the named part; for a mechanism with no gate yet write `to-register` with the proposed wording |
| `pilot-cannot-fail` | Pilot register loader | A pilot whose expected outcome is certain: falsifier text empty or `cannot fail` | State a falsifier the result can fail |
| `result-protocol-mismatch` | Pilot register loader | A result whose `protocol_blake3` differs from the registered protocol hash | Register the changed protocol under a new pilot identifier and run it there; the old record keeps its result |
| `result-before-registration` | Pilot register loader | A result whose date or ledger sequence does not follow the registration's | Register the record first, then run the pilot and append the result |
| `pilot-receipt-uncommitted` | Pilot register loader | A result whose receipt is not committed byte-identical at `receipts/pilots/<hash>.json` (the file absent, or its bytes not hashing to the name) | Commit the receipt's exact bytes at that path in the commit that appends the result |
| `fallback-tier-not-lower` | Pilot register loader | `fallback.ships` names a mechanism whose tier is not lower than the pilot's (for `tier = "S"`, not a Solid mechanism) | Name a lower-tier mechanism the design already contains; put a conditional upper-tier part in `fallback.also` with the pilot that funds it |
| `fallback-also-unknown` | Pilot register loader | A `fallback.also` entry naming no pilot record | Name the existing record that funds the part, or remove the entry |
| `tier-s-without-required-by` | Pilot register loader | A record with `tier = "S"` and no `required_by` | Name in `required_by` the chapter subsection that requires the pilot, or remove the record (a Solid mechanism needs none otherwise) |
| `plan-time-record-unknown` | Pilot register loader | A `plan_time_records` identifier that is not a record of the plan-time ledger (N38a-10) | Name an existing ledger record (`PILOT-PT-<nn>` or `PILOT-PT-S<n>`), or remove the identifier |

Exits: the validator refuses a write with exit 3 and writes nothing (101 from its Cargo tests); the tier runner's
`no-claim-on-non-reality` fails the Cargo test target (101); the pilot register loader's ten reasons above fail its
Cargo tests (101); the funding check is an answer, not a refusal, printed by `cargo run -p frob-conformance --example pilots -- fundable …
--json` with exit 0 and `Blocked{<reason>}` per mechanism; the refused status change of `moonshot-cap` exits
3. N40-01 depends on this bead, whose reasons its validator prints (no cycle: no dependency of this bead
reaches N40-01).

**Existing public codes the chapter's rules rely on (meaning as defined by the owner; exit code from the code
family: 3 for a refused input or question, 5 for a capability or scope refusal).**

| Code | Meaning | Where the Gauntlet's rules invoke it | Exit |
|---|---|---|---|
| FROB-REF-104 | Inertia not physically consistent (smallest eigenvalue reported) | Certifier self-test of physical consistency | 3 |
| FROB-REF-532 | A fixed-n study has no pre-analysis, or names a verdict it cannot reach | Certifier self-test of the pre-analysis; the planted tie at n = 64 | 3 |
| FROB-REF-537, -546, -548 | A rung used without a rank-agreement record; calibration from survivors; pooling across two model identities | Calibration sets and the rank-agreement self-test | 3 |
| FROB-REF-544 | A training gate record without its negative controls | Behaviour, not plumbing | 3 |
| FROB-REF-560, -567 | A bound offered episodes not drawn nominally; a hard or search-selected world offered as a confirmation sample | Reading the hard-world set | 3 |
| FROB-REF-569 | Splitting requested where no randomness enters after the fork | Rare-event self-test control | 3 |
| FROB-REF-814 | Pooling across cells, across model identities, or of synthetic with real data | The `synthetic` label; the Reality lane | 3 |
| FROB-REF-815 | Closing a bead without a resolving log line from another session | Typed close reasons; no self-certification | 3 |
| FROB-REF-816, -833 | An artifact whose authorization flags are missing or not `false` | `sim_to_real_certified` has no setter | 3 |
| FROB-REF-819, -834 | A result cited or presented above its claim product | Label propagation; the claim-product self-test | 3 |
| FROB-REF-823, -826, -829, -850 | Unknown proof kind; quantifier prefix mismatch; missing side condition; wrong frame, unit, sign or port order | The certificate mutation corpus | 3 |
| FROB-REF-825 | A result of class D4 offered as evidence | The determinism-classifier self-test | 3 |
| FROB-REF-840 | Sealed confirmation data asked for before close without the administrative scope | The confirmation firewall in the qualification self-test | 5 |
| FROB-REF-869 | Citing a claim whose evidence closure is dangling or tombstoned | A `pass` whose evidence does not resolve | 3 |
| FROB-REF-873 | A dataset lacking licence, provenance, rights or measurement uncertainty | Admission of the first real dataset | 3 |
| FROB-REF-880 | An attribution cited as an established cause | Attribution self-test | 3 |

## Acceptance criteria

- [ ] `reasons.toml` lists every harness reason above with meaning, repairs and exit; the enumeration and the
      file agree; the twenty-three reasons of N38a-03, N38a-05, N38a-10, N38a-12, N38a-14 to N38a-19, N38a-36,
      N38a-40, N38b-05 and N38b-07 are present with the exits stated (3 for the five recorder reasons, 0 for
      `boundary-only` and `plant-changed`, 101 otherwise), and `defect-or-expect-empty` is present with the other
      control-loader reasons of BD-025 (exit 101).
- [ ] The gate-register validator's reasons (N40-01) and the pilot register's funding reasons and ten load
      reasons (N38a-09; exit 101) are present with their tools and exits; the four shared with the recorder are
      one row each, naming both tools; `no-claim-on-non-reality` names the validator and N38a-02's tier runner.
- [ ] The recorder's own reasons of N38b-19 (`receipt-stack-dev`, `dev-build-identity`, `receipt-set-invalid`,
      `part-rows-dropped`) and its step-7 hook reason `receipt-not-of-record` are present with exit 3 and the
      repairs N38b-19 states.
- [ ] Every reason string printed anywhere in the conformance crate or its scripts is a registered reason (a
      scan of the source and of the end-to-end logs' `refusal` events finds no unregistered one).
- [ ] Every public code cited by a Gauntlet test exists in `registries/refusals.toml`; a cited code that the
      public registry does not know fails the test.
- [ ] No record of the public registry names the conformance crate as its owner.
- [ ] `scripts/e2e/gauntlet_reasons.sh` passes; its log shows the planted missing record caught (exit 101,
      named) and zero unknown reasons or codes in the scanned Gauntlet logs.

## Test plan

### Unit tests

`crates/frob-conformance/tests/reasons.rs`: `enumeration_matches_registry`; `every_reason_has_repair`;
`no_unregistered_reason_in_source` (scan for `reason=`); `cited_public_codes_exist` (the list above against the
public registry); `no_public_code_owned_by_conformance`; `exit_codes_by_family` (the 5xx and 8xx codes above map
to 3; FROB-REF-840 to 5); `emitting_beads_reasons_registered` (the twenty-three reasons of the emitting beads,
each with its tool and exit; a planted `reasons.toml` without `due-unbound` fails, naming it, and one without
`origin-not-admissible`, or without `rerun-disagrees-with-pilot`, fails the same way; `plant-changed` carries
exit 0 and its two dispositions); `loader_reasons_cover_bd025_plants` (the twelve plants of BD-025's
validator each print a registered reason, `defect-or-expect-empty` among them); `recorder_reasons_exit_3` (the
five recorder reasons carry exit 3, and so do N38b-19's `receipt-stack-dev`, `dev-build-identity`,
`receipt-set-invalid`, `part-rows-dropped` and `receipt-not-of-record`; the allocation validator's three and
N38b-05's six validation reasons exit 101);
`gate_validator_and_pilot_reasons_registered` (the seven validator ids, the eleven funding ids and the ten pilot
load ids of the tables above are present, the load ids with exit 101; `pass-without-control` names both the
recorder and the validator; `no-claim-on-non-reality` names the validator and the tier runner; a scratch
`reasons.toml` without `moonshot-cap`, or without `pilot-receipt-uncommitted`, fails, naming it).

### End-to-end script and logging

Every end-to-end script of the Gauntlet logs harness reasons in `refusal` events with the field `reason`, and
public codes with the field `code`; the log linter rejects a `refusal` event with neither.

This bead's script, `scripts/e2e/gauntlet_reasons.sh` (sources the STD-E2E helper; writes
`target/e2e-logs/gauntlet_reasons/<utc>.ndjson` and a transcript; idempotent; no device time): (1) runs
`FROB_TEST_SEED=0 cargo test -p frob-conformance --test reasons` and asserts exit 0; (2) runs the same target
against a scratch copy of `reasons.toml` without `defect-or-expect-empty` (through the test kit's scratch
registry root) and asserts exit 101 with `reason=` naming the missing record on the first failure line; (3)
scans every `refusal` event of the newest log of each Gauntlet script under `target/e2e-logs/` and asserts
that each `reason` is registered and each `code` is a row of `registries/refusals.toml`, logging the count of
events read per script and every unknown value. Each `cmd` line carries the exact command, exit code and wall
time; each check is an `assert` line with expected and found values; the `summary` line gives reasons
registered (by tool and by exit), public codes cited, events scanned and unknowns found (threshold 0).

### Known answers and negative controls

Negative controls: an unregistered reason string planted in a test; a cited public code that does not exist
(`FROB-REF-999`). Determinism: D0.

## Considerations

- If a harness reason ever needs to reach an agent through a verb, it gets a public code from its owning
  chapter then; the harness enumeration is not a back door into the public registry.
- The exit code 1 is never emitted on purpose anywhere in the program; a 1 means an unhandled failure.

## References

§38.10, §38.11, §38.13, §38.14; Appendix O; §32.4; §37.12; ADR-GAUNT-01.
