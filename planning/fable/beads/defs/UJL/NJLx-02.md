---
key: "NJLx-02"
title: "frob-conformance: known-answer rows for the verb registry and the nine worked artifacts (Appendices J and L)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "N38a-12", "N38a-40", "STD-E2E", "STD-TEST"]
needs: []
related: ["NJLx-51"]
gates: ["GATE-GAUNT-02"]
plan_refs: ["Appendix J", "Appendix L", "Appendix M", "§38.6"]
---
## Background

A known answer is a case whose truth is derived on file, compared by a test with a stated tolerance, on a stated
tier and site, with a negative control that must make it fail. Known answers are rows `KA-nnn` of
`crates/frob-conformance/registry/known_answers.toml`; gates cite them by number; the printed suite is generated
from the registry. Numbers are append-only: never reused, never renumbered. The printed suite ends at KA-835 and
is grouped by chapter, so it has no group for the two appendices that state hand-checkable truths of their own:
the verb registry (the closed list of 243 verbs with their kinds, clusters and milestones) and the nine worked
artifacts (one fully worked file of each exchange format, with every number derivable by hand).

The beads that build those two appendices state their truths inline and say that they have no number (the
registry reconciliation, the task-cluster assignment, eight golden-fixture beads and the second half of the
scripted agent session). Every other chapter has a registry bead that enters its rows; these two appendices had
none, so their truths could not be cited by a gate, could not be listed by the registry's completeness check, and
had no place where their controls were registered. This bead is that registry bead. Tier: Solid. It serves the
rule that a gate nothing can fail validates nothing: each row here names the mutation that must fail it.

Which statements become rows (DESIGN). A statement becomes a known answer only if it can be recomputed by hand
from the artifact's printed inputs: a count, an exact rational, an integer energy sum, a closed-form bound.
Values that a simulation produced when the specimen was recorded (digests, trajectories, the outcome counts of a
seeded run) stay **golden fixtures** under the blessing rule of the fixture layout, because their truth is "what
the pinned build produced", not a derivation. Rejected: registering every golden value as a known answer (a
re-blessing would then rewrite truths, which the suite forbids); leaving the hand-derivable values as test
constants in the fixture beads (no number, no registered control, invisible to the gate register).

## Scope

