---
key: "N22-CX"
title: "registry: Reflex counterexamples CX-234 to CX-245 and nine appended entries, as catalogue entries for controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "reflex", "frob-conformance", "registry", "test", "solid", "review-notes-open"]
parent: "WP-14"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N22-KA", "N22-REF", "N22a-26", "N22a-31", "N22a-32", "N22a-33", "N22a-34", "N22a-38", "N22a-39", "N22c-02", "N22c-03", "N22c-04", "N22c-27", "N22c-49", "N00-CX", "N38b-04"]
gates: []
plan_refs: ["Appendix N (N.19)", "Appendix N (CX-012)", "§22.16", "§22.20", "§22.21", "§22.22", "§22.23", "§22.24", "§22.25", "§0.7"]
---
## Background

A rule of this project: where a plausible shortcut is wrong, the plan gives the smallest hand-checkable case that
breaks it, and that case becomes a **negative control**: a planted defect (a "mutant" implementing the shortcut)
that a gate's test must catch. A gate that nothing can fail validates nothing, so a convergence gate has not passed
until its registered control has been shown to fail. The cases live in the counterexample catalogue of
`frob-conformance` (the Gauntlet's crate), keyed by counterexample number; the controls derived from them live in
the negative-control registry beside it.

Reflex (estimation, baseline control, sampling control, gait orbits, skills, energy budgets, runtime shields) has
twelve numbered counterexamples. The later subsections of the chapter (a lifted planning model, the pre-slip
aliasing fixture, the delivered-controller lane, autonomous orbits, stride sequences, sampled damping) state nine
more shortcuts in their text that the catalogue has not numbered yet. This bead enters all twenty-one as catalogue
entries so that every mechanism bead can cite a control by key and so that none is lost.

## Scope

In: one catalogue entry per counterexample with the shortcut, the smallest case and its numbers, the correct
answer, the rule it protects, the planted defect and the gates that must catch it; the block request for the
nine late entries; a host test that recomputes every number; the chapter's script that shows each planted defect
fails its gate.

Out, with the bead that has it: the catalogue's record schema and loader, and the seeding tool that turns entries
into rows of `frob-conformance/registry/controls.toml`, allocates their `NC-` identifiers, keeps the seed map,
resolves a rule that names no gate and proves completeness by the join (N38b-05). This bead writes no row of
`controls.toml` by hand: its controls appear there when the seeding is re-run over its entries, which is
append-only and idempotent. The allocation ledger that issues numbers above CX-384 (N38a-40). The control runner
and the registry's rules, including that a gate with no control cannot pass (N38b-04). The mechanisms themselves.
The known-answer registry of the chapter (key N22-KA, written by the unit that owns §22.1 to §22.9) and the late
hand cases (N22c-04).

## Technical approach

**Data (DESIGN).** Each row is an entry of `frob-conformance/registry/counterexamples.toml` in the catalogue's
record schema (N38b-05): `id` (the CX number), `shortcut`, `case` (inputs with units, the shortcut's wrong answer
and the right answer), `numeric`, `plan_case` (`"words"` or `"numbers"`, the form of the case in the plan's text,
immutable), `case_supplied_by`, `rule`, `rule_ids` (the gates, invariants, obligations, decisions and refusal
codes of the rule column), `defined_in`, `same_as`, `label` (DESIGN or FACT with source), `control_kind`
(`mutation`, `planted-input`, `planted-design`, `planted-world`, `a-a`) and `plant`, plus `slug` for the late
entries. `plant` names the planted defect: for a `mutation`, a flag compiled only under the test configuration that
puts the shortcut into the real component; for the other kinds, the planted request, design or world. A plant is
real code run through the real test; no mock stands in for it. Every gate among an entry's `rule_ids` must fail
with the plant enabled. DESIGN, correcting an earlier draft of this bead that wrote `[[cx]]` rows with the fields
`wrong_answer`, `right_answer`, `mutant` and `caught_by`: the wrong and right answers are stated in `case` and
recomputed by the host test below, `mutant` is the catalogue's `plant`, and the gates of `caught_by` are the gate
subset of `rule_ids`; the test that catches each plant is named in this bead's script log.

**The twelve numbered entries.** In the last column the name before the semicolon is the entry's `plant`, and the
gate after it is in `rule_ids` together with the identifiers of the rule column.

| CX | The tempting shortcut | The smallest case that breaks it (numbers) | Rule it protects | Plant; gate |
|---|---|---|---|---|
| CX-234 | Score an estimator after fitting a scale | An estimate of 2 m for a true motion of 1 m: a fitted factor of ½ gives zero residual; aligned by one translation and one yaw only, the score is 1 m | The scoring rule: one translation and one yaw alignment per trajectory, never a scale; FROB-REF-464 | `score_with_similarity_fit`; GATE-REFLEX-03 |
| CX-235 | Report "no solution" when the cone solver fails to converge | A 50 N body pushed with 30 N through one contact with μ = 0.5 is infeasible, and the proof is the dual vector y = (0.5, −1): it lies in the dual cone {(a, c): a ≥ μ·abs(c)} and yᵀb = 25 − 30 = −5 N < 0. An unconverged run has no such vector and proves nothing | The three end states (`optimal`, `infeasible`, `unconverged`); the exact rational re-check of every certificate (PO-REFLEX-03) | `unconverged_reported_infeasible`; GATE-REFLEX-05 |
| CX-236 | Take the lower rows of a space-frame screw Jacobian as the tool-point Jacobian | Two-link arm, links 0.4 m and 0.3 m at angles 0 and π/2, limit 2.5 N·m: the point Jacobian [[−0.3, −0.3], [0.4, 0.0]] m/rad gives abs(F_x) ≤ 2.5/0.3 = 25/3 N from joint 2; the space-frame rows give a different bound | Typed Jacobians (frame and point are part of the type); GATE-REFLEX-08 | `space_frame_rows_as_point_jacobian`; GATE-REFLEX-08 |
| CX-237 | Compute gait stability on the symmetric subspace only | ẋ = −x, ẏ = +y with symmetry y ↦ −y over a period T: on Fix(K) = {y = 0} the multiplier is e^(−T); the full-space multiplier in the antisymmetric block is e^(+T) > 1. The restricted answer says stable for a robot that falls sideways | PO-GAIT-01: the monodromy is assembled over the full reduced state | `monodromy_on_fixed_subspace`; GATE-GAIT-01 (the restricted computation must be marked incomplete) |
| CX-238 | Resolve simultaneous impacts in a fixed order | A uniform bar of half-length l on supports at ±l/3 (β = a²/k² = 1/3) arriving flat at speed v: the coupled solve gives rest; one support first leaves the first end at v/4 with 1/16 of the kinetic energy. The sign of J₁·M⁻¹·J₂ᵀ = (1 − β)/m decides whether order matters | One coupled solve for simultaneous events; no refusal for simultaneity alone | `forced_impact_order`; GATE-GAIT-02 |
| CX-239 | Budget the payload's kinetic energy alone | A rotor of inertia 5 × 10⁻⁵ kg·m² behind a 50:1 reduction on a wheel of radius 0.05 m has effective mass 50²·(5 × 10⁻⁵)/0.05² = 50 kg, against an 8 kg cart | `shield.budget`: T_ub comes from the full mass matrix, in which reflected inertia already appears | `payload_only_kinetic_term`; GATE-SHIELD-01 (the plant must show an exceedance) |
| CX-240 | Project onto the constraint row, then clip to the actuator box | R = I, u_π = (1, 0), box [0, 1]², row u₁ + u₂ ≥ 1.5: the unboxed projection is (1.25, 0.25), outside the box; clipped it is (1, 0.25), which violates the row; the breakpoint solution is λ = 0.5, u = (1, 0.5) | PO-SHIELD-02: the single-row projection is the exact minimizer inside the box | `project_then_clip`; GATE-SHIELD-02 |
| CX-241 | Estimate a monitor's miss rate from the monitor's own log | A monitor that is silent on every breach it cannot see reports δ₁ = 0 from its own output | δ₁, δ₂, δ₃ are estimated on independent hidden-state labels from the simulator | `self_labelled_miss_rate`; GATE-SHIELD-04 |
| CX-242 | A transported skill refines its specification if it can select the good branch | A target whose successors are {stable, fall} cannot refine an abstract transition whose only successor is stable | Refinement quantifies over every target successor; FROB-REF-444 | `refine_by_selecting_branch`; GATE-REFLEX-08 |
| CX-243 | Use the fixed-base pullback τ = JᵀF for a floating-base robot | The map must come from the contact-consistent constrained solve. The chapter gives no numeric case; the fixture bead N22c-04 supplies one (DESIGN, `synthetic`): a 10 kg base on a level floor with μ_min = 0.4 whose arm can produce 60 N at the tool is asked for a 50 N horizontal push. The fixed-base map admits it (50 ≤ 60); the contact-consistent allocation refuses, because the floor can react at most 0.4·10·9.81 = 39.24 N, with the base's floor contacts binding | The limits written into every `.fskill` artifact | `fixed_base_pullback_on_floating_base`; GATE-REFLEX-08 |
| CX-244 | Tie a controller's weights under the mechanical symmetry group | Two identical latches, exactly one must close, observation fixed by the mirror: the in-law controller succeeds and the tied deterministic one fails; on a one-sided slippery patch, tying under Γ_mech loses return | The stabilizer intersection Γ_use; FROB-REF-461 | `tie_under_gamma_mech`; GATE-REFLEX-07 |
| CX-245 | Use the sampling controller as an acceptance evaluator because one episode is cheap | Bounding a 1% failure rate at 95% takes ⌈ln 0.05 / ln 0.99⌉ = 299 episodes: about 99 and 299 GPU-minutes under sampling control (quadruped, humanoid) against about 10 and 60 to train once | Screen, not evaluator; FROB-REF-817 | a planted acceptance request (`planted-input`); GATE-REFLEX-06 |

**CX-243's case.** The catalogue states CX-243 in words only, and N38b-05 lists it among the seventeen word-only
obligations, each needing a concrete planted defect from its owning chapter. This bead discharges that obligation:
the entry carries the DESIGN case above (`label = "DESIGN (N22c-04 fixture 16)"`), `numeric = true`,
`case_supplied_by = "N22c-04 fixture 16"` and the plant named, so the seeding tool writes a runnable control
instead of an obligation. It keeps `plan_case = "words"`: the loader counts the seventeen word-only entries by
`plan_case`, never by `numeric = false`, so the count stays seventeen after a chapter supplies a case (N38b-05).
Every other entry of this bead carries `plan_case = "numbers"`.

**Nine entries appended from the later subsections.** The catalogue has no number for these. Their numbers come
from the allocation ledger `crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`,
bead N38a-40), never from "the next free number" at commit time: this bead files one `[[block]]` request with
`registry = "cx"`, `count = 9`, `owner = "N22-CX"`, `section = "§22.20 to §22.25"`; the allocator fills `first`
and `last` (the plan-time catalogue ends at CX-384, and CX-385 to CX-400 are the verification chapter's), and the
nine tags below are mapped to the block's numbers in table order in one commit. Until then the tag of the first
column is the entry's name; after mapping it stays in the catalogue's `slug` field. A tag still used in a
registry, test name or derivation file at this bead's milestone (M1) fails the ledger check
`provisional-tag-past-due`. Mechanism beads of later milestones cite these entries by slug and use the issued
number in their files. An earlier draft of this bead named them `CX-R22-a` to `CX-R22-g`; those names map to the
slugs as listed in the last column, and the former row g, which bundled three shortcuts with three plants, is
three entries, because the seeding tool derives one control per entry.

| Slug | The tempting shortcut | The smallest case that breaks it (numbers) | Rule it protects | Plant; gate; `same_as`; earlier name |
|---|---|---|---|---|
| `reflex-lift-resid` | A lifted linear model with zero training residual is a good planning model | Reset map x⁺ = abs(x), dictionary {1, x, x²}, samples x ∈ {−1, 0, 1} with targets {1, 0, 1}: the fit is x⁺ = x² with zero residual; at x = 0.5 it predicts 0.25 where the truth is 0.5 | Admission by a held-out band inside one contact mode, never by the fitting residual | `admit_on_training_residual`; GATE-REFLEX-10; none; CX-R22-a |
| `reflex-untimely` | A slip monitor with full braking as backup makes an optimistic friction assumption safe | Cart at 2.0 m/s, commanded deceleration 5.0 m/s², worlds μ = 0.8 and μ = 0.3, controller brakes at s ≤ 0.45 m: the transcripts are identical before braking; in the μ = 0.3 world the alarm is raised at the first braking tick and the cart still crosses at 1.162 m/s and stops 0.2296 m beyond the line | The timeliness rule: an assumption whose monitor is not timely goes inside the quantifier; BET-25 (the budget is enforced by a shield, not only observed by a monitor) | `list_untimely_assumption_as_monitored`; GATE-SHIELD-06; CX-012; CX-R22-b |
| `reflex-intersample` | Test the continuous-time barrier at each tick of a sampled loop | Same cart, h = s − v²/(2·μ_min·g) with μ_min·g = 2.943 m/s² and Δt = 0.02 s: at s = 0.70 m, h = 0.0204 and coasting is allowed; at the next tick s = 0.66 m; braking stops 0.0196 m beyond the line in the μ = 0.3 world. The overshoot is at most v·Δt = 0.04 m | The inter-sample term L·Δt of the shield margin | `drop_intersample_term`; GATE-SHIELD-06; CX-012; CX-R22-c |
| `reflex-privileged` | Episodes of a planner that reads the plant's parameters are evidence about the delivered controller | Cart at 2 m/s, friction drawn from {0.2, 0.8}: the privileged planner brakes at 1.019 m and 0.255 m and never fails; the same planner with nominal μ = 0.5 brakes at 0.408 m and overruns by 0.612 m on every slippery world: zero failures in 299 privileged episodes for a controller that fails every second episode | Lane condition 1 (not model-privileged); FROB-REF-817 | a privileged run offered as acceptance evidence (`planted-input`); GATE-REFLEX-11; none; CX-R22-d |
| `reflex-free-period` | Apply the fold test and the gradient solve of clock-forced gaits to any gait | Planar loop ṙ = r·(μ − r), φ̇ = 1: the time-2π monodromy has multipliers 1 and e^(−2πμ), so det(I − M) = 0 on every orbit and every design. The section map has the single multiplier e^(−2πμ) | Phase-source classification; section or bordered form for autonomous loops | `stroboscopic_on_autonomous_loop` (must be refused, not reported as a fold); GATE-GAIT-04; none; CX-R22-e |
| `reflex-mode-seq` | A gait that alternates stride modes is stable if each mode's multipliers are inside the unit circle | A = [[1/2, 1], [0, 1/2]], B = Aᵀ: each alone has both multipliers 1/2; B·A = [[1/4, 1/2], [1/2, 5/4]] has eigenvalue 3/4 + 1/√2 ≈ 1.457, a growth of (1 + √2)/2 ≈ 1.2071 per stride | Stability over admitted stride sequences (lower bound by cycle products, upper bound by a common quadratic norm) | `per_mode_multiplier_test`; GATE-GAIT-05; none; CX-R22-f |
| `reflex-damp-gain` | A sampled damper is passive because the continuous one is | A free 1 kg slider at +1 m/s, gain 300 N·s/m held 10 ms (κ = 3): the speed ends at −2 m/s and the kinetic energy rises from 0.5 J to 2 J (+1.5 J of work by the damper) | The gain bound 2·M̄ − h·K_d ⪰ 0 (stored energy non-increasing at ticks); the refusal "sampled damping gain above the passivity bound" | `damping_gain_above_bound`; GATE-SHIELD-07; none; CX-R22-g (first part) |
| `reflex-held-work` | The work of a held command is the power at the tick times the period | Same slider: power at the start times the duration predicts −3 J; the work done is +1.5 J = F·v·h + F²·h²/(2m) = −3 + 4.5 J | The held-work bound F·v·h + F²·h²/(2m), reserved with the least admitted inertia | `work_as_initial_power`; GATE-SHIELD-07; none; CX-R22-g (second part) |
| `reflex-delay-kappa` | The undelayed gain bound κ < 2 also holds with one tick of actuation latency | With one tick of latency the scalar loop v′ = v − κ·v_prev at κ = 3/2 diverges with spectral radius √(3/2) although κ < 2 | κ < 1 under one tick of latency; the exact one-tick map of A-22 with a contraction in the energy norm | `undelayed_bound_on_delayed_loop`; GATE-SHIELD-07; none; CX-R22-g (third part) |

**Planting beads (DESIGN).** Each plant is built by the bead that owns the component it is planted in, and that
bead cites the entry; `plant` is written `<planting bead>:<plant name>` (for example `N22c-02:unconverged_reported_infeasible`),
and the planting bead may refine the name by updating the entry. CX-234 N22a-02 (the scoring rule; GATE-REFLEX-03
is measured by N22a-05); CX-235 N22c-02 (the certificate auditor, whose second mutant marks an `unconverged` run
`infeasible`); CX-236 N22b-02 (`space_rows_differ`); CX-237 N22a-32 (the planted transverse instability); CX-238
N22a-34 (the bar fixture; the forced-order solver is N22c-54's control); CX-239 N22b-11 (the payload-only budget
as planted mutant); CX-240 N22b-22; CX-241 BD-339 (the self-labelling mutant); CX-242 BD-336; CX-243 N22b-05 (the
fixed-base pullback, with the case of N22c-04 fixture 16); CX-244 N22b-37; CX-245 N22a-26 (the planted acceptance
request); `reflex-lift-resid` N22c-23 (admission of the lifted model; N22c-25 measures GATE-REFLEX-10);
`reflex-untimely` and `reflex-intersample` N22c-27; `reflex-privileged` N22c-32 (the delivered-controller lane;
N22c-33 measures GATE-REFLEX-11); `reflex-free-period` N22c-36; `reflex-mode-seq` N22c-41 (N22c-42 measures
GATE-GAIT-05); `reflex-damp-gain` and `reflex-delay-kappa` N22c-44; `reflex-held-work` N22c-45.

**The pre-slip entries and CX-012.** Counterexample CX-012 (§4, BET-25: two friction regimes with identical
histories before first slip, where a monitor alone cannot see the breach in time) is stated without numbers, and
its numeric instance is owed by the energy-budget tier of this chapter (N00-CX writes CX-012 with
`numeric = false`, so that its control is seeded as an obligation; N38b-05 lists it as obligation (b)).
`reflex-untimely` is that instance: it carries `same_as = "CX-012"` and `numeric = true`,
its `case` is the fixture of N22c-27 (registered by N22c-04), and its plant is the planted defect that CX-012's
control needs (CX-012 names the same plant, `N22c-27:list_untimely_assumption_as_monitored`). `reflex-intersample` uses the same fixture and also carries `same_as = "CX-012"`. Each entry still
gets its own control, caught by its own rule's gates, as the seeding tool requires. Once `reflex-untimely` is in
the catalogue, the next seeding fills CX-012's `case_supplied_by` with it, and CX-012's own control (rule BET-25,
with GATE-SHIELD-06 among its `rule_ids` in N00-CX) becomes runnable under the NC number it had as an
obligation (N38b-05); no entry of N00-CX is edited by hand.

**Withdrawn mechanisms.** If a mechanism's kill gate fires (KILL-SOILLIFT for the lifted model), the entry stays
in the catalogue as a guard against a later proposal of the same shortcut; its control is marked `pending` with the
ADR, and a block number is never marked `void` for an entry that was written.

**Registry check.** A host test of this bead walks the chapter's entries and fails if an entry has no `plant` (an
entry with `numeric = false` is the only exception, and this chapter has none after CX-243's case), if a `plant`
does not name its planting bead, if a `same_as` target is not an earlier entry, or if a gate in `rule_ids` is not
in `gates.toml`. Whether the plant flag exists under the test configuration is checked only once its planting bead
has built it: before that the entry is reported `pending: <bead>` with that bead's milestone, not failed (an
unbuilt plant is not a load error, BD-025), and a planting bead whose milestone has passed with the flag still
missing fails the check. The completeness join (every entry has exactly one control, every control's
gate lists it) is N38b-05's and is re-run by the script below.

