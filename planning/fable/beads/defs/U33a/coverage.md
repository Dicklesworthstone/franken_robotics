# Coverage of drafting unit U33a

Chapter §33 Suite consumption, subsections §33.1 to §33.6 and the chapter preamble; 15 seed beads; the
known-answer registry of the chapter. 50 bead files: 15 seed beads, 34 new beads (`N33a-01` to `N33a-34`) and
the registry `N33-KA`.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §33 preamble | What the chapter fixes; sibling facts are FACT with a path, are re-read by each gate's admission test and are never copied into a record | N33a-09 (the fact registry and probes), N33a-07 (`facts_reread` in every record), N33a-01 (the rule stated with the vocabulary) |
| §33.1 | Postures | N33a-01 (five postures, a posture belongs to a use, the feature refinement, Subprocess as first fallback, the lift registry), N33a-02 (postures per foundation and per use), N33a-03 (posture strings in the lock) |
| §33.2 | The FrankenSim decision | BD-006 (ADR-SUITE-01, the reasons, the precedents, the three repeated root settings, INT-FSIM-01), BD-007 (the digest half), N33a-08 (hazard 5, build identity), N33a-09 (hazards 1 to 8 and the four limits as probes), N33a-10 (hazards 1 and 6 as KILL-FSIM triggers), N33a-14 (hazard 7), N33a-15 (hazard 2), N33a-04 and N33a-05 (hazards 3 and 8) |
| §33.3 | The airlock | N33a-11 (crate, ten feature groups, compile costs, FROB-REF-790), N33a-12 (bedrock-direct list), N33a-13 (rule 1: converts, never reinterprets), N33a-06 (rule 2: no cache across pins), N33a-14 (rule 3: capacities as data), BD-007 (`RefereeDynamics`), N33a-15 (`Integrator`, `PortSystem`), N33a-16 (`GeometryKernel`, `SceneQuery`), N33a-30 (`MeshExchange`, field and topology queries), BD-009 (`Ledger`), N33a-17 (`Session`, `Recompute`), N33a-18 (`Racing`), N33a-31 (`Optimizer`, `Archive`), N33a-19 (`Magnetics`, `ThermalNetwork`, `CurrentLoop`, `PortHamiltonian`), N33a-29 and N33a-34 (end-to-end) |
| §33.4 | Foundation ledger | N33a-02 (all eighteen rows and the four honest notes), BD-009 (FD-FSQL), BD-015 (FD-CUDA), BD-016 and N33a-22 (FD-FT), BD-036 and N33a-21 (FD-FNP), N33a-20 (FD-FSCI, FD-FKER), BD-019 and N33a-32 (FD-FSYM), BD-380 (FD-FLN), BD-300 with N33a-23 and N33a-24 (FD-FTUI, FD-F3D), BD-398 with N33a-25 (FD-FDGR, FD-FSS), BD-416 with N33a-26 (FD-FTX), BD-096 (FD-ASUP lab runtime), BD-220 with N33a-33 (FD-FSQL pager policy). FD-FMCP, FD-FJAX and FD-FFW are rows in N33a-02 whose work belongs to other units |
| §33.5 | The stack lock | N33a-03 (format, parser, stack-lock digest), N33a-04 (the resolution receipt, tree hash, drift classes, path independence, KA-767), N33a-05 (`check-stack` rules, the dirty-tree PASS guard, the read-only rule), N33a-06 (receipt hash on results), N33a-27 (end-to-end), BD-044 (no PASS on a dirty tree at the exit) |
| §33.6 | Admission gates | N33a-07 (record validity, `DEPENDENCY_ADMISSION.md`, never inherited), N33a-28 (end-to-end for M0), N33a-34 (end-to-end for groups and later admissions), and the gate beads of table (b) |

Subsections of other units that this unit's seed beads draw on and carry in full: §33.10 and §33.11 row 3
(BD-036), §33.10 row INT-FSYM-02 (N33a-32, BD-105), §33.15 lab runtime (BD-096), pager policy (N33a-33, BD-220),
frankentui and franken_threed (BD-300, N33a-23, N33a-24), drone reconstruction (BD-398, N33a-25),
franken_textiles (BD-416, N33a-26); §12.4 (BD-380); §19.13 (BD-398); §19.10.6 and §19.10.7 (BD-416); §21.9
(BD-015, BD-016); §28.4 and C-11 (BD-105); §28.14 (BD-220); §29.15 (BD-096); §32.14 and §32.15 (BD-300); §43.2
and §43.3 (BD-044).

