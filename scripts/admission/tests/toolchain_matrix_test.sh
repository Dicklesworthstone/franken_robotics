#!/usr/bin/env bash
# scripts/admission/tests/toolchain_matrix_test.sh
#
# Unit test suite for scripts/admission/toolchain_matrix.sh (bead N33c-01).
# Runs all 12 named unit tests plus planted negative controls on real Rust
# nightlies with small fixture workspaces (and optional --live mode for
# real sibling checks). Never deletes any file or directory (COMMON.md 9b).
#
# Usage:
#   scripts/admission/tests/toolchain_matrix_test.sh [--filter <substr>] [--live]

set -euo pipefail

TESTS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
REPO_ROOT="$(cd "${TESTS_DIR}/../../.." && pwd -P)"
HARNESS="${REPO_ROOT}/scripts/admission/toolchain_matrix.sh"

FILTER=""
LIVE_MODE="0"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --filter)
      [ "$#" -ge 2 ] || { printf 'usage: %s [--filter <substr>] [--live]\n' "$0" >&2; exit 2; }
      FILTER="$2"
      shift 2
      ;;
    --live)
      LIVE_MODE="1"
      shift
      ;;
    -h|--help)
      printf 'Usage: %s [--filter <substr>] [--live]\n' "$0"
      exit 0
      ;;
    *)
      printf 'toolchain_matrix_test.sh: unknown arg: %s\n' "$1" >&2
      exit 2
      ;;
  esac
done

B3SUM_BIN=""
if command -v b3sum >/dev/null 2>&1; then
  B3SUM_BIN="$(command -v b3sum)"
elif [ -x /opt/homebrew/bin/b3sum ]; then
  B3SUM_BIN="/opt/homebrew/bin/b3sum"
else
  printf 'toolchain_matrix_test.sh: b3sum not found\n' >&2
  exit 2
fi

SCRATCH_BASE="${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-BlackOx"
mkdir -p "$SCRATCH_BASE"
RUN_TS="$(date -u +%Y%m%dT%H%M%SZ)"
SCRATCH_ROOT="$(mktemp -d "${SCRATCH_BASE}/toolchain-matrix-tests-${RUN_TS}-XXXXXX")"

# Keep all STD-E2E logs and scratch from tests isolated under our scratch dir.
export E2E_LOGS_ROOT="${SCRATCH_ROOT}/e2e-logs"
export E2E_SCRATCH_ROOT="${SCRATCH_ROOT}/e2e-scratch"
mkdir -p "$E2E_LOGS_ROOT" "$E2E_SCRATCH_ROOT"

# Track any chmod a-w dirs so EXIT trap restores u+w (never deleting anything).
RO_DIRS=()
restore_ro_dirs() {
  if [ "${#RO_DIRS[@]}" -gt 0 ]; then
    local d
    for d in "${RO_DIRS[@]}"; do
      chmod -R u+w "$d" 2>/dev/null || true
    done
  fi
}
trap restore_ro_dirs EXIT

SYNTH_RECEIPT="1111111111111111111111111111111111111111111111111111111111111111"
SYNTH_RECEIPT_2="2222222222222222222222222222222222222222222222222222222222222222"

PASS_COUNT=0
FAIL_COUNT=0
SKIP_COUNT=0

# Helper: create a tiny standalone Cargo workspace crate with offline Cargo.lock.
make_crate() {
  local dir="$1"
  local pkg_name="$2"
  local extra_pkg_toml="$3"
  local extra_deps_toml="$4"
  local lib_rs="$5"
  local gen_lock="${6:-1}"

  mkdir -p "${dir}/src"
  {
    printf '[workspace]\n\n'
    printf '[package]\n'
    printf 'name = "%s"\n' "$pkg_name"
    printf 'version = "0.0.1"\n'
    printf 'edition = "2021"\n'
    printf 'publish = false\n'
    if [ -n "$extra_pkg_toml" ]; then
      printf '%s\n' "$extra_pkg_toml"
    fi
    printf '\n[dependencies]\n'
    if [ -n "$extra_deps_toml" ]; then
      printf '%s\n' "$extra_deps_toml"
    fi
  } > "${dir}/Cargo.toml"

  printf '%s\n' "$lib_rs" > "${dir}/src/lib.rs"

  if [ "$gen_lock" = "1" ]; then
    CARGO_TARGET_DIR="${SCRATCH_ROOT}/lockgen-target" \
      cargo generate-lockfile --offline --manifest-path "${dir}/Cargo.toml" >/dev/null 2>&1
  fi
}

extract_kv() {
  local text="$1"
  local key="$2"
  printf '%s\n' "$text" | awk -F= -v k="$key" '$1 == k { print substr($0, length(k) + 2); exit }'
}

# ===========================================================================
# Test 1: cell_pass — zero-dep probe crate passes on all four channels
# ===========================================================================
test_cell_pass() {
  local tdir="${SCRATCH_ROOT}/t_cell_pass"
  local crate_dir="${tdir}/probe"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  make_crate "$crate_dir" "probe-ok" "" "" "pub fn ok() -> u32 { 42 }"

  local out
  out="$("$HARNESS" \
    --sets 1 \
    --root "$tdir" \
    --set-dir "1=${crate_dir}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson summary
  ndjson="$(extract_kv "$out" "ndjson")"
  summary="$(extract_kv "$out" "summary")"

  python3 - "$ndjson" "$summary" "$out_dir" "$B3SUM_BIN" <<'PY'
import json, os, subprocess, sys
ndjson, summary, out_dir, b3 = sys.argv[1:5]
lines = [json.loads(l) for l in open(ndjson, encoding="utf-8") if l.strip()]
assert len(lines) == 4, f"expected 4 cells, got {len(lines)}"
channels = [r["channel"] for r in lines]
assert channels == ["nightly-2026-07-06", "nightly-2026-08-20", "nightly-2026-08-25", "nightly-2026-08-31"], channels
for r in lines:
    assert r["schema"] == "frob.toolchain-matrix/1", r
    assert r["set"] == 1 and r["status"] == "pass" and r["exit"] == 0, r
    assert r["failure_kind"] is None and r["first_error"] is None, r
    assert r["rustc_version"].startswith("rustc "), r
    assert r["packages"] == 1, r
    assert isinstance(r["peak_rss_bytes"], int) and r["peak_rss_bytes"] >= 0, r
    assert r["output_file"] and os.path.isfile(os.path.join(out_dir, r["output_file"])), r
    actual_h = subprocess.check_output([b3, "--no-names"], input=open(os.path.join(out_dir, r["output_file"]), "rb").read()).decode("ascii").strip()
    assert r["output_hash"] == actual_h, (r["output_hash"], actual_h)
    assert "--ignore-rust-version" not in r["argv"], r["argv"]
    assert "-j" in r["argv"] and "8" in r["argv"], r["argv"]

s = json.load(open(summary, encoding="utf-8"))
assert s["schema"] == "frob.toolchain-matrix-summary/1"
assert s["counts"]["pass"] == 4 and s["counts"]["fail"] == 0
for ch in channels:
    assert s["columns"][ch]["all_run_passed"] is True, s["columns"][ch]
    assert s["columns"][ch]["citeable_for_pass"] is True, s["columns"][ch]
PY
}

