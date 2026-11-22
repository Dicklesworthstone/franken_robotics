---
key: "ART-L6"
title: "golden fixture L.6: the `.fmcw` minimal counterexample world of three edits, its invariants and CLI script"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "frob-conformance", "frontier", "test", "e2e"]
parent: "WP-55"
deps: ["BD-270", "BD-271", "BD-272", "BD-273", "BD-274", "ART-L1", "ART-L3", "NJL-07", "STD-E2E"]
needs: []
related: ["N26b-13", "BD-279", "BD-369", "BD-318", "BD-410", "ART-L9", "NJL-09"]
gates: []
plan_refs: ["Appendix L.6", "§26.3", "§26.5", "§26.6", "§26.7", "§26.13", "Appendix G.3"]
---
## Background

FrankenRobotics (a robotics simulation and design system reached through the command line `frob`) searches
for adversarial worlds in which a candidate (a design with its controller or policy) fails a task, and then
**shrinks** a failing world to a minimal counterexample: the smallest set of edits to a passing base world
that still produces the same failure. The result is a file, `.fmcw` (format FMT-FMCW, schema `frob.mcw/1`),
written by the verb `nemesis.shrink`. A minimal world is what an engineer argues about, what an autopsy
intervenes on, and what a retrofit of the place tries to fix. Shrinking is Frontier.

This bead turns the sixth worked artifact of the specification into a golden fixture. In the nine-artifact
story (a quadruped carries a tray through a commercial-kitchen galley and sets it down at a pass counter),
`nemesis.shrink` reduced a failing world of seven edits to three: a grease film on one floor patch, a swing
door that closes harder, and a fuller tray. Remove any one and the tray is delivered. The numbers are DESIGN
specimens of the format, not results.

Why this fixture matters, and what it can and cannot pin (DESIGN). The file's structure and its internal
arithmetic are exact and are pinned exactly. Its content (which edits, how many seeds fail, the margin) is the
outcome of simulated episodes; on the real simulator those values will differ, so they are pinned by
invariants, not by value. A fixture that pretended otherwise would either never pass or be regenerated until
it did. Rejected: asserting the specimen's values on produced output (false precision); keeping the specimen
in documentation only (the format would drift unnoticed).

## Scope

In: the specimen and resolved files; what each field means; the invariants every `.fmcw` must satisfy, with
the specimen's numbers as their instance; the seven-edit starting world of the story; the script that
searches, shrinks and re-checks. Out: the search space and legality profile (BD-270), the oracle (BD-271), the
artifact guard (BD-272), the search (BD-273) and shrinking itself (BD-274); the planted-cause recovery gate of
shrinking (measured by those beads); the layers and the identity map (NJL-07).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L6/`: `kitchen-tray.specimen.fmcw.json` (the text below);
`kitchen-tray.resolved.fmcw.json`; `world.seven-edits.json` (the starting world of the story);
`invariants.expect.json`; `fields.toml`.

**The artifact.**

```json
{ "schema": "frob.mcw/1", "id": "w:3fa9…", "task": "t:kitchen-tray@88c1…", "requirement": "delivered",
  "candidate": {"model": "m:5d20…", "controller": "c:2be8…", "policy": "p:6f1a…"},
  "base": {"scene": "s:kitchen-17@a4e0…", "world": {"run_seed": 901, "world": 5120, "episode": 3}},
  "edits": [
    {"op": "set", "path": "/floor/patch[galley-2]/film", "from": "dry", "to": "grease", "changes": ["material"]},
    {"op": "set", "path": "/fixtures/door[pass]/closer_torque", "from": "14 N·m", "to": "19 N·m",
     "changes": ["fixture"]},
    {"op": "set", "path": "/payloads/tray/fill", "from": "0.50", "to": "0.80", "changes": ["payload"]} ],
  "minimality": {"kind": "1-minimal", "pairs_exhausted": true, "tests": 42, "seeds_failing": "55/64"},
  "failure": {"class": "slip", "binding_atom": "upright", "margin": -0.42, "time_s": "11.3"},
  "replay": {"backend": "gpu-cuda", "class": "D2", "state_digests": "b3:0c55…"},
  "promotion": {"tier": "referee", "reproduced": true,
                "variants": ["half-step", "iterations-x2", "law-2", "tol-div-10"]},
  "oracle": {"class": "solvable-causal", "controller": "c:77e1…", "margin": 0.18, "trace": "b3:a9d2…"},
  "legality": {"inside": true, "touched": [{"bound": "closer_torque", "provenance": "assumed"},
                                           {"bound": "tray_fill", "provenance": "assumed"}]},
  "prior": {"log10_density_ratio": -4.6},
  "neighbours_passing": ["w:51b0…", "w:e7c4…", "w:0a9d…"],
  "evidence": {"class": "Tested", "tier": "referee", "standing": "established"},
  "authorization": {"execution_authorized": false, "hardware_safety_signoff": false,
                       "sim_to_real_certified": false} }
