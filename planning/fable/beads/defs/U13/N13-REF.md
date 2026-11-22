---
key: "N13-REF"
title: "Refusal registry for Marrow: FROB-REF-212 to -215 with triggers, payloads, ranked repairs and exit codes"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "marrow", "frob-lie", "frob-algebra", "solid", "registry"]
parent: "WP-03"
deps: ["BD-005"]
needs: ["§35.11 and Appendix O: the owner of `registries/refusals.toml` assigns a code to the refusal 'lifted law on a joint without turn state' (§13.11), which is stated without a code"]
related: ["BD-011", "BD-048", "N13-10", "N13-11", "N13-13", "N13-14", "N13-15", "N13-34", "N13-04", "BD-022"]
gates: []
plan_refs: ["§13.9", "Appendix O.2", "§35.11", "§32.4", "§13.11"]
---
## Background
A refusal has a stable code, a message and ranked repairs. The standing rules: refuse, never truncate; a budget stop publishes nothing partial as a result; a misspelt parameter is a refusal, not a default. Families: `1xx` design and model; `2xx` numerics and integrator admission; `3xx` scene; `4xx` control and estimation; `5xx` learning and evaluation protocol; `6xx` backend and device; `7xx` budget and capability; `8xx` evidence and protocol; `9xx` internal. The machine-readable registry is `registries/refusals.toml`; an unknown code is a load-time error; codes are never reused. On the command line a refusal is an envelope with `ok: false` and a `refusal` object (`code`, `message`, `pointer`, `observed`, `required`, `unit`, `affected_claims`, `retry` ∈ {`never`, `after-change`, `backoff`}, ranked `repairs`); refused input or question exits with code 3.

The algebraic core defines four codes, all in the `2xx` family: 212, 213, 214, 215. This bead enters them in the registry with everything a caller needs, and lists the other refusals that Marrow's subsections state or raise so that none is lost. Tier of the content: Solid.

## Scope
In: the four registry records; the record for the uncoded turn-state refusal once a code is assigned; the cross-reference list; the tests that each code is raised with its payload and exit code.
Out: the mechanisms that raise them (owners listed); codes defined by other chapters.

## Technical approach
**The four codes** (exit code 3 for each; `retry: after-change`). "Returned" is what the chapter requires to travel with the refusal; repairs are ranked. Repairs for 212 are stated by the dynamics chapter's refusal table; those for 213 to 215 are DESIGN, derived from the reason each refusal exists.

| Code | Refused (meaning and trigger) | Returned with the refusal | Ranked repairs | Raised by |
|---|---|---|---|---|
| FROB-REF-212 | A Cayley update with ‖ω‖·h above 0.2 in one substep. Trigger: the guarded predicate (ω·ω)·h² ≤ κ is false, κ the stored constant nearest 0.04. Reason: the Cayley step rotates by 2·atan(‖ω‖h/2) instead of ‖ω‖h (relative shortfall (‖ω‖h)²/12; 0.33% at the bound), and the bound also guarantees at most one turn-cut crossing per substep | The substep count that admits it, ⌈‖ω‖·h/0.2⌉ | 1. substep by the returned count (the step planner does this automatically); 2. use the exponential on a tier that has it | `frob-lie` `cay_step` (BD-011); the guarded kernel form (N13-11) |
| FROB-REF-213 | A logarithm or its derivative inside the stated margin of a chart cut, with no second chart supplied. Trigger: `log`, `cay_inv` or an inverse differential requested within the margin of a rotation by π (quaternion scalar part within the margin of zero). No value is returned from the wrong side of a cut | The element, the margin and the charts that cover it | 1. supply the covering chart named in the reply (the chart based at the rotation by π about the axis of the largest quaternion component keeps the angle at most 2π/3); 2. reformulate the query on the group element instead of its logarithm; 3. if the request came from a derivative path, withdraw that chart pair from the derivative path and refuse the query | `frob-lie` (BD-011, BD-045 by component index for products) |
| FROB-REF-214 | A representation with a constituent of quaternionic type, or a symmetry group outside the supported families. Triggers: a Frobenius–Schur indicator of −1 for a constituent; a group whose closure exceeds 256 elements; an n-fold rotation with n above 12 on the exact-arithmetic route. Reason: no group that arises here has a quaternionic irreducible, so meeting one means the group computation is wrong | The character and its indicator; the group's order and generators | 1. if the group came from a design, re-run the symmetry analysis and report the group computation as defective (`retry: never` for the same input); 2. restrict to a supported subgroup named in the reply; 3. declare the symmetry by construction (`radial`), or take the tolerance route labelled `Tested`; 4. use augmentation without tying | `frob-algebra` (BD-048, N13-14, N13-15, BD-325) |
| FROB-REF-215 | An exact evaluation whose bit budget exceeds its cap: a guarded predicate over 256 bits at build time, or a host computation over its declared rational size | The expression, the input ranges and the budget | 1. rescale the expression (the cap is a signal to rescale it); 2. narrow the declared input ranges; 3. split the predicate into factors of lower degree; 4. for a host computation, declare a larger size with its reason in the contract, or take the tolerance route labelled `Tested` | `frob-lie` exact kernel and predicate specifications (N13-10, N13-11); `frob-algebra` field arithmetic (N13-13); `frob-kir` at build time (BD-053) |

