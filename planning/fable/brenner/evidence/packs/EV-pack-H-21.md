# EV-pack H-21 — policy inheritance along design edits

Session `RS-20261121-frob-fable-10x`. Sources read 2026-11-21. FETCHED = read this session; MEMORY = not re-read.

## Claim under test

Fine-tuning a child design's policy from its parent's (or a shared policy) reaches specialist return with at most 20% of from-scratch steps for parameter edits and at most 50% for single-limb topology edits. Falsifier: above 60% of steps, or worse optima than fresh policies in more than 20% of edits.

## Evidence against (excerpts)

- Chained inheritance is continual learning, and PPO degrades under it. "The standard PPO learning algorithm fails catastrophically on the non-stationary ant problem." Even without change: "the average performance of PPO increased for about 3 million steps but then collapsed." Dohare et al., Nature 2024, https://pmc.ncbi.nlm.nih.gov/articles/PMC11338828/ [FETCHED]
- Inheritance can end at a worse optimum. "On the flat terrain tasks, learning from scratch is comparable to end-to-end finetuning"; on one flat-terrain test morphology scratch scored 2191.50 against 1635.82 for full fine-tuning (table). https://arxiv.org/html/2508.03660 [FETCHED]
- Topology-diverse transfer gives 33–50% of steps, not 20%: "2 to 3 times higher sample efficiency compared to training from scratch." MetaMorph, https://ar5iv.labs.arxiv.org/html/2203.11931 [FETCHED]
- The benefit falls with edit distance and biases the search. Under novelty pressure the Lamarckian advantage "is no longer significant (p=0.136)", because "promoting diversity reduces parent-offspring similarity". https://arxiv.org/html/2604.15854 [FETCHED]. "the morphological diversity of the Lamarckian system declines at a more rapid rate". https://arxiv.org/html/2309.13099 [FETCHED]
- An inherited controller "favors morphologies with first-mover advantage in control". https://arxiv.org/html/2608.23100 [FETCHED; the paper cites prior work, it ran no inheritance experiment]

## Evidence for (excerpts)

- Similar bodies (humanoid to humanoid), fine-tune epochs against 50,000 from scratch: Adam 2,000; T1 500; H1-2 5,000; AGo 500, at equal return (Table III). "pretrained policies can adapt to novel embodiments within hundreds of training epochs." H-Zero, https://arxiv.org/html/2512.00971 [FETCHED]
- "fine-tune the policies for 20 million steps only on the Silver Badger itself", against 100 million steps per robot from scratch. URMA, https://arxiv.org/html/2409.06366v4 [FETCHED]
- "the Lamarckian system reaches the top fitness levels of the Darwinian system with just half of the effort", and "the most similar the offspring is to the parent, the higher the fitness of the offspring." [FETCHED, 2309.13099]
- "fine-tuned generalist rollouts show an approximate 10% improvement in return compared to specialist ones." XHugWBC, https://arxiv.org/html/2602.05791v3 [FETCHED]

## Kill tests

- Artifact, lever, scale: pass. The cost is ordinary PPO.
- Foundation: needs a token-per-link policy and an edit-to-input map; fallback is inheriting from the shared prior (H-02).
- Conflation: all numbers are simulation returns.
- Rival: partial kill. Resuming a checkpoint is available in every incumbent training stack. Inheritance saves compute against per-design PPO, but it is no engine advantage.

## Cheapest discriminating test

Edit corpus on three base robots: 10 parameter edits (10–30% on lengths and masses) and 5 single-limb topology edits. Three arms, three seeds each: from scratch, inherit from parent, inherit from shared prior. Measure steps to 95% of the from-scratch final return and the final-return gap. Add one chain of 10 successive edits to expose plasticity loss. Kill: median parameter-edit cost above 60% of steps, or the inherited arm more than 5% worse in over 20% of edits. Cost: about 15 children × 9 runs × 0.5 GPU-hours, roughly 70 GPU-hours. The 0.5 GPU-hour figure is my assumption for flat-terrain PPO on one RTX 4090.

## Recommended state and the claim that survives

NARROWED.
- Parameter edits within one topology: 1–20% of from-scratch steps. The stated 20% bound survives.
- Single-limb topology edits: 33–100% of steps. The 50% bound is unsupported, and on flat terrain the evidence shows no gain.
- Chains of edits need a plasticity guard: inherit from the shared prior or reset periodically, not parent to child without limit.

## Open points

- H-Zero's 10–100× ratio uses a fixed 50,000-epoch scratch budget; scratch runs reached similar return at 20,000–50,000 epochs, so the honest ratio may be several times smaller.
- No source measures the fraction of edits that end at a worse optimum; the falsifier's 20% threshold is untested.
- Inheritance favours bodies close to the parent, which conflicts with any diversity objective in H-23.
- Excerpts were pulled through a fetch tool; table values need a hand check.
