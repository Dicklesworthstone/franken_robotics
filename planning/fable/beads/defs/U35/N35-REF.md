---
key: "N35-REF"
title: "Numerics refusal registry: FROB-REF-247 to -252, -561, -562, -761, -861 to -866 with repairs and exits"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "frob-core", "solid", "registry", "review-notes-open"]
parent: "WP-01"
deps: ["BD-005", "N35-33"]
needs: []
related: ["N35x-01", "N35x-02", "N36-20", "N37-REF"]
gates: []
plan_refs: ["§35.11", "Appendix O", "§35.2", "§35.3", "§35.6", "§35.8", "§35.10", "§35.12", "§36.5", "§36.6", "§36.7", "§36.14", "§32.4"]
---
## Background

Every refusal of the system has a stable code, a message and ranked repairs. A refusal is a typed answer: it
says what was at fault, what was observed against what was required, which claims are affected, whether a retry
can help and what to do next, so that an agent can repair without guessing. The machine-readable registry is
`registries/refusals.toml`; an unknown code is a load-time error and codes are never reused. The envelope's
`refusal` block carries `code`, `message`, `pointer` (the field or part at fault), `observed`, `required`,
`unit`, `affected_claims`, `retry` (`never`, `after-change`, `backoff`) and ranked `repairs`.

Code families: `1xx` design and model; `2xx` numerics and integrator admission; `3xx` scene; `4xx` control and
estimation; `5xx` learning and evaluation protocol; `6xx` backend and device; `7xx` budget and capability;
`8xx` evidence and protocol (for example citing a provisional result as established); `9xx` internal. Exit codes
of `frob`: 0 completed; 2 completed with findings or flags; 3 refused input or question (codes 1xx to 5xx, and
8xx other than conflicts); 4 budget, refused at admission or stopped; 5 capability; 6 backend or device; 7
conflict; 8 accepted and still running; 9 indeterminate. Exit code 1 is never emitted on purpose.

Three standing rules: refuse, never truncate; a budget stop publishes nothing partial as a result; a misspelt
parameter is a refusal, not a default (tested by N35-26).

The numerics chapter's table defines fifteen codes: six about arithmetic and kernel admission (247 to 252), two
about what a determinism class licenses (561, 562), one about the compute envelope (761) and six about evidence
(861 to 866). Six of the fifteen (761, 862, 863, 866, and in part 864 and 865) are raised by the performance
program and by other chapters' mechanisms; the table lists them here because the registry row was defined in
this chapter. This bead puts all fifteen rows in the registry with trigger, pointer, ranked repairs and exit
code, and binds each to the bead that raises it. It is due at M0 because two build-time codes (250, 251) are
raised by M0 beads. Tier: Solid.

## Scope

In: the fifteen rows as data; message templates; `pointer`, `observed` and `required` shapes; ranked repairs;
`retry`; exit codes; relationships to neighbouring codes; the binding to raising beads; registry tests.
Out: the mechanisms that raise the refusals (last column); the `6xx` device codes (the Spark registry); the
end-to-end sweeps that provoke each code from the CLI (N35-31 for 247 to 249, 562 and 864; N35-32 for 561, 562, 861;
`scripts/e2e/perf_refusals.sh` of N36-REF for 761, 862, 863, 866); the raising-site matrix and the per-trigger
tests of the four performance rows (N36-REF, which depends on this bead and writes no row of its own).

## Technical approach

The chapter's table gives each code's condition. It gives no repair list, so the repairs are ranked here from
the rules each code enforces (DESIGN). `retry` is `after-change` for every row: none of these conditions clears
by waiting. "Build time" codes are raised by the kernel toolchain and by `xtask`, never at run time; the build
fails and the diagnostic carries the same fields.

