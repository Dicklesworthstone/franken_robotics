# Fix decisions for the reconciliation pass

The plan's chapters exist as part files under `parts/` (§1–§37) and `parts2/` (§38–§47 and appendices). Two
reviews and two completeness audits found contradictions between chapters. The exact-substitution fixes of the two
reviews have already been applied, and the audits' insertion blocks have already been inserted as new subsections.
What remains is the list-form findings below. A fixer applies every finding that touches its files, by editing the
part files in place.

## Sources of findings (read all)

1. `GAPFILL_1.md`: sections "(b)", "(b2)" and "(d)" (15 contradictions).
2. `GAPFILL_2.md`: section "(b) Changes required by the adjudication" (corrected wordings with locations) and
   section "(d)" (36 contradictions).
3. `/Users/jemanuel/projects/franken_robotics/planning/fable/brenner/deliverables/CHALLENGE_ADJUDICATION.md`.
4. The `<!-- LEDGER -->` blocks at the ends of `parts2/P13a.md`, `P13b.md`, `P14.md`, `P15.md`, `P16a.md`,
   `P16b.md`: each lists inconsistencies and numeric discrepancies its author found in the parts.
5. `REVIEW_MATH.md`: findings F-17 and F-21 (multi-line corrections given as text in the table) and any finding
   whose replacement was not single-line. `REVIEW_COHERENCE.md`: the prose instructions outside code fences in
   section 2, and section 5 ("What an implementing agent still cannot tell").

Line numbers in those documents predate the insertions; locate text by searching for it.

## General rules

- **The owner chapter governs.** Where two chapters state one mechanism differently, the chapter that owns the
  mechanism keeps its statement and the other is changed to agree and to cite it. Owners: §12 certificates and
  proof kinds; §16 the step, tiers and tier correction arithmetic; §17 mixed batches, families, paired-difference
  gradients; §18 contact laws; §19 scenes; §20 sensors and clocks; §21 device kernels; §22 controllers, gaits,
  shields; §23 learning; §24 tasks, requirements, qualification; §25 mission queries; §26 adversarial search,
  rare events and the screening audit; §27 intervention semantics; §28 the ladder, comparison, lineage; §29 the
  store, telemetry, the energy ledger, the tier-correction ledger, the fork mechanism; §30 calibration; §31
  structure and build packets; §32 the verb registry and the daemon; §33–§34 foundations, crates, formats; §35
  determinism; §36 performance numbers; §39 the ten-fold test; §40 the gate register.
- **The evidence governs numbers.** A number the adjudication or an audit corrected is corrected everywhere it
  appears. Recompute any arithmetic you touch.
- **Minimal edits.** Change the sentences that are wrong. Do not rewrite sections, do not add praise, do not add
  attribution to any plan, model or exercise, keep the specification register, Unicode mathematics, and the wrap
  near 118 characters. Do not touch `<!-- LEDGER -->` blocks except to correct a stale line a finding names.
- **Do not renumber** sections, gates, bets or refusal codes. New rows may be added to tables.

## Specific decisions

1. **Verb names.** The owner chapter's verb name is the registered name; §32.6 and Appendix J list exactly those.
   `world.separate` becomes `mission.separate`. `skill.transport` moves a skill artifact with a learned residual
   [M]; `mission.transport` moves a certified claim through an interface map with a bounded error [F]; say so in
   §22.10 and §25.3. `opt.cascade` runs the co-design cascade; `solve.run` is the agent-facing orchestration verb
   that calls it, and KILL-SOLVE and GATE-ATEL-03 are judged on the same A/B: say so in §28.8 and §32.
   `telemetry.export-html` belongs to the telemetry cluster of §29; §20.10 keeps a pointer to the viewer text
   in §32.
2. **Proof kinds.** One closed registry, `frob.cert.kinds/1`, in §12.4. Add the rows `taylor-box`, `sos-gram`,
   `symmetry-transport` and `composition`. §25.5.2 lists the kinds mission answers use and cites §12.4.
3. **Forks.** §29.8 owns the fork mechanism and adopts the five classes of §27.4; GATE-CHRON-05 tests those five;
   the warm-start-graft refusal has one code, the one §27.4 uses.
4. **Screening audit.** One mechanism, owned by §26.10: verb `nemesis.audit`, gate GATE-NEMESIS-07, exact bound
   1 − δ^(1/m) (9.5% at m = 30, δ = 0.05). §28.12 applies it and cites it; GATE-ATEL-09 becomes the statement that
   Atelier does not trust a rung whose audit is missing or failed.
