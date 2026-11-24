# Polishing round 3, slice F-U29b

20 beads.

| Key | Milestone | Type | Parent | Changed last round | Title | File |
|---|---|---|---|---|---|---|
| N29b-01 | M1 | task | WP-13 | yes | frob-chronicle: energy-ledger edge cases: phase attribution, signed parts, per-task exclusions, residuals | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-01.md` |
| N29b-02 | M1 | task | WP-13 | yes | frob-chronicle: the created-energy monitor, residual sign over sliding 0.5 s windows and the flag energy-created | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-02.md` |
| N29b-05 | M2 | task | WP-50 | yes | frob-chronicle: failure monitors on batched tiers, first-anomaly latches, ring flush and outcome-kind counts | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-05.md` |
| N29b-06 | M2 | task | WP-50 | yes | e2e: scripts/e2e/gate_chron_08.sh: failure accounting on batched tiers, the comparison control, one gate record | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-06.md` |
| N29b-07 | M2 | task | WP-50 | yes | frob-chronicle: the tier-correction ledger store, stratum rows from exact paired moments, ρ_ep and referee reach | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-07.md` |
| N29b-08 | M3 | task | WP-50 | yes | frob-chronicle: tier-correction audit rounds, INV-CHRON-13 by write order, and the FROB-REF-811 read guard | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-08.md` |
| N29b-09 | M1 | task | WP-43 | yes | decision OPEN-CHRON-3: artifact-graph identifier types, imported from franken_kernel or local in frob-core | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-09.md` |
| N29b-10 | M1 | task | WP-43 | yes | frob-verbs, frob-chronicle: the citation guard on every surface: claim states, citing operations, class checks | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-10.md` |
| N29b-11 | M2 | task | WP-43 | yes | simulation testing, second wave: pagination, retention, memo store, shard takeover; the 10⁵-schedule lane | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-11.md` |
| N29b-12 | M2 | task | WP-51 | yes | decision OPEN-CHRON-4: retention windows for failure captures and the default store budget on each machine | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-12.md` |
| N29b-13 | M2 | task | WP-51 | no (here for a request) | frob-chronicle: retention edge cases: pins re-checked at sweep, every claim writer pins, withdrawal as a surface | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-13.md` |
| N29b-14 | M3 | task | WP-58 | yes | frob-chronicle: telemetry of a private scene keeps the scene's privacy class through retention, export and purge | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-14.md` |
| N29b-15 | M2 | task | WP-43 | yes | frob-conformance: Chronicle invariants INV-CHRON-01 to -13 as registered property tests with planted violations | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-15.md` |
| N29b-16 | M2 | task | WP-43 | yes | Chronicle proof obligations PO-CHRON-01 to -07: derivations on file, exhaustive and statistical discharges | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-16.md` |
| N29b-17 | M2 | task | WP-51 | yes | experiment OPEN-CHRON-1: column compression without a third-party codec, measured ratios and the decision | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-17.md` |
| N29b-18 | M1 | task | WP-43 | yes | decision OPEN-CHRON-2: the chunk length L and the depth of the device ring, from costs computed and measured | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-18.md` |
| N29b-21 | M1 | task | WP-39 | yes | frob-dyn: predictor bookkeeping in the step; the energy split for joint-limit, stiction and backlash rows | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-21.md` |
| N29b-22 | M1 | feature | WP-13 | yes | frob-chronicle: ledger line algorithmic exchange; thermal and wear states take constitutive heat only | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-22.md` |
| N29b-23 | M1 | task | WP-13 | yes | e2e: scripts/e2e/gate_chron_14.sh on the referee: rest, slide and impact at 32, 64, 2,000 Hz; heat mutant | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29b-23.md` |
| N29bp1-01 | M1 | task | WP-43 | yes | frob-conformance: re-affirmation of recorded PASS parts and bead closing lines against the live artifact graph | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29b/N29bp1-01.md` |

## Addressed to your beads

- `N29b-06`: request from round 3, slice F-U22a: It writes GATE-CHRON-08's one row at M2; later classes extend the fixture list after their monitors land (gait-instability by N22ap2-03 at M4, model-drift by BD-394 at M5, slosh-tip by N19b-13 at M6), each with a tier run receipt. Convention 9 allows one recorder per gate: state who records a later class's re-run from its receipt (for example a part per class recorded by the switching bead), so N22ap2-03's recording has a named owner.
- `N29b-06`: request from round 3, slice F-U24a-2: Take N24a-39's receipt as GATE-CHRON-08's referee rows and record the gate's one row with record_gate; and name, in the gate register, which bead records a later wave's extension of the fixture list (N24a-38 wave 1 at M3; N22ap2-03 wave 2 at M4).
- `N29b-13`: request from round 3, slice H-verbs-7: Name a code (an integrity 9xx through N35-33's ledger) for the store.collect pre-mark audit abort (audit::pins_cover_live_claims), so the row can list it.
- `N29b-23`: request from round 3, slice H-verbs-6: Use uppercase telemetry levels; lowercase is 802.
- `N29bp1-01`: request from round 3, slice H-owners-1: Use `cargo run -p frob-conformance --example record_gate -- reaffirm ...` (N38b-19 provides the mode-word dispatch and the shared lock; this bead adds the reaffirm mode and its tests), not a `frob-conformance` binary or a separate recorder command.
