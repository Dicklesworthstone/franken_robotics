# Coverage of drafting unit U28b

Chapter §28 Atelier, subsections §28.7 to §28.15, and six seed beads (BD-090, BD-248, BD-332, BD-345, BD-346,
BD-374). 48 bead files: 6 seed beads and 42 new beads (`N28b-01` to `N28b-42`).

Several seed beads whose best-matching subsection lies in this unit are written by other units (BD-113,
BD-343, BD-329, BD-310, BD-347, BD-349, BD-351, BD-352 by U28a; BD-341 by U41; BD-342, BD-353 by U17b; BD-328
by U23b; BD-265, BD-266 by U16b; BD-292 by U25a; BD-111, BD-220 by U33; BD-285 by U17a). The new beads of this
unit are the components, refusals, controls, tests, pilots and kill-gate decisions of those mechanisms, each
with an explicit scope line saying what the seed bead holds.

## (a) Subsections

| Subsection | Title | Content | Beads of this unit | Seed beads of other units that also carry it |
|---|---|---|---|---|
| §28.7 | Lineage: diff, blame, bisect, merge | Diff, blame | N28b-06 (end to end) | BD-113 |
| §28.7 | | An edit returns its invalidation set | N28b-01 | BD-058, BD-063 |
| §28.7 | | Bisect is honest about non-monotone lineages; bisect controls its own error; inconclusive probe | N28b-02, N28b-03, N28b-06 | BD-343 |
| §28.7 | | Merge is order-independent (INV-ATEL-09) | N28b-04 | BD-343, BD-342 |
| §28.7 | | Migration keeps old replay possible | N28b-05 | BD-059 |
| §28.8 | Optimization | Continuous parameters through the airlock | N28b-07 | BD-341 |
| §28.8 | | Racing and the race ledger | N28b-08 | BD-341 |
| §28.8 | | Quality-diversity archive | N28b-04 (join laws), N28b-08 (end to end) | BD-342 |
| §28.8 | | The fidelity ladder; co-design with control | N28b-12 | BD-329 |
| §28.8 | | A family per dispatch [F] (consumer of KILL-FDGRAD) | N28b-09 | BD-353 |
| §28.8 | | One verb for the whole campaign; one A/B, not two | BD-374, N28b-10, N28b-11, N28b-28 | BD-352, BD-372 |
| §28.9 | The ladder of evaluation rungs | The rung table; the sampling controller's rule; D2-only scores; the evidence about the middle rungs | N28b-12 | BD-329 |
| §28.9 | | Layer 1: the fixed table | N28b-15 (statement and use), N28b-17 (cost), N28b-19 (as baseline arm) | BD-329 |
| §28.9 | | Layer 2, step 1: the audit lane | N28b-13 | BD-329 |
| §28.9 | | Layer 2, step 2: calibration; obligations (coverage, Beta law, misordering) | N28b-14 | BD-329, BD-349 |
| §28.9 | | Layer 2, steps 3 and 4: the survivor set, the trust check, `budget-truncated` | N28b-15 | BD-329 |
| §28.9 | | Layer 2, step 5: the alarm | N28b-16 | BD-329 |
| §28.9 | | Layer 3: value of information at the finalist stage, one-way handover | N28b-15, N28b-21 | BD-351, BD-349 |
| §28.9 | | The numbers the evidence supports; the ceiling and the worked case | N28b-17 | — |
| §28.9 | | Pilot and falsifier for layers 2 and 3 | N28b-18, N28b-19, N28b-20, N28b-21 | BD-310 |
| §28.10 | The rank-agreement ledger | The row; two sources; published neighbours | N28b-22 | BD-328, BD-265 |
| §28.10 | | The three modes, thresholds, fail-closed default, FROB-REF-537 | N28b-23 | BD-328, BD-329 |
| §28.10 | | "The report says so"; "the program is told" (KILL-RANK) | N28b-23 | BD-328, BD-302 |
| §28.10 | | Bias is handled by region; the tier correction switches off | N28b-24 | BD-264 |
| §28.11 | The six-stage co-design cascade | The stage table; honest endings; `not-run` | N28b-25 | BD-352 |
| §28.11 | | Stage 1 and 2 controls; stage 1's counts | N28b-26 | BD-285, BD-286 |
| §28.11 | | Stage 3 control | BD-345, N28b-29 | — |
| §28.11 | | Stage 4 and 6 controls | N28b-27 | BD-341, BD-259, BD-260 |
| §28.11 | | Stage 5 variants and consequences | BD-346 | BD-266 |
| §28.11 | | End to end (GATE-ATEL-16) | N28b-28 | BD-352, BD-377 |
| §28.12 | Screening audits | The stratified audit of discards; the planted winner | N28b-29, BD-345, BD-248 | — |
| §28.12 | | Atelier does not trust a rung without a passed audit | BD-345 | — |
| §28.12 | | Prune soundness | N28b-30 | — |
| §28.12 | | End to end | N28b-31 | — |
| §28.13 | Revalidation after edits | The verb; the verdict record; four grades | N28b-32 | BD-292, BD-347 |
| §28.13 | | The assumption lattice with polarity; five rules; the hair-widening counterexample; the kept negative result | N28b-34 | BD-292, BD-347 |
| §28.13 | | Required reruns with their cost; the ceiling | N28b-33, N28b-32 | BD-347 |
| §28.13 | | Pilot with a hard stop for grade 3; KILL-REUSE | N28b-35, N28b-36 | BD-347 |
| §28.14 | The memo graph | Node kinds; the `Memo` trait; keys are derived | N28b-37 | BD-111, BD-063 |
| §28.14 | | Store policy (hits, shadow, loss model, budget, rollout, baseline) | N28b-38 | BD-111, BD-220, BD-209 |
| §28.14 | | Controller-synthesis and derived nodes; where the reuse is | N28b-39 | BD-220 |
| §28.14 | | PO-ATEL-07; targets; `fs-recompute` | N28b-40 (analysis part), N28b-42 (episode part), N28b-37 | BD-111, BD-220 |
| §28.15 | Campaign-level deduplication | Atelier's three rules for the digest | N28b-41, N28b-26 | BD-285 |

