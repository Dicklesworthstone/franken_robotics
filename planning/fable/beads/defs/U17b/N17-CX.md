---
key: "N17-CX"
title: "Phylum counterexample registry: CX-153 to CX-166 as negative controls, with the limb-packet standing case"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "phylum", "frob-phylum", "frob-conformance", "solid", "registry", "test"]
parent: "WP-44"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N17-KA", "N17a-04", "N17a-08", "N17a-10", "N17a-11", "N17a-12", "N17a-13", "N17a-16", "N17a-17", "N17a-19", "N17a-21", "N17a-23", "N17a-24", "N17a-25", "N17a-26", "N17a-27", "N17a-28", "N17a-30", "N17b-13", "N17b-22", "N21c-16", "N21c-17"]
gates: []
plan_refs: ["Appendix N (N.14)", "§17.5", "§17.6", "§17.9", "§17.10", "§17.11", "§17.13", "§17.21", "§38.11"]
---
## Background

Phylum (crate `frob-phylum`) batches populations of different robot designs and removes duplicate work from design
campaigns. Almost every mechanism in it has a plausible shortcut that is wrong, and the project's rule is that a
shortcut ships with the smallest hand-checkable case that breaks it. Each such case is a counterexample with a
stable number, and each is the seed of at least one record in the negative-control registry
(`crates/frob-conformance/registry/controls.toml`): the shortcut is planted as a code mutation or as an input, and
the gate of the rule it protects must catch it. The reason is stated in the plan as "a gate that nothing can fail validates
nothing": a gate records PASS only together with a caught negative control.

This bead is the registry of the fourteen counterexamples of the Phylum chapter, CX-153 to CX-166, with the exact
numbers of each case, the rule it protects, the form in which it is planted, and the bead whose test must catch it.
It also carries two cases that the chapter states for limb-archetype kernels (§17.21) and that the printed
catalogue does not number; they are appended to the catalogue with numbers from the allocation ledger.

Terms used below. Γ_mech is the mechanical symmetry group of a robot body; Γ_obs is its subgroup that also preserves
every sensor pose and field of view, the payload, the task's goal orientation and the terrain's symmetry. A mission
digest is a hash of exactly what the step plan and the task read from a design. Ω_p is the padding overhead of a
batch padded to its largest member for work that grows as the p-th power of robot size L: Ω_p = L_maxᵖ / E[Lᵖ],
with r = E[L]/L_max.

## Scope

In: the fourteen catalogue entries CX-153 to CX-166 and the two appended entries, as entries of the counterexample
catalogue `crates/frob-conformance/registry/counterexamples.toml` in its record schema (N38b-05), each with the
substance it needs to become a control: the rule identifiers and gates it protects, the kind of control, and one
plant identifier per planted defect; the block request for the two appended numbers; a host test that recomputes
every number; the chapter's script that seeds and joins the entries.
Out, with the bead that has it: the catalogue's record schema and loader, and the seeding tool that turns entries
into rows of `crates/frob-conformance/registry/controls.toml`, allocates their `NC-` identifiers, keeps the seed
map and proves completeness by the join (N38b-05). This bead writes no row of `controls.toml`, by hand or by copy
(BD-025, "Who writes a record"): its controls appear there when the seeding is re-run over its entries, which is
append-only and idempotent. The control runner, which builds each plant and runs it against its gate, and the
rule that a gate with no caught control cannot pass (N38b-04, BD-025). The allocation ledger and its checks
(N38a-40). The mechanisms, their plants and their own tests: the planting bead is named in each entry's `plant`
id (table below), and the bead whose test catches it in the last column of the catalogue table. The orbit-pair
and digest replays of GATE-PHYLUM-01 that catch CX-153, CX-154 and CX-157 are N17a-28's; CX-160 to CX-162 are
asserted from the user's side by N17a-08; CX-166 is exercised by N17a-21. The known answers of the chapter are
the registry `N17-KA` (written by the unit that owns the first half of the chapter).

## Technical approach

