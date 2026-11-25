# FrankenRobotics Unit-Test Conventions (`STD-TEST`)

This document states the binding unit-test conventions for every crate in FrankenRobotics, enforced mechanically by `frob_core::testkit` (behind the `test-support` feature of `frob-core`) and `crates/frob-conformance/tests/std_test_lint.rs`.

The conventions prevent fake progress: every test runs the real component, every non-real fixture declares its kind (`synthetic`, `oracle`, or `recorded`), every known answer is a numbered registry row with a derivation, and no gate can print `pass` without catching a registered planted defect in the same run ("a gate that no deficient model can fail validates nothing").

---

## 1. Where Tests Live

- **Private logic:** inline `#[cfg(test)] mod tests` in the owning module.
- **Public surface:** integration tests under `crates/<crate>/tests/*.rs`.
- **Crate contract:** `crates/<crate>/tests/conformance.rs`.
- **Execution:** run via `cargo test -p <crate>` through the build-offload shim (`rch`).
- **Resource budget:** every unit or integration test finishes inside the default machine envelope (`16 GiB` RAM, `8` cores, `1 hour`). Anything longer belongs in a Gauntlet tier test target in `frob-conformance` (`g0_laws`, `g1_known_answers`, `g2_parity`, `g3_metamorphic`, `g4_chaos`, `g5_determinism`) with its explicit budget named.
- **Shared test support (`frob_core::testkit`):** lives inside `frob-core` behind the Cargo feature `test-support` (off by default). Crates enable `test-support` only in `[dev-dependencies]`; `frob-conformance` alone may enable `test-support` as a normal dependency so its commands and validators share the single registry parsers of Section 5. Product binaries and workers never enable `test-support`. `frob_core::testkit` depends only on the Rust standard library and `frob-core`.

---

## 2. Names and Registry Binding

Test functions use `snake_case` with the subject first: `<subject>_<behaviour>[_<condition>]`.

Tests bound to registry rows, laws, invariants, proof obligations, determinism lanes, or refusal codes use fixed prefixes checkable directly from `cargo test -- --list` (no procedural macro crate):

| Prefix | Bound to | Example |
|---|---|---|
| `ka_<nnn>_` | Known answer `KA-nnn` | `ka_501_docking_grid_344_217_9` |
| `cx_<nnn>_` | Counterexample `CX-nnn`, shown to break the shortcut | `cx_262_euler_order_spectral_radius` |
| `nc_<nnnn>_` | Negative control `NC-nnnn`, shown caught | `nc_0412_wheel_loop_mutation_caught` |
| `mr_<id>_` | Metamorphic relation `MR-nn`, or carried relation `MR-Cnn` (`mr_c01_`) | `mr_06_similarity_rescaling_bits` |
| `inv_<area>_<nn>_`, `po_<area>_<nn>_` | An invariant or proof obligation | `inv_kin_04_batch_independence` |
| `law_` | An algebraic law (tier G0) | `law_se3_adjoint_homomorphism` |
| `det_<lane>_` | A determinism lane | `det_twice_referee_digest` |
| `refusal_<code>_` | A refusal code | `refusal_532_unreachable_verdict` |

A `#[ignore]` attribute on any test must carry a non-empty reason string (`#[ignore = "..."]`).

---

## 3. No Mocks and No Fakes

A test calls the real function, crate, or binary.

- **Forbidden:**
  - Mocking crates in any workspace manifest (`mockall`, `mockito`, `wiremock`, `faux`, `double`).
  - A trait implementation written solely to stand in for a collaborator under test.
  - A test that asserts on a stub's own canned answer.
- **Sanctioned stand-ins (named and labelled):**
  - `synthetic`: data constructed analytically to have a known answer.
  - `simulated`: the simulated device used by daemon tests, whose outcomes are a pure function of the world address and kept honest by dual runs against the CPU tier.
  - `development-oracle`: a pinned development oracle comparator.
