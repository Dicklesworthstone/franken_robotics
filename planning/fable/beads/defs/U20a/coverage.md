# Coverage of drafting unit U20a

Chapter §20 Sensorium: sensors, clocks, observability; the chapter preamble and subsections §20.1 to §20.12; and the seed beads BD-078, BD-098, BD-120, BD-134, BD-188, BD-189, BD-190, BD-237, BD-238, BD-239, BD-240.
60 bead files: 11 seed beads, 48 new beads (`N20a-01` to `N20a-48`) and the registry bead `N20-KA`. `N20-CX` and `N20-REF` are written by U20b.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §20 preamble | Owner crate, layer, evidence labels of the chapter | N20a-01 |
| §20.1 | What it owns | N20a-01 (ownership and what it does not own); each owned item has its bead below |
| §20.2 | Decisions | ADR-SENS-01 and -06: N20a-01; ADR-SENS-02: BD-078; ADR-SENS-03: N20a-05; ADR-SENS-04: N20a-06; ADR-SENS-05: BD-098; ADR-SENS-07: N20a-02; ADR-SENS-08: N20a-24; ADR-SENS-09: N20a-03; ADR-SENS-10: N20a-04; ADR-SENS-11: N20a-01 (statement and reason; mechanism in U20b's symmetry beads BD-241, BD-102); ADR-SENS-12: BD-188 |
| §20.3 | The rule | N20a-01 (sensor as a component; the observation service in the model identity; `FROB-REF-442`, first clause) |
| §20.4.1 | Exact time | BD-078 (lattice, Q, admission, `FROB-REF-440`, phase accumulator, PO-SENS-01) |
| §20.4.2 | Seven instants | N20a-05 (fields, order constraints, device clock, age) |
| §20.4.3 | The boundary ordering rule | N20a-06 (seven stages, PO-SENS-02, half-open windows, reset); N20a-16 (the impulse case through a force sensor) |
| §20.4.4 | Events inside a step | N20a-07 (split, interpolate, hold; lateness; segmented pose history; PO-SENS-03); N20a-12 (OPEN-SENS-1) |
| §20.4.5 | Packets and queues, inside every checkpoint | BD-098 (rings, capacity, `FROB-REF-441`, delivered observations, checkpoint section, three adversarial instants); N20a-08 (link model); N20a-04 (noise-process states) |
| §20.4.6 | A worked schedule, and the shortcut it replaces | BD-078 (the 400/30 schedule; the every-thirteenth-step control) |
| §20.4.7 | Schedule races [F] | N20a-09 (pilot), N20a-10 (mechanism), N20a-11 (KILL-SCHED) |
| §20.4 on every tier | (seed) | BD-189; end-to-end script N20a-44 |
| §20.5 | Proprioception [S] | Encoders: N20a-13 (referee basics in BD-099, U20b); motor current, estimated torque, temperatures, battery, contact switches: N20a-15; IMU chain: N20a-14 (lever-arm kinematics and the at-rest, free-fall and turntable cases in BD-099 and BD-187, U20b); force-torque: N20a-16; wheel odometry: N20a-17; tactile arrays [F]: N20a-18 (pilot), N20a-19 |
| §20.6 | Exteroception [S] | Ray queries, height scans, lidar patterns, labels: BD-187 (U20b); range noise, grazing dropout, range limits: N20a-20; optical class table, `diffuse`, `absorber`, `retroreflector`, `emitter`, unknown region, one-bounce rule: BD-237; `glass`, `mirror`, `specular-floor`: N20a-21; optical volumes: N20a-22; pattern-projecting depth cameras: N20a-23; why causes are not coins: N20a-24 |
| §20.7 | Cameras with exposure and rolling shutter [S from M2] | BD-188 (rows, midpoint, auto exposure, beam instants, GATE-SENS-06); N20a-25 (pilot), N20a-26 (exposure integration [F]), N20a-27 (KILL-EXPOSURE); radiometric RGB [F]: N20a-28; event cameras [F] and the rule for the other Frontier sensor models: N20a-29 (pilots), N20a-30; end-to-end script N20a-45 |
| §20.8 | Sensor failure modes | BD-238 (the whole table as a register, cards, provenance, `FROB-REF-447`, GATE-SENS-14); row owners: N20a-13, N20a-14, N20a-15, N20a-16, N20a-17, N20a-20, N20a-21, N20a-22, N20a-23, BD-188, N20a-08, N20a-05, N20a-31 (extrinsic bump, stuck, dead); BD-237 (optical rows, GATE-SENS-07); N20a-32 (OPEN-SENS-4); end-to-end script N20a-46 |
| §20.9 | Images | Stills for agents [S]: N20a-33 (renderer, identifier buffer, pixel lookup; the verbs are BD-191, U24a); RGB cameras for policies [F] and hero renders [F]: N20a-28 (epic BD-420, U19b) |
| §20.10 | The viewer | N20a-33 (what Sensorium supplies; the no-feedback rule; INV-SENS-01 at unit level; the viewer page is BD-300, U33a) |
| §20.11 | The observability Gramian [S] | BD-120 (linearized, gauge, corridor, global-ambiguity flag); BD-239 (empirical form, agreement); N20a-34 (verb, route profile, design-check warning, shield input); N20a-35 (placement pilot), N20a-36 (placement score [F] and its kill rule); end-to-end script N20a-47 |
| §20.12 | The observation-alias impossibility witness | N20a-37 (object, statement PO-SENS-07, example, checker, certificate); N20a-38 (structural source); N20a-39 (symmetry and repetition); BD-134 (enumerated source; late-packet docking witness); N20a-40 (noisy sensing, deadline); N20a-41 (verb, three engineering answers, four kinds of change, effect on other verbs, `FROB-REF-445`); BD-240 (searched source, GATE-SENS-09); N20a-42 (pilot and ceiling measurement), N20a-43 (KILL-ALIAS); end-to-end script N20a-48 |
| §20.16 (owned by U20b; read for BD-134, BD-190) | The docking fixture | BD-190 (grid, three arithmetics, late packet, third gain, GATE-SENS-13, with its own end-to-end script); BD-134 (the late-packet witness) |
| Appendix M.19 | Known answers of §20 | N20-KA |

Identifiers carried. ADR-SENS-01 to -12 as in the §20.2 row. INV-SENS-01 (N20a-01, N20a-33), -02 (N20a-08, N20a-06), -03 (BD-098), -04 (BD-078, N20a-07), -05 (N20a-02), -06 (N20a-04), -07 (N20a-03); INV-SENS-08 and -09 belong to U20b's subsections (INV-SENS-09 is cited in BD-189 and N20-KA). PO-SENS-01 (BD-078), -02 (N20a-06), -03 (N20a-07), -04 (N20a-21), -06 (BD-190), -07 (N20a-37), -09 (BD-120, BD-239); PO-SENS-05 and -08 belong to U20b. OPEN-SENS-1 (decision bead N20a-12), -2 (BD-078, Considerations, with a proposed menu), -3 (N20a-03, N20a-05, Considerations), -4 (decision bead N20a-32; BD-238), -5 (N20a-21, BD-237, Considerations). Constructions C-19 (BD-078, N20a-05, N20a-09, N20a-10, N20a-37, BD-134, N20a-40; `SPEC-C-19` in deps) and C-20 (BD-120, BD-239; `SPEC-C-20`), C-07 (N20a-17). Theorem cards T-23 (BD-190, BD-134), T-30 (N20a-37; `CARD-T-30`). Analyses A-17 (BD-120, N20a-34), A-14 and A-21 (named where they consume this unit's outputs). Goals GOAL-19, GOAL-30, GOAL-31; North-Star scenario NS-22 (BD-134). RISK-45 (BD-189).

