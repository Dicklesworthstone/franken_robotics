---
key: "N41-RISK3"
title: "registries/risks.toml, part 3: physics, numerics and device risks (RISK-04 to -08, -13, -31, -32, -40 to -42, -45, -47, -48, -58)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "spark", "kinesis", "traction", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-01"]
needs: ["§16 to §23, §35: the keys named in the Beads column are owned by the chapter units; they are named for traceability, not as blocking dependencies"]
related: ["N41-RISK1", "N41-RISK2", "N41-RISK4", "N41-PM", "N41-02", "N33c-37", "N23b-30", "BD-176", "N21b-34", "BD-213"]
gates: ["GATE-WS-07"]
plan_refs: ["§41.3", "§41.4", "§41.5", "§16", "§18", "§21", "§35"]
---
## Background

Third part of the risk register (conventions in N41-RISK1; schema and checks in N41-01). Its fifteen rows are the
ways the **physics, the arithmetic and the devices** can mislead: a contact model with known artifacts, a referee
that cannot complete the scenes where decisions are made, tiers that agree with each other because they share the
same cards, single precision and backend differences, a GPU gateway with hard limits, kernels that get faster by
quietly doing less, and a noise sampler whose marginals are right while its joint law is wrong.

Terms for a reader of this bead alone. The *referee* is the slow, event-located CPU tier that fast-tier results are
promoted against. *Determinism classes*: D0 exact; D1 bit-equal across backends; D2 bit-equal per backend across
runs, worker counts and batch composition; D3 statistically equivalent at the decision level; D4 uncontrolled. A
*card* is an immutable record of what is known about a part (actuator, interface, material, sensor), each field
with provenance (`measured`, `datasheet`, `generic`, `assumed`) and uncertainty. *Model-conditional* marks a design
ranking that changes under another admissible contact law.

## Scope

In: the fifteen rows below as records of `registries/risks.toml`. Out: the schema, loader and exit review (N41-01);
the planted linear-phase sampler as a GATE-WS-07 instance (N41-31; the sampler itself is carried by the conformance
bead named in the row); the mitigations.

## Technical approach

Record fields as in N41-01.