# ===========================================================================
# Test 2: cell_fail_names_crate — non-existent feature gate names failing crate
#         + synthetic fs-qty failure with generic_const_exprs -> feature-regressed
# ===========================================================================
test_cell_fail_names_crate() {
  local tdir="${SCRATCH_ROOT}/t_cell_fail_names_crate"
  local crate_dir="${tdir}/synthetic"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  make_crate "$crate_dir" "synthetic" "" "" '#![feature(nonexistent_frob_gate_xyz)]
pub fn bad() {}'

  local out
  out="$("$HARNESS" \
    --sets 1 \
    --channels nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "1=${crate_dir}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson
  ndjson="$(extract_kv "$out" "ndjson")"

  python3 - "$ndjson" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
assert len(lines) == 1
r = lines[0]
assert r["status"] == "fail", r
assert r["exit"] != 0, r
fe = r["first_error"]
assert isinstance(fe, dict), r
assert fe["crate"] == "synthetic", f"expected crate 'synthetic', got {fe}"
assert fe["code"] == "E0635", f"expected E0635, got {fe}"
assert len(fe["line"].encode("utf-8")) <= 400, fe
assert r["failure_kind"] == "other", r
PY

  # Also verify acceptance criterion 4: failure in fs-qty concerning generic_const_exprs
  # is classified as failure_kind == "feature-regressed".
  local qty_dir="${tdir}/fs_qty_synth"
  local out_dir2="${tdir}/out_qty"
  make_crate "$qty_dir" "fs-qty" "" "" '#![feature(generic_const_exprs)]
#![feature(nonexistent_generic_const_exprs_probe)]
pub struct Qty;'

  local out2
  out2="$("$HARNESS" \
    --sets 1 \
    --channels nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "1=${qty_dir}" \
    --out-dir "$out_dir2" \
    --target-base "${tbase}-qty" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson2
  ndjson2="$(extract_kv "$out2" "ndjson")"
  python3 - "$ndjson2" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
assert len(lines) == 1
r = lines[0]
assert r["status"] == "fail", r
assert r["first_error"]["crate"] == "fs-qty", r
assert r["failure_kind"] == "feature-regressed", r
PY
}

# ===========================================================================
# Test 3: cell_rust_version — rust-version = "9.0" fails with kind rust-version
# ===========================================================================
test_cell_rust_version() {
  local tdir="${SCRATCH_ROOT}/t_cell_rust_version"
  local crate_dir="${tdir}/msrv_probe"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  make_crate "$crate_dir" "synthetic-msrv" 'rust-version = "9.0"' "" "pub fn future() {}"

  local out
  out="$("$HARNESS" \
    --sets 1 \
    --channels nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "1=${crate_dir}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson
  ndjson="$(extract_kv "$out" "ndjson")"

  python3 - "$ndjson" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
assert len(lines) == 1
r = lines[0]
assert r["status"] == "fail", r
assert r["failure_kind"] == "rust-version", r
assert r["first_error"]["crate"] == "synthetic-msrv", r
assert "--ignore-rust-version" not in r["argv"], r["argv"]
PY
}

# ===========================================================================
# Test 4: cell_timeout — build.rs sleeping past --timeout-s 5 -> fail / timeout
# ===========================================================================
test_cell_timeout() {
  local tdir="${SCRATCH_ROOT}/t_cell_timeout"
  local crate_dir="${tdir}/sleep_probe"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  make_crate "$crate_dir" "sleep-probe" "" "" "pub fn sleepy() {}" 0
  cat > "${crate_dir}/build.rs" <<'EOF'
fn main() {
    std::thread::sleep(std::time::Duration::from_secs(30));
}
EOF
  CARGO_TARGET_DIR="${SCRATCH_ROOT}/lockgen-target" \
    cargo generate-lockfile --offline --manifest-path "${crate_dir}/Cargo.toml" >/dev/null 2>&1

  local out
  out="$("$HARNESS" \
    --sets 1 \
    --channels nightly-2026-08-31 \
    --timeout-s 5 \
    --root "$tdir" \
    --set-dir "1=${crate_dir}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson
  ndjson="$(extract_kv "$out" "ndjson")"

  python3 - "$ndjson" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
assert len(lines) == 1
r = lines[0]
assert r["status"] == "fail", r
assert r["failure_kind"] == "timeout", r
assert r["exit"] == 124, r
assert r["wall_s"] < 25.0, f"timeout did not terminate promptly: wall_s={r['wall_s']}"
PY
}

# ===========================================================================
# Test 5: cell_resolve_missing_path — negative control for resolve failure
# ===========================================================================
test_cell_resolve_missing_path() {
  local tdir="${SCRATCH_ROOT}/t_cell_resolve"
  local crate_dir="${tdir}/missing_dep_probe"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  make_crate "$crate_dir" "missing-dep-probe" "" 'ghost-dep = { path = "../does_not_exist_at_all" }' "pub fn x() {}" 0

  local out
  out="$("$HARNESS" \
    --sets 1 \
    --channels nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "1=${crate_dir}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson
  ndjson="$(extract_kv "$out" "ndjson")"

  python3 - "$ndjson" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
assert len(lines) == 1
r = lines[0]
assert r["status"] == "fail", r
assert r["failure_kind"] == "resolve", r
assert r["first_error"] is not None, r
PY
}

