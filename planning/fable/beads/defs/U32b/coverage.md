# Coverage of drafting unit U32b: §32.13 to §32.24 (Tether: capabilities, dashboard, viewer, sessions, documentation, refusals, invariants, known answers, gates, open questions, the diagnosis battery, the replay packet)

Files written: 45 bead files (`N32-REF`, `N32-CX`, `N32b-01` to `N32b-43`) and this file. Seed beads owned by this
unit: none (the chapter's 17 seed beads are written by U32a; the seeds that touch these subsections from other
chapters are BD-191 by U24a, BD-258 by U37, BD-300 by U33a).

Read first: nine of the 43 new beads overlap decision or experiment beads that the chapter's other unit wrote
while this unit was drafting. They are listed in part (e), item 1. Each carries an overlap notice in its Scope.
This unit tried to remove them and was stopped by the repository's command guard, which requires the owner's
permission for file deletion; they are therefore left in place for the lead to drop or merge.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §32.13 | Capabilities, namespaces and many agents | N32b-01 (scope catalogue, token, minting, check before every verb, FROB-REF-720/-723), N32b-02 (attenuation, budget debited from the parent), N32b-03 (namespaces, lineage ownership, FROB-REF-725, shared archive as a grow-only set, leaderboard total order, lineage identifiers as mail threads), N32b-04 (`session.open`, `session.show`), N32b-05 (end-to-end with many agents). Seed BD-258 (U37) carries scopes, session budgets and the path jail under its safety gate |
| §32.14 | The dashboard | N32b-06 (core, read-only rule, cancel with scope and confirmation), N32b-07 (Races, Budgets, Evidence, gate view), N32b-08 (Devices, Queue, Failures), N32b-09 (conformance: dashboard half of GATE-TETHER-14), N32b-31 (KILL-TUI). Seed BD-300 (U33a) is the admission and gate record |
| §32.15 | The viewer and its overlays | N32b-10 (the page, snapshot sources, picking, test entry), N32b-11 (overlay interface; contact cones, energy flow, actuator state, balance, sensing), N32b-12 (film and friction, clearance, counterfactual ghost, world difference, reality band; deviation and unknown-region layers), N32b-13 (packaging, FrankenThreeD route, byte budgets, RISK-19), N32b-14 (conformance: viewer half of GATE-TETHER-14). Stills and pixel inspection are seed BD-191 (U24a) |
| §32.16 | Two sessions, end to end | N32b-15 (the studio session as a script), N32b-16 (the mission session as a script, with the four rules and the exact qualification arithmetic). Seeds BD-313 and BD-373 (U32a) run the fresh-agent tests on them |
| §32.17 | Documentation for agents | N32b-17 (`robot-docs`, `--cluster`, token budget, compact encoding), N32b-18 (skill file, `frob://docs` texts, prompt texts, drift check, the import-text screen), N32b-19 (`capabilities`), N32b-20 (`schema`, `doctor`). Seeds BD-022 and BD-313 (U32a) hold the first form and the completed guide |
| §32.18 | Refusals | N32-REF (the eighteen rows with triggers, repairs, retry, exit; the uncoded surface refusals), N32b-21 (planted-violation battery on both surfaces; the three standing rules) |
| §32.19 | Invariants and proof obligations | N32b-22 (INV-TETHER-01 to -13, one property test and one planted violation each), N32b-23 (PO-TETHER-01 to -05: written arguments and checks). INV-TETHER-10 and -12 also in N32b-06, N32b-09, N32b-10, N32b-14; INV-TETHER-13 also in N32b-18 |
| §32.20 | Known answers and counterexamples | N32b-24 (KA-755 to KA-766 executable; control rows), N32-CX (CX-345 to CX-351 and unnumbered cases). The registry bead `N32-KA` is U32a's |
| §32.21 | Gates | N32b-25 (the register rows for all sixteen gates and seven kill gates, with owners, controls and reconciliations), N32b-09 and N32b-14 (GATE-TETHER-14), N32b-39 (GATE-TETHER-15), N32b-43 (GATE-TETHER-16), N32b-31 (KILL-TUI), N32b-32 (KILL-REPLAYPACK); overlapping drafts N32b-26 to N32b-30 for the five kill gates whose mechanisms are U32a's |
| §32.22 | Open questions | Summarized with owners, blocks and defaults in N32b-25 (Considerations). OPEN-TETHER-1 (decided) with the KILL-MCP decision; OPEN-TETHER-2 to -5: U32a's N32a-12, N32a-33, N32a-22, N32a-34; overlapping drafts N32b-33, N32b-34, N32b-35, N32b-36 |
| §32.23 | The agent diagnosis battery | N32b-37 (thirty tasks, seeding rules, minimal evidence, sealed key), N32b-38 (what the battery needs of transcripts; counts; the clean client), N32b-39 (scorer, two arms, GATE-TETHER-15) |
| §32.24 | The offline replay packet | N32b-40 (the page reading a packet, verification, rendering policy, in-page probe), N32b-41 (forks, tables, licence switch; the format and size arithmetic restated in full), N32b-42 (fork ghost and tables panel in the page), N32b-43 (GATE-TETHER-16), N32b-32 (KILL-REPLAYPACK), N32b-13 (the 600,000-byte page budget). The parent-only writer is N24a-41 (U24a) |