## (b) Gates, kill gates and admission gates of the chapter

| Gate | Carried by this unit | Expected from another unit |
|---|---|---|
| INT-TOOLCHAIN-01 | BD-006 (the core row: candidates (a) and (b) for the FrankenSim closure) | BD-031 (the matrix and the choice of T; U33c) |
| INT-FSIM-01 | BD-006 (build and costs), BD-007 (digest), N33a-28 (script) | |
| INT-FSIM-02, -03 | | BD-032 (U33b) |
| INT-FSIM-04 | N33a-14 and N33a-29 exercise the capacity refusal | BD-035 (U33b) |
| INT-FSIM-05 | | BD-033 (U33b) |
| INT-FSIM-06 | | BD-034 (U33b) |
| INT-FSIM-07 | N33a-13 (the typed fixed-witness refusal) | BD-035, BD-093 (U33b) |
| INT-FSIM-08 | N33a-16 supplies the trait it runs through | BD-079 (U33b) |
| INT-FSIM-09 | N33a-19 supplies the traits it runs through | BD-068 (U33b) |
| INT-FSIM-10 | | BD-243 (U33b) |
| INT-FSIM-11 | | BD-020 (U21b) |
| INT-FSIM-13 | | BD-333 (U33b) |
| INT-FSIM-14 | | BD-124 (U33b) |
| INT-FSIM-15 | N33a-30 admits the mesh half at first use (M2) | BD-401 (U33b) |
| INT-FSIM-16 | N33a-17 supplies `Recompute` | BD-111 (U33b) |
| INT-FSIM-17 | BD-416 consumes it | BD-413 (U33b) |
| INT-FSIM-18 | | BD-038 (U33c) |
| INT-ASUP-01 | observed in BD-006, BD-009 | BD-008 (U33b) |
| INT-ASUP-02 | BD-096 | |
| INT-FSQL-01 | BD-009, N33a-28 | |
| INT-FSQL-02 | N33a-33 (the M1 record), BD-220 (re-run on episode traces at M2) | |
| INT-FT-01 | BD-016, N33a-28 | |
| INT-FT-02 | N33a-22 re-reads the host-tensor fact | BD-028 (U33b) |
| INT-CUDA-01 | BD-015, N33a-28 | |
| INT-FSYM-01 | BD-019, N33a-28 | BD-027 (the auditor replay half; U25b) |
| INT-FSYM-02 | N33a-32 (the M0 record), BD-105 (re-run on the leg at M1) | |
| INT-FLN-01, -02 | BD-380 | |
| INT-FMCP-01 | BD-044 records its outcome | BD-023 (U32a) |
| INT-FNP-01, -02 | BD-036, N33a-28 | |
| INT-FTUI-01 | N33a-23 (pilot), BD-300 | |
| INT-F3D-01 | N33a-24 (pilot), BD-300 | |
| INT-FDGR-01 | N33a-25 (M3 schema part), BD-398 (M5 scan import) | |
| INT-FJAX-01 | | BD-221 (U21a) |
| INT-FTX-01 | N33a-26 (pilot and reference half), BD-416 | |
| GATE-WS-06 | | the unit that owns §33.17 (U33c) |
| GATE-WS-09 | | the unit that owns §33.18 (U33c) |
| KILL-FSIM | N33a-10 (the trigger measurements), BD-044 (the M0 ADR) | the decision bead proper, with §33.8 (U33b) |

Gates of other chapters that this unit's seed beads measure or decide: GATE-CHRON-11, CPU part (BD-096);
GATE-ATEL-12, episode part (BD-220); GATE-TETHER-14 and KILL-TUI (BD-300); GATE-HAB-03 and KILL-INGEST
(BD-398); KILL-LIVING (BD-416); GATE-GAUNT-01 and KILL-MCP (BD-044).

End-to-end scripts. Four beads are scripts for the capabilities of §33.1 to §33.6: N33a-27
(`scripts/e2e/stack_receipt.sh`), N33a-28 (`admission_m0.sh`), N33a-29 (`airlock_refusals.sh`) and N33a-34
(`airlock_groups.sh`). Seed beads that deliver a capability of another chapter specify their script inside the
bead and depend on `STD-E2E`: BD-044 (`exit_m0.sh`), BD-096 (`sim_ledger_submit.sh`), BD-105
(`analyze_workspace_singularities.sh`), BD-220 (`memo_episodes.sh`), N33a-33 (`memo_eviction_oracle.sh`),
BD-300 (`dashboard_viewer.sh`), BD-380 (`lean_admission.sh`), BD-398 (`scene_import_real_place.sh`).

