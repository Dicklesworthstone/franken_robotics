---
key: "N41-OPEN6"
title: "registries/open.toml, part 6: Atelier, Chronicle, Ground, Foundry and Tether questions (27 rows, one decided)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "atelier", "chronicle", "ground", "foundry", "tether", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-03"]
needs: []
related: ["N41-OPEN7"]
gates: []
plan_refs: ["§42.3", "§42.2", "§28.24", "§29.21", "§30.23", "§31.20", "§32.22"]
---
## Background

Sixth part of the open-question register (rule and schema in N41-03; how to read a row in N41-OPEN1). The
subsystems:

- **Atelier**: the design studio (analyses, comparison, optimization, the ladder of evaluation rungs,
  revalidation after edits, decision cores);
- **Chronicle**: the ledger, episode records in chunks, replay, retention, the artifact graph;
- **Ground**: reality as an import lane (dataset admission, calibration, frozen predictions, bands inside a
  calibrated cell, `reality: no-claim` everywhere else);
- **Foundry**: from a qualified design to a build packet (loads, sizing, manufacturability, tolerances);
- **Tether**: the agent surface (one verb registry generating the CLI and the MCP tools, the job engine, the
  daemon).

The last column names the bead that decides each question. For four owner questions two beads exist: the chapter
unit's decision bead, which carries the engineering evidence and the wiring, and this unit's owner packet, which
carries the question as it is put to the owner with its default and the cost of the default: OPEN-CHRON-4 (N41-11
and N29b-12), OPEN-GROUND-2 (N41-10 and N30b-26), OPEN-FOUNDRY-2 (N41-13 and N31b-25) and OPEN-TETHER-5 (N41-12
and N32a-34). One of the two writes the question's ADR and moves its row: N29b-12 (ADR-PROG-11), N30b-26
(ADR-PROG-07), N41-13 (ADR-PROG-14) and N41-12 (ADR-PROG-12). The record has one `decision_bead` field (N41-03):
it names that bead, written first in the last column, and the record's `note` names the other ("owner packet
N41-11", "engineering N31b-25"), so the printout shows both. The recorded decision OPEN-TETHER-1 is in N41-14.

## Scope

In: the twenty-seven rows below as records of `registries/open.toml`. Out: the schema and loader (N41-03); the
decisions.

## Technical approach

