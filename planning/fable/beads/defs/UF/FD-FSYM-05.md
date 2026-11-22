---
key: "FD-FSYM-05"
title: "upstream frankensympy: faster Gröbner bases and multivariate resultants"
type: "task"
priority: 3
milestone: "M4"
labels: ["m4", "suite", "augur", "upstream", "frankensympy", "frontier"]
parent: "WP-67"
deps: ["BD-037", "BD-105", "NF-01", "STD-E2E"]
needs: []
related: ["BD-361", "BD-019", "FD-FSYM-01", "SPEC-C-11", "SPEC-C-34"]
gates: []
plan_refs: ["Appendix F.1", "Appendix F.3", "§10 C-11", "§25.21.3", "§12.1", "§1.7", "§33.4", "§33.8"]
---
## Background

Kinematics is algebraic geometry: inverse kinematics, singular loci (det J = 0), workspace boundaries and the
synthesis of linkages from precision poses are all systems of polynomial equations. FrankenRobotics solves
them exactly, with certificates, where the systems are small, using FrankenSymPy. FACT
(`frankensympy/crates/fsym-polys/src/groebner.rs`, `fsym-solvers/src/real_rur.rs`; read 2026-11-21): Buchberger
over ℚ and 𝔽ₚ with a **cap of 1,024 basis polynomials and 10⁶ pending pairs, no F4, univariate resultants
only**; the real solver `real_rur` is capped at 8 variables, 64 equations, 4,096 terms, total degree 256,
4,096-bit coefficients, 2·10⁸ work units and 512 MiB, for zero-dimensional ideals only; there is no homotopy
solver. Upstream register priority: **P3**.

