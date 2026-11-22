---
key: "N38-REF"
title: "frob-conformance: refusal vocabulary of the Gauntlet: harness reasons, and the existing codes its rules rely on"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "BD-005", "N38b-04", "N38b-19", "STD-TEST"]
needs: ["Appendix O: the refusal registry `registries/refusals.toml` and its loader (BD-005), in which the existing codes cited here are defined by their owning chapters", "§32.4: the exit-code table of the command-line surface, which fixes the exit code of each code family"]
related: ["N38b-03", "N38b-05", "N38b-06", "N38b-07", "N38b-08", "N38b-18", "N38b-20", "N38b-22"]
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
| `unmatched-seed`, `orphan-control`, `rule-without-gate` | Seeding join | A catalogue entry or stated control cell with no control; a control with no seed; a rule that resolves to no gate | Seed the entry; fix the seed; add a gate or withdraw the entry with a reason |
| `removed`, `reused`, `renumbered`, `out-of-order`, `gap` | Registry diff | An identifier family is not append-only | Restore the identifier; append instead |
| `untyped`, `foreign-test`, `no-output`, `self-closed`, `shared-reason`, `shared-output` | Close audit | A bead's close reason does not cite its own test and output from another session | Re-close with a typed reason and its own evidence |
| `burst` (warning) | Close audit | More than 5 closes by one session in 60 seconds | Written disposition by the exit reviewer |
| `stand-in-suspected`, `unbound-dependency` | Live-dependency helper | A perturbed input does not change the output, or changes it without changing the bound identities | Remove the stand-in; bind the input in the envelope |
| `views-differ` | Status views | A generated view differs from the committed file | Regenerate; never edit by hand |
| `reduced-not-recordable` | Recorder | A reduced-count receipt offered for a full-count threshold | Run the full count or record the reduced value as measured |

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
      file agree.
- [ ] Every reason string printed anywhere in the conformance crate or its scripts is a registered reason (a
      scan of the source and of the end-to-end logs' `refusal` events finds no unregistered one).
- [ ] Every public code cited by a Gauntlet test exists in `registries/refusals.toml`; a cited code that the
      public registry does not know fails the test.
- [ ] No record of the public registry names the conformance crate as its owner.

## Test plan

### Unit tests

`crates/frob-conformance/tests/reasons.rs`: `enumeration_matches_registry`; `every_reason_has_repair`;
`no_unregistered_reason_in_source` (scan for `reason=`); `cited_public_codes_exist` (the list above against the
public registry); `no_public_code_owned_by_conformance`; `exit_codes_by_family` (the 5xx and 8xx codes above map
to 3; FROB-REF-840 to 5).

### End-to-end script and logging

No script of its own. Every end-to-end script of the Gauntlet logs harness reasons in `refusal` events with the
field `reason`, and public codes with the field `code`; the log linter rejects a `refusal` event with neither.

### Known answers and negative controls

Negative controls: an unregistered reason string planted in a test; a cited public code that does not exist
(`FROB-REF-999`). Determinism: D0.

## Considerations

- If a harness reason ever needs to reach an agent through a verb, it gets a public code from its owning
  chapter then; the harness enumeration is not a back door into the public registry.
- The exit code 1 is never emitted on purpose anywhere in the program; a 1 means an unhandled failure.

## References

§38.10, §38.11, §38.13, §38.14; Appendix O; §32.4; §37.12; ADR-GAUNT-01.