# ===========================================================================
# Test 6: not_run_is_retried — absent target -> not-run naming bead; after
#         creating target (same receipt), plain re-run builds ONLY that cell
# ===========================================================================
test_not_run_is_retried() {
  local tdir="${SCRATCH_ROOT}/t_not_run_is_retried"
  local ws_root="${tdir}/ws"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  mkdir -p "${ws_root}/tests/admission/toolchain/set1_fs_qty"
  make_crate "${ws_root}/tests/admission/toolchain/set1_fs_qty" "probe-s1" "" "" "pub fn s1() {}"

  # Run 1: set 1 exists, set 4 (spark/frob-spark-metal) is absent -> not-run naming BD-016.
  local out1
  out1="$("$HARNESS" \
    --sets 1,4 \
    --channels nightly-2026-08-31 \
    --root "$ws_root" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson1
  ndjson1="$(extract_kv "$out1" "ndjson")"
  local ndjson1_hash_before
  ndjson1_hash_before="$("$B3SUM_BIN" --no-names "$ndjson1")"

  python3 - "$ndjson1" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
by_set = {r["set"]: r for r in lines}
assert by_set[1]["status"] == "pass", by_set[1]
assert by_set[4]["status"] == "not-run", by_set[4]
assert by_set[4]["missing_bead"] == "BD-016", by_set[4]
PY

  # Now create set 4 target (same receipt) and re-run without --force.
  make_crate "${ws_root}/spark/frob-spark-metal" "frob-spark-metal" "" "" "pub fn metal() {}"

  local out2
  out2="$("$HARNESS" \
    --sets 1,4 \
    --channels nightly-2026-08-31 \
    --root "$ws_root" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson2 summary2 ndjson1_hash_after
  ndjson2="$(extract_kv "$out2" "ndjson")"
  summary2="$(extract_kv "$out2" "summary")"
  ndjson1_hash_after="$("$B3SUM_BIN" --no-names "$ndjson1")"

  [ "$ndjson1_hash_before" = "$ndjson1_hash_after" ] || { printf 'FAIL: run 2 modified run 1 ndjson\n' >&2; return 1; }

  python3 - "$ndjson2" "$summary2" "$out2" <<'PY'
import json, sys
ndjson2, summary2, out2 = sys.argv[1:4]
lines = [json.loads(l) for l in open(ndjson2, encoding="utf-8") if l.strip()]
assert len(lines) == 1, f"expected ONLY set 4 to be built in run 2, got {lines}"
assert lines[0]["set"] == 4 and lines[0]["status"] == "pass", lines[0]
assert "executed=1 resumed=1" in out2, out2
s2 = json.load(open(summary2, encoding="utf-8"))
assert s2["columns"]["nightly-2026-08-31"]["rows"] == {"1": "pass", "4": "pass"}, s2
assert s2["columns"]["nightly-2026-08-31"]["all_run_passed"] is True, s2
PY
}

# ===========================================================================
# Test 7: resume_skips_done — second invocation executes ZERO cargo commands
#         and reproduces byte-identical summary JSON
# ===========================================================================
test_resume_skips_done() {
  local tdir="${SCRATCH_ROOT}/t_resume_skips_done"
  local crate_dir="${tdir}/probe"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  local wrap_dir="${tdir}/wrap_bin"
  mkdir -p "$wrap_dir"
  make_crate "$crate_dir" "probe-resume" "" "" "pub fn r() -> i32 { 1 }"

  local real_cargo
  real_cargo="$(command -v cargo)"
  cat > "${wrap_dir}/cargo" <<EOF
#!/usr/bin/env bash
printf '%s\n' "\$*" >> "\${CARGO_COUNT_LOG}"
exec "${real_cargo}" "\$@"
EOF
  chmod +x "${wrap_dir}/cargo"

  local log1="${tdir}/cargo_calls_run1.log"
  local log2="${tdir}/cargo_calls_run2.log"
  : > "$log1"
  : > "$log2"

  local out1
  out1="$(PATH="${wrap_dir}:${PATH}" CARGO_COUNT_LOG="$log1" "$HARNESS" \
    --sets 1 \
    --channels nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "1=${crate_dir}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local out2
  out2="$(PATH="${wrap_dir}:${PATH}" CARGO_COUNT_LOG="$log2" "$HARNESS" \
    --sets 1 \
    --channels nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "1=${crate_dir}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local s1 s2 calls1 calls2
  s1="$(extract_kv "$out1" "summary")"
  s2="$(extract_kv "$out2" "summary")"
  calls1="$(wc -l < "$log1" | tr -d ' ')"
  calls2="$(wc -l < "$log2" | tr -d ' ')"

  [ "$calls1" -ge 1 ] || { printf 'FAIL: run 1 executed 0 cargo commands\n' >&2; return 1; }
  [ "$calls2" -eq 0 ] || { printf 'FAIL: run 2 executed %s cargo commands (expected 0)\n' "$calls2" >&2; cat "$log2" >&2; return 1; }
  cmp -s "$s1" "$s2" || { printf 'FAIL: summary JSON differed between run 1 and run 2\n' >&2; diff -u "$s1" "$s2" >&2; return 1; }
}

