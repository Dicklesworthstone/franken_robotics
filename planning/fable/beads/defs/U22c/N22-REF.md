---
key: "N22-REF"
title: "registry: the refusal codes of Reflex (FROB-REF-421 to -464 and -817), with triggers, payloads, repairs and exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "reflex", "frob-control", "frob-gait", "frob-shield", "registry", "solid"]
parent: "WP-12"
deps: ["BD-005", "BD-022"]
needs: []
related: ["N22-CX", "BD-118", "BD-114", "BD-251", "BD-252", "BD-327", "BD-334", "BD-336", "BD-337", "BD-338", "BD-340"]
gates: []
plan_refs: ["§22.14", "Appendix O", "§32.4", "§35", "§22.20", "§22.21", "§22.22", "§22.23", "§22.25"]
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
it and one negative control. It also records four refusals that later subsections state in words and that have no
number yet.

## Scope

In: the registry rows for the eleven codes; the payload each must carry; the exit code; the verbs that may return
each; a registry test that every code is reachable and that its repairs are well-formed; the four worded refusals
with proposed slugs. Out: the mechanisms that raise the codes (the beads named in each row).

## Technical approach

**Row format (DESIGN).** One `[[refusal]]` table per code in `registries/refusals.toml`: `code`, `slug`, `family`,
`message` (one sentence, no jargon), `pointer_kind` (what `refusal.pointer` addresses), `payload` (named fields with
units), `retry` (`never`, `after-change` or `backoff`), `repairs` (ranked list; each either a typed edit, a verb
to call next, or a text instruction), `exit` and `defined_in`. The envelope written by every verb with `--json`
carries the refusal as `refusal { code, message, pointer, observed, required, unit, affected_claims, retry,
repairs }` when `ok` is false.

**Exit code.** Every code below exits **3** ("refused input or question": the rule is that `FROB-REF-1xx` to `5xx`
and `8xx` other than conflicts exit 3). None of them is a budget stop (4), a missing capability (5), a device
fault (6) or a conflict (7).

**The eleven codes.**

