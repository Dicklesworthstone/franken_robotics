#!/bin/sh
# Pre-commit verification recipe (bead N34-15, rules 7 and 8).
#
# Runs five steps in order under frob_envelope_run at class E0:
#   1. ubs on files changed against HEAD
#   2. cargo fmt --check
#   3. cargo clippy and cargo test for crates touched by the change (via cargo shim)
#   4. shell test suites under scripts/lib/tests/
#   5. every workspace check that `cargo xtask` lists when invoked with no argument
#
# Fails with the failing step's exit code and prints the failing step name.
# On success, prints the commit-message verification block (rule 8).
# POSIX sh compatible; never deletes any file or directory (COMMON.md 9b).

set -eu

SCRIPT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)"
DEFAULT_ROOT="$(CDPATH='' cd -- "${SCRIPT_DIR}/.." && pwd)"

ROOT="${DEFAULT_ROOT}"
TARGET_DIR=""

usage() {
  cat <<'EOF'
Usage: scripts/precommit.sh [--root <dir>] [--target-dir <dir>] [-h|--help]

Runs the five pre-commit steps in order under frob_envelope_run (class E0):
  1. ubs on files changed against HEAD
  2. cargo fmt --check
  3. cargo clippy and cargo test for touched crates
  4. shell test suites under scripts/lib/tests/
  5. every workspace check listed by `cargo xtask` (no-arg usage output)
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --root)
      [ "$#" -ge 2 ] || { usage >&2; exit 2; }
      ROOT="$2"
      shift 2
      ;;
    --target-dir)
      [ "$#" -ge 2 ] || { usage >&2; exit 2; }
      TARGET_DIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'precommit.sh: unknown option: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

ROOT="$(CDPATH='' cd -- "$ROOT" && pwd)"
FROB_REPO_ROOT="${DEFAULT_ROOT}"
export FROB_REPO_ROOT

# Source the POSIX machine-envelope guard.
if [ -f "${ROOT}/scripts/lib/envelope.sh" ]; then
  # shellcheck source=scripts/lib/envelope.sh
  . "${ROOT}/scripts/lib/envelope.sh"
else
  # shellcheck source=scripts/lib/envelope.sh
  . "${DEFAULT_ROOT}/scripts/lib/envelope.sh"
fi

if [ -z "$TARGET_DIR" ]; then
  TARGET_DIR="$(frob_target_dir "frob-precommit")"
fi

SCRATCH_DIR="$(frob_scratch_dir "frob-precommit-$$-$(date -u +%s)")"
mkdir -p "$SCRATCH_DIR"
SUMMARY_FILE="${SCRATCH_DIR}/summary.txt"
: > "$SUMMARY_FILE"

record_result() {
  _rr_status="$1"
  _rr_step="$2"
  _rr_cmd="$3"
  _rr_rc="$4"
  printf '  [%s] %s: %s -> exit %s\n' "$_rr_status" "$_rr_step" "$_rr_cmd" "$_rr_rc" >> "$SUMMARY_FILE"
}

fail_step() {
  _fs_step="$1"
  _fs_cmd="$2"
  _fs_rc="$3"
  record_result "FAIL" "$_fs_step" "$_fs_cmd" "$_fs_rc"
  printf 'precommit: FAILED at %s (exit %s): %s\n' "$_fs_step" "$_fs_rc" "$_fs_cmd" >&2
  printf '\n--- BEGIN PRECOMMIT VERIFICATION BLOCK (FAILED) ---\n'
  printf 'Verification (scripts/precommit.sh, envelope E0, root=%s):\n' "$(basename -- "$ROOT")"
  cat "$SUMMARY_FILE"
  printf '%s\n' '--- END PRECOMMIT VERIFICATION BLOCK ---'
  exit "$_fs_rc"
}

cd "$ROOT" || exit 2

