# Coverage of drafting unit U38a

Chapter §38 "Gauntlet: the verification program", preamble and §38.1 to §38.9; the reserved keys `STD-E2E` and
`STD-TEST`; seed beads BD-041, BD-417, BD-418, BD-421, BD-432. 43 bead files: 5 seed, 2 reserved, 36 new
(`N38a-01` to `N38a-36`).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §38 preamble | What the Gauntlet is, its two duties, what it owns | N38a-01 (ownership, duties); STD-E2E, STD-TEST (the standards every bead follows) |
| §38.1 | What it owns, and decisions | N38a-01 (ownership table, ADR-GAUNT-01 to -06 as checks, scope cap of 24,000 lines, "nothing depends on it") |
| §38.2 | Six tiers | N38a-02 (tiers, sites, selection, receipts, first table); N38a-03 (second table: 9 subjects × 6 tiers, 54 cells); N38a-35 (tier G5 in full); STD-TEST (determinism lanes) |
| §38.3 | Oracles and pins | N38a-04 (the thirteen oracle rows, records, rules 1, 2, 5, 6; INV-GAUNT-01); N38a-05 (rules 3 and 4; INV-GAUNT-03; oracle hygiene); N38a-06 (the development oracle, rule 7; INV-GAUNT-04; feeds KILL-DEVORACLE) |
| §38.4 | The form of a method experiment | N38a-07 (the six parts, admission, three further rules, refusals, the worked reading); N38a-08 (ceiling, paired lower bound, potency, A/A; PO-GAUNT-01); N38a-36 (the design corpus as potency control) |
| §38.5 | Paper-scale pilots before funding | N38a-09 (register, registration before run, funding rule); N38a-10 (the twenty plan-time pilots and the three limits); BD-041 (docking, two gains and three); N38a-11 (fixed-n rule, draft ten-fold rule, wheel loop, rotor and cadence, both conditioning surveys); N38a-06 (pilots on the development oracle). Rows re-run by other units: stance and stance falsifier (BD-042), probe, pad family, sensor aliasing, pusher and audit, labels, reuse, bus, sill (BD-043), boundary screen (BD-030) |
| §38.6 | Known answers | N38a-12 (registry, row schema, five rules, derivation files, printout, the carried list of the earlier design, plan-time counts); N38a-13 (the G1 run and the accounting of GATE-GAUNT-02); STD-TEST (loader and `assert_ka`) |
| §38.7 | Metamorphic relations | N38a-14 (registry, runner, mutation harness, PO-GAUNT-02); N38a-15 (the eleven carried relations, MR-C01 to MR-C11); N38a-16 (MR-01 to MR-14); N38a-17 (MR-15 to MR-28); N38a-18 (MR-29 to MR-42); N38a-19 (end-to-end measurement) |
| §38.8 | The standard battery and the hard-world set | N38a-20 (standard battery, identity, freeze and change events); N38a-21 (hard-world set under three refusals; regression label); N38a-22 (the two fixtures' analytic layers on every relevant change); N38a-36 (design corpus, labels frozen first) |
| §38.9 | Chaos | N38a-23 (framework: schedules, oracle, leak audit, coverage, minimization, the release TARGET); N38a-24 (cancellation, device faults, transaction faults, capacity); N38a-25 (job protocol, chunk protocol, daemon and ledger, worker fallback); N38a-26 (malformed input, confirmation firewall); N38a-27 (end-to-end measurement) |

Seed beads outside the chapter's text (the seed index maps them to §38 by keyword; their content is in other
chapters, which were read and carried):

| Seed | Content drawn from | Child beads |
|---|---|---|
| BD-041 | §38.5, §46 step 7, §20.16, §25.15, C-19 | none |
| BD-417 Epic: soft soil at scale | §18.10, §18.16, §18.17, §43.9 | N38a-28 (pilot), N38a-29 (mechanism) |
| BD-418 Epic: tactile sensing and compliant pads | §14.10, §18.17, §20.5, §20.7 | N38a-30 (pilot), N38a-31 (mechanism) |
| BD-421 Epic: moving people as kinematic actors | §19.3, §19.7, Appendix G | N38a-32 (pilot), N38a-33 (mechanism) |
| BD-432 Epic: neutral adjudication | §39.1, §39.3, §39.9, §39.17 to §39.19 | N38a-34 (dry-run pilot) |

## (b) Gates, kill gates and admission gates

§38.1 to §38.9 define no gate of their own: the chapter's gates are tabulated in §38.15, which unit U38b owns
(definition, decision and status accounting). The beads of this unit measure them as follows.