## (b) Gates and kill gates of §20, and the beads that measure or decide them

| Gate | Measured or decided by (this unit) | Expected from another unit |
|---|---|---|
| GATE-SENS-01 | — | BD-099, BD-187 (U20b). This unit supplies N20a-14 and N20a-20, whose checks re-run its cases through the full chains |
| GATE-SENS-02 | — | BD-187 (U20b) |
| GATE-SENS-03 | — | BD-215 (U20b). Unit-level forms in N20a-01, N20a-04, N20a-33 |
| GATE-SENS-04 | BD-078 (CPU rows), N20a-06, N20a-07, N20a-16 (impulse and arrival rows), BD-189 (every tier), N20a-12 (default policy), N20a-44 (script) | — |
| GATE-SENS-05 | BD-098 (referee rows), BD-189 (every tier), N20a-44 | — |
| GATE-SENS-06 | BD-188, N20a-07, N20a-45 | — |
| GATE-SENS-07 | BD-237, N20a-21, N20a-22, N20a-23, N20a-24, N20a-46 | — |
| GATE-SENS-08 | BD-120 (linearized, M1 rows), BD-239 (empirical, agreement), N20a-34, N20a-47 | — |
| GATE-SENS-09 | BD-134 (docking rows, M1), N20a-37, N20a-38, N20a-39, N20a-40, N20a-41, N20a-42, BD-240, N20a-48 | — |
| GATE-SENS-10 | — | BD-241 (U20b) |
| GATE-SENS-11 | — | BD-242 (U20b) |
| GATE-SENS-12 | — | BD-119, BD-215 (U20b) |
| GATE-SENS-13 | BD-190 | — |
| GATE-SENS-14 | BD-238, N20a-08, N20a-13, N20a-14, N20a-15, N20a-16, N20a-17, N20a-20, N20a-31, N20a-46 | The extrinsic-bump row's first check is BD-242's (U20b) |
| GATE-SENS-15 | — | U20b (§20.23); N20a-16 exposes the hooks the ordered chain needs |
| GATE-SENS-16 | — | U20b (§20.24) |
| GATE-SENS-17 | — | U20b (§20.24) |
| GATE-SENS-18 | — | U20b (§20.25) |
| KILL-SCHED | N20a-11 (decision), N20a-09 (pilot), N20a-10, BD-189 (records it) | — |
| KILL-EXPOSURE | N20a-27 (decision), N20a-25 (pilot), N20a-26, BD-188 (records it), N20a-45 | — |
| KILL-ALIAS | N20a-43 (decision), N20a-42 (pilot), BD-240 (records it), N20a-48 | — |
| KILL-VISIBILITY | N20a-35 (the shared pilot that supplies one of its two triggers) | The decision bead: U20b (§20.24) |
| Placement kill rule of §20.11 (no identifier in the gate register) | N20a-36 | — |

