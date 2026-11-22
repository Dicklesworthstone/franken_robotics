# Coverage of drafting unit U27a

Chapter §27 Autopsy: counterfactual debugging, subsections §27.1 to §27.10 and the chapter preamble.
Seed beads: BD-205, BD-281, BD-282, BD-283, BD-375. Registry: N27-KA. New beads: N27a-01 to N27a-19.
The other unit of the chapter, U27b, owns §27.11 to §27.19, the seed beads BD-279, BD-280 and BD-284, and the
registries N27-CX and N27-REF.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §27 preamble | Crate `frob-autopsy`, layer 5 | N27a-10 (crate, layering, contract), N27a-01 (the shared vocabulary in `frob-core` that the layering needs) |
| §27.1 | What Autopsy owns | N27a-10 (ownership, what is consumed, the four engine needs as a precondition check, the Solid-except-one-claim statement, FACTs); BD-375 and N27a-19 (the one HYPOTHESIS, GATE-AUTOPSY-08); N27a-13 (the reading of first divergences on forks); N27-KA (the rule that derived values are known answers) |
| §27.2 | Decisions | ADR-AUTOPSY-01: BD-282 (stated in full; governs abstention), BD-283 (the placebo re-run), and BD-280 of U27b for fork contrasts. ADR-AUTOPSY-02: N27a-02, BD-205. ADR-AUTOPSY-03: BD-283, N27a-16. ADR-AUTOPSY-04: BD-205 (stated in full; the null fork and the double-precision decision check at the fork point), and BD-280 of U27b (precision routing). ADR-AUTOPSY-05: BD-281, N27a-11. ADR-AUTOPSY-06: N27a-15. ADR-AUTOPSY-07: N27a-10. Each is stated with its reason and the rejected alternative in the Background of the bead named first |
| §27.3 | Forks from snapshots [S] | BD-205 (mechanism, indexed copy, batching, the verb), N27a-03 (where forks start, the ring as an optimization, memory arithmetic, lead ladder, horizon), N27a-04 (identity, prefix by reference, INV-AUTOPSY-04, stream modes, replicate modes), N27a-01 (types), N27a-06 (null forks and batch independence measured) |
| §27.4 | Classifying a fork [S] | N27a-02 (the five classes, rule table, earliest valid tick, forcing fields, FROB-REF-580, -581, -582), N27a-05 (state handling per class), N27a-07 (the 40 catalogued pairs, the two named refusals, the planted change sets), BD-205 (the verb returns the classification), N27a-08 (script) |
| §27.5 | Interventions and the placebo [S] | Seed bead BD-279 of U27b carries the vocabulary `do(factor := value)`, the one-mechanism rule and FROB-REF-586, the null, placebo and replicate forks, ρ₀ and the 10⁻⁷ series. From this unit: N27a-01 (the `Intervention` and `Perturbation` types), N27a-04 (a replicate changes one declared thing: a state perturbation or one stream key), N27a-05 (how a placebo and an information intervention are restored), N27a-06 (the null fork, INV-AUTOPSY-01, measured at M2), N27a-14 (planted chaotic cases for the placebo), N27a-15 (diagnostic interventions typed apart from repairs) |
| §27.6 | Interventional contrasts and the flip rate [S] | Seed bead BD-280 of U27b carries the three quantities, the exact test and e-process, the level divided by the number of factors, the label table and the worked numbers. From this unit: BD-281 (the test and the rescue criterion applied to coalitions; `joint` and `alternative` from the minimal rescuing coalitions), BD-282 (`chaotic: unattributable` and no-cause abstention, INV-AUTOPSY-05 over the pipeline), BD-283 (the episode-start application that `precedes-snapshot` reads; labels for re-runs), N27a-12 and N27a-14 (planted cases for the labels), N27a-18 (script) |
| §27.7 | Coalitions and budgeted Shapley summaries [S] | BD-281 (v(S), the budget of 12 factors, 78 coalitions and 689 forks, pair interactions, the Shapley trigger, GATE-AUTOPSY-04), N27a-11 (the summary: exact for m ≤ 5, sampled otherwise, PO-AUTOPSY-01, what it cannot say, the sibling estimator not used), N27a-12 (planted joint and alternative cases, host tables), N27a-18 (script). Minimal repair sets: BD-281 supplies the minimal rescuing coalitions; the shrinking search and the verb `autopsy.repair` are built with seed bead BD-284 of U27b |
| §27.8 | Design parameters re-run from the episode start [S] | BD-283 (`autopsy.rerun`, the four steps, pairing, the report, GATE-AUTOPSY-06), N27a-15 (controller eligibility, FROB-REF-587, the diagnostic forced fork), N27a-16 (the thermal counterexample as a fixture, with its derivation), N27a-17 (script and gate record) |
| §27.9 | Precision for attribution [S] | Seed bead BD-280 of U27b carries the precision routing, FROB-REF-584, `precision-sensitive` and the stability report (flip rates and separation growth). From this unit: BD-205 (step 1 and step 2 of the procedure at the fork point: null fork on the originating backend, double-precision replay with the decision-level comparison), N27a-03 (the published exponents and the lead bound; reconstruction on the originating profile), BD-282 (the failed-equivalence fallback trigger, forced and checked), N27-KA (the growth arithmetic as derived values, KA-679) |
| §27.10 | The first-divergence algorithm [S] | N27a-09 (the algorithm restated as the contract under test, the trap, the reading rules, PO-AUTOPSY-02 and -03, GATE-AUTOPSY-07), N27a-13 (the reading on forks: class order, first decision-level divergence, topology rule, `acts_by`). The implementation and the verb `telemetry.divergence` are seed bead BD-206 of unit U29a |

