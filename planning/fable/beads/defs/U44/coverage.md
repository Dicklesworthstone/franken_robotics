# Coverage of drafting unit U44: §44, work packages

Files in this directory: 80 work-package epics (`WP-01` … `WP-80`), 5 new beads (`N44-01` … `N44-05`), and this file.
The unit owns no seed bead. The epic files were assembled by a script from the plan's own tables (the package rows,
the crate map, the gate register, the kill-gate tables, the seed list) and hand-written prose for each package, so
gate thresholds, kill-gate triggers and fallbacks, and crate roles are quoted from the plan, not retyped.

## (a) Subsections and the beads that carry them

| Subsection | Title | Carried by |
|---|---|---|
| §44 (introduction) | What a work package is; the table regenerated from the bead graph by transitive reduction; carried (WP-01 to WP-33) and new (WP-34 onward) packages; tier tags and the pilot before funding; upstream work items inside packages, the two P0 items in WP-02 and WP-06 | N44-03 (regeneration), N44-04 (tier tags, pilots, upstream placement, carried and new counts), N44-05 (end-to-end run); WP-02 (FD-ASUP-01) and WP-06 (FD-FMCP-01); every epic's Background, Order and Frontier paragraphs |
| §44.1 | The packages | WP-01 … WP-80, one epic per row; N44-03 for the depends-on column |
| §44.2 | Packages that need more than the default envelope | N44-01 (the ten requests as an owner decision and a registry), N44-02 (end-to-end test of refusal and fallback); the Envelope bullet in the Considerations of WP-21, -28, -34, -44, -47, -48, -53, -55, -60, -61, -62, -76, -77, -79 |
| §44.3 | Coverage | N44-04 (crate coverage, milestone coverage, the counts by first milestone, carried and new), N44-05 |

## (b) Gates, kill gates and admission gates

§44 defines no gate, kill gate or admission gate of its own. What the epics carry from the register:

| Kind | Count | Where |
|---|---|---|
| Gate identifiers in the key-gates column of the 80 rows | 318 distinct (33 admission, 285 convergence or program) | The `gates` field and one acceptance line each in the epic of the row; 13 are named by two rows |
| Register gates that no row names, attached to the epic whose seed measures them | 11 | GATE-ARENA-06 in WP-41; GATE-ARENA-08 in WP-21; GATE-ATEL-05 in WP-11; GATE-CORTEX-10 in WP-24; GATE-TENFOLD-01 in WP-34; GATE-TENFOLD-02 in WP-34; GATE-TENFOLD-09 in WP-34; INT-F3D-01 in WP-52; INT-FDGR-01 in WP-32; INT-FJAX-01 in WP-15; INT-FTX-01 in WP-73 |
| Kill gates of the register tables (112 by subsystem and 7 of the ten-fold test) | 119 of 119 | Each in the `gates` field of at least one epic, with trigger, fallback and decision milestone in its Considerations |
| Kill gates defined in later subsections | 25 of 25 | Named with trigger and fallback in the epic that holds the mechanism: KILL-DEVORACLE in WP-14; KILL-ROWQUEUE in WP-15; KILL-LOOPELIM in WP-15; KILL-ANCESTOR in WP-15; KILL-FOIL in WP-24; KILL-ITEMBANK in WP-24; KILL-EXPODOMAIN in WP-24; KILL-MPCRECORD in WP-27; KILL-VISIBILITY in WP-28; KILL-SOILLIFT in WP-33; KILL-SHORTGRAD in WP-33; KILL-LIMBKERNEL in WP-44; KILL-REPLAYPACK in WP-52; KILL-ENSEMBLE in WP-56; KILL-SKEL-CELL in WP-62; KILL-AUGUR-BRACKET in WP-62; KILL-BUDGETCURVE in WP-62; KILL-FREEPERIOD in WP-64; KILL-STRIDESEQ in WP-64; KILL-WEAKVAR in WP-66; KILL-SYNTH-REGRESSOR in WP-67; KILL-FARFIELD in WP-73; KILL-STEPADJOINT in WP-74; KILL-INTERACT in WP-78; KILL-COUPLING in WP-79 |

Per package: first milestone, parent, number of key gates, kill gates carried, seed beads.

