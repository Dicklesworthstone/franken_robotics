---
key: "N14-REF"
title: "Refusal registry of Skeleton (§14): FROB-REF-101 to -126 with triggers, payloads, ranked repairs and exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-design", "solid", "registry"]
parent: "WP-07"
deps: ["BD-005", "BD-022", "BD-058"]
needs: ["§32.4: the envelope's `refusal` object and the exit-code table"]
related: ["N14b-10", "N14b-18", "N14b-34", "BD-064", "BD-110", "N14-CX"]
gates: ["GATE-SKEL-04"]
plan_refs: ["§14.13", "§14.4", "§14.8", "§14.11", "§14.17", "§32.4", "Appendix O"]
---
## Background

The Skeleton chapter's rule is: refusals instead of repairs in silence. An invalid design is refused with a
stable code, a message, and ranked repairs expressed as typed edits. The reason is that every silent repair of a
design (a defaulted fill, a clipped limit, a dropped field) yields a plausible wrong robot, and an agent cannot
learn from an error it never sees. A refusal with a ranked, executable repair is the opposite: the agent's next
action is in the envelope.

Codes 101 to 129 belong to this chapter; 130 to 139 are the actuator chapter's. Twenty-five codes are defined:
101 to 116 and 118 to 126. Code 117 is not assigned and, because codes are never reused, must not be given a new
meaning without the registry owner's decision. The machine-readable form is `registries/refusals.toml`; an
unknown code is a load-time error.

This bead is the registry of the chapter's codes: for each, its meaning, where it is raised, what its payload
carries, its ranked repairs and its exit code. The mechanisms that raise the codes are built in other beads;
this bead owns the registry rows, the per-code payload contract and the test that every code is raised by at
least one fixture and carries at least one repair.

## Scope

In: the 25 rows; the payload contract per code; one unnumbered refusal defined late in the chapter; the registry
file's rows and their tests. Out: the checks that raise the codes; the seeded-defect corpus (N14b-18); codes of
other chapters that preflight also raises (130 to 134).

## Technical approach

**Envelope contract.** When a verb refuses, `ok` is false and `refusal` carries: `code`, `message`, `pointer`
(the field or part at fault, as a path into the design, task or scene), `observed`, `required`, `unit`,
`affected_claims`, `retry` (`never`, `after-change` or `backoff`), and ranked `repairs`; each repair is a typed
edit with its arguments and, where it can be computed, a predicted effect. The first repair is also printed as a
command in `next_actions`. Text taken from an imported or user-supplied name appears only in a field marked
`x-frob-untrusted`.

**Exit codes.** A refused input exits 3. A stale parent is a conflict and exits 7. Exit 1 is never emitted on
purpose.

**The registry.** "Stage" is the preflight stage that raises the code in `design.check`; other verbs that raise
it are named. All have `retry: after-change` unless stated.