| Code | Refused because (trigger) | Pointer; observed; required | Ranked repairs | Exit | Raised and tested in |
|---|---|---|---|---|---|
| FROB-REF-247 | The determinism class a verb requires is not attainable on the requested placement (for example a replay with a claim of bit equality on a device family that holds only class D3 against the CPU; a fork on a family with no current batch-independence record) | The verb and the placement; the class attained there with the record that says so; the class required and the use that requires it | (1) Run on a placement where the class is attained (the refusal lists them, `batch-cpu` first); (2) if the record is stale and not absent, `frob spark check --sweeps all` to make it current; (3) ask for the weaker use the licence table offers (a tolerance replay, `paired-by-address`, an unpaired comparison) by naming it in the request | 3 | N35-04, N35-07 |
| FROB-REF-248 | A solve is ill-conditioned beyond its precision profile (κ̂·u above the limit, default 0.1), under the `refuse` policy | The world address and step; κ̂ and κ̂·u (named a lower bound), the precision profile; κ̂·u ≤ limit | (1) `--precision stiff` (one refinement step with a double-float residual); (2) run the world on `batch-cpu` in double precision or on the referee; (3) soften the contact impedance or replace the near-rigid interface card; (4) `--on-degraded continue-flagged`, which keeps the world and marks it `Degraded: ill-conditioned` | 3 | N21b-38 (the conditioning certificate; its test `refusal_248_under_refuse`); BD-179; N35-04 (the option); N35-29 (the limit) |
| FROB-REF-249 | An anchored offset left its range in one step (magnitude 16 m or more after integration), or a cell index left the scene bounds | The world address, body and axis; the offset or cell index; magnitude below 16 m and index inside the declared bounds | (1) `frob sim replay <episode> --tier batch-cpu` to locate the first divergence of the blown-up world; (2) reduce the step or check actuator and contact parameters (a body that moves 12 m in one step is not physical); (3) enlarge the scene's declared bounds if the body legitimately left them | 3 | N21b-37 (anchored positions; its test `refusal_249`); BD-157; BD-179 |
| FROB-REF-250 | A kernel is not dimensionally homogeneous (build time): a sum of terms with different exponent vectors over the base dimensions, or a square root of an odd exponent vector | The kernel, the statement and both exponent vectors; equal vectors | (1) Fix the derivation (the diagnostic names the sum); (2) declare the missing conversion constant with its dimension; (3) none: the check is never suppressed | 3 (build failure) | BD-018 (the dimension checker); theorem card T-09 |
| FROB-REF-251 | A banned operation appears in kernel source (build time): a fast-math relaxation, a fused multiply-add (contracted or explicit), a built-in transcendental or reciprocal square root, a floating-point atomic | The file, line and token; the ban it falls under | (1) Replace by the admitted form: a `det_*` function, the split product, count then exclusive scan then write, a select; (2) move the computation to the host if it does not belong in a batched kernel | 3 (build failure) | BD-013 (dialect lint); N35-18 (ban table, compile-flag audit, corpus) |
| FROB-REF-252 | A strict predicate in floating point without an exact form or a certified guard (build time) | The kernel and the comparison; its predicate class; `exact` or `guarded` | (1) Express the predicate in the exact-integer kernel class on quantized inputs; (2) a guarded kernel: an `f32` residual with a derived error bound and an exact fallback; (3) if the requirement allows it, declare the comparison non-strict in the requirement (a change of the requirement, recorded as one) | 3 (build failure) | BD-053 |
| FROB-REF-561 | A paired statistic was requested with an arm of class D4 or of unrecorded class, or as paired across device families without class D1, at the numerics licence check (`licence::check`, N35-07). On `eval.compare` the comparison path's `pairing::admit` runs first and answers a D4 arm, arms of different classes and a D3 pair without a decision-equivalence record with 542 (N28-REF), so there 561 answers only what 542 does not cover: an arm of unrecorded class, and a pairing demanded `exact` that the placement cannot license (a cross-family pair without D1, a D3 pair). On `study.compare` a stored study with an arm of class D4 or unrecorded is 825 (N37-28's evidence boundary; VERBS-study lists neither 542 nor 561 there). On `eval.detect` it answers a D4 or unrecorded arm. On `eval.promote` a stored result of class D4 or unrecorded is 825 (N37-28's boundary runs first, in the dispatcher), and 561 answers the fast-tier result and its re-evaluation paired across device families without class D1, and arms that cannot be paired (VERBS-eval). The Arena study verbs (`study.plan` and the others admitted by `study::admit`) answer 533 | The arm; its class and profile, or both device families; both arms class D2 on one family with a current batch-independence record, or D1 across families, or D3 with a decision-equivalence record | (1) Re-evaluate the offending arm under `device-replay` or stricter; (2) run both arms on one device family, or on `batch-cpu`; (3) request the comparison as `unpaired`, with the larger sample that needs; (4) under class D3 with a recorded decision-level equivalence, request `paired-by-address` | 3 | N35-07 (the check; KA-802 calls it directly, where a D4 arm is 561); BD-269 (the wiring after `pairing::admit`); BD-262 |
| FROB-REF-562 | Attribution or a counterfactual fork was requested below class D2 (the source run's class is D3, D4 or unrecorded, or the batch-independence record of its device family is not current) | The source run; its class and the age of the record; D2 with GATE-SPARK-15 current for the family | (1) Teleport the world to `batch-cpu` and fork there; (2) re-run the episode under `device-replay` on a family with a current record; (3) `frob spark check --sweeps all` to renew the record | 3 | N35-07; BD-269; the fork verb (BD-205) |
| FROB-REF-761 | A job exceeds the compute envelope in force for its scope (default E0: 16 GiB, 8 cores, one hour, one device). The scope of a demand is, in this order: the token of a launcher it runs under (`frob-launch`, the campaign runner), the `envelope_scope` the request cites (`--envelope-scope <gate:…|campaign:…|pretrain:…>`, a common input of every `run` verb, NJL-01), a scope the verb derives from its own inputs (`train.family`), otherwise `adhoc`, whose class in force is E0. Triggers: the job's demand (memory, cores, devices, the upper end of its wall-time interval, or the cumulative device-hours of its gate, campaign or pretraining run) needs a class above the one in force for its scope, or exceeds an approved scope's per-run or cumulative limit; an approved pretraining class (E3) without a written ceiling and kill rule; a child run verb whose demand is not contained in the declaration of the launcher token it was started under (N36-20; `pointer` the limit of the declaration that was crossed, `required` the token's declared demand); DESIGN: a cited `envelope_scope` that contradicts the scope of the launcher's token (`pointer` `/envelope_scope`, `observed` the cited scope, `required` the token's; its repairs: (1) drop the citation, so the token's scope applies; (2) run under a launcher of the cited scope). Raised before the job starts and before any allocation: after the device-fit check (602) and, for a job queued by the daemon, after the size classifier's 704; before the session-budget check 701 | The limit crossed; the job's demand vector and the class `sim.estimate` computed for it (the class needed); the class in force with its limits | (1) A smaller job that fits (fewer worlds, threads or steps): the refusal names the largest world count, thread count or duration that fits; (2) split it into runs that each fit under the per-run limit; (3) request approval of the class printed from the owner, with its ceiling and kill rule written first, then resubmit citing the approved scope with `--envelope-scope <scope>` (over MCP, the `envelope_scope` input); (4) take the registered not-approved fallback of the work package | 4 | BD-219 (`sim.estimate`); N36-20 (run admission and the device-hour meter); the envelope register; every site in the matrix of N36-REF |
| FROB-REF-861 | A number observed during a `fast-training` run (a learning curve, a training return) was cited as evidence, or a policy trained under it is offered for a claim with no re-evaluation under `device-replay` or stricter | The cited value and its run, or the policy; `trained_under: D4`; a re-evaluation record for the same `PolicyId` | (1) `frob eval run --policy <id> --determinism device-replay` and cite that evaluation; (2) retrain under `device-replay` if the training curve itself is what must be cited | 3 | N35-06; BD-269 |
| FROB-REF-862 | A performance figure without a complete disclosure block. Triggers: a required field of the block is absent; `na` used where the applicability table forbids it; `energy` is zero instead of a value or a named gap; a step rate without its decimation; a stale fingerprint digest; a ratio block without its comparator revision or with fewer than ten pairs recorded; the parity-defect field missing while a defect is open. Interim trigger: the block's rates violate the unit ordering, raised as 862 with `detail = "rates-violate-unit-ordering"` and pointer `rates.ordering` (the interim of N36-REF's ledger request `rates-violate-unit-ordering`, `8xx`, a first-round request of N35-33 held under N35x-01) | The first missing or inadmissible field of the figure's block; the missing or inadmissible fields by name; the complete block | (1) Re-run through the performance harness, which fills the block from the run's receipts; (2) supply the named fields from the run's receipt; (3) present the figure as context without the Performance claim | 3 | BD-216; every site in the matrix of N36-REF |
| FROB-REF-863 | A performance ratio without a paired lower bound. Triggers: a ratio stated with no campaign; a campaign of fewer than ten pairs; arms whose units or decimations are not comparable; a void campaign (an arm failed); a threshold judged on a mean instead of the bound | The ratio field; the number of pairs and whether a bound was computed; at least ten pairs and the one-sided 95% bound exp(ℓ̄ − t·s/√n) | (1) Run a paired campaign of at least ten pairs, the two arms of each pair back to back in randomized order on one host state, and report the bound; (2) state the two rates separately, without a ratio; (3) cite the figures as context with no claim | 3 | BD-216; every site in the matrix of N36-REF |
| FROB-REF-864 | A device result cited without a current CPU-versus-device check for its kernels on that device family, driver and compiler | The result; the stale or missing check rows with fingerprint digests; a current check for every kernel in the step plan | (1) `frob spark check` on that family; (2) `frob spark fingerprint` first if the fingerprint changed; (3) cite the `batch-cpu` result | 3 | N35-07 (the citation rule); BD-176 (currency) |
| FROB-REF-865 | A summary statistic that omits what it excluded (worlds marked `Degraded` or `Faulted`, unscored pairs, masked samples) | The statistic; its inputs counted against its denominator; the `excluded` table with counts by reason | (1) Regenerate through `frob telemetry summary`, which states exclusions; (2) add the counts by reason and the denominator | 3 | N35-28 |
| FROB-REF-866 | A speed experiment requested without its computed ceiling, or with a ceiling below its goal. Triggers: a campaign of kind `optimization` or `speed-experiment` with no ceiling record; a ceiling filed after the first measurement; a ceiling computed on a cold or uncached baseline; a ceiling below the goal; a pilot whose falsifier is a speed ratio with no ceiling | The experiment's `ceiling` field, or its `goal` field when the ceiling is below it; the ceiling (absent, or its value, with when and on which baseline it was computed) and the goal; a ceiling present, filed before the first measurement, computed on the warm cached baseline, and at least the goal | (1) Compute the ceiling from the current phase profile and attach it (removing a phase of share f gives at most 1/(1 − f)); (2) lower the goal to at most the ceiling; (3) choose another lever: a target holding a larger share of the baseline; (4) fund the work for a reason other than speed, in writing, with no speed figure; failing that, do not run the experiment | 3 | BD-354; N35-05 (the profile's own ceiling); every site in the matrix of N36-REF |

**Neighbouring codes, and the one-condition-one-code reading (DESIGN; the overlaps are listed for the lead).**

| Pair | Reading used |
|---|---|
| 561, 542 and 533 (a paired statistic under an uncontrolled determinism class), and 825 (a result of class D4 offered as evidence) | A D4 arm at the planning or admission of a paired comparison is a property of the execution being planned, and one condition at three layers (N35x-02's pair table, N28-REF's layer rule): 542 on `eval.compare`, whose `pairing::admit` (BD-262, N28a-18) runs first; 533 at the Arena study admission (`study.plan`, `study.submit`, `eval.qualify`, `study::admit`); 561 at the numerics licence check (N35-07) for any other paired statistic. On `eval.compare` the licence check runs after `pairing::admit` and keeps 561 for an arm of unrecorded class and for a pairing demanded `exact` that the placement cannot license (VERBS-eval). 825 is the evidence boundary's code for a stored result of class D4 or unrecorded offered to one of its five consumers: promotion, qualification, a certificate, a build packet, and paired statistics computed from stored results (a study assembled outside `eval.compare`, among them a stored study read by `study.compare`). N37-REF, N28-REF and VERBS-eval state the same division |
| 247 and 894 (a numerical profile that does not fit the request) | 247 is admission's code (N35-04 steps 2 and 4, N35-07): the determinism class the verb requires is not attainable on the placement. 894 is the snapshot container's code (N29a-18; N29-REF's row): a snapshot restored under another numerical profile and labelled a continuation, raised by `sim.replay` (BD-203), its teleport (BD-204) and `run.fork` (BD-205). A restore across profiles is never 247 |
| 861 and 867 (a D4 training result cited without re-evaluation, the learning chapter's code) | A number observed during the run (a learning curve, a training return) is 861 wherever it is cited (the plan's known answers KA-801 and KA-802 name 861 for a cited training return). A D4-trained policy without re-evaluation is 867 where a `frob-learn` verb or record builder is the consumer (`train.export` summaries, the training-gate record builder, a ladder score) and 861 at every other citation point. The learning registry N23-REF states the same split in its row of 867 |
| 562 and 584 (attribution for a failure neither lane admits, or under an uncontrolled class) | 562 is the licence check on the source run's class at admission of a fork or a re-run, and fires before any code of the counterfactual-debugging chapter; 584 is raised at attribution time, and its class clause concerns a fork set whose own children are of class D4 or unrecorded. The registry N27-REF states the same split |
| 864 and 605 (the CPU-versus-device check is not current for a kernel in the plan) | 605 refuses to run on the device; 864 refuses to cite a device result that already exists |
| 248 and 244 (stiffness admission on single-precision tiers) | 244 is the admission-time estimate from the interface card; 248 is the run-time measurement κ̂·u |
| 761 and the `70x` budget codes | 701 to 703 are per-session and per-job budgets and the admission guard; they never concern the envelope. 704 (a campaign-class job without an approved compute envelope) and 761 both concern the envelope and are divided by raising site, so that one oversized job gets one code (convention 14): a job-submitting verb queued by the daemon is classified when the job is submitted, and a job in the `campaign` size class whose scope has no approved envelope is refused with 704 by the size classifier, before the envelope admission is reached. Every other case is 761, raised by `sim.estimate` and run admission before the job starts: a run executed in process by `frob` (where no size class exists) above the class in force for its scope (E0 for `adhoc`), a daemon job whose scope has an approved envelope and that still exceeds its per-run limits or cumulative device-hours, a child run verb above its launcher token's declaration, and a cited `envelope_scope` that contradicts the launcher's token. Both come before the session-budget check 701, so an agent is never sent to raise a budget for a job the envelope refuses anyway. Both exit 4 with `retry: after-change`. The surface registry N32-REF (owner of row 704) and N36-REF state the same division |

The plan defines each of these pairs in two chapters and does not divide them, so each reading above is a
DESIGN choice with one reason: one condition gets one code per raising site, so that an agent never has to
handle one event two ways. The registries of the partner codes carry the same text.

**The four performance rows.** Rows 761, 862, 863 and 866 are written here and nowhere else: this registry
owns their meaning, triggers, pointer, `observed` and `required` shapes, retry class, ranked repairs and exit
code. The performance program's bead N36-REF holds what the program does with them: the raising-site matrix
(which bead raises which code on which trigger), a test per trigger, the interim reporting of the
unit-ordering rejection, and the script `scripts/e2e/perf_refusals.sh`. Its tests read the repair order from
these rows. DESIGN, for the pointer of 862 and 866: the pointer is the field at fault inside the figure's block
or the experiment record, not the figure or the experiment as a whole, because a repair acts on a field and
the interim ordering rejection must be able to point at `rates.ordering`.

**Exit code of 761.** The exit table assigns 4 to budget refusals and names `70x`; 761 is a budget refusal at
admission in the `76x` range, which the table does not name. Used here: exit 4. The registry owner's exit rule
(N35-33, a dependency of this bead) gives `76x` exit 4, so the row loads; without that rule the loader would
reject it as `ExitMismatch`, and if the rule were changed this row would follow it.

**Registry row format.** One TOML table per code: `code`, `name`, `family`, `message_template`, `exit`,
`retry`, `pointer`, `observed`, `required`, `repairs` (ordered), `defined_in = "§35.11"`, `raised_by` (bead
keys), `build_time` (boolean).

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the fifteen rows, schema-valid, each with ordered repairs, one `retry` value and the exit code of the table.
- [ ] `cargo test -p frob-core` loads the registry; a duplicated code, an unknown code and a row whose exit is outside its family rule are load-time errors; the 761 row loads with exit 4 once the family rule names `76x`.
- [ ] Each row's `raised_by` names at least one bead whose test provokes the code; the registry check lists rows whose raising test is not yet present as `unbound`, never as passed.
- [ ] The three build-time rows (250, 251, 252) are raised by the toolchain with the same fields as a run-time refusal, and the build exits non-zero.
- [ ] The relationship table is present in the registry as `related` fields in both directions for each pair, including 761 and 704.
- [ ] Rows 761, 862, 863 and 866 carry the full trigger lists and ranked repairs above, and no other bead writes a second trigger or repair list for them (N36-REF reads these rows).
- [ ] No code is reused: the loader's retired-code list contains none of the fifteen.

## Test plan

### Unit tests
- `refusals_numerics_rows_load`: the fifteen codes parse; families are 2xx (6 rows), 5xx (2), 7xx (1), 8xx (6).
- `refusals_numerics_exit_codes`: 247 to 252, 561, 562, 861 to 866 map to exit 3; 761 maps to exit 4.
- `refusals_numerics_repairs_ranked`: every row has at least two repairs, in a fixed order; the first repair of 248 is `--precision stiff`; of 861 the re-evaluation command.
- `refusals_numerics_related_symmetric`: 561↔825, 561↔542, 561↔533, 247↔894, 861↔867, 562↔584, 864↔605, 248↔244, 761↔704 appear in both rows of each pair (the partner rows belong to other chapters; the test reads them from the same file), and the two sentences of each pair name different raising sites (N35-33's rule 5).
- `refusals_perf_rows_complete`: rows 761, 862, 863 and 866 list their triggers (six demand cases, the pretraining rule and the two launcher-token cases; seven; five; five) and their repairs in the order of the table (four, three, three, four); the 761 row names the scope order and the stage (after 602 and 704, before 701), its third repair cites `--envelope-scope`, and the 862 row's interim trigger carries `detail = "rates-violate-unit-ordering"`.
- `refusal_unknown_code_is_load_error`: a serialized refusal whose code is absent from the registry and from the allocation ledger fails `RefusalRegistry::load` (planted; the test computes the lowest such `2xx` number at run time rather than naming a literal, because 253 is the candidate number of the stream-entity collision in the first allocation round of N35-33).
- `refusal_exit_family_rule`: a planted 2xx row with exit 6 fails with `ExitMismatch`.

### End-to-end script and logging
No script of its own. The codes are provoked from the CLI by `scripts/e2e/num_profiles.sh` (N35-31: 247, 248,
249, 562, 864), `scripts/e2e/num_licences_d4.sh` (N35-32: 561, 562, 861), `scripts/e2e/num_summary_wording.sh`
(N35-28: 865), the ban corpus of N35-18 for the build-time 251 (KA-805) and `scripts/e2e/perf_refusals.sh` (N36-REF: 761, 862, 863, 866). Each logs a `refusal` event with the
code, the exit code, `pointer`, `observed`, `required` and the first repair, through the STD-E2E helper; a code
whose mechanism is absent is logged `skipped: capability absent`, never as passed.

### Known answers and negative controls
- KA-802 (the licence refusals fire on their planted cases: 561 twice, 562, 861); KA-805 (the lint raises 251);
  KA-801's control (citing a training return raises 861).
- Negative controls: a registry with a duplicated code; a row with a wrong exit; a refusal block with a
  misspelt field (a refusal about a refusal: the envelope schema rejects it).
- Determinism: the registry loads to identical rows on every platform; codes are integers and compared as such.

## Considerations

- The chapter states conditions, not repairs. The repairs above are DESIGN and are the registry's content until
  an owning mechanism's bead refines its own row; a refinement is a new revision of the row.
- Several codes describe nearly the same condition from two or three chapters (561 with 542, 533 and 825; 861
  and 867; 562 and 584; 761 and 704; 247 and 894). The reading above keeps one code per condition and per raising
  site; the other chapters' registry beads (N37-REF, N28-REF, N24-REF, N23-REF, N27-REF, N29-REF, N32-REF) state
  the same reading, and a change to one is a change to both.
- OPEN-NUM-4 (the limit behind 248; default κ̂·u above 0.1) is decided by N35-29; the row's `required` field
  reads the limit from the frozen record, never from a literal.
- A mission query that stops on budget returns the cells certified so far with the unresolved remainder and
  says the answer is incomplete; no cell is returned half-certified. That clarification of the second standing
  rule is tested in N35-26.

## References

§35.11; Appendix O (rows 247 to 252, 561, 562, 761, 861 to 866); §32.4 (exit codes); §36.5, §36.6, §36.7,
§36.14 (the performance rows). GATE-SPARK-17. KA-801, KA-802, KA-805.
