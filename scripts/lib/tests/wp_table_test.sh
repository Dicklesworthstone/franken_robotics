#!/bin/sh
# Unit tests for scripts/wp-table.sh and scripts/lib/wp_graph.awk (N44-03).
# Runs directly (`sh scripts/lib/tests/wp_table_test.sh`) using real jq and awk
# on synthetic fixtures under scripts/lib/tests/fixtures/wp/.

set -eu
export LC_ALL=C

SCRIPT_DIR=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH='' cd -- "$SCRIPT_DIR/../../.." && pwd)
WP_TABLE="$REPO_ROOT/scripts/wp-table.sh"
AWK_GRAPH="$REPO_ROOT/scripts/lib/wp_graph.awk"
FIX_DIR="$SCRIPT_DIR/fixtures/wp"
PLAN_TSV="$REPO_ROOT/docs/delivery/work-packages.plan.tsv"

SCRATCH_BASE="${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-wp-table-test"
mkdir -p "$SCRATCH_BASE"
TMP_ROOT=$(mktemp -d "$SCRATCH_BASE/run.XXXXXX")

PASS=0
FAIL=0

pass() {
    PASS=$((PASS + 1))
    printf 'PASS: %s\n' "$1"
}

fail() {
    FAIL=$((FAIL + 1))
    printf 'FAIL: %s — %s\n' "$1" "$2" >&2
}

assert_eq() {
    name="$1"
    expected="$2"
    actual="$3"
    if [ "$expected" = "$actual" ]; then
        return 0
    fi
    fail "$name" "expected='$expected' actual='$actual'"
    return 1
}

run_gen() {
    out_dir="$1"
    shift
    sh "$WP_TABLE" generate --out-dir "$out_dir" "$@" >/dev/null
}

# ------------------------------------------------------------------------------
# 1. tr_chain: a->b->c plus a->c in three packages -> {WP-01->WP-02, WP-02->WP-03}
# ------------------------------------------------------------------------------
test_tr_chain() {
    out="$TMP_ROOT/tr_chain"
    run_gen "$out" --input "$FIX_DIR/tr_chain.jsonl" --no-plan
    edges=$(jq -r '[.package_view.edges[] | .from + "->" + .to] | join(",")' "$out/work-packages.json")
    removed=$(jq -r '[.package_view.removed_edges[] | .from + "->" + .to] | join(",")' "$out/work-packages.json")
    if assert_eq "tr_chain" "WP-01->WP-02,WP-02->WP-03" "$edges" &&
       assert_eq "tr_chain_removed" "WP-01->WP-03" "$removed"; then
        pass "tr_chain"
    fi
}

# ------------------------------------------------------------------------------
# 2. tr_diamond: A->B, A->C, B->D, C->D, A->D -> only A->D removed (4 remain)
# ------------------------------------------------------------------------------
test_tr_diamond() {
    out="$TMP_ROOT/tr_diamond"
    run_gen "$out" --input "$FIX_DIR/tr_diamond.jsonl" --no-plan
    edges=$(jq -r '[.package_view.edges[] | .from + "->" + .to] | join(",")' "$out/work-packages.json")
    removed=$(jq -r '[.package_view.removed_edges[] | .from + "->" + .to] | join(",")' "$out/work-packages.json")
    if assert_eq "tr_diamond" "WP-01->WP-02,WP-01->WP-03,WP-02->WP-04,WP-03->WP-04" "$edges" &&
       assert_eq "tr_diamond_removed" "WP-01->WP-04" "$removed"; then
        pass "tr_diamond"
    fi
}

# ------------------------------------------------------------------------------
# 3. tr_keeps_parallel_paths: A->B, A->C, B->D, C->D with no A->D -> nothing removed
# ------------------------------------------------------------------------------
test_tr_keeps_parallel_paths() {
    out="$TMP_ROOT/tr_parallel"
    run_gen "$out" --input "$FIX_DIR/tr_parallel.jsonl" --no-plan
    edges=$(jq -r '[.package_view.edges[] | .from + "->" + .to] | join(",")' "$out/work-packages.json")
    rem_count=$(jq -r '.package_view.removed_edges | length' "$out/work-packages.json")
    if assert_eq "tr_keeps_parallel_paths" "WP-01->WP-02,WP-01->WP-03,WP-02->WP-04,WP-03->WP-04" "$edges" &&
       assert_eq "tr_keeps_parallel_paths_removed" "0" "$rem_count"; then
        pass "tr_keeps_parallel_paths"
    fi
}

