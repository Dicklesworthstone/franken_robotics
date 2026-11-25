#!/usr/bin/env bash
# Oracle capture script: analytic simple pendulum period (`N38a-04`).
# inputs: none (pure analytic elliptic-integral formula; zero workspace inputs)
#
# Usage:
#   tools/oracle/analytic/pendulum_period.sh --pin "docs/derivations/" --out <fixture.toml>
#
# Prints the tool version string, refuses to run when --pin differs from the
# pinned derivation root, and writes a self-authenticated `kind = "oracle"`
# TOML fixture with its script SHA-256 and whole-file bytes_sha256.

set -euo pipefail

EXPECTED_PIN="docs/derivations/"
PIN=""
OUT=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --pin)
      [ "$#" -ge 2 ] || { echo "error: --pin requires a value" >&2; exit 2; }
      PIN="$2"
      shift 2
      ;;
    --out)
      [ "$#" -ge 2 ] || { echo "error: --out requires a value" >&2; exit 2; }
      OUT="$2"
      shift 2
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

if [ -z "${PIN}" ] || [ -z "${OUT}" ]; then
  echo "usage: $0 --pin <pin> --out <fixture.toml>" >&2
  exit 2
fi

PY_VER="$(python3 --version 2>&1)"
printf 'oracle-capture-tool: analytic-pendulum-v1 (%s)\n' "${PY_VER}"

if [ "${PIN}" != "${EXPECTED_PIN}" ]; then
  printf 'error: pin mismatch: requested "%s", installed/expected "%s"\n' "${PIN}" "${EXPECTED_PIN}" >&2
  exit 3
fi

SCRIPT_PATH="${BASH_SOURCE[0]}"
mkdir -p "$(dirname "${OUT}")"

python3 - "${SCRIPT_PATH}" "${PIN}" "${OUT}" <<'PY'
import hashlib, math, sys

script_path, pin, out_path = sys.argv[1], sys.argv[2], sys.argv[3]
script_bytes = open(script_path, "rb").read()
script_sha = hashlib.sha256(script_bytes).hexdigest()

# Analytic simple pendulum period: L = 1.0 m, g = 9.80665 m/s^2, theta0 = 0.1 rad.
# T = 2*pi*sqrt(L/g) * (1 + (1/16)*theta0^2 + (11/3072)*theta0^4)
length_m = 1.0
g_mps2 = 9.80665
theta0_rad = 0.1
t_small = 2.0 * math.pi * math.sqrt(length_m / g_mps2)
k_corr = 1.0 + (theta0_rad ** 2) / 16.0 + 11.0 * (theta0_rad ** 4) / 3072.0
period_s = t_small * k_corr

zero = "0" * 64
template = f"""# oracle
kind = "oracle"
oracle = "analytic-derivations"
pin = "{pin}"
captured = "2026-11-24"
script = {{ path = "tools/oracle/analytic/pendulum_period.sh", sha256 = "{script_sha}" }}
inputs = []
units = "s"
tolerance_note = "analytic elliptic-integral expansion through O(theta0^4); abs <= 1e-4 s"
bytes_sha256 = "{zero}"
status = "active"

[payload]
length_m = "1.0"
gravity_mps2 = "9.80665"
theta0_rad = "0.1"
period_small_angle_s = "{t_small:.6f}"
period_elliptic_s = "{period_s:.6f}"
"""

digest = hashlib.sha256(template.encode("utf-8")).hexdigest()
final = template.replace(f'bytes_sha256 = "{zero}"', f'bytes_sha256 = "{digest}"', 1)
open(out_path, "w", encoding="utf-8", newline="\n").write(final)
print(f"wrote {out_path} (bytes_sha256={digest})")
PY
