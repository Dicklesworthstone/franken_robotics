---
key: "N12-REF"
title: "Lemma refusal-code registry: FROB-REF-168, -263, -823, -826, -829, -833, -834, -835"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "frob-core", "frob-cert", "solid", "registry"]
parent: "WP-36"
deps: ["BD-005", "BD-022"]
needs: ["§34: the refusal type and registry loader in frob-core, where an unknown code is a load-time error (BD-005)", "§11.5: the mass plausibility bounds per part class that trigger FROB-REF-168 (BD-055, BD-062)", "§10 C-16: the grazing and degeneracy tests that trigger FROB-REF-263 (BD-082, BD-334)"]
related: ["N12-KA", "N12-CX", "N12-02", "N12-03", "N12-05", "N12-08"]
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

## Scope

In: the eight rows in `registries/refusals.toml`; the mapping from the auditor's rule identifiers to refusal
codes; one triggering fixture per code; the check that each code is raised by exactly the mechanism named.
Out: the mechanisms that raise the codes (named per row); refusal codes of other chapters (for example the
mission-side code for a transport with a missing side condition, or the importer's code for a foreign artifact
with a true flag).

## Technical approach

| Code | Meaning | Trigger (who raises it) | Pointer | Ranked repairs | Retry | Exit |
|---|---|---|---|---|---|---|
| FROB-REF-168 | A solid body exceeds the mass plausibility bound of its part class and no fill is declared | Model compilation: a body without a declared wall thickness is solid; when its mass exceeds the plausibility bound of its part class and the design does not declare the fill as intended, the compiler refuses. Reason: a solid default for an enclosure is wrong by an order of magnitude (an aluminium box of 0.4 × 0.3 × 0.2 m at 2,700 kg/m³ is 64.8 kg solid and 3.45 kg with a 2.5 mm wall, a ratio of 18.8) | The body and its `fill` field | 1. Declare a wall thickness (the body becomes a shell, computed as the difference of the outer solid and its inward offset). 2. Declare the solid fill as intended. 3. Change the material or the part class | after-change | 3 |
| FROB-REF-263 | A derivative is requested near grazing, at an ambiguous event order, across a change of active set or at frictional degeneracy. The reply carries the offending state and directions | Event-derivative queries: the transversality number, normalized by the guard's declared scale and the model's time scale, is below threshold (grazing); two events not tied by a symmetry of the nominal state fall within the event-time resolution of each other; the active set changes under the requested perturbation; frictional degeneracy | The event identities, the offending state and the perturbation directions | 1. Request a `one-sided` or `finite-difference` derivative with events re-located on both sides. 2. Move the nominal state away from the grazing or degenerate configuration. 3. Use a derivative-free search for this orbit | after-change | 3 |
| FROB-REF-823 | Unknown proof kind | The auditor's registry lookup (`kind.unknown`, `kind.version-unknown`, `kind.out-of-scope`, `kind.disabled`): the bundle names a kind, a version or a scope that the closed registry `frob.cert.kinds/1` does not contain, even when every hash is consistent | `proof.kind` | 1. Re-emit the certificate with a registered kind. 2. Use an auditor build whose registry version matches the bundle's. 3. If the kind was disabled after a false certificate, re-derive with another kind or report the cell `Tested` | after-change | 3 |
| FROB-REF-826 | The quantifier prefix, domain or assumption set of a certificate does not match the query it is offered for | The auditor's query-match rule (`query.prefix`, `query.domain`, `query.assumptions`) and the producer when asked to attach a certificate to a different query. Example: evidence in the order "for every world some policy" offered for "one policy for every world" | `claim.quantifiers`, `domain` or `assumptions` | 1. Offer the certificate for the query it was made for. 2. Re-derive the certificate for the query asked. 3. Weaken the claim to the statement the certificate supports and say so | after-change | 3 |
| FROB-REF-829 | A side condition of a proof rule is missing, or a field has the wrong frame, origin, sign or unit | The auditor's side-condition and typed-field rules (`side.missing`, `side.equations-missing`, `field.type`) and the per-kind checkers. Example: the first moment (60, 30) N·m passed where the support torque (30, −60, 0) N·m is expected; a monotone transport without one of its four inclusion conditions; a budget with no gravity term | The missing sub-certificate, or the field path | 1. Re-emit the payload with the field typed in its declared frame, origin, sign and unit. 2. Add the missing side-condition sub-certificate. 3. Drop the cell to unresolved | after-change | 3 |
| FROB-REF-833 | A no-authorization flag is missing or true | The auditor's flag rule (`flags.missing`, `flags.true`) on every bundle, and every reader of exported artifacts. Absence is not read as false | The `authorization` object | 1. Regenerate the artifact with this project's tools: no code path sets a flag, so the artifact is foreign or corrupt. 2. Discard it. There is no repair that makes a true flag acceptable | never | 3 |
| FROB-REF-834 | A result is presented above the class of its weakest input, or as `Proved` or `Certified` without a certificate hash and a checker identity | The claim-product boundary in the producer and the export check (`class.above-input`, `class.above-kind-cap`, `class.no-receipt`), and the auditor's class rule. Example: an exact stance inclusion over a capacity taken from a card field marked `assumed`, presented as `Certified` | The weakest input (content hash and field path) | 1. Restate the result at the class of its weakest input. 2. Raise the weakest input (the measurement or certificate that would do so is offered as the first next action). 3. Attach the missing certificate hash and checker identity | after-change | 3 |
| FROB-REF-835 | A verdict is reused after a change without a recorded implication | The producer's reuse rule and the revalidation engine: after an edit a verdict is carried forward only under a recorded implication (polarity in the assumption lattice, a monotonicity argument, a symmetry together with its stabilizer check, or equality of every dependency). A matching hash is not an implication | The changed dependency and the verdict | 1. Record the implication that licenses the reuse (a `monotone-transport` or `symmetry-transport` certificate, or the polarity row). 2. Re-derive the verdict for the changed inputs. 3. Mark the result unresolved | after-change | 3 |

