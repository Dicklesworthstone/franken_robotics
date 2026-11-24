---
key: "N25-CX"
title: "Augur counterexample registry: CX-270 to CX-287, and the eleven late counterexamples awaiting numbers"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "augur", "frob-conformance", "solid", "registry", "test"]
parent: "WP-42"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N25-KA", "N25-REF", "N25a-08", "N25a-11", "N25d-10", "N25d-18", "N25d-19", "N25d-28", "N25d-29", "N25d-37", "N25d-38", "N25d-39", "N25d-41", "N25e-12", "N25e-30", "N25e-36", "N25e-41", "N25e-44", "N25e-45"]
gates: []
plan_refs: ["Appendix N.22", "§25.4.7", "§25.22", "§25.29", "§25.30", "§25.31", "§25.32", "§25.33", "§25.34", "§25.35", "§25.36", "§25.37", "§25.38", "§25.39", "§0.7"]
---
## Background

Counterexamples are first-class in this program: where a plausible shortcut is wrong, the smallest hand-checkable
case that breaks it is written down, and the case enters the negative-control registry and the counterexample
catalogue. Each seeds a negative control: a test in which the shortcut is planted and must be caught. A rule that
ships without such a case has not been thought through.

Augur, the subsystem of mission queries and inverse design, answers questions about sets of designs with an inner
set and an outer set and a checked reason per cell. Its shortcuts are ways of making a family answer say more than
was proved: a convex hull through two witnesses, a per-design command read as a family command, a failed sample
read as an excluded cell, a marginal rate multiplied, a probe trusted outside its mechanism set. The catalogue
gives Augur eighteen numbered entries, CX-270 to CX-287. Eleven further counterexamples are stated by the rules
added to the chapter later; they have no catalogue number yet and are registered here under provisional tags so
that none is lost. Tier Solid. Mechanism beads cite the numbers; this bead holds the rows as data and binds each to
the control test that realizes it.

## Scope

In: the eighteen Augur entries in the suite's one catalogue, `crates/frob-conformance/registry/counterexamples.toml`
(some documents call the chapter's part `registries/counterexamples/augur.toml`; that is a logical alias, not a
second file); the eleven late entries in the same file under provisional tags `augur-cx-late-01` to
`augur-cx-late-11` until the allocator issues their numbers (the `augur-cx-` prefix keeps the tags unique: other
chapters' late entries carry working names of the form `late-nn`, for example N27-CX's `late-01` and `late-02`,
and N25-KA's late known-answer rows carry `augur-late-01` to `augur-late-18`, which N38a-40's stale-tag check
searches in the same test names and derivation files; the tables below write the tags without the prefix for
brevity); the request for that block; the binding of each entry to the planted defect and
the test that catches it; the registry check. Out: the catalogue schema, the seeding of `controls.toml` and the
join (N38b-05: this bead writes catalogue entries and never a row of `controls.toml`); the allocation ledger and
its rules (N38a-40); the control tests' implementations (in the beads named); other chapters' counterexamples.

## Technical approach

