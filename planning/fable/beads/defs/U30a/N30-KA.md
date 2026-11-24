---
key: "N30-KA"
title: "Ground known-answer registry: KA-721 to KA-736, the synthetic rehearsal's truths with tolerances, tiers and controls"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "ground", "frob-ground", "frob-conformance", "registry", "test", "frontier", "review-notes-open"]
parent: "WP-68"
deps: ["BD-025", "N38a-12", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N30a-02", "N30a-11", "N30a-13", "N30a-14", "N30a-15", "N30a-19", "N30a-24", "N30a-26", "N30a-33", "N30a-35", "N30a-36", "N30a-37", "N30a-39", "N30a-41", "N30a-42", "N30a-44", "N30a-45", "N30a-46", "N30a-47", "N30a-48", "N30b-29", "N30b-30", "N30b-31", "BD-139", "BD-388", "N38b-34"]
gates: []
plan_refs: ["Appendix M.29", "§30.2", "§30.7", "§30.8", "§30.9", "§30.11", "§30.12", "§30.13", "§30.14", "§30.21", "§30.22", "§30.24", "§38.6"]
---
## Background

Ground (crate `frob-ground`, layer 5) is the subsystem that compares simulation with measured reality. Nothing in
the project has been compared with a physical robot, so the correctness of Ground's machinery is decided without
real data, on a **synthetic rehearsal**: a truth simulator (the referee tier with planted parameters and one planted
difference of model form) emits logs through the sensor models with planted clock skew and offset, and Ground must
recover what was planted. Everything the truth simulator produces carries `origin = "synthetic"`; recovering planted
values shows that a bounded loop is sound and nothing more, and the Reality claim stays `no-claim`.

A known answer (KA) is a case whose truth is known before the code runs: an analytic value, an arithmetic identity
or a planted parameter. This registry is the single list of the sixteen known answers of the Ground chapter, plus
one late answer stated with the gate GATE-GROUND-11. Each mechanism bead cites the rows it implements and repeats
the values it needs; this bead exists so that the rows are enumerable, so that each has a negative control, and so
that the conformance harness can assert that every row has a test that names it. A gate that nothing can fail
validates nothing: where the plan states no negative control, this registry proposes one (marked DESIGN) so that
every row has a planted defect its test must catch.

Tier names: G0 laws and self-tests; G1 known answers. Execution sites: `host` (exact or double-precision arithmetic
in a test binary), `referee` (the trusted CPU tier, here driven by the synthetic truth simulator), `daemon` (`frobd`).

## Scope

In: the sixteen rows KA-721 to KA-736 as `[[ka]]` records of the suite's one known-answer file,
`crates/frob-conformance/registry/known_answers.toml` (schema and loader: N38a-12), with case, truth, tolerance, tier,
sites and the negative control's text; the late row of GATE-GROUND-11 and its block request in
`crates/frob-conformance/registry/allocations.toml` (N38a-40); one derivation file per row; the supporting
arithmetic the chapter states without a KA number, recomputed by this bead's test; the completeness test and a
registry-only end-to-end script. Out: the implementations (each row names its bead); the `NC-` records of the
controls, which the seeding tool of N38b-05 issues from the control texts entered here (this bead writes no control
record); the runner entries that bind each row (entered by the carrying and gate-script beads, N38a-13); the
counterexample registry `N30-CX` and the refusal registry `N30-REF` (written by the unit that owns the chapter's
second half).

## Technical approach

**Storage (DESIGN correction, request S12 of round 2).** An earlier draft kept a chapter file
`crates/frob-conformance/registry/known_answers/ground.toml` with fields of its own (`ka`, `section`, `case`,
`truth`, `tolerance`, `tier`, `site`, `negative_control`, `test`, `bead`, `pending`). N38a-12 allows one physical
file and refuses any other `known_answers*.toml`, or a `.toml` under a `known_answers` directory, with
`ka-second-file`, and refuses unknown keys. The rows are therefore `[[ka]]` records of `known_answers.toml`, field
for field N38a-12's: `ka` → `id`; `section` → `section` with `owner` (below); `site` → `sites` (the table's
"referee (synthetic)" is site `referee` with the rehearsal's `synthetic` origin asserted by the test, not a site of
its own); `negative_control` → `control.text`, whose `nc` the seeding tool fills; `fixture` one identity per
physical case, named in the derivation file, with `same_as` empty unless the carrying bead builds two rows on one
case (KA-728's control and KA-736 both use the shifted regime's 16 of 20: if N30a-36 and N30a-37 build it as one
fixture, KA-736 carries `same_as = "KA-728"`); `derivation` = `docs/derivations/<fixture>.md`; `label` `DESIGN`
for the arithmetic rows and for the planted-parameter rows (the truth is what was planted), `FACT` for none;
`printed` empty (every value recomputes, see the unit tests). The columns `test` and `bead` ("Carried by") are
never fields: each row's derivation file lists in its front matter the KA identifier, the test that checks it
(crate, path and name) and the owning bead.

