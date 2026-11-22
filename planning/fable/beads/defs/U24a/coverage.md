# Coverage of drafting unit U24a

Chapter §24 (Arena: tasks, requirements, fixtures, metrics, qualification), preamble and §24.1 to §24.8, plus the
ten seed beads assigned to the unit and the chapter's known-answer registry. 54 bead files: 10 seed beads, 44 new
beads (`N24-KA`, `N24a-01` to `N24a-43`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §24 preamble | Crates `frob-spec` and `frob-arena`, both in layer 4 | N24a-01 |
| §24.1 | What Arena owns | N24a-01 (ownership, the two crates, the boundary table of seven concerns, the note on derived numbers); N24a-13 (what Nemesis, Autopsy, Cortex and Reflex read); N24a-31 (Arena enforces the scene split at scoring time, GATE-ARENA-02); BD-121 (the language crate assembled) |
| §24.2 | Decisions | N24a-01 (ADR-ARENA-01 to -10, each with reason and rejected alternative); restated where they govern: ADR-ARENA-01 in N24a-13, N24a-14, N24a-15, BD-257; -02 in N24a-04, N24a-09; -03 in N24a-05; -04 in N24a-07, N24a-08, N24a-09; -05 in N24a-10; -06 in N24a-06; -08 in BD-260; -09 in N24a-22, BD-253, N24a-25; -10 in N24a-32. ADR-ARENA-07 (one study type) is governed by the study-type beads of unit U24b |
| §24.3 | Task families | N24a-23 (registry of the thirteen families and the shipping rule); BD-201 (velocity tracking); N24a-25 (terrain traversal); BD-253 (interior navigation); N24a-26 (payload transport); N24a-27 (low-friction crossing); N24a-28 (tight-space maneuvering); BD-368 with N24a-43 (moving surfaces); N24a-29 (stair and curb climbing for wheels and tracks); N24a-30 (reach and carry); N24a-24 (reproducibility of every family, GATE-ARENA-01). Stand and recover, threshold delivery and timed docking are seed BD-125 (U24c); split-grip landing is seed BD-202 (U24b) |
| §24.4 | The task file `.ftask` [S] | N24a-02 (schema `frob.task/1`, nine sections, canonical JSON, `TaskId`); N24a-03 (S-expression view and the requirements grammar); N24a-16 (the `reset` section); N24a-06 (the `clocks` and `signals` sections as the checker reads them); N24a-42 (end to end). Verbs `task.new`, `task.show`, `task.list` are seed BD-122 (U24c) |
| §24.5 | The requirement language [S] | BD-121 (core assembled, differential campaign); N24a-04 (signals, atoms, margin table, laws); N24a-05 (units, scales, exact atoms, the composition counterexample); N24a-06 (clocks, three rules, `sampled` and `interval` labels); N24a-07 (online block kernels, capacity 4,096); N24a-08 (referee deque monitor); N24a-09 (offline evaluation, reach, `undetermined`, decided-early); N24a-10 (boundary guard 2⁻²¹ and exact re-evaluation); N24a-11 (verdict `frob.verdict/1`, four values, purity; stages and joint accounting); N24a-12 (labelled examples, mutation check, vacuous atoms); N24a-33 (paper pilot); N24a-32 and BD-256 (requirement writing, GATE-ARENA-09); N24a-42 (end to end) |
| §24.6 | Requirements and rewards | N24a-13 (the eight uses, hard limits frozen, the curriculum rule); N24a-14 (the optional added reward term [F]); BD-257 (its pilot under three arms); N24a-15 (KILL-SPEC-REWARD decision) |
| §24.7 | The settled-pose reset cache [S] | N24a-16 (contents, settling criterion, selection, failure, `task.settle` on the CPU, FROB-REF-529); BD-195 (batched tiers, `VecEnv` reset, GATE-ARENA-08) |
| §24.8 | Metrics and scorecards [S] | N24a-17 (metric registry; success, fall and collision rates; requirement margin; time to verdict; the quantile sample-count rule); N24a-18 (cost of transport; energy per completed task; the two-figure example); N24a-19 (tracking error, Froude number, actuator utilization, tail robustness); N24a-20 (friction utilization, slip distance and its quadrature invariance, clearance); N24a-21 (maneuver cost, fallback count); N24a-22 (scorecard, seven outcome columns, two denominators, battery identity); N24a-24 (`arena_scorecards.sh`); N24a-37 (the energy view the metrics read) |

Material outside §24.1 to §24.8 that the unit's seed beads draw on, and where it landed:

| Source | Content | Beads |
|---|---|---|
| §29.10 | Failure taxonomy: classes, kinds, first cause | BD-094 (mechanical classes, thermal runaway, brownout, `telemetry.failures`); N24a-38 (the ten remaining world and perception classes) |
| §29.12 | Summaries an agent can afford; the self-contained replay | N24a-35 (`telemetry.summary`); N24a-41 (`telemetry.export-html`) |
| §29.4, §29.5 | Columns, channel kinds, pinned pagination | N24a-36 (`telemetry.query`) |
| §29.9, §29.22, §15.11 | The energy ledger view, residual and algorithmic-exchange lines, audit flags | N24a-37 (`telemetry.energy`) |
| §20.9, §32.15 | Stills for agents; overlays on stills; the quantity under a pixel | BD-191 (`telemetry.stills`); N24a-40 (`telemetry.inspect-pixel`) |
| §32.24 | The offline replay packet | N24a-41 (writer, size rule, refusals, known answers); its third-milestone gate and kill gate stay with the unit that owns §32 |
| §24.13, §24.17 | The 500-retry gate on no-signal worlds | BD-260; N24a-34 (world families and adversary) |
| §24.17 | Gates ARENA-01, -02, -03, -08, -09; the kill gate; the two pilots | See table (b) |
| §19.3, §18.9 | Moving surfaces and their boundary work | N24a-43, BD-368 |
| Appendix M.23 | Known answers KA-594 to KA-623 | N24-KA |

End-to-end beads (one per user-visible capability): N24a-42 (requirement pipeline), N24a-24 (tasks, metrics and
scorecards), N24a-39 (telemetry read verbs), BD-191 (stills, pixel inspection, replay export:
`telemetry_stills.sh`), BD-195 (`gate_arena_08_reset_cache.sh`), N24a-31 (`gate_arena_02_scene_split.sh`), BD-256
(`gate_arena_09_requirement_writing.sh`), BD-260 (`gate_qual_01_retries.sh`); each task-family bead owns
`task_family_<name>.sh`.

Pilot beads: N24a-33 (requirement writing on paper, before the admission rules of N24a-12 are fixed); BD-257 (the
Frontier reward term; N24a-14 depends on N24a-15, which depends on BD-257).

## (b) Gates, kill gates and admission gates

| Gate | Milestone | Measured or decided by (this unit) | Notes |
|---|---|---|---|
| GATE-ARENA-01 | M2 | N24a-24 (the gate harness); BD-201, N24a-25 (M2 families); BD-253, N24a-26, N24a-27, N24a-28 (M3 families); BD-368, N24a-29, N24a-30 (M4 families) | The M1 families are measured by seed BD-125 (U24c) |
| GATE-ARENA-02 | M3 | N24a-31 | |
| GATE-ARENA-03 | M1 (host half) | BD-121 (monitor rows); N24a-42 (the script and receipt, both halves of the host gate); N24a-05, N24a-06, N24a-07, N24a-08, N24a-09, N24a-10, N24a-12 (their rows) | Refusal rows need seed BD-122 (U24c). The device half waits on OPEN-ARENA-5 (decision bead in U24c) and seed BD-255 |
| GATE-ARENA-07 | M3 | N24a-22 (the accounting mechanism and its property tests) | Gate measurement on batteries: seed BD-254 (U24b) |
| GATE-ARENA-08 | M2 | BD-195 | CPU core in N24a-16 |
| GATE-ARENA-09 | M3 | BD-256; N24a-32 (corpus); N24a-33 (paper pilot) | |
| GATE-QUAL-01 | M3 | BD-260; N24a-34 | Lane mechanism: seed BD-259 (U24b). Blocked for real campaigns by OPEN-ARENA-3 |
| KILL-SPEC-REWARD | M3 | N24a-15 (decision); BD-257 (measurement); N24a-14 (the branch that survives only if the gate does not fire) | |
| GATE-CHRON-08 (owned by §29) | M2; referee rows at M1 | BD-094, N24a-39 (failure-taxonomy half, thirteen classes, referee); N24a-38 (the ten further classes) | The created-energy half is seed BD-208 |

Gates of the chapter that this unit expects another unit to carry (their mechanisms are in §24.9 to §24.23):

| Gate | Expected owner |
|---|---|
| GATE-ARENA-04, GATE-ARENA-05, GATE-ARENA-06 | U24b (fixtures 1 and 2, `eval.preanalyze`). N24a-20 supplies the slip metric whose invariance GATE-ARENA-06 tests |
| GATE-QUAL-02, GATE-QUAL-03 | U24b (firewall; replay by the offline verifier) |
| GATE-ARENA-10, -11, -12, -13, -14, -15 | U24c (§24.19 to §24.23) |
| KILL-FOIL, KILL-ITEMBANK, KILL-EXPODOMAIN | U24c (§24.20, §24.21, §24.22) |

## (c) Counts

- Known answers: 30 in the registry `N24-KA` (KA-594 to KA-623, every row of Appendix M.23, with formula or value,
  tolerance, tier, negative control and owning beads). 17 of them are also stated with their values in mechanism
  or test beads of this unit (KA-594 to KA-602, KA-605, KA-606, KA-611, KA-615, KA-619 to KA-622). Four known
  answers of other chapters are cited where used: KA-593, KA-456, KA-505, KA-413.
- Counterexamples: 7 of the chapter's 11 are stated with their numbers in this unit's beads (CX-259, -260, -261,
  -263, -267, -268, -269); CX-196 and CX-363 of other chapters are cited. The registry `N24-CX` is written by
  U24c.
- Refusal codes: of the chapter's 21, 12 are carried with trigger and repair in mechanism beads of this unit (502,
  520, 523, 525, 526, 528, 529, 535, 536, 538, 539, 840) and 8 more are named where they bound a mechanism (530,
  532, 552, 841 to 845); 533 is not used here. Codes of other chapters carried: 315, 440, 554, 888, 891 to 893,
  896. The registry `N24-REF` is written by U24c.
