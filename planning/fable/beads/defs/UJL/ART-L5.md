---
key: "ART-L5"
title: "golden fixture L.5: the `.fenergy` budget for the stop at the pass counter, its integer arithmetic and CLI script"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "reflex", "frob-shield", "frob-conformance", "frontier", "test", "e2e"]
parent: "WP-65"
deps: ["BD-337", "BD-338", "ART-L1", "NJL-07", "STD-E2E"]
needs: []
related: ["N22b-18", "BD-092", "BD-132", "BD-410", "ART-L9"]
gates: []
plan_refs: ["Appendix L.5", "§22.11", "§12.4", "§8.2"]
---
## Background

FrankenRobotics (a robotics simulation and design system reached through the command line `frob`) has an
**energy-budget safety tier**. Certifying a contact sequence is hard; bounding the energy that can reach a
boundary is not. Passive contact, with any friction coefficient μ ≥ 0, any restitution between 0 and 1 and any
sequence of impacts, can dissipate or redistribute mechanical energy and cannot create it. So if everything
that could deliver energy to a boundary is bounded, and the absorber at that boundary can take that much, the
stop holds for every contact sequence, and no contact model appears in the argument. The budget is a file,
`.fenergy` (format FMT-FENERGY, schema `frob.energy/1`), built and admitted by the verb `shield.budget` and
replayed against runs by `shield.check`. The mechanism is Frontier, with kill gate KILL-ENERGY.

This bead turns the fifth worked artifact of the specification into a golden fixture. In the nine-artifact
story (a quadruped carries a tray through a commercial-kitchen galley and sets it down at a pass counter) the
boundary is a bumper at the pass counter, and the budget bounds the energy the robot can deliver to it during
a stop, whatever the contact sequence and the friction. The numbers are DESIGN specimens; the arithmetic is
exact and is the content of the fixture.

Why this fixture matters (DESIGN). The budget's predicate is an exact-integer inequality, evaluated identically
on every backend and recomputed by the offline auditor. A fixture whose every term is a small integer is the
known answer for the kernel, for the verb and for the auditor at once. It also pins the rule that the weakest
input sets the class: the arithmetic is exact, the absorber envelope is `assumed`, so the claim is
`Estimated`. Rejected: testing the monitor only on the chapter's cart example (it has no legs, no tray and one
drive; this one has six bodies on the load path).

## Scope

