---
key: "NJL-08"
title: "golden fixture L.9, part A: the MCP session driver and the exact half of the transcript (orient, mission, author)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "augur", "frob-mcp", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-06"
deps: ["BD-138", "BD-126", "BD-135", "BD-118", "BD-110", "NJL-02", "NJL-05", "NJL-07", "ART-L1", "ART-L4", "STD-E2E"]
needs: ["§14, §32.6: the verbs `design.new` (from a file, a template or a mission cell) and `design.show` (drafted by unit U14a as N14a-21)"]
related: ["N14a-21", "BD-023", "BD-140", "BD-298", "BD-313", "BD-258", "NJL-06", "NJL-09", "ART-L9"]
gates: []
plan_refs: ["Appendix L.9 (orient, mission, author; rules 1 to 3)", "§32.4", "§32.12.3", "§32.12.4", "§32.12.5", "§32.12.9", "§32.16", "§25.3.3"]
---
## Background

FrankenRobotics (a robotics simulation and design system) serves its verbs to agents as MCP tools (MCP: the
Model Context Protocol) through the server `frob-mcp`, which is generated from the verb registry, links the
verb implementations and runs them in process; the daemon can host the same server (`frobd --mcp`). A tool
call is `tools/call` with the tool name and its arguments; the response's `structuredContent` is the envelope
`frob.envelope/1` that the command line prints for the same verb.

The specification ends with nine worked artifacts for one story (a quadruped carries a tray through a
commercial-kitchen galley and sets it down at a pass counter). The ninth is a whole agent session over MCP:
one agent, one session, the tools of six task clusters beyond the default. This bead carries the first half
of that transcript, which is **exact**: every number in it follows from the stance mission of artifact L.4 by
rational arithmetic, so a correct system must reproduce it value for value. The second half (adversarial
search, autopsy, qualification) is carried by bead NJL-09, and the full scripted session with its rules by
bead ART-L9. This bead also builds the tool that drives a scripted MCP session, which the other two reuse.

The transcript abridges each response to the fields that matter at that step. The session's capability token
arrives from the environment (stdio transport). Tool names are those of the task clusters: at most seven
tools are listed at a time, plus the three base tools `session_focus`, `job_wait` and `job_cancel`, and the
agent changes cluster with `session_focus`.

Why a scripted session as a fixture (DESIGN). Each verb has its own tests; what no verb's test covers is the
sequence an agent actually performs, with identities flowing from one answer into the next call, focus
changing in between, and the budget decreasing. A scripted session with exact expectations is the regression
test for that sequence on the real server. Rejected: a recorded session replayed against a stub (the stub
would be the thing tested); testing the sequence on the CLI only (the MCP surface has its own failure modes:
tool visibility, structured content, bounded waits).

## Scope

In: the driver `frob-mcp-drive` and the step-file format; the transcript of the orient, mission and author
sections; the expectation of each step with its field classes; the rules the first half demonstrates; the
script that runs it in process. Out: the second half of the transcript (NJL-09); running both halves in both
hosting modes with the session-wide rules (ART-L9); the MCP server, task clusters and the bounded wait (BD-138,
BD-298, NJL-05); the mission and design mechanisms (their beads).

## Technical approach

**Driver.** `crates/frob-conformance/src/bin/frob-mcp-drive.rs`, using the MCP client module of the verb
conformance harness. `frob-mcp-drive --steps <file> --transport stdio|http --server <command or url> --out
<dir>` performs the steps in order, substitutes captured values into later arguments, and writes one NDJSON
record per request and response (`request`, `structuredContent`, `isError`, wall time, JSON-RPC error if
any). It asserts nothing itself; assertions are made by the script with the shared helper on the recorded
responses, so that the log shows observed and expected values.

**Step file** (`fixtures/golden/appendix-l/L9/session.a.steps.json`; DESIGN):

```json
{ "id": "A7", "tool": "mission_synthesize",
  "arguments": { "mission": "${mission_id}", "idempotency_key": "syn-1",
                 "budget": { "cpu_core_s": 60, "checker_s": 10 }, "sync_wait_ms": 2000 },
  "capture": { "answer_1": "/result/answer", "bundle_1": "/result/certificate/bundle" },
  "expect_exit": [0, 2], "expect": "expect/A7.json" }
```

