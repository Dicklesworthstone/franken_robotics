---
key: "N41-OPEN2"
title: "registries/open.toml, part 2: open questions of Kinesis, Phylum and Traction (16 rows)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "kinesis", "phylum", "traction", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-03"]
needs: []
related: ["N41-OPEN3"]
gates: []
plan_refs: ["§42.3", "§42.2", "§16.18", "§17.20", "§18.18"]
---
## Background

Second part of the open-question register (rule and schema in N41-03; how to read a row in N41-OPEN1). The three
subsystems here are the body of the simulator:

- **Kinesis**: dynamics and stepping (the referee tier with its transactional, event-located step; the batched
  step; the trust ladder between them);
- **Phylum**: many different robots in one batch (parameter variants, model variants inside padded size buckets,
  the head-to-head experiment against per-family templates);
- **Traction**: contact (distance fields, contact laws, the friction cone with a spin dimension, witness
  migration, film and soil cards).

Most of these questions are numbers with a safe starting value, decided by a measurement the program takes anyway
(the M2 profile, a fixture, a pilot). Two need the owner: the compute envelope for the head-to-head, and which
measured film and soil tables enter the catalogue.

## Scope

In: the sixteen rows below as records of `registries/open.toml`. Out: the schema and loader (N41-03); the
decisions (the beads in the last column).

## Technical approach

| ID | Question | Fail-closed default | Owner | Blocks, or decided by | Deadline | Decided in |
|---|---|---|---|---|---|---|
| OPEN-KIN-1 | The default physics step and Newton iteration budget | 2 ms; 4 iterations with warm start | Kinesis lead | The fidelity–throughput frontier at M2 | M2 | N16a-16 |
| OPEN-KIN-2 | Whether sleeping and islands are worth their complexity for single-robot worlds on lockstep tiers | No; islands are always formed under a transactional profile | Kinesis lead | The M2 profile | M2 | N16c-07 |
| OPEN-KIN-3 | Final contact and row capacities per workload | The capacity table of the batched step (quadruped C = 32, R = 160; humanoid C = 64, R = 320; and the other reference workloads); overflow is flagged and the caller chooses refuse, reset or continue-flagged | Kinesis lead | GATE-KIN-07 | M2 | N16a-15 |
| OPEN-KIN-4 | Which law the `strict` profile runs by default once the hard-contact law is admitted | The compliant default law with located events; the hard-contact law as a named variant | Kinesis lead | Nothing; decided with GATE-TRAC-09 | M5 | N16c-09 |
| OPEN-KIN-5 | L_max (event bracketing depth), E_max (events per physics interval) and the retry budget of the transactional step | 16, 64 and 3; exhaustion refuses | Kinesis lead | The refusal rate on the standard battery at M1 | M1 | N16a-09 |
| OPEN-KIN-6 | The created-energy bound per law and step | 1% of actuator work per episode | Kinesis lead | Re-baselined once with GATE-FID-01 | M2 | N16c-11 |
| OPEN-PHYLUM-1 | The registered default bucket set and the padding-waste limit | Buckets {12, 18, 24, 36, 48, 64} degrees of freedom and 25% waste; beyond that the planner refuses (`FROB-REF-741`) | Phylum lead | The M1 pilot of padding overhead and bucket count | M2 | N17b-11 |
| OPEN-PHYLUM-2 | Run length per model, 16 or 32 worlds | 32 on CUDA; the device's reported execution width on Metal | Phylum lead | The M2 profile | M2 | N17b-11 |
| OPEN-PHYLUM-3 | The compute envelope for the head-to-head and for pretraining populations | The one-device, one-hour rule, which admits the pilot and not the full experiment | Owner | GATE-PHYLUM-03 | M2 | N17b-07 |
| OPEN-PHYLUM-4 | Which incumbents and versions fill arms C and D of the head-to-head | None; the arms report `not run` | Gauntlet lead | Frozen with the rival audit of the ten-fold test before any run | M2 | N17b-08 |
| OPEN-PHYLUM-5 | Whether contact capacity needs its own bucket axis | No; one axis on degrees of freedom, with capacities from the largest member | Phylum lead | Overflow and padding statistics at M2 | M2 | N17b-11 |
| OPEN-TRAC-1 | Dense bricks or a sparse brick tree for interior scenes at 1 cm | Dense bricks with a coarse outer level | Traction lead | Measured memory at M3 | M3 | N18b-37 |
| OPEN-TRAC-2 | The source of the effective patch radius r_eff: sample geometry, a pressure assumption per material, or a card field | 2a/3 from sample geometry (uniform pressure on a disk of radius a), overridden by a card | Traction lead | GATE-TRAC-04 | M2 | N18a-27 |
| OPEN-TRAC-3 | Provenance for film and soil cards: which measured tables are admitted | Generic ranges labelled `assumed`, which keep every result `Estimated` | Project lead; asked of the owner at M3, one milestone before its M4 deadline, with OPEN-SINEW-1 and OPEN-SENS-4 | Claims of realism for wet floors and soft ground, not validity | M4 | N18b-36 |
| OPEN-TRAC-4 | The admission bound on scaled stiffness κ·u for `f32` tiers | 0.05 | Traction lead | GATE-TRAC-10 | M2 | N18bp4-01 |
| OPEN-TRAC-5 | Witness-migration tolerances: δ_w and the largest normal rotation transported | The sample radius and 30°; beyond them, reset with an event | Traction lead | The sill fixture at M1 | M1 | N18b-38 |

