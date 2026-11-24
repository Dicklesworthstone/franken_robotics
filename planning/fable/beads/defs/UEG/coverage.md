# Coverage of drafting unit UEG: Appendix E (worked cards) and Appendix G (reference robots, scene assets, licences, hazard lists, archetypes)

Bead files in this directory: 110 (0 seed beads; 41 card beads `GB-01` to `GB-41`; 19 reference-robot beads `ROBOT-<name>`; 18 archetype beads `ZOO-<name>`; 32 new beads `NEG-01` to `NEG-32`, of which NEG-32 was added at integration and is not in the counts that follow). By milestone: M0 8, M1 41, M2 11, M3 37, M4 7, M5 5. Sizes: 84 lines (NEG-07) to 172 lines (NEG-28), 12,063 lines in all.

The unit owns no seed bead. Every `deps`, `parent` and `related` key of the 109 files resolves to a bead in `INDEX.tsv` (own keys, seed keys, reserved keys, and new beads of other units named by their real key). No bead of this unit lies on a dependency cycle, no dependency points to a later milestone, and no bead of another unit depends on a bead of this unit.

Checks run over the whole directory when this file was written:

- Every file parses: front matter with all eleven fields, valid JSON lists, key equal to file name, the seven body sections and the three test-plan parts present and non-empty.
- Every card of Appendix E was compared with its bead number by number: 818 numeric tokens across the 41 cards, all present in the bead (one written in another form: 0.88 appears as 88% in GB-08); every section, construction, gate, kill-gate and refusal identifier in a card's "Supports" line is present in its bead; all 41 beads carry the five fields (Equation, Substituted, Intuition, Assumptions, Flip), both lanes and at least one planted control.
- The 59 rows of table G.3 were compared with NEG-28 cell by cell (hazard, dimension, mechanism, failure): all present; four cells differ only by a dropped section pointer.
- The 18 rows of table G.4 were compared with the `ZOO-…` beads: actuated count, group and every construction, analysis and card identifier present.
- The 54 two-part verb names used by the unit's scripts and tests (`actuator.thermal`, `design.check`, `scene.import` and so on) all exist in the verb-registry beads of unit UJL.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| Appendix E, preamble | What a card is; the five fields; FACT inputs and their re-read; recomputed arithmetic; which carried cards were corrected | NEG-01 (file format `frob.card/1`, loader, two lanes, FACT re-read rule and `citable` flag, coverage lint), NEG-02 (end-to-end script over all 41), every `GB-nn` bead (the five fields in full) |
| E.1 | Cards carried from version 0.2 (GB-01 to GB-07) | GB-01, GB-02, GB-03, GB-04, GB-05, GB-06, GB-07; the corrections to GB-04 and GB-05 pinned by NEG-14; the correction to GB-07 pinned by a control inside GB-07 |
| E.2 | The thesis (GB-08 and GB-09) | GB-08, GB-09; NEG-05 (the 7.7 and 7.8 ceilings) |
| E.3 | The body (GB-10 to GB-25) | GB-10 to GB-25 (sixteen beads); NEG-06 (GB-18's 1.46), NEG-07 (GB-16's referee share), NEG-09 (GB-23's 616), NEG-12 (GB-21's tolerance) |
| E.4 | The mind (GB-26 to GB-36) | GB-26 to GB-36 (eleven beads); NEG-13 (GB-26's labels and reserve), NEG-04 (GB-32's 76,959), NEG-08 (GB-35's ladder cost) |
| E.5 | The record and the world (GB-37 to GB-39) | GB-37, GB-38, GB-39 |
| E.6 | Substrate and performance (GB-40 and GB-41) | GB-40, GB-41; NEG-11 (the capacities GB-41 uses) |
| E.7 | Discrepancies found while recomputing | NEG-03 (the register `registries/cards/discrepancies.toml`, the list of enumerations that reproduced, the stale-value scan), NEG-04 to NEG-10 (rows 1 to 7), NEG-11 to NEG-13 (the three inconsistencies between chapters), NEG-14 (the two corrections to carried cards) |
| Appendix G, preamble | Licences are FACT from one research pass, re-checked at admission; an asset whose licence was not read stays outside the default pipeline | NEG-15 (licence check, admitted list, `not-read` handling, the mark that must be `P` before a gate cites an asset), NEG-16 (end-to-end script), NEG-21 and NEG-27 (the not-verified and not-read rows) |
| G.1 | Reference robots | NEG-15 (layout `assets/robots/<name>/`, manifest `frob.robot.asset/1`, validation, index and archetype mapping, the never-Reality rule), NEG-16 (script), the nineteen `ROBOT-…` beads (table a.3), NEG-32 (the last row of the table: a sibling catalogue builder lowered through the airlock to a design term, the `fsim-catalog` format of `design.import`, used by ROBOT-fsim-g1 and ROBOT-iiwa7). The closing sentence (twelve archetypes without a reference model use their own template as baseline; OPEN-ARENA-2 tracks the hexapod): the count is in NEG-15, each of the twelve affected `ZOO-…` beads states that no reference robot is on file, and ZOO-hexapod-radial names the open question; the baseline rule itself is the scorecard bead's (N24a-22, unit U24a) and the open question's decision bead is N24c-16 (unit U24c) |
| G.2 | Scene assets | NEG-17 to NEG-26 (one bead per source, rows 1 to 10), NEG-27 (rows 11 and 12 and the correction: interior generation is not claimed as new), NEG-16 (scene-asset loop of the script). The manifest entry that travels with every pack, the pack-time refusal FROB-REF-310 and the starter-pack rule are mechanisms of N19b-02 (unit U19b); each bead here depends on it and tests its own row against it |
| G.3 | Hazard lists, consolidated | NEG-28 (the 59 rows as `registries/hazards/INDEX.toml`, the consolidation check against the seven per-archetype files, the all-`assumed` invariant, `dimensions_assumed`, FROB-REF-312, OPEN-HAB-1, OPEN-HAB-4, the Frontier-row fallback, the counts), NEG-29 (the cover map `registries/hazards/coverage.toml`, the three-part check that every hazard is covered by a requirement, a shield or a refusal, the uncovered-hazard report per battery, the script `hazard_covers.sh`) |
| G.4 | The eighteen robot archetypes | NEG-30 (the `zoo/` directory, `frob.zoo.expect/1`, the generic conformance test, the exactly-eighteen check), NEG-31 (end-to-end script), the eighteen `ZOO-…` beads (table a.6). The differential-drive rule (closed-form chord, never the sibling's planar-drive routine, as the known answer) is in ZOO-diff-drive. The closing paragraph (six variants outside the eighteen, behind feature flags and off the critical path) is carried in full by N14b-05 (unit U14b) |

### (a.1) Worked cards, one row per card

Every card bead adds `registries/cards/GB-nn.toml`, the test `gb_nn` (reference lane, mechanism lane, controls) in `crates/frob-conformance/tests/cards/`, and depends on NEG-01. "Mechanism beads" are the dependencies whose code the mechanism lane calls. "E.7" names the discrepancy item and the bead that pins it.

| Card | Question and relation | Owning crate | Milestone | Parent | Mechanism beads | Known answers of Appendix M cited | Gates and kill gates the card supports | E.7 |
|---|---|---|---|---|---|---|---|---|
| GB-01 | how wrong is a Cayley step: angle shortfall (ωh)²/12 and the refusal bound 0.2 | `frob-lie` | M0 | WP-03 | BD-011 | KA-052, KA-237 | none | |
| GB-02 | what a symmetry group is worth to a policy: n·m·\|Γ\| free weights against n·m·\|Γ\|² | `frob-algebra` | M1 | WP-03 | BD-048, N13-14 | KA-232 | KILL-EQUIV | |
| GB-03 | does the mass spiral close: m = m_f/(1 − κ), loop gain κ = n·c·g·L/ρ_τ | `frob-algebra`, `frob-analyze` | M1 | WP-11 | BD-052, BD-103 | KA-077, KA-262 | GATE-AUGUR-17 | |
| GB-04 | why positions are anchored: f32 spacing 2^(e−23), 0.24 µm in a centred 8 m cell | `frob-kernels` | M2 | WP-22 | N21b-37, BD-179 | KA-792 | GATE-SPARK-05 | E7-K1 (NEG-14) |
| GB-05 | what moving sideways costs a differential drive: about 4·√(d·b) of wheel travel | `frob-analyze` | M1 | WP-11 | BD-106, SPEC-C-07 | KA-267 | none | E7-K2 (NEG-14) |
| GB-06 | how many paired episodes a comparison needs: about 2·ln(1/α)/δ² pairs | `frob-arena` | M1 | WP-41 | BD-124 | none | none | |
| GB-07 | how much memory a scene is: volume/spacing³ samples, 2 bytes each, and the narrow band | `frob-scene` | M2 | WP-17 | BD-162, BD-164 | KA-459, KA-461 | GATE-HAB-13, KILL-POOL | corrected in place (control in GB-07) |
| GB-08 | what an unresolved mission does to the ten-fold ratio: R ≤ 1/q | `frob-conformance` | M0 | WP-34 | N00-17 | KA-009 | none | |
| GB-09 | which single strategy can reach ten-fold: three ceilings, none of which does | `frob-conformance` | M0 | WP-34 | N00-17 | KA-010, KA-013, KA-818 | none | E7-2 (NEG-05) |
| GB-10 | how wrong a solid box is where a shell was meant: mass ratio 11.8, gyration 31% off | `frob-model` | M1 | WP-07 | BD-062, BD-055 | KA-261, KA-262, KA-263 | GATE-SKEL-07 | |
| GB-11 | when a winding runs away: equilibrium P/(G − s) and I_crit = √(G/(R₀·α)) | `frob-actuate` | M1 | WP-09 | BD-072, N15a-14, N15a-15 | KA-305, KA-308, KA-309, KA-312 | GATE-SINEW-05 | |
| GB-12 | how much torque is left above base speed: field weakening on both limits | `frob-actuate` | M1 | WP-09 | BD-073 | KA-315 | GATE-SINEW-06 | |
| GB-13 | how much power the bus can deliver before it collapses: P ≤ E²/(4·R) | `frob-actuate` | M1 | WP-09 | BD-075, N15b-03 | KA-324 | GATE-SINEW-07 | |
| GB-14 | how many referee pairs promote a fast-tier result: the integer table (n, k) → verdict | `frob-opt` | M3 | WP-25 | N16b-05, BD-263 | KA-089, KA-350, KA-695 | GATE-ATEL-13, GATE-FID-02 | |
| GB-15 | what pairing with the fast tier saves on the referee: 1/(1 − ρ²) | `frob-opt` | M3 | WP-50 | BD-264, SPEC-C-25 | KA-016, KA-132, KA-352 | GATE-CHRON-09, GATE-FID-08, KILL-DR | |
| GB-16 | how much of a GPU run the referee can shadow: share c·ρ_rt/(T·h), under 1% | `frob-dyn` | M2 | WP-50 | BD-214, N16b-26 | KA-361, KA-821 | GATE-FID-07, KILL-SHADOW | E7-4 (NEG-07) |
| GB-17 | when a size bucket pays for itself: padding overhead Ω_p, crossover at 360 worlds | `frob-phylum` | M2 | WP-44 | N17a-04, BD-171 | KA-368, KA-371 | GATE-PHYLUM-03, KILL-BUCKET | |
| GB-18 | the most a family batch can gain: speed-up ≤ 1/(1 − f + f/r + h) | `frob-phylum` | M3 | WP-63 | N17a-11, N00-17 | KA-014, KA-373, KA-809 | GATE-AUGUR-14, KILL-FAMILYLANE | E7-3 (NEG-06) |
| GB-19 | how many distinct designs a symmetric sweep holds: Burnside counts, ballast classes | `frob-algebra` | M1 | WP-03 | BD-049, N13-17 | KA-123, KA-127, KA-233, KA-365, KA-367, KA-648, KA-697, KA-698 | GATE-PHYLUM-01, KILL-QUOTIENT | |
| GB-20 | how much friction is left when a foot also spins: the four-dimensional cone | `frob-contact` | M2 | WP-45 | BD-154, N18a-27 | KA-399 | GATE-TRAC-04 | |
| GB-21 | how long a foot rides a film before it grips: Stefan squeeze time 3π·η·a⁴/(4·N·h²) | `frob-contact` | M3 | WP-53 | BD-230, N19b-05 | KA-404, KA-438 | GATE-HAB-06, GATE-TRAC-05, KILL-FILM | E7-B (NEG-12) |
| GB-22 | why half precision is rejected for contact solves: κ·u₁₆ = 2.34, cost 9k/n | `frob-contact` | M2 | WP-45 | N18b-27, BD-156 | KA-423 | none | |
| GB-23 | a 30 Hz policy against 400 Hz physics: Q = 1,200, steps 0, 14, 27, 40, 600 decisions | `frob-sense` | M1 | WP-40 | BD-078 | KA-201, KA-471 | GATE-SENS-04, GATE-SENS-05 | E7-6 (NEG-09) |
| GB-24 | how bad the sensor may be when the first move is a probe: ε < (U + ½)/4 | `frob-mission`, `frob-sense` | M1 | WP-42 | N10b-08, BD-129 | KA-501, KA-637, KA-656 | GATE-AUGUR-03, GATE-SENS-13 | |
| GB-25 | why regrouping a sum is a different kernel: class E is bit-exact, class R is not | `frob-kir` | M0 | WP-05 | BD-018, SPEC-C-14 | KA-087, KA-212, KA-705 | none | |
| GB-26 | can the stop absorb everything that can reach it: 1.49 J against 2.00 J | `frob-shield` | M4 | WP-65 | BD-337, N22b-10, N22b-09 | KA-153, KA-568 | GATE-SHIELD-01, KILL-ENERGY | E7-C (NEG-13) |
| GB-27 | what an impact does to a perturbation: the saltation matrix, rimless-wheel multiplier ½ | `frob-gait` | M4 | WP-64 | N22a-33, BD-333, N10b-03 | KA-093 | GATE-GAIT-01, GATE-GAIT-02 | |
| GB-28 | after how many designs a family policy repays its pretraining: N > P/(s·(1 − f)) | `frob-learn` | M4 | WP-61 | BD-320 | KA-589 | GATE-CORTEX-11, KILL-FAMILY | |
| GB-29 | what a retry costs in the qualification lane: α_i = α/[i·(i + 1)] | `frob-arena` | M3 | WP-54 | BD-259, N24b-15 | KA-621 | GATE-QUAL-01 | |
| GB-30 | which verdicts 64 pairs can return: 521, 521, 1,103 inconclusive, and no tie | `frob-arena` | M1 | WP-41 | BD-124, N33b-19 | KA-612, KA-693 | GATE-ARENA-05, GATE-ATEL-06 | |
| GB-31 | how narrow the stance can be: b ≥ 3/14 m, with witnesses and a dual exclusion | `frob-mission` | M1 | WP-42 | BD-130, BD-132 | KA-022, KA-158, KA-159, KA-624, KA-625, KA-626 | GATE-AUGUR-02, GATE-AUGUR-05 | |
| GB-32 | what it costs to certify a rare failure rate: n = ⌈ln δ/ln(1 − p⋆)⌉ clean episodes | `frob-nemesis` | M3 | WP-55 | BD-276, BD-277 | KA-664 | GATE-NEMESIS-04, KILL-RARE | E7-1 (NEG-04) |
| GB-33 | how good a greedy battery of scenes is: OPT_k ≤ f(S) + top-k remaining gains | `frob-nemesis` | M3 | WP-55 | BD-278 | KA-669 | GATE-NEMESIS-08 | |
| GB-34 | did the intervention rescue the episode, or did the fork: contrast against placebo | `frob-autopsy` | M3 | WP-56 | BD-279, BD-280, N27b-05 | KA-673, KA-674 | GATE-AUTOPSY-03, KILL-AUTOPSY | |
| GB-35 | the survivor margin q̂ and how far its coverage can move: Beta(n + 1 − l, l) | `frob-opt` | M4 | WP-62 | BD-329, N28b-14 | KA-136 | GATE-ATEL-08, GATE-GROUND-06, KILL-LADDER | E7-5 (NEG-08) |
| GB-36 | which experiment is worth buying next: EVSI = s̃·[φ(z) − z·Φ(−z)] | `frob-opt` | M4 | WP-28 | BD-351, BD-349 | KA-187 | GATE-ATEL-15, KILL-VOI | |
| GB-37 | which cycles and tolerances carry the cost: linear damage, cube-root allocation | `frob-foundry` | M5 | WP-71 | BD-402, BD-406 | KA-169, KA-745 | GATE-FOUNDRY-03, GATE-FOUNDRY-08, KILL-TOLERANCE | |
| GB-38 | does the structure loop converge in its budget: (A + λ·M₀)/(1 − λ), gain κ + λ | `frob-foundry` | M5 | WP-70 | BD-404, GB-03 | KA-743 | GATE-FOUNDRY-06 | |
| GB-39 | how long a 10 s job waits behind a training run: 2,000 s, 621 s, or 0.17 s | `frob-verbs` | M3 | WP-26 | BD-296, BD-309 | KA-703, KA-758 | GATE-TETHER-10, KILL-PREEMPT, KILL-SLOT | |
| GB-40 | is that figure in world-steps or in env-steps: world-steps/s = d × env-steps/s | `frob-conformance` | M2 | WP-47 | BD-216 | KA-806 | GATE-SPARK-19 | |
| GB-41 | does memory or throughput bound the number of worlds: 35 kB and 97 kB per world | `frob-conformance` | M2 | WP-47 | BD-219, N16b-26, N17a-07 | KA-814 | GATE-SPARK-22 | E7-A (NEG-11) |

A card's milestone is that of the mechanism its second lane calls; before that milestone the reference lane and the controls run and the mechanism lane reports `not run`, never `pass`.

### (a.2) The items of E.7, one row per item

| Item | Where it was printed | First printed | Settled value | Bead | Milestone | Depends on (beyond NEG-03) |
|---|---|---|---|---|---|---|
| E7-1 (row 1) | Rare-failure certification, tenth qualification request at 10⁻⁴ | 76,963 clean episodes | 76,959 by the exact rule ⌈ln δ/ln(1 − p⋆)⌉ | NEG-04 | M3 | GB-32, BD-276 |
| E7-2 (row 2) | Engine-speed ceiling at a step share of 0.87 | 7.8 | 7.7 from 0.87; 7.8 only from the unrounded share 82/94 | NEG-05 | M0 | GB-09, N00-17 |
| E7-3 (row 3) | Ceiling of a zero-cost solver | "94% shared work" giving about 1.46 (1.345/0.94 is 1.43) | 842.8 ÷ 575.7 = 1.46; the share attribution is withdrawn | NEG-06 | M0 | N00-17, BD-043 |
| E7-4 (row 4) | Referee share on a humanoid batch | 0.5 M world-steps/s, 1,000 world-seconds per second, 0.5 to 2.4% | 1.7 M, 3,400 and 0.14 to 0.71% | NEG-07 | M2 | GB-16, N16b-26 |
| E7-5 (row 5) | Ladder cost with six survivors | 2.6 h with the audit lane | 2.0 h before the audit lane; 2.2 h with a tenth added; 3.0 h with 19 audit designs at the second rung | NEG-08 | M4 | GB-35, BD-329 |
| E7-6 (row 6) | Every-thirteenth-step scheduling, decisions in 20 s | 615 | 616 in a half-open window, against 600; rate error 2.6%, overpayment on counts 2.7% | NEG-09 | M1 | GB-23, BD-078 |
| E7-7 (row 7) | Wheel-loop spectral radius, instantaneous-current row | 2.7075 beside 1.4820 and 0.9207 | 2.6917 with the fixture's 0.002 N·m·s/rad of shaft damping; 2.7075 without it | NEG-10 | M1 | BD-108, BD-123 (no card uses the row) |
| E7-A | Default capacities | C = 24, R = 128 (27 kB) and C = 40, R = 224 (72 kB) | C = 32, R = 160 (35 kB) and C = 64, R = 320 (97 kB), read from one table | NEG-11 | M2 | GB-41, N21b-18, BD-148 |
| E7-B | Squeeze-film tolerance | 1% at the scene gate, 2% at the contact gate | 2% at both gates, registered once | NEG-12 | M3 | GB-21, BD-230, N19b-06, N18a-37 |
| E7-C | Energy-budget labels | 0.25 J stored energy, 0.10 J reserve | 0.25 J external work, 0.10 J stored, reserve a further 0.10 J; total 1.49 J; cutoff 50.5 ms, 45.5 ms with the reserve | NEG-13 | M4 | GB-26, N22b-10, BD-337 |
| E7-K1 | GB-04, worst spacing inside an 8 m cell | 0.48 µm | 0.24 µm with the centred offset | NEG-14 | M2 | GB-04 |
| E7-K2 | GB-05, cost of a 0.10 m sideways shift | 0.57 m (small-angle only) | 0.57 m small-angle and 0.55 m exact at 0.72 rad | NEG-14 | M2 | GB-05 |

### (a.3) Reference robots of G.1, one row per robot

Every robot bead adds `assets/robots/<name>/` (manifest, licence text, import golden, loss report, validation golden) and depends on NEG-15. None is ever the source of a Reality claim.

| Bead | Model | Source | Licence on file | Role | Archetype it serves | Milestone | Priority |
|---|---|---|---|---|---|---|---|
| ROBOT-go1 | Unitree Go1 | MuJoCo Menagerie | BSD-3 | parity fixture and baseline | `quad-mammal` | M1 | 1 |
| ROBOT-go2 | Unitree Go2 | MuJoCo Menagerie | BSD-3 | parity fixture and default baseline | `quad-mammal` | M1 | 1 |
| ROBOT-a1 | Unitree A1 | MuJoCo Menagerie | BSD-3 | parity fixture and baseline | `quad-mammal` | M1 | 1 |
| ROBOT-anymal-b | ANYmal B | MuJoCo Menagerie | BSD-3 | parity fixture and baseline | `quad-mammal` | M1 | 1 |
| ROBOT-anymal-c | ANYmal C | MuJoCo Menagerie | BSD-3 | parity fixture and baseline | `quad-mammal` | M1 | 1 |
| ROBOT-spot | Spot | MuJoCo Menagerie | BSD-3 | parity fixture and baseline | `quad-mammal` | M1 | 1 |
| ROBOT-h1 | Unitree H1 | MuJoCo Menagerie | BSD-3 | parity fixture and baseline | `humanoid` | M1 | 1 |
| ROBOT-g1 | Unitree G1 | MuJoCo Menagerie | BSD-3 | the 38-link parity reference of the humanoid mass row | `humanoid` | M1 | 1 |
| ROBOT-cassie | Cassie | MuJoCo Menagerie | MIT | closed-loop legs; reference | `biped-point` | M3 | 2 |
| ROBOT-stretch3 | Stretch 3 | MuJoCo Menagerie | Apache-2.0 | parity fixture and baseline | `diff-drive` (base), `mobile-manipulator` | M1 | 1 |
| ROBOT-tiago | TIAGo | MuJoCo Menagerie | Apache-2.0 | parity fixture and baseline | `mecanum` (omni base), `mobile-manipulator` | M1 | 1 |
| ROBOT-booster-t1 | Booster T1 | MuJoCo Menagerie | Apache-2.0 | import parity only | `humanoid` | M1 | 2 |
| ROBOT-apollo | Apptronik Apollo | MuJoCo Menagerie | Apache-2.0 | import parity only | `humanoid` | M1 | 2 |
| ROBOT-fourier-n1 | Fourier N1 | MuJoCo Menagerie | Apache-2.0 | import parity only | `humanoid` | M1 | 2 |
| ROBOT-op3 | OP3 | MuJoCo Menagerie | Apache-2.0 | import parity only | `humanoid` | M1 | 2 |
| ROBOT-talos | TALOS | MuJoCo Menagerie | Apache-2.0 | import parity only | `humanoid` | M1 | 2 |
| ROBOT-barkour | Barkour | MuJoCo Menagerie | Apache-2.0 | import parity only | `quad-mammal` | M1 | 2 |
| ROBOT-fsim-g1 | Unitree G1, 29 degrees of freedom | the sibling physics library's robot catalogue, with provenance | the suite's | declared reduced profile `g1-reduced-30` (30 links, 31.96314202 kg); M0 smoke model; never the mass reference | `humanoid` | M1 | 1 |
| ROBOT-iiwa7 | KUKA iiwa 7 | the sibling physics library's robot catalogue, with provenance | the suite's | the arm; workspace and singular-locus fixture | `mobile-manipulator` (arm) | M3 | 2 |

The six import-parity-only robots are one step less urgent than their milestone's default priority: they add importer coverage and no baseline.

### (a.4) Scene assets of G.2, one row per table row

| Row | Asset | Licence on file | Use stated | Bead | Register class | In the default pipeline | Milestone |
|---|---|---|---|---|---|---|---|
| 1 | Procedural generators in `frob-scene` | this project's | default; seven interior archetypes from one grammar system, structural proxies first | NEG-17 | `redistributable` | yes | M3 |
| 2 | Infinigen Indoors | BSD-3 | offline external generator for populated interiors | NEG-18 | `attribution` | yes, offline only, never linked into the product | M5 |
| 3 | Google Scanned Objects | CC-BY 4.0 | household clutter | NEG-19 | `attribution` | yes, attribution in every pack | M5 |
| 4 | RoboCasa | code MIT, assets CC BY 4.0 | kitchen fixtures and layouts | NEG-20 | `attribution` | yes, assets only | M5 |
| 5 | ProcTHOR | code Apache-2.0; dataset licence not verified | generator reference only | NEG-21 | `unknown` | no | M3 |
| 6 | HSSD | CC BY-NC 4.0 | user-supplied import only; never redistributed | NEG-22 | `research-only` | no | M3 |
| 7 | HM3D | academic, non-commercial | user-supplied import only; never redistributed | NEG-23 | `research-only` | no | M3 |
| 8 | PartNet-Mobility | non-commercial research terms | user-supplied import only; never redistributed | NEG-24 | `research-only` | no | M3 |
| 9 | BEHAVIOR-1K | its own agreement | not used; supported only as a user-supplied import | NEG-25 | `proprietary` | no | M3 |
| 10 | Objaverse-XL | per object | not used by default | NEG-26 | `per-object` | no | M3 |
| 11 | RoboBenchMart, MarketGen | not read | reference only, prior art for retail interiors | NEG-27 | `unknown` (two rows, `read = false`) | no | M3 |
| 12 | Isaac Sim 6.2.0 Warehouse Creator | not read | reference only, prior art for warehouse interiors | NEG-27 | `unknown` (`read = false`) | no | M3 |

NEG-27 also carries the correction paragraph: six published generators across four scene types, none found for manufacturing plants or commercial kitchens (an absence found by a bounded search), and the four things this project adds to a generated interior (the hazard list with dimensions and provenance, the optical map, the feasibility tag with its cut, the acceptance report), with a lint that rejects a novelty claim.

### (a.5) Hazard lists of G.3, by archetype

The rows themselves are in NEG-28 (numbered 1 to 59, in the order of the table). "Requirement", "shield" and "refusal" count hazards with at least one cover of that kind in the cover map that NEG-29 checks.

| Archetype | Hazards | With a requirement | With a shield | With a refusal | (hazard, cover) pairs | Rows naming a Frontier mechanism |
|---|---|---|---|---|---|---|
| Home | 10 (rows 1 to 10) | 9 | 1 | 3 | 19 | 3 |
| Apartment | 5 (rows 11 to 15) | 3 | 1 | 4 | 9 | 0 |
| Office | 7 (rows 16 to 22) | 7 | 0 | 5 | 14 | 0 |
| Warehouse | 10 (rows 23 to 32) | 9 | 2 | 4 | 20 | 2 |
| Manufacturing plant | 8 (rows 33 to 40) | 8 | 1 | 1 | 14 | 1 |
| Retail store | 8 (rows 41 to 48) | 6 | 1 | 4 | 12 | 1 |
| Commercial kitchen | 11 (rows 49 to 59) | 11 | 2 | 3 | 21 | 1 |
| Total | 59 | 53 | 8 | 24 | 109 | 8 |

Every hazard has at least one cover (59 of 59); six are covered by a refusal alone (hallway, right-angle corridor turn, compact kitchen galley, warehouse aisle, end cap, checkout lane). Every dimension is `assumed`. Of the eight rows that name a Frontier mechanism, five have a Solid alternative beside it (rows 3, 4, 29, 32, 59) and three have none (rows 9, 39, 48: pets, floor vibration, shoppers).

### (a.6) Archetypes of G.4, one row per archetype

Every archetype bead adds `zoo/<name>.fbot` and `zoo/<name>.expect.toml` (morphology, symmetry and its breakers, what each preflight stage must report, known answers, control variants) and depends on NEG-30 and on the design-language bead that writes its template.

| # | Archetype | Bead | Actuated | Γ_mech as built | Milestone | Template bead | Known answers of Appendix M cited | Refusal codes its controls and variants must raise | Reference robots |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `quad-mammal` | ZOO-quad-mammal | 12 | C₂ × C₂ | M1 | BD-109 | KA-096, KA-270, KA-276 | FROB-REF-105, FROB-REF-109 | ROBOT-go1, -go2, -a1, -anymal-b, -anymal-c, -spot; -barkour (import parity only) |
| 2 | `quad-sprawl` | ZOO-quad-sprawl | 12 | C₂ × C₂ | M3 | N14b-02 | KA-270 | FROB-REF-132 | none on file |
| 3 | `hexapod-radial` | ZOO-hexapod-radial | 18 | D₆ (C₆ with handed legs) | M3 | N14b-02 | KA-233, KA-270, KA-365, KA-697 | FROB-REF-109 | none on file (OPEN-ARENA-2) |
| 4 | `octopod` | ZOO-octopod | 24 | D₈ radial; C₂ in a spider layout | M3 | N14b-02 | KA-270 | FROB-REF-109 | none on file |
| 5 | `biped-point` | ZOO-biped-point | 6 | C₂ | M3 | N14b-02 | KA-270 | FROB-REF-121, FROB-REF-124 | ROBOT-cassie |
| 6 | `humanoid` | ZOO-humanoid | 29 | C₂ | M1 | BD-109 | KA-270 | none (a reduced import without its block is refused) | ROBOT-g1, -h1, -fsim-g1; -booster-t1, -apollo, -fourier-n1, -op3, -talos (import parity only) |
| 7 | `diff-drive` | ZOO-diff-drive | 2 | C₂ | M1 | BD-109 | KA-265, KA-267, KA-270 | FROB-REF-109 | ROBOT-stretch3 (base) |
| 8 | `ackermann` | ZOO-ackermann | 3 | C₂ | M3 | N14b-03 | KA-266, KA-270 | FROB-REF-110 | none on file |
| 9 | `mecanum` | ZOO-mecanum | 4 | C₂ × C₂ | M1 | BD-109 | KA-268, KA-270 | FROB-REF-118 | ROBOT-tiago (omni base) |
| 10 | `omni-3` | ZOO-omni-3 | 3 | D₃ | M3 | N14b-03 | KA-270 | FROB-REF-109, FROB-REF-118 | none on file |
| 11 | `swerve-4` | ZOO-swerve-4 | 8 | D₄ on a square base | M3 | N14b-03 | KA-270, KA-399 | FROB-REF-109 | none on file |
| 12 | `tracked-flipper` | ZOO-tracked-flipper | 6 | C₂ × C₂ | M3 | N14b-04 | KA-270, KA-282, KA-418 | FROB-REF-109 | none on file |
| 13 | `wheel-leg` | ZOO-wheel-leg | 16 | C₂ × C₂ | M3 | N14b-04 | KA-270, KA-283 | FROB-REF-121 | none on file |
| 14 | `tri-wheel-stair` | ZOO-tri-wheel-stair | 4 | C₂; C₃ or D₃ in each cluster | M3 | N14b-04 | KA-270 | none | none on file |
| 15 | `rocker-bogie` | ZOO-rocker-bogie | 6 | C₂ | M3 | N14b-03 | KA-270 | FROB-REF-110 | none on file |
| 16 | `mobile-manipulator` | ZOO-mobile-manipulator | 9 | trivial with the arm off-centre | M3 | N14b-04 | KA-270 | FROB-REF-109, FROB-REF-119 | ROBOT-iiwa7 (arm), ROBOT-stretch3, ROBOT-tiago |
| 17 | `snake-crawler` | ZOO-snake-crawler | 8 | C₂ under head-to-tail reversal | M3 | N14b-04 | KA-254, KA-270 | none | none on file |
| 18 | `balancer-2w` | ZOO-balancer-2w | 2 | C₂ | M3 | N14b-03 | KA-269, KA-270 | none | none on file |

Six archetypes have a reference robot on file and twelve have none, as G.1 states.

## (b) Gates

Neither appendix defines a gate, a kill gate or an admission gate. Three gates of other chapters are measured in part by beads of this unit (they are in those beads' `gates` field):

| Gate | Defined in | Beads of this unit | What they contribute |
|---|---|---|---|
| GATE-SKEL-03 (M1 for four archetypes, M3 for eighteen) | §14.15, §40 | NEG-31, with NEG-30 and the eighteen `ZOO-…` beads behind it | Per-archetype rows: the template's default instance compiles, passes preflight stages 1 to 6 with no refusal, and has its symmetry groups on file. The standing and rolling clause and the gate record are BD-246's |
| GATE-TRAC-05 (M3) | §18, §40 | NEG-12 | The single registration of the squeeze-time tolerance (2%) that the gate reads; the gate is BD-230's |
| GATE-HAB-06 (M3) | §19, §40 | NEG-12 | The same registration, read by the scene-side gate, so that the two gates cannot hold one known answer to two tolerances |

Admission tests of other chapters that beads here record or rely on: the roll-up of the sibling's G1 catalogue under GATE-WS-09 (ROBOT-fsim-g1); the sibling's planar-drive routine under INT-FSIM-05, which is not used as an oracle (ZOO-diff-drive depends on BD-033, the independent chord formula and that admission); the sibling's tri-wheel geometry module (ZOO-tri-wheel-stair, in its `needs`).

Gates named by this unit's beads because a card, an asset or an archetype supplies a hand-checkable golden or a fixture for the relation the gate measures. The gate's record belongs to the owner beads in the last column (found by the `gates` field in `INDEX.tsv`):

| Gate | Beads of this unit that name it | Beads that list it in `gates` |
|---|---|---|
| GATE-ARENA-05 | GB-30 | BD-124, N24b-06, N24b-21, N24c-08 and 4 more |
| GATE-ATEL-06 | GB-30 | BD-124, N28a-18, N28a-19, N28a-30 and 3 more |
| GATE-ATEL-08 | GB-35 | BD-310, BD-329, N28a-32, N28b-19 and 1 more |
| GATE-ATEL-13 | GB-14 | BD-309, BD-341, BD-349, N28a-27 and 10 more |
| GATE-ATEL-15 | GB-36 | BD-351, N28b-18, N28b-21, N28c-16 and 3 more |
| GATE-AUGUR-02 | GB-31 | BD-130, BET-19, N25a-05, N25a-06 and 10 more |
| GATE-AUGUR-03 | GB-24 | BD-129, BET-19, N25c-20, N25c-21 and 5 more |
| GATE-AUGUR-05 | GB-31 | BD-133, BET-19, N25b-06, N25d-27 and 3 more |
| GATE-AUGUR-14 | GB-18 | BD-354, N25a-27, N25c-06, N25d-30 and 1 more |
| GATE-AUGUR-17 | GB-03 | BD-360, BD-361, N10a-11, N25d-10 and 8 more |
| GATE-AUTOPSY-03 | GB-34 | BD-280, BET-23, EPIC-M3, N10c-01 and 5 more |
| GATE-CHRON-09 | GB-15 | BD-264, FD-FSIM-17, N29b-08, N40-08 and 1 more |
| GATE-CORTEX-11 | GB-28 | BD-321, BET-15, EPIC-M4, N23a-23 and 5 more |
| GATE-FID-02 | GB-14 | BD-263, EPIC-M3, N16b-31, N16c-04 and 2 more |
| GATE-FID-07 | GB-16 | BD-214, BD-268, BET-34, N16b-34 and 1 more |
| GATE-FID-08 | GB-15 | BD-264, N16b-31, N16c-04, N40-08 and 1 more |
| GATE-FOUNDRY-03 | GB-37 | BD-402, N10c-06, N31a-21, N31a-55 and 2 more |
| GATE-FOUNDRY-06 | GB-38 | BD-404, N31a-24, N31a-54, N31b-02 and 2 more |
| GATE-FOUNDRY-08 | GB-37 | BD-406, BET-29, N10c-07, N31a-32 and 6 more |
| GATE-GAIT-01 | GB-27 | BD-333, BET-26, FD-FSIM-20, N22a-32 and 5 more |
| GATE-GAIT-02 | GB-27 | BD-334, FD-FSIM-20, N10a-04, N22a-34 and 5 more |
| GATE-GROUND-06 | GB-35 | BD-393, BET-28, EPIC-M5, N30a-36 and 10 more |
| GATE-HAB-06 | GB-21, NEG-12 | BD-230, BET-36, N18a-37, N19b-06 and 2 more |
| GATE-HAB-13 | GB-07 | BD-164, BD-226, N19c-17, N19c-19 and 1 more |
| GATE-NEMESIS-04 | GB-32 | BD-276, BET-32, N10a-09, N26a-30 and 1 more |
| GATE-NEMESIS-08 | GB-33 | BD-278, N26b-12, WP-55 |
| GATE-PHYLUM-01 | GB-19 | BD-285, BET-31, N17a-27, N17a-28 and 1 more |
| GATE-PHYLUM-03 | GB-17 | BD-185, BD-186, BET-18, EPIC-M2 and 2 more |
| GATE-QUAL-01 | GB-29 | BD-259, BD-260, BET-21, EPIC-M3 and 8 more |
| GATE-SENS-04 | GB-23 | BD-078, BD-189, N20a-06, N20a-07 and 4 more |
| GATE-SENS-05 | GB-23 | BD-098, BD-189, N20a-44, WP-40 |
| GATE-SENS-13 | GB-24 | BD-190, N20b-13, N20b-14, WP-49 |
| GATE-SHIELD-01 | GB-26 | BD-337, BET-25, EPIC-M4, N22b-15 and 7 more |
| GATE-SINEW-05 | GB-11 | BD-072, N15-KA, N15a-14, N15a-15 and 4 more |
| GATE-SINEW-06 | GB-12 | BD-073, BD-151, N15-KA, N15a-16 and 2 more |
| GATE-SINEW-07 | GB-13 | BD-075, FD-FSIM-21, N15-KA, N15b-03 and 3 more |
| GATE-SKEL-01 (import parity) | NEG-15 and the seventeen file-import `ROBOT-…` beads (all but ROBOT-fsim-g1 and ROBOT-iiwa7) | BD-065, BD-066, EPIC-M1, N14a-31 and 4 more |
| GATE-SKEL-03 | NEG-30, NEG-31 | BD-109, BD-118, BD-246, BD-247 and 7 more |
| GATE-SKEL-07 (mass rows) | GB-10, NEG-15, ZOO-humanoid and the eleven parity-and-baseline `ROBOT-…` beads | BD-062, BD-109, BD-246, N14a-27 and 5 more |
| GATE-SKEL-11 | ROBOT-g1 | N14b-24, N14b-33, N14b-34 |
| GATE-SPARK-05 | GB-04 | BD-179, BET-08, WP-22 |
| GATE-SPARK-19 | GB-40 | BD-216, N36-03, N36-06, N36-13 and 2 more |
| GATE-SPARK-22 | GB-41 | BD-219, N36-16, N36-17, N36-18 and 3 more |
| GATE-TETHER-10 | GB-39 | BD-296, N32a-20, N32a-21, N32a-22 and 2 more |
| GATE-TRAC-04 | GB-20 | BD-154, BD-155, BD-167, N15b-37 and 6 more |
| GATE-TRAC-05 | GB-21, NEG-12 | BD-230, BET-36, N18a-37, NS-04 and 1 more |
| GATE-WS-09 | ROBOT-fsim-g1 | N33c-41, N33c-42, N33c-43, N33c-44 and 9 more |

Kill gates named in a card's Flip or in a bead's Considerations as the fallback, none decided here. The second column is the bead of this unit, the third the beads that decide or carry the kill gate elsewhere:

| Kill gate | Bead of this unit | Beads elsewhere |
|---|---|---|
| KILL-AUTOPSY | GB-34 | N27b-11, BD-280, BD-319, BET-23 |
| KILL-BUCKET | GB-17 | N21b-19, BD-171, BD-186, BD-224 |
| KILL-DR | GB-15 | N16b-10, BD-264, BD-319, N29b-07 |
| KILL-DRESSING | NEG-18 (NEG-19 and NEG-20 depend on the same dressing pilot, N19a-40) | N19a-41, BD-399, BD-412, BD-421 |
| KILL-ENERGY | GB-26 | N22b-16, BD-337, BD-378 |
| KILL-EQUIV | GB-02 | BD-326, N23a-33, BD-378, BET-07 |
| KILL-FAMILY | GB-28 | N23a-26, BD-320, BD-378, BET-15 |
| KILL-FAMILYLANE | GB-18 | N17a-14, BD-356, N17a-11, N17a-13 |
| KILL-FILM | GB-21 | N18a-34, BET-36, N18a-33, N40-24 |
| KILL-LADDER | GB-35 | N28a-25, N28b-20, BD-378, EPIC-M4 |
| KILL-POOL | GB-07 | N19c-18, N19c-19, BD-164, BD-224 |
| KILL-PREEMPT | GB-39 | N32a-24, N32b-28, BD-296, N32a-20 |
| KILL-QUOTIENT | GB-19 | N17a-29, BD-285, BET-31, N17a-25 |
| KILL-RARE | GB-32 | N26a-32, BD-277, BD-319, BET-32 |
| KILL-SHADOW | GB-16 | N16b-19, N16b-20, BD-268, BD-319 |
| KILL-SLOT | GB-39 | N32a-23, N32b-27, BD-295, BD-319 |
| KILL-TOLERANCE | GB-37 | N31b-18, BD-406, BD-412, BET-29 |
| KILL-VOI | GB-36 | N28a-26, N28b-21, N28c-17, N40-26 |

Open questions named with their fail-closed defaults, none decided here:

| Open question | Bead of this unit | Decision or experiment bead elsewhere |
|---|---|---|
| OPEN-ARENA-2 (a permissively licensed hexapod reference model) | ZOO-hexapod-radial; the rule in NEG-15 | N24c-16 |
| OPEN-ATEL-1 | GB-14, GB-36 | N28c-26 |
| OPEN-HAB-1 (cited norms for hazard dimensions) | NEG-28 | N19c-30, N26b-19 |
| OPEN-HAB-4 (retrofit prices and legality constraints) | NEG-28 | N19b-43 |
| OPEN-NEMESIS-3 | GB-32 | N26b-21 |
| OPEN-SPARK-3 | NEG-11 | N21b-18 |
| OPEN-TRAC-2 | GB-20 | N18a-27 |
| OPEN-TRAC-3 | GB-21 | N18b-36 |

## (c) Counts

- Worked cards: 41 of 41, each with its equation, substituted values, intuition, assumptions and flip. About 388 golden rows in the 41 tables of substituted values (each a named output of a card file with its tolerance kind); every card has at least one planted control.
- Items of E.7: 12 of 12 (seven arithmetic rows, three inconsistencies between chapters, two corrections to carried cards) in 11 beads, plus the register and scan (NEG-03). The third carried-card correction named in the appendix preamble (GB-07) is pinned inside GB-07.
- Reference robots: 19 of 19 (seventeen from MuJoCo Menagerie: eight under BSD-3, one under MIT, eight under Apache-2.0; two from the sibling library's catalogue under the suite's licence).
- Scene-asset rows: 12 of 12, in 11 beads.
- Hazards: 59 of 59 in seven archetypes (10, 5, 7, 10, 8, 8, 11), every dimension `assumed`; 109 (hazard, cover) pairs over a cover vocabulary of 24 codes (16 requirement templates, 3 shield kinds, 5 refusals).
- Archetypes: 18 of 18 (four at M1, fourteen at M3).
- Known answers of Appendix M cited with their values (Appendix M has no section for Appendix E or G, so the unit has no `-KA` registry bead): 91 distinct. KA-009, KA-010, KA-013, KA-014, KA-016, KA-022, KA-052, KA-077, KA-087, KA-089, KA-093, KA-096, KA-123, KA-127, KA-132, KA-136, KA-153, KA-158, KA-159, KA-169, KA-187, KA-201, KA-212, KA-232, KA-233, KA-237, KA-254, KA-261, KA-262, KA-263, KA-265, KA-266, KA-267, KA-268, KA-269, KA-270, KA-276, KA-282, KA-283, KA-305, KA-308, KA-309, KA-312, KA-315, KA-324, KA-350, KA-352, KA-361, KA-365, KA-367, KA-368, KA-371, KA-373, KA-399, KA-404, KA-418, KA-423, KA-438, KA-459, KA-461, KA-471, KA-501, KA-568, KA-589, KA-612, KA-621, KA-624, KA-625, KA-626, KA-637, KA-648, KA-656, KA-664, KA-669, KA-673, KA-674, KA-693, KA-695, KA-697, KA-698, KA-703, KA-705, KA-743, KA-745, KA-758, KA-792, KA-806, KA-809, KA-814, KA-818, KA-821.
- Known answers new to this unit, stated inline and without an Appendix M number: the card outputs that no Appendix M row prints (all of GB-06; the flip values and the enumerations of the other cards); the settled values of the twelve E.7 items; the zoo expectation rows that are closed forms worked in the archetype beads (chord step and half turn, threshold lever arms, the 3 × 3 wheel map of `omni-3` and its inverse, turning radius L/tan δ, orbit counts under C₆, the coupling zonotope, bytes per world of the humanoid, the bucket of the octopod, and others); the hazard counts and the cover-map totals; the sibling G1 roll-up (31.96314202 kg against 33.34014202 kg, a difference of 1.37700000 kg).
- Counterexamples of Appendix N cited by number (Appendix N has no section for Appendix E or G, so no `-CX` registry bead): 6 (CX-025, CX-118, CX-131, CX-133, CX-134, CX-265). Planted wrong formulas carried by statement as controls: at least one per card (41 cards), per archetype (18), per E.7 item (the first-printed value used as a labelled stale control), per robot and per scene-asset source.
- Refusal codes: neither appendix defines one, so no `-REF` registry bead. Codes of other chapters named with their meanings: 32 distinct. FROB-REF-102, -104, -105, -107, -109, -110, -116, -118, -119, -121, -123, -124, -131, -132, -133, -135, -163, -165, -212, -242, -245, -249, -260, -263, -310, -312, -532, -602, -740, -741, -862, -866.
- Other identifiers named where a bead serves or relies on them: 24 constructions (C-04, C-05, C-07, C-08, C-09, C-10, C-12, C-14, C-15, C-16, C-17, C-19, C-22, C-23, C-25, C-26, C-27, C-29, C-31, C-32, C-35, C-37, C-38, C-39); 9 theorem cards (T-07, T-12, T-17, T-21, T-22, T-23, T-24, T-26, T-27); 10 analyses (A-01, A-05, A-06, A-07, A-08, A-09, A-10, A-11, A-12, A-22); the 11 decision cores DC-01 to DC-11; 10 architecture decisions (ADR-AUGUR-03, ADR-FOUNDRY-07, ADR-HAB-05, ADR-MARROW-02, ADR-NUM-06, ADR-PHYLUM-02, ADR-SINEW-04, ADR-SINEW-08, ADR-SKEL-03, ADR-TRAC-06); 7 proof obligations (PO-ARENA-02, PO-KIN-03, PO-NEMESIS-03, PO-NUM-01, PO-SHIELD-01, PO-SKEL-02, PO-TRAC-02); 4 invariants (INV-AUTOPSY-05, INV-NEMESIS-03, INV-PERF-01, INV-QUAL-03); 12 bets (BET-07, -08, -13, -14, -15, -17, -25, -26, -31, -32, -34, -36).
- Tests: about 790 named unit tests across the 109 beads. End-to-end scripts owned by the unit: `scripts/e2e/worked_cards.sh` (NEG-02), `scripts/e2e/assets_licences.sh` (NEG-16), `scripts/e2e/zoo_goldens.sh` (NEG-31), `scripts/e2e/hazard_covers.sh` (NEG-29); one step added to the scene unit's `scripts/e2e/scene_hazards.sh` (NEG-28). Each end-to-end bead depends on STD-E2E.

## (d) Needs from other slices

The `needs` fields of the unit's beads, to be resolved to keys by the lead:

- NEG-03: Appendix M.38: the eleven rows of values that did not reconcile with their own formulas, as registry rows owned by the known-answer suite (four overlap with E.7: its rows 7, 2, 1 and 6 are E7-1, E7-2, E7-3 and E7-6).
- NEG-15: §37.7: the licence join over derivation graphs (a report or pack carries the join of its inputs' licences), as the rule the robot manifest feeds.
- NEG-18, NEG-19, NEG-20: §19.13: `scene.import` for single assets and whole generated interiors at M3 (decode, compile, loss report, licence manifest entry).
- NEG-22, NEG-23, NEG-24: §19.13: `scene.import` for user-supplied scenes, scanned scenes and articulated assets (scale label, unknown mask, joint mapping, loss report).
- NEG-27: Appendix H: the source-register rows for the two retail-scene generators (abstracts only, no URL recorded) and the warehouse builder's documentation pages, as registry entries with their marks.
- NEG-29: §24: the registry of requirement templates per task family, as the target of the sixteen R codes; §22.12: the registry of shield kinds available at M3 and M4 (stopping distance, geofence, energy budget), as the target of the three S codes.
- ROBOT-cassie: Appendix F: the upstream loop-constraint work item for exact loop closures on the referee, by its identifier (ZOO-ackermann and ZOO-rocker-bogie wait on the same item).
- ROBOT-fsim-g1: Appendix F: the upstream work item that folds or declares the omitted masses of the sibling's G1 catalogue, by its identifier.
- ZOO-balancer-2w: §38.6: the derivation file for the balancer's open-loop pole, which the specification names and does not write.
- ZOO-tri-wheel-stair: §33: the admission test of the sibling's tri-wheel cluster geometry module before it is used as an oracle.

Further points for the lead:

- Content of these appendices that is carried by beads of other units, which this unit cites and does not duplicate: the six variants outside the eighteen archetypes (N14b-05, unit U14b); the licence manifest, the asset-source register, the pack-time refusal FROB-REF-310 and the starter-pack rule (N19b-02, unit U19b); the seven per-archetype hazard registry files and their verb (BD-227 and N19a-31 to N19a-37), provenance promotion and the FROB-REF-312 path (N19a-39), the incidence export (N19a-38); the baseline-is-the-template rule (N24a-22) and OPEN-ARENA-2 (N24c-16); OPEN-HAB-1 (N19c-30) and OPEN-HAB-4 (N19b-43). The lead should confirm that those beads keep them.
- Repository layout: the unit adds `registries/cards/GB-nn.toml`, `registries/cards/discrepancies.toml`, `zoo/` (with `INDEX.toml`), `assets/robots/` (with `INDEX.toml`), `registries/hazards/INDEX.toml` and `registries/hazards/coverage.toml`. The layout table of the workspace chapter lists `registries/ … cards/` and `assets/` and has no `zoo/`; the workspace unit (U34) owns that table.
- Known-answer numbers: Appendix M has no rows for these appendices. The unit's new known answers are keyed by their file and row (`GB-nn/<output>`, `zoo/<name>/<row>`, `E7-n`); if the known-answer registry of the verification chapter wants numbers for them, the list is in (c).
- The verb form for building a design from a sibling builder was an assumption of NEG-16 with no carrier. Settled at integration: it is the `fsim-catalog` format of `design.import`, carried by NEG-32, which adds the format value to the registry row of that verb (VERBS-design, unit UJL). The 54 verb names the unit uses are in the verb registry.
- `scripts/e2e/zoo_goldens.sh` (NEG-31) is driven by the zoo's expectation files; the archetype-catalogue script of unit U14b walks the template catalogue and the standing rows. They share GATE-SKEL-03 and should stay two scripts with one gate record (BD-246).

## (e) Contradictions and things that could not be turned into work as written

1. The brief places card files at `cards/GB-nn.toml`; the repository layout has `cards/` under `registries/` and no top-level `cards/`. Done: `registries/cards/GB-nn.toml` (NEG-01). Theorem cards use `registries/theorems.toml`, so the directory is not shared.
2. The brief places archetype goldens at `zoo/<name>.fbot`; G.4 says archetypes ship in `assets/archetypes/`. Done: both exist with distinct roles. The template in `assets/archetypes/` is parameterized and may evolve; the zoo instance is the template's default, pinned by `DesignId`, and the conformance test asserts that the two agree (NEG-30).
3. The preamble of Appendix E says three carried cards were corrected (GB-04, GB-05, GB-07); E.7 describes two corrections (GB-04, GB-05). Done: GB-07's correction (the 600 MB figure belongs to 4 cm, not 2 cm) is pinned by a planted wrong golden inside GB-07 and noted in NEG-14; the register has no item for it.
4. E.7 lists twelve items and Appendix M.38 lists eleven rows; four are the same correction and M.38's row 8 (a negative control that cannot fail because the approximation over-counts) is a remark E.7 does not carry. Done: NEG-03 cross-references the four and does not duplicate them; NEG-04 implements the control that undercounts.
5. E.7 row 5: the stated inputs (81 designs; rungs of 10 s, 3 min, 30 min; six survivors; three finalists) do not determine the 2.6 h first printed. Done: NEG-08 pins 2.0 h, 2.2 h and 3.0 h with the audit lane reported separately and the audit rung named.
6. E.7 row 7 has no card ("no card uses the row"), so there is no card golden to pin it. Done: NEG-10 pins the three radii on the wheel-loop fixture itself (BD-108, BD-123) and requires one plant model for all three.
7. G.1 gives no degrees of freedom, mass or actuator figures for the seventeen Menagerie robots, and the licences are at the level of one research pass with no commit pinned; the brief asks each robot bead for those facts. Done: each `ROBOT-…` bead states recalled figures as expectations, labelled as such; the manifest's `facts` and `mass_on_file` are computed from the files at a pinned commit and a human-typed figure is a validation error (NEG-15); a robot's rows are not citable by a gate until its licence is read from the primary text at the pin.
8. G.1 lists the sibling library's G1 as a `humanoid` reference; the foundation-defect list says its builder keeps 30 links summing to 31.96314202 kg and omits attachments, against 33.34014202 kg for the 38-link source. Done: ROBOT-fsim-g1 is admitted as the declared reduced profile `g1-reduced-30`, usable as a fast fixture and the M0 smoke model and refused as the reference of the humanoid mass row; ROBOT-g1 is that reference.
9. The "reference robots on file" column of G.4 and the table of G.1 differ: G.4 row 1 omits Barkour, and row 6 lists the sibling's G1 and H1 only, omitting the Menagerie G1 and the five import-parity-only humanoids. Done: the index mapping follows G.1 (NEG-15), with import-parity-only robots kept out of baselines and out of the mass gate.
10. G.2 gives no milestone for third-party assets. Done: the register rows, the user-supplied import paths and the prior-art checks are M3 (with the scene licence manifest); the three redistributable third-party sources (rows 2, 3, 4) are M5 under the real-place ingestion work package (WP-32), after the scene-dressing pilot (N19a-40) and behind KILL-DRESSING, and off the critical path.
11. G.2 rows 11 and 12 are "not read", and no URL was recorded for the two retail generators. Done: NEG-27 keeps all three out of the tree with `read = false`; its first task is to record a primary URL or a recorded "not found"; no file from a not-read source may exist in the repository.
12. G.3 says each row marked [F] is represented "by a Frontier mechanism and by the Solid alternative beside it"; three of the eight such rows (pets, floor vibration, shoppers) print no Solid alternative. Done: until the Frontier mechanism lands those three are marked `not represented` in a pack's manifest and reported uncovered by a battery that needs them; a hazard is never silently dropped (NEG-28, NEG-29).
13. The brief asks for a check that every hazard is covered by a requirement, a shield or a refusal; table G.3 has no such column, and the registries of requirement templates and shield kinds belong to other units. Done: a DESIGN cover vocabulary of 24 codes and a "Covered by" column recorded as data (NEG-28), checked in three parts (non-empty, covers exist, covers can fire on a trigger fixture and stay silent on a control) by NEG-29; a cover whose target has not landed is `pending(Mn)` and does not count. FROB-REF-312 is deliberately not a cover: it guards a claim about the scene, not a failure of the robot.
14. The brief asks for hazard registry beads; the per-archetype registries are already a seed bead of the scene unit (BD-227) with seven beads behind it. Done: NEG-28 is the consolidated index and the check that the seven files agree with it, not a second copy.
15. G.4 row 2 says `quad-sprawl` is "stable and still failing A-11 thermally", while the catalogue gate says every archetype passes stages 1 to 6 with no refusal. Done: the default instance passes; a named variant is the failing fixture (ZOO-quad-sprawl, following the template bead).
16. G.4 rows 3, 4 and 14 give two groups for one archetype (D₆ or C₆; D₈ radial or C₂ spider; C₃ or D₃ per cluster). Done: one default instance each. The hexapod's default is unhanded (D₆) with `variants/handed.fbot` for C₆; the octopod's default is radial (D₈) with `variants/spider.fbot` for C₂; the tri-wheel's default is the handed cluster, and its expectation file records the group orders of both cases with the condition for each.
17. G.4 row 18 names the open-loop pole's closed form and the specification does not write it. Done: ZOO-balancer-2w carries a candidate derivation, marks the row `blocked: derivation` until the derivation file is on record, and supports no gate with it meanwhile.
18. Several archetype rows have no known answer in Appendix M beyond the group on file (KA-270). Done: each `ZOO-…` bead derives its own closed-form rows inline and plants a control for each; they are listed in (c) as new known answers.
19. Appendix M and Appendix N have no sections for Appendix E or G, and neither appendix defines a refusal code. Done: no `-KA`, `-CX` or `-REF` registry bead for this unit; values are inline and counted in (c).
20. Some cards substitute illustrative outcomes that no real run would return value for value (GB-34's 164, 38 and 12 rescues of 200; GB-33's greedy gains; GB-35's residual of 0.04 and score of 0.74; GB-36's variance components). Done: the reference lane recomputes the card's arithmetic; the mechanism lane hands the real routine a `synthetic` table or instance that holds the card's counts and compares the rates, bounds, labels and rankings it returns. Cards that rest on TARGET or FACT inputs (GB-09, GB-16, GB-18, GB-22, GB-24, GB-25, GB-28, GB-40, GB-41) tag each input, and a card with a FACT input not yet re-read from its source reports `citable = false`: its arithmetic runs and no gate may cite it.
