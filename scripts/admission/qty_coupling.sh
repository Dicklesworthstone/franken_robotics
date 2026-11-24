#!/usr/bin/env bash
# Measure the fs-qty coupling graph of FrankenSim at a pinned git revision.
#
# Usage:
#   scripts/admission/qty_coupling.sh [--rev <40-hex sha>] [--repo <path>]
#                                     [--stack-lock <path>] [--out-dir <path>]
#                                     [--working-tree]
#
# Exit codes:
#   0  Measurement succeeded; canonical frob.qty-coupling/1 record written.
#   1  Cargo metadata resolution failed (fail-closed); failure record written
#      with exact cargo stderr and no invented counts.
#   2  Repository/revision error or sibling-immutability invariant violated.
#   3  Usage error or missing/invalid revision with no usable FD-FSIM lock row
#      (writes nothing).
#
# Invariants:
#   - Sibling repositories are strictly READ-ONLY (INV-SUITE-04).
#   - Uses `cargo metadata --format-version 1 --locked --offline` via PATH shim
#     (never --no-deps, never mutating Cargo.lock, never touching network).
#   - Never deletes any file or directory (scratch exports use mkdtemp under
#     ${BIG_SCRATCH:-${TMPDIR:-/tmp}}/frob-<actor>/ and are left intact).

set -euo pipefail

exec python3 - "$@" <<'PY'
import datetime
import io
import json
import os
import re
import shutil
import stat
import struct
import subprocess
import sys
import tempfile
import tomllib
import unicodedata
from pathlib import Path

SCHEMA = "frob.qty-coupling/1"
TARGET_CRATE = "fs-qty"
CUT_SOURCE_CRATE = "fs-evidence"
FREED_SCOPE = "freed in FrankenSim's graph"
REPORTED = {
    "coupled": 22,
    "direct": 12,
    "freed": 8,
    "indirect": 10,
}

BEDROCK_DIRECT = (
    "fs-ga",
    "fs-math",
    "fs-qty",
    "fs-rand",
    "fs-blake3",
    "fs-ivl",
    "fs-simd",
    "fs-exec",
    "fs-evidence",
    "fs-ad",
)

AIRLOCK_GROUPS = {
    "referee": (
        "fs-mbd",
        "fs-mbd-material",
        "fs-contact",
        "fs-tribo",
        "fs-time",
        "fs-motion",
        "fs-ports",
        "fs-system",
    ),
    "geometry": (
        "fs-query",
        "fs-geom",
        "fs-rep-sdf",
        "fs-rep-mesh",
        "fs-rep-frep",
        "fs-scene",
        "fs-topo",
        "fs-io",
    ),
    "ledger": (
        "fs-ledger",
        "fs-session",
        "fs-package",
        "fs-recompute",
    ),
    "optimize": (
        "fs-dfo",
        "fs-bo",
        "fs-ascent",
        "fs-archive",
        "fs-opt",
        "fs-sos",
        "fs-race",
        "fs-eproc",
        "fs-tropical",
        "fs-symmetry",
    ),
    "stats": (
        "fs-uq",
        "fs-voi",
        "fs-surrogate",
        "fs-assimilate",
        "fs-robust",
        "fs-regime",
    ),
    "spectral": (
        "fs-spectral",
    ),
    "actuation": (
        "fs-em",
        "fs-thermal",
        "fs-control",
        "fs-phs",
        "fs-matdb",
    ),
    "structure": (
        "fs-solid",
        "fs-topopt",
        "fs-topols",
        "fs-toleralloc",
        "fs-asbuilt",
        "fs-geocon",
        "fs-fab",
        "fs-adjoint",
    ),
    "multiphysics": (
        "fs-lbm",
        "fs-couple",
    ),
    "harness": (
        "fs-roofline",
        "fs-detaudit",
        "fs-propcheck",
        "fs-render",
        "fs-la",
    ),
}

M1_GROUP_NAMES = ("referee", "geometry", "ledger")
HEX40_RE = re.compile(r"^[0-9a-f]{40}$")
SMALL_REPO_FILE_THRESHOLD = 2000


