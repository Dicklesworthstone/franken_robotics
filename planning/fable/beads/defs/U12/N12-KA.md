---
key: "N12-KA"
title: "Lemma known-answer registry: KA-207 to KA-227 (proofs, certificates, the offline auditor)"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-conformance", "frob-cert", "solid", "registry", "test"]
parent: "WP-36"
deps: ["BD-025", "BD-026", "N00-KA", "N10-KA1", "N10-KA3", "N38a-12", "N38b-05", "STD-TEST"]
needs: []
related: ["N12-REF", "N12p2-02", "N38a-13"]
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

In: the twenty-one rows in the suite's one known-answer registry,
`crates/frob-conformance/registry/known_answers.toml` (same file and schema as the suite's other chapters; this
bead first named a file of its own, `registries/known-answers/lemma.toml`, which is not used); the binding of
each row to a test identifier; the registry check; one fixture file per row where the row needs one; the
owner's control for KA-210.

Out, with the bead that has it:
- the tests themselves (owned by the beads named in the last column); known answers of other chapters;
- the controls owed by the five rows that state none and gain none here (KA-222 to KA-226), and a second
  control for KA-210: proposed, checked against the binding rule for controls and registered as records of the
  negative-control registry by N38b-30, which depends on this bead. That bead does not edit a row; it adds
  control records whose `seed` is the row's number, and this registry's check then finds them.

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
| KA-212 | §12.7 | Rewrite trace with a reassociation filed as class E | `reject` | exact (verdict) | G0 · auditor | The row is itself the control | N12p2-02 |
| KA-213 | §12.7 | Energy budget without its gravity term | `reject` | exact (verdict) | G0 · auditor | The row is itself the control | N12-17 |
| KA-214 | §12.7 | Bundle with `execution_authorized` set to true | `reject` (FROB-REF-833) | exact (verdict) | G0 · auditor | The row is itself the control | N12-03, N12-11 |
| KA-215 | §12.7 | Bundle with an unknown proof kind and consistent hashes | `reject` (FROB-REF-823) | exact (verdict) | G0 · auditor | The row is itself the control | N12-02, N12-03 |
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

Registry mechanics. Each row is one `[[ka]]` record of N38a-12's schema, field for field: `id`; `section` (the
"Stated in" column), with `owner` = the gate, `PO-LEMMA-nn` or `INV-LEMMA-nn` the row serves where the case names
one; `fixture` and `same_as` (below); `case`; `truth`; `tolerance`; `tier` and `sites` (the "Tier · site"
column); `control` (`text` = the "Negative control" cell as printed, "The row is itself the control" kept as
printed, empty for "none stated"; its `nc` list is filled by N38b-05's seeding, and this bead writes no record of
`controls.toml`); `derivation` (`docs/derivations/<fixture>.md`); `label`; `due` (the milestone of the realizing
bead named first); `printed` (empty: no row of this chapter printed a value its recomputation disagrees with);
`slug` (empty). The "Realized by" column is not a field: it is written, with each test's crate and name, into the
front matter of the row's derivation file, and the bead named first supplies the runner binding `ka:<id>@<site>`
(N38a-13). DESIGN, correcting an earlier draft that used a row format of its own (`site`, `negative_control`
with the literals `self` and `none-stated`, `tests`, `fixtures`), which N38a-12's parser refuses as unknown
fields.

Fixtures. A case this chapter states again from an earlier chapter is one fixture with two rows and one
derivation file, and the later row's `same_as` names the earlier one: KA-207 and KA-208 take `stance-family`
(`same_as` KA-019), KA-209 and KA-210 take `docking-fixture` (`same_as` KA-023), KA-212 takes the fixture of the
rewrite control in `f32` (`same_as` KA-087), KA-213 the fixture of the energy budget at a guided stop (`same_as`
KA-153), KA-214 and KA-216 take `claim-product` (`same_as` KA-030, as N37-KA does for KA-828), and KA-217 takes
`quantifier-order` (`same_as` KA-032); this bead appends its rows to those derivation files' front matter and
writes no second file. The other rows have fixtures of their own (`lemma-<case>`) with `same_as` empty.

The registry check (`ka_registry_lemma.rs`, below) fails if a row due at the selected milestone has a derivation
file that names no realizing test, if a named test does not exist, or if a row with a stated negative control has
no record in `controls.toml` after N38b-05's seeding. Rows whose control text is empty are reported
`obligation: control` by N38a-12's loader and listed by the check (six rows as printed: KA-210, KA-222, KA-223,
KA-224, KA-225, KA-226).

Where the six controls come from, so that no row has its control specified in two ways:

| Row | Control | Registered by |
|---|---|---|
| KA-210 | First (the owner's): a late-packet exclusion bundle whose two worlds' packet histories differ in one packet must be rejected. Second: the same exclusion bundle with its arrival field edited to "on time" and its hashes recomputed must be rejected | this bead; N38b-30 |
| KA-222 | Key-order canonicalization disabled in the writer: bytes differ | N38b-30 |
| KA-223 | The chunk counter not advanced: mismatch on inputs longer than one chunk (1,024 bytes) | N38b-30 |
| KA-224 | One row's polarity flipped: fresh evaluation contradicts it | N38b-30 |
| KA-225 | Interval product without outward rounding: enclosure fails on a corpus item | N38b-30 |
| KA-226 | Levels α/i²: the sum is α·π²/6, above α | N38b-30 |

A row may carry more than one control record; the check counts a row as covered when at least one is
registered and runnable.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers.toml` holds exactly KA-207 to KA-227 for this chapter, with
      the columns above.
- [ ] Every row's derivation file names at least one realizing test; the registry check passes at the milestone
      where the row's realizing bead lands, and before that the runner reports the row per site as N38a-13
      does (`unbound` once due with no function bound), never as a pass.
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
before the added control, five after it, and none once the control records of N38b-30 exist);
`ka210_added_control_rejects`; `row_missing_test_fails_check`
(negative control of the check: delete one binding).

### End-to-end script and logging

The rows are exercised by `lemma_kernel_cert.sh` (KA-211, KA-220), `lemma_auditor_mutations.sh` (KA-207 to
KA-218, KA-222), `lemma_second_machine.sh` (KA-219, KA-224), `lemma_flags_claims.sh` (KA-214, KA-216, KA-221),
`lemma_reflect.sh` (KA-227); each script logs the KA identifiers it asserts in its `assert` lines. Three rows have
no end-to-end script by nature: KA-223 and KA-225 are build-time differential tests of the auditor (N12-10) and
KA-226 is a host-site enumeration (CARD-T-26, N12-13); their evidence comes from the test kit's
`frob.testevidence/1` lines, which the registry check reads, so every row has a recorded run.

### Known answers and negative controls

This bead is the registry. Its own negative control is the deleted binding. Determinism: all rows are exact
verdicts, bytes or digests; none has a floating tolerance.

## Considerations

- Oracles are pinned: specimens and mutants are fixtures, never regenerated from the candidate.
- KA-227's 60 s is a TARGET until measured on the Linux host.
- New Lemma known answers continue the suite's numbering at its end; they are not inserted here.

## References

Appendix M.11; §12.2 to §12.7; §38.11 (registry paths); GATE-LEMMA-00, -03, -04, -05, -06; PO-LEMMA-01 to -04;
INV-LEMMA-02, -03.
