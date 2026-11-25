# ADR-TETHER-06: MCP Tasks Extension as Optional Adapter Only

- **ID:** `ADR-TETHER-06`
- **Status:** `accepted`
- **Governing bead:** `N32a-30`
- **First milestone:** `M3`
- **Plan references:** §32.3, §32.12.1, KILL-MCPTASKS

## Decision

The MCP Tasks protocol extension is implemented strictly as an optional, compile-time-removable adapter layered over the `frobd` job ledger and the exit-8 job-handle contract. It is never the primary contract for long-running verbs, and no verb semantics depend on Tasks being negotiated or compiled in.

## Reason

`FACT`: the sibling MCP library (`fastmcp_rust`) labels its Tasks support experimental and provisional. By contrast, the registry's job-handle contract (`sync_wait_ms`, exit 8, `job.wait`, `job.cancel`, idempotency recovery) works identically across every MCP client, protocol version, and CLI invocation.

## Alternative excluded

Using MCP Tasks as the sole representation for asynchronous jobs, which would couple core job execution to a provisional protocol extension and break clients that do not implement Tasks.

## Enforced by

- `mcp_handle_contract_without_tasks` (pending at M3, `N32a-30`): verifies the complete long-job handle contract (`sync_wait_ms`, exit 8, `job.wait`, `job.cancel`, recovery by key) passes with the MCP Tasks adapter compiled out.

## Revisit when

- `KILL-MCPTASKS` fires: the optional Tasks adapter is removed cleanly while this record's core rule (the job-handle contract is primary) remains in force.
- MCP Tasks stabilizes across major clients and `fastmcp_rust` promotes it out of experimental status (still evaluated only as an additive adapter unless a superseding ADR changes the primary contract).
