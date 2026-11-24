---
key: "N33-REF"
title: "registry: refusal codes of Suite consumption (FROB-REF-790, -791, -792, -890): triggers, repairs, exit codes"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "tether", "frob-core", "frob-verbs", "registry", "solid", "e2e"]
parent: "WP-06"
deps: ["BD-005", "BD-022", "N35-33", "N35x-02", "STD-E2E"]
needs: []
related: ["N33a-06", "N33a-11", "N33a-13", "N33c-10", "N33c-11", "N33c-13", "N33c-31", "N33c-32"]
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

In: the three registry rows of the chapter in full, and a fourth row added at integration (`FROB-REF-792`); the exit-code assignment; the verbs that may return each; the end-to-end
provocation of each code and one negative control per code (each provocation step runs once its raising bead has
landed). Out: the mechanisms that raise them (N33c-32 for 790; N33c-13 for 791; N33a-13 for 792; N33c-31 for 890,
at M1), which depend on this registry.

## Technical approach

**Row format (DESIGN).** One `[[refusal]]` table per code in `registries/refusals.toml`: `code`, `slug`,
`family`, `message` (one sentence, no jargon), `pointer_kind`, `payload` (named fields), `retry` (`never`,
`after-change`, `backoff`), `repairs` (ranked; each a verb to call next, a command, or a text instruction),
`exit`, `defined_in`. The envelope of every verb with `--json` carries, when `ok` is false, `refusal { code,
message, pointer, observed, required, unit, affected_claims, retry, repairs }`.

**The codes.**

| Code | Meaning | Trigger (exact) | Pointer and payload | Ranked repairs | Raised by | Retry | Exit |
|---|---|---|---|---|---|---|---|
| FROB-REF-790 `foundation-group-absent` | This binary was built without the airlock feature group the verb needs | At admission, before a job is planned or a budget is charged: the verb's `needs_groups` (from the verb registry, possibly argument-dependent) names a group that is (a) not compiled into this binary, or (b) compiled but without a passing admission record at this pin; in mode `remote`, a group missing from the worker's reported features. Also raised by the multiphysics bridge for a throughput-tier request, which the bridge never serves | `foundation/<group>/compiled`, `…/admitted` or `…/tier`; `observed`: the groups compiled (list); `required`: the missing group; the group's admission gate (INT-FSIM-01, -13, -14, -15, -16 or -17) when case (b) | 1. Use a build that has the group (`frob capabilities --json` lists this binary's groups). 2. Rebuild with `--features frob-fsim/<group>` (the message states the closure cost for `structure` and `multiphysics`). 3. For an unadmitted group, run its admission gate. 4. Use the documented fallback where one exists (a fluid film falls back to the Solid film-friction law; coupled islands have none) | Any verb with a non-empty `needs_groups`; `sim.run` with a coupled island; statistics, optimization, spectral, structure verbs as they arrive | `after-change` | 5 |
| FROB-REF-791 `referee-worker-mismatch` | The referee worker is absent, or is not the build the job names | Before any job frame is sent: (a) no worker executable or the spawn fails; (b) no handshake within the start timeout; (c) the worker's protocol major version differs; (d) the worker's stack receipt differs from the job's `stack_receipt`; (e) a development-build worker offered for gate-grade evidence. Also raised by the worker itself when a job's receipt differs from its own | `referee/worker/absent`, `…/no-handshake`, `…/protocol`, `…/receipt`, `…/development-build`; `observed`: the worker's receipt or protocol, or `none`; `required`: the expected value | 1. Rebuild the worker at the pinned commit (`scripts/build/worker.sh`). 2. Run `cargo xtask check-stack` and resolve the drift it reports. 3. If the worker is meant to stay on an older pin, set `worker_rev` in the stack lock and name that receipt in the job. 4. Use mode `inproc` if this binary has it | Any verb that reaches the referee in mode `remote`: `sim.run --tier referee`, `sim.replay --tier referee`, `fidelity.shadow`, the cloth oracle driver | `after-change` | 5 |
| FROB-REF-890 `stack-mismatch` | Evidence recorded under another stack receipt was offered as same-stack evidence | An operation that pools episodes, pairs two arms, serves a memoized result, promotes, or forms a performance ratio receives inputs whose stack identities differ (receipt, or referee stack) and the operation is not typed cross-stack; or a throughput-tier claim receives an input that crossed the multiphysics bridge or came from a worker at a different receipt | `evidence/<index>/stack`; `observed`: the offered receipt (and worker receipt); `required`: the current receipt; `affected_claims`: the claims of the request | 1. Revalidate the result under the current stack (`frob result revalidate <id>`). 2. Recompute. 3. Present the result as cross-stack evidence with both receipts shown (display and export only; not pooling or pairing) | `eval.compare`, `study.compare`, `eval.promote`, `eval.scorecard` over runs under two receipts (N33c-31's end-to-end case), the memo graph's serving path, performance verbs, `report.*` when asked to pool (from M1, N33c-31) | `after-change` | 3 |
| FROB-REF-792 `referee-upstream-refusal` | The referee refused for a reason that has no dedicated code | In an airlock adapter, when FrankenSim returns a typed refusal or error whose kind is neither capacity or fixed witness (`FROB-REF-721`) nor indeterminate contact (`FROB-REF-224`): an input outside the upstream model's stated domain, a numerical limit of an upstream routine, or any other upstream error. Raised by the refusal conversion (N33a-13) before any result is returned | `pointer`: `referee/<crate>/<variant>`; payload: `kind` (`outside-domain`, `owner-limit`, `other`), the upstream crate, variant and message verbatim (bounded to 2 KiB), the stack-lock digest, and `observed` and `required` where the upstream error carries numbers | 1. `outside-domain`: lower the model as the airlock's contract states, or fix the input named. 2. `owner-limit`: use the path this project owns for that quantity, which reports `unsupported`. 3. `other`: read the upstream message and record the case as an upstream work item with a reproducing fixture | every verb that reaches the referee through an airlock adapter (`sim.run --tier referee` at M0) | `after-change` for the first two kinds, `never` for `other`. The envelope has three retry classes only (`never`, `after-change`, `backoff`); the conversion's internal class for `owner-limit` is `RetryClass::NumericalOnly` (N33a-13: the caller's retry ladder may try another numerical plan with the same physical inputs, and must not weaken them), which is presented to an agent as `after-change` with the repair saying which path or plan to request; `outside-domain` is `AfterChange` and `other` is `Never` in both | 5 |

