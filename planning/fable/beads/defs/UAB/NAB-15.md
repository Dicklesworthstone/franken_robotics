---
key: "NAB-15"
title: "registries/sources.toml: the 25 plan-time checks and the reproduction archive pinned by commit and digest"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "delivery", "solid", "registry", "pilot"]
parent: "WP-37"
deps: ["NAB-09"]
needs: []
related: ["NAB-33", "NAB-37", "NAB-38", "NAB-48"]
gates: ["GATE-GAUNT-08"]
plan_refs: ["Appendix H.5", "Appendix K.3", "§0.6", "§25.22", "§38.5"]
---
## Background

Before the specification of FrankenRobotics was written, a number of small programs were run to test ideas at
paper scale: exact classifiers of whether a body can carry a load in a stance, a docking problem solved in
closed form, a probe that tells friction laws apart, an enumeration of every outcome of a fixed-sample
decision rule, a screen of 65,536 boundary cases on three GPUs, condition numbers of contact matrices. The
specification cites their results as FACT. Each is a fact about **a small program on a small model**. None used
FrankenSim dynamics, FrankenSim contact or a real sensor pipeline; the runs on GPUs executed one scalar
decision kernel. No physical claim follows from any of them: the material laws were artificial, the loads
stipulated, the plants scalar or planar.

This bead enters the 25 checks into the source register (`registries/sources.toml`, schema `frob.sources/1`,
NAB-09) as sources of kind `plan-check`, and pins the archive that holds their programs and outputs. A
plan-time check may be cited by a gate only after it has been re-run here and reproduced (GATE-GAUNT-08: the
plan-time pilots re-run at M0 reproduce every count exactly and every timing ratio inside its recorded band);
the re-runs are owned by the pilot re-run beads and their status is held by the plan-time pilot ledger
(N38a-10, records in the pilot register `pilots.toml`); this bead gives the checks stable citable ids and the
recorded figures of record.

What the checks do **not** establish is recorded with the same weight as what they do. Every strong baseline
tied or won: a lifted linear program tied the projection, a scenario-tree program tied the docking family, a
dependency graph tied the semantic receipt, and quantifier elimination produced the composed-solver answers
directly for 8 of 12 queries. The checks support mechanisms being *sound* on small cases; they support no
speed advantage and no claim about a robot. Tier: Solid (registration); the checks themselves are pilots.

## Scope

In: 25 `[[source]]` records of kind `plan-check` (`SRC-CHK-01` to `SRC-CHK-25`, the ids `PC-01` to `PC-25` of
the table), which are the citable identity and the figures of record of each check; the archive record with its
revisions and digests; the arithmetic identities over the recorded figures (asserted here and nowhere else); the
stored text of the limits.

Out, and where it is:
- the re-run status of each check, its re-run owner, the expected values a re-run is compared with, the timing
  bands and the reproduction rule: the plan-time pilot ledger N38a-10 (twenty records `PILOT-PT-01` to `-20`
  and four supplementary ones, `PILOT-PT-S1` to `-S4`, in `pilots.toml`, each listing in `covers` the
  `SRC-CHK` ids it governs; S4 lists none, below). That bead depends on this one and carries the test that every
  figure held by both registers is equal;
- the held-command device pilot of the dynamics chapter (ledger record `PILOT-PT-S4`, re-run by N16c-45 at M2
  with GATE-KIN-13): it has no source record here, by the rule of step 2a;
- re-running the checks: BD-041 docking, BD-042 stance, BD-043 the nine mission-query groups and the admission
  fixtures, BD-030 the boundary screen, N38a-11 the fixed-n enumeration, the draft-rule power, the wheel loop
  and the conditioning rows (PC-20, PC-21), N00-35 the CPU rate and the Metal query (PC-22, PC-23);
- the scan that refuses a citation of a check that is not `reproduced`: the citation lint NAB-10.

## Technical approach

**1. The archive record** (`id = "SRC-REC-05"`, `kind = "planning-record"`).

