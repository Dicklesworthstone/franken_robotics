---
key: "ART-L4"
title: "golden fixture L.4: the `.fmission` query on stance and knee ratio, hand-derived answer, paving, CLI script"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "augur", "frob-mission", "frob-spec", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-42"
deps: ["BD-126", "BD-128", "BD-130", "BD-132", "BD-133", "BD-135", "BD-136", "N25a-12", "N25b-07", "N25c-21", "NJL-07", "NJLx-02", "STD-E2E"]
needs: []
related: ["ART-L9", "BD-042", "BD-140", "BD-141", "GB-31", "N24b-26", "N25a-10", "N25a-23"]
gates: []
plan_refs: ["Appendix L.4", "§25.3.2", "§25.3.3", "§25.5.2", "§25.13", "§25.3.6", "Appendix H.5"]
---
## Background

In FrankenRobotics (a robotics simulation and design system reached through the command line `frob`) a
**mission query** is a `.fmission` file (schema `frob.mission/1`): a quantified statement about a design
family, a world family, an observation channel and a policy class. The verb `mission.synthesize` answers it
with an **inner set** (cells of the design domain with a constructive witness), **excluded** cells with
proofs, **unresolved** cells, cost bounds and a certificate bundle; the **outer set** is the domain minus the
excluded cells.

This bead turns the fourth worked artifact of the specification into a golden fixture. In the nine-artifact
story (a quadruped carries a tray through a commercial-kitchen galley) it is the opening question: which
stance half-widths and knee belt ratios can hold the loaded tray in a four-foot stance over the whole declared
load envelope, with every foot inside its rating and every knee inside its continuous torque? The query is
static and exact. It does not ask whether the robot can walk; the task and the studies of the later artifacts
ask that. The design of artifact L.1 is created at the centre of an inner cell of this query's refined answer.

Why this fixture matters (DESIGN). The query is small enough to answer by hand in rational arithmetic, so
every number the engine returns has a known answer: the feasibility boundary, each cell's verdict, the witness
forces, the dual vector of the exclusions, the cost gap and its closure after one refinement. A mission engine
that returns any other number on this file is wrong, and the fixture says where. Rejected: using only the
chapter's own stance family (one free parameter): this query has two, so it also exercises a two-dimensional
paving, an unresolved band and a refinement.

## Scope