| ID | Question | Fail-closed default | Owner | Blocks, or decided by | Deadline | Decided in |
|---|---|---|---|---|---|---|
| OPEN-ATEL-1 | The loss constants of the promotion decision core and the exchange rate between GPU-seconds and real experiment cost in the experiment-suggestion core | The fixed 30-pair rule and the fixed experiment order | Atelier lead | GATE-ATEL-13 | M3 | N28c-26 |
| OPEN-ATEL-2 | The definition of a stratum for rank agreement (the mode thresholds are settled at 0.7 and 0.5) | One stratum per morphology class; an unmeasured stratum is `uninformative` | Atelier lead | GATE-ATEL-07 | M3 | N28c-27 |
| OPEN-ATEL-3 | Whether the siblings `fs-recompute`, `fs-voi` and `fs-race` pass admission through the airlock on the toolchain T | The node record, the closed-form value and the race are implemented locally against the same known answers | Suite lead | GATE-ATEL-12, GATE-ATEL-15 | M1 | N28c-28 |
| OPEN-ATEL-4 | Whether `result.revalidate` and `mission.revalidate` stay two verbs over one engine or become one verb | Two verbs, one engine | Tether lead | The verb registry freeze at M3 | M3 | N28c-29 |
| OPEN-ATEL-5 | The planted winner for morphology classes that have no earlier specialist result | The control is reported `not-run`, and finalists are described as "k survivors" | Arena lead | GATE-ATEL-09 for those classes | M4 | N28c-30 |
| OPEN-ATEL-6 | Whether a fine-tune fraction of 5% is the right second rung, or two such rungs are needed | One rung at 5% | Cortex lead | The calibration experiment of the learning chapter | M4 | N28c-31 |
| OPEN-CHRON-1 | Column compression without a third-party codec | Uncompressed quantized columns | Chronicle lead | Nothing before M2 | M2 | N29b-17 |
| OPEN-CHRON-2 | The chunk length L and the depth of the device ring | 256 control ticks per chunk and 2 s of ring | Chronicle lead | GATE-CHRON-04 | M2 | N29b-18 |
| OPEN-CHRON-3 | Whether identifier types are imported from the sibling `franken_kernel` under the single-version rule for asupersync, or defined locally with the same layout | Defined locally in `frob-core` | Suite lead | GATE-CHRON-10 | M1 | N29b-09 (N34-11 and N33c-27 carry the default) |
| OPEN-CHRON-4 | Retention windows for failure captures and the default store budget on each machine | 30 days and 64 GB | Owner | GATE-CHRON-12 | M2 | N29b-12; N41-11 (owner packet) |
| OPEN-CHRON-5 | Lease granularity: one generation per run, or one per shard with a versioned world-to-shard map | One per run | Tether lead | GATE-CHRON-04 on two devices | M2 | N29b-19 |
| OPEN-GROUND-1 | Which public paired simulation-and-reality dataset, if any, has enough robots per class to test exchangeability across units; none was identified | The second-simulator stand-in, labelled `synthetic` | Ground lead | GATE-GROUND-10 only | M5 | N30b-25; the owner half is OPEN-GAUNT-1 (N41-10, N38b-26) |
| OPEN-GROUND-2 | Whether any bench hardware (the six optional bench packages) is acquired | None; Reality stays `no-claim` | Project lead; a question for the owner | Nothing; sourcing may start at M0 | M5 | N30b-26; N41-10 (owner packet) |
| OPEN-GROUND-3 | Calibration expiry by age and operating hours | 90 days or 200 operating hours, whichever comes first, and always on part replacement | Ground lead | GATE-GROUND-10 | M5 | N30b-27 |
| OPEN-GROUND-4 | The exchange rate between operator time and GPU-seconds in the experiment table | Real episodes are recommended, never scheduled, and no rate is assumed | Project lead | Nothing | none | considerations of N30b-09 and N30b-08 (the experiment table it concerns is BD-351) |
| OPEN-GROUND-5 | Who is the authenticated authority that could promote a sibling's estimated candidate | Nobody; every Ground output stays `Estimated` | Lemma lead | Any Ground output above `Estimated` | M5 | N30b-28 |
| OPEN-FOUNDRY-1 | Whether the sibling `fs-mesh` produces a conforming mesh of an arbitrary optimized surface | If it does not, the independent check is a second cut-cell solve at another resolution and offset, labelled `check: same-family` | Foundry lead | GATE-FOUNDRY-04 | M5 | N31b-24 |
| OPEN-FOUNDRY-2 | Which parts catalogue and price data, under what licence | Card references and no prices | Project lead; asked of the owner at M4, one milestone before its M5 deadline | Nothing | M5 | N41-13 (owner packet); N31b-25 |
| OPEN-FOUNDRY-3 | Which current-capacity table governs conductor sizing | Current density against a limit declared on the cable card, with no capacity claim | Foundry lead | Any conductor capacity claim | M5 | N31b-26 |
| OPEN-FOUNDRY-4 | Stress-life data for printed material by process and orientation | `fatigue: no-claim` for printed parts | Foundry lead | The fatigue leg of GATE-FOUNDRY-05, for printed fixtures only | M5 | N31b-27 |
| OPEN-FOUNDRY-5 | Wear, insulation and cell-fade coefficients with provenance | Generic cards marked `assumed`, which `--strict-data` refuses | Sinew lead | GATE-FOUNDRY-12 | M6 | N31b-28 |
| OPEN-FOUNDRY-6 | The time budget per bit-vector obligation | 2 s (TARGET); a timeout is undecided and is therefore refused | Lemma lead | GATE-FOUNDRY-11 | M8 | N31b-29 |
| OPEN-TETHER-1 | Decided with OPEN-SUITE-2 by the owner on 2026-11-22 and kept as a recorded decision. The question was whether `frob-mcp` can link handler crates in process | Decision: `frob-mcp` is a member of the core graph and runs verbs in process, once the upstream work item FD-FMCP-01 has brought fastmcp_rust onto T; KILL-MCP's fallbacks if the admission INT-FMCP-01 cannot pass | Tether lead | Decided; confirmed by INT-FMCP-01 | M0 (confirmation) | N41-14; BD-023 |
| OPEN-TETHER-2 | Whether the agent harnesses in use speak one of the two protocol eras the library serves, and act on catalog-changed notifications | Static clusters by launch flag, plus `verb_invoke` | Tether lead | GATE-TETHER-12 | M3 | N32a-12 |
| OPEN-TETHER-3 | The default and maximum of `sync_wait_ms`, against measured agent turn time and measured verb latency | 5,000 and 30,000 | Tether lead | GATE-TETHER-13 | M4 | N32a-33 |
| OPEN-TETHER-4 | The slot reserve, per-environment job cap, quantum and ageing period of the daemon's dispatch, from the M2 queue trace | 5%, 64, 5 s, 600 s | Daemon lead | GATE-TETHER-10 | M3 | N32a-22 |
| OPEN-TETHER-5 | Whether the peer link between the two machines earns its upkeep | Standalone machines | Owner | M4 | M4 | N41-12 (owner packet); N32a-34 |

