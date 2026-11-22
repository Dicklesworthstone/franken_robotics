---
key: "N38-KA"
title: "frob-conformance: known-answer registry rows adopted by the Gauntlet: five corrected fixtures, nine late fixtures"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-14"
deps: ["BD-025", "N38a-12", "STD-TEST", "STD-E2E"]
needs: ["Appendix M: the registry beads of the owner sections (actuators, traction, the algebra core, mission synthesis, the first fixture, learning, numerics of views, shields, sensing), whose mechanisms these rows test", "§38.6: the rule that numbers are append-only; the fourteen numbers below are reserved after KA-835 and are confirmed by the registry at first commit"]
related: ["N38-CX", "N38b-23", "N38b-39", "N38b-40", "N38b-41", "N38b-42", "N38b-43", "N38b-44", "N38b-45", "N38b-46"]
gates: ["GATE-FID-09", "GATE-GAUNT-11", "GATE-GAUNT-02"]
plan_refs: ["§38.17", "§38.18", "§38.6", "Appendix M.1", "§15.7", "§18.10", "§13.4", "§25.21.4", "§24.10", "§23.9", "§21.9", "§33.11", "§22.12.5", "§18.19", "§19.10.3", "§20.14"]
---
## Background

The known-answer suite of FrankenRobotics is a registry of cases whose truth is known without the system under
test; each row has a derivation on file, a tolerance, a tier and site, and a negative control (the mutation or
planted failure that must make the row fail). The printed suite ends at KA-835 and has no section for the
verification subsystem's own chapter. That chapter adopts fourteen further rows, owned by the conformance
crate and run under the verbs of the subsystems they test: five from *corrected fixtures* (cases whose first
printed values were wrong or whose shortcut had gone unnoticed; gate GATE-FID-09) and nine from *late fixtures*
(the smallest cases that distinguish a mechanism from its tempting shortcut; gate GATE-GAUNT-11). Every value
is derived on file from the stated inputs; none is a measurement. One row quotes a recorded planning pilot
(FACT, to be re-run before a gate cites it).

This bead enters the fourteen rows, with their derivations in enough detail to write the derivation files, so
that no row depends on the planning documents. Numbers KA-836 to KA-849 are reserved in table order.

## Scope

In: fourteen `[[ka]]` records, fourteen derivation files, the loader test that checks the arithmetic rows on
the host. Out: the tests through each subsystem's verb at each row's milestone (N38b-39 to N38b-45); the
end-to-end script (N38b-46); the mechanisms themselves.

## Technical approach

Site abbreviations: `host` (exact or double-precision arithmetic in a test binary), `referee`, `strict` (the
CPU batch tier under its transactional profile), `cuda`, `metal`, `auditor` (the offline certificate checker).

**Corrected fixtures (GATE-FID-09: times to 0.1 s; sinkage to 10⁻⁵ m in double precision; counts exact;
admissions exact; every negative control fails).**

