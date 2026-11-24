---
key: "N12-REF"
title: "Lemma refusal-code registry: FROB-REF-168, -263, -823, -826, -829, -833, -834, -835"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "lemma", "frob-core", "frob-cert", "solid", "registry", "review-notes-open"]
parent: "WP-36"
deps: ["BD-005", "BD-022"]
needs: []
related: ["N12-07", "N12x-08", "N13-03", "N13-04", "N13-REF", "N25-REF", "N25e-45"]
gates: []
plan_refs: ["§12.7", "Appendix O", "§11.5", "§10 C-16", "§32.4 (exit codes)", "§9 D-11"]
---
## Background

Every refusal of the system has a stable code, a message and ranked repairs. A refusal is not an error message:
it is a typed answer that says what was at fault, what was observed against what was required, which claims are
affected, whether a retry can help, and what to do next, so that an agent can repair without guessing. The
machine-readable registry is `registries/refusals.toml`; an unknown code is a load-time error and codes are never
reused. The envelope's `refusal` block carries `code`, `message`, `pointer` (the field or part at fault),
`observed`, `required`, `unit`, `affected_claims`, `retry` (`never`, `after-change`, `backoff`) and ranked
`repairs`. Exit codes of the `frob` binary: 3 for a refused input or question (codes 1xx to 5xx, and 8xx other
than conflicts), 7 for conflicts; exit code 1 is never emitted on purpose.

Lemma's chapter lists eight codes. Six are raised by Lemma's own mechanisms (the auditor's generic rules and the
certificate producer). Two are listed with Lemma because their statements rest on Lemma's cards (the inertia
plausibility refusal belongs with pseudo-inertia validation, card T-19; the derivative refusal belongs with the
saltation identity, card T-17), though their triggers live in the model compiler and the event-derivative code.
This bead puts all eight rows in the registry with trigger, pointer, ranked repairs and exit code, and binds each
to the mechanism that raises it and the test that exercises it. Tier: Solid.

Why M0: five of the codes are raised at M0, by the generic rules of N12-03 and the `prove.verify` handler of
BD-027 (823, 826, 829, 833, 834) and by the claim renderer of N00-25 (834), and VERBS-prove's M0 rows list them;
`RefusalCode` is generated from `registries/refusals.toml` and the loader refuses a verb row that names a code with
no row, so the rows must exist before those beads build. A registry row does not wait for the mechanism that
raises it: the rows of 835, 168 and 263 enter here too, and their fixtures run when their mechanisms land.

## Scope

In: the eight rows in `registries/refusals.toml`; the mapping from the auditor's rule identifiers to refusal
codes; one triggering fixture per code; the check that each code is raised by exactly the mechanism named.
Out: the mechanisms that raise the codes (named per row); refusal codes of other chapters (for example the
mission-side code for a transport with a missing side condition, or the importer's code for a foreign artifact
with a true flag).

## Technical approach

