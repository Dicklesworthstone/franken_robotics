---
key: "N41-REF"
title: "frob-conformance: refusal vocabulary of the delivery registers: harness reasons and the public codes relied on"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "EPIC-M0"
deps: ["BD-025", "BD-005", "STD-TEST"]
needs: ["§38.11: the harness-reason registry `crates/frob-conformance/registry/reasons.toml` and its enumeration, to which these reasons are added, and the reasons already defined there for the close audit and the recorder (N38-REF, N38b-03)", "Appendix O: the public refusal registry `registries/refusals.toml`, in which the existing codes cited here are defined by their owning chapters (BD-005)", "§32.4: the exit-code table of the command-line surface, which fixes the exit code of each code family (seed BD-022)", "§23.19, §28.20: codes for three refusals that this unit's seed beads use under provisional names (N23-REF, N28-REF)"]
related: ["N41-KA", "N41-CX", "N41-01", "N41-03", "N41-16", "N41-17", "N41-25", "N41-26", "N41-27", "N41-28", "N41-29", "N41-32", "N41-PM", "BD-200", "BD-344"]
gates: ["GATE-GAUNT-01", "GATE-WS-08"]
plan_refs: ["§41.4", "§42.1", "§42.3", "§45.11", "§46", "§47.2", "§47.4", "Appendix O", "§38.11"]
---
## Background

A **refusal** is a decision not to do what was asked, reported with a stable identifier, a meaning and ranked
repairs, so that an agent can act on it without reading prose. The product's public refusals have codes
(`FROB-REF-nnn`) in one registry; codes are never reused. The five delivery chapters this unit carries (risks,
open questions, the bead graph, the first session, the definition of done and the release rubric) define **no
public code**: the printed refusal registry has no row whose section is one of them. What they define is rules
whose violation a tool must report: a register row that mitigates by a hope, an exit with a question decided by
silence, a tracker export with an orphan, a release cell that lacks evidence of the kind it declares.

DESIGN: those reports are **harness reasons**, not public codes. A harness reason belongs to a tool of the
verification program (`frob-conformance`, `xtask`, the exit scripts), is a member of a closed enumeration with a
registry file, and is matched by scripts as an identifier and never as prose. Minting public codes for them was
rejected: the public registry is for verbs that a user or agent calls on a design, and a reason such as "the
reviewer implemented the milestone" has no meaning there. If a harness reason ever needs to reach an agent
through a verb, it gets a public code from its owning chapter.

This bead allocates the reasons of this unit's tools, states the trigger and repairs of each, lists the existing
public codes that the unit's rules and the register's fail-closed defaults rely on, and records three refusals
that this unit's seed beads use under provisional names.

## Scope

In: the reasons below as records of the harness-reason registry; the cross-reference from each tool's failure to
its reason; the table of existing codes; the test that every reason used by this unit's tools is registered and
every registered reason is reachable. Out: the tools themselves (the beads named in each row); public codes (their
owning chapters); the reasons of the close audit and the recorder, which already exist and are reused, not
redefined.

## Technical approach

**Registry.** Records are appended to `crates/frob-conformance/registry/reasons.toml` with `id`, `tool`,
`meaning`, `repairs` (ranked) and `exit`. Conventions of that registry: a tool that is a test fails the test and
prints `reason=<id>` on the first line of the failure; a command that refuses a request exits 3 and prints the
same line; a warning exits 0. `xtask` checks exit non-zero with the reason in their first line. Exit code 1 is
never emitted on purpose: it means an unhandled failure.

