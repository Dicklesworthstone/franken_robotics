---
key: "NAB-32"
title: "killed-idea register: registries/killed_ideas.toml, docs/decisions/killed-ideas.md and the revive lint"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "decision-record", "test"]
parent: "EPIC-M0"
deps: ["BD-002", "BD-004", "BD-005", "BD-025", "N00-10", "NAB-17", "STD-TEST"]
needs: []
related: []
gates: []
plan_refs: ["Appendix C (preamble)", "Appendix C.2 (preamble)", "§0.7", "§0.8"]
---
## Background

One of the hygiene rules of FrankenRobotics is: **killed ideas stay on the page with the calculation that
killed them.** A mechanism whose strong form was refuted is not built in that form, and its narrowed form is
written with the claim that survived. The reason is practical. A multi-agent program rediscovers its rejected
ideas constantly: half-precision contact solves on tensor cores, copy-on-write forks on the device, one
universal policy for every robot, gradients through every contact. Each sounds right, each was proposed by
more than one independent planner, and each was refuted by arithmetic that takes a paragraph. Without a
register, the paragraph is redone every few weeks, or worse, it is not redone and the idea is built.

The register holds 67 entries. Each gives the **strong form** as it was proposed (its numbers are the
proposal's own: quoted, not endorsed), the **refutation** with its numbers, and **what survives**, with the
section, gate and kill gate that hold the surviving form. An entry that corresponds to a planning hypothesis
carries the hypothesis identifier and its state (ACTIVE, NARROWED, REFUTED, DEFERRED). The **reversal
condition** of an entry is the measurement named in its "survives" line: a verdict changes only on new
evidence of the kind that produced it (a pilot, a measurement at a gate, or a primary source re-read at a
pinned version). The rule of the hypothesis register is that a strong form appears **once** in the project's
documents, and this register is the place.

This bead builds the register as data, the generated document `docs/decisions/killed-ideas.md`, and the
**revive lint**: a proposal that matches a killed idea must cite its record, and may proceed only as the
surviving form or under an accepted revival ADR whose evidence is of the right kind. The entries are carried
by NAB-33 to NAB-40. Shortcuts that are wrong for a hand-checkable reason, as distinct from mechanisms that
were proposed and deflated, live in the counterexample catalogue and are not duplicated here.

Design decisions (DESIGN): entries are data so that their arithmetic can be recomputed by a test and their
revival conditions can be read by the milestone sweeps; the document is generated so that it cannot drift
from the data; revival is an ADR, never an edit. Rejected: free-text keyword blocking with no escape (it
would also block the surviving forms, which share vocabulary with what they replaced). Tier: Solid.

## Scope

In: the file and schema; the loader; the recomputation harness for `calculations`; the generator; the revive
lint; the revival ADR template. Out: the entries (NAB-33 to NAB-40); the discrepancy pins (NAB-41); the
end-to-end script (NAB-50).

## Technical approach

**1. File and schema.** `registries/killed_ideas.toml`, schema `frob.killedidea/1`, JSON-Schema at
`schemas/registries/killed_ideas.schema.json`.

```toml
schema = "frob.killedidea/1"

[[idea]]
id          = "KI-06"                    # KI-nn register entries (never renumbered); KD-nn declined with no
                                         # refuting calculation on file; KP-nn parked from the not-yet-placed list
group       = "arithmetic-precision-kernels"
title       = "Unordered floating-point scatter, accepted if 99.9% of windows replay bitwise"
hypothesis  = []                         # e.g. ["H-15"]
hypothesis_state = ""                    # REFUTED | NARROWED | DEFERRED | ACTIVE | "" (as the register prints it)
state       = "refuted"                  # refuted | narrowed | deflated | deferred | assumed | declined |
                                         #   parked | withdrawn-claim | process. `assumed` marks an entry kept
                                         #   as a record of what was assumed while its hypothesis is ACTIVE
strong_form = "Drop the ordered scatter for unordered atomics if the two agree on 99.9% of windows."
refutation  = "0.999^1000 = 0.37: a campaign of a thousand windows would replay exactly about one time in three…"
survives    = "A stable-key sort and a segmented reduction; integer atomics only for maximum, minimum, or and wrapping add."
revive_only_if = "Never on a replayable profile; admissible only inside the labelled D4 profile."
revive_evidence = "none"                 # pilot | gate | primary-source | none (cannot be revived by measurement)
revive_gates = []                        # gates or kill gates whose records can meet the condition
sections    = ["§35.7"]
gates       = []
kill_gates  = []
bets        = []
refusals    = []
carriers    = ["BD-152", "BD-181"]
sources     = []                         # source-register ids behind FACT figures
keywords    = ["unordered scatter", "floating-point atomics", "non-deterministic reduction"]
ornamental  = []                         # KI-50 only: the labels of the glossary's [[ornamental]] records (NAB-38)
restates    = []                         # KI-64 to KI-66: the earlier records the entry restates (NAB-40)
reason_on_file = ""                      # KD-nn: "none" when no refuting calculation is on file (NAB-40)
reason_proposed = ""                     # KD-nn and KP-nn: the reason proposed for the owner to confirm; their
                                         #   "Reopened by" text goes in revive_only_if (NAB-40, NAB-49)
owner_confirmed = false                  # KD-nn and KP-nn: true only with the owner's decision record

  [[idea.calculation]]                   # every number of the refutation that can be recomputed
  name  = "window-survival"
  expr  = "0.999^1000"
  value = "0.3677"
  tol   = "5e-4"
  label = "DESIGN arithmetic"              # DESIGN arithmetic | FACT (FACT names a `sources` id)

  [[idea.method]]                        # KI-45 only: one sub-record per rejected method (NAB-37)
  method       = ""
  core         = ""
  why_not      = ""
  used_instead = ""                      # what is used instead; it exists as a bead
  keywords     = []                      # the revive lint matches each method separately

  [[idea.row]]                           # KI-61 and KI-63 only: one sub-record per row of their tables (NAB-39)
  withdrawn_claim  = ""                  # KI-61: withdrawn_claim, contradicted_by, claimed_instead
  contradicted_by  = ""
  claimed_instead  = ""
  item             = ""                  # KI-63: item, evidence_against, done_instead
  evidence_against = ""
  done_instead     = ""
  keywords         = []                  # the revive lint matches each row separately

  [[idea.history]]                       # appended only by an accepted revival ADR (step 5); none at plan time
  exit     = "M2"
  adr      = "ADR-REVIVE-KI-03"
  change   = ""                          # what the record now says, in one line
  evidence = []                          # gate records or pilot records the ADR rests on

[[alias]]                                # the refuted-or-declined index (NAB-40 step 3)
name     = ""                            # the name under which the idea is usually proposed
survives = ""                            # what survives of it, in one line
records  = []                            # the KI, KD or KP records the name resolves to
```

Fields that only some records use (`ornamental`, `restates`, the `KD` and `KP` fields, `[[idea.method]]`,
`[[idea.row]]`) are optional in the JSON-Schema and absent elsewhere; any field not listed here is refused.

Groups (nine: the register's eight, and the four further entries as the ninth): arithmetic, precision and kernels (KI-01 to KI-09); memory, forks,
telemetry and scheduling (KI-10 to KI-16); worlds and physics (KI-17 to KI-25); control, learning and transfer
(KI-26 to KI-32); search, statistics and decision rules (KI-33 to KI-46); algebra, synthesis, proofs and
certificates (KI-47 to KI-56); reality, structure and the thesis (KI-57 to KI-59); claims, wording and process
(KI-60 to KI-63); and four further entries recorded so they are not proposed again (KI-64 to KI-67).

**2. Recomputation harness.** `crates/frob-conformance/tests/killed_ideas.rs` evaluates every
`[[idea.calculation]]`: `expr` is a small arithmetic expression (integers, decimals, `+ − × ÷ ^`, `sqrt`, `ln`,
`log2`, `C(n, k)`, parentheses) evaluated in exact rationals where possible and in `f64` otherwise, compared
with `value` within `tol`. A calculation labelled `FACT` must name a `sources` id; one labelled `DESIGN
arithmetic` must not depend on a source. The harness is the standing form of the statement "every refuting
calculation was recomputed".

**3. Generator.** `xtask::generate::killed_ideas` renders `docs/decisions/killed-ideas.md` under
`check-generated`: the labels paragraph, then per group each entry as title, hypothesis and state, strong form,
refutation, what survives, the revival condition and the carrying beads; then the index by hypothesis.

**4. The revive lint** (`crates/frob-conformance/tests/revive_lint.rs`).

*What is scanned:* the tracker export of bead titles and descriptions; every `CONTRACT.md` Purpose and Known
gaps section and every feature flag in `contract.toml`; work-package funding records; `registries/ev_matrix.toml`
(moves out of the parked table); ADRs under the ADR directory.

*What matches:* a scanned text matches a record when it contains one of the record's `keywords` as a phrase
(case-insensitive, word boundaries) in a sentence that proposes, builds, enables or funds something (verbs
`add`, `build`, `implement`, `enable`, `fund`, `adopt`, `use`, or the text is a bead title or a feature flag).

*What a match must satisfy,* one of:

1. It cites the record (`KI-nn`) and describes the **surviving form**: the same sentence or bead names one of
   the record's `gates`, `kill_gates` or `carriers`. Most matches are this case: the bead that builds the
   segmented reduction mentions the unordered scatter it replaces.
2. It cites the record and a **revival ADR** `ADR-REVIVE-KI-nn` exists, is accepted, and states (a) the new
   evidence, (b) its kind, which must equal the record's `revive_evidence`, (c) the measured values against
   `revive_only_if`, (d) the gate records or pilot records it rests on. A record with `revive_evidence = "none"`
   cannot be revived by measurement; its ADR must instead show that the refuting argument no longer applies
   (for example the profile on which a replay claim is made has changed class) and needs the owner's decision.
3. It is inside the register, its generated document, the counterexample catalogue, or a test fixture.

*Findings:* `ki.match-without-citation` (path, line, the keyword, the record); `ki.revival-without-adr`;
`ki.adr-evidence-kind-mismatch`; `ki.adr-condition-not-met` (the ADR's measured values do not satisfy the
condition as written); `ki.strong-form-as-goal` (a goal, bet, gate threshold or claims-registry wording states
the strong form's claim of a REFUTED or NARROWED record); `ki.strong-form-duplicated` (the strong-form text
occurs verbatim outside the register, its printout and quoted fixtures; no pattern is involved); `ki.refusal-missing` (a record names a refusal
code that the refusal registry lacks); `ki.carrier-missing` (a carrier key that no bead has).

*One rule, one pattern list (DESIGN; stated identically in N00-10).* The hypothesis verdict register has a
wording lint for the same rule as `ki.strong-form-as-goal`: a match of a `banned` pattern of a REFUTED or
NARROWED hypothesis row in documents, contracts, help text and report templates (its finding
`hyp.banned-wording`). Two pattern lists for one strong form could disagree, so there is one. For a record
whose `hypothesis` field names a row, `ki.strong-form-as-goal` is computed from that row's `banned` patterns,
read through `frob_conformance::hypotheses::banned(id)` of N00-10, and the finding carries the row id and the
index of the pattern; the record's `keywords` then serve only the proposal matching above
(`ki.match-without-citation`, `ki.revival-without-adr`). For a record with no hypothesis the record's
`keywords` are the patterns. The two lints differ in what they scan (this one: goals, bets, gate thresholds
and claims-registry wording, beside the inputs listed above); where both scan one file, both report, and the
shared row id and pattern index show that it is one cause. The loader refuses a record that names a hypothesis
identifier absent from the hypothesis register, and one whose `hypothesis_state` differs from the state that
register prints.

*Rows that land later (DESIGN).* Refusal rows are entered by each chapter's refusal-registry bead. Of the three
codes named at plan time, `FROB-REF-214` (Marrow) is entered at M0, and `FROB-REF-245` (Traction) and
`FROB-REF-817` (Reflex) at M1. The codes the records name are frozen with the milestone of their registry row in
`tests/fixtures/ki/refusals.tsv`; a code of that list that is absent from the registry is reported with a
recorded `skip` while the exit record of that milestone does not exist, and is `ki.refusal-missing` afterwards.
A kill gate named by a record follows the same rule (the frozen list and the ledger groups of NAB-17 step 6).
The M1 sweep (NAB-24) requires that no such skip remains.

**5. The revival ADR template** (`docs/adr/ADR-REVIVE-KI-nn.md`, in the directory and form the kill-gate
decision ADRs use; every decision record lives in `docs/adr/`): record id; the strong form quoted; the refutation's numbers; the new evidence
with receipts; the condition as written and as measured; what is proposed (usually the strong form at a lower
tier with its own pilot and kill gate); the owner's decision. An accepted revival appends a history entry to
the record; the record is never deleted.

**6. Readers.** The revive lint; the hypothesis register's once-only rule; the expected-value matrix (rejected
rows point here); the milestone sweeps (which evaluate `revive_only_if` when its gates are recorded); the
reader map's path for a sceptical reviewer. Determinism class D0.

## Acceptance criteria

- [ ] `registries/killed_ideas.toml` validates and loads; with NAB-33 to NAB-40 landed it holds 67 `KI`
      records, the `KD` and `KP` records those beads define, and every record has `strong_form`, a refutation
      or a stated reason, `survives`, `revive_only_if` and at least one carrier or the note "nothing survives".
- [ ] The harness recomputes every `calculation` within tolerance.
- [ ] `docs/decisions/killed-ideas.md` is current under `check-generated`; `xtask` still has six subcommands.
- [ ] Each of the eight findings of the revive lint is produced by its planted fixture, and the surviving-form
      case passes without an ADR.
- [ ] A revival ADR whose evidence kind differs from the record's is refused; one whose measured values do
      not meet the condition is refused.
- [ ] Every refusal code named by a record (`FROB-REF-245`, `FROB-REF-214`, `FROB-REF-817` at plan time) exists
      in the refusal registry; the two whose rows are entered at M1 are reported under a recorded `skip` until
      the M1 exit and not later.

## Test plan

### Unit tests

`crates/frob-conformance/tests/killed_ideas.rs`:

- `expr_evaluator`: `0.999^1000` → 0.3677; `1 + 48/0.01` → 4801; `C(62, 2)` → 1891; `ln(20)/1e-5` → 299573;
  `(13 − 3·sqrt(17))/4` → 0.157671; `log2(1e4)` → 13.29; `2^(−13.29/20)` → 0.631.
- `fact_calculation_requires_source`; `design_arithmetic_has_no_source`.
- `every_record_is_complete` (once the entries exist).

`crates/frob-conformance/tests/revive_lint.rs` (fixture trees, labelled `synthetic`):

- `surviving_form_passes`: a bead "Segmented reduction `K-seg-reduce` replacing unordered scatter (KI-06)" with
  carrier BD-152 passes.
- `match_without_citation_fails`: a bead titled "Copy-on-write forks for device state" with no `KI-10`.
- `revival_without_adr_fails`: the same bead citing `KI-10` and proposing to build the strong form.
- `adr_kind_mismatch_fails`: a revival of KI-03 (evidence kind `gate`: the M2 profile's specializable share)
  whose ADR cites only a published paper.
- `adr_condition_not_met_fails`: KI-03's ADR reporting a share of 0.25 where at least one third is required.
- `adr_accepted_passes`: the same with a share of 0.36 and the profile's receipt.
- `none_kind_needs_owner_decision`.
- `strong_form_as_goal_fails`: a gate threshold "no-adaptation return at 70% of specialists on any unseen
  design" (the strong form of KI-27).
- `strong_form_duplicated_fails`: KI-01's strong-form sentence pasted into a contract.
- `as_goal_uses_hypothesis_patterns`: for a record tied to a hypothesis, the finding carries that row's id and
  pattern index, and removing the pattern from the hypothesis row (in a fixture copy) removes the finding;
  `hypothesis_state_must_agree`: a record whose `hypothesis_state` differs from the register's is refused.
- `refusal_missing_fails`; `carrier_missing_fails`.
- `register_and_fixtures_are_exempt`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) loads the register, runs the harness and the lint on the real tree,
regenerates the document, and plants the eight findings. Logged per STD-E2E: counts of records by state and
group, calculations recomputed and mismatched (expected 0), matches by case (surviving form, revival, exempt),
findings (expected 0), digests of the registry and the document.

### Known answers and negative controls

Known answers: 67 entries in the groups 9, 7, 9, 7, 14, 10, 3, 4 and 4; the evaluator values above. Negative
controls: the eight findings; plus a calculation whose stored value was altered (2.75 for the refinement ratio
at n = 18, k = 6, where 9k/n = 3.0); plus a record deleted from the file (the loader compares ids against the
frozen id list and fails `ki.record-removed`). Metamorphic relation: reordering records changes no lint result.

## Considerations

- Keyword matching is crude on purpose. The escape for honest work is cheap (cite the record and name the
  surviving gate); the cost of a false negative (building a refuted mechanism for a month) is high. If the
  lint's citation burden exceeds a handful of beads per milestone, the keyword lists are tightened.
- The register quotes strong forms with their original numbers. Those numbers are not claims of this project
  and the generated document says so at its head.
- A revival is not a defeat of the register. Several entries name a measurement that is scheduled; if it comes
  back the other way, the ADR is the correct outcome and the history shows why the idea was right to wait.

## References

Appendix C preamble and C.2 preamble; §0.7 (killed ideas stay on the page; mechanisms are named plainly);
§0.8 (the four states; a verdict changes only on new evidence of the kind that produced it).