- Invariants and proof obligations of the chapter carried here: INV-ARENA-01, -02, -03 (and -04 stated in
  N24a-01); INV-QUAL-01 to -04 in BD-260; PO-ARENA-01, -04 (BD-121, N24a-04, N24a-07, N24a-08), PO-ARENA-02
  (BD-260, N24-KA). INV-ARENA-05 and PO-ARENA-03 belong to the study and pre-analysis beads of U24b.
- Architecture decisions: ADR-ARENA-01 to -10, all in N24a-01 and in the beads they govern.
- Open questions: OPEN-ARENA-1, -2, -3, -5 appear in the Considerations or `needs` of the beads they concern;
  their decision beads are written by U24c, which owns §24.18. OPEN-ARENA-4 concerns fixture 2 only (U24b).
- Risks named: RISK-14, RISK-15, RISK-34, RISK-44.

## (d) Needs from other slices

Prerequisites for which no seed key could be named (also in each bead's `needs` field):

| Bead | Need |
|---|---|
| BD-191, N24a-40 | §20.9: the still renderer, identifier buffer, range buffer and pixel lookup in `frob-sense` (drafted by U20a as a new bead) |
| BD-191, N24a-40 | §32.15: overlay functions owned by other subsystems, each a pure function of a snapshot with channel, unit and legend |
| BD-191 | §37: the privacy class of a scene and of its artifacts |
| N24a-41 | §32.15: the viewer page within its 600,000-byte budget; §32.24: GATE-TETHER-16 and KILL-REPLAYPACK at the third milestone; §27.4 and §27.17: fork classification, harness gates and the contrast table; registry codes for the four packet refusals |
| N24a-37 | §29.22: the per-row quantities `algorithmic exchange` and `constitutive dissipation` from the contact and joint-limit rows |
| N24a-38 | §19.10 slosh state; §22.9 stride events and return-map deviation; §30 drift monitors and `retired` events; §19.3 pinch zones with closing-force limits |
| N24a-42 | §20.16: host-side recorded runs of the exact docking mission at the first milestone (the device form is seed BD-190, second milestone) |
| N24a-02 | §24.22: the exposure-domain field and the versioned exposure table in the task identity; §23.26: the digest of the actor's information contract |
| N24a-07 | §16: the batched step's stage list and lockstep rule, to register the monitor kernel in the last stage |
| N24a-10, BD-121 | §24.18: the decision on OPEN-ARENA-5 (guard band per backend) |
| BD-260 | §24.18: the owner decision on OPEN-ARENA-3 |
| N24a-16 | §24.14: registration of `task.settle` in the first-milestone verb set |
| N24a-22 | §26.5: infeasibility witnesses for the two infeasible columns; §24.9: battery definitions |
| BD-201 | §35.9 and §24.22: a random stream keyed by physical time for command switches |
| BD-253 | §19.3: route graphs, goals, task regions and articulated doors; §26.5: necessary-condition witnesses |
| BD-256 | §32.13: namespaces and the path jail that hide the labelled trajectories |
| N24a-26, N24a-30 | §15: payload, battery and gripper cards (mass, inertia, attachment, retention limit) |
| N24a-27 | §19.8: kitchen hazard rows for wet and greasy floors |
| N24a-30 | §28: analysis A-24, task-wrench attainability |
| N24a-31 | §17: the `DesignSplit` of generated morphologies |
| N24a-43 | §16.5: the transactional step's credit of boundary work from prescribed surfaces at event time |
| N24-KA | §24.19 to §24.23: known answers of the late subsections, which have no row in Appendix M |

## (e) Contradictions, gaps and what was done

1. **Overlap with the sibling units of the chapter.** The three units drafted concurrently and several of this
   unit's beads meet beads of U24b and U24c on the same mechanism: N24a-15 and N24c-12 (the KILL-SPEC-REWARD
   decision); N24a-33 and N24c-11 (the requirement-writing paper pilot); N24a-12 and N24c-01 (labelled examples
   and the mutation check inside `task.check`); N24a-11 and N24c-02 (`frob.verdict/1` and `task.verdict`);
   N24a-16 and N24c-03 (`task.settle`); N24a-31 and N24c-05 (the scene-split check); N24a-42 and N24c-06 (end to
   end of the task-file verbs); N24a-24 and N24c-07 (end to end of task families); N24a-20 and N24b-11 (the slip
   metric); N24a-17 to N24a-22 and BD-254 (metrics and scorecard accounting); N24a-34 and N24b-18 (the campaign
   simulator and adversary). In each pair this unit's bead holds the mechanism in the crate (or the gate harness
   its seed owns) and the other holds the verb, the fixture or the battery-level gate. The lead should either
   merge each pair or make one depend on the other; nothing was removed here, so that no content is lost.
