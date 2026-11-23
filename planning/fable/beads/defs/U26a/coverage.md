# Coverage of drafting unit U26a

Chapter §26 Nemesis (adversarial worlds and rare failures): the chapter preamble and subsections §26.1 to §26.9.
Five seed beads (BD-270, BD-272, BD-273, BD-276, BD-277), thirty-five new beads (N26a-01 to N26a-35) and the
chapter's known-answer registry (N26-KA). The counterexample and refusal registries of the chapter (`N26-CX`,
`N26-REF`) are written by U26b; the beads here cite counterexample numbers and refusal codes with their content
inline and name the registries under `related`.

Three of this unit's subsections have their seed bead in another unit: §26.5 (the oracle, BD-271, U26b), §26.7
(shrinking, BD-274, U26b) and §26.8 (the hard-world set, BD-275, U15a, already written with its script N15a-28).
For §26.5 and §26.7 this unit wrote the component beads, the planted fixtures, the end-to-end scripts that
measure the gates, and the kill-gate decision, so that nothing depends on which unit a reader expects to carry
them. See (d) and (e) for what the lead should link or merge.

## (a) Subsections and the beads that carry them

| Subsection | Title | Content | Beads |
|---|---|---|---|
| §26 preamble | Crate `frob-nemesis`, layer 5 | The crate, its module map, `CONTRACT.md`, the proof-boundary dependency rule | N26a-01 |
| §26.1 | What Nemesis owns | What randomization cannot do; what Nemesis owns and consumes; the airlock crates | N26a-01 |
| §26.1 | | The rule that a derived value is a known answer | N26a-01, N26-KA |
| §26.1 | | The tier table (Solid and Frontier mechanisms with reasons) | N26a-01; pilots N26a-06, N26a-28 |
| §26.2 | Decisions | ADR-NEMESIS-01 to -07, each with reason and rejected alternative | N26a-01 (all); -01 BD-270, N26a-03; -02 N26a-13, N26a-14; -03 BD-272; -04 N26a-17, N26a-18; -05 N26a-23; -06 N26a-01, BD-273, N26a-20; -07 named in N26a-01 (mechanism in U26b's §26.10) |
| §26.3 | The search space and the legality set | A world: base scene, edit list, disturbance schedule, parameter vector; `SceneId` | N26a-02 |
| §26.3 | | The seven edit classes and the causal paths each declares; an edit that cannot say is not admitted (FROB-REF-362) | N26a-02, N26a-05 |
| §26.3 | | The legality profile `frob.legality/1`, provenance classes, the check on the final world (INV-NEMESIS-02), FROB-REF-360, -361 | BD-270, N26a-05 |
| §26.3 | | Hazard registries as the scene-edit vocabulary (seed title) | N26a-02 (vocabulary), BD-270 (default profiles) |
| §26.3 | | Admissibility and prior; the density under the nominal randomization | N26a-03 |
| §26.3 | | Small: the lexicographic size of a world | N26a-03 |
| §26.3 | | Planted illegal edits (100) with legal twins | N26a-04 |
| §26.4 | The falsification score and the search | The score: the requirement margin, the boundary check, candidate failures | BD-273 |
| §26.4 | | The design-pair question for rivals A and B | N26a-10 |
| §26.4 | | Methods table: card-uniform baseline arm and fallback | N26a-07 |
| §26.4 | | Cross-entropy search; derivative-free descent | N26a-08 |
| §26.4 | | Quality-diversity archive; grammar mutation | N26a-09 |
| §26.4 | | Splitting on forks as a search method | BD-277 (mechanism), BD-273 (portfolio share) |
| §26.4 | | The scenario-language sampler as a second baseline arm | N26a-07 (`given` arm), N26a-11 |
| §26.4 | | Value-of-information ordering that never decides a verdict | N26a-10 |
| §26.4 | | Every evaluated world recorded with address and proposing law; search on a fast tier; nothing counts before oracle and guard | BD-273, N26a-01, N26a-07 |
| §26.4 | | The ceiling stated before any speed test | BD-273, N26a-06, N26a-11 |
| §26.4 | | Verb `nemesis.search`; GATE-NEMESIS-02; KILL-NEMESIS | BD-273, N26a-11, N26a-12 |
| §26.5 | The solvability oracle | The four classes and what each counts as | N26a-13, N26a-14, N26a-33; seed BD-271 (U26b) |
| §26.5 | | Necessary conditions (route in the clearance complex, wrench and torque envelopes, energy budget) with dual vector or bottleneck; infeasible columns; retrofit hand-off | N26a-13 |
| §26.5 | | What the task forces (the mandatory task events the conditions are posed for), read from the verdict formula; added at integration, since no Arena bead exports it | N26a-36 |
| §26.5 | | Witness sources (baseline at other gains, another family member, sampling controller; privileged trajectory) | N26a-14 |
| §26.5 | | Three rules: a witness passes the guard; a failed search proves nothing; the candidate's body, not a point robot | N26a-14, N26a-13, N26a-33 |
| §26.5 | | Planted impossible and solvable traps; the oracle rows of GATE-NEMESIS-01 | N26a-04, N26a-33 |
| §26.6 | The artifact guard | One tier up (decision-level equivalence); four numerical variants; the energy audit (INV-NEMESIS-05) | BD-272 |
| §26.6 | | The label `artifact-candidate`, exclusion, the fidelity-ledger record | BD-272, N26a-16 |
| §26.6 | | The two planted artifacts (box-cone traction; energy-pumping penetration recovery) and genuine failures | N26a-15 |
| §26.6 | | GATE-NEMESIS-06 measured from the user's side | N26a-16 |
| §26.7 | Shrinking to a minimal counterexample world | Input: a failing world whose base passes; the base as counterexample | N26a-17; seed BD-274 (U26b) |
| §26.7 | | The test relation (legal, deterministic replay, same atom and class, 48 of 64 paired seeds; a different mechanism is a separate finding) | N26a-17 |
| §26.7 | | The algorithm: delta debugging, 1-minimality, bisection of continuous edits | N26a-18 |
| §26.7 | | What minimal means: the order-dependence counterexample; the pair check (78 tests at most); "minimum among pairs" | N26a-18, N26a-34 |
| §26.7 | | Size TARGET of three edits; `non-minimal(k)` | N26a-18, N26a-34, N26a-35 |
| §26.7 | | The file: FMT-FMCW, `.fmcw`, `frob.mcw/1`; what replays it (INV-NEMESIS-04); passing neighbours as contrasts; retrofit input | N26a-19 |
| §26.7 | | The loop (propose, search, classify, guard, shrink, update the hard-world set and the handled region, edit, repeat) | N26a-20 |
| §26.7 | | GATE-NEMESIS-03 measured; KILL-MCW decided, with its fallback specified | N26a-34, N26a-35 |
| §26.8 | The hard-world set | Schema `frob.hardset/1`, `nemesis.hardset`, admission after referee promotion, retirement by decision, the three refused uses | BD-275 and N15a-28 (U15a) |
| §26.8 | | The guard's label reaching the set ("none in the hard-world set") | BD-272, N26a-16 |
| §26.8 | | Refusal as nominal draws for a bound (FROB-REF-560) | N26a-23, N26a-30 |
| §26.8 | | Refusal as a confirmation sample (FROB-REF-567) from the search side; the set updated inside the loop | BD-273, N26a-20 |
| §26.9 | Rare failures: exploration and certification | Two questions; the estimator table; proof class `Tested`, never `Certified` | BD-276, N26a-21 |
| §26.9 | | Crude sampling: zero-failure rule, the cost table, the qualification-level count, extreme-value extrapolation not built | N26a-21 |
| §26.9 | | Crude sampling, sequential (the sibling's Bernoulli mixture, cap 10⁶) and our exact bound beyond the cap | N26a-22 |
| §26.9 | | Bounded-weight importance sampling with the defensive mixture; what the sibling provides | BD-276 |
| §26.9 | | What we add: monotone transforms to card distributions; the mixture for discrete edits | N26a-24 |
| §26.9 | | What we add: the variance-adaptive betting sequence; the floor ln(1/δ)/(ε·n); refutation and estimates | N26a-25, BD-276 |
| §26.9 | | Exploration: cross-entropy tilting | N26a-08 |
| §26.9 | | Exploration: splitting, child addresses, re-keyed streams, FROB-REF-569, `Estimated` rates; the TARGET and the evidence both ways | BD-277 |
| §26.9 | | The distress score with fixed levels | N26a-29 |
| §26.9 | | The catalogue of modes and the Good–Turing monitor | N26a-27 |
| §26.9 | | The decision model (policy table, finalists only, obligation, primary risk, fallback, budgeted mode, baseline, rollback, verification) | N26a-26 |
| §26.9 | | The countermeasure to the primary risk: episodes carry their proposal law (INV-NEMESIS-03); FROB-REF-560, -563, -568 | N26a-23, N26a-01 |
| §26.9 | | Worked numbers (6 × 10⁴ fragments, 7% coefficient of variation, 21 hours) | BD-277, N26a-21, N26a-26 |
| §26.9 | | Scope of every bound | N26a-21, BD-276 |
| §26.9 | | Verb `nemesis.bound`; GATE-NEMESIS-04, -05; KILL-RARE; the exploration pilot | BD-276, N26a-30, BD-277, N26a-31, N26a-32, N26a-28 |

## (b) Gates, kill gates and admission gates

| Gate | Milestone | Beads that measure or decide it |
|---|---|---|
| GATE-NEMESIS-01 | M3 | Legality row: BD-270 (record), N26a-05 (script). Oracle rows: N26a-33 (script and combined record), with components N26a-13, N26a-14 and plants N26a-04; the seed bead BD-271 (U26b) also lists the gate |
| GATE-NEMESIS-02 | M3 | BD-273 (record), N26a-11 (registered method experiment and script) |
| GATE-NEMESIS-03 | M3 | N26a-34 (script and record, with the planted-cause registry); the seed bead BD-274 (U26b) also lists the gate |
| GATE-NEMESIS-04 | M3 | BD-276 (record), N26a-30 (script); row 3 by N26a-23 |
| GATE-NEMESIS-05 | M3 | BD-277 (record), N26a-31 (registered method experiment and script) |
| GATE-NEMESIS-06 | M3 | BD-272 (crate-level gate test), N26a-16 (script and record); BD-275 (U15a) lists the hard-world-set clause |
| GATE-NEMESIS-07 | M4 | Expected from U26b (§26.10) and the seed bead BD-345 (U28b). Not carried here |
| GATE-NEMESIS-08 | M3 | Expected from U26b (§26.11, seed bead BD-278). Not carried here |
| KILL-NEMESIS | M3 | N26a-12 (decision), BD-273 and N26a-11 (measurements), N26a-07 (the fallback pipeline) |
| KILL-RARE | M3 | N26a-32 (decision), N26a-28 (the pilot the trigger names), BD-277 and N26a-31 (product-scale measurements) |
| KILL-MCW | M3 | N26a-35 (decision and the fallback's specification), N26a-34 (measurements); the seed bead BD-274 (U26b) also lists it |

The chapter defines no admission gate. Gates of other chapters touched: GATE-GAUNT-08 (pilots before funding;
N26a-06, N26a-28 write the records), GATE-GAUNT-09 (method-experiment registration; N26a-11, N26a-31),
GATE-ATEL-13 (the decision-core table of DC-04; N26a-26 supplies the policy record, the seed bead BD-349
registers it), GATE-AUGUR-13 (design-pair separation; N26a-10 supplies the search, BD-293 measures).

## (c) Counts

- Bead files: 41. Seed beads: 5. New beads: 36 (N26a-01 to N26a-35 and N26-KA).
- Subsections: the preamble and 9 subsections (§26.1 to §26.9), all in table (a).
- Gates carried: 9 of the 11 listed in the brief (GATE-NEMESIS-01 to -06; KILL-NEMESIS, KILL-RARE, KILL-MCW);
  2 expected from U26b (GATE-NEMESIS-07, -08).
- Known answers of §26 (Appendix M, KA-659 to KA-669, 11 rows): all 11 in the registry N26-KA; 9 carried with
  their values in mechanism and script beads (KA-659 to KA-667); KA-668 and KA-669 belong to U26b's subsections.
  Known answers of the two constructions cited with values: 7 (KA-138 to KA-144).
- Counterexamples of §26 (Appendix N, CX-288 to CX-295, 8 rows): 7 carried with their case (CX-288, -289, -290,
  -291, -292, -294, -295); CX-293 (screening) belongs to U26b. Others cited: CX-065, CX-066, CX-067.
- Refusal codes of §26 (12): 10 carried with meaning, trigger, ranked repairs and exit code (360, 361, 362, 560,
  563, 564, 567, 568, 569, 760); 565 and 566 belong to U26b's subsections. Code of another chapter used: 840.
- Invariants carried in test plans: INV-NEMESIS-01 (N26a-13, N26a-14, N26a-19, N26a-33), -02 (BD-270), -03
  (N26a-23), -04 (N26a-19, N26a-34), -05 (BD-272, N26a-14), -06 (N26a-01, BD-273, N26a-20). Proof obligations:
  PO-NEMESIS-01 (N26a-18), -02 (N26a-24, BD-276), -04 (N26a-21); PO-NEMESIS-03 belongs to U26b.
- Architecture decisions stated with reasons: ADR-NEMESIS-01 to -07.
- Open questions placed: OPEN-NEMESIS-1 (BD-270), -2 (N26a-29, BD-277, N26a-31), -3 (N26a-21, N26a-26, BD-276),
  -4 (N26a-22, BD-276, BD-277); OPEN-NEMESIS-5 is in BD-275 (U15a). Each carries its fail-closed default; the
  decision beads for them, if any, are U26b's (§26.16).
- Pilots: 2 (N26a-06, N26a-28). Kill-gate decisions: 3 (N26a-12, N26a-32, N26a-35). End-to-end scripts: 8
  (N26a-05, -11, -16, -20, -30, -31, -33, -34). Planted-fixture registries: 2 beads (N26a-04, N26a-15) plus the
  planted causes inside N26a-34.

## (d) Needs from other slices

Structure the lead should apply:

- BD-271 (U26b) should take N26a-13 and N26a-14 as dependencies, and BD-274 (U26b) should take N26a-17, N26a-18
  and N26a-19, if U26b did not split out the same pieces. If U26b wrote its own pilot beads, kill-gate decision
  beads or gate scripts for GATE-NEMESIS-01 to -06, KILL-NEMESIS, KILL-RARE or KILL-MCW, they duplicate
  N26a-06, -28, -12, -32, -35, -05, -11, -16, -30, -31, -33, -34 and should be merged.
- BD-275 (U15a) is complete for §26.8; nothing was added there.

Prerequisites with no known key (each is a `needs` entry on the bead named):

- Habitat (§19): the typed scene edit of `frob-scene` and its application to a scene pack **at M3** (the priced
  retrofit catalog that also uses it is a later milestone); per-aspect digests of a compiled scene; the minimum
  passage width of a scene and scene acceptance on an edited scene; per-feature draw probabilities of the
  interior generators; two interior generators at M3 (N26a-02, BD-270, N26a-03, N26a-24, N26a-04).
- Airlock (§33.3): the `optimize` feature group admitted (traits over `fs-dfo`, `fs-archive`, `fs-eproc`), and
  the traits `Uncertainty`, `Optimizer`, `Archive`, `ValueOfInformation` exposed to layer 5 (N26a-01, N26a-08,
  N26a-09, N26a-10, N26a-22, BD-276).
- Kinesis and contact (§16.9, §16.10, §18.6): model-form variant step plans (half step, twice the iterations,
  the second contact law, tolerance divided by ten); the box friction cone and the position-level penetration
  correction as labelled non-production modes (BD-272, N26a-15). Per-world scene and parameter variants in one
  batch (N26a-17). A robot on the CPU referee or a pinned development oracle for the two pilots (N26a-06,
  N26a-28).
- Arena (§24): mandatory task events from a requirement; per-atom minima in verdicts; the online margin as a
  device signal; a synthetic rate task; the quadruped fixture with per-step pushes (N26a-13, N26a-09, N26a-29,
  N26a-30, N26a-31).
- Chronicle and Autopsy (§27.3, §29.8 to §29.10): the replicate mode of a fork; a snapshot at a device-side
  predicate; `sim.replay` on a world identity or a `.fmcw`; the energy ledger's monitors for a finished run;
  stable failure classes (BD-277, N26a-29, N26a-19, N26a-34, BD-272, N26a-17, N26a-27).
- Reflex and Cortex (§22, §23.11, §15.3, §23.15): baseline controllers with a declared gain family and the
  causal-policy harness; addressable policy-family members; distribution families of randomized card fields
  (N26a-14, N26a-04, N26a-24).
- Atelier and Augur (§28.4, §28.11, §28.13, §28.19.1, §25.9, §25.30): the task-wrench cone program in static
  form at M3; the finalist record of a campaign; `result.revalidate`; the decision-core contract; the
  design-pair question's margin and admissibility contract; the sign-change detector for sweeps (N26a-13,
  N26a-26, BD-276, N26a-20, N26a-10, BD-273).
- Tether (§32.7, §32.12): idempotent commitment before dispatch; registration of `nemesis_search` and
  `nemesis_bound` in the MCP cluster `attack` (N26a-23, BD-273, BD-276).
- Gauntlet and performance (§38.4, §38.5, §36.6, §39.6, §16.13): the method-experiment registration record and
  its gate; the pilot register; the paired lower-bound routine; the phase ledger's step accounting;
  `sim.estimate` (N26a-11, N26a-31, N26a-06, N26a-28, BD-273, N26a-07, N26a-21).
- Lemma (§12.3): a theorem-card number for PO-NEMESIS-04, which the plan calls a candidate card (N26a-21).
- Workspace (§34.9, §35.9): the format-registry entry for FMT-FMCW and the canonical writer; registered random
  streams for certification draws and the per-step stream registry of a task (N26a-19, N26a-23, BD-277).
- U26b: the tests that carry KA-668 and KA-669, the registries `N26-CX` and `N26-REF`, and any decision beads
  for OPEN-NEMESIS-1 to -4 (N26-KA and the Considerations named in (c)).

## (e) Contradictions and things that could not be turned into work as written

1. **Three formulas for the zero-failure count.** §26.9 and the worked card use the exact ⌈ln δ/ln(1 − p⋆)⌉
   (29,956; 299,572; 2,995,731); the construction card and the decision core write ⌈ln(1/δ)/p⋆⌉ (29,958 for the
   first); the known answer's negative control, "ln(1/δ)/p⋆ rounded down", is 29,957, which is one more than
   the exact count, not less. Done: the exact formula governs and is the known answer; the approximations are
   documented as sufficient and never returned (N26a-21, N26-KA).
2. **The `.fmcw` example's closer torque.** §26.7 prints 14 → 21 N·m with `legality.inside: true` in a
   commercial kitchen, whose hazard row bounds a swing door at 5 to 20 N·m; the worked-artifact appendix prints
   19 N·m. Done: examples use 19 N·m; 21 N·m against [5, 20] N·m is the unit test of a violation (BD-270,
   N26a-19).
3. **Causal-path tags.** The class table, the retrofit catalog (`clocks`, `actors`) and the file examples
   (`fixture`, `payload`) use different words. Done: one closed set of eight tags with a mapping (N26a-02).
4. **"A profile with the bound removed admits it"** against "a bound for every editable quantity". Done: a path
   with no bound is not editable (fail closed); the negative control widens the bound to the physical domain
   (BD-270, N26a-05).
5. **The reference of `log10_density_ratio`** is not stated. Done: the ratio is taken against the base world's
   own nominal draw, factor by factor, with an explicit `undefined_for` list (N26a-03). An owner decision could
   change the reference without changing the field.
6. **`non-minimal(k)`** is defined as "a shrunk world with more than three edits", which can still be 1-minimal,
   while the type sketch offers only `OneMinimal` or `NonMinimal(k)`. Done: a companion field
   `one_minimal_verified` (N26a-18); the on-wire `kind` values are unchanged.
7. **A failure that replays but is seed-fragile** (fewer than 48 of 64 paired seeds on the full edit set) has no
   stated treatment. Done: returned unshrunk as `non-minimal(n)` with the reason (N26a-17).
8. **Exit code of FROB-REF-760.** The exit table lists `70x` under exit 4 and `72x`, `73x` under exit 5; 760 is
   in neither. Done: exit 4 (the budget family), flagged for the refusal registry (N26a-21, N26a-26).
9. **No refusal code for a non-finalist bound request** ("a screening rung never buys a rare-failure bound").
   Done: refusal with exit 3; a free code is proposed to the registry owner (N26a-26).
10. **Two pilot falsifiers for exploration.** The construction card says "not found at least three times sooner
    than by uniform sampling, or fewer than 1% of forks reach the next level"; §26.15 and the gate register say
    10 times at 10⁻⁵, coverage below 85%, a mode missed in more than 20%. Done: the register governs; both are
    recorded (N26a-28).
11. **KILL-RARE is worded on the pilot** while the seed bead for splitting is done only when the gate is decided
    at M3 after GATE-NEMESIS-05. Done: the decision reads the product-scale values where they exist and the
    pilot's otherwise, and prints both (N26a-32).
12. **GATE-NEMESIS-04's coverage row** ("at least 95% of 10⁴ studies") is a coin toss for an exactly calibrated
    bound. Done: the study plan uses (n, p) pairs whose exact coverage is about 0.96, computed by enumeration,
    and says so (BD-276, N26a-30).
13. **GATE-NEMESIS-05 counts episodes**, and splitting runs fragments. Done: episode-equivalents by simulated
    steps, with fork fragments, level pilots, oracle and guard runs charged to the arm (N26a-31).
14. **"A planning recomputation … near 6 (Appendix C)"**: the recomputation was not found in the appendix it
    cites. Done: carried as a stated planning figure, labelled, in the ceiling text (BD-273, N26a-06, N26a-11).
15. **Subset simulation** is an arm of the exploration pilot in §26.15 and is "deferred unless forks carry no
    fresh randomness" in the list of rejected methods. Done: a pilot arm only; not planned for the product
    (N26a-28).
16. **Surfaces added so that gates can be measured from the user's side** (DESIGN, not in the plan's verb
    notes): `nemesis.search --method given --worlds <file>` (also the carrier of the scenario-language arm),
    `--first`, and `nemesis.bound --episodes <run set>`. They are needed to offer planted worlds and to exercise
    FROB-REF-560; flagged for the verb-registry owner (BD-273, N26a-07, BD-276).
17. **"The region of worlds the design is known to handle"** has no schema. Done: a record
    `frob.nemesis.handled/1` whose `certificate` is the constant false and whose wording is fixed (N26a-20).
18. **Planted traps "of known nominal probability"**: the plan does not say how a physical trap gets a known
    probability. Done: trigger boxes in normal germs with analytic probability and a calibration protocol; the
    probability is recorded as an interval whose upper end is analytic (N26a-04).
