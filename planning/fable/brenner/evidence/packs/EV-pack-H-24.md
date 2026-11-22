# EV-pack H-24 — geometric-algebra, tree-structured policy

Session `RS-20261121-frob-fable-10x`. Sources read 2026-11-21. FETCHED = read this session; MEMORY = not re-read.

## Claim under test

A policy with link tokens carrying multivector features, equivariant to rigid motions and relabelling, with message passing along the kinematic tree, beats a plain morphology transformer at equal parameters: at least 1.3× sample efficiency in pretraining and higher zero-shot return. Falsifier: no better on held-out designs at equal compute.

## Evidence against (excerpts)

- No published geometric-algebra policy for multi-embodiment locomotion was found; the claim is an extrapolation.
- Pure geometric-algebra networks in the control loop were slow. "Policies using P-GATr as the denoising network fail across all tasks due to extremely slow convergence." Epoch time 23.49 s against 3.12 s for a U-Net (table). https://arxiv.org/html/2507.05695v4 [FETCHED]
- Fixed tree message passing has negative evidence. "removing morphological information does not harm the performance of these models." A plain transformer "substantially outperforms GNN-based alternatives with fixed message-passing schemes". https://ar5iv.labs.arxiv.org/html/2010.01856 [FETCHED]. Strict body-graph masking "shows relatively inferior results in hard-exploration tasks", and "our BoT is designed for tasks with a single morphology." https://ar5iv.labs.arxiv.org/html/2408.06316 [FETCHED]
- Gravity leaves only a subgroup: "the symmetry is only restrained to the rotations/reflections along the direction of" gravity. In the same paper, "a simple MLP with heading normalization may outperform the benefits brought by equivariance and attention". https://ar5iv.labs.arxiv.org/html/2305.18951 [FETCHED]
- Augmentation recovers the data-efficiency gain: "data augmentation makes a baseline transformer as data-efficient as an equivariant model." https://arxiv.org/html/2410.23179v2 [FETCHED]. For description conventions: retained return "rises from 3.6% to 80.6% when training spans equivalent axis conventions" (as reported by the fetch tool). https://arxiv.org/abs/2610.07597 [FETCHED, abstract]
- Hard constraints cost robustness: "PPOeqic is more vulnerable to distribution shifts compared to methods without hard constraints". Gains were small (4.47% mean success rate). https://arxiv.org/html/2403.17320 [FETCHED]

## Evidence for (excerpts)

- At equal compute, "the equivariant model achieves a loss that is lower by approximately a factor of 2." Rigid-body prediction, not control. [FETCHED, 2410.23179]
- "GATr consistently outperforms both non-geometric and equivariant baselines in terms of error, data efficiency, and scalability." https://arxiv.org/abs/2305.18415 [FETCHED, abstract]
- As encoder and decoder only, in manipulation imitation: baselines "require roughly three times more training epochs to match this level." Epochs are about 1.4× slower, so about 2× in time. [FETCHED, 2507.05695]
- Subequivariant transformer, zero-shot to an unseen morphology: 206.8 against 17.9 for the non-equivariant baseline on one cross-domain task; 161.9 against 152.0 on another (Table 2). [FETCHED, 2305.18951]
- The failure this targets is real: MetaMorph scores "only 51.6 when those same robots are equivalently re-described", and "A new description can therefore be more damaging than a new robot." [FETCHED, 2610.07597]
- Single robot, graph-equivariant actor: "51.8%, 46.6%, and 61.4% lower RMSE-O" than baselines. https://arxiv.org/html/2512.00727v1 [FETCHED]

## Kill tests

- Artifact, lever: pass.
- Scale: passes, but geometric-algebra layers ran 1.4–7.5× slower per epoch, so equal parameters is not equal compute.
- Foundation: needs `fs-ga`; fallback is a plain link-token transformer with canonical inputs.
- Conflation: none.
- Rival: no incumbent offers it. Even if true, 1.3× on an amortized pretraining run is not a source of the ten-fold.

## Cheapest discriminating test

Same embodiment set and wall-clock budget, three seeds each. Arm A: plain link-token transformer with heading normalization and convention augmentation. Arm B: A plus geometric-algebra encoder and decoder. Arm C: full geometric-algebra tree message passing. Measure time to a return threshold, zero-shot return on held-out designs, and retention under re-description. Kill: neither B nor C is at least 1.3× faster than A, nor higher zero-shot beyond seed spread. Cost: about 90 GPU-hours with shortened runs of about 10 GPU-hours each (my estimate).

## Recommended state and the claim that survives

NARROWED. What survives: the policy must be invariant to heading and to description conventions; that failure mode is large and documented. Canonical inputs and augmentation deliver most of it. The geometric-algebra and tree-recursion specifics are unproven for locomotion: expected gain 0–1.3× in reinforcement learning, with up to 2× seen only in supervised dynamics and imitation. Demote to an ablation, with geometric-algebra layers as encoder and decoder only.

## Open points

- Published baselines rarely include augmentation plus heading normalization, so reported equivariance gains are upper bounds.
- Whether an equivariant architecture passes the re-description benchmark is not stated in its abstract.
- Excerpts were pulled through a fetch tool; table values need a hand check.