## (b) Gates and kill gates

| Gate | Milestone | Beads of this unit that measure or decide it |
|---|---|---|
| GATE-AUTOPSY-01 | M2 | BD-205 (seed done-condition), N27a-06 (measurement and record) |
| GATE-AUTOPSY-02 | M2 | BD-205 (seed done-condition), N27a-02 (the mechanism measured), N27a-07 (catalogue, measurement and record), N27a-08 (from the user's side) |
| GATE-AUTOPSY-03 | M3 | None here. Expected from U27b: seed bead BD-280 and its script |
| GATE-AUTOPSY-04 | M3 | BD-281 (seed done-condition and record), N27a-12 (planted cases), N27a-18 (script) |
| GATE-AUTOPSY-05 | M3 | BD-282 (seed done-condition and record), N27a-14 (planted cases), N27a-18 (script) |
| GATE-AUTOPSY-06 | M3 | BD-283 (seed done-condition), N27a-16 (fixture), N27a-17 (script and record) |
| GATE-AUTOPSY-07 | M2 | N27a-09 (battery, script and record). Also in the done-condition of seed bead BD-206 of U29a |
| GATE-AUTOPSY-08 | M4 | BD-375 (run, verdict and record), N27a-19 (protocol and pre-registration) |
| GATE-AUTOPSY-09 | M3 | None here. Expected from U27b (the single-precision lane, §27.19) |
| KILL-AUTOPSY | M3 | None here. Expected from U27b (decision bead; seed bead BD-280). BD-281 is on the branch the kill removes and states so in `needs`; BD-283 is the fallback and says so |
| KILL-ENSEMBLE | M3 | None here. Expected from U27b (§27.19) |
| GATE-CHRON-05 (defined in §29.20; in the done-condition of BD-205) | M2 | BD-205, N27a-02, N27a-07, N27a-08; its null-replay clause is cited from N27a-06 |

No admission gate is defined in §27.1 to §27.10.

## (c) Counts

- Known answers: registry N27-KA carries 10 numbered rows of the chapter (KA-670 to KA-679), 8 numbered rows
  stated in other chapters that bind the same tests (KA-145 to KA-148, KA-716 to KA-718, part of KA-802), the 2
  unnumbered answers of §27.19, and 10 derived values with their formulas. Two rows that the plan lists with no
  stated control (KA-674, KA-676) are given registered controls.
- Counterexamples: the registry N27-CX is U27b's. This unit's mechanism beads carry by value 5 of the chapter's 6
  (CX-296, CX-297, CX-298, CX-299, CX-300; CX-301 belongs to §27.13) and 8 from other chapters (CX-066, CX-069,
  CX-070, CX-071, CX-226, CX-292, CX-320, CX-321).
- Refusal codes: the registry N27-REF is U27b's. This unit's beads state trigger, returned fields, ranked
  repairs and exit code for 4 of the chapter's 8 (FROB-REF-580, -581, -582 in N27a-02 and BD-205; FROB-REF-587
  in N27a-15 and BD-283), check the delivery of 2 more (FROB-REF-583, -584 in BD-282 and BD-205), and use 3 codes
  of other chapters (FROB-REF-562, -894, -840).
- Identifiers carried. Invariants: INV-AUTOPSY-01 (N27a-06, BD-205), -02 (N27a-01 type; enforced by BD-279 of
  U27b), -03 (N27a-02, N27a-15, BD-283), -04 (N27a-04), -05 (BD-282, property over the pipeline), -06 (N27a-10).
  Proof obligations: PO-AUTOPSY-01 (N27a-11), -02 and -03 (N27a-09). Decisions: ADR-AUTOPSY-01 to -07 as in table
  (a); ADR-CHRON-06 (N27a-02, BD-205). Open questions: OPEN-AUTOPSY-2 (BD-281, N27a-11), -3 and -4 (N27a-03);
  OPEN-AUTOPSY-1 is noted in N27a-10's defaults table; none blocks work, and the decision beads belong to §27.18
  (U27b). Risk RISK-50: BD-282, N27a-10. Bet BET-23: BD-205, BD-281, BD-282, BD-375, N27a-19. Construction C-29:
  cited throughout. Decision core DC-06: BD-282, BD-281. Scenario NS-16: BD-283, BD-375, N27a-10.

