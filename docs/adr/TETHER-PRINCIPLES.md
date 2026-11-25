# Tether Surface Principles

Tether is the thin agent- and human-facing surface over every FrankenRobotics subsystem. It owns the verb registry, the result envelope, sessions and capability tokens, the job ledger and daemon dispatcher, the MCP server, the terminal dashboard, and the viewer. It owns **no physics and no statistics**.

Six standing principles govern Tether, each bound to a mechanical check. Four of these checks are registered as named lints in `registries/adr/tether.toml` (`[[principle_lint]]`).

## 1. One verb registry

The registry (`registries/verbs.toml`) defines each verb once: name, cluster, kind (`read`, `run`, `mutate`, `admin`), required capabilities, input and output schema identifiers, possible refusals, idempotency rule, cost model, and examples. The CLI, the MCP tools, the JSON schemas, and the agent documentation are all generated from it, and a parity gate diffs the served surfaces.

- **Evidence label:** `FACT`: this is the design the sibling project FrankenMining adopted.
- **Check:** Parity harness (`GATE-TETHER-02`, `N32a-11`).

## 2. Agents ask questions and spend budgets; they do not drive solvers

A verb takes a question, a confidence or tolerance where relevant, and a budget; it either answers with labelled evidence or refuses with a typed refusal code and ranked repairs stating what would make the question answerable.

- **Evidence label:** `FACT`: FrankenSim addendum proposal 8.
- **Check:** Registry lint `principle-no-solver-step-verb` — no registered verb exposes a solver iteration, a single simulation step call, or a raw kernel launch as its unit of work.

## 3. The Five Explicits

Units, seeds, budgets, versions, and capabilities are never implicit.

- **Evidence label:** `FACT`: FrankenSim principle P10.
- **Check:** Schema lint `principle-five-explicits` — every physical quantity field carries an explicit unit; every stochastic verb takes an explicit seed input and echoes it; every `run` verb has an explicit `budget` property; every schema carries an explicit major version; every verb entry explicitly declares `capabilities` (empty only with `public = true`).

## 4. One application service, two adapters

The CLI (`frob`) and the MCP server (`frob-mcp`) both link `frob-verbs` and call the same handlers in process through one dispatch interface, and both hand long work to `frobd` as jobs. Simulation ticks run inside a worker owned by `frobd`; calls on the control plane act on whole designs, jobs, chunks, and queries, never on single steps.

- **Evidence label:** `DESIGN` reason: a per-step remote call would put the agent's network and reasoning latency inside the physics loop.
- **Check:** Workspace/surface lint `principle-no-per-verb-surface-code` (`xtask check-layers` + verb-literal lint ensuring no per-verb code lives in `frob-cli` or `surface/frob-mcp` outside the allowlisted MCP half of `session.focus`) and parity harness (`GATE-TETHER-02`).

## 5. Every answer says what changed

After a `run`, the envelope names the change the agent made since the compared run, the first physical consequence of it, and the evidence for the comparison (`delta` block).

- **Evidence label:** `DESIGN`: grounded agent iteration requires causal feedback tied to the claim product rather than naked summary numbers.
- **Check:** Envelope conformance on `run` verbs (`GATE-TETHER-03`, `N32a-03`).

## 6. What is claimed for this surface

MCP access to a simulator is prior art.

- **Evidence label:** `FACT`: one incumbent ships an MCP server for documentation lookup, and a community server exposes 42 tools that drive that simulator; neither documents compute budgets, provenance, or a typed refusal taxonomy.
- **Claim boundary:** The claim made for FrankenRobotics is strictly narrower: every verb carries a compute budget, provenance, and a typed refusal, and both CLI and MCP surfaces are generated from one registry.
- **Check:** Documentation lint `principle-surface-claim-wording` — asserts `README.md` and `robot-docs` state this surface claim in this wording and no broader one (wired by `BD-022`).

## Immutability of Decision Records

A Tether decision record (`docs/adr/ADR-TETHER-01.md` through `ADR-TETHER-11.md`) is never deleted. When a decision changes, the existing record's status in both its document and `registries/adr/tether.toml` transitions to `superseded` with an explicit successor record.
