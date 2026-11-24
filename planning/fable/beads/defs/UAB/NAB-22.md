---
key: "NAB-22"
title: "frob-conformance: milestone-exit sweep of the decision records: re-score rule, due list, record, check"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "decision-record", "test"]
parent: "EPIC-M0"
deps: ["BD-025", "N00-10", "N38a-09", "N40-03", "N40-04", "N40-05", "NAB-09", "NAB-10", "NAB-17", "NAB-18", "NAB-20", "NAB-21", "NAB-32", "NAB-42", "STD-TEST"]
needs: []
related: []
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

Design decisions (DESIGN). (1) The sweep is made at the exit candidate commit, before the exit record: it reads
the milestone's gate records and kill-gate ADRs through the gate report of N40-04 at that commit, writes its
sweep record, and its `check` runs as an attached check of the exit runner (N40-05 section 5a), so that the exit
review reads the sweep record and copies its section into the exit record. It implements nothing and decides no
gate. Each sweep bead (NAB-23 to NAB-31) therefore depends on the decision beads it reads and on N40-04, and the
exit review of its milestone depends on the sweep, never the other way round (a sweep that waited for the exit
review would be the one bead of its milestone that is not an ancestor of the review). (2) A row with no new evidence is recorded as *looked at and
unchanged*, with the evidence that was looked at, so that "unchanged" and "not swept" differ. (3) A score may
not change without evidence; an entry with empty evidence is refused. (4) The sweep is done by a session that
did not implement the mechanisms whose rows it re-scores, the same independence rule as an exit review, because
the builder of a mechanism is the party most tempted to raise its confidence. Rejected: re-scoring
continuously as beads close (a closed bead is not acceptance; only a gate record is); and re-scoring only at
the end (the matrix would then describe a program that no longer exists). Tier: Solid.

## Scope

In: the re-score rule; the due-list generator; the sweep record format and its generated report; the
completeness check; the tool, declared as an operator binary of `frob-conformance`; its record in the exit
runner's table of attached checks. Out: the nine sweeps (NAB-23 to NAB-31); the registries themselves; the exit
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
   the sweep writes the judgment and its two-line reason; the check verifies that evidence is cited, resolves
   and is of a kind rule 1 allows for each factor changed, never the direction. Each evidence id is classified
   by where it resolves: a gate record or kill-gate ADR; a pilot; a hypothesis; a phase-share row (a ten-fold
   gate's phase ledger, or the share table a sweep stores); an effort record (a scope-cap measurement, or the
   schedule gate N40-11); an upstream work item. An Impact change needs a phase-share row or the record of a
   gate that the family's own row names; an Effort change needs an effort record or an upstream item. A gate
   record whose evidence is labelled `synthetic` (a rehearsal or fixture dataset) fits no factor of a family
   whose named gates have claim Reality: that lane rests at `no-claim` until real data is admitted.
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
  The deciding milestone is the first milestone named in the register's field ("M6, with GATE-HAB-07" is M6;
  "M4; the pilot runs before funding" is M4). Two register forms add re-reads that are listed under
  `due_because = ["re-read: <gate>"]` and are **not** counted in the kill-gate family counts below: a gate
  marked "Standing, from M<a>" (KILL-AUGUR-FALSECERT from M1, KILL-AUGUR-REUSE from M3) is counted at M<a>
  and re-read at every later exit; a gate marked "M<a>; re-read at M<b>" (KILL-MCP: M0, re-read at M1 with
  the M1 verbs) is counted at M<a> and re-read at M<b>. A gate marked "M<a>, then rolling" (KILL-FSIM, which
  has no family row and is recorded against the posture record KI-63) puts KI-63 on the killed-idea list of
  every exit from M<a>. At M7 every family of the `solid` and `frontier-moonshot` tables (100) is due for an
  Impact evaluation against the tournament's phase ledgers (`due_because = ["impact: tournament"]`, NAB-30),
  also outside the kill-gate counts. At M8 every family, killed-idea record, queue item, idea record and
  `[[unverified]]` item is due for a final state (`due_because = ["final"]`, NAB-31), also outside those
  counts.