## (d) Needs from other slices

Stated in the `needs` fields; gathered here by owner.

- U27b (same chapter): the host-fixture trait and the exact doubling-map and one-node thermal host systems
  (§27.5); the fork runner behind `autopsy.intervene` (§27.5); the exact one-sided test, the rescue criterion and
  the label table (§27.6); the precision routing (§27.9); the attribution decision policy as a stage machine with
  rollback flag and evidence records (§27.11); the paper-scale pilot and the KILL-AUTOPSY decision (§27.17); the
  forced-fork flag on the intervention verb; the verbs `autopsy.intervene` and `autopsy.attribute` reachable
  through `frob`. All of these exist as beads of U27b (the parts of BD-279 and BD-280); the lead should resolve
  the `needs` to those keys.
- §29 (Chronicle): T1 event channels for first-dependence ticks (§29.5); stored block envelopes and samples
  (§29.5); the address-keyed reservoir for rings (§29.6); the chunk store's one-hash-per-key rule (§29.4); the
  class scale table and the divergence search callable from layer 5 (§29.8).
- §35 (numerics): the determinism licence check FROB-REF-562 (§35.3); registration of stream purposes
  (`autopsy.shapley`, the re-run sample) (§35.9).
- §32 (Tether): resident environments with free slots (§32.11); cost estimates and budget admission (§32.9);
  capability attenuation per session (§32.13); transcript capture (§32.23).
- §24 (Arena): the sealed-run check FROB-REF-840 and sealed acceptance clusters (§24.13); `eval.compare` with a
  declared controller arm (§24.12).
- §16 and §20: a first-tick step plan with a doubled iteration count as a distinct identity (§16.6); rebuilding a
  packet queue and filter state from a stated sensor law (§20.4).
- §22 and §23: the sampling controller with an arbitrary planning model (§22.8); the policy manifest
  `frob.policy/1` and a family policy's class-membership test (§23.7).
- §34: an ADR ranking `frob-nemesis` before `frob-autopsy` in layer 5 (§34.3).
- §38 and §40: the known-answer, negative-control, planted-defect, pilot and method-experiment registers
  (§38.4, §38.5, §38.6, §38.11); gate records readable per backend and kernel version (§40); the shared
  batch-independence fixture (§40.7).
- §36 and §39: approval of the compute envelope for the agent campaigns (§36.14); phase tagging of agent turns
  and the campaign ledger (§39.6).

## (e) Contradictions, gaps and what was done

1. **Two units, one mechanism.** The brief assigns §27.5, §27.6 and §27.9 to this unit, and assigns the seed beads
   that implement them (BD-279, BD-280) to U27b, which has decomposed them into mechanism beads (vocabulary, model
   difference, perturbations, statistics, labels, precision routing). Writing a second set here would have
   produced two specifications of one mechanism. This unit therefore wrote only the parts those seeds do not
   contain (the fork-side types and restore handling, the coalition, abstention, re-run and divergence work) and
   maps the three subsections to the seed keys in table (a). The lead should confirm that every sentence of
   §27.5, §27.6 and §27.9 is in U27b's beads. Points checked while drafting: both units read "8 replicates" with
   replicate 0 as the unperturbed fork (so that 689 holds), take v(∅) = ρ₀, use Fisher's exact one-sided test with
   α = 1/20 as the stated level, and use the same e-process.
2. **The replicate count against the unperturbed fork.** §27.6 measures a flip rate against "the unperturbed fork
   of the same intervention" while §27.7 and §27.11 count 78 × 8 + 64 + 1 = 689 forks with no extra fork per
   coalition. Resolved as above: replicate 0 is unperturbed. The alternative (a ninth fork, 767 in all) breaks a
   stated derived value.
3. **Stale note in the type appendix.** The note under sketch I.12 says §27.4 lists five classes and §29.8 three.
   Both sections and the sketch list five. Five are implemented (N27a-01).
4. **A fork verb that does not exist.** §43.6 lists `frob autopsy fork` among M3 artifacts; §27.14 and the verb
   registry say Autopsy registers no fork verb and that `run.fork` (M2) is the only one. The latter is followed.
