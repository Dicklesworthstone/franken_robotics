---
key: "ART-L1"
title: "golden fixture L.1: the `.fbot` design `quad-tray`, its canonical form, both views, derived facts, CLI script"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-design", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-07"
deps: ["ART-L4", "BD-054", "BD-055", "BD-056", "BD-057", "BD-061", "BD-062", "BD-102", "BD-109", "BD-110", "N14a-21", "NJL-07", "NJLx-02", "STD-E2E"]
needs: []
related: ["ART-L9", "BD-069", "BD-075", "BD-099", "BD-141", "N14a-28", "N14bx-03"]
gates: []
plan_refs: ["Appendix L.1", "§14.4", "§14.5", "§14.9", "§14.11", "§13.4", "Appendix G.4"]
---
## Background

FrankenRobotics (a robotics simulation and design system reached through the command line `frob`) stores a
robot design as a `.fbot` file: canonical JSON with schema `frob.design/1`, whose `DesignId` is the hash of
the canonical bytes. The specification ends with nine worked artifacts for one story (a quadruped that carries
a tray through a commercial-kitchen galley and sets it down at a pass counter). This bead turns the first of
them, the design `quad-tray`, into a golden fixture: the artifact carried completely, its schema validation,
its round-trip tests and the end-to-end script that produces it from the command line.

Place in the story. A static mission query (artifact L.4) asks which stance half-widths and knee belt ratios
can hold the loaded tray; `quad-tray` is the design created at the centre of the inner cell c:06a of its
refined answer, at v = 1/half-width = 7 per metre and knee ratio r = 47/20. Hence the half-width 1/7 m and
the belt ratio 2.35 in the file. Its leg is the module of artifact L.2; it is the design the later artifacts
attack, edit, compare and qualify.

The artifact is a DESIGN specimen: its numbers illustrate the format and are not results. It is written **as
authored**; the canonical form sorts keys, removes white space and writes out every default. The half-width
1/7 m has no terminating decimal and is therefore written as a ratio.

Why a golden fixture of this design (DESIGN): it exercises, in one file, every block of the format that the
first milestone ships (nested mirror constructors, three fills, a belt transmission, a foot with a contact
law, a removable payload, the power bus with a shared thermal sink, four sensors on two clocks, budgets,
levers on three manifolds, locks and both symmetry assertions), and it is the hinge of the nine-artifact
story, so a format change that breaks it breaks a test before it breaks an agent. Rejected: generating the
fixture from the archetype template (the test would then compare the template with itself).

## Scope

