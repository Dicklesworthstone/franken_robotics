---
key: "CARD-T-35"
title: "Theorem card T-35: reflection soundness (a reflected checker returns true only on inputs meeting its spec)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-72"
deps: ["BD-382", "N12-06", "N12-24"]
needs: []
related: ["N12-28", "N12-32", "N12-26", "N12-27", "CARD-T-21", "CARD-T-19", "CARD-T-27"]
gates: []
plan_refs: ["§12.3 (T-35)", "§12.4 (kernel reflection)", "§12.2 (class Proved through a reflected checker)", "§12.6"]
---
## Background

The class `Proved` covers a general statement accepted by a Lean kernel and also an instance evaluated by the
kernel through a reflected checker whose soundness theorem is `Proved`. A reflected checker is a function
`check : Input → Bool` defined in Lean together with a soundness theorem `check x = true → Spec x`. An instance is
proved by the kernel evaluating `check x` to `true`, with no tactic at checking time and no compiled evaluator.
Everything that makes such an instance trustworthy is concentrated in one place: the soundness theorem. If a
reflector's theorem were missing, weakened or about a different function than the one evaluated, every instance
it "proves" would be unsupported. This card is that requirement, stated once for all reflectors.

Statement (T-35). Reflection soundness: each reflected checker returns true only on inputs that satisfy its
specification. For the four planned reflectors:
- `farkas M r y = true →` no x ≥ 0 over any linearly ordered field containing ℚ satisfies M·x = r;
- `ldltPos A = true →` for symmetric A, xᵀ·A·x > 0 for every nonzero x over any linearly ordered field containing ℚ;
- `polyEq p q = true →` p and q evaluate equally in every commutative ℚ-algebra under every assignment;
- `replay src t = some tgt →` src and tgt denote the same real function.

Route: Lean, written in stock Lean with Mathlib, re-checked by FrankenLean. Tier: Frontier. ADR-LEMMA-05:
reflection promotes selected instance certificates and is never the default; instances are checked by kernel
evaluation, never by a compiled evaluator, which keeps the trusted base at the kernel and the soundness
theorems. Rejected alternative: `native_decide`, which would add the compiler and its runtime to the trusted
base.

## Scope

In: the card's statement and registry row; the rule that a reflector is enabled only while its soundness capsule
is sealed, bound and current; the link between the function evaluated and the function proved about; the
negative controls. Out: the reflectors and their proofs (BD-382 for the first two; N12-28 for the other two);
budgets (N12-27); the kill-gate decision (N12-32).

## Technical approach

1. Registry row with one sub-entry per reflector: name, registered `Statement` (by its definition hash), capsule
   hash, checker identities (stock Lean build, `fln` build), status (`hypothesis`, `checked`, `parked`), and the
   definition hash of the `check` function itself.
2. Enabling rule: `prove.reflect` may use a reflector only if its sub-entry is `checked` and three hashes agree:
   the function named in the soundness theorem's statement, the function the instance file evaluates, and the
   function whose definition hash is recorded. The instance capsule's manifest lists the constants it depends on;
   the check is on that list, so an instance evaluated with a look-alike function (same name in another
   namespace, or an edited definition) is refused.
3. Hygiene inherited from every capsule: no `sorry`, no axiom outside the whitelist, no `native_decide`
   (INV-LEMMA-06); the statement binding of N12-24, so that the theorem proved is the registered statement and
   not a weakened one (for example soundness only for 1×1 systems, or only when y ≥ 0).
4. What an instance's evidence carries: class `Proved`; receipts = the soundness capsule hash, the instance
   capsule hash, both checker identities; scope = the specification's text instantiated at the instance. The
   trusted base printed with it: the Lean kernel(s) and the two capsules; nothing else of the producer.
5. Soundness is one direction only: a reflector may return `false` on inputs that satisfy the specification (it
   is not required to be complete), and a `false` leaves the instance at its previous class.
6. Class and limits: `Proved` for each soundness theorem once sealed, and for instances within the reflection
   budget. Ceiling: reflection relabels a few instance certificates from `Certified` to `Proved`; by itself it
   changes no design decision and no speed.
7. Artifacts: `docs/math/cards/T-35.md`; the registry row; the enabling check in `frob-lemma`.

## Acceptance criteria

- [ ] After the pilot, the sub-entries for `farkas` and `ldlt-pos` are `checked` with capsule hashes and both
      checker identities, or the whole card is `parked` by KILL-REFLECT.
- [ ] `prove.reflect` refuses, each as a negative control: a reflector whose soundness capsule is missing; one
      whose capsule's statement hash differs from the registered `Statement` (a weakened theorem); an instance
      file that evaluates a function other than the one in the soundness theorem; a capsule containing
      `native_decide`.
- [ ] An instance on which the reflector returns `false` produces no `Proved` evidence and no error: the
      certificate stays `Certified`.
- [ ] Every artifact carrying `Proved` obtained by reflection lists both capsule hashes (INV-LEMMA-01).
- [ ] When N12-28 lands, the sub-entries for `poly-eq` and `replay` follow the same rule.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t35.rs`: `reflector_requires_sealed_soundness`;
`weakened_soundness_statement_refused`; `lookalike_function_refused` (an instance evaluating
`Frob.Other.farkas` is rejected by the constant-list check); `native_decide_capsule_refused`;
`false_result_keeps_certified`; `proved_by_reflection_has_two_receipts`; `parked_card_disables_verb`.
`lean/tests/reflect/`: `soundness_not_completeness` (a valid Farkas system whose supplied y is not a witness
evaluates to `false`, which is allowed).

### End-to-end script and logging

`scripts/e2e/lemma_reflect.sh` (bead N12-38) asserts the enabling rule on the two pilot reflectors and logs the
three agreeing hashes per reflector and the refusals of the negative controls.

### Known answers and negative controls

KA-227 (each `check x` evaluates to `true` in the kernel, with no tactic at checking time and no compiled
evaluator). Negative controls as listed. Determinism: capsule and definition hashes reproducible on the pins.

## Considerations

- KILL-REFLECT (GATE-LEMMA-05 fails after one repair cycle, or no reflector's closure avoids the blocked
  modules): fallback, instance certificates stay `Certified` and reflection is parked; this card is then
  `parked`.
- OPEN-LEMMA-2 (closures of the soundness theorems) and OPEN-LEMMA-3 (kernel evaluation cost) are answered by
  N12-26 and N12-27.
- A change to a `check` function invalidates its capsule and every instance proved with it; the definition hash
  makes that visible.

## References

§12.2, §12.3, §12.4, §12.6; ADR-LEMMA-05; INV-LEMMA-06; GATE-LEMMA-05; KILL-REFLECT; KA-227.
