---
key: "N33-REF"
title: "registry: refusal codes of Suite consumption (FROB-REF-790, -791, -890) with triggers, repairs, exit codes"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "tether", "frob-core", "frob-verbs", "registry", "solid", "e2e"]
parent: "WP-06"
deps: ["BD-005", "BD-022", "STD-E2E"]
needs: ["§32.4: the exit-code table and the rule that every refusal code maps to exactly one exit code (GATE-TETHER-04); the `79x` range is not named in that table", "§33.15 and §33.17: refusals that the suite chapter states in words without a number (a scan whose scale is not metric; an action sampler without a qualification receipt), which belong to the registries of the scene and learning chapters"]
related: ["N33-CX", "N33c-13", "N33c-31", "N33c-32", "N33c-11", "BD-137", "BD-299"]
gates: []
plan_refs: ["§33.16", "Appendix O", "§35.11", "§32.4", "§33.14", "§33.9"]
---
## Background

A verb of FrankenRobotics never fails bare. A request that cannot be answered returns a **refusal**: a stable
code, a message, a pointer to the field at fault, the observed and required values, the claims affected, a retry
class and **ranked repairs** an agent can act on. Codes live in the machine-readable registry
`registries/refusals.toml`; an unknown code is a load-time error, and a code is never reused. The CLI, the daemon
protocol and the MCP tool definitions are generated from the verb registry, which lists for each verb the
refusals it may return. Families: `1xx` design and model; `2xx` numerics and integrator admission; `3xx` scene;
`4xx` control and estimation; `5xx` learning and evaluation protocol; `6xx` backend and device; `7xx` budget and
capability; `8xx` evidence and protocol; `9xx` internal. Three standing rules: refuse, never truncate; a budget
stop publishes nothing partial as a result; a misspelt parameter is a refusal, not a default.

The suite-consumption chapter, which fixes what the project takes from its sibling projects and behind which
boundary, defines three codes. They guard the three ways the boundary can be crossed wrongly: asking a binary for
a foundation it was not built with; talking to a referee worker that is not the referee the job names; and
treating results from two different stacks as one.

| Code | Name | When |
|---|---|---|
| `FROB-REF-790` | `foundation-group-absent` | A verb needs an airlock feature group that this binary was not built with |
| `FROB-REF-791` | `referee-worker-mismatch` | The worker is absent, or was built under a different stack receipt than the job names |
| `FROB-REF-890` | `stack-mismatch` | Evidence recorded under another stack receipt is offered as same-stack evidence without revalidation |

Terms: the *airlock* is the crate `frob-fsim`, the only crate that names FrankenSim's volatile types; its traits
are grouped into Cargo features (`referee`, `geometry`, `ledger`, `optimize`, `stats`, `spectral`, `actuation`,
`structure`, `multiphysics`, `harness`). The *referee worker* is the binary `frob-fsim-worker`, FrankenSim's code
behind a process boundary. A *stack receipt* is the BLAKE3 hash of the resolution receipt: sibling revisions,
toolchain, lockfiles, patches and build environment of a build.

## Scope

In: the three registry rows in full; the exit-code assignment; the verbs that may return each; the end-to-end
provocation of each code and one negative control per code. Out: the mechanisms that raise them (N33c-32 for
790; N33c-13 for 791; N33c-31 for 890).

## Technical approach

**Row format (DESIGN).** One `[[refusal]]` table per code in `registries/refusals.toml`: `code`, `slug`,
`family`, `message` (one sentence, no jargon), `pointer_kind`, `payload` (named fields), `retry` (`never`,
`after-change`, `backoff`), `repairs` (ranked; each a verb to call next, a command, or a text instruction),
`exit`, `defined_in`. The envelope of every verb with `--json` carries, when `ok` is false, `refusal { code,
message, pointer, observed, required, unit, affected_claims, retry, repairs }`.

**The three codes.**