- As recorded at plan time: the single `.zip` file under `planning/` in this repository, 241,568 bytes,
  SHA-256 `0326e89d76b88a2aa7d198d53556f80ab131fe5a902113dd37f7f32e93b09a4f`. Its runner replays the
  standard-library checks with `python3 -B verify.py` and the full set with `--all`; the mission-query chapter's
  pilot table gives the inner paths of each check.
- FACT (observed while drafting this bead, 2026-11-22, by `shasum -a 256` and `git log` on the file): the file
  now in the working tree is **275,178 bytes** with SHA-256
  `9620da42ec82c175209bbcea21ec71d376da31b687f4f2729e36c20476cd7980` and 179 entries. The repository history
  holds four revisions of the archive:

| Commit | Bytes | SHA-256 | Entries |
|---|---|---|---|
| `7041f51` | 230,188 | `291471cc53fa462bf3d786519f582213eb8cd06b9ace50fbf79603f9a01689c1` | not counted |
| `a635b91` | 241,568 | `0326e89d76b88a2aa7d198d53556f80ab131fe5a902113dd37f7f32e93b09a4f` | 161 |
| `4d99e2a` | 258,254 | `68cb587f8d02c695bc22a8538d5d2b9e6eed8f19471e0b282ddc417a6c64023d` | not counted |
| `47e8e89` | 275,178 | `9620da42ec82c175209bbcea21ec71d376da31b687f4f2729e36c20476cd7980` | 179 |

  The digest the planning record cites is the archive at commit `a635b91`; the working tree holds a later
  revision with 18 more files (follow-up checks). The record therefore stores `revisions = [...]` with all four
  rows and `cited_revision = "a635b91"`.
- Rule (DESIGN): a re-run names the revision it ran (commit and digest). A check's recorded value is compared
  with the output of the **cited** revision first; a re-run on a later revision is a second record. If the two
  revisions give different outputs for one check, that check is `differs` in the ledger until explained. The
  archive is never edited here; its top level holds `README.md`, `SHA256SUMS`, `verify.py`,
  `verify_negative.py` and `check_additions.py`, with the checks in the directories `mor/`, `pusher/`, `sill/`,
  `study/`, `information/`, `native/` and `followups/`, and two earlier packagings kept beside them,
`native-edition/` and `original-package/`, each with its own `README.md`, `SHA256SUMS` and `verify.py` (FACT:
listing of the working-tree revision, 179 entries). The first
  step of every re-run is the runner's own inventory and SHA-256 check inside the extracted revision.

**2. One record per check.** `kind = "plan-check"`, `locator = "SRC-REC-05:<inner path>"`, one `reading` with
`mark = "P"` (the output was produced by a program, not read from a page), the recorded `figures`, each with
`how = "executed"` (NAB-09's figure field), typed exactly (counts as integers, fractions as rationals, timings in microseconds), `cited_in` (the
chapters that cite it), and `scripts_retained` (false for the rows whose scripts were not kept: PC-20 and PC-21
on conditioning, PC-22 the CPU incumbent rate and PC-23 the Metal device query, which the review log counts as
three checks and which are re-run from scratch).
`reread_before_gate` is not typed by hand for these records: NAB-09's loader derives it from the re-run status
it is given (NAB-09 step 2), as true unless that status is `reproduced`. The citation lint (NAB-10) supplies the
plan-time pilot ledger's `pilots::status_of_source(id)` (N38a-10, which reads this register and so is not a
dependency of this bead); with no status supplied, and for a check that no ledger record covers, the record
stays uncitable by a gate (fail closed).

**2a. A plan-time result with no recorded source (one case).** The dynamics chapter states as "FACT (a recorded
pilot, Appendix K; to be re-run before a gate cites it)" that a rational referee and a native single-precision
kernel on the Metal device and on each RTX 4090 agreed on all 192 decisions of the first three rows of the
held-command fixture, with device read-backs equal byte for byte to independently reconstructed single-precision
terminal states. FACT (read at polishing): that pilot is in none of the places that record plan-time checks: not
in the 25-row table below, not in the nine-row review table, and not in the archive `SRC-REC-05`. DESIGN: no
`SRC-CHK` id is allocated for it, because a source record asserts a place where the result was recorded and none
exists; the claim is recorded as having no source. Its one citable identity is the ledger record `PILOT-PT-S4`
(N38a-10, `covers = []`, `bundle_receipt = "none: not in the plan-time record"`). A citation of it, by that
identifier or by the plan-time phrase beside it, is resolved by the citation lint (NAB-10, citation form 5) to that
record, whose status is `not-run` until N16c-45 appends a reproduced result, so every gate, contract or close
record that cites it fails `cite.plan-check-not-reproduced` until then. Rejected: a 26th record `SRC-CHK-26` with
an empty locator, which would make an unrecorded result look like a registered reading and shift the count of
the plan-time table this register follows row by row.

