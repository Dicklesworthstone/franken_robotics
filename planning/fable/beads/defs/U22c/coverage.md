# Coverage of drafting unit U22c (§22.14 to §22.25 of Reflex)

Files: 60 bead files. One seed bead (BD-118). 59 new beads: the registries `N22-CX` and `N22-REF`, and
`N22c-01` to `N22c-57`. Keys of other units' new beads are not known, so cross-unit prerequisites are in `needs`.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §22.14 | Refusals | N22-REF (the eleven codes with triggers, payloads, ranked repairs, exit code 3; four worded refusals of the later subsections with proposed slugs) |
| §22.15 | Invariants and proof obligations | N22c-02 (INV-REFLEX-01, -02; PO-REFLEX-01, -03; due M1), N22c-03 (INV-REFLEX-03, -04; INV-GAIT-01; INV-SHIELD-01, -02; PO-REFLEX-02; PO-GAIT-01, -02; PO-SHIELD-01 to -04; due M4); BD-118 (INV-REFLEX-01, -02 at the verb); N22c-44 and N22c-46 (PO-SHIELD-04, sampled part) |
| §22.16 | Known answers and negative controls | N22c-04 (the fifteen hand fixtures as data, plus fixture 16 for CX-243 and eight provisional rows from the later subsections); N22-CX (the negative controls as registered mutants). The registry `N22-KA` belongs to unit U22a |
| §22.17 | Pilots and ceilings for the Frontier and Moonshot items | N22c-05, N22c-06 (Frontier rows of baseline synthesis, two classes each); N22c-07 (baseline plus residual); N22c-08 (sampling controller); N22c-09 (gait orbits and continuation); N22c-10 (energy budget); N22c-11 (reduced-order barrier shield); N22c-12 (skill transport, Moonshot); N22c-13 (reachable-set tables, Moonshot) |
| §22.18 | Gates and kill gates | N22c-49 (all 23 convergence gates and 12 kill gates as registry rows with full thresholds); BD-118 and N22c-01 (GATE-REFLEX-01 at M1); N22c-47 (GATE-REFLEX-01 at M2); N22c-48 (GATE-REFLEX-01 at M4); kill-gate decisions N22c-14 (KILL-BASELINE), N22c-15 (KILL-RESIDUAL), N22c-16 (KILL-MPC), N22c-17 (KILL-GAIT), N22c-18 (KILL-ENERGY), N22c-19 (KILL-SHIELD), N22c-20 (KILL-SKILL), N22c-57 (KILL-TUBE) |
| §22.19 | Open questions | N22c-50 (OPEN-REFLEX-1, experiment), N22c-51 (OPEN-REFLEX-2, settled by the register), N22c-52 (OPEN-REFLEX-3), N22c-53 (OPEN-REFLEX-4, experiment), N22c-54 (OPEN-GAIT-2, and OPEN-GAIT-1's recorded default; OPEN-GAIT-1 also in the Considerations of N22c-36), N22c-55 (OPEN-SHIELD-1), N22c-56 (OPEN-SHIELD-2, owner) |
| §22.20 | A lifted linear planning model for one soft-soil regime | N22c-21 (pilot), N22c-23 (`control.lift`: fit, calibration, admission), N22c-24 (`control.mpc --planner lifted:<id>`), N22c-25 (GATE-REFLEX-10), N22c-26 (KILL-SOILLIFT), N22c-22 (upstream change in `fs-surrogate`) |
| §22.21 | The pre-slip aliasing fixture and the sampled-data backup domain | N22c-27 (fixture), N22c-28 (timeliness rule in `shield.fit`, sampled-data backup domain, `shield.check` replay), N22c-29 (GATE-SHIELD-06) |
| §22.22 | The sampling controller as the delivered controller | N22c-30 (pilot), N22c-31 (planner record, `--freeze`, latency model), N22c-32 (lane admission in `frob-arena`, verdict, voiding, ledger), N22c-33 (GATE-REFLEX-11), N22c-34 (KILL-MPCRECORD) |
| §22.23 | Orbits of a loop with no clock: the free period and the trivial multiplier | N22c-35 (pilot), N22c-36 (classification, refusal, section form), N22c-37 (bordered form, folds, dT*/dθ, adjoint), N22c-38 (GATE-GAIT-04), N22c-39 (KILL-FREEPERIOD) |
| §22.24 | Stability over admitted sequences of strides | N22c-40 (pilot), N22c-41 (mode graph, cycle-product lower bound, common-norm upper bound, verdict, exact check), N22c-42 (GATE-GAIT-05), N22c-43 (KILL-STRIDESEQ) |
| §22.25 | Sampled damping, and the work of a held command | N22c-44 (gain bound in `shield.fit`, rational check, latency case, refusal, physical dampers), N22c-45 (held-command work bound and reservation rule; ledger), N22c-46 (GATE-SHIELD-07) |

Seed bead BD-118 (`control.synth` with three-outcome reporting, and `control.show`) draws on §22.4 and §22.6,
which unit U22a owns; it is written completely here, with its end-to-end script in N22c-01.

## (b) Gates, kill gates and the beads that measure or decide them

| Gate | Milestone (register) | Measured or decided here | Expected from another unit |
|---|---|---|---|
| GATE-REFLEX-01 | M1; M2; M3; M4 | BD-118 + N22c-01 (M1, referee); N22c-47 (M2, `batch-cpu`); N22c-48 (M4, tracked and hybrid); pilots N22c-05, N22c-06 | BD-247 (M3, all but tracked and hybrid; U14b) |
| GATE-REFLEX-02 | M3 | N22c-14 reads it for KILL-BASELINE | BD-249 (U22a) |
| GATE-REFLEX-03 | M1 | N22c-02 feeds PO-REFLEX-01 | BD-114 (U22a) |
| GATE-REFLEX-04 | M3; M4 | N22c-51 fixes its thresholds | BD-328 (U23b) |
| GATE-REFLEX-05 | M3 | N22c-02, N22c-03 feed its properties; N22c-52 unblocks it | BD-251 (U22a) |
| GATE-REFLEX-06 | M4 | N22c-16 reads it; N22c-08 (pilot), N22c-53 (planning step) | BD-331 (U22a) |
| GATE-REFLEX-07 | M4 | — (fixtures 13, 14 in N22c-04; INV-REFLEX-04 case in N22c-03) | BD-327 (U22b) |
| GATE-REFLEX-08 | M4 | — (fixtures 4, 15, 16 in N22c-04) | BD-336 (U22b) |
| GATE-REFLEX-09 | M8 | N22c-20 reads it; N22c-12 (pilot) | BD-436 (U25a) |
| GATE-REFLEX-10 | M6 | N22c-25 | — |
| GATE-REFLEX-11 | M3 | N22c-33 | — |
| GATE-GAIT-01 | M4 | N22c-03 (PO-GAIT-01 case) | BD-333 (U33b) |
| GATE-GAIT-02 | M4 | N22c-54 unblocks it; N22c-03 (PO-GAIT-02 case) | BD-334 (U22a) |
| GATE-GAIT-03 | M4 | N22c-17 reads it; N22c-09 (pilot) | BD-335 (U22a) |
| GATE-GAIT-04 | M4 | N22c-38 | — |
| GATE-GAIT-05 | M4 | N22c-42 | — |
| GATE-SHIELD-01 | M4 | N22c-18 reads it; N22c-10 (pilot) | BD-337 (U22b) |
| GATE-SHIELD-02 | M4 | N22c-03 (PO-SHIELD-02, -03 cases) | BD-338 (U22b) |
| GATE-SHIELD-03 | M4 | N22c-19 reads it; N22c-11 (pilot); N22c-56 fixes its budget | BD-339 (U22b) |
| GATE-SHIELD-04 | M4 | N22c-56 fixes α | BD-340 (U22b) |
| GATE-SHIELD-05 | M8 | N22c-57 reads it; N22c-13 (pilot) | BD-440 (U22b) |
| GATE-SHIELD-06 | M4 | N22c-29 | — |
| GATE-SHIELD-07 | M4 | N22c-46 | — |
| GATE-SKEL-03 (standing rows) | M1; M3 | BD-118 + N22c-01 (first archetypes) | the Skeleton unit for the rest |
| KILL-BASELINE | M3 | N22c-14 | measurement in BD-249 |
| KILL-RESIDUAL | M4 | N22c-15 | measurement in BD-330 |
| KILL-MPC | M4 | N22c-16 | measurement in BD-331 |
| KILL-GAIT | M4 | N22c-17 | measurement in BD-335 |
| KILL-SKILL | M8 | N22c-20 | measurement inside BD-436 |
| KILL-ENERGY | M4 | N22c-18 | measurement in BD-337 |
| KILL-SHIELD | M4 | N22c-19 | measurement in BD-339 |
| KILL-TUBE | M8 | N22c-57 | measurement inside BD-440 |
| KILL-SOILLIFT | M6 | N22c-26 | — |
| KILL-MPCRECORD | M3 | N22c-34 | — |
| KILL-FREEPERIOD | M4 | N22c-39 | — |
| KILL-STRIDESEQ | M4 | N22c-43 | — |

All 23 convergence gates and 12 kill gates are also registry rows in N22c-49. The seeds BD-249, BD-330, BD-331,
BD-335, BD-337, BD-339, BD-436 and BD-440 say "KILL-… decided" in their done-conditions: the decision beads
above depend on those seeds for the measurement, so the seeds should list the decision bead as `related`, not as
a dependency (otherwise the graph has a cycle). If another unit also wrote a decision bead for one of these
kill gates, merge it into the one here.

## (c) Counts carried

- Subsections: 12 (§22.14 to §22.25).
- Gates: 23 convergence gates registered (7 measured directly here: GATE-REFLEX-01 parts M1, M2 and M4,
  GATE-REFLEX-10, -11, GATE-GAIT-04, -05, GATE-SHIELD-06, -07); 12 kill gates, each with a decision bead.
- Known answers: 17 of the chapter's 23 numbered rows carried by value in the fixture pack N22c-04 (KA-553, -554,
  -556 to -560, -562, -563, -565, -566, -568, -569, -571, -572, -574, -575); the other 6 (KA-555, -561, -564, -567,
  -570, -573) are gate-level rows quoted in N22c-03 and N22c-49; 8 provisional rows (KA-R22-a to -h) for the hand
  cases of §22.20 to §22.25, which Appendix M does not number.
