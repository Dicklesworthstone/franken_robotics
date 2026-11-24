---
key: "N22-REF"
title: "registry: the refusal codes of Reflex (FROB-REF-421 to -464 and -817) with triggers, repairs and exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "reflex", "frob-control", "frob-gait", "frob-shield", "registry", "solid"]
parent: "WP-12"
deps: ["BD-005", "BD-022", "N35x-02", "STD-E2E"]
needs: []
related: ["N14bx-01", "N22a-08", "N22a-24", "N22a-26", "N22a-30", "N28ap1-02", "N22a-33", "N22a-38", "N22a-39", "N22a-40", "N22c-23", "N22c-24", "N22c-28", "N22c-31", "N22c-32", "N22c-36", "N22c-37", "N22c-44", "N22c-49", "N35-33", "NAB-36"]
gates: []
plan_refs: ["§22.14", "Appendix O", "§32.4", "§35", "§22.8", "§22.12.4", "§22.20", "§22.21", "§22.22", "§22.23", "§22.25"]
---
## Background

Reflex is the subsystem that owns every closed loop that needs no learning (the state estimator, the baseline
controller synthesized from the model, the sampling-based predictive controller, gait-orbit analysis, skill
artifacts) and the safety layers that wrap any loop (the energy budget and the runtime shields). Its crates are
`frob-control` and `frob-gait` (layer 4) and `frob-shield` (layer 5).

The project's rule is that a verb never fails bare. A request that cannot be answered returns a **refusal**: a
stable code, a message, a pointer to the field at fault, the observed and required values, and **ranked repairs**
that an agent can act on. Codes live in the machine-readable registry `registries/refusals.toml`; an unknown code is
a load-time error, and a code is never reused. The CLI, the daemon protocol and the MCP tool definitions are
generated from the verb registry, which lists for each verb the refusals it may return, so the surfaces cannot
drift from the registry.

This bead is the registry of the eleven codes that Reflex defines: ten in the `4xx` family (control, gaits, skills,
shields, symmetry, estimation) and one in the evidence family (`817`). It exists so that no code, trigger or
repair is lost when the mechanisms are built by different people, and so that each code has one test that fires
it and one negative control. It also carries six refusals that later subsections state in words (four in §22.20 to §22.25, one
implied by the margin rule of §22.12.4, and the screening-episode cap of the sampling controller in §22.8) and that
have no number in the plan, through the project's refusal allocation ledger.

## Scope

In: the registry rows for the eleven codes; the payload each must carry; the exit code; the verbs that may return
each; a registry test that every code is reachable and that its repairs are well-formed; the full registry entries
of the six worded refusals once the ledger assigns their numbers. Out: the mechanisms that raise the codes (the
beads named in each row); the ledger rows of the six worded refusals and the run that numbers them (N35x-02); the
allocation rule and the registry's cross-chapter checks (N35-33).

## Technical approach

