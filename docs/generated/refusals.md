<!-- @generated from registries/refusals.toml, registries/refusal_allocations.toml by xtask::generate::refusal_printout (N35-33); do not edit by hand. -->

# FrankenRobotics Refusal Code Registry

Generated from `registries/refusals.toml` (`frob.refusals/1`) and `registries/refusal_allocations.toml` (`frob.refalloc/1`).
Sorted by family (`1xx`..`9xx`), then numerically; codes are compared as whole tokens (`FROB-REF-[1-9][0-9]{2,3}`).

## Family Capacity Table

| Family | Scope | Default Exit | Printed (Plan) | Live Rows | Open Requests | Status |
|---|---|---:|---:|---:|---:|---|
| `1xx` | Design and model | 3 | 42 | 0 | 0 | ok (55 free 3-digit) |
| `2xx` | Numerics and integrator admission | 3 | 26 | 6 | 0 | ok (72 free 3-digit) |
| `3xx` | Scene | 3 | 22 | 0 | 0 | ok (77 free 3-digit) |
| `4xx` | Control and estimation | 3 | 17 | 0 | 0 | ok (82 free 3-digit) |
| `5xx` | Learning and evaluation protocol | 3 | 56 | 2 | 0 | ok (20 free 3-digit) |
| `6xx` | Backend and device | 6 | 18 | 18 | 0 | ok (80 free 3-digit) |
| `7xx` | Budget and capability | 4 | 19 | 11 | 0 | ok (78 free 3-digit) |
| `8xx` | Evidence and protocol | 3 | 94 | 22 | 8 | over capacity (4 free 3-digit; continues at FROB-REF-8100 per ADR-NUM-refusal-8xx) |
| `9xx` | Internal | 9 | 0 | 1 | 0 | ok (97 free 3-digit) |

## Family `1xx` — Design and model

_No rows registered yet at current milestone._

## Family `2xx` — Numerics and integrator admission

| Code | Meaning | Defined in | Exit |
|---|---|---|---:|
| `FROB-REF-247` | Determinism class not attainable on requested placement (e.g. bit-equality replay on a D3 device family, or fork on a family without a current batch-independence record); admission-time check (distinct from snapshot restore across profiles FROB-REF-894); pointer verb and placement; observed attained class and record; required class and use | §35.11 | 3 |
| `FROB-REF-248` | Solve ill-conditioned beyond precision profile (kappa_hat * u above frozen OPEN-NUM-4 limit, default 0.1) under refuse policy; run-time measurement (distinct from admission-time interface-card estimate FROB-REF-244); pointer world address and step; observed kappa_hat, lower bound kappa_hat * u, and precision profile; required kappa_hat * u <= limit | §35.11 | 3 |
| `FROB-REF-249` | Anchored offset left its range in one step (magnitude >= 16 m after integration) or a cell index left scene bounds; pointer world address, body and axis; observed offset or cell index; required magnitude below 16 m and index inside declared bounds | §35.11 | 3 |
| `FROB-REF-250` | Kernel is not dimensionally homogeneous (build time): sum of terms with different base-dimension exponent vectors or square root of an odd exponent vector (theorem card T-09); pointer kernel, statement and both exponent vectors; required equal exponent vectors | §35.11 | 3 |
| `FROB-REF-251` | A banned operation appears in kernel source (build time): a fast-math relaxation, a fused multiply-add (contracted or explicit), a built-in transcendental or reciprocal square root, a floating-point atomic; pointer file, line and token; observed ban category | §35.11 | 3 |
| `FROB-REF-252` | Strict predicate in floating point without an exact form or certified guard (build time); pointer kernel and comparison; observed predicate class; required exact or guarded | §35.11 | 3 |

## Family `3xx` — Scene

_No rows registered yet at current milestone._

## Family `4xx` — Control and estimation

_No rows registered yet at current milestone._

## Family `5xx` — Learning and evaluation protocol

