---
key: "N41-OPEN7"
title: "registries/open.toml, part 7: open questions of suite consumption, workspace, numerics, performance and safety (23 rows, one decided)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "suite", "workspace", "spark", "safety", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-03"]
needs: ["§33 to §37: the decision and experiment beads named in the last column are owned by the chapter units; for OPEN-SAFE-2, -3 and -4 this unit also wrote an owner packet (N41-11) and both keys are named"]
related: ["N41-OPEN6", "N41-OPEN8", "N41-11", "N41-14", "N33c-33", "N33c-34", "N33c-35", "N33c-04", "N40-35"]
gates: []
plan_refs: ["§42.3", "§42.2", "§33.16", "§34.13", "§35.16", "§36.15", "§37"]
---
## Background

Seventh part of the open-question register (rule and schema in N41-03; how to read a row in N41-OPEN1). The
chapters here cut across subsystems:

- **Suite consumption**: how sibling libraries enter the build (pinned commits, one toolchain T for the whole
  build graph, one asupersync package, the airlock crate that confines volatile types);
- **Workspace**: the crate map, layering rules, the six `xtask` checks, formats;
- **Numerics and determinism**: determinism classes, banned constructs, anchored positions, addressed draws,
  scaling for single-precision solves;
- **Performance**: the measurement protocol, parity against incumbents, disclosure blocks;
- **Safety, security and privacy**: allowlists for weights, private places, licences, retention.

Owner questions in this part: OPEN-SUITE-3 and OPEN-SUITE-5 at M0 (a dirty sibling checkout for development
builds; the budget of the cold build), and OPEN-SAFE-2, -3 and -4 between M2 and M4 (privacy inheritance for
weights; sandboxed custom reward terms; retention periods). OPEN-SUITE-2 was decided by the owner on 2026-11-22.

## Scope

In: the twenty-three rows below as records of `registries/open.toml`. Out: the schema and loader (N41-03); the
decisions.

## Technical approach

| ID | Question | Fail-closed default | Owner | Blocks, or decided by | Deadline | Decided in |
|---|---|---|---|---|---|---|
| OPEN-SUITE-1 | Which asupersync version the whole graph resolves to | The one published version that the upstream work item FD-ASUP-01 aligns the siblings on; until it lands, whatever FrankenSim's pinned commit builds with (0.6.2 by path on the day of writing) | Build lead | The admission gate INT-ASUP-01 | M0 | N33c-35 |
| OPEN-SUITE-2 | Decided by the owner on 2026-11-22 and kept as a recorded decision. The question was whether the core moves to a nightly at or above 1.100 so that `frob-mcp` may join the core graph | Decision: one toolchain T for the whole build graph, chosen at INT-TOOLCHAIN-01 in the stated order of candidates; fastmcp_rust is brought onto T by FD-FMCP-01; `frob-mcp` is a member of the core graph and runs verbs in process. Cost: the upstream work of FD-FMCP-01 and FD-ASUP-01 at M0; KILL-MCP's fallbacks if INT-FMCP-01 cannot pass | Owner | Decided; confirmed by INT-TOOLCHAIN-01 and INT-FMCP-01 | M0 (confirmation) | N41-14; BD-031, BD-023 |
| OPEN-SUITE-3 | Whether a dirty asupersync checkout is acceptable for development builds between receipts | Yes for development, with the tree hash in the receipt; no gate records a PASS | Owner | INT-FSIM-01 | M0 | N33c-33 |
| OPEN-SUITE-4 | The alias set of `frob_core::qty`, and whether it stays within 64 | The quantities used by the algebra, design, actuator, dynamics, mixed-batch and contact chapters only | Marrow lead | INT-TOOLCHAIN-01 | M0 | N33c-04 |
| OPEN-SUITE-5 | The budget for the cold build of the M1 closure on the build workers | The machine envelope: one hour, 16 GiB, 8 cores | Owner | INT-FSIM-01 | M0 | N33c-34 |
| OPEN-SUITE-6 | Whether computational crates take asupersync's `Cx` or a capability-free token from `frob-core` | `Cx` wherever cancellation is observed; the token only if INT-ASUP-01 fails after FD-ASUP-01 and a fallback splits the graph | Tether lead | INT-ASUP-01 | M0 | N33c-35 |
| OPEN-WS-1 | The rank order inside layers 4 and 5, where the product chapters may need edges the crate map did not foresee | The order of the crate map; a change is an ADR | Build lead | The first contract of each crate | M1 | N34-02; BD-003 |
| OPEN-WS-2 | Whether `frob-tui` stays in the core workspace or becomes an annex | Core. If the admission INT-FTUI-01 shows that frankentui does not build on T, the remedy is an upstream change in frankentui or the toolchain train, with no dashboard until it lands | Tether lead | INT-FTUI-01 | M3 | N33c-23; BD-300 |
| OPEN-WS-3 | The stable release named in `frob-cert`'s toolchain file | The newest stable release on the day of its first commit, then moved only by ADR | Lemma lead | The first commit of `frob-cert` | M0 | BD-026; N34-25 |
| OPEN-WS-4 | Whether the mission file format is owned by `frob-spec` or by `frob-mission` | `frob-spec`, so that there is one parser for requirement terms | Augur lead | The contract of `frob-spec` | M1 | N24a-01; N34-19 |
| OPEN-WS-5 | Whether the Gymnasium adapter binds in process or across a process boundary | Across a process boundary with shared array files, so that no interpreter is ever in a `frob` binary | Cortex lead | The adapter's contract | M3 | N23b-27 |
| OPEN-NUM-1 | Whether an explicit fused multiply-add is ever admitted in batched kernels | No | Spark lead | Nothing; the evidence is the probe's record of fused rounding on each device family | none | seed BD-017; N21a-10 (decision bead needed if reopened) |
| OPEN-NUM-2 | The decision-equivalence thresholds (1% discordant bound and 0.5% net rate) and the ensemble size | As stated, frozen once at M2 with a written rationale | Spark lead | GATE-SPARK-16 | M2 | N35-11 |
| OPEN-NUM-3 | The cell side of anchored positions | 8 m, with the offset centred in [−4, 4) m | Spark lead | Nothing | none | N21b-37; BD-157 |
| OPEN-NUM-4 | The limit for `Degraded: ill-conditioned` and the refinement count of the `stiff` profile | Conditioning figure κ̂·u above 0.1; one refinement step | Spark lead | GATE-SPARK-18 | M2 | N35-29 (also N21b-38) |
| OPEN-PERF-1 | The tolerance ratio of 0.67 against the incumbent and the band below it | As stated; below 0.33 KILL-PARITY fires | Performance lead | GATE-SPARK-03 | M2 | N36-24 |
| OPEN-PERF-2 | Which incumbent revisions and configurations are the comparators | The revisions in the source register, frozen by the ten-fold protocol before any paired run | Gauntlet lead | GATE-SPARK-21 | M0 | N36-25 |
| OPEN-PERF-3 | A credible energy source on each machine | A named gap in every disclosure block | Performance lead | Nothing | none | considerations of BD-216 |
| OPEN-PERF-4 | How the reference performance rows are restated if the default physics step moves from 2 ms (OPEN-KIN-1) | Every row is restated with its new decimation before any gate cites it | Performance lead | Every gate that cites the reference rows | M2 | N36-26 (with N16c-06) |
| OPEN-SAFE-1 | Which weight layout and which policy architectures are on the allowlist at M2 | The single layout and family of the learning chapter's reference policy | Cortex lead | GATE-SAFE-07 | M2 | N37-08 |
| OPEN-SAFE-2 | Whether policy weights trained in a private or research-only scene inherit its mark and licence set | They inherit both | Owner | GATE-SAFE-04, GATE-SAFE-05 | M2 | N41-11 (owner packet); N37-20 |
| OPEN-SAFE-3 | Whether sandboxed custom reward terms are admitted, and in what sandbox | Not admitted; the closed expression language only | Owner | M4 | M4 | N41-11 (owner packet); N37-09 |
| OPEN-SAFE-4 | Retention periods for job logs, idempotency keys and retired confirmation clusters | Job logs and keys for 30 days; retired clusters sealed indefinitely | Owner | M3 | M3 | N41-11 (owner packet); N37-30 |