In: eighteen rows with case, truth, tolerance, tier, control and owner; the request for a block of eighteen
known-answer numbers; the derivation files; the mapping from provisional tags to numbers; the registry tests.
Out: the verbs, the fixtures and the tests that exercise them (the beads named in the Owner column); the
allocation ledger and its rules (N38a-40); the row schema and the loader (N38a-12); counterexample numbers (the
ten shortcuts these appendices keep alive belong to their chapters' catalogues, and this bead requests none);
known answers of the suite that the fixtures only cite (KA-090, KA-612, KA-693, KA-755, KA-756, KA-757, KA-760,
KA-762, KA-763 keep their numbers and owners).

## Technical approach

**1. Block request.** One `[[block]]` entry in `crates/frob-conformance/registry/allocations.toml`:
`registry = "ka"`, `count = 18`, `owner = "NJLx-02"`, `section = "Appendix J, Appendix L"`. The allocator fills
`first` and `last`; nobody types them. Until the block is issued the rows carry the tags `JL-01` to `JL-18`; once
it is issued the tags are mapped to numbers in table order in one commit, and a tag left in a registry, a test
name or a derivation file after this bead's milestone fails the allocator's check.

**2. Rows.** Every row: tier `G0 · host` (exact integer or rational arithmetic in a test), `section` as given.

| Tag | Stated in | Case | Truth | Tolerance | Negative control | Owner |
|---|---|---|---|---|---|---|
| JL-01 | Appendix J | The baseline verb registry (the appendix's closed list, `fixtures/golden/verbs/baseline-243.txt`), with `fidelity.rank` unregistered (the fail-closed default of its owner decision) | 243 verbs in 44 registry clusters; the complete registry holds 243 plus one row per `[[addition]]` record (244 with `artifact.import` of VERBS-artifact), minus each baseline name withdrawn into the retired-name table; while clusters are still unregistered the registry is the list restricted to the registered clusters, plus the additions (NJL-03's staged check) | exact | A registry with one row removed; one with a row duplicated under an alias | NJL-03 |
| JL-02 | Appendix J | Default kind of each baseline verb | 136 read, 83 run, 15 mutate, 6 admin, and 3 rows printed with two kinds (`nemesis.hardset`, `scene.bank`, `spark.cache`; cluster beads have since declared actions on further rows, which the baseline does not count); 136 + 83 + 15 + 6 + 3 = 243 | exact | A `mutate` verb relabelled `read` | NJL-03 |
| JL-03 | Appendix J | First milestone of each baseline verb | M0 16, M1 72, M2 41, M3 53, M4 46, M5 11, M6 1, M8 3; sum 243 | exact | A verb moved one milestone earlier | NJL-03 |
| JL-04 | Appendix J | Reconciliation with the mechanical inventory | 163 inventory rows + 80 added rows = 243 | exact | An added row that duplicates an inventory row | NJL-03 |
| JL-05 | Appendix J | Task clusters of the baseline | 45 clusters = 17 principal + 11 named by the appendix + 17 added; 240 seated verbs + 3 base tools = 243; no focus lists more than seven tools | exact | A server patched to list eight tools after a focus | NJL-05 |
| JL-06 | Appendix L.1 | The design `quad-tray` respelled: key order, white space, unit spelling | One identity for every spelling; the canonical form's pointer table as printed | exact | A reader built with key sorting disabled | ART-L1 |
| JL-07 | Appendix L.1 | Derived facts of `quad-tray` | 12 actuated coordinates; feet at (±1/5, ±1/7) m; mechanical symmetry group of order 4 and observation-preserving subgroup of order 2; v = 7 and r = 47/20 at the cell centre | exact | The mechanical group used as the policy's symmetry (must be refused for weight sharing) | ART-L1 |
| JL-08 | Appendix L.2 | The module `leg-3r-qdd` read against `quad-tray` | 4 × 38 A = 152 A; 4 × 0.4 W/K = 1.6 W/K; 22.2 V inside [18, 25.2] V; 27 = 21 + 6 scalars; selector diag(0, 1, 1, 1, 1, 1) | exact | A 1.5 W/K sink; a 26 V bus; a reversed power sign | ART-L2 |
| JL-09 | Appendix L.3 | Clocks of the kitchen tray task | 50 and 1,000 ticks; 51 ticks in the closed one-second window; reach 20 s; lidar on ticks 25 + 50·k | exact | A half-open window (50 ticks); an evaluator that reads only the first 20 s | ART-L3 |
| JL-10 | Appendix L.3 | The eleven labelled traces | The eleven labels as printed; margin −0.42 on the failing trace | exact | Each of the nine planted trace defects; margins composed without scales | ART-L3 |
| JL-11 | Appendix L.4 | The stance and knee-ratio query, closed form | Witness forces (76, 44, 40, 0) N and (88, 32, 40, 0) N; dual values 64 and 94; candidate 1/6 m against bound 1/8 m, gap 1/24 m, then closed; centre reserve 6 N and 6/5 N·m | exact | A swapped foot order with unchanged forces; an unreported positive force floor; an incumbent reported as a minimum | ART-L4 |
| JL-12 | Appendix L.4 | The paving and the grid check | 12 cells = 1 inner + 3 unresolved + 8 excluded; 13 cells after refinement; 515 of 2,511 grid points (81 × 31) feasible; zero mismatches | exact | A cell with a flipped verdict; an off-by-one grid (80 × 31) | ART-L4 |
| JL-13 | Appendix L.5 | The energy budget of the stop at the pass counter, in µJ | 2,433,600 + 1,200,000 + 110,000 + 150,000 + 100,000 = 3,993,600 ≤ 4,500,000 (½·18·0.52² J; 40 W·0.030 s; gravity 0.105948 J bounded by 0.11 J; stored; reserve; absorber 150 N over 30 mm) | exact | The rotor's reflected inertia omitted from the kinetic term; the gravity term rounded down | ART-L5 |
| JL-14 | Appendix L.5 | Boundary pairs of the same budget | Cutoff delay 42,660 µs: sum 4,500,000, margin 0, admitted; 42,661 µs: 4,500,040, margin −40, refused. Reading 0.55 m/s: 4,484,100, margin +15,900, admitted; 0.56 m/s: 4,587,600, margin −87,600, cut | exact | A strict comparison at the limit (refuses margin 0); a rounding of the delay to milliseconds | ART-L5 |
| JL-15 | Appendix L.6 | Shrinking the three-edit world | 7 single and 21 pair tests, 28 of 42 run; threshold 48 of 64 paired seeds; margin (12 − 17.04)/12 = −0.42; with edits listed (a, c, d, e, b) and failure when a and b, or c, d and e, are present, delta debugging stops at three edits while a two-edit failure exists | exact | A shrinker that reports the 1-minimal world as the smallest failure | ART-L6 |
| JL-16 | Appendix L.7 | Entry e2 and its neighbours in the certificate bundle | Each of the eight force vectors sums to 160 N, has first moments (±16, ±12) N·m and stays at or below 88 N; with y = (0, ½, ½), 40 + 6v > 40·r on v ∈ [4, 8], r ∈ [1, 3/2], least gap 4 N at (4, 3/2); thirteen cells partition [4, 12] × [1, 5/2] | exact | A first moment (16, 12) passed as a torque; a planted overlap; a planted gap; a non-reduced rational | ART-L7 |
| JL-17 | Appendix L.8 | The fixed-n study plan at n = 64 with counts (a, b) = (2, 19) | Interval [0.0554, 0.4392] | to the printed digits | Labels A and B swapped without negating the interval; a one-sided bound printed as two-sided | ART-L8 |
| JL-18 | Appendix L.9 | The second sealed request of the scripted session, 200 sealed clusters, α = 0.05 | Level α/[2·3] = 0.05/6; upper bounds 0.0425 at 2 failures and 0.0505 at 3, so at most 2 failures admit; the budget identities of the specimen hold exactly | bounds to the printed digits; identities exact | A constant level 0.05 on the second request; a retry that charges twice | NJL-09 |

**3. Derivation files.** `docs/derivations/appendix-j/registry-counts.md` and
`docs/derivations/appendix-l/l<n>.md`, one per artifact: the model, the derivation step by step, the value with
its digits, and the error budget (none for exact rows; the rounding rule for JL-17 and JL-18). The registry row
points to its file; a row without one cannot support a gate.

**4. Values are not improved in transit.** Each truth is carried as the appendix prints it. A recomputation that
disagrees keeps the printed value, is listed with the two values in the suite's reconciliation list, and the
owner bead records the finding; this bead does not edit a fixture.

**5. Printout.** The generated suite gains one group after the last chapter group, "Appendices J and L", from
the rows' `section` field. Determinism class D0; outputs are Known answers (class Tested); nothing enters the run
ledger.

## Acceptance criteria

- [ ] The block request for 18 known-answer numbers is on file; after issue, the eighteen rows carry numbers
      inside that block, in table order, and no `JL-` tag remains anywhere in the tree.
- [ ] Every row loads with case, truth, tolerance, tier, site, control, owner bead and derivation file.
- [ ] `cargo test -p frob-conformance --test g0_laws -- appendix_jl` (the tier target's long name: BD-025's lint
      refuses `--test g0`) recomputes every truth from the committed registry and fixtures in integer or exact
      rational arithmetic and passes.
- [ ] Each row's negative control is registered in the negative-control registry and is recorded as caught.
- [ ] Each owner bead's test cites its row by number, and the registry's completeness check lists no row of
      this block without a citing test.
- [ ] The owner beads depend on this one, so at its close every row is `pending: <owner>`: its truth is
      recomputed from the derivation file's printed inputs (the arithmetic tests below), while the comparison
      with the committed registry or fixture, the control's `caught` record, the citing test and step (2) of
      the script are required from the owner's landing; a row still pending after its owner's milestone fails
      both the test target and the script.
- [ ] The generated suite prints the group "Appendices J and L" with eighteen rows.

## Test plan

### Unit tests
`crates/frob-conformance/tests/g0/appendix_jl.rs`, a module of the tier target `g0_laws` (`tests/g0_laws.rs`
includes it, so the filter `appendix_jl` selects its tests; real registry files and real fixtures; no mocks):
- `jl_rows_load_complete`: eighteen rows, every field present, tolerances `exact` except the two stated.
- `jl_rows_inside_block`: every number lies in the block owned by this bead; a row numbered outside it fails.
- `jl_registry_counts`: on the baseline list: 243, 44; 136 + 83 + 15 + 6 + 3; the milestone histogram sums to
  243; 163 + 80. `jl_total_is_baseline_plus_additions`: the committed registry's row count equals the number of
  baseline names of the registered clusters, plus the number of `[[addition]]` records, minus the baseline names
  withdrawn into the retired-name table (243 plus the additions once every cluster is registered: 244 with
  `artifact.import`), and equals `[counts].total` from the change in which NJLx-51 writes that table.
- `jl_cluster_counts` (on NJL-05's target file): 17 + 11 + 17 = 45; 240 seated baseline verbs + 3 = 243 (241 + 3 =
  244 with `artifact.import`); the largest focus list has at most seven tools.
- `jl_energy_budget_integers`: the five terms sum to 3,993,600; the four boundary sums and margins.
- `jl_stance_arithmetic`: the two witness vectors sum to 160 N; 40 + 6·4 = 64 and 40 + 6·9 = 94; 1/6 − 1/8 = 1/24;
  2,511 = 81 × 31.
- `jl_bundle_arithmetic`: for (88, 32, 40, 0) at v = 8, (1/5)·(88 + 32 − 40 − 0) = 16 and
  (1/8)·(88 − 32 + 40 − 0) = 12; least gap 64 − 60 = 4.
- `jl_levels`: 0.05/(1·2) = 0.025 and 0.05/(2·3) = 0.05/6 as exact rationals.
- `jl_tag_past_due`: a scratch registry that still holds `JL-07` at this milestone fails naming the file.

### End-to-end script and logging
`scripts/e2e/appendix_jl_known_answers.sh` (header: `bead=NJLx-02 milestone=M1 labels=gauntlet,known-answers
gates=GATE-GAUNT-02 seed=1 needs=cpu plants=stale-tag,empty-control,foreign-number`): (1) the test target above; (2) `frob capabilities --json` and `frob
robot-docs --json`, asserting that both list the count of `jl_total_is_baseline_plus_additions` (44 clusters and
243 verbs plus the number of `[[addition]]` records once every cluster is registered: 244 with `artifact.import`,
247 once the three late analysis verbs of NJLx-51 have landed) against rows JL-01 and JL-04 from the user's
side; (3) a
scan of the newest logs of the nine fixture scripts for an `assert` line that names each row's number, failing
on a row no script cites; a row still marked `pending: <bead>` before that bead's milestone is skipped and
logged as `pending` with the owner, and is a failure after it; (4) with a plant selected (a stale `JL-` tag, a
row with an empty control, a number from another owner's block) the script must exit non-zero with a failed
assertion naming the plant. It logs through the STD-E2E helper to
`target/e2e-logs/appendix_jl_known_answers/<utc-timestamp>.ndjson`: `cmd` lines with exact command line, exit
code and wall time; one `assert` per row with the measured value, the truth and the tolerance; `metric` lines
for rows loaded, rows cited and controls caught; the digest of the registry file; a `summary` line naming
GATE-GAUNT-02. No private-scene data is involved.

### Known answers and negative controls
- The eighteen rows above are the known answers; this bead adds none beyond them.
- Negative controls of the registry itself: a row with an empty control; a row whose truth was edited to match
  a failing test (caught by the append-only diff); a number taken from another owner's block; a stale tag.
- Metamorphic: reordering rows in the registry file changes no verdict. Determinism lanes: twice on one
  machine; debug and release; Linux and macOS; class D0.

## Considerations

- The milestone is M1 because the first owner beads land then; rows whose owner lands later (the task, the
  minimal counterexample world, the bundle, the study plan, the session's second half) are listed as `pending:
  <bead>` until that bead's milestone and are an error after it.
- JL-01 to JL-05 each hold two values: the baseline value, fixed by the appendix's list of 243 names, and the
  committed value, which is the baseline plus the registry's `[[addition]]` records and is read from the
  registry (from its `[counts]` table once NJLx-51 has written it). Adding a verb therefore never rewrites a
  baseline truth, and the committed values move in the commit that adds the rows. The first addition is
  `artifact.import` (VERBS-artifact at M1, recorded by NJL-03): 244 verbs, 84 `run`, M2 42, 81 rows outside the
  inventory and 241 seated verbs. NJLx-51 follows (three analysis verbs at M3): 247 verbs, 139 `read`, M3 55 and
  M4 47, 84 rows outside the inventory and 244 seated verbs. The 44 registry clusters and 45 task clusters are
  unchanged by both, and each amends the derivation file of these rows with the reason. The relation "committed
  = baseline + additions" is what JL-01 checks.
- If the owner decision on `fidelity.rank` registers that verb, it is an addition in the same sense and the
  baseline rows do not change.
- A golden value promoted to a known answer later needs a derivation first; the reverse (a known answer demoted
  to a golden value) is not allowed, because numbers are never withdrawn without a void entry in the block.

## References

Appendix J (the verb registry), Appendix L (worked artifacts L.1 to L.9), Appendix M and §38.6 (rules for a
row); ADR-GAUNT-02 (registries are data), INV-GAUNT-07 (append-only numbers); GATE-GAUNT-02.
