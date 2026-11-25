# `frob-core` Contract

`frob-core` is layer 0 (rank 1, tier Solid, cap 16,000 lines with tests per ADR-WS-04) of the FrankenRobotics core workspace.

## Purpose

Provide the shared, dependency-minimal vocabulary that must mean the same thing across every crate in the workspace: content-addressed identities and domain-separated hashing (`id`), canonical JSON serialization from typed terms with explicit code-point sorting (`canon`), evidence classes and the weakest-input-wins claim product (`evidence`), constant-false no-authorization flags (`auth`), raw-byte strict JSON admission (`json`), plain mission proof objects (`proof`), performance workload records and typed throughput units (`perf::workload`, `perf::units`), build-embedded stack identities and the same-stack rule (`stack`), typed refusals and the refusal-registry loader (`refusal`, `registry::refusals`), the strict TOML-subset reader (`registry::toml`), the closed SI quantity re-export surface (`qty`), and the opt-in test-support kit (`testkit` behind feature `test-support`).

## Inputs

- Raw byte slices for content-addressed blob hashing (`id::hash_blob`), raw-byte JSON admission (`json::admit`), and strict TOML-subset parsing (`registry::toml::parse`, capped at 16 MiB and nesting depth 64).
- Validated `SchemaName` values, typed `canon::Canonical` terms, and opaque `CanonicalBytes` values for canonical encoding (`canon::CanonWriter`, `json::Admitted::into_canonical`) and domain-separated artifact identity (`id::identity`).
- Leaf evidence parts, scopes, determinism classes, physics tiers, reality scopes, and condition flag sets combined through `evidence::ClaimProduct::combine`.
- JSON objects representing serialized `NoAuthorization`, `PacketAuthorization`, and `Refusal` records.
- `registries/refusals.toml` (embedded at compile time and also accepted as synthetic byte slices for tests) and verb refusal tables validated against the registry.
- SI scalar values for the closed set of 36 `qty` types backed by `fs-qty`.
- Exact rationals, outward-rounded intervals, typed frame/unit quantities, ordered port sequences, quantifier prefixes, expression DAGs, assumptions, dependency supports, and per-kind payload structs for `proof::ProofObject::new`.
- At compile time (`build.rs`), the two one-line 64-hex files `receipts/HEAD` and `receipts/LOCK_DIGEST` (maintained by `xtask check-stack --write-receipt`) plus the `FROB_STACK_DEV` environment variable.
- Typed throughput inputs (`perf::units`, `N36-03`): positive reduced `Decimation` (`d >= 1`), positive reduced `Rational` physics step `h`, `StepScope`, `Inclusion`, `Settings` (`Matched`/`Native`), `StepCounts` (`valid`, `degraded`, `faulted`), submission-to-completion `wall_s`, `HardwareKey` groups, and raw-byte rate JSON payloads admitted via `frob_core::json`.

## Outputs

- 32-byte `Blake3` digests (64 lower-case hexadecimal characters), opaque `CanonicalBytes` values (constructed solely by `canon::CanonWriter::finish` and `json::Admitted::into_canonical`), and typed identity newtypes (`DesignId`, `ModelId`, `SceneId`, `KernelId`, `PolicyId`, `EpisodeId`) prefixed `d:`, `m:`, `s:`, `k:`, `p:`, `e:`.
- `ClaimProduct` values whose class, tier, determinism, scope, reality scope, condition set, and sorted inputs are computed solely by weakest-input-wins combination, carrying `NoAuthorization`.
- Canonical JSON serializations of `NoAuthorization` (three `false` fields), `PacketAuthorization` (four `false` fields), and `Refusal` (omitting `detail` when absent).
- Validated `RefusalRegistry` structures and deterministic Rust source output (`render_refusal_code_enum`) matching `src/refusal/code_gen.rs`.
- Typed `registry::toml::Table` / `Fields` values and closed-set SI quantity wrappers (`qty`).
- Plain mission proof objects (`proof` module: fifteen `ProofKind` variants, per-kind exact-number `ProofPayload`, `Establishes`, canonical `ExprDag`, `Support`, `ProofObject`, and deterministic D0 canonical bytes/digests) — class `"none: data only"` (`N25b-01`, `OPEN-AUGUR-6`, `ADR-WS-02`).
- Typed stack identities (`stack` module: `StackReceiptHash` prefixed `sr:`, `StackLockDigest` prefixed `sl:`, `StackIdentity { receipt, lock, dev }`, compile-time constant `BUILT_UNDER`, `same_stack`, `require_same_stack`, and `StackKeyed`) serialized via `frob_core::canon` (`N33a-06`).
- Typed throughput rates and pure checks (`perf::units`, `N36-03`): `Decimation`, `StepScope`, `Inclusion`, `Settings`, `RateUnit`, `StepUnit`, `StepCounts`, `Rate`, `ExactRational`, `HardwareKey`, `KeyedRate`, exact count/unit conversions (`env_steps_to_world_steps`, `world_steps_to_env_steps`, `world_steps_to_world_seconds`), `comparable`, `check_ordering` / `check_group_ordering` (`OrderingViolation`), canonical wire JSON (`to_canonical_json`, `parse_rate_at`, `parse_rates_json`), and named refusal constants (`ORDERING_DETAIL`, `ORDERING_POINTER`, `ORDERING_INTERIM_REFUSAL_CODE`).

