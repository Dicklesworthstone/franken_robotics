---
key: "N41-OPEN3"
title: "registries/open.toml, part 3: open questions of Habitat, Sensorium and Spark (19 rows)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "habitat", "sensorium", "spark", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-03"]
needs: ["§19 to §21: the decision and experiment beads named in the last column are owned by the chapter units"]
related: ["N41-OPEN2", "N41-OPEN4", "N21c-34", "N21c-35", "N21c-38", "N19c-30", "N20b-26", "N40-33", "N40-35"]
gates: []
plan_refs: ["§42.3", "§42.2", "§19.20", "§20.22", "§21.25"]
---
## Background

Third part of the open-question register (rule and schema in N41-03; how to read a row in N41-OPEN1). The
subsystems:

- **Habitat**: scenes (distance-field bricks with analytic overlays, interior archetypes with hazard lists,
  living-world proxies, clearance, retrofits, ingestion of real places);
- **Sensorium**: sensors with clocks, latency and noise drawn by address;
- **Spark**: the device layer (one kernel source on CPU, CUDA and Metal; GPU backends in annex workspaces;
  placement; the hardware fingerprint).

This part holds the two questions the owner must answer first, at M0. OPEN-SPARK-1: ratify the architecture
decision that GPU backends live in annex workspaces with registered foreign-function capsules, outside the
Franken-only core; until then GPU results are labelled exploratory, no Performance or Parity claim rests on a
device result, and M2 exits in its no-GPU variant. OPEN-SPARK-2: the compute envelope classes (E0 the default of
16 GiB, 8 cores, one hour, one device; E1 a named gate measurement; E2 a design campaign; E3 pretraining of one
family policy); until an envelope is approved the throughput and scaling gates, the head-to-head and every
campaign cannot run as specified.

## Scope

In: the nineteen rows below as records of `registries/open.toml`. Out: the schema and loader (N41-03); the
decisions (the beads in the last column).

## Technical approach

| ID | Question | Fail-closed default | Owner | Blocks, or decided by | Deadline | Decided in |
|---|---|---|---|---|---|---|
| OPEN-HAB-1 | Which published building and workplace norms supply the dimension distributions | Generic ranges labelled `assumed`; realism claims refused | Project lead; asked of the owner at M3 | GATE-HAB-02 for claims of realism, not for validity | M3 | N19c-30 |
| OPEN-HAB-2 | Whether bricks carry a one-sample apron, which removes neighbour lookups and costs 42% more memory | No apron; up to eight directory lookups for a query in a boundary cell | Spark lead | The M2 profile; blocks nothing | M2 | N19c-31 |
| OPEN-HAB-3 | Default spacings and truncation distance (2 cm fine and 16 cm coarse, truncation 8 cm) against 1 cm interiors | As stated | Habitat lead | Measured memory and the fidelity ledger at M3 | M3 | N19c-32 |
| OPEN-HAB-4 | Sources for retrofit prices and legality constraints | Prices `assumed`; constraints marked unchecked in every report | Project lead; asked of the owner at M5 | Any claim about cost in a published report | M5 | N19b-43 |
| OPEN-HAB-5 | Which reconstruction package and version is the import contract | The walkthrough twin package; the drone route when it publishes geometry | Habitat lead; asked of the owner at M5 | GATE-HAB-03 | M5 | N19c-09 |
| OPEN-HAB-6 | Whether contact with unknown space degrades the world or refuses the run | Degrade, exclude from metrics, count | Habitat lead | M5, with the first real place | M5 | N19c-10 |
| OPEN-HAB-7 | Whether scene breadth in training improves held-out success | The world bank serves evaluation | Habitat lead | The breadth experiment (16, 160 and 1,600 distinct homes at a fixed step budget) | M3 | N19c-26 |
| OPEN-HAB-8 | Sources for film roughness, carried thickness and transfer fraction | `assumed`; outputs `Estimated`; Reality `no-claim` | Habitat lead | Any Reality claim about slip on films | M5 | N19b-22 |
| OPEN-SENS-1 | The default policy for clock events inside a step on throughput tiers | `hold`, with recorded lateness | Kinesis and Sensorium leads | GATE-SENS-04, for the choice of default only | M2 | N20b-23 (also N20a-12) |
| OPEN-SENS-2 | The limit on the clock lattice and the menu of declared rationals for common device rates | Lattice denominator Q at most 2⁴⁰ | Sensorium lead | M2 | M2 | N20b-24 |
| OPEN-SENS-3 | Whether observation age is a default policy input | Always in telemetry; given to a policy only when its schema declares it | Sensorium lead | M3, with the first tasks that randomize latency | M3 | N20b-25 |
| OPEN-SENS-4 | Which sensor and optical cards ship with datasheet or measured fields | All `assumed`; Reality `no-claim` (`FROB-REF-447`) | Project lead; asked of the owner at M4 | Any Reality claim that depends on sensing | M4 | N20b-26 (also N20a-32) |
| OPEN-SENS-5 | Whether glass reports the stronger return or draws between returns | The stronger return, deterministic | Sensorium lead | A measured card for one depth camera and one lidar | M5 | N20b-27 |
| OPEN-SPARK-1 | Owner ratification of the architecture decision that GPU backends live in annex workspaces with registered foreign-function capsules, outside the Franken-only core | An exploratory label on GPU results | Owner | The M2 exit | Asked at M0; due M2 | N21c-34 |
| OPEN-SPARK-2 | The compute envelope for gate measurements and training campaigns (classes E0 to E3) | E0: FrankenSim's machine rule for CPU work (16 GiB, 8 cores, one hour), and one device for one hour per GPU job | Owner | GATE-SPARK-03, -04, -20, -21 as specified | Asked at M0; blocks from M1 | N21c-35 |
| OPEN-SPARK-3 | The bucket set and the capacities of each reference workload | Links {12, 18, 24, 36, 48, 64}, the same set as the mixed-batch chapter, and the capacities of the performance chapter's reference rows; the M1 pilot may revise the set for both chapters at once | Spark lead | GATE-SPARK-10; the M2 profile | M2 | N21b-18 (also N21c-36) |
| OPEN-SPARK-4 | Whether the pinned `cudarc` exposes graph capture and the asynchronous pool calls, and if not whether the capsule may declare them | Preallocated transient buffers and per-launch submission, with the throughput cost disclosed | Spark lead | The admission gate INT-CUDA-01 | M0 | N21b-03 (also N21c-37) |
| OPEN-SPARK-5 | What Spark does if the Metal gateway admission INT-FT-01 fails, or until the upstream work item FD-FT-01 lands; in the owner's words: whether a Metal binding of our own may be built before that item lands | Stay inside the gateway's limits and accept KILL-METAL's fallback; no Metal binding of our own without an owner decision | Owner | KILL-METAL | M2 | N21c-38 (also N21b-09) |
| OPEN-SPARK-6 | Whether the two RTX 4090 of `trj` allow device-to-device access | Host-staged transfers of small statistics only | Spark lead | The hardware fingerprint (GATE-SPARK-07) | M0 | N21c-03 |