**Where stated, owner and due.** `section` is Appendix M.29's "Where stated", with §30.21 (the rehearsal table)
for every row except KA-735 and KA-736: §30.7 for KA-721 and KA-725; §30.8 for KA-722 and KA-723; §30.9 for KA-724
and KA-727; §30.11 for KA-726, KA-732 and KA-734; §30.12 for KA-728, KA-729 and KA-736; §30.13 for KA-730 and
KA-731; §30.14 for KA-733; §30.2 for KA-735; §30.24 for the late row. `owner` is the gate whose script binds the
row: GATE-GROUND-01 for KA-735; GATE-GROUND-02 for KA-721 and KA-725; GATE-GROUND-03 for KA-722 and KA-723;
GATE-GROUND-04 for KA-724, KA-726 and KA-727; GATE-GROUND-05 for KA-732 and KA-734; GATE-GROUND-06 for KA-728,
KA-729 and KA-736; GATE-GROUND-07 for KA-730 and KA-731; GATE-GROUND-08 for KA-733; GATE-GROUND-11 for the late row.
`due` is M1 for KA-735 (GATE-GROUND-01 is recorded at M1 and at every exit; its carriers N30a-02 and BD-139 are M1
beads) and M5 for every other row.

**Milestone of this bead (placement correction).** This bead was placed at M5 with the rest of Ground. KA-735 is
due at M1 and cited from M1 by N30a-02 and BD-139, so a registry entered at M5 would leave GATE-GROUND-01's known
answer unregistered for four milestones, and the rule is that a registry never waits on its mechanisms. The
registry is data and depends only on M0 beads, so it is placed at M1 (inside WP-68, an M1 work package). The
fifteen rows due at M5 are loaded from M1 and reported `not due` until then.

**Rows before their carriers (no `pending` field).** The registry is entered before the mechanisms. An earlier
draft marked such a row `pending = "<bead key>"`, a key the schema does not have. Nothing is stored instead:
the loader computes each row's eligibility (derivation, tolerance, registered control). The runner of N38a-13
reports a row that is due and has no runner entry `ka:<id>@<site>` as `unbound`, which fails the gate. Before
its `due` milestone a row is listed `not due` with the bead that will bind it.
`ground_ka_registry_complete` reads the derivation front matter and checks three things: each row names an
existing bead key and a test path; a row whose due milestone has been reached is bound; and the count of rows
not yet bound is printed. No row may be unbound when the M5 exit review runs. Binding map: each row is bound by
the gate-script bead of its gate, which depends on this registry, adds the row's runner entry, names its test in
the derivation front matter and logs the row by KA number:
- KA-721 and KA-725: N30a-14.
- KA-722 and KA-723: N30a-19.
- KA-724, KA-726 and KA-727: N30a-26.
- KA-728, KA-729 and KA-736: N30a-39.
- KA-730 and KA-731: N30a-44.
- KA-732 and KA-734: N30a-35.
- KA-733: N30a-48.
- KA-735: N30a-02 and BD-139 at M1. It never waits.
- The late row: N30b-30.

**The rows.**

