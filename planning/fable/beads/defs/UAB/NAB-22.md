---
key: "NAB-22"
title: "frob-conformance: milestone-exit sweep of the decision records: re-score rule, due list, record, check"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "decision-record", "test"]
parent: "EPIC-M0"
deps: ["NAB-17", "NAB-21", "NAB-32", "NAB-42", "NAB-09", "NAB-10", "BD-025", "STD-TEST"]
needs: ["§40.6: the milestone exit as a query over the gate register, and its report `docs/delivery/exits/M<n>/gate-report.json` (N40-04)", "§40.4: the schedule of kill decisions due at each exit and the decision ADRs (N40-30, N40-03)", "§0.8: the hypothesis register and its append-only change rule (N00-10)", "§0.7, §38.5: the pilot ledger with pilot results and the count of running Moonshots (N00-07)", "§40.5, §45.11: the exit protocol and exit record `docs/delivery/exits/M<n>/EXIT.md`, which should list the sweep record as an input of the next milestone (N40-05)"]
related: ["NAB-18", "NAB-19", "NAB-20", "NAB-23", "NAB-24", "NAB-25", "NAB-26", "NAB-27", "NAB-28", "NAB-29", "NAB-30", "NAB-31", "NAB-50", "N40-04", "N40-05", "N40-30", "N00-10"]
gates: []
plan_refs: ["Appendix C.1", "Appendix C.2 (preamble)", "§0.8", "§0.7", "§40.6", "§43"]
---
## Background

FrankenRobotics keeps five decision records as data: the expected-value matrix (143 scored families of
mechanism, each adopted, admitted, parked, deferred or rejected), the Moonshot queue (21 waiting items under a
cap of four active), the killed-idea register (67 ideas refuted or deflated, each with the condition that
would revive it), the idea-pass register (which idea lives where, and which are not yet placed) and the source
register (what was read, and what still needs a reading). All five were written **before any code existed**.
Every score is a judgment made on paper evidence, every "survives" range is a HYPOTHESIS, and the rule of the
program is that *a verdict changes only on new evidence of the kind that produced it: a pilot, a measurement at
a gate, or a primary source re-read at a pinned version*.

Milestone exits are when that evidence arrives. At each of the nine exits (M0 to M8) gates record measured
values, kill gates are decided, pilots report, and hypotheses change state. If the decision records are not
re-read against that evidence at the same moment, they rot: a family stays "adopted [F]" after its kill gate
fired, a parked family's reopening evidence arrives and nobody notices, a Moonshot slot stays occupied by a
dead item. This bead builds the mechanism that prevents it: the re-score rule, the *due list* (what must be
looked at, at which exit), the sweep record, and the check that a sweep was complete. The nine sweeps
themselves are NAB-23 to NAB-31, one per exit, each with its own due list.

Design decisions (DESIGN). (1) The sweep follows the exit review of its milestone and consumes its record; it
implements nothing and decides no gate. (2) A row with no new evidence is recorded as *looked at and
unchanged*, with the evidence that was looked at, so that "unchanged" and "not swept" differ. (3) A score may
not change without evidence; an entry with empty evidence is refused. (4) The sweep is done by a session that
did not implement the mechanisms whose rows it re-scores, the same independence rule as an exit review, because
the builder of a mechanism is the party most tempted to raise its confidence. Rejected: re-scoring
continuously as beads close (a closed bead is not acceptance; only a gate record is); and re-scoring only at
the end (the matrix would then describe a program that no longer exists). Tier: Solid.

## Scope

In: the re-score rule; the due-list generator; the sweep record format and its generated report; the
completeness check; the tool. Out: the nine sweeps (NAB-23 to NAB-31); the registries themselves; the exit
reviews (seed beads) and the exit protocol (N40-05); deciding kill gates (the decision beads of the owning
units).

## Technical approach

**1. The re-score rule.** For a family record of `registries/ev_matrix.toml`:

1. *Evidence classes that may move a factor.* **Confidence** moves only with a recorded gate value, a kill-gate
   decision, a pilot result, or a changed hypothesis verdict that names the family. **Impact** moves only with a
   measured phase share or ceiling (for example a measured share s of a loop phase, which bounds a mechanism's
   benefit at 1/(1 − s)), or with a gate that shows the family changed or did not change decisions. **Effort**
   moves only with measured effort against the crate's scope cap (a crate past 115% of its cap is the trigger),
   or with an upstream work item landing or being refused.
2. *Direction is not prescribed.* A passed gate usually raises confidence and a fired kill gate lowers it, but
   the sweep writes the judgment and its two-line reason; the lint checks only that evidence is cited.
