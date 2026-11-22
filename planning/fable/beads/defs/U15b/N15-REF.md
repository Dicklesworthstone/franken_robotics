---
key: "N15-REF"
title: "registry: Sinew refusal codes FROB-REF-130 to 136 with triggers, returned data, ranked repairs and exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "sinew", "frob-actuate", "solid", "registry", "test"]
parent: "WP-09"
deps: ["BD-005", "STD-TEST"]
needs: []
related: ["BD-069", "BD-072", "BD-073", "BD-074", "BD-075", "BD-103", "BD-110", "BD-137", "N15-CX", "N15b-38"]
gates: []
plan_refs: ["§15.13", "§14.13", "Appendix O", "§32.4", "§32.5"]
---
## Background

Sinew is the subsystem for actuators, transmissions, batteries, thermal limits and power (crate `frob-actuate`).
The program's rule is that an invalid design or request is refused with a stable code, a message and ranked
repairs expressed as typed edits; nothing is repaired in silence. Codes 101 to 129 belong to the design
language; **130 to 139 are reserved for Sinew**, of which 130 to 136 are assigned and 137 to 139 are free.
Codes are never reused. The machine-readable registry is `registries/refusals.toml`, loaded by `frob-core`
(seed bead BD-005 owns the refusal type and the loader); an unknown code is a load-time error, so a mechanism
bead cannot raise one of these codes until its row exists. That is why the mechanism beads of this chapter
depend on this bead.

This bead is the single place where the seven Sinew codes are written out in full, so that no implementer has
to reconstruct a trigger or a repair from prose. Every row of the table is DESIGN where it adds a repair
ranking or a pointer layout to the chapter's two columns ("refused" and "returned with the refusal"); the
meanings themselves are fixed.

## Scope

In: the seven rows of `registries/refusals.toml`; the payload each refusal returns; the ranked repairs; the
exit code and failure class; the stage or verb that raises each; one rejection fixture per code. The
end-to-end script that triggers each code through the CLI is bead N15b-38.

Out: the mechanisms that detect the conditions (card checks in BD-069; envelope in BD-073; thermal in BD-072;
bus in BD-075; transmissions in BD-074; rows in BD-084 and BD-155; stage 6 in BD-103; aggregation in BD-110).
Codes of other chapters that Sinew raises are listed at the end for orientation only: their rows belong to
their owners.

## Technical approach

**Envelope form.** A refusal is the `refusal` object of the verb envelope `frob.envelope/1` with `ok: false`:
`code`, `message`, `pointer` (the field or part at fault), `observed`, `required`, `unit`, `affected_claims`,
`retry` (`never`, `after-change` or `backoff`) and ranked `repairs` (typed edits that also appear as
`next_actions`). All seven Sinew codes are Domain failures with **exit code 3** and `retry: after-change`.
On MCP the same envelope is the structured content of a tool result with `isError: true`.