**Harness reasons allocated here** (the tool's bead in parentheses).

| Reason | Tool | Trigger | Ranked repairs |
|---|---|---|---|
| `risk-series-broken` | Risk loader (N41-01) | A gap in `RISK-nn`, a duplicate, or a retired number reused | Restore the row; give the new risk the next free number |
| `risk-hope-mitigation` | Risk loader | A mitigation with no gate, no named fallback and no owner default | Name the mechanism's gate, the kill gate's fallback, or the open question |
| `risk-bare-upstream` | Risk loader | An upstream work item as the only mitigation, or one with no interim posture | State the interim posture that ships; add the fallback |
| `risk-dangling-ref` | Risk loader | A gate, bead or open question named by a row does not exist | Correct the identifier; create the missing record first |
| `risk-without-plant` | Risk loader | A row with no `planted` field | Declare the plant with slug, text and due milestone |
| `risk-no-reading` | Risk review (N41-01) | A selected row has no `fired` reading at an exit | Record the reading with the observed value and its artifact |
| `risk-fired-no-outcome` | Risk review | A fired signal with none of the three outcomes; or an outcome naming a kill-gate decision or ADR that does not exist | Record the kill-gate decision, the ADR, or a written reason naming the evidence |
| `plant-declared-past-due`, `plant-missed`, `plant-stale`, `plant-unexpected`, `plant-not-run` | Plant runner (N41-32, N41-37) | A plant still declared at or after its due exit; a detector that did not report the signal; a fixture that cannot be planted; a detector that failed for another reason; a plant recorded on neither machine | Bind the plant; fix the detector, never the plant's expectation, unless the detector's observable changed by design; rebuild the fixture; run on the machine that has the device |
| `plant-stub-detector` | Plant runner | The detector's live-dependency proof fails (a stand-in) | Fire against the real check |
| `premortem-warning-unanswered` | Exit runner (N41-PM) | An outcome-level reason is adverse and the exit record has no written answer | Answer in the exit record; record the measurement if it exists |
| `open-rule-broken` | Open-question loader (N41-03) | A row with an empty owner, default, decider or deadline; an identifier outside the scheme; counts that do not add to 161 | Complete the row; correct the count in the same change that adds or closes a question |
| `open-default-adds-claim` | Open-question loader | A default classified as adding a claim or capability | Rewrite the default so that it removes one |
| `open-closed-without-record` | Open-question loader | A measurement-decided question marked decided with no gate, pilot or ledger record | Take the measurement; until then leave it open or take the default |
| `open-id-reused` | Open-question loader | A retired or decided identifier reused; an alias resolving to more than one row | Give the new question a new identifier |
| `open-no-decision-bead` | Open-question loader | A blocking row with no decision bead, or a pending marker still set at the row's asked-at milestone | Name the bead; write it if it does not exist |
| `open-decided-by-silence` | Exit runner (N41-03, with the owner calendar) | A question at its deadline with no decision record and no default ADR | Record the owner's answer, or run `open default` to take the default as an ADR |
| `open-default-blocks-exit` | Exit runner (N41-15) | The one default that blocks a milestone is in force at that exit (the ten-fold envelope at M3) | Record the approval; or record the gate `not run` and stay in the milestone |
| `tracker-cycle`, `tracker-unresolved-key`, `tracker-orphan`, `tracker-exit-chain-broken` | Tracker check (N41-16) | A dependency cycle; a key that does not exist; a bead with no parent or not an ancestor of its exit review; a broken chain of exit reviews | Remove the edge; fix the key; set the parent and add the bead to its exit review's ancestry; restore the chain |
| `tracker-gate-uncarried`, `tracker-part-without-bead` | Tracker check | A gate in the register cited by no bead; a gate that closes in parts with a part no bead carries | Add the bead, or attach the gate to the bead that measures it |
| `tracker-no-test-plan`, `tracker-no-negative-control` | Tracker check | A task or feature body without the seven sections and three test-plan parts; a test plan naming no negative control | Complete the body; name the planted defect |
| `tracker-label-missing`, `tracker-frontier-without-pilot` | Tracker check | A bead without its milestone or subsystem label; a Frontier or Moonshot bead that depends on no pilot | Add the label; add the pilot bead and the dependency |
| `tracker-later-milestone-dep`, `tracker-see-section` | Tracker check (warnings; the second fails from M1) | A dependency on a later milestone's bead; a body line that is only a pointer to a plan section | Move the bead or the dependency; write the content |
| `fallback-without-adr`, `superseded-without-adr`, `dependent-not-reread` | Lifecycle check (N41-17) | A `fallback` or `superseded` close with no ADR or a fallback name that differs from the kill gate's record; a dependent of a `fallback` close with no re-read note | Record the ADR; re-read the dependent and note `kept` or close it `superseded` |
| `not-ready-no-pilot`, `not-ready-no-approval` | Lifecycle check | A Frontier or Moonshot bead in progress with no pilot result; an over-envelope run with no approval cited | Run the pilot; obtain and cite the approval, or run the fallback form |
| `split-rule-broken` | Lifecycle check | Children that do not inherit the parent's dependencies; a parent closed before its last child; a discovered bead that is not an ancestor of its exit review | Correct the dependencies; reopen the parent; add the dependency to the exit review |
| `exit-review-implements`, `upstream-without-mirror`, `status-from-bead-states`, `partial-clauses-unstated` | Lifecycle check | An exit-review bead with implementation log lines; an `upstream` bead with no registry record or mirror; a status generator that reads bead states; a partial capability bead that does not list the open conditions | Move the work to its own bead; register and mirror; read the gate register; list the conditions |
| `session-record-invalid`, `session-silent-retry`, `session-sibling-changed` | First-session closing step (N41-25) | A step record without commands, digests or a verdict; a pass after a failed attempt with no recorded change; a sibling repository changed outside the upstream work item of step 3 | Re-record the step; state what changed, or record the failure; revert and open the work item |
| `done-clause-invalid` | Done checker (N41-26) | A clause naming an unknown gate, or a gate whose only bead is a Moonshot | Correct the clause; no Moonshot is required for done |
| `release-rule-1` to `release-rule-9` | Release checker (N41-27) | In order: a cell's status differs from the computed eight conditions; no demo, a demo over 60 s or citing a claim outside the gate register; a status table not generated; a name broader than the qualifying fixture; no reproducible losing result; a performance figure without its disclosure block or a ratio without the protocol; an export without the flags or documentation contradicting them; no exit record by a fresh-context session; a report without its reality line | Per rule: correct the status or list the missing conditions; add or shorten the demo; regenerate; name the capability with its limit; add the losing result with commands and seeds; add the block and the protocol record; restore the flags; include the exit record; print `no-claim` or name the calibrated cell |
| `rubric-not-offered`, `rubric-unmet`, `rubric-forbidden-statement`, `rubric-withdrawal-omitted` | Rubric check (N41-28) | A cell for a capability the rubric does not offer at that milestone; a "must be true" entry not satisfied; a statement that must not be made while its lifting condition is false; a withdrawn capability missing from the table | Remove the cell; satisfy or record the entry; remove the statement or produce the evidence; list the capability as withdrawn with its ADR |
| `release-evidence-kind` | `xtask check-contracts` (N41-29) | The plan's refusal "release cell lacks evidence of the kind it declares": no kind or an inconsistent one; a receipt of the wrong class; a fallback on a claimed stage; a cell scheduled ahead of its evidence | Declare the kind the cell's text implies; cite a re-measured row or a device receipt with no fallback; move the cell to the milestone at which its evidence exists |
| `release-disposition` | `xtask check-contracts` (N41-29) | A disposition that differs from the study's verdict; a `tied`, `infeasible`, `incomparable` or `inconclusive` study listed as a capability or reworded | Copy the verdict; list the cell as a result under its own name |

Reused, not redefined (owned by the verification chapter's vocabulary): `self-certified` (a reviewer or recorder
who implemented the work), `untyped`, `no-output`, `self-closed`, `shared-reason`, `shared-output`, `burst` (the
close audit, on which the lifecycle check stands), `views-differ` (a generated file edited by hand).

**Existing public codes that this unit's rules, plants and defaults rely on** (meaning as defined by the owner;
exit 3, a refused request, unless stated).

