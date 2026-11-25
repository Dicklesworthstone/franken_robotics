# Dependency Admission Ledger (`DEPENDENCY_ADMISSION.md`)

A foundation (a sibling library or external crate consumed by FrankenRobotics) is not used above the airlock crate until its admission gate has a record in this file.

## Admission Rules

1. **Validity rule.** An admission record is valid only if it cites a commit of this repository whose tracked `Cargo.lock` contains the foundation, together with the exact command, its output (and content-addressed output hash), and the stack resolution receipt hash.
2. **One record per admission gate run, passing or failing.** Every gate attempt is recorded honestly; a failing or blocked observation is a record, and no gate `PASS` is recorded while any sibling checkout or the repository working tree is dirty (such observations are recorded as `development build`).
3. **Fields shown by each record.** Gate identifier, foundation identifier, the commit of this repository whose tracked `Cargo.lock` contains the foundation (or core skeleton for bootstrap), the command, its exit code and output excerpt/hash, the resolution receipt hash, measured values against `TARGET` thresholds, and date.
4. **Never inherited.** An admission is never inherited: a crate or feature group first used at a later milestone gets its own record then, at the pin of that day.

---

## Bootstrap Observation Record — BD-001 (Core Workspace Skeleton)

- **Gate / Scope:** `INT-TOOLCHAIN-01` (provisional `T = nightly-2026-08-31` only — **not** a gate `PASS`)
- **Status:** `development build` (no gate `PASS` recorded: sibling `FD-FSIM` and repository working tree have dirty tracked paths; recorded honestly per BD-001 and OPEN-SUITE-3)
- **Date:** `2026-11-24`
- **Workspace skeleton commit (commit 1, tracked `Cargo.lock`):** `0f363a137fdeae13454ae3fdcb250ad479db55df`
- **Receipt-generation `repo_rev`:** `3888caeed175cf101fc9951a41c657274eb238b3`
- **Tracked `Cargo.lock` BLAKE3 (`core`, 1 package, 0 registry):** `88310da1d0350ed8bf20417a11ed54c17d78199609a9f08f9841085d2619c80a`
- **Resolution receipt (`frob.stack-receipt/1`):** `d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4` (`receipts/d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4.json`)
- **Repeated root settings observed:**
  - `[patch.crates-io]`: `asupersync -> ../asupersync`
  - Profile overrides: `fsqlite*: opt-level 1` (20 `[profile.dev.package.fsqlite*]` tables copied from `../frankensim/Cargo.toml` at HEAD `d02ad9c170d5932667d15359cd362d3198ac4806`)
  - Build identity `[env]` (`.cargo/config.toml`): `FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"`

### Commands and Measured Outputs

| Step | Command | Exit | Wall Time | Output BLAKE3 | Observation |
|---|---|---|---|---|---|
| Skeleton check (initial) | `CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-skeleton timeout 1200 cargo check --workspace --all-targets` | `0` | `45.838 s` | `6c512320c45035dc7ed1cf3ece1e619a76d639d476a63e77897a57c0756864f7` | Offloaded via `rch` to remote `ovh-b` (`Checking xtask v0.0.1`, `Finished dev profile in 0.19s`); expected warnings for unused `asupersync` patch and unused `fsqlite*` dev profile specs. |
| Skeleton check (`--locked`) | `CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-skeleton timeout 1200 cargo check --locked --workspace --all-targets` | `0` | `96.421 s` | `aa5901cf35914de2753e85d9374320cae04dc6211ee594f60cb2768ba3d32b02` | Offloaded via `rch` to remote `ovh-b` (`Finished dev profile in 0.03s`) against committed `Cargo.lock`. |
| `xtask` alias check | `CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-skeleton cargo xtask` | `3` | — | — | Prints usage text (`usage: cargo xtask <check> [--root <dir>] [--json]`) and exits `3`. |
| First resolution receipt (`KA-767`) | `scripts/bootstrap/first_receipt.sh` (followed by `--stdout` byte comparison) | `0` | — | `d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4` | Two consecutive runs produced byte-identical canonical JSON; file name equals BLAKE3 of bytes; zero absolute paths; all 7 local sibling checkouts unchanged (`HEAD` and `git status --porcelain`). |
| Bootstrap unit tests (part B) | `bash scripts/bootstrap/tests/run_all.sh` | `0` | — | — | 7/7 passed (`receipt_is_canonical`, `tree_hash_matches_definition`, `drift_reason_recorded`, `receipt_twice_identical`, `receipt_has_no_absolute_path`, `lock_parses_and_names_T`, `drift_class_table`). |
| Workspace skeleton E2E (`--scratch-only`, part B) | `bash scripts/e2e/suite_workspace_skeleton.sh --scratch-only` | `0` | — | — | `passed=8 failed=0 controls_caught=2` (dirty-sibling control and 1-byte edited-receipt control caught). |

