---
key: "ART-L9"
title: "golden fixture L.9: the scripted agent session over MCP, run on frob-mcp in process and through frobd --mcp"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "frob-mcp", "frob-cli", "frob-conformance", "solid", "test", "e2e", "gate", "review-notes-open"]
parent: "WP-26"
deps: ["BD-023", "BD-210", "BD-298", "BD-304", "NJL-05", "NJL-07", "NJL-08", "NJL-09", "STD-E2E"]
needs: []
related: ["BD-138", "BD-140", "BD-258", "BD-295", "BD-299", "BD-313", "BD-373", "NJL-02", "NJL-06"]
gates: ["GATE-TETHER-12"]
plan_refs: ["Appendix L.9", "§32.3 (ADR-TETHER-03, -04, -05)", "§32.5", "§32.11", "§32.12.2", "§32.12.4", "§32.12.5", "§32.12.6", "§32.12.9", "§32.13", "§32.19", "§32.21 (GATE-TETHER-12)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) has two surfaces generated from one verb registry:
the command line `frob` and the MCP server `frob-mcp` (MCP: the Model Context Protocol, by which an agent
harness lists and calls tools). The server is built on `fastmcp_rust`, the MCP library of the same software suite, and is an
ordinary member of the core build graph: it links the verb implementations and **runs verbs in process**. (The
owner's ruling: a mismatch with a sibling library is fixed in the sibling, so the library is brought onto the
project's one Rust toolchain by the upstream work item FD-FMCP-01 and admitted by the gate INT-FMCP-01.) The
daemon `frobd` owns devices, kernel caches and the job queue; both `frob` and `frob-mcp` hand long work to it
as jobs, and both run CPU-tier verbs in process when no daemon is running, saying so in the envelope. The
daemon can also **host the MCP server itself** (`frobd --mcp`), so that one process serves the job queue and
the MCP transports.

The specification's ninth worked artifact is one agent session over MCP in the story of a quadruped that
carries a tray through a commercial-kitchen galley: one agent, one session, the tools of six task clusters
beyond the default. The agent asks a mission query, creates a design from an inner cell, attacks it with
adversarial worlds, shrinks and diagnoses the worst failure, edits the design, revalidates, attacks again,
qualifies the design in the blinded lane, bundles the certificates and has the bundle verified offline. This
bead turns the artifact into the **scripted MCP session**: the transcript is carried completely by the two
part beads (NJL-08: orient, mission, author, which is exact; NJL-09: attack to offline verification, which is
simulated), and this bead runs both halves as one session in both hosting modes and asserts what only the
whole session can show.

Decisions behind the two modes (DESIGN, with reasons). The server is a protocol adapter with one dispatch
trait and two back ends, in-process handlers (the default) and a local socket to the daemon; a killed adapter
must never own a job, which is why long work lives in the daemon's ledger. Long tools take a bounded
synchronous wait and otherwise return a job handle, because most studio verbs finish inside the wait and an
avoided poll saves an agent turn. Wire cancellation of a waiting call ends the wait, not the job; `job.cancel`
cancels the job. A session test that runs in one mode only would not notice a result that depends on which
process hosts the server; running both and comparing is the check.

## Scope

In: the session as one scripted run (28 steps, the last an offline command); the three hosting lanes; the
comparison of lanes; the session-wide rules (focus, authority, budget, flags, imported text); the clauses of
the MCP gate that a session can measure; the script and its log. Out: the per-step expectations (NJL-08,
NJL-09); the driver (NJL-08); the MCP server, the daemon and the job protocol (BD-023, BD-298, BD-210); the
fresh-agent tests of the surface, in which a model rather than a script chooses the calls (BD-313, BD-373).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L9/`: `transcript.a.specimen.txt`, `transcript.b.specimen.txt`
(owned by the part beads); `session.steps.json` (the concatenation of the two step files: 28 steps);
`session.expect.json` (the session-wide expectations of this bead).

**The session in one table** (step identifiers are those of the part beads; the focus column is the task
cluster listed while the step runs).

| Focus | Steps | What happens |
|---|---|---|
| `orient` (default) | A1, A2 | List the tools; show the session: namespace, scopes, budget |
| `mission` | A3 to A10 | Create and check the mission query; estimate; synthesize (one inner, three unresolved, eight excluded cells; gap 1/24 m); read the outer set; refine (the minimum 1/8 m is attained); refute one design |
| `author` | A11 to A14 | Create the design at the centre of the inner cell; check it; synthesize the baseline controller |
| `attack` | B1 to B4 | Search for adversarial worlds (a job handle, then the result); shrink the worst failure to a minimal counterexample world |
| `simulate` | B5 | Classify the three proposed changes at a fork before the failure |
| `diagnose` | B6, B7 | Run interventions, placebo forks and pairs; read the attribution |
| `author` | B8 | Edit the foot radius against the expected parent |
| `mission` | B9 | Revalidate: what the edit left standing, by read set |
| `attack` | B10 | Search again: none found, which is not a certificate |
| `qualify` | B11 to B13 | Dry run and committed request in the blinded lane; bundle the certificates and the receipt |
| (offline) | B14 | `frob-verify` on a machine with neither the agent, a GPU nor the simulator |

The session makes nine `session_focus` calls and visits seven task clusters (the default and six others).

**Three lanes.**
1. *In process, no daemon* (`frob-mcp` over stdio; the token arrives in the environment). Only the CPU-tier
   steps can run: A1 to A14, B8, B9, the dry run B11 and the bundle steps on the mission answer. Each envelope
   states that the verb ran in process. A step that needs a device or the job queue is refused with the
   backend-unavailable class (exit 6) or answers with the not-implemented refusal; it is never run silently on
   another tier.
2. *In process, with the daemon* (`frob-mcp` over stdio, `frobd` running). All steps run. Short verbs run in
   the server's process; long work is submitted to the daemon as jobs under the step's idempotency key.
3. *Hosted by the daemon* (`frobd --mcp`, streamable HTTP bound to 127.0.0.1; a bearer token in the
   `Authorization` header maps to the session's capability token). All steps run in one process.

**Each lane in its own root** (DESIGN). Every lane runs in its own scratch root: its own daemon state root,
job ledger, namespace and test campaign. Reason: idempotency keys are scoped to (principal, verb), so lanes
sharing one ledger under the specimen's keys would hand lane 3 the stored receipts of lane 2
(`idempotent_replay: true`) and the comparison would compare a receipt with itself; and a second committed
qualification request in one campaign would be request 2 at level 0.05/6, not the specimen's request 1. The
test campaign of each root is opened and closed with the admin actions of `eval.ledger` as NJL-09 specifies
(on the synthetic claim family while OPEN-ARENA-3 is undecided).

**Comparison of lanes** (`session.expect.json`). For every step that ran in two lanes, the envelopes agree on
`verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and `next_actions`, after removing
session, job and campaign identifiers, wall times and cost. For the exact half this is byte equality of the
canonical `result`. For the simulated half it is equality as well, because both lanes submit the same request
under the same seed protocol to the same daemon build at determinism class D2: a world's digest depends on its
address and identities, never on which process asked. Exception: steps B12 and B13 consume each lane's own
sealed clusters, so they are compared on their exact fields (schema, `index: 1`, `alpha_i`, `alpha_spent`,
`units_used`, the sealed-field list, the flags, the evidence class) and not on the decision, the receipt hash or
the bundle root; B14 is compared on its verdict, entry count and class.

**Session-wide rules this bead asserts** (rules 1 to 3 are also asserted per step by NJL-08, rules 4 to 11 by
NJL-09; here they are checked across the whole session).

| Rule | Session-wide check |
|---|---|
| 1. Seven tools at a time | After each of the nine focus calls, `tools/list` holds at most seven task tools and the three base tools; `session_show` reports identical scopes after every focus call |
| 2. Inner and outer sets are different answers | The design of A12 lies in an inner cell of A9; no later step treats an unresolved cell as a candidate |
| 3. A minimum needs a matching lower bound | `attained` is true only in A9 |
| 4. A long tool returns a handle | Every `run` tool returns within its `sync_wait_ms` plus 500 ms with a result or a handle; a handle carries progress with a denominator; after a killed connection `job_find` with the step's key returns the job, with one charge |
| 5. A certified static answer does not make the robot walk | The envelopes of the two synthesis answers (A7, A9) carry `claims: ["Proof"]`; envelopes of search, shrink, attribution and study steps never do |
| 6. A counterexample is shrunk before it is argued about | The fork of B5 is made on the world of B4, not on the unshrunk world of B3 |
| 7. Attribution is interventional and labelled | The attribution is class Estimated; a mid-episode structural intervention is refused with `FROB-REF-581` |
| 8. An edit puts evidence in question, by read set | The mission answer of A9 is inherited in B9; the dynamic results are not |
| 9. "None found" is not a certificate | A search with zero promoted failures carries the statement and no Proof claim |
| 10. Qualification spends error budget | The committed request advances the campaign index by one and spends α/2; the dry run spends nothing; sealed fields are unreadable (`FROB-REF-840`) |
| 11. The weakest input sets the class | The bundle is Tested; every exporting step's envelope carries the three no-authorization flags, all false |

Further session-wide checks: the budget identity summed over the session (budget at A2 minus the sum of all
step costs equals the budget after B13, for GPU-seconds and CPU core-seconds, as exact decimals); every
`mutate` and job-submitting `run` step carries an idempotency key, and replaying the whole step file under the
same keys returns every stored answer with `idempotent_replay: true` and charges nothing; no tool description,
prompt, server instruction or `next_actions.command` contains text that originated in an import (a design
whose name is an instruction-like string is created before the session, and the recorded session is searched
for that string).

**Gate clauses measured.** GATE-TETHER-12 (TARGET; claim Known answer; M3): every `run` tool returns within
its `sync_wait_ms` plus 500 ms with a result or a handle; a wire-cancelled wait leaves the job running; after
`session_focus` the list holds at most seven tools plus the base three; each resource kind is listed and
readable; prompts match `robot-docs`. This session measures the first three clauses on the tools it calls and
reads the task-cluster directory resource once; the remaining clauses are measured by the MCP surface bead.

## Acceptance criteria

- [ ] The 28 steps (27 over MCP and the offline command) run as one session in lane 2 and in lane 3, and every per-step
      assertion of the two part beads holds in both.
- [ ] Lane 1 runs the CPU-tier steps with envelopes that say the verb ran in process, and refuses the others
      with a typed refusal; no step is silently run on another tier.
- [ ] Lanes 2 and 3 agree step by step under the comparison rule; the exact half also agrees with lane 1.
- [ ] The eleven session-wide rules of the table hold in lanes 2 and 3.
- [ ] GATE-TETHER-12, the three session clauses: the largest overrun of any `run` call beyond its wait is at
      most 500 ms; a wire-cancelled `job_wait` leaves the search job running (the job later completes and is
      found by its key); after each focus call at most ten tools are listed. Measured values are logged against
      the thresholds.
- [ ] Killing `frob-mcp` (lane 2) or the client connection (lane 3) during the wait of B3 leaves the daemon's
      ledger consistent and the job discoverable by the key `nem-1`, with one budget charge.
- [ ] The budget identity over the whole session holds for both resources; the full replay under the same keys
      charges nothing.
- [ ] The poisoned-name control: the instruction-like string appears in no tool description, prompt, server
      instruction or `next_actions.command` of the recorded session.
- [ ] `frob-verify` accepts the bundle in an empty environment: six of six entries, class Tested, flags false.
- [ ] Skips (a device absent, a search that finds no failure) are logged as skips with reasons and are never
      counted as passes; in small mode the log says that the gate is not claimable.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test golden_l9_session` (the comparison and accounting code on recorded
fixture sessions labelled `synthetic`; the real server over the in-memory transport for the listing checks):
- `session_steps_concatenate`: 14 + 14 steps; identifiers unique; every capture is used or exported.
- `focus_calls_nine_clusters_seven`: the step file has nine `session_focus` steps over seven distinct clusters,
  the default included.
- `lane_compare_ignores_only_listed_fields`: a planted difference in `result` fails; a difference in cost,
  wall time or job identifier does not.
- `session_budget_identity`: on the specimen's numbers, 1,800 − (212 + 61 + 240) = 1,287 GPU-seconds.
- `claims_by_step`: the synthesis answers A7 and A9 carry Proof; search, shrink, attribution and study steps do
  not; the receipt carries Design comparison.
- `lanes_use_separate_roots`: the lane plan gives three distinct state roots; a planted plan that runs lane 3 on
  lane 2's root is rejected before any request (it would compare stored receipts with themselves).
- `flags_on_exporting_steps`: every step whose verb exports an artifact has the three flags, all false; a
  recorded envelope with a missing flag fails.
- `no_imported_text`: a recorded session containing the planted string in a `next_actions.command` fails.
- `listing_never_exceeds_ten`: over the in-memory transport, after each focus of the step file.

### End-to-end script and logging

`scripts/e2e/art_l9_mcp_session.sh` (header: `bead=ART-L9 milestone=M3 labels=golden,appendix-l,mcp,tether
gates=GATE-TETHER-12 seed=901 needs=cpu,cuda,mcp,frobd plants=lane-diverges,wait-overrun,focus-grants-scope`).
Steps: (1) set up one scratch root per lane, each with the fixtures of artifacts L.1, L.3 and L.4, a test
campaign opened under an administrative session (`frob eval ledger --open --task t:kitchen-tray@88c1…
--world-family <synthetic family> --key l9-open --json`, as NJL-09 does) and the poisoned-name design; (2) lane 1: start `frob-mcp` over stdio with no daemon, run `frob-mcp-drive` on the
CPU-tier subset, assert the in-process statement and the typed refusals; (3) lane 2: start `frobd`, run the full
step file over stdio; (4) lane 3: start `frobd --mcp` on a loopback port, run the full step file over HTTP with
the bearer token; (5) run the part beads' assertions on the recordings of lanes 2 and 3; (6) compare the lanes
step by step (`e2e_assert_digest_eq` on canonical results); (7) the session-wide rules, one read of the resource
`frob://docs/clusters` compared with the focused lists, and the budget identity (`e2e_assert_num`, exact); (8)
the wire-cancel and kill checks of rule 4, each followed by `job_find`; (9) the full replay under the same keys;
(10) the search for the poisoned string; (11) the offline `frob-verify`; (12) close each lane's test campaign
(`frob eval ledger <campaign> --close --key l9-close --json`) after the sealed-read refusals; (13) `frob-golden
check --artifact L9`.
Plants: `lane-diverges` (the hosted server patched to alter one result) must fail step (6); `wait-overrun` (a
tool that sleeps 2 s past its wait) must fail the gate metric; `focus-grants-scope` (a server that adds a scope
on focus) must fail rule 1. The log (`target/e2e-logs/art_l9_mcp_session/<utc>.ndjson`, with its transcript
beside it) has one `cmd` line per MCP request per lane with the request verbatim, wall time, the envelope's exit
code and `evidence` block (claims, class, tier, determinism, identities, seed); metrics `wait_overrun_ms` (<=
500), `tools_listed` (<= 10), `lanes_differing_steps` (== 0), `session_gpu_s_spent` and
`session_cpu_core_s_spent` with the budget identity as their check, each tagged `--gate GATE-TETHER-12` where it
is a gate clause; `assert` lines with observed and expected values and, on failure, the first differing step,
pointer and the paths of both recordings; refusals with code and first repair; skips with reasons; and the
`summary` with the gate touched. Sealed fields and private-scene data never enter the log.

### Known answers and negative controls

- Known answers: those of the two part beads (the mission answers; the binomial bounds 0.0432 and 0.0504 at
  the first level and 0.0425 and 0.0505 at the second; the level sequence); nine focus calls and seven
  clusters; the specimen's session spend of 513 GPU-seconds.
- KA-763 (one job and one charge per key; a retry returns the stored receipt) is the full-replay check;
  KA-755 (the exit-code table) is exercised by the typed refusals of lane 1.
- Negative controls: the three plants; the poisoned name; the killed adapter; a step file replayed with one
  changed argument under an old key (refused, `FROB-REF-803`, exit 7).
- Invariants carried: INV-TETHER-05 (one job and one budget charge per principal, verb and key, under any
  interleaving and any crash point); INV-TETHER-08 (a world's digest and results depend on its address and
  identities, never on its slot, co-tenants or dispatch order), as the lane comparison; INV-TETHER-10 (killing
  `frob`, `frob-mcp` or the dashboard at any instant leaves the daemon's ledger consistent and every admitted
  job discoverable by its key); INV-TETHER-11 (a cancelled wait is not a cancelled job unless the call asked
  for that); INV-TETHER-13 (no tool description, prompt, server instruction or `next_actions.command` contains
  text that originated in an import).
- Determinism lanes: lane 2 twice on one machine; lane 2 against lane 3; the exact half on Linux and macOS and
  in debug and release.

## Considerations

- Kill gate KILL-MCP (the admission gate of the MCP server cannot pass within the M0 budget after the two
  upstream changes were attempted). Fallback 1: `frob-mcp` as its own workspace talking to `frobd` over a
  local socket; lanes 2 and 3 then both use the socket back end and lane 1 disappears. Fallback 2: agents use
  the CLI; the session is then run as its CLI twin and the MCP lanes are recorded as not run.
- The specimen's introduction says the agent changes focus eight times; the transcript has nine `session_focus`
  calls. The step file follows the transcript. Recorded in the unit's coverage file.
- The specimen's token has six scopes, fewer than a per-cluster scope rule would require for the tools called;
  the script opens the session with the union computed from the registry and logs the difference (metric
  `scopes_beyond_specimen`, as NJL-08 does). The capability bead BD-258 has decided the question: scopes are
  named by the verb's own cluster, and only `<cluster>:run` and `<cluster>:write` imply `<cluster>:read`; no
  scope implies another cluster's, so `design:write` does not cover `control_synth`, and the union is the
  registry's.
- OPEN-TETHER-2 (owner: Tether lead; blocks GATE-TETHER-12): whether agent harnesses act on catalog-changed
  notifications. The script re-lists after every focus and also checks the tag filter, so it passes under the
  fail-closed default (static clusters by launch flag plus the compatibility tool).
- OPEN-TETHER-3 (owner: Tether lead): the default and maximum of `sync_wait_ms`; fail-closed default 5,000 and
  30,000, which the step file uses.
- Wire cancellation is only partly qualified in the MCP library on some transports; the handler therefore
  checks for cancellation at every job frame, and the drain that protects device state is done by the
  daemon's job engine, never by the library's process-exit drain.
- Cost: the full session is about 500 GPU-seconds and 900 CPU core-seconds per lane in the story; the gate
  lane runs lanes 2 and 3 once each.

## References

Appendix L.9; §32.3 (ADR-TETHER-03, -04, -05), §32.5, §32.11, §32.12.2, §32.12.4 to §32.12.6, §32.12.9,
§32.13, §32.19, §32.21; GATE-TETHER-12; INV-TETHER-05, -08, -10, -11, -13; KA-755, KA-763; KILL-MCP;
OPEN-TETHER-2, OPEN-TETHER-3.