| KA | Case | Truth (formula or value) | Tolerance | Tier · site | Negative control | Carried by |
|---|---|---|---|---|---|---|
| KA-721 | Clock recovery. Planted skew s = 100 ppm, offset a = 3.7 ms, mark jitter σ_m = 50 µs, two marks 10 s apart | Skew uncertainty σ_s = √2·σ_m/(t₂ − t₁) = 7.07 ppm; recovered skew within 3·σ_s = 21 ppm of 100 ppm; aligned instants cover the truth; in at least 99% of 1,000 resamples (GATE-GROUND-02) | as stated | G1 · referee (synthetic) | One pulse only: refused (FROB-REF-876). An offset fitted to the scored event: detected | N30a-11, N30a-13, N30a-14 |
| KA-722 | The eight structural aliases (armature against link inertia; Coulomb friction against a bias torque; viscous friction against back-EMF damping; torque constant against the scale of inertia and friction; delay against clock offset; friction coefficient against the torque limit; static friction with no dwell at zero velocity; contact stiffness against damping) | Each flagged, 8 of 8, with the identified combination named (GATE-GROUND-03) | exact | G1 · host | A fit request on each aliased set is refused (FROB-REF-879) | N30a-15, BD-390, N30a-19 |
| KA-723 | Friction against the torque limit: drawbar force F = min(k_t·I/r, μ·N), k_t = 0.2 N·m/A, r = 0.06 m, μ ∈ {0.4, 0.8} | At 1 A under 30 N and 2 A under 40 N: F = 3.33 N and 6.67 N for both μ (torque-limited: k_t·I/r = 3.333, 6.667 N against μ·N = 12, 16, 24, 32 N). At 3 A under 10 N: k_t·I/r = 10 N against μ·N = 4 N and 8 N, so F = 4 N and 8 N: separated | to the printed digits | G1 · host | The torque-limited data alone yields no μ | N30a-15, BD-390 |
| KA-724 | Actuator recovery of J_a, b, τ_c, τ_s, v_s and d on a single-joint synthetic rig | Each within 10% of the planted value; delay within one grid step | 10%; one grid step | G1 · referee (synthetic) | Unidirectional data: τ_c is refused | N30a-21 to N30a-23, N30a-26 |
| KA-725 | Delay against offset: actuator delay 8 ms, clock offset −8 ms, no shared hardware mark | Reported as "delay plus offset = 0" | exact | G1 · host | A card delay update is refused | N30a-13, N30a-15 |
| KA-726 | Omitted law: the truth has a Stribeck term the fit class lacks | The candidate is rejected at validation on sealed data | exact (verdict) | G1 · referee (synthetic) | The same fit scored on its own calibration data passes (which shows why sealing matters) | N30a-28, N30a-32, N30a-26 |
| KA-727 | Wrong reason: speed matched by a compensating error in k_t and friction | The energy cross-check marks the fit `wrong-reason` | exact (label) | G1 · referee (synthetic) | None stated. DESIGN: the same fit with the energy check disabled is offered for admission and the test must fail | N30a-24 |
| KA-728 | Band coverage: exchangeable residuals, n = 20, target 0.90, 1,000 resamples | Quantile index ⌈21·0.9⌉ = 19; realized coverage ~ Beta(19, 2): mean 19/21 = 0.905, standard deviation 0.063, P(coverage < 0.80) = 0.8¹⁹·(20·0.2 + 0.8) = 0.069 | mean within 0.01 of 19/21; the fraction below 0.80 within 0.03 of 0.069 (GATE-GROUND-06) | G0 · host | Pooled shifted regime: 16 of 20 | N30a-36, N30a-39 |
| KA-729 | Band refusal: an unbuilt design; a cell with 8 points; a new unit with 3 units calibrated | Refused in each case (FROB-REF-895; 889; 889) | exact (verdict) | G1 · host | None stated. DESIGN: a build with the minimum-count guard removed issues a band for the 8-point cell and the test must fail | N30a-37, N30a-39 |
| KA-730 | Drift: friction raised by 30% after episode k | The calibration is retired within 20 further episodes (in at least 90% of resamples, GATE-GROUND-07) | as stated | G0 · referee (synthetic) | Null runs: false retirement at most 5% over 10⁴ runs | N30a-41, N30a-42, N30a-44 |
| KA-731 | Miss-monitor arithmetic at n = 20, p₀ = 0.22, p₁ = 0.5 | A miss multiplies the evidence by p₁/p₀ = 2.27, a cover by (1 − p₁)/(1 − p₀) = 0.641; four straight misses give 26.7 (retire, threshold 20); three give 11.7 (do not) | to the printed digits | G0 · host | None stated. DESIGN: a monitor that uses p₀ = 0.10 (the nominal miss rate) retires after two misses (e = 25) and must fail the null-run bound | N30a-42 |
| KA-732 | Reachability: zero failures in 20 real trials | One-sided 95% upper bound on the failure probability 1 − 0.05^(1/20) = 13.9%; a claim of "under 5%" is unreachable below 59 failure-free trials | to the printed digits | G0 · host | A registered claim of "under 5%" at n = 20 is refused at registration | N30a-33 |
| KA-733 | As-built registration: a planted rotation and translation; a 0.3 mm wall deviation | Recovered inside the stated uncertainty | inside stated uncertainty | G1 · host | Two fiducials: refused (FROB-REF-812). Scale 25.4: unit suspicion raised | N30a-45, N30a-46, N30a-48 |
| KA-734 | Protocol: read before freeze; refit without a new version; holdout reuse | Each refused (FROB-REF-883, 883, 886) | exact (verdict) | G1 · daemon | None stated. DESIGN: a store build with the sealed-area guard disabled must make the test fail | N30a-31, N30a-34, N30a-35 |
| KA-735 | The `reality:` line (GATE-GROUND-01) | Printed on every report; `no-claim` with no dataset; a negative battery of 50 planted misuses is refused in full | exact | G1 · host | The 50 planted misuses | N30a-02 (type and grammar); BD-139 (every report, the battery); N30b-01 |
| KA-736 | Pooled conformal calibration at a 90% target across two regimes | 20 of 20 covered in one regime and 16 of 20 in the shifted one: 36 of 40 = 90% overall, 80% where it matters | exact (count) | G0 · host | Pooling across cells (refused, FROB-REF-814) | N30a-37 |