| Code | Meaning | Trigger (exact) | Pointer and payload | Ranked repairs | Raised by | Retry | Exit |
|---|---|---|---|---|---|---|---|
| FROB-REF-790 `foundation-group-absent` | This binary was built without the airlock feature group the verb needs | At admission, before a job is planned or a budget is charged: the verb's `needs_groups` (from the verb registry, possibly argument-dependent) names a group that is (a) not compiled into this binary, or (b) compiled but without a passing admission record at this pin; in mode `remote`, a group missing from the worker's reported features. Also raised by the multiphysics bridge for a throughput-tier request, which the bridge never serves | `foundation/<group>/compiled`, `…/admitted` or `…/tier`; `observed`: the groups compiled (list); `required`: the missing group; the group's admission gate (INT-FSIM-01, -13, -14, -15, -16 or -17) when case (b) | 1. Use a build that has the group (`frob capabilities --json` lists this binary's groups). 2. Rebuild with `--features frob-fsim/<group>` (the message states the closure cost for `structure` and `multiphysics`). 3. For an unadmitted group, run its admission gate. 4. Use the documented fallback where one exists (a fluid film falls back to the Solid film-friction law; coupled islands have none) | Any verb with a non-empty `needs_groups`; `sim.run` with a coupled island; statistics, optimization, spectral, structure verbs as they arrive | `after-change` | 5 |
| FROB-REF-791 `referee-worker-mismatch` | The referee worker is absent, or is not the build the job names | Before any job frame is sent: (a) no worker executable or the spawn fails; (b) no handshake within the start timeout; (c) the worker's protocol major version differs; (d) the worker's stack receipt differs from the job's `stack_receipt`; (e) a development-build worker offered for gate-grade evidence. Also raised by the worker itself when a job's receipt differs from its own | `referee/worker/absent`, `…/no-handshake`, `…/protocol`, `…/receipt`, `…/development-build`; `observed`: the worker's receipt or protocol, or `none`; `required`: the expected value | 1. Rebuild the worker at the pinned commit (`scripts/build/worker.sh`). 2. Run `cargo xtask check-stack` and resolve the drift it reports. 3. If the worker is meant to stay on an older pin, set `worker_rev` in the stack lock and name that receipt in the job. 4. Use mode `inproc` if this binary has it | Any verb that reaches the referee in mode `remote`: `sim.run --tier referee`, `sim.replay --tier referee`, `fidelity.shadow`, the cloth oracle driver | `after-change` | 5 |
| FROB-REF-890 `stack-mismatch` | Evidence recorded under another stack receipt was offered as same-stack evidence | An operation that pools episodes, pairs two arms, serves a memoized result, promotes, or forms a performance ratio receives inputs whose stack identities differ (receipt, or referee stack) and the operation is not typed cross-stack; or a throughput-tier claim receives an input that crossed the multiphysics bridge or came from a worker at a different receipt | `evidence/<index>/stack`; `observed`: the offered receipt (and worker receipt); `required`: the current receipt; `affected_claims`: the claims of the request | 1. Revalidate the result under the current stack (`frob result revalidate <id>`). 2. Recompute. 3. Present the result as cross-stack evidence with both receipts shown (display and export only; not pooling or pairing) | `eval.compare`, `study.compare`, `eval.promote`, the memo graph's serving path, performance verbs, `report.*` when asked to pool | `after-change` | 3 |

