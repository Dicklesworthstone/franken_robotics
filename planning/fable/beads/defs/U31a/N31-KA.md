---
key: "N31-KA"
title: "Known-answer registry for Foundry: KA-737 to KA-754, related construction answers, derived values and controls"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "foundry", "frob-conformance", "frontier", "registry", "test"]
parent: "WP-70"
deps: ["STD-TEST"]
needs: ["§38.6: the known-answer registry format of the conformance crate (identifier, case, truth with derivation, tolerance, verification tier, negative control, the test that carries it) and its check that every row has a test and a control", "§31.21, §31.22 and §31.23: the known answers stated in those subsections (the tube reproduced from each export; the unit-error ratio 10⁹; the stale-verdict case; the damage ratios 1.61 and 2.59; the cantilevered tube sizing case), whose tests are drafted by unit U31b"]
related: ["BD-400", "BD-401", "BD-402", "BD-403", "BD-404", "BD-405", "BD-406", "BD-407", "BD-408", "BD-423", "BD-438", "N31a-07", "N31a-08", "N31a-09", "N31a-10", "N31a-12", "N31a-16", "N31a-18", "N31a-19", "N31a-20", "N31a-21", "N31a-23", "N31a-24", "N31a-28", "N31a-32", "N31a-33", "N31a-35", "N31a-40", "N31a-41", "N31a-45", "N31a-47", "N31a-48", "N31a-49", "N31-CX", "N31-REF", "SPEC-C-35"]
gates: []
plan_refs: ["§31.18", "Appendix M.30", "§10 C-35", "Appendix D", "Appendix E GB-37", "Appendix E GB-38", "§38.6", "§31.21", "§31.22", "§31.23"]
---
## Background

A known answer is a case whose right result is fixed by a formula or a construction before any code exists,
with a derivation on file. The program's rule is that known-answer tests pass against values that were never
regenerated from the candidate, and that every gate has at least one negative control: a planted defect the
test must catch, because a gate that nothing can fail validates nothing.

This bead is the registry of the known answers of the structure, manufacture and lifecycle chapter (the
subsystem Foundry; crates `frob-foundry` and `frob-lifecycle`). The chapter states that its values are hand
calculations filed under `docs/derivations/`, that fixture dimensions and material constants are DESIGN
choices, and that each row has a negative control. The registry has four parts: the eighteen numbered rows;
the four numbered rows of the structure construction that bind the same tests; the answers stated in the
chapter's later subsections; and the additional derived values the mechanism beads rely on. Mechanism beads
cite these numbers and restate the values they need; this bead is where the whole list lives and where its
completeness is checked.

Verification tiers named below: G1 (a known answer on the stated tier) and G5 (determinism); "host" means the
check runs on the host with no simulation tier involved.

## Scope

In: the registry file `crates/frob-conformance/registry/known-answers/foundry.toml`; one entry per row below
with its derivation reference under `docs/derivations/foundry/`; the mapping from each entry to the test that
carries it; controls for the rows that state none; tolerances for the rows that say "to set"; the completeness
check.
Out: the tests themselves (the beads named in the last column); the counterexample registry and the refusal
registry of the chapter (registry beads of unit U31b).

## Technical approach

**Part 1. The numbered known answers (eighteen rows).**

