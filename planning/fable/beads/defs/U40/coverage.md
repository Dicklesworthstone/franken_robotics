# Coverage of drafting unit U40: §40 (the gate register) and §43 (milestones M0–M8)

54 bead files: 6 seed beads (the exit reviews BD-224, BD-319, BD-378, BD-412, BD-427, BD-442), 9 milestone epics
(EPIC-M0 to EPIC-M8), 37 new beads N40-01 to N40-37 and 2 registries (N40-KA, N40-CX).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §40 preamble | What the register is; its counts | N40-02, N40-10, N40-KA |
| §40.1 | Semantics (the nine rules) | N40-01 (all nine); N40-03 (rule 2); N40-04 (rules 5, 6, 8); N40-07 (rule 7); N40-09 (rules 4, 9); N40-02 (rule 3) |
| §40.2 | Admission gates | N40-02 (rows, the three notes); N40-07 (first-needed by exit; never inherited); EPIC-M0, -M1, -M2, -M3, -M5, -M6 (each row with its criterion in brief). The gates themselves are the suite units' |
| §40.3 | Convergence gates by subsystem; claims the register assigned; milestones the register assigned; shared fixtures | N40-02 (counts, claims, two-claim readings, milestone assignment); N40-08 (shared fixtures); EPIC-M0 to EPIC-M8 (every row with its criterion in brief, by exit). The gates themselves are the chapter units' |
| §40.4 | Kill gates | N40-23 (foundations, 13); N40-24 (the body, 35); N40-25 and N40-26 (the mind, 17 + 28); N40-27 (record, world, surface, 19); N40-03 (record semantics); N40-30 (cross-check and schedule); EPIC-M0 to EPIC-M8 and the six exit reviews (by exit) |
| §40.5 | Program gates; the kill gates of the ten-fold test; bets and the gates that decide them | N40-11 (schedule gate); N40-12 (GATE-TENFOLD-01); N40-13 (-02); N40-14 (-03, -04, -05); N40-15 (-06); N40-16 (-07); N40-17 (-08); N40-18 (-09); N40-19 (-10); N40-20 (-11); N40-21 (-12); N40-22 (-13); N40-28 (seven kill gates); N40-31 (37 bets) |
| §40.6 | Gates by milestone exit; the load by exit | N40-04 (the query and the load table); EPIC-M0 to EPIC-M8 (the lists); N40-30 (kill decisions by exit); N40-KA R-11 to R-13 |
| §40.7 | Discrepancies resolved by this register | N40-09 (kill gates stated in two chapters; admission gates stated twice; KILL-MCP; rows against the milestone skeleton); N40-08 (related thresholds under different identifiers); N40-24, N40-26, N40-27 (the resolutions restated on the rows they govern) |
| §40.8 | Gates defined by subsections added after the register was compiled | N40-10 (index, amended gates, the EXPODOMAIN note); N40-29 (the 22 kill gates); N40-08 (the two related-threshold rows printed after the table); EPIC-M0 to EPIC-M8 (by exit, with full statements) |
| §40.9 | Gates defined in subsections added by the final audit | N40-10 (index); N40-29 (KILL-FREEPERIOD, KILL-STRIDESEQ, KILL-DEVORACLE); N40-14 (GATE-TENFOLD-04, -05); EPIC-M0 to EPIC-M8 (by exit, with full statements) |
| §43 preamble | A milestone is a set of demonstrations, not a date; the seven fields | N40-06; EPIC-M0 to EPIC-M8 |
| §43.1 | The shape of the program, and why M1 is a CPU studio | N40-06 (table, critical path, estimates, the three reasons); EPIC-M1; N40-11 (the 10.5-week threshold) |
| §43.2 | Exit rules, and the dependency firewall in brief | N40-05 (exit rules, procedure, the cap on Moonshot items); N40-07 (firewall table); N40-04 (the query); each epic (its firewall row) |
| §43.3 | M0: spine and airlock | EPIC-M0 (the exit review is seed BD-044, another unit's); N40-12; N40-23, N40-27; N40-35 |
| §43.4 | M1: referee studio and first mission slice (CPU only) | EPIC-M1 (the exit review is seed BD-147, another unit's); N40-11; N40-13; N40-18; N40-35 |
| §43.5 | M2: batch, backends, families; the no-GPU variant | EPIC-M2; BD-224; N40-33 (the variant); N40-24 |
| §43.6 | M3: places, tasks, adversaries, qualification | EPIC-M3; BD-319; N40-14, N40-15, N40-19, N40-20, N40-22 |
| §43.7 | M4: amortized control, co-design, mission queries at scale; the variant with no family policy | EPIC-M4; BD-378; N40-34 (the variant) |
| §43.8 | M5: proofs, reality, manufacture | EPIC-M5; BD-412 |
| §43.9 | M6: frontier physics | EPIC-M6; BD-427 |
| §43.10 | M7: the ten-fold tournament | EPIC-M7 (the exit review is seed BD-434, another unit's); N40-16, N40-17, N40-21, N40-22 |
| §43.11 | M8: frontier delivery | EPIC-M8; BD-442 |
| §43.12 | No milestone is blocked by one failed bet | N40-32 (the twenty rows and their rehearsals); each epic (its rows) |

Per-milestone fields and where each landed: purpose, artifact, demonstrations, acceptance, work packages → the epic;
kill gates decided → the epic, the exit review and the ledger beads; owner input wanted → the epic, the exit review
and N40-35; stop or re-plan decision → the epic and the exit review (and N40-11 for the schedule gate).

## (b) Gates, kill gates and admission gates, and the beads that measure or decide them

The register collects gates that the chapters define. For those, this unit carries the register row (identity,
claim, milestone parts, the brief) and the exit at which each is read; the chapter units carry the measurement.
Program gates and the fallback ledger are this unit's.

**Program gates (§40.5).**

| Gate | Bead that records and applies it | Exit review that reads it |
|---|---|---|
| The schedule gate (BET-12) | N40-11 | BD-147 |
| GATE-TENFOLD-01 | N40-12 | BD-044 |
| GATE-TENFOLD-02 | N40-13 | BD-147 |
| GATE-TENFOLD-03, -04, -05 | N40-14 | BD-319, BD-434 |
| GATE-TENFOLD-06 | N40-15 | BD-319 |
| GATE-TENFOLD-07 | N40-16 | BD-434 |
| GATE-TENFOLD-08 | N40-17 | BD-434 |
| GATE-TENFOLD-09 | N40-18 | Every exit from M1 |
| GATE-TENFOLD-10 | N40-19 | BD-319 |
| GATE-TENFOLD-11 | N40-20 | BD-319, BD-434 |
| GATE-TENFOLD-12 | N40-21 | BD-434 |
| GATE-TENFOLD-13 | N40-22 | BD-319, BD-434 |
| KILL-TENFOLD-SPEED, -TIE, -CERT, -LEAK, -LEVER, -ADJ, -REACH | N40-28 (ledger); N40-13, N40-14, N40-15, N40-16, N40-18, N40-22 (application) | BD-147, BD-224, BD-319, BD-434 |
| GATE-GAUNT-01, GATE-GAUNT-10 (as they bear on the register) | N40-01, N40-04, N40-36, N40-CX | Every exit |

**Kill gates (§40.4, §40.8, §40.9): ledger bead and deciding exit.** The decision bead of each is the owning chapter
unit's; N40-30 checks it against the ledger row.

| Kill gate | Ledger bead | Decided | Exit review that reads the decision |
|---|---|---|---|
| KILL-FSIM | N40-23 | M0, then rolling | BD-044 |
| KILL-FAMILYCERT | N40-23 | M3 | BD-319 |
| KILL-LEMMA | N40-23 | M5 | BD-412 |
| KILL-REFLECT | N40-23 | M5 | BD-412 |
| KILL-MARROW-BLOCKS | N40-23 | M4 | BD-378 |
| KILL-MARROW-GUARD | N40-23 | M2 | BD-224 |
| KILL-SKEL-ALIAS | N40-23 | M3 | BD-319 |
| KILL-SKEL-PORT | N40-23 | M5 | BD-412 |
| KILL-SKEL-SEXPR | N40-23 | M1 | BD-147 |
| KILL-SKEL-SUBST | N40-23 | M1 | BD-147 |
| KILL-SINEW-AGE | N40-23 | M6, before the lifecycle work package | BD-427 |
| KILL-SINEW-ROTOR | N40-23 | M6 | BD-427 |
| KILL-SINEW-ROWS | N40-23 | M2 | BD-224 |
| KILL-SHADOW | N40-24 | M3 | BD-319 |
| KILL-TREEOP | N40-24 | M5 | BD-412 |
| KILL-VARIATIONAL | N40-24 | M5 | BD-412 |
| KILL-FAMILYLANE | N40-24 | M4 | BD-378 |
| KILL-FDGRAD | N40-24 | M4; the pilot runs before funding | BD-378 |
| KILL-PHYLUM | N40-24 | M2 | BD-224 |
| KILL-QUOTIENT | N40-24 | M3 | BD-319 |
| KILL-CONTACT | N40-24 | M2 | BD-224 |
| KILL-FILM | N40-24 | M3 | BD-319 |
| KILL-ADVECT | N40-24 | M6, with GATE-HAB-07 | BD-427 |
| KILL-HARDCONTACT | N40-24 | M5 | BD-412 |
| KILL-MESHGPU | N40-24 | M3 at the earliest, before the work package | BD-319 |
| KILL-RESPONSE | N40-24 | M4 | BD-378 |
| KILL-SOIL | N40-24 | M4 | BD-378 |
| KILL-BANK-BREADTH | N40-24 | M3 | BD-319 |
| KILL-DRESSING | N40-24 | M5 | BD-412 |
| KILL-FCLEAR | N40-24 | M3 | BD-319 |
| KILL-INGEST | N40-24 | M5 | BD-412 |
| KILL-LIVING | N40-24 | M6, before the island work package | BD-427 |
| KILL-POOL | N40-24 | M2 | BD-224 |
| KILL-PROXY | N40-24 | M6 | BD-427 |
| KILL-RETROFIT | N40-24 | M4 | BD-378 |
| KILL-SLOSH | N40-24 | M6 | BD-427 |
| KILL-ALIAS | N40-24 | M3 | BD-319 |
| KILL-EXPOSURE | N40-24 | M2 | BD-224 |
| KILL-SCHED | N40-24 | M2 | BD-224 |
| KILL-BUCKET | N40-24 | M2 | BD-224 |
| KILL-D1 | N40-24 | M2 | BD-224 |
| KILL-FLEET | N40-24 | M4 | BD-378 |
| KILL-KIR | N40-24 | M2 | BD-224 |
| KILL-MEGA | N40-24 | M8, before the work package | BD-442 |
| KILL-METAL | N40-24 | M2 | BD-224 |
| KILL-SPECIALIZE | N40-24 | M4 | BD-378 |
| KILL-PARITY | N40-24 | M2 | BD-224 |
| KILL-SHARD | N40-24 | M2 | BD-224 |
| KILL-BASELINE | N40-25 | M3 | BD-319 |
| KILL-ENERGY | N40-25 | M4 | BD-378 |
| KILL-GAIT | N40-25 | M4 | BD-378 |
| KILL-MPC | N40-25 | M4 | BD-378 |
| KILL-RESIDUAL | N40-25 | M4 | BD-378 |
| KILL-SHIELD | N40-25 | M4 | BD-378 |
| KILL-SKILL | N40-25 | M8 | BD-442 |
| KILL-TUBE | N40-25 | M8 | BD-442 |
| KILL-ALTALGO | N40-25 | M4 at the earliest, per algorithm, before funding | BD-378 |
| KILL-DISTILL | N40-25 | M4 | BD-378 |
| KILL-EQUIV | N40-25 | M4 | BD-378 |
| KILL-FAMILY | N40-25 | M4 | BD-378 |
| KILL-GATREE | N40-25 | M8 | BD-442 |
| KILL-GYMADAPT | N40-25 | M3 | BD-319 |
| KILL-INHERIT | N40-25 | M4 | BD-378 |
| KILL-LEARNKERN | N40-25 | M2 | BD-224 |
| KILL-SPEC-REWARD | N40-25 | M3 | BD-319 |
| KILL-AUGUR-ALIAS | N40-26 | M3 | BD-319 |
| KILL-AUGUR-CUTS | N40-26 | M4 | BD-378 |
| KILL-AUGUR-DOMINANCE | N40-26 | M4 | BD-378 |
| KILL-AUGUR-FACILITY | N40-26 | M8 | BD-442 |
| KILL-AUGUR-FALSECERT | N40-26 | Standing, from M1 | BD-147 |
| KILL-AUGUR-LABELS | N40-26 | M4 | BD-378 |
| KILL-AUGUR-PHASE | N40-26 | M6 | BD-427 |
| KILL-AUGUR-PROBE | N40-26 | M3 | BD-319 |
| KILL-AUGUR-REGION | N40-26 | M3 | BD-319 |
| KILL-AUGUR-REUSE | N40-26 | Standing, from M3 | BD-319 |
| KILL-AUGUR-SEPARATE | N40-26 | M3 | BD-319 |
| KILL-AUGUR-SYMEVENT | N40-26 | M6 | BD-427 |
| KILL-AUGUR-TIE | N40-26 | M3 | BD-319 |
| KILL-AUGUR-TRANSPORT | N40-26 | M8 | BD-442 |
| KILL-SYNTH-CODESIGN | N40-26 | M4 | BD-378 |
| KILL-SYNTH-CONT | N40-26 | M8 | BD-442 |
| KILL-SYNTH-LINKAGE | N40-26 | M4 | BD-378 |
| KILL-SYNTH-TRAJOPT | N40-26 | M4 | BD-378 |
| KILL-MCW | N40-26 | M3 | BD-319 |
| KILL-NEMESIS | N40-26 | M3 | BD-319 |
| KILL-RARE | N40-26 | M3 | BD-319 |
| KILL-AUTOPSY | N40-26 | M3 | BD-319 |
| KILL-ATLAS | N40-26 | M4 | BD-378 |
| KILL-LADDER | N40-26 | M4 | BD-378 |
| KILL-RANK | N40-26 | M4 | BD-378 |
| KILL-REUSE | N40-26 | M4 | BD-378 |
| KILL-STATIC | N40-26 | M3 | BD-319 |
| KILL-VOI | N40-26 | M4 | BD-378 |
| KILL-DR | N40-27 | M3 | BD-319 |
| KILL-ASBUILT | N40-27 | M5 | BD-412 |
| KILL-BAND | N40-27 | M5 | BD-412 |
| KILL-CLOCK | N40-27 | M5 | BD-412 |
| KILL-GROUND | N40-27 | M5 | BD-412 |
| KILL-KIT | N40-27 | M5 | BD-412 |
| KILL-FAB | N40-27 | M5 | BD-412 |
| KILL-FIRMWARE | N40-27 | M8 | BD-442 |
| KILL-LIFECYCLE | N40-27 | M6 | BD-427 |
| KILL-LOADS | N40-27 | M5 | BD-412 |
| KILL-PACKET | N40-27 | M5 | BD-412 |
| KILL-TOLERANCE | N40-27 | M5 | BD-412 |
| KILL-TOPOPT | N40-27 | M5 | BD-412 |
| KILL-MCP | N40-27 | M0; re-read at M1 with the M1 verbs | BD-044, BD-147 |
| KILL-MCPTASKS | N40-27 | M3 | BD-319 |
| KILL-PREEMPT | N40-27 | M3 | BD-319 |
| KILL-SLOT | N40-27 | M3 | BD-319 |
| KILL-SOLVE | N40-27 | M4 | BD-378 |
| KILL-TUI | N40-27 | M3 | BD-319 |
| KILL-DEVORACLE | N40-29 | M1 | BD-147 |
| KILL-ROWQUEUE | N40-29 | M2 | BD-224 |
| KILL-LOOPELIM | N40-29 | M2 | BD-224 |
| KILL-ANCESTOR | N40-29 | M2 | BD-224 |
| KILL-LIMBKERNEL | N40-29 | M2 | BD-224 |
| KILL-MPCRECORD | N40-29 | M3 | BD-319 |
| KILL-FOIL | N40-29 | M3 | BD-319 |
| KILL-ITEMBANK | N40-29 | M3 | BD-319 |
| KILL-EXPODOMAIN | N40-29 | M3 (the exit at which the last part of GATE-ARENA-14 is due; the gate's first part is due at M2) | BD-319 |
| KILL-ENSEMBLE | N40-29 | M3 | BD-319 |
| KILL-REPLAYPACK | N40-29 | M3 | BD-319 |
| KILL-SKEL-CELL | N40-29 | M4 | BD-378 |
| KILL-VISIBILITY | N40-29 | M4 | BD-378 |
| KILL-AUGUR-BRACKET | N40-29 | M4 | BD-378 |
| KILL-SYNTH-REGRESSOR | N40-29 | M4 | BD-378 |
| KILL-WEAKVAR | N40-29 | M4 | BD-378 |
| KILL-BUDGETCURVE | N40-29 | M4 | BD-378 |
| KILL-FREEPERIOD | N40-29 | M4 | BD-378 |
| KILL-STRIDESEQ | N40-29 | M4 | BD-378 |
| KILL-COUPLING | N40-29 | M4 for the finite part (a violated transported bound); M8 for the tray clause | BD-378, BD-442 |
| KILL-STEPADJOINT | N40-29 | M6 | BD-427 |
| KILL-FARFIELD | N40-29 | M6 | BD-427 |
| KILL-SOILLIFT | N40-29 | M6 | BD-427 |
| KILL-SHORTGRAD | N40-29 | M6 | BD-427 |
| KILL-INTERACT | N40-29 | M8 | BD-442 |
| KILL-TENFOLD-SPEED | N40-28 | M3 for the classes run; M7 | BD-319, BD-434 |
| KILL-TENFOLD-TIE | N40-28 | M3; M7 | BD-319, BD-434 |
| KILL-TENFOLD-CERT | N40-28 | Standing, from M3 | BD-319, BD-434 |
| KILL-TENFOLD-LEAK | N40-28 | Standing, from M3 | BD-319, BD-434 |
| KILL-TENFOLD-LEVER | N40-28 | M1; M2 | BD-147, BD-224 |
| KILL-TENFOLD-ADJ | N40-28 | M3; M7 | BD-319, BD-434 |
| KILL-TENFOLD-REACH | N40-28 | M3; M7 | BD-319, BD-434 |

**Admission and convergence gates (§40.2, §40.3, §40.8, §40.9): the epic or epics that list each with its part and
criterion in brief.** The measuring bead of each is the owning chapter unit's.

| Gate | Exits with a part due | Epics that list it |
|---|---|---|
| INT-TOOLCHAIN-01 | M0 | EPIC-M0 |
| INT-FSIM-01 | M0 | EPIC-M0 |
| INT-FSIM-02 | M0 | EPIC-M0 |
| INT-FSIM-03 | M0 | EPIC-M0 |
| INT-FSIM-04 | M0 | EPIC-M0 |
| INT-FSIM-05 | M0 | EPIC-M0 |
| INT-FSIM-06 | M0 | EPIC-M0 |
| INT-FSIM-07 | M0, M1 | EPIC-M0, EPIC-M1 |
| INT-FSIM-08 | M0 | EPIC-M0 |
| INT-FSIM-09 | M0 | EPIC-M0 |
| INT-FSIM-10 | M0 | EPIC-M0 |
| INT-FSIM-11 | M0 | EPIC-M0 |
| INT-FSIM-13 | M3 | EPIC-M3 |
| INT-FSIM-14 | M3 | EPIC-M3 |
| INT-FSIM-15 | M5 | EPIC-M5 |
| INT-FSIM-16 | M1 | EPIC-M1 |
| INT-FSIM-17 | M6 | EPIC-M6 |
| INT-FSIM-18 | M0 | EPIC-M0 |
| INT-ASUP-01 | M0 | EPIC-M0 |
| INT-ASUP-02 | M2 | EPIC-M2 |
| INT-FSQL-01 | M0 | EPIC-M0 |
| INT-FSQL-02 | M1 | EPIC-M1 |
| INT-FT-01 | M0 | EPIC-M0 |
| INT-FT-02 | M0 | EPIC-M0 |
| INT-CUDA-01 | M0 | EPIC-M0 |
| INT-FSYM-01 | M0 | EPIC-M0 |
| INT-FSYM-02 | M0 | EPIC-M0 |
| INT-FLN-01 | M5 | EPIC-M5 |
| INT-FLN-02 | M5 | EPIC-M5 |
| INT-FMCP-01 | M0, M1, M3 | EPIC-M0, EPIC-M1, EPIC-M3 |
| INT-FNP-01 | M0 | EPIC-M0 |
| INT-FNP-02 | M0 | EPIC-M0 |
| INT-FTUI-01 | M3 | EPIC-M3 |
| INT-F3D-01 | M3 | EPIC-M3 |
| INT-FDGR-01 | M3, M5 | EPIC-M3, EPIC-M5 |
| INT-FJAX-01 | M2 | EPIC-M2 |
| INT-FTX-01 | M6 | EPIC-M6 |
| GATE-LEMMA-00 | M0 | EPIC-M0 |
| GATE-LEMMA-01 | M5 | EPIC-M5 |
| GATE-LEMMA-02 | M5 | EPIC-M5 |
| GATE-LEMMA-03 | M1 | EPIC-M1 |
| GATE-LEMMA-04 | M1, M4 | EPIC-M1, EPIC-M4 |
| GATE-LEMMA-05 | M5 | EPIC-M5 |
| GATE-LEMMA-06 | M0 | EPIC-M0 |
| GATE-MARROW-01 | M0 | EPIC-M0 |
| GATE-MARROW-02 | M1 | EPIC-M1 |
| GATE-MARROW-03 | M3 | EPIC-M3 |
| GATE-MARROW-04 | M4 | EPIC-M4 |
| GATE-MARROW-05 | M2 | EPIC-M2 |
| GATE-SKEL-01 | M1 | EPIC-M1 |
| GATE-SKEL-02 | M1 | EPIC-M1 |
| GATE-SKEL-03 | M1, M3 | EPIC-M1, EPIC-M3 |
| GATE-SKEL-04 | M1, M3 | EPIC-M1, EPIC-M3 |
| GATE-SKEL-05 | M1 | EPIC-M1 |
| GATE-SKEL-06 | M1 | EPIC-M1 |
| GATE-SKEL-07 | M1, M3 | EPIC-M1, EPIC-M3 |
| GATE-SKEL-08 | M1 | EPIC-M1 |
| GATE-SINEW-01 | M1 | EPIC-M1 |
| GATE-SINEW-02 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-SINEW-03 | M2 | EPIC-M2 |
| GATE-SINEW-04 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-SINEW-05 | M1 | EPIC-M1 |
| GATE-SINEW-06 | M1 | EPIC-M1 |
| GATE-SINEW-07 | M1 | EPIC-M1 |
| GATE-SINEW-08 | M1 | EPIC-M1 |
| GATE-SINEW-09 | M2 | EPIC-M2 |
| GATE-KIN-01 | M1 | EPIC-M1 |
| GATE-KIN-02 | M1 | EPIC-M1 |
| GATE-KIN-03 | M2 | EPIC-M2 |
| GATE-KIN-04 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-KIN-05 | M2 | EPIC-M2 |
| GATE-KIN-06 | M2 | EPIC-M2 |
| GATE-KIN-07 | M2 | EPIC-M2 |
| GATE-FID-01 | M2 | EPIC-M2 |
| GATE-FID-02 | M3 | EPIC-M3 |
| GATE-FID-03 | M2 | EPIC-M2 |
| GATE-FID-04 | M3 | EPIC-M3 |
| GATE-FID-05 | M3 | EPIC-M3 |
| GATE-FID-06 | M2 | EPIC-M2 |
| GATE-FID-07 | M2 | EPIC-M2 |
| GATE-FID-08 | M3 | EPIC-M3 |
| GATE-PHYLUM-01 | M3 | EPIC-M3 |
| GATE-PHYLUM-02 | M2 | EPIC-M2 |
| GATE-PHYLUM-03 | M2 | EPIC-M2 |
| GATE-PHYLUM-04 | M2 | EPIC-M2 |
| GATE-PHYLUM-05 | M3 | EPIC-M3 |
| GATE-PHYLUM-06 | M4 | EPIC-M4 |
| GATE-PHYLUM-07 | M3 | EPIC-M3 |
| GATE-PHYLUM-08 | M4 | EPIC-M4 |
| GATE-TRAC-01 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-TRAC-02 | M1 | EPIC-M1 |
| GATE-TRAC-03 | M2 | EPIC-M2 |
| GATE-TRAC-04 | M2 | EPIC-M2 |
| GATE-TRAC-05 | M3 | EPIC-M3 |
| GATE-TRAC-06 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-TRAC-07 | M3 | EPIC-M3 |
| GATE-TRAC-08 | M4 | EPIC-M4 |
| GATE-TRAC-09 | M5, M6 | EPIC-M5, EPIC-M6 |
| GATE-TRAC-10 | M2 | EPIC-M2 |
| GATE-HAB-01 | M2 | EPIC-M2 |
| GATE-HAB-02 | M3 | EPIC-M3 |
| GATE-HAB-03 | M5 | EPIC-M5 |
| GATE-HAB-04 | M3 | EPIC-M3 |
| GATE-HAB-05 | M3 | EPIC-M3 |
| GATE-HAB-06 | M3 | EPIC-M3 |
| GATE-HAB-07 | M6 | EPIC-M6 |
| GATE-HAB-08 | M6 | EPIC-M6 |
| GATE-HAB-09 | M6 | EPIC-M6 |
| GATE-HAB-10 | M3 | EPIC-M3 |
| GATE-HAB-11 | M4 | EPIC-M4 |
| GATE-HAB-12 | M3 | EPIC-M3 |
| GATE-HAB-13 | M2 | EPIC-M2 |
| GATE-HAB-14 | M3 | EPIC-M3 |
| GATE-SENS-01 | M2 | EPIC-M2 |
| GATE-SENS-02 | M2 | EPIC-M2 |
| GATE-SENS-03 | M2 | EPIC-M2 |
| GATE-SENS-04 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-SENS-05 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-SENS-06 | M2 | EPIC-M2 |
| GATE-SENS-07 | M3 | EPIC-M3 |
| GATE-SENS-08 | M1, M3 | EPIC-M1, EPIC-M3 |
| GATE-SENS-09 | M1, M3 | EPIC-M1, EPIC-M3 |
| GATE-SENS-10 | M3 | EPIC-M3 |
| GATE-SENS-11 | M3 | EPIC-M3 |
| GATE-SENS-12 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-SENS-13 | M2 | EPIC-M2 |
| GATE-SENS-14 | M3 | EPIC-M3 |
| GATE-SPARK-00 | M0 | EPIC-M0 |
| GATE-SPARK-01 | M0 | EPIC-M0 |
| GATE-SPARK-02 | M2 | EPIC-M2 |
| GATE-SPARK-03 | M2 | EPIC-M2 |
| GATE-SPARK-04 | M2 | EPIC-M2 |
| GATE-SPARK-05 | M2 | EPIC-M2 |
| GATE-SPARK-06 | M2 | EPIC-M2 |
| GATE-SPARK-07 | M0 | EPIC-M0 |
| GATE-SPARK-08 | M0 | EPIC-M0 |
| GATE-SPARK-09 | M2 | EPIC-M2 |
| GATE-SPARK-10 | M2 | EPIC-M2 |
| GATE-SPARK-11 | M2 | EPIC-M2 |
| GATE-SPARK-12 | M2 | EPIC-M2 |
| GATE-SPARK-13 | M4 | EPIC-M4 |
| GATE-SPARK-14 | M4 | EPIC-M4 |
| GATE-SPARK-15 | M2 | EPIC-M2 |
| GATE-SPARK-16 | M2 | EPIC-M2 |
| GATE-SPARK-17 | M3 | EPIC-M3 |
| GATE-SPARK-18 | M2 | EPIC-M2 |
| GATE-SPARK-19 | M2 | EPIC-M2 |
| GATE-SPARK-20 | M2 | EPIC-M2 |
| GATE-SPARK-21 | M0, M1 | EPIC-M0, EPIC-M1 |
| GATE-SPARK-22 | M2 | EPIC-M2 |
| GATE-REFLEX-01 | M1, M2, M3, M4 | EPIC-M1, EPIC-M2, EPIC-M3, EPIC-M4 |
| GATE-REFLEX-02 | M3 | EPIC-M3 |
| GATE-REFLEX-03 | M1 | EPIC-M1 |
| GATE-REFLEX-04 | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-REFLEX-05 | M3 | EPIC-M3 |
| GATE-REFLEX-06 | M4 | EPIC-M4 |
| GATE-REFLEX-07 | M4 | EPIC-M4 |
| GATE-REFLEX-08 | M4 | EPIC-M4 |
| GATE-REFLEX-09 | M8 | EPIC-M8 |
| GATE-GAIT-01 | M4 | EPIC-M4 |
| GATE-GAIT-02 | M4 | EPIC-M4 |
| GATE-GAIT-03 | M4 | EPIC-M4 |
| GATE-SHIELD-01 | M4 | EPIC-M4 |
| GATE-SHIELD-02 | M4 | EPIC-M4 |
| GATE-SHIELD-03 | M4 | EPIC-M4 |
| GATE-SHIELD-04 | M4 | EPIC-M4 |
| GATE-SHIELD-05 | M8 | EPIC-M8 |
| GATE-CORTEX-01 | M2 | EPIC-M2 |
| GATE-CORTEX-02 | M2 | EPIC-M2 |
| GATE-CORTEX-03 | M4 | EPIC-M4 |
| GATE-CORTEX-04 | M3 | EPIC-M3 |
| GATE-CORTEX-05 | M2 | EPIC-M2 |
| GATE-CORTEX-06 | M2 | EPIC-M2 |
| GATE-CORTEX-07 | M2 | EPIC-M2 |
| GATE-CORTEX-08 | M2 | EPIC-M2 |
| GATE-CORTEX-09 | M2 | EPIC-M2 |
| GATE-CORTEX-10 | M3 | EPIC-M3 |
| GATE-CORTEX-11 | M4 | EPIC-M4 |
| GATE-CORTEX-12 | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-CORTEX-13 | M4 | EPIC-M4 |
| GATE-CORTEX-14 | M4 | EPIC-M4 |
| GATE-CORTEX-15 | M3 | EPIC-M3 |
| GATE-CORTEX-16 | M3 | EPIC-M3 |
| GATE-CORTEX-17 | M8 | EPIC-M8 |
| GATE-ARENA-01 | M2 | EPIC-M2 |
| GATE-ARENA-02 | M3 | EPIC-M3 |
| GATE-ARENA-03 | M1 | EPIC-M1 |
| GATE-ARENA-04 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-ARENA-05 | M1 | EPIC-M1 |
| GATE-ARENA-06 | M2 | EPIC-M2 |
| GATE-ARENA-07 | M3 | EPIC-M3 |
| GATE-ARENA-08 | M2 | EPIC-M2 |
| GATE-ARENA-09 | M3 | EPIC-M3 |
| GATE-QUAL-01 | M3 | EPIC-M3 |
| GATE-QUAL-02 | M3 | EPIC-M3 |
| GATE-QUAL-03 | M5 | EPIC-M5 |
| GATE-AUGUR-01 | M1 | EPIC-M1 |
| GATE-AUGUR-02 | M1 | EPIC-M1 |
| GATE-AUGUR-03 | M1 | EPIC-M1 |
| GATE-AUGUR-04 | M1 | EPIC-M1 |
| GATE-AUGUR-05 | M1 | EPIC-M1 |
| GATE-AUGUR-06 | M1, M3 | EPIC-M1, EPIC-M3 |
| GATE-AUGUR-07 | M1 | EPIC-M1 |
| GATE-AUGUR-08 | M3 | EPIC-M3 |
| GATE-AUGUR-09 | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-AUGUR-10 | M3 | EPIC-M3 |
| GATE-AUGUR-11 | M4 | EPIC-M4 |
| GATE-AUGUR-12 | M3 | EPIC-M3 |
| GATE-AUGUR-13 | M3 | EPIC-M3 |
| GATE-AUGUR-14 | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-AUGUR-15 | M4 | EPIC-M4 |
| GATE-AUGUR-16 | M4 | EPIC-M4 |
| GATE-AUGUR-17 | M4 | EPIC-M4 |
| GATE-AUGUR-18 | M4 | EPIC-M4 |
| GATE-AUGUR-19 | M4 | EPIC-M4 |
| GATE-AUGUR-20 | M6 | EPIC-M6 |
| GATE-AUGUR-21 | M6 | EPIC-M6 |
| GATE-AUGUR-22 | M8 | EPIC-M8 |
| GATE-AUGUR-23 | M8 | EPIC-M8 |
| GATE-AUGUR-24 | M8 | EPIC-M8 |
| GATE-NEMESIS-01 | M3 | EPIC-M3 |
| GATE-NEMESIS-02 | M3 | EPIC-M3 |
| GATE-NEMESIS-03 | M3 | EPIC-M3 |
| GATE-NEMESIS-04 | M3 | EPIC-M3 |
| GATE-NEMESIS-05 | M3 | EPIC-M3 |
| GATE-NEMESIS-06 | M3 | EPIC-M3 |
| GATE-NEMESIS-07 | M4 | EPIC-M4 |
| GATE-NEMESIS-08 | M3 | EPIC-M3 |
| GATE-AUTOPSY-01 | M2 | EPIC-M2 |
| GATE-AUTOPSY-02 | M2 | EPIC-M2 |
| GATE-AUTOPSY-03 | M3 | EPIC-M3 |
| GATE-AUTOPSY-04 | M3 | EPIC-M3 |
| GATE-AUTOPSY-05 | M3 | EPIC-M3 |
| GATE-AUTOPSY-06 | M3 | EPIC-M3 |
| GATE-AUTOPSY-07 | M2 | EPIC-M2 |
| GATE-AUTOPSY-08 | M4 | EPIC-M4 |
| GATE-ATEL-01 | M3 | EPIC-M3 |
| GATE-ATEL-02 | M1, M4 | EPIC-M1, EPIC-M4 |
| GATE-ATEL-03 | M4 | EPIC-M4 |
| GATE-ATEL-04 | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-ATEL-05 | M1, M4 | EPIC-M1, EPIC-M4 |
| GATE-ATEL-06 | M1 | EPIC-M1 |
| GATE-ATEL-07 | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-ATEL-08 | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-ATEL-09 | M4 | EPIC-M4 |
| GATE-ATEL-10 | M4 | EPIC-M4 |
| GATE-ATEL-11 | M1, M3, M4 | EPIC-M1, EPIC-M3, EPIC-M4 |
| GATE-ATEL-12 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-ATEL-13 | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-ATEL-14 | M4 | EPIC-M4 |
| GATE-ATEL-15 | M4 | EPIC-M4 |
| GATE-ATEL-16 | M4 | EPIC-M4 |
| GATE-CHRON-01 | M2 | EPIC-M2 |
| GATE-CHRON-02 | M2 | EPIC-M2 |
| GATE-CHRON-03 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-CHRON-04 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-CHRON-05 | M2 | EPIC-M2 |
| GATE-CHRON-06 | M2 | EPIC-M2 |
| GATE-CHRON-07 | M2 | EPIC-M2 |
| GATE-CHRON-08 | M2 | EPIC-M2 |
| GATE-CHRON-09 | M3 | EPIC-M3 |
| GATE-CHRON-10 | M1 | EPIC-M1 |
| GATE-CHRON-11 | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-CHRON-12 | M2 | EPIC-M2 |
| GATE-CHRON-13 | M2 | EPIC-M2 |
| GATE-GROUND-01 | M1 | EPIC-M1 |
| GATE-GROUND-02 | M5 | EPIC-M5 |
| GATE-GROUND-03 | M5 | EPIC-M5 |
| GATE-GROUND-04 | M5 | EPIC-M5 |
| GATE-GROUND-05 | M5 | EPIC-M5 |
| GATE-GROUND-06 | M5 | EPIC-M5 |
| GATE-GROUND-07 | M5 | EPIC-M5 |
| GATE-GROUND-08 | M5 | EPIC-M5 |
| GATE-GROUND-09 | M5 | EPIC-M5 |
| GATE-GROUND-10 | tied to no milestone (listed at M5) | EPIC-M5 |
| GATE-FOUNDRY-01 | M5 | EPIC-M5 |
| GATE-FOUNDRY-02 | M5 | EPIC-M5 |
| GATE-FOUNDRY-03 | M5 | EPIC-M5 |
| GATE-FOUNDRY-04 | M5 | EPIC-M5 |
| GATE-FOUNDRY-05 | M5 | EPIC-M5 |
| GATE-FOUNDRY-06 | M5 | EPIC-M5 |
| GATE-FOUNDRY-07 | M5 | EPIC-M5 |
| GATE-FOUNDRY-08 | M5 | EPIC-M5 |
| GATE-FOUNDRY-09 | M5 | EPIC-M5 |
| GATE-FOUNDRY-10 | M5 | EPIC-M5 |
| GATE-FOUNDRY-11 | M8 | EPIC-M8 |
| GATE-FOUNDRY-12 | M6 | EPIC-M6 |
| GATE-TETHER-01 | M3 | EPIC-M3 |
| GATE-TETHER-02 | M1 | EPIC-M1 |
| GATE-TETHER-03 | M0 | EPIC-M0 |
| GATE-TETHER-04 | M2 | EPIC-M2 |
| GATE-TETHER-05 | M2 | EPIC-M2 |
| GATE-TETHER-06 | M2 | EPIC-M2 |
| GATE-TETHER-07 | M1 | EPIC-M1 |
| GATE-TETHER-08 | M2 | EPIC-M2 |
| GATE-TETHER-09 | M3 | EPIC-M3 |
| GATE-TETHER-10 | M3 | EPIC-M3 |
| GATE-TETHER-11 | M3 | EPIC-M3 |
| GATE-TETHER-12 | M3 | EPIC-M3 |
| GATE-TETHER-13 | M4 | EPIC-M4 |
| GATE-TETHER-14 | M3 | EPIC-M3 |
| GATE-SAFE-01 | M1 | EPIC-M1 |
| GATE-SAFE-02 | M3 | EPIC-M3 |
| GATE-SAFE-03 | M3 | EPIC-M3 |
| GATE-SAFE-04 | M3 | EPIC-M3 |
| GATE-SAFE-05 | M3 | EPIC-M3 |
| GATE-SAFE-06 | M2 | EPIC-M2 |
| GATE-SAFE-07 | M2 | EPIC-M2 |
| GATE-SAFE-08 | M1 | EPIC-M1 |
| GATE-SAFE-09 | M3 | EPIC-M3 |
| GATE-SAFE-10 | M1 | EPIC-M1 |
| GATE-WS-01 | M0 | EPIC-M0 |
| GATE-WS-02 | M1, M5 | EPIC-M1, EPIC-M5 |
| GATE-WS-03 | M1 | EPIC-M1 |
| GATE-WS-04 | M0 | EPIC-M0 |
| GATE-GAUNT-01 | every exit | EPIC-M0, EPIC-M1, EPIC-M2, EPIC-M3, EPIC-M4, EPIC-M5, EPIC-M6, EPIC-M7, EPIC-M8 |
| GATE-WS-05 (later subsection, §34.14) | M0 | EPIC-M0 |
| GATE-WS-06 (later subsection, §33.17) | M0, M2 | EPIC-M0, EPIC-M2 |
| GATE-WS-07 (later subsection, §41.5) | M0, M2, M3 | EPIC-M0, EPIC-M2, EPIC-M3 |
| GATE-WS-09 (later subsection, §33.18) | M0, M2 | EPIC-M0, EPIC-M2 |
| GATE-GAUNT-02 (later subsection, §38.15) | M0 | EPIC-M0 |
| GATE-GAUNT-05 (later subsection, §38.15) | M0 | EPIC-M0 |
| GATE-GAUNT-08 (later subsection, §38.15) | M0 | EPIC-M0 |
| GATE-GAUNT-10 (later subsection, §38.15) | M0 | EPIC-M0 |
| GATE-MARROW-06 (later subsection, §13.11) | M1 | EPIC-M1 |
| GATE-SKEL-11 (later subsection, §14.17) | M1 | EPIC-M1 |
| GATE-SINEW-10 (later subsection, §15.15) | M1 | EPIC-M1 |
| GATE-TRAC-12 (later subsection, §18.20) | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-AUGUR-31 (later subsection, §25.35) | M1 | EPIC-M1 |
| GATE-AUGUR-32 (later subsection, §25.36) | M1 | EPIC-M1 |
| GATE-CHRON-14 (later subsection, §29.22) | M1, M2 | EPIC-M1, EPIC-M2 |
| GATE-WS-08 (later subsection, §47.4) | M1 | EPIC-M1 |
| GATE-FID-09 (later subsection, §38.17) | M1, M4, M8 | EPIC-M1, EPIC-M4, EPIC-M8 |
| GATE-GAUNT-04 (later subsection, §38.15) | M1, M3 | EPIC-M1, EPIC-M3 |
| GATE-GAUNT-09 (later subsection, §38.15) | M1 | EPIC-M1 |
| GATE-GAUNT-11 (later subsection, §38.18) | M1 | EPIC-M1 |
| GATE-KIN-08 (later subsection, §16.19) | M2 | EPIC-M2 |
| GATE-KIN-11 (later subsection, §16.22) | M2 | EPIC-M2 |
| GATE-KIN-12 (later subsection, §16.23) | M2 | EPIC-M2 |
| GATE-KIN-13 (later subsection, §16.24) | M2 | EPIC-M2 |
| GATE-PHYLUM-09 (later subsection, §17.21) | M2 | EPIC-M2 |
| GATE-SENS-15 (later subsection, §20.23) | M2 | EPIC-M2 |
| GATE-SENS-18 (later subsection, §20.25) | M2 | EPIC-M2 |
| GATE-SPARK-23 (later subsection, §21.26) | M2 | EPIC-M2 |
| GATE-CORTEX-18 (later subsection, §23.25) | M2 | EPIC-M2 |
| GATE-CORTEX-19 (later subsection, §23.26) | M2 | EPIC-M2 |
| GATE-CORTEX-20 (later subsection, §23.27) | M2 | EPIC-M2 |
| GATE-ARENA-10 (later subsection, §24.19) | M2 | EPIC-M2 |
| GATE-ARENA-14 (later subsection, §24.22) | M2, M3 | EPIC-M2, EPIC-M3 |
| GATE-HAB-16 (later subsection, §19.22) | M2 | EPIC-M2 |
| GATE-GAUNT-03 (later subsection, §38.15) | M2 | EPIC-M2 |
| GATE-GAUNT-06 (later subsection, §38.15) | M2, M3 | EPIC-M2, EPIC-M3 |
| GATE-GAUNT-07 (later subsection, §38.15) | M2 | EPIC-M2 |
| GATE-SKEL-09 (later subsection, §14.16) | M3 | EPIC-M3 |
| GATE-SENS-16 (later subsection, §20.24) | M3 | EPIC-M3 |
| GATE-REFLEX-11 (later subsection, §22.22) | M3 | EPIC-M3 |
| GATE-CORTEX-21 (later subsection, §23.28) | M3 | EPIC-M3 |
| GATE-ARENA-11 (later subsection, §24.20) | M3 | EPIC-M3 |
| GATE-ARENA-12 (later subsection, §24.21) | M3 | EPIC-M3 |
| GATE-ARENA-13 (later subsection, §24.21) | M3 | EPIC-M3 |
| GATE-ARENA-15 (later subsection, §24.23) | M3 | EPIC-M3 |
| GATE-AUGUR-26 (later subsection, §25.30) | M3 | EPIC-M3 |
| GATE-AUGUR-27 (later subsection, §25.31) | M3 | EPIC-M3 |
| GATE-AUGUR-33 (later subsection, §25.37) | M3, M4 | EPIC-M3, EPIC-M4 |
| GATE-AUTOPSY-09 (later subsection, §27.19) | M3 | EPIC-M3 |
| GATE-ATEL-17 (later subsection, §28.25) | M3 | EPIC-M3 |
| GATE-ATEL-19 (later subsection, §28.26) | M3 | EPIC-M3 |
| GATE-FOUNDRY-15 (later subsection, §31.23) | M3 | EPIC-M3 |
| GATE-TETHER-15 (later subsection, §32.23) | M3 | EPIC-M3 |
| GATE-TETHER-16 (later subsection, §32.24) | M3 | EPIC-M3 |
| GATE-SKEL-10 (later subsection, §14.16) | M4 | EPIC-M4 |
| GATE-TRAC-11 (later subsection, §18.19) | M4 | EPIC-M4 |
| GATE-SENS-17 (later subsection, §20.24) | M4 | EPIC-M4 |
| GATE-SHIELD-06 (later subsection, §22.21) | M4 | EPIC-M4 |
| GATE-SHIELD-07 (later subsection, §22.25) | M4 | EPIC-M4 |
| GATE-GAIT-04 (later subsection, §22.23) | M4 | EPIC-M4 |
| GATE-GAIT-05 (later subsection, §22.24) | M4 | EPIC-M4 |
| GATE-AUGUR-25 (later subsection, §25.29) | M4, M8 | EPIC-M4, EPIC-M8 |
| GATE-AUGUR-28 (later subsection, §25.32) | M4 | EPIC-M4 |
| GATE-AUGUR-29 (later subsection, §25.33) | M4 | EPIC-M4 |
| GATE-AUGUR-30 (later subsection, §25.34) | M4 | EPIC-M4 |
| GATE-AUGUR-34 (later subsection, §25.38) | M4 | EPIC-M4 |
| GATE-AUGUR-35 (later subsection, §25.39) | M4 | EPIC-M4 |
| GATE-ATEL-18 (later subsection, §28.25) | M4 | EPIC-M4 |
| GATE-FOUNDRY-13 (later subsection, §31.21) | M5 | EPIC-M5 |
| GATE-FOUNDRY-14 (later subsection, §31.22) | M5 | EPIC-M5 |
| GATE-GROUND-11 (later subsection, §30.24) | M5 | EPIC-M5 |
| GATE-KIN-10 (later subsection, §16.21) | M6 | EPIC-M6 |
| GATE-HAB-15 (later subsection, §19.21) | M6 | EPIC-M6 |
| GATE-REFLEX-10 (later subsection, §22.20) | M6 | EPIC-M6 |
| GATE-CORTEX-22 (later subsection, §23.29) | M6 | EPIC-M6 |
| GATE-KIN-09 (later subsection, §16.20) | M8 | EPIC-M8 |

## (c) Counts

- Subsections covered: 21 (§40.1 to §40.9; §43.1 to §43.12), plus the two chapter preambles.
- Gates carried: 37 admission gates, 279 convergence gates of the compiled tables, 76 gates of later subsections,
  13 ten-fold gates and the schedule gate, each listed in at least one epic or program-gate bead; 144 kill gates
  (112 + 7 + 25), each with a ledger row. 550 identifiers in all.
- Known answers of Appendix M: 0. The suite has no section for §40 or §43. N40-KA carries 29 register-level known
  answers (R-01 to R-29) with local identifiers.
- Counterexamples of Appendix N: 0. The catalogue has no section for §40 or §43. N40-CX carries 56 register-level
  planted defects (RC-01 to RC-56) with local identifiers.
- Refusal codes (Appendix O): 0 defined by these chapters. The register's validator has ten write-time rejection
  reasons (N40-01) and further named reasons in N40-03, N40-04, N40-07, N40-08, N40-09, N40-18 and N40-30; these
  are test-harness reasons, not agent-facing `FROB-REF` codes, and none is proposed for Appendix O.
- Invariants and decisions named: INV-GAUNT-02, -05, -06, -07; INV-TENFOLD-01 to -08; ADR-GAUNT-01 to -06;
  doctrine D-6; bets BET-01 to BET-37 (N40-31); risks RISK-33, RISK-52; North-Star scenarios NS-01 to NS-24
  (each in the epic of its milestone).

## (d) Needs from other slices

| Need | Owner (as far as the briefs show) | Used by |
|---|---|---|
| The status-accounting bead of §38.13 (base gate record, GATE-GAUNT-10), the per-run "control caught" mark of §38.11, the pilot register, the method-experiment registry, and the beads for GATE-GAUNT-02 to -11 and KILL-DEVORACLE | The Gauntlet unit (§38) | N40-01, N40-04, N40-10, N40-18, N40-29, BD-427, BD-442 |
| `frob.beadlog/1` extended with `blocked`/`unblocked` events carrying a cause tag; the artifact graph | The Chronicle unit (seed BD-095) | N40-11, N40-01, N40-36 |
| A `[[gate]]` declaration block in each crate's `contract.toml` (owner or consumer; claim; parts; trigger, fallback, decision point); the ADR path and numbering convention; `check-generated` extended to the register printout and exit reports | The workspace unit (§34) | N40-09, N40-30, N40-03, N40-02 |
| The bet beads BET-01 to BET-37; the pilot ledger; a registry of the thesis's levers | The unit that owns Part 0 and §1 to §8 | N40-31, N40-32, N40-13, N40-18, BD-427, BD-442 |
| The ten-fold harness, sealed suite, arms, adjudicator, estimator, scorer and replication kit; the decision beads of the seven KILL-TENFOLD rules | The ten-fold units (§39) | N40-12 to N40-22, N40-28 |
| The risk register and the open-question register as data; the tracker conversion rules | The unit that owns §41, §42, §45.10–45.11 | N40-05, N40-35, N40-30 |
| The work-package epics with a recorded justification per package (speed lever, or trust, coverage, correctness) | The unit that owns §44 | N40-18, N40-13 |
| The detailed dependency firewall, the admission record format, the stack lock's pins; the upstream work-item registry with landed state and interim postures | The suite units (§33) and the Appendix F unit | N40-07, N40-11 |
| The decision bead of every kill gate (label `kill-gate`), the tier list of every gate, the defining bead of every late gate | Every chapter unit | N40-23 to N40-30, N40-33, N40-10 |
| Epics for the tree-operator route on devices and the mesh path on devices, which §43.9 lists for M6 and which have no seed epic | The Kinesis and Traction units | BD-427 |
| Dependencies to add on beads this unit does not write: BD-044 (M0 exit) on N40-04, N40-05, N40-12; BD-147 (M1 exit) on N40-04, N40-05, N40-11, N40-13, N40-18; BD-434 (M7 exit) on N40-04, N40-05, N40-14, N40-16, N40-17, N40-20, N40-21, N40-22 | The lead | Exit reviews of M0, M1, M7 |

## (e) Contradictions, gaps and what was done

1. **Two status lists.** §40.1 lists `not run`, `pass`, `fail`, `killed: fallback adopted`, `blocked: ⟨open
   question⟩`; §38.13's schema lists `not-run`, `pass`, `fail`, `killed`, `no-claim`, `superseded`. N40-01 uses the
   union of seven and records the reconciliation as an ADR.
2. **One status per gate against parts at two milestones.** §38.13's record has a single `milestone` and `status`;
   rule 6 of §40.1 gives 35 convergence gates and 3 admission gates a part at more than one exit. N40-01 adds
   `[[gate.part]]`; this must be reconciled with the Gauntlet unit's bead (fail-closed until then: any unrecorded
   part prints "not run").
3. **Counts.** §38.13 states 421 identifiers; the compiled tables hold 448 (37 + 279 + 13 + 112 + 7), and 550 with
   the later subsections and the schedule gate. Treated as a snapshot taken earlier; the tooling recomputes, and
   N40-KA pins the current counts.
4. **§40.6's program column is incomplete.** It omits GATE-TENFOLD-09 at M1, -10, -11, -13 at M3 and -11, -12, -13
   at M7, which §40.5 and §43 state. The program-gate rows govern (N40-04, N40-09).
5. **§43 lists against the register.** §43.4 omits the M1 part of INT-FSIM-07; §43.5 omits INT-FJAX-01. The
   register governs (N40-09). No other difference was found when every list was expanded and compared.
6. **Two stray rows after the table of §40.8** (training time against the incumbent; no-authorization flags) belong
   to the related-thresholds table of §40.7; carried in N40-08.
7. **§40.9's statements are cut at a fixed width, mid-word.** The cut text is not used; the epics and N40-10 take
   each statement in full from its defining row.
8. **Late kill gates without a decision point.** The neighbour rule was applied (N40-29). Three readings are DESIGN
   and flagged: KILL-EXPODOMAIN at M3 (its neighbour gate has parts at M2 and M3); KILL-COUPLING at M4 and M8 (two
   clauses); GATE-FID-09 row 4 at M8 (with continuation-based synthesis).
9. **The schedule gate has no identifier and its "because" clause no decision rule.** N40-11 uses
   `GATE-PROG-SCHEDULE` in the registry (DESIGN) and a but-for reading with three branches, flagged for the owner's
   ratification with the written reading as fail-closed default.
10. **Where ADRs live is not stated.** DESIGN default `docs/delivery/adr/` (N40-03).
11. **No verb and no `xtask` subcommand may record or report gates** (ADR-GAUNT-01; `xtask` has exactly six
    checks). The exit query, lints and rehearsals are therefore test targets of `frob-conformance`, and the
    end-to-end scripts drive them with `cargo test` and `cargo xtask check-generated` and drive the `frob` binary
    for what users see (`frob capabilities`, `frob evidence trace`, the demonstration verbs).
12. **New registry records are DESIGN.** `registries/milestones.toml` and the `[[amendment]]`, `[[variant]]`,
    `[[bet]]`, `[[fallback_exit]]`, `[[owner_input]]`, `[[shared_fixture]]`, `[[related_threshold]]`,
    `[[resolved]]` records in `registries/gates.toml` are proposed here; each names its reader, as the plan's rule
    on registries requires.
13. **Seed BD-427's dependencies omit two M6 epics** (BD-413 advected films, BD-415 articulated proxies); both
    were added. Every other seed dependency is kept unchanged.
14. **Late gates whose defining row prints two claims.** N40-10 gives a procedure for the governing claim and
    leaves the row `blocked` until the owning unit confirms; no reading was invented per row.
15. **A default with no stated value.** §43.4 asks for "the owner's reading of the M0 ledger"; it has no question
    identifier and no default. N40-35 records a DESIGN default and marks it as such.
16. **"One repair cycle" and "one optimization pass" are used by dozens of triggers and defined nowhere.** N40-03
    gives a DESIGN definition to be recorded as an ADR before the first kill decision.
17. **Titles of the six seed beads were sharpened**; keys, milestones and seed dependencies are unchanged.