# ===========================================================================
# Test 8: record_is_canonical — every NDJSON line parses, sorted keys,
#         re-serializes byte-identically, and STD-E2E log + transcript valid
# ===========================================================================
test_record_is_canonical() {
  local tdir="${SCRATCH_ROOT}/t_record_is_canonical"
  local crate_dir="${tdir}/probe"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  local e2e_logs="${tdir}/e2e-logs"
  mkdir -p "$e2e_logs"
  make_crate "$crate_dir" "probe-canon" "" "" "pub fn c() {}"

  local out
  out="$(E2E_LOGS_ROOT="$e2e_logs" "$HARNESS" \
    --sets 1,4 \
    --channels nightly-2026-08-20,nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "1=${crate_dir}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson summary
  ndjson="$(extract_kv "$out" "ndjson")"
  summary="$(extract_kv "$out" "summary")"

  python3 - "$ndjson" "$summary" "$e2e_logs" "${FROB_E2E_TOOL:-}" <<'PY'
import json, math, os, subprocess, sys, unicodedata

ndjson, summary, e2e_logs, frob_e2e_tool = sys.argv[1:5]

def canon_str(s):
    s = unicodedata.normalize("NFC", s)
    out = ['"']
    for ch in s:
        cp = ord(ch)
        if ch == '"': out.append('\\"')
        elif ch == "\\": out.append("\\\\")
        elif cp < 0x20: out.append(f"\\u{cp:04x}")
        else: out.append(ch)
    out.append('"')
    return "".join(out)

def canon_json(v):
    if v is None: return "null"
    if isinstance(v, bool): return "true" if v else "false"
    if isinstance(v, int): return str(v)
    if isinstance(v, float):
        assert math.isfinite(v)
        return json.dumps(0.0 if v == 0.0 else v, allow_nan=False)
    if isinstance(v, str): return canon_str(v)
    if isinstance(v, list): return "[" + ",".join(canon_json(x) for x in v) + "]"
    if isinstance(v, dict):
        return "{" + ",".join(f"{canon_str(k)}:{canon_json(v[k])}" for k in sorted(v.keys())) + "}"
    raise TypeError(type(v))

raw_lines = open(ndjson, "rb").read().splitlines()
assert len(raw_lines) == 4, f"expected 4 lines (2 pass + 1 not-applicable + 1 not-run), got {len(raw_lines)}"
for idx, raw in enumerate(raw_lines):
    text = raw.decode("utf-8")
    obj = json.loads(text)
    assert list(obj.keys()) == sorted(obj.keys()), f"line {idx} keys unsorted"
    re_ser = canon_json(obj)
    assert re_ser == text, f"line {idx} not canonical:\norig: {text}\ncanon:{re_ser}"
    assert obj["rustc_version"].startswith("rustc "), obj
    assert len(obj["receipt"]) == 64, obj
    assert isinstance(obj["siblings_dirty"], bool), obj

# Summary JSON also canonical.
s_raw = open(summary, "rb").read().decode("utf-8")
s_obj = json.loads(s_raw)
assert canon_json(s_obj) == s_raw, "summary JSON is not canonical"

# Verify STD-E2E log & transcript emitted through lib.sh.
latest_path = os.path.join(e2e_logs, "toolchain_matrix", "LATEST")
assert os.path.isfile(latest_path), f"missing STD-E2E LATEST at {latest_path}"
stem = open(latest_path, encoding="utf-8").read().strip()
log_file = os.path.join(e2e_logs, "toolchain_matrix", f"{stem}.ndjson")
tr_file = os.path.join(e2e_logs, "toolchain_matrix", f"{stem}.transcript.txt")
assert os.path.isfile(log_file) and os.path.isfile(tr_file)
events = [json.loads(l) for l in open(log_file, encoding="utf-8") if l.strip()]
ev_types = [e["event"] for e in events]
assert ev_types[0] == "start" and ev_types[-1] == "summary", ev_types
assert "cmd" in ev_types, ev_types
metric_names = {e["name"] for e in events if e["event"] == "metric"}
assert any(n.endswith(".peak_rss_bytes") for n in metric_names), metric_names
assert any(n.endswith(".packages") for n in metric_names), metric_names
assert "cells_pass" in metric_names and "cells_not_run" in metric_names and "cells_not_applicable" in metric_names, metric_names
if frob_e2e_tool and os.path.isfile(frob_e2e_tool):
    subprocess.run([frob_e2e_tool, "lint", log_file], check=True)
PY
}

# ===========================================================================
# Test 9: probe_is_own_workspace — core metadata has no tests/admission/ pkg;
#         real set-1 and set-2 probes resolve offline with their own lockfiles
# ===========================================================================
test_probe_is_own_workspace() {
  local tdir="${SCRATCH_ROOT}/t_probe_ws"
  mkdir -p "$tdir"
  local s1_file="${tdir}/s1_meta.json"
  local s2_file="${tdir}/s2_meta.json"
  local core_file="${tdir}/core_meta.json"

  # Verify set1_fs_qty and set2_bedrock resolve strictly --locked --offline.
  cargo metadata --format-version 1 --locked --offline \
    --manifest-path "${REPO_ROOT}/tests/admission/toolchain/set1_fs_qty/Cargo.toml" > "$s1_file"
  cargo metadata --format-version 1 --locked --offline \
    --manifest-path "${REPO_ROOT}/tests/admission/toolchain/set2_bedrock/Cargo.toml" > "$s2_file"
  cargo metadata --format-version 1 --no-deps \
    --manifest-path "${REPO_ROOT}/Cargo.toml" > "$core_file"

  python3 - "$core_file" "$s1_file" "$s2_file" "$REPO_ROOT" <<'PY'
import json, os, sys
core = json.load(open(sys.argv[1], encoding="utf-8"))
s1 = json.load(open(sys.argv[2], encoding="utf-8"))
s2 = json.load(open(sys.argv[3], encoding="utf-8"))
repo_root = os.path.realpath(sys.argv[4])

for p in core["packages"]:
    mp = os.path.realpath(p["manifest_path"])
    rel = os.path.relpath(mp, repo_root)
    assert not rel.startswith("tests/admission/"), f"core workspace claimed probe package: {rel}"

s1_names = {p["name"] for p in s1["packages"]}
assert "frob-probe-set1-fs-qty" in s1_names and "fs-qty" in s1_names, s1_names
assert os.path.realpath(s1["workspace_root"]).endswith("tests/admission/toolchain/set1_fs_qty"), s1["workspace_root"]

s2_names = {p["name"] for p in s2["packages"]}
expected_bedrock = {
    "frob-probe-set2-bedrock", "fs-ga", "fs-math", "fs-qty", "fs-rand",
    "fs-blake3", "fs-ivl", "fs-simd", "fs-exec", "fs-evidence", "fs-ad", "fs-la",
}
missing = expected_bedrock - s2_names
assert not missing, f"set2 missing expected crates: {missing}"
assert os.path.realpath(s2["workspace_root"]).endswith("tests/admission/toolchain/set2_bedrock"), s2["workspace_root"]
PY
}

# ===========================================================================
# Test 10: select_sets_and_channels — --sets 1 --channels nightly-2026-08-31
#          runs 1 cell; following full run resumes it and runs the other 18
# ===========================================================================
test_select_sets_and_channels() {
  local tdir="${SCRATCH_ROOT}/t_select_sets_and_channels"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"

  # Create 6 tiny zero-dep crates representing all 6 sets so all 19 run cells execute rapidly.
  local sid
  for sid in 1 2 3 4 5 6; do
    make_crate "${tdir}/s${sid}" "probe-set-${sid}" "" "" "pub fn s${sid}() -> u32 { ${sid} }"
  done

  local set_args=(
    --set-dir "1=${tdir}/s1"
    --set-dir "2=${tdir}/s2"
    --set-dir "3=${tdir}/s3"
    --set-dir "4=${tdir}/s4"
    --set-dir "5=${tdir}/s5"
    --set-dir "6=${tdir}/s6"
  )

  # Step A: single cell (--sets 1 --channels nightly-2026-08-31)
  local out1
  out1="$("$HARNESS" \
    --sets 1 \
    --channels nightly-2026-08-31 \
    --root "$tdir" \
    "${set_args[@]}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson1
  ndjson1="$(extract_kv "$out1" "ndjson")"
  python3 - "$ndjson1" "$out1" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
assert len(lines) == 1, lines
assert lines[0]["set"] == 1 and lines[0]["channel"] == "nightly-2026-08-31" and lines[0]["status"] == "pass"
assert "executed=1 resumed=0" in sys.argv[2], sys.argv[2]
PY

  # Step B: full run resumes (1, nightly-2026-08-31) and runs the other 18 run cells + 5 N/A.
  local out2
  out2="$("$HARNESS" \
    --root "$tdir" \
    "${set_args[@]}" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson2 summary2
  ndjson2="$(extract_kv "$out2" "ndjson")"
  summary2="$(extract_kv "$out2" "summary")"

  python3 - "$ndjson2" "$summary2" "$out2" <<'PY'
import json, sys
ndjson2, summary2, out2 = sys.argv[1:4]
lines = [json.loads(l) for l in open(ndjson2, encoding="utf-8") if l.strip()]
# 18 newly executed run cells + 5 not-applicable cells = 23 lines in run 2 ndjson.
run_lines = [r for r in lines if r["status"] != "not-applicable"]
na_lines = [r for r in lines if r["status"] == "not-applicable"]
assert len(run_lines) == 18, f"expected 18 newly run cells, got {len(run_lines)}"
assert len(na_lines) == 5, f"expected 5 not-applicable cells, got {len(na_lines)}"
assert not any(r["set"] == 1 and r["channel"] == "nightly-2026-08-31" for r in lines), "should have resumed (1, 08-31)"
assert "executed=18 resumed=1" in out2, out2

s2 = json.load(open(summary2, encoding="utf-8"))
assert s2["counts"] == {"fail": 0, "not-applicable": 5, "not-run": 0, "pass": 19}, s2["counts"]
for ch in s2["channels"]:
    assert s2["columns"][ch]["all_run_passed"] is True, (ch, s2["columns"][ch])
PY
}

