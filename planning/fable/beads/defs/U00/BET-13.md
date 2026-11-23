---
key: "BET-13"
title: "Decide BET-13: ten-fold on the sealed suite against the strongest composed rival, on at least one named stratum"
type: "task"
priority: 1
milestone: "M7"
labels: ["m7", "tenfold", "frob-conformance", "solid", "bet", "gate", "owner-decision"]
parent: "WP-77"
deps: ["N00-28", "N00-19", "N00-20", "BD-314", "BD-317", "STD-E2E"]
needs: ["§39.7 and §39.20: the decision rule on the pooled ratio of restricted mean times, its censoring rule, multiplicity control, power analysis and the pre-analysis known answers (GATE-TENFOLD-10)", "§39.10: the outcome rules written in advance, per range of the ratio", "§39.8: the ceiling computed before the test (GATE-TENFOLD-09)"]
related: ["BET-14", "BET-17", "BD-143", "BD-428", "BD-430", "BD-431", "NS-24", "N00-17", "N00-18", "BD-429", "BD-432", "BD-433"]
gates: ["GATE-TENFOLD-06", "GATE-TENFOLD-07", "KILL-TENFOLD-SPEED"]
plan_refs: ["§4 (BET-13)", "§40.5", "§2.1", "§2.6", "§39", "§43.6", "§43.10", "§47.1"]
---
## Background

**The bet.** The project's headline ambition, stated as something that can be lost: on at least one named
stratum of a sealed mission suite, the restricted mean time to an independently accepted answer is ten times
shorter than the strongest composed rival's. It is one of the three bets on which the thesis turns.

**Terms.** An *answer* is a design and controller passing a frozen acceptance battery, a family answer with
inner and outer design sets and witnesses, a correct withdrawal or confirmation of earlier verdicts after an
amendment, or a truthful `unknown` with its witness. *Independently accepted*: accepted by a checker that is not
the proposer. The clock T runs from receipt of the mission to acceptance and includes everything in between.
*Restricted mean time* at deadline H: E[min(T, H)]; an unresolved mission costs H. *Ratio* per stratum:
R = E[min(T_rival, H)] ÷ E[min(T_frob, H)]. The *strongest composed rival*: leading simulators, an optimizer,
analytic bounds, a checker and the same AI agent, assembled and tuned by a competent baseline engineer on
development fixtures and then frozen.

**Honest status.** Nothing is measured. No single mechanism delivers ten-fold and no conjunction has been shown
to. Against the incumbent as commonly run (per-design training), the expected range is 3 to 10 for quadruped
briefs and 5 to 20 for humanoid briefs; against a composed rival that hosts every portable lever, the expected
ratio on fixed-topology CUDA missions is near one unless an engine-internal property binds. A missed-deadline
fraction q caps the ratio at 1/q. A draft rule requiring a point estimate of at least 10 on every brief would
confirm a true ratio of 10 in at most 18% of simulated campaigns, so the rule is on a pooled ratio with its
power computed before the suite is opened. Tier: Solid as a measurement; it depends on Frontier levers.

## Scope

In: pre-registration; the partial reading at M3 and the tournament reading at M7 of one question: per mission
class and stratum, does the pooled ratio of restricted mean times against the strongest composed rival confirm
10, or fall under 3. Also in: the wording that follows each range of the ratio, and the decision records
`docs/adr/bet-13-m3.md` and `docs/adr/bet-13-m7.md`.

Out, and where it is:

- Whether the gain needs this engine: the ablation arm, the composed incumbent, the tie test, the recovery
  ratios of GATE-TENFOLD-13, KILL-TENFOLD-TIE and KILL-AUGUR-TIE. That is BET-14, which reads the same records
  of GATE-TENFOLD-06 and GATE-TENFOLD-07 for a different question.
- The tournament seen from the user's side (NS-24).
- The protocol, suite, arms, estimator, adjudication and publication: BD-314 to BD-317 and the gate record
  N40-15 for the partial gate; N39b-13 and the gate record N40-16 for the tournament.
- Applying the trigger of KILL-TENFOLD-SPEED and writing the kill-gate ADR (N39b-06). This bead reads that
  decision.

## Technical approach

1. **Falsifier clauses.**
   a. At M7 the decision rule does not confirm a ratio of 10 on any named stratum.
   b. Earlier, per mission class: a measured ratio under 3 on a mission class at the M3 partial gate withdraws
      the speed claim for that class.
