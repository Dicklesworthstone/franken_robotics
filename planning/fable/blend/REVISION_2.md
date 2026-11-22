# Revision 2: every sibling is the owner's, so siblings are changed, not worked around

Owner's ruling, 2026-11-22: fastmcp_rust is the owner's own project, as is every sibling this plan names. A
mismatch between a sibling and this project is therefore fixed in the sibling. In particular the plan's default of
running the MCP server across a process boundary, because fastmcp_rust declares a newer Rust version than
FrankenSim's pin, is wrong: fastmcp_rust is simply brought onto the Rust toolchain this project targets.

These decisions replace charter decision G14's consequences, Phase 2 brief decision R3, and every statement in the
plan that siblings are read-only. They are binding for the plan text and for the beads.

## REV-1 Ownership: sibling work is our work

Every sibling project the plan consumes belongs to the project owner: frankensim, frankentorch, franken_numpy,
frankensympy, franken_lean, frankenscipy, frankenjax, asupersync, frankensqlite, fastmcp_rust, frankentui,
franken_threed, franken_textiles, franken_drone_geometry_reconstruction and the others named in §33.

1. A limitation, defect or mismatch in a sibling is work this program does, in the sibling's repository, under
   that repository's own `AGENTS.md`, gates and conventions. It is a bead in this program's graph (label
   `upstream`), mirrored as a bead in the sibling's tracker.
2. The rule "sibling repositories are read-only to this program; needs are filed as asks" is withdrawn. Appendix F
   is the list of **upstream work items**, not of requests to someone else. The word "ask" is replaced by "upstream
   work item" or "upstream change"; identifiers `FD-…` keep their numbers.
3. What stays, because it is still good engineering:
   - every upstream work item keeps an **interim posture** and a **switch-over rule**, so that no milestone waits
     on a sibling change landing;
   - upstream changes are **additive** (a new mode, type or feature; existing behaviour is not changed; any
     lossiness is self-described), and they follow the sibling's own rules. For FrankenSim that includes its layer
     checks, contracts, unsafe-capsule discipline, its ban on in-process foreign kernels in production paths (so
     the GPU annexes stay in this repository) and its ban on branches and worktrees;
   - pins advance through the compatibility train (move the pin, rerun admission and parity gates, relock); a
     sibling's newer clean head is never rolled back locally; the airlock crate `frob-fsim` stays, because
     FrankenSim's API still moves daily;
   - fallbacks stay written down (KILL-FSIM, KILL-MCP and the others), now as true fallbacks and not as defaults.
4. Risk rows and open questions that assumed asks might never be staffed are restated: the risk is now the cost
   and sequencing of upstream work, and collisions with other agents working in the same sibling.

## REV-2 One toolchain for the whole build graph

1. The franken_robotics build graph has **one Rust toolchain**, called T, recorded in `franken-stack.lock` and in
   `rust-toolchain.toml`. Every crate that any `frob` binary links builds on T: the core, the GPU annexes, the
   derivation tool and the MCP server. Separate toolchain files for annexes are withdrawn, except for `lean/`,
   which is not Rust.
2. T is chosen at INT-TOOLCHAIN-01 and then moved only by a **toolchain train**: one coordinated change that moves
   T in this repository and in every consumed sibling that needs it, followed by the admission and parity gates.
   Candidates, in order of preference:
   - (a) the suite's common nightly, the one most siblings already pin (`nightly-2026-08-31` at the time of
     writing: asupersync, frankentorch, franken_numpy, franken_lean, franken_threed and others). This requires
     advancing FrankenSim's pin from `nightly-2026-07-06` in FrankenSim's repository, moving frankensympy from
     `nightly-2026-08-20` and fastmcp_rust from `nightly-2026-08-25`.
   - (b) FrankenSim's present pin, if FrankenSim cannot yet build on (a). Then fastmcp_rust's declared
     `rust-version` and toolchain file are changed to build on that pin, and the train to (a) is scheduled.
   In both cases the result is the same: **fastmcp_rust is upgraded to match T**, by an upstream work item
   (FD-FMCP-01, priority P0, at M0) done by this program in fastmcp_rust's repository, with fastmcp_rust's own test
   suite and MCP conformance checks passing on T.
