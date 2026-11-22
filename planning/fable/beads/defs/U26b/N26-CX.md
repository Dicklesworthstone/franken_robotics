---
key: "N26-CX"
title: "Nemesis counterexample registry: CX-288 to CX-295 with the shortcut, the smallest breaking case, rule and mutant"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "frob-conformance", "solid", "registry", "test"]
parent: "WP-55"
deps: ["STD-TEST", "BD-025"]
needs: ["§26.3 to §26.9: the tests that catch the mutants of CX-288 to CX-292 and CX-295 in the oracle, guard, shrinker, bound and splitting beads (beads N26a-13, N26a-14, N26a-18, N26a-21, N26a-23, N26a-24, seeds BD-272 and BD-277, drafted by unit U26a; seed BD-275, drafted by unit U15a, for CX-294)", "Appendix N: registry numbers for the six chapter-local controls listed below, which have no counterexample number"]
related: ["N26-KA", "N26-REF", "N26b-15", "N26b-17", "N26b-02", "N26b-04", "N26b-06", "N26b-10", "N26b-11", "BD-271", "BD-274", "BD-278", "BD-275", "BD-318", "BD-146"]
gates: []
plan_refs: ["Appendix N N.23", "§26.5", "§26.6", "§26.7", "§26.8", "§26.9", "§26.10", "§26.11", "§26.14", "§38.11", "§10 C-27", "§10 C-28"]
---
## Background

A counterexample, in this program's sense, is the smallest concrete case that breaks a tempting shortcut: a rule
of thumb an implementer or an agent would reach for because it is cheaper and usually right. The catalogue of
counterexamples exists so that each shortcut is refuted once, with numbers, and stays refuted: every entry seeds
the **negative-control registry**, where it becomes a planted defect (a mutant build, or a planted input) that a
named test must catch. The rule that makes this binding: a gate that nothing can fail validates nothing, and
every convergence gate needs a registered negative control it has been seen to catch. Numbers are append-only:
an entry is never renumbered and a number is never reused.

The chapter on adversarial worlds and rare failures contributes eight entries, CX-288 to CX-295. This bead
enters them in the registry with the shortcut, the case with its numbers, the rule it protects, the mutant that
embodies the shortcut and the bead whose test catches it. It also names three entries of the two constructions
behind the chapter that the chapter's beads cite, and six controls the chapter states in its known-answer and
decision tables that have no catalogue number yet.

## Scope

In: the eight registry rows; their bindings to mutants and tests; the cross-referenced construction entries; the
provisional rows; the registry check.
Out: the tests themselves (the mechanism beads); the known-answer registry (N26-KA); the refusal registry
(N26-REF).

## Technical approach