| Package | First milestone | Parent | Key gates | Kill gates in `gates` | Seed beads |
|---|---|---|---|---|---|
| WP-01 | M0 | EPIC-M0 | 3 | — | 6 |
| WP-02 | M0 | EPIC-M0 | 3 | KILL-FSIM | 5 |
| WP-03 | M0 | EPIC-M0 | 2 | — | 12 |
| WP-04 | M0 | EPIC-M0 | 5 | — | 7 |
| WP-05 | M0 | EPIC-M0 | 3 | — | 4 |
| WP-06 | M0 | EPIC-M0 | 4 | KILL-MCP | 4 |
| WP-07 | M1 | EPIC-M1 | 4 | KILL-SKEL-SEXPR | 11 |
| WP-08 | M1 | EPIC-M1 | 1 | — | 2 |
| WP-09 | M1 | EPIC-M1 | 6 | — | 10 |
| WP-10 | M1 | EPIC-M1 | 4 | — | 6 |
| WP-11 | M1 | EPIC-M1 | 2 | KILL-ATLAS | 11 |
| WP-12 | M1 | EPIC-M1 | 2 | — | 4 |
| WP-13 | M1 | EPIC-M1 | 1 | — | 4 |
| WP-14 | M1 | EPIC-M1 | 1 | — | 8 |
| WP-15 | M2 | EPIC-M2 | 4 | KILL-KIR | 9 |
| WP-16 | M2 | EPIC-M2 | 2 | KILL-CONTACT | 4 |
| WP-17 | M2 | EPIC-M2 | 2 | KILL-POOL | 4 |
| WP-18 | M2 | EPIC-M2 | 3 | — | 2 |
| WP-19 | M2 | EPIC-M2 | 3 | KILL-METAL | 1 |
| WP-20 | M2 | EPIC-M2 | 3 | — | 2 |
| WP-21 | M2 | EPIC-M2 | 4 | KILL-LEARNKERN | 5 |
| WP-22 | M2 | EPIC-M2 | 4 | — | 4 |
| WP-23 | M3 | EPIC-M3 | 1 | — | 4 |
| WP-24 | M3 | EPIC-M3 | 5 | KILL-SPEC-REWARD | 7 |
| WP-25 | M3 | EPIC-M3 | 4 | KILL-STATIC | 5 |
| WP-26 | M3 | EPIC-M3 | 5 | KILL-SLOT, KILL-PREEMPT, KILL-MCPTASKS | 9 |
| WP-27 | M3 | EPIC-M3 | 5 | KILL-BASELINE, KILL-MPC, KILL-RESIDUAL | 7 |
| WP-28 | M4 | EPIC-M4 | 3 | KILL-VOI, KILL-SOLVE | 7 |
| WP-29 | M4 | EPIC-M4 | 3 | KILL-EQUIV, KILL-MARROW-BLOCKS | 3 |
| WP-30 | M4 | EPIC-M4 | 1 | KILL-SOIL, KILL-RESPONSE | 4 |
| WP-31 | M5 | EPIC-M5 | 5 | KILL-LEMMA | 4 |
| WP-32 | M5 | EPIC-M5 | 2 | KILL-INGEST, KILL-DRESSING | 2 |
| WP-33 | M6 | EPIC-M6 | 0 | KILL-SINEW-AGE | 0 |
| WP-34 | M0 | EPIC-M0 | 4 | KILL-TENFOLD-LEVER | 4 |
| WP-35 | M0 | EPIC-M0 | 2 | — | 3 |
| WP-36 | M0 | EPIC-M0 | 5 | — | 2 |
| WP-37 | M0 | EPIC-M0 | 13 | — | 10 |
| WP-38 | M1 | EPIC-M1 | 5 | KILL-SKEL-SUBST, KILL-SKEL-ALIAS | 8 |
| WP-39 | M1 | EPIC-M1 | 2 | — | 5 |
| WP-40 | M1 | EPIC-M1 | 2 | KILL-SCHED | 5 |
| WP-41 | M1 | EPIC-M1 | 5 | — | 6 |
| WP-42 | M1 | EPIC-M1 | 8 | KILL-AUGUR-FALSECERT | 11 |
| WP-43 | M1 | EPIC-M1 | 8 | — | 8 |
| WP-44 | M2 | EPIC-M2 | 5 | KILL-PHYLUM, KILL-BUCKET | 8 |
| WP-45 | M2 | EPIC-M2 | 6 | KILL-SINEW-ROWS | 5 |
| WP-46 | M2 | EPIC-M2 | 8 | KILL-D1, KILL-MARROW-GUARD, KILL-FLEET, KILL-SPECIALIZE | 9 |
| WP-47 | M2 | EPIC-M2 | 3 | KILL-PARITY | 3 |
| WP-48 | M2 | EPIC-M2 | 4 | KILL-SHARD | 5 |
| WP-49 | M2 | EPIC-M2 | 9 | KILL-EXPOSURE, KILL-ALIAS | 8 |
| WP-50 | M2 | EPIC-M2 | 9 | KILL-SHADOW, KILL-DR | 7 |
| WP-51 | M2 | EPIC-M2 | 7 | — | 4 |
| WP-52 | M2 | EPIC-M2 | 4 | KILL-TUI | 2 |
| WP-53 | M3 | EPIC-M3 | 9 | KILL-FILM, KILL-MESHGPU, KILL-FCLEAR, KILL-BANK-BREADTH, KILL-RETROFIT | 9 |
| WP-54 | M3 | EPIC-M3 | 4 | — | 3 |
| WP-55 | M3 | EPIC-M3 | 8 | KILL-NEMESIS, KILL-RARE, KILL-MCW | 9 |
| WP-56 | M3 | EPIC-M3 | 5 | KILL-AUTOPSY | 7 |
| WP-57 | M3 | EPIC-M3 | 1 | KILL-GYMADAPT | 1 |
| WP-58 | M2 | EPIC-M2 | 8 | — | 8 |
| WP-59 | M3 | EPIC-M3 | 7 | KILL-AUGUR-REGION, KILL-AUGUR-REUSE, KILL-AUGUR-SEPARATE, KILL-AUGUR-PROBE, KILL-AUGUR-ALIAS, KILL-FAMILYCERT | 6 |
| WP-60 | M3 | EPIC-M3 | 8 | KILL-TENFOLD-SPEED, KILL-TENFOLD-TIE, KILL-TENFOLD-CERT, KILL-TENFOLD-LEAK, KILL-TENFOLD-ADJ, KILL-TENFOLD-REACH, KILL-AUGUR-TIE | 6 |
| WP-61 | M4 | EPIC-M4 | 4 | KILL-FAMILY, KILL-INHERIT, KILL-DISTILL, KILL-ALTALGO | 6 |
| WP-62 | M3 | EPIC-M3 | 8 | KILL-RANK, KILL-LADDER | 8 |
| WP-63 | M3 | EPIC-M3 | 8 | KILL-QUOTIENT, KILL-REUSE, KILL-FDGRAD, KILL-FAMILYLANE | 8 |
| WP-64 | M4 | EPIC-M4 | 4 | KILL-GAIT | 4 |
| WP-65 | M4 | EPIC-M4 | 4 | KILL-ENERGY, KILL-SHIELD | 4 |
| WP-66 | M4 | EPIC-M4 | 5 | KILL-AUGUR-CUTS, KILL-AUGUR-DOMINANCE, KILL-AUGUR-LABELS | 7 |
| WP-67 | M4 | EPIC-M4 | 2 | KILL-SYNTH-CODESIGN, KILL-SYNTH-LINKAGE, KILL-SYNTH-TRAJOPT | 3 |
| WP-68 | M1 | EPIC-M1 | 10 | KILL-GROUND, KILL-BAND, KILL-CLOCK, KILL-ASBUILT, KILL-KIT | 13 |
| WP-69 | M5 | EPIC-M5 | 1 | — | 0 |
| WP-70 | M5 | EPIC-M5 | 7 | KILL-LOADS, KILL-TOPOPT | 5 |
| WP-71 | M5 | EPIC-M5 | 5 | KILL-FAB, KILL-TOLERANCE, KILL-PACKET | 5 |
| WP-72 | M5 | EPIC-M5 | 2 | KILL-REFLECT, KILL-VARIATIONAL, KILL-HARDCONTACT, KILL-TREEOP, KILL-SKEL-PORT | 4 |
| WP-73 | M6 | EPIC-M6 | 5 | KILL-ADVECT, KILL-SLOSH, KILL-PROXY, KILL-LIVING | 4 |
| WP-74 | M6 | EPIC-M6 | 2 | KILL-SINEW-ROTOR, KILL-AUGUR-PHASE, KILL-AUGUR-SYMEVENT | 9 |
| WP-75 | M6 | EPIC-M6 | 1 | KILL-LIFECYCLE | 1 |
| WP-76 | M7 | EPIC-M7 | 5 | KILL-TENFOLD-ADJ, KILL-TENFOLD-REACH, KILL-TENFOLD-LEAK | 3 |
| WP-77 | M7 | EPIC-M7 | 3 | KILL-TENFOLD-SPEED, KILL-TENFOLD-TIE, KILL-TENFOLD-CERT | 3 |
| WP-78 | M8 | EPIC-M8 | 1 | KILL-AUGUR-FACILITY | 1 |
| WP-79 | M8 | EPIC-M8 | 4 | KILL-SKILL, KILL-AUGUR-TRANSPORT, KILL-GATREE, KILL-TUBE | 3 |
| WP-80 | M8 | EPIC-M8 | 2 | KILL-SYNTH-CONT, KILL-FIRMWARE, KILL-MEGA | 3 |