# ------------------------------------------------------------------------------
# 4. plan_table_reduction: 80-package fixture; 139 in, 125 out, exact 14 removals,
#    no cycle, roots WP-01 and WP-33, longest chain 17 ending at WP-77
# ------------------------------------------------------------------------------
test_plan_table_reduction() {
    out="$TMP_ROOT/plan_80"
    run_gen "$out" --input "$FIX_DIR/plan_table_80.jsonl" --plan "$PLAN_TSV"
    j="$out/work-packages.json"

    pkgs=$(jq -r '.counts.packages' "$j")
    raw=$(jq -r '.counts.package_view_edges_raw' "$j")
    red=$(jq -r '.counts.package_view_edges_output' "$j")
    cyc=$(jq -r '.counts.package_view_cycles' "$j")
    pcyc=$(jq -r '.counts.part_cycles' "$j")
    roots=$(jq -r '.package_view.roots | join(",")' "$j")
    lc_edges=$(jq -r '.package_view.longest_chain.edges' "$j")
    lc_end=$(jq -r '.package_view.longest_chain.end' "$j")
    removed=$(jq -r '[.package_view.removed_edges[] | .from + "->" + .to] | join(",")' "$j")
    rel_miss=$(jq -r '.plan_comparison.related_missing | length' "$j")

    expected_removed="WP-10->WP-02,WP-11->WP-07,WP-15->WP-07,WP-31->WP-05,WP-38->WP-07,WP-41->WP-10,WP-44->WP-15,WP-54->WP-24,WP-58->WP-06,WP-61->WP-48,WP-67->WP-05,WP-68->WP-13,WP-76->WP-60,WP-80->WP-15"

    if assert_eq "plan_80_pkgs" "80" "$pkgs" &&
       assert_eq "plan_80_raw" "139" "$raw" &&
       assert_eq "plan_80_red" "125" "$red" &&
       assert_eq "plan_80_cyc" "0" "$cyc" &&
       assert_eq "plan_80_pcyc" "0" "$pcyc" &&
       assert_eq "plan_80_roots" "WP-01,WP-33" "$roots" &&
       assert_eq "plan_80_lc_edges" "17" "$lc_edges" &&
       assert_eq "plan_80_lc_end" "WP-77" "$lc_end" &&
       assert_eq "plan_80_removed" "$expected_removed" "$removed" &&
       assert_eq "plan_80_rel_missing" "0" "$rel_miss" &&
       sh "$WP_TABLE" check --strict-acyclic --input "$FIX_DIR/plan_table_80.jsonl" --plan "$PLAN_TSV" --out-dir "$out" >/dev/null; then
        pass "plan_table_reduction"
    fi
}

# ------------------------------------------------------------------------------
# 5. parts_resolve_two_milestone_package: X@M4->Y@M4, Y@M4->X@M3 -> acyclic parts,
#    cyclic unreduced package view; check --strict-acyclic passes
# ------------------------------------------------------------------------------
test_parts_resolve_two_milestone_package() {
    out="$TMP_ROOT/two_ms"
    run_gen "$out" --input "$FIX_DIR/parts_two_milestone.jsonl" --no-plan
    j="$out/work-packages.json"

    pcyc=$(jq -r '.counts.part_cycles' "$j")
    pkg_acyc=$(jq -r '.package_view.acyclic' "$j")
    pkg_red=$(jq -r '.package_view.reduced' "$j")
    pkg_cyc_m=$(jq -r '.package_view.cycles[0].members | join(",")' "$j")
    pkg_edges=$(jq -r '[.package_view.edges[] | .from + "->" + .to] | join(",")' "$j")

    if assert_eq "parts_resolve_pcyc" "0" "$pcyc" &&
       assert_eq "parts_resolve_pkg_acyc" "false" "$pkg_acyc" &&
       assert_eq "parts_resolve_pkg_red" "false" "$pkg_red" &&
       assert_eq "parts_resolve_pkg_cyc_m" "WP-01,WP-02" "$pkg_cyc_m" &&
       assert_eq "parts_resolve_pkg_edges" "WP-01->WP-02,WP-02->WP-01" "$pkg_edges" &&
       sh "$WP_TABLE" check --strict-acyclic --input "$FIX_DIR/parts_two_milestone.jsonl" --no-plan --out-dir "$out" >/dev/null; then
        pass "parts_resolve_two_milestone_package"
    fi
}

# ------------------------------------------------------------------------------
# 6. planted_cycle_reported: X@M2->Y@M2 and Y@M2->X@M2 -> component + 2 witnesses,
#    fails check --strict-acyclic, passes default check
# ------------------------------------------------------------------------------
test_planted_cycle_reported() {
    out="$TMP_ROOT/planted_cycle"
    run_gen "$out" --input "$FIX_DIR/planted_cycle.jsonl" --no-plan
    j="$out/work-packages.json"

    pcyc=$(jq -r '.counts.part_cycles' "$j")
    cid=$(jq -r '.parts.cycles[0].id' "$j")
    mems=$(jq -r '.parts.cycles[0].members | join(",")' "$j")
    wits=$(jq -r '[.parts.cycles[0].edges[] | .witness[0] + "->" + .witness[1]] | join(",")' "$j")

    if ! assert_eq "planted_cycle_count" "1" "$pcyc" ||
       ! assert_eq "planted_cycle_id" "WP-01@M2+WP-02@M2" "$cid" ||
       ! assert_eq "planted_cycle_mems" "WP-01@M2,WP-02@M2" "$mems" ||
       ! assert_eq "planted_cycle_wits" "N01-01->N02-01,N02-01->N01-01" "$wits"; then
        return
    fi

    # Default check must succeed (cycles are information unless --strict-acyclic)
    if ! sh "$WP_TABLE" check --input "$FIX_DIR/planted_cycle.jsonl" --no-plan --out-dir "$out" >/dev/null; then
        fail "planted_cycle_reported" "default check should exit 0 on cycle"
        return
    fi

    # Strict check must fail
    if sh "$WP_TABLE" check --strict-acyclic --input "$FIX_DIR/planted_cycle.jsonl" --no-plan --out-dir "$out" >/dev/null 2>&1; then
        fail "planted_cycle_reported" "check --strict-acyclic exited 0 on planted cycle"
        return
    fi
    pass "planted_cycle_reported"
}