| Code | Meaning | Trigger (who raises it) | Pointer | Ranked repairs | Retry | Exit |
|---|---|---|---|---|---|---|
| FROB-REF-168 | A solid body exceeds the mass plausibility bound of its part class and no fill is declared | Model compilation, at design-check stage 2 and in importers that synthesize a fill, under the rule that decision bead N13-03 settles and N13-04 raises. In force until that decision is recorded (its option B, the fail-closed default): the design language has no default fill (an absent fill is its own refusal, FROB-REF-105), so the conventions sentence "a body without a wall thickness is solid" is read as "a body whose written fill is `solid` has no wall"; a written `solid` with no statement of intent is refused when its solid mass exceeds the bound; the bound is one quarter of the design's `mass_max` for every body until a part-class table is approved; a design without `mass_max` is not evaluated and gets the advisory text only. Reason: a solid enclosure is wrong by an order of magnitude (an aluminium box of 0.4 × 0.3 × 0.2 m at 2,700 kg/m³ is 64.8 kg solid and 3.45 kg with a 2.5 mm wall, a ratio of 18.8) | The body and its `fill` field; returned: the body path, the solid mass, the bound and `mass_max` | 1. Declare `{shell: t}` with a wall thickness (the body becomes a shell, computed as the difference of the outer solid and its inward offset). 2. Declare `{lattice: φ}`. 3. Declare the solid fill as intended, `{solid: {intended: true}}`. 4. Change the material or the Region | after-change | 3 |
| FROB-REF-263 | A derivative is requested near grazing, at an ambiguous event order, across a change of active set or at frictional degeneracy. The reply carries the offending state and directions | Event-derivative queries: the transversality number, normalized by the guard's declared scale and the model's time scale, is below threshold (grazing); two events not tied by a symmetry of the nominal state fall within the event-time resolution of each other; the active set changes under the requested perturbation; frictional degeneracy. A model with neither a positive canonical time scale nor a declared `t_scale` is refused as grazing (N10b-01) | Payload: `cause` (one of the four); the offending state and the perturbation directions; for grazing, ν and ν_min (or, from a producer on a tier that locates no events, the normalized row margin `min_margin` and its guard `g` in their place, N16b-24), and `t_scale: "missing"` when the model has no time scale; for an ambiguous order, the two event keys and their time gap | 1. Ask for a `one-sided` derivative on a named side. 2. Ask for `finite-difference` with events re-located on both sides. 3. Ask for a `smoothed` surrogate with a declared parameter. 4. Move the nominal state or the section away from the guard. 5. Use derivative-free search for this orbit. With `t_scale: "missing"` the repair is: declare the model's time scale | after-change | 3 |
| FROB-REF-823 | Unknown proof kind | The auditor's registry lookup (`kind.unknown`, `kind.version-unknown`, `kind.out-of-scope`, `kind.disabled`): the bundle names a kind, a version or a scope that the closed registry `frob.cert.kinds/1` does not contain, even when every hash is consistent | `proof.kind` | 1. Re-emit the certificate with a registered kind. 2. Use an auditor build whose registry version matches the bundle's. 3. If the kind was disabled after a false certificate, re-derive with another kind or report the cell `Tested` | after-change | 3 |
| FROB-REF-826 | The quantifier prefix, domain or assumption set of a certificate does not match the query it is offered for | The auditor's query-match rule (`query.prefix`, `query.domain`, `query.assumptions`, `query.policy-class`: rule 4 of N12-03, which also compares the bundle's `plant.policy_class` with the digest recomputed from the policy-class section of the offered query, since a family certificate is about its declared policy class) and the producer when asked to attach a certificate to a different query. Example: evidence in the order "for every world some policy" offered for "one policy for every world"; the on-time docking inclusion offered for the late-packet query (equal prefix, domain and assumptions, different policy class) | `claim.quantifiers`, `domain`, `assumptions` or `plant.policy_class` | 1. Offer the certificate for the query it was made for. 2. Re-derive the certificate for the query asked. 3. Weaken the claim to the statement the certificate supports and say so | after-change | 3 |
| FROB-REF-829 | A side condition of a proof rule is missing, or a field has the wrong frame, origin, sign or unit | The auditor's side-condition and typed-field rules (`side.missing`, `side.equations-missing`, `field.type`) and the per-kind checkers. Example: the first moment (60, 30) N·m passed where the support torque (30, −60, 0) N·m is expected; a monotone transport without one of its four inclusion conditions; a budget with no gravity term | The missing sub-certificate, or the field path | 1. Re-emit the payload with the field typed in its declared frame, origin, sign and unit. 2. Add the missing side-condition sub-certificate. 3. Drop the cell to unresolved | after-change | 3 |
| FROB-REF-833 | A no-authorization flag is missing or true | The auditor's flag rule (`flags.missing`, `flags.true`) on every bundle, and every reader of exported artifacts. Absence is not read as false | The `authorization` object | 1. Regenerate the artifact with this project's tools: no code path sets a flag, so the artifact is foreign or corrupt. 2. Discard it. There is no repair that makes a true flag acceptable | never | 3 |
| FROB-REF-834 | A result is presented above the class of its weakest input, or as `Proved` or `Certified` without a certificate hash and a checker identity | The claim-product boundary in the producer and the export check (`class.above-input`, `class.above-kind-cap`, `class.no-receipt`), the auditor's class rule, and the not-decidable guard of `frob-lemma` (N12-07: a Proof claim on a subject listed in `registries/not-decidable.toml` cannot carry a receipt, and the row's `instead` text is offered as the first repair). Example: an exact stance inclusion over a capacity taken from a card field marked `assumed`, presented as `Certified` | The weakest input (content hash and field path) | 1. Restate the result at the class of its weakest input. 2. Raise the weakest input (the measurement or certificate that would do so is offered as the first next action). 3. Attach the missing certificate hash and checker identity | after-change | 3 |
| FROB-REF-835 | A verdict is reused after a change without a recorded implication | The producer's reuse rule and the revalidation engine: after an edit a verdict is carried forward only under a recorded implication (polarity in the assumption lattice, a monotonicity argument, a symmetry together with its stabilizer check, or equality of every dependency). A matching hash is not an implication | The changed dependency and the verdict | 1. Record the implication that licenses the reuse (a `monotone-transport` or `symmetry-transport` certificate, or the polarity row). 2. Re-derive the verdict for the changed inputs. 3. Mark the result unresolved | after-change | 3 |

