---
key: "N17-CX"
title: "Phylum counterexample registry: CX-153 to CX-166 as negative controls, with the limb-packet standing case"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "phylum", "frob-phylum", "frob-conformance", "solid", "registry", "test"]
parent: "WP-44"
deps: ["BD-025", "STD-TEST", "STD-E2E"]
needs: ["§38.11: assignment of catalogue numbers to the two counterexamples of the limb-archetype kernels that the chapter states and the catalogue does not yet list (the catalogue is append-only and its owner assigns numbers)"]
related: ["BD-170", "BD-171", "BD-186", "BD-285", "BD-286", "BD-287", "BD-353", "BD-356", "BD-371", "N17b-13", "N17b-22", "N17b-27"]
gates: []
plan_refs: ["Appendix N (N.14)", "§17.5", "§17.6", "§17.9", "§17.10", "§17.11", "§17.13", "§17.21", "§38.11"]
---
## Background

Phylum (crate `frob-phylum`) batches populations of different robot designs and removes duplicate work from design
campaigns. Almost every mechanism in it has a plausible shortcut that is wrong, and the project's rule is that a
shortcut ships with the smallest hand-checkable case that breaks it. Each such case is a counterexample with a
stable number, and each is the seed of at least one record in the negative-control registry
(`frob-conformance/registry/controls.toml`): the shortcut is planted as a code mutation or as an input, and the gate
of the rule it protects must catch it. The reason is stated in the plan as "a gate that nothing can fail validates
nothing": a gate records PASS only together with a caught negative control.

This bead is the registry of the fourteen counterexamples of the Phylum chapter, CX-153 to CX-166, with the exact
numbers of each case, the rule it protects, the form in which it is planted, and the bead whose test must catch it.
It also carries two cases that the chapter states for limb-archetype kernels and that have no catalogue number yet.

Terms used below. Γ_mech is the mechanical symmetry group of a robot body; Γ_obs is its subgroup that also preserves
every sensor pose and field of view, the payload, the task's goal orientation and the terrain's symmetry. A mission
digest is a hash of exactly what the step plan and the task read from a design. Ω_p is the padding overhead of a
batch padded to its largest member for work that grows as the p-th power of robot size L: Ω_p = L_maxᵖ / E[Lᵖ],
with r = E[L]/L_max.

## Scope

In: the fourteen registry records and the two unnumbered ones; for each, a planted control (mutation or input) wired
to a named test target; the check that every record is exercised by at least one gate run.
Out: the mechanisms and their own tests (the catching bead is named per row). Where a row names a seed bead, the
planted control is wired to the test target of the component bead that carries the mechanism, and the seed bead
is the capability it belongs to: CX-153 in N17a-23 (orbit representatives) with N17a-19 (splits); CX-154 and
CX-157 in N17a-26 (the mission digest), measured by N17a-27 and N17a-28; CX-155 in the replay audit N17a-28;
CX-156 in N17a-24 (result transport); CX-158 and CX-159 also in N17a-26 (memo entries of the digest); CX-160
and CX-161 in N17a-04 and N17a-05 (bucket arithmetic and planner), asserted from the user's side by N17a-08;
CX-162 also in N17a-04; CX-163 in N17a-11 and N17a-13 (the family-lane pilot and measurement); CX-164 in N17a-15
and N17a-16 (survivor specialization) and in N17b-24 and N17b-27 for the limb-archetype level; CX-166 in N17a-19,
exercised by N17a-21. The known answers of the chapter are
the registry `N17-KA` (written by the unit that owns the first half of the chapter).

## Technical approach

