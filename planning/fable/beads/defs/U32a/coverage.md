# Coverage of drafting unit U32a

Chapter §32 Tether (CLI, daemon, MCP, dashboard, viewer), first half: the chapter preamble and §32.1 to §32.12.
The other half (§32.13 to §32.24) is unit U32b's.

Files: 53 bead files (17 seed beads, 36 new beads: `N32a-01` to `N32a-35` and the registry `N32-KA`) and this file.
By milestone: M0 8, M1 9, M2 8, M3 19, M4 8, M5 1.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §32 preamble | What Tether is and is not | N32a-01 |
| §32.1 | What Tether owns | N32a-01 (the ownership map as a record); BD-022 (`frob-verbs`, `frob`); BD-023 (`frob-mcp`); N32a-14 (`frobd`); the dashboard and viewer rows are U32b's |
| §32.2 | Principles | N32a-01 (all six, each bound to a check); N32a-02 (one registry); N32a-04 (one service, two adapters; control-plane granularity); N32a-03 (every answer says what changed; the Five Explicits in the envelope); N32a-15 (questions and budgets); BD-298 (what is claimed for the surface) |
| §32.3 | Decisions | N32a-01 (ADR-TETHER-01 to -11 as records). Stated with reason and excluded alternative in the governing bead: -01 and -02 in N32a-02; -03 in N32a-04; -04 in N32a-16; -05 in N32a-17; -06 in N32a-30 and N32a-29; -07 in BD-295; -08 in N32a-21 and BD-296; -09 in N32a-07; -11 in N32a-09; -10 (dashboard and viewer are ordinary clients) is recorded in N32a-01 and governs U32b's beads |
| §32.4 | The envelope | N32a-03 (schema, field rules, exit codes, refusal and job blocks, one-envelope rule); BD-022 (the CLI that emits it; GATE-TETHER-03) |
| §32.5 | Three failure classes | N32a-19 (fixture, kill harness, M2 record); BD-299 (full M3 surface); N32a-17 (transport class mechanics); BD-023 (error skeletons and the unknown-tool exemption) |
| §32.6 | Verb clusters | BD-022 (operator cluster and M0 entries); BD-138 (`session` cluster); BD-137, BD-222, BD-312, BD-376, BD-409 (the verbs of M1 to M5 by cluster, each mapped to the bead that builds it); N32a-02 (the registry itself, one name per verb) |
| §32.6.1 | The two-phase study | N32a-13 (`study.compare`, `study.cancel`, plan identity, FROB-REF-809, pair accounting); the CPU verbs `study.plan`, `study.submit`, `study.status`, `eval.compare` are seed BD-124 and its verb bead in another unit |
| §32.6.2 | The one-call pipeline | N32a-31 (pilot: plumbing turns), N32ap1-01 (KILL-SOLVE clause 1, before the verb), BD-372 (`solve.run`), N32a-32 (KILL-SOLVE clause 2 and final status) |
| §32.7 | Idempotency | N32a-09 (the admission transaction, PO-TETHER-01, in-process rows); BD-211 (on the daemon; GATE-TETHER-05) |
| §32.8 | Pagination and numbers | §32.8.1: N32a-10 (cursor, PO-TETHER-02), BD-211 (GATE-TETHER-06). §32.8.2: N32a-07 (types, lint, double-decoding client; GATE-TETHER-07 at M1), BD-211 (re-read at M2) |
| §32.9 | Estimates and budgets | N32a-15 (`--estimate`, what a budget caps, upper-end admission, reservations, attributable device time, the overhead line); BD-297 (the conformal interval per stratum, PO-TETHER-04, GATE-TETHER-11) |
| §32.10 | Jobs: the crash-consistent protocol | N32a-08 (states, trial accounting, leases, cancellation receipt, the ledger and its single writer, restart recovery); N32a-18 (GATE-TETHER-08 end to end); N32a-14 (the daemon as the writer). The chunk-publishing transaction and the job verbs are seeds BD-091 and BD-210, the lab-runtime exploration is seed BD-096, all in other units |
| §32.11 | The daemon | N32a-14 (process, state root, socket, `frob.rpc/1`, one per host, `frobd --mcp`, no-daemon posture); N32a-04 (the framing and the client side); N32a-34 (peer link pilot and OPEN-TETHER-5); N32a-35 (peer link) |
| §32.11.1 | Slot-level episode dispatch | BD-295 (mechanism, arithmetic, ceiling, budgeted mode, PO-TETHER-03, shadow rollout, GATE-TETHER-09); N32a-23 (KILL-SLOT) |
| §32.11.2 | Size classes, shortest estimated work, preempt-resume | BD-296 (mechanism, admission guard, GATE-TETHER-10); N32a-20 (M/G/1 model, simulator, Kingman's correction, KA-758, CX-351); N32a-24 (KILL-PREEMPT); N32a-22 (OPEN-TETHER-4) |
| §32.11.3 | The decision model | N32a-21 (guard table with hysteresis, loss, three non-interference rules, PO-TETHER-05); BD-296 (wiring) |
| §32.12 | The MCP server (preamble) | BD-023, BD-298 |
| §32.12.1 | What the library is today | BD-023 (the fact table); N32a-17, N32a-25, N32a-26, N32a-27, N32a-28 (the facts each relies on) |
| §32.12.2 | The build graph | BD-023 (INT-FMCP-01), N32a-05 (KILL-MCP), N32a-06 (fallback 1), N32a-02 (`registry-only`) |
| §32.12.3 | How a registry entry becomes a tool definition | BD-023 (`VerbTool`, the derivation table, build settings, error skeletons); N32a-02 (name map lint); N32a-11 (served `tools/list` against the registry) |
| §32.12.4 | Task clusters of at most seven, and how an agent switches | BD-138 (clusters, base three, `session_focus`, 12 kB target); N32a-12 (three fallbacks, OPEN-TETHER-2); BD-298 (all seventeen and the further clusters) |
| §32.12.5 | The bounded synchronous wait | N32a-16; N32a-33 (OPEN-TETHER-3) |
| §32.12.6 | Progress and cancellation | N32a-17 (progress, cancellation); N32a-28 (Tasks pilot), N32a-29 (KILL-MCPTASKS), N32a-30 (Tasks adapter) |
| §32.12.7 | Resources | N32a-25 |
| §32.12.8 | Prompts | N32a-26 |
| §32.12.9 | Transports | N32a-27 |
| §32.12.10 | A tool in the library's style | BD-023 (the handler sketch), N32a-16 (the wait loop sketch) |

Seed beads that draw on U32b's subsections and were written completely here: BD-313 (§32.16 the two sessions,
§32.17 documentation), BD-373 (§32.16 the mission session), BD-299 and N32a-19 (§32.18 exit codes), N32-KA
(§32.20).

