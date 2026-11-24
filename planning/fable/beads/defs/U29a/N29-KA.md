---
key: "N29-KA"
title: "Chronicle known-answer registry: KA-707 to KA-720 with truths, tolerances, tiers and negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "chronicle", "frob-conformance", "solid", "registry", "test"]
parent: "WP-13"
deps: ["BD-025", "N38a-12", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N29a-04", "N29a-06", "N29a-10", "N29a-12", "N29a-13", "N29a-14", "N29a-15", "N29a-16", "N29a-19", "N29a-21", "N29a-22", "N29a-24", "N29b-20", "N29b-22", "N29b-24", "N38b-27"]
gates: []
plan_refs: ["§29.19", "Appendix M.28", "§38.6", "§29.20", "§29.22"]
---
## Background

A known answer is a case whose truth is fixed before the implementation exists: an analytic value, a counted
quantity, or a behaviour that follows from the design by argument. The record subsystem (Chronicle, crate
`frob-chronicle`) is tested against fourteen of them, numbered KA-707 to KA-720 in the project's known-answer
suite. Each has a *negative control*: a planted defect or a tempting shortcut that the test must catch. The
project's rule is that a gate nothing can fail validates nothing, so a known answer with no caught control does not
count as passed.

This bead is the registry of those fourteen rows, and of three more that a late subsection of the chapter (the
solver's kinetic-energy correction is not heat) states without a number in the suite. It carries each row in
full so that the mechanism beads can cite a number and a value, and it delivers the registry entries (data),
not the mechanisms. Tier names: G0 laws, G1
known answers, G4 chaos, G5 determinism; the site after the tier says where the test runs (`host` = a test binary
in exact or double-precision arithmetic; `referee` = the double-precision reference tier; `lockstep` = the
throughput tiers; `daemon` = the job daemon under the deterministic simulation runtime; `all tiers` = referee,
`batch-cpu`, CUDA and Metal).

## Scope

In: the fourteen numbered registry rows and the three appended rows (K1 to K3 below) as `[[ka]]` records in the
suite's one known-answer file, `crates/frob-conformance/registry/known_answers.toml` (schema and loader: N38a-12;
the same file as every other chapter's rows; this bead first named a file of its own,
`registries/known-answers/chronicle.toml`, which is not used); for each, the fixture identity, the truth, the
tolerance, the tier and sites, the negative control's text (`control.text`, from which the seeding tool of
N38b-05 issues the `NC-` records of `crates/frob-conformance/registry/controls.toml` and fills `control.nc`;
this bead writes no control record itself), the derivation file, and, as planning
provenance, the bead whose tests must cite the row; one derivation file per case (the fifteen listed below);
the request for a block of three numbers for K1 to K3 in `crates/frob-conformance/registry/allocations.toml`
(N38a-40). Out: the tests themselves (the mechanism and gate beads named in the last column); the counterexample
registry (`N29-CX`) and the refusal registry (`N29-REF`), which another unit writes.

## Technical approach

Row format: the `[[ka]]` record of N38a-12, field for field: `id`, `section` (where stated, with `owner` naming
the gate, proof obligation or invariant the row serves), `fixture` (one identity per physical case), `same_as`
(empty here), `case`, `truth` (with its `kind`), `tolerance` (`exact`, `rel`, `abs` or `to-set`), `tier` with
`sites` (the site names below; `all tiers` is written as the four sites `referee`, `batch-cpu`, `cuda`, `metal`),
`control` (`text` and the `nc` identifiers of its records in `controls.toml`), `derivation`
(`docs/derivations/<file>.md`, whose front matter lists the KA identifiers it serves), `label` (`DESIGN` for every
row here: the values are arithmetic or follow from the design by argument), `due` (M1, M2 or M3: the milestone
of the earliest carrier that must cite the row; M3 only for KA-719, whose carrier BD-264 is an M3 bead) and `printed` (empty: no recomputation disagrees with a printed value). The
"Carried by" column is planning provenance, not a field: the suite's lint that every row is cited works from the
KA identifiers that tests cite. The suite's rule that a row counts as passed only if its registered control was
caught in the same run (STD-TEST) applies to every row; this bead's planted record below checks that the runner
enforces it for these rows.

