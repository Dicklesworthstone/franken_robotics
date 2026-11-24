---
key: "N18-CX"
title: "frob-conformance: Traction counterexample registry CX-167 to CX-183, seeding the negative-control registry"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "traction", "gauntlet", "frob-conformance", "registry", "test", "review-notes-open"]
parent: "WP-14"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N18-KA", "N18-REF", "N18b-32", "N18b-38", "N18b-40", "N18b-42", "N18b-43", "N18b-45", "N38b-04"]
gates: []
plan_refs: ["Appendix N.15", "§18.3", "§18.4", "§18.5", "§18.6", "§18.7", "§18.8", "§18.9", "§18.11", "§18.12", "§18.16", "§18.19", "§18.20"]
---
## Background

Where a plausible shortcut is wrong, the project records the smallest hand-checkable case that breaks it. Such a
case is first-class: it enters the counterexample catalogue and seeds the negative-control registry, so that
every gate it protects is tested against the shortcut planted as a mutation. A gate that nothing can fail
validates nothing, and a rule that ships without such a case has not been thought through.

This bead is the registry of the 17 counterexamples that belong to the contact subsystem (collision, contact,
wheels, tracks, terrain), plus the counterexamples stated in two late subsections that have no catalogue number
yet. Each row gives the tempting shortcut, the case with its numbers, and the rule it protects. Mechanism beads
cite these numbers in their test plans and state the values they need inline; this bead is where the complete
set lives as data and where the mapping from counterexample to planted mutation is kept. A CX number is never
reused and never renumbered; new rows append.

## Scope

In: the rows below as entries of the counterexample catalogue `frob-conformance/registry/counterexamples.toml`,
with the substance each needs to become a control (the rule identifiers and gates it protects, the kind of
control, and one planted-defect identifier per row); a host test that recomputes every number; the chapter's
script that shows each planted defect fails its gate.

Out, with the bead that has it: the catalogue's record schema and loader; the seeding tool that turns catalogue
entries into rows of `frob-conformance/registry/controls.toml`, allocates their `NC-` identifiers, keeps the seed
map, resolves a rule that names no gate, and proves completeness by the join (N38b-05). This bead writes no row
of `controls.toml` by hand: its controls appear there when the seeding is re-run over its entries, which is
append-only and idempotent. The control runner and the registry's five rules, including that a gate with no
control cannot pass (N38b-04). The gates and mechanisms that use the controls.

## Technical approach

