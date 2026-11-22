# Coverage of drafting unit U25e

Chapter §25 (Augur: mission queries and inverse design), subsections §25.29 to §25.39, plus three seed beads and
two chapter registries. 50 bead files: 3 seed beads, 45 new beads (`N25e-01` to `N25e-45`), 2 registry beads
(`N25-CX`, `N25-REF`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Tier | Beads |
|---|---|---|---|
| §25.29 | Chance claims transport only through a coupling | [F] | N25e-01 (pilot, 200 finite fixtures), N25e-02 (claim kinds, exact coupling budget, transported bounds), N25e-03 (bundle block and checker rules), N25e-04 (common-noise paired runs, tray fixture, M8), N25e-05 (e2e, finite part), N25e-06 (KILL-COUPLING); refusal in N25e-45 |
| §25.30 | No monotone fill without a proof of monotonicity | [S] | N25e-07 (no interpolation; licensed fill as a proof step), N25e-08 (detector, fidelity-ledger routing, `model-conditional`, fixed requirement clock), N25e-09 (banded fixture, creep rig, e2e); BD-302 prints the condition |
| §25.31 | Monomial screens in exponent coordinates | [S] | N25e-10 (screen and integer certificate), N25e-11 (sums of monomials, worklist, `mission.refute` family report), N25e-12 (generated corpus, mutants, e2e); checker is U12's N12-14 |
| §25.32 | Utility brackets and the dominance stop | [F] | N25e-13 (pilot on the solvable family), N25e-14 (upper and lower ends), N25e-15 (labels, α/(2M), the stop, `eval.compare --bracket`, `mission.refute --utility`, refusal), N25e-16 (e2e and the 3× comparison), N25e-17 (KILL-AUGUR-BRACKET) |
| §25.33 | Placement and sizing along a prescribed motion | [F] | N25e-18 (pilot: cart and trot), N25e-19 (regressor Y and identity), N25e-20 (cone program, three limits, scope rule, screening signal), N25e-21 (e2e and the 30-design comparison), N25e-22 (KILL-SYNTH-REGRESSOR) |
| §25.34 | Weak-variable enclosure and the split set | [F] | BD-358 (the base region mechanism of §25.6 that this extends), N25e-23 (pilot: 20-parameter case and control), N25e-24 (driver, split set, stop rule, `--split-cap`), N25e-25 (e2e and the 12-parameter comparison), N25e-26 (KILL-WEAKVAR), N25e-27 (OPEN-AUGUR-3 experiment with |aᵢ| per parameter) |
| §25.35 | The controller class is part of a family answer | [S] | N25e-28 (`policy_class.law`, lowering for four classes, `refute` under affine), N25e-29 (law object and checker rules), N25e-30 (e2e); refusal in N25e-45 |
| §25.36 | Docking to a finite tolerance over a continuous gain interval | [S] | N25e-31 (closed form, lowering, algebraic optimum, `attained`), N25e-32 (certificates, witness policy, 133,185 cases, eligibility refusal), N25e-33 (e2e) |
| §25.37 | Channel laws: the joint error law of repeated reads, and the code that carries a label | [S] | N25e-34 (`error_dependence`, credit rule, enumeration decoder), N25e-35 (label channel, least length, code, lower bound), N25e-36 (e2e, error-law rows, M3), N25e-37 (e2e, label rows, M4); BD-294 (best-matching seed); refusal in N25e-45 |
| §25.38 | Shared nuisance parameters, and the probe for four admitted laws | [F] | N25e-38 (scope, shared symbols, witness rule), N25e-39 (four-law probe, tie-break), N25e-40 (pilot re-run and pinned truth), N25e-41 (e2e; added trigger of KILL-AUGUR-PROBE); refusal in N25e-45 |
| §25.39 | Sizing witnesses for fixed points, and Pareto coverage of a catalog | [S] | N25e-42 (bracket with F(U) ≤ U, `lower-bound-only`), N25e-43 (dispositions, `pareto-candidates`, constrained queries), N25e-44 (e2e); refusal in N25e-45 |

Seed beads written (key, milestone and seed dependencies kept):

| Seed | Title in the seed | Draws on | Notes |
|---|---|---|---|
| BD-294 | Checker independence repeated over every proof kind in use at M3 | §25.27 (GATE-AUGUR-06), §25.2, §25.5.4, §25.5.5, §25.25 | Dependencies added: BD-288, N25e-10, N25e-12, N25e-29, STD-E2E |
| BD-302 | `report.design`, `report.run`, `report.campaign`, `report.mission` | §28.18, §32.6, §29.13 (the seed was mapped to §25 by keyword) | Dependencies added: BD-139, BD-266, STD-E2E |
| BD-358 | Region certificates on small sub-boxes | §25.6 (owned by U25b), §25.34 | Dependencies added: BD-127, BD-288, BD-132, N25e-27 |

Registries: `N25-CX` (counterexamples of §25), `N25-REF` (the twenty refusal codes of §25). `N25-KA` is written by
U25a.

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone | Measured or decided by | Mechanism beads |
|---|---|---|---|
| GATE-AUGUR-06 (repeat at M3) | M3 | BD-294 | (M1 run: BD-136, U25b) |
| GATE-AUGUR-09 | M3 pilot, M4 | BD-358 (gate); N25e-27 (OPEN-AUGUR-3, which blocks it) | (pilot: BD-289, U25b) |
| GATE-AUGUR-25 | M4 finite part; M8 tray | N25e-05 (finite), N25e-04 (tray) | N25e-01, N25e-02, N25e-03 |
| GATE-AUGUR-26 | M3 | N25e-09 | N25e-07, N25e-08 |
| GATE-AUGUR-27 | M3 | N25e-12 | N25e-10, N25e-11 |
| GATE-AUGUR-28 | M4 | N25e-16 | N25e-13, N25e-14, N25e-15 |
| GATE-AUGUR-29 | M4 | N25e-21 | N25e-18, N25e-19, N25e-20 |
| GATE-AUGUR-30 | M4 | N25e-25 | N25e-23, N25e-24 |
| GATE-AUGUR-31 | M1 | N25e-30 | N25e-28, N25e-29 |
| GATE-AUGUR-32 | M1 | N25e-33 | N25e-31, N25e-32 |
| GATE-AUGUR-33 | M3 (error-law rows); M4 (label rows) | N25e-36; N25e-37 | N25e-34; N25e-35 |
| GATE-AUGUR-34 | M4 | N25e-41 | N25e-38, N25e-39, N25e-40 |
| GATE-AUGUR-35 | M4 | N25e-44 | N25e-42, N25e-43 |
| KILL-COUPLING | first reading M4, decision M8 | N25e-06 | — |
| KILL-AUGUR-BRACKET | M4 | N25e-17 | — |
| KILL-SYNTH-REGRESSOR | M4 | N25e-22 | — |
| KILL-WEAKVAR | M4 | N25e-26 | — |
| KILL-AUGUR-PROBE (added trigger only) | M3 gate, trigger measured at M4 | N25e-41 measures the added trigger; N25e-38 implements the rule | The decision bead belongs to the unit of §25.14 and §25.27 |

Gates of the chapter that this unit expects other units to carry (their mechanisms are in §25.1 to §25.28):
GATE-AUGUR-01 to -05, -07, -08, -10 to -24; GATE-AUGUR-06 at M1 (BD-136); KILL-AUGUR-ALIAS, -CUTS, -DOMINANCE,
-FACILITY, -FALSECERT, -LABELS, -PHASE, -PROBE (decision), -REGION, -REUSE, -SEPARATE, -SYMEVENT, -TIE, -TRANSPORT;
KILL-SYNTH-CODESIGN, -CONT, -LINKAGE, -TRAJOPT. This unit's beads name KILL-AUGUR-REGION (BD-358, N25e-27),
KILL-AUGUR-FALSECERT (BD-294, N25e-03, N25e-10, N25e-29) and KILL-AUGUR-LABELS (N25e-35, N25e-37) in their
Considerations without deciding them.

## (c) Counts

- Known answers. Appendix M has no rows for §25.29 to §25.39. This unit states 18 late known-answer groups in full
  in its beads (list below) and cites 15 numbered rows of §25 with their values (KA-624, -626, -628, -630, -631,
  -633, -635 to -640, -645, -647, -652), plus KA-683 of §28.
- Counterexamples: 18 numbered (CX-270 to CX-287) and 11 late unnumbered, all in `N25-CX`.
- Refusal codes: 20 in `N25-REF` (FROB-REF-846 to -859, -868, -874, -877, -881, -884, -887); 5 late refusals with
  no code, in `N25e-45` with an interim mapping.
- Other identifiers carried: ADR-AUGUR-04, -05, -06 (corrected reading), -07, -10; INV-AUGUR-03, -05, -06, -07,
  -08, -11; PO-AUGUR-01, -02, -03, -07; OPEN-AUGUR-3 (experiment bead N25e-27), OPEN-AUGUR-7 (N25-REF,
  Considerations); H-25, H-28; BET-19, BET-20; C-03, C-04, C-12, C-17, C-32, C-39, C-40; A-10, A-17, A-20, A-24;
  RISK-35, RISK-36, RISK-39.

Late known answers that need Appendix M numbers (each stated with values in the beads named):

| # | Subsection | Known answer | Beads |
|---|---|---|---|
| 1 | §25.29 | Two-outcome reversal: Σ ε = 0.98, transported bound 0.01, refused | N25e-01, -02, -05 |
| 2 | §25.29 | Ten-step chain, ε = 0.003 per step: 0.99 transports to at least 0.96 | N25e-01, -02, -05 |
| 3 | §25.30 | Banded sill mount: pass, fail, pass; licensed creep fill equals the exhaustive grid; unlicensed fill refused | N25e-09 |
| 4 | §25.31 | Leg: λ = (1, 1), 8829/8000; control 8829/10000 with witness (0.2 m, 10) | N25e-10, -12 |
| 5 | §25.31 | Two-term cases (0.6, 0.6) and (0.9, 0.15) | N25e-11, -12 |
| 6 | §25.32 | Carrier: 16.22 kg and 0.622; 15.29 kg and 0.529; 313.92 J against 318 J | N25e-13, -14, -16 |
| 7 | §25.33 | Cart: 35/654 m; 3.066 m/s²; infeasible at 4 m/s²; feasible with all wheels driven | N25e-18, -20, -21 |
| 8 | §25.33 | τ²/N² is not convex: 1/4 > 2/9 at the midpoint | N25e-18, -20 |
| 9 | §25.34 | 20 parameters: five slabs, 0.58, 0.40, 0.02 | N25e-23, -24, -25 |
| 10 | §25.34 | Negative control Σ xᵢ − 10: unresolved after one enclosure | N25e-23, -24, -25 |
| 11 | §25.35 | Stance under three classes: 3/14 m, 1/2 m, none; (160, 130, 70, 40) N; 175 N | N25e-28, -29, -30 |
| 12 | §25.36 | δ* = (√1745 − 41)/32 m; p* = (39 + √1745)/128; inclusion at 1/40, exclusion at 1/50; 133,185 cases | N25e-31, -32, -33 |
| 13 | §25.37 | Seven reads: 3807/4096, 3/4, 55/64 | N25e-34, -36 |
| 14 | §25.37 | Three labels: 2, 3, 5 bits with codes; four bits under one flip excluded | N25e-35, -37 |
| 15 | §25.38 | Margin table: 4.50, 4.72, −13.06 N shared; 0.28, −0.078, −15.08 N independent | N25e-38, -40, -41 |
| 16 | §25.38 | Four-law probe: r = 3 at 1/2 s, gap 0.1090; 900 of 900; none at 1/4 s or less; ratio 5/3 | N25e-39, -40, -41 |
| 17 | §25.39 | Pack: 100 kg; 99.5 kg after 527 steps with a 5 g increment; F(99.5) = 99.505 | N25e-42, -44 |
| 18 | §25.39 | Builds (0, 6), (4, 4), (6, 0): (4, 4) minimizes no weighted sum; budget four selects it | N25e-43, -44 |

All numbers above were recomputed while drafting (exact rationals where the plan gives rationals) and agree with
the plan.

## (d) Needs from other slices

Prerequisites owned by other units, as written in the `needs` fields (candidate keys in parentheses):

- Lemma (U12): the `monomial-product`, `monotone-transport`, `symmetry-transport`, `composition` checkers (N12-14);
  `taylor-box`, `conic-farkas` (N12-16); `complex-connectivity` (N12-15); `sample-coverage` (N12-13); the generic
  rules and the rule catalogue (N12-02, N12-03); the arithmetic oracle (N12-10). New rules this unit asks of
  `frob-cert`, which its own beads specify and which the Lemma unit may prefer to own: the `transport` block rules
  (N25e-03), the law-object rules of `lp-primal` (N25e-29; BD-132 already checks the `multilinear-vertex`
  specimen), a code-certificate rule for label channels (N25e-35), one value per scope instance (N25e-38), the
  catalog-coverage use of `partition` for Pareto claims (N25e-43).
- Schema owners: `policy_class.law`, the label query's `channel` and `bit_time`, nuisance-parameter `scope`, the
  answer-side claim `pareto-candidates` in `frob.mission/1` (BD-126); `error_dependence` and the declared-law
  table on sensor cards (BD-238).
- Verb registry (U32): rows or flags for `mission.transport` at M4 with `--budget paired-runs`, `--fill licensed`,
  `--screen monomial`, `--weights`, `eval.compare --bracket`, `mission.refute --utility`, `--strategy placement`,
  `--ratios`, `--split-cap`, `--enclosure`, `--reuse`, and three analysis verbs: `analyze.sweep-monotone`,
  `analyze.constraint-graph`, `analyze.regressor-identity` (BD-376 and the M1 and M3 verb beads).
- Refusal registry: five new codes (N25e-45 asks the registry owner).
- Other units of §25: the static core and tray fixture of `mission.transport` (BD-436, U25a); the worklist
  (BD-128, BD-357, U25b); the region pilot and its 12-parameter family (BD-289, U25b); the probe-design base, its
  case grid and the KILL-AUGUR-PROBE decision (BD-291, U25c or U25d); the label colouring (BD-363, U25c); the
  co-design lane (BD-360, U25d); the M1 mutation corpus and campaign (BD-136, U25b).
- Atelier (U28): the controller-budget curves of §28.25 and the reuse ledger of §28.26 (new beads there); the
  ladder (BD-329), screening audits (BD-345), forward co-design (BD-352), A-24 (BD-348).
- Kinesis, Arena, Reflex, Phylum, Nemesis: the model-form variant runner (BD-266), the fidelity ledger (BD-212),
  the creep rigs of the cart-sill fixture (BD-123), the gait atlas and skills as a motion library (BD-332,
  BD-336), sweep results in point-status form (BD-184, BD-273).
- Spark: a `frob-kir` pass that extracts coefficients with respect to a declared linear input (N25e-19).
- Gauntlet: Appendix M numbers for the 18 late known answers; Appendix N numbers for the 11 late counterexamples;
  a held-out brief limited by energy or torque with adjudication (BD-314, BD-317).

## (e) Contradictions and things that could not be turned into work as written

1. Five refusals of §25.29, §25.35, §25.37, §25.38 and §25.39 are stated in words with no code, and the 8xx block
   is nearly full. Done: decision bead N25e-45 with fixed row content and a fail-closed interim mapping to
   existing codes with a `detail` discriminator.
2. §25.24 says the mission codes use the block 840 to 859; 840 to 845 belong to §24.14 and six mission codes lie
   outside. Done: N25-REF follows Appendix O and records the discrepancy.
3. Appendix M and Appendix N cite FROB-REF-826 for the pusher prefix mismatch where §25.24 defines FROB-REF-881
   for the same situation. Done: N25e-45 and N25-REF record a default mapping (the auditor reports 826; mission
   verbs present 881 citing 826).
4. Appendix M and Appendix N have no rows for §25.29 to §25.39. Done: values are in the beads; the lists are in
   (c) above and in N25-CX; numbering is requested from the catalogue's owner.
5. §45.12 lists KILL-COUPLING beside §24.22, whose own kill gate is KILL-EXPODOMAIN (§40.8 says so). Done:
   N25e-06 treats KILL-COUPLING as §25.29's only and notes the index error.
6. §25.29 names no registry row for the coupling budget, and the registry of proof kinds is closed. Done: N25e-03
   carries the budget as a typed `transport` block with its own rule area and no new kind; flagged as an open
   point for the Lemma lead.
7. KILL-COUPLING, KILL-AUGUR-BRACKET, KILL-SYNTH-REGRESSOR and KILL-WEAKVAR have no row in §40.4; §40.8 says a
   kill gate with no milestone is decided at the milestone of the gate beside it. Done: decisions placed at M8
   (with an M4 reading), M4, M4 and M4.
8. §25.33 names its pilot and gives no numeric falsifier for it, and §25.32 names no separate pilot. Done:
   N25e-18 states a falsifier derived from the kill gate and labels it DESIGN; N25e-13 makes the solvable
   family of the gate's first half the paper-scale pilot, with "one false stop" as falsifier.
9. §25.34's known answer (five slabs with shares exactly 0.58, 0.40, 0.02) cannot be produced by midpoint
   bisection alone, and its weak width of 0.01 needs a dependency-aware square in the enclosure. Done: N25e-24
   adds a threshold cut (a one-dimensional contractor, sound because each piece is re-enclosed) and states the
   square-rule requirement; both are labelled DESIGN and tested. The rule for choosing S (shortest leading
   prefix whose complement is at most a quarter of its own width, capped at six) is also this unit's DESIGN.
10. §25.36's exclusion at δ = 1/50 is labelled kind `alias-pair` with a `partition`, but two of its three pieces
    (p ≤ 0 and p > 63/100) fail by single-world actuator reach, not by aliasing. Done: N25e-32 composes an
    alias-pair piece and two reach sub-certificates under one partition and says so. The plan's count of 95
    probes at δ = 1/50 has no stated grid; N25e-32 uses p = k/128, k = 1 to 95, labelled DESIGN (verified: no
    probe passes).
11. GATE-AUGUR-33 has parts at M3 and M4. Done: two end-to-end beads (N25e-36, N25e-37).
12. BD-302 was mapped to this chapter by keyword; its content is §28.18 and §32.6. Done: written completely
    from those sections; `report.campaign` is M4 in the verb registry while the seed is M3, so the bead delivers
    the M3 form and names BD-352 for the M4 content. The Atelier unit may also carry §28.18; the lead should
    merge if so.
13. OPEN-AUGUR-3 is listed in §25.28 (another unit's subsection) and amended by §25.34. Done: experiment bead
    N25e-27 here, because it blocks BD-358; the lead should merge it with any decision bead the unit of §25.28
    writes.
14. N25e-29 overlaps BD-132 (U12), which already checks the `multilinear-vertex` law of the stance specimen.
    Done: the overlap is stated in N25e-29's Considerations.
15. Bead count is 50, above the typical 20 to 45: eleven late subsections, each with its own gate, five Frontier
    mechanisms needing pilots, and four kill gates each need their own beads under the charter.
