# EV-pack H-12 — simulation-driven generative structure

Source labels: **FETCHED** = page or file read this session; **SEARCH** = search summary only, page not opened;
**MEMORY** = recalled, unverified; **DERIVED** = my arithmetic. Web quotes came through a summarizing fetch tool;
the XDog quote was extracted from the PDF text directly.

## Claim under test

Feeding telemetry load spectra into topology optimization and fatigue analysis yields links at least 15% lighter at
equal stiffness and life than primitive shapes, and the loads-mass loop converges.

## Evidence against

1. **"Primitive shapes" is a weak baseline.** Against engineered parts the published range is 7 to 36%, and equal
   stiffness and life were not enforced together. Table-tennis robot arm, https://arxiv.org/html/2606.26643v1
   (FETCHED): Link 4 0.84 → 0.60 kg (28.57%), Link 3 2.27 → 1.70 kg (25.11%), total 15.50 → 10.60 kg (31.6%). The
   optimizer was set to "minimize mass subject to lower-bound constraints on the natural frequencies";
   "stiffness, peak stresses, and fatigue life were evaluated separately in the subsequent finite-element
   verification." Two links were excluded because they "were already close to their structural limits". Other
   reports (SEARCH): an industrial forearm 7%, an exoskeleton arm 10.4%, a hydraulic leg unit 24.5%.
2. **Link structure is a small share of robot mass.** XDog quadruped, https://www.atlantis-press.com/article/15203.pdf
   (FETCHED): "XDog has a mass of 15 kg in total, while the four legs together weigh about 4 kg (27% of total mass)
   and all the 12 actuators weigh about 6 kg (40% of total mass)." DERIVED: 15% off the whole leg mass is about 4%
   of the robot, and less once gears and shafts are excluded.
3. **Fatigue in legged robots is reported for gears, not links.** A bipedal robot paper, arXiv 2511.01774 (FETCHED
   PDF text) places "fatigue analysis" with actuator selection; its gear figures are 496 MPa and about 2.8e5 cycles
   (SEARCH). No published case of a legged-robot link failing by fatigue was found (null result).
4. **It lengthens the loop.** Each design gains finite-element solves per link. Nothing here reduces time to a
   decision.
5. **Sibling fatigue is quasistatic.** `frankensim/crates/fs-solid/src/fatigue.rs` (FETCHED): "Both models require
   fixed-state, small-strain, elastic, rate-independent applicability" and "time labels do not turn the quasistatic
   solve into dynamics." Footfall impact loads need more, and no telemetry-to-load-case extractor exists.

## Evidence for

- Strongest foundation of the seven: `fs-topopt` (68,735 lines; density method with multi-load, stress and robust
  variants), `fs-topols` (16,807 lines; level set), `fs-solid` fatigue with rainflow counting, a critical-plane
  criterion and crack growth (FETCHED files).
- Savings of 25 to 36% under frequency constraints on a real arm (item 1).
- Feeding optimized mass and inertia back into the dynamic model is a real coupling that learning pipelines skip.

## Kill tests

1. Artifact: passes.
2. Lever: fails for the ten-fold. It may improve the design; it does not shorten the decision. The 15% figure is
   against a baseline nobody ships.
3. Scale: passes for finalists only; three-dimensional optimization of 12 to 20 links is CPU-hours.
4. Foundation: passes with two gaps (dynamic fatigue loads, load-case extraction).
5. Conflation: fatigue life from handbook curves on printed or cast parts is a model claim.
6. Rival: fires. The cited arm study used a commercial optimizer (Altair OptiStruct); loads from multibody
   simulation into topology optimization is established commercial practice (MEMORY).

## Cheapest discriminating test

One quadruped lower-leg link. Baseline: a thin-walled tube sized by hand to the same peak load. Load cases: three
envelope cases from 1,000 simulated episodes. Run `fs-topopt` multi-load with stiffness and stress constraints,
then iterate loads and mass five times. Kill: saving under 5% against the tube at equal first bending frequency and
equal fatigue utilization, or mass still changing by more than 2% between iterations four and five. Cost: two to
three engineer-days, CPU only.

## Recommended state and the claim that survives

**DEFERRED.** Off the ten-fold path. Supported claim: 7 to 30% link-mass saving against engineered baselines, worth
about 4% of robot mass or less, as a finalist-stage refinement.

## Open points

- Mass shares for larger quadrupeds and humanoids were not retrieved.
- Whether the loads-mass loop converges has no published evidence either way.