Pilots (each its own bead, each a dependency of the mechanism it funds): N33a-23 (FrankenTUI, before BD-300),
N33a-24 (the FrankenThreeD route, before BD-300), N33a-25 (real-place ingestion, before BD-398), N33a-26 (the
cloth oracle, before BD-416). BD-019 with N33a-32 is itself the M0 spike of the derivation route, whose kill
gate KILL-KIR is decided at M2 by another unit.

## (c) Counts

- Known answers: 14 carried in the registry `N33-KA` (KA-767 to KA-780), each with truth, tolerance, tier,
  control and carrying bead. This unit's mechanism beads test KA-767 directly and the small-scale or control
  halves of KA-770, KA-771, KA-777 and KA-778; they also cite KA-074 and KA-429 from other chapters with their
  values. Two rows had gaps that the registry closes: KA-770 (tolerance `to set`) and KA-776 (control `none
  stated`).
- Counterexamples: the chapter has 6 (CX-352 to CX-357); the registry `N33-CX` is written by U33c. This unit's
  test plans cite CX-353 and CX-355 with their cases, and CX-151, CX-180, CX-184 and CX-218 from other
  chapters.
- Refusal codes: the chapter defines 3 (790, 791, 890); the registry `N33-REF` is written by U33c. This unit
  carries the mechanisms of two: `FROB-REF-790` (N33a-11, exercised in N33a-29) and `FROB-REF-890` (N33a-06,
  exercised in N33a-27). `FROB-REF-791` belongs to the worker fallback (BD-038).
- Other identifiers carried: ADR-SUITE-01 (BD-006, N33a-11); BET-04 (BD-006, N33a-10); INV-SUITE-01 (N33a-11,
  N33a-12), INV-SUITE-03 and -04 (N33a-05), INV-SUITE-05 (N33a-06, BD-416), as mechanisms, with their
  statements; OPEN-SUITE-1 (N33a-02), OPEN-SUITE-3 and -5 (BD-006, N33a-04, N33a-05, BD-044), OPEN-SUITE-6
  (N33a-12); RISK-01, -02, -03, -08, -16, -23.

## (d) Needs from other slices

1. Decision beads for OPEN-SUITE-3 and OPEN-SUITE-5 (§33.16), which INT-FSIM-01 decides with the owner; and the
   KILL-FSIM decision bead proper with the compatibility train (§33.8). This unit supplies their measurements.
2. The registry entries of `FROB-REF-790` and `FROB-REF-890`, and a new registered code for a referee refusal
   whose upstream reason has no dedicated code (§33.16, §35.11, Appendix O); N33a-13 cannot close without it.
3. `xtask check-deps` reading the ledger rows, the bedrock-direct list and the group consumer lists (§34.6;
   BD-003), and the `check-stack` subcommand shell calling the library functions of N33a-03 to N33a-05 and
   N33a-07 (BD-004). BD-004 and BD-001 should depend on, or be merged with, N33a-03, N33a-04 and N33a-05.
4. A field in the verb registry naming the airlock groups a verb requires, the `doctor` verb, and the
   envelope's `evidence.stack_receipt` (§32.4, §32.6; BD-022); a `stack_receipt` column in the ledger tables
   (§29.3; BD-090).
5. The gate-record runner of `frob-conformance` and `registries/gates.toml` semantics (§38.13); the pilot
   register (§38.5); registration of this unit's planted failures as negative controls (§38.11).
6. The group gates and adapters of `spectral`, `stats`, `structure` and `multiphysics` (§33.9) and the
   file-level admission tests INT-FSIM-02 to -11 (§33.10).
7. The toolchain matrix, the choice of T, the toolchain train and the units escape hatch (§33.12, §33.13;
   BD-031); the worker fallback (§33.14; BD-038); GATE-WS-06 and GATE-WS-09 (§33.17, §33.18).
