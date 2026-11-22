---
key: "N36-REF"
title: "registry: performance refusals FROB-REF-862, -863, -866, -761, -602: triggers, repairs, exit codes, sites"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "spark", "tether", "gauntlet", "frob-core", "frob-conformance", "registry", "test", "e2e", "solid"]
parent: "WP-47"
deps: ["BD-005", "BD-022", "N36-05", "N36-06", "N36-13", "N36-20", "BD-219", "STD-TEST", "STD-E2E"]
needs: ["§35.11: the registry rows of `FROB-REF-761` and `FROB-REF-862` to `-866` in `registries/refusals.toml`, which the numerics chapter's table defines (its drafting unit owns those rows); this bead supplies the performance program's triggers, repairs and raising sites for them", "§21.21: the registry row of `FROB-REF-602` among the device refusals (drafted by the Spark unit's refusal registry, N21-REF)", "Appendix O: assignment of a code for a performance report whose rates violate the unit ordering (INV-PERF-01); the plan requires the rejection and names no code (proposed here: the next free `8xx` code)"]
related: ["N36-KA", "N36-CX", "N36-03", "N36-07", "N36-08", "N36-16", "N36-19", "N36-28", "N36-30", "N36-32", "N21-REF", "N21c-35", "BD-216"]
gates: []
plan_refs: ["§35.11", "§36.5", "§36.6", "§36.7", "§36.10", "§36.14", "§21.21", "§32.4", "§32.18", "Appendix O"]
---
## Background

Every way in which the system declines to answer is a typed refusal: a stable code, a message, the field at
fault, observed and required values with their unit, and repairs ranked by how likely they are to help. Standing
rules: refuse, never truncate; a budget stop publishes nothing partial as a result; a misspelt parameter is a
refusal, not a default. Codes live in one machine-readable registry, `registries/refusals.toml`, loaded by
`frob-core`; an unknown code is a load-time error and a code is never reused. Families: `1xx` design and model,
`2xx` numerics, `3xx` scene, `4xx` control, `5xx` learning and evaluation protocol, `6xx` backend and device,
`7xx` budget and capability, `8xx` evidence and protocol, `9xx` internal.

The performance chapter has no refusal table of its own. Its refusals are numbered in two other tables: the
numerics chapter's table lists the codes "defined by this chapter and by the performance program" (`761` and
`862` to `866`), and the device table lists `602`. But it is the performance program that gives five of those
codes their conditions, their repairs and every one of their raising sites. A refusal that exists only as a row in
someone else's table is the kind that never gets raised, and agents then plan around repairs the system never
offers. This bead is the performance program's side of the registry: the full rows for its five codes, the exit
codes, the raising-site matrix, and the test that each code is actually raised by each of its triggers.

## Scope

In: the five rows with trigger, pointer, retry class and ranked repairs; the exit-code mapping; the raising-site
matrix; the unnumbered ordering rejection and the proposal for its code; registry tests; one end-to-end script
that provokes each code. Out: detecting each condition (the raising beads); the registry rows' ownership (the
numerics and Spark registries); the other codes of those tables.

## Technical approach

**Envelope contract.** With `ok: false` the envelope carries `refusal = { code, message, pointer, observed,
required, unit, affected_claims, retry, repairs[] }`; `retry` is `never`, `after-change` or `backoff`; each repair
is a typed suggestion with a one-line reason. Exit codes by family: `8xx` evidence and protocol refusals exit 3
(refused input or question); budget refusals exit 4; device refusals exit 6. Exit code 1 is never emitted on
purpose.

**Rows.**