3. *A fired kill gate.* The family's `decision_text` gains "fell back to <fallback>" and the family is described
   by the fallback's name everywhere. If the family was Frontier-only and the rule of the matrix says so (kernel
   reflection), the row moves to the parked table by an entry `move = "frontier-moonshot → parked"`.
4. *A parked or rejected family whose `reopen_on` evidence has arrived.* The sweep does not move the row. It
   opens a proposal (an ADR draft naming the evidence and the killed-idea record) for the owner; until the ADR
   is accepted the row stays where it is.
5. *After re-scoring,* rule 1 of the matrix is re-checked: a family now below 2.0 that is neither a Moonshot nor
   needed by something else is proposed for parking.
6. Each change appends one `[[family.history]]` entry: `exit`, the three factors, `score`, `decision`,
   `reason`, `evidence = [gate record ids, ADR ids, hypothesis ids, pilot ids]`, `session`. An unchanged row
   appends `{ exit, unchanged = true, looked_at = [...] }`.

**2. The due list.** `due(exit) -> DueList`, computed from the registries and the gate register, never typed by
hand:

- *Families:* every family with a kill gate whose deciding milestone in the gate register is `exit`, or a gate
  recorded for the first time at `exit`, or a bet or hypothesis whose state changed since the previous exit.
- *Killed ideas:* every `KI` record whose "revive only if" names a gate, kill gate or measurement due at
  `exit`.
- *Queue:* every Moonshot-queue item whose earliest milestone is `exit`, plus every active item.
- *Ideas:* every idea record whose carrier gates were decided at `exit`; every not-yet-placed idea with a
  trigger read at `exit`.
- *Sources:* every reading task attached to a gate due at `exit`; every `[[unverified]]` item whose settling
  gate is due at `exit`.
- *Glossary:* the unused-term and allow-file reports of the glossary lint.

**3. The sweep record.** `docs/delivery/exits/M<n>/decision-records.json` (schema `frob.decision-sweep/1`) and
a generated `decision-records.md` beside the exit record:

```
{ "exit": "M2", "commit": "...", "session": "...", "implementer_sessions_excluded": true,
  "families":   [ { "id": "EV-F23", "due_because": ["KILL-PHYLUM", "KILL-BUCKET"], "outcome": "changed" |
                    "unchanged" | "fell-back" | "moved" | "proposal-opened", "history_ref": "...", "evidence": [...] } ],
  "killed_ideas": [ { "id": "KI-42", "condition_met": false, "measured": [...], "proposal": null } ],
  "queue":      { "freed": [...], "entered": [...], "active_after": n },
  "ideas":      [ { "id": "IP-12", "state": "placed" | "fell-back" | "not-placed" | "declined", "note": "" } ],
  "sources":    { "reading_tasks_open": n, "unverified_settled": [...] },
  "carried_forward": [ { "id": "...", "why": "gate recorded not-run", "to_exit": "M3" } ] }
```

**4. The completeness check** (`crates/frob-conformance/tests/decision_sweep.rs`): for every exit whose exit
record exists, (a) a sweep record exists; (b) every item of `due(exit)` appears in it with an outcome or in
`carried_forward` with a reason; (c) every `changed` outcome has a history entry with non-empty evidence that
resolves (gate records in the gate register, ADRs on disk, hypothesis ids in the hypothesis register); (d) no
family named by a fired kill gate still reads "adopted" without "fell back"; (e) the Moonshot cap holds after
`entered`; (f) the sweep's session is not among the implementer sessions of the beads whose gates it cites;
(g) no reading task is open for a gate recorded at this exit. Findings: `sweep.missing`, `sweep.row-not-swept`,
`sweep.change-without-evidence`, `sweep.kill-not-reflected`, `sweep.cap-exceeded`, `sweep.self-certified`,
`sweep.reading-task-open`, `sweep.proposal-applied-without-adr`.

**5. The tool.** `cargo run -p frob-conformance --bin decision-sweep -- due --exit M<n>` prints the due list
(Markdown and JSON); `… -- skeleton --exit M<n>` writes a sweep record with every due item at outcome `todo`;
`… -- check --exit M<n>` runs the completeness check. The binary holds no judgment and writes nothing but the
skeleton. It is not an `xtask` subcommand.

**6. Determinism and evidence.** The due list and the check are D0. The sweep record makes no claim about the
system; it is a decision record, and its changes are DESIGN judgments with FACT evidence attached.

## Acceptance criteria

