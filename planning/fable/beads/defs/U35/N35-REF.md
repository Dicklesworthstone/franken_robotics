---
key: "N35-REF"
title: "Numerics refusal registry: FROB-REF-247 to -252, -561, -562, -761, -861 to -866 with repairs and exits"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "spark", "frob-core", "solid", "registry"]
parent: "WP-01"
deps: ["BD-005"]
needs: ["§32.18: the verb registry's refusal list per verb and the test that every refusal code maps to exactly one exit code (the Tether verb beads)", "§36.5, §36.6, §36.7, §36.14: the raising sites of FROB-REF-761, -862, -863 and -866 in the performance harness and in `sim.estimate` (seed beads BD-216, BD-219, BD-354)"]
related: ["N35-KA", "N35-CX", "N35-04", "N35-06", "N35-07", "N35-18", "N35-26", "N35-28", "BD-013", "BD-018", "BD-053", "BD-176", "BD-179", "BD-216", "BD-219", "BD-269", "BD-354", "CARD-T-09"]
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
end-to-end sweeps that provoke each code from the CLI (N35-31 for 247 to 249; N35-32 for 561, 562, 861; the
performance harness's script for 761, 862, 863, 866).

## Technical approach

The chapter's table gives each code's condition. It gives no repair list, so the repairs are ranked here from
the rules each code enforces (DESIGN). `retry` is `after-change` for every row: none of these conditions clears
by waiting. "Build time" codes are raised by the kernel toolchain and by `xtask`, never at run time; the build
fails and the diagnostic carries the same fields.

| Code | Refused because (trigger) | Pointer; observed; required | Ranked repairs | Exit | Raised and tested in |
|---|---|---|---|---|---|
| FROB-REF-247 | The determinism class a verb requires is not attainable on the requested placement (for example a replay with a claim of bit equality on a device family that holds only class D3 against the CPU; a fork on a family with no current batch-independence record) | The verb and the placement; the class attained there with the record that says so; the class required and the use that requires it | (1) Run on a placement where the class is attained (the refusal lists them, `batch-cpu` first); (2) if the record is stale and not absent, `frob spark check --sweeps all` to make it current; (3) ask for the weaker use the licence table offers (a tolerance replay, `paired-by-address`, an unpaired comparison) by naming it in the request | 3 | N35-04, N35-07 |
| FROB-REF-248 | A solve is ill-conditioned beyond its precision profile (κ̂·u above the limit, default 0.1), under the `refuse` policy | The world address and step; κ̂ and κ̂·u (named a lower bound), the precision profile; κ̂·u ≤ limit | (1) `--precision stiff` (one refinement step with a double-float residual); (2) run the world on `batch-cpu` in double precision or on the referee; (3) soften the contact impedance or replace the near-rigid interface card; (4) `--on-degraded continue-flagged`, which keeps the world and marks it `Degraded: ill-conditioned` | 3 | The conditioning bead of the Spark unit; BD-179; N35-04 (the option); N35-29 (the limit) |
| FROB-REF-249 | An anchored offset left its range in one step (magnitude 16 m or more after integration), or a cell index left the scene bounds | The world address, body and axis; the offset or cell index; magnitude below 16 m and index inside the declared bounds | (1) `frob sim replay <episode> --tier batch-cpu` to locate the first divergence of the blown-up world; (2) reduce the step or check actuator and contact parameters (a body that moves 12 m in one step is not physical); (3) enlarge the scene's declared bounds if the body legitimately left them | 3 | The anchored-position bead of the Spark unit; BD-157; BD-179 |
| FROB-REF-250 | A kernel is not dimensionally homogeneous (build time): a sum of terms with different exponent vectors over the base dimensions, or a square root of an odd exponent vector | The kernel, the statement and both exponent vectors; equal vectors | (1) Fix the derivation (the diagnostic names the sum); (2) declare the missing conversion constant with its dimension; (3) none: the check is never suppressed | 3 (build failure) | BD-018 (the dimension checker); theorem card T-09 |
| FROB-REF-251 | A banned operation appears in kernel source (build time): a fast-math relaxation, a fused multiply-add (contracted or explicit), a built-in transcendental or reciprocal square root, a floating-point atomic | The file, line and token; the ban it falls under | (1) Replace by the admitted form: a `det_*` function, the split product, count then exclusive scan then write, a select; (2) move the computation to the host if it does not belong in a batched kernel | 3 (build failure) | BD-013 (dialect lint); N35-18 (ban table, compile-flag audit, corpus) |
| FROB-REF-252 | A strict predicate in floating point without an exact form or a certified guard (build time) | The kernel and the comparison; its predicate class; `exact` or `guarded` | (1) Express the predicate in the exact-integer kernel class on quantized inputs; (2) a guarded kernel: an `f32` residual with a derived error bound and an exact fallback; (3) if the requirement allows it, declare the comparison non-strict in the requirement (a change of the requirement, recorded as one) | 3 (build failure) | BD-053 |
| FROB-REF-561 | A paired statistic was requested with an arm of class D4 or of unrecorded class, or as paired across device families without class D1 | The arm; its class and profile, or both device families; both arms class D2 on one family with a current batch-independence record, or D1 across families, or D3 with a decision-equivalence record | (1) Re-evaluate the offending arm under `device-replay` or stricter; (2) run both arms on one device family, or on `batch-cpu`; (3) request the comparison as `unpaired`, with the larger sample that needs; (4) under class D3 with a recorded decision-level equivalence, request `paired-by-address` | 3 | N35-07; BD-269; BD-262 |
| FROB-REF-562 | Attribution or a counterfactual fork was requested below class D2 (the source run's class is D3, D4 or unrecorded, or the batch-independence record of its device family is not current) | The source run; its class and the age of the record; D2 with GATE-SPARK-15 current for the family | (1) Teleport the world to `batch-cpu` and fork there; (2) re-run the episode under `device-replay` on a family with a current record; (3) `frob spark check --sweeps all` to renew the record | 3 | N35-07; BD-269; the fork verb (BD-205) |
| FROB-REF-761 | A job exceeds the compute envelope in force (default E0: 16 GiB, 8 cores, one hour, one device) | The job; the class `sim.estimate` computed for it; the class in force | (1) Reduce the job (fewer worlds or steps), or split it into jobs that each fit; (2) request approval of the class printed, with its ceiling and kill rule written first; (3) take the registered not-approved fallback of the work package | 4 | BD-219 (`sim.estimate`); the envelope register |
| FROB-REF-861 | A number observed during a `fast-training` run (a learning curve, a training return) was cited as evidence, or a policy trained under it is offered for a claim with no re-evaluation under `device-replay` or stricter | The cited value and its run, or the policy; `trained_under: D4`; a re-evaluation record for the same `PolicyId` | (1) `frob eval run --policy <id> --determinism device-replay` and cite that evaluation; (2) retrain under `device-replay` if the training curve itself is what must be cited | 3 | N35-06; BD-269 |
| FROB-REF-862 | A performance figure without a complete disclosure block | The figure; the missing fields by name; the complete block | (1) Re-run through the performance harness, which fills the block; (2) supply the named fields from the run's receipt; (3) present the number as context without the Performance claim | 3 | BD-216 |
| FROB-REF-863 | A performance ratio without a paired lower bound | The ratio; the number of pairs and whether a bound was computed; at least ten pairs and the one-sided 95% bound exp(ℓ̄ − t·s/√n) | (1) Run at least ten pairs back to back in randomized order on one host state and report the bound; (2) state the two rates separately, without a ratio | 3 | BD-216 |
| FROB-REF-864 | A device result cited without a current CPU-versus-device check for its kernels on that device family, driver and compiler | The result; the stale or missing check rows with fingerprint digests; a current check for every kernel in the step plan | (1) `frob spark check` on that family; (2) `frob spark fingerprint` first if the fingerprint changed; (3) cite the `batch-cpu` result | 3 | N35-07 (the citation rule); BD-176 (currency) |
| FROB-REF-865 | A summary statistic that omits what it excluded (worlds marked `Degraded` or `Faulted`, unscored pairs, masked samples) | The statistic; its inputs counted against its denominator; the `excluded` table with counts by reason | (1) Regenerate through `frob telemetry summary`, which states exclusions; (2) add the counts by reason and the denominator | 3 | N35-28 |
| FROB-REF-866 | A speed experiment requested without its computed ceiling, or with a ceiling below its goal | The experiment; the ceiling (absent, or its value) and the goal; ceiling present and at least the goal | (1) Compute the ceiling from the current phase profile and attach it (removing a phase of share f gives at most 1/(1 − f)); (2) lower the goal to the ceiling or choose another lever; (3) do not run the experiment | 3 | BD-354; N35-05 (the profile's own ceiling) |

**Neighbouring codes, and the one-condition-one-code reading (DESIGN; the overlaps are listed for the lead).**

| Pair | Reading used |
|---|---|
| 561 and 825 (a result of class D4 offered as evidence) | A D4 arm at the admission of a paired comparison is 561, which names the arm and the pairing; 825 is the evidence boundary's code on promotion, qualification, certificates and build packets, and on a stored study assembled outside `eval.compare` |
| 861 and 867 (a D4 training result cited without re-evaluation, the learning chapter's code) | A number observed during the run is 861. A D4-trained policy without re-evaluation is 867 where a `frob-learn` verb is the consumer and 861 at every other citation point |
| 562 and 584 (attribution for a failure neither lane admits, or under an uncontrolled class) | 562 is the licence check on the source run at admission of a fork or a re-run; 584 is raised at attribution time |
| 864 and 605 (the CPU-versus-device check is not current for a kernel in the plan) | 605 refuses to run on the device; 864 refuses to cite a device result that already exists |
| 248 and 244 (stiffness admission on single-precision tiers) | 244 is the admission-time estimate from the interface card; 248 is the run-time measurement κ̂·u |
| 761 and the `70x` budget codes | 761 is the envelope class of the machine rule; the `70x` codes are per-session and per-job budgets |

**Exit code of 761.** The exit table assigns 4 to budget refusals and names `70x`; 761 is a budget refusal at
admission in the `76x` range, which the table does not name. Used here: exit 4, with the row marked for the
registry owner to confirm (a `76x` entry in the family rule of the loader is needed, or the loader rejects the
row as `ExitMismatch`).

**Registry row format.** One TOML table per code: `code`, `name`, `family`, `message_template`, `exit`,
`retry`, `pointer`, `observed`, `required`, `repairs` (ordered), `defined_in = "§35.11"`, `raised_by` (bead
keys), `build_time` (boolean).

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the fifteen rows, schema-valid, each with ordered repairs, one `retry` value and the exit code of the table.
- [ ] `cargo test -p frob-core` loads the registry; a duplicated code, an unknown code and a row whose exit is outside its family rule are load-time errors; the 761 row loads with exit 4 once the family rule names `76x`.
- [ ] Each row's `raised_by` names at least one bead whose test provokes the code; the registry check lists rows whose raising test is not yet present as `unbound`, never as passed.
- [ ] The three build-time rows (250, 251, 252) are raised by the toolchain with the same fields as a run-time refusal, and the build exits non-zero.
- [ ] The relationship table is present in the registry as `related` fields in both directions for each pair.
- [ ] No code is reused: the loader's retired-code list contains none of the fifteen.

## Test plan

### Unit tests
- `refusals_numerics_rows_load`: the fifteen codes parse; families are 2xx (6 rows), 5xx (2), 7xx (1), 8xx (6).
- `refusals_numerics_exit_codes`: 247 to 252, 561, 562, 861 to 866 map to exit 3; 761 maps to exit 4.
- `refusals_numerics_repairs_ranked`: every row has at least two repairs, in a fixed order; the first repair of 248 is `--precision stiff`; of 861 the re-evaluation command.
- `refusals_numerics_related_symmetric`: 561↔825, 861↔867, 562↔584, 864↔605, 248↔244 appear in both rows of each pair (the partner rows belong to other chapters; the test reads them from the same file).
- `refusal_unknown_code_is_load_error`: a serialized refusal with code 253 fails `RefusalRegistry::load` (planted).
- `refusal_exit_family_rule`: a planted 2xx row with exit 6 fails with `ExitMismatch`.

### End-to-end script and logging
No script of its own. The codes are provoked from the CLI by `scripts/e2e/num_profiles.sh` (N35-31: 247, 248,
249), `scripts/e2e/num_licences_d4.sh` (N35-32: 561, 562, 861, 864), `scripts/e2e/num_summary_wording.sh`
(N35-28: 865) and the performance harness's script (761, 862, 863, 866). Each logs a `refusal` event with the
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
- Three pairs of codes describe nearly the same condition from two chapters (561 and 825; 861 and 867; 562 and
  584). The reading above keeps one code per condition and per raising site; it is flagged for the lead in this
  unit's coverage file, since the other chapters' registry beads must agree.
- OPEN-NUM-4 (the limit behind 248; default κ̂·u above 0.1) is decided by N35-29; the row's `required` field
  reads the limit from the frozen record, never from a literal.
- A mission query that stops on budget returns the cells certified so far with the unresolved remainder and
  says the answer is incomplete; no cell is returned half-certified. That clarification of the second standing
  rule is tested in N35-26.

## References

§35.11; Appendix O (rows 247 to 252, 561, 562, 761, 861 to 866); §32.4 (exit codes); §36.5, §36.6, §36.7,
§36.14 (the performance rows). GATE-SPARK-17. KA-801, KA-802, KA-805.