- *Killed ideas:* every `KI` record whose "revive only if" names a gate, kill gate or measurement due at
  `exit`.
- *Queue:* every Moonshot-queue item whose earliest milestone is `exit`, plus every active item.
- *Ideas:* every idea record whose carrier gates were decided at `exit`; every not-yet-placed idea whose
  `trigger_exit` (NAB-42) is `exit`.
- *Sources:* every reading task attached to a gate due at `exit`; every `[[unverified]]` item whose settling
  gate is due at `exit`.
- *Glossary:* the unused-term and allow-file reports of the glossary lint.

**3. The sweep record.** `docs/delivery/exits/M<n>/decision-records.json` (schema `frob.decision-sweep/1`) and
a generated `decision-records.md` beside the exit record:

```
{ "exit": "M2", "commit": "...", "session": "...", "implementer_sessions_excluded": true,
  "families":   [ { "id": "EV-F23", "due_because": ["KILL-PHYLUM", "KILL-BUCKET"], "outcome": "changed" |
                    "unchanged" | "fell-back" | "moved" | "proposal-opened", "history_ref": "...", "evidence": [...] } ],
  "killed_ideas": [ { "id": "KI-42", "condition_met": false,
                    "threshold": [ { "name": "homogeneous-ratio-512", "op": ">=", "value": "0.5" } ],
                    "measured":  [ { "name": "homogeneous-ratio-512", "value": "0.41", "unit": "1",
                                     "receipt": "<gate record id>" } ], "proposal": null } ],
  "queue":      { "freed": [...], "entered": [...], "active_after": n,
                  "final": [ { "id": "MQ-01", "outcome": "passed", "reason": "", "evidence": [...] } ] },  # M8
  "ideas":      [ { "id": "IP-12", "state": "planned" | "in-progress" | "delivered" | "fell-back" |
                    "not-delivered", "note": "" } ],              # the state vocabulary of NAB-42
  "unplaced":   [ { "id": "UP-06", "trigger_met": false, "measured": [...],
                    "outcome": "trigger-not-met" | "proposal-opened" } ],   # conditional ideas of NAB-49
  "sources":    { "reading_tasks_open": n,
                  "unverified_settled": [ { "id": "NV-01", "status": "measured", "record": "<gate record id>" } ] },
  "shares":     [ { "id": "SH-M3-01", "family": "EV-S14", "phase": "checking", "arm": "ours", "stratum": "",
                    "share": 0.04, "ceiling": 1.042, "receipt": "<gate record id>" } ],   # optional
  "readers":    [ { "registry": "registries/ideas.toml", "readers_run": [...], "proposal": null } ],  # optional
  "carried_forward": [ { "id": "...", "why": "gate recorded not-run", "to_exit": "M3" } ] }
```

