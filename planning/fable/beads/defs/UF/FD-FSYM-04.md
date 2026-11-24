---
key: "FD-FSYM-04"
title: "upstream frankensympy: certificate families for matrix identities and rational-Gram sums of squares"
type: "task"
priority: 2
milestone: "M5"
labels: ["m5", "suite", "lemma", "upstream", "frankensympy", "frontier"]
parent: "WP-31"
deps: ["BD-027", "BD-037", "N12-16", "N12-20", "N33b-04", "STD-E2E"]
needs: []
related: ["NF-11"]
gates: []
plan_refs: ["Appendix F.1", "Appendix F.3", "§12.1", "§12.4", "§25.4", "§33.4", "§33.18", "§33.8"]
---
## Background

Certificates in FrankenRobotics are built to be checked by tools that exist: polynomial and rational identities,
exact inequalities and enclosures ("the algebraic route"). FrankenSymPy issues them at build time and checks
them (FACT: exact polynomial identity checking with an offline verifier, `fsym-proof-kernel` and
`fsym-capsule-consumer`; exact rational linear algebra with certificates for LU, QR and LDLᵀ; interval
certificates up to 16 coordinates), and this project's own auditor, which depends on nothing, re-checks them.

Two things are missing upstream, and each has a cost here. A matrix statement (a rotation update is orthogonal;
an adjoint matrix has a stated block form) has no certificate family of its own, so it is issued as **entry-wise
polynomial identities**: nine certificates for one 3×3 fact, with nothing saying they belong together. And a
polynomial nonnegativity statement by a sum of squares with a rational Gram matrix has no family at all, so the
`sos-gram` proof kind stays Frontier. Upstream register priority: **P2**.

Rejected: accepting a floating-point sum-of-squares value as a certificate (two cases are on record in which a
returned value from the suite's `fs-sos` is not a bound: coefficients below 10⁻¹⁵ trimmed, and a last
subtraction rounded to nearest; no such value enters a certificate here).

## Scope

In: in FrankenSymPy, two certificate families with their checkers and capsule forms; here, re-issue of the
affected certificates through the instance-certificate pipeline (N12-20), and the one auditor-side rule the
re-issue needs (the matrix group rule below). Out: the auditor's own checkers (`poly-identity`, `rational-ldlt`
and the rest, BD-027, BD-132; `sos-gram`, N12-16), which stay independent of FrankenSymPy; region certificates as
a mechanism (BD-358); `fs-sos` (NF-11).

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

**The change (DESIGN).**

1. *Matrix identities.* A claim kind for "A(x) = B(x) as matrices over ℚ[x]" (and its special cases
   AᵀA = I, A = −Aᵀ, block structure), whose certificate is one object: the shape, the entry-wise normal-form
   witnesses, and a digest binding them, so that the checker verifies all entries and the binding together and
   a missing entry is a failed certificate, not a shorter list.
2. *Sums of squares with a rational Gram matrix.* A claim kind for "p(x) = z(x)ᵀ·G·z(x) with G positive
   semidefinite", with z a declared monomial vector and G a rational symmetric matrix. The certificate is G
   and an exact LDLᵀ factorization G = L·D·Lᵀ with rational L and non-negative rational D. The checker expands
   z(x)ᵀ·G·z(x) exactly and compares coefficients with p, and checks the factorization and the signs of D
   exactly. An optional multiplier form covers "p ≥ 0 on a box" as p = σ₀ + Σ σᵢ·gᵢ with each σ a certified sum
   of squares and each gᵢ ≥ 0 defining the box.
3. Both families get capsule forms that the offline consumer replays, and both are exact: no floating-point
   number appears in a certificate.

Additive: existing claim kinds and capsules are unchanged.

**Interim posture here.** Entry-wise polynomial identities, grouped by this project's bundle format; the
`sos-gram` proof kind stays Frontier and is checked in exact rationals by this project's auditor alone.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSYM-04/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSYM-04`, compiled as the `frob-conformance` integration test `upstream_fd_fsym_04` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks the pinned FrankenSymPy to issue one matrix-identity certificate
for the planar Cayley rotation and one sum-of-squares certificate for 2x² − 2x + 1 (both claim kinds are
absent at the pin).

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

**What the switch-over re-runs.** The rule is "re-issue certificates": every certificate that was issued as a
group of entry-wise identities for one matrix fact is re-issued in the matrix family, and each new certificate
is re-checked by this project's auditor and by FrankenSymPy's offline consumer (two independent checkers); the
old groups are retired only when both agree on every one. For the auditor to check a matrix certificate as one
object it gains one rule (DESIGN, the project-side change of this item, in
`crates/frob-cert/src/kinds/poly_identity.rs` under a new kind version `poly-identity` v2 so that v1 bundles keep
their meaning): the bundle's `query` states the matrix identity with its shape r × c, and the payload carries
exactly r·c entry identities indexed in row-major order, each checked by the existing rule; a missing, duplicated
or extra index rejects the bundle, so a missing entry is a failed certificate and not a shorter list. The rule
reads only the bundle, never FrankenSymPy. `sos-gram` certificates gain FrankenSymPy as a second
checker; whether that moves the proof kind out of Frontier is decided by the certificate-coverage work at M5,
not by this item. Bundle hashes change with the re-issue, so every artifact that cites a re-issued certificate
is revalidated.