## (b) Gates, kill gates and admission gates

| Gate | Beads that measure or decide it |
|---|---|
| GATE-TETHER-01 | BD-313 |
| GATE-TETHER-02 | N32a-11 (the harness); BD-137 (M1), BD-138 (MCP at M1), BD-222 (M2), BD-312 (M3), BD-376 (M4), BD-409 (M5); N32a-02 (generation half) |
| GATE-TETHER-03 | BD-022 (measures), N32a-03 (mechanism and fixture) |
| GATE-TETHER-04 | N32a-19 (first record at M2), BD-299 (full surface at M3), N32a-17 (transport clause) |
| GATE-TETHER-05 | N32a-09 (in-process rows, PO-TETHER-01), BD-211 (on the daemon) |
| GATE-TETHER-06 | N32a-10 (in-process rows, PO-TETHER-02), BD-211 |
| GATE-TETHER-07 | N32a-07 (M1 record), BD-211 (M2 re-read) |
| GATE-TETHER-08 | N32a-18 (end-to-end measurement), N32a-08 (CPU rows), N32a-14 (daemon rows); seed BD-210 in unit U29a also lists it |
| GATE-TETHER-09 | BD-295; read by N32a-23 |
| GATE-TETHER-10 | BD-296; N32a-20 (model and negative control), N32a-21 (ageing clause), N32a-22 (parameters); read by N32a-24 |
| GATE-TETHER-11 | BD-297 |
| GATE-TETHER-12 | BD-298 (all five clauses); N32a-16 (wait), N32a-17 (cancel), BD-138 and N32a-12 (focus), N32a-25 (resources), N32a-26 (prompts) |
| GATE-TETHER-13 | BD-373 (first arm), BD-372 (second arm), N32a-32 (decision read on it), N32a-33 (precondition) |
| GATE-TETHER-14, -15, -16 | Expected from U32b (§32.14, §32.15, §32.23, §32.24 hold their mechanisms); seed BD-300 in U33a lists GATE-TETHER-14 |
| KILL-MCP | N32a-05 (decision), BD-023 (measurement), N32a-06 (fallback 1) |
| KILL-SLOT | N32a-23 (decision), BD-295 (measurement). Not in the brief's list; §32.21 owns it and the Spark units point here |
| KILL-PREEMPT | N32a-24 (decision), BD-296 (measurement), N32a-20 (flip point), N32a-21 (fallback configuration) |
| KILL-MCPTASKS | N32a-29 (decision), N32a-28 (pilot), N32a-30 (the adapter, only if not fired), BD-298 |
| KILL-SOLVE | N32ap1-01 (clause 1 decision: the ceiling, read before BD-372 starts; added in polishing round 1), N32a-32 (clause 2 and final status), N32a-31 (ceiling), BD-372 (A/B arm) |
| KILL-PEERLINK | N32a-35 (trigger, fallback and four-week reading; added in polishing round 1) |
| KILL-TUI, KILL-REPLAYPACK | Expected from U32b |
| INT-FMCP-01 | BD-023 (M0), BD-137 (M1 verbs), N32a-27 and BD-298 (M3 full surface), N32a-05 |
| GATE-ATEL-03 (shared A/B) | BD-372 and N32a-32 name it; owned by another unit (seed BD-374) |

