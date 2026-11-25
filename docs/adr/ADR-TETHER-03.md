# ADR-TETHER-03: `frob-mcp` Protocol Adapter with Dual Dispatch Back Ends

- **ID:** `ADR-TETHER-03`
- **Status:** `accepted`
- **Governing bead:** `N32a-04`
- **First milestone:** `M0`
- **Plan references:** §32.3, §32.12.2, OPEN-TETHER-1

## Decision

`surface/frob-mcp` is a thin protocol adapter built over one `frob-verbs` dispatch trait with two back ends:

1. **In-process back end (default):** `frob-mcp` is a member of the core workspace on toolchain `T`, links `frob-verbs` whole, executes fast verbs in process through the shared dispatch interface, and submits long or device-backed work to `frobd` as jobs. This records the owner's resolution of `OPEN-TETHER-1` on 2026-11-22 (contingent on upstream `FD-FMCP-01` bringing `fastmcp_rust` onto toolchain `T`).
2. **Local-socket back end (complete fallback):** Talks exclusively to `frobd` over a local socket (`frob.rpc/1`), linking `frob-verbs --no-default-features --features registry-only` (`ADR-TETHER-02`). This is Fallback 1 of `KILL-MCP` (`N32a-06`); Fallback 2 is a CLI-only surface (`frob --json`) with no MCP server.

## Reason

The daemon `frobd` owns devices and long-running jobs in every deployment mode, so a killed or restarted MCP adapter must never own a running job. Keeping a single dispatch trait behind both back ends ensures Fallback 1 of `KILL-MCP` is reachable immediately without redesigning the MCP adapter, its schema layer, or its tool contracts.

## Alternative excluded

An MCP server that directly owns GPU/device work (losing jobs when an MCP client disconnects or crashes), or an adapter hardcoded to run exclusively across a process boundary (forcing unnecessary IPC overhead for cheap studio queries when in-process linking succeeds).

## Enforced by

- `INV-TETHER-10` kill and back-end parity tests (pending at M0, `N32a-04` and `N32b-22`): verify that killing the MCP process leaves admitted daemon jobs intact and that the socket back end passes the identical conformance suite as the in-process back end.

## Revisit when

- `KILL-MCP` fires (`N32a-05`): `ADR-TETHER-02` and the socket-only back end of `ADR-TETHER-03` become the design of record (Fallback 1, `N32a-06`), or if `fastmcp_rust` remains incompatible even in a separate workspace, `frob-mcp` is retired in favor of Fallback 2 (CLI only).