**Fallback if the change cannot land.** Entry-wise identities, and `sos-gram` checked by one checker.

## Acceptance criteria

- [ ] In FrankenSymPy: both claim kinds exist with checkers and capsule forms, exact throughout; existing claims and capsules replay unchanged.
- [ ] In FrankenSymPy: the known answers below are issued and verified; the two negative cases are rejected.
- [ ] Here: every matrix fact previously certified entry-wise has one re-issued certificate accepted by both checkers; the count of certificates before and after is recorded per kernel.
- [ ] Here: a re-issued certificate with one entry removed, or with a Gram matrix having a negative pivot, is rejected by this project's auditor (independently of FrankenSymPy).
- [ ] Here: artifacts citing re-issued certificates are revalidated, and none cites a retired certificate.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSymPy (added):
- `planar_cayley_is_orthogonal`: R(c) = (1/(1 + c²))·[[1 − c², −2c], [2c, 1 − c²]]; the claim RᵀR = I reduces to (1 − c²)² + 4c² = (1 + c²)² and is certified as one matrix identity.
- `quaternion_norm_is_multiplicative`: ‖q·c‖² = ‖q‖²·‖c‖² as a polynomial identity in eight variables (theorem card T-03), unchanged by the new family.
- `sos_two_squares`: p(x) = 2x² − 2x + 1 with z = (1, x) and G = [[1, −1], [−1, 2]]; LDLᵀ gives D = (1, 1) and L = [[1, 0], [−1, 1]], so p = (1 − x)² + x²; verified exactly.
- `sos_rank_one`: p = x⁴ − 2x² + 1 with z = (1, x, x²) and G = [[1, 0, −1], [0, 0, 0], [−1, 0, 1]]; D = (1, 0, 0); verified.
- `negative_pivot_is_rejected`: G = [[1, 2], [2, 1]] (D = (1, −3)); rejected.
- `motzkin_has_no_certificate`: x⁴y² + x²y⁴ − 3x²y² + 1 is non-negative and not a sum of squares; the issuer must report failure and never emit a certificate.
Here:
- `auditor_and_sibling_agree`: on every re-issued certificate.
- `missing_entry_fails_the_bundle`: a matrix certificate with one entry's witness deleted is rejected.
- `duplicate_or_extra_index_rejected`: the planar Cayley 2 × 2 certificate with entry (1, 1) listed twice, or with
  a fifth entry, is rejected by the v2 group rule.
- `v1_bundles_unchanged`: every `poly-identity` v1 specimen of BD-027 gives the same verdict and report bytes.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsym_04.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsym_04 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also re-issues the
set (`frob-derive --manifest derive/kernels.toml --stage certify`, N12-20), runs `frob prove verify --all --json`
and `frob prove list --json` over it and `fsym-capsule-consumer` on the same certificates, and logs the counts of
certificates accepted by each checker and the count per kernel before and after.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsym_04/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSYM-04`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the Cayley identity (1 − c²)² + 4c² = (1 + c²)²; 2x² − 2x + 1 = (1 − x)² + x²; the Motzkin polynomial as a true statement with no sum-of-squares certificate.
- Negative controls from the record on `fs-sos`: a coefficient 2⁻⁵⁰ trimmed away makes 1 − 2⁻⁵⁰·x read as the constant 1 and be "certified" as 1 on |x| ≤ 2²⁸, where the minimum is 1 − 2⁻²²; an exact rational certificate cannot make that error, and the test asserts the exact checker gives 1 − 2⁻²² at the endpoint.
- Checker independence: a mutated certificate must be rejected by both checkers; one accepting what the other rejects is itself a failure to investigate.
- Determinism lanes: exact; canonical bytes of each certificate identical on every platform (class D0).

## Considerations

- A true certificate of an uninteresting statement still reads as evidence; the statement embedded in the bundle is what a reader must check, and the matrix family makes the statement one object instead of nine.
- RISK-07 (FrankenSymPy budgets block certificates by expression swell): a matrix identity over many symbols is larger than its entries taken singly only by the binding digest.
- Sum-of-squares certificates with rational Gram matrices need rational rounding of a numerical solution and can fail to exist near the boundary; the family certifies what is found and promises nothing about finding.
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

Appendix F.1 (row FD-FSYM-04), F.3; §12.1, §12.4 (certificate mechanics; the `sos-gram` kind), §25.4, §33.4, §33.18 (row 13, the two `fs-sos` cases), §33.8; theorem cards T-01, T-03.