| KA | The case | The truth | Tolerance | Tier | Negative control | Carried by |
|---|---|---|---|---|---|---|
| KA-737 | Cantilever: L = 0.20 m, section 20 × 10 mm, E = 69 GPa, tip load 50 N | Tip deflection P·L³/(3·E·I) = 1.159 mm (I = b·h³/12 = 1.6667 × 10⁻⁹ m⁴); root stress 6·P·L/(b·h²) = 30 MPa; shear adds about 0.2% | To the printed digits for the closed forms; 2% and 5% for the finite-element path (GATE-FOUNDRY-01) | G1 · host | Halving the load halves both; a millimetre-for-metre unit error is caught by dimension typing | BD-401; N31a-19; N31a-55 |
| KA-738 | Planted overstressed link: the same beam with a root section 4 mm deep, 80 N, yield 240 MPa | Stress 6 × 80 × 0.2/(0.02 × 0.004²) = 300 MPa; utilization 1.25; located at the root | To the printed digits; 1.25 ± 5% on the finite-element path | G1 · host | The unthinned beam passes (48 MPa, utilization 0.20) | BD-401; N31a-19; N31a-55 |
| KA-739 | Two-bar optimum: load P at distance L from a wall, domain height 2L | Bars at 45°; compliance 4·P²·L²/(E·V); minimum volume 2·P·L/σ_allow | The plan says "to set"; set here from GATE-FOUNDRY-04: angle within 5°, compliance within 10% at the extracted volume | G1 · host | Swapping the support and the load port is caught | N31a-18; N31a-15; N31a-54 |
| KA-740 | Assembled peaks: simply supported span L, P at L/3 at one instant and at 2L/3 at another | Mid-span moment P·L/6 per instant; the assembled case gives P·L/3, or 0 with opposite signs | Exact | G1 · host | The assembling mutation is refused (FROB-REF-167) | N31a-10; BD-401; N31a-53 |
| KA-741 | Fatigue block: N(S) = 10⁶·(100 MPa/S)³, fully reversed; 2,000 cycles at 200 MPa, 50,000 at 100 MPa, 400,000 at 50 MPa | Damage 0.016 + 0.050 + 0.050 = 0.116 per block; 8.62 blocks to a sum of 1 | Exact to the printed digits; 10⁻¹² relative in GATE-FOUNDRY-03 | G1 · host | Feeding unsigned stress changes the count (0.029 per block) and is refused | N31a-21; N31a-55 |
| KA-742 | Rainflow sequence −2, 1, −3, 5, −1, 3, −4, 4, −2 in units of 40 MPa, no mean-stress correction | Ranges 3, 4, 6, 8, 9 with counts 0.5, 1.5, 0.5, 1.0, 0.5; damage 8.752 × 10⁻⁶ (amplitude = half the range × 40 MPa on the curve of KA-741) | Exact for counts; printed digits for damage; 10⁻¹² relative in GATE-FOUNDRY-03 | G1 · host | Splitting after the fifth point and finishing each window loses the range-9 half cycle and gives 7.816 × 10⁻⁶, 10.7% low (the split windows count ranges 3, 4, 6, 7, 8 with counts 0.5, 1.0, 1.0, 0.5, 1.0) | N31a-21; N31a-09; N31a-55 |
| KA-743 | Loads–mass loop: M₀ = 12 kg, A = 0.6 kg, λ = 0.15, start 4.0 kg | Iterates 3.0, 2.85, 2.8275, 2.8241, 2.8236; fixed point 48/17 = 2.8235 kg; λ̂ = 0.15 | To the printed digits; 10⁻⁶ and 0.01 in GATE-FOUNDRY-06 | G1 · host | λ = 1.1 is refused as divergent within three iterations; a planted two-size cycle is detected | N31a-24; N31a-05; N31a-54 |
| KA-744 | Mass properties of a tube: ρ = 2,700 kg/m³, radii 15 and 13 mm, length 0.20 m | Mass 0.0950 kg; axial inertia 1.87 × 10⁻⁵ kg·m²; transverse inertia 3.26 × 10⁻⁴ kg·m² | To the printed digits | G1 · host | The volume enclosure must contain the exact volume (3.5186 × 10⁻⁵ m³) | N31a-23; N31a-40; N31a-17; N31a-57 |
| KA-745 | Tolerance allocation: sensitivities 1 and 4, equal cost, k = 3, √B = 10 µm | 16.0 µm and 6.35 µm (15.9904 and 6.3458); equal tolerances 7.28 µm; 19.9% cheaper under the 1/t cost model | To the printed digits; 10⁻⁹ against the closed form in GATE-FOUNDRY-08 | G1 · host | q = x² at nominal zero: refused (FROB-REF-165), then fails the band-extreme check | N31a-32; N31a-33; N31a-56 |
| KA-746 | Voltage drop: copper, 1 m each way, 0.5 mm², 10 A, 24 V bus | 0.0688 Ω; 0.688 V (2.9%); 6.9 W; 20 A/mm² | To the printed digits | G1 · host | Halving the area doubles the drop | N31a-41; N31a-57 |
| KA-747 | Cable slack: offset 15 mm, joint range 2.4 rad | 36 mm | Exact | G1 · host | None stated in the plan. Registered here (DESIGN): the degrees-for-radians mutation gives 2,063 mm | N31a-41; N31a-57 |
| KA-748 | STEP round trip of a tetrahedron | 18 instances; bit-exact replay | Exact | G5 · host | An open shell is refused; a part above the bound is refused with the bound stated | N31a-35; N31a-57 |
| KA-749 | Fixed point, Q15: (−1)·(−1); negation of −32768; an integrator adding 400 counts per tick from zero | 0x8000·0x8000 = 0x40000000 shifts to 0x8000 and reads as −1; the negation overflows; the integrator overflows sixteen bits at tick 82, and without a clamp the one-step query returns the state 32,400 at once. Each counterexample is found; the clamped controller is certified | Exact | G1 · host | A widened input range breaks a certificate | N31a-45; N31a-46; N31a-58; CARD-T-14 |
| KA-750 | Averaged growth: ż = ε·a·x², x = sin ωt | z(T) = ε·a·T/2 over whole periods | 5% at a compression of 50 in steps (the pilot falsifier) | G1 · host | None stated in the plan. Registered here (DESIGN): averaging over a non-integer number of periods without a half-width (0.4% bias at 20.125 periods) must be flagged | N31a-47; N31a-59 |
| KA-751 | Wear feedback: β̇ = ε·(c₀ + c₁·β) | β(t) = (β₀ + c₀/c₁)·exp(ε·c₁·t) − c₀/c₁ | The plan says "to set"; set here to 5% at a compression of at least 50, as for KA-750 and GATE-FOUNDRY-12 | G1 · host | A planted gait collapse at β* must force full resolution before β* | N31a-48; N31a-49; N31a-59 |
| KA-752 | Rate averaging: insulation life halves per 10 K; half the time at T_ref and half at T_ref + 20 K | Life is consumed at 2.5/τ_ref, not 2.0/τ_ref | Exact | G1 · host | Averaging the input (evaluating the law at the mean temperature) is caught | N31a-48; N31a-59 |
| KA-753 | Cell fade: 1% per √month calendar, 0.5% per 100 cycles; 9 months, 300 cycles | 3% + 1.5% = 4.5% | Exact | G1 · host | None stated in the plan. Registered here (DESIGN): calendar fade linear in time gives 9% + 1.5% | N31a-48; N31a-59 |
| KA-754 | Quasi-static superposition limit | With the first natural frequency at least five times the highest loaded frequency, the dynamic amplification 1/(1 − (f/f₁)²) is at most 25/24 = 1.042 | To the printed digits | G1 · host | None stated in the plan. Registered here (DESIGN): a ratio of 4.17 gives 1.061 and must fail the separation test | N31a-12; N31a-53 |