**The controls owed and the second records.** Appendix M counts 16 rows for this chapter, 4 of them with "none
stated" as the negative control (KA-727, -729, -731, -734). The DESIGN controls above are the owner's and are
first; N38b-34 links its proposals for KA-727, KA-729 and KA-734 to them as the same defect and adds a second
record for KA-731 (the likelihood ratio inverted, p₀/p₁, giving 0.037 and 0.085, so that the alarm never fires).
The seeding tool of N38b-05 issues the records from these texts. The DESIGN controls of the refusal rows KA-729 and
KA-734 are check-removed mutations, not restated refusals, which is the form N38a-12's rule 3 asks for.

**The late row of GATE-GROUND-11 (§30.24).** GATE-GROUND-11 is a known-answer gate whose values are printed in its
register row and have no KA number. An earlier draft listed it as an auxiliary row `ground.aux.alias9`, an
identifier the loader does not accept. It is a `[[ka]]` row:
- **Number.** This bead files one block request in `allocations.toml`: `registry = "ka"`, `count = 1`,
  `owner = "N30-KA"`, `section = "§30.24"`. Nobody types `first`.
- **Tag.** Until the block is issued the row carries `slug = "ground-alias9"` and is cited by that tag. When the
  block is issued the number replaces the tag in one commit, in the registry, the test names and the derivation
  file. A tag still cited at the row's due milestone (M5) fails N38a-40's check `provisional-tag-past-due`.
- **Case.** The exposure delay against the mount translation on a straight pass. The reading is L − v·(t + δ) − b,
  with δ = 20 ms and b = 0 against δ = 0 and b = 40 mm, at v = +2 m/s, then reversed at −2 m/s, then at +1 m/s.
- **Truth.** The two explanations give bit-equal samples on the +2 m/s pass, and the fit reports "delay times
  speed plus offset = 40 mm" with no card update. With the reversed pass added, both the 20 ms delay and the
  planted 40 mm offset are recovered inside their intervals. The predicted separation is 80 mm, and 20 mm at
  +1 m/s.
- **Tolerance.** `gate` (GATE-GROUND-11): the samples are bit-equal, the predicted separation is 0.08 m to 10⁻⁹,
  and the recovery is held to the gate's interval rule.
- **Tier and sites.** Tier G1 at site `referee`, on the rehearsal of BD-388, whose `synthetic` origin the test
  asserts. **Due:** M5.
- **Control.** N30b-29's: a fit that splits the combination by a prior, reporting a delay from one pass (plant
  `N30b-29:cx-<nnn>`, the plant of CX-G-01 in N30-CX, `<nnn>` being the number issued to that tag, so the
  seeding tool of N38b-05 reuses CX-G-01's control and allocates no second one); and a detector that compares
  speeds instead of axial velocities, so that a reversed pass at the same speed stays flagged (no counterexample
  names this plant, so its control is seeded from this row).
