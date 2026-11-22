# Brenner Step-0: Question of Record (`Q-001`)

**Session ID:** `RS-20261121-franken-robotics-10x`  
**Mode:** `design-space-breakthrough` (hybrid BrennerBot + Dueling Idea Wizards + Modes of Reasoning)  
**Target:** `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md`

---

## Question of Record (`Q-001`)

> **What concrete mathematical, compiler, GPU-hardware, world-simulation, and AI-agent-studio architectural leaps transform `franken_robotics` into a true 10× paradigm break over MuJoCo/MJX, Isaac Lab, Drake, and Genesis — rather than a cleaner, faster 2×–3× reimplementation of the same "edit XML → run black-box RL → inspect scalar reward" loop — while staying strictly executable, formally grounded, and buildable in pure Rust on the FrankenSuite + M5 Ultra (256 GiB Metal 4) + Dual RTX 4090?**

## Paradox (`◊ Paradox-Hunt` — Mandatory Phase 1 Invariant)

Three sharp, well-attested paradoxes govern robotics engineering today:

1. **Paradox 1 (The Algebra-vs-Speed Inversion):** Conventional wisdom assumes high-level abstract mathematics (Lie theory, operads/category theory, formal Lean proofs, symbolic computer algebra, algebraic topology, representation theory) adds *overhead* and slows down simulation compared to raw C++/CUDA floating-point arrays (`franken_threed` §7.2 explicitly warns against runtime Clifford overhead). **Yet** in robotics, 80% of wasted compute is *semantic waste*: redundant flops on canonical joint axes, learning identical limb behaviors $|\Gamma|\times$ redundantly, exploring kinematically/thermally infeasible morphology regions, re-training policies from scratch after small geometric edits, and running tiny timesteps to fight non-symplectic drift. *Resolution hypothesis:* Used at **compile/synthesis/representation time** (never as runtime pointer-chasing!), higher algebra deletes orders of magnitude of search space and flops simultaneously.
2. **Paradox 2 (The Heterogeneous-Morphology GPU Wall):** Existing GPU physics engines (MJX, Isaac Lab) achieve massive throughput ONLY by simulating $B = 16,384$ **identical copies of ONE frozen robot** in one static world. The moment an AI Design Studio wants to evolve 512 **distinct robot morphologies** across 64 **distinct cluttered kitchens/warehouses**, conventional SIMD/SIMT warp divergence and recompilation overhead collapse throughput by 10×–100×. *Resolution hypothesis:* How do we break the single-morphology GPU homogeneity wall algebraically and structurally?
3. **Paradox 3 (The Verification-vs-Contact Chasm):** Formal methods (`Lean 4`, SOS Lyapunov/CBF) work cleanly on smooth low-dimensional polynomial ODEs, whereas real-world indoor/outdoor robots live in high-dimensional, nonsmooth, hybrid impact-friction-perception chaos (greasy kitchen tiles, steam, swinging doors, shifting payloads, mud). *Resolution hypothesis:* How do we bridge exact symbolic/formal certificates and chaotic multi-contact reality without either (a) faking safety claims or (b) confining Lean to trivial toy checks?

## Crisp Falsifier (`✂ Exclusion-Test` — Mandatory)

Any proposed "10× breakthrough" mechanism MUST be **KILLED (`†`)** if it fails any of these four tests:
1. **No executable artifact:** Cannot name the exact Rust crate, data layout, algorithm, or Lean/SymPy theorem it compiles into.
2. **No $\ge 5\times$–$10\times$ measurable lever:** Cannot state a concrete, falsifiable quantitative metric (design iterations/hour, sample complexity, flops/bandwidth, feasible-design discovery rate, sim-to-real failure catch rate, or previously impossible capability) with an explicit baseline comparison and kill gate.
3. **Violates physics/hardware scale (`⊞ Scale-Check`):** Exceeds M5 Ultra (256 GiB unified / ~800 GB/s) or Dual RTX 4090 (2× 24 GiB / 2 TB/s, PCIe Gen4) memory/compute/compile envelopes.
4. **Conflates claim axes:** Claims a formal proof certifies unmodeled real-world friction/perception, or claims novelty without showing why MuJoCo/Isaac/Drake/Genesis structurally cannot do it.
