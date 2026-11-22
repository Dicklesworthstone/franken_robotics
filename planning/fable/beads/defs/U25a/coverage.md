# Coverage of drafting unit U25a (the preamble and §25.1 to §25.4 of Augur: mission queries and inverse design)

48 bead files: 13 seed beads (BD-002, BD-127, BD-130, BD-131, BD-135, BD-140, BD-288, BD-292, BD-316, BD-354,
BD-355, BD-387, BD-436) and 35 new beads (N25a-01 to N25a-34, and the registry N25-KA). The chapter's other
registries are written by unit U25e (N25-CX, N25-REF); this unit cites their rows and states the values inline.

The unit was drafted in two sittings. The first wrote the 13 seed beads and N25a-01 to N25a-24. The second
wrote N25a-25 to N25a-34, N25-KA and this file, and made three small additions to existing files where a
subsection item had no bead: the worked illustration and the list of what Augur takes from other subsystems
(N25a-01, Background); the refusal codes 850 and 851 beside the stance mutations that trigger them (BD-130,
test plan).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §25 preamble | Crates, formats, labelling convention | N25a-01 (crates `frob-mission` and `frob-synth`, the payload and checking rules in `frob-cert`, who writes bundles; the DESIGN, FACT and TARGET convention of the chapter, carried into every mission bead) |
| §25.1 | What Augur owns and what it is for | N25a-01 (the mission query, I ⊆ F ⊆ O, the contrast with scoring one design, the carrier illustration with its four choices, the loop from game to checker to next test, the crate table, the tier of each fragment, what Augur takes from other subsystems, feature flags); N25a-32 (owners probed before new code) |
| §25.2 | Decisions (ADR-AUGUR-01 to -12) | Each decision is stated with its reason and rejected alternatives in the bead it governs: -01 N25a-05; -02 N25a-03, N25a-02, BD-131; -03 BD-127, BD-130, BD-131, N25a-18, N25a-30, N25a-32; -04 N25a-06, N25a-01, BD-127, BD-355, N25a-31; -05 N25a-01; -06 N25a-02, N25a-31; -07 N25a-04, N25a-33; -08 BD-135, N25a-05, N25a-24, N25a-33; -09 BD-140, N25a-18; -10 N25a-05; -11 N25a-02; -12 N25a-02, N25a-03 |
| §25.3.1 | Verbs | BD-135 (`mission.synthesize`, `mission.refute`, `mission.distinguish` in its first form, `mission.show`: registry entries, envelope, estimate, jobs); N25a-12 (point and existence queries; the one-verb route from a brief to survivors); N25a-09 (`mission.refute`); N25a-16 (`mission.revalidate`); BD-436 (`mission.transport` and `skill.transport`, the two scopes of transport); BD-140 (the HYPOTHESIS that the one verb saves serial agent turns, measured against a lower-verbs arm); N25a-34 (end-to-end script of `mission.distinguish`). `mission.separate`, the full `mission.distinguish` and `experiment.suggest` are carried by units U25b and U25c |
| §25.3.2 | The `.fmission` schema | N25a-02 (the thirteen field rules, admission order, refusals, `MissionId` sensitivity, fragment classification, the report); N25a-33 (the `resolution` field: measures and the three mandatory obligations); N25a-03 (`quantifiers`, `world.persistence`, `policy_class`); N25a-11 (`prob` binder, `evidence` profile `empirical`); N25a-10 (`objective` and its claims); BD-130 and N25a-21 (the complete stance query); N25a-02 (the carrier fragment, as a refused abridged query). The schema, parser and canonical form are BD-126 (unit U24b) |
| §25.3.3 | The answer | N25a-05 (cell lists, what every cell names, `outer` by definition, class, address, withdrawal log, authorization flags); BD-135 (the envelope with `warnings` and `next_actions`); N25a-10 (`cost_bounds`); N25a-33 (when an answer is sufficient; no mass without a weighting) |
| §25.3.4 | Budgets, estimates and stops | BD-135 (estimate in four cost lines, session budget refusal, the budget-stop rule and its three exits, jobs, progress events, cancellation that drains, immutable addressed answers); N25a-24 (end-to-end script); N25a-33 (the predicate "the unresolved set satisfies `resolution`"); N25a-12 (refinement of an earlier answer) |
| §25.3.5 | Chance constraints | N25a-11 (population, event and confidence; joint law against additive budgets; the twenty-world case; conditional-success lower bound; refusal 846; study types and the blinded lane for empirical cells) |
| §25.3.6 | Minimum, repair and gap claims | N25a-10 (incumbent with matching lower bound, `feasible-candidate`, attained and unattained infima, refusal 853); N25a-09 (binding constraint from the dual, smallest admitted change as a typed edit); N25a-16 (feasibility and optimality as separate obligations after an edit); N25a-34 (open thresholds and a catalog repair, from the user's side) |
| §25.4.1 | Objects and the feasible set | N25a-03 (D, W, Π_Y, T, A, the definition of F, the clause T ≠ ∅); N25a-04 (undefined dynamics do not satisfy a universal quantifier); N25a-02 (design domain quotient, world fixed before candidates are tested) |
| §25.4.2 | Quantifier order | N25a-03 (the three weaker statements, the seven rules that follow from the order, refusals 881, 884, 887); BD-131 and N25a-22 (the standing pusher regression under both prefixes); N25a-02 (policy indexing, persistence with no default, one decision maker) |
| §25.4.3 | Progress, Zeno behaviour and unknown | N25a-04 (the six rules: deadlock and Zeno, model-level failure, admitted completion, numerical failure is unknown, comparators as declared, scalarization is the caller's) |
| §25.4.4 | The answer as data | N25a-05 (`Cell`, `Binder`, `Verdict`, `Answer`, `Explanation`; coverage and confidence in separate fields); N25a-33 (`Resolution`, `ResolutionReport`) |
| §25.4.5 | Four proof rules | N25a-07 (one accepted fixture and the deliberately invalid fixtures per rule: the sill work cut with its omitted source, constructive inclusion and its scope today, the symmetric-actuator case, partitions and composition); N25a-31 (sign proofs valid over a cell, for rules 1 and 2); BD-288 (rule 3); the proposers themselves are BD-128 (unit U25b) |
| §25.4.6 | Monotonicity and its test | BD-288 (the two transports, the three-part test, the falsification harness, refusal 852); N25a-15 (pilot and corpus: 23 non-monotone fixtures, 10 controls); N25a-25 (GATE-AUGUR-08 script) |
| §25.4.7 | Shortcuts that are not proofs | N25a-08 (twelve fixtures with their shortcut mutants, registered as negative controls); each row is also cited by the mechanism bead that replaces the shortcut (BD-130, BD-131, N25a-03, N25a-04, N25a-11, N25a-07) |
| §25.4.8 | Arithmetic, and where proof assistants are used | N25a-06 (interval and affine bounds, rational reconstruction of linear-program witnesses and duals, checker inputs against proposer annotations, bulk predicate classes, where proof assistants are used); N25a-30 and N25a-31 (the sum-of-squares bound on polynomial subdomains: pilot and proposer); BD-355 and N25a-28 (exact-integer and guarded kernels on devices) |
| §25.4.9 | Owners worth probing before new code is written | BD-127 (the second linear-program solver, `fs-ivl`, `fs-sos`: admission records); N25a-32 (`fs-regime`, `fs-voi`, `fs-eproc`, `fs-checker` as probes with executable pin facts; the register of what the suite lacks; the rule that every mapping has an invalid fixture) |

Seed beads of this unit that draw on subsections owned elsewhere, written completely here:

| Seed | Draws on | Own beads that complete it |
|---|---|---|
| BD-002 | §34, §33.6, §38.13 (companion documents, registries, the honest-status table) | — |
| BD-127 | §33 (admission), §25.15 (the 51 docking programs) | N25a-32 |
| BD-130 | §25.13 (stance family), §10 C-32 | N25a-06, N25a-13, N25a-21 |
| BD-131 | §25.22, §25.26 (the ten frozen cases) | N25a-03, N25a-22 |
| BD-135 | §25.14 (first six steps of the algorithm), §32 | N25a-09, N25a-10, N25a-12, N25a-24, N25a-33, N25a-34 |
| BD-140 | §25.23 (slice 1), §25.27 | N25a-14 (the ten scripted amendments and rubric) |
| BD-288 | §25.5.2, §25.6 | N25a-15, N25a-25 |
| BD-292 | §25.8, §28.13, §28.14 | N25a-13 (M1 form), N25a-16, N25a-17, N25a-19, N25a-26 |
| BD-316 | §39 (protocol, phase ledger, disqualification) | N25a-18 |
| BD-354 | §25.12, §36.5, §38.4, §39.8 | N25a-27 |
| BD-355 | §25.12, §35 | N25a-28 |
| BD-387 | §30 (Ground) | N25a-29 |
| BD-436 | §25.11, §22.10 (epic) | N25a-20 |

Identifiers defined in the chapter and where this unit carries them:

| Identifier | Beads |
|---|---|
| INV-AUGUR-01 to -11 | All eleven are stated in N25a-01 (the crate contract). Tested: -01 N25a-07; -02 N25a-05, N25a-33; -03 N25a-02; -04 N25a-05, N25a-24, N25a-33; -05 N25a-05; -06 N25a-04, N25a-03, N25a-31; -07 N25a-02, N25a-03; -09 N25a-05; -10 N25a-12, N25a-32, N25a-31; -11 N25a-01. INV-AUGUR-08 (self-contained payload) is stated here and tested by unit U25b's bundle beads |
| PO-AUGUR-01, PO-AUGUR-06 | PO-AUGUR-01: BD-130 (second lowering), BD-131, N25a-07, N25a-32; PO-AUGUR-06: N25a-07. PO-AUGUR-02 to -05 and -07 concern the checker and are carried by units U25b and U25d |
| OPEN-AUGUR-1, -2, -4, -5, -7, -9, -11 | Default and owner in the Considerations of N25a-02 (-1, -9, -11), N25a-06, BD-130 and BD-127 (-2), N25a-18, BD-131 and BD-127 (-4), N25a-07 and N25a-04 (-5), BD-135, N25a-24 and N25a-33 (-7, -11). Also named where they bear: -3 (N25a-30, N25a-31), -6, settled (N25a-01), -8 (N25a-05), -10 (BD-436, N25a-20). The decision beads are unit U25d's (see (d)) |
| KILL-AUGUR-TIE, -REUSE, -TRANSPORT | Decision beads N25a-18, N25a-19, N25a-20 |
| KILL-AUGUR-FALSECERT | BD-316 (disqualifies the arm); procedure and decision in unit U25b |
| BET-13, BET-14, BET-19, BET-33; RISK-17, RISK-26, RISK-35, RISK-36, RISK-38, RISK-54 | BET-13: BD-316; BET-14: N25a-18; BET-19: BD-140, N25a-18; BET-33: BD-292, N25a-13, N25a-17, N25a-19, N25a-26. RISK-17: N25a-18; RISK-26: BD-140, N25a-18; RISK-35: N25a-03; RISK-36: N25a-01; RISK-38: BD-292, N25a-13, N25a-19, N25a-26; RISK-54: BD-002 |
| C-17, C-18, C-32; T-21, T-22 | SPEC-C-17: N25a-03, BD-131, BD-288; SPEC-C-18: BD-288, BD-292, N25a-13, N25a-16; SPEC-C-32: BD-130, N25a-31; theorem cards cited by BD-130 and N25a-06 |
| Verbs (Appendix J) | `mission.synthesize`, `mission.refute`, `mission.distinguish` (first form), `mission.show`: BD-135. `mission.revalidate`: N25a-16. `result.revalidate` (engine): BD-292. `ground.ingest`: BD-387. `mission.transport`, `skill.transport`: BD-436 (epic) |
| Type sketches (Appendix I) | N25a-05 (cells, binders, verdicts, answers, explanations); BD-292 (the verdict record); N25a-33 (resolution) |

## (b) Gates and kill gates

Gates whose mechanism lies in §25.1 to §25.4, and gates named by this unit's seed beads:

| Gate | Milestone | Measured or decided by |
|---|---|---|
| GATE-AUGUR-01 | M1 | Admission side: N25a-02, N25a-03, N25a-04, N25a-11 (the refusals), N25a-07 and N25a-08 (the invalid fixtures and shortcut rows the corpus must contain), N25a-23 (script from the command line). The corpus of at least 60 cases and the gate record are BD-136 (unit U25b) |
| GATE-AUGUR-02 | M1 | N25a-21 (script and record); mechanism BD-130 with N25a-05, N25a-06, N25a-10, N25a-13 |
| GATE-AUGUR-04 | M1 | N25a-22 (script and record); mechanism BD-131, N25a-03 |
| GATE-AUGUR-07 | M1 | BD-140 (the run and the record); N25a-14 (fixture and rubric); BD-135, N25a-09, N25a-12 (what the agent drives) |
| GATE-AUGUR-08 | M3 | N25a-25 (script and record); BD-288 (mechanism); N25a-15 (pilot and corpus) |
| GATE-AUGUR-12 | M3 | N25a-26 (script and record); BD-292, N25a-16 (mechanism); N25a-17 (campaign) |
| GATE-AUGUR-14 | M3, M4 | N25a-27 (script and record; audit-only mode for the M3 part); BD-354 (the check) |
| GATE-AUGUR-15 | M4 | N25a-28 (script, two site records and their merge); BD-355 (mechanism) |
| GATE-AUGUR-23 | M8 | BD-436 (epic); the experiment and its script are unit U25c's |
| KILL-AUGUR-TIE | M3; first reading at M1 | N25a-18 (decision); BD-140 (first reading); record from BD-317 |
| KILL-AUGUR-REUSE | Standing, from M3 | N25a-19 (decision per rule); N25a-26 (evidence); BD-292, N25a-16 |
| KILL-AUGUR-TRANSPORT | M8 | N25a-20 (decision); BD-436 |
| KILL-AUGUR-FALSECERT | Standing, from M1 | BD-316 (one false certificate disqualifies our arm of the partial gate); decision bead in unit U25b |
| GATE-ATEL-11 (§28) | M1 (mission cells), M3, M4 | N25a-13 (M1 rows on mission cells); BD-292, N25a-17, N25a-26 (M3 dependency-grade rows); grades 3 and 4 at M4 are BD-347 |
| GATE-TENFOLD-06 (§39) | M3 | BD-316 (our arm); record by BD-317 |
| GATE-GAUNT-10 (§38) | M0 | BD-002 |
| GATE-REFLEX-09, KILL-SKILL (§22) | M8 | BD-436 (epic; the mechanism and decision are unit U22b's) |

Gates of chapter §25 that this unit expects other units to carry, because their mechanisms lie in other
subsections: GATE-AUGUR-03 (docking family) and GATE-AUGUR-11, -16, -19, -20, -24 with KILL-AUGUR-PROBE,
-DOMINANCE, -LABELS, -PHASE, -FACILITY (unit U25c); GATE-AUGUR-05, -06, -09, -10, -13 with KILL-AUGUR-REGION,
-CUTS, -SEPARATE, -ALIAS and the decision for KILL-AUGUR-FALSECERT (unit U25b; BD-294 and BD-358 in unit
U25e); GATE-AUGUR-17, -18, -21, -22 with KILL-AUGUR-SYMEVENT and KILL-SYNTH-CODESIGN, -TRAJOPT, -LINKAGE,
-CONT (unit U25d, which owns the gates subsection); GATE-AUGUR-25 to -35 with KILL-COUPLING,
KILL-AUGUR-BRACKET, KILL-SYNTH-REGRESSOR and KILL-WEAKVAR (unit U25e).

Mechanisms of this unit with no gate in the register: the sum-of-squares proposer (N25a-31; its pilot N25a-30
carries a registered falsifier and fallback instead); the resolution test (N25a-33); `mission.distinguish` in
its first form (N25a-34 is its script; its gate, GATE-AUGUR-11, belongs to the full form); data admission for
Ground (BD-387, N25a-29).

## (c) Counts

- Subsections covered: 4 (§25.1, §25.2, §25.3 with its six sub-subsections, §25.4 with its nine), plus the
  chapter preamble.
- Gates and kill gates carried in a `gates` field: 18 (9 convergence gates of §25, 4 kill gates of §25, and
  GATE-ATEL-11, GATE-TENFOLD-06, GATE-GAUNT-10, GATE-REFLEX-09, KILL-SKILL through seed beads).
- Known answers. Registry N25-KA: all 35 rows of the chapter (KA-624 to KA-658) with case, truth, tolerance,
  tier, control and realizing bead; a proposed control for each of the 8 rows that state none and a tolerance
  for the 1 row that states none; 18 late known answers of §25.29 to §25.39 under provisional tags. Cited with
  their values in mechanism and script beads: 27 (KA-624 to KA-632, KA-635 to KA-639, KA-641, KA-643, KA-644,
  KA-651, KA-654, KA-656 to KA-658 of §25; KA-001 of Part 0; KA-022 of §7; KA-699 to KA-701 of §28).
- Counterexamples cited with their cases: 20 (CX-270 to CX-285 and CX-287 of §25, which is 17 of the chapter's
  18; CX-001, CX-314, CX-316 from other chapters). CX-286 (repeated reads) is carried by unit U25b. The
  registry N25-CX is unit U25e's.
- Refusal codes: 18 of the chapter's 20 are carried with meaning, trigger and first repair in the bead whose
  mechanism raises them (846, 847, 848, 849, 850, 851, 852, 853, 854, 855, 856, 858, 868, 874, 877, 881, 884,
  887); 857 (synthesis caps) and 859 (unrealizable experiment) belong to units U25d and U25c. Codes of other
  chapters used: 701, 702, 724, 802, 814, 826, 829, 835, 866, 873, 883. The registry N25-REF is unit U25e's.
- Metamorphic relations cited: MR-10, MR-11, MR-12, MR-35, MR-40.
- End-to-end script beads: 11 (N25a-21, -22, -23, -24, -25, -26, -27, -28, -29, -34, and BD-140's agent run);
  pilot beads: 3 (N25a-15, N25a-17, N25a-30); kill-gate decision beads: 3 (N25a-18, N25a-19, N25a-20).
- Numbers recomputed while drafting, exactly, and in agreement with the plan: the stance boundary 3/14 m and
  its witnesses; the stance grid (2,254 feasible states, 114 covering bodies, 1,710 bodies accepted by the weak
  screen, of which 1,596 wrongly); the docking grid (344, 217, 9); the 65,536-row screen (34,590; 30,946;
  1,820) on both grids; the 1,024-design batch (616, 368, 40); the majority-vote and stop-at-gap-two values;
  the jaw bounds 28/17 and 4/3; the label deadlines; 1/65 m; every ceiling in N25a-27.

## (d) Needs from other slices

Each bead's `needs` field lists these with the plan subsection; grouped here by owner.

- **Lemma (§12) and the bundle (unit U12 and unit U25b).** The checking rules for `lp-primal`, `lp-farkas`,
  `partition`, `finite-game` (BD-132); for `monotone-transport` at M3 and `sos-gram` at M4; the writer's
  mapping for those kinds (BD-133 extended); implication records in the family bundle and the auditor's refusal
  of reuse without one (835); the prefix rule (826); the theorem-card registry for the four proof rules; the
  plain proof-object types in `frob-core`; the one-false-certificate procedure and the decision bead for
  KILL-AUGUR-FALSECERT.
- **The other units of this chapter.** U25b: inconsistent missions (877), the full search loop, quotients and
  cuts, the alias and jaw fixtures, the blind payload task. U25c: the docking family as a public query
  (BD-129) and its script (GATE-AUGUR-03), probe design and the full `mission.distinguish`, `experiment.suggest`,
  the static core and the experiment of `mission.transport`, the family batch planner and placement. U25d: the
  constructive lanes behind `--strategy`, the delivery-order beads, the refusal battery, the invariant and
  proof-obligation suites, the known-answer batteries that realize N25-KA's rows, the gate and pilot registry
  rows, and the decision beads for OPEN-AUGUR-1, -2, -4, -5, -7, -9 and -10 (N25a-18 needs the one for
  OPEN-AUGUR-4 before the M3 partial gate; BD-135 and N25a-24 follow the default of OPEN-AUGUR-7 and change
  in one place if it is decided otherwise). U25e: `policy_class.law` and the law object in `lp-primal`
  certificates, the coupling budget, the no-fill rule, monomial screens and utility bounds for
  `mission.refute`, `--split-cap`, Pareto coverage, docking to a tolerance, and N25-CX and N25-REF.
- **Gauntlet (§38).** The gate runner that writes rows of `registries/gates.toml` and refuses a self-certified
  pass (BD-025), including a merge of per-site records (N25a-28); the loader and row schema of the
  known-answer and negative-control registries (BD-146); suite numbers for the 18 late known answers; the
  registry of method experiments with its six parts; the wiring of the 30-amendment campaign into the M4
  battery (BD-377).
- **Tether (§32).** Generated schemas and option names for the mission verbs (BD-137); MCP tools (BD-138,
  BD-298); calibrated cost intervals for `--estimate` (BD-297); the flag set and exit-code table (BD-022).
- **Atelier (§28) and Chronicle (§29).** Revalidation grades 3 and 4 and KILL-REUSE (BD-347); the memo graph
  (BD-111); fork classification (BD-205); the registry entry of `result.revalidate`.
- **Suite and workspace (§33, §34).** The foundation ledger and stack lock; `xtask check-deps` and
  `check-layers` (BD-003); the group gate INT-FSIM-14 (BD-124); the final name of the performance harness's
  registration entry point (BD-216).
- **Ten-fold test (§39).** The pinned agent model and its caps, the adjudicator, the campaign compute class
  (OPEN-TENFOLD-2, -4, -6); the ledger schema and its closure check; GATE-TENFOLD-09 and KILL-TENFOLD-TIE.
- **Other subsystems.** Typed step outcomes of the referee (BD-081 to BD-083); typed edits (BD-058); module
  invalidation sets (BD-063, BD-077); kernel classes with derived error bounds (BD-053); placement receipts
  (BD-178); study types and the blinded lane (BD-124, BD-259); for Ground: clock fitting, frozen predictions,
  drift monitors, the reality line, the private mark (BD-389, BD-392, BD-394, BD-139, BD-305).
- **Owner.** An approved compute envelope for the device runs of N25a-28; the funding decision for Moonshot
  slices behind BD-436.

## (e) Contradictions, gaps and what was done

1. **GATE-AUGUR-14 is due at M3 and M4; its seed bead BD-354 is at M4.** N25a-27 runs in audit-only mode at
   the M3 exit against the program-wide check that exists from M1 (GATE-TENFOLD-09) restricted to this
   subsystem; the full script runs at M4. Stated as a DESIGN in N25a-27.
2. **The frozen 30-amendment campaign is listed in the M4 conformance seed (BD-377), while GATE-AUGUR-12 and the
   dependency-grade rows of GATE-ATEL-11 are due at M3.** N25a-17 creates and seals it at M3; the M4 run uses a
   new campaign.
3. **`mission.revalidate` and `result.revalidate` have first milestone M4 in the verb registry, M3 in the
   milestone plan and the gate, and appear in M1 amendment examples.** N25a-13 gives the M1 mission-cell form
   (grades 1 and 2), BD-292 and N25a-16 the M3 engine and verb.
4. **`ground.ingest` is M1 in the verb table and its seed bead is M5; the verb table's refusal numbers for the
   cluster differ from the chapter's table and the registry.** BD-387 and N25a-29 follow the seed's milestone
   and the registry's codes (873, 883, 814).
5. **The worked session writes refinement as `--strategy refine --from a:…`, while `refine` is also the name of
   the trajectory-optimization lane.** N25a-12 uses `--from` alone; the registry freeze must settle the name.
6. **The worked session shows exit codes that differ from the stated budget-stop rule.** The rule is taken as
   normative (BD-135, N25a-24, N25a-33).
7. **Wrong-prefix evidence is refused with 826 in the known-answer and counterexample rows and with 881 in the
   chapter's refusal table.** Both are kept: 826 is the auditor's rejection of a bundle, 881 the mission
   verb's refusal (N25-KA, N25a-03, N25a-22).
8. **KA-634 (passive jaw) is filed under §25.11 while its text is in §25.10; Appendix M has no rows for §25.29
   to §25.39.** N25-KA keeps the suite's reference and adds the other, and registers the 18 late known answers
   under provisional tags.
9. **Eight known-answer rows state no negative control and one states no tolerance.** N25-KA proposes a
   mutation for each and a tolerance for KA-649 (from GATE-AUGUR-20), marked as proposals that must be
   registered before a gate cites the row.
10. **The sum-of-squares bound of §25.4.8 is marked Frontier but has no pilot, falsifier, gate or kill gate in
    the plan.** N25a-30 defines a pilot in the required form (twelve exact fixtures, four arms, a registered
    falsifier, ceiling and fallback) and N25a-31 the proposer, which exists only if the pilot does not meet
    its falsifier. By hand analysis a Bernstein coefficient-sign check decides most of the pilot's fixtures at
    once, so the expected outcome may well be the fallback; the pilot says so.
11. **The meaning of the mandatory `interior`, `exterior` and `boundary` obligations of `resolution` is not
    defined beyond one rule ("a vacuous enclosure cannot satisfy a mandatory obligation"), and only two
    measures appear in examples.** N25a-33 fixes a reading, as DESIGN, consistent with both worked queries.
12. **The plan fixes the size and labels of the 1,024-design family batch and not its contents; "restricted to
    the lower verbs" is not listed; more than one decision maker has no refusal code; `name`, `response` and
    `budget` are not said to be outside the hashed mission bytes; the three admissions of BD-127 have no INT
    identifier; no verb registers a speed comparison.** Each is settled in its bead as a labelled DESIGN
    (BD-355; BD-140; N25a-02 with code 724; N25a-02; BD-127; BD-354 and N25a-27 through the harness).
13. **Facts about sibling crates were read at a head later than the plan's.** The plan cites the simulation
    sibling at `483d09c682f2`; the local checkout read for N25a-30, N25a-31 and N25a-32 was at `e49c0f7d7657`.
    Signatures quoted there are labelled with that head, and N25a-32 turns each fact into a test that fails
    when it stops holding.
14. **Overlap with beads of other units, for the lead to merge or cross-link.** The chapter's seed beads draw
    on subsections other units own, and those units wrote complements; several pairs cover the same work:
    N25a-16 with N25b-18 (`mission.revalidate`); N25a-17 with N25b-19 (the campaign); N25a-19 with N25b-20
    (KILL-AUGUR-REUSE); N25a-26 with N25b-21 (GATE-AUGUR-12 script); N25a-20 with N25c-05
    (KILL-AUGUR-TRANSPORT); N25a-13 with N25c-11 (the passive-spring amendment); BD-130's validation grid with
    N25c-12; N25a-12 with N25b-12 (the worklist); N25a-07 with N25d-36 (the proof rules as candidate cards);
    N25a-23 with N25d-32 (refusal scripts); N25a-27 with N25c-06 (f, r, h of the family workflow); N25a-28
    with N25c-09 (placement of screens); BD-140 with N25d-27 (the first slice from the user's side). Within
    this unit the dependencies are consistent; none of the listed pairs was merged here.
15. **BD-436 is an epic whose Augur-side children are in unit U25c** (the tray pilot, the static core, the
    relation, the experiment); only the kill decision N25a-20 is here, and U25c also wrote one.

Self-check: 48 files parse; every key matches its file name; every `deps` and `parent` key is a seed key, a
reserved key or a key of this unit; every seed dependency is kept; no dependency points to a later milestone;
the dependency graph inside the unit has no cycle; every body has the seven sections and a checklist.