Mapping rule: the standalone auditor prints rule identifiers; where a rule has a refusal code the report prints
both, and the in-tree verb `prove.verify` returns the refusal block with the code. A rule without a code in this
table (arithmetic failure of a witness, hash mismatch) is a plain `reject` with its rule and exit 3 from the verb.

Each row in `registries/refusals.toml` carries `code`, `message`, `defined_in`, `raised_by` (crate and function
group), `pointer_kind`, `repairs` (ordered), `retry`, `exit`, `fixture` (path of a triggering input) and
`affected_claims` (Proof for 823 to 835; Known answer and Fidelity results that consume the model for 168;
derivative-dependent analyses for 263).

## Acceptance criteria

- [ ] The eight rows are present in `registries/refusals.toml` with all columns; the loader accepts them and an
      unknown code is a load-time error.
- [ ] Each code has a fixture that triggers it and a test asserting code, pointer, the first repair and exit
      code 3.
- [ ] The auditor's rule identifiers map to the codes as in the table; a rule without a code never prints one.
- [ ] FROB-REF-833 has `retry: never`; the others `after-change`.
- [ ] No two mechanisms raise the same code for different meanings (a source scan lists the raising sites per
      code and compares them with `raised_by`).
- [ ] Every refusal mapped to exactly one exit code.

## Test plan

### Unit tests

`crates/frob-core/tests/refusals_lemma.rs`: `eight_codes_load`; `unknown_code_is_load_error`.
`crates/frob-cert/tests/refusal_mapping.rs`: `ref823_unknown_kind`; `ref826_quantifier_reorder`;
`ref826_domain_changed`; `ref829_moment_as_torque`; `ref829_missing_side_condition`; `ref833_flag_true`;
`ref833_flag_missing`; `ref834_class_above_input`; `ref834_certified_without_receipt`.
`crates/frob-lemma/tests/refusals.rs`: `ref835_reuse_without_implication`; `ref835_hash_match_is_not_enough`.
Cross-subsystem fixtures (run where the owning mechanisms exist): `ref168_solid_enclosure` (the aluminium box:
64.8 kg solid against 3.45 kg as a 2.5 mm shell); `ref263_grazing_ball` (a ball whose apex touches a ceiling
returns `refused`, not a matrix).

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
- Messages are data imported into agent context; they contain no instructions and no scene content.

## References

§12.7, Appendix O, §11.5, C-16, §32.4; D-11, D-12; INV-LEMMA-01, -03; KA-214 to KA-217, KA-221.