**The 25 checks as recorded.**

| Check | What it measured | Result | Cited in | Id |
|---|---|---|---|---|
| Static stance family | Two exact classifiers of body–load feasibility; a projection against a lifted robust program | 11,520 of 11,520 states agree; half-width 3/14 m from both programs; a support-polygon-plus-capacity screen wrongly accepts 1,596 bodies | §25.13, C-32 | PC-01 |
| Stance falsifier | The same body under equal passive springs | (175, 115, 85, 25) N against a 160 N rating | §25.13 | PC-02 |
| Material probe | Generated probe, programmed expert and direct measurement over 900 cases and four laws | 675 of 675 in class, 0 of 225 on the withheld law; direct dwell 900 of 900 | §25.14 | PC-03 |
| Pad family | Minimum certified mass for a 75 N pull | 300 g and 240 g at centred readings only; an admitted reading drops the bound to 73.79 N | §25.14 | PC-04 |
| Sensor aliasing | Exact Bayes success with repeated binary reads | 27/32, 459/512, 3807/4096; a shared error bit stays at 3/4 | §20.12, C-19 | PC-05 |
| Docking, two gains | Exact three-tick docking with one velocity packet | ε < (U + ½)/4; of 561 cells, 344 admitted, 217 excluded, nine on the boundary; 51 linear programs agree | §20.16, §25.15 | PC-06 |
| Docking, three gains | Adding the gain 33/32 | Exact docking unsatisfiable; 34,590 positive labels per grid reverse | §20.16 | PC-07 |
| Specialized against composed solver | Ten fixtures, six corrupted certificates | 20 of 20 intervals match in each arm; six of six bad certificates rejected; median time ratio 1.345 warm and 1.088 cold | §2, §25.22 | PC-08 |
| Timing ceiling of that test | The part of the specialized batch that the solver does not touch | A residual of 575.7 µs (parsing, assembly, checking, serialization and harness, not separately timed), the 610.1 µs full median less the 34.4 µs solve-only median: a zero-cost solver would reach about 1.46 warm (842.8 ÷ 575.7) | §2, D-17 | PC-09 |
| Labels and deadlines | Payload-width frontier against number and timing of labels | 3/32, 7/32, 11/32 mm for one, two, three early labels; one label unsatisfiable, two satisfiable | §25.18 | PC-10 |
| Decision reuse | A semantic receipt against a dependency graph over four edit states | A tie; omitting one dependency inherits a stale promotion | §28.13 | PC-11 |
| Shared bus | Two fingers on one unit-current bus | Zero one-tick schedules, two two-tick schedules; an additive peak rule falsely prunes | §14.6, §31.12 | PC-12 |
| Admission fixtures | Exact counterexamples for mass support, design-dependent load, information, symmetric actions, dependent stage outcomes | Each prints its counterexample | §25.4.7 | PC-13 |
| Boundary screen on three GPUs | A 65,536-row docking-boundary screen on the M5 Ultra and both RTX 4090 | Unguarded single precision: 452 false admissions, 23,438 false exclusions; guarded with exact fallback: zero final errors, identical bytes; exact integer kernels: zero errors | §13.7, §24.5, §35 | PC-14 |
| Static against dynamic sill | Sampled statics against a planar dynamic scorer, 156 cases | 94 agree; 62 static rejections succeed dynamically; an audit found scorer defects (a latched dwell timer, no strike guard) | §19.12, §24.10 | PC-15 |
| Sampled wheel loop | Spectral radius of the one-tick map of a 75 mm wheel with a current target held for 10 ms | 1.4820 with target-current feed-forward, 0.9207 with measured-current feed-forward; an instantaneous-current model gives 2.6917 for both with the fixture's shaft damping (2.7075 without it); an explicit current-then-wheel update at 0.5 ms gives 1.497 where 0.921 is right | §24.10, §28.4 (A-22) | PC-16 |
| Winding feed-forward term | R·i_ref against R·i in the planar threshold pilot | 24 of 156 outcome labels change | §30.9 | PC-17 |
| Fixed-n rule enumeration | Every outcome of the paired rule at n = 64 and a five-point margin | 2,145 count pairs: 521 promote, 521 inferior, 1,103 inconclusive, no tie; a tie first reachable at n = 86 | §24.12, §28.6 | PC-18 |
| Power of the draft decision rule | Monte Carlo of the earlier ten-fold rule (log-normal run times, 5 to 20 seeds) | About 18% power against a true ratio of 10 | §39 | PC-19 |
| Half-precision conditioning | κ·u for contact Hessians of three robots after unit-diagonal scaling, with u = 2⁻¹¹ | κ·u ≥ 1 on a large share of contact states, so refinement from a half-precision factor does not converge | §21.20 | PC-20 |
| Humanoid conditioning | κ₂ of the humanoid contact Hessian over 201 poses | 3·10⁴ to 1·10⁵ raw and 2·10³ to 9·10³ scaled, at default contact impedance | §35 | PC-21 |
| CPU incumbent rate on this Mac | A reference quadruped at 2 ms, double precision, 36 threads, under background load | About 410,000 steps/s | §36 | PC-22 |
| Metal device query | Working-set limit and largest buffer on this Mac | About 222.7 GiB and 167.0 GiB | §21.16, §36 | PC-23 |
| Worked numbers of the body chapters | Thermal card, shell masses and gyration radii, orbit counts, voltage closed forms, bus ceiling, gear fixture, backlash impact, the single-precision predicate | Recomputed by script | §13 to §15 | PC-24 |
| Worked artifacts of the specification | The stance family of L.4 in exact rationals on a 81 × 31 grid against its closed form; the paving of L.4; the binomial bounds of L.9 | Zero mismatches; one inner, three unresolved and eight excluded cells; bounds 0.0432, 0.0504, 0.0425, 0.0505 | Appendix L | PC-25 |

