---
key: "BET-11"
title: "Decide BET-11: proof-carrying kernels are affordable (certificates cover at least 80% of derived kernels)"
type: "task"
priority: 2
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "solid", "bet", "gate"]
parent: "WP-31"
deps: ["BD-379", "N00-10", "N00-28", "N12-31", "STD-E2E"]
needs: []
related: ["BET-20", "N00-15", "N40-31", "N40-32", "NS-11"]
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

In: pre-registration, the M5 reading, the decision-consequence count, verdict and decision record. Out, with the
beads that have them: the identity catalogue and the instance-certificate pipeline (N12-20); the coverage
measurement, its report and the GATE-LEMMA-01 record (BD-379) and its end-to-end script (N12-36); the
decision-effect ledger that counts consumed and changed certificates (N12-12); the KILL-LEMMA decision (N12-31);
checkers, the auditor, Lean capsules and reflection (their own beads).

## Technical approach

1. **Falsifier.** Instance certificates cover fewer than 80% of derived kernels within the symbolic tool's
   budgets. Coverage is counted by kernel, as BD-379 computes it: coverage = covered kernels ÷ derived kernels,
   an exact fraction, where a kernel is covered only if it declares every identity the identity catalogue
   requires for its family (a kernel that declares fewer counts as uncovered, so an empty declaration is never
   full coverage) and every declared identity has a replayable instance certificate, bound to the kernel's
   content hash, that `frob-verify` accepts offline. A kernel with two declared identities and one certificate
   is uncovered. Identity-level counts are printed beside it but do not decide.
2. **Potency control.** Mutated kernels (a flipped sign, a dropped term) whose certificates must fail to check
   (BD-379's control: each mutant ends `not-certified` with no certificate, its forged bundle is rejected by
   `frob-verify`, and its pure relabel fails `check-generated` and is not counted). A coverage figure obtained with
   a checker that also accepts the mutants is void.
3. **Record required beside the verdict.** For every certificate consumed by a decision anywhere in the system
   (a promotion, a mission answer, a qualification), whether the decision would have differed without it: the
   decision-effect ledger's `cert_effect` rows (N12-12: the decision rule re-evaluated on the counterfactual
   inputs, `changed` true, false or `unknown`, and `unknown` never counted as changed), read with
   `frob prove list --effects --json`. The counts consumed, changed, unchanged and unknown are printed with the
   verdict. This does not enter the falsifier; it answers the open question of value.
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
7. **Evidence so far.** The symbolic sibling's caps are known (FACT at FrankenSymPy commit `29f847c1`, re-read at
   the pin: its bytecode interpreter caps an expression at 8,192 operations; Buchberger stops at 1,024 basis
   polynomials and has no F4; the real solver `real_rur` is capped at 8 variables, 64 equations, degree 256 and
   4,096-bit coefficients, zero-dimensional ideals only); the lean-checker sibling's tactic support is narrow (a
   decision-tree procedure and `omega`, 1,423 lines). Three theorem cards are certified for the M0 kernels as the
   first data point, as four certificates on three kernels (GATE-LEMMA-00).
8. **Procedure.** Register the kernel list, the identities per kernel and the budgets before the coverage run;
   read at M5 exit; `docs/adr/bet-11-m5.md`.

## Acceptance criteria

- [ ] Pre-registered with the list of derived kernels and declared identities frozen at registration.
- [ ] Coverage recorded as covered kernels over derived kernels (exact fraction and percentage), with each
      kernel's declared and certified identities and the uncovered cases listed by kernel with their status from
      N12-20's closed set (`not-certified`, `budget-exceeded` with the cap, `not-polynomial`, `tested-only`, or
      the kernel-level `uncovered: under-declared`), before and after the one repair cycle.
- [ ] Every mutated kernel's certificate failed to check, in the same run (status `not-certified`, forged bundle
      rejected, relabel not counted).
- [ ] The decision-consequence count is printed (consumed, changed).
- [ ] The bet's verdict is recorded as an ADR that cites the KILL-LEMMA decision record. The kill gate is decided
      once, by the kill-gate bead N12-31, which applies the trigger and writes the kill ADR; this bead reads that
      record, evaluates the bet's own falsifier and writes the bet verdict, and writes no second kill decision.

## Test plan

### Unit tests
`crates/frob-conformance/tests/bets.rs` (N00-28's verdict function on synthetic gate records):
`bet11_coverage_0_84_holds` (42 of 50 kernels covered); `bet11_coverage_0_71_falsifies_without_kill`
(N12-31's record: not fired; bet `Falsified`, fallback not installed, coverage printed);
`bet11_reads_kill_lemma_fired_at_0_46` (23 of 50 after the repair cycle; N12-31's record says fired; the bet
carries the record's hash and names the fallback); `bet11_kernel_with_one_of_two_identities_is_uncovered`;
`bet11_mutant_accepted_voids_coverage`; `bet11_identity_list_changed_after_registration_rejected`;
`bet11_unknown_effect_not_counted_as_changed`.

### End-to-end script and logging
`scripts/e2e/bet_11_decide.sh` reads the gate rows with `cargo run -p frob-conformance --example
registry_report -- gates --chapter 12 --milestone M5 --json`, recounts coverage from `frob prove list --coverage
--json` against the frozen registration (kernel list and declared identities), replays every counted
certificate offline with `frob-verify` on a host with no GPU, reads the mutant control's results from the
GATE-LEMMA-01 record of N12-36, reads `frob prove list --effects --json` for the consumed, changed, unchanged and
unknown counts, reads the verdict of BET-11 from the bet register's report (`cargo run -p frob-conformance
--example bets -- status --json`, example `bets` of N00-28, the one verdict rule), and checks that
`docs/adr/bet-11-m5.md` cites the same values and the
KILL-LEMMA outcome and ADR of N12-31 (`docs/adr/ADR-KILL-LEMMA-M5.md`). Logged per STD-E2E
(`target/e2e-logs/bet_11_decide/<utc>.ndjson`: `ts`, `script`, `bead` (`BET-11`), `step`, `event`, the exact
command line, exit code, wall time, the envelope's `evidence` block): kernels, identities declared and
certified, coverage as an exact fraction against 4/5 (bet) and 1/2 (kill), replay verdicts with certificate
hashes, mutants caught out of those planted, the four effect counts, the receipt hashes read; on failure the
first value that differs between the ADR and the gate row, or between the registration and the coverage rows;
`summary` naming GATE-LEMMA-01 and KILL-LEMMA.

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
