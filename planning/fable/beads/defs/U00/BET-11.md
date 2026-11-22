---
key: "BET-11"
title: "Decide BET-11: proof-carrying kernels are affordable (certificates cover at least 80% of derived kernels)"
type: "task"
priority: 2
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "solid", "bet", "gate"]
parent: "WP-31"
deps: ["N00-28", "N00-10", "BD-379", "STD-E2E"]
needs: ["§12.6: the per-kernel manifest field that declares which identities a kernel claims, and FrankenSymPy's time and size budgets for one instance certificate"]
related: ["BD-021", "BD-027", "BD-381", "BD-382", "BET-01", "BET-20", "NS-11", "N00-15"]
gates: ["GATE-LEMMA-01", "KILL-LEMMA"]
plan_refs: ["§4 (BET-11)", "§40.5", "§12.6", "§0.8 (last open disagreement)", "§1.6 (N5)", "§43.8"]
---
## Background

**The bet.** Named kernels carry offline-checkable certificates of algebraic identities: for example, that an
update stays on the group, or that it conserves spatial momentum in free flight. A certificate is produced at
build time by the symbolic sibling's proof kernel for the kernel's derived expression (an *instance
certificate*), travels in a bundle, and is re-checked offline by an auditor that links nothing but the standard
library. The bet is that this is affordable: that most derived kernels can be certified within the symbolic
tool's budgets.

**What the certificates do not say.** Nothing about floating-point results, and nothing about agreement with the
world. And their value to any design decision is unproven: there is no evidence either way that a formal
certificate has ever changed a decision in such a loop. The differentiator's permitted wording says exactly this
and carries the bracket "value unproven until BET-11". Tier: Solid.

## Scope

In: pre-registration, the M5 reading, the decision-consequence count, verdict and decision record. Out:
certificate production, checkers, the auditor, Lean capsules, reflection.

## Technical approach

1. **Falsifier.** Instance certificates cover fewer than 80% of derived kernels within the symbolic tool's
   budgets (coverage counted per identity each kernel's manifest declares).
2. **Potency control.** Mutated kernels (a flipped sign, a dropped term) whose certificates must fail to check.
   A coverage figure obtained with a checker that also accepts the mutants is void.
3. **Record required beside the verdict.** For every certificate consumed by a decision anywhere in the system
   (a promotion, a mission answer, a qualification), whether the decision would have differed without it: the
   ledger event `certificate.consumed {certificate, decision, decision_without, changed}` whose shape is fixed
   by the evidence ledger bead. The count of consumed certificates and of those that changed a decision is
   printed with the verdict. This does not enter the falsifier; it answers the open question of value.
4. **Deciding gate (TARGET).** GATE-LEMMA-01 (Proof, M5): at least 80% of derived kernels carry a replayable
   instance certificate for each identity their manifest declares.
5. **Kill gate and fallback.** KILL-LEMMA (M5): GATE-LEMMA-01 below 50% after one repair cycle. Fallback:
   certificates become exact-rational randomized identity tests recorded as class Tested; the word
   "certificate" leaves kernel manifests. Between 50% and 80% the bet is falsified without the kill gate
   firing: certified kernels keep their certificates, the rest carry the randomized tests, and coverage is
   printed. (DESIGN: this is the conversion's reading of the gap between the bet's 80% and the kill gate's 50%;
   the source states both numbers and no rule for the band, so the reading is flagged for the lead.)
6. **Downstream if falsified.** The differentiator N5 is rendered with the measured coverage or withdrawn;
   the scenario "a kernel with a certificate" is demonstrated for the kernels that have one. Family certificates
   for mission answers are a different mechanism and are unaffected. Work on the proof subsystem beyond the
   coverage gate is not funded further before the tournament if no consumed certificate has changed a decision
   by the M5 exit.
7. **Evidence so far.** The symbolic sibling's caps are known (8 variables, degree 256, 4,096 bits,
   zero-dimensional ideals; FACT with path); the lean-checker sibling's tactic support is narrow. Three theorem
   cards are certified for the M0 kernels as the first data point (a gate of M0).
8. **Procedure.** Register the kernel list, the identities per kernel and the budgets before the coverage run;
   read at M5 exit; `docs/adr/bet-11-m5.md`.

## Acceptance criteria

- [ ] Pre-registered with the list of derived kernels and declared identities frozen at registration.
- [ ] Coverage recorded as certified identities over declared identities, with the budget-exceeded and
      not-expressible cases listed by kernel.
- [ ] Every mutated kernel's certificate failed to check, in the same run.
- [ ] The decision-consequence count is printed (consumed, changed).
- [ ] Verdict and the KILL-LEMMA decision recorded as an ADR.

## Test plan

### Unit tests
`bets.rs`: `bet11_coverage_0_84_holds`; `bet11_coverage_0_71_falsifies_without_kill`;
`bet11_coverage_0_46_fires_kill_lemma`; `bet11_mutant_accepted_voids_coverage`;
`bet11_identity_list_changed_after_registration_rejected`.

### End-to-end script and logging
`scripts/e2e/bet_11_decide.sh` recounts coverage from the kernel manifests and the bundle store, replays every
certificate offline with `frob-verify` on a host with no GPU, runs the mutation corpus, and recomputes the
verdict. Logged per STD-E2E: kernels, identities declared and certified, replay results, mutants rejected, the
consumed and changed counts; `summary` naming GATE-LEMMA-01 and KILL-LEMMA.

### Known answers and negative controls
Thresholds 80% (bet) and 50% (kill). Negative controls: the sign-flip and dropped-term mutants; a manifest
that declares fewer identities after registration to raise coverage (rejected by the frozen list).

## Considerations

- Coverage can be gamed by declaring trivial identities; the identity list is reviewed at registration against
  the theorem cards.
- M5 is off the critical path: none of this shortens the time to a decision, and none of it gates the
  tournament.

## References

§4 (BET-11), §40.5, §12.6, §0.8, §1.6 (N5), §43.8; GATE-LEMMA-01; KILL-LEMMA.
