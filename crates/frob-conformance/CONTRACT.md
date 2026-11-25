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
| `frob-tenfold` | `src/bin/frob-tenfold.rs` | `N39a-01` | Gate store `tenfold/<gate>/`, ten-fold harness records (`frob.tenfold.*/1`), and harness registries | Gate store `tenfold/<gate>/` (append-only) and harness registries | Reads and writes no product registry (`registries/gates.toml` untouched) |
| `frob-tenfold-score` | `src/bin/frob-tenfold-score.rs` | `N39a-01` | Ten-fold harness records (`frob.tenfold.*/1`) | Scorer's derived records only, append-only, never rewriting an input | Reads and writes no registry |

## 4. Invariants

- **Closed dependency top (`layer = 7`, `rank = 4`):** No workspace manifest depends on `frob-conformance`.
- **No agent verb or `frob-conformance` binary (`ADR-GAUNT-01`):** `autobins = false`, no `default-run`, no binary named `frob-conformance`, every `src/bin/*.rs` declared in `Cargo.toml` and in the `Binaries` table above, and no entry of the verb registry belongs to this crate.
- **One recorder per gate (`N38b-19`):** No file under `src/bin/` or `examples/` other than `examples/record_gate.rs` may call `register::record`, `register::write`, or `recorder::reaffirm`.
- **No caught control, no pass (`INV-GAUNT-02`, `ADR-GAUNT-03`, `GATE-GAUNT-01`):** Every gate with status `pass` has at least one due `runnable` control in `registry/controls.toml`, every bidirectional gate-control link agrees, and every due runnable control carries `last_seen.caught = true`.
- **Append-only control identifiers (`INV-GAUNT-07`):** `NC-nnnn` identifiers are strictly increasing, never reused, never removed, and never renumbered (a placeholder `target` on a control with no `last_seen` may be replaced once by its real target).
- **Mutation isolation:** `gauntlet-mutations` is a compile error in release builds (`not(debug_assertions)`), lists every workspace crate's `conformance` feature, and never lists any per-control mutation feature (`cx-*`, `nc-*`).
- **Memory safety:** `#![deny(unsafe_code)]` across workspace crates.

### Gate Register Base Record Contract (`N38b-18`, `frob.gates/1`)

`src/register/{row,counts}.rs` owns the base record types (`Claim`, `Status`, `GateBase`, `PartBase`, `BaseDocument`, `GateCountsExpectation`), depending strictly on `std` and `frob-core`. Cross-field validation and write-time refusals are implemented by `N40-01` against the following contract:
- **Closed claim kinds (7):** `proof`, `known-answer`, `parity`, `fidelity`, `design-comparison`, `reality`, `performance`. `"method"` and `"Performance of the method"` are rejected and mapped to `design-comparison` only by explicit row edit.
- **Reconciled status set (7):** `not-run` (printed `"not run"`, never passed), `pass`, `fail` (requires `measured` plus register-layer `commit`, `command`, and resolving `receipt`), `killed` (names kill gate and adopted fallback in `detail`; printed `"killed: fallback adopted (<fallback>)"`), `blocked` (names `OPEN-…` in `detail`; printed `"blocked: OPEN-…"`), `no-claim` (terminal, valid only for `claim = "reality"` import-lane gates), `superseded` (names successor gate in `detail`).
- **Status per `[[gate.part]]`:** 40 convergence gates (`GATE-MARROW-01`, `GATE-HAB-01`, `GATE-HAB-13`, `GATE-ARENA-01`, `GATE-LEMMA-03`, and 35 others per `N40-02` step 5) plus 3 admission gates carry parts at multiple milestone exits. A gate's status is derived (`not-run` when no part is recorded or due; otherwise its latest due part).
- **Kill rows measure nothing:** `kind = "kill"` rows never carry `claim`, `claim_id`, `threshold`, `tier`, `controls`, or `[[gate.part]]`.
- **Four `pass` refusal conditions (validator contract, `N40-01`):** (a) `controls` is empty; (b) any listed control was not caught in the same run as the passing measurement; (c) `recorded.session` is a member of `recorded.implemented_by` (sorted comma-joined implementing sessions, `N38b-19`); (d) an `evidence` item or `claim_id` does not resolve in the artifact graph (`FROB-REF-869`), or the part's `receipt` does not name a committed `receipts/gates/<hash>.json` whose bytes hash to it.
- **`measured` and append-only identity:** `measured` carries the measured value (non-empty and never byte-equal to `threshold`); gate IDs are unique and append-only; plan-time inventory (`278 + 111 + 32 = 421`, noting 112 in register intro) and dated workspace revisions live in `registry/gate_counts.toml`.

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
- Base gate-record types, D0 TOML printer, plan-time/revision count assertions, and additive schema contract land in `N38b-18`; cross-field validation, register-layer fields, and the `record_gate` example land in `N38b-19` and `N40-01`.
- 325 plan-time known-answer rows carry no stated negative control yet; they fail closed as obligations until their owning chapter beads (`N38b-28` onward) register controls.

