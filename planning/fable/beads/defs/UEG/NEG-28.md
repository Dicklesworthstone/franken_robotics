---
key: "NEG-28"
title: "registry: the consolidated hazard list: 59 hazards in seven archetypes, every dimension `assumed`"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "frob-scene", "frob-conformance", "solid", "registry", "test"]
parent: "WP-23"
deps: ["BD-227", "N14a-32", "N19a-31", "N19a-32", "N19a-33", "N19a-34", "N19a-35", "N19a-36", "N19a-37", "N19a-39"]
needs: []
related: []
gates: []
plan_refs: ["Appendix G.3", "§19.8", "§19.10", "§19.12", "§28", "OPEN-HAB-1", "OPEN-HAB-4", "FROB-REF-312", "Appendix H.6"]
---
## Background

A hazard is a feature of a place that causes a specific robot failure. Each of the seven interior archetypes
has a hazard registry, `registries/hazards/<archetype>.toml`, from which the scene generators draw; a row has
a name, a dimension range with units, a distribution, the mechanism that represents it, the failure it
provokes and a provenance field. The seven per-archetype files are entered by the hazard-registry bead
(BD-227), and each is bound to its generator and tested by its archetype's bead. This bead is the
consolidated view across all seven: the single table that says what the registries contain at plan time, the
check that the seven files agree with it, and the rules that apply to the set as a whole.

**Every dimension is `assumed`.** The ranges are generic values chosen to make scenes valid. No building
code, workplace regulation or product standard has been read for them, and none is cited. A row's provenance
changes to `norm: <citation>` or `measured: <dataset>` only when the source is recorded in the source
register; a pack counts the `assumed` dimensions it used (`dimensions_assumed` in its manifest); and a report
that claims a scene is representative of real buildings is refused while that count is non-zero
(FROB-REF-312). Open question OPEN-HAB-1 owns the search for cited norms (fail-closed default: everything
stays `assumed` and realism claims are refused). Retrofit prices and legality constraints
(`registries/retrofits.toml`) are `assumed` on the same terms (OPEN-HAB-4). Rows whose mechanism is marked
[F] are represented by a Frontier mechanism and by the Solid alternative beside it until that mechanism
lands. The same registries supply the rows of the scene-by-failure-mode incidence table from which
evaluation batteries are selected, so a battery can be asked which hazards it leaves uncovered.

## Scope

In: the consolidated table as data (`registries/hazards/INDEX.toml`); the consolidation check against the
seven files; the counts; the all-`assumed` invariant at plan time, checked over the whole set; the
Frontier-row fallback rule, checked over the whole set; the coverage column that NEG-29 checks.

Out, with the bead that has it: the seven registry files as data, their row schema and loader, the row
identifier `<archetype>.<slug>`, the draw, the realism-claim function behind FROB-REF-312 and the verb
`scene.hazards` (BD-227; this bead's index is a second, cross-checked statement of the same 59 rows, not a
second registry: generators and tasks never draw from `INDEX.toml`); per archetype, the binding of rows to
mechanisms, the draw-and-represent tests, the exposure fixtures and the generator's `not-represented` record
(N19a-31 to N19a-37); provenance promotion, which must update the index in the same change (N19a-39); the
incidence export and the uncovered-hazard query (N19a-38).

## Technical approach

**The table at plan time.** Dimension provenance is `assumed` in every row. "Covered by" uses the vocabulary
defined in NEG-29 (R = a requirement template, S = a shield, X = a refusal code) and is DESIGN.

