# Coverage of drafting unit U33c

Chapter §33 Suite consumption, subsections §33.12 to §33.18, the seed beads BD-001, BD-031 and BD-038, and the
chapter registries N33-CX and N33-REF.

Files: 53 beads. Three seed beads (BD-001, BD-031, BD-038), two registry beads (N33-CX, N33-REF), 48 new beads
(N33c-01 to N33c-48). By milestone: M0 35, M1 6, M2 8, M3 3, M6 1. Largest bead 194 lines (BD-001), smallest 110
(N33c-04).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §33.12 | The `fs-qty` coupling and the `frob-units` escape hatch | BD-031 (the gate: matrix, decision rule, choice of T, the two ADRs); N33c-01 (the matrix harness: six build sets, four nightlies, nineteen cells); N33c-02 (the coupling graph: direct and indirect dependents, crates freed by FD-FSIM-31); N33c-03 (`frob_core::qty` as the one import path, the closed alias set, compile-fail tests, the alias-count gate of 64); N33c-04 (decision OPEN-SUITE-4); N33c-05 (`frob-units`, conditional: dimension vector, aliases, macro products, checked runtime quantity, cap 3 thousand lines); N33c-06 (conversions in `frob-fsim`, the differential test, the re-export switch); the three ways the coupling bites are in BD-031 and N33c-05; the decision rule also in N33c-08 (the later-train clause) |
| §33.13 | Toolchain facts across the suite | N33c-07 (the fifteen-repository fact table as a re-read survey); BD-001 (one toolchain file naming candidate (a) as the provisional T; the three root settings); BD-031 (the two candidates and the choice); N33c-08 (consequence 1: the toolchain train and the one-toolchain rule in `check-stack`, the `frob-cert` and `worker/` exceptions, the M7 freeze); N33c-26 (consequence 2: the fastmcp_rust facts re-read; the upstream work and the gate are seed BD-023, unit U32a); N33c-09 (consequence 3: no asupersync 0.5.0, the frankentui executor forbidden, frankenjax and franken_threed never dependencies); N33c-29 (one asupersync package per binary); N33c-35 (the version decision) |
| §33.14 | The worker-process fallback | BD-038 (INT-FSIM-18: the gate measurement); N33c-10 (the job protocol: job, chunk, refuse, canonical bytes, job hash, fixed partition, framing); N33c-11 (mode `remote`: client, idempotent submission, resume, cancellation; ADR-SUITE-02; what the fallback preserves and costs); N33c-12 (`frob-fsim-worker` and the `worker/` workspace); N33c-13 (supervision, one worker per device or process slot, the receipt handshake, `FROB-REF-791`); N33c-14 (the numeric trigger for fallback (b) at M2; teleport, shadow sampling and promotion through the worker) |
| §33.15 | What the other siblings contribute | asupersync's lab runtime: N33c-15 (admission, fault schedules, replay fixtures, the virtual-clock fallback, the caution about `lab/conformal.rs`), N33c-16 (the lab-clock device stub and the dual run), N33c-17 (INT-ASUP-02). FrankenSQLite's pager policies: N33c-18 (S3-FIFO state machine), N33c-19 (bytes, cost-weighted admission, pins, LRU fallback), N33c-20 (INT-FSQL-02). frankenjax's e-graph: N33c-21 (ceiling and pilot, INT-FJAX-01), N33c-22 (the pass, conditional). frankentui: N33c-23 (INT-FTUI-01 as pilot). franken_threed: N33c-24 (INT-F3D-01 as pilot). Drone reconstruction: N33c-25 (INT-FDGR-01, schema half; CX-355; the kill rule). fastmcp_rust: N33c-26. The table of siblings off every critical path: N33c-09 (postures as data and deny rows for franken-kernel, franken_textiles, franken_firmware, franken_networkx, franken_markdown, frankenmermaid), N33c-27 (franken-kernel identifier oracle), N33c-28 (INT-FTX-01). The five sibling gates as rows: N33c-23, N33c-24, N33c-25, N33c-21, N33c-28 |
| §33.16 | Invariants, refusals and open questions | Invariants: N33c-29 (INV-SUITE-01, -02), N33c-30 (INV-SUITE-03, -04), N33c-31 (INV-SUITE-05). Refusals: N33-REF (all three), N33c-32 (`FROB-REF-790`), N33c-13 (`FROB-REF-791`), N33c-31 (`FROB-REF-890`). Open questions: N33c-35 (OPEN-SUITE-1 and OPEN-SUITE-6), BD-031 and BD-001 (OPEN-SUITE-2, decided, recorded), N33c-33 (OPEN-SUITE-3), N33c-04 (OPEN-SUITE-4), N33c-34 (OPEN-SUITE-5) |
| §33.17 | Further foundation facts and their admission tests (added late) | N33c-36 (row 5: `MixedWrench`, the hand case, the probe and mutant); N33c-37 (row 6: the planted linear-phase sampler); N33c-38 (row 7: `serde_json` pins, the conflict probe, the `ftx-drape` plant); N33c-39 (row 8: the `metal` binding facts, the language version, the 64-bit atomic record); N33c-40 (GATE-WS-06); N33c-47 (the upstream work items FD-FSIM-32, -33, FD-FTX-01, FD-F3D-01); N33c-28 (row 7's consequence for the cloth oracle) |
| §33.18 | Six later foundation facts and their admission tests (added late) | N33c-41 (row 9: G1 catalog mass); N33c-42 (row 10: SE(3) differential limit, the closed form and hand case); N33c-43 (row 11: transformer sign); N33c-44 (row 12: swept sphere and the arc); N33c-45 (row 13: `fs-sos` bounds, the two rules); N33c-46 (row 14: ray direction renormalized); N33c-47 (the six upstream work items described without identifiers); N33c-48 (GATE-WS-09 and the fixed-upstream review) |
| Appendix N.30 | Counterexamples of §33 | N33-CX (CX-352 to CX-357, and nine unnumbered shortcut cases of §33.17 and §33.18) |
| Appendix O | Refusal codes of §33 | N33-REF (`FROB-REF-790`, `-791`, `-890`) |

Identifiers carried: ADR-SUITE-02 (N33c-10, N33c-11, N33c-12, BD-038); the ADR for the choice of T and the ADR
for the `frob-units` decision (BD-031); INV-SUITE-01 to -05 (five of five); OPEN-SUITE-1 to -6 (six of six);
BET-04 (BD-031, BD-038, N33c-11, N33c-14); risks RISK-01, -02, -03, -12, -19, -20, -21, -22, -23, -24, -48, -55 and
pre-mortem rows 2, 24 and 25, each in the Considerations of the bead it concerns; the two late risk rows about an
exact pin blocking an oracle and a sampler with a wrong joint law (N33c-38, N33c-28, N33c-37); theorem card T-27
(N33c-21, N33c-22); construction C-32 (N33c-45); metamorphic relation MR-37 (BD-038, N33c-11, N33c-14).

## (b) Gates, kill gates and the beads that measure or decide them

Gates whose mechanism is in this unit's subsections, or whose seed bead is this unit's:

| Gate | Milestone | Measured or decided by | Mechanism and supporting beads |
|---|---|---|---|
| INT-TOOLCHAIN-01 | M0 | BD-031 | N33c-01 (matrix), N33c-02 (coupling record), N33c-03 (the `fs-qty` import plant), N33c-07 (survey), N33c-08 (trains after the gate), BD-001 (provisional T). The core row is also in seed BD-006 (U33a) |
| INT-FSIM-18 | M0 | BD-038 | N33c-10, N33c-11, N33c-12, N33c-13 |
| INT-ASUP-02 | M2 (register); lab runtime first wired at M1 | N33c-17 | N33c-15, N33c-16; the M1 ledger and submit-path tests are seed BD-096 (U33a) |
| INT-FSQL-02 | M1 (register); cited again at M2 by seed BD-220 (U33a) | N33c-20 | N33c-18, N33c-19 |
| INT-FJAX-01 | M2 at the earliest, only if the profile asks | N33c-21 (ceiling decision and pilot) | N33c-22 exists only if the pilot passes; seed BD-221 (U21a) names the gate in its title |
| INT-FTUI-01 | M3 | N33c-23 (the gate run as the pilot) | The dashboard itself is seed BD-300 (U33a) |
| INT-F3D-01 | M3 | N33c-24 (the gate run as the pilot) | Viewer overlays: seed BD-300 (U33a) |
| INT-FDGR-01 | M3 (schema fixtures); M5 (scan import) | N33c-25 (M3 half) | The M5 half: seed BD-398 (U33a) |
| INT-FTX-01 | M6 | N33c-28 | N33c-38 (the pin conflict), N33c-10 (the job framing); the epic is seed BD-416 (U33a) |
| GATE-WS-06 | M0; the sampler row at M2 | N33c-40 (rows 5, 7, 8 at M0); N33c-37 (row 6 at M2) | N33c-36, N33c-38, N33c-39. Listed in §40.8 without a row of its own |
| GATE-WS-09 | M0; rows 12 and 14 at M2 | N33c-48 (rows 9, 10, 11, 13); N33c-44 (row 12) and N33c-46 (row 14) at M2 | N33c-41, N33c-42, N33c-43, N33c-45, N33c-47 |
| ✖ KILL-FSIM | M0, then rolling | BD-038 (shows whether fallback (a) functions); N33c-14 (the numeric trigger for fallback (b), M2) | The gate is defined in §33.8 (U33b); the M0 decision is recorded as an ADR by seed BD-044 (U33a) |

Kill rules in these subsections that have no `KILL-` identifier of their own: the frankenjax item (kill: either
condition of INT-FJAX-01 fails; N33c-21); the drone-reconstruction lane (kill: no published geometry by the M5
entry review; N33c-25); frankentui (the kill is KILL-TUI of §32; N33c-23 feeds it).

Gates of the chapter that this unit expects another unit to carry (each has a seed bead):

| Gate | Expected carrier (seed; unit) | This unit's supporting bead |
|---|---|---|
| INT-FSIM-01 | BD-006 (U33a) | N33c-33, N33c-34 (owner decisions taken from its record) |
| INT-FSIM-02, -03 | BD-032 (U33b) | N33c-42 depends on it |
| INT-FSIM-04 | BD-035 (U33b) | N33c-44 depends on it |
| INT-FSIM-05 | BD-033 (U33b) | N33-CX (CX-352) |
| INT-FSIM-06 | BD-034 (U33b) | N33-CX (CX-357), N33c-37 |
| INT-FSIM-07 | BD-035 (refusal half), BD-093 (witness migration) (U33b) | |
| INT-FSIM-08 | BD-079 (U33b) | |
| INT-FSIM-09 | BD-068 (U33b) | N33c-43 (probe of the same crate) |
| INT-FSIM-10 | BD-243 (U33b) | |
| INT-FSIM-11 | BD-020 (U21b) | |
| INT-FSIM-13 | BD-333 (U33b) | N33c-32 (the refusal for an absent group) |
| INT-FSIM-14 | BD-124 or a new bead of U33b (the gate is in §33.9; work package WP-54 lists it) | N33c-32 |
| INT-FSIM-15 | BD-401 (U33b) | N33c-32 |
| INT-FSIM-16 | BD-111 (U33b) | N33c-18 depends on the memo graph |
| INT-FSIM-17 | BD-413 (U33b) | N33c-31, N33c-32 (the bridge's label and refusals) |
| INT-ASUP-01 | BD-008 (U33b) | N33c-29, N33c-35 |
| INT-FSQL-01 | BD-009 (U33a) | |
| INT-FT-01 | BD-016 (U33a) | N33c-39 (the language version in its record) |
| INT-FT-02 | BD-028 (U33b) | |
| INT-CUDA-01 | BD-015 (U33a) | |
| INT-FSYM-01 | BD-019 (U33a) | |
| INT-FSYM-02 | BD-105 (U33a) | |
| INT-FLN-01, -02 | BD-380 (U33a) | |
| INT-FMCP-01 | BD-023 (U32a) | N33c-26 |
| INT-FNP-01, -02 | BD-036 (U33a) | N33-CX (CX-353) |

## (c) Counts

- Known answers: Appendix M lists 14 for §33 (KA-767 to KA-780); the registry bead N33-KA is U33a's. This unit's
  beads carry the three that belong to its subsections, with values: KA-767 (BD-001, N33c-30), KA-779 (N33c-03,
  N33c-05, N33c-06), KA-780 (BD-038, N33c-10 to N33c-13). The known answers of the two late gates have no KA
  number and are carried with their values: the four mixed-wrench momentum vectors, the sign agreement 0.6655
  with its lag table, the `serde_json` conflict, the enumeration ending at version 3.1 (GATE-WS-06); 31.96314202
  kg, ∓16.5 m, rates (12, −8) and (−12, 8), the arc bounds 0.55 m and 1.05 m, the minima 1 − 2⁻²² and 1 − 2⁻⁵⁶,
  the ray parameters 2 m and 3 m (GATE-WS-09).
- Counterexamples: 6 of 6 (CX-352 to CX-357) in N33-CX, plus 9 unnumbered shortcut cases proposed for numbering.
- Refusal codes: 3 of 3 (`FROB-REF-790`, `-791`, `-890`) in N33-REF.
- Invariants: 5 of 5. Open questions: 6 of 6 (one already decided). Architecture decisions: ADR-SUITE-02, plus
  two ADRs the gate INT-TOOLCHAIN-01 must write. Gates measured or decided by this unit: 11, and KILL-FSIM in
  part.

## (d) Needs from other slices

Things these beads depend on that another unit owns and for which no single seed key could be named (the `needs`
fields carry the full text; 72 entries in all, most of which also name a seed):

| Need | Wanted by | Likely owner |
|---|---|---|
| §33.5: the resolution-receipt schema and writer in `check-stack`, the drift classes, and the receipt covering the `worker/` lockfile | BD-001, BD-031, BD-038, N33c-12, N33c-30, N33c-33 | U33a (and seed BD-004, U34) |
| §33.6: the admission-record format of `DEPENDENCY_ADMISSION.md` and the gate rows of `registries/gates.toml` | BD-031, BD-038, N33c-17, N33c-20, N33c-30, N33c-40, N33c-48 | U33a (and seed BD-002) |
| §33.4: the full foundation ledger rows and their `confined_to` column | BD-001, N33c-09 | U33a |
| §33.3: airlock traits shaped as whole-job calls; the three adapter rules; the feature groups with the crates allowed to enable each | N33c-10, N33c-11, N33c-29, N33c-32 | U33a (seed BD-007) |
| §33.8: the compatibility train for pins; the register format of upstream work items | N33c-08, N33c-47 | U33b (and seed BD-037, U18a) |
| §33.9: the group gates and the evaluation-tier label of the multiphysics bridge; `fs-phs` and `fs-sos` behind their groups | N33c-31, N33c-32, N33c-43, N33c-45 | U33b |
| §29.15 and §29.4: the simulation-test oracles; the chunk publisher and partition | N33c-10, N33c-15, N33c-16 | U29 units |
| §29.16: the pin query for nodes in the evidence closure of a live claim | N33c-19 | U29 units |
| §21.16 and §21.26: the arithmetic-feature probes in the fingerprint; wide counts without 64-bit atomics | N33c-39 | U21c |
| §21.7: the closed table of exact rewrites, the node budget, manifest fields for the pass | N33c-21, N33c-22 | U21a |
| §23.25: the joint-law battery of rule P-12 and GATE-CORTEX-18 | N33c-37, N33c-40 | U23 units |
| §18.20: the arc-aware swept bounds and GATE-TRAC-12 | N33c-44 | U18 units |
| §19.22, §19.11, §19.16: the scaled-instance rules and GATE-HAB-16; the unknown-space label; a `3xx` number for a scan whose scale is not metric | N33c-25, N33c-46 | U19 units |
| §14.17: the reduced-model record, the profile `g1-reduced-30`, import parity against the 38-link source | N33c-41 | U14 units |
| §15.15: the signed motion witness in `actuator.check` and the port-sign refusal | N33c-43 | U15 units |
| §32.4: an explicit exit code for the `79x` refusals; §32.10 and §32.11: the job event stream and the daemon's slot model; §32.15: the viewer's replay page and picking | N33-REF, N33c-13, N33c-23, N33c-24, N33c-46 | U32 units |
| §34.14: the `serde_json` version and feature-set guard | N33c-38 | U34 |
| §36.9 and §36.14: referee-coverage arithmetic; the envelope classes | N33c-14, N33c-34 | U36 |

## (e) Contradictions, gaps and what was done about them

1. **The first receipt and the stack lock refer to each other.** The lock's rows carry the hash of the receipt,
   and the receipt carries `repo_rev`, the commit of this repository. BD-001 fixes a two-commit order (`repo_rev`
   is the commit before the hash is written) and requires `check-stack` to apply the same rule.
2. **`check-stack` does not exist when BD-001 runs** (BD-004 depends on BD-003, which depends on BD-001), yet
   BD-001 owes the first receipt. BD-001 writes it with a bootstrap script that `check-stack` must later
   reproduce byte for byte.
3. **"`cargo check` is green on the empty workspace".** Cargo rejects a virtual manifest with no members (to be
   confirmed on T). BD-001 makes the `xtask` package skeleton the one member; no `frob-*` crate is created.
4. **Profile overrides.** The receipt example writes `fsqlite*: opt-level 1`, which is a summary and not Cargo
   syntax. BD-001 copies the override tables verbatim from FrankenSim's manifest at the pin.
5. **The coupling counts 22 / 12 / 10 / 8** are reported without the population they were counted over. N33c-02
   measures three populations and records which, if any, reproduces them.
6. **The worker's toolchain.** §33.14 builds the worker "on FrankenSim's toolchain at the pinned commit"; §34.1
   says every workspace builds on T, with `worker/` on an older toolchain only when it is kept at an older pin.
   Read together (N33c-08, N33c-12): on T while the pins coincide; on the old pin's channel only under fallback
   (a) with a diverged pin, marked by `worker_rev` in the stack lock.
7. **`worker/` "re-targets an existing crate" and "no new crate is created",** but a package can belong to one
   workspace only. N33c-12 uses a manifest-only package with the binary's `path` pointing into `frob-fsim`, with
   a one-line `main.rs` as the stated fallback; the admission record says which form Cargo accepted.
8. **The protocol lists three messages** and also requires a receipt report from the worker, resume and
   cancellation. N33c-10 adds `hello`, `resume` and `cancel` as DESIGN, labelled as additions.
9. **"The same traits" in both modes against "never a per-tick call".** N33c-11 reads the trait surface as
   whole-job calls and records a need on the airlock's first version (seed BD-007).
10. **Which receipt a job names when the worker stays on an old pin.** A job that named the core's own receipt
    would always be refused. N33c-10, N33c-12 and N33c-13 define the job's `stack_receipt` as the referee
    receipt of the `FD-FSIM` row, and N33c-31 keeps such results out of same-stack claims.
11. **Exit codes for `FROB-REF-790` and `-791`.** The exit table names `70x`, `72x` and `73x` in the `7xx`
    family and not `79x`. N33-REF assigns exit 5 to both with the reason, and flags it for the owner of the exit
    table.
12. **INT-ASUP-02's milestone.** The gate register says M2; the dependency firewall and seed BD-096 use the lab
    runtime at M1. N33c-15 admits the harness at M1; N33c-17 records the gate at M2.
13. **INT-FSQL-02's milestone.** The gate register says M1; the seed that cites it (BD-220) is at M2. N33c-20
    records the gate at M1; the admission is repeated at M2 when episodes enter the store.
14. **franken_threed: "never a Cargo dependency" against "`f3d-*` crates are used with their `serde` feature
    off"** and a ray-transform probe "through the adapter". N33c-09 and N33c-46: never in a workspace that builds
    a `frob` binary; an admission-probe workspace may compile `f3d-math` with `serde` off.
15. **Six upstream work items of §33.18 have no identifier** in Appendix F (the register ends at FD-FSIM-33,
    FD-FTX-01 and FD-F3D-01). N33c-47 registers them under working names and leaves the numbers to the
    register's owner; the totals become 59 items (38 in frankensim, 2 in franken_threed).
16. **Refusals stated in words without a number:** a scan whose scale is not metric (N33c-25, working slug
    `scan-scale-not-metric`); a throughput-tier request through the multiphysics bridge (raised as
    `FROB-REF-790` with its own pointer, N33c-32); the report `unsupported` for a limit of a sibling
    implementation (N33c-42, a typed error and not a registered refusal code). N33-REF lists them for the owning
    registries.
17. **The planted sampler's radius uniform** is not specified by the plan (only the phase recurrence is).
    N33c-37 uses an independent addressed draw, so that the marginal is exactly Gaussian and only the joint law
    is defective.
18. **Reported numbers that were never run** (the 38-link mass and the 1.377 kg; the transformer rates; the two
    `fs-sos` returned values; the absence of 64-bit atomics). Each probe records what the pin does; a number that
    is not reproduced is replaced in the record and marked superseded.
19. **S3-FIFO parameters** (promotion threshold, reinsertion bound, the adaptation rule of the small share) are
    not in the plan, and the gate demands exact agreement with the sibling. N33c-18 takes them from the sibling
    at the pin and records them in the crate's contract; the published algorithm's values are the starting point.
20. **GATE-WS-06 and GATE-WS-09 are M0 gates that the M0 acceptance list of §43.3 does not name** (they were
    added after it). §40 governs; N33c-40 and N33c-48 say so for the exit reviews.
21. **No type sketch in Appendix I and no verb in Appendix J belongs to these subsections.** Probes are run
    through the conformance driver (`frob-conformance probe …`). One test-only verb is proposed to provoke
    `FROB-REF-790` at M0 (`conformance.probe --needs-group`), since no released M0 verb needs an off-by-default
    group; the verb registry's owner may prefer to exercise the refusal through the driver alone.
22. **The alias set of `frob_core::qty`** is left open by OPEN-SUITE-4 with only a default ("the quantities used
    by §13 to §18"). N33c-03 proposes a starter set of 36 aliases as DESIGN and N33c-04 decides it.
23. **Key overlap to watch when the lead merges units:** seed BD-096 (U33a) and N33c-15 both touch the lab
    runtime (N33c-15 is the admission and shared harness; BD-096 the M1 tests); seed BD-300 (U33a) and N33c-23,
    N33c-24 (the pilots that fund it); seed BD-220 (U33a) and N33c-18 to N33c-20 (the eviction policy and its
    gate); seed BD-398 (U33a) and N33c-25 (the M3 half of INT-FDGR-01); seed BD-023 (U32a) and N33c-26. In each
    pair the seed should depend on the bead of this unit.