| ID, severity | Risk | Earliest signal (detector) | Mitigation | Owner | Detected or decided by | Beads that carry the mitigation | Planted instance; first readable |
|---|---|---|---|---|---|---|---|
| RISK-04, Medium | Bit equality across backends is unattainable on a device family | The record of each family's best attainable class from the IEEE probe (GATE-SPARK-01) | KILL-D1; class D3 with the decision-level equivalence test; paired statistics refused under an uncontrolled class; the property claimed is batch-composition invariance (GATE-SPARK-15), with bit equality only where class D1 is attained | Spark lead | GATE-SPARK-01; GATE-SPARK-15; GATE-SPARK-16; KILL-D1 | BD-017, N21b-34, BD-176, BD-177, BD-181, BD-020 | A one-ulp change planted in one backend prelude is reported by the divergence ledger; a paired statistic across device families without class D1 is refused. M0 |
| RISK-05, High | The convex contact model's creep misleads wheel and stair results | Creep and slip quantities in the fidelity ledger; the share of finalists typed `model-conditional` | The four-dimensional cone and joint rows; the second compliant law (Hunt–Crossley); the model-form rank check; the hard-contact law as an arm; KILL-CONTACT | Traction lead | GATE-TRAC-04; GATE-FID-05; GATE-TRAC-09; KILL-CONTACT | BD-154, BD-155, N18a-16, N18a-19, N18b-33, BD-266, BD-386, BET-03 | A finalist whose advantage comes from diagonal traction on a pyramidal cone (up to √2·μ) is typed `model-conditional`; with the model-form stage skipped it is reported as the winner, and that run must fail its control. M2 |
| RISK-06, Medium | Derived kernels are slower than hand-written ones | The M2 profile against the bound of 1.3 (BET-01) | KILL-KIR; specialization only for survivors, since its ceiling on an incumbent trace is about 1.2 | Spark lead | KILL-KIR; KILL-SPECIALIZE | BD-221, N21a-14, BD-371, N21c-18, BET-01 | A synthetic profile with derived kernels at 1.4 times hand-written makes the KILL-KIR trigger evaluate to "fires". M2 |
| RISK-07, Medium | FrankenSymPy budgets block derivations or certificates, by expression swell on a 29-joint humanoid | Derivation time and peak memory per kernel body at INT-FSYM-01 | Derive per kernel body and per limb, never per robot; BET-11; KILL-LEMMA | Lemma lead | INT-FSYM-01; KILL-LEMMA | BD-019, N33a-32, N12-31, BET-11 | A whole-robot derivation request for the 29-joint humanoid is refused by the derivation budget before it runs. M0 |
| RISK-08, Medium | The Metal gateway is synchronous, takes at most 31 buffer arguments, binds at offset zero and compiles from source only (FACT, to re-read at admission) | Launch counts and blocked host time per chunk on Metal | Packed slabs and offset tables; one batch per chunk; upstream work item FD-FT-01; KILL-METAL (the Mac keeps the world bank, proof, replay and `batch-cpu` roles). The earlier fallback of a direct binding is withdrawn: a Metal binding of our own needs an owner decision (OPEN-SPARK-5) | Spark lead | INT-FT-01; GATE-SPARK-04; KILL-METAL | BD-016, BD-028, N21b-08, N21c-33, N21b-09, N21c-38, FD-FT-01, BET-09 | A step plan that binds 32 buffers in one kernel is refused at plan build. M0 |
| RISK-13, Medium | Reference robot imports hide lossy contact mappings | Fields missing from a loss report in the field-coverage diff | Loss reports (GATE-SKEL-01); the mapped soft-contact law labelled parity-only | Skeleton lead | GATE-SKEL-01 | BD-065, BD-066, N14a-31 | An import fixture with one contact field the importer cannot map: the field appears in the loss report; an importer that drops it silently fails the diff. M1 |
| RISK-31, High | The referee cannot complete the scenes where decisions are made: the upstream sustained solve admits at most 21 contacts and keeps fixed witnesses (FACT) | Referee coverage per robot class under 80% at GATE-FID-03 | The transactional step with witness migration; the `strict` profile as named adjudicator; upstream work items FD-FSIM-12 and FD-FSIM-13; BET-34 | Kinesis lead | GATE-FID-03; GATE-KIN-04; GATE-TRAC-06 | BD-081, BD-082, BD-083, BD-161, BD-213, N16a-20, N33a-14, FD-FSIM-12, FD-FSIM-13, BET-34 | A 22-contact set offered to the referee is refused, never truncated; a class whose coverage figure is 70% prints its named adjudicator. M1 |
| RISK-32, High | Common-mode model error: tiers agree with each other and disagree with reality, because they share cards | The share of `assumed` and `generic` card fields in benchmark designs | Provenance on every card field; the report line with that share; the published-anchor comparison; the calibration subsystem; a neutral evaluator in the ten-fold test | Sinew lead | GATE-GROUND-01; GATE-TENFOLD-05 | BD-069, N15b-06, N15b-07, BD-200, BD-139, BD-432 | The same wrong card on every tier: the tiers agree, and the result still prints its `assumed` share and, where the verdict changes sign inside the band, the label card-conditional. M1 |
| RISK-40, Medium | Contact capacity biases a comparison against the morphology with more contacts | `Degraded` shares that differ by class; `capacity-biased` labels | The rule against capacity bias; GATE-KIN-07 (at most 10⁻⁴ `Degraded` world-steps, every one flagged); refusal instead of silent truncation; capacities in every disclosure block | Traction lead | GATE-KIN-07; GATE-TRAC-06 | N16b-03, N16a-14, BD-159, BD-168 | A forced contact overflow is flagged `Degraded` and never dropped; a paired comparison with unequal overflow shares carries `capacity-biased`. M2 |
| RISK-41, Medium | A scene looks plausible and cannot support a robot | The shares of `unresolved` and unintended `impossible` scenes (GATE-HAB-02) | Scene acceptance (GATE-HAB-04); feasibility tags; clearance complexes | Habitat lead | GATE-HAB-02; GATE-HAB-04; GATE-HAB-10 | BD-228, BD-234, BD-233, N19a-43 | A generated room whose only doorway is narrower than every orientation of the nominal footprint is tagged `impossible` with the doorway named. M3 |
| RISK-42, Medium | A model fits nominal motion for the wrong physical reason | Speed matches while current draw and contact work disagree in the energy ledger | The identifiability analysis and GATE-GROUND-03; the `wrong-reason` label; the energy ledger evaluated on real logs | Ground lead | GATE-GROUND-03 | BD-390 | The identifiability fixture: zero sensitivity to friction at 1 A and 2 A, 4 N against 8 N at 3 A under 10 N; a fit that reports friction from the first two runs is flagged. M5 |
| RISK-45, Medium | An optimization changes unrelated semantics: the kernel is faster while contact counts or sensor rates fall | Throughput rises while a count in the disclosure block moves | Digest identity for rewrites that claim bit equality; the decision-level test for the rest; the performance harness of GATE-SPARK-19 | Performance lead | GATE-SPARK-19; GATE-SPARK-16 | BD-216, BD-177 | A "faster" kernel variant that halves the contact samples: the disclosure-count comparison flags it and the speed figure is not admitted. M2 |
| RISK-47, Medium | Search or gradients exploit a contact-order or model-form artifact | A ranking that flips under another admissible law; a derivative used outside its validity label | Derivative validity labels (doctrine rule D-15); the model-form checks (GATE-FID-05, GATE-ATEL-10); KILL-FDGRAD | Kinesis lead | GATE-FID-05; GATE-ATEL-10; KILL-FDGRAD | BD-266, BD-346, N17b-20, BD-353 | A run with one planted mode switch: a gradient reported as trustworthy across the switch fails the validity-label control. M3 |
| RISK-48, Medium | A driver, system or binding update changes kernel results | A fingerprint change; a failed `spark.check` | The fingerprint gate; `--require-current-check`; the kernel cache keyed by fingerprint; pinned bindings | Spark lead | GATE-SPARK-07 | BD-029, N21c-06, N21c-01 | A changed driver string in a scratch fingerprint makes the check stale: a run with `--require-current-check` is refused until `spark.check` is re-run. M0 |
| RISK-58, High | A sampler has correct marginals and a wrong joint law, so the likelihood ratio is computed for a variable that was not sampled and exploration is correlated across channels | A pairwise sign-agreement frequency outside ½ ± 0.005 in the sampler battery; a learning curve that changes when channels are reordered | The joint-law rule for sampled actions and its gate; counter-based draws keyed per channel; no sibling sampler is linked | Cortex lead | GATE-WS-07 (M2 row); GATE-CORTEX-18 | N23b-29, N23b-30, N33c-37, FDEF-06, BD-180, N41-31 | The planted linear-phase sampler (a copy of the sibling defect, whose sign agreement is 0.6655): the battery rejects it in 100 of 100 runs. M2 |

