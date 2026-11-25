# `frob-core` Contract

`frob-core` is layer 0 (rank 1, tier Solid, cap 8,000 lines with tests) of the FrankenRobotics core workspace.

## Purpose

Provide the shared, dependency-minimal vocabulary that must mean the same thing across every crate in the workspace: content-addressed identities and domain-separated hashing (`id`), evidence classes and the weakest-input-wins claim product (`evidence`), constant-false no-authorization flags (`auth`), typed refusals and the refusal-registry loader (`refusal`, `registry::refusals`), the strict TOML-subset reader (`registry::toml`), the closed SI quantity re-export surface (`qty`), and the opt-in test-support kit (`testkit` behind feature `test-support`).

## Inputs

- Raw byte slices for content-addressed blob hashing (`id::hash_blob`) and strict TOML-subset parsing (`registry::toml::parse`, capped at 16 MiB and nesting depth 64).
- Validated `SchemaName` values and opaque `CanonicalBytes` values for domain-separated artifact identity (`id::identity`).
- Leaf evidence parts, scopes, determinism classes, physics tiers, reality scopes, and condition flag sets combined through `evidence::ClaimProduct::combine`.
- JSON objects representing serialized `NoAuthorization`, `PacketAuthorization`, and `Refusal` records.
- `registries/refusals.toml` (embedded at compile time and also accepted as synthetic byte slices for tests) and verb refusal tables validated against the registry.
- SI scalar values for the closed set of 36 `qty` types backed by `fs-qty`.

## Outputs

- 32-byte `Blake3` digests (64 lower-case hexadecimal characters) and typed identity newtypes (`DesignId`, `ModelId`, `SceneId`, `KernelId`, `PolicyId`, `EpisodeId`) prefixed `d:`, `m:`, `s:`, `k:`, `p:`, `e:`.
- `ClaimProduct` values whose class, tier, determinism, scope, reality scope, condition set, and sorted inputs are computed solely by weakest-input-wins combination, carrying `NoAuthorization`.
- Canonical JSON serializations of `NoAuthorization` (three `false` fields), `PacketAuthorization` (four `false` fields), and `Refusal` (omitting `detail` when absent).
- Validated `RefusalRegistry` structures and deterministic Rust source output (`render_refusal_code_enum`) matching `src/refusal/code_gen.rs`.
- Typed `registry::toml::Table` / `Fields` values and closed-set SI quantity wrappers (`qty`).

## Invariants

- **Zero `unsafe`, zero stdout/stderr printing (L-7), zero `serde`, zero async runtime (L-5).** Dependencies are strictly `std`, `fs-blake3`, and `fs-qty`.
- **Weakest input wins by type (INV-NUM-06, INV-LEMMA-02):** `EvidenceClass` is ordered `Estimated < Tested < Certified < Proved`; `ClaimProduct::combine` takes the minimum class (capped at `Tested` for any non-`Proof` claim), weakest `DetClass` (`D4 < D3 < D2 < D1 < D0`), meet of `TierMeet` (`Static > Referee > {BatchCpu, GpuCuda, GpuMetal}` with set join across incomparable fast tiers), intersection of `Scope`, `NoClaim` unless every part shares one calibrated reality family, and union of `FlagSet` conditions (no drop/remove API).
- **No upward conversion (GATE-LEMMA-06):** No `From<ClaimProduct>` into `Evidence<K>`, no setter on `class`, and no public constructor taking an `EvidenceClass`. Doorway constructors (`tested`, `proof_certified`, `proof_proved`, `Evidence::doorway_*`) are `#[doc(hidden)]` and `#[deprecated]` so workspace `deprecated` denial plus `xtask check-unsafe` (N34-07) confine callers to `frob-conformance`, `frob-arena`, and `frob-lemma`.
- **Single-inhabitant no-authorization flags (INV-SAFE-05, PO-SAFE-03):** `False` has one value serializing to JSON `false`; no constructor, setter, or parser produces `true`, and missing fields are refused (`FROB-REF-816`), never defaulted to `false`.
- **Canonical-only identity hashing:** `CanonicalBytes` has no public constructor in BD-005; only the canonical writer and checked-canonical reader path (N34-18) construct it.
- **Closed refusal registry:** `RefusalCode` is generated from `registries/refusals.toml` into `src/refusal/code_gen.rs`; unknown codes, duplicates, unknown TOML keys, overlapping or dangling exception lists, and `[exit_rule]` / family-exit mismatches are load-time errors.
- **Closed quantity surface (N33c-03):** `qty` exports at most 64 named SI types (currently 36), separates equal-dimension kinds (`Torque` vs `Energy` vs `AngularStiffness`, etc.), and never exports generic `fs_qty::Qty`.

