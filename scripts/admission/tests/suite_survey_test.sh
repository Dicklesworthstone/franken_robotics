#!/usr/bin/env bash
# Unit tests for scripts/admission/suite_survey.sh (bead N33c-07).
#
# All fixtures are `synthetic` small git repositories created in a scratch
# directory and removed on exit. Never touches any real sibling repository.
# Bash 3.2 compatible; runs on macOS and Linux.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
SURVEY_SH="$REPO_ROOT/scripts/admission/suite_survey.sh"
EMBEDDED_BASELINE="$REPO_ROOT/tests/admission/survey_2026-11-21.json"

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/frob-suite-survey-test.XXXXXX")"
trap 'rm -rf "$SCRATCH"' EXIT

PASS=0
FAIL=0

pass() {
  PASS=$((PASS + 1))
  printf 'PASS %s\n' "$1"
}

fail() {
  FAIL=$((FAIL + 1))
  printf 'FAIL %s: %s\n' "$1" "$2" >&2
}

init_git_fixture() {
  local dir="$1"
  mkdir -p "$dir"
  git -C "$dir" init -q
  git -C "$dir" config user.email "synthetic@franken.test"
  git -C "$dir" config user.name "Synthetic Fixture"
}

commit_all() {
  local dir="$1"
  local msg="$2"
  git -C "$dir" add -A
  git -C "$dir" commit -q -m "$msg"
}

extract_record_path() {
  local output="$1"
  printf '%s\n' "$output" | awk '/^RECORD /{print $2; exit}'
}

assert_py() {
  local test_name="$1"
  local expr="$2"
  shift 2
  if python3 -c "$expr" "$@"; then
    return 0
  else
    fail "$test_name" "assertion failed"
    return 1
  fi
}