3. One asupersync version across the graph is obtained the same way (FD-ASUP-01, P0, at M0): FrankenSim's
   constellation, FrankenTorch and fastmcp_rust are aligned on one published asupersync version by changes in
   those repositories. INT-ASUP-01 stays as the gate that proves it.
4. The `fs-qty` nightly-feature coupling stays a named risk tested at INT-TOOLCHAIN-01; the `frob-units` escape
   hatch stays as the fallback. Under REV-1 the preferred fix is an upstream change in FrankenSim.

## REV-3 The MCP server is in the graph

1. `frob-mcp` is built on fastmcp_rust and is an ordinary member of the core workspace build graph. It links
   `frob-verbs` and calls verb implementations **in process**; it shares the one toolchain and the one asupersync
   package with the rest of the graph. Its source stays under `surface/frob-mcp/` because it is a surface, not
   because it is a separate workspace.
2. `frobd`, the daemon, still owns devices, kernel caches and the job queue. `frob-mcp` and `frob` (the CLI) both
   hand long work to `frobd` as jobs, and both run CPU-tier verbs in process when no daemon is running, saying so
   in the envelope. `frobd` can also host the MCP server itself (`frobd --mcp`), so that one process serves the
   job queue and the MCP transports.
3. **INT-FMCP-01** (admission, M0): `frob-mcp` builds in the core graph on T with exactly one asupersync package
   (`cargo tree -d` shows no duplicate), lists the tools generated from the verb registry, and passes the MCP
   protocol smoke test over stdio and HTTP.
4. **KILL-MCP** is now a true fallback. Trigger: INT-FMCP-01 cannot pass within the M0 budget after FD-FMCP-01 and
   FD-ASUP-01 have been attempted. Fallback 1: `frob-mcp` as its own workspace talking to `frobd` over a local
   socket in newline-delimited JSON (the former default). Fallback 2: agents use the CLI, which is complete on its
   own.
5. Everything else in the MCP design stands: tools generated from the verb registry; clusters of at most seven
   tools per task; a bounded synchronous wait on long tools with a job handle otherwise; progress and draining
   cancellation; catalogs, stills, reports and schemas as resources; three failure classes; atomic idempotency
   binding; pagination pinned to a committed frontier; integers above 2⁵³ as strings.

## Where the plan text changes

- §0.7 (hygiene rule on sibling repositories), §1 (why the suite), §3 (principles, where "consume, never fork"
  appears: consuming stays, read-only goes).
- §32 (the MCP design, the daemon, KILL-MCP, the gates), §37 where the boundary is cited as a security property.
- §33 (postures, the FrankenSim decision's hazards, the foundation ledger rows for fastmcp_rust and asupersync,
  upstream work and drift, toolchain facts, the admission gates, the dependency firewall, open question
  OPEN-SUITE-2, which is now decided by REV-2), §34 (repository layout, the crate map row for `frob-mcp`, the rule
  about annex toolchain files, what ships with the first commit).
- §40 (INT-FMCP-01, INT-TOOLCHAIN-01, INT-ASUP-01 rows; the KILL-MCP row), §41 (the toolchain-split risk and the
  unstaffed-asks risk), §42 (OPEN-SUITE-2 and related), §43 (M0 artifact and acceptance), §44 (work packages for
  the surface and for upstream work), §45 (the M0 beads for the MCP smoke, the toolchain and asupersync), §46
  (the first session gains a step: build fastmcp_rust on T), §47.
- Appendix F (title, preamble and the FD-FMCP-01 and FD-ASUP-01 rows), Appendix H (the toolchain facts stay as
  FACTs), Appendix J and L only where they describe the boundary.
