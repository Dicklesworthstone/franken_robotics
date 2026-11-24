#!/usr/bin/env bash
# scripts/bootstrap/first_receipt.sh
#
# Bootstrap generator and verifier for the resolution receipt (schema: frob.stack-receipt/1).
# Implements BD-001 items 5 and 6, matching N33a-04's receipt format and N34-18's canonical JSON rules.
#
# Usage:
#   scripts/bootstrap/first_receipt.sh [--root <dir>] [--lock <path>] [--out-dir <dir>]
#   scripts/bootstrap/first_receipt.sh --stdout [--root <dir>] [--lock <path>]
#   scripts/bootstrap/first_receipt.sh --verify <receipt.json> [--exact] [--root <dir>] [--lock <path>]
#   scripts/bootstrap/first_receipt.sh --tree-hash <git-dir>
#
# Read-only on sibling repositories: invokes only read-only `git --no-optional-locks -C <sibling>`
# subcommands and never modifies any file outside this repository (or --out-dir).

set -euo pipefail

usage() {
  cat >&2 <<'EOF'
Usage:
  first_receipt.sh [--root <dir>] [--lock <path>] [--out-dir <dir>]
  first_receipt.sh --stdout [--root <dir>] [--lock <path>]
  first_receipt.sh --verify <receipt.json> [--exact] [--root <dir>] [--lock <path>]
  first_receipt.sh --tree-hash <git-dir>
EOF
  exit 3
}

MODE="write"
ROOT_DIR=""
LOCK_PATH=""
OUT_DIR=""
VERIFY_FILE=""
VERIFY_EXACT="0"
TREE_HASH_DIR=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --root)
      [ "$#" -ge 2 ] || usage
      ROOT_DIR="$2"
      shift 2
      ;;
    --lock)
      [ "$#" -ge 2 ] || usage
      LOCK_PATH="$2"
      shift 2
      ;;
    --out-dir)
      [ "$#" -ge 2 ] || usage
      OUT_DIR="$2"
      shift 2
      ;;
    --stdout)
      MODE="stdout"
      shift
      ;;
    --verify)
      [ "$#" -ge 2 ] || usage
      MODE="verify"
      VERIFY_FILE="$2"
      shift 2
      ;;
    --exact)
      VERIFY_EXACT="1"
      shift
      ;;
    --tree-hash)
      [ "$#" -ge 2 ] || usage
      MODE="tree-hash"
      TREE_HASH_DIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      ;;
    *)
      printf 'first_receipt.sh: unknown argument: %s\n' "$1" >&2
      usage
      ;;
  esac
done

# Resolve b3sum (prefer PATH, fall back to /opt/homebrew/bin/b3sum on macOS)
B3SUM_BIN=""
if command -v b3sum >/dev/null 2>&1; then
  B3SUM_BIN="$(command -v b3sum)"
elif [ -x /opt/homebrew/bin/b3sum ]; then
  B3SUM_BIN="/opt/homebrew/bin/b3sum"
else
  printf 'first_receipt.sh: b3sum not found on PATH or at /opt/homebrew/bin/b3sum\n' >&2
  exit 2
fi

if ! command -v python3 >/dev/null 2>&1; then
  printf 'first_receipt.sh: python3 not found on PATH\n' >&2
  exit 2
fi

if ! command -v git >/dev/null 2>&1; then
  printf 'first_receipt.sh: git not found on PATH\n' >&2
  exit 2
fi

if [ "$MODE" != "tree-hash" ]; then
  if [ -z "$ROOT_DIR" ]; then
    ROOT_DIR="$(git --no-optional-locks rev-parse --show-toplevel 2>/dev/null || pwd)"
  fi
  if [ ! -d "$ROOT_DIR" ]; then
    printf 'first_receipt.sh: root directory does not exist: %s\n' "$ROOT_DIR" >&2
    exit 2
  fi
  ROOT_DIR="$(cd "$ROOT_DIR" && pwd)"
  if [ -z "$LOCK_PATH" ]; then
    LOCK_PATH="${ROOT_DIR}/franken-stack.lock"
  fi
  if [ -z "$OUT_DIR" ]; then
    OUT_DIR="${ROOT_DIR}/receipts"
  fi
fi

exec python3 - "$MODE" "$B3SUM_BIN" "$ROOT_DIR" "$LOCK_PATH" "$OUT_DIR" "$VERIFY_FILE" "$VERIFY_EXACT" "$TREE_HASH_DIR" <<'PYEOF'
import json
import os
import re
import stat
import struct
import subprocess
import sys
import tempfile
import tomllib
import unicodedata