Mapping rule: the standalone auditor prints rule identifiers; where a rule has a refusal code the report prints
both, and the in-tree verb `prove.verify` returns the refusal block with the code. A rule without a code in this
table (arithmetic failure of a witness, hash mismatch) is a plain `reject` with its rule and exit 3 from the verb.

Each row in `registries/refusals.toml` carries `code`, `message`, `defined_in`, `raised_by` (the bead keys
below: the field the registry-wide check of N35-33 reads, which requires a bead key or the word `unbound`),
`raised_at` (crate and function group), `pointer_kind`, `repairs` (ordered), `retry`, `exit`, `fixture` (path of
a triggering input) and `affected_claims` (Proof for 823 to 835; Known answer and Fidelity results that consume
the model for 168; derivative-dependent analyses for 263).

**Raising beads (`raised_by`), with the first milestone at which each raises the code.**
- 168: N13-04 (the rule as N13-03 decides it; M1), surfaced by `design.check` stage 2 (BD-100) and the analysis
  verb `analyze.mass` (N28a-05).
- 263: N16b-24 (labelled fixed-mode step derivatives through a step, M3) and N33b-18 (`analyze.sensitivity`, A-16,
  M3); N10b-01 (the nine derivative-validity labels of C-16, M4); N22a-33 (gait tangent maps: an ambiguous event
  order or frictional degeneracy is returned as 263 itself, grazing and folds are re-raised as the gait code 431,
  M4); passed through by `gait.gradient` (BD-334) and `mission.synthesize`'s `refine` lane (`--strategy refine`,
  BD-362), M4;
  N25d-02 and the adjoint path (N16c-25, N16c-26), M6.
- 823: N12-02 (the closed kind registry and its dispatch), N12-03 (generic rules), BD-027 (`prove.verify`); M0.
- 826: N12-03, BD-027 (M0); the producers that attach a certificate to a query (BD-133, BD-135, M1).
- 829: N12-03, BD-027 (M0); the per-kind checkers of the theorem cards (from M1).
- 833: N12-03, BD-027 (M0); the flag sweep over exported artifacts (N12-11, M0).
- 834: N12-03, BD-027 and the claim renderer N00-25 (M0); N12-05 (class capped by inputs), N12-07 (the
  not-decidable guard) and N12-08 (evidence-class wording), M1.
