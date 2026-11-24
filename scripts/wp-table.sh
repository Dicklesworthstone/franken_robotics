#!/bin/sh
# Regenerate and check the FrankenRobotics work-package table from the bead graph
# by Tarjan SCC collapse and transitive reduction (N44-03).
#
# Runtime: POSIX sh + jq + POSIX awk only. Never invokes the tracker binary (`br`/`bv`).
# Writes only inside the configured output directory (default: docs/delivery/) and its
# own scratch directory.

set -eu
export LC_ALL=C

# ==============================================================================
# Export schema field bindings (single table; edit here if tracker export changes)
# ==============================================================================
FIELD_ID="id"
FIELD_KEY="external_ref"
FIELD_TYPE="issue_type"
FIELD_STATUS="status"
FIELD_LABELS="labels"
FIELD_TITLE="title"
FIELD_DESCRIPTION="description"
FIELD_GATES="gates"
FIELD_DEPS="dependencies"
FIELD_DEP_TARGET="depends_on_id"
FIELD_DEP_TYPE="type"
DEP_TYPE_PARENT="parent-child"
DEP_TYPE_BLOCKS="blocks"
DEP_TYPE_RELATED="related"

SCRIPT_DIR=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH='' cd -- "$SCRIPT_DIR/.." && pwd)
AWK_GRAPH="$SCRIPT_DIR/lib/wp_graph.awk"

MODE=""
INPUT_FILE="$REPO_ROOT/.beads/issues.jsonl"
PLAN_FILE="$REPO_ROOT/docs/delivery/work-packages.plan.tsv"
USE_PLAN=1
OUT_DIR="$REPO_ROOT/docs/delivery"
STRICT_ACYCLIC=0

usage() {
    cat <<'EOF'
Usage: scripts/wp-table.sh <generate|check> [options]

Modes:
  generate              Write work-packages.json and work-packages.md into --out-dir
  check                 Regenerate into a scratch dir, check staleness, graph integrity,
                        and drafted related-link coverage

Options:
  --input <path>        Tracker JSONL export (default: .beads/issues.jsonl)
  --plan <path>         Drafted plan TSV (default: docs/delivery/work-packages.plan.tsv)
  --no-plan             Skip drafted plan TSV comparison (for standalone fixtures)
  --out-dir <dir>       Output directory (default: docs/delivery)
  --strict-acyclic      In check mode, fail if any part-level cycle survives the split
  -h, --help            Print this usage message
EOF
}

if [ $# -lt 1 ]; then
    usage >&2
    exit 3
fi

case "$1" in
    generate|check)
        MODE="$1"
        shift
        ;;
    -h|--help)
        usage
        exit 0
        ;;
    *)
        printf 'wp-table: unknown mode: %s\n' "$1" >&2
        usage >&2
        exit 3
        ;;
esac

while [ $# -gt 0 ]; do
    case "$1" in
        --input)
            [ $# -ge 2 ] || { printf 'wp-table: --input requires a path\n' >&2; exit 3; }
            INPUT_FILE="$2"
            shift 2
            ;;
        --plan)
            [ $# -ge 2 ] || { printf 'wp-table: --plan requires a path\n' >&2; exit 3; }
            PLAN_FILE="$2"
            USE_PLAN=1
            shift 2
            ;;
        --no-plan)
            USE_PLAN=0
            shift
            ;;
        --out-dir)
            [ $# -ge 2 ] || { printf 'wp-table: --out-dir requires a directory\n' >&2; exit 3; }
            OUT_DIR="$2"
            shift 2
            ;;
        --strict-acyclic)
            STRICT_ACYCLIC=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            printf 'wp-table: unknown option: %s\n' "$1" >&2
            usage >&2
            exit 3
            ;;
    esac
done

if [ ! -f "$AWK_GRAPH" ]; then
    printf 'wp-table: missing graph library: %s\n' "$AWK_GRAPH" >&2
    exit 1
fi
if [ ! -f "$INPUT_FILE" ] || [ ! -s "$INPUT_FILE" ]; then
    printf 'wp-table: tracker export missing or empty: %s\n' "$INPUT_FILE" >&2
    exit 1
fi
if [ "$USE_PLAN" -eq 1 ] && [ ! -f "$PLAN_FILE" ]; then
    printf 'wp-table: drafted plan TSV missing: %s\n' "$PLAN_FILE" >&2
    exit 1
fi

SCRATCH_BASE="${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-wp-table"
mkdir -p "$SCRATCH_BASE"
TMP_WORK=$(mktemp -d "$SCRATCH_BASE/run.XXXXXX")