**Exit codes.** `FROB-REF-890` exits **3** by the table's rule for the `8xx` family other than conflicts
(refused input or question). For `FROB-REF-790` and `FROB-REF-791` the table names exit 4 for the `70x` range
(budget) and exit 5 for the `72x` and `73x` ranges (capability not implemented, not admitted, or outside the
session token's scopes) and does not name `79x`. DESIGN: both exit **5**. Reason: each reports a capability this
installation does not have (a group not built or not admitted; a referee that is not present in the required
build); neither is a budget stop, a device fault (exit 6, the `6xx` family) or a conflict (exit 7). The row
states the exit explicitly, because every refusal code must map to exactly one exit code. On the MCP surface the
same envelope is the structured content of a tool result with `isError: true`.

**What none of the three may do.** None is retried with weakened inputs. None is downgraded to a warning: a
referee result from the wrong build, a pooled estimate across stacks, or an empty answer in place of a missing
group would each be a silent wrong answer. None returns a partial result.

**Refusals stated in words in this chapter's subsections, with no number here.** (1) A geometry generation whose
scale status is not metric is refused (the scan-import adapter); it belongs to the scene chapter's `3xx`
registry, working slug `scan-scale-not-metric`. (2) `train.start` refuses a sampler with no qualification receipt
(the learning chapter's refusal). (3) A request to use the reduced G1 profile as an import-parity reference is
refused or relabelled (the design chapter's import refusals). This bead records them so that the owners of those
registries number them; it assigns no code.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the three rows with every field of the table; the registry loads, and an unknown code is a load-time error.
- [ ] Each code is provoked end to end through the real `frob` binary and returns its code, pointer, observed and required values, repairs, and exit code (5, 5, 3).
- [ ] The generated refusal list of every verb named in "Raised by" contains the code; no verb returns one of the three without listing it.
- [ ] Each code maps to exactly one exit code on both surfaces; the MCP result carries the same envelope with `isError: true`.
- [ ] For each code the "must not" holds under test: no retry with altered inputs, no warning-and-continue, no partial result.
- [ ] The three unnumbered refusals are filed with the owning registries.

## Test plan

### Unit tests

`cargo test -p frob-core refusal_registry_suite` and `cargo test -p frob-verbs refusals_suite`:
- `rows_790_791_890_load`: fields non-empty; `exit` is 5, 5, 3; `retry` is `after-change`.
- `repairs_are_ranked_and_actionable`: each repair is a verb, a command or a text instruction; verbs named exist in the verb registry.
- `pointer_kinds_enumerated`: the pointer strings of each code are exactly the listed cases.
- `one_exit_code_per_refusal`: no code of this set appears with two exits.
- `verb_refusal_lists_contain_codes`.
- `envelope_shape`: a refusal envelope for each code validates against the envelope schema, with `observed` and `required` present.

### End-to-end script and logging

`scripts/e2e/suite_refusals.sh`:
1. **790.** With the default binary, call a verb that needs an off-by-default group (the conformance verb
   `conformance.probe --needs-group stats` at M0); expect exit 5, pointer `foundation/stats/compiled`.
2. **791, absent.** In mode `remote` with `--referee-worker /nonexistent`, run `frob sim run g1 --tier referee
   --seconds 0.1 --json`; expect exit 5, pointer `referee/worker/absent`.
3. **791, receipt.** With a worker built under another receipt; expect exit 5, pointer `referee/worker/receipt`,
   both hashes in `observed` and `required`.
4. **890.** Produce episodes under two receipts and request a pooled comparison; expect exit 3, pointer
   `evidence/<index>/stack`.
5. Repeat steps 1 and 4 through `frob-mcp` over stdio where the server exists; expect the same envelope with
   `isError: true`.
The script sources the STD-E2E helper and writes `target/e2e-logs/suite_refusals/<utc>.ndjson` and a transcript.
Each provocation logs a `cmd` event (exact command, exit code, wall time) and a `refusal` event (code, pointer,
observed, required, retry, the repairs offered); an `assert` event checks the exit code against the registry row;
the `summary` counts codes provoked of codes registered (3 of 3) and names no gate.

### Known answers and negative controls

Known answers: exit codes 5, 5, 3; the pointer strings. Negative controls, one per code: for 790, the same verb
on a binary built with the group must succeed (the refusal is not unconditional); for 791, a worker built under
the matching receipt must be accepted, and a supervisor patched to warn and continue must be caught by the
assertion that zero job frames were sent; for 890, the pooled request restricted to one receipt must succeed with
identical numbers on a rerun, and a pooling function that ignores the stack identity must be caught. A control
on the registry: a row with a duplicate code or a reused retired code must fail to load. Lanes: both surfaces
(CLI and MCP); both machines.

## Considerations

- The exit-code reading for the `79x` range should be confirmed when the Tether registry of exit codes is
  reviewed; if exit 6 were chosen for 791 instead, only the registry row and one assertion change.
- The codes are deliberately few. Other boundary failures already have codes elsewhere: a capacity overflow of
  the referee's contact solver is `FROB-REF-721`; a verb not implemented in this build is `FROB-REF-724`; a job
  above the compute envelope is `FROB-REF-761`.
- Appendix-level registry and chapter must agree; the chapter governs meaning and repairs. The chapter's table
  gives name and condition only, so the payloads and ranked repairs above are DESIGN of this bead and of the
  three mechanism beads, and are the text that the registry will carry.

## References

Plan §33.16 (the refusal table), Appendix O (`7xx` and `8xx` registries), §35.11 (families and standing rules),
§32.4 (exit codes), §33.14 (`FROB-REF-791`), §33.9 (the gated bridge); GATE-TETHER-04.
