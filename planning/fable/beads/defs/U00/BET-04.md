---
key: "BET-04"
title: "Decide BET-04: FrankenSim can be consumed by path from outside its tree, stably, on one toolchain"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "frob-fsim", "solid", "bet", "gate"]
parent: "WP-02"
deps: ["N00-28", "BD-006", "BD-031", "BD-038", "STD-E2E"]
needs: ["§33.8: the drift log that counts working days lost to upstream breakage in each four-week window", "§33.13: the toolchain train procedure and the `frob-units` escape hatch"]
related: ["BD-007", "BD-032", "BD-033", "BD-034", "BD-035", "BD-036", "BD-044", "N00-16", "BET-12"]
gates: ["INT-FSIM-01", "INT-TOOLCHAIN-01", "KILL-FSIM"]
plan_refs: ["§4 (BET-04)", "§40.5", "§1.7", "§33.8", "§43.3", "§43.12"]
---
## Background

**The bet.** FrankenSim, the sibling project that supplies referee-grade dynamics, contact, geometry, units and
statistics, can be consumed by path dependency from outside its own tree, stably, and its units crate's coupling
to a nightly compiler feature does not block the core. The whole architecture assumes it: FrankenSim is the
referee and the bedrock, taken by path and pinned by commit behind one airlock crate, never forked.

**Why it could fail.** FrankenSim's API moves daily. Its units crate needs the nightly feature
`generic_const_exprs`, and most of its physics crates depend on that crate, so the toolchain is tied to nightlies
on which the feature compiles. The consumed siblings start on four different pinned nightlies; the build graph
must end on one toolchain T. The closure a lift would have to carry is about 1.5 million lines. Tier: Solid.

**Owner's ruling that frames the fallback.** Every sibling is the owner's own project. A defect or mismatch is
fixed in the sibling's repository, under its own rules, as an upstream work item with an interim posture and a
switch-over rule; pins advance by a compatibility train; a sibling's newer clean head is never rolled back
locally.

## Scope

In: pre-registration, the M0 reading, the rolling re-reading at each pin move, the verdict and decision record.
Out: the admission tests, the toolchain matrix, the worker fallback and upstream fixes (their own beads).

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
- [ ] KILL-FSIM is recorded as fired or not fired, and the worker fallback has its INT-FSIM-18 record whichever
      way the bet falls.
- [ ] The drift log exists and the four-week share is computed at each re-reading.

## Test plan

### Unit tests
`bets.rs`: `bet04_both_admissions_pass_holds`; `bet04_cold_check_70_minutes_fires_kill` (over the one-hour
envelope); `bet04_breakage_5_of_20_days_falsifies_c` (25% > 20%); `bet04_breakage_4_of_20_days_holds` (20% is
not "more than 20%"); `bet04_defect_test_passing_on_defect_commit_is_untested`.

### End-to-end script and logging
`scripts/e2e/bet_04_decide.sh` reads the admission records and the drift log, recomputes the verdict and checks
the ADR; it then confirms that `frob doctor --json` lists the admitted foundations with the same records. Logged
per STD-E2E: gate, commit, toolchain, check time and memory against the envelope, digest equality, drift share
per window, defect tests' results on the defect and fixed commits; `summary` naming the gates and KILL-FSIM.

### Known answers and negative controls
KA-003 (planar drive: displacement about (0.834, 0.551) m), KA-004 (ratio stays 1, not 1.582459), KA-005,
KA-006, KA-007 (stride 3, not 12), KA-008 (witness migration). Negative controls: the defect commits; a stack
lock with a second toolchain; a locally patched sibling (caught by the stack check).

## Considerations

- The risk is no longer that an upstream request goes unstaffed; it is the cost and sequencing of upstream work
  and collisions with other agents in the same sibling.
- Admission is never inherited: a foundation first used at a later milestone is admitted then, at that day's
  pin.
- A pin move during the ten-fold tournament voids the runs it spans.

## References

§4 (BET-04), §40.5, §1.7, §33.8, §43.3, §43.12; INT-FSIM-01, INT-TOOLCHAIN-01, INT-FSIM-18; KILL-FSIM.