# Collect files changed against HEAD (tracked modified/staged + untracked non-ignored).
CHANGED_FILE="${SCRATCH_DIR}/changed_files.txt"
: > "$CHANGED_FILE"
if git --no-optional-locks -C "$ROOT" rev-parse --verify HEAD >/dev/null 2>&1; then
  {
    git --no-optional-locks -C "$ROOT" diff --name-only HEAD 2>/dev/null || true
    git --no-optional-locks -C "$ROOT" diff --cached --name-only 2>/dev/null || true
    git --no-optional-locks -C "$ROOT" ls-files --others --exclude-standard 2>/dev/null || true
  } | LC_ALL=C sort -u > "$CHANGED_FILE"
elif git --no-optional-locks -C "$ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  {
    git --no-optional-locks -C "$ROOT" diff --cached --name-only 2>/dev/null || true
    git --no-optional-locks -C "$ROOT" ls-files --others --exclude-standard 2>/dev/null || true
  } | LC_ALL=C sort -u > "$CHANGED_FILE"
fi

# ---------------------------------------------------------------------------
# Step 1: ubs on files changed against HEAD
# ---------------------------------------------------------------------------
STEP1="step 1 (ubs)"
EXISTING_CHANGED="${SCRATCH_DIR}/existing_changed.txt"
: > "$EXISTING_CHANGED"
while IFS= read -r _cf; do
  [ -n "$_cf" ] || continue
  if [ -f "${ROOT}/${_cf}" ]; then
    printf '%s\n' "$_cf" >> "$EXISTING_CHANGED"
  fi
done < "$CHANGED_FILE"

if [ -s "$EXISTING_CHANGED" ]; then
  if ! command -v ubs >/dev/null 2>&1; then
    fail_step "$STEP1" "ubs <changed-files>" 127
  fi
  set --
  while IFS= read -r _ef; do
    [ -n "$_ef" ] || continue
    set -- "$@" "$_ef"
  done < "$EXISTING_CHANGED"
  _ubs_cmd="ubs --no-color $*"
  _ubs_rc=0
  frob_envelope_run --class E0 -- ubs --no-color "$@" || _ubs_rc=$?
  if [ "$_ubs_rc" -ne 0 ]; then
    fail_step "$STEP1" "$_ubs_cmd" "$_ubs_rc"
  fi
  record_result "PASS" "$STEP1" "$_ubs_cmd" "0"
else
  record_result "PASS" "$STEP1" "ubs (0 changed files against HEAD)" "0"
fi

# ---------------------------------------------------------------------------
# Step 2: cargo fmt --check
# ---------------------------------------------------------------------------
STEP2="step 2 (cargo-fmt)"
if [ -f "${ROOT}/Cargo.toml" ]; then
  _fmt_cmd="cargo fmt --all -- --check"
  _fmt_rc=0
  frob_envelope_run --class E0 -- cargo fmt --all -- --check || _fmt_rc=$?
  if [ "$_fmt_rc" -ne 0 ]; then
    fail_step "$STEP2" "$_fmt_cmd" "$_fmt_rc"
  fi
  record_result "PASS" "$STEP2" "$_fmt_cmd" "0"
else
  record_result "PASS" "$STEP2" "cargo fmt --all -- --check (no Cargo.toml)" "0"
fi

