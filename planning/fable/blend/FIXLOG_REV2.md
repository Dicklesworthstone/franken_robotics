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

## parts2/P14.md

- P14, §41.1 pre-mortem row 11: residual "depends on sibling progress" → the proof siblings maturing is upstream work of this program.
- P14, §41.1 pre-mortem row 14: "asks FD-FT-01" → "upstream work item FD-FT-01".
- P14, §41.1 pre-mortem row 25: "the toolchain split blocked a merge" restated as the toolchain train stalling; countermeasure is one toolchain T, FD-FMCP-01 and FD-ASUP-01 at M0, KILL-MCP's fallbacks; four nightlies named the starting state.
- P14, §41.3 register intro: RISK-02 and RISK-20 named as restated.
- P14, §41.3 RISK-01: "ask FD-FSIM-08" → "upstream work item FD-FSIM-08".
- P14, §41.3 RISK-02: restated per REV-1 item 4 as the cost and breakage risk of the toolchain train; the toolchain FACTs kept as the starting state; mitigation is the ordered choice of T, FD-FMCP-01 at P0, the train rule, KILL-MCP's fallbacks.
- P14, §41.3 RISK-03: mitigation "process boundary as fallback" → FD-ASUP-01 at P0 and KILL-MCP's first fallback.
- P14, §41.3 RISK-08: "asks FD-FT-01" → "upstream work item FD-FT-01".
- P14, §41.3 RISK-20: "upstream asks are never staffed" restated as the cost and sequencing of upstream work, with signal and mitigation.
- P14, §41.3 RISK-21: "whole core" → "whole build graph"; signal includes a toolchain train; preferred fix is an upstream change (FD-FSIM-15, -31), `frob-units` the fallback.
- P14, §41.3 RISK-31: "asks FD-FSIM-12 and -13" → "upstream work items".
- P14, §41.3 new row RISK-55 (next free number, appended after RISK-54): collisions with other agents working in the same sibling repository; mitigation is file reservations and the sibling's own coordination rules.
- P14, §41.4 last bullet: "An upstream ask is never a mitigation by itself" → an upstream work item is not; its interim posture is.
- P14, §41.5 second row: "the pin move is an upstream ask" → upstream work item (FD-FTX-01, FD-F3D-01).
- P14, §42.1 count sentence: closed questions two → four.
- P14, §42.2 OPEN-SPARK-5: "if the gateway's asks never land" → before the upstream work item FD-FT-01 lands.
- P14, §42.2 OPEN-SUITE-2: marked decided by the owner, with the decision text, its cost and the gates that confirm it.
- P14, §42.3 OPEN-SINEW-4: "needs an upstream ask / file the ask" → upstream change / open the upstream work item (FD-FSIM-21).
- P14, §42.3 OPEN-SPARK-5: "or the asks of FD-FT-01 never land" → until the upstream work item FD-FT-01 lands.
- P14, §42.3 OPEN-NEMESIS-4: "an ask is filed and nothing is scheduled against it" → upstream work items FD-FSIM-18 and -19 open, no milestone waits.
- P14, §42.3 OPEN-ATEL-3: "on the core toolchain" → "on T".
- P14, §42.3 OPEN-TETHER-1: marked decided with OPEN-SUITE-2.
- P14, §42.3 OPEN-SUITE-1: default restated around FD-ASUP-01.
- P14, §42.3 OPEN-SUITE-2: marked decided, with the decision text.
- P14, §42.3 OPEN-SUITE-6: "only if INT-ASUP-01 forces a process boundary" → only if it fails after FD-ASUP-01 and a fallback splits the graph.
- P14, §42.3 OPEN-WS-2: default restated (core; upstream change or toolchain train).
- P14, §42.4 closing paragraph: "Two questions are closed" → four, naming OPEN-SUITE-2 and OPEN-TETHER-1 and the decision; paragraph rewrapped.
- P14, §43.2 firewall table, row M0: choice of T and the two P0 upstream work items added.
- P14, §43.2 firewall table, row M7: T frozen with the pins.
- P14, §43.3 M0 artifact: admission file also records the choice of T; new bullet for fastmcp_rust on T (FD-FMCP-01) and one asupersync version (FD-ASUP-01).
- P14, §43.3 M0 artifact: "through `frob-mcp` across the process boundary" → `frob-mcp` in the core graph on T, linking `frob-verbs`, in process.
- P14, §43.3 M0 acceptance: "INT-FMCP-01 as a smoke test" → the REV-3 criterion; upstream items landed or KILL-MCP decided.
- P14, §43.3 M0 kill gates: KILL-MCP restated (trigger after FD-FMCP-01 and FD-ASUP-01 attempted; fallback 1 own workspace over a socket; fallback 2 CLI); choice of T follows §33.12 and §33.13.
- P14, §43.4 M1 kill gates (schedule gate): wording aligned with BET-12 (upstream work item not landed, no interim posture); paragraph rewrapped.
- P14, §43.4 M1 owner input: OPEN-SUITE-2 removed (decided).
- P14, §43.12 variants table, second M0 row: "the MCP stack cannot be reached across the process boundary" → INT-FMCP-01 cannot pass within the M0 budget; exits with KILL-MCP's two fallbacks.
- P14, §44 intro: sentence added that upstream work items are beads labelled `upstream` inside existing packages; the two P0 items sit in WP-02 and WP-06.
- P14, §44.1 WP-01: name extended (the one toolchain T and the stack lock).
- P14, §44.1 WP-02: name and crates extended with FD-ASUP-01.
- P14, §44.1 WP-06: name and crates extended with FD-FMCP-01 and `frob-mcp` in the core graph, in process.
- P14, §44.3 coverage: `surface/frob-mcp` moved from the Annexes row to the layer 7 row.
- P14, §46 working rules: "no sibling repository is modified" → changed only as an upstream work item, in that repository, under its rules, with files reserved.
- P14, §46 step 1: "FrankenSim's toolchain pin" → one `rust-toolchain.toml` naming candidate (a) as provisional T; paragraph rewrapped.
- P14, §46 step 3: extended (step numbers kept) with: T falls to candidate (b) if FrankenSim does not build on (a); build fastmcp_rust on T in its repository, run its suite and conformance checks, record the result (FD-FMCP-01, BD-023).
- P14, §46 observation table, row 3: observation and decisions extended to match.
- P14, §46 closing paragraph: three honest measurements → four (fastmcp_rust on T added).
- P14, §47.1 "The foundations are respected": "none was modified. Every ask has a bead in its home tracker" → none was patched locally; changes made in the sibling as upstream work items with mirrored beads; one toolchain T.