MODE, B3SUM_BIN, ROOT_DIR, LOCK_PATH, OUT_DIR, VERIFY_FILE, VERIFY_EXACT, TREE_HASH_DIR = sys.argv[1:9]

HEX40_RE = re.compile(r"^[0-9a-f]{40}$")
HEX64_RE = re.compile(r"^[0-9a-f]{64}$")
INT53_MAX = (1 << 53) - 1
DRIFT_REASON_ORDER = ("dirty", "retreated", "diverged", "unknown-pin")
WORKSPACES = (
    ("core", "."),
    ("apps", "apps"),
    ("bindings", "bindings/frob-gymnasium"),
    ("derive", "derive/frob-derive"),
    ("spark-cuda", "spark/frob-spark-cuda"),
    ("spark-metal", "spark/frob-spark-metal"),
    ("worker", "worker"),
)


def fail(msg: str, code: int = 2) -> None:
    sys.stderr.write(f"first_receipt.sh: {msg}\n")
    sys.exit(code)


def nfc(s: str) -> str:
    return unicodedata.normalize("NFC", s)


def canon_str(s: str) -> str:
    s = nfc(s)
    out = ['"']
    for ch in s:
        cp = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif cp < 0x20:
            out.append(f"\\u{cp:04x}")
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def canon_json(val) -> str:
    """Serialize according to N34-18 rules 1-5:
    UTF-8, no BOM, no insignificant whitespace, no trailing newline,
    strings NFC with minimal escapes (\", \\, \\u00XX for <0x20),
    object keys sorted by Unicode code point, int53 checked.
    """
    if val is None:
        return "null"
    if isinstance(val, bool):
        return "true" if val else "false"
    if isinstance(val, int):
        if abs(val) > INT53_MAX:
            fail(f"integer out of int53 range: {val}")
        return str(val)
    if isinstance(val, str):
        return canon_str(val)
    if isinstance(val, list):
        return "[" + ",".join(canon_json(item) for item in val) + "]"
    if isinstance(val, dict):
        norm_items = {}
        for k, v in val.items():
            if not isinstance(k, str):
                fail(f"non-string JSON object key: {k!r}")
            nk = nfc(k)
            if nk in norm_items:
                fail(f"duplicate JSON object key after NFC: {nk!r}")
            norm_items[nk] = v
        # Python str ordering is Unicode code-point order (matches UTF-8 byte order).
        parts = [f"{canon_str(k)}:{canon_json(norm_items[k])}" for k in sorted(norm_items.keys())]
        return "{" + ",".join(parts) + "}"
    fail(f"unsupported value for canonical JSON: {type(val).__name__}")