| KA | Case and inputs | Truth, with derivation | Tolerance | Tier · site | Negative control | Due |
|---|---|---|---|---|---|---|
| KA-836 | Thermal runaway, second card. Constant current I = 25.4 A; winding resistance R₀ = 0.54 Ω referenced to T₀ = 20 °C; temperature coefficient α = 0.00393 K⁻¹; heat capacity C = 140 J/K; conductance to ambient G = 1/2.2 W/K; ambient and initial winding temperature 25 °C; limit 120 °C | Loss I²·R₀·(1 + α·(T − T₀)): 348.4 W at 20 °C, 485.3 W at 120 °C. With θ = T − T_amb the model is C·θ̇ = P_amb + (s − G)·θ, where s = I²·R₀·α = 1.369 W/K and P_amb = I²·R₀·(1 + α·5) = 355.2 W. s exceeds G = 0.4545 W/K, so there is no equilibrium; growth rate λ = (s − G)/C = 0.006533 s⁻¹; θ(t) = (P_amb/(s − G))·(e^(λt) − 1) reaches 95 K at t = ln(1 + 95·(s − G)/P_amb)/λ = 33.49 s. Regearing from ratio 12 to 16 at equal output torque scales current by 12/16: copper loss falls by 1 − (12/16)² = 43.75%, s falls to 0.770 W/K, still above G, rate 0.002254 s⁻¹: the runaway remains. Ratio 22: s = 0.407 W/K, below G, rate −0.000337 s⁻¹, yet the equilibrium rise is P_amb/(G − s) = 2,240 K: the winding is at 58.7 °C after 45 s and at the limit after 128.6 s, so the sign of the rate decides nothing without the duty time | 0.1 s; 0.1 W; 0.1 K; rates to four significant figures | G1 · host, referee | Reading R₀ as referenced to 25 °C gives 34.08 s; the constant-resistance model (resistance frozen at its 25 °C value) gives 39.92 s; both fall outside 0.1 s | M1 |
| KA-837 | Plate sinkage, power law p = (k_c/b + k_φ)·zⁿ with n = 0.5. k_c = 0.41 kN/m^1.5; k_φ = 21.6 kN/m^2.5; two tracks of length 0.55 m and width b; g = 9.81 m/s² | z = (p/(k_c/b + k_φ))^(1/n) with p = m·g/(2·0.55·b). 45 kg on 8 cm tracks: p = 5.017 kPa, modulus 26.725, z = 3.52 cm. 75 kg on 8 cm: p = 8.361 kPa, z = 9.79 cm, more than an 8 cm belly clearance. 75 kg on 12 cm: p = 5.574 kPa, modulus 25.017, z = 4.96 cm | 10⁻⁵ m in double precision | G1 · host, strict | The same constants with n = 0.7 give 9.17, 19.01 and 11.71 cm: a mistranscribed exponent is caught | M4 |
| KA-838 | Contact strata under the observation subgroup. Four feet, each open, stick or slip (3⁴ = 81 states; 81·8 = 648 one-foot transitions) | Under the four-element leg-permutation group (identity, left–right swap, front–back swap, both): by orbit counting (81 + 9 + 9 + 9)/4 = 27 strata and 648/4 = 162 transitions. Under the two-element subgroup that a forward-facing sensor leaves: (3⁴ + 3²)/2 = 45 strata and 648/2 = 324 transitions | exact (integers) | G1 · host | Using the mechanical group where the observation subgroup is required collapses 45 strata to 27 | M1 |
| KA-839 | Reality of roots at saturation. A Stewart–Gough platform instance with 40 real assembly modes; a serial six-revolute instance with 16 real inverse-kinematics solutions (FACT, literature: Dietmaier 1998; Manseur and Doty 1989; to be re-read) | The continuation lane certifies 40 of 40 and 16 of 16 roots as real by the conjugate-ball test. A generic instance of each family has the same complex count, and its real count is whatever the test certifies | exact (counts) | G1 · host (exact rational evaluation) | A checker that calls a root real when its imaginary part is below a tolerance, instead of by ball disjointness, is caught on a planted near-real conjugate pair | With the continuation lane (M8) |
| KA-840 | A single wheel at a sharp lip. Axle load W = 25 N; lip height h = 12 mm; pure drive couple M; a = √(2·r·h − h²), b = r − h; lift-off needs M = W·a and friction μ ≥ a/b | r = 60 mm: a = 36 mm, b = 48 mm, M = 0.900 N·m, μ ≥ 0.7500. r = 75 mm: a = 40.69 mm, b = 63 mm, M = 1.017 N·m, μ ≥ 0.6459. With μ = 0.70 and a 1.2 N·m cap only the 75 mm wheel is admitted; with μ = 0.80 and a 0.95 N·m cap only the 60 mm wheel is. General rule with a forward axle push P and couple M: lift-off b·P + M = W·a; corner reaction a·P + b·W ≥ 0; friction \|M\| ≤ μ·(a·P + b·W) | exact (admissions); torques and ratios to four significant figures | G1 · host, referee | A check of torque alone admits both wheels in the first condition | M1 |

**Late fixtures (GATE-GAUNT-11: single-precision rows bit-exact on every backend; rational rows exact; the root
row to four significant figures in exact rational evaluation of α and γ; every negative control fails).**