## parts2/P15.md

- P15, §45 "How to read a row", sibling bullet: "Sibling repositories are read-only. No bead schedules an upstream change" → upstream work is bead work (label `upstream`, mirrored bead; BD-037; P0 items in BD-008 and BD-023; interim posture).
- P15, §45.1 intro: "The MCP server is a separate workspace across a process boundary. The core toolchain follows FrankenSim's" → M0 chooses T, brings fastmcp_rust onto it, aligns asupersync; MCP server in the core graph, in process.
- P15, §45.1 BD-001 title: `rust-toolchain.toml` "at FrankenSim's pin" → one file for the whole graph naming candidate (a) as provisional T.
- P15, §45.1 BD-006 title and done-condition: built on candidate (a), else (b); the record names the candidate FrankenSim builds on.
- P15, §45.1 BD-008 title and done-condition: extended to cover FD-ASUP-01 (no new bead, no new dependency edge).
- P15, §45.1 BD-023 title and done-condition: "own workspace on its annex toolchain … across the process boundary" → FD-FMCP-01 (fastmcp_rust onto T) and `frob-mcp` in the core workspace, in process; done-condition is the REV-3 INT-FMCP-01 criterion (no new bead, dependencies unchanged).
- P15, §45.1 BD-031 title and done-condition: adds the choice and recording of T; every row passes on T.
- P15, §45.1 BD-037 title and done-condition: "asks filed … no local bead schedules an upstream change" → upstream work items opened as `upstream` beads and mirrored; `registries/upstream.toml`.
- P15, §45.1 BD-044 title: the state of the two P0 upstream work items is recorded at the M0 exit review.

## parts2/P13b.md