Seed beads of this unit that draw on other chapters: BD-090 (§29.3 to §29.5: ledger, `.frun`, levels,
streams, channel kinds), BD-248 (§24.9, §38.8: the corpus content), BD-332 (§10 C-10, §22.9: the gait atlas),
BD-374 (§4 BET-10, §32.6.2, §39.2 to §39.7: the surface A/B).

## (b) Gates, kill gates and admission gates

| Gate | Carried by (this unit) | Also carried by | Note |
|---|---|---|---|
| GATE-ATEL-02 | BD-332 (A-04 row and the M4 completion record) | The analysis beads of U28a and other units | |
| GATE-ATEL-03 | BD-374 (N28b-10 and N28b-11 are its prerequisites) | BD-372 reads the third arm | |
| GATE-ATEL-04 | N28b-03 (bisect row) | Comparison row: U28a (BD-262) | |
| GATE-ATEL-07 | N28b-23 (the consequence applied) | BD-328 (measurement), BD-310 (pilot) | |
| GATE-ATEL-08 | N28b-19 | BD-329, BD-310 (pilot row) | |
| GATE-ATEL-09 | BD-345, N28b-30, N28b-31 | — | |
| GATE-ATEL-10 | BD-346 | BD-266 measures GATE-FID-05 | |
| GATE-ATEL-11 | N28b-32 (M1 surface), N28b-35 (M4 run) | BD-292, N25a-13, N25a-17 (U25a); BD-347 | |
| GATE-ATEL-12 | N28b-40 (analysis part, M1), N28b-42 (episode part, M2) | BD-111, BD-220 | |
| GATE-ATEL-15 | N28b-18 (instrument and pilot), N28b-21 (M4 record) | BD-351 | |
| GATE-ATEL-16 | N28b-28 | BD-352, BD-342 | |
| GATE-NEMESIS-07 | N28b-29, BD-345, N28b-31 | Possibly a bead of U26b for §26.10 | Defined in §26.15 |
| KILL-LADDER | N28b-20 | BD-329 (seed line "decided") | |
| KILL-VOI | N28b-21 | BD-351 (seed line "decided") | |
| KILL-REUSE | N28b-36 | BD-347 (seed line "decided") | |
| GATE-ATEL-01, KILL-STATIC | — | Expected from U28a (§28.4; BD-250) | BD-248 supplies the corpus |
| GATE-ATEL-05, KILL-ATLAS | — | Expected from U28a (§28.4) | |
| GATE-ATEL-06 | — | Expected from U28a (§28.6) and the study beads | |
| GATE-ATEL-13, GATE-ATEL-14 | — | Expected from U28c and U28a (BD-309, BD-349, BD-350) | N28b-19 forces the DC-02 and DC-03 fallback triggers once |
| GATE-ATEL-17, -18, -19, KILL-BUDGETCURVE | — | Expected from U28c (§28.25, §28.26) | |
| KILL-RANK | — | BD-328, N16b-12 | Its switch is implemented in N28b-23 |
| KILL-FDGRAD | — | N17b-20 (owner §17.13) | N28b-09 is the consumer |