`${name}` refers to a value captured by an earlier step. A step may be `tools/list` (`"tool": null`,
`"method": "tools/list"`). Expectation files list assertions by JSON pointer, each with a field class of the
golden-fixture layout: `exact`, `identity`, `structural` or `illustrative`.

**The transcript, first half** (`fixtures/golden/appendix-l/L9/transcript.a.specimen.txt`; `→` is a request,
`←` a response).

```text
── orient ──────────────────────────────────────────────────────────────────────────────────────────────
→ tools/list
← session_focus, job_wait, job_cancel                                  (base tools, always listed)
  capabilities, doctor, robot_docs, schema, design_templates, scene_list, session_show     (cluster orient)

→ session_show {}
← { "ok": true, "exit": 0,
    "result": { "session": "s:2f6a…", "namespace": "agent-17", "focus": "orient",
                "scopes": ["design:write", "mission:run", "nemesis:run", "autopsy:run", "sim:run", "qual:run"],
                "budget": { "gpu_s_left": 1800, "cpu_core_s_left": 7200 } } }

── mission ─────────────────────────────────────────────────────────────────────────────────────────────
→ session_focus { "cluster": "mission" }
← { "ok": true, "exit": 0,
    "result": { "focus": "mission",
                "tools": ["mission_new", "mission_check", "mission_synthesize", "mission_refute",
                          "mission_distinguish", "mission_show", "result_revalidate"] } }

→ mission_new { "document": "…the .fmission of L.4…", "idempotency_key": "mis-1" }
← { "verb": "mission.new", "ok": true, "exit": 0, "result": { "mission_id": "q:3c9d…" },
    "bound": { "family": "template:quad-mammal@1#b3:8e02…" } }

→ mission_check { "mission": "q:3c9d…" }
← { "verb": "mission.check", "ok": true, "exit": 0,
    "result": { "domain": "nonempty", "topologies": 1, "free": ["v", "r"],
                "quantifiers": "∃ policy per nominal design, ∀ worlds, ∀ executions",
                "fragment": "linear static: exact answer available", "horizon": "static" } }

→ mission_synthesize { "mission": "q:3c9d…", "estimate_only": true }
← { "ok": true, "exit": 0,
    "result": { "estimate": { "admission_s": 0.01, "lowering_s": 0.01, "proposing_s": 0.02, "checker_s": 0.02,
                              "cpu_core_s": 0.06, "interval_90": [0.04, 0.2], "gpu_s": 0 } } }

→ mission_synthesize { "mission": "q:3c9d…", "idempotency_key": "syn-1",
                       "budget": { "cpu_core_s": 60, "checker_s": 10 }, "sync_wait_ms": 2000 }
← { "schema": "frob.envelope/1", "verb": "mission.synthesize", "ok": true, "exit": 0,
    "result": {
      "mission_id": "q:3c9d…", "answer": "a:0e51…",
      "inner":      [ { "cell": "c:03", "topology": "rect4", "chart": "v, r", "box": [["4", "6"], ["2", "5/2"]],
                        "closed": [[true, true], [true, true]],
                        "policy": "selectable-per-nominal-design", "proofs": ["p0"] } ],
      "excluded":   [ { "cell": "c:01", "box": [["4", "6"], ["1", "3/2"]], "proofs": ["p1"],
                        "binding": "knee-hold at foot.pp under P = (16, 12) N*m" },
                      { "cell": "c:04", "box": [["6", "8"], ["1", "3/2"]],
                        "closed": [[false, true], [true, true]], "proofs": ["p1"],
                        "binding": "knee-hold at foot.pp under P = (16, 12) N*m" },
                      { "cell": "c:07", "box": [["8", "10"], ["1", "3/2"]],
                        "closed": [[false, true], [true, true]], "proofs": ["p2"],
                        "binding": "rating at foot.pp under P = (16, 12) N*m" }
                      /* c:08 to c:12 likewise, each excluded by proof p2 */ ],
      "unresolved": [ { "cell": "c:02", "box": [["4", "6"], ["3/2", "2"]], "explanation": "straddles-threshold" },
                      { "cell": "c:05", "box": [["6", "8"], ["3/2", "2"]], "explanation": "straddles-threshold" },
                      { "cell": "c:06", "box": [["6", "8"], ["2", "5/2"]], "explanation": "straddles-threshold" } ],
      "witnesses":  [ { "proof": "p0", "kind": "lp-primal", "vertex": ["16", "12"],
                        "forces_N": ["76", "44", "40", "0"], "at": { "v": "6", "r": "2" } } ],
      "cost_bounds": { "incumbent": "1/6", "lower_bound": "1/8", "unit": "m", "attained": false, "gap": "1/24" },
      "next_test": { "kind": "split-cell", "cell": "c:06", "at": { "r": "11/5" },
                     "why": "the lower bound on half-width is reached only inside c:06" },
      "certificate": { "bundle": "b3:61f0…", "kinds": ["lp-primal", "lp-farkas", "partition"],
                       "checker": "frob-verify/1" },
      "assumptions": ["independent-normal-force authority", "payload wrench observed before allocation",
                      "point feet, coplanar, static", "cap, lam and tau_c are stipulated by the query"] },
    "evidence": { "class": "Certified", "tier": "static", "determinism": "exact",
                  "claims": ["Proof"], "reality": "no-claim" },
    "cost": { "wall_ms": 41, "cpu_core_s": 0.05, "gpu_s": 0 },
    "budget": { "gpu_s_left": 1800, "cpu_core_s_left": 7199.95 },
    "warnings": [ "objective: claimed minimum not established; reported as feasible-candidate with gap 1/24 m" ],
    "authorization": { "execution_authorized": false, "hardware_safety_signoff": false,
                       "sim_to_real_certified": false },
    "next_actions": [ { "command": "frob mission synthesize q:3c9d… --strategy refine --from a:0e51…",
                        "why": "Splitting c:06 at r = 11/5 settles the minimum half-width." } ] }

→ mission_show { "mission": "q:3c9d…", "answer": "a:0e51…", "set": "outer" }
← { "verb": "mission.show", "ok": true, "exit": 0,
    "result": { "set": "outer", "definition": "domain minus excluded cells",
                "cells": ["c:02", "c:03", "c:05", "c:06"], "measure": { "area_fraction": "1/3" },
                "inner_within": ["c:03"], "unresolved_within": ["c:02", "c:05", "c:06"] } }

→ mission_synthesize { "mission": "q:3c9d…", "strategy": "refine", "from": "a:0e51…",
                       "idempotency_key": "syn-2", "sync_wait_ms": 2000 }
← { "verb": "mission.synthesize", "ok": true, "exit": 2,
    "result": {
      "answer": "a:8b27…", "refines": "a:0e51…",
      "inner":      [ { "cell": "c:03",  "box": [["4", "6"], ["2", "5/2"]], "proofs": ["p0"] },
                      { "cell": "c:06a", "box": [["6", "8"], ["11/5", "5/2"]],
                        "closed": [[false, true], [true, true]], "proofs": ["p4"] } ],
      "unresolved": [ { "cell": "c:02" }, { "cell": "c:05" },
                      { "cell": "c:06b", "box": [["6", "8"], ["2", "11/5"]],
                        "closed": [[false, true], [true, false]], "explanation": "straddles-threshold" } ],
      "witnesses":  [ { "proof": "p4", "kind": "lp-primal", "vertex": ["16", "12"],
                        "forces_N": ["88", "32", "40", "0"], "at": { "v": "8", "r": "11/5" } } ],
      "cost_bounds": { "incumbent": "1/8", "lower_bound": "1/8", "unit": "m", "attained": true, "gap": "0" },
      "next_test": null,
      "certificate": { "bundle": "b3:d4a9…", "kinds": ["lp-primal", "lp-farkas", "partition"],
                       "checker": "frob-verify/1" } },
    "evidence": { "class": "Certified", "tier": "static", "determinism": "exact",
                  "claims": ["Proof"], "reality": "no-claim" },
    "warnings": [ "boundary design has zero reserve at foot.pp (rating, knee-hold) and at foot.mm (unilateral)" ],
    "next_actions": [ { "command": "frob design new quad-tray --from-cell q:3c9d…/c:06a",
                        "why": "The centre of c:06a keeps 6 N of force reserve and 1.2 N·m of knee reserve." } ] }

→ mission_refute { "mission": "q:3c9d…", "design": { "v": "9", "r": "5/2" } }
← { "verb": "mission.refute", "ok": true, "exit": 0,
    "result": { "verdict": "excluded for every allocation", "proof": "p2", "kind": "lp-farkas",
                "binding": "rating at foot.pp: 94 N required, 88 N rated",
                "smallest_relaxing_change": [ { "change": "cap", "to": "94 N" },
                                              { "change": "P_y envelope", "to": "32/3 N*m" } ] } }

── author ──────────────────────────────────────────────────────────────────────────────────────────────
→ session_focus { "cluster": "author" }
→ design_new { "name": "quad-tray", "from_cell": "q:3c9d…/c:06a", "idempotency_key": "des-1" }
← { "verb": "design.new", "ok": true, "exit": 0,
    "result": { "design": "d:7a3e…", "at": { "v": "7", "r": "47/20" }, "half_width": "1/7 m",
                "symmetry": { "mech": "C2xC2", "obs": "C2" } } }                         (the .fbot of L.1)

→ design_check { "design": "quad-tray" }
← { "verb": "design.check", "ok": true, "exit": 0,
    "result": { "closure": "closed", "symmetry": "C2xC2", "findings": [] },
    "evidence": { "class": "Tested", "tier": "static", "model_id": "m:5d20…", "determinism": "exact",
                  "claims": [], "reality": "no-claim" }, "bound": { "design": "d:7a3e…" } }

→ control_synth { "design": "quad-tray", "idempotency_key": "ctl-1" }
← { "verb": "control.synth", "ok": true, "exit": 0,
    "result": { "controller": "c:2be8…", "gait": "trot from the atlas", "standing": "baseline" } }
```