| Code | Meaning | Defined in | Exit |
|---|---|---|---:|
| `FROB-REF-561` | Paired statistic requested with a D4 or unrecorded arm, or paired across device families without D1, at the numerics licence check (licence::check, N35-07). On eval.compare, pairing::admit first raises FROB-REF-542 for a D4 arm, different classes, or a D3 pair lacking decision equivalence; 561 covers an unrecorded class or an exact pairing the placement cannot license. On eval.detect, 561 covers a D4 or unrecorded arm. On eval.promote, 561 covers a fast-tier result and its re-evaluation paired across device families without D1, or arms that cannot be paired. On study.compare and eval.promote the stored-result evidence boundary raises FROB-REF-825 for D4 or unrecorded evidence; Arena study::admit raises FROB-REF-533. Pointer arm; observed class/profile or both device families; required both arms D2 on one family with current batch-independence record, D1 across families, or D3 with decision-equivalence record | §35.11 | 3 |
| `FROB-REF-562` | Attribution or counterfactual fork requested below class D2 (source run class D3, D4 or unrecorded, or device family batch-independence record not current) at fork or re-run admission (distinct from attribution-time fork-set failure FROB-REF-584); pointer source run; observed class and record age; required D2 with GATE-SPARK-15 current for the family | §35.11 | 3 |

## Family `6xx` — Backend and device

| Code | Meaning | Defined in | Exit |
|---|---|---|---:|
| `FROB-REF-601` | Device unavailable or lost: the requested device cannot be opened or is lost during a run (pointer: device); also when a device-tier run verb is dispatched with no frobd answering so no process owns the device (pointer: daemon; first repair: start frobd); carries fingerprint { device, digest \| null } (due M0) | §21.21 | 6 |
| `FROB-REF-602` | The memory estimate (live-interval peak bytes) exceeds the device budget at admission before any allocation; five trigger cases: (1) request peak exceeds card budget, (2) one shard of a two-card job exceeds its own card, (3) scene variants exceed budget (pointer: scenes; never fewer worlds), (4) camera frame buffers exceed budget (pointer: frames; never lower resolution or cadence of the same task), (5) adjoint step-buffer store K*d world-steps exceeds budget (pointer: adjoint; merged from adjoint-store-over-budget); pointer slab class in {worlds, rollout, scenes, frames, adjoint}; observed bytes, required budget; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-603` | Kernel compile failure; device compiler rejects a kernel with verbatim compiler diagnostics attached unedited; pointer: kernel; carries fingerprint { device, digest \| null } (due M0) | §21.21 | 6 |
| `FROB-REF-604` | Strict backend: a stage has no kernel on the requested backend; raised by the planner under strict:gpu-cuda or strict:gpu-metal before any world is stepped; pointer: stage; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-605` | The CPU-versus-device check is not current for a kernel in the step plan on this device family, driver, and compiler under --require-current-check (run-time refusal, distinct from citation refusal FROB-REF-864); pointer: kernel; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-606` | Fingerprint missing or changed since admission: no stored fingerprint for the device or its identity differs from the live machine at admission, chunk commit, or emission; pointer: device and differing field; carries fingerprint { device, digest \| null } (null digest allowed here only) (due M0) | §21.21 | 6 |
| `FROB-REF-607` | A launch or command buffer failed after bounded retries (at most four attempts with growing waits; every retry counted in the receipt); pointer: launch or command buffer; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-608` | Worlds marked Faulted (frozen by a non-finite sentinel) exceed the caller's threshold; observed count, required threshold; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-609` | A double-float start-up canary failed: TwoSum(1, 2^-30) != (1, 2^-30) or split product (1 + 2^-12)^2 != (1 + 2^-11, 2^-24); device refused for double-float kernels; pointer: canary; carries fingerprint { device, digest \| null } (due M0) | §21.21 | 6 |
| `FROB-REF-610` | Capacity overflow on a device tier under the caller's refuse policy (distinct from lockstep/batch-cpu overflow FROB-REF-722); pointer: capacity; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-611` | A shape change (capacity, grid size, or layout) was requested inside a running resident environment; pointer: environment shape; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-612` | Kernel cache entry corrupt (digest check failed) and recompilation unavailable (offline or no device compiler); pointer: cache entry; carries fingerprint { device, digest \| null } (due M0/M2) | §21.21 | 6 |
| `FROB-REF-613` | A specialized kernel is not bit-identical to its generic kernel (bit-identity fence found a differing state or normal form, or no current identity row exists); pointer: specialized kernel; carries fingerprint { device, digest \| null } (due M4) | §21.21 | 6 |
| `FROB-REF-614` | Host access to a device slab outside its epoch-fenced lease (read lease only on current committed slab, snapshot slab, and previous ring slot; write lease only between chunks); pointer: slab; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-615` | Device stalled (watchdog): a chunk exceeded the upper end of its predicted time by the configured factor and the watchdog thread marked the device Stalled; pointer: device; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-616` | The device lacks an arithmetic feature the kernel declares (e.g. 64-bit integers): no admissible variant of the kernel family exists for the active device families; raised by the planner before any world is stepped; pointer: kernel feature; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |
| `FROB-REF-617` | A view cannot cross a stride convention (signed byte strides vs unsigned element strides) without materializing a copy under a zero-copy contract (CX-218); pointer: view stride convention; carries fingerprint { device, digest \| null } (due M0) | §21.21 | 6 |
| `FROB-REF-618` | A transfer class that may not cross machines (compiled scene field, world state, rollout buffer, unregistered kind, or a TelemetrySummary item above 16 MiB / 16,777,216 bytes treated as a mislabelled trace with observed bytes); raised at export and at artifact.import through XferReader with nothing admitted; pointer: transfer item; carries fingerprint { device, digest \| null } (due M2) | §21.21 | 6 |

