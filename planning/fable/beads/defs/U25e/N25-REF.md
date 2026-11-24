---
key: "N25-REF"
title: "Augur refusal-code registry: FROB-REF-846 to -859, -868, -874, -877, -881, -884, -887"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "augur", "frob-core", "frob-mission", "frob-cert", "solid", "registry", "review-notes-open"]
parent: "WP-42"
deps: ["BD-005", "BD-022", "STD-E2E"]
needs: []
related: ["BD-291", "BD-359", "N25a-13", "N25a-16", "N25c-14", "N25c-21", "N25c-30", "N25d-02", "N25d-06", "N25d-23", "N25d-31", "N25d-32", "N25d-41", "N25e-32", "N25e-45", "N28b-32", "N35x-02", "NAB-45", "NAB-46"]
gates: []
plan_refs: ["§25.24", "Appendix O.8", "§32.4", "§25.3.4", "§25.3.5", "§25.4", "§25.5", "§35"]
---
## Background

Every refusal of the system has a stable code, a message and ranked repairs. A refusal is a typed answer, not an
error message: it says what was at fault, what was observed against what was required, which claims are affected,
whether a retry can help and what to do next, so that an agent can repair without guessing. The machine-readable
registry is `registries/refusals.toml`; an unknown code is a load-time error and codes are never reused. The
envelope's `refusal` block carries `code`, `message`, `pointer` (the field or part at fault), `observed`,
`required`, `unit`, `affected_claims`, `retry` (`never`, `after-change`, `backoff`) and ranked `repairs`. Exit codes
of `frob`: 0 completed; 2 completed with findings or flags; 3 refused input or question (codes 1xx to 5xx, and 8xx
other than conflicts); 4 budget; 5 capability; 6 backend; 7 conflict; 8 accepted and still running; 9
indeterminate. Exit code 1 is never emitted on purpose.

Augur (mission queries and inverse design, crates `frob-mission` and `frob-synth`, with the certificate payload
checked by `frob-cert`) defines twenty codes in the evidence-and-protocol family. They guard the definition of a
mission answer: the quantifier prefix, the world family, causal information, chance constraints, the certificate's
typing, and the scope in which a certificate may be reused. This bead puts all twenty rows in the registry with
trigger, pointer, ranked repairs and exit code, and binds each to the bead whose tests raise it. Tier Solid.

Standing rules of the chapter, which are not codes and must hold everywhere: a failed training run is never an
exclusion; a solver failure is never an exclusion; an unresolved cell is never reported as either verdict; and a
query whose estimate exceeds its budget is refused under the budget family (exit 4) with the estimate attached. A
budget stop of a running mission query is not a refusal: it returns the certified cells with exit 0 when the
unresolved set satisfies the query's `resolution`, exit 2 with the flag `resolution-unmet` when it does not, and
exit 4 only when the budget ended before admission and the first checked cell (OPEN-AUGUR-7; owner: the Tether
lead; this is the default until decided).

## Scope

In: the twenty rows as data; message templates; pointers; ranked repairs; `retry`; exit codes; the binding to
raising beads; the registry tests. Out: the mechanisms that raise the refusals; the six refusals of the chapter
that have no code yet (five late rules and the geometric-phase query; N25e-45, numbered through the allocation
ledger of N35-33); the auditor's own codes (FROB-REF-823, -826, -829, -833, -834, -835, the Lemma registry); the
battery that triggers all twenty codes through the public verbs (N25d-32, script `augur_refusals.sh`) and the
refusal payloads with offending field and ranked repairs as emitted (N25d-31).

## Technical approach

The first repair in each row is the one the chapter's table offers; the further repairs are ranked after it
(DESIGN, from the rules the code enforces). Every row: exit 3; `retry: after-change`.

