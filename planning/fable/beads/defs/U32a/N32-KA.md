---
key: "N32-KA"
title: "frob-conformance: known-answer registry of the surface chapter (KA-755 to KA-766) with derivations"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-06"
deps: ["BD-025", "STD-E2E"]
needs: ["§38.6 and §38.11: the known-answer registry format with derivation files, and the negative-control registry `crates/frob-conformance/registry/controls.toml` (seed BD-025 and the Gauntlet unit)", "Appendix N.29: the counterexample registry of this chapter, CX-345 to CX-351, which seeds the negative controls named here (the registry bead of the other Tether unit)", "§24.13: the blinded qualification lane, whose arithmetic KA-760 and KA-762 state (seed BD-259)"]
related: ["BD-022", "BD-211", "BD-295", "BD-296", "BD-297", "BD-299", "BD-313", "BD-373", "N32a-03", "N32a-07", "N32a-08", "N32a-09", "N32a-10", "N32a-13", "N32a-18", "N32a-20", "STD-TEST"]
gates: []
plan_refs: ["Appendix M.31", "§32.20", "§32.4", "§32.7", "§32.8.1", "§32.8.2", "§32.9", "§32.10", "§32.11.1", "§32.11.2", "§32.16", "§38.6"]
---
## Background

A **known answer** is a small case whose truth is computed by hand, by a closed form or by exhaustive
enumeration, with a stated tolerance, the verification tier it runs on, and where the plan names one a
**negative control**: a planted defect the test must catch (the project's rule is that a gate nothing can fail
validates nothing). Every chapter of the FrankenRobotics plan contributes rows to one registry in the test
harness crate `frob-conformance`.

This bead is the registry of the twelve known answers that belong to the surface chapter (the CLI, the daemon,
the MCP server). It exists so that none of them is lost between the beads that implement the mechanisms: each
mechanism bead cites its KA numbers and repeats the values it needs, and this bead is the single list a reviewer
checks them against. The rows are data (records in the known-answer registry, each with a derivation file), not
prose.

Verification tiers used below. G0: laws and self-tests (algebraic identities checked in a test binary). G1:
known answers (analytic and synthetic truths). G4: chaos (cancellation at every boundary, injected faults; for
this chapter, crash consistency of the job and chunk protocols under deterministic simulation of the daemon and
ledger). Execution sites: `host` is exact or double-precision arithmetic in a test binary; `daemon` is `frobd`,
and `daemon (simulated)` is `frobd`'s code under the deterministic simulation runtime with a fake device.

## Scope

In: the twelve rows with case, truth, tolerance, tier, negative control and owning mechanism bead; a derivation
file per row; registration; the check that every row is claimed by a test and every stated control by a
registered negative control. Out: the tests themselves (in the mechanism beads); the counterexample registry and
the refusal registry of this chapter (the other Tether unit).

## Technical approach

**The rows.**

