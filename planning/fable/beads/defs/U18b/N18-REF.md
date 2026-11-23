---
key: "N18-REF"
title: "frob-contact: Traction refusal registry: FROB-REF-240 to -246, -320, -321, -822 with repairs and exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "traction", "frob-contact", "registry", "test"]
parent: "WP-10"
deps: ["BD-005", "BD-022", "N32a-03"]
needs: ["§32.4: the envelope's refusal block and the exit-code table (verb-surface unit); every code here maps to exactly one exit code"]
related: ["N18-CX", "N18b-28", "N18b-26", "N18b-27", "N18b-22", "N18b-12"]
gates: []
plan_refs: ["§18.14", "Appendix O", "§18.19", "§18.20", "§32.4", "§35.11"]
---
## Background

A refusal is a typed answer, not an error. Every refusal has a stable code, a message and ranked repairs, and
three standing rules apply everywhere: refuse, never truncate; a budget stop publishes nothing partial as a
result; a misspelt parameter is a refusal, not a default. Code families: `1xx` design and model; `2xx` numerics
and integrator admission; `3xx` scene; `4xx` control and estimation; `5xx` learning and evaluation protocol;
`6xx` backend and device; `7xx` budget and capability; `8xx` evidence and protocol; `9xx` internal. The
machine-readable registry is `registries/refusals.toml`; an unknown code is a load-time error and codes are
never reused.

The contact subsystem defines ten codes. This bead is their registry entry and their test: each code with its
name, the condition that raises it, the repairs in rank order, the exit code, the retry class and the bead whose
mechanism raises it. It exists so that no code is lost between the mechanism beads and so that an agent that
receives one of them always gets the same structured remedy.

## Scope

In: the ten registry entries; the refusal-block contents for each; one planted trigger per code; the list of
refusals the chapter names without a code. Out: the mechanisms that raise them (named per row).

## Technical approach

**Envelope form.** With `--json` a refused request returns one envelope with `ok: false` and
`refusal: { code, message, pointer, observed, required, unit, affected_claims, retry, repairs[] }`. `pointer` is
the field or part at fault; `retry` is `never`, `after-change` or `backoff`; `repairs` are ranked and, where
possible, typed edits or exact commands.

**Exit codes.** 3 for a refused input or question (`1xx` to `5xx`, and `8xx` other than conflicts); 5 for a
capability not implemented, not admitted, or outside the session's scopes (`72x`, `73x`). All ten codes of this
registry map to exit 3. On the MCP surface the same envelope is the structured content of a tool result with
`isError: true`.

**The registry.**

| Code | Name | Raised when | Ranked repairs | Exit | Retry | Raised by |
|---|---|---|---|---|---|---|
| FROB-REF-240 | Invalid cone | μ < 0, r_eff < 0 or a non-positive anisotropy | 1. Fix the card | 3 | after-change | Interface-card load (BD-167); roller wheels (N18b-02); cone kernels (BD-154); `contact.probe` (N18b-29) |
| FROB-REF-241 | No rotor state | Backlash rows requested at a card level without a rotor degree of freedom | 1. Raise the card level (to an actuator level that carries rotor state) | 3 | after-change | Joint rows (BD-084, BD-155) |
| FROB-REF-242 | Film law out of domain | The Hersey number Π or the ratio h/σ lies outside the card's calibrated range under `--strict-data` | 1. Use a calibrated card. 2. Accept `extrapolated` (drop `--strict-data`) | 3 | after-change | Film law (BD-230); `contact.probe` (N18b-29) |
| FROB-REF-243 | Soil or tyre law out of range | Pressure, slip or sinkage outside the card's validity range | 1. Another card. 2. The rigid-floor model | 3 | after-change | Soil laws (N18b-12, N18b-13); tyre laws (N18b-04); `contact.soil` (BD-366) |
| FROB-REF-244 | Stiffness beyond single precision | Estimated scaled κ·u above the admission bound (default 0.05) on an `f32` tier | 1. `batch-cpu` in `f64`. 2. A softer qualified card | 3 | after-change | Stiffness admission (N18b-26) |
| FROB-REF-245 | Half precision | A half-precision solve or state buffer is requested | None; not offered | 3 | never | Precision admission (N18b-27) |
| FROB-REF-246 | Law not admitted | A law requested on a tier or profile where its gate has not passed | 1. The admitted law. 2. `strict` | 3 | after-change | Law-admission table (N18b-28) |
| FROB-REF-320 | Thin feature | Scene geometry thinner than two fine voxels with no thin route | 1. A thin primitive. 2. A finer brick. 3. The mesh path | 3 | after-change | Scene compile and the mesh path (BD-232) |
| FROB-REF-321 | No enclosure | A clearance certificate requested with no registered error bounds for a body or scene | 1. Register bounds. 2. Accept `unknown` | 3 | after-change | Enclosure tests, `contact.clearance` (BD-232, BD-233) |
| FROB-REF-822 | Capacity-biased verdict | A `capacity-biased` comparison is cited as a verdict | 1. Raise capacity. 2. Apply a qualified patch reduction to both arms | 3 | after-change | Capacity-bias rule (N18b-22) |

**Refusals the chapter names without a code.** Three refusals are stated in prose with no entry in the refusal
registry. Each is appended to the registry by the bead that raises it (a new, never-used code in the family
shown; the registry owner confirms the number), and until then is listed here so that it is not lost:

