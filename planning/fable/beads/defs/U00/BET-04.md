---
key: "BET-04"
title: "Decide BET-04: FrankenSim can be consumed by path from outside its tree, stably, on one toolchain"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "frob-fsim", "solid", "bet", "gate"]
parent: "WP-02"
deps: ["BD-006", "BD-031", "BD-038", "N00-28", "N33a-10", "N33b-04", "N33b-05", "N33c-08", "STD-E2E"]
needs: []
related: ["BET-12", "N00-16", "N33c-14", "N40-31", "N40-32", "N41-19"]
gates: ["INT-FSIM-01", "INT-TOOLCHAIN-01", "KILL-FSIM"]
plan_refs: ["§4 (BET-04)", "§40.5", "§1.7", "§33.8", "§43.3", "§43.12"]
---
## Background

**The bet.** FrankenSim, the sibling project that supplies referee-grade dynamics, contact, geometry, units and
statistics, can be consumed by path dependency from outside its own tree, stably, and its units crate's coupling
to a nightly compiler feature does not block the core. The whole architecture assumes it: FrankenSim is the
referee and the bedrock, taken by path and pinned by commit behind one airlock crate, never forked.

**Why it could fail.** FrankenSim's API moves daily. Its units crate `fs-qty` needs the nightly feature
`generic_const_exprs`, and most of its physics crates depend on that crate, so the toolchain is tied to nightlies
on which the feature compiles. The consumed siblings start on four different pinned nightlies; the build graph
must end on one toolchain T. The closure a lift would have to carry is about 1.5 million lines. Tier: Solid.

**Owner's ruling that frames the fallback.** Every sibling is the owner's own project. A defect or mismatch is
fixed in the sibling's repository, under its own rules, as an upstream work item with an interim posture and a
switch-over rule; pins advance by a compatibility train; a sibling's newer clean head is never rolled back
locally.

## Scope

In: pre-registration, the M0 reading, the rolling re-reading at each pin move, the verdict and decision record.
Out, with the beads that have them: INT-FSIM-01 and the core row of INT-TOOLCHAIN-01 (BD-006); the toolchain
matrix and the choice of T (BD-031); the worker fallback exercised as INT-FSIM-18 (BD-038); the KILL-FSIM trigger
ledger with the effort clock, the cold-check envelope and the breakage share (N33a-10); the compatibility train
and its log (N33b-04); the KILL-FSIM decision itself (N33b-05), which this bead reads and cites and never
re-decides; the M0 exit review that records it (BD-044); the upstream fixes (their FD-FSIM items).

## Technical approach

1. **Falsifier clauses** (any falsifies):
   a. INT-FSIM-01 fails: a crate depending by path on the bedrock crates (`fs-ga`, `fs-mbd`, `fs-contact`,
      `fs-time`, `fs-query`) must pass `cargo check --all-targets` and reproduce the digest of a G1
      forward-dynamics state at the pinned commit; the cold check fits one hour, 16 GiB and 8 cores.
   b. INT-TOOLCHAIN-01 fails: T is chosen and recorded in the stack lock and the toolchain file, in the order of
      preference (a) the suite's common nightly, (b) FrankenSim's present pin; the whole build graph builds and
      tests on T; the matrix of which nightlies compile the units crate is recorded.
   c. Upstream breakage costs more than 20% of working days in any four-week window.
2. **Potency control.** The admission tests for the five reported foundation defects must fail on the pinned
   commit if the defects are real, and pass after the upstream fix: the planar-drive near-straight case; the
   three policy-gradient update errors; the joint law of sampled actions; the stride convention; the
   fixed-witness contact path. A test that passes on a commit known to have the defect has no power.
3. **Kill gate and fallback, in order.** KILL-FSIM (M0, then rolling): INT-FSIM-01 cannot pass within two weeks
   of M0 effort, or its cold check exceeds the machine envelope, or clause (c) holds. Fallback (a): one pinned
   FrankenSim worker process per device that exchanges whole jobs through the airlock traits, already exercised
   by INT-FSIM-18 (a 2,000-step G1 referee rollout through the worker returns chunk digests bit-equal to the
   in-process run; a worker killed mid-job and resubmitted returns the same result). Fallback (b), only if (a)
   fails or the pin can no longer be built: lift the minimal referee set with provenance headers and golden
   tests, and state lift-only. If only the units crate blocks: a small stable `frob-units` crate.
