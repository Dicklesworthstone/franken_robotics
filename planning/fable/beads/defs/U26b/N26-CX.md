---
key: "N26-CX"
title: "Nemesis counterexample registry: CX-288 to CX-295 with the shortcut, the smallest breaking case, rule and mutant"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "nemesis", "frob-nemesis", "frob-conformance", "solid", "registry", "test"]
parent: "WP-55"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-TEST"]
needs: []
related: ["N26-KA", "N26-REF", "N26a-01", "N26a-13", "N26a-14", "N26a-15", "N26a-16", "N26a-17", "N26a-18", "N26a-21", "N26a-23", "N26a-24", "N26a-30", "N26a-31", "N26a-33", "N26a-34", "N26b-01", "N26b-02", "N26b-03", "N26b-04", "N26b-05", "N26b-06", "N26b-07", "N26b-09", "N26b-10", "N26b-11", "N26b-12", "N26b-14", "N26b-15", "N26b-17", "N26b-18"]
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

In: the eight numbered entries in the suite's one counterexample catalogue; the plant of each and the plan of
where it is caught; the cross-referenced construction entries; the six late entries under their slugs and the
request for their numbers; the registry check.
Out: the tests themselves (the mechanism beads); the catalogue's schema, the seeding of `controls.toml` and
the join of the registries (N38b-05); running a control and recording that it was caught (N38b-04, invoked by
N26b-17 and N26b-07); the allocation ledger and its rules (N38a-40); the known-answer registry (N26-KA); the
refusal registry (N26-REF).

## Technical approach

**The eight entries of the chapter.**

| CX | The tempting shortcut | The smallest case that breaks it | The rule it protects | Planted as; caught by |
|---|---|---|---|---|
| CX-288 | A world reduced by delta debugging is the smallest failing world | The world fails when {a, b} or {c, d, e} is present. With edits listed (a, c, d, e, b), the halves {a, c} and {d, e, b} pass, the singletons and {e, b} pass, the complement {c, d, e, b} fails, and the result is {c, d, e}: 1-minimal with three edits, while {a, b} exists | 1-minimal is the only claim; the one- and two-edit subset check (78 tests at most for 12 edits) labels a world minimum among pairs | A shrinker whose label says "minimum" or that skips the pair check; the order-dependence trace test of the delta-debugging bead and the shrink script's two-cause trials (N26a-18, BD-274) |
| CX-289 | Any world that defeats the design is a counterexample | A doorway narrower than the body defeats every controller; an oracle that checks a point robot calls it solvable | ADR-NEMESIS-02, INV-NEMESIS-01: only worlds with a `solvable-causal` or `plant-feasible` witness count | Build feature `nemesis-oracle-point-robot`, and `nemesis-mut-count-without-witness`; the oracle's doorway tests (body 0.75 m, door 0.70 m), the oracle script, and the crate-wide test of INV-NEMESIS-01 (BD-271, N26b-15) |
| CX-290 | A good sampler discounts the price of proving absence | At δ = 0.05 a bound of 10⁻⁴ needs 29,956 nominal failure-free episodes, 10⁻⁵ needs 299,572 and 10⁻⁶ needs 2,995,731, whatever the explorer does; inside the qualification lane the tenth request needs 76,959 for 10⁻⁴ | Two lanes: exploration finds, nominal or bounded-weight sampling certifies | Build feature `nemesis-lane-open` (exploration episodes admitted to a bound) and a count that undercounts the exact bound (29,955 where 29,956 is required: KA-664's control, CX-400; the count rounded down from the approximation, 29,957, errs on the safe side and cannot fail the row); the lane firewall's tests and the bound script (N26a-23, BD-276, N26a-30) |
| CX-291 | Importance weights may be unbounded if the proposal is good | A mixture without its nominal share on a mis-centred proposal misses the Gaussian tail P(z > 4) = 3.167 × 10⁻⁵; with q = ε·p + (1 − ε)·p_shifted every weight is at most 1/ε = 10 | Defensive mixture with bounded weights | A proposal with ε = 0 centred at −4 instead of +4; the mixture's coverage tests and the bound script's mis-centred row (N26a-24, BD-276) |
| CX-292 | Forks of a deterministic simulator give independent children | A walk with no noise after the fork returns identical children; the splitting estimate is refused | Fresh randomness after a fork (child streams keyed by child world addresses); FROB-REF-569 | Build feature `nemesis-fork-same-key`; the splitting bead's duplicate-children test on the random walk (truth 0.5/(1.5²⁰ − 1) = 1.504 × 10⁻⁴) (BD-277, N26a-31) |
| CX-293 | A comparison among survivors validates the screen | An e-value between the eighth and ninth finalists is silent about a design the screen discarded at the first rung; no top-k design among 30 audited discards only bounds the false-discard rate at 9.5% | Audit of discarded strata and planted winners; label `screen-biased` | Build feature `nemesis-audit-mut-survivors-only`; the audit fixtures and script (N26b-06, N26b-07), and the bound's verdict test that 30 clean audits are not `passed` at a 5% tolerance (N26b-02) |
| CX-294 | A battery made of past failures is a good battery | It overfits them; the hard-world set is refused as the only content of a battery, as a confirmation sample (FROB-REF-567) and as nominal draws for a bound (FROB-REF-560) | The hard-world set's three refused uses; one slot in five of a covering battery is drawn from held-out scenes | An assembly that back-fills held-out slots with greedy picks, and a pool that admits incidence rows (N26b-11); a hard-set-only battery definition (the hard-world-set bead, BD-275) |
| CX-295 | A failure found under one contact law is a failure of the design | Planted box-cone traction and energy-pumping penetration recovery vanish under a law variant and are typed `artifact-candidate`; a genuine planted failure survives every variant | The artifact guard (INV-NEMESIS-05) | Build feature `nemesis-guard-off`; the guard's planted-artifact tests (a box-linearized cone grants √2 times the friction coefficient along its diagonals: a slope of tan θ = 0.6 held at μ = 0.5 because 0.707 > 0.6) and the guard script (BD-272, N26a-15, N26a-16) |

**Entries of the constructions that the chapter's beads cite (owned by the constructions' registry).**