## Acceptance criteria

- [ ] Twelve catalogue entries (CX-234 to CX-245) and the nine appended entries exist in `counterexamples.toml` with every field filled, each with a `plant` and the gates it must fail among its `rule_ids`.
- [ ] The nine appended entries carry numbers from a block issued to N22-CX in `allocations.toml`, in table order, with their tags in `slug`; `registry::alloc::validate` passes and no tag remains outside `slug` at the M1 exit.
- [ ] `reflex-untimely` and `reflex-intersample` carry `same_as = "CX-012"`; CX-243 carries the DESIGN case of N22c-04 fixture 16 with `numeric = true`, `plan_case = "words"` and `case_supplied_by` filled.
- [ ] After the seeding tool of N38b-05 is re-run, each of the 21 entries has exactly one control in `controls.toml` whose `seed` names it, the tool's second run changes no byte, and the join passes; no entry of this bead ends in the seeding tool's findings file.
- [ ] Each entry's arithmetic is reproduced by a host test in exact rationals or to the printed digits (values in the tables above).
- [ ] Every entry's `plant` names the planting bead listed above; an entry whose planting bead has not built its flag is reported `pending` with that bead, not failed.
- [ ] `scripts/e2e/reflex_negative_controls.sh` exits 0.
- [ ] No entry uses a mock: each `mutation` plant is a flag on the real component, impossible to enable in a release build.
- [ ] The earlier names `CX-R22-a` to `CX-R22-g` appear in no registry, test name or derivation file.

