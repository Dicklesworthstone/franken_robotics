---
key: "N33-KA"
title: "frob-conformance: known-answer registry of suite consumption (KA-767 to KA-780) with derivation files and controls"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-37"
deps: ["BD-025"]
needs: []
related: ["N33a-04", "N33a-27", "BD-036", "BD-033", "BD-034", "BD-035", "BD-038", "BD-031", "BD-111", "BD-124", "BD-333", "BD-401", "BD-413", "STD-TEST"]
gates: []
plan_refs: ["Appendix M.32", "§33.5", "§33.9", "§33.11", "§33.12", "§33.14", "§38.6", "§38.11"]
---
## Background

FrankenRobotics consumes sibling projects (FrankenSim, asupersync, FrankenSQLite, FrankenTorch, franken_numpy and
others) instead of copying them. Every statement the project makes about a sibling is a fact that can go stale, so
each one a gate leans on is turned into a **known answer**: a small case whose truth is computed by hand or by an
independent formula, with a stated tolerance, the verification tier it runs on, and a **negative control** (a
planted defect the test must catch; the rule is that a gate nothing can fail validates nothing).

This bead is the registry of the fourteen known answers that belong to the suite-consumption chapter. It exists so
that none of them is lost between the beads that implement the mechanisms: each mechanism bead cites its KA numbers
and repeats the values it needs, and this bead is the single list a reviewer can check them against. The registry
is data in `frob-conformance` (the Gauntlet harness crate, layer 7), not prose: a row here becomes a record in the
known-answer registry and its negative control becomes a record in `frob-conformance/registry/controls.toml`.

Verification tiers used below: G0 is static and algebraic checks on the host; G1 is known answers against closed
forms; G3 is metamorphic and randomized equivalence; G5 is determinism and replay (twice on one machine, across
worker counts, across platforms). "host" means the CPU with no device; "referee" means the CPU referee tier that
runs FrankenSim's own dynamics through the airlock crate `frob-fsim`.

Why the rows carry the reported wrong values too: several rows describe defects reported in sibling code (a
reversed clip branch in a PPO update, a planar-drive step that returns zero translation, a stride convention
trap). The wrong value is what the pinned sibling commit is reported to compute. It is the negative control's
expected observation, and it is FACT to recheck: a reported number stays a report until its admission test records
it at the pinned commit.

## Scope

In: the fourteen rows KA-767 to KA-780 as registry records with derivation files; one negative-control record per
row that states a control; the completeness test that joins rows to gates and to the beads that run them.
Out: the tests themselves, which live with their mechanisms (keys in the last column); the counterexample registry
of the chapter (`N33-CX`, written by another unit); the refusal registry (`N33-REF`).

## Technical approach

