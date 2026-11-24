# Polishing round 3, slice F-U14a

33 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| BD-055 | M1 | task | WP-07 | yes | .fbot fields: fill with no default, power bus and sink, sensors, payloads, budgets, levers, locks, symmetry | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-055.md` |
| BD-059 | M1 | feature | WP-07 | yes | verb design.patch (`frob.design.patch/1`): typed edits on an expected parent, with locks and idempotency key | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-059.md` |
| BD-066 | M1 | feature | WP-08 | yes | frob-io: MJCF import with a loss report (design.import --format mjcf), integrating export; GATE-SKEL-01 rows | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-066.md` |
| BD-077 | M1 | feature | WP-38 | yes | verb module.substitute: leaf-local or global by four conditions, and the reuse table of C-12 (GATE-SKEL-06) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-077.md` |
| BD-227 | M3 | feature | WP-23 | yes | frob-scene: hazard registries `registries/hazards/<archetype>.toml` with provenance; the verb scene.hazards | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-227.md` |
| BD-245 | M3 | feature | WP-38 | yes | frob-contact: laws `belt`, `skid`, `pad`, `roller-wheel`, `wheel-rolling`, all tiers; scrub-work known answer | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/BD-245.md` |
| N14a-01 | M1 | task | WP-07 | yes | frob-design: exact quantity atoms, canonical numbers (rules 2 to 5) and the parameter record | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-01.md` |
| N14a-03 | M1 | task | WP-07 | yes | .fbot top-level and Body blocks: Regions, material, declared mass properties, collision and visual, ports | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-03.md` |
| N14a-06 | M1 | task | WP-07 | yes | frob-design: the lever interface for optimizers and the lock rule (ADR-SKEL-07, INV-SKEL-08, FROB-REF-111) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-06.md` |
| N14a-07 | M1 | task | WP-07 | yes | frob-model: elaborate stage: expanded instance tree, resolved parameters, instance identifiers, source paths | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-07.md` |
| N14a-10 | M1 | task | WP-07 | yes | frob-model: the budget functor: per-subtree summaries, the budget graph, and budget findings (FROB-REF-115) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-10.md` |
| N14a-11 | M1 | task | WP-07 | no (here for a request) | frob-model: the symmetry functor and the assertion check: computed Γ_mech and Γ_obs, FROB-REF-109, INV-SKEL-07 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-11.md` |
| N14a-12 | M1 | task | WP-07 | yes | frob-model: lowerings 3 and 4: analysis views (limb kinematics, limb permutation, budget graph), shape summary | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-12.md` |
| N14a-14 | M1 | task | WP-07 | no (here for a request) | frob-model: union, difference and mesh Regions; mesh shells; collision primitives fitted with a recorded error | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-14.md` |
| N14a-15 | M1 | task | WP-38 | yes | Pilot: module-substitution reuse: hand-listed reusable products against a full-recompile diff, per archetype | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-15.md` |
| N14a-16 | M1 | task | WP-38 | yes | Kill-gate decision KILL-SKEL-SUBST: keep leaf-local reuse under substitution, or make all substitutions global | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-16.md` |
| N14a-17 | M1 | task | WP-07 | yes | Typed-edit catalogue conformance: 23 rows, declared symmetry effect and predicted invalidation scope tested | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-17.md` |
| N14a-18 | M1 | task | WP-07 | yes | frob-design: schema versions, canonicalization revision, the `migrate` edit, the `.fbot` artifact envelope | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-18.md` |
| N14a-21 | M1 | feature | WP-07 | yes | verbs design.new, design.show: create a design from a file, template or mission cell; read compiled sections | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-21.md` |
| N14a-22 | M5 | task | WP-72 | yes | frob-model: the limb port packet (`LimbPortPacket`, `SymPsd6`): layout, reference recursion, PO-SKEL-04 | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-22.md` |
| N14a-23 | M5 | task | WP-72 | yes | Pilot: the limb port interface by hand: packet from the recursion against the reaction from the full equations | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-23.md` |
| N14a-24 | M5 | task | WP-72 | yes | Kill-gate decision KILL-SKEL-PORT: keep limb port packets, or route all dynamics through the whole-model path | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-24.md` |
| N14a-25 | M1 | feature | WP-08 | yes | frob-io: URDF and MJCF export with the reverse loss report, and the verb design.export | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-25.md` |
| N14a-26 | M1 | task | WP-07 | yes | tools/oracle: independent reference canonicalizer and golden corpus that pin `DesignId` for `frob.design/1` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-26.md` |
| N14a-27 | M1 | task | WP-07 | yes | e2e: `scripts/e2e/skel_author_compile.sh`: author, compile, read every section, refuse planted stage-1 defects | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-27.md` |
| N14a-28 | M1 | task | WP-07 | yes | e2e: `scripts/e2e/skel_view_roundtrip.sh`: design.view round trips in both directions (GATE-SKEL-05) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-28.md` |
| N14a-29 | M1 | task | WP-07 | yes | e2e: `scripts/e2e/skel_patch.sh`: design.patch stale parents, key replays, locks, migration (GATE-SKEL-08) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-29.md` |
| N14a-30 | M1 | task | WP-38 | yes | e2e: `scripts/e2e/skel_substitute.sh`: module.substitute controls, random edits, time saving (GATE-SKEL-06) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-30.md` |
| N14a-31 | M1 | task | WP-08 | yes | e2e: `scripts/e2e/skel_import_mjcf_export.sh`: MJCF import parity, export round trips (GATE-SKEL-01 MJCF rows) | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-31.md` |
| N14a-32 | M3 | task | WP-23 | yes | e2e: `scripts/e2e/scene_hazards.sh`: scene.hazards, a pack's `assumed` count and the realism-claim refusal | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-32.md` |
| N14a-33 | M3 | task | WP-38 | yes | e2e: `scripts/e2e/contact_selectors.sh`: five contact-law selectors on every tier, scrub work and controls | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-33.md` |
| N14ax-01 | M3 | task | WP-38 | yes | .fbot block `travel`: declared travel postures, safety margin and travel capabilities with provenance | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14ax-01.md` |
| N14ax-02 | M3 | task | WP-38 | yes | frob-design: typed edits record the parent-to-child correspondence of frames, actuators, sensors and contacts | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14ax-02.md` |

## Addressed to your beads

- `BD-227`: request from round 3, slice H-owners-1: `hazard-registry-invalid` is now a N35x-02 row (3xx, interim 802 with detail); no change of behaviour.
- `N14a-11`: request from round 2, slice S26: N13-15's input now has the required field `base: Free | Fixed` (a free-base tree is re-rooted at its centre so that root-moving automorphisms such as a chain reversal are found): pass it from the design's base kind, and note that reported indices and signs stay in the design's own rooting
- `N14a-14`: request from round 3, slice H-owners-1: Raise the Region refusal behind a constant for `region-not-admissible` (N35x-02 row; interim 102 with pointer).
- `N14a-21`: request from round 3, slice H-verbs-1: Align its entry sketches with the registry. design.new is a mutate, so idempotent = false; its stage-1 codes 105, 121 and 126 are findings (exit 2, the design is stored), not refusals; 111 is a refusal. design.show's chapter codes are 102, 103 and 125 (734 and 828 are appended by the field rule)
- `N14ax-01`: request from round 3, slice H-owners-1: Raise the two travel refusals behind constants named for `travel-value-out-of-domain` and `travel-envelope-optimistic` (N35x-02 rows; interim 111 with the pointers).