**3. Consistency assertions on the recorded values** (exact arithmetic; they are the known answers of this
bead and catch transcription errors before any re-run):

- PC-01: 11,520 of 11,520; the half-width 3/14 m is a rational; 1,596 wrongly accepted bodies is a count of the
  support-polygon-plus-capacity screen, not of the exact classifier.
- PC-02: the four spring loads sum to 175 + 115 + 85 + 25 = 400 N, and the largest, 175 N, exceeds the 160 N
  rating: the same body that passes under independent forces fails under equal passive springs.
- PC-03: 675 + 225 = 900 cases; in class 675 of 675; on the withheld law 0 of 225; direct dwell 900 of 900.
- PC-05: 27/32 = 0.84375, 459/512 = 0.896484375, 3807/4096 = 0.929443359375, increasing with the number of
  reads; a shared error bit stays at 3/4 whatever the number of reads.
- PC-06: 344 + 217 = 561 cells; the boundary is ε < (U + ½)/4 with equality excluded, so at U = 3/4 the
  boundary value is 5/16 and a cell exactly on it is excluded; nine cells lie on the boundary.
- PC-08 and PC-09: 610.1 µs − 34.4 µs = 575.7 µs; 842.8 ÷ 575.7 = 1.4640; the median time ratios 1.345 (warm)
  and 1.088 (cold) are medians of per-fixture ratios and are not the ratio of the medians (842.8 ÷ 610.1 =
  1.381), so no test may assert that equality.
- PC-10: the frontier values 3/32, 7/32 and 11/32 mm increase by 4/32 mm per additional early label.
- PC-14: 452 false admissions and 23,438 false exclusions of 65,536 rows for unguarded single precision; zero
  final errors and identical bytes for the guarded path with exact fallback; zero errors for exact integer
  kernels.