## Family `7xx` — Budget and capability

| Code | Meaning | Defined in | Exit |
|---|---|---|---:|
| `FROB-REF-701` | The estimate's upper end exceeds the remaining session budget at admission (pointer /budget/<unit>; observed upper estimate, required remaining budget; due M2) | §32.18 | 4 |
| `FROB-REF-702` | Budget exhausted during a run at a kernel-batch boundary; unpublished state is discarded, nothing partial is published as a result, and the acknowledgement names the last committed tick per world (pointer /budget/<unit>; due M1) | §32.18 | 4 |
| `FROB-REF-703` | Admission guard triggered: device utilization >= 0.8 or shape-class queue already holds 64 jobs; carries poll_after_ms (pointer /queue; due M3) | §32.18 | 4 |
| `FROB-REF-704` | A campaign-class job has no approved compute envelope for its scope (launcher token, cited envelope_scope, verb-derived scope, or adhoc), raised at daemon submission and at dispatch or resume on lapsed approval (pointer /envelope_scope; due M3) | §32.18 | 4 |
| `FROB-REF-720` | The session token lacks a capability scope required by the verb's registry entry, checked before the handler runs so no partial data exists (pointer /session/scopes; due M2) | §32.18 | 5 |
| `FROB-REF-723` | Session token invalid, expired, closed, unverifiable without a reachable daemon, missing on a non-public daemon call, or widening attenuation beyond parent or root grant (pointer /session/token; due M2) | §32.18 | 5 |
| `FROB-REF-724` | The verb or an explicitly named mechanism is not in this build: reason in {not-linked, feature-not-built, killed: <gate>} with mechanism and declared_tier when a mechanism is named (pointer /verb or /mechanism; due M0) | §32.18 | 5 |
| `FROB-REF-725` | A mutate verb targets a mutable name or lineage whose owning namespace is not the session token's namespace (pointer /lineage; due M3) | §32.18 | 5 |
| `FROB-REF-726` | Mechanism is built but not admitted for the run's declared purpose, tier, or inputs (mechanism-not-admitted): reason in {island-in-training, tier-not-eligible, purpose-not-eligible, card-provenance}; purpose check precedes build check; carries mechanism, reason, declared_tier, run_purpose, decision ADR path (for purpose-not-eligible), and field/provenance_found/provenances_admitted (for card-provenance) (pointer /mechanism or /actuator_level; due M3, card-provenance M6) | §32.18 | 5 |
| `FROB-REF-761` | Job exceeds compute envelope in force for its scope (scope order: frob-launch launcher token, cited --envelope-scope, verb-derived scope e.g. train.family, else adhoc at default E0: 16 GiB, 8 cores, 1 hour, 1 device). Triggers: demand exceeds class or approved per-run/cumulative limit across (1) memory, (2) cores, (3) devices, (4) wall-time interval upper end, (5) cumulative device-hours, or (6) approved scope limit; approved pretraining E3 without written ceiling and kill rule; child run verb exceeding launcher token declaration; cited envelope_scope contradicting launcher token (pointer /envelope_scope; observed cited scope; required token scope; repairs in order: drop the citation so the token scope applies, or run under a launcher of the cited scope). Raised before allocation after FROB-REF-602 and FROB-REF-704 (daemon campaign without approved envelope), before FROB-REF-701 (session budget); pointer limit crossed; observed demand vector and sim.estimate class; required class in force and limits | §35.11 | 4 |
| `FROB-REF-790` | This binary was built without the airlock feature group the verb needs | §33.16 | 5 |