The design is sized to those caps: certified inverse kinematics and exact synthesis are limited to leg-sized
systems (a spatial 3R positioning chain, one six-joint leg inside the caps), general 6R inverse kinematics is
not promised, and unbounded whole-robot Gröbner synthesis is not built. That sizing is honest and stays. The
upstream item would move the boundary of "leg-sized". Rejected: a numerical homotopy solver as the exact lane
(it finds roots; it does not certify that all were found); calling a foreign computer-algebra system from the
product (outside the suite's dependency rules; an oracle in tests at most).

## Scope

In: in FrankenSymPy, a faster Gröbner engine and multivariate resultants, with certificates kept; here, an
optional re-measurement of the exact lanes. Out: the exact synthesis lane and the derived leg kernels (BD-361,
BD-105); numerical solvers; the caps of `real_rur`, except as they follow from the Gröbner engine.

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `frankensympy` repository
(`../frankensympy` beside this one; head `29f847c14428` and toolchain channel `nightly-2026-08-20` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions. Its workspace refers to two pin directories, `../franken_numpy-pin` and `../frankenscipy-pin`, that do not
exist on this machine (FACT: `frankensympy/Cargo.toml:95–96`), so its own build instructions are read before
its test suite is run.

1. Read that repository's `AGENTS.md` and its coordination rules first. They govern; where they differ from this
   bead, they win, and the difference is written into this bead's closing note.
2. Other agents work in the same repository. Before editing, reserve every file to be touched through Agent
   Mail and follow the repository's own coordination rules; release the reservations once the change has landed.
3. The change is additive (DESIGN): a new mode, entry point, type or feature. Existing behaviour is not changed,
   and any lossiness is self-described in the new item's documentation and return type.
4. It lands on that repository's `main` through its own tests and gates. Nothing in this repository patches,
   forks, vendors or edits the sibling, and no tool of this repository writes into its checkout (INV-SUITE-04).
5. The work is tracked twice: this bead, and a mirrored bead in the sibling's tracker whose identifier is stored
   in the `mirror` field of this item's row in `registries/upstream.toml` (schema `frob.upstream/1`). The row's
   `status` moves `open` → `landed` (merged on the sibling's `main`) → `adopted` (switch-over run here).

**The change (DESIGN; FrankenSymPy's own roadmap decides the method).**

1. *Gröbner bases.* A matrix-based reduction in the style of F4 (reduce many S-polynomials at once by sparse
   linear algebra over ℚ or modulo primes with rational reconstruction), behind the same interface as the
   present Buchberger, with the caps as parameters and the present values as defaults. The output must be the
   same reduced basis for the same order, and the membership certificates the present engine produces (each
   basis element as a combination of the generators) must still be produced or be reconstructible and
   verified exactly.
2. *Multivariate resultants.* Elimination of several variables at once (a Macaulay or Dixon construction), with
   the extraneous factor identified or bounded, returning an exact polynomial whose roots include every
   projection of a common root.
3. *Budgets.* Every entry keeps a work budget and refuses by name when it is exhausted; nothing loops without
   a bound.

Additive: the present engine remains and is the differential oracle for the new one.

**Interim posture here.** Inverse kinematics and exact synthesis limited to leg-sized systems; a system
outside the caps is refused with the cap it exceeded, and the numerical lane is offered with its label.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSYM-05/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSYM-05`, compiled as the `frob-conformance` integration test `upstream_fd_fsym_05` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It gives the pinned FrankenSymPy a benchmark ideal chosen to exceed the
present caps (the spatial 6R inverse-kinematics system in its standard 14-equation form) and expects a reduced
basis within the stated work budget; at the pin it is refused at a cap.

**Switch-over (the compatibility train).** When the change is on the sibling's `main`: (1) move the sibling's pin
in `franken-stack.lock` to a clean commit that contains it (a pin only moves forward; a sibling's newer clean head
is never rolled back locally); (2) run `cargo xtask check-stack`, which writes a new resolution receipt
(`frob.stack-receipt/1`) under `receipts/`; (3) re-run that sibling's admission gates, the parity gates and the
gates this item names; (4) relock (`Cargo.lock` regenerated and committed, builds `--locked` on the one toolchain
T) and append the record, with the receipt hash, to `DEPENDENCY_ADMISSION.md`. Only then does the row's `status`
become `adopted` and the interim posture end. No PASS is recorded on a dirty sibling tree, and neither a pin nor
T moves while a ten-fold tournament is running. If a gate named here does not exist yet when the change lands,
the pin still moves with the gates that do exist, the row stays `landed`, and the gate's first run is made
against the new pin.

**Adoption.** The register gives this item **no switch-over rule**: nothing here must change when it lands.
DESIGN, for when it does: the derived-kernel fixtures of the leg (INT-FSYM-02: the planar two-link Jacobian
compiles and agrees with closed forms at a regular and a singular configuration) and the exact synthesis
fixtures are re-run on the new engine with identical results required; the size of system admitted to the
exact lane is raised only after a recorded pilot on that size (time, memory, and all solutions found against a
known count), and the admitted size is a constant in the lane's contract, not a hope.

**Fallback if the change cannot land.** The present caps and the present sizing.

## Acceptance criteria

- [ ] In FrankenSymPy: the new engine returns the same reduced Gröbner basis as Buchberger on every problem both can finish (differential test on at least 200 seeded ideals), with membership certificates that verify exactly.
- [ ] In FrankenSymPy: on the leg-sized benchmarks below it is at least 5 times faster than the present engine (TARGET), and it completes at least one benchmark the present engine refuses at a cap.
- [ ] In FrankenSymPy: budgets refuse by name; no entry is unbounded.
- [ ] Here: INT-FSYM-02 and the exact-lane fixtures give identical results on the new engine at the new pin.
- [ ] Here: any increase of the admitted system size is backed by a recorded pilot and stated in the lane's contract.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSymPy (added):
- `circle_and_line`: the ideal ⟨x² + y² − 1, x − y⟩ in lexicographic order with x > y has reduced basis {x − y, 2y² − 1}; two real solutions (±1/√2, ±1/√2).
- `planar_2r_inverse_kinematics`: links of length 1 and 1, target (1, 1); exactly two solutions, joint angles (0°, 90°) and (90°, −90°).
- `spatial_3r_has_at_most_four_solutions`: a generic positioning target; the certified solver finds exactly the solutions the closed form gives (up to four).
- `new_equals_buchberger`: the differential test of the acceptance line.
- `resultant_eliminates_two_variables`: for f = x − a, g = y − b, h = x·y − 6 the resultant eliminating x and y is a·b − 6 up to a unit.
- `budget_refuses_by_name`: a work budget of 10³ on the 6R system; expected the named refusal with the budget.
Here:
- `exact_lane_results_unchanged`: the fixtures of the leg kernel and of bounded linkage synthesis on both engines.
- `over_cap_system_is_refused_with_the_cap`: a nine-variable system at the present caps; expected the refusal naming the variable cap.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsym_05.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsym_05 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob mission synthesize --strategy linkage --json` on the
bounded-synthesis fixture and logs wall time and peak memory beside the values recorded at the old pin.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsym_05/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSYM-05`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the basis {x − y, 2y² − 1}; two solutions for the planar 2R target; up to four for the spatial 3R chain; the general 6R problem has at most 16 solutions, the benchmark count a complete solver must not exceed and, on a generic instance, must reach.
- Negative controls: a modular computation reconstructed from too few primes must fail the exact verification of its certificates and be rejected; a solver that returns a subset of the roots must be caught by the solution count on the planted instances.
- Determinism lanes: exact; the reduced basis is canonical for a given order, so output is identical on every platform (class D0).

## Considerations

- General 6R inverse kinematics by Gröbner bases was assessed and rejected at current tool limits; inside the caps, certified inverse kinematics is Frontier. A faster engine reopens that assessment; it does not reverse it by itself.
- Coefficient growth, not polynomial count, is often what ends an exact computation; the 4,096-bit and 512 MiB limits of the real solver bind independently of this item.
- KILL-SYNTH-LINKAGE's numerical lane remains the fallback of exact synthesis whatever the engine's speed.
- No milestone waits on this bead. Its milestone label says when its switch-over gate first exists, not that the
  milestone's exit depends on it. Until the switch-over has been run, the interim posture above is the design of
  record. M1 depends only on FrankenSim, asupersync and FrankenSQLite, and no upstream work item is on its
  critical path.
- If this change never lands, the program still ships on the interim posture, at the cost stated there. A change
  that does land is adopted only through its switch-over rule: never by a local patch, a fork or a vendored copy,
  and never by rolling a sibling's newer clean head back. Neither a pin nor T moves during the ten-fold
  tournament of M7.
- The priority in the upstream register states the item's value to this project and its place in this program's
  queue. The risks carried at program level are the cost and ordering of upstream work and collisions with other
  agents in the same sibling; the mitigations are file reservation, small commits and the sibling's own gates.

## References

Appendix F.1 (row FD-FSYM-05), F.3; §10 C-11 (honest limits, foundations, test), §25.21.3 (bounded exact linkage synthesis), §12.1, §1.7, §33.4, §33.8; INT-FSYM-02.
