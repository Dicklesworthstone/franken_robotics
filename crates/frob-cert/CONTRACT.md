# `frob-cert` Contract (`CONTRACT.md`)

## 1. Purpose

`frob-cert` (layer 6, subsystem Lemma) and its standalone binary `frob-verify` form the independent offline certificate auditor for FrankenRobotics (ADR-WS-02, ADR-LEMMA-03, ADR-AUGUR-04, ADR-AUGUR-05). They re-check `.frobcert` bundles against the Rust standard library and nothing else, without sharing a parser, hash implementation, arithmetic library, or units crate with any proposer, simulator, or core library.

## 2. Inputs

- Byte sequences representing canonical JSON certificate bundles (`.frobcert`), supplied either directly to the library API (`frob_cert::audit_bytes`, `frob_cert::json::parse_canonical`) or as file paths on the `frob-verify` command line.
- Optional `--reserialize <path>` output file path and `--json` output flag on `frob-verify`.
- Exact integer strings (`"p"`), rational pairs (`["p","q"]`), slash rationals (`"p/q"` or `"p"`), and dyadic precision declarations `p` (`u32`) for interval arithmetic (`frob-cert/dyadic-1`).
- `frob-verify` reads no environment variables, opens no network sockets, and spawns no child processes.

## 3. Outputs

- Library verdicts (`Verdict::Accept`, `Verdict::Reject { rule, refusal, path, message }`, `Verdict::Unsupported { kind, message }`) together with the canonical BLAKE3 digest, the compiled limits table, and the trusted-base statement.
- Re-serialized canonical JSON bytes (byte-identical to any accepted canonical input; PO-LEMMA-01, KA-222).
- `frob-verify` text or JSON verdict report on stdout, optional canonical re-serialization to `--reserialize <path>`, and exit codes: `0` (all accepted), `2` (none rejected, at least one unsupported), `3` (any rejected, unreadable input, or usage error).

## 4. Invariants

- **Zero dependencies (ADR-WS-02, INV-WS-02, GATE-WS-03):** `crates/frob-cert/Cargo.toml` has empty `[dependencies]` and `[dev-dependencies]` tables and inherits no keys from the workspace (`workspace = true` is absent). A standalone copy of `crates/frob-cert/` builds offline with `cargo build --release --offline` on the pinned stable release named in `crates/frob-cert/rust-toolchain.toml` (`1.99.0`), and the same sources build as a workspace member on toolchain `T`.
- **Memory safety:** `#![forbid(unsafe_code)]` at both library and binary crate roots.
- **Process/network/environment isolation:** No use of `std::net`, `std::process::Command`, or environment-variable readers (`std::env::var*`, `std::env::current_dir`, `std::env::temp_dir`); library modules contain no `std::env` references at all.
- **No floating point:** Neither `f32` nor `f64` appears anywhere in `frob-cert`. All arithmetic is exact arbitrary-precision integer/rational arithmetic or outward-rounded dyadic interval arithmetic (`frob-cert/dyadic-1`).
- **Canonical uniqueness:** Accepted JSON inputs have no UTF-8 BOM, no insignificant whitespace, no trailing newline, unique decoded member names at every level, keys in ascending Unicode code-point order, minimal string escapes (`\"`, `\\`, `\u00XX` lowercase for `< 0x20`), and integers within `±(2^53 - 1)` only.
- **Closed proof-kind dispatch:** Unknown or unregistered proof kinds are rejected (`kind.unknown`), never accepted on hash consistency alone.
- **Strict resource bounds:** Every integer operand/result above `16,384` bits, JSON depth above `64`, numeric count above `1,000,000`, or bundle file above `64 MiB` fails closed with `limits.exceeded`.

## 5. Refusals

- `limits.exceeded`: file > 64 MiB, JSON nesting > 64, numeric entries > 1,000,000, or integer > 16,384 bits.
- `input.unreadable`: named input file cannot be opened or read.
- `json.bom`, `json.syntax`, `json.duplicate-key`, `json.key-order`, `json.whitespace`, `json.escape`: canonical JSON violations.
- `num.float-in-bundle`, `num.int-range`, `num.int-form`, `num.rational-form` (`FROB-REF-849`): malformed or non-exact certificate arithmetic (floating-point JSON numbers, integers outside `±(2^53 - 1)`, leading zeros, `-0`, non-reduced rationals, non-positive denominators, `"p/1"` slash form).
- `kind.unknown`: bundle names an unknown or unregistered proof kind.