- 835: N12-05 (the producer's reuse rule) and N28b-32 (the revalidation engine behind `result.revalidate`), M1;
  N10a-11 (the invalidation controls of C-12, M1); N25a-16 (`mission.revalidate`, M3). The doctrine check N10a-08
  exercises it and raises nothing of its own.

## Acceptance criteria

- [ ] The eight rows are present in `registries/refusals.toml` with all columns; the loader accepts them and an
      unknown code is a load-time error.
- [ ] Each code has a fixture that triggers it and a test asserting code, pointer, the first repair and exit
      code 3: at M0 for 823, 826, 829, 833 and 834 (the mutations of N12-03 and BD-027 through `prove.verify`);
      835 and the not-decidable case of 834 when N12-05 and N12-07 land (M1); 168 when N13-04 lands (M1); 263 when
      N16b-24 lands (M3), and its label cases when N10b-01 lands (M4). Until then each such test is listed
      `not yet` with its milestone, never as passing.
- [ ] Every row's `raised_by` names the beads listed under "Raising beads"; a key that is not live in the tracker
      index fails the check.
- [ ] The auditor's rule identifiers map to the codes as in the table; a rule without a code never prints one.
- [ ] FROB-REF-833 has `retry: never`; the others `after-change`.
- [ ] No two mechanisms raise the same code for different meanings (a source scan lists the raising sites per
      code and compares them with `raised_by`).
- [ ] Every refusal mapped to exactly one exit code.

## Test plan

### Unit tests

`crates/frob-core/tests/refusals_lemma.rs`: `eight_codes_load`; `unknown_code_is_load_error`;
`raised_by_names_live_beads` (each row's `raised_by` equals the list above and every key is in the tracker index;
a planted row with `raised_by = "frob-cert::audit"` fails, because a crate path belongs in `raised_at`).
`crates/frob-cert/tests/refusal_mapping.rs`: `ref823_unknown_kind`; `ref826_quantifier_reorder`;
`ref826_domain_changed`; `ref826_policy_class_differs` (the docking golden offered for a copy of its query whose
policy-class section differs only in one packet's arrival instant: rule `query.policy-class`, pointer
`plant.policy_class`); `ref829_moment_as_torque`; `ref829_missing_side_condition`; `ref833_flag_true`;
`ref833_flag_missing`; `ref834_class_above_input`; `ref834_certified_without_receipt`.
`crates/frob-lemma/tests/refusals.rs` (from M1, with N12-05 and N12-07): `ref835_reuse_without_implication`;
`ref835_hash_match_is_not_enough`;
`ref834_not_decidable_subject` (a Proof claim on subject `cross-backend-bit-equality` is refused with 834 and
the row's `instead` text as the first repair).
Cross-subsystem fixtures (run where the owning mechanisms exist): `ref168_solid_enclosure` (the aluminium box:
64.8 kg solid against 3.45 kg as a 2.5 mm shell; refused when written `solid` without intent in a design whose
`mass_max` is below 259.2 kg, four times its solid mass, and accepted as a shell or as an intended solid; the
mechanism's own fixture is the 22.68 kg torso in a 14 kg design, N13-04); `ref263_grazing_ball` (a ball whose
apex touches a ceiling returns `refused`, not a matrix, with `cause` grazing and the five repairs in the row's
order; first runnable with N16b-24 at M3).

### End-to-end script and logging

`lemma_auditor_mutations.sh` logs a `refusal` event with the code for each mutant that maps to one; the model and
gait scripts of the owning subsystems log 168 and 263. Each `refusal` line carries code, pointer, the first
repair and the exit code.

### Known answers and negative controls

KA-214, KA-216 (833), KA-215 (823), KA-217 (826), KA-221 (834); for the two cross-listed codes: the shell-rule
known answer (as wall thickness tends to zero a spherical shell of radius r tends to I = (2/3)·m·r², against
(2/5)·m·r² for the solid ball) and the grazing known answer (the derivative query returns `refused`). Negative
control: a build in which the query-match rule returns a plain reject without its code fails `ref826_*`.
Determinism: codes and pointers are identical across platforms.

## Considerations

- A related code owned by the safety chapter covers a foreign artifact met by an importer (a reader that meets a
  true flag treats the artifact as foreign); the auditor's code for the same condition in a bundle is
  FROB-REF-833. The two are kept distinct on purpose: one is about import, the other about checking.
- The mission side has its own code for a transport with a missing side condition; the auditor reports
  FROB-REF-829 for the bundle that carries such a transport.
- FROB-REF-168 is listed in the proofs chapter's table and ruled by the inertia convention. This bead enters its
  row once, with the wording of the fail-closed default above; the Marrow registry N13-REF cross-references the
  code and adds no row. When N13-03 records its decision the row is amended to the ADR's text in the same
  commit, or tombstoned and never reused if the decision removes the refusal. The mechanism and its fixture are
  N13-04; a registry row does not wait for the mechanism that raises it.
- Messages are data imported into agent context; they contain no instructions and no scene content.

## References

§12.7, Appendix O, §11.5, C-16, §32.4; D-11, D-12; INV-LEMMA-01, -03; KA-214 to KA-217, KA-221.
