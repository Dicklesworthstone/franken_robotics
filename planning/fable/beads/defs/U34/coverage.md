# Coverage of drafting unit U34: §34 Workspace, crates, formats

34 bead files: 6 seed beads (BD-003, BD-004, BD-005, BD-141, BD-145, BD-410), 25 new beads (N34-01 to N34-25) and
3 registry beads (N34-KA, N34-CX, N34-REF). The chapter is Solid throughout; it defines no Frontier or Moonshot
mechanism, so there is no pilot bead and no kill-gate bead. It owns no verb of the verb registry.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §34.1 | Repository layout | N34-01 (workspaces, lockfiles, toolchain files, the apps workspace, directory rules); N34-13 (toolchain-file and lockfile findings); N34-16 (inventory) |
| §34.2 | Crate map | N34-02 (the 43 rows with layer, rank, tier, responsibility, cap; totals; non-Rust caps; conditional `frob-units`); N34-08 (line counts against caps, the 115% rule); BD-005 (`frob-core` itself) |
| §34.3 | Layer and dependency rules | BD-003 (L-1, L-2, L-3, L-4, L-5, L-9, L-10, L-11; the honesty note on the closure; the rank counterexample); N34-03 (L-12; asupersync; frankentui; airlock groups; all-features resolution); N34-05 (L-6, L-7); N34-06 (the L-10 traits and register); BD-004 (L-8); N34-25 (L-9 in depth) |
| §34.4 | Unsafe code | N34-07 |
| §34.5 | Contracts | N34-08 (`CONTRACT.md`, `tests/conformance.rs`, `contract.toml`, the dependency-target rule); N34-10 (safe mode and the convention records) |
| §34.6 | The six checks | BD-004 (the table, exactly six, `check-generated`); BD-003 (`check-layers`, `check-deps`); N34-03, N34-04, N34-25 (`check-deps` rules); N34-07 (`check-unsafe`); N34-08, N34-09, N34-10 (`check-contracts`); N34-13 (`check-stack`); N34-14 (end to end) |
| §34.7 | Working rules inherited from the machine and the suite | N34-15 (all twelve rules with enforcement points; the envelope guard); BD-145 (the compute request under the envelope question); N34-13 (the stack-check rule) |
| §34.8 | What ships with the first commit | N34-16 (the inventory checked end to end); produced by BD-001 and BD-002 of other units |
| §34.9 | The format registry | N34-19 (ADR-WS-03, the thirteen rows, registry rules 1 to 6, types); BD-141 (seven formats at M1, producers and consumers); BD-410 (the remaining six, complete at M5); N34-22 (the round-trip harness); N34-23 (end to end); N34-24 and N34-REF (the three refusals); N34-09 (owner, tests, single parser) |
| §34.10 | The rule for creating a new crate | N34-02 (ADR-WS-01, the three reasons, additions, merges, deletion of `frob-lifecycle`); BD-003 (the proof-boundary edge table) |
| §34.11 | The independent auditor | N34-25 (ADR-WS-02's six parts as every-commit rules; the gate record); BD-003 (L-9 manifest rules); the crate and its script are BD-026 and N25b-05 of other units |
| §34.12 | Suite conventions this project adopts | N34-11 (artifact-graph identifiers); N34-08 (contracts as code); N34-09 and N34-12 (demo files); N34-10 (shadow-run; the controller gate; the table of all six); BD-003 and N34-06 (two planes) |
| §34.13 | Invariants, gates and open questions | Invariants: BD-003 (INV-WS-01), N34-25 (INV-WS-02), N34-09 (INV-WS-03), N34-03 (INV-WS-04), BD-004 (INV-WS-05). Gates: table (b). Open questions: N34-02 and BD-003 (OPEN-WS-1), N34-02 (OPEN-WS-2, OPEN-WS-5), N34-25 (OPEN-WS-3), N34-19 and BD-141 (OPEN-WS-4) |
| §34.14 | The canonical reader and writer, and JSON features that unify across the build (late subsection) | N34-17 (the reader); N34-18 (the writer, `CanonicalBytes`); N34-20 (admission order on both surfaces); N34-04 (the guard); N34-21 (the corpus, the four-way build, the gate) |

## (b) Gates, and the beads that measure or decide them

| Gate | Claim | Milestone | Beads |
|---|---|---|---|
| GATE-WS-01 | Known answer | M0 | BD-004 (statement, ownership rule, in-crate matrix); N34-14 (measures and records on the real tree); plant owners BD-003, N34-03, N34-07, N34-08, N34-13 |
| GATE-WS-02 | Known answer | M1, then at each new format; complete at M5 | N34-22 (the four clauses and the control); BD-141 (seven formats); BD-410 (thirteen, records the pass); N34-23 (user side, gate line) |
| GATE-WS-03 | Known answer | M1 | N34-25 (clause (a) every commit; assembles the four-clause record); measured also by BD-026, N25b-05 and N12-34 of other units |
| GATE-WS-04 | Performance | M0 | BD-004 (budget shares); N34-14 (measures and records) |
| GATE-WS-05 | Known answer | M0 | N34-21 (measures and records); N34-17, N34-18, N34-20, N34-04 (one clause each) |

No kill gate and no admission gate is defined in §34. Other identifiers of the chapter: ADR-WS-01 (N34-02),
ADR-WS-02 (N34-25), ADR-WS-03 (N34-19); INV-WS-01 to -05 (row §34.13 above); OPEN-WS-1 to -5 (none blocks work;
each sits with its fail-closed default in the Considerations of the beads named above). Risk rows carried:
RISK-03 (N34-03), RISK-10 (N34-15, BD-145), RISK-18 (N34-02, N34-08, BD-003, BD-004), RISK-21 (BD-005), RISK-23
(N34-13), RISK-27 (BD-145), RISK-33 (N34-02), RISK-36 (N34-25), and the two build rows of the late risk register
(N34-04, N34-18).

## (c) Counts

- Known answers carried: 4 (KA-781 to KA-784) in N34-KA. KA-767 of the suite-consumption chapter is realized by
  N34-13 and cited there.
- Counterexamples carried: 6 (CX-358 to CX-363) in N34-CX, seeding at least ten negative-control records.
- Refusal codes carried: 3 (FROB-REF-891, -892, -893) in N34-REF.

## (d) Needs from other slices

| Need | Where it arises | Owner (as far as known) |
|---|---|---|
| The stack lock gains two tables, `[json_library]` (version, unified features, allowed crates) and `[auditor]` (the checker's stable channel) | N34-04, N34-25, N34-13 | BD-001 (U33c) |
| `schemas/contract.schema.json` gains `plane`, `[api.float_ok]`, `[[replaces]]`, `[[controller]]`; format rows gain a number rule, a view kind (`lossless` or `export`) and a declared writer; `demo.yaml` gains `expect_exit`, `expect_refusal`, `compare`, `tier` | N34-05, N34-08, N34-10, N34-19, BD-141, N34-12 | BD-002 (U25a) |
| Verb-registry parameter kinds (`artifact-input`, `artifact-ref`, `artifact-identity`, `foreign-file`) and a `released` mark per verb | N34-24, N34-09 | The verb-registry beads (BD-022, BD-137) |
| A read-only count of job and run rows in the ledger at M0 (a ledger block in `doctor`) | N34-21, N34-20 | BD-022, BD-024 |
| Raw bytes of each MCP frame before the library parses it; an upstream work item on the MCP library if its transports cannot be fronted | N34-20 | BD-023; a new upstream item if needed |
| The first backend trait placed in a core crate at or below `frob-dyn`, and row 1 of the inversion register corrected to name it | N34-06 | BD-014 |
| The known-answer, counterexample, negative-control and pilot registries of the conformance crate, and test targets per tier that select a gate by identifier | N34-KA, N34-CX, N34-14, N34-22, BD-145 | BD-025, BD-146 |
| The shared script helper sources the envelope guard | N34-15 | STD-E2E |
| The registration form for method experiments (six parts) and its ledger record | BD-145 | The Gauntlet unit |
| The owner's rulings on OPEN-SPARK-2 (envelope classes) and OPEN-PROG-4 (rank-agreement compute) | BD-145, N34-15 | Owner-decision beads of the delivery unit |
| The release manifest and its cells in `check-contracts` (GATE-WS-08); the gates GATE-WS-06 and GATE-WS-07, which read the output of `check-deps` | N34-09, N34-03, N34-04 | The delivery, suite and risk units |
| Each format owner's `Subject` generator and the test names `fmt_<ext>_roundtrip`, `_refusals`, `_view_roundtrip`, fuzz target `fmt_<ext>_read` | N34-22, N34-09, BD-141, BD-410 | The format owners' beads |
| The doorway scan that BD-021 expects "in the six checks" is placed in `check-unsafe` (N34-07), not in BD-004 | N34-07 | For the lead to re-point |

## (e) Contradictions and gaps found in the plan, and what was done

1. **KA-784 lists six plants; GATE-WS-01 lists seven** (it adds a stale stack lock). The gate register governs:
   the tests plant seven; N34-KA records the difference without rewriting the row.
2. **L-6 and L-7 are assigned to none of the six checks**, although §34.6 says everything is assigned. N34-05
   enforces them with the workspace lint table and each crate's conformance test; no seventh check is added.
3. **§34.14 names two refusals and mints no code** (duplicate member name after escape decoding; identifier not
   printable); the brief lists three codes. They are reported under existing codes (FROB-REF-802 for arguments;
   the owner's duplicate-field code, FROB-REF-103 for a design, for files) with the fault in `observed`
   (N34-20, N34-REF). The registry's owner should decide whether to mint codes.
4. **Two codes for an unsupported version or unknown field**: the design chapter's FROB-REF-125 and -103, and the
   registry's FROB-REF-891. Rule adopted: an owner with its own code uses it; every other owner uses 891
   (N34-19, N34-REF). This matches the format beads already drafted by other units.
5. **franken_threed and frankenjax**: "never Cargo dependencies" (§33.13) against "used with their serde feature
   off" (§33.17) and "in the derivation annex only" (§33.15). The fail-closed reading is implemented in N34-03:
   absent from every graph until a stack-lock row with an admission gate lists the workspace.
6. **No machine-readable home is named for the crate map and caps.** N34-02 makes it a compiled table in
   `xtask`, changed only by a commit citing an ADR.
7. **Shadow-run and the controller gate are "assigned to one of the six checks"** but the plan does not say
   which. N34-10 assigns `check-contracts`, through records in `contract.toml`.
8. **The claim identifier's prefix is spelled two ways** (`clm:` in the gate-registry example, `c:` in the
   bead-log example). N34-11 fixes one spelling, `clm:`, and the parser refuses the other.
9. **"crates/frob-*: 38 crates"** counts `xtask`, which is not named `frob-*`. It is placed at `crates/xtask/`.
10. **`xtask` may link no parser** (rule L-5), yet must read TOML, JSON and YAML. Strict subset readers are
    specified: TOML and JSON in `frob-core` (BD-005, N34-17), a YAML subset in `xtask` (N34-09). FACT: the
    sibling's own `xtask` links no parser crate.
11. **The cap of `xtask` (3 thousand lines with tests)** looks tight for six checks, generators and plant
    tests. Not resolved; flagged as RISK-18 in BD-003, BD-004 and N34-02.
12. **GATE-WS-02's negative control cannot fail a self-consistent writer** (`write∘read∘write = write` holds
    with unsorted keys). N34-22 adds an independent reference canonicalizer so that the stated control fails.
13. **Clause 2 of GATE-WS-02 ("where a view exists")** cannot apply to one-way renderings (array files,
    summaries, STEP, bills of materials). BD-141 distinguishes `lossless` views, which get the clause, from
    `export` views, which are tested for having no identity.
14. **Clause 3 (single-byte mutations)** is provable only on the strict identity-bearing read path; on a
    tolerant path an equivalent escape spelling is accepted with an unchanged identity. N34-22 defines it so.
15. **`frob-cert` cannot implement a `frob-core` trait** (it has no dependency). The bundle format is tested
    through an adapter in the conformance crate (BD-141).
16. **The MCP library parses a frame before any handler runs**, so a duplicate member is already gone. N34-20
    specifies a byte-level guard in front of the transports as a HYPOTHESIS with its test and its fallback.
17. **10⁷ fuzz executions per reader** with no fuzz workspace in the layout and no third-party engine allowed
    in the core. N34-22 specifies a seeded in-house mutational fuzzer; coverage-guided fuzzing is left to the
    importer-fuzzing bead.
18. **`job.list` is an M2 verb**, but GATE-WS-05 (M0) must show that no job identifier exists after a refusal.
    N34-21 needs a ledger row count from `doctor` at M0.
19. **Plants that change a manifest also change `Cargo.lock`**, so "caught by no other check" fails unless the
    plant carries the normalization a real change would (lockfile regenerated, receipt rewritten). Defined in
    BD-004 and N34-14. Likewise the stack check records duplicate packages without failing, so that a second
    asupersync package has one owner (`check-deps`), although another seed says the asupersync admission is
    "wired into check-stack".
20. **BD-145 was mapped to this unit by keyword** (the compute envelope of §34.7). Its content comes from the
    dynamics, learning and studio chapters; it is written in full here. No refusal code exists for "a
    registration edited after the first result"; the conflict code FROB-REF-809 is used.
21. **The brief marks GATE-WS-05 as absent from §40.** It is in §40's table of late subsections, at M0.
22. **BD-002 states a need on BD-003 and BD-004.** To avoid a cycle, BD-004 and N34-08 do not depend on BD-002;
    it is listed as related.
23. **Seed BD-004 names four checks.** Three are split into their own beads (N34-07, N34-08, N34-13), on which
    BD-004 depends; BD-004 keeps `check-generated`, the six-subcommand test and the two gates.
24. **L-10 places the device-backend trait in `frob-dyn`**, while the M0 backend trait is written beside the
    kernel harness. N34-06 registers the row and leaves its correction to the bead that writes the trait.