**Steps and expectations.**

| Step | Call | Class | What is asserted |
|---|---|---|---|
| A1 | `tools/list` | exact | Exactly the three base tools and the seven tools of `orient`: `capabilities`, `doctor`, `robot_docs`, `schema`, `design_templates`, `scene_list`, `session_show` |
| A2 | `session_show` | structural | A session identifier, a namespace, `focus: "orient"`, a scope list, and a budget with GPU-seconds and CPU core-seconds left; exit 0 |
| A3 | `session_focus {cluster: "mission"}` | exact | `focus: "mission"`; the seven tools `mission_new`, `mission_check`, `mission_synthesize`, `mission_refute`, `mission_distinguish`, `mission_show`, `result_revalidate`; a following `tools/list` holds these plus the base three |
| A4 | `mission_new` with the query of artifact L.4, key `mis-1` | identity | `result.mission_id` equals the identity of the query; `bound.family` names the template with its content hash |
| A5 | `mission_check` | exact | domain `nonempty`; topologies 1; free `["v", "r"]`; quantifiers "∃ policy per nominal design, ∀ worlds, ∀ executions"; fragment "linear static: exact answer available"; horizon `static` |
| A6 | `mission_synthesize {estimate_only: true}` | structural | An `estimate` with admission, lowering, proposing and checker seconds, a 90% interval and `gpu_s: 0`; nothing is charged (the budget of A2 is unchanged) |
| A7 | `mission_synthesize`, key `syn-1`, `sync_wait_ms` 2,000 | exact | The first answer: inner c:03 = [4, 6] × [2, 5/2]; excluded c:01, c:04 (binding knee-hold at foot.pp), c:07 to c:12 (binding rating at foot.pp); unresolved c:02, c:05, c:06; witness forces (76, 44, 40, 0) N at v = 6, r = 2; cost bounds 1/6, 1/8, gap 1/24, not attained; next test "split c:06 at r = 11/5"; certificate kinds `lp-primal`, `lp-farkas`, `partition`; evidence class Certified, tier `static`, determinism `exact`, claims [Proof], `reality: no-claim`; the warning that the minimum is not established; the three flags false; a `next_actions` command that is a registered CLI path. Cost and budget are structural: budget left = budget before − cost |
| A8 | `mission_show {set: "outer"}` | exact | Cells c:02, c:03, c:05, c:06; area fraction 1/3; inner within c:03; unresolved within c:02, c:05, c:06 |
| A9 | `mission_synthesize {strategy: "refine", from: <answer 1>}`, key `syn-2` | exact | Inner c:03 and c:06a = (6, 8] × [11/5, 5/2]; unresolved c:02, c:05, c:06b; witness (88, 32, 40, 0) N at v = 8, r = 11/5; cost bounds 1/8 and 1/8, gap 0, attained; `next_test: null`; the zero-reserve warning; a `next_actions` command that creates a design from cell c:06a |
| A10 | `mission_refute {design: {v: "9", r: "5/2"}}` | exact | Excluded for every allocation; `lp-farkas`; "rating at foot.pp: 94 N required, 88 N rated"; relaxing changes: rating to 94 N, P_y envelope to 32/3 N·m |
| A11 | `session_focus {cluster: "author"}` | exact | The seven tools of `author` |
| A12 | `design_new {name: "quad-tray", from_cell: <mission>/c:06a}`, key `des-1` | exact, identity | `at: {v: "7", r: "47/20"}`; `half_width: "1/7 m"`; symmetry mech `C2xC2`, obs `C2`; the design identity |
| A13 | `design_check` | exact | `closure: "closed"`; `symmetry: "C2xC2"`; no finding of severity high; evidence class Tested, tier `static`, determinism `exact`; `bound.design` is the identity of A12 |
| A14 | `control_synth`, key `ctl-1` | structural | A controller identity, a gait and a standing (`baseline`); one of the three synthesis outcomes, never a bare failure |

