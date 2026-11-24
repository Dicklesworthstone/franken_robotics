# POSIX awk graph engine for work-package part and package dependency graphs.
#
# Input (tab-separated, LC_ALL=C):
#   NODE<TAB><node_id>
#   EDGE<TAB><from><TAB><to><TAB><witness_from><TAB><witness_to>
#
# Notes:
#   - Edge u -> v means "u depends on v" ("u needs v").
#   - Multiple occurrences of (u, v) retain the witness pair (wf, wt) smallest
#     in bytewise key order, with any real bead witness preferred over "implicit".
#   - Uses strict POSIX awk only (no gawk extensions, no recursion).
#
# Output (tab-separated, deterministic bytewise order):
#   RAW_EDGE<TAB><u><TAB><v><TAB><wf><TAB><wt>
#   CYCLE<TAB><collapsed_id><TAB><comma_members>
#   CYCLE_EDGE<TAB><collapsed_id><TAB><u><TAB><v><TAB><wf><TAB><wt>
#   CONDENSED_NODE<TAB><cid>
#   REDUCED_EDGE<TAB><cu><TAB><cv><TAB><wf><TAB><wt>
#   REMOVED_EDGE<TAB><cu><TAB><cv><TAB><wf><TAB><wt>
#   ROOT<TAB><cid>
#   LONGEST_CHAIN<TAB><edge_count><TAB><end_cid><TAB><comma_path_root_to_end>
#   STATS<TAB><n_nodes><TAB><n_raw_edges><TAB><n_cycles><TAB><n_cnodes><TAB><n_cedges><TAB><n_reduced><TAB><n_removed>

BEGIN {
    FS = "\t"
    OFS = "\t"
}

function witness_is_better(nwf, nwt, cwf, cwt) {
    if (cwf == "") return 1
    if (nwf == "implicit" && cwf != "implicit") return 0
    if (nwf != "implicit" && cwf == "implicit") return 1
    if (nwf < cwf) return 1
    if (nwf > cwf) return 0
    return (nwt < cwt) ? 1 : 0
}

function add_edge(u, v, wf, wt,    k) {
    if (u == "" || v == "" || u == v) return
    node_seen[u] = 1
    node_seen[v] = 1
    k = u SUBSEP v
    if (!(k in edge_seen)) {
        edge_seen[k] = 1
        ewf[k] = wf
        ewt[k] = wt
    } else if (witness_is_better(wf, wt, ewf[k], ewt[k])) {
        ewf[k] = wf
        ewt[k] = wt
    }
}

$1 == "NODE" {
    if ($2 != "") node_seen[$2] = 1
    next
}

$1 == "EDGE" {
    add_edge($2, $3, $4, $5)
    next
}

# Iterative O(n log n) heapsort on 1-indexed array a[1..n] (bytewise order).
function sift_down(a, start, end_idx,    root, child, swap_idx, tmp) {
    root = start
    while (root * 2 <= end_idx) {
        child = root * 2
        swap_idx = root
        if (a[swap_idx] < a[child]) swap_idx = child
        if (child + 1 <= end_idx && a[swap_idx] < a[child + 1]) swap_idx = child + 1
        if (swap_idx == root) return
        tmp = a[root]
        a[root] = a[swap_idx]
        a[swap_idx] = tmp
        root = swap_idx
    }
}

function heapsort(a, n,    i, tmp) {
    if (n <= 1) return
    for (i = int(n / 2); i >= 1; i--) {
        sift_down(a, i, n)
    }
    for (i = n; i >= 2; i--) {
        tmp = a[1]
        a[1] = a[i]
        a[i] = tmp
        sift_down(a, 1, i - 1)
    }
}