# ===========================================================================
# Test 11: siblings_untouched — git status --porcelain & Cargo.lock hash
#          of sibling checkout unchanged across a 4-channel run of set 1
# ===========================================================================
test_siblings_untouched() {
  local tdir="${SCRATCH_ROOT}/t_siblings_untouched"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"

  if [ "$LIVE_MODE" = "1" ]; then
    local fsim="${REPO_ROOT}/../frankensim"
    local stat_before head_before lock_before
    stat_before="$(git --no-optional-locks -C "$fsim" status --porcelain)"
    head_before="$(git --no-optional-locks -C "$fsim" rev-parse HEAD)"
    lock_before="$("$B3SUM_BIN" --no-names "${fsim}/Cargo.lock")"

    "$HARNESS" \
      --sets 1 \
      --out-dir "$out_dir" \
      --target-base "$tbase" \
      --receipt "$SYNTH_RECEIPT" \
      --min-free-gib 0 >/dev/null

    local stat_after head_after lock_after
    stat_after="$(git --no-optional-locks -C "$fsim" status --porcelain)"
    head_after="$(git --no-optional-locks -C "$fsim" rev-parse HEAD)"
    lock_after="$("$B3SUM_BIN" --no-names "${fsim}/Cargo.lock")"

    [ "$stat_before" = "$stat_after" ] || { printf 'FAIL: frankensim git status changed\n' >&2; return 1; }
    [ "$head_before" = "$head_after" ] || { printf 'FAIL: frankensim HEAD changed\n' >&2; return 1; }
    [ "$lock_before" = "$lock_after" ] || { printf 'FAIL: frankensim Cargo.lock hash changed\n' >&2; return 1; }
    return 0
  fi

  # Default fixture mode (small read-only sibling git checkout with Cargo.lock).
  local sib="${tdir}/frankensim"
  mkdir -p "${sib}/crates/fs-qty/src"
  cat > "${sib}/Cargo.toml" <<'EOF'
[workspace]
resolver = "2"
members = ["crates/*"]
EOF
  cat > "${sib}/crates/fs-qty/Cargo.toml" <<'EOF'
[package]
name = "fs-qty"
version = "0.0.1"
edition = "2021"
EOF
  printf 'pub struct Length(pub f64);\n' > "${sib}/crates/fs-qty/src/lib.rs"
  CARGO_TARGET_DIR="${SCRATCH_ROOT}/lockgen-target" \
    cargo generate-lockfile --offline --manifest-path "${sib}/Cargo.toml" >/dev/null 2>&1

  git -C "$sib" init -q
  git -C "$sib" config user.name "BlackOx Test"
  git -C "$sib" config user.email "blackox@test.local"
  git -C "$sib" add -A
  git -C "$sib" commit -q -m "synthetic frankensim sibling"
  local sib_head
  sib_head="$(git --no-optional-locks -C "$sib" rev-parse HEAD)"

  local repo="${tdir}/repo"
  local probe="${repo}/tests/admission/toolchain/set1_fs_qty"
  make_crate "$probe" "probe-sib-ro" "" 'fs-qty = { path = "../../../../../frankensim/crates/fs-qty", default-features = false }' \
    "pub type P = fs_qty::Length;"

  cat > "${repo}/franken-stack.lock" <<EOF
toolchain = "nightly-2026-08-31"

[[foundation]]
id = "FD-FSIM"
name = "frankensim"
posture = "consume-path"
path = "../frankensim"
git_rev = "${sib_head}"
toolchain = "nightly-2026-07-06"
admission_gate = "INT-FSIM-01"
receipt = "${SYNTH_RECEIPT}"
fallback = "fb"
EOF

  local stat_before lock_before
  stat_before="$(git --no-optional-locks -C "$sib" status --porcelain)"
  lock_before="$("$B3SUM_BIN" --no-names "${sib}/Cargo.lock")"

  # Make sibling checkout strictly read-only to prove zero writes happen inside it.
  RO_DIRS=(${RO_DIRS[@]+"${RO_DIRS[@]}"} "$sib")
  chmod -R a-w "$sib"

  local out
  out="$("$HARNESS" \
    --sets 1 \
    --root "$repo" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  chmod -R u+w "$sib"
  local stat_after head_after lock_after
  stat_after="$(git --no-optional-locks -C "$sib" status --porcelain)"
  head_after="$(git --no-optional-locks -C "$sib" rev-parse HEAD)"
  lock_after="$("$B3SUM_BIN" --no-names "${sib}/Cargo.lock")"

  [ "$stat_before" = "$stat_after" ] || { printf 'FAIL: sibling git status changed\n' >&2; return 1; }
  [ "$sib_head" = "$head_after" ] || { printf 'FAIL: sibling HEAD changed\n' >&2; return 1; }
  [ "$lock_before" = "$lock_after" ] || { printf 'FAIL: sibling Cargo.lock hash changed\n' >&2; return 1; }

  local ndjson
  ndjson="$(extract_kv "$out" "ndjson")"
  python3 - "$ndjson" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
assert len(lines) == 4
for r in lines:
    assert r["status"] == "pass", r
    assert r["siblings_dirty"] is False, r
    assert r["sibling_drift"]["FD-FSIM"]["class"] == "on-pin", r["sibling_drift"]
PY
}

