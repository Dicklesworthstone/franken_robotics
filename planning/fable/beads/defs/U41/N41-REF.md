---
key: "N41-REF"
title: "frob-conformance: refusal vocabulary of the delivery registers: harness reasons and the public codes relied on"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "EPIC-M0"
deps: ["BD-005", "BD-025", "N38-REF", "STD-E2E", "STD-TEST"]
needs: []
related: ["N41-05", "N41-27", "N28cx-01", "N23bx-03", "N40-35"]
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
| `risk-dangling-ref` | Risk loader | A gate or bead named by a row does not exist, or the bead is superseded (plain load); an open question named by a row does not exist (`risks check --cross`) | Correct the identifier; create the missing record first |
| `risk-field-invalid` | Risk loader | A `severity` or `status` outside its closed set; a retired row with no `retired_by`; a plant whose `due` is earlier than the row's `first_readable` | Use a value of the closed set; name the gate or ADR that retired the row; move `due` to the first-readable milestone or later |
| `risk-without-plant` | Risk loader | A row with no `planted` field | Declare the plant with slug, text and due milestone |
| `risk-no-reading` | Risk review (N41-01) | A selected row has no `fired` reading at an exit | Record the reading with the observed value and its artifact |
| `risk-fired-no-outcome` | Risk review | A fired signal with none of the three outcomes; or an outcome naming a kill-gate decision or ADR that does not exist | Record the kill-gate decision, the ADR, or a written reason naming the evidence |
| `plant-declared-past-due`, `plant-missed`, `plant-stale`, `plant-unexpected`, `plant-not-run` | Plant runner (N41-32, N41-37) | A plant still declared at or after its due exit; a detector that did not report the signal; a fixture that cannot be planted; a detector that failed for another reason; a plant recorded on neither machine | Bind the plant; fix the detector, never the plant's expectation, unless the detector's observable changed by design; rebuild the fixture; run on the machine that has the device |
| `plant-stub-detector` | Plant runner | The detector's live-dependency proof fails (a stand-in) | Fire against the real check |
| `premortem-warning-unanswered` | Exit runner (N41-PM), as a block of the attached check `premortem` | An outcome-level reason is adverse and the exit record has no written answer | Answer in the exit record; record the measurement if it exists |
| `open-rule-broken` | Open-question loader (N41-03); the owner calendar's loader (N40-35) | A row with an empty owner, default, decider or deadline; a `deadline` that is not a milestone, `none` or `per-row`, or `none` on a row that blocks something, or `per-row` with no `tracked_by`; an `asked_at` that is not the milestone before the deadline; an identifier outside the scheme; counts that do not add to 161 or one of the four rows settled at entry not in its status; a calendar row naming no question row, copying a field the question's row owns, or missing for an owner's question | Complete the row; move the commentary to `note`; correct the count in the same change that adds or closes a question; add the calendar row at or before the deadline |
| `open-default-adds-claim` | Open-question loader | A default classified as adding a claim or capability | Rewrite the default so that it removes one |
| `open-closed-without-record` | Open-question loader | A measurement-decided question marked decided with no gate, pilot or ledger record | Take the measurement; until then leave it open or take the default |
| `open-id-reused` | Open-question loader | A retired or decided identifier reused; an alias resolving to more than one row | Give the new question a new identifier |
| `open-no-decision-bead` | Open-question loader | A blocking row with no decision bead, or a pending marker still set at the row's asked-at milestone | Name the bead; write it if it does not exist |
| `open-decided-by-silence` | Exit runner (N41-03, with the owner calendar) | A question at its deadline with no decision record and no default ADR | Record the owner's answer, or run `open default` to take the default as an ADR |
| `open-default-blocks-exit` | Exit runner: the owner calendar's check names the question from M0 (N40-35, check f); N41-15 states the rule for the ten-fold envelope at the real M3 exit | A gate due at the exit is `not-run` and a question at its default lists that gate in `blocks` (the one case in the register: the ten-fold envelope at M3 with no stratum of the partial gate recorded). The block is the gate query's; this reason names the default behind it | Record the strata that fit the default class and mark the others `not run` with the reason (a recordable outcome); or record the approval and run; or stay in the milestone |
| `tracker-cycle`, `tracker-unresolved-key`, `tracker-orphan`, `tracker-exit-chain-broken` | Tracker check (N41-16) | A dependency cycle; a key that does not exist; a bead with no parent or not an ancestor of its exit review; a broken chain of exit reviews | Remove the edge; fix the key; set the parent and add the bead to its exit review's ancestry; restore the chain |
| `tracker-gate-uncarried`, `tracker-part-without-bead` | Tracker check | A gate in the register cited by no bead; a gate that closes in parts with a part no bead carries | Add the bead, or attach the gate to the bead that measures it |
| `tracker-no-test-plan`, `tracker-no-negative-control` | Tracker check | A task or feature body without the seven sections and three test-plan parts; a test plan naming no negative control | Complete the body; name the planted defect |
| `tracker-label-missing`, `tracker-frontier-without-pilot` | Tracker check | A bead without its milestone or subsystem label; a Frontier or Moonshot bead that depends on no pilot | Add the label; add the pilot bead and the dependency |
| `tracker-later-milestone-dep`, `tracker-see-section` | Tracker check (warnings; the second fails from M1) | A dependency on a later milestone's bead; a body line that is only a pointer to a plan section | Move the bead or the dependency; write the content |
| `tracker-exit-waits-on-upstream` | Tracker check (N41-16, check 3(d); a warning at every exit) | An exit review that has among its ancestors the bead of an upstream item other than FD-ASUP-01 and FD-FMCP-01, listed with one path to it | Move the dependency to the exit review's `related`, where check 3(b) exempts it; or record why the item is a real prerequisite |
| `fallback-without-adr`, `superseded-without-adr`, `dependent-not-reread` | Lifecycle check (N41-17) | A `fallback` or `superseded` close with no ADR or a fallback name that differs from the kill gate's record; a dependent of a `fallback` close with no re-read note | Record the ADR; re-read the dependent and note `kept` or close it `superseded` |
| `not-ready-no-pilot`, `not-ready-no-approval` | Lifecycle check | A Frontier or Moonshot bead in progress with no pilot result; an over-envelope run with no approval cited | Run the pilot; obtain and cite the approval, or run the fallback form |
| `split-rule-broken` | Lifecycle check | Children that do not inherit the parent's dependencies; a parent closed before its last child; a discovered bead that is not an ancestor of its exit review | Correct the dependencies; reopen the parent; add the dependency to the exit review |
| `exit-review-implements`, `status-from-bead-states`, `partial-clauses-unstated` | Lifecycle check | An exit-review bead with implementation log lines; a status generator that reads bead states; a partial capability bead that does not list the open conditions | Move the work to its own bead; read the gate register; list the conditions |
| `implemented-while-gate-open` | Lifecycle check (rule 1) | A record of the capability registry `registries/capabilities.toml` (N00-02) printed `implemented` while a bead that cites its gate is open | Set the record back to `partial` and list the conditions that stay open; print `implemented` when the bead that closes the gate is closed |
| `unresolved-claim` | Lifecycle check (rule 2) | An identifier in a `pass` close line's `claim_ids` that resolves to no claim node of the artifact graph (BD-095) whose evidence closure contains the cited test's receipt, or that resolves to a claim of another bead | Cite the claim the bead's own test produced; re-run the test so that its receipt enters the claim's evidence closure |
| `withdrawn-without-decision`, `recorded-close-invalid` | Lifecycle check (rule 5) | A `withdrawn` close citing a decision that does not exist or does not say what the close says (an ADR whose entry in force has another outcome, a `[[pilot.result]]` whose verdict is not `falsifier-holds`, an envelope row whose status on the close date was not `requested` or `declined`); a `recorded` close whose gate is in no kill row's `reads`, or that cites no gate-record line of that gate with verdict `fail` recorded outside the implementing sessions with a resolving receipt listing every control `caught`, or, where the trigger names a repair cycle or an optimization pass, no earlier `note` line opening the cycle | Cite the decision that took this branch, or reopen the bead; cite the `fail` line recorded by another session with its receipt and the note that opened the cycle, or close the bead with the type its evidence supports |
| `wp-check-not-run` | Lifecycle check (rule 6) | No report of the work-package script of N44-04 exists for the export the check reads | Run the work-package script on the export |
| `upstream-pass-not-landed` | Lifecycle check (rule 10) | An `upstream` bead closed `pass` whose record in `registries/upstream.toml` is not `landed` or `adopted`, or whose closing line does not cite the sibling commit and the sibling gate run that accepted it | Land the change through the sibling's own gates and cite its commit and gate run; until then the bead stays open under its interim posture |
| `session-record-invalid`, `session-silent-retry`, `session-sibling-changed` | First-session closing step (N41-25), the step-3 record (N41-20) and the step-6 and step-7 records (N41-23, N41-24) | A step record without commands, digests or a verdict; a step-6 record that holds a figure for this product's speed, or whose verdict is other than `recorded`; a step-7 record whose fixture identity differs from the one recomputed from the fixture's constants; a pass after a failed attempt with no recorded change; a sibling repository changed outside the upstream work item of step 3 (among them a step-3 record whose sibling head or tree hash differs before and after the step: N41-20's validator case `siblings_unchanged_by_step`, planted by X41-36) | Re-record the step; state what changed, or record the failure; revert and open the work item |
| `done-clause-invalid` | Done checker (N41-26), and the one block of its attached check `done-report` of the exit runner (N40-05) | A clause naming an unknown gate, or a gate whose only bead is a Moonshot (the clause registry does not load) | Correct the clause; no Moonshot is required for done |
| `release-rule-1` to `release-rule-9` | Release checker (N41-27) | In order: a cell's status differs from the computed eight conditions; no demo, a demo over 60 s or citing a claim outside the gate register; a status table not generated; a name broader than the qualifying fixture; no reproducible losing result; a performance figure without its disclosure block or a ratio without the protocol; an export without the flags or documentation contradicting them; no exit record by a fresh-context session; a report without its reality line | Per rule: correct the status or list the missing conditions; add or shorten the demo; regenerate; name the capability with its limit; add the losing result with commands and seeds; add the block and the protocol record; restore the flags; include the exit record; print `no-claim` or name the calibrated cell |
| `release-visibility` | Release checker (N41-27), running the visibility guard `scripts/release/visibility-guard.sh` of N41-05 before the nine rules | A public remote under a private OPEN-PROG-1 decision; a `LICENSE` identifier line that differs from the decision; or the reading `unverified` (no network) at a release (at an exit the guard's `unverified` is a warning, exit 0, and only a mismatch blocks); the refusal prints the decision in force and the first mismatch | Align the remote's visibility or the `LICENSE` file with ADR-PROG-01; run the checker with network so the guard can verify |
| `rubric-not-offered`, `rubric-unmet`, `rubric-forbidden-statement`, `rubric-withdrawal-omitted` | Rubric check (N41-28) | A cell for a capability the rubric does not offer at that milestone; a "must be true" entry not satisfied; a statement that must not be made while its lifting condition is false; a withdrawn capability missing from the table | Remove the cell; satisfy or record the entry; remove the statement or produce the evidence; list the capability as withdrawn with its ADR |
| `release-evidence-kind` | `xtask check-contracts` (N41-29) | The plan's refusal "release cell lacks evidence of the kind it declares": no kind or an inconsistent one; a receipt of the wrong class; a fallback on a claimed stage; a cell scheduled ahead of its evidence | Declare the kind the cell's text implies; cite a re-measured row or a device receipt with no fallback; move the cell to the milestone at which its evidence exists |
| `release-disposition` | `xtask check-contracts` (N41-29) | A disposition that differs from the study's verdict; a `tied`, `infeasible`, `incomparable` or `inconclusive` study listed as a capability or reworded | Copy the verdict; list the cell as a result under its own name |

Reused, not redefined (owned by the verification chapter's vocabulary): `self-certified` (a reviewer or recorder
who implemented the work), `untyped`, `foreign-test`, `no-output`, `self-closed`, `shared-reason`,
`shared-output`, `burst` (the close audit of N38b-03, on which the lifecycle check stands and whose findings it
copies), `views-differ` (a generated file edited by hand),
`suspect-undisposed` (a control never caught in recorded history, with no disposition at an exit: the review that
N41-37's sweep feeds, and the reason a row of N41-CX left `declared` past its `due` milestone fails an exit).

Retired and never reused: `upstream-without-mirror`. The presence of an `upstream` bead's record and mirror is
check H-5 of N00-08, and the close of an `upstream` bead is `upstream-pass-not-landed` above; no tool of this
unit emits the old reason, so `every_reason_reachable` does not count it and `every_reason_registered` fails a
tool that prints it.

**Existing public codes that this unit's rules, plants and defaults rely on** (meaning as defined by the owner;
exit 3, a refused request, unless stated: budget codes exit 4 and capability codes exit 5, as marked).

| Code | Meaning | Where this unit relies on it |
|---|---|---|
| FROB-REF-113 | An actuator card with a required field of class `assumed` under `--strict-data` | The default of the catalogue questions (OPEN-SINEW-1); N41-13 |
| FROB-REF-310 | A scene pack would export a research-only or private asset (raised only by the pack export check, N19b-02; a report, a still, weights or any other derived artifact is refused at the export door with 731 or 733 instead) | The plant of RISK-11 |
| FROB-REF-311, -312 | A scene group that straddles a split; a realism claim citing a scene with `assumed` dimensions | The plant of RISK-44; the default of OPEN-HAB-1 |
| FROB-REF-731, -733 | A private object, or anything derived from one, without the private scope and flag; the licence set of the artifact forbids this export (both exit 5, capability) | The plant of RISK-12 (a still whose lineage reaches a private root); the default of OPEN-SAFE-2 (weights carry the mark and the licence set) |
| FROB-REF-447 | A Reality claim resting on a sensor or optical card field of class `assumed` | The default of OPEN-SENS-4 |
| FROB-REF-533, -534, -537, -540, -545 | Paired statistics under an uncontrolled determinism class; a family policy outside its class; a rung used without a rank-agreement record; an unreliable gradient; a rung asked to select where its mode is not `selective` | Plants of RISK-04, RISK-28, RISK-27, RISK-47; the default of OPEN-PROG-4 (every cheap rung `uninformative`) |
| FROB-REF-704, -761 | A campaign-class job without an approved envelope; a job that exceeds the envelope in force (both exit 4, budget: refused at admission) | The default of OPEN-SPARK-2 and of every envelope question; RISK-10 and part (iii) of RISK-28; the first session's overrun rule; readiness of over-envelope beads |
| FROB-REF-741, -760 | More than eight buckets needed to hold padding waste at the limit; the episodes a claim needs exceed the budget (both exit 4, budget) | The defaults of OPEN-PHYLUM-1 and OPEN-NEMESIS-3; the plant of RISK-29 |
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

**Refusals used under provisional names by this unit's seed beads**, and where each now stands in the
allocation ledger `registries/refusal_allocations.toml` (the rule of N35-33; this bead reads the status and
assigns nothing):

| Name | Raised by | Owner of the decision | Ledger status | What the seed bead raises meanwhile |
|---|---|---|---|---|
| `randomize-no-interval` (randomization requested on a card field with no interval) | BD-200 | N23bx-03, for the learning chapter's registry | A first-round request of N35-33 (`5xx`, proposed 571, filed by N23bx-03) | The name behind one constant, replaced by the assigned number in one line |
| `prediction-changed-after-freeze` | BD-344 | N28cx-01, for the studio chapter's registry | `merged` into FROB-REF-883 (its studio site, pointer `prediction`) | FROB-REF-883 with `pointer: prediction` (BD-344); no longer a provisional name |
| `prediction-after-outcome` | BD-344 | N28cx-01 | `held` (an `8xx` request under the hold of N35x-01) | The interim FROB-REF-883 with `detail = "registration-after-outcome"` behind one constant |

## Acceptance criteria

- [ ] `reasons.toml` holds every harness reason above with meaning, ranked repairs and exit convention; the
      enumeration and the file agree.
- [ ] Every reason printed by a tool of this unit is registered, and every registered reason is produced by at
      least one planted case: a row of N41-CX, a plant state of N41-32, or a negative control named in the test
      plan of the tool's own bead (for example `risk-field-invalid` by `invalid_field_rejected` of N41-01, and
      `open-default-blocks-exit` at M0 by `default_behind_not_run_gate_named` of N40-35).
- [ ] Every public code cited above exists in `registries/refusals.toml` with the meaning quoted; a cited code
      that its owner has changed or retired fails the cross-check.
- [ ] No tool of this unit emits a public code of its own or matches on prose.
- [ ] `scripts/e2e/delivery_reasons.sh` passes; its log shows the planted missing record caught and zero
      unregistered identifiers in the scanned logs.
- [ ] The three provisional names are listed with their ledger status: a `requested` or `held` name is
      `pending` with its interim form; the check fails when a name whose request is `assigned` or `merged` is
      still emitted by its seed bead (so `prediction-changed-after-freeze`, merged into 883, already fails if
      BD-344 emits the name instead of 883 with `pointer: prediction`).

## Test plan

### Unit tests
`crates/frob-conformance/tests/reasons_delivery.rs`: `every_reason_registered`; `every_reason_reachable` (maps
each reason to the planted case that produces it); `reason_line_is_first_line` (a failing tool prints
`reason=<id>` first); `exit_conventions` (a refusing command exits 3; a warning exits 0; an `xtask` check exits
non-zero); `release_visibility_registered` (the reason is present with tool N41-27 and exit 3; the guard's
three outcomes map to it: a public remote under a private decision and a differing `LICENSE` at both an exit
and a release, `unverified` at a release only); `cited_public_codes_exist_with_meaning` (with the exit status stated here: 704, 741, 760 and 761 exit
4; 731, 733 and 840 exit 5; the rest exit 3); `code_310_cited_only_for_scene_packs`; `no_prose_matching` (a source scan of this unit's scripts for
string comparisons against message text); `provisional_names_pending`.

### End-to-end script and logging
Every end-to-end script of this unit logs harness reasons and public codes in `refusal` events with the
identifier, the tool, the trigger's subject (row, question, bead, cell) and the first ranked repair; the log
linter rejects a `refusal` event whose identifier is not registered.

This bead's script, `scripts/e2e/delivery_reasons.sh` (sources the STD-E2E helper; writes
`target/e2e-logs/delivery_reasons/<utc>.ndjson` and a transcript; idempotent; no network needed): (1) runs
`FROB_TEST_SEED=0 cargo test -p frob-conformance --test reasons_delivery` and asserts exit 0; (2) runs the same
target against a scratch copy of `reasons.toml` from which `release-visibility` is removed and asserts exit
101 with the missing id on the first failure line; (3) for every reason of the table, reads the newest log of
the script or test that produces it (the planted case named by `every_reason_reachable`) and records `checked`,
or `pending: <bead> (<milestone>)` while that bead is later than the milestone under test, never `pass`; (4)
scans the `refusal` events of this unit's scripts under `target/e2e-logs/` for unregistered identifiers. Each
`cmd` line carries the exact command, exit code and wall time; each check is an `assert` line with expected and
found values; the `summary` line counts reasons registered, reached, pending and unknown (threshold 0), and the
provisional names by ledger status.

### Known answers and negative controls
Known answers: the reason set above; nine release-rule reasons and `release-visibility`, checked before them;
the reused reasons of the close audit. Negative
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