| Gate | What this unit's beads do | Beads |
|---|---|---|
| GATE-GAUNT-01 (every passing gate has a caught control) | Supply the rule at library level (`GateReport`), the lint, the tier runner's control run | STD-TEST, N38a-02, N38a-01, N38a-21 |
| GATE-GAUNT-02 (known answers due pass; obligations counted) | Registry, run and account | N38a-12, N38a-13, N38a-22 |
| GATE-GAUNT-03 (relations hold on 10⁴ scenarios; mutations break them) | Registry, harness, 53 records, measurement | N38a-14 to N38a-19 |
| GATE-GAUNT-05 (oracle hygiene) | Fixture and pin checks, typed parity, production-graph check | N38a-04, N38a-05 |
| GATE-GAUNT-06 (chaos) | Framework, ten families, measurement (kernel part at M2 by N38a-24's script; whole at M3) | N38a-23 to N38a-27 |
| GATE-GAUNT-07 (determinism) | Lanes, tier G5 runner, MR-01 on 10⁴ worlds | STD-TEST, N38a-35, N38a-16 |
| GATE-GAUNT-08 (pilots) | Register and funding rule; plan-time ledger; re-runs | N38a-09, N38a-10, BD-041, N38a-11, N38a-06, N38a-28, N38a-30, N38a-32, N38a-34 |
| GATE-GAUNT-09 (experiment form) | Register, admission, statistics, potency | N38a-07, N38a-08, N38a-36 |
| GATE-GAUNT-10 (status accounting) | Supporting evidence only (the harness log and gate report are views; ADR checks on generated tables); the gate itself is measured by U38b's beads | N38a-01 (STD-E2E supports it and claims no gate) |
| KILL-DEVORACLE | Produces the count `changed_of_first_five` the decision reads | N38a-06 |
| GATE-ARENA-07, GATE-FID-01 | Battery identity and column sums; the battery the fidelity gate reads | N38a-20 |
| GATE-ARENA-04 | Fixture analytic rows per change, with both controls | N38a-22 |
| GATE-SAFE-07, GATE-QUAL-02 | Drives the importers' and firewall's fault families | N38a-26 |
| GATE-TRAC-08, KILL-SOIL | Rut leak at batch scale; the M4 gate's leak clause extended to devices | BD-417, N38a-28, N38a-29 |
| KILL-DRESSING | Re-read on the actor mechanism | BD-421, N38a-32, N38a-33 |
| GATE-TENFOLD-05, KILL-TENFOLD-ADJ, KILL-TENFOLD-CERT, KILL-TENFOLD-LEAK | Adjudication contract and dry run | BD-432, N38a-34 |
| KILL-RUT, KILL-TACTILE (proposed; not in the gate register) | Wording written in the epics; registration is a `needs` | BD-417, BD-418 |

## (c) Counts

- Known answers: the unit owns no chapter registry (Appendix M has no section for §38; the rows of §38.17 and
  §38.18 are U38b's). 20 KA rows are cited with their values in test plans: KA-023 to KA-026, KA-108 to KA-111,
  KA-415, KA-416, KA-498 to KA-502, KA-523, KA-636 to KA-638, KA-807. The suite's plan-time counts (835 rows;
  133 tolerances to set; 325 controls not stated; tiers 135, 591, 16, 36, 19, 38) are carried as assertions in
  N38a-12.
- Counterexamples: no chapter registry owned; 6 CX entries cited with their cases: CX-208, CX-213, CX-214,
  CX-262, CX-275, CX-282.
- Refusal codes: §38.1 to §38.9 define none; 13 existing codes are used with meaning and repair: FROB-REF-243,
  -532, -552, -560, -561, -562, -567, -734, -801, -810, -861, -863, -866.
- Metamorphic relations: 53 carried in full (11 carried from the earlier design, 42 added), each with source,
  sites, comparison and mutation.
- Plan-time pilots: 20 rows carried with expected values; 8 re-run by this unit (rows 6, 7, 13, 14, 17 to 20).
- Architecture decisions ADR-GAUNT-01 to -06; invariants INV-GAUNT-01, -03, -04, -07 and proof obligations
  PO-GAUNT-01, -02 are carried where their mechanism lives (their table, §38.14, is U38b's).

## (d) Needs from other slices

- §38.11: the negative-control registry `controls.toml`, its record format and its seeding from the
  counterexample catalogue (U38b, with BD-025 and BD-146).
- §38.13: the recorder for `registries/gates.toml` (U38b).
- §38.15 and §38.16: the decision beads for KILL-DEVORACLE and OPEN-GAUNT-2, -3, -4, -5, -6, -7, -8 (U38b).
- §38.12: the self-tests of the certifiers, including the determinism classifier (U38b).
- §24.9: the battery manifest format; §22: the gait atlas; Appendix G: the hazard registry.
- §39: the rival audit that freezes `comparators.toml`; the sealed suite, decision rule and arms of the
  tournament; OPEN-PROG-2.
- §40: registration of the proposed kill gates KILL-RUT and KILL-TACTILE.
- §34.6: `check-deps` and `check-generated` learning the Gauntlet's generated files (BD-003, BD-004).
- §35.11, §24.14, §26.12: refusal codes defined by their owning chapters.
- Each relation's transform code and each tier-matrix cell's test, in the owning subsystem.

## (e) Contradictions and things that could not be turned into work as written

1. **§38.8 names "BD-104" as the bead that freezes the standard battery.** Seed BD-104 is "A-13 self-collision
   and clearance" (M1). The seed that freezes the battery is BD-212. N38a-20 says so and relates to BD-212.
2. **Registry paths.** The brief names `registries/known_answers.toml` and `registries/negative_controls.toml`;
   §38.1 places them at `frob-conformance/registry/known_answers.toml` and `controls.toml`. The plan's paths are
   canonical; the brief's names are logical aliases resolved by the loader; a second physical copy is a lint
   failure (STD-TEST).
3. **`schemas/` is "generated, never edited"** (§34.1), while the brief asks for `schemas/e2e-log.schema.json`.
   The file is generated by the harness tool and compared by its self-test; the generated-file check must be
   told of that generator (STD-E2E, Considerations).
4. **§34.1's layout has no `scripts/` directory**; the charter requires `scripts/e2e/`. The charter is followed.
5. **ADR-GAUNT-01 forbids a Gauntlet verb or `xtask` subcommand**, yet the harness needs tools. They are a
   binary target (`frob-e2e`) and test targets of `frob-conformance`; no verb and no seventh check.
6. **The eleven carried relations of §38.7 have no identifiers and no mutations**, although every relation must
   be registered with one. Identifiers MR-C01 to MR-C11 and one mutation each are assigned as DESIGN (N38a-15).
7. **§38.7 says a relation runs "on every tier it names" and names none.** Sites are assigned per relation as
   DESIGN, with the rule that `bits` relations are claimed only where class D2 holds.
8. **GATE-GAUNT-08 asks timing ratios to fall "inside its recorded band"; no band is recorded.** The band is
   defined as the minimum to maximum of the per-trial ratios in the original receipt (N38a-10), as BD-043 does.
9. **The reproduction archive on disk differs from the recorded digest** (275,178 bytes, SHA-256 `9620da42…`
   against the recorded 241,568 bytes, `0326e89d…`). Every re-run records the digest it used.
10. **Two plan-time pilots have no retained script** (the conditioning surveys), and the draft ten-fold rule's
    script was not found by name. N38a-11 re-creates them from their stated protocols and records `re-created`.
11. **The epics BD-417 and BD-418 have no kill gate in the register.** Their wording is proposed in the epics
    and their registration is a `needs`.
12. **Seed mapping.** The four epics were mapped to §38 by keyword; their substance is in §18, §19, §20 and §39.
    They were written from those chapters. Their parents are WP-74 and WP-77.
13. **`STD-E2E` has no blocking dependency, and `STD-TEST` depends only on `STD-E2E`,** on purpose: nearly every
    bead depends on them, so an edge to a seed bead (BD-001, BD-005, BD-022, BD-025) could close a cycle through
    that bead's own test plan. Those beads are listed as `related`. The lead may add edges after a cycle check.
14. **Overlap with U38b's seeds** BD-025, BD-146, BD-223 and BD-318, whose titles name tiers, known answers,
    relations, batteries and chaos. This unit's beads own the registries, schemas, runners and measurements of
    §38.2 to §38.9; those seeds wire content per milestone. N38a-01 to N38a-05, N38a-09 and N38a-12 depend on
    BD-025 for the crate's existence.
15. **A/A threshold.** §38.4 says an A/A campaign that clears 1.02 means a broken harness; the performance
    program's gate allows up to 9 of 100 A/A campaigns to clear it. N38a-08 applies the strict form to the
    Gauntlet's own fixture and cites the frequency form as the performance program's.
16. **"At least 300 worlds" for the decision-level test** (§35.4) is a round figure; the exact smallest n with a
    zero-discordance bound of at most 1% is 299 (N38a-35 pins both).