## (c) Counts

- Known answers cited with their values: 19. Of the nine §28 rows that belong to §28.7 to §28.15 or to this
  unit's seed beads, all nine are carried: KA-692, KA-696, KA-697, KA-698, KA-699, KA-700, KA-701, KA-702,
  KA-706. Others carried for seed beads and controls: KA-680, KA-682, KA-684, KA-687 (as prune and group
  controls), KA-072, KA-073, KA-353, KA-668, KA-715, KA-009.
- Counterexamples cited with their cases: 12. The five of §28.7 to §28.15 are all carried: CX-313, CX-314,
  CX-315, CX-316, CX-318. Others: CX-302, CX-303, CX-305, CX-306, CX-293, CX-034, CX-322.
- Refusal codes with meaning, trigger and repair: 21. Of the fifteen §28 codes, eleven have their trigger in
  this unit's subsections and are carried: 542, 545, 546, 548, 549, 550, 551, 557, 870, 871, 872 (the other
  four, 543, 553, 555, 556, belong to §28.6). From other chapters: 537, 540, 541, 566, 811, 854, 869, 878,
  888, 894.
- Invariants carried as property tests: INV-ATEL-01 to -09, -11, -12 (INV-ATEL-10 is §28.6's; it is asserted
  on probe and comparison results in N28b-02 and N28b-39). Proof obligations: PO-ATEL-03, -04 (per-campaign
  form), -06, -07, -08. Decisions stated with reasons: ADR-ATEL-01, -03 to -10; ADR-CHRON-01, -02, -03, -05,
  -09.
- Open questions placed in Considerations with their fail-closed defaults: OPEN-ATEL-1 to -6, OPEN-CHRON-1
  to -3. Their decision beads are expected from the unit that owns §28.24 and §29.21.

## (d) Needs from other slices

Recorded in `needs` on the beads; the main ones:

- **U28a.** BD-329 (the rung state machine and fixed table), BD-343 (the bisect verb), BD-347 (grades 3 and 4
  in production), BD-352 (the orchestrator), BD-351 (the ranking), BD-349 (compiled core tables), BD-310 (the
  logged tensor for the chooser replay). The analysis registry row for A-04.
- **U28c.** The registry rows of the §28 refusal codes (N28-REF), the counterexample registry (N28-CX), the
  decision beads of OPEN-ATEL-1 to -6, BD-350 (interference tests), the reuse ledger and controller-budget
  curves that read N28b-24 and N28b-38.
- **U24b.** N24b-02 (the corpus registry that BD-248 populates). **U25a.** N25a-13, N25a-16, N25a-17, N25a-19
  (mission-cell revalidation, mission polarity, the M3 frozen campaign and incumbent, KILL-AUGUR-REUSE).
  **U23b.** N23b-35 (the winner check, containment rank). **U16b.** N16b-13, N16b-15 (model-form variants and
  planted exploits). **U17b.** N17b-19, N17b-20 (the paired-gradient pilot and KILL-FDGRAD). **U10a.** N10a-17
  (the C-10 pilot). **U26b.** Reconciliation of §26.10 with N28b-29. **U38b.** BD-377 (the seeded cascade
  population with its plants and stored full evaluation; the M4 frozen campaign; the planted lineages).
  **U39, U32a.** The session recorder and phase ledger; `solve.run`; capability profiles.