**Record format (DESIGN).** One TOML record per row in `crates/frob-conformance/registry/known_answers/suite.toml`:
`id`, `section`, `case`, `truth`, `tolerance`, `tier`, `control` (the `NC-` identifier or `"none"`), `gate` (list),
`derivation` (path to a file under `crates/frob-conformance/derivations/suite/` that reproduces the truth with the
standard library alone), `runs_in` (the test path). Identifiers are append-only and never renumbered. A row whose
tolerance is `to set` or whose control is `none stated` is not counted toward a milestone exit (the rule of the
Gauntlet's known-answer gate), so the two such rows below state what this bead sets.

**The rows.**

| KA | Case | Truth (formula or value) | Tolerance | Tier | Negative control | Gate; carried by |
|---|---|---|---|---|---|---|
| KA-767 | The stack resolution receipt (`frob.stack-receipt/1`) computed twice on an unchanged tree | The two files are byte-identical; so are their BLAKE3 hashes | exact | G5 · host | Touch one tracked file in a sibling copy: `dirty` flips to true, `tree_hash` changes, `check-stack` fails | §33.5; N33a-04, N33a-05, N33a-27 |
| KA-768 | Floquet multipliers of the monodromy of ẍ + x = 0 over one period T = 2π, through the airlock trait `Spectral` | The monodromy is the 2×2 identity, so both multipliers equal 1 | 10⁻¹² | G1 · host | A monodromy with a multiplier of modulus 1.01 is reported unstable | INT-FSIM-13; BD-333 |
| KA-769 | Two designs with mean gap Δ and standard deviations s₁, s₂; and a multilevel estimate of E[x²] for x standard normal | Ranking-flip probability Φ(−Δ/√(s₁² + s₂²)), Φ the standard normal distribution function (for Δ = 1, s₁ = s₂ = 1: Φ(−1/√2) = 0.23975006109347677); the estimate of E[x²] is 1 inside its own stated interval | 10⁻¹² for the closed form | G1 · host | Importance weights from a proposal that misses the failure region trip the effective-sample-size floor and the estimate is refused | INT-FSIM-14; BD-124 |
| KA-770 | A tolerance allocation on a two-term stack y = a₁·x₁ + a₂·x₂; a mesh written through `fs-io` | First-order variance split σ_y² = a₁²·σ₁² + a₂²·σ₂². Fixture set here: a = (1, 2), σ = (0.03, 0.02) mm gives contributions 0.0009 and 0.0016 mm², σ_y = 0.05 mm, shares 36% and 64%. The mesh reads back with the same content hash | exact for the hash; 10⁻¹² relative for the split (set by this bead; the plan left it `to set`) | G1 · host | A STEP file above the stated size bound is quarantined with a receipt and is not parsed | INT-FSIM-15; BD-401 |
| KA-771 | Incremental recomputation through `fs-recompute` on 10⁴ random edit sequences | Equal to full recomputation, digest for digest | exact | G3 · host | An input left out of a node key is caught by the shadow recompute within 64 hits | INT-FSIM-16; BD-111 |
| KA-772 | Poiseuille flow between parallel plates; film mass over 10⁴ steps, across the multiphysics bridge | The velocity profile is the parabola u(y) = (G/(2·μ))·y·(H − y) for pressure gradient G, viscosity μ and gap H, to the crate's own stated tolerance; total film mass is conserved | 10⁻¹² relative for mass | G1 · referee (evaluation tier) | The bridge refuses when the `multiphysics` feature is off (`FROB-REF-790`) and refuses a throughput-tier request | INT-FSIM-17; BD-413 |
| KA-773 | PPO clipped surrogate L = min(ρ·Â, clip(ρ, 1 − ε, 1 + ε)·Â) with advantage Â = −1 and ε = 0.2 | At ratio ρ = 0.5 the terms are −0.5 and −0.8: the clipped term is the minimum and the gradient is zero. At ρ = 2 they are −2 and −1.2: the unclipped term is the minimum. Finite differences of the loss in the log standard deviation at ρ = 0.5, 1, 2: 0, −0.5, −1 | 10⁻³ relative in single precision | G1 · host | The reported sibling code path gives −0.25, −0.5 and 0 | INT-FSIM-06; BD-034 |
| KA-774 | Critic output 2 with stored value 0 and return 0; two independently reset one-step episodes, packed | The squared-error objective gives gradient 2 into the critic bias; a frozen policy's likelihood ratio is 1 on both episodes | ratio 1 to 10⁻⁶ | G1 · host | The reported sibling code path gives gradient 0 and a second ratio of 1.582459 | INT-FSIM-06; BD-034 |
| KA-775 | Planar drive near zero curvature: heading θ = 32 rad, wheel speeds 1 and 1 + 6·10⁻¹³ m/s, track 0.5 m, Δt = 1/400 s, 400 steps | ω = (6·10⁻¹³)/0.5 = 1.2·10⁻¹² rad/s and ω·Δt = 3·10⁻¹⁵ rad, below half the spacing of doubles at 32 (2⁻⁴⁸ ≈ 3.55·10⁻¹⁵). The true displacement is one metre along the heading: (cos 32, sin 32) ≈ (0.83422336, 0.55142668) m | 10⁻¹² | G1 · host, referee | The reported branch returns exactly zero translation (θ + ω·Δt rounds to θ, so both trigonometric differences are zero) | INT-FSIM-05; BD-033 |
| KA-776 | The independent chord formula Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)) with δ = ω·Δt, sinc(x) = sin(x)/x, sinc(0) = 1: unit speed through a half turn from θ = 0 | With v·Δt = 1 m of path and δ = π (an arc of radius 1/π m): sinc(π/2) = 2/π and the direction is (cos π/2, sin π/2) = (0, 1), so the chord is (0, 2/π) m = (0, 0.6366197723675814) m | 10⁻¹² | G1 · host | Set here (the plan states none): replacing sinc(δ/2) by 1 (the straight-line step) returns (0, 1) m and must fail | INT-FSIM-05; BD-033 |
| KA-777 | A contiguous 2×2 single-precision view: strides (8, 4) in bytes and (2, 1) in elements; a second view with byte origin 4 and byte strides (8, −4) | Index [1, 1] is element 1·2 + 1·1 = 3. The negative-stride view addresses elements (1, 0, 3, 2) and is materialized with the copy charged to the caller, or refused under a zero-copy contract (`FROB-REF-617`) | exact | G1 · host | Copying (8, 4) verbatim as element strides over a 16-element buffer addresses element 8 + 4 = 12, in bounds, with no fault | INT-FNP-02; BD-036 |
| KA-778 | A wheel rolling flat, onto an edge, then onto the top of a riser | The sustained contact step returns its typed refusal through the airlock at the pin; the same transition completes under witness migration with the energy ledger closed to its stated residual | the ledger's residual bound | G1 · referee | A set of 22 contacts (capacity 21) returns the capacity refusal `FROB-REF-721` | INT-FSIM-07, INT-FSIM-04; BD-035, BD-093 |
| KA-779 | `frob-units` aliases against `fs-qty` (only if the escape hatch opens) | Every alias product agrees in a differential test while both crates exist; a round trip through the airlock conversion preserves the value bits; adding a pressure to a time fails to compile | exact | G0 · host | Adding a torque to an energy must fail to compile although both have the dimension vector (kilogram 1, metre 2, second −2) | INT-TOOLCHAIN-01; BD-031 |
| KA-780 | A 2,000-step G1 referee rollout submitted through the worker process `frob-fsim-worker` | Chunk digests equal, bit for bit, to the same job run in process; the same after killing the worker mid-job and resubmitting; fixed overhead at most 50 ms per job and at most 5% of wall time on jobs of one second or more (TARGET) | exact for digests; overhead as stated | G5 · referee | A worker built at a different commit refuses the job (`FROB-REF-791`) | INT-FSIM-18; BD-038 |

