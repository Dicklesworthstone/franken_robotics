# Phase 2 brief (read after INTEGRATION_CHARTER.md)

Phase 1 is finished: chapters §1–§37 exist as thirteen part files, 22,900 lines in total. Phase 2 writes the
chapters and appendices that depend on them. The charter's style rules, names and binding decisions still apply
in full (no attribution to any plan or model, specification register, labels, tiers, Unicode mathematics, prose
wrapped near 118 characters, a `<!-- LEDGER -->` block at the end of your file).

## 1. Where the Phase 1 text is

Directory: `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/parts/`. The part files are too large to
read whole (1.9 MB). **Read the inventories in full, then `grep` and read targeted line ranges of the parts.**

| § | Chapter | File | Starts at line |
|---|---|---|---|
| header, Part 0 | Document control | P00.md | 1 |
| 1–8 | Thesis: what exists; ten-fold thesis; principles; bets; readers; goals; North-Star; claims | P00.md | 351, 641, 834, 885, 949, 967, 1080, 1226 |
| 9–12 | Doctrine; constructions C-01–C-40; conventions; Lemma | P01.md | 1, 81, 1776, 1961 |
| 13–15 | Marrow; Skeleton; Sinew | P02.md | 1, 414, 1110 |
| 16–18 | Kinesis; Phylum; Traction | P03.md | 1, 648, 1209 |
| 19–20 | Habitat; Sensorium | P04.md | 1, 1250 |
| 21, 35, 36 | Spark; numerics and determinism; performance program | P05.md | 1, 844, 1241 |
| 22–23 | Reflex; Cortex | P06.md | 1, 850 |
| 25 | Augur | P07.md | 1 |
| 24, 26, 27 | Arena; Nemesis; Autopsy | P08.md | 1, 785, 1270 |
| 28–29 | Atelier; Chronicle | P09.md | 1, 1227 |
| 30–31 | Ground; Foundry | P10.md | 1, 651 |
| 32, 37 | Tether; safety, security, privacy | P11.md | 1, 894 |
| 33–34 | Suite consumption; workspace, crates, formats | P12.md | 1, 813 |

Inventories in `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/` (generated mechanically from the
parts; identifiers are final):

| File | Contents |
|---|---|
| `LEDGERS.md` | Every part's LEDGER block: identifiers it defined, crates touched, verbs, formats, needs from other parts |
| `GATES_INVENTORY.md` | All 421 gate, kill-gate and admission-gate identifiers with the subsection that defines each and its row text |
| `OPENS_INVENTORY.md` | All 141 open questions with their subsection and text |
| `VERBS_INVENTORY.md` | All 169 `cluster.verb` names with the chapters that mention them |
| `APPENDIX_O_REFUSALS.md` | The refusal-code registry (296 codes), already final text for Appendix O |
| `COLLISIONS.md` | Identifiers defined in more than one part |

Totals in Phase 1: 40 constructions (C-01–C-40), 35 theorem cards (T-01–T-35), 26 analyses (A-01–A-26), 11
decision cores (DC-01–DC-11), 37 bets (BET-01–BET-37), 43 goals (GOAL-01–GOAL-43), 24 North-Star scenarios
(NS-01–NS-24), 13 differentiators (N1–N13), 206 architecture decisions, 190 invariants, 114 proof obligations, 269
convergence gates, 110 kill gates, 33 admission gates, 141 open questions, 296 refusal codes, 13 formats, 169
verbs.

## 2. Reconciliation decisions (binding; they settle what the Phase 1 drafters left open)

- **R1 Duplicate kill gates.** Six kill gates are stated in two chapters. The owner chapter governs the trigger
  and the fallback; the gate register lists each once and names the second chapter as a consumer: KILL-DR (owner
  §29, consumer §16); KILL-RANK (owner §28, consumer §16); KILL-FDGRAD (owner §17, consumer §28); KILL-FILM
  (owner §18, consumer §19); KILL-SLOT (owner §32, consumer §21); KILL-SPECIALIZE (owner §21, consumer §17). The
  register (§40) states: where a chapter and the register differ, the register governs.
- **R2 Certificates.** `frob-cert` has an empty dependency table; `frob-verify` is its binary. `frob-mission`
  emits plain proof objects; `frob-lemma` writes `.frobcert` bundles against the schema and shares no code with
  the reader; `frob-verbs` runs the checker before returning an answer.
- **R3 MCP boundary.** FACT: fastmcp_rust declares `rust-version = "1.100"` on its own nightly; FrankenSim pins a
  1.99 nightly. `surface/frob-mcp` is therefore its own workspace and talks to `frobd` across a process boundary by
  default. Merging it into the core graph is an upstream-dependent improvement, not a plan assumption.
- **R4 Anchored positions.** Cell side 8 m, offset centred in [−4, 4) m.
- **R5 Training-time parity.** GATE-CORTEX-05: wall-clock to the return threshold at most 1.25 times the
  incumbent re-measured on the same card (published context about 410 s on one RTX 4090). Throughput parity
  tolerance: at least 0.67 of the incumbent measured on `trj`; KILL-PARITY below 0.33.
- **R6 Rank agreement.** Top-weighted Kendall τ thresholds 0.7 (use the cheap rung to discard) and 0.5 (below
  this the cheap rung may not discard).