```

**Field by field.** `task`, `requirement`: the task of artifact L.3 and its verdict `delivered`. `candidate`:
model, controller and policy identities. `base`: the scene and the world address (run seed, world, episode) of
the passing base world. `edits`: atomic edits, each `{op, path, from, to, changes}`; `changes` declares the
causal aspect of the scene the edit touches (`material`, `fixture`, `payload`), so that each edit can serve as
one intervention in an autopsy. `minimality`: the kind, whether all single and pair subsets were tested, the
number of tests, and the paired seeds on which the failure appears. `failure`: class, binding atom, margin and
time. `replay`: backend, determinism class and state digests. `promotion`: the tier on which the failure was
reproduced and the numerical variants it survived. `oracle`: the solvability class with a witness controller
and its margin. `legality`: whether the world is inside the declared profile and which touched bounds have
provenance `assumed`. `prior`: the log density ratio of the world against the nominal law.
`neighbours_passing`: the worlds with one edit removed. `evidence`, `authorization`: class, tier, standing and
the three no-authorization flags.

**How shrinking defines the content** (so that the invariants below can be read).
- Test relation: a subset of the edits *fails* when the world built from it is legal, replays the candidate
  deterministically, violates the same requirement atom with the same failure class, and does so on at least
  48 of 64 paired seeds (TARGET). A subset that fails by another mechanism is a separate finding, not the same
  failure.
- Algorithm: delta debugging. Partition the edits into k parts, starting at k = 2; if some part fails, recurse
  on it; otherwise if some complement fails, recurse on it with k − 1 parts; otherwise double k; stop when k
  exceeds the number of edits. The result is 1-minimal. Then each remaining continuous edit is moved toward
  its nominal value by bisection until the failure is lost, and the last failing value is kept.
- Minimal means 1-minimal under removal of atomic edits, and nothing stronger. When the original list has at
  most 12 edits, every subset of one or two edits is also tested (at most 78 tests), and a world that passes
  this is labelled *minimum among pairs*. No world is called the closest failure, because failure sets are
  not monotone.
- Size: TARGET at most three atomic edits; a larger result is kept and labelled `non-minimal(k)`.

**Reading the specimen.**
1. Three edits, three mechanisms: grease on patch `galley-2` (material), the pass door's closer torque from 14
   to 19 N·m (fixture), the tray fill from 0.50 to 0.80 (payload).
2. 1-minimal, and minimum among pairs: the original list had seven edits, so 7 singles and 21 pairs were
   tested (28 of the 42 tests); no single edit and no pair fails.
3. The same failure: the atom `upright` with class `slip` on 55 of 64 paired seeds (at least 48 needed). The
   margin −0.42 is (12 − 17.04)/12 for a tilt of 17.04° against the atom's 12° scale.
4. Not an artifact of the fast tier: reproduced on the referee and under four numerical variants (half step,
   doubled iterations, the second contact law, tolerance divided by ten).
5. Not an impossible world: the oracle found a causal controller that delivers the tray in this world with
   margin 0.18, so the world is solvable and the failure belongs to the candidate.
6. Two touched bounds are `assumed`: the closer torque and the fill level sit inside the legality profile at
   values whose ranges no cited norm supports; a report that cites the world says so.
7. It carries what replays it: a file that does not replay with identical digests on its originating backend
   is invalid.

**Invariants of every `.fmcw`** (`invariants.expect.json`; checked on the resolved specimen and on produced
files):

| Invariant | In the specimen |
|---|---|
| Every edit has `op`, `path`, `from`, `to` and a non-empty `changes`; no two edits share a path | three edits; `material`, `fixture`, `payload` |
| `neighbours_passing` has exactly one world per edit, each the world with that edit removed, and each passes on replay | three neighbours |
| `seeds_failing` is `a/b` with b = 64 and a ≥ 48 | 55/64 |
| If `pairs_exhausted` and the original list had n ≤ 12 edits, `tests` ≥ n + n·(n − 1)/2 | n = 7: 28 ≤ 42 |
| `failure.binding_atom` is an atom of the task's requirement; `failure.margin` < 0 | `upright`, −0.42 |
| `oracle.class` is `solvable-causal` or `plant-feasible`, with a witness and a margin > 0 | `solvable-causal`, 0.18 |
| `legality.inside` is true; every touched bound of provenance `assumed` is listed | two listed |
| `evidence.tier` is `referee` and `standing` is `established` only if `promotion.reproduced` is true | true |
| `evidence.class` is at most Tested | Tested |
| The three authorization flags are present and false | present, false |
| Replaying the file on its originating backend reproduces `replay.state_digests` | checked on produced files |
| The number of edits is at most three, or the file is labelled `non-minimal(k)` | three |

**The seven-edit starting world of the story** (`world.seven-edits.json`). The specification gives the three
surviving edits and says only that the original list had seven. DESIGN: the fixture adds four decoy edits
chosen off the robot's route, so that the story can be run: a water film on a floor patch in the storage
room; the closer torque of the storage-room door from 14 to 16 N·m; a clutter box moved 0.2 m in the dish
area; the lighting level of the galley halved. All seven lie inside the legality profile
`kitchen-commercial/default`. If a decoy proves causal on the real simulator, shrinking keeps it; the
invariants still decide the test.

**Refusals on planted defects.** Shrinking a failure that does not replay, or that the artifact guard labelled
an artifact candidate: `FROB-REF-564`. A world or edit outside the legality profile: `FROB-REF-360`, naming
the violated bound. An edit that does not declare the causal paths it changes: `FROB-REF-362`. A request that
demands `cited` bounds while a touched bound is `assumed`: `FROB-REF-361`. A file whose flags are missing or
not false: `FROB-REF-816`. Offering the world as a confirmation sample: `FROB-REF-567`.

## Acceptance criteria

- [ ] The specimen equals the text above byte for byte (manifest SHA-256); the resolved file parses strictly,
      validates against `frob.mcw/1`, round-trips with zero byte differences and satisfies every invariant of
      the table.
- [ ] From the command line: the candidate passes the base world; `nemesis.search` or the seven-edit world
      yields a failing world; `nemesis.shrink` writes a `.fmcw` that satisfies every invariant, including
      replay with identical digests and one passing neighbour per edit.
- [ ] If the seven-edit world does not fail in this build, the step is logged as a skip with that reason (never
      as a pass), and the invariants are asserted on the shrink of the search's own worst world.
- [ ] Each planted violation of an invariant in a copy of the resolved file is rejected by the reader or by
      `frob-golden check`: a fourth edit with three neighbours; `seeds_failing: "40/64"`; `tests: 20` with
      `pairs_exhausted: true`; an oracle class `unsolvable`; a true flag; a duplicate path.
- [ ] The six refusals of the list above fire on their planted requests with the stated codes.
- [ ] The produced file is identical on two runs on one backend (determinism class D2 for the replay), and its
      canonical bytes do not depend on debug or release builds.

## Test plan

### Unit tests

`cargo test -p frob-nemesis --test golden_l6` (the real reader, validator and the delta-debugging routine on a
`synthetic` failure predicate; no mocks of the simulator are used where a replay is asserted: those assertions
are in the script):
- `l6_specimen_parses_and_validates`; `l6_manifest_hash`; `l6_roundtrip_value`.
- `l6_invariants_hold`: the twelve rows on the resolved specimen.
- `l6_invariant_violation_<n>`: one mutated copy per invariant row that can be checked statically; each fails
  and names the row.
- `l6_margin_matches_task`: −0.42 = (12 − 17.04)/12 with the scale read from the task fixture.
- `l6_pairs_count`: for n = 7, 7 + 21 = 28; for n = 12, 12 + 66 = 78.
- `l6_delta_debugging_seven_to_three` (`synthetic` predicate "fails iff edits 1, 2 and 3 are all present"):
  the routine returns {1, 2, 3}, 1-minimal, and the pairs pass finds no smaller subset.
- `l6_one_minimal_is_not_minimum` (`synthetic` predicate "fails iff a and b, or c, d and e", order (a, c, d,
  e, b)): delta debugging ends at {c, d, e}; the pairs pass replaces it with {a, b}.
- `l6_different_mechanism_is_separate`: a subset that fails another atom is recorded as a separate finding and
  does not count as failing.

### End-to-end script and logging

`scripts/e2e/art_l6_fmcw.sh` (header: `bead=ART-L6 milestone=M3 labels=golden,appendix-l,nemesis,
verb:nemesis.search,verb:nemesis.oracle,verb:nemesis.shrink seed=901 needs=cpu,cuda,frobd
plants=drop-neighbour,non-replaying`). Steps: (1) create `quad-tray`, its baseline controller and the task from
the fixtures of artifacts L.1 and L.3; (2) run the candidate in the base world (64 paired seeds), assert the
verdict `satisfied`; (3) apply `world.seven-edits.json` and run, recording pass or fail as a metric; (4) `frob
nemesis search quad-tray --task <id> --legality kitchen-commercial/default --budget gpu_s=240 --key l6-nem
--sync-wait-ms 5000 --json`, following the job handle with `frob job wait`, and assert the result's shape
(worlds searched, classes, the baseline arm's count at equal budget, the artifact guard's line); (5) `frob
nemesis shrink <failing world> --key l6-shr --json`; (6) assert the twelve invariants on the written file with
`e2e_assert_json` and `e2e_assert_num`; (7) replay the file and each neighbour: `e2e_assert_digest_eq` for the
file, verdict `satisfied` for each neighbour; (8) `frob nemesis oracle <mcw> --candidate quad-tray --json`,
assert a solvable class; (9) the six refusals through `e2e_expect_refusal`; (10) `frob-golden check --artifact
L6`. Plants: `drop-neighbour` removes a neighbour from the produced file and must fail step (6);
`non-replaying` perturbs one state digest and must fail step (7). The log
(`target/e2e-logs/art_l6_fmcw/<utc>.ndjson`) records every command with exit code and wall time, the evidence
block (class Tested, tier, determinism D2, identities, seed), the metrics `edits`, `tests`, `seeds_failing`
with their thresholds (≤ 3, ≥ 28 when pairs are exhausted, ≥ 48), digests, skips with reasons, refusals with
code and first repair, and the `summary`.

### Known answers and negative controls

- Known answers (exact): 7 + 21 = 28 single and pair tests of 42; 55 ≥ 48 of 64; −0.42 from 17.04° and 12°;
  the two delta-debugging cases of the unit tests. They have no number in the known-answer suite; the unit's
  coverage file lists them.
- Counterexample kept alive: calling a 1-minimal world the smallest failure. With failure when a and b are
  both present or when c, d and e are all present, and the edits listed as (a, c, d, e, b), delta debugging
  ends at three edits while a two-edit counterexample exists.
- Negative controls: the six mutated files; the two plants; an impossible world (no solvable witness) must not
  be written as a `.fmcw`.
- Invariants carried: INV-NEMESIS-01 (only solvable worlds count), INV-NEMESIS-02 (every world evaluated is
  inside the legality profile, checked on the final world), INV-NEMESIS-04 (a `.fmcw` replays with identical
  digests on its originating backend), INV-NEMESIS-05 (a failure counts only after the artifact guard).
- Metamorphic relation: permuting the order of the edits in the starting list leaves the minimum-among-pairs
  result unchanged when a failing pair exists.
- Determinism lanes: the shrink twice on one backend (class D2); the format tests in debug and release, on
  Linux and macOS (class D0).

## Considerations

- Kill gate KILL-MCW (on unplanted failures fewer than half of shrunk worlds have at most three edits, or
  fewer than 90% replay): fallback, full failing worlds are shipped with replay and with the single-edit
  contrasts ranked by margin; the format stays, so this fixture keeps its format lane and steps (1) to (4).
- The four decoy edits are this bead's; the specification does not list them. They are data in the fixture,
  so changing them is a reviewed change.
- The story's values (55/64, 42 tests, margins) will not be reproduced by a real run; `fields.toml` marks them
  `structural`. Only the format, the invariants and the arithmetic are exact.
- The hazard dimensions touched by two edits have provenance `assumed`; the fixture is a format and mechanism
  test and supports no realism claim about kitchens.
- The search in step (4) costs GPU time (240 GPU-seconds in the story); in small mode the script uses the
  seven-edit world only and the log says that no gate is claimable.

## References

Appendix L.6; §26.3 (legality), §26.5 (the oracle), §26.6 (the artifact guard), §26.7 (shrinking and the
file), §26.13; INV-NEMESIS-01, -02, -04, -05; GATE-NEMESIS-03 is measured by the shrinking beads; KILL-MCW.
