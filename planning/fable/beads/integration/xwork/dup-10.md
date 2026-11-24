# Cross-group overlap packet dup-10

36 pairs in 21 clusters. Each cluster is a set of beads connected by reported overlaps; decide a cluster as a whole.

## Cluster of 6 beads

- `BD-109` (U14b, M1, feature) frob-design: the first archetypes (quad-mammal, humanoid, diff-drive, mecanum) and verb design.templates  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14b/BD-109.md`
- `N13-21` (U13, M1, task) e2e: analyze.symmetry on the first four archetypes, the forward-lidar case and M1 planted asymmetries  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U13/N13-21.md`
- `ZOO-diff-drive` (UEG, M1, task) zoo/diff-drive.fbot: archetype 7, two driven wheels and a caster: growth vector (2, 3), chord step, threshold  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-diff-drive.md`
- `ZOO-humanoid` (UEG, M1, task) zoo/humanoid.fbot: archetype 6, humanoid with arms and waist: 29 actuated, n_v = 35, C₂, coupled transmissions  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-humanoid.md`
- `ZOO-mecanum` (UEG, M1, task) zoo/mecanum.fbot: archetype 9, four mecanum wheels: C₂ × C₂, growth vector (3), parallel rollers are rank 2  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-mecanum.md`
- `ZOO-quad-mammal` (UEG, M1, task) zoo/quad-mammal.fbot: archetype 1, dog-like quadruped: C₂ × C₂, 27 contact strata, Γ_obs = C₂ with a lidar  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/ZOO-quad-mammal.md`

- **pair** `BD-109` / `ZOO-diff-drive`
  - reported by GEG: same closed forms (near-straight chord translation (0.8342, 0.5514) m; raised-threshold lever arms 36.0 and 40.7 mm, KA-265) in the template's tests and the expectation file
- **pair** `BD-109` / `ZOO-humanoid`
  - reported by GEG: same rows (the ±15% mass window of preset `@g1`, 28.339 to 38.341 kg around 33.34014202 kg; the reduced 31.96314202 kg) in the template's tests and in the zoo expectation file
- **pair** `BD-109` / `ZOO-mecanum`
  - reported by GEG: same roller-rank and FROB-REF-118 rows in the template's tests and the expectation file
- **pair** `BD-109` / `ZOO-quad-mammal`
  - reported by GEG: same rows (C₂ × C₂, 27 contact strata, the trusted-declaration control KA-276, CX-133) in the template's tests and in the zoo expectation file; each side has the template with presets or the pinned instance with its DesignId
- **pair** `N13-21` / `ZOO-quad-mammal`
  - reported by GEG: Γ_mech and Γ_obs of the first four archetypes with the forward-lidar case are asserted by the symmetry end-to-end bead and by the four M1 zoo expectation files (N13-22 does the same for all eighteen at M3)

## Cluster of 5 beads