| Code | Refused because (trigger) | Pointer | Ranked repairs | First needed | Raised and tested in |
|---|---|---|---|---|---|
| FROB-REF-846 | A chance constraint lacks its population, event or confidence; or marginal stage rates are multiplied without a dependence assumption | The `prob` binder, or the stage list | (1) The joint-outcome request; (2) declare the missing population, event or confidence; (3) allocate additive failure budgets that sum to the mission allowance; (4) state conditional success of at least the rate at every history that reaches the stage | M1 | BD-128; BD-136 (interim use: N25e-34) |
| FROB-REF-847 | An observation has no arrival instant, or a policy uses information before it arrives | The observation field | (1) The instant at which it becomes usable; (2) declare the arrival instant; (3) move the decision after the arrival, or drop the field from the policy's arguments | M1 | BD-126; BD-128; N25e-28; N25e-29 |
| FROB-REF-848 | A proof kind is unknown, at an unknown version, disabled, or used outside its scope | The proof's `kind` | (1) The kinds admitted for this claim; (2) re-emit with an admitted kind; (3) upgrade the auditor if the bundle names a newer registry | M1 | BD-133; BD-135; BD-136 |
| FROB-REF-849 | Certificate arithmetic is malformed: a non-reduced rational, a size limit exceeded, an interval without a declared rounding | The number or payload | (1) Re-encode (reduce, declare the rounding); (2) split the cell so the certificate fits the limits; (3) leave the cell unresolved | M1 | BD-133; BD-135; BD-136; N25e-10 |
| FROB-REF-850 | A frame, unit, sign or port order in a certificate does not match its definitions (a first moment passed as a torque; reordered feet) | The typed field that disagrees | (1) The typed field that disagrees, with the expected typing; (2) re-emit with the definitions of the bundle; (3) correct the definitions if they are wrong, which changes the bundle identity | M1 | BD-133; BD-135; BD-136 (interim use: N25e-38) |
| FROB-REF-851 | A certificate assumes an authority the design does not declare, or omits a force floor or other active bound | The authority tag or the missing bound | (1) Re-solve under the declared authority; (2) declare the authority the witness needs, if the design has it; (3) add the omitted bound and re-solve | M1 | BD-130; BD-133; BD-135; BD-136; N25e-29 |
| FROB-REF-852 | A monotone or symmetry transport lacks a side condition | The transport proof | (1) The missing sub-certificate; (2) evaluate or certify the target cell directly; (3) leave the target unresolved | M3 | BD-288; N25e-07 (interim use: N25e-02); N25d-02 (`gait.gradient`: a symmetry transport across an event with no licence; M6) |
| FROB-REF-853 | A minimum is claimed without a matching lower bound, or an unattained infimum is reported as attained | `objective.claim` or `cost_bounds.attained` | (1) `feasible-candidate` with its gap; (2) supply the lower bound; (3) report the open threshold with a feasible priced candidate and its gap | M1 | BD-128; BD-135 (interim use: N25e-42) |
| FROB-REF-854 | A classifier, screen or certificate is reused outside its eligibility: support, deadline, error units or tolerance changed | The reused artifact and the changed field (`pointer` at the first differing field of the query, for the three-gain docking amendment `/world/hidden/g`; `observed` the artifact's value, `required` the query's) | (1) `mission.revalidate` (the ready command is rendered for a verb that answers in the running build, N25d-31: before M3 `frob result revalidate a:… --against q:…`, the M1 surface of the same engine, N25a-13; from M3 `frob mission revalidate a:… --after-edit e:…`, N25a-16); (2) recompile the screen for the new support; (3) restore the original eligibility | M1 | N25c-21 (the eligibility binding and the `--reuse` check of `mission.synthesize` and `mission.refute`, on the docking family of BD-129; M1, the first emitter); N25e-32 (the gain interval, M1); BD-292 (the revalidation engine); N28b-32 (`result.revalidate` lists it); N25c-14 (`--from` an answer outside its reading, law or layout; M3); BD-291 (`mission.distinguish`: a stored probe rule applied to a target outside its mechanism set; M3) |
| FROB-REF-855 | Evidence profiles are mixed inside one cell, or a probability mass is requested without a supplied weighting | The cell, or `resolution` | (1) Split the cell; supply the weighting; (2) choose one evidence profile for the cell; (3) report the unresolved set without a mass | M1 | BD-128 |
| FROB-REF-856 | A region certificate is requested over more parameters than the admitted limit, or a joint paving is requested | The constraint or the request | (1) The per-constraint decomposition; (2) weak-variable enclosure with a split cap, where admitted; (3) pointwise checks at proposed designs | M1 (admission trigger), M3 (pilot), M4 (region-certificate trigger) | Admission trigger: N25a-02, asserted by N25a-23 (case a12) and N25d-32; region-certificate trigger: BD-289; N25b-10; BD-358; N25e-24; N25e-26 |
| FROB-REF-857 | A synthesis system is outside the symbolic caps or is not zero-dimensional | The system | (1) A smaller template, or the continuation lane with its label; (2) fix parameters to reduce the system; (3) multi-start numerical optimization, classed `Tested` | M4 | BD-361 |
| FROB-REF-858 | Stage composition fails: an exit set not inside the next entry set, a clock mismatch, or a resource spent twice | The violating state or interval | (1) The violating state or interval; (2) tighten the first stage's exit or widen the second's entry and re-certify; (3) repair the clock or resource ledger | M4 | BD-128; N25c-30 (`mission.synthesize`: a declared stage chain of the facility query that fails composition; M8) |
| FROB-REF-859 | An experiment or world edit cannot be realized through the declared interfaces, or does not declare the causal path it changes | The experiment or edit | (1) The nearest realizable intervention; (2) declare the causal path (geometry, material, optics, clocks or actors); (3) the direct measurement at mission conditions | M3 | BD-291; BD-293 |
| FROB-REF-868 | The `.fmission` file is not valid: unknown field, missing unit, misspelt parameter, number not an exact string | The field (JSON pointer) | (1) The schema diff; (2) write numbers as exact strings ("p/q"; integers above 2⁵³ never as JSON numbers); (3) `mission.check` before submitting | M1 | BD-126 (interim use: N25e-28) |
| FROB-REF-874 | The world family is empty, or its scope changed after candidates were tested | The emptying constraint, or the changed scope | (1) The emptying constraint; (2) relax or correct it; (3) for a changed scope: open a new query, since the world family is part of the mission identity | M1 | BD-126; BD-128 |
| FROB-REF-877 | The predicates conflict with the world family for every design | The conflicting predicates | (1) A minimal conflicting subset, with its Farkas certificate where the conflict is linear; (2) relax one predicate of the subset; (3) narrow the world family and say so | M1 | BD-126; BD-128 |
| FROB-REF-881 | Evidence or a certificate establishes a different quantifier prefix than the query's, including clairvoyant evidence offered for a robust claim | The proof's `establishes` field | (1) The prefix it does establish; (2) re-prove under the query's prefix; (3) keep the clairvoyant result as a labelled relaxation, which can exclude and never include | M1 | BD-128; BD-131; BD-136; BD-359 (`mission.refute --baseline`: a controller retuned or trained per body offered to the fixed-controller comparison, whose prefix binds no policy; `pointer` at the controller's entry in the `comparison` block; M4) |
| FROB-REF-884 | No quantifier prefix, or a hidden parameter with no persistence declaration | `quantifiers`, or `world.persistence` | (1) The two candidate declarations and the fixture that separates them; (2) declare the prefix; (3) declare `fixed-per-execution` or `resampled` at a named instant | M1 | BD-126 |
| FROB-REF-887 | A policy is indexed by a parameter the controller cannot observe | `policy_class.indexed_by` | (1) Move the parameter into the world, or add the calibration observation; (2) index by the known nominal design | M1 | BD-126; BD-128 |