END {
    # 1. Collect and sort all nodes.
    n_nodes = 0
    for (u in node_seen) {
        nodes[++n_nodes] = u
    }
    heapsort(nodes, n_nodes)

    # 2. Build sorted adjacency list for the input graph.
    n_raw_edges = 0
    for (k in edge_seen) {
        split(k, ep, SUBSEP)
        u = ep[1]
        v = ep[2]
        deg[u]++
        adj[u SUBSEP deg[u]] = v
        n_raw_edges++
    }
    for (i = 1; i <= n_nodes; i++) {
        u = nodes[i]
        d = deg[u] + 0
        if (d > 1) {
            for (j = 1; j <= d; j++) tmp_adj[j] = adj[u SUBSEP j]
            heapsort(tmp_adj, d)
            for (j = 1; j <= d; j++) adj[u SUBSEP j] = tmp_adj[j]
        }
    }

    # Emit RAW_EDGE in deterministic sorted (u, v) order.
    for (i = 1; i <= n_nodes; i++) {
        u = nodes[i]
        d = deg[u] + 0
        for (j = 1; j <= d; j++) {
            v = adj[u SUBSEP j]
            k = u SUBSEP v
            print "RAW_EDGE", u, v, ewf[k], ewt[k]
        }
    }

    # 3. Iterative Tarjan SCC in deterministic sorted node & successor order.
    t_counter = 0
    ts_top = 0
    n_sccs = 0
    for (i = 1; i <= n_nodes; i++) {
        start = nodes[i]
        if (start in t_idx) continue

        t_idx[start] = ++t_counter
        t_low[start] = t_idx[start]
        ts_stack[++ts_top] = start
        on_ts[start] = 1

        cs_top = 1
        cs_node[1] = start
        cs_next[1] = 1

        while (cs_top > 0) {
            v = cs_node[cs_top]
            idx_k = cs_next[cs_top]
            d = deg[v] + 0
            if (idx_k <= d) {
                cs_next[cs_top] = idx_k + 1
                w = adj[v SUBSEP idx_k]
                if (!(w in t_idx)) {
                    t_idx[w] = ++t_counter
                    t_low[w] = t_idx[w]
                    ts_stack[++ts_top] = w
                    on_ts[w] = 1
                    cs_top++
                    cs_node[cs_top] = w
                    cs_next[cs_top] = 1
                } else if (w in on_ts) {
                    if (t_idx[w] < t_low[v]) t_low[v] = t_idx[w]
                }
            } else {
                cs_top--
                if (cs_top > 0) {
                    parent_v = cs_node[cs_top]
                    if (t_low[v] < t_low[parent_v]) t_low[parent_v] = t_low[v]
                }
                if (t_low[v] == t_idx[v]) {
                    cnt = 0
                    while (1) {
                        x = ts_stack[ts_top--]
                        delete on_ts[x]
                        scc_tmp[++cnt] = x
                        if (x == v) break
                    }
                    heapsort(scc_tmp, cnt)
                    cname = scc_tmp[1]
                    for (m = 2; m <= cnt; m++) cname = cname "+" scc_tmp[m]
                    n_sccs++
                    scc_name[n_sccs] = cname
                    scc_size[cname] = cnt
                    for (m = 1; m <= cnt; m++) {
                        node_comp[scc_tmp[m]] = cname
                        scc_mem[cname SUBSEP m] = scc_tmp[m]
                    }
                }
            }
        }
    }

    # 4. Sort and emit cycles (SCCs with size > 1) and their internal edges.
    n_cycles = 0
    for (i = 1; i <= n_sccs; i++) {
        cid = scc_name[i]
        if (scc_size[cid] > 1) {
            cycles[++n_cycles] = cid
        }
    }
    heapsort(cycles, n_cycles)
    for (i = 1; i <= n_cycles; i++) {
        cid = cycles[i]
        sz = scc_size[cid]
        mlist = scc_mem[cid SUBSEP 1]
        for (m = 2; m <= sz; m++) mlist = mlist "," scc_mem[cid SUBSEP m]
        print "CYCLE", cid, mlist
        for (m = 1; m <= sz; m++) {
            u = scc_mem[cid SUBSEP m]
            d = deg[u] + 0
            for (j = 1; j <= d; j++) {
                v = adj[u SUBSEP j]
                if (node_comp[v] == cid) {
                    k = u SUBSEP v
                    print "CYCLE_EDGE", cid, u, v, ewf[k], ewt[k]
                }
            }
        }
    }

    # 5. Build condensed DAG over collapsed SCC nodes.
    n_cnodes = n_sccs
    for (i = 1; i <= n_sccs; i++) cnodes[i] = scc_name[i]
    heapsort(cnodes, n_cnodes)

    n_cedges = 0
    for (i = 1; i <= n_nodes; i++) {
        u = nodes[i]
        cu = node_comp[u]
        d = deg[u] + 0
        for (j = 1; j <= d; j++) {
            v = adj[u SUBSEP j]
            cv = node_comp[v]
            if (cu == cv) continue
            ck = cu SUBSEP cv
            ok = u SUBSEP v
            if (!(ck in cedge_seen)) {
                cedge_seen[ck] = 1
                cewf[ck] = ewf[ok]
                cewt[ck] = ewt[ok]
                cdeg[cu]++
                cadj[cu SUBSEP cdeg[cu]] = cv
                n_cedges++
            } else if (witness_is_better(ewf[ok], ewt[ok], cewf[ck], cewt[ck])) {
                cewf[ck] = ewf[ok]
                cewt[ck] = ewt[ok]
            }
        }
    }
    for (i = 1; i <= n_cnodes; i++) {
        cu = cnodes[i]
        print "CONDENSED_NODE", cu
        d = cdeg[cu] + 0
        if (d > 1) {
            for (j = 1; j <= d; j++) tmp_adj[j] = cadj[cu SUBSEP j]
            heapsort(tmp_adj, d)
            for (j = 1; j <= d; j++) cadj[cu SUBSEP j] = tmp_adj[j]
        }
    }

    # 6. Compute reach(w) on condensed DAG (nodes reachable in >= 1 step).
    for (i = 1; i <= n_cnodes; i++) {
        src = cnodes[i]
        qhead = 1
        qtail = 0
        d = cdeg[src] + 0
        for (j = 1; j <= d; j++) {
            w = cadj[src SUBSEP j]
            rk = src SUBSEP w
            if (!(rk in reach)) {
                reach[rk] = 1
                q[++qtail] = w
            }
        }
        while (qhead <= qtail) {
            cur = q[qhead++]
            cd = cdeg[cur] + 0
            for (j = 1; j <= cd; j++) {
                nxt = cadj[cur SUBSEP j]
                rk = src SUBSEP nxt
                if (!(rk in reach)) {
                    reach[rk] = 1
                    q[++qtail] = nxt
                }
            }
        }
    }

    # 7. Transitive reduction of condensed DAG:
    #    u -> v is removed iff some OTHER successor w != v of u has v in reach(w).
    n_reduced = 0
    n_removed = 0
    for (i = 1; i <= n_cnodes; i++) {
        cu = cnodes[i]
        d = cdeg[cu] + 0
        for (j = 1; j <= d; j++) {
            cv = cadj[cu SUBSEP j]
            redundant = 0
            for (m = 1; m <= d; m++) {
                cw = cadj[cu SUBSEP m]
                if (cw != cv && ((cw SUBSEP cv) in reach)) {
                    redundant = 1
                    break
                }
            }
            ck = cu SUBSEP cv
            if (redundant) {
                n_removed++
                print "REMOVED_EDGE", cu, cv, cewf[ck], cewt[ck]
            } else {
                n_reduced++
                print "REDUCED_EDGE", cu, cv, cewf[ck], cewt[ck]
            }
        }
    }

    # 8. Roots (out-degree 0 in condensed DAG) and longest dependency chain.
    for (i = 1; i <= n_cnodes; i++) {
        cu = cnodes[i]
        out_rem[cu] = cdeg[cu] + 0
        dist[cu] = 0
        prev_dep[cu] = ""
        d = cdeg[cu] + 0
        for (j = 1; j <= d; j++) {
            cv = cadj[cu SUBSEP j]
            rdeg[cv]++
            radj[cv SUBSEP rdeg[cv]] = cu
        }
    }
    qhead = 1
    qtail = 0
    for (i = 1; i <= n_cnodes; i++) {
        cu = cnodes[i]
        if (out_rem[cu] == 0) {
            print "ROOT", cu
            q[++qtail] = cu
        }
    }
    while (qhead <= qtail) {
        cv = q[qhead++]
        rd = rdeg[cv] + 0
        for (j = 1; j <= rd; j++) {
            cu = radj[cv SUBSEP j]
            cand = dist[cv] + 1
            if (cand > dist[cu] || (cand == dist[cu] && (prev_dep[cu] == "" || cv < prev_dep[cu]))) {
                dist[cu] = cand
                prev_dep[cu] = cv
            }
            out_rem[cu]--
            if (out_rem[cu] == 0) {
                q[++qtail] = cu
            }
        }
    }

    if (n_cnodes > 0) {
        best_u = cnodes[1]
        best_d = dist[best_u]
        for (i = 2; i <= n_cnodes; i++) {
            cu = cnodes[i]
            if (dist[cu] > best_d) {
                best_d = dist[cu]
                best_u = cu
            }
        }
        clen = 0
        cur = best_u
        while (cur != "") {
            chain[++clen] = cur
            cur = prev_dep[cur]
        }
        cpath = chain[clen]
        for (i = clen - 1; i >= 1; i--) {
            cpath = cpath "," chain[i]
        }
        print "LONGEST_CHAIN", best_d, best_u, cpath
    } else {
        print "LONGEST_CHAIN", 0, "", ""
    }

    print "STATS", n_nodes, n_raw_edges, n_cycles, n_cnodes, n_cedges, n_reduced, n_removed
}
