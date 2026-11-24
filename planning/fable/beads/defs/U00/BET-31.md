---
key: "BET-31"
title: "Decide BET-31: campaign quotienting (exact deduplication under symmetry and observables, losing no winner)"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "phylum", "atelier", "frob-phylum", "frontier", "bet", "gate"]
parent: "WP-63"
deps: ["BD-285", "N00-28", "N17a-28", "N17a-29", "N38a-09", "STD-E2E"]
needs: []
related: ["BET-33", "GB-19", "N00-20", "N00-CX", "N17a-30", "N40-31"]
gates: ["GATE-PHYLUM-01", "KILL-QUOTIENT"]
plan_refs: ["§4 (BET-31)", "§40.5", "§2.4 (lever 6)", "§17.19"]
---
## Background

**The bet.** A design campaign proposes many candidates, and some of them are the same robot for the purposes
of the mission: mirror images of each other under a symmetry the task preserves, or designs that differ only in
ways no mission observable can see. *Campaign quotienting* evaluates one representative per equivalence class.
Two notions are used: orbit representatives under the observation-preserving symmetry group (a canonical form
of the design up to that group), and a *mission-observable digest* (a content hash of everything about a design
that the mission's observables depend on). The bet is that exact deduplication of this kind removes a
measurable share of campaign work and never removes a winner. Tier: Frontier.

**What the evidence supports.** Deduplication by symmetry gives 2× with mirror symmetry alone and more on
special fixtures. The danger is a false equivalence: two designs declared the same whose outcomes differ.

## Scope

In: pre-registration, the M3 reading, verdict and decision record. Out: canonical forms, orbit
representatives and the mission digest (`phylum.quotient`, BD-285); the frozen campaign corpus (N17a-30); the
quotient audit that replays 10⁴ in-class pairs and counts dispatches saved against the unquotiented arm
(N17a-28); applying the two triggers of KILL-QUOTIENT and writing the kill ADR, which is the kill-gate decision
bead N17a-29, a dependency: this bead reads that record and the quotient audit it was decided on, evaluates the
bet's further falsifiers (the planted winner's rank, agreement of the top eight) and writes the bet verdict,
and writes no second kill record.

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. One false equivalence: two designs in one class whose mission outcomes differ beyond the determinism
      class.
   b. The planted winner loses rank 1.
   c. Fewer than six of the top eight agree with the unquotiented arm.
   d. Digest classes save less than 5% of dispatches on the campaign corpus.
2. **Competing arm.** The same campaign with quotienting off. **Potency controls.** A planted winner; the
   coupled-bus fixture, in which two modules with identical local summaries share one current-limited bus and
   must not be merged (on a unit-current bus no one-tick schedule is feasible and two two-tick schedules are:
   the coupling, not the local summary, decides).
3. **Deciding gate (TARGET).** GATE-PHYLUM-01 (Known answer, M3): the orbit-count and ballast known answers
   exact (86,185 and 500,500 hexapod representatives; 24,546 ballast classes from 735,471 placements; see Known
   answers); zero false quotients in 10⁴ sampled class pairs by replay; the lidar control (a sensor that breaks a
   symmetry the mechanics have) and the equal-Σx control (ballast placements with equal Σx and different Σx²)
   caught.
4. **Kill gate and fallback.** KILL-QUOTIENT (M3): one false quotient survives a repair cycle, or digest
   classes save less than 5% of dispatches on the campaign corpus. Fallback: orbit representatives only; the
   digest is kept as a cache key with no claim. The bet's own fallback: deduplication by canonical form and
   content hash only.
5. **Downstream if falsified.** The "do not recompute or re-decide" lever keeps the memo graph and revalidation
   and loses quotienting's share; family search in mission queries memoizes on full model identity only.
6. **Procedure.** Register the corpus, the planted winner, top-k = 8 with the six-of-eight rule and the 5%
   saving threshold before the campaign; read at M3 exit; `docs/adr/bet-31-m3.md`.

