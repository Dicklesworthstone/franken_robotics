#!/usr/bin/env bash
# Suite survey (bead N33c-07): read-only survey of fifteen sibling repositories.
#
# Re-reads git HEAD, modified path count, toolchain channel, asupersync
# requirement, and watched manifest keys for every sibling marked `survey = true`
# in `registries/siblings.toml` (schema `frob.siblings/1`). Writes a canonical
# JSON record `receipts/surveys/<utc>.json` (schema `frob.suite-survey/1`) and
# prints a one-line-per-change diff against the previous survey (or the embedded
# 2026-11-21 starting table `tests/admission/survey_2026-11-21.json`).
#
# Never runs `cargo` and never writes inside any sibling repository.
#
# Exit codes:
#   0  survey completed; no consumed sibling changed since the previous survey
#   2  usage, schema, or I/O error
#   3  survey completed; at least one consumed sibling changed head, channel, or
#      asupersync requirement (with no stack lock, every surveyed sibling counts
#      as consumed, fail-closed)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEFAULT_REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

REPO_ROOT="$DEFAULT_REPO_ROOT"
STACK_LOCK=""
STACK_LOCK_EXPLICIT=0
REGISTER_PATH=""
OUT_DIR=""
BASELINE_PATH=""
BASELINE_EXPLICIT=0
DATE_OVERRIDE=""
DERIVE_CHECK=""

usage() {
  cat <<'EOF'
Usage: scripts/admission/suite_survey.sh [OPTIONS]

Read-only suite survey across the sibling repositories listed in
registries/siblings.toml (schema frob.siblings/1).

Options:
  --stack-lock <path>  Path to franken-stack.lock (default: <repo-root>/franken-stack.lock)
  --register <path>    Path to siblings.toml (default: <repo-root>/registries/siblings.toml)
  --out-dir <path>     Directory for survey records (default: <repo-root>/receipts/surveys)
  --baseline <path>    Previous survey JSON to diff against (default: newest file in
                       --out-dir, or <repo-root>/tests/admission/survey_2026-11-21.json)
  --previous <path>    Alias for --baseline
  --repo-root <path>   Base directory used to resolve relative sibling paths
  --date <utc>         Override UTC timestamp (YYYY-MM-DDTHH:MM:SSZ) for the record
  --derive-only <json> Recompute derived fields of an existing survey JSON and print
                       canonical JSON to stdout (no file written)
  -h, --help           Show this help and exit 0
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --stack-lock)
      [ "$#" -ge 2 ] || { echo "suite_survey: --stack-lock requires an argument" >&2; exit 2; }
      STACK_LOCK="$2"
      STACK_LOCK_EXPLICIT=1
      shift 2
      ;;
    --register)
      [ "$#" -ge 2 ] || { echo "suite_survey: --register requires an argument" >&2; exit 2; }
      REGISTER_PATH="$2"
      shift 2
      ;;
    --out-dir)
      [ "$#" -ge 2 ] || { echo "suite_survey: --out-dir requires an argument" >&2; exit 2; }
      OUT_DIR="$2"
      shift 2
      ;;
    --baseline|--previous)
      [ "$#" -ge 2 ] || { echo "suite_survey: $1 requires an argument" >&2; exit 2; }
      BASELINE_PATH="$2"
      BASELINE_EXPLICIT=1
      shift 2
      ;;
    --repo-root)
      [ "$#" -ge 2 ] || { echo "suite_survey: --repo-root requires an argument" >&2; exit 2; }
      REPO_ROOT="$2"
      shift 2
      ;;
    --date)
      [ "$#" -ge 2 ] || { echo "suite_survey: --date requires an argument" >&2; exit 2; }
      DATE_OVERRIDE="$2"
      shift 2
      ;;
    --derive-only)
      [ "$#" -ge 2 ] || { echo "suite_survey: --derive-only requires an argument" >&2; exit 2; }
      DERIVE_CHECK="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "suite_survey: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [ "$STACK_LOCK_EXPLICIT" -eq 0 ]; then
  STACK_LOCK="$REPO_ROOT/franken-stack.lock"
fi
if [ -z "$REGISTER_PATH" ]; then
  REGISTER_PATH="$REPO_ROOT/registries/siblings.toml"
fi
if [ -z "$OUT_DIR" ]; then
  OUT_DIR="$REPO_ROOT/receipts/surveys"
