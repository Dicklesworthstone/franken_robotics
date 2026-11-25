# ADR-TETHER-05: Wire Cancellation Ends the Wait, Not the Job

- **ID:** `ADR-TETHER-05`
- **Status:** `accepted`
- **Governing bead:** `N32a-17`
- **First milestone:** `M2`
- **Plan references:** §32.3, §32.12.5, §32.19 (INV-TETHER-11)

## Decision

Cancelling or dropping the wire connection of a waiting verb call (CLI interrupt or MCP request cancellation) ends only the caller's wait and leaves the admitted job running in `frobd`, unless the call explicitly set `on_cancel = "cancel_job"` (default `on_cancel = "leave"`). Explicit job termination goes through the `job.cancel` verb.

## Reason

A client timeout, network hiccup, or dropped MCP transport is not an operator decision to discard admitted, budget-charged compute. Leaving the job intact lets the agent recover the handle or result deterministically via its idempotency key (`frob job find --verb <verb> --key <key>`).

## Alternative excluded

Automatically killing or aborting the backing job whenever the waiting client disconnects or cancels its transport wait, which wastes spent compute budgets on transient client timeouts and turns transport flakes into nondeterministic partial work.

## Enforced by

- `INV-TETHER-11` cancellation tests (pending at M2, `N32a-17` and `N32b-22`): drop the client connection and issue MCP wire cancellations under both `on_cancel = "leave"` (job runs to completion and is recovered by idempotency key) and `on_cancel = "cancel_job"` (daemon receives explicit cancellation and stops cleanly at a safe boundary).

## Revisit when

- A deployment environment mandates strict foreground-only execution with no daemon job ledger (must be handled as a separate explicit session policy rather than silently changing the default of `on_cancel`).