- `BET-14` (U00, M7, task) Decide BET-14: the mechanisms need this engine (the composed-incumbent tie rule and the recovery ratio)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-14.md`
- `BET-19` (U00, M4, task) Decide BET-19: mission queries over a design family beat point-by-point evaluation and survive amendment  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-19.md`
- `N25a-18` (U25a, M3, task) Decision KILL-AUGUR-TIE at M3: ship the conventional robust program behind the verbs where it ties, per stratum  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-18.md`
- `N25b-04` (U25b, M1, task) Kill-gate decision KILL-AUGUR-FALSECERT, standing from M1: cause analysis and the false-certificate drill  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25b/N25b-04.md`
- `N39b-07` (U39b, M3, task) Kill-gate decision KILL-TENFOLD-TIE at M3 and M7: a tie or a recovery ratio of 0.8 ships the layer  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-07.md`

- **pair** `BET-14` / `N25a-18`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-AUGUR-TIE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-AUGUR-TIE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-14` / `N39b-07`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-TENFOLD-TIE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-TENFOLD-TIE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-19` / `N25a-18`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-AUGUR-TIE. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-AUGUR-TIE decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
- **pair** `BET-19` / `N25b-04`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-AUGUR-FALSECERT. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-AUGUR-FALSECERT decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.

## Cluster of 4 beads

- `BD-132` (U12, M1, feature) frob-cert: checkers for the seven mission proof kinds of the first slice (GATE-LEMMA-03)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/BD-132.md`
- `CARD-T-30` (U12, M1, task) Theorem card T-30: an indistinguishability witness is valid (identical histories, disjoint continuations)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U12/CARD-T-30.md`
- `N20a-37` (U20a, M1, feature) frob-sense: the alias witness object and its independent checker, with the two-line theorem (PO-SENS-07)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U20a/N20a-37.md`
- `N25e-29` (U25e, M1, task) frob-cert and frob-lemma: the law object of an lp-primal certificate and its checking rules  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25e/N25e-29.md`

- **pair** `BD-132` / `N20a-37`
  - reported by G12: the alias-pair checker: BD-132 builds the rule in frob-cert, N20a-37 specifies an independent checker of the same witness object; one of them should own the frob-cert rule (BD-132) and the other cite it
- **pair** `BD-132` / `N25e-29`
  - reported by G25cde: overlap: BD-132 already checks the multilinear-vertex specimen of the stance family; N25e-29 specifies the law-object rules of lp-primal, which the Lemma unit may prefer to own
- **pair** `CARD-T-30` / `N20a-37`
  - reported by G12: N20a-37 carries the theorem statement and proofs of PO-SENS-07 'as cards'; CARD-T-30 is the instance card; the general statements are candidates decided in N12-52

## Cluster of 4 beads

- `N19b-02` (U19b, M3, task) frob-scene: asset licence manifests, the asset-source register, pack-time refusal FROB-REF-310, starter pack  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19b/N19b-02.md`
- `N37-18` (U37, M3, task) frob-verbs/frob-chronicle: licences at export: one export door, FROB-REF-733, attribution and dependency notices  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U37/N37-18.md`
- `NEG-17` (UEG, M3, task) scene asset source `frob-scene`: the project's own generators as default source; a pure starter pack  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-17.md`
- `VERBS-report` (UJL, M3, task) frob-verbs registry: cluster `report`, four rows, with parity, e2e and schema-snapshot checks  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/VERBS-report.md`

- **pair** `N19b-02` / `N37-18`
  - reported by G37: the two disagree on the code for a report or other non-pack export that carries a research-only asset: N19b-02 expects artifact.export and the report verbs to call its pack export check (FROB-REF-310); N37-18 sends every export through one door that refuses with FROB-REF-733 and accepts 310 only at pack time
- **pair** `N19b-02` / `NEG-17`
  - reported by GEG: the asset-source register assets/licences/sources.toml and the starter-pack check are in N19b-02 with all rows of G.2; NEG-17 to NEG-27 each own one row with its admission reading, import profile and tests (this pair: row `frob-scene` and the starter pack)
- **pair** `N37-18` / `VERBS-report`
  - reported by G37: the registry row of the report verbs lists FROB-REF-310 for a report that would export a research-only or private asset; N37-18 and N37-REF give that case FROB-REF-733 (licence) and BD-305 gives FROB-REF-731 (private)

## Cluster of 3 beads

- `BD-295` (U32a, M3, feature) frobd: persistent environments per device and shape class with slot-level episode dispatch; GATE-TETHER-09  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/BD-295.md`
- `N17a-09` (U17a, M2, feature) frob-phylum: slot-level admission into a running cohort: slot table, reset-time writes, budgeted mode  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-09.md`
- `N21b-21` (U21b, M3, task) frob-kernels: persistent environments with slot refill at masked resets (Spark side) and the per-job fallback  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U21b/N21b-21.md`

- **pair** `BD-295` / `N17a-09`
  - reported by G17: the pure slot-assignment function, the job cap and the 5% reserve are specified on the Phylum side and in the daemon
  - reported by G32: the pure slot-assignment function, the job cap and the 5% reserve are specified on the Phylum side and in the daemon; BD-295 depends on N17a-09
- **pair** `N17a-09` / `N21b-21`
  - reported by G17: refill at masked resets with the reserve and job-cap limits is also the device-side bead's scope

## Cluster of 3 beads

