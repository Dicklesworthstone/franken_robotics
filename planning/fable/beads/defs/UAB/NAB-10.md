---
key: "NAB-10"
title: "frob-conformance: citation lint: no gate, contract or close reason cites a source that still needs a reading"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "gauntlet", "frob-conformance", "solid", "test", "gate", "review-notes-open"]
parent: "WP-01"
deps: ["BD-002", "BD-025", "N00-01", "N38a-10", "N40-01", "NAB-09", "STD-TEST"]
needs: []
related: ["NAB-11", "NAB-12", "NAB-13", "NAB-14", "NAB-15"]
gates: []
plan_refs: ["Appendix H (preamble)", "Appendix H.1", "Appendix H.6", "§0.1", "§0.7", "§38"]
---
## Background

The source register (`registries/sources.toml`, NAB-09) records, for every outside fact the specification of
FrankenRobotics relies on, where it was read, at what version, and how: from the primary text (mark P), through
a summary (S), fetched with no record of which (F, treated as S), from a search-engine summary (R), recalled
(M), or with no mark (U). The rule that gives the register its teeth is short: **only a P reading may be cited
by a gate without a further reading; every other mark is a reading task that the gate's bead performs first.**
The same holds for a local source that has not been re-read at the pinned commit of its repository, and for a
plan-time check that has not been re-run.

Without a mechanical check the rule decays into a sentence. This bead is the check. It also enforces the list
of things the planning record explicitly did **not** verify and did not rely on (carried by NAB-14): a gate,
a contract or a report that quietly starts relying on one of them fails.

Why this matters to the program: the comparative claims (parity with incumbents, the ten-fold test) rest on
published incumbent figures. Several were read through a summarizing tool, one training-time figure was read
from a rendered plot by eye (about ± 20 s), and absolute incumbent throughput on the hardware this project owns
was never read at all. A threshold derived from a number nobody re-read is a threshold derived from nothing.

Design decision (DESIGN): the lint is a test in `frob-conformance`, not a seventh `xtask` check (the workspace
allows exactly six). Rejected: checking citations only at milestone exit, because the reading task belongs to
the bead that first cites the source and is cheapest there.

Tier: Solid.

## Scope

In: the citation grammar, the scan, the grading rule, the reading-task report, the unverified-reliance check,
the test target. This bead is the one place that resolves a citation against the register and grades it: no
other lint reports whether a cited source exists, whether its reading may be cited by a gate, or whether a
plan-time check has been reproduced.

Out, and where it is:
- the register and its loader (NAB-09); the records (NAB-11 to NAB-15);
- finding the statements labelled FACT that name an outside product, paper or benchmark, and checking that each
  carries a citation token at all: lint L-SOURCE of the evidence-label lint (N00-01), which exports the list of
  sites this lint grades and does no resolution or grading itself;
- the re-run status of a plan-time check: the plan-time pilot ledger (N38a-10), read through
  `pilots::rerun_status` and `pilots::status_of_source`. The ledger holds no citation scan; the refusal of a
  gate that cites an unreproduced check is this bead's finding `cite.plan-check-not-reproduced`;
- performing the re-reads and re-runs (N00-34, N00-16, N00-35 and the pilot re-run beads); the wording of
  claims (N00-15).

## Technical approach

**1. Citation forms recognised.**

