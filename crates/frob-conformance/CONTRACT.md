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

- The Gauntlet registries under `crates/frob-conformance/registry/`: `controls.toml`, `known_answers.toml`, `relations.toml`, `counterexamples.toml`, `pilots.toml` (`frob.gauntlet.pilot/1`, `N38a-09`), `pilots_expected.toml`, `reasons.toml`, `hypotheses.toml` (`frob.hypothesis/1`, `N00-10`) (parsed exclusively through `frob_core::testkit::registry` and `frob_core::registry::toml`).
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

### Gate Register Contract (`N38b-18`, `N40-01`, `ADR-GAUNT-04`, `frob.gates/1`)

`src/register/{row,counts}.rs` owns the base record types (`Claim`, `Status`, `GateBase`, `PartBase`, `BaseDocument`, `GateCountsExpectation`), depending strictly on `std` and `frob-core`. `src/register/{validate,write}.rs` and the read-only validator example `examples/validate_register.rs` (`cargo run -p frob-conformance --example validate_register -- --file <path>`) implement the full register types (`GateRow`, `Part`, `Kind`, `Needs`, `Register`), nine-rule validator, ten write-time refusals, and atomic validated write path:
- **Inputs:** `registries/gates.toml` (`frob.gates/1`, strict `frob_core::registry::toml` subset; additive top-level tables `amendment`, `shared_fixture`, `related_threshold`, `fallback_exit`, `variant`, and `retired_ids`), `crates/frob-conformance/registry/controls.toml`, committed gate receipts under `receipts/gates/<hash>.json` (including receipt sets `frob.gauntlet.receipt-set/1`), live `ClaimId` / evidence sets, and previous/retired gate ID sets.
- **Outputs:** Validated `Register` values, D0 canonical TOML (`Register::print_toml`), exit due-sets (`Register::due_gates`), reconciled printed statuses (`GateRow::printed_status`), and structured `Violation` diagnostics (exit `0` when valid, exit `3` naming the offending gate row and `N38-REF` reason slug when refused). On any write-path refusal (`register::record`), `registries/gates.toml` is left bit-identical.
- **Closed claim kinds (7, rule 3):** `proof`, `known-answer`, `parity`, `fidelity`, `design-comparison`, `reality`, `performance`. `"method"`, `"Performance of the method"`, and array claims are refused (`claim-not-one-of-seven`).
- **Reconciled status set (7, `docs/adr/ADR-GAUNT-04.md`):** `not-run` (printed `"not run"`, never passed), `pass`, `fail` (requires `measured` distinct from `threshold`, `commit`, `command`, and committed `receipt`), `killed` (names kill gate or `BET-` for `program` rows, adopted fallback, and `ADR-` in `detail`; printed `"killed: fallback adopted (<fallback>)"`), `blocked` (names `OPEN-…` or `claim-reading: <bead>` in `detail`; printed `"blocked: <detail>"`), `no-claim` (terminal, valid only for `claim = "reality"` with non-empty reason in `detail`, e.g. `no-dataset-admitted`), `superseded` (names an existing successor gate in `detail`).
- **Status per `[[gate.part]]` and import-lane / pilot scopes:** 40 convergence gates plus 3 admission gates carry parts at multiple milestone exits. A gate's status is derived (`not-run` when no part is recorded or due; otherwise its latest due part). Import-lane gates (`needs = "import-lane"`, valid only for `claim = "reality"`, e.g. `GATE-GROUND-10`) and non-blocking admission rows (`blocks_exit = false`, valid only on `admission` rows) never enter any milestone exit's due set. A part whose `scope` begins `pilot: <PILOT-id>` must match `N38a-09`'s pilot-ID grammar and stay `not-run`. Optional `sources` entries cite `SRC-…` (optional `#figure`).
- **Kill rows measure nothing:** `kind = "kill"` rows never carry `claim`, `claim_id`, `threshold`, `tier`, `controls`, or `[[gate.part]]`; rule 3 does not apply to kill rows.
- **Rule 8 recurring gates:** Compiled recurring set `{GATE-GROUND-01, GATE-SAFE-01, GATE-GAUNT-01, GATE-TENFOLD-09}` (4) and late recurring set `{GATE-GAUNT-02, GATE-GAUNT-03, GATE-GAUNT-05, GATE-GAUNT-10, GATE-WS-08}` (5) carry `recurring = true`; any extra or missing `recurring` flag fails validation naming the row.
- **Ten write-time refusals (`N38-REF` slugs):** `pass-without-control` (`controls` empty on `pass`), `control-not-caught` (any due control `due <= part.milestone` missing or uncaught on `pass`; future-milestone controls are `not-yet-due`), `self-certified` (`recorded.session` empty or a member of comma-joined `recorded.implemented_by` on `pass`), `evidence-unresolved` (surfaces as public `FROB-REF-869` for a missing/malformed `N34-11` `ClaimId`, unresolved claim or evidence item, or receipt/receipt-set member missing from `receipts/gates/<hash>.json` or failing BLAKE3 byte verification), `target-as-measurement` (`measured` empty or byte-equal to `threshold` on `pass`/`fail`), `claim-not-one-of-seven`, `no-claim-on-non-reality` (`no-claim` or `needs = "import-lane"` on non-reality gate, or empty `detail` on `no-claim`), `blocked-without-question`, `killed-without-decision`, and `duplicate-id` / `reused-id` (append-only unique IDs; no removal, renumbering, or reuse of retired IDs).
- **Plan-time & revision counts:** Plan-time inventory (`278 + 111 + 32 = 421`, noting 112 in register intro) and dated workspace revisions live in `registry/gate_counts.toml`.

