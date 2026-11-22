---
key: "N33-CX"
title: "registry: counterexamples of Suite consumption (CX-352 to CX-357) and the late foundation-fact shortcuts"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "gauntlet", "frob-conformance", "registry", "solid", "test"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.11: the record format of the negative-control registry and the rule that every gate that passed has a registered negative control that it catches (GATE-GAUNT-01)", "§33.11: the admission probes of the planar drive (seed BD-033), the stride converter (seed BD-036) and the trainer's update (seed BD-034), which hold the fixtures of CX-352, CX-353 and CX-357"]
related: ["N33-REF", "BD-038", "BD-033", "BD-034", "BD-036", "BD-146", "N33c-03", "N33c-05", "N33c-25", "N33c-36", "N33c-37", "N33c-38", "N33c-41", "N33c-42", "N33c-43", "N33c-44", "N33c-45", "N33c-46"]
gates: []
plan_refs: ["Appendix N.30", "Appendix N.1", "Appendix N.35", "§33.11", "§33.12", "§33.14", "§33.15", "§33.17", "§33.18"]
---
## Background

The project keeps a catalogue of counterexamples: for each tempting shortcut, the smallest case that breaks it and
the rule that exists because of the case. The catalogue is not documentation. Each entry becomes a negative
control: a planted defect, input, design or world that a gate must catch. The rule of the verification program is
that a gate nothing can fail validates nothing, and that every gate that passed has a registered negative
control that it catches.

This bead is the registry of the counterexamples that belong to the suite-consumption chapter, the chapter that
fixes what FrankenRobotics takes from its sibling projects and under which boundary. Six entries are numbered in
the catalogue (CX-352 to CX-357). The chapter's late subsections state several more shortcut cases with their
numbers and without catalogue numbers; they are listed here too, so that none is lost, and are proposed for
numbering under the catalogue's append-only rule.

How an entry becomes a control (the catalogue's own table):

| Kind of shortcut | Kind of control |
|---|---|
| A wrong formula or implementation choice | `mutation`: a test build in which the shortcut replaces the rule |
| A wrong inference from a right computation | `planted-input`: a request that asks for the unsound conclusion and must be refused |
| A wrong screen or prune | `planted-design`: a design the shortcut would accept or discard wrongly |
| A wrong assumption about worlds | `planted-world`: a world or world pair that separates the shortcut from the rule |
| A wrong experimental procedure | `a-a` or a scripted adversary |

A case whose number is a FACT from a sibling's source is re-run in this repository before it is registered as a
control.

## Scope

In: the registry rows for the six numbered entries and for the unnumbered cases; for each, the kind of control,
the bead that holds its fixture, and the gate it is registered against; a registry test. Out: the fixtures
themselves (the beads named per row); the catalogue's numbering authority.

## Technical approach

**Row format (DESIGN).** One `[[control]]` table per entry in `registries/negative_controls.toml` (the file and
its schema are owned by the verification program): `id` (the CX number, or a working slug until numbered),
`shortcut`, `case` (with its numbers and units), `rule`, `kind`, `fixture` (a test path), `gates` (the gates
whose pass depends on catching it), `source_fact` (`sibling-source`, `arithmetic` or `design`), `rerun_here`
(the test that re-runs a sibling-sourced number).

**The six numbered entries.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Kind; fixture owner; gate |
|---|---|---|---|---|
| CX-352 | A branch on abs(ω) ≤ 10⁻¹² rad/s separates straight from curved motion | θ = 32 rad, wheel speeds 1 and 1 + 6·10⁻¹³ m/s, track 0.5 m, Δt = 1/400 s: ω = 1.2·10⁻¹² rad/s takes the curved branch, ω·Δt = 3·10⁻¹⁵ rad is below half the spacing of doubles at 32 (2⁻⁴⁸ ≈ 3.55·10⁻¹⁵), θ′ rounds to θ and both differences are exactly zero. Over 400 steps the true displacement is one metre, about (0.83422336, 0.55142668) m | The half-angle chord form Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)), with δ = ω·Δt, sinc(x) = sin(x)/x and sinc(0) = 1; INT-FSIM-05 | `mutation` (the thresholded branch in place of the chord form); the planar-drive admission bead; INT-FSIM-05 |
| CX-353 | A stride tuple means the same thing in every array library | A contiguous 2×2 single-precision view has strides (8, 4) in bytes and (2, 1) in elements; (8, 4) copied into an element-stride view over a 16-element buffer addresses element 12 at [1, 1], where element 3 belongs, in bounds. Byte origin 4 with strides (8, −4) addresses (1, 0, 3, 2), which unsigned strides cannot express | A typed converter at the export boundary; no bare stride tuple crosses it; INT-FNP-02 | `mutation` (a verbatim stride copy); the stride-converter admission bead; INT-FNP-02 |
| CX-354 | A dimension vector is all the typing a quantity needs | A torque and an energy have the same vector (kilogram 1, metre 2, second −2); dimension checking alone cannot separate them | Distinct aliases over one dimension; the compile-fail test adds a torque to an energy | `planted-input` (a source file that adds a torque to an energy and must fail to compile); N33c-03, and N33c-05 if `frob-units` exists; INT-TOOLCHAIN-01 (the quantity rule) |
| CX-355 | Import the mesh and trust it | A corridor whose far half was never observed appears as empty space in a naive import, and a traversal certificate over it would be false; with the coverage mask the same query returns unknown | Unscanned space is unknown, never free | `mutation` (an importer that ignores the coverage mask) and `planted-world` (the half-observed 10 m corridor, 160 of 320 voxels unknown at 0.5 m); N33c-25; INT-FDGR-01 |
| CX-356 | A fallback that exists on paper is a fallback | The worker-process fallback is exercised at M0 although inactive: a 2,000-step rollout through the worker must return bit-equal chunk digests, also after the worker is killed mid-job | INT-FSIM-18 | `a-a` (the same job in process and through the worker) with the `mutation` of a worker built at a different commit, which must be refused; BD-038; INT-FSIM-18 |
| CX-357 | A sibling's training code can serve as an oracle or a baseline | The suite's only PPO update reverses the clip branch for negative advantage (finite differences −0.25, −0.5, 0 where 0, −0.5, −1 are correct), forms the critic gradient from the stored value (0 where 2 is correct) and forwards two reset episodes as one sequence (ratio 1.582459 where 1 is correct) | Not linked, not an oracle, not a baseline; INT-FSIM-06; the same three probes test this project's PPO | `mutation` (each of the three errors planted in this project's own update); the trainer admission bead; INT-FSIM-06. Uses the same fixture as CX-252 of the learning chapter (≡ CX-252) |

