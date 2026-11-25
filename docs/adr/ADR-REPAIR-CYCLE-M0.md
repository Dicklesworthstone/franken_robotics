# ADR-REPAIR-CYCLE-M0: Bounded Meaning of "One Repair Cycle" and "One Optimization Pass"

- **Status:** Accepted (DESIGN reading, recorded before the first M0 kill decision per `N40-03` step 5)
- **Milestone:** M0
- **Bead:** `N40-03` (`frob-n40-03-bod4`)
- **Plan references:** §40.1, §40.4, §43.2, §29 (beadlog `frob.beadlog/1`), `N41-17` rule 2

## Context

Many kill-gate triggers in `registries/gates.toml` allow *"one repair cycle"* or *"one optimization pass"* before a failing measurement triggers the fallback (for example, `KILL-METAL`: *"GATE-SPARK-04 fails after one optimization pass..."*). Without a bounded, machine-auditable definition, "one pass" degrades into unbounded tuning against a fixed fixture or quiet threshold relaxation after the measurement is known.

## Decision

1. **One bounded attempt.** "One repair cycle" or "one optimization pass" is strictly **one** bounded attempt to fix the root cause of a failed measurement:
   - **Opened** by a `note` entry in the measuring bead's log (`frob.beadlog/1`, `N29a-09`) with:
     ```
     cause = "repair-cycle: <failed receipt hash>: <hypothesis>"
     ```
   - **Closed** by a re-measurement under the **same threshold** on the **same fixture**.
   - If the re-measured value remains outside its bound, the part is recorded `fail` and the measuring bead closes `recorded` (`N41-17` rule 2), making the kill-gate decision bead ready.

2. **Fresh instances when exposure occurred.** Where the first failing measurement exposed the fixture's instances (their labels, or per-instance or per-pair results that a fix could be tuned to), the re-measurement MUST run on **fresh instances** drawn by the fixture's registered construction with a new seed. The opening log line records both that exposure reason and the seed or digest of the fresh instances (e.g., `N24c-26` re-runs on fresh planted tasks; `N24c-34` on a newly recorded matrix).

3. **Forbidden edits during a cycle.** A repair cycle or optimization pass changes **only the mechanism**. It NEVER changes:
   - the fixture's construction,
   - its label rule,
   - the gate's threshold (which changes only by a dated `[[amendment]]` preceding the measurement it governs), or
   - the negative control.

4. **Second cycle requires a `deferred` decision ADR.** Any second repair cycle or optimization pass is NOT part of the original allowance; it is a `deferred` kill-gate decision recorded as a committed ADR under `docs/adr/ADR-<gate-name>-<milestone>.md` with a non-empty `reason` and `deferred_to` strictly later than the current exit.

## Consequences

- Every repair cycle has an auditable start (`failed receipt hash` + `hypothesis` + fresh seed/digest where applicable) and an unambiguous stop condition.
- Post-measurement threshold lowering is refused mechanically by `amended-after-measurement` (`RC-10`).
- Unilateral second cycles without an explicit `deferred` ADR are caught by the exit due list and review checks.