Notes on particular rows.
- 877: the minimal conflicting subset is found at admission by deletion filtering over the linear and interval
  relaxation with every design constraint relaxed; Solid for linear conflicts, Frontier beyond.
- 881 and 826: the auditor reports a prefix, domain or assumption mismatch between certificate and query as
  `FROB-REF-826`; the mission verbs present a prefix mismatch as 881 and cite 826 as the cause (N25e-45 records
  the mapping). The pusher pair is the fixture: for every known gain the command u = 1/g works, and no single
  command works for the family.
- 848 to 851: the auditor (BD-132) never reports these codes; it reports its own (FROB-REF-823, -829, or
  the rule only). The mission verbs raise them through the rule-to-refusal mapping of BD-133's check before an
  answer returns: in a withheld cell's explanation when the verb's own proposer fails the check, and as a
  refusal (exit 3) when evidence supplied by the caller fails it (`--from a:…`, a supplied proof, a rival
  bundle), which BD-135 lists among the refusals of `mission.synthesize` and `mission.refute`.
- 884: the separating fixtures are the pusher pair (prefix) and the out-and-back motion with a hidden gain in
  {1, 2}: net displacement 0 when the gain is fixed per execution, {−1, 0, 1} when resampled per move.
- 857: the caps are those of the symbolic sibling's real solver (8 variables, 64 equations, 4,096 terms, total
  degree 256, 4,096-bit coefficients, 2 × 10⁸ work units, 512 MiB, zero-dimensional ideals only).
- The chapter's sentence that its codes use the block 840 to 859 is not exact: 840 to 845 belong to the
  qualification lane and six of these codes lie outside the block. The registry governs.
- Six refusals of the chapter have no code (five stated by the late rules, one by the geometric-phase query):
  N25e-45 fixes their content and interim mapping, and the allocation ledger gives their numbers.
- 856 has two triggers: the admission trigger (a query whose `evidence` is `enclosed` and names a joint paving,
  refused by admission at M1) and the region-certificate trigger (more parameters than the split cap without
  weak-variable treatment, or a joint paving asked of the region mode, from M4). Both use the same code, pointer
  and first repair.

## Acceptance criteria

- [ ] `registries/refusals.toml` contains the twenty rows with message template, pointer kind, `retry`, ranked repairs and exit code 3; the loader accepts them; a duplicate or unknown code fails the load.
- [ ] Every row names its raising beads (both triggers for 856) and its smallest triggering fixture; `binding_names_known_beads` passes. The triggers themselves are asserted by the raising beads (N25d-31 per code, N25d-32 through the public verbs), which depend on this bead.
- [ ] The exit-code map test passes: every one of the twenty maps to exactly one exit code.
- [ ] The generated agent documentation lists each code with its message and repairs.
- [ ] The standing rules are written in the registry's preamble and in the generated documentation (a solver failure and a failed training run never produce an exclusion; a budget stop returns certified cells with exit 0 or 2, never a partial result under exit 4); they are enforced and tested in N25d-31.
- [ ] `scripts/e2e/augur_refusal_registry.sh` passes with the summary above.