- **Owner approvals.** Compute class E2 for the surface A/B and the paired gate campaigns; the agent model.

Suggested dependency edits in beads owned by other units (for the lead): BD-329 on N28b-12, N28b-13, N28b-14,
N28b-15, N28b-16, N28b-23; BD-343 on N28b-02, N28b-04; BD-341 on N28b-07; BD-347 on N28b-33, N28b-34,
N28b-36; BD-351 on N28b-21; BD-352 on N28b-25, N28b-26, N28b-27 (it already depends on BD-345 and BD-346);
BD-111 on N28b-37, N28b-38; BD-220 on N28b-38; BD-137 on N28b-32; BD-248 on N24b-02; BD-372 should read the
third arm of BD-374 and run no second experiment.

## (e) Contradictions and gaps, and what was done

1. **Bisect on a non-monotone lineage.** The seed line of BD-343 says "bisect on a non-monotone predicate is
   refused"; §28.7 says it returns a true `a-change` and only an unlicensed `first-change` is refused
   (FROB-REF-550). Done: N28b-02 follows §28.7.
2. **Per-probe level under a prefix scan.** §28.7 gives α/⌈log₂ L⌉; a scan makes up to L more probes. Done:
   α divided by the planned probe count, fixed before the first probe (DESIGN), which equals the plan's value
   without a scan.
3. **First milestone of `result.revalidate`.** Appendix J lists M4; §40 and §43 give GATE-ATEL-11 an M1 row
   on mission cells, and N25a-13 needs the registry entry at M1. Done: N28b-32 lands the surface at M1 in
   reduced form.
4. **One name per verb against the `analyze` family.** J.2 counts 26 `analyze` verbs, J.4 registers A-04 as
   `gait.atlas` and A-19 as `fidelity.modelform` and forbids aliases. Done: BD-332 and BD-346 add no second
   verb and flag the question for the registry freeze.
5. **BET-10's two wordings** ("at most half the restricted mean time" and "lower bound above 1.5"). Done:
   aim 2, gate on the lower bound above 1.5 (BD-374).
6. **"The design benchmark of §24" is named and not specified**; GATE-PHYLUM-08 calls it the benchmark of
   §39. Done: N28b-10 specifies one benchmark (briefs, acceptance, clock) under the program's estimand.
7. **Unmeasured stratum.** §38.8 says `discard-only`; §28.10 says `uninformative` and calls it the
   fail-closed default. Done: §28.10 (N28b-23).
8. **Three wordings of `model-robust`** (§16.9, §23.16, §28.11). Done: the cascade's strict form in BD-346,
   layered on BD-266's result; each finalist carries the ranking's type plus an `implicated` list (DESIGN
   reading of "each finalist typed").
9. **GATE-ATEL-15 is M4 while its replay is the M3 pilot.** Done: N28b-18 builds and runs it at M3; N28b-21
   records the gate at M4 on a log from our own engine.
10. **Per-campaign controls of stages 4 and 6.** §28.11 states the properties; their proofs are gate-level
    (512 replays; 2,000 campaigns). Done: N28b-27 designs per-campaign forms (label permutation within
    addresses; an inert-edit candidate), labelled DESIGN.
11. **BD-090 is mapped to §28.14 by keyword** and belongs to §29.3 to §29.5. Done: written from §29.
12. **§26.10 belongs to another unit** while the seed bead for its verb is here. Done: N28b-29 specifies the
    mechanism in full; to be reconciled with U26b.
13. **GATE-ATEL-12's M1 part.** BD-111's seed line names only the shadow clause; PO-ATEL-07 and the latency
    clause had no carrier. Done: N28b-40 (and N28b-42 for the episode part).
14. **Overlap with seed beads of other units** is deliberate and bounded by scope lines; where a seed bead
    turns out to hold the same content, the component bead here should become its child or be merged by the
    lead.