# ===========================================================================
# Test 12: cell_build_script_salt_unset — probe reaching fs-la outside repo
#          root (no .cargo/config.toml above it, neither salt env var set)
#          fails with kind build-script and first_error.crate == "fs-la";
#          same probe inside repo root passes via .cargo/config.toml [env].
# ===========================================================================
test_cell_build_script_salt_unset() {
  local tdir="${SCRATCH_ROOT}/t_salt_unset"
  local tbase="${tdir}/target"

  if [ "$LIVE_MODE" = "1" ]; then
    # Live mode: copy real set2_bedrock outside repo root with paths to real frankensim/asupersync.
    local outside_dir="${tdir}/outside_set2"
    mkdir -p "${outside_dir}/src"
    cp "${REPO_ROOT}/tests/admission/toolchain/set2_bedrock/src/lib.rs" "${outside_dir}/src/lib.rs"
    cp "${REPO_ROOT}/tests/admission/toolchain/set2_bedrock/Cargo.lock" "${outside_dir}/Cargo.lock"
    python3 - "${REPO_ROOT}/tests/admission/toolchain/set2_bedrock/Cargo.toml" "${outside_dir}/Cargo.toml" "$REPO_ROOT" <<'PY'
import os, sys
src, dst, repo = sys.argv[1:4]
parent = os.path.realpath(os.path.join(repo, ".."))
txt = open(src, encoding="utf-8").read()
txt = txt.replace("../../../../../frankensim", os.path.join(parent, "frankensim"))
txt = txt.replace("../../../../../asupersync", os.path.join(parent, "asupersync"))
open(dst, "w", encoding="utf-8").write(txt)
PY
    local out_fail
    out_fail="$(env -u FRANKENSIM_DEPGRAPH_SALT -u FRANKENSIM_DEPGRAPH_RECEIPT "$HARNESS" \
      --sets 2 \
      --channels nightly-2026-08-31 \
      --root "$tdir" \
      --set-dir "2=${outside_dir}" \
      --out-dir "${tdir}/out_fail" \
      --target-base "${tbase}/outside" \
      --receipt "$SYNTH_RECEIPT" \
      --min-free-gib 0)"
    local ndjson_fail
    ndjson_fail="$(extract_kv "$out_fail" "ndjson")"
    python3 - "$ndjson_fail" <<'PY'
import json, sys
r = json.loads(open(sys.argv[1], encoding="utf-8").readline())
assert r["status"] == "fail" and r["failure_kind"] == "build-script" and r["first_error"]["crate"] == "fs-la", r
PY
    return 0
  fi

  # Default fixture mode: minimal fs-la crate enforcing the exact same fail-closed
  # build.rs salt check as ../frankensim/crates/fs-la/build.rs.
  local sib_la="${tdir}/sib_frankensim/crates/fs-la"
  mkdir -p "${sib_la}/src"
  cat > "${sib_la}/Cargo.toml" <<'EOF'
[package]
name = "fs-la"
version = "0.0.1"
edition = "2021"
EOF
  printf 'pub fn identity() -> &'\''static str { env!("FS_LA_TEST_IDENTITY") }\n' > "${sib_la}/src/lib.rs"
  cat > "${sib_la}/build.rs" <<'EOF'
fn main() {
    println!("cargo:rerun-if-env-changed=FRANKENSIM_DEPGRAPH_RECEIPT");
    println!("cargo:rerun-if-env-changed=FRANKENSIM_DEPGRAPH_SALT");
    if let Ok(r) = std::env::var("FRANKENSIM_DEPGRAPH_RECEIPT") {
        println!("cargo:rustc-env=FS_LA_TEST_IDENTITY={r}");
        return;
    }
    if let Ok(s) = std::env::var("FRANKENSIM_DEPGRAPH_SALT") {
        println!("cargo:rustc-env=FS_LA_TEST_IDENTITY={s}");
        return;
    }
    panic!("GEMM build identity requires dependency-graph evidence: neither FRANKENSIM_DEPGRAPH_RECEIPT nor FRANKENSIM_DEPGRAPH_SALT is set");
}
EOF

  # Create a fixture repository with .cargo/config.toml supplying [env] salt (just like repo root).
  local repo="${tdir}/fixture_repo"
  mkdir -p "${repo}/.cargo"
  cat > "${repo}/.cargo/config.toml" <<'EOF'
[env]
FRANKENSIM_DEPGRAPH_SALT = "franken-robotics-dev-v1"
EOF

  # Inside-repo probe (no .cargo/config.toml inside probe dir; inherits fixture_repo/.cargo/config.toml).
  local inside_probe="${repo}/tests/admission/toolchain/set2_bedrock"
  make_crate "$inside_probe" "probe-set2-inside" "" "fs-la = { path = \"${sib_la}\" }" \
    "pub fn id() -> &'static str { fs_la::identity() }"

  # Outside-repo copy of the exact same probe (no .cargo/config.toml in any ancestor directory).
  local outside_probe="${tdir}/outside_probe"
  mkdir -p "${outside_probe}/src"
  cp "${inside_probe}/Cargo.toml" "${outside_probe}/Cargo.toml"
  cp "${inside_probe}/Cargo.lock" "${outside_probe}/Cargo.lock"
  cp "${inside_probe}/src/lib.rs" "${outside_probe}/src/lib.rs"

  # 1. Outside repo with neither env var set -> MUST fail with build-script + crate == "fs-la".
  local out_outside
  out_outside="$(env -u FRANKENSIM_DEPGRAPH_SALT -u FRANKENSIM_DEPGRAPH_RECEIPT "$HARNESS" \
    --sets 2 \
    --channels nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "2=${outside_probe}" \
    --out-dir "${tdir}/out_outside" \
    --target-base "${tbase}/outside" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  # 2. Inside repo with neither env var set in shell -> MUST pass via .cargo/config.toml [env] alone.
  local out_inside
  out_inside="$(env -u FRANKENSIM_DEPGRAPH_SALT -u FRANKENSIM_DEPGRAPH_RECEIPT "$HARNESS" \
    --sets 2 \
    --channels nightly-2026-08-31 \
    --root "$repo" \
    --out-dir "${tdir}/out_inside" \
    --target-base "${tbase}/inside" \
    --receipt "$SYNTH_RECEIPT_2" \
    --min-free-gib 0)"

  local nd_out nd_in
  nd_out="$(extract_kv "$out_outside" "ndjson")"
  nd_in="$(extract_kv "$out_inside" "ndjson")"

  python3 - "$nd_out" "$nd_in" <<'PY'
import json, sys
r_out = json.loads(open(sys.argv[1], encoding="utf-8").readline())
r_in = json.loads(open(sys.argv[2], encoding="utf-8").readline())
assert r_out["status"] == "fail", r_out
assert r_out["failure_kind"] == "build-script", r_out
assert r_out["first_error"]["crate"] == "fs-la", r_out
assert r_in["status"] == "pass" and r_in["exit"] == 0, r_in
PY
}

