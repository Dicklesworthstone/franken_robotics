# EV-pack-H-15 — half-precision tensor-core contact solves

Provenance: FETCHED-raw = source text read directly (PDF pages or downloaded source); FETCHED-sum = page read
through the fetch tool, short quotes as returned; MEASURED = computed this session; MEMORY = unverified.

## Claim under test

Factoring H = A + JᵀGJ in `f16` on tensor cores with `f32` iterative refinement speeds the contact solve 4× or
more (falsifier: condition number of H times the `f16` unit round-off at or above one for typical contact problems).

## Evidence against

- The theory promises 2×, not 4×, against a single-precision baseline. With half-precision LU and single working
  precision "it will be possible to solve certain linear systems Ax = b up to twice as fast"; and "the bound on
  κ∞(A) for convergence to be guaranteed drops from 10^8 to 10^4". Full single-precision accuracy at that bound
  also needs residuals computed in double. Half precision has unit roundoff 2^-11 ≈ 4.9 × 10^-4 and range 10^±5
  (FETCHED-raw, Carson and Higham 2018, https://eprints.maths.manchester.ac.uk/2629/1/cahi18.pdf, abstract,
  Table 1.2, section 9).
- The published 4× is against FP64 at huge sizes: "we achieve a 4×−5× performance increase" ... "versus the
  standard FP64 implementation"; "The matrix size is n = 22 000"; "a 2.4× speedup over sgetrf"; FP16 enters only the
  trailing update: "The panel factorizations ... are done in FP32." (FETCHED-sum, https://pmc.ncbi.nlm.nih.gov/articles/PMC7735315/).
- The slate's own falsifier fires (MEASURED: MuJoCo 4.0.1, random-control rollouts, 120 contact states per model,
  H = M + JᵀDJ over active constraint rows, pyramidal cones, default solver parameters):

| Model (degrees of freedom) | median κ₂(H) raw | median κ₂(H) Jacobi-scaled | states with κ·u16 ≥ 1 (scaled) | `f16` Cholesky + `f32` refinement fails | median / p90 refinement steps |
|---|---|---|---|---|---|
| DeepMind humanoid (27) | 2.5e4 | 2.0e3 | 49% | 9% | 7 / 22 |
| Unitree G1 (35) | 1.5e4 | 1.2e3 | 37% | 9% | 6 / 14 |
| Unitree Go2 (18) | 1.7e6 | 2.7e3 | 64% | 18% | 6 / 17 |

  Unscaled, 99 to 100% of states exceed the bound. With products accumulated in `f32` (tensor-core style) failures
  fall to 3 to 8%, at a median of 5 to 6 steps.
- "The joint-space inertia matrix of a robot mechanism can be highly ill-conditioned"; the condition number
  "grows anywhere from O(N) to O(N4)" (FETCHED-sum, Featherstone 2004, ANU record).
- No use of half precision in any rigid-body contact solver was found (one search; absence, not proof).

## Evidence for

- GMRES-based refinement (half, single, double) carries a bound of κ∞ ≤ 10^8 (FETCHED-raw, same paper, Table 8.1).
- Mass matrices alone are benign after scaling (MEASURED): κ₂(M) scaled is 6 for Go2, 244 for G1, 387 for the humanoid.
- Single precision is already standard: "MJWarp utilizes single precision float32" (FETCHED-raw, MuJoCo Warp docs).

## Kill tests (arithmetic)

- Scale, precision: κ·u16 ≥ 1 in 37 to 64% of contact states after best-practice scaling. **Killed by its own falsifier.**
- Scale, compute: an `f32` Cholesky costs n³/3 flops; each refinement step costs about 3n² (one product, two
  triangular solves). At n = 27 with 7 steps: 15,309 against 6,561, ratio 2.3. At n = 35: 1.5. At n = 18: 2.75.
  **Refinement alone costs more than the factorization it replaces**, so the speed-up is below 1 even with a free
  `f16` factor. The ratio 9k/n falls below 0.25 only for n above about 200 at k = 6.
- Hardware: tensor cores accelerate large matrix products; per-world matrices of 18 to 64 rows are smaller than
  one panel. Apple GPUs have no tensor cores (MEMORY for both hardware statements).
- Lever: the 4× baseline was FP64 at n = 22,000. No lever at robot sizes.
- Rival: none does it, which is itself evidence. Conflation: none. Foundation: not assessed.

## Cheapest discriminating test

Already run: about 30 minutes, zero GPU-hours. Script (ephemeral):
`/Volumes/SANDISK_SSD/tmp/claude-501/-Users-jemanuel-projects-franken-robotics/008252e0-1ff2-45d3-82ab-96f5e73f1a2a/scratchpad/scripts/h15_h11_probe.py`.
A confirming GPU test: batched Cholesky at n = 32 in `f16` and `f32` on a 4090, 8,192 worlds. The hypothesis is dead
at under 1.5×. Cost: half an engineer-day.

## Recommended state and the claim that survives

**REFUTED** as stated. Nothing about contact solves survives. The only residue is `f16` storage of read-mostly,
well-scaled buffers to save memory bandwidth, which is a different and unquantified claim.

## Open points

Emulation used NumPy half arithmetic, not a GPU. Stiffer contact parameters than MuJoCo defaults raise κ(H) further.
Elliptic-cone Hessian blocks were not included. An inexact `f16` factor as a preconditioner inside the Newton
iteration was not tested; its hardware gain is still absent at these sizes.