`threshold` is the killed-idea condition as numbers (the sweep bead's table states each one); every
threshold `name` needs a `measured` entry of the same name, and `condition_met` must equal the conjunction of
the comparisons, so a value measured on another quantity (a topology edit where the condition names parameter
edits) never meets it. A condition that has no number (a state such as "every backend reports 64-bit atomics")
is one threshold with `op = "=="` on a named Boolean.

`shares` is the share table a sweep stores (rule 1.2): one row per measured phase share, with `ceiling` =
1/(1 − `share`) to four figures and a `receipt` that resolves to the gate record of the phase ledger it was read
from (the incumbent design-loop ledger at M1, the partial ten-fold gate at M3, the tournament at M7). A share
row cited as evidence resolves to an `id` in the `shares` of this or an earlier sweep record. `readers` is the
reader check of a decision registry (NAB-26 step 7). Both are optional and empty at exits that store none; a
field outside this sketch fails the schema.

**4. The completeness check** (a library function, `frob_conformance::sweep::check(exit)`, run by the test
target `crates/frob-conformance/tests/decision_sweep.rs` for every exit whose exit record exists, and by the
binary's `check` for the one exit it names, at the candidate commit before the exit record is written): (a) a
sweep record exists; (b) every item of `due(exit)` appears in it with an outcome or in
`carried_forward` with a reason, and an `unchanged` outcome whose `looked_at` cites only gate records with
status `not-run` is `sweep.row-not-swept` (a row read only through unrun gates belongs in `carried_forward`),
and so is a `readers` entry with an empty `readers_run` and no `proposal`, and, at M8, a queue item with no
`final` entry, a `final` outcome outside the six of NAB-31 (the five terminal registry states and
`never-funded`), or `never-funded` with an empty `reason`; a `final` outcome `passed` whose `evidence` holds no
resolving gate record is `sweep.change-without-evidence`; (c) every `changed` outcome has a history entry with a non-empty `reason` (rule
1.2; an empty one is reported as `sweep.change-without-evidence`) and non-empty evidence that resolves (gate
records in the gate register, ADRs on disk, hypothesis ids in the hypothesis register, pilot ids in the pilot
register (a pilot whose claim is a speed ratio resolves only when its `ceiling` block, N38a-09, was recorded
before its first run record), share rows in the `shares` of this or an earlier sweep record, whose `ceiling` must
equal 1/(1 − `share`)) and whose kind fits each factor it changes (rule 1.2), and whose `reason` states a ratio
(a number with "times", "×" or "-fold", or the word "ten-fold") only as the measured value of a gate record in
its evidence (KI-60: no multiplier is printed except a gate's measured value); every killed-idea evaluation with
`condition_met` set cites `measured` values whose records resolve at this exit, and its `condition_met` equals
its `threshold` comparisons, with no `proposal` while `condition_met` is false; and every `unverified_settled`
entry has a `record` that resolves by the rule of the source register (NAB-09: a gate record for `measured`, a
source id with a P reading or a confirmed re-read for `read`, an ADR for `withdrawn`). Each failure of (c) is
`sweep.change-without-evidence`; (d) no
family named by a fired kill gate still reads "adopted" without "fell back", and one that rule 3 moves on a
fired kill gate (kernel reflection) has its `move` entry and is in the parked table, and no queue item whose
kill gate fired is in `entered` (`sweep.kill-not-reflected`); (e) the Moonshot cap holds after `entered`, and
no item is in `entered` while the item its `waits_for` names is not `passed`, and `entered` is a prefix of
`next_entries` at this exit (NAB-21) unless an accepted ADR reorders the queue (`sweep.cap-exceeded`, naming the
rule broken); (f) the sweep's session is not among the implementer sessions of the beads whose gates it cites;
(g) no reading task is open for a gate recorded at this exit. Findings: `sweep.missing`, `sweep.row-not-swept`,
`sweep.change-without-evidence`, `sweep.kill-not-reflected`, `sweep.cap-exceeded`, `sweep.self-certified`,
`sweep.reading-task-open`, `sweep.proposal-applied-without-adr`.

**5. The tool.** An operator binary of `frob-conformance`, declared by this bead under the crate's rule for
binaries (BD-025: `autobins = false`, so a binary exists only by declaration): a `[[bin]]` entry with
`name = "decision-sweep"` and `path = "src/bin/decision_sweep.rs"`, and a row in the binary table of the crate's
`CONTRACT.md` naming NAB-22 as its owner and stating what it reads (`registries/ev_matrix.toml`,
`registries/killed_ideas.toml`, `registries/ideas.toml`, `registries/sources.toml`, `registries/gates.toml`, the
hypothesis and pilot registers, the gate report of N40-04 and `docs/delivery/exits/`) and writes (only the
skeleton `docs/delivery/exits/M<n>/decision-records.json` when that file does not yet exist; never a row of
`registries/gates.toml`). Without the declaration the command-form lint of BD-025 refuses every
`--bin decision-sweep` line as `unknown-target`. Subcommands:
`cargo run -p frob-conformance --bin decision-sweep -- due --exit M<n>` prints the due list (Markdown, or JSON
with `--json`); `… -- skeleton --exit M<n>` writes a sweep record with every due item at outcome `todo`;
`… -- check --exit M<n> [--root <dir>] [--json]` runs the completeness check of step 4. With `--json`, `check`
follows the exit runner's check contract (N40-05 section 5a): it prints one object `{check: "decision-sweep",
milestone, section, blocks: [{id, reason}], warnings: [{id, text}]}`, where `section` is the Markdown of the
generated `decision-records.md` and every finding of step 4 is a block whose `reason` is the finding name; it
exits 0 whenever it ran and non-zero only when it could not run (an unreadable registry). `--root` names the
checkout the runner cloned. The binary holds no judgment and writes nothing but the skeleton. It is not an
`xtask` subcommand, not a `frob` verb and not an MCP tool.

**5a. Attachment to the exit runner.** This bead adds one record to `scripts/exit/checks.toml` (schema
`frob.exitchecks/1`, N40-05): `id = "decision-sweep"`, `owner = "NAB-22"`, `step = 8`, `since = "M0"`,
`command = "cargo run -p frob-conformance --bin decision-sweep -- check --exit"`, which the runner calls as
`<command> <Mn> --root <dir> --json`. Step 8 (DESIGN) is the procedure's step for kill decisions due, the
evidence most sweep entries rest on; it precedes the Moonshot-cap count of step 11, so an item the sweep entered
is counted by that check. A missing sweep record is the block `sweep.missing`, so no exit can be recorded without
its sweep.

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
- [ ] `decision-sweep` is a declared `[[bin]]` of `frob-conformance` with its row in the crate's `CONTRACT.md`;
      BD-025's `binaries_declared` and `no_conformance_command` tests pass with this bead's scripts calling it.
- [ ] `scripts/exit/checks.toml` holds the record of step 5a; `scripts/exit/review.sh M0` on a scratch clone runs
      it, copies its section into the exit record under `decision-sweep`, and with the M0 sweep record removed
      lists `sweep.missing` among the blocking parts.

## Test plan

### Unit tests

`crates/frob-conformance/tests/decision_sweep.rs` (fixture registries and fixture gate registers under
`tests/fixtures/sweep/`, labelled `synthetic`; the real registries for the due-list known answers):

- `due_families_by_exit` (deciding milestones are read from the gate register; for a kill gate whose ledger
  group is entered at M1 they are read from the frozen list of NAB-17 step 6 until that group is entered, and
  the two must then agree): on the plan-time registries and the gate register's deciding milestones, the family
  ids due are: M0 `EV-S29`; M1 `EV-F04`, `EV-F19`; M2 `EV-S15`, `EV-S17`, `EV-S24`, `EV-S27`, `EV-S32`,
  `EV-F23`, `EV-F26`; M3 nineteen (`EV-S14`, `EV-S21`, `EV-S22`, `EV-S23`, `EV-S36`, `EV-S41`, `EV-F01`,
  `EV-F06`, `EV-F07`, `EV-F10`, `EV-F11`, `EV-F14`, `EV-F15`, `EV-F17`, `EV-F18`, `EV-F19`, `EV-F21`, `EV-F24`,
  `EV-F29`); M4 twenty-one; M5 seven; M6 eight; M7 none; M8 seven. (The full lists are in NAB-23 to NAB-31.)
- `row_due_twice_is_swept_twice`: `EV-F19` (M1 and M3), `EV-S41` (M3 and M4) and `EV-F11` (M3 and M4) appear in
  both due lists; 72 family-exit pairs cover 69 distinct families.
- `standing_and_reread_gates_are_listed_not_counted`: `EV-S29` is in the M1 due list as `re-read: KILL-MCP`,
  `EV-F19` is in the M2 to M8 lists as `re-read: KILL-AUGUR-FALSECERT`, and neither changes the counts above;
  the M7 list holds the 100 `impact: tournament` items and still counts zero kill-gate families.
- `exit_record_without_sweep_fails_missing` (`sweep.missing`); `due_row_absent_fails_row_not_swept`
  (`sweep.row-not-swept`); `open_reading_task_fails` (`sweep.reading-task-open`).
- `unchanged_needs_looked_at`; `change_needs_evidence`; `evidence_must_resolve`.
- `change_needs_reason`: a confidence raise with resolving evidence of the right kind and an empty `reason`
  fails `sweep.change-without-evidence`; the same entry with its reason stated passes.
- `evidence_kind_must_fit_factor`: an Impact raised citing only the schedule gate's record fails
  `sweep.change-without-evidence`; the same record cited for an Effort change passes.
- `fired_kill_requires_fallback_text`; `kernel_reflection_moves_to_parked_on_kill`.
- `reopen_evidence_opens_proposal_not_move`.
- `below_two_after_rescore_is_proposed_for_parking`: a Frontier family re-scored from (4, 3, 4) = 3.0 to
  (4, 2, 5) = 1.6 with no Moonshot tier produces a parking proposal.
- `cap_after_entry`: four active plus one entered → `sweep.cap-exceeded`; MQ-08 entered while MQ-02 is
  `queued`, or MQ-03 entered ahead of MQ-02 with no ADR, → `sweep.cap-exceeded`; an item whose kill gate fired
  in `entered` → `sweep.kill-not-reflected`.
- `synthetic_record_moves_no_reality_factor`: a Confidence raise of EV-F32 whose only evidence is a gate record
  on `synthetic` data → `sweep.change-without-evidence`.
- `m8_final_states`: at M8 a queue item with no `final` entry, a `final` outcome `active`, or `never-funded`
  with an empty reason → `sweep.row-not-swept`; `passed` with no resolving gate record →
  `sweep.change-without-evidence`.
- `condition_met_follows_threshold`: KI-03 with threshold `specializable-share >= 1/3` and measured 0.25 fails
  `sweep.change-without-evidence` when `condition_met = true` or when a proposal is opened, and passes with
  `condition_met = false` and no proposal; a measured value under another name leaves the threshold unmet.
- `unchanged_on_unrun_gates_is_not_swept`: a family `unchanged` whose `looked_at` holds one `not-run` gate record
  fails `sweep.row-not-swept`; the same family in `carried_forward` passes.
- `unverified_settled_needs_record`: NV-01 listed as `measured` with an empty `record` fails
  `sweep.change-without-evidence`; with a gate record of GATE-SPARK-21 in the fixture register it passes.
- `share_row_resolves`: an Impact change citing `SH-M3-01` passes when that row is in the record's `shares` with
  `ceiling` 1.042 for `share` 0.04, and fails `sweep.change-without-evidence` when the row is absent or its
  ceiling is 1.5.
- `self_certified_sweep_is_refused`.
- `check_json_follows_runner_contract`: `check --exit M0 --root <fixture> --json` on a fixture with no sweep
  record prints one object with `check = "decision-sweep"`, `milestone = "M0"` and a block `sweep.missing`, and
  exits 0; on a complete fixture record it prints no block; on an unreadable `ev_matrix.toml` it exits non-zero.
- `skeleton_never_overwrites`: `skeleton --exit M0` on a fixture whose sweep record already exists leaves its
  bytes unchanged and says so.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) runs `decision-sweep due` for every exit, compares the family
counts with the known answers, writes a skeleton for a scratch exit, fills it from a fixture gate report and
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