**The eight entries of the chapter.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Planted as; caught by |
|---|---|---|---|---|
| CX-288 | A world reduced by delta debugging is the smallest failing world | The world fails when {a, b} or {c, d, e} is present. With edits listed (a, c, d, e, b), the halves {a, c} and {d, e, b} pass, the singletons and {e, b} pass, the complement {c, d, e, b} fails, and the result is {c, d, e}: 1-minimal with three edits, while {a, b} exists | 1-minimal is the only claim; the one- and two-edit subset check (78 tests at most for 12 edits) labels a world minimum among pairs | A shrinker whose label says "minimum" or that skips the pair check; the order-dependence trace test of the delta-debugging bead and the shrink script's two-cause trials |
| CX-289 | Any world that defeats the design is a counterexample | A doorway narrower than the body defeats every controller; an oracle that checks a point robot calls it solvable | ADR-NEMESIS-02, INV-NEMESIS-01: only worlds with a `solvable-causal` or `plant-feasible` witness count | Build feature `nemesis-oracle-point-robot`, and `nemesis-mut-count-without-witness`; the oracle's doorway tests (body 0.75 m, door 0.70 m), the oracle script, and the crate-wide test of INV-NEMESIS-01 (BD-271, N26b-15) |
| CX-290 | A good sampler discounts the price of proving absence | At δ = 0.05 a bound of 10⁻⁴ needs 29,956 nominal failure-free episodes, 10⁻⁵ needs 299,572 and 10⁻⁶ needs 2,995,731, whatever the explorer does; inside the qualification lane the tenth request needs 76,959 for 10⁻⁴ | Two lanes: exploration finds, nominal or bounded-weight sampling certifies | Build feature `nemesis-lane-open` (exploration episodes admitted to a bound) and a count taken from the approximation rounded down (29,957); the lane firewall's tests and the bound script |
| CX-291 | Importance weights may be unbounded if the proposal is good | A mixture without its nominal share on a mis-centred proposal misses the Gaussian tail P(z > 4) = 3.167 × 10⁻⁵; with q = ε·p + (1 − ε)·p_shifted every weight is at most 1/ε = 10 | Defensive mixture with bounded weights | A proposal with ε = 0 centred at −4 instead of +4; the mixture's coverage tests and the bound script's mis-centred row |
| CX-292 | Forks of a deterministic simulator give independent children | A walk with no noise after the fork returns identical children; the splitting estimate is refused | Fresh randomness after a fork (child streams keyed by child world addresses); FROB-REF-569 | Build feature `nemesis-fork-same-key`; the splitting bead's duplicate-children test on the random walk (truth 0.5/(1.5²⁰ − 1) = 1.504 × 10⁻⁴) |
| CX-293 | A comparison among survivors validates the screen | An e-value between the eighth and ninth finalists is silent about a design the screen discarded at the first rung; no top-k design among 30 audited discards only bounds the false-discard rate at 9.5% | Audit of discarded strata and planted winners; label `screen-biased` | Build feature `nemesis-audit-mut-survivors-only`; the audit fixtures and script (N26b-06, N26b-07), and the bound's verdict test that 30 clean audits are not `passed` at a 5% tolerance (N26b-02) |
| CX-294 | A battery made of past failures is a good battery | It overfits them; the hard-world set is refused as the only content of a battery, as a confirmation sample (FROB-REF-567) and as nominal draws for a bound (FROB-REF-560) | The hard-world set's three refused uses; one slot in five of a covering battery is drawn from held-out scenes | An assembly that back-fills held-out slots with greedy picks, and a pool that admits incidence rows (N26b-11); a hard-set-only battery definition (the hard-world-set bead, BD-275) |
| CX-295 | A failure found under one contact law is a failure of the design | Planted box-cone traction and energy-pumping penetration recovery vanish under a law variant and are typed `artifact-candidate`; a genuine planted failure survives every variant | The artifact guard (INV-NEMESIS-05) | Build feature `nemesis-guard-off`; the guard's planted-artifact tests (a box-linearized cone grants √2 times the friction coefficient along its diagonals: a slope of tan θ = 0.6 held at μ = 0.5 because 0.707 > 0.6) and the guard script |

**Entries of the constructions that the chapter's beads cite (owned by the constructions' registry).**

| CX | Shortcut | Case | Cited by |
|---|---|---|---|
| CX-065 | A defensive mixture proposal makes certifying the absence of failures cheaper | With q = β·p + (1 − β)·q′ the weights are at most 1/β, and zero observed failures certify only p ≤ ln(1/δ)/(β·n); at δ = 0.05 and p* = 10⁻⁴ crude sampling needs about 3 × 10⁴ nominal episodes and the mixture needs that many on its nominal share | The derivation of PO-NEMESIS-02 (N26b-16); the bound beads |
| CX-066 | Forking a world at a splitting level multiplies the evidence | A fork without fresh randomness is an identical copy: children that share the parent's streams reproduce the parent's trajectory bit for bit and add nothing | The splitting bead; CX-292 is its chapter-level form |
| CX-067 | The shrunk failing world is the smallest one, and its size measures distance to failure | Edit 1 (a grease patch) fails the design; edit 2 (a rug over the patch) masks it: {1} fails and {1, 2} passes, so failure is not monotone in the edit set. With eight edits and failure exactly when 3 and 6 are present the search returns {3, 6}, 1-minimal by construction and nothing more | The shrink beads (BD-274); CX-288 is its chapter-level form |

**Chapter-local controls without a catalogue number (provisional rows; a number is the registry owner's to
assign).**

