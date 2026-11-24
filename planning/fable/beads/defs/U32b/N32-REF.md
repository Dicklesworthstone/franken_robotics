---
key: "N32-REF"
title: "Tether refusal-code registry: FROB-REF-701 to -704, -720, -723 to -726, -801 to -810, the packet codes and the internal fault"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "frob-verbs", "frob-core", "solid", "registry", "review-notes-open"]
parent: "WP-06"
deps: ["BD-005", "N35-33", "N35x-02", "STD-E2E"]
needs: []
related: ["N19b-03", "N24a-41", "N32a-25", "N32a-33", "N32ax-01", "N32b-02", "N32b-03", "N32b-05", "N32b-15", "N32b-19", "N32b-20", "N32b-24", "N32b-25", "N32bx-02", "N35-26", "N35-REF", "N35x-01", "N35x-02", "N36-REF"]
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
mechanism bead can raise its code without other reading, and tests the rows themselves. Five refusals the
specification states without a number (DESIGN, below) are requests of this bead in the allocation ledger: four
in N35-33's first round, and `resource-read-over-cap` in N35x-02's second. Two of them are rows: 726
`mechanism-not-admitted` and the internal fault 990 `internal-fault`, the proposals the ledger's rule takes.
The other three are in the `8xx` family, whose four free numbers (836 to 839) are held by the owner decision
N35x-01 until its ruling: they are `held` requests with interim `none` (a proposed or held number is never an
interim, rule 7 of N35-33), listed by slug in the interim list, and they become rows only when the ruling
releases numbers; until then the mechanisms that raise them stay open on that point. The mechanisms raise the
codes; the planted-violation battery (N32b-21) fires each on both surfaces.

Exit codes (DESIGN; exit 1 is never emitted on purpose, so a 1 always means an unhandled failure):

| Exit | Meaning | Codes of this registry |
|---|---|---|
| 0 | Completed, nothing to flag | none |
| 2 | Completed with findings or flags | none |
| 3 | Refused input or question | 801, 802, 805, 806, 807, 808 |
| 4 | Budget: refused at admission or stopped; nothing partial is published as a result | 701, 702, 703, 704 |
| 5 | Capability not implemented, not admitted, or outside the token's scopes | 720, 723, 724, 725, 726 |
| 6 | Backend or device unavailable | none (the `6xx` block) |
| 7 | Conflict: key reuse with different content, stale parent, lineage or plan conflict | 803, 804, 809 |
| 8 | Accepted and still running; the envelope carries a job handle; not a failure | none |
| 9 | Indeterminate; never retry blindly; recover by key. Also integrity and internal faults | 810, 990 |

Reason for refusals instead of warnings at this boundary: an agent optimizes against whatever the surface lets
through. A truncated page, a silently defaulted parameter or a half-published result each produce a number that
means something other than what its reader assumes, and a warning is dropped at the first copy.

## Scope

