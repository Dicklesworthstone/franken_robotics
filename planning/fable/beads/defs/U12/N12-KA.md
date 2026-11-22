---
key: "N12-KA"
title: "Lemma known-answer registry: KA-207 to KA-227 (proofs, certificates, the offline auditor)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-conformance", "frob-cert", "solid", "registry", "test"]
parent: "WP-36"
deps: ["BD-025", "STD-TEST", "BD-026"]
needs: ["§38.11: the loader of the known-answer registry and the negative-control registry in frob-conformance (BD-146)"]
related: ["N12-CX", "N12-REF", "N12-34", "N12-35", "N12-33", "N12-11", "N12-10"]
gates: []
plan_refs: ["Appendix M.11", "§12.7", "§12.2", "§12.3", "§12.4", "§12.6", "§38.2"]
---
## Background

A known answer is an analytic or synthetic truth the implementation must recover, with a derivation on file, a
tolerance, a verification tier and, wherever one exists, a negative control: a planted defect the test must
catch, because a gate that nothing can fail validates nothing. The known-answer suite is the content of
verification tier G1, and its rows are also cited by gates at other tiers. Lemma's rows are KA-207 to KA-227,
twenty-one in all. Almost all of them are verdicts of the offline auditor `frob-verify` (execution site
`auditor`): accept this bundle, reject that mutation. They are the fixed points against which every checker,
every specimen and every end-to-end script of the subsystem is judged.

This bead registers the rows as data in the known-answer registry of the conformance crate, binds each to the test
that realizes it, and checks that every row has a realizing test and every stated negative control is registered.
The mechanism beads cite these numbers and restate the values they need. Tier: Solid.

## Scope

