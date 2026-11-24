---
key: "CARD-T-27"
title: "Theorem card T-27: a class-E rewrite trace maps source to target using only rules of the closed table"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "lemma", "frob-cert", "frob-kir", "frontier", "test"]
parent: "WP-66"
deps: ["N12-06", "N12p2-02", "N21a-13", "N21a-19", "SPEC-C-14"]
needs: []
related: ["CARD-T-35", "GB-25", "N12-28", "N21a-18", "N33c-21", "N33c-22"]
gates: []
plan_refs: ["§12.3 (T-27)", "§12.7", "§10 C-14", "§21.7", "§28 (DC-11)", "§38.7 (MR-38)"]
---
## Background

A kernel's arithmetic is a straight-line program generated from symbolic expressions. Rewrites of it fall into
two classes. Class E changes no bit under round-to-nearest with contraction off: commutation,
common-subexpression elimination, dead-code elimination, scaling by a power of two away from overflow and
underflow, and scheduling (reordering independent operations), together with the exact simplifications x·1,
x + (−0), double negation, select simplification and folding of exact constant operations. Class R is equal over
the reals only: reassociation, distribution, fused multiply-add. A class-E rewrite keeps the kernel's identity
and its evidence; a class-R rewrite makes a different kernel with a new identity, an error bound and a fidelity
run. Replay, digests and both evidence ledgers depend on bits, so a rewrite wrongly filed under class E would
silently invalidate them. The certificate of a class-E rewrite is (source hash, target hash, rule trace),
replayed by an independent checker.

Statement (T-27). A class-E rewrite trace maps the source program to the target program using only rules of the
closed table: replaying the trace's steps, each checked against its rule's syntactic precondition, transforms
the source into a program whose canonical text hashes to the target hash, and no step uses a rule outside the
table.

Route: trace replay by an independent checker (`rewrite-trace`). Tier: Frontier. What the card establishes is
syntactic conformance to the table; that each rule of the table is bit-exact is shown once per rule on its
domain and by digests matching on a pinned corpus on every backend (a test), and equality over the reals is the
theorem a reflector could prove (N12-28). Rejected alternative: accepting a rewrite because outputs matched on
test vectors.

## Scope

In: statement document, registry row, the per-rewrite `rewrite-trace` certificates bound to this card, the
closed rule table as registered data, fixtures and mutants. Out: finding rewrites and deciding whether to apply
them (the decision core and kernel specialization), including the core's rule that a kernel keeps its identity
across a rewrite only with an accepted class-E certificate, and that rule's test (DC-11, N28a-28, at the same
milestone and not a dependency of this card); the rule table's content and the trace format (N21a-19);
the auditor's checker (N12p2-02, M2); the Lean reflector
(N12-28); class-R rewrites (they need a polynomial-identity certificate, an error bound, the fidelity battery
and a new kernel version).

## Technical approach