- **Carried by.** N30b-29, the mechanism. N30b-30 is the gate script that binds the row.

**Supporting arithmetic stated in the chapter without a KA number.** These are derived values, not registry rows:
they have no KA number and no control. Each is recomputed from its formula by `ground_ka_arithmetic` below and
asserted by a host test in the bead named. The DESIGN proposal of an earlier draft to make them rows
`ground.aux.*` is withdrawn, because those identifiers are not KA numbers; the one value set with a known-answer
gate is the late row above.

| Value | Derivation | Bead |
|---|---|---|
| Two clocks 100 ppm apart that coincide at the start differ by 0.8 ms after 8 s | 100·10⁻⁶·8 s | N30a-11 |
| σ_s = 7.1 ppm at σ_m = 50 µs, marks 10 s apart | √2·50·10⁻⁶/10 | N30a-11 |
| σ_t(t)² = σ_m²·[(t₂ − t)² + (t − t₁)²]/(t₂ − t₁)²: σ_m² at either mark, σ_m²/2 at the midpoint, growing quadratically outside | Linear interpolation of two independent marks | N30a-11, N30a-12 |
| Crossing predicted at 4.212 s, logged at 4.180 s: fitting the offset to the event reports error 0 and offset 32 ms | 4.212 − 4.180 | N30a-13 |
| Finite band needs n ≥ (1 − α)/α: 9 at 90%, 19 at 95%, 99 at 99%; at n = 20 a 95% band is the largest residual and any level above 20/21 = 95.2% is unbounded | ⌈(n+1)·(1−α)⌉ ≤ n | N30a-36 |
| n = 200, target 0.90: coverage ~ Beta(181, 20), mean 0.900, standard deviation 0.021, P(coverage < 0.80) below 0.001 | l = ⌊201·0.1⌋ = 20 | N30a-36 |
| Rank-monitor slack δ = max(1/√n, 0.02): 0.224 at n = 20, 0.071 at n = 200 | — | N30a-41 |
| Miss-monitor null rate p₀(n), the 95th percentile of Beta(l, n + 1 − l): about 0.22 at n = 20 (Beta(2, 19)), about 0.14 at n = 200 (Beta(20, 181)) | 1 − (1 − x)¹⁹·(1 + 19·x) = 0.95 at x ≈ 0.216 | N30a-42 |
| Zero failures in n trials: upper bound 13.9% at n = 20, 9.5% at n = 30, 4.95% at n = 59 | 1 − 0.05^(1/n) | N30a-33 |
| Interval score S = (u − l) + (2/α)·(l − y)₊ + (2/α)·(y − u)₊: for [4.05, 4.42] at level 0.90, y = 4.20 gives 0.37 and y = 4.50 gives 0.37 + 20·0.08 = 1.97 | α = 0.10 | N30a-32 |
| Ceiling of paired simulation on real-trial count: about 1.3 | 1/(1 − 0.20) to 1/(1 − 0.25) | N30a-38 |
| The late row's arithmetic: at v = +2 m/s, δ = 20 ms with b = 0 and δ = 0 with b = 40 mm give bit-equal samples (v·δ + b = 40 mm); reversed at −2 m/s they differ by 80 mm, at +1 m/s by 20 mm | 2·0.02 = 0.04 m; 0.04 − (−0.04); 0.04 − 0.02 | N30b-29, N30b-30 |

## Acceptance criteria

- [ ] `known_answers.toml` holds sixteen `[[ka]]` records, KA-721 to KA-736, and the late row, held under its tag
      until the block is issued and under its number afterwards. Each record has every field of N38a-12, with
      the section, owner and due stated above, and the loader validates the file. No file
      `registry/known_answers/ground.toml`, nor any other file under a `known_answers` directory, exists.
- [ ] The block request for one `ka` number (owner N30-KA, §30.24) is in `allocations.toml`. Once the block is
      issued, no `ground-alias9` tag remains in the registry, a test name or a derivation file.
- [ ] Every row carries its control text: the plan's control, or the DESIGN control above where the plan states
      none. After N38b-05's seeding tool is re-run, each row's `control.nc` resolves to records of `controls.toml`.
      The four DESIGN controls are the first record of their rows, and N38b-34's second record for KA-731 is
      present.
