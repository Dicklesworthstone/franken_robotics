---
key: "FD-FSYM-03"
title: "upstream frankensympy: permutation groups and character tables in Rust"
type: "task"
priority: 3
milestone: "M2"
labels: ["m2", "suite", "marrow", "upstream", "frankensympy", "solid", "review-notes-open"]
parent: "WP-03"
deps: ["BD-037", "BD-048", "N13-14", "N33b-04", "STD-E2E"]
needs: []
related: ["SPEC-C-09"]
gates: ["GATE-MARROW-02"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§10 C-09", "§13.1", "§12.1", "§33.4", "§33.8"]
---
## Background

Robots are symmetric: a quadruped has a mirror plane, a hexapod a dihedral group, a wheel a cyclic one.
FrankenRobotics uses those symmetries exactly: it computes the symmetry group of a mechanism and of its
sensing, block-diagonalizes modal problems, builds equivariant policies by irreducible blocks, and counts
distinct designs up to symmetry. All of that rests on finite group theory: closure from generators, conjugacy
classes, character tables, isotypic projectors, orbit counts.

FACT: FrankenSymPy has **no group theory in Rust**; FrankenLean's independent re-check accepted none of
Mathlib's 43 representation-theory modules; FrankenSim's `fs-symmetry` is about 480 lines covering cyclic
groups. The groups here have at most a few dozen elements, so closure, classes, characters and projectors are
built in this project's `frob-algebra`. Upstream register priority: **P3**.

A character table is a small object with strong internal checks (orthogonality), so the local code is
well-guarded already. The upstream item would give it an independent source to cross-check against, and give
the suite's symbolic engine a capability it lacks. Rejected: trusting tables typed in from a textbook without
an executable check; a general computer-algebra dependency from outside the suite.

## Scope

In: in FrankenSymPy, exact permutation groups and character tables for small finite groups; here, a
cross-check of `frob-algebra`'s tables. Out: `frob-algebra` itself (BD-048, BD-049); symmetry detection on
mechanisms (BD-102); equivariant maps (BD-325).

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

**The change (DESIGN).** Exact, for groups of order up to a stated bound (TARGET: at least 1,000):

1. *Permutation groups.* A group from generators (permutations of n points), with closure, order, element
   enumeration in a canonical order, conjugacy classes with sizes and representatives, subgroup and normality
   tests, and orbits and stabilizers.
2. *Character tables.* The table of irreducible characters over an exact field that contains the needed roots
   of unity (cyclotomic numbers, or FrankenSymPy's algebraic numbers), by a standard exact method (for example
   the class-matrix eigenvector method), each row with its degree and its Frobenius–Schur indicator
   ι = (1/|G|)·Σ_g χ(g²) ∈ {1, 0, −1} (real, complex or quaternionic type).
3. *Self-checks emitted as claims.* Row orthogonality (1/|G|)·Σ_C |C|·χ_i(C)·conj(χ_j(C)) = δ_ij; column
   orthogonality; Σ_i χ_i(1)² = |G|.

Additive: a new module.

**Interim posture here.** Small-group code in `frob-algebra`: finite groups from generators, classes,
characters and projectors in exact field elements, checked by the same orthogonality relations and against
tables on file.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSYM-03/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSYM-03`, compiled as the `frob-conformance` integration test `upstream_fd_fsym_03` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks the pinned FrankenSymPy for the character table of S₃ (the entry
is absent at the pin).

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

**What the switch-over re-runs.** The rule is "cross-check tables": for every group of GATE-MARROW-02 (the
cyclic groups Cₙ and dihedral groups Dₙ with n ≤ 12, their products with C₂, and S₄), `frob-algebra`'s
character table and the sibling's are compared after matching classes by representative and characters up to
row order; every entry must be equal exactly. GATE-MARROW-02 is then re-run (character tables and isotypic
projectors correct for those groups, against tables on file). `frob-algebra` stays the runtime owner; the
sibling becomes a second source in `frob-conformance`.

**Fallback if the change cannot land.** `frob-algebra`'s tables, checked by orthogonality and by tables on file.

## Acceptance criteria

- [ ] In FrankenSymPy: groups from generators with classes and orbits; character tables with degrees and indicators; the three self-checks emitted and passing; refusal above the stated order bound.
- [ ] In FrankenSymPy: the known answers below pass exactly.
- [ ] Here: the cross-check passes for every group of GATE-MARROW-02 with zero unequal entries; the comparison is a registered conformance test.
- [ ] Here: GATE-MARROW-02 passes at the new pin.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSymPy (added):
- `s3_character_table`: classes of sizes 1 (identity), 3 (transpositions), 2 (three-cycles); rows (1, 1, 1), (1, −1, 1) and (2, 0, −1); 1² + 1² + 2² = 6.
- `s4_degrees_and_classes`: five classes of sizes 1, 6, 3, 8, 6; irreducible degrees 1, 1, 2, 3, 3; sum of squares 24.
- `cyclic_c4_has_complex_characters`: χ_k(g^j) = i^(jk); two of the four characters have indicator 0.
- `d4_and_q8_have_equal_tables_and_different_indicators`: the dihedral group of order 8 and the quaternion group share a character table; the degree-2 character has indicator +1 for D₄ and −1 for Q₈.
- `row_orthogonality_exact`: for every group above.
- `orbit_count_burnside`: the number of colourings of the four corners of a square with two colours up to the dihedral group of order 8 is (1/8)·(16 + 2 + 4 + 2 + 4 + 4 + 8 + 8) = 6.
Here:
- `tables_cross_check`: the comparison of the acceptance line.
- `projectors_are_idempotent`: isotypic projectors built from either table satisfy P² = P and sum to the identity, exactly.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsym_03.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsym_03 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs `frob algebra irreps --family <family> --in natural --out natural --json` for each group of
the gate (the families `C1` … `C12` and `D1` … `D12`, each also with `xC2`, and `S4`, as `algebra.irreps` spells
them; the row takes a channel space, `--in` and `--out` or `--rep`, beside the group source, and this is the form
N13-20's gate loop uses) and
compares its table with the sibling's, logging the count of entries compared and unequal.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsym_03/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSYM-03`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: the S₃ table; the S₄ degrees; the D₄ and Q₈ indicators; the Burnside count 6.
- Negative controls: a table with one sign changed must fail row orthogonality; a comparison that matches classes by size alone (two classes of S₄ have size 6) must be shown to mis-pair on a planted case and is therefore done by representative.
- Determinism lanes: exact; canonical ordering of elements and classes makes output identical on every platform (class D0).

## Considerations

- Cyclotomic entries need an exact field; if FrankenSymPy's algebraic numbers are slow, tables for the gate's groups are still tiny (the largest, D₁₂ × C₂, has 48 elements).
- Formal proof of representation-theoretic facts is a different item (FD-FLN-01): a cross-checked table is tested evidence, not a proof.
- The counting lemma used for orbit representatives is cited from a proof library only if its closure re-checks; otherwise counts are `Tested`.
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

Appendix F.1 (row FD-FSYM-03), F.3; §10 C-09 (foundations), §13.1, §12.1, §33.4, §33.8; GATE-MARROW-02.