**The fourth row (DESIGN; number proposed at integration).** The airlock's first adapter rule requires every
FrankenSim refusal to become a typed referee refusal, and the plan assigns codes only for capacity and fixed
witnesses (`FROB-REF-721`) and for indeterminate contact (`FROB-REF-224`). The refusal conversion (N33a-13)
cannot close on an unregistered code, since an unknown code is a load-time error, so this registry adds
`FROB-REF-792` `referee-upstream-refusal`: the next free number of the `79x` range this chapter uses, with exit 5
like its neighbours (the referee cannot adjudicate the request; it is not a budget stop, a device fault or a
conflict). The number is filed in the allocation ledger `registries/refusal_allocations.toml` and assigned by its
second round (N35x-02, a dependency, which honours the free proposals 170, 171 and 792); if the ledger ever
reassigns it, only this row and one constant change.

**Exit codes.** `FROB-REF-890` exits **3** by the table's rule for the `8xx` family other than conflicts
(refused input or question). For `FROB-REF-790` and `FROB-REF-791` the table names exit 4 for the `70x` range
(budget) and exit 5 for the `72x` and `73x` ranges (capability not implemented, not admitted, or outside the
session token's scopes) and does not name `79x`. Both exit **5**, the exit the allocation ledger's rule fixes for
the `79x` decade (N35-33's `[exit_rule]`, a dependency; BD-005's loader requires the row to state it). Reason: each reports a capability this
installation does not have (a group not built or not admitted; a referee that is not present in the required
build); neither is a budget stop, a device fault (exit 6, the `6xx` family) or a conflict (exit 7). The row
states the exit explicitly, because every refusal code must map to exactly one exit code. On the MCP surface the
same envelope is the structured content of a tool result with `isError: true`.

**What none of the three may do.** None is retried with weakened inputs. None is downgraded to a warning: a
referee result from the wrong build, a pooled estimate across stacks, or an empty answer in place of a missing
group would each be a silent wrong answer. None returns a partial result.

**Refusals stated in words in this chapter's subsections, with no number here.** (1) A geometry generation whose
scale status is not metric is refused (the scan-import adapter); it is `FROB-REF-304` of the scene chapter's registry
(scale authority below what the task requires), raised with the pointer `scan-scale-not-metric`, and needs no new
number. (2) `train.start` refuses a sampler with no qualification receipt
(the learning chapter's `sampler-unqualified`, N23-REF, a first-round request of N35-33 with proposal 558). (3) A
request to use the reduced G1 profile `g1-reduced-30` as an import-parity reference is refused or relabelled
(the design chapter's import refusals: the code N14b-34 raises, N33c-41). This bead names them and their owners;
it assigns no code.

## Acceptance criteria
- [ ] `registries/refusals.toml` holds the four rows (790, 791, 792, 890) with every field of the table; the registry loads, and an unknown code is a load-time error.
- [ ] Each code is provoked end to end through the real `frob` binary and returns its code, pointer, observed and required values, repairs, and exit code (790: 5, 791: 5, 792: 5, 890: 3), once its raising bead has landed (N33c-32 for 790, N33c-13 for 791, N33a-13 for 792, N33c-31 for 890 at M1). Before that the script's step for the code is logged `not-run` with the raising bead's key and is never counted as a pass; this bead closes on the registry step and the steps whose raising beads exist, and the script is re-run when each remaining raising bead closes and at the M0 and M1 exit reviews.
- [ ] The generated refusal list of every verb named in "Raised by" contains the code; no verb returns one of the four without listing it. 790 is appended by the generator to every verb whose `needs_groups` is non-empty (N33c-32); 791 and 792 are listed by the rows of `sim.run`, `sim.replay` and `fidelity.shadow` (the referee forms), and 890 by the rows of `eval.compare`, `study.compare`, `eval.promote` and `eval.scorecard` from M1.
- [ ] Each code maps to exactly one exit code on both surfaces; the MCP result carries the same envelope with `isError: true`.
- [ ] For each code the "must not" holds under test: no retry with altered inputs, no warning-and-continue, no partial result.
- [ ] The three unnumbered refusals are filed with the owning registries.
- [ ] The fourth row `FROB-REF-792` loads; it is provoked end to end by an input outside the upstream model's stated domain through `sim.run --tier referee`, carries the upstream variant and message verbatim, and exits 5.

## Test plan

### Unit tests

`cargo test -p frob-core refusal_registry_suite` and `cargo test -p frob-verbs refusals_suite`:
- `rows_790_791_792_890_load`: fields non-empty; `exit` is 5, 5, 5, 3 in that order; `retry` is `after-change` for 790, 791 and 890, and for 792 depends on `kind` (`after-change` for `outside-domain` and `owner-limit`, `never` for `other`).
- `owner_limit_is_numerical_only_presented_after_change`: the row records, for `owner-limit`, the internal class `NumericalOnly` of N33a-13 and the envelope class `after-change`; a converted `OwnerLimit` refusal renders `retry: after-change` with "use the path this project owns" as its first repair, and a row or envelope that prints `numerical-only` as a retry class fails (it is not one of the three classes).
- `row_792_kinds_closed`: the `kind` payload of 792 accepts exactly `outside-domain`, `owner-limit`, `other`; an upstream message longer than 2 KiB is cut at 2 KiB with the cut stated.
- `repairs_are_ranked_and_actionable`: each repair is a verb, a command or a text instruction; verbs named exist in the verb registry.
- `pointer_kinds_enumerated`: the pointer strings of each code are exactly the listed cases.
- `one_exit_code_per_refusal`: no code of this set appears with two exits.
- `verb_refusal_lists_contain_codes`.
- `envelope_shape`: a refusal envelope for each code validates against the envelope schema, with `observed` and `required` present.

### End-to-end script and logging

`scripts/e2e/suite_refusals.sh` (this bead writes the script and its registry step; each provocation step is
enabled by its raising bead, and a step whose raising bead has not landed is logged `not-run: <bead> open`):
0. **Registry.** Load `registries/refusals.toml` through `frob` (any verb's start-up loads it; an unknown code
   must abort the load), and assert that the four rows are present with the exit codes 5, 5, 5, 3 and that the
   generated refusal list of each verb named in "Raised by" contains its code.
1. **790** (N33c-32). With the default binary, ask for an off-by-default group: at M0
   `frob capabilities --require-group stats --json`, the option N33c-32 defines on the `capabilities` row (its
   `needs_groups` comes from the argument); expect exit 5, pointer `foundation/stats/compiled`.
2. **791, absent** (N33c-13). In mode `remote` with `FROB_FSIM_WORKER=/nonexistent` in the environment (the
   worker-path order of N33c-13; no verb option is needed), run `frob sim run g1 --tier referee --seconds 0.1
   --key sr-791a --json`; expect exit 5, pointer `referee/worker/absent`.
3. **791, receipt** (N33c-13). With `FROB_FSIM_WORKER` naming a worker built under another receipt, the same
   command with `--key sr-791b`; expect exit 5, pointer `referee/worker/receipt`, both hashes in `observed` and
   `required`, and zero job frames sent.
4. **792** (N33a-13). Run `frob sim run <synthetic model> --tier referee --seconds 0.1 --key sr-792 --json` on
   a `synthetic` model with an input outside the upstream model's stated domain; expect exit 5, pointer
   `referee/<crate>/<variant>`, `kind` `outside-domain`, and the upstream crate, variant and message verbatim.
5. **890** (N33c-31, M1). Following N33c-31's procedure: `frob eval run <design> --battery smoke --tier referee
   --key sr-890a --json` with the default binary, then the same command with `--key sr-890b` with a binary built
   under a second verified receipt; then the pooled read over both runs that N33c-31 names, the several-runs form
   of `eval.scorecard` (VERBS-eval): `frob eval scorecard <run of sr-890a> <run of sr-890b> --json`; expect exit
   3, pointer `evidence/<index>/stack`, both receipts in `observed` and `required`. Control:
   the scorecard of `sr-890a` alone succeeds with identical numbers on a rerun.
6. Repeat steps 1 and 5 through `frob-mcp` over stdio where the server exists; expect the same envelope with
   `isError: true`.
The script sources the STD-E2E helper and writes `target/e2e-logs/suite_refusals/<utc>.ndjson` and a transcript.
Each provocation logs a `cmd` event (exact command, exit code, wall time, the envelope's `evidence` block) and a
`refusal` event (code, pointer, observed, required, unit, retry, the repairs offered); an `assert` event checks
the exit code and the pointer against the registry row; on a mismatch the saved envelope's path. The `summary`
counts codes provoked of codes registered (4 of 4 when every raising bead has landed), lists the `not-run` steps
with their beads, and names no gate.

### Known answers and negative controls

Known answers: exit codes 5, 5, 5, 3 (790, 791, 792, 890); the pointer strings. Negative controls, one per code: for 790, the same verb
on a binary built with the group must succeed (the refusal is not unconditional); for 791, a worker built under
the matching receipt must be accepted, and a supervisor patched to warn and continue must be caught by the
assertion that zero job frames were sent; for 890, the pooled request restricted to one receipt must succeed with
identical numbers on a rerun, and a pooling function that ignores the stack identity must be caught; for 792, the
same model with an input inside the upstream domain must run, and a conversion that maps the upstream error to an
empty result or to a generic `9xx` code must be caught by the assertion on the code. A control
on the registry: a row with a duplicate code or a reused retired code must fail to load. Lanes: both surfaces
(CLI and MCP); both machines.

## Considerations

- The exit 5 of the `79x` decade is N35-33's exit rule (its own DESIGN, stated once there); if that rule changed
  for 791 (to exit 6, say), only the registry row and one assertion here would change.
- The codes are deliberately few. Other boundary failures already have codes elsewhere: a capacity overflow of
  the referee's contact solver is `FROB-REF-721`; a verb not implemented in this build is `FROB-REF-724`; a job
  above the compute envelope is `FROB-REF-761`.
- Appendix-level registry and chapter must agree; the chapter governs meaning and repairs. The chapter's table
  gives name and condition only, so the payloads and ranked repairs above are DESIGN of this bead and of the
  three mechanism beads, and are the text that the registry will carry.

## References

Plan §33.16 (the refusal table), Appendix O (`7xx` and `8xx` registries), §35.11 (families and standing rules),
§32.4 (exit codes), §33.14 (`FROB-REF-791`), §33.9 (the gated bridge); GATE-TETHER-04.