## Invariants

- **Zero `unsafe`, zero stdout/stderr printing (L-7), zero `serde`, zero async runtime (L-5).** Dependencies are strictly `std`, `fs-blake3`, and `fs-qty`.
- **Weakest input wins by type (INV-NUM-06, INV-LEMMA-02):** `EvidenceClass` is ordered `Estimated < Tested < Certified < Proved`; `ClaimProduct::combine` takes the minimum class (capped at `Tested` for any non-`Proof` claim), weakest `DetClass` (`D4 < D3 < D2 < D1 < D0`), meet of `TierMeet` (`Static > Referee > {BatchCpu, GpuCuda, GpuMetal}` with set join across incomparable fast tiers), intersection of `Scope`, `NoClaim` unless every part shares one calibrated reality family, and union of `FlagSet` conditions (no drop/remove API).
- **No upward conversion (GATE-LEMMA-06):** No `From<ClaimProduct>` into `Evidence<K>`, no setter on `class`, and no public constructor taking an `EvidenceClass`. Doorway constructors (`tested`, `proof_certified`, `proof_proved`, `Evidence::doorway_*`) are `#[doc(hidden)]` and `#[deprecated]` so workspace `deprecated` denial plus `xtask check-unsafe` (N34-07) confine callers to `frob-conformance`, `frob-arena`, and `frob-lemma`.
- **Single-inhabitant no-authorization flags (INV-SAFE-05, PO-SAFE-03):** `False` has one value serializing to JSON `false`; no constructor, setter, or parser produces `true`, and missing fields are refused (`FROB-REF-816`), never defaulted to `false`.
- **Canonical-only identity hashing (N34-18):** `CanonicalBytes` has no public struct literal or `From<Vec<u8>>` constructor; only `canon::CanonWriter::finish` and `json::Admitted::into_canonical` construct it, enforcing explicit Unicode code-point member sorting, minimal escapes, and exact number rules.
- **Closed refusal registry:** `RefusalCode` is generated from `registries/refusals.toml` into `src/refusal/code_gen.rs`; unknown codes, duplicates, unknown TOML keys, overlapping or dangling exception lists, and `[exit_rule]` / family-exit mismatches are load-time errors.
- **Closed quantity surface (N33c-03):** `qty` exports at most 64 named SI types (currently 36), separates equal-dimension kinds (`Torque` vs `Energy` vs `AngularStiffness`, etc.), and never exports generic `fs_qty::Qty`.
- **Distinct stack identities, fail-closed embedding, and the same-stack rule (N33a-06, INV-SUITE-05):** `StackReceiptHash` (`sr:<64 lower-case hex>`, wide identity of the resolved `frob.stack-receipt/1` JSON) and `StackLockDigest` (`sl:<64 lower-case hex>`, narrow identity of canonical `franken-stack.lock` mixed into airlock memo keys via `StackKeyed`) are distinct Rust types with no cross-conversion or arbitrary-string constructor outside their parsers. `build.rs` reads `receipts/HEAD` and `receipts/LOCK_DIGEST` without hashing or reading siblings, declares `rerun-if-changed` on both files and `franken-stack.lock` plus `rerun-if-env-changed=FROB_STACK_DEV`, and fails closed when either file is absent unless `FROB_STACK_DEV=1` is set (marking `BUILT_UNDER.dev = true` and zeroing missing fields). `same_stack(a, b)` holds iff `!a.dev && !b.dev && a.receipt == b.receipt` (never equal lock digests alone, never any `dev: true` identity); `require_same_stack` raises `FROB-REF-890` with both receipt hashes, `retry: after-change`, and ranked repairs (`result.revalidate`, `rerun`, `present-separately`).
- **Typed throughput units and INV-PERF-01 ordering (`perf::units`, ADR-PERF-02, N36-03, KA-806, CX-374):** Step-family rates (`WorldStepsPerSec`, `EnvStepsPerSec`, `WorldSecPerWallSec`) cannot be constructed without a non-optional reduced `Decimation` (`num >= den >= 1`) and `h: Rational`, whereas `RealTimeMultiple` and `EpisodesPerSec` take none. `Rate` derives `value` strictly from `counts.valid / wall_s` (`degraded` and `faulted` steps never enter the numerator) and enforces `StepScope` / `Inclusion` agreement. `check_ordering` enforces `training <= full_env_step <= physics_only` in `env-steps/s` per `HardwareKey` strictly with zero noise tolerance. `comparable(a, b)` requires a shared `StepScope` and either equal `Matched { h, d, newton_iterations }` or `EnvStepsPerSec` under `Native` settings (refusing `Native` world-steps comparisons across decimations).

