---
key: "N23-KA"
title: "Known-answer registry for Cortex (KA-576 to KA-593): gym semantics, PPO rules, shards, strides, requirements"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "cortex", "frob-learn", "frob-conformance", "registry", "test", "solid"]
parent: "WP-48"
deps: ["BD-025", "STD-TEST"]
needs: ["§23.25 to §23.28: the known answers of the late subsections (the sign-agreement frequency 0.6655 of the planted linear-phase sampler; the docking exclusion that flips with reward visibility; the first-crossing probability 0.78548 and the episode counts 36, 46, 74, 103; the forty-design fixture with τ = 41/45), which are stated in those subsections and have no row in the chapter's known-answer table; the unit that drafts them registers them"]
related: ["BD-192", "BD-193", "BD-197", "BD-198", "BD-255", "BD-321", "N23a-07", "N23a-08", "N23a-10", "N23a-11", "N23a-19", "BD-267", "BD-146", "BD-223"]
gates: []
plan_refs: ["Appendix M.22", "§23.21", "§38"]
---
## Background
A known answer is a case whose correct result is known independently of the code under test: from a closed form, an exact enumeration or a hand calculation. The program's rule is that every capability has known-answer tests against a pinned oracle that was never regenerated from the candidate, and that every gate has at least one negative control (a planted defect the test must catch; a gate that nothing can fail validates nothing). This bead is the registry of the learning subsystem's known answers: eighteen rows, KA-576 to KA-593, each with its case, its truth with the formula or value, its tolerance, the test tier and its negative control. Mechanism beads cite these numbers and also state the values inline; this registry is where they are held once, as data, so that a test, a gate record and a report all quote the same figure.

Tiers named in the rows: G1 is the known-answer tier of the conformance harness (exact or closed-form truths), G3 the convergence tier; "host" means plain host arithmetic, "lockstep" the batched tier in lockstep mode, "referee" the reference tier, "cuda" a CUDA device, "all tiers" every stepping tier. Where the source table gives the tolerance as "to set", the tolerance column below keeps that status and proposes a value (DESIGN) that the owning bead fixes when it first records its gate.