5. **Energy audit.** One flag, `energy-created`, with the window and denominator of §29.9; §16.10 cites it.
6. **Derivative labels.** The label set of §9 (D-15) and C-16 governs; §22.9.3 maps its four words onto it.
7. **Requirement verdicts.** `satisfied`, `violated`, `undetermined`, `invalid` (§24.5). C-30 uses `undetermined`.
8. **Plan-time pilots.** Cited as "FACT, Appendix K". The solver pilot: 1.345 times warm and 1.088 times cold.
   The attribution "94% is shared checking and serialization" is withdrawn at source: write "a residual of
   575.7 µs that the solver does not touch (parsing, assembly, checking, serialization and harness, not separately
   timed)". The ceiling of about 1.46 (842.8 ÷ 575.7) stands.
9. **Rounded figures.** The environment-step ceiling is "about 7.7 (1/(1 − 0.87))". The training-time planning
   value at 1.25 times 410 s is 513 s. Use the corrected values that `parts2/P16a.md`'s ledger lists for §0.8
   H-05 (about 1% to 63% of a 410 s training run), §16.11, §20.4.6, §21.20, §24.10, §26.9 and §28.9 after
   recomputing each.
10. **Device memory budget.** 85% of the measured working-set limit, in §19.15 and §36.10.
11. **Size buckets.** {12, 18, 24, 36, 48, 64} links, as §17 states.
12. **Notation.** The mechanical symmetry group is Γ_mech and the observation-preserving subgroup is Γ_obs
    everywhere.
13. **Model-table size.** About 192 bytes per link (12 kB for a 64-link variant, about 3 MB for 256 variants), as
    §21.11 states; collision sample lists are counted separately. Correct §17.4 and §17.7.
14. **Matrix accelerators.** Not adopted for policies of a few hundred units (§23.8); correct the last sentence
    of §18.12.
15. **Mixed-batch figure.** An incumbent's mixed batch of three body plans runs at 0.86 of the harmonic mean of
    its homogeneous rates (about 86 thousand samples per second against about 100 thousand).
16. **Structural mass ceiling.** Links 7–30% lighter; 2–8% of robot mass on the one quadruped with a published
    breakdown.
17. **Rare-event figures.** Published measured gains are 2–20 times and about 5 times in mean-square error; 10–100
    times at 10⁻⁴–10⁻⁵ is a TARGET. Bounded-weight sampling is for refutation and for estimates once failures are
    observed; it never certifies absence more cheaply than crude sampling. Restate BET-32 accordingly.
18. **Sampling-controller cost.** About 0.33 and 1 GPU-minutes per 20 s episode (quadruped, humanoid) at a 20 ms
    planning step, about 1.7 and 5.2 at 4 ms. BET-27 and GATE-REFLEX-06 use one pair of thresholds: 1 and 6.
    Screening defaults to three episodes per design; paired comparisons may use up to the break-even (about 30
    and 60).
19. **Tier-correction weights.** Bounded without clipping, as §29.11 states; §16.8 agrees and cites it.
20. **Foundation defects.** Five defects or limits (three errors in the suite's PPO update, its action sampler's
    joint law, the planar-drive integration defect, the stride-convention trap, the fixed-witness assumption count
    as listed in §33.11 after the insertion of §33.17), plus the mixed-frame `fs-mbd::Wrench` as a convention
    hazard. Make the counts in §1.7, §23 and §33.11 agree with §33.17. The rainflow counter lives in `fs-material`.
    The cloth oracle cannot share a Cargo graph with FrankenSim (conflicting exact `serde_json` pins): its posture
    is Subprocess. 64-bit atomics are "reported absent on Metal, to be re-run by the fingerprint".
21. **The ten-fold decision rule.** The sealed-suite result is a finite-suite result. A population claim needs at
    least 32 independent mission families (§39). §2.1 points to §39 and does not state "ten paired campaigns".
22. **Energy-budget card.** The labels of §22.11 govern (external work, stored, reserve); make C-31's test and the
    `.fenergy` sketch arithmetic agree with it.
23. **Tube tables.** [M], at M8, as §22.12.5 states; C-31's tier line says so, and the inclusion is stated once.
24. **Referee coverage.** The band "about 0.1–2.4% of GPU world-seconds" always travels with its assumption (a
    referee at 0.2–1.0 times real time per core on 24 cores).
25. **The verdict register.** §0.8 prints the register as revised by the adjudication (GAPFILL_2 section b.1),
    including the split rows and H-25 to H-29, and the new closing paragraph.
