---
key: "N20-REF"
title: "registry: the Sensorium refusal codes FROB-REF-440 to -447, with triggers, repairs, exit codes and tests"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "sensorium", "frob-sense", "frob-core", "solid", "registry", "test"]
parent: "WP-40"
deps: ["BD-005", "BD-022"]
needs: ["Appendix O owner: two refusals that late subsections of this chapter state without a code (a peak-load or sensor-overload claim without a resolved or modeled force history; a stage-7 target whose lead is shorter than the declared latency chain), and one rule with no code (evaluation of a policy with no passing causal-policy receipt): codes to be assigned in `registries/refusals.toml`"]
related: ["BD-078", "BD-098", "BD-215", "N20b-10", "N20b-02", "BD-240", "N20b-05", "N20b-18", "N20b-17", "N20b-29", "N20b-33", "N20b-12"]
gates: []
plan_refs: ["§20.18", "Appendix O", "§35.11", "§32.4", "§20.23", "§20.24"]
---
## Background
A refusal in this system has a stable code, a message and ranked repairs. Three standing rules: refuse, never truncate; a budget stop publishes nothing partial as a result; a misspelt parameter is a refusal, not a default. Codes are never reused; the machine-readable registry is `registries/refusals.toml` and an unknown code is a load-time error. Families: `1xx` design and model; `2xx` numerics and integrator admission; `3xx` scene; `4xx` control and estimation; `5xx` learning and evaluation protocol; `6xx` backend and device; `7xx` budget and capability; `8xx` evidence and protocol; `9xx` internal. Every refusal that reaches a caller travels in the standard envelope with `ok: false` and a `refusal` object: `code`, `message`, `pointer` (the field or part at fault), `observed`, `required`, `unit`, `affected_claims`, `retry` (`never`, `after-change`, `backoff`) and ranked `repairs`. A refused input or question in families `1xx` to `5xx` exits with code 3 (Domain failure class); over MCP it is a tool result with `isError: true` whose structured content is the same envelope.

The Sensorium chapter defines seven codes, FROB-REF-440, -441, -442, -443, -445, -446 and -447 (FROB-REF-444 belongs to the control chapter: a transported skill does not refine its specification). This bead is the chapter's refusal registry: it carries each code with its meaning, trigger, ranked repairs and exit code, names the bead whose mechanism raises it, and owns the registry rows and the test that every code can be triggered and is raised by nothing else. The chapter governs the meaning and the repairs; the consolidated appendix is compiled from it.

## Scope
In: the seven registry rows; the trigger fixtures; a conformance test that triggers each code from the user's side and checks the envelope; the list of code-less refusals found in the chapter's late subsections, with their fail-closed interim handling.
Out: the mechanisms that raise the codes (each named below).

## Technical approach
**The registry (to be written into `registries/refusals.toml`, family `4xx`, defined in the Sensorium chapter).**

