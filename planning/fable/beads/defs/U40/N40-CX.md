---
key: "N40-CX"
title: "registry: planted defects of the gate register and milestone layer, each a control the tooling must catch"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-01"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N38b-04", "N40-01", "N40-02", "N40-03", "N40-04", "N40-05", "N40-06", "N40-07", "N40-08", "N40-09", "N40-10", "N40-11", "N40-24", "N40-25", "N40-26", "N40-27", "N40-29", "N40-30", "N40-31", "N40-32", "N40-33", "N40-35", "N40-36", "N40-37", "N40-KA", "N41-CX"]
gates: ["GATE-GAUNT-01", "GATE-GAUNT-10"]
plan_refs: ["§40.1", "§40.7", "§43.2", "§38.11", "Appendix N"]
---
## Background

"A gate that no deficient model can fail validates nothing." Every gate in the program must have a registered
**negative control**: a planted defect that its test catches. The rule applies with full force to the tooling that
*keeps* the gates: a status register whose refusals cannot be triggered, an exit query that nothing can block, or
a kill ledger that accepts a softened trigger is the failure the whole verification program exists to prevent.

The program's printed counterexample catalogue is organized by subsystem and has **no section for the gate
register or the milestone chapter**. This registry supplies the missing entries: for each rule of the register and
of the exit protocol, the tempting shortcut, the smallest planted case that takes it, and the rule the case
protects. Each entry seeds a control in the conformance registry, attached to GATE-GAUNT-10 (status accounting)
and, where it concerns passing without a caught control, GATE-GAUNT-01.

## Scope

In: the 70 entries, appended to the counterexample catalogue through one block of numbers; the planted inputs (the
scratch-copy edit scripts and fixtures); the compiled-in table of catching tests (each entry carries its own
`due`); the binding check and its script. Out, with the bead that has it: the catalogue's record schema and the seeding tool that
writes `crates/frob-conformance/registry/controls.toml`, allocates `NC-` numbers and proves completeness by the join
(N38b-05; this bead writes no row of `controls.toml`); the number blocks (N38a-40); the control runner (N38b-04);
the catching tests (the beads named per row); known answers (N40-KA); physics counterexamples (the chapter
registries); the ten-fold harness's own controls (N39-CX; the program-gate beads state inline the ones they use).

## Technical approach

**1. Entries.** Each row is one entry of `crates/frob-conformance/registry/counterexamples.toml` in the record schema
of N38b-05 and no other field: `id` (the tag until the block is issued, then the CX number), `shortcut`, `case`
(the planted case), `numeric = true`, `plan_case = "numbers"` (each case is a concrete edit with its values),
`case_supplied_by` (empty), `rule`, `rule_ids` (GATE-GAUNT-10; also GATE-GAUNT-01 for RC-01 and RC-02; the rule
numbers and invariants the rule column names), `defined_in` (§40.1, §40.7 or §43.2), `same_as` (empty), `label`
(DESIGN), `control_kind = "planted-input"`, `plant`, `due` (the milestone of the earliest catching bead, listed
below) and `slug` (the `RC-` tag). The plant is the edit script
`crates/frob-conformance/fixtures/register_controls/<plant>.edit` (or a fixture directory of that name), written by
this bead and applied only to a scratch copy, never to the real register; the entry's `plant` is one plant record of
N38b-05's form with `id = "N40-CX:<plant>"` (this bead is the planting bead, and the name after the colon is the
fixture). The plant record's `target` is the rule column (the rule's check), its `defect` the planted case and its
`expect` the catching test's refusal with the reason it names, and its `removed_check` names that check as the
guard whose removal would let the case through (BD-025's rule 5: a control is a defect, not a boundary); the
seeding tool copies them to the control. DESIGN, correcting an earlier draft of this
bead that kept the rows in a private file `registry/counterexamples/register.toml` with fields `mutant`,
`caught_by`, `gate`, `bead` and `due` and wrote `[[control]]` records itself: the registries named in the
conventions are the only registry files, and only the seeding tool writes `controls.toml`. The catching test and
its bead are kept in one compiled-in table keyed by CX number and slug in
`crates/frob-conformance/tests/register_controls.rs`; `due` is the entry's own field of N38b-05's schema, which the
seeding tool copies to the control's optional `due` (BD-025), so the table holds no milestone.

**2. Numbers.** A bead never invents a number above the plan-time maximum (CX-384): this bead files one `[[block]]`
request in `crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`, N38a-40) with
`registry = "cx"`, `count = 70`, `owner = "N40-CX"`, `section = "§40.1, §40.7, §43.2"`; the allocator fills `first`
and `last`; the tags RC-01 to RC-70 are mapped to the block's numbers in table order in one commit and kept in
`slug`, and prose elsewhere may go on citing the `RC-` alias. A tag still used as an identifier in a registry, a
test name or a derivation file at M0 fails the ledger check `provisional-tag-past-due`; test names use the
`cx_<nnn>_` prefix (STD-TEST).