**Where stated and owner.** `section` is Appendix M.28's "Where stated", each with §29.19: §29.9 for KA-707 to
KA-710, §29.4 for KA-711, §29.12 for KA-712, §29.6 for KA-713, §29.7 for KA-714, §29.5 for KA-715, §29.8 for
KA-716 to KA-718, §29.11 for KA-719, §29.13 for KA-720; §29.22 for K1 to K3. `owner` is the gate whose register
text the row serves: GATE-CHRON-03 (the ledger and the pendulum) for KA-707 and KA-708; GATE-CHRON-08 (the
created-energy audit) for KA-709 and KA-710; GATE-CHRON-04 (the chunk protocol) for KA-711 and KA-712;
GATE-CHRON-07 (sampling and histograms) for KA-713 and KA-714; GATE-CHRON-05 (the empty intervention equals
replay) for KA-716; GATE-CHRON-06 for KA-717; GATE-CHRON-02 (teleport) for KA-718; GATE-CHRON-09 for KA-719;
GATE-CHRON-10 for KA-720; GATE-CHRON-14 for K1 to K3. KA-715 serves no gate of the register and its `owner` is
empty (its carrier BD-090 states it as the admitted `sum` reduction).

| KA | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Carried by |
|---|---|---|---|---|---|---|
| KA-707 | Energy ledger of a damped pendulum. Fixture (DESIGN numbers; the suite names the case without numbers): point mass 1 kg on a massless 1 m rod, g = 9.81 m/s², viscous joint damping 0.5 N·m·s/rad, released at rest from 60°, run 60 s | Equal to the hand calculation: Δ potential = −m·g·l·(1 − cos 60°) = −4.905 J; Δ kinetic = 0 (below 10⁻¹² J at 60 s); joint damping + numerical residual = 4.905 J; ledger residual below 10⁻⁹ relative (GATE-CHRON-03) | 10⁻⁹ relative | G1 · referee | A sign error seeded at one port is localized to that port | BD-092; N29b-01 (the ledger's edge cases) |
| KA-708 | Energy per task: design A succeeds once for 10 J and fails once for 20 J; design B succeeds twice for 12 J each | Mean over completed episodes: A 10 J, B 12 J. Total over all attempts divided by completions: A 30 J, B 12 J. The first figure alone ranks them wrongly | exact | G1 · host | A report that prints only the conditional mean | BD-092; N29b-01 (per-task exclusions) |
| KA-709 | Created energy: a body bouncing on a stiff penalty contact at a step near the stability limit; the same drop under the convex contact law at the default step | The bounce raises `energy-created` and `contact-active`; the convex law raises neither | exact (flags) | G1 · referee, lockstep | The audit switched off: the bouncing policy wins the comparison | N29b-02 (M1: the `energy-created` rule on the bounce's host-model sums, the convex drop on the referee); N29a-22 (M2: both flags on the lockstep tiers), with N29b-03 and N29b-04 (the contact-level monitors, the flags on the episode record) and N29b-06 (the comparison control on the batched tiers) |
| KA-710 | Cone excess: a block pushed along the diagonal of a pyramidal (four-sided) friction cone | Friction utilization ‖f_t‖/(μ·f_n) reaches √2 ≈ 1.41421; `cone-excess` is raised | to set (DESIGN: 10⁻³ relative on utilization) | G1 · lockstep | A circular-cone run must not be flagged | N29a-22; N29b-03 (the reference evaluator); N29b-06 |
| KA-711 | Chunk protocol | Two identical publishes yield one row and one receipt; a crash after commit returns the original receipt | exact | G4 · daemon | A late lease that writes; overlapping ranges that commit; a reset at tick zero that collides | BD-091, N29a-06; device rows BD-210, N29a-24 |
| KA-712 | Pagination: 10⁴ rows read in pages during concurrent appends | Each row visible at the pin appears exactly once | exact | G4 · daemon | A query ordered by timestamp alone repeats or skips equal-time samples | N29a-04, N29a-06 |
| KA-713 | Sampling of traced worlds by address | The traced set is identical under 100 slot assignments and shard counts | exact | G5 · all tiers | Arrival-order reservoir sampling gives different sets | N29a-12, N29a-16; BD-207 (the traced-world mask on devices) |
| KA-714 | Integer histogram | Counts identical under 100 permutations and shard splits, on every backend; integer bin counts 1, 1, 1 for the streams 1, 2, 3 and 3, 2, 1 | exact | G5 · all tiers | An order-dependent sketch: a compactor of capacity two that keeps the lower of each sorted pair gives medians 1 and 2 for the two orders | N29a-13, N29a-14, N29a-16; BD-207 (histograms on devices) |
| KA-715 | Downsampling: an impulse train summed over a window | Momentum transfer is conserved (the window sum equals the sum of the impulses) | to set (exact in quantized integers) | G1 · host | The same train averaged | BD-090 (another unit) |
| KA-716 | Fork | The empty intervention replays bit for bit; a setpoint change at tick t reuses the prefix to the snapshot before t | exact (digest) | G5 · all tiers | The 60 mm wheel's contact warm start grafted onto the 75 mm wheel (refused, FROB-REF-582) | BD-205 (another unit); BD-203 for the replay half |
| KA-717 | First divergence: traces that differ at ticks 1 and 5 and agree at ticks 2 to 4 | Tick 1 | exact | G1 · host | Midpoint search on the pointwise predicate "tick k differs" reports tick 5 | BD-206, N29a-21 |
| KA-718 | Teleport | A planted single-kernel divergence is named by step and stage | exact | G1 · all tiers | None stated in the suite. Control added here (DESIGN): a mutant localizer that always names the last state-writing stage (stage 9), and so cannot see the stage of a plant; it must be wrong, and caught, on every trial planted in another stage (a stage-6 plant reported as stage 9) | BD-204; N29a-25 (the conformance-only plant hook) |
| KA-719 | Tier correction (the estimator Zᵢ = mᵢ + Aᵢ·(Y_ref,i − mᵢ)/pᵢ, θ̂ = mean of Zᵢ) | With every address audited (pᵢ = 1) it equals the plain referee mean exactly; with a prediction off by a constant it is still unbiased | exact for the first; inside the interval for the second | G0 · host | Audit bits chosen after seeing fast-tier failures, without recorded probabilities | BD-264 (another unit); N29b-08 (the audit rounds) |
| KA-720 | Artifact graph | Every gate row resolves to evidence and traces | exact | G1 · host | A claim printed after its evidence was collected | BD-095, N29a-10, BD-209 |

**Rows appended from the late subsection on the solver's kinetic-energy correction.** The known-answer suite has
no rows for these three cases. K1 to K3 are this bead's labels for them; the rows are cited under provisional
tags (rule 5 of N38a-40) until their numbers are issued: this bead files a block
request (`registry = "ka"`, `count = 3`, `owner = "N29-KA"`, `section = "§29.22"`; nobody types `first`) and
stores each row under its provisional tag in `slug` (`chron14-rest-block`, `chron14-slide-block`,
`chron14-plastic-drop`: N38a-12's field for a late case's working tag, built from the fixture names the
carrying beads N29b-23 and N29b-24 already use). DESIGN correction: an earlier draft wrote
`id = "KA-pending-chron14-<n>"`, which is not a KA number and which the loader's identifier rule refuses. When
the allocator issues the block, the three tags are mapped to its numbers in table order (K1, K2, K3) in one
commit, in the registry, in the tests that cite them and in the derivation file; the tag stays in `slug`, and a
tag still cited elsewhere at the rows' due milestone (M1) fails N38a-40's check `provisional-tag-past-due`.
Common fixture parameters, all exactly representable in binary floating point: gravity 8 m/s² (a fixture
parameter), a 1 kg rigid block on a stationary horizontal support, steps of 1/32 s, 1/64 s and 1/2,000 s, 4 s of
simulated time. *Algorithmic exchange* is the kinetic energy a velocity-level contact solve removes to undo the
free velocity its own predictor added; *constitutive dissipation* is what the contact law itself dissipates
(friction, restitution loss) and is the only contact term that may heat a thermal state.

| Label | Case | Truth | Tolerance | Tier · site | Negative control that must be caught | Carried by |
|---|---|---|---|---|---|---|
| K1 | `rest-block`: the block rests on a frictionless support | Constitutive dissipation exactly 0 J and heat 0 at every rate. Algorithmic exchange on the constraint side −½·m·(g·h)² per step: −1 J/s at 32 Hz, −0.5 J/s at 64 Hz, −0.016 J/s at 2,000 Hz; the predictor side is equal and opposite, net 0 | exact for the zero (compared as bits); 10⁻⁹ J/s on the rates | G1 · referee (M1), `strict` and lockstep (M2) | A mutant that feeds algorithmic exchange into the film temperature: over 4 s the 1 J/K test node warms by 4 K at 32 Hz and 2 K at 64 Hz, a factor of 2 between rates, where the correct build gives 0 K at both | N29b-20, N29b-21, N29b-22 (mechanisms); N29b-23 (referee rows); N29b-24 (batch tiers) |
| K2 | `slide-block`: the same block slides at a steady 1 m/s on μ = 0.5 under a constant 4 N push | Sliding friction heat μ·m·g·v = 4 W at every rate (16 J over 4 s) | 0.1% | G1 · referee (M1), `strict` and lockstep (M2) | The same mutant: the node's temperature rise then differs between 32 and 64 Hz | N29b-20, N29b-22; N29b-23; N29b-24 |
| K3 | `plastic-drop`: the block starts in contact with a normal approach speed of exactly 2 m/s, restitution 0 | Impact dissipation ½·m·(1 − e²)·v_n² = 2 J, booked once, in the interval of the located event; algorithmic exchange 0 on the event | 10⁻⁹ J | G1 · referee (M1), `strict` (M2; lockstep tiers do not locate events and report the impact as a fidelity row) | The same mutant; and an impact loss booked in more than one interval | N29b-20, N29b-21, N29b-22; N29b-23; N29b-24 |

Derivation files (one page each under `docs/derivations/`, written by this bead so that every row meets rule 1 of
N38a-12 from the start; a carrying bead may extend its file and never contradicts it):
`chronicle-pendulum-ledger.md` (KA-707), `chronicle-energy-per-task.md` (KA-708),
`chronicle-pyramid-diagonal.md` (KA-710: a pyramid with |f_x| ≤ μ·f_n and |f_y| ≤ μ·f_n admits
(μ·f_n, μ·f_n), whose norm is √2·μ·f_n), `chronicle-k-smallest-uniform.md` (KA-713),
`chronicle-compactor.md` (KA-714: the stream 1, 2, 3 compacts to {1 with weight 2, 3}, median 1; the stream
3, 2, 1 compacts to {2 with weight 2, 1}, median 2), `chronicle-first-divergence.md` (KA-717: the chained
digest c_k = H(c_{k−1}, d_k) covers ticks 0 to k, so "c_k differs" is monotone in k and bisection on it finds
tick 1; the pointwise predicate "tick k differs" is true at ticks 1 and 5 and false at 2 to 4, so a midpoint
search that probes tick 3 or 4 reports tick 5; the file `first-divergence-monotonicity.md` is N27a-09's, for
KA-677, KA-678 and PO-AUTOPSY-02, and this bead does not write it),
`chronicle-algorithmic-exchange.md` (K1 to K3: ½·1·(8/32)²·32 = 1 J/s and ½·1·(8/64)²·64 = 0.5 J/s for the
resting block; 0.5·1·8·1 = 4 W for the sliding block; ½·1·2² = 2 J for the plastic drop; N29b-20 cites it); and,
for the rows whose truth is an argument rather than a number: `chronicle-penalty-bounce.md` (KA-709: a penalty
spring integrated explicitly near its stability limit returns more energy than it stored; the convex law's
dissipation is non-negative by construction), `chronicle-chunk-protocol.md` (KA-711: one row per (key, content)
by the unique key and the receipt lookup), `chronicle-pinned-pagination.md` (KA-712: the total order by commit
sequence and the pin make the page sequence a partition of the rows visible at the pin),
`chronicle-impulse-window.md` (KA-715: a window sum of quantized impulses is an integer sum, exact in any
grouping), `chronicle-fork-prefix.md` (KA-716: the empty intervention changes no input of the step, so the
digests are equal), `chronicle-teleport-plant.md` (KA-718: a plant in stage s changes the stage-s digest and no
earlier one, so the first differing stage digest names s; the mutant localizer names stage 9, the last
state-writing stage, whatever s is, and is therefore wrong on every plant outside stage 9), `chronicle-tier-correction.md` (KA-719: with
pᵢ = 1 every Aᵢ = 1 and Zᵢ = Y_ref,i; with a constant offset in mᵢ, E[Zᵢ] = Y_ref,i because E[Aᵢ] = pᵢ) and
`chronicle-artifact-graph.md` (KA-720: every gate row's claim resolves by the closure walk; a tombstoned node
makes the claim `unavailable`).

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers.toml` holds exactly fourteen numbered rows of this chapter,
      KA-707 to KA-720, and the three appended rows K1 to K3 (under their pending tags until N38a-40 issues the
      block, then under its numbers), each a `[[ka]]` record with every field of N38a-12; the loader of N38a-12
      validates the file, and the block request for three numbers is present in `allocations.toml`.
- [ ] Every row carries its control text; after N38b-05's seeding tool is re-run each row's `control.nc`
      resolves to records of `controls.toml`; the row whose control is `none stated` in the suite (KA-718)
      carries the control added above as its text, labelled DESIGN, and is seeded like the others.
- [ ] The fifteen derivation files exist; each lists in its front matter the KA identifiers it serves and
      reproduces its truth by hand arithmetic or by the argument stated; the loader computes every row
      `eligible` except KA-710 (tolerance `to-set` until frozen with the first recorded run, rule 2 of N38a-12).
      KA-715, also printed `to set`, is eligible because its derivation file states the tolerance (exact in
      quantized integers) with its reason, which is how rule 2 closes a `to set` row; the row's `printed` keeps
      "to set".
- [ ] Once the carrying bead of a row is built, a test of that bead cites the row's identifier (the suite's
      citation lint); rows whose carrying bead belongs to a later milestone are listed `uncited` with their
      `due`, never as passed.
- [ ] A row marked `pass` without a caught control in the same run is refused by the runner (planted).
- [ ] Rows owned by another unit's bead (KA-715, KA-716 fork half, KA-719) point at that bead's test and are not
      duplicated.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test ka_chronicle` (`crates/frob-conformance/tests/ka_chronicle.rs`):
- `registry_has_fourteen_rows`: the chapter's numbered ids in the suite file are exactly KA-707..KA-720, no gaps,
  no duplicates; the three appended rows are present and carry either an assigned KA number or the pending tag.
- `rows_validate_with_suite_loader`: the N38a-12 loader accepts the file; every row has `label`, `due`, `sites`
  and a `derivation` whose front matter lists the row.
- `pending_tags_have_block_request`: each pending tag maps to the block request owned by N29-KA in
  `allocations.toml`; a pending tag without a request is refused (planted).
- `k1_k3_arithmetic`: ½·1·(8/32)²·32 = 1 and ½·1·(8/64)²·64 = 0.5 exactly in binary; 0.5·1·8·1 = 4; ½·1·2² = 2;
  the mutant's rise over 4 s is 4 K at 32 Hz and 2 K at 64 Hz.
- `every_row_has_control`: each row's `control.nc` resolves to records of `controls.toml`; each row cited by a
  built test resolves to that test (citation lint), the others are listed `uncited` with their `due`.
- `ka708_arithmetic`: (10)/1 = 10 and (10 + 20)/1 = 30; (12 + 12)/2 = 12 both ways.
- `ka710_arithmetic`: ‖(1, 1)‖ = √2; utilization 1.41421356 within 10⁻⁸ of √2.
- `ka714_compactor`: the capacity-two compactor on 1, 2, 3 and on 3, 2, 1 yields medians 1 and 2; integer bins
  yield counts (1, 1, 1) both ways.
- `ka707_hand_value`: 1 × 9.81 × 1 × (1 − 0.5) = 4.905.
- `pass_without_control_refused` (planted): a result record with `verdict = pass` and an empty `controls_caught`
  list is rejected.

### End-to-end script and logging

`scripts/e2e/chronicle_known_answers.sh` (header `bead=N29-KA milestone=M1 labels=chronicle,registry gates=`)
first runs `cargo test -p frob-conformance --test registry_check` and `--test ka_chronicle`, then runs the rows
the way a gate recorder does, one invocation per row in the tier target its tier names (there is no `frob
gauntlet` command, ADR-GAUNT-01): `FROB_TEST_SEED=<s> cargo test -p frob-conformance --test g1_known_answers --
KA-707` for the G1 rows (KA-707 to KA-710, KA-715, KA-717, KA-718, KA-720 and K1 to K3), `--test g4_chaos` for
KA-711 and KA-712, `--test g5_determinism` for KA-713, KA-714 and KA-716, `--test g0_laws` for KA-719; and the
same selection with `--features gauntlet-mutations`, in which each row's registered control must be reported
caught. It reads the hash-named receipt each run writes, read-only (the three appended rows are selected by
their `slug` until they have numbers), records no gate, and asserts that each of the seventeen rows that has a
runner entry reports `pass` with its control caught, and that every other row is reported `unbound` or `not due`
with its carrying bead and `due`. The script never waits on a carrier: at this bead's close no row is bound yet,
and the script passes on the registry checks alone; it fails only on `fail` or `control-missed`. It sources the STD-E2E
helper and writes `target/e2e-logs/chronicle_known_answers/<utc>.ndjson` with a transcript: one `assert` line
per row with the measured value, the truth, the tolerance, the tier and site, the control's outcome and the
evidence block; every line carries `ts`, `script`, `bead`, `step`, `event`, and on `cmd` lines the exact
command, exit code and wall time; on a failure the first differing value with both numbers and the receipt
path; a `summary` line with counts passed, failed and not run, the digest of `known_answers.toml` and
`gates: []`.

### Known answers and negative controls

This bead is the registry; its own negative control is the planted `pass` without a caught control. Determinism:
the registry file is canonical TOML and its digest is recorded in the gate log, so a silent edit of a truth
changes the digest.

## Considerations

- KA-707's numbers are a DESIGN choice of this registry (the suite names the case only); a change of fixture
  constants after the first recorded pass changes the fixture's identity and must be recorded as such.
- KA-710's tolerance is "to set": the value above is the proposal and is frozen with the first recorded run.
- Rows due at M2 (KA-710, -713, -714, -716 to -718) and KA-719 (due M3, with the tier-correction ledger BD-264)
  are `not run` at the M1 exit by design. KA-709 is split: its referee half (N29b-02: the `energy-created` rule
  on the bounce's host-model sums and the convex drop on the referee) runs at M1, and the row passes only when
  N29a-22 adds both flags on the lockstep tiers at M2; the referee's commit check refuses to commit the bounce's
  created energy, so the flagged bounce is a lockstep result. K1 to K3 run on the referee at M1 and gain their
  `strict` and lockstep sites at M2.
- K1 to K3 come from a subsection added after the suite was numbered; the gate they serve (GATE-CHRON-14) is
  likewise listed only among the late gates. Their truths were first stated in N29b-20 to N29b-23 and in the
  counterexample registry N29-CX (row C1), which filed them here.

## References

§29.19; Appendix M.28 (KA-707 to KA-720); §29.22 (the three appended rows); §38.6; GATE-CHRON-03, -04, -06, -07,
-08, -10, -14.