## Refusals

- `FROB-REF-802` (exit 3, `after-change`): invalid caller argument, such as an empty `parts` slice passed to `ClaimProduct::combine`.
- `FROB-REF-816` (exit 3, `after-change`): missing, duplicate, unknown, non-boolean, or `true` no-authorization flag encountered while parsing `NoAuthorization` or `PacketAuthorization`.
- `FROB-REF-881` (exit 3, `after-change`): quantifier-prefix mismatch across parts passed to `ClaimProduct::combine`.
- `FROB-REF-890` (exit 3, `after-change`): `stack-mismatch` — evidence recorded under another stack receipt or a `dev: true` identity is offered as same-stack evidence without revalidation (`stack::require_same_stack`).
- Typed boundary errors in `perf::units` (`RateSchemaError`, `OrderingViolation`; no refusal object constructed in `perf::units`): mapped at disclosure and report validation boundaries (`N36-13`, `BD-216`, `N36-REF`) to `FROB-REF-862` (exit 3) — missing decimation at `rates[i].decimation` with repair `"restate the figure with its decimation"`, and INV-PERF-01 ordering violations as the interim of `rates-violate-unit-ordering` behind `ORDERING_DETAIL = "rates-violate-unit-ordering"` and `ORDERING_POINTER = "rates.ordering"` with repair `"more pairs, or a longer duration"`.

## Determinism class

`D0` (Exact). Every operation in `frob-core` — including `perf::units` count/unit rational conversions (`ExactRational`, `Decimation`, `Rational`), comparability, and INV-PERF-01 ordering checks — is a pure function of its inputs with deterministic tie-breaking (lowest content hash, then field path), sorted sets/maps (`BTreeSet`/`BTreeMap`), and bit-exact BLAKE3 hashing. Measured rate values ingested into `perf::units::Rate` carry evidence class `Tested` at the measurement boundary.

## Evidence class of outputs

`Estimated`. `frob-core` creates `Estimated` leaves publicly and supplies the sealed doorway functions through which `frob-conformance` / `frob-arena` emit `Tested` leaves and `frob-lemma` emits `Certified` / `Proved` `Proof` leaves.

## Validity domain

- `id::SchemaName`: non-empty lower-case ASCII (`[a-z0-9.-]`) starting with `[a-z]`, `major >= 1`.
- `id::Blake3` / typed IDs: 32 bytes / exactly 64 lower-case hex digits with closed single-letter prefix (`d:`, `m:`, `s:`, `k:`, `p:`, `e:`).
- `registry::toml`: UTF-8 without BOM, `<= 16 MiB`, nesting depth `<= 64`, strict subset of TOML 1.0.0.
- `registry::refusals`: codes `FROB-REF-[1-9][0-9]{2,3}` (3-digit `100..=999` and 4-digit family continuations `1000..=9999` per `ADR-NUM-refusal-8xx`, compared as whole tokens), valid surface exit codes `{3, 4, 5, 6, 7, 9}`, `9xx` fixed to exit 9 with `retry = "never"`.
- `qty`: finite SI `f64` scalars over the closed 36-type robotics set; runtime-checked `Checked` conversions validate exact 6-component SI dimension vectors.

## Oracles

- Official BLAKE3 empty-input test vector (`af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262`) and `fs-blake3` derive-key domain separation.
- Vendored `toml-test` 1.0.0 conformance corpus (717 cases: 208 valid, 509 invalid) in `tests/toml_corpus.rs` and `scripts/e2e/toml_reader.sh`.
- Compile-fail doctests with pinned `rustc` error codes verifying absent conversions, private fields, and immutable `False` flags.
- Unit-test conventions (`STD-TEST`): [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, and determinism lanes.

## Known gaps

- `frob_core::json` (N34-17) and `frob_core::canon` + `Admitted::into_canonical` (N34-18) are implemented; `auth` and `refusal` still include minimal std-only JSON parsing for their own closed objects until surface wiring in N34-20; the four-way feature build and golden corpus live in N34-21.
- `registries/refusals.toml` is seeded with `FROB-REF-802`, `FROB-REF-816`, `FROB-REF-881`, and `FROB-REF-890` plus the fixed surface `[exit_rule]`; N35-33 and chapter registry beads populate the full 294+ rows, `74x`/`76x`/`79x` rules, and exception lists.
- Full `DetProvenance` and kernel-class mapping land in N35-01; artifact-graph IDs (`clm:`, `ev:`) land in N34-11; L-10 inversion traits land in N34-06; registry-generated `qty` product tables and `frob-units` escape hatch land in N33c-03 and N33c-05.
