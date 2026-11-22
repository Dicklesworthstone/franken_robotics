---
key: "FD-FSYM-01"
title: "upstream frankensympy: multi-output straight-line code generation with CSE; the 8,192-operation cap lifted"
type: "task"
priority: 2
milestone: "M2"
labels: ["m2", "suite", "spark", "lemma", "upstream", "frankensympy", "solid"]
parent: "WP-15"
deps: ["BD-037", "BD-019", "NF-01", "STD-E2E"]
needs: []
related: ["BD-018", "BD-221", "BD-149", "BD-150", "NF-04", "SPEC-C-14"]
gates: ["INT-FSYM-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§10 C-14", "§33.4", "§33.15", "§41", "§33.8"]
---
## Background

The arithmetic inside FrankenRobotics's physics kernels is derived, not hand-written: at build time
`derive/frob-derive` uses FrankenSymPy to derive expressions (a Cayley step, a Jacobian row, a cone projection)
and their certificates, and the expressions are emitted as straight-line programs for the CPU and for each GPU
dialect. FACT (FrankenSymPy at `29f847c14428`, read 2026-11-21): it has `Expr`, `fsym_calculus::diff`,
`Matrix::jacobian`, `simplify`, a proof kernel with `Claim::AlgebraicIdentity` and a `PolynomialNormalForm`
rule, and an offline verifier (`fsym-capsule-consumer`). It has **no multi-output code generation with
common-subexpression elimination** and no GPU emitters, and **its bytecode interpreter caps an expression at
8,192 operations**. Upstream register priority: **P2**.

So this project owns the straight-line-program representation and its emitters (`frob-kir`), and uses
FrankenSymPy as the derivation engine and certificate checker only. That split is sound and stays. The
upstream item is still worth doing: a kernel body with several outputs (the twelve entries of a pose update,
the rows of a Jacobian) shares most of its arithmetic, and a derivation engine that can only emit one
expression at a time, capped at 8,192 operations, forces the derivation to be cut by hand at the engine's limit
and not at the kernel's natural boundary. Rejected: emitting GPU code from FrankenSymPy (the dialects and their
determinism rules are this project's); raising the cap by a local patch.

## Scope

In: in FrankenSymPy, a multi-output straight-line generator with sharing and a parameterized operation cap;
here, a regenerate-and-compare run. Out: `frob-kir`, its emitters and its dimension checker (BD-018); the
measurement of the derivation toolchain against its kill gate (BD-221); FrankenSymPy's toolchain move (NF-04).

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

1. *Multi-output programs.* An entry that takes a list of named output expressions over named inputs and
   returns one straight-line program: a sequence of operations, each on earlier values, with a table of which
   value is which output. Structurally equal subexpressions are computed once (hash-consing on the expression
   graph). The order of operations is canonical: a topological order with ties broken by a stable key, so
   that the same inputs give the same program and the same digest on every run and machine.
2. *No silent rewriting of floating-point meaning.* Sharing is of structurally identical subterms only. The
   generator does not reassociate, distribute or contract; any algebraic rewrite is a separate, recorded step
   with its own identity certificate.
3. *The cap.* The limit of 8,192 operations becomes a parameter with the present value as default; evaluation
   of a longer program by the interpreter uses the same instruction set, with the limit a stated budget and
   not a format constant.

Additive: single-expression compilation and its results are unchanged at the default.