- Counterexamples: 12 numbered (CX-234 to CX-245) and 7 provisional (CX-R22-a to -g) in N22-CX: 19.
- Refusal codes: 11 numbered (421, 424, 431, 444, 448, 451, 454, 457, 461, 464, 817) and 4 worded refusals
  with proposed slugs in N22-REF: 15.
- Invariants and proof obligations: 7 and 9, all with a conformance case and a planted violation (N22c-02,
  N22c-03).
- Pilots: 13 (nine for §22.17, one each for §22.20, §22.22, §22.23, §22.24).
- Open questions: 8 (seven beads; OPEN-GAIT-1 shares N22c-54).

## (d) Needs from other slices

| Need | Asked by |
|---|---|
| §38.5: the pilot register `frob-conformance/registry/pilots.toml` and its funding rule (GATE-GAUNT-08) | all 13 pilot beads |
| §22.16: the registry bead `N22-KA` (unit U22a) | N22c-04 |
| §12.4: an offline-auditor checker for conic dual certificates (orthant and second-order cone, exact rationals) | N22c-02 |
| §12.3: a theorem card for the continuous-time damping-injection identity (the chapter refers to one; none is numbered) | N22c-03, N22c-44 |
| §22.10: the GATE-REFLEX-09 measurement inside epic BD-436 | N22c-20 |
| §22.12.5: the GATE-SHIELD-05 measurement inside epic BD-440 | N22c-57 |
| §33.9: admission INT-FSIM-14 of the frankensim `stats` group (`fs-surrogate`) | N22c-21, N22c-23 |
| Appendix F: an upstream work-item number for the input-affine EDMD change | N22c-22 |
| §32: the verb-registry entry for `control.lift` | N22c-23 |
| §39.6: the phase ledger for our own runs | N22c-32 |
| §22.9: the labelled-derivative machinery of the orbit solver; the gait battery and its autonomous and multi-mode members | N22c-36, N22c-39, N22c-43 |
| §24.4: the task's terrain grammar | N22c-41 |
| §22.7: any work allowance defined for the learned residual | N22c-45 |
| §21: device backends able to run the control-program kernels | N22c-50 |
| §15 and §30: absorber interface cards with provenance classes; the Ground import lane | N22c-55 |