| Code | Meaning | Where this unit relies on it |
|---|---|---|
| FROB-REF-113 | An actuator card with a required field of class `assumed` under `--strict-data` | The default of the catalogue questions (OPEN-SINEW-1); N41-13 |
| FROB-REF-310, -311, -312 | Export of a research-only or private asset; a scene group that straddles a split; a realism claim citing a scene with `assumed` dimensions | Plants of RISK-11, RISK-12, RISK-44; the default of OPEN-HAB-1 |
| FROB-REF-447 | A Reality claim resting on a sensor or optical card field of class `assumed` | The default of OPEN-SENS-4 |
| FROB-REF-533, -534, -537, -540, -545 | Paired statistics under an uncontrolled determinism class; a family policy outside its class; a rung used without a rank-agreement record; an unreliable gradient; a rung asked to select where its mode is not `selective` | Plants of RISK-04, RISK-28, RISK-27, RISK-47; the default of OPEN-PROG-4 (every cheap rung `uninformative`) |
| FROB-REF-704, -761 | A campaign-class job without an approved envelope; a job that exceeds the envelope in force | The default of OPEN-SPARK-2 and of every envelope question; RISK-10; readiness of over-envelope beads |
| FROB-REF-741, -760 | More than eight buckets needed to hold padding waste at the limit; the episodes a claim needs exceed the budget | The defaults of OPEN-PHYLUM-1 and OPEN-NEMESIS-3; the plant of RISK-29 |
| FROB-REF-813, -869 | A deletion that would remove a pinned object; citing a claim whose evidence closure is dangling or tombstoned | RISK-39; the "record is real" clause of done |
| FROB-REF-815 | Closing a bead without a resolving log line from another session | The lifecycle check's close rule |
| FROB-REF-816, -833, -860 | Authorization flags missing or not false; an export without the flags | Release rule 7; the "honesty is real" clause |
| FROB-REF-818 | A priced parts list without a pinned, licensed catalogue | The defaults of OPEN-SKEL-2 and OPEN-FOUNDRY-2 |
| FROB-REF-819, -834 | A result cited or presented above its claim product or above its weakest input | Release rules 1 and 4 |
| FROB-REF-822 | A `capacity-biased` comparison cited as a verdict | The plant of RISK-40 |
| FROB-REF-826 | A certificate whose quantifier prefix does not match the query | The plant of RISK-35 |
| FROB-REF-840, -841, -845 | Sealed confirmation data asked for without the administrative scope (exit 5); a request that differs from its commitment; a result cited as qualified without a promoting receipt | The default of OPEN-ARENA-3; plants of RISK-34 and RISK-44 |
| FROB-REF-862, -863, -864, -866 | A performance figure without its disclosure block; a ratio without a paired lower bound; a device result without a current check; a speed experiment with a ceiling below its goal | Release rule 6; plants of RISK-25 and RISK-48; the first session's incumbent step |
| FROB-REF-873, -879, -889 | A dataset lacking licence, provenance, rights or uncertainty; a fit on a parameter set that fails identifiability; a band for an uncalibrated cell | The default of OPEN-GAUNT-1; plants of RISK-42 and RISK-43; release rule 9 |

