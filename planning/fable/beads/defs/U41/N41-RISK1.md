---
key: "N41-RISK1"
title: "registries/risks.toml, part 1: foundations, toolchain, build and workspace risks (RISK-01, -02, -03, -16, -18 to -24, -54 to -57)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "suite", "workspace", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-01"]
needs: ["§33: the keys named in the Beads column are owned by the suite, workspace and upstream units; they are named here for traceability and are not blocking dependencies of this registry"]
related: ["N41-RISK2", "N41-RISK3", "N41-RISK4", "N41-PM", "N41-02", "N41-32", "BD-044", "N33b-05", "BD-031", "BD-008", "N34-04", "N33c-38"]
gates: ["GATE-WS-07"]
plan_refs: ["§41.3", "§41.4", "§41.5", "§33", "§34"]
---
## Background

The risk register asks how the program fails, names the earliest measurement that would show each failure coming,
and points at the mechanism that answers it. It is data (`registries/risks.toml`, schema and checks in N41-01), read
at every milestone exit by the fresh-context session that records the exit. This bead enters the fifteen rows whose
subject is the foundations the program stands on: sibling libraries consumed by path, the one toolchain T of the
build graph, the build itself, and the workspace's own records.

Conventions for every part of the register. **Severity** is High when the risk can invalidate the thesis, the
architecture or a milestone exit; Medium when it costs a capability or a claim; Low when it costs convenience. The
**owner** is a role, not a person. Every threshold in a signal or mitigation is a TARGET repeated from the gate
named beside it, and the gate register governs where the two differ; incumbent and sibling figures are FACTs to be
re-read at the pinned commits. **No risk is mitigated by a hope**: a mitigation names a mechanism with a gate, a
fallback named in advance, or an owner decision with a fail-closed default; an upstream work item is never a
mitigation by itself, its interim posture is. Each row also names a **planted instance**: a deliberately
constructed case, registered in the negative-control registry, that makes the row's earliest signal fire, so that
the signal is known to work before it is needed.

Identifiers. RISK-01 to RISK-55 are the plan's. The four risks that follow from late foundation facts were given
no numbers in the plan ("identifiers are assigned with the rest of the series"); they are assigned here as RISK-56
(a dependency feature changes canonical bytes), RISK-57 (an exact version pin blocks an oracle), RISK-58 (a sampler
with a wrong joint law, in part 3) and RISK-59 (a rare slow mission absent from the sealed sample, in part 2).

## Scope

In: the fifteen rows below, complete, as records of `registries/risks.toml`. Out: the schema, loader and exit review
(N41-01); the planted instances' implementation for RISK-56 to RISK-59 and gate GATE-WS-07 (N41-02); the
mitigations themselves (the beads named per row).

## Technical approach

Each row becomes one `[[risk]]` record with the fields of N41-01: `id`, `title`, `severity`, `signal` (text plus
the `detector`: a check, gate, admission gate or ledger query that produces it), `mitigation`, `owner`, `detects`
(gate and kill-gate identifiers), `beads`, `planted`, `first_readable` (the earliest milestone exit at which the
signal can be evaluated).