**Shortcut cases stated with numbers in the late subsections and not numbered in the catalogue (proposed
entries; the working slug is the `id` until the catalogue assigns a number).**

| Working slug | The tempting shortcut | The smallest case that breaks it | The rule it protects | Kind; fixture owner; gate |
|---|---|---|---|---|
| `suite-wrench-copy` | A wrench's two vectors can be copied field by field into the sibling's wrench | Body frame rotated 90° about z (body x is world y): a body-frame force (1, 0, 0) N means a push along world y; copied unconverted into `force_world` it pushes along world x, with the same magnitude and energy, and nothing faults | `MixedWrench` with exactly two constructors; the mixed-wrench probe | `mutation` (the unconverted constructor); N33c-36; GATE-WS-06 |
| `suite-sampler-marginals` | Correct marginals establish a correct sampler | A Box–Muller cosine whose phase advances by the fixed fraction 0.167245 of a turn per draw: each channel is positive half the time, and adjacent channels agree in sign with probability 0.6655 (reported counts 66,552, 66,552, 66,551 of 100,000) | Rule P-12: the joint law of the sampled action equals the law the likelihood assumes | `mutation` (the planted linear-phase sampler); N33c-37; GATE-WS-06 (sampler row), GATE-CORTEX-18 |
| `suite-pins-compatible` | A passing build means the pins are compatible | franken_textiles pins `serde_json = "=1.0.151"` and FrankenSim's graph resolves 1.0.154; a build that never enables the cloth-oracle feature resolves, and the conflict appears only when the feature is first turned on | Resolution is checked with every optional feature group enabled; the oracle runs as a worker process in its own workspace | `planted-design` (a dependency on `ftx-drape` behind an off-by-default feature); N33c-38; GATE-WS-06 |
| `suite-reduced-catalog` | A catalog robot model is the robot | The G1 catalog builder keeps 30 links summing to 31.96314202 kg; the source assembly is reported at 33.34014202 kg over 38 links. At 9.81 m/s² a 320 N support passes on the reduction (313.5584 N) and fails on the assembly (327.0668 N) | A reduced model declares its omissions and is never the import-parity reference | `planted-design` (the 320 N support fixture); N33c-41; GATE-WS-09 |
| `suite-refusal-means-singular` | A sibling's "ill-conditioned" refusal means the pose is singular | A pure translation of 33 m: the sibling's series refuses because the infinity norm of ad exceeds 32, while the differential is exactly I ± ad/2 with determinant one (linear y component ∓16.5 m for a unit rotation rate about z) | A limit of an owner is reported as `unsupported`; `frob-lie` owns the closed form | `planted-input` (the 33 m twist); N33c-42; GATE-WS-09 |
| `suite-power-detects-sign` | A power balance detects a reversed transformer | Two unit stores, efforts (2, 3), coefficient 4: rates (12, −8) from one entry point and (−12, 8) from the other; both give 2·12 + 3·(−8) = 0 | The signed motion witness; direction is an explicit typed argument | `mutation` (an adapter that forwards a signed coefficient to the legacy entry point); N33c-43; GATE-WS-09 |
| `suite-chord-bound` | A swept-sphere box built from the end positions encloses the motion | A 1 m arm from −60° to +60°, tip radius 5 cm, obstacle at (1, 0) m: the chord lies on x = 0.5 m and its padded box reaches 0.55 m; the tip passes through the obstacle. The sagitta bound (0.5 m) reaches 1.05 m | Arc-aware swept bounds for rotating links | `mutation` (the chord-only bound); N33c-44; GATE-WS-09 (row 12), GATE-TRAC-12 |
| `suite-float-bound` | A value returned by a floating-point bound routine is a bound | 1 − 2⁻⁵⁰·x on abs(x) ≤ 2²⁸ is certified as 1 after its coefficient is trimmed (minimum 1 − 2⁻²²); 1 − x/2 on abs(x) ≤ 2⁻⁵⁵ returns 1 after a subtraction rounded to nearest (minimum 1 − 2⁻⁵⁶) | A bound is evaluated on the authored coefficients and its last operation is rounded outward | `planted-input` (the two polynomials offered as evidence with the returned value 1); N33c-45; GATE-WS-09 |
| `suite-local-ray-parameter` | A ray's hit parameter is a world distance | Two planes on one ray: the foreground at 2 m in an instance of scale 1/2 reports 4; the background at 3 m in an instance of scale 3 reports 1; sorting local parameters returns the occluded plane, and a 2.5 m range gate keeps the background | s_W = s_L/‖A⁻¹·d‖ before sorting, gating or use as optical depth | `mutation` (sorting local parameters); N33c-46; GATE-WS-09 (row 14), GATE-HAB-16 |

