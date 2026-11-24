---
key: "CARD-T-35"
title: "Theorem card T-35: reflection soundness (a reflected checker returns true only on inputs meeting its spec)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "lemma", "frob-lemma", "frontier", "test"]
parent: "WP-72"
deps: ["CARD-T-21", "N12-06", "N12-24", "N12-26"]
needs: []
related: ["BD-382", "N12-27", "N12-28", "N12-32", "N12-38"]
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

In: the card's statement and registry row with one sub-entry per reflector; the registered specifications and
the soundness template (`lean/Frob/Statements/T35.lean`), in the elementary forms that the closure experiment
N12-26 found clean; the rule that a reflector is enabled only while its soundness capsule is sealed, bound and
current; the link between the function evaluated and the function proved about; the law tests of that rule and
the negative controls. Out: the reflectors, their proofs and the code of `prove.reflect` that applies the
enabling rule (BD-382 for the first two; N12-28 for the other two); the registered Farkas statement (CARD-T-21,
a dependency); budgets (N12-27); the kill-gate decision (N12-32).

Order of work. This card is finished before the code that implements or uses its theorem and never depends on it:
the reflection pilot BD-382 (which depends on this card) and N12-28 are `related`, not dependencies (a card that
waited for its implementer would close a loop, because the implementer's own acceptance runs the card's law
tests). What the card does depend on is in `deps`: the card registry (N12-06), the statement binding (N12-24),
the Farkas statement of CARD-T-21, and the closure experiment N12-26, the first step of the reflection pilot,
whose answer fixes the form of each specification. It closes on its statement document, its registered
specifications and template, its registry row (every sub-entry at `hypothesis`), the enabling rule as registered
data, and its law tests and negative-control fixtures. The acceptance lines that can only be observed on live
output (sealed soundness capsules, `prove.reflect` refusing the controls, an instance proved by kernel
evaluation) are run inside BD-382 (and N12-28 for its two reflectors), with this card's law tests, as part of that
bead's acceptance; the receipts produced there are what move each sub-entry from `hypothesis` to `checked`.

## Technical approach

1. Registry row with one sub-entry per reflector: name, registered `Statement` (by its definition hash), capsule
   hash, checker identities (stock Lean build, `fln` build), status (`hypothesis`, `checked`, `parked`), and the
   definition hash of the `check` function itself. The registered form (DESIGN), in
   `lean/Frob/Statements/T35.lean`: one specification per reflector in the elementary form N12-26 recorded as
   clean (`PosSpec A := ∀ K [LinearOrderedField K] (x : Fin n → K), x ≠ 0 → 0 < x ⬝ᵥ (A.map Rat.cast).mulVec x`
   for symmetric A, without `Matrix.PosDef`; the Farkas specification is the one of `T21.lean`; the `polyEq` and
   `replay` specifications are added by N12-28 over its input types, in the form N12-26 recorded), and the template
   `SoundFor (check : α → Bool) (Spec : α → Prop) : Prop := ∀ x, check x = true → Spec x`. A reflector's soundness
   `Statement` is `SoundFor <check> <Spec>` instantiated at the function whose definition hash the sub-entry
   records; the statement binding of N12-24 then requires the sealed theorem's type to be exactly that constant,
   so neither the specification nor the direction of the implication can be weakened.
   Names (the binding rule of N12-24): one constant per reflector,
   `Frob.Statements.T35.<check>.Statement : Prop := SoundFor Frob.Reflect.<check> Frob.Statements.T35.<Spec>`, with
   `<check>` one of `farkas`, `ldltPos`, `polyEq`, `replay` (the Lean definition names, so the four sub-entries map
   one to one onto four constants; the CLI values `farkas`, `ldlt-pos`, `poly-eq`, `replay` name the same
   sub-entries), each proved by `theorem T35_<check>_sound : Frob.Statements.T35.<check>.Statement`.
   Placement (DESIGN, because a Lean constant cannot name a function that does not exist yet, and this card closes
   before any reflector is defined): at this card's close `T35.lean` holds the template, `PosSpec`, the re-export
   of `T21.lean`'s specification as `FarkasSpec`, and nothing that names a `check` function, so it elaborates. The
   exact text of each reflector's `Statement` line is registered data of this card (one line per reflector, with
   the hashes of the template and of the specification it names); the line itself is appended to `T35.lean`, with
   the import of `Frob.Reflect.<check>`, by the bead that defines the function (BD-382 for `farkas` and `ldltPos`;
   N12-28 for `polyEq` and `replay`, which also adds their specifications in the form N12-26 recorded), and the law
   test `statement_lines_match_registered_text` requires the appended line to equal the registered text byte for
   byte. The implementer therefore chooses only the function, which is what the statement is about.
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
7. Artifacts: `docs/math/cards/T-35.md`; the registry row with its four sub-entries; `lean/Frob/Statements/T35.lean`;
   the enabling rule as registered data (the three hashes and the forbidden constants it compares); the law tests
   `crates/frob-lemma/tests/cards/t35.rs` and `lean/tests/reflect/`, with their fixtures (a look-alike function in
   another namespace, an edited definition, a weakened statement, a capsule containing `native_decide`), which the
   code of `prove.reflect` in BD-382 must pass.