In: the specimen and resolved files; the budget inequality and each term; the integer arithmetic at a monitor
reading of 0.50 m/s and at the two boundary readings; the longest admissible cutoff delay; the premises and
the class of the claim; refusals on planted mutations; the script. Out: the budget format, the monitor kernel
and the verb (BD-337); shields with a backup controller and the replay check (BD-338); the auditor's checker
for the proof kind `energy-budget` (the certificate beads); the layers and the identity map (NJL-07).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L5/`: `pass_counter_stop.specimen.fenergy.json` (the text below);
`pass_counter_stop.resolved.fenergy.json`; `arithmetic.expect.json`; `fields.toml`.

**The artifact.**

```json
{
  "schema": "frob.energy/1",
  "boundary": "pass_counter_stop",
  "load_path": ["torso", "tray", "leg_fl", "leg_fr", "leg_rl", "leg_rr"],
  "unit_scale": { "energy": "1e-6 J", "time": "1e-6 s" },
  "terms": {
    "kinetic_ub": { "mass_cap_kg": "18", "speed_error_mps": "0.02", "includes_reflected_inertia": true },
    "drive": { "p_max_w": "40", "cutoff_delay_s": "0.030" },
    "external": [{ "source": "gravity", "bound_j": "0.11", "basis": "slope <= 0.02 over 0.030 m" }],
    "stored": [{ "source": "leg_series_compliance", "bound_j": "0.15" }],
    "reserve_j": "0.10"
  },
  "absorber": { "card": "bumper-150N-30mm", "w_abs_j": "4.50", "provenance": "assumed" },
  "premises": { "sources_complete": "port-graph-digest", "absorber_on_path": "declared-guide" },
  "claim": { "class": "Estimated", "reality": "no-claim", "flags": "the no-authorization flags of §12" }
}
```

**The inequality.** For a declared boundary and a time t:

    E_deliv(t) = T_ub(t) + E_drive(t) + E_ext + E_stored(t) + E_reserve  ≤  W_abs

- T_ub: an upper bound on the kinetic energy of every body on the load path, ½·m̄·(v + δv)², with the measured
  speed v inflated by its sensing error δv and the mass at its cap m̄, reflected rotor inertia included.
- E_drive: what the drives can still inject before the cutoff acts, bounded by P_max·Δt_cut, where Δt_cut is
  detection plus transport plus driver plus brake delay.
- E_ext: an upper bound on work done by sources that are not cut (here gravity over the absorber stroke).
- E_stored: elastic energy that can be released onto the boundary (here the series compliance of the legs).
- E_reserve: numerical and sensing reserve, at least the energy ledger's residual bound over the window.
- W_abs: the lower envelope of the work the absorber takes before its limit (here 150 N over 30 mm).

Quantities are decimal strings, so that no binary rounding enters, and the monitor works in integers of the
declared unit scales (10⁻⁶ J and 10⁻⁶ s).

**The arithmetic at a monitor reading of 0.50 m/s, in integers of 10⁻⁶ J.**

| Term | Calculation | Value |
|---|---|---|
| T_ub | ½ · 18 kg · (0.50 + 0.02)² m²/s² | 2,433,600 |
| E_drive | 40 W · 0.030 s | 1,200,000 |
| E_ext | gravity: 18 · 9.81 · 0.030 · 0.02 = 0.106 J, bounded by 0.11 J | 110,000 |
| E_stored | leg series compliance | 150,000 |
| E_reserve | at least the ledger's residual bound over the window | 100,000 |
| Sum | | 3,993,600 |
| W_abs | 150 N over 30 mm | 4,500,000 |

The predicate 3,993,600 ≤ 4,500,000 holds with a margin of 506,400, that is 0.5064 J.

**Further known answers derived from the same file** (`arithmetic.expect.json`):

| Quantity | Calculation | Value |
|---|---|---|
| Longest admissible cutoff delay at 0.50 m/s | (4.50 − 2.4336 − 0.11 − 0.15 − 0.10) / 40 s | 0.04266 s = 42,660 µs (declared: 30,000 µs) |
| Predicate at a cutoff delay of 42,660 µs | E_drive = 1,706,400; sum = 4,500,000 | margin 0: admitted, at the limit |
| Predicate at a cutoff delay of 42,661 µs | E_drive = 1,706,440; sum = 4,500,040 | margin −40: refused |
| Reading 0.55 m/s | T_ub = ½·18·0.57² = 2.9241 J; sum = 4,484,100 | margin +15,900: admitted |
| Reading 0.56 m/s | T_ub = ½·18·0.58² = 3.0276 J; sum = 4,587,600 | margin −87,600: the monitor cuts |
| Gravity term before bounding | 18 · 9.81 · 0.030 · 0.02 J | 0.105948 J, bounded above by 0.11 J |

**The monitor.** One accumulator and one comparison per control tick, in the exact-integer kernel class
(kernel `K-energy-monitor`): it outputs the margin W_abs − E_deliv; at zero margin it cuts drive power and
hands over to the backup controller. The longest admissible cutoff delay is the closed form
Δt_cut,max = (W_abs − T_ub − E_ext − E_stored − E_reserve) / P_max.

**Premises, each checked or declared in the file.** (1) Sources are complete: every port of the design's port
graph that can inject power into the load path is in the budget with a bound, or is cut by the monitor;
`shield.budget` walks the port graph and refuses a budget with an unlisted source (`FROB-REF-451`). The
`premises.sources_complete` field holds the digest of that port graph. (2) Caps are card values with
provenance. (3) The cutoff acts within Δt_cut; if the packet-delay distribution has a tail beyond it, the
claim becomes a probability and the artifact says so. (4) The absorber is on every forward path to the
boundary: a geometric premise, here a declared guide.

**The class of the claim.** "For every contact sequence and friction coefficient, the energy delivered to the
declared boundary during a stop does not exceed the absorber's stated minimum capacity, in the model, under
premises 1 to 4." The arithmetic is exact, and the claim's class is that of its weakest input: the absorber
envelope has provenance `assumed`, so the claim is `Estimated`, not `Certified`; requesting a class above
`Estimated` on an `assumed` absorber is refused (`FROB-REF-451`). Reality is `no-claim`. One bench measurement
of the bumper and one of the drive would raise the class for every later policy on this hardware, since no
term depends on the policy.

**What the budget does not support.** It answers an energy question. It supports no statement about peak
force, about clamping after the stop (a drive that keeps pushing poses a force question), about contacts that
bypass the bumper, or about sources outside the port graph (a second robot, a falling object, a person
pushing); and it is no regulatory assessment. The artifact carries the three no-authorization flags, all
false.

**A spelling to settle.** The specimen writes the last field of `claim` as `flags`, with a prose placeholder;
the owner chapter's sketch of the same format writes `authorization`. DESIGN default for the resolved layer:
the key `authorization`, holding the object `{"execution_authorized": false, "hardware_safety_signoff": false,
"sim_to_real_certified": false}`; the specimen layer stays as printed. If the format's owner bead (BD-337)
chooses otherwise, the resolved layer follows it.

## Acceptance criteria

- [ ] The specimen equals the text above byte for byte (manifest SHA-256); the resolved file parses strictly,
      validates against `frob.energy/1` and round-trips with zero byte differences.
- [ ] `frob shield budget` for `quad-tray` and the boundary `pass_counter_stop` produces a budget whose terms
      equal the fixture's, and admits it; the envelope's evidence is class Estimated with the absorber named
      as the weakest input and `reality: no-claim`.
- [ ] The seven integer rows at 0.50 m/s reproduce exactly (2,433,600; 1,200,000; 110,000; 150,000; 100,000;
      3,993,600; 4,500,000), with margin 506,400.
- [ ] The six further known answers reproduce exactly, including the pair 42,660 µs (margin 0) and 42,661 µs
      (margin −40), and the pair 0.55 m/s (admitted) and 0.56 m/s (cut).
- [ ] The offline auditor recomputes the predicate from the file alone and returns `accept` with class
      Estimated; a file whose sum is edited by one unit is rejected.
- [ ] Refusals: a budget with the leg compliance removed while the port graph lists it → `FROB-REF-451`; a
      request for class Certified on the `assumed` absorber → `FROB-REF-451`; a flag missing or true →
      `FROB-REF-833`. Each is exit 3 with one envelope.
- [ ] `frob shield check` on a recorded stop of `quad-tray` at the bumper reports no exceedance and a margin
      trace whose minimum is non-negative (structural: the values depend on the run).
- [ ] The integer results are identical on `batch-cpu`, CUDA and Metal and on two runs (the exact-integer
      kernel class), and the canonical bytes are identical in debug and release, on Linux and macOS.

## Test plan

### Unit tests

`cargo test -p frob-shield --test golden_l5` (the real reader and the real monitor kernel on `batch-cpu`; no
mocks):
- `l5_specimen_parses_and_validates`; `l5_manifest_hash`; `l5_roundtrip_value`.
- `l5_terms_at_050`: the seven integers of the table.
- `l5_margin_506400`.
- `l5_cutoff_max_42660_us`: the closed form in integers; 42,660 µs → margin 0; 42,661 µs → margin −40.
- `l5_reading_boundary_pair`: 0.55 m/s → +15,900; 0.56 m/s → −87,600.
- `l5_gravity_bound`: 0.105948 ≤ 0.11; a declared bound of 0.10 J fails the basis check (the work computed from
  the stated slope and stroke exceeds it).
- `l5_class_estimated_from_assumed`: with absorber provenance `assumed` the class is Estimated; with
  `measured` (a fixture card) the class rises to Certified for the arithmetic.
- `l5_unlisted_source_refuses_451`; `l5_class_request_refuses_451`; `l5_flag_true_refuses_833`.
- `l5_no_binary_rounding`: every term parses to an exact integer of the unit scale; `"0.11"` J is 110,000.
- Property `prop_monitor_matches_rational`: for random admissible terms the integer monitor equals the exact
  rational evaluation of the inequality.

### End-to-end script and logging

`scripts/e2e/art_l5_fenergy.sh` (header: `bead=ART-L5 milestone=M4 labels=golden,appendix-l,reflex,
verb:shield.budget,verb:shield.check seed=1 needs=cpu plants=payload-only,sum-off-by-one`). Steps: (1) create
`quad-tray` from artifact L.1's fixture; (2) `frob shield budget quad-tray --boundary pass_counter_stop
--absorber bumper-150N-30mm --cutoff-delay 0.030s --reserve 0.10J --key l5-bud --json`, compare the written
file with the resolved fixture (`e2e_assert_digest_eq`) and assert class Estimated; (3) evaluate the predicate
at the readings 0.50, 0.55 and 0.56 m/s through the verb's `--at-speed` option and assert the integer terms
and margins with `e2e_assert_num` (exact); (4) the cutoff pair 42,660 and 42,661 µs; (5) `frob prove bundle`
for the budget and the offline `frob-verify` on it, `accept`, class Estimated; (6) the three refusals through
`e2e_expect_refusal`; (7) a referee run of a stop at the bumper, then `frob shield check <run> --budget …
--json`, assert no exceedance; (8) `frob-golden check --artifact L5`. Plants: `payload-only` sets the mass cap
to the tray alone and must show an exceedance in step (7); `sum-off-by-one` must be rejected by the auditor.
The log (`target/e2e-logs/art_l5_fenergy/<utc>.ndjson`) records every command with exit code and wall time,
the evidence block, each integer term as a metric with its unit (µJ, µs) and expected value, digests, refusals
with code and first repair, and the `summary`.

### Known answers and negative controls

- Known answers (exact integers; recomputed for this bead): the tables above. They have no number in the
  known-answer suite; the unit's coverage file lists them for registration.
- Related worked card of the owner chapter (kept by its own beads): a cart of 8 kg at 0.4 m/s with a 20 W
  drive, a 25 ms delay and a 2.00 J absorber: 1.49 J delivered; longest delay 50.5 ms, or 45.5 ms with 0.10 J
  of reserve.
- Counterexample kept alive: budgeting the payload's kinetic energy alone omits the rotors. A rotor of
  inertia 5 × 10⁻⁵ kg·m² behind a 50:1 reduction driving a wheel of radius 0.05 m has an effective mass of
  50² · (5 × 10⁻⁵) / 0.05² = 50 kg; T_ub is therefore computed from the full mass matrix.
- Negative controls: the two plants; the three refusals; a simulated stop under any μ in [0, 5], any
  restitution in [0, 1] and any contact law must never show an exceedance of an admitted budget.
- Metamorphic relation: changing the friction coefficient or the contact law of the stop changes no term of
  the budget.
- Determinism lanes: twice on one machine; `batch-cpu`, CUDA and Metal (bit-identical integers); debug and
  release; Linux and macOS.

## Considerations

- Kill gate KILL-ENERGY (an exceedance under the premises survives one repair cycle at the energy-tier gate):
  fallback, the claim is withdrawn and the monitor remains as an alarm of class Estimated. The fixture's
  arithmetic and steps (1) to (6) stand; step (7) then asserts the alarm, not a guarantee.
- The mass cap of 18 kg is a card value: the design's mass budget is 14 kg and the loaded tray adds the rest.
  If the catalog's caps differ, the fixture's `kinetic_ub` follows the cards and the integers are re-derived
  by a blessed change.
- The names in `load_path` (`leg_fl` and the like) are the specimen's; the compiled design derives instance
  identifiers of its own. The resolved layer uses the derived identifiers, and the story-consistency check
  requires each to resolve.
- The `--at-speed` option of step (3) is a proposal for evaluating the predicate at a stated reading without a
  run; if the owner bead exposes it otherwise, the script follows.

## References

Appendix L.5; §22.11 (the budget, the monitor, the premises, the claim and its limits), §12.4 (proof kind
`energy-budget`), §8.2 (the weakest input sets the class); PO-SHIELD-01 (the inequality is an exact-integer
predicate that the offline auditor recomputes); GATE-SHIELD-01 is measured by the energy-tier beads.