| Code | Meaning | Trigger and where it is raised | Returned with the refusal | Ranked repairs (DESIGN ranking) |
|---|---|---|---|---|
| FROB-REF-130 | A card or transmission that is not passive | Loss negative in some quadrant; an efficiency above one; torque and back-EMF constants inconsistent in power-invariant coordinates beyond the card's stated uncertainty. Raised by `actuator.check` at card admission and again when the card is bound to a joint; by stage 1 of `design.check`; by the transmission check | The quadrant (sign of torque × sign of speed, motor side) and the power created in watts at the witness operating point | 1. Replace the single efficiency by a pair `efficiency_forward`, `efficiency_back`, each at most one, or by a loss law. 2. Reconcile `k_t` and `k_e` (a mismatch by √(3/2), √2 or √3 names the missing current convention). 3. Correct the sign of a braking-mode loss |
| FROB-REF-131 | A required operating point outside the torque, speed and voltage envelope | A standing, crouching, stepping-up or nominal-gait point of A-11 lies outside the card's envelope at the stated winding temperature and bus voltage. Raised at stage 6 and by `actuator.envelope` when asked about a point | The point (torque, speed), the envelope boundary at that speed, and the gear-ratio or card edit that admits it | 1. `regear` to the ratio that admits the point. 2. `swap-actuator` to the smallest catalog card that admits it. 3. Raise the bus nominal voltage. 4. Evaluate at L2 if the card carries the L1 reactive-drop flag |
| FROB-REF-132 | No stable thermal equilibrium at a required operating point, or an equilibrium above a limit | The temperature-dependent steady state (loss curve against the removal line) has no stable solution, or its temperature exceeds `t_max_winding` or `t_max_magnet`. Raised at stage 6 and by `actuator.thermal` | The equilibrium temperature, or the margin to the tangency when none exists; the largest sustainable load | 1. `swap-actuator` to a card with a higher continuous rating. 2. `regear` so that the same joint torque needs less current. 3. Raise the sink conductance. 4. Reduce payload or duty to the largest sustainable load |
| FROB-REF-133 | Peak demand above the bus ceiling E²/(4·R) | The peak electrical demand of the nominal gait, taken from the time-resolved profile, exceeds the ceiling at end-of-discharge voltage and cold-temperature resistance. Raised at stage 6 and by `actuator.bus` | The ceiling, the demand, and the battery or harness edit that admits it | 1. Lower `harness_resistance`. 2. A battery card with lower R₀ or a higher voltage. 3. Stagger the peaks (the joint schedule of A-23). 4. Reduce the peak demand |
| FROB-REF-134 | A card without a current convention, or with conventions that disagree | `current_convention` absent, or `current_convention`, `pole_pairs`, `modulation` and the quoted constants imply different conventions. Raised at card admission and stage 1 | The fields in conflict | 1. Write `current_convention` (`dc`, `phase-peak`, `phase-rms` or `q-axis`). 2. Convert the quoted constant with the factor for the stated convention |
| FROB-REF-135 | Power requested from an exhausted battery, or state of charge driven outside its bounds | A step or a caller asks for power when the state of charge is at its lower bound, or a charge request would pass the upper bound. Raised at run time on every tier; the kernel never clips | The step and the energy short in joules | 1. A battery card with more capacity. 2. A shorter mission or lower demand. 3. A higher starting state of charge |
| FROB-REF-136 | A row option the requested tier does not carry | A stiction or backlash row option (for example the Gaussian Stribeck shape, hard backlash rows, a restitution value) is requested on a tier that does not implement it. Raised when the step plan is built | The tiers that carry it | 1. Run on a listed tier (`referee` or `strict`). 2. Change the option to one the tier carries (the rational Stribeck shape). 3. Raise the card level |

**Registry row layout (DESIGN).** One TOML table per code: `code`, `owner = "frob-actuate"`, `chapter`,
`message` (a template with named slots), `payload` (the typed fields of the fourth column, each with its
unit), `repairs` (edit names in rank order), `exit = 3`, `retry = "after-change"`, `first_milestone`, and
`fixtures` (the rejection fixtures below). The `registries/verbs.toml` entries of the verbs that can raise a
code list it in their `refusals` array; a lint in `frob-verbs` fails the build when a verb raises a code it
does not list.

**Run-time refusals inside a batch.** FROB-REF-135 raised inside a simulated world is recorded as an event of
that world (`brownout` or exhaustion, failure taxonomy kind `physical`), and the run completes with findings
(exit 2). It is exit 3 only when the verb's question itself cannot be answered.

**Codes Sinew raises that other chapters own:** FROB-REF-108 (power sign mismatch; the direction witness),
FROB-REF-113 (an `assumed` card field under `--strict-data`), FROB-REF-114 (module digest or port signature
mismatch), FROB-REF-115 (budget exceeded), FROB-REF-116 (co-design does not close), FROB-REF-241 (backlash rows
requested at a card level with no rotor state).