## Family `8xx` — Evidence and protocol

| Code | Meaning | Defined in | Exit |
|---|---|---|---:|
| `FROB-REF-801` | Unknown verb on the CLI (names no registry entry; MCP answers an unknown tool with JSON-RPC -32602; pointer /verb; due M0) | §32.18 | 3 |
| `FROB-REF-802` | Arguments fail the schema, including an unknown parameter or a sync_wait_ms outside 0..=30000 (pointer /sync_wait_ms or offending parameter; nothing charged; due M0) | §32.18 | 3 |
| `FROB-REF-803` | Idempotency key reused with different resolved canonical request content for (principal, verb, key); both resolutions are shown (pointer /idempotency_key; due M1) | §32.18 | 7 |
| `FROB-REF-804` | Stale parent (expected_parent is not lineage head) or stale field value in design.edit two-value form (set-param <field> <expected> <new>); payload carries head, field, expected, and current (pointer /expected_parent; due M1) | §32.18 | 7 |
| `FROB-REF-805` | A mutate verb or job-submitting run verb arrived without an idempotency key, refused before anything is charged (pointer /idempotency_key; due M1) | §32.18 | 3 |
| `FROB-REF-806` | Pagination cursor does not match the query digest (object, filters, ordering) or schema version, fails authentication tag verification, or was issued to another principal (pointer /cursor; due M1) | §32.18 | 3 |
| `FROB-REF-807` | Pagination cursor expired because the frontier pinned in the cursor has been garbage-collected (pointer /cursor; due M1) | §32.18 | 3 |
| `FROB-REF-808` | An integer outside +/-(2^53 - 1) (+/-9007199254740991) in an int53 number-typed input field; input validation only (serializer out-of-range int53 is FROB-REF-990; due M1) | §32.18 | 3 |
| `FROB-REF-809` | Plan not found or bound to a superseded identity (design, scene, task, backend, or stack) in study.submit (pointer /plan_id; due M1) | §32.18 | 7 |
| `FROB-REF-810` | Chunk conflict: different bytes offered for an already-committed extent; job fails as indeterminate with both content hashes attached (pointer /chunk; due M1) | §32.18 | 9 |
| `FROB-REF-816` | An artifact whose authorization flags are missing or not false | §37.12 | 3 |
| `FROB-REF-861` | Number observed during a fast-training run (learning curve or training return) cited as evidence, or a policy trained under fast-training offered for a claim without re-evaluation under device-replay or stricter (distinct from frob-learn consumer code FROB-REF-867); pointer cited value and run or policy; observed trained_under: D4; required re-evaluation record for the same PolicyId | §35.11 | 3 |
| `FROB-REF-862` | Performance figure without a complete disclosure block. Triggers: (1) required field absent; (2) na used where applicability table forbids it; (3) energy is zero instead of a value or named gap; (4) step rate without its decimation; (5) stale fingerprint digest; (6) ratio block without comparator revision or fewer than ten pairs; (7) parity-defect field missing while a defect is open. Interim trigger: rates violate unit ordering raised with detail = "rates-violate-unit-ordering" and pointer rates.ordering (N36-REF ledger request rates-violate-unit-ordering). Pointer first missing or inadmissible field inside the figure's block; observed missing/inadmissible fields by name; required complete block | §35.11 | 3 |
| `FROB-REF-863` | Performance ratio without a paired lower bound. Triggers: (1) ratio stated with no campaign; (2) campaign of fewer than ten pairs; (3) arms whose units or decimations are not comparable; (4) void campaign where an arm failed; (5) threshold judged on mean instead of bound. Pointer ratio field; observed pair count and whether a bound was computed; required at least ten pairs and the one-sided 95% bound exp(l_bar - t * s / sqrt(n)) | §35.11 | 3 |
| `FROB-REF-864` | Device result cited without a current CPU-versus-device check for its kernels on that device family, driver and compiler (distinct from run-time refusal FROB-REF-605 which refuses to run on the device); pointer result; observed stale or missing check rows with fingerprint digests; required current check for every kernel in the step plan | §35.11 | 3 |
| `FROB-REF-865` | Summary statistic that omits what it excluded (worlds marked Degraded or Faulted, unscored pairs, masked samples); pointer statistic; observed inputs counted against denominator; required excluded table with counts by reason | §35.11 | 3 |
| `FROB-REF-866` | Speed experiment requested without its computed ceiling, or with a ceiling below its goal. Triggers: (1) campaign of kind optimization or speed-experiment with no ceiling record; (2) ceiling filed after first measurement; (3) ceiling computed on cold or uncached baseline; (4) ceiling below goal; (5) pilot whose falsifier is a speed ratio with no ceiling. Pointer experiment's ceiling field, or goal field when ceiling is below it; observed ceiling (absent or value, when, baseline) and goal; required ceiling present, filed before first measurement, computed on warm cached baseline, and at least the goal | §35.11 | 3 |
| `FROB-REF-881` | Evidence or a certificate establishes a different quantifier prefix than the query's, including clairvoyant evidence offered for a robust claim | §25.24 | 3 |
| `FROB-REF-890` | Evidence recorded under another stack receipt is offered as same-stack evidence without revalidation | §33.16 | 3 |
| `FROB-REF-891` | Format version unsupported (format-version-unsupported): unknown schema major version or unknown required field; pointer is the schema member or JSON Pointer of the unknown field; fixture crates/frob-core/tests/fixtures/workspace_refusals/ref891_major_raised.json (and ref891_unknown_field.json); affected_claims filled per occurrence from the verb's declared outputs (CX-361) | §34.9 | 3 |
| `FROB-REF-892` | View is not identity (view-is-not-identity): a registered view, non-canonical JSON, or stored-view reference is offered where an identity is required; pointer is the parameter with observed view:<kind> or non-canonical-spelling; fixture crates/frob-core/tests/fixtures/workspace_refusals/ref892_view_sexp.sexp (and ref892_noncanonical.json); affected_claims filled per occurrence from the verb's declared outputs (CX-360) | §34.9 | 3 |
| `FROB-REF-893` | Format owner bypass (format-owner-bypass): a file with a registered extension or JSON with a registered top-level schema reaches a verb outside its owner's reader; pointer is the parameter and file with observed naming the registered format; fixture crates/frob-core/tests/fixtures/workspace_refusals/ref893_registered_ext.fbot (and ref893_registered_schema.json); affected_claims filled per occurrence from the verb's declared outputs (CX-358) | §34.9 | 3 |

