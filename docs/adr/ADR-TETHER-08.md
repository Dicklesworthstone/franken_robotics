# ADR-TETHER-08: Scheduler Isolation — When Work Runs, Never What Is Computed

- **ID:** `ADR-TETHER-08`
- **Status:** `accepted`
- **Governing bead:** `N32a-21, BD-296`
- **First milestone:** `M3`
- **Plan references:** §32.3, §32.19 (INV-TETHER-09), GATE-TETHER-10

## Decision

The daemon scheduler decides only *when* and *where* jobs run and never alters *what* is computed. No wall-clock time, queue depth, preemption schedule, or host load quantity may enter a physics computation, a statistical verdict, a canonical artifact digest, or an evidence record (`cost.wall_ms` is the sole wall-clock field on an envelope and is strictly excluded from every digest and evidence record).

## Reason

Any feedback from cluster load or wall-clock timing into solver parameters, sample sizes, or convergence checks destroys deterministic replay and makes engineering verdicts depend on what other jobs happened to share the machine.

## Alternative excluded

Load-adaptive solver effort, wall-clock-terminated convergence loops that publish partial results, or sample counts adjusted dynamically by queue length.

## Enforced by

- `INV-TETHER-09` scheduler isolation tests (pending at M3, `N32a-21` and `N32b-22`): verify no wall-clock quantity enters canonical digests or evidence records.
- `GATE-TETHER-10` (pending at M3, `BD-296`): verifies result digests are bit-identical under random preemption, reordering, and background load.

## Revisit when

- Any adaptive anytime algorithm is proposed for a verb (must be modeled as an explicit deterministic budget parameter in the request input and evidence record, never an implicit wall-clock cutoff).
