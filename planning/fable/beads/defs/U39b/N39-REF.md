---
key: "N39-REF"
title: "frob-conformance: the refusal registry of the ten-fold test: product codes and harness refusals"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tenfold", "gauntlet", "frob-conformance", "solid", "registry"]
parent: "WP-34"
deps: ["BD-005", "BD-025", "N32-REF", "N35-33", "N35-REF", "N39a-01", "STD-TEST"]
needs: []
related: ["N39a-15", "N39a-18", "N39a-20", "N39a-21", "N39a-27", "N39a-32", "N39a-38", "N39b-13", "N39b-15"]
gates: ["GATE-TENFOLD-03"]
plan_refs: ["§39.17", "§39.20", "§39.14", "§39.10", "§39.13", "§39.21", "§39.22", "§39.8", "§24.14", "§35.11", "Appendix O", "§39.3", "§39.4", "§39.5", "§39.6", "§39.7"]
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
harness's typed refusals. DESIGN: harness refusals are **refusal reasons**: stable kebab-case strings in the
namespace `tenfold.`, each with a meaning, a trigger and ranked repairs, used by the operator tool `frob-tenfold`,
the scorer and every harness record that can say "refused". They are not product codes, because the harness is
not a product surface and its records travel in the replication kit, not through `frob`. Where a reason is the
harness-side form of a refusal the product already registers, its row names that code, so that the two
vocabularies stay aligned. Why a registry and not prose: tests and exit reviews need to assert *which* refusal
occurred, not merely that something failed; and a reason that is free text drifts.

Rejected: inventing new `FROB-REF-…` numbers for harness conditions (the product registry has one owner and
codes are never reused); reusing product codes for harness conditions they do not describe.

This bead is the one refusal registry of the whole ten-fold protocol: the product codes the test uses, the
harness reasons of its first half (records, roles, acceptance, suite, arms, phase ledger, statistics, ceilings)
and those of its second half (disqualification, outcome rules, the kit, the rival audit, ablations, wording, the
tournament).

Placement (DESIGN). The registry is data and comes first: the harness skeleton's strict readers (M0) already
refuse with `record-*` reasons, the ledger and wrapper (M0) with `ledger-*` and `tagmap-*` reasons, and the
ceiling calculators and the speed-comparison gate (M1) with `ceiling-*` and `order-rule-violation`. The file,
its loader, the generated enum and the consistency test therefore land at M0, right after the harness skeleton,
with every row of both halves entered at once; rows whose raising mechanism lands at M3 or M7 are data until
then, and their producing tests arrive with those mechanisms.

## Scope

In: the tables below as data (`registries/refusals.toml` rows for product codes;
`crates/frob-conformance/registry/tenfold_refusals.toml` for harness reasons, with the generated enum in
`crates/frob-conformance/src/tenfold/refusal.rs`), the loader, the two-way consistency test, messages, ranked
repairs, exit codes of the two harness binaries, and one test per row. Out: the mechanisms that raise the
refusals (their beads are named in the last column of the first two tables; the reasons of the third table are
raised by the harness mechanism named by the row's subject: records by the harness skeleton, roles by the role
registry, cards by the mission card, families by the family registry, the `dev-*` checks of the public
development pool by the development-suite bead N39ax-01 (M3), controls by the anti-shortcut controls,
sealing by the sealing ledger, freezes by the freeze manifest and equal terms, pairs and verdicts by the scorer,
ceilings by the ceiling calculators and the speed-comparison gate, reports by the report record); any change to
the product registry beyond the one new code.

## Technical approach

**File and loader.** `crates/frob-conformance/registry/tenfold_refusals.toml`, one table per reason:
`[[reason]] id = "…"; meaning = "…"; trigger = "…"; repairs = ["…", "…"]; maps_to = "FROB-REF-…" | ""; exit = 3;
aliases = []; raised_by = "<bead key>"; producing_test = "<crate>::<test path>"`. The loader refuses duplicate or unknown ids at start-up. A refusal is emitted as
`{"refusal": {"reason": "<id>", "pointer": "<JSON pointer or record path>", "observed": …, "required": …,
"repairs": [...], "maps_to": "…"}}` on standard output (a product refusal carries `code` in place of `reason`),
with the exit code of its row and its repairs in rank order.