# ------------------------------------------------------------------------------
# 7. no_package_beads_counted: 3 beads under EPIC-M1 only -> count 3, exit 0
# ------------------------------------------------------------------------------
test_no_package_beads_counted() {
    out="$TMP_ROOT/unpackaged"
    run_gen "$out" --input "$FIX_DIR/unpackaged_only.jsonl" --no-plan
    cnt=$(jq -r '.counts.unpackaged_beads' "$out/work-packages.json")
    keys=$(jq -r '[.unpackaged[].key] | join(",")' "$out/work-packages.json")
    if assert_eq "no_package_cnt" "3" "$cnt" &&
       assert_eq "no_package_keys" "BD-901,BD-902,BD-903" "$keys"; then
        pass "no_package_beads_counted"
    fi
}

# ------------------------------------------------------------------------------
# 8. broken_parent_chain_fails: missing parent fails naming the bead
# ------------------------------------------------------------------------------
test_broken_parent_chain_fails() {
    out="$TMP_ROOT/broken_parent"
    errfile="$TMP_ROOT/broken_parent.err"
    if sh "$WP_TABLE" generate --input "$FIX_DIR/broken_parent.jsonl" --no-plan --out-dir "$out" >"$errfile" 2>&1; then
        fail "broken_parent_chain_fails" "expected non-zero exit"
        return
    fi
    if grep -q "BD-BROKEN" "$errfile"; then
        pass "broken_parent_chain_fails"
    else
        fail "broken_parent_chain_fails" "error output did not name BD-BROKEN: $(cat "$errfile")"
    fi
}

# ------------------------------------------------------------------------------
# 9. missing_milestone_label_fails: missing m0..m8 label fails naming the bead
# ------------------------------------------------------------------------------
test_missing_milestone_label_fails() {
    out="$TMP_ROOT/missing_ms"
    errfile="$TMP_ROOT/missing_ms.err"
    if sh "$WP_TABLE" generate --input "$FIX_DIR/missing_milestone.jsonl" --no-plan --out-dir "$out" >"$errfile" 2>&1; then
        fail "missing_milestone_label_fails" "expected non-zero exit"
        return
    fi
    if grep -q "BD-NOMS" "$errfile"; then
        pass "missing_milestone_label_fails"
    else
        fail "missing_milestone_label_fails" "error output did not name BD-NOMS: $(cat "$errfile")"
    fi
}

# ------------------------------------------------------------------------------
# 10. deterministic_bytes: shuffled fixture lines give byte-identical outputs
# ------------------------------------------------------------------------------
test_deterministic_bytes() {
    out1="$TMP_ROOT/det1"
    out2="$TMP_ROOT/det2"
    shuf_in="$TMP_ROOT/plan_80_shuffled.jsonl"
    # Deterministic non-trivial permutation (reverse + interleave via awk)
    awk '{ lines[NR] = $0 } END {
        for (i = NR; i >= 1; i -= 2) print lines[i]
        for (i = NR - 1; i >= 1; i -= 2) print lines[i]
    }' "$FIX_DIR/plan_table_80.jsonl" > "$shuf_in"

    run_gen "$out1" --input "$FIX_DIR/plan_table_80.jsonl" --plan "$PLAN_TSV"
    run_gen "$out2" --input "$shuf_in" --plan "$PLAN_TSV"

    if cmp -s "$out1/work-packages.json" "$out2/work-packages.json" &&
       cmp -s "$out1/work-packages.md" "$out2/work-packages.md"; then
        pass "deterministic_bytes"
    else
        fail "deterministic_bytes" "shuffled input produced different bytes"
    fi
}

# ------------------------------------------------------------------------------
# 11. witness_is_smallest_pair: smallest (a, b) in key order wins
# ------------------------------------------------------------------------------
test_witness_is_smallest_pair() {
    out="$TMP_ROOT/witness"
    run_gen "$out" --input "$FIX_DIR/witness_smallest.jsonl" --no-plan
    wit=$(jq -r '.package_view.edges[0].witness | join(",")' "$out/work-packages.json")
    pwit=$(jq -r '.parts.edges[0].witness | join(",")' "$out/work-packages.json")
    if assert_eq "witness_pkg" "BD-010,BD-002" "$wit" &&
       assert_eq "witness_part" "BD-010,BD-002" "$pwit"; then
        pass "witness_is_smallest_pair"
    fi
}