| Code | Refused | Raised by | Payload: pointer; observed and required | Ranked repairs (typed edits) | Exit |
|---|---|---|---|---|---|
| FROB-REF-101 | Port type mismatch; joint class and screw basis disagree | Stage 1 (ports); stage 3 (class against basis, by the subgroup classification) | The two port paths and their kinds; or the joint path, its declared class and the class its screws generate | `replace` the component with one whose port kind matches; rebind the `use` to the right port; `set-param` the joint class to the generated one, or correct the screw | 3 |
| FROB-REF-102 | Unit or dimension error | Stage 1 | The field; the dimension found and the dimension required | `set-param` with the same number in a unit of the required dimension (offered only when the conversion is unambiguous); otherwise the list of accepted unit tokens | 3 |
| FROB-REF-103 | Unknown, misspelt or duplicate field | Stage 1; every parser of `frob.design/1` and `frob.module/1` | The offending key and its location; the nearest known keys by edit distance | Rename to the nearest known key; remove the field; for a duplicate, keep one | 3 |
| FROB-REF-104 | Inertia not physically consistent | Stage 2 (exact LDLᵀ of the pseudo-inertia) | The body; the offending pivot and the smallest eigenvalue of the pseudo-inertia; the nearest consistent matrix | Accept the nearest consistent matrix as declared data (provenance `assumed`); `set-fill` to a geometric fill so the inertia is computed; correct the declared values | 3 |
| FROB-REF-105 | A body without a fill; a solid written as a shell | Stage 1 | The body; for a shell, the thickness and half the Region's smallest dimension | `set-fill` to `solid`, `{shell: t}`, `{lattice: φ}`, `declared` or `massless`, with the mass each would give; for an over-thick shell, `set-fill solid` or a thinner wall | 3 |
| FROB-REF-106 | Declared mass properties violate the mass-support bound | Stage 2 | The body; ‖c − o‖² + tr(I_C)/(2m) and the bound R², or tr(P·Q) for an ellipsoid | Correct the declared inertia or centre of mass; enlarge the declared mass Region to the real envelope; `set-fill` to a geometric fill | 3 |
| FROB-REF-107 | Empty joint range | Stage 3 | The joint and its interval or cone | `set-limits` with the bounds swapped when inverted; `set-limits unbounded` only if the caller asks | 3 |
| FROB-REF-108 | Power sign mismatch, copied power port or algebraic loop | Stage 5 | The junction and the ports on it with their signs; for a loop, the cycle of ports | Reverse the sign declaration on the named port; route a copied power port through a junction that balances flows; break the loop with a storage element or a declared causality | 3 |
| FROB-REF-109 | Declared symmetry differs from computed symmetry | Stage 5 | The asserted and computed groups; the breaker (the element that removes the excluded generator) | `assert-symmetry` with the computed group; or the edit that restores the symmetry (move the sensor onto the plane, mirror the payload, restore the gear sign), with the group it would give | 3 |
| FROB-REF-110 | A closure that over-constrains the mechanism (mobility below one) | Stage 3 | The closure; the mobility found by the rank test | `closure-remove`; change the closure's joint class to one that leaves mobility one; adjust the link the rank test names | 3 |
| FROB-REF-111 | A lever outside its bounds; an edit to a locked field | Stage 1; `design.edit`, `design.patch`, optimizer steps | The lever and its bounds, or the locked path | `set-lever` to the nearest bound; `declare-lever` with wider bounds; `unlock` explicitly, then retry | 3 |
| FROB-REF-112 | Stale parent in a patch | `design.patch` | The named parent, the current head, and the diff between the two | Re-read the head and re-issue the patch against it; `design.diff` to merge by hand | 7 |
| FROB-REF-113 | An actuator card with a required field of class `assumed` when the caller asked for `--strict-data` | Stage 6 | The card, the field and its provenance | Supply a `datasheet` or `measured` value in a local card; choose another card; drop `--strict-data` and accept an advisory | 3 |
| FROB-REF-114 | Module digest or port signature mismatch on import or substitution | Stage 1; `module.substitute`, `module.check` | The module reference; the digests or the two signatures | Re-pin the module to the digest that exists; choose a module with the required signature; for a substitution, accept it as global | 3 |
| FROB-REF-115 | A design budget exceeded: mass, cost, peak power, energy or envelope | Stage 6 | The budget; the value and the limit with units; the largest contributors | Edits on declared levers ranked by predicted effect per unit change; `swap-actuator`; `set-budget` if the limit itself is wrong | 3 |
| FROB-REF-116 | Co-design does not close: loop gain at or above one | Stage 6 (the mass-spiral fixed point) | The loop gain and the parameters in it | Shorten the limb lever; `swap-actuator` to a higher torque density; `regear`; reduce supported mass | 3 |
| FROB-REF-118 | The base cannot produce a motion the task class requires | Stage 4 | The growth vector or curvature span; the missing direction as a body twist | The roller edit, the steering edit or the joint edit that restores it, with the predicted growth vector | 3 |
| FROB-REF-119 | No admissible route or reach in the scene | Stage 7 | The bottleneck as a named scene feature with certified and impossible margins; or the task height and the reach | Design edits that restore passage or reach (a width or length lever, `reshape`, a larger wheel); separately, the scene retrofit that removes the bottleneck, when the catalog has one | 3 |
| FROB-REF-120 | Observation alias under every admitted probe | Stage 8 | The two worlds, the transcript, the decision instant, the probes covered | `add-sensor` of the cheapest catalog entry whose transcript differs on the pair; `retime` for a higher rate or earlier packet; add a probe to the task | 3 |
| FROB-REF-121 | Contact law missing or incompatible with its end effector | Stage 1 | The end effector; the law and the reason (missing; a torsion radius above the end effector's own contact radius; a law that needs a field the geometry lacks) | `set-contact-law` to a compatible law with its interface card; reduce the torsion radius; add the missing field | 3 |
| FROB-REF-122 | An unsupported element in an import under `--strict` | `design.import` | The element with file and line; the loss-report entries | Remove or map the element; import without `--strict` and read the loss report; expand macros outside | 3 |
| FROB-REF-123 | Self-collision or interpenetration at the reference pose or over declared ranges | Stage 3 | The two bodies (or body and ground), the configuration and the depth | `set-limits` to the collision-free range; `move-mount`; `reshape` the smaller body | 3 |
| FROB-REF-124 | A static margin the task requires is not met | Stage 6 | The margin and the requirement; the stance and the load-sharing model | Widen the stance (`move-mount`); lower or centre the mass; `set-budget` of payload; for a composed-screen exclusion, the certificate | 3 |
| FROB-REF-125 | Unsupported schema version | Stage 1; every parser | The version found and the versions supported; the named migration | `migrate` with the named migration; open with the compiler version that wrote it | 3 |
| FROB-REF-126 | A sensor without a parent frame, a rate or a clock | Stage 1 | The sensor and the missing field | `add-sensor` fields: the parent frame, a rational rate in hertz, a clock name | 3 |

**The unnumbered refusal.** A late subsection of the chapter adds: an import that drops a body of nonzero mass
without declaring a reduction is refused. It has no code in the registry. Proposed row (to be assigned by the
registry owner; 127 is the next free code of this chapter's block): *mass dropped at import without a declared
reduction*; raised by `design.import`; payload: each dropped body with its mass and the total; repairs: fold
the body, import it, or declare the reduction; exit 3.

**Registry rows.** Each code is an entry of `registries/refusals.toml` with `code`, `meaning`, `defined_in`,
`exit`, `retry`, `payload` (the field names above) and `repairs` (the typed-edit kinds it may offer). The verb
registry's refusal lists refer to these entries.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the 25 rows with the meanings above, each with exactly one exit code, and
      loads without error; a use of an unknown code fails at load time.
- [ ] Code 117 is absent and reserved; the late refusal has an assigned code before the reduced-model bead
      closes.
- [ ] For every code there is at least one fixture that raises it and whose envelope carries the payload fields
      listed, at least one repair as a typed edit, and a `next_actions` command.
- [ ] For every code whose first repair is executable, applying it removes that refusal (checked by the
      preflight end-to-end script for the preflight codes, and by the patch and import scripts for 112 and 122).
- [ ] No code is raised with a message alone: a refusal with an empty `repairs` list fails the conformance test
      unless the row says no repair exists.

## Test plan

### Unit tests

- `registry_has_25_skeleton_codes`: {101…116, 118…126}.
- `one_exit_per_code`; `stale_parent_is_exit_7`; `all_others_exit_3`.
- `payload_fields_per_code`: for each row, a fixture's refusal contains the listed fields.
- `repairs_are_typed_edits`: every repair's `edit` is in the typed-edit vocabulary or is a scene retrofit marked
  as a scene edit.
- `unknown_code_is_load_error`; `code_117_not_reusable`.
- Per-code spot values: 104 on principal moments (1, 1, 3) kg·m² reports the pivot and a proposed matrix; 106 on
  the 1 kg, 0.008 kg·m², 0.10 m fixture reports 0.012 m² against 0.010 m²; 109 on the forward lidar reports
  computed C₂ against asserted C₂ × C₂ and names the lidar mount; 112 returns the current head and a diff; 118
  on parallel rollers reports growth vector (2, 3) and the missing direction; 125 names the migration.

### End-to-end script and logging

Covered by `scripts/e2e/skel_design_check.sh` (N14b-19) for the preflight codes, which asserts the full payload
and applies the first repair; by the import script for 122 and the late refusal; and by the patch script of the
patch bead for 112. Each logs the code, the pointer, observed and required values with units, the repair applied
and whether it removed the refusal.

### Known answers and negative controls

- KA-263 (105 on an over-thick shell), KA-276 (109), KA-268 (118), KA-277 to KA-284 (the first rejection
  fixtures and their codes), KA-292 (112 in 100 of 100 trials).
- Negative controls: a registry in which two exit codes are given to one refusal code; a refusal emitted with a
  code of another chapter's block from a Skeleton check; a repair expressed as free text instead of a typed
  edit. Each must fail its test.

## Considerations

- The exit code of 112 needs the surface owner's confirmation: the exit table's rule for the `1xx` family gives
  3, and its conflict row names a stale parent and gives 7. This registry uses 7, because the caller's remedy is
  the conflict remedy (re-read the head), and records the choice for the parity gate that requires exactly one
  exit code per refusal code.
- The chapter's refusal table lists meanings; triggers and repairs are spread over the field reference, the
  stage table and the mechanism subsections. Where the chapter states a repair it is quoted here (104, 109, 112,
  118, 119, 120, 125); the others are DESIGN and are refined by the beads that raise them, without changing a
  code's meaning.
- Codes are stable interface. A change of meaning is a new code; a retired code stays in the registry as
  retired.
- 119 and 120 depend on Frontier stages; if stage 8's kill gate fires, 120 stays registered and is no longer
  emitted by preflight.

## References

§14.13 (the refusal table), §14.4 (field reference: 103, 105, 107, 109, 111, 121, 125, 126), §14.8 (112),
§14.11 (which stage raises what), §14.17 (the late refusal), §32.4 (envelope and exit codes), Appendix O.
