# `frob-conformance` Contract (`CONTRACT.md`)

## 1. Purpose

`frob-conformance` (layer 7, rank 4, tier Solid, scope cap 24,000 lines) is the Gauntlet verification crate of FrankenRobotics (`ADR-GAUNT-01`, `ADR-GAUNT-02`, `ADR-GAUNT-03`, `BD-025`). It sits at the top of the workspace dependency graph: it may depend on any workspace crate as needed by a target, and no workspace crate may depend on `frob-conformance`.

**How the Gauntlet is run (`ADR-GAUNT-01`):**
- There is **no** `frob conformance` command, verb, or subcommand, and no `xtask` subcommand for the Gauntlet.
- There is **no** binary named `frob-conformance` and no `default-run` key in `Cargo.toml`.
- A gate or known-answer row is run as a Cargo test target:
  `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test <tier> -- <gate|row>`
  where `<tier>` is one of the six full target names `g0_laws`, `g1_known_answers`, `g2_parity`, `g3_metamorphic`, `g4_chaos`, `g5_determinism` (never `g0`..`g5`), adding `--features gauntlet-mutations` (and `<crate>/<feature>` per control) when running negative controls.
- A gate result is recorded exclusively through the recorder example target (`N38b-19`):
  `cargo run -p frob-conformance --example record_gate -- --gate <id> --receipt <hash>`
- Subsystem harness drivers run as declared examples (`cargo run -p frob-conformance --example <harness> -- <sub> ...`).

## 2. Inputs

- The Gauntlet registries under `crates/frob-conformance/registry/`: `controls.toml`, `known_answers.toml`, `relations.toml`, `counterexamples.toml`, `pilots.toml` (parsed exclusively through `frob_core::testkit::registry` and `frob_core::registry::toml`).
- The workspace gate register `registries/gates.toml` (`frob.gates/1`).
- Previous-commit snapshots of `controls.toml` for append-only and placeholder-target checks.
- End-to-end script headers (`# e2e:`) and `frob.e2elog/1` NDJSON logs for `frob-e2e`.
- Workspace manifests, crate sources, scripts (`scripts/`, `crates/xtask/`), fixtures, and `frob.testevidence/1` NDJSON streams for structural lints.

## 3. Outputs

- Typed validation reports and harness reasons (`out-of-order`, `reused`, `removed`, `renumbered`, `no-gate`, `unknown-gate`, `kind-not-closed`, `feature-missing`, `fixture-missing`, `defect-or-expect-empty`, `boundary-not-control`, `unknown-seed-form`, `no-control`, `dangling-control`, `one-way-link`, `uncaught-control`).
- Deterministically formatted `not-eligible` listings for gates without due runnable controls.
- `frob-e2e` structured NDJSON logs, transcripts, capture artifacts, mode-`0600` secret token files, and gate reports (`gates.json`, `gates.md`).

### Binaries

| Binary | Path | Owning bead | Reads | Writes | Registries |
|---|---|---|---|---|---|
| `frob-e2e` | `src/bin/frob-e2e.rs` | `STD-E2E` | Script headers (`# e2e:`), `frob.e2elog/1` NDJSON logs, and the captured output of the commands it runs | `frob.e2elog/1` NDJSON logs, transcripts, capture artifacts, mode-`0600` session token files, the gate report (`gates.json`, `gates.md`), and the `frob.e2elog/1` schema print | Reads and writes no registry |

## 4. Invariants