- **R7 Flags.** Exported artifacts carry `execution_authorized`, `hardware_safety_signoff`,
  `sim_to_real_certified`, and for build packets `fabrication_released`; all are always `false` and have no
  setter.
- **R8 Names fixed by Phase 1.** Decision cores are DC-01–DC-11. Step profiles are `lockstep`, `strict`,
  `referee`. The design schema is `frob.design/1` (extension `.fbot`). Exit code 8 means accepted and still
  running. One revalidation engine serves `result.revalidate` and `mission.revalidate`. Crate scope caps total 561
  thousand lines.
- **R9 Budget stops.** A budget stop publishes nothing partial as a result, with one clarification: a mission
  query returns the cells certified so far plus the unresolved remainder, each cell complete in itself.
- **R10 Moonshot cap.** At most four [M] items are active (funded and in progress) at once; the catalogue of
  admitted [M] items is longer and each waits its turn.

## 3. Milestone skeleton (binding for §43, §44, §45, §39 and §40)

| M | Name | What lands | Exit is defined by |
|---|---|---|---|
| M0 | Spine and airlock | Workspace, stack lock, admissions; airlock and referee smoke; Lie core for SO(3), SE(3); one kernel from one source on three backends; device probes; derivation spike and first certificates; verb registry, envelope, CLI skeleton, MCP smoke across the process boundary; the incumbent phase ledger measured on `trj` (parity baseline); the plan-time pilots re-run and recorded | Admission gates; GATE-MARROW-01; GATE-SPARK-00, -01; GATE-LEMMA-00; GATE-TETHER-03; the parity-baseline gate |
| M1 | Referee studio and first mission slice (CPU only) | `.fbot` design language, modules, first archetypes, preflight stages 1–6, import and export; actuator levels L1, L2; the transactional referee step; static analyses; invariant filter and baseline controllers; clock contract; telemetry T0–T2, energy ledger, explain; requirement language core; the cart docking fixture; the first mission queries on static conditions with `frob-cert` and `frob-verify`; both study types on the CPU; lineage basics; M1 verbs on CLI and MCP | NS-01, NS-07 and the docking scenario demonstrated; the program gate on schedule |
| M2 | Batch, backends, families | The batched step on `batch-cpu`, CUDA and Metal; scene packs and terrain; contact law CL-1 with the four-dimensional cone and joint rows; proprioception, height scans, depth, lidar, camera timing; `VecEnv` with the corrected semantics; PPO on device; per-world parameter variants and model variants (Phylum) with the mixed-batch head-to-head experiment; fidelity and divergence ledgers; replay, teleport, forks; the sampled referee; throughput and training parity gates | Kill decisions for the kernel toolchain, contact, Metal and mixed batches recorded as ADRs |
| M3 | Places, tasks, adversaries, qualification | Seven interior archetypes with hazard lists; clearance complexes; Arena task families and requirement verdicts; the blinded qualification lane; Nemesis; Autopsy; promotion and exploitation detectors; the daemon with slot dispatch, capabilities and the full MCP surface; the design corpus; baseline controllers for every archetype; the sampling controller as fallback; **the partial ten-fold gate** on legged and wheeled briefs | Partial ten-fold gate recorded, whatever the ratio |
| M4 | Amortized control, co-design, mission queries at scale | Family policies, inheritance, the rank-agreement ladder; equivariance under the observation-preserving subgroup; gait orbits with continuation; optimization, racing, quality-diversity archive, bisect, predictions; the co-design cascade; mission queries over families on the GPU; region certificates; trajectory optimization with design variables; bounded exact linkage synthesis; tyres, tracks, hybrids, moving surfaces; the retrofit catalog; the energy-budget tier and shields; experiment suggestion; the agent-surface A/B | Rank agreement measured; family-policy bets decided |
| M5 | Proofs, reality, manufacture | Certificate coverage, Lean capsules, the reflection pilot; the variational integrator; Ground (procedure, bands, replication kit); real-place ingestion; Foundry (structure loop, tolerances, build packet); the hard-contact batch law | Reality lane has a record or a recorded no-claim |
| M6 | Frontier physics | Robot-advected films, slosh, cable and rug proxies; then rods, cloth and coupled islands; soft soil at scale; tactile sensing; the rotor port; RGB and splat scenes; moving people; gradients through contact; lifecycle compression | Per-epic plans and kill gates |
| M7 | The ten-fold tournament | The sealed suite, the incumbent arms, neutral adjudication, publication of the result | The measured ratio is published with its phase ledger |
| M8 | Frontier delivery | Facility co-design and multi-robot handoff; skill transport with learned residuals; continuation-based synthesis; firmware export; geometric-algebra tree policies at scale | Per-epic plans and stop rules |

## 4. Appendices (final letters)

A Glossary · B Idea passes · C EV matrix and killed-idea register · D Mathematics compendium · E Worked cards ·
F Upstream asks · G Robots, scenes, licences, hazard lists · H Source register · I Type sketches · J Verb
registry · K Review log (written by the lead) · L Worked artifacts · M Known-answer suite · N Counterexample
catalogue · O Refusal code registry (already generated).

## 5. Phase 2 part files

Write to `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/parts2/<name>.md`. Start each chapter with
its `## N. Title` heading and each appendix with `## Appendix X. Title`.