Status at entry: twenty-six `open`, one `decided` (OPEN-TETHER-1). One default is confirmed by the gate register and
is open only to new evidence: the single revalidation engine of OPEN-ATEL-4. What the owner defaults cost:
OPEN-CHRON-4, none beyond the default itself; OPEN-GROUND-2, the bench packages never run; OPEN-FOUNDRY-2, no cost
claim in a published report; OPEN-TETHER-5, the fleet scheduler assigns jobs by the role table only.

## Acceptance criteria

- [ ] `registries/open.toml` contains the twenty-seven records with no empty required field; OPEN-TETHER-1 has
      status `decided` with its ADR.
- [ ] OPEN-ATEL-4 has `confirmed_by_gate_register = true`.
- [ ] Every blocking row names a decision or experiment bead that exists in the tracker; where two beads decide
      one question, `decision_bead` is the one that writes the ADR (N29b-12, N30b-26, N41-13, N41-12), `note`
      names the other, and the printout shows both.
- [ ] The owner rows link to N41-10, N41-11, N41-12, N41-13 and N41-14.

## Test plan

### Unit tests
`crates/frob-conformance/tests/open_part6.rs`: `twenty_seven_rows_load`; `ids_match_expected_set` (ATEL-1 to -6,
CHRON-1 to -5, GROUND-1 to -5, FOUNDRY-1 to -6, TETHER-1 to -5); `tether_1_is_decided`;
`pending_decision_marker_fails_at_asked_milestone` (synthetic: a row with the marker still set at its asked-at
exit fails); `double_decided_rows_listed` (the four owner questions with two beads each); `owner_rows_link_to_this_unit`; `defaults_quote_known_numbers` (256 ticks
and 2 s; 30 days and 64 GB; 90 days or 200 hours; 2 s; 5,000 and 30,000; 5%, 64, 5 s, 600 s).

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04).

### Known answers and negative controls
Known answers: 27 rows; 1 decided; 1 confirmed. Negative controls: a Ground output labelled above `Estimated` while
OPEN-GROUND-5 is at its default; a fatigue claim on a printed part while OPEN-FOUNDRY-4 is at its default; a
pending-decision marker left set past its asked-at milestone.

## Considerations

- No row of this part is waiting for a decision bead any more; the pending marker of the schema stays available
  for questions added later, and makes such a gap fail one milestone before the question's deadline.
- OPEN-GROUND-1 and OPEN-GAUNT-1 are two halves of one matter: the engineering half (does a suitable dataset
  exist) belongs to the Ground lead; which dataset is admitted, with what licence, is the owner's.

## References

§42.3 (rows for Atelier, Chronicle, Ground, Foundry, Tether), §42.2 (owner rows), §42.4 (decided and confirmed
rows).