- **Closed dependency top (`layer = 7`, `rank = 4`):** No workspace manifest depends on `frob-conformance`.
- **No agent verb or `frob-conformance` binary (`ADR-GAUNT-01`):** `autobins = false`, no `default-run`, no binary named `frob-conformance`, every `src/bin/*.rs` declared in `Cargo.toml` and in the `Binaries` table above, and no entry of the verb registry belongs to this crate.
- **One recorder per gate (`N38b-19`):** No file under `src/bin/` or `examples/` other than `examples/record_gate.rs` may call `register::record`, `register::write`, or `recorder::reaffirm`.
- **No caught control, no pass (`INV-GAUNT-02`, `ADR-GAUNT-03`, `GATE-GAUNT-01`):** Every gate with status `pass` has at least one due `runnable` control in `registry/controls.toml`, every bidirectional gate-control link agrees, and every due runnable control carries `last_seen.caught = true`.
- **Append-only control identifiers (`INV-GAUNT-07`):** `NC-nnnn` identifiers are strictly increasing, never reused, never removed, and never renumbered (a placeholder `target` on a control with no `last_seen` may be replaced once by its real target).
- **Mutation isolation:** `gauntlet-mutations` is a compile error in release builds (`not(debug_assertions)`), lists every workspace crate's `conformance` feature, and never lists any per-control mutation feature (`cx-*`, `nc-*`).
- **Memory safety:** `#![deny(unsafe_code)]` across workspace crates.

## 5. Refusals

`frob-conformance` defines no public agent-verb refusal codes (`FROB-REF-nnn`); its checks fail closed with typed Gauntlet harness reasons (`N38-REF`):
- Control loader: `out-of-order`, `reused`, `removed`, `renumbered`, `no-gate`, `unknown-gate`, `kind-not-closed`, `feature-missing`, `fixture-missing`, `defect-or-expect-empty`, `boundary-not-control`, `unknown-seed-form`.
- Registry check: `no-control`, `dangling-control`, `one-way-link`, `uncaught-control`.
- Command-form lint: `frob conformance`, command-position `frob-conformance`, default `cargo run -p frob-conformance`, short tier targets `--test g0..g5`, and `unknown-target`.

## 6. Determinism Class

**D0 (bit-exact deterministic across machines, operating systems, and build profiles).** Registry validation, control serialization, command-form linting, binary declaration checks, and `frob-e2e` log/report operations are pure functions of their inputs.

## 7. Evidence Class of Outputs

The negative-control registry check and structural lints produce evidence of class **`Tested`**, claim kind **`known-answer`**, determinism class **`D0`**. They establish structural eligibility and control coverage; they do not substitute for physics simulation, proof checking, or physical reality claims.

## 8. Validity Domain

- Workspace repositories carrying `franken-stack.lock`, `registries/gates.toml` (`frob.gates/1`), and `crates/frob-conformance/registry/*.toml` encoded in the strict `frob_core::registry::toml` subset.
- Host-only builds with default features (no `cuda` or `metal` annex required for G0, host G1, or structural checks).

## 9. Oracles

- **Unit-test conventions (`STD-TEST`):** [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, address-keyed property tests, the single-reader registry loaders, determinism lanes, the `frob.testevidence/1` evidence stream, and the no-caught-control-no-pass rule enforced by `tests/std_test_lint.rs`.
- **Wheel-loop negative-control known answer (`CX-262`, `NC-0412`):** True spectral radius `0.9207` versus explicit-Euler mutant `1.497`.
- **Negative controls of the harness itself:** Emptied control list (`no-control`), dangling control identifier (`dangling-control`), one-way gate-control link (`one-way-link`), uncaught control on `pass` (`uncaught-control`), boundary-only control without `removed_check` (`boundary-not-control`), all twelve loader plants, all five refused command forms, and all seven manifest/umbrella plants.
- **Metamorphic relation:** Reordering `[[gate]]` rows in `registries/gates.toml` leaves every registry check verdict unchanged.

## 10. Known Gaps

- At v0 (`BD-025`), the six tier test targets (`g0_laws`, `g1_known_answers`, `g2_parity`, `g3_metamorphic`, `g4_chaos`, `g5_determinism`) carry empty static entry tables and self-tests; tier execution semantics, site selection, and hash-named receipts land in `N38a-02`.
- Automated control execution and staleness detection land in `N38b-04`; deterministic seeding of all 384 counterexamples and 510 stated control cells lands in `N38b-05`.
- Full gate register fields, cross-field validation, and the `record_gate` example land in `N38b-18`, `N38b-19`, and `N40-01`.
- 325 plan-time known-answer rows carry no stated negative control yet; they fail closed as obligations until their owning chapter beads (`N38b-28` onward) register controls.
