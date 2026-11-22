---
key: "N21-REF"
title: "Spark refusal registry: FROB-REF-601 to -618 with triggers, ranked repairs, exit code 6 and raising sites"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "spark", "frob-kernels", "frob-core", "solid", "registry", "test", "e2e"]
parent: "WP-46"
deps: ["BD-005", "BD-022", "N21c-05", "STD-TEST", "STD-E2E"]
needs: ["§21.5 to §21.15: the raising sites of 601 to 605, 607 to 612, 614, 615 and 617, owned by the backend, arena, planner, check and fault beads of the other two Spark drafting units (seeds named per row)", "Appendix O: assignment of a code for \"a device count could wrap within a chunk\", which the plan states as a refusal without a number (proposed here as `FROB-REF-619`)"]
related: ["N21c-08", "N21c-19", "N21c-39", "N21c-42", "N21c-28", "N21c-31", "N21-CX", "BD-172", "BD-173", "BD-174", "BD-175", "BD-176", "BD-178", "BD-219", "BD-036", "BD-183", "BD-204"]
gates: []
plan_refs: ["§21.21", "Appendix O.6", "§32.4", "§35.11", "§21.9", "§21.10", "§21.11", "§21.12"]
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
that provokes each code; the proposed nineteenth row. Out: detecting each condition (the raising beads).

## Technical approach

**Envelope (contract of `frob.envelope/1`).** With `ok: false` the envelope carries `refusal = { code, message,
pointer, observed, required, unit, affected_claims, retry, repairs[] }`; `retry` is `never`, `after-change` or
`backoff`; each repair is a typed suggestion (a verb with arguments) with a one-line reason. Every `6xx` refusal
adds `fingerprint: { device, digest | null }`. Exit code for the whole family: **6** (backend or device
unavailable). Exit 1 is never emitted on purpose.

**Rows.**