## Ten-fold harness

### 1. Purpose

The ten-fold harness (`src/tenfold/`, binaries `frob-tenfold` and `frob-tenfold-score`, standalone scorer `kit/scorer/`, test target `tests/tenfold.rs`; owned by `N39a-01`, Plan §39, BET-13, BET-14) is the independent comparative measuring instrument for FrankenRobotics. It measures wall-clock time from a robot-design question to an independently accepted answer across arms `A` (FrankenRobotics), `B` (strongest composed rival), `C` (ablation over an incumbent engine), and `D` (optional product layer over an incumbent engine). It exposes no product `frob` verb, is never exposed over MCP, and drives FrankenRobotics strictly through public `frob` verbs and rival tools strictly out of process (`ADR-GAUNT-01`, `ADR-GAUNT-04`).

### 2. Inputs

- The five harness record schemas (never product formats): `frob.tenfold.mission/1`, `frob.tenfold.freeze/1`, `frob.tenfold.ledger/1`, `frob.tenfold.receipt/1`, and `frob.tenfold.report/1`.
- Common header fields (`schema`, `gate` in `M0-baseline | M1-baseline | M3-partial | M7-tournament | rehearsal-<name>`), identifier types (`GateId`, `Arm`, `StratumId`, `FamilyId`, `MissionHash`, `CampaignIndex`, `Regime`, `Phase`, `Millis`, `Hash32`), canonical decimal seconds strings (`^(0|[1-9][0-9]*)(\.[0-9]{0,2}[1-9])?$`), and exact reduced rationals (`"p/q"` or `"p"`).
- Gate store directories under `tenfold/<gate>/` (`roles.json`, `seal.ndjson`, `comments.ndjson`, `strata.json`, `families.json`, `preanalysis.json`, `cards/`, `sealed/`, `freeze/`, `campaigns/`, `report.json`).

### 3. Outputs