# ---------------------------------------------------------------------------
# Test 1: reads_channel_and_requirement
# Fixture with channel = "nightly-2026-08-20" and
# asupersync = { version = "=0.5.0", optional = true } yields those values.
# ---------------------------------------------------------------------------
test_reads_channel_and_requirement() {
  local t="reads_channel_and_requirement"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib="$base/sib_alpha"
  local out_dir="$base/out"
  mkdir -p "$repo" "$out_dir"

  init_git_fixture "$sib"
  cat >"$sib/rust-toolchain.toml" <<'EOF'
[toolchain]
channel = "nightly-2026-08-20"
EOF
  cat >"$sib/Cargo.toml" <<'EOF'
[package]
name = "sib-alpha"
version = "0.2.0"
edition = "2024"
rust-version = "1.100"

[dependencies]
asupersync = { version = "=0.5.0", optional = true }
serde_json = "=1.0.151"
EOF
  commit_all "$sib" "synthetic init"
  local expected_head
  expected_head="$(git -C "$sib" rev-parse HEAD)"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_alpha"
path = "../sib_alpha"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = "dependencies.asupersync"
watched = { rust_version = "Cargo.toml:package.rust-version", workspace_version = "Cargo.toml:package.version", serde_json = "Cargo.toml:dependencies.serde_json" }
posture = []
cargo = ""
allowed_in = []
forbidden_features = []
reason = ""
fallback = ""
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_dir" --baseline "$base/empty_baseline.json" --stack-lock "$base/no-lock.toml" 2>&1)" || rc=$?
  if [ "$rc" -ne 0 ]; then
    fail "$t" "expected exit 0, got $rc: $out"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  if assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
assert doc["schema"] == "frob.suite-survey/1"
assert len(doc["rows"]) == 1
r = doc["rows"][0]
assert r["name"] == "sib_alpha"
assert r["head"] == sys.argv[2]
assert r["modified_paths"] == 0
assert r["channel"] == "nightly-2026-08-20"
assert r["asupersync"] == {"kind": "registry", "requirement": "=0.5.0", "optional": True, "default_features": True}, r["asupersync"]
assert r["watched"] == {"rust_version": "1.100", "serde_json": "=1.0.151", "workspace_version": "0.2.0"}, r["watched"]
assert doc["channels_in_play"] == ["nightly-2026-08-20"]
assert doc["asupersync_0_5_0_named_by"] == ["sib_alpha"]
assert doc["not_on_candidate_a"] == ["sib_alpha"]
assert doc["minimum_rust_versions"] == {"sib_alpha": "1.100"}
' "$rec" "$expected_head"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 2: path_requirement
# asupersync = { path = "../asupersync" } yields kind "path".
# ---------------------------------------------------------------------------
test_path_requirement() {
  local t="path_requirement"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib="$base/sib_path"
  local out_dir="$base/out"
  mkdir -p "$repo" "$out_dir"

  init_git_fixture "$sib"
  cat >"$sib/rust-toolchain.toml" <<'EOF'
[toolchain]
channel = "nightly-2026-08-31"
EOF
  cat >"$sib/Cargo.toml" <<'EOF'
[workspace]
members = []

[workspace.dependencies]
asupersync = { path = "../asupersync" }
EOF
  commit_all "$sib" "synthetic init"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_path"
path = "../sib_path"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = "workspace.dependencies.asupersync"
watched = {}
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_dir" --baseline "$base/empty_baseline.json" --stack-lock "$base/no-lock.toml" 2>&1)" || rc=$?
  if [ "$rc" -ne 0 ]; then
    fail "$t" "expected exit 0, got $rc: $out"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  if assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
r = doc["rows"][0]
assert r["asupersync"] == {"kind": "path", "requirement": "../asupersync", "optional": False, "default_features": True}, r["asupersync"]
assert doc["asupersync_0_5_0_named_by"] == []
assert doc["not_on_candidate_a"] == []
' "$rec"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 3: counts_modified_paths
# Two modified tracked files yield modified_paths: 2.
# ---------------------------------------------------------------------------
test_counts_modified_paths() {
  local t="counts_modified_paths"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib="$base/sib_dirty"
  local out_dir="$base/out"
  mkdir -p "$repo" "$out_dir"

  init_git_fixture "$sib"
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib/rust-toolchain.toml"
  printf 'a\n' >"$sib/file1.txt"
  printf 'b\n' >"$sib/file2.txt"
  printf 'c\n' >"$sib/file3.txt"
  commit_all "$sib" "synthetic init"

  printf 'a-edited\n' >"$sib/file1.txt"
  printf 'b-edited\n' >"$sib/file2.txt"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_dirty"
path = "../sib_dirty"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local before_porc after_porc
  before_porc="$(git -C "$sib" status --porcelain)"
  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_dir" --baseline "$base/empty_baseline.json" --stack-lock "$base/no-lock.toml" 2>&1)" || rc=$?
  after_porc="$(git -C "$sib" status --porcelain)"
  if [ "$rc" -ne 0 ]; then
    fail "$t" "expected exit 0, got $rc: $out"
    return
  fi
  if [ "$before_porc" != "$after_porc" ]; then
    fail "$t" "sibling git status changed during survey"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  if assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
assert doc["rows"][0]["modified_paths"] == 2, doc["rows"][0]
' "$rec"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 4: absent_sibling
# A missing directory yields head = "absent" and exit 0.
# ---------------------------------------------------------------------------
test_absent_sibling() {
  local t="absent_sibling"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local out_dir="$base/out"
  mkdir -p "$repo" "$out_dir"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_missing"
path = "../does_not_exist"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_dir" --baseline "$base/empty_baseline.json" --stack-lock "$base/no-lock.toml" 2>&1)" || rc=$?
  if [ "$rc" -ne 0 ]; then
    fail "$t" "expected exit 0, got $rc: $out"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  if assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
assert len(doc["rows"]) == 1
r = doc["rows"][0]
assert r["name"] == "sib_missing"
assert r["head"] == "absent"
assert r["channel"] is None
assert r["modified_paths"] is None
' "$rec"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 5: diff_flags_consumed_change
# - changed channel on sibling with posture = "consume-path" -> exit 3 + 1 diff line
# - same change on sibling with no foundation row -> exit 0 + 1 diff line
# - sibling whose row has `uses` list with one `build-time` entry (both table
#   array and inline array forms) -> exit 3 + 1 diff line
# ---------------------------------------------------------------------------
test_diff_flags_consumed_change() {
  local t="diff_flags_consumed_change"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib_c="$base/sib_consumed"
  local sib_u="$base/sib_unconsumed"
  local sib_b="$base/sib_buildtime"
  local sib_bi="$base/sib_buildtime_inline"
  mkdir -p "$repo"

  local name
  for name in sib_consumed sib_unconsumed sib_buildtime sib_buildtime_inline; do
    init_git_fixture "$base/$name"
    printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$base/$name/rust-toolchain.toml"
    commit_all "$base/$name" "init"
  done

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_consumed"
path = "../sib_consumed"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}

[[sibling]]
name = "sib_unconsumed"
path = "../sib_unconsumed"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}

[[sibling]]
name = "sib_buildtime"
path = "../sib_buildtime"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}

[[sibling]]
name = "sib_buildtime_inline"
path = "../sib_buildtime_inline"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}
EOF

  cat >"$base/stack.lock" <<'EOF'
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-CON"
name = "sib_consumed"
posture = "consume-path"
path = "../sib_consumed"
fallback = "none"

[[foundation]]
id = "FD-BT"
name = "sib_buildtime"
path = "../sib_buildtime"
fallback = "none"

[[foundation.uses]]
posture = "pattern"
purpose = "design"
confined_to = []

[[foundation.uses]]
posture = "build-time"
purpose = "derive"
confined_to = ["derive/frob-derive"]

[[foundation]]
id = "FD-BTI"
name = "sib_buildtime_inline"
path = "../sib_buildtime_inline"
uses = [{ posture = "oracle", purpose = "check", confined_to = ["frob-conformance"] }, { posture = "build-time", purpose = "codegen", confined_to = ["derive/frob-derive"] }]
fallback = "none"
EOF

  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"
  local out_init="$base/out_init"
  mkdir -p "$out_init"
  local init_out rc=0
  init_out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_init" --baseline "$base/empty_baseline.json" --stack-lock "$base/stack.lock" 2>&1)" || rc=$?
  if [ "$rc" -ne 0 ]; then
    fail "$t" "initial baseline run failed rc=$rc: $init_out"
    return
  fi
  local prev_rec
  prev_rec="$(extract_record_path "$init_out")"

  # Subcase A: change channel on unconsumed sibling only -> exit 0, 1 DIFF line
  printf '[toolchain]\nchannel = "nightly-2026-09-07"\n' >"$sib_u/rust-toolchain.toml"
  local out_a rc_a=0
  out_a="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out_a" --baseline "$prev_rec" --stack-lock "$base/stack.lock" 2>&1)" || rc_a=$?
  local diff_a
  diff_a="$(printf '%s\n' "$out_a" | grep -c '^DIFF ' || true)"
  if [ "$rc_a" -ne 0 ] || [ "$diff_a" -ne 1 ]; then
    fail "$t" "subcase A (unconsumed): expected rc=0 and 1 DIFF line, got rc=$rc_a diffs=$diff_a: $out_a"
    return
  fi
  # Verify stack_lock digest recorded in JSON
  local rec_a lock_b3
  rec_a="$(extract_record_path "$out_a")"
  lock_b3="$(b3sum --no-names "$base/stack.lock")"
  assert_py "$t" 'import json,sys; d=json.load(open(sys.argv[1])); assert d["stack_lock"] == sys.argv[2]' "$rec_a" "$lock_b3" || return
  # Restore sib_u
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib_u/rust-toolchain.toml"

  # Subcase B: change channel on consume-path sibling -> exit 3, 1 DIFF line
  printf '[toolchain]\nchannel = "nightly-2026-09-07"\n' >"$sib_c/rust-toolchain.toml"
  local out_b rc_b=0
  out_b="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out_b" --baseline "$prev_rec" --stack-lock "$base/stack.lock" 2>&1)" || rc_b=$?
  local diff_b
  diff_b="$(printf '%s\n' "$out_b" | grep -c '^DIFF ' || true)"
  if [ "$rc_b" -ne 3 ] || [ "$diff_b" -ne 1 ]; then
    fail "$t" "subcase B (consume-path): expected rc=3 and 1 DIFF line, got rc=$rc_b diffs=$diff_b: $out_b"
    return
  fi
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib_c/rust-toolchain.toml"

  # Subcase C: change channel on sibling with [[foundation.uses]] containing build-time -> exit 3, 1 DIFF line
  printf '[toolchain]\nchannel = "nightly-2026-09-07"\n' >"$sib_b/rust-toolchain.toml"
  local out_c rc_c=0
  out_c="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out_c" --baseline "$prev_rec" --stack-lock "$base/stack.lock" 2>&1)" || rc_c=$?
  local diff_c
  diff_c="$(printf '%s\n' "$out_c" | grep -c '^DIFF ' || true)"
  if [ "$rc_c" -ne 3 ] || [ "$diff_c" -ne 1 ]; then
    fail "$t" "subcase C (uses build-time): expected rc=3 and 1 DIFF line, got rc=$rc_c diffs=$diff_c: $out_c"
    return
  fi
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib_b/rust-toolchain.toml"

  # Subcase D: inline uses array with build-time -> exit 3, 1 DIFF line
  printf '[toolchain]\nchannel = "nightly-2026-09-07"\n' >"$sib_bi/rust-toolchain.toml"
  local out_d rc_d=0
  out_d="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out_d" --baseline "$prev_rec" --stack-lock "$base/stack.lock" 2>&1)" || rc_d=$?
  local diff_d
  diff_d="$(printf '%s\n' "$out_d" | grep -c '^DIFF ' || true)"
  if [ "$rc_d" -ne 3 ] || [ "$diff_d" -ne 1 ]; then
    fail "$t" "subcase D (inline uses build-time): expected rc=3 and 1 DIFF line, got rc=$rc_d diffs=$diff_d: $out_d"
    return
  fi

  pass "$t"
}