**Rules the first half demonstrates** (asserted, not illustrated).
1. *Seven tools at a time.* After every `session_focus` the list holds at most seven task tools and three base
   tools. Focus changes what is listed, not what is permitted: the token's scopes are fixed at the first call.
2. *Inner and outer sets are different answers.* The inner set holds the cells with a constructive witness;
   the outer set is the domain minus what was excluded. The agent builds from an inner cell and treats the
   three unresolved cells as work, not as candidates.
3. *A minimum needs a matching lower bound.* The first answer is a feasible candidate with a gap of 1/24 m.
   Only the refined answer, in which incumbent and lower bound meet at 1/8 m, states a minimum, and it warns
   that the boundary design has no reserve. The agent takes the cell centre, which keeps 6 N of force reserve
   and 1.2 N·m of knee reserve.

**Envelope facts used by the assertions.** Exit codes: 0 completed; 2 completed with findings or flags; 8
accepted and still running (not a failure). A `run` tool takes `sync_wait_ms` (0 to 30,000; MCP default 5,000)
and an `idempotency_key`; repeating a call with the same key and content returns the stored answer with
`idempotent_replay: true`. Rationals travel as strings (`"11/5"`). The `bound` block echoes the immutable
identities that names resolved to.

## Acceptance criteria

- [ ] `frob-mcp-drive` runs a step file over stdio and over the in-memory transport, substitutes captures, and
      records one line per request and response; a step whose capture pointer is missing fails the run.
