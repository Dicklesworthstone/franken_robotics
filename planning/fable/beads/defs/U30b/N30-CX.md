---
key: "N30-CX"
title: "Ground counterexample registry: CX-326 to CX-335 and the chapter's unnumbered cases, as negative controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "ground", "frob-ground", "frob-conformance", "solid", "registry", "test"]
parent: "WP-68"
deps: ["BD-025", "STD-TEST"]
needs: ["§38.11: the negative-control registry file frob-conformance/registry/controls.toml and its five rules (BD-025 at M0, BD-146 at M1)", "§30.8: the eight structural-alias fixtures that seed GATE-GROUND-03 (BD-390)"]
related: ["N30-REF", "N30b-22", "BD-139", "BD-388", "BD-390", "BD-411"]
gates: []
plan_refs: ["Appendix N.27", "§30.2", "§30.5", "§30.7", "§30.8", "§30.9", "§30.10", "§30.12", "§30.14", "§30.21", "§30.24", "§38.11"]
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
chapter's registry: the ten numbered entries of the catalogue, and twelve further cases the chapter states with
numbers but which carry no catalogue number. Mechanism beads cite these entries and state the values they need;
this bead holds the full set and the records that seed the control registry.

## Scope

In: the ten numbered counterexamples with their cases, numbers and the rule each protects; the unnumbered cases;
one control record per entry (the control is the build or input in which the rule was removed, not the boundary
case itself); the test that every record resolves to a gate.
Out: implementing the fixtures (the synthetic rehearsal, BD-388, and the mechanism beads); the known-answer
registry of the chapter (`N30-KA`, written with the first half of the chapter).

## Technical approach

**Numbered entries.**

| CX | The tempting shortcut | The smallest case that breaks it, with its numbers | The rule it protects | Control (kind; expected observation) | Gate |
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

**Unnumbered cases stated by the chapter** (registered under local identifiers `CX-G-01` … `CX-G-12` until the
catalogue owner assigns numbers).

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

**Control records (DESIGN).** For each row one record, for example:

```toml
[[control]]
id     = "NC-GROUND-0330"
seed   = "CX-330"
gate   = ["GATE-GROUND-06"]
kind   = "mutation"
target = "frob-ground::band::bucket_key"
defect = "bucket keyed by design only, so two surfaces share one calibration set"
expect = "a band is issued for the pooled set; held-out coverage is 16 of 20 in the shifted regime; the gate fails"
```

A control is a defect, not a boundary: "a cell with 8 points is refused" tests the limit, and its control is the
build in which the minimum count was removed. Records are read by the batteries of the mechanism beads; this
bead supplies them and checks that each resolves.

## Acceptance criteria

- [ ] Twenty-two control records exist (ten numbered, twelve local), each with a gate that exists in the gate
      registry and a target path that exists in the workspace once its mechanism has landed; before that the
      record is marked `pending` with the mechanism bead named.
- [ ] Every Ground gate has at least one control: the lint that refuses a pass without a control is green for
      GATE-GROUND-01 to -09 and -11.
- [ ] The arithmetic in each case reproduces: 3.33, 6.67, 4 and 8 N; 13.9%, 9.5% and 4.95%; 36 of 40; 32 ms;
      80 mm and 20 mm.
- [ ] A control that has never been caught in any recorded run is listed for review at each milestone exit.

## Test plan

### Unit tests

`crates/frob-conformance/tests/controls_ground.rs`: `twenty_two_records_load`; `each_record_names_a_ground_gate`;
`friction_alias_arithmetic` (min(0.2·I/0.06, μ·N) at the six operating points, exact to the printed digits);
`zero_failure_bound` (1 − 0.05^(1/n) at n = 20, 30, 59 gives 0.1391, 0.0950, 0.04951; n = 58 gives 0.05034, above
5%); `pooled_coverage_counts` (20 + 16 = 36 of 40 = 0.90; 16 of 20 = 0.80); `exposure_alias_separations`
(v·δ + b = 40 mm for both explanations at +2 m/s; 80 mm apart at −2 m/s; 20 mm apart at +1 m/s);
`pending_records_name_their_bead`.

### End-to-end script and logging

No script of its own. Each control is exercised by the script of the gate it belongs to with the helper's
negative-control form (`e2e_expect_caught <control> -- <check>`), which logs the control identifier, the check
that caught it and the first differing value.

### Known answers and negative controls

This bead is the negative-control source for the chapter. Its own control: a records file with the record for
CX-330 deleted must make the pass-without-control lint fail for GATE-GROUND-06.

## Considerations

- The local identifiers exist because late and in-text cases have no catalogue number. They are stable inside
  this repository and are replaced only by an assignment from the catalogue owner.
- CX-013 (an omitted-law fixture that passes every in-family residual check and reverses the decision) belongs
  to the bets chapter and is the control named in the bet on calibration bands; CX-G-11 is its Ground-side
  record and cites it.
- Several controls need a mutated build. The conformance harness builds mutants behind a `cfg` that is refused
  in release profiles, so a mutant can never ship.

## References

Appendix N (N.27); §30.2, §30.5, §30.7 to §30.10, §30.12, §30.14, §30.21, §30.24; §38.11; BET-28; CX-013.