## Test plan

### Unit tests

`crates/frob-core/tests/refusal_augur.rs`: `twenty_rows_load`; `codes_unique`; `all_exit_3`;
`repairs_nonempty_and_ordered`; `first_repair_matches_table` (each row's first repair equals the chapter's);
`message_templates_render` (each template with a sample pointer and observed value); `binding_names_known_beads`
(every key in the last column is a key of the tracker index); `two_triggers_of_856` (the row lists both raising
sites). The smallest triggering fixture of each code that the mission core raises at M1 is fixed here, for the
raising beads to assert (N25d-31's `crates/frob-mission/tests/refusals.rs`, which also tests the four standing
rules, and N25d-32's battery): 846 (twenty worlds, two stages at 19/20: a product claim of 0.9025 without a
dependence assumption); 847 (an observation field with no arrival instant); 853 (the docking family at ε = 5/16,
where repair needs U > 3/4 and no smallest increase exists, reported as attained); 855 (a cell mixing `exact` and
`empirical`); 856 (an `enclosed` query naming a joint paving); 868 (a number written as a JSON float); 874 (a
world box with lower end above upper end); 877 (required y ∈ [3/4, 5/4] with a world that forces y ≤ 1/2); 881
(per-gain pusher commands offered for the shared-command query); 884 (no `quantifiers`; a hidden gain with no
persistence); 887 (a policy indexed by the hidden gap d); 854 (the two-gain bundle offered with `--reuse` to the
three-gain docking mission, N25c-21: `pointer` `/world/hidden/g`, `observed` {1, 2}, `required` {1, 33/32, 2},
nothing published, and the first repair the M1 ready command `frob result revalidate a:… --against q:…`).

### End-to-end script and logging

`scripts/e2e/augur_refusal_registry.sh`: the registry seen from the user's side. It runs
`frob schema --refusals --json` and asserts that the twenty Augur rows are present with message template,
pointer kind, `retry: after-change`, ranked repairs (first repair equal to the table's) and exit 3; renders each
message template through `frob schema frob.refusal/1 --json` validation; checks that the generated agent
documentation lists each code with its message and repairs; and asserts the negative controls below from the
command line (a scratch registry with a duplicated code makes `frob schema --refusals` fail to load with the
duplicate named). It needs no mission mechanism, as befits a registry that comes first. The battery that triggers every code
through the public verbs, with the fixtures above and the certificate mutations (849: a non-reduced rational;
850: (60, 30) passed as the torque, and a swapped foot order; 851: a `passive` authority tag on a certificate that
assumes independent forces, and an unreported force floor; 852: a transport with one inclusion condition
removed; 854: the two-gain screen on a three-gain mission), is `scripts/e2e/augur_refusals.sh` of N25d-32; this
bead does not write a second script of that name. Logs through the STD-E2E helper to
`target/e2e-logs/augur_refusal_registry/<utc-timestamp>.ndjson` with a transcript: `ts`, `script`, `bead`
(`N25-REF`), `step`, `event`, exact command, exit code, wall time, and a `refusal` line per triggered case with
code, message, pointer, observed, required, retry and repairs; one `assert` line per row checked; the `summary`
gives rows present (threshold 20), rows with exit 3 (threshold 20) and the duplicate control caught.

### Known answers and negative controls

Fixture values quoted above (the pusher pair; twenty worlds: 18/20 or 19/20 against 0.9025; docking equality at
U = 3/4, ε = 5/16; the stance torque (30, −60, 0) N·m against a packet that passes (60, 30)). Related registry rows:
KA-628, KA-630, KA-631, KA-637; CX-272, CX-276, CX-282. Negative controls: a registry with a duplicated code; a
verb that returns exit 4 for a protocol refusal; a refusal emitted with no repairs. Determinism: the registry and
its generated documentation are byte-identical across hosts.

## Considerations

- Repairs are ranked for an agent: the first is always the piece of information that lets it fix the request in
  one step.
- Codes whose mechanisms arrive later (852, 856 to 859) are registered at M1 so that the registry is complete;
  their triggers are tested at the milestones shown.
- A refusal is never a verdict about a design: none of these codes marks a design as excluded.

## References

§25.24, Appendix O.8, §32.4; FROB-REF-846 to -859, -868, -874, -877, -881, -884, -887; OPEN-AUGUR-7.