- **Fixture file headers:** every fixture file under a `fixtures/` directory begins with a header naming its kind:
  - `synthetic`
  - `oracle` (stating its pin, date, and capture script)
  - `recorded`

---

## 4. Property Tests (`testkit::prop::check`)

`testkit::prop::check(name, cases, |g| ...)` provides deterministic, address-keyed property testing with no third-party dependency:

1. **Base seed:** first 8 bytes (little-endian `u64`) of `BLAKE3(crate_name || 0x00 || test_name)`, XORed with `FROB_TEST_SEED` (default `0`).
2. **Address-keyed draws:** draw `k` of stream `s` in case `c` is `mix(seed, c, s, k)` — a counter-based 64-bit hash (SplitMix64 finalizer applied to the four words combined by multiply-xorshift). Adding or reordering streams changes no other stream, and cases may run in any order.
3. **Case count:** default `256` cases; `FROB_PROP_CASES` scales the count. A gate requiring a fixed large count (`10^4`, `10^6`, `10^9`) is a tier test that passes its count explicitly.
4. **Deterministic shrinking (at most `1,000` steps):**
   - Integers halve toward `0`.
   - Floats try `0.0`, `±1.0`, then halve exponent distance toward zero.
   - Vectors drop halves, then single elements, then shrink elements.
   - Tuples shrink left to right.
   - Failure reports print the seed, case index, original input, shrunk input, and exact replay command:
     `FROB_TEST_SEED=<s> FROB_PROP_CASE=<c> cargo test -p <crate> <name>`
5. **Committed regressions:** a discovered failure is appended as `(seed, case)` to `crates/<crate>/tests/regressions/<name>.txt`, committed, and replayed first on every subsequent run.
6. **Boundary enumeration:** every law, invariant, and proof obligation has a property test whose boundary set (identity, zero, signed zero, largest finite, values straddling a comparison) is enumerated before random cases.

---

## 5. Registry Loaders and Number Authority

The Gauntlet registries live as single physical TOML files in `crates/frob-conformance/registry/`:

- `crates/frob-conformance/registry/known_answers.toml` (logical alias: `registries/known_answers.toml`)
- `crates/frob-conformance/registry/controls.toml` (logical alias: `registries/negative_controls.toml`)
- `crates/frob-conformance/registry/relations.toml`
- `crates/frob-conformance/registry/counterexamples.toml`
- (Plus `pilots.toml`, `comparators.toml`, and `allocations.toml`, owned respectively by `N38a-09`, `N38a-04`, and `N38a-40`.)

A second physical copy of any registry, or a chapter-private registry file or row format, is a lint failure.

`testkit::registry::root()` finds the workspace root by walking upward to the directory containing `franken-stack.lock`.

**One reader per file:** `frob_core::testkit::registry` provides the sole parsers for the four registries (`KnownAnswers`, `Controls`, `Relations`, `Counterexamples`), built directly on `frob_core::registry::toml`. Every parsed type carries every required and optional field defined by its schema owner (`N38a-12`, `BD-025`, `N38a-14`, `N38b-05`), and any unknown field is a load error naming the file and row. Identifiers are append-only: duplicate or out-of-order (`KA`, `CX`, `NC`, `MR`) identifiers are load errors.

### Parsed Record Summaries

- **`KnownAnswers::get(id_or_slug) -> Row` (`N38a-12`):**
  - Fields: `id`, `section`, `owner`, `fixture`, `same_as`, `case`, `truth { kind, value }`, `tolerance`, `tier`, `sites`, `control { text, nc, removed_check }`, `derivation`, `label`, `due`, `printed`, `slug`, `safe` (`Option<Safe>`: `"higher"` | `"lower"`).
  - Late rows before number allocation may carry `id = ""` when `slug` is non-empty; `KnownAnswers::get` resolves such rows by `slug`, and a row with both `id` and `slug` empty is a load error.
  - `Tolerance` variants: `Exact | Abs(q) | Rel(q) | PrintedDigits(n) | Gate(id) | ToSet`.
  - `testkit::assert_ka(&row, site, observed)` compares using exact rational/decimal arithmetic (never rounded floating-point subtraction) and reports the site. Under `Tolerance::ToSet`, or when `control.text == "none stated"` or `control.nc.is_empty()`, it reports `obligation` with reason `tolerance` or `control`, never `pass`.