| KA | Where stated | The case | The truth | Tolerance | Tier · site | Negative control | Tested in |
|---|---|---|---|---|---|---|---|
| KA-755 | envelope; known answers | Exit-code table on a fixture of malformed, unknown, over-budget, unscoped and conflicting inputs | The exit code and refusal code for each: malformed → `FROB-REF-802`, exit 3; unknown verb → `-801`, exit 3; over budget → `-701`, exit 4; unscoped → `-720`, exit 5; conflicting key → `-803`, exit 7 | exact | G1 · host | none stated | N32a-03, BD-022, N32a-19, BD-299 |
| KA-756 | integers | 2⁵³ + 1 = 9,007,199,254,740,993 through a double-decoding client, as `int53` and as `u64str` | Refused as `int53` (`FROB-REF-808`); preserved exactly as `u64str`. As a bare JSON number it decodes to 9,007,199,254,740,992 | exact | G1 · host | A schema with a bare integer type fails the schema lint | N32a-07, BD-211 |
| KA-757 | known answers | Pair accounting on a planted comparison | a_only + b_only + both + neither + unscored = planned | exact | G1 · host | none stated | N32a-13 |
| KA-758 | scheduling; known answers | The M/G/1 table against a discrete-event simulation of the dispatcher | With 90% of jobs 10 s and 10% of jobs 30 min (E[S] = 189 s, E[S²] = 324,090 s²): first-come waits 857, 2,001, 3,430 and 7,716 s at utilization 0.5, 0.7, 0.8 and 0.9; short-class waits 439, 621, 713 and 806 s under priority only, and 0.12, 0.17, 0.20 and 0.22 s plus one quantum under preempt-resume; slowdown of long jobs 2.4%, 3.4%, 4.0% and 4.5%. Each cell within 5% at 10⁶ simulated jobs (TARGET) | 5% | G1 · daemon (simulated) | Priority without preemption | N32a-20, BD-296 |
| KA-759 | known answers | Conformal index at n = 19, α = 0.05 | The 19th smallest residual, that is, the largest | exact (index) | G0 · host | none stated | BD-297 |
| KA-760 | known answers | Error budget of the blinded lane | Σ over i = 1..n of α ÷ [i·(i + 1)] = α·n ÷ (n + 1) < α | exact (rational) | G0 · host | none stated | BD-313, BD-373 |
| KA-761 | known answers | Slot share of a 96-pair comparison in 8,192 worlds | 192 ÷ 8,192 = 2.34% | exact | G1 · host | none stated | BD-295, N32a-15 |
| KA-762 | known answers | Exact binomial bound, 200 worlds, α = 0.025 | Upper bounds 0.0183, 0.0275, 0.0357, 0.0432, 0.0504 for 0 to 4 failures | to the printed digits | G0 · host | none stated | BD-313, BD-373 |
| KA-763 | idempotency | Requests A and B with the same key and payload, interleaved | One job and one budget charge; the retry returns the stored receipt with `idempotent_replay: true`; the same key with a different digest is refused (`FROB-REF-803`, exit 7) | exact | G4 · daemon (simulated) | Look up, then insert, in two statements: two jobs run and the budget is charged twice | N32a-09, BD-211, BD-299 |
| KA-764 | pagination | Rows at t = 1, 2, 3, 4, page size 2; a row at t = 1.5 commits after page one; two worlds each with a sample at t = 2 | Pinned to the committed frontier with order key (instant, world address, channel, sample index), page two returns {3, 4} and no row is repeated or skipped | exact | G4 · daemon (simulated) | Offset pagination returns {2, 3}, repeating row 2; a time-only cursor skips the second sample at t = 2 | N32a-10, BD-211 |
| KA-765 | jobs | Chunk partition: a retry splits a shard into tick ranges [0, 100) and [50, 150); a reset reuses tick zero | With the fixed partition [k·C, (k + 1)·C) per (world, episode), the second extent is a duplicate (original receipt) or a conflict (`FROB-REF-810`); the second episode does not overwrite the first | exact | G4 · daemon (simulated) | Range-keyed chunks: both ranges commit. A (world, tick) key: the second episode overwrites the first | N32a-08, N32a-18 |
| KA-766 | jobs | Terminal receipt of a job | Completed, failed, refused and unstarted trial counts sum to the planned number | exact | G1 · daemon | none stated | N32a-08, N32a-18 |

**Derivations (one file per row under `crates/frob-conformance/known_answers/tether/`; the content each must hold).**
- *KA-755.* The table of exit codes: 0 completed; 2 completed with findings; 3 refused input or question; 4
  budget; 5 capability; 6 device; 7 conflict; 8 accepted and still running; 9 indeterminate; 1 never emitted on
  purpose. The five fixture rows follow from the refusal registry's exit field.
- *KA-756.* An IEEE double has a 53-bit significand, so consecutive integers are representable only up to
  magnitude 2⁵³ − 1 = 9,007,199,254,740,991; 2⁵³ + 1 is odd and lies between two representable even values;
  round-to-nearest-even gives 2⁵³ = 9,007,199,254,740,992.
- *KA-757.* Each planned pair falls in exactly one of five classes (DESIGN reading of the five names: only arm A
  met the requirement, only arm B, both, neither, or the pair was not scored), so the five counts partition the
  planned pairs.
