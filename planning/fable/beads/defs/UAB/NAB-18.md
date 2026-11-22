---
key: "NAB-18"
title: "registries/ev_matrix.toml: the 41 families adopted at tier Solid, with scores, decisions and kill gates"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "solid", "registry", "decision-record"]
parent: "EPIC-M0"
deps: ["NAB-17"]
needs: ["§40.4: the kill-gate records that the decision column names, each with its trigger, fallback and deciding milestone (N40-23 to N40-27)"]
related: ["NAB-19", "NAB-20", "NAB-21", "NAB-22", "NAB-50", "N40-30"]
gates: []
plan_refs: ["Appendix C.1.1", "Appendix C.1"]
---
## Background

FrankenRobotics scored every family of mechanism it considered: Score = (Impact × Confidence) ÷ Effort, each
from 1 to 5, printed to one decimal. Impact is the effect on the time and compute to an independently accepted
design decision (and, for evidence mechanisms, on what may be trusted); Confidence is the judgment that the
mechanism delivers it, after the planning evidence; Effort is engineering cost with 5 the largest. The scores
are DESIGN judgments, recorded so that they can be argued with. The registry is `registries/ev_matrix.toml`
(schema `frob.evmatrix/1`, NAB-17).

This bead carries the first of its three tables: the 41 families **adopted at tier Solid** (established
mathematics and engineering, where the risk is implementation quality). These are the spine of the system and
the mechanisms next to it: the Lie-typed algebraic core, the trust ladder, the energy ledger, the clock
contract, the blinded qualification lane, convex compliant contact, the claim product and the offline auditor.
A kill gate in the decision column names what the system falls back to if a measurement goes against the
optional part of the family; **a row without a kill gate has no optional part**. Rows are sorted by score,
highest first. Tier: Solid.

## Scope

In: 41 `[[family]]` records, ids `EV-S01` to `EV-S41`, table `solid`. Out: the schema and lints (NAB-17); the
other two tables (NAB-19, NAB-20); re-scoring (NAB-22 and the sweeps).

## Technical approach

**Transcription rule.** One record per row, in table order. `family` is the second column; `where` is the third
column split at semicolons and commas into constructions (`C-nn`), sections, analyses (`A-nn`), decision cores
(`DC-nn`), gates and bets; `impact`, `confidence`, `effort` the three integers; `score` the printed string;
`decision_text` the last column verbatim; `decision = "adopted"` for every row of this table; `tiers` the tier
letters the decision names in order (`["S"]`, or `["S", "F"]` where a Frontier part is named); `kill_gates`,
`gates`, `bets` the identifiers found in the row; `killed_idea` the `KI-nn` a row points to; `carried = true`
for families that existed in the earlier version of the matrix. `needed_by` is empty throughout: no row of
this table is below 2.0.

**What the decision column tells an implementer, row by row class.**

- *No optional part* (28 rows with no kill gate): the family is built as specified; there is no fallback
  because there is nothing to fall back from. Examples: inertia on the positive-definite cone, the
  Euler–Poincaré framing, e-processes for paired comparison, the two study types, gym semantics, Cayley updates,
  screening audits, power-of-two scaling, submodular scene coverage, the trust ladder, invariant filtering, the
  qualification lane, the energy ledger, integer histograms.
- *A Solid core with a Frontier extension named in the decision* (rows that say "… is [F]"): the Lie-typed core
  (lattice-based mobility analysis is [F]), the inertia cone (Riemannian identification is [F] in the
  calibration lane), the growth vector (the maneuverability index and bracket primitives are [F]), finite
  symmetry groups (adopted [S] for detection of both groups, augmentation and metamorphic tests), the canonical
  form (canonical policy inputs are [F]), rare-failure bounds (splitting and adaptive proposals are [F] and
  explore only), the value of an experiment (probe synthesis is [F]).
