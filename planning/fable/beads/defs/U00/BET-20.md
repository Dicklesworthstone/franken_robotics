---
key: "BET-20"
title: "Decide BET-20: region certificates (per-constraint, about six split parameters) resolve a useful family share"
type: "task"
priority: 1
milestone: "M3"
labels: ["m3", "augur", "lemma", "frob-mission", "frontier", "bet", "gate"]
parent: "WP-59"
deps: ["BD-136", "BD-289", "N00-10", "N00-28", "N38a-09", "STD-E2E"]
needs: []
related: ["N00-11", "N00-CX", "N25e-23", "N25e-24", "N25e-26", "N40-31", "N40-32"]
gates: ["GATE-AUGUR-09", "KILL-AUGUR-REGION"]
plan_refs: ["§4 (BET-20)", "§40.5", "§0.8 (H-06)", "§2.4 (lever 5)", "§25.27", "§43.12"]
---
## Background

**The bet.** A family answer needs statements about whole regions of design space, not about sample points: "
every design in this box satisfies the constraint". The naive way, covering a design box with small certified
boxes in every dimension (isotropic paving), is out of reach for a 10 to 20 dimensional box and is not
attempted. The bet is on a narrower mechanism: a certificate *per constraint*, with bisection confined to a
*split set* of about six parameters the constraint is sensitive to, while the remaining parameters are enclosed
at full range; a box of any dimension is admitted when that enclosure is narrower than the constraint's margin.
Per-constraint certificates are then composed by monotonicity, symmetry and witnesses. The bet is that this
resolves a useful share of a family. Tier: Frontier.

**What is unknown.** Whether real robot constraints have narrow weak-variable enclosures is unmeasured. And
whether any formal certificate changes a design decision is unknown either way; the consumed-certificate record
is kept for this bet as for the kernel-certificate bet.

Notation: I is the inner set (certified to satisfy), O the outer set (not excluded); the unresolved band is
O ∖ I.

## Scope