## (c) Counts

- Known answers: 12 (KA-755 to KA-766), all in `N32-KA` with derivations; each cited with its values in at least two mechanism beads.
- Counterexamples: 7 (CX-345 to CX-351), each stated with its numbers and turned into a negative control in a mechanism bead (CX-345 in N32a-09 and BD-211; CX-346 and CX-347 in N32a-10 and BD-211; CX-348 in N32a-07 and BD-211; CX-349 and CX-350 in N32a-08 and N32a-18; CX-351 in N32a-20 and BD-296). The registry bead `N32-CX` is U32b's.
- Refusal codes: all 18 of the chapter (701, 702, 703, 704, 720, 723, 724, 725, 801 to 810) are stated with meaning and exit code in N32a-03 and used in mechanism beads. The registry bead `N32-REF` is U32b's.
- Invariants: INV-TETHER-01 to -11 and -13 are carried as tests (01: N32a-11; 02: N32a-03; 03: N32a-08, N32a-15; 04: N32a-08, N32a-13; 05: N32a-09, BD-211; 06: N32a-10; 07: N32a-07; 08: BD-295; 09: N32a-21, BD-296; 10: N32a-14, N32a-17; 11: N32a-17; 13: N32a-26, BD-023, BD-298). INV-TETHER-12 (viewer and dashboard change no digest) is recorded in N32a-01 and belongs to U32b's beads.
- Proof obligations: PO-TETHER-01 (N32a-09), -02 (N32a-10), -03 (BD-295), -04 (BD-297), -05 (N32a-21).
- Architecture decisions: ADR-TETHER-01 to -11 (N32a-01 and the governing beads above).
- Open questions: OPEN-TETHER-1 (decided; recorded in N32a-05, BD-023, N32a-01), -2 (N32a-12), -3 (N32a-33), -4 (N32a-22), -5 (N32a-34, label `owner-decision`). From other chapters, placed here: OPEN-ATEL-4 (decided at the registry freeze, BD-312), OPEN-AUGUR-7 (N32a-03, BD-137), OPEN-CHRON-5 (N32a-08), OPEN-SUITE-6 (N32a-04), OPEN-SPARK-2 (BD-296, N32a-15).
- Pilots for Frontier mechanisms: N32a-28 (Tasks adapter), N32a-31 (`solve.run`), N32a-34 (peer link). The slot-dispatch pilot is the Spark unit's.
- Risks named: the surface-size risk (BD-313, BD-298, BD-373), RISK-49 (BD-373, N32a-26), RISK-02 and RISK-03 (BD-023, N32a-05).

## (d) Needs from other slices