## Scope
In: the eighteen rows as machine-readable data; the loader and its schema check; a completeness test tying each row to at least one test and each negative-control cell to a registered control. Out: the tests themselves (owned by the beads in the last column); counterexamples (the chapter's counterexample registry, drafted by another unit); refusal codes (likewise); the known answers of the late subsections, which have no row in the source table.

## Technical approach
**Storage.** `crates/frob-conformance/registry/known_answers/cortex.toml`, one `[[ka]]` record per row with fields `id`, `where`, `case`, `truth`, `tolerance`, `tier`, `negative_control`, `tests` (test names), `gates`. Values that are exact are stored as exact strings (rationals such as `"4/3"`, `"32/9"`; decimals as printed). The loader refuses a record without a negative control unless the source row itself says none is stated (one row, KA-589), and refuses a duplicate identifier.

**The rows.**

| KA | The case | The truth (formula or value) | Tolerance | Tier | Negative control | Carried by |
|---|---|---|---|---|---|---|
| KA-576 | Bootstrap target: a true 8 s deadline with terminal reward 0; an external cut at the same instant on a continuing task with V = 10 and γ = 0.99 | 0 for the deadline; r + γ·V = 9.9 for the cut; termination wins when both flags are set (0) | exact | G1 · host | A single `done` flag gives 0 for both | BD-192, N23a-07, BD-197 |
| KA-577 | Final observation under explicit reset and under masked automatic reset | The same values in explicit mode's `obs` and in `final_obs` | exact (bitwise) | G1 · all tiers | A mode that returns only post-reset observations fails the truncation fixture | BD-192 |
| KA-578 | Reward rate: a rate term over a 10 s episode at 50 Hz and at 100 Hz | The same total at both rates (rate × simulated time) | to set (DESIGN proposal: 10⁻⁶ relative) | G3 · all tiers | A penalty of 1 per control step totals 500 and 1,000 | BD-192 |
| KA-579 | Physical discount γ(Δt) = exp(−β·Δt) with β = 0.5025 /s | γ = 0.99 at 20 ms and 0.99499 at 10 ms; a constant reward rate r is worth r·Δt/(1 − e^(−β·Δt)): 2.000·r at 20 ms and 1.995·r at 10 ms, near r/β = 1.990·r | to the printed digits | G1 · host | One per-step γ = 0.99 at both rates gives 2.0·r and 1.0·r | N23a-07, BD-197 |
| KA-580 | Stride conversion: a contiguous 2 × 2 `f32` view has strides (8, 4) in signed bytes and (2, 1) in unsigned elements; a view with byte origin 4 and strides (8, −4) | Index [1, 1] reads element 3; the negative-stride view indexes elements (1, 0, 3, 2) and is materialized or refused | exact | G1 · host | A verbatim copy of the stride numbers into a view on a 16-element buffer reads element 12, in bounds | BD-193 |
| KA-581 | Mixed-unit requirement: clearance 0.05 m with scale 0.10 m and thermal margin 12 K with scale 10 K (a temperature margin is a difference, so its scale is in kelvin; the source row prints the same magnitudes in degrees Celsius) | Margins 0.5 and 1.2; the clearance binds in every unit system | exact | G1 · host | The unscaled minimum is 0.05 in metres and 12 with the clearance written as 50 mm | N23a-05, BD-255 |
| KA-582 | Action law (rule P-3): unit-variance Gaussians clipped at +1 with means 0 and 1 | Endpoint ratio 0.5/0.158655 = 3.15149 for the clipped law (masses 1 − Φ(0) and 1 − Φ(1)); e^0.5 = 1.64872 for the latent law | to the printed digits | G1 · host | A mixture of the two laws is refused (FROB-REF-527) | N23a-08, BD-197 |
| KA-583 | Clipped surrogate (rule P-4) with ε = 0.2 and a negative advantage, finite differences in log σ at ratios 0.5, 1 and 2 | 0, −0.5 and −1 | to set (DESIGN proposal: 10⁻³ relative against finite differences in single precision; 10⁻⁸ in double) | G1 · host | The reversed branch gives −0.25, −0.5 and 0 | N23a-10, BD-197 |
| KA-584 | Critic gradient (rule P-5): critic output 2, stored value and return 0, loss c_V·(V − R)² | Gradient into the value head 2·c_V·(V − R) = 4·c_V | exact | G1 · host | The stored-value form gives 0 | N23a-10, BD-197 |
| KA-585 | Packed episodes (rule P-6): two independently reset one-step episodes packed together, frozen policy | Likelihood ratios 1 and 1 | to set (DESIGN proposal: exactly 1 in class D2 for our implementation; 10⁻⁶ for an external implementation) | G1 · host | Reading across the reset gives 1.582459 | N23a-11, BD-197 |
| KA-586 | PPO rules P-2, P-8, P-9, P-10, P-11 | Two collections from one snapshot are identical; with the clock, the value at the deadline equals the terminal reward; a planted `Faulted` world changes no gradient and appears in the failure count; the normalizer round-trips; the entropy of the clipped law at a bound matches a quadrature reference and differs from the Gaussian's | exact for P-2, P-9, P-10; to set for P-8 (DESIGN proposal: 2% of the terminal reward) and P-11 (10⁻⁶ against quadrature) | G1 · host, lockstep | Three planted defects that reproduce the reversed branch, the stored-value gradient and the packed-sequence read each fail the battery (GATE-CORTEX-07) | N23a-10, N23a-11, N23a-08, N23a-06, BD-197 |
| KA-587 | Shard gradient: one sample with gradient 1 and three with gradient 0, on one device and on shards of sizes 1 and 3 | 0.25 (sum 1 over global valid count 4) | exact | G1 · host, cuda | The mean of shard means gives 0.5 | N23a-19, BD-197 |
| KA-588 | Shard statistics: shards {0, 0} and {4} | Global mean 4/3, variance 32/9 (merge of (2, 0, 0) and (1, 4, 0): d = 4, μ = 4/3, S = 16·2·1/3 = 32/3, S/n = 32/9) | exact (rational) | G1 · host, cuda | Per-shard variance is 0 | N23a-19, BD-197 |
| KA-589 | Family break-even N > P/(s·(1 − f)) at P = 60 GPU-hours, f = 0.1, s = 410 s | About 590 quadruped designs (216,000/(410 × 0.9) = 585.4); about 70 to 130 humanoid designs at s = 0.5 to 1 GPU-hour (66.7 to 133.3); ceiling on the training phase 1/f | arithmetic; inputs are FACT to re-read | G1 · host | none stated | N23b-24, BD-321 |
| KA-590 | Training gate controls | Zero-weight, random and shuffled-label policies fail the return threshold | exact (verdict) | G1 · lockstep | An exported all-zero policy head passes a plumbing test and fails this one | BD-198, BD-323, N23a-13 |
| KA-591 | Box-cone exploit (a friction cone linearized as a box grants √2·μ along its diagonals, 41% more traction) | Flagged `law-sensitive`; the ranking is typed `model-conditional`; a genuine winner stays `model-robust` | exact (label) | G1 · lockstep, referee | An honest policy on the circular cone is not flagged | the exploitation-detector and model-form beads of another unit (BD-267 and the model-form rank check) |
| KA-592 | Exploitation detectors (GATE-CORTEX-04) | Each detector flags its planted exploit and passes an honest policy | exact (verdict) | G1 · lockstep | The honest policy is the control | BD-267 (another unit) |
| KA-593 | Streaming past-time robustness against offline evaluation on 10⁴ random traces (GATE-CORTEX-10) | Equal (bitwise: minimum, maximum and negation are exact in floating point) | exact | G1 · host | A future-time online reward is refused | BD-255 |

**Derived values the tests need (carried here so that no test re-derives them differently).** Φ(1) = 0.841345, so 1 − Φ(1) = 0.158655; exp(−0.01005) = 0.990000 and exp(−0.005025) = 0.994988; 0.02/(1 − 0.990000) = 2.0001 and 0.01/(1 − 0.994988) = 1.9950; 1/0.5025 = 1.99005; with κ = 2.565 /s, exp(−0.0513) = 0.9500; the P-4 fixture uses a sample with squared standardized residual 3/2, for which ∂ρ/∂ log σ = ρ/2.

**Completeness test.** `cargo test -p frob-conformance known_answers::cortex`: loads the file; asserts 18 records KA-576 … KA-593 with no gap; each record's `tests` names exist in the test tree once their owning beads have landed (until then the record carries `pending: <bead key>` and the test reports the pending count instead of failing); each non-empty negative-control cell maps to a control in the negative-control registry, attached to the gates of the rule it protects.

## Acceptance criteria
- [ ] `cortex.toml` holds exactly the eighteen rows above with the values as printed; the loader round-trips it and rejects a duplicate identifier, a missing truth, and a missing negative control (except KA-589).
- [ ] Every row names at least one owning bead and, once that bead has landed, at least one test; no row is `pending` when the M2 exit review runs, except KA-589, KA-591 and KA-592 (M4 and M3 rows), which must be resolved by their milestones.
- [ ] Each negative-control cell has a corresponding entry in the negative-control registry with its gate.
- [ ] The derived values are checked by a unit test in double precision against the printed digits.
- [ ] The discrepancy in KA-581's unit (the source table prints degrees Celsius where the chapter's own rule says a temperature margin's scale is written in kelvin) is recorded in the row's notes, and the row uses kelvin.

## Test plan
### Unit tests
`ka_cortex_registry_has_18_rows_576_to_593`; `ka_cortex_loader_rejects_duplicate_and_incomplete`; `ka_cortex_negative_controls_registered`; `ka_cortex_derived_values` (Φ(1), the two exponentials, the two constant-rate values, 1/β, the λ value, 0.5/0.158655 = 3.15149, e^0.5 = 1.64872, 216,000/369 = 585.4, the merge to 4/3 and 32/9 in exact rationals); `ka_cortex_tests_exist_or_pending`.
### End-to-end script and logging
No script of its own. The two Cortex scripts (`scripts/e2e/cortex_env.sh` and `scripts/e2e/cortex_train.sh`) read expected values from this registry through `frob eval run --battery … --json`, whose rows echo the KA identifier beside each expected and measured value; their NDJSON `assert` lines therefore carry `ka: "KA-5xx"`, which lets a reviewer trace a log line back to this table.
### Known answers and negative controls
This bead *is* the list. Negative control for the registry itself: a planted edit of one expected value (for example 9.9 → 9.8) must make the owning fixture fail, proving that tests read the registry and not a private copy; a planted deletion of a row must make the completeness test fail.

## Considerations
- Three rows (KA-590, KA-591, KA-592) are stated "where" in the detectors' subsection, which another unit owns; they are listed here because the registry is per chapter, and their tests are owned by that unit's beads.
- Rows with tolerance "to set" must have their tolerance fixed by the owning bead before its gate's first record; the proposals above are defaults, not measurements.
- The late subsections of the chapter state further known answers that have no row in the source table; they should be appended as new rows by the unit that drafts them, with fresh identifiers issued by the lead, never by renumbering these.

## References
Appendix M.22 (rows KA-576 to KA-593), §23.21 (the chapter's fixture table), §38 (known-answer and negative-control rules).