| Code | Meaning | Trigger (exact) | Payload beyond the standard fields | Ranked repairs | Raised by | Exit |
|---|---|---|---|---|---|---|
| FROB-REF-421 | No synthesis route exists for the detected morphology class and no fallback is admitted | `control.synth` detected a class for which no baseline route is built or admissible (for example no gait in the atlas satisfies static stability with these leg counts), **and** the caller needs a controller and has admitted neither the sampling controller nor rung R4 (a trained policy) | detected class; the reason; the missing ports or sensors; the dual certificate when the reason is a program | 1. Add the ports or sensors the class requires. 2. Admit the sampling controller for screening. 3. Go to rung R4 | `control.synth` under `--require`; any run verb that needs a baseline (`sim.run`, `eval.run`) | 3 |
| FROB-REF-424 | A control program is infeasible and the controller has no admissible fallback | At a control tick the conic program ended `infeasible` (a dual certificate exists) or `unconverged`, and neither the last feasible command nor a backup controller is admissible | tick; program name; active inequalities; residuals; the certificate y (rational); the binding actuator or contact | 1. Read the dual certificate. 2. Relax the task. 3. Change the binding actuator or contact | controllers of `frob-control` at run time; surfaced by `sim.run`, `control.synth` | 3 |
| FROB-REF-431 | A gait derivative was requested at grazing or at a fold | `gait.gradient` or a monodromy assembly where the transversality number abs(Dg·f⁻) is below its threshold (grazing), or where I − M is singular in the trivial block (a fold) | the offending state; the perturbation directions; the one-sided estimates; the transversality number or the singular value | 1. Use the one-sided estimates returned. 2. Move the parameter. 3. Derivative-free search | `gait.gradient`, `gait.orbit`, `gait.continue` | 3 |
| FROB-REF-444 | A transported skill does not refine its specification | A refinement check found a target state, abstract command and disturbance for which some target successor is related to no allowed abstract successor (for example successors {stable, fall} against an abstract transition whose only successor is stable) | the uncovered successor; the target state; the command; the disturbance | 1. Add a guard. 2. Narrow the entry set | `skill.check`, `skill.transport` | 3 |
| FROB-REF-448 | A skill's task wrench lies outside the body's attainable set | The allocation program for a contact mode is infeasible: a direction c in task space has support value max cᵀ(w₀ + B_m·u) below cᵀw* | direction c; support value; requirement; gap; contact mode; binding actuator or contact; the exact certificate | The witness names the binding actuator or contact: 1. regear. 2. reshape. 3. change the wrench profile | `skill.compile`, `skill.check` | 3 |
| FROB-REF-451 | An energy budget omits a source in the port graph, or a class above `Estimated` is requested on an `assumed` absorber envelope | `shield.budget` walked the design's port graph and found a port that can inject power into the load path and is neither bounded in the budget nor cut by the monitor; or the request asks for class `Certified` or `Tested` while the absorber card's provenance is `assumed` | the unlisted port and its path to the boundary; or the requested class and the card's provenance | 1. Bound the source or cut it. 2. Import the absorber measurement | `shield.budget`, `shield.check` | 3 |
| FROB-REF-454 | A shield's rows have no common admissible command on the calibration set, or more than four are coupled | `shield.fit` found a calibration state where the rows and the actuator box have empty intersection; or the fitted row set has more than four coupled rows | the state; the conflicting rows; the box; the count of coupled rows | 1. Reduce the row set. 2. Widen the actuator box. 3. Lower the speed limit | `shield.fit` | 3 |
| FROB-REF-457 | A shield margin is used outside its scope key | A shield was asked to run with a measured margin ε̂ whose key (design class, controller or policy identity, task family, tier) differs from the run's in any component | the stored key; the run's key; the differing component | Recalibrate for the new policy, class, task family or tier | `shield.check`, any run with a shield stage | 3 |
| FROB-REF-461 | A symmetric reduction was requested under a group not contained in Γ_use | A gain tying, a block diagonalization or a reduced orbit solve named a group element outside Γ_use = Γ_obs ∩ Stab(gravity) ∩ Stab(boundary conditions) ∩ Stab(contact mode) ∩ Stab(commanded twist); or the reduced and full models disagree on either side of a mode switch | the requested group; Γ_use; the generators outside it and the stabilizer that removed each | 1. Use Γ_use. 2. Remove the asymmetry with the typed edit that analysis A-03 proposes | `control.symmetry`, `control.synth`, `gait.orbit` | 3 |
| FROB-REF-464 | An estimator was scored with a fitted scale, or an estimated-state run lacks a sensor the filter needs | A scoring request asked for a similarity alignment (scale fitted); or `--state estimated` was requested on a design without an IMU, encoders or contact switches (legged) or wheel odometry (wheeled) | the alignment requested; or the missing sensor and the filter that needs it | 1. Score with translation and yaw alignment only. 2. Add the sensor, or run privileged and say so | `control.estimate`, `eval.run` | 3 |
| FROB-REF-817 | Sampling-controller episodes were offered as the scoring source of acceptance, promotion or qualification | Scoring episodes have controller kind `mpc-sample` and the request is an acceptance, a promotion or a qualification, **unless** the task declares the sampling controller as the delivered controller and a frozen planner record meets all four lane conditions (not model-privileged; a plant the planner does not own; measured latency; the same bar). When the lane is declared and a condition fails, the same code fires and names the missing condition | controller kind; the request kind; the planner record digest if any; the failing condition (`model-privileged`, `plant-owned`, `latency-unmeasured`, `bar-differs`, `no-record`) | 1. Train or fine-tune a policy for the finalist. 2. Keep the sampling result as screening | `eval.qualify`, `eval.promote`, acceptance in `frob-arena` | 3 |

**Refusals stated in words by later subsections (no number yet).** Four refusals are defined with a message and a
payload and without a code. DESIGN: each gets a slug now, and its number is allocated from the free `4xx` range by
whoever adds the row to `registries/refusals.toml` (the registry's own rule is that codes are never reused and that
allocation is serial in the registry file, so that two authors cannot pick the same number). Until then the slug
is the stable key used in tests.

| Slug (proposed) | Refused | Payload | Repairs | Raised by | Exit |
|---|---|---|---|---|---|
| `lifted-planner-out-of-regime` | A query to a lifted planning model outside its calibrated regime, or a planner tick whose measured state leaves the regime box | the regime field violated (soil card range, speed or yaw box, slope range, track contact); the measured value | The tick is served by `mpc-sample` on the compiled model; refit for the new regime | `control.lift`, `control.mpc --planner lifted:<id>` | 3 for a query; a per-tick event (not an exit) inside a run |
| `fixed-period-shooting-on-free-period` | A stroboscopic (fixed-period) orbit solve on a loop whose `ControllerId` declares phase source `event-reset` or `none` | the phase source; the available transverse events | Use the section form on a named event, or the bordered form with a phase condition | `gait.orbit` | 3 |
| `monitored-assumption-not-timely` | A shield lists as monitored an assumption whose monitor cannot put the state in the backup's domain in time | the first distinguishing tick t_d; the monitor latency ℓ; the state at t_d + ℓ; the two admitting changes with numbers | 1. Move the assumption's whole range inside the quantifier (widen the row). 2. Apply one of the printed admitting changes (earlier trigger; lower speed limit) | `shield.fit` | 3 |
| `sampled-damping-above-passivity-bound` | A damping-injection backup gain that violates 2·M̄ − h·K_d ⪰ 0 (or, with latency or stored energy in the loop, a one-tick map that does not contract in the energy norm) | M̄ (least reflected inertia), h, K_d, the smallest eigenvalue of 2·M̄ − h·K_d as a rational | Lower the gain inside the no-reversal bound M̄ − h·K_d ⪰ 0; shorten the control period; use a physical damper (brake, shorted winding) | `shield.fit` | 3 |

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the eleven rows above with every column filled, and loads without error.
- [ ] Each of the eleven codes is listed in the `refusals` field of every verb that can return it, and the generated refusal table shows it.
- [ ] For each code one test fires it through `frob --json` and asserts `ok: false`, the code, exit 3, a non-empty `pointer` and the payload fields of the table.
- [ ] For each code one negative control shows the request passing when the trigger is removed.
- [ ] The four worded refusals are registered with allocated numbers before the gate of their subsection is recorded, and their slugs appear in this bead's test names.
- [ ] No Reflex verb returns exit 1, and none returns a refusal without ranked repairs.

