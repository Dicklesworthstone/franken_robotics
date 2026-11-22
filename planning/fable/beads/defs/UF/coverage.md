# Coverage of drafting unit UF: Appendix F (upstream work items)

71 bead files: 53 upstream work items of the register (`FD-…`), 6 defect reports (`FDEF-01` … `FDEF-06`),
12 new beads (`NF-01` … `NF-12`). No seed bead is assigned to this unit.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| Appendix F, preamble | The rule (ownership, additive changes, the sibling's rules, Agent Mail reservation, mirrored beads, interim posture, switch-over, third-party gaps are not upstream items, the numbering note) | Carried in the Technical approach of every `FD-…` bead; the switch-over procedure itself is NF-01; the registry is BD-037 (unit U18a) |
| F.1 | The register (53 rows) | One bead per row, key equal to the row's identifier (table below) |
| F.2 | The reported foundation defects, as defect reports (6 rows) | FDEF-01 … FDEF-06 (table below) |
| F.3 | What the upstream work items do not change | The Considerations of every `FD-…`, `FDEF-…` and upstream `NF-…` bead; NF-01 and NF-02 (never roll back, no move during the tournament) |
| §33.8 (by instruction 4) | The compatibility train | NF-01 (procedure, script, train log, guards, the breakage share that KILL-FSIM reads); NF-06 (end-to-end rehearsal) |
| §33.13 (by instruction 4) | The toolchain train | NF-02 (procedure); NF-03 (FrankenSim leg); NF-04 (FrankenSymPy leg); FD-FMCP-01 (fastmcp_rust leg); NF-05 (the first run); NF-06 (rehearsal) |
| §33.18, last paragraph | Six upstream work items "added to Appendix F by description" | NF-07 … NF-12 |

Register rows (F.1). Columns: register priority; bead priority; milestone label; parent.

| Row | Reg. | Pri. | M | Parent | Row | Reg. | Pri. | M | Parent |
|---|---|---|---|---|---|---|---|---|---|
| FD-FSIM-01 | P1 | 1 | M2 | WP-10 | FD-FSIM-28 | P3 | 3 | M5 | WP-70 |
| FD-FSIM-02 | P1 | 1 | M2 | WP-22 | FD-FSIM-29 | P3 | 3 | M5 | WP-71 |
| FD-FSIM-03 | P1 | 1 | M1 | WP-10 | FD-FSIM-30 | P3 | 3 | M5 | WP-70 |
| FD-FSIM-04 | P2 | 2 | M2 | WP-39 | FD-FSIM-31 | P2 | 2 | M1 | WP-01 |
| FD-FSIM-05 | P3 | 3 | M6 | WP-74 | FD-FSIM-32 | P2 | 2 | M2 | WP-48 |
| FD-FSIM-06 | P2 | 2 | M1 | WP-02 | FD-FSIM-33 | P2 | 2 | M1 | WP-02 |
| FD-FSIM-07 | P2 | 2 | M2 | WP-14 | FD-FT-01 | P1 | 1 | M2 | WP-19 |
| FD-FSIM-08 | P1 | 1 | M1 | WP-07 | FD-FT-02 | P2 | 2 | M2 | WP-21 |
| FD-FSIM-09 | P3 | 3 | M2 | WP-46 | FD-FT-03 | P2 | 2 | M2 | WP-18 |
| FD-FSIM-10 | P2 | 2 | M1 | WP-37 | FD-FNP-01 | P2 | 2 | M1 | WP-37 |
| FD-FSIM-11 | P2 | 2 | M1 | WP-37 | FD-FSYM-01 | P2 | 2 | M2 | WP-15 |
| FD-FSIM-12 | P1 | 1 | M2 | WP-39 | FD-FSYM-02 | P3 | 3 | M2 | WP-05 |
| FD-FSIM-13 | P1 | 1 | M2 | WP-39 | FD-FSYM-03 | P3 | 3 | M2 | WP-03 |
| FD-FSIM-14 | P2 | 2 | M1 | WP-01 | FD-FSYM-04 | P2 | 2 | M5 | WP-31 |
| FD-FSIM-15 | P1 | 1 | M1 | WP-01 | FD-FSYM-05 | P3 | 3 | M4 | WP-67 |
| FD-FSIM-16 | P2 | 2 | M1 | WP-02 | FD-FLN-01 | P3 | 3 | M5 | WP-31 |
| FD-FSIM-17 | P2 | 2 | M3 | WP-50 | FD-FLN-02 | P3 | 3 | M5 | WP-31 |
| FD-FSIM-18 | P3 | 3 | M4 | WP-55 | FD-ASUP-01 | P0 | 0 | M0 | WP-02 |
| FD-FSIM-19 | P3 | 3 | M4 | WP-55 | FD-FMCP-01 | P0 | 0 | M0 | WP-06 |
| FD-FSIM-20 | P3 | 3 | M4 | WP-64 | FD-FSQL-01 | P3 | 3 | M2 | WP-63 |
| FD-FSIM-21 | P3 | 3 | M2 | WP-09 | FD-FTUI-01 | P3 | 3 | M3 | WP-52 |
| FD-FSIM-22 | P3 | 4 | M6 | WP-73 | FD-FKER-01 | P3 | 3 | M1 | WP-36 |
| FD-FSIM-23 | P3 | 3 | M5 | WP-68 | FD-FDGR-01 | P3 | 3 | M5 | WP-32 |
| FD-FSIM-24 | P3 | 3 | M5 | WP-68 | FD-FSS-01 | P3 | 3 | M5 | WP-32 |
| FD-FSIM-25 | P3 | 3 | M5 | WP-68 | FD-FTX-01 | P3 | 4 | M6 | WP-73 |
| FD-FSIM-26 | P3 | 3 | M5 | WP-71 | FD-F3D-01 | P3 | 3 | M3 | WP-52 |
| FD-FSIM-27 | P3 | 3 | M5 | WP-70 | | | | | |

Defect reports (F.2) and the later foundation facts (§33.18).

| Source row | Component | Report bead | Upstream bead |
|---|---|---|---|
| F.2 row 1 | `fs_g1_train::ppo_update` | FDEF-01 | FD-FSIM-10 |
| F.2 row 2 | `fs-mbd::planar_drive` | FDEF-02 | FD-FSIM-11 |
| F.2 row 3 | Strides in `fnp-ndarray` and `ft-core` | FDEF-03 | FD-FNP-01 |
| F.2 row 4 | Sustained contact step (fixed witnesses; capacity 21) | FDEF-04 | FD-FSIM-12, FD-FSIM-13 |
| F.2 row 5 | `fs-mbd::Wrench` (convention hazard) | FDEF-05 | FD-FSIM-33 |
| F.2 row 6 | `fs_g1_train::gaussian_action` | FDEF-06 | FD-FSIM-32 |
| §33.17 row 7 | `serde_json` pins across siblings | (in the upstream beads) | FD-FTX-01, FD-F3D-01 |
| §33.17 row 8 | The `metal` 0.33.0 binding | (in the upstream bead) | FD-FT-01 |
| §33.18 row 9 | G1 catalog omits 1.377 kg | NF-07 (report and change in one bead) | NF-07 |
| §33.18 row 10 | `fs_ga::Se3` Jacobians refuse at 33 m | NF-08 | NF-08 |
| §33.18 row 11 | Legacy `fs_phs::transformer` sign | NF-09 | NF-09 |
| §33.18 row 12 | `swept_sphere_aabb` encloses straight motion only | NF-10 | NF-10 |
| §33.18 row 13 | `fs-sos`: two values that are not bounds | NF-11 | NF-11 |
| §33.18 row 14 | `f3d_math::Ray::apply_matrix4` renormalizes | NF-12 | NF-12 |

## (b) Gates

Appendix F defines no gate, kill gate or admission gate of its own. The table lists every gate that a
switch-over rule re-runs, with the beads of this unit that name it in `gates`. Each gate is measured by the
unit that owns its chapter; the beads here re-run it at a new pin and record the result.

| Gate | Beads of this unit | Gate | Beads of this unit |
|---|---|---|---|
| INT-TOOLCHAIN-01 | FD-FSIM-15, FD-FSIM-31, NF-02, NF-03, NF-04, NF-05 | GATE-KIN-01 | FD-FSIM-04 |
| INT-FSIM-01 | FD-FSIM-14, FD-FSIM-16, NF-03 | GATE-KIN-02, -03 | FD-FSIM-01 |
| INT-FSIM-02, -03 | FD-FSIM-06 | GATE-FID-01 | FD-FSIM-02, FD-FSIM-12 |
| INT-FSIM-04 | FD-FSIM-13, FDEF-04 | GATE-FID-03 | FD-FSIM-12, FD-FSIM-13 |
| INT-FSIM-05 | FD-FSIM-11, FDEF-02 | GATE-TRAC-02 | FD-FSIM-03 |
| INT-FSIM-06 | FD-FSIM-10, FDEF-01 | GATE-TRAC-06 | FD-FSIM-13 |
| INT-FSIM-07 | FD-FSIM-12, FDEF-04 | GATE-TRAC-12 | NF-10 |
| INT-FSIM-09 | FD-FSIM-08 | GATE-SINEW-07 | FD-FSIM-21 |
| INT-FSIM-14 | FD-FSIM-17 | GATE-SINEW-10 | NF-09 |
| INT-FSIM-15 | FD-FSIM-28 | GATE-SPARK-02 | FD-FT-01, FD-FT-03 |
| INT-ASUP-01 | FD-ASUP-01, FD-FSIM-16, FD-FKER-01 | GATE-SPARK-03 | FD-FT-03 |
| INT-FMCP-01 | FD-FMCP-01 | GATE-SPARK-04 | FD-FT-01 |
| INT-FSQL-02 | FD-FSQL-01 | GATE-CORTEX-02 | FD-FT-02 |
| INT-FT-01, -02 | FD-FT-01 | GATE-CORTEX-18 | FD-FSIM-32, FDEF-06 |
| INT-CUDA-01 | FD-FT-03 | GATE-CHRON-09 | FD-FSIM-17 |
| INT-FSYM-01 | FD-FSYM-01, NF-04 | GATE-NEMESIS-05 | FD-FSIM-18, FD-FSIM-19 |
| INT-FLN-01 | FD-FLN-02 | GATE-GAIT-01, -02 | FD-FSIM-20 |
| INT-FNP-01, -02 | FD-FNP-01, FDEF-03 | GATE-GROUND-04 | FD-FSIM-23 |
| INT-FTUI-01 | FD-FTUI-01 | GATE-GROUND-08 | FD-FSIM-25 |
| INT-F3D-01 | FD-F3D-01 | GATE-FOUNDRY-04 | FD-FSIM-27, FD-FSIM-30 |
| INT-FDGR-01 | FD-FDGR-01 | GATE-FOUNDRY-05 | FD-FSIM-28 |
| INT-FTX-01 | FD-FTX-01 | GATE-FOUNDRY-07 | FD-FSIM-29 |
| GATE-WS-06 | FD-FSIM-32, FD-FSIM-33, FD-FTX-01, FD-F3D-01, FDEF-05, FDEF-06 | GATE-FOUNDRY-09 | FD-FSIM-26 |
| GATE-WS-09 | NF-07 … NF-12 | GATE-HAB-03 | FD-FDGR-01, FD-FSS-01 |
| GATE-LEMMA-02 | FD-FLN-01 | GATE-HAB-09 | FD-FSIM-22 |
| GATE-MARROW-02 | FD-FSYM-03 | GATE-HAB-16 | NF-12 |

Kill gates. None is decided by a bead of this unit. They are stated, with trigger and fallback, where an
upstream item's fallback is one: KILL-MCP (FD-FMCP-01, FD-ASUP-01, NF-05); KILL-FSIM (NF-01, which supplies the
breakage-share measurement, and FD-FSIM-08); KILL-METAL (FD-FT-01); KILL-LIVING (FD-FSIM-22, FD-FTX-01);
KILL-INGEST (FD-FDGR-01, FD-FSS-01); KILL-LEARNKERN (FD-FT-02); KILL-TOPOPT (FD-FSIM-28). Their decision beads
belong to the units that own those chapters.

