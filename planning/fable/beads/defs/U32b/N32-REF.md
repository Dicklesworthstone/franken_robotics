---
key: "N32-REF"
title: "Tether refusal-code registry: FROB-REF-701 to -704, -720, -723 to -725 and -801 to -810"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "frob-verbs", "frob-core", "solid", "registry"]
parent: "WP-06"
deps: ["BD-005", "BD-022"]
needs: ["§35.11: the three standing boundary rules (refuse, never truncate; a budget stop publishes nothing partial; a misspelt parameter is a refusal) as tested rules of the determinism and numerics chapter", "§29.17: the Chronicle codes FROB-REF-878 (different bytes for a committed chunk key) and FROB-REF-896 (a cursor used with a different pin, filter or order), whose conditions overlap FROB-REF-810 and FROB-REF-806 and must be assigned to distinct layers", "§14.13: FROB-REF-112 (stale parent in a patch), whose condition overlaps FROB-REF-804"]
related: ["N32-CX", "N32-KA", "N32a-03", "N32b-21", "N32b-01", "N32b-02", "N32b-03", "N32b-19", "N32b-41", "BD-299", "BD-211", "BD-297"]
gates: []
plan_refs: ["§32.18", "§32.4", "§32.5", "§32.7", "§32.8", "§32.9", "§32.10", "§32.11.2", "§32.13", "§32.24", "Appendix O.7", "Appendix O.8"]
---
## Background

Tether is the surface subsystem of FrankenRobotics: the verb registry, the result envelope, the command line `frob`,
the daemon `frobd`, the MCP server `frob-mcp`, the dashboard and the viewer. A verb either answers or refuses, and a
refusal says what would make the question answerable. Every refusal has a stable code, a message and ranked repairs.
The machine-readable registry is `registries/refusals.toml` (loader in `frob-core`); an unknown code is a load-time
error and codes are never reused.

With `--json` every verb writes exactly one envelope (`frob.envelope/1`). When `ok` is false the envelope carries a
`refusal` object: `code`, `message`, `pointer` (the field or part at fault, a JSON pointer into the request),
`observed`, `required`, `unit`, `affected_claims`, `retry` (`never`, `after-change` or `backoff`) and ranked
`repairs` (each an edit or command with its arguments). On MCP the same envelope is the structured content of a tool
result with `isError: true`.

This chapter defines eighteen codes: four budget codes and four capability codes in the `7xx` block, and ten protocol
codes in the `8xx` block. This bead enters the eighteen rows with trigger and repairs complete enough that a
mechanism bead can raise its code without other reading, and tests the rows themselves. The mechanisms raise the
codes; the planted-violation battery (N32b-21) fires each on both surfaces.

Exit codes (DESIGN; exit 1 is never emitted on purpose, so a 1 always means an unhandled failure):

| Exit | Meaning | Codes of this registry |
|---|---|---|
| 0 | Completed, nothing to flag | none |
| 2 | Completed with findings or flags | none |
| 3 | Refused input or question | 801, 802, 805, 806, 807, 808 |
| 4 | Budget: refused at admission or stopped; nothing partial is published as a result | 701, 702, 703, 704 |
| 5 | Capability not implemented, not admitted, or outside the token's scopes | 720, 723, 724, 725 |
| 6 | Backend or device unavailable | none (the `6xx` block) |
| 7 | Conflict: key reuse with different content, stale parent, lineage or plan conflict | 803, 804, 809 |
| 8 | Accepted and still running; the envelope carries a job handle; not a failure | none |
| 9 | Indeterminate; never retry blindly; recover by key. Also integrity faults | 810 |

Reason for refusals instead of warnings at this boundary: an agent optimizes against whatever the surface lets
through. A truncated page, a silently defaulted parameter or a half-published result each produce a number that
means something other than what its reader assumes, and a warning is dropped at the first copy.

## Scope