### Out-of-Tree `fs-la` Online Probes (AC5 — Salted Positive Passed; Salt-Removed Negative Honest Observation)

Both out-of-tree online probes used an isolated scratch workspace (`[workspace]`, `fs-la = { path = "../../../frankensim/crates/fs-la" }`, `[patch.crates-io] asupersync = { path = "../../../asupersync" }`, `--offline` and `--locked` dropped for the scratch workspace only, writing nothing inside sibling checkouts):

1. **Salted positive probe (`repro/fs-la-probe-ac5-salted`, `.cargo/config.toml` sets `FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"`):**
   - **Command:** `CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-probe-fs-la-salted timeout 600 cargo check`
   - **Outcome:** **exit `0`** on remote worker `ovh-b` (`wall = 425.159 s`, output BLAKE3 `1c8dcb79cc547257c403c00337ce85c3a68b28472a18471f588db9e927e79173`).
   - **Observed:** `fs-la/build.rs` succeeded, `fs-la` compiled, `Checking frob-fs-la-probe v0.0.1`, `Finished dev profile [unoptimized + debuginfo] target(s) in 3m 39s`.
   - **Sibling immutability:** `../frankensim` `HEAD` (`d02ad9c170d5932667d15359cd362d3198ac4806`) and `git status --porcelain` (hash `6ccad6d926d8634814da4eeef11724eeaa7d372ea552de3d4762f63dfeb5c03b`) byte-identical before and after; `../asupersync` `HEAD` (`70a37b2cfb78ce39dc001739101e0a1a7fbee416`) and `git status --porcelain` byte-identical before and after.