Entry schema: the catalogue record of N38b-05: `id` (CX-nnn, append-only), `shortcut`, `case` (with its numbers),
`numeric` (true for every Augur entry), `plan_case` (`"numbers"` for every Augur entry; immutable),
`case_supplied_by` (empty), `rule` (text), `rule_ids` (the GATE-, KILL-, INV-, PO-, ADR- and
FROB-REF- identifiers named in the rule column), `defined_in`, `same_as` (CX-010 for CX-270, CX-052 for CX-286;
empty otherwise), `label` (DESIGN, or FACT with its source for the pilot-quoted cases listed in Considerations), `rerun` and
`rerun_receipt` (on the FACT entries),
`control_kind` (`mutation`, `planted-input`, `planted-design` or `planted-world`, by N38b-05's table), `plant`
(the Cargo feature under `gauntlet-mutations`, or the `synthetic` fixture path), and `slug` (the provisional
tag of a late entry). The "Control realized by" column below names the beads whose tests carry the plant; each such
test is tagged with the entry's identifier. The registry check (`cargo test -p frob-conformance --test cx_augur`)
verifies that every entry names a plant and a catching bead, and that every control test tagged with a
counterexample identifier names an existing entry.

Milestones of the controls (DESIGN, so that a registry at M1 does not wait on mechanisms of M3 to M8): the
catalogue has no state field. An entry whose plant has not been built yet seeds a control the control runner
reports `stale` (an unbuilt plant is not a load error, BD-025); the check lists it as pending with its catching
bead, derived from the plant's carrier and the control's state, not from the tracker, so no list is kept by hand
and no status is read from `br`. A FACT entry with `rerun = false` is an `obligation`, written so by the seeding
tool. An entry becomes runnable when its plant is built; at each milestone exit no entry whose catching bead
belongs to that milestone may remain pending (the suspect report of N38b-04, GATE-GAUNT-01). By the catching beads' milestones: M0 or M1 for CX-270,
CX-272 to CX-276, CX-278, CX-283 and late-08; M3 for CX-279, CX-282 (BD-292), CX-286, CX-287, late-02 to late-04
and late-09; M4 for CX-271, CX-277, CX-280, CX-284 (BD-355, whose own run is the entry's receipt), CX-285, late-01
(finite part), late-05 to late-07, late-10 and late-11; M6 for CX-281.

**Kind, plant and label per entry.** `plant` is `<planting bead>:<name>`: the Cargo feature under
`gauntlet-mutations` (for a `mutation`) or the `synthetic` fixture (for the other kinds) under which that bead
builds the shortcut; it may refine the name, keeping the number or tag in it. For CX-270 to CX-281 the first
plant is N25a-08's `shortcut_nn` evaluator (row nn of its table, `frob_conformance::fixtures::shortcuts`), whose
self-test `assert_ne!(truth, shortcut)` is the control's own check; the mechanism beads of the "Control realized
by" column carry the shortcut into the real component. Gates in `rule_ids`: GATE-AUGUR-01 for CX-270 to CX-281
(N25a-08's battery), with the owning mechanism's gate as N25a-08 assigns it: GATE-AUGUR-02 for CX-270, CX-273 and
CX-274; GATE-AUGUR-19 for CX-271; GATE-AUGUR-04 for CX-272; GATE-AUGUR-16 for CX-277; GATE-AUGUR-17 for CX-278
and CX-280; GATE-AUGUR-11 for CX-279; GATE-AUGUR-20 for CX-281. Every other entry gives the identifiers of its
rule column and its defining section, which the seeding tool resolves through `registry/rule_gates.toml`; an entry
that resolves to no gate is written to `registry/seed_findings.toml` with `gate-added` requested.

| Entry | `control_kind` | `plant` | `label` |
|---|---|---|---|
| CX-270 | planted-design | `N25a-08:shortcut_01` (midpoint interpolation of feasibility) | DESIGN |
| CX-271 | planted-design | `N25a-08:shortcut_02` (pairwise compatibility) | DESIGN |
| CX-272 | planted-input | `N25a-08:shortcut_03` (per-build command sets intersected late) | DESIGN |
| CX-273 | planted-design | `N25a-08:shortcut_04` (the active allocation credited to a passive body) | DESIGN |
| CX-274 | planted-design | `N25a-08:shortcut_05` (the polygon-plus-capacity screen) | FACT: the stance-grid count (1,596 of 2,304); `rerun = false` until N25a-08's exact recount is recorded as its receipt |
| CX-275 | planted-design | `N25a-08:shortcut_06` (failing samples taken as exclusion) | DESIGN |
| CX-276 | mutation | `N25a-11:cx-276` (stage rates multiplied), with `N25a-08:shortcut_07` as fixture | DESIGN |
| CX-277 | planted-design | `N25a-08:shortcut_08` (projections enclosed separately) | DESIGN |
| CX-278 | planted-design | `N25a-08:shortcut_09` (the additive peak rule) | DESIGN |
| CX-279 | planted-world | `N25a-08:shortcut_10` (the withheld mechanism D) | DESIGN |
| CX-280 | planted-design | `N25a-08:shortcut_11` (a root without its circuit) | DESIGN |
| CX-281 | planted-input | `N25a-08:shortcut_12` (a bracket direction taken as reachable) | DESIGN |
| CX-282 | planted-world | `BD-292:cx-282` (the third docking gain 33/32 added to a certified support) | DESIGN |
| CX-283 | mutation | `{ id = "BD-043:cx-283", kind = "mutation", target = "BD-043's sill comparison (crates/frob-conformance/tests/pilots_rerun.rs: the 156 pairs of static verdict and dynamic outcome)", defect = "every statically rejected case's dynamic outcome replaced by fail (a static rejection taken as final)", expect = "7 pass both, 149 fail both, 156 agree, 0 static rejections succeeding dynamically; the sill row reports differs with first differing value fail both: expected 87, observed 149", gates = ["GATE-GAUNT-08"], feature = "cx-283" }` | FACT: `PILOT-PT-16` (the planar sill study, 62 of 156) |
| CX-284 | mutation | `BD-355:cx-284` (strict predicates in unguarded single precision) | FACT: the 65,536-row docking screen; `rerun = false` until BD-355 re-runs it |
| CX-285 | planted-input | `BD-354:cx-285` (a speed test requested before its ceiling) | FACT: the pusher timings; `rerun = false` until BD-354 re-runs them |
| CX-286 | planted-world | `BD-290:cx-286` (repeated reads that share one fault bit) | DESIGN |
| CX-287 | mutation | `BD-292:cx-287` (a memo key that omits one dependency) | FACT: the four-edit-state replay; `rerun = false` until BD-292 re-runs it |
| late-01 | planted-input | `N25e-01:cx-<nnn>` (a chance claim transported without a coupling budget) | DESIGN |
| late-02 | mutation | `N25e-07:cx-<nnn>` (monotone fill above the first success) | FACT: the sill fixture's outcomes at μ = 0.500, 0.525, 0.550; `rerun = false` until N25e-07 re-runs the fixture |
| late-03 | mutation | `N25e-11:cx-<nnn>` (the term-wise screen as a decision) | DESIGN |
| late-04 | mutation | `N25e-10:cx-<nnn>` (a box screen in place of the monomial screen) | DESIGN |
| late-05 | mutation | `N25e-13:cx-<nnn>` (a relaxation without approach energy) | DESIGN |
| late-06 | mutation | `N25e-18:cx-<nnn>` (minimum energy treated as convex in τ and N) | DESIGN |
| late-07 | mutation | `N25e-24:cx-<nnn>` (splitting continued below the weak width) | DESIGN |
| late-08 | mutation | `N25e-28:cx-<nnn>` (one command per load vertex) | DESIGN |
| late-09 | planted-world | `N25e-34:cx-<nnn>` (the pairwise-independent seven-read law (43, 3, 9, 9)/64) | DESIGN |
| late-10 | mutation | `N25e-37:cx-<nnn>` (the channel-blind label count; N25e-38 carries the independent-extremes half) | DESIGN |
| late-11 | mutation | `N25e-42:cx-<nnn>` (a small increment taken as a fixed point; N25e-43 carries the weight-sweep half) | DESIGN |

A late entry's plant `id` is `<planting bead>:cx-<nnn>`, with `<nnn>` the number the block issues to its tag, so
the Cargo feature or fixture the planting bead declares is `cx-<nnn>` (N38b-05); the tag (`augur-cx-late-nn`)
appears only in `slug`, because N38a-40's `provisional-tag-past-due` check counts a tag in a plant identifier,
feature or test name as a use. The late entries are written with their issued numbers, since the block is issued
when this bead runs the allocator and an entry's `id` is a CX number.

CX-283 is pilot-derived (N38b-05): it carries `rerun = true` with `rerun_receipt` equal to the receipt of BD-043's
sill result on `PILOT-PT-16` when `pilots::rerun_status("PILOT-PT-16")` reads `reproduced`, and `rerun = false`
only while the record reads not run or `differs`; a pair that disagrees with the register is the load error
`rerun-disagrees-with-pilot`. Its plant is written in full above, from BD-043. A FACT entry's control with
`rerun = false` is written by the seeding tool as an `obligation` (not runnable; it makes no gate eligible); it
becomes runnable when `rerun = true` with a receipt.

Numbers for the late entries: this bead files one request in N38a-40's allocation ledger
(`crates/frob-conformance/registry/allocations.toml`: `registry = "cx"`, `count = 11`, `owner = "N25-CX"`,
`section = "§25.29 to §25.39"`); when the block is issued, the tags are mapped to its numbers in table order
(late-01 to the first number) in one commit, the tests' tags follow, and a tag still present at this bead's
milestone exit fails the allocator's `provisional-tag-past-due` check.

Numbered rows:

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control realized by |
|---|---|---|---|---|
| CX-270 | The midpoint of two feasible designs is feasible | Feet at (±1 − e, ±1) m, each capped at 0.5 N, load W at the origin; the positive-x pair carries W·(1 + e)/2. (e, W) = (0, 1.9) and (0.9, 1) each need 0.95 N and pass; the midpoint (0.45, 1.45) needs 841/800 N from a pair capped at 1 N | Convexity is claimed only in a chart where the constraints are affine with design-independent mission vertices (equivalent to CX-010) | BD-130, BD-136 |
| CX-271 | Variants that are pairwise compatible can share a controller | Commands (x, y) ∈ [0, 1]²; three variants need x ≥ 2/3, y ≥ 2/3 and x + y ≤ 1. Every pair has a common command; the triple needs x + y ≥ 4/3 and x + y ≤ 1 | Minimal conflict sets, not pairs | BD-363 |
| CX-272 | A command exists for each design, so one exists for the family | Pusher y = max(0, g·u − d), g ∈ [1/2, 2], d ∈ [0, 1/4], target [3/4, 5/4]: the per-build command set is all of [1/2, 2]; the shared command set is empty | The quantifier prefix is explicit in the query and in the certificate (`FROB-REF-826`, `FROB-REF-881`) | BD-131, BD-136 |
| CX-273 | A feasible allocation exists, so the body is admitted | Stance at half-width 1/4 m: the active allocation (155, 135, 105, 5) N fits 160 N ratings; an equal-spring passive suspension produces (175, 115, 85, 25) N | The authority that realizes the allocation, and the observations that arrive in time, are part of the certificate | BD-130, N25e-28 |
| CX-274 | The support polygon contains the load and total capacity suffices | Of 2,304 bodies in the stance grid, this screen accepts 1,596 that the exact model rejects | Per-foot ratings and moment balance together | BD-130 |
| CX-275 | Sample points in a cell fail, so the cell is excluded | Any cell that straddles a boundary: the 9 boundary cells of the 561-cell docking grid | A certificate valid over the cell; otherwise the cell is unresolved | BD-129, BD-358, N25e-07 |
| CX-276 | Both stages succeed 95% of the time, so the mission succeeds 90.25% | Twenty worlds, each stage failing in one: 18/20 if the failures differ, 19/20 if they coincide | Joint law or additive budgets (`FROB-REF-846`) | N25a-11, N25a-08 |
| CX-277 | Each projection of a product covers its marginal outcomes, so the dominance check may enclose them separately | Paired outcomes (0, 0) and (1, 1) project onto {0, 1} for both robots and hide the unreachable (1, 0) | Enclose the joint outcome set | BD-359 |
| CX-278 | The per-component limits are met, so the assembly is feasible; and its converse, an additive peak rule | Two fingers each need one powered tick and the bus powers one per tick: zero one-tick schedules, two two-tick schedules. Local motor limits are identical in both cases, and an additive peak rule prunes the feasible two-tick case | The joint, time-dependent resource contract | BD-360, BD-108 |
| CX-279 | The probe separates every admitted mechanism, so its verdict is safe | The three-law probe scores 675 of 675 in class; a fourth mechanism shares the probe's early trace and reverses all 225 held-out decisions | Conditional witness; direct measurement of the target is always admitted | BD-291, N25e-39 |
| CX-280 | A root of the synthesis equations is a mechanism | Crank-rocker 1, 4, 3 on ground 4: B = (11/5, ±12/5) at crank angle π lie on two assembly circuits | Circuit and branch certificates | BD-361 |
| CX-281 | A Lie-bracket direction is reachable | ẋ = u with u ∈ [0, 1]: rank one, and negative x is unreachable from zero | The integrated finite witness | BD-425 |
| CX-282 | A certificate that held for the declared worlds holds when a world is added | Docking with gains {1, 2} admits ε < (U + 1/2)/4; adding gain 33/32 at ε = 1/64 leaves no exact docking, best worst-case error 1/65 m, and 34,590 positive labels on the grid reverse | Certificates are bound to the declared hypothesis support; revalidation polarity (`FROB-REF-854`) | BD-129, BD-292, N25e-32 |
| CX-283 | A static rejection is a rejection | Of 156 sill cases, 94 agree between sampled statics and a planar dynamic scorer and 62 static rejections succeed dynamically | A static rejection is a candidate for a dynamic test, not a verdict | BD-043, N25e-14 |
| CX-284 | Compare strict predicates in unguarded single precision | On a 65,536-row docking-boundary screen: 452 false admissions and 23,438 false exclusions on the near grid; with the guard and exact fallback, zero final errors and identical bytes on every placement | Exact-integer kernel class and guarded single precision | BD-030, BD-355 |
| CX-285 | Run the speed test, then interpret it | Pusher: composed over specialized median time 1.345 warm; a residual of 575.7 µs that the solver does not touch (parsing, assembly, checking, serialization and harness, not separately timed) bounds a zero-cost solver at about 1.46 (842.8 ÷ 575.7). The test could not have shown a large ratio | Compute the ceiling before any speed test | BD-354 |
| CX-286 | Repeated reads of a sensor always add information | Majority vote at 3/4 gives 27/32, 459/512, 3807/4096 for 3, 5, 7 independent reads, and stays at 3/4 when the reads share one fault bit | Error dependence is declared (equivalent to CX-052) | BD-290, N25e-34 |
| CX-287 | A memo keyed by a semantic receipt beats a dependency graph, or may omit an input | Four edit states: both do 3 pair evaluations and 2 reductions and match fresh replay; omitting one dependency inherits a stale promotion | Complete memo keys; no reuse claim over a correctly keyed graph | BD-292 |

Late rows (stated by the rules added after the catalogue was compiled; provisional tags until numbered):

| Tag | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control realized by |
|---|---|---|---|---|
| late-01 | A simulation relation transports a chance claim | One action, two outcomes; source succeeds with probability 0.99 and target with 0.01. The relation holds; "at least 0.95" is true of the source and false of the target; every coupling disagrees on at least 0.98 of its mass | A chance or expectation claim crosses only with a coupling budget: P_target ≥ P_source − Σ εₜ | N25e-01, N25e-02, N25e-03 |
| late-02 | Fill every parameter value above the first success | Sill fixture, 75 mm wheel, sharp lip, payload 40 mm back: success, timeout, success at μ = 0.500, 0.525, 0.550; at half the physics step: timeout, timeout, success; stiffer contact: all succeed | No monotone fill without a proof of monotonicity; non-monotone outcomes are `model-conditional` | N25e-07, N25e-08, N25e-09 |
| late-03 | Every term of a sum is at most 1, so the sum is | Two terms 0.6 and 0.6: each passes and the sum is 1.2. With 0.9 and 0.15 (sum 1.05) equal weights give 0.54 and do not exclude; weights (6/7, 1/7) give 1.05⁷ ≈ 1.407 and do | The term-wise screen is not a decision; relaxations only exclude | N25e-11, N25e-12 |
| late-04 | Boxes will find a contradiction between two sizing relations | Leg holding 117.72 N and moving at 1.5 m/s with a 2 N·m, 80 rad/s motor: each constraint alone is satisfiable over ℓ ∈ [0.1, 0.4] m, N ∈ [5, 50]; together they need ℓ/N ≤ 0.01699 and ℓ/N ≥ 0.01875. The multipliers (1, 1) give 8829/8000 > 1 | Monomial screen in exponent coordinates with an integer certificate | N25e-10, N25e-12 |
| late-05 | A relaxation that omits a resource still bounds every controller | Carrier of 10 kg, 300 J, h = 2 m, arrival at up to 1.5 m/s: the complete bound allows 16.22 kg (Ū = 0.622); without approach energy 15.29 kg (0.529), which stops the design against a competitor at 0.60, falsely: 6 kg is feasible (313.92 J ≤ 318 J) | The dominance stop needs a relaxation that encloses every declared state and energy source (`FROB-REF-551`) | N25e-13, N25e-14, N25e-15 |
| late-06 | Minimum energy is convex in torque and gear ratio | g(τ, N) = τ²/N²: g(0, 1) = 0, g(2, 3) = 4/9, midpoint g(1, 2) = 1/4 > 2/9 | Gear ratios are enumerated; a certificate excludes a motion, not a design (rear-drive cart: 4 m/s² infeasible for every battery placement, feasible with all wheels driven) | N25e-18, N25e-20 |
| late-07 | A bounded split set can decide any constraint | g = Σ_{i=1..20} xᵢ − 10 ≤ 0 on [0, 1]²⁰: with any six parameters split the weak width is 14 against a split width of 6; no cell can be decided although half the volume is feasible | Weak-variable enclosure is for weak dependence; stop when the split width falls below the weak width | N25e-23, N25e-24 |
| late-08 | One feasible command per load vertex certifies the controller | Stance family: the four vertex allocations at b = 3/14 m are not values of one affine law ((160, 240, 240, 160) N against (240, 160, 160, 240) N); an affine law needs b = 1/2 m; a constant command works for no b | The controller class is part of a family answer; the certificate carries the law | N25e-28, N25e-29, N25e-30 |
| late-09 | Pairwise independent reads are independent | Seven reads at accuracy 3/4 with K wrong reads distributed (43, 3, 9, 9)/64 on K = 1, 2, 3, 4: every pair has the independent table, and the best success is 55/64, not 3807/4096 | A sensor budget states the joint error law; redundancy is not credited under `pairwise` | N25e-34, N25e-36 |
| late-10 | A label count is enough; and independent extremes make a counterexample | Three labels need 2 bits on a clean channel, 3 under one erasure, 5 under one flip (four bits give a distance sum of at most 8 < 9). Pad margin under the delayed law: 4.72 N with q and τ shared, −0.078 N from independent extremes, which is no trial | A label is a message over a declared channel; shared nuisance parameters stay shared | N25e-35, N25e-37, N25e-38, N25e-41 |
| late-11 | A small increment is a fixed point; a weight sweep finds every nondominated build | Pack F(m) = 1 + 0.99·m: after 527 iterations 99.5 kg with a 5 g step, and F(99.5) = 99.505 > 99.5; the least feasible mass is 100 kg. Builds (0, 6), (4, 4), (6, 0): (4, 4) minimizes no weighted sum, yet a facility budget of four selects it | Sizing on a witness F(U) ≤ U; Pareto claims on dispositions | N25e-42, N25e-43, N25e-44 |

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/counterexamples.toml` holds the eighteen numbered Augur entries and the eleven late entries with every field of N38b-05's schema filled; N38b-05's loader accepts the file and counts 18 for the Augur chapter.
- [ ] Every entry carries the `control_kind`, `plant`, gates and `label` of the table above; the six FACT entries carry `rerun = false` and seed as obligations; every other entry whose plant is not yet built is reported pending with its catching bead, never as caught.
- [ ] After N38b-05's seeding tool is re-run twice, each of the 29 entries has exactly one control whose `seed` names it, the second run leaves the digest of `controls.toml` unchanged, and the join passes.
- [ ] The case and rule cells keep every number and identifier of Appendix N.22 (CX-285's untimed residual stated in full).
- [ ] The registry check passes; an entry without a plant or catching bead, or a control naming a missing entry, fails the build.
- [ ] Entries stated as equivalent to entries of other chapters (CX-270 with CX-010; CX-286 with CX-052) carry `same_as` and share one fixture.
- [ ] The request for eleven `cx` numbers is in the allocation ledger; when the block is issued the provisional tags are replaced in one commit and the tests' tags follow; no tag survives this bead's milestone exit (`provisional-tag-past-due`).
- [ ] `scripts/e2e/augur_counterexamples.sh` exits 0.

## Test plan

### Unit tests

`crates/frob-conformance/tests/cx_augur.rs`: `registry_has_18_numbered_rows`; `registry_has_11_late_rows`;
`ids_unique`; `every_row_has_control`; `controls_exist_and_are_tagged`; `schema_is_n38b05` (every entry parses
with the catalogue loader of N38b-05; a scratch entry with `equivalent_to` instead of `same_as` is rejected);
`pending_until_plant_built` (a scratch entry whose plant is not built is listed pending with its catching bead,
and a FACT entry with `rerun = true` and no receipt is refused by the loader); `kinds_plants_labels` (every entry's
`control_kind`, `plant` and `label` equal the table; CX-270 to CX-281 name N25a-08's `shortcut_nn` and
GATE-AUGUR-01; the six FACT entries carry a `rerun` field, and CX-283's pair agrees with `PILOT-PT-16` through
`pilots::rerun_status`; each late entry's plant is `<planting bead>:cx-<nnn>` with its own number);
`no_state_field` (a scratch entry with `state` is rejected as an unknown field); `late_block_requested` (the
allocation ledger holds `owner = "N25-CX"`, `count = 11`); `late_slugs_prefixed_unique` (each of the eleven slugs
is `augur-cx-late-nn`, no slug of the shared file occurs twice, and no slug equals one of N25-KA's
`augur-late-nn` tags: a scratch copy with an Augur entry tagged `late-01` beside N27-CX's fails, and so does one
tagged `augur-late-02`); spot checks of the cases' arithmetic in
exact rationals: `cx270_midpoint_841_800` ((1.45)·(1.45)/2 = 841/800 > 1); `cx271_triple_infeasible` (4/3 > 1);
`cx276_18_19_of_20`; `late01_tv_49_50`; `late03_two_terms` (1.2; 0.54; (21/20)⁷); `late04_8829_8000`;
`late06_quarter_above_two_ninths`; `late09_55_64`; `late11_fixed_point_100`. All run as `FROB_TEST_SEED=0 cargo
test -p frob-conformance --test cx_augur`.

### End-to-end script and logging

`scripts/e2e/augur_counterexamples.sh` (it calls only cargo targets of its dependencies, so no `frob` command):
runs the unit tests above; runs N38b-05's seeding tool, `cargo test -p frob-conformance --test seed_controls --
--ignored seed`, twice, asserting that the second run leaves the digest of the committed `controls.toml`
unchanged; runs the join, `cargo test -p frob-conformance --test seed_join`, for the Augur entries of
`crates/frob-conformance/registry/counterexamples.toml` (CX-270 to CX-287 and the late block); and reads the
control runner's history `registry/control_history.ndjson` (N38b-04) to report each control's state and latest
outcome. Running each runnable plant against its gate, with `--features gauntlet-mutations` and the plant, is the
control runner's, invoked by the planting beads' own scripts (N25a-08's for CX-270 to CX-281), and is not repeated
here. Logs through the STD-E2E helper to
`target/e2e-logs/augur_counterexamples/<utc-timestamp>.ndjson` with a transcript: `ts`, `script`, `bead`
(`N25-CX`), `step`, `event`, command, exit code, wall time; one `assert` line per runnable entry with the
identifier, the control's `NC-` identifier, the plant, the gates and its latest recorded outcome (`caught`,
`missed`, `never run`); one `metric` line per pending entry with its catching bead and milestone; the digest of
`controls.toml` before and after the second seeding; the `summary` gives caught of runnable, missed (threshold:
0), obligations and the pending count, which is 0 at M8.

### Known answers and negative controls

Every row is itself a negative control. Own controls of the registry, each on a scratch copy: an entry with no
`plant` (`every_row_has_control`); an entry with a `state` field (`no_state_field`); CX-283 relabelled DESIGN
(`kinds_plants_labels`); an Augur entry tagged `late-01` beside N27-CX's, and one tagged with N25-KA's
`augur-late-02` (`late_slugs_prefixed_unique`); 841/800
written 841/900 (`cx270_midpoint_841_800`). A row whose control is replaced by a test that always passes is
reported by the control runner as `missed` (N38b-04), and the script reports it as a failure.

## Considerations

- Several late rows pair two cases stated together by one rule (late-10, late-11); the catalogue's owner may
  split them when numbering.
- The cases quoted from plan-time pilots (CX-274, CX-283, CX-284, CX-285, CX-287, late-02) are FACTs about small
  programs, to be re-run from the reproduction bundle before a gate cites them: by N25a-08, BD-043, BD-355,
  BD-354, BD-292 and N25e-07 respectively, each of which records the receipt that sets `rerun = true`.
- Owed by the planting beads and the control runner, not by this registry: every entry whose plant is built has
  its control test, tagged with the entry's identifier, and that test fails when the shortcut is planted (a gate
  that nothing can fail validates nothing).
- N25a-08 writes no row of `controls.toml`: its `shortcut_nn` evaluators are the plants of CX-270 to CX-281, and
  the controls are seeded from these catalogue entries by N38b-05's tool, so there is one source.
- No row is a claim about a physical robot; each is arithmetic on a stated model.

## References

Appendix N.22; §25.4.7; §25.22; §25.29 to §25.39; CX-270 to CX-287; CX-010; CX-052.
