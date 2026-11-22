# Work packages (§44), verbatim rows. Use `WP-nn` as `parent`.

| WP | Name | Crates | Milestone | Key gates | Depends on |
|---|---|---|---|---|---|
| WP-01 | Bootstrap and stack: the one toolchain T and the stack lock | workspace, `xtask`, `frob-core` | M0 | INT-TOOLCHAIN-01; GATE-WS-01, -04 | — |
| WP-02 | Airlock and referee smoke; one asupersync version across the consumed siblings (upstream work item FD-ASUP-01) | `frob-fsim`; upstream in frankensim, frankentorch and fastmcp_rust | M0 | INT-FSIM-01, INT-ASUP-01, INT-FSQL-01 | WP-01 |
| WP-03 | Marrow core | `frob-lie`, `frob-algebra` | M0, M1 | GATE-MARROW-01, -02 | WP-02 |
| WP-04 | Spark spikes | the kernel dialect, `spark/frob-spark-cuda`, `spark/frob-spark-metal`, the backend trait | M0 | INT-CUDA-01, INT-FT-01, -02; GATE-SPARK-00, -01 | WP-01 |
| WP-05 | Derivation spike | `frob-kir`, `derive/frob-derive`, `frob-lemma` | M0 | INT-FSYM-01, -02; GATE-LEMMA-00 | WP-03 |
| WP-06 | Verb registry and surfaces; fastmcp_rust brought onto T (upstream work item FD-FMCP-01), with `frob-mcp` in the core graph running verbs in process | `frob-verbs`, `frob-cli`, `surface/frob-mcp`; upstream in fastmcp_rust | M0, M1 | INT-FMCP-01; GATE-TETHER-02, -03, -07 | WP-01 |
| WP-07 | Design language and compiler | `frob-design`, `frob-model` | M1 | GATE-SKEL-02, -03, -05, -08 | WP-03 |
| WP-08 | Import and export | `frob-io` | M1 | GATE-SKEL-01 | WP-07 |
| WP-09 | Actuators | `frob-actuate` | M1 | GATE-SINEW-01, -02, -05 to -08 | WP-07 |
| WP-10 | Referee tier | `frob-dyn`, `frob-contact` | M1 | GATE-KIN-01, -02; GATE-TRAC-01, -02 | WP-02, WP-09 |
| WP-11 | Static analyses I | `frob-analyze` | M1 | GATE-ATEL-02 | WP-07, WP-09 |
| WP-12 | Reflex I: invariant filter and baseline controllers | `frob-control` | M1 | GATE-REFLEX-01, -03 | WP-10 |
| WP-13 | Chronicle I: telemetry, energy ledger, explain | `frob-chronicle` | M1 | GATE-CHRON-03 | WP-10 |
| WP-14 | Gauntlet harness and oracles | `frob-conformance`, `tools/oracle` | M1 | GATE-GAUNT-01 | WP-02 |
| WP-15 | Kernel library and batched step | `frob-kernels`, `frob-dyn` | M2 | GATE-KIN-03, -05, -07 | WP-05, WP-07, WP-09 |
| WP-16 | Traction, batched | `frob-contact` | M2 | GATE-TRAC-01, -03 | WP-15, WP-17 |
| WP-17 | Scene packs and terrain | `frob-scene` | M2 | GATE-HAB-01, -13 | WP-02 |
| WP-18 | CUDA backend | `spark/frob-spark-cuda` | M2 | GATE-SPARK-02, -03, -06 | WP-04, WP-15 |
| WP-19 | Metal backend | `spark/frob-spark-metal` | M2 | GATE-SPARK-02, -04, -06 | WP-04, WP-15 |
| WP-20 | Sensors I | `frob-sense` | M2 | GATE-SENS-01, -02, -03 | WP-16 |
| WP-21 | Cortex I: `VecEnv` and PPO on device | `frob-learn` | M2 | GATE-CORTEX-01, -02, -05 | WP-18, WP-19, WP-20 |
| WP-22 | Trust ladder and replay | `frob-dyn`, `frob-chronicle` | M2 | GATE-FID-01; GATE-CHRON-01, -02; GATE-SPARK-05 | WP-13, WP-16 |
| WP-23 | Interiors: seven archetypes and hazard lists | `frob-scene` | M3 | GATE-HAB-02 | WP-17 |
| WP-24 | Arena: task families, scorecards, the design corpus and benchmark | `frob-arena` | M3 | GATE-ARENA-01, -02, -07, -09 | WP-21, WP-23 |
| WP-25 | Atelier: compare, explain, promote, detect | `frob-opt`, `frob-learn` | M3 | GATE-ATEL-01, -04; GATE-FID-02; GATE-CORTEX-04 | WP-22, WP-24 |
| WP-26 | Tether, full: slot dispatch, capabilities, the full MCP surface | `frob-cli`, `frob-verbs`, `surface/frob-mcp` | M3 | GATE-TETHER-01, -09 to -12 | WP-06, WP-25 |
| WP-27 | Reflex II: baselines for every archetype, the sampling controller, skill artifacts [S], [F] | `frob-control` | M3, M4 | GATE-REFLEX-02, -04, -05, -06, -08 | WP-12, WP-15 |
| WP-28 | Optimization, lineage, experiment suggestion and the surface A/B [S], [F] | `frob-opt` | M4 | NS-06; GATE-ATEL-03, -15; GATE-TETHER-13 | WP-25 |
| WP-29 | Symmetry: equivariant policies and symmetric controllers [S], [F] | `frob-algebra`, `frob-analyze`, `frob-learn` | M4 | GATE-CORTEX-03; GATE-MARROW-04; GATE-REFLEX-07 | WP-11, WP-21 |
| WP-30 | Wheels, tracks, hybrids, moving surfaces, soil [S], [F] | `frob-contact`, `frob-analyze` | M4 | GATE-TRAC-08; known answers of §18.16 | WP-16, WP-11 |
| WP-31 | Lemma: certificate coverage and Lean capsules [F] | `frob-lemma`, `lean/`, `derive/frob-derive` | M5 | INT-FLN-01, -02; GATE-LEMMA-01, -02; GATE-QUAL-03 | WP-05, WP-15 |
| WP-32 | Real-place ingestion [F] | `frob-scene`, `frob-conformance`, `frob-analyze` | M5 | GATE-HAB-03 | WP-23, WP-22 |
| WP-33 | Frontier epics not named below | — | M6 | Per-epic plans | — |
| WP-34 | Incumbent phase ledger and parity baseline | `frob-conformance`, `tools/oracle` | M0, M1 | GATE-SPARK-21 | WP-01 |
| WP-35 | Hardware fingerprint; exact and guarded kernels | `frob-kir`, `frob-kernels`, the Spark annexes | M0 | GATE-SPARK-07, -08 | WP-04 |
| WP-36 | Claim product, flags and the offline auditor | `frob-core`, `frob-cert` (binary `frob-verify`) | M0, M1 | GATE-LEMMA-06, -03; GATE-WS-03; GATE-SAFE-01, -08 | WP-01 |
| WP-37 | Worker fallback, file-by-file admission tests, the plan-time pilots | `frob-fsim` (binary `frob-fsim-worker`), `worker/`; `frob-units` only if forced | M0 | INT-FSIM-02 to -11, INT-FSIM-18; INT-FNP-01, -02 | WP-02 |
| WP-38 | Modules, archetypes and preflight | `frob-design`, `frob-model`, `frob-analyze`, `frob-algebra` | M1, M3 | GATE-SKEL-03, -04, -06, -07; GATE-MARROW-03 | WP-07, WP-11 |
| WP-39 | Transactional referee step | `frob-dyn`, `frob-contact`, `frob-fsim` | M1 | GATE-KIN-04; GATE-TRAC-06 | WP-10 |
| WP-40 | Clock contract | `frob-sense` | M1, M2 | GATE-SENS-04, -05 | WP-10 |
| WP-41 | Requirement language, fixtures and the two study types | `frob-spec`, `frob-arena` | M1 | GATE-ARENA-03, -04, -05; GATE-ATEL-06 | WP-10, WP-13 |
| WP-42 | First mission slice [F] | `frob-mission`, `frob-spec`, `frob-lemma`, `frob-cert` | M1 | GATE-AUGUR-01 to -07; GATE-LEMMA-04 | WP-36, WP-41 |
| WP-43 | Chunk store, job protocol, job engine and artifact graph | `frob-chronicle`, `frob-verbs` | M1, M2 | GATE-CHRON-04, -10, -11; INT-ASUP-02; GATE-TETHER-04, -05, -06, -08 | WP-13, WP-06 |
| WP-44 | Phylum: variants, buckets, cohorts and the head-to-head [S], [F] | `frob-phylum`, `frob-kernels` | M2 | GATE-PHYLUM-02, -03, -04; GATE-KIN-06; GATE-SPARK-10 | WP-15, WP-18, WP-19 |
| WP-45 | Contact cone, joint rows and scaling | `frob-contact`, `frob-actuate`, `frob-kernels` | M2 | GATE-TRAC-04, -10; GATE-SINEW-03, -04, -09; GATE-SPARK-18 | WP-16 |
| WP-46 | Determinism, placement and device services; the fleet scheduler and survivor specialization [F] | `frob-kernels`, `frob-kir`, the Spark annexes, `frob-cli` | M2, M4 | GATE-SPARK-09, -11, -12, -15, -16; GATE-MARROW-05; GATE-SPARK-13, -14 | WP-18, WP-19, WP-35 |
| WP-47 | Performance harness and parity gates | `frob-conformance` | M2 | GATE-SPARK-19, -20, -22 | WP-34, WP-18 |
| WP-48 | Gym semantics, the PPO battery and the two-device update | `frob-learn` | M2 | GATE-CORTEX-06, -07, -08, -09 | WP-21 |
| WP-49 | Sensors II and the sensing analyses | `frob-sense`, `frob-analyze` | M2, M3 | GATE-SENS-06, -12, -13; GATE-SENS-07 to -11, -14 | WP-20, WP-40 |
| WP-50 | Fidelity ledger, sampled referee, energy audit, tier correction and model-form checks [S], [F] | `frob-dyn`, `frob-chronicle`, `frob-opt` | M2, M3 | GATE-FID-03 to -08; GATE-CHRON-08, -09; GATE-CORTEX-15 | WP-22, WP-39 |
| WP-51 | Replay, teleport and forks | `frob-chronicle` | M2 | GATE-CHRON-05, -06, -07, -12; GATE-AUTOPSY-01, -02, -07 | WP-22, WP-43 |
| WP-52 | Viewer, stills and dashboard | `viewer/`, `frob-tui`, `frob-chronicle` | M2, M3 | GATE-TETHER-14; GATE-CHRON-13; INT-FTUI-01 | WP-20, WP-43 |
| WP-53 | Habitat II: clearance complexes, retrofit catalogue, world bank, film patches, rigid proxies [S], [F] | `frob-scene`, `frob-contact` | M3, M4 | GATE-HAB-04 to -06, -10, -11, -12, -14; GATE-TRAC-05, -07 | WP-23, WP-16 |
| WP-54 | Blinded qualification lane | `frob-arena`, `frob-verbs` | M3 | GATE-QUAL-01, -02; GATE-SAFE-03; INT-FSIM-14 | WP-24, WP-26 |
| WP-55 | Nemesis [S], [F] | `frob-nemesis` | M3, M4 | GATE-NEMESIS-01 to -08 | WP-24, WP-51 |
| WP-56 | Autopsy | `frob-autopsy` | M3, M4 | GATE-AUTOPSY-03 to -06, -08 | WP-51 |
| WP-57 | Gymnasium-compatible adapter [F] | `bindings/frob-gymnasium`, `frob-learn` | M3 | GATE-CORTEX-16 | WP-48, WP-37 |
| WP-58 | Safety, privacy and supply chain | `frob-verbs`, `frob-io`, `frob-chronicle` | M2, M3 | GATE-SAFE-02, -04, -05, -06, -07, -09, -10; GATE-SPARK-17 | WP-06, WP-43 |
| WP-59 | Mission queries II: amendments, transport, separation, probes [F] | `frob-mission`, `frob-opt` | M3 | GATE-AUGUR-06, -08 to -13 | WP-42, WP-55 |
| WP-60 | The rank-agreement pilot and the partial ten-fold gate | `frob-conformance` | M3 | GATE-CORTEX-12 (pilot); GATE-TENFOLD-03 to -06, -10, -11, -13 | WP-34, WP-54, WP-57, WP-59 |
| WP-61 | Family policies, inheritance and distillation [F] | `frob-learn` | M4 | GATE-CORTEX-11, -12, -13, -14 | WP-44, WP-48, WP-60 |
| WP-62 | The ladder, the cascade and the decision cores [S], [F] | `frob-opt`, `frob-verbs` | M3, M4 | GATE-ATEL-07 to -10, -13, -14, -16; GATE-PHYLUM-08 | WP-28, WP-50, WP-61 |
| WP-63 | Revalidation, the memo graph and quotienting | `frob-verbs`, `frob-chronicle`, `frob-phylum` | M3, M4 | GATE-ATEL-11, -12; GATE-PHYLUM-01, -05, -06, -07; INT-FSIM-16; INT-FSQL-02 | WP-43, WP-44 |
| WP-64 | Gait orbits with continuation [F] | `frob-gait` | M4 | GATE-GAIT-01, -02, -03; INT-FSIM-13 | WP-27, WP-39 |
| WP-65 | Energy-budget tier and runtime shields [F] | `frob-shield`, `frob-control` | M4 | GATE-SHIELD-01 to -04 | WP-27, WP-13 |
| WP-66 | Mission queries at scale and region certificates [F] | `frob-mission`, `frob-cert`, `frob-lemma` | M4 | GATE-AUGUR-14, -15, -16, -19; GATE-LEMMA-04 | WP-59, WP-44 |
| WP-67 | Synthesis lanes: co-design, trajectory optimization, bounded exact linkages [F] | `frob-synth`, `derive/frob-derive` | M4 | GATE-AUGUR-17, -18 | WP-66, WP-05 |
| WP-68 | Ground: admission, clocks, identification, frozen predictions, bands, the kit [F] | `frob-ground` | M1 (the reality line), M5 | GATE-GROUND-01 to -09; GATE-GROUND-10 when data is admitted | WP-13, WP-41 |
| WP-69 | Bench packages A to F of §30.17, optional imports | — | None; sourcing may start at M0 | GATE-GROUND-10 | WP-68 |
| WP-70 | Foundry I: loads, sizing and the structure loop [F] | `frob-foundry` | M5 | GATE-FOUNDRY-01 to -06; INT-FSIM-15 | WP-11, WP-22 |
| WP-71 | Foundry II: manufacturability, tolerance and the build packet [F] | `frob-foundry`, `frob-cert` | M5 | GATE-FOUNDRY-07 to -10; GATE-WS-02 | WP-70, WP-54 |
| WP-72 | Reflection pilot, variational integrator, hard-contact batch law, the tree-operator pilot [F] | `frob-lemma`, `lean/`, `frob-dyn`, `frob-contact` | M5 | GATE-LEMMA-05; GATE-TRAC-09 | WP-31, WP-39, WP-45 |
| WP-73 | Living worlds II: advected films, slosh, articulated proxies; coupled islands [F], [M] | `frob-scene`, `frob-contact`, `frob-fsim` | M6 | GATE-HAB-07, -08, -09; INT-FSIM-17 | WP-53 |
| WP-74 | Frontier physics and sensing: soil at scale, the tree-operator route, meshes on devices, tactile sensing, the rotor port, RGB and splat scenes, moving people, gradients through contact [F], [M] | `frob-contact`, `frob-dyn`, `frob-sense`, `frob-actuate`, `frob-scene`, `frob-mission`, `viewer/` | M6 | GATE-AUGUR-20, -21; per-epic known answers | WP-30, WP-49 |
| WP-75 | Lifecycle compression [M] | `frob-lifecycle` | M6 | GATE-FOUNDRY-12 | WP-70 |
| WP-76 | Sealed suite, rival arms and the freeze | `frob-conformance` | M7 | GATE-TENFOLD-03, -04, -05, -11, -12 | WP-60, WP-62, WP-66 |
| WP-77 | Tournament, adjudication and publication | `frob-conformance`, `docs/` | M7 | GATE-TENFOLD-07, -08, -13; NS-24 | WP-76 |
| WP-78 | Facility co-design and multi-robot handoff [M] | `frob-mission`, `frob-scene` | M8 | GATE-AUGUR-24 | WP-66, WP-53 |
| WP-79 | Learned transport and tree policies: skill transport with learned residuals, geometric-algebra tree policies, reachable-set tables [M] | `frob-control`, `frob-learn`, `frob-shield`, `frob-mission` | M8 | GATE-REFLEX-09; GATE-AUGUR-23; GATE-CORTEX-17; GATE-SHIELD-05 | WP-61, WP-65 |
| WP-80 | Continuation-based synthesis, firmware export and the fused step kernel [M] | `frob-synth`, `frob-foundry`, `frob-kernels` | M8 | GATE-AUGUR-22; GATE-FOUNDRY-11 | WP-67, WP-71, WP-15 |
| WP | What exceeds the default | Class asked | If not approved |
|---|---|---|---|
| WP-34, WP-47 | Incumbent and parity measurements on both cards and on all cores | E1 per gate | Single-card and eight-core rows only; the other rows are recorded `not run` |
| WP-21, WP-48 | Training to threshold on two cards; the scaling rows | E1 | One card; GATE-SPARK-20 is recorded `not run` |
| WP-44 | The head-to-head on 512 topologies with four arms (OPEN-PHYLUM-3) | E1, repeated per arm | KILL-PHYLUM is decided on the pilot |
| WP-53 | The world bank resident on the M5 Ultra, far above 16 GiB | E1, memory to the device budget of §36.10 | A bank that fits the default; GATE-HAB-12 reports the smaller count |
| WP-55 | A certified failure rate of 10⁻⁶ (OPEN-NEMESIS-3) | E2 | Certify to 10⁻⁵; report 10⁻⁶ as `Estimated` |
| WP-60 | The rank-agreement pilot (OPEN-PROG-4) and the partial gate's campaigns | E2 | Cheap rungs are `uninformative` and neither select nor discard; the partial gate runs its CPU strata |
| WP-61 | Pretraining of one family policy (OPEN-CORTEX-1) | E3, approved per run with its ceiling and kill rule written first | The variant of M4 with no family policy |
| WP-62, WP-28 | Design campaigns for the cascade and the surface A/B | E2 per campaign | Campaigns sized to the default, with fewer briefs and wider intervals |
| WP-76, WP-77 | The tournament: 24 mission families per cell by default, and at least 32 for a cell that declares a population result (§39.7, §39.20), on both machines | E2 per campaign, and E3 where an arm pretrains | Fewer strata, each declared `not run` with the reason |
| WP-79 | Pretraining for tree policies [M] | E3 | The epic is not funded |
|---|---|