| Bead | Needs |
|---|---|
| N32a-03, N32a-19, BD-299 | The refusal registry with exactly one exit code per code across all chapters (`N32-REF` of U32b and every unit's REF bead); explicit exits for the families §32.4 does not name (`71x`, `74x` to `79x`) |
| N32a-02, N32a-07 | `xtask check-generated`; the canonical JSON writer and the float-spelling rule (§34.14) |
| BD-022 | The stack lock, resolution receipt and admission records (§33.5); `spark.devices`; the gate status recorder (§38.13); `registries/upstream.toml` |
| BD-023, N32a-27 | The re-read of the fastmcp_rust facts at the pinned commit (the suite unit's admission bead); `check-deps` forbidding a second asupersync package |
| N32a-08, N32a-18, BD-211 | The chunk-publishing transaction on CPU and devices (seeds BD-091, BD-210); the lab-runtime harness (seed BD-096); retention with tombstones honouring cursor pins (seed BD-209); device fault injection (seed BD-175) |
| N32a-09, N32a-27, N32a-25, BD-298, BD-299 | Capability tokens, scopes, attenuation, namespaces (§32.13; seed BD-258, unit U37) |
| BD-138, N32a-16, N32a-17 | Registry entries and daemon forms of the job verbs (seed BD-210); their in-process M1 forms are served by BD-138 |
| N32a-13 | The CPU study verbs (seed BD-124 and its verb bead); the verdict rule (seed BD-262) |
| BD-295 | The Spark side of slot dispatch and its pilot (§21.11; the Spark unit's new beads); the Phylum slot table (§17.8) |
| BD-296, N32a-21 | Snapshot contents for resume (seeds BD-203, BD-189); the DC-09 record (seed BD-309); the compute classes (OPEN-SPARK-2) |
| BD-297, N32a-15 | The cost model's point prediction and the fingerprint fields that define an epoch (seeds BD-219, BD-029) |
| N32a-25, N32a-26, BD-373 | The private mark and FROB-REF-731 (seed BD-305); the untrusted mark and the hostile corpus (seed BD-304); a refusal code for an oversized resource read |
| BD-313, BD-373, N32a-12, N32a-31, N32a-33 | The pinned agent harness configurations (§39.5); the design corpus with planted truths (seed BD-248); recorded sessions of the partial ten-fold arm (seed BD-316) |
| BD-372, N32a-32 | The surface A/B, its registration with the pre-registered factor, and its adjudication (seed BD-374 and the Atelier unit's protocol bead) |
| BD-137, BD-222, BD-312, BD-376, BD-409 | Handlers of verbs that no seed bead names: `design.new`, `design.show`, `design.import`, `design.export`, `run.export`, `module.characterize`, `sense.visibility`, `task.audit`, `eval.items`, `contact.clearance`, `fidelity.rank`, `scene.fit`, `scene.retrofit`, `scene.hazards`, `scene.probe-request`, `control.lift` |
| N32a-34, N32a-35 | The fleet scheduler, its role table, pilot and KILL-FLEET (seed BD-370 and the Spark unit); a threat review for a network listener (the safety unit) |
| N32a-05 | The M0 exit review (seed BD-044) does not list a KILL-MCP decision bead among its dependencies; the lead should link it to N32a-05 |

Other units asked this unit for: the session recorder (delivered as the session transcript record in BD-313); the
dispatch guard table and its place in the configuration digest (N32a-21, BD-296); the peer link and its pilot
(N32a-34, N32a-35); the M1 job engine and idempotency binding for the lab-runtime tests (N32a-08, N32a-09);
KILL-SLOT and KILL-MCP decision beads (N32a-23, N32a-05); the envelope's refusal block and exit table (N32a-03).

## (e) Contradictions and things that could not be turned into work as written

1. **Two codes for one chunk conflict.** §32.10 and §32.18 give `FROB-REF-810` (exit 9) for different bytes on a committed extent; §29.17 gives `FROB-REF-878`. The beads use 810 for the job-level failure a caller sees and say that the registry owner must state whether they are one code (N32a-08, N32a-18, N32-KA).
2. **Where the daemon's code lives.** §32.1 puts the daemon in `frob-cli`; §34.2 gives `frob-verbs` "the daemon's job engine". Resolved as: process shell in `frob-cli`, job engine, ledger and admission as libraries in `frob-verbs` (N32a-01, N32a-14).
3. **GATE-TETHER-04 is due at M2 (§40, §43.5) but its seed bead BD-299 is at M3** behind the full MCP surface. Added N32a-19 to record the gate at M2 on the M2 surface; BD-299 closes it at M3.
4. **GATE-TETHER-07 is due at M1 (§40; WP-06) but its seed bead BD-211 is at M2.** Added N32a-07 to record it at M1; BD-211 re-reads it.
5. **Session verbs.** Appendix J dates `session.open`, `.show`, `.focus` at M0; the seed that builds them (BD-138) is M1. They are registered as declared at M0 (BD-022).
6. **Job verbs.** Appendix J dates the job verbs at M2, but M1 `run` tools over MCP need `job_wait` once the bounded wait elapses. BD-138 serves in-process CPU forms of `job_wait` and `job_cancel` at M1.
7. **Verb counts.** Appendix J.3 says 55 named verbs and a registry of 243; counting J.3's rows gives 56 and 244. No bead hard-codes the number; BD-409 tests against the list of names.
8. **First milestones in Appendix J disagree with seed beads** for several verbs (earlier in seeds: the CPU study verbs, `mission.distinguish`, `mission.separate`, the revalidation pair, the CPU sensing verbs, `fidelity.shadow`; later in seeds: `contact.laws`, `ground.ingest`, `task.settle`, `shield.budget`, `shield.check`, `foundry.loads`, `foundry.size`, `ground.excite`). Rule applied in the verb-set beads: the registry's `first_milestone` is that of the first reduced form; a verb without a handler is registered as declared and answers `FROB-REF-724`.
9. **Verbs with two kinds** (`nemesis.hardset`, `scene.bank`, `spark.cache`): the plan does not say how one entry carries two kinds. DESIGN in N32a-02: a kind list with an `actions` table.
10. **`study.plan` is `mutate` and "idempotent by construction", while every `mutate` verb must take an idempotency key.** Resolved as a narrow, lint-checked exemption for verbs whose result identifier is the content address of the resolved request (N32a-13, N32a-09).
11. **`study.cancel` and `result.show`** are attributed to §32.6 by Appendix J and have no seed bead anywhere. Carried in N32a-13 and BD-376.
12. **Exit codes for refusal families §32.4 does not name** (`71x`, `74x` to `79x`; other units noticed the same). N32a-03 requires an explicit exit in each such registry row and does not choose the values.
13. **No refusal code for a resource read above 4 MiB** (§32.12.7 says such reads are refused in favour of `artifact.read`). N32a-25 flags it as a need.
14. **Two spellings of the telemetry order key** (§32.8.1: instant, world address, channel, sample index; §29.4: tick, world, episode, stream, sample identifier). N32a-10 requires only that the declared key be total.
15. **GATE-TETHER-10's replayed mix against the one-hour machine envelope.** The model case (30 min jobs at utilization 0.7) needs many hours of device time for a meaningful sample. BD-296 measures the model case in the simulator lane and the gate on a device lane inside the envelope, and says a full-scale replay needs an approved compute class.
16. **KILL-SLOT** is missing from the brief's gate list although §32.21 owns it and §40.7 says §32 governs; written as N32a-23.
17. **"The M0 budget"** in KILL-MCP's trigger is not a number; read as "still failing at the M0 exit review" (N32a-05).
18. **"The pre-registered factor"** of GATE-TETHER-13's second arm has no value in the plan (the hypothesis is five-fold). N32a-32 requires it to be registered by the A/B before data, bounded by the pilot's ceiling, with a fail-closed default of 1.5 (DESIGN).
19. **"Budget violation"** in GATE-TETHER-01 and -13 is undefined; defined in BD-313 and BD-373 as a task ending in `FROB-REF-702` or spending above its per-task budget (DESIGN).
20. **No verb-set seed for M6 and M8.** `control.lift`, `foundry.lifecycle`, `mission.transport`, `skill.transport` and `foundry.export --with-firmware` are registered as declared by BD-409 and flipped by their epics.
21. **The conformal calibrator of asupersync** is offered by §32.9 "to consume or follow"; another unit's reading of that file is that it calibrates the lab's own oracle metrics. BD-297 follows the pattern and does not link the code.
22. **Envelope field names for "every answer says what changed"** are not given; N32a-03 names a `delta` block as DESIGN.
23. **MCP Tasks state names** are not given; N32a-28 and N32a-30 treat them as facts to read from the library at the pin.
24. **The peer link's transport** is not specified and would be the system's only network listener; N32a-35 specifies it as DESIGN, off by default, behind the owner's decision and a threat review.
25. **Decision procedures for the kill gates** (what counts as "after one optimization pass", per-device-family decisions, classification of an INT-FMCP-01 failure) go beyond the plan's one-line triggers; each is labelled DESIGN in its decision bead.
26. **KA-760 and KA-762** are arithmetic of the blinded qualification lane (§24) stated in §32.20 because the worked session uses them; N32-KA carries them and notes that the lane's own registry must agree.
