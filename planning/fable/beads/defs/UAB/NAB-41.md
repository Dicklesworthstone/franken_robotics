---
key: "NAB-41"
title: "frob-conformance: discrepancy pins: eight figures that did not follow from their formulas, nine more found"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "decision-record", "test"]
parent: "EPIC-M0"
deps: ["BD-025", "N00-07", "N00-17", "N38b-50", "N40-02", "N40-27", "NAB-05", "NAB-06", "NAB-14", "NAB-15", "NAB-18", "NAB-21", "NAB-32", "NAB-33", "NAB-34", "NAB-35", "NAB-39", "NAB-40", "NAB-43", "NAB-49", "STD-TEST"]
needs: []
related: ["NEG-03", "NEG-05", "NEG-06"]
gates: []
plan_refs: ["Appendix C.3", "§0.1", "§2.3", "§18.12", "§19.10.1", "§21.20", "§40.7", "Appendix M.38", "Appendix E.7"]
---
## Background

When the killed-idea register of FrankenRobotics was compiled, **every refuting calculation in it was
recomputed**. Six figures, in the planning record or in other chapters, did not follow exactly from their own
formulas, and two assignments differed between chapters. None changes a conclusion, and five were corrected in
the chapters. They are worth keeping as data for one reason: a wrong figure that has been corrected once tends
to come back, copied from an older note or from an agent's memory of the earlier text. A pin is a test that
recomputes the right value and a lint that refuses the withdrawn one in its context.

While the specification was being converted into work items, nine further inconsistencies of the same kind
were found (a stale archive digest, stale bead numbers in an idea list, a glossary row that describes an
earlier state of a register, a missing glossary row, three declined ideas with no recorded reason, a list of
"not yet placed" ideas of which seven had since been placed, two texts naming different robots for one
measurement, a reference to marks that no source carries, and a milestone stated two ways). They are pinned
here in the same form so that the beads that corrected them cannot silently regress.

Two other lists of the same kind exist, each made by a different recomputation: the reconciliation register
of the known-answer suite (`crates/frob-conformance/registry/reconciliations.toml`, records REC-01 to REC-11,
bead N38b-50) and the discrepancy register of the worked cards (`registries/cards/discrepancies.toml`, twelve
items E7-…, bead NEG-03). Two records here are also records there: DX-02 is REC-02 and E7-2 (the ceiling at a
step share of 87%), and DX-03 is REC-01 and E7-3 (the solver pilot's ceiling). The three registers stay
separate files, but the search for a withdrawn figure in its context is one engine, built in N38b-50 and run
by all three.

Tier: Solid. This is hygiene, bounded to seventeen records and one test file.

## Scope

In: seventeen `[[discrepancy]]` records in `crates/frob-conformance/registry/discrepancy_pins.toml`; the
recomputation tests; the owner-agreement tests; the withdrawn-figure lint, which is the shared scan run over
these records and reported as the finding `dx.withdrawn-figure`; the links of DX-02 and DX-03 to the
reconciliation register.

Out, with the bead that has it:
- the scan engine (entry form, the 200-character window, the walk set, the exemption list with the marker
  `stale-as-control: <id>`, the hit record, link resolution): N38b-50. This bead registers its file as a
  source of that engine and specifies none of the engine's rules a second time;
- the settled values of the two shared records in the known-answer rows: N38b-51 (REC-02) and N38b-50
  (REC-01); the print rule and the ceiling formatter for the first: NEG-05; the rule that a ceiling is a ratio
  of two timed totals, for the second: NEG-06; the twelve E7 items: NEG-03;
- the calculators and registries that own the corrected values (named per record);
- the general wording and evidence-label lints (N00-01).

## Technical approach

**1. Record** (in `crates/frob-conformance/registry/discrepancy_pins.toml`, schema `frob.discrepancy/1`.
DESIGN: the file is named for the pins, after its test target `discrepancy_pins`, and not
`discrepancies.toml`, because `registries/cards/discrepancies.toml` is the worked cards' register of NEG-03
with a different schema, and two registers of one base name in one workspace would be taken for each other):

```toml
[[discrepancy]]
id        = "DX-02"
topic     = "step share and its ceiling"
withdrawn = "a ceiling of about 7.8 at a step share of 87%"
correct   = "1/(1 − 0.87) = 7.69, printed as about 7.7"
expr      = "1/(1 - 0.87)"
value     = "7.692"
tol       = "1e-3"
context   = ["step share", "87%", "ceiling"]       # words that must co-occur with the withdrawn value
withdrawn_tokens = ["7.8"]                         # the token refused in that context
owner     = "N00-17"                               # bead or registry that holds the corrected value
status    = "corrected-in-chapters"                # corrected-in-chapters | consistent-as-printed | resolved-by-register |
                                                   #   corrected-in-conversion | open
same_as   = ["REC-02"]                             # the reconciliation register's record of the same correction
                                                   #   (DX-02 → REC-02, DX-03 → REC-01; absent on the other fifteen)
```

`context` and `withdrawn_tokens` are this register's form of a stale entry of the shared scan: the loader
hands each token to the engine as a literal `pattern` with the `context` words as its `all_of` list. A linked
record's `value` is compared with the figure the REC record settles, within `tol` (7.692 against 100/13;
1.464 against 8428/5757).