def usage(msg: str | None = None) -> None:
    if msg:
        print(f"qty_coupling.sh: {msg}", file=sys.stderr)
    print(
        "Usage: scripts/admission/qty_coupling.sh "
        "[--rev <40-hex sha>] [--repo <path>] [--stack-lock <path>] "
        "[--out-dir <path>] [--working-tree]",
        file=sys.stderr,
    )


def parse_args(argv: list[str]) -> dict:
    opts = {
        "rev": None,
        "repo": None,
        "stack_lock": None,
        "out_dir": "receipts/coupling",
        "working_tree": False,
    }
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg in ("-h", "--help"):
            usage()
            sys.exit(0)
        elif arg in ("--rev", "--repo", "--stack-lock", "--out-dir"):
            if i + 1 >= len(argv):
                usage(f"missing value for {arg}")
                sys.exit(3)
            key = arg[2:].replace("-", "_")
            opts[key] = argv[i + 1]
            i += 2
        elif arg == "--working-tree":
            opts["working_tree"] = True
            i += 1
        else:
            usage(f"unknown argument: {arg}")
            sys.exit(3)
    return opts


def load_fd_fsim_row(lock_path: Path) -> tuple[dict | None, str | None]:
    if not lock_path.is_file():
        return None, f"stack lock not found: {lock_path}"
    try:
        data = tomllib.loads(lock_path.read_text(encoding="utf-8"))
    except Exception as exc:
        return None, f"failed to parse stack lock {lock_path}: {exc}"
    foundations = data.get("foundation")
    if not isinstance(foundations, list):
        return None, f"stack lock {lock_path} has no [[foundation]] table"
    for row in foundations:
        if isinstance(row, dict) and row.get("id") == "FD-FSIM":
            return row, None
    return None, f"stack lock {lock_path} has no FD-FSIM foundation row"


def resolve_inputs(opts: dict) -> tuple[Path, str, str]:
    lock_path = Path(opts["stack_lock"]) if opts["stack_lock"] is not None else Path("franken-stack.lock")
    lock_row, lock_err = load_fd_fsim_row(lock_path)

    if opts["rev"] is not None:
        rev = opts["rev"]
        if not HEX40_RE.match(rev):
            usage(f"--rev must be a 40-hex lowercase git SHA, got: {rev!r}")
            sys.exit(3)
        rev_source = "argument"
    else:
        if lock_row is None:
            usage(f"no --rev given and {lock_err}")
            sys.exit(3)
        git_rev = lock_row.get("git_rev")
        if not isinstance(git_rev, str) or not HEX40_RE.match(git_rev):
            usage(
                f"no --rev given and FD-FSIM row in {lock_path} has invalid git_rev: {git_rev!r} "
                "(expected 40-hex SHA)"
            )
            sys.exit(3)
        rev = git_rev
        rev_source = "stack-lock"

    if opts["repo"] is not None:
        repo = Path(opts["repo"])
    elif lock_row is not None and isinstance(lock_row.get("path"), str) and lock_row["path"].strip():
        raw_p = Path(lock_row["path"])
        repo = raw_p if raw_p.is_absolute() else (lock_path.parent / raw_p)
    else:
        repo = Path("../frankensim")

    return repo, rev, rev_source