**Derivation files.** For each row, a short program or a text derivation that needs no FrankenRobotics code:
KA-769 evaluates Φ by the complementary error function of the deterministic elementary-function set; KA-775
prints the three numbers 1.2·10⁻¹², 3·10⁻¹⁵ and 2⁻⁴⁸ and shows the rounding; KA-777 enumerates the four indices
of both views. A derivation file is never regenerated from the candidate implementation.

**Controls.** Each `Negative control` cell becomes one record in `controls.toml` with a stable `NC-` identifier,
`seed` naming the KA, the gates that must catch it, `kind` (`mutation` for the sibling-defect rows KA-773 to
KA-775 and KA-777, `planted-input` for KA-767, KA-768, KA-769, KA-770, KA-772, KA-778 and KA-780) and `expect`
with the wrong value quoted above.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers/suite.toml` holds exactly fourteen records, KA-767 to KA-780, and validates against the registry schema.
- [ ] Each record has a derivation file that reproduces its truth with the standard library alone, and a `runs_in` path that exists once the carrying bead lands; until then the row's status is `not-run`, never `pass`.
- [ ] Thirteen negative-control records exist (every row; KA-776's is the one set here) and each names at least one gate.
- [ ] KA-770's tolerance is recorded as 10⁻¹² relative with the fixture a = (1, 2), σ = (0.03, 0.02) mm, so that no row of this chapter remains `to set`.
- [ ] The completeness test (below) passes: no row without a carrying test, no control without a gate.
- [ ] The registry is listed by `frob --json` wherever the conformance harness reports known answers, with the tier and the status of each row.

## Test plan

### Unit tests (`cargo test -p frob-conformance suite_registry`)
- `suite_ka_rows_are_fourteen_and_contiguous`: identifiers 767 to 780, no gap, no duplicate.
- `suite_ka_derivations_reproduce`: each derivation program runs and prints the truth: Φ(−1/√2) = 0.23975006109347677 to 10⁻¹⁵; 2/π = 0.6366197723675814; element index 3 and the tuple (1, 0, 3, 2); (cos 32, sin 32) = (0.83422336, 0.55142668) to eight digits; 0.0009 + 0.0016 = 0.0025 and √0.0025 = 0.05.
- `suite_ka_rounding_argument`: in `f64`, 32.0 + 3e-15 == 32.0 is true, and 2⁻⁴⁸ > 3e-15 is true.
- `suite_ka_every_row_has_control_and_gate`: a join of the three registries (known answers, controls, gates) with no unmatched row (the registry-seed completeness obligation PO-GAUNT-03, restricted to this chapter).
- `suite_ka_ids_append_only`: a planted registry diff that renumbers KA-770 fails.

### End-to-end script and logging
No script of its own: the rows are exercised by `scripts/e2e/stack_receipt.sh` (KA-767, bead N33a-27),
`scripts/e2e/admission_m0.sh` (rows due at M0, bead N33a-28) and the scripts of the carrying beads. Each of those
scripts logs, per row, an `assert` line with the KA number, the measured value with its unit, the tolerance, the
tier and the control's `NC-` identifier with `caught: true|false`, in the shared NDJSON format of `STD-E2E`.

### Known answers and negative controls
The table above is the content. Metamorphic relation that applies to the registry itself: reordering the records
in the file changes no test outcome. Determinism lanes: the derivation programs print identical bytes twice on one
machine, in debug and release, and on Linux and macOS.

## Considerations

- KA-773 to KA-775 and KA-777 quote values reported from a source reading on 2026-11-21 that was not re-executed. The admission tests record what the pinned commit really does; if a sibling has since been fixed, the control is still a valid planted mutation and the record notes "fixed upstream at <rev>".
- KA-779 only runs if the `frob-units` escape hatch opens (the units crate fails on the chosen toolchain and the upstream change cannot land within a toolchain train). Until then its status is `not-run` with the reason, which is not a pass and does not block an exit.
- KA-772 is an evaluation-tier row at M6 and is off every critical path.
- The alias count gate of the units escape hatch (at most 64 aliases, TARGET) is OPEN-SUITE-4; it is carried with BD-031, not here.

## References

Appendix M.32 (KA-767 to KA-780); §33.5, §33.9, §33.11, §33.12, §33.14; INT-FSIM-05, -06, -07, -13 to -18,
INT-FNP-02, INT-TOOLCHAIN-01; PO-GAUNT-03; FROB-REF-617, -721, -790, -791.