- **`Controls::for_gate(gate)` and `Controls::all() -> Vec<Control>` (`BD-025`):**
  - Fields: `id`, `seed`, `gate` (list), `kind` (`mutation | planted-input | planted-design | planted-world | a-a`), `target`, `defect`, `expect`, `last_seen` (`Option<{ caught, session, date }>`), `feature`, `fixture`, `due`, `permanent` (default `false`), `state` (`Runnable` default | `Obligation`), `removed_check`.
  - `testkit::control(control, || -> ControlOutcome)` runs the planted check and records `Caught`, `Missed`, or `Stale`. A check that fails for a reason other than the record's `expect` is reported `Missed`. On an `Obligation` record, `control()` returns `Obligation` without invoking the closure; obligations count toward nothing.
- **`Relations::get(id) -> RelationRow` (`N38a-14`):**
  - Fields: `id` (`MR-nn` or `MR-Cnn`), `statement`, `source`, `sites`, `compare` (`Bits | Exact | Digest | Bound | Order | LeadingOrder`), `bound`, `mutations` (`Vec<{ name, text, control, feature }>`, parsed from TOML `mutation` as either one table or an array of tables), `entry`, `owner`, `due`, `generated_from` (`Option<String>`).
- **`Counterexamples::get(id) -> Counterexample` (`N38b-05`):**
  - Fields: `id`, `shortcut`, `case`, `numeric`, `plan_case`, `case_supplied_by`, `rule`, `rule_ids`, `defined_in`, `same_as`, `label`, `rerun` (`Option<bool>`, present on `FACT` entries only), `rerun_receipt`, `control_kind`, `plant`, `due`, `slug`.
  - `plant` is a `Vec<Plant>` where `Plant { id, kind, target, defect, expect, gates, feature, fixture, removed_check }` (`id` required, rest optional). A plain string `plant = "SPEC-C-01:cx-019"` loads as `[Plant { id: "SPEC-C-01:cx-019", .. }]`; an empty list represents an obligation with no plant yet.

### Where Numbers Come From

A bead never invents a registry identifier:

- `KA` / `CX` numbers beyond the plan-time ranges (`KA-835`, `CX-384`) come only from blocks issued in `allocations.toml` (`N38a-40`).
- `NC` numbers are appended in order by the seeding tool (`N38b-05`) or by the bead registering a non-seeded control under `BD-025` rules.
- `MR` numbers are owned by `N38a-14`.
- Refusal codes (`FROB-REF-nnn`) come from the allocation ledger (`N35-33`, `N35x-02`).

---

## 6. Determinism Lanes (`testkit::det::lane`)

`testkit::det::lane(lane, class, run) -> LaneResult` evaluates digests of declared state at named checkpoints against the item's declared determinism class (`D0` exact; `D1` bitwise across backends; `D2` bitwise per backend and batch-independent; `D3` within envelope plus decision agreement; `D4` unspecified, never evidence):

| Lane | What varies | `D0`, `D1`, `D2` | `D3` | `D4` |
|---|---|---|---|---|
| `twice` | Nothing: two runs on one machine | Identical bytes | Inside envelope, decisions agree | Not asserted, labelled |
| `workers` | Worker count `1`, `2`, `8`; device shard count | Identical bytes | As above | Not asserted |
| `profiles` | Debug and release builds | Identical bytes (no FMA contraction, no fast-math) | As above | Not asserted |
| `platforms` | `arm64` macOS and `x86-64` Linux | `D0` identical; referee `f64` identical where cross-ISA lands, else `D3` | As above | Not asserted |
| `backends` | `batch-cpu` (`f32`), CUDA, Metal | `D0` identical; `D1` identical where IEEE probe passed; `D2` not cross-backend | Envelope plus decision-level test | Not asserted |