| Refusal | Raised when | Proposed family | Repairs | Raised by |
|---|---|---|---|---|
| Response batch requested against a stale anchor | A response batch is requested against an anchor digest other than the one it was factored for | `2xx` | Refactor for the new anchor; request with the current anchor | N18b-39 |
| Swept bound requested without a speed bound | A transactional profile is requested for a model that supplies neither a joint speed limit nor actuator and load limits | `2xx` | Declare a joint speed limit; declare actuator torque and load limits | N18b-43 |
| Missing interface pair | A (mover material, substrate material, condition) triple that can meet has no interface card | `2xx` | Add a card for the pair and condition; choose a material that has one | BD-167 |

**Codes of other chapters that contact beads raise or cite** (not owned here; meanings carried for
self-containment): FROB-REF-121 (contact law missing or incompatible with its end effector; also a torsion
radius above the foot's own contact radius); FROB-REF-222 (`strict` or `referee` requested on a GPU tier);
FROB-REF-224 (indeterminate contact: no solution, or several distinct post-event motions, the Painlevé rod);
FROB-REF-248 (a solve ill-conditioned beyond its precision profile under the `refuse` policy, the run-time
counterpart of 244); FROB-REF-721 (an island exceeds the referee's contact cap of 21, or the fixed-witness gate
refuses; repairs: patch reduction with its fixture, `--profile strict`; exit 5); FROB-REF-722 (a world exceeds
its contact or row capacity and the caller chose refuse; repairs: raise capacity with a new `StepPlanId`, reset,
continue-flagged; exit 5).

**Registry entry form (DESIGN).**

```toml
[[refusal]]
code = "FROB-REF-244"
name = "Stiffness beyond single precision"
family = "2xx"
exit = 3
retry = "after-change"
defined_in = "frob-contact"
message = "Estimated scaled κ·u {observed} exceeds the admission bound {required} on tier {tier}."
repairs = ["Run on batch-cpu in f64", "Use a softer qualified card"]
```

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the ten entries with name, family, exit code 3, retry class, message
      template and ranked repairs exactly as tabulated; the loader rejects a duplicate or unknown code.
- [ ] For each of the ten codes a planted trigger through `frob --json` returns one envelope with that code, the
      stated exit code and the repairs in rank order; the same through the MCP surface with `isError: true`.
- [ ] Every code maps to exactly one exit code; exit code 1 never appears.
- [ ] `FROB-REF-245` returns an empty repair list and `retry: never`.
- [ ] The three uncoded refusals are either assigned codes in the registry (with this table updated by the
      raising beads) or tracked as open registry items; none is implemented as a generic error.
- [ ] `contact.laws`, `contact.probe`, `contact.capacity`, `contact.audit`, `contact.soil` and
      `contact.response` each list their possible refusals in `registries/verbs.toml`, and that list is a subset
      of the registry.

## Test plan

### Unit tests

`crates/frob-contact/tests/refusals.rs`.

- `ref240_invalid_cone`: μ = −0.1; r_eff = −1 mm; μ₂ = 0.
- `ref241_no_rotor`: backlash rows requested on a card level without rotor state.
- `ref242_film_domain`: Π outside the calibrated range with `--strict-data`; without the flag the result is
  flagged `extrapolated` and is not a refusal.
- `ref243_soil_range`: a pressure of 200 kPa on a card valid to 100 kPa; slip 1.2 on a card valid to 1.0.
- `ref244_stiffness`: admission figure 0.06 against 0.05 on an `f32` tier; admitted in `f64`.
- `ref245_half`: `--precision f16`; repairs empty.
- `ref246_law`: CL-2 on `gpu-metal` before its admission.
- `ref320_thin`: a 2 mm plate in a 2 cm field with no thin route.
- `ref321_no_enclosure`: a clearance certificate with no registered bounds; `unknown` is offered.
- `ref822_cite`: citing a `capacity-biased` comparison.
- `registry_roundtrip`: the registry file parses, codes are unique, each has at least one repair except 245.
- `exit_code_map`: each code → 3.

### End-to-end script and logging

`scripts/e2e/traction_refusals.sh` (helper of STD-E2E): issues one planted request per code available at the
time (codes whose mechanisms land later are skipped with a logged reason), through `frob --json` and through
`frob-mcp`; asserts code, exit code, retry class and the repair list; logs per NDJSON line the command, exit
code, wall time and the full refusal block. `summary` counts codes exercised, skipped and failed.

### Known answers and negative controls

KA-420 (citing the capacity-biased comparison refuses with 822), KA-422 (cards over the bound refuse with 244),
KA-423 (a half-precision request refuses with 245), CX-170 (the thin plate, 320), CX-167 (clearance, 321).
Negative controls: a planted handler that returns a default instead of refusing (for the missing pair and for
half precision) must fail the tests; a repair list in the wrong order must fail `registry_roundtrip`.

## Considerations

- Several codes are raised by mechanisms that land at later milestones (242 at M3, 243 at M4, 320 and 321 at
  M3). The registry entries land at M1 so that the codes are reserved and documented; their triggers are tested
  when the mechanisms land.
- The uncoded refusals are a gap in the printed registry, not in the design; assigning their numbers is a
  registry change with no behavioural content.
- A refusal's ranked repairs are part of its contract: changing their order is a registry change that the
  agent-facing documentation is regenerated from.

## References

§18.14; Appendix O (O.2, O.3, O.8); §18.19, §18.20 (uncoded refusals); §32.4 (exit codes); §35.11 (families).