| Code | Condition | Trigger and `pointer` | Ranked repairs | `retry` | Raised by |
|---|---|---|---|---|---|
| `FROB-REF-601` | Device unavailable or lost | The requested device cannot be opened, or is lost during a run; `pointer`: the device | 1. Rerun on another device. 2. Rerun on `batch-cpu` under `mixed-allowed`. 3. Resume from the last committed epoch when the device returns | `backoff` | BD-175, BD-172, BD-174 |
| `FROB-REF-602` | The memory estimate exceeds the device budget at admission | The estimator's exact byte figure exceeds the budget, before anything is allocated; `observed` bytes, `required` budget | 1. Fewer worlds. 2. A smaller capacity. 3. The Mac (the large-memory machine) | `after-change` | BD-219, BD-173 |
| `FROB-REF-603` | Kernel compile failure; compiler diagnostics verbatim | The device compiler rejects a kernel; `pointer`: the kernel; the diagnostics are attached unedited | 1. Report (the diagnostics and source hash). 2. The generic kernel, if a specialized one failed | `after-change` | BD-172, BD-174, BD-173; N21c-19 for the specialized case |
| `FROB-REF-604` | Strict backend: a stage has no kernel on the requested backend | Raised by the planner under `strict:gpu-cuda` or `strict:gpu-metal`, before any world is stepped; `pointer`: the stage | 1. `mixed-allowed`. 2. The other backend | `after-change` | BD-178 |
| `FROB-REF-605` | The CPU-versus-device check is not current for a kernel in the plan | Under `--require-current-check`, a kernel lacks a current check for this device family, driver and compiler; `pointer`: the kernel | 1. `frob spark check` | `after-change` | BD-178, BD-176 |
| `FROB-REF-606` | Fingerprint missing, or changed since admission | No stored fingerprint for the device, or its identity differs from the live machine at admission, at a chunk commit or at emission; `pointer`: the device and the differing field | 1. `frob spark fingerprint`. 2. `frob spark check`. 3. `batch-cpu` | `after-change` | N21c-05 |
| `FROB-REF-607` | A launch or command buffer failed after bounded retries | A chunk's command buffer or launch ended in a state other than completed in each of at most four attempts with growing waits; every retry is counted in the receipt | 1. Resume from the last committed epoch. 2. Another device | `backoff` | BD-174, BD-172, BD-175 |
| `FROB-REF-608` | Worlds marked `Faulted` exceed the caller's threshold | The count of worlds frozen by a non-finite sentinel exceeds the threshold given by the caller; `observed` count, `required` threshold | 1. Teleport a faulted world to `batch-cpu` to locate the step and stage. 2. Raise the threshold knowingly | `after-change` | BD-175, BD-204 |
| `FROB-REF-609` | A double-float canary failed | At device start-up the error-free sum of 1 and 2⁻³⁰ did not return (1, 2⁻³⁰), or the split product of 1 + 2⁻¹² with itself did not return (1 + 2⁻¹¹, 2⁻²⁴); the device is refused for double-float kernels | 1. Run without double-float kernels, labelled. 2. File the compiler evidence | `after-change` | The canary kernels of the precision beads (BD-179) |
| `FROB-REF-610` | Capacity overflow under the `refuse` policy | A world exceeded a capacity (contacts, rows) and the caller's policy is `refuse` (the other policies mark the world `Degraded` and reset or continue flagged); `pointer`: the capacity | 1. A larger capacity. 2. `continue-flagged` | `after-change` | BD-175, BD-183 |
| `FROB-REF-611` | A shape change was requested inside a running environment | A reset, admission or edit would change a capacity, a grid size or a layout of a resident environment | 1. A new bucket | `after-change` | BD-183, BD-172 |
| `FROB-REF-612` | Kernel cache entry corrupt and recompilation unavailable | A cache hit fails its digest check (which alone is only a miss) and the host cannot recompile (offline, or no compiler) | 1. `frob spark precompile` on a build host, then copy the content-addressed directory | `after-change` | BD-173 |
| `FROB-REF-613` | A specialized kernel is not bit-identical to its generic kernel | The bit-identity fence found a differing state or a differing normal form, or no current identity row exists | 1. The generic kernel. 2. `frob spark check --specialized`. 3. Report the specializer defect | `after-change` | N21c-19 |
| `FROB-REF-614` | Host access to a slab outside its lease | A host read or write outside an epoch-fenced lease (a read lease exists only on the current committed slab, the snapshot slab and the previous ring slot; a write lease only between chunks) | 1. Read the committed or snapshot slab | `after-change` | BD-173 |
| `FROB-REF-615` | Device stalled (watchdog) | A chunk exceeded the upper end of its predicted time by a set factor; a watchdog on a second thread marks the device `Stalled` | 1. Restart the device worker (the supervisor resumes from the last committed epoch) | `backoff` | BD-175, BD-174 |
| `FROB-REF-616` | The device lacks an arithmetic feature the kernel declares (for example 64-bit integers) | No admissible variant of a kernel family for the device families in use; raised by the planner before any world is stepped | 1. The 32-bit variant. 2. Another backend. 3. `frob spark fingerprint` if the feature is `unavailable` | `after-change` | N21c-42 |
| `FROB-REF-617` | A view cannot cross a stride convention without a copy, under a zero-copy contract | A view with signed byte strides must be handed to a receiver with unsigned element strides (or the reverse) and cannot be represented there without materializing | 1. Allow the copy (it is charged to the receipt) | `after-change` | BD-036 (the typed stride converter) |
| `FROB-REF-618` | A transfer class that may not cross machines | The cross-machine bundle is asked to carry a compiled scene field, world state, a rollout buffer or an unregistered kind | 1. Send the seed or recipe. 2. Send address, design, weights and recipe and replay there. 3. Keep the work on the machine that holds the data | `after-change` | N21c-08 |

**Proposed addition (DESIGN; open item for the registry).** The wide-count rule states a refusal without a number:
"a device count could wrap within a chunk". Proposed row: `FROB-REF-619`, family `6xx`, exit 6, `observed`: the
largest number of increments one cell can receive in a chunk, `required`: at most 4,294,967,295; repairs: 1. a
shorter chunk (the largest admissible length is printed); 2. fewer samples per interval; 3. fewer worlds per
bucket. Raised by N21c-39. The number 619 is free in the registry as compiled; if the registry's owners choose
another, only this row changes.

**Registry row format.** One TOML table per code: `code`, `name`, `family = "6xx"`, `message_template`, `exit = 6`,
`retry`, `repairs` (ordered), `defined_in = "spark"`, `first_milestone` (601, 603, 606, 609, 617: M0; 602, 604,
605, 607, 608, 610, 611, 612, 614, 615, 616, 618, 619: M2; 613: M4). A row may exist before its mechanism; the verb
that would raise it then answers the capability refusal `FROB-REF-724` (not implemented in this build).

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the eighteen rows (and the proposed nineteenth, marked `proposed` until assigned), schema-valid, each with ordered repairs, one `retry` value and exit 6.
- [ ] `cargo test -p frob-core` loads the registry; a duplicated code, an unknown code and a `6xx` row with an exit other than 6 are load-time errors.
- [ ] Each code has a raising test in its raising bead asserting code, `pointer`, `observed`, `required`, `retry`, the repair order and the presence of the fingerprint block.
- [ ] The `CONTRACT.md` of `frob-kernels` and of each annex lists the codes they raise with the same wording.
- [ ] The end-to-end script provokes every code whose mechanism is present and gets exit 6 with the registered code; a code whose mechanism is absent is logged `skipped: capability absent`, never as passed.
- [ ] The generated refusal table and the served MCP tool list show the same refusal lists per verb.