## (c) Known answers, counterexamples and refusal codes

- Known answers owned by §44: 0. Cited: KA-823 (the compute-envelope arithmetic), for which N44-01 sets the tolerance
  and a negative control that the plan leaves unset. New exact known answers stated by this unit about the package
  table itself (N44-03, N44-04): 80 packages; 139 depends-on edges; 14 transitively implied edges, hence 125 after
  reduction; no cycle; roots WP-01 and WP-33; longest chain 17 edges ending at WP-77; first-milestone counts 10, 15,
  18, 14, 8, 5, 4, 2, 3 with WP-69 unscheduled; 33 carried and 47 new; 46 names and 157 crate–package pairs in the
  coverage table; 30 packages with a tier tag.
- Counterexamples owned by §44: 0.
- Refusal codes owned by §44: 0. Cited: `FROB-REF-761` and `FROB-REF-704` (the envelope), and in single epics
  `FROB-REF-220`, `-245`, `-582`, `-602`, `-862`, `-863`, `-873`, `-899` and the requirement-language and study codes.

## (d) Needs from other slices

- `STD-E2E`: the shared end-to-end helper (N44-02, N44-05).
- BD-219 (`sim.estimate`): the class a job needs, the class in force and the refusal `FROB-REF-761`; and which of
  `FROB-REF-761` and `FROB-REF-704` fires where (with BD-296).
