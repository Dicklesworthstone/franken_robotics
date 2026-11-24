#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=gauntlet,harness seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

e2e_init "st_pass" "STD-E2E"
e2e_step "basics" "Exercise passing assertions, command capture, exact decimal/rational comparisons, and key replay"
e2e_cmd --name echo_seed -- printf '%s\n' "${E2E_SEED}"
e2e_assert_eq "echo_seed_out" "$(cat "${E2E_LAST_STDOUT}")" "${E2E_SEED}"
e2e_assert_eq "eq_ok" "hello" "hello"
e2e_assert_num "num_ok" "1e-3" "<" "0.0011" "m"
e2e_assert_close "close_ok" "1.0000004" "1" "rel" "5e-7" "rad"
e2e_assert_rational "rat_ok" "6/8" "3/4"
k1="$(e2e_key "alpha")"
k2="$(e2e_key "alpha" --again)"
e2e_assert_eq "key_replay_eq" "$k1" "$k2"
e2e_scene_ready "synthetic-scene-0"