- Any mismatch reports the first differing checkpoint index, step, and quantity name.
- A derived result never claims a determinism class stronger than its weakest input; a class is never raised by relabelling, only by re-running under a stricter profile.
- **D3 decision-level test (TARGET thresholds):** the one-sided 95% exact Clopper-Pearson binomial upper bound on the discordant task-decision rate is at most `1%` (`0.01`; with zero discordant decisions, `1 - 0.05^(1/n) <= 0.01` holds from `n = 299`, rounded to `300` worlds in prose), and on the net rate at most `0.5%` (`0.005`).

---

## 7. Unit-Test Evidence Stream (`frob.testevidence/1`)

When `FROB_TEST_EVIDENCE_DIR` is set, every `testkit` check appends one JSON object per line to `<dir>/<crate>.ndjson` with schema `"frob.testevidence/1"` and fields:

- `ts` (RFC 3339 UTC timestamp with milliseconds)
- `crate`, `test`
- `kind`: `ka | cx | nc | mr | inv | po | det | law | gate`
- `id`
- `status`: `pass | fail | obligation | caught | missed | stale | skip | not-eligible | no-claim`
- `reason`: set on `obligation` (`tolerance`, `control`), on `not-eligible` (`no-control`, `stale-control`), and on `no-claim` (the entry's reason)
- `observed`, `expected`, `tolerance`, `unit`, `tier`, `site`, `seed`, `determinism`, `synthetic`

Tier runners and the gate recorder consume these lines; a unit test never writes a gate status directly into `registries/gates.toml`.

---

## 8. No Caught Control, No Pass (`testkit::GateReport` & `GateLine`)

`testkit::GateReport::new(gate)` collects `.measured(name, value, unit, threshold)` and `.control(id, outcome)` (or `.no_claim(reason)` for Reality-lane gates with no measurements and no controls) and finishes with `.finish() -> GateLine`.

`GateReport::finish` is the **only** function in the workspace permitted to produce the text `pass` for a gate. `GateLine` has no public constructor.

Resolution order in `GateReport::finish()`:

1. `NoClaim(reason)` — produced only via `.no_claim(reason)` with no measurements and no controls (Reality lane, `claim = "reality"`): prints `no-claim: <reason>` and emits evidence status `no-claim`. Never prints `pass`.
2. `Fail` — if any measurement misses its threshold or any control outcome is `Missed`: prints `fail`.
3. `NotEligible(StaleControl { id })` — if any control outcome is `Stale` (even if another control was `Caught`): prints `not-eligible: control <id> stale`.
4. `NotEligible(NoControl)` — if zero controls were `Caught` in this report (none ran, or every control was `Obligation`): prints `not-eligible: no control caught in this run`.
5. `Pass` — otherwise (every measurement met its threshold, no control was `Missed` or `Stale`, and at least one control was `Caught`): prints `pass`.

---

## Enforcement

Run `cargo test -p frob-conformance --test std_test_lint` (and `scripts/e2e/std_test_lint.sh`) to verify all eight structural lint rules across the workspace:

1. No forbidden mocking crates in any `Cargo.toml`.
2. No `#[ignore]` attribute without an explicit reason string.
3. Every `ka_`, `cx_`, `nc_`, or `mr_` test identifier exists in its registry.
4. Every registry row due at or before the current milestone has a bound test or runner entry.
5. Every fixture file carries a valid kind header (`synthetic`, `oracle`, or `recorded`).
6. No gate identifier is printed with `pass` outside `GateReport`.
7. No second physical copy of a registry file exists.
8. No evidence line carries `kind: "gate", status: "pass"` without a matching `caught` control line for the same gate and run.
