---
key: "N16-CX"
title: "frob-conformance: Kinesis counterexample registry (CX-144 to CX-152 and the late-subsection cases)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "kinesis", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "WP-10"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N16-REF", "N16b-02", "N16b-04", "N16b-06", "N16b-13", "N16b-14", "N16b-15", "N16b-28", "N16b-29", "N16b-32", "N16b-33", "N16c-03", "N16c-04", "N16c-05", "N38b-05"]
gates: []
plan_refs: ["Appendix N.13", "§16.4", "§16.5", "§16.6", "§16.8", "§16.9", "§16.10", "§16.11", "§16.19", "§16.20", "§16.21", "§16.22", "§16.23", "§16.24"]
---
## Background

A counterexample (CX) is the smallest hand-checkable case that breaks a tempting shortcut. The project's rule is that every plausible shortcut ships with its counterexample, and that the case becomes a permanent negative control: a planted defect that some gate must catch ("a gate that nothing can fail validates nothing"). The counterexample catalogue is data: `frob-conformance/registry/counterexamples.toml` is the authority for the entries and the printed appendix is generated from it; the negative-control registry `frob-conformance/registry/controls.toml` is derived from the catalogue by the seeding tool of N38b-05, which allocates each control's `NC-` number once and never rewrites it. A CX number is never reused and never renumbered; new rows append.

This bead is the registry of the counterexamples that belong to Kinesis (the dynamics chapter: three tiers, the transactional step, the batched step, the trust ladder, and the six late subsections on solve queues, decoupled stepping, step adjoints, loop elimination, ancestor-sparse assembly and the held command). It exists so that no counterexample is lost when the mechanisms are split across beads: each row below names the rule it protects, the gate that must catch its mutant, and the bead whose tests carry it.

## Scope

In: catalogue entries in `registry/counterexamples.toml` for CX-144 to CX-152 (the nine rows the catalogue already numbers for this chapter); the fifteen hand cases stated in the late subsections, which the catalogue has not numbered yet, entered as appended entries; the linkage test (every row has a plant, an owning test and a gate); the host test that recomputes every number; two corrections to the printed catalogue. Out, with the bead that has it: the catalogue's record schema, the loader, the seeding tool that turns entries into rows of `controls.toml`, the seed map and the join (N38b-05; this bead writes no row of `controls.toml` by hand); the number blocks for appended entries (N38a-40); running controls (N38b-04); implementing the mutants (the mechanism beads and N16c-03, N16c-04 do that); counterexamples of other chapters.

## Technical approach