## Refusals

- `FROB-REF-802` (exit 3, `after-change`): invalid caller argument, such as an empty `parts` slice passed to `ClaimProduct::combine`.
- `FROB-REF-816` (exit 3, `after-change`): missing, duplicate, unknown, non-boolean, or `true` no-authorization flag encountered while parsing `NoAuthorization` or `PacketAuthorization`.
- `FROB-REF-881` (exit 3, `after-change`): quantifier-prefix mismatch across parts passed to `ClaimProduct::combine`.

## Determinism class

`D0` (Exact). Every operation in `frob-core` is a pure function of its inputs with deterministic tie-breaking (lowest content hash, then field path), sorted sets/maps (`BTreeSet`/`BTreeMap`), and bit-exact BLAKE3 hashing.

## Evidence class of outputs

`Estimated`. `frob-core` creates `Estimated` leaves publicly and supplies the sealed doorway functions through which `frob-conformance` / `frob-arena` emit `Tested` leaves and `frob-lemma` emits `Certified` / `Proved` `Proof` leaves.

## Validity domain

- `id::SchemaName`: non-empty lower-case ASCII (`[a-z0-9.-]`) starting with `[a-z]`, `major >= 1`.
- `id::Blake3` / typed IDs: 32 bytes / exactly 64 lower-case hex digits with closed single-letter prefix (`d:`, `m:`, `s:`, `k:`, `p:`, `e:`).
- `registry::toml`: UTF-8 without BOM, `<= 16 MiB`, nesting depth `<= 64`, strict subset of TOML 1.0.0.
- `registry::refusals`: codes `FROB-REF-100`..`FROB-REF-999`, valid surface exit codes `{3, 4, 5, 6, 7, 9}`, `9xx` fixed to exit 9 with `retry = "never"`.
- `qty`: finite SI `f64` scalars over the closed 36-type robotics set; runtime-checked `Checked` conversions validate exact 6-component SI dimension vectors.

## Oracles

- Official BLAKE3 empty-input test vector (`af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262`) and `fs-blake3` derive-key domain separation.
- Vendored `toml-test` 1.0.0 conformance corpus (717 cases: 208 valid, 509 invalid) in `tests/toml_corpus.rs` and `scripts/e2e/toml_reader.sh`.
- Compile-fail doctests with pinned `rustc` error codes verifying absent conversions, private fields, and immutable `False` flags.
- Unit-test conventions (`STD-TEST`): [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md) governs test placement, registry-bound naming, the no-mocks rule, and determinism lanes.

## Known gaps

- `CanonicalBytes` constructors (`CanonWriter::finish` and `Admitted::into_canonical`) land in N34-18; BD-005 exposes no public constructor.
- The strict JSON admission reader lands in N34-17; `auth` and `refusal` currently include minimal std-only JSON parsing for their own closed objects.
- `registries/refusals.toml` is seeded with `FROB-REF-802`, `FROB-REF-816`, and `FROB-REF-881` plus the fixed surface `[exit_rule]`; N35-33 and chapter registry beads populate the full 294+ rows, `74x`/`76x`/`79x` rules, and exception lists.
- Full `DetProvenance` and kernel-class mapping land in N35-01; artifact-graph IDs (`clm:`, `ev:`) land in N34-11; L-10 inversion traits land in N34-06; registry-generated `qty` product tables and `frob-units` escape hatch land in N33c-03 and N33c-05.