Status at entry: sixteen `open`. Related questions elsewhere: the bucket set and the capacities of the two
reference workloads are also asked as OPEN-SPARK-3 (part 3), and the M1 pilot may revise the set for both chapters
at once; if the default physics step moves from 2 ms, every performance row is restated with its new decimation
before any gate cites it (OPEN-PERF-4, part 7). What the owner's default for OPEN-PHYLUM-3 costs: KILL-PHYLUM is
decided on the pilot alone.

OPEN-KIN-6 is the one row of this part whose decision is a table. N16c-11 records the bound per law and step in
`docs/adr/ADR-KIN-OPEN-6.md`, sets the row's `adr` to that file and writes the BLAKE3 hash of the bound table into
the row's optional field `decision_digest` (N41-03's record), where BD-212 reads it for the one re-baseline of
GATE-FID-01. The digest is write-once: N41-03's loader rule 9 locks the first digest a decided row carries in
`registries/open.lock`, and a second, different hash on the decided row fails the register's load. This part
enters the row `open` with `adr` empty and `decision_digest` empty.

## Acceptance criteria

- [ ] `registries/open.toml` contains the sixteen records with no empty required field.
- [ ] Every row names a decision or experiment bead that exists in the tracker. For OPEN-KIN-1, -3, -5 and
      OPEN-TRAC-1 two units had drafted a decision bead; the chapter groups kept one of each pair (N16a-16, N16a-15,
      N16a-09, N18b-37) and marked the other superseded, and the record names the kept bead.
- [ ] OPEN-PHYLUM-1 and OPEN-SPARK-3 cross-reference each other, and the loader refuses a state in which their
      recorded bucket sets differ.
- [ ] Deadlines agree with the milestone of the gate or measurement each row names.

## Test plan

### Unit tests
`crates/frob-conformance/tests/open_part2.rs`: `sixteen_rows_load`; `ids_match_expected_set` (KIN-1 to -6,
PHYLUM-1 to -5, TRAC-1 to -5); `defaults_quote_known_numbers` (2 ms and 4; 16, 64, 3; 1%; {12, 18, 24, 36, 48, 64}
and 25%; 0.05; 30°), each compared with the constant its owning crate's contract declares once that contract
exists; `bucket_sets_agree_across_questions`; `decision_beads_resolve`; `owner_rows` (OPEN-PHYLUM-3 has owner
"Owner").

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04).

### Known answers and negative controls
Known answers: 16 rows; the default values above. Negative controls: OPEN-KIN-1 marked decided with no frontier
record; a default changed in the registry without an ADR (the loader compares against the last recorded state).

## Considerations

- Defaults here are engineering constants that enter identities (the step plan's identity includes the step, the
  iteration count and capacities), so changing one is not free: results before and after are different
  identities. The registry records the default so that a change is a decision with an ADR, never a drift.
- Four questions had decision beads in two drafting units. The chapter groups kept one of each pair and marked the
  other superseded; the register points at the kept bead, and a superseded key in a row is a loader error.

## References

§42.3 (rows for Kinesis, Phylum, Traction), §42.2 (OPEN-PHYLUM-3 and the grouped data-source row), §16.6
(capacities), §16.5 (the transactional step's limits), §17.5 (buckets), §18.4 (witness migration), §18.12.