1. A field `sources = ["SRC-PUB-027#mixed-batch-ratio", …]` in a record of `registries/gates.toml`, of a
   `contract.toml`, and of any registry under `registries/` or `crates/frob-conformance/registry/` whose schema
   declares the field (the killed-idea register's `sources`, NAB-32). The part after `#` names a figure or a
   `covers` phrase of a reading; without it the citation is of the source's weakest reading. Neither of the
   first two schemas has the field today, and both refuse an unknown one, so this bead adds it in the same
   commit as the scan: the optional list of strings `sources` on a `[[gate]]` record of `frob.gates/1` (the
   gate register's schema and N40-01's loader, which fails a record with an undeclared field) and the optional
   key `sources` in the `[crate]` table of the hand-written `schemas/contract.schema.json` (the rule of
   `check-contracts`, N34-08: the bead that introduces a `contract.toml` key adds it to the schema).
2. In prose (`CONTRACT.md`, `docs/**`, `README.md`): the form `[SRC-…]` or `[SRC-…#figure]` after a statement.
3. In a bead's close reason: the close-reason text of each closed issue in the tracker's exported issue file
   `.beads/issues.jsonl` (the file the close audit of N38b-03 reads), scanned for the forms of items 2, 4 and 5.
   The per-bead logs `docs/delivery/logs/<bead>.ndjson` (`frob.beadlog/1`, N29a-09) have no citation field and
   are not a citation form.
4. A bare URL, or a path into a sibling repository, anywhere in the scanned files outside code blocks.
5. A plan-time check cited without a source id: the phrase "FACT, plan-time" or "FACT, Appendix K" after a
   statement, or a ledger identifier (`PILOT-PT-nn`, `PILOT-PT-Sn`) in a record of `registries/gates.toml`, a
   contract or a bead close record. Such a citation is resolved through the plan-time pilot ledger (N38a-10) to
   the ledger record and the `SRC-CHK` ids it covers; a phrase that names no identifiable check, or an
   identifier that is not in the ledger, is `cite.unknown-source`.

**2. The scan and its findings.** `crates/frob-conformance/tests/citations.rs`, run as
`cargo test -p frob-conformance --test citations`:

| Finding | Condition | Repair printed |
|---|---|---|
| `cite.unknown-source` | A cited id is not in the register | The nearest ids by topic |
| `cite.unknown-figure` | The `#figure` part matches no figure or `covers` phrase of the source | The source's figures |
| `cite.unregistered-url` | A bare URL or sibling path that no source's `locator` equals | "register the source, with its mark" |
| `cite.needs-reading` | A gate record with status other than `not-run`, or a contract's Oracles section, cites a reading with `reread_before_gate = true` | The reading task: source, what to read, at which version |
| `cite.pass-on-unread` | A gate record with status `pass` cites a reading with `reread_before_gate = true` | As above; the record must be withdrawn or the reading done |
| `cite.plan-check-not-reproduced` | A citation (by `SRC-CHK` id, by ledger identifier or by the plan-time phrase) of a plan-time check whose re-run status in the plan-time pilot ledger, `pilots::rerun_status`, is `not-run` or `differs`; a `plan-check` source that no ledger record covers counts as `not-run` | The re-run bead named by the ledger record (`rerun_owner`) |
| `cite.local-not-at-pin` | A citation of a local source not re-read at the stack-lock commit | The fact-sheet script to run |
| `cite.relies-on-unverified` | A gate threshold, a contract statement or a report cites an `[[unverified]]` item as a fact | The gate or open question that measures the item |
| `cite.fact-without-citation` | A statement labelled FACT with a number and no citation. The sites come from `fact_sites` of the evidence-label lint (N00-01), the single detector; that lint's own run reports the same site as `L-SOURCE`, and this lint lists it in the reading-task report without detecting it a second time | "cite a source or relabel" |
| `cite.by-eye-without-uncertainty` | A figure with `how = "figure-by-eye"` cited without its uncertainty | The recorded uncertainty |

`cite.needs-reading` is a warning while the citing gate's status is `not-run` (the reading task is listed, the
build passes) and a failure once the gate records anything else. All other findings are failures.

**3. The grading rule in one place.** A citation is *clean* iff it resolves and
`reread_before_gate(reading) = false`. A gate may be recorded only with clean citations. A contract's Oracles
section may name only clean citations. A hypothesis or a bet may cite any registered reading: hypotheses are
allowed to rest on unread sources, gates are not.

**4. The reading-task report.** `frob.reading-tasks/1`, printed by the test with `--nocapture` and written to
`target/citations/reading-tasks.json`: for every reading with `reread_before_gate = true`, the gates, contracts
and beads that cite it, the milestone of the earliest citing gate, and the instruction (re-read the primary
text at the recorded version; or open the page a search summary pointed to; or locate a URL for an unlocated
source). The milestone-exit sweep (NAB-23 to NAB-31) reads this report: no gate due at an exit may still have
an open reading task.

**5. The unverified list.** NAB-14 carries `[[unverified]]` records (absolute incumbent throughput on an
RTX 4090; the RTX 4090's double-precision ratio, memory bandwidth and peer access; correct rounding of Metal's
single-precision division and square root; and the others). Each names the gate or open question that will
measure it. The lint treats a citation of an unverified item as clean only when that gate has a recorded value.

**6. Budget, root and determinism.** Under 5 s on the full tree (TARGET). The tree walked is the workspace root,
or the directory named by `FROB_LINT_ROOT` (the convention of NAB-02's glossary lint; NAB-02 is not a dependency,
so this bead walks the tree itself and shares a walker with the other lints only where one already exists), so
that NAB-16 can run the lint on a
scratch copy holding its planted defects. D0: a pure function of the scanned bytes, the register, the gate
register and the plan-time pilot ledger (the plan-time records of `pilots.toml`).

## Acceptance criteria

- [ ] `cargo test -p frob-conformance --test citations` passes on the clean tree and prints the reading-task
      report.
- [ ] Each of the ten findings is produced by its planted fixture, with the path, line and source id.
- [ ] A gate record with status `pass` that cites an S reading fails `cite.pass-on-unread`; after a `reread`
      with `result = "confirmed"` is appended to the source, the same record is clean.
- [ ] A gate record with status `not-run` that cites an S reading passes, with the reading task listed.
- [ ] A contract that states an absolute incumbent throughput on an RTX 4090 as FACT fails
      `cite.relies-on-unverified` until the parity-baseline gate (GATE-SPARK-21) has a recorded value.
- [ ] The 410 s training-time figure, read by eye, cannot be cited without "± 20 s".
- [ ] The lint is not an `xtask` subcommand.