## (c) Counts carried

- Known answers cited with their values: 12 (KA-075, KA-767 to KA-770, KA-773 to KA-779). The registry bead
  `N33-KA` is written by unit U33a; this unit writes no registry bead, since Appendix F has no known-answer,
  counterexample or refusal section of its own.
- Counterexamples cited with their cases: 7 (CX-182, CX-192, CX-352, CX-353, CX-354, CX-355, CX-357 ≡ CX-252).
- Refusal codes used with their meaning: 9 (FROB-REF-304, -607, -617, -721, -790, -791, -822, -824, -890). None
  is defined by Appendix F.
- Invariants named in test plans: INV-SUITE-01 to -05, INV-KIN-02, INV-NUM-06, INV-SPARK-02. Architecture
  decisions stated: ADR-MARROW-01, ADR-SPARK-09, ADR-LEMMA-02. Open questions stated with their defaults:
  OPEN-SUITE-1 to -6, OPEN-SINEW-4, OPEN-SPARK-4, -5, OPEN-NEMESIS-2 to -4, OPEN-HAB-5, OPEN-LEMMA-1, -2, -6,
  OPEN-WS-2, OPEN-TETHER-1. Risks: RISK-01, -02, -03, -06 to -09, -21 to -24, -31. Bets: BET-04, -09, -29, -34.