- *A kill gate with its fallback* (13 rows): static analyses in `design check` (KILL-STATIC downgrades verdicts
  to advisory lints); the clock contract (race enumeration falls back to the fixed total order under
  KILL-SCHED); exact-integer and guarded kernels (KILL-MARROW-GUARD); the requirement language (as a reward term
  only through the pilot of KILL-SPEC-REWARD); rare-failure bounds (KILL-RARE); interventional autopsy
  (KILL-AUTOPSY); the four-dimensional contact cone and joint rows (KILL-SINEW-ROWS); the memo graph with
  revalidation (grades 1 and 2 adopted; higher grades stand or fall with KILL-REUSE); batch-composition
  invariance and decision-level equivalence (KILL-D1 demotes a device family to class D2); the MCP surface
  (KILL-MCP falls back to a separate workspace over a local socket, then to the CLI); convex compliant contact
  as law CL-1 (KILL-CONTACT); the film-friction law and rigid proxies (KILL-FILM); the value of an experiment
  (KILL-VOI, KILL-AUGUR-PROBE).
- *Decision cores*: DC-01 to DC-11 are adopted, each shipping in shadow behind its fixed-table fallback.

**The rows.**

| Id | Family | Where | I | C | E | Score | Decision |
|---|---|---|---|---|---|---|---|
| EV-S01 | Inertia on the positive-definite cone | C-03; §11.5, §13, §14.5 | 4 | 5 | 1 | 20.0 | Adopted [S]; Riemannian identification of inertial parameters is [F] in the calibration lane (§30.9) |
| EV-S02 | Euler–Poincaré framing and the momentum map | C-04; §11.2, §16 | 4 | 5 | 1 | 20.0 | Adopted [S] |
| EV-S03 | E-processes for paired comparison on common world addresses | C-15; §24.12, §28.6 | 4 | 5 | 1 | 20.0 | Adopted [S]; one of the two declared study types |
| EV-S04 | Two declared study types, never mixed: anytime-valid, and fixed-n with a pre-analysis of reachable verdicts | §24.12, §28.6; `eval.preanalyze` | 4 | 5 | 1 | 20.0 | Adopted [S]; GATE-ARENA-05, GATE-ATEL-06 |
| EV-S05 | Gym semantics: explicit reset with the final observation kept, termination apart from truncation, action likelihood, physical discounting | §23.3 | 4 | 5 | 1 | 20.0 | Adopted [S]; masked automatic reset is a labeled mode |
| EV-S06 | Group-preserving Cayley updates | C-05; §16.7 | 4 | 4 | 1 | 16.0 | Adopted [S] |
| EV-S07 | Screening audits: sampled discarded strata and a planted winner | §26.10, §28.12 | 4 | 4 | 1 | 16.0 | Adopted [S]; GATE-ATEL-09 |
| EV-S08 | Diagonal power-of-two scaling for single-precision contact solves | §18.12, §35.10 | 3 | 5 | 1 | 15.0 | Adopted [S]; the surviving alternative to half precision (KI-01) |
| EV-S09 | Submodular coverage of scene sets by lazy greedy selection | §26.11; DC-10 | 3 | 5 | 1 | 15.0 | Adopted [S] |
| EV-S10 | Lie-typed core on `fs-ga` | C-01, C-02; §13 | 5 | 5 | 2 | 12.5 | Adopted [S]; lattice-based mobility analysis is [F] |
| EV-S11 | Trust ladder, fidelity ledger and the promotion rule | §16.8; DC-01 | 5 | 5 | 2 | 12.5 | Adopted [S] |
| EV-S12 | Growth vector and maneuverability | C-07; A-07, §28.4 | 3 | 4 | 1 | 12.0 | Adopted [S] for the growth vector; the index and the bracket primitives are [F] |
| EV-S13 | Invariant filtering | C-13; §22.3 | 4 | 5 | 2 | 10.0 | Adopted [S] |
| EV-S14 | Static analyses in `design check` | §14.11, §28.4; BET-05 | 5 | 4 | 2 | 10.0 | Adopted [S]; KILL-STATIC downgrades verdicts to advisory lints |
| EV-S15 | Clock contract: rational periods, phase accumulators, seven instants, packet queues inside every checkpoint | C-19; §11.7, §11.8, §20.4 | 4 | 5 | 2 | 10.0 | Adopted [S]; race enumeration falls back to the fixed total order under KILL-SCHED |
| EV-S16 | Blinded qualification lane with the error budget α_i = α/[i·(i+1)] | §24.13; GATE-QUAL-01 to -03 | 5 | 4 | 2 | 10.0 | Adopted [S] |
| EV-S17 | Exact-integer and guarded single-precision kernel classes | §13.7, §21.8; GATE-SPARK-08 | 4 | 5 | 2 | 10.0 | Adopted [S]; KILL-MARROW-GUARD |
| EV-S18 | Energy ledger, and the audit for energy created by the simulator | §16.10, §29.9 | 4 | 5 | 2 | 10.0 | Adopted [S] |
| EV-S19 | Deterministic integer log-bin histograms and a reservoir chosen by address | §29.6, §29.7, §35.7 | 2 | 5 | 1 | 10.0 | Adopted [S]; replaces order-dependent sketches (KI-11) |
| EV-S20 | Finite symmetry groups, computed: the mechanical group and its observation-preserving subgroup | C-09; §13.4, §20.13 | 4 | 4 | 2 | 8.0 | Adopted [S] for detection of both groups, augmentation and metamorphic tests |
| EV-S21 | Requirement language: bounded-time temporal logic over unit-normalized predicates | C-30; §24.5, §24.6 | 4 | 4 | 2 | 8.0 | Adopted [S] as acceptance metric and search score; as a reward term only through the pilot of KILL-SPEC-REWARD |
| EV-S22 | Rare-failure bounds from bounded-weight importance sampling and crude sampling | C-27; §26.9; DC-04 | 4 | 4 | 2 | 8.0 | Adopted [S]; splitting and adaptive proposals are [F] and explore only; KILL-RARE |
| EV-S23 | Interventional autopsy on forks, with a placebo fork and a flip rate | C-29; §27.3 to §27.6; DC-06 | 4 | 4 | 2 | 8.0 | Adopted [S]; KILL-AUTOPSY |
| EV-S24 | Four-dimensional contact cone, and joint-level stiction, backlash and stop rows | C-37; §15.6, §18.6, §18.7 | 4 | 4 | 2 | 8.0 | Adopted [S]; KILL-SINEW-ROWS |
| EV-S25 | Port-Hamiltonian actuators with a thermal node | C-38; §15.5, §15.7 | 4 | 4 | 2 | 8.0 | Adopted [S] |
| EV-S26 | Memo graph past model identity, with dependency-directed revalidation of verdicts | §28.13, §28.14 | 4 | 4 | 2 | 8.0 | Adopted [S] for reuse grades 1 and 2; higher grades stand or fall with KILL-REUSE |
| EV-S27 | Batch-composition invariance and decision-level equivalence under class D3 | §35.4, §35.5; GATE-SPARK-15, -16 | 4 | 4 | 2 | 8.0 | Adopted [S]; KILL-D1 demotes a device family to class D2 |
| EV-S28 | Offline auditor `frob-verify` and the claim product | §8.2, §12.2, §12.4, §34.11 | 4 | 4 | 2 | 8.0 | Adopted [S]; GATE-WS-03 |
| EV-S29 | MCP surface on `fastmcp_rust`, generated from the verb registry, in the core graph with verbs run in process | §32.12 | 4 | 4 | 2 | 8.0 | Adopted [S]; KILL-MCP falls back to a separate workspace over a local socket, then to the CLI |
| EV-S30 | Symmetric-cone abstraction and one cone solver | C-06; §22.5 | 3 | 5 | 2 | 7.5 | Adopted [S] |
| EV-S31 | Crash-consistent job protocol with atomic idempotency binding | §32.7, §32.10 | 3 | 5 | 2 | 7.5 | Adopted [S] |
| EV-S32 | Convex compliant contact with a referee audit | C-06; §16.8, §18.5 | 5 | 4 | 3 | 6.7 | Adopted [S] as law CL-1; KILL-CONTACT |
| EV-S33 | Per-world parameter variants within one topology | §17.3, §21.11 | 5 | 4 | 3 | 6.7 | Adopted [S] |
| EV-S34 | Observability Gramian modulo a gauge | C-20; §20.11; A-17 | 3 | 4 | 2 | 6.0 | Adopted [S] |
| EV-S35 | Canonical form of a design up to the similarity group | C-21; §14, §17.11 | 3 | 4 | 2 | 6.0 | Adopted [S] for the canonical form and the exact power-of-two test; canonical policy inputs are [F] |
| EV-S36 | Film-friction law and rigid proxies | §18.8, §19.10.2 | 3 | 4 | 2 | 6.0 | Adopted [S]; KILL-FILM |
| EV-S37 | Decision cores DC-01 to DC-11, each with a fixed-table fallback | §28.19 | 3 | 4 | 2 | 6.0 | Adopted; each core ships in shadow behind its fallback (GATE-ATEL-13, -14) |
| EV-S38 | Placement planner with requested-versus-actual receipts and strict-backend errors | §21.12; GATE-SPARK-09 | 3 | 4 | 2 | 6.0 | Adopted [S] |
| EV-S39 | Scene acceptance | §19.5 | 3 | 4 | 2 | 6.0 | Adopted [S] |
| EV-S40 | Transactional, event-located referee step | §16.5 | 4 | 4 | 3 | 5.3 | Adopted [S] on the referee and the `strict` profile; throughput tiers keep the lockstep solve |
| EV-S41 | Value of an experiment in closed form, used as a stop rule | C-39; §28.16, §30.15; DC-05 | 3 | 3 | 2 | 4.5 | Adopted [S] for the closed form and the ranking; probe synthesis is [F]; KILL-VOI, KILL-AUGUR-PROBE |