## Test plan

### Unit tests

`crates/frob-conformance/tests/citations.rs` with fixture trees under `tests/fixtures/citations/` (real files,
labelled `synthetic`):

- `clean_gate_passes`: a gate citing a P reading at its recorded version.
- `sources_field_is_declared`: a `[[gate]]` record with `sources` loads through N40-01's loader and validates
  against the gate register's schema, and a `contract.toml` with `[crate] sources` validates against
  `schemas/contract.schema.json`; the same records with the misspelled `source` fail both.
- `close_reason_citation_is_scanned`: a fixture `.beads/issues.jsonl` whose closed issue's reason ends
  `[SRC-PUB-999]` fails `cite.unknown-source` with the issue id.
- `pass_on_s_reading_fails`; `not_run_on_s_reading_warns_and_lists_task`.
- `confirmed_reread_makes_clean`; `corrected_reread_keeps_old_figure_uncitable`.
- `unknown_source_fails`; `unknown_figure_fails_and_lists_figures`.
- `bare_url_in_contract_fails` (a URL equal to no `locator`); `url_in_code_block_is_skipped`.
- `plan_check_needs_reproduced` (re-run status `not-run`, which is a ledger record still `registered`, and
  `differs` both fail a citing gate; a record that is `reproduced` passes). This is the one test of the rule "a
  gate row cites a plan-time pilot only when its record is `reproduced`", run against the real ledger function
  of N38a-10.
- `plan_time_phrase_resolves_to_ledger_record`: a contract sentence ending "FACT, plan-time" beside
  `PILOT-PT-06` resolves to that record; `PILOT-PT-99` is `cite.unknown-source`; a gate citing `SRC-CHK-24`,
  which no ledger record covers, fails `cite.plan-check-not-reproduced`; a gate citing `SRC-CHK-13`, covered by
  the supplementary record `PILOT-PT-S3` while that record has no result, fails the same way.
- `unsourced_plan_time_pilot_resolves_by_ledger_id`: a gate record citing `PILOT-PT-S4` (the held-command device
  pilot, which has no `SRC-CHK` id: NAB-15 step 2a; its ledger record covers none) resolves to that record and
  fails `cite.plan-check-not-reproduced` naming N16c-45 as the re-run owner while the record is `not-run`; on a
  scratch ledger in which the record carries a reproduced result the same citation is clean.
- `local_source_needs_pin_reread`.
- `unverified_reliance_fails_until_gate_recorded`.
- `by_eye_needs_uncertainty`.
- `hypothesis_may_cite_unread`: a hypothesis record citing an R reading is not a finding.
- `oracles_section_citing_unread_fails`: a contract whose Oracles section cites `[SRC-PUB-027#other]` (the S
  reading of the mixed-readings fixture) → `cite.needs-reading` as a failure, not a warning, because the
  contract is not a `not-run` gate.
- `fact_without_citation_from_fact_sites`: a contract sentence "FACT: the incumbent steps 4,096 worlds at
  1.2 M steps/s." with no citation, reported by N00-01's `fact_sites` → `cite.fact-without-citation` listed
  once in the reading-task report, and the same sentence ending `[SRC-PUB-027#table-a5]` → clean.
- `reading_task_report_groups_by_earliest_gate`: three gates at M0, M2 and M3 citing one S reading → the task is
  listed once, under M0.

### End-to-end script and logging

`scripts/e2e/sources.sh` (NAB-16) runs the lint on the real tree and on the planted trees. Logged per STD-E2E:
counts of citations by form, clean and unclean, per finding kind as `metric` lines; the number of open reading
tasks per milestone; for a failure the path, line, source id and mark.

### Known answers and negative controls

Known answers on the fixtures: the grouping case yields exactly one task at M0; the mixed-readings source
(P for its abstract and Table A.5, S elsewhere) is clean for `#table-a5` and unclean for `#other`. Negative
controls: every row of the findings table as a planted defect; plus a control that the lint cannot be satisfied
by marking a reading P in the citing file (marks live only in the register); plus a register in which a `reread`
entry was edited (the loader of NAB-09 fails first). Metamorphic relation: reordering source records changes no
finding. Determinism lanes: twice on one machine and on both platforms, identical report bytes.

## Considerations

- Close reasons in the tracker export are scanned but never rewritten; a close reason with an unclean citation
  is reported to the exit review, which already rejects closes that do not cite their own test.
- A source's page may vanish. `result = "not-found"` keeps the reading uncitable and lists every dependent gate;
  the gate either finds another source or is restated as a measurement on our own hardware.
- The lint shares its file walker with the evidence-label lint (N00-01), the identifier lint (N00-03) and the
  glossary lint (NAB-02).
- Risk: authors avoid the lint by not labelling a number FACT. The evidence-label lint covers that side: a
  number with no label is itself a finding.

## References

Appendix H preamble, H.1 and H.6; §0.1 rules 1 to 3; §0.7 (the incumbent is measured before a mechanism is
funded against it); §38 (false-progress guards).