**Interim posture here.** `frob-kir` owns code generation: straight-line programs, common-subexpression
elimination, the Rust and dialect emitters and the dimension checker. Derivations are cut per kernel body and
per limb, never per robot.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FSYM-01/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FSYM-01`, compiled as the `frob-conformance` integration test `upstream_fd_fsym_01` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It asks FrankenSymPy for one program with two outputs sharing a
subexpression (the entry is absent at the pin) and for the evaluation of a 10,000-operation program (refused
at the pin by the cap).

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

**What the switch-over re-runs.** The rule is "regenerate and compare digests": every checked-in derived kernel
is regenerated twice, by `frob-kir`'s own elimination and from the sibling's multi-output program, and the two
are compared: operation counts, the digest of the emitted source, and outputs bit for bit on the conformance
vectors. `frob-kir` keeps the representation and the emitters in every case; the sibling's generator replaces
`frob-kir`'s elimination pass only if digests are identical or the emitted kernels are bit-identical on every
vector with no more operations. INT-FSYM-01 is re-run (`derive/` builds on T against the `fsym-*` crates by
path, derives `K-cay-step`, and its certificate replays in the offline auditor), and `check-generated` must
pass on the result.

**Fallback if the change cannot land.** `frob-kir`'s own pass, as today.

## Acceptance criteria

- [ ] In FrankenSymPy: the multi-output entry exists, is deterministic (identical program and digest on repeated runs), and shares structurally equal subterms; the cap is a parameter whose default leaves existing behaviour unchanged.
- [ ] In FrankenSymPy: the program's outputs equal the separately compiled single expressions bit for bit on 10⁴ seeded inputs.
- [ ] Here: the regenerate-and-compare table exists for every checked-in kernel (operation counts, digests, bit equality), and the decision per kernel is recorded.
- [ ] Here: `cargo xtask check-generated` passes; INT-FSYM-01 passes at the new pin; regeneration reproduces the checked-in sources bit for bit under whichever pass is chosen.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenSymPy (added):
- `shares_common_subexpression`: outputs f₁ = (a + b)·c and f₂ = (a + b)·d; expected a program of three operations (one addition, two multiplications), not four.
- `does_not_reassociate`: in single precision with a = 10⁸, b = −10⁸, c = 1, the outputs (a + b) + c and a + (b + c) are 1 and 0; expected the program to keep them distinct and to return 1 and 0.
- `canonical_order_is_stable`: the same output list presented in two orders of construction yields the same program and digest.
- `cap_is_a_parameter`: a 10,000-operation program is refused at the default and evaluated with the cap raised.
- `multi_output_equals_single`: bit equality against separately compiled expressions.
Here:
- `kernel_digests_compare`: the table of the acceptance line, for `K-cay-step` and `K-normalize` first.
- `emitted_kernel_has_no_sin_cos_or_fma`: the dialect lint still passes on kernels regenerated through the sibling's program.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fsym_01.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fsym_01 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Phase `post` also runs the derivation for the M0 kernels both ways, `cargo xtask
check-generated`, and `frob prove verify --json` on their certificates; it logs operation counts and digests.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fsym_01/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FSYM-01`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged.

### Known answers and negative controls
- Known answers: three operations for the shared case; (10⁸ + (−10⁸)) + 1 = 1 and 10⁸ + ((−10⁸) + 1) = 0 in single precision; adjoint duality ⟨J·v, w⟩ = ⟨v, Jᵀ·w⟩ for every derived derivative pair.
- Negative controls: a generator that merges the two reassociated outputs must be rejected by the trace checker; a kernel source with one coefficient edited must fail `check-generated`; an elimination pass that depends on hash-map iteration order must fail `canonical_order_is_stable`.
- Determinism lanes: derivation twice on one machine and on both platforms gives identical digests (class D0).

## Considerations

- RISK-07: FrankenSymPy budgets block derivations or certificates by expression swell on a 29-joint humanoid; earliest signal is derivation time and peak memory per kernel body. A lifted cap moves the limit; it does not make whole-robot derivation sensible.
- RISK-06 (derived kernels slower than hand-written) and the derivation toolchain's kill gate are measured at M2 on this project's own profile; this item can change operation counts, so its comparison table feeds that measurement.
- An e-graph extraction over FrankenJAX's pattern is a separate, optional pilot, admitted only if the M2 profile says arithmetic is a large enough share of the step.
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

Appendix F.1 (row FD-FSYM-01), F.3; §10 C-14 (foundations, test), §33.4 (FD-FSYM), §33.15, §33.8; INT-FSYM-01; RISK-06, RISK-07.