In: the specimen and resolved query; the schema's field rules as they apply to it; the closed form; the first
answer (twelve cells) and the refined answer (thirteen cells) as expected values; the outer set; a point
refutation; refusals on planted mutations of the query; the script. Out: the mission schema and admission
(BD-126), the engine and its proof rules (BD-128, BD-130), the checkers and bundles (BD-132, BD-133), the verbs
(BD-135); the bundle listing of artifact L.7; the layers and the identity map (NJL-07).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L4/`: `tray-stance-and-knee.specimen.fmission.json` (the text
below); `…resolved.fmission.json`; `answer.first.expect.json`; `answer.refined.expect.json`;
`outer.expect.json`; `refute.expect.json`; `closed-form.grid.json` (the 81 × 31 grid with labels);
`a1-attained.frobcert` (the 853 case below: the bundle of the first answer, written by `frob prove bundle <a1>`,
with its cost block edited to claim the minimum as attained and every hash recomputed by BD-136's mutation
library; generated once, pinned in the manifest, and regenerated only by a blessed change, since every input
of it is class D0); `fields.toml`.

**The artifact.**

```json
{
  "schema": "frob.mission/1",
  "name": "tray-stance-and-knee",
  "verb": "mission.synthesize",
  "design_domain": {
    "family": "template:quad-mammal@1",
    "topologies": ["rect4"],
    "ports": ["foot.pp", "foot.pm", "foot.mp", "foot.mm"],
    "isomorphisms": ["mirror-x", "mirror-y"],
    "free": [
      { "name": "v", "meaning": "1 / half_width", "unit": "1/m", "chart": "inverse-length", "box": ["4", "12"] },
      { "name": "r", "meaning": "knee belt ratio", "unit": "1", "chart": "linear", "box": ["1", "5/2"] }
    ],
    "bound": [
      { "name": "u", "meaning": "1 / half_length", "unit": "1/m", "value": "5" },
      { "name": "cap", "meaning": "normal-force rating per foot, port order", "unit": "N",
        "value": ["88", "88", "88", "88"] },
      { "name": "lam", "meaning": "knee lever arm in the loaded crouch", "unit": "m", "value": "1/10" },
      { "name": "tau_c", "meaning": "continuous knee torque at the motor", "unit": "N*m", "value": "4" }
    ],
    "tolerances": []
  },
  "world": {
    "kind": "bounded-set",
    "family": "static-tray-envelope@1",
    "frame": { "origin": "footprint-centre", "axes": "x-forward, y-left, z-up", "handedness": "right" },
    "parameters": [
      { "name": "W", "meaning": "total vertical load", "unit": "N", "value": "160" },
      { "name": "P", "meaning": "first moment (Σ xᵢ·fᵢ, Σ yᵢ·fᵢ) of the support forces", "unit": "N*m",
        "hull": [["16", "12"], ["16", "-12"], ["-16", "12"], ["-16", "-12"]] }
    ],
    "hidden": [],
    "persistence": {}
  },
  "observation": {
    "channel": "payload-wrench@1",
    "fields": ["W", "P"],
    "noise": "none",
    "timing": { "arrival": "before-decision", "clock": "static" }
  },
  "policy_class": {
    "kind": "causal",
    "indexed_by": "nominal-design",
    "authority": "independent-normal-force",
    "randomization": "none",
    "decision_makers": ["allocator"]
  },
  "horizon": { "kind": "static" },
  "predicates": {
    "accept": [
      { "id": "force-balance",  "expr": "Σ fᵢ = W" },
      { "id": "moment-balance", "expr": "Σ qᵢ·fᵢ = P" },
      { "id": "unilateral",     "expr": "fᵢ ≥ 0" },
      { "id": "rating",         "expr": "fᵢ ≤ capᵢ" },
      { "id": "knee-hold",      "expr": "lam·fᵢ ≤ r·tau_c" }
    ],
    "hard_loss": [],
    "margins": []
  },
  "quantifiers": [
    { "q": "exists", "var": "policy", "scope": "per-nominal-design" },
    { "q": "forall", "var": "world" },
    { "q": "forall", "var": "execution" }
  ],
  "objective": { "minimize": "1 / v", "unit": "m", "claim": "minimum-with-matching-lower-bound" },
  "evidence": { "profile": "exact", "arithmetic": "rational", "accept_classes": ["certified"] },
  "resolution": { "max_unresolved": { "measure": "area-fraction", "chart": "v, r", "value": "1/4" },
                  "mandatory": ["interior", "exterior"] },
  "response": ["inner", "excluded", "unresolved", "witnesses", "cost_bounds", "candidate_cost_gap",
               "next_test", "certificate"],
  "budget": { "wall_s": 60, "cpu_core_s": 60, "gpu_s": 0, "referee_s": 0, "checker_s": 10, "sync_wait_ms": 2000 }
}
```

**Field rules of the schema that this file exercises.** `design_domain`: a family, admitted topologies, free
parameters with unit, chart and box (a free parameter names its chart, here `inverse-length`, because
convexity and monotonicity are properties of a chart), bound parameters, tolerances and admitted
isomorphisms. `world`: a nonempty declared family fixed before any candidate is tested; `kind` is
`bounded-set` (worst case); every hidden parameter needs a persistence declaration (none here).
`observation`: the channel, with noise and timing; a field with no arrival instant is refused. `policy_class`:
`causal`, indexed by the nominal design, with authority `independent-normal-force`. `horizon`: `static`.
`predicates.accept`: atoms with comparator and units. `quantifiers`: the ordered prefix ∃ policy per nominal
design, ∀ world, ∀ execution; it is part of the `MissionId`. `objective`: minimize 1/v (the half-width) with
the claim `minimum-with-matching-lower-bound`. `evidence`: profile `exact`, rational arithmetic, accepted
class `certified`. `resolution`: at most 1/4 of the area of the (v, r) box unresolved; interior and exterior
obligations mandatory. `budget`: includes checker seconds as its own line. Every number is an exact string
(a rational is `"p/q"`, reduced, with positive denominator).

**The answer by hand, and the first paving.**

**The answer by hand.** Order the feet [++, +−, −+, −−] at (±1/u, ±1/v). With α = P_x·u/4 and β = P_y·v/4, every
allocation that balances force and both moments is

  f = (W/4 + α + β + t,  W/4 + α − β − t,  W/4 − α + β − t,  W/4 − α − β + t)

for one free scalar t. At the vertex P = (16, 12) N·m, W/4 = 40 N, α = 20 N and β = 3v N, so
f = (60 + 3v + t, 60 − 3v − t, 20 + 3v − t, 20 − 3v + t). The last foot cannot pull: f₋₋ ≥ 0 forces t ≥ 3v − 20, and
then f₊₊ ≥ 40 + 6v. Both `rating` and `knee-hold` bound f₊₊ from above, by 88 N and by r·τ_c/λ = 40·r N. Taking
t = 3v − 20 gives f = (40 + 6v, 80 − 6v, 40, 0), which is the witness family, affine in v. The same holds at the other
three vertices by the two mirrors. Hence:

  a design (v, r) is feasible  ⟺  40 + 6v ≤ min(88, 40·r)  ⟺  v ≤ 8 and r ≥ 1 + 3v/20.

The narrowest feasible body has v = 8, a half-width of 1/8 m, and needs r ≥ 11/5. At v = 8, r = 11/5 and P = (16, 12)
the forces are (88, 32, 40, 0) N: they sum to 160 N, give Σ xᵢ·fᵢ = (1/5)·(88 + 32 − 40 − 0) = 16 N·m and
Σ yᵢ·fᵢ = (1/8)·(88 − 32 + 40 − 0) = 12 N·m, and the knee torque is (1/10)·88 = 8.8 N·m against 2.2·4 = 8.8 N·m. In
right-handed axes with +z upward the support torque is Σ rᵢ × (0, 0, fᵢ) = (P_y, −P_x, 0) = (12, −16, 0) N·m; a packet
that passes (16, 12) directly as the torque must be rejected. The exclusion has a dual witness: with y = (0, ½, ½) on
the three balance equations, yᵀA = (1, 0, 0, −1), so f₊₊ − f₋₋ = 40 + 6v on every balanced allocation, while
f₊₊ − f₋₋ ≤ min(88, 40·r) on every admissible one.

**What a first pass returns.** The engine paves the box with cells and decides each from its corners, since
feasibility is monotone (decreasing in v, increasing in r). With four cells in v and three in r:

| Cell | v | r | Worst corner feasible | Best corner feasible | Verdict | Proof |
|---|---|---|---|---|---|---|
| c:01 | [4, 6] | [1, 3/2] | No | No: 64 > 60 at (4, 3/2) | Excluded | `lp-farkas`, knee |
| c:02 | [4, 6] | (3/2, 2) | No | Yes | Unresolved: straddles the threshold | — |
| c:03 | [4, 6] | [2, 5/2] | Yes: 76 ≤ 80 at (6, 2) | Yes | Inner | `lp-primal`, affine in v |
| c:04 | (6, 8] | [1, 3/2] | No | No: more than 76 against 60 | Excluded | `lp-farkas`, knee |
| c:05 | (6, 8] | (3/2, 2) | No | Yes | Unresolved: straddles the threshold | — |
| c:06 | (6, 8] | [2, 5/2] | No: 88 > 80 at (8, 2) | Yes | Unresolved: straddles the threshold | — |
| c:07 to c:09 | (8, 10] | all three | No | No: more than 88 against 88 | Excluded | `lp-farkas`, rating |
| c:10 to c:12 | (10, 12] | all three | No | No | Excluded | `lp-farkas`, rating |

One inner cell, three unresolved, eight excluded. The outer set is the domain minus the excluded cells: c:02, c:03,
c:05 and c:06. The unresolved area is 3 of 12 equal cells, which meets the declared resolution of 1/4. The objective
is not yet settled: the best certified design has half-width 1/6 m, the outer set reaches 1/8 m, and the gap is 1/24 m,
so the first answer is a feasible candidate with a gap and not a minimum. A refinement that splits c:06 at r = 11/5
makes c:06a = (6, 8] × [11/5, 5/2] inner (worst corner 88 ≤ 88) and leaves c:06b = (6, 8] × [2, 11/5) unresolved;
the incumbent and the lower bound then meet at 1/8 m. The design of L.1 sits at the centre of c:06a, at v = 7 and
r = 47/20, where the peak foot force is 82 N against 88 N and the knee torque 8.2 N·m against 9.4 N·m.

**Closed form and grid.** A design (v, r) is feasible exactly when 40 + 6·v ≤ min(88, 40·r), that is, when
v ≤ 8 and r ≥ 1 + 3·v/20. On the grid v = 4 + i/10 (i = 0…80), r = 1 + j/20 (j = 0…30), 2,511 points, an exact
rational linear-program test over the four load vertices agrees with the closed form at every point, and 515
points are feasible (FACT: recomputed for this bead).

**Expected answers** (the envelope's `result`; all values exact).

| Call | Expected |
|---|---|
| `mission.check` | domain `nonempty`; one topology; free parameters v and r; quantifiers echoed as "∃ policy per nominal design, ∀ worlds, ∀ executions"; fragment "linear static: exact answer available"; horizon `static` |
| `mission.synthesize`, first pass | Inner: c:03 = [4, 6] × [2, 5/2], proof p0. Excluded: c:01 and c:04 (proof p1, binding "knee-hold at foot.pp under P = (16, 12) N·m"); c:07 to c:12 (proof p2, binding "rating at foot.pp"). Unresolved: c:02, c:05, c:06, each `straddles-threshold`. Witness p0: `lp-primal`, vertex (16, 12), forces (76, 44, 40, 0) N at v = 6, r = 2. Cost bounds: incumbent 1/6 m, lower bound 1/8 m, gap 1/24 m, `attained: false`. Next test: split c:06 at r = 11/5. Certificate kinds `lp-primal`, `lp-farkas`, `partition`. Warning: the claimed minimum is not established; reported as a feasible candidate with gap 1/24 m |
| `mission.show --set outer` | Cells c:02, c:03, c:05, c:06; area fraction 1/3; inner within: c:03; unresolved within: c:02, c:05, c:06 |
| `mission.synthesize --from` the first answer (the refinement) | Inner: c:03 and c:06a = (6, 8] × [11/5, 5/2] (lower end in v open), proof p4. Unresolved: c:02, c:05, c:06b = (6, 8] × [2, 11/5). Witness p4: forces (88, 32, 40, 0) N at v = 8, r = 11/5. Cost bounds: incumbent 1/8 m, lower bound 1/8 m, gap 0, `attained: true`. Next test: none. Warning: the boundary design has zero reserve at foot.pp (rating, knee-hold) and at foot.mm (unilateral). Thirteen cells in all |
| `mission.refute` at v = 9, r = 5/2 | Excluded for every allocation; `lp-farkas`, proof p2; binding "rating at foot.pp: 94 N required, 88 N rated"; smallest relaxing changes: the rating to 94 N, or the P_y envelope to 32/3 N·m |
| Evidence on every answer | class Certified, tier `static`, determinism `D0` (the wire form of class D0, N35-01), claims [Proof], `reality: no-claim`; the three no-authorization flags, all false |

Open and closed ends: c:03 is closed on all four sides; cells in v ∈ (6, 8] are open at v = 6; c:06a is closed
at r = 11/5 and c:06b open there. The design of artifact L.1 sits at the centre of c:06a, v = 7 and
r = 47/20, where the peak foot force is 82 N against 88 N and the knee torque 8.2 N·m against 9.4 N·m (a
reserve of 6 N and 1.2 N·m).

**What the answer does not say.** It is a Proof claim of class Certified about the declared model:
independent control of the four normal forces, a load wrench observed before allocation, point feet, a static
stance. Under equal passive springs the same body carries different forces and the certificate does not
transfer. It says nothing about walking, slipping or the door.

**Refusals on planted mutations of the query.** An unknown field, a missing unit or a number that is not an
exact string: `FROB-REF-868`. An empty world family (W with no value): `FROB-REF-874`. Predicates that
conflict with the world family for every design (a rating of 30 N per foot against a 160 N load):
`FROB-REF-877` (the check at admission position 15, N25b-07, a dependency for that reason). No quantifier prefix: `FROB-REF-884`. An observation with no arrival instant:
`FROB-REF-847`. Each is exit 3 with one envelope, from `frob mission check` on the mutated file (the admission
of BD-126 and N25b-07). One more planted case is an answer, not a query, because 853 is not an admission code
and `mission.check` never raises it: the first answer's bundle with its cost block edited to claim the minimum
as attained (`attained: true` with incumbent 1/6 m against the lower bound 1/8 m), re-sealed with every hash
recomputed by BD-136's mutation library, offered to `frob mission synthesize <id> --reuse <bundle>` (N25c-21's
option) as caller-supplied evidence: `FROB-REF-853` (a minimum claimed without a matching lower bound; N25a-10),
exit 3 (the shape of N25a-23's case a17).

## Acceptance criteria

- [ ] The specimen equals the text above byte for byte (manifest SHA-256); the resolved file parses strictly,
      validates against `frob.mission/1` and round-trips canonical → value → canonical with zero byte
      differences; reordering the quantifier list changes the `MissionId`.
- [ ] `frob mission check` returns the first row of the expected-answers table.
- [ ] The first answer equals `answer.first.expect.json`: one inner, three unresolved and eight excluded cells
      with the boxes, closed flags, bindings and proofs of the table; unresolved area 3/12 ≤ 1/4; incumbent
      1/6, lower bound 1/8, gap 1/24, not attained.
- [ ] The refined answer equals `answer.refined.expect.json`: thirteen cells; c:06a inner; incumbent and lower
      bound both 1/8, gap 0, attained; the zero-reserve warning.
- [ ] The outer set is c:02, c:03, c:05, c:06 with area fraction 1/3, and is never listed as cells with
      witnesses.
- [ ] `frob mission refute` at (9, 5/2) returns 94 N against 88 N and the two relaxing changes (94 N; 32/3).
- [ ] The engine's verdict agrees with the closed form on all 2,511 grid points (515 feasible), in exact
      rationals.
- [ ] Each witness is checked by the offline auditor: forces sum to 160 N; first moments equal the vertex;
      every force within [0, 88] N; knee torque (1/10)·f ≤ r·4 N·m.
- [ ] The five planted query mutations are refused by `mission.check` with their codes (868, 874, 877, 884,
      847); the unmutated query is accepted; the re-sealed first-answer bundle that claims the minimum as
      attained is refused by `mission.synthesize --reuse` with `FROB-REF-853`, exit 3.
- [ ] The bundle of the refined answer, written by `frob prove bundle`, is accepted by `frob prove verify` and
      by the offline `frob-verify`.
- [ ] `MissionId`, both answers and the bundle root are identical on two runs, in debug and release, on Linux
      and macOS (class D0); once blessed they equal the map's entries for `q:3c9d…`, `a:0e51…`, `a:8b27…`,
      `b3:61f0…` and `b3:d4a9…`.

## Test plan

### Unit tests

`cargo test -p frob-mission --test golden_l4` and `cargo test -p frob-spec --test golden_l4` (the real engine,
checker and readers; exact rational arithmetic; no mocks):
- `l4_specimen_parses_and_validates`; `l4_manifest_hash`; `l4_roundtrip_value`; `l4_quantifier_order_in_id`.
- `l4_allocation_family`: at P = (16, 12), W = 160, u = 5, every balanced allocation is (60 + 3v + t,
  60 − 3v − t, 20 + 3v − t, 20 − 3v + t); with t = 3v − 20 it is (40 + 6v, 80 − 6v, 40, 0).
- `l4_closed_form`: feasible ⟺ v ≤ 8 and r ≥ 1 + 3v/20; spot values (8, 11/5) feasible with forces
  (88, 32, 40, 0); (8, 2) infeasible (88 > 80); (4, 3/2) infeasible (64 > 60, gap 4 N).
- `l4_grid_2511`: zero mismatches against the closed form; 515 feasible points.
- `l4_first_paving`: the twelve verdicts of the paving table, each from its corners.
- `l4_refinement`: splitting c:06 at r = 11/5 gives c:06a inner (worst corner 88 ≤ 88) and c:06b unresolved.
- `l4_cost_bounds`: 1/6 − 1/8 = 1/24 before; 1/8 = 1/8 after.
- `l4_dual_witness`: y = (0, 1/2, 1/2) gives yᵀA = (1, 0, 0, −1) and the value 40 + 6v; 64 at v = 4, 94 at
  v = 9.
- `l4_torque_sign`: the support torque for P = (16, 12) is (12, −16, 0) N·m; a packet that passes (16, 12) as
  the torque is rejected.
- `l4_centre_reserve`: at (7, 47/20) the peak force is 82 N and the knee torque 41/5 N·m against 47/5 N·m.
- `l4_refute_point`: 94 N against 88 N; relaxing changes 94 N and 32/3 N·m.
- `l4_mutation_<code>` for 868, 874, 877, 884, 847, 853.

### End-to-end script and logging

`scripts/e2e/art_l4_fmission.sh` (header: `bead=ART-L4 milestone=M1 labels=golden,appendix-l,augur,
verb:mission.new,verb:mission.check,verb:mission.synthesize,verb:mission.show,verb:mission.refute seed=1
needs=cpu plants=cap-80,torque-as-moment`). Steps: (1) `frob mission new …resolved.fmission.json --key l4-new
--json`, capture the `MissionId`; (2) `frob mission check`; (3) `frob mission synthesize <id> --estimate
--json` (nothing runs; an estimate with an interval); (4) `frob mission synthesize <id> --paving
v:4,r:3:lu --budget cpu_core_s=60,checker_s=10 --key l4-syn-1 --sync-wait-ms 2000 --json`, compare the result with
`answer.first.expect.json` field by field (`e2e_assert_rational` for every bound and force); (5) `frob mission
show <id> --answer <a1> --set outer --json`; (6) `frob mission synthesize <id> --from <a1>
--key l4-syn-2 --json`, compare with `answer.refined.expect.json`; (7) `frob mission refute <id> --design
v=9,r=5/2 --key l4-ref --json` (`mission.refute` is a `run` verb, so the call carries a key); (8) `frob prove
bundle <a2> --out <scratch>/l4.frobcert --key l4-bun --json` (VERBS-prove's mission-answer form, BD-133), then
`frob prove verify <scratch>/l4.frobcert --json` and the offline `frob-verify` on the same file, both `accept`;
(9) the five query mutations as `frob mission check <mutated file> --json` through `e2e_expect_refusal <code>
3`, and the 853 case, `frob mission synthesize <id> --reuse fixtures/golden/appendix-l/L4/a1-attained.frobcert
--key l4-m853 --json` through `e2e_expect_refusal FROB-REF-853 3` (the file's digest logged); (10)
`frob-golden check --artifact L4`. Plants: `cap-80` lowers the rating to 80 N and must change the boundary to
v ≤ 20/3 (the comparison with the expected answer fails); `torque-as-moment` must make the auditor reject. The
log (`target/e2e-logs/art_l4_fmission/<utc>.ndjson`) records every command with exit code and wall time, the
evidence block (claims [Proof], class Certified, tier `static`, determinism `D0`), each compared value as
an exact rational with its expectation, the bundle digests, each refusal with code and first repair, and the
`summary`.

### Known answers and negative controls

- Known answers (exact; recomputed for this bead): the feasibility boundary; the twelve and thirteen cells;
  the witness forces (76, 44, 40, 0) and (88, 32, 40, 0) N; the dual vector and its values 64 and 94; the gap
  1/24 m and its closure; the grid count 515 of 2,511; the centre reserve 6 N and 6/5 N·m. FACT recorded at
  plan time for the same artifact: zero mismatches on the 81 × 31 grid; one inner, three unresolved and eight
  excluded cells. These are rows JL-11 (the closed form) and JL-12 (the paving and the grid) of NJLx-02's
  known-answer block (a dependency), which this bead's tests cite by number once the block is issued.
- Related known answer of the mission chapter (a different family, kept by its own beads): the 400 N stance
  family with boundary exactly 3/14 m.
- Counterexample kept alive: reporting an incumbent as a minimum. The first answer has a feasible candidate at
  1/6 m and a lower bound at 1/8 m; only when the two meet is a minimum stated.
- Negative controls: the two plants; the five query mutations and the re-sealed 853 answer; a swapped foot order with unchanged forces; an
  unreported positive force floor; a passive-authority tag (each must be rejected by the auditor).
- Metamorphic relations: mirroring the load hull in x or y permutes the witness forces and changes no verdict;
  scaling W, the hull and the ratings by one factor leaves the boundary in v and scales forces.
- Determinism lanes: twice on one machine; debug and release; Linux and macOS; a second machine re-checks the
  bundle; class D0.

## Considerations

- The specimen session shows exit code 0 for the first synthesis and 2 for the refinement although both carry
  a warning. Exit 2 means "completed with findings or flags"; the script accepts 0 or 2 for both calls and
  asserts the warnings by content. The discrepancy is recorded in the unit's coverage file.
- The cell labels (`c:01` … `c:12`, `c:06a`, `c:06b`) and proof labels (`p0` … `p4`) are the specimen's;
  the engine may number cells otherwise. The comparison is by box, not by label, and the expected files map
  the specimen's labels to boxes.
- The specimen session writes the refinement as `--strategy refine --from a:…`, and its first answer's
  `next_actions` carries that command. N25a-12 refines with `--from` alone and refuses `--strategy refine` at M1
  with `FROB-REF-724`, because `refine` names the trajectory-optimization lane (BD-362); step (6) therefore uses
  `--from`, and the `next_actions` command is asserted to be the registered path `frob mission synthesize` with
  `--from` and the first answer and no `--strategy`, which reproduces step (6); it is not compared with the
  specimen's string. NJL-08, the MCP transcript of the same session, asserts the same.
- Kill gate KILL-AUGUR-FALSECERT: one accepted false certificate fails the mission gates. The plants of this
  fixture are part of the seeded-defect suite that must yield zero false accepts.
- The paving the engine chooses may differ from four by three: on this linear-static fragment its default is
  `exact` (one split at the boundary), and `adaptive` elsewhere (N25a-12). A finer paving is acceptable when its
  inner set contains c:03, its excluded set contains the eight excluded cells and its unresolved area is at most
  1/4. The exact-equality comparison of step (4) is made with the registered option `--paving v:4,r:3:lu`
  (N25a-12's explicit grid: four cells in v, three in r, the second r cut owned by the upper cell), which gives
  the specimen's cells with their open and closed ends; N25a-12 is a dependency for that reason. An earlier
  draft named a fixture-only `--paving 4x3`, which no bead defines.

## References

Appendix L.4; §25.3.2 (schema), §25.3.3 (the answer), §25.3.6 (minimum and gap claims), §25.5.2 (proof kinds
`lp-primal`, `lp-farkas`, `partition`), §25.13; INV-AUGUR-02 (the outer set is the domain minus the excluded
cells); GATE-AUGUR-01, GATE-AUGUR-02 and GATE-LEMMA-04 are measured by the mission and certificate beads.
