# FIXLOG_REV2: edits applying REVISION_2.md (REV-1, REV-2, REV-3) to the part files

One line per edit: file, subsection, what changed. `<!-- LEDGER -->` blocks were not touched.

## parts/P00.md

- P00, header table, row "Language": "a pinned dated nightly that follows FrankenSim's" replaced by the one toolchain T chosen at INT-TOOLCHAIN-01 for every crate a `frob` binary links.
- P00, Contents, appendix list: "F upstream asks" → "F upstream work items".
- P00, §0.1 rule 2: "an upstream ask" → "an upstream work item".
- P00, §0.5 intro: `surface/frob-mcp` stated as a member of the core workspace; the remaining annex paths stay separate workspaces or projects.
- P00, §0.7 hygiene rule on sibling repos: "read-only to this program / never a local bead" replaced by REV-1 (siblings are changed in their own repositories under their own rules; bead labelled `upstream` mirrored in the sibling tracker; interim posture and switch-over rule; no milestone waits).
- P00, §1.7 "The MCP substrate exists": "has a CLI-only fallback" → KILL-MCP names its fallbacks, the last being the CLI alone.
- P00, §1.7 "The limits of the siblings": four nightlies kept as FACT and stated as the starting state; "core follows FrankenSim's and annexes carry their own toolchain files" replaced by one toolchain T chosen at INT-TOOLCHAIN-01 and a toolchain train; `fs-qty` coupling reworded to the toolchain, not "the core toolchain".
- P00, §1.7 "FACT (to recheck)" lead-in: "an upstream ask" → "an upstream work item".
- P00, §3 principle 4: "Sibling repositories are read-only … filed upstream as an ask … never scheduled here" replaced by change-in-the-sibling with failing fixture, interim posture and switch-over rule.
- P00, §4 BET-12 falsifier: "no fallback in Appendix F applied" → upstream work item not landed and no interim posture applied.
- P00, §5 readers table, sibling row: reader, question and answer restated for upstream work items.
- P00, §6 GOAL-08: "file precise asks for the rest" → "do the rest as upstream work items in their repositories".
- P00, §6 anti-goals: "Scheduling a change to a sibling repository as a local bead" replaced by forking or patching a sibling locally, and a milestone that waits on an upstream change with no interim posture.

## parts/P12.md

