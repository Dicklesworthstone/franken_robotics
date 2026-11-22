---
key: "NEG-10"
title: "discrepancy E7-7 resolved: the wheel-loop row uses one plant: 2.6917 with shaft damping, 2.7075 without"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "arena", "atelier", "frob-analyze", "solid", "test", "card"]
parent: "WP-41"
deps: ["NEG-03", "BD-108", "BD-123"]
needs: []
related: ["N16c-42", "BD-117", "BD-043"]
gates: []
plan_refs: ["Appendix E.7 row 7", "§24.10 (wheel-loop row)", "§28.4 A-22", "Appendix H.5 (sampled wheel loop)"]
---
## Background

Analysis A-22 (sampled-loop stability) checks a digital control loop by the spectral radius of its one-tick
map: above 1 the loop is unstable at that sample period however good the continuous design. Its fixture is a
75 mm wheel whose drive holds a current target for 10 ms. Three numbers describe it: with target-current
feed-forward the radius is 1.4820 (unstable); with measured-current feed-forward it is 0.9207 (stable); and
an instantaneous-current approximation, which ignores the winding's electrical dynamics, gives a third number
that shows why the approximation cannot be used. The fixture table first printed 2.7075 for that third number.
1.4820 and 0.9207 reproduce only with the fixture's shaft damping of 0.002 N·m·s/rad included; 2.7075
reproduces only without it; with the damping the instantaneous-current figure is 2.6917. The row therefore
mixed two plant models. No worked card uses the row, but the fixture is a known answer of A-22 and of the
first Arena fixture, so the three figures must come from one declared plant. Related FACT from the same
plan-time check: an explicit current-then-wheel update at 0.5 ms gives 1.497 where 0.921 is right, the control
for a wrong discretization order.

## Scope

In: one plant record for the fixture; the three radii computed from it; the regression test; the scan. Out:
A-22 itself (BD-108); the fixture's bodies and motor (BD-123); the tier-integrator variant of A-22 (N16c-42).

## Technical approach

1. **One plant record (DESIGN).** The fixture's parameters live in one file read by every computation:
   wheel radius 0.075 m, hold period 10 ms, shaft damping 0.002 N·m·s/rad, and the remaining motor and load
   parameters (winding resistance and inductance, torque constant, reflected inertia, loop gains) exactly as
   the first Arena fixture defines them. This bead does not restate those values from memory: the test reads
   the fixture file, and the plan-time reproduction bundle is the oracle for the three radii (re-run by the
   pilot bead before a gate cites them).
2. **Three computations on that record.** (a) target-current feed-forward: build the one-tick map by exact
   discretization (matrix exponential of the continuous plant over the hold period, with the held input);
   expected spectral radius 1.4820. (b) measured-current feed-forward: 0.9207. (c) instantaneous-current
   model (current equals its target within the tick): 2.6917. Each to four decimals (tolerance 5 × 10⁻⁵).
3. **The undamped variant** is computed only as a labelled control: damping set to zero in model (c) gives
   2.7075; applied to (a) and (b) it must not reproduce 1.4820 and 0.9207, which the test asserts.
4. **Rule.** A table row that lists several results for one fixture carries the fixture's plant digest; A-22's
   report prints the digest beside every radius. Two radii with different digests in one row are rejected.
5. **Register and scan.** Item E7-7: stale pattern `2.7075` in the scope of wheel-loop, spectral or
   instantaneous-current text, unless labelled as the undamped control; pinned by the test below.

## Acceptance criteria

- [ ] From the single plant record: 1.4820, 0.9207 and 2.6917 (each within 5 × 10⁻⁵).
- [ ] With damping zero, model (c) gives 2.7075 and models (a), (b) differ from their damped values.
- [ ] The explicit current-then-wheel update at 0.5 ms gives 1.497 and is flagged as the wrong-order control.
- [ ] A-22's report rejects a row whose entries carry different plant digests; the stale-value scan finds no
      unlabelled 2.7075.

## Test plan

### Unit tests

- `wheel_loop_radii_one_plant`: the three radii from the fixture file.
- `wheel_loop_undamped_control`: 2.7075 for (c) at zero damping; (a) and (b) move.
- `wheel_loop_wrong_order_control`: 1.497 from the explicit split update against 0.921 from the exact
  discretization.
- `wheel_loop_verdicts`: (a) unstable, (b) stable, (c) unstable; the verdict of (a) and (b) is what the
  fixture's dynamic layer shows (growing against decaying oscillation over 50 ticks).
- `a22_row_requires_one_digest`.

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) runs the stale-value step for item E7-7. The radii themselves are
exercised by the Arena fixture's analytic layer (`frob task fixture cart-sill-dock --analytic --json`, owned by
the fixture's bead), whose log lines carry the plant digest, the model name and the radius against its golden.

### Known answers and negative controls

FACT (plan-time check of the sampled wheel loop): 1.4820 with target-current feed-forward, 0.9207 with
measured-current feed-forward; the instantaneous-current model gives 2.6917 with the fixture's shaft damping
and 2.7075 without it; an explicit current-then-wheel update at 0.5 ms gives 1.497 where 0.921 is right.
Controls: the undamped mix and the wrong update order. Determinism: `f64` linear algebra on a 2 × 2 or 3 × 3
map, D0 on one machine.

## Considerations

- The parameters behind the radii are owned by the fixture; if the fixture's file changes, the three goldens
  are recomputed together and the change is a dated amendment to the fixture, never a silent edit here.
- The plan-time values are FACT about a small program; they are re-run from the reproduction bundle before
  any gate cites them.

## References

Appendix E.7 row 7; §24.10; §28.4 (A-22); Appendix H.5.
