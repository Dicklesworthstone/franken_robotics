---
key: "N30-CX"
title: "Ground counterexample registry: CX-326 to CX-335 and the chapter's unnumbered cases, as negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "ground", "frob-ground", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "WP-68"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-TEST"]
needs: []
related: ["BD-139", "BD-387", "BD-389", "BD-390", "BD-391", "BD-393", "BD-394", "BD-396", "BD-397", "N30-KA", "N30-REF", "N30a-13", "N30a-15", "N30a-24", "N30a-25", "N30a-27", "N30a-29", "N30a-33", "N30a-37", "N30a-45", "N30b-01", "N30b-06", "N30b-14", "N30b-15", "N30b-16", "N30b-23", "N30b-29", "N30b-30", "N30b-37", "N30b-38", "N38b-04"]
gates: []
plan_refs: ["Appendix N.27", "§30.2", "§30.3", "§30.5", "§30.7", "§30.8", "§30.9", "§30.10", "§30.11", "§30.12", "§30.13", "§30.14", "§30.18", "§30.21", "§30.22", "§30.23", "§30.24", "§38.11"]
---
## Background

A counterexample is the smallest hand-checkable case in which a plausible shortcut gives a wrong answer. The
program keeps a catalogue of them, and each becomes at least one **negative control**: a planted defect that a
gate must catch every time it runs. The rule behind this is that a gate which no deficient implementation can
fail validates nothing. Controls live in `frob-conformance/registry/controls.toml`, one record each with a
stable identifier, the catalogue entry it came from, the gates that must catch it, its kind (`mutation`,
`planted-input`, `planted-design`, `planted-world` or `a-a`), the target, the defect and the expected
observation. A gate with no registered control cannot record a pass, and a gate run in which a control was not
caught is a fail whatever its passing tests say.

Ground is the subsystem that compares simulation with measured reality (crate `frob-ground`). Its shortcuts are
unusually tempting, because each one makes a result look better: a clock fitted to the event it scores reports
zero timing error; a fit scored on its own data passes; pooled calibration looks nominal. This bead is the
chapter's registry: the ten numbered entries of the catalogue, and fifteen further cases the chapter states
(five with numbers, ten in words) but which carry no catalogue number. Mechanism beads cite these entries and
state the values they need; this bead holds the full set as entries of the counterexample catalogue, each with
the plant that becomes its control.

## Scope

In: the twenty-five entries in `crates/frob-conformance/registry/counterexamples.toml` (the ten numbered
counterexamples and the fifteen late ones) with their cases, numbers, the rule each protects, the gates and the
plant (the build or input in which the rule was removed, not the boundary case itself); the block request for
the fifteen late numbers; the registry test.
Out: the catalogue's record schema, the seeding tool that writes `controls.toml` and the join (N38b-05: this
bead writes no row of `controls.toml`); the allocator (N38a-40); the control runner (N38b-04); implementing the
plants and fixtures (the planting beads of the plant table; the synthetic rehearsal, BD-388); the known-answer
registry of the chapter (`N30-KA`, written with the first half of the chapter).

## Technical approach

**Numbered entries.**