| CX | Shortcut | Case | Cited by |
|---|---|---|---|
| CX-065 | A defensive mixture proposal makes certifying the absence of failures cheaper | With q = β·p + (1 − β)·q′ the weights are at most 1/β, and zero observed failures certify only p ≤ ln(1/δ)/(β·n); at δ = 0.05 and p* = 10⁻⁴ crude sampling needs about 3 × 10⁴ nominal episodes and the mixture needs that many on its nominal share | The derivation of PO-NEMESIS-02 (N26b-16); the bound beads |
| CX-066 | Forking a world at a splitting level multiplies the evidence | A fork without fresh randomness is an identical copy: children that share the parent's streams reproduce the parent's trajectory bit for bit and add nothing | The splitting bead; CX-292 is its chapter-level form |
| CX-067 | The shrunk failing world is the smallest one, and its size measures distance to failure | Edit 1 (a grease patch) fails the design; edit 2 (a rug over the patch) masks it: {1} fails and {1, 2} passes, so failure is not monotone in the edit set. With eight edits and failure exactly when 3 and 6 are present the search returns {3, 6}, 1-minimal by construction and nothing more | The shrink beads (BD-274); CX-288 is its chapter-level form |

**Chapter-local entries without a catalogue number (late entries, cited by tag until the allocator issues their
numbers).** Each is a shortcut with its smallest breaking case, so each is a catalogue entry; its tag is the
entry's `slug`. An earlier form of this bead printed the six as NC-26-01 to NC-26-06, in this order: that
spelling is withdrawn, because `NC-` followed by digits is the identifier grammar of the control registry, whose
numbers only the seeding tool assigns.