# ---------------------------------------------------------------------------
# Test 6: no_lock_is_fail_closed
# Missing --stack-lock treats every surveyed sibling as consumed (exit 3),
# prints "no stack lock: every surveyed sibling treated as consumed", and
# records stack_lock: null.
# ---------------------------------------------------------------------------
test_no_lock_is_fail_closed() {
  local t="no_lock_is_fail_closed"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib="$base/sib_unlisted"
  mkdir -p "$repo"

  init_git_fixture "$sib"
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib/rust-toolchain.toml"
  commit_all "$sib" "init"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_unlisted"
path = "../sib_unlisted"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"
  local first_out
  first_out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out1" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" 2>&1)"
  local first_rec
  first_rec="$(extract_record_path "$first_out")"

  # Mutate channel and re-run with missing lock
  printf '[toolchain]\nchannel = "nightly-2026-09-07"\n' >"$sib/rust-toolchain.toml"
  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out2" --baseline "$first_rec" --stack-lock "$base/missing.lock" 2>&1)" || rc=$?
  if [ "$rc" -ne 3 ]; then
    fail "$t" "expected exit 3, got $rc: $out"
    return
  fi
  if ! printf '%s\n' "$out" | grep -Fxq "no stack lock: every surveyed sibling treated as consumed"; then
    fail "$t" "missing required fail-closed notice in output: $out"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  if assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