2. **Salt-removed negative probe (`repro/fs-la-probe-ac5-nosalt`, `FRANKENSIM_DEPGRAPH_SALT` and `FRANKENSIM_DEPGRAPH_RECEIPT` unset) — honest observation (did NOT hit the `fz2.6` no-salt panic from inside `repro/`):**
   - **Run A (Cargo parent `.cargo/config.toml` inheritance observed on `omarchy`):** `CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-probe-fs-la-nosalt timeout 900 cargo check -p fs-la` exited `124` (outer timeout, `wall = 900.020 s`, output BLAKE3 `4d4a8955d9255e930782e45ac1e82a2a3d438934818f35db8784e8acc8c73966`; `../frankensim/HEAD` unchanged at `93a6432287f7b7becef7aebd5bb2c138c874c5dc`, `../asupersync` byte-identical). Inside that run, `fs-la/build.rs` **executed** (exit `101`) and printed:
     - `cargo:rustc-env=FS_LA_GEMM_GRAPH_EVIDENCE=salt:franken-robotics-dev-v1` — proving that because `repro/fs-la-probe-ac5-nosalt` sits beneath the repository root, Cargo walked up to ancestor `franken_robotics/.cargo/config.toml` on the worker and inherited `FRANKENSIM_DEPGRAPH_SALT` despite `env -u`.
     - `fs-la/build.rs:55` then panicked with `cannot read required GEMM build-identity input .../frankensim/crates/fs-la/../../constellation.lock: No such file or directory (os error 2)` because `rch` synced only the path-dep crate trees and not sibling root `constellation.lock` to `omarchy`.
   - **Run B (clean-worker retry on `vmi1153651`):** `CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-probe-fs-la-nosalt RCH_CHECK_SLOTS=1 RCH_WORKER=vmi1153651 timeout 600 cargo check -p fs-la -j 1` exited `120` (`wall = 289.155 s`, output BLAKE3 `b50bb5bdaf2e2dd2f7bdf37aa15c233312dfd2606554938a8a30b028f137552b`) with `[RCH-E330] remote_build_disk_budget_exceeded: admission`; `../frankensim` (`HEAD 93a6432287f7b7becef7aebd5bb2c138c874c5dc`, `status` hash `e1a76cb3...`, `Cargo.lock` hash `d3e690ac...`) and `../asupersync` (`HEAD 70a37b2cfb78ce39dc001739101e0a1a7fbee416`) were byte-identical before and after.
   - **Run C (`-C /tmp` isolation attempt on `ovh-b`/`vmi1153651`):** `cargo -Z unstable-options -C /tmp check -p fs-la -j 1 --manifest-path ...` exited `103` (`wall = 8.529 s`, output BLAKE3 `6e9c088d398eed2cb21aaef2d64472b635dcacf541cc3e61b998894972c44149`): `rch` refused planning (`--manifest-path with env --chdir/-C or cargo -C requires an explicit working-directory mapping`); both siblings byte-identical.
   - **Earlier `--locked --offline` probes (retained for context):**
     - Direct sibling workspace (`cargo metadata --manifest-path ../frankensim/Cargo.toml --locked --offline --format-version 1`) exited `101` on `fsci-special = "=0.4.0"` vs `../frankenscipy` `0.5.1` (`N33c-02` drift).
     - Out-of-tree `--locked --offline` on `hz3` exited `101` (`wall = 211.140 s`, output BLAKE3 `8e3657703dfa7a9cf15b5e33469010688e0113166de0084b43503b74fee4a043`) on `failed to download crc32fast v1.5.6: --offline`.
   - **Why live salt-absent `fz2.6` panic was not observed here:** Under `rch` canonical-root rules, a probe placed inside `franken_robotics/repro/` inherits ancestor `franken_robotics/.cargo/config.toml` on any worker that has built `franken_robotics`, while creating a new sibling directory outside the repository under `~/projects/` would persist permanently under the no-deletion rule. Static inspection of `../frankensim/crates/fs-la/build.rs:304-350` confirms `add_depgraph_evidence` panics fail-closed (`"GEMM build identity requires dependency-graph evidence (bead fz2.6)"`) whenever `FRANKENSIM_DEPGRAPH_RECEIPT` and `FRANKENSIM_DEPGRAPH_SALT` are genuinely absent; a live no-salt proof requires running the probe either in a sibling directory outside `franken_robotics/` (with owner/orchestrator path approval) or in a clean worktree where `.cargo/config.toml` does not set `FRANKENSIM_DEPGRAPH_SALT`.

### Sibling Snapshot Recorded in First Receipt

| FD | Sibling | Revision (`HEAD`) | Tracked Dirty | Own Lock | Drift Class | Reason | Tree Hash |
|---|---|---|---|---|---|---|---|
| `FD-ASUP` | `asupersync` | `70a37b2cfb78ce39dc001739101e0a1a7fbee416` | `false` | `null` | `on-pin` | `null` | `null` |
| `FD-FMCP` | `fastmcp_rust` | `83e918637222a98bffc568ef29e1b6ec9a69722a` | `false` | `null` | `on-pin` | `null` | `null` |
| `FD-FNP` | `franken_numpy` | `d548147b049a3d2fe76c20d677cb87d7423b89eb` | `false` | `null` | `on-pin` | `null` | `null` |
| `FD-FSIM` | `frankensim` | `d02ad9c170d5932667d15359cd362d3198ac4806` | `true` | `constellation.lock` (`b5fa60c9185817f42b83395fa3ee1c1a380bf4bdc3af3ec7caa8b08b915d717b`) | `drift` | `dirty` | `b2b6a51d59f4b26e2bf8b11ad6444582b1e6adc9bdee85a0c9e10eeca01ea4bc` |
| `FD-FSQL` | `frankensqlite` | `976b63ccb573b941c24b78d1a64450eb632b1336` | `false` | `null` | `on-pin` | `null` | `null` |
| `FD-FSYM` | `frankensympy` | `1b4cfab25b55aaeba2be4a62ed97048eca6bf4d5` | `false` | `null` | `on-pin` | `null` | `null` |
| `FD-FT` | `frankentorch` | `d138a752cf7fe4abd5ba4baae3f6597c2202461e` | `false` | `null` | `on-pin` | `null` | `null` |

*(Note: `FD-CUDA` is third-party crate `cudarc = "=0.19.9"` with no local sibling checkout and therefore no row in `siblings[]`.)*