Status at entry: nineteen `open`. What the owner defaults cost, as recorded with each: OPEN-SPARK-1, no Performance
or Parity claim rests on a device result and M2 exits in its no-GPU variant; OPEN-SPARK-2, the throughput and
scaling gates, the head-to-head and every campaign cannot run as specified; OPEN-SPARK-5, the Mac may be a
development and replay tier only; OPEN-HAB-1 and OPEN-HAB-4 (with OPEN-SKEL-2 and OPEN-FOUNDRY-2), no claim about
realism or cost in a published report and `--strict-data` refuses a cost verdict; OPEN-SENS-4 (with OPEN-SINEW-1
and OPEN-TRAC-3), every result that depends on those cards stays `Estimated` and realism and Reality claims are
refused.

## Acceptance criteria

- [ ] `registries/open.toml` contains the nineteen records with no empty required field.
- [ ] OPEN-SPARK-1 and OPEN-SPARK-2 have owner "Owner", are asked at M0, and appear first in the M0 listing of the
      exit runner with their defaults and the cost of each default.
- [ ] Every row names a decision or experiment bead that exists; duplicate decision beads across units are listed
      (OPEN-SENS-1, OPEN-SENS-4, OPEN-SPARK-3, OPEN-SPARK-4, OPEN-SPARK-5).
- [ ] OPEN-SPARK-3's bucket set equals OPEN-PHYLUM-1's (loader cross-check).

## Test plan

### Unit tests
`crates/frob-conformance/tests/open_part3.rs`: `nineteen_rows_load`; `ids_match_expected_set` (HAB-1 to -8, SENS-1
to -5, SPARK-1 to -6); `owner_rows` (SPARK-1, SPARK-2, SPARK-5); `m0_listing_starts_with_spark_1_and_2`;
`decision_beads_resolve`; `bucket_sets_agree_across_questions`; `defaults_quote_known_numbers` (42%; 2 cm, 16 cm,
8 cm; 2⁴⁰).

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04); the M0 walk asserts that leaving OPEN-SPARK-1 unanswered
produces, at the M2 exit, the default ADR and the expectation of the no-GPU variant.

### Known answers and negative controls
Known answers: 19 rows; three owner rows. Negative controls: a device Performance claim recorded while OPEN-SPARK-1
is at its default (refused by the owner-calendar check); OPEN-SPARK-6 marked decided with no fingerprint record.

## Considerations

- OPEN-SPARK-1 is a doctrine question (the suite's core is Franken-only with no foreign-function code; GPU drivers
  cannot be reached without it). The annex design keeps the core clean whichever way it is answered; the answer
  decides only what device results may be called.
- OPEN-HAB-2 blocks nothing and is still worth deciding by measurement, because it moves 42% of scene memory.

## References

§42.3 (rows for Habitat, Sensorium, Spark), §42.2 (owner rows), §42.4 (the six that matter most), §36.14 (envelope
classes), §21.2 (the annex decision).