## Test plan

### Unit tests

- `refusals_spark_rows_present`: eighteen codes 601 to 618, family `6xx`.
- `refusals_spark_exit_code_6`.
- `refusals_first_repairs`: 601 → another device; 602 → fewer worlds; 604 → `mixed-allowed`; 605 → `spark check`; 606 → `spark fingerprint`; 607 → resume from the last committed epoch; 613 → the generic kernel; 618 → send the seed or recipe.
- `refusals_6xx_carry_fingerprint_block` (null digest allowed only for 606).
- `refusal_row_before_mechanism`: with specialization absent, the specialization option answers `FROB-REF-724`, not 613.
- `proposed_row_not_loadable_as_assigned`: a `proposed` row cannot be raised until its state is `assigned`.

### End-to-end script and logging

`scripts/e2e/spark_refusals.sh --backend gpu-cuda|gpu-metal`; sources the shared helper; logs newline-delimited
JSON to `target/e2e-logs/spark_refusals/<utc>.ndjson` with a transcript. One step per code, each a `cmd` event
(exact command line, exit code, wall time, evidence block) and a `refusal` event (code, pointer, observed,
required, retry, repairs, fingerprint digest), then `assert` events:
1. 606: a device job on an empty state root. 2. 601: `--device cuda:7` (absent), and the device-loss injection.
3. 602: `frob sim run --workload H --worlds 2000000 --tier strict:<b> --json` (about 194 GB by the byte model).
4. 603: `frob spark precompile @fixture:bad-kernel --json`. 5. 604: `--withhold K-hess --tier strict:<b>`.
6. 605: `--require-current-check` with the check ledger emptied. 7. 607: five consecutive injected command-buffer
or launch faults. 8. 608: a planted non-finite world with `--faulted-threshold 0`. 9. 609: the fast-math mutant
build. 10. 610: a capacity-overflow fixture with `--capacity-policy refuse`. 11. 611: admission of a variant of
another shape class into the running environment. 12. 612: a corrupted cache entry with `--offline`. 13. 613: the
reassociated specialized fixture. 14. 614: a debug-seam host read of the uncommitted slab. 15. 615: an injected
stall. 16. 616: a 64-bit-only kernel under `strict:gpu-metal`. 17. 617: a zero-copy hand-off across stride
conventions. 18. 618: export of a compiled scene field to the other host. 19. the boundary histogram plan (the
proposed code). The final `summary` gives assertions passed and failed and steps skipped with reasons.

### Known answers and negative controls

Known values: 2,000,000 × 97 kB ≈ 194 GB against a 24 GB card; four attempts before 607; the canary pairs of 609.
Negative controls: a planted registry row without repairs (the registry lint must fail); a raising site that
returns `ok: true` with a warning in place of 604 (step 5 must fail: a strict request never scores on a fallback);
a `6xx` refusal without the fingerprint block; a second exit code for one row. Counterexamples behind the codes:
CX-218 (617), CX-220 (609), CX-229 (614), CX-216 (the exact-kernel rule beside 616). Determinism: the script is run
twice on one machine and the two logs agree on every asserted field.

## Considerations

- Two families describe capacity overflow: 610 here (device-side, under the caller's `refuse` policy) and a `7xx`
  code of the dynamics chapter for a world that exceeds its contact or row capacity when the caller chose refuse.
  The raising site decides: the batched device path raises 610; the registry rows cross-reference each other so
  that an agent sees one repair list.
- Several steps need fault seams that exist only in debug builds; the script says which build it ran and marks the
  others skipped.
- 613 and 618 depend on Frontier mechanisms; if their kill gates fire, 613 stays registered without a raising site
  and 618 stays in force (the seeds-only rule is Solid).

## References

Plan §21.21 (the eighteen rows), Appendix O (the `6xx` section), §32.4 (envelope and exit codes), §35.11 (families and standing rules).