- Unique canonical UTF-8 JSON bytes (reusing `frob-cert`'s canonical JSON serializer unchanged: bytewise sorted object keys, zero insignificant whitespace, minimal string escapes).
- `b3:<64 lowercase hex>` BLAKE3 content addresses over canonical bytes (with `manifest_hash`, `receipt_hash`, or `report_hash` blanked to `""` before hashing).
- Exact reduced rationals paired with exact decimal renderings (point estimates rounded half-to-even; upper/lower confidence bounds rounded outward).
- Structured `N39-REF`-shaped refusal JSON (`{"refusal":{"observed":...,"pointer":...,"reason":...,"repairs":[...],"required":...}}`) on stdout with exit code `3` (input/schema) or `9` (integrity).
- Append-only artifacts inside `tenfold/<gate>/` written via staging file + rename; never writes `registries/gates.toml`.

### 4. Invariants

- **Scorer boundary (`BOUNDARY_FILES`):** Every file under `tenfold/score/`, plus `tenfold/ids.rs`, `tenfold/refusal.rs`, `tenfold/canon.rs`, and `tenfold/records/*.rs`, imports strictly `std`, `frob_cert`, and listed boundary files; calls zero platform transcendental math (`f64::ln`, `exp`, `powf`, etc.); and builds standalone offline via `kit/scorer/Cargo.toml` with `frob-cert` as its sole path dependency, matched one-to-one by `#[path]` modules in `kit/scorer/src/lib.rs`.
- **Canonicalization idempotency & key-order invariance:** `canon(canon(x)) == canon(x)` and permuting object keys leaves canonical bytes and `b3:` content addresses bit-identical.
- **Self-hash integrity:** `manifest_hash`, `receipt_hash`, and `report_hash` are always computed with that top-level field blanked to `""`; any mismatch fails closed with `record-hash-mismatch` (exit `9`).
- **No-authorization & simulation-only flags:** Every record carries `execution_authorized: false`, `hardware_safety_signoff: false`, `sim_to_real_certified: false`, and `reality: "no-claim"`.
- **Append-only gate store:** Re-writing identical bytes is a no-op; attempting to overwrite an existing path with different bytes fails closed with `store-overwrite` (exit `9`).
- **Closed exclusive phase accounting:** The fourteen `Phase` buckets (thirteen phases plus `unattributed`) sum in exact integer milliseconds to `clock_s`.

### 5. Refusals

Harness records fail closed without repairing input, citing the RFC 6901 JSON pointer at fault (`N39-REF` shape):
- `record-unknown-field` (exit `3`): unrecognized object key.
- `record-missing-field` (exit `3`): absent required field.
- `record-duplicate-key` (exit `3`): duplicate JSON key at any nesting level.
- `record-noncanonical-number` (exit `3`): quantity passed as a JSON number, non-canonical duration (`"01"`, `"1.50"`, `"1.500"`, `"1.0005"`, `"1."`, `"-1"`, `"1e3"`), invalid enum/identifier/rational/flag, or phase sum mismatch.
- `record-unknown-schema` (exit `3`): schema outside the closed set of five names.
- `record-hash-mismatch` (exit `9`, integrity): stored content address or self-hash fails recomputation.
- `store-overwrite` (exit `9`, integrity): second write of different bytes to an existing store path.

### 6. Determinism Class

**D0 (bit-exact deterministic across machines, operating systems, and debug/release profiles).** Record parsing, canonicalization, BLAKE3 content addressing, exact rational rendering, and all `tenfold/score/` computations are pure functions of their inputs using integer milliseconds, `BigInt`, and `Rational` (zero floats in stored quantities; no platform math libraries). Measured campaign wall times are empirical measurements and carry no determinism class.

### 7. Evidence Class of Outputs

Harness records and scorer outputs carry evidence class **`Tested`** (never `Certified` or `Proved`, and always `reality: "no-claim"`). Gate values cited from ten-fold records are entered in `registries/gates.toml` exclusively by `examples/record_gate.rs`, never by `frob-tenfold` or `frob-tenfold-score`.

### 8. Validity Domain

- UTF-8 JSON harness records within `frob-cert`'s compiled resource limits (`max_file_bytes`, `max_json_depth`, `max_numeric_entries`, `max_bigint_bits`) and small counts within `±(2^53 - 1)`.
- Durations in exact milliseconds representable in `u64` (`0..=u64::MAX / 1000` seconds).
- Offline standalone scorer builds (`kit/scorer/Cargo.toml`) on host toolchains without network access or third-party crates.

### 9. Oracles

- **Exact phase-ledger known answer (`N39a-01`, `N39a-23`):** Exclusive phases `[600, 360, 20, 0, 12, 96, 150, 60, 40, 0, 30, 42, 90, 0]` s sum exactly to `1,500` s (`1,500,000` ms).
- **Exact rational & outward-rounding known answers (`N39a-01`):** `54,000 / 20,400` reduces to `45/17` and renders half-to-even as `2.647`; `15,255 / 130,000` (`0.11734615...`) renders outward-upper as `0.1174` and outward-lower as `0.1173`.
- **Negative controls (`tests/tenfold.rs`, `scripts/e2e/tenfold_records.sh`):** Duration as JSON number, renamed/unknown field, missing field, duplicate key, non-canonical duration (`"1.50"`), unknown schema, unblanked/mismatched self-hash, `execution_authorized: true`, planted `use rand` / `x.ln()` / `crate::tenfold::store` inside `BOUNDARY_FILES`, and store overwrite with differing bytes.

### 10. Known Gaps

- At skeleton commit (`N39a-01`), domain-specific fields and runtime workflows for mission cards (`N39a-05`), freeze manifests (`N39a-18`), full phase attribution & 2% closure (`N39a-23`, `N39a-24`), receipts (`N39a-09`), full reports (`N39a-36`), roles/sealing (`N39a-02`, `N39a-15`), strata/grid (`N39a-12`), bootstrap/verdict statistics (`N39a-27`..`N39a-37`), and the full `N39-REF` refusal registry binding are staged as empty boundary-clean skeleton modules owned by their respective beads.
