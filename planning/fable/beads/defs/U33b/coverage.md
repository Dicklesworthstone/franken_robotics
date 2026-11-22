# Coverage of drafting unit U33b

Chapter §33 Suite consumption, subsections §33.7 to §33.11, and the fifteen seed beads assigned to this unit.
36 bead files: 15 seed beads and 21 new beads (`N33b-01` to `N33b-21`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads of this unit | Carried elsewhere (expected) |
|---|---|---|---|
| §33.7 | The dependency firewall | N33b-01 (the table as data on the stack-lock rows, the consistency test, the per-milestone report); N33b-02 (the M1 isolation build: M1 ships on FrankenSim, asupersync and FrankenSQLite alone); N33b-03 (the M7 freeze of every pin and of T) | The M0 exit review BD-044 and its successors read the report |
| §33.8 | Upstream work and drift | BD-008 (FD-ASUP-01 executed; INT-ASUP-01); N33b-04 (the compatibility train, the train log, the breakage share of BET-04, the rehearsal); N33b-05 (decision KILL-FSIM); N33b-06 (fallback (a), the worker posture, if adopted); N33b-07 (fallback (b), the lift, if adopted); N33b-08 (the twelve upstream items of the chapter: one standing check per interim posture, switch-over checklists, the staleness report of RISK-20) | BD-037 (U18a: `registries/upstream.toml`, mirrored beads); the `FD-…` beads (unit UF); BD-023 (U32a: FD-FMCP-01, INT-FMCP-01) |
| §33.9 | FrankenSim crates reached through the airlock for the new subsystems | N33b-09 (both crate tables as feature records, consumer allow-lists, pattern-only, oracle-only and not-admitted rules in `check-deps`, the absent-group refusal); N33b-11 (group `spectral`, INT-FSIM-13); N33b-12 (M1 part of INT-FSIM-14: fs-eproc); N33b-13 (group `stats`, INT-FSIM-14); N33b-14 (group `structure`, INT-FSIM-15); N33b-15 (`Recompute` over fs-recompute, INT-FSIM-16, local fallback); N33b-16 (the gated multiphysics bridge, INT-FSIM-17); N33b-21 (end-to-end script over all groups and FROB-REF-790); seed consumers BD-111 (memo graph), BD-124 with N33b-19 (study types and their verbs), BD-333 (`gait.orbit`), BD-401 (`foundry.size`, A-15), BD-413 (epic: advected films) | The first group table of §33.3 (U33a) |
| §33.10 | First admission tests, file by file | BD-032 (INT-FSIM-02, -03); BD-035 (INT-FSIM-04; INT-FSIM-07 refusal half); BD-033 (INT-FSIM-05); BD-034 (INT-FSIM-06); N33b-10 (INT-FSIM-08, -09, -10 first tests at M0); BD-079 (INT-FSIM-08 at the M1 pin with `frob-contact` primitives); BD-068 (INT-FSIM-09 at the M1 pin with production adapters); BD-243 with N33b-17 and N33b-18 (INT-FSIM-10 at the M3 pin with A-09, A-14, A-16); BD-028 (INT-FT-02); N33b-20 (end-to-end battery) | INT-FSIM-11: BD-020 (U21b). INT-FSYM-02: BD-105 (U33a). INT-FLN-02: BD-380 (U33a). INT-FNP-02: BD-036 (U33a). "INT-FSIM-12 is reserved and unused" is recorded here and needs no bead |
| §33.11 | Reported foundation defects | Row 1 (PPO update): BD-034. Row 2 (planar drive): BD-033. Row 4 (fixed witnesses, capacity 21): BD-035 (refusal at the pin) and BD-093 (completion under witness migration, INT-FSIM-07 in full). The interim postures of all four rows are re-asserted by N33b-08 | Row 3 (strides): BD-036 (U33a). Rows 5 and 6 are in §33.17 (U33c). Defect reports `FDEF-nn`: unit UF |

## (b) Gates, kill gates and admission gates

| Gate | Bead that measures or decides it (this unit) | Notes |
|---|---|---|
| INT-ASUP-01 | BD-008; re-asserted from the user's side by N33b-20 | |
| INT-FT-02 | BD-028; N33b-20 | |
| INT-FSIM-02, INT-FSIM-03 | BD-032; N33b-20 | |
| INT-FSIM-04 | BD-035; N33b-20 | |
| INT-FSIM-05 | BD-033; N33b-20 | |
| INT-FSIM-06 | BD-034; N33b-20 | |
| INT-FSIM-07 | BD-035 (M0, part 1: the typed refusal); BD-093 (M1, part 2: completion with the ledger closed) | §40 gives both milestones |
| INT-FSIM-08 | N33b-10 (M0 first test); BD-079 (M1 pin) | |
| INT-FSIM-09 | N33b-10 (M0 first test); BD-068 (M1 pin); N33b-21 | |
| INT-FSIM-10 | N33b-10 (M0 first test); BD-243 and N33b-18 (M3 pin) | |
| INT-FSIM-13 | N33b-11 (M3, per §40); BD-333 re-runs at the M4 pin; N33b-21 | |
| INT-FSIM-14 | N33b-12 (M1 part); N33b-13 (M3, in full); BD-124 consumes the M1 part; N33b-21 | |
| INT-FSIM-15 | N33b-14 (M5); BD-401 re-runs at its pin; N33b-21 | |
| INT-FSIM-16 | N33b-15 (M1); BD-111 through the production memo graph; N33b-21 | |
| INT-FSIM-17 | N33b-16 (M6); BD-413 (epic); N33b-21 | |
| KILL-FSIM | N33b-05 (decision); N33b-06 and N33b-07 (the two fallback branches) | Recorded as an ADR by the M0 exit review BD-044 (U33a) |
| GATE-TRAC-02 | BD-079 | Gate of §18, carried by this unit's seed bead |
| GATE-ATEL-12 (analysis part) | BD-111 | Episode part: BD-220 (U33a) |
| GATE-ARENA-05, GATE-ATEL-06 | BD-124; N33b-19 | Rule enumerator: N24b-06 (U24b) |
| GATE-GAIT-01 | BD-333 | |
| GATE-FOUNDRY-01 | BD-401 | |
| GATE-HAB-07 | BD-413 (epic); measured by N19b-10 (U19b) | |
| GATE-ATEL-02 (rows A-09, A-14) | BD-243, N33b-17 | The gate as a whole belongs to the Atelier units |
| INT-FSIM-18 | re-measured in production by N33b-06 only on the KILL-FSIM branch | The gate itself: BD-038 (U33c) |

Gates of the brief's list that this unit expects another unit to carry: GATE-WS-06, GATE-WS-09, INT-TOOLCHAIN-01
(BD-031), INT-FSIM-18 (BD-038) in U33c; INT-FSIM-01 (BD-006), INT-FSQL-01 (BD-009), INT-FSQL-02 (BD-220),
INT-CUDA-01 (BD-015), INT-FT-01 (BD-016), INT-FSYM-01 (BD-019), INT-FSYM-02 (BD-105), INT-FNP-01 and INT-FNP-02
(BD-036), INT-FLN-01 and INT-FLN-02 (BD-380), INT-ASUP-02 (BD-096), INT-FTUI-01 and INT-F3D-01 (BD-300),
INT-FDGR-01 (BD-398), INT-FTX-01 (BD-416) in U33a; INT-FSIM-11 (BD-020) in U21b; INT-FJAX-01 (BD-221) in U21a;
INT-FMCP-01 (BD-023) in U32a.

## (c) Counts

- Known answers of Appendix M.32 carried with their values: 10 (KA-768, -769, -770, -771, -772, -773, -774, -775,
  -776, -778); cited as used by a test: 2 more (KA-767 in BD-008, N33b-03, N33b-04; KA-780 in N33b-06). KA-777
  (strides) is BD-036's and KA-779 (units) is U33c's. Known answers of other chapters carried by the seed beads:
  KA-003, KA-004, KA-090, KA-092, KA-093, KA-425, KA-442, KA-443, KA-560, KA-561, KA-585, KA-612, KA-613.
- Counterexamples of Appendix N.30 carried: 3 (CX-352 in BD-033; CX-356 in N33b-05 and N33b-06; CX-357 in BD-034).
  CX-353 belongs to BD-036 (U33a); CX-354 and CX-355 to U33c. From other chapters: CX-151, CX-180, CX-182, CX-191,
  CX-237, CX-265, CX-266.
- Refusal codes of §33 used by a mechanism here: 3 (FROB-REF-790 in N33b-09, N33b-21 and every group bead;
  FROB-REF-791 in N33b-06; FROB-REF-890 in N33b-03, N33b-04, N33b-16). The registry bead `N33-REF` is U33c's.
  Other codes carried with meaning and repairs: 113, 118, 119, 163, 220, 221, 431, 532, 533, 542, 543, 552, 553,
  555, 556, 721, 724.
- Invariants and proof obligations carried in test plans: INV-SUITE-01 (N33b-09), INV-SUITE-02 (BD-008),
  INV-SUITE-03 and INV-SUITE-04 (BD-008, N33b-03, N33b-04, N33b-20), INV-SUITE-05 (N33b-16, N33b-06);
  INV-TRAC-06 and INV-KIN-02 (BD-093); INV-ARENA-05, INV-ATEL-02 (BD-124); INV-ATEL-08, PO-ATEL-07 (BD-111);
  INV-GAIT-01, PO-GAIT-01 (BD-333); INV-FOUNDRY-02, -06 (BD-401); INV-HAB-07, PO-HAB-05 (BD-413).
- Decisions stated with reasons: ADR-SUITE-01 and ADR-SUITE-02 (N33b-05, N33b-06), ADR-FOUNDRY-02, -03, -05, -06,
  -07 (BD-401), ADR-TRAC-01 (BD-079).
- Open questions placed: OPEN-SUITE-1, -3, -6 (BD-008); OPEN-SUITE-3, -5 (N33b-05); OPEN-ATEL-3 (N33b-15, N33b-12,
  N33b-13, BD-111); OPEN-SINEW-4 (BD-068); OPEN-TRAC-5 (BD-093); OPEN-GAIT-1, -2 (BD-333). None defined in
  §33.7 to §33.11 needs its own decision bead; the suite chapter's open questions are in §33.16 (U33c).
- Risks placed: RISK-01, RISK-23 (N33b-04, N33b-05); RISK-03, RISK-55 (BD-008); RISK-08 (BD-028); RISK-20,
  RISK-55 (N33b-08); RISK-31 (BD-035, BD-093). Bets: BET-04 (N33b-04, N33b-05, BD-033), BET-26 (BD-333), BET-29
  (BD-401), BET-34 (BD-035, BD-093).

## (d) Needs from other slices (no seed key could be named)

- §33.3: the first feature-group table of the airlock with its trait names, the bedrock-direct list, and the three
  adapter rules as production code (U33a). Needed by N33b-09 to N33b-16, BD-068, BD-079.
- §33.5 and §33.6: the stack-lock rows, the resolution receipt writer and the record format of
  `DEPENDENCY_ADMISSION.md` (U33a). Needed by every admission bead.
- §33.13: the toolchain train (U33c). Needed by N33b-03, N33b-04.
- §33.16: registry rows of FROB-REF-790, -791, -890 with exit codes and repairs (`N33-REF`, U33c).
- §18.4: the witness-migration algorithm and the narrow-phase record (N18a-14, N18a-13). Needed by BD-093, BD-079.
- §24.10 and §24.12: the fixed-n rule enumerator and study admission rules (N24b-06, N24b-14). Needed by BD-124,
  N33b-19.
- §22.17 and §22.23: the gait-orbit pilot (N22c-09) and the autonomous-loop section and bordered forms (N22c-36).
  Needed by BD-333.
- §10 C-08: the holonomy pilot (N10a-16). Needed by BD-243.
- §19.10.3: the advected-film pilot, field, update, gate and kill decision (N19b-07 to N19b-11). Children of BD-413.
- §31.19 and §31.23: the structure-loop pilot and the screening-time sizing check with GATE-FOUNDRY-15 (the Foundry
  unit; no directory existed when this unit was drafted). Needed by BD-401.
- §20.24: `sense.visibility` (Sensorium units). Related to N33b-17.
- §39: the tournament harness's run records and pooling rule. Needed by N33b-03.
- Appendix F: the `FD-…` beads and `FDEF-nn` defect reports (unit UF). N33b-08 tests the postures they leave in
  force; BD-008 executes FD-ASUP-01.

## (e) Contradictions and choices, with what was done

1. **Milestones of INT-FSIM-08, -09, -10.** §40.2 and the M0 acceptance of §43.3 require records at M0; the seed
   beads that name them sit at M1 (BD-079, BD-068) and M3 (BD-243). Done: N33b-10 runs the three first tests at M0
   through test-only adapters; the seed beads keep their milestones and re-run the gates at their own pins
   ("an admission is never inherited").
2. **Milestones of INT-FSIM-13 and INT-FSIM-14.** §40.2 says M3 for both; the seed consumers are BD-333 at M4 and
   BD-124 at M1; §33.7's M1 cell says the study statistics are "admitted when first used". Done: N33b-11 at M3;
   N33b-12 as the M1 part of INT-FSIM-14; N33b-13 as the full gate at M3.
3. **Where sequential statistics live.** §33.3 puts `fs-eproc` and `fs-race` in group `optimize`; the seed BD-124
   ties them to INT-FSIM-14, the gate of group `stats`; §33.9 defines no group gate for `optimize` or `actuation`.
   Done: the feature stays `optimize`; the admission is recorded as added rows of INT-FSIM-14 (N33b-12);
   `actuation` is covered by INT-FSIM-09 (BD-068). `fs-race` gets its row at first use (BD-262, BD-341).
4. **`fs-io`'s group.** §33.3 lists it under `geometry`; §33.9 gives it group gate INT-FSIM-15. Done: N33b-09
   records it under `geometry` with its STEP surface admitted under INT-FSIM-15 (N33b-14).
5. **Verb milestones.** Appendix J dates `study.plan`, `study.submit`, `study.status` at M2 and `eval.compare` at
   M3; the seed BD-124 delivers them on the CPU at M1. `foundry.size` is M4 in the registry, its gate "may be
   recorded at M4" and is due at M5 (§40.6), the seed is M5, and §31.23's GATE-FOUNDRY-15 calls the verb at M3.
   Done: seed milestones kept; both notes are in the beads' Considerations.
6. **Admission gates that record a real defect.** INT-FSIM-05 and INT-FSIM-06 are expected to fail for the sibling
   at the first pin. §40.1 says a failed gate with no kill gate blocks an exit; §43.2 says of M0 "a failed
   admission is the result, and its fallback is decided here". Done: such a gate is recorded `fail` with the
   interim posture adopted and listed as non-blocking (BD-033, BD-034, N33b-20); the lead may want a distinct
   status string in `registries/gates.toml`.
7. **Exit code of the `79x` refusals.** §32.4 maps `70x` to exit 4 and `72x`, `73x` to exit 5 and is silent on
   `79x`. Done: exit 5 is used (capability not admitted) and N33b-21 reads the value from the registry.
8. **Shadow selection.** §28.14 says one hit in 64 "chosen by key hash"; §33.9 requires a missing input to be
   caught "within 64 hits". A pure per-key choice cannot bound detection. Done: a deterministic per-kind hit
   ordinal with a hashed phase, exactly one shadowed hit per window of 64 (BD-111, N33b-15).
9. **The worker posture and the transactional step.** §33.14 forbids per-tick calls across the worker boundary
   and makes the worker a target of the layer-0 airlock crate; §16.5 and §18.4 make the referee transaction call
   the sibling step per sub-interval with migration between calls. Under KILL-FSIM fallback (a) these cannot both
   hold. Done: N33b-06 states the fail-closed default (such scenes are adjudicated by `strict`, the coverage
   figure reports the loss) and prepares an owner decision on letting the worker host the driver.
10. **Duplicate study refusals.** §24.14 (552, 532, 533) and §28.20 (553, 555, 542) overlap. Done: both sets kept,
    mapped to study creation and to comparison time (BD-124); U24b reports the same overlap.
11. **Missing material pair.** §33.10 requires a refusal; no code exists at the airlock. Done: a typed airlock
    refusal carrying the upstream reason (BD-068, N33b-10); the contact unit appends a registry code for local
    cards (BD-167).
12. **Consumer of `fs-couple`.** The table says "coupled islands [M]" with no crate. Done: `frob-scene` in the
    allow-list (N33b-09), flagged.
13. **Seed beads split.** BD-243 keeps A-09, the registry wiring and INT-FSIM-10; A-14 is N33b-17 and A-16 is
    N33b-18. BD-124 keeps the engines; its five verbs and the end-to-end script are N33b-19.
14. **Parents across milestones.** BD-111 (M1) sits under WP-63 (dated M3 to M4) because that package names the
    memo graph and INT-FSIM-16; N33b-11 (M3) under WP-64 (M4); N33b-13 (M3) under WP-54; N33b-02, N33b-08 and
    N33b-21 under WP-14. The lead may re-parent.
15. **Possible overlap with unit UF.** UF's brief lets it write the compatibility-train procedure if no seed bead
    carries it. N33b-04 carries it here as part of §33.8, together with the train log that KILL-FSIM's third
    trigger needs. One of the two should be kept.
16. **Data files added by this unit's beads** (each stated as DESIGN with its reason): fields `required_at`,
    `optional_at`, `uses` and a `[freeze]` table in `franken-stack.lock`; feature records with consumer lists in
    `crates/frob-fsim/contract.toml`; `registries/trains.toml`; fields `posture_test` and `switch_over_steps` in
    `registries/upstream.toml`. No `xtask` subcommand is added.
17. **Pilots.** No Frontier or Moonshot mechanism is defined in §33.7 to §33.11, so this unit writes no pilot bead.
    The Frontier seed beads (BD-243, BD-333, BD-401, BD-413) depend on pilots drafted by the units that own their
    mathematics; those are named in `needs` because their keys are not seed keys.
18. **Probes this unit had to design.** The first-use probes for `fs-regime`, `fs-robust`, `fs-surrogate` and
    `fs-assimilate` in N33b-13, the definitions of A-14 (targets, lookahead, reasons) in N33b-17 and the Morris
    design and band-extreme check of A-16 in N33b-18 go beyond one-line statements in the source text; each is
    labelled DESIGN in its bead.