**Naming rule (DESIGN; it reconciles the two spellings in use).** The registry `id` is the bare kebab-case
reason (`freeze-after-open`). Outside a harness record (in an event log shared with product refusals, a script
log, a report or another bead's text) the reason is written with its namespace, `tenfold.freeze-after-open`; the
two spellings name the same row. Five conditions were named twice while the protocol was specified. For four of
them there is one id, and the other name is registered as an alias, which the loader resolves and which is never
emitted; the fifth is one condition seen from two surfaces, a harness reason and a product code:

| Id (emitted) | Alias | Condition |
|---|---|---|
| `role-conflict` | `role-overlap` | A person or session holds two of the roles builder, rival advocate, adjudicator |
| `record-hash-mismatch` | `record-edited` | A ledger, receipt or other record fails its content address or hash chain |
| `frozen-arm-changed` | `arm-changed` | A frozen arm's workspace, manifest or pack digest differs from the published one |
| `family-reused` | `family-retired` | A family that served a gate, or was touched by a leak, is offered for sealing |
| `suite-too-small-for-population` | (product slug `suite-too-small-for-population-claim`) | A population result with too few families: the harness reason at sealing, the product code at `eval.preanalyze` |

**Exit codes of `frob-tenfold` and of the scorer (DESIGN, aligned with the product's).** 0 completed; 2 completed
with flags (for example a `ledger-open` run was built); 3 refused; 4 refused for budget or envelope; 9 integrity
fault (a hash, signature or chain does not verify). 1 is never emitted for a refusal; it is reserved for the two
check modes that compare a record with what it should be: the scorer's `--check`, which exits 1 when a
recomputed value differs from the published one and prints the first differing field, and the gate auditor
`frob-tenfold audit` (N39b-23), which exits 1 when any invariant finding exists and prints one finding per
violation. Product refusals raised through `frob` exit with the code the product registry gives them
(3 for those used here). The command that *detects* a condition exits with its row's code; a void or a
disqualification that follows is a recorded event, and the command that records it exits 0 and sets a `void` or
`disqualified` status on the cell, arm or gate.

**Product codes used by the test.**

The repairs column says how each code's repairs apply in this test. It is not a second repair list: the ranked
repairs a product code emits are those of its own registry row (N35-REF owns 761, 862, 863, 865 and 866 and
forbids any other bead a second trigger or repair list for them), and the build-time check compares meanings.

| Code | Meaning | Trigger in the ten-fold test | Repairs as they apply here | Raised by |
|---|---|---|---|---|
| New code, 5xx block, slug `suite-too-small-for-population-claim` | A ten-fold study plan declares a population result with fewer independent families than a valid one-sided bound needs | `eval.preanalyze` on a plan with result type `population` and n < ⌈ln(1/α)/ln(1.1)⌉ (32 at 0.05, 46 at 0.0125, 49 at 0.01), or n below the count the plan's registered bound requires (182 at 0.05 for the bounded paired contrast) | 1. Declare a finite-suite result. 2. Enlarge the cell to the necessary count. 3. Name fewer confirmatory cells so each has a larger level. 4. Register a bound with a smaller required count and its proof | N39b-34 |
| FROB-REF-532 | A fixed-n study has no pre-analysis, or names a verdict it cannot reach | A confirmatory cell of four families (4 of 4 has tail 0.0625); a cell opened without its recorded pre-analysis | 1. Open the cell as `descriptive`. 2. Enlarge m until the guard is reachable and the power conditions hold. 3. Remove the cell from the confirmatory set | N39b-34, N39a-34 |
| FROB-REF-552 | A request mixes study types | The fixed-n verdict is requested at a monitor's stopping time, or a monitor output is offered as an input to the fixed-n computation | 1. Wait until n and compute once. 2. At the partial gate, read the monitor's own `withdrawn` on its own authority. 3. At the tournament, continue: the stop is disabled | harness statistics; N39b-23 |
| FROB-REF-866 | A speed experiment requested without its computed ceiling, or with a ceiling below its goal | A registered speed comparison lacks p or (f, r, h) from a `ledger-closed` run of the strong baseline, or its ceiling is below the ratio it sets out to show | 1. Record the shares from a closed baseline ledger. 2. Lower the goal below the ceiling. 3. Fund the work for trust, coverage or correctness with that reason and no speed figure | N36-05 (`admit_experiment`, the product-side raiser); its harness form (`ceiling-*`, `lever-share-above-ledger`, `order-rule-violation`) is raised by N39a-37 and N39a-38 |
| FROB-REF-899 | A private or non-redistributable input to a kit | A replication kit whose lineage contains a private scene or dataset, or incumbent code | 1. Replace the input by its generator seed and recipe. 2. Pin the revision and give fetch instructions. 3. Exclude the stratum from the kit and print it `not replicable` | N39b-18 |
| FROB-REF-816 | An artifact whose authorization flags are missing or not `false` | A kit artifact read by the kit verifier or the scorer with a flag `true`, absent, or not the literal false | None: the artifact is foreign; rebuild it through the exporting path | N39b-18, N39b-21 |
| FROB-REF-862 | A performance figure without a complete disclosure block | Any baseline or gate figure missing a field of the block | 1. Re-record with the block. 2. Publish as context, not as a Performance claim | BD-143, N39b-08 |
| FROB-REF-863 | A performance ratio without a paired lower bound | A reported ratio without [L, U] | 1. Compute the family bootstrap bounds. 2. Report the cell as `descriptive` | N39b-14 |
| FROB-REF-865 | A summary statistic that omits what it excluded | A ratio reported without its count of excluded pairs and censored runs | 1. Print the counts. 2. Recompute with every run | N39b-14 |
| FROB-REF-761 | A job exceeds the compute envelope in force | A gate campaign above the approved class | 1. Obtain approval of the campaign class. 2. Run the cells that fit and record the rest `not run` | N39b-29 |
| FROB-REF-840 | A session without the sealing capability asks for sealed confirmation data before close | An arm's agent or tool reads a sealed card, world address, per-world outcome or probe point | None for the arm: the attempt is logged and audited as a possible leak | N39b-02 |
| FROB-REF-860 | An export without the no-authorization flags, or an attempt to produce a true flag | A kit artifact without `execution_authorized`, `hardware_safety_signoff`, `sim_to_real_certified` all false | 1. Regenerate through the exporting path | N39b-18 |

**Harness refusals and voids of the second half of the protocol (DESIGN; shown with their namespace and with the
emitted id, never the alias; exit 3 except `frozen-arm-changed`, `record-hash-mismatch` and
`incumbent-pin-drift`, which are integrity reasons and exit 9).**

| Slug | Meaning and trigger | Effect | Ranked repairs | Raised by |
|---|---|---|---|---|
| `tenfold.freeze-after-open` | A paired run is requested while an arm's freeze manifest is missing, unpublished, or dated after the suite opening | The run is refused | 1. Run at the next gate on a rotated suite. None within this gate | N39b-01 |
| `tenfold.frozen-arm-changed` (alias `arm-changed`) | A frozen arm's manifest or pack digest differs from the published one | The arm is void for the gate (disqualifying ground) | 1. Re-freeze and run at the next gate | N39b-01, N39b-37 |
| `tenfold.role-conflict` (alias `role-overlap`) | A person or session holds two of the roles builder, rival advocate, adjudicator | The action is refused | 1. Assign the action to a session of the right role | N39b-23 |
| `tenfold.record-hash-mismatch` (alias `record-edited`) | A ledger or receipt fails its hash chain | Disqualifying ground for the arm that holds it | None | N39b-01 |
| `tenfold.adjudicator-in-tuning` | An arm's logs show the adjudicating simulator in development or tuning | The arm is disqualified on that stratum | None within the gate | N39b-01 |
| `tenfold.parameters-frozen` | A change to 3, 10, 5, 1.5, 0.05 or the four-cell limit after the first suite hash | Refused | 1. Propose for the next gate, before its suite hash | N39b-05 |
| `tenfold.retest-on-same-suite` | A cell with a kill outcome is submitted again on the same sealed suite | Refused | 1. Measure again at the next gate on fresh families | N39b-05 |
| `tenfold.family-reused` (alias `family-retired`) | A family that served a gate, or was touched by a leak, is offered for sealing | Refused | 1. Author a new family | N39b-23 |
| `tenfold.audit-incomplete` | Sealing is requested before the rival audit's sign-off and pack digest are recorded | The suite is not sealed | 1. Complete the audit | N39b-37 |
| `tenfold.ablation-above-bound` | An ablation reports a ratio above 1/s or 1/(1 − s) | The stratum's ablations are void as an accounting error | 1. Re-trace the baseline. 2. Correct the touched-share accounting | N39b-45 |
| `tenfold.word-not-earned` | A document uses "ten-fold" for a cell without that verdict, population wording on a finite-suite result, or "independently accepted" under `self-adjudicated` | The document fails the wording lint | 1. Use the sentence the outcome rule prescribes | N39b-15 |
| `tenfold.kit-gate-open` | A kit build requested for a gate that is not closed and revealed | Refused | 1. Close and reveal the gate first | N39b-18 |
| `tenfold.kit-credential-found` | A transcript or wrapper offered to a kit contains a credential pattern (provider key, token, private host) | Refused until the span is replaced by a typed redaction marker recording the span's hash | 1. Redact through the marker table and rebuild | N39b-18 |
| `tenfold.kit-incomplete` | A kit's manifest of hashes does not cover a published number's inputs | The kit is refused for publication | 1. Add the missing records | N39b-18 |
| `tenfold.incumbent-pin-drift` | At any launch of an incumbent (`frob-tenfold incumbent run`, BD-039 step 2), the container digest or the warm configuration's hash differs from the pin recorded in `comparators.toml` | The launch is refused before the container starts, so before any timing, and no run record is written (exit 9, integrity) | 1. Restore the pinned container. 2. Re-pin through the audit, which changes the harness identity and invalidates the frozen warm configuration | BD-039 (the launcher); the rival audit and baseline timing runs (N39b-08) are the first to exercise it |
| `tenfold.warm-config-provenance` | A frozen warm configuration whose tuning log names a mission of the test (development or sealed) instead of an incumbent reference task | The configuration is refused by the audit | 1. Re-tune on the incumbent's reference tasks only and re-freeze | N39b-08 |
| `tenfold.exclusion-not-harness-fault` | An exclusion of pairs requested on a ground other than a harness fault that is not the arm's own (for example dropping the runs a leak touched and scoring the rest of the cell) | Refused; the pairs stay in the sums | 1. Void the whole cell for that arm under the leak rule. 2. Record a harness fault only with its fault record, before times are compared | N39b-04 |
| `tenfold.protocol-breach` | A pin moved during the freeze, sealed material read by an arm, or an arm changed after its freeze | The runs, cells or tournament the breach's extent covers are void (N39b-13): a pin move with the stack freeze still open voids the runs that span it and their cells (`void: stack moved`, N33b-03); a bounded leak voids the touched cells (N39b-02); an arm change voids that arm's cells run after it, or all its cells if the time is unknown (N39b-01); only an aborted freeze, a run that cannot be attributed to one receipt, or an unbounded leak voids the tournament. Recorded with its evidence | 1. Re-run the void cells, or the tournament, on a rotated suite of fresh families | N39bp3-01 (the classification by extent and the void record); N39b-13 re-runs what is void |
| `tenfold.step-out-of-order` | A tournament step entered (`frob-tenfold tournament step <n>`) before step n − 1 is entered, or with a time earlier than its predecessor's (the sealing steps keep their own `seal-out-of-order`) | Refused | 1. Enter the missing step | N39bp3-01 |
| `tenfold.interim-stop-refused` | An operator's request to stop a cell of a gate of type tournament before n (`frob-tenfold tournament stop-cell`), where the monitor's futility stop is disarmed | Refused; the request is entered in the event log and the cell's campaigns keep starting | None: the cell runs to n | N39bp3-01 |
| `tenfold.incumbent-licence-not-cleared` | The rival audit or an arm's freeze names an incumbent whose licence reading for *run* or *publish* is not `yes` (`unclear` counts as `no`), or whose reading is stale because its pin moved | Refused; the incumbent does not enter the arm | 1. Replace it by the next strongest candidate whose licence permits publication and print the replacement. 2. Repeat the reading at the new pin. 3. Ask the owner to rule on an `unclear` reading | N39b-32 |
| `tenfold.agent-revision-differs` | Before a campaign's clock starts (from the M1 baseline on), the provider reports an agent revision other than the pin recorded by OPEN-TENFOLD-6 | The campaign is refused before its clock starts; during a gate the gate is suspended and its incomplete cells are void | 1. Run under the pinned revision. 2. If the revision was retired, suspend the gate and re-run the incomplete cells at the next gate | N39b-31 |
| `tenfold.agent-caps-provisional` | A freeze is requested while the recorded agent caps are still `provisional` | Refused | 1. Re-derive the caps from the baseline runs by the twice-p90 rule and record them `final` | N39b-31 |

**Harness refusal reasons of the first half of the protocol (ids as registered; several ids in one cell share a
row's repairs and exit code).** Repairs are ranked; "—" in the fourth column means no product code describes the
condition.

| Reason | Meaning and trigger | Ranked repairs | Maps to | Exit |
|---|---|---|---|---|
| `record-unknown-field`, `record-missing-field`, `record-duplicate-key` | A harness record has a field outside its schema, lacks one, or repeats a key | Regenerate the record with the harness writer | — | 3 |
| `record-noncanonical-number` | A quantity is a JSON number, or a duration is not a canonical decimal string of seconds | Write quantities as strings; use at most three fractional digits | — | 3 |
| `record-unknown-schema` | Schema name or version is not one of the five known | Use the kit's scorer for that version | — | 3 |
| `record-hash-mismatch` | A stored content address or self-hash does not verify | Restore the record from the store; treat an edited ledger or receipt as a disqualifying event | — | 9 |
| `store-overwrite` | A second write of different bytes to a store path | Write a superseding record; never rewrite | — | 9 |
| `role-conflict` | An identity already holds another role in this gate | Assign the role to a different identity | — | 3 |
| `role-authorship` | An advocate identity has authored product code, or an adjudicator identity has authored any arm | Choose a party with no such authorship | — | 3 |
| `role-not-permitted` | An action outside the actor's row of the permission matrix | Have the permitted role perform it | — | 3 |
| `acceptance-code-authorship` | A file of the acceptance tree was authored by a builder or advocate | The adjudicator rewrites it | — | 3 |
| `channel-closed` | A comment on the referee protocol after the suite hash was committed | None; the protocol is frozen | — | 3 |
| `card-kind-mismatch`, `card-kind-not-in-stratum` | A mission card names no or several answer kinds, obligations of another kind, or a kind its stratum does not admit | Fix the card before sealing | — | 3 |
| `card-requirement-invalid` | The requirement does not parse, is not unit-consistent, exceeds the horizon, or its monitor is not the adjudicator's | Correct the requirement; register the monitor | FROB-REF-526 (horizon) | 3 |
| `card-deadline-mismatch`, `card-caps-differ`, `card-unequal-inputs` | A card's deadline, caps or permitted inputs differ from the stratum's or are not available to every arm | Align the card with the stratum and the equal-terms record | — | 3 |
| `card-unknown-without-truth`, `card-vacuous-obligations`, `card-sealed-missing` | A truthful-unknown card has no ground truth; a family card's obligations are empty; the sealed part or its canary is absent | Complete the sealed part | — | 3 |
| `stratum-unknown`, `stratum-deadline-missing` | A stratum outside the registry of eleven; a stratum in scope with no deadline | Use a registered stratum; set the deadline before sealing | — | 3 |
| `grid-cell-empty` | At a tournament, a grid cell with fewer than two sealed families and no `not applicable` reason recorded before sealing | Author families, or record the reason before the suite hash | — | 3 |
| `family-split`, `family-connected-to-known` | Missions that share a generator draw, site, material batch or parent design are in different families or pools; a sealed candidate is connected to a development or revealed mission | Merge the component; replace the candidate | — | 3 |
| `family-reused` | A family that served an earlier gate is offered as sealed | Author a fresh family | — | 3 |
| `suite-too-small-for-population` | A cell declares a population result with fewer than 32 families (46 where four cells share the level) | Declare a finite-suite result; or enlarge the cell | — | 3 |
| `dev-count`, `dev-families` | `frob-tenfold suite check --development`: a stratum's public development pool does not hold exactly 12 missions, or holds them in fewer than 6 families (connected-components rule) | Add or remove development missions until the stratum has 12 in at least 6 families | — | 3 |
| `dev-control-coverage` | The development pool lacks a control that must be rehearsable in public: fewer than 2 conventional-win missions, no `unknown` mission with a witness, a mutation kind absent, the quantifier-order fixture absent from `envelope`, or a warm card without exactly one consequential edit among at least two inconsequential ones | Add the named control to the pool | — | 3 |
| `dev-reference-missing`, `dev-asset-mark`, `dev-keys-missing` | A development mission has no reference answer that the acceptance code accepts or no rejected planted answer per mutation kind; an asset a development card names carries a private or research-only mark; the pool's family keys are not recorded | File the reference answer and the mutants; replace the asset by a public, redistributable one; record the family keys before sealing | — | 3 |
| `controls-easy-win-quota`, `controls-unknown-quota`, `controls-mutation-missing`, `controls-consequence-exposed`, `controls-quantifier-fixture-missing`, `controls-tier-trap-missing` | A stratum lacks an anti-shortcut control: fewer than one easy-win family in six or one unresolved family in twelve; a mutation kind absent; the consequential edit revealed; the quantifier-order fixture absent from `envelope`; no tier trap in a design stratum | Add the missing control before sealing | — | 3 |
| `seal-out-of-order`, `seal-time-regression` | A sealing step recorded before its predecessor is complete, or with a time earlier than the previous entry | Perform the steps in order; never backdate | — | 3 |
| `suite-changed-after-commit` | The suite, protocol, thresholds or deadlines no longer open the committed hash | Void the gate instance and re-seal a new suite | — | 9 |
| `open-preconditions` | `open` requested while a step, the pre-run notice or a pre-opening gate record is missing | Complete the named precondition | — | 3 |
| `freeze-before-suite-commit` | A manifest frozen before the suite and protocol hashes were committed (missions could have been written against it) | Re-freeze after the commitment | — | 3 |
| `freeze-after-open`, `pair-arm-not-frozen`, `freeze-missing` | A freeze entry later than the suite opening; a paired run requested while an arm of the pair has no published manifest | None for this gate: the run is refused; freeze every arm before opening | — | 3 |
| `freeze-agent-differs`, `freeze-hardware-differs`, `terms-mismatch` | A manifest or campaign disagrees with the equal-terms record (agent revision, caps, device fingerprint, shared inputs) | Align the arm with the gate's terms before freezing | — | 3 |
| `freeze-statement-missing`, `freeze-duplicate` | No verifying advocate statement; a second manifest for one arm and stratum | Sign the statement; keep one configuration per stratum | — | 3 |
| `freeze-rival-underbuilt` | The frozen rival wins no easy-win development family of the stratum, or an under-built objection that the adjudicator ruled `sustained` has no recorded response (N39a-20 step 5; both are the under-built ground) | Improve the rival within its budget; the adjudicator rules on a written objection | — | 3 |
| `freeze-home-task-unexplained` | The frozen rival's home-task training time on the Linux host is above 1.2 × its published figure and no written explanation is attached (N39a-20 step 6) | Fix the installation and re-measure; attach the written explanation, after which the measured figure replaces the published one | — | 3 |
| `freeze-selection-not-minimum` | The frozen configuration of a `development_scores` table is not the one with the smallest development restricted mean time and no tie-break reason is recorded (N39a-20 step 2) | Freeze the fastest candidate; record the tie-break reason | — | 3 |
| `capability-unexplained` | A documented incumbent capability is unused on the development suite with no reason given | Use it, or record a concrete reason | — | 3 |
| `guide-over-budget` | The advocate-written agent guide exceeds the documentation budget | Shorten the guide | — | 3 |
| `armc-over-limit` | The ablation arm's counted program text exceeds 2,000 lines | Shorten it; or record the arm as the unbounded composed incumbent | — | 3 |
| `armc-engine-differs` | Arm C pins an engine, learner or environment revision other than arm B's for the stratum, or a second engine (N39a-21) | Pin arm B's revisions for the stratum; use one engine | — | 3 |
| `armc-forbidden-use` | An arm C pack file imports or invokes a FrankenRobotics crate, binary or artifact, or names the adjudicating simulator (N39a-21) | Remove the reference from the pack | — | 3 |
| `frozen-arm-changed` | A campaign workspace differs from its arm's manifest | None: a disqualifying event for the arm; affected cells are void | — | 9 |
| `cold-start-foreign-file` | A file in a cold workspace resolves neither to the freeze manifest nor to the card | Remove it; declare standing assets in the manifest | — | 3 |
| `caps-differ`, `acceptance-rule-differs`, `pair-mismatch`, `regime-mixed`, `platform-mixed` | The campaigns of a pair, or the records of a cell, differ in caps, acceptance rule, mission, index, deadline, regime or platform set | Score only matched records; never pool regimes or platforms | — | 3 |
| `missing-run` | A scheduled campaign has no final record, supersession or exclusion | Recover the record; a run is never dropped | FROB-REF-865 (a summary that omits what it excluded) | 3 |
| `exclusion-after-reveal` | An exclusion or supersession recorded after times were released | None: the pair stays in the sums | — | 3 |
| `submission-commitment-mismatch`, `draw-before-commit` | A payload differs from its commitment; units were drawn for an index with no commitment | Resubmit (spends the next level); void the submission | FROB-REF-841 | 3 |
| `receipt-signature-invalid` | A receipt does not verify under the adjudicator's committed key | Treat as an edited receipt | — | 9 |
| `tagmap-uncovered-tool`, `wrapper-escape` | A tool of the tool layer has no tag rule; a process left the wrapper's process group | Add the rule before the freeze; fix the tool layer | — | 3 |
| `ledger-disclosure-missing` | A ledger names no resolvable disclosure block or freeze manifest | Rebuild the ledger with its references | FROB-REF-862 | 3 |
| `ceiling-needs-closed-ledger`, `ceiling-without-baseline`, `ceiling-wrong-arm` | A ceiling requested from a `ledger-open` run, with no rival ledger, or from arm A's phase shares | Use closed ledgers of the strong baseline | FROB-REF-866 | 3 |
| `ceiling-below-goal` | A registered speed comparison's ceiling is below the ratio it sets out to show | Lower the goal to the ceiling; register a mechanism with a larger share; fund on non-speed grounds | FROB-REF-866 | 3 |
| `lever-share-above-ledger`, `order-rule-violation` | A removable share asserted above the ledger's; a product of lever effects that reuses removed time | Read shares from the ledger; measure each factor on the time earlier factors left | FROB-REF-866 | 3 |
| `lost-factor-missing` | A kill gate that promised a lost-factor record fired without one | Record the factor before the report | — | 3 |
| `cell-guard-unreachable`, `cell-not-reachable`, `preanalysis-missing`, `too-many-confirmatory-cells` | A cell named confirmatory whose guard cannot reject at its level (four families at 0.05), that fails a reachability condition, has no pre-analysis entry, or is the fifth named | Open the cell as `descriptive`; enlarge it for the next gate | FROB-REF-532, FROB-REF-555 | 3 |
| `verdict-before-n`, `verdict-already-computed`, `verdict-input-missing` | The fixed-n verdict requested before the cell is closed (including at a monitor's stopping time), a second time with other inputs, or without a required input record | Wait for n; void the cell instead of recomputing; supply the record | FROB-REF-552, FROB-REF-553 | 3 |
| `ratio-incomplete`, `warm-without-assets` | A ratio offered to a report without its regime, m, n, H, interval, adjudication label or freeze hashes; a warm ratio without S and N* | Build the ratio from the cell's verdict record | FROB-REF-863 | 3 |
| `report-stratum-missing`, `report-status-drift` | A registered stratum is neither reported nor listed `not run`; a cell's status differs from the pre-run notice | List the stratum with its reason; never change a status after opening | FROB-REF-865 | 3 |
| `envelope-not-approved` | A campaign above the default compute envelope without the owner's recorded approval of its class | Obtain approval; or record the cell `not run` | FROB-REF-704, FROB-REF-761 | 4 |
| `prereg-section-missing`, `prereg-rule-paraphrased` | `frob-tenfold prereg check`: a section of the twelve of the pre-registration is absent or empty; a section's rule text differs from the text of the bead that owns the rule (pointer the section number) | Regenerate the document with `frob-tenfold prereg build`; quote the owning rule text | — | 3 |

The freeze checks of the two incumbent arms are raised through the freeze-check hook of the freeze manifest
(N39a-18): `capability-unexplained`, `freeze-rival-underbuilt`, `freeze-home-task-unexplained` and
`freeze-selection-not-minimum` registered by arm B (N39a-20), and `armc-over-limit`, `armc-engine-differs` and
`armc-forbidden-use` registered by arm C (N39a-21). `frob-tenfold freeze write` then exits 3 with the reason and
appends no step-4 entry; the rows' `raised_by` is the registering bead and their producing tests are N39a-20's
and N39a-21's hook tests (M3, `pending` until then).

The pre-registration check of the partial gate (BD-314, M3: `frob-tenfold prereg check <prereg.json>`) raises
`prereg-section-missing` and `prereg-rule-paraphrased` with the section number as the pointer and exits 3; the
rows' `raised_by` is BD-314 and their producing test is BD-314's `prereg::` test (M3, `pending` until then).

Rows that the second-half table also lists (`freeze-after-open`, and the four aliased conditions) are one row of the
registry each: the id, repairs and exit code are those of the table below; the effect and the raising bead are those
stated above.

**Consistency test.** A unit test collects every reason literal used in the harness source and every `id` in the
file and requires the two sets to be equal; a reason removed from code stays in the file marked `retired` and is
never reused. The generated enum in `refusal.rs` equals the file.

## Acceptance criteria

- [ ] The registry file holds every reason of the two harness tables and loads; duplicate and unknown ids fail
      at start-up; each alias resolves to its id and is never emitted.
- [ ] The two-way consistency test passes; adding an unregistered reason literal to the harness fails it.
- [ ] Every refusal emitted by either binary has the JSON shape above and the exit code of its row.
- [ ] Rows that map to a product code quote that code's registered meaning correctly (checked against the
      product registry at build time). The codes owned by registries that exist at M0 (N35-REF: 761, 862, 863,
      865, 866; N32-REF: 704) must be present, or the build fails. A code whose owning registry lands later
      (N24-REF and N28-REF at M1 for 526, 532, 552, 553, 555, 840, 841; N37-REF at M1 for 816; N30-REF at M1 for
      899; N31-REF at M5 for 860) is listed `pending` with its owner by `maps_to_exist`; from the moment the owning
      registry is closed a missing or differing code fails the build, and GATE-TENFOLD-03 (N39b-22) requires no
      `pending` mapping at M3 among the codes its mechanisms raise.
- [ ] No row allocates a product code except the one new code named in the first table.
- [ ] Every row names its producing test (`producing_test = "<crate>::<path>"`); for the rows whose raising
      mechanism exists when this bead closes (the M0 records, roles, ledger and wrapper reasons) that test exists
      and checks code or slug, exit code and the order of repairs; for the others the field names the test the
      raising bead must write, and `rows_have_producing_tests` lists them as `pending`. GATE-TENFOLD-03 (N39b-22)
      requires no `pending` row among the reasons of mechanisms due by M3, and at M7 none at all.
- [ ] The new product refusal is this bead's ledger request `suite-too-small-for-population-claim` in
      `registries/refusal_allocations.toml` (family `5xx`, no proposal, raised by N39b-34, interim none; the
      ledger's rule of N35-33 gives the number, this bead never types one); once assigned, its row is in
      `registries/refusals.toml` and the loader accepts it.
- [ ] No harness path exits non-zero without a refusal record; no product path returns an unregistered code.
- [ ] A cap breach is not a refusal: the run is censored at the instant of the breach and costs H.

## Test plan

### Unit tests

`cargo test -p frob-conformance tenfold::refusal`: one test per slug (for example
`freeze_after_open_is_refused_with_repairs`, `retest_on_same_suite_refused`,
`parameters_frozen_after_first_hash`); `all_slugs_documented` (the enum equals the table);
`cap_breach_censors_not_refuses`. Product codes are tested in the beads that raise them.

`cargo test -p frob-conformance --test tenfold -- refusal_registry::`
- `loads_and_unique`.
- `code_and_registry_agree` (both directions).
- `aliases_resolve`: the four aliases load as their ids; emitting an alias is a test failure.
- `maps_to_exist`: every `maps_to` value of an M0 registry (761, 862, 863, 865, 866, 704) is present in
  `registries/refusals.toml` with the quoted meaning; every other value is either present with that meaning or
  listed `pending` with its owning registry bead; a planted mapping to 866 with a different meaning fails.
- `exit_codes`: a sample of ten reasons, each provoked through its binary, returns the tabulated exit code;
  `check_difference_exits_one` for the scorer's comparison mode.
- `shape`: emitted refusals parse against the refusal shape and carry at least one repair where the table has one.
- `retired_never_reused`.
- `rows_have_producing_tests`: every row names a producing test; the M0 rows' tests exist and pass.
- `second_half_slugs_are_ids`: every slug in the second-half table resolves to an emitted id, never an alias.
- `freeze_hook_reasons_registered`: the seven freeze-check reasons of N39a-20 and N39a-21 are rows with exit 3
  and `raised_by` the registering bead; a planted hook check that returns an unregistered reason fails the
  consistency test.
- `protocol_breach_effect_by_extent`: the row's effect names the four extents (spanned runs with the freeze open,
  touched cells, the changed arm's later cells, the whole tournament) and the three conditions that void the
  tournament, as N39bp3-01 states them, and its `raised_by` is N39bp3-01.
- `late_rows_registered`: `prereg-section-missing` and `prereg-rule-paraphrased` are rows with exit 3, no
  `maps_to` and `raised_by` BD-314; `tenfold.step-out-of-order` and `tenfold.interim-stop-refused` are rows with
  exit 3 and `raised_by` N39bp3-01, the second with no repair; `incumbent-pin-drift` has exit 9 and `raised_by`
  BD-039.

### End-to-end script and logging

`scripts/e2e/tenfold_known_answers.sh` (N39b-22) drives each refusal through `e2e_expect_refusal` (product codes)
or through the harness driver with an asserted slug, logging `event: refusal` with the ranked repairs.

`scripts/e2e/tenfold_refusals.sh` (depends on STD-E2E): provokes one refusal from each row's first reason on
rehearsal fixtures (a record with a number-typed duration, a role conflict, a card of the wrong kind, an
out-of-order sealing step, a late freeze, a four-family confirmatory cell, a verdict before n, a ceiling below its
goal, and so on) and asserts reason, exit code and first repair with the shared helper's refusal assertion. A row
whose raising mechanism is not yet built is logged `event: assert`, `status: pending` with its raising bead, and
does not fail the script at M0; the script is re-run by N39b-22 at M3 and M7, where a pending row of a due
mechanism fails it. Each line logs `ts`, `script`, `bead: "N39-REF"`, `step`, the exact command line, exit code,
wall time, the reason, the pointer, the first repair, the product code it maps to and the registry file's
digest; the final `summary` line counts rows provoked, pending and failed, and names GATE-TENFOLD-03.

### Known answers and negative controls

Known answers: the family counts 32, 46, 49 and 182 in the `required` field of the population refusal; the tail
0.0625 in the four-family refusal; the row count equals the number of distinct reason literals in the harness.
Negative controls: a build in which `tenfold.freeze-after-open` is downgraded to a warning must fail the test
that expects the run to be refused; a refusal emitted as free text (the shape test fails); a reason reused with a
new meaning; a mapping to a product code whose meaning differs (the build-time check quotes both).

## Considerations

- Tests of the population refusal match on its slug field, so the number the ledger assigns changes no test.
- A disqualification is not a refusal: it is a recorded outcome with scope and evidence, and it awards nothing
  to the other arm.
- Refusal messages never contain sealed content.
- OPEN (registry owner): whether the harness reasons that correspond to protocol rules a product user could also
  hit (for example `verdict-before-n`) deserve product codes. Default: no new codes; the mapping column is the
  bridge.
- Reason strings appear in gate records and exit reviews; renaming one after a gate has been recorded breaks
  citations, hence the never-reuse rule and the alias table.

## References

§39.3 to §39.8 (the refusals stated in prose), §39.17, §39.20, §39.14, §39.10, §39.13, §39.21, §39.22, §24.14,
§28.20, §32.18, §35.11, §32 (exit-code conventions), Appendix O; GATE-TENFOLD-03, -08, -09, -10; product codes
FROB-REF-526, -532, -552, -553, -555, -704, -761, -816, -840, -841, -860, -862, -863, -865, -866, -899.
