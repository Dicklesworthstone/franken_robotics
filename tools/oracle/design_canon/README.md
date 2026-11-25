# Independent Reference Canonicalizer for `frob.design/1` (`N14a-26`)

This directory contains the standalone, out-of-process reference canonicalizer
that pins canonical bytes and `DesignId` values for the `frob.design/1` golden
corpus under `crates/frob-conformance/fixtures/design_golden/`.

## Independence Guarantees

- Written in Python 3 using **only** the Python standard library (`fractions`,
  `struct`, `re`, `decimal` for planted binary64 mutants only).
- Implements BLAKE3 from the published specification (`blake3_ref.py`) and
  verifies itself against the official published BLAKE3 hash and `derive_key`
  test vectors (`selftest.sh`).
- Holds its own explicit coherent-SI unit table (`units.py`) and explicit
  table of written-out schema defaults (`canon.py`).
- Lives completely outside the Rust/Cargo build graph and shares **no** code,
  schema file, JSON parser, or unit table with `frob-core` or `frob-design`.
- Produces byte-identical canonical bytes and `DesignId`s on Linux and macOS.

## Files

| File | Role |
|---|---|
| `blake3_ref.py` | Pure-stdlib BLAKE3 (`hash_bytes`, `derive_key`, `design_id`) + published test vectors |
| `units.py` | Independent exact rational unit table, `shortest_decimal`, `format_turn_ratio` |
| `canon.py` | Clean-room strict JSON parser, `frob.design/1` canonicalizer, minimal-escape writer, and 4 planted wrong modes |
| `selftest.sh` | Runs BLAKE3 vectors, `shortest_decimal`, `turn_ratio`, duplicate-key, and escaping self-tests |
| `regenerate.sh` | The **only** legal producer of `*.canon` and `manifest.toml` expectations |

## Running Self-Tests and Regeneration

```bash
# Run reference self-tests (BLAKE3 vectors, exact decimal/turn printing, duplicate-key refusal)
bash tools/oracle/design_canon/selftest.sh

# Re-verify golden corpus into a scratch directory (without touching the tree)
bash tools/oracle/design_canon/regenerate.sh --out-dir "${BIG_SCRATCH:-$TMPDIR}/design_golden_check"

# Regenerate stored expectations in-tree (ONLY under one of the two legal reasons below)
bash tools/oracle/design_canon/regenerate.sh
```

## Legal Procedure for Changing a Golden Value

A stored `.canon` expectation or `DesignId` in
`crates/frob-conformance/fixtures/design_golden/` may change **only** for one
of two legal reasons:

1. **Deliberate format-rule change with migration.** The `frob.design` format
   rules change deliberately under a new canonicalization revision and
   accompanying migration record; prior-revision expectations remain pinned so
   historical `DesignId`s continue to verify.
2. **Confirmed defect in the reference canonicalizer.** A genuine bug is
   discovered in `tools/oracle/design_canon/` against the format specification,
   the defect is fixed in the reference **first** (with a new regression case in
   `selftest.sh`), `regenerate.sh` is re-run, and the resulting diff of golden
   expectations is reviewed by someone who did **not** write the fix.

**A golden value is NEVER updated to match the product implementation.** If the
product (`frob-design` / `frob-core`) disagrees with a stored golden byte or
`DesignId`, treat it as a finding against the product unless the format
specification itself proves the reference wrong under reason (2).