fi
if [ "$BASELINE_EXPLICIT" -eq 0 ]; then
  BASELINE_PATH="$DEFAULT_REPO_ROOT/tests/admission/survey_2026-11-21.json"
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "suite_survey: python3 is required" >&2
  exit 2
fi
if ! command -v git >/dev/null 2>&1; then
  echo "suite_survey: git is required" >&2
  exit 2
fi
if ! command -v b3sum >/dev/null 2>&1; then
  echo "suite_survey: b3sum is required" >&2
  exit 2
fi

export FROB_SURVEY_REPO_ROOT="$REPO_ROOT"
export FROB_SURVEY_STACK_LOCK="$STACK_LOCK"
export FROB_SURVEY_REGISTER="$REGISTER_PATH"
export FROB_SURVEY_OUT_DIR="$OUT_DIR"
export FROB_SURVEY_BASELINE="$BASELINE_PATH"
export FROB_SURVEY_BASELINE_EXPLICIT="$BASELINE_EXPLICIT"
export FROB_SURVEY_DATE="$DATE_OVERRIDE"
export FROB_SURVEY_DERIVE_ONLY="$DERIVE_CHECK"

python3 - <<'PY'
import datetime
import glob
import json
import os
import re
import subprocess
import sys
import tomllib
import unicodedata

CANDIDATE_A_CHANNEL = "nightly-2026-08-31"
CONSUMED_POSTURES = frozenset({"consume", "consume-path", "build-time"})
VALID_POSTURES = frozenset({
    "consume",
    "consume-path",
    "subprocess",
    "lift",
    "oracle",
    "pattern",
    "build-time",
})
VALID_CARGO_POLICIES = frozenset({"", "allowed", "dev-only", "never"})
ALLOWED_TOP_KEYS = frozenset({"schema", "sibling"})
ALLOWED_ROW_KEYS = frozenset({
    "name",
    "path",
    "survey",
    "toolchain_file",
    "asupersync_manifest",
    "asupersync_key",
    "watched",
    # Policy columns owned by N33c-09 (may be empty or omitted here)
    "posture",
    "cargo",
    "allowed_in",
    "forbidden_features",
    "reason",
    "fallback",
})
HEX40_RE = re.compile(r"^[0-9a-f]{40}$")
UTC_ISO_RE = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
CARGO_WILD_RE = re.compile(r"^(?:(\*|[xX])|(\d+)\.(\*|[xX])|(\d+)\.(\d+)\.(\*|[xX]))$")
CARGO_COMP_RE = re.compile(
    r"^(>=|<=|!=|=|>|<|\^|~)?\s*(\d+)(?:\.(\d+))?(?:\.(\d+))?(?:-([0-9A-Za-z.-]+))?(?:\+[0-9A-Za-z.-]+)?$"
)


def nfc(obj):
    if isinstance(obj, str):
        return unicodedata.normalize("NFC", obj)
    if isinstance(obj, list):
        return [nfc(x) for x in obj]
    if isinstance(obj, dict):
        return {nfc(k): nfc(v) for k, v in obj.items()}
    return obj