Mechanism beads of U22a and U22b that are Frontier or Moonshot should depend on the pilot beads here: BD-247 on
N22c-05 and N22c-06; BD-330 on N22c-07; BD-252 on N22c-08; BD-333 and BD-335 on N22c-09; BD-337 on N22c-10;
BD-338 on N22c-11; BD-436 on N22c-12; BD-440 on N22c-13.

## (e) Contradictions and things that could not be turned into work as written

1. **Worded refusals have no code.** §22.20, §22.21, §22.23 and §22.25 each define a refusal in words ("lifted
   planner outside its calibrated regime", an untimely monitored assumption, "fixed-period shooting on a loop
   with a free period", "sampled damping gain above the passivity bound") and Appendix O has no number for them.
   N22-REF registers them under slugs and leaves the number to whoever adds the row to `registries/refusals.toml`,
   so that two units cannot allocate the same code.
2. **Late known answers and counterexamples have no numbers.** Appendix M and Appendix N stop at KA-575 and
   CX-245 for this chapter. The hand cases of §22.20 to §22.25 are registered provisionally (KA-R22-a to -h in
   N22c-04; CX-R22-a to -g in N22-CX) with aliases to be kept after renumbering.
3. **OPEN-REFLEX-2 is already settled, with a different interim rule.** §22.19 leaves the threshold pair open
   with the default "the stricter pair, and until the gate has run the baseline rung discards only". §40.7
   adopts (0.7, 0.5) everywhere and says that below 0.5 or with no record a rung neither selects nor discards.
   The register governs: N22c-51 records the decision and the stricter interim rule (`uninformative` with no
   record).
4. **GATE-REFLEX-11 names two claims** ("Known answer; Fidelity"), against the rule of one claim per gate. It
   is recorded with the promotion clause (Fidelity) governing and the cart as its known-answer precondition
   (N22c-33, N22c-49).
5. **GATE-REFLEX-01's M1 set names the humanoid**, whose baseline class (biped balance) is Frontier with its
   route seeded at M3 (BD-247) behind a pilot. BD-118 and N22c-01 record the humanoid's M1 row with whichever
   of the three outcomes holds (expected: `blocked: route-not-built`), never as a pass by relabelling.
6. **§22.22 names no pilot** although the lane is Frontier. N22c-30 builds one from the subsection's stated
   hypothesis (a planner of 3,072 steps per tick completes reference tasks for some classes) and from the
   second clause of KILL-MPCRECORD.
7. **The upstream change of §22.20 has no entry in Appendix F.** N22c-22 carries it as an `upstream` bead with
   interim posture and switch-over rule, and asks the lead for an identifier.
8. **PO-SHIELD-04 and GATE-SHIELD-07 refer to a theorem card that the card list does not contain.** The
   continuous-time identity is tested on the host in N22c-03, and N22c-46 fails its own step visibly if the card
   is missing; the card itself is a need from the proofs unit.
9. **The worked energy card rounds a bound down.** 8·9.81·0.040·0.08 = 0.2511 J is printed as 0.25 J, and the
   `.fenergy` sketch carries `bound_j: "0.25"`. As arithmetic "to the printed digits" it is reproduced; as a
   bound it must be rounded outward (0.2512 J, total 1.4912 J, longest cutoff delay 50.44 ms). N22c-10 logs both
   and states the issue; the tier's owner (BD-337, U22b) should fix the card or its tolerance.
10. **Interpretations labelled DESIGN in the beads**, where the text gives one line: the per-class rule of
    KILL-BASELINE's fallback (N22c-14); the generalized closed forms and the Arm 1 trigger for drawn speeds in
    the pre-slip fixture (N22c-27); the two "admitting changes" algorithm (N22c-28); the deadline-miss draw in
    the latency model (N22c-31); "contraction in the energy norm" as a rational certificate (P, T) (N22c-44); the
    distinction between the outcome `not-synthesizable` and refusal FROB-REF-421 (BD-118, N22-REF); the default
    damping gain at half the no-reversal bound (N22c-44); decision rules proposed for OPEN-REFLEX-1, -3, -4 and
    OPEN-SHIELD-1 (N22c-50, -52, -53, -55).
11. **Bead count.** 60 beads is above the typical range. The unit holds an unusually enumerable share of the
    chapter: 12 kill gates, 13 pilots, 8 open questions and six late subsections each with its own gate; the
    charter asks for one bead per kill gate, per pilot and per blocking open question.