Rows in this table: 41.

## Acceptance criteria

- [ ] 41 records `EV-S01` … `EV-S41` exist in table `solid`, in the order above, and the loader and lints of
      NAB-17 pass on them.
- [ ] Every printed score equals the derived score; five rows score 20.0 and the last row scores 4.5.
- [ ] 13 rows name at least one kill gate, 14 distinct kill gates in all, and each resolves to a record of the
      gate register; 28 rows name none.
- [ ] No row is below 2.0 (rule 1 has nothing to excuse in this table).
- [ ] The generated printout shows the 41 rows with their decision text character for character.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ev_matrix.rs`, module `solid`:

- `count_is_41`; `order_matches_frozen_list`.
- `scores`: each row's `(impact, confidence, effort)` gives its printed score; spot values: the Lie-typed core
  (5, 5, 2) → "12.5"; convex compliant contact (5, 4, 3) → "6.7"; the transactional referee step (4, 4, 3) →
  "5.3"; the value of an experiment (3, 3, 2) → "4.5".
- `five_rows_at_20`: inertia on the positive-definite cone; Euler–Poincaré framing; e-processes; the two study
  types; gym semantics.
- `kill_gate_rows_are_13` and `kill_gates_are_14`: KILL-STATIC, KILL-SCHED, KILL-MARROW-GUARD, KILL-SPEC-REWARD,
  KILL-RARE, KILL-AUTOPSY, KILL-SINEW-ROWS, KILL-REUSE, KILL-D1, KILL-MCP, KILL-CONTACT, KILL-FILM, KILL-VOI,
  KILL-AUGUR-PROBE.
- `decision_text_frozen`: byte comparison with `tests/fixtures/ev/solid.tsv` (this table).
- `cross_references_resolve`: `KI-01` (half precision, named by the power-of-two scaling row) and `KI-11`
  (order-dependent sketches, named by the integer-histogram row) exist in the killed-idea register.

### End-to-end script and logging

`scripts/e2e/decision_records.sh` (NAB-50) asserts the count (41), zero score mismatches and zero rows below
2.0 for this table, logged as `metric` lines per STD-E2E.

### Known answers and negative controls

Known answers: 41 rows; scores from 20.0 down to 4.5; 13 kill-gate rows. Deciding milestones of the kill gates
in this table (from the gate register): KILL-MCP at M0 (re-read at M1); KILL-SCHED, KILL-MARROW-GUARD,
KILL-SINEW-ROWS, KILL-D1 and KILL-CONTACT at M2; KILL-STATIC, KILL-SPEC-REWARD, KILL-RARE, KILL-AUTOPSY,
KILL-FILM and KILL-AUGUR-PROBE at M3; KILL-REUSE and KILL-VOI at M4. Negative controls: a row moved above a
higher-scoring row (`ev.unsorted`); the trust-ladder row given a kill gate that does not exist
(`ev.kill-gate-unknown`); a score edited to "12.0" for (5, 5, 2).

## Considerations

- "Adopted, Solid" is not "done": every row still inherits the definition of implemented (code in its crate, a
  contract, known answers against a pinned oracle, evidence on outputs, a CPU reference path, reachability from
  the command line, cancellation tested, a recorded gate).
- When a kill gate of this table fires, the row does not move to another table; its decision text gains the
  fallback's name and a history entry (NAB-22), and the capability is described by the fallback's name.

## References

Appendix C.1.1; the three table rules of C.1.
