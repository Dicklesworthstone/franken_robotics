#!/usr/bin/env bash
set -eu
test -f Cargo.toml
printf '%s\n' "$1"
printf '%s\n' "$FROB_RUNNER_FIXTURE" >&2
exit 3