| Code | Meaning (message) | Trigger, precisely | Ranked repairs | Exit | Retry | Raised by |
|---|---|---|---|---|---|---|
| FROB-REF-440 | The clock lattice is not admissible: a period is zero or negative, or the common denominator exceeds the limit | At admission of a run, task or design: any clock (physics step, sensor, link, estimator, controller, actuator channel) has a period ≤ 0; or Q, the least common multiple of the denominators of all periods (including drifting clocks' periods T·(1 + ε) with ε rational), exceeds 2⁴⁰ (TARGET limit; at the limit a run of 10⁶ s stays below 2⁶⁰ quanta) | 1. Round the offending rate to a declared rational (the refusal names the clock and proposes the nearest rationals from the declared menu) | 3 | after-change | The clock contract (BD-078); surfaced by `sense.schedule` and `sense.profile` |
| FROB-REF-441 | History buffers cannot cover the declared latency and exposure window | At admission: for some world-and-sensor packet ring, the capacity implied by the cards (the longest latency plus the longest exposure, divided by the period, plus the reordering window) exceeds what the declared queue capacity or the tier's history buffers provide; a queue never overflows silently | 1. Raise the queue capacity. 2. Shorten the latency bound. 3. Lower the rate | 3 | after-change | Packet queues (BD-098, BD-189); surfaced by `sense.profile` |
| FROB-REF-442 | An observation is requested from presentation state, or an actor schema names a privileged channel | (a) A task, wrapper or consumer asks the observation service for an observation whose source is the viewer, a still, a presentation timestamp or level-of-detail state. (b) An actor's inference schema names a channel in the truth namespace or with access class other than `actor` | 1. Use the observation service (name the delivered-packet channel). 2. Move the channel to the training-only schema | 3 | after-change | (a) BD-215; (b) N20b-10 (`freeze_actor_schema`) |
| FROB-REF-443 | A symmetry consumer names a group element outside Γ_obs | A request for equivariant weight sharing, augmentation, deduplication of closed-loop results or a gait-orbit quotient names an element of the mechanical group that is not in the observation-preserving subgroup stored for the current (design, sensor profile, task, scene family), or presents a stale subgroup | 1. Use Γ_obs (the licensed elements are listed). 2. Change the sensor suite (the typed edit that restores the element). 3. Restrict to open-loop use | 3 | after-change | N20b-02 |
| FROB-REF-445 | A training or optimization run is requested on a mission with a standing alias witness for its interface | A `run` verb that trains or optimizes is asked to start on a mission for which an accepted alias witness exists under the same observation interface (two admitted worlds with identical delivered transcripts up to a commitment time and disjoint sets of succeeding continuations), and the caller did not override for diagnosis | 1. Read the witness (attached). 2. Apply a separating change (a sensor, a retrofit, a body change, or information by commitment, each priced). 3. Override for diagnosis (the run is then labelled diagnostic and cannot be cited as evidence) | 3 | after-change | The alias-witness beads (BD-134, BD-240) and the training and optimization verbs that check for a standing witness |
| FROB-REF-446 | A calibration request exceeds what the graph supports: no anchor, or fault localization on a graph without the needed cycles | (a) The pose graph has no anchored node, or a component not connected to the anchor (gauge). (b) Fault localization is requested on a tree (the consistency residual is identically zero), or bad-edge identification on a graph where the edge is not identifiable (a single cycle) | 1. Anchor the base (or name a sensor coincident with it). 2. Add pairwise measurements (the report lists which edges would make each edge identifiable) | 3 | after-change | N20b-04 (anchor), N20b-05 (localization), BD-242 (verb) |
| FROB-REF-447 | A Reality claim rests on a sensor or optical card field of class `assumed` | A result is requested with, or is about to be given, a Reality claim, and its sensing dependency set contains a sensor-card or optical-card field (or a latent's conditional, or an optical volume parameter) whose provenance is `assumed` | 1. Record a datasheet or a measurement for the listed fields. 2. Drop the claim (the result is returned with `reality: "no-claim"`) | 3 | after-change | N20b-18; also N20b-08 on imported logs |

**Refusals stated in the chapter without a code (recorded so they are not lost; fail-closed until codes exist).**
| Stated in | Refusal | Interim handling |
|---|---|---|
| The force-history subsection | A peak-load claim without a resolved or modeled force history | The requirement atom evaluates to `undetermined` (never `satisfied`), the reading's `overload` field is `unknown`, and a verb asked to assert the claim refuses under a reserved registry slot `sens-peak-load-ungraded` (N20b-29) |
| The visibility subsection | At stage 7 of `design check`, a target with a lead shorter than the declared latency chain | Refused at stage 7 under a reserved slot `sens-lead-too-short`, with ranked repairs as typed edits (raise or tilt the mount, move the payload, shorten the latency) (N20b-33) |
| The causal-policy subsection (rule, not a table row) | Evaluation of a policy with no passing causal-policy receipt for its interface | Evaluation is refused under a reserved slot `causal-receipt-missing` (N20b-12) |
Reserved slots are registered names without a final number; the registry owner assigns numbers (in `4xx` for the first two, `5xx` for the third) before the M2 and M3 verbs list them. Their existence is reported to the lead in the unit's coverage file.

**Conformance test.** `crates/frob-conformance/tests/g1.rs` rows `refusal_sens_44x`: for each code a minimal fixture that triggers it through the real verb, asserting the code, the family, exit 3, `retry`, that `repairs` are present in the stated order, and that `pointer` names the offending field. A second test (`refusal_sens_exclusive`) runs the chapter's known-answer battery and asserts that none of the seven codes appears on a passing case. The fixtures: (440) a design with a sensor rate whose period makes Q = 2⁴¹; a clock with period 0; (441) a link latency bound of 2 s on a 200 Hz sensor with a queue capacity of 8; (442a) a task file naming a viewer buffer as an observation; (442b) an actor schema naming `truth.base_velocity`; (443) augmentation under the fore-aft mirror on a forward-camera quadruped; (445) `train start` on the T-junction mission with its witness on file; (446a) a rig with no anchor; (446b) localization on a tree rig; (447) a Reality claim over default cards.

## Acceptance criteria
- [ ] `registries/refusals.toml` holds the seven rows with message, family, defining chapter and repairs; the loader accepts them and rejects an unknown `FROB-REF-449` in a verb's list (load-time error).
- [ ] Each code is triggered by its fixture through `frob --json` and through MCP, with exit 3 and the repairs in order.
- [ ] No code is raised on any passing case of the chapter's battery.
- [ ] Every code maps to exactly one exit code (the surface chapter's parity requirement for this family).
- [ ] The three reserved slots are registered, listed in the output of `frob schema --refusals`, and flagged `number: pending`.
- [ ] Each mechanism bead named in the table lists this registry as the source of its code's message and repairs (no bead carries a divergent message).
- [ ] Negative control: a build in which FROB-REF-441 is replaced by silently dropping the oldest packet passes no test that expects the refusal (the fixture detects truncation), confirming "refuse, never truncate" is tested.

## Test plan
### Unit tests
(`cargo test -p frob-core refusals`, `cargo test -p frob-conformance --test g1 -- refusal_sens`)
- `seven_codes_registered_family_4xx`.
- `unknown_code_is_load_time_error`.
- `each_code_has_ordered_repairs`.
- `trigger_440_q_above_limit`, `trigger_440_zero_period`, `trigger_441_history`, `trigger_442_presentation`, `trigger_442_privileged`, `trigger_443_outside_obs`, `trigger_445_standing_witness`, `trigger_446_no_anchor`, `trigger_446_tree_localize`, `trigger_447_assumed_field`.
- `codes_not_raised_on_passing_battery`.
- `reserved_slots_pending`.
### End-to-end script and logging
`scripts/e2e/sense_refusals.sh` (sources the STD-E2E helper): runs the ten trigger fixtures on the CLI and through the MCP server; asserts code, exit 3, repairs and surface parity; logs to `target/e2e-logs/sense_refusals/<utc>.ndjson` with a transcript: each `cmd` with its exact command line, exit code and wall time; a `refusal` line per trigger with code, pointer, observed, required and repairs; an `assert` per check; a summary with the number of codes exercised (7 of 7) and the reserved slots seen.
### Known answers and negative controls
KA-474 (a lattice with common denominator above 2⁴⁰ is refused with FROB-REF-440; at the limit a run of 10⁶ s stays below 2⁶⁰ quanta), KA-496's control (a tree refuses localization, FROB-REF-446), CX-206 (the T-junction mission behind FROB-REF-445), CX-209 (behind FROB-REF-443). Control: the silent-truncation build. Determinism: refusals are functions of the request; identical envelopes across platforms.

## Considerations
- Milestones: the codes arrive with their mechanisms (440 and 441 at M1, 442 at M1 and M2, 443, 446 and 447 at M3, 445 at M3); the registry rows are written at M1 so that verbs can list them from the start, and a trigger test is enabled when its mechanism lands.
- A code's repairs are ranked: the first is the one the chapter expects to be right most often; agents read them in order.
- FROB-REF-445 has an override for diagnosis; an overridden run is labelled and its results cannot be cited, which must be visible in the evidence block.

## References
§20.18 (the refusal table), the consolidated refusal registry (rows 440 to 447), the refusal families and rules of the numerics chapter, the envelope and exit codes of the surface chapter; KA-474; CX-206, CX-209.
