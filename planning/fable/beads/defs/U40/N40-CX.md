---
key: "N40-CX"
title: "registry: planted defects of the gate register and milestone layer, each a control the tooling must catch"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-01"
deps: ["BD-025", "STD-TEST"]
needs: ["Appendix N: the printed counterexample catalogue has no section for the gate register or the milestones; if these rows are to receive CX and NC numbers, the unit that owns the catalogue's numbering assigns them (numbers are append-only)"]
related: ["N40-KA", "N40-01", "N40-02", "N40-03", "N40-04", "N40-05", "N40-06", "N40-07", "N40-08", "N40-09", "N40-10", "N40-11", "N40-23", "N40-24", "N40-25", "N40-26", "N40-27", "N40-29", "N40-30", "N40-31", "N40-32", "N40-33", "N40-35", "N40-36", "N40-37"]
gates: ["GATE-GAUNT-01", "GATE-GAUNT-10"]
plan_refs: ["§40.1", "§40.7", "§43.2", "§38.11", "Appendix N"]
---
## Background

"A gate that no deficient model can fail validates nothing." Every gate in the program must have a registered
**negative control**: a planted defect that its test catches. The rule applies with full force to the tooling that
*keeps* the gates: a status register whose refusals cannot be triggered, an exit query that nothing can block, or
a kill ledger that accepts a softened trigger is the failure the whole verification program exists to prevent.

The program's printed counterexample catalogue is organized by subsystem and has **no section for the gate
register or the milestone chapter**. This registry supplies the missing rows: for each rule of the register and of
the exit protocol, the tempting shortcut, the smallest planted case that takes it, and the rule the case protects.
Each row seeds a control in the conformance registry, attached to GATE-GAUNT-10 (status accounting) and, where it
concerns passing without a caught control, GATE-GAUNT-01.

## Scope

In: the rows; their home in the conformance registry; the binding check that each plant is caught. Out: known
answers (N40-KA); physics counterexamples (the chapter registries); the ten-fold harness's own controls (the
ten-fold chapter's registry; the program-gate beads state inline the ones they use).

## Technical approach

**1. File.** `crates/frob-conformance/registry/counterexamples/register.toml` (DESIGN path): `id`, `shortcut`,
`case`, `rule`, `mutant` (a scratch-copy edit script or a test-only fixture; never applied to the real register),
`caught_by` (test symbol), `gate`, `bead`. Each row also becomes a `[[control]]` record of kind `planted-input` in
`frob-conformance/registry/controls.toml`.

**2. Rows.**