**Part 2. Numbered answers of the structure construction (C-35) that bind the same tests.**

| KA | The case | The truth | Carried by |
|---|---|---|---|
| KA-168 | Two-bar truss with a load at the apex | The volume at fixed stress is proportional to 1/(sin θ·cos θ), least at 45° (2.128, 2.000, 2.128 at 35°, 45°, 55°) | N31a-18 |
| KA-169 | Linear damage with Basquin exponent m = 3 | 10⁶ cycles at an amplitude of life 2 × 10⁶ give D = 0.5; a further 10³ cycles at five times the amplitude, of life 16,000, add 0.0625, so 0.1% of the cycles carry 11% of the damage | N31a-21; N31a-49 |
| KA-170 | Rainflow residue: the history 0, 10, 0 split into the windows [0, 10] and [10, 0] | One full cycle of range 10; a counter that drops residues at window ends counts none | N31a-21; N31a-04 (sibling probe) |
| KA-171 | Tolerance allocation: equal sensitivity, cost weights 1 and 8, statistical limit 0.1 mm | tᵢ ∝ (kᵢ/aᵢ²)^⅓ gives the ratio 1 : 2: 0.0447 mm and 0.0894 mm | N31a-32; N31a-56 |

**Part 3. Answers stated in the chapter's later subsections (tests drafted by unit U31b).**