def b3sum_bytes(data: bytes) -> str:
    proc = subprocess.run(
        [B3SUM_BIN, "--no-names"],
        input=data,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        fail(f"b3sum failed: {proc.stderr.decode('utf-8', 'replace').strip()}")
    digest = proc.stdout.decode("ascii").strip()
    if not HEX64_RE.match(digest):
        fail(f"unexpected b3sum output: {digest!r}")
    return digest


def b3sum_file(path: str) -> str:
    with open(path, "rb") as f:
        proc = subprocess.Popen(
            [B3SUM_BIN, "--no-names"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        assert proc.stdin is not None
        while True:
            chunk = f.read(65536)
            if not chunk:
                break
            proc.stdin.write(chunk)
        proc.stdin.close()
        out = proc.stdout.read()
        err = proc.stderr.read()
        rc = proc.wait()
    if rc != 0:
        fail(f"b3sum failed on {path}: {err.decode('utf-8', 'replace').strip()}")
    digest = out.decode("ascii").strip()
    if not HEX64_RE.match(digest):
        fail(f"unexpected b3sum output for {path}: {digest!r}")
    return digest


def git_ro(repo_dir: str, *args: str, check: bool = True) -> subprocess.CompletedProcess:
    """Run a strictly read-only git command with --no-optional-locks."""
    cmd = ["git", "--no-optional-locks", "-C", repo_dir, *args]
    proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    if check and proc.returncode != 0:
        fail(
            f"git command failed ({' '.join(args)}) in {os.path.basename(repo_dir)}: "
            f"{proc.stderr.decode('utf-8', 'replace').strip()}"
        )
    return proc


def compute_tree_hash(repo_dir: str) -> str:
    """Compute frob.tree/1 BLAKE3 tree hash (BD-001 item 6 / N33a-04):
    - Domain tag: 'frob.tree/1' fed first
    - Tracked files via `git ls-files -z`, deduplicated, sorted bytewise
    - Per path: u64_le(len(path)) + path
      - If deleted in working tree: marker byte 0xFF and no content
      - If symlink: 0x00 + u64_le(len(target)) + target
      - If regular file: (0x01 if executable else 0x00) + u64_le(len(content)) + content
    """
    proc_ls = git_ro(repo_dir, "ls-files", "-z")
    raw_entries = [p for p in proc_ls.stdout.split(b"\x00") if p]
    paths = sorted(set(raw_entries))
    repo_dir_b = os.fsencode(repo_dir)

    b3 = subprocess.Popen(
        [B3SUM_BIN, "--no-names"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    assert b3.stdin is not None
    try:
        b3.stdin.write(b"frob.tree/1")
        for rel_b in paths:
            b3.stdin.write(struct.pack("<Q", len(rel_b)))
            b3.stdin.write(rel_b)
            full_b = os.path.join(repo_dir_b, rel_b)
            if not os.path.lexists(full_b):
                b3.stdin.write(b"\xff")
                continue
            st = os.lstat(full_b)
            if stat.S_ISLNK(st.st_mode):
                target_b = os.readlink(full_b)
                b3.stdin.write(b"\x00")
                b3.stdin.write(struct.pack("<Q", len(target_b)))
                b3.stdin.write(target_b)
            elif stat.S_ISDIR(st.st_mode):
                # Gitlink/submodule entry: hash its staged commit OID if present.
                stage = git_ro(
                    repo_dir,
                    "ls-files",
                    "--stage",
                    "-z",
                    "--",
                    os.fsdecode(rel_b),
                    check=False,
                )
                oid = b""
                if stage.returncode == 0 and stage.stdout:
                    first = stage.stdout.split(b"\x00")[0].split(b"\t")[0].split(b" ")
                    if len(first) >= 2:
                        oid = first[1]
                b3.stdin.write(b"\x00")
                b3.stdin.write(struct.pack("<Q", len(oid)))
                b3.stdin.write(oid)
            else:
                exec_byte = b"\x01" if (st.st_mode & 0o111) != 0 else b"\x00"
                with open(full_b, "rb") as f:
                    # Measure exact readable bytes so header length always matches stream bytes.
                    f.seek(0, os.SEEK_END)
                    size = f.tell()
                    f.seek(0)
                    b3.stdin.write(exec_byte)
                    b3.stdin.write(struct.pack("<Q", size))
                    remaining = size
                    while remaining > 0:
                        chunk = f.read(min(65536, remaining))
                        if not chunk:
                            fail(f"short read while hashing tracked file: {os.fsdecode(rel_b)}")
                        b3.stdin.write(chunk)
                        remaining -= len(chunk)
    finally:
        b3.stdin.close()

    out = b3.stdout.read()
    err = b3.stderr.read()
    rc = b3.wait()
    if rc != 0:
        fail(f"b3sum failed during tree hash: {err.decode('utf-8', 'replace').strip()}")
    digest = out.decode("ascii").strip()
    if not HEX64_RE.match(digest):
        fail(f"unexpected tree_hash output: {digest!r}")
    return digest


def is_dirty_tracked(repo_dir: str) -> bool:
    proc = git_ro(repo_dir, "status", "--porcelain", "--untracked-files=no")
    return len(proc.stdout.strip()) > 0


def head_rev(repo_dir: str) -> str:
    rev = git_ro(repo_dir, "rev-parse", "HEAD").stdout.decode("ascii").strip()
    if not HEX40_RE.match(rev):
        fail(f"HEAD in {os.path.basename(repo_dir)} is not a 40-hex commit: {rev!r}")
    return rev


def classify_checkout(repo_dir: str, pin_rev):
    """Classify checkout against pin_rev per N33a-04:
    1. dirty -> ('drift', 'dirty')  [highest precedence]
    2. clean & H == P -> ('on-pin', None)
    3. clean & P unknown -> ('drift', 'unknown-pin')
    4. clean & P ancestor of H -> ('stale-lock', None)
    5. clean & H ancestor of P -> ('drift', 'retreated')
    6. clean & neither ancestor -> ('drift', 'diverged')
    """
    if is_dirty_tracked(repo_dir):
        return ("drift", "dirty")

    if pin_rev == "unadmitted":
        return ("on-pin", None)

    if not isinstance(pin_rev, str) or not HEX40_RE.match(pin_rev):
        return ("drift", "unknown-pin")

    h = head_rev(repo_dir)
    if h == pin_rev:
        return ("on-pin", None)

    exists = git_ro(repo_dir, "cat-file", "-e", f"{pin_rev}^{{commit}}", check=False)
    if exists.returncode != 0:
        return ("drift", "unknown-pin")

    anc_p_h = git_ro(repo_dir, "merge-base", "--is-ancestor", pin_rev, h, check=False)
    if anc_p_h.returncode == 0:
        return ("stale-lock", None)

    anc_h_p = git_ro(repo_dir, "merge-base", "--is-ancestor", h, pin_rev, check=False)
    if anc_h_p.returncode == 0:
        return ("drift", "retreated")

    return ("drift", "diverged")


def combine_classifications(pairs):
    """Combine (class, reason) pairs with precedence:
    drift (dirty > retreated > diverged > unknown-pin) > stale-lock > on-pin.
    """
    drift_reasons = [r for c, r in pairs if c == "drift"]
    if drift_reasons:
        for preferred in DRIFT_REASON_ORDER:
            if preferred in drift_reasons:
                return ("drift", preferred)
        return ("drift", drift_reasons[0])
    if any(c == "stale-lock" for c, _ in pairs):
        return ("stale-lock", None)
    return ("on-pin", None)


def parse_own_lock_entries(sib_dir: str):
    """Inspect a sibling's own lock file if present (constellation.lock, SUITE.lock, or franken-stack.lock).
    Returns (filename, blake3_hash, [(dep_dir_name, pin_rev), ...]) or (None, None, None).
    Never trusts absolute paths embedded in the sibling's lock file: resolves siblings beside sib_dir.
    """
    constellation = os.path.join(sib_dir, "constellation.lock")
    if os.path.isfile(constellation):
        lock_hash = b3sum_file(constellation)
        with open(constellation, "rb") as f:
            data = json.loads(f.read().decode("utf-8"))
        entries = []
        for lib in data.get("libraries", []):
            name = lib.get("lib")
            pin = lib.get("git_head")
            if isinstance(name, str) and name:
                entries.append((name, pin if isinstance(pin, str) else ""))
        return ("constellation.lock", lock_hash, entries)

    suite_lock = os.path.join(sib_dir, "SUITE.lock")
    if os.path.isfile(suite_lock):
        lock_hash = b3sum_file(suite_lock)
        entries = []
        with open(suite_lock, "r", encoding="utf-8") as f:
            for raw_line in f:
                line = raw_line.split("#", 1)[0].strip()
                if not line:
                    continue
                parts = line.split()
                if len(parts) >= 3 and parts[0] == "suite":
                    repo_name = parts[1]
                    commit = ""
                    for tok in parts[2:]:
                        if tok.startswith("commit="):
                            commit = tok.split("=", 1)[1]
                            break
                    entries.append((repo_name, commit))
        return ("SUITE.lock", lock_hash, entries)

    sub_stack_lock = os.path.join(sib_dir, "franken-stack.lock")
    if os.path.isfile(sub_stack_lock):
        lock_hash = b3sum_file(sub_stack_lock)
        with open(sub_stack_lock, "rb") as f:
            data = tomllib.load(f)
        entries = []
        for row in data.get("foundation", []):
            rel = row.get("path")
            pin = row.get("git_rev")
            if isinstance(rel, str) and rel and isinstance(pin, str):
                entries.append((os.path.basename(rel.rstrip("/")), pin))
        return ("franken-stack.lock", lock_hash, entries)

    return (None, None, None)


def classify_sibling_own_lock(sib_dir: str, lock_pin_rev):
    own_file, own_hash, entries = parse_own_lock_entries(sib_dir)
    if own_file is None:
        cls, reason = classify_checkout(sib_dir, lock_pin_rev)
        return {
            "agrees": cls == "on-pin",
            "class": cls,
            "file": None,
            "hash": None,
            "reason": reason,
        }

    pairs = [classify_checkout(sib_dir, lock_pin_rev)]
    sib_parent = os.path.dirname(os.path.abspath(sib_dir))
    for dep_name, dep_pin in entries:
        if not dep_name or dep_name in (".", "..") or "/" in dep_name or "\\" in dep_name:
            fail(f"invalid sibling dependency name in {own_file}: {dep_name!r}")
        dep_dir = os.path.join(sib_parent, dep_name)
        if os.path.isdir(dep_dir) and (
            os.path.exists(os.path.join(dep_dir, ".git"))
        ):
            pairs.append(classify_checkout(dep_dir, dep_pin))

    cls, reason = combine_classifications(pairs)
    return {
        "agrees": cls == "on-pin",
        "class": cls,
        "file": own_file,
        "hash": own_hash,
        "reason": reason,
    }


def validate_rel_path(label: str, path_str: str) -> str:
    if not isinstance(path_str, str) or not path_str:
        fail(f"empty path for {label}")
    if path_str.startswith("/") or path_str.startswith("~") or os.path.isabs(path_str):
        fail(f"absolute or home-relative path refused for {label}: {path_str}")
    parts = [p for p in path_str.split("/") if p]
    # N33a-03 rule 4: relative path, no '..' after the first component.
    if any(p == ".." for p in parts[1:]):
        fail(f"path traversal after first component refused for {label}: {path_str}")
    return path_str


def load_toml_file(path: str, label: str) -> dict:
    if not os.path.isfile(path):
        fail(f"required file missing ({label}): {os.path.basename(path)}")
    try:
        with open(path, "rb") as f:
            return tomllib.load(f)
    except Exception as exc:
        fail(f"failed to parse {label}: {exc}")


def parse_patches(cargo_toml: dict):
    patches_list = []
    patch_paths_by_crate = {}
    patch_tbl = cargo_toml.get("patch", {})
    if not isinstance(patch_tbl, dict):
        return patches_list, patch_paths_by_crate
    for src_name in sorted(patch_tbl.keys()):
        src_entries = patch_tbl[src_name]
        if not isinstance(src_entries, dict):
            continue
        for dep_name in sorted(src_entries.keys()):
            spec = src_entries[dep_name]
            if isinstance(spec, dict) and "path" in spec:
                rel = validate_rel_path(f"patch.{src_name}.{dep_name}", spec["path"])
                patches_list.append(f"{dep_name} -> {rel}")
                patch_paths_by_crate[dep_name] = rel
            elif isinstance(spec, dict) and "git" in spec:
                patches_list.append(f"{dep_name} -> {spec['git']}")
            elif isinstance(spec, str):
                patches_list.append(f"{dep_name} -> {spec}")
    patches_list.sort()
    return patches_list, patch_paths_by_crate


def parse_profile_overrides(cargo_toml: dict):
    overrides = []
    profiles = cargo_toml.get("profile", {})
    if not isinstance(profiles, dict):
        return overrides
    fsqlite_dev_opt1 = False
    for prof_name in sorted(profiles.keys()):
        prof_cfg = profiles[prof_name]
        if not isinstance(prof_cfg, dict):
            continue
        pkgs = prof_cfg.get("package", {})
        if not isinstance(pkgs, dict):
            continue
        for pkg_name in sorted(pkgs.keys()):
            pkg_cfg = pkgs[pkg_name]
            if not isinstance(pkg_cfg, dict):
                continue
            if (
                prof_name == "dev"
                and (pkg_name == "fsqlite" or pkg_name.startswith("fsqlite-"))
                and list(pkg_cfg.keys()) == ["opt-level"]
                and pkg_cfg.get("opt-level") == 1
            ):
                fsqlite_dev_opt1 = True
                continue
            for k in sorted(pkg_cfg.keys()):
                v = pkg_cfg[k]
                if prof_name == "dev":
                    overrides.append(f"{pkg_name}: {k} {v}")
                else:
                    overrides.append(f"{prof_name}.{pkg_name}: {k} {v}")
    if fsqlite_dev_opt1:
        overrides.append("fsqlite*: opt-level 1")
    overrides.sort()
    return overrides


def parse_build_env(root_dir: str) -> dict:
    cfg_path = os.path.join(root_dir, ".cargo", "config.toml")
    if not os.path.isfile(cfg_path):
        return {}
    cfg = load_toml_file(cfg_path, ".cargo/config.toml")
    env_tbl = cfg.get("env", {})
    if not isinstance(env_tbl, dict):
        return {}
    out = {}
    for k in sorted(env_tbl.keys()):
        v = env_tbl[k]
        if isinstance(v, dict):
            out[k] = str(v.get("value", ""))
        else:
            out[k] = str(v)
    return out


def parse_cargo_lock_counts(lock_file: str):
    data = load_toml_file(lock_file, "Cargo.lock")
    pkgs = data.get("package", [])
    if not isinstance(pkgs, list):
        pkgs = []
    total = len(pkgs)
    registry = sum(1 for p in pkgs if isinstance(p, dict) and str(p.get("source", "")).startswith("registry+"))
    return total, registry


def run_cargo_ro(ws_dir: str, argv: list[str]) -> subprocess.CompletedProcess:
    """Invoke cargo via PATH (never absolute path), in ws_dir."""
    proc = subprocess.run(
        argv,
        cwd=ws_dir,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    return proc


def parse_duplicates(tree_d_stdout: bytes) -> list[str]:
    seen = set()
    dup_re = re.compile(r"^([A-Za-z0-9_-]+)\s+v([0-9]+\.[0-9]+\.[0-9]+[^\s]*)")
    for line in tree_d_stdout.decode("utf-8", "replace").splitlines():
        if not line or line[0].isspace() or line[0] in ("├", "└", "│", "("):
            continue
        m = dup_re.match(line)
        if m:
            seen.add((m.group(1), m.group(2)))
    # List asupersync duplicates first, then remaining sorted bytewise.
    ordered = sorted(seen, key=lambda pair: (0 if pair[0] == "asupersync" else 1, pair[0], pair[1]))
    return [f"{name} {ver}" for name, ver in ordered]


def scan_no_machine_paths(obj, path="$"):
    home = os.path.expanduser("~")
    if isinstance(obj, str):
        if obj.startswith("/"):
            fail(f"absolute path leaked into receipt at {path}: {obj}")
        for forbidden in ("/Users/", "/home/", "/private/"):
            if forbidden in obj:
                fail(f"machine path segment ({forbidden}) leaked into receipt at {path}: {obj}")
        if home and len(home) > 1 and home in obj:
            fail(f"home directory leaked into receipt at {path}")
    elif isinstance(obj, list):
        for i, item in enumerate(obj):
            scan_no_machine_paths(item, f"{path}[{i}]")
    elif isinstance(obj, dict):
        for k, v in obj.items():
            scan_no_machine_paths(k, f"{path}.<key>")
            scan_no_machine_paths(v, f"{path}.{k}")


def build_receipt(root_dir: str, lock_path: str):
    cargo_toml = load_toml_file(os.path.join(root_dir, "Cargo.toml"), "Cargo.toml")
    toolchain_toml = load_toml_file(os.path.join(root_dir, "rust-toolchain.toml"), "rust-toolchain.toml")
    stack_lock = load_toml_file(lock_path, "franken-stack.lock")

    tc_tbl = toolchain_toml.get("toolchain", {})
    tc_channel = tc_tbl.get("channel") if isinstance(tc_tbl, dict) else None
    lock_channel = stack_lock.get("toolchain")
    if not isinstance(tc_channel, str) or not tc_channel:
        fail("rust-toolchain.toml missing [toolchain].channel")
    if not isinstance(lock_channel, str) or not lock_channel:
        fail("franken-stack.lock missing top-level toolchain")
    if tc_channel != lock_channel:
        fail(
            f"toolchain mismatch: rust-toolchain.toml has {tc_channel!r} "
            f"but franken-stack.lock has {lock_channel!r}"
        )

    patches, patch_paths_by_crate = parse_patches(cargo_toml)
    profile_overrides = parse_profile_overrides(cargo_toml)
    build_env = parse_build_env(root_dir)

    repo_rev = head_rev(root_dir)
    repo_dirty = is_dirty_tracked(root_dir)
    repo_tree_hash = compute_tree_hash(root_dir) if repo_dirty else None

    # Validate lockfiles and collect package counts + duplicate scan per existing workspace.
    lockfiles = []
    commands = []
    all_dups = []

    for ws_name, ws_rel in WORKSPACES:
        ws_dir = os.path.normpath(os.path.join(root_dir, ws_rel))
        ws_manifest = os.path.join(ws_dir, "Cargo.toml")
        ws_lock = os.path.join(ws_dir, "Cargo.lock")
        if ws_name == "core":
            if not os.path.isfile(ws_lock):
                fail("core Cargo.lock is missing; run cargo check and commit Cargo.lock first")
        else:
            if not os.path.isfile(ws_manifest):
                continue
            if not os.path.isfile(ws_lock):
                fail(f"annex workspace {ws_name} ({ws_rel}/Cargo.toml) is missing Cargo.lock")

        # Verify lockfile integrity via cargo metadata --locked --offline --format-version 1 (read-only).
        meta_proc = run_cargo_ro(ws_dir, ["cargo", "metadata", "--locked", "--offline", "--format-version", "1"])
        if meta_proc.returncode != 0:
            fail(
                f"cargo metadata --locked failed in workspace {ws_name}: "
                f"{meta_proc.stderr.decode('utf-8', 'replace').strip()}"
            )

        pkg_count, reg_count = parse_cargo_lock_counts(ws_lock)
        lockfiles.append(
            {
                "hash": b3sum_file(ws_lock),
                "packages": pkg_count,
                "registry": reg_count,
                "workspace": ws_name,
            }
        )

        # Run cargo tree -d to record duplicate packages and command receipt.
        tree_argv = ["cargo", "tree", "-d"]
        tree_proc = run_cargo_ro(ws_dir, tree_argv)
        if tree_proc.returncode != 0:
            fail(
                f"cargo tree -d failed in workspace {ws_name}: "
                f"{tree_proc.stderr.decode('utf-8', 'replace').strip()}"
            )
        # Strip checkout root and parent prefixes (both realpath and abspath,
        # longest first) so symlinked roots (/var -> /private/var) and renamed
        # checkout directories produce identical command output hashes.
        stdout_norm = tree_proc.stdout
        root_variants = {os.path.abspath(root_dir), os.path.realpath(root_dir)}
        parent_variants = {os.path.dirname(p) for p in root_variants}
        replacements = []
        for rp in root_variants:
            rp_clean = rp.rstrip(os.sep)
            if rp_clean:
                replacements.append((os.fsencode(rp_clean + os.sep), b"./"))
                replacements.append((os.fsencode(rp_clean), b"."))
        for pp in parent_variants:
            pp_clean = pp.rstrip(os.sep)
            if pp_clean:
                replacements.append((os.fsencode(pp_clean + os.sep), b"../"))
        replacements.sort(key=lambda pair: len(pair[0]), reverse=True)
        for prefix, repl in replacements:
            if prefix in stdout_norm:
                stdout_norm = stdout_norm.replace(prefix, repl)
        commands.append(
            {
                "argv": tree_argv if ws_name == "core" else ["cargo", "tree", "-d", "--manifest-path", f"{ws_rel}/Cargo.toml"],
                "exit": tree_proc.returncode,
                "output_hash": b3sum_bytes(stdout_norm),
            }
        )
        for dup in parse_duplicates(stdout_norm):
            if dup not in all_dups:
                all_dups.append(dup)

    all_dups.sort(key=lambda s: (0 if s.startswith("asupersync ") else 1, s))
    lockfiles.sort(key=lambda item: item["workspace"])

    # Inspect siblings from franken-stack.lock.
    siblings = []
    seen_fds = set()
    for row in stack_lock.get("foundation", []):
        if not isinstance(row, dict):
            continue
        fd = row.get("id")
        name = row.get("name", "")
        if not isinstance(fd, str) or not fd:
            fail("foundation row missing id")
        if fd in seen_fds:
            fail(f"duplicate foundation id in franken-stack.lock: {fd}")
        seen_fds.add(fd)

        rel_path = row.get("path")
        if not rel_path and isinstance(name, str) and name in patch_paths_by_crate:
            rel_path = patch_paths_by_crate[name]
        if not rel_path:
            continue

        rel_path = validate_rel_path(f"foundation.{fd}.path", rel_path)
        sib_dir = os.path.normpath(os.path.join(root_dir, rel_path))
        if not os.path.isdir(sib_dir):
            fail(f"sibling directory for {fd} ({rel_path}) does not exist")

        sib_rev = head_rev(sib_dir)
        sib_dirty = is_dirty_tracked(sib_dir)
        sib_tree_hash = compute_tree_hash(sib_dir) if sib_dirty else None
        sib_features = list(row.get("features", [])) if isinstance(row.get("features"), list) else []
        sib_own_lock = classify_sibling_own_lock(sib_dir, row.get("git_rev"))

        siblings.append(
            {
                "dirty": sib_dirty,
                "fd": fd,
                "features": sib_features,
                "own_lock": sib_own_lock,
                "rev": sib_rev,
                "tree_hash": sib_tree_hash,
            }
        )

    siblings.sort(key=lambda item: item["fd"])

    receipt = {
        "build_env": build_env,
        "commands": commands,
        "duplicates": all_dups,
        "lockfiles": lockfiles,
        "patches": patches,
        "profile_overrides": profile_overrides,
        "repo_dirty": repo_dirty,
        "repo_rev": repo_rev,
        "repo_tree_hash": repo_tree_hash,
        "schema": "frob.stack-receipt/1",
        "siblings": siblings,
        "toolchain": tc_channel,
    }
    scan_no_machine_paths(receipt)
    raw = canon_json(receipt).encode("utf-8")
    digest = b3sum_bytes(raw)
    return receipt, raw, digest


def first_diff_byte(a: bytes, b: bytes):
    n = min(len(a), len(b))
    for i in range(n):
        if a[i] != b[i]:
            return i
    if len(a) != len(b):
        return n
    return None


def verify_receipt(root_dir: str, lock_path: str, verify_file: str, exact: bool):
    if not os.path.isfile(verify_file):
        fail(f"receipt file does not exist: {verify_file}", code=2)

    with open(verify_file, "rb") as f:
        stored_bytes = f.read()

    actual_hash = b3sum_bytes(stored_bytes)
    base = os.path.basename(verify_file)
    if not base.endswith(".json"):
        fail(f"receipt filename must end with .json: {base}", code=2)
    expected_hash = base[:-5]
    if not HEX64_RE.match(expected_hash):
        fail(f"receipt filename stem is not 64-hex BLAKE3: {expected_hash}", code=2)
    if actual_hash != expected_hash:
        fail(
            f"receipt hash mismatch (tampered or misnamed): filename={expected_hash} actual={actual_hash}",
            code=2,
        )

    try:
        stored_obj = json.loads(stored_bytes.decode("utf-8"))
    except Exception as exc:
        fail(f"receipt is not valid UTF-8 JSON: {exc}", code=2)

    recanon = canon_json(stored_obj).encode("utf-8")
    if recanon != stored_bytes:
        off = first_diff_byte(stored_bytes, recanon)
        fail(f"receipt is not canonical JSON (first diff byte offset {off})", code=2)

    if stored_obj.get("schema") != "frob.stack-receipt/1":
        fail(f"unexpected schema: {stored_obj.get('schema')!r}", code=2)

    scan_no_machine_paths(stored_obj)

    stored_rev = stored_obj.get("repo_rev")
    if not isinstance(stored_rev, str) or not HEX40_RE.match(stored_rev):
        fail(f"invalid repo_rev in receipt: {stored_rev!r}", code=2)

    current_head = head_rev(root_dir)
    if exact:
        if stored_rev != current_head:
            fail(f"repo_rev mismatch under --exact: stored={stored_rev} head={current_head}", code=2)
    else:
        anc = git_ro(root_dir, "merge-base", "--is-ancestor", stored_rev, current_head, check=False)
        if anc.returncode != 0:
            fail(f"receipt repo_rev {stored_rev} is not an ancestor of HEAD {current_head}", code=2)

    current_obj, _, _ = build_receipt(root_dir, lock_path)

    stored_cmp = dict(stored_obj)
    current_cmp = dict(current_obj)
    if not exact:
        stored_cmp.pop("repo_rev", None)
        current_cmp.pop("repo_rev", None)

    stored_cmp_bytes = canon_json(stored_cmp).encode("utf-8")
    current_cmp_bytes = canon_json(current_cmp).encode("utf-8")
    if stored_cmp_bytes != current_cmp_bytes:
        off = first_diff_byte(stored_cmp_bytes, current_cmp_bytes)
        diffs = []
        for k in sorted(set(stored_cmp.keys()) | set(current_cmp.keys())):
            if stored_cmp.get(k) != current_cmp.get(k):
                diffs.append(k)
        fail(
            f"receipt verification failed: differing fields={diffs} first_diff_offset={off}",
            code=2,
        )

    sys.stdout.write(f"ok {actual_hash}\n")


if MODE == "tree-hash":
    if not os.path.isdir(TREE_HASH_DIR):
        fail(f"directory does not exist: {TREE_HASH_DIR}", code=2)
    sys.stdout.write(compute_tree_hash(TREE_HASH_DIR) + "\n")
elif MODE == "stdout":
    _, raw, _ = build_receipt(ROOT_DIR, LOCK_PATH)
    sys.stdout.buffer.write(raw)
elif MODE == "verify":
    verify_receipt(ROOT_DIR, LOCK_PATH, VERIFY_FILE, VERIFY_EXACT == "1")
elif MODE == "write":
    _, raw, digest = build_receipt(ROOT_DIR, LOCK_PATH)
    os.makedirs(OUT_DIR, exist_ok=True)
    dest = os.path.join(OUT_DIR, f"{digest}.json")
    if os.path.isfile(dest):
        with open(dest, "rb") as f:
            existing = f.read()
        if existing != raw:
            fail(
                f"existing receipt at {os.path.basename(dest)} has tampered bytes; refusing to overwrite",
                code=2,
            )
    else:
        fd_tmp, tmp_path = tempfile.mkstemp(prefix=".receipt-", suffix=".tmp", dir=OUT_DIR)
        try:
            with os.fdopen(fd_tmp, "wb") as f:
                f.write(raw)
                f.flush()
                os.fsync(f.fileno())
            os.replace(tmp_path, dest)
        except Exception:
            raise
    sys.stdout.write(digest + "\n")
else:
    fail(f"unknown mode: {MODE}", code=3)
PYEOF