## Family `9xx` — Internal

| Code | Meaning | Defined in | Exit |
|---|---|---|---:|
| `FROB-REF-990` | Internal fault (internal-fault): handler panicked, returned an error mapping to no refusal code, or serializer refused an out-of-range int53; one envelope is still written with pointer naming the verb and observed holding the diagnostic file path; never listed in any verb's refusals (RL-06; due M0) | §32.4 | 9 |

## Unsettled Allocation Requests & Interim Map

| Request | Interim Code(s) | Detail | Pointer | Owner | Status |
|---|---|---|---|---|---|
| `ancestor-path-after-kill` | `FROB-REF-802` | `ancestor-path-after-kill` | `-` | `N16-REF` | `requested` |
| `solve-queues-unqualified` | `none` | `solve-queues-unqualified` | `-` | `N16-REF` | `requested` |
| `task-audit-missing` | `none` | `task-audit-missing` | `-` | `N24-REF` | `requested` |
| `task-audit-unresolved` | `none` | `task-audit-unresolved` | `-` | `N24-REF` | `requested` |
| `wins-outside-intent-unresolved` | `none` | `wins-outside-intent-unresolved` | `-` | `N24-REF` | `requested` |
| `packet-over-byte-cap` | `none` | `packet-over-byte-cap` | `-` | `N32-REF` | `requested` |
| `packet-frames-not-retained` | `none` | `packet-frames-not-retained` | `-` | `N32-REF` | `requested` |
| `rates-violate-unit-ordering` | `FROB-REF-862` | `rates-violate-unit-ordering` | `rates.ordering` | `N36-REF` | `requested` |