**Record format (DESIGN).** One record per counterexample:
`{ cx, shortcut, planted_as: "mutation" | "input", plant: <feature flag or fixture path>, catches: [<gate or
invariant>], test: <cargo test target or e2e script>, expected: "caught" }`. A mutation is compiled in only under
the test feature `planted-defects` of the crate under test; an input is a fixture file marked `synthetic`. A record
whose test passes with the plant in place is a failed control and blocks the gate.

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
| CX-162 | Mixing many designs in one batch is a large speedup at any batch shape | Against D = 512 designs run one after another with W worlds each, the per-step ratio D·(c_L + W·τ)/(c_L + Ω·D·W·τ) at Ω = 1.3 is about 40 at W = 16, 11 at W = 64, 3.5 at W = 256, 1.4 at W = 1,024 and 1.1 at W = 2,048. The benefit is occupancy at few worlds per design and vanishes near a thousand worlds per design; a comparator that batches parameter variants, compiles in parallel or keeps one template per family closes most of the gap | The ceiling stated before any speed test; the four arms of the head-to-head; GATE-PHYLUM-03; KILL-PHYLUM | Input: a speed claim submitted against the sequential comparator only, which the ceiling check must refuse to publish as the headline | BD-186 |
| CX-163 | A pruning ratio and a kernel speedup multiply; improving most of the work gives a large factor | speedup ≤ 1/(1 − f + f/r + h) for a fraction f of matched wall time improved by a factor r with relative overhead h. Improving 85% of the work without limit cannot reach ten (the bound is 6.7); with f = 0.97, r = 40 and h = 0.02 the bound is about 13.5. 100 candidates of 10 s each, 90 pruned, the ten survivors twice as fast: 1,000/50 = 20, not 10 × 5 = 50. For the family lane f = 0.15 to 0.35 and r at most 4 give 1.10 to 1.32 | The speedup bound with "matched" work on the strongest baseline's critical path; KILL-FAMILYLANE | Input: a registered speed comparison that multiplies a pruning ratio by a speedup measured on the full set | BD-356 |
| CX-164 | A kernel specialized to one design gives a large step speedup | In a published humanoid step trace everything a design-specialized recursion could touch sums to 53 of 262 ns, 20% of the step. An infinitely fast specialized recursion gives 262/209 = 1.25 and one twice as fast gives 1.11; on a manipulation scene the share is 16% and the ceiling 1.19 | Specialization is funded only if the M2 profile shows a specializable share of at least one third; KILL-SPECIALIZE (gain below 1.2 or compile above 60 s) | Input: a funding request for specialization with a measured share below one third | BD-371; N17b-28 for the limb-archetype level |
| CX-165 | A finite-difference design gradient can be taken of any metric over any horizon | A binary outcome has zero gradient almost everywhere and a finite-difference variance that grows as 1/δ. In contact-rich simulation a perturbation of 5 × 10⁻⁴ degree has been reported to diverge after 50 steps (FACT about a published source); after the active contact sets of a pair differ, its difference is noise of variance about σ²/(2δ²) | `FROB-REF-540`; the horizon cap of 50 control intervals; truncation at the divergence step with the label `truncated-horizon`; INV-PHYLUM-07; KILL-FDGRAD | Inputs: a binary success metric; a bouncing body near grazing. Mutation: an estimator that averages the full horizon | N17b-22; BD-353 |
| CX-166 | Splitting designs into training and held-out sets by seed keeps the test set unseen | A mirrored copy of a training design is the same closed-loop system; split by seed alone it can appear in the test set (the chapter gives no numeric case) | `DesignSplit` keeps every orbit of the observation-preserving group on one side; GATE-PHYLUM-07 (no orbit straddles a split) | Mutation: a split by seed only, with a mirrored pair generated across the boundary | BD-287; N17b-15 |

**Two cases stated for limb-archetype kernels, not yet numbered.**

| Working name | The tempting shortcut | The case | The rule it protects | Planted as | Caught by |
|---|---|---|---|---|---|
| limb-packet-standing | Close the base equation from limb port packets and skip the contact rows | A point mass m on a massless leg stands on a floor under gravity g. As a free tree limb its packet reports the body's inertia and a bias equal to the weight m·g, and the base equation closed from the packet gives a downward acceleration g. With the contact row the normal force is m·g and the acceleration is zero. The shortcut is wrong by exactly g | Contact stages run for the whole robot; an archetype kernel only computes terms the coupled solve needs; GATE-PHYLUM-09 (hand control 1) | Mutation: a base assembly path that skips the constraint rows when every limb is an archetype instance | N17b-27 |
| limb-packet-singular | A limb port packet can be inverted at the port | The packet with value diag(0, 1, 1, 1, 1, 1) (the result of eliminating a joint with axis s, the first basis vector, from the identity inertia: I − I·s·(sᵀI·s)⁻¹·sᵀI). It is symmetric positive semidefinite, so it is accepted as a packet, and it is singular, so it is refused as an invertible matrix | The semidefinite packet type; GATE-PHYLUM-09 (hand control 2). Same fixture as CX-129 and CX-024 of other chapters | Input: the packet handed to any routine that inverts at the port | N17b-27 |

