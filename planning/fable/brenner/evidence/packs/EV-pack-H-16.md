# EV-pack H-16 — living multiphysics interiors

Source labels: **FETCHED** = page or file read this session; **SEARCH** = search summary only, page not opened;
**MEMORY** = recalled, unverified; **DERIVED** = my arithmetic. Web quotes came through a summarizing fetch tool.

## Claim under test

Sparse islands of fluid film, cables and cloth coupled to rigid robots reveal failure modes of home and kitchen
robots that friction patches and rigid proxies miss, at 10% or more of rigid-only throughput when active in 5% of
worlds.

## Evidence against

1. **Incumbents already ship cables, cloth and particles coupled to rigid robots.** Newton 1.2.0 solver table,
   https://newton-physics.github.io/newton/1.2.0/api/newton_solvers.html (FETCHED): the VBD solver supports rigid
   bodies, particles, cloth and the CABLE joint; it is "An implicit solver using Vertex Block Descent (VBD) for
   particles and Augmented VBD (AVBD) for rigid bodies." A separate solver covers "granular and elasto-plastic
   materials". No liquid solver is listed. Genesis,
   https://raw.githubusercontent.com/Genesis-Embodied-AI/Genesis/main/README.md (FETCHED): "a unified multi-physics
   engine integrating Rigid, FEM, MPM, Particle (PBD / SPH)", "all sharing one scene and one state", with liquid and
   cloth examples. MuJoCo has a cable plugin (SEARCH).
2. **The field models spills as friction, and tests them that way.** A quadruped friction paper,
   https://arxiv.org/abs/2509.19105 (FETCHED abstract), maps terrain "to terrain labels and friction coefficients";
   its hardware test used cooking oil on a whiteboard as a low-friction section (SEARCH). This points toward the
   hypothesis's own falsifier: a friction patch reproducing most liquid failures.
3. **The sibling cannot meet the throughput gate.** `frankensim/README.md` line 1612 (FETCHED): "GPU/accelerator
   execution | Not implemented". `franken_textiles/README.md` (FETCHED): drapes take "14 and 40 CPU seconds on a
   laptop"; "Why not a GPU cloth solver? Garments here are a few thousand vertices; correctness ... matters more
   than throughput". `fs-solid/src/rod.rs` (FETCHED): "Dense-LU Newton with load stepping remains fixture-scale by
   design (≤ a few hundred DOFs)" and "SE(3) dynamics remain successor scope", so there is no dynamic cable.
   DERIVED: with 5% of worlds active, mean cost 0.95 + 0.05c ≤ 10 requires a coupled world to cost at most 181
   rigid worlds. CPU implicit cloth against a batched rigid step misses that by orders of magnitude.
4. **A lattice fluid cannot resolve a spill film.** `fs-lbm` free surface is a three-dimensional lattice method
   (FETCHED); a film under 1 mm thick over square metres is out of reach (DERIVED).

## Evidence for

- The failure modes are real. Robot vacuums are widely reported to stop on cords, socks and rug tassels, and one
  vendor built a product generation around recognizing "cords and pet waste" (SEARCH; the vendor page returned 403).
  No field study of legged or humanoid home robots with cables or liquids was found.
- Entanglement is topological; a friction patch cannot represent a cord wrapping a wheel. A rigid chain can.
- A constant friction coefficient misses one effect (DERIVED from the Stefan squeeze-film formula, MEMORY; smooth
  rigid flat pad, so an upper bound; not checked against experiment). Time to squeeze a film to 10 µm: water under
  a 2 cm, 100 N foot, about 0.04 s; cooking oil at 60 mPa·s, about 2 s; water under a 6 cm, 400 N flat foot, about
  0.8 s. Against a 0.3 s stance, oil and large flat feet give friction that depends on contact time and load. A
  per-contact film law captures this with no fluid island; `fs-tribo` has "Quasistatic, incompressible
  Reynolds-film pressure" (FETCHED).
- Isaac Lab docs, https://docs.nvidia.com/isaaclab/isaac-lab/reproducibility-and-determinism (FETCHED): "PhysX does
  not guarantee determinism for any scene with non-rigid bodies, such as cloth or soft bodies." A small opening.

## Kill tests

1. Artifact: nameable.
2. Lever: none on the ten-fold. It adds cost per world.
3. Scale: fails the throughput gate with sibling solvers.
4. Foundation: fails for cloth, cable and film at batch speed. Fallback: proxies plus a contact-level film law.
5. Conflation: unvalidated coupled models would be presented as realism.
6. Rival: fires for cloth, cables and particles.

## Cheapest discriminating test

On an incumbent with cable and cloth support: three scenes (cord across the path, rug with a free edge, 1 m² oil
patch), 200 randomized episodes each, one wheeled and one legged robot. Rerun with proxies: a 20-link rigid chain,
a hinged thin plate, a friction patch, and the patch with a contact-time film law. Metric: share of full-model
failures the proxy reproduces in the same class within 1 s. Kill: 90% or more. Cost: about one engineer-week,
under ten GPU-hours, no new solver.

## Recommended state and the claim that survives

**REFUTED as stated** (kill tests 3, 4 and 6). Residue worth keeping: a contact-level lubricated-friction material,
rigid-chain cords and hinged-plate rugs in the fast tier, and full cloth or cable only as a CPU referee check on
finalists.

## Open points

- The proxy-adequacy test above has not been run by anyone I could find.
- Throughput of coupled scenes in the incumbents was not found in their documentation.