| ID, severity | Risk | Earliest signal (detector) | Mitigation | Owner | Detected or decided by | Beads that carry the mitigation | Planted instance; first readable |
|---|---|---|---|---|---|---|---|
| RISK-01, High | FrankenSim API churn in `fs-mbd` and the `Component` trait. Context (FACT): 409 commits touched `fs-mbd` in thirty days; the component contract version is 0 | Adapter changes per pin move; the upstream breakage share (above 20% of working days in a four-week window is the trigger of BET-04). Detector: the compatibility train log | Airlock (volatile types named in one crate), pinned commit, compatibility train; upstream work item FD-FSIM-08; KILL-FSIM with the worker process first and the lift second | Airlock lead | KILL-FSIM; BET-04 | BD-006, BD-007, N33a-11, N33b-04, NF-01, N33a-10, N33b-05, BD-038, FD-FSIM-08, BET-04 | A scratch sibling commit that renames a symbol the airlock uses: the adapter-change count rises by one and the train log records a breaking day. M0 |
| RISK-02, High | The toolchain train costs more than M0 allows, or breaks a sibling. One toolchain T serves the whole build graph and the siblings start apart: four nightlies and one stable release are in play; fastmcp_rust declares a minimum of 1.100 where FrankenSim pins a 1.99 nightly (FACT) | A failing cell in the matrix of INT-TOOLCHAIN-01; FD-FMCP-01 still open at the M0 exit review | T chosen in order: candidate (a) the suite's common nightly, candidate (b) FrankenSim's present pin if FrankenSim cannot yet build on (a); FD-FMCP-01 at P0 brings fastmcp_rust onto T; afterwards T moves only by a toolchain train followed by the admission and parity gates; KILL-MCP's fallbacks | Build lead | INT-TOOLCHAIN-01; INT-FMCP-01; KILL-MCP | BD-031, N33c-01, N33c-08, NF-02, NF-03, NF-04, NF-05, BD-023, FD-FMCP-01, BD-044, N41-20 | A toolchain file naming a nightly on which one build set fails: its matrix cell is recorded failing and the exit check lists it. M0 |
| RISK-03, High | Two asupersync versions in one build. Three siblings still pin 0.5.0 (FACT) | `cargo tree -d` at INT-ASUP-01 shows two packages | FD-ASUP-01 at P0 aligns the consumed siblings on one published version; `check-deps` forbids 0.5.0 and the frankentui executor feature; those siblings never enter a graph; KILL-MCP's first fallback if a surface still brings a second package | Build lead | INT-ASUP-01; GATE-WS-01 | BD-008, FD-ASUP-01, N34-03, N33c-09, N33c-29, BD-003 | A seeded second asupersync version in a scratch manifest fails `check-deps` and `check-stack`. M0 |
| RISK-16, Low | Lean work needs a large-memory Linux host | The closure size, memory and time measured at INT-FLN-01 | Measure first (OPEN-LEMMA-1: no claim before the measurement); specifications stated in elementary form without analysis modules (OPEN-LEMMA-2) | Lemma lead | INT-FLN-01; GATE-LEMMA-05 | BD-380, N12-23, N12-26 | A re-check job run under a 1 GiB memory cap must end as "budget exceeded", never as a pass. M5 |
| RISK-18, Medium | Crate scope caps are exceeded early | A crate at 115% of its line cap in `check-contracts` | The scope conversation at 115%; an ADR raises the cap or moves a mechanism; no silent growth (line counts are caps, never schedules) | Project lead | GATE-WS-01 | N34-08, N34-02, BD-004 | A scratch crate padded past 115% of its cap fails `check-contracts` with the crate named. M0 |
| RISK-19, Low | The viewer drifts outside FrankenThreeD's admitted subset. FrankenThreeD has no single-file packaging, so the HTML export is ours (FACT) | The admission test INT-F3D-01 | Plain Three.js is the fallback and is always built | Viewer lead | INT-F3D-01 | BD-300, N33a-24, N33c-24, FD-F3D-01 | A viewer page that calls an interface outside the admitted subset fails the admission test while the plain bundle still builds. M3 |
| RISK-20, Medium | Upstream work costs more than planned, or is done in the wrong order: an upstream work item that a switch-over needs is still open when its milestone exits | An upstream work item older than two milestones with no change of state; a P0 item still open at the M0 exit review. Detector: `registries/upstream.toml` ages | Every upstream work item has an interim posture that ships and a switch-over rule; no milestone waits on one landing; P0 items are done at M0 and the rest are ordered by priority in this program's queue | Project lead | The exit review's upstream listing | BD-037, N33b-08, BD-008, BD-023, BD-044 | A registry record whose state has not changed for two milestones is listed as stale by the exit check. M0 |
| RISK-21, High | The `fs-qty` coupling ties the whole build graph to nightlies on which an incomplete compiler feature (`generic_const_exprs`) still builds one crate | The `fs-qty` row of the INT-TOOLCHAIN-01 matrix after a pin move or a toolchain train | Quantities reach every crate through `frob_core::qty` only; the preferred fix is an upstream change in FrankenSim (FD-FSIM-15, FD-FSIM-31); the `frob-units` escape hatch as the fallback | Build lead | INT-TOOLCHAIN-01 | BD-031, N33c-02, N33c-03, N33c-05, N33c-06, BD-005, FD-FSIM-15, FD-FSIM-31 | A direct `fs_qty` import outside `frob-core` and `frob-fsim` fails `check-deps`; a matrix cell with `fs-qty` failing triggers the escape-hatch decision rule. M0 |
| RISK-22, High | The FrankenSim closure makes builds slow and makes the lift expensive: the M1 set compiles 65 crates and about 2.06 million lines (FACT) | Cold check time and peak memory at INT-FSIM-01 against the machine envelope (one hour, 16 GiB, 8 cores) | Feature groups in the airlock; the worker process before any lift (INT-FSIM-18); the cold-build budget question OPEN-SUITE-5 | Airlock lead | INT-FSIM-01; INT-FSIM-18; KILL-FSIM | BD-006, N33a-11, BD-038, N33c-12, N33c-34, N33a-10, N33b-05 | A synthetic admission record with a cold check over the envelope makes the KILL-FSIM trigger evaluate to "fires". M0 |
| RISK-23, Medium | Dirty or off-pin sibling trees make a receipt unrepeatable | A drift class other than on-pin in `check-stack` at a gate run | The resolution receipt hashes bytes; no gate records a PASS on a dirty tree (OPEN-SUITE-3: dirty is acceptable for development only); the compatibility train | Build lead | GATE-WS-01 | N33a-04, N33a-05, N34-13, N33c-30, N33c-33 | A sibling checkout with one modified file during a gate run: the PASS is refused and the receipt records the drift class. M0 |
| RISK-24, High | A reported foundation defect is relied on before it is rechecked (the PPO update, the planar drive, the stride convention, fixed contact witnesses) | A known answer, an oracle or a baseline that cites one of those modules before its admission test has passed | INT-FSIM-05, -06, -07 and INT-FNP-02; the independent formula; our own PPO passes the same three probes first; witness migration above the airlock; an admission is never inherited | Airlock lead | INT-FSIM-05, INT-FSIM-06, INT-FSIM-07, INT-FNP-02 | BD-033, BD-034, BD-035, BD-036, FDEF-01, FDEF-02, FDEF-03, FDEF-04, N40-07, N33a-07 | A known-answer row that cites an unadmitted module is rejected by the dependency firewall. M0 |
| RISK-54, Medium | The plan and the registries drift apart | An identifier defined twice; a chapter threshold that differs from the gate register | The gate register governs; registries and status tables are generated; the delivery log is not kept in the plan file | Project lead | GATE-GAUNT-10 | N40-09, N40-02, N00-03, BD-004 | A duplicated identifier and a threshold stated two ways in a scratch registry are each reported by the lint. M0 |
| RISK-55, Medium | An upstream change collides with other agents working in the same sibling repository: overlapping edits to one file, a head that moves under a half-made change, or a reservation conflict that stalls the item | A file-reservation conflict in Agent Mail on a sibling path; an upstream work item whose sibling bead changes state without its bead here | File reservations through Agent Mail before any edit in a sibling; the sibling's own `AGENTS.md` and coordination rules; small additive changes that land on the sibling's `main` through its own gates; the interim posture ships meanwhile | Project lead | The exit review's upstream listing (mirror check) | BD-037, N33c-30, N33b-08, NF-06 | A mirrored bead whose state differs from its bead here is listed by the mirror check; a rehearsal edit without a reservation is refused by the train script. M0 |
| RISK-56, High | A dependency feature changes canonical bytes: a sibling that enters the build graph switches the JSON library to insertion-ordered maps or to another number representation, and digests of unchanged artifacts move | `xtask check-deps` reports a unified feature set of the JSON library that differs from the stack lock; a golden digest changes with no change to its artifact | Canonical bytes come only from this project's writer; the four-way feature build; the stack lock records the feature set | Workspace lead | GATE-WS-07 (M0 row); GATE-WS-05 | N34-04, N34-18, N34-21, N41-02 | A dummy dependency that enables the two library features: `check-deps` reports the feature set, and the golden corpus digest is unchanged because the writer does not use library maps. M0 |
| RISK-57, Medium | An exact version pin in a sibling blocks an oracle: two siblings require different exact versions of one package, so they cannot share a build graph | Dependency resolution reports two exact requirements on one package; the admission gate of the oracle cannot build | The oracle runs as a worker process in its own workspace; the pin move is an upstream work item (FD-FTX-01, FD-F3D-01) | Suite lead | GATE-WS-07 (M0 row); INT-FTX-01 | N33c-38, N33c-28, N33a-26, FD-FTX-01, FD-F3D-01, N41-02 | A second exact pin on one package behind an optional feature: resolution with every feature group enabled fails on it, although the default build resolves. M0 |

