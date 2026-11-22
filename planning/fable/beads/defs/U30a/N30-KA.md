---
key: "N30-KA"
title: "Ground known-answer registry: KA-721 to KA-736, the synthetic rehearsal's truths with tolerances, tiers and controls"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "ground", "frob-ground", "registry", "test", "frontier"]
parent: "WP-68"
deps: ["STD-TEST"]
needs: ["§30.21: the synthetic rehearsal generator that plants these truths (BD-388)", "§30.24: the ninth alias (exposure delay against mount translation) and its gate GATE-GROUND-11, which has no Appendix M row; the unit that owns §30.24 states its values"]
related: ["BD-388", "BD-139", "BD-389", "BD-390", "BD-391", "BD-392", "BD-393", "BD-394", "BD-395"]
gates: []
plan_refs: ["Appendix M.29", "§30.7", "§30.8", "§30.9", "§30.11", "§30.12", "§30.13", "§30.14", "§30.21", "§30.22"]
---
## Background

Ground (crate `frob-ground`, layer 5) is the subsystem that compares simulation with measured reality. Nothing in
the project has been compared with a physical robot, so the correctness of Ground's machinery is decided without
real data, on a **synthetic rehearsal**: a truth simulator (the referee tier with planted parameters and one planted
difference of model form) emits logs through the sensor models with planted clock skew and offset, and Ground must
recover what was planted. Everything the truth simulator produces carries `origin = "synthetic"`; recovering planted
values shows that a bounded loop is sound and nothing more, and the Reality claim stays `no-claim`.

A known answer (KA) is a case whose truth is known before the code runs: an analytic value, an arithmetic identity
or a planted parameter. This registry is the single list of the sixteen known answers of the Ground chapter. Each
mechanism bead cites the rows it implements and repeats the values it needs; this bead exists so that the rows are
enumerable, so that each has a negative control, and so that the conformance harness can assert that every row has a
test that names it. A gate that nothing can fail validates nothing: where the plan states no negative control, this
registry proposes one (marked DESIGN) so that every row has a planted defect its test must catch.

Tier names: G0 laws and self-tests; G1 known answers. Execution sites: `host` (exact or double-precision arithmetic
in a test binary), `referee` (the trusted CPU tier, here driven by the synthetic truth simulator), `daemon` (`frobd`).

## Scope