- *KA-758.* Pollaczek–Khinchine: with arrival rate λ, utilization ρ = λ·E[S] and W₀ = λ·E[S²] ÷ 2, the
  first-come wait is W₀ ÷ (1 − ρ); non-preemptive priority for the short class waits W₀ ÷ (1 − ρ₁) with ρ₁ =
  λ₁·E[S₁]; under preempt-resume the short class sees only itself, λ₁·E[S₁²] ÷ (2·(1 − ρ₁)), plus at most one
  quantum; long jobs slow by 1 ÷ (1 − ρ₁) − 1. Here E[S] = 0.9·10 + 0.1·1,800 = 189 s; E[S²] = 0.9·100 +
  0.1·3,240,000 = 324,090 s²; λ = ρ ÷ 189; λ₁ = 0.9·λ; E[S₁] = 10 s; E[S₁²] = 100 s². Unrounded values at ρ =
  0.5, 0.7, 0.8, 0.9: first come 857.4, 2,000.6, 3,429.5, 7,716.4 s; priority 439.1, 620.9, 713.1, 806.2 s;
  preempt-resume 0.122, 0.172, 0.198, 0.224 s; slowdown 2.44%, 3.45%, 3.96%, 4.48%.
- *KA-759.* The split-conformal index is k = ⌈(n + 1)·(1 − α)⌉. With n = 19 and α = 1 ÷ 20: ⌈20 × 19 ÷ 20⌉ = 19.
  The index exists only when k ≤ n: n ≥ 19 at α = 0.05 and n ≥ 9 at α = 0.10.
- *KA-760.* α ÷ [i·(i + 1)] = α·(1 ÷ i − 1 ÷ (i + 1)); the sum telescopes to α·(1 − 1 ÷ (n + 1)) = α·n ÷ (n + 1),
  which is below α for every n. With α = 0.05: α₁ = 0.025, α₂ = 0.05 ÷ 6 = 0.008333…; the first ten levels sum
  to 0.05 × 10 ÷ 11 = 0.045454….
- *KA-761.* 96 pairs are 192 worlds; 192 ÷ 8,192 = 3 ÷ 128 = 0.0234375.
- *KA-762.* The one-sided exact (Clopper–Pearson) upper bound p_U for k failures in n trials at level α solves
  Σ over j = 0..k of C(n, j)·p^j·(1 − p)^(n − j) = α. For k = 0 it is 1 − α^(1 ÷ n) = 1 − 0.025^(1 ÷ 200) =
  0.01828. Values for k = 0..4 at n = 200, α = 0.025: 0.0183, 0.0275, 0.0357, 0.0432, 0.0504. Consequence used
  by the worked mission session: a requirement "failure probability at most 0.05" admits at most 3 failures on
  the first submission; at α₂ = α ÷ 6 the bounds are 0.0425 at 2 failures and 0.0505 at 3, so a second
  submission admits at most 2.
- *KA-763.* The admission transaction: begin immediate; look up (principal, verb, key); insert binding, debit
  and job row; commit; with a unique index on (principal, verb, key). Any interleaving of two submitters
  serializes on the immediate transaction; the second finds the binding.
- *KA-764.* Rows at or below the frontier and strictly after the last key, in a total order: after page {1, 2}
  the remaining rows of the snapshot are {3, 4}; t = 1.5 has a commit sequence above the frontier and is
  invisible.
- *KA-765.* The key of a chunk is (world, episode, chunk index) with extents fixed by C; a re-split shard can
  only produce extents that already exist (duplicates) or bytes that differ (a conflict); the episode index
  separates tick zero of two episodes.
- *KA-766.* Each planned trial ends in exactly one of completed, failed, refused, unstarted.