| CX | The tempting shortcut | The smallest case that breaks it, with its numbers | The rule it protects | Control (kind; expected observation; the plant table below gives the control's texts) | Gate |
|---|---|---|---|---|---|
| CX-326 | Fit the clock offset to the event being scored | The prediction says the wheel crosses the sill at 4.212 s; the log shows 4.180 s. The offset that aligns them reports a crossing-time error of zero and an offset of 32 ms. The whole error has moved into the clock | The clock is fitted to declared marks only, never to a scored event | mutation: a fit that takes the scored event as a mark; the detector reports a 32 ms disagreement with the mark-based offset and the score is refused | GATE-GROUND-02 |
| CX-327 | Cross-correlating command and response fits the clock | A delay of 8 ms with an offset of −8 ms produces the same logs as no delay and no offset. Without a shared hardware mark only "delay plus offset" is identified | Without a shared mark the quantity is reported as "delay plus offset" and the card's delay is not updated; structural alias 5 | mutation: a fit that writes the cross-correlation lag into the card's delay; refused with FROB-REF-879 | GATE-GROUND-02, GATE-GROUND-03 |
| CX-328 | A good local fit identifies its parameters | Eight structural aliases: armature N²·J_r with link inertia (a joint moving alone shows only I_link + N²·J_r); Coulomb friction with a constant bias in one direction (τ_c·sgn(q̇) + τ_b is one number); viscous friction with back-EMF damping under voltage drive (b + k_t·k_e/R is one number); torque constant with the scale of inertia and friction; delay with clock offset; friction coefficient with the torque limit; static friction when the run never dwells at zero velocity; contact stiffness with damping from one drop height | Identifiability is analysed before any fit; a fit on an unidentifiable combination is refused, never regularized quietly | mutation: a regularized fit that returns a value for each aliased member; each of the eight must be refused | GATE-GROUND-03 |
| CX-329 | Friction can be identified from any traction run | Drawbar force F = min(k_t·I/r, μ·N) with k_t = 0.2 N·m/A and r = 0.06 m. At 1 A and 2 A under 30 N and 40 N the force is 3.33 N and 6.67 N for μ = 0.4 and for μ = 0.8. At 3 A under 10 N it is 4 N and 8 N | The traction-limited regime must be excited as well as the torque-limited one | planted-input: only the 1 A and 2 A runs; a fit that reports μ is caught (sensitivity to μ is exactly zero) | GATE-GROUND-03 |
| CX-330 | Pool calibration data across regimes to get more points | Pooled conformal calibration at a 90% target covers 20 of 20 episodes in one regime and 16 of 20 in a shifted one: 36 of 40, nominal overall and 80% where it matters | The cell is the bucket; two cells are never pooled | mutation: a band keyed by design only; the pooled band is issued and the shifted regime covers 16 of 20 | GATE-GROUND-06 |
| CX-331 | Score a fit on its own calibration data | A fit class without the truth's Stribeck term passes on its calibration data and is rejected on sealed data | Predictions are frozen and scored on sealed trials | mutation: `ground.score` reading the calibration split; the omitted-law candidate is admitted | GATE-GROUND-04, GATE-GROUND-05 |
| CX-332 | Matching the observed speed validates the model | Speed matched by a compensating error in k_t and friction. The energy cross-check marks the fit `wrong-reason` | The energy ledger is evaluated on the real log, and a fit whose electrical-to-mechanical residual exceeds the instrument bound cannot be admitted | mutation: admission without the energy check; the compensated fit is admitted | GATE-GROUND-04 |
| CX-333 | Twenty clean real trials support "under 5%" | Zero failures in 20 trials give a one-sided 95% upper bound of 1 − 0.05^(1/20) = 13.9% (9.5% at 30; 4.95% at 59) | The reachability pre-analysis runs at registration | planted-input: a prediction registering "failure rate under 5%" at n = 20; accepted by a registrar without the pre-analysis | GATE-GROUND-05 |
| CX-334 | Recovering planted parameters from a simulator is a Reality result | The truth simulator is the referee with planted parameters; everything it emits has origin `synthetic` and the claim stays `no-claim` | Data of origin `synthetic` never yields a status other than `synthetic` | planted-input: a rehearsal dataset relabelled `real`; it fails its hash, and its chunk hashes are refused inside any `real` dataset | GATE-GROUND-01 |
| CX-335 | Agreement between tiers shows the model is right | Every tier consumes the same actuator card, so Fidelity and Parity results say nothing about whether the card describes a real motor | The report line that prints the share of `generic` and `assumed` card fields; Fidelity and Parity are never cited as Reality | planted-input: a tier-agreement result offered as a Reality citation; refused with FROB-REF-819 | GATE-GROUND-01 |

**Unnumbered cases stated by the chapter** (late entries: the tags `CX-G-01` to `CX-G-15` are their working
names, kept in `slug`; their numbers come from the block requested below).

| Id | Shortcut | Case | Rule | Gate |
|---|---|---|---|---|
| CX-G-01 | A synchronized clock separates exposure delay from mount position | A range sensor approaching a wall at +2 m/s reads L − v·(t + δ) − b. A delay of 20 ms with no offset and an offset of 40 mm with no delay give the same 101 samples of a one-second pass at 100 Hz. Reversed at −2 m/s they differ by 80 mm; at +1 m/s by 20 mm | The ninth structural alias: only v·δ + b is identified at one axial velocity | GATE-GROUND-11 |
| CX-G-02 | Twenty episodes are twenty draws | Twenty episodes on one machine are one cluster. For a new unit of the same design the draws are units, and a 90% band across units needs at least 9 calibrated units | The exchangeable unit is the robot | GATE-GROUND-06 |
| CX-G-03 | A scale estimate far from one is shrinkage | A similarity fit returning a scale near 25.4 or 0.001 is a unit error | The unit-suspicion diagnostic | GATE-GROUND-08 |
| CX-G-04 | The registration residual is the transform's uncertainty | The residual is a fit diagnostic. A residual at or above the deviation under review means the signal is below the noise and no deviation is reported | The noise screen and the calibrated pose covariance | GATE-GROUND-08 |
| CX-G-05 | A log of completed trials is a log of trials | A log holding only the trials that completed calibrates a band on survivors | `attempts = "all"`, checked against the trial counter | GATE-GROUND-01 |
| CX-G-06 | Split rows at random | Rows inside one episode are dependent; a random split leaks the validation set into the fit | Splits follow hardware, condition or day | GATE-GROUND-05 |
| CX-G-07 | Regularize an unidentifiable fit | A regularized fit returns the prior and labels it a measurement | Refuse, never regularize quietly | GATE-GROUND-03 |
| CX-G-08 | Fit contact before the actuator | A contact parameter fitted before the actuator is right absorbs the actuator's error and predicts the wrong thing on the next surface. In a planar threshold pilot, changing one winding feed-forward term from R·i_ref to R·i changed 24 of 156 outcome labels | The fixed identification order: clocks, actuators, contact and surface, inertia and geometry, sensors | GATE-GROUND-04 |
| CX-G-09 | Sample identified fields independently | Armature and friction estimates are correlated; independent marginals are wider than the evidence in some directions and narrower in others | The card stores the joint covariance; draws use a copula | GATE-GROUND-04 |
| CX-G-10 | More samples always shrink the posterior | A thousand samples that share one encoder bias are not a thousand measurements; posterior variance is never reported below the model-form floor | The common-mode source and the discrepancy floor | GATE-GROUND-04 |
| CX-G-11 | A probe that is perfect in its law class is right | A probe built for three material laws classified 675 of 675 cases inside the class and 0 of 225 under an omitted delayed law; measuring the target directly was right in 900 of 900 | The omitted-law control in the rehearsal; the direct-measurement recommendation | GATE-GROUND-04 |
| CX-G-12 | A kernel or optimal-transport drift test will see a shift | Those tests have no power at about twenty scalar residuals | Drift is tested on ranks and misses | GATE-GROUND-07 |
| CX-G-13 | Screen a kit's direct inputs for the private mark | A score derived from a dataset recorded in a private place carries the mark by inheritance; a builder that checks only the inputs the result names directly ships the private data inside the kit | The builder refuses any input that carries or inherits the private mark, with no override flag (FROB-REF-899) | GATE-GROUND-09 |
| CX-G-14 | A kit is for the result that won | A builder that packages only confirmed predictions, or drops the superseded versions of a prediction, hides the losing results and the versions that preceded the winner | Losing results too: a kit is built for a falsified prediction as readily as for a confirmed one, and lists every prediction version | GATE-GROUND-09 |
| CX-G-15 | The stand-in is close enough to be the first real cell | The default answer to the dataset question is a second-simulator stand-in labelled `synthetic`; recording it as the first real cell turns a simulator into a Reality claim | The first real cell needs data of origin `real`; the stand-in stays `synthetic`, its chunk hashes are refused inside any `real` dataset (FROB-REF-814), and the gate stays at `no-claim` | GATE-GROUND-10 |

**Registry form.** The twenty-five entries are records of the suite's one counterexample catalogue,
`crates/frob-conformance/registry/counterexamples.toml`, in the record of N38b-05 and with no field of the
chapter's own: `id` (CX-326 to CX-335; a late entry takes its number from the block below, in table order),
`shortcut`, `case` (with its numbers), `numeric` and `plan_case` (`true` and `"numbers"` for the ten numbered
entries and for CX-G-01, -02, -03, -08 and -11; `false` and `"words"` for the other ten late entries, whose
cases the chapter states in words), `case_supplied_by` (empty), `rule`, `rule_ids`, `defined_in`, `same_as`
(empty), `label`, `rerun` and `rerun_receipt` (on the two FACT entries), `control_kind`, `plant`, `due` and, for
a late entry, `slug` (its tag). This bead writes no row of `controls.toml`: N38b-05's seeding tool derives one
control per plant (an `NC-` number it allocates, the entry's CX number as `seed`, the plant's gates), and the
join proves that every entry has its controls. DESIGN, correcting an earlier text of this bead that wrote one
control record per entry itself, with identifiers of the form `NC-GROUND-0330`: BD-025's loader admits only
`NC-` and four digits issued by the seeding tool, and a chapter's registry bead writes catalogue entries only.

`rule_ids` hold the identifiers of the rule and the gates of the gate column, as Appendix N.27 gives them for the
numbered entries: CX-326 §30.7 rule 2 and GATE-GROUND-02; CX-327 §30.7 rule 3, structural alias 5,
FROB-REF-879, GATE-GROUND-02 and GATE-GROUND-03; CX-328 §30.8 and GATE-GROUND-03; CX-329 §30.8 and
GATE-GROUND-03; CX-330 §30.12 and GATE-GROUND-06; CX-331 §30.11, GATE-GROUND-04 and GATE-GROUND-05; CX-332 §30.9
and GATE-GROUND-04; CX-333 §30.11 and GATE-GROUND-05; CX-334 INV-GROUND-06 and GATE-GROUND-01; CX-335 §30.2,
FROB-REF-819 and GATE-GROUND-01. `defined_in`: §30.7 for CX-326 and CX-327, §30.8 for CX-328 and CX-329, §30.12
for CX-330, §30.21 for CX-331 to CX-334, §30.2 for CX-335. For the late entries `rule_ids` hold the rule's
identifiers and the gate of the late table, and `defined_in` is the subsection that states the case: CX-G-01
§30.24; CX-G-02 §30.12; CX-G-03 and CX-G-04 §30.14; CX-G-05 and CX-G-06 §30.5; CX-G-07 §30.3 (ADR-GROUND-04);
CX-G-08 §30.9; CX-G-09 §30.10; CX-G-10 §30.9 and §30.10; CX-G-11 §30.8; CX-G-12 §30.3 (ADR-GROUND-09) and
§30.13; CX-G-13 and CX-G-14 §30.18; CX-G-15 §30.22 and §30.23.

`label` is DESIGN for every entry except two pilot-derived FACT entries (N38b-05): CX-G-08 names `PILOT-PT-17`
(the planar threshold pilot, 24 of 156 labels) and CX-G-11 names `PILOT-PT-03` (the material probe, 675 of 675
and 0 of 225). Each carries `rerun = true` with `rerun_receipt` equal to the receipt of its record's result when
`pilots::rerun_status` reads `reproduced`, and `rerun = false` while it reads not run or `differs`; a pair that
disagrees with the register is the load error `rerun-disagrees-with-pilot`.

**Numbers for the late entries.** A bead never invents a number above the plan-time maximum (CX-384). This bead
files one `[[block]]` request in `crates/frob-conformance/registry/allocations.toml` (schema
`frob.gauntlet.alloc/1`, N38a-40): `registry = "cx"`, `count = 15`, `owner = "N30-CX"`, `section = "§30.3, §30.5,
§30.8 to §30.10, §30.12 to §30.14, §30.18, §30.22 to §30.24"`; the allocator fills `first` and `last`, and the
fifteen tags are mapped to the block's numbers in table order (CX-G-01 to the first number) in one commit. The tag
stays in `slug`; a tag still used in a registry, a test name or a derivation file at this bead's milestone (M1)
fails `provisional-tag-past-due`. Beads that cite a late case cite its tag until the block is issued and its
number afterwards.

**Plants.** Each entry has one plant; `id` is `<planting bead>:cx-<nnn>`, where `<nnn>` is the entry's CX number
(for a late entry, the number issued to its tag), and `cx-<nnn>` is the Cargo feature (a `mutation`) or the
`synthetic` fixture (the other kinds) that the planting bead declares (N38b-05). The target, defect and expected
observation of the table are the texts the seeding tool copies into the control's `target`, `defect` and
`expect`; a planted input states the build in which the check was removed, so that it is a defect and not a
boundary (BD-025 rule 5). `due` is the milestone of the planting bead; until the plant is built the control is
reported `stale` and makes its gate ineligible.

| Entry | `control_kind` | Plant `id` | Target | Defect | Expected observation | Gates | `due` |
|---|---|---|---|---|---|---|---|
| CX-326 | mutation | `BD-389:cx-326` | the clock fit of `frob-ground` | the scored event taken as a declared mark | the crossing-time error reads zero and the offset 32 ms; the gate's clock row fails on the 32 ms disagreement with the mark-based offset | GATE-GROUND-02 | M5 |
| CX-327 | mutation | `N30a-13:cx-327` | the card writer of `frob-ground` | the writer accepts `DelayPlusOffset` and writes the cross-correlation lag into the card's delay | an 8 ms lag is written as the card's delay where only "delay plus offset" is identified and the writer must refuse with FROB-REF-879; the card-update check fails | GATE-GROUND-02, GATE-GROUND-03 | M5 |
| CX-328 | mutation | `BD-390:cx-328` | the fit guard of `frob-ground` | a ridge-regularized solver in place of the refusal | a value (the prior mean for μ) is returned for an aliased member, where none of the eight may return a value; the guard's test fails | GATE-GROUND-03 | M5 |
| CX-329 | planted-input | `N30a-15:cx-329` | the identifiability analysis and the fit guard | a traction data set holding only the 1 A and 2 A runs, offered for a fit of μ, with the zero-sensitivity check removed | a value of μ is reported although its sensitivity is exactly zero (3.33 N and 6.67 N for μ = 0.4 and for μ = 0.8), where the analysis must report zero sensitivity and request the run at 3 A under 10 N | GATE-GROUND-03 | M5 |
| CX-330 | mutation | `BD-393:cx-330` | `frob-ground::band::bucket_key` | bucket keyed by design only, so two surfaces share one calibration set | a band is issued for the pooled set; held-out coverage is 16 of 20 in the shifted regime; the gate fails | GATE-GROUND-06 | M5 |
| CX-331 | mutation | `N30b-06:cx-331` | `ground.score` | scoring reads the calibration split instead of the sealed trials | the omitted-law candidate (no Stribeck term) passes and is admitted, where it is rejected on sealed data | GATE-GROUND-04, GATE-GROUND-05 | M5 |
| CX-332 | mutation | `N30a-24:cx-332` | the admission of an identified card | admission without the energy cross-check | the compensated fit (speed matched by compensating errors in k_t and friction) is admitted, where the energy cross-check marks it `wrong-reason` | GATE-GROUND-04 | M5 |
| CX-333 | planted-input | `N30a-33:cx-333` | the prediction registrar | a prediction registering "failure rate under 5%" at n = 20, offered to a registrar with the reachability pre-analysis removed | the claim is accepted at registration, although zero failures in 20 trials bound the rate only at 13.9% | GATE-GROUND-05 | M5 |
| CX-334 | planted-input | `BD-139:cx-334` | the Reality line of GATE-GROUND-01 (rows 26 to 30 of BD-139's battery) | synthetic data promoted: a synthetic manifest edited to `real` with the old and with a recomputed id, a rehearsal result with `synthetic` stripped, a band record on a synthetic dataset, a stand-in offered as the first real cell | with `synthetic` made non-sticky (BD-139's mutation `synthetic-not-sticky`) row 28 passes as `calibrated` and the gate fails; with the check in place the five rows end in 873, 814, 819, 814 and 814 | GATE-GROUND-01 | M1 |
| CX-335 | planted-input | `BD-139:cx-335` | the Reality citation check of GATE-GROUND-01 (rows 31 to 35 of BD-139's battery) | another claim offered as Reality: agreement with a pinned oracle, with a published real-robot figure, between tiers; recovery of planted parameters; a paired verdict or a throughput figure | with the Reality-citation check removed the tier-agreement result (row 33) is accepted as a Reality citation and the gate fails; with the check in place the five rows are refused with FROB-REF-819 | GATE-GROUND-01 | M1 |
| CX-G-01 | mutation | `N30b-29:cx-<nnn>` | the alias-9 fit | a fit that splits v·δ + b by a prior | a delay is reported from one pass; the unchanged-address assertion and the refusal test fail | GATE-GROUND-11 | M5 |
| CX-G-02 | mutation | `N30a-37:cx-<nnn>` | the band authorization for a new unit | episodes counted as draws for a new unit | a band is issued for a new unit from one machine's twenty episodes, where at least 9 calibrated units are needed | GATE-GROUND-06 | M5 |
| CX-G-03 | mutation | `N30a-45:cx-<nnn>` | the as-built similarity registration | a scale near 25.4 accepted as shrinkage and applied silently | no unit suspicion is raised for a scale of 25.4 | GATE-GROUND-08 | M5 |
| CX-G-04 | mutation | `N30a-45:cx-<nnn>` | the as-built registration report | the residual RMS reported as the transform's uncertainty | a deviation smaller than the registration residual is reported, where the noise screen reports none | GATE-GROUND-08 | M5 |
| CX-G-05 | mutation | `BD-387:cx-<nnn>` | `ground.ingest` | the check of `attempts = "all"` against the trial counter removed | a log holding only the completed trials is admitted and a band is calibrated on survivors | GATE-GROUND-01 | M5 |
| CX-G-06 | mutation | `BD-387:cx-<nnn>` | the manifest validator of `ground.ingest` | the validator accepts `by = "rows"` | a random split of rows is admitted; `split_by_rows_refused` fails | GATE-GROUND-05 | M5 |
| CX-G-07 | mutation | `BD-391:cx-<nnn>` | the identification fit | a regularized fit that returns a value for an aliased member | the prior is returned and labelled a measurement | GATE-GROUND-03 | M5 |
| CX-G-08 | mutation | `N30a-25:cx-<nnn>` | the fixed identification order | the contact fit run before the actuator stage is admitted, with a deliberately wrong actuator card and no stage gate | the contact parameter absorbs the actuator's error, is admitted, and predicts the wrong outcome on the next surface | GATE-GROUND-04 | M5 |
| CX-G-09 | mutation | `N30a-27:cx-<nnn>` | the correlated draws of a card version | identified fields drawn from independent marginals, ignoring the stored joint covariance | the draws' covariance differs from the card's; the variance identities of the joint law fail | GATE-GROUND-04 | M5 |
| CX-G-10 | mutation | `N30a-29:cx-<nnn>` | the discrepancy floor | a floor of zero | samples that share one encoder bias shrink the posterior variance below the model-form floor; `floor_applies` fails | GATE-GROUND-04 | M5 |
| CX-G-11 | planted-world | `BD-397:cx-<nnn>` | the rehearsal's omitted-law world | a probe built for three material laws, run where a fourth, delayed law is present | the probe is right in 675 of 675 in-class cases and in 0 of 225 under the omitted law; a decision taken on it without the direct-measurement recommendation fails the rehearsal | GATE-GROUND-04 | M5 |
| CX-G-12 | mutation | `BD-394:cx-<nnn>` | the drift monitor | a kernel-discrepancy drift test in place of the rank and miss tests | at about twenty scalar residuals the planted drift is not detected and the calibration is not retired | GATE-GROUND-07 | M5 |
| CX-G-13 | mutation | `BD-396:cx-<nnn>` | the replication-kit builder | the private mark checked on the direct inputs only | an inherited private mark passes and the private data ships in the kit | GATE-GROUND-09 | M5 |
| CX-G-14 | mutation | `BD-396:cx-<nnn>` | the replication-kit builder | superseded and failed predictions omitted from the kit | the losing history is lost; `superseded_predictions_included` fails | GATE-GROUND-09 | M5 |
| CX-G-15 | mutation | `BD-397:cx-<nnn>` | the GATE-GROUND-10 recorder | the origin check removed | a stand-in labelled `synthetic` is recorded as the first real cell; `synthetic_receipt_refused_origin_not_admissible` records a pass and fails | GATE-GROUND-10 | M5 |

The ten late entries stated in words carry `numeric = false`, so the seeding tool writes their controls as
`obligation` (N38b-05 step 4): not runnable, and they make no gate eligible. When the planting bead builds the
plant it records its fixture in the entry's `case_supplied_by` (the bead and fixture) and sets `numeric = true`;
`plan_case` stays `"words"`, and the control becomes runnable at the next seeding under the same `NC-` number.

A control is a defect, not a boundary: "a cell with 8 points is refused" tests the limit, and its control is the
build in which the minimum count was removed. The controls are run by the control runner (N38b-04) from the gate
scripts of the planting beads.

## Acceptance criteria

- [ ] `counterexamples.toml` holds twenty-five Ground entries in the record of N38b-05 (CX-326 to CX-335 and the
      fifteen late entries), each with the `control_kind`, plant `id`, target, defect, expected observation,
      gates and `due` of the plant table; N38b-05's loader accepts them and counts ten for the chapter; no row of
      `controls.toml` is written by this bead.
- [ ] The block request for fifteen `cx` numbers owned by N30-CX is committed in `allocations.toml`; once issued,
      the late entries carry its numbers in table order with their tags in `slug`, `registry::alloc::validate`
      passes, and no tag remains outside `slug` at M1.
- [ ] After the seeding tool of N38b-05 is re-run, each entry has exactly one control whose `seed` names it,
      with the plant's gates; the second run changes no byte; the join passes; the ten word-stated late entries
      and the FACT entries with `rerun = false` are obligations; a control whose plant is not built is `stale`
      with its planting bead named.
- [ ] Every Ground gate is named by at least one entry: GATE-GROUND-01 to -11 (GATE-GROUND-09 through CX-G-13 and
      CX-G-14, GATE-GROUND-10 through CX-G-15); CX-334 and CX-335 are runnable at M1 against GATE-GROUND-01.
- [ ] The arithmetic in each case reproduces: 3.33, 6.67, 4 and 8 N; 13.9%, 9.5% and 4.95%; 36 of 40; 32 ms;
      80 mm and 20 mm.
- [ ] A control that has never been caught in any recorded run is listed for review at each milestone exit.

## Test plan

### Unit tests

`crates/frob-conformance/tests/controls_ground.rs`: `twenty_five_entries_load` (ten numbered and fifteen late
entries parse with N38b-05's loader; a scratch entry carrying a field of its own, such as `nc_id`, is rejected);
`each_entry_names_a_ground_gate`; `every_ground_gate_01_to_11_named`; `plants_named_by_planting_bead` (every plant
`id` is `<planting bead>:cx-<nnn>` with the entry's own number, and the planting bead is live in the bead index);
`late_block_of_fifteen` (the ledger holds the request; after issue the fifteen numbers are contiguous, in table
order, with the tags in `slug`); `word_cases_are_obligations` (the ten word-stated entries carry `numeric = false`
and seed as obligations; a scratch copy with `case_supplied_by` filled and `numeric = true` seeds them runnable
under the same `NC-` numbers); `fact_entries_follow_pilots` (on a scratch register, CX-G-08 and CX-G-11 carry
`rerun = false` while `PILOT-PT-17` and `PILOT-PT-03` read `registered`, and `rerun = true` with the result's
receipt when they read `reproduced`; `rerun = true` against a `registered` record is refused
`rerun-disagrees-with-pilot`); `friction_alias_arithmetic` (min(0.2·I/0.06, μ·N) at the six operating points,
exact to the printed digits); `zero_failure_bound` (1 − 0.05^(1/n) at n = 20, 30, 59 gives 0.1391, 0.0950,
0.04951; n = 58 gives 0.05034, above 5%); `pooled_coverage_counts` (20 + 16 = 36 of 40 = 0.90; 16 of 20 = 0.80);
`exposure_alias_separations` (v·δ + b = 40 mm for both explanations at +2 m/s; 80 mm apart at −2 m/s; 20 mm apart
at +1 m/s); `stale_plants_name_their_bead`. All run as `cargo test -p frob-conformance --test controls_ground`.

### End-to-end script and logging

No script of its own. Each control is exercised by the script of the gate it belongs to with the helper's
negative-control form (`e2e_expect_caught <control> -- <check>`), which logs the control identifier, the check
that caught it and the first differing value.

### Known answers and negative controls

This bead is the negative-control source for the chapter. Its own controls, each on a scratch copy of
`registry/`: CX-330's entry deleted after seeding makes the join fail on the control whose `seed` names the
missing entry; an entry whose plant `id` has no planting bead is rejected; a late entry left under its tag as its
`id` at M1 fails `provisional-tag-past-due`.

## Considerations

- The tags `CX-G-01` to `CX-G-15` are the working names of the late cases; they stay in `slug` after the block is
  issued and are never a catalogue identifier.
- CX-013 (an omitted-law fixture that passes every in-family residual check and reverses the decision) belongs
  to the bets chapter and is the control named in the bet on calibration bands; CX-G-11 is its Ground-side
  entry and cites it.
- Several controls need a mutated build. The planting bead builds the mutant behind its `cx-<nnn>` feature, which
  compiles only under `gauntlet-mutations` and never in a release profile (N38b-04), so a mutant can never ship.

## References

Appendix N (N.27); §30.2, §30.3, §30.5, §30.7 to §30.14, §30.18, §30.21 to §30.24; §38.11; BET-28; CX-013.