## Acceptance criteria

- [ ] Pre-registered with the corpus digest and the planted winner sealed from the campaign's operator.
- [ ] False quotients out of 10⁴ sampled class pairs (must be zero); dispatches saved as a share; top-eight
      agreement; the planted winner's rank.
- [ ] The coupled-bus modules were not merged; the lidar and equal-sum controls were caught.
- [ ] The bet verdict is recorded as an ADR that cites the KILL-QUOTIENT decision (the kill ADR is on record by
      N17a-29).

## Test plan

### Unit tests
`bets.rs`: `bet31_zero_false_12_percent_saved_7_of_8_holds`; `bet31_one_false_equivalence_falsifies_a`;
`bet31_planted_winner_rank_2_falsifies_b`; `bet31_five_of_eight_falsifies_c`;
`bet31_saving_3_percent_falsifies_d`; `bet31_reads_kill_quotient_record` (N17a-29's record fired on the 5%
saving trigger → `Falsified` through its kill gate, the record's hash carried; the trigger is applied there);
`bet31_coupled_bus_merged_fails_control`.

### End-to-end script and logging
`scripts/e2e/bet_31_decide.sh` runs `frob phylum quotient --designs <set> --task <task> --json` on a small design
set containing a mirror pair, the lidar-broken pair and the coupled-bus pair, replays one member of each merged
class against its representative with `frob sim replay <episode> --key b31-replay-<n> --json` (a run verb, one
key per call) to compare outcome digests, recomputes the campaign statistics from the ledger, reads the verdict
of BET-31 from the bet register's report (`cargo run -p frob-conformance --example bets -- status --json`,
example `bets` of N00-28, the one verdict rule) and checks `docs/adr/bet-31-m3.md` against it. Logged per STD-E2E: class sizes, representatives, replayed digest equality, dispatches with and
without quotienting, top-eight lists, planted-winner rank; `summary` naming GATE-PHYLUM-01 and KILL-QUOTIENT.

### Known answers and negative controls
Burnside's count for the simplest case: a design with an option in {A, B} on its left leg and, independently,
on its right leg gives 4 designs; the mirror swaps the legs and fixes the 2 designs with equal options, so
there are (4 + 2)/2 = 3 distinct designs and mirror deduplication saves 25% of that population; for a generic
asymmetric population the saving approaches one half (the "2×" above). The gate's rows (§17.18): a hexapod
sweep of ten leg types on six legs under the dihedral group of order 12 has
(10⁶ + 3·10⁴ + 4·10³ + 2·10² + 2·10)/12 = 86,185 representatives (negative control: counting under the rotation
subgroup only); under the sagittal mirror alone (10⁶ + 10³)/2 = 500,500 (negative control: quotienting under the
mechanical group when a front-only sensor is mounted, the lidar control); eight identical ballast slugs in 24
collinear slots give C(24, 8) = 735,471 placements in 24,546 mission-observable classes, and two eight-slug
placements that share six slugs and differ only in slots {1, 4} against {2, 3} (slots numbered 1 to 24; the
differing pair has Σx = 5 in both and Σx² of 17 against 13) must split (the equal-Σx control; the same fixture as
BD-285 and card GB-19). CX-011, the
coupled-bus fixture: zero one-tick schedules and two two-tick schedules on a unit-current bus (negative control:
pruning by summed peak demand). Negative controls: the coupled-bus pair; the lidar pair; the equal-Σx pair; a
digest that omits a field the mission observes.

## Considerations

- "Beyond the determinism class" is the test for a false equivalence: two representatives must agree bit for
  bit under class D2 or within the envelope under D3, as the campaign declares.
- A merge is only as safe as the list of mission observables; an amended mission invalidates digest classes.

## References

§4 (BET-31), §40.5, §2.4, §17.19; GATE-PHYLUM-01; KILL-QUOTIENT; CX-011.