**Registry record (per row; DESIGN layout following the harness's format).** `id`, `chapter = "tether"`, `case`,
`truth` (a value, a table or a predicate), `tolerance`, `tier`, `site`, `derivation` (path), `control` (the
identifier of a registered negative control, or `none-stated`), `tests` (the test identifiers that claim it),
`first_milestone` (M0 for KA-755, KA-759 to KA-762; M1 for KA-756, KA-763 to KA-766 in simulated form; M2 for
their daemon forms; M3 for KA-757 and KA-758 on the real dispatcher).

**Checks.** `known_answers_tether_complete`: the twelve identifiers are present once each; every row has a
derivation file whose stated value equals the registry's; every row whose milestone has been reached is claimed
by at least one passing test; every `control` other than `none-stated` resolves to a registered negative
control with a caught status. The arithmetic rows (755 table shape, 756, 758 closed forms, 759, 760, 761, 762)
are recomputed by the check itself in exact or high-precision arithmetic, independently of the mechanism code.

## Acceptance criteria

- [ ] Twelve registry records, KA-755 to KA-766, each with case, truth, tolerance, tier, site, derivation and control fields as in the table.
- [ ] Twelve derivation files, each reproducing its row's value from first principles as stated above.
- [ ] `cargo test -p frob-conformance known_answers_tether` recomputes the arithmetic rows independently: 9,007,199,254,740,993 → 9,007,199,254,740,992 through a double; the sixteen M/G/1 cells to the printed digits; k = 19; the telescoping sum as an exact rational for n = 1..1,000; 192 ÷ 8,192 = 3 ÷ 128; the five binomial bounds to four decimals.
- [ ] The five rows with a stated negative control (KA-756, -758, -763, -764, -765) each resolve to a registered control, and the check fails if one is removed.
- [ ] A row cannot be recorded as passing before a test claims it; rows ahead of their milestone are `not yet due`, never `pass`.

## Test plan

### Unit tests (`cargo test -p frob-conformance known_answers_tether`)
- `twelve_rows_present`.
- `ka756_double_rounding`: `9007199254740993_f64 as u64 == 9007199254740992`.
- `ka758_closed_forms`: the unrounded values listed in the derivation, to 0.1 s (waits) and 0.01 percentage points (slowdowns).
- `ka759_index`: integer arithmetic gives 19; for n = 18 no index exists.
- `ka760_telescoping_exact`: rational arithmetic, n = 1..1,000; also α₁ = α ÷ 2 and α₂ = α ÷ 6.
- `ka761_share`: 192 ÷ 8,192 as the rational 3 ÷ 128; printed 2.34%.
- `ka762_binomial_bounds`: root-finding on the exact binomial sum in high precision; 0.0183, 0.0275, 0.0357, 0.0432, 0.0504; and at α ÷ 6: 0.0425 at 2, 0.0505 at 3.
- `ka762_zero_failures_closed_form`: 1 − 0.025^(1 ÷ 200) = 0.01828 to five decimals.
- `controls_resolve`; `removing_a_control_fails`.
- `derivation_value_matches_registry` for each row.

### End-to-end script and logging
`scripts/e2e/known_answers_tether.sh`: runs the registry check and the independent recomputation, then, for each
row whose milestone has been reached, runs the claiming tests named in the record and collects their results.
It sources the shared helper and writes `target/e2e-logs/known_answers_tether/<utc>.ndjson` plus a transcript:
one `assert` line per row with the expected value, the recomputed value and the tolerance; `metric` lines for
rows present (12), rows due, rows claimed, rows passing, controls resolved (5 of 5); on failure the row, the
first differing digit and the derivation path; a `summary` with counts.

### Known answers and negative controls
- The rows above are the known answers.
- Negative controls of this registry itself: a derivation file edited to a wrong digit (the value-match check must fail); a row marked passing with no claiming test (the completeness check must fail); a planted wrong cell in the M/G/1 table, 621 changed to 612 (the recomputation must fail).
- Determinism lanes: the recomputation identical twice on one machine, in debug and release, on both hosts.

## Considerations

- KA-760 and KA-762 are stated by the surface chapter because its worked mission session uses them; the mechanism they describe belongs to the blinded qualification lane, whose own registry may hold the same arithmetic under its own numbers. The two must agree; this registry's check recomputes them independently.
- KA-758's tolerance is a target on a simulation with a heavy-tailed service distribution; the claiming test reports a confidence interval and the run length used.
- KA-765 names `FROB-REF-810` for the chunk conflict; the telemetry chapter lists a second code for different bytes under a committed chunk key. The row's truth is the behaviour (duplicate or conflict; no overwrite), and the code asserted is whichever the refusal registry assigns.
- Rows marked "none stated" still need the rule that a gate must be able to fail: the mechanism beads that claim them plant their own controls (for example a wrong exit in the fixture for KA-755).

## References

Appendix M.31 (KA-755 to KA-766), §32.20, §32.4, §32.7, §32.8.1, §32.8.2, §32.9, §32.10, §32.11.1, §32.11.2,
§32.16, §38.6; CX-345 to CX-351.
