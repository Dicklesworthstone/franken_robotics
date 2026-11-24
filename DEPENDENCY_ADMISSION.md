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

### Salt-Removed `fs-la` Probe (Honest Observation — Blocked Before `build.rs`)

The live salt-removed `fs-la` negative-control build did **not** reach `../frankensim/crates/fs-la/build.rs`'s fail-closed panic under `--locked --offline` on this day's sibling checkouts. Two probes were executed read-only against `../frankensim` (zero writes to sibling repositories):

1. **Direct sibling workspace probe** (`cargo metadata --manifest-path ../frankensim/Cargo.toml --locked --offline --format-version 1`): exited `101` during dependency resolution before compiling any build script:
   - `error: failed to select a version for the requirement 'fsci-special = "=0.4.0"'` — candidate `0.5.1` found at `../frankenscipy/crates/fsci-special` (required by `fs-math` dev-dependency; confirms `N33c-02` sibling version drift finding).
2. **Out-of-tree path consumer probe** (isolated scratch workspace with `fs-la = { path = "../../frankensim/crates/fs-la" }` and `[patch.crates-io] asupersync = { path = "../../asupersync" }`, `FRANKENSIM_DEPGRAPH_SALT` and `FRANKENSIM_DEPGRAPH_RECEIPT` removed, `CARGO_TARGET_DIR=${RCH_TARGET_BASE:-${TMPDIR:-/tmp}}/frob-windydog-probe`, `timeout 600 cargo check --locked --offline`):
   - Local offline lockfile generation exited `0` (bypassing `fs-math`'s `fsci-special` dev-dependency).
   - Remote check via `rch` on worker `hz3` exited `101` (`wall = 211.140 s`, output BLAKE3 `8e3657703dfa7a9cf15b5e33469010688e0113166de0084b43503b74fee4a043`) with `error: failed to download crc32fast v1.5.6: attempting to make an HTTP request, but --offline was specified`, before `fs-la/build.rs` executed.
3. **Static fact re-read at HEAD `d02ad9c170d5932667d15359cd362d3198ac4806`:** `../frankensim/crates/fs-la/build.rs` lines 304–350 confirm `add_depgraph_evidence` panics fail-closed (`"GEMM build identity requires dependency-graph evidence (bead fz2.6)"`) when both `FRANKENSIM_DEPGRAPH_RECEIPT` and `FRANKENSIM_DEPGRAPH_SALT` are absent. The live negative control remains open for `BD-006` (`INT-FSIM-01`) once upstream sibling pins and worker offline caches are aligned.

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
