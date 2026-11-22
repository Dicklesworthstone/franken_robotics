---
key: "N39-REF"
title: "frob-conformance: the refusal registry of the ten-fold test: product codes and harness refusals"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tenfold", "gauntlet", "frob-conformance", "solid", "registry"]
parent: "WP-60"
deps: ["BD-005", "BD-025", "STD-TEST"]
needs: ["Appendix O and §35.11: `registries/refusals.toml`, the loader that makes an unknown code a load-time error, and the allocation of one new code in the 5xx (study) block for `suite-too-small-for-population-claim`", "§39.1: the harness driver in `frob-conformance/tenfold/` that emits the harness's own refusal records"]
related: ["N39-KA", "N39-CX", "N39b-34", "N39b-01", "N39b-18", "N39b-15", "BD-124"]
gates: ["GATE-TENFOLD-03"]
plan_refs: ["§39.17", "§39.20", "§39.14", "§39.10", "§39.13", "§39.21", "§39.22", "§39.8", "§24.14", "§35.11", "Appendix O"]
---
## Background

In this program a request that cannot be honoured is refused with a stable code, a message and ranked repairs; a
refusal is an answer, not an error. The ten-fold test refuses in two places. First, two product verbs carry part
of its protocol (`eval.preanalyze` for a ten-fold study plan, and the speed-comparison registration that
enforces "ceiling before speed"), and they refuse with product codes `FROB-REF-nnn` from the program's refusal
registry (codes are never reused; an unknown code is a load-time error). Second, the harness itself, which has no
product verbs, refuses or voids through typed records in the gate's event log. The chapter that defines the test
assigns no numeric codes of its own; it names behaviours ("the run is refused", "refused as unreachable", "a kit
with a private input is refused"). This bead is the registry that binds each behaviour to an existing product
code where the registry already has one, registers the one product refusal that has none, and names the
harness's typed refusals. DESIGN: harness refusals use slugs `tenfold.<name>` and are not product codes, because
the harness is not a product surface and its records travel in the replication kit, not through `frob`.

## Scope

In: the table below as data (`registries/refusals.toml` rows for product codes;
`crates/frob-conformance/src/tenfold/refusal.rs` for harness slugs), messages, ranked repairs, exit codes, and one
test per row. Out: the mechanisms that raise them (their beads are named in the last column).

## Technical approach

Exit code: 3 for every typed refusal, from `frob` and from the harness driver alike; a void is a recorded event
with exit 0 for the command that records it and a `void` status on the cell, arm or gate. Every refusal record
has `code` or `slug`, `message`, `pointer` (what was refused), `observed`, `required`, and `repairs` in rank order.

**Product codes used by the test.**

| Code | Meaning | Trigger in the ten-fold test | Ranked repairs | Raised by |
|---|---|---|---|---|
| New code, 5xx block, slug `suite-too-small-for-population-claim` | A ten-fold study plan declares a population result with fewer independent families than a valid one-sided bound needs | `eval.preanalyze` on a plan with result type `population` and n < ⌈ln(1/α)/ln(1.1)⌉ (32 at 0.05, 46 at 0.0125, 49 at 0.01), or n below the count the plan's registered bound requires (182 at 0.05 for the bounded paired contrast) | 1. Declare a finite-suite result. 2. Enlarge the cell to the necessary count. 3. Name fewer confirmatory cells so each has a larger level. 4. Register a bound with a smaller required count and its proof | N39b-34 |
| FROB-REF-532 | A fixed-n study has no pre-analysis, or names a verdict it cannot reach | A confirmatory cell of four families (4 of 4 has tail 0.0625); a cell opened without its recorded pre-analysis | 1. Open the cell as `descriptive`. 2. Enlarge m until the guard is reachable and the power conditions hold. 3. Remove the cell from the confirmatory set | N39b-34, N39b-25 |
| FROB-REF-552 | A request mixes study types | The fixed-n verdict is requested at a monitor's stopping time, or a monitor output is offered as an input to the fixed-n computation | 1. Wait until n and compute once. 2. At the partial gate, read the monitor's own `withdrawn` on its own authority. 3. At the tournament, continue: the stop is disabled | harness statistics; N39b-23 |
| FROB-REF-866 | A speed experiment requested without its computed ceiling, or with a ceiling below its goal | A registered speed comparison lacks p or (f, r, h) from a `ledger-closed` run of the strong baseline, or its ceiling is below the ratio it sets out to show | 1. Record the shares from a closed baseline ledger. 2. Lower the goal below the ceiling. 3. Fund the work for trust, coverage or correctness with that reason and no speed figure | N39b-09 |
| FROB-REF-899 | A private or non-redistributable input to a kit | A replication kit whose lineage contains a private scene or dataset, or incumbent code | 1. Replace the input by its generator seed and recipe. 2. Pin the revision and give fetch instructions. 3. Exclude the stratum from the kit and print it `not replicable` | N39b-18 |
| FROB-REF-862 | A performance figure without a complete disclosure block | Any baseline or gate figure missing a field of the block | 1. Re-record with the block. 2. Publish as context, not as a Performance claim | BD-143, N39b-08 |
| FROB-REF-863 | A performance ratio without a paired lower bound | A reported ratio without [L, U] | 1. Compute the family bootstrap bounds. 2. Report the cell as `descriptive` | N39b-14 |
| FROB-REF-865 | A summary statistic that omits what it excluded | A ratio reported without its count of excluded pairs and censored runs | 1. Print the counts. 2. Recompute with every run | N39b-14 |
| FROB-REF-761 | A job exceeds the compute envelope in force | A gate campaign above the approved class | 1. Obtain approval of the campaign class. 2. Run the cells that fit and record the rest `not run` | N39b-29 |
| FROB-REF-840 | A session without the sealing capability asks for sealed confirmation data before close | An arm's agent or tool reads a sealed card, world address, per-world outcome or probe point | None for the arm: the attempt is logged and audited as a possible leak | N39b-02 |
| FROB-REF-860 | An export without the no-authorization flags, or an attempt to produce a true flag | A kit artifact without `execution_authorized`, `hardware_safety_signoff`, `sim_to_real_certified` all false | 1. Regenerate through the exporting path | N39b-18 |

**Harness refusals and voids (DESIGN, slugs).**

| Slug | Meaning and trigger | Effect | Ranked repairs | Raised by |
|---|---|---|---|---|
| `tenfold.freeze-after-open` | A paired run is requested while an arm's freeze manifest is missing, unpublished, or dated after the suite opening | The run is refused | 1. Run at the next gate on a rotated suite. None within this gate | N39b-01 |
| `tenfold.arm-changed` | A frozen arm's manifest or pack digest differs from the published one | The arm is void for the gate (disqualifying ground) | 1. Re-freeze and run at the next gate | N39b-01, N39b-37 |
| `tenfold.role-overlap` | A person or session holds two of the roles builder, rival advocate, adjudicator | The action is refused | 1. Assign the action to a session of the right role | N39b-23 |
| `tenfold.record-edited` | A ledger or receipt fails its hash chain | Disqualifying ground for the arm that holds it | None | N39b-01 |
| `tenfold.adjudicator-in-tuning` | An arm's logs show the adjudicating simulator in development or tuning | The arm is disqualified on that stratum | None within the gate | N39b-01 |
| `tenfold.parameters-frozen` | A change to 3, 10, 5, 1.5, 0.05 or the four-cell limit after the first suite hash | Refused | 1. Propose for the next gate, before its suite hash | N39b-05 |
| `tenfold.retest-on-same-suite` | A cell with a kill outcome is submitted again on the same sealed suite | Refused | 1. Measure again at the next gate on fresh families | N39b-05 |
| `tenfold.family-retired` | A family that served a gate, or was touched by a leak, is offered for sealing | Refused | 1. Author a new family | N39b-23 |
| `tenfold.audit-incomplete` | Sealing is requested before the rival audit's sign-off and pack digest are recorded | The suite is not sealed | 1. Complete the audit | N39b-37 |
| `tenfold.ablation-above-bound` | An ablation reports a ratio above 1/s or 1/(1 − s) | The stratum's ablations are void as an accounting error | 1. Re-trace the baseline. 2. Correct the touched-share accounting | N39b-45 |
| `tenfold.word-not-earned` | A document uses "ten-fold" for a cell without that verdict, population wording on a finite-suite result, or "independently accepted" under `self-adjudicated` | The document fails the wording lint | 1. Use the sentence the outcome rule prescribes | N39b-15 |
| `tenfold.kit-incomplete` | A kit's manifest of hashes does not cover a published number's inputs | The kit is refused for publication | 1. Add the missing records | N39b-18 |
| `tenfold.protocol-breach` | A pin moved during the freeze, sealed material read by an arm, or an arm changed after its freeze | The tournament is void and re-run | 1. Re-run on a rotated suite | N39b-13 |

## Acceptance criteria

- [ ] Every row has a test that produces it and checks code or slug, exit code and the order of repairs.
- [ ] The new product code is allocated in `registries/refusals.toml` (the number is the registry's next free
      one in its block; this bead does not pre-empt it) and the loader accepts it.
- [ ] No harness path exits non-zero without a refusal record; no product path returns an unregistered code.
- [ ] A cap breach is not a refusal: the run is censored at the instant of the breach and costs H.

## Test plan

### Unit tests

`cargo test -p frob-conformance tenfold::refusal`: one test per slug (for example
`freeze_after_open_is_refused_with_repairs`, `retest_on_same_suite_refused`,
`parameters_frozen_after_first_hash`); `all_slugs_documented` (the enum equals the table);
`cap_breach_censors_not_refuses`. Product codes are tested in the beads that raise them.

### End-to-end script and logging

`scripts/e2e/tenfold_known_answers.sh` (N39b-22) drives each refusal through `e2e_expect_refusal` (product codes)
or through the harness driver with an asserted slug, logging `event: refusal` with the ranked repairs.

### Known answers and negative controls

Known answers: the family counts 32, 46, 49 and 182 in the `required` field of the population refusal; the tail
0.0625 in the four-family refusal. Negative control: a build in which `tenfold.freeze-after-open` is downgraded
to a warning must fail the test that expects the run to be refused.

## Considerations

- The slug for the population refusal is used in tests until its number exists; tests match on the slug field.
- A disqualification is not a refusal: it is a recorded outcome with scope and evidence, and it awards nothing
  to the other arm.
- Refusal messages never contain sealed content.

## References

§39.17, §39.20, §39.14, §39.10, §39.13, §39.21, §39.22, §39.8, §24.14, §35.11; GATE-TENFOLD-03, -08, -09, -10.