## Acceptance criteria

- [ ] `registries/refusals.toml` holds rows 130 to 136 with every field above; `frob-core`'s loader accepts
  the file and rejects a file with one row removed whenever a registered verb lists that code.
- [ ] Codes 137, 138 and 139 are marked `reserved = "sinew"` and loading a verb that raises one fails.
- [ ] Each payload has a typed schema with units (watts, volts, kelvin, amperes, joules, tier names), and
  `frob schema frob.envelope/1` documents the payload fields of the seven codes.
- [ ] Each row names its rejection fixtures, and the codes raised in stages 1 to 6 (130, 131, 132, 133,
  134) each name at least two seeded defects for the 100-defect battery of the preflight (the rule "at
  least two per refusal code").
- [ ] The registry lint fails when a verb raises a Sinew code that its `refusals` array does not list.
- [ ] The behaviour of the seven refusals through the CLI is measured by bead N15b-38, which depends on this
  bead and on the mechanisms; this bead closes on the registry and its loader tests alone.

## Test plan

### Unit tests
In `crates/frob-core/tests/refusal_registry.rs`, real registry file, no mocks.
- `registry_has_sinew_codes`: loads the shipped file; asserts codes 130..=136 exist with exit 3,
  `retry = "after-change"`, a non-empty message template and a non-empty repairs list.
- `unknown_code_is_load_error`: a copy of the registry without 133 plus a verb entry that lists 133 fails to
  load.
- `reserved_codes_unusable`: a verb entry listing 137 fails to load.
- `payload_schemas_typed`: each payload field has a unit or an enumerated type; for 130 the fields are
  `quadrant` and `power_created_w`; for 131 `point`, `boundary`, `admitting_edit`; for 132 `equilibrium_k`
  or `tangency_margin`, `largest_sustainable_load`; for 133 `ceiling_w`, `demand_w`, `admitting_edit`; for
  134 `fields_in_conflict`; for 135 `step`, `energy_short_j`; for 136 `tiers`.
- `lint_catches_unlisted_code`: a synthetic verb entry that raises 132 without listing it fails the lint.
- `fixtures_named`: every row's `fixtures` entries resolve to files under
  `assets/fixtures/sinew/refusals/`.

### End-to-end script and logging
`scripts/e2e/sinew_refusals.sh` is written by bead N15b-38. It triggers each of the seven codes through the
real `frob` binary on a `synthetic` fixture, asserts exit 3 and the payload, checks that the first repair
removes the refusal, and logs one NDJSON `refusal` event per code with the command, exit code, wall time,
evidence block and payload to `target/e2e-logs/sinew_refusals/<utc-timestamp>.ndjson`.

### Known answers and negative controls
The payload values that the fixtures must reproduce are stated here so that the registry and the tests
agree: KA-320 (2.5 W created by the single-multiplier gear, quadrant back-driven), KA-313 (the 20 A design
refused under 132 with equilibrium 190 °C), KA-325 (133 at stage 6; ceiling 7,200 W for E = 48 V and
R = 0.08 Ω), KA-326 (135), KA-280 (131). Negative controls: a registry mutant that maps 133 to exit 2 must
fail `registry_has_sinew_codes`; a registry with a payload field lacking a unit must fail
`payload_schemas_typed`. Determinism: the registry loads to identical bytes twice and in debug and release
(class D0).

## Considerations

- A repair ranking is advice. It is DESIGN and is revisited when the co-design cascade measures which repair
  agents actually accept.
- A refusal never weakens its inputs to proceed; the airlock rule that a sibling refusal is not retried with
  weakened inputs applies to card checks as well.
- Privacy: payloads echo card field paths and numbers, never free text from an imported file without the
  untrusted mark.

## References

Plan §15.13 (the refusal table), §14.13, Appendix O, §32.4 (exit codes), §32.5 (failure classes);
INV-SINEW-01, INV-SINEW-04.
