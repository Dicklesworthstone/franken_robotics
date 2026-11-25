# ADR-TETHER-02: `frob-verbs` `registry-only` Feature Without Physics Dependencies

- **ID:** `ADR-TETHER-02`
- **Status:** `accepted`
- **Governing bead:** `N32a-02`
- **First milestone:** `M0`
- **Plan references:** §32.3, §32.12.2

## Decision

`frob-verbs` provides a `registry-only` feature (`--no-default-features --features registry-only`) that compiles the static verb table, embedded canonical schemas, name map, envelope and refusal types, and request canonicalization with no dependency on any physics crate, subsystem handler crate, or the FrankenSim airlock crate `frob-fsim`. In the default core-workspace build (`default = ["handlers"]`), `frob-mcp` links `frob-verbs` whole.

## Reason

`registry-only` is the exact dependency surface that the first fallback of kill gate `KILL-MCP` (`N32a-06`) links. If `fastmcp_rust` cannot join the core workspace on toolchain `T`, `surface/frob-mcp` is built as its own separate workspace and talks to `frobd` over a local socket; it must still share the exact verb table, JSON schemas, and envelope types with zero duplication and zero physics build weight.

## Alternative excluded

Maintaining a second copy of the registry, schema, and envelope types inside `surface/frob-mcp` or a forked compatibility crate, which would split the single source of truth and reintroduce surface drift.

## Enforced by

- `frob-verbs::conformance::registry_only_has_no_physics` (`crates/frob-verbs/tests/conformance.rs`, active at M0): runs `cargo tree -p frob-verbs --no-default-features --features registry-only -e normal --locked --offline` and asserts that `frob-fsim`, every FrankenSim physics crate, and every subsystem crate are absent (only `frob-core` and its layer-0 non-physics utilities `fs-blake3` and `fs-qty` appear).

## Revisit when

- `KILL-MCP` fires (`N32a-05`): `registry-only` ceases to be a fallback-only build mode and becomes the design of record for the standalone `surface/frob-mcp` workspace (`N32a-06`).
- Any dependency added to `frob-core` pulls a physics, solver, or async crate into the `registry-only` dependency closure.
