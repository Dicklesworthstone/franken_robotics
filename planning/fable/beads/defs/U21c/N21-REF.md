---
key: "N21-REF"
title: "Spark refusal registry: FROB-REF-601 to -618 with triggers, ranked repairs, exit code 6 and raising sites"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "frob-kernels", "frob-core", "solid", "registry", "test", "e2e", "review-notes-open"]
parent: "WP-35"
deps: ["BD-005", "N35-33", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-022", "N34p3-01", "BD-210", "N17a-09", "N17b-05", "N21a-02", "N21a-05", "N21c-39", "N32a-04", "N34-01", "VERBS-spark", "N21b-07", "N21b-11", "N21b-12", "N21b-16", "N21b-23", "N21b-24", "N21b-29", "N21b-36", "N21c-04", "N21c-05", "N21c-08", "N21c-12", "N21c-19", "N21c-20", "N21c-23", "N21c-28", "N21c-31", "N21c-42", "N35x-01", "N36-08", "N36-16", "N36-19", "N36-33", "N16c-26"]
gates: []
plan_refs: ["§21.21", "Appendix O.6", "§32.4", "§35.11", "§21.9", "§21.10", "§21.11", "§21.12", "§36.10", "§36.12"]
---
## Background

Every way in which the system declines to answer is a typed refusal: a stable code, a message, the field at fault,
observed and required values with their unit, and repairs ranked by how likely they are to help. Standing rules:
refuse, never truncate; a budget stop publishes nothing partial as a result; a misspelt parameter is a refusal, not
a default. Codes live in one machine-readable registry, `registries/refusals.toml`, loaded by `frob-core`; an
unknown code is a load-time error and a code is never reused. Families: `1xx` design and model, `2xx` numerics,
`3xx` scene, `4xx` control, `5xx` learning and evaluation protocol, **`6xx` backend and device**, `7xx` budget and
capability, `8xx` evidence and protocol, `9xx` internal.

Spark (GPU backends and the kernel toolchain) defines the `6xx` family: eighteen codes. "Each carries the
fingerprint digest and ranked repairs." This bead is their registry: full rows, the exit code, and the test that
every code is actually raised by its trigger. The mechanisms that raise them are other beads, named per row so
that no code is left without a raising site: a refusal that exists only as a table row is worse than none, because
agents plan around repairs the system never offers.

## Scope

In: the eighteen rows; the exit-code mapping; the `6xx` envelope additions; registry tests; one end-to-end script
that provokes each code; the three ledger rows without a printed number (a nineteenth `6xx` row and two internal
rows) and their registry rows under the assigned numbers. Out: detecting each condition (the raising beads).

## Technical approach

**Envelope (contract of `frob.envelope/1`).** With `ok: false` the envelope carries `refusal = { code, message,
pointer, observed, required, unit, affected_claims, retry, repairs[] }`; `retry` is `never`, `after-change` or
`backoff`; each repair is a typed suggestion (a verb with arguments) with a one-line reason. Every `6xx` refusal
adds `fingerprint: { device, digest | null }`. Exit code for the whole family: **6** (backend or device
unavailable). Exit 1 is never emitted on purpose.

**Rows.**

| Code | Condition | Trigger and `pointer` | Ranked repairs | `retry` | Raised by |
|---|---|---|---|---|---|
| `FROB-REF-601` | Device unavailable or lost | The requested device cannot be opened, or is lost during a run; `pointer`: the device. Also (DESIGN, the dispatch rule names only "the backend-unavailable family"): a device-tier `run` verb is dispatched and no `frobd` answers, so no process owns the device; `pointer`: `daemon` | 1. Rerun on another device. 2. Rerun on `batch-cpu` under `mixed-allowed`. 3. Resume from the last committed epoch when the device returns. For `pointer: daemon`, first: start `frobd` (the command is printed), then 2 | `backoff` | BD-175, BD-172, BD-174; the device open of the M0 harnesses (BD-014, N21b-32) and of the fingerprint verb `spark.fingerprint` (BD-029), and the backend registration at start (N34-01); device loss under the job protocol (BD-210); the dispatch route, step 7, for `pointer: daemon` (N32a-04, M0; the operator verbs of BD-022 run on it) |
| `FROB-REF-602` | The memory estimate exceeds the device budget at admission | The estimator's exact byte figure (the live-interval peak: the largest set of slabs alive at once) exceeds the budget, before anything is allocated. Five cases: the request's peak exceeds the device's budget; one shard of a two-card job exceeds its own card (each shard is admitted on its own); scene variants exceed the budget; camera frame buffers exceed the budget; the adjoint step-buffer store of K control intervals (K·d world-steps × bytes per world × worlds, d being the world-steps per control interval) exceeds the budget (merged into this row by N35-33's rule 1 from Kinesis's ledger row `adjoint-store-over-budget`). `pointer`: the slab class that exceeds (`worlds`, `scenes`, `frames`, `rollout`, `adjoint`); `observed` bytes, `required` budget | By the slab class of the pointer. `worlds` and `rollout` (both scale with the world count): 1. Fewer worlds (the largest count that fits is named). 2. A smaller capacity. 3. The Mac (the large-memory machine). `scenes`: 1. Fewer distinct scene variants. 2. A coarser scene (a different scene identity). 3. The Mac; never fewer worlds. `frames`: 1. Fewer worlds. 2. The Mac. 3. A different task with smaller cameras; never a lower resolution or cadence of the same task. `adjoint`: 1. Fewer worlds. 2. A shorter horizon K. 3. The paired stencil | `after-change` | BD-219, BD-173; per-shard admission N36-08; scene-bound requests N36-16; camera-bound requests N36-19; the adjoint store of step adjoints N16c-26 |
| `FROB-REF-603` | Kernel compile failure; compiler diagnostics verbatim | The device compiler rejects a kernel; `pointer`: the kernel; the diagnostics are attached unedited | 1. Report (the diagnostics and source hash). 2. The generic kernel, if a specialized one failed | `after-change` | M0: the annex compile paths BD-015 (NVRTC, test `compile_error_refuses_603`) and BD-016 (Metal), reached by `spark.check` (N21b-32) and `spark.probe` (BD-017); M2: BD-172, BD-174, BD-173, and `spark.precompile --unit` (N21b-13, N21a-10); N21c-19 for the specialized case |
| `FROB-REF-604` | Strict backend: a stage has no kernel on the requested backend | Raised by the planner under `strict:gpu-cuda` or `strict:gpu-metal`, before any world is stepped; `pointer`: the stage | 1. `mixed-allowed`. 2. The other backend | `after-change` | BD-178 |
| `FROB-REF-605` | The CPU-versus-device check is not current for a kernel in the plan | Under `--require-current-check`, a kernel lacks a current check for this device family, driver and compiler; `pointer`: the kernel | 1. `frob spark check` | `after-change` | BD-178, BD-176 |
| `FROB-REF-606` | Fingerprint missing, or changed since admission | No stored fingerprint for the device, or its identity differs from the live machine at admission, at a chunk commit or at emission; `pointer`: the device and the differing field | 1. `frob spark fingerprint`. 2. `frob spark check`. 3. `batch-cpu` | `after-change` | N21c-05 |
| `FROB-REF-607` | A launch or command buffer failed after bounded retries | A chunk's command buffer or launch ended in a state other than completed in each of at most four attempts with growing waits; every retry is counted in the receipt | 1. Resume from the last committed epoch. 2. Another device | `backoff` | N21b-07 (the commit fence: the bounded retry ladder and the refusal); reached through the annex capstones BD-174, BD-172, BD-175 |
| `FROB-REF-608` | Worlds marked `Faulted` exceed the caller's threshold | The count of worlds frozen by a non-finite sentinel exceeds the threshold given by the caller; `observed` count, `required` threshold | 1. Teleport a faulted world to `batch-cpu` to locate the step and stage. 2. Raise the threshold knowingly | `after-change` | BD-175, BD-204 |
| `FROB-REF-609` | A double-float canary failed | At device start-up the error-free sum of 1 and 2⁻³⁰ did not return (1, 2⁻³⁰), or the split product of 1 + 2⁻¹² with itself did not return (1 + 2⁻¹¹, 2⁻²⁴); the device is refused for double-float kernels | 1. Run without double-float kernels, labelled. 2. File the compiler evidence | `after-change` | The start-up canaries `K-canary` (N21b-36, M0); the precision battery (BD-179) re-runs them at M2 |
| `FROB-REF-610` | Capacity overflow under the `refuse` policy | A world exceeded a capacity (contacts, rows) and the caller's policy is `refuse` (the other policies mark the world `Degraded` and reset or continue flagged); `pointer`: the capacity | 1. A larger capacity. 2. `continue-flagged` | `after-change` | N21b-23 (device tiers: the policy surface `--on-overflow` and the raising function), BD-175, BD-183 |
| `FROB-REF-611` | A shape change was requested inside a running environment | A reset, admission or edit would change a capacity, a grid size or a layout of a resident environment | 1. A new bucket | `after-change` | BD-183, BD-172; at variant-slot admission on the device tiers (N21b-16); the slot table answers `ShapeChange` (N17a-09) and the admission controller raises 611 only when a request pinned to the running environment would change its shape (N17b-05) |
| `FROB-REF-612` | Kernel cache entry corrupt and recompilation unavailable | A cache hit fails its digest check (which alone is only a miss) and the host cannot recompile (offline, or no compiler) | 1. `frob spark precompile` on a build host, then copy the content-addressed directory | `after-change` | BD-173 |
| `FROB-REF-613` | A specialized kernel is not bit-identical to its generic kernel | The bit-identity fence found a differing state or a differing normal form, or no current identity row exists | 1. The generic kernel. 2. `frob spark check --specialized`. 3. Report the specializer defect | `after-change` | N21c-19 |
| `FROB-REF-614` | Host access to a slab outside its lease | A host read or write outside an epoch-fenced lease (a read lease exists only on the current committed slab, the snapshot slab and the previous ring slot; a write lease only between chunks) | 1. Read the committed or snapshot slab | `after-change` | N21b-11 (the epoch-fenced lease check and the refusal, with its `sim.run` debug seam); reached through BD-173 |
| `FROB-REF-615` | Device stalled (watchdog) | A chunk exceeded the upper end of its predicted time by a set factor; a watchdog on a second thread marks the device `Stalled` | 1. Restart the device worker (the supervisor resumes from the last committed epoch) | `backoff` | BD-175, BD-174 |
| `FROB-REF-616` | The device lacks an arithmetic feature the kernel declares (for example 64-bit integers) | No admissible variant of a kernel family for the device families in use; raised by the planner before any world is stepped | 1. The 32-bit variant. 2. Another backend. 3. `frob spark fingerprint` if the feature is `unavailable` | `after-change` | N21c-42 |
| `FROB-REF-617` | A view cannot cross a stride convention without a copy, under a zero-copy contract | A view with signed byte strides must be handed to a receiver with unsigned element strides (or the reverse) and cannot be represented there without materializing | 1. Allow the copy (it is charged to the receipt) | `after-change` | BD-036 (the typed stride converter) |
| `FROB-REF-618` | A transfer class that may not cross machines | The cross-machine bundle is asked to carry a compiled scene field, world state, a rollout buffer or an unregistered kind; a `TelemetrySummary` item above 16 MiB (16,777,216 bytes, DESIGN default) is a mislabelled trace and is treated as an unregistered kind, with `observed` its bytes. Raised at export and at import (`artifact.import` through the reader `XferReader`, nothing admitted) | 1. Send the seed or recipe. 2. Send address, design, weights and recipe and replay there. 3. Keep the work on the machine that holds the data | `after-change` | N21c-08 |

**Row 602 is written here and nowhere else.** The device table gives one first repair for it ("fewer worlds, a
smaller capacity, or this Mac"), which is right only when world state is what does not fit. The performance
program's memory arithmetic adds two cases in which that advice is wrong, and the row carries them (DESIGN, with
the reason for each): scenes are shared by every world of a device (a device holds K scene variants over one
brick pool and every world indexes one), so a `602` on `scenes` is repaired by fewer variants or a coarser scene
and never by fewer worlds; a camera's resolution and cadence are part of the task identity (changing the cadence
changes the control problem), so a `602` on `frames` never offers a lower resolution or cadence of the same
task. `rollout` takes the list of `worlds` because rollout storage is per world. The step-adjoint store of
Kinesis (N16c-26) is a fifth case, merged into this row by N35-33's rule 1 (a store over the device budget at
admission is 602's condition): it scales with the world count and with the horizon, so its list is fewer
worlds, then a shorter horizon K, then the paired stencil (N16-REF's repairs). The performance program's bead
N36-REF holds the raising-site matrix for this code, a test per trigger case and the two forbidden-repair
checks; it depends on this bead and reads the repair lists from this row.

**Spark's refusals without a printed number (allocation ledger rows).** Spark has three refusals without a number;
each is a row of `registries/refusal_allocations.toml` (slugs DESIGN) with `requested_by = "N21-REF"`, numbered by
N35-33's rule. The first two are rows of N35-33's first round, numbered by its run at M0 before any raising bead
starts (each raising bead depends on this registry, which depends on N35-33); the third is a later row this bead
appends to the ledger (a chapter registry appends its own later rows, N35-33), numbered by the same rule. This
bead enters each as a full registry row under its assigned number, and the raising beads cite it through one
named constant and never type the number:

| Ledger slug | Condition | Family; exit; retry | Payload and repairs | Raised by |
|---|---|---|---|---|
| `device-count-wrap` (`proposed = 619`) | The wide-count rule: "a device count could wrap within a chunk" | `6xx`; 6; `after-change` | `pointer` the counter; `observed` the largest number of increments one cell can receive in a chunk; `required` at most 4,294,967,295; `unit` increments per chunk; repairs: 1. a shorter chunk (the largest admissible length is printed); 2. fewer samples per interval; 3. fewer worlds per bucket | N21c-39 (M2), at plan time and at run admission, before any world is stepped; listed by `spark.place` (VERBS-spark) |
| `device-receipt-incomplete` (no proposal) | A device result fails its own schema: an evidence field is missing or a validation of the device-result check fails | `9xx`; 9; `never` ("never retry blindly") | `pointer` the missing field, or the number of the failing validation (AE-01 to AE-12); the dispatcher emits no result and the run's artifacts are kept for diagnosis; the raising beads state no caller repair (the defect is ours, not the caller's), so the row gives a `fewer_repairs_reason` | N21a-02 (M0) and N21c-28 (M2), both through the constant `DEVICE_RECEIPT_INCOMPLETE` |
| `xfer-item-digest-mismatch` (no proposal; `defined_in = "§21.17"`) | An item of a cross-machine bundle whose bytes do not hash to its manifest digest: an integrity fault, never a `6xx` code | `9xx`; 9; `never` | `pointer` the item id; `observed` and `required` the two digests; nothing is admitted; the raising bead states no caller repair (the bundle's bytes are corrupt), so the row gives a `fewer_repairs_reason` | N21c-08 (M2), the reader `XferReader` behind `artifact.import` |

619 is free in the registry as compiled, so rule 5 of N35-33 gives `device-count-wrap` its proposal unless an
earlier-ordered request claims it; the row and the constant carry whatever number the run gives. The two internal
codes are not `6xx` rows and are not counted among the eighteen; this bead enters them, with exit 9 and retry
`never`, because their raising beads are Spark's.

**Registry row format.** One TOML table per code: `code`, `name`, `family = "6xx"`, `message_template`, `exit = 6`,
`retry`, `repairs` (ordered), `defined_in = "§21.21"` (the plan subsection, by which N35-33 orders and checks rows),
`raised_by` (the bead keys of the "Raised by" column, the field N35-33's registry-wide check reads), `first_milestone` (601, 603, 606, 609, 617: M0; 602, 604,
605, 607, 608, 610, 611, 612, 614, 615, 616, 618 and `device-count-wrap`: M2; 613: M4). A row may exist before its mechanism; the verb
that would raise it then answers the capability refusal `FROB-REF-724` (not implemented in this build).

**Why this registry is an M0 bead (DESIGN).** Five codes have `first_milestone` M0 and are raised by M0 beads: 606
by fingerprint currency (N21c-05), 601 by the device open of the fingerprint verb and the annex harnesses (BD-029),
603 by the M0 compile path of the annexes, 609 by the start-up canaries (N21b-36) and 617 by the typed stride
converter (BD-036). An unknown code is a load-time error, so the rows must exist before the first of them is
raised; the registry is therefore written whole at M0, every raising bead depends on it, and its checks are staged
by `first_milestone`: a row whose milestone has not been reached is listed `pending (Mn)` and its absent raising
site is not a failure; a row whose milestone has been reached and whose raising test is missing, or has no
recorded run in the current build, fails the registry check. The end-to-end script is re-run at the exit review
of M0, M2 and M4.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the eighteen rows and the nineteenth `6xx` row `device-count-wrap` under its assigned number, schema-valid, each with ordered repairs, one `retry` value and exit 6.
- [ ] The three ledger rows `device-count-wrap`, `device-receipt-incomplete` and `xfer-item-digest-mismatch` exist with `requested_by = "N21-REF"`; under its assigned number the first is entered here as a full `6xx` row and the other two as internal rows with exit 9 and retry `never`; no raising bead uses a literal number.
- [ ] Rows 607 and 614 name their mechanism beads (N21b-07, N21b-11) in `raised_by` besides the capstones; row 618 carries the 16 MiB `TelemetrySummary` trigger.
- [ ] Row 601 carries the `pointer: daemon` trigger with `frobd` as its first repair.
- [ ] Row 602 carries its five trigger cases, the slab-class pointer (`adjoint` among the classes) and one ordered repair list per slab class, and lists N16c-26 among its raisers; the `scenes` list does not contain "fewer worlds" and the `frames` list contains no change of resolution or cadence; no other bead writes a second trigger or repair list for 602 (N36-REF reads this row).
- [ ] `cargo test -p frob-core` loads the registry; a duplicated code, an unknown code and a `6xx` row with an exit other than 6 are load-time errors.
- [ ] Each code whose `first_milestone` has been reached has a raising test in its raising bead asserting code, `pointer`, `observed`, `required`, `retry`, the repair order and the presence of the fingerprint block; the registry check fails for such a row whose raising test is missing or has not run, and lists later rows as `pending (Mn)`.
- [ ] The `CONTRACT.md` of `frob-kernels` and of each annex lists the codes they raise with the same wording, each from the milestone in which its raising site lands.
- [ ] At the M0 exit review the script provokes 601, 606, 609 and 617, logs 603 as `skipped (M2)` (its M0 raising test is BD-015's `compile_error_refuses_603`, which the registry check reads) and every other code as skipped with its `first_milestone`; at the M2 and M4 exit reviews it provokes every code due by then, 603 included.
- [ ] The end-to-end script provokes every code whose mechanism is present and gets exit 6 with the registered code; a code whose mechanism is absent is logged `skipped: capability absent`, never as passed.
- [ ] The generated refusal table and the served MCP tool list show the same refusal lists per verb.

## Test plan

### Unit tests

- `refusals_spark_rows_present`: eighteen codes 601 to 618, family `6xx`.
- `refusals_spark_exit_code_6`.
- `refusals_first_repairs`: 601 → another device; 602 → fewer worlds for the slab classes `worlds`, `rollout`, `frames` and `adjoint`, and fewer distinct scene variants for `scenes`; 604 → `mixed-allowed`; 605 → `spark check`; 606 → `spark fingerprint`; 607 → resume from the last committed epoch; 613 → the generic kernel; 618 → send the seed or recipe.
- `refusals_602_repairs_by_slab_class`: the row holds a list for each of `worlds`, `rollout`, `scenes`, `frames` and `adjoint`; "fewer worlds" is absent from `scenes`; no entry of `frames` changes a resolution or a cadence; the `adjoint` list is, in order, fewer worlds, a shorter horizon K, the paired stencil, and the row's `raised_by` holds N16c-26.
- `refusals_6xx_carry_fingerprint_block` (null digest allowed only for 606).
- `refusal_row_before_mechanism`: with specialization absent, the specialization option answers `FROB-REF-724`, not 613.
- `ledger_rows_entered_under_assigned_numbers`: each of the three Spark ledger rows is `assigned` and the registry holds a row of that number with its slug as `name`; a planted `6xx` registry row beyond the printed eighteen with no assigned ledger row fails (N35-33's check 4).
- `spark_ledger_rows_filed`: the ledger holds `device-count-wrap` (`6xx`, proposed 619), `device-receipt-incomplete` (`9xx`, exit 9, retry `never`) and `xfer-item-digest-mismatch` (`9xx`, exit 9, retry `never`, `defined_in = "§21.17"`), each with `requested_by = "N21-REF"`.
- `refusal_618_summary_size` (with N21c-08, M2): a `TelemetrySummary` item of 16,777,216 bytes is sent and one of 16,777,217 bytes is refused with 618, `observed` its bytes.
- `refusal_601_no_daemon` (the dispatch suite of N32a-04, M0): a device-tier `run` call with the server loop down → exit 6, `FROB-REF-601`, `pointer: daemon`, first repair naming `frobd`; a CPU-tier `run` call in the same state runs in process with the warning `executor: in-process`.
- `refusals_check_staged_by_milestone`: with the build at M0, a `synthetic` registry in which 606 has no raising test fails the check and one in which 613 has none passes it with 613 listed `pending (M4)`; at M4 the second fails.

### End-to-end script and logging

`scripts/e2e/spark_refusals.sh --backend gpu-cuda|gpu-metal`; sources the shared helper; logs newline-delimited
JSON to `target/e2e-logs/spark_refusals/<utc>.ndjson` with a transcript; the run records the milestone it is run
for, and a code whose `first_milestone` is later is logged `skipped` with that milestone. One step per code, each a `cmd` event
(exact command line, exit code, wall time, evidence block) and a `refusal` event (code, pointer, observed,
required, retry, repairs, fingerprint digest), then `assert` events:
1. 606: a device job on an empty state root. 2. 601: `--device cuda:7` (absent), and the device-loss injection.
3. 602: `frob sim run --workload H --worlds 2000000 --tier strict:gpu-cuda --key ref-602 --json` (about 194 GB by the byte model,
against a 24 GB card); under `--backend gpu-metal` the world count is taken above the largest that fits, because
194 GB is inside that machine's device budget of about 203 GB.
4. 603 (from M2): `frob spark precompile <device> --unit @fixture:bad-kernel --key ref-603 --json` (the
option `--unit` of N21a-10; the fixture is a dialect unit that passes the lint and that the device compiler
rejects; `--unit` is a path argument, so the `@fixture:` name resolves in the workspace `fixtures/` directory
through the M0 resolver of N34p3-01). No M0 verb compiles a kernel source the caller names, so at M0 the step is
logged `skipped (M2)` and 603 is asserted by the annex test `compile_error_refuses_603` (BD-015). 5. 604: `--withhold K-hess --tier strict:<b>`.
6. 605: `--require-current-check` with the check ledger emptied. 7. 607: five consecutive injected command-buffer
or launch faults. 8. 608: a planted non-finite world with `--max-faulted 0` (the option of `sim.run`). 9. 609: the fast-math mutant
build. 10. 610: a capacity-overflow fixture with `--on-overflow refuse` (the option of `sim.run`, N21b-23). 11. 611: admission of a variant of
another shape class into the running environment. 12. 612: a corrupted cache entry with `--offline`. 13. 613: the
reassociated specialized fixture. 14. 614: a debug-seam host read of the uncommitted slab. 15. 615: an injected
stall. 16. 616: a 64-bit-only kernel under `strict:gpu-metal`. 17. 617: a zero-copy hand-off across stride
conventions. 18. 618: export of a compiled scene field to the other host. 19. `device-count-wrap`: the boundary histogram
plan (its assigned code, read from the ledger, never typed). The final `summary` gives assertions passed and failed and steps skipped with reasons.

### Known answers and negative controls

Known values: 2,000,000 × 97 kB ≈ 194 GB against a 24 GB card (and inside the Mac's budget of about 203 GB, which holds about 2.1 M worlds of that workload, so the Metal step uses a larger count); four attempts before 607; the canary pairs of 609.
Negative controls: a planted registry row without repairs (the registry lint must fail); a raising site that
returns `ok: true` with a warning in place of 604 (step 5 must fail: a strict request never scores on a fallback);
a `6xx` refusal without the fingerprint block; a second exit code for one row. Counterexamples behind the codes:
CX-218 (617), CX-220 (609), CX-229 (614), CX-216 (the exact-kernel rule beside 616). Determinism: the script is run
twice on one machine and the two logs agree on every asserted field.

## Considerations

- Two families describe capacity overflow: 610 here (device-side, under the caller's `refuse` policy; exit 6) and
  `FROB-REF-722` of the batched-step chapter (N16-REF; exit 5), raised by BD-159 on `batch-cpu` and the other
  lockstep tiers. The raising site decides, from one function with one pointer and needed count (N21b-23's test
  `overflow_code_by_tier`): CUDA and Metal raise 610, `batch-cpu` raises 722; the registry rows cross-reference each
  other so that an agent sees one repair list.
- Several steps need fault seams that exist only in debug builds; the script says which build it ran and marks the
  others skipped.
- 613 and 618 depend on Frontier mechanisms; if their kill gates fire, 613 stays registered without a raising site
  and 618 stays in force (the seeds-only rule is Solid).

## References

Plan §21.21 (the eighteen rows), Appendix O (the `6xx` section), §32.4 (envelope and exit codes), §35.11 (families and standing rules), §36.10 and §36.12 (the scene and camera cases of 602).