- [ ] Each row's derivation file exists and lists in its front matter the KA identifier, the test path and the
      owning bead. `ground_ka_registry_complete` passes. It fails when a row's derivation file is deleted, when a
      named bead key does not exist, and when a row past its due milestone is unbound. It prints the count of rows
      not yet bound, and that count is 0 at the M5 exit review.
- [ ] KA-735 is loaded and eligible at M1, when GATE-GROUND-01 is first recorded.
- [ ] Every host-site arithmetic row (KA-723, -725, -728, -729, -731, -732, -736) and every supporting value is
      asserted by a test that needs no simulator and runs in under one second.
- [ ] Every referee-site row runs on data whose manifest says `origin = "synthetic"`, and every envelope it
      produces prints `reality: synthetic` or `reality: no-claim`, never another status.

## Test plan

### Unit tests

`crates/frob-conformance/tests/ground_ka_registry.rs` (STD-TEST's parser `frob_core::testkit::registry`, no
second reader):
- `ground_ka_rows_are_sixteen`: KA-721 to KA-736 present, with no gaps and no duplicates, each with every field
  of N38a-12, the section, owner and due of the table above, and a tolerance other than `to-set`.
- `ground_ka_registry_complete`: the checks of the acceptance criteria, with three planted scratch copies: a
  derivation file deleted, a bead key misspelt, and a row due M1 unbound.
- `ground_ka_no_chapter_file`: a scratch copy that writes the rows to `registry/known_answers/ground.toml`, or
  adds a key `pending` or `test` to a row, is refused (`ka-second-file`; unknown key).
- `ground_ka_late_row_by_tag_or_block`: the late row carries the tag `ground-alias9` and a matching block request.
  A scratch copy at M5 with the tag left in a test name fails `provisional-tag-past-due`.
- `ground_ka_negative_controls_seeded`: 16 rows with control texts, 12 of them stated in the plan and 4 DESIGN.
  After seeding, every `nc` resolves, and the DESIGN controls of KA-729 and KA-734 are seeded as mutations; the
  late row's `nc` lists CX-G-01's control (plant `N30b-29:cx-<nnn>`) and one control seeded from the row for the
  speed-comparing detector, and no second record of the prior-split fit.
- `ground_ka_arithmetic`: recomputes every row value in exact rationals or double precision:
  - 19/21; 0.8¹⁹·4.8 = 0.0692 within 10⁻⁴; the Beta(19, 2) standard deviation √(38/(441·22)) = 0.0626.
  - (0.5/0.22)⁴ = 26.68 and (0.5/0.22)³ = 11.74; (1 − 0.5)/(1 − 0.22) = 0.641; 5² = 25 for the KA-731 control.
  - 1 − 0.05^(1/20) = 0.1391; 1 − 0.05^(1/59) = 0.0495 and 1 − 0.05^(1/58) = 0.0503.
  - √2·50·10⁻⁶/10 = 7.07·10⁻⁶; 0.2/0.06 = 3.333; min(10, 4) = 4 and min(10, 8) = 8; 36/40 = 0.9.
- `ground_ka_supporting_values`: recomputes every value of the supporting table:
  - 100·10⁻⁶·8 = 8·10⁻⁴ s; 4.212 − 4.180 = 0.032 s.
  - 9, 19 and 99 from ⌈(n + 1)(1 − α)⌉ ≤ n, and 20/21 = 0.952.
  - The Beta(181, 20) mean 0.9005 and standard deviation 0.0211; its CDF at 0.80 is below 0.001 (4.5·10⁻⁵ by
    the regularized incomplete beta).
  - 1/√20 = 0.224 and 1/√200 = 0.071.
  - The 95th percentiles 0.216 of Beta(2, 19) and 0.136 of Beta(20, 181).
  - 0.37 and 1.97; 1.25 and 1.333; and 0.04, 0.08 and 0.02 m.

### End-to-end script and logging

`scripts/e2e/ground_ka_registry.sh` checks the registry from the user's side. It does not check the mechanisms,
and there is no `frob gauntlet` or `frob conformance` command (ADR-GAUNT-01).
- **Header and log.** Its header is `bead=N30-KA milestone=M1 labels=ground,registry gates=`. It sources the
  STD-E2E helper and writes NDJSON to `target/e2e-logs/ground_ka_registry/<utc>.ndjson` with a transcript.