## (d) Needs from other slices

| Bead | Needs |
|---|---|
| NF-01, NF-02 | §39.13: who declares the tournament freeze and under what name (this unit proposes `receipts/FREEZE.toml`) |
| NF-01 | §33.14: the rebuild of `frob-fsim-worker` at a pin move (BD-038) |
| NF-02 | §33.12: the commands with which BD-031 records each cell of the toolchain matrix |
| FD-FSIM-21 | §15.14: the decision bead for OPEN-SINEW-4 (unit U15b wrote one, key N15b-28) |
| FD-FSIM-22 | §19.10.7: the proxy-adequacy pilot, GATE-HAB-09 and the KILL-LIVING decision |
| FD-FSIM-32, FDEF-06 | §23.25: the joint-law battery of rule P-12 and its planted sampler (unit U23b: N23b-29, N23b-30) |
| FD-FSIM-33, FDEF-05 | §33.17: the mixed-wrench probe of GATE-WS-06 and `MixedWrench` in `frob-fsim` (unit U33c) |
| FD-FDGR-01 | §33.15: the schema adapter and fixtures of INT-FDGR-01 |
| FD-FSS-01 | §19.13: the inspection pilot of one twin package |
| FD-FTX-01 | §33.15: INT-FTX-01 through the worker process (BD-416) |
| NF-07 … NF-12 | §33.18: the six admission probes of GATE-WS-09 (unit U33c); §18.20 and §19.22 for NF-10 and NF-12 |
| NF-07 … NF-12 | Appendix F.1: `FD-` identifiers (see (e), item 1) |

