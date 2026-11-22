---
key: "N22-CX"
title: "registry: the counterexamples of Reflex (CX-234 to CX-245, and seven from later subsections) as negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "reflex", "frob-conformance", "registry", "test", "solid"]
parent: "WP-14"
deps: ["BD-025", "STD-TEST"]
needs: []
related: ["N22-REF", "N22c-04", "BD-146"]
gates: []
plan_refs: ["Appendix N (N.19)", "§22.16", "§22.20", "§22.21", "§22.22", "§22.23", "§22.24", "§22.25", "§0.7"]
---
## Background

A rule of this project: where a plausible shortcut is wrong, the plan gives the smallest hand-checkable case that
breaks it, and that case becomes a **negative control**: a planted defect (a "mutant" implementing the shortcut)
that a gate's test must catch. A gate that nothing can fail validates nothing, so a convergence gate has not passed
until its registered control has been shown to fail. The controls live in the negative-control registry of
`frob-conformance` (the Gauntlet's crate), keyed by counterexample number.

Reflex (estimation, baseline control, sampling control, gait orbits, skills, energy budgets, runtime shields) has
twelve numbered counterexamples. The later subsections of the chapter (a lifted planning model, the pre-slip
aliasing fixture, the delivered-controller lane, autonomous orbits, stride sequences, sampled damping) state seven
more in their text that the catalogue has not numbered yet. This bead registers all nineteen so that every
mechanism bead can cite a control by key and so that none is lost.

## Scope

In: one registry row per counterexample with the shortcut, the smallest case and its numbers, the correct answer,
the rule it protects, the mutant to plant and the gate that must catch it; a test that every row has a live mutant
and a gate. Out: the mechanisms themselves; the known-answer registry of the chapter (key N22-KA, written by the
unit that owns §22.1 to §22.9).

## Technical approach

**Row format (DESIGN).** `frob-conformance/registry/counterexamples.toml`, one `[[cx]]` per row: `id`, `shortcut`,
`case` (inputs with units), `wrong_answer` (what the shortcut returns), `right_answer`, `rule`, `mutant` (the
name of the planted implementation, compiled only under the test configuration), `caught_by` (gate and test
name), `defined_in`. A mutant is real code run through the real test; no mock stands in for it.

**The twelve numbered rows.**

| CX | The tempting shortcut | The smallest case that breaks it (numbers) | Rule it protects | Mutant; caught by |
|---|---|---|---|---|
| CX-234 | Score an estimator after fitting a scale | An estimate of 2 m for a true motion of 1 m: a fitted factor of ½ gives zero residual; aligned by one translation and one yaw only, the score is 1 m | The scoring rule: one translation and one yaw alignment per trajectory, never a scale; FROB-REF-464 | `score_with_similarity_fit`; GATE-REFLEX-03 |
| CX-235 | Report "no solution" when the cone solver fails to converge | A 50 N body pushed with 30 N through one contact with μ = 0.5 is infeasible, and the proof is the dual vector y = (0.5, −1): it lies in the dual cone {(a, c): a ≥ μ·abs(c)} and yᵀb = 25 − 30 = −5 N < 0. An unconverged run has no such vector and proves nothing | The three end states (`optimal`, `infeasible`, `unconverged`); the exact rational re-check of every certificate (PO-REFLEX-03) | `unconverged_reported_infeasible`; GATE-REFLEX-05 |
| CX-236 | Take the lower rows of a space-frame screw Jacobian as the tool-point Jacobian | Two-link arm, links 0.4 m and 0.3 m at angles 0 and π/2, limit 2.5 N·m: the point Jacobian [[−0.3, −0.3], [0.4, 0.0]] m/rad gives abs(F_x) ≤ 2.5/0.3 = 25/3 N from joint 2; the space-frame rows give a different bound | Typed Jacobians (frame and point are part of the type); GATE-REFLEX-08 | `space_frame_rows_as_point_jacobian`; GATE-REFLEX-08 |
| CX-237 | Compute gait stability on the symmetric subspace only | ẋ = −x, ẏ = +y with symmetry y ↦ −y over a period T: on Fix(K) = {y = 0} the multiplier is e^(−T); the full-space multiplier in the antisymmetric block is e^(+T) > 1. The restricted answer says stable for a robot that falls sideways | PO-GAIT-01: the monodromy is assembled over the full reduced state | `monodromy_on_fixed_subspace`; GATE-GAIT-01 (the restricted computation must be marked incomplete) |
| CX-238 | Resolve simultaneous impacts in a fixed order | A uniform bar of half-length l on supports at ±l/3 (β = a²/k² = 1/3) arriving flat at speed v: the coupled solve gives rest; one support first leaves the first end at v/4 with 1/16 of the kinetic energy. The sign of J₁·M⁻¹·J₂ᵀ = (1 − β)/m decides whether order matters | One coupled solve for simultaneous events; no refusal for simultaneity alone | `forced_impact_order`; GATE-GAIT-02 |
| CX-239 | Budget the payload's kinetic energy alone | A rotor of inertia 5 × 10⁻⁵ kg·m² behind a 50:1 reduction on a wheel of radius 0.05 m has effective mass 50²·(5 × 10⁻⁵)/0.05² = 50 kg, against an 8 kg cart | T_ub comes from the full mass matrix, in which reflected inertia already appears | `payload_only_kinetic_term`; GATE-SHIELD-01 (the mutant must show an exceedance) |
| CX-240 | Project onto the constraint row, then clip to the actuator box | R = I, u_π = (1, 0), box [0, 1]², row u₁ + u₂ ≥ 1.5: the unboxed projection is (1.25, 0.25), outside the box; clipped it is (1, 0.25), which violates the row; the breakpoint solution is λ = 0.5, u = (1, 0.5) | PO-SHIELD-02: the single-row projection is the exact minimizer inside the box | `project_then_clip`; GATE-SHIELD-02 |
| CX-241 | Estimate a monitor's miss rate from the monitor's own log | A monitor that is silent on every breach it cannot see reports δ₁ = 0 from its own output | δ₁, δ₂, δ₃ are estimated on independent hidden-state labels from the simulator | `self_labelled_miss_rate`; GATE-SHIELD-04 |
| CX-242 | A transported skill refines its specification if it can select the good branch | A target whose successors are {stable, fall} cannot refine an abstract transition whose only successor is stable | Refinement quantifies over every target successor; FROB-REF-444 | `refine_by_selecting_branch`; GATE-REFLEX-08 |
| CX-243 | Use the fixed-base pullback τ = JᵀF for a floating-base robot | The map must come from the contact-consistent constrained solve. The chapter gives no numeric case; the fixture bead N22c-04 supplies one (DESIGN, `synthetic`): a 10 kg base on a level floor with μ_min = 0.4 whose arm can produce 60 N at the tool is asked for a 50 N horizontal push. The fixed-base map admits it (50 ≤ 60); the contact-consistent allocation refuses, because the floor can react at most 0.4·10·9.81 = 39.24 N, with the base's floor contacts binding | The limits written into every `.fskill` artifact | `fixed_base_pullback_on_floating_base`; GATE-REFLEX-08 |
| CX-244 | Tie a controller's weights under the mechanical symmetry group | Two identical latches, exactly one must close, observation fixed by the mirror: the in-law controller succeeds and the tied deterministic one fails; on a one-sided slippery patch, tying under Γ_mech loses return | The stabilizer intersection Γ_use; FROB-REF-461 | `tie_under_gamma_mech`; GATE-REFLEX-07 |
| CX-245 | Use the sampling controller as an acceptance evaluator because one episode is cheap | Bounding a 1% failure rate at 95% takes ⌈ln 0.05 / ln 0.99⌉ = 299 episodes: about 99 and 299 GPU-minutes under sampling control (quadruped, humanoid) against about 10 and 60 to train once | Screen, not evaluator; FROB-REF-817 | a planted acceptance request; GATE-REFLEX-06 |

**Seven further rows from the later subsections.** The catalogue has no number for these; they are registered under
provisional identifiers `CX-R22-a` to `CX-R22-g` and renumbered when the catalogue is regenerated.

| Provisional id | The tempting shortcut | The smallest case that breaks it (numbers) | Rule it protects | Mutant; caught by |
|---|---|---|---|---|
| CX-R22-a | A lifted linear model with zero training residual is a good planning model | Reset map x⁺ = abs(x), dictionary {1, x, x²}, samples x ∈ {−1, 0, 1} with targets {1, 0, 1}: the fit is x⁺ = x² with zero residual; at x = 0.5 it predicts 0.25 where the truth is 0.5 | Admission by a held-out band inside one contact mode, never by the fitting residual | `admit_on_training_residual`; GATE-REFLEX-10 |
| CX-R22-b | A slip monitor with full braking as backup makes an optimistic friction assumption safe | Cart at 2.0 m/s, commanded deceleration 5.0 m/s², worlds μ = 0.8 and μ = 0.3, controller brakes at s ≤ 0.45 m: the transcripts are identical before braking; in the μ = 0.3 world the alarm is raised at the first braking tick and the cart still crosses at 1.162 m/s and stops 0.2296 m beyond the line | The timeliness rule: an assumption whose monitor is not timely goes inside the quantifier | `list_untimely_assumption_as_monitored`; GATE-SHIELD-06 |
| CX-R22-c | Test the continuous-time barrier at each tick of a sampled loop | Same cart, h = s − v²/(2·μ_min·g) with μ_min·g = 2.943 m/s² and Δt = 0.02 s: at s = 0.70 m, h = 0.0204 and coasting is allowed; at the next tick s = 0.66 m; braking stops 0.0196 m beyond the line in the μ = 0.3 world. The overshoot is at most v·Δt = 0.04 m | The inter-sample term L·Δt of the shield margin | `drop_intersample_term`; GATE-SHIELD-06 |
| CX-R22-d | Episodes of a planner that reads the plant's parameters are evidence about the delivered controller | Cart at 2 m/s, friction drawn from {0.2, 0.8}: the privileged planner brakes at 1.019 m and 0.255 m and never fails; the same planner with nominal μ = 0.5 brakes at 0.408 m and overruns by 0.612 m on every slippery world: zero failures in 299 privileged episodes for a controller that fails every second episode | Lane condition 1 (not model-privileged); FROB-REF-817 | a privileged run offered as acceptance evidence; GATE-REFLEX-11 |
| CX-R22-e | Apply the fold test and the gradient solve of clock-forced gaits to any gait | Planar loop ṙ = r·(μ − r), φ̇ = 1: the time-2π monodromy has multipliers 1 and e^(−2πμ), so det(I − M) = 0 on every orbit and every design. The section map has the single multiplier e^(−2πμ) | Phase-source classification; section or bordered form for autonomous loops | `stroboscopic_on_autonomous_loop` (must be refused, not reported as a fold); GATE-GAIT-04 |
| CX-R22-f | A gait that alternates stride modes is stable if each mode's multipliers are inside the unit circle | A = [[1/2, 1], [0, 1/2]], B = Aᵀ: each alone has both multipliers 1/2; B·A = [[1/4, 1/2], [1/2, 5/4]] has eigenvalue 3/4 + 1/√2 ≈ 1.457, a growth of (1 + √2)/2 ≈ 1.2071 per stride | Stability over admitted stride sequences (lower bound by cycle products, upper bound by a common quadratic norm) | `per_mode_multiplier_test`; GATE-GAIT-05 |
| CX-R22-g | A sampled damper is passive because the continuous one is; and the work of a held command is power at the tick times the period | A free 1 kg slider at +1 m/s, gain 300 N·s/m held 10 ms: the speed ends at −2 m/s and the kinetic energy rises from 0.5 J to 2 J (+1.5 J); power at the start times duration predicts −3 J. With one tick of latency, κ = 3/2 diverges with spectral radius √(3/2) although κ < 2 | The gain bound 2·M̄ − h·K_d ⪰ 0; the held-work bound F·v·h + F²·h²/(2m); κ < 1 under one tick of latency | `damping_gain_above_bound`, `work_as_initial_power`, `undelayed_bound_on_delayed_loop`; GATE-SHIELD-07 |

**Registry check.** A build-time test walks the registry and fails if a row has no mutant, if a mutant is not
referenced by a test, or if a named gate does not list the row among its negative controls.

## Acceptance criteria

- [ ] All nineteen rows are in `frob-conformance/registry/counterexamples.toml` with every field filled.
- [ ] Each row's arithmetic is reproduced by a host test in exact rationals or to the printed digits (values in the tables above).
- [ ] Each mutant exists as code under the test configuration and is shown to fail its gate's test (a recorded failing run per mutant).
- [ ] The registry check passes; removing any mutant makes it fail.
- [ ] The seven provisional rows carry catalogue numbers once the catalogue is regenerated, and the provisional identifiers remain as aliases.

## Test plan

### Unit tests
`crates/frob-conformance/tests/cx_reflex.rs`:
- `cx234_scale_fit_scores_zero`: similarity alignment of (2 m vs 1 m) returns 0; rigid alignment returns 1 m.
- `cx235_certificate_exact`: y = (1/2, −1), A = I, b = (50, 30), μ = 1/2: y ∈ K*, yᵀb = −5 exactly; nudging y to (0.49, −1) fails membership.
- `cx236_point_vs_space_rows`: the point Jacobian gives 25/3 N; the mutant's bound differs.
- `cx237_restricted_vs_full`: multipliers e^(−T) and e^(+T) for T = 1 to 10⁻¹².
- `cx238_bar`: β = 1/3 gives first-end speed v/4 and energy ratio 1/16; β = 1 and β = 2 give rest for every order.
- `cx239_reflected_mass`: 2500·5e-5/0.0025 = 50 kg exactly.
- `cx240_projection`: breakpoint λ = 1/2, u = (1, 1/2); mutant output (1, 1/4) violates the row.
- `cx245_episode_count`: ⌈ln 0.05 / ln 0.99⌉ = 299.
- `cxr22a_abs_lift`: the fitted row is (0, 0, 1); prediction 0.25 at 0.5.
- `cxr22b_c_cart`: 1.162 m/s, 0.2296 m, 0.0204, 0.0196 m to four digits.
- `cxr22d_privileged`: 1.019 m, 0.255 m, 0.408 m, 0.612 m.
- `cxr22e_trivial_multiplier`: det(I − M) = 0 to 10⁻¹² for μ ∈ {0.1, 1, 3}.
- `cxr22f_product`: eigenvalue 3/4 + 1/√2 to 10⁻¹².
- `cxr22g_slider`: −2 m/s, +1.5 J, −3 J; delayed map radius √(3/2).

### End-to-end script and logging
`scripts/e2e/reflex_negative_controls.sh` runs, for each mutant whose mechanism exists at the current milestone,
the gate's own test twice: with the real implementation (must pass) and with the mutant enabled through the test
configuration (must fail). Per STD-E2E each step logs the command, exit code, the gate, the row id, the measured
value against the right and wrong answers, and the first differing value on a mutant failure; the summary lists
rows `caught`, `not caught` (a failure of this script) and `not run` (mechanism not yet built).

### Known answers and negative controls
This bead is itself the negative-control registry of the chapter. Its own control: a mutant deliberately wired to
return the right answer ("a control that cannot fail") must be reported by the registry check as ineffective.
Determinism: every row is host arithmetic in exact rationals or `f64` with stated digits; results are identical
run to run and across platforms.

## Considerations

- CX-243 has no numbers in the source; the fixture bead supplies the 39.24 N case above. If the skill layer's owner
  prefers another case, the row keeps its number and the case is replaced, with the mutant unchanged.
- The seven provisional rows must not be dropped when the catalogue is regenerated; aliases are kept so that
  recorded gate rows remain resolvable.
- Rows are seeds: mechanism beads add implementation-level mutants of their own and register them here.

## References

Appendix N (N.19), §22.16; §22.20 to §22.25 for the provisional rows; §0.7 (counterexamples are first-class); §38
(negative-control registry). Gates: GATE-REFLEX-03, -05, -06, -07, -08, -10, -11; GATE-GAIT-01, -02, -04, -05;
GATE-SHIELD-01, -02, -04, -06, -07.