**Data.** Each row is an entry of `frob-conformance/registry/counterexamples.toml` in the catalogue's record
schema (N38b-05): `id` (the CX number), `shortcut`, `case`, `numeric`, `plan_case` (`"words"` or `"numbers"`:
the form of the case in the plan's text, immutable), `case_supplied_by` (empty, or the bead and fixture that
supplied a numeric case for a word-only entry), `rule`, `rule_ids` (the gates, invariants, obligations,
decisions and refusal codes of the last column), `defined_in`, `same_as`, `label`, `control_kind`, `plant` and,
for an appended entry, `slug`. CX-168 and CX-175 are two of the catalogue's seventeen word-only entries
(`plan_case = "words"`): CX-175 takes `numeric = true` with `case_supplied_by = "N18-CX: scaled stopping-test
instance"` (the instance in its row), and CX-168 stays `numeric = false`, so the seeding writes it as an
obligation until the enclosure bead supplies its fixture. `plant` names the planted defect: for the mutation kind, a build or plan flag that plants the
shortcut in the real component (never a mock). Every gate among a row's `rule_ids` must fail with the plant
enabled. DESIGN, correcting an earlier draft of this bead that wrote rows `{ cx, shortcut, case, rule,
defined_in, mutation, gates }` straight into `controls.toml`: that file is written only by the seeding tool, so
that control identifiers are allocated once and never rewritten; a chapter states its counterexamples and their
plants in the catalogue, and the tool derives the controls. The field this bead called `mutation` is the
catalogue's `plant`, and `gates` is the gate subset of `rule_ids`.

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects |
|---|---|---|---|
| CX-167 | A positive nominal gap is clearance | A nominal gap of 6 mm with 4 mm of outward error on each side leaves the outer enclosures overlapping by 2 mm: the answer is `unknown` until an error term is reduced. With 10 mm nominal and the same errors the answer is `clear` with 2 mm to spare | INV-TRAC-03 (`clear` from disjoint outer enclosures only, `penetrates` from intersecting inner enclosures only); GATE-TRAC-07; strict inequalities by outward rounding or exact-integer kernels |
| CX-168 | Two swept volumes that overlap in space collide | The bodies can occupy the shared region at different times and never touch; the test is per time slab, never on whole-sweep volumes (no numeric case is given) | The per-slab enclosure test; GATE-TRAC-07 |
| CX-169 | If the gap is positive at both ends of a step, no contact occurred in it | A sample sphere of radius 2 mm moves at 5 m/s with h = 2 ms, so it travels 10 mm. A 2 mm plate lies 1 mm ahead. Clearing the plate takes 1 + 4 + 2 = 7 mm, so at the end of the step the sphere is 3 mm beyond the far face: both endpoints are clear and the step encloses a collision | Swept bounds and event location under transactional profiles; the speculative margin of at least v·h + ε_g on lockstep tiers; GATE-TRAC-06 (the endpoint-only mutation must miss); GATE-TRAC-03 |
| CX-170 | A scene distance field at working resolution represents all the geometry | A trilinear field on a 2 cm grid cannot hold a 2 mm plate: the plate vanishes or is thickened to the grid | Features thinner than two fine voxels are routed to thin primitives (`K-np-thin`) or the mesh path; FROB-REF-320 for a thin feature with no route |
| CX-171 | Sliding friction and torsional friction can be independent rows | With sliding speed v and spin ω on the four-dimensional cone the tangential force is μ·N·v/√(v² + (r_eff·ω)²), which is μ·N/√2 at v = r_eff·ω. Independent rows grant μ·N of sliding friction and μ·N·r_eff of torsional friction at once, 41% more in magnitude than the surface can supply; a foot that pivots while it slides would hold where it should slip | The four-dimensional cone (PO-TRAC-01); GATE-TRAC-04 (the independent-row mutation must fail) |
| CX-172 | Backlash can be modelled as a dead zone on the torque command | A dead zone produces no impact when the gap closes and no hunting in a position loop. The rows give both: a rotor of inertia 10⁻⁴ kg·m² at 5 rad/s behind a ratio of 10 crosses a 0.5° gap in β·N/ω₀ = 17.5 ms and, closing inelastically on an output of 0.05 kg·m², leaves it at J·N·ω₀/(J·N² + I_out) = 0.083 rad/s | Backlash as two unilateral rows on the relative angle with the rotor as its own coordinate; FROB-REF-241; GATE-TRAC-04; GATE-SINEW-04 |
| CX-173 | A tracked or skid-steer base can be modelled with rolling (no lateral slip) constraints | Exact lateral no-slip at contact stations a ahead of and behind the centre requires v_y + a·ω = 0 and v_y − a·ω = 0, hence ω = 0. The base turns only by slipping, so its turning is a property of the friction law | The belt law with an anisotropic friction ellipse and yaw scrub; the rejection fixture "turning in place at zero scrub work"; the nonholonomic model is a planning model and an oracle for rolling only |
| CX-174 | Diagonal scaling makes the Cholesky factor more accurate | Without pivoting, the computed factor of S·H·S for power-of-two S is S times the computed factor of H bit for bit, absent overflow and underflow: its rounding errors are already those of the scaled problem. What scaling repairs is everything around the factor: tolerances, stopping tests, line-search thresholds, iterative passes | PO-TRAC-03; GATE-TRAC-10; ADR-TRAC-07 |
| CX-175 | One tolerance on the unscaled Newton gradient serves every robot | Degrees of freedom mix metres and radians, and their diagonal entries differ by orders of magnitude, so one tolerance on the unscaled gradient is a dimension error; in scaled variables the gradient norm is dimensionless. Numeric instance (from the numerics rules): H = diag(1, 10⁻⁸) with residual (0, 10⁻⁷) passes an absolute tolerance of 10⁻⁶ while the error in the second unknown is 10 | Diagonal scaling in stage 8 (`K-scale-diag`); INV-TRAC-05 (impulses and forces typed apart; no tolerance applies to both) |
| CX-176 | A half-precision factor with single-precision refinement cheapens the contact solve | Refinement contracts only if κ·u₁₆ < 1, with u₁₆ ≈ 4.9·10⁻⁴. A rank-one contact operator with proximal shift ρ = 10⁻²·tr(W)/48 for 16 contacts has κ = 1 + 48/0.01 = 4,801 and κ·u₁₆ = 2.34. Cost: k refinement steps cost 9k/n times the factor they replace: 2.3 at n = 27 with seven steps, 1.5 at n = 35 with six, 3.0 at n = 18 with six. Emulated on three reference models (FACT, to be re-run): κ·u₁₆ ≥ 1 in 37 to 64% of sampled contact states and failure in 9 to 18% | ADR-TRAC-07; FROB-REF-245; single precision admitted only below the scaled κ·u bound (FROB-REF-244) |
| CX-177 | A wet floor is a map of constant friction coefficients | By the integrated Stefan law, a film thins to 10 µm in 0.038 s under a 2 cm patch carrying 100 N on water, in 2.3 s under the same patch on cooking oil, and in 0.76 s under a 6 cm patch carrying 400 N on water. Against a stance of 0.3 s a small foot on water reaches boundary friction almost at once, while oil and large flat feet ride a film for the whole stance | The film law with per-contact film thickness in contact memory; GATE-TRAC-05; KILL-FILM (the law must change at least 10% of outcomes against a constant patch at μ_b) |
| CX-178 | A foot that has squeezed the film out stands on a dry floor | A squeezed film recovers to the boundary-lubricated coefficient μ_b, not to the dry coefficient μ_d; only h₀ = 0 returns μ_d exactly | INV-TRAC-07 |
| CX-179 | A wider sole gives more traction on a patchy floor | Sole straddling a low-grip strip (μ 0.25 inside, 0.8 outside, uniform pressure) with the boundary at offset z: the high-grip share is clip(½ + z/w, 0, 1). At z = 0 the capacity is 0.525·N for any width; at z = 12 mm it is 0.690·N for w = 40 mm and 0.619·N for w = 70 mm. Area alone does not predict traction | The sole-strip row of the known-answer battery; load-weighted quadrature of patches |
| CX-180 | When a world exceeds its contact capacity the extra contacts can be dropped | A comparison is honest only if both arms were simulated. A tracked design against a point-foot design under an artificially low cap must return `capacity-biased` and no winner; the label applies whenever the arms differ by more than one percentage point of worlds in `Degraded` or capacity-refused episodes | INV-TRAC-04; INV-KIN-05; FROB-REF-721, FROB-REF-722, FROB-REF-822; GATE-TRAC-06 |
| CX-181 | The convex (associated) friction model is Coulomb friction | Dropping the term μ·‖u_T‖ from the cone complementarity gives the associated model, in which a sliding contact acquires a separation velocity μ·‖u_T‖. The compliant default law has that artifact in compliant form | The hard-contact law as the written specification of the referee; the fidelity ledger measures the artifact; the contact-law arm of the model-form check; GATE-TRAC-09 (no separation velocity on a sliding block) |
| CX-182 | A library integrator for a differential drive can serve as the rolling oracle | Heading 32 rad, wheel speeds 1 and 1 + 6·10⁻¹³ m/s, track 0.5 m, 400 steps of 1/400 s: the sibling's planar-drive integrator at the pinned commit is reported to return zero translation where about (0.8342, 0.5514) m is correct (FACT to recheck). Identical to CX-029 of the design-language chapter: one fixture identity | The closed form Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)) with sinc(0) = 1 is the oracle; the admission test INT-FSIM-05 and upstream work item FD-FSIM-11 |
| CX-183 | Friction memory can stay on a contact when its feature changes | A wheel rolling over a sharp sill moves its witnesses from face to edge to face. Tangential memory kept on the changed feature produces a couple the contact forces do not explain; the fixture compares work, impulse and the feature sequence under step refinement (bounds: 1% at h/4 and 10⁻⁹ N·m in `f64`) | INV-TRAC-06 (memory is transported or reset at every witness change, each reset an event); GATE-TRAC-06 |