def canonical_json(obj) -> str:
    return json.dumps(nfc(obj), sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def blake3_file(path: str) -> str:
    proc = subprocess.run(["b3sum", "--no-names", path], capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(f"b3sum failed on {path}: {proc.stderr.strip()}")
    digest = proc.stdout.strip().lower()
    if len(digest) != 64:
        raise RuntimeError(f"unexpected b3sum digest length for {path}: {digest!r}")
    return digest


def blake3_bytes(data: bytes) -> str:
    proc = subprocess.run(["b3sum", "--no-names", "-"], input=data, capture_output=True)
    if proc.returncode != 0:
        raise RuntimeError(f"b3sum failed: {proc.stderr.decode('utf-8', 'replace').strip()}")
    return proc.stdout.decode("utf-8").strip().lower()


def load_siblings_register(path: str) -> list[dict]:
    if not os.path.isfile(path):
        raise ValueError(f"siblings register not found: {path}")
    with open(path, "rb") as f:
        raw = f.read()
    data = tomllib.loads(raw.decode("utf-8"))

    unknown_top = set(data.keys()) - ALLOWED_TOP_KEYS
    if unknown_top:
        raise ValueError(f"{path}: unknown top-level key(s): {sorted(unknown_top)}")
    if data.get("schema") != "frob.siblings/1":
        raise ValueError(f"{path}: expected schema = 'frob.siblings/1', got {data.get('schema')!r}")

    rows = data.get("sibling")
    if not isinstance(rows, list):
        raise ValueError(f"{path}: [[sibling]] array of tables is required")

    seen_names = set()
    validated = []
    for idx, row in enumerate(rows):
        if not isinstance(row, dict):
            raise ValueError(f"{path}: sibling[{idx}] is not a table")
        unknown = set(row.keys()) - ALLOWED_ROW_KEYS
        if unknown:
            raise ValueError(f"{path}: sibling[{idx}] ({row.get('name', '?')}): unknown key(s): {sorted(unknown)}")
        name = row.get("name")
        rel_path = row.get("path")
        survey = row.get("survey")
        if not isinstance(name, str) or not name:
            raise ValueError(f"{path}: sibling[{idx}]: 'name' must be a non-empty string")
        if name in seen_names:
            raise ValueError(f"{path}: duplicate sibling name {name!r}")
        seen_names.add(name)
        if not isinstance(rel_path, str) or not rel_path:
            raise ValueError(f"{path}: sibling {name!r}: 'path' must be a non-empty string")
        norm_parts = [p for p in rel_path.replace("\\", "/").split("/") if p and p != "."]
        if (
            rel_path.startswith("/")
            or rel_path.startswith("~")
            or not norm_parts
            or norm_parts == [".."]
            or ".." in norm_parts[1:]
        ):
            raise ValueError(f"{path}: sibling {name!r}: 'path' must be a relative sibling path with no secondary '..' traversal, got {rel_path!r}")
        if not isinstance(survey, bool):
            raise ValueError(f"{path}: sibling {name!r}: 'survey' must be a boolean")

        toolchain_file = row.get("toolchain_file", "rust-toolchain.toml")
        if not isinstance(toolchain_file, str):
            raise ValueError(f"{path}: sibling {name!r}: 'toolchain_file' must be a string")

        asup_manifest = row.get("asupersync_manifest", "Cargo.toml")
        asup_key = row.get("asupersync_key", "")
        if not isinstance(asup_manifest, str) or not isinstance(asup_key, str):
            raise ValueError(f"{path}: sibling {name!r}: asupersync_manifest/asupersync_key must be strings")

        watched = row.get("watched", {})
        if not isinstance(watched, dict) or not all(
            isinstance(k, str) and isinstance(v, str) for k, v in watched.items()
        ):
            raise ValueError(f"{path}: sibling {name!r}: 'watched' must be a table of string -> string")

        # Validate policy columns when populated; empty/omitted is accepted (owned by N33c-09)
        posture = row.get("posture", [])
        if not isinstance(posture, list) or not all(isinstance(p, str) for p in posture):
            raise ValueError(f"{path}: sibling {name!r}: 'posture' must be a list of strings")
        bad_postures = [p for p in posture if p not in VALID_POSTURES]
        if bad_postures:
            raise ValueError(f"{path}: sibling {name!r}: unknown posture(s): {bad_postures}")

        cargo_pol = row.get("cargo", "")
        if not isinstance(cargo_pol, str) or cargo_pol not in VALID_CARGO_POLICIES:
            raise ValueError(f"{path}: sibling {name!r}: invalid cargo policy {cargo_pol!r}")

        for list_col in ("allowed_in", "forbidden_features"):
            val = row.get(list_col, [])
            if not isinstance(val, list) or not all(isinstance(x, str) for x in val):
                raise ValueError(f"{path}: sibling {name!r}: {list_col!r} must be a list of strings")
        for str_col in ("reason", "fallback"):
            val = row.get(str_col, "")
            if not isinstance(val, str):
                raise ValueError(f"{path}: sibling {name!r}: {str_col!r} must be a string")

        validated.append({
            "name": name,
            "path": rel_path,
            "survey": survey,
            "toolchain_file": toolchain_file,
            "asupersync_manifest": asup_manifest,
            "asupersync_key": asup_key,
            "watched": watched,
        })
    return validated


def load_stack_lock(lock_path: str) -> tuple[str | None, set[str] | None]:
    """Return (lock_blake3_or_None, consumed_names_set_or_None).

    When the lock file is absent, returns (None, None) meaning fail-closed:
    every surveyed sibling counts as consumed.
    """
    if not os.path.isfile(lock_path):
        return None, None

    digest = blake3_file(lock_path)
    with open(lock_path, "rb") as f:
        data = tomllib.loads(f.read().decode("utf-8"))

    consumed: set[str] = set()
    foundations = data.get("foundation", [])
    if not isinstance(foundations, list):
        raise ValueError(f"{lock_path}: [[foundation]] must be a list of tables")
    for row in foundations:
        if not isinstance(row, dict):
            continue
        name = row.get("name")
        if not isinstance(name, str) or not name:
            continue
        is_consumed = False
        posture = row.get("posture")
        if isinstance(posture, str) and posture in CONSUMED_POSTURES:
            is_consumed = True
        uses = row.get("uses")
        if isinstance(uses, list):
            for entry in uses:
                if isinstance(entry, dict):
                    up = entry.get("posture")
                    if isinstance(up, str) and up in CONSUMED_POSTURES:
                        is_consumed = True
                        break
        if is_consumed:
            consumed.add(name)
    return digest, consumed


def lookup_dotted(doc: dict, dotted_key: str):
    if not dotted_key:
        return None
    cur = doc
    for part in dotted_key.split("."):
        if isinstance(cur, dict) and part in cur:
            cur = cur[part]
        else:
            return None
    return cur


def read_toml_file(path: str) -> dict | None:
    if not os.path.isfile(path):
        return None
    with open(path, "rb") as f:
        return tomllib.loads(f.read().decode("utf-8"))


def read_channel(sibling_dir: str, toolchain_file: str) -> str | None:
    candidates = [toolchain_file] if toolchain_file else []
    for fallback in ("rust-toolchain.toml", "rust-toolchain"):
        if fallback not in candidates:
            candidates.append(fallback)
    for rel in candidates:
        fp = os.path.join(sibling_dir, rel)
        if not os.path.isfile(fp):
            continue
        with open(fp, "rb") as f:
            raw = f.read().decode("utf-8")
        try:
            doc = tomllib.loads(raw)
            ch = lookup_dotted(doc, "toolchain.channel")
            if isinstance(ch, str) and ch.strip():
                return ch.strip()
        except tomllib.TOMLDecodeError as exc:
            if rel.endswith(".toml"):
                raise ValueError(f"invalid TOML in {rel}: {exc}") from exc
            # Legacy plain-text rust-toolchain file containing just the channel name
            for line in raw.splitlines():
                s = line.strip()
                if not s or s.startswith("#"):
                    continue
                if s.startswith("[") or "=" in s:
                    raise ValueError(f"invalid TOML in {rel}: {exc}") from exc
                return s
    return None


def parse_asupersync_spec(val, key_path: str) -> dict:
    # Self package (asupersync repository itself)
    if key_path == "package.version" and isinstance(val, str):
        return {
            "kind": "self",
            "requirement": val,
            "optional": False,
            "default_features": True,
        }
    if isinstance(val, str):
        return {
            "kind": "registry",
            "requirement": val,
            "optional": False,
            "default_features": True,
        }
    if isinstance(val, dict):
        optional = bool(val.get("optional", False))
        if "default-features" in val:
            default_features = bool(val["default-features"])
        elif "default_features" in val:
            default_features = bool(val["default_features"])
        else:
            default_features = True
        if "path" in val and isinstance(val["path"], str):
            return {
                "kind": "path",
                "requirement": val["path"],
                "optional": optional,
                "default_features": default_features,
            }
        if "git" in val and isinstance(val["git"], str):
            ref = val.get("rev") or val.get("tag") or val.get("branch")
            req = f"{val['git']}@{ref}" if isinstance(ref, str) and ref else val["git"]
            return {
                "kind": "git",
                "requirement": req,
                "optional": optional,
                "default_features": default_features,
            }
        if "version" in val and isinstance(val["version"], str):
            return {
                "kind": "registry",
                "requirement": val["version"],
                "optional": optional,
                "default_features": default_features,
            }
    return {
        "kind": "none",
        "requirement": None,
        "optional": False,
        "default_features": None,
    }


def read_asupersync(sibling_dir: str, manifest_rel: str, key_path: str, watched_vals: dict) -> dict:
    inherited_optional: bool | None = None
    inherited_df: bool | None = None

    # 1. Try the configured manifest and key first.
    if manifest_rel and key_path:
        doc = read_toml_file(os.path.join(sibling_dir, manifest_rel))
        if doc is not None:
            val = lookup_dotted(doc, key_path)
            if isinstance(val, dict) and val.get("workspace") is True:
                # Member inherits from root [workspace.dependencies]; retain member overrides
                # and fall through to resolve the workspace-level specification.
                if "optional" in val:
                    inherited_optional = bool(val["optional"])
                if "default-features" in val:
                    inherited_df = bool(val["default-features"])
                elif "default_features" in val:
                    inherited_df = bool(val["default_features"])
            elif val is not None:
                spec = parse_asupersync_spec(val, key_path)
                if spec["kind"] != "none":
                    # Cargo does not allow `optional = true` under [workspace.dependencies];
                    # honor a declared watched `asupersync_optional` boolean if present.
                    if (
                        key_path.startswith("workspace.dependencies.")
                        and isinstance(watched_vals.get("asupersync_optional"), bool)
                    ):
                        spec["optional"] = watched_vals["asupersync_optional"]
                    return spec

    # 2. Fallback read-only scan of root Cargo.toml and declared workspace members
    #    so workspace-inherited or newly added asupersync dependencies are never missed.
    root_doc = read_toml_file(os.path.join(sibling_dir, "Cargo.toml"))
    if root_doc is not None:
        if lookup_dotted(root_doc, "package.name") == "asupersync":
            ver = lookup_dotted(root_doc, "package.version")
            if isinstance(ver, str):
                return parse_asupersync_spec(ver, "package.version")
        for candidate_key in (
            "patch.crates-io.asupersync",
            "workspace.dependencies.asupersync",
            "dependencies.asupersync",
        ):
            val = lookup_dotted(root_doc, candidate_key)
            if val is not None:
                spec = parse_asupersync_spec(val, candidate_key)
                if spec["kind"] != "none":
                    if inherited_optional is not None:
                        spec["optional"] = inherited_optional
                    elif (
                        candidate_key == "workspace.dependencies.asupersync"
                        and isinstance(watched_vals.get("asupersync_optional"), bool)
                    ):
                        spec["optional"] = watched_vals["asupersync_optional"]
                    if inherited_df is not None:
                        spec["default_features"] = inherited_df
                    return spec
        members = lookup_dotted(root_doc, "workspace.members")
        excludes = lookup_dotted(root_doc, "workspace.exclude")
        excluded_dirs: set[str] = set()
        if isinstance(excludes, list):
            for epat in excludes:
                if not isinstance(epat, str):
                    continue
                for edir in glob.glob(os.path.join(sibling_dir, epat)):
                    excluded_dirs.add(os.path.realpath(edir))
                excluded_dirs.add(os.path.realpath(os.path.join(sibling_dir, epat)))
        if isinstance(members, list):
            for pat in members:
                if not isinstance(pat, str):
                    continue
                for member_dir in sorted(glob.glob(os.path.join(sibling_dir, pat))):
                    mreal = os.path.realpath(member_dir)
                    if any(mreal == ed or mreal.startswith(ed + os.sep) for ed in excluded_dirs):
                        continue
                    mdoc = read_toml_file(os.path.join(member_dir, "Cargo.toml"))
                    if mdoc is None:
                        continue
                    val = lookup_dotted(mdoc, "dependencies.asupersync")
                    if val is not None and not (isinstance(val, dict) and val.get("workspace") is True):
                        spec = parse_asupersync_spec(val, "dependencies.asupersync")
                        if spec["kind"] != "none":
                            return spec

    return {
        "kind": "none",
        "requirement": None,
        "optional": False,
        "default_features": None,
    }


def normalize_watched_value(val):
    if isinstance(val, (str, bool, int)):
        return val
    if isinstance(val, dict):
        # For dependency specs like serde_json = { version = "=1.0.151", ... }
        if "version" in val and isinstance(val["version"], str):
            return val["version"]
        if "path" in val and isinstance(val["path"], str):
            return val["path"]
        out = {}
        for k in sorted(val.keys()):
            v = val[k]
            if isinstance(v, (str, bool, int)):
                out[k] = v
            elif isinstance(v, list) and all(isinstance(x, (str, bool, int)) for x in v):
                out[k] = v
        return out
    return None


def read_watched(sibling_dir: str, watched_spec: dict[str, str]) -> dict:
    result = {}
    toml_cache: dict[str, dict | None] = {}
    for wname in sorted(watched_spec.keys()):
        spec = watched_spec[wname]
        if ":" not in spec:
            continue
        manifest_rel, key_path = spec.split(":", 1)
        if manifest_rel not in toml_cache:
            toml_cache[manifest_rel] = read_toml_file(os.path.join(sibling_dir, manifest_rel))
        doc = toml_cache[manifest_rel]
        if doc is None:
            continue
        val = lookup_dotted(doc, key_path)
        if val is None:
            continue
        norm = normalize_watched_value(val)
        if norm is not None:
            result[wname] = norm
    return result


def survey_one_sibling(repo_root: str, entry: dict) -> dict:
    name = entry["name"]
    sibling_dir = os.path.normpath(os.path.join(repo_root, entry["path"]))
    if not os.path.isdir(sibling_dir):
        return {
            "name": name,
            "head": "absent",
            "modified_paths": None,
            "channel": None,
            "asupersync": {
                "kind": "absent",
                "requirement": None,
                "optional": None,
                "default_features": None,
            },
            "watched": {},
        }

    head_proc = subprocess.run(
        ["git", "--no-optional-locks", "-C", sibling_dir, "rev-parse", "HEAD"],
        capture_output=True,
        text=True,
    )
    if head_proc.returncode != 0:
        raise RuntimeError(f"git rev-parse HEAD failed in {entry['path']}: {head_proc.stderr.strip()}")
    head = head_proc.stdout.strip().lower()
    if not HEX40_RE.match(head):
        raise RuntimeError(f"unexpected HEAD in {entry['path']}: {head!r}")

    porc_proc = subprocess.run(
        ["git", "--no-optional-locks", "-C", sibling_dir, "status", "--porcelain"],
        capture_output=True,
        text=True,
    )
    if porc_proc.returncode != 0:
        raise RuntimeError(f"git status --porcelain failed in {entry['path']}: {porc_proc.stderr.strip()}")
    modified_paths = len([ln for ln in porc_proc.stdout.splitlines() if ln])

    channel = read_channel(sibling_dir, entry["toolchain_file"])
    watched = read_watched(sibling_dir, entry["watched"])
    asupersync = read_asupersync(
        sibling_dir,
        entry["asupersync_manifest"],
        entry["asupersync_key"],
        watched,
    )

    return {
        "name": name,
        "head": head,
        "modified_paths": modified_paths,
        "channel": channel,
        "asupersync": asupersync,
        "watched": watched,
    }


def _clause_matches_release(clause: str, target: tuple[int, int, int]) -> bool:
    """Exact Cargo semver comparator evaluation against a release triple (M, m, p)."""
    tmaj, tmin, tpat = target
    wm = CARGO_WILD_RE.match(clause)
    if wm:
        if wm.group(1) is not None:
            return True
        if wm.group(2) is not None:
            return tmaj == int(wm.group(2))
        return tmaj == int(wm.group(4)) and tmin == int(wm.group(5))

    m = CARGO_COMP_RE.match(clause)
    if not m:
        return False
    op = m.group(1) or "^"
    maj = int(m.group(2))
    minor = int(m.group(3)) if m.group(3) is not None else None
    patch = int(m.group(4)) if m.group(4) is not None else None
    pre = m.group(5)  # Non-None means a pre-release comparator (e.g. -alpha.1)

    if op == "^":
        if minor is None:
            lo, hi = (maj, 0, 0), (maj + 1, 0, 0)
        elif patch is None:
            lo = (maj, minor, 0)
            hi = (maj + 1, 0, 0) if maj > 0 else (0, minor + 1, 0)
        else:
            lo = (maj, minor, patch)
            if maj > 0:
                hi = (maj + 1, 0, 0)
            elif minor > 0:
                hi = (0, minor + 1, 0)
            else:
                hi = (0, 0, patch + 1)
        if pre is not None and target == lo:
            # Release 0.5.0 is greater than 0.5.0-alpha.1, so ^0.5.0-alpha.1 matches 0.5.0
            return lo <= target < hi
        return lo <= target < hi

    if op == "~":
        if minor is None:
            lo, hi = (maj, 0, 0), (maj + 1, 0, 0)
        else:
            lo, hi = (maj, minor, patch or 0), (maj, minor + 1, 0)
        return lo <= target < hi

    if op == "=":
        if pre is not None:
            return False
        if minor is None:
            return tmaj == maj
        if patch is None:
            return tmaj == maj and tmin == minor
        return target == (maj, minor, patch)

    if op == "!=":
        if pre is not None:
            return True
        if minor is None:
            return tmaj != maj
        if patch is None:
            return (tmaj, tmin) != (maj, minor)
        return target != (maj, minor, patch)

    if op == ">=":
        return target >= (maj, minor or 0, patch or 0)

    if op == ">":
        if pre is not None and target == (maj, minor or 0, patch or 0):
            return True
        if minor is None:
            return tmaj > maj
        if patch is None:
            return (tmaj, tmin) > (maj, minor)
        return target > (maj, minor, patch)

    if op == "<=":
        if pre is not None and target == (maj, minor or 0, patch or 0):
            return False
        if minor is None:
            return tmaj <= maj
        if patch is None:
            return (tmaj, tmin) <= (maj, minor)
        return target <= (maj, minor, patch)

    if op == "<":
        return target < (maj, minor or 0, patch or 0)

    return False


def names_asupersync_0_5_0(asup: dict) -> bool:
    if not isinstance(asup, dict):
        return False
    if asup.get("kind") != "registry":
        return False
    req = asup.get("requirement")
    if not isinstance(req, str) or not req.strip():
        return False
    clauses = [c.strip() for c in req.split(",") if c.strip()]
    if not clauses:
        return False
    return all(_clause_matches_release(clause, (0, 5, 0)) for clause in clauses)


def compute_derived(rows: list[dict]) -> dict:
    channels = sorted({r["channel"] for r in rows if isinstance(r.get("channel"), str) and r["channel"]})
    asup_050 = [r["name"] for r in rows if names_asupersync_0_5_0(r.get("asupersync", {}))]
    not_cand_a = [
        r["name"]
        for r in rows
        if isinstance(r.get("channel"), str) and r["channel"] and r["channel"] != CANDIDATE_A_CHANNEL
    ]
    min_rust = {}
    for r in rows:
        watched = r.get("watched") or {}
        rv = watched.get("rust_version")
        if isinstance(rv, str) and rv:
            min_rust[r["name"]] = rv
    return {
        "channels_in_play": channels,
        "asupersync_0_5_0_named_by": asup_050,
        "not_on_candidate_a": not_cand_a,
        "minimum_rust_versions": dict(sorted(min_rust.items())),
    }


def format_asup(asup: dict) -> str:
    kind = asup.get("kind")
    req = asup.get("requirement")
    opt = asup.get("optional")
    df = asup.get("default_features")
    if kind in ("none", "absent", "not-read"):
        return str(kind)
    return f"{kind}:{req}(optional={json.dumps(opt)},default_features={json.dumps(df)})"


def heads_equal(old_head: str | None, new_head: str | None) -> bool:
    if old_head == new_head:
        return True
    if (
        isinstance(old_head, str)
        and isinstance(new_head, str)
        and old_head != "absent"
        and new_head != "absent"
        and len(old_head) >= 12
        and new_head.startswith(old_head)
    ):
        return True
    return False


def diff_surveys(
    prev_doc: dict | None,
    curr_rows: list[dict],
    consumed_names: set[str] | None,
) -> tuple[list[str], bool]:
    if prev_doc is None:
        return [], False
    prev_by_name = {r["name"]: r for r in prev_doc.get("rows", []) if isinstance(r, dict) and "name" in r}
    lines: list[str] = []
    consumed_changed = False

    for row in curr_rows:
        name = row["name"]
        if name not in prev_by_name:
            continue
        old = prev_by_name[name]
        is_consumed = consumed_names is None or name in consumed_names
        tag = "consumed" if is_consumed else "unconsumed"

        if not heads_equal(old.get("head"), row.get("head")):
            lines.append(f"DIFF {name} head: {old.get('head')} -> {row.get('head')} [{tag}]")
            if is_consumed:
                consumed_changed = True

        if old.get("channel") != row.get("channel"):
            lines.append(f"DIFF {name} channel: {old.get('channel')} -> {row.get('channel')} [{tag}]")
            if is_consumed:
                consumed_changed = True

        old_asup = old.get("asupersync") or {}
        new_asup = row.get("asupersync") or {}
        asup_keys = ("kind", "requirement", "optional", "default_features")
        if tuple(old_asup.get(k) for k in asup_keys) != tuple(new_asup.get(k) for k in asup_keys):
            lines.append(
                f"DIFF {name} asupersync: {format_asup(old_asup)} -> {format_asup(new_asup)} [{tag}]"
            )
            if is_consumed:
                consumed_changed = True

    return lines, consumed_changed


def find_previous_survey(out_dir: str, baseline_path: str, baseline_explicit: bool) -> str | None:
    if baseline_explicit:
        if not os.path.isfile(baseline_path):
            raise ValueError(f"baseline survey not found: {baseline_path}")
        return baseline_path
    if os.path.isdir(out_dir):
        existing = sorted(
            (f for f in os.listdir(out_dir) if f.endswith(".json") and os.path.isfile(os.path.join(out_dir, f))),
            key=lambda fn: fn[:-5],
        )
        if existing:
            return os.path.join(out_dir, existing[-1])
    if os.path.isfile(baseline_path):
        return baseline_path
    return None


def main() -> int:
    derive_only = os.environ.get("FROB_SURVEY_DERIVE_ONLY", "")
    if derive_only:
        try:
            with open(derive_only, "rb") as f:
                doc = json.loads(f.read().decode("utf-8"))
            rows = doc.get("rows", [])
            out = dict(doc)
            out.update(compute_derived(rows))
            print(canonical_json(out))
            return 0
        except Exception as exc:
            print(f"suite_survey: derive-only error: {exc}", file=sys.stderr)
            return 2

    repo_root = os.environ["FROB_SURVEY_REPO_ROOT"]
    stack_lock_path = os.environ["FROB_SURVEY_STACK_LOCK"]
    register_path = os.environ["FROB_SURVEY_REGISTER"]
    out_dir = os.environ["FROB_SURVEY_OUT_DIR"]
    baseline_path = os.environ["FROB_SURVEY_BASELINE"]
    baseline_explicit = os.environ["FROB_SURVEY_BASELINE_EXPLICIT"] == "1"
    date_override = os.environ.get("FROB_SURVEY_DATE", "").strip()

    try:
        entries = load_siblings_register(register_path)
    except Exception as exc:
        print(f"suite_survey: register error: {exc}", file=sys.stderr)
        return 2

    try:
        stack_lock_digest, consumed_names = load_stack_lock(stack_lock_path)
    except Exception as exc:
        print(f"suite_survey: stack-lock error: {exc}", file=sys.stderr)
        return 2

    if stack_lock_digest is None:
        print("no stack lock: every surveyed sibling treated as consumed")

    try:
        prev_path = find_previous_survey(out_dir, baseline_path, baseline_explicit)
        prev_doc = None
        if prev_path is not None:
            with open(prev_path, "rb") as f:
                prev_doc = json.loads(f.read().decode("utf-8"))
    except Exception as exc:
        print(f"suite_survey: baseline error: {exc}", file=sys.stderr)
        return 2

    surveyed_entries = [e for e in entries if e["survey"]]
    rows = []
    try:
        for entry in surveyed_entries:
            rows.append(survey_one_sibling(repo_root, entry))
    except Exception as exc:
        print(f"suite_survey: survey error: {exc}", file=sys.stderr)
        return 2

    if date_override:
        if not UTC_ISO_RE.match(date_override):
            print(f"suite_survey: invalid --date (expected YYYY-MM-DDTHH:MM:SSZ): {date_override!r}", file=sys.stderr)
            return 2
        date_iso = date_override
        utc_stem = re.sub(r"[-:]", "", date_iso)
    else:
        now = datetime.datetime.now(datetime.timezone.utc)
        date_iso = now.strftime("%Y-%m-%dT%H:%M:%SZ")
        utc_stem = now.strftime("%Y%m%dT%H%M%SZ")

    record = {
        "schema": "frob.suite-survey/1",
        "date": date_iso,
        "stack_lock": stack_lock_digest,
        "rows": rows,
    }
    record.update(compute_derived(rows))

    payload_bytes = canonical_json(record).encode("utf-8")
    try:
        record_hash = blake3_bytes(payload_bytes)
    except Exception as exc:
        print(f"suite_survey: hash error: {exc}", file=sys.stderr)
        return 2

    os.makedirs(out_dir, exist_ok=True)
    out_file = os.path.join(out_dir, f"{utc_stem}.json")
    seq = 1
    while os.path.exists(out_file):
        out_file = os.path.join(out_dir, f"{utc_stem}-{seq:04d}.json")
        seq += 1
    tmp_file = f"{out_file}.tmp.{os.getpid()}"
    with open(tmp_file, "wb") as f:
        f.write(payload_bytes)
    os.replace(tmp_file, out_file)

    rel_out = os.path.relpath(out_file, repo_root) if os.path.abspath(out_file).startswith(os.path.abspath(repo_root) + os.sep) else out_file

    diff_lines, consumed_changed = diff_surveys(prev_doc, rows, consumed_names)
    for line in diff_lines:
        print(line)

    print(f"RECORD {rel_out} blake3={record_hash}")
    return 3 if consumed_changed else 0


if __name__ == "__main__":
    sys.exit(main())
PY
