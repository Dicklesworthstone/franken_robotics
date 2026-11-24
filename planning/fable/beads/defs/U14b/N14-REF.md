---
key: "N14-REF"
title: "Refusal registry of Skeleton (§14): FROB-REF-101 to -126 with triggers, payloads, ranked repairs, exit codes"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-design", "solid", "registry", "review-notes-open"]
parent: "WP-07"
deps: ["BD-005", "BD-022", "N32a-03"]
needs: []
related: ["N13-34", "N14a-14", "N14ax-01", "N14b-10", "N14b-18", "N14bp2-02", "N18-REF", "N23bx-03", "N32-REF", "N35-33", "N35x-02", "N41p2-01"]
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

In: the 25 rows; the payload contract per code; the five refusals of the chapter with no printed code (one
defined late in the chapter, the Region-admission refusal, the two travel refusals and the unmet module
assumption), each with its ledger slug, payload, repairs and interim form; the registry file's rows and their
tests. Out: the checks that raise the codes; the seeded-defect corpus (N14b-18); codes of
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
| FROB-REF-107 | Empty joint range; a joint with no `limits` member | Stage 3 (empty or inverted range, `home` outside it); stage 1 (a missing `limits` member, refused when the document is typed, because limits are a physics field with no default) | The joint and its interval or cone | `set-limits` with the bounds swapped when inverted; `set-limits unbounded` only if the caller asks | 3 |
| FROB-REF-108 | Power sign mismatch, copied power port or algebraic loop | Stage 5 | The junction and the ports on it with their signs; for a loop, the cycle of ports | Reverse the sign declaration on the named port; route a copied power port through a junction that balances flows; break the loop with a storage element or a declared causality | 3 |
| FROB-REF-109 | Declared symmetry differs from computed symmetry | Stage 5 | The asserted and computed groups; the breaker (the element that removes the excluded generator) | `assert-symmetry` with the computed group; or the edit that restores the symmetry (move the sensor onto the plane, mirror the payload, restore the gear sign), with the group it would give | 3 |
| FROB-REF-110 | A closure that over-constrains the mechanism (mobility below one) | Stage 3 | The closure; the mobility found by the rank test | `closure-remove`; change the closure's joint class to one that leaves mobility one; adjust the link the rank test names | 3 |
| FROB-REF-111 | A lever outside its bounds; an edit to a locked field | Stage 1; `design.edit`, `design.patch`, optimizer steps | The lever and its bounds, or the locked path | `set-lever` to the nearest bound; `declare-lever` with wider bounds; `unlock` explicitly, then retry | 3 |
| FROB-REF-112 | Stale parent in a patch: the `parent` named in the patch document is not the head | `design.patch` (the patch reader; the surface's lineage write does not also report 804 for this request) | The named parent, the current head, and the diff between the two | Re-read the head and re-issue the patch against it; `design.diff` to merge by hand | 7 |
| FROB-REF-113 | A card field of class `assumed` on which a stage-6 verdict rests, when the caller asked for `--strict-data`: a required field of an actuator card, or an assumed price on any card summed for cost (actuator, battery, sensor or material card) | Stage 6; `actuator.check`; `module.characterize` under `--strict-data` (N14b-28, M3: an assumed field of the torque envelope) | The card, the field and its provenance; for a cost verdict, the cards whose price is `assumed` | Supply a `datasheet` or `measured` value in a local card (for a price: cite or measure it); choose another card; drop `--strict-data` and accept an advisory | 3 |
| FROB-REF-114 | Module digest or port signature mismatch on import or substitution | Stage 1; `module.substitute`, `module.check`; `module.show` (N15b-02: a hash that does not match the file's bytes); `module.characterize` (N14b-28, M3: a digest or port signature mismatch on the module reference given); `design.edit` `extract-module` (N41p2-01: a declared port that contradicts the derived signature, or a `use` that does not reproduce the subterm it replaces) | The module reference; the digests or the two signatures | Re-pin the module to the digest that exists; choose a module with the required signature; for a substitution, accept it as global | 3 |
| FROB-REF-115 | A design budget exceeded: mass, cost, peak power, energy or envelope | Stage 6 | The budget; the value and the limit with units; the largest contributors | Edits on declared levers ranked by predicted effect per unit change; `swap-actuator`; `set-budget` if the limit itself is wrong | 3 |
| FROB-REF-116 | Co-design does not close: loop gain at or above one | Stage 6 (the mass-spiral fixed point) | The loop gain and the parameters in it | Shorten the limb lever; `swap-actuator` to a higher torque density; `regear`; reduce supported mass | 3 |
| FROB-REF-118 | The base cannot produce a motion the task class requires | Stage 4 | The growth vector or curvature span; the missing direction as a body twist | The roller edit, the steering edit or the joint edit that restores it, with the predicted growth vector | 3 |
| FROB-REF-119 | No admissible route or reach in the scene | Stage 7 | The bottleneck as a named scene feature with certified and impossible margins; or the task height and the reach | Design edits that restore passage or reach (a width or length lever, `reshape`, a larger wheel); separately, the scene retrofit that removes the bottleneck, when the catalog has one | 3 |
| FROB-REF-120 | Observation alias under every admitted probe | Stage 8 | The two worlds, the transcript, the decision instant, the probes covered | `add-sensor` of the cheapest catalog entry whose transcript differs on the pair; `retime` for a higher rate or earlier packet; add a probe to the task | 3 |
| FROB-REF-121 | Contact law missing or incompatible with its end effector | Stage 1 (the law selectors and their table, N14a-05, M1; the braked pair, N14bp2-02, M3); the precondition of the `set-contact-law` edit (BD-058); the module reader and `module.check` (BD-060); `design.edit` `extract-module` (N41p2-01); the selector table for tyre cards (N18b-04, M4) | The end effector; the law and the reason: missing; a law that does not fit the kind (a `wheel` with `foot-patch`; a tyre card on a foot); a torsion radius above the end effector's own contact radius; a law that needs a field the geometry lacks; an interface card whose first material is not the end effector's. From M3, the triggers of the braked pair (N14bp2-02), each with the observed and the admitted value: a second law (`braked`, `switch`) on a `foot`, a `track` or a wheel whose primary law is not `wheel-rolling`; a pair without a switching rule; a rule without a pair; a braked law that the table's `braked_on` column does not admit; `patch_radius` above the wheel's contact radius; `torsion_radius` above `patch_radius`; a `switch.joint` that is not the wheel's own `R` joint (naming the joint found) | `set-contact-law` to a compatible law with its interface card; reduce the torsion radius; add the missing field. For the braked pair, ranked: 1. `set-contact-law` with the same object completed by `switch` on the wheel's own joint, or with `braked` removed (the single-law object); 2. `set-contact-law` with `patch_radius` set to the contact radius, or `torsion_radius` set to `patch_radius`; 3. `reshape` the wheel to a wider tread | 3 |
| FROB-REF-122 | An unsupported element in an import under `--strict` | `design.import` | The element with file and line; the loss-report entries | Remove or map the element; import without `--strict` and read the loss report; expand macros outside | 3 |
| FROB-REF-123 | Self-collision or interpenetration at the reference pose or over declared ranges | Stage 3; the precondition of a `design.edit` `mirror` or `radial` wrap (BD-058): the wrapped limb is mounted on the constructor's plane or axis, so its image would coincide with it | The two bodies (or body and ground), the configuration and the depth; for a wrap, the limb, its mount and the plane or axis | `set-limits` to the collision-free range; `move-mount`; `reshape` the smaller body | 3 |
| FROB-REF-124 | A static margin the task requires is not met | Stage 6 | The margin and the requirement; the stance and the load-sharing model | Widen the stance (`move-mount`); lower or centre the mass; `set-budget` of payload; for a composed-screen exclusion, the certificate | 3 |
| FROB-REF-125 | Unsupported schema version | Stage 1; every parser | The version found and the versions supported; the named migration | `migrate` with the named migration; open with the compiler version that wrote it | 3 |
| FROB-REF-126 | A sensor without a parent frame, a rate or a clock | Stage 1 | The sensor and the missing field | `add-sensor` fields: the parent frame, a rational rate in hertz, a clock name | 3 |

**Refusals of the chapter with no printed code.** Five conditions that Skeleton checks refuse have no code in
the printed registry. Each is a row of the refusal allocation ledger `registries/refusal_allocations.toml`
(second round, N35x-02), family `1xx`, exit 3, retry `after-change`, under the slug below; its number is the
one N35-33's allocation rule gives it at the run, and this registry enters each as a full row (`code` the
assigned number, `name` the slug, the payload and repairs below, `defined_in`, `raised_by`). The raising bead
reads the code through one named constant and never types a number. The ledger's `interim` field records the
existing code and `pointer` under which each condition is told apart wherever it is raised on an interim
(N35-33 rule 7: that code, with `detail` equal to the slug); a refusal of this kind is never downgraded to a
warning.

| Ledger slug | `defined_in` | Refused | Raised by | Payload | Repairs, ranked | Proposal; interim (code and `pointer`) |
|---|---|---|---|---|---|---|
| `import-mass-dropped` | §14.17 | An import drops a body of nonzero mass without declaring a reduction | `design.import` (the reduced-model bead N14b-34) | Each dropped body with its mass, and the total | Fold the body; import it; declare the reduction | 127; none |
| `region-not-admissible` | §14.4 | A Region not admissible as a mass support, at stage 1: a mesh that fails admission (open, inconsistently oriented, non-positive signed volume), or a quadrature Region whose relative mass bound exceeds 10⁻² | Stage 1 (the Region kernels, N14a-14) | The body, the Region, the kernel's reason (open-edge count, orientation, signed volume) or the measured bound against 10⁻² | Write the Region as disjoint primitives; supply a closed mesh; `set-fill declared` | none; 102 with `region.admission` (mesh) or `region.bound` (quadrature bound) |
| `travel-value-out-of-domain` | §14.4 | A travel value outside its domain (a negative margin, step height, gap, force or torque) or a posture joint value outside that joint's `limits` | Stage 1, the `travel` block (N14ax-01, M3), constant `TRAVEL_VALUE_OUT_OF_DOMAIN` | The value and its interval; for a posture joint, the joint and its interval | Put the value inside the interval shown; widen the joint's limits if the design intends it | none; 111 with `travel.domain` or `travel.posture-limits` |
| `travel-envelope-optimistic` | §14.4 | A declared travel envelope less conservative than the one derived from geometry | Stage 1, the `travel` block (N14ax-01, M3), constant `TRAVEL_ENVELOPE_OPTIMISTIC` | The declared and the derived values | Declare `derived`; raise `top` or lower `step_over` to the derived value | none; 111 with `travel.envelope` |
| `module-assumption-unmet` | §14.6 | A module port whose contract `assumes` is not met by what its junction offers (the bus nominal outside `bus_voltage`; a shared sink's conductance divided over its members below `sink_conductance_min`) | The port-junction check at the join (N15b-01) | `pointer` `contract.assumes.<name>`, the observed and the required value with units | (DESIGN) Change the junction so that it meets the assumption (the value shown); substitute a module whose `assumes` the junction meets | none; 114 with `contract.assumes.<name>` |

The `travel` block's fields are part of the closed field reference (§14.4), which is why both travel rows are
defined there. 128 is the proposal of the algebraic core's ledger row `lifted-law-without-turn-state`
(N13-REF; first round, N35-33), a refusal of the model compiler at stage 1, so the two `1xx` proposals of
§13 and §14 are 128 and 127 and are distinct. The block's next free number below the actuator chapter's 130 is
then 129; which request receives it, and what the others receive, is decided by the rule (N35-33 rule 4: the
lowest number above the chapter's highest code if free, otherwise the lowest free number outside every chapter's
block, otherwise `requested` with `range-full`), not by this registry.

**Registry rows.** Each code is an entry of `registries/refusals.toml` with `code`, `meaning`, `defined_in`,
`exit`, `retry`, `payload` (the field names above), `repairs` (the typed-edit kinds it may offer) and `raised_by`
(bead keys, the field the registry-wide check of N35-33 reads; `unbound` only for a code no bead raises yet).
The verb registry's refusal lists refer to these entries. `raised_by` per code: the stage-1 owners are those of
the stage-1 bead N14a-08 (101: N14a-02, N14a-07; 102: N14a-01; 103: BD-054, N14a-03, N14a-04, N14a-05, BD-055;
105: BD-055; 107, the missing `limits` case: N14a-04; 111: BD-055, N14a-06; 114: BD-060; 121: N14a-05, and
N14bp2-02 from M3; 125: BD-054; 126: BD-055); `design.check` assembles stages 1 to 6 (BD-110) and stages 7 and 8
(BD-244), and the stage beads named in the rows raise the later-stage codes (104 and 106: BD-100 with the cone
predicate BD-051; 108: N15b-01, the port junctions (a junction sign, a copied power port, an algebraic loop);
109: N14a-11; 110: BD-101; 113: N15b-06, the strict-data rule over a verdict's read set, with `actuator.check`
(BD-069), `module.characterize` (N14b-28, M3) and the cost verdict on assumed prices (N41-13, M5); 115: N14a-10,
and BD-103 in `analyze.closure` (a mass spiral that closes above a declared mass or power budget); 116: BD-103 at
stage 6 and in `analyze.closure`; 118: N14b-11; 119: N14b-12; 120: N14b-13; 123: BD-104; 124: BD-100, the
static-margin finding of A-08); the codes raised outside preflight have their verbs' beads (111, 114, 121 and the
stage-1 codes on an edit, and 123 on a `mirror` or `radial` wrap: BD-058; 112: BD-059; 103, 108, 111, 114, 121
and 125 in the `frob.module/1` reader (BD-060: an unknown field, a reversed port sign, a parameter default outside
its bounds, a digest mismatch, a contact port whose law is missing or incompatible, an unsupported schema
version); 114: BD-077, `module.check` and `module.show` (N15b-02) and `module.characterize`
(N14b-28); 114, 108 and 121 on `extract-module`: N41p2-01; 122: BD-065, BD-066).

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the 25 rows with the meanings above, each with exactly one exit code, and
      loads without error; a use of an unknown code fails at load time.
- [ ] Code 117 is absent and reserved; the late refusal has an assigned code before the reduced-model bead
      closes, and that code is not the one proposed by N13-REF (128).
- [ ] The five refusals with no printed code are ledger rows under the slugs of the table (N35x-02), and the
      registry holds a full row for each one the ledger has assigned; `interim_codes_listed` checks that every
      interim code-and-pointer pair raised by a Skeleton check is either a registered row or a ledger row.
- [ ] A stale `parent` in a `design.patch` document yields 112 and never 804 (`stale_patch_parent_is_112_not_804`).
- [ ] For every code there is at least one fixture that raises it and whose envelope carries the payload fields
      listed, at least one repair as a typed edit, and a `next_actions` command.
- [ ] For every code whose first repair is executable, applying it removes that refusal (checked by the
      preflight end-to-end script for the preflight codes, and by the patch and import scripts for 112 and 122).
- [ ] No code is raised with a message alone: a refusal with an empty `repairs` list fails the conformance test
      unless the row says no repair exists.

## Test plan

### Unit tests

- `registry_has_25_skeleton_codes`: {101…116, 118…126}.
- `one_exit_per_code`; `stale_parent_is_exit_7`; `all_others_exit_3`; `stale_patch_parent_is_112_not_804`.
- `payload_fields_per_code`: for each row, a fixture's refusal contains the listed fields.
- `repairs_are_typed_edits`: every repair's `edit` is in the typed-edit vocabulary or is a scene retrofit marked
  as a scene edit.
- `unknown_code_is_load_error`; `code_117_not_reusable`; `interim_codes_listed` (the interim pairs
  `102/region.admission`, `102/region.bound`, `111/travel.*` and `114/contract.assumes.*` are ledger rows, under
  the slugs `region-not-admissible`, `travel-value-out-of-domain`, `travel-envelope-optimistic` and
  `module-assumption-unmet`).
- `uncoded_rows_entered`: every one of the five slugs whose ledger status is `assigned` has a registry row
  with that number, `name` equal to the slug and the payload fields of the table; `import-mass-dropped` and
  `lifted-law-without-turn-state` hold distinct numbers.
- Per-code spot values: 104 on principal moments (1, 1, 3) kg·m² reports the pivot and a proposed matrix; 106 on
  the 1 kg, 0.008 kg·m², 0.10 m fixture reports 0.012 m² against 0.010 m²; 109 on the forward lidar reports
  computed C₂ against asserted C₂ × C₂ and names the lidar mount; 112 returns the current head and a diff; 118
  on parallel rollers reports growth vector (2, 3) and the missing direction; 125 names the migration; 121 on the
  braked wheel pair (run with N14bp2-02, M3; cylinder wheel of radius 0.06 m and width 0.03 m): `patch_radius`
  20 mm reports 15 mm admitted and offers it, `torsion_radius` 14 mm with `patch_radius` 12 mm offers 12 mm,
  `braked` without `switch` offers the completed object first.

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

- 112 and the surface's FROB-REF-804 (stale parent: the expected parent is not the head of the lineage) name
  one situation at two layers. DESIGN, stated identically in the surface registry N32-REF (owner of 804), so
  that one request gets one code: a `design.patch` whose patch document names a `parent` that is not the head
  is refused with 112 by the patch reader of the design crate, which returns the current head and the diff
  between the two; that request is never also reported as 804. 804 is raised by the surface's lineage write for
  the `expected_parent` argument of every other `mutate` verb. Both exit 7. The plan defines both codes and
  does not divide them; the division by raising site keeps the patch's richer payload (the diff) where the
  patch is read.
- The exit code of 112: the exit table's rule for the `1xx` family gives 3, and its conflict row names a stale
  parent and gives 7. This registry uses 7, as the surface registry N32-REF does for the same division, because
  the caller's remedy is the conflict remedy (re-read the head); 112 is therefore one of the codes the registry's
  `[exit_rule]` lists under `conflict` (N35-33), so that every row's exit equals its rule and the parity gate's
  one exit code per refusal code holds.
- The chapter's refusal table lists meanings; triggers and repairs are spread over the field reference, the
  stage table and the mechanism subsections. Where the chapter states a repair it is quoted here (104, 109, 112,
  118, 119, 120, 125); the others are DESIGN and are refined by the beads that raise them, without changing a
  code's meaning.
- Codes are stable interface. A change of meaning is a new code; a retired code stays in the registry as
  retired.
- 121 and the contact chapter's `interface-pair-missing` (a ledger row of N18-REF, family `2xx`) are two
  conditions, not one, and are not merged (DESIGN, stated identically in N18-REF): 121 is the design-level
  refusal (an end effector names no law, or a law or interface card that does not fit it); a (mover material,
  substrate material, condition) triple that can meet and has no interface card in the table is
  `interface-pair-missing`, raised at the card lookup (BD-167) and at any binding that reads the table.
- 119 and 120 depend on Frontier stages; if stage 8's kill gate fires, 120 stays registered and is no longer
  emitted by preflight.

## References

§14.13 (the refusal table), §14.4 (field reference: 103, 105, 107, 109, 111, 121, 125, 126), §14.8 (112),
§14.11 (which stage raises what), §14.17 (the late refusal), §32.4 (envelope and exit codes), Appendix O.