- P13b, §40.2 INT-TOOLCHAIN-01 row: "Rust toolchain shared with FrankenSim … on FrankenSim's pinned nightly" → the gate that chooses and records T for the whole build graph; T then moves only by a toolchain train.
- P13b, §40.2 INT-ASUP-01 row: one published version across workspaces; proves FD-ASUP-01.
- P13b, §40.2 INT-FSYM-01 row: `derive/` builds on T.
- P13b, §40.2 INT-FMCP-01 row: replaced by the REV-3 criterion; milestone label "M0 (smoke)" → "M0 (admission)".
- P13b, §40.2 note under the table: "INT-FMCP-01 is run across the process boundary … not required" → run inside the core graph on T; FD-FMCP-01 and FD-ASUP-01 make it passable; KILL-MCP if it cannot pass.
- P13b, §40.4 KILL-MCP row: trigger and two fallbacks restated as in REV-3 ("the separate workspace is the design of record" removed).
- P13b, §40.5 schedule gate (BET-12) row: wording aligned with BET-12 (upstream work item not landed, no interim posture).
- P13b, §40.7 paragraph on KILL-MCP: history of the row removed; states the design of record (core graph, in process) and the trigger and fallbacks once.

## parts2/P16a.md

- P16a, Appendix A, row "Annex": no longer "outside the core's toolchain"; defined by the lockfile and the unsafe and foreign-interface policy, on T; `surface/` removed from the list and `frob-mcp` named a core workspace member.
- P16a, Appendix A, row "Stack lock": records the one toolchain T.
- P16a, Appendix A, new row "Toolchain T; toolchain train" (alphabetical position).
- P16a, Appendix A, row "Upstream ask" → "Upstream work item", redefined per REV-1.
- P16a, Appendix B, row "Symbolic elimination of kinematic loop closures": "waits on an upstream ask" → "upstream work item (FD-FSIM-04)".
- P16a, Appendix C EV matrix, row "MCP surface on fastmcp_rust": "across a process boundary" → in the core graph with verbs run in process; KILL-MCP's two fallbacks named.
- P16a, Appendix C killed-idea register, group 63, row on sibling changes: "scheduled as a local bead / siblings are read-only / upstream asks" → a local patch or fork is what is not adopted; upstream work items done in the sibling.

## parts/P05.md

- P05, §21.2 ADR-SPARK-01, second bullet: annex workspaces kept with their own lockfiles; sentence added that they build on T and that the boundary exists for the unsafe and foreign-interface policy.
- P05, §21.3 ADR-SPARK-09: "upstream asks, never local beads / sibling repositories are read-only" → upstream work items done in FrankenTorch's repository (FD-FT-01), never a local patch or second binding; reason restated.
- P05, §21.5 stride paragraph: "an upstream ask (Appendix F)" → "an upstream work item (FD-FNP-01, Appendix F)".
- P05, §21.9 gateway limits table: header "Ask" → "Upstream work item"; last row "pinned as upstream pins it; migration is upstream's" → pinned as FrankenTorch pins it; migration is part of FD-FT-01.
- P05, §21.9 sentence under the table: "Until the asks in Appendix F land … does not schedule a change to it" → until the upstream work of FD-FT-01 lands; no local patch, no second binding.
- P05, §21.10 kernel cache bullet: "part of ask FD-FT-01" → "part of upstream work item FD-FT-01".
- P05, §21.25 OPEN-SPARK-5: "or the asks of FD-FT-01 never land" → until the upstream work item FD-FT-01 lands.
- P05, §35.1 determinism classes paragraph: "that is ask FD-FSIM-07" → "upstream work item FD-FSIM-07".

## parts/P01.md, P02.md, P03.md, P04.md, P06.md, P08.md, P09.md, P10.md, parts2/P13a.md (wording only: "ask" → "upstream work item")