## (c) Counts

- Known answers: 60 (KA-465 to KA-524), all in `N20-KA`; every one is cited by at least one mechanism or test bead of this unit or assigned to a U20b seed bead in the registry's owner column. 18 have a tolerance "to set" and 35 no stated control at plan time; owner beads of this unit propose tolerances and controls where they own the row.
- Counterexamples cited by this unit's beads: 12 of the chapter's 16 (CX-200 to CX-208, CX-213, CX-214, CX-215), plus CX-104 of the conventions chapter. CX-209 to CX-212 belong to U20b's subsections. The registry `N20-CX` is U20b's.
- Refusal codes: 5 of the chapter's 7 are specified here with trigger, repairs and exit code 3: `FROB-REF-440` (BD-078), `-441` (BD-098), `-442` (N20a-01 and N20a-03, one clause each), `-445` (N20a-41), `-447` (BD-238); also `FROB-REF-120` of the design chapter as used by the information stage (N20a-41). `-443` and `-446` belong to U20b. The registry `N20-REF` is U20b's.
- Gates measured or decided by this unit: 8 convergence gates (GATE-SENS-04 to -09, -13, -14) and 3 kill gates, plus the pilot input to KILL-VISIBILITY.
- Subsections: 12 (§20.1 to §20.12) plus the preamble, with the seven sub-subsections of §20.4.
- Pilots: 7 (N20a-09, -18, -25, -29, -35, -42, and the RGB epic's own pilot referenced by N20a-28). Kill-gate decisions: 3. Owner decisions: 2 (N20a-12, N20a-32). End-to-end scripts: 5 beads (N20a-44 to N20a-48) plus the docking fixture's script inside BD-190.

## (d) Needs from other slices (no seed key could be named)

- §11.8 (U13): a ruling on the exposure-window convention; see (e) 1.
- §20.13, §20.14, §20.15, §20.17 (U20b): the symmetry subgroup's recomputation after a sensor move (N20a-36), the calibration self-check as the first check of the extrinsic bump (N20a-31, BD-238), `sense.causal-test` (N20a-31, N20a-46), the dependency analysis behind noninterference (N20a-38).
- §20.22 (U20b): if U20b also writes decision beads for OPEN-SENS-1 or OPEN-SENS-4, they duplicate N20a-12 and N20a-32 and the lead should keep one of each.
- §20.23, §20.25 (U20b): the ordered force-sensor chain and grades of force history, the load-cell cut, the integrating accelerometer's jump term, and sampled-encoder unwrapping, which extend N20a-16, N20a-14 and N20a-13; their known answers have no KA number yet (N20-KA).
- §20.24 (U20b): the visibility query and the KILL-VISIBILITY decision that reads N20a-35.
- §15: transmission card backlash and gear ratio; actuator bus and driver latency; true electrical and thermal quantities (N20a-05, N20a-13, N20a-15).
- §16, §13: the integrator's retraction and per-step twist per link; the dyadic definition of noise over a base interval; batch creation from explicit initial states with open-loop command replay (N20a-07, N20a-04, BD-239).
- §18, §19.10.2: slip distance in contact telemetry; a film patch (N20a-17).
- §19: the optical map and optical volumes of a compiled scene; zone lighting levels and emitters; routes; the interior grammar's record of repeated and mirrored modules; hazard-list speed ranges (BD-237, N20a-22, N20a-23, BD-188, N20a-34, N20a-39, N20a-25).
- §21, §35: the exact-integer and guarded kernel classes, the shared radix sort and compaction kernels, the deterministic exponential, the Philox port and stream registry (BD-189, BD-190, N20a-22, N20a-04, N20a-26).
- §22: the estimator's error-state manifest and Φ_k; `--state estimated` and `--state privileged`; the aligned drift metric; shield assumption monitors (BD-120, N20a-34, N20a-35, N20a-36).
- §23, §24: observation and critic schemas of the gym; the M3 mission corpus; registered tasks that would need a Frontier sensor model (N20a-03, N20a-42, N20a-18, N20a-29, N20a-30).
- §25: the docking family's game and comparator; pricing of separating changes; channel laws for partially dependent reads (BD-134, BD-190, N20a-41, N20a-40).
- §26, §27: the adversarial search's legality set and proposal machinery; forks with interventions (N20a-31, BD-240, N20a-42, N20a-35).
- §28: the memo graph, the optimizer, revalidation, the analysis registry (N20a-34, N20a-36, N20a-41).
- §29, §32: snapshot and checkpoint containers, telemetry query and divergence, mask-reason histograms, the stills and viewer surfaces (BD-098, N20a-02, N20a-33, the end-to-end beads).
- §38, §40: the pilot register, the gate register and its ADR rule, the known-answer registry file and meta-test (pilot and decision beads, N20-KA).
- §42, §30, Appendix G: the joint owner decision on catalogues and licences; admission of measured data (N20a-32).

## (e) Contradictions and things that could not be turned into work as written

1. **Exposure-window convention.** §11.8 (and its rows KA-204, CX-104, as drafted by U13, U10b and U12) states windows as (a, b] with "an impact at exactly instant b belongs to the window ending at b". §20.4.2 and §20.4.3 (and KA-475, CX-201, §20.25 and GATE-SENS-18) state [open, close) with "an impulse exactly at t belongs to the window that opens at t". Both cannot hold, and U13's bead N13-06 would test a `frob-sense` accumulator against the opposite boundary case. What I did: implemented the Sensorium convention behind one membership function (N20a-06), which also matches the step's base intervals [t0, t1); flagged the pair of rows `unreconciled` in N20-KA; named the ruling as a `needs` of N20a-06. The lead must rule.
2. **Third-gain amendment on the 561-cell grid.** §20.16 and GATE-SENS-13 say the amendment "reverses every admitted cell". With gains {1, 33/32, 2}, separability needs ε < p/64 ≤ 3/256, so every admitted cell with ε > 0 reverses (327 of 344), and the 17 cells with ε = 0 stay admitted (an exact reading separates any two gains). BD-190 tests 327 and 17 and records the discrepancy; the gate wording should read "every admitted cell with positive error".
3. **"M1 (docking rows)" on GATE-SENS-08.** The register gives GATE-SENS-08 (observability: corridor and gauge) an M1 part labelled "docking rows", but the gate has no docking fixture. I read it, with the seed bead BD-120, as "the linearized rows on the CPU, needed by the mission slice at M1".
4. **Verb milestones.** The verb registry gives `sense.schedule` M2 and `sense.observability` and `sense.alias` M3, while seed beads at M1 name them (BD-078, BD-120, BD-134). I treated M1 as the reduced form on the referee and CPU and wrote the full verb surfaces at their registry milestones (BD-189 for the schedule on every tier; N20a-34; N20a-41).
5. **PO-SENS-02 under duplication.** The ordering key (stage, arrival, sensor identifier, sequence number) has ties when the link duplicates a packet, because a duplicate shares its sequence number. N20a-06 and N20a-08 add a copy index to the key (labelled DESIGN).
6. **Exposure integration against its own kill threshold.** Two or four sub-instants on every pixel add 100% to 300% rays, far above the 25% of KILL-EXPOSURE. N20a-26 proposes candidate-only casting with a fixed capacity (labelled DESIGN), and N20a-25 and N20a-27 carry the arithmetic. The plan gives the Frontier row no milestone; I placed the mechanism at M3 and the decision at M2 (the register's milestone), decided on the pilot's estimate and re-armed on the measured cost.
7. **"The causal-policy harness sees the mask" for a stuck value.** A stuck value is a number, not a masked sample. N20a-31 defines a dead sensor as masked samples (`fault: dead`), a stuck sensor as frozen values with an optional annunciation flag, and applies the first check to the dead sensor and the annunciated stuck sensor.
8. **Names and thresholds the plan leaves open.** Nine mask-reason names (N20a-02), the warning threshold λ_warn of the design check (N20a-34), the menu of declared clock rationals (BD-078), the fixtures and falsifiers of the three unfunded Frontier sensor models (N20a-29), the falsifier of the tactile pilot (N20a-18), and tolerances for the rows marked "to set" are proposals, each labelled as such in its bead.
9. **Sub-step pose of a distal sensor.** §20.7 says the pose inside a step is the previous pose "advanced along that step's twist by the integrator's own retraction" at a cost of one retraction per row. That is exact for a sensor on the floating base; for a sensor on a distal link the integrator implies an articulated path. N20a-07 defines the per-link step twist from the two committed poses (so both ends agree, as PO-SENS-03 requires) and puts the interior deviation under the declared bound of `interpolate`, with a property test and a stated fallback.
10. **T-junction at M1.** Ray sensors reach the referee at M2, so the T-junction cannot be simulated with a lidar when the witness checker lands at M1. N20a-37 uses an exact abstraction at M1 and BD-240 and N20a-48 run the simulated-lidar form at M3.
11. **Open questions listed in U20b's subsection.** OPEN-SENS-1 to -5 are listed in §20.22 and concern only mechanisms of §20.4 to §20.8. I wrote decision beads for the two that block work (OPEN-SENS-1, OPEN-SENS-4) and carried the other three in Considerations with their fail-closed defaults.