**2. The eight recorded discrepancies.**

| Id | What was printed | Recomputation | Standing |
|---|---|---|---|
| DX-01 | Refinement cost against factorization cost at n = 18: 2.75 | With k = 6 refinement steps the ratio is 9k/n = 54/18 = 3.0. The contact chapter prints 3.0 and is correct; the device chapter's list of rejected methods omits the case | Consistent as printed where it is printed; the killed-idea record KI-01 holds 3.0 |
| DX-02 | Step share 87% with a ceiling of about 7.8 | 1/(1 − 0.87) = 7.69; the two agreed only at a share of 87.2% (1/(1 − 0.872) = 7.81) | Corrected: the ceiling is printed as about 7.7, and the share is re-read from its primary source before the ten-fold test cites it |
| DX-03 | An unremovable share of 94%, a measured ratio of 1.345 and a ceiling of 1.46 for the solver pilot | 1.345/0.94 = 1.43, not 1.46 | Corrected: the share attribution is withdrawn. The specialized batch holds a residual of 575.7 µs that the solver does not touch and that was not separately timed, and the ceiling of about 1.46 is 842.8 ÷ 575.7 |
| DX-04 | Orbit cost "2% to over 100% of the cost of a training run" for solves of 4 to 260 s | That range is against a four-minute run (4/240 = 1.7%; 260/240 = 108%) | Corrected: against the 410 s figure used elsewhere it is about 1% to 63% (4/410 = 0.98%; 260/410 = 63.4%) |
| DX-05 | Lattice Mach number 5.2 at 2 mm cells beside 0.35 at 10 cm | At one flow speed of 1 m/s and a 20 ms step, Ma = √3·u·Δt/Δx is 17.3 at 2 mm; 5.2 corresponds to 0.3 m/s | The living-worlds chapter prints 0.35, 3.5 and 6.9 at one speed and is consistent; KI-18 holds those three |
| DX-06 | Fork copy cost "about a millisecond" and "a quarter of one batch step" for a copy of 0.25 GB | At 1,008 GB/s, read plus write is 0.49 GB of traffic: about 0.49 ms, an eighth of a 4.1 ms batch step | Corrected to about 0.5 ms; KI-10 holds both the original upper bounds and the recomputed value |
| DX-07 | Lifecycle compression: landed with frontier physics at M6 in the milestone skeleton, with its gate and kill gate recorded at M8 in the structure chapter | The gate register places both at M6, and the milestone chapter agrees | Resolved by the register: M6. The Moonshot queue's wording "the gate is recorded at M8" is kept verbatim with the register's milestone beside it (NAB-21) |
| DX-08 | Kernel reflection tiered Frontier with a pilot at M5 in the proof chapter, and Moonshot in the foundation rows of the suite chapter | One mechanism, two tiers | Corrected: Frontier in both places; it is not counted against the Moonshot cap, and moves to the parked table if its kill gate fires |

**3. Nine found during conversion** (each corrected in a bead of this set, named in the last column).