**Registry test.** `frob-conformance` loads the registry and asserts, for every row of this bead: the fixture
path exists; the fixture, when run, catches the control (the mutation build fails its test; the planted input is
refused; the planted design is rejected); the gates listed exist in `registries/gates.toml`; a sibling-sourced
number has a `rerun_here` test. A row whose rule has no gate is reported as a finding at the next milestone
exit.

## Acceptance criteria

- [ ] `registries/negative_controls.toml` contains the six rows CX-352 to CX-357 with the cases and numbers above, each with a kind, a fixture path and at least one gate.
- [ ] The nine unnumbered cases are present under their working slugs with the same fields, and a request for catalogue numbers is filed with the catalogue's owner.
- [ ] Every row's fixture exists and catches its control when run (15 of 15), or the row is marked `due <milestone>` with the bead that will supply the fixture.
- [ ] CX-357 is marked as sharing its fixture with CX-252, and both rules (the baseline ban and the sequence rule) have a gate that catches it.
- [ ] Each sibling-sourced number (the planar-drive arithmetic, the stride case, the trainer's three numbers, 0.6655, 31.96314202, the transformer rates, the two `fs-sos` values) has a `rerun_here` test in this repository.
- [ ] The registry test passes and prints the count of controls by kind and by milestone.

## Test plan

### Unit tests

`cargo test -p frob-conformance registry_cx_suite`:
- `rows_present_and_well_formed`: 15 rows; no empty field.
- `fixtures_catch_controls`: runs each fixture's "mutant must fail" test (or checks its `due` mark).
- `gates_exist`.
- `cx_357_shares_fixture_with_cx_252`.
- `sibling_numbers_have_rerun`.
- Hand-check tests that carry the arithmetic of two entries so the registry cannot drift from its numbers: `cx_352_arithmetic` (ω = (v_r − v_l)/track = 6·10⁻¹³/0.5 = 1.2·10⁻¹² rad/s; ω·Δt = 3·10⁻¹⁵ rad; 2⁻⁴⁸ = 3.5527·10⁻¹⁵; `32.0 + 3e-15 == 32.0` in `f64`; (cos 32, sin 32) = (0.83422336, 0.55142668)); `cx_353_indexing` (strides (8, 4) read as elements address 8 + 4 = 12; the correct element strides (2, 1) address 3).

### End-to-end script and logging

`scripts/e2e/suite_controls.sh`: runs the registry test, then for each row whose fixture is an end-to-end plant
(the different-commit worker; the `ftx-drape` dependency; the half-observed corridor) invokes the owning script's
plant step and asserts it was caught. It logs one `assert` event per control (`id`, `kind`, `gate`, `caught:
true|false`, the fixture path) and a `summary` with controls caught of controls registered.

### Known answers and negative controls

This bead is the registry of negative controls; its own control is a registry row pointing at a fixture that
does not fail under its mutation (a `synthetic` row with a mutation that changes nothing), which
`fixtures_catch_controls` must report. Known answers: the arithmetic of CX-352 and CX-353 above. Lanes: the
registry test on both machines; fixtures follow the lanes of their owning beads.

## Considerations

- Catalogue numbers are append-only; this bead does not assign numbers to the nine unnumbered cases. Until they
  are numbered they are still registered controls, because the registry holds one control per protected rule.
- Rows 1 to 4 of the foundation-defect table belong to another drafting unit's subsection; their fixtures are in
  the seed beads for the planar drive, the trainer's update, the stride converter and the contact step, and
  three of them are the numbered entries above.
- A counterexample found in use (a false certificate, a false promotion, a withdrawn result) is appended to the
  catalogue and becomes a permanent control; for this chapter the likely source is a pin move that changes a
  sibling's behaviour under a recorded probe.

## References

Plan Appendix N.30 (CX-352 to CX-357), N.1 and N.35 (rules and kinds of control); §33.11, §33.12, §33.14, §33.15,
§33.17, §33.18; GATE-GAUNT-01; PO-GAUNT-03.