assert doc["stack_lock"] is None, doc["stack_lock"]
' "$rec"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 7: register_schema_shared
# registries/siblings.toml with empty policy columns loads; a row with
# survey = false is not read and does not appear in rows; unknown key fails.
# ---------------------------------------------------------------------------
test_register_schema_shared() {
  local t="register_schema_shared"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib_yes="$base/sib_yes"
  local sib_no="$base/sib_no_not_a_git_repo"
  mkdir -p "$repo" "$sib_no"

  init_git_fixture "$sib_yes"
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib_yes/rust-toolchain.toml"
  commit_all "$sib_yes" "init"

  # sib_no is deliberately NOT a git repo: if the script tried to read it, git would fail.
  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_yes"
path = "../sib_yes"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}
posture = []
cargo = ""
allowed_in = []
forbidden_features = []
reason = ""
fallback = ""

[[sibling]]
name = "sib_no"
path = "../sib_no_not_a_git_repo"
survey = false
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}
posture = []
cargo = ""
allowed_in = []
forbidden_features = []
reason = ""
fallback = ""
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" 2>&1)" || rc=$?
  if [ "$rc" -ne 0 ]; then
    fail "$t" "expected exit 0, got $rc: $out"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
names = [r["name"] for r in doc["rows"]]
assert names == ["sib_yes"], names
' "$rec" || return

  # Negative check: unknown column must be rejected with exit 2
  cat >"$base/bad_siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_yes"
path = "../sib_yes"
survey = true
unknown_extra_field = "boom"
EOF
  local bad_rc=0
  "$SURVEY_SH" --repo-root "$repo" --register "$base/bad_siblings.toml" --out-dir "$base/out_bad" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" >/dev/null 2>&1 || bad_rc=$?
  if [ "$bad_rc" -ne 2 ]; then
    fail "$t" "expected exit 2 on unknown key, got $bad_rc"
    return
  fi

  pass "$t"
}

