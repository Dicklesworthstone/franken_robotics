# `xtask` Contract

`xtask` is layer 7 (rank 5, tier Solid, scope cap 10,000 lines with tests per ADR-WS-04; ADR-WS-05 governs cap re-sizing) of the FrankenRobotics core workspace.

## Purpose

Provide the workspace-level static verification and status-accounting CLI (`cargo xtask <check> [--root <dir>] [--json]`) enforcing FrankenRobotics architecture, dependency, unsafe-capsule, contract, generated-file, and stack-lock invariants across workspace manifests, registries, and source trees without running physics, recording gate results, or compiling target crates.

## Inputs

- Workspace manifests (`Cargo.toml`, `crates/*/Cargo.toml`, annex manifests) and offline declared/resolved `cargo metadata --format-version 1 --locked --offline` outputs.
- Strict TOML-subset registries under `registries/` (`gates.toml`, `formats.toml`, `refusals.toml`, `lifts.toml`, `capsules.toml`, `verbs.toml`, `kernels.toml`, `upstream.toml`) and JSON Schema files under `schemas/`.
- Stack lock and posture inputs (`franken-stack.lock`, sibling repo commit/status receipts) and per-crate `CONTRACT.md`, `contract.toml`, `SAFETY.md`, and `*.rs` source files.

## Outputs

- Canonical `frob.xtask.report/1` JSON reports encoded through `frob_core::canon` under `--json`, and deterministic human-readable summaries (including per-capsule and per-crate line tables) on stdout.
- Uniform exit codes: `0` (clean, zero findings), `2` (one or more findings), `3` (usage error or offline cache miss requiring `cargo fetch`), `1` (internal tool or tree fault such as non-LF line endings).
- Generated `README.md` honest-status table (`<!-- status:begin -->` .. `<!-- status:end -->`) derived solely from `registries/gates.toml`.

## Invariants

- **Closed six-check ownership:** `xtask` hosts the six workspace checks (`check-layers`, `check-deps`, `check-unsafe`, `check-generated`, `check-contracts`, `check-stack`) with mutually disjoint rule IDs and no Gauntlet or physics execution (`ADR-GAUNT-01`, §34.6).
- **Zero `unsafe`, zero network, zero compilation:** `xtask` inherits `[workspace.lints.rust] unsafe_code = "deny"`, touches no network, never invokes `cargo` by absolute path, and never deletes files or directories.
- **Single TOML and JSON readers:** Registries, locks, and `contract.toml` files are parsed strictly through `frob_core::registry::toml`; JSON inputs and reports pass through `frob_core::json::admit` and `frob_core::canon`.
- **Single authority tables:** The closed 43-crate map (`crate_map.rs`), inversion register (`inversions.rs`), and doorway table (`check_unsafe::DOORWAY_SITES`) each live once in `xtask` and are shared across checks without duplication.

## Refusals

`xtask` defines no public agent-verb refusal codes (`FROB-REF-nnn`); check violations are reported as sorted `Finding { rule, subject, path, line, message, repair }` records in `frob.xtask.report/1` with exit code `2`, usage/offline errors exit `3`, and internal or tree faults (such as CR/CRLF line endings) exit `1`.

## Determinism class

`D0` (Exact). Every check sorts and deduplicates its findings and facts deterministically, counts lines strictly by `\n` (faulting on `\r`), and serializes reports exclusively via `frob_core::canon`, producing identical bytes across repeated runs and platforms when `elapsed_ms` is normalized.

## Evidence class of outputs

`Estimated`. `xtask` is a workspace verification tool and creates no `Tested`, `Certified`, or `Proved` evidence leaves.

## Validity domain

- FrankenRobotics workspace trees using LF (`\n`) line endings in `*.rs` files, with UTF-8 manifests and strict-subset TOML registries under `registries/`.
- Offline Cargo metadata reads require the workspace `Cargo.lock` and local crate cache to be populated beforehand.

## Oracles

- Synthetic planted-violation scratch workspaces exercising every rule of `check-layers`, `check-deps`, `check-unsafe`, `check-contracts`, `check-stack`, and `status_table` (including `GATE-WS-01` plants, `CX-363`, `KA-784`, and `INV-WS-01`..`INV-WS-03`).
- Known-answer line-count fixture (`10 + 20 + 30 = 60` lines excluding `target/`) and exact 115% cap boundary tests (`1000`, `1149`, `1150` against cap 1 kLoC).
- Unit-test conventions (`STD-TEST`): [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, negative controls, and the no-mocks discipline.

## Known gaps

- `check-generated` (`N34p4-03`), registry format/demo single-parser cross-checks (`N34-09`), shadow-run/controller contract records (`N34-10`), release-manifest evidence cells (`N41-29`), and the six-check assembly matrix (`BD-004`, `N34-14`) land in downstream beads.
- Workspace scope caps in `crate_map.rs` reflect ADR-WS-04 (576 kLoC total, `xtask` 10 kLoC); under ADR-WS-05 (`a2eb191`), cap re-sizing awaits owner decision while over-115% crates continue to be reported honestly.