- [ ] `decision-sweep due --exit Mn` prints, for the plan-time registries, the family lists of the test plan
      (1, 2, 7, 19, 21, 7, 8, 0 and 7 families for M0 to M8).
- [ ] A history entry with empty `evidence` is refused by the loader of the matrix, and `sweep.change-without-
      evidence` fires on a sweep record that claims a change without one.
- [ ] `check` fails with `sweep.row-not-swept` when one due family is absent from the record, and with
      `sweep.kill-not-reflected` when a fired kill gate's family still reads "adopted" with no fallback.
- [ ] `check` fails with `sweep.self-certified` when the sweep's session implemented a bead whose gate it cites.
- [ ] A proposal to reopen a parked family is recorded as `proposal-opened`; moving the row without an accepted
      ADR fails `sweep.proposal-applied-without-adr`.
- [ ] The generated `decision-records.md` lists changed, unchanged, fell-back, moved and carried-forward items
      with their evidence.

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs` (fixture registries and fixture gate registers under
`tests/fixtures/sweep/`, labelled `synthetic`; the real registries for the due-list known answers):

- `due_families_by_exit`: on the plan-time registries and the gate register's deciding milestones, the family
  ids due are: M0 `EV-S29`; M1 `EV-F04`, `EV-F19`; M2 `EV-S15`, `EV-S17`, `EV-S24`, `EV-S27`, `EV-S32`,
  `EV-F23`, `EV-F26`; M3 nineteen (`EV-S14`, `EV-S21`, `EV-S22`, `EV-S23`, `EV-S36`, `EV-S41`, `EV-F01`,
  `EV-F06`, `EV-F07`, `EV-F10`, `EV-F11`, `EV-F14`, `EV-F15`, `EV-F17`, `EV-F18`, `EV-F19`, `EV-F21`, `EV-F24`,
  `EV-F29`); M4 twenty-one; M5 seven; M6 eight; M7 none; M8 seven. (The full lists are in NAB-23 to NAB-31.)
- `row_due_twice_is_swept_twice`: `EV-F19` (M1 and M3), `EV-S41` (M3 and M4) and `EV-F11` (M3 and M4) appear in
  both due lists; 72 family-exit pairs cover 69 distinct families.
- `unchanged_needs_looked_at`; `change_needs_evidence`; `evidence_must_resolve`.
- `fired_kill_requires_fallback_text`; `kernel_reflection_moves_to_parked_on_kill`.
- `reopen_evidence_opens_proposal_not_move`.
- `below_two_after_rescore_is_proposed_for_parking`: a Frontier family re-scored from (4, 3, 4) = 3.0 to
  (4, 2, 5) = 1.6 with no Moonshot tier produces a parking proposal.
- `cap_after_entry`: four active plus one entered → `sweep.cap-exceeded`.
- `self_certified_sweep_is_refused`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) runs `decision-sweep due` for every exit, compares the family
counts with the known answers, writes a skeleton for a scratch exit, fills it from a fixture exit report and
runs `check`; then plants each finding. Logged per STD-E2E: per exit a `metric` with the count of due
families, killed ideas and queue items; per planted finding an `assert`.

### Known answers and negative controls

Known answers: due-family counts 1, 2, 7, 19, 21, 7, 8, 0, 7; 72 family-exit pairs over 69 distinct families
with a kill gate (13 Solid, 55 Frontier or Moonshot, 1 parked); re-score arithmetic (4, 3, 4) → "3.0" and
(4, 2, 5) → "1.6". Negative controls: each of the eight findings of step 4 as a planted defect; plus a sweep
record that marks every due row `unchanged` with empty `looked_at` (must fail). Determinism: the due list is
identical across two runs and both platforms.

## Considerations

- The sweep is bookkeeping, and bookkeeping is where process growth hides. It is bounded: the due list is
  computed, a sweep touches only due items, and its record is one file per exit. If a sweep takes more than a
  day, the due rule is too wide and is narrowed, not staffed.
- Scores stay judgments. The value of the sweep is not a better number; it is that every judgment in the
  matrix is dated, has evidence, and was last looked at when the evidence changed.
- M7 has no family due by a kill gate, because the tournament decides the thesis and not a mechanism; its sweep
  (NAB-30) is driven by the tournament's phase ledger instead.
- OPEN (owner: the program lead): whether the owner signs each sweep. Fail-closed default: a sweep is valid
  without a signature, and every row move still needs an accepted ADR.

## References

Appendix C.1 (the score and its rules); Appendix C.2 preamble (the reversal condition); §0.8 (a verdict changes
only on new evidence of the kind that produced it); §0.7; §40.6; §43.