**Refusals used under provisional names by this unit's seed beads**, awaiting codes from their owners:
`randomize-no-interval` (randomization requested on a card field with no interval; BD-200; the learning chapter's
registry) and `prediction-after-outcome`, `prediction-changed-after-freeze` (BD-344; the studio chapter's
registry).

## Acceptance criteria

- [ ] `reasons.toml` holds every harness reason above with meaning, ranked repairs and exit convention; the
      enumeration and the file agree.
- [ ] Every reason printed by a tool of this unit is registered, and every registered reason is produced by at
      least one planted case of N41-CX or by a plant state of N41-32.
- [ ] Every public code cited above exists in `registries/refusals.toml` with the meaning quoted; a cited code
      that its owner has changed or retired fails the cross-check.
- [ ] No tool of this unit emits a public code of its own or matches on prose.
- [ ] The three provisional names are listed as pending and the check fails once their owners have assigned
      codes and the seed beads still use the names.

## Test plan

### Unit tests
`crates/frob-conformance/tests/reasons_delivery.rs`: `every_reason_registered`; `every_reason_reachable` (maps
each reason to the planted case that produces it); `reason_line_is_first_line` (a failing tool prints
`reason=<id>` first); `exit_conventions` (a refusing command exits 3; a warning exits 0; an `xtask` check exits
non-zero); `cited_public_codes_exist_with_meaning`; `no_prose_matching` (a source scan of this unit's scripts for
string comparisons against message text); `provisional_names_pending`.

### End-to-end script and logging
No script of its own. Every end-to-end script of this unit logs harness reasons and public codes in `refusal`
events with the identifier, the tool, the trigger's subject (row, question, bead, cell) and the first ranked
repair; the log linter rejects a `refusal` event whose identifier is not registered.

### Known answers and negative controls
Known answers: the reason set above; nine release-rule reasons; the reused reasons of the close audit. Negative
controls: a tool that prints an unregistered reason; a registered reason that no planted case can produce; a
script that greps for message text; a cited public code that does not exist. Determinism: exact.

## Considerations

- The plan states one refusal in these chapters in words and assigns it no code: "release cell lacks evidence of
  the kind it declares". It is raised by a workspace check on a manifest, not by a verb, so it is a harness
  reason (`release-evidence-kind`); if release manifests ever pass through a verb, the workspace chapter's
  registry should assign it a public code.
- The lifecycle check and the verification chapter's close audit inspect the same close records. The close audit
  owns the reasons about the reason text and the recording session; the lifecycle check adds the reasons about
  kill gates, readiness, splitting and exit reviews. One pass over the export produces both lists.
- Repairs are ranked so that the first is the one that keeps a claim honest (record the failure, take the
  default, list the cell as a result) and never the one that makes the message go away.
- Public codes cited here are quoted from the registry as printed; their owners govern their meaning.

## References

§41.4, §42.1, §42.3 (defaults that quote refusal codes), §45.11, §46, §47.2, §47.4 (the uncoded refusal),
§38.11; Appendix O (the printed registry, which has no row for these chapters).
