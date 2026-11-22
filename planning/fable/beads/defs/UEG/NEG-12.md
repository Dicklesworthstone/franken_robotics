---
key: "NEG-12"
title: "inconsistency E7-B resolved: both film gates hold the three squeeze times to 2%, the bound of the law's owner"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "traction", "habitat", "frob-contact", "solid", "test", "card", "gate"]
parent: "WP-53"
deps: ["NEG-03", "GB-21", "BD-230", "N19b-06", "N18a-37"]
needs: []
related: ["N18-KA", "N19-KA"]
gates: ["GATE-TRAC-05", "GATE-HAB-06"]
plan_refs: ["Appendix E.7 (squeeze-film tolerance)", "§18.8", "§19.10", "GATE-TRAC-05", "GATE-HAB-06", "Appendix M KA-404, KA-438"]
---
## Background

The film-friction law is checked by two gates, because two subsystems meet in it: Traction owns the law
(GATE-TRAC-05) and Habitat owns the film patches that scenes place on floors and the per-contact thickness
state (GATE-HAB-06). Both gates use the same known answer, the three Stefan squeeze times of the worked card
(0.038 s for water under a small foot, 2.3 s for cooking oil under the same foot, 0.76 s for water under a
large foot). The scene gate held them to 1% and the traction gate to 2%. A mechanism that passed one gate
could fail the other on the same numbers. Settled: both gates use 2%, the bound of the chapter that owns the
law. The general rule this bead installs: a known answer has one tolerance, registered once with its owner,
and every gate that cites it reads that registration.

## Scope

In: the single tolerance registration for the squeeze-time known answer; the two gates' rows reading it; the
regression test. Out: the law (BD-230); the two end-to-end scripts that measure the gates (N18a-37, N19b-06);
the card (GB-21).

## Technical approach

1. **One registration.** The known-answer registry row for the squeeze times (the Traction row and its Habitat
   twin, which are marked as the same answer) carries `tolerance = "2% relative"` and `owner = "Traction"`.
   The Habitat row stores no tolerance of its own: it references the Traction row by identifier.
2. **Gate rows.** In `registries/gates.toml`, GATE-TRAC-05 and GATE-HAB-06 each cite the known answer by
   identifier; the gate runner resolves the tolerance from the registry at run time. A gate row that states a
   numeric tolerance for a cited known answer is rejected by the registry lint.
3. **Values.** t = 3π·η·a⁴/(4·N·h²) to h = 10 µm: 0.0376991 s, 2.2619467 s, 0.7634070 s. Acceptance windows
   at 2%: [0.036945, 0.038453], [2.216708, 2.307186], [0.748139, 0.778675] s.
4. **Why 2% and not 1% (DESIGN reasoning recorded with the settlement; the settlement itself is "the bound
   of the chapter that owns the law").** The law is integrated per contact sample with the tier's step; on lockstep tiers at
   a 2 to 2.5 ms step the handover time is resolved to within a step, which for the 0.038 s case is already
   5 to 7% of the interval unless the update is exact in 1/h². The exact update makes the time step-
   independent, and 2% then covers the handover detection and single-precision rounding on GPU tiers; 1% was
   never derived from an error budget.
5. **Register and scan.** Item E7-B: stale pattern `1%` in the scope of squeeze, film and the Habitat gate's
   identifier; pinned by the test below.

## Acceptance criteria

- [ ] The registry has one tolerance (2%) for the squeeze-time answer; the Habitat row references it.
- [ ] Both gate rows resolve to 2%; the lint rejects a gate row with its own numeric tolerance for a cited
      known answer.
- [ ] Both gates pass or fail together on a `synthetic` measurement at 1.5% error (pass) and at 2.5% (fail).
- [ ] The stale-value scan finds no hit for E7-B.

## Test plan

### Unit tests

- `squeeze_tolerance_single_registration`: one tolerance; the twin row has a reference, not a number.
- `gates_resolve_same_tolerance`: both gates read 2%.
- `gates_agree_on_boundary_cases`: measurements at +1.5% and +2.5% on each of the three times give
  (pass, pass) and (fail, fail), never a split verdict.
- `registry_lint_rejects_local_tolerance`.
- `squeeze_windows`: the three windows above from the closed form.

### End-to-end script and logging

The two existing scripts measure the gates: `scripts/e2e/traction_film.sh` (N18a-37) and the Habitat film
script (N19b-06). This bead adds one assertion to each: the tolerance printed in the log's `metric` events for
the squeeze times equals the registry's, and the two logs' thresholds are identical. The stale-value step of
`scripts/e2e/worked_cards.sh` (NEG-02) covers item E7-B.

### Known answers and negative controls

KA-404 and KA-438 (the same three squeeze times, listed once under each chapter). Negative control: a planted
1% in the Habitat gate row must be rejected by the lint; a planted measurement at 1.5% error must not produce
a split verdict. Determinism: registry resolution is D0.

## Considerations

- If a tier later demonstrates 1% with an error budget on file, the registration changes once, with its
  derivation, and both gates tighten together.
- The same single-registration rule applies to every known answer cited by more than one gate; the lint added
  here enforces it across the registry.

## References

Appendix E.7 (squeeze-film tolerance); §18.8; §19.10; GATE-TRAC-05; GATE-HAB-06; KA-404, KA-438.