Status at entry: twenty-two `open`, one `decided` (OPEN-SUITE-2). Two defaults are confirmed by the gate register
and are open only to new evidence: the anchored cell of OPEN-NUM-3 and the parity tolerance of OPEN-PERF-1. What
the owner defaults cost: OPEN-SUITE-3 and OPEN-SUITE-5, a gate run waits for a clean tree, and KILL-FSIM fires if
the cold check exceeds the envelope; OPEN-SAFE-2, -3, -4, none beyond the defaults themselves.

## Acceptance criteria

- [ ] `registries/open.toml` contains the twenty-three records with no empty required field; OPEN-SUITE-2 has
      status `decided` with its ADR.
- [ ] OPEN-NUM-3 and OPEN-PERF-1 have `confirmed_by_gate_register = true` and their values (8 m, [−4, 4) m; 0.67 and
      0.33) equal the gate register's and the contracts'.
- [ ] OPEN-NUM-1 (no decision bead: it blocks nothing and its default is a ban enforced by the lint) is listed as
      carried in considerations; every blocking row names a decision bead that exists.
- [ ] The owner rows at M0 (OPEN-SUITE-3, OPEN-SUITE-5) appear in the M0 listing with their defaults and costs.

## Test plan

### Unit tests
`crates/frob-conformance/tests/open_part7.rs`: `twenty_three_rows_load`; `ids_match_expected_set` (SUITE-1 to -6,
WS-1 to -5, NUM-1 to -4, PERF-1 to -4, SAFE-1 to -4); `suite_2_is_decided`; `confirmed_rows_match_gate_register`;
`owner_rows` (SUITE-2, -3, -5; SAFE-2, -3, -4); `non_blocking_rows_listed`; `defaults_quote_known_numbers` (64
aliases; one hour, 16 GiB, 8 cores; 1% and 0.5%; 8 m; 0.1; 0.67 and 0.33; 30 days).

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04).

### Known answers and negative controls
Known answers: 23 rows; 1 decided; 2 confirmed. Negative controls: a gate PASS recorded on a dirty sibling tree
(refused whatever the answer to OPEN-SUITE-3); a kernel with an explicit fused multiply-add admitted while
OPEN-NUM-1 is at its default (refused by the dialect lint); a custom reward term in a sandbox while OPEN-SAFE-3 is
at its default.

## Considerations

- OPEN-SUITE-2 and OPEN-TETHER-1 are one decision seen from two chapters; both identifiers stay in the register
  with the decision and are confirmed by two admission gates at M0.
- OPEN-PERF-2 is due at M0 because the denominator of the ten-fold ratio is measured at M0; a comparator that
  changes after a paired run invalidates the pair.

## References

§42.3 (rows for suite consumption, workspace, numerics, performance, safety), §42.2 (owner rows), §42.4 (decided
and confirmed rows), §33.13 (the toolchain decision).