| Tag (`slug`) | Shortcut | Case | Rule | Planted as; caught by |
|---|---|---|---|---|
| `nemesis-late-01` | The bound on a false-discard rate may use ln(1/δ)/m | At m = 30 and δ = 0.05 the approximation gives 10.0% where the exact 1 − δ^(1/m) gives 9.5% | The exact binomial bound is used; the approximation is not | `nemesis-audit-mut-approximation`; the audit's known-answer row (N26b-02, N26b-07) |
| `nemesis-late-02` | The audited miss rate is the bound | With 19 audits of a stratum of 100 holding 20 false discards, the audited rate k/m covers the planted 20% in 56% of draws; the exact bound covers it in 99.1% | Report the rate and the bound; decide on both | `nemesis-audit-mut-point-estimate`; the coverage row of the audit gate (N26b-06, N26b-07) |
| `nemesis-late-03` | The audit lane may be chosen after the scores are known | A lane that prefers discards far from the cut under-covers when the false discards sit near the cut | A keyed draw committed before any score exists | `nemesis-audit-mut-post-score-draw`; the commitment test (N26b-01) |
| `nemesis-late-04` | Coverage of a set of scenes is the sum of the scenes' values | The six greedy scenes of the worked instance have single-scene values 31, 30, 26, 19, 14, 12, which sum to 132; they cover 86 of a total weight of 100 | f(S) counts a mode once | `nemesis-cover-mut-double-count`; the worked-instance test (N26b-09, N26b-10) |
| `nemesis-late-05` | The certificate may reuse the lazy queue's cached gains, or stop at f(S) | Cached gains give 99 where fresh gains give 98; with no remaining-gain term the "bound" 86 is below the true optimum 90 | The certificate is recomputed against the final selection and checked | `nemesis-cover-mut-stale-gains`, `nemesis-cover-mut-no-remaining-term`; the certificate tests (N26b-10) |
| `nemesis-late-06` | Greedy selection is optimal | On the worked instance greedy covers 86 and the best six scenes cover 90 | Report the certificate, never "optimal" | The oracle test (N26b-10); a result field named "optimal" fails the wording lint |