In: the twenty-one rows in `registries/known-answers/lemma.toml` (same schema as the suite's other chapters);
the binding of each row to a test identifier; the registry check; one fixture file per row where the row needs
one. Out: the tests themselves (owned by the beads named in the last column); known answers of other chapters.

## Technical approach

Tiers: G0 laws and self-tests; G1 known answers; G2 parity; G3 metamorphic; G5 determinism. Sites: `auditor`
(`frob-verify`), `host` (exact arithmetic in a test binary). "The row is itself the control" means the row's
case is a rejection, so the row is the negative control.

| KA | Stated in | The case | The truth | Tolerance | Tier · site | Negative control | Realized by |
|---|---|---|---|---|---|---|---|
| KA-207 | §12.7 | Stance bundle at b = 3/14 m (`lp-primal`, forces (160, 130, 110, 0) N at four load vertices) | `accept` | exact (verdict) | G1 · auditor | Each of the four stance mutations gives `reject`: a swapped foot order with unchanged forces; an unreported positive force floor; a passive-authority tag; (60, 30) passed as the torque | BD-132, N12-34 |
| KA-208 | §12.7 | Dual witness at b = 1/5 m (`lp-farkas`) | `accept` for the exclusion | exact (verdict) | G1 · auditor | A perturbed multiplier gives `reject` | BD-132, N12-34 |
| KA-209 | §12.7 | Docking cell (U, ε) = (¾, ¼), packet on time | `accept` for the inclusion (ε < (U + ½)/4) | exact (verdict) | G1 · auditor | The same bundle with the quantifiers reordered gives `reject` | BD-132, N12-03 |
| KA-210 | §12.7 | Docking, packet late | `accept` for the exclusion at every U and ε | exact (verdict) | G1 · auditor | none stated | BD-132 |
| KA-211 | §12.7 | T-01 for the M0 Cayley kernel: cay(Ω)ᵀ·cay(Ω) = I and det cay(Ω) = 1 | `accept` | exact (verdict) | G0 · auditor | One changed coefficient gives `reject` | N12-33, CARD-T-01 |
| KA-212 | §12.7 | Rewrite trace with a reassociation filed as class E | `reject` | exact (verdict) | G0 · auditor | The row is itself the control | N12-18 |
| KA-213 | §12.7 | Energy budget without its gravity term | `reject` | exact (verdict) | G0 · auditor | The row is itself the control | N12-17 |
| KA-214 | §12.7 | Bundle with `execution_authorized` set to true | `reject` (FROB-REF-833) | exact (verdict) | G0 · auditor | The row is itself the control | N12-03, N12-11 |
| KA-215 | §12.7 | Bundle with an unknown proof kind and consistent hashes | `reject` (FROB-REF-823) | exact (verdict) | G0 · auditor | The row is itself the control | N12-02 |
| KA-216 | §12.2 | Artifact in which a no-authorization flag is missing | `reject` (FROB-REF-833); every exported artifact carries the three flags, all false (INV-LEMMA-03) | exact (verdict) | G0 · auditor | The row is itself the control | N12-03, N12-11 |
| KA-217 | §12.4 | Bundle whose quantifier prefix or domain differs from the query while every hash is consistent | `reject` (FROB-REF-826) | exact (verdict) | G0 · auditor | The row is itself the control | N12-03 |
| KA-218 | §12.6 | Per-kind specimens (GATE-LEMMA-03) | Every proof kind in use has one specimen that `frob-verify` accepts and at least three mutations that it rejects; rejection of the four stance mutations, an unknown kind with consistent hashes and a changed quantifier is 100%; the binary lists no dependency outside the standard library and runs in an empty environment | exact (100%) | G0 · auditor | The mutations are the controls | N12-34 |
| KA-219 | §12.6 | Seeded-defect suite on the first mission slice (GATE-LEMMA-04) | Zero accepted false certificates; a re-check on a second machine accepts every shipped bundle | exact (zero; one accepted false certificate fails the gate) | G0 · auditor | The seeded defects are the controls | N12-35, BD-364 |
| KA-220 | §12.6 | Instance certificates for the M0 kernels (GATE-LEMMA-00) | T-01, T-03 (‖q·c‖² = ‖q‖²·‖c‖²) and T-04 (Ad(gh) = Ad(g)·Ad(h); the group coadjoint is the inverse transpose) are certified and replay offline | exact (verdict) | G0 · auditor | One changed coefficient gives `reject` | N12-33 |
| KA-221 | §12.2 | Claim product and class order (GATE-LEMMA-06, INV-LEMMA-02) | The class of a combined result is the minimum of its inputs' classes, so an exact argument over an `Estimated` material card is `Estimated`; compile-fail tests show no conversion from a claim product to a factor and no constructor of `Proved` evidence outside `frob-lemma` | exact | G0 · host | A result presented above the class of its weakest input is refused (FROB-REF-834) | N12-11, N12-05 |
| KA-222 | §12.7 | Bundle round trip (PO-LEMMA-01) | Re-serializing a bundle with `frob-verify` gives identical bytes, on every specimen | exact (bytes) | G3 · auditor | none stated | N12-04 |
| KA-223 | §12.7 | The auditor's own BLAKE3 (PO-LEMMA-02) | Equal to the published vectors and to `fs-blake3` on 10⁶ random inputs | exact (digest) | G2 · auditor | none stated | N12-10 |
| KA-224 | §12.7 | Polarity table on the docking and stance fixtures (PO-LEMMA-03) | Each row gives the stated survival, by fresh evaluation of all twelve moves per fixture | exact (verdict) | G1 · auditor | none stated | N12-05 |
| KA-225 | §12.7 | Rational and interval arithmetic of the auditor (PO-LEMMA-04) | Agrees with FrankenSymPy's on a shared corpus, in a differential test at build time; FrankenSymPy is not linked into the auditor | exact | G2 · auditor | none stated | N12-10 |
| KA-226 | §12.3 | Level series of the qualification lane (T-26) | Σᵢ α/[i·(i + 1)] = α (telescoping; α·m/(m + 1) after m terms); for a declared fixed-n rule and sample size the set of verdicts reachable over all outcome counts is as stated, by exhaustive enumeration in exact rationals | exact | G0 · host | none stated | CARD-T-26, N12-13 |
| KA-227 | §12.4 | Reflection pilot specimens (GATE-LEMMA-05): the Farkas reflector on the stance specimen; the rational LDLᵀ reflector on one 4×4 pseudo-inertia with 64-bit numerators | Each `check x` evaluates to `true` in the kernel, with no tactic at checking time and no compiled evaluator | within 60 s per specimen (TARGET) | G0 · host (Lean kernel, Linux) | A closure that cannot avoid a blocked module, or a specimen not evaluated within 60 s, falsifies the pilot | BD-382, N12-38 |

Values a test needs, restated: stance geometry feet (±1/3, ±3/14) m in port order [++, +−, −+, −−], capacities
160 N, weight 400 N, first moments (±60, ±30) N·m, support torque for (60, 30) equal to (30, −60, 0) N·m; dual
multiplier y = (0, ½, ½) on rows (Σ f; f₁ + f₂ − f₃ − f₄; f₁ − f₂ + f₃ − f₄) with right side (400, 180, 30/b), giving
90 + 15/b = 165 N at b = 1/5 m against 160 N; docking bound at U = ¾ equal to 5/16, shared reading at equality
15/16; level series spent α·m/(m + 1).

Registry mechanics: each row has `id`, `section`, `case`, `truth`, `tolerance`, `tier`, `site`,
`negative_control` (text, or the literal `self`, or `none-stated`), `tests` (identifiers of realizing tests) and
`fixtures`. The registry check fails if a row has no realizing test, if a realizing test does not exist, or if a
row with a stated negative control has no registered control test. Rows whose control is "none stated" are listed
by the check so that a control can be added later (six rows: KA-210, KA-222, KA-223, KA-224, KA-225, KA-226).

## Acceptance criteria

- [ ] `registries/known-answers/lemma.toml` holds exactly KA-207 to KA-227 with the columns above.
- [ ] Every row is bound to at least one existing test; the registry check passes at the milestone where the
      row's realizing bead lands and reports `not run` rows before that.
- [ ] Every row with a stated negative control has a registered control test that fails when the defect is
      planted.
- [ ] Rows with "none stated" are listed by the check (six rows) so they are visible, and at least KA-210 gains a
      control in this bead: a late-packet exclusion bundle whose two worlds' packet histories differ in one
      packet must be rejected.
- [ ] The identifiers are never renumbered.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ka_registry_lemma.rs`: `lemma_rows_207_to_227_present`; `each_row_has_test`;
`stated_controls_registered`; `none_stated_rows_listed` (expects KA-210, KA-222, KA-223, KA-224, KA-225, KA-226
before the added control, and five after); `ka210_added_control_rejects`; `row_missing_test_fails_check`
(negative control of the check: delete one binding).

### End-to-end script and logging

The rows are exercised by `lemma_kernel_cert.sh` (KA-211, KA-220), `lemma_auditor_mutations.sh` (KA-207 to
KA-218, KA-222), `lemma_second_machine.sh` (KA-219, KA-224), `lemma_flags_claims.sh` (KA-214, KA-216, KA-221),
`lemma_reflect.sh` (KA-227); each script logs the KA identifiers it asserts in its `assert` lines.

### Known answers and negative controls

This bead is the registry. Its own negative control is the deleted binding. Determinism: all rows are exact
verdicts, bytes or digests; none has a floating tolerance.

## Considerations

- Oracles are pinned: specimens and mutants are fixtures, never regenerated from the candidate.
- KA-227's 60 s is a TARGET until measured on the Linux host.
- New Lemma known answers continue the suite's numbering at its end; they are not inserted here.

## References

Appendix M.11; §12.2 to §12.7; GATE-LEMMA-00, -03, -04, -05, -06; PO-LEMMA-01 to -04; INV-LEMMA-02, -03.