- `GB-11` (UEG, M1, task) card GB-11 (frob-actuate): when a winding runs away: equilibrium P/(G − s) and I_crit = √(G/(R₀·α))  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-11.md`
- `GB-12` (UEG, M1, task) card GB-12 (frob-actuate): how much torque is left above base speed: field weakening on both limits  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/GB-12.md`
- `N15a-26` (U15a, M1, task) e2e sinew_static_sizing: actuator.thermal and actuator.envelope on the worked cards, refusals 131 and 132  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U15a/N15a-26.md`

- **pair** `GB-11` / `N15a-26`
  - reported by GEG: the end-to-end script sinew_static_sizing runs actuator.thermal on this worked card (fixture thermal_worked.fcard: R₀ = 0.20 Ω, rise 164.7 K, i_crit 31.9 A) and checks every published number; NEG-02 runs the same card through the same verb
- **pair** `GB-12` / `N15a-26`
  - reported by GEG: the same script runs actuator.envelope on this worked card (fixture envelope_worked.fcard); NEG-02 does too

## Cluster of 3 beads

- `N16b-14` (U16b, M2, task) frob-dyn: the created-energy audit on the step, positive residual by contact mode, storage and consequences  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-14.md`
- `N16b-33` (U16b, M2, task) e2e: the created-energy audit and GATE-FID-06 through `frob fidelity energy` (`scripts/e2e/energy_audit.sh`)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16b/N16b-33.md`
- `N29a-22` (U29a, M2, feature) Created-energy audit: `energy-created`, `contact-active`, `cone-excess`; verb fidelity.energy; GATE-CHRON-08  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29a-22.md`

- **pair** `N16b-14` / `N29a-22`
  - reported by G16: both specify the created-energy audit (windowed rule, flags, consequences); N29a-22 owns the monitors and the verb fidelity.energy, N16b-14 the split by contact mode on the step and GATE-FID-06
  - reported by G29: the created-energy audit (windowed rule, flags, consequences) is in both; N29a-22 owns the monitors and fidelity.energy, N16b-14 the step-side split and depends on N29a-22
- **pair** `N16b-33` / `N29a-22`
  - reported by G29: both own a script named scripts/e2e/energy_audit.sh; one needs another name

## Cluster of 3 beads