| Code | Meaning | Triggers | Pointer | Retry | Ranked repairs | Exit |
|---|---|---|---|---|---|---|
| `FROB-REF-862` | A performance figure without a complete disclosure block | A required field of the block is absent; `na` used where the applicability table forbids it; `energy` is zero instead of a value or a named gap; a step rate without its decimation; a stale fingerprint digest; a ratio block without its comparator revision or with fewer than ten pairs recorded; the parity-defect field missing while a defect is open | The first missing or inadmissible field | after-change | 1. Re-run under the performance harness, which fills the block from the run's receipts. 2. Supply the named field. 3. Cite the figure as context with no claim | 3 |
| `FROB-REF-863` | A performance ratio without a paired lower bound | A ratio stated with no campaign; a campaign of fewer than ten pairs; arms whose units or decimations are not comparable; a void campaign (an arm failed); a threshold judged on a mean | The ratio field | after-change | 1. Run a paired campaign of at least ten interleaved pairs. 2. State the two figures separately with no ratio. 3. Cite as context | 3 |
| `FROB-REF-866` | A speed experiment requested without its computed ceiling, or with a ceiling below its goal | A campaign of kind `optimization` or `speed-experiment` with no ceiling record; a ceiling filed after the first measurement; a ceiling computed on a cold or uncached baseline; a ceiling below the goal; a pilot whose falsifier is a speed ratio with no ceiling | `ceiling` or `goal` | after-change | 1. Compute the ceiling from the current profile. 2. Lower the goal to at most the ceiling. 3. Choose a target holding a larger share of the baseline. 4. Fund the work for a reason other than speed, in writing, with no speed figure | 3 |
| `FROB-REF-761` | A job exceeds the compute envelope in force | The job's demand (memory, cores, devices, the upper end of its wall-time interval, cumulative device-hours for its gate, campaign or pretraining run) needs a class above the one approved; an approved pretraining class without a written ceiling and kill rule | The limit crossed | after-change | 1. A smaller job that fits, with the largest world count, thread count or duration that does. 2. Split into runs under the per-run limit. 3. Request the named class from the owner | 4 |
| `FROB-REF-602` | The memory estimate exceeds the device budget at admission | The live-interval peak of the request exceeds the device's budget; one shard of a two-card job exceeds its own card; scene variants exceed the budget; camera frame buffers exceed the budget. Carries the fingerprint digest | The slab class that exceeds (`worlds`, `scenes`, `frames`, `rollout`) | after-change | For worlds: fewer worlds (the largest count that fits is named), a smaller capacity, or the Mac. For scenes: fewer distinct scene variants, a coarser scene (a different scene identity), or the Mac; never fewer worlds. For frames: fewer worlds, the Mac, or a different task with smaller cameras; never a lower resolution or cadence of the same task | 6 |

**Ordering rejection (no code assigned).** A report in which, in env-steps/s on one hardware key, training
exceeds the full environment step or the full step exceeds physics only must be rejected (the unit-ordering
invariant). The plan names no code for it. Until the registry's owner assigns one, the rejection is reported as
`FROB-REF-862` with pointer `rates.ordering` and a message naming the invariant, because a block whose rates
contradict each other cannot be a complete and true disclosure. Proposed: a dedicated `8xx` code "performance
rates violate the unit ordering", exit 3, repairs: more pairs or a longer duration so that the overhead is
resolved; check that the three figures share a hardware key and a decimation.

**Raising-site matrix (code × bead that raises it).**

| Code | Raising sites |
|---|---|
| 862 | Disclosure completeness check (N36-13); rate schema, missing decimation (N36-03); report validation rules 1 and 3 (BD-216); parity-defect field while a defect is open (N36-28); the incumbent adapter (N36-29) |
| 863 | The ratio type and estimator (N36-06); report validation rule 4 (BD-216); the target register's `judge` on a point estimate (N36-01); the optimization ledger's landing check (N36-07) |
| 866 | Experiment admission (N36-05); report validation rule 5 (BD-216); the optimization ledger (N36-07); the sharding pilot's consequence and the asynchronous variant's funding rule (N36-09, N36-12); the Jacobian-traffic variants (N36-18) |
| 761 | Run admission and the device-hour meter (N36-20); two-shard jobs under the default class (N36-08); every campaign arm, ours or an incumbent's (BD-216, N36-29) |
| 602 | Estimator admission (BD-219); per-shard admission (N36-08); scene-bound requests (N36-16); camera-bound requests (N36-19) |