### Gate Recorder Contract (`N38b-19`, `examples/record_gate.rs`, `register::record`)

`src/register/record.rs` and `examples/record_gate.rs` (`cargo run -p frob-conformance --example record_gate -- --gate <id> --receipt <hash> [--part <milestone>]`) are the sole writer of `[[gate.part]]` measurements in `registries/gates.toml`:
- **Session identity mechanism and limit (`INV-GAUNT-05`):** Every recording session identifies itself via `FROB_SESSION` (`session:<name>`), and every commit carries a `Frob-Session:` trailer. The recorder derives `recorded.implemented_by` as the sorted, comma-joined union of `Frob-Session:` trailers on capability commits up to the receipt's commit plus every session on an `implement` line of the delivering bead log(s), and refuses `pass` with `self-certified` whenever `recorded.session` is a member of `implemented_by`. **Limit:** This is a discipline check, not cryptographic authentication — a session can falsify an identity by exporting another `FROB_SESSION` value; detection relies on the planted self-certification negative control (`GATE-GAUNT-10`), independent fresh-context milestone exit reviews, and commit/log audits.
- **Exclusive registry lock (`registries/gates.toml.lock`):** Recording and the `reaffirm` mode (`N29bp1-01`) both serialize under an exclusive-create lock file storing holder session, PID, and timestamp, removed on exit. Contending recorders wait up to a bounded timeout and then refuse naming the holder; a stale lock whose PID has exited is reported with its holder and never silently broken.
- **Same-run controls, due filter, and `pilot:` parts:** On `pass`, every listed control with `due <= part.milestone` must appear `caught` in the receipt(s) (`control-missing-from-receipt`, `control-not-caught`); future-milestone controls are reported `not-yet-due`, and a part whose listed controls are all not yet due refuses with `pass-without-control` (`N38b-04`). A part whose `scope` begins `pilot:` is never recorded (`pilot-part-not-recordable`; pilot verdicts belong strictly to `registry/pilots.toml`). For `GATE-LEMMA-03` parts after M1, the recording owner is the milestone exit review bead (`BD-224`, `BD-319`, `BD-378`, `BD-412`, `BD-442`), recorded by the review session before the exit runner starts.
- **Receipts, receipt sets, hooks, and non-killing:** Every cited receipt (and for multi-tier parts, the canonical `frob.gauntlet.receipt-set/1` manifest with members in tier order `G0`..`G5`) is committed byte-identical at `receipts/gates/<hash>.json`. Step 7 runs the `N40-01` register validator followed by every pure hook registered in `register::record::CHECKS` in registration order; the first refusal exits `3` with `reason=<id>` and writes nothing. The recorder never writes status `killed`; `killed` is derived strictly by `register::kill::apply_decisions`.