In: pre-registration, the M3 reading (the pilot on the 12-parameter box), verdict and decision record. Out,
with the beads that have them: the M4 addendum (the gate's second part at scale), written by N25b-11 after
this bead; the pilot on the 12-parameter
box with its four paving arms, and the KILL-AUGUR-REGION decision (BD-289), which this bead reads and cites and
never re-decides; the per-constraint certificates at scale (BD-358, M4) and their end-to-end script (N25b-11);
the mutation corpus (BD-136); the weak-variable enclosure that may lift the six-parameter limit, with its own
kill gate KILL-WEAKVAR (N25e-23, N25e-24, N25e-26, M4; not this bet's kill gate).

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. On the reference families the unresolved band O ∖ I exceeds the mission's declared maximum after the
      budget.
   b. A certificate check exceeds its budget.
   c. A registered mutation (swapped contact ordering, an unreported force floor, a passive-authority tag) is
      accepted.
2. **Competing arm.** Dense sampling with statistical labels. **Potency control.** The midpoint counterexample:
   two feasible designs whose midpoint is infeasible must be excluded from every inner set. Numbers: the designs
   (e, W) = (0, 1.9) and (0.9, 1) each need 0.95 N; their midpoint needs 841/800 N from a pair capped at 1 N.
3. **Deciding gate (TARGET).** GATE-AUGUR-09 (Known answer; M3 as a pilot, M4): region certificates on the
   12-parameter box: the per-constraint arm decides at least 50% of each constraint's sub-box volume within 10⁴
   boxes; no single monotone bound gives more than 95% of decided volume; zero certified boxes contradicted by
   10⁶ point evaluations.
4. **Kill gate and fallback.** KILL-AUGUR-REGION (M3): the pilot meets its falsifier (more than 90% undecided in
   the best arm, or more than 95% of decided volume from one monotone bound), or GATE-AUGUR-09 fails after one
   repair cycle. Fallback: region certificates are removed; family answers use dual witnesses, monotone bounds
   and pointwise static analyses; untested regions are reported as sample coverage, class Estimated, and make
   no inner-set claim.
5. **Downstream if falsified.** Mission answers on continuous families are reported as tested sample sets with
   witnesses; the "answer over families" lever is delivered by exclusions and witnesses on polyhedral and
   finite fragments only; the scale gate of mission queries on GPUs keeps its exact-integer screens.
6. **Evidence so far.** The pilot fixtures on small plants (exact family answers by conventional means tie a
   specialized method); the arithmetic that isotropic paving is infeasible (about 10⁴ boxes are 13 bisections
   in total; spread over a 20-dimensional box they shrink each side only to 2^(−13/20) = 0.64 of its length).
7. **Procedure.** Register the families, the declared maximum band, the budgets and the mutation list before
   the pilot; read at M3 exit; `docs/adr/bet-20-m3.md`. The M4 part (the gate's second reading at scale) is
   written by the M4 gate script (N25b-11), which comes after this bead in the graph, as
   `docs/adr/bet-20-m4.md` citing this record; this bead's acceptance is the M3 reading and never waits on M4.

## Acceptance criteria

- [ ] Pre-registered with budgets and the declared maximum unresolved band per reference family.
- [ ] Per constraint: decided sub-box volume share within 10⁴ boxes; the largest share contributed by one
      monotone bound; the count of certified boxes contradicted by 10⁶ point evaluations (must be zero).
- [ ] All three registered mutations rejected; the midpoint excluded from every inner set.
- [ ] Verdict recorded in `docs/adr/bet-20-m3.md`, citing the KILL-AUGUR-REGION decision on record by BD-289 (by
      identifier and hash; with no decision on record the verdict is not filed); the consumed-certificate count
      printed.

## Test plan

### Unit tests
`bets.rs`: `bet20_decided_58_percent_holds`; `bet20_decided_44_percent_fails_gate`;
`bet20_reads_kill_region_at_93_percent_undecided` (BD-289's record: fired; the bet is `Falsified` and carries
the record's hash); `bet20_reads_kill_region_one_bound_97_percent` (fired because the mechanism is then a
monotone bound, not region certificates; read from the record, not computed here); `bet20_one_contradicted_box_falsifies`;
`bet20_mutation_accepted_falsifies_c`; `bet20_midpoint_in_inner_set_fails_control`.

### End-to-end script and logging
`scripts/e2e/bet_20_decide.sh` recomputes volume shares from the pilot's cell records (BD-289's
`tools/pilots/region-box/` output, read by its protocol digest), checks the recorded digest of the 10⁶ point
evaluations, runs the three registered mutations of clause (c) from BD-136's corpus through the standalone
`frob-verify <bundle>.frobcert --json` (each must be rejected; the pilot's own boxes are not bundles, so their
soundness rests on BD-289's point check), reads the verdict of BET-20 from the bet register's report (`cargo run
-p frob-conformance --example bets -- status --json`, example `bets` of N00-28, the one verdict rule) and checks
`docs/adr/bet-20-m3.md` against it. Logged per STD-E2E to `target/e2e-logs/bet_20_decide/<utc>.ndjson`: per call the command line, exit
code and wall time; per constraint the boxes used, decided share against 50%, largest single-bound share
against 95%; contradictions against 0; check time against budget; per mutation the first failing rule; the
KILL-AUGUR-REGION record's identifier and hash; `summary` naming GATE-AUGUR-09 and KILL-AUGUR-REGION.

### Known answers and negative controls
CX-010 (the midpoint: 0.95 N, 0.95 N, 841/800 = 1.05125 N against a 1 N cap). 2^(−13/20) = 0.637. Negative
controls: the three mutations; a convexity shortcut that admits midpoints.

## Considerations

- A share decided by one monotone bound is a success of monotonicity, and the kill rule exists so that it is
  not credited to region certificates.
- A certified box is a statement about the declared model and its assumptions; assumptions are inputs with
  their own claim products.

## References

§4 (BET-20), §40.5, §0.8 (H-06), §2.4, §25.27, §43.12; GATE-AUGUR-09; KILL-AUGUR-REGION; CX-010.