**Registry record** (per code, in `registries/refusals.toml`): `code`, `name` (`cayley-bound`, `chart-cut`, `unsupported-representation`, `exact-budget-exceeded`), `family = "2xx"`, `message` template, `returned` (field names and units: for 212 `substeps: integer`, `omega_h: rad`; for 213 `element`, `margin: rad`, `covering_charts`; for 214 `character`, `indicator`, `order`, `generators`; for 215 `expression`, `ranges`, `budget_bits`, `cap_bits`), `repairs` (ordered, each a typed suggestion), `exit = 3`, `retry`, `defined_in`, `raised_by`.
**The refusal stated without a code.** "A lifted law on a joint without turn state" (raised at design-check stage 1 when an element that reads the lifted angle sits on a joint whose state omits the turn count). It belongs to the `1xx` family; the candidate is the next free code there (FROB-REF-127), to be confirmed by the registry's owner. Returned: the joint, the element and its card field, the joint's `turns` declaration. Repairs: 1. track the turn count on the joint; 2. replace the element by a periodic one where the physics allows; 3. remove the element. Raised by the model compiler (N13-34).
**Refusals stated or raised by Marrow's subsections and defined elsewhere** (cross-references; each is registered by its defining chapter):
- FROB-REF-168: a solid body exceeds the mass plausibility bound of its part class and no fill is declared (stated with the inertia convention; listed in the proofs chapter's refusal table; rule decided in N13-03, raised by N13-04).
- FROB-REF-106: declared mass properties violate the mass-support bound (raised by the inertia cone's predicate, BD-051; fixture in N13-04).
- FROB-REF-104 and FROB-REF-105: inertia not physically consistent; a body without a fill or a solid written as a shell.
- FROB-REF-109: declared symmetry differs from computed symmetry (the compiler compares a declaration with the groups computed by N13-15 and N13-16).
- FROB-REF-252: a strict predicate in floating point without an exact form or a certified guard, at build time (the lint side of the predicate classes).

## Acceptance criteria
- [ ] `registries/refusals.toml` holds the four records with every field; the registry loads; an unknown code is a load-time error (tested by loading a file with FROB-REF-216 undefined and observing the error).
- [ ] Each code is raised by its owner with the returned payload and exit code 3 through `frob --json` where a verb reaches it: 214 and 215 through `frob algebra irreps`; 212 and 213 through a verb that steps or takes a logarithm when one exists, and through library tests before that.
- [ ] Each record's repairs are present in the envelope's `refusal.repairs` in the stated order.
- [ ] The turn-state refusal has a registered code (the candidate or the owner's choice) before N13-34 closes.
- [ ] The count for this chapter is 4 codes (212, 213, 214, 215), matching the refusal appendix.
- [ ] Negative control: a build in which the Cayley bound silently clamps ‖ω‖·h to 0.2 instead of refusing is caught by the test that expects FROB-REF-212 (refuse, never truncate).

## Test plan
### Unit tests
- `refusal_registry_has_212_to_215`: the four records parse with all fields.
- `unknown_code_is_load_error`.
- `ref_212_payload`: ‖ω‖·h = 0.25 → code 212, `substeps` 2; ‖ω‖·h = 0.61 → `substeps` 4.
- `ref_213_payload`: a rotation by π − 2⁻²¹ rad → code 213 with the margin and a covering chart; with the chart supplied the call succeeds.
- `ref_214_payload_quaternionic`: the regular representation of the quaternion group → code 214 with the degree-two character and indicator −1; `ref_214_payload_unsupported`: closure beyond 256 elements → code 214 with the order reached and the generators.
- `ref_215_payload`: a predicate with a 279-bit budget → code 215 with expression, ranges, 279 and 256; a host computation forced over its declared size → code 215.
- `refusal_envelope_shape`: `ok: false`, exit 3, `retry`, ordered `repairs`.
### End-to-end script and logging
`scripts/e2e/marrow_refusals.sh`: `frob algebra irreps` with the quaternion-group generators (214), with an oversize group (214), with a request forced over its size budget (215); when stepping verbs exist, a `frob sim run` on a fixture spun fast enough to violate the Cayley bound with substepping disabled (212). Asserts exit code 3 and the payload fields. `STD-E2E` NDJSON at `target/e2e-logs/marrow_refusals/<utc>.ndjson` with a transcript: each `cmd` with exit code and wall time; `refusal` events with code, message, pointer, observed, required, unit, retry and the repairs; the envelope evidence block; on failure the saved envelope path; a summary with refusals observed against expected.
### Known answers and negative controls
Substep counts ⌈0.25/0.2⌉ = 2 and ⌈0.61/0.2⌉ = 4; the 256-bit cap; the 256-element cap. Control: the clamping build. Determinism: refusals are deterministic functions of their inputs; identical envelopes on every platform.

## Considerations
- Codes are never reused and are assigned by the registry's owner; this bead proposes names and payload schemas for the four codes the chapter defines and flags the one refusal the chapter states without a code.
- The repairs for 213 to 215 are DESIGN and may be refined by the surfaces work; the returned payloads are normative.
- A refusal is a result an agent acts on; each message names the quantity, its unit and the bound, never only the code.

## References
§13.9 (the four refusals and what is returned with each), Appendix O.2, §35.11 (families and standing rules), §32.4 (the envelope and exit codes), §16.14 (repairs of 212), §13.11 (the uncoded refusal).