For the lead: BD-008 should list FD-ASUP-01 in `deps`, and BD-023 should list FD-FMCP-01, since each seed
bead's done-condition includes the upstream change landing. The two P0 beads here depend only on BD-001 and
BD-006, so no cycle arises. BD-037 (registry, fixtures, mirrored beads) is a dependency of every non-P0 `FD-…`
bead, directly or through its `FDEF-…` bead.

## (e) Contradictions, gaps and what was done

1. **Six items in §33.18 are "added to Appendix F by description" and are absent from F.1**, whose stated count
   is 53. They are written as NF-07 to NF-12 with proposed identifiers FD-FSIM-34 to -38 and FD-F3D-02 (in
   `needs`). If the lead assigns numbers, BD-037's row-count test moves from 53 to 59.
2. **The legs of the first toolchain train in FrankenSim and FrankenSymPy have no `FD-` number** (they appear
   only in the row of FD-FMCP-01). Written as NF-03 and NF-04; the train's first run is NF-05.
3. **"Additive, never a change to existing behaviour" against rows that say "correct"** (FD-FSIM-10, -11, -32).
   Each bead gives the additive form (a corrected entry point beside the present one, the present one
   documented) and leaves correcting in place to the sibling's own rules.
4. **FD-FKER-01's rule "replace local types" against the fact that `franken-kernel` depends on `serde` while
   `frob-core` admits none.** The bead proposes that the upstream change make `serde` optional for the
   identifier types, and gives a two-case switch-over (re-export if possible; otherwise keep local types under
   a five-kind round-trip oracle).