| # | Archetype | Hazard | Dimension | Represented by | Failure it provokes | Covered by |
|---|---|---|---|---|---|---|
| 1 | Home | Interior door, clear width | 0.70–0.90 m | Jamb geometry, hinged leaf | Footprint does not pass, or passes only at one heading | X:119, R:pass |
| 2 | Home | Threshold or door bar | 5–25 mm high | Overlay (rounded box) | Small wheel stalls; pitch excursion; underslung strike | R:progress, R:pitch, R:clear |
| 3 | Home | Rug or mat | 5–20 mm thick; curled edge up to 30 mm | Rigid slab [S]; hinged plates [F] | Edge snag; rug pushed instead of climbed; slip on rug over hardwood | R:progress, R:slip |
| 4 | Home | Cord or cable across a path | 5–12 mm diameter | Fixed ridge [S]; chain cord [F] | Wheel or foot snag, entanglement | R:progress, R:nocoll |
| 5 | Home | Stairs | Riser 150–200 mm, tread 230–300 mm | Overlays with exact nosings | Step-height limit, toe strike, fall | X:119, R:clear, R:fall |
| 6 | Home | Hallway | 0.85–1.20 m wide | Walls | Turning envelope | X:119 |
| 7 | Home | Floor transition | Hardwood to carpet, friction step | Interface map | Yaw disturbance, odometry error | R:odom |
| 8 | Home | Toys and small clutter | 20–150 mm | Free rigid bodies | Ride-up, tip, blocked castor | R:pitch, R:progress |
| 9 | Home | Pets | 0.2–0.6 m, moving | Kinematic actor [F] | Unexpected obstacle within stopping distance | S:stop, R:sep |
| 10 | Home | Dim hallway | Low ambient light | Lighting level | Camera exposure lengthens, motion blur | R:sense |
| 11 | Apartment | Entry door | 0.80–0.90 m clear; closer torque 10–40 N·m | Hinged leaf with closer | Door cannot be pushed, or closes on the robot | X:131, R:pass, S:energy |
| 12 | Apartment | Lift door and sill | 0.80–1.10 m clear; sill gap 10–35 mm; level offset ±10 mm | Prescribed car and doors; overlay for the gap | Castor drops into the gap; door timing missed | R:drop, R:pass |
| 13 | Apartment | Right-angle corridor turn | Corridor 0.90–1.10 m | Walls | Body too long to turn | X:119 |
| 14 | Apartment | Balcony or sliding-door track | 15–40 mm high | Overlay | Wheel stall | R:progress, X:131 |
| 15 | Apartment | Compact kitchen galley | 0.80–1.00 m aisle | Walls and fixtures | No room to turn with a payload | X:119 |
| 16 | Office | Glass door or partition | Full height | Solid geometry with optical class `glass` | Not seen by optical ranging; collision | R:sense, R:nocoll, X:120 |
| 17 | Office | Castor chair | Five castors, base radius 0.30–0.35 m, 10–20 kg | Free body with castor joints | Pushed into the path; legs under the scan plane | R:nocoll, R:sense |
| 18 | Office | Carpet tile | Rolling-resistance step | Interface map, rolling-resistance row | Energy per metre rises; turn-in-place torque | R:energy, X:131 |
| 19 | Office | Door closer | 10–40 N·m closing torque | Closer torque law | Push capability exceeded | X:131, R:pass |
| 20 | Office | Cable cover | 10–20 mm high, bevelled | Overlay (wedge and box) | Small wheel stall | R:progress, X:131 |
| 21 | Office | Glossy lobby floor | Low roughness | Optical class `specular-floor` | Phantom depth below the floor | R:sense |
| 22 | Office | Repeated corridors and doors | Identical modules | Grammar repetition | Localization alias | X:120, R:loc |
| 23 | Warehouse | Aisle | 1.5–3.5 m wide | Racking | Turning and passing limits | X:119 |
| 24 | Warehouse | Rack beam, first level | 1.2–2.0 m above floor | Geometry above the scan plane | Overhead strike of a tall body or raised load | R:clear, R:sense |
| 25 | Warehouse | Pallet | 1.2 m × 0.8–1.0 m × 0.14–0.16 m | Free or static body with fork openings | Wheel caught in an opening; partial occlusion | R:progress, R:nocoll |
| 26 | Warehouse | Dock plate | Lip 10–30 mm; slope up to 10% | Wedge overlay; hinged plate | Traction limit with payload; pitch at the lip | X:131, R:slip, R:pitch |
| 27 | Warehouse | Ramp | 5–10% slope | Wedge | Traction and thermal limit | X:131, X:132, R:slip |
| 28 | Warehouse | Floor joint | 10–25 mm wide | Periodic or single slot overlay | Castor drop, vibration | R:drop |
| 29 | Warehouse | Stretch-wrap debris | Thin film on the floor | Low-friction patch; hinged plate [F] | Slip, wrap around an axle | R:slip, R:progress |
| 30 | Warehouse | Dust | Optical extinction and friction change | Optical volume; interface condition `dusty` | Range dropout; longer stopping distance | R:sense, S:stop |
| 31 | Warehouse | Identical aisles | Repeated bays | Grammar repetition | Localization alias | X:120, R:loc |
| 32 | Warehouse | Forklifts and carts | Moving | Kinematic actor [F]; free cart | Collision within stopping distance | S:stop, R:sep |
| 33 | Manufacturing plant | Conveyor | Belt 0.1–1.0 m/s, 0.6–0.9 m high | Moving surface | Placement error on a moving target; boundary work | R:place, R:energy |
| 34 | Manufacturing plant | Floor grate | Slot 15–30 mm, bar 5 mm | Periodic bar overlay | Foot or castor snag; twist at the contact | R:progress, R:drop |
| 35 | Manufacturing plant | Oil or coolant film | 0.05–1 mm thick, 20–100 mPa·s | Film patch | Slip that depends on dwell time and load | R:slip, X:242 |
| 36 | Manufacturing plant | Cable tray or cover | 15–40 mm high | Overlay | Stall, strike | R:progress, R:clear |
| 37 | Manufacturing plant | Fence and keep-out zone | Declared regions | Keep-out tag | Zone violation | R:zone, S:zone |
| 38 | Manufacturing plant | Polished metal | Mirror-like | Optical class `mirror` | Phantom points, glare | R:sense |
| 39 | Manufacturing plant | Floor vibration | 5–50 Hz, small amplitude | Prescribed floor motion [F] | Estimator bias, loosened grasp | R:odom, R:hold |
| 40 | Manufacturing plant | Chips and swarf | Millimetre debris | Interface condition; small free bodies | Rolling-element slip | R:slip |
| 41 | Retail store | Shelving aisle | 0.9–1.5 m wide; shelves 1.2–2.2 m high with lips | Geometry | Shelf strike; reach limit | X:119, R:clear |
| 42 | Retail store | Shopping cart | 0.6 m × 1.0 m, 15–30 kg, castor front wheels | Free body with castor joints | Blocked aisle; cart pushed into a display | R:nocoll, X:119 |
| 43 | Retail store | End cap | Tight turn at an aisle end | Geometry | Turning envelope with payload | X:119 |
| 44 | Retail store | Glass-fronted case | Glass with condensation | Optical class `glass` with a fog latent | Depth dropout or false return | R:sense, R:nocoll |
| 45 | Retail store | Spill | 0.1–2 mm film | Film patch | Slip | R:slip |
| 46 | Retail store | Entrance mat | Edge 5–12 mm | Rigid slab; overlay | Edge snag | R:progress |
| 47 | Retail store | Checkout lane | 0.6–0.9 m wide | Geometry | Footprint does not pass | X:119 |
| 48 | Retail store | Shoppers | Moving | Kinematic actor [F] | Stopping distance | S:stop, R:sep |
| 49 | Commercial kitchen | Galley aisle | 0.9–1.2 m wide | Fixtures | Turn with a carried container | X:119, R:hold |
| 50 | Commercial kitchen | Wet floor | Water film 0.1–2 mm | Film patch with wetness latent | Slip; specular depth artifacts | R:slip, R:sense |
| 51 | Commercial kitchen | Greasy floor | Oil or grease film 0.05–1 mm, 30–100 mPa·s | Film patch | Slip that persists through a stance | R:slip, X:242 |
| 52 | Commercial kitchen | Floor drain | Slot 8–15 mm; floor dished 1–2% | Periodic bar overlay; sloped floor | Castor drop; drift toward the drain on a film | R:drop, R:slip |
| 53 | Commercial kitchen | Anti-fatigue mat | 10–20 mm thick, bevelled, perforated | Rigid slab; overlay | Edge snag; mat slides on a wet floor | R:progress, R:slip |
| 54 | Commercial kitchen | Swing door | Two-way spring, closing torque 5–20 N·m | Swing-door fixture | Door returns onto the robot or its load | S:energy, R:pass, R:hold |
| 55 | Commercial kitchen | Low shelf | 0.15–0.30 m clearance under counters | Geometry | Strike of a tall chassis; lost line of sight | R:clear, X:119 |
| 56 | Commercial kitchen | Steam | Plume near dishwasher, kettle, fryer | Optical volume | Range dropout, false returns | R:sense, S:stop |
| 57 | Commercial kitchen | Hot liquid in a pot or tray | Fill 30–80% | Sloshing payload [F]; rigid ballast [S] (DESIGN, below) | Spill; destabilizing oscillation at step frequency | R:hold |
| 58 | Commercial kitchen | Stainless steel | Mirror-like | Optical class `mirror` | Phantom points | R:sense |
| 59 | Commercial kitchen | Cold-room threshold and strip curtain | 10–40 mm; hanging strips | Overlay; hinged plates [F] | Stall; occluded sensors | R:progress, R:sense |