# ------------------------------------------------------------------------------
# 12. implicit_edges_order_parts: M1, M3, M5 -> M3->M1 and M5->M3 (implicit),
#     M5->M1 removed, no self-edge in package view
# ------------------------------------------------------------------------------
test_implicit_edges_order_parts() {
    out="$TMP_ROOT/implicit"
    run_gen "$out" --input "$FIX_DIR/implicit_parts.jsonl" --no-plan
    j="$out/work-packages.json"

    pedges=$(jq -r '[.parts.edges[] | .from + "->" + .to + ":" + .witness] | join(",")' "$j")
    prem=$(jq -r '[.parts.removed_edges[] | .from + "->" + .to + ":" + .witness] | join(",")' "$j")
    pkge=$(jq -r '.package_view.edges | length' "$j")

    if assert_eq "implicit_reduced" "WP-01@M3->WP-01@M1:implicit,WP-01@M5->WP-01@M3:implicit" "$pedges" &&
       assert_eq "implicit_removed" "WP-01@M5->WP-01@M1:implicit" "$prem" &&
       assert_eq "implicit_no_pkg_self_edge" "0" "$pkge"; then
        pass "implicit_edges_order_parts"
    fi
}

# ------------------------------------------------------------------------------
# 13. related_links_undirected: WP-01->WP-02 on WP-01, WP-02->WP-03 on WP-03 -> ok
#     Also verifies children_by_kind counts (seed/new/pilot/kill_gate/e2e/upstream).
# ------------------------------------------------------------------------------
test_related_links_undirected() {
    out="$TMP_ROOT/rel_ok"
    run_gen "$out" --input "$FIX_DIR/related_undirected.jsonl" --plan "$FIX_DIR/related_undirected.plan.tsv"
    j="$out/work-packages.json"

    miss=$(jq -r '.plan_comparison.related_missing | length' "$j")
    extra=$(jq -r '.plan_comparison.related_extra | length' "$j")
    # Check kind counts (catches jq filter-parameter bugs)
    k1=$(jq -r '.packages[] | select(.key=="WP-01") | "\(.children.by_kind.seed.open),\(.children.by_kind.pilot.open),\(.children.by_kind.new.total)"' "$j")
    k2=$(jq -r '.packages[] | select(.key=="WP-02") | "\(.children.by_kind.new.closed),\(.children.by_kind.e2e.closed),\(.children.by_kind.upstream.closed)"' "$j")
    k3=$(jq -r '.packages[] | select(.key=="WP-03") | "\(.children.by_kind.kill_gate.open)"' "$j")

    if assert_eq "rel_undir_miss" "0" "$miss" &&
       assert_eq "rel_undir_extra" "0" "$extra" &&
       assert_eq "kind_wp01" "1,1,0" "$k1" &&
       assert_eq "kind_wp02" "1,1,1" "$k2" &&
       assert_eq "kind_wp03" "1" "$k3" &&
       sh "$WP_TABLE" check --input "$FIX_DIR/related_undirected.jsonl" --plan "$FIX_DIR/related_undirected.plan.tsv" --out-dir "$out" >/dev/null; then
        pass "related_links_undirected"
    fi
}