| Id | What the text says | What is the case | Where it is corrected |
|---|---|---|---|
| DX-09 | The planning reproduction archive is 241,568 bytes with SHA-256 `0326e89d…9a4f` | The file in the working tree is 275,178 bytes with SHA-256 `9620da42…7980`; the cited digest is the archive two commits earlier | NAB-15 records all four revisions and pins the cited one |
| DX-10 | The first-pass idea list cites bead numbers for its best fifteen ideas (for example the trust ladder at BD-104 and BD-112) | Those numbers belong to an earlier numbering; in the seed set BD-104 is the self-collision analysis and BD-112 is `explain.run` | NAB-43 drops the stale numbers and gives carriers by current key |
| DX-11 | Sixteen ideas are "not yet placed: no chapter contains these" | Seven of them were placed by later-added subsections (closed-form loop closures; far-field reuse; controller-budget curves; task audit by foil bodies; the item bank; placement and sizing along a motion; visibility under self-occlusion). Nine remain, which equals the tally "9 not yet placed" printed elsewhere | NAB-49 |
| DX-12 | The glossary defines the subsystems | 22 of the 23 subsystem names are defined; `Reflex` is missing | NAB-06 adds the row |
| DX-13 | Glossary: a hypothesis verdict is "supported, narrowed or refuted", for H-01 to H-24 | The register has four states (ACTIVE, NARROWED, REFUTED, DEFERRED) and rows to H-29 | NAB-05 corrects the row |
| DX-14 | Every idea of the refuted-or-declined list "is treated in the killed-idea register with the calculation or evidence that refuted it" | Three have no entry: debates as a product lever; custom stator and gearbox co-design; live editing in the viewer | NAB-40 enters them as declined records pending the owner's confirmation |
| DX-15 | The unretained CPU-rate check measured "a reference quadruped at 2 ms" (the checks table) and "a humanoid" (the review log) | Two texts, two robots, one check | NAB-15 stores the table's wording; the re-run records the model |
| DX-16 | The not-verified list speaks of public "sources marked M or U" | No public bracket or reading is marked M or R (163 brackets: 39 P, 92 F, 30 S, 2 U; entered as 174 readings: 43 P, 95 F, 34 S, 2 U); the recalled sources are the standard references | NAB-14 enters the references with mark M |
| DX-17 | The expected-value tables hold "every construction C-16 to C-40" and, in the planning tally, 112 distinct ideas of which 90 are placed, 13 refuted or declined and 9 not placed | Both hold as counted: C-01 to C-40 all occur; 90 + 13 + 9 = 112. Pinned because both counts are easy to break when a row moves | NAB-17 and NAB-42 hold the checks |

**4. Tests and lint** (`crates/frob-conformance/tests/discrepancy_pins.rs`):