## Test plan

### Unit tests
`crates/frob-conformance/tests/cx_reflex.rs` (host arithmetic, exact where rational; tests of appended entries are
named by slug until the block is mapped and by CX number from the mapping commit on):
- `cx234_scale_fit_scores_zero`: similarity alignment of (2 m vs 1 m) returns 0; rigid alignment returns 1 m.
- `cx235_certificate_exact`: y = (1/2, −1), A = I, b = (50, 30), μ = 1/2: y ∈ K*, yᵀb = −5 exactly; nudging y to (0.49, −1) fails membership.
- `cx236_point_vs_space_rows`: the point Jacobian gives 25/3 N; the plant's bound differs.
- `cx237_restricted_vs_full`: multipliers e^(−T) and e^(+T) for T = 1 to 10⁻¹².
- `cx238_bar`: β = 1/3 gives first-end speed v/4 and energy ratio 1/16; β = 1 and β = 2 give rest for every order.
- `cx239_reflected_mass`: 2500·5e-5/0.0025 = 50 kg exactly.
- `cx240_projection`: breakpoint λ = 1/2, u = (1, 1/2); the plant's output (1, 1/4) violates the row.
- `cx243_floor_reaction`: 0.4·10·9.81 = 39.24 N exactly in decimal rationals; 50 ≤ 60 admits under the fixed-base map; 50 > 39.24 refuses.
- `cx245_episode_count`: ⌈ln 0.05 / ln 0.99⌉ = 299.
- `lift_resid_abs_lift`: the fitted row is (0, 0, 1); prediction 0.25 at 0.5.
- `untimely_and_intersample_cart`: 1.162 m/s, 0.2296 m, 0.0204, 0.0196 m to four digits.
- `privileged_cart`: 1.019 m, 0.255 m, 0.408 m, 0.612 m.
- `free_period_trivial_multiplier`: det(I − M) = 0 to 10⁻¹² for μ ∈ {0.1, 1, 3}.
- `mode_seq_product`: eigenvalue 3/4 + 1/√2 to 10⁻¹².
- `damp_gain_slider`: −2 m/s and +1.5 J at κ = 3; the bound 2·1 − 0.01·300 = −1 < 0 is violated and 2·1 − 0.01·100 = 1 ≥ 0 holds.
- `held_work_slider`: F·v·h = −3 J, F²·h²/(2m) = 4.5 J, sum +1.5 J exactly.
- `delay_kappa_radius`: the companion matrix [[1, −κ], [1, 0]] at κ = 3/2 has spectral radius √(3/2) to 10⁻¹²; at κ = 0.99 the radius is below 1.
- `registry_entries_complete`: every entry of the chapter has a `plant` naming its planting bead, `same_as` targets resolve, and gates in `rule_ids` exist; a plant flag that the planting bead has built exists under the test configuration, and the others are listed `pending` with their bead.