# ------------------------------------------------------------------------------
# 14. related_missing_fails: remove WP-03's link -> check fails naming WP-02 & WP-03;
#     also test adding WP-01-WP-03 -> related-extra reported without failing;
#     also test removing WP-01-WP-02 on 80-package fixture -> fails naming WP-01 & WP-02.
# ------------------------------------------------------------------------------
test_related_missing_fails() {
    # 14a: Remove WP-03 -> WP-02 link from related_undirected.jsonl
    mod_miss="$TMP_ROOT/rel_miss.jsonl"
    out_miss="$TMP_ROOT/rel_miss_out"
    err_miss="$TMP_ROOT/rel_miss.out"
    jq -c 'if .external_ref == "WP-03" then .dependencies = [] else . end' "$FIX_DIR/related_undirected.jsonl" > "$mod_miss"
    run_gen "$out_miss" --input "$mod_miss" --plan "$FIX_DIR/related_undirected.plan.tsv"
    if sh "$WP_TABLE" check --input "$mod_miss" --plan "$FIX_DIR/related_undirected.plan.tsv" --out-dir "$out_miss" >"$err_miss" 2>&1; then
        fail "related_missing_fails" "check exited 0 when WP-02->WP-03 related link removed"
        return
    fi
    if ! grep -q "related-missing: WP-02 WP-03" "$err_miss"; then
        fail "related_missing_fails" "expected 'related-missing: WP-02 WP-03', got: $(cat "$err_miss")"
        return
    fi

    # 14b: Add extra link WP-01--WP-03 (not in drafted plan) -> reported as related-extra, exits 0
    mod_extra="$TMP_ROOT/rel_extra.jsonl"
    out_extra="$TMP_ROOT/rel_extra_out"
    log_extra="$TMP_ROOT/rel_extra.log"
    jq -c 'if .external_ref == "WP-01" then .dependencies += [{"depends_on_id":"syn-wp-03","type":"related"}] else . end' "$FIX_DIR/related_undirected.jsonl" > "$mod_extra"
    run_gen "$out_extra" --input "$mod_extra" --plan "$FIX_DIR/related_undirected.plan.tsv"
    if ! sh "$WP_TABLE" check --input "$mod_extra" --plan "$FIX_DIR/related_undirected.plan.tsv" --out-dir "$out_extra" >"$log_extra" 2>&1; then
        fail "related_missing_fails" "related-extra unexpectedly caused check to fail: $(cat "$log_extra")"
        return
    fi
    if ! grep -q "related-extra: WP-01 WP-03" "$log_extra"; then
        fail "related_missing_fails" "expected 'related-extra: WP-01 WP-03', got: $(cat "$log_extra")"
        return
    fi

    # 14c: Remove WP-01--WP-02 from 80-package fixture -> fails naming WP-01 and WP-02
    mod80="$TMP_ROOT/plan_80_no_1_2.jsonl"
    out80="$TMP_ROOT/plan_80_no_1_2_out"
    log80="$TMP_ROOT/plan_80_no_1_2.log"
    jq -c '
        if .external_ref == "WP-01" then
            .dependencies |= map(select((.type == "related" and .depends_on_id == "syn-wp-02") | not))
        elif .external_ref == "WP-02" then
            .dependencies |= map(select((.type == "related" and .depends_on_id == "syn-wp-01") | not))
        else . end
    ' "$FIX_DIR/plan_table_80.jsonl" > "$mod80"
    run_gen "$out80" --input "$mod80" --plan "$PLAN_TSV"
    if sh "$WP_TABLE" check --input "$mod80" --plan "$PLAN_TSV" --out-dir "$out80" >"$log80" 2>&1; then
        fail "related_missing_fails" "80-pkg check exited 0 when WP-01--WP-02 link removed"
        return
    fi
    if ! grep -q "related-missing: WP-02 WP-01" "$log80"; then
        fail "related_missing_fails" "80-pkg expected 'related-missing: WP-02 WP-01', got: $(cat "$log80")"
        return
    fi

    # 14d: Move WP-01--WP-02 link to the opposite end (WP-02 -> WP-01) -> check passes
    mod80_rev="$TMP_ROOT/plan_80_rev_1_2.jsonl"
    out80_rev="$TMP_ROOT/plan_80_rev_1_2_out"
    jq -c '
        if .external_ref == "WP-02" then
            .dependencies += [{"depends_on_id":"syn-wp-01","type":"related"}]
        else . end
    ' "$mod80" > "$mod80_rev"
    run_gen "$out80_rev" --input "$mod80_rev" --plan "$PLAN_TSV"
    if ! sh "$WP_TABLE" check --input "$mod80_rev" --plan "$PLAN_TSV" --out-dir "$out80_rev" >/dev/null 2>&1; then
        fail "related_missing_fails" "80-pkg link stored on reverse end (WP-02) did not satisfy check"
        return
    fi

    pass "related_missing_fails"
}

# ------------------------------------------------------------------------------
# 15. stale_output_fails: hand-edit work-packages.md -> check fails naming line
# ------------------------------------------------------------------------------
test_stale_output_fails() {
    out="$TMP_ROOT/stale"
    log="$TMP_ROOT/stale.log"
    run_gen "$out" --input "$FIX_DIR/tr_chain.jsonl" --no-plan
    # Hand-edit line 5 of work-packages.md
    awk 'NR == 5 { print "## Hand Edited Summary"; next } { print }' "$out/work-packages.md" > "$TMP_ROOT/stale_edited.md"
    cat "$TMP_ROOT/stale_edited.md" > "$out/work-packages.md"

    if sh "$WP_TABLE" check --input "$FIX_DIR/tr_chain.jsonl" --no-plan --out-dir "$out" >"$log" 2>&1; then
        fail "stale_output_fails" "check exited 0 on hand-edited work-packages.md"
        return
    fi
    if grep -q "stale: .*work-packages.md differs at line 5:" "$log"; then
        pass "stale_output_fails"
    else
        fail "stale_output_fails" "expected first differing line 5, got: $(cat "$log")"
    fi
}