| KA | Case and inputs | Truth, with derivation | Tolerance | Tier · site | Negative control | Due |
|---|---|---|---|---|---|---|
| KA-841 | Moment merge in single precision. Two equal-size shards of constants: 4,095 and 4,097; −1 and +1; 8,191 and 8,193 | Population variance 1 in each case by the centred merge of per-shard triples (n, mean μ, sum of squared deviations S): n = n_a + n_b, d = μ_b − μ_a, μ = μ_a + d·n_b/n, S = S_a + S_b + d²·n_a·n_b/n; here S_a = S_b = 0 and d = 2, so S/n = 1 | bit-exact on every backend | G1 · strict, cuda, metal | The raw second moment minus the squared mean returns 0 for the first and third pairs, because 4,096² + 1 = 16,777,217 and 8,192² + 1 = 67,108,865 are not representable in single precision, and 1 for the second | M2 |
| KA-842 | Unresolved outcomes in a comparison. Baseline: 60 successes in 100 valid trials. Candidate: 2 successes in 2 valid trials and 98 unresolved | Complete-case rate 100%; success rate anywhere in [2%, 100%]; verdict: no preference; unresolved counts are reported per arm, and the unresolved worlds are re-run on the referee before a ranking that depends on them is promoted | exact | G1 · host | A ranking on complete cases prefers the candidate | M1 |
| KA-843 | Independence of a writable view. A 2×2 view with element strides (1, 1) | Index [0, 1] and index [1, 0] address one element (offset 1); stores of 7 and 11 leave 7 or 11 by order; the view is refused for writing (shared elements are admitted for reads; a write needs an injective index map, a declared reduction or separate storage) | exact (verdict) | G1 · host, cuda, metal | A check of address bounds alone admits it | M2 |
| KA-844 | Reality of a certified root. f(z) = z³ + z at z = 0.001 + 1.001·i. Newton quantities: β = \|f/f′\|, γ = max over k ≥ 2 of \|f⁽ᵏ⁾/(k!·f′)\|^(1/(k−1)), α = β·γ | α(f, z) = 0.0021 (β = 0.001412, γ = 1.497), below (13 − 3√17)/4 ≈ 0.1577, so a root near z is certified: it is +i. α(f, Re z) = 0.0010 certifies a root near Re z = 0.001: it is 0, a different root. ‖z − Re z‖ = 1.001 exceeds 1/(20·γ(f, z)) = 0.033, so the robust reality test (α < 0.03 and ‖z − Re z‖ < 1/(20·γ)) does not call the root real | four significant figures, in exact rational evaluation of α and γ | G1 · host, auditor | A test that infers reality from α(f, Re z) alone calls +i real | M8 |
| KA-845 | The Cayley map on the rigid-motion group. ω = (0, 0, 2), v = (1, 0, 0); cay(x) = (I − x/2)⁻¹·(I + x/2) | Rotation cay(ω̂): a quarter turn about z, the matrix [[0, −1, 0], [1, 0, 0], [0, 0, 1]]. Translation (I − ω̂/2)⁻¹·v = (1/2, 1/2, 0) | exact (rationals) | G1 · host | The map that pairs cay(ω̂) with v unchanged returns (1, 0, 0) and fails the differential test (the derivative of the map compared with finite differences of the group action) | M0 |
| KA-846 | Tube erosion with a one-sided error. One dimension; a cell [0, 1] of measured states; true state = measured + e with e in E = [0, 0.1]; A ⊖ B = {x : x + B ⊆ A} | Admissible true successors are [0.1, 1.0], the erosion by −E (every measurement x − e of a true x must lie in the cell) | exact (rationals) | G1 · host | Erosion by E gives [0, 0.9]: it admits 0.05, whose measurement can be −0.05, and rejects 0.95, whose measurements lie in [0.85, 0.95] | M8 |
| KA-847 | Residual bound in single precision. H = 3, b = 1 (one row; unit roundoff u = 2⁻²⁴) | x̂ = 0.33333334; the computed residual b − H·x̂ is exactly 0; the true error is 9.93 × 10⁻⁹. Rounding margin ρ = γ·(\|H\|·\|x̂\| + \|b\|) with γ = (n + 1)·u/(1 − (n + 1)·u), n = 1: ρ = 2.4 × 10⁻⁷; bound (‖r̂‖ + ρ)/λ_lo = 7.9 × 10⁻⁸ with λ_lo = 3, which contains the true error | bit-exact for x̂ and the residual; bound to two significant figures | G1 · strict, cuda, metal | The bound without ρ claims zero error | M4 |
| KA-848 | Film front at a dry cell. Two cells with film thickness h = (1 mm, 0); gravity spreading flux ∝ h_f³·∇h with face mobility h_f³ | Arithmetic mean of the cubes: h_f³ = ½·(h_L³ + h_R³) = 0.5 mm³, so volume crosses the face and the planar front follows t^(1/5) | exact (mobility); exponent within 2% of 1/5 | G1 · host, strict | A harmonic or geometric mean of the cubes gives zero flux, and the spill never reaches the dry cell | M6 |
| KA-849 | One edge of the extrinsic calibration graph. Predicted pose of sensor 2 in sensor 1: a translation of (1, 0, 0) m. Sensor 1's mount is off by a rotation δ₁ of 0.01 rad about z; sensor 2's is exact. Residual r₁₂ ≈ δ₂ − Ad(T̂₂₁)·δ₁ with T̂₂₁ the inverse of the predicted pose; for a twist (ω, v), Ad(R, p)·(ω, v) = (R·ω, p × R·ω + R·v) | r₁₂ = −Ad(T̂₂₁)·δ₁ has angular part (0, 0, −0.01) rad and linear part (0, −0.01, 0) m: sensor 2 appears at (1, −0.01, 0) m | exact to first order (rationals) | G1 · host, referee | Using Ad(T̂₁₂) on the tail gives a linear part of (0, +0.01, 0) m | M3 |