In: the specimen file and its resolved twin; pointer-level expectations of the canonical form; the
S-expression view; the facts the compiler must derive; refusals on planted mutations; the script that creates
the design from the file and from the mission cell and compares. Out: the format, the canonical writer, the
constructors, the compiler and the analyses (their own beads, in `deps`); the layers and the identity map
(NJL-07); the format-registry round-trip gate over random designs (BD-141).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L1/`: `quad-tray.specimen.fbot.json` (the text below, byte for
byte); `quad-tray.resolved.fbot.json` (the mission identity in `meta.source` filled from the identity map);
`quad-tray.canonical.fbot.json` and `quad-tray.sexpr` (written by the implementation, blessed once);
`canonical.expect.json`, `derived.expect.json`, `fields.toml` (expectations and field classes).

**The artifact, as authored.**

```json
{ "schema": "frob.design/1", "name": "quad-tray",
  "meta": { "author": "agent-17", "licence": "CC0-1.0", "source": "mission q:3c9d…, cell c:06a, centre" },
  "root": { "body": "torso", "region": {"box": ["0.46 m", "0.24 m", "0.10 m"]},
            "fill": {"shell": "2.5 mm"}, "material": "al-6061-t6",
            "ports": { "hip_mount":  {"xyz": ["0.20 m", "1/7 m", "0 m"]},
                       "tray_mount": {"xyz": ["0 m", "0 m", "0.05 m"]} } },
  "limbs": { "mirror": "xz", "of": { "mirror": "yz", "of": {
      "attach": "torso.hip_mount",
      "chain": [
        { "joint": {"class": "R", "axis": [1,0,0], "limits": ["-45 deg", "45 deg"]},  "actuator": "qdd-m",
          "body": {"region": {"capsule": ["0.03 m", "0.06 m"]}, "fill": "solid", "material": "al-6061-t6"} },
        { "joint": {"class": "R", "axis": [0,1,0], "limits": ["-90 deg", "150 deg"]}, "actuator": "qdd-m",
          "body": {"region": {"capsule": ["0.025 m", "0.21 m"]}, "fill": {"shell": "2 mm"},
                   "material": "al-6061-t6"} },
        { "joint": {"class": "R", "axis": [0,1,0], "limits": ["-160 deg", "-30 deg"]}, "actuator": "qdd-l",
          "transmission": {"belt": {"ratio": "2.35"}},
          "body": {"region": {"capsule": ["0.02 m", "0.21 m"]}, "fill": {"shell": "1.5 mm"},
                   "material": "cfrp-ud"} },
        { "foot": {"region": {"sphere": "0.022 m"}, "fill": "solid", "material": "rubber-60a",
                   "contact": {"law": "foot-patch", "interface": "rubber-60a/default",
                               "torsion_radius": "8 mm"}} } ] } } },
  "payloads": [ { "body": "tray", "region": {"box": ["0.40 m", "0.30 m", "0.03 m"]}, "fill": {"shell": "2 mm"},
                  "material": "pp-homopolymer", "mount": "torso.tray_mount", "removable": true } ],
  "power_bus": { "id": "main", "battery": "li-6s-5ah", "nominal": "22.2 V", "current_max": "60 A",
                 "regen": "to-battery", "sink": {"conductance": "1.6 W/K"} },
  "sensors": [ {"id": "imu", "card": "imu-generic", "parent": "torso", "rate": "500 Hz", "clock": "mcu"},
               {"id": "enc", "card": "encoder-14b", "parent": "joints:*", "rate": "1000 Hz", "clock": "mcu"},
               {"id": "fc", "card": "foot-contact", "parent": "feet:*", "rate": "500 Hz", "clock": "mcu"},
               {"id": "lidar", "card": "lidar-2d-generic", "parent": "torso",
                "mount": {"xyz": ["0.21 m", "0 m", "0.06 m"]}, "rate": "10 Hz", "clock": "lidar"} ],
  "budgets": { "mass_max": "14 kg", "cost_max": "4000 USD", "power_peak_max": "900 W",
               "envelope_max": ["0.70 m", "0.40 m", "0.50 m"], "payload_min": "2 kg" },
  "levers": [ {"id": "half_width", "path": "/root/ports/hip_mount/xyz/1",
               "bounds": ["1/12 m", "1/4 m"], "manifold": "R+"},
              {"id": "knee_ratio", "path": "/limbs/of/of/chain/2/transmission/belt/ratio",
               "bounds": ["1.0", "2.5"], "manifold": "R+"},
              {"id": "foot_radius", "path": "/limbs/of/of/chain/3/foot/region/sphere",
               "bounds": ["0.018 m", "0.032 m"], "manifold": "R+"} ],
  "locks": ["power_bus", "payloads"],
  "symmetry": {"assert_mech": "C2xC2", "assert_obs": "C2"} }
