---
key: "FD-FSYM-02"
title: "upstream frankensympy: a Lie-algebra module (structure constants, ad matrices, BCH coefficients)"
type: "task"
priority: 3
milestone: "M2"
labels: ["m2", "suite", "marrow", "lemma", "upstream", "frankensympy", "solid"]
parent: "WP-05"
deps: ["BD-037", "BD-019", "NF-01", "STD-E2E"]
needs: []
related: ["BD-010", "BD-011", "BD-045", "BD-021", "FD-FSYM-03"]
gates: []
plan_refs: ["Appendix F.1", "Appendix F.3", "§12.1", "§13.1", "§33.4", "§33.8"]
---
## Background

The mathematics of FrankenRobotics's motion is Lie-theoretic: rotations and rigid motions are groups, joint
motions are one-parameter subgroups, and integrators, Jacobians and their certificates are statements about
exponentials, adjoints and brackets. Those statements are derived and certified at build time by
`derive/frob-derive` with FrankenSymPy. FACT (audit of `frankensympy` at `29f847c1`): FrankenSymPy has **no Lie
algebras, quaternions, noncommutative symbols or group theory in Rust**. Upstream register priority: **P3**.

So the derivation tool writes its Lie algebras out by hand, as explicit matrices: a basis of so(3) or se(3) as
arrays of rationals, brackets by matrix multiplication, adjoint matrices assembled entry by entry. This is the
"algebraic route" the certificate design chose on purpose (formulate what is certified as polynomial or
rational identities, because that is what the tools can check now), and it works for the handful of algebras
in use. The upstream item would give the derivations a checked library instead of hand-assembled tables, and a
second implementation to compare with. Rejected: symbolic noncommutative algebra in general (far more than is
needed; the algebras here have dimension at most about a dozen).

## Scope

In: in FrankenSymPy, a small exact module for finite-dimensional matrix Lie algebras; here, a cross-check of
the explicit matrices. Out: `frob-lie`'s runtime types and laws (BD-010, BD-011, BD-045); analytic statements
about exp and log on manifolds, which wait on proof tooling (FD-FLN-01).

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

**The change (DESIGN).** Over exact rationals (and FrankenSymPy's polynomial coefficients where symbolic):

1. *Structure constants.* Given a basis E₁ … E_n of a matrix Lie algebra, compute c_ij^k with
   [E_i, E_j] = E_i·E_j − E_j·E_i = Σ_k c_ij^k·E_k, by exact linear solve; refuse a list that is not closed under
   the bracket or not linearly independent, naming the offending pair.
2. *Adjoint matrices.* ad_X for X = Σ x_i·E_i, with (ad_{E_i})_kj = c_ij^k, so that ad_X·y is the coordinate
   vector of [X, Y].
3. *Checks.* Antisymmetry c_ij^k = −c_ji^k and the Jacobi identity
   [X, [Y, Z]] + [Y, [Z, X]] + [Z, [X, Y]] = 0, verified exactly, each emitted as an algebraic-identity claim
   the existing proof kernel can check.
4. *Baker–Campbell–Hausdorff coefficients* to a requested order as exact rationals:
   log(e^X·e^Y) = X + Y + ½·[X, Y] + (1/12)·([X, [X, Y]] + [Y, [Y, X]]) − (1/24)·[Y, [X, [X, Y]]] + …,
   returned as nested-bracket terms with coefficients, and, for a given algebra, as polynomial maps in
   coordinates.

Additive: a new module; nothing existing changes.

**Interim posture here.** Explicit matrices in `derive/frob-derive`, with the identities they must satisfy
checked as entry-wise polynomial identities.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSYM-02/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSYM-02`, compiled as the `frob-conformance` integration test `upstream_fd_fsym_02` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks the pinned FrankenSymPy for the structure constants of so(3) from
its standard basis (the entry is absent at the pin).

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

**Adoption.** The register gives this item **no switch-over rule**: nothing in this project must change when
it lands. DESIGN, for when it does: the derivation tool's explicit tables are cross-checked against the
module's output for every algebra in use (so(3), se(3), so(2), se(2) and the extended pose groups), entry by
entry and exactly; the tables remain the source of the derivations unless a later decision replaces them, and
`check-generated` must show no generated kernel changed.

**Fallback if the change cannot land.** Explicit matrices, as today.

## Acceptance criteria

- [ ] In FrankenSymPy: the module computes structure constants, adjoint matrices and BCH coefficients exactly, emits the antisymmetry and Jacobi claims, and refuses a non-closed or dependent basis by name.
- [ ] In FrankenSymPy: the known answers below pass.
- [ ] Here: every explicit table in `derive/frob-derive` equals the module's output exactly, or the difference is recorded with the convention that explains it (basis order or sign).
- [ ] Here: no generated kernel's digest changes as a result of the cross-check.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSymPy (added):
- `so3_structure_constants`: basis (L_x, L_y, L_z) with (L_i)_jk = −ε_ijk; expected [L_x, L_y] = L_z and cyclic, so c_12^3 = c_23^1 = c_31^2 = 1 and the antisymmetric partners −1, all others 0.
- `so3_ad_is_the_hat_map`: ad of ω = (1, 2, 3) is the skew matrix [[0, −3, 2], [3, 0, −1], [−2, 1, 0]].
- `se3_ad_block_form`: for a twist (ω, v) with the angular part first, ad = [[ω̂, 0], [v̂, ω̂]] as a 6×6 matrix, checked on ω = (0, 0, 1), v = (1, 0, 0).
- `jacobi_holds_exactly`: for so(3) and se(3), the identity as a verified claim.
- `bch_coefficients`: ½, 1/12, 1/12 and −1/24 for the terms above, as exact rationals.
- `bch_truncation_on_nilpotent_algebra`: in the Heisenberg algebra ([X, Y] = Z, Z central) the series stops: log(e^X·e^Y) = X + Y + ½·Z exactly.
- `non_closed_basis_is_refused`: (L_x, L_y) alone; expected refusal naming the pair whose bracket leaves the span.
Here:
- `explicit_tables_match_module`: the cross-check for every algebra in use.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsym_02.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsym_02 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs the derivation of the M0 kernels and `cargo xtask check-generated`,
asserting unchanged digests.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsym_02/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSYM-02`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: [L_x, L_y] = L_z; the Heisenberg truncation; the Killing form of so(3) in this basis, trace(ad_{L_i}·ad_{L_j}) = −2·δ_ij.
- Negative controls: a table with one sign flipped must fail antisymmetry or Jacobi and be named; a BCH table with 1/6 in place of 1/12 must fail on the strictly upper-triangular 4×4 matrices, a nilpotent algebra in which exp and log are finite polynomials and the series stops after its third-order terms, so the comparison is exact.
- Determinism lanes: exact arithmetic, identical on every platform (class D0).

## Considerations

- Conventions differ between sources (angular part first or last in a twist; left or right adjoint); the module must state its own, and the cross-check records the map to this project's coadjoint convention.
- P3 with no switch-over: its value is assurance for derivations already working. It should be scheduled when someone is in FrankenSymPy for FD-FSYM-01 or FD-FSYM-04.
- RISK-07 (expression swell) is unaffected: these are constant tables.
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

Appendix F.1 (row FD-FSYM-02), F.3; §12.1 (the capability audit and the algebraic route), §13.1, §33.4 (FD-FSYM), §33.8.