**Kind, plant and label per entry (DESIGN).** `plant` is `<carrier>:cx-<nnn>` (or `<carrier>:<tag>` for an
appended entry): the Cargo feature (for a `mutation`) or the `synthetic` fixture (for the other kinds) under which
the carrying bead builds the shortcut in the real component; the carrier may refine the name, keeping the number
or tag in it. A rule column that names no gate (CX-170, CX-173, CX-176, CX-178, CX-179, CX-182) gives `rule_ids`
its identifiers and the defining section, which the seeding tool resolves through `registry/rule_gates.toml`.

| Entry | `control_kind` | `plant` (the planted shortcut) | `label` |
|---|---|---|---|
| CX-167 | mutation | `N18a-03:cx-167` (clearance read from the nominal gap) | DESIGN |
| CX-168 | planted-world | `N18a-03:cx-168` (two bodies crossing one region at different times; `numeric = false`, an obligation until the fixture exists) | DESIGN |
| CX-169 | mutation | `N18a-13:cx-169` (detection on step endpoints only) | DESIGN |
| CX-170 | planted-design | `BD-232:cx-170` (a 2 mm plate in a 2 cm field with no thin route) | DESIGN |
| CX-171 | mutation | `BD-167:cx-171` (independent sliding and torsional rows) | DESIGN |
| CX-172 | mutation | `BD-084:cx-172` (backlash as a dead zone on the torque command) | DESIGN |
| CX-173 | planted-design | `BD-245:cx-173` (a skid base under rolling constraints; the "turning in place at zero scrub work" fixture) | DESIGN |
| CX-174 | mutation | `N18b-25:cx-174` (a scaling that is not a power of two, so the scaled factor is no longer S times the unscaled one bit for bit) | DESIGN |
| CX-175 | mutation | `BD-156:cx-175` (one absolute tolerance on the unscaled gradient) | DESIGN |
| CX-176 | planted-input | `N18b-27:cx-176` (a half-precision factor requested with the `FROB-REF-245` check removed, `removed_check`) | FACT: emulation on three reference models; pilot record `PILOT-PT-19` (the conditioning survey, re-run by N38a-11 at M0). The pair is entered from that record when the row is written: `rerun = true` with `rerun_receipt` = the receipt of the record's result where `pilots::rerun_status("PILOT-PT-19")` reads `reproduced`, `rerun = false` where it reads `not-run` or `differs` (N38b-05 refuses any other pair, `rerun-disagrees-with-pilot`); N18b-27 then replaces the figures with the measured share |
| CX-177 | mutation | `BD-230:cx-177` (a constant-μ patch for a wet floor) | DESIGN |
| CX-178 | mutation | `BD-230:cx-178` (a squeezed film recovering to μ_d) | DESIGN |
| CX-179 | mutation | `N18b-32:cx-179` (sole capacity proportional to contact area) | DESIGN |
| CX-180 | mutation | `N18b-22:cx-180` (overflowing contacts dropped and a winner returned) | DESIGN |
| CX-181 | mutation | `N18a-21:cx-181` (the associated model taken as the referee's specification) | DESIGN |
| CX-182 | planted-input | `BD-033:cx-182` (the sibling's planar-drive integrator offered as the rolling oracle) | FACT: the sibling's routine at the pinned commit, reported; `rerun = false` until BD-033's admission INT-FSIM-05 re-runs it |
| CX-183 | mutation | `N18a-14:cx-183` (tangential memory kept on a changed feature) | DESIGN |
| `trac-arc` | mutation | `N18b-43:trac-arc` (the chord-only swept bound, D = 0 in `swept::bound`; the plant record carries `feature = "chord-only-swept-bound"`, the test-only feature of `frob-contact` that N18b-43 builds and catches on its containment row; N18b-45 runs it on the referee and N18b-44 on the batch profile) | DESIGN |
| `trac-resid` | mutation | `N18b-40:trac-resid` (the residual bound without the rounding margin ρ) | DESIGN |
| `trac-lambda` | mutation | `N18b-40:trac-lambda` (an uncertified eigenvalue, λ = 3, as divisor) | DESIGN |
| `trac-mode` | mutation | `N18b-40:trac-mode` (the response column answered on the frozen active set with the mode guard off) | DESIGN |

A FACT entry's control is written by the seeding tool as an `obligation` until `rerun = true` with a receipt.

**Correction to the printed catalogue.** CX-182's row prints the admission test as INT-FSIM-01; the gate
register, which governs, gives the planar-drive admission as INT-FSIM-05 (INT-FSIM-01 is the bedrock-crates build
check). The registry row says INT-FSIM-05.

**Related rows owned by other chapters and cited by contact beads.** CX-148 (a four-sided friction pyramid
grants √2·μ along its diagonals, 41% more than the cone; GATE-TRAC-04, GATE-FID-05). CX-151 (the referee admits
at most 21 contacts: a sole with eight samples per foot fits when standing, 16, and overflows in a fall; six
feet with four samples need 24).

**Counterexamples stated in late subsections, without a catalogue number.** They are appended to the catalogue,
which is append-only. Their numbers come from the allocation ledger
`crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`, bead N38a-40), never from
"the next free number" at commit time: this bead files one `[[block]]` request with `registry = "cx"`,
`count = 4`, `owner = "N18-CX"`, `section = "§18.19, §18.20"`, the allocator fills `first` and `last` (the
plan-time catalogue ends at CX-384 and CX-385 to CX-400 are the verification chapter's), and the four tags below
are mapped to the block's numbers in table order in one commit. Until then the tag of the first column is the
row's name; after mapping it stays in the catalogue's `slug` field. A tag still used in a registry, test name or
derivation file at this bead's milestone (M1) fails the ledger check `provisional-tag-past-due`.

| Provisional id | The tempting shortcut | The smallest case that breaks it | The rule it protects |
|---|---|---|---|
| `trac-arc` | A bound swept along the chord between end positions encloses the motion | A 1 m arm turns from −60° to +60° about the origin; tip sphere of radius 5 cm; obstacle at (1, 0) m. End positions (0.5, ∓0.866) m; the chord lies on x = 0.5 m and the padded chord reaches x = 0.55 m, yet the tip passes through (1, 0). The sagitta is 0.5 m, so the true bound reaches 1.05 m | Swept bounds enclose arcs: dilation R·(1 − cos(Δθ/2)) or ½·√(L² − ‖B − A‖²); GATE-TRAC-12 (the chord-only mutant fails) |
| `trac-resid` | A computed residual of zero means zero error | In single precision H = 3, b = 1 give x̂ = 0.33333334, a computed residual of exactly 0 and a true error of 9.9·10⁻⁹; with the rounding margin ρ = 2.4·10⁻⁷ the bound is 7.9·10⁻⁸ | The response-batch bound (‖r̂‖₂ + ρ)/λ_lo; GATE-TRAC-11; GATE-GAUNT-11 |
| `trac-lambda` | Any eigenvalue serves as the divisor of the residual bound | H = [[2, 1], [1, 2]] (eigenvalues 1 and 3), b = (3, 3), x̂ = (1.01, 0.99): residual norm 0.01414 and true error 0.01414. A bound formed with λ = 3 claims 0.00471 and is violated | λ_lo is a certified lower bound on the smallest eigenvalue; GATE-TRAC-11 (the λ = 3 mutation is caught) |
| `trac-mode` | A linear response on the frozen active set is valid for any counterfactual load | A block rests on a floor with μ·N = 5 N, anchor in stick. A tangential load of 4 N is answered by friction; a load of 6 N slides under a net 1 N, while the frozen sticking set reports no motion | The mode guard of the response batch reroutes the column to the full solve; GATE-TRAC-11 |

## Acceptance criteria

- [ ] 17 catalogue rows (CX-167 to CX-183) and the four appended rows exist in `counterexamples.toml`, each with
      a `plant` identifier and the gates it must fail among its `rule_ids` (or `numeric = false` and the state
      `obligation` for CX-168, which has no numeric case yet). CX-168 and CX-175 carry `plan_case = "words"`;
      CX-175 carries `numeric = true` and a non-empty `case_supplied_by`; every other row `plan_case = "numbers"`.
- [ ] The four appended rows carry numbers from a block issued to N18-CX in `allocations.toml`, in table order,
      with their tags in `slug`; `registry::alloc::validate` passes and no tag remains outside `slug`.
- [ ] After the seeding tool of N38b-05 is re-run, each of the 21 entries has exactly one control in
      `controls.toml` whose `seed` names it, the tool's second run changes no byte, and the join passes.
- [ ] A host test recomputes every numeric case in the tables and matches the printed values.
- [ ] Every entry carries the `control_kind`, `plant` and `label` of the table above; CX-176 and CX-182 are FACT
      with their re-run beads named: CX-176's label names `PILOT-PT-19` and its pair equals that record's
      derived status (`rerun = true` with the record's receipt where `reproduced`, otherwise `rerun = false`),
      and CX-182 carries `rerun = false` until BD-033's re-run; CX-182's `rule_ids` name INT-FSIM-05.
- [ ] `scripts/e2e/traction_controls.sh` exits 0.
- [ ] Every entry resolves to at least one gate after seeding, directly or through `rule_gates.toml`; an entry
      that resolves to none is in the seeding tool's findings file with `gate-added` requested and is reported,
      never dropped.
- [ ] No row uses a mock: each mutation is a flag on the real component.

## Test plan

### Unit tests

`crates/frob-conformance/tests/traction_counterexamples.rs` (host arithmetic, exact where rational).

- `cx167_enclosures`: 6 − 2·4 = −2 mm (overlap, `unknown`); 10 − 8 = +2 mm (`clear`).
- `cx169_thin_barrier`: 5 m/s × 2 ms = 10 mm; 1 + 4 + 2 = 7 mm; 10 − 7 = 3 mm.
- `cx171_coupling`: at v = r_eff·ω the factor is 1/√2 = 0.7071; independent rows give magnitude √2, 41% more.
- `cx172_backlash`: 0.01745 s and 0.0833 rad/s.
- `cx173_skid`: the 2 × 2 system v_y + a·ω = 0, v_y − a·ω = 0 has the unique solution v_y = ω = 0 for a ≠ 0.
- `cx175_unscaled_tolerance`: H = diag(1, 10⁻⁸), residual r = (0, 10⁻⁷): ‖r‖ = 10⁻⁷ passes an absolute
  tolerance of 10⁻⁶, while the error H⁻¹·r = (0, 10) has a second component of 10; in scaled variables
  (S = diag(1, 10⁴)) the same state fails the tolerance.
- `cx176_half`: κ = 4,801; κ·2⁻¹¹ = 2.344; 63/27, 54/35, 54/18.
- `cx177_stefan`: 0.0377 s, 2.26 s, 0.763 s from t = 3π·η·a⁴/(4·N·h²) with h = 10 µm, a the patch radius
  (2 cm, 2 cm, 6 cm), N = 100, 100, 400 N, and η = 1 mPa·s (water) or 60 mPa·s (cooking oil).
- `cx179_strip`: 0.525, 0.690, 0.619.
- `cx182_chord`: (cos 32, sin 32) = (0.83422, 0.55143).
- `cx_traction_kinds_plants`: every entry's `control_kind`, `plant` and `label` equal the table; every `plant`
  names its carrier; CX-182 carries `rerun = false` and a source; CX-176's label names `PILOT-PT-19`, and on a
  scratch register its pair loads only when it equals `pilots::rerun_status("PILOT-PT-19")` (`rerun = true`
  with the record's receipt when `reproduced`; `rerun = false` when `not-run` or `differs`).
- `arc`, `resid`, `lambda`, `mode`: the provisional rows' numbers (0.55, 0.5, 1.05; 9.93·10⁻⁹, 2.38·10⁻⁷,
  7.95·10⁻⁸; 0.01414 against 0.00471; 4 N held, 6 N slides).

### End-to-end script and logging

`scripts/e2e/traction_controls.sh` (helper of STD-E2E; it calls only cargo targets of its dependencies): runs
`cargo test -p frob-conformance --test traction_counterexamples`; runs N38b-05's seeding tool, `cargo test -p
frob-conformance --test seed_controls -- --ignored seed`, twice, asserting that the second run leaves the digest
of the committed `controls.toml` unchanged; runs the join, `cargo test -p frob-conformance --test seed_join`;
reads `registry/control_history.ndjson` and reports per entry its control's state (`runnable`, `obligation`,
`stale` while the carrier has not built the plant) and last recorded outcome; then plants the registry's own
negative control below on a scratch copy. Running each plant against its gate is the control runner's (N38b-04),
invoked by the carriers' own scripts, and is not repeated here. Logs per NDJSON line, beside the helper's
standard fields (command line, exit code, wall time), the CX number or tag, the control's `NC-` identifier, the
plant, the gate, the state and the last outcome; the digest of `controls.toml` before and after the second
seeding; on failure the first missing entry or unresolved gate. `summary` lists entries, controls runnable,
obligations, stale controls with their carriers, and findings.

### Known answers and negative controls

This bead is the counterexample catalogue of the chapter, from which its negative controls are seeded. Its own
controls, each on a scratch copy: an entry whose `plant` names no carrier and one with an empty `control_kind`
(`cx_traction_kinds_plants`); CX-182 relabelled DESIGN (`cx_traction_kinds_plants`); CX-176 written
`rerun = true` while its pilot record reads `differs` (N38b-05's `rerun-disagrees-with-pilot`); an appended
entry numbered outside N18-CX's block (the ledger's `unallocated-number`). A plant that does not change behaviour (a no-op flag)
is reported by the control runner as `missed`, a failure of the control, not a pass (N38b-04).

## Considerations

- Rows CX-168 and CX-175 were printed with no numeric case. CX-175 takes the numeric instance of the scaled
  stopping test; CX-168 needs a fixture (two bodies crossing one region at different times) defined by the
  enclosure bead before its control can run.
- Owed by the carriers and the control runner, not by this registry: for every row whose mechanism exists, the
  conformance run with the plant enabled fails the listed gate and with it disabled passes; until then the
  control is `stale` and its gate cannot pass.
- A mutation flag is test-only and must be impossible to enable in a release build (compile-time feature, checked
  by the release rubric).

## References

Appendix N.15 (CX-167 to CX-183); §18.19 and §18.20 for the provisional rows; §38.11 (false-progress guards).
