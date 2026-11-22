---
key: "NAB-42"
title: "registries/ideas.toml: the idea-pass register (frob.ideapass/1), four rules of reading, traceability lint"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "decision-record", "test"]
parent: "EPIC-M0"
deps: ["BD-002", "BD-004", "BD-005", "BD-025", "STD-TEST", "NAB-32"]
needs: ["§38.13: `registries/gates.toml`, against which the gates and kill gates of a placed idea are resolved (N40-01, N40-02)", "§45.11: the tracker export of bead keys and states, against which an idea's carriers are resolved"]
related: ["NAB-43", "NAB-44", "NAB-45", "NAB-46", "NAB-47", "NAB-48", "NAB-49", "NAB-50", "NAB-22", "NAB-17", "N00-04", "N00-05"]
gates: []
plan_refs: ["Appendix B (preamble)", "Appendix B.2", "§0.7", "§0.8"]
---
## Background

The mechanisms of FrankenRobotics did not arrive as a design; they arrived as ideas, generated in several
independent passes, argued over, and winnowed. A first pass generated thirty ideas against the brief, thought
each through (how it works, how an agent would experience it, how it is built, how it fails), and ranked them
by one criterion: *obviously accretive and pragmatic for an agent designing a robot*, not mathematical
interest. Three further rounds followed, each against a later draft: in each, several independent passes
generated about thirty ideas apiece under one question (what would make this program an order of magnitude
better than existing systems at its stated purpose), each pass winnowed its own list and then examined the
others' lists, every idea received its strongest technical objection, and its proposer answered or conceded.
Audited reasoning passes checked claims against sibling source code, and the last round executed small exact
experiments on a CPU and on both GPU back ends. FACT (the planning review log): nine proposal passes produced
about 225 raw entries, which merge into 112 distinct ideas.

That record adds no design. What it adds is **traceability and memory**: for every surviving idea, where it
lives now and at what tier; for every contested idea, the strongest argument on each side and the measurement
that settles it; for every blind spot that only cross-examination exposed, the mechanism that now covers it;
for every refuted idea, the record that holds its refutation; and for every idea that no one has placed, a
line that says so. This bead builds the register that holds it, `registries/ideas.toml`, with the four rules
that govern how it is read and a lint that keeps it joined to the work. The content is carried by NAB-43
(first pass), NAB-44 and NAB-45 (ideas on which independent passes converged), NAB-46 (ideas placed on a
single proposal), NAB-47 (contested ideas), NAB-48 (blind spots) and NAB-49 (not yet placed).

**Four rules govern how this material is read.**

1. **Convergence is evidence about a problem, not about a mechanism.** An idea counts as converged when two or
   more of the nine passes proposed essentially the same mechanism without sight of each other. Several
   converged ideas were cut by every pass that proposed them, and several were refuted by calculation.
2. **No claimed ratio is a result.** Outside the executed plan-time checks, every ratio in the passes was
   derived on paper. A ratio is carried only as a HYPOTHESIS with its test, and the ceiling of the benefit is
   stated before any speed test.
3. **Every idea has one of three dispositions.** Placed at a tier, with a gate and, for Frontier and Moonshot,
   a kill gate with its fallback; refuted or declined, with what survives; or not yet placed.
4. **The owning text governs.** A row here is a pointer. The design, the thresholds and the fallback are in
   the bead that carries the idea; where a row and the gate register differ, the register governs.

Tally (FACT, by count): of the 112 distinct ideas, 90 are placed in full or in a narrowed form (some rows
merging several catalogued variants), 13 are refuted or declined outright, and 9 are not yet placed.

Design decisions (DESIGN): the register is data so that "every adopted idea has a carrier" is a test and not a
hope; row numbers of the planning tables become stable ids here, because the tables' own numbers were declared
unstable; the record is never cited as evidence for a mechanism (rule 1), and the lint enforces that.
Tier: Solid.

## Scope

In: the file and schema; the loader; the generator of `docs/decisions/idea-passes.md`; the four rules as
checks where mechanical; the traceability lint; the tally check. Out: the rows (NAB-43 to NAB-49); the
killed-idea register (NAB-32 onward); the sweeps that update `state` (NAB-23 to NAB-31).

## Technical approach

**1. File and schema.** `registries/ideas.toml`, schema `frob.ideapass/1`, JSON-Schema at
`schemas/registries/ideas.schema.json`.

```toml
schema = "frob.ideapass/1"

[[idea]]
id          = "IP-12"       # FP-01…FP-30 first-pass ideas; IP-01…IP-88 placed rows of the later rounds;
                            # CT-01…CT-19 contested records; BS-01…BS-27 blind spots; UP-01…UP-16 not yet placed
name        = "Many different robots in one launch"
proposals   = 6             # how many of the nine proposal passes proposed it independently; 0 when it arose in
                            #   an audited reasoning pass or was named only after the last cross-examination
mechanism   = "Morphology becomes table data read per world, so a population of different trees steps in one batch…"
where       = ["§17.3–§17.8", "§21.11", "§16.6"]
tier_text   = "[S] parameter variants; [F] model variants and design families"
tiers       = ["S", "F"]
gates       = ["GATE-KIN-06", "GATE-PHYLUM-02", "GATE-PHYLUM-03", "GATE-PHYLUM-04", "GATE-SPARK-10"]
kill_gates  = ["KILL-PHYLUM", "KILL-BUCKET", "KILL-FAMILYLANE"]
bets        = ["BET-18"]
disposition = "placed"      # placed | placed-narrowed | refuted | declined | parked | not-placed
contested   = "CT-01"       # the contested record, if any
killed_idea = []            # KI, KD or KP records that hold a refuted strong form or a declined idea
carriers    = ["BD-169", "BD-170", "BD-171", "BD-183", "BD-184", "BD-185", "BD-186", "BD-356"]
state       = "planned"     # planned | in-progress | delivered | fell-back | not-delivered (set by the sweeps)
```