| Where | The case | The truth |
|---|---|---|
| Packet review | The tube of KA-744 reproduced from each geometry export | 0.0950 kg within 10⁻⁶ of the model for each export |
| Packet review | A surface file written in millimetres and read as metres | 10⁹ times the volume; the check reports the ratio and names the file |
| Packet review | A packet that cites a verdict for the pre-snap design | Fails the verdict item of the mechanical pre-review |
| Referee re-extraction | A load 10% high | 1.1⁵ = 1.61 times the damage at m = 5; 1.1¹⁰ = 2.59 times at m = 10 |
| Referee re-extraction | Direction set of a two-port link | 2 × 18 = 36 windows of 1 s per link class; 108 to 180 referee-seconds per iteration for three to five classes |
| Member sizing in screening | Cantilevered tube 0.3 m, 600 N across its tip, S_y = 276 MPa, safety factor 2, mean radius 15 mm, 2,700 kg/m³ | Root moment 180 N·m; allowable 138 MPa; t ≥ 1.85 mm; mass 0.141 kg; inertia about the root 4.2 × 10⁻³ kg·m²; a design declaring 0.05 kg is under-built by a factor of 2.8 |

**Part 4. Additional derived values (DESIGN fixtures with hand derivations), registered so that each has one
home.**

| Id | The case | The truth | Carried by |
|---|---|---|---|
| F-01 | Free body: 2 kg link at rest under gravity; spun at 10 rad/s with its centre of mass 0.1 m from the axis | 19.6133 N; 20 N toward the axis | N31a-07 |
| F-02 | Centripetal field: ω = (1, 2, 3) rad/s at x = (0.1, 0, 0) m | (−1.3, 0.2, 0.3) m/s²; products (1, 4, 9, 2, 6, 3) | N31a-09 |
| F-03 | Load coordinates | 18 for a two-port link, 24 for three ports; 36 and 648 directions for the two direction sets at n = 18 | N31a-09; N31a-10 |
| F-04 | Traction on a circular face of radius 10 mm | 100 N: 0.3183 MPa; 1 N·m bending: 1.273 MPa peak; 1 N·m torsion: 0.6366 MPa peak | N31a-08 |
| F-05 | Offset payload: 15 kg robot, feet at (±0.20, ±0.15) m, 2 kg payload 0.10 m to the left | Left feet 44.95 N, right feet 38.41 N; ratio 1.170 | N31a-11 |
| F-06 | Mirror parity under P = diag(1, −1, 1) | Force (1, 2, 3) → (1, −2, 3); moment (1, 2, 3) → (−1, 2, −3) | N31a-11 |
| F-07 | Trot statics of a 15 kg quadruped | 73.55 N per supporting foot; 220.65 N at factor 3 | N31a-13 |
| F-08 | Combined loop gain | 0.59 + 0.15 = 0.74; 0.74⁴ = 0.30; amplification 3.8 against 2.4; S* = 2.35 kg for A = 0.5, λ = 0.15, M₀ = 10 | N31a-24; N31a-25 |
| F-09 | Ceiling | 0.27 × 0.30 = 0.081; 0.27 × 0.07 = 0.019 | N31a-02 |
| F-10 | Euler column: pinned tube, mean radius 10 mm, wall 1 mm, 0.5 m, 69 GPa | 8,558 N | N31a-20; BD-401 |
| F-11 | First bending mode of the KA-737 beam in aluminium (2,700 kg/m³), clamped–free | 204.2 Hz; 45.6 Hz with a 0.5 kg tip mass | N31a-20 |
| F-12 | Stress concentration at a small circular hole in a wide plate under tension | 3 | N31a-16 |
| F-13 | Two-bar slab fixture: L = 0.10 m, P = 1,000 N, V = 2.4 × 10⁻⁵ m³, E = 69 GPa | Compliance 0.02415 J; least volume 1.449 × 10⁻⁶ m³ at 138 MPa | N31a-18 |
| F-14 | Overhang of a sphere of radius 10 mm at a 45° limit | 2·π·R²·(1 − cos 45°) = 184.0 mm² | N31a-28; N31a-30 |
| F-15 | Spherical cavity of radius 5 mm | 5.236 × 10⁻⁷ m³ | N31a-27 |
| F-16 | Correlated stack: a = (1, 1), σ = (3, 4) µm | 6.083 µm at correlation 0.5; 5 and 7 µm at 0 and 1 | N31a-32 |
| F-17 | Quadratic quantity q = x²/L, L = 1 mm, margin 10 µm | Worst-case box 100 µm; 40 µm deviation at 200 µm | N31a-33 |
| F-18 | STEP instance bound | N = 3·T + V + 2; T ≤ 285,712 for a closed surface at 10⁶ instances | N31a-35 |
| F-19 | Two-finger bus | No schedule at a one-tick deadline; exactly two at two ticks | N31a-39 |
| F-20 | Q15 gain 3/10 | 9,830 counts; error 1.22 × 10⁻⁵ | N31a-46 |
| F-21 | Lifecycle step counts | 1.8 × 10⁹ steps for 1,000 h at 500 Hz; 2.25 × 10⁶ compressed; ratio 800; 3.6 × 10⁶ for the gate | N31a-47; N31a-50 |
| F-22 | Planted collapse: c₀ = 10⁻³, c₁ = 50, ε = 10⁻⁴, β* = 0.02 | t* = ln(1001)/0.005 = 1,381.8 s | N31a-49 |