- The surface unit: a way to run `frob` and `frobd` against a scratch copy of `registries/` (N44-02).
- The decision beads of OPEN-SPARK-2, OPEN-PHYLUM-3, OPEN-NEMESIS-3 and OPEN-PROG-4 (N44-01 records their
  consequences per package; OPEN-CORTEX-1 already has one, N23a-22).
- `registries/gates.toml` with a decision milestone per kill gate (N44-04, check 8), and a crate label on every bead
  (N44-04, check 5).
- The tracker's JSONL export with keys, parent links and blocking links (N44-03); field names are bound on first run.

## (e) Contradictions, gaps and what was done about them

1. **Seed membership.** 335 of the 442 seed beads already have a file from their own unit; the tables of contents follow the `parent` those files state. The other 107 (units U28a 4, U29a 13, U30a 2, U30b 10, U31b 10, U32a 17, U33a 4, U35 3, U36 6, U37 7, U38b 6, U39a 9, U39b 3, U41 13) are assigned by crate, title and gate, and each epic says that the bead's own `parent` governs. Drafted parents that differed from this unit's first reading were adopted: BD-005 to WP-01 (first reading WP-36); BD-068 to WP-09 (WP-37); BD-113 to WP-07 (WP-28); BD-345 to WP-62 (WP-55); BD-141 to WP-14 (WP-01); BD-220 to WP-43 (WP-63); BD-243 to WP-38 (WP-11). The nine milestone exit reviews belong to no package.
2. **The depends-on column is not transitively reduced**, although the plan says the table is the transitive reduction of the graph: 14 of its 139 edges are implied by others (WP-10→WP-02, WP-11→WP-07, WP-15→WP-07, WP-31→WP-05, WP-38→WP-07, WP-41→WP-10, WP-44→WP-15, WP-54→WP-24, WP-58→WP-06, WP-61→WP-48, WP-67→WP-05, WP-68→WP-13, WP-76→WP-60, WP-80→WP-15). The epics keep the column exactly as drafted in `related`; N44-03 computes the reduction from the bead graph and reports the difference.
3. **Package order against bead order.** Two drafted edges point to a package whose first milestone is later (WP-62, first milestone M3, waits for WP-28 and WP-61, both M4). Bead dependencies also run against the column (a bead of WP-23 depends on a bead of WP-53, which waits for WP-23; beads of WP-21 need the environment semantics of WP-48, which waits for WP-21). The quotient of the bead graph by packages is therefore not acyclic. N44-03 works on package parts, one per package and milestone, and reports any cycle that survives.
4. **Seeds outside their row's milestones.** By the membership rule some beads sit in a package whose row names another milestone: BD-025 (M0) and the conformance beads of M2 to M5 in WP-14 (row M1); BD-053 (M1) in WP-35 (M0); BD-144 (M1) in WP-44 (M2); BD-145 (M1) in WP-60 (M3); BD-111 (M1) in WP-63 (M3, M4); BD-201 (M2) in WP-24 (M3); BD-202 (M2) in WP-41 (M1); BD-311 (M3) in WP-68 (M1, M5); BD-348 (M4) in WP-11 (M1); BD-376 (M4) and BD-409 (M5) in WP-26 (M3). Each epic says so under 'What lands at each milestone'. No package owns general static analyses after M1, the verb registry after M3, or the conformance harness at M4; those three assignments are the least certain.
5. **WP-69 has no milestone.** The row says 'None; sourcing may start at M0'. The epic is filed under M5 with parent `EPIC-M5`, because the milestone chapter lists it with the M5 packages and the lane that would receive its data lands there, at priority 4 (optional, outside the critical path). N44-04 treats it as the one unscheduled package.
6. **WP-33 names no crate and no gate.** It is written as a queue of M6 frontier items that no named package holds; its `gates` field holds only KILL-SINEW-AGE, which a drafted decision bead places there, and its acceptance is the per-item plan, pilot and kill decision.
7. **Tier tags are incomplete.** Rows WP-05, WP-52 and WP-63 carry no tag but hold Frontier mechanisms with pilots or kill gates (the derivation annex; the dashboard; campaign quotienting, paired gradients and family lanes); WP-46 is tagged [F] although its M2 services are Solid; WP-33 carries no tag and holds only upper-tier items. The tier labels of those five epics follow the content (`solid` and `frontier`, or `frontier` and `moonshot`), and N44-04 keys its pilot check on the children's labels and reports a name without a tag.
8. **Gates in the register that no row names** (11): GATE-ARENA-06, GATE-ARENA-08, GATE-ATEL-05, GATE-CORTEX-10, GATE-TENFOLD-01, -02, -09, INT-F3D-01, INT-FDGR-01, INT-FJAX-01, INT-FTX-01. Each was attached to the epic whose seed bead measures it and is marked as not being in the row's column.
9. **Gates defined in later subsections are in no row.** The gates the register lists as added after its tables were compiled (for example GATE-KIN-08 to -13, GATE-AUGUR-25 to -35, GATE-ARENA-10 to -15, GATE-CORTEX-18 to -22, GATE-GAUNT-02 to -11, GATE-WS-05 to -09) do not appear in the package table and are not listed in the epics; they are carried by the beads of their chapters. N44-03 regenerates an 'all gates' column from the children so that they reach the table. The 25 kill gates of those subsections are named in the epics.
10. **Rows that list a gate whose seed sits elsewhere.** WP-37's row lists INT-FSIM-02 to -11, but the beads that measure INT-FSIM-08, -09, -10, -11 and the second half of -07 are in WP-10, WP-09, WP-38 or WP-11, WP-04 and WP-39; WP-43's row lists GATE-TETHER-04, whose seed is in WP-26; WP-36's row lists GATE-SAFE-01 and -08, whose M3 seeds are in WP-58; WP-71's row lists GATE-WS-02, whose M1 seed is in WP-14; WP-59's row lists GATE-AUGUR-12, whose engine is in WP-63, and GATE-AUGUR-11, which the register dates M4; WP-49's row lists GATE-SENS-12, whose seeds are in WP-40 and WP-20. The epics keep the row's gate and say where the measuring bead is.
11. **GATE-SAFE-10 is dated M1 by the register**, and the only seed bead that names it (BD-258) is an M3 bead behind the device job protocol. WP-58 records the open point; the lead or the safety unit must say which bead owes the M1 part.
12. **Two refusal codes for one condition**: `FROB-REF-761` (a job exceeds the compute envelope in force) and `FROB-REF-704` (a campaign-class job without an approved compute envelope). N44-01 and N44-02 accept either and ask the estimator's and the daemon's beads to state the division.
13. **Where approvals are recorded** is not fixed by §44.2. N44-01 uses `registries/envelopes.toml`, the registry another unit already uses for the family-policy decision, with the record fields the script guard of the workspace unit accepts, and names its readers; it is to be folded into whatever carrier the estimator's bead chooses.
14. **What regenerates the table** is not named by the plan, and `xtask` may hold exactly six checks. N44-03 and N44-04 are shell scripts outside `xtask`, with a stated deletion rule if no exit review reads their output by the M1 exit.
15. **Size.** The brief asks for epic bodies of 30 to 70 lines; they are 81 to 135 lines (median 103). The excess is the gate briefs, one line per key gate, and the kill gates with trigger and fallback; removing either would lose content the charter asks for. The gate briefs are quoted from the register and keep the register's own section pointers; the full criteria are in the mechanism beads.