- PC-15: 94 + 62 = 156 cases.
- PC-16: spectral radii 1.4820 (target-current feed-forward, unstable), 0.9207 (measured-current feed-forward,
  stable), 2.6917 with the fixture's shaft damping and 2.7075 without it for an instantaneous-current model, and
  1.497 where 0.921 is right for an explicit current-then-wheel update at 0.5 ms: stability flips with the
  model of the sampled loop.
- PC-17: 24 of 156 outcome labels change with the winding feed-forward term (15.4%).
- PC-18: 521 + 521 + 1,103 + 0 = 2,145, which is the number of count pairs (a, b) with a + b ≤ 64, namely
  66·65/2; a practical tie is first reachable at n = 86.
- PC-19: about 18% power against a true ratio of 10: the draft decision rule could not have confirmed the claim
  it was written to test. The pilot table of the planning record prints the same figure as "at most 18% of
  campaigns"; the typed figure is the bound, confirmation rate ≤ 0.18 at the maximum over the simulated grid
  (DESIGN, the same choice as ledger row 14 of N38a-10: a bound is what a re-run can be compared with).
- PC-21: κ₂ of 3·10⁴ to 1·10⁵ raw and 2·10³ to 9·10³ scaled, over 201 poses.
- PC-22: about 410,000 steps/s for a reference quadruped at 2 ms in double precision on 36 threads under
  background load. The review log describes the same unretained check as CPU rates "for a humanoid"; the two
  texts name different robots, so the record stores the table's wording and the re-run records which model it
  ran. (This figure of about 410,000 steps/s is unrelated to the 410 s training-time figure of an incumbent.)
- PC-25: 1 + 3 + 8 = 12 cells in the paving; the binomial bounds 0.0432, 0.0504, 0.0425 and 0.0505.

**4. Limits, stored on the archive record as `limits = [...]` and printed in the header of every report that
cites a plan-time check.** This is the one stored text (DESIGN): the plan-time pilot ledger (N38a-10) reads it
from this record and copies it into every re-run result, and tests there assert the copy equals it. Three
entries:

1. None used FrankenSim dynamics, FrankenSim contact or a real sensor pipeline. The analytic pilots used small
   CPU programs and conventional solvers; the native runs executed one scalar decision kernel; the planar sill
   pilots used their own law and scorer. No hypothesis of the program was tested by running FrankenRobotics
   code, because none existed.
2. Every pilot's strong baseline tied or won. A lifted linear program tied the projection; a programmed expert
   tied the probe generator and direct measurement beat both on the withheld law; exact Bayes tied the sensor
   calculation; a scenario tree tied the docking family; a dependency graph tied the receipt; quantifier
   elimination produced the pusher answers (8 of 12 queries directly). What survives is an interface and a
   discipline, not a speed claim.
3. No physical claim follows. The material laws are artificial, the loads stipulated, the plants scalar or
   planar.

## Acceptance criteria

- [ ] 25 `plan-check` records exist with the recorded values of the table typed exactly, each with
      `cited_in`, and the four rows without retained scripts (PC-20 to PC-23) marked `scripts_retained = false`.
- [ ] `SRC-REC-05` holds the four archive revisions with bytes and digests, `cited_revision = "a635b91"`, and
      the test that recomputes the digest of the working-tree file reports which revision it is.
- [ ] Every consistency assertion of step 3 is a passing test.
- [ ] Every `plan-check` record loads with `reread_before_gate = true` when no re-run status is supplied or the
      supplied status is `not-run` or `differs`, and with `false` only for `reproduced`. The refusal of a gate
      that cites such a record is NAB-10's finding `cite.plan-check-not-reproduced`, shown by NAB-16's script;
      NAB-10 and the ledger both come after this bead.
- [ ] The limits text is attached to the archive record and appears in the generated summary of the register.
- [ ] Each of the 25 records carries the `SRC-CHK` id and the recorded values that the plan-time pilot ledger
      (N38a-10) keys its coverage on, and no record names a re-run owner. GATE-GAUNT-08's re-run clause is
      evaluated from these records and that ledger together by the ledger's own check, which names the re-run
      owner of each check (BD-030, BD-041, BD-042, BD-043, N38a-11 or N00-35) and lists any `SRC-CHK` id that no
      record covers; it is not this bead's close condition.