## (b) Gates, kill gates and admission gates

| Gate | Milestone | Measured or decided by | Unit |
|---|---|---|---|
| GATE-TETHER-01 | M3 | Seed BD-313; its scripted precondition is N32b-15 | U32a |
| GATE-TETHER-02 | M1 | Seeds BD-137, BD-138, BD-222, BD-312, BD-376, BD-409; N32a-02, N32a-11. This unit's verb beads N32b-04, N32b-17, N32b-19, N32b-20 name it for their verbs' parity | U32a (and this unit for its verbs) |
| GATE-TETHER-03 | M0 | Seed BD-022; N32a-03. N32b-24 names it for KA-755 | U32a |
| GATE-TETHER-04 | M2 | Seed BD-299; N32a-19. N32b-21 measures its one-exit-per-code clause | U32a (clause: this unit) |
| GATE-TETHER-05, -06, -07 | M2, M2, M1 | Seed BD-211; N32a-09, N32a-10, N32a-07. N32b-24 binds their negative controls | U32a |
| GATE-TETHER-08 | M2 | Seed BD-210 (U29a); N32a-18. N32b-24 binds its controls | U29a, U32a |
| GATE-TETHER-09 | M3 | Seed BD-295 | U32a |
| GATE-TETHER-10 | M3 | Seed BD-296; N32a-20 to N32a-22 | U32a |
| GATE-TETHER-11 | M3 | Seed BD-297 | U32a |
| GATE-TETHER-12 | M3 | Seed BD-298; N32b-18 measures its "prompts match `robot-docs`" clause | U32a (clause: this unit) |
| GATE-TETHER-13 | M4 | Seed BD-373; second arm seed BD-372; scripted precondition N32b-16 | U32a |
| GATE-TETHER-14 | M3 | N32b-09 (dashboard half), N32b-14 (viewer half); recorded by seed BD-300 | This unit; U33a records |
| GATE-TETHER-15 | M3 (not in §40's main tables; §40.9) | N32b-39 | This unit |
| GATE-TETHER-16 | M3 (§40.9) | N32b-43 | This unit |
| KILL-MCP | M0, re-read at M1 | N32a-05 (overlapping draft N32b-26); recorded at the exit review BD-044 | U32a |
| KILL-SLOT (not in the brief's list; in §32.21 and §40) | M3 | N32a-23 (overlapping draft N32b-27) | U32a |
| KILL-PREEMPT | M3 | N32a-24 (overlapping draft N32b-28) | U32a |
| KILL-MCPTASKS | M3 | N32a-29 (overlapping draft N32b-29) | U32a |
| KILL-SOLVE | M4 | N32a-32 (overlapping draft N32b-30) | U32a |
| KILL-TUI | M3 | N32b-31, from the measurements of N32b-09; seed BD-300 records the outcome | This unit |
| KILL-REPLAYPACK | M3 (§40.9) | N32b-32, from the record of N32b-43 | This unit |
| INT-FMCP-01, INT-FTUI-01, INT-F3D-01 (admission gates defined in §33, used here) | M0, M3, M3 | Seed BD-023 (U32a); seed BD-300 (U33a). N32b-09 supplies the INT-FTUI-01 measurements; N32b-13 the route for INT-F3D-01 | Other units |

All sixteen gates and seven kill gates have a register row specified in N32b-25.

## (c) Counts

- Subsections covered: 12 of 12.
- Gates: 16 acceptance gates and 7 kill gates registered (N32b-25); measured or decided in this unit: GATE-TETHER-14,
  -15, -16, KILL-TUI, KILL-REPLAYPACK, plus two clauses (of -04 and -12).
- Known answers: 12 (KA-755 to KA-766), all executable in N32b-24; additional known answers stated by §32.24 (four
  packet sizes, the null-fork zero, the refused 6,200,000-byte request, probe equality) are in N32b-40 to N32b-43.
- Counterexamples: 7 catalogued (CX-345 to CX-351) in N32-CX, plus 3 without a catalogue number (one from §32.24,
  two DESIGN additions).
- Refusal codes: 18 (701, 702, 703, 704, 720, 723, 724, 725, 801 to 810) in N32-REF, plus 3 provisional new codes
  and 2 mappings to existing codes for refusals the chapter states without a code.
- Invariants: 13 (INV-TETHER-01 to -13). Proof obligations: 5 (PO-TETHER-01 to -05). Decisions: ADR-TETHER-10 and
  -11 are stated where they govern (N32b-06, N32b-10, N32b-03); ADR-TETHER-02, -03, -04, -06, -07 and -08 are restated in the
  kill-gate and experiment drafts; the full decision record is U32a's N32a-01. Risks named: RISK-03, RISK-19, RISK-49.

## (d) Needs from other slices (no key known, or the edge must be added by the lead)

1. Edges the lead should add, because the seeds were already written: BD-300 (U33a) should depend on N32b-06 to
   N32b-14 and reads the decision N32b-31 (its own `needs` asks for the KILL-TUI decision bead; N32b-31 depends on
   N32b-09 and not on BD-300, so no cycle arises). BD-313 should depend on N32b-15, N32b-17 and N32b-18; BD-373 on
   N32b-16; the wrapper `scripts/e2e/worked_sessions.sh` of BD-313 calls the two scripts of N32b-15 and N32b-16.
   N24a-41 (U24a) needs N32b-13 and N32b-40 (the page), and is extended by N32b-41. BD-258 (U37) is the seed for
   §32.13; N32b-01 to N32b-05 are its mechanism beads.
2. The pilots for the dashboard and for the FrankenThreeD route (INT-FTUI-01 and INT-F3D-01 run on recorded data
   before funding): BD-300 cites `N33a-23` and `N33a-24`; N32b-06 and N32b-13 name them only in `needs`.
3. Read verbs the panes and overlays consume, owned by other chapters: `opt.status` (M4; the Races pane shows
   studies only until then), `study.status`, `eval.scorecard`, `eval.ledger`, `spark.devices`, `job.list`,
   `autopsy.show`, `nemesis.hardset`, `telemetry.summary`; telemetry channels for each overlay (contact cone
   utilization including the spin component, energy ledger, actuator state, balance, the seven instants, film
   fields, the clearance complex export, fork records, `.fmcw` edit lists, `ground.band`).
4. The surface by which the shared archive and the leaderboard are read (`opt.archive`, `eval.scorecard`): N32b-03
   supplies the view functions; the verbs' schemas belong to the Atelier and Arena units.
5. The fresh-agent harness, pinned browsers and a headless driver as test tooling pins (the conformance and
   ten-fold units).
6. The third-milestone form of `experiment.suggest` (the expert grid) for the battery's inconclusive tasks; the
   value-of-information form is BD-351 at M4.
7. A `shapes` entry in the replay packet header (see (e) item 6), to be added by the writer bead N24a-41.
8. Allocation of three refusal numbers and three counterexample numbers by the registry owners (see (e) 4, 5).

## (e) Contradictions, gaps and what was done

1. **Overlap with U32a, unresolved in the file system.** U32a wrote decision and experiment beads for the gates
   and open questions whose mechanisms lie in its subsections: KILL-MCP (N32a-05), KILL-SLOT (N32a-23),
   KILL-PREEMPT (N32a-24), KILL-MCPTASKS (N32a-29), KILL-SOLVE (N32a-32), OPEN-TETHER-2 (N32a-12), OPEN-TETHER-3
   (N32a-33), OPEN-TETHER-4 (N32a-22), OPEN-TETHER-5 (N32a-34). This unit, owning §32.21 and §32.22, had drafted
   the same nine (N32b-26, -27, -28, -29, -30, -33, -34, -35, -36) before those existed. By the brief's rule (a
   gate belongs to the unit whose subsection holds the mechanism) U32a's beads govern. Deleting this unit's nine
   was blocked by the command guard (file deletion needs the owner's approval), so they remain, each with an
   overlap notice and the other bead's key in `related`. Recommendation: load U32a's nine and drop these nine,
   folding in the items each notice lists; then remove `N32b-27` from the deps of `N32b-28` if either survives.
   Nothing else in this unit depends on the nine.
2. **Seeds cut across this unit's subsections.** BD-313 (U32a) covers `robot-docs`, the skill file, both worked
   sessions and session transcripts; BD-300 (U33a) covers the dashboard and viewer in one bead; BD-191 and N24a-41
   (U24a) cover the parent-only replay writer. This unit's beads are the detailed mechanism and test beads under
   those seeds. Names were aligned where the seeds had fixed them (`skills/frob-design-loop/SKILL.md`,
   `session.show --transcript`, `frob.session-transcript/1`); N32b-38 was rewritten to consume BD-313's
   transcript record instead of defining a second one.
3. **The written sessions omit idempotency keys** on several mutating and job-submitting commands, while §32.7
   refuses a missing key (FROB-REF-805). The scripts (N32b-15, N32b-16) supply a key on every such call and add one
   step that omits it and expects the refusal.
4. **Refusals without codes.** §32.24 states four packet refusals and §32.12.7 a resource size cap, with no codes.
   N32-REF maps two to existing codes (FROB-REF-562 for a fork below class D2; FROB-REF-724 for fork content not
   licensed in the build) and proposes three provisional numbers (836 over the byte cap, 837 frames from a chunk
   neither stored nor pinned, 838 a resource read over the cap), subject to append-only allocation.
5. **Overlapping codes across chapters**, recorded in N32-REF for the owners to assign to layers: 804 and 112
   (stale parent), 806 and 896 (cursor mismatch), 810 and 878 (chunk conflict), 704 and 761 (compute envelope).
6. **The replay packet lists no geometry.** A page cannot draw bodies from poses alone. N32b-40 requires a
   `shapes` entry (collision primitives) in the header, digest-covered and counted in the size computation; N32b-41
   restates the format with it; the parent-only writer bead must add it.
7. **Two size figures for an exported replay** (15 MB in §32.15, 8 MiB default in §32.24). Treated as an absolute
   ceiling and a default cap, as the writer bead already does.
8. **GATE-TETHER-15's restraint clause** ("at most twice the minimal number of simulations in the median task") is
   undefined as a ratio where the minimal count is zero, which is most tasks. N32b-39 adopts and registers the
   reading "median over the 30 tasks of (simulations − 2 × minimal) ≤ 0" and publishes the per-task table.
9. **FROB-REF-724 against "partial" capabilities.** Read literally, a verb that is not yet implemented in all
   eight senses would be refused, and then no gate could run through the surface to make it implemented. N32b-19
   and N32-REF adopt: refuse when no handler is in the build or the capability was killed; a partial capability
   runs and says so.
10. **GATE-TETHER-09's claim kind** is "Parity; Performance" in the chapter and "Performance" in §40; the register
    governs (N32b-25). KILL-SLOT is missing from the brief's gate list but is defined in §32.21 and §40; it has a
    row in N32b-25.
11. **Attenuation has no verb of its own** in the registry. N32b-02 makes it an option of `session.open`
    (DESIGN), keeping the registry's four session verbs.
12. **How a browser page reaches verbs** in live mode is not fixed by the plan. N32b-10 uses read verbs over the
    daemon's loopback MCP endpoint, with a fail-closed default of exported views and packets only.
13. **Not turned into work:** the native viewport on the Mac (Frontier; the plan funds no work package and names
    no pilot) is recorded in the Considerations of N32b-10 and N32b-13 only. The wrong-units seeding rule of the
    battery depends on a length field whose admissible range spans three decades; N32b-37 states the selection
    rule and the fallback if a fixture has no such field.