- **Steps.** It runs `cargo test -p frob-conformance --test registry_check` and `cargo test -p frob-conformance
  --test ground_ka_registry`. It then runs `FROB_TEST_SEED=<s> cargo test -p frob-conformance --test
  g1_known_answers -- KA-735`, and the same command with `--features gauntlet-mutations` for the control. It logs
  the row's outcome: `unbound` before N30a-02 and BD-139 add its runner entry, then `pass` with the control
  caught. The script never waits on a carrier, and fails only on `fail` or `control-missed`. The other rows are
  run only to log `not due` before M5.
- **Metric lines.** It logs:
  - the 16 rows and the late row by tag or number, with the block's `first`, `last` and `status`;
  - the rows per computed eligibility state;
  - the rows bound and not yet bound, with their binding bead;
  - the `control.nc` count per row;
  - the digest of `known_answers.toml`.
- **Planted scratch defects.** On scratch copies it plants KA-731's control text deleted, a row moved into
  `registry/known_answers/ground.toml`, and a `pending` key, and requires each to be caught.
- **Line fields.** Every line carries `ts`, `script`, `bead`, `step`, `event`, the exact command, the exit code
  and the wall time. On a failure it also logs the first differing value and the scratch path. The `summary`
  gives the assertions passed and failed and `gates: []`.

The scripts that assert the rows themselves belong to the carrying beads:
- `scripts/e2e/ground_clock.sh` (N30a-14), `ground_excite.sh` (N30a-19) and `ground_identify.sh` (N30a-26);
- `ground_protocol.sh` (N30a-35), `ground_band.sh` (N30a-39), `ground_drift.sh` (N30a-44) and
  `ground_asbuilt.sh` (N30a-48);
- `ground_alias9.sh` (N30b-30) for the late row, and `ground_rehearsal.sh` (N30b-31) for every row on the
  rehearsal.

Each of these logs, for each row it asserts, an `assert` line with the KA number, the measured value with its
unit, the truth, the tolerance and the outcome, and a `metric` line for each control with the planted defect and
whether it was caught.

### Known answers and negative controls

This bead is the registry. Its own negative controls are on the registry check: a deleted derivation file, a
second chapter file, an unknown key, a row past its due milestone that is unbound, and a late tag past its due
milestone each fail. Metamorphic relations that apply across rows:
- Scaling every residual of a calibration set by a positive constant scales q̂ by the same constant and leaves
  coverage counts unchanged (KA-728).
- Permuting calibration episodes leaves q̂ unchanged (KA-728) and leaves the miss monitor's value unchanged
  (KA-731).
- Re-expressing clock marks in another time unit leaves skew unchanged (KA-721).

Determinism lanes:
- Host rows are class D0 or exact in double precision. They are run twice, in debug and release, on both
  development machines.
- Referee rows are run twice on one machine with a pinned seed and compared bit for bit, and across machines
  inside the recorded envelope. The ensemble lane used by KA-724 makes no cross-machine bit claim.
- The registry and its listing are byte-identical across runs.

## Considerations

- The four DESIGN controls (KA-727, -729, -731, -734) should be confirmed by the Ground lead before the gates
  that cite them are recorded. They stay first, and N38b-34's records are linked to them or follow them as second
  records.
- GATE-GROUND-11 (the exposure-delay and mount-translation alias, §30.24) is now a late row with a block of one
  number. Its mechanism and script (N30b-29, N30b-30) cite the row by its tag until the number is issued.
- The gate register (§40) places GATE-GROUND-03 at M5 with the note that it may be recorded at M4. KA-722 and
  KA-723 therefore have host tests that can run as soon as the analysis exists.
- Placing this bead at M1 adds no M1 work beyond data and arithmetic. Every mechanism stays at M5, and the rows
  due at M5 cannot be counted as passing before then.
- Risk: a known answer on synthetic data can be mistaken for evidence about reality. Every row's test asserts the
  `synthetic` origin, and no row is ever cited for the Reality claim (invariant INV-GROUND-06: data of origin
  `synthetic` never yields a status other than `synthetic`).

## References

Appendix M.29 (KA-721 to KA-736); §30.2 (GATE-GROUND-01); §30.21 (the rehearsal table); §30.22 (gates
GATE-GROUND-01 to -08); §30.24 (GATE-GROUND-11); §30.7, §30.8, §30.9, §30.11, §30.12, §30.13, §30.14 (the
arithmetic); §38.6; INV-GROUND-06; PO-GROUND-01, -02, -03.
