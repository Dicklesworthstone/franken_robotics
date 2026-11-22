# Coverage of drafting unit U27b

Chapter §27 Autopsy (counterfactual debugging): subsections §27.11 to §27.19. Three seed beads (BD-279, BD-280,
BD-284), thirty new beads (N27b-01 to N27b-30) and the chapter's counterexample and refusal registries (N27-CX,
N27-REF). The known-answer registry of the chapter (`N27-KA`) is written by U27a; the beads here cite known-answer
numbers with their values inline and name the registry under `related`.

Two of this unit's seed beads draw on subsections U27a owns: BD-279 (`autopsy.intervene`) carries §27.5 in full
and the Autopsy-side parts of §27.3 and §27.4; BD-280 (`autopsy.attribute`) carries §27.6 and §27.9 in full. Both
were written completely, as the brief asks, with their components split into new beads. Conversely §27.11 (the
decision model) is this unit's subsection while its seed bead BD-282 is U27a's; this unit wrote the policy
mechanics as N27b-12 so that nothing depends on which unit a reader expects to carry them. See (d) and (e) for
what the lead should link or merge.

## (a) Subsections and the beads that carry them

| Subsection | Title | Content | Beads |
|---|---|---|---|
| §27.11 | The decision model for attribution | States, actions, loss; the decision card (decision, artifacts, obligations, failure modes) | N27b-12 |
| §27.11 | | Fallback (first-anomaly ordering, `Estimated`) and its three triggers; delivery of the fallback | N27b-12; refusals in N27b-03 (583), N27b-07 (584) |
| §27.11 | | Budgeted mode: 12 factors, singles and pairs, 8 replicates, 64 placebo forks, 689 forks; Shapley summary only if no single and no pair changes the outcome | N27b-12 (stage table), BD-279 (fork plan and counts); coalition stage in BD-281 (U27a) |
| §27.11 | | Baseline (heuristic ordering; one-at-a-time contrasts: do ten forks answer as well as seven hundred) | N27b-12 (baselines, shadow), N27b-08 (arms), N27b-09 (agreement recorded), N27b-13 (decision) |
| §27.11 | | Rollback flag; recorded forks remain valid | N27b-12 |
| §27.11 | | Verification list (planted single and joint causes; planted chaotic failure; thermal counterexample) | N27b-09, N27b-12; BD-281, BD-282, BD-283 (U27a) |
| §27.12 | Time-travel debugging as an agent workflow | Five moves, one verb each, bounded outputs | BD-284, N27b-18 |
| §27.12 | | Transcript `frob.autopsy.transcript/1` that replays every fork from its address | BD-284 |
| §27.12 | | The worked session (explain, `run fork`, intervene with a refused gear ratio, attribute, rerun, `eval compare`) and the attribution object | BD-284 (shape), BD-280 (record), BD-279 (intervene, refusal 581), N27b-18 (script) |
| §27.12 | | "The heuristic proposes, forks test, design parameters go back to the start, and a study decides" | BD-284, N27b-18 |
| §27.12 | | The viewer overlays a fork on its parent as a ghost, synchronized on the fork tick | N27b-17 |
| §27.13 | What Autopsy does not claim | The eight non-claims, each with an enforcement and a test | N27b-15 |
| §27.13 | | Not a minimal repair; not a passing design | N27b-14, N27b-15 |
| §27.13 | | Not a proof: contrast `Tested` on its tier, attribution `Estimated` | BD-280, N27b-15, N27-CX (CX-301) |
| §27.13 | | Not a differentiator of the engine (the composed-incumbent arm hosts the same method) | N27b-15 (contract and documentation), BD-280 (Considerations) |
| §27.14 | Verbs and refusals | `autopsy.intervene` | BD-279, N27b-04 |
| §27.14 | | `autopsy.attribute` | BD-280, N27b-09 |
| §27.14 | | `autopsy.repair` | N27b-14, N27b-18 |
| §27.14 | | `autopsy.rerun` | Seed bead BD-283 (U27a); registry entry N27b-16; refusal 587 row N27-REF; sweep N27b-19 |
| §27.14 | | `autopsy.show` | BD-284, N27b-17 (`--format ghost`) |
| §27.14 | | Registry entries, kinds, milestones, task clusters; no fork verb and no divergence verb | N27b-16 |
| §27.14 | | The eight refusal codes | N27-REF (rows), N27b-19 (end-to-end sweep); raised in BD-279, N27b-02, N27b-03, N27b-07, N27b-15, BD-280, BD-283 (U27a), BD-205 (U27a) |
| §27.15 | Invariants and proof obligations | INV-AUTOPSY-01 to -06 as an artifact auditor with mutants | N27b-20 |
| §27.15 | | INV-AUTOPSY-01 | N27b-03, BD-279, N27b-04, N27b-20 |
| §27.15 | | INV-AUTOPSY-02 | N27b-02, N27b-20 |
| §27.15 | | INV-AUTOPSY-03 | BD-279 (refusal 581, labelled diagnostic mode), N27b-20 |
| §27.15 | | INV-AUTOPSY-04 | BD-279, N27b-04, N27b-20 |
| §27.15 | | INV-AUTOPSY-05 | N27b-05, N27b-06, N27b-20 |
| §27.15 | | INV-AUTOPSY-06 | N27b-15, BD-280, N27b-20 |
| §27.15 | | PO-AUTOPSY-01 (Shapley laws, exact rationals) | N27b-20 (independent cross-check); mechanism BD-281 (U27a) |
| §27.15 | | PO-AUTOPSY-02 (exact-mode first divergence; the monotonicity derivation in `docs/derivations/`) | N27b-20 (derivation file and cross-check); mechanism BD-206 (U29a) |
| §27.15 | | PO-AUTOPSY-03 (tolerance mode, outward-rounded block bounds) | N27b-20 (cross-check); mechanism BD-206 (U29a) |
| §27.16 | Known answers | The ten rows with fixtures, derivations, bound tests and negative controls | N27b-21 (suite wiring and host fixtures); registry of record `N27-KA` (U27a) |
| §27.16 | | Planted single causes (KA-670) | N27b-08, BD-280, N27b-09 |
| §27.16 | | Null fork (KA-671); chaotic placebo (KA-672) | N27b-03, BD-279, N27b-04 |
| §27.16 | | Joint and alternative pairs (KA-673); negative Shapley value (KA-674) | N27b-06, N27b-14, N27b-20; BD-281 (U27a) |
| §27.16 | | Thermal re-run (KA-675); warm-start graft (KA-676) | N27b-21 (thermal fixture and derivation), BD-279 (graft, forced fork); BD-283, BD-205 (U27a) |
| §27.16 | | First divergence (KA-677); tolerance mode with a gap (KA-678) | N27b-20; BD-206 (U29a) |
| §27.16 | | Precision (KA-679) | N27b-07, N27b-21 |
| §27.17 | Gates, kill gate and pilot | GATE-AUTOPSY-03 | BD-280 (crate-level), N27b-09 (script and record), N27b-28 (lane clause) |
| §27.17 | | KILL-AUTOPSY with its fallback | N27b-11 |
| §27.17 | | The pilot (paper scale; falsifier; ceiling stated in advance; "if GATE-AUTOPSY-08 fails the tools stay") | N27b-10; the M4 note in BD-284, N27b-11, N27b-30 |
| §27.17 | | The other gates of the table, read as a set at exit | N27b-30; N27b-04 (01 and the refusal clause of 02, re-read) |
| §27.18 | Open questions | OPEN-AUTOPSY-1, -2 (decision beads with defaults) | N27b-13; defaults in N27b-03, BD-279, N27b-05, N27b-12 |
| §27.18 | | OPEN-AUTOPSY-3 (Lyapunov exponents of our loops) | N27b-22 (pilot measurement), N27b-28 (G(W) on our tiers), N27b-07, N27b-23 (Considerations), N27b-13 (status) |
| §27.18 | | OPEN-AUTOPSY-4 (snapshot reservoir size) | BD-279, N27b-27 (Considerations), N27b-13 (status) |
| §27.19 | Attribution in single precision on the originating tier | Why it is admissible (growth arithmetic; the hypothesis about our exponents) | N27b-22, N27b-23 |
| §27.19 | | Procedure steps 1 to 3: null fork, R₀ placebo forks at 10⁻⁷, admission (ρ₀ ≤ 0.1; G(W)·u ≤ 10⁻⁴) | N27b-23 |
| §27.19 | | Step 3: attribution on the originating tier with at least 16 replicates at 10⁻⁷; `tier: <backend>, precision: f32` | N27b-24 |
| §27.19 | | Step 4: not admitted: the longest window that passes, the bound on how late a fork may start, escalation | N27b-23, N27b-24 |
| §27.19 | | Step 5: numerical-variant interventions first; `tier-specific: <variant>`; artifact guard; `precision-sensitive` | N27b-25, N27b-07 |
| §27.19 | | The cross-check that can fail (one in k = 8, and every reported attribution) | N27b-26 |
| §27.19 | | Known answers (linear map; pendulum, `marginal`) | N27b-21, N27b-23, N27b-03, N27-CX (`late-01`) |
| §27.19 | | Cost and ceiling (320 world-seconds; 7.0 × 10⁵; 3 minutes against 12 minutes to 2 hours) | N27b-27, N27b-22, N27b-24 |
| §27.19 | | Owner and verb: `autopsy.attribute --lane {f64, tier-f32, auto}` | N27b-24, N27b-16 |
| §27.19 | | GATE-AUTOPSY-09 | N27b-28 |
| §27.19 | | KILL-ENSEMBLE | N27b-29; pilot N27b-22 |