**Due milestones (from the catching beads).** M0: RC-01 to RC-24, RC-33 to RC-35, RC-42, RC-49, RC-50, RC-52 to
RC-54, RC-57, RC-59 to RC-64, RC-66 to RC-70 (45 rows).
M1: RC-28 to RC-32 (N40-09), RC-36, RC-37 (N40-11), RC-38, RC-44, RC-45,
RC-58 (N40-30), RC-39 (N40-24), RC-40 (N40-25), RC-41 (N40-26), RC-43 (N40-29), RC-46 to RC-48 (N40-31), RC-55
(N40-36), RC-56 (N40-37) (20 rows). M2: RC-25 to RC-27 (N40-08), RC-51 and RC-65 (N40-33) (5 rows). DESIGN reason:
GATE-GAUNT-10 is recorded at every exit from M0, and a control whose catching bead lands at M2 cannot be planted at
M0; registered without `due`, it would be `stale` and leave GATE-GAUNT-10 `not-eligible` until M2.

**3. Rows.**

| Tag | The tempting shortcut | The planted case | The rule it protects | Plant | Caught by (bead) |
|---|---|---|---|---|---|
| RC-01 | Record a pass now and register the control later | A `pass` part whose gate has `controls = []` | A gate passes only with a registered control (rule 5; INV-GAUNT-02) | `pass-with-no-control` | N40-01 |
| RC-02 | The passing test ran, so the gate passed | A `pass` part whose control has `caught = false` in the same run | A gate run in which a control was not caught is a fail | `pass-with-uncaught-control` | N40-01 |
| RC-03 | The implementer knows best whether it passed | `recorded.session` equal to `recorded.implemented_by` | No self-certification (INV-GAUNT-05) | `self-certified-session` | N40-01, N40-05 |
| RC-04 | Cite the receipt; nobody will follow the link | An evidence item whose hash resolves to nothing | A record resolves in the artifact graph (rule 1) | `dangling-evidence-hash` | N40-01 |
| RC-05 | The target is what we measured, near enough | `measured` byte-equal to `threshold` | A pass carries the measured value, not the target | `measured-equals-threshold` | N40-01 |
| RC-06 | Fix the status table by hand; regeneration is slow | The README table edited to show `pass` | Status tables are generated and never edited (INV-GAUNT-06) | `hand-edited-status-table` | N40-01, N40-36 |
| RC-07 | A gate can be both a known answer and a performance claim | A row with `claim = ["known-answer", "performance"]` | Exactly one of the seven claims (rule 3) | `two-claims-on-one-row` | N40-02 |
| RC-08 | The mechanism slipped, so the gate's milestone slips with it | A part's milestone changed from M2 to M3 with no ADR | A gate may not move later without an ADR | `milestone-moved-without-adr` | N40-02, N40-04 |
| RC-09 | A gate nobody can pass can be left out of the table | One convergence row deleted | The register is the single list; counts are pinned | `convergence-row-deleted` | N40-02 |
| RC-10 | The trigger was a guess; adjust it now that we have data | A kill trigger lowered by an amendment dated after the governing receipt | A trigger changes only by a dated amendment made before the measurement (rule 2) | `trigger-lowered-after-receipt` | N40-03 |
| RC-11 | The fallback wording can be tidied at decision time | A fallback text differing from the amendment replay | Never quietly re-scoped (rule 2) | `fallback-text-differs` | N40-03 |
| RC-12 | Keep the capability's name; add a footnote about the fallback | A fired gate whose capability still prints under its original name | A capability that fell back is described by the fallback's name | `fired-capability-old-name` | N40-03, N40-37 |
| RC-13 | The decision is obvious; the ADR need not cite numbers | A kill decision with no receipt cited | Every decision is an ADR with measured values beside the trigger | `kill-decision-without-receipt` | N40-03, N40-30 |
| RC-14 | A gate that is not ready is simply not due | A due part removed from an exit by editing its milestone | The exit's due set is computed from the register (rule 6) | `due-part-milestone-edited` | N40-04 |
| RC-15 | The demonstration ran on my machine | A demonstration receipt from a tree with uncommitted changes | Every demonstration runs from a clean checkout | `demo-from-dirty-tree` | N40-05 |
| RC-16 | Close the remaining beads with the same note | Two bead logs with byte-equal close reasons | A shared close reason is a failure signal | `shared-close-reason` | N40-05 |
| RC-17 | The risk fired, but it is under control | A risk whose earliest signal fired, with no recorded outcome | A fired signal ends in a kill decision, an ADR or a written reason | `fired-risk-without-outcome` | N40-05 |
| RC-18 | This report is internal; it needs no reality line | A report with no `reality:` line | Every report prints its `reality:` line (GATE-GROUND-01) | `report-without-reality-line` | N40-05 |
| RC-19 | One more Moonshot will not hurt | Five items active at one commit | At most four Moonshot items active at once | `five-moonshots-active` | N40-05 |
| RC-20 | We are within the estimate, so the gate is fine | A gate threshold that cites a week count | The estimates are not acceptance evidence and no gate cites them | `threshold-cites-weeks` | N40-06 |
| RC-21 | Enter the exit date by hand | An `exited` time the bead logs do not support | Start and exit times come from the logs | `exit-time-unsupported` | N40-06, N40-11 |
| RC-22 | The sibling was admitted at M0; that covers everything | A group first used at M3 with only an M0 record at an old pin | An admission is never inherited (rule 7) | `admission-inherited` | N40-07 |
| RC-23 | It is only a small dependency | An M1 crate linking a GPU driver package | The dependency firewall per milestone | `m1-crate-links-gpu-driver` | N40-07 |
| RC-24 | The pin move is a patch release | A sibling commit changed inside the tournament window | A pin move voids the runs it spans | `sibling-commit-in-window` | N40-07 |
| RC-25 | Re-run my gate's copy of the experiment on fresh seeds | One member of a shared-fixture group citing its own receipt | One fixture, run once per milestone | `shared-fixture-own-receipt` | N40-08 |
| RC-26 | The ledger allows 10⁻³, so the port balance can too | GATE-SINEW-02's bound changed to 10⁻³ | A looser bound on a wider scope does not relax a tighter one | `port-balance-bound-loosened` | N40-08 |
| RC-27 | A lower τ pair would let more rungs select | The pair (0.6, 0.3) entered for a rule that depends on τ | One pair of thresholds, 0.7 and 0.5, everywhere | `tau-pair-changed` | N40-08 |
| RC-28 | The chapter's claim label is good enough | An owner declaration with claim "method" | The register governs the claim | `owner-claim-method` | N40-09 |
| RC-29 | The chapter's milestone is the real one | An owner declaration stating GATE-HAB-11 at M3 | The register follows the milestone skeleton | `owner-milestone-differs` | N40-09 |
| RC-30 | The chapter's trigger is more nuanced | An owner declaration with a changed trigger number | The register governs triggers | `owner-trigger-differs` | N40-09 |
| RC-31 | Everyone knows who owns that gate | A register row with no owner declaration | Each gate has exactly one owner (rule 9) | `row-without-owner` | N40-09 |
| RC-32 | Both chapters' clauses should count | A consumer declaration turning the effective-sample-size diagnostic of KILL-DR into a trigger | The owner chapter governs; the consumer restates only | `consumer-diagnostic-as-trigger` | N40-09 |
| RC-33 | Gates added later are tracked in their chapters | A late gate absent from its exit's report | Late gates are binding and listed | `late-gate-missing-from-exit` | N40-10 |
| RC-34 | The index's statement is the criterion | A late gate's brief entered from the abridged index | The defining row governs; no cut text is entered | `brief-from-abridged-index` | N40-10 |
| RC-35 | Two similar names are the same gate | KILL-EXPODOMAIN entered with Sensorium as owner | KILL-EXPODOMAIN is Arena's; KILL-EXPOSURE is Sensorium's | `expodomain-wrong-owner` | N40-10 |
| RC-36 | Reclassify the delay afterwards | Blocked events retagged from `sibling-primitive-missing` to `own-work` | Logs are append-only; attribution uses the tag at the time | `blocked-events-retagged` | N40-11 |
| RC-37 | The estimate was too tight; raise it | `estimate_weeks` raised with no dated amendment | Estimates change only by a dated amendment | `estimate-raised-silently` | N40-11 |
| RC-38 | The ledger will catch up | A kill gate declared by a crate and absent from the ledger | The ledger lists every kill gate, once | `kill-gate-missing-from-ledger` | N40-30 |
| RC-39 | Film is film | Habitat's advected-film trigger entered under KILL-FILM | Two mechanisms, two gates: KILL-FILM and KILL-ADVECT | `advect-under-kill-film` | N40-24 |
| RC-40 | Either clause is bad enough | KILL-FAMILY evaluated as a disjunction of its two measured clauses | The trigger is a conjunction (or the pilot's falsifier) | `kill-family-disjunction` | N40-25 |
| RC-41 | One false certificate is an outlier | A false certificate recorded and its proof kind left in service | KILL-AUGUR-FALSECERT: suspend, downgrade, decide the cause | `false-cert-kind-in-service` | N40-26 |
| RC-42 | MCP will not make it; take the fallback now | KILL-MCP recorded as fired before either upstream item was attempted | The trigger requires both attempts | `kill-mcp-fired-early` | N40-27 |
| RC-43 | A kill gate with no milestone has no deadline | A late kill row with an empty decision point | Decided at the milestone of the gate beside it | `late-kill-without-decision-point` | N40-29 |
| RC-44 | "At most" and "below" are the same | A declaration softening "below 0.5" to "at most 0.5" | Numbers and comparison direction must match | `below-softened-to-at-most` | N40-30 |
| RC-45 | The kill is recorded; the tracker can stay as it is | A fired gate whose dependent is left open on the killed capability | Dependents are re-read: superseded, or standing on the fallback | `dependent-left-on-killed` | N40-30, N40-37 |
| RC-46 | The bet looks fine so far | A bet marked `held` while its deciding gate is `not-run` | A bet is decided by a recorded gate | `bet-held-on-not-run` | N40-31 |
| RC-47 | This other result also supports the bet | A gate cited as deciding a bet that the table does not list for it | The register's table governs deciders | `bet-decider-not-listed` | N40-31 |
| RC-48 | Come back to that bet next milestone | A bet with a failed decider and no decision at its exit | An open bet blocks the exit | `failed-bet-undecided` | N40-31 |
| RC-49 | Every M2 gate obviously needs a GPU | A device-only gate added to the M2 due set with no CPU part and no variant listing | No milestone is blocked by one failed bet; the no-GPU variant must still exit | `device-only-gate-in-m2` | N40-32, N40-33 |
| RC-50 | The fallback is in force; the paperwork can follow | A fallback exit taken with no kill decision recorded | The exit is ready because the decision is recorded, not because failures are ignored | `fallback-exit-without-decision` | N40-32 |
| RC-51 | Under the variant, skip the whole gate | A gate with a CPU tier listed wholesale as `not run` | CPU rows run and must pass under the no-GPU variant | `cpu-tier-skipped-wholesale` | N40-33 |
| RC-52 | Nobody answered, so assume yes | A question past its deadline with no ADR and work proceeding on the generous answer | Nothing is decided by silence | `decided-by-silence` | N40-35 |
| RC-53 | A sensible default is to assume the catalogue price | A default that adds a claim | A default removes a claim or a capability; it never adds one | `default-adds-claim` | N40-35 |
| RC-54 | Reuse the old identifier for the new question | A tombstoned identifier given a new question | A closed question is tombstoned, not reused | `tombstone-reused` | N40-35 |
| RC-55 | The pass is recorded; the receipt can be collected | A recorded pass whose receipt was deleted | A claim whose closure dangles prints `evidence: unavailable` and cannot be cited | `receipt-deleted-after-pass` | N40-36 |
| RC-56 | One more repair cycle and it will pass | A second repair cycle with no `deferred` ADR | One repair cycle; a second is a deferred decision | `second-repair-cycle` | N40-37 |
| RC-57 | The M7 reading replaces the M3 one; keep the file tidy | A kill row's M3 decision entry edited in place to the M7 outcome instead of a new entry appended | Earlier decision entries are kept and never edited; the latest per scope is in force | `decision-entry-edited` | N40-03 |
| RC-58 | The ledger and the chapter will say the same thing | An owner declaration recording `fired` where the ledger row's decision entry for the same exit and scope says `not-fired` | The two copies of a decision agree, or the cross-check fails | `ledger-chapter-disagree` | N40-30 |
| RC-59 | The calendar can keep its own copy of the default | An `[[owner_input]]` row carrying a `default` that differs from its question's `[[open]]` row | The calendar is a view of the question register and copies nothing | `calendar-copies-default` | N40-35 |
| RC-60 | A sibling's pilot is a gate like any other; let it block | INT-FTUI-01 entered with `blocks_exit = true` | Sibling admission gates are for foundations off every critical path and never block a milestone (N40-KA R-08) | `sibling-pilot-blocks-exit` | N40-02 |
| RC-61 | KILL-MCP was decided at M0; the M1 reading is the same decision | KILL-MCP entered with `reread = []` | KILL-MCP is decided at M0 and re-read at M1 with the M1 verbs, a decision at each exit (N40-KA R-12: 113 for 112 gates) | `kill-mcp-no-reread` | N40-04, N40-27 |
| RC-62 | A check that crashed found nothing, so its step passed | An attached check record in `scripts/exit/checks.toml` whose command exits non-zero with no output, its step reported as passed | A check that is absent, exits non-zero or prints no valid object is `not-run`, and the verdict is `not exited` (the runner's table of attached checks, §40.7) | `nonzero-check-as-pass` | N40-05 (`attached_check_contract`) |
| RC-63 | The owner will be asked when the question comes due | An `Owner` question with status `open` and deadline M2 (OPEN-SPARK-5) with no `[[owner_input]]` row, or with one only at M3 | Every owner's question is put to the owner at or before its deadline: a calendar row at or before the deadline, else a load error (clause (e) of the owner calendar) | `owner-question-never-asked` | N40-35 (`owner_rows_have_calendar_row`) |
| RC-64 | The gate is `not-run`; why is someone else's business | A scratch M3 exit with GATE-TENFOLD-06 `not-run` and OPEN-TENFOLD-4 `defaulted`, the report naming no question | A `not-run` gate due at the exit that a defaulted question lists in `blocks` is reported with that question and `open-default-blocks-exit` (clause (f)) | `default-behind-not-run-unnamed` | N40-35 (`default_behind_not_run_gate_named`) |
| RC-65 | Nothing fired, so the kill gate did not fire | At the M2 exit under the no-GPU variant, a kill gate recorded `not-fired` while a part its record reads is `not run (variant: no-GPU)` | Under the variant a kill gate is `fired` on its CPU receipt or `deferred` naming M3 and the clause evaluated; it is never `not-fired` on missing measurements | `kill-not-fired-under-variant` | N40-33 (`kill_decisions_under_variant`) |
| RC-66 | A kill gate is a gate; give its row a claim like the others | A `kind = "kill"` row carrying `claim = "known-answer"` | A kill row decides and measures nothing: it carries none of the measurement fields (`claim`, `claim_id`, `threshold`, `tier`, `controls`, a `[[gate.part]]`), and one that does fails the load naming the field | `kill-row-with-claim` | N40-01 (`kill_row_has_no_measurement_fields`) |
| RC-67 | A failed run needs no paperwork | A `fail` part with `measured = "0.41"`, a commit and a command, and `receipt = ""` | A failing value is a record: a `fail` part carries its measured value, commit, command and a receipt that resolves under `receipts/gates/`, so that the failure can be reproduced and its stack read (rule 1) | `fail-part-without-receipt` | N40-01 (`fail_is_a_record`) |
| RC-68 | Any label that names the scope will do | A decision entry with `scope = "family:cuda"` on a kill row whose `scope_kind` is `backend` (KILL-D1) | An entry's scope is `<kind>:<value>` of its row's `scope_kind`, a kind of N40-03's closed table; a scope of another kind, or on a row with no `scope_kind`, fails the load (`decision-scope-invalid`) | `decision-scope-wrong-kind` | N40-03 (`scope_kinds_closed`) |
| RC-69 | The second reading at this exit simply replaces the first | Two decision entries for one scope at one exit (KILL-SHORTGRAD at M6), the second with no `reading` | A second entry for one scope at one exit carries a distinct `reading` (`final` or `reopened`) and its own ADR; two entries with the same `reading`, absent counting as one value, fail the load (`decision-entry-duplicate`) | `second-entry-without-reading` | N40-03 (`second_reading_named_final`) |
| RC-70 | A kill gate proposed late is simply added to the register | A kill row that no ledger bead seeds, entered with no `introduce` amendment | A kill gate outside the ledgers enters by an `introduce` amendment dated before every receipt of its reads; a row with neither a ledger seed nor that amendment is refused (`changed-without-amendment`) | `kill-row-without-introduce` | N40-03 (`introduce_seeds_row`) |

**4. Binding check.** A harness test (`register_controls_bite`) asserts for each row that (a) on the clean fixture
the named test passes, and (b) with the row's plant applied in a scratch copy the same test fails with the
expected reason. Run for an exit, the check plants every row due at or before it; a row due later is listed
`pending: <bead>` and is not run; a row that is due but whose catching bead has not landed is reported `stale` with
the bead (the seeded control is `stale`, and GATE-GAUNT-10 cannot record a pass, which is the gate's rule, not this
bead's acceptance); a row whose catching bead is closed while its catching test is absent or does not fail under
the plant fails the check (the suite's rule: a registry row due at or before the current milestone has a bound
test). Under the conformance registry's own rule, a control that has never failed its gate in any recorded run is
itself suspect and is reviewed at each milestone exit.

## Acceptance criteria

- [ ] `counterexamples.toml` holds the 70 entries RC-01 to RC-70 in the schema of N38b-05 and no other field, each
      with a plant, its gates, its `due` milestone and, in the test file's table, a catching test; the due counts
      are 45 at M0, 20 at M1 and 5 at M2; no private file `registry/counterexamples/register.toml` exists.
- [ ] The block request for 70 numbers is committed; after issue the entries carry the block's numbers in table
      order with their `RC-` tags in `slug`; `registry::alloc::validate` passes and no tag remains as an identifier
      at M0.
- [ ] After the seeding tool of N38b-05 is re-run, each entry has exactly one control of kind `planted-input`
      attached to GATE-GAUNT-10 (and RC-01, RC-02 also to GATE-GAUNT-01), so that GATE-GAUNT-10 cannot record a pass
      at an exit without every row due by that exit caught (45 at M0, 65 at M1, 70 from M2); the second run changes
      no byte; the join passes; this bead writes no record of `controls.toml`.
- [ ] The binding check passes for every row whose catching bead is closed, lists the later rows `pending` and the
      rows whose bead has not landed `stale` with their beads, and the clean fixture passes.
- [ ] No plant is ever applied to the real `registries/gates.toml`: plants run only on scratch copies.
- [ ] Row identifiers are append-only.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test register_controls` (real registry files and scratch copies, no mocks):
`register_controls_bite`, one generated case per row named `cx_<nnn>_<plant>` (70; a row due after the selected
milestone reports `pending`, a row whose catching bead has not landed `stale`), plus `clean_fixture_passes_all`,
`mutants_touch_scratch_only` (the real register's digest is unchanged after the run), `ids_append_only`,
`schema_fields_only` (a scratch entry carrying `caught_by` is rejected by the loader), `block_request_of_70`, and
`due_counts` (45, 20, 5; each row's `due` equals the earliest milestone among its catching beads, read from the bead
index; a planted row due at M0 whose closed catching bead has no such test fails the check, and the same row with
`due = "M1"` is `pending` at M0). The nine rows added after RC-61 are checked against their catching tests by name:
RC-62 with `attached_check_contract` (N40-05), RC-63 with `owner_rows_have_calendar_row` and RC-64 with
`default_behind_not_run_gate_named` (N40-35), RC-65 with `kill_decisions_under_variant` (N40-33), RC-66 with
`kill_row_has_no_measurement_fields` and RC-67 with `fail_is_a_record` (N40-01), RC-68 with `scope_kinds_closed`,
RC-69 with `second_reading_named_final` and RC-70 with `introduce_seeds_row` (N40-03).

### End-to-end script and logging

`scripts/e2e/register_controls.sh` (header `bead=N40-CX milestone=M0 labels=delivery,registry
plants=inert-plant,drop-plant,private-field,stale-tag`): re-runs the seeding tool (`cargo test -p frob-conformance
--test seed_controls -- --ignored seed`) and the join (`cargo test -p frob-conformance --test seed_join`) on a
scratch copy of `registry/` and asserts by digest that a second seeding run is a no-op; applies each plant in a
fresh scratch copy, runs its catching test in the form of ADR-GAUNT-01 (`FROB_TEST_SEED=0 cargo test -p
frob-conformance --test <tier from the gate record> -- <row>`), and records caught or not caught; with `--milestone
<Mn>` (default: the current milestone) asserts every row due by that exit whose bead has landed caught (45 of 45 at
the M0 exit; 70 of 70 from M2), the rows not yet due listed `pending: <bead>` and the stale rows counted apart, and
the clean fixture passing; then applies the inert plant and asserts it is reported not caught, and plants on
scratch copies an entry with its plant removed, an entry carrying a private field and a tag left as an identifier
at M0, asserting each is rejected. It calls no `frob` verb. Logs through the STD-E2E helper to
`target/e2e-logs/register_controls/<utc>.ndjson`, per row: `ts`, `script`, `bead`, `step`, `event`, the command,
exit code, wall time, the evidence block `{claim: known-answer, class: Tested, tier: G0:host, determinism: D0, seed:
0}`, the CX number and `RC-` slug, the expected and observed refusal reason, the digest of the real register before
and after (equal), the scratch path, and on failure the first row not caught; `metric` lines give caught against
due, pending and stale by milestone; the summary names GATE-GAUNT-01 and GATE-GAUNT-10.

### Known answers and negative controls

This bead is the registry of negative controls for its unit. Its own control: a plant made inert (an edit script
that changes nothing) must be reported as *not caught* and fail the binding check, which shows the check
distinguishes a caught plant from a plant that was never applied.

## Considerations

- Several rows describe human shortcuts more than code defects. They are still testable, because each leaves a
  trace in a file that a rule constrains.
- A defect found in use (a softened trigger that slipped through, a pass that was void) becomes a permanent row
  with the case that exposed it, appended through a new block.
- The `RC-` identifiers stay as aliases in `slug`; the beads that name a row (N40-05, N40-33, N40-35 and the other
  catching beads) may cite either form.
- Rows RC-62 to RC-65 were added in polishing round 3 for rules N40-05, N40-35 and N40-33 had tested with no
  registered control, and RC-66 to RC-70 in round 4 for five plants that the unit tests of N40-01 and N40-03 caught
  with no row here; a rule added later to a catching bead takes the next tag here, numbered through a new block
  once this one is issued.

## References

Plan §40.1, §40.7, §43.2, §38.11 (the five rules of the negative-control registry), §0.7. Appendix N (no section
for these chapters). Gates GATE-GAUNT-01, GATE-GAUNT-10. Invariants INV-GAUNT-02, -05, -06, -07.