# Convert plan TSV (if enabled) into JSON map for jq.
# Write awk output to a file before invoking jq so POSIX sh (no pipefail) does
# not swallow an invalid-header failure from awk.
PLAN_JSON="$TMP_WORK/plan.json"
if [ "$USE_PLAN" -eq 1 ]; then
    PLAN_ROWS="$TMP_WORK/plan.rows.tsv"
    awk -F '\t' '
        NR == 1 {
            if (NF != 3 || $1 != "package" || $2 != "first_milestone" || $3 != "depends_on") {
                printf("wp-table: invalid plan TSV header: %s\n", $0) > "/dev/stderr"
                bad = 1
                exit 1
            }
            next
        }
        NF == 0 || $1 ~ /^#/ { next }
        {
            if (NF != 3 || $1 == "" || $2 == "") {
                printf("wp-table: malformed plan TSV row at line %d (expected 3 tab-separated fields): %s\n", NR, $0) > "/dev/stderr"
                bad = 1
                exit 1
            }
            if ($1 in seen_pkg) {
                printf("wp-table: duplicate package key in plan TSV at line %d: %s\n", NR, $1) > "/dev/stderr"
                bad = 1
                exit 1
            }
            seen_pkg[$1] = 1
            printf("%s\t%s\t%s\n", $1, $2, $3)
        }
        END {
            if (bad) exit 1
            if (NR == 0) {
                printf("wp-table: invalid plan TSV header: empty file\n") > "/dev/stderr"
                exit 1
            }
        }
    ' < "$PLAN_FILE" > "$PLAN_ROWS"
    jq -Rs '
        split("\n") | map(select(length > 0)) |
        map(split("\t") | {
            key: .[0],
            first_milestone: .[1],
            depends_on: (if (.[2] // "-") == "-" or (.[2] // "") == "" then [] else (.[2] | split(",") | map(select(length > 0))) end)
        }) | sort_by(.key)
    ' "$PLAN_ROWS" > "$PLAN_JSON"
else
    printf 'null\n' > "$PLAN_JSON"
fi

# ==============================================================================
# Step 1-5: Validate schema, resolve parent chains & milestones, extract graph
# ==============================================================================
EXTRACT_JSON="$TMP_WORK/extract.json"

jq -s \
    --arg f_id "$FIELD_ID" \
    --arg f_key "$FIELD_KEY" \
    --arg f_type "$FIELD_TYPE" \
    --arg f_status "$FIELD_STATUS" \
    --arg f_labels "$FIELD_LABELS" \
    --arg f_title "$FIELD_TITLE" \
    --arg f_desc "$FIELD_DESCRIPTION" \
    --arg f_gates "$FIELD_GATES" \
    --arg f_deps "$FIELD_DEPS" \
    --arg f_dep_target "$FIELD_DEP_TARGET" \
    --arg f_dep_type "$FIELD_DEP_TYPE" \
    --arg dt_parent "$DEP_TYPE_PARENT" \
    --arg dt_blocks "$DEP_TYPE_BLOCKS" \
    --arg dt_related "$DEP_TYPE_RELATED" \
    --slurpfile plan_arg "$PLAN_JSON" \
'
def die($msg): ("wp-table: " + $msg + "\n") | halt_error(1);

def has_any_key($arr; $k): any($arr[]; type == "object" and has($k));

def parse_gates_from_desc($d):
    if ($d | type) != "string" then []
    else
        ($d | split("\n")
            | map(select(startswith("**Gates measured or decided here:**")))
            | map(sub("^\\*\\*Gates measured or decided here:\\*\\*[[:space:]]*"; "")
                | sub("\\.[[:space:]]*$"; "")
                | split(","))
            | add // []
            | map(gsub("^[[:space:]]+|[[:space:]]+$"; ""))
            | map(select(length > 0)))
    end;

def extract_gates($it):
    ((if ($it[$f_gates] | type) == "array" then $it[$f_gates] else [] end)
     + parse_gates_from_desc($it[$f_desc]))
    | unique | sort;

def is_crate_label($l):
    ($l | test("^frob-[a-z0-9-]+$")) or $l == "xtask" or $l == "viewer" or $l == "lean";

def is_wp_key($k): ($k | test("^WP-[0-9][0-9]$"));
def is_top_epic($k): ($k == "EPIC-PROGRAM" or ($k | test("^EPIC-M[0-8]$")));

. as $raw |
if ($raw | length) == 0 then die("tracker export contains no issues") else . end |

# Schema presence check over bound field names
(
    [
        (if has_any_key($raw; $f_id) then empty else $f_id end),
        (if has_any_key($raw; $f_key) then empty else $f_key end),
        (if has_any_key($raw; $f_type) then empty else $f_type end),
        (if has_any_key($raw; $f_status) then empty else $f_status end),
        (if has_any_key($raw; $f_labels) then empty else $f_labels end)
    ]
) as $missing_fields |
if ($missing_fields | length) > 0 then
    die("required schema field(s) absent from export: " + ($missing_fields | join(", ")))
else . end |

# Check dependency object schema whenever dependencies are present
(
    [ $raw[] | (.[$f_deps] // [])[] |
      select((has($f_dep_target) | not) or (has($f_dep_type) | not)) ]
) as $bad_dep_objs |
if ($bad_dep_objs | length) > 0 then
    die("dependency object missing bound fields " + $f_dep_target + " / " + $f_dep_type)
else . end |

# Check duplicate IDs
(
    $raw | map(.[$f_id]) | group_by(.) | map(select(length > 1) | .[0])
) as $dup_ids |
if ($dup_ids | length) > 0 then
    die("duplicate issue id: " + ($dup_ids | join(", ")))
else . end |

# Partition non-work-item unkeyed records (tombstones or unkeyed chores such as policy canaries)
(
    [ $raw[] | select((.[$f_status] == "tombstone") or ((.[$f_key] // "") == "" and .[$f_type] == "chore")) | .[$f_id] ] | sort
) as $ignored_unkeyed |
(
    [ $raw[] | select(((.[$f_status] == "tombstone") or ((.[$f_key] // "") == "" and .[$f_type] == "chore")) | not) ]
) as $active |

# Any active non-chore issue missing a key is an error
(
    [ $active[] | select((.[$f_key] // "") == "") | .[$f_id] ]
) as $missing_key_ids |
if ($missing_key_ids | length) > 0 then
    die("issue(s) missing bead key (" + $f_key + "): " + ($missing_key_ids | join(", ")))
else . end |

# Duplicate bead key check (Step 1)
(
    $active | map(.[$f_key]) | group_by(.) | map(select(length > 1) | .[0]) | sort
) as $dup_keys |
if ($dup_keys | length) > 0 then
    die("duplicate bead key: " + ($dup_keys | join(", ")))
else . end |

# Build normalized issue records keyed by id and by bead key
(
    $active | map(
        . as $it |
        (.[$f_labels] // []) as $lbls |
        ([$lbls[] | select(test("^m[0-8]$")) | ascii_upcase]) as $ms |
        {
            id: .[$f_id],
            key: .[$f_key],
            type: .[$f_type],
            status: .[$f_status],
            title: (.[$f_title] // ""),
            labels: ($lbls | unique | sort),
            milestones_found: $ms,
            gates: extract_gates($it),
            parents: [(.[$f_deps] // [])[] | select(.[$f_dep_type] == $dt_parent) | .[$f_dep_target]],
            blocks: [(.[$f_deps] // [])[] | select(.[$f_dep_type] == $dt_blocks) | .[$f_dep_target]],
            related: [(.[$f_deps] // [])[] | select(.[$f_dep_type] == $dt_related) | .[$f_dep_target]]
        }
    )
) as $items |

# Step 3: Validate milestone label (m0..m8) on every bead
(
    [ $items[] | select((.milestones_found | length) != 1) |
      if (.milestones_found | length) == 0 then
          .key + " (no milestone label m0..m8)"
      else
          .key + " (multiple milestone labels: " + (.milestones_found | join(",")) + ")"
      end
    ] | sort
) as $bad_milestones |
if ($bad_milestones | length) > 0 then
    die("milestone label error: " + ($bad_milestones | join("; ")))
else . end |

($items | map(. + {milestone: .milestones_found[0]})) as $norm |
($norm | map({(.id): .}) | add // {}) as $by_id |
($norm | map({(.key): .}) | add // {}) as $by_key |

# Step 2: Parent chain resolution
def resolve_chain($it; $seen):
    if ($it.parents | length) > 1 then
        {err: ("multiple parents on " + $it.key + ": " + ($it.parents | join(", ")))}
    elif ($it.parents | length) == 0 then
        if is_wp_key($it.key) then {kind: "wp", pkg: $it.key}
        elif is_top_epic($it.key) then {kind: "top", top: $it.key}
        else {err: ("broken parent chain: " + $it.key + " has no parent")}
        end
    else
        $it.parents[0] as $pid |
        if ($seen | index($pid)) != null then
            {err: ("parent chain loop involving " + $it.key + " -> " + $pid)}
        elif $by_id[$pid] == null then
            {err: ("broken parent chain: " + $it.key + " references missing parent " + $pid)}
        else
            $by_id[$pid] as $p |
            if is_wp_key($p.key) then
                {kind: "packaged", pkg: $p.key}
            elif is_top_epic($p.key) then
                {kind: "unpackaged", top: $p.key}
            else
                resolve_chain($p; $seen + [$pid])
            end
        end
    end;

# Validate WP epics own parent chains (must not break or loop)
def check_wp_parent($it; $seen):
    if ($it.parents | length) == 0 then {ok: true}
    elif ($it.parents | length) > 1 then {err: ("multiple parents on package epic " + $it.key)}
    else
        $it.parents[0] as $pid |
        if ($seen | index($pid)) != null then
            {err: ("parent chain loop on package epic " + $it.key)}
        elif $by_id[$pid] == null then
            {err: ("broken parent chain on package epic " + $it.key + " -> " + $pid)}
        else
            $by_id[$pid] as $p |
            if is_top_epic($p.key) then {ok: true}
            else check_wp_parent($p; $seen + [$pid])
            end
        end
    end;

(
    [ $norm[] | select(is_wp_key(.key)) | check_wp_parent(.; [.id]) | select(.err != null) | .err ]
) as $wp_parent_errs |
if ($wp_parent_errs | length) > 0 then
    die($wp_parent_errs | join("; "))
else . end |

# Resolve every non-structural bead
(
    [ $norm[] | select((is_wp_key(.key) or is_top_epic(.key)) | not) |
      . as $b |
      (resolve_chain($b; [$b.id])) as $r |
      {bead: $b, res: $r}
    ]
) as $resolved |

(
    [ $resolved[] | select(.res.err != null) | .res.err ] | sort
) as $chain_errs |
if ($chain_errs | length) > 0 then
    die($chain_errs | join("; "))
else . end |

# Package epics in export
([ $norm[] | select(is_wp_key(.key)) ] | sort_by(.key)) as $wp_epics |
($wp_epics | map(.key)) as $wp_keys |

# Validate package keys against plan TSV when provided
($plan_arg[0]) as $plan |
(
    if $plan != null then
        ($plan | map(.key)) as $plan_keys |
        ($wp_keys - $plan_keys) as $unknown_wps |
        ($plan_keys - $wp_keys) as $missing_wps |
        [
            ($unknown_wps[] | . as $uk |
                ([ $resolved[] | select(.res.kind == "packaged" and .res.pkg == $uk) | .bead.key ] | sort) as $kids |
                "unknown package key: " + $uk + (if ($kids | length) > 0 then " (children: " + ($kids | join(", ")) + ")" else "" end)),
            ($missing_wps[] | "missing package key from plan: " + .)
        ]
    else []
    end
) as $plan_key_errs |
if ($plan_key_errs | length) > 0 then
    die($plan_key_errs | join("; "))
else . end |

# Packaged & unpackaged beads
([ $resolved[] | select(.res.kind == "packaged") | .bead + {pkg: .res.pkg, part: (.res.pkg + "@" + .bead.milestone)} ] | sort_by(.key)) as $packaged |
([ $resolved[] | select(.res.kind == "unpackaged") | {key: .bead.key, milestone: .bead.milestone, status: .bead.status, top_epic: .res.top} ] | sort_by(.key)) as $unpackaged |
($packaged | map({(.id): .}) | add // {}) as $pkg_by_id |

# Part nodes (Step 4): one per (pkg, milestone) that has >= 1 bead
($packaged | map(.part) | unique | sort) as $part_nodes |

# Blocking edges between packaged beads (Step 5)
(
    [ $packaged[] | . as $a |
      $a.blocks[] | . as $bid |
      $pkg_by_id[$bid] as $b |
      select($b != null) |
      select($a.part != $b.part) |
      {from: $a.part, to: $b.part, wf: $a.key, wt: $b.key, pfrom: $a.pkg, pto: $b.pkg}
    ]
) as $bead_part_edges |

# Implicit edges between parts of one package: WP-nn@Mk -> WP-nn@Mj for j < k
(
    $packaged
    | group_by(.pkg)
    | map(
        .[0].pkg as $p |
        (map(.milestone) | unique | sort) as $ms |
        [ range(0; ($ms | length)) as $i |
          range($i + 1; ($ms | length)) as $j |
          {from: ($p + "@" + $ms[$j]), to: ($p + "@" + $ms[$i]), wf: "implicit", wt: ""}
        ]
    ) | add // []
) as $implicit_part_edges |

# Package-view raw edges (Step 8): cross-package edges from unreduced part graph (never implicit)
(
    [ $bead_part_edges[] | select(.pfrom != .pto) |
      {from: .pfrom, to: .pto, wf: .wf, wt: .wt}
    ]
) as $pkg_raw_edges |

# Undirected related links between package epics
(
    [ $wp_epics[] | . as $w |
      $w.related[] | $by_id[.] as $t |
      select($t != null and is_wp_key($t.key) and $t.key != $w.key) |
      (if $w.key < $t.key then [$w.key, $t.key] else [$t.key, $w.key] end)
    ] | unique | sort
) as $wp_related_undir |

# Per-package lookup from plan
(if $plan != null then ($plan | map({(.key): .}) | add // {}) else {} end) as $plan_by_key |

# Per-package column aggregations (Step 9)
def kind_counts($kids; pred_fn):
    ($kids | map(select(pred_fn))) as $sub |
    ($sub | map(select(.status == "closed")) | length) as $cl |
    (($sub | length) - $cl) as $op |
    {open: $op, closed: $cl, total: ($sub | length)};

(
    $wp_epics | map(
        . as $w |
        ([ $packaged[] | select(.pkg == $w.key) ]) as $kids |
        ($kids | map(.milestone) | unique | sort) as $child_ms |
        ($child_ms | map($w.key + "@" + .)) as $wparts |
        ([$kids[].labels[] | select(is_crate_label(.))] | unique | sort) as $crates |
        ([$kids[].gates[]] | unique | sort) as $child_gates |
        (($w.gates + $child_gates) | unique | sort) as $all_gates |
        ([ $wp_related_undir[] | if .[0] == $w.key then .[1] elif .[1] == $w.key then .[0] else empty end ] | unique | sort) as $wrel |
        ($kids | map(select(.status == "closed")) | length) as $tot_closed |
        {
            key: $w.key,
            id: $w.id,
            title: $w.title,
            epic_milestone: $w.milestone,
            plan_first_milestone: (if $plan_by_key[$w.key] != null then $plan_by_key[$w.key].first_milestone else null end),
            plan_depends_on: (if $plan_by_key[$w.key] != null then ($plan_by_key[$w.key].depends_on | sort) else [] end),
            milestones: $child_ms,
            parts: $wparts,
            crates: $crates,
            key_gates: $w.gates,
            all_gates: $all_gates,
            related: $wrel,
            children: {
                total: {open: (($kids | length) - $tot_closed), closed: $tot_closed, total: ($kids | length)},
                by_kind: {
                    seed: kind_counts($kids; (.key | test("^BD-"))),
                    new: kind_counts($kids; ((.key | test("^BD-")) | not)),
                    pilot: kind_counts($kids; ((.labels | index("pilot")) != null)),
                    kill_gate: kind_counts($kids; ((.labels | index("kill-gate")) != null)),
                    e2e: kind_counts($kids; ((.labels | index("e2e")) != null)),
                    upstream: kind_counts($kids; ((.labels | index("upstream")) != null))
                }
            }
        }
    )
) as $pkg_meta |

# Milestone mismatches: epic milestone != first milestone of its children
(
    [ $pkg_meta[] |
      select((.milestones | length) > 0 and .epic_milestone != .milestones[0]) |
      {
          package: .key,
          epic_milestone: .epic_milestone,
          first_child_milestone: .milestones[0],
          child_milestones: .milestones
      }
    ] | sort_by(.package)
) as $milestone_mismatches |

{
    schema_fields: {
        id: $f_id,
        key: $f_key,
        type: $f_type,
        status: $f_status,
        labels: $f_labels,
        title: $f_title,
        description: $f_desc,
        dependencies: $f_deps,
        dep_target: $f_dep_target,
        dep_type: $f_dep_type
    },
    use_plan: ($plan != null),
    plan_edges: (if $plan != null then [ $plan[] | .key as $u | .depends_on[] | {from: $u, to: .} ] | sort_by([.from, .to]) else [] end),
    ignored_unkeyed: $ignored_unkeyed,
    unpackaged: $unpackaged,
    packaged_count: ($packaged | length),
    wp_keys: $wp_keys,
    part_nodes: $part_nodes,
    part_edges_in: ($bead_part_edges + $implicit_part_edges),
    pkg_edges_in: $pkg_raw_edges,
    wp_related_undir: $wp_related_undir,
    milestone_mismatches: $milestone_mismatches,
    packages: $pkg_meta
}
' "$INPUT_FILE" > "$EXTRACT_JSON"

# Build TSV inputs for wp_graph.awk
PART_IN="$TMP_WORK/part_graph.in"
PART_OUT="$TMP_WORK/part_graph.out"
PKG_IN="$TMP_WORK/pkg_graph.in"
PKG_OUT="$TMP_WORK/pkg_graph.out"

jq -r '
    (.part_nodes[] | "NODE\t" + .),
    (.part_edges_in[] | "EDGE\t" + .from + "\t" + .to + "\t" + .wf + "\t" + .wt)
' "$EXTRACT_JSON" > "$PART_IN"

jq -r '
    (.wp_keys[] | "NODE\t" + .),
    (.pkg_edges_in[] | "EDGE\t" + .from + "\t" + .to + "\t" + .wf + "\t" + .wt)
' "$EXTRACT_JSON" > "$PKG_IN"

# Run Tarjan SCC + Transitive Reduction in POSIX awk
awk -F '\t' -f "$AWK_GRAPH" "$PART_IN" > "$PART_OUT"
awk -F '\t' -f "$AWK_GRAPH" "$PKG_IN" > "$PKG_OUT"

# Convert awk outputs to structured JSON
awk_out_to_json() {
    jq -Rs '
        def mk_wit($wf; $wt):
            if $wf == "implicit" then "implicit" else [$wf, $wt] end;

        (split("\n") | map(select(length > 0) | split("\t"))) as $rows |
        ($rows | map(select(.[0] == "STATS")) | .[0] // ["STATS","0","0","0","0","0","0","0"]) as $st |
        ($rows | map(select(.[0] == "LONGEST_CHAIN")) | .[0] // ["LONGEST_CHAIN","0","",""]) as $lc |
        (
            $rows | map(select(.[0] == "CYCLE_EDGE"))
            | group_by(.[1])
            | map({key: .[0][1], value: map({from: .[2], to: .[3], witness: mk_wit(.[4]; .[5])})})
            | from_entries
        ) as $cyc_edges |
        {
            stats: {
                nodes: ($st[1] | tonumber),
                raw_edges: ($st[2] | tonumber),
                cycles: ($st[3] | tonumber),
                condensed_nodes: ($st[4] | tonumber),
                condensed_edges: ($st[5] | tonumber),
                reduced_edges: ($st[6] | tonumber),
                removed_edges: ($st[7] | tonumber)
            },
            condensed_nodes: [ $rows[] | select(.[0] == "CONDENSED_NODE") | .[1] ],
            raw_edges: [ $rows[] | select(.[0] == "RAW_EDGE") | {from: .[1], to: .[2], witness: mk_wit(.[3]; .[4])} ],
            cycles: [ $rows[] | select(.[0] == "CYCLE") | {
                id: .[1],
                members: (.[2] | split(",")),
                edges: ($cyc_edges[.[1]] // [])
            } ],
            reduced_edges: [ $rows[] | select(.[0] == "REDUCED_EDGE") | {from: .[1], to: .[2], witness: mk_wit(.[3]; .[4])} ],
            removed_edges: [ $rows[] | select(.[0] == "REMOVED_EDGE") | {from: .[1], to: .[2], witness: mk_wit(.[3]; .[4])} ],
            roots: [ $rows[] | select(.[0] == "ROOT") | .[1] ],
            longest_chain: {
                edges: ($lc[1] | tonumber),
                end: (if ($lc[2] // "") == "" then null else $lc[2] end),
                path: (if ($lc[3] // "") == "" then [] else ($lc[3] | split(",")) end)
            }
        }
    ' "$1"
}

PART_JSON="$TMP_WORK/part_graph.json"
PKG_JSON="$TMP_WORK/pkg_graph.json"
awk_out_to_json "$PART_OUT" > "$PART_JSON"
awk_out_to_json "$PKG_OUT" > "$PKG_JSON"

# ==============================================================================
# Step 8-10: Assemble canonical sorted-key work-packages.json & work-packages.md
# ==============================================================================
GEN_DIR="$TMP_WORK/gen"
mkdir -p "$GEN_DIR"
GEN_JSON="$GEN_DIR/work-packages.json"
GEN_MD="$GEN_DIR/work-packages.md"

jq -n -S \
    --slurpfile ex "$EXTRACT_JSON" \
    --slurpfile pg "$PART_JSON" \
    --slurpfile kg "$PKG_JSON" \
'
$ex[0] as $E |
$pg[0] as $P |
$kg[0] as $K |

($K.stats.cycles == 0) as $pkg_acyclic |
(if $pkg_acyclic then $K.reduced_edges else $K.raw_edges end) as $pkg_out_edges |
(if $pkg_acyclic then $K.removed_edges else [] end) as $pkg_removed_edges |

# Build depends_on and depended_on_by per package from $pkg_out_edges
(
    $pkg_out_edges | group_by(.from)
    | map({key: .[0].from, value: (map(.to) | unique | sort)})
    | from_entries
) as $deps_by_pkg |
(
    $pkg_out_edges | group_by(.to)
    | map({key: .[0].to, value: (map(.from) | unique | sort)})
    | from_entries
) as $rdeps_by_pkg |

# Undirected key helper
def undir($a; $b): if $a < $b then ($a + "|" + $b) else ($b + "|" + $a) end;

($E.wp_related_undir | map({(.[0] + "|" + .[1]): true}) | add // {}) as $rel_set |
($E.plan_edges | map({(.from + "|" + .to): true}) | add // {}) as $plan_dir_set |
($E.plan_edges | map({(undir(.from; .to)): true}) | add // {}) as $plan_undir_set |
($pkg_out_edges | map({(.from + "|" + .to): true}) | add // {}) as $regen_dir_set |
($pkg_out_edges | map({(undir(.from; .to)): true}) | add // {}) as $regen_undir_set |

# Plan & related-link comparisons
(
    if $E.use_plan then
        {
            enabled: true,
            plan_edges: ($E.plan_edges | length),
            added: [ $pkg_out_edges[] | select($plan_dir_set[.from + "|" + .to] | not) ],
            removed: [ $E.plan_edges[] | select($regen_dir_set[.from + "|" + .to] | not) ],
            related_missing: [ $E.plan_edges[] | select($rel_set[undir(.from; .to)] | not) ],
            related_extra: [ $E.wp_related_undir[] | select($plan_undir_set[.[0] + "|" + .[1]] | not) ],
            regenerated_not_in_related: [ $pkg_out_edges[] | select($rel_set[undir(.from; .to)] | not) ],
            related_not_in_regenerated: [ $E.wp_related_undir[] | select($regen_undir_set[.[0] + "|" + .[1]] | not) ],
            milestone_mismatches: $E.milestone_mismatches
        }
    else
        {
            enabled: false,
            plan_edges: 0,
            added: [],
            removed: [],
            related_missing: [],
            related_extra: [],
            regenerated_not_in_related: [ $pkg_out_edges[] | select($rel_set[undir(.from; .to)] | not) ],
            related_not_in_regenerated: [ $E.wp_related_undir[] | select($regen_undir_set[.[0] + "|" + .[1]] | not) ],
            milestone_mismatches: $E.milestone_mismatches
        }
    end
) as $plan_cmp |

# Enrich packages with depends_on / depended_on_by and cyclic marker
(
    [ $K.cycles[].members[] ] | map({(.): true}) | add // {}
) as $in_pkg_cycle |

(
    $E.packages | map(. + {
        depends_on: ($deps_by_pkg[.key] // []),
        depended_on_by: ($rdeps_by_pkg[.key] // []),
        in_package_cycle: ($in_pkg_cycle[.key] // false)
    })
) as $packages_out |

{
    schema: "frob.work-packages/1",
    counts: {
        packages: ($packages_out | length),
        packaged_beads: $E.packaged_count,
        unpackaged_beads: ($E.unpackaged | length),
        ignored_unkeyed: ($E.ignored_unkeyed | length),
        parts: $P.stats.nodes,
        part_edges_raw: $P.stats.raw_edges,
        part_edges_condensed: $P.stats.condensed_edges,
        part_edges_reduced: $P.stats.reduced_edges,
        part_edges_removed: $P.stats.removed_edges,
        part_cycles: $P.stats.cycles,
        package_view_edges_raw: $K.stats.raw_edges,
        package_view_edges_output: ($pkg_out_edges | length),
        package_view_edges_removed: ($pkg_removed_edges | length),
        package_view_cycles: $K.stats.cycles,
        plan_edges: ($E.plan_edges | length),
        related_links: ($E.wp_related_undir | length)
    },
    ignored_unkeyed: $E.ignored_unkeyed,
    unpackaged: $E.unpackaged,
    parts: {
        nodes: $E.part_nodes,
        condensed_nodes: $P.condensed_nodes,
        cycles: $P.cycles,
        edges: $P.reduced_edges,
        removed_edges: $P.removed_edges,
        roots: $P.roots,
        longest_chain: $P.longest_chain
    },
    package_view: {
        acyclic: $pkg_acyclic,
        reduced: $pkg_acyclic,
        cycles: $K.cycles,
        edges: $pkg_out_edges,
        removed_edges: $pkg_removed_edges,
        roots: (if $pkg_acyclic then $K.roots else [ $packages_out[] | select((.depends_on | length) == 0) | .key ] end),
        longest_chain: (if $pkg_acyclic then $K.longest_chain else null end)
    },
    plan_comparison: $plan_cmp,
    packages: $packages_out
}
' > "$GEN_JSON"

# Render deterministic Markdown table (six-column package table + part table + cycles + unpackaged)
jq -r '
def esc_md: gsub("\\|"; "\\\\|") | gsub("\n"; " ");
def fmt_list($arr): if ($arr | length) == 0 then "—" else ($arr | join(", ")) end;
def fmt_wit($w): if ($w | type) == "string" then $w else ($w[0] + "→" + $w[1]) end;

"# Work Packages (Generated from Bead Graph)\n\n" +
"Generated by `scripts/wp-table.sh` from the bead graph; do not edit by hand.\n\n" +
"## Summary\n\n" +
"- Packages: " + (.counts.packages | tostring) +
" | Packaged beads: " + (.counts.packaged_beads | tostring) +
" | Unpackaged beads: " + (.counts.unpackaged_beads | tostring) + "\n" +
"- Parts: " + (.counts.parts | tostring) +
" | Part edges (raw / reduced / removed): " +
(.counts.part_edges_raw | tostring) + " / " +
(.counts.part_edges_reduced | tostring) + " / " +
(.counts.part_edges_removed | tostring) +
" | Part cycles: " + (.counts.part_cycles | tostring) + "\n" +
"- Package view: " + (if .package_view.acyclic then "acyclic (reduced)" else "cyclic (unreduced; cycles marked)" end) +
" | Edges: " + (.counts.package_view_edges_output | tostring) +
" | Package cycles: " + (.counts.package_view_cycles | tostring) + "\n" +
"- Package roots: " + fmt_list(.package_view.roots) +
(if .package_view.longest_chain != null then
    " | Longest chain: " + (.package_view.longest_chain.edges | tostring) +
    " edges ending at " + (.package_view.longest_chain.end // "—")
else "" end) + "\n\n" +

"## Package Table\n\n" +
"| Package | Name | Crates | Milestones | Key gates | Depends on |\n" +
"|---|---|---|---|---|---|\n" +
(
    .packages | map(
        "| " + .key + (if .in_package_cycle then " [cycle]" else "" end) +
        " | " + (.title | esc_md) +
        " | " + (fmt_list(.crates) | esc_md) +
        " | " + fmt_list(.milestones) +
        " | " + fmt_list(.key_gates) +
        " | " + fmt_list(.depends_on) + " |"
    ) | join("\n")
) + "\n\n" +

"## Part Table\n\n" +
(
    (.parts.edges | group_by(.from) | map({key: .[0].from, value: .}) | from_entries) as $pe |
    "| Part node | Depends on (reduced, with witness) |\n" +
    "|---|---|\n" +
    (
        .parts.condensed_nodes | map(
            . as $cn |
            ($pe[$cn] // []) as $es |
            "| " + $cn + " | " +
            (if ($es | length) == 0 then "—"
             else ($es | map(.to + " (" + fmt_wit(.witness) + ")") | join(", "))
             end) + " |"
        ) | join("\n")
    )
) + "\n\n" +

"## Part Cycles\n\n" +
(
    if (.parts.cycles | length) == 0 then
        "None.\n\n"
    else
        (
            .parts.cycles | map(
                "### " + .id + "\n\n" +
                "- Members (" + (.members | length | tostring) + "): " + (.members | join(", ")) + "\n" +
                "- Internal edges (" + (.edges | length | tostring) + "): " +
                (.edges | map(.from + "→" + .to + " (" + fmt_wit(.witness) + ")") | join("; ")) + "\n"
            ) | join("\n")
        ) + "\n"
    end
) +

"## Package Cycles\n\n" +
(
    if (.package_view.cycles | length) == 0 then
        "None.\n\n"
    else
        (
            .package_view.cycles | map(
                "### " + .id + "\n\n" +
                "- Members (" + (.members | length | tostring) + "): " + (.members | join(", ")) + "\n" +
                "- Internal edges (" + (.edges | length | tostring) + "): " +
                (.edges | map(.from + "→" + .to + " (" + fmt_wit(.witness) + ")") | join("; ")) + "\n"
            ) | join("\n")
        ) + "\n"
    end
) +

"## Unpackaged Beads\n\n" +
(
    if (.unpackaged | length) == 0 then
        "None.\n"
    else
        "Count: " + (.unpackaged | length | tostring) + "\n\n" +
        (.unpackaged | map("- " + .key + " (" + .milestone + ", " + .top_epic + ", " + .status + ")") | join("\n")) + "\n"
    end
)
' "$GEN_JSON" > "$GEN_MD"

# Print schema fields discovered on this run
SCHEMA_LINE=$(jq -r '
    .schema_fields |
    "schema-fields: id=" + .id + " key=" + .key + " type=" + .type +
    " status=" + .status + " labels=" + .labels + " deps=" + .dependencies +
    " dep_target=" + .dep_target + " dep_type=" + .dep_type
' "$EXTRACT_JSON")

if [ "$MODE" = "generate" ]; then
    mkdir -p "$OUT_DIR"
    cp "$GEN_JSON" "$OUT_DIR/work-packages.json"
    cp "$GEN_MD" "$OUT_DIR/work-packages.md"
    printf '%s\n' "$SCHEMA_LINE"
    jq -r '
        "generated: packages=" + (.counts.packages | tostring) +
        " parts=" + (.counts.parts | tostring) +
        " part_edges_reduced=" + (.counts.part_edges_reduced | tostring) +
        " part_cycles=" + (.counts.part_cycles | tostring) +
        " pkg_view_acyclic=" + (.package_view.acyclic | tostring) +
        " pkg_view_edges=" + (.counts.package_view_edges_output | tostring) +
        " unpackaged=" + (.counts.unpackaged_beads | tostring)
    ' "$GEN_JSON"
    exit 0
fi

# ==============================================================================
# Mode: check
# ==============================================================================
FAIL=0
printf '%s\n' "$SCHEMA_LINE"

compare_file() {
    committed="$1"
    fresh="$2"
    label="$3"
    if [ ! -f "$committed" ]; then
        printf 'stale: %s is missing\n' "$label"
        FAIL=1
        return
    fi
    if ! cmp -s "$committed" "$fresh"; then
        diff_info=$(WP_CMP_FRESH="$fresh" awk '
            BEGIN { f2 = ENVIRON["WP_CMP_FRESH"]; l = 0; found = 0 }
            {
                l++
                if ((getline other < f2) <= 0) {
                    printf("line %d: committed has extra line: %s", l, $0)
                    found = 1
                    exit 0
                }
                if ($0 != other) {
                    printf("line %d: committed=%s | generated=%s", l, $0, other)
                    found = 1
                    exit 0
                }
            }
            END {
                if (!found) {
                    if ((getline other < f2) > 0) {
                        printf("line %d: generated has extra line: %s", l + 1, other)
                    } else {
                        printf("byte difference (trailing newline or encoding)")
                    }
                }
            }
        ' < "$committed")
        printf 'stale: %s differs at %s\n' "$label" "$diff_info"
        FAIL=1
    fi
}

compare_file "$OUT_DIR/work-packages.json" "$GEN_JSON" "$OUT_DIR/work-packages.json"
compare_file "$OUT_DIR/work-packages.md" "$GEN_MD" "$OUT_DIR/work-packages.md"

# Failing check: related-missing
REL_MISSING=$(jq -r '.plan_comparison.related_missing[] | "related-missing: " + .from + " " + .to' "$GEN_JSON")
if [ -n "$REL_MISSING" ]; then
    printf '%s\n' "$REL_MISSING"
    FAIL=1
fi

# Failing check (only with --strict-acyclic): part cycles surviving the split
PART_CYCLES_COUNT=$(jq -r '.counts.part_cycles' "$GEN_JSON")
if [ "$PART_CYCLES_COUNT" -gt 0 ]; then
    jq -r '
        def fmt_wit($w): if ($w | type) == "string" then $w else ($w[0] + "->" + $w[1]) end;
        .parts.cycles[] |
        "part-cycle: " + .id + " members=[" + (.members | join(",")) + "] witnesses=[" +
        (.edges | map(.from + "->" + .to + ":" + fmt_wit(.witness)) | join(";")) + "]"
    ' "$GEN_JSON"
    if [ "$STRICT_ACYCLIC" -eq 1 ]; then
        printf 'strict-acyclic: %s part cycle(s) survived the split into parts\n' "$PART_CYCLES_COUNT"
        FAIL=1
    fi
fi

# Non-failing reports: plan diff, related-extra, related vs regenerated, milestone mismatches
jq -r '
    def fmt_wit($w): if ($w | type) == "string" then $w else ($w[0] + "->" + $w[1]) end;
    (.plan_comparison.added[] | "plan-diff-added: " + .from + " -> " + .to + " (" + fmt_wit(.witness) + ")"),
    (.plan_comparison.removed[] | "plan-diff-removed: " + .from + " -> " + .to),
    (.plan_comparison.related_extra[] | "related-extra: " + .[0] + " " + .[1]),
    (.plan_comparison.regenerated_not_in_related[] | "regen-not-in-related: " + .from + " -> " + .to + " (" + fmt_wit(.witness) + ")"),
    (.plan_comparison.related_not_in_regenerated[] | "related-not-in-regen: " + .[0] + " " + .[1]),
    (.plan_comparison.milestone_mismatches[] |
        "milestone-mismatch: " + .package + " epic=" + .epic_milestone +
        " first_child=" + .first_child_milestone + " parts=[" + (.child_milestones | join(",")) + "]"),
    ("report-summary: packages=" + (.counts.packages | tostring) +
     " parts=" + (.counts.parts | tostring) +
     " part_cycles=" + (.counts.part_cycles | tostring) +
     " pkg_cycles=" + (.counts.package_view_cycles | tostring) +
     " unpackaged=" + (.counts.unpackaged_beads | tostring) +
     " plan_added=" + (.plan_comparison.added | length | tostring) +
     " plan_removed=" + (.plan_comparison.removed | length | tostring) +
     " related_missing=" + (.plan_comparison.related_missing | length | tostring) +
     " related_extra=" + (.plan_comparison.related_extra | length | tostring) +
     " regen_not_in_related=" + (.plan_comparison.regenerated_not_in_related | length | tostring) +
     " related_not_in_regen=" + (.plan_comparison.related_not_in_regenerated | length | tostring) +
     " milestone_mismatches=" + (.plan_comparison.milestone_mismatches | length | tostring))
' "$GEN_JSON"

if [ "$FAIL" -ne 0 ]; then
    exit 1
fi
exit 0
