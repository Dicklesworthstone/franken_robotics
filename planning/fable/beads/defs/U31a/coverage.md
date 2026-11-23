# Coverage of drafting unit U31a

Chapter §31 (Foundry: structure, manufacture, lifecycle), the preamble and subsections §31.1 to §31.16.
Output: 63 bead files: no seed bead (the brief assigns none), 62 new beads (N31a-01 to N31a-62) and the
chapter's known-answer registry N31-KA. Unit U31b owns §31.17 to §31.23, the chapter's ten seed beads, and the
registries N31-CX and N31-REF; seed bead BD-401 (the beam rung's verb) is written by unit U33b.

Reading note for the lead. This unit's work was written in two sessions. The first wrote N31a-01 to N31a-59
and N31-KA and was interrupted before this file existed. The second audited those 60 files against the brief
(all complete, none truncated), added N31a-60 and N31a-61 (the beam rung of §31.7 inside the structure loop,
which no bead of this unit carried) and N31a-62 (the hardware side of firmware export), extended N31-KA
(derived values F-23 to F-31; second controls for three rows), and added the new keys to `deps` of N31a-05
and N31a-54 and to `related` of N31a-02, N31a-15, N31a-44 and N31a-58. Nothing else in the first session's
files was changed.

How these beads relate to the seed beads. U31b wrote each seed bead completely, as its brief requires. The
beads here are the subtasks of those capabilities: one mechanism, refusal set, surface or test each. Suggested
wiring (cycle-free as drafted): each seed bead depends on its subtask beads below, and this unit's end-to-end
beads depend on the seed beads (they already do).

| Seed bead (unit) | Subtask beads of this unit | End-to-end bead |
|---|---|---|
| BD-400 load cases and spectra (U31b) | N31a-06 to N31a-13 | N31a-53 |
| BD-401 beam rung and `foundry.size` (U33b) | N31a-60, N31a-61 (both depend on BD-401 and extend it) | the seed's own script; N31a-60, N31a-61 |
| BD-402 stress, buckling, modes, fatigue (U31b) | N31a-19 to N31a-22 | N31a-55 |
| BD-403 topology optimization (U31b) | N31a-05, N31a-14 to N31a-18, N31a-29 | N31a-54 |
| BD-404 inertia feedback and convergence (U31b) | N31a-23 to N31a-25 | N31a-54 |
| BD-405 manufacturability (U31b) | N31a-26 to N31a-30 | N31a-55 |
| BD-406 tolerance allocation (U31b) | N31a-31 to N31a-33 | N31a-56 |
| BD-407 bounded geometry export (U31b) | N31a-34, N31a-35, N31a-40 | N31a-57 |
| BD-408 the build packet (U31b) | N31a-36 to N31a-39, N31a-41 to N31a-43 | N31a-57 |
| BD-438 epic, firmware export (U31b) | N31a-44 to N31a-46, N31a-62 (children: `parent` is the epic) | N31a-58 |
| BD-423 epic, lifecycle compression (U31b) | N31a-47 to N31a-50 (children) | N31a-59 |
| none (cluster-wide) | N31a-01 to N31a-04 (scaffold, ceiling, admission, sibling adapters), N31a-51 (verbs), N31a-52 (refusals), N31-KA | |

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §31 preamble | Crates, cluster, tiers, feature flags | N31a-01 (crate `frob-foundry`, module map, features, firewall entries, "nothing depends on the Moonshots"); N31a-47 (crate `frob-lifecycle`) |
| §31.1 | What Foundry owns | N31a-01 (the three ownership groups; what Foundry does not do; the Estimated ceiling as a type); N31a-37 (the machine-readable flags) |
| §31.2 | An output-quality subsystem, not a speed lever | N31a-02 (the five FACT groups with their sources re-read; the ceiling f_L·s ≤ 0.08 printed before any saving; wording lint); N31a-03 (the consequence: finalists after qualification); N31a-25 (the mass-spiral multiplier 1/(1 − κ) as HYPOTHESIS, measured); N31a-61 (the engineered baseline a saving is stated against) |
| §31.3 | Decisions | N31a-01 (all twelve, with reasons and rejected alternatives, in `docs/adr/foundry.md`); each in the Background of the bead it governs: ADR-01 N31a-03; ADR-02 N31a-05, N31a-60, N31a-61; ADR-03 N31a-10; ADR-04 N31a-11; ADR-05 N31a-17, N31a-19; ADR-06 N31a-21, N31a-22; ADR-07 N31a-25, N31a-24; ADR-08 N31a-35; ADR-09 N31a-37; ADR-10 N31a-39; ADR-11 N31a-37, N31a-62; ADR-12 N31a-01, N31a-44, N31a-47 |
| §31.4 | What the siblings provide today | N31a-04 (one wrapper and one fact probe per sibling crate, fourteen crates at the pinned commits; the gap table; interim posture and switch-over test for FD-FSIM-26 to -30; the worker-process posture); facts restated in each consuming bead |
| §31.5 | The structure loop | N31a-05 (stage machine, rung interface, pairing of campaigns, budgets, resume, cancel, loop record); N31a-54 (end to end); the chain after the loop: N31a-26, N31a-32, N31a-34, N31a-43, and the as-built hand-off in N31a-33 and N31a-40 |
| §31.6 | Load cases extracted from telemetry [F] | Source and campaign: N31a-06 (rule 6, FROB-REF-169). The load at one instant and the free-body check: N31a-07. Unit loads, influence matrix, spectra: N31a-09. Envelope cases and rule 1: N31a-10. Rule 2: N31a-11. Rules 3 and 4: N31a-12 (FROB-REF-262). Rule 5: N31a-08. Declared load sets (the KILL-LOADS fallback): N31a-13. End to end: N31a-53 |
| §31.7 | Sizing and topology optimization of links [F] | Beam rung: BD-401 (U33b) and the early kernel N31b-38 (U31b) for the closed forms; N31a-60 (section forces from a full link load, rectangle, member checks); N31a-61 (stock sizing with notch fatigue, the baseline, the loop's `beam` rung). Design region: N31a-14 (FROB-REF-164). Topology rung and budgets: N31a-15. Plate parts: N31a-16. Mirror-image links as one problem: N31a-11, N31a-15, N31a-29. Extraction and the 1% smoothing budget: N31a-17. Fixtures: N31a-18 |
| §31.8 | Stress, buckling, modes and cumulative fatigue damage [F] | Mesh independence: N31a-17, N31a-19. Item 1: N31a-19 (FROB-REF-261, `singular: fillet required`). Items 2 and 3: N31a-20. Item 4: N31a-21 (five rules), N31a-22 (curve validity, FROB-REF-163). Item 5: N31a-01, N31a-19. End to end: N31a-55 |
| §31.9 | Inertia fed back into the model, and the convergence test [F] | N31a-23 (mass properties, typed edit, new `DesignId`); N31a-24 (λ̂, the four conditions, Aitken, divergence, cycling, FROB-REF-260); N31a-25 (actuators fixed; outer re-selection; the combined gain 0.74); N31a-05 (re-synthesis, re-run on the same world addresses) |
| §31.10 | Manufacturability [F] | N31a-26 (process cards, dispatcher over the feature table, findings with repairs, cost and carbon); N31a-27 (corner radius, enclosed voids); N31a-28 (tool access, overhang); N31a-29 (constraints active inside the optimization; the tightened re-run); N31a-30 (fixture battery) |
| §31.11 | Tolerance allocation driven by sensitivities [F] | N31a-31 (quantities of interest; sensitivities and their classes); N31a-32 (steps 1 to 3; the worked case); N31a-33 (steps 4 to 7; the zero-sensitivity counterexample; FROB-REF-165; the report); N31a-56 (end to end) |
| §31.12 | Bounded export and the build packet `.fbuild` [F] | Geometry export: N31a-34 (STL, 3MF, GLB), N31a-35 (STEP, the bound, FROB-REF-824). Container: N31a-36. Flags: N31a-37 (FROB-REF-860). Offline audit: N31a-38. Parts list, snap rule, prune counterexample, prices: N31a-39 (FROB-REF-818). Geometry, datums, fiducials, mass properties: N31a-40. Harness: N31a-41. Schedule: N31a-42. Evidence sections and assembly: N31a-43. Review as an import lane: N31a-37, N31a-36 (statuses; the protocol is U31b's). End to end: N31a-57 |
| §31.13 | Firmware export [M] | N31a-44 (scope, the generated crate, the bus trait, the flags); N31a-45 (checks 1 to 3; the seeded counterexamples; FROB-REF-827); N31a-46 (checks 4 to 6; the ceiling); N31a-62 (the stable-toolchain lane, the boundary pattern, imported timing receipts); N31a-58 (end to end) |
| §31.14 | Lifecycle compression [M] | N31a-47 (the question; two-timescale method; step counts and the ceiling); N31a-48 (the four laws; the averaging counterexample; validity domains, FROB-REF-831); N31a-49 (return to full resolution; rare overloads); N31a-50 (outputs; the verb; the 100-hour run); N31a-59 (end to end) |
| §31.15 | Verbs | N31a-51 (the seven registry entries, schemas, corrected refusal lists, waiting and jobs, parity). Handlers: `loads` N31a-06 to N31a-13; `size` BD-401 with N31a-60; `optimize` N31a-05; `check` N31a-19 to N31a-21, N31a-26; `tolerance` N31a-32, N31a-33; `export` N31a-34, N31a-43; `lifecycle` N31a-50 |
| §31.16 | Refusals | N31a-52 (fourteen typed payloads, ranked repairs, trigger battery). Triggers: 166 N31a-03; 167 N31a-07, N31a-10; 169 N31a-06; 163 N31a-22; 164 N31a-14; 165 N31a-33; 260 N31a-24; 261 N31a-19 to N31a-21; 262 N31a-12; 860 N31a-37, N31a-38; 818 N31a-39; 824 N31a-35; 827 N31a-45; 831 N31a-48. Registry rows: N31-REF (U31b) |

Identifiers of §31.17 that constrain this unit's mechanisms are tested where the mechanism is: INV-FOUNDRY-01
N31a-37, N31a-38; -02 N31a-07, N31a-10; -03 N31a-17, N31a-19, N31a-43; -04 N31a-23, N31a-40; -05 N31a-36;
-06 N31a-01; -07 N31a-01, N31a-44; PO-FOUNDRY-01 N31a-09; -02 N31a-08; -03 N31a-24; -04 N31a-45. U31b's
N31b-01 to N31b-03 hold the chapter-level property tests and the firewall.

## (b) Gates, kill gates and who measures or decides them

| Gate | Beads of this unit | Elsewhere |
|---|---|---|
| GATE-FOUNDRY-01 | N31a-19 (finite-element path on arbitrary geometry); N31a-55 (script) | BD-401 (U33b) measures the closed forms |
| GATE-FOUNDRY-02 | N31a-07 (free-body residual); N31a-09 (superposition); N31a-10 (assembled-peak mutation); N31a-11 (equal-limb mutation); N31a-53 (script) | BD-400 |
| GATE-FOUNDRY-03 | N31a-21; N31a-55 (script) | BD-402 |
| GATE-FOUNDRY-04 | N31a-17 (meshing and audit); N31a-18 (two-bar and the twenty fixtures); N31a-54 (script) | BD-403; N31b-24 (the open question that blocks it) |
| GATE-FOUNDRY-05 | N31a-05 (the loop that runs the comparison); N31a-15 (budget clause); N31a-61 (the baseline and the equal-margin rule); N31a-54 (script) | N31b-13 (the registered measurement); BD-403 |
| GATE-FOUNDRY-06 | N31a-24; N31a-54 (script) | BD-404 |
| GATE-FOUNDRY-07 | N31a-27, N31a-28 (evaluators); N31a-30 (battery); N31a-55 (script) | BD-405 |
| GATE-FOUNDRY-08 | N31a-32, N31a-33; N31a-56 (script) | BD-406 |
| GATE-FOUNDRY-09 | N31a-35 (round trip, refusals); N31a-40 (mass identity); N31a-57 (script) | BD-407 |
| GATE-FOUNDRY-10 | N31a-36 (hashes, atomic write); N31a-37 (flags); N31a-38 (`frob-verify`); N31a-43 (cross-consistency); N31a-57 (script) | BD-408; N31b-44 |
| GATE-FOUNDRY-11 | N31a-45 (cases); N31a-46 (vectors); N31a-58 (script) | N31b-20; BD-438 |
| GATE-FOUNDRY-12 | N31a-47 to N31a-50; N31a-59 (script) | N31b-22; BD-423 |
| GATE-FOUNDRY-13, -14, -15 | none: their mechanisms are in §31.21 to §31.23 | N31b-34, N31b-37, N31b-43 |
| KILL-LOADS | N31a-53 (produces the record); N31a-13 (the fallback) | N31b-16 decides |
| KILL-TOPOPT | N31a-54 (record); N31a-61 (the fallback rung) | N31b-15 decides |
| KILL-FAB | N31a-55 (record); N31a-26 (advisory mode) | N31b-17 decides |
| KILL-TOLERANCE | N31a-56 (record); N31a-33 (worst-case boxes) | N31b-18 decides |
| KILL-PACKET | N31a-57 (record); N31a-43 (reduced profile) | N31b-19 decides |
| KILL-FIRMWARE | N31a-58 (record); N31a-46 (parameters-only output) | N31b-21 decides |
| KILL-LIFECYCLE | N31a-59 (record); N31a-50 (hand-specified wear states) | N31b-23 decides |

Gates this unit expects another unit to carry: GATE-FOUNDRY-13 to -15 and every kill-gate decision (U31b);
INT-FSIM-15 for the `structure` group (N33b-14, U33b); GATE-WS-02 for the `.fbuild` format row (the workspace
unit; N31a-36 supplies the round trip); the recurring flags gate GATE-SAFE-01 and the auditor's dependency
gate GATE-LEMMA-03 (N31a-37 and N31a-38 register their specimens there); KILL-SINEW-AGE (the actuator
chapter's unit; N31a-48 and N31a-59 follow its outcome).

## (c) Counts

- Known answers: 18 numbered rows (KA-737 to KA-754) in N31-KA, each with case, truth, tolerance, tier,
  negative control and the beads that test it; tolerances set for the two rows printed "to set"; controls
  registered for the four rows printed without one. Also in N31-KA: 4 construction rows (KA-168 to KA-171),
  6 answers of §31.21 to §31.23 (tests by U31b) and 31 derived values (F-01 to F-31) for fixtures the
  mechanism beads introduce. Every row's values are also stated inline in the bead that tests it.
- Counterexamples: all 9 of the chapter (CX-336 to CX-344) are cited with their numbers in this unit's test
  plans (N31a-10, -11, -21, -33, -39, -45, -48 and the scripts); the registry bead N31-CX is U31b's.
- Refusal codes: all 14 (163, 164, 165, 166, 167, 169, 260, 261, 262, 818, 824, 827, 831, 860) have a typed
  payload, ranked repairs and a trigger test in N31a-52 and a trigger in a mechanism bead; the registry bead
  N31-REF is U31b's. One refusal is introduced without a code (`firmware-receipt-mismatch`, N31a-62).
- Invariants 7 of 7 and proof obligations 4 of 4 tested in mechanism beads; architecture decisions 12 of 12;
  open questions 6 of 6 implemented as their fail-closed defaults (decision beads: U31b); verbs 7 of 7;
  upstream items FD-FSIM-25 to -30 each with interim posture and switch-over test; end-to-end beads 7
  (N31a-53 to N31a-59) plus the scripts of N31a-60 to N31a-62; pilots 5 of 5 named in `needs` (beads: U31b).

## (d) Needs from other slices (no key could be named)

- **Foundry, unit U31b.** The five pilots of §31.19 (every [F] and [M] mechanism bead names its pilot); the
  six open-question decisions of §31.20; the kill-gate decisions; the review-record protocol and the
  mechanical pre-review of §31.21 (N31a-37, -40, -43, -55, -57, -62); referee re-extraction and the
  provisional-case refusal of §31.22 (N31a-03, -10, -43, -53); the early sizing kernel and class load factors
  of §31.23 (N31a-13, -60, -61); the registry rows of N31-REF and N31-CX.
- **Design language (§14).** Port boss geometry; keep-in envelopes, clearance margins and cable ports;
  declared notches; process declarations on made parts; `member_end` and `section_up`; the per-body mass
  roll-up; the `declared` fill and its mass-support test; `design.check` on an edited design.
- **Kinematics and algebra (§13).** The mechanical symmetry group as isometries per body; the
  product-of-exponentials map evaluable in exact and interval arithmetic.
- **Actuators and cards (§15).** Provenance classes and `--strict-data`; stock cards; thermal derating per
  joint; battery ageing states settable per world; the KILL-SINEW-AGE decision.
- **Batching, contact, sensing, kernels (§17, §18, §20, §21, §35).** Per-world parameter variants; a per-run
  contact-stiffness factor and alternative contact law as part of model identity; sensor clock contracts;
  the kernel IR's fixed-point types; the exact-integer kernel class; a registered stream for test vectors.
- **Control and gait (§22).** Re-synthesis of the baseline controller for an edited design; the energy-budget
  monitor and shield rows as kernel-IR programs; gait period and periodicity residual.
- **Arena, mission, adversarial search (§24, §25, §26).** A read of the qualification ledger; the qualified
  task set with push-recovery and drop templates; paired comparison of two controller arms; evaluation of
  per-world model variants; the exact feasibility oracle of a mission; rare-event rates with bounds.
- **Analysis (§28).** A-08 statics, A-10, A-11, A-12 (bandwidth; reflected mass at a port), A-13 swept charts,
  A-16 sensitivities, the per-body mass roll-up of a design.
- **Chronicle (§29).** T2 channels for joint constraint wrenches, contact wrenches, twists and
  accelerations; `telemetry.query` with dropped intervals; `sim.replay` to a tick with its digest.
- **Ground (§30).** The as-built audit against allocated tolerances; fiducial requirements of registration.
- **Tether (§32).** The job protocol; registration of the seven verbs (BD-409 covers the M5 set); cost
  estimates before admission; the envelope's refusal object; MCP tool generation and the `make` profile.
- **Suite and airlock (§33).** The `structure` group and INT-FSIM-15; `MeshExchange`; a signed distance field
  of a mesh with an error bound; the syntax-level STEP entry points; the mass-properties entry; admission of
  `fs-uq::product_epistemic` and of the `spectral` group for these crates; whether an optimization job
  accepts an external differentiable filter stage; the stack lock; the franken_firmware ledger and survey
  rows and the never-dependency rule.
- **Workspace, Lemma, safety, conformance, gates (§34, §12, §37, §38, §40).** The crate rows and caps; the
  format rows FMT-FBUILD and one for `frob.firmware.receipt/1`; the auditor's artifact-kind registry;
  `bv_decide` as a subprocess on the Linux host; the recurring flags gate; the known-answer and
  negative-control registry formats; the gate-record format and store; the source-register entry format.
- **Registry owner.** A refusal code for `firmware-receipt-mismatch`.

## (e) Contradictions, gaps and what was done

1. **Milestones.** The chapter tags GATE-FOUNDRY-01 to -03 and KILL-LOADS at M4 and GATE-FOUNDRY-12 and
   KILL-LIFECYCLE at M8; the gate register sets M5 (recordable at M4) and M6. The verb registry dates
   `foundry.loads` and `foundry.size` at M4 and `foundry.lifecycle` at M8. Beads follow the register and the
   seed beads' milestones (M5; M6 for lifecycle; M8 for firmware) and say so (N31a-47, N31a-51).
2. **The verb table's refusal column** prints an older numbering that the refusal registry assigns to other
   chapters. Read by position against the chapter's own refusal table: 160 → 166, 161 → 167, 162 → 169,
   861 → 818, 862 → 824, 863 → 827, 864 → 831. N31a-51 writes the registry entries with the Foundry codes:
   `loads` 167, 169, 262; `size` 163 (and 167 for a load file that is not one instant); `optimize` 166, 164,
   260, 261; `check` 163, 261 (860 for a packet argument); `tolerance` 165; `export` 166, 860, 818, 824, 261
   (827 with `--with-firmware`); `lifecycle` 831 (and 169). A test compares each listed code's registry
   meaning with the meaning used.
3. **The beam rung has three carriers.** The early kernel (N31b-38, M3), the seed bead BD-401 (U33b: verb,
   A-15, finite-element form) and this unit's N31a-60 and N31a-61 (what the plan's sentence "passes every
   envelope case and the fatigue utilization at declared notches" needs once a case is a full link load with
   a body field). N31a-60 and N31a-61 depend on BD-401 and name the kernel in `needs`; BD-401 should depend
   on N31b-38, as U31b also notes. N31a-60 adds one output option to the seed's verb, `foundry.size
   --diagram`; no verb is added.
4. **Two units, one script.** N31a-58 and N31b-20 both specify `scripts/e2e/foundry_firmware.sh` for
   GATE-FOUNDRY-11; N31a-59 and N31b-22 both specify `scripts/e2e/foundry_lifecycle.sh` for GATE-FOUNDRY-12;
   N31a-54 (`foundry_structure_loop.sh`) and N31b-13 (`gate_foundry_05.sh`) both measure GATE-FOUNDRY-05.
   They were drafted in parallel. Suggested resolution: keep this unit's bead as the verb's script (it
   depends on the mechanism subtasks) and reduce U31b's to the gate row, the pilot and decision dependencies
   and the ledger entry, depending on this unit's bead; or merge each pair. Not changed here.
5. **Exploratory export: the two units read it differently.** N31a-03, N31a-36 and N31a-43 refuse
   `foundry.export --exploratory` for a packet (FROB-REF-166), stamp only loose geometry as exploratory, and
   give the manifest's `exploratory` field a type with the single value false. U31b's BD-408 writes a packet
   with `exploratory: true` and `qualification: null` as a labelled artifact that readers do not treat as a
   build packet. The plan supports both ("cannot enter a build packet"; a manifest field that can be null).
   The stricter reading is the fail-closed one; the owner should choose, and one of the two beads changes.
6. **Feature names.** This unit's beads use the chapter's Cargo feature name `firmware-export` and the
   separate crate `frob-lifecycle`; U31b's N31b-03 applies the workspace rule (`moonshot-` prefix) to Cargo
   names and keeps the chapter's names at the user surface. If the workspace rule stands, the `cargo test
   --features …` lines of N31a-01, N31a-44 to N31a-46 and N31a-62 change name only.
7. **Known-answer tolerances and controls.** Two rows are printed "to set" and four without a control.
   N31-KA sets the tolerances from the gates and registers controls; U31b's N31b-04 proposes its own. Where
   the two differed (KA-747, KA-750, KA-754) N31-KA now registers both, so the beads agree row for row. All
   remain proposals until the registry owner accepts them.
8. **Manufacturability table.** A "Moulded" row (draft and undercut) exists while no moulded process is among
   the six a part may declare, and the text says the sibling measures two features where the table gives it
   three rows. N31a-26 follows the table and keeps the moulded row dormant until a card declares a demould
   direction; N31a-30 covers the gate's "undercut" both as a milling undercut and as a draft undercut.
9. **Schema name.** The format registry's `frob.build/1` is used; the `frob.fbuild/1` of a type-sketch note
   is recorded as superseded (N31a-36).
10. **What "equal margin" means.** The plan compares an optimized part with the engineered part "at equal
    first bending frequency and equal fatigue utilization" and gives no rule. N31a-61 fixes one (DESIGN): the
    reference is the continuous-wall tube, not the next stock size; both parts go through the same
    independent check; the saving counts only when the optimized part's frequency is within 2% of or above
    the reference's and its fatigue utilization is no higher. Each choice can only lower a reported saving.
    N31a-60 likewise adds a neutral-axis shear point, closed forms for a rectangular section and a 1.5%
    margin on the one-mode frequency, where the plan gives formulas for the tube only. For the owner to
    confirm.
11. **Pilot threshold against gate threshold.** The pilot's falsifier is a saving under 5%; the gate and the
    bet use 7%. N31a-54 asserts the gate's 7% (with 15% as the target) and records the measured saving; the
    5% reading belongs to the pilot beads of U31b, which report against both.
12. **GATE-FOUNDRY-06 asks for the fixed point within 10⁻⁶** inside a five-iteration budget; plain iteration
    at gain 0.15 is not that close by iteration five. N31a-24 reports the Aitken value of the last three
    iterates as the fixed-point estimate (exact for the linear fixture) and tests it in rational arithmetic.
13. **Contacts on links without a contact port.** The plan's load coordinates have no place for a wrench on
    an undeclared patch (a knee striking the ground). N31a-07 keeps such samples in the free-body sum, tags
    them `off-port`, excludes them from cases and spectra, and counts them toward the 0.1% budget (DESIGN).
14. **Gaps at the airlock with no upstream item.** An overhang filter inside a sibling optimization job
    needs an external differentiable stage (N31a-29 runs it as a post-check until then); interval evaluation
    by `fs-uq::product_epistemic` is not in the airlock's consumer list for Foundry (N31a-33). Both are in
    (d); neither blocks a milestone.
15. **Firmware's hardware side.** The plan says timing is `no-claim` "until a receipt from a linked binary on
    real hardware is imported", names franken_firmware as a pattern, and notes that firmware export would be
    a stable-only target, without giving a record format, a floor or a lane. N31a-62 supplies them (DESIGN):
    a sidecar addressed by the firmware manifest digest that changes no byte and raises no class (imported
    with a flag of the existing verb, `foundry.check --firmware-receipt`), a stable floor read from the
    sibling's pin, and a build lane outside the workspace. The foundation ledger gives
    that sibling no admission gate until the M8 package is funded, so the bead measures no gate.
16. **OPEN-SKEL-4** (lattice anisotropy; owner: the Foundry lead) is an open question of the design-language
    chapter that "blocks nothing before §31". Its default (isotropic fraction, the assumption printed) is
    carried in N31a-61; the decision bead belongs to that chapter's unit.
17. **Parents of the Moonshot beads** are the seed epics BD-438 and BD-423 (under WP-80 and WP-75), as in
    U31b, not the work packages directly.
18. **Mutual use without an order.** The frequency-separation test (N31a-12) reads the first frequency that
    the modal check (N31a-20) produces, and the modal floor reads f₉₉ from the load spectra. Inside the loop
    each uses the other's previous-iteration value, so neither is in the other's `deps`; N31a-12 and N31a-16
    name beads of this unit in `needs` for the lead to record as `related`.