Subsections of U27a that this unit's seed beads carry, for the lead's cross-check:

| Subsection | What is carried here | Beads |
|---|---|---|
| §27.2 | ADR-AUTOPSY-01 (BD-280, N27b-05), -02 and -03 (BD-279), -04 (N27b-07, N27b-24, BD-280), -05 (BD-280), -06 (N27b-01, N27b-14), -07 (N27b-01, N27b-15, BD-280), each with its reason and a rejected alternative | as listed |
| §27.3 | Child identity and fork paths for the Autopsy series, randomness (same streams; a replicate changes one declared thing), batching K children, horizon | N27b-03, BD-279 |
| §27.4 | The use of the five classes at admission; state handling per class as executed; FROB-REF-580, -581, -582 as raised by the Autopsy verb | BD-279, N27-REF |
| §27.5 | Interventions, factors, one declared mechanism, the split-or-refuse rule; null, placebo and replicate forks | N27b-01, N27b-02, N27b-03, BD-279 |
| §27.6 | The three quantities, the test, the label table, the illustrative card | N27b-05, N27b-06, BD-280 |
| §27.7 | Minimal repair sets only | N27b-14 |
| §27.8 | The forced fork in diagnostic mode (`--diagnostic-force`); FROB-REF-587's row | BD-279, N27-REF |
| §27.9 | The double-precision default; the three-step procedure; reporting duties | N27b-07, BD-280 |
| §27.10 | Reading rules for a fork against its parent (in the ghost feed); the monotonicity derivation and the trap | N27b-17, N27b-20 |

