#!/usr/bin/env bash
# Self-test for the independent reference canonicalizer (`tools/oracle/design_canon/`, N14a-26).
# Verifies:
#   1. Published BLAKE3 standard hash and derive_key test vectors.
#   2. `shortest_decimal` on 1/8, 1/3, 1/40, 7/6, 3/1, -0, 420/1000, 0.30000000000000001.
#   3. `format_turn_ratio` / angle conversion on 45 deg, 360 deg, -30 deg, -160 deg, 0.125 turn, 1.5708 rad.
#   4. Duplicate-key refusal on `{"a":1,"a":2}` and escaped depth-8 duplicates.
#   5. Minimal string escaping (`\n` -> `\u000a`, U+007F and non-ASCII raw) and code-point key sorting.
#
# Never deletes files or directories (COMMON.md 9b).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"

python3 - "${SCRIPT_DIR}" <<'PY'
from fractions import Fraction
import sys

sys.path.insert(0, sys.argv[1])

import blake3_ref
import canon
import units

# 1. BLAKE3 published vectors (hash + derive_key + domain separation)
blake3_ref.run_blake3_vectors()

# 2. shortest_decimal known answers (N14a-26 test plan + N14a-01)
assert units.shortest_decimal(Fraction(1, 8)) == "0.125"
assert units.shortest_decimal(Fraction(1, 3)) == "1/3"
assert units.shortest_decimal(Fraction(1, 40)) == "0.025"
assert units.shortest_decimal(Fraction(7, 6)) == "7/6"
assert units.shortest_decimal(Fraction(3, 1)) == "3"
assert units.shortest_decimal(Fraction(0, 1)) == "0"
assert units.shortest_decimal(units.parse_exact_magnitude("-0")) == "0"
assert units.canonicalize_quantity_string("420 mm", "length")[0] == "0.42 m"
assert units.canonicalize_quantity_string("2.5 mm", "length")[0] == "0.0025 m"
assert units.canonicalize_quantity_string("1/3 m", "length")[0] == "1/3 m"
assert units.canonicalize_quantity_string("1e-3 m", "length")[0] == "0.001 m"
assert units.canonicalize_quantity_string("-0 m", "length")[0] == "0 m"
assert (
    units.canonicalize_quantity_string("0.30000000000000001 m", "length")[0]
    == "0.30000000000000001 m"
)
assert units.canonicalize_quantity_string("29.97 Hz", "frequency")[0] == "29.97 Hz"
assert units.canonicalize_quantity_string("4000 USD", "currency")[0] == "4000 USD"

# 3. turn_ratio known answers (45 deg, 360 deg, -30 deg, -160 deg, 0.125 turn, 1.5708 rad)
assert units.canonicalize_quantity_string("45 deg", "angle")[0] == "1/8 turn"
assert units.canonicalize_quantity_string("0.125 turn", "angle")[0] == "1/8 turn"
assert units.canonicalize_quantity_string("360 deg", "angle")[0] == "1 turn"
assert units.canonicalize_quantity_string("0 deg", "angle")[0] == "0 turn"
assert units.canonicalize_quantity_string("-30 deg", "angle")[0] == "-1/12 turn"
assert units.canonicalize_quantity_string("-160 deg", "angle")[0] == "-4/9 turn"
assert units.canonicalize_quantity_string("150 deg", "angle")[0] == "5/12 turn"
assert units.canonicalize_quantity_string("1.5708 rad", "angle")[0] == "1.5708 rad"

# 4. Duplicate-key detection on {"a":1,"a":2} and escaped depth-8 duplicates
try:
    canon.parse_raw_json(b'{"a":1,"a":2}')
    raise AssertionError('{"a":1,"a":2} must be refused')
except canon.CanonRefusal as exc:
    assert exc.reason == "duplicate-key", f"expected duplicate-key, got {exc.reason}"

try:
    canon.parse_raw_json(b'{"k1":{"k2":{"k3":{"k4":{"k5":{"k6":{"k7":{"a":1,"\\u0061":2}}}}}}}}')
    raise AssertionError("escaped depth-8 duplicate must be refused")
except canon.CanonRefusal as exc:
    assert exc.reason == "duplicate-key", f"expected duplicate-key, got {exc.reason}"

# BOM and invalid UTF-8 and null refused
for bad_bytes, exp_reason in [
    (b'\xef\xbb\xbf{"a":1}', "bom-forbidden"),
    (b'\xff\xfe', "invalid-utf8"),
    (b'{"a":null}', "null-value"),
]:
    try:
        canon.parse_raw_json(bad_bytes)
        raise AssertionError(f"expected {exp_reason} refusal")
    except canon.CanonRefusal as exc:
        assert exc.reason == exp_reason, f"expected {exp_reason}, got {exc.reason}"

# 5. Minimal escaping (\n -> \u000a, U+007F raw, non-ASCII raw) and code-point vs UTF-16 order
esc = canon.serialize_canonical({"s": "a\nb\x01\x7fé\"\\"}, mode="ref")
assert esc == '{"s":"a\\u000ab\\u0001\x7fé\\"\\\\"}'.encode("utf-8"), f"bad escape: {esc!r}"

obj_cp = {"\U00010000": 1, "\uE000": 2}
bytes_cp = canon.serialize_canonical(obj_cp, mode="ref")
bytes_u16 = canon.serialize_canonical(obj_cp, mode="utf16-keys")
assert bytes_cp != bytes_u16
assert bytes_cp.index("\uE000".encode("utf-8")) < bytes_cp.index("\U00010000".encode("utf-8"))
assert bytes_u16.index("\U00010000".encode("utf-8")) < bytes_u16.index("\uE000".encode("utf-8"))

# Format registry known answer: {"b":1,"a":2} == {"a":2,"b":1}
b1 = canon.serialize_canonical({"b": 1, "a": 2}, mode="ref")
b2 = canon.serialize_canonical({"a": 2, "b": 1}, mode="ref")
assert b1 == b2 == b'{"a":2,"b":1}'
assert blake3_ref.design_id(b1) == blake3_ref.design_id(b2)

print("design_canon selftest: PASS (blake3 vectors, shortest_decimal, turn_ratio, duplicate-key, escapes, code-point order)")
PY