# ===========================================================================
# Test 13: harness_exports_no_salt — harness source, probe manifests, and
#          every recorded argv contain neither build-identity variable name
# ===========================================================================
test_harness_exports_no_salt() {
  # Ensure at least one run of matrix records exists even when run standalone via --filter.
  local sdir="${SCRATCH_ROOT}/t_no_salt_records"
  mkdir -p "$sdir"
  "$HARNESS" \
    --sets 3,4,5,6 \
    --root "$sdir" \
    --out-dir "${sdir}/out" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0 >/dev/null

  python3 - "$HARNESS" "$REPO_ROOT" "$SCRATCH_ROOT" <<'PY'
import json, os, sys
harness, repo_root, scratch_root = sys.argv[1:4]
forbidden = ("FRANKENSIM_DEPGRAPH_SALT", "FRANKENSIM_DEPGRAPH_RECEIPT")

files_to_scan = [
    harness,
    os.path.join(repo_root, "tests/admission/toolchain/set1_fs_qty/Cargo.toml"),
    os.path.join(repo_root, "tests/admission/toolchain/set2_bedrock/Cargo.toml"),
]
for fpath in files_to_scan:
    text = open(fpath, encoding="utf-8").read()
    for tok in forbidden:
        assert tok not in text, f"forbidden token {tok} found in {fpath}"

# Scan every NDJSON cell record generated so far across all tests.
checked_records = 0
for dp, _dns, fns in os.walk(scratch_root):
    for fn in fns:
        if not fn.endswith(".ndjson"):
            continue
        p = os.path.join(dp, fn)
        for line in open(p, encoding="utf-8"):
            line = line.strip()
            if not line:
                continue
            obj = json.loads(line)
            if obj.get("schema") != "frob.toolchain-matrix/1":
                continue
            checked_records += 1
            for field in ("argv", "test_argv"):
                arr = obj.get(field) or []
                joined = " ".join(arr)
                for tok in forbidden:
                    assert tok not in joined, f"forbidden token {tok} found in {field} of {p}: {arr}"
assert checked_records > 0, "no matrix records scanned"
PY
}