## Retired Codes (Never Reused)

| Code | Replaced By | Reason |
|---|---|---|
| `FROB-REF-730` | `FROB-REF-840` | Plan-time safety/Arena scope refusal moved to FROB-REF-840 (exit 5 via exit_rule.capability) |
| `FROB-REF-882` | `FROB-REF-582` | Plan-time retirement replaced by FROB-REF-582 |

## Cross-Chapter Neighbour Pairs

| Left | Left Raising Site | Right | Right Raising Site |
|---|---|---|---|
| `FROB-REF-244` | Admission-time interface-card stiffness estimate prior to simulation launch | `FROB-REF-248` | Run-time solve ill-conditioning measurement (kappa_hat * u > limit) |
| `FROB-REF-561` | Numerics licence check (licence::check) at study or comparison admission | `FROB-REF-825` | Stored-result evidence boundary on study.compare and eval.promote |
| `FROB-REF-562` | Attribution or counterfactual fork admission when source determinism is below D2 | `FROB-REF-584` | Attribution-time fork-set execution failure during counterfactual rollout |
| `FROB-REF-605` | Run-time device execution boundary refusing to launch without a current CPU-versus-device check | `FROB-REF-864` | Evidence citation boundary refusing to cite a device result without a current CPU-versus-device check |
| `FROB-REF-861` | Performance figure admissibility check at report publication | `FROB-REF-867` | Telemetry summary and benchmark envelope admission boundary |
| `FROB-REF-704` | Tether surface resource or wall-time budget admission check | `FROB-REF-761` | Numerics and study step/wall budget enforcement at runtime |
| `FROB-REF-724` | Capability absent because optional subsystem or specialized kernel is not linked in this build | `FROB-REF-726` | Mechanism present in plan but not yet admitted at the current build milestone |
| `FROB-REF-808` | Input validation rejecting an integer outside +/-(2^53 - 1) on an int53 input field | `FROB-REF-990` | Internal serializer fault when an emitted value overflows the int53 wire bound |