| Row | The tempting shortcut | The planted case | The rule it protects | Caught by (bead) |
|---|---|---|---|---|
| RC-01 | Record a pass now and register the control later | A `pass` part whose gate has `controls = []` | A gate passes only with a registered control (rule 5; INV-GAUNT-02) | N40-01 |
| RC-02 | The passing test ran, so the gate passed | A `pass` part whose control has `caught = false` in the same run | A gate run in which a control was not caught is a fail | N40-01 |
| RC-03 | The implementer knows best whether it passed | `recorded.session` equal to `recorded.implemented_by` | No self-certification (INV-GAUNT-05) | N40-01, N40-05 |
| RC-04 | Cite the receipt; nobody will follow the link | An evidence item whose hash resolves to nothing | A record resolves in the artifact graph (rule 1) | N40-01 |
| RC-05 | The target is what we measured, near enough | `measured` byte-equal to `threshold` | A pass carries the measured value, not the target | N40-01 |
| RC-06 | Fix the status table by hand; regeneration is slow | The README table edited to show `pass` | Status tables are generated and never edited (INV-GAUNT-06) | N40-01, N40-36 |
| RC-07 | A gate can be both a known answer and a performance claim | A row with `claim = ["known-answer", "performance"]` | Exactly one of the seven claims (rule 3) | N40-02 |
| RC-08 | The mechanism slipped, so the gate's milestone slips with it | A part's milestone changed from M2 to M3 with no ADR | A gate may not move later without an ADR | N40-02, N40-04 |
| RC-09 | A gate nobody can pass can be left out of the table | One convergence row deleted | The register is the single list; counts are pinned | N40-02 |
| RC-10 | The trigger was a guess; adjust it now that we have data | A kill trigger lowered by an amendment dated after the governing receipt | A trigger changes only by a dated amendment made before the measurement (rule 2) | N40-03 |
| RC-11 | The fallback wording can be tidied at decision time | A fallback text differing from the amendment replay | Never quietly re-scoped (rule 2) | N40-03 |
| RC-12 | Keep the capability's name; add a footnote about the fallback | A fired gate whose capability still prints under its original name | A capability that fell back is described by the fallback's name | N40-03, N40-37 |
| RC-13 | The decision is obvious; the ADR need not cite numbers | A kill decision with no receipt cited | Every decision is an ADR with measured values beside the trigger | N40-03, N40-30 |
| RC-14 | A gate that is not ready is simply not due | A due part removed from an exit by editing its milestone | The exit's due set is computed from the register (rule 6) | N40-04 |
| RC-15 | The demonstration ran on my machine | A demonstration receipt from a tree with uncommitted changes | Every demonstration runs from a clean checkout | N40-05 |
| RC-16 | Close the remaining beads with the same note | Two bead logs with byte-equal close reasons | A shared close reason is a failure signal | N40-05 |
| RC-17 | The risk fired, but it is under control | A risk whose earliest signal fired, with no recorded outcome | A fired signal ends in a kill decision, an ADR or a written reason | N40-05 |
| RC-18 | This report is internal; it needs no reality line | A report with no `reality:` line | Every report prints its `reality:` line (GATE-GROUND-01) | N40-05 |
| RC-19 | One more Moonshot will not hurt | Five items active at one commit | At most four Moonshot items active at once | N40-05 |
| RC-20 | We are within the estimate, so the gate is fine | A gate threshold that cites a week count | The estimates are not acceptance evidence and no gate cites them | N40-06 |
| RC-21 | Enter the exit date by hand | An `exited` time the bead logs do not support | Start and exit times come from the logs | N40-06, N40-11 |
| RC-22 | The sibling was admitted at M0; that covers everything | A group first used at M3 with only an M0 record at an old pin | An admission is never inherited (rule 7) | N40-07 |
| RC-23 | It is only a small dependency | An M1 crate linking a GPU driver package | The dependency firewall per milestone | N40-07 |
| RC-24 | The pin move is a patch release | A sibling commit changed inside the tournament window | A pin move voids the runs it spans | N40-07 |
| RC-25 | Re-run my gate's copy of the experiment on fresh seeds | One member of a shared-fixture group citing its own receipt | One fixture, run once per milestone | N40-08 |
| RC-26 | The ledger allows 10⁻³, so the port balance can too | GATE-SINEW-02's bound changed to 10⁻³ | A looser bound on a wider scope does not relax a tighter one | N40-08 |
| RC-27 | A lower τ pair would let more rungs select | The pair (0.6, 0.3) entered for a rule that depends on τ | One pair of thresholds, 0.7 and 0.5, everywhere | N40-08 |
| RC-28 | The chapter's claim label is good enough | An owner declaration with claim "method" | The register governs the claim | N40-09 |
| RC-29 | The chapter's milestone is the real one | An owner declaration stating GATE-HAB-11 at M3 | The register follows the milestone skeleton | N40-09 |
| RC-30 | The chapter's trigger is more nuanced | An owner declaration with a changed trigger number | The register governs triggers | N40-09 |
| RC-31 | Everyone knows who owns that gate | A register row with no owner declaration | Each gate has exactly one owner (rule 9) | N40-09 |
| RC-32 | Both chapters' clauses should count | A consumer declaration turning the effective-sample-size diagnostic of KILL-DR into a trigger | The owner chapter governs; the consumer restates only | N40-09 |
| RC-33 | Gates added later are tracked in their chapters | A late gate absent from its exit's report | Late gates are binding and listed | N40-10 |
| RC-34 | The index's statement is the criterion | A late gate's brief entered from the abridged index | The defining row governs; no cut text is entered | N40-10 |
| RC-35 | Two similar names are the same gate | KILL-EXPODOMAIN entered with Sensorium as owner | KILL-EXPODOMAIN is Arena's; KILL-EXPOSURE is Sensorium's | N40-10 |
| RC-36 | Reclassify the delay afterwards | Blocked events retagged from `sibling-primitive-missing` to `own-work` | Logs are append-only; attribution uses the tag at the time | N40-11 |
| RC-37 | The estimate was too tight; raise it | `estimate_weeks` raised with no dated amendment | Estimates change only by a dated amendment | N40-11 |
| RC-38 | The ledger will catch up | A kill gate declared by a crate and absent from the ledger | The ledger lists every kill gate, once | N40-30 |
| RC-39 | Film is film | Habitat's advected-film trigger entered under KILL-FILM | Two mechanisms, two gates: KILL-FILM and KILL-ADVECT | N40-24 |
| RC-40 | Either clause is bad enough | KILL-FAMILY evaluated as a disjunction of its two measured clauses | The trigger is a conjunction (or the pilot's falsifier) | N40-25 |
| RC-41 | One false certificate is an outlier | A false certificate recorded and its proof kind left in service | KILL-AUGUR-FALSECERT: suspend, downgrade, decide the cause | N40-26 |
| RC-42 | MCP will not make it; take the fallback now | KILL-MCP recorded as fired before either upstream item was attempted | The trigger requires both attempts | N40-27 |
| RC-43 | A kill gate with no milestone has no deadline | A late kill row with an empty decision point | Decided at the milestone of the gate beside it | N40-29 |
| RC-44 | "At most" and "below" are the same | A declaration softening "below 0.5" to "at most 0.5" | Numbers and comparison direction must match | N40-30 |
| RC-45 | The kill is recorded; the tracker can stay as it is | A fired gate whose dependent is left open on the killed capability | Dependents are re-read: superseded, or standing on the fallback | N40-30, N40-37 |
| RC-46 | The bet looks fine so far | A bet marked `held` while its deciding gate is `not-run` | A bet is decided by a recorded gate | N40-31 |
| RC-47 | This other result also supports the bet | A gate cited as deciding a bet that the table does not list for it | The register's table governs deciders | N40-31 |
| RC-48 | Come back to that bet next milestone | A bet with a failed decider and no decision at its exit | An open bet blocks the exit | N40-31 |
| RC-49 | Every M2 gate obviously needs a GPU | A device-only gate added to the M2 due set with no CPU part and no variant listing | No milestone is blocked by one failed bet; the no-GPU variant must still exit | N40-32, N40-33 |
| RC-50 | The fallback is in force; the paperwork can follow | A fallback exit taken with no kill decision recorded | The exit is ready because the decision is recorded, not because failures are ignored | N40-32 |
| RC-51 | Under the variant, skip the whole gate | A gate with a CPU tier listed wholesale as `not run` | CPU rows run and must pass under the no-GPU variant | N40-33 |
| RC-52 | Nobody answered, so assume yes | A question past its deadline with no ADR and work proceeding on the generous answer | Nothing is decided by silence | N40-35 |
| RC-53 | A sensible default is to assume the catalogue price | A default that adds a claim | A default removes a claim or a capability; it never adds one | N40-35 |
| RC-54 | Reuse the old identifier for the new question | A tombstoned identifier given a new question | A closed question is tombstoned, not reused | N40-35 |
| RC-55 | The pass is recorded; the receipt can be collected | A recorded pass whose receipt was deleted | A claim whose closure dangles prints `evidence: unavailable` and cannot be cited | N40-36 |
| RC-56 | One more repair cycle and it will pass | A second repair cycle with no `deferred` ADR | One repair cycle; a second is a deferred decision | N40-37 |

**3. Binding check.** A harness test (`register_controls_bite`) asserts for each row that (a) on the clean
fixture the named test passes, and (b) with the row's mutant applied in a scratch copy the same test fails with
the expected reason. A row whose mechanism has not landed is `pending: <bead>`. Under the conformance registry's
own rule, a control that has never failed its gate in any recorded run is itself suspect and is reviewed at each
milestone exit.

## Acceptance criteria

- [ ] The file holds rows RC-01 to RC-56, each with a mutant, a catching test and a gate.
- [ ] The binding check passes: every plant is caught by its named test with the expected reason, and the clean
      fixture passes.
- [ ] Each row is registered as a control of GATE-GAUNT-10 (and RC-01, RC-02 also of GATE-GAUNT-01), so that
      GATE-GAUNT-10 cannot record a pass without them caught.
- [ ] No mutant is ever applied to the real `registries/gates.toml`: mutants run only on scratch copies.
- [ ] Row identifiers are append-only.

## Test plan

### Unit tests

`cargo test -p frob-conformance register_controls_bite`: one generated case per row (56), plus
`clean_fixture_passes_all`, `mutants_touch_scratch_only` (the real register's digest is unchanged after the run)
and `ids_append_only`.

### End-to-end script and logging

`scripts/e2e/register_controls.sh`: applies each mutant in a fresh scratch copy, runs its catching test through
the conformance target, and records caught or not caught; asserts 56 of 56 caught and the clean fixture passing.
Logs per row: `ts`, `script`, `bead`, `step`, `event`, the command, exit code, wall time, the expected and
observed refusal reason, the scratch path, and on failure the first row not caught; the summary names
GATE-GAUNT-01 and GATE-GAUNT-10.

### Known answers and negative controls

This bead is the registry of negative controls for its unit. Its own control: a mutant made inert (an edit script
that changes nothing) must be reported as *not caught* and fail the binding check, which shows the check
distinguishes a caught plant from a plant that was never applied.

## Considerations

- Several rows describe human shortcuts more than code defects. They are still testable, because each leaves a
  trace in a file that a rule constrains.
- A defect found in use (a softened trigger that slipped through, a pass that was void) becomes a permanent row
  with the case that exposed it.
- If the catalogue's owner assigns CX and NC numbers, the `RC-` identifiers stay as aliases.

## References

Plan §40.1, §40.7, §43.2, §38.11 (the five rules of the negative-control registry), §0.7. Appendix N (no section
for these chapters). Gates GATE-GAUNT-01, GATE-GAUNT-10. Invariants INV-GAUNT-02, -05, -06, -07.
