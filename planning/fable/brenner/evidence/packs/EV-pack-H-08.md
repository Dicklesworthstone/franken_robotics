# EV-pack H-08 — counterfactual autopsy by deterministic forks

Source labels: **FETCHED** = page or file read this session; **SEARCH** = search summary only, page not opened;
**MEMORY** = recalled, unverified; **DERIVED** = my arithmetic. Web quotes came through a summarizing fetch tool.

## Claim under test

Restoring a snapshot before a failure and applying single-factor interventions attributes the failure to its cause:
on planted single-cause failures the top factor is correct in at least 90% of cases with at most 1,024 forks of
2 seconds.

## Evidence against

1. **Closed-loop legged control is chaotic.** Young and Pugeault, NeurIPS 2024,
   https://arxiv.org/html/2410.10674v1 (FETCHED): "RL policies can be deterministically chaotic as small
   perturbations to the system state have a large impact". Maximal Lyapunov exponents for Dreamer V3: Walker Walk
   0.1614, Walker Run 0.1345, Cheetah Run 0.0337. A torso-angle perturbation of ±5×10^-4 degrees: "this small
   perturbation causes the systems to significantly diverge after 50 steps". "all deep RL methods produce chaotic
   dynamics" on the walker and cheetah tasks. The uncontrolled system is not chaotic, so the sensitivity comes from
   policy plus contact.
2. **What that means for a 2-second fork (DERIVED).** At 0.16 per control step and 80 steps (25 ms step, MEMORY),
   amplification is about 4e5. Single-precision round-off (6e-8) grows to about 2% of state scale; double precision
   (1e-16) stays near 4e-11. Marginal failures can flip in single precision, which is what GPU tiers use.
3. **Measured attribution accuracy on real robots is about 70%.** Diehl and Ramirez-Amaro, RA-L 2022,
   https://ar5iv.arxiv.org/html/2204.04483 (FETCHED): "The obtained causal models reach a sim2real accuracy of 70%
   and 72%, respectively", from 20,000 and 800,000 simulated samples, with six and eight variables that "need to be
   defined by the experiment designer".
4. **Budget arithmetic (DERIVED).** Exact Shapley over 10 binary factors is 2^10 = 1,024 forks, with no replicates
   left to measure stability, which the hypothesis's own falsifier requires.
5. **Sibling code warns against the reading.** `frankensim/crates/fs-uq/src/product_shapley.rs` (FETCHED):
   "Dependence and interactions both contribute; these are not causal effects."
6. **No prior art for the exact method.** A search for Shapley or counterfactual attribution of robot failures by
   simulator interventions returned grid-world and one lever task only (SEARCH, null result).

## Evidence for

- Counterfactual replay is established in driving. Scanlon et al. 2021 (SEARCH): 72 reconstructed fatal crashes;
  the replaced driver as responder would "prevent 82%" and "mitigate an additional 10%". The authors caution that
  single-instance counterfactual simulations "can yield variable results".
- A planted single cause with a large effect is the easy case: removing it flips the outcome unless the failure is
  marginal.
- `fs-mbd` is "Deterministic, safe-Rust rigid-body and articulated dynamics" in double precision (FETCHED file).

## Kill tests

1. Artifact: passes (fork primitive, intervention list, planted-failure test).
2. Lever: weak. The number is an accuracy, not time saved; it matters only through H-18.
3. Scale: passes; 1,024 forks of 2 seconds is small.
4. Foundation: CPU double-precision determinism exists; no GPU tier exists; the existing Shapley code is
   variance-based, not interventional.
5. Conflation: the "cause" is a cause in the model. Must be labelled so.
6. Rival: fires. Isaac Lab docs (FETCHED,
   https://docs.nvidia.com/isaaclab/isaac-lab/reproducibility-and-determinism): Newton `deterministic_mode`
   "run_to_run" requests "reproducibility on one GPU"; CPU MuJoCo is already reproducible. A state copy is a fork.

## Cheapest discriminating test

CPU MuJoCo, one trained quadruped policy, 200 planted failures in five cause classes (friction drop, torque cap,
payload shift, latency, terrain step), ten candidate factors each. Run one-at-a-time contrasts (10 forks) and exact
Shapley (1,024 forks). Repeat under 20 snapshot perturbations at 1e-7 and at 1e-12 relative. Kill: top-1 accuracy
below 90%, or the top factor changes in more than 30% of cases at 1e-7. Also report whether 10 forks match 1,024.
Cost: about one day on a 32-core machine, two to three engineer-days.

## Recommended state and the claim that survives

**NARROWED.** Interventional contrasts on deterministic forks, reported with a flip rate under perturbation. The
90% figure is plausible for non-marginal single-cause failures in double precision and is unmeasured. For marginal
or interacting causes, report a set of factors, not a top factor. Portable to incumbents.

## Open points

- Lyapunov exponents for trained quadruped policies in the target engines were not found.
- Whether ten one-at-a-time forks suffice, making the 1,024 budget unnecessary.