5. **FD-F3D-01's interim posture ("`f3d-*` crates are used with their `serde` feature off") against "franken_threed
   is never a Cargo dependency" and its asupersync 0.5.0 pin.** The bead restricts use to crates that bring no
   asupersync (a HYPOTHESIS recorded at INT-F3D-01) and notes that the asupersync pin has no register item.
6. **Four rows have no switch-over rule** ("—" or "none needed"): FD-FSYM-02, FD-FSYM-05, FD-FLN-02,
   FD-FTUI-01. Each bead says so and gives an adoption procedure labelled DESIGN.
7. **Milestones.** The plan gives a milestone only for the two P0 items. Rule used: the milestone at which the
   item's switch-over gate first exists, never earlier than M1 for a non-P0 item; every bead states that no
   milestone waits on it. Bead priority is the register priority (P0 to P3 as 0 to 3), except the two items
   whose consuming tier is Moonshot (FD-FSIM-22, FD-FTX-01), which are 4 by the charter's rule.
8. **Size.** FD-FT-01 (eleven gateway changes) and FD-FDGR-01 (the sibling's own roadmap to published
   geometry) exceed five days. Each is kept as one bead, as the brief asks, and is written in stages that land
   and are adopted separately; the lead may split them by stage.
9. **INT-FSIM-05's row "incremental against one-shot integration to 10⁻¹²"** on the near-straight case: the
   heading increment per step (3·10⁻¹⁵ rad) is absorbed at θ = 32 rad even by a correct step, so 400 steps and
   one step differ by 1.2·10⁻¹² rad in heading and about 1.2·10⁻¹² m in position. FD-FSIM-11 states this and
   proposes a compensated heading or a computed expected difference; unit U33b owns the test.
10. **FD-FSS-01 is conditional** on a pilot and may be closed as not needed; its first step is recording the
    pilot's finding.
11. **Design choices made here, each labelled DESIGN in its bead:** the train scripts under `scripts/train/`
    (no seventh `xtask` subcommand); the train log `receipts/train-log.ndjson` and `breakage_share_28d`; the
    freeze marker; per-item fixtures as `#[ignore]`d `frob-conformance` integration tests with `expected.toml`;
    per-item switch-over scripts `scripts/e2e/upstream_<id>.sh --phase pre|post`; defect reports stored at
    `tests/upstream/<FD-id>/REPORT.md` and used as the body of the mirrored bead.
12. **STD-E2E.** Every bead that owns a script depends on STD-E2E, except the two P0 beads, which list it in
    `related` so that the first session's toolchain work is not held behind the harness; their gates'
    scripts belong to BD-008 and BD-023.
13. **Labels** carry the sibling's repository name as written (with underscores where it has them).