**Data.** Each row is an entry of `crates/frob-conformance/registry/counterexamples.toml` in the catalogue's record
schema (N38b-05): `id` (the CX number), `shortcut`, `case`, `numeric`, `plan_case` (`"words"` or `"numbers"`: the
form of the case in the plan's text, immutable), `case_supplied_by` (empty, or the bead and fixture that supplied a
numeric case for a word-only entry), `rule`, `rule_ids` (the gates, invariants, obligations, decisions and refusal
codes of the rule column), `defined_in`, `same_as`, `label` (with `rerun` on a FACT entry), `control_kind`, `plant`
and, for an appended entry, `slug`. Every entry is `plan_case = "numbers"` and `numeric = true` except CX-166, whose
plan text gives no numeric case: it is `plan_case = "words"` (one of the catalogue's seventeen word-only entries),
`numeric = true`, with `case_supplied_by = "N17a-19: the mirror fixture split by seed parity"` (a quadruped whose
left-front thigh is 0.24 m and the other three 0.21 m, against the same design with the right-front thigh long, with
a forward lidar so that Γ_obs is the left-right mirror). `same_as` is empty on the fourteen; the appended
`limb-packet-singular` is `same_as = "CX-024"` (the same packet as CX-024 and CX-129). A gate among an entry's
`rule_ids` must fail with the plant enabled; a rule column that names no gate, kill gate or admission gate (CX-156,
CX-158, CX-159, CX-160, CX-161) gives `rule_ids` its invariants, obligations and refusal codes with the defining
section, which the seeding tool resolves through `registry/rule_gates.toml`.
DESIGN, correcting an earlier draft of this bead that wrote records `{ cx, shortcut, planted_as, plant, catches,
test, expected }` straight into `controls.toml` and compiled mutations under a feature `planted-defects`: that file
is written only by the seeding tool, so that control identifiers are allocated once and never rewritten, and the
feature of a mutation is the name after the colon of its plant id, built one control per build as `--features
gauntlet-mutations,<crate>/<feature>` (BD-025, N38b-04). `planted_as` is the catalogue's `control_kind`, `plant`
its `plant`, and `catches` the gate subset of `rule_ids`.

**The fourteen catalogue rows.**

| CX | The tempting shortcut | The smallest case that breaks it, with its numbers | The rule it protects | Planted as | Caught by |
|---|---|---|---|---|---|
| CX-153 | A design sweep may be quotiented under the robot's mechanical symmetry group | Ten leg types on the six slots of a radial hexapod: (10⁶ + 2·10 + 2·10² + 10³ + 3·10⁴ + 3·10³)/12 = 86,185 orbits under the dihedral group of order 12, a reduction of 11.6. With a front-only sensor only the sagittal mirror preserves observations and the count is (10⁶ + 10³)/2 = 500,500, a reduction of 2.0. Quotienting under the larger group merges designs the closed loop can tell apart | INV-PHYLUM-03 (quotients use Γ_obs only); `FROB-REF-161`; GATE-PHYLUM-01 (the lidar control) | Input: a hexapod with a front-only lidar and a request to quotient under Γ_mech | BD-285; N17b-15 |
| CX-154 | Placements with the same first moment are the same experiment | Ballast fixture: eight identical slugs in 24 collinear slots enter rigid-body dynamics through the count, Σx and Σx². Of C(24, 8) = 735,471 placements there are 24,546 distinct (Σx, Σx²) pairs. Slots {1, 4} against slots {2, 3}, beside six common slugs, give Σx = 5 twice and Σx² = 17 against 13: two classes, not one | The mission digest hashes exactly what the step plan and the task read; GATE-PHYLUM-01 (the equal-Σx control); KILL-QUOTIENT | Mutation: a digest that omits the second moment | BD-285 |
| CX-155 | A local summary that carries peak demand is a sound key for pruning or reuse | Two fingers with identical local summaries ("peak demand one unit") on a bus limited to one unit of current. One-tick schedules: both must draw in the same tick, 2 > 1, so zero are feasible. Two-tick schedules: finger A then B, or B then A: two are feasible. A contract that adds peak demands concludes 2 > 1 and prunes a feasible design | INV-PHYLUM-04; the negative controls of the invalidation contract; GATE-PHYLUM-05 | Mutation: a reuse key built from summed peaks with no time-resolved profile | BD-286 |
| CX-156 | Members of a symmetry orbit give equal outcomes at equal world addresses | The design g·d in world w behaves as d does in world g⁻¹·w. Orbit members have equal outcome distributions, not equal outcomes at equal addresses. Distributional results are shared; a per-address result is shared only after the address is mapped through the group action, and a comparison paired by address against a third design is re-run | The orbit rule of campaign quotienting; the control "an unmapped per-address result reused" | Mutation: per-address results served across an orbit with the identity address map | BD-286; BD-285 |
| CX-157 | Equal mission digests prove that two designs give identical traces | The statement is about the simulator and cannot be discharged in a proof kernel. It holds by construction, because the kernels read nothing but the tables, and is checked by replay of sampled pairs inside a class (evidence class Tested, determinism class D2) | "Structural, not proved"; GATE-PHYLUM-01 (zero false quotients in 10⁴ sampled class pairs); KILL-QUOTIENT | Mutation: a kernel that reads one field outside the digested tables (a planted read of a label hash) | BD-285 |
| CX-158 | A memo key can be written by hand from the inputs the author remembers | A dependency omitted from a key returned a stale promotion in an executed test (four edit states of a two-scene fixture; the key without packet delivery time kept a stale result at the fourth state) | Keys are derived from declared inputs; one hit in 64 is recomputed in shadow and one mismatch stops serving that node kind; `FROB-REF-162`; INV-PHYLUM-04 | Mutation: a node kind whose key omits one declared input | N17b-13 |
| CX-159 | Exact reuse after an edit is a speed lever | An edit that touches a shared battery bus, a shared thermal sink, a kinematic loop across branches or any mass or geometry of a robot in contact invalidates every episode-level result of the design, because the floating body couples all of it. Exact reuse after a dynamics-affecting edit is at most about a tenth of compute (FACT about published loops; HYPOTHESIS for this system until its phase ledger exists) | The invalidation contract is hygiene and an evidence rule; INV-PHYLUM-04 | Input: a mass edit on a robot in contact, with the assertion that zero rollouts are reused | BD-286 |
| CX-160 | Padding a mixed batch to its largest robot costs about the mean-to-maximum ratio | 89% of designs at 18 degrees of freedom, 10% at 35 and 1% at 66: r = 0.31, and the overhead is 3.27 on linear stages, 9.59 on quadratic ones and 23.3 on cubic ones. One large hybrid in a hundred designs makes a single padded batch cost three to twenty-three times the unpadded work | 1/r ≤ Ω_p ≤ r⁻ᵖ (PO-PHYLUM-02); size buckets with a cap of eight; `FROB-REF-741` | Input: the three-size population given to the planner with one bucket forced | BD-171; N17b-15 |
| CX-161 | Finer buckets always pay | Worked card (90% at 18, 10% at 35; unpadded world-step τ = 0.5 µs; launch overhead c_L = 0.45 ms per plan; blended Ω = 3.5 in one bucket and 1 in two): at 8,192 worlds two buckets cost 5.0 ms against 14.8 ms; at 512 worlds 1.16 ms against 1.35 ms; at 128 worlds the single padded bucket wins, 0.67 ms against 0.96 ms. The crossover is N = c_L/(τ·(Ω − Ω_b)) = 360 worlds | The bucket rule N·τ·(Ω − Ω_b) > (B − 1)·c_L; the planner's dynamic program; `phylum.plan` | Mutation: a planner that always splits to the bucket cap | BD-171 |
| CX-162 | Mixing many designs in one batch is a large speedup at any batch shape | Against D = 512 designs run one after another with W worlds each, the per-step ratio D·(c_L + W·τ)/(c_L + Ω·D·W·τ) at Ω = 1.3, with the card figures of CX-161 (τ = 0.5 µs, c_L = 0.45 ms), is about 40 at W = 16, 11 at W = 64, 3.5 at W = 256, 1.4 at W = 1,024 and 1.1 at W = 2,048. The benefit is occupancy at few worlds per design and vanishes near a thousand worlds per design; a comparator that batches parameter variants, compiles in parallel or keeps one template per family closes most of the gap | The ceiling stated before any speed test; the four arms of the head-to-head; GATE-PHYLUM-03; KILL-PHYLUM | Input: a speed claim submitted against the sequential comparator only, which the ceiling check must refuse to publish as the headline | BD-186 |
| CX-163 | A pruning ratio and a kernel speedup multiply; improving most of the work gives a large factor | speedup ≤ 1/(1 − f + f/r + h) for a fraction f of matched wall time improved by a factor r with relative overhead h. Improving 85% of the work without limit cannot reach ten (the bound is 6.7); with f = 0.97, r = 40 and h = 0.02 the bound is about 13.5. 100 candidates of 10 s each, 90 pruned, the ten survivors twice as fast: 1,000/50 = 20, not 10 × 5 = 50. For the family lane f = 0.15 to 0.35 and r at most 4 give 1.10 to 1.32 | The speedup bound with "matched" work on the strongest baseline's critical path; KILL-FAMILYLANE | Input: a registered speed comparison that multiplies a pruning ratio by a speedup measured on the full set | BD-356 |
| CX-164 | A kernel specialized to one design gives a large step speedup | In a published humanoid step trace everything a design-specialized recursion could touch sums to 53 of 262 ns, 20% of the step. An infinitely fast specialized recursion gives 262/209 = 1.25 and one twice as fast gives 1.11; on a manipulation scene the share is 16% and the ceiling 1.19 | Specialization is funded only if the M2 profile shows a specializable share of at least one third; KILL-SPECIALIZE (gain below 1.2 or compile above 60 s) | Input: a funding request for specialization with a measured share below one third | BD-371; N17b-28 for the limb-archetype level |
| CX-165 | A finite-difference design gradient can be taken of any metric over any horizon | A binary outcome has zero gradient almost everywhere and a finite-difference variance that grows as 1/δ. In contact-rich simulation a perturbation of 5 × 10⁻⁴ degree has been reported to diverge after 50 steps (FACT about a published source); after the active contact sets of a pair differ, its difference is noise of variance about σ²/(2δ²) | `FROB-REF-540`; the horizon cap of 50 control intervals; truncation at the divergence step with the label `truncated-horizon`; INV-PHYLUM-07; KILL-FDGRAD | Inputs: a binary success metric; a bouncing body near grazing. Mutation: an estimator that averages the full horizon | N17b-22; BD-353 |
| CX-166 | Splitting designs into training and held-out sets by seed keeps the test set unseen | A mirrored copy of a training design is the same closed-loop system; split by seed alone it can appear in the test set (the chapter gives no numeric case) | `DesignSplit` keeps every orbit of the observation-preserving group on one side; GATE-PHYLUM-07 (no orbit straddles a split) | Mutation: a split by seed only, with a mirrored pair generated across the boundary | BD-287; N17b-15 |

**Kind, plant and label per entry (DESIGN).** `plant` is `<planting bead>:cx-<nnn>` (or `<planting bead>:<slug>`
for an appended entry, and a suffix where one entry needs several plants): the Cargo feature (for a `mutation`) or
the `synthetic` fixture (for the other kinds) under which the planting bead builds the shortcut in the real
component; the planting bead may refine the name, keeping the number or slug in it. A planted input whose
expectation would otherwise be the product's own refusal carries `removed_check`, the check disabled in the build
that carries it (BD-025's rule 5: without it the control is rejected `boundary-not-control`).

| Entry | `control_kind` | `plant` (the planted shortcut) | `label` |
|---|---|---|---|
| CX-153 | planted-input | `N17a-23:cx-153` (a hexapod with a front-only lidar quotiented under Γ_mech; `removed_check`: the Γ_obs check of the requested group, so the request is answered instead of refused with 161; the orbit-pair replay of GATE-PHYLUM-01 must find a merged pair whose replays differ) | DESIGN |
| CX-154 | mutation | `N17a-26:cx-154` (a digest that omits the second moment: the `cfg(feature = "cx-154")` branch of N17a-26's digest in `frob-phylum`, built as `--features gauntlet-mutations,frob-phylum/cx-154`) | DESIGN |
| CX-155 | mutation | `BD-286:cx-155` (a reuse key built from summed peak demands with no time-resolved profile, which prunes the feasible two-tick design) | DESIGN |
| CX-156 | mutation | `N17a-24:cx-156` (per-address results served across an orbit with the identity address map) | DESIGN |
| CX-157 | mutation | `N17a-26:cx-157` (a kernel that reads one field outside the digested tables, a label hash) | DESIGN |
| CX-158 | mutation | `N17b-13:cx-158` (a node kind whose key omits one declared input: the four-state fixture's key without packet delivery time, which keeps a stale result at the fourth state) | FACT: an executed planning test (the same four-state case as CX-315); `rerun = false` until N17b-13's fixture reproduces it in this repository |
| CX-159 | planted-input | `BD-286:cx-159` (a mass edit on a robot in contact, with the assertion that zero rollouts are reused) | DESIGN (the case's "at most about a tenth of compute" is FACT about published loops, Appendix H, and no plant's expectation uses it) |
| CX-160 | planted-input | `N17a-05:cx-160` (the three-size population given to the planner with one bucket forced, `--max-buckets 1`: the padded cost must be reported with Ω₁ = 3.27, Ω₂ = 9.59, Ω₃ = 23.3, never as the mean-to-maximum ratio) | DESIGN |
| CX-161 | mutation | `N17a-05:cx-161` (a planner that always splits to the bucket cap) | DESIGN |
| CX-162 | planted-input | `BD-186:cx-162` (a speed claim against the sequential comparator only; `removed_check`: the ceiling check of the head-to-head report, so the claim reaches the headline; the report check of GATE-PHYLUM-03 must flag a headline without arms C and D) | DESIGN |
| CX-163 | planted-input | `N17a-11:cx-163` (a registered speed comparison that multiplies a pruning ratio by a speedup measured on the full set, 10 × 5 = 50; `removed_check`: the matched-work rule of the bound calculator, so the product is accepted; the pilot's check of the bound 1/(1 − f + f/r + h) must report 20 from matched wall times, 1,000 s/50 s) | DESIGN |
| CX-164 | planted-input | `N21c-16:cx-164` (a funding request for specialization with a measured share below one third; `removed_check`: the s ≥ 1/3 clause of the funding rule, so the request reaches the decision; KILL-SPECIALIZE's reading must not fund) and `N17b-28:cx-164-limb` (the same request at the limb-archetype level, read by KILL-LIMBKERNEL's decision) | DESIGN (the 53 of 262 ns trace is FACT, KA-374; the plants' expectations rest on the one-third rule, not on the trace) |
| CX-165 | planted-input (the default; the other two plants state their own kinds) | `N17b-22:cx-165-binary` (a binary success metric; `removed_check`: the smoothness check behind 540's `pointer: metric`, so the estimate is returned; its variance must grow as 1/δ), `N17b-22:cx-165-grazing` (planted-world: a bouncing body near grazing, each pair flagged and truncated at its divergence step), `N17b-22:cx-165-full-horizon` (mutation: an estimator that averages the full horizon past divergence) | DESIGN (the reported divergence after 50 steps at 5 × 10⁻⁴ degree is FACT about a published source and no plant's expectation uses it) |
| CX-166 | mutation | `N17a-19:cx-166` (a split by seed only, the test hook that assigns sides by seed parity, with the mirror pair generated across the boundary) | DESIGN |
| `limb-packet-standing` | mutation | `N17b-27:limb-packet-standing` (a base assembly path that skips the constraint rows when every limb is an archetype instance) | DESIGN |
| `limb-packet-singular` | planted-input | `N17b-27:limb-packet-singular` (the packet diag(0, 1, 1, 1, 1, 1) handed to a routine that inverts at the port; `removed_check`: the zero-pivot check of the packet's checked factorization, so an inverse is formed; clause 5 of GATE-PHYLUM-09 must catch it) | DESIGN |

A FACT entry's control is written by the seeding tool as an `obligation` until `rerun = true` with a receipt.

**Two cases stated for limb-archetype kernels, appended to the catalogue.** They are appended to the catalogue,
which is append-only. Their numbers come from the allocation ledger
`crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`, bead N38a-40), never from
"the next free number" at commit time: this bead files one `[[block]]` request with `registry = "cx"`,
`count = 2`, `owner = "N17-CX"`, `section = "§17.21"`, the allocator fills `first` and `last`, and the two working
names below are mapped to the block's numbers in table order in one commit. Until then the working name of the
first column is the row's name; after mapping it stays in the catalogue's `slug` field. A working name still used
in a registry, a test name or a derivation file at this bead's milestone (M2) fails the ledger check
`provisional-tag-past-due`; the plant ids above keep the name by the ledger's rule for plant ids.

| Working name | The tempting shortcut | The case | The rule it protects | Planted as | Caught by |
|---|---|---|---|---|---|
| limb-packet-standing | Close the base equation from limb port packets and skip the contact rows | A point mass m on a massless leg stands on a floor under gravity g. As a free tree limb its packet reports the body's inertia and a bias equal to the weight m·g, and the base equation closed from the packet gives a downward acceleration g. With the contact row the normal force is m·g and the acceleration is zero. The shortcut is wrong by exactly g | Contact stages run for the whole robot; an archetype kernel only computes terms the coupled solve needs; GATE-PHYLUM-09 (hand control 1) | Mutation: a base assembly path that skips the constraint rows when every limb is an archetype instance | N17b-27 |
| limb-packet-singular | A limb port packet can be inverted at the port | The packet with value diag(0, 1, 1, 1, 1, 1) (the result of eliminating a joint with axis s, the first basis vector, from the identity inertia: I − I·s·(sᵀI·s)⁻¹·sᵀI). It is symmetric positive semidefinite, so it is accepted as a packet, and it is singular, so it is refused as an invertible matrix | The semidefinite packet type; GATE-PHYLUM-09 (hand control 2). Same fixture as CX-129 and CX-024 of other chapters | Input: the packet handed to any routine that inverts at the port | N17b-27 |

**Cross-references to counterexamples other chapters own and Phylum's gates also use.** CX-146 (a reduction over a
padded axis grouped by buffer capacity: in `f32` with a = 10⁸, b = −10⁸, c = 1, (a + b) + c = 1 and
a + (b + c) = 0) is the negative control of GATE-PHYLUM-02. CX-058 and CX-059 are the construction-level statements
of CX-153 and CX-154. CX-222 and CX-224 restate CX-160 and CX-164 from the device side.

## Acceptance criteria

- [ ] The fourteen catalogue rows (CX-153 to CX-166) and the two appended rows exist in `crates/frob-conformance/registry/counterexamples.toml` in N38b-05's record schema, each with the `control_kind`, `plant` and `label` of the table above and the gates it must fail among its `rule_ids`; CX-166 carries `plan_case = "words"`, `numeric = true` and a non-empty `case_supplied_by`, every other numbered row `plan_case = "numbers"`; CX-158 carries `rerun = false` until its re-run; the catalogue's loader accepts the file and its Phylum count is 14.
- [ ] The two appended rows carry numbers from a block issued to N17-CX in `allocations.toml`, in table order, with their working names in `slug`; `registry::alloc::validate` passes and no working name remains outside `slug` and the plant ids.
- [ ] After the seeding tool of N38b-05 is re-run, each plant of the sixteen entries has exactly one control in `controls.toml` whose `seed` names its entry (a mutation's `feature` the name after the colon of its plant id), the tool's second run changes no byte, and the join passes; this bead writes no row of `controls.toml` itself.
- [ ] Every entry resolves to at least one gate after seeding, directly or through `rule_gates.toml`; an entry that resolves to none is in the seeding tool's findings file with `gate-added` requested and is reported, never dropped.
- [ ] The arithmetic of every row is reproduced by a host test (exact integers and rationals where the row is exact).
- [ ] `scripts/e2e/phylum_negative_controls.sh` exits 0. Running each plant against its gate (fails with the plant built, passes without) is the control runner's (N38b-04), invoked by the planting beads' own scripts; until a planting bead has built its plant the control is reported `stale` and its gate cannot pass, which this bead's script reports and does not wait on.

## Test plan

### Unit tests

- `cx153_orbit_counts`: Burnside counts 86,185 and 500,500 from the cycle structures (identity: 6 cycles; two rotations by one step: 1 cycle; two by two steps: 2; one by three steps: 3; three reflections through opposite slots: 4; three through opposite edges: 3).
- `cx154_ballast_classes`: enumerates C(24, 8) = 735,471 placements and counts 24,546 distinct (Σx, Σx²) pairs; asserts {1, 4} and {2, 3} with six common slugs differ in Σx² (17 against 13) and agree in Σx (5).
- `cx155_bus_schedules`: enumerates schedules of two unit draws under a unit limit: 0 feasible in one tick, 2 in two ticks.
- `cx160_padding_three_sizes`: Ω₁ = 3.27, Ω₂ = 9.59, Ω₃ = 23.3 and r = 0.31 for the 89/10/1 population, to three figures from exact rationals.
- `cx161_bucket_crossover`: 14.8, 5.0, 1.35, 1.16, 0.67, 0.96 ms and a crossover of 360 worlds.
- `cx162_mixing_ratio`: 40.6, 11.3, 3.46, 1.44, 1.11 for W = 16, 64, 256, 1,024, 2,048.
- `cx163_speedup_bound`: 6.67, 13.5 and the 20-not-50 case.
- `cx164_specialization_ceiling`: 262/209 = 1.2536; 262/(209 + 26.5) = 1.1125; 1/(1 − 0.16) = 1.19.
- `cx165_variance_scalings` (synthetic, exact from the normal distribution function Φ): for the binary metric J = 1[θ + ξ > 0] at θ = 0 with ξ ~ N(0, 1) shared by the two worlds of a pair, D = (J₊ − J₋)/(2δ) equals 1/(2δ) with probability p = 2Φ(δ) − 1 and 0 otherwise, so Var D = p(1 − p)/(4δ²); δ·Var D is 0.1833 at δ = 0.1, 0.1979 at 0.01 and 0.1993 at 0.001, tending to φ(0)/2 = 0.1995 (the variance grows as 1/δ). After divergence, two independent worlds of variance σ² = 1 give Var D = σ²/(2δ²) = 50 at δ = 0.1.
- `limb_packet_standing_accel`: m = 2 kg, g = 9.81 m/s²: the base equation closed from the packet alone gives an acceleration of −9.81 m/s²; with the contact row the normal force is 19.62 N and the acceleration 0.
- `limb_packet_singular_psd`: I − I·s·(sᵀI·s)⁻¹·sᵀI with I the 6 × 6 identity and s = e₁ equals diag(0, 1, 1, 1, 1, 1); its eigenvalues are {0, 1, 1, 1, 1, 1} (accepted as positive semidefinite) and a Cholesky factorization meets a zero pivot at index 0 (refused as invertible).
- `cx_phylum_kinds_plants`: every entry's `control_kind`, `plant`, `label` and `plan_case` equal the table; every
  plant id has the form `<planting bead>:<name>` with a live bead key; CX-166's `case_supplied_by` names N17a-19;
  `limb-packet-singular` carries `same_as = "CX-024"`; each planted input whose expectation is a refusal (CX-153,
  CX-162, CX-163, CX-164, the binary plant of CX-165, `limb-packet-singular`) carries a `removed_check`.

The tests live in `crates/frob-conformance/tests/phylum_counterexamples.rs` (host arithmetic, exact where
rational) and run with `cargo test -p frob-conformance --test phylum_counterexamples`.

### End-to-end script and logging

`scripts/e2e/phylum_negative_controls.sh` (helper of STD-E2E; it calls only cargo targets of its dependencies)
logs to `target/e2e-logs/phylum_negative_controls/<utc>.ndjson` with a transcript beside it. It runs `cargo test -p
frob-conformance --test phylum_counterexamples`; runs N38b-05's seeding tool, `cargo test -p frob-conformance
--test seed_controls -- --ignored seed`, twice, asserting that the second run leaves the digest of the committed
`controls.toml` unchanged; runs the join, `cargo test -p frob-conformance --test seed_join`; runs the ledger check,
`cargo test -p frob-conformance --test registry_allocations`; reads `registry/control_history.ndjson` and reports
per entry and plant its control's state (`runnable`, `obligation`, `stale` while the planting bead has not built
the plant) and last recorded outcome; then plants the registry's own negative controls below on a scratch copy.
Running each plant against its gate is the control runner's (N38b-04, `--features
gauntlet-mutations,<crate>/<feature>`, one control per build), invoked by the planting beads' own scripts, and is
not repeated here. Logs per NDJSON line, beside the helper's standard fields (command line, exit code, wall time),
the CX number or working name, the control's `NC-` identifier, the plant, the gate, the state and the last
outcome; the digest of `controls.toml` before and after the second seeding; on failure the first missing entry or
unresolved gate. `summary` lists entries, controls runnable, obligations, stale controls with their planting beads,
and findings.

### Known answers and negative controls

- Known answers these controls sit beside: KA-365, KA-366, KA-367 (CX-153, CX-154), KA-368 to KA-372 (CX-160 to CX-162), KA-373 (CX-163), KA-374 (CX-164), KA-378 and KA-377 (CX-155, CX-156), KA-380 (CX-157), KA-381 and KA-382 (CX-165), KA-385 (CX-166).
- The registry's own negative controls, each on a scratch copy: an entry whose `plant` names no planting bead, and one with an empty `control_kind` (`cx_phylum_kinds_plants`); CX-153 with its `removed_check` deleted (`cx_phylum_kinds_plants`); CX-158 written FACT without `rerun` (N38b-05's `fact-without-rerun`); an appended entry numbered outside N17-CX's block (the ledger's `unallocated-number`). A plant that does not change behaviour (a no-op feature) is reported by the control runner as `missed`, a failure of the control, not a pass (N38b-04).
- Determinism: the script's verdicts are identical run to run; the host arithmetic tests are exact and identical on Linux and macOS.

## Considerations

- A planted mutation must never be reachable in a release build: `gauntlet-mutations` under the release profile is a compile error, and a per-control feature `<crate>/cx-<nnn>` is never in the umbrella and is added only on the control runner's command line, one control per build (BD-025).
- Several rows protect mechanisms that may fall back at their kill gates (family lanes, specialization, the digest quotient, paired gradients). A control whose mechanism fell back is disposed `withdrawn`, with the kill decision as its reason, in `registry/control_reviews.toml` (N38b-04); the catalogue entry and the control's identifier stay (append-only).
- CX-158 and CX-159 rest on facts about small executed programs and published loops: CX-158's case is re-run in this repository (`rerun`) before its control is runnable; CX-159's figure is re-read at its source before a gate cites it, and its plant does not use it.
- The limb-packet cases overlap fixtures owned by the design-language and dynamics chapters (CX-129, CX-024); the fixture file is shared and the appended entry carries `same_as = "CX-024"`.

## References

Plan Appendix N (N.14: CX-153 to CX-166), §17.5, §17.6, §17.9, §17.10, §17.11, §17.13, §17.21, §38.11. Related:
CX-024, CX-058, CX-059, CX-129, CX-146, CX-222, CX-224; PO-PHYLUM-02; INV-PHYLUM-03, -04, -07.