4. **Downstream if falsified.** M0's stop rule: re-plan before M1 if INT-FSIM-01 cannot pass within two weeks
   and the worker also fails. Under fallback (a) every referee capability is described as "through the worker"
   and latency is disclosed; the CPU studio of M1 still ships.
5. **Evidence so far (FACT).** The sibling code exists at commit `483d09c682f2` with the pieces named in the
   foundation fact sheet (N00-16). Nothing has been built against it from this repository.
6. **Procedure.** Register before the first admission run; read at M0 exit; re-read at every pin move and at the
   end of each four-week window from the drift log; `docs/adr/bet-04-m0.md`, appended on each re-reading.

## Acceptance criteria

- [ ] Pre-registered before INT-FSIM-01 is first run.
- [ ] INT-FSIM-01 and INT-TOOLCHAIN-01 each have a record (pass or fail) with check time, peak memory, package
      count and sibling drift; the choice of T is recorded.
- [ ] The defect admission tests were shown to fail on the defect commit (or the defect is recorded as not
      reproduced, which is itself a finding).
- [ ] KILL-FSIM is on record as fired or not fired (N33b-05's ADR, cited by identifier and hash; if it is not on
      record the verdict is not filed), and the worker fallback has its INT-FSIM-18 record whichever way the bet
      falls.
- [ ] The drift log exists and the four-week share is computed at each re-reading.

## Test plan

### Unit tests
`bets.rs`: `bet04_both_admissions_pass_holds`; `bet04_cold_check_70_minutes_falsifies_a` (70 minutes is over the
one-hour envelope, so INT-FSIM-01 fails; the KILL-FSIM outcome is read from N33b-05's record, not computed here); `bet04_breakage_5_of_20_days_falsifies_c` (25% > 20%); `bet04_breakage_4_of_20_days_holds` (20% is
not "more than 20%"); `bet04_defect_test_passing_on_defect_commit_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_04_decide.sh` reads the admission records and the drift log, recomputes the breakage share per
four-week window, reads the verdict of BET-04 from the bet register's report (`cargo run -p frob-conformance
--example bets -- status --json`, example `bets` of N00-28, the one verdict rule; no second verdict rule in the
script) and checks the ADR against both; it then confirms that `frob doctor --json` lists the admitted foundations with the same records. Logged
per STD-E2E: gate, commit, toolchain, check time and memory against the envelope, digest equality, drift share
per window, defect tests' results on the defect and fixed commits; `summary` naming the gates and KILL-FSIM.

### Known answers and negative controls
KA-003 (planar drive at heading 32 rad, wheel speeds differing by 6 × 10⁻¹³ m/s, one second: displacement about
(0.834, 0.551) m against an independent SE(2) exponential, not zero), KA-004 (two independently reset one-step
episodes under a frozen policy: the second probability ratio stays 1, not 1.582459), KA-005 (negative advantage:
the clipped surrogate on the correct side of the ratio), KA-006 (critic gradient into the shared bias non-zero,
equal to the squared-error gradient), the action sampler's joint law (adjacent channels agree in sign with
probability ½ under independence, not 0.6655), KA-007 (converted stride 3, not 12), KA-008 (rolling round a sharp
lip changes the material witness and the contact frame, and the witness migrates). Negative controls: the defect commits; a stack
lock with a second toolchain; a locally patched sibling (caught by the stack check).

## Considerations

- The risk is no longer that an upstream request goes unstaffed; it is the cost and sequencing of upstream work
  and collisions with other agents in the same sibling.
- Admission is never inherited: a foundation first used at a later milestone is admitted then, at that day's
  pin.
- A pin move during the ten-fold tournament voids the runs it spans.

## References

§4 (BET-04), §40.5, §1.7, §33.8, §43.3, §43.12; INT-FSIM-01, INT-TOOLCHAIN-01, INT-FSIM-18; KILL-FSIM.