2. **`task.settle` has two milestones.** The verb table places it at the first milestone; the only seed that names
   it (BD-255) is at the third. N24a-16 delivers the cache core and the verb on the CPU at the first milestone;
   BD-195 delivers the batched form at the second.
3. **Seed beads that bundle several families or verbs.** BD-201, BD-253 and BD-368 each name several task
   families; each seed bead carries one family and the gate for its milestone, and depends on new beads for the
   others. BD-094 bundles the taxonomy with four verbs and BD-191 bundles three verbs; both were split the same
   way (N24a-35, N24a-36, N24a-37; N24a-40, N24a-41).
4. **Failure monitors have no predicates in the plan.** §29.10 names 23 classes and the gating rule and defines no
   anomaly predicate or threshold. BD-094 and N24a-38 propose them, labelled DESIGN, recorded per run and
   versioned in the registry.
5. **Two size limits for the replay file.** §32.15 states at most 15 MB and §32.24 a default cap of 8 MiB.
   N24a-41 treats 8,388,608 bytes as the default and 15,000,000 bytes as the highest value `--cap` accepts.
6. **A one-byte discrepancy in a known answer.** §32.24 prints 8,266,667 encoded bytes for a 6,200,000-byte
   payload (4/3 rounded up); exact padded Base64 is 8,266,668. N24a-41 prints the specification's figure in the
   size table and decides on the exact length.
7. **GATE-CHRON-08 is a second-milestone gate carried partly by a first-milestone seed.** BD-094's done-condition
   is the failure-taxonomy half of that gate. BD-094 and N24a-39 record its referee rows at the first milestone;
   the batched rows and the created-energy half remain with seed BD-208.
8. **Known answers reconstructed.** Appendix M prints values for the filleted lip (0.7332, 0.6351) and the
   wheel-loop spectral radii (1.4820, 0.9207, 2.6917, 2.7075) without their formulas. `N24-KA` states readings
   that reproduce every printed digit (checked numerically while drafting) and names the fixture's derivation files
   as authoritative. KA-616 is the same case as KA-413 of another chapter; the registry ties both to one record.
9. **Gates listed for the chapter that are not in the gate register (§40).** GATE-ARENA-10, -11, -12, -14 and the
   three kill gates of the late subsections have no row in §40; they belong to U24c's subsections and are not
   carried here.
10. **Titles.** Thirty titles drafted above the charter's length were shortened to at most 114 characters; no
    body text changed.