- P01, §10 C-06 (Foundations bullet): "an upstream ask" → "an upstream work item".
- P01, §10 C-07 (Foundations bullet): "an upstream ask" → "an upstream work item".
- P01, §10 C-25 (Foundations bullet): "an upstream ask (Appendix F)" → "an upstream work item (FD-FSIM-17, Appendix F)".
- P02, §14.10 differential-drive known answer: "the upstream ask" → "the upstream work item".
- P02, §15.14 OPEN-SINEW-4: "needs an upstream ask. Default: file the ask" → upstream change; open the upstream work item (FD-FSIM-21).
- P03, §16.2 ADR-KIN-06 reason: "sibling limits are facts to plan around" → facts at the pinned commit, each with an interim posture until its upstream work item lands.
- P03, §16.4 paragraph before the limits table: "(asks FD-FSIM-02 and FD-FSIM-04)" → upstream work items.
- P03, §16.4 lead-in to the limits table: "each has an upstream ask …; none is scheduled as local work on FrankenSim" → each has an upstream work item done in FrankenSim's repository; the posture holds until it lands.
- P03, §16.4 limits table: header "Posture until the ask lands" → "…until the upstream change lands"; four cells "Ask FD-FSIM-0x" → "Upstream work item FD-FSIM-0x".
- P03, §16.8 promotion paragraph: "a batch entry point is an upstream ask" → upstream work item FD-FSIM-17; lines rewrapped.
- P03, §18.3 collision paragraph: "(ask FD-FSIM-03)" → "(upstream work item FD-FSIM-03)".
- P03, §18.9 planar-drive paragraph: "an upstream ask" → "an upstream work item (FD-FSIM-11)".
- P03, §18.11 capacity list, item 2: "an upstream ask for the referee" → "an upstream work item for the referee, FD-FSIM-13".
- P04, §19.10.6: "are asks upstream, never local beads" → upstream work items (FD-FSIM-22); no milestone waits.
- P04, §19.13 ingestion: "Changes to either are asks upstream" → upstream work items (FD-FSS-01, FD-FDGR-01).
- P04, §19.13 ceiling, pilot and kill: "an upstream ask and not an importer" → upstream work item (FD-FSS-01); paragraph rewrapped.
- P06, §22.9.3: "waits on upstream asks" → "waits on upstream work item FD-FSIM-20".
- P06, §22.20 FACT paragraph: "a request for it goes upstream" → adding it to `fs-surrogate` is an upstream change.
- P06, §23.6 PPO paragraph: "admission tests and upstream asks" → "upstream work items".
- P06, §23.8 device learning bullet: "An upstream ask (FD-FT-02) requests …. If it lands" → "An upstream work item (FD-FT-02) adds …. When it lands".
- P08, §26.16 OPEN-NEMESIS-4: "an upstream ask is filed and nothing is scheduled against it" → upstream work items FD-FSIM-18 and -19 open; no milestone waits.
- P09, §28.24 OPEN-ATEL-3: "on the core toolchain" → "on the toolchain T".
- P10, §30.4: "upstream asks, to be numbered in Appendix F" → upstream work items, numbered FD-FSIM-23 to -25; rewrapped.
- P10, §31.4: "Upstream asks, to be numbered in Appendix F" → upstream work items, numbered FD-FSIM-26 to -30; rewrapped.
- P13a, Appendix N, row CX-029: "an upstream ask" → "an upstream work item".
- P13a, Appendix N, row CX-182: "the upstream ask" → "the upstream work item FD-FSIM-11".

## Later touches to files already logged above

- P00, §4 BET-34 fallback cell: "Upstream asks." → "Upstream work items FD-FSIM-12 and FD-FSIM-13. Until they land, …".
- P12, §34.1 workspace paragraph: sentence added that `worker/` builds on an older pin's toolchain only when KILL-FSIM's fallback (a) keeps the worker at that pin.
- P12, §34.3 L-5, §34.8 and §34.14: rewrapped after the edits above (no change of meaning).
- P12 §33.8, P16b F.1, P15 BD-023, P14 §46 step 3: FD-FMCP-01 wording made exact: the toolchain file changes, and the declared `rust-version` changes only where T requires it.

## Not edited, by instruction or by finding

- `<!-- LEDGER -->` blocks in every part still use the old wording ("upstream asks", "process boundary", "own toolchain file", "OPEN-SUITE-2 to -6" as open). They were not touched.
- "process boundary" remains outside ledgers only for mechanisms that are not the MCP server: ADR-SUITE-02 and KILL-FSIM fallback (a) (P12 §33.8, §33.14), the Gymnasium adapter (OPEN-WS-5 in P12 §34.13 and P14 §42.3; OPEN-CORTEX-4 in P14 §42.3). KILL-MCP's first fallback is worded "its own workspace … over a local socket".
- GPU annexes: kept as separate lockfile workspaces on T (ADR-SPARK-01, OPEN-SPARK-1, INT-FT-01 and BD-016 unchanged), on the FACTs cited in P12 §34.1 (engine-kit pattern outside FrankenSim's workspace; `cudarc` third party; FrankenTorch's `block` patch at the resolving root). Only the toolchain-file statements were withdrawn.
- §45: no bead was added. FD-FMCP-01 is carried by BD-023 and FD-ASUP-01 by BD-008; adding a bead or an edge would have broken the topological numbering or the recorded edge count and longest chain of §45.
- §37: no passage cited the process boundary as a security property; one bullet was added to §37.3 stating what bounds an MCP client.
- P09 §29.14 sample record still shows `"toolchain":"nightly-2026-07-06"`; it is an illustrative value and equals candidate (b).