In: the eighteen rows; message templates; `pointer`, `observed`, `required` conventions; retry class; ranked
repairs; exit code; the verbs each code is listed under; registry tests; the mapping of the four replay-packet
refusals, which the specification states without codes.
Out: raising each code (the mechanism beads in the table); the envelope type (N32a-03); the battery on both surfaces
(N32b-21); codes this chapter only consumes (`FROB-REF-731` private object without the private scope, `-734` a path
outside the session's roots, `-840` sealed confirmation data, `-562` a fork below determinism class D2).

## Technical approach

Rows to enter in `registries/refusals.toml`. "Due" is the milestone at which the raising mechanism first exists.

| Code | Refused | Trigger (the condition the check tests) | Ranked repairs | Retry | Exit | Raised by; verbs | Due |
|---|---|---|---|---|---|---|---|
| FROB-REF-701 | The estimate's upper end exceeds the remaining budget | At admission, the upper end of the estimate interval (twice the point estimate while the stratum is `uncalibrated`) exceeds what is left in any capped unit of the session. The estimate, its interval and the binding unit are attached | 1. Ask a smaller question: the refusal names the largest worlds × seconds that fit. 2. Use a cheaper tier. 3. Open or derive a session with a larger budget | after-change | 4 | Admission; every `run` verb | M2 |
| FROB-REF-702 | Budget exhausted during a run | A running job's charged spend reaches its reservation at a kernel-batch boundary. The job drains, unpublished state is discarded, nothing partial is published as a result, and the acknowledgement names the last committed tick per world | 1. Resubmit with a larger budget and a new key; estimate against actual is attached. 2. Reduce the question. 3. Read the telemetry committed up to the named tick | after-change | 4 | Job engine; every `run` verb, `job.wait`, `job.status` | M2 |
| FROB-REF-703 | Admission guard | Device utilization at or above 0.8 (TARGET), or the environment of the job's shape class already holds 64 jobs (TARGET). `retry: backoff` with `poll_after_ms` | 1. Retry after the stated delay with the same idempotency key. 2. Ask for a CPU tier. 3. Shrink the job into the interactive class (point estimate at most 30 s of device time) | backoff | 4 | Dispatcher; every `run` verb | M3 |
| FROB-REF-704 | A campaign-class job without an approved compute envelope | The system's own estimate places the job in the `campaign` class (above the machine envelope) and the request cites no approved envelope | 1. Split the work below the envelope; the refusal gives the envelope and the estimate. 2. Ask the owner to approve an envelope and resubmit citing it | after-change | 4 | Size classifier; `study.submit`, `train.start`, `opt.start`, `opt.cascade`, `mission.synthesize`, `nemesis.search` | M3 |
| FROB-REF-720 | The session token lacks a required scope | A capability named in the verb's registry entry is not in the token's scope set. Checked before the handler runs, so no partial data exists | 1. Open a session that holds the scope (named in `required`). 2. Ask the parent session to derive a token that includes it | after-change | 5 | Token check (N32b-01); every verb | M2 |
| FROB-REF-723 | Token invalid or expired, or a widening attenuation | The token's authentication tag does not verify, its expiry has passed, its session is closed, or a derivation asks for a scope or a budget its parent does not hold | 1. Open a new session. 2. For a derivation, request a subset: `observed` lists the widening elements and `required` the parent's values | after-change | 5 | Token check and derivation (N32b-01, N32b-02); every verb, `session.open` | M2 |
| FROB-REF-724 | The verb is not implemented in this build | The verb's registry entry is present but no handler is linked in this build (it arrives at a later milestone, or its feature group is off), or its capability was killed at its kill gate. A capability whose handler is linked but which does not yet meet all eight conditions of "implemented" (code in its crate, a ten-section contract, known answers against a pinned oracle, the claim product on outputs, a CPU reference path, reachability through `frob --json`, tested cancellation, a recorded gate pass) runs and is reported `partial`; it is not refused | 1. `frob capabilities --json` lists what is implemented and the entry's `first_milestone`. 2. Use the alternative the entry names, if any | never | 5 | Dispatch (N32b-19); every verb | M0 |
| FROB-REF-725 | A write outside the caller's namespace or lineage | A `mutate` verb targets a mutable name or a lineage whose owning namespace is not the token's | 1. Fork into your own lineage: `design new <name> --from <content address>`. 2. Ask the owner; the lineage identifier is usable as a mail thread identifier | after-change | 5 | Namespace check (N32b-03); every `mutate` verb | M3 |
| FROB-REF-801 | Unknown verb (CLI) | The command names no registry entry. On MCP the library answers an unknown tool with a JSON-RPC error (−32602) before any handler runs; that is the one permitted difference between the surfaces | 1. Up to three nearest registry names by edit distance. 2. `frob robot-docs` | after-change | 3 | CLI parser | M0 |
| FROB-REF-802 | Arguments fail the schema | Schema validation fails, including an unknown parameter and a `sync_wait_ms` outside 0 to 30,000. Nothing was charged | 1. The nearest valid parameter name. 2. `frob schema <id>` for a minimal valid example | after-change | 3 | CLI parser; MCP input validation mapped to an envelope skeleton; every verb | M0 |
| FROB-REF-803 | Idempotency key reused with different resolved content | In the admission transaction, (principal, verb, key) exists with another canonical request digest. Both resolutions are shown | 1. Use a new key for the new request. 2. Resend the original content, which `job.find` returns | after-change | 7 | Idempotency binding; every `mutate` and job-submitting `run` verb | M2 |
| FROB-REF-804 | Stale parent | `expected_parent` is not the head of the lineage. The current head is shown | 1. `design diff <expected> <head>`, re-apply the edit on the head, resubmit. 2. Fork a new lineage from the old parent | after-change | 7 | Lineage write; every `mutate` verb | M1 |
| FROB-REF-805 | No idempotency key | A `mutate` verb or a job-submitting `run` verb arrives without a key | 1. Supply `--key <string>`. 2. For a look without spending, use `--estimate`, which needs none | after-change | 3 | Dispatch | M2 |
| FROB-REF-806 | Cursor does not match the query | The cursor's query digest (object, filters, ordering) or schema version differs from the request's | 1. Start a new query. 2. Repeat the original filters with the cursor | after-change | 3 | Cursor decoder; every paginated verb | M2 |
| FROB-REF-807 | Cursor expired | The frontier pinned in the cursor has been garbage-collected | 1. Start a new query; the answer states the new frontier. 2. Read sooner, or use `follow` | after-change | 3 | Cursor decoder | M2 |
| FROB-REF-808 | An integer outside ±(2⁵³ − 1) in a number-typed field | An `int53` field holds, or would be serialized with, a value beyond ±9,007,199,254,740,991 | 1. Send the value in the field's `u64str` or `i64str` form, which the schema names. 2. If it is a count, it is out of range: reduce it | after-change | 3 | Serializer and input validation | M1 |
| FROB-REF-809 | Plan not found, or bound to a superseded identity | `study.submit` names a plan identifier that does not exist, or whose manifest binds an identity (design, scene, task, backend, stack) that has since been superseded | 1. Run `study.plan` again and submit the new identifier. 2. `study.status` lists live plans | after-change | 7 | Two-phase study; `study.submit` | M1 |
| FROB-REF-810 | Chunk conflict | Different bytes are offered for an extent that is already committed. The job fails as indeterminate; both content hashes are attached | 1. Do not resubmit blindly: `job.find` by key. 2. Run `spark check`, then replay from the last committed checkpoint as a new job. 3. Keep both hashes: this is an alarm | never | 9 | Publishing transaction | M2 |

**Message templates and field conventions (DESIGN).** One line each, stating what was refused and the observed and
required values. `pointer` is a JSON pointer into the resolved request (`/budget/gpu_s`, `/sync_wait_ms`,
`/cursor`, `/session/scopes`). `observed` and `required` are typed values with `unit` (`s`, `core_s`, `bytes`,
`scope`, `digest`). Budget codes fill `observed` with the estimate's upper end and `required` with what is left.
`affected_claims` is empty for all eighteen, since none concerns evidence strength. Messages never contain text
that originated in an import.

**The four replay-packet refusals.** The specification states them without codes. DESIGN mapping, which the packet
beads (N32b-41 and the writer) use:

| Condition | Code | Exit | Note |
|---|---|---|---|
| A packet over its byte cap | New: `FROB-REF-836` (provisional number) | 3 | Carries the size table and the three reductions (half the frame rate, fewer forks, bodies restricted to a named subtree) |
| Frames requested from a chunk that is neither stored nor pinned | New: `FROB-REF-837` (provisional number) | 3 | Names the chunk keys and their retention state |
| A fork overlay for a run whose determinism class does not license replay | Existing `FROB-REF-562` (a counterfactual fork below class D2) | 3 | Not redefined here |
| A fork overlay before the fork-harness gates are recorded for the build | Existing `FROB-REF-724` | 5 | Fork content in packets is switched off per build until those gates have a recorded pass, so the capability is not in the build |

One more surface refusal is stated without a code: an MCP resource read returns at most 4 MiB (TARGET), and a
larger object must be read with `artifact.read` by byte range or exported under the session's export root.
DESIGN: `FROB-REF-838` (provisional number), exit 3, retry `after-change`, repairs: (1) `frob artifact read
<digest> --range <a>:<b>`; (2) `artifact.export`. It carries the object's size and the cap.

Numbers 836, 837 and 838 are free in the registry as specified. Allocation is append-only: if any is taken when
this bead is executed, the next free `8xx` number is used and recorded here.

**Three standing rules** tested with the rows: refuse, never truncate (a long answer is paginated and says so); a
budget stop publishes nothing partial as a result; a misspelt parameter is a refusal, not a default.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the eighteen rows with message, pointer convention, retry class, at least one
      repair each, exit code and owning section; the loader accepts the file and rejects a planted unknown code.
- [ ] Every code maps to exactly one exit code, and the map equals the table above (a test over the whole
      registry, not only these rows).
- [ ] Every code is listed in the `refusals` field of at least one entry of `registries/verbs.toml` once its
      mechanism exists; a code listed by no verb at its due milestone fails the lint.
- [ ] The three provisional codes are allocated (or re-numbered) and the mapping table is in the crate's
      `CONTRACT.md` refusals section.
- [ ] `frob schema frob.refusal/1 --json` and the MCP resource `frob://registry/refusals` return these rows.
- [ ] The overlaps named under Considerations are resolved in writing, each with a test that the two codes are
      raised by different layers.

## Test plan

### Unit tests
`crates/frob-verbs/tests/refusal_registry.rs` (real registry file, no mocks):
- `tether_rows_present`: the eighteen codes exist with non-empty message and repairs.
- `one_exit_per_code`: the map code → exit is a function; 701–704 → 4; 720, 723, 724, 725 → 5; 801, 802, 805–808 →
  3; 803, 804, 809 → 7; 810 → 9.
- `retry_classes`: 703 is `backoff`; 724 and 810 are `never`; the rest `after-change`.
- `unknown_code_is_load_error`: a planted `FROB-REF-799` reference in a verb entry fails the load.
- `codes_never_reused`: the registry history file lists retired codes; adding a row with a retired number fails.
- `messages_carry_no_import_text`: every template's placeholders are typed identifiers or numbers, never strings of
  untrusted type.
- `sync_wait_bounds_in_802`: 30,001 and −1 select 802 with `pointer: /sync_wait_ms`; 0 and 30,000 are accepted.

### End-to-end script and logging
The rows are exercised from the user's side by `scripts/e2e/tether_refusals.sh` (N32b-21). This bead adds a
registry step to it: `frob schema frob.refusal/1 --json` and `frob robot-docs --json`, asserting the eighteen codes
and their exits appear. NDJSON lines per the shared helper: `ts`, `script`, `bead`, `step`, `event`, command, exit
code, wall time, the envelope's `evidence` block, the count of rows against the threshold 18, and on failure the
first missing or differing row.

### Known answers and negative controls
- KA-755: on a fixture of malformed, unknown, over-budget, unscoped and conflicting inputs, each returns the exit
  and refusal code of the tables above (exact).
- Negative controls: a registry in which one code maps to two exits must fail `one_exit_per_code`; a verb entry
  that lists no refusals must fail the registry lint; a serializer that truncates a long list instead of paginating
  must fail the standing-rule test in N32b-21.

## Considerations

- Overlapping conditions, to be assigned to layers and not merged silently. (a) 804 and 112: 112 is raised by the
  design-patch reader for a patch file whose recorded parent is not the head; 804 by the surface for the
  `expected_parent` argument of any `mutate` verb. (b) 806 and 896: 806 by the surface cursor decoder on a query
  digest mismatch; 896 by the telemetry reader on a pin, filter or order mismatch inside a stream. (c) 810 and 878:
  810 is what the job and its caller see (exit 9); 878 is the store's record of the conflicting chunk key. (d) 704
  and 761: 704 when no envelope is approved; 761 when a job exceeds the envelope in force. If the owning units
  decide to merge any pair, the retired code stays retired.
- The exit of 810 is 9, not 3, because it is an integrity fault: the caller cannot know which bytes are right.
- OPEN-TETHER-3 can change the bounds of `sync_wait_ms`; the 802 row reads them from the registry, not a constant.
- A refusal is a product surface: repairs must stay executable. The battery runs the first repair of every code.

## References

§32.18 (table and standing rules), §32.4 (envelope, exit codes), §32.5, §32.7 to §32.11, §32.13, §32.24
(packet refusals); Appendix O.7 and O.8; KA-755; GATE-TETHER-03, GATE-TETHER-04.