In: the eighteen rows; message templates; `pointer`, `observed`, `required` conventions; retry class; ranked
repairs; exit code; the verbs each code is listed under; registry tests and a registry end-to-end script; the
mapping of the four replay-packet refusals and of the resource-size refusal, which the specification states
without codes; the ruling on living-world refusals (724 and the new 726); the internal-fault row 990, which the
envelope bead N32a-03 raises from M0 and which no verb lists.
Out: raising each code (the mechanism beads in the table); the envelope type (N32a-03); the battery on both surfaces
and the check that `frob schema frob.refusal/1` and the resource `frob://registry/refusals` serve these rows
(N32b-21, M3, since the verbs and the resource are built by BD-022 and N32a-25, which depend on this registry);
codes this chapter only consumes (`FROB-REF-731` private object without the private scope, `-734` a path outside
the session's roots, `-840` sealed confirmation data, `-562` a fork below determinism class D2).

## Technical approach

Rows to enter in `registries/refusals.toml`. "Due" is the milestone at which the raising mechanism first exists.

| Code | Refused | Trigger (the condition the check tests) | Ranked repairs | Retry | Exit | Raised by; verbs | Due |
|---|---|---|---|---|---|---|---|
| FROB-REF-701 | The estimate's upper end exceeds the remaining budget | At admission, the upper end of the estimate interval (twice the point estimate while the stratum is `uncalibrated`) exceeds what is left in any capped unit of the session. The estimate, its interval and the binding unit are attached | 1. Ask a smaller question: the refusal names the largest worlds × seconds that fit. 2. Use a cheaper tier. 3. Open or derive a session with a larger budget | after-change | 4 | Admission (N32a-15); every `run` verb | M2 |
| FROB-REF-702 | Budget exhausted during a run | A running job's charged spend reaches its reservation at a kernel-batch boundary. The job drains, unpublished state is discarded, nothing partial is published as a result, and the acknowledgement names the last committed tick per world | 1. Resubmit with a larger budget and a new key; estimate against actual is attached. 2. Reduce the question. 3. Read the telemetry committed up to the named tick | after-change | 4 | Job engine; every `run` verb, `job.wait`, `job.status` | M2 |
| FROB-REF-703 | Admission guard | Device utilization at or above 0.8 (TARGET), or the environment of the job's shape class already holds 64 jobs (TARGET). `retry: backoff` with `poll_after_ms` | 1. Retry after the stated delay with the same idempotency key. 2. Ask for a CPU tier. 3. Shrink the job into the interactive class (point estimate at most 30 s of device time) | backoff | 4 | Dispatcher; every `run` verb | M3 |
| FROB-REF-704 | A campaign-class job without an approved compute envelope | The system's own estimate places the job in the `campaign` class (above the machine envelope) and its scope has no approved envelope. The scope of a demand is, in order: the token of the launcher it runs under (`frob-launch`, the campaign runner), the `envelope_scope` the request cites, a scope the verb derives from its own inputs (`train.family`), otherwise `adhoc`. Raised by the daemon's size classifier when the job is submitted, after the device-fit check (602) and before the envelope admission (761) and the session-budget check (701); the scheduler raises it again at dispatch and at resume when the approval the job was queued under has lapsed (BD-296). An over-envelope run executed in process, and a job above an approved scope's per-run or cumulative limit, are FROB-REF-761 (Considerations, d) | 1. Split the work below the envelope; the refusal gives the envelope and the estimate. 2. Ask the owner to approve an envelope for the scope (a request row of `registries/compute_envelope.toml`) and resubmit citing it with `--envelope-scope <scope>` (over MCP, the `envelope_scope` input) | after-change | 4 | Size classifier at submission; scheduler at dispatch and resume (BD-296); `study.submit`, `train.start`, `opt.start`, `opt.cascade`, `mission.synthesize`, `nemesis.search` | M3 |
| FROB-REF-720 | The session token lacks a required scope | A capability named in the verb's registry entry is not in the token's scope set. Checked before the handler runs, so no partial data exists | 1. Open a session that holds the scope (named in `required`). 2. Ask the parent session to derive a token that includes it | after-change | 5 | Token check (N32b-01); every verb | M2 |
| FROB-REF-723 | Token invalid or expired, or a widening attenuation | The token is malformed, its authentication tag does not verify, its expiry has passed, its session is closed, or a derivation asks for a scope or a budget its parent does not hold. Minting a root session (N32b-01): a root `session.open` for a principal with no `principal.<id>` table, or any root call when the configuration grants no principal (`observed` the principal); a root request beyond the grant (a scope outside it, a budget above the grant's amount or a unit it lacks, a lifetime above `max_lifetime_s`, a namespace other than the grant's) or for an `owner_only` scope (`observed` each widening element, `required` the grant's values); a root open that presents a valid token. Verification (N32b-01): a call that presents no token to a reachable daemon, other than `session.open` without `derive` and the public operator verbs `capabilities`, `schema`, `doctor` and `robot-docs` (`observed: "no token"`); a token presented when no daemon is reachable to verify it (fail closed) | 1. Open a new session (`frob session open`; the first repair of a call with no token). 2. For a derivation, or a root request beyond the grant, request a subset: `observed` lists the widening elements and `required` the parent's or the grant's values. For a principal with no table: the owner adds a `principal.<id>` table to `safety.toml`. For a root open that presents a token: derive (`derive: true`), or open a root session without a token. For a token with no daemon: start `frobd`, or drop the token and run in process as the principal `local` | after-change | 5 | Token check, root minting and derivation (N32b-01, N32b-02); every verb, `session.open` | M2 |
| FROB-REF-724 | The verb, or a mechanism the request names, is not in this build | The verb's registry entry is present but no handler is linked in this build (it arrives at a later milestone, or its feature group is off), or its capability was killed at its kill gate (the gate's decision in force, its decision entry of N40-03, has outcome `fired`); likewise a mechanism a request names explicitly (a living-world tier forced by `--require-living-tier`, a fork overlay in a replay packet) whose feature is not built or whose kill gate's decision in force has outcome `fired` with a fallback that excludes the mechanism for every purpose (a `fired` decision whose fallback form keeps the mechanism for some purposes makes a forced tier in another purpose 726 `purpose-not-eligible`). A capability whose handler is linked but which does not yet meet all eight conditions of "implemented" (code in its crate, a ten-section contract, known answers against a pinned oracle, the claim product on outputs, a CPU reference path, reachability through `frob --json`, tested cancellation, a recorded gate pass) runs and is reported `partial`; it is not refused. Block fields: `reason` (`not-linked`, `feature-not-built`, `killed: <gate>`), and where a mechanism is named `mechanism`, `declared_tier` | 1. `frob capabilities --json` lists what is implemented and the entry's `first_milestone`. 2. Use the alternative the entry names, if any (for a living-world mechanism: accept the recorded Solid alternative) | never | 5 | Dispatch (N32b-19); every verb. Living-world admission (N19b-03); packet fork switch (N32b-41) | M0 |
| FROB-REF-725 | A write outside the caller's namespace or lineage | A `mutate` verb targets a mutable name or a lineage whose owning namespace is not the token's | 1. Fork into your own lineage: `design new <name> --from <content address>`. 2. Ask the owner; the lineage identifier is usable as a mail thread identifier | after-change | 5 | Namespace check (N32b-03); every `mutate` verb | M3 |
| FROB-REF-726 (DESIGN, new; `mechanism-not-admitted`, assigned by N35-33's first round) | A mechanism that is in this build is not admitted for the run's declared purpose, tier or inputs | The capability is built and not excluded outright by a kill decision, but its admission rule excludes this use. Four reasons: `island-in-training` (a coupled living-world island named by a training or throughput run; the purpose is checked before the build, so this is the answer on every build, with or without `moonshot-islands`); `tier-not-eligible` (an island, or a living-world tier forced with `--require-living-tier`, on a tier its admission rule excludes, the CPU referee being the island's one eligible tier), when the request forbids substitution or substitution is not allowed; `purpose-not-eligible` (a living-world tier forced with `--require-living-tier` in a run whose purpose a kill decision's fallback form excludes: KILL-PROXY's decision in force `fired` on cost with the form "articulated proxies in evaluation batches only", N19b-19, under which N19b-03's admission, step 2, admits a chain cord or hinged rug for the purposes `evaluation`, `oracle-check` and `counterexample-verification` and not for `training` or `throughput`); `card-provenance` (a mechanism whose admission requires a card field of provenance `measured` or a cited `datasheet`, given a field of provenance `generic` or `assumed`: the series elasticity of an actuator, which needs `series_stiffness` so, N15b-18). Block fields: `mechanism`, `reason`; for the first three also `declared_tier` and `run_purpose`; for `purpose-not-eligible` also `decision` (the path of the decision's ADR, `docs/adr/ADR-<gate>-<milestone>.md`); for `card-provenance` also `field`, `provenance_found` and `provenances_admitted` | For `island-in-training` and `tier-not-eligible`: 1. Accept the recorded Solid alternative (drop `--require-living-tier`). 2. Run the island under purpose `oracle-check` or `counterexample-verification` on the CPU referee. For `purpose-not-eligible`: 1. Drop `--require-living-tier` and accept the Solid alternative. 2. Run under `--purpose evaluation`. For `card-provenance`: 1. Use a card revision whose field has provenance `measured` or a cited `datasheet`. 2. Run without the option and accept the flag the mechanism names (`compliance not modelled` for series elasticity) | after-change | 5 | Living-world admission (N19b-03); `sim.run`, `sim.estimate` (which applies the run's admission to the request it prices, N19b-03 step 7; the KILL-PROXY decision, N19b-19, provokes `purpose-not-eligible` through it), `train.start`, `eval.run`, `study.submit`. Actuator model admission (N15b-18, M6): every `run` verb that admits a model with the option enabled (`sim.run`, `task.fixture`) | M3 (`card-provenance` M6) |
| FROB-REF-801 | Unknown verb (CLI) | The command names no registry entry. On MCP the library answers an unknown tool with a JSON-RPC error (−32602) before any handler runs; that is the one permitted difference between the surfaces | 1. Up to three nearest registry names by edit distance. 2. `frob robot-docs` | after-change | 3 | CLI parser | M0 |
| FROB-REF-802 | Arguments fail the schema | Schema validation fails, including an unknown parameter and a `sync_wait_ms` outside 0 to 30,000. Nothing was charged | 1. The nearest valid parameter name. 2. `frob schema <id>` for a minimal valid example | after-change | 3 | CLI parser; MCP input validation mapped to an envelope skeleton; every verb | M0 |
| FROB-REF-803 | Idempotency key reused with different resolved content | In the admission transaction, (principal, verb, key) exists with another canonical request digest. Both resolutions are shown | 1. Use a new key for the new request. 2. Resend the original content, which `job.find` returns | after-change | 7 | Idempotency binding (N32a-09 in the ledger, BD-211 on the daemon); every `mutate` and job-submitting `run` verb | M1 |
| FROB-REF-804 | Stale parent, or a stale field value | `expected_parent` is not the head of the lineage; or, in `design.edit`'s two-value form (`set-param <field> <expected> <new>`), the field's value at the parent differs from the expected value (DESIGN, BD-058: a field-level stale read is the same conflict as a stale parent). The current head is shown; for the field form the payload also names `field`, `expected` and `current` | 1. `design diff <expected> <head>`, re-apply the edit on the head, resubmit (for the field form: read the field at the head and resubmit with its current value as the expected one). 2. Fork a new lineage from the old parent | after-change | 7 | Lineage write; every `mutate` verb that takes `expected_parent`; `design.edit`'s two-value form (BD-058). A `design.patch` document whose own `parent` field is stale is FROB-REF-112, raised by the patch reader (Considerations, a) | M1 |
| FROB-REF-805 | No idempotency key | A `mutate` verb or a job-submitting `run` verb arrives without a key | 1. Supply `--key <string>`. 2. For a look without spending, use `--estimate`, which needs none | after-change | 3 | Dispatch, before anything is charged (N32a-09) | M1 |
| FROB-REF-806 | Cursor does not match the query | The cursor's query digest (object, filters, ordering) or schema version differs from the request's, or its authentication tag fails, or it was issued to another principal | 1. Start a new query. 2. Repeat the original filters with the cursor | after-change | 3 | Cursor decoder (N32a-10); every paginated verb | M1 |
| FROB-REF-807 | Cursor expired | The frontier pinned in the cursor has been garbage-collected | 1. Start a new query; the answer states the new frontier. 2. Read sooner, or use `follow` | after-change | 3 | Cursor decoder (N32a-10) | M1 |
| FROB-REF-808 | An integer outside ±(2⁵³ − 1) in a number-typed field | An `int53` field holds, or would be serialized with, a value beyond ±9,007,199,254,740,991 | 1. Send the value in the field's `u64str` or `i64str` form, which the schema names. 2. If it is a count, it is out of range: reduce it | after-change | 3 | Serializer and input validation | M1 |
| FROB-REF-809 | Plan not found, or bound to a superseded identity | `study.submit` names a plan identifier that does not exist, or whose manifest binds an identity (design, scene, task, backend, stack) that has since been superseded | 1. Run `study.plan` again and submit the new identifier. 2. `study.status` lists live plans | after-change | 7 | Two-phase study; `study.submit` (N33b-19 on the CPU at M1 for a missing plan; N32a-13 adds the superseded-identity rules on the daemon at M3) | M1 |
| FROB-REF-810 | Chunk conflict | Different bytes are offered for an extent that is already committed. The job fails as indeterminate; both content hashes are attached | 1. Do not resubmit blindly: `job.find` by key. 2. Run `spark check`, then replay from the last committed checkpoint as a new job. 3. Keep both hashes: this is an alarm | never | 9 | Publishing transaction (N32a-08 in the ledger; BD-210 on devices) | M1 |
| FROB-REF-990 (DESIGN, new; `internal-fault`, assigned by N35-33's first round) | An internal fault | A handler panicked or returned an error that maps to no code, or the serializer refused to emit an out-of-range `int53` (a programming error, never a clipped number). Caught at the top of `main` and around every handler call in a long-lived process; one envelope is still written, `ok: false`; `pointer` names the verb; `observed` holds the path of the diagnostic file | 1. When the call carried an idempotency key, `frob job find --verb <verb> --key <key>`, because a job may have been admitted before the fault. 2. `frob doctor` | never | 9 | Envelope writer (N32a-03); `frob` (BD-022); `frob-mcp` (BD-023, as the structured content of an error result); the integer serializer (N32a-07). Raisable by every verb and listed in no verb's `refusals` (N32a-02, RL-06: it is outside every list and never appended) | M0 |

**Message templates and field conventions (DESIGN).** One line each, stating what was refused and the observed and
required values. `pointer` is a JSON pointer into the resolved request (`/budget/gpu_s`, `/sync_wait_ms`,
`/cursor`, `/session/scopes`). `observed` and `required` are typed values with `unit` (`s`, `core_s`, `bytes`,
`scope`, `digest`). Budget codes fill `observed` with the estimate's upper end and `required` with what is left.
`affected_claims` is empty for every row of this registry, since none concerns evidence strength. Messages never contain text
that originated in an import.

**The four replay-packet refusals.** The specification states them without codes. DESIGN mapping, which the packet
beads (N32b-41 and the writer) use:

| Condition | Code | Exit | Note |
|---|---|---|---|
| A packet over its byte cap | Ledger request `packet-over-byte-cap` (proposed 836), held under N35x-01; interim none | 3 | Carries the size table and the three reductions (half the frame rate, fewer forks, bodies restricted to a named subtree) |
| Frames requested from a chunk that is neither stored nor pinned | Ledger request `packet-frames-not-retained` (proposed 837), held under N35x-01; interim none | 3 | Names the chunk keys and their retention state |
| A fork overlay for a run whose determinism class does not license replay | Existing `FROB-REF-562` (a counterfactual fork below class D2) | 3 | Not redefined here |
| A fork overlay before the fork-harness gates are recorded for the build | Existing `FROB-REF-724` | 5 | Fork content in packets is switched off per build until those gates have a recorded pass, so the capability is not in the build |

One more surface refusal is stated without a code: an MCP resource read returns at most 4 MiB (TARGET), counted
on the encoded response (a base64 `blob` therefore carries at most 3 MiB of binary), and a larger object must be
read with `artifact.read` by byte range or exported under the session's export root. DESIGN: the ledger
request `resource-read-over-cap` (proposed 838), held under N35x-01 with interim none, exit 3, retry
`after-change`, repairs: (1) `frob artifact read <digest> --range <a>:<b>`;
(2) `artifact.export`. It carries the object's digest, its encoded size and the cap. Raised by the resource
handlers of `frob-mcp` (N32a-25, M3).

Numbers 726, 836, 837, 838 and 990 were free in the printed registry (checked against every bead and the plan
when this was written). The five are requests of this bead in the allocation ledger
`registries/refusal_allocations.toml` (rule of N35-33, a dependency), each with its number as `proposed`, its
family and decade and its condition; the ledger's rule decides the number, never the order of arrival. Four are
rows of N35-33's first round and one of N35x-02's second. The status of each (DESIGN slugs):

| Slug | Round | Proposed | Family, decade | Ledger status | Interim |
|---|---|---|---|---|---|
| `mechanism-not-admitted` | first (N35-33) | 726 | `7xx`, `72x` (exit 5) | `assigned` 726: rule 5 takes the proposal (the lowest free number above Tether's 725 in its decade) | none needed: the row 726 |
| `packet-over-byte-cap` | first (N35-33) | 836 | `8xx` (exit 3) | `held` under N35x-01 (836 to 839 are the family's only free numbers; rule 8) | none: listed in the interim list, not a row; N24a-41 and N32b-41 stay open on the number |
| `packet-frames-not-retained` | first (N35-33) | 837 | `8xx` (exit 3) | `held` under N35x-01 | none; as above |
| `resource-read-over-cap` | second (N35x-02) | 838 | `8xx` (exit 3) | `held` under N35x-01 | none; N32a-25 stays open on the number |
| `internal-fault` | first (N35-33) | 990 | `9xx` (exit 9, `retry = "never"` by BD-005's family rule) | `assigned` 990: the proposal is free under rule 4's test | none needed: the row 990 |

When the ruling of N35x-01 is recorded and the round is re-run, a held request receives its proposal or another
number and becomes a row of this registry under it; the beads that raise it (N24a-41 and N32b-41 for the two
packet refusals, N32a-25 for the resource read) take the number in the same change, by the one-line edit of
their named constant (`REFUSAL_PACKET_OVER_BYTE_CAP` and `REFUSAL_PACKET_FRAMES_NOT_RETAINED` for the packet
refusals). Allocation is append-only.

**Ruling on the living-world refusals (asked by N19b-03).** DESIGN: a living-world mechanism that is not built or
was killed at its kill gate, when the request forbids substitution, is 724 with `reason` `feature-not-built` or
`killed: <gate>` (the capability is not in this build: one code, and `frob capabilities --json` stays the single
place an agent looks). A mechanism that is built but not admitted for this run's purpose or tier is 726, not 724.
Reason: 724 says "not in this build" with `retry: never` and a first repair of reading `capabilities`, which would
report the island as implemented; the agent's actual repair is to change the purpose or accept the alternative,
which is `after-change`. Two codes keep each refusal's retry class and first repair truthful. Order of the checks
(confirmed for N19b-03): the run purpose is checked before the build, so a training or throughput run that names
an island answers 726 `island-in-training` on every build, including one built without `moonshot-islands`: the
repair is to change the purpose, never to build a feature. 724 is reached only when the purpose is admitted and
the mechanism's feature is not built or was killed. A mechanism refused for the provenance of a card field its
admission needs (`card-provenance`, N15b-18) is 726 for the same reason: it is built, and the repair is a better
card, not a build. So is a mechanism that a kill decision keeps for some purposes only (`purpose-not-eligible`,
KILL-PROXY `fired` on cost, N19b-19): it is built and admitted under another purpose, and the repair is the
purpose or the alternative.

**Three standing rules** tested with the rows: refuse, never truncate (a long answer is paginated and says so); a
budget stop publishes nothing partial as a result; a misspelt parameter is a refusal, not a default.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the eighteen rows and the two DESIGN rows 726 and 990 (the numbers N35-33's
      first round assigned) with message, pointer convention, retry class, at least one repair each, exit code,
      owning section, raising bead and due milestone; the three held requests (`packet-over-byte-cap`,
      `packet-frames-not-retained`, `resource-read-over-cap`) are in the interim list by slug with interim `none`,
      their condition and repairs, and are not rows; the loader accepts the file and rejects a planted unknown
      code and a planted row 836 while the hold of N35x-01 stands.
- [ ] Every code maps to exactly one exit code, and the map equals the table above (a test over the whole
      registry, not only these rows).
- [ ] Every code is listed in the `refusals` field of at least one entry of `registries/verbs.toml` once its
      mechanism exists; a code listed by no verb at its due milestone fails the lint (the lint reads the `due`
      field, so it bites at M1 for 803 to 810 and at M3 for 703, 704, 725, 726). 990 is exempt: by rule no verb
      lists it (N32a-02, RL-06), and a verb entry that writes it fails the registry lint. The held
      `resource-read-over-cap` is exempt too: it is raised by the MCP resource handlers (N32a-25), which are not
      verbs, and its entry in the interim list names those handlers as its raiser.
- [ ] The five requests are rows of the allocation ledger: `mechanism-not-admitted` and `internal-fault`
      `assigned` 726 and 990; `packet-over-byte-cap`, `packet-frames-not-retained` and `resource-read-over-cap`
      `held` with interim `none`; the mapping tables are in the crate's `CONTRACT.md` refusals section.
- [ ] The overlaps named under Considerations are recorded in both rows of each pair (`related`, and `surfaces_as`
      on 878 and 896), each with a test that the two codes are raised by different layers.
- [ ] `scripts/e2e/tether_refusal_registry.sh` passes and its log names the 20 rows, the 3 held entries and
      their exits.

## Test plan

### Unit tests
`crates/frob-verbs/tests/refusal_registry.rs` (real registry file, no mocks):
- `tether_rows_present`: the eighteen codes and the two DESIGN rows (726, 990) exist with non-empty message and
  repairs.
- `held_requests_have_no_interim`: the ledger rows `packet-over-byte-cap`, `packet-frames-not-retained` and
  `resource-read-over-cap` are `held` with `interim = none`; the interim list of `frob --json schema` carries the
  three by slug with message and repairs; none is a registry row; a scratch registry that enters 836 as a row,
  and a scratch ledger whose held row has `interim = 836`, each fail to load while the hold stands.
- `one_exit_per_code`: the map code → exit is a function; 701–704 → 4; 720, 723, 724, 725, 726 → 5; 801, 802,
  805–808 → 3; 803, 804, 809 → 7; 810, 990 → 9.
- `retry_classes`: 703 is `backoff`; 724, 810 and 990 are `never`; the rest (726 included) `after-change`.
- `internal_fault_row`: 990's repairs are, in order, `frob job find --verb <verb> --key <key>` and `frob doctor`;
  its `observed` is typed as a path; no entry of `registries/verbs.toml` lists it.
- `ref804_covers_field_form`: the 804 row's payload fields include `field`, `expected`, `current` and `head`, and
  its raising sites include `design.edit` (BD-058).
- `living_world_codes_distinct`: the 724 row's `reason` vocabulary is {`not-linked`, `feature-not-built`,
  `killed: <gate>`} and the 726 row's is {`island-in-training`, `tier-not-eligible`, `purpose-not-eligible`,
  `card-provenance`}; the two sets are disjoint; the 726 row documents that the purpose check precedes the build
  check; its block for `purpose-not-eligible` carries `decision`; its repairs for `purpose-not-eligible` (drop the
  forced tier; run under `evaluation`) and for `card-provenance` (a better card; run without the option) differ
  from those of the first two reasons; its raising sites include `sim.estimate`.
- `due_milestones`: 724, 801, 802, 990 → M0; 803 to 810 → M1; 701, 702, 720, 723 → M2; 703, 704, 725, 726 → M3.
- `unknown_code_is_load_error`: a planted `FROB-REF-799` reference in a verb entry fails the load.
- `codes_never_reused`: the registry history file lists retired codes; adding a row with a retired number fails.
- `messages_carry_no_import_text`: every template's placeholders are typed identifiers or numbers, never strings of
  untrusted type.
- `sync_wait_bounds_in_802`: 30,001 and −1 select 802 with `pointer: /sync_wait_ms`; 0 and 30,000 are accepted.
- `layer_pairs_cross_referenced`: the rows of 804, 806, 810 and 704 name their partner codes 112, 896, 878 and 761
  in `related`, and the rows of 896 and 878 carry `surfaces_as` pointing at 806 and 810 with equal exit codes.

### End-to-end script and logging
`scripts/e2e/tether_refusal_registry.sh` (header `bead=N32-REF milestone=M0 labels=tether,registry`) sources the
STD-E2E helper, runs `cargo test -p frob-verbs --test refusal_registry`, then applies three plants to scratch copies
of the registry (one code given two exits; a verb entry citing `FROB-REF-799`; a retired number reused) and
requires each to be refused by the loader. It writes `target/e2e-logs/tether_refusal_registry/<utc>.ndjson` and a
transcript: `ts`, `script`, `bead`, `step`, `event`, the exact command line, exit code, wall time, one `assert` per
row (code, exit, retry, due) against the table, a `metric` line with rows present against the threshold 20 and
held entries (interim `none`) against 3, one
`refusal` line per plant with the loader's reason, the digest of `registries/refusals.toml`, on failure the first
missing or differing row and the registry path, and a `summary`. From the user's side the rows are exercised by
`scripts/e2e/tether_refusals.sh` (N32b-21, M3), which fires each code through `frob` and `frob-mcp` and also reads
them back through `frob schema frob.refusal/1 --json`, `frob robot-docs --json` and `frob://registry/refusals`.

### Known answers and negative controls
- KA-755: on a fixture of malformed, unknown, over-budget, unscoped and conflicting inputs, each returns the exit
  and refusal code of the tables above (exact).
- Negative controls: a registry in which one code maps to two exits must fail `one_exit_per_code`; a verb entry
  that lists no refusals must fail the registry lint; a serializer that truncates a long list instead of paginating
  must fail the standing-rule test in N32b-21.

## Considerations

- Overlapping conditions, assigned to layers and not merged silently. DESIGN: the plan defines each pair in two
  chapters and does not divide them; each is divided here by raising site, for one reason: one event must reach a
  caller under one code, or agents handle it two ways. The partner registries (N14-REF for 112; N29-REF for 878
  and 896; N35-REF and N36-REF for 761) state the same division, and a change to one is a change to both.
  (a) 804 and 112: a `design.patch` whose patch document names a `parent` that is not the head is refused with
  112 by the patch reader of the design crate, which returns the current head and the diff between the two; that
  request is never also reported as 804. 804 is raised by the surface's lineage write for the `expected_parent`
  argument of every other `mutate` verb. Both exit 7.
  (b) 806 and 896: through any verb the envelope's refusal is 806. The surface decodes the cursor before the
  store is read and refuses a query digest (object, filters, ordering) or schema version that differs from the
  request's; a changed filter with an old cursor is therefore 806 for every caller of a verb, as the pagination
  gate asserts. 896 is the store reader's own check of pin, filter and order: it is what a direct caller of the
  store's paging interface receives (a library consumer, the conformance worker), and when it fires under a verb
  the envelope carries 806 with 896 as its cause (the row of 896 has `surfaces_as = "FROB-REF-806"`). Both exit 3.
  (c) 810 and 878: 878 is the store's refusal to the publishing worker and its record of the conflicting chunk
  key; 810 is what the job and its caller see, with 878 as its cause (the row of 878 has `surfaces_as =
  "FROB-REF-810"`). Both exit 9 with `retry: never`.
  (d) 704 and 761 (convention 14): a job-submitting verb queued by the daemon is classified when the job is
  submitted, and a job in the `campaign` size class whose scope (the launcher's token, the cited
  `envelope_scope`, the scope the verb derives, otherwise `adhoc`) has no approved envelope is refused with 704
  by the size classifier, before the envelope admission is reached; the scheduler raises 704 again at dispatch
  and resume when the approval has lapsed (BD-296). Every other case is 761, raised by `sim.estimate` and run
  admission before the job starts: a run executed in process by `frob` (where no size class exists) above the
  class in force for its scope (E0 for `adhoc`), a daemon job whose scope has an approved envelope and that
  still exceeds its per-run limits or cumulative device-hours, and a cited `envelope_scope` that contradicts the
  launcher's token. Both come before the session-budget check 701, so an agent is never sent to raise a budget
  for a job the envelope refuses anyway. Both exit 4 with `retry: after-change`.
  No code of the four pairs is retired; if the registry's owner later merges a pair, the retired code stays
  retired.
- The exit of 810 is 9, not 3, because it is an integrity fault: the caller cannot know which bytes are right. 990
  is exit 9 for the same reason: after a fault the caller cannot know whether a job was admitted, so the first
  repair is a lookup by key, never a blind retry.
- OPEN-TETHER-3 can change the bounds of `sync_wait_ms`; the 802 row reads them from the registry, not a constant.
- A refusal is a product surface: repairs must stay executable. The battery runs the first repair of every code.

## References

§32.18 (table and standing rules), §32.4 (envelope, exit codes), §32.5, §32.7 to §32.11, §32.13, §32.24
(packet refusals); Appendix O.7 and O.8; KA-755; GATE-TETHER-03, GATE-TETHER-04.