**Row format (DESIGN).** One `[[refusal]]` table per code in `registries/refusals.toml`: `code`, `slug`, `family`,
`message` (one sentence, no jargon), `pointer_kind` (what `refusal.pointer` addresses), `payload` (named fields with
units), `retry` (`never`, `after-change` or `backoff`), `repairs` (ranked list; each either a typed edit, a verb
to call next, or a text instruction), `exit`, `defined_in` and `raised_by` (the bead keys listed under the
table, the field N35-33's registry-wide check reads). The envelope written by every verb with `--json`
carries the refusal as `refusal { code, message, pointer, observed, required, unit, affected_claims, retry,
repairs }` when `ok` is false.

**Exit code.** Every code below exits **3** ("refused input or question": the rule is that `FROB-REF-1xx` to `5xx`
and `8xx` other than conflicts exit 3). None of them is a budget stop (4), a missing capability (5), a device
fault (6) or a conflict (7).

**The eleven codes.**

| Code | Meaning | Trigger (exact) | Payload beyond the standard fields | Ranked repairs | Raised by | Exit |
|---|---|---|---|---|---|---|
| FROB-REF-421 | No synthesis route exists for the detected morphology class and no fallback is admitted | `control.synth` detected a class for which no baseline route is built or admissible (for example no gait in the atlas satisfies static stability with these leg counts), **and** the caller needs a controller and has admitted neither the sampling controller nor rung R4 (a trained policy) | detected class; the reason; the missing ports or sensors; the dual certificate when the reason is a program | 1. Add the ports or sensors the class requires. 2. Admit the sampling controller for screening. 3. Go to rung R4 | `control.synth` under `--require`; any run verb that needs a baseline (`sim.run`, `eval.run`) | 3 |
| FROB-REF-424 | A control program is infeasible and the controller has no admissible fallback | At a control tick the conic program ended `infeasible` (a dual certificate exists) or `unconverged`, and neither the last feasible command nor a backup controller is admissible | tick; program name; active inequalities; residuals; the certificate y (rational); the binding actuator or contact | 1. Read the dual certificate. 2. Relax the task. 3. Change the binding actuator or contact | the cone programs and their prescribed fallbacks (N22a-08, M1), at synthesis and in the controllers of `frob-control` at run time; surfaced by `sim.run`, `control.synth` | 3 |
| FROB-REF-431 | A gait derivative was requested at grazing or at a fold | `gait.gradient` or a monodromy assembly where the transversality number abs(Dg·f⁻) is below its threshold (grazing), or where I − M is singular in the trivial block (a fold; for an autonomous loop, whose I − M is always singular, the test is on the section map I − D P_Σ (N22c-36) or on the bordered matrix (N22c-37)) | the offending state; the perturbation directions; the one-sided estimates; the transversality number or the singular value | 1. Use the one-sided estimates returned. 2. Move the parameter. 3. Derivative-free search | `gait.gradient`, `gait.orbit`, `gait.continue` | 3 |
| FROB-REF-444 | A transported skill does not refine its specification | A refinement check found a target state, abstract command and disturbance for which some target successor is related to no allowed abstract successor (for example successors {stable, fall} against an abstract transition whose only successor is stable) | the uncovered successor; the target state; the command; the disturbance | 1. Add a guard. 2. Narrow the entry set | `skill.check`, `skill.transport` | 3 |
| FROB-REF-448 | A skill's task wrench lies outside the body's attainable set | The allocation program for a contact mode is infeasible: a direction c in task space has support value max cᵀ(w₀ + B_m·u) below cᵀw* | direction c; support value; requirement; gap; contact mode; binding actuator or contact; the exact certificate | The witness names the binding actuator or contact: 1. regear. 2. reshape. 3. change the wrench profile | `skill.compile`, `skill.check`, `skill.transport` (an infeasible target is refused before any learned component runs, N22b-07 step 1); the analysis A-24 `analyze.wrench` (N28ap1-02, M4) and `design.check` stage 6 through its row for A-24 (N14bx-01) | 3 |
| FROB-REF-451 | An energy budget omits a source in the port graph, or a class above `Estimated` is requested on an `assumed` absorber envelope | `shield.budget` walked the design's port graph and found a port that can inject power into the load path and is neither bounded in the budget nor cut by the monitor; or the request asks for class `Certified` or `Tested` while the absorber card's provenance is `assumed`; or (N22bx-01) the budget's conditions (temperature, rate bound, cycle count, mounting orientation) fall outside the absorber card's, or a non-reusable card with prior cycles ≥ 1 is used, so the absorber term is unavailable; or (N22bx-01) a `.fenergy` document read against its card (`shield.check <run> --energy-budget <file>`) has a `w_abs_j` above the card's recomputed W_abs | the unlisted port and its path to the boundary; or the requested class and the card's provenance; or `pointer` at the failing card condition (or the card's cycle count); or `pointer` `absorber.w_abs_j` with `observed` the copy and `required` the recomputed value | Omitted source: 1. Bound the source or cut it. Class on an `assumed` card: 1. Import the absorber measurement. Conditions not covered, or a used non-reusable card: 1. Use a card whose conditions cover the case, or measure. Copy above the card: 1. Rebuild the budget with `shield.budget` | `shield.budget`, `shield.check` | 3 |
| FROB-REF-454 | A shield's rows have no common admissible command on the calibration set, or more than four are coupled | `shield.fit` found a calibration state where the rows and the actuator box have empty intersection; or the fitted row set has more than four coupled rows | the state; the conflicting rows; the box; the count of coupled rows | 1. Reduce the row set. 2. Widen the actuator box. 3. Lower the speed limit | `shield.fit` | 3 |
| FROB-REF-457 | A shield margin is used outside its scope key | A shield was asked to run with a measured margin ε̂ whose key (design class, controller or policy identity, task family, tier) differs from the run's in any component | the stored key; the run's key; the differing component | Recalibrate for the new policy, class, task family or tier | `shield.fit` (a margin offered for reuse, or found stored, under another key: BD-338 step 4, `--reuse-margin`), `shield.check`, any run with a shield stage | 3 |
| FROB-REF-461 | A symmetric reduction was requested under a group not contained in Γ_use | A gain tying, a block diagonalization or a reduced orbit solve named a group element outside Γ_use = Γ_obs ∩ Stab(gravity) ∩ Stab(boundary conditions) ∩ Stab(contact mode) ∩ Stab(commanded twist); or the reduced and full models disagree on either side of a mode switch | the requested group; Γ_use; the generators outside it and the stabilizer that removed each | 1. Use Γ_use. 2. Remove the asymmetry with the typed edit that analysis A-03 proposes | `control.symmetry`, `control.synth`, `gait.orbit`; `sim.run` under `--verify-reduction` (N22b-38: a reduced controller that disagrees with its full twin, raised with the tick and the mode) | 3 |
| FROB-REF-464 | An estimator was scored with a fitted scale, or an estimated-state run lacks a sensor the filter needs | A scoring request asked for a similarity alignment (scale fitted); or `--state estimated` was requested on a design without an IMU, encoders or contact switches (legged) or wheel odometry (wheeled) | the alignment requested; or the missing sensor and the filter that needs it | 1. Score with translation and yaw alignment only. 2. Add the sensor, or run privileged and say so | `control.estimate`, `eval.run`; and for `--state estimated` (named, or the default for a run whose controller reads state) the run admission of N22a-03 on `sim.run` and `eval.compare` (every evaluation, §22.3) | 3 |
| FROB-REF-817 | Sampling-controller episodes were offered as the scoring source of acceptance, promotion or qualification | Scoring episodes have controller kind `mpc-sample` and the request is an acceptance, a promotion or a qualification, **unless** the task declares the sampling controller as the delivered controller and a frozen planner record meets all four lane conditions (not model-privileged; a plant the planner does not own; measured latency; the same bar). When the lane is declared and a condition fails, the same code fires and names the missing condition | controller kind; the request kind; the planner record digest if any; the failing condition (`model-privileged`, `plant-owned`, `latency-unmeasured`, `bar-differs`, `no-record`; at freeze also `planning-model-not-compiled`, N22c-31) | 1. Train or fine-tune a policy for the finalist. 2. Keep the sampling result as screening | `eval.qualify`, `eval.promote`, acceptance in `frob-arena` | 3 |

**`raised_by` per code (bead keys; the verbs of the table are reached through these beads).** 421: BD-118,
N22a-12. 424: N22a-08. 431: N22a-33 (grazing and the re-raised 263), N22c-37 (folds of free-period orbits),
BD-334 (`gait.gradient`). 444: N22b-04, BD-336. 448: BD-336, N22b-07, N28ap1-02, N14bx-01. 451: BD-337, N22b-11, N22bx-01 (the absorber card's conditions and the `.fenergy` copy).
454: N22b-20, N22b-23, BD-338. 457: N22b-29 (the scope key), BD-338. 461: N22b-37, N22b-38, BD-327, BD-333,
N22a-31. 464: N22a-02, N22a-03, N22a-04. 817: N22a-25, N22c-31 (the frozen planner record).

**Refusals stated in words by later subsections (no number in the plan).** Six refusals are defined with a
message and a payload and without a code. Their numbers are not typed by any Reflex bead: each is a row of the
allocation ledger `registries/refusal_allocations.toml`, filed under the slug below (family `4xx`) by N35x-02 and
numbered by the pure allocation rule of N35-33 (deduplicate; family by what is refused; the lowest free number in
the family above the defining chapter's highest code). The round runs at M0, before this bead and before any
raising bead, so each raising bead raises its refusal under the assigned number from the start, through one
named constant. This bead enters each assigned number, read from the generated hand-off page
`docs/generated/refusal_round2.md`, as a full registry entry (`code`, `slug`, `family`, `message`,
`pointer_kind`, `payload`, `retry`, `repairs`, exit 3, `defined_in`); the slug stays the entry's `slug` and the
stable key used in tests. DESIGN: no Reflex bead takes "the next free `4xx` number" itself, because two chapters
numbering in parallel could collide, which the ledger exists to prevent. A row the run leaves `held` keeps the
form rule 7 of N35-33 admits (an existing code with `detail` equal to the slug, or none, the raising bead then
staying open on the number).

| Ledger slug | Refused | Payload | Repairs | Raised by | Exit |
|---|---|---|---|---|---|
| `lifted-planner-out-of-regime` | A query to a lifted planning model outside its calibrated regime, or a planner tick whose measured state leaves the regime box | the regime field violated (soil card range, speed or yaw box, slope range, track contact); the measured value | The tick is served by `mpc-sample` on the compiled model; refit for the new regime | `control.lift`, `control.mpc --planner lifted:<id>` | 3 for a query; a per-tick event (not an exit) inside a run |
| `fixed-period-shooting-on-free-period` | A stroboscopic (fixed-period) orbit solve on a loop whose `ControllerId` declares phase source `event-reset` or `none` | the phase source; the available transverse events | Use the section form on a named event, or the bordered form with a phase condition | `gait.orbit`; `gait.continue` on an explicit fixed-period request (`--form stroboscopic`) for such a loop (BD-335) | 3 |
| `monitored-assumption-not-timely` | A shield lists as monitored an assumption whose monitor cannot put the state in the backup's domain in time | the first distinguishing tick t_d; the monitor latency ℓ; the state at t_d + ℓ; the two admitting changes with numbers | 1. Move the assumption's whole range inside the quantifier (widen the row). 2. Apply one of the printed admitting changes (earlier trigger; lower speed limit) | `shield.fit` | 3 |
| `sampled-damping-above-passivity-bound` | A damping-injection backup gain that violates 2·M̄ − h·K_d ⪰ 0 (or, with latency or stored energy in the loop, a one-tick map that does not contract in the energy norm) | M̄ (least reflected inertia), h, K_d, the smallest eigenvalue of 2·M̄ − h·K_d as a rational | Lower the gain inside the no-reversal bound M̄ − h·K_d ⪰ 0; shorten the control period; use a physical damper (brake, shorted winding) | `shield.fit` | 3 |
| `insufficient-calibration` | A measured shield margin requested from a calibration set too small for the level: n < (1 − α)/α, so the ⌈(n + 1)·(1 − α)⌉-th smallest score does not exist and no finite margin ε̂ can be stated (§22.12.4; for α = 10⁻³ the minimum is 999) | n; α; the needed count ⌈(1 − α)/α⌉ | 1. Run more calibration episodes, up to the needed count. 2. Calibrate at a larger α, which OPEN-SHIELD-2 fixes (default 10⁻³) and only the owner changes | `shield.fit` (N22b-29 step 2; N22b-32 step 14) | 3 |
| `mpc-episode-cap` | A screening request to the sampling controller beyond its cap (§22.8): the default-allowance episodes on record under the scope key (the campaign, or outside a campaign the caller's namespace; `DesignId`; `TaskId`), whatever the planner setting was, plus those requested exceed three; or a request under a named exception (`--compare`, `--protocol`, `--demonstrations`) exceeds that exception's own count. Nothing is admitted: no job, no episode. Not counted: a replay under the same idempotency key, `sim.replay`, `--diag-replay` | `requested`; `cap`; the exception named, if any, with its count; the episodes on record under the scope key with their run identities; steps per episode; the cost against training once; the class's break-even count | 1. Ask for at most the remaining allowance. 2. Name a paired comparison (`--compare`), a registered protocol (`--protocol`) or a distillation job (`--demonstrations`). 3. Train a policy for the design, which the arithmetic shows is cheaper beyond the break-even count | `control.mpc` at admission (the rule: N22a-24, M3; asserted end to end by N22a-26 step 3) | 3 |

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the eleven rows above with every column filled, and loads without error.
- [ ] Each of the eleven codes is listed in the `refusals` field of every verb that can return it, and the generated refusal table shows it.
- [ ] For each code one test fires it through `frob --json` and asserts `ok: false`, the code, exit 3, a non-empty `pointer` and the payload fields of the table.
- [ ] For each code one negative control shows the request passing when the trigger is removed.
- [ ] The six worded refusals exist as rows of `refusal_allocations.toml` (filed by N35x-02: the four of §22.20 to §22.25, `insufficient-calibration` and `mpc-episode-cap`); this bead enters each assigned number as a full registry entry in one commit, before the gate of its subsection is recorded, and the slug remains the entry's `slug`.
- [ ] Each raising verb returns the assigned number with the slug, exit 3, the payload fields and ranked repairs, never a generic error; a row the run left `held` fires in its rule-7 form.
- [ ] No Reflex verb returns exit 1, and none returns a refusal without ranked repairs.

## Test plan

### Unit tests
In `crates/frob-core/tests/refusal_registry.rs` and the owning crates:
- `reflex_codes_load`: the registry parses; the set of codes with `defined_in = "§22.14"` is exactly {421, 424, 431, 444, 448, 451, 454, 457, 461, 464, 817}.
- `reflex_codes_exit_3`: every one maps to exit 3.
- `reflex_repairs_ranked`: each row has at least one repair, and the first repair of each matches the table.
- `ref_817_names_missing_condition`: with a declared lane and a record whose `state_source = privileged`, the payload's failing condition is `model-privileged`; with no record it is `no-record`.
- `ref_451_four_triggers`: an unlisted spring gives the "omits a source" form; a `Certified` request on an `assumed` absorber gives the class form; a budget whose temperature lies outside the absorber card's range gives the conditions form with `pointer` at the condition (with N22bx-01, M4); a `.fenergy` copy with `w_abs_j` "2.50" on `bumper-50N-40mm` gives `pointer` `absorber.w_abs_j` (N22bx-01's `document_copy_cannot_exceed_card`); the payloads and first repairs differ.
- `ref_454_two_triggers`: an empty intersection and a five-row coupled set both fire, with different pointers.
- `unknown_code_is_load_error`: a row citing `FROB-REF-499` from a verb entry with no registry row fails the load.
- `uncoded_slugs_in_ledger`: the six slugs exist in `refusal_allocations.toml` with family `4xx` and `defined_in` in §22.20 to §22.25 (§22.12.4 for `insufficient-calibration`, §22.8 for `mpc-episode-cap`); for each row whose status is `assigned`, `refusals.toml` holds an entry with that number and slug, and no Reflex bead's test or registry file types a number the ledger did not assign.
- `worded_refusals_fire`: each of the six fires through its verb under the number its ledger row assigns (or, for a `held` row, in its rule-7 form), with the slug, exit 3 and the payload fields of the table.

### End-to-end script and logging
`scripts/e2e/reflex_refusals.sh` drives the real `frob` binary once per code with its smallest triggering input
(listed below), then once with the trigger removed; the six worded refusals are driven the same way by slug and
logged with their ledger status (the assigned number, or `held` with its rule-7 form). Per STD-E2E it logs for each step the command, the exit code,
the envelope's `refusal` block, the `evidence` block, and an `assert` line per checked field; the final `summary`
counts codes fired, controls passed and names any code whose mechanism is not yet built at the current milestone
(reported as `not run`, never as a pass).

### Known answers and negative controls
Smallest triggers, with values:
- 421: a legged design with one leg, no admitted fallback. Control: admit the sampling controller, outcome `not-synthesizable` without a refusal.
- 424: the friction fixture as a run-time program (weight 50 N, push 30 N, μ = 0.5) with no fallback admitted; the certificate y = (1/2, −1) with yᵀb = −5 N is in the payload. Control: a fallback admitted gives a `fallback` event, no refusal.
- 431: a planted fold, the planar loop ṙ = r·(θ + r² − r⁴), φ̇ = 1 (`synthetic`, autonomous), whose orbits r*² = (1 ± √(1 + 4θ))/2 merge at θ = −1/4, r* = 1/√2, where the section multiplier e^(2π·2r*²·(1 − 2r*²)) equals 1; and a ball whose apex grazes a ceiling. Control: at θ = −0.24 on the outer orbit (r*² = 0.6, section multiplier e^(−0.48π) = 0.2214) `gait.gradient` returns dr*/dθ = 1/(2r*·(2r*² − 1)) = 3.2275. (The rimless wheel is not a fold fixture: its multiplier is cos²(2α) on every slope, so it has no fold.)
- 444: the {stable, fall} target (KA-566, CX-242).
- 448: the two-link arm, links 0.4 m and 0.3 m at angles 0 and π/2, limit 2.5 N·m, required (10, 0) N: support 25/3 N, binding joint 2 (KA-565).
- 451: the worked energy card with an unlisted series spring (KA-568).
- 454: a box [0, 1]² with rows u₁ + u₂ ≥ 2.5 (empty) ; a five-row coupled set.
- 457: a margin calibrated for policy P reused with policy P′ (KA-573's control).
- 461: tying left and right gains on a side slope (KA-575).
- 464: the 2 m estimate of a 1 m motion scored with a fitted scale (KA-554, CX-234).
- 817: a planted acceptance request over `mpc-sample` episodes with no frozen record (KA-558's control, CX-245).
- `insufficient-calibration`: `shield.fit` with 998 calibration episodes at α = 10⁻³; the payload names the needed count 999. Control: 999 episodes give a finite margin, the ⌈1000·0.999⌉ = 999th smallest score.
- `mpc-episode-cap` (with N22a-24, M3; the end-to-end form is N22a-26 step 3, in a fresh session namespace): `control.mpc` on `quad-ref` with `--episodes 4` and no exception is refused at admission with requested 4, cap 3, episodes on record 0; then `--episodes 2` is admitted and a second request for two episodes of the same design and task under another key and another planner setting is refused naming the two episodes on record and the first run's identity. Control: `--episodes 1` after the first two is admitted; the same second request for an edited design (a new `DesignId`) is admitted.

## Considerations

- Two codes have more than one trigger (451 four, 454 two) and one has a conditional exemption (817). Tests must
  cover each branch; a payload that does not say which branch fired is a defect.
- FROB-REF-421 is distinct from the outcome `not-synthesizable`: the outcome is a result of `control.synth` (the
  verb succeeded in saying that no route exists); the refusal fires only when a caller needs a controller and has
  admitted no fallback. Conflating them would make a healthy answer look like an error to an agent.
- Grazing in the general derivative machinery has its own code in the dynamics chapter (FROB-REF-263); 431 is the
  gait-level refusal and carries the one-sided estimates. A gait request refused below by 263 is re-raised as 431
  with the lower refusal attached.
- The six worded refusals have no number in the plan's registry. Allocation is a ledger action (N35x-02, N35-33);
  inventing numbers in separate beads would risk a collision. If a chapter contests the `4xx` family, the row is
  `held` with the reason and keeps its rule-7 form until the two beads agree.

## References

§22.14, Appendix O (O.4 and the evidence family), §32.4 (envelope and exit codes), §35 (refusal form); worded
refusals from §22.20, §22.21, §22.23, §22.25, §22.12.4 and §22.8; lane conditions from §22.22. INV-REFLEX-03, INV-REFLEX-04. N35x-02
(ledger rows), N35-33 (allocation rule).