1. The rule table is one registered artifact (`registries/rewrite-rules.toml`, version-pinned with the proof-kind
   registry): for each rule its name, its syntactic precondition, its transformation, and its class. Its content
   is the engine's closed table (N21a-19), under the names its trace format uses, which are the names the checker
   (N12p2-02) replays; no second vocabulary exists. Class E rules: E1 `share`, E2 `dead`, E3 `comm-add`, E4
   `comm-mul`, E5 `mul-one`, E6 `add-negzero` (the literal −0 only), E7 `neg-neg`, E8 `select-same`, E9
   `select-const`, E10 `fold-exact`, E11 `pow2-scale` (with the declared range excluding overflow and underflow),
   and `schedule` (N21a-18's exchange of two adjacent independent operations). Listed for refusal with class R:
   `reassociate`, `distribute`, `fma`, `reciprocal`.
2. A certificate carries the source and target programs as canonical `frob.kir/1` bytes, their hashes, and the
   ordered steps of the trace (`frob.kir.trace/1`: rule, node, operands before, result node). The auditor
   replays: at each step the precondition must hold syntactically at the node, and the transformation is
   applied; at the end the canonical bytes must hash to the target.
3. `fold-exact` folds only operations whose mathematically exact result is representable in the element type
   (N21a-19's E10: division and square root only when exact); the checker re-derives exactness from the literals
   in exact rationals, it does not trust a flag.
4. Class and limits: `Certified` for conformance of the trace. Bit-exactness in floating point of class E is a
   test (the metamorphic relation "a class-E rewrite changes no bit; a specialized kernel is bit-identical to its
   generic kernel", with the negative control "a reassociation filed as class E"); the card does not replace it.
5. Relies on it: construction C-14; the rewrite decision core (a class-E rewrite changes no identity; 1,000
   worlds keep their digests after it); specialized kernels for survivors.
6. Artifacts: `docs/math/cards/T-27.md`; registry row; the rule table; specimens
   `rewrite-trace/accept-T-27-*.frobcert` and mutants.

## Acceptance criteria

- [ ] `frob-verify` accepts a specimen trace using commutation, common-subexpression elimination, dead-code
      elimination and power-of-two scaling, and rejects: a trace containing a reassociation filed as class E; a
      trace with a fused multiply-add; a step whose precondition does not hold at its location; a trace whose
      result does not hash to the target; a `pow2-scale` step without its range declaration.
- [ ] The rule table is registered data with a digest; a trace that names a rule absent from the pinned table is
      rejected.
- [ ] Registry row `checked`, class `Certified`, with hash and checker identity; the row states that bit-exactness
      is tested, not certified.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t27.rs`: `class_e_trace_accepts`; `reassociation_as_class_e_rejects`;
`reassociation_single_precision_witness` (a = 10⁸, b = −10⁸, c = 1 in single precision: (a + b) + c = 1 and
a + (b + c) = 0, because the spacing of single-precision values near 10⁸ is 8, so b + c rounds to b);
`fma_rejects`; `precondition_false_rejects` (a `share` step naming two expressions that differ);
`target_hash_mismatch_rejects`; `fold_exact_rejects_inexact_constant` (folding 1/3 is not exact);
`rule_not_in_pinned_table_rejects`; `scale_pow2_without_range_rejects` (a `pow2-scale` step by 2⁴ with no declared
range excluding overflow and underflow: FROB-REF-829, missing side condition); `table_equals_engine_table` (the
registered table's rule names and classes equal N21a-19's eleven rules plus `schedule`, so a rule renamed or added
on one side only fails).

### End-to-end script and logging

`scripts/e2e/lemma_auditor_mutations.sh` (bead N12-34) runs the specimens and mutants and logs the first
non-class-E rule; the kernel work's script logs digests before and after an accepted rewrite on the pinned
corpus. For this card their NDJSON lines (STD-E2E helper) carry `ts`, `script`, `bead` ("CARD-T-27"), `step`
(`T-27/<specimen-or-mutant>`), `event`, the exact command line, exit code, wall time, the source and target
hashes, the rule-table digest, the number of steps replayed, the expected and observed verdict, and on a
rejection the first failing step (index, rule, location) and the path to reproduce.

### Known answers and negative controls

KA-212 (rewrite trace with a reassociation filed as class E → `reject`). CX-111. The single-precision witness
above. Negative controls as listed. Determinism: exact; replay is syntactic.

## Considerations

- A skipped rewrite costs a few per cent of speed; a silent bit change invalidates replay and both ledgers. The
  asymmetry is why the table is closed and small.
- The table changes only with a registry version bump; each new class-E rule must be shown exact once on its
  domain before it is admitted.
- Budgeted mode of the decision core: 60 s of search per kernel; one class-R candidate per release.

## References

§12.3, §12.7; C-14; DC-11; KA-212; CX-111; the class-E metamorphic relation.
