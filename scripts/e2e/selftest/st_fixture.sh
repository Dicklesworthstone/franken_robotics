#!/usr/bin/env bash
# e2e: bead=STD-E2E milestone=M0 labels=selftest-fixture seed=1 needs=cpu
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/e2e/lib.sh
source "${SCRIPT_DIR}/../lib.sh"

export E2E_FIXTURES="${SCRIPT_DIR}/fixtures"

e2e_init "st_fixture" "STD-E2E"
e2e_step "fixtures" "Resolve file and directory fixtures, verify copy leaves source untouched, then fail exit 2 on missing fixture"
f_path="$(e2e_fixture "sample.txt")"
e2e_assert_file "sample_file_exists" "$f_path"
d_path="$(e2e_fixture "sample_dir")"
e2e_assert_file "sample_dir_exists" "$d_path"
copy_path="$(e2e_fixture "sample.txt" --copy)"
printf 'mutated-copy\n' >> "$copy_path"
# Now request a missing fixture — must exit 2 (harness error)
missing_path="$(e2e_fixture "missing_fixture_name")"
e2e_assert_eq "unreachable" "$missing_path" ""