**Counts (FACT about this table).** 59 hazards in seven archetypes: Home 10, Apartment 5, Office 7, Warehouse
10, Manufacturing plant 8, Retail store 8, Commercial kitchen 11. Every one `assumed`.

**Frontier rows.** The plan's table marks eight rows with a Frontier mechanism: rug hinged plates (3), chain
cord (4), pets (9), stretch-wrap hinged plate (29), forklifts as actors (32), prescribed floor motion (39),
shoppers (48) and strip-curtain hinged plates (59). Row 57 is a ninth: the table prints its one mechanism, the
sloshing payload, with no mark, but the slosh model is Frontier in the living-world tiers and first lands at
M6, with the kill gate KILL-SLOSH whose named fallback is rigid ballast with a spill metric from payload
acceleration. DESIGN, stated identically in BD-227 (which owns the registry data), N19a-37 and N19a-38: the
index and the kitchen file enter row 57 with two mechanisms, the sloshing payload (tier F) and rigid ballast
(tier S), because the kitchen task of the plan's worked artifacts draws the tray's fill from this row at M3
and a row with only a Frontier mechanism is not drawn. So nine rows name a Frontier mechanism. Six of them
have a Solid alternative beside the Frontier one (3, 4, 29, 32, 57, 59) and three have only the Frontier
mechanism (9, 39, 48). Rule: until the
Frontier mechanism lands, a generator that draws such a hazard uses the Solid alternative beside it (rows 3,
4, 29, 32, 57, 59), or, where there is none (rows 9, 39, 48), does not draw it and records the hazard as
`not-represented` in the pack (the spelling the generators and the uncovered-hazard query use; BD-227's
loader flags the same three rows with `solid_alternative = false`); a battery that needs it reports it
uncovered. A hazard is never silently dropped.