**2. The rules as checks** (`crates/frob-conformance/tests/idea_register.rs`):

| Finding | Condition |
|---|---|
| `idea.placed-without-carrier` | `disposition` is `placed` or `placed-narrowed` and `carriers` is empty, or a carrier key is absent from the tracker export |
| `idea.placed-without-gate` | A placed idea names no gate, or names a gate or kill gate absent from the gate register (ideas recorded as "program rule" must name the gates of the rule) |
| `idea.upper-tier-without-kill` | `tiers` contains F or M and there is neither a kill gate nor a stated fallback in `tier_text` |
| `idea.refuted-without-record` | `disposition` is `refuted` or `declined` and `killed_idea` is empty or does not resolve |
| `idea.not-placed-without-default` | A `UP` record with no default disposition and no carrier |
| `idea.cited-as-evidence` | A gate record, a contract's Oracles section, an acceptance criterion or a bead close reason cites an `IP-`, `FP-`, `CT-`, `BS-` or `UP-` id, or the count of proposals, as support (rule 1: convergence is not evidence about a mechanism) |
| `idea.ratio-as-result` | A record's text states a ratio without the word HYPOTHESIS, TARGET or FACT within the sentence (rule 2) |
| `idea.tally` | The counts by disposition differ from the recorded tally without a history entry that explains the change |
| `idea.state-ahead-of-gate` | `state = "delivered"` while a named gate has no recorded pass |

**3. Tally arithmetic.** The plan-time counts are stored as `[tally] distinct = 112, placed = 90, refuted = 13,
not_placed = 9, raw_entries = 225, passes = 9` and checked as 90 + 13 + 9 = 112. The register holds more
records than 112 because it also carries the first-pass list, the contested records and the blind spots, which
overlap the 112; each record carries `counts_toward_tally = true | false`.

**4. Generator.** `xtask::generate::idea_passes` renders `docs/decisions/idea-passes.md` under
`check-generated`: the four rules; the tally; the tables by section with a carriers column and the current
`state`.

**5. Readers.** The traceability lint; the milestone sweeps, which set `state` from the carriers' gate records;
the revive lint, which uses `killed_idea` joins; the reader map's path for a reviewer who asks where an idea
went. Determinism class D0.

## Acceptance criteria

- [ ] `registries/ideas.toml` validates and loads; with NAB-43 to NAB-49 landed it holds 30 `FP`, 88 `IP`,
      19 `CT`, 27 `BS` and 16 `UP` records.
- [ ] Every `IP` record has at least one carrier that exists as a bead, and every named gate and kill gate
      exists in the gate register.
- [ ] Each of the nine findings is produced by its planted fixture.
- [ ] `[tally]` satisfies 90 + 13 + 9 = 112 and the generated document prints it with the label FACT.
- [ ] `docs/decisions/idea-passes.md` is current under `check-generated`; `xtask` still has six subcommands.
- [ ] No gate record, contract oracle or close reason in the tree cites an idea id as evidence.

## Test plan

### Unit tests

`crates/frob-conformance/tests/idea_register.rs` (fixture registries, labelled `synthetic`, and the real
registry for counts):

- `counts_by_prefix`: 30, 88, 19, 27, 16.
- `tally_adds_up`: 90 + 13 + 9 = 112.
- `placed_needs_carrier_and_gate`; `upper_tier_needs_kill_or_fallback`; `refuted_needs_record`.
- `idea_id_in_gate_record_is_refused`: a gate record whose `evidence` lists `IP-33` fails
  `idea.cited-as-evidence`; the same id in a bead's Background passes.
- `proposal_count_as_support_is_refused`: the sentence "proposed independently by seven passes, so it is
  adopted" in a contract fails.
- `ratio_needs_label`: "a factor of 12.7" without a label fails; "a factor of 12.7 (HYPOTHESIS as proposed)"
  passes.
- `delivered_needs_gate_pass`.
- `generator_is_deterministic`.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) loads the register, runs the lint against the real gate register
and tracker export, regenerates the document, and plants each finding. Logged per STD-E2E: counts by prefix and
by disposition, carriers resolved and unresolved (expected 0 unresolved), findings (expected 0).

### Known answers and negative controls

Known answers: 180 records (30 + 88 + 19 + 27 + 16); tally 112 = 90 + 13 + 9; nine passes; about 225 raw
entries. Negative controls: the nine findings; plus a carrier key that was renamed in the tracker (the lint
must name the idea and the missing key); plus a record whose `proposals` was raised to make an idea look
better supported (the frozen-count comparison fails). Metamorphic relation: permuting records changes no
finding.

## Considerations

- The register is provenance, not justification. The surest sign of misuse is an idea id in an acceptance
  criterion; the lint exists for that case.
- Carriers were assigned at conversion time from the seed set and from the beads other units had drafted;
  where a carrying unit had not yet drafted, the seed key stands. The lint's first real run is the check on
  that assignment.
- If the idea-pass document is not read by anyone by M3, the registry-must-have-a-reader rule reduces it to the
  frozen generated document.

## References

Appendix B preamble and B.2; §0.7; §0.8.
