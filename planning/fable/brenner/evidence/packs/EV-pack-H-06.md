# EV-pack H-06 — Interval region certificates over design space

Session `RS-20261121-frob-fable-10x`. Marks: FETCHED = read this session; SUMMARY = search-engine summary only.

## Claim under test

Interval and Taylor-model evaluation of static analyses certifies boxes of design space as feasible or infeasible, deciding at least half the volume of a typical 10 to 20 dimensional design box without simulation. Falsifier: more than 90% undecided at 10⁴ boxes.

## Evidence against (excerpts)

1. Arithmetic of the stated budget. 10⁴ boxes are 13 bisections. In 20 dimensions that is less than one cut per axis; uniform boxes have side 0.63 of the domain (0.40 in 10 dimensions). A constraint surface that depends on most parameters crosses nearly every such box, so nearly all stay undecided. The claim can hold only if each constraint depends on few parameters or is monotone.
2. Published robot-design practice is low-dimensional and slow. "such an algorithm is of the branch and bound type, whose worst case complexity is exponential because of the bisection process"; the worked design example has "five design parameters"; the method "may be used for small and medium size problems" and needs "an extended knowledge of possible heuristics" (FETCHED, Merlet, 2009, https://zbc.uz.zgora.pl/repozytorium/Content/78778/download). A three-dimensional workspace paving: "For an ε of 0.05 we found in about 5 hours that the volume of the dextrous workspace is 1.468 with an error bound of [0,0.48]" (FETCHED, Chablat, Wenger, Merlet, https://arxiv.org/pdf/0705.1284). After five hours in three dimensions the undecided volume was still up to a third of the certified volume.
3. Taylor models do not lift the dimension limit. They "only seem to delay the curse of dimensionality (i.e., the exponential growth of work with dimension on many problem classes) very little"; "If dependent intervals are present but without the cancellation structure, gains from a Taylor arithmetic are slight, and the much higher cost and storage does not justify their use."; "Taylor arithmetic in itself does not eliminate the wrapping effect." (FETCHED, Neumaier, https://arnold-neumaier.at/ms/taylor.pdf). Interval-based design of parallel manipulators is reported as time-consuming and storage-hungry (SUMMARY, Nefzi, Riedel, Corves).
4. The metric is weak. Half the volume can be removed by one monotone bound (a mass budget), which needs no interval machinery. Deciding volume is not the same as saving simulations: an agent only needs checks at the points it proposes.

## Evidence for (excerpts)

- "the practical complexity is quite often tractable"; "problems with up to 400 unknowns have been solved" (system solving, not design-space paving) (FETCHED, Merlet, 2009).
- Certified inner and outer workspace volumes for cable robots by branch and prune (SUMMARY, Gouttefarde, Daney, Merlet, 2011).
- Legged static analyses are sparse by construction: torque, reach and clearance of one leg depend on roughly 3 to 6 parameters of that leg, so per-constraint boxes are low-dimensional and products of them cover the full box.

## Kill tests

1. Artifact: nameable. 2. Lever: FAILS as written; "half the volume" is not tied to compute saved or to a baseline. 3. Scale: passes if per-constraint dimension is at most about 6. 4. Foundation: mostly present. `fs-ivl` (11 k lines) has outward-rounded intervals, affine forms, multivariate Taylor models with `compose`, `subdivide_axis`, `range_subdomain` (`src/multi_taylor.rs`), and interval Newton and Krawczyk with `DEFAULT_MAX_ROOT_BOXES = 65_536` (`src/newton.rs`). Missing: a paving driver, contractors (forward-backward propagation), and interval forms of the robot static analyses. 5. Conflation: a certificate covers the static rigid model only; it says nothing about dynamic performance. 6. Rival: pointwise static checks with an existing library give the same pruning for proposed designs; region certificates are not available from incumbents, but are not needed for speed.

## Cheapest discriminating test

A 12-parameter quadruped box (three link lengths, two hip offsets, three masses, gear ratio, motor mass, body length and width). Constraints: static joint torque in three stance postures, reach to a foothold set, self-clearance. Pave with 10⁴ boxes four ways: plain intervals, affine forms, contractors, and per-constraint decomposition. Kill if the best arm leaves more than 90% undecided, or if over 95% of the decided volume falls to a single monotone bound. Cost: two engineer-days, CPU minutes.

## Recommended state and the claim that survives

NARROWED. Surviving claim: certified feasible or infeasible regions for individual static constraints over sub-boxes of at most about 6 parameters, combined by products and monotonicity. Joint paving of a 10 to 20 dimensional box within 10⁴ boxes is refuted by arithmetic. The mechanism is a source of evidence quality and tolerance robustness, not of the ten-fold.

## Open points

Actual sparsity of the constraint-to-parameter graph for the plan's robot classes; closure constraints for linkage legs raise the coupled dimension; no measurement exists yet of simulations avoided.