def git_capture(repo: Path, args: list[str], *, text: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(
        ["git", "--no-optional-locks", "-C", str(repo), *args],
        capture_output=True,
        text=text,
    )


def snapshot_repo_state(repo: Path) -> tuple[str, bytes]:
    head_proc = git_capture(repo, ["rev-parse", "HEAD"], text=True)
    if head_proc.returncode != 0:
        print(
            f"qty_coupling.sh: failed to read git HEAD of {repo}: {head_proc.stderr.strip()}",
            file=sys.stderr,
        )
        sys.exit(2)
    stat_proc = git_capture(repo, ["status", "--porcelain"], text=False)
    if stat_proc.returncode != 0:
        print(
            f"qty_coupling.sh: failed to read git status of {repo}: "
            f"{stat_proc.stderr.decode('utf-8', errors='replace').strip()}",
            file=sys.stderr,
        )
        sys.exit(2)
    return head_proc.stdout.strip(), stat_proc.stdout


def blake3_bytes(payload: bytes) -> str:
    proc = subprocess.run(
        ["b3sum", "--no-names"],
        input=payload,
        capture_output=True,
        check=True,
    )
    return proc.stdout.decode("ascii").strip()


def blake3_file(path: Path) -> str:
    proc = subprocess.run(
        ["b3sum", "--no-names", str(path)],
        capture_output=True,
        check=True,
    )
    return proc.stdout.decode("ascii").strip()


def compute_tree_hash(repo: Path) -> str:
    """Compute the frob.tree/1 BLAKE3 hash of tracked working-tree bytes (N33a-04 / BD-001)."""
    ls_proc = git_capture(repo, ["ls-files", "-z"], text=False)
    if ls_proc.returncode != 0:
        raise RuntimeError(f"git ls-files -z failed: {ls_proc.stderr!r}")
    raw_paths = sorted(set(p for p in ls_proc.stdout.split(b"\x00") if p))

    proc = subprocess.Popen(
        ["b3sum", "--no-names"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    assert proc.stdin is not None
    try:
        proc.stdin.write(b"frob.tree/1")
        for rel_b in raw_paths:
            proc.stdin.write(struct.pack("<Q", len(rel_b)))
            proc.stdin.write(rel_b)
            rel_s = os.fsdecode(rel_b)
            full = repo / rel_s
            try:
                st = os.lstat(full)
            except FileNotFoundError:
                proc.stdin.write(b"\xff")
                continue
            if stat.S_ISLNK(st.st_mode):
                target_b = os.fsencode(os.readlink(full))
                proc.stdin.write(b"\x00")
                proc.stdin.write(struct.pack("<Q", len(target_b)))
                proc.stdin.write(target_b)
            elif stat.S_ISDIR(st.st_mode):
                stage = git_capture(repo, ["ls-files", "--stage", "-z", "--", rel_s], text=False)
                oid = b""
                if stage.returncode == 0 and stage.stdout:
                    first = stage.stdout.split(b"\x00")[0].split(b"\t")[0].split(b" ")
                    if len(first) >= 2:
                        oid = first[1]
                proc.stdin.write(b"\x00")
                proc.stdin.write(struct.pack("<Q", len(oid)))
                proc.stdin.write(oid)
            else:
                exec_byte = b"\x01" if (st.st_mode & 0o111) else b"\x00"
                content = full.read_bytes()
                proc.stdin.write(exec_byte)
                proc.stdin.write(struct.pack("<Q", len(content)))
                proc.stdin.write(content)
    finally:
        proc.stdin.close()
    stdout, stderr = proc.communicate()
    if proc.returncode != 0:
        raise RuntimeError(f"b3sum failed: {stderr.decode('utf-8', errors='replace')}")
    return stdout.decode("ascii").strip()


def make_scratch_dir(utc_tag: str) -> Path:
    base_root = Path(os.environ.get("BIG_SCRATCH") or os.environ.get("TMPDIR") or "/tmp")
    actor = os.environ.get("FROB_SCRATCH_ACTOR", "CopperDove")
    parent = base_root / f"frob-{actor}"
    parent.mkdir(parents=True, exist_ok=True)
    return Path(tempfile.mkdtemp(prefix=f"qty-coupling-{utc_tag}-", dir=str(parent)))


def is_auto_target_rel(rel_under_crate: str) -> bool:
    if rel_under_crate in ("src/lib.rs", "src/main.rs", "build.rs"):
        return True
    parts = rel_under_crate.split("/")
    if len(parts) == 3 and parts[0] == "src" and parts[1] == "bin" and parts[2].endswith(".rs"):
        return True
    if len(parts) == 4 and parts[0] == "src" and parts[1] == "bin" and parts[3] == "main.rs":
        return True
    for group in ("tests", "examples", "benches"):
        if len(parts) == 2 and parts[0] == group and parts[1].endswith(".rs"):
            return True
        if len(parts) == 3 and parts[0] == group and parts[2] == "main.rs":
            return True
    return False


def explicit_manifest_paths(crate_dir: str, manifest_bytes: bytes) -> set[str]:
    result: set[str] = set()
    try:
        parsed = tomllib.loads(manifest_bytes.decode("utf-8"))
    except Exception:
        return result
    prefix = f"{crate_dir}/" if crate_dir else ""

    def add_rel(val: object) -> None:
        if isinstance(val, str) and val.strip():
            norm = os.path.normpath(prefix + val)
            if not norm.startswith(".."):
                result.add(norm)

    pkg = parsed.get("package")
    if isinstance(pkg, dict):
        build_val = pkg.get("build")
        if isinstance(build_val, str):
            add_rel(build_val)
        add_rel(pkg.get("readme"))
        add_rel(pkg.get("license-file"))

    lib = parsed.get("lib")
    if isinstance(lib, dict):
        add_rel(lib.get("path"))

    for key in ("bin", "test", "example", "bench"):
        entries = parsed.get(key)
        if isinstance(entries, list):
            for entry in entries:
                if isinstance(entry, dict):
                    add_rel(entry.get("path"))
    return result


def cat_file_batch_extract(repo: Path, rev: str, paths: list[str], export_dir: Path) -> None:
    if not paths:
        return
    query = "".join(f"{rev}:{p}\n" for p in paths).encode("utf-8")
    proc = subprocess.run(
        ["git", "--no-optional-locks", "-C", str(repo), "cat-file", "--batch"],
        input=query,
        capture_output=True,
    )
    if proc.returncode != 0:
        err = proc.stderr.decode("utf-8", errors="replace")
        raise RuntimeError(f"git cat-file --batch failed (exit {proc.returncode}): {err}")

    stream = io.BytesIO(proc.stdout)
    for rel in paths:
        hdr = stream.readline().decode("utf-8").strip()
        parts = hdr.split()
        if len(parts) < 3 or parts[1] != "blob":
            raise RuntimeError(f"unexpected git cat-file header for {rev}:{rel}: {hdr!r}")
        size = int(parts[2])
        body = stream.read(size)
        stream.read(1)  # trailing LF
        dst = export_dir / rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        dst.write_bytes(body)


def export_commit_readonly(repo: Path, rev: str, export_dir: Path) -> None:
    ls_proc = git_capture(repo, ["ls-tree", "-r", "--name-only", rev], text=True)
    if ls_proc.returncode != 0:
        print(
            f"qty_coupling.sh: failed to list tree at {rev}: {ls_proc.stderr.strip()}",
            file=sys.stderr,
        )
        sys.exit(2)
    all_paths = [p for p in ls_proc.stdout.splitlines() if p]
    all_path_set = set(all_paths)

    if len(all_paths) <= SMALL_REPO_FILE_THRESHOLD:
        p1 = subprocess.Popen(
            ["git", "--no-optional-locks", "-C", str(repo), "archive", rev],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        p2 = subprocess.Popen(
            ["tar", "-x", "-C", str(export_dir)],
            stdin=p1.stdout,
            stderr=subprocess.PIPE,
        )
        assert p1.stdout is not None
        p1.stdout.close()
        _, err2 = p2.communicate()
        _, err1 = p1.communicate()
        if p1.returncode != 0 or p2.returncode != 0:
            raise RuntimeError(
                f"git archive/tar failed: git={p1.returncode} ({err1.decode(errors='replace')}) "
                f"tar={p2.returncode} ({err2.decode(errors='replace')})"
            )
    else:
        # Large workspace: extract every Cargo manifest, lockfile, .cargo config,
        # toolchain file, root metadata file, and every Cargo target entrypoint
        # (both auto-discovered and explicitly declared in any Cargo.toml).
        stage1 = sorted(
            p
            for p in all_paths
            if p == "Cargo.toml"
            or p.endswith("/Cargo.toml")
            or p in ("Cargo.lock", "rust-toolchain.toml", "rust-toolchain")
            or p.startswith(".cargo/")
            or ("/" not in p and p in ("LICENSE", "README.md"))
        )
        cat_file_batch_extract(repo, rev, stage1, export_dir)

        crate_dirs = set()
        explicit_needed: set[str] = set()
        for p in stage1:
            if p == "Cargo.toml" or p.endswith("/Cargo.toml"):
                cdir = "" if p == "Cargo.toml" else p[: -len("/Cargo.toml")]
                crate_dirs.add(cdir)
                explicit_needed.update(
                    explicit_manifest_paths(cdir, (export_dir / p).read_bytes())
                )

        stage2_set: set[str] = set()
        for cand in explicit_needed:
            if cand in all_path_set and cand not in stage1:
                stage2_set.add(cand)

        for p in all_paths:
            if p in stage1 or p in stage2_set:
                continue
            for cdir in crate_dirs:
                if cdir == "":
                    if is_auto_target_rel(p):
                        stage2_set.add(p)
                        break
                elif p.startswith(cdir + "/"):
                    sub = p[len(cdir) + 1 :]
                    if is_auto_target_rel(sub):
                        stage2_set.add(p)
                        break

        cat_file_batch_extract(repo, rev, sorted(stage2_set), export_dir)

    make_tree_readonly(export_dir)


def export_working_tree_readonly(repo: Path, export_dir: Path) -> None:
    """Copy manifests, lockfile, config, and Cargo targets from working tree read-only."""
    manifests: list[Path] = []
    for root_name in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "rust-toolchain", "LICENSE", "README.md"):
        src = repo / root_name
        if src.is_file():
            dst = export_dir / root_name
            dst.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src, dst)
            if root_name == "Cargo.toml":
                manifests.append(Path("Cargo.toml"))

    cargo_dir = repo / ".cargo"
    if cargo_dir.is_dir():
        for entry in cargo_dir.rglob("*"):
            if entry.is_file():
                rel = entry.relative_to(repo)
                dst = export_dir / rel
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(entry, dst)

    # Walk workspace directories without descending into target/ or .git/
    skip_dirs = {".git", "target", "node_modules", ".beads", "data", "assets", "docs"}
    for cur_root, dirs, files in os.walk(repo):
        dirs[:] = [d for d in dirs if d not in skip_dirs]
        cur_p = Path(cur_root)
        if "Cargo.toml" in files and cur_p != repo:
            rel_dir = cur_p.relative_to(repo)
            rel_manifest = rel_dir / "Cargo.toml"
            dst = export_dir / rel_manifest
            dst.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(cur_p / "Cargo.toml", dst)
            manifests.append(rel_manifest)

    for rel_manifest in manifests:
        cdir_rel = "" if rel_manifest == Path("Cargo.toml") else str(rel_manifest.parent)
        cdir_abs = repo / cdir_rel if cdir_rel else repo
        needed = explicit_manifest_paths(cdir_rel, (export_dir / rel_manifest).read_bytes())

        for std_rel in ("src/lib.rs", "src/main.rs", "build.rs"):
            cand = cdir_abs / std_rel
            if cand.is_file():
                needed.add(str((Path(cdir_rel) / std_rel) if cdir_rel else Path(std_rel)))

        for group in ("src/bin", "tests", "examples", "benches"):
            gdir = cdir_abs / group
            if gdir.is_dir():
                for child in gdir.iterdir():
                    if child.is_file() and child.suffix == ".rs":
                        needed.add(str(child.relative_to(repo)))
                    elif child.is_dir() and (child / "main.rs").is_file():
                        needed.add(str((child / "main.rs").relative_to(repo)))

        for rel_s in sorted(needed):
            src = repo / rel_s
            dst = export_dir / rel_s
            if src.is_file() and not dst.exists():
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(src, dst)

    make_tree_readonly(export_dir)


def make_tree_readonly(root: Path) -> None:
    dirs: list[Path] = []
    for cur_root, subdirs, files in os.walk(root):
        cur_p = Path(cur_root)
        dirs.append(cur_p)
        for fname in files:
            fpath = cur_p / fname
            if not fpath.is_symlink():
                os.chmod(fpath, 0o444)
    for dpath in reversed(dirs):
        os.chmod(dpath, 0o555)


def symlink_siblings(repo_real: Path, stack_dir: Path, scratch_dir: Path) -> None:
    parent = repo_real.parent
    scratch_real = scratch_dir.resolve()
    for entry in parent.iterdir():
        if entry.name == repo_real.name:
            continue
        if not entry.is_dir():
            continue
        try:
            entry_real = entry.resolve()
        except OSError:
            continue
        # Avoid recursive symlink cycles if scratch lives inside parent.
        if entry_real == scratch_real or scratch_real.is_relative_to(entry_real):
            continue
        os.symlink(entry, stack_dir / entry.name)


def build_normal_workspace_graph(meta: dict) -> tuple[set[str], dict[str, set[str]]]:
    ws_ids = set(meta.get("workspace_members", []))
    id_to_name: dict[str, str] = {}
    for pkg in meta.get("packages", []):
        pid = pkg["id"]
        if pid in ws_ids:
            id_to_name[pid] = pkg["name"]

    ws_names = set(id_to_name.values())
    adj: dict[str, set[str]] = {name: set() for name in ws_names}

    resolve = meta.get("resolve")
    if not isinstance(resolve, dict) or not isinstance(resolve.get("nodes"), list):
        raise RuntimeError("cargo metadata output missing resolve.nodes graph")

    for node in resolve["nodes"]:
        src_id = node.get("id")
        if src_id not in id_to_name:
            continue
        src_name = id_to_name[src_id]
        for dep in node.get("deps", []):
            dst_id = dep.get("pkg")
            if dst_id not in id_to_name:
                continue
            dep_kinds = dep.get("dep_kinds", [])
            # Normal dependency iff at least one dep_kind has kind == None (null in JSON).
            is_normal = any(dk.get("kind") is None for dk in dep_kinds)
            if is_normal:
                dst_name = id_to_name[dst_id]
                if dst_name != src_name:
                    adj[src_name].add(dst_name)

    return ws_names, adj


def compute_reachable(adj: dict[str, set[str]], start: str) -> set[str]:
    """All nodes reachable from `start` via >= 1 directed edges."""
    visited: set[str] = set()
    stack = list(adj.get(start, ()))
    while stack:
        cur = stack.pop()
        if cur in visited:
            continue
        visited.add(cur)
        for nxt in adj.get(cur, ()):
            if nxt not in visited:
                stack.append(nxt)
    return visited


def compute_reflexive_closure(adj: dict[str, set[str]], seeds: set[str]) -> set[str]:
    visited = set(seeds)
    stack = list(seeds)
    while stack:
        cur = stack.pop()
        for nxt in adj.get(cur, ()):
            if nxt not in visited:
                visited.add(nxt)
                stack.append(nxt)
    return visited


def analyze_population(
    name: str,
    pop: set[str],
    adj: dict[str, set[str]],
    reachable_map: dict[str, set[str]],
    reachable_cut_map: dict[str, set[str]],
) -> dict:
    direct = sorted(u for u in pop if TARGET_CRATE in adj.get(u, ()))
    direct_set = set(direct)
    indirect_names = sorted(
        u for u in pop if u not in direct_set and TARGET_CRATE in reachable_map.get(u, ())
    )

    indirect_rows = []
    for u in indirect_names:
        carriers = sorted(
            v
            for v in adj.get(u, ())
            if v != TARGET_CRATE and TARGET_CRATE in reachable_map.get(v, ())
        )
        if not carriers:
            raise AssertionError(f"indirect crate {u} in {name} has no carriers")
        indirect_rows.append({"carriers": carriers, "crate": u})

    coupled = sorted(direct_set | set(indirect_names))
    still_coupled = sorted(
        u for u in coupled if TARGET_CRATE in reachable_cut_map.get(u, ())
    )
    still_set = set(still_coupled)
    freed = sorted(u for u in coupled if u not in still_set)

    # Self-verify all mathematical invariants from Acceptance Criterion #2.
    assert len(coupled) == len(direct) + len(indirect_rows)
    assert set(freed).issubset(set(coupled))
    assert set(freed).isdisjoint(still_set)
    assert (set(freed) | still_set) == set(coupled)

    return {
        "coupled_count": len(coupled),
        "direct": direct,
        "direct_count": len(direct),
        "freed_by_fd_fsim_31": freed,
        "freed_count": len(freed),
        "freed_scope": FREED_SCOPE,
        "indirect": indirect_rows,
        "indirect_count": len(indirect_rows),
        "name": name,
        "size": len(pop),
        "still_coupled_after_31": still_coupled,
    }


def analyze_mode(feature_mode: str, meta: dict) -> dict:
    ws_names, adj = build_normal_workspace_graph(meta)

    reachable_map = {u: compute_reachable(adj, u) for u in ws_names}

    adj_cut = {u: set(vs) for u, vs in adj.items()}
    if CUT_SOURCE_CRATE in adj_cut:
        adj_cut[CUT_SOURCE_CRATE].discard(TARGET_CRATE)
    reachable_cut_map = {u: compute_reachable(adj_cut, u) for u in ws_names}

    p1_universe = set(BEDROCK_DIRECT)
    for grp_crates in AIRLOCK_GROUPS.values():
        p1_universe.update(grp_crates)
    pop_p1 = p1_universe & ws_names

    m1_seeds: set[str] = set()
    for gname in M1_GROUP_NAMES:
        m1_seeds.update(AIRLOCK_GROUPS[gname])
    pop_p2 = compute_reflexive_closure(adj, m1_seeds & ws_names)

    pop_p3 = set(ws_names)

    populations = [
        analyze_population("P1", pop_p1, adj, reachable_map, reachable_cut_map),
        analyze_population("P2", pop_p2, adj, reachable_map, reachable_cut_map),
        analyze_population("P3", pop_p3, adj, reachable_map, reachable_cut_map),
    ]

    reproduced_by = "none"
    for p in populations:
        if (
            p["coupled_count"] == REPORTED["coupled"]
            and p["direct_count"] == REPORTED["direct"]
            and p["indirect_count"] == REPORTED["indirect"]
            and p["freed_count"] == REPORTED["freed"]
        ):
            reproduced_by = p["name"]
            break

    return {
        "feature_mode": feature_mode,
        "populations": populations,
        "reproduced_by": reproduced_by,
    }


def normalize_nfc(obj):
    if isinstance(obj, str):
        return unicodedata.normalize("NFC", obj)
    if isinstance(obj, list):
        return [normalize_nfc(x) for x in obj]
    if isinstance(obj, dict):
        return {unicodedata.normalize("NFC", k): normalize_nfc(v) for k, v in obj.items()}
    return obj


def canonical_json_bytes(record: dict) -> bytes:
    normalized = normalize_nfc(record)
    text = json.dumps(normalized, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return text.encode("utf-8")


def choose_output_path(out_dir: Path, utc_tag: str) -> Path:
    out_dir.mkdir(parents=True, exist_ok=True)
    candidate = out_dir / f"{utc_tag}.json"
    if not candidate.exists():
        return candidate
    suffix = 1
    while True:
        candidate = out_dir / f"{utc_tag}-{suffix}.json"
        if not candidate.exists():
            return candidate
        suffix += 1


def sanitize_scratch_text(text: str, scratch_dir: Path) -> str:
    s = text.replace(str(scratch_dir.resolve()), "<scratch>")
    s = s.replace(str(scratch_dir), "<scratch>")
    return s


def main() -> int:
    opts = parse_args(sys.argv[1:])
    repo_arg, rev, rev_source = resolve_inputs(opts)

    if not repo_arg.is_dir():
        print(f"qty_coupling.sh: repository directory does not exist: {repo_arg}", file=sys.stderr)
        return 2
    repo_real = repo_arg.resolve()

    # Check BEFORE state of sibling checkout (INV-SUITE-04).
    before_head, before_status = snapshot_repo_state(repo_real)

    # Confirm the requested revision is a valid commit in the repository.
    verify_proc = git_capture(repo_real, ["rev-parse", "--verify", f"{rev}^{{commit}}"], text=True)
    if verify_proc.returncode != 0 or verify_proc.stdout.strip() != rev:
        print(
            f"qty_coupling.sh: revision {rev} is not a commit in {repo_arg}",
            file=sys.stderr,
        )
        return 2

    utc_tag = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    scratch_dir = make_scratch_dir(utc_tag)
    stack_dir = scratch_dir / "stack"
    export_dir = stack_dir / repo_real.name
    export_dir.mkdir(parents=True, exist_ok=True)

    working_tree = opts["working_tree"]
    dirty = False
    tree_hash: str | None = None
    if working_tree:
        if rev != before_head:
            print(
                f"qty_coupling.sh: --working-tree requires requested rev ({rev}) to match "
                f"checkout HEAD ({before_head})",
                file=sys.stderr,
            )
            return 2
        tracked_stat = git_capture(repo_real, ["status", "--porcelain", "--untracked-files=no"], text=False)
        if tracked_stat.returncode != 0:
            print(
                f"qty_coupling.sh: failed to check tracked dirty state of {repo_arg}",
                file=sys.stderr,
            )
            return 2
        dirty = len(tracked_stat.stdout.strip()) > 0
        if dirty:
            tree_hash = compute_tree_hash(repo_real)
        export_working_tree_readonly(repo_real, export_dir)
    else:
        export_commit_readonly(repo_real, rev, export_dir)

    symlink_siblings(repo_real, stack_dir, scratch_dir)

    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(scratch_dir / "target")
    manifest_path = export_dir / "Cargo.toml"

    modes_out = []
    cargo_errors = []

    for feature_mode, extra_flags in (("default", []), ("all", ["--all-features"])):
        argv = [
            "cargo",
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--offline",
            *extra_flags,
            "--manifest-path",
            str(manifest_path),
        ]
        proc = subprocess.run(argv, capture_output=True, text=True, env=env)
        if proc.returncode != 0:
            cargo_errors.append(
                {
                    "argv": [
                        "cargo",
                        "metadata",
                        "--format-version",
                        "1",
                        "--locked",
                        "--offline",
                        *extra_flags,
                        "--manifest-path",
                        f"<scratch>/stack/{repo_real.name}/Cargo.toml",
                    ],
                    "exit": proc.returncode,
                    "feature_mode": feature_mode,
                    "stderr": sanitize_scratch_text(proc.stderr.strip(), scratch_dir),
                }
            )
            continue
        meta = json.loads(proc.stdout)
        modes_out.append(analyze_mode(feature_mode, meta))

    # Check AFTER state of sibling checkout (INV-SUITE-04).
    after_head, after_status = snapshot_repo_state(repo_real)
    if after_head != before_head or after_status != before_status:
        print(
            f"qty_coupling.sh: INV-SUITE-04 violation: {repo_arg} changed during measurement "
            f"(HEAD before={before_head} after={after_head}; "
            f"status_blake3 before={blake3_bytes(before_status)} after={blake3_bytes(after_status)})",
            file=sys.stderr,
        )
        return 2

    out_dir = Path(opts["out_dir"])
    record_path = choose_output_path(out_dir, utc_tag)

    if cargo_errors:
        record = {
            "cargo_errors": cargo_errors,
            "dirty": dirty,
            "frankensim_rev": rev,
            "modes": [],
            "reported": REPORTED,
            "reproduced_by": "none",
            "rev_source": rev_source,
            "schema": SCHEMA,
            "source_tree": "working-tree" if working_tree else "commit",
            "status": "resolution-failed",
            "tree_hash": tree_hash,
        }
        raw_bytes = canonical_json_bytes(record)
        record_path.write_bytes(raw_bytes)
        digest = blake3_file(record_path)
        print(
            f"frankensim_rev={rev} rev_source={rev_source} status=resolution-failed "
            f"source_tree={record['source_tree']} dirty={str(dirty).lower()}"
        )
        for err in cargo_errors:
            print(
                f"qty_coupling.sh: cargo metadata ({err['feature_mode']}) exited {err['exit']}:\n"
                f"{err['stderr']}",
                file=sys.stderr,
            )
        print(f"record_path={record_path}")
        print(f"record_blake3={digest}")
        return 1

    top_reproduced = "none"
    for m in modes_out:
        if m["reproduced_by"] != "none":
            top_reproduced = m["reproduced_by"]
            break

    status = "ok-dirty" if (working_tree and dirty) else "ok"
    record = {
        "dirty": dirty,
        "frankensim_rev": rev,
        "modes": modes_out,
        "reported": REPORTED,
        "reproduced_by": top_reproduced,
        "rev_source": rev_source,
        "schema": SCHEMA,
        "source_tree": "working-tree" if working_tree else "commit",
        "status": status,
        "tree_hash": tree_hash,
    }

    raw_bytes = canonical_json_bytes(record)
    record_path.write_bytes(raw_bytes)
    digest = blake3_file(record_path)

    print(
        f"frankensim_rev={rev} rev_source={rev_source} status={status} "
        f"source_tree={record['source_tree']} dirty={str(dirty).lower()}"
    )
    for m in modes_out:
        fmode = m["feature_mode"]
        for p in m["populations"]:
            print(
                f"mode={fmode} pop={p['name']} size={p['size']} "
                f"coupled={p['coupled_count']} direct={p['direct_count']} "
                f"indirect={p['indirect_count']} "
                f"freed_in_frankensim_graph={p['freed_count']}"
            )
    print(f"reproduced_by={top_reproduced}")
    print(f"record_path={record_path}")
    print(f"record_blake3={digest}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
PY
