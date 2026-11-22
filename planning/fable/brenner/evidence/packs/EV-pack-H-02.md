# EV-pack H-02 — universal morphology-conditioned policy prior

Session `RS-20261121-frob-fable-10x`. Sources read 2026-11-21. FETCHED = read this session; MEMORY = not re-read.

## Claim under test

A pretrained, design-conditioned policy walks unseen in-distribution designs zero-shot at 70% of specialist return and reaches specialist return with 10% of from-scratch steps (falsifier: below 40%, or above 50% of steps).

## Evidence against (excerpts)

- Out of distribution it fails outright. "the policy is not able to do any initial transfer to the Unitree H1 or the Talos." and "zero-shot transfer to embodiments that are completely out of the training distribution is still problematic." URMA, https://arxiv.org/html/2409.06366v4 [FETCHED]
- Topology-diverse space, 100 training morphologies: "three MetaMorph policies score 4030.6 on 80 familiar robots"; held-out robots score 1489.6 (37% of the familiar score). GaugeBench, https://arxiv.org/abs/2610.07597 [FETCHED]
- Same space, variable terrain, one test morphology: zero-shot 136.82 against 1406.58 from scratch (about 10%). "On the flat terrain tasks, learning from scratch is comparable to end-to-end finetuning". https://arxiv.org/html/2508.03660 [FETCHED]
- Fine-tuning gain in that space is 2–3×, not 10×: "2 to 3 times higher sample efficiency compared to training from scratch." MetaMorph, https://ar5iv.labs.arxiv.org/html/2203.11931 [FETCHED]
- Cost of the strong results. "Training all experts takes approximately 5 days on 160 NVIDIA RTX 4090/3090 GPUs" (about 19,200 GPU-hours by my arithmetic; roughly 400 days on two RTX 4090), flat terrain only, with "body mass distribution, joint damping, and actuation type, are held fixed". https://arxiv.org/html/2505.05753v2 [FETCHED]. "LocoFormer takes 500 times more compute for 100000 more robots" and "it fails in the first trial." https://arxiv.org/html/2509.23745v1 [FETCHED]
- No source found covers tracked robots.

## Evidence for (excerpts)

- "it scales to twelve distinct embodiments, reaching about 85% of specialist performance." (humanoids randomized around one template). XHugWBC, https://arxiv.org/html/2602.05791v3 [FETCHED]
- "72.64% on unseen robot designs." UniLegs, https://arxiv.org/abs/2507.22653v2 [FETCHED, abstract only]
- "the universal controller impressively obtains up to 80% of the performance of individually trained controllers." N-LIMB, https://ar5iv.labs.arxiv.org/html/2207.11773 [FETCHED]
- LocoFormer, 10 unseen robots including two wheeled: zero-shot 0.96 against expert 0.99 (table, normalized displacement). [FETCHED]
- Fine-tuning within humanoids: 500–5,000 epochs against 50,000 from scratch (1–10%). H-Zero Table III, https://arxiv.org/html/2512.00971 [FETCHED]. URMA: "fine-tune the policies for 20 million steps" against 100 million per robot (20%). [FETCHED]
- Cheap end: "100 million steps per robot (1.6 billion steps in total) takes around 2.5 days" on one GPU (URMA, 16 robots); MetaMorph "only needs 1.5 GPU days". [FETCHED]
- Similarity scaling: "the scaled transfer exhibits no loss of performance on dynamically similar contexts." Pendulum and HalfCheetah only. https://www.arxiv.org/abs/2510.08768 [FETCHED]

## Kill tests

- Artifact, lever: pass.
- Scale: passes at 16–100 training embodiments (36–60 GPU-hours). The results that reach 85–97% used compute far outside two RTX 4090.
- Foundation: needs heterogeneous batches (H-03); fallback is per-class padding.
- Conflation: almost all numbers are simulation returns.
- Rival: partial kill. Multi-embodiment policies already train on Isaac and MuJoCo stacks, so the prior is no engine advantage. It is a lever only against per-design PPO.

## Cheapest discriminating test

Train a URMA-style prior on 200 generated designs (quadruped, biped, wheeled). Hold out 20 parametric and 20 single-limb-edit designs and train a specialist for each. Nearest alternative: the nearest parent's policy applied zero-shot (H-21). Kill: median zero-shot below 40% of specialist on parametric held-outs, or fine-tuning above 50% of steps. Cost: about 60 GPU-hours pretraining plus about 40 for specialists.

## Recommended state and the claim that survives

NARROWED. Within one morphology class and a parametric neighbourhood of trained templates: zero-shot 70–85% of specialist; fine-tuning at 1–20% of from-scratch steps. Across topology-diverse designs at an affordable pretraining scale: zero-shot 10–40%; fine-tuning 2–3× at best and no gain on flat terrain. The "10% of steps" figure survives only inside a class.

## Open points

- H-Zero's ratio uses a fixed 50,000-epoch scratch budget, so it may overstate the saving.
- LocoFormer's zero-shot includes in-context adaptation over trials.
- Dynamic-similarity normalization is untested on legged robots.
- Excerpts were pulled through a fetch tool; table values need a hand check before becoming load-bearing.
