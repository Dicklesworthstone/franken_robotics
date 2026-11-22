---
key: "FD-FLN-02"
title: "upstream franken_lean: its suite lock certified on macOS aarch64 (today only x86-64 Linux)"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "lemma", "upstream", "franken_lean", "frontier"]
parent: "WP-31"
deps: ["BD-037", "BD-380", "NF-01", "STD-E2E"]
needs: []
related: ["BD-381", "FD-FLN-01", "BD-026"]
gates: ["INT-FLN-01"]
plan_refs: ["Appendix F.1", "Appendix F.3", "§12.4", "§33.4", "§33.13", "§33.6", "§33.8"]
---
## Background

FrankenLean is the independent checker that re-checks and seals this project's Lean proofs. Its pins are
governed by its own `SUITE.lock`. FACT (constraints that are facts today): **FrankenLean's suite lock certifies
only `x86_64-unknown-linux-gnu`**, and the project's primary workstation, an Apple-silicon Mac, has no Lean
4.32.0 toolchain installed. So all Lean work of FrankenRobotics runs on a Linux host: the admission test
INT-FLN-01 is stated that way ("`fln` builds on a Linux host, re-checks and seals one trivial theorem from
`lean/`, and the capsule checks from an empty environment"). Upstream register priority: **P3**.

The cost is modest and real: a proof cannot be re-checked or a capsule verified on the machine where most of
the work happens; the certificate pipeline has a step that only one of the two machines can run; and a
"second machine" re-check of a capsule, which this project asks of family answers, cannot be the Mac.
Rejected: running `fln` on the Mac uncertified and trusting it (a checker's verdict on a platform its own lock
does not certify is not evidence); emulating x86-64 Linux on the Mac for routine checks (slow, and it certifies
nothing about the native build).

## Scope

In: in FrankenLean, macOS aarch64 added to the platforms its suite lock certifies, by running its own
conformance receipts there; here, INT-FLN-01 runnable on the Mac. Out: unblocking Mathlib modules (FD-FLN-01);
the Lean project and capsule kind (BD-380, BD-381); the standalone auditor, which is a separate program with
no dependencies and already builds everywhere.

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

**The change (DESIGN; FrankenLean's certification procedure governs).**

1. Install the pinned Lean toolchain (v4.32.0, FACT: FrankenLean's own pin) on macOS aarch64 and build
   FrankenLean's crates there on its pinned Rust toolchain.
2. Run its conformance series on that platform: the re-check of the Lean standard library (expected 2,433 of
   2,433 modules) and the Mathlib re-check at its pinned commit (expected the same accepted set as on Linux,
   9,131 of 11,083 at the time of the audit, with 0 failed), within the machine's memory (a full Mathlib
   re-check peaked at 92.8 GiB on Linux; the peak on the Mac is recorded, and the run is refused if it would
   exceed the machine's working-set limit).
3. Compare, module by module, the verdicts on the two platforms; any difference is a finding that blocks
   certification until explained.
4. Seal one capsule on each platform from the same inputs and compare bytes; verify each platform's capsule on
   the other.
5. Add `aarch64-apple-darwin` to `SUITE.lock` with the receipts, under FrankenLean's rules for that file.

Additive: the Linux certification is unchanged.

**Interim posture here.** Lean work on a Linux host: compilation by stock Lean, re-check and sealing by `fln`,
and capsule verification all run there; the Mac runs only this project's own dependency-free auditor.

**Fixture (one reproducible failing fixture, DESIGN).** `tests/upstream/FD-FLN-02/`, opened by BD-037 and kept current
by this bead: one test tagged `FD-FLN-02`, compiled as the `frob-conformance` integration test `upstream_fd_fln_02` and
marked `#[ignore]` so that the default suite stays green, with `expected.toml` beside it holding the signature
recorded at the pinned commit (`observed_at`, the failing assertion and the observed value). It fails at the pin
and passes once the change is adopted; a fixture that passes at the pin no longer demonstrates the limit and is
itself a failure of the registry check. It reads FrankenLean's `SUITE.lock` at the pinned commit and asserts that
`aarch64-apple-darwin` is among the certified platforms (false at the pin).

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

**Adoption.** The register gives this item **no switch-over rule**: the Linux host keeps working. DESIGN, for
when it lands: INT-FLN-01 is run a second time on the Mac (`fln` builds, re-checks and seals one trivial
theorem from `lean/`, and the capsule checks from an empty environment), its record is added beside the Linux
one, and the Mac becomes eligible as the second machine for capsule re-checks. The Linux host stays the
reference platform for sealing; whether the Mac may also seal is the Lemma lead's decision once both platforms
have agreeing records.

**Fallback if the change cannot land.** Lean work on the Linux host, as today; stock Lean alone, recorded as a
single-kernel check, remains the fallback of the foundation itself.

## Acceptance criteria

- [ ] In FrankenLean: `SUITE.lock` lists `aarch64-apple-darwin` with its conformance receipts; the per-module verdicts on macOS equal those on Linux, with 0 failed on both.
- [ ] In FrankenLean: a capsule sealed on either platform verifies on the other, and the two capsules sealed from the same inputs are byte-identical (or the differing field is documented as platform metadata outside the hashed content).
- [ ] Here: INT-FLN-01 has a passing record on the Mac beside the Linux record, each citing its machine fingerprint and the stack receipt.
- [ ] Here: a capsule re-check requested on a platform the pinned `SUITE.lock` does not certify is refused by name, at every pin.
- [ ] The item's row in `registries/upstream.toml` names its fixture, this bead and the mirrored bead in the sibling's tracker; its `status` is `adopted`, or `landed` with the reason the switch-over has not run yet.
- [ ] Every file edited in the sibling was reserved through Agent Mail before the edit and released after landing; the change is on the sibling's `main` and passed that repository's own gates under its `AGENTS.md`.
- [ ] Nothing in this repository patches, forks or vendors the sibling; `cargo xtask check-stack` reports it on-pin and clean under the new receipt, and the record in `DEPENDENCY_ADMISSION.md` cites that receipt.

## Test plan

### Unit tests
In FrankenLean (added to its conformance procedure):
- `stdlib_recheck_on_macos`: 2,433 of 2,433.
- `verdicts_equal_across_platforms`: the two per-module receipts are compared; expected identical accepted sets.
- `capsule_cross_verify`: Linux-sealed verified on macOS and the reverse.
Here (`tests/upstream/FD-FLN-02/`):
- `suite_lock_lists_macos_aarch64`: the fixture.
- `int_fln_01_on_mac`: build, re-check, seal, verify from an empty environment, on the Mac.
- `uncertified_platform_is_refused`: with a scratch copy of the lock from which the platform is removed, the capsule re-check verb refuses.
- `sign_mutated_twin_rejected_on_mac`: the INT-FLN-02 pattern, run on the Mac.

### End-to-end script and logging
`scripts/e2e/upstream_fd_fln_02.sh --phase pre|post` (plain shell, seed-explicit, idempotent, non-zero on failure).
Phase `pre`, valid at any pin before adoption: `cargo xtask check-stack`; `frob doctor --json` (the registry
section shows this item with its status); `cargo test -p frob-conformance --test upstream_fd_fln_02 -- --ignored`,
asserting that the fixture fails with the signature in `expected.toml`; then the interim posture's own tests,
asserting that they pass. Phase `post`, after the pin move: the same fixture must pass, the gates named in this
bead are re-run, and the registry row must read `adopted` with a receipt newer than the pin move. Run once on each machine; the second run compares capsule hashes with the first run's
log. Phase `post` also runs `frob prove verify --json` on one sealed capsule on the Mac.

Logging per STD-E2E: the script sources the shared helper and writes NDJSON to
`target/e2e-logs/upstream_fd_fln_02/<utc-timestamp>.ndjson` with a human-readable transcript beside it. Every line has `ts`,
`script`, `bead` (`FD-FLN-02`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact
command line, exit code, wall time and the envelope's `evidence` block (claim, class, tier, determinism,
identities, seed). It also records the sibling's pinned commit, the stack-receipt hash, the phase, each measured
value with its unit and threshold, digests where state is compared and, on failure, the first differing value
and the artifact paths needed to reproduce. A final `summary` line gives assertions passed and failed and the
gates touched. No private-scene data is logged. The machine fingerprint digest is logged with every capsule hash.

### Known answers and negative controls
- Known answers: the standard-library total 2,433 of 2,433 and the Mathlib totals of the audit; byte-identical capsules from identical inputs.
- Negative controls: a capsule with one byte of its proof content altered must fail verification on both platforms; a lock entry for the new platform without receipts must be rejected by FrankenLean's own lock check.
- Determinism lanes: this item is the cross-platform lane for capsules.

## Considerations

- OPEN-LEMMA-1 (memory and time to re-check one theorem's closure; default: measure before any claim) is measured per platform once this lands.
- The item makes nothing provable that was not provable before; it removes a single-machine step from the certificate pipeline.
- FrankenLean's toolchain is the suite's common nightly, so no toolchain train is involved; `lean/` is not Rust and keeps its own Lean toolchain in any case.
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

Appendix F.1 (row FD-FLN-02), F.3; §12.4 (constraints that are facts today), §33.4 (FD-FLN), §33.13, §33.6 (INT-FLN-01), §33.8; OPEN-LEMMA-1.