### Pilot Register Contract (`N38a-09`, `frob.gauntlet.pilot/1`, `GATE-GAUNT-08`)

`src/pilots/` and `examples/pilots.rs` (`cargo run -p frob-conformance --example pilots -- ...`) own the pilot register (`registry/pilots.toml`), seed list (`registry/pilots_expected.toml`), and generated printout (`docs/delivery/pilots.md`):
- **One file, one schema, append-only IDs:** Only `frob.gauntlet.pilot/1` is accepted (`frob.pilot/1` rejected). Identifiers satisfy `^PILOT-(C-[0-9]{2}(-[a-z]+)?|PT-([0-9]{2}|S[0-9])|[A-Z]{2,14}-[0-9]{2})$`, are append-only, and are never reused or removed.
- **Registration before run & committed receipts:** Every record states four mandatory parts (`falsifier`, `ceiling`, `fallback.ships`, `fallback.kill_gate`) and a lower-tier unconditional `fallback.ships`. Results must match `protocol.blake3`, follow registration in date and `ledger_seq`, and cite a receipt committed byte-identical at `receipts/pilots/<hash>.json`. Changing a protocol file or `protocol.sampling_episodes_per_design` after registration sets effective status to `void`.
- **Convention 15 (pilot script exits):** A pilot's script exits `0` when its measurement was made and its record written whichever status the record carries, and non-zero only when a step could not run, a sanity assertion failed, or a negative control was not caught; the record's status, not the script's exit code, carries the verdict.
- **Per-mechanism & per-bead funding (Round-4 Review Notes 3 & 4):** A pilot's own `bead` is never self-gated (`Fundable`, note `pilot-bead`). A bead listed in no record's `gated_beads` is answered only by the records of its work package that list no `gated_beads` at all (and never blocked by a record naming it in `fallback.beads`), preventing a held falsifier from blocking fallback work. A chapter-required `tier = "S"` record (`required_by` mandatory) may set `fallback.kill_gate = "none"` when its subsection attaches no kill gate, funding on its result alone. At most four tier-`M` pilots may be `running` in one milestone (`reason=moonshot-cap`, exit `3`). Pilots are evidence for funding only and never set or support any product gate.

## 5. Refusals

`frob-conformance` defines no public agent-verb refusal codes (`FROB-REF-nnn`); its checks fail closed with the closed Gauntlet harness reason vocabulary (`N38-REF`, `registry/reasons.toml`, `src/reasons.rs`):
- Control loader: `out-of-order`, `reused`, `removed`, `renumbered`, `no-gate`, `unknown-gate`, `kind-not-closed`, `feature-missing`, `fixture-missing`, `defect-or-expect-empty`, `boundary-not-control`, `unknown-seed-form`.
- Registry check: `no-control`, `dangling-control`, `one-way-link`, `uncaught-control`.
- Full closed harness vocabulary (105 reasons across registry check, control loader/runner/review, tier runner, gate recorder, seeding join, registry diff, close audit, live-dependency helper, status views, known-answer/relation/allocation/counterexample validators, gate-register validator, pilot register, and Moonshot transitions): `crates/frob-conformance/registry/reasons.toml` (`frob.gauntlet.reasons/1`).
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