# ------------------------------------------------------------------------------
# 16. duplicate_key_and_unknown_pkg_and_loop_fail: structural validation guards
# ------------------------------------------------------------------------------
test_validation_guards() {
    # 16a: duplicate key
    dup_in="$TMP_ROOT/dup.jsonl"
    cat "$FIX_DIR/tr_chain.jsonl" > "$dup_in"
    printf '%s\n' '{"id":"syn-dup-id","external_ref":"N01-01","issue_type":"task","status":"open","title":"dup","description":"","labels":["synthetic","m0"],"dependencies":[{"depends_on_id":"syn-wp-01","type":"parent-child"}]}' >> "$dup_in"
    if sh "$WP_TABLE" generate --input "$dup_in" --no-plan --out-dir "$TMP_ROOT/dup_out" >"$TMP_ROOT/dup.err" 2>&1; then
        fail "validation_guards" "duplicate key did not fail"
        return
    fi
    if ! grep -q "N01-01" "$TMP_ROOT/dup.err"; then
        fail "validation_guards" "duplicate key error did not name N01-01"
        return
    fi

    # 16b: parent chain loop
    loop_in="$TMP_ROOT/loop.jsonl"
    cat <<'EOF' > "$loop_in"
{"id":"syn-l1","external_ref":"BD-L1","issue_type":"task","status":"open","title":"L1","description":"","labels":["synthetic","m0"],"dependencies":[{"depends_on_id":"syn-l2","type":"parent-child"}]}
{"id":"syn-l2","external_ref":"BD-L2","issue_type":"task","status":"open","title":"L2","description":"","labels":["synthetic","m0"],"dependencies":[{"depends_on_id":"syn-l1","type":"parent-child"}]}
EOF
    if sh "$WP_TABLE" generate --input "$loop_in" --no-plan --out-dir "$TMP_ROOT/loop_out" >"$TMP_ROOT/loop.err" 2>&1; then
        fail "validation_guards" "parent loop did not fail"
        return
    fi
    if ! grep -q "loop" "$TMP_ROOT/loop.err"; then
        fail "validation_guards" "parent loop error did not mention loop"
        return
    fi

    # 16c: unknown package key WP-99 with child BD-ORPHAN
    unk_in="$TMP_ROOT/unk.jsonl"
    cat "$FIX_DIR/related_undirected.jsonl" > "$unk_in"
    printf '%s\n' '{"id":"syn-wp-99","external_ref":"WP-99","issue_type":"epic","status":"open","title":"[WP-99] Unknown","description":"","labels":["synthetic","m0"],"dependencies":[]}' >> "$unk_in"
    printf '%s\n' '{"id":"syn-orphan","external_ref":"BD-ORPHAN","issue_type":"task","status":"open","title":"[BD-ORPHAN] Orphan","description":"","labels":["synthetic","m0"],"dependencies":[{"depends_on_id":"syn-wp-99","type":"parent-child"}]}' >> "$unk_in"
    if sh "$WP_TABLE" generate --input "$unk_in" --plan "$FIX_DIR/related_undirected.plan.tsv" --out-dir "$TMP_ROOT/unk_out" >"$TMP_ROOT/unk.err" 2>&1; then
        fail "validation_guards" "unknown package WP-99 did not fail"
        return
    fi
    if ! grep -q "WP-99" "$TMP_ROOT/unk.err" || ! grep -q "BD-ORPHAN" "$TMP_ROOT/unk.err"; then
        fail "validation_guards" "unknown package error must name WP-99 and BD-ORPHAN: $(cat "$TMP_ROOT/unk.err")"
        return
    fi

    pass "validation_guards"
}

