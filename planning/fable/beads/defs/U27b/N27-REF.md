---
key: "N27-REF"
title: "Autopsy refusal-code registry: FROB-REF-580 to -584, -586, -587 and -880 with triggers, repairs and exit codes"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "autopsy", "frob-core", "frob-autopsy", "frob-chronicle", "solid", "registry"]
parent: "WP-51"
deps: ["BD-005", "BD-022", "N32a-03"]
needs: []
related: ["N27a-02", "N27a-07", "N27a-10", "N27a-15", "N27b-01", "N27b-02", "N27b-03", "N27b-04", "N27b-07", "N27b-12", "N27b-15", "N27b-18", "N27b-24", "N27b-25", "N27b-28", "N29-REF", "N35-REF"]
gates: []
plan_refs: ["§27.14", "§27.4", "§27.5", "§27.8", "§27.9", "§27.13", "§27.19", "§29.17", "Appendix O.5", "Appendix O.8", "§32.4", "§35.11"]
---
## Background

Every refusal of the system has a stable code, a message and ranked repairs. A refusal is a typed answer, not an
error message: it says what was at fault, what was observed against what was required, which claims are
affected, whether a retry can help and what to do next, so that an agent can repair without guessing. The
machine-readable registry is `registries/refusals.toml`; an unknown code is a load-time error and codes are
never reused. The envelope's `refusal` block carries `code`, `message`, `pointer` (the field or part at fault),
`observed`, `required`, `unit`, `affected_claims`, `retry` (`never`, `after-change`, `backoff`) and ranked
`repairs`. Exit codes of `frob`: 0 completed; 2 completed with findings or flags; 3 refused input or question
(codes 1xx to 5xx, and 8xx other than conflicts); 4 budget; 5 capability; 6 backend; 7 conflict; 8 accepted and
still running; 9 indeterminate. Exit code 1 is never emitted on purpose. Three standing rules: refuse, never
truncate; a budget stop publishes nothing partial as a result; a misspelt parameter is a refusal, not a default.

Autopsy (counterfactual debugging: forks, interventions, a placebo, attribution) defines eight codes. Seven are
in the family "learning and evaluation protocol" (5xx) and guard what a fork and an attribution may be; one is in
the family "evidence and protocol" (8xx) and guards what an attribution may be cited as. Four of them
(580, 581, 582 and 583) are first raised at M2 by the fork verb `run.fork`, which is why this registry is due
then; the others are first raised at M3. This bead puts all eight rows in the registry with trigger, pointer, ranked
repairs and exit code, and binds each to the beads whose tests raise it. Tier Solid.

## Scope

In: the eight rows as data; message templates; pointers; `observed` and `required` shapes; ranked repairs;
`retry`; exit codes; the partial-refusal convention; relationships to neighbouring codes; the binding to raising
beads; the registry tests.
Out: the mechanisms that raise the refusals; the end-to-end sweep that raises each from the CLI (N27b-19); codes
of other subsystems (listed in the second table for reference).

## Technical approach

The Autopsy specification's table gives each code's meaning. One repair is stated by the program (for FROB-REF-582: fork at
the earliest valid tick returned; or restart and settle); the other repairs are ranked here from the rules each
code enforces (DESIGN). Every row: exit 3 when the refusal is the answer to the whole request; `retry:
after-change` unless stated.