# ---------------------------------------------------------------------------
# Test 8: known_answer_2026_11_21
# Embedded 2026-11-21 fixture has the exact known-answer derived fields and
# canonical JSON round-trips byte-for-byte through --derive-only.
# ---------------------------------------------------------------------------
test_known_answer_2026_11_21() {
  local t="known_answer_2026_11_21"
  local recomputed="$SCRATCH/$t.recomputed.json"
  "$SURVEY_SH" --derive-only "$EMBEDDED_BASELINE" >"$recomputed"
  if assert_py "$t" '
import json, sys
raw_orig = open(sys.argv[1], "rb").read()
raw_recomp = open(sys.argv[2], "rb").read().rstrip(b"\n")
assert raw_orig == raw_recomp, f"canonical mismatch:\n{raw_orig[:200]}\nvs\n{raw_recomp[:200]}"
doc = json.loads(raw_orig.decode("utf-8"))
assert doc["schema"] == "frob.suite-survey/1"
assert len(doc["rows"]) == 15
assert doc["channels_in_play"] == [
    "1.99.0",
    "nightly-2026-07-06",
    "nightly-2026-08-20",
    "nightly-2026-08-25",
    "nightly-2026-08-31",
], doc["channels_in_play"]
assert doc["asupersync_0_5_0_named_by"] == ["frankentui", "franken_threed", "frankenjax"], doc["asupersync_0_5_0_named_by"]
assert doc["not_on_candidate_a"] == ["frankensim", "frankensympy", "fastmcp_rust", "franken_firmware"], doc["not_on_candidate_a"]
' "$EMBEDDED_BASELINE" "$recomputed"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 9: negative_control_toolchain_change_detected
# Changing rust-toolchain.toml between two surveys MUST produce a channel
# DIFF line naming the new channel (proves channel is read from disk, not
# copied from the plan table).
# ---------------------------------------------------------------------------
test_negative_control_toolchain_change_detected() {
  local t="negative_control_toolchain_change_detected"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib="$base/frankensim"
  local out_dir="$base/out"
  mkdir -p "$repo" "$out_dir"

  init_git_fixture "$sib"
  printf '[toolchain]\nchannel = "nightly-2026-07-06"\n' >"$sib/rust-toolchain.toml"
  commit_all "$sib" "init"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "frankensim"
path = "../frankensim"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = ""
watched = {}
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"
  "$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_dir" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" --date "2026-11-24T00:00:01Z" >/dev/null

  # Now modify toolchain file and run again (automatically diffs against newest in out_dir)
  printf '[toolchain]\nchannel = "nightly-2026-09-21"\n' >"$sib/rust-toolchain.toml"
  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_dir" --stack-lock "$base/missing.lock" --date "2026-11-24T00:00:02Z" 2>&1)" || rc=$?
  if [ "$rc" -ne 3 ]; then
    fail "$t" "expected exit 3 after toolchain change, got $rc: $out"
    return
  fi
  if ! printf '%s\n' "$out" | grep -q 'DIFF frankensim channel: nightly-2026-07-06 -> nightly-2026-09-21'; then
    fail "$t" "expected channel diff line not found: $out"
    return
  fi
  pass "$t"
}

# ---------------------------------------------------------------------------
# Test 10: idempotent_rows_unchanged_tree
# Two runs on unchanged trees write byte-identical `rows` and derived fields.
# ---------------------------------------------------------------------------
test_idempotent_rows_unchanged_tree() {
  local t="idempotent_rows_unchanged_tree"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib="$base/sib_det"
  local out_dir="$base/out"
  mkdir -p "$repo" "$out_dir"

  init_git_fixture "$sib"
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib/rust-toolchain.toml"
  printf '[package]\nname = "det"\nversion = "0.1.0"\n' >"$sib/Cargo.toml"
  commit_all "$sib" "init"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_det"
path = "../sib_det"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = "dependencies.asupersync"
watched = { workspace_version = "Cargo.toml:package.version" }
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local out1 out2 rec1 rec2
  out1="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_dir" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" --date "2026-11-24T01:00:01Z" 2>&1)"
  out2="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$out_dir" --stack-lock "$base/missing.lock" --date "2026-11-24T01:00:02Z" 2>&1)"
  rec1="$(extract_record_path "$out1")"
  rec2="$(extract_record_path "$out2")"
  if assert_py "$t" '
import json, sys
d1 = json.load(open(sys.argv[1]))
d2 = json.load(open(sys.argv[2]))
s1 = json.dumps(d1["rows"], sort_keys=True, separators=(",", ":"))
s2 = json.dumps(d2["rows"], sort_keys=True, separators=(",", ":"))
assert s1 == s2, "rows differed across identical runs"
for k in ("channels_in_play", "asupersync_0_5_0_named_by", "not_on_candidate_a", "minimum_rust_versions"):
    assert d1[k] == d2[k], f"derived field {k} differed"
' "$rec1" "$rec2"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 11: asupersync_0_5_range_and_exclusions
# Cargo version-req clauses like ">=0.5.0, <0.6" name the 0.5 line, while
# exclusions like ">0.5.0" or "!=0.5.0" do not.
# ---------------------------------------------------------------------------
test_asupersync_0_5_range_and_exclusions() {
  local t="asupersync_0_5_range_and_exclusions"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  mkdir -p "$repo" "$base/out"

  local name
  for name in sib_range sib_gt sib_ne; do
    init_git_fixture "$base/$name"
    printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$base/$name/rust-toolchain.toml"
  done
  printf '[package]\nname = "a"\nversion = "0.1.0"\n[dependencies]\nasupersync = ">=0.5.0, <0.6"\n' >"$base/sib_range/Cargo.toml"
  printf '[package]\nname = "b"\nversion = "0.1.0"\n[dependencies]\nasupersync = ">0.5.0"\n' >"$base/sib_gt/Cargo.toml"
  printf '[package]\nname = "c"\nversion = "0.1.0"\n[dependencies]\nasupersync = "!=0.5.0"\n' >"$base/sib_ne/Cargo.toml"
  for name in sib_range sib_gt sib_ne; do
    commit_all "$base/$name" "init"
  done

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_range"
path = "../sib_range"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = "dependencies.asupersync"
watched = {}

[[sibling]]
name = "sib_gt"
path = "../sib_gt"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = "dependencies.asupersync"
watched = {}

[[sibling]]
name = "sib_ne"
path = "../sib_ne"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = "dependencies.asupersync"
watched = {}
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" 2>&1)" || rc=$?
  if [ "$rc" -ne 0 ]; then
    fail "$t" "expected exit 0, got $rc: $out"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  if assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
assert doc["asupersync_0_5_0_named_by"] == ["sib_range"], doc["asupersync_0_5_0_named_by"]
' "$rec"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 12: workspace_exclude_respected_in_fallback
# A crate listed in [workspace] exclude must not be scanned as a workspace
# member when falling back to member manifest inspection.
# ---------------------------------------------------------------------------
test_workspace_exclude_respected_in_fallback() {
  local t="workspace_exclude_respected_in_fallback"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib="$base/sib_excl"
  mkdir -p "$repo" "$base/out" "$sib/crates/real" "$sib/crates/excluded"

  init_git_fixture "$sib"
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib/rust-toolchain.toml"
  cat >"$sib/Cargo.toml" <<'EOF'
[workspace]
members = ["crates/*"]
exclude = ["crates/excluded"]
EOF
  printf '[package]\nname = "real"\nversion = "0.1.0"\n' >"$sib/crates/real/Cargo.toml"
  printf '[package]\nname = "excluded"\nversion = "0.1.0"\n[dependencies]\nasupersync = "=0.5.0"\n' >"$sib/crates/excluded/Cargo.toml"
  commit_all "$sib" "init"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_excl"
path = "../sib_excl"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = "workspace.dependencies.asupersync"
watched = {}
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" 2>&1)" || rc=$?
  if [ "$rc" -ne 0 ]; then
    fail "$t" "expected exit 0, got $rc: $out"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  if assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
assert doc["rows"][0]["asupersync"]["kind"] == "none", doc["rows"][0]["asupersync"]
assert doc["asupersync_0_5_0_named_by"] == [], doc["asupersync_0_5_0_named_by"]
' "$rec"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 13: default_features_underscore_alias
# Cargo accepts `default_features = false` as an alias for `default-features`.
# ---------------------------------------------------------------------------
test_default_features_underscore_alias() {
  local t="default_features_underscore_alias"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  local sib="$base/sib_df"
  mkdir -p "$repo" "$base/out"

  init_git_fixture "$sib"
  printf '[toolchain]\nchannel = "nightly-2026-08-31"\n' >"$sib/rust-toolchain.toml"
  printf '[package]\nname = "df"\nversion = "0.1.0"\n[dependencies]\nasupersync = { version = "0.6.2", default_features = false }\n' >"$sib/Cargo.toml"
  commit_all "$sib" "init"

  cat >"$base/siblings.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "sib_df"
path = "../sib_df"
survey = true
toolchain_file = "rust-toolchain.toml"
asupersync_manifest = "Cargo.toml"
asupersync_key = "dependencies.asupersync"
watched = {}
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local out rc=0
  out="$("$SURVEY_SH" --repo-root "$repo" --register "$base/siblings.toml" --out-dir "$base/out" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" 2>&1)" || rc=$?
  if [ "$rc" -ne 0 ]; then
    fail "$t" "expected exit 0, got $rc: $out"
    return
  fi
  local rec
  rec="$(extract_record_path "$out")"
  if assert_py "$t" '
import json, sys
doc = json.load(open(sys.argv[1]))
assert doc["rows"][0]["asupersync"]["default_features"] is False, doc["rows"][0]["asupersync"]
' "$rec"; then
    pass "$t"
  fi
}

# ---------------------------------------------------------------------------
# Test 14: path_traversal_and_date_validation
# Rejects secondary `..` segments in sibling path, rejects malformed --date,
# and exits 2 cleanly without traceback on missing --derive-only file.
# ---------------------------------------------------------------------------
test_path_traversal_and_date_validation() {
  local t="path_traversal_and_date_validation"
  local base="$SCRATCH/$t"
  local repo="$base/repo"
  mkdir -p "$repo" "$base/out"

  cat >"$base/bad_path.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "escape"
path = "../foo/../../etc"
survey = true
EOF
  printf '{"rows":[],"schema":"frob.suite-survey/1"}\n' >"$base/empty_baseline.json"

  local rc1=0
  "$SURVEY_SH" --repo-root "$repo" --register "$base/bad_path.toml" --out-dir "$base/out" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" >/dev/null 2>&1 || rc1=$?
  if [ "$rc1" -ne 2 ]; then
    fail "$t" "expected exit 2 on path traversal '../foo/../../etc', got $rc1"
    return
  fi

  cat >"$base/ok.toml" <<'EOF'
schema = "frob.siblings/1"

[[sibling]]
name = "missing_ok"
path = "../missing_ok"
survey = true
EOF
  local rc2=0
  "$SURVEY_SH" --repo-root "$repo" --register "$base/ok.toml" --out-dir "$base/out" --baseline "$base/empty_baseline.json" --stack-lock "$base/missing.lock" --date "../../evil" >/dev/null 2>&1 || rc2=$?
  if [ "$rc2" -ne 2 ]; then
    fail "$t" "expected exit 2 on malformed --date '../../evil', got $rc2"
    return
  fi

  local err3 rc3=0
  err3="$("$SURVEY_SH" --derive-only "$base/does_not_exist.json" 2>&1)" || rc3=$?
  if [ "$rc3" -ne 2 ]; then
    fail "$t" "expected exit 2 on missing --derive-only file, got $rc3: $err3"
    return
  fi
  if printf '%s\n' "$err3" | grep -q 'Traceback'; then
    fail "$t" "expected clean error message without Python traceback: $err3"
    return
  fi

  pass "$t"
}

test_reads_channel_and_requirement
test_path_requirement
test_counts_modified_paths
test_absent_sibling
test_diff_flags_consumed_change
test_no_lock_is_fail_closed
test_register_schema_shared
test_known_answer_2026_11_21
test_negative_control_toolchain_change_detected
test_idempotent_rows_unchanged_tree
test_asupersync_0_5_range_and_exclusions
test_workspace_exclude_respected_in_fallback
test_default_features_underscore_alias
test_path_traversal_and_date_validation

printf '\nSUMMARY: %d passed, %d failed\n' "$PASS" "$FAIL"
[ "$FAIL" -eq 0 ]