## Acceptance criteria

- [ ] `registries/risks.toml` contains the fifteen records above with no empty field; every `beads` and `detects`
      entry resolves.
- [ ] Every row names a planted instance and its first-readable milestone; RISK-58 carries GATE-WS-07.
- [ ] Thresholds repeated in signals agree with the gate register (1.3 for derived kernels, 80% referee coverage,
      10⁻⁴ `Degraded` world-steps, ½ ± 0.005 sign agreement, 31 buffer arguments as a recorded FACT).
- [ ] The loader's rule test passes: RISK-08 and RISK-31 name upstream work items and also name the interim posture
      or fallback beside each.

## Test plan

### Unit tests
`crates/frob-conformance/tests/risks_part3.rs`: `fifteen_rows_load`; `ids_match_expected_set`; `severity_counts`
(4 High: RISK-05, -31, -32, -58; 11 Medium); `thresholds_agree_with_gate_register`; `upstream_rows_have_posture`;
`every_bead_key_resolves`.

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04); the M2 run fires the sampler instance through the sampler
battery's own script and records the observed sign-agreement frequency against the band.

### Known answers and negative controls
Known answers: the row count, identifier set, severity tally and the thresholds above. Negative controls: a row
whose signal names a gate that does not exist; a row with a threshold that differs from the gate register; the
RISK-08 row with its fallback removed (a bare upstream item).

## Considerations

- RISK-32 has no gate that can pass it: agreement between tiers is never evidence about a card. The row's value is
  that every report prints the share of assumed fields, so the limit travels with the result.
- RISK-04 and RISK-48 are detected by the same machinery (probe, fingerprint, check currency); they stay separate
  rows because one is a property of a device family and the other an event in time.
- Signals of RISK-05 and RISK-47 depend on the fidelity ledger existing at M2; before that the review records "not
  yet readable".

## References

§41.3 (rows RISK-04 to -08, -13, -31, -32, -40 to -42, -45, -47, -48), §41.5 (the sampler row), §16.4, §18.5,
§18.11, §21.9, §35.1, §35.4, §23.25.