5. **Where the classifier lives.** §27.4 owns the five classes; §29.1 and work package WP-51 place the mechanism
   and both M2 gates in `frob-chronicle` (layer 6); `frob-autopsy` is layer 5 and cannot name a layer-6 crate.
   Resolved: vocabulary and service trait in `frob-core` (N27a-01), classifier in `frob-chronicle` (N27a-02),
   specification and catalogue from the Autopsy chapter (N27a-07).
6. **Layer rank.** The crate map lists `frob-autopsy` before `frob-nemesis` in layer 5, and rule L-1 allows
   same-layer dependencies only on earlier rows, yet §27.1 has Autopsy consume Nemesis's typed world edits and
   shrinker. An ADR changing the rank is needed (the rule foresees one); N27a-10 states the fail-closed default
   until it exists.
7. **Streams of a child.** The glossary, GB-34 and MR-08 speak of forks with "fresh keyed streams"; §27.3 says a
   child reads its parent's streams by default; §26.9 calls re-keying "the replicate mode of §27.3". Resolved by
   explicit stream modes (N27a-04): `shared` by default, re-keying on request.
8. **A surface touched earlier.** §27.4 defines exogenous by "not yet touched" and contact-affecting by "in contact
   at the fork tick"; a surface touched earlier and not in contact now is neither. Classified as
   contact-affecting with the label `InterventionFromTick` and a reported `exogenous_until` (N27a-02; catalogue
   rows K5 and I8).
9. **FROB-REF-581 against FROB-REF-582.** The wheel-radius change at the lip is both a structural change at a
   mid-episode tick and a graft of state across models. Split used: 581 for the plain request, 582 for a request
   that carries or names state across models (N27a-02).
10. **FROB-REF-562 against FROB-REF-584.** §35.11 refuses a fork or attribution below class D2 with 562; §27.14's
    584 includes "under an uncontrolled determinism class". Used: 562 at the fork and re-run verbs (the licence
    check), 584 for attribution-time refusals, left to U27b.
11. **`--classify-only`.** §29.8 says it restores nothing; the worked session of Appendix L shows a classify-only
    call returning a fork handle and a null-replay result. Read as "restores no changed child"; the fork point
    is still opened and verified (BD-205).
12. **Label wording in KA-146.** The construction's row labels the "either alone" case `but-for`; §27.6 and KA-673
    label it `alternative`. The chapter governs (N27-KA records both).
13. **The 40 catalogued pairs are not listed in the plan.** N27a-07 supplies a DESIGN catalogue, eight per class,
    with expected values written from the rule table.
14. **"100 of 100" for two kinds.** GATE-AUTOPSY-05 states one count for chaotic and no-cause failures. Each kind
    is given 100 cases and held to 100 of 100 (N27a-14, BD-282), with 30 controls so that always abstaining fails.
15. **Planted chaos in simulation.** Physical sensitivity over a 2 s fork window is too weak to plant reliably
    (about 9 × 10⁶ at the published exponent). The simulated chaotic cases use a test-only amplifier atom; the
    host map and a referee knife-edge fixture are the other two forms (N27a-14).
16. **Labels for a re-run.** §27.8 says a re-run reports paired outcomes labelled `Estimated`; the worked session
    labels a 17-of-23 rescue `contributing`. BD-283 defines the rule: a placebo re-run supplies ρ₀, the exact test
    applies, and `but-for` needs replicates, so without them the strongest label is `contributing`, which matches
    the session.
17. **The thermal counterexample assumes constant resistance.** Its closed form needs heating ∝ 1/N² at a fixed
    resistance, while the actuator model that BD-283 depends on has temperature-dependent resistance. The fixture
    pins the coefficient to zero, says so in its card, and tests that the pin matters (N27a-16).
18. **Controller switched by a repair of FROB-REF-587.** The plan does not say what the original design runs under
    when the edited design gets a re-synthesized controller. N27a-15 requires the original to be re-run under the
    same controller kind, so the pair is not confounded.
19. **A dependency added to a seed.** BD-205's seed lists only BD-203. The double-precision decision check shown
    in the worked session needs the decision-level equivalence test (BD-177, M2); it and BD-169 were added.
20. **Ceiling of GATE-AUTOPSY-08.** The gate's threshold (one third of the turns) is confirmable only if diagnosis
    is at least two thirds of repair turns. N27a-19 measures that share first and states which verdicts are
    reachable; the plan already says the tools stay if the gate fails.
21. **Overlap noted, not resolved.** U27b also names GATE-AUTOPSY-01 and -02 on an M3 script through
    `autopsy.intervene`, and writes a known-answer suite bead for KA-670 to KA-679; this unit's N27a-06, N27a-07
    and N27-KA are the M2 measurements and the registry the brief assigns here. They are complementary; the
    lead may wish to add `related` links.