## Acceptance criteria

At this card's close (M5, before BD-382):
- [ ] `lean/Frob/Statements/T35.lean` elaborates under stock Lean at the pin, its import closure equals the one
      N12-26 recorded for the chosen forms and avoids every blocked module; its definition hashes are in the
      registry row, and the four sub-entries read `hypothesis`.
- [ ] The registered text of the four `Frob.Statements.T35.<check>.Statement` lines is committed with its digest;
      a line differing in one character (a narrowed specification name, the converse template) fails
      `statement_lines_match_registered_text` on a fixture file.
- [ ] A theorem stating the converse (`Spec x → check x = true`), one with the specification restricted to 1×1
      systems or to y ≥ 0, and one about a function of another namespace each fail the binding check of N12-24
      against the registered `SoundFor` statement (fixture capsules; no reflector proof is needed for this).
- [ ] The law tests and their fixtures are committed, and the enabling rule is registered data with a digest.

Observed on live output, inside BD-382 (and N12-28), with this card's law tests:
- [ ] After the pilot, the sub-entries for `farkas` and `ldlt-pos` are `checked` with capsule hashes and both
      checker identities, or the whole card is `parked` by KILL-REFLECT (decision N12-32).
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

`crates/frob-lemma/tests/cards/t35.rs` (written here; the tests that call `prove.reflect` run in BD-382's
acceptance): `statement_module_elaborates_with_recorded_closure`; `converse_statement_fails_binding`;
`restricted_spec_fails_binding` (1×1 only; y ≥ 0 only); `reflector_requires_sealed_soundness`;
`weakened_soundness_statement_refused`; `lookalike_function_refused` (an instance evaluating
`Frob.Other.farkas` is rejected by the constant-list check); `edited_definition_hash_refused` (the instance's
`farkas` has a definition hash other than the recorded one); `native_decide_capsule_refused`;
`false_result_keeps_certified`; `proved_by_reflection_has_two_receipts`; `parked_card_disables_verb`;
`all_sub_entries_hypothesis_at_close`; `statement_lines_match_registered_text` (each appended
`Frob.Statements.T35.<check>.Statement` line equals the registered text; a fixture line with `Spec` replaced by a
narrower constant, and one with the arguments of `SoundFor` swapped, each fail); `theorem_names_follow_binding`
(the sealed theorem for sub-entry `<check>` is `T35_<check>_sound`; a capsule whose theorem is named
`farkas_sound` and restates the proposition fails the binding check of N12-24).
`lean/tests/reflect/`: `soundness_not_completeness` (a valid Farkas system whose supplied y is not a witness
evaluates to `false`, which is allowed; run once `farkas` exists, in BD-382).

### End-to-end script and logging

`scripts/e2e/lemma_reflect.sh` (bead N12-38) asserts the enabling rule on the two pilot reflectors and logs the
three agreeing hashes per reflector and the refusals of the negative controls. For this card its NDJSON lines
(STD-E2E helper) carry `ts`, `script`, `bead` ("CARD-T-35"), `step` (`T-35/<reflector>/<case>`), `event`, the
exact command line, exit code, wall time, per reflector the registered `Statement` hash, the hash of the function
named in the sealed theorem, the hash of the function the instance evaluates and the recorded definition hash
(the last three must agree), the sub-entry status, the soundness and
instance capsule hashes, the class issued, and for each negative control the expected and observed refusal with
its first failing rule and the path to reproduce.

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
