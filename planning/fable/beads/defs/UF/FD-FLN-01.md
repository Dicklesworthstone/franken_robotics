---
key: "FD-FLN-01"
title: "upstream franken_lean: Mathlib roots unblocked (matrix exponential, Lie groups, quaternions, representations)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "lemma", "upstream", "franken_lean", "frontier"]
parent: "WP-31"
deps: ["BD-037", "BD-380", "NF-01", "STD-E2E"]
needs: []
related: ["BD-381", "BD-382", "FD-FLN-02", "FD-FSYM-02", "FD-FSYM-03"]
gates: ["GATE-LEMMA-02"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§12.1", "§12.2", "§12.3", "§12.4", "§12.8", "§33.4", "§33.8"]
---
## Background

In FrankenRobotics Lean is a second opinion, not a prover the system drives (ADR-LEMMA-02): theorems are
written in stock Lean with Mathlib as a development-time tool, stock Lean compiles them, and FrankenLean
re-checks the compiled result with its independent kernel and seals it in a capsule, when the proof's
dependency closure avoids modules FrankenLean cannot yet check. A sealed capsule contains no `sorry`, no axiom
outside a pinned whitelist and no proof of a weakened statement.

FACT (audit of `franken_lean` at `4426f8ea7`): re-checking compiled `.olean` files, FrankenLean accepts the
Lean standard library in full (2,433 of 2,433 modules) and **9,131 of 11,083 Mathlib modules, with 0 failed**.
Among the modules a robotics proof would want, accepted: `Algebra.Lie.Basic`, `Algebra.Lie.OfAssociative`,
`Algebra.Quaternion`, special linear and unitary groups, permutation groups, `Order.FixedPoints`. **Blocked:
`Geometry.Manifold.Algebra.LieGroup`, `Analysis.Normed.Algebra.MatrixExponential`, `Analysis.Quaternion`, and
all representation theory** (none of Mathlib's 43 representation-theory modules is accepted). It does not
elaborate tactic proofs (`MetaM` and `TacticM` are not wired). Upstream register priority: **P3**.

The consequence is the algebraic route: what this project certifies is formulated as polynomial or rational
identities, and only order-theory closures are used in Lean; analytic statements about exp, log and manifolds
wait. A theorem card whose natural proof goes through a blocked module can be `Certified` (an instance
certificate checked exactly) and not `Proved`. Rejected: weakening the capsule rules to admit an unchecked
closure; restating analytic theorems in elementary form at any cost (done where cheap, per OPEN-LEMMA-2; it
does not scale to the theory of Lie groups).

## Scope

In: in FrankenLean, whatever its checker lacks for the root modules on which the four blocked areas depend;
here, promotion of selected theorem cards. Out: the Lean project under `lean/` and the admission tests
INT-FLN-01 and INT-FLN-02 (BD-380); the capsule proof kind (BD-381); reflection (BD-382); certifying the suite
lock on macOS (FD-FLN-02).

## Technical approach

**Where the work is done, and under whose rules (binding).** The change is made in the `franken_lean` repository
(`../franken_lean` beside this one; head `4426f8ea745a` and toolchain channel `nightly-2026-08-31` when this was written
on 2026-11-21, both FACT to re-read), by an agent of this program working there under that repository's own
`AGENTS.md`, gates and conventions. Its pins are governed by its own `SUITE.lock`, which certifies only `x86_64-unknown-linux-gnu` (FACT), so its
gates are run on a Linux host.

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

**The change (DESIGN; FrankenLean's own conformance work governs).**

1. *Find the roots.* From FrankenLean's per-module conformance receipt, compute for each blocked target
   (`Geometry.Manifold.Algebra.LieGroup`, `Analysis.Normed.Algebra.MatrixExponential`, `Analysis.Quaternion`,
   the representation-theory modules) the set of modules in its import closure that are themselves not
   accepted and all of whose imports are accepted: these are the roots. The 1,952 unaccepted modules are
   expected to hang from a small number of them (HYPOTHESIS, settled by the computation).
2. *For each root, classify why it is not accepted*: a kernel feature not implemented, a resource limit, a
   declaration form the checker declines. Record it in the mirrored bead.
3. *Implement or lift,* root by root, under FrankenLean's rule that the independent kernel never accepts what
   it has not checked. Each landed root is followed by a re-run of the receipt.
4. *Invariant.* The count of accepted modules never decreases, and the count of failed modules stays 0: a
   module is accepted or not yet checked, never wrongly accepted.

Additive: nothing accepted today changes status.

**Interim posture here.** The algebraic route; only order-theory closures are used in Lean. Theorem cards
whose closure touches a blocked module stay `Certified` or `Tested` and say so.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FLN-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FLN-01`, compiled as the `frob-conformance` integration test `upstream_fd_fln_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. Four one-line theorems under `lean/`, each importing one blocked target
and stating a trivial true fact from it; stock Lean compiles them; `fln` is asked to re-check and seal each. At
the pin all four are declined because their closures contain unaccepted modules.

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

**What the switch-over re-runs.** The rule is "promote selected cards from `Certified` to `Proved`". For each
unblocked area, the theorem cards whose Lean statements were waiting on it are compiled by stock Lean,
re-checked by `fln`, sealed, and verified from an empty environment; a card is promoted only when its capsule
verifies and its statement is mapped to the exact symbolic form of the artifact it is about. GATE-LEMMA-02 is
re-run (at least three Lean theorem cards compiled by stock Lean, re-checked by `fln`, and sealed; the capsule
verifies from an empty environment). The orbit-counting lemma used by one card is cited only after a recorded
re-check of the Mathlib module that holds it (OPEN-LEMMA-6; until then that count is `Tested`).

**Fallback if the change cannot land.** The algebraic route. KILL-LEMMA and KILL-REFLECT remain the kill gates
of the proof routes and are not triggered by this item.

## Acceptance criteria

- [ ] In FrankenLean: the roots are listed with the reason each is not accepted; after each landed root the conformance receipt is re-run and shows accepted count not lower and failed count 0.
- [ ] In FrankenLean: at least one of the four target areas becomes re-checkable (its target module accepted), with the receipt as evidence.
- [ ] Here: the fixture theorem of each unblocked area is re-checked and sealed, and its capsule verifies from an empty environment; the sign-mutated twin of each is rejected.
- [ ] Here: each promoted card records the capsule hash, the Lean version and the Mathlib commit of FrankenLean's own pin, and the memory and time its closure took (OPEN-LEMMA-1: measured before any claim).
- [ ] Here: no card is labelled `Proved` whose capsule's closure contains an unaccepted module.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenLean (added to its conformance suite):
- `roots_of_blocked_targets`: the computed root set is non-empty and every root's imports are accepted.
- `accepted_count_is_monotone`: the receipt after a change accepts a superset of the modules accepted before.
- `no_false_accept`: a module whose `.olean` has one proof term corrupted is rejected, in each newly supported area.
Here (`lean/` and `tests/upstream/FD-FLN-01/`; run on the Linux host):
- `capsule_verifies_from_empty_environment`: for each fixture theorem.
- `sign_mutated_twin_is_rejected`: the same statement with one sign flipped must fail to check (the pattern of INT-FLN-02: admit a true port identity; reject its sign-mutated twin; list axioms and coverage).
- `no_sorry_no_extra_axiom`: each capsule's axiom list is inside the pinned whitelist.
- `closure_resources_recorded`: peak memory and wall time of the re-check are stored (a full Mathlib re-check peaked at 92.8 GiB; a single theorem's closure is far smaller and is measured, not assumed).

### End-to-end script and logging
`scripts/e2e/upstream_fd_fln_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fln_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Run on the Linux host. Phase `post` also runs `frob prove verify --json` on the promoted
cards' capsules and `frob prove list --json`, asserting the proof class of each card.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fln_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FLN-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged. Each line for a capsule carries its hash, the axiom list and the
closure's module count.

### Known answers and negative controls
- Known answers: the receipt's present totals (2,433 of 2,433; 9,131 of 11,083; 0 failed), against which monotonicity is judged; theorem card T-03 (the quaternion update preserves the norm) already has a Lean route through `Mathlib.Algebra.Quaternion`, which is accepted.
- Negative controls: the sign-mutated twins; a capsule with a `sorry`; a capsule whose statement was weakened (a changed quantifier with consistent hashes) must be rejected by the statement mapping.
- Determinism lanes: capsule hashes identical on repeated sealing; verification on a second machine.

## Considerations

- Lemma does not claim that any Mathlib result about Lie groups, matrix exponentials or representation theory has been re-checked until this item says so for that module; the cards say `Certified` meanwhile, which is a true and weaker statement.
- FrankenLean's suite lock certifies only x86-64 Linux, so all of this runs on the Linux host (FD-FLN-02).
- The item is open-ended by nature (four areas, an unknown number of roots); it is worked one root at a time, and each landed root is adopted by its own pin move.
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

Appendix F.1 (row FD-FLN-01), F.3; §12.1 (the capability table), §12.2, §12.3 (theorem cards), §12.4 (mechanics), §12.8 (OPEN-LEMMA-1, -2, -6), §33.4, §33.8; GATE-LEMMA-02; ADR-LEMMA-02; KILL-LEMMA.