**Registry mechanics.** Each entry: `id`, `case`, `truth` (value and formula), `tolerance`, `tier`,
`negative_control`, `derivation` (a file under `docs/derivations/foundry/`), `tests` (fully qualified test
names). The completeness check (`cargo test -p frob-conformance registry::foundry`) fails if an entry has no
derivation file, no test, or no negative control; if a listed test does not exist; or if a number in a test's
expected values differs from the registry's truth. Oracle values are entered by hand from the derivations and
are never produced by running the code under test.

## Acceptance criteria

- [ ] The registry file has 18 entries for KA-737 to KA-754, 4 for KA-168 to KA-171, 6 for Part 3 and 22 for Part 4, each with truth, tolerance, tier, control and derivation.
- [ ] Every entry of Parts 1, 2 and 4 names at least one existing test; Part 3 entries name the tests of unit U31b's beads once those exist and are otherwise marked `pending: U31b`.
- [ ] The four rows the plan leaves without a control (KA-747, KA-750, KA-753, KA-754) have a registered control, and the two rows marked "to set" (KA-739, KA-751) have a tolerance.
- [ ] Each derivation file exists and its final numbers equal the registry's.
- [ ] The completeness check fails on a planted entry with no control and on a planted test whose expected value differs from the registry.
- [ ] No registry value was generated by the code under test (a review item recorded in the bead's closing note, with the derivation files' authorship).

## Test plan

### Unit tests
`cargo test -p frob-conformance registry::foundry`:
- `foundry_registry_counts` (18, 4, 6, 22).
- `every_entry_has_derivation_test_and_control`.
- `tests_named_in_registry_exist`.
- `expected_values_match_registry` (parses the expected constants of the named tests).
- `planted_entry_without_control_fails`; `planted_value_mismatch_fails`.
- Arithmetic re-checks, in exact or extended precision, of every closed form in the tables (one test per row, for example `ka_737_values`, `ka_742_split_counts`, `ka_745_closed_form`, `f_18_bound`).

### End-to-end script and logging
None of its own. Each cluster script logs, in its `summary` line, the KA identifiers it asserted; the helper
`scripts/e2e/foundry_ka_coverage.sh` reads the logs of one full run and exits non-zero if any of KA-737 to
KA-754 was asserted by no script.

### Known answers and negative controls
This bead is the list. Its own negative controls are the two planted registry defects. Determinism: the
registry check is a host test, identical everywhere.

## Considerations

- Fixture dimensions and material constants are DESIGN choices; changing one changes the truth and needs a new
  derivation, not an edit of the number.
- Two tolerances are set here where the plan says "to set"; both reuse a gate's threshold, so no new number is
  introduced.
- The split-window counts of KA-742 (ranges 3, 4, 6, 7, 8) are derived here by applying the four-point rule to
  the two windows that share the fifth point; they reproduce the plan's 7.816 × 10⁻⁶ and are recorded so that
  the mutation test has a full expected table.
- Part 4 exists because the mechanism beads introduce fixtures the plan does not number; registering them
  keeps "every test has an oracle that was not regenerated" true for them as well.

## References

§31.18, Appendix M.30 (KA-737 to KA-754), §10 C-35 (KA-168 to KA-171), Appendix D, GB-37, GB-38, §38.6.