All run as `FROB_TEST_SEED=0 cargo test -p frob-conformance --test cx_reflex`.

### End-to-end script and logging
`scripts/e2e/reflex_negative_controls.sh` (helper of STD-E2E; it calls only cargo targets of its dependencies):
runs the unit tests above; re-runs N38b-05's seeding tool, `cargo test -p frob-conformance --test seed_controls --
--ignored seed`, twice, asserting one control per entry and that the second run leaves the digest of the committed
`controls.toml` unchanged; runs the join, `cargo test -p frob-conformance --test seed_join`; then reads the control
runner's history `registry/control_history.ndjson` (N38b-04) and reports each control's latest outcome. Running a
gate's conformance target with the plant off and on is the control runner's, invoked by the planting beads' own
scripts, and is not repeated here. Each step logs, one NDJSON line per step: the command, exit code, wall time, the
CX number and slug, the control's `NC-` identifier, the plant and its planting bead, the gate, the state
(`runnable`, `stale` while the plant is unbuilt) and the latest outcome; the digest of `controls.toml` before and
after the second seeding; on failure the first missing entry or unresolved gate. The summary lists entries
`caught`, `missed` (a failure of this script), `never run` and `pending` (plant not yet built, with the bead
named).

### Known answers and negative controls
This bead is itself the counterexample catalogue of the chapter. Its own controls, each on a scratch copy: an
entry with no `plant`, and one whose `plant` names no planting bead (`registry_entries_complete`); a `same_as` that
names a later entry; a gate id misspelled; an appended entry numbered outside N22-CX's block (the ledger's
`unallocated-number`); a history line `missed` for one control, which the script must report as a failure. A plant
that is a no-op flag ("a control that cannot fail") is reported by the control runner as `missed` (N38b-04). Determinism: every entry is host arithmetic in exact rationals or `f64`
with stated digits; results are identical run to run and across platforms.

## Considerations

- CX-243 has no numbers in the source; the fixture bead supplies the 39.24 N case above. If the skill layer's owner
  prefers another case, the entry keeps its number and the case is replaced, with the plant unchanged.
- The nine appended entries must not be dropped when the catalogue is regenerated; their slugs stay in `slug` so
  that recorded gate rows that cite them remain resolvable.
- Entries are seeds: mechanism beads add implementation-level plants of their own and enter them in the catalogue
  through this chapter's registry, never as hand-written rows of `controls.toml`.
- Owed by the planting beads and the control runner, not by this registry: for every entry whose mechanism exists, the conformance run with the plant enabled fails the listed gate and with it disabled passes, with a recorded failing run per plant.
- A plant flag is test-only and must be impossible to enable in a release build (compile-time feature, checked by
  the release rubric).

## References

Appendix N (N.19; CX-012), §22.16; §22.20 to §22.25 for the appended entries; §0.7 (counterexamples are
first-class); §38 (negative-control registry). N38b-05 (catalogue schema, seeding); N38a-40 (allocation ledger);
N00-CX (CX-012). Gates: GATE-REFLEX-03, -05, -06, -07, -08, -10, -11; GATE-GAIT-01, -02, -04, -05; GATE-SHIELD-01,
-02, -04, -06, -07.