| Code | Refused because (trigger) | Pointer; observed; required | Ranked repairs | First needed | Raised and tested in |
|---|---|---|---|---|---|
| FROB-REF-580 | A continuation (replay, teleport, the null fork, a placebo) is requested from a checkpoint whose physical, clock, sensor or policy contract does not match the run, or from a checkpoint with a contract field missing (a packet queue stripped outright) | The checkpoint; the mismatching contract and its two identities; "all four contracts equal to the recorded run's" | (1) Fork from the nearest snapshot whose four contracts match (the refusal names it); (2) reconstruct the fork point by replay from the world address to the tick; (3) if the contract was changed on purpose, declare the change as an intervention in the change set, which is then classified, or request a comparison replay | M2 | BD-205 (`stripped_queue_refused_580`); BD-279 (`contract_mismatch_580`, at fork-handle admission); N27b-19 |
| FROB-REF-581 | A structural or design-parameter intervention (mass, inertia, geometry, a gear ratio, an actuator card, a sensor mount, controller gains or memory, policy weights) is requested at a mid-episode tick | The `do` string; `earliest_valid_fork_tick` (0) and `forced_by` (the checkpoint fields that force the restart, for example actuator thermal state, policy history, contact memory); "the authored initial condition" | (1) `frob autopsy rerun <run> --edit '<typed edit>' --worlds failing,sample:64`: the paired re-run from the episode start; (2) fork at the earliest valid tick returned; (3) `--diagnostic-force`, which runs the forced fork in the labelled diagnostic mode and answers a different question | M2 | BD-205; BD-279 (`gear_change_refused_581`); N27b-04; N27b-18; N27b-19 |
| FROB-REF-582 | State from one model grafted onto another: a fork that asks to reuse state its change set invalidates (the contact warm start of a 60 mm wheel on the 75 mm design at a lip). Reached three ways: a `--reuse` field the change set invalidates, on `run.fork` and on `autopsy.intervene` (where it refuses only the interventions that invalidate the field, which is the `pointer`); a checkpoint of a run under another `ModelId` (`run.fork`); and, under `autopsy.intervene --diagnostic-force`, an edit that changes the kinematic structure or the collision geometry (a wheel radius, an added limb, a moved mount) | The change set; the field whose reuse is invalid (for example the contact memory), or the structure or geometry digest that changed; "a prefix identical by construction" | (1) Fork at the earliest valid tick returned; (2) restart and settle (the edited design is settled again from its own reset cache) | M2 | BD-205 (`reuse_graft_refused_582`); BD-279 (`warm_start_graft_refused_582`, through `--reuse`); N27a-15 (`forced_fork_wheel_radius_refused_582`); N27b-04; N27b-19 |
| FROB-REF-583 | The null fork does not reproduce its parent: the empty intervention does not replay bit for bit on the originating profile, and the request would build on that fork point (children asked of `run.fork`; a fork handle whose null replay is `mismatch` given to `autopsy.intervene`, refused at fork-handle admission before anything is restored; `autopsy.attribute` and `autopsy.repair` on such a fork). Opening or re-reading the fork point is not refused: `run.fork` reports `null_replay: mismatch` with exit 2 (BD-205) | The fork; the first divergent tick and stage with both digests; "identical digests" | (1) `frob telemetry divergence <parent> <null fork>` to name the first divergent tick, stage and variables; (2) fork from another stored snapshot, or by replay from the address, to rule out a damaged checkpoint; (3) re-record the run under a replay profile and file the determinism violation with both digests; (4) `frob explain run <run> --budget 600`: the first-anomaly ordering, labelled `Estimated`, the fallback of the attribution decision policy, whose trigger a replay mismatch is (N27b-12). Nothing is attributed until one of the first three succeeds | M2 (`run.fork` refuses to spawn children; the attribution verbs from M3) | BD-205 (`spawn_refused_on_mismatch_583`); N27b-03 (`null_fork_stripped_queue_refused`); BD-279 (`null_mismatch_handle_refused_583`); BD-280 (`refuse_583`); N27b-14 (`propagates_583`); N27b-19 |
| FROB-REF-584 | Attribution requested for a failure that neither lane admits (not decision-equivalent in double precision and not admitted to the single-precision lane: `precision-sensitive`), or under an uncontrolled determinism class (a fork set with a child of class D4 or of unrecorded class). `autopsy.intervene` raises it at admission from the precision route stored with the fork handle (N27b-07): a failure that is not decision-equivalent in double precision, under `--lane f64`, or under `--lane auto` in a build without the single-precision lane; the attribution and repair verbs raise it on the stored route | The fork; the equivalence table (verdict, class, atom, failure time against its envelope) and the lane's admission figures (ρ₀ at 10⁻⁷, G(W)·u, windows tried), or the offending class; when the lane's figures are absent, `observed.lane` says why: `"not-requested"` (the caller forced double precision with `--lane f64`) or `"not-built"` (a build without the single-precision lane); "decision-equivalent in double precision, or admitted to the single-precision lane, at class D2 or better" | (1) Fork closer to the failure (`--dt 0.5s`, then `0.25s`) and try again; (2) read the artifact guard's verdict for the world: a failure that double precision does not reproduce is an artifact candidate; (3) re-record the episode from its start on `batch-cpu` in double precision and attribute the failure there if it occurs; (4) `frob explain run`, the first-anomaly ordering labelled `Estimated`. For the class clause: re-run the forks under a profile with a current class-D2 record | M3 | N27b-07 (`refusal_584_fields`, `uncontrolled_class_refused`); BD-279 (`not_equivalent_refused_584_lane_f64`, at admission); N27b-24; N27b-25; BD-280 (`refuse_584`); N27b-14 (`propagates_584`); N27b-19 |
| FROB-REF-586 | An intervention changes more than one declared mechanism (a wet floor that changes both friction and depth returns), and no split is declared | The `do` string; the mechanisms with the changed fields of each; "exactly one mechanism" | (1) The split interventions that would be admitted, written out as `--do` strings; (2) a narrower factor path that touches one mechanism; (3) declare the split in the mechanism registry if the two parts are separately meaningful | M3 | N27b-02 (`wet_floor_refused_without_split`, `coalition_hidden_interaction`); BD-279; N27b-14 (a candidate coalition that changes a mechanism none of its members declares); N27b-19 |
| FROB-REF-587 | A design-parameter re-run with a policy that cannot act on the edited body without retraining (a policy trained for the original body alone): its failure on the new body would be attributed to the edit | The policy; its declared body scope against the edited design identity; "a controller that can act on the edited body without retraining" | (1) `frob autopsy rerun <run> --edit '<typed edit>' --controller resynth`: both designs under baseline controllers re-synthesized per design; (2) `--controller family:<policy>`: a family policy whose class contains both designs (printed with the note that none is registered, when none is); (3) `--controller mpc`: the sampling controller; (4) if a retrained specialist is the question, that is a design comparison with a declared controller arm, run as a study (`frob eval compare`), not an Autopsy re-run | M3 | BD-283 with the eligibility rule N27a-15; N27a-17; N27b-19 |
| FROB-REF-880 | An attribution cited as an established cause, or as a cause in the physical robot. Raised by the citation guard (`offer(claim, role)`, N27b-15) for any claim an Autopsy verb produced, in the roles `Established`, `Reality` or an input requiring a class above `Estimated`, at three call sites: report generation, when a statement or a `--cite` puts the claim under a heading or claim kind that needs `established` standing or the Reality claim (the verbs `report.run`, `report.design`, `report.mission`, `report.campaign`: this row's `raised_by`, DESIGN, since they are the verbs that cite); validation of a verb input that carries a claim identifier as supporting evidence in a stated role; and the check of gate rows and demonstration manifests. No Autopsy verb raises it | The citing statement or evidence input; the claim's class (`Estimated`), scope ("cause in the model at this tier, among the listed factors") and producer; the role asked for | (1) Cite it as described, with the permitted wording, class and scope; (2) to establish something about the design: `frob autopsy rerun …`, then a declared study (`frob eval compare …`), and cite the study's verdict; (3) for the physical robot there is no repair in simulation: without imported measurements and a calibrated band the result stays `reality: no-claim` | M3 | N27b-15 (`offer_established_880`, `offer_reality_880`); N27b-18; N27b-19; BD-284 |

**Partial refusals.** `autopsy.intervene` takes several interventions. When some are refused (581, 582, 586) and
others run, the verb completes with exit 2 and lists each refused intervention under `refused` with its code and
fields; when nothing can run, the first refusal is the envelope's `refusal` and the exit is 3. A child plan of
`run.fork` (`--children`, BD-205) follows the same convention for 581 and 582. 580, 583 and 584 concern the
fork as a whole and always refuse the request.

**Neighbouring codes (owned elsewhere; listed so their boundary with Autopsy's is fixed).**

| Code | Meaning | Boundary with Autopsy's codes |
|---|---|---|
| FROB-REF-562 | Attribution or a counterfactual fork was requested below class D2 | The licence check at admission, on the *source run's* class; it fires before any Autopsy code. FROB-REF-584's class clause concerns a fork set whose own children are of uncontrolled class |
| FROB-REF-894 | A snapshot restored under another numerical profile and labelled a continuation | The profile case; FROB-REF-580 is the contract case (physical, clock, sensor, policy) |
| FROB-REF-819, -820 | Citing a result above its claim product; a result that is not `established` cited as established | General rules; where an Autopsy claim is the one cited, FROB-REF-880 is returned because its repairs (re-run, then a study) are the useful ones |
| FROB-REF-882 | Retired in favour of FROB-REF-582 | Tombstoned; never emitted |
| FROB-REF-585 | Not assigned | The number is skipped in the sequence; using it is a load-time error |
| FROB-REF-553 | A comparison that mixes study types, or re-reads fixed-n data as a sequential study | Raised in two places, both before anything is bought: by `autopsy.intervene` (N27bp2-01) for a request any of whose interventions, by canonical string, was already run on the fork under the other test type, in both directions (`fixed` after `sequential` and `sequential` after `fixed`); and by `autopsy.attribute` (BD-280, `pooled_mixed_types_553`) for a pooled read with `--forks` over a `fixed` set and a `sequential` set. It is not an attribution's fixed test re-read sequentially by `autopsy.attribute`, which reads each set under the type its ledger header declares |
| FROB-REF-362 | An edit does not declare the causal paths it changes | Raised by the factor registry (N27b-01) when a world factor cannot say which declared mechanism it changes |
| FROB-REF-866 | A speed experiment requested without its computed ceiling, or with a ceiling below its goal | Raised by the admission check of the method-experiment register (N38a-07, through N36-05's `admit_experiment`) when the speed experiment of the bulk attribution of a failure cluster on a device (N27b-27) is registered, before any of its runs. No Autopsy verb raises it: a bulk `autopsy.intervene` call carries no ceiling and is never refused with 866 |
| FROB-REF-840 | A session without `qual:admin` asks for sealed confirmation data before close | Every row of the Autopsy cluster lists it, since every Autopsy verb can expose a world of the run it reads; it fires before any Autopsy code |
| FROB-REF-801, -802, -805, -701, -702, -724, -761 | Unknown verb; arguments fail the schema; no idempotency key (`run` verbs); budget at admission; budget during a run; not implemented in this build; above the compute envelope in force (`run` verbs) | Surface codes the generator appends to the Autopsy rows (802 also carries the verb-specific cases of BD-279: more than 12 factors, an unreachable request without `--allow-unreachable`) |

**Registry data.** Each row in `registries/refusals.toml`: `code`, `family`, `message` (a template with named
fields), `pointer_kind`, `observed_schema`, `required`, `retry`, `exit`, `repairs` (ordered templates),
`defined_in`, `raised_by` (verbs). Message templates, for example: 581: "A {class} intervention ({do}) cannot
start from a mid-episode state; earliest valid fork tick {earliest_valid_fork_tick}; forced by {forced_by}."

## Acceptance criteria

- [ ] The eight rows load from `registries/refusals.toml`; an unknown code, a duplicate code and the unassigned
      FROB-REF-585 are load-time errors; FROB-REF-882 loads only as a tombstone pointing to 582.
- [ ] Every row maps to exit 3 and to one family; the partial-refusal convention is documented in the rows of
      581, 582 and 586.
- [ ] Each row's `raised_by` equals the set of verbs whose registry entries list it, and each row is raised by
      at least one test named in the last column (a test collects raised codes across the suite). This bead
      closes at M2 before most raisers exist (a registry never waits on its mechanisms): at its close the
      binding test is hard for 580, 581, 582 and 583 (raised by `run.fork`, BD-205) and reports the other four
      as `pending: <raising bead>`; from the Autopsy exit review (N27b-30) on, a pending row fails the check.
- [ ] Repairs that are commands parse as valid `frob` invocations after template substitution.
- [ ] The boundary cases with neighbouring codes are pinned by tests: a run of class D3 gets 562 and not 584; a
      checkpoint under another numerical profile gets 894 and not 580; an Autopsy claim cited as established
      gets 880 and not 820.
- [ ] The generated refusal table in the agent documentation shows the eight rows with their repairs.

## Test plan

### Unit tests
`crates/frob-core/tests/refusals_autopsy.rs`: `eight_rows_load`; `unknown_and_duplicate_are_load_errors`;
`unassigned_585_is_load_error`; `tombstone_882_points_to_582`; `exit_code_is_3`; `templates_have_no_free_fields`
(every placeholder is filled by the raising code's `observed` schema); `repair_commands_parse`.
`crates/frob-verbs/tests/refusal_binding_autopsy.rs`: `raised_by_matches_verb_lists`;
`every_code_raised_by_a_test` (with the `pending` list until the M3 exit; a scratch copy with the exit flag set
and one raiser missing fails); `boundary_562_before_584`, `boundary_894_vs_580`, `boundary_880_over_820` (each
runs when both codes of its pair have a raiser; pending until then).

### End-to-end script and logging
`scripts/e2e/autopsy_refusals.sh` (N27b-19) raises each code from the CLI and through `frob-mcp` and compares the
envelope with the registry row; each `refusal` log line carries `code`, `pointer`, `observed`, `required`,
`retry`, the ranked repairs and the exit code.

### Known answers and negative controls
KA-671's control (a checkpoint with its packet queue stripped does not replay and is refused: 580 at
classification when the strip leaves a contract field missing, 583 when the strip is disguised and the null
fork is what catches it); KA-676 and
CX-321 (the warm-start graft: 582); the mid-episode request of the thermal counterexample (581; CX-298); KA-679
(the marginal failure: 584); CX-301 (880). Negative controls for the registry: a row with its repairs reordered
(the end-to-end comparison fails); a verb entry that omits a code its tests raise (the binding test fails); a
code mapped to exit 2 when it is the whole answer. Determinism: registry loading and envelopes are exact and
byte-identical across platforms.

## Considerations

- 562 and 584 overlap in wording ("below class D2"; "under an uncontrolled determinism class"). The resolution
  above (562 on the source run at admission; 584 on the lanes' outcome and on a fork set's own class) is this
  bead's reading; if the owner of the determinism rules prefers one code for both class cases, the row of 584
  loses its class clause and its tests move, with no change to callers' repairs.
- If KILL-ENSEMBLE fires, the first clause of 584 reads "not decision-equivalent in double precision" and the
  lane's figures leave its `observed` schema; the code and its exit are unchanged.
- If KILL-AUTOPSY fires, all eight codes remain: forks, re-runs and citations still exist.
- Messages are for agents first: each names the next command. A message that only describes the fault is a
  defect of the row.

## References

Plan §27.14 (the chapter's table), §27.4, §27.5, §27.8, §27.9, §27.13, §27.19, §29.17 (582 and its repair; 882
retired), Appendix O (O.5, O.8), §32.4 (envelope and exit codes), §35.11. KA-671, KA-676, KA-679; CX-298, CX-301,
CX-321.