- P12, §33 intro: chapter scope extended to how this program changes a sibling (§33.8).
- P12, §33.2 item 3: "core adopts FrankenSim's toolchain pin" → whole build graph on one toolchain T; one asupersync version obtained by FD-ASUP-01.
- P12, §33.2 after the hazard list: hazards 2, 3, 4, 5, 7 named as upstream work items done in FrankenSim's repository, with interim postures; airlock stays for hazard 1.
- P12, §33.4 ledger row FD-ASUP: fallback "process boundary between surfaces and core" → FD-ASUP-01 alignment; a surface with a second package goes to KILL-MCP fallback 1.
- P12, §33.4 ledger row FD-FMCP: posture "Consume, annex only" → "Consume"; use restated as a member of the core graph on T after FD-FMCP-01.
- P12, §33.4 honest note on frankensympy: `derive/` "has its own toolchain file" → builds on T, no toolchain file of its own; frankensympy's nightly kept as FACT and as starting state.
- P12, §33.5 stack-lock sample: top-level `toolchain` key for T added; FD-FSIM row `toolchain` becomes the sibling's own channel; FD-ASUP `fallback` restated.
- P12, §33.5 receipt sample: `"toolchains": {core, surface, derive}` → single `"toolchain": "<T>"`.
- P12, §33.6 gate INT-TOOLCHAIN-01: restated as the gate that chooses and records T (preference (a) then (b)) for the whole build graph; T then moves only by a toolchain train.
- P12, §33.6 gate INT-ASUP-01: one package and one published version across workspaces; named as the proof of FD-ASUP-01.
- P12, §33.6 gate INT-FSYM-01: `derive/` builds on T.
- P12, §33.6 gate INT-FMCP-01: replaced by the REV-3 statement (core graph on T, one asupersync package, tools listed from the registry, MCP smoke over stdio and HTTP).
- P12, §33.7 firewall, rows asupersync and fastmcp_rust (M0 cells): FD-ASUP-01 and FD-FMCP-01 added.
- P12, §33.7 closing paragraph: the two P0 upstream work items named, off M1's critical path through KILL-MCP; M7 also freezes T.
- P12, §33.8 title: "Upstream asks and drift" → "Upstream work and drift".
- P12, §33.8 body: read-only rule replaced by REV-1 (work done in the sibling under its rules; `upstream` bead mirrored in the sibling tracker; interim posture, switch-over rule; additive changes; FrankenSim's rules; file reservations and the sibling's coordination rules; compatibility train; airlock stays; fallbacks stay as fallbacks).
- P12, §33.8 table: header "Ask / Request" → "Upstream work item / Change"; row FD-FMCP-01 rewritten (fastmcp_rust onto T, P0 at M0, interim posture CLI with KILL-MCP, switch-over INT-FMCP-01 in the core graph).
- P12, §33.8 after the table: paragraph on priorities rewritten (P0 items FD-FMCP-01 and FD-ASUP-01 at M0; priority orders this program's queue; cost, sequencing and collisions are risks in §41).
- P12, §33.8 KILL-FSIM row: "keep asks open" → "keep the upstream work items open".
- P12, §33.11 lead-in and table header: "upstream ask" / "Ask" → "upstream work item".
- P12, §33.12 risk paragraph: "the whole core" → "the whole build graph"; "ask" → "change"; "filed beside it" → "listed beside it".
- P12, §33.12 "three ways" items 1 and 2: restated for a toolchain train and for the starting-state mismatch that the choice of T and FD-FMCP-01 remove.
- P12, §33.12 matrix: columns labelled as candidates (a) and (b) and as starting-state pins; per-annex "must pass" cells replaced by "recorded" with the rule that every row passes on T.
- P12, §33.12 decision rule: annex-across-a-process-boundary clause removed; T chosen in order (a), (b); upstream change in FrankenSim preferred; `frob-units` kept as the fallback.
- P12, §33.13 consequences 1–3: rewritten as one toolchain T with the toolchain train and candidates (a)/(b); fastmcp_rust upgraded to T by FD-FMCP-01 and `frob-mcp` in the core graph; one asupersync version by FD-ASUP-01. Toolchain table kept as FACT and named the starting state.
- P12, §33.15 frankentui: "moves to its own workspace" → upstream change or toolchain train; no toolchain of its own.
- P12, §33.15 fastmcp_rust: posture "in the annex, across the process boundary by default" → member of the core workspace, links `frob-verbs`, in process, after FD-FMCP-01.
- P12, §33.16 INV-SUITE-04: wording "read-only" replaced; clause added that upstream changes are made by agents in the sibling repository, never by this repository's tooling.
- P12, §33.16 OPEN-SUITE-1: default restated around FD-ASUP-01.
- P12, §33.16 OPEN-SUITE-2: marked decided, with the decision text.
- P12, §33.16 OPEN-SUITE-6: "only if INT-ASUP-01 forces a process boundary" → only if INT-ASUP-01 fails after FD-ASUP-01 and a fallback splits the graph.
- P12, §33.17 table row 8: "the upstream ask is to wrap" → "the upstream work item (FD-FT-01) wraps".
- P12, §33.17 "Row 7, the consequence": "an upstream ask … not scheduled here" → upstream work item FD-FTX-01.
- P12, §33.18 closing paragraph: "Upstream asks … None is scheduled here" → upstream work items done in the sibling under its rules; interim postures hold.
- P12, §34.1 layout block: root comment names T; `surface/frob-mcp` shown as a core workspace member linking `frob-verbs`; GPU annexes and `derive/` shown as annex workspaces on T; "own toolchain" comments removed; `asks.toml` → `upstream.toml`.
- P12, §34.1 workspace paragraph: seven/eight workspaces → six/seven (`surface/` no longer one); all on T; annex boundary justified by the unsafe and foreign-interface policy, not the compiler; GPU annexes keep separate lockfiles on the cited FACTs.
- P12, §34.2 crate map: `surface/frob-mcp` row moved from the annex block to layer 7 before `frob-cli`, responsibility restated (links `frob-verbs`, in process, hosted by `frobd --mcp`); totals row and layer table updated (39 core, 4 annex; layer 7: 5 crates, cap 47; annexes: 4, cap 32; totals unchanged).
- P12, §34.3 L-5: `surface/frob-mcp` added to the boundary crates that may use `serde`.
- P12, §34.6 `check-stack` row: "read-only towards siblings" → the check writes nothing to a sibling checkout; T named.
- P12, §34.8: toolchain file is one for the whole graph; initial beads include the two P0 upstream work items.
- P12, §34.10 ADR-WS-01: `derive/frob-derive` reason "FrankenSymPy and its nightly" → FrankenSymPy, which no runtime crate may link.
- P12, §34.12 identifiers: "asks upstream for them" → "adds them upstream".
- P12, §34.13 OPEN-WS-2: default restated (core; upstream change or toolchain train if frankentui does not build on T).
- P12, §34.14: sentence added that `frob-mcp` in the core graph puts fastmcp_rust's `arbitrary_precision` in the unified feature set.

## parts/P11.md

- P11, §32.1 table, MCP row: "(annex workspace)" → a member of the core workspace; binary column adds `frobd --mcp`.
- P11, §32.2 "One application service, two adapters": CLI and MCP server both link `frob-verbs`, call handlers in process and hand long work to the daemon.
- P11, §32.3 ADR-TETHER-02: reason "the MCP annex builds on a different toolchain" → the `registry-only` feature is what KILL-MCP's first fallback links.
- P11, §32.3 ADR-TETHER-03: in-process handlers are the default back end; the socket-only form is KILL-MCP's first fallback; reason restated.
- P11, §32.11 first paragraph: CLI and `frob-mcp` both hand long work to `frobd` and both run CPU-tier verbs in process without a daemon; `frobd --mcp` added; paragraph rewrapped.
- P11, §32.12 intro: `frob-mcp` stated as an ordinary member of the core workspace that calls verbs in process.
- P11, §32.12.2 "The build graph": toolchain and asupersync FACTs kept as the starting state; DESIGN rewritten (FD-FMCP-01 and FD-ASUP-01 at M0; core workspace member; links `frob-verbs`; in process; long work to `frobd`; `frobd --mcp`); INT-FMCP-01 and KILL-MCP stated as in REV-3.
- P11, §32.12.6 "Cancellation": "a handler does no physics, waits on daemon frames" → in-process verbs observe `Cx`; long tools submit a job and wait on its frames; drain is the daemon job engine's.
- P11, §32.12.9 transports table, HTTP row: `frobd --mcp` can host the shared server.
- P11, §32.12.10 sketch: field `daemon` → `dispatch` (two occurrences) with a comment that the in-process back end is the default.
- P11, §32.21 KILL-MCP row: trigger and the two fallbacks restated as in REV-3.
- P11, §32.22 OPEN-TETHER-1: marked decided with OPEN-SUITE-2 (in process, core graph), identifier kept.
- P11, §37.3: bullet added, "What bounds an MCP client" (capability scopes, registry-generated tool set, imported text as data). No passage of §37 cited the process boundary as a security property, so nothing was removed.
- P11, §37.8 "One runtime package": sentence added that FD-ASUP-01 aligns the siblings and INT-ASUP-01 proves it.
- P11, §37.8 "Pins and locks": builds are on the one toolchain T.
- P11, §37.8 last bullet: "Sibling repositories are read-only … an ask upstream" → changed in place by an upstream work item, never patched locally.

## parts2/P16b.md

- P16b, Appendix F title: "Appendix F. Upstream asks" → "Appendix F. Upstream work items".
- P16b, Appendix F preamble, "The rule": read-only rule replaced by REV-1 (changes made by this program in the sibling under its rules; `upstream` bead mirrored in the sibling tracker; `registries/asks.toml` → `registries/upstream.toml`; no milestone waits; no local patch or fork).
- P16b, Appendix F preamble, second paragraph: "Asks are additive … not a schedule" → upstream changes additive, follow the sibling's rules, file reservations and the sibling's coordination rules; priority orders this program's queue; P0 items at M0.
- P16b, Appendix F preamble, third and fourth paragraphs: "asks" → "upstream work items"; "requests" → "changes".
- P16b, F.1 table header: "Ask / Request" → "Upstream work item / Change".
- P16b, F.1 row FD-ASUP-01: repo list made explicit; P1 → P0 at M0; interim posture no longer "a process boundary where it fails"; switch-over is INT-ASUP-01 on the whole graph.
- P16b, F.1 row FD-FMCP-01: rewritten as "bring fastmcp_rust onto T" (P0, M0), interim posture CLI with KILL-MCP's fallbacks, switch-over INT-FMCP-01 in the core graph.
- P16b, F.1 row FD-FSS-01: "filed only if" → "opened only if".
- P16b, F.1 counts sentence: "53 asks … Ten are P1" → 53 upstream work items; two P0, eight P1, sixteen P2, twenty-seven P3.
- P16b, F.2 table header: "Ask" → "Upstream work item".
- P16b, F.3 title and three bullets: "asks" → "upstream work items"; "`frob-mcp` stays a separate workspace" replaced by KILL-MCP's fallbacks applying only if the two P0 items cannot land within the M0 budget; T also frozen during the tournament.
- P16b, Appendices H, I, J, L: read; no passage describes the MCP boundary and the toolchain FACTs are file citations only; no edit.