**Record (the catalogue schema of N38b-05).** Each row below is one entry of `registry/counterexamples.toml`: `id` (the CX number), `shortcut`, `case` (with its numbers), `numeric`, `plan_case` (`"words"` or `"numbers"`: the form of the case in the plan's text, immutable), `case_supplied_by`, `rule` (text: the invariant, proof obligation, refusal or rule protected), `rule_ids` (the gates, invariants, obligations, decisions and refusal codes of the last two columns; the gate subset is what the seeding tool attaches the control to), `defined_in` (the plan subsection), `same_as`, `label` (DESIGN, or FACT with source), `control_kind` and `plant`, and, for an appended entry, `slug`. Kinds: a planted build defect is `control_kind = "mutation"` with `plant` the cargo feature of the owning crate that puts the shortcut into the real component (never a mock); an input the system must refuse or label is `planted-input`, `planted-design` or `planted-world` with `plant` the fixture; an arithmetic ceiling that a claim must not exceed (K5, K6) is `planted-input`: a request for the unsound claim, which must be refused, with the ceiling recomputed by the host test. The carrier column below names the owning bead; the owning test carries the entry's id (or slug until numbered) in its name, which is how the linkage test finds it. CX-149 and CX-150 are two of the catalogue's word-only entries (`plan_case = "words"`, "no numeric case is given in the chapter"): this bead supplies their cases, so they carry `numeric = true` and `case_supplied_by = "N16-CX: decision-level example"` and `"N16-CX: unresolved-trials arithmetic"`. Every other entry has `plan_case = "numbers"`.

**The nine numbered rows.**

| CX | Tempting shortcut | Smallest case that breaks it | Rule it protects | Gate; carrier |
|---|---|---|---|---|
| CX-144 | A rejected step may redraw its random inputs | A fair sign; a step is rejected when the sign is negative and redrawn once. P(accepted sign positive) = ½ + ½·½ = ¾ | INV-KIN-03 (a random input is a function of its address and interval, never redrawn); the dyadic refinement law (PO-KIN-04); `FROB-REF-223` | GATE-KIN-04; BD-083, N16c-01 |
| CX-145 | Simultaneous events can be solved one at a time, with constraints imposed afterwards | Two 1 kg sliders joined by a rigid tie reach a wall together at 1 m/s. Stopping both needs 2 N·s. Solving the wall contact for the first slider alone gives 1 N·s; imposing the tie afterwards leaves 1 kg·m/s shared by 2 kg: both move into the wall at 0.5 m/s | Coupled impulses per island with W = J·P·Jᵀ, P = M⁻¹ − M⁻¹Aᵀ(A·M⁻¹·Aᵀ)⁻¹A·M⁻¹; simultaneous touchdowns are ordinary inputs | GATE-KIN-04 (exact to 10⁻¹² N·s); BD-082, N16c-03 |
| CX-146 | A reduction over a padded axis may group its terms by the buffer's capacity | In `f32`, a = 10⁸, b = −10⁸, c = 1: (a + b) + c = 1 and a + (b + c) = 0, because the spacing near 10⁸ is 8. A grouping that depends on capacity makes a world's bits depend on its neighbours | PO-KIN-03 (pairwise trees aligned to powers of two, real entries in a prefix, canonical padding −0.0); INV-KIN-04 | GATE-KIN-05, GATE-PHYLUM-02; BD-181, N16c-04 |
| CX-147 | Penetration can be removed by a position-level correction at no cost in energy | Removing depth d in one step gives a velocity d/h from nothing: m = 1 kg, d = 1 mm, h = 2 ms gives 0.5 m/s and 0.125 J per event. The compliant law at k = 10⁵ N/m stores 0.05 J and returns less. A hopping policy that finds the first behaviour gains height from the solver | INV-KIN-06; the flag `energy-created` (above 1% of the absolute work exchanged in any 0.5 s window on a fast tier) | GATE-FID-06 (first planted exploit); BD-208, N16c-04 |
| CX-148 | A four-sided friction pyramid with the same μ is equivalent to the cone for ranking designs | A pyramid grants √2·μ along its diagonals, 41% more than the cone; the cone gives μ·N in every direction | The model-form rank check (cone arm) returning `model-conditional` | GATE-FID-05, GATE-FID-06 (second planted exploit), GATE-TRAC-04; BD-266 |
| CX-149 | A state error inside the fidelity envelope means the two tiers agree | A success predicate x ≥ 1 m with one tier ending at 1.0000001 m and the other at 0.9999999 m: state error 2·10⁻⁷ m, verdicts differ. The fidelity ledger records task decisions (success, terminal class, verdict of a comparison); a state error inside its envelope with a different decision is a disagreement | Decision-level equivalence as a ledger quantity; the promotion rule | GATE-FID-01, GATE-FID-02; BD-212, BD-263 |
| CX-150 | A world whose solver gave up can be scored as a failed episode | A world that exhausts its retry ladder ends `Faulted` with cause `numerical-fault` (`FROB-REF-221`) and publishes nothing from the failed interval; it is kept apart from `fall`, `slip` and `collision`, and summaries state how many worlds were excluded. Arithmetic illustration: a candidate with 2 successes in 2 valid trials and 98 unresolved has a complete-case rate of 100% and a true rate anywhere in [2%, 100%] | ADR-KIN-05; the failure taxonomy; step T7 of the transaction | GATE-KIN-04; BD-083 |
| CX-151 | The referee can adjudicate every scenario the fast tiers run | FrankenSim's solve admits at most 21 contacts. A sole with eight samples per foot fits when standing (16) and overflows in a fall or a crawl; six feet down with four samples each need 24; a tracked robot with patch samples along two belts overflows in normal driving | `FROB-REF-721` (a named capacity refusal, never a truncated contact set); the referee's own coverage per class; `established` names its adjudicator where coverage is below 90% | GATE-FID-03; BD-213 |
| CX-152 | A sampled CPU referee shadows most of a GPU run | A humanoid batch at 1.7 M world-steps/s and h = 2 ms produces 3,400 world-seconds per second, a quadruped batch at 2 M produces 4,000; a referee at 0.2 to 1.0 times real time per core on 24 cores produces 4.8 to 24. Share: 0.14 to 0.71% and 0.12 to 0.6%. A five-point gap in success rate needs 16·p·(1 − p)/δ² = 576 to 1,600 episodes per tier, 8 minutes to 1.9 hours of background referee time | The `referee_coverage` figure on every receipt; the background referee is a correctness guard and removes no training | GATE-FID-07 (at least 0.1%, reported); BD-214 |

**Rows appended from the late subsections.** Labels K1 to K15 are provisional tags: the entries take, in table order, the numbers of a block issued to N16-CX in `allocations.toml` by the allocator of N38a-40, and keep the tags in `slug` (`kin-k1` … `kin-k15`); beads cite the tag until the block exists, then the number.

| Label | Tempting shortcut | Smallest case that breaks it | Rule it protects | Gate; carrier |
|---|---|---|---|---|
| K1 | Sort worlds into row classes once, at reset | A trotting quadruped with four samples per foot has 8 active contacts in diagonal support and 16 in double support. With C = 24 the class bounds are 6, 12 and 24: the world crosses a class boundary twice per stride. A reset-time sort either leaves the loop bound below the active count (dropped contacts, forbidden) or parks the world in the largest class (nothing saved) | Per-step queue assignment | GATE-KIN-08; N16c-13, N16c-16 |
| K2 | A lane group goes idle once most of its worlds have converged | With a live fraction ℓ = ¼ and independent liveness across 32 lanes a group is busy with probability 1 − (¾)³² = 1 − 1.0 × 10⁻⁴: every group stays busy while three quarters of its lanes wait. After regrouping, 8,192 worlds fill 64 groups, not 256 | Regrouping of live worlds every second Newton iteration | GATE-KIN-08; N16c-13 |
| K3 | A reduction tree may be sized by the class loop bound | A kernel that sizes its tree by the class bound instead of anchoring it changes a digest when a world changes class (the CX-146 arithmetic) | INV-KIN-04 applied to work classes; anchored trees | GATE-KIN-08; N16c-13 |
| K4 | Two robots cannot touch within a tile if their closing speed is zero | Two robots at rest, 0.10 m between bounding spheres, each able to accelerate at 5 m/s²: the gap closes when 2·(½·5·t²) = 0.10 m, at t = √0.02 = 0.141 s. A velocity-only bound certifies any tile length and the robots collide inside a tile, unseen | The separation bound γ(t) = d − r_a − r_b − (v_a + v_b)·t − ½·(ā_a + ā_b)·t² with its acceleration term | GATE-KIN-09; N16c-19 |
| K5 | Per-world time tiles between interactions are a general ten-fold lever | With a fraction p of step time inside tiles of L steps and overhead o the bound is 1/(1 − p + p/L + o): ten needs p ≥ 0.924 at L = 64 and o = 0.01, and is out of reach for any p once L ≤ 10 (9.1 at p = 1). Policy and sensor frames bound a tile to 5 to 20 steps | The rejection of adaptive tiles as a general mechanism; the control tick as a tile fence | arithmetic; N16c-18 |
| K6 | Decoupled stepping of separated robots is a throughput lever | Against a comparator that factors island by island the saving is the cross-island broad phase alone; collision is about 4% of an incumbent's humanoid step, so the ratio is below 1.05. (Against a dense coupled factor: (1 − φ) + φ·N² against 1 + o gives 2.45 at N = 4 and 7.16 at N = 8 for φ = 0.1, o = 0.02, the weakest comparator) | The ceiling-first rule; the mechanism is claimed for admission only | arithmetic; N16c-18, N16c-22 |
| K7 | A zero gradient means a stationary point | A 1 kg block on a floor with μ = 0.5, pushed with u for one step of 0.01 s; breakaway at μ·m·g = 4.905 N. At u = 4 N, v⁺ = 0 and dv⁺/du = 0, although raising u past 4.905 N makes progress; at u = 6 N, v⁺ = 0.01095 m/s and dv⁺/du = 0.01 s/kg. Under a regularized law with a stiction speed of 1 mm/s the stick derivative is 0.01/(1 + 49.05) ≈ 2.0 × 10⁻⁴ s/kg, set by a numerical parameter | A `fixed-mode` gradient in stick is reported as uninformative; the consumer takes a derivative-free step | GATE-KIN-10; N16c-25 |
| K8 | The derivative of the step at a contact boundary is one number | Φ(v; z) = ½·m·(v − z)² + ½·g_c·v² with m = 2, g_c = 6: dv⁺/dz = ¼ with the contact closed and 1 with it open; at the boundary the one-sided values are 1 and ¼ | The label `one-sided`; both values on request | GATE-KIN-10; N16c-25 |
| K9 | Either root of the loop equation will do | Parallelogram a = c = 1, b = d = 2 at θ = 90°: the open branch gives φ = 90°; the crossed branch gives φ = −143.13° with the coupler end at (1.2, −0.6). A solver that returns the crossed root for a knee assembled open has folded the leg the other way | The branch recorded from the reference pose; refusal when the branches cannot be told apart | GATE-KIN-11; N16c-30, N16c-31 |
| K10 | A closed-form closure is valid over the whole joint range | The same parallelogram at θ = 0: all four links are collinear, the argument of the arccosine reaches +1, the branches meet and n(θ) is unbounded | Refusal of elimination where the sine of the transmission angle falls below 0.17 anywhere in the declared range | GATE-KIN-11; N16c-30 |
| K11 | A leaves-first factor has no fill for any contact row | A one-coordinate base with two one-coordinate limbs (n = 3). Rows on single limbs give H an arrow pattern with a zero at the (limb 1, limb 2) entry; one row touching both limbs fills that entry, and a factor that skips it is wrong | Routing a self-contact step to the dense path under a flag | GATE-KIN-12; N16c-37 |
| K12 | Tree sparsity helps every mechanism | A single chain (d_k = k) has ratio exactly 1 in every operation count: (n³ − n)/6 both ways. There is nothing to exploit, and the route reports that instead of running | The chain report; no gain is claimed without the ledger | GATE-KIN-12; N16c-35 |
| K13 | A loop that is stable for the exact held-input plant is stable on the tier | A slider with u = −(3/2)·(x + w) held over a 10 ms period: the exact one-tick map has spectral radius ½ and passes 64 of 64 cases; one semi-implicit step per tick has radius (1 + √3)/2 ≈ 1.366 and passes 1 of 64 (the origin) | Analysis A-22 evaluated with the tier's own integrator at the task's decimation | GATE-KIN-13; N16c-41, N16c-42 |
| K14 | An asymptotically stable loop meets its deadline | The same loop under two semi-implicit steps of h/2: radius (5 + √57)/16 ≈ 0.784, yet 49 of 64 cases miss max(abs(x), abs(w)) ≤ 1/32 mm at tick 16 | Requirements with deadlines are compared by verdicts, case by case, not by a stability label | GATE-KIN-13; N16c-44 |
| K15 | Halving the control period together with the step is a refinement study | Such a study compares two controllers: the controller's period is a physical parameter. With the clocks held fixed and h halved the fixture moves from 1 of 64 to 15 of 64; with the control period halved as well the loop itself has changed | Refusal "refinement study that changes a clock" | GATE-KIN-13; N16c-43 |

**Kind, plant and identifiers per entry (DESIGN).** `plant` is named `<carrier>:cx-<nnn>` for a numbered entry and `<carrier>:kin-k<n>` for an appended one: the Cargo feature (for a `mutation`) or the `synthetic` fixture (for the other kinds) under which the carrier builds the planted shortcut in the real component; the carrier may refine the name, keeping the entry's number or slug in it. `rule_ids` hold the identifiers of the rule column and the gates of the last column; K5 and K6 name no measuring gate, so their `rule_ids` are D-10 (every Frontier construction has a pilot and a ceiling) and KILL-INTERACT, the kill gate of §16.20 whose reading the two ceilings feed.

| Entry | `control_kind` | `plant` (the planted shortcut) | `label` |
|---|---|---|---|
| CX-144 | mutation | `BD-083:cx-144` (a rejected step redraws its random input) | DESIGN |
| CX-145 | mutation | `N16c-03:cx-145` (each wall contact solved alone, the tie imposed afterwards) | DESIGN |
| CX-146 | mutation | `N16c-04:cx-146` (a reduction grouped by buffer capacity) | DESIGN |
| CX-147 | mutation | `N16c-04:cx-147` (penetration removed by a position-level correction) | DESIGN |
| CX-148 | mutation | `BD-266:cx-148` (a four-sided pyramid at the same μ in place of the cone) | DESIGN |
| CX-149 | planted-input | `BD-212:cx-149` (two tiers inside the state envelope with different decisions, offered as agreement) | DESIGN |
| CX-150 | mutation | `BD-083:cx-150` (a faulted world scored as a failed episode) | DESIGN |
| CX-151 | mutation | `BD-213:cx-151` (the referee truncates a 24-contact set to 21 instead of refusing with `FROB-REF-721`) | FACT: `fs-contact/src/sustained/solve.rs`, `MAX_CONTACTS = 21`; `rerun = false` until BD-035 reads the constant at the pin (INT-FSIM-04) |
| CX-152 | planted-input | `BD-214:cx-152` (a receipt that claims a shadowed share above the computed `referee_coverage`) | DESIGN |
| K1 | mutation | `N16c-13:kin-k1` (row classes assigned once at reset) | DESIGN |
| K2 | mutation | `N16c-13:kin-k2` (no regrouping of live worlds) | DESIGN |
| K3 | mutation | `N16c-13:kin-k3` (a tree sized by the class loop bound) | DESIGN |
| K4 | mutation | `N16c-19:kin-k4` (the separation bound without its acceleration term) | DESIGN |
| K5 | planted-input | `N16c-18:kin-k5` (a claim of a ten-fold lever from adaptive tiles at L ≤ 10) | DESIGN |
| K6 | planted-input | `N16c-18:kin-k6` (a throughput claim for decoupled stepping against the dense coupled factor) | DESIGN |
| K7 | mutation | `N16c-25:kin-k7` (a zero stick gradient reported as stationary, without `fixed-mode`) | DESIGN |
| K8 | mutation | `N16c-25:kin-k8` (one derivative returned at the contact boundary, label `one-sided` dropped) | DESIGN |
| K9 | mutation | `N16c-30:kin-k9` (the loop solver returns either root) | DESIGN |
| K10 | mutation | `N16c-30:kin-k10` (the closed-form closure with its transmission-angle refusal removed) | DESIGN |
| K11 | mutation | `N16c-37:kin-k11` (a leaves-first factor that skips the fill entry) | DESIGN |
| K12 | planted-input | `N16c-35:kin-k12` (a sparsity gain claimed for a single chain) | DESIGN |
| K13 | mutation | `N16c-41:kin-k13` (A-22 evaluated with the exact held-input plant, not the tier's integrator) | DESIGN |
| K14 | planted-input | `N16c-44:kin-k14` (a stability label offered for a deadline requirement) | DESIGN |
| K15 | planted-input | `N16c-43:kin-k15` (a refinement study that halves the control period with the step) | DESIGN |

A FACT entry's control is written by the seeding tool as an `obligation` (not runnable, makes no gate eligible) until `rerun = true` with a receipt; CX-151's rule is checked against the constant read at the sibling's pin, never a literal.

**Numbers for the appended entries.** This bead files one request in the allocation ledger `crates/frob-conformance/registry/allocations.toml` (N38a-40): `[[block]]` with `registry = "cx"`, `count = 15`, `owner = "N16-CX"`, `section = "§16.19 to §16.24"`; the allocator fills `first` and `last`. When the block is issued, K1 to K15 are mapped to its numbers in table order in one commit; a tag still used as an identifier at this bead's milestone exit fails the ledger check `provisional-tag-past-due`.

**Corrections to the printed catalogue.** (1) The CX-146 row prints "canonical zero +0"; the chapter and the Numerics chapter require −0.0 as the neutral padding for sums, because (−0.0) + (+0.0) = +0.0 changes the bits of a sum that is −0.0. The registry row says −0.0. (2) The CX-151 row prints "below 80%" for the coverage under which `established` must name its adjudicator; the gate register (which governs) says 90%. The registry row says 90%.

**Linkage test.** A G0 entry of `frob-conformance` (`FROB_TEST_SEED=0 cargo test -p frob-conformance --test g0_laws -- registry::kinesis_cx`, the ADR-GAUNT-01 form of BD-025) asserts for every entry: it has a `control_kind` and a `plant` that names its carrier; every gate among its `rule_ids` exists in `registries/gates.toml`; no number is duplicated. Whether the owning test exists, tagged with the entry's id or slug, and whether a `mutation` entry's `plant` is a feature declared in the owning crate's manifest, are checked only once the carrier has built it: before that the entry is reported `pending: <carrier>` with the carrier's milestone, not failed (the rule BD-025 states for unbuilt plants), and a carrier whose milestone has passed with the plant still missing fails the test.

## Acceptance criteria

- [ ] Twenty-four entries (nine numbered, fifteen appended) are in `counterexamples.toml` with every field of the N38b-05 schema filled; CX-149 and CX-150 carry `plan_case = "words"`, `numeric = true` and a non-empty `case_supplied_by`; no row of `controls.toml` is written by hand.
- [ ] The fifteen appended entries carry numbers from the block issued to N16-CX in `allocations.toml`, in table order, with their tags in `slug`; `registry::alloc::validate` passes, and the numbers are recorded back into the beads that cite the tags.
- [ ] After the seeding tool of N38b-05 is re-run, each of the 24 entries has exactly one control in `controls.toml` whose `seed` names it, attached to the gates of its `rule_ids`; the tool's second run changes no byte; the join passes and no entry of this bead lands in `seed_findings.toml`.
- [ ] Every entry carries the `control_kind`, `plant` and `label` of the table above; CX-151 is FACT with `rerun = false` until BD-035's reading; K5 and K6 carry D-10 and KILL-INTERACT in `rule_ids`.
- [ ] The linkage test passes, reports a carrier not yet built as `pending` (not failed), and fails when a built row's owning test is deleted (planted).
- [ ] The two corrections are in the registry and the regenerated appendix shows them.
- [ ] Every `arithmetic` row has a host test that recomputes its numbers exactly: ¾; 0.5 m/s; 1 and 0; 0.125 J and 0.05 J; √2; 3,400 and 4,000; 576 and 1,600; 1 − (¾)³²; 0.141 s; 0.924 and 9.1; 2.45 and 7.16; 4.905 N and 2.0 × 10⁻⁴; ¼ and 1; −143.13° and (1.2, −0.6); 1.366 and 0.784.
- [ ] `scripts/e2e/kinesis_cx_registry.sh` exits 0.

## Test plan

### Unit tests

- `cx_registry_rows_complete`: schema check, unique ids, every field present.
- `cx_arithmetic_recomputed`: the list of numbers above, in exact rationals or certified double precision (for example (1 + √3)/2 enclosed in [1.36602, 1.36603]).
- `cx_linkage`: owner tests, mutant features and gates resolve for every built carrier; the others are listed `pending` with their carrier and milestone.
- `cx_kinds_and_plants`: the 24 `control_kind` and `plant` values equal the table; every `plant` names its carrier; CX-151 is FACT with `rerun = false` and a source.
- `cx_block_request`: the ledger holds `registry = "cx"`, `count = 15`, `owner = "N16-CX"`; after issue the fifteen numbers are contiguous and in table order, with `kin-k1` … `kin-k15` in `slug`.
- `cx_k9_branches`: at θ = 90° the crossed branch's coupler end is (1.2, −0.6) and φ = −143.13° (atan2(−0.6, −0.8)); the open branch gives 90°.
- `cx_planted_missing_owner_fails`: negative control of the linkage test itself.

All run as `FROB_TEST_SEED=0 cargo test -p frob-conformance --test g0_laws -- registry::kinesis_cx`.

### End-to-end script and logging

`scripts/e2e/kinesis_cx_registry.sh` (it calls only cargo targets of its dependencies) runs the G0 registry row; N38b-05's seeding tool, `cargo test -p frob-conformance --test seed_controls -- --ignored seed`, twice, asserting that the second run leaves the digest of the committed `controls.toml` unchanged; the join, `cargo test -p frob-conformance --test seed_join`; it then reads `registry/control_history.ndjson` and reports, per entry, its control's last recorded outcome (`caught`, `missed`, `stale`) or `never run`, and plants the registry's own negative controls on scratch copies under `target/e2e-scratch/kinesis_cx_registry/`. Running a mutant against its gate is the control runner's (N38b-04), invoked by each carrier's own script, and is not repeated here. NDJSON at `target/e2e-logs/kinesis_cx_registry/<utc-timestamp>.ndjson` with a transcript, through the STD-E2E helper so that every line carries `ts`, `script`, `bead`, `step`, `event` and the exact command line: a `cmd` line per invocation with exit code and wall time; per entry a `metric` line (`row`, `kind`, `plant`, `gate`, control state, last outcome); the digest of `controls.toml` before and after the second seeding; on failure the first missing entry or unresolved gate with the registry path; a `summary` line with entries, controls, pending carriers and the gates touched.

### Known answers and negative controls

The rows are the negative controls. The registry's own controls, each on a scratch copy: the planted missing owner (`cx_planted_missing_owner_fails`); an entry with an empty `plant` and one whose `plant` names no carrier (`cx_kinds_and_plants`); CX-151 relabelled DESIGN (`cx_kinds_and_plants`); a gate id misspelled (`cx_linkage`); an appended entry numbered outside N16-CX's block (`cx_block_request`, reason `unallocated-number` from the ledger). Determinism: the arithmetic rows are class D0 and must be identical on every host.

## Considerations

- Appended rows must not be renumbered once assigned; beads cite the provisional labels until then.
- Owed by the carriers and the control runner (N38b-04), not by this registry: at each milestone exit the rows whose gate is due have been caught in the same run that recorded the gate's pass.
- Rows CX-149 and CX-150 had no numeric case in the chapter; the cases given here are the decision-level example of the Numerics chapter and the unresolved-trials arithmetic of the Arena chapter, attached so that each row has something a test can compute.
- A row whose mechanism is killed (for example row K1 if KILL-ROWQUEUE fires) stays in the registry as a killed-idea record with its calculation; its mutant is retired.

## References

Plan Appendix N.13 (CX-144 to CX-152); §16.19 to §16.24 (the hand cases); §35.4, §35.5; §38.1 (controls before tests; registries are data).