**Cross-references to counterexamples other chapters own and Phylum's gates also use.** CX-146 (a reduction over a
padded axis grouped by buffer capacity: in `f32` with a = 10⁸, b = −10⁸, c = 1, (a + b) + c = 1 and
a + (b + c) = 0) is the negative control of GATE-PHYLUM-02. CX-058 and CX-059 are the construction-level statements
of CX-153 and CX-154. CX-222 and CX-224 restate CX-160 and CX-164 from the device side.

## Acceptance criteria

- [ ] `frob-conformance/registry/controls.toml` holds one record per row above (fourteen numbered, two with working names), each with its plant and its test target.
- [ ] Every plant exists: each mutation behind the `planted-defects` feature of its crate, each input as a fixture marked `synthetic` with the numbers of the table.
- [ ] For every record, the named test fails with the plant active and passes without it; both outcomes are logged.
- [ ] No Phylum gate can be recorded PASS unless the controls that name it were caught in the same run; the conformance harness enforces this by reading the registry.
- [ ] The two unnumbered cases are filed with the catalogue owner for numbers; until then they are referenced by working name and are not silently dropped.
- [ ] The arithmetic of every row is reproduced by a host test (exact integers and rationals where the row is exact).

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
- `controls_registry_complete`: every row of this bead has a record, and every record names a test target that exists.

### End-to-end script and logging

`scripts/e2e/phylum_negative_controls.sh` sources the helper of `STD-E2E`, logs to
`target/e2e-logs/phylum_negative_controls/<utc>.ndjson`, and for each record runs its test twice: with the plant
(`cargo test -p <crate> --features planted-defects --test <target> -- <cx>`, expecting failure with the first
differing value logged) and without (expecting success). Each run is a `cmd` event with the command line, exit code
and wall time; each expectation is an `assert` event naming the CX; the `summary` line reports controls caught,
controls missed and the gates each control serves.

### Known answers and negative controls

- Known answers these controls sit beside: KA-365, KA-366, KA-367 (CX-153, CX-154), KA-368 to KA-372 (CX-160 to CX-162), KA-373 (CX-163), KA-374 (CX-164), KA-377 and KA-378 (CX-155, CX-156), KA-381 and KA-382 (CX-165), KA-385 (CX-166).
- The registry's own negative control: a record whose plant is a no-op (a mutation that changes nothing) must be reported as `missed`, not as `caught`; one such dummy record is kept in the test fixtures of the harness, never in the live registry.
- Determinism: the script's verdicts are identical run to run; the host arithmetic tests are exact and identical on Linux and macOS.

## Considerations

- A planted mutation must never be reachable in a release build: the `planted-defects` feature is rejected by the release profile check.
- Several rows protect mechanisms that may fall back at their kill gates (family lanes, specialization, the digest quotient, paired gradients). A control whose mechanism fell back is retired with a tombstone that names the kill decision; it is not deleted.
- CX-158 and CX-159 are facts about small executed programs and published loops; they are re-run or re-read before a gate cites the numbers.
- The limb-packet cases overlap fixtures owned by the design-language chapter (CX-129); the fixture file is shared and the record here points at it.

## References

Plan Appendix N (N.14: CX-153 to CX-166), §17.5, §17.6, §17.9, §17.10, §17.11, §17.13, §17.21, §38.11. Related:
CX-024, CX-058, CX-059, CX-129, CX-146, CX-222, CX-224; PO-PHYLUM-02; INV-PHYLUM-03, -04, -07.
