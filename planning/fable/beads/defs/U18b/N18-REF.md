---
key: "N18-REF"
title: "frob-contact: Traction refusal registry: FROB-REF-240 to -246, -320, -321, -822 with repairs and exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "traction", "frob-contact", "registry", "test"]
parent: "WP-10"
deps: ["BD-005", "BD-022", "N32a-03", "N35x-02", "STD-E2E"]
needs: []
related: ["BD-167", "N12-51", "N14-REF", "N15ax-02", "N16a-02", "N18a-15", "N18b-04", "N18b-12", "N18b-22", "N18b-26", "N18b-27", "N18b-28", "N18b-39", "N18b-43", "N18bx-03", "N35x-02", "NAB-33"]
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
| FROB-REF-241 | No rotor state | Backlash rows requested at a card level without a rotor degree of freedom | 1. Raise the card level (to an actuator level that carries rotor state: L3, the level that carries the rotor as its own coordinate). 2. Set the card's `backlash` to zero (no backlash rows are then built) | 3 | after-change | N15a-09's function that builds the backlash descriptor from the card, the one raise site; the joint rows reach it and add no second site (BD-084 on the referee, BD-155 on the batched tiers, the row view of N18a-32) |
| FROB-REF-242 | Film law out of domain | The Hersey number Π or the ratio h/σ lies outside the card's calibrated range under `--strict-data` | 1. Use a calibrated card. 2. Accept `extrapolated` (drop `--strict-data`) | 3 | after-change | Film law (BD-230); `contact.probe` (N18b-29) |
| FROB-REF-243 | Soil or tyre law out of range | Pressure, slip or sinkage outside the card's validity range | 1. Another card. 2. The rigid-floor model | 3 | after-change | Soil laws (N18b-12, N18b-13); tyre laws (N18b-04); `contact.soil` (BD-366) |
| FROB-REF-244 | Stiffness beyond single precision | Estimated scaled κ·u above the admission bound (default 0.05) on an `f32` tier | 1. `batch-cpu` in `f64`. 2. A softer qualified card | 3 | after-change | Stiffness admission (N18b-26) |
| FROB-REF-245 | Half precision | A half-precision solve or state buffer is requested | None; not offered | 3 | never | Precision admission (N18b-27) |
| FROB-REF-246 | Law not admitted | A law requested on a (tier, profile) cell whose status in the law-admission table is not `admitted`: `not-built` (no admission row for the cell), `pending`, `failed` or `fallback` (the cell's gate has not passed there, or a kill gate fired and the message names the fallback), or, from M2, `not-certified` (a lockstep cell whose law has no checked convexity certificate for its current definition hash). `observed` = (law, tier, profile, status), and for `not-certified` also the definition hash that has no receipt; `required` = the gate that must pass | 1. The admitted law for that tier and profile. 2. `--profile strict` on `batch-cpu` where the law is admitted there. For `not-certified`, ranked: 1. regenerate and check the certificate; 2. the admitted law; 3. the referee | 3 | after-change | The check `admitted(law, tier, profile)` (N18a-15); the law-admission table, its status derivation and the envelope payload (N18b-28), called by plan admission (`admit`, N16a-02) in every verb that creates an environment; the certificate status `not-certified` (N12-51, with the rows of BD-167; M2) |
| FROB-REF-320 | Thin feature | Scene geometry thinner than two fine voxels with no thin route | 1. A thin primitive. 2. A finer brick. 3. The mesh path | 3 | after-change | Scene compile and the mesh path (BD-232) |
| FROB-REF-321 | No enclosure | A clearance certificate requested with no registered error bounds for a body or scene | 1. Register bounds. 2. Accept `unknown` | 3 | after-change | Enclosure tests, `contact.clearance` (BD-232, BD-233) |
| FROB-REF-822 | Capacity-biased verdict | A `capacity-biased` comparison is cited as a verdict | 1. Raise capacity. 2. Apply a qualified patch reduction to both arms | 3 | after-change | Capacity-bias rule (N18b-22) |

**Refusals the chapter names without a code.** Three refusals are stated in prose with no entry in the refusal
registry, and three more are the contact beads' own (below the table). No contact bead types their numbers:
each is a row of the allocation ledger `registries/refusal_allocations.toml` (second round, N35x-02) under the
slug below, numbered at the M0 run by the pure allocation rule of N35-33 (deduplicate; family by what is
refused; the lowest free number in the family above the defining chapter's highest code). This bead enters each
as a full registry entry (name, family, exit, retry, message, repairs) under the number that the generated
hand-off page `docs/generated/refusal_round2.md` gives it, and the raising bead raises that number behind one
named constant; a row the ledger holds (`held`) is raised in the ledger's interim form, never as a generic error:

| Ledger slug | Refusal | Raised when | Family | Repairs, ranked | Exit | Retry | Raised by |
|---|---|---|---|---|---|---|---|
| `response-batch-stale-anchor` | Response batch requested against a stale anchor | A response batch is requested against an anchor digest other than the one it was factored for | `2xx` | 1. Refactor for the new anchor. 2. Request with the current anchor | 3 | after-change | N18b-39 |
| `response-anchor-unconverged` | Response anchor requested at a step that did not converge | An anchor is requested at a step whose final scaled gradient norm is above the plan's tolerance, or that is flagged `Degraded` (the factor would belong to an iterate, not to the solution); `observed` the norm against the tolerance, or the `Degraded` flag with its cause; `required` a converged, non-degraded step | `2xx` | 1. Anchor the last committed step of that world that converged and is not `Degraded` (the default of `contact.response --step`). 2. Re-run the step to convergence from its snapshot under a larger iteration budget (`--newton-iterations`) and anchor that run | 3 | after-change | N18b-39 (capture) |
| `swept-bound-without-speed-bound` | Swept bound requested without a speed bound | A transactional profile is requested for a model that supplies neither a joint speed limit nor actuator and load limits | `2xx` | 1. Declare a joint speed limit. 2. Declare actuator torque and load limits | 3 | after-change | N18b-43 |
| `interface-pair-missing` | Missing interface pair | A (mover material, substrate material, condition) triple that can meet has no interface card | `2xx` | 1. Add a card for the pair and condition. 2. Choose a material that has one | 3 | after-change | BD-167 |
| `patch-reduction-unqualified` | Patch reduction not qualified | A patch reduction is requested for a (sample group, scenario class) pair with no passing qualification receipt in `registries/patch_reductions.toml` | `2xx` | 1. Run the qualification fixture for the pair. 2. Choose a qualified reduction listed by `contact.capacity`. 3. `--profile strict` (no contact cap) | 3 | after-change | N18b-21 |
| `rut-capacity-overflow` | Rut capacity overflow, refused | A rut write needs a tile beyond the world's rut pool capacity T_rut and the caller chose refuse | `7xx`, sub-family `72x` beside 722 | 1. Raise T_rut (a new `StepPlanId`). 2. Reset. 3. Continue-flagged | 5 | after-change | N18b-17, N38a-29 |

`interface-pair-missing` is not `FROB-REF-121` and is not merged into it (DESIGN, read from the plan, which
states the two separately; the same verdict is written in N14-REF). 121 is a design-level refusal at preflight
stage 1: an end effector names no contact law, or a law or card that does not fit it. `interface-pair-missing`
is a table lookup: the design's law may be fine, but the (mover material, substrate material, condition) triple
that the scene makes meet has no card. It is raised at the card lookup (BD-167) and by any binding that reads the
interface table, so a binding that finds no card for an attachment pair raises this slug, not 121.

Three rows are not named with a code in the chapter's prose. `response-anchor-unconverged` is the capture rule
of the response anchor (DESIGN of N18b-39: an anchor is taken only from a step that converged and is not
`Degraded`, §18.19). Of the last two rows, the first is the refusal the documented patch reduction of §18.11
needs once reductions are a registry of qualified rows (DESIGN of N18b-21); the second is "the capacity refusal for rut tiles" that the rut mechanism (§18.10, M4 form and batch-scale form)
names, which `FROB-REF-722` (contact or row capacity) does not cover. All three are rows of the same ledger, so
they are numbered by the same rule and never emitted as a generic error.

**Codes of other chapters that contact beads raise or cite** (not owned here; meanings carried for
self-containment): FROB-REF-121 (contact law missing or incompatible with its end effector; also a torsion
radius above the foot's own contact radius, a tyre card on a foot (N18b-04), and from M3 the braked wheel-pair
triggers of N14bp2-02, all listed in N14-REF); FROB-REF-222 (`strict` or `referee` requested on a GPU tier);
FROB-REF-224 (indeterminate contact: no solution, or several distinct post-event motions, the Painlevé rod);
FROB-REF-248 (a solve ill-conditioned beyond its precision profile under the `refuse` policy, the run-time
counterpart of 244); FROB-REF-721 (an island exceeds the referee's contact cap of 21, or the fixed-witness gate
refuses; repairs: patch reduction with its fixture, `--profile strict`; exit 5); FROB-REF-722 (a world exceeds
its contact or row capacity and the caller chose refuse; repairs: raise capacity with a new `StepPlanId`, reset,
continue-flagged; exit 5). The overflow code depends on the tier, by one function (N21b-23, test
`overflow_code_by_tier`, recorded in N16-REF and N21-REF): `batch-cpu` raises 722 (exit 5), and `gpu-cuda` and
`gpu-metal` raise FROB-REF-610 (device capacity overflow under the `refuse` policy, exit 6). A contact bead that
asserts an overflow refusal on a GPU tier therefore expects 610, never 722.

**Registry entry form (DESIGN).**

```toml
[[refusal]]
code = "FROB-REF-244"
name = "Stiffness beyond single precision"
family = "2xx"
exit = 3
retry = "after-change"
defined_in = "§18.14"
message = "Estimated scaled κ·u {observed} exceeds the admission bound {required} on tier {tier}."
repairs = ["Run on batch-cpu in f64", "Use a softer qualified card"]
raised_by = ["N18b-26"]
```

`defined_in` is the plan subsection (N35-33 orders and checks rows by it) and `raised_by` holds the bead keys of
the table's "Raised by" column, the field N35-33's registry-wide check reads.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the ten entries with name, family, exit code 3, retry class, message
      template and ranked repairs exactly as tabulated; the loader rejects a duplicate or unknown code.
- [ ] Each entry renders its message template with sample `observed` and `required` values, and `frob --json
      schema` lists the ten codes with their exit codes and ranked repairs (registry-level checks, all at M1).
- [ ] For each of the ten codes a planted-trigger test and an e2e step are written here: through `frob --json`
      one envelope with that code, exit 3 and the repairs in rank order, and the same through the MCP surface
      with `isError: true`. The raising beads depend on this registry, so a trigger runs from its raising bead's
      landing on (it is an acceptance item of that bead, named per row) and is skipped with a logged reason
      before; a skipped trigger is never counted as passed.
- [ ] Every code maps to exactly one exit code; exit code 1 never appears.
- [ ] `FROB-REF-245` returns an empty repair list and `retry: never`.
- [ ] The six uncoded refusals have ledger rows under their slugs; every assigned number is entered here as a
      full entry; a row still `held` is emitted in the ledger's interim form; none is implemented as a generic
      error.
- [ ] `contact.laws`, `contact.probe`, `contact.capacity`, `contact.audit`, `contact.soil` and
      `contact.response` each list their possible refusals in `registries/verbs.toml`, and that list is a subset
      of the registry.

## Test plan

### Unit tests

`crates/frob-contact/tests/refusals.rs`. The registry-level tests (`registry_roundtrip`, `exit_code_map`,
`message_templates_render`, `uncoded_slugs_in_ledger`) run at M1; each `refNNN_*` trigger test below is written
here with its fixture and is enabled by the raising bead's landing (it is part of that bead's acceptance).

- `message_templates_render`: every template renders with sample values and no unfilled placeholder.
- `uncoded_slugs_in_ledger`: the six slugs exist in `refusal_allocations.toml`, the three chapter-named ones,
  `response-anchor-unconverged` and `patch-reduction-unqualified` with family `2xx`, and `rut-capacity-overflow`
  in `7xx`, sub-family `72x`; the exit code of each entered entry follows its family (3, or 5 for `72x`).
- `ref240_invalid_cone`: μ = −0.1; r_eff = −1 mm; μ₂ = 0.
- `ref241_no_rotor`: backlash rows requested on a card level without rotor state; the two repairs in order
  (raise the level to L3; set `backlash` to zero), and the same refusal whether reached through BD-084's or
  BD-155's rows (one raise site, N15a-09).
- `ref242_film_domain`: Π outside the calibrated range with `--strict-data`; without the flag the result is
  flagged `extrapolated` and is not a refusal.
- `ref243_soil_range`: a pressure of 200 kPa on a card valid to 100 kPa; slip 1.2 on a card valid to 1.0.
- `ref244_stiffness`: admission figure 0.06 against 0.05 on an `f32` tier; admitted in `f64`.
- `ref245_half`: `--precision f16`; repairs empty.
- `ref246_law`: CL-2 on `gpu-metal` before its admission (status `pending`, CL-1 the first repair); (CL-2,
  `referee`) with status `not-built`; and, run with N12-51 (M2), a lockstep plan naming a law whose receipt is
  for another definition hash, status `not-certified` with the hash and the three repairs in order, while the
  same law is admitted on the referee.
- `ref121_not_interface_pair`: a design whose foot names an admitted law, placed on a substrate whose material
  triple has no card, is refused with `interface-pair-missing` and not with 121 (run with BD-167).
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