## 6. Determinism Class

**D0 (bit-exact deterministic across machines, operating systems, and build profiles).** Identical input bytes yield identical verdicts, rules, digests, and re-serialized bytes on macOS arm64 and Linux x86-64 in both debug and release builds.

## 7. Evidence Class of Outputs

The auditor produces **verdicts**, not claims. It does not run physics or attest that a real-world experiment occurred. When a proof-kind checker (added in downstream beads) accepts a bundle, the evidence class of that bundle is the class stated by the bundle capped by its proof kind's registry row. Every report states the trusted computing base: `frob-verify`, the Rust standard library, and the Rust compiler.

## 8. Validity Domain

- Canonical JSON documents up to `64 MiB` and nesting depth `64`.
- Arbitrary-precision integers up to `16,384` bits; exact rationals over `Q` within that bit limit.
- Outward-rounded dyadic intervals `m * 2^-p` (`frob-cert/dyadic-1`) for operations: addition, subtraction, negation, multiplication, division excluding zero, non-negative integer powers, non-negative square root, absolute value, minimum, and maximum.
- Cryptographic BLAKE3 hashing in plain, keyed, and derive-key modes with arbitrary XOF output length.

## 9. Oracles

- **BLAKE3 (`KA-223`):** Official BLAKE3 published test vectors checked in under `tests/vectors/blake3_official_vectors.json` (plain, keyed, and derive-key modes with extended 131-byte output across all standard boundary input lengths).
- **Exact arithmetic (`PO-AUGUR-02`):** Independent in-crate schoolbook reference implementation in `tests/` differential-tested across `10^6` seeded random operations and fixture certificate values.
- **Interval outward rounding (`PO-AUGUR-03`):** Property tests verifying that exact rational results lie inside every computed dyadic enclosure and that each bound is within 1 ULP (`2^-p`) of the exact bound, plus the stance-dual worked case (`[14/3, 6]` vs `[47/10, 6]` at `2^-40`).
- **Canonical round-trip (`KA-222`):** Byte-exact re-serialization and negative-control mutation builds (`nc-skip-key-order`, `nc-interval-mul-nearest`, `nc-dup-key-first-wins`).
- **Cross-implementation oracles (downstream, `N12-10`):** Differential comparison in `frob-conformance` against `fs-blake3` and FrankenSymPy.
- **Test and evidence conventions (`STD-TEST`):** All unit, property, negative-control, and end-to-end tests in `frob-cert` follow `docs/testing/CONVENTIONS.md`.

## 10. Known Gaps

- The closed proof-kind registry (`frob.cert.kinds/1`, 27 kinds, `RuleId` catalogue, `FROB-REF-823` / `FROB-REF-834`, closed `claim.kind` scope table) is compiled in (`N12-02`), while individual proof-kind checkers (`BD-027`, `BD-132`, `N12-13`..`N12-19`, `N12p1-01`, `N12p2-02`, `BD-381`), full bundle schema validation (`N12-03`), and the expanded report/exit CLI (`N12-04`) land in downstream beads; unregistered kinds reject (`kind.unknown`), and registered `not-implemented` kinds return `unsupported`, never `accept`.
- Transcendental functions (`sin`, `cos`, `exp`, `log`) are intentionally outside the M0 interval set and yield `unsupported` unless rewritten in algebraic form by the producer.
- Multiplication and division use straightforward schoolbook and Knuth Algorithm D routines (within the 12,000-line readability cap); asymptotically faster algorithms (e.g., Karatsuba) are deferred unless profiling shows a certificate budget breach.
- The fourth clause of `GATE-WS-03` (corpus-wide verdict parity between standalone `frob-verify` and in-daemon `prove.verify`) is measured at M1 by `N25bp4-01`.