8. Upstream work items opened as beads (BD-037): FD-FSIM-03, -08, -12, -13, -14, -18, -21, -22, -26; FD-FNP-01;
   FD-FSQL-01; FD-FT-01, -02; FD-FTUI-01; FD-F3D-01; FD-FKER-01; FD-FLN-01, -02; FD-FDGR-01; FD-FSS-01;
   FD-FTX-01.
9. From other chapters, for seed beads: the M1 job engine and idempotency binding (§32.7, §32.10) for BD-096;
   the hardware fingerprint and the determinism classifier (§21.16, §35) for BD-220; the daemon's event stream
   and read verbs (§32.11) for BD-300; `scene.import` for assets at M3, the KILL-INGEST decision bead and
   OPEN-HAB-5 and -6 (§19.13, §19.19, §19.20) for BD-398; the proxy-adequacy pilot and the KILL-LIVING decision
   bead (§19.10.7) for BD-416; the KILL-MCP and KILL-TUI decision beads (§32.21); the `lean-capsule` proof kind
   (§12.4) for BD-380.

## (e) Contradictions and items that could not be turned into work as written

1. **INT-FSYM-02: M0 in the gate register, M1 in its seed bead.** §40.2 and §40.6 date the gate at M0; seed
   BD-105 (A-05, A-06, the leg kernel) is an M1 bead and keeps its milestone. Added N33a-32, which records the
   gate at M0 on the planar two-link fixture; BD-105 depends on it and re-runs the gate on the leg.
2. **INT-FSQL-02: M1 in the register, M2 in its seed bead.** Added N33a-33, which builds the eviction policy
   and its differential oracle and records the gate at M1 on analysis traces; BD-220 re-runs it on episode
   traces at M2.
3. **Gates of another unit with the same kind of conflict** (flagged, not resolved here): the register dates
   INT-FSIM-08, -09 and -10 at M0, while their seed beads BD-079, BD-068 and BD-243 are M1, M1 and M3; it dates
   INT-FSIM-13 at M3 while BD-333 is M4. BD-044 lists the three M0 gates as blocking the M0 exit if no record
   exists; N33a-34 checks -08, -09 and -16 at M1.
4. **`fs-io` is in the default-on `geometry` group (§33.3) but its known answer is under the `structure` gate
   INT-FSIM-15 at M5 (§33.9).** Resolved by the never-inherited rule: the mesh half is admitted at first use
   (N33a-30, M2), the bounded STEP half stays with INT-FSIM-15.
5. **No refusal code exists for an upstream refusal other than capacity and indeterminate contact**, although
   §33.3 requires a typed referee refusal for every FrankenSim refusal. N33a-13 maps what has a code and
   requests one code for the rest; it uses no placeholder number.
6. **"Evidence-bearing performance builds export a receipt" (§33.2, hazard 5) gives no form for the receipt.**
   N33a-08 adopts the fail-closed reading: until the form is read from the pinned build script, every build is
   a development-salt build and no Performance or Parity PASS can be recorded.
7. **The group table names three traits for eight `geometry` crates and three for ten `optimize` crates**, and
   gives "reached by" lists only for the groups of §33.9. N33a-11 fixes initial consumer lists as DESIGN data;
   N33a-30 adds a field-query extension for the representation crates; the trait-less `optimize` entries
   (`fs-sos`, `fs-symmetry`, `fs-tropical`) are left to the beads that admit them (BD-127, BD-243, BD-332).
8. **"The digest computed inside FrankenSim" against the rule that no sibling checkout is modified.** BD-007
   computes it in an export of the pinned commit (`git archive`) with a probe crate placed in the export, the
   mechanism FrankenSim's own out-of-workspace crates use.
9. **`xtask` is capped at 3 thousand lines and may record no gate.** The lock parser, receipt writer,
   verification rules, lift validator and admission validator are all assigned to it by §34.6. The writer of
   admission records is therefore in `frob-conformance` and only validation is in `xtask`; the cap may still
   need an ADR.
10. **INT-FTX-01 compares a candidate cloth model that exists only if the Moonshot tier is funded.** N33a-26
    records the reference half and the worker's readiness, and the gate as `not run` with its reason until a
    candidate exists.
11. **Seed BD-300 spans two artifacts** (`frob-tui` and the viewer). Kept as one feature in two parts, as the
    seed requires, with one pilot bead for each.
12. **`scene.import` is registered from M3 for assets, but the only seed bead naming it (BD-398) is M5.**
    BD-398 covers the real-place lane and states the asset lane as a need.