# ---------------------------------------------------------------------------
# Step 3: cargo clippy and cargo test for crates touched by the change
# ---------------------------------------------------------------------------
STEP3="step 3 (cargo-clippy-and-test)"
TOUCHED_CRATES_FILE="${SCRATCH_DIR}/touched_crates.txt"
: > "$TOUCHED_CRATES_FILE"
if [ -f "${ROOT}/Cargo.toml" ] && [ -s "$CHANGED_FILE" ]; then
  while IFS= read -r _tc_path; do
    [ -n "$_tc_path" ] || continue
    case "$_tc_path" in
      crates/*/*)
        _tc_dir="${_tc_path#crates/}"
        _tc_dir="${_tc_dir%%/*}"
        if [ -f "${ROOT}/crates/${_tc_dir}/Cargo.toml" ]; then
          _tc_pkg="$(awk -F'"' '
            /^\[package\]/ { in_pkg=1; next }
            /^\[/ { in_pkg=0 }
            in_pkg && /^[[:space:]]*name[[:space:]]*=/ { print $2; exit }
          ' "${ROOT}/crates/${_tc_dir}/Cargo.toml")"
          if [ -n "$_tc_pkg" ]; then
            printf '%s\n' "$_tc_pkg" >> "$TOUCHED_CRATES_FILE"
          fi
        fi
        ;;
      src/*)
        _tc_pkg="$(awk -F'"' '
          /^\[package\]/ { in_pkg=1; next }
          /^\[/ { in_pkg=0 }
          in_pkg && /^[[:space:]]*name[[:space:]]*=/ { print $2; exit }
        ' "${ROOT}/Cargo.toml")"
        if [ -n "$_tc_pkg" ]; then
          printf '%s\n' "$_tc_pkg" >> "$TOUCHED_CRATES_FILE"
        fi
        ;;
    esac
  done < "$CHANGED_FILE"
fi

TOUCHED_UNIQ="${SCRATCH_DIR}/touched_crates_uniq.txt"
LC_ALL=C sort -u "$TOUCHED_CRATES_FILE" > "$TOUCHED_UNIQ"

if [ -s "$TOUCHED_UNIQ" ]; then
  while IFS= read -r _pkg; do
    [ -n "$_pkg" ] || continue
    _clip_cmd="CARGO_TARGET_DIR=${TARGET_DIR} cargo clippy -p ${_pkg} --all-targets -- -D warnings"
    _clip_rc=0
    frob_envelope_run --class E0 -- env "CARGO_TARGET_DIR=${TARGET_DIR}" cargo clippy -p "$_pkg" --all-targets -- -D warnings || _clip_rc=$?
    if [ "$_clip_rc" -ne 0 ]; then
      fail_step "$STEP3" "$_clip_cmd" "$_clip_rc"
    fi
    record_result "PASS" "$STEP3" "$_clip_cmd" "0"

    _test_cmd="CARGO_TARGET_DIR=${TARGET_DIR} cargo test -p ${_pkg}"
    _test_rc=0
    frob_envelope_run --class E0 -- env "CARGO_TARGET_DIR=${TARGET_DIR}" cargo test -p "$_pkg" || _test_rc=$?
    if [ "$_test_rc" -ne 0 ]; then
      fail_step "$STEP3" "$_test_cmd" "$_test_rc"
    fi
    record_result "PASS" "$STEP3" "$_test_cmd" "0"
  done < "$TOUCHED_UNIQ"
else
  record_result "PASS" "$STEP3" "cargo clippy & cargo test (0 touched Rust crates)" "0"
fi

# ---------------------------------------------------------------------------
# Step 4: shell test suites under scripts/lib/tests/
# ---------------------------------------------------------------------------
STEP4="step 4 (shell-tests)"
SHELL_TESTS_LIST="${SCRATCH_DIR}/shell_tests.txt"
: > "$SHELL_TESTS_LIST"
if [ -d "${ROOT}/scripts/lib/tests" ]; then
  for _st in "${ROOT}/scripts/lib/tests"/*_test.sh; do
    if [ -f "$_st" ]; then
      printf '%s\n' "$_st" >> "$SHELL_TESTS_LIST"
    fi
  done
fi

if [ -s "$SHELL_TESTS_LIST" ]; then
  LC_ALL=C sort "$SHELL_TESTS_LIST" > "${SHELL_TESTS_LIST}.sorted"
  while IFS= read -r _st_path; do
    [ -n "$_st_path" ] || continue
    _st_rel="${_st_path#"${ROOT}"/}"
    _st_cmd="sh ${_st_rel}"
    _st_rc=0
    frob_envelope_run --class E0 -- sh "$_st_path" || _st_rc=$?
    if [ "$_st_rc" -ne 0 ]; then
      fail_step "$STEP4" "$_st_cmd" "$_st_rc"
    fi
    record_result "PASS" "$STEP4" "$_st_cmd" "0"
  done < "${SHELL_TESTS_LIST}.sorted"
else
  record_result "PASS" "$STEP4" "scripts/lib/tests/*_test.sh (0 suites)" "0"
fi

# ---------------------------------------------------------------------------
# Step 5: every workspace check that `cargo xtask` lists with no argument
# ---------------------------------------------------------------------------
STEP5="step 5 (xtask-checks)"
XTASK_USAGE_OUT="${SCRATCH_DIR}/xtask_usage.txt"
_xt_rc=0
frob_envelope_run --class E0 -- env "CARGO_TARGET_DIR=${TARGET_DIR}" cargo xtask >"$XTASK_USAGE_OUT" 2>&1 || _xt_rc=$?
if [ "$_xt_rc" -ne 3 ] && [ "$_xt_rc" -ne 0 ]; then
  cat "$XTASK_USAGE_OUT" >&2
  fail_step "$STEP5" "cargo xtask" "$_xt_rc"
fi

# Parse listed check subcommands from no-arg `cargo xtask` output.
# Accepts:
#   - indented lines (`  check-foo   description...`)
#   - inline `checks: a, b` or `subcommands: a b` lines
XTASK_CHECKS="${SCRATCH_DIR}/xtask_checks.txt"
awk '
  function is_ident(tok) {
    return (tok ~ /^[a-z][a-z0-9_-]*$/) && tok != "usage" && tok != "no" && tok != "exit"
  }
  /^[[:space:]]{2,}[a-z][a-z0-9_-]*([[:space:]]|$)/ {
    line = $0
    sub(/^[[:space:]]+/, "", line)
    split(line, f, /[[:space:]]+/)
    if (is_ident(f[1])) {
      print f[1]
    }
    next
  }
  /^[[:space:]]*(checks|subcommands|commands|available checks)[[:space:]]*:[[:space:]]*[^[:space:]]/ {
    low = tolower($0)
    sub(/^[[:space:]]*(checks|subcommands|commands|available checks)[[:space:]]*:[[:space:]]*/, "", low)
    gsub(/,/, " ", low)
    n = split(low, f, /[[:space:]]+/)
    for (i = 1; i <= n; i++) {
      if (is_ident(f[i])) print f[i]
    }
    next
  }