```

**What each block says.** `root`: the torso, a box of 0.46 × 0.24 × 0.10 m with a 2.5 mm aluminium shell, with
two named mount frames. `limbs`: one leg (hip abduction about x, hip pitch about y, knee about y through a belt
of ratio 2.35, and a rubber foot with the `foot-patch` contact law) attached at `torso.hip_mount`, mirrored
across the yz-plane and then across the xz-plane: four legs. `payloads`: the tray, removable, centred on the
torso. `power_bus`: one 22.2 V bus limited to 60 A, with a shared thermal sink of 1.6 W/K. `sensors`: an IMU,
encoders on every joint, foot contact switches, and one forward 2D lidar on its own clock. `budgets`: limits
checked at preflight. `levers`: the three parameters an optimizer may move, each by JSON pointer: the
half-width in [1/12, 1/4] m, the knee ratio in [1.0, 2.5], the foot radius in [0.018, 0.032] m. `locks`:
blocks no edit may change without an explicit unlock. `symmetry`: asserted groups, which the compiler must
confirm.

**Canonical form: the seven rules, and what they do to this file.** (1) UTF-8, object keys sorted by code
point, no insignificant white space, a duplicate key refuses. (2) A quantity is a string of an exact decimal
and a unit token; a bare string has provenance `designed`. (3) The canonical unit is the coherent SI unit and
the canonical decimal is the shortest that equals the exact rational; a value with no terminating decimal is a
ratio. (4) An angle given in degrees or turns is an exact rational of a turn. (5) Integers above 2⁵³, hashes
and rationals travel as strings. (6) Every field the compiler will read is present; schema defaults are
written out with provenance `default`. (7) Constructor terms stay unexpanded.

Pointer-level known answers for the canonical file (`canonical.expect.json`):

| Authored | Canonical | Rule |
|---|---|---|
| `"2.5 mm"`, `"2 mm"`, `"1.5 mm"`, `"8 mm"` | `"0.0025 m"`, `"0.002 m"`, `"0.0015 m"`, `"0.008 m"` | 3 |
| `"0.20 m"`, `"0.10 m"`, `"0.40 m"`, `"0.30 m"`, `"0.70 m"`, `"0.50 m"` | `"0.2 m"`, `"0.1 m"`, `"0.4 m"`, `"0.3 m"`, `"0.7 m"`, `"0.5 m"` | 3 |
| `"1/7 m"`, `"1/12 m"` | unchanged (no terminating decimal) | 3 |
| `"1/4 m"` (upper bound of the half-width lever) | `"0.25 m"` | 3 |
| `"1.0"` (lower bound of the knee ratio) | `"1"` | 3 |
| `"-45 deg"`, `"45 deg"` | `"-1/8 turn"`, `"1/8 turn"` | 4 |
| `"-90 deg"`, `"150 deg"` | `"-1/4 turn"`, `"5/12 turn"` | 4 |
| `"-160 deg"`, `"-30 deg"` | `"-4/9 turn"`, `"-1/12 turn"` | 4 |
| absent joint `origin`; absent `collision`; absent `power_bus.members` | written out: identity; the Region; every actuator, each with provenance `default` | 6 |
| the two `mirror` terms and the `chain` | still constructor terms; no expanded leg appears | 7 |

**Facts the compiler must derive** (`derived.expect.json`; `frob design show` and `frob analyze symmetry`):
1. Twelve actuated coordinates: four legs of three revolute joints.
2. Feet at (±1/5, ±1/7) m in the nominal stance (the hip mount is at x = 0.20 m, y = 1/7 m).
3. Γ_mech = C₂ × C₂ (four elements), generated by the two nested mirrors; the centred tray preserves both.
4. Γ_obs = C₂ (two elements): the forward lidar at x = 0.21 m, y = 0 breaks the fore-aft mirror and keeps the
   left-right mirror. The report names the lidar as the breaker.
5. The archetype is `quad-mammal`.
6. Every body has an explicit fill (there is no default fill), and inertia is derived from Region, fill and
   material: no body carries declared mass properties.
7. The three lever paths resolve, and the current values lie inside their bounds: 1/7 in [1/12, 1/4]; 2.35 in
   [1, 2.5]; 0.022 in [0.018, 0.032].

**Validation and refusals on planted mutations.** A field the schema does not list refuses (`FROB-REF-103`);
an unknown major version refuses (`FROB-REF-125`) and names the migration; a body without a fill refuses
(`FROB-REF-105`); an empty or inverted joint interval refuses (`FROB-REF-107`); a declared symmetry that
differs from the computed one refuses with the breaker (`FROB-REF-109`); a lever value outside its bounds, or
an edit to a locked block, refuses (`FROB-REF-111`); a sensor without a parent frame, a rate or a clock
refuses (`FROB-REF-126`). Each is exit 3 with one envelope and ranked repairs. That is eight planted mutations
for seven codes (111 twice), each refused by the stage that owns it (N14b-10), through the verb that refuses
there. `frob design new --file` refuses only a document with no canonical form, with the reader's code and
nothing stored: the unknown field (103) and the unknown major version (125). A file that has a canonical form
and fails stage 1 is stored by `design new` and answered with exit 2 and the code as a finding (N14a-21: the
stage-1 codes are findings of that verb, not refusals), so the missing fill (105), the lever value outside its
bounds (111) and the sensor without a clock (126) are loaded with `frob design new --file` (exit 2, the finding
asserted) and then refused with exit 3 at stage 1 by `frob design check` on the stored design; the inverted
interval (107) is refused at stage 3 and the declared symmetry (109) at stage 5, by `frob design check` on the
stored design; and the lock (111) by `frob design edit` on a locked block, since a lock constrains edits, not
files.

**Round trips.** (a) authored → canonical → value → canonical: zero byte differences, and a second
canonicalization changes nothing; (b) canonical JSON → S-expression view → canonical JSON: the identity, with
equal `DesignId` (the invariant that both views have one identity); (c) respellings that must keep the
identity: reordered keys, added white space, `"200 mm"` for `"0.20 m"`, `"0.125 turn"` for `"45 deg"`;
(d) respellings that must change it: `"0.142857 m"` for `"1/7 m"` (a different exact value).

**From the mission cell.** `frob design new quad-tray --from-cell <mission>/c:06a` instantiates the template
`quad-mammal` at the cell centre. The result must have half-width `"1/7 m"` and ratio `"2.35"`, and its
compiled model identity must equal that of the fixture (the `meta` block, which no mechanical compile product
reads, may differ, so `DesignId` equality is asserted only when `meta` is passed identically).

## Acceptance criteria

- [ ] The specimen file equals the text above byte for byte (manifest SHA-256), parses strictly and validates
      against `frob.design/1`.
- [ ] The canonical file satisfies every row of the pointer table; canonicalization is idempotent; the three
      round trips of (a) to (c) hold with zero byte differences and one `DesignId`; (d) changes the identity.
- [ ] `frob design show` and `frob analyze symmetry` report the seven derived facts exactly (twelve
      coordinates; feet at (±1/5, ±1/7) m; C₂ × C₂; C₂ with the lidar named; `quad-mammal`; derived inertia;
      levers in bounds).
- [ ] Each of the eight planted mutations (seven codes) is refused with its code and exit 3 by the verb named
      above; the unmutated file is accepted.
- [ ] `frob design check quad-tray --json` returns `closure: "closed"`, `symmetry: "C2xC2"` and no finding of
      severity high, with evidence class Tested, tier `static`, determinism `D0` (the wire form of class D0,
      N35-01, whose reader rejects `exact`).
- [ ] The design created from cell c:06a has half-width 1/7 m, ratio 2.35 and the fixture's model identity.
- [ ] `DesignId`, canonical bytes and the S-expression text are identical on two runs, in debug and release,
      on Linux and macOS (class D0); once blessed, the identity equals the map's entry for `d:7a3e…`.

## Test plan

### Unit tests

`cargo test -p frob-design --test golden_l1` and `cargo test -p frob-analyze --test golden_l1` (the real reader,
writer and compiler on the fixture; no mocks):
- `l1_specimen_parses_and_validates`; `l1_manifest_hash`.
- `l1_canonical_pointer_table`: one assertion per row of the table (for example `/limbs/of/of/chain/1/joint/
  limits/1` is `"5/12 turn"`; `/levers/0/bounds/1` is `"0.25 m"`; `/root/ports/hip_mount/xyz/1` is `"1/7 m"`).
- `l1_canonical_idempotent`; `l1_roundtrip_value`; `l1_view_roundtrip_same_id`.
- `l1_respelling_keeps_id` (four cases); `l1_respelling_changes_id` (`"0.142857 m"`).
- `l1_twelve_actuated`; `l1_feet_positions` (exact rationals ±1/5, ±1/7); `l1_gamma_mech_c2xc2`;
  `l1_gamma_obs_c2_lidar_breaker`; `l1_archetype_quad_mammal`; `l1_no_declared_inertia`; `l1_levers_in_bounds`.
- `l1_mutation_<code>` for 103, 105, 107, 109, 111 (bounds), 111 (lock), 125, 126: each returns its refusal
  with a pointer to the mutated field.
- `l1_constructors_unexpanded`: the canonical text contains two `mirror` keys and no `leg.fl` identifier.

### End-to-end script and logging

`scripts/e2e/art_l1_fbot.sh` (header: `bead=ART-L1 milestone=M1 labels=golden,appendix-l,skeleton,
verb:design.new,verb:design.show,verb:design.view,verb:design.check,verb:analyze.symmetry seed=1 needs=cpu
plants=ratio-as-decimal,lidar-centred-assert`). Steps: (1) `frob design new quad-tray --file
fixtures/golden/appendix-l/L1/quad-tray.resolved.fbot.json --key l1-new --json`, exit 0, capture the
`DesignId`; (2) `frob design show quad-tray --json`, assert the derived facts with `e2e_assert_json` and
`e2e_assert_rational`; (3) `frob design view quad-tray --as sexpr`, then `--as json` on its output, assert
digest equality with the canonical file; (4) `frob analyze symmetry quad-tray --json`, assert both groups and
the breaker; (5) `frob design check quad-tray --json`, assert closure and symmetry; (6) run the mission steps
of artifact L.4's script in the same scratch root, then `frob design new quad-tray-cell --from-cell
<mission>/c:06a --key l1-cell --json`, assert half-width, ratio and model identity; (7) the eight mutations
through `e2e_expect_refusal <code> 3`, each through the verb that refuses it, as above: `design new --file` for
103 and 125; `design check` on the design stored from the mutated file for 105, the lever value of 111, 126,
107 and 109 (each such file is first loaded with `design new --file` under a name and a key of its own, and for
the three stage-1 codes that call's exit 2 and finding are asserted); `design edit` for the lock;
(8) `frob-golden check --artifact L1`. Plants: `ratio-as-decimal` feeds
`"0.142857 m"` and must fail the identity assertion; `lidar-centred-assert` asserts `assert_obs: "C2xC2"` and
must be refused with 109. The log (`target/e2e-logs/art_l1_fbot/<utc>.ndjson`) records every command with exit
code and wall time, the envelope's evidence block, digests where state is compared, each assertion with
observed and expected values, each refusal with code and first repair, and the `summary`.

### Known answers and negative controls

- Known answers (exact; hand-checkable): the pointer table; twelve coordinates; feet at (±1/5, ±1/7) m; the
  orders 4 and 2 of the two groups; v = 7 and r = 47/20 at the cell centre. They are rows JL-06 (one identity
  for every spelling, with the pointer table) and JL-07 (the derived facts) of NJLx-02's known-answer block (a
  dependency), which this bead's tests cite by number once the block is issued, with the rows' controls (a
  reader built with key sorting disabled; the mechanical group used as the policy's symmetry).
- Counterexample this fixture keeps alive: taking the mechanical symmetry group as the policy's symmetry. A
  quadruped with one forward lidar has the four-element group mechanically and a two-element group in closed
  loop; only the observation-preserving subgroup may be used for weight sharing and deduplication.
- Negative controls: the eight mutations; the two plants; a reader built with key sorting disabled must fail
  `l1_roundtrip_value`.
- Metamorphic relations: key order, white space and unit respelling leave the identity unchanged; mirroring
  the lidar mount to x = −0.21 m leaves Γ_obs of order 2 (the same subgroup).
- Determinism lanes: twice on one machine; debug and release; Linux and macOS; all class D0.

## Considerations

- The card names (`qdd-m`, `qdd-l`, `li-6s-5ah`, `imu-generic`, `encoder-14b`, `foot-contact`,
  `lidar-2d-generic`, `al-6061-t6`, `cfrp-ud`, `rubber-60a`, `pp-homopolymer`) must exist in the generic
  catalog shipped with the first milestone. If the catalog's values make the design fail a preflight stage,
  the catalog and this fixture are reconciled by an ADR; the check is not weakened.
- The field spellings are the owner chapter's. One spelling is not fixed anywhere: the tray's `mount` and
  `removable` fields follow the payload row of the field reference.
- Kill gate KILL-SKEL-SEXPR (the S-expression view cannot be kept lossless): fallback, JSON is the only
  syntax; the view round trip of this bead is then recorded as not applicable and the rest stands.
- The fixture is public: it derives from no private scene and no licensed asset (its `meta.licence` is
  CC0-1.0).

## References

Appendix L.1; §14.4 (field reference and canonical form), §14.5, §14.9, §14.11, §13.4, Appendix G.4;
INV-SKEL-04 (both views have one `DesignId`); PO-SKEL-01 (canonicalization is idempotent); GATE-SKEL-05 and
GATE-WS-02 are measured by their own beads on random designs.