FACT for KA-841 (a recorded planning pilot; to be re-run before a gate cites it): a native single-precision
kernel on the Metal device and on each RTX 4090 returned 0 by the raw form and 1 by the centred merge for the
pair at 4,095 and 4,097, 1 by both for the pair at ±1, and 0 by the raw form at 8,192. Each device merged
locally; nothing about communication between devices or training throughput follows.

**Records and files.** One `[[ka]]` record per row in `crates/frob-conformance/registry/known_answers.toml`
(`section`, `fixture`, `case`, `truth`, `tolerance`, `tier`, `sites`, `control`, `derivation`, `label`, `due`);
one derivation file per fixture under `docs/derivations/` stating the model, the steps above, the digits and
the error budget; each control is registered as an NC record naming its gate.

## Acceptance criteria

- [ ] Fourteen records exist with numbers KA-836 to KA-849 (or the next free block, reported), each eligible:
      derivation on file, tolerance set, control registered.
- [ ] The host test reproduces every arithmetic value above at its tolerance.
- [ ] Each negative control's wrong value is reproduced by the test (34.08 s and 39.92 s; 9.17, 19.01, 11.71
      cm; 27; (1, 0, 0); [0, 0.9]; zero bound; zero flux; +0.01) and differs from the truth by more than the
      tolerance.
- [ ] KA-839 and KA-841 carry the label FACT with their sources and "to be re-run or re-read".
- [ ] The two gates' rows cite these records and no others.

## Test plan

### Unit tests

`crates/frob-conformance/tests/adopted_rows_host.rs` (exact rationals where the truth is rational; `f64`
otherwise; `f32` for the single-precision rows):
- `ka_836_thermal_runaway_times`: 33.49 s; rates 0.006533, 0.002254, −0.000337 s⁻¹; 58.7 °C at 45 s; 128.6 s.
- `ka_836_controls`: 34.08 s and 39.92 s.
- `ka_837_sinkage`: 0.03523 m, 0.09787 m, 0.04964 m; `ka_837_exponent_control`: 0.0917, 0.1901, 0.1171 m.
- `ka_838_strata_counts`: 27 and 162; 45 and 324 (by enumeration of the 81 states, not by formula).
- `ka_840_wheel_lip`: 0.900 and 1.017 N·m; 0.7500 and 0.6459; the two admission tables.
- `ka_841_moment_merge_f32`: 1.0 three times; raw form 0, 1, 0.
- `ka_842_unresolved_bounds`: [0.02, 1.00]; no preference.
- `ka_843_view_alias`: offsets of [0, 1] and [1, 0] are equal.
- `ka_844_alpha_values`: α = 0.002114, γ = 1.4970, 1/(20γ) = 0.03340, α at the real part 0.0010000.
- `ka_845_cayley_se3`: the rotation matrix and (1/2, 1/2, 0), in rationals.
- `ka_846_erosion`: [1/10, 1]; control [0, 9/10].
- `ka_847_residual_bound_f32`: x̂ bits 0x3EAAAAAB; residual 0; error 9.934 × 10⁻⁹; ρ = 2.384 × 10⁻⁷; bound
  7.947 × 10⁻⁸.
- `ka_848_face_mobility`: 0.5 mm³; harmonic and geometric means 0.
- `ka_849_edge_residual`: (0, 0, −0.01; 0, −0.01, 0); control linear part (0, +0.01, 0).

### End-to-end script and logging

`scripts/e2e/adopted_known_answers.sh` (bead N38b-46) runs the rows through their subsystems' verbs per
milestone and logs each value against its truth and tolerance, with each control's outcome.

### Known answers and negative controls

The rows are the known answers. Negative controls: one per row, as tabulated. Metamorphic relations that
apply: rescaling KA-837's lengths by 2 and pressures by the matching power leaves the dimensionless sinkage
ratio unchanged; mirroring KA-840 changes no admission. Determinism: D0 on the host; KA-841, KA-843 and KA-847
bit-exact on every backend.

## Considerations

- KA-838 runs at M1 on the host through the orbit-counting verb of the algebra core and again under the gait
  atlas when that verb exists (M4).
- KA-839 and KA-844 belong to a Moonshot lane; they land with it and never earlier, and the lane's kill gate
  removes them with the lane.
- The first printed thermal figures of the corrected fixture were wrong in the direction of optimism; the row
  exists so that the corrected derivation is the tested one.

## References

§38.17 (GATE-FID-09), §38.18 (GATE-GAUNT-11), §38.6; owner sections §15.7, §18.10, §13.4, §25.21.4, §24.10,
§23.9, §21.9, §33.11, §22.12.5, §18.19, §19.10.3, §20.14; constructions C-05, C-16, C-34, C-36.