- [ ] `transcript.a.specimen.txt` equals the text above byte for byte (manifest SHA-256).
- [ ] The fourteen steps run in order against `frob-mcp` in process and every `exact` and `identity` assertion
      of the table holds; every `structural` assertion holds; nothing `illustrative` is asserted.
- [ ] Rule 1: after each of the two focus changes the listed set is exactly the cluster plus the base three,
      and `session_show` reports the same scopes before and after.
- [ ] Rule 2: the outer set of A8 contains the inner cell and the three unresolved cells and is never returned
      with witnesses; rule 3: `attained` is false in A7 and true in A9, with the gap 1/24 then 0.
- [ ] Repeating A7 with the key `syn-1` returns the same answer with `idempotent_replay: true` and no second
      charge; repeating it with the key and a changed budget is refused (`FROB-REF-803`, exit 7).
- [ ] A6 charges nothing: the budget after A6 equals the budget after A2.
- [ ] Each `run` call returns within its `sync_wait_ms` plus 500 ms with a result or a job handle.
- [ ] Each step's CLI twin (the same verb through `frob … --json`) returns an envelope that agrees with the
      MCP envelope on `verb`, `ok`, `exit`, `result`, `evidence`, `bound`, `warnings` and `next_actions`.
- [ ] The recorded responses of two runs are equal after removing time, cost and session identifiers.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test mcp_drive` (the real server over the in-memory transport; no mocks):
- `drive_substitutes_captures`: `${mission_id}` in step 2 is the value captured in step 1.
- `drive_missing_capture_fails`; `drive_records_error_for_unknown_tool` (a JSON-RPC error, code −32602).
- `drive_tools_list_step`: a `tools/list` step records the tool names in order.
- `step_file_schema`: the fixture step file validates; a step with an unknown key is refused.
- `expect_classes`: an `exact` mismatch fails; a `structural` value of the right shape passes; an
  `illustrative` pointer is never compared.
- `a7_expectation_matches_l4_fixture`: the expectation file of A7 equals the first-answer expectation of
  artifact L.4, and that of A9 the refined answer (one source of truth for the numbers).

### End-to-end script and logging

`scripts/e2e/art_l9_session_a.sh` (header: `bead=NJL-08 milestone=M1 labels=golden,appendix-l,mcp,
verb:mission.new,verb:mission.check,verb:mission.synthesize,verb:mission.show,verb:mission.refute,
verb:design.new,verb:design.check,verb:control.synth,verb:session.show,verb:session.focus seed=1 needs=cpu,mcp
plants=eighth-tool,gap-attained`). It opens a session whose scopes are the union the registry requires for
the tools of the step file, starts `frob-mcp` over stdio with the token in the environment, runs
`frob-mcp-drive` on `session.a.steps.json`, and then asserts each row of the table on the recorded responses
(`e2e_assert_json`, `e2e_assert_rational` for every bound and force, `e2e_assert_eq` on tool lists). It then
runs the idempotent-replay and conflict checks, the wait-bound check and the CLI twins. Plants:
`eighth-tool` (a server patched to list eight tools after a focus) must fail rule 1; `gap-attained` (an
answer that reports `attained: true` at A7) must fail rule 3. The log
(`target/e2e-logs/art_l9_session_a/<utc>.ndjson`, written through the shared helper) has one `cmd` line per
MCP request with the request verbatim, wall time, exit code of the envelope and its `evidence` block; one
`assert` line per expectation with observed and expected values and the step identifier; refusals with code
and first repair; the metrics `tools_listed` (<= 10) and `wait_overrun_ms` (<= 500) per step; and the
`summary`.

### Known answers and negative controls

- Known answers (exact; shared with the fixture of artifact L.4): the first and refined answers, the outer
  set with area fraction 1/3, the refutation at (9, 5/2) with 94 N against 88 N, the cell centre (7, 47/20)
  with half-width 1/7 m, the two symmetry groups of orders 4 and 2.
- KA-763 (idempotency: the same key and payload give one job and one charge and the retry returns the stored
  receipt with `idempotent_replay: true`; the same key with a different digest is refused, `FROB-REF-803`,
  exit 7) is exercised on step A7.
- Negative controls: the two plants; a call to `nemesis_search` while `mission` is focused (refused by the
  library as a disabled tool); a step file with a tool the registry lacks.
- Metamorphic relation: running the steps of A3 to A10 on the CLI instead of MCP gives the same results.
- Determinism lanes: twice on one machine; debug and release; Linux and macOS (the first half is class D0
  apart from session identifiers and costs).

## Considerations

- The specimen lists six scopes for the session. Under a per-cluster scope rule the session's tools need
  more (for example `control:run` for `control_synth`). The script computes the needed union from the
  registry, asserts that the specimen's six are a subset, and logs the difference as the metric
  `scopes_beyond_specimen`; the capability bead (BD-258) decides whether a broader scope such as
  `design:write` implies them.
- The transcript shows exit 0 for A7 and exit 2 for A9 although both carry a warning; the step file accepts
  either code for both and asserts the warnings by content.
- If kill gate KILL-MCP fires, the driver's stdio lane runs against the socket fallback of the server; under
  the CLI-only fallback the table is asserted on the CLI twins alone and the MCP half is recorded as not run.
- Cell and proof labels are the specimen's; comparisons are by box and by proof kind, with the specimen's
  labels mapped in the expectation files.

## References

Appendix L.9 (first half; rules 1 to 3), §32.4 (envelope and exit codes), §32.12.3 to §32.12.5, §32.12.9,
§32.16, §25.3.3; INV-TETHER-05 (one job and one charge per principal, verb and key); KA-763; KILL-MCP.