## Test plan

### Unit tests
In `crates/frob-core/tests/refusal_registry.rs` and the owning crates:
- `reflex_codes_load`: the registry parses; the set of codes with `defined_in = "§22.14"` is exactly {421, 424, 431, 444, 448, 451, 454, 457, 461, 464, 817}.
- `reflex_codes_exit_3`: every one maps to exit 3.
- `reflex_repairs_ranked`: each row has at least one repair, and the first repair of each matches the table.
- `ref_817_names_missing_condition`: with a declared lane and a record whose `state_source = privileged`, the payload's failing condition is `model-privileged`; with no record it is `no-record`.
- `ref_451_two_triggers`: an unlisted spring gives the "omits a source" form; a `Certified` request on an `assumed` absorber gives the class form; the payloads differ.
- `ref_454_two_triggers`: an empty intersection and a five-row coupled set both fire, with different pointers.
- `unknown_code_is_load_error`: a row citing `FROB-REF-499` from a verb entry with no registry row fails the load.

### End-to-end script and logging
`scripts/e2e/reflex_refusals.sh` drives the real `frob` binary once per code with its smallest triggering input
(listed below), then once with the trigger removed. Per STD-E2E it logs for each step the command, the exit code,
the envelope's `refusal` block, the `evidence` block, and an `assert` line per checked field; the final `summary`
counts codes fired, controls passed and names any code whose mechanism is not yet built at the current milestone
(reported as `not run`, never as a pass).

### Known answers and negative controls
Smallest triggers, with values:
- 421: a legged design with one leg, no admitted fallback. Control: admit the sampling controller, outcome `not-synthesizable` without a refusal.
- 424: the friction fixture as a run-time program (weight 50 N, push 30 N, μ = 0.5) with no fallback admitted; the certificate y = (1/2, −1) with yᵀb = −5 N is in the payload. Control: a fallback admitted gives a `fallback` event, no refusal.
- 431: the rimless wheel at the slope where the fixed point disappears; a ball whose apex grazes a ceiling.
- 444: the {stable, fall} target (KA-566, CX-242).
- 448: the two-link arm, links 0.4 m and 0.3 m at angles 0 and π/2, limit 2.5 N·m, required (10, 0) N: support 25/3 N, binding joint 2 (KA-565).
- 451: the worked energy card with an unlisted series spring (KA-568).
- 454: a box [0, 1]² with rows u₁ + u₂ ≥ 2.5 (empty) ; a five-row coupled set.
- 457: a margin calibrated for policy P reused with policy P′ (KA-573's control).
- 461: tying left and right gains on a side slope (KA-575).
- 464: the 2 m estimate of a 1 m motion scored with a fitted scale (KA-554, CX-234).
- 817: a planted acceptance request over `mpc-sample` episodes with no frozen record (KA-558's control, CX-245).

## Considerations

- Two codes have two triggers each (451, 454) and one has a conditional exemption (817). Tests must cover each
  branch; a payload that does not say which branch fired is a defect.
- FROB-REF-421 is distinct from the outcome `not-synthesizable`: the outcome is a result of `control.synth` (the
  verb succeeded in saying that no route exists); the refusal fires only when a caller needs a controller and has
  admitted no fallback. Conflating them would make a healthy answer look like an error to an agent.
- Grazing in the general derivative machinery has its own code in the dynamics chapter (FROB-REF-263); 431 is the
  gait-level refusal and carries the one-sided estimates. A gait request refused below by 263 is re-raised as 431
  with the lower refusal attached.
- The four worded refusals have no number in the plan's registry. Allocation is a registry action; inventing
  numbers in separate beads would risk a collision.

## References

§22.14, Appendix O (O.4 and the evidence family), §32.4 (envelope and exit codes), §35 (refusal form); worded
refusals from §22.20, §22.21, §22.23, §22.25; lane conditions from §22.22. INV-REFLEX-03, INV-REFLEX-04.