**Consolidation check (`hazard_index_matches_registries`).** `registries/hazards/INDEX.toml` holds the table
(number, archetype, the row identifier `<archetype>.<slug>` of BD-227, whose Slug column is the list of record
for the 59 slugs and is not derived a second time here, name, dimension text, mechanism list
with tier marks, failure text, provenance, cover codes). The archetype part of an identifier is the registry
file's name: `home`, `apartment`, `office`, `warehouse`, `manufacturing-plant`, `retail-store`,
`commercial-kitchen`. The check: the multiset of (archetype, name) in the seven files equals the index's, and
so does the set of row identifiers; each file's row count equals its archetype's count; dimension ranges,
units, mechanisms and failures agree field by field; provenance (one value per row, as in BD-227's schema) in
every file is `assumed` unless the source register holds the cited source, in which case the index is updated
by the same change.

## Acceptance criteria

- [ ] `INDEX.toml` has 59 rows with the per-archetype counts 10, 5, 7, 10, 8, 8, 11.
- [ ] The consolidation check passes against the seven files and fails on a planted mismatch of each kind (a
      missing row, a changed range, a changed unit, a provenance not backed by the source register).
- [ ] At plan time `assumed` count is 59 of 59; a pack built from any archetype reports a non-zero
      `dimensions_assumed`, and a realism claim on it is refused with FROB-REF-312.
- [ ] A Frontier-only hazard requested before its mechanism exists appears as `not-represented` in the pack
      and as uncovered in the battery query.

## Test plan

### Unit tests

- `hazard_index_has_59_rows`, `hazard_index_counts_per_archetype`.
- `hazard_index_matches_registries`: with four planted mismatches.
- `hazard_all_assumed_at_plan_time`; `hazard_provenance_promotion_requires_source_row`.
- `hazard_frontier_rows_have_solid_alternative_or_not_represented`: rows 3, 4, 29, 32, 57, 59 against 9, 39,
  48; nine rows in all carry a tier-F mechanism, in the index and in the seven files alike.
- `hazard_index_identifiers_match`: the 59 row identifiers of the index equal those derived from the seven
  file names and slugs (`home.threshold`, `commercial-kitchen.wet-floor`, …); a file named `kitchen.toml` is
  reported as an unknown registry.
- `hazard_dimension_ranges_parse_with_units`: every range has lower ≤ upper and a unit of the right
  dimension (length, torque, speed, viscosity, slope as a ratio, frequency).

### End-to-end script and logging

`scripts/e2e/scene_hazards.sh` (N14a-32) runs `frob scene hazards <archetype> --json` (the verb's form in
BD-227: the archetype is positional) for the seven archetypes; this bead adds one step to it: the concatenated output is compared with `INDEX.toml`
row by row. Log lines carry the archetype, the row count, the `assumed` count, and on a mismatch the first
differing field.

### Known answers and negative controls

The counts 59 and (10, 5, 7, 10, 8, 8, 11); the invariant "every dimension `assumed`". Controls: the four
planted mismatches; a row promoted to `norm:` with no source-register entry; a realism claim on a pack with
`dimensions_assumed > 0` (must be refused). Determinism: the check is a pure function of the registry files.

## Considerations

- None of these ranges is a statement about real buildings. They make scenes valid and failures reachable;
  the provenance field and FROB-REF-312 exist so that this cannot be forgotten downstream.
- The "Covered by" column is a design proposal recorded as data; NEG-29 checks it and the owners of the
  requirement templates and shields may change an assignment by editing the registry.

## References

Appendix G.3; §19.8; §19.10; §19.12; §28; OPEN-HAB-1; OPEN-HAB-4; FROB-REF-312; Appendix H.6.