**Registry form.** The fourteen entries (eight numbered, six late) are records of the suite's one
counterexample catalogue, `crates/frob-conformance/registry/counterexamples.toml`, in the record of the
catalogue bead N38b-05 and with no field of the chapter's own: `id` (CX-288 to CX-295; a late entry carries
its tag in `slug` and takes its `id` from the block requested below), `shortcut`, `case`, `numeric = true` and
`plan_case = "numbers"` (every entry has a numeric case), `rule` (the text of the rule column), `rule_ids`,
`defined_in` (the §26 subsection), `same_as` (empty: CX-288, CX-290 and CX-292 restate CX-067, CX-065 and
CX-066 at chapter level with cases of their own, so each keeps its own control), `label = "DESIGN"`,
`control_kind` (by N38b-05's table: `mutation` where the last column names a build feature, `planted-input`
where it names a request or a definition that must be refused) and `plant` (the chapter's identifier of the
planted defect: the feature name or the planted input of the last column). Where the last column names more
than one plant, the first named is the entry's `plant`; the others are further controls of the same rule,
registered against the same gates by the beads that own them (through the known-answer rows that state them, or
with the `GATE-…` seed form of BD-025). DESIGN: N38b-05's `plant` admits a list, but each further plant is
already registered by the bead that owns it, so naming it here as well would seed a second control for one
defect; the entry keeps the first plant only.
For `nemesis-late-06` the plant is the planted result that carries a field named `optimal`.
`rule_ids` per entry hold the identifiers its rule names and, DESIGN, the gate whose negative controls in the
chapter's gate bead (N26b-18) include its plant, so that gate resolution does not rest on a section lookup:
CX-288 PO-NEMESIS-01, GATE-NEMESIS-03; CX-289 ADR-NEMESIS-02, INV-NEMESIS-01, GATE-NEMESIS-01; CX-290
ADR-NEMESIS-05, INV-NEMESIS-03, GATE-NEMESIS-04; CX-291 PO-NEMESIS-02, GATE-NEMESIS-04; CX-292 FROB-REF-569,
C-27, GATE-NEMESIS-05; CX-293 ADR-NEMESIS-07, GATE-NEMESIS-07; CX-295 INV-NEMESIS-05, GATE-NEMESIS-06;
`nemesis-late-01` to `-03` ADR-NEMESIS-07, GATE-NEMESIS-07; `nemesis-late-04` to `-06` PO-NEMESIS-03,
GATE-NEMESIS-08. CX-294 names INV-NEMESIS-06, FROB-REF-567, FROB-REF-560 and §26.8 and no gate: no Nemesis
gate measures the three refused uses of the hard-world set, so its control is attached to the gates that
exercise those two refusals as the seeding tool resolves them, and if it resolves to none the tool lists it as
a finding whose disposition (a gate added, or the entry carried as a registered control of the hard-world-set
bead's tests) is given at the M3 exit.

This bead writes catalogue entries and never a row of `controls.toml` (the division N38b-05 fixes for every
chapter; an earlier form of this bead put its rows in a section of `controls.toml` with fields `mutant`,
`caught_by`, `due` and `first_caught`, which that file's one schema, BD-025's, does not have). The seeding tool
of N38b-05 turns each entry into one control (`NC-nnnn`, the record of BD-025) attached to the gates its
`rule_ids` resolve to. The milestone at whose exit a control must have been caught is that of its gates: M4 for
CX-293 and `nemesis-late-01` to `-03` (GATE-NEMESIS-07, whose owners are the audit beads), M3 for the other
ten.

**Numbers for the late entries.** This bead files one request in the allocation ledger of N38a-40
(`crates/frob-conformance/registry/allocations.toml`: `registry = "cx"`, `count = 6`, `owner = "N26-CX"`,
`section = "§26.10, §26.11, §26.14"`). When the block is issued the tags are mapped to its numbers in table
order (`nemesis-late-01` to the first number) in one commit, the tests' names follow, and a tag still used in
place of a number at this bead's milestone exit fails the allocator's `provisional-tag-past-due` check. This
bead prints no number above CX-384 before then.

**Direction (DESIGN, as for the known-answer registry N26-KA).** This registry depends on no mechanism: the
mechanism beads that own the catching tests come later and depend on it. The last column of each table is the
plan of where the plant is caught. The binding itself is the test kit's and needs no field: a test that shows
an entry's shortcut caught is named `cx_<nnn>_…` (STD-TEST) once the entry has its number, and the runner view
of N26b-17 names the entries it runs, by number or by slug. The check `caught_by_exist` (provided here) reports
per entry `resolved` (such a test or runner entry exists) or `planned`, and lists the planned ones; the status
is computed on each run and is not stored in the catalogue. The demonstration that each plant is caught is the
control runner's (N38b-04): it builds with the plant, re-runs the entries of the control's gates, and writes
the control's `last_seen` and one line of `registry/control_history.ndjson`. It is invoked by the known-answer
and control runner N26b-17 for the ten entries due at M3 and by the audit script's self-test N26b-07 for the
four due at M4. What earlier forms of this bead called `first_caught` is the first `caught` line of that
history; no bead writes such a field by hand. At a milestone exit an entry due there that is still `planned`
fails the suite's lint (a registry row due at the current milestone with no bound test and no runner entry),
and one whose control has no `caught` line in the history is a suspect control of N38b-04's review, which
fails GATE-GAUNT-01 until it is disposed of.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/counterexamples.toml` holds the eight entries CX-288 to CX-295 with the
      shortcut, case and rule as above and every field of N38b-05's record filled (a `plant`, a `control_kind`
      and `rule_ids` among them); N38b-05's loader accepts the file and counts 8 for the Nemesis chapter; this
      bead has written no row of `controls.toml`.
- [ ] `caught_by_exist` reports an entry `resolved` when a test named `cx_<nnn>_…` or a runner entry for it
      exists and lists the others as `planned`; it stores nothing. The observed-to-fail demonstration is the
      control runner's, invoked by N26b-17 (entries due at M3) and N26b-07 (CX-293 and `nemesis-late-01` to
      `-03`, due at M4), which depend on this bead: each such control then has `last_seen.caught = true` and a
      `caught` line in `control_history.ndjson`.
- [ ] The three construction entries are cross-referenced, not duplicated: this bead writes no second record
      for CX-065, CX-066 or CX-067, and its tests resolve them in the constructions' entries.
- [ ] The six late entries are present under their slugs `nemesis-late-01` to `nemesis-late-06`, each with a
      plant and `rule_ids`; the request for six `cx` numbers is in the allocation ledger, and when the block is
      issued the tags are replaced in one commit (no tag survives this bead's milestone exit:
      `provisional-tag-past-due`). No registry, test or derivation file prints the withdrawn `NC-26-` names.
- [ ] The registry check passes, and when a bound test of a `resolved` entry is renamed the entry is reported
      `planned` again and is listed; a plant that stops being caught shows as a control without a `caught`
      line at the next milestone exit (N38b-04's suspect report).

## Test plan

### Unit tests
`crates/frob-conformance/tests/nemesis_cx_registry.rs`: `eight_entries_present`; `schema_is_n38b05` (every
entry parses with the catalogue loader of N38b-05; a scratch entry carrying `mutant`, `caught_by` or
`first_caught` is rejected as an unknown field); `every_entry_has_plant_and_rule_ids`;
`plants_declared_as_features` (on a fixture workspace a `plant` naming a declared feature is found and one
naming none is listed; on the real workspace the result is reported, not asserted, because the features are
declared by the mechanism beads, which come later, and BD-025's loader checks them once a gate that lists the
control leaves `not-run`);
`late_rows_slugged` (six entries with slugs `nemesis-late-01` to `-06`, each unique in the shared file; a
scratch entry tagged `late-01`, or named `NC-26-01`, fails); `late_block_requested` (the allocation ledger
holds `registry = "cx"`, `owner = "N26-CX"`, `count = 6`);
`cross_references_resolve` (CX-065, -066, -067 exist in the constructions' entries);
`numbers_append_only` (the file's numbers are a superset of the previous committed version's);
`due_follows_gates` (on a scratch seeding, the controls of CX-293 and `nemesis-late-01` to `-03` resolve to
GATE-NEMESIS-07, due at M4; the nine other entries that name a gate resolve to that gate, due at M3; CX-294,
which names none, resolves through its two refusal codes or is listed in the seeding findings, never dropped);
`no_row_in_controls_toml_by_hand` (the only writer of the chapter's controls is the seeding tool: the seed map
of N38b-05 accounts for every Nemesis control whose seed is a CX entry);
`caught_by_exist` (on a fixture workspace an entry with a test named `cx_<nnn>_…` is reported `resolved`, one
with none is reported `planned` and is listed; on the real workspace the result is reported, not asserted);
`exit_needs_caught_line` (on a fixture registry and history at an M3 exit, a control due at M3 with no
`caught` line is listed as suspect; a control due at M4 is not).
The mutant builds themselves are run against their bound tests by N26b-15 (`nemesis_mutants.rs`, shared with
the invariant ledger) and N26b-17, which depend on this bead and on the mechanisms.

### End-to-end script and logging
No script of its own. The known-answer runner (`scripts/e2e/nemesis_known_answers.sh`, N26b-17) runs every
control due at M3 and logs the counterexample number (or slug) beside the control it embodies, with the
control's identifier, its outcome and the history line the control runner wrote; the audit script's self-test
(N26b-07) does the same for the four entries due at M4; the per-capability scripts' self-tests run the other
mutant builds. Assertions on these entries carry the helper's `--cx CX-nnn` tag, so a log search for a CX
number finds the run in which its plant was caught.

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