' "$XTASK_USAGE_OUT" | awk '!seen[$0]++' > "$XTASK_CHECKS"

if [ -s "$XTASK_CHECKS" ]; then
  while IFS= read -r _chk; do
    [ -n "$_chk" ] || continue
    _chk_cmd="cargo xtask ${_chk}"
    _chk_rc=0
    frob_envelope_run --class E0 -- env "CARGO_TARGET_DIR=${TARGET_DIR}" cargo xtask "$_chk" || _chk_rc=$?
    if [ "$_chk_rc" -ne 0 ]; then
      fail_step "$STEP5" "$_chk_cmd" "$_chk_rc"
    fi
    record_result "PASS" "$STEP5" "$_chk_cmd" "0"
  done < "$XTASK_CHECKS"
else
  record_result "PASS" "$STEP5" "cargo xtask (0 checks listed; usage exit ${_xt_rc})" "0"
fi

# Rule 8: print copy-pasteable commit-message verification block.
printf '\n--- BEGIN PRECOMMIT VERIFICATION BLOCK ---\n'
printf 'Verification (scripts/precommit.sh, envelope E0, root=%s):\n' "$(basename -- "$ROOT")"
cat "$SUMMARY_FILE"
printf '%s\n' '--- END PRECOMMIT VERIFICATION BLOCK ---'
exit 0