Counterexample carried by RISK-57 (the shortcut "a passing build means the pins are compatible"): a build that
never enables the oracle's feature flag resolves, and the conflict appears only when the flag is first turned on at
the oracle's milestone. The resolution check therefore runs with every optional feature group enabled.

## Acceptance criteria

- [ ] `registries/risks.toml` contains the fifteen records above; each loads under the schema of N41-01 with no
      empty field; each `beads` entry resolves to a bead in the tracker; each `detects` entry resolves in
      `registries/gates.toml` or is the literal exit-review listing.
- [ ] Every row names a planted instance and the milestone at which its signal is first readable.
- [ ] The two late rows carry the assigned identifiers RISK-56 and RISK-57 and the gate GATE-WS-07.
- [ ] No mitigation is a bare upstream work item: each row that names one also names its interim posture or a
      fallback (checked by the loader rule of N41-01).

## Test plan

### Unit tests
`crates/frob-conformance/tests/risks_part1.rs`: `fifteen_rows_load`; `ids_match_expected_set` (the exact list in the
title); `every_bead_key_resolves`; `every_detector_resolves`; `no_hope_mitigation` (the rule test of N41-01 applied
to these rows); `severity_counts` (7 High: RISK-01, -02, -03, -21, -22, -24, -56; 6 Medium: RISK-18, -20, -23,
-54, -55, -57; 2 Low: RISK-16, -19; the test prints the tally and compares it with this table).

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04), which loads the full register, fires the planted instances
whose milestone has been reached and writes one `assert` line per row with the signal's observed value.

### Known answers and negative controls
Known answers: the row count (15), the identifier set, the severity tally. Negative controls: a row with an empty
`planted` field; a row whose only mitigation is an upstream work item; a `beads` key that does not exist; each must
be rejected by the loader.

## Considerations

- The severity tally in the unit test is derived from the table, so an edit to a row's severity must change both;
  the test exists to make that edit deliberate.
- Rows whose signal is a process observation (RISK-20, RISK-55) are detected by the exit review's listing of
  `registries/upstream.toml`, not by a numeric gate; their planted instances exercise that listing.
- A risk that can no longer occur is tombstoned with the gate that retired it and is not renumbered (for example
  RISK-02 after T has been stable through two trains would still stay in the register until an ADR retires it).

## References

§41.3 (rows RISK-01 to -03, -16, -18 to -24, -54, -55), §41.5 (the two build rows and GATE-WS-07), §41.4 (use of
the register), §33.2, §33.12, §33.13, §34.14.