**Registry tests.** For each code: the row exists in `registries/refusals.toml` with the meaning above; the exit
code mapping is unique; every trigger in the table has a test that provokes it and asserts code, pointer, retry
class and the order of repairs; every raising site named in the matrix has such a test in its own bead (checked
by name against the workspace's test listing).

## Acceptance criteria

- [ ] The five codes are present in the refusal registry with the meanings above; loading a registry without one of them fails.
- [ ] Each trigger listed in the table has a passing test that asserts code, pointer, `retry` and the repairs in order; the scene and frame variants of `602` are shown never to offer the forbidden repairs.
- [ ] Exit codes: 3 for `862`, `863` and `866`; 4 for `761`; 6 for `602`; each code maps to exactly one exit code.
- [ ] Every cell of the raising-site matrix names a test that exists.
- [ ] The ordering rejection is reported with pointer `rates.ordering`, and the proposal for a dedicated code is filed with the registry's owner.
- [ ] `602` refusals carry the fingerprint digest; `761` refusals carry the demand vector, the class in force and the class needed.
- [ ] The end-to-end script provokes all five through the real binaries and logs each envelope.

## Test plan

### Unit tests

`cargo test -p frob-core refusals::perf` and `cargo test -p frob-conformance registry::refusals::perf`:
- `rows_present_and_exit_codes`: 862 → 3, 863 → 3, 866 → 3, 761 → 4, 602 → 6.
- `triggers_862` (seven cases), `triggers_863` (five), `triggers_866` (five), `triggers_761` (six demand cases and the pretraining rule), `triggers_602` (four slab classes).
- `repairs_in_order` per code; `scene_602_never_offers_fewer_worlds`; `frames_602_never_offers_lower_resolution`.
- `ordering_rejection_pointer`.
- `raising_sites_exist` (matrix against the test listing).
- `unknown_code_is_load_error`; `codes_are_unique`.

### End-to-end script and logging

`scripts/e2e/perf_refusals.sh` (owned by this bead; sources the shared end-to-end helper; seed-explicit;
idempotent; uses scratch registries and `synthetic` job specifications so that refused jobs spend no device
time). Steps: `602` via `frob sim run --workload H --worlds <above the largest that fits> --json` (asserts exit 6
and that allocated device bytes are unchanged); `761` via `frob sim run @fixture:envelope-two-device --json`
(exit 4); `862`, `863` and `866` via the harness's validator on three planted records, run by `cargo test -p
frob-conformance --test g1 -- refusals::perf` (exit code 3 is asserted on the equivalent envelope built by
`frob-verbs`); the ordering rejection via the planted report. Each NDJSON line has `ts`, `script`, `bead`, `step`,
`event`, the exact command line, exit code, wall time and the evidence block; `refusal` lines carry the whole
refusal object (code, message, pointer, observed, required, unit, retry, repairs). A `summary` line counts
assertions and lists the five codes with the number of triggers exercised.

### Known answers and negative controls

- KA-825 (`862` and `863`), KA-812 (`866`), KA-823 (`761`), KA-826 (`602`).
- Negative controls: a build in which the disclosure check returns a warning instead of a refusal (the claim would be emitted; the script must fail); a build in which `602` is raised after allocation (the allocated-bytes assertion fails); a build that maps `761` to exit 3.
- Determinism: refusals are pure functions of their inputs; the same planted record gives the same envelope bytes twice (apart from timestamps).

## Considerations

- Ownership is split on purpose: the numerics and Spark registries own the rows, this bead owns what the performance program does with them. If either registry changes a meaning, the tests here fail first.
- `FROB-REF-704` (a campaign-class job without an approved compute envelope) is the daemon's sibling of `761` and is not raised by the performance program; the envelope bead keeps the two apart.
- `FROB-REF-864` (a device result cited without a current CPU-versus-device check) and `FROB-REF-606` (fingerprint missing or changed) are raised by other chapters on the same figures; a performance figure can therefore be refused for reasons outside this table, and the disclosure check runs after those.
- At M2 the surfaces that can state a Performance claim are run envelopes and gate records; when report verbs arrive later they must route through the same checks, which is why the checks live in `frob-core` and not in the harness.

## References

§35.11 (the code table), §36.5, §36.6, §36.7, §36.10, §36.14, §21.21, §32.4 (envelope and exit codes), §32.18;
Appendix O; KA-812, KA-823, KA-825, KA-826; INV-PERF-01.
