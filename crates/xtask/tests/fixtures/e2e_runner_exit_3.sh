#!/usr/bin/env bash
# synthetic: runner fixture that echoes its argument and environment, then exits 3.
set -eu
test -f Cargo.toml
printf '%s\n' "$1"
printf '%s\n' "$FROB_RUNNER_FIXTURE" >&2
exit 3