In: the sixteen rows with case, truth, tolerance, tier, site and negative control; the supporting arithmetic the
chapter states without a KA number; the registry file and its completeness test. Out: the implementations (each row
names its bead); the counterexample registry `N30-CX` and the refusal registry `N30-REF` (written by the unit that
owns the chapter's second half).

## Technical approach

**Registry file.** `crates/frob-conformance/registry/known_answers/ground.toml` (DESIGN), one table per row: `ka`,
`section`, `case`, `truth`, `tolerance`, `tier`, `site`, `negative_control`, `test` (the path and name of the test
that asserts it), `bead`. A test `ground_ka_registry_complete` fails if a row has no `test`, if a named test does not
exist, or if a row's negative control is not registered in the negative-control registry of `STD-TEST`.

**The rows.**

| KA | Case | Truth (formula or value) | Tolerance | Tier · site | Negative control | Carried by |
|---|---|---|---|---|---|---|
| KA-721 | Clock recovery. Planted skew s = 100 ppm, offset a = 3.7 ms, mark jitter σ_m = 50 µs, two marks 10 s apart | Skew uncertainty σ_s = √2·σ_m/(t₂ − t₁) = 7.07 ppm; recovered skew within 3·σ_s = 21 ppm of 100 ppm; aligned instants cover the truth; in at least 99% of 1,000 resamples (GATE-GROUND-02) | as stated | G1 · referee (synthetic) | One pulse only: refused (FROB-REF-876). An offset fitted to the scored event: detected | N30a-11, N30a-13, N30a-14 |
| KA-722 | The eight structural aliases (armature against link inertia; Coulomb friction against a bias torque; viscous friction against back-EMF damping; torque constant against the scale of inertia and friction; delay against clock offset; friction coefficient against the torque limit; static friction with no dwell at zero velocity; contact stiffness against damping) | Each flagged, 8 of 8, with the identified combination named (GATE-GROUND-03) | exact | G1 · host | A fit request on each aliased set is refused (FROB-REF-879) | N30a-15, BD-390, N30a-19 |
| KA-723 | Friction against the torque limit: drawbar force F = min(k_t·I/r, μ·N), k_t = 0.2 N·m/A, r = 0.06 m, μ ∈ {0.4, 0.8} | At 1 A under 30 N and 2 A under 40 N: F = 3.33 N and 6.67 N for both μ (torque-limited: k_t·I/r = 3.333, 6.667 N against μ·N = 12, 16, 24, 32 N). At 3 A under 10 N: k_t·I/r = 10 N against μ·N = 4 N and 8 N, so F = 4 N and 8 N: separated | to the printed digits | G1 · host | The torque-limited data alone yields no μ | N30a-15, BD-390 |
| KA-724 | Actuator recovery of J_a, b, τ_c, τ_s, v_s and d on a single-joint synthetic rig | Each within 10% of the planted value; delay within one grid step | 10%; one grid step | G1 · referee (synthetic) | Unidirectional data: τ_c is refused | N30a-21 to N30a-23, N30a-26 |
| KA-725 | Delay against offset: actuator delay 8 ms, clock offset −8 ms, no shared hardware mark | Reported as "delay plus offset = 0" | exact | G1 · host | A card delay update is refused | N30a-13, N30a-15 |
| KA-726 | Omitted law: the truth has a Stribeck term the fit class lacks | The candidate is rejected at validation on sealed data | exact (verdict) | G1 · referee (synthetic) | The same fit scored on its own calibration data passes (which shows why sealing matters) | N30a-28, N30a-32, N30a-26 |
| KA-727 | Wrong reason: speed matched by a compensating error in k_t and friction | The energy cross-check marks the fit `wrong-reason` | exact (label) | G1 · referee (synthetic) | None stated. DESIGN: the same fit with the energy check disabled is offered for admission and the test must fail | N30a-24 |
| KA-728 | Band coverage: exchangeable residuals, n = 20, target 0.90, 1,000 resamples | Quantile index ⌈21·0.9⌉ = 19; realized coverage ~ Beta(19, 2): mean 19/21 = 0.905, standard deviation 0.063, P(coverage < 0.80) = 0.8¹⁹·(20·0.2 + 0.8) = 0.069 | mean within 0.01 of 19/21; the fraction below 0.80 within 0.03 of 0.069 (GATE-GROUND-06) | G0 · host | Pooled shifted regime: 16 of 20 | N30a-36, N30a-39 |
| KA-729 | Band refusal: an unbuilt design; a cell with 8 points; a new unit with 3 units calibrated | Refused in each case (FROB-REF-895; 889; 889) | exact (verdict) | G1 · host | None stated. DESIGN: a build with the minimum-count guard removed issues a band for the 8-point cell and the test must fail | N30a-37, N30a-39 |
| KA-730 | Drift: friction raised by 30% after episode k | The calibration is retired within 20 further episodes (in at least 90% of resamples, GATE-GROUND-07) | as stated | G0 · referee (synthetic) | Null runs: false retirement at most 5% over 10⁴ runs | N30a-41, N30a-42, N30a-44 |
| KA-731 | Miss-monitor arithmetic at n = 20, p₀ = 0.22, p₁ = 0.5 | A miss multiplies the evidence by p₁/p₀ = 2.27, a cover by (1 − p₁)/(1 − p₀) = 0.641; four straight misses give 26.7 (retire, threshold 20); three give 11.7 (do not) | to the printed digits | G0 · host | None stated. DESIGN: a monitor that uses p₀ = 0.10 (the nominal miss rate) retires after two misses (e = 25) and must fail the null-run bound | N30a-42 |
| KA-732 | Reachability: zero failures in 20 real trials | One-sided 95% upper bound on the failure probability 1 − 0.05^(1/20) = 13.9%; a claim of "under 5%" is unreachable below 59 failure-free trials | to the printed digits | G0 · host | A registered claim of "under 5%" at n = 20 is refused at registration | N30a-33 |
| KA-733 | As-built registration: a planted rotation and translation; a 0.3 mm wall deviation | Recovered inside the stated uncertainty | inside stated uncertainty | G1 · host | Two fiducials: refused (FROB-REF-812). Scale 25.4: unit suspicion raised | N30a-45, N30a-46, N30a-48 |
| KA-734 | Protocol: read before freeze; refit without a new version; holdout reuse | Each refused (FROB-REF-883, 883, 886) | exact (verdict) | G1 · daemon | None stated. DESIGN: a store build with the sealed-area guard disabled must make the test fail | N30a-31, N30a-34, N30a-35 |
| KA-735 | The `reality:` line (GATE-GROUND-01) | Printed on every report; `no-claim` with no dataset; a negative battery of 50 planted misuses is refused in full | exact | G1 · host | The 50 planted misuses | N30a-02 (type and grammar); BD-139 (every report, the battery) |
| KA-736 | Pooled conformal calibration at a 90% target across two regimes | 20 of 20 covered in one regime and 16 of 20 in the shifted one: 36 of 40 = 90% overall, 80% where it matters | exact (count) | G0 · host | Pooling across cells (refused, FROB-REF-814) | N30a-37 |

**Supporting arithmetic stated in the chapter without a KA number** (each is asserted by a host test in the bead
named; DESIGN: they are proposed as additional registry rows `ground.aux.*`).

| Value | Derivation | Bead |
|---|---|---|
| Two clocks 100 ppm apart that coincide at the start differ by 0.8 ms after 8 s | 100·10⁻⁶·8 s | N30a-11 |
| σ_s = 7.1 ppm at σ_m = 50 µs, marks 10 s apart | √2·50·10⁻⁶/10 | N30a-11 |
| σ_t(t)² = σ_m²·[(t₂ − t)² + (t − t₁)²]/(t₂ − t₁)²: σ_m² at either mark, σ_m²/2 at the midpoint, growing quadratically outside | Linear interpolation of two independent marks | N30a-11, N30a-12 |
| Crossing predicted at 4.212 s, logged at 4.180 s: fitting the offset to the event reports error 0 and offset 32 ms | 4.212 − 4.180 | N30a-13 |
| Finite band needs n ≥ (1 − α)/α: 9 at 90%, 19 at 95%, 99 at 99%; at n = 20 a 95% band is the largest residual and any level above 20/21 = 95.2% is unbounded | ⌈(n+1)·(1−α)⌉ ≤ n | N30a-36 |
| n = 200, target 0.90: coverage ~ Beta(181, 20), mean 0.900, standard deviation 0.021, P(coverage < 0.80) below 0.001 | l = ⌊201·0.1⌋ = 20 | N30a-36 |
| Rank-monitor slack δ = max(1/√n, 0.02): 0.224 at n = 20, 0.071 at n = 200 | — | N30a-41 |
| Miss-monitor null rate p₀(n), the 95th percentile of Beta(l, n + 1 − l): about 0.22 at n = 20 (Beta(2, 19)), about 0.14 at n = 200 (Beta(20, 181)) | 1 − (1 − x)¹⁹·(1 + 19·x) = 0.95 at x ≈ 0.216 | N30a-42 |
| Zero failures in n trials: upper bound 13.9% at n = 20, 9.5% at n = 30, 4.95% at n = 59 | 1 − 0.05^(1/n) | N30a-33 |
| Interval score S = (u − l) + (2/α)·(l − y)₊ + (2/α)·(y − u)₊: for [4.05, 4.42] at level 0.90, y = 4.20 gives 0.37 and y = 4.50 gives 0.37 + 20·0.08 = 1.97 | α = 0.10 | N30a-32 |
| Ceiling of paired simulation on real-trial count: about 1.3 | 1/(1 − 0.20) to 1/(1 − 0.25) | N30a-38 |

## Acceptance criteria

- [ ] `ground.toml` holds sixteen rows, KA-721 to KA-736, each with a truth, a tolerance, a tier, a site, a negative
      control (the plan's, or the DESIGN control above where the plan states none) and a test path.
- [ ] `ground_ka_registry_complete` passes, and fails when any one row's `test` is deleted (its own control).
- [ ] Every host-site row (KA-723, -725, -728, -729, -731, -732, -736 and the auxiliary rows) is asserted by a test
      that needs no simulator and runs in under one second.
- [ ] Every referee-site row runs on data whose manifest says `origin = "synthetic"`, and every envelope it
      produces prints `reality: synthetic` or `reality: no-claim`, never another status.
- [ ] Each negative control is registered with `STD-TEST` and is caught by the test of its row.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ground_ka_registry.rs`: `ground_ka_registry_complete`;
`ground_ka_rows_are_sixteen`; `ground_ka_arithmetic` (recomputes in exact rationals or double precision: 19/21;
0.8¹⁹·4.8 = 0.0692 within 10⁻⁴; (0.5/0.22)⁴ = 26.68 and (0.5/0.22)³ = 11.74; 1 − 0.05^(1/20) = 0.1391;
1 − 0.05^(1/59) = 0.0495 and 1 − 0.05^(1/58) = 0.0503; √2·50·10⁻⁶/10 = 7.07·10⁻⁶; 0.2/0.06 = 3.333;
min(10, 4) = 4 and min(10, 8) = 8); `ground_ka_negative_controls_registered`.

### End-to-end script and logging

No script of its own. The scripts that assert the rows are `scripts/e2e/ground_clock.sh` (N30a-14),
`ground_excite.sh` (N30a-19), `ground_identify.sh` (N30a-26), `ground_protocol.sh` (N30a-35), `ground_band.sh`
(N30a-39), `ground_drift.sh` (N30a-44) and `ground_asbuilt.sh` (N30a-48). Each logs, per row it asserts, an `assert`
line carrying the KA number, the measured value with its unit, the truth, the tolerance and the outcome, and a
`metric` line for each control with the planted defect and whether it was caught.

### Known answers and negative controls

This bead is the registry. Metamorphic relations that apply across rows: scaling every residual of a calibration set
by a positive constant scales q̂ by the same constant and leaves coverage counts unchanged (KA-728); permuting
calibration episodes leaves q̂ unchanged (KA-728) and leaves the miss monitor's value unchanged (KA-731);
re-expressing clock marks in another time unit leaves skew unchanged (KA-721). Determinism lanes: host rows are
class D0 or exact in double precision and are run twice, in debug and release, on both development machines;
referee rows are run twice on one machine with a pinned seed and compared bit for bit, and across machines inside
the recorded envelope (the ensemble lane used by KA-724 makes no cross-machine bit claim).

## Considerations

- Appendix M counts 16 rows for this chapter, 4 of them with "none stated" as the negative control (KA-727, -729,
  -731, -734). The controls proposed here are DESIGN and should be confirmed by the Ground lead before the gates
  that cite them are recorded.
- GATE-GROUND-11 (the exposure-delay and mount-translation alias) has known-answer values in its gate row (40 mm,
  80 mm, 20 ms) and no KA number; it is listed under `needs` so that it is not lost.
- The gate register (§40) places GATE-GROUND-03 at M5 with the note that it may be recorded at M4; KA-722 and
  KA-723 therefore have host tests that can run as soon as the analysis exists.
- Risk: a known answer on synthetic data can be mistaken for evidence about reality. Every row's test asserts the
  `synthetic` origin, and no row is ever cited for the Reality claim (invariant INV-GROUND-06: data of origin
  `synthetic` never yields a status other than `synthetic`).

## References

Appendix M.29 (KA-721 to KA-736); §30.21 (the rehearsal table); §30.22 (gates GATE-GROUND-01 to -08); §30.7, §30.8,
§30.11, §30.12, §30.13 (the arithmetic); INV-GROUND-06; PO-GROUND-01, -02, -03.