1. *Recompute.* Each record with an `expr` is evaluated and compared with `value`.
2. *Owner agrees.* For each record with an `owner` registry, the corrected value is read from that registry
   (the killed-idea record, the thesis calculator's fixture, the source register, the glossary) and compared.
3. *Withdrawn figure in context.* The file is registered as a source of the scan engine of N38b-50, with the
   finding code `dx.withdrawn-figure` and, as its declared exempt paths, the killed-idea document, the
   register it is printed from (`registries/killed_ideas.toml`) and the killed-idea beads' verbatim fixtures
   (`tests/fixtures/killed/`): the register keeps refuted strong forms and refutations verbatim, and KI-10's
   refutation states DX-06's original bounds ("about a millisecond", "a quarter of one 4.1 ms batch step") on
   purpose. The engine is run over the records. Over contracts, `docs/**`, `README.md`, registries, test directories and
   report templates (the engine's walk set), a match of one of `withdrawn_tokens` whose window contains every
   word of `context` is the finding `dx.withdrawn-figure` with the record id, every linked id and the correct
   value. The window is the engine's: 200 characters either side of the match on text with white space
   collapsed (DESIGN: this replaces "within one sentence", so that the three registers match by one rule; a
   sentence splitter breaks on decimal points, and the window keeps the result unchanged under reflow). The
   declared exempt paths above, the three register files, a record's derivation file and quoted fixtures are
   exempt; a quoted fixture is a line that carries the engine's marker `stale-as-control: <id>`.
4. *Links.* `resolve_links` of the engine is called: DX-02 and DX-03 name REC-02 and REC-01, and their values
   agree with the figures those records settle.

## Acceptance criteria

- [ ] Seventeen records `DX-01` … `DX-17` exist; every `expr` recomputes within tolerance.
- [ ] For each record the owner registry holds the corrected value (3.0; 7.69; 842.8 ÷ 575.7 = 1.464; 1% to
      63%; 0.35, 3.5, 6.9; about 0.49 ms; M6; Frontier; the four archive revisions; and so on).
- [ ] The lint fails each planted sentence of the test plan with `dx.withdrawn-figure` and passes the corrected
      sentence.
- [ ] No withdrawn figure occurs in its context anywhere in the tree outside the exempt files.
- [ ] The lint is the scan engine of N38b-50 run over this register: no second matching rule, walk set or
      exemption list exists in `discrepancy_pins.rs`.
- [ ] DX-02 and DX-03 resolve to REC-02 and REC-01 with agreeing values; a planted DX-02 sentence is reported
      once, under DX-02, REC-02 and E7-2.
- [ ] The test target runs in under 2 s (TARGET) and is not an `xtask` subcommand.

## Test plan

### Unit tests

`crates/frob-conformance/tests/discrepancy_pins.rs`:

- `dx01`: 9 × 6 / 18 = 3.0 (and the withdrawn 2.75 is refused in a sentence with "refinement" and "n = 18").
- `dx02`: 1/(1 − 0.87) = 7.692; 1/(1 − 0.872) = 7.8125.
- `dx03`: 1.345/0.94 = 1.4309; 842.8/575.7 = 1.4640; 610.1 − 34.4 = 575.7.
- `dx04`: 4/240 = 0.0167; 260/240 = 1.083; 4/410 = 0.0098; 260/410 = 0.634.
- `dx05`: √3 × 1 × 0.02/0.002 = 17.32; √3 × 0.3 × 0.02/0.002 = 5.196; and 0.346, 3.46, 6.93 at 10 cm, 1 cm, 5 mm.
- `dx06`: 2 × 0.2458 GB / 1,008 GB/s = 0.488 ms; 0.488/4.096 = 0.119 ≈ 1/8.
- `dx07`: the gate register's milestone for the lifecycle gate and its kill gate is M6.
- `dx08`: the pilot ledger's tier for kernel reflection is `frontier`, and the Moonshot cap count excludes it.
- `dx09`: the digest of the archive in the working tree equals one of the four recorded revisions, and the
  cited revision is the second.
- `dx10`: no document cites BD-104 or BD-112 as carriers of the trust ladder.
- `dx11`: seven of the sixteen not-yet-placed ideas have carriers; nine have a disposition and no carrier.
- `dx12`, `dx13`: the glossary has `Reflex`, and its `Hypothesis verdict` row lists four states.
- `dx14`: `KD-01` to `KD-03` exist.
- `dx15`: the source register's record `SRC-CHK-22` (PC-22) stores the checks table's wording "a reference
  quadruped at 2 ms" and names the review log's "humanoid" as the conflicting text; its re-run record, once
  written, names the model it ran.
- `dx16`: the public register has no reading marked R or M.
- `dx17`: C-01 to C-40 occur in the matrix; 90 + 13 + 9 = 112.
- `withdrawn_in_context_is_caught`: planted sentences "an infinitely fast step gives a ceiling of about 7.8 at
  a step share of 87%" (DX-02), "the unremovable share of 94% gives a ceiling of 1.46" (DX-03), "a fork copy
  costs a quarter of one batch step" (DX-06).
- `figure_out_of_context_is_not_caught`: "7.8 mm clearance" passes.
- `links_resolve_and_agree`: DX-02 → REC-02 (7.692 against 100/13 within 10⁻³); DX-03 → REC-01 (1.464 against
  8428/5757 within 10⁻³); a link to a record that does not exist fails.
- `quoted_fixture_is_exempt`: a planted sentence on a line marked `stale-as-control: DX-02` passes; the same
  sentence unmarked is the finding.
- `killed_idea_register_is_exempt`: KI-10's record in `registries/killed_ideas.toml` produces no finding; its
  refutation sentence copied into a contract is the finding under DX-06.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) runs the pins and the lint on the real tree and on the planted
sentences. Logged per STD-E2E: per record the recomputed value, the owner's value and the verdict; findings
(expected 0).

### Known answers and negative controls

Known answers: every value in the two tables. Negative controls: the three planted sentences; an owner registry
edited back to a withdrawn value (2.75 in KI-01's calculation table); an archive whose digest matches no
revision. Metamorphic check: the lint's result is unchanged by reflowing a sentence across lines.

## Considerations

- The pins are deliberately few. A general "numbers must be right" lint does not exist; these are the specific
  numbers that were once wrong.
- DX-07 and DX-08 are not arithmetic: they pin an assignment (a milestone, a tier) that two chapters stated
  differently. Their owner is the gate register and the pilot ledger.
- If a later review finds another figure that does not follow from its formula, it is appended as `DX-18`
  onward with the same three checks; the list is append-only.
- DX-02 and DX-03 are recorded three times in the workspace (here, as E7-2 and E7-3, and as REC-02 and
  REC-01) because three recomputations found them. The settled values live in the known-answer rows the REC
  records name; this register and the worked cards' register point at those records and must agree with them.

## References

Appendix C.3 (items 1 to 8); §0.1 (every number carries a label); §2.3; §18.12; §19.10.1; §21.20; §40.7;
Appendix M.38 (rows 1 and 2) and Appendix E.7 (rows 2 and 3), which record the same two corrections.