## (b) Gates, kill gates and admission gates

| Gate | Milestone | Beads that measure or decide it |
|---|---|---|
| GATE-AUTOPSY-01 | M2 | Expected from U27a's seed bead BD-205 (the measurement of record). Here: N27b-04 re-reads it at M3 through `autopsy.intervene` (1,000 null forks; alone against a batch of 512); N27b-30 reviews the row |
| GATE-AUTOPSY-02 | M2 | Expected from BD-205 (the 40 catalogued pairs). Here: N27b-04 re-reads the refusal clause (warm-start graft; mid-episode gear change); N27b-30 reviews the row |
| GATE-AUTOPSY-03 | M3 | BD-280 (crate-level measurement), N27b-09 (script and record, double-precision clauses and one-at-a-time agreement), N27b-28 (the lane clause), N27b-30 (review) |
| GATE-AUTOPSY-04 | M3 | Expected from U27a's seed bead BD-281. Here: N27b-30 reviews the row; N27b-20 cross-checks the Shapley identities |
| GATE-AUTOPSY-05 | M3 | Expected from U27a's seed bead BD-282. Here: N27b-12 supplies the policy it measures; N27b-30 reviews the row |
| GATE-AUTOPSY-06 | M3 | Expected from U27a's seed bead BD-283. Here: N27b-21 supplies the thermal host fixture and derivation; N27b-30 reviews the row |
| GATE-AUTOPSY-07 | M2 | Expected from U29a's seed bead BD-206. Here: N27b-20 cross-checks exact and tolerance modes and writes the derivation; N27b-30 reviews the row |
| GATE-AUTOPSY-08 | M4 | Expected from U27a's seed bead BD-375. Not carried here; N27b-18 is its fixture and N27b-30 records the hand-over |
| GATE-AUTOPSY-09 | M3 | N27b-28 (script and record), with N27b-23 to N27b-27 as mechanisms; N27b-30 (review) |
| KILL-AUTOPSY | M3 | N27b-11 (decision and fallback specification), N27b-09 and N27b-10 (measurements), BD-280 (seed condition "decided") |
| KILL-ENSEMBLE | M3 (not in §40's tables; listed in §40.8 without a milestone) | N27b-29 (decision), N27b-28 and N27b-22 (measurements), N27b-26 (the agreement ledger and alarm) |

The chapter defines no admission gate. Gates of other chapters touched: GATE-GAUNT-08 (pilots before funding:
N27b-10 and N27b-22 write the records), GATE-GAUNT-09 (method-experiment registration: the speed experiment of
N27b-27), GATE-ATEL-13 (the decision-core table of DC-06 at M4: N27b-12 supplies the policy record, seed bead
BD-349 registers it), GATE-TETHER-16 (the replay packet, which consumes N27b-17), GATE-SPARK-15 (batch
independence, on which every fork batch rests).

## (c) Counts

- Bead files: 35. Seed beads: 3 (BD-279, BD-280, BD-284). New beads: 32 (N27b-01 to N27b-30, N27-CX, N27-REF).
- Subsections: 9 (§27.11 to §27.19), all in table (a); parts of 9 further subsections carried for the seed
  beads.
- Gates named in a `gates` field here: 10 of the 11 in the brief (GATE-AUTOPSY-01 to -07 and -09; KILL-AUTOPSY;
  KILL-ENSEMBLE). Measured or decided here: GATE-AUTOPSY-03, GATE-AUTOPSY-09, KILL-AUTOPSY, KILL-ENSEMBLE.
  Re-read or reviewed here and measured elsewhere: 01, 02, 04, 05, 06, 07. Expected from U27a only:
  GATE-AUTOPSY-08.
- Known answers of §27 (Appendix M, KA-670 to KA-679, 10 rows): all 10 cited with their truths, tolerances and
  controls (table in N27b-21); 4 realized by this unit's tests (KA-670, -671, -672, -679), 6 bound to U27a's and
  U29a's tests with independent cross-checks here (KA-673 to KA-678). Known answers of construction C-29 cited
  with values: 4 (KA-145 to KA-148). Lane answers without a number: 2 (provisional `lane-1`, `lane-2`).
- Counterexamples of §27 (Appendix N, CX-296 to CX-301, 6 rows): all 6 in N27-CX with controls and mutants;
  equivalents recorded: 3 (CX-069, CX-070, CX-071); others cited: CX-288, CX-321; late cases under provisional
  tags: 2.
- Refusal codes of §27 (8): all 8 in N27-REF with trigger, pointer, ranked repairs, retry and exit code, and
  raised end to end by N27b-19. Codes of other chapters used and bounded: 362, 553, 562, 701, 702, 720, 724,
  801, 802, 803, 805, 819, 820, 866, 894; tombstone 882; unassigned 585.
- Invariants: INV-AUTOPSY-01 to -06 (6), each in a mechanism bead's test plan and in the auditor of N27b-20.
  Proof obligations: PO-AUTOPSY-01 to -03 (3), cross-checked in N27b-20.
- Architecture decisions stated with reasons and rejected alternatives: ADR-AUTOPSY-01 to -07 (7).
- Open questions: OPEN-AUTOPSY-1 to -4 (4): two decided by N27b-13, two placed with owner and default.
- Bets, risks, cores, constructions, cards named: BET-23; RISK-50; DC-06; C-29, C-28, C-15; GB-34; MR-08.
- Pilots: 2 (N27b-10, N27b-22). Kill-gate decisions: 2 (N27b-11, N27b-29). Owner decisions: 1 (N27b-13).
  End-to-end scripts: 5 capability scripts (N27b-04, N27b-09, N27b-18, N27b-19, N27b-28) and the review's
  runner (N27b-30). Exit review: 1 (N27b-30).
- Largest bead: BD-284, 198 lines. Smallest: N27-CX and N27b-04, 126 lines each.

## (d) Needs from other slices

Prerequisites written as `needs` because the owning unit's key was not known, grouped by owner.

- **U27a (§27.1 to §27.10).** The `frob-autopsy` crate skeleton with `CONTRACT.md` and the shared types (§27.1);
  the coalition value table, pair interactions and the Shapley stage as callable inputs (§27.7, BD-281); the
  episode-start result as an input type (§27.8, BD-283); the registry `N27-KA`. Requests to the lead: make
  BD-281 depend on the pilot N27b-10 (the pilot runs "before the coalition machinery is funded"); link BD-282
  and BD-349 to N27b-12 (policy record for DC-06); make BD-375 use the fixture of N27b-18; let `run.fork`
  (BD-205) print `double_precision` from N27b-07's route from M3.
- **U29a (Chronicle).** Entry points and block bounds of the first-divergence search for tests (BD-206); a
  comparison replay of a single-precision checkpoint under the double-precision profile; forks and placebo
  batches on single-precision tiers; reconstruction of fork points by replay for many worlds; pose telemetry for
  fork children; the citation operation of the artifact graph with a hook for per-producer rules (BD-095); a
  ledger for per-stratum agreement figures.
- **U26a, U26b (Nemesis).** The seven edit classes as typed values with declared causal paths; the
  delta-debugging routine with its pair check as a generic library function; the planted solver artifacts of the
  artifact guard on single-precision tiers.
- **U16 (Kinesis).** The fidelity ledger's envelope for "time of first failure" per stratum; the four
  model-form variant step plans as distinct identities on single-precision tiers.
- **U35 (numerics).** Registration of the stream purpose `autopsy.perturb`; the class-D2 record per device
  family that licenses forks there.
- **U32a, U37 (Tether, boundary).** Session capture of verb calls; token-budgeted summaries; pagination; task
  clusters and their lint (a proposed `repair` cluster for `autopsy_repair` and `autopsy_rerun`); the generator
  from the verb registry; estimates and budgets for fork batches; capability scope names.
- **U28 (Atelier).** The decision-core contract and table format; the report generator's statement model and
  the admissibility hook; the corpus entries for planted single causes with the committed label-file hash
  (BD-248).
- **U38, U39 (Gauntlet, ten-fold).** The pilot register and its record schema; the development oracle and its
  pin; registration of method experiments; the negative-control registry and catalogue; the incumbent phase
  ledger for the share of `diagnose` turns.
- **U36 (performance).** Measured world-seconds per second of the fork workload on `batch-cpu` and on a device;
  the disclosure block.
- **U23 (Cortex).** The trained quadruped policy and a humanoid policy on a device tier; a policy artifact that
  declares the body it was trained for (for FROB-REF-587).
- **U19 (Habitat).** The generated kitchen with a film patch on the route.
- **U21 (Spark).** Per-world variants in one device batch for children of different parents.
- **U33a, U24a (viewer, stills).** The counterfactual-ghost overlay and the replay packet writer that consume
  N27b-17.
- **Catalogue owners.** Appendix N numbers for the two late counterexamples; Appendix M numbers for the two lane
  known answers.

## (e) Contradictions, gaps and what was done

1. **Seed condition of BD-279: "a placebo fork equals replay on 1,000 worlds".** A placebo fork is perturbed by
   10⁻¹² and cannot equal replay after the fork tick. Read as three assertions (N27b-04, BD-279): the null fork
   equals replay; a placebo child is bit-identical to replay before its perturbation and differs after it in
   exactly one coordinate; a zero-size placebo equals the null fork.
2. **`frob autopsy fork` in the M3 milestone text (§43.6)** against §27.14 and Appendix J ("Autopsy registers no
   fork verb"). The registry governs: no such verb; N27b-16 pins it and returns the unknown-verb refusal with
   `run.fork` as the repair.
3. **`--lane` is specified on `autopsy.attribute`, a read verb** (§27.19), but forks are run by
   `autopsy.intervene`. Both verbs take `--lane`: intervene decides where forks run; attribute selects which
   fork set to read (N27b-24).
4. **Illustrative numbers that cannot arise from one replicate set** (§27.12, Appendix L): rescued 3/8 with a
   10⁻¹² flip rate of 0.125; rescued 7/8 with a 10⁻¹² flip rate of 0.0 labelled `alternative`; a 10⁻⁷ flip rate
   of 0.0 for an intervention that rescued nothing (no second series runs for it). The plan says its numbers
   are illustrative and fix only the shape. Design adopted: replicate 0 of the 10⁻¹² series is the unperturbed
   fork (the fork budget 689 = 78·8 + 64 + 1 leaves no room for a separate one), so flips equal the rescue count
   or R minus it; a series that did not run prints `null`; tests use consistent count tables (N27b-03, N27b-06,
   BD-280).
5. **"Perturbed by a relative 10⁻¹²" does not say relative to what, nor with what size distribution.** Relative
   to the coordinate's own value, the pendulum answer of §27.19 fails (5 × 10⁻⁸ rad × 10⁻⁷ cannot flip a sign);
   with one fixed size, the doubling map's amplified shift is 0.0995 mod 1 and only about 20% of forks flip,
   not "about half". Adopted: relative to the physical scale of the coordinate's class, with a size factor drawn
   on [1, 2) and a drawn sign, which satisfies both known answers (N27b-03; registered as `late-02` in N27-CX).
6. **v(∅) is not defined in §27.7.** The pair interactions printed in §27.12 and Appendix L (−0.34, −0.84) imply
   v(∅) = ρ₀; adopted (N27b-05).
7. **KA-146 against KA-673.** The construction's row calls the either-alone case two `but-for` causes; the
   chapter's label table calls each `alternative`. The chapter's finer table governs; the test asserts both
   facts (N27b-06).
8. **FROB-REF-562 (§35) and FROB-REF-584 (§27) overlap** on determinism class. Reading fixed in N27-REF: 562 is
   the licence check on the source run at admission; 584's class clause concerns a fork set whose own children
   are of uncontrolled class.
9. **The type sketch's `CauseLabel` lacks `marginal` and `tier-specific`** (the lane is later than the sketch),
   and its note about "three classes in §29.8" is stale (§29.8 lists five). Extended and used five (N27b-06).
10. **"The stricter step profile" (§27.19) against "a solver tolerance ten times tighter" (§26.6)** for the
    fourth numerical variant. Treated as one variant, tag `tol-div-10` (N27b-25).
11. **GATE-AUTOPSY-09 names two claims** (Known answer; Design comparison), against the rule that a gate names
    one. Recorded as Known answer with the speed clause measured under the method-experiment form (N27b-28).
12. **KILL-ENSEMBLE has no milestone and is ambiguous in two clauses.** Decided at M3 with its gate. "Fewer than
    half … admitted at any window of 0.5 s or more" is read as "no window of at least 0.5 s admits at least
    half" (the plan's own arithmetic expects 2 s windows not to pass); "a bulk speedup below 3" is read on the
    paired lower bound. Both readings are pre-registered (N27b-28, N27b-29).
13. **The lane clause of GATE-AUTOPSY-03** ("90% of 200 failures … in the single-precision lane") does not say
    whether unadmitted failures count. Pass is read over admitted cases; the figure over all 200 is recorded
    beside it (N27b-28).
14. **"The top factor changes in at most 30% of cases under 10⁻⁷ perturbation"** has no protocol. Product gate:
    ranking recomputed from the 10⁻⁷ series; pilot: 20 perturbed snapshots, rate over (failure, perturbation)
    pairs; both named in their reports (N27b-06, N27b-08, N27b-10).
15. **FROB-REF-880 has no surface on which an attribution is "cited".** Written as one citation rule with three
    call sites (reports, evidence inputs, gate rows and manifests); the command-line spelling belongs to the
    artifact-graph and report owners (N27b-15).
16. **The test is unnamed and its level unstated** ("an exact one-sided test … or an e-process … at a stated
    level"). Adopted: Fisher's exact test, α = 1/20 divided by the factors run, a fixed-bet e-process with
    λ = ½, and a reachability table enumerated before sampling (N27b-05). At 8 replicates `but-for` then needs
    all eight rescued.
17. **"The bound the first-divergence algorithm gives on how late a fork may start"** is not defined. Adopted
    W_min = failure time minus the earlier of the first decision-relevant divergence against the
    double-precision replay and the first-anomaly time (N27b-23).
18. **The transcript's replay command is not named.** Adopted `autopsy.show --format script` and `--compare`,
    keeping `show` a read verb (BD-284).
19. **`autopsy.repair` and `autopsy.rerun` sit in no task cluster of Appendix J.5**, though the lint requires
    every verb in one. A `repair` cluster is proposed (N27b-16).
20. **Autopsy is Solid but names a pilot, and the Frontier lane names none.** Both pilots written: N27b-10 as
    the chapter states it; N27b-22 with its falsifier taken from KILL-ENSEMBLE.
21. **Seed bead BD-284 names two verbs.** `autopsy.repair` was split into N27b-14; BD-284 keeps the transcript,
    `autopsy.show` and both seed conditions, and depends on N27b-14 and N27b-15.
22. **N27-REF is placed at M2 under WP-51**, not at the chapter's M3, because `run.fork` raises 580, 581 and 582
    at M2.
23. **Possible overlaps for the lead to merge:** N27b-12 with BD-282 (decision model); N27b-21's thermal host
    fixture with BD-283's tests; N27b-04's re-reading of GATE-AUTOPSY-01 and -02 with BD-205's gate tests;
    N27b-16 with the general M3 verb sweep BD-312.
