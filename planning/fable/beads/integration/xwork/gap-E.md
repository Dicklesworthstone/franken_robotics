# Gap packet gap-E

22 reported gaps. Several groups often reported the same gap: treat those as one.

## Gap 11 (reported by G15)
- plan section: §24.10
- what is missing: the input of task.fixture (N24b-03) has fixture, layer, rows, tier, designs and control; the Sinew scripts also pass an actuator level, a measured row, a precision and a seed
- needed by: N15a-12, N15a-24, N15b-13, N15b-30, N15b-32, N15b-35, N15b-37

## Gap 17 (reported by G17)
- plan section: §19 (used by §17.11)
- what is missing: a scene-family operation that maps a world address through an isometry g (the address whose scene is the image under g), where the family supports it; BD-241 builds image worlds for its equivariance test but returns no address
- needed by: N17a-24, BD-286

## Gap 18 (reported by G17)
- plan section: §24.8
- what is missing: a smoothness attribute on registered metrics (smooth accumulation, margin, binary outcome, count); the metric registry N24a-17 has none, so the gradient admission test would have to guess
- needed by: N17b-22

## Gap 19 (reported by G17)
- plan section: §24.4
- what is missing: an explicit declared read set of design fields in the task file; BD-122 has no such field and N17a-26 derives the set from the file's terms
- needed by: N17a-26

## Gap 20 (reported by G17)
- plan section: §23.13 (code of §17.16)
- what is missing: the weight-tie raising site of FROB-REF-161: N23a-31 refuses a group outside Γ_obs by a type barrier and another family's refusal and does not raise the registered code
- needed by: N17-REF

## Gap 22 (reported by G18)
- plan section: §22.11 (W_abs is read from 'an interface card with provenance')
- what is missing: an absorber work-envelope card or card field (force–displacement lower envelope, stroke, force limit): the interface card of BD-167 is keyed by material pair and condition and carries friction, stiffness and rolling data only, and §18 specifies no absorber, so no bead of G18 carries it (also reported by G22)
- needed by: N22b-11, N22b-17

## Gap 31 (reported by G20)
- plan section: §24
- what is missing: no Arena bead gives observation, action, reward and requirement terms a declared transformation kind, nor a task's goal set an invariance group (BD-122 and BD-194 have neither; N14a-11 expects the same from BD-122)
- needed by: N20b-01

## Gap 32 (reported by G20)
- plan section: §19
- what is missing: no Habitat bead lets a scene family declare the group of world isometries under which its distribution is invariant in law
- needed by: N20b-01

## Gap 46 (reported by G24)
- plan section: §19.3
- what is missing: a declared pinch zone on closing fixtures (doors, lift gates, conveyor nips): the fixture library N19a-07 carries the closer law and no pinch-zone field
- needed by: N24a-38

## Gap 47 (reported by G24)
- plan section: §19.3 (with §24.20)
- what is missing: a corridor query on a scene pack's route graph (signed distance from a body footprint to the boundary of a corridor of declared half-width) recorded as a telemetry channel; N19a-04 carries the route graph as data only
- needed by: N24c-21

## Gap 48 (reported by G24)
- plan section: §22.12.3
- what is missing: milestone conflict: the backup controllers per morphology class are needed as plain controllers at M3 (third idle law of GATE-ARENA-15), and the bead that builds them, N22b-24, is M4
- needed by: N24c-27

## Gap 49 (reported by G24)
- plan section: §19 (with §24.22)
- what is missing: milestone conflict: the per-world cache of field samples in frob-scene is needed by the M2 fixture half of GATE-ARENA-14, and the per-world scene state pools of N19a-09 are M3
- needed by: N24c-36, N24c-38

## Gap 58 (reported by G25cde)
- plan section: §19.12
- what is missing: retrofit kit fields for installation time, access constraints, calibration, tolerances and maintenance, which the priced world-edit grammar of §25.17 reads
- needed by: N25c-31

## Gap 61 (reported by G26)
- plan section: §19.12
- what is missing: Habitat's typed scene edit (closed set of edit kinds, application to an immutable pack giving a new SceneId, the declared-aspect check) exists only in N19b-38 at M4, behind the retrofit pilot and the feature frontier-retrofit; §26.1 has Nemesis consume it at M3. Needed: an M3 Solid bead split out of N19b-38, on which N26a-02 and N19b-38 both depend
- needed by: N26a-02

## Gap 62 (reported by G26)
- plan section: §19.7
- what is missing: BD-225 and BD-226 own the grammar's production and attribute draws but do not state an export of the probability mass of each discrete scene feature (a film present, a cart in an aisle); N26a-03 and N26a-24 need it for the prior of a discrete edit and for exact mixture weights, and meanwhile mark such features undefined_for
- needed by: N26a-03, N26a-24

## Gap 66 (reported by G27)
- plan section: §20.4
- what is missing: frob-sense exposes no operation that rebuilds a packet queue at a given tick under a changed link law (in-flight packets keep sampling instant and value and receive a new delivery instant) or re-initializes a filter state from a stated source; N27a-05 specifies the behaviour for information forks and has to reach into the queue layout of BD-189. Needed as an entry point beside BD-189 and N20a-08
- needed by: N27a-05

## Gap 67 (reported by G27)
- plan section: §23
- what is missing: no bead delivers a trained reference humanoid policy evaluated on a device tier by M3 (BD-198 trains the quadruped only). GATE-AUTOPSY-09 records G(W) for the reference quadruped and humanoid policies; without the policy N27b-28 records the humanoid sub-clause as not run
- needed by: N27b-28, N27b-22

## Gap 111 (reported by G35)
- plan section: §24 and §23 (GATE-CORTEX-10)
- what is missing: decision records (frob.verdict/1) on batched tiers are written from M3 (BD-255); GATE-SPARK-16 compares task decisions on device placements at M2, so until then verdicts must be evaluated on the host from recorded traces
- needed by: N35-09, BD-177

## Gap 115 (reported by G38)
- plan section: §19.7, §40.4
- what is missing: the M6 re-read of KILL-DRESSING on the actor mechanism (cost above 25%, a broken launch shape or determinism class) has no decision bead: N19a-41 decides at M5, before the mechanism exists; BD-421 requires the actors' decision as an ADR with measured values
- needed by: BD-421, N38a-33

## Gap 118 (reported by G40)
- plan section: §23.23 (register §40.3)
- what is missing: GATE-CORTEX-17 is named only by epics (BD-439, WP-79, EPIC-M8): no task bead measures the three-arm comparison of geometric-algebra tree policies; the pilot N23b-01 and the decision N23b-02 exist
- needed by: BD-442

## Gap 123 (reported by G41)
- plan section: Appendix O, §23.19
- what is missing: a public refusal code for `randomization requested on a card field with no interval`: the rule is stated in words, the Cortex registry N23-REF has no row for it, and BD-200 raises it under the provisional name `randomize-no-interval`
- needed by: BD-200, N41-REF

## Gap 131 (reported by GF)
- plan section: Appendix F.1, §22.20
- what is missing: a register row and an FD- identifier for the upstream item N22c-22 (input-affine EDMD fit in fs-surrogate), which §22.20 calls an upstream change without listing it in F.1; proposed FD-FSIM-39, after FD-FSIM-34 to -38 proposed for NF-07 to NF-11; neither BD-037 (53 rows) nor N33c-47 (six more) writes it
- needed by: N22c-22