# ------------------------------------------------------------------------------
# 17. witnesses_exist_in_export: every edge witness pair exists in the export
# ------------------------------------------------------------------------------
test_witnesses_exist_in_export() {
    out="$TMP_ROOT/plan_80"
    bad=$(jq -s '
        .[0] as $gen |
        (.[1:] | map(.external_ref) | map({(.): true}) | add) as $keys |
        [ ($gen.package_view.edges + $gen.parts.edges)[] |
          select(.witness != "implicit") |
          select((($keys[.witness[0]] // false) and ($keys[.witness[1]] // false)) | not)
        ] | length
    ' "$out/work-packages.json" "$FIX_DIR/plan_table_80.jsonl")
    if assert_eq "witnesses_exist_in_export" "0" "$bad"; then
        pass "witnesses_exist_in_export"
    fi
}

# ------------------------------------------------------------------------------
# 18. metamorphic_relations:
#     (a) adding an already-implied edge changes no reduced edge;
#     (b) removing the unique witness of an edge removes it or reveals an implied one.
# ------------------------------------------------------------------------------
test_metamorphic_relations() {
    # (a) Add WP-77 -> WP-01 (already implied through 17-edge chain) to plan_table_80.jsonl
    out_orig="$TMP_ROOT/plan_80"
    add_in="$TMP_ROOT/meta_add.jsonl"
    out_add="$TMP_ROOT/meta_add_out"
    jq -c '
        if .external_ref == "BD-177" then
            .dependencies += [{"depends_on_id":"syn-bead-wp-01","type":"blocks"}]
        else . end
    ' "$FIX_DIR/plan_table_80.jsonl" > "$add_in"
    run_gen "$out_add" --input "$add_in" --plan "$PLAN_TSV"

    e_orig=$(jq -c '[.package_view.edges[] | [.from, .to]]' "$out_orig/work-packages.json")
    e_add=$(jq -c '[.package_view.edges[] | [.from, .to]]' "$out_add/work-packages.json")
    if ! assert_eq "meta_implied_edge_invariant" "$e_orig" "$e_add"; then
        return
    fi

    # (b) On tr_chain (A->B, B->C, A->C), remove unique witness B->C => A->C is revealed!
    rem_in="$TMP_ROOT/meta_rem.jsonl"
    out_rem="$TMP_ROOT/meta_rem_out"
    jq -c '
        if .external_ref == "N02-01" then
            .dependencies |= map(select(.type != "blocks"))
        else . end
    ' "$FIX_DIR/tr_chain.jsonl" > "$rem_in"
    run_gen "$out_rem" --input "$rem_in" --no-plan
    e_rem=$(jq -r '[.package_view.edges[] | .from + "->" + .to] | join(",")' "$out_rem/work-packages.json")
    if assert_eq "meta_unique_witness_reveal" "WP-01->WP-02,WP-01->WP-03" "$e_rem"; then
        pass "metamorphic_relations"
    fi
}

# ------------------------------------------------------------------------------
# 19. negative_control_reach_u_bug: mutated wp_graph.awk checking v in reach(u)
#     removes ALL edges on tr_parallel (leaves 0 instead of 4), proving the test catches it.
# ------------------------------------------------------------------------------
test_negative_control_reach_u_bug() {
    mut_awk="$TMP_ROOT/wp_graph_buggy.awk"
    sed 's/if (cw != cv && ((cw SUBSEP cv) in reach))/if ((cu SUBSEP cv) in reach)/' "$AWK_GRAPH" > "$mut_awk"
    if cmp -s "$AWK_GRAPH" "$mut_awk"; then
        fail "negative_control_reach_u_bug" "mutation did not alter wp_graph.awk"
        return
    fi
    in_par="$TMP_ROOT/par.in"
    cat <<'EOF' > "$in_par"
NODE	WP-01
NODE	WP-02
NODE	WP-03
NODE	WP-04
EDGE	WP-01	WP-02	a	b
EDGE	WP-01	WP-03	a	c
EDGE	WP-02	WP-04	b	d
EDGE	WP-03	WP-04	c	d
EOF
    good_cnt=$(awk -F '\t' -f "$AWK_GRAPH" "$in_par" | awk -F '\t' '$1=="REDUCED_EDGE"{c++} END{print c+0}')
    bad_cnt=$(awk -F '\t' -f "$mut_awk" "$in_par" | awk -F '\t' '$1=="REDUCED_EDGE"{c++} END{print c+0}')
    if assert_eq "neg_ctrl_good" "4" "$good_cnt" &&
       assert_eq "neg_ctrl_buggy" "0" "$bad_cnt"; then
        pass "negative_control_reach_u_bug"
    fi
}

# ------------------------------------------------------------------------------
# 20. posix_awk_parity & no_tracker_binary_call
# ------------------------------------------------------------------------------
test_posix_and_hygiene() {
    # Ensure scripts/wp-table.sh never invokes br or bv as a command
    if awk '!/^[[:space:]]*#/ && /(^|[[:space:];|&(])(br|bv)[[:space:]]/' "$WP_TABLE" | grep -q .; then
        fail "posix_and_hygiene" "scripts/wp-table.sh appears to invoke br or bv"
        return
    fi
    # If macOS /usr/bin/awk exists alongside PATH awk, verify identical graph output
    if [ -x /usr/bin/awk ]; then
        in_par="$TMP_ROOT/par.in"
        awk -F '\t' -f "$AWK_GRAPH" "$in_par" > "$TMP_ROOT/awk1.out"
        /usr/bin/awk -F '\t' -f "$AWK_GRAPH" "$in_par" > "$TMP_ROOT/awk2.out"
        if ! cmp -s "$TMP_ROOT/awk1.out" "$TMP_ROOT/awk2.out"; then
            fail "posix_and_hygiene" "GNU awk and /usr/bin/awk produced different outputs"
            return
        fi
    fi
    pass "posix_and_hygiene"
}

# ------------------------------------------------------------------------------
# 21. invalid_plan_tsv_header_fails:
#     an invalid plan TSV header must fail even when the export has no WP
#     epics (guards against POSIX sh swallowing awk errors in a pipeline).
# ------------------------------------------------------------------------------
test_invalid_plan_tsv_header_fails() {
    bad_tsv="$TMP_ROOT/bad_header.tsv"
    printf 'wrong\theader\tcols\n' > "$bad_tsv"
    if sh "$WP_TABLE" generate --input "$FIX_DIR/unpackaged_only.jsonl" --plan "$bad_tsv" --out-dir "$TMP_ROOT/bad_plan_out" >"$TMP_ROOT/bad_plan.out" 2>"$TMP_ROOT/bad_plan.err"; then
        fail "invalid_plan_tsv_header_fails" "invalid plan TSV header exited 0 on unpackaged_only.jsonl"
        return
    fi
    if ! grep -q "invalid plan TSV header" "$TMP_ROOT/bad_plan.err"; then
        fail "invalid_plan_tsv_header_fails" "expected invalid plan TSV header error, got: $(cat "$TMP_ROOT/bad_plan.err")"
        return
    fi
    pass "invalid_plan_tsv_header_fails"
}

# ------------------------------------------------------------------------------
# 22. plan_tsv_empty_truncated_duplicate_and_bytewise_sort:
#     (a) empty 0-byte plan TSV fails closed (missing header);
#     (b) truncated plan row (missing depends_on column) fails closed;
#     (c) duplicate package row in plan TSV fails closed;
#     (d) wp_graph.awk sorts numeric-looking node/witness keys bytewise ("10" < "2"),
#         not numerically;
#     (e) compare_file works when out-dir path contains '='.
# ------------------------------------------------------------------------------
test_plan_tsv_empty_truncated_duplicate_and_bytewise_sort() {
    # 22a: empty 0-byte plan TSV
    empty_tsv="$TMP_ROOT/empty_plan.tsv"
    : > "$empty_tsv"
    if sh "$WP_TABLE" generate --input "$FIX_DIR/unpackaged_only.jsonl" --plan "$empty_tsv" --out-dir "$TMP_ROOT/empty_plan_out" >"$TMP_ROOT/empty_plan.out" 2>"$TMP_ROOT/empty_plan.err"; then
        fail "plan_tsv_empty_truncated_duplicate_and_bytewise_sort" "empty 0-byte plan TSV exited 0"
        return
    fi

    # 22b: truncated plan row (only 2 columns, missing depends_on)
    trunc_tsv="$TMP_ROOT/trunc_plan.tsv"
    printf 'package\tfirst_milestone\tdepends_on\nWP-01\tM0\n' > "$trunc_tsv"
    if sh "$WP_TABLE" generate --input "$FIX_DIR/tr_chain.jsonl" --plan "$trunc_tsv" --out-dir "$TMP_ROOT/trunc_plan_out" >"$TMP_ROOT/trunc_plan.out" 2>"$TMP_ROOT/trunc_plan.err"; then
        fail "plan_tsv_empty_truncated_duplicate_and_bytewise_sort" "truncated 2-column plan TSV row exited 0"
        return
    fi

    # 22c: duplicate package key in plan TSV
    dup_tsv="$TMP_ROOT/dup_plan.tsv"
    printf 'package\tfirst_milestone\tdepends_on\nWP-01\tM0\t-\nWP-01\tM0\tWP-02\nWP-02\tM0\t-\nWP-03\tM0\t-\n' > "$dup_tsv"
    if sh "$WP_TABLE" generate --input "$FIX_DIR/tr_chain.jsonl" --plan "$dup_tsv" --out-dir "$TMP_ROOT/dup_plan_out" >"$TMP_ROOT/dup_plan.out" 2>"$TMP_ROOT/dup_plan.err"; then
        fail "plan_tsv_empty_truncated_duplicate_and_bytewise_sort" "duplicate package key in plan TSV exited 0"
        return
    fi

    # 22d: bytewise (string) sort in wp_graph.awk on numeric-looking keys ("10" < "2")
    num_in="$TMP_ROOT/num_keys.in"
    printf 'NODE\t2\nNODE\t10\nEDGE\t2\t10\t2\t9\nEDGE\t2\t10\t10\t1\n' > "$num_in"
    num_nodes=$(awk -F '\t' -f "$AWK_GRAPH" "$num_in" | awk -F '\t' '$1=="CONDENSED_NODE"{printf "%s,", $2}')
    num_wit=$(awk -F '\t' -f "$AWK_GRAPH" "$num_in" | awk -F '\t' '$1=="RAW_EDGE"{print $4 ":" $5}')
    if ! assert_eq "bytewise_nodes" "10,2," "$num_nodes" ||
       ! assert_eq "bytewise_witness" "10:1" "$num_wit"; then
        return
    fi

    # 22e: check stale diff with '=' in --out-dir path
    eq_dir="$TMP_ROOT/dir=eq"
    run_gen "$eq_dir" --input "$FIX_DIR/tr_chain.jsonl" --no-plan
    printf 'corrupted\n' > "$eq_dir/work-packages.md"
    if sh "$WP_TABLE" check --input "$FIX_DIR/tr_chain.jsonl" --no-plan --out-dir "$eq_dir" >"$TMP_ROOT/eq_dir.log" 2>&1; then
        fail "plan_tsv_empty_truncated_duplicate_and_bytewise_sort" "stale check with '=' in path exited 0"
        return
    fi
    if ! grep -q "stale: .*work-packages.md differs at line 1:" "$TMP_ROOT/eq_dir.log"; then
        fail "plan_tsv_empty_truncated_duplicate_and_bytewise_sort" "compare_file failed with '=' in path: $(cat "$TMP_ROOT/eq_dir.log")"
        return
    fi

    pass "plan_tsv_empty_truncated_duplicate_and_bytewise_sort"
}

# Run all tests
test_tr_chain
test_tr_diamond
test_tr_keeps_parallel_paths
test_plan_table_reduction
test_parts_resolve_two_milestone_package
test_planted_cycle_reported
test_no_package_beads_counted
test_broken_parent_chain_fails
test_missing_milestone_label_fails
test_deterministic_bytes
test_witness_is_smallest_pair
test_implicit_edges_order_parts
test_related_links_undirected
test_related_missing_fails
test_stale_output_fails
test_validation_guards
test_witnesses_exist_in_export
test_metamorphic_relations
test_negative_control_reach_u_bug
test_posix_and_hygiene
test_invalid_plan_tsv_header_fails
test_plan_tsv_empty_truncated_duplicate_and_bytewise_sort

printf 'SUMMARY: passed=%d failed=%d\n' "$PASS" "$FAIL"
if [ "$FAIL" -ne 0 ]; then
    exit 1
fi
exit 0
