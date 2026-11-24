---
key: "N36-REF"
title: "registry: performance refusals FROB-REF-862, -863, -866, -761, -602: raising-site matrix, a test per trigger, script"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "spark", "tether", "gauntlet", "frob-core", "frob-conformance", "registry", "test", "e2e", "solid", "review-notes-open"]
parent: "WP-47"
deps: ["BD-005", "BD-022", "N21-REF", "N35-REF", "STD-E2E", "STD-TEST"]
needs: []
related: ["N35x-01", "N36-05", "N36-20", "N36-33"]
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
codes their conditions, their repairs and most of their raising sites (the others, in other chapters, reach
`866`, `761` and `602` through the performance program's admission functions). A refusal that exists only as a row in
someone else's table is the kind that never gets raised, and agents then plan around repairs the system never
offers. This bead is the performance program's side of the registry: the raising-site matrix for its five
codes, a test for each trigger of each code, and the script that provokes all five through the real binaries.
The rows themselves (meaning, trigger list, pointer, retry class, ranked repairs, exit code) are written once,
by the registries that own them: the numerics registry N35-REF for `761`, `862`, `863` and `866`, and the Spark
registry N21-REF for `602`. One row has one trigger list and one repair list; this bead reads them and does not
keep a second copy that could drift in wording or order.

## Scope

In: the raising-site matrix (code × bead that raises it); the list of trigger cases to provoke, one test each,
taken from the owners' rows; the checks that every trigger and every matrix cell has a test and that the repairs
arrive in the order of the owner's row; the two forbidden-repair checks of `602`; the unnumbered ordering
rejection, its interim reporting and the proposal for its code; one end-to-end script that provokes each code.
Out: the five registry rows, which are written in N35-REF (`761`, `862`, `863`, `866`: meaning, the trigger
lists, pointer, `observed` and `required`, retry class, ranked repairs, exit 3 or 4) and in N21-REF (`602`:
the four trigger cases, the slab-class pointer, one repair list per slab class, exit 6); this bead depends on
both and writes no row, no trigger wording and no repair list of its own. Also out: detecting each condition
(the raising beads of the matrix below; they are related and none is a dependency, because each trigger's test
lives in its raising bead, the matrix check and the script's step for a code are enabled as that bead lands,
and this bead closes when every cell is covered); the other codes of those two registries.

## Technical approach

**Envelope contract.** With `ok: false` the envelope carries `refusal = { code, message, pointer, observed,
required, unit, affected_claims, retry, repairs[] }`; `retry` is `never`, `after-change` or `backoff`; each repair
is a typed suggestion with a one-line reason. Exit codes by family: `8xx` evidence and protocol refusals exit 3
(refused input or question); budget refusals exit 4; device refusals exit 6. Exit code 1 is never emitted on
purpose.

**The five rows, read from their owners.** The row of each code is the one in `registries/refusals.toml`
entered by its owner; the tests below load it and compare the envelope with it. A change of a trigger list or of
a repair list is made in the owner's bead, and the cases below follow it.

| Code | Row written in | Exit (from the row) | Trigger cases provoked here, one test each |
|---|---|---|---|
| `FROB-REF-862` (a performance figure without a complete disclosure block) | N35-REF | 3 | Seven: (1) a required field of the block absent; (2) `na` used where the applicability table forbids it; (3) `energy` zero instead of a value or a named gap; (4) a step rate without its decimation; (5) a stale fingerprint digest; (6) a ratio block without its comparator revision or with fewer than ten pairs recorded; (7) the parity-defect field missing while a defect is open |
| `FROB-REF-863` (a performance ratio without a paired lower bound) | N35-REF | 3 | Five: (1) a ratio stated with no campaign; (2) a campaign of fewer than ten pairs; (3) arms whose units or decimations are not comparable; (4) a void campaign (an arm failed); (5) a threshold judged on a mean |
| `FROB-REF-866` (a speed experiment without its computed ceiling, or with a ceiling below its goal) | N35-REF | 3 | Five: (1) a campaign of kind `optimization` or `speed-experiment` with no ceiling record; (2) a ceiling filed after the first measurement; (3) a ceiling computed on a cold or uncached baseline; (4) a ceiling below the goal; (5) a pilot whose falsifier is a speed ratio with no ceiling |
| `FROB-REF-761` (a job exceeds the compute envelope in force for its scope) | N35-REF | 4 | Six demand cases, one rule and two launcher-token cases: the job needs a class above the one in force for its scope by (1) memory, (2) cores, (3) devices, (4) the upper end of its wall-time interval, (5) the cumulative device-hours of its gate, (6) the cumulative device-hours of its campaign or pretraining run; (7) an approved pretraining class without a written ceiling and kill rule; (8) a child run verb whose demand is not contained in its launcher token's declaration (pointer the limit of the declaration crossed); (9) a cited `envelope_scope` that contradicts the launcher token's scope (pointer `/envelope_scope`) |
| `FROB-REF-602` (the memory estimate exceeds the device budget at admission) | N21-REF | 6 | Four: (1) the live-interval peak of the request exceeds the device's budget; (2) one shard of a two-card job exceeds its own card (each shard is admitted on its own); (3) scene variants exceed the budget; (4) camera frame buffers exceed the budget. Between them the four cases exercise the four slab classes the pointer can name (`worlds`, `rollout`, `scenes`, `frames`) |

Each test asserts the code, the pointer the row names (the first missing or inadmissible field for `862`; the
ratio field for `863`; `ceiling` or `goal` for `866`; the limit crossed for `761`; the slab class for `602`),
the retry class `after-change`, the exit code, and the repairs in the order of the owner's row. Two checks
concern repairs that must never be offered, and they are tested here because the performance program's memory
arithmetic is their reason: a `602` on `scenes` never offers fewer worlds (scenes are shared by every world of a
device, so the world count does not change their bytes), and a `602` on `frames` never offers a lower resolution
or cadence of the same task (a camera's resolution and cadence are part of the task identity, because changing
the cadence changes the control problem). The rule itself is part of the row in N21-REF.

**Ordering rejection (no code assigned).** A report in which, in env-steps/s on one hardware key, training
exceeds the full environment step or the full step exceeds physics only must be rejected (the unit-ordering
invariant). The plan names no code for it. It is this bead's ledger request `rates-violate-unit-ordering`
(DESIGN slug; N35-33's first round, row "Performance (N36-REF)"): family `8xx`, no proposal, exit 3, condition
"performance rates violate the unit ordering", repairs: more pairs or a longer duration so that the overhead is
resolved; check that the three figures share a hardware key and a decimation. As an `8xx` request it is `held`
under the owner decision N35x-01 (the family's four free numbers are held). Its interim (rule 7 of N35-33) is
`FROB-REF-862`, raised with `detail = "rates-violate-unit-ordering"`, pointer `rates.ordering` and a message
naming the invariant, because a block whose rates contradict each other cannot be a complete and true
disclosure. The interim is raised behind one named constant in `frob-core`, so that assignment is a one-line
change; the interim use is listed by the registry check at each milestone exit.

**Raising-site matrix (code × bead that raises it).**

| Code | Raising sites |
|---|---|
| 862 | Disclosure completeness check (N36-13); rate schema, missing decimation (N36-03); the ordering check `check_ordering`, the interim of `rates-violate-unit-ordering` (`detail` the slug, pointer `rates.ordering`; N36-03); report validation rules 1 and 3 (BD-216); parity-defect field while a defect is open (N36-28); the incumbent adapter (N36-29) |
| 863 | The ratio type and estimator (N36-06); report validation rule 4 (BD-216); the target register's `judge` on a point estimate (N36-01); the optimization ledger's landing check (N36-07) |
| 866 | Experiment admission (N36-05); report validation rule 5 (BD-216); the funding screen over the M2 step profile, including a request with no current profile (N21a-27); the optimization ledger (N36-07); the sharding pilot's consequence and the asynchronous variant's funding rule (N36-09, N36-12); the Jacobian-traffic variants (N36-18). Every site raises it through `admit_experiment` (N36-05), the one place the code is built |
| 761 | Run admission and the device-hour meter (N36-20: `admit_run` / `admit`, the one place the code is built, reached by every run verb's admission, whose generated refusal list carries 761); a child run verb started under a launcher token whose demand exceeds the token's declaration, and a cited `envelope_scope` that contradicts the token (N36-20); two-shard jobs under the default class (N36-08); every campaign arm, ours or an incumbent's (BD-216, N36-29) |
| 602 | Estimator admission (BD-219); per-shard admission (N36-08); scene-bound requests (N36-16); camera-bound requests (N36-19) |

**Sites outside the performance program (866).** Other chapters' speed experiments and comparisons raise 866
too. N36-05 is the only home of the code: every bead that needs a ceiling for a speed experiment calls its
`ceiling` and `admit_experiment` and files a ceiling record, and none builds the refusal itself. Every site
below names that call in its own bead, and a test of `raising_sites_exist` per site asserts that the refusal it
provokes carries the repairs of the N35-REF row in order, which only `admit_experiment` produces. They are listed so that the matrix
is complete and the check can find their tests, but they are not this bead's closing condition:
`raising_sites_exist` checks each one when its bead lands and reports the others `pending: <bead>
(<milestone>)`, never as a pass.

| Chapter | Raising sites (milestone) |
|---|---|
| Kinesis | The lane comparison's ceiling, goal 1.15 (BD-356, M4) |
| Spark | The specialization pilot, goal 1.2 (N21c-17, M2); the fleet pilot's ceiling 1/(1 − g) below 1.3 (N21c-11, BD-370, M4); the megakernel pilot and the fused kernel, goal 2.0 at h = 0 (N21c-21, BD-441, M8) |
| Augur | The speed comparison register `speedcmp` (BD-354, N25a-27, M4), its phase-table ceiling (N25c-06, M4) and the ceiling attachment of the gate (N25ax-02, M3) |
| Autopsy | The bulk job's speed experiment, goal 3 (N27b-27, M3) |
| Suite and numerics | The Forge e-graph screen (N33c-21, M2); the lag study's knobs (N35-05, M3) |
| Program funding | The funding record check, `speed-lever` with a ceiling below the claim (N00-09, M1; its planted instance is a row of N41-33) |

The ten-fold harness's ceiling reasons (`ceiling-*`, `lever-share-above-ledger`, `order-rule-violation`; N39a-37,
N39a-38, N40-18 at M1, BD-314 at M3) are not raising sites of the code: they are harness reasons emitted by
`frob-tenfold` and its gate, which carry `maps_to = "FROB-REF-866"` in the ten-fold registry N39-REF so that
the two vocabularies stay aligned. They are listed by N39-REF, not by this matrix.

**One job, one code: `602` before `761` (DESIGN, stated identically in BD-219).** At run admission the device-fit
test comes first, then the envelope checks (for a job queued by the daemon, the size classifier's `704` at
submission, then the envelope admission's `761`; convention 14), then session budgets (`701`). A job that does not fit the device is
not repaired by approving a larger envelope class, so a job that fails both (for example 0.2 M humanoid worlds on
a 24 GB card, about 19 GB, which is also above E0's 16 GiB) is refused with `602` and its fit repairs, never with
`761` and its "request the class" repair. A job that fits the device and exceeds the class in force is `761`.

**Registry tests.** For each code: the owner's row exists in `registries/refusals.toml` (loading a registry
without it fails); the exit code mapping is unique; every trigger case in the table above has a test that
provokes it and asserts code, pointer, retry class and the order of repairs as the row gives them; every
raising site named in the matrix has such a test in its own bead (checked by name against the workspace's test
listing). A trigger case that the owner's row does not list, or a row trigger with no case here, fails
`trigger_cases_match_rows`.

## Acceptance criteria

- [ ] The five codes are present in the refusal registry as their owners entered them (N35-REF, N21-REF); loading a registry without one of them fails; this bead adds no row and no second trigger or repair list.
- [ ] Each trigger case listed in the table has a passing test that asserts code, pointer, `retry` and the repairs in the order of the owner's row; the scene and frame variants of `602` are shown never to offer the forbidden repairs.
- [ ] The trigger cases of this bead and the trigger lists of the owners' rows are the same sets (`trigger_cases_match_rows`).
- [ ] Exit codes: 3 for `862`, `863` and `866`; 4 for `761`; 6 for `602`; each code maps to exactly one exit code.
- [ ] Every cell of the raising-site matrix of the performance program names a test that exists; every site of the table of other chapters' 866 sites is either checked (its bead has landed and its test exists and raises through `admit_experiment`) or reported `pending` with its bead and milestone.
- [ ] The ordering rejection is reported under the interim 862 with `detail = "rates-violate-unit-ordering"` and pointer `rates.ordering`, behind its named constant, and its ledger row `rates-violate-unit-ordering` (N35-33's first round) is `held` under N35x-01.
- [ ] `602` refusals carry the fingerprint digest; `761` refusals carry the demand vector, the class in force and the class needed.
- [ ] The end-to-end script provokes all five through the real binaries and logs each envelope.

## Test plan

### Unit tests

`cargo test -p frob-core refusals::perf` and `cargo test -p frob-conformance registry::refusals::perf`:
- `rows_present_and_exit_codes`: 862 → 3, 863 → 3, 866 → 3, 761 → 4, 602 → 6.
- `triggers_862` (seven cases, plus `ordering_is_interim_862`: the KA-806 report → `862` with `detail = "rates-violate-unit-ordering"`, pointer `rates.ordering` and a message naming INV-PERF-01), `triggers_863` (five), `triggers_866` (five), `triggers_761` (six demand cases, the pretraining rule and the two launcher-token cases), `triggers_602` (four slab classes).
- `repairs_in_order` per code, against the list loaded from the owner's row (four repairs for 761, three for 862, three for 863, four for 866; for 602 one list per slab class); `scene_602_never_offers_fewer_worlds`; `frames_602_never_offers_lower_resolution`.
- `ordering_rejection_pointer`.
- `fit_precedes_envelope`: 200,000 humanoid worlds (200,000 × 96,704 = 19.3 GB) on a 24 GB card with a 16 GiB
  budget under E0 → `602` (pointer `worlds`, exit 6), not `761`; 50,000 humanoid worlds on two declared devices
  under E0 (each shard fits) → `761` (pointer `devices`, exit 4).
- `raising_sites_exist` (matrix against the test listing; the other chapters' 866 sites as `checked` or
  `pending: <bead> (<milestone>)`; a scratch listing in which a landed site's test is absent fails, and a site
  whose refusal carries repairs in another order than the N35-REF row fails as not routed through
  `admit_experiment`).
- `trigger_cases_match_rows`: the cases of the table above equal the trigger lists of the rows loaded from the registry, code by code.
- `unknown_code_is_load_error`; `codes_are_unique`.

### End-to-end script and logging

`scripts/e2e/perf_refusals.sh` (owned by this bead; sources the shared end-to-end helper; seed-explicit;
idempotent; uses scratch registries and `synthetic` job specifications so that refused jobs spend no device
time). Steps, each enabled when the bead that defines its verb, option or fixture has landed (logged `not-run:
<bead> open` before that, never counted as a pass): `602` via `frob sim run --workload H --worlds 200000
--seconds 1 --tier gpu-cuda --key pr-602 --json` (`--workload` is N36-33's argument group on `sim.run`; on the
Mac the same with `--tier gpu-metal`; 200,000 humanoid worlds are about 19.3 GB, above a 16 GiB device budget and
above E0's 16 GiB; asserts exit 6, code `602` with pointer `worlds` and not `761`, and that allocated device bytes
are unchanged); `761` via `frob sim run @fixture:envelope-two-device --key pr-761 --json` (N36-20's fixture,
run in process so it never meets the daemon's size classifier; exit 4, pointer `devices`, the demand vector and
both classes in the payload); `862`, `863` and `866` via the harness's validator on three planted records, run
by `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test g1_known_answers -- refusals::perf` (exit code 3
is asserted on the equivalent envelope built by `frob-verbs`); the ordering rejection via the planted report
(KA-806). Each NDJSON line has `ts`, `script`, `bead`, `step`,
`event`, the exact command line, exit code, wall time and the evidence block; `refusal` lines carry the whole
refusal object (code, message, pointer, observed, required, unit, retry, repairs). A `summary` line counts
assertions and lists the five codes with the number of triggers exercised.

### Known answers and negative controls

- KA-825 (`862` and `863`), KA-812 (`866`), KA-823 (`761`), KA-826 (`602`).
- Negative controls: a build in which the disclosure check returns a warning instead of a refusal (the claim would be emitted; the script must fail); a build in which `602` is raised after allocation (the allocated-bytes assertion fails); a build that maps `761` to exit 3.
- Determinism: refusals are pure functions of their inputs; the same planted record gives the same envelope bytes twice (apart from timestamps).

## Considerations

- Ownership is split on purpose: the numerics and Spark registries own the rows, with their trigger and repair lists; this bead owns what the performance program does with them (who raises what, and the proof that each trigger is raised). If either registry changes a meaning, a trigger or a repair, the tests here fail first.
- `FROB-REF-704` (a campaign-class job without an approved compute envelope) is the daemon's sibling of `761` and is not raised by the performance program. DESIGN, stated identically in the surface registry N32-REF (owner of row 704) and in N35-REF (owner of row 761), so that one oversized job gets one code (convention 14): a job-submitting verb queued by the daemon is classified when the job is submitted, and a job in the `campaign` size class whose scope (the launcher's token, the cited `envelope_scope`, the scope the verb derives, otherwise `adhoc`) has no approved envelope is refused with 704 by the size classifier, before the envelope admission is reached; every other case is 761, raised by `sim.estimate` and run admission before the job starts: a run executed in process by `frob` (where no size class exists) above the class in force for its scope (E0 for `adhoc`), a daemon job whose scope has an approved envelope and that still exceeds its per-run limits or cumulative device-hours, a child run verb above its launcher token's declaration, and a cited `envelope_scope` that contradicts the launcher's token. Both come before the session-budget check 701. The plan defines both codes and does not divide them; the reason for dividing by raising site is that an agent must not have to handle one event two ways. The script's `761` step runs in process (`frob sim run`), so it never meets the classifier.
- `FROB-REF-864` (a device result cited without a current CPU-versus-device check) and `FROB-REF-606` (fingerprint missing or changed) are raised by other chapters on the same figures; a performance figure can therefore be refused for reasons outside this table, and the disclosure check runs after those.
- At M2 the surfaces that can state a Performance claim are run envelopes and gate records; when report verbs arrive later they must route through the same checks, which is why the checks live in `frob-core` and not in the harness.

## References

§35.11 (the code table), §36.5, §36.6, §36.7, §36.10, §36.12, §36.14, §21.21, §32.4 (envelope and exit codes), §32.18, §32.11.2;
Appendix O; KA-812, KA-823, KA-825, KA-826; INV-PERF-01.