- `N29a-24` (U29a, M2, task) GATE-TETHER-08 and device rows of GATE-CHRON-04 and -11: job protocol under seeded faults on devices; e2e  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29a-24.md`
- `N32a-18` (U32a, M2, task) e2e: the job protocol under seeded faults on a running daemon, from submit to receipt (GATE-TETHER-08)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-18.md`
- `N33c-17` (U33c, M2, task) INT-ASUP-02: job protocol under the lab runtime, 100 of 100 identical traces, planted lost-chunk bug found  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U33c/N33c-17.md`

- **pair** `N29a-24` / `N32a-18`
  - reported by G32: two end-to-end scripts measure GATE-TETHER-08 (the job protocol under seeded faults): job_protocol on a running daemon here, the device rows with GATE-CHRON-04 and -11 there; one should own the gate record and the other feed it
- **pair** `N29a-24` / `N33c-17`
  - reported by G29: the job protocol under the lab runtime at M2 is measured by both (GATE-CHRON-11 device row and INT-ASUP-02); dependency N29a-24 -> N33c-17 recorded

## Cluster of 2 beads

- `BD-092` (U29a, M1, feature) frob-chronicle: energy ledger on the referee: port lines, stored heat, braking modes, residual, first offender  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/BD-092.md`
- `N24a-37` (U24a, M1, feature) verb telemetry.energy: the energy ledger by line, joint, limb and phase, with its residual and audit flags  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U24a/N24a-37.md`

- **pair** `BD-092` / `N24a-37`
  - reported by G29: BD-092's script reads the ledger through telemetry.energy, which is N24a-37 and depends on BD-092; those script steps can run only after N24a-37

## Cluster of 2 beads

- `BD-231` (U41, M3, feature) frob-scene: rigid proxies: a rug as a thin free slab, a cord as a fixed ridge or free rod, rigid clutter  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U41/BD-231.md`
- `N19b-18` (U19b, M6, task) e2e GATE-HAB-08, proxy rows: catenary sag within 2% at 20 links; free-slab rug slides (castor case); cord cost  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19b/N19b-18.md`

- **pair** `BD-231` / `N19b-18`
  - reported by G41: the castor case of the free-slab rug (KA-440) is BD-231's seed condition at M3 and a row of GATE-HAB-08 in N19b-18 at M6; N19b-18 depends on BD-231

## Cluster of 2 beads

- `BD-399` (U19a, M5, feature) frob-scene: asset-populated interiors with licence manifests, inside the proxies' launch shape  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/BD-399.md`
- `NEG-18` (UEG, M5, task) scene asset source `infinigen-indoors` (BSD-3): offline external generator, never in the production graph  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-18.md`

- **pair** `BD-399` / `NEG-18`
  - reported by GEG: BD-399 restates the G.2 source table and admits assets through scene.import; NEG-18, NEG-19 and NEG-20 are the three default-pipeline sources with their admission readings and import profiles

## Cluster of 2 beads

- `BET-28` (U00, M5, task) Decide BET-28: calibration bands (conformal bands reach stated coverage within one calibrated family)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/BET-28.md`
- `N30b-18` (U30b, M5, task) Kill-gate decision KILL-BAND at M5: conformal reality bands, or no-claim with the raw residual table  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U30b/N30b-18.md`

- **pair** `BET-28` / `N30b-18`
  - reported by G00: overlap with distinct scope, no merge: both record the decision KILL-BAND. The kill-gate bead applies the trigger and writes the kill ADR; the bet bead reads that record, evaluates the bet's falsifier clauses and writes the bet verdict. The bet bead's acceptance line "the KILL-BAND decision recorded" means on record by the other bead. The dependency of the bet on this bead is in place.
  - reported by G30: overlap with distinct scope, as G00 recorded: the kill-gate bead decides KILL-BAND, the bet bead reads it

## Cluster of 2 beads

- `N00-15` (U00, M0, task) The claims-wording registry: thirteen differentiators, sixteen withdrawn claims, and the gate-conditioned lint  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U00/N00-15.md`
- `N39b-15` (U39b, M3, task) frob-conformance tenfold: the wording lint and the limits block: where the word ten-fold may appear  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-15.md`

- **pair** `N00-15` / `N39b-15`
  - reported by G00: both lint where the word ten-fold may appear: N00-15 through the gate-conditioned claims lint over the differentiators, N39b-15 with the limits block and the citation syntax of the report. N39b-15 registers its rules in the registry of N00-15; distinct scope

## Cluster of 2 beads

- `N10b-07` (U10b, M1, task) pilot PILOT-C-18: polarity on the mission slice's certificates under twelve scripted amendments  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U10b/N10b-07.md`
- `N28a-23` (U28a, M4, task) pilot: grade-3 reuse of verdicts: derivations, twelve scripted amendments, dry run on the frozen campaign  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28a/N28a-23.md`

- **pair** `N10b-07` / `N28a-23`
  - reported by G10: overlap: verdict reuse exercised by twelve scripted amendments (M1 polarity pilot here; M4 grade-3 pilot there)
  - reported by G28: overlap with distinct scope: twelve scripted amendments exercise polarity in both (PILOT-C-18 at M1 on mission certificates; the grade-3 pilot at M4 on studio verdicts)

## Cluster of 2 beads

- `N14a-24` (U14a, M5, task) Kill-gate decision KILL-SKEL-PORT: keep limb port packets, or route all dynamics through the whole-model path  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U14a/N14a-24.md`
- `N16a-23` (U16a, M5, task) Kill-gate decisions KILL-TREEOP and KILL-SKEL-PORT: fund the tree-operator route or keep dense with a cap of 64  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U16a/N16a-23.md`

- **pair** `N14a-24` / `N16a-23`
  - reported by G14: both decide KILL-SKEL-PORT (N16a-23 decides it together with KILL-TREEOP)
  - reported by G16: both record the KILL-SKEL-PORT decision from BD-385's pilot; N16a-23 decides it together with KILL-TREEOP

## Cluster of 2 beads

- `N17a-26` (U17a, M3, task) frob-phylum: mission-observable digest: declared read set, exact lowering, memoization, split reports  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U17a/N17a-26.md`
- `N28b-41` (U28b, M3, task) frob-opt: three rules for the mission-observable digest: memo key, structural replay rule, reported factor  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U28b/N28b-41.md`

- **pair** `N17a-26` / `N28b-41`
  - reported by G17: the memo key built on the digest, its store policy and the replay rule are specified in both
  - reported by G28: the memo key built on the mission-observable digest, its store policy and the replay rule are specified in both

## Cluster of 2 beads

- `N19a-38` (U19a, M3, task) frob-scene: hazard incidence export for battery selection, and the uncovered-hazard query  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U19a/N19a-38.md`
- `NEG-29` (UEG, M3, task) frob-conformance: every hazard is covered by a requirement, a shield or a refusal: the cover map and its check  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UEG/NEG-29.md`

- **pair** `N19a-38` / `NEG-29`
  - reported by GEG: both report uncovered hazards for a battery: other from incidence (which hazards a battery's packs drew), mine from the cover map (which drawn hazards have a requirement, shield or refusal that can fire)

## Cluster of 2 beads

- `N25a-23` (U25a, M1, task) e2e mission_admission.sh: every admission refusal of a mission query from the CLI, with code, pointer, first repair  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25a/N25a-23.md`
- `N25d-32` (U25d, M1, task) e2e: the Augur refusal battery: twenty codes through the public verbs, pointer and first repair asserted  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U25d/N25d-32.md`

- **pair** `N25a-23` / `N25d-32`
  - reported by G25ab: overlap: N25a-23 fires every admission refusal from the command line, N25d-32 every one of the chapter's twenty codes through the public verbs; the admission codes are exercised twice
  - reported by G25cde: overlap: both trigger the admission refusals of a mission query from the command line and assert code, pointer and first repair; N25d-32 covers all twenty codes

## Cluster of 2 beads

- `N27a-03` (U27a, M2, task) frob-chronicle: fork points from snapshots, rings and replay-to-tick, with the lead ladder and fork horizon  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U27a/N27a-03.md`
- `N29a-11` (U29a, M2, task) frob-chronicle: per-world device ring for T2, T3: flushed on failure, frozen at mode change or requested tick  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U29a/N29a-11.md`

- **pair** `N27a-03` / `N29a-11`
  - reported by G29: per-world rings and the snapshot reservoir appear in both; N27a-03 depends on N29a-11

## Cluster of 2 beads

- `N32a-11` (U32a, M1, task) frob-conformance: surface parity harness: served CLI and MCP diffed against the registry (GATE-TETHER-02)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U32a/N32a-11.md`
- `NJL-02` (UJL, M0, task) frob-conformance: verb registry conformance harness: surface parity, e2e coverage lint, help-schema snapshots  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/UJL/NJL-02.md`

- **pair** `N32a-11` / `NJL-02`
  - reported by G32: two surface parity harnesses in frob-conformance: served CLI and MCP diffed against the registry (GATE-TETHER-02) here, the registry conformance harness with parity, e2e coverage lint and help-schema snapshots there
  - reported by GJL: both build the parity harness that diffs the served CLI and MCP surfaces against the registry for GATE-TETHER-02; NJL-02 adds the e2e-coverage lint and help-schema snapshots

## Cluster of 2 beads

- `N39b-42` (U39b, M7, task) e2e tenfold_suite_hygiene.sh: GATE-TENFOLD-12: easy wins, unresolved answers, mutations, withdrawals  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U39b/N39b-42.md`
- `N40-21` (U40, M7, task) program gate GATE-TENFOLD-12: suite hygiene (easy wins, unresolved answers, planted mutations, withdrawals)  
  file: `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/U40/N40-21.md`

- **pair** `N39b-42` / `N40-21`
  - reported by G39: both record GATE-TENFOLD-12
  - reported by G40: both measure GATE-TENFOLD-12 on the sealed suite and apply the per-stratum consequence; N40-21 is made to depend on N39b-42