2. **Competing arm.** The strongest composed rival, improved by a competent baseline engineer on development
   fixtures and then frozen. **Potency controls.** The ceiling computed before the test; routine easy wins for
   conventional methods locked into the suite (an easy win and an unresolved answer in every stratum);
   consequential edits hidden among inconsequential ones.
3. **Deciding gates.**
   - GATE-TENFOLD-06, the partial gate (M3): on the legged-quadruped and wheeled strata, and on the envelope
     and sensing strata where ready: the pooled ratio with its interval, the verdict, completion, coverage, the
     tie test and phase ledgers for three arms; the written outcome rules applied. Met when the record exists,
     whatever the ratio.
   - GATE-TENFOLD-07, the tournament (M7): the same record for every stratum run, on fresh sealed families with
     every pin frozen, published with its ledgers and its replication kit. Met when the report is published,
     whatever it says.
4. **Kill gate.** KILL-TENFOLD-SPEED (M3 for the classes run; M7): a measured ratio below 3 on a confirmatory
   cell, or a futility stop, withdraws the speed claim for that class; the product is described by what does
   not depend on speed.
5. **What ships for each outcome (the fallback is a wording and a positioning, fixed now).**
   - Confirmed on a stratum: the word "ten-fold" may be used for that stratum only, with the ratio, its
     interval and the rival's configuration.
   - Between 3 and 10: the word "ten-fold" is not used; the product is described by its measured ratio per
     stratum.
   - Under 3: the product is positioned on auditability: replay, qualification, certificates and the evidence
     ledger. Under 3 on every class at M3: speed work is no longer funded, and the family-policy spend of M4 is
     decided again.
   - A sealed-suite result is a finite-suite result; a claim about the population of missions needs at least 32
     independent mission families.
6. **Evidence so far.** The ceilings (N00-17), the composed published ledger (N00-18), the expected ranges
   (N00-20). The M0 and M1 incumbent phase ledgers are the first measurements that bear on it.
7. **Procedure.** Register clauses, strata, the decision rule and the wording table before the M3 suite is
   sealed; read at M3 exit and at M7 exit; `docs/adr/bet-13-m3.md`, `bet-13-m7.md`.

## Acceptance criteria

- [ ] Pre-registered before the M3 suite is sealed; the wording table is part of the registration.
- [ ] At M3: a verdict per mission class run, with ratio, interval, completion and the consequence applied
      (speed claim kept or withdrawn for that class).
- [ ] At M7: a verdict per stratum; "ten-fold" appears in the report and in any document only where the
      decision rule confirmed it (checked by the metric contract and the claims lint).
- [ ] Every stated ratio conforms to the metric contract (cold and warm separate, resource columns, interval,
      rival configuration digest, ratio not above 1/q).

## Test plan

### Unit tests
`bets.rs`: `bet13_confirmed_on_one_stratum_holds_for_that_stratum`; `bet13_no_stratum_confirmed_falsifies_a`;
`bet13_class_ratio_2_4_withdraws_speed_claim_at_m3`; `bet13_wording_between_3_and_10_has_no_tenfold_word`;
`bet13_all_classes_under_3_stops_speed_funding` (the funding check then refuses `speed-lever` records);
`bet13_population_wording_needs_32_families`.

### End-to-end script and logging
`scripts/e2e/bet_13_decide.sh` reads the gate records and the published report, runs the metric-contract
conformance check on the report, recomputes the per-stratum verdict and wording, and scans the tree's documents
for the word "ten-fold" outside confirmed strata. Logged per STD-E2E: stratum, ratio, interval, completion, cap
1/q, verdict, wording; `summary` naming GATE-TENFOLD-06, GATE-TENFOLD-07 and KILL-TENFOLD-SPEED.

### Known answers and negative controls
KA-009 (R ≤ 1/q; 25/3 at 12% unfinished); the draft-rule power of at most 18% (CX-003). Negative controls: a
report that pools cold and warm runs; one that drops unresolved missions (CX-008); one that uses "ten-fold" for
a stratum with ratio 7.

## Considerations

- A published ratio of 2 with an honest ledger satisfies the program's definition of done; an unpublished ratio
  of 12 does not.
- Where no neutral party adjudicated, the report says `self-adjudicated` and "independently accepted" is not
  used.
- The tournament is not stopped by an interim number; it is voided only for a breach of protocol.

## References

§4 (BET-13), §40.5, §2.1, §2.6, §39, §43.6, §43.10, §47.1; GATE-TENFOLD-06, -07; KILL-TENFOLD-SPEED; KA-009;
CX-003, CX-008.