## Test plan

### Unit tests

`crates/frob-conformance/tests/sources_content.rs`, module `plan_checks`:

- `twenty_five_checks`; `ids_pc01_to_pc25_in_table_order`.
- `archive_revisions`: four rows; the cited digest equals the `a635b91` row; `sha256(planning/*.zip)` equals
  exactly one row (the test prints which) and fails `src.archive-unknown-revision` otherwise.
- One test per bullet of step 3, named for its check: `pc02_springs_sum_and_exceed_rating`,
  `pc03_probe_counts`, `pc05_alias_fractions_increase`, `pc06_docking_cells_and_boundary`,
  `pc08_pc09_residual_and_ceiling` (asserts 575.7 and 1.4640 to four figures and asserts **inequality** of
  1.345 and 842.8 ÷ 610.1), `pc10_label_frontier_step`, `pc15_sill_counts`, `pc17_share_15_4_percent`,
  `pc18_reachable_verdicts_2145`, `pc25_paving_cells`.
- `limits_present_on_archive_record` (three entries).
- `plan_check_flag_follows_supplied_status`: `SRC-CHK-01` loads flagged with no status supplied and with
  `not-run` or `differs`, and unflagged with `reproduced`.
- (The test that each check is covered by at most one ledger record with a re-run owner is in N38a-10, which
  reads both registers.)

### End-to-end script and logging

`scripts/e2e/sources.sh` (NAB-16) extracts the cited revision (`git show a635b91:planning/<archive name>`)
and the revision found in the working tree, each into its own new scratch directory, and runs
`python3 -I -B verify.py` in each (inventory, digests and the standard-library checks; `--all` is left to the
re-run beads because it needs native builds); it compares the cited revision's printed counts with the recorded
values of the records, and the working-tree revision's with the cited one's (by the rule of step 1). Logged per STD-E2E: the archive digest and revision, per check a `metric` line with
recorded value, re-run value and verdict, and the limits header. The script never claims a gate and writes no
re-run status; GATE-GAUNT-08's re-run clause is evaluated from the plan-time pilot ledger (N38a-10).

### Known answers and negative controls

Known answers: every recorded value of the table and every arithmetic identity of step 3. Negative controls:
a record of PC-06 with 560 of 561 cells (must be `differs`, never rounded to agreement); a ceiling of 1.43
derived as 1.345 ÷ 0.94 (a withdrawn derivation: the test must fail it and name 842.8 ÷ 575.7); a gate that
cites PC-14 before BD-030 has reproduced it; an archive file whose digest matches no recorded revision. The
archive's own `verify_negative.py` is run by the end-to-end script, and its planted failures must fail.

## Considerations

- The archive is an untrusted input in the technical sense: the script extracts it into its own new, empty
  scratch directory, runs it there with `python3 -I -B`, and never executes it during a build.
- The discrepancy between the cited digest and the working-tree file is recorded, not repaired. Whoever owns
  the archive decides whether the citation moves to the newer revision; until then figures are compared with
  the cited one.
- The ledger of re-run status (N38a-10) holds twenty pilot records and four supplementary ones, and groups
  them under the nine checks of the review record; this register counts 25 rows because it follows the source
  table row by row. The two are joined by id: each ledger record lists in `covers` the `SRC-CHK` ids it governs
  (ledger row 17 covers PC-16 and PC-17; ledger row 18, the rotor and cadence sweep, has no row here; PC-13 is
  covered by the supplementary record S3, re-run by BD-043; PC-22 and PC-23 by S1 and S2; S4, the held-command
  device pilot, covers no row (step 2a); PC-24 and PC-25 have no ledger record until their re-run owner adds
  one, and are uncitable by a gate until then).
- These checks are the main evidence behind several refuted ideas (half precision, a fixed-n rule with an
  unreachable verdict, a speed test with no power). The killed-idea register cites them by `PC-` id.

## References

Appendix H.5; Appendix K.3; §0.6; §25.22 (inner paths of the pilots); §38.5 and GATE-GAUNT-08.
