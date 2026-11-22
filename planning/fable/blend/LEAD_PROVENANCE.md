This plan was produced in four stages. Each left artifacts under `planning/` in the repository; this section says
what was done, and Appendix K gives the counts and what each review changed.

1. **Study of the exemplars.** The plan documents named by the owner were read: FrankenSim's main plan and its four
   companion plans, FrankenTextiles, FrankenMining, FrankenThreeD, FrankenLean and FrankenSymPy. Their structural
   devices are reused on purpose: evidence labels and the definition of "implemented"; claims that are never
   conflated; ambition tags and the layer discipline; kill gates with fallbacks named in advance; a first
   implementation session and a definition of done; an idea-pass appendix.
2. **Audits of the foundations.** Seven code audits reported what the sibling projects contain in code as opposed
   to in plans (FrankenSim's workspace, multibody, contact and GPU policy; FrankenTextiles; FrankenMining;
   FrankenThreeD and the adjacent 3D projects; FrankenLean with FrankenSymPy; the numeric siblings and the MCP
   library). The load-bearing facts were then checked directly in source, and a public-source pass covered GPU
   simulators, Rust-to-GPU routes, geometric robotics, scene assets and benchmarks. Later stages re-read sibling
   code many times; every sibling fact in this plan is stated as FACT with a path, or as FACT-to-recheck where it
   was reported and not re-read.
3. **Version 0.2 and the ambition pass.** A first complete plan (3,666 lines) was written with an idea pass of
   thirty ideas winnowed to fifteen. The owner then asked for a far more ambitious plan and a ten-fold advance
   over every existing system. Five exercises followed, each run to its own method:
   - *An analysis by distinct modes of reasoning.* A neutral case file with an evidence ledger; five independent
     passes (an Amdahl accounting of where design time goes, an adversarial audit of what incumbents already do, a
     transfer of mechanisms from other fields with their breakpoints, hardware envelopes, and a backcast with a
     pre-mortem); 94 atomic claims; independent audit of 76 of them against primary sources and by recomputation.
   - *An adversarial idea duel.* Three agents of different model families each proposed thirty ideas for the plan,
     scored the others' ideas, and reacted to the scores they received, including steelman and blind-spot rounds.
   - *A hypothesis session in the style of Brenner.* A question of record with a falsifier; twenty-four hypotheses,
     two of them deliberate third alternatives; four investigators working refuter-first; a verdict register; then
     a challenge round in which agents of two other model families attacked the register, and an adjudication
     that changed sixteen of its rows.
   - *A harvest of neglected computer-science primitives* against thirteen anticipated hot spots, with the rule
     that every hot spot is anticipated until the first profile.
   - *A compilation of eleven decision problems* (promotion, allocation across evaluation rungs, rank fidelity,
     rare failures, the next experiment, attribution, calibration, solver effort, scheduling, scene selection,
     kernel rewriting) into mathematical families with runtime artifacts, proof obligations and fallbacks.
4. **The blend.** Two other complete plans of this project, written independently
   (`COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__ASTRA.md` and `COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md`),
   were digested in full and compared with version 0.2 without sparing it: where each was better, where version
   0.2 was better, where they conflict, and which of their claims should not be inherited. A catalog merged 112
   distinct ideas from every exercise, with the calculations that killed some of them. An integration charter
   fixed names, tiers and sixteen binding decisions; this document was then written chapter by chapter against
   that charter. The owner's instruction for the blend was maximalist: include every sound mechanism, at the tier
   its evidence supports. The evidence of stage 3 decides the tier (§0.8).
5. **Review of the blend.** A mathematical review of the spine and the physics chapters; a coherence review across
   all chapters; two completeness audits against the two other plans and the idea catalog; a reconciliation pass;
   and a final audit of the two other plans' latest revisions, which changed while this document was being
   written. Appendix K lists the findings and the revisions integrated.

What has **not** happened: no FrankenRobotics code exists; nothing in this plan was compiled, run or measured as
FrankenRobotics; no incumbent simulator was run by this program (every statement about one is a documentation
finding at a pinned version, or a published figure); the external review rounds the planning method calls for have
not been run on this version; and the plan has not been converted to beads. Paper-scale pilots were run at plan
time by one of the two other plans' authors; they are recorded in Appendix K and in §25, and none of them used
FrankenSim dynamics, contact or a real sensor pipeline.