# ===========================================================================
# Test 14: test_channel_runs_tests_on_sets_3_to_6 — --test-channel runs
#          `cargo +<c> test --locked -j 8` on sets 3..6 and NOT on sets 1..2
# ===========================================================================
test_test_channel_runs_tests_on_sets_3_to_6() {
  local tdir="${SCRATCH_ROOT}/t_test_channel"
  local out_dir="${tdir}/out"
  local tbase="${tdir}/target"
  make_crate "${tdir}/s1" "probe-tc-1" "" "" '#[test] fn t1() { assert_eq!(1, 1); }'
  make_crate "${tdir}/s3" "probe-tc-3" "" "" '#[test] fn t3() { assert_eq!(2 + 2, 4); }'

  local out
  out="$("$HARNESS" \
    --sets 1,3 \
    --channels nightly-2026-08-31 \
    --test-channel nightly-2026-08-31 \
    --root "$tdir" \
    --set-dir "1=${tdir}/s1" \
    --set-dir "3=${tdir}/s3" \
    --out-dir "$out_dir" \
    --target-base "$tbase" \
    --receipt "$SYNTH_RECEIPT" \
    --min-free-gib 0)"

  local ndjson
  ndjson="$(extract_kv "$out" "ndjson")"
  python3 - "$ndjson" <<'PY'
import json, sys
lines = [json.loads(l) for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
by_set = {r["set"]: r for r in lines}
assert by_set[1]["status"] == "pass" and "test_argv" not in by_set[1], by_set[1]
assert by_set[3]["status"] == "pass", by_set[3]
assert by_set[3].get("test_argv") == ["cargo", "+nightly-2026-08-31", "test", "--locked", "-j", "8"], by_set[3]
PY
}

# ===========================================================================
# Test 15: exec_cell_timeout_and_limits — exercises --_exec-cell timeout
#          enforcement (process-group kill) and RSS/exit capture directly
#          without cargo compilation (always runnable on dispatchers too).
# ===========================================================================
test_exec_cell_timeout_and_limits() {
  local tdir="${SCRATCH_ROOT}/t_exec_cell"
  mkdir -p "$tdir"
  local meta_to="${tdir}/timeout.meta.json"
  local out_to="${tdir}/timeout.log"
  local rc=0
  "$HARNESS" --_exec-cell \
    --timeout-s 2 \
    --mem-limit-bytes 17179869184 \
    --meta-out "$meta_to" \
    --output-out "$out_to" \
    -- python3 -c "import time; print('sleeping', flush=True); time.sleep(30)" || rc=$?

  [ "$rc" -eq 124 ] || { printf 'FAIL: expected timeout exit 124, got %d\n' "$rc" >&2; return 1; }

  local meta_ok="${tdir}/ok.meta.json"
  local out_ok="${tdir}/ok.log"
  "$HARNESS" --_exec-cell \
    --timeout-s 10 \
    --mem-limit-bytes 17179869184 \
    --meta-out "$meta_ok" \
    --output-out "$out_ok" \
    -- python3 -c "print('hello-exec-cell')"

  python3 - "$meta_to" "$out_to" "$meta_ok" "$out_ok" <<'PY'
import json, sys
m_to = json.load(open(sys.argv[1], encoding="utf-8"))
o_to = open(sys.argv[2], encoding="utf-8").read()
m_ok = json.load(open(sys.argv[3], encoding="utf-8"))
o_ok = open(sys.argv[4], encoding="utf-8").read()
assert m_to["timed_out"] is True and m_to["exit"] == 124 and m_to["wall_s"] < 10.0, m_to
assert "sleeping" in o_to, o_to
assert m_ok["timed_out"] is False and m_ok["oom_killed"] is False and m_ok["exit"] == 0, m_ok
assert m_ok["peak_rss_bytes"] > 0, m_ok
assert "hello-exec-cell" in o_ok, o_ok
PY
}

# ===========================================================================
# Test 16: classifier_unit_all_7_kinds — pure unit test of classify_failure()
#          extracted verbatim from toolchain_matrix.sh across all 7 kinds and
#          400-byte UTF-8 truncation (non-cargo; does not replace live tests).
# ===========================================================================
test_classifier_unit_all_7_kinds() {
  python3 - "$HARNESS" <<'PY'
import re, sys

src = open(sys.argv[1], encoding="utf-8").read()
m = re.search(r"<<'PY_RECORD'\n(.*?)\nPY_RECORD", src, re.S)
assert m, "could not locate PY_RECORD block in toolchain_matrix.sh"
body = m.group(1)
# Extract function definitions up to `with open(plan_json_path`
cut = body.index("with open(plan_json_path")
ns = {"TIMEOUT_S": 5, "ANSI_RE": re.compile(r"\x1b\[[0-9;]*[A-Za-z]"), "INT53_MAX": (1 << 53) - 1, "HEX64_RE": re.compile(r"^[0-9a-f]{64}$")}
exec("import json, math, os, re, subprocess, sys, unicodedata\n" + body[body.index("def nfc"):cut], ns)
classify = ns["classify_failure"]

# 1. timeout
k, e = classify("   Compiling sleep-probe v0.0.1 (/tmp/x)\n", {"timed_out": True, "oom_killed": False, "exit": 124}, "/tmp/x", [], "sleep-probe")
assert k == "timeout" and e["crate"] == "sleep-probe", (k, e)

# 2. memory
k, e = classify("memory allocation of 99999999999 bytes failed\n", {"timed_out": False, "oom_killed": True, "exit": 137}, "/tmp/x", [], "big-crate")
assert k == "memory" and len(e["line"].encode("utf-8")) <= 400, (k, e)

# 3. rust-version (modern & classic cargo)
k, e = classify("error: rustc 1.100.0-nightly is not supported by the following package:\n  synthetic-msrv@0.0.1 requires rustc 9.0\n", {"timed_out": False, "oom_killed": False, "exit": 101}, "/tmp/x", [], "synthetic-msrv")
assert k == "rust-version" and e["crate"] == "synthetic-msrv", (k, e)

# 4. build-script (fs-la fail-closed salt panic)
bs_out = """   Compiling fs-la v0.0.1 (/p/frankensim/crates/fs-la)
error: failed to run custom build command for `fs-la v0.0.1 (/p/frankensim/crates/fs-la)`

Caused by:
  process didn't exit successfully
  --- stderr
  thread 'main' panicked at crates/fs-la/build.rs:343:5:
  GEMM build identity requires dependency-graph evidence
"""
k, e = classify(bs_out, {"timed_out": False, "oom_killed": False, "exit": 101}, "/tmp/x", [], "probe")
assert k == "build-script" and e["crate"] == "fs-la" and len(e["line"].encode("utf-8")) <= 400, (k, e)

# 5. resolve (missing path dependency)
res_out = "error: failed to get `ghost-dep` as a dependency of package `missing-dep-probe v0.0.1 (/tmp/x)`\n\nCaused by:\n  failed to load source for dependency `ghost-dep`\n"
k, e = classify(res_out, {"timed_out": False, "oom_killed": False, "exit": 101}, "/tmp/x", [], "missing-dep-probe")
assert k == "resolve" and e["crate"] == "ghost-dep", (k, e)

# 6. compiler error naming crate + code E0635 (synthetic non-existent feature)
syn_out = """    Checking synthetic v0.0.1 (/tmp/x)
error[E0635]: unknown feature `nonexistent_frob_gate_xyz`
 --> src/lib.rs:1:12
  |
1 | #![feature(nonexistent_frob_gate_xyz)]
error: could not compile `synthetic` (lib) due to 1 previous error
"""
k, e = classify(syn_out, {"timed_out": False, "oom_killed": False, "exit": 101}, "/tmp/x", [("/tmp/x", "synthetic")], "synthetic")
assert k == "other" and e["crate"] == "synthetic" and e["code"] == "E0635", (k, e)

# 7. feature-regressed in fs-qty concerning generic_const_exprs
qty_out = """    Checking fs-qty v0.0.1 (/p/frankensim/crates/fs-qty)
error[E0635]: unknown feature `generic_const_exprs`
 --> /p/frankensim/crates/fs-qty/src/lib.rs:20:12
  |
20 | #![feature(generic_const_exprs)]
error: could not compile `fs-qty` (lib) due to 1 previous error
"""
k, e = classify(qty_out, {"timed_out": False, "oom_killed": False, "exit": 101}, "/tmp/x", [("/p/frankensim/crates/fs-qty", "fs-qty")], "probe")
assert k == "feature-regressed" and e["crate"] == "fs-qty" and e["code"] == "E0635", (k, e)

# 8. 400-byte UTF-8 boundary truncation (multi-byte chars must stay valid UTF-8 <= 400 bytes)
long_err = "error[E0308]: " + ("é" * 300) + "\n --> src/lib.rs:1:1\nerror: could not compile `synthetic` (lib) due to 1 previous error\n"
k, e = classify(long_err, {"timed_out": False, "oom_killed": False, "exit": 101}, "/tmp/x", [("/tmp/x", "synthetic")], "synthetic")
assert len(e["line"].encode("utf-8")) <= 400 and e["code"] == "E0308" and e["crate"] == "synthetic", (k, len(e["line"].encode("utf-8")), e)
PY
}

# ===========================================================================
# Runner
# ===========================================================================
ALL_TESTS=(
  cell_pass
  cell_fail_names_crate
  cell_rust_version
  cell_timeout
  cell_resolve_missing_path
  not_run_is_retried
  resume_skips_done
  record_is_canonical
  probe_is_own_workspace
  select_sets_and_channels
  siblings_untouched
  cell_build_script_salt_unset
  harness_exports_no_salt
  test_channel_runs_tests_on_sets_3_to_6
  exec_cell_timeout_and_limits
  classifier_unit_all_7_kinds
)

for tname in "${ALL_TESTS[@]}"; do
  if [ -n "$FILTER" ]; then
    case "$tname" in
      *"$FILTER"*) ;;
      *)
        SKIP_COUNT=$((SKIP_COUNT + 1))
        continue
        ;;
    esac
  fi
  printf '=== RUN  %s ===\n' "$tname"
  if "test_${tname}"; then
    printf 'PASS: %s\n' "$tname"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
    printf 'FAIL: %s\n' "$tname" >&2
    FAIL_COUNT=$((FAIL_COUNT + 1))
  fi
done

printf '\nSummary: passed=%d failed=%d skipped=%d\n' "$PASS_COUNT" "$FAIL_COUNT" "$SKIP_COUNT"
[ "$FAIL_COUNT" -eq 0 ]
