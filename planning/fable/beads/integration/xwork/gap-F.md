# Gap packet gap-F

19 reported gaps. Several groups often reported the same gap: treat those as one.

## Gap 1 (reported by G00)
- plan section: §28.8 and §28.23 (GATE-ATEL-03), with §4 (BET-10)
- what is missing: ablation arms per verb cluster in the agent-surface A/B. BET-10 has no kill gate; its written consequence is that the surface is reduced to the verbs that showed an effect, which needs a per-cluster effect. N28b-11 registers three arms (minimal, full, solve) and BD-374 records per-verb usage only, so nothing measures which clusters showed an effect
- needed by: BET-10

## Gap 8 (reported by G14)
- plan section: §29
- what is missing: the third clause of GATE-SKEL-08 (M1: after a schema migration old runs are replayable) needs replay of a recorded referee run; the verb sim.replay is BD-203 (M2) and replay across a migration is N28b-05 (M3), so no M1 bead delivers it and N14a-29 can only log that clause as partial
- needed by: N14a-29, N14a-18, BD-059

## Gap 10 (reported by G15)
- plan section: §29.10
- what is missing: the failure taxonomy (BD-094) has brownout, thermal-limit and thermal-runaway but no class for battery exhaustion, which BD-075 and N15b-03 raise as an event (FROB-REF-135)
- needed by: BD-075, N15b-03

## Gap 14 (reported by G16)
- plan section: §25.17 (with §19.7)
- what is missing: a scene description for a world with several robots and free props, and its admission by sim.run (or a multi-robot shape class); the facility epic's beads (N25c-28 to N25c-35) compose missions and none builds or admits the multi-robot world that §16.20 steps
- needed by: N16c-19, N16c-20, N16c-21

## Gap 15 (reported by G16)
- plan section: §31 (with §8 and §23.19)
- what is missing: a prototype-budget consumer that refuses a model-conditional ranking with FROB-REF-830; the build packet has BD-408, no bead carries prototype spend
- needed by: BD-266

## Gap 23 (reported by G18)
- plan section: §25 (a probe on a compliant pad, simulated on the referee)
- what is missing: an interface law with time-dependent relaxation: §18 specifies none (CL-1 has one dissipation time; N18b-10 puts a covering's stiffness in series), so no bead of G18 carries it (also reported by G25cde)
- needed by: N25c-17

## Gap 27 (reported by G19)
- plan section: §28.8, §25.17
- what is missing: a design-search entry point that returns the cheapest typed design edit flipping a named verdict, priced from part cards, with the controller obtained by the route the original used; BD-341 optimizes levers against a task metric and N25a-09 returns the smallest relaxing change of a refuted mission
- needed by: N19b-40

## Gap 45 (reported by G24)
- plan section: §29.4
- what is missing: the channel-kind table of BD-090 lists five kinds and no interval extremum, which §24.5 requires; the kind, its downsampling rule and its referee producers are now the new bead N24a-44, and BD-090 and the device telemetry bead BD-207 should name it
- needed by: N24a-44, N24b-05, N24a-06

## Gap 53 (reported by G25ab)
- plan section: §28.4
- what is missing: no analysis bead states that the static analyses (mass roll-up, reach and workspace, actuator envelope, self-clearance, the bound analyses of A-20 to A-23) run over a generic scalar, so that an interval, an affine form or an expression-recording value can stand in for a number
- needed by: N25b-08, N25b-09, N25b-13, N25b-16, N25a-32

## Gap 73 (reported by G28)
- plan section: §25.4 (reported by the Augur integrator against §28.4)
- what is missing: interval and affine forms of the static analyses: §28.4 states no generic-scalar rule, and the plan lists interval forms of the robot static analyses as missing in the suite and places them with the region-certificate work, lowered twice by independent code (PO-AUGUR-01); no bead of §25 or §28 carries that lowering for A-02, A-08, A-10, A-11 and A-13
- needed by: N25b-08, N25b-09, BD-358

## Gap 78 (reported by G29)
- plan section: §29.18
- what is missing: the invariant of §29.22 (thermal and wear states take input only from constitutive dissipation and electrical loss) has no number; N29b-22 carries it as INV-CHRON-14 (proposed)
- needed by: N29b-22

## Gap 79 (reported by G30)
- plan section: §30.5
- what is missing: the dataset manifest frob.ground.dataset/1 of BD-387 lacks fields Ground's components read: the kind of each clock mark; a maintenance log and per-unit operating hours; fiducials, length unit and an external-registration block for scans; optional resolution and bias_sigma per instrument
- needed by: N30a-13, N30a-40, N30a-45, N30a-08

## Gap 94 (reported by G31)
- plan section: §29.5
- what is missing: no bead carries an event-level (T1) device-side capture of one time-coherent sample per checked link per episode at the running maximum of a registered scalar
- needed by: N31b-40

## Gap 100 (reported by G32)
- plan section: §28.17
- what is missing: a third-milestone form of experiment.suggest (the expert grid) that names the experiment deciding between two causes; BD-351 delivers the verb at M4 and the diagnosis battery needs it at M3 for its inconclusive tasks
- needed by: N32b-37

## Gap 102 (reported by G32)
- plan section: §25.3
- what is missing: the mission template `delivery-and-placement` used by `frob mission new --template` in the worked mission session, and a small instance of it (an inner cell, an excluded cell with a witness, an unresolved cell) that synthesizes on the CPU slice; no mission bead carries either, so N32b-16 authors the instance as its own fixture
- needed by: N32b-16, BD-373

## Gap 103 (reported by G32)
- plan section: §29.12
- what is missing: the header of the replay packet has no geometry block: the page cannot draw bodies from poses alone. N32b-40 reads a `shapes` entry (collision primitives per body and analytic scene elements, digest-covered, counted in the size computation) that the parent-only writer N24a-41 does not embed
- needed by: N32b-40, N32b-41, N32b-43

## Gap 120 (reported by G40)
- plan section: §25.27 (register §40.3)
- what is missing: the M3 part of GATE-AUGUR-14 (the register gives M3 and M4): every bead that measures it (BD-354, N25a-27) is M4, so nothing records the gate at the M3 exit
- needed by: BD-319

## Gap 121 (reported by G40)
- plan section: §25.29 (register §40.8)
- what is missing: the finite clause of KILL-COUPLING (a violated transported bound that survives a repair cycle), which the fallback ledger N40-29 reads at M4 beside the finite part of GATE-AUGUR-25: its only decision bead N25e-06 is M8; either an M4 first reading is added or the ledger's M4 reading is withdrawn by amendment
- needed by: BD-378, N40-29

## Gap 124 (reported by G41)
- plan section: Appendix O, §28.20
- what is missing: public refusal codes for `a prediction registered after its outcome exists` and `a frozen prediction changed`: the Atelier registry N28-REF has no rows for them, and BD-344 raises them under the provisional names `prediction-after-outcome` and `prediction-changed-after-freeze`
- needed by: BD-344, N41-REF