| Provisional | Shortcut | Case | Rule | Planted as; caught by |
|---|---|---|---|---|
| NC-26-01 | The bound on a false-discard rate may use ln(1/δ)/m | At m = 30 and δ = 0.05 the approximation gives 10.0% where the exact 1 − δ^(1/m) gives 9.5% | The exact binomial bound is used; the approximation is not | `nemesis-audit-mut-approximation`; the audit's known-answer row (N26b-02, N26b-07) |
| NC-26-02 | The audited miss rate is the bound | With 19 audits of a stratum of 100 holding 20 false discards, the audited rate k/m covers the planted 20% in 56% of draws; the exact bound covers it in 99.1% | Report the rate and the bound; decide on both | `nemesis-audit-mut-point-estimate`; the coverage row of the audit gate (N26b-06, N26b-07) |
| NC-26-03 | The audit lane may be chosen after the scores are known | A lane that prefers discards far from the cut under-covers when the false discards sit near the cut | A keyed draw committed before any score exists | `nemesis-audit-mut-post-score-draw`; the commitment test (N26b-01) |
| NC-26-04 | Coverage of a set of scenes is the sum of the scenes' values | The six greedy scenes of the worked instance have single-scene values 31, 30, 26, 19, 14, 12, which sum to 132; they cover 86 of a total weight of 100 | f(S) counts a mode once | `nemesis-cover-mut-double-count`; the worked-instance test (N26b-09, N26b-10) |
| NC-26-05 | The certificate may reuse the lazy queue's cached gains, or stop at f(S) | Cached gains give 99 where fresh gains give 98; with no remaining-gain term the "bound" 86 is below the true optimum 90 | The certificate is recomputed against the final selection and checked | `nemesis-cover-mut-stale-gains`, `nemesis-cover-mut-no-remaining-term`; the certificate tests (N26b-10) |
| NC-26-06 | Greedy selection is optimal | On the worked instance greedy covers 86 and the best six scenes cover 90 | Report the certificate, never "optimal" | The oracle test (N26b-10); a result field named "optimal" fails the wording lint |

**Registry form.** The negative-control registry of the conformance crate
(`crates/frob-conformance/registry/controls.toml`, of which the counterexample catalogue is the seed), in a
section `nemesis`: one entry per row with `cx`
(or the provisional name), `shortcut`, `case`, `rule`, `defined_in`, `mutant` (feature name or planted input),
`caught_by` (crate test names and end-to-end steps), `first_caught` (commit, filled when the mutant is first
observed to fail its test). The registry check fails for an entry with no `caught_by`, for a `caught_by` that
does not exist, and at each milestone exit for an entry due there whose `first_caught` is empty.

## Acceptance criteria

- [ ] The registry file holds the eight entries CX-288 to CX-295 with the shortcut, case and rule as above, each
      with a mutant and at least one `caught_by` test that exists.
- [ ] Each of the eight mutants (or planted inputs) has been observed to fail its test, with the commit recorded;
      CX-293's is due with the audit at M4, the other seven at M3.
- [ ] The three construction entries are cross-referenced, not duplicated: the file names their numbers and the
      beads that cite them.
- [ ] The six provisional rows are present, marked provisional, and bound to mutants and tests; a request for
      numbers is on record with the registry owner.
- [ ] The registry check passes, and fails when a `caught_by` test is renamed or a mutant stops failing.

## Test plan

### Unit tests
`crates/frob-conformance/tests/nemesis_cx_registry.rs`: `eight_entries_present`; `every_entry_bound`;
`mutants_exist_as_features` (each named feature is declared by the crate it mutates);
`provisional_rows_marked`; `cross_references_resolve` (CX-065, -066, -067 exist in the constructions' file);
`numbers_append_only` (the file's numbers are a superset of the previous committed version's).
`crates/frob-conformance/tests/nemesis_mutants.rs` (shared with the invariant ledger): each registered mutant
build fails the test bound to it.

### End-to-end script and logging
No script of its own. The known-answer runner (`scripts/e2e/nemesis_known_answers.sh`, N26b-17) runs every
control and logs the counterexample number beside the control it embodies; the per-capability scripts' self-tests
run the mutant builds. A log search for a CX number finds the run in which its mutant was caught.

### Known answers and negative controls
Every row of this registry is a negative control. The numbers they rely on: 78 = 12 + 66; 29,956, 299,572,
2,995,731 and 76,959; 3.167 × 10⁻⁵ and 1/ε = 10; 1.504 × 10⁻⁴; 9.5% and 10.0%; 132, 86, 98, 99 and 90;
√2 × 0.5 = 0.707 against tan θ = 0.6. The registry's own negative control: an entry whose mutant is replaced by
the correct build must be reported as "control not caught".

## Considerations

- The catalogue marks no chapter entry as a duplicate of an earlier one, but three pairs are the same idea at two
  levels (CX-288 and CX-067; CX-292 and CX-066; CX-290 and CX-065). They are kept as separate numbers because the
  catalogue is append-only; the mutants are shared.
- A counterexample is a statement about a shortcut, not about a person; the "planted as" column is what turns it
  from a paragraph into a test.
- If a kill gate removes a mechanism (splitting, the search methods, subset shrinking), its entries stay in the
  catalogue and their registry rows are marked "mechanism removed" by a recorded change; the numbers are not
  reused.

## References

Plan Appendix N (N.23: CX-288 to CX-295; the constructions' entries CX-065 to CX-067), §26.5 to §26.11, §26.14,
§38.11 (negative controls), §10 C-27 and C-28. KA-659 to KA-669; GATE-GAUNT-01 (every gate has a registered
negative control that it catches).
