# Tether Ownership Map

Tether owns **no physics and no statistics**. A verb's domain handler lives in the crate of the subsystem that answers the question; Tether owns the surface contract, registry, envelope, daemon job infrastructure, adapters, and read-only clients listed below.

## Ownership Table

| Artifact | Crate or directory | Binary | Tier | First milestone |
|---|---|---|---|---|
| Verb registry `registries/verbs.toml`; generated schemas, envelope, exit codes, refusal table | `frob-verbs` | none | Solid | M0 |
| Command line | `frob-cli` | `frob` | Solid | M0 |
| Daemon: sessions, tokens, budgets, job ledger, dispatcher, device ownership | `frob-cli` (process), `frob-verbs` (job engine library) | `frobd` | Solid | M2 for the job engine; slot dispatch M3 |
| MCP server on the sibling library `fastmcp_rust` | `surface/frob-mcp`, a member of the core workspace | `frob-mcp`; also hosted by `frobd --mcp` | Solid; the Tasks adapter is Frontier | M0 smoke, M1 verbs, M3 full |
| Terminal dashboard on FrankenTUI | `frob-tui` | `frob-tui` | Solid, optional | M3 |
| Viewer page, overlays, stills, self-contained HTML replay | `viewer/` | none | Solid; a native viewport is Frontier and unfunded | M2 |
| Agent guide, skill file, MCP prompts | generated from the registry | none | Solid | M1 |

## Daemon Crate Split (`DESIGN`)

`DESIGN`: the daemon `frobd` is intentionally split across two core crates to reconcile the ownership table and the workspace crate map (`§34.2`):

- **`frob-verbs` (layer 6, rank 4):** owns the job engine library — sessions, capability token checks, budget accounting, SQLite job ledger, dispatcher trait, and slot-pool scheduling logic — so both in-process dispatch and the daemon share one tested engine with no duplicate state machine.
- **`frob-cli` (layer 7, rank 2):** owns the `frobd` process shell (CLI entry point, signal handling, socket listener, and `frobd --mcp` hosting) alongside the `frob` command-line binary.

## Crate Map Alignment

All four Rust crates named above (`frob-verbs`, `frob-cli`, `frob-mcp` at `surface/frob-mcp`, `frob-tui`) carry explicit rows in `crates/xtask/src/crate_map.rs` (`§34.2`). Non-Rust web assets live in `viewer/`, and generated agent documentation (`robot-docs`, skill files, MCP prompts) is derived directly from `registries/verbs.toml`.
