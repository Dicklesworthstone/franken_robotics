#!/usr/bin/env python3
"""Independent reference canonicalizer for `frob.design/1` (bead N14a-26).

Written strictly from the format specification (§14.4 seven rules, §34.9, §34.14)
using only the Python 3 standard library plus `blake3_ref.py` and `units.py`.
Shares zero code, zero schema files, and zero unit tables with any Rust crate.

CLI:
  canon.py [--mode ref|utf16-keys|bin64-roundtrip|bin64-exact|no-defaults] [file]
    Reads raw `.fbot` bytes from `file` (or stdin).
    On accept: writes a single JSON line to stdout:
      {"design_id":"d:...","canon_hex":"...","status":"ok"}
      or with `--raw-bytes`: writes raw canonical UTF-8 bytes directly to stdout.
    On refusal: exits 3 and writes JSON to stdout:
      {"detail":"...","reason":"...","status":"refused"}
"""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction
import json
import re
import sys
from typing import Any, Dict, List, Optional, Sequence, Tuple

from blake3_ref import design_id as compute_design_id
from units import (
    UnitError,
    canonicalize_quantity_string,
    parse_exact_magnitude,
    shortest_decimal,
)

MAX_SAFE_INT53 = (1 << 53) - 1  # 9007199254740991
MAX_DEPTH = 64

VALID_MODES = frozenset(
    {"ref", "utf16-keys", "bin64-roundtrip", "bin64-exact", "no-defaults"}
)

VALID_PROVENANCES = frozenset({"designed", "datasheet", "measured", "assumed", "default"})
VALID_JOINT_CLASSES = frozenset({"fixed", "R", "P", "H", "C", "E", "S", "free"})
VALID_REGEN = frozenset({"to-battery", "shunt", "clip"})
VALID_MANIFOLDS = frozenset({"R", "R+", "S1", "S2", "SE3", "inertia-cone", "finite"})
VALID_CONTACT_LAWS = frozenset(
    {"foot-patch", "wheel-rolling", "roller-wheel", "belt", "skid", "pad"}
)
VALID_MIRROR_PLANES = frozenset({"xy", "xz", "yz"})
VALID_RADIAL_AXES = frozenset({"x", "y", "z"})

HEX64_RE = re.compile(r"^b3:[0-9a-f]{64}$")
INT_LEXEME_RE = re.compile(r"^-?(?:0|[1-9]\d*)$")

# Explicit table of the five written-out non-physics defaults (step 4, N14a-26 / BD-054 / BD-055).
DEFAULT_JOINT_ORIGIN: Dict[str, Any] = {
    "rot": {"angle": "0 turn", "axis": [0, 0, 1]},
    "xyz": ["0 m", "0 m", "0 m"],
}
DEFAULT_FRAME_ROT: Dict[str, Any] = {"angle": "0 turn", "axis": [0, 0, 1]}
DEFAULT_BODY_COLLISION = "region"
DEFAULT_BUS_MEMBERS = "*"
DEFAULT_PAYLOAD_REMOVABLE = False
DEFAULT_LEVER_LOCKED = False


class CanonRefusal(Exception):
    """Structured refusal with a closed reason class."""

    def __init__(self, reason: str, detail: str) -> None:
        super().__init__(detail)
        self.reason = reason
        self.detail = detail


@dataclass(frozen=True)
class RawNumber:
    """Preserves the exact JSON number lexeme without touching binary64."""

    lexeme: str


# ============================================================================
# Step 1 & 2: Strict raw-byte JSON parser (UTF-8, no BOM, duplicate-key refusal)
# ============================================================================

class _StrictJsonParser:
    def __init__(self, text: str) -> None:
        self.s = text
        self.n = len(text)
        self.i = 0

    def _skip_ws(self) -> None:
        s = self.s
        n = self.n
        i = self.i
        while i < n and s[i] in " \t\r\n":
            i += 1
        self.i = i

    def parse(self) -> Any:
        self._skip_ws()
        val = self._parse_value(depth=0)
        self._skip_ws()
        if self.i != self.n:
            raise CanonRefusal("invalid-json", f"trailing bytes at char offset {self.i}")
        return val

    def _parse_value(self, depth: int) -> Any:
        if depth > MAX_DEPTH:
            raise CanonRefusal("invalid-json", f"nesting exceeds limit {MAX_DEPTH}")
        self._skip_ws()
        if self.i >= self.n:
            raise CanonRefusal("invalid-json", "unexpected end of input")
        ch = self.s[self.i]
        if ch == "{":
            return self._parse_object(depth + 1)
        if ch == "[":
            return self._parse_array(depth + 1)
        if ch == '"':
            return self._parse_string()
        if ch == "t":
            if self.s[self.i : self.i + 4] == "true":
                self.i += 4
                return True
            raise CanonRefusal("invalid-json", f"invalid literal at {self.i}")
        if ch == "f":
            if self.s[self.i : self.i + 5] == "false":
                self.i += 5
                return False
            raise CanonRefusal("invalid-json", f"invalid literal at {self.i}")
        if ch == "n":
            if self.s[self.i : self.i + 4] == "null":
                self.i += 4
                raise CanonRefusal("null-value", "null is forbidden anywhere in frob.design/1")
            raise CanonRefusal("invalid-json", f"invalid literal at {self.i}")
        if ch == "-" or ("0" <= ch <= "9"):
            return self._parse_number()
        raise CanonRefusal("invalid-json", f"unexpected char {ch!r} at {self.i}")

    def _parse_object(self, depth: int) -> Dict[str, Any]:
        self.i += 1  # '{'
        self._skip_ws()
        obj: Dict[str, Any] = {}
        if self.i < self.n and self.s[self.i] == "}":
            self.i += 1
            return obj
        while True:
            self._skip_ws()
            if self.i >= self.n or self.s[self.i] != '"':
                raise CanonRefusal("invalid-json", f"expected member name string at {self.i}")
            key = self._parse_string()
            if key in obj:
                raise CanonRefusal(
                    "duplicate-key",
                    f"duplicate member name {key!r} after escape decoding at depth {depth}",
                )
            self._skip_ws()
            if self.i >= self.n or self.s[self.i] != ":":
                raise CanonRefusal("invalid-json", f"expected ':' after key {key!r} at {self.i}")
            self.i += 1
            val = self._parse_value(depth)
            obj[key] = val
            self._skip_ws()
            if self.i >= self.n:
                raise CanonRefusal("invalid-json", "unterminated object")
            if self.s[self.i] == ",":
                self.i += 1
                continue
            if self.s[self.i] == "}":
                self.i += 1
                return obj
            raise CanonRefusal("invalid-json", f"expected ',' or '}}' at {self.i}")

    def _parse_array(self, depth: int) -> List[Any]:
        self.i += 1  # '['
        self._skip_ws()
        arr: List[Any] = []
        if self.i < self.n and self.s[self.i] == "]":
            self.i += 1
            return arr
        while True:
            arr.append(self._parse_value(depth))
            self._skip_ws()
            if self.i >= self.n:
                raise CanonRefusal("invalid-json", "unterminated array")
            if self.s[self.i] == ",":
                self.i += 1
                continue
            if self.s[self.i] == "]":
                self.i += 1
                return arr
            raise CanonRefusal("invalid-json", f"expected ',' or ']' at {self.i}")

    def _parse_hex4(self) -> int:
        if self.i + 4 > self.n:
            raise CanonRefusal("invalid-json", "truncated \\uXXXX escape")
        h = self.s[self.i : self.i + 4]
        for c in h:
            if c not in "0123456789abcdefABCDEF":
                raise CanonRefusal("invalid-json", f"non-hex digit in \\u{h}")
        self.i += 4
        return int(h, 16)

    def _parse_string(self) -> str:
        self.i += 1  # opening '"'
        out: List[str] = []
        s = self.s
        n = self.n
        while self.i < n:
            ch = s[self.i]
            if ch == '"':
                self.i += 1
                return "".join(out)
            if ord(ch) < 0x20:
                raise CanonRefusal("invalid-json", f"unescaped control char U+{ord(ch):04X}")
            if ch == "\\":
                self.i += 1
                if self.i >= n:
                    raise CanonRefusal("invalid-json", "truncated escape sequence")
                esc = s[self.i]
                self.i += 1
                if esc == '"':
                    out.append('"')
                elif esc == "\\":
                    out.append("\\")
                elif esc == "/":
                    out.append("/")
                elif esc == "b":
                    out.append("\b")
                elif esc == "f":
                    out.append("\f")
                elif esc == "n":
                    out.append("\n")
                elif esc == "r":
                    out.append("\r")
                elif esc == "t":
                    out.append("\t")
                elif esc == "u":
                    code = self._parse_hex4()
                    if 0xD800 <= code <= 0xDBFF:
                        if self.i + 2 > n or s[self.i : self.i + 2] != "\\u":
                            raise CanonRefusal("invalid-json", "lone high surrogate")
                        self.i += 2
                        low = self._parse_hex4()
                        if not (0xDC00 <= low <= 0xDFFF):
                            raise CanonRefusal("invalid-json", "invalid low surrogate")
                        cp = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00)
                        out.append(chr(cp))
                    elif 0xDC00 <= code <= 0xDFFF:
                        raise CanonRefusal("invalid-json", "lone low surrogate")
                    else:
                        out.append(chr(code))
                else:
                    raise CanonRefusal("invalid-json", f"unknown escape \\{esc}")
            else:
                out.append(ch)
                self.i += 1
        raise CanonRefusal("invalid-json", "unterminated string")

    def _parse_number(self) -> RawNumber:
        start = self.i
        s = self.s
        n = self.n
        if s[self.i] == "-":
            self.i += 1
            if self.i >= n:
                raise CanonRefusal("invalid-json", "bare '-' in number")
        if s[self.i] == "0":
            self.i += 1
            if self.i < n and "0" <= s[self.i] <= "9":
                raise CanonRefusal("invalid-json", "leading zero in number")
        elif "1" <= s[self.i] <= "9":
            while self.i < n and "0" <= s[self.i] <= "9":
                self.i += 1
        else:
            raise CanonRefusal("invalid-json", f"invalid number at {start}")
        if self.i < n and s[self.i] == ".":
            self.i += 1
            if self.i >= n or not ("0" <= s[self.i] <= "9"):
                raise CanonRefusal("invalid-json", f"missing fraction digits at {start}")
            while self.i < n and "0" <= s[self.i] <= "9":
                self.i += 1
        if self.i < n and s[self.i] in "eE":
            self.i += 1
            if self.i < n and s[self.i] in "+-":
                self.i += 1
            if self.i >= n or not ("0" <= s[self.i] <= "9"):
                raise CanonRefusal("invalid-json", f"missing exponent digits at {start}")
            while self.i < n and "0" <= s[self.i] <= "9":
                self.i += 1
        return RawNumber(s[start : self.i])


def parse_raw_json(raw_bytes: bytes) -> Any:
    if raw_bytes.startswith(b"\xef\xbb\xbf"):
        raise CanonRefusal("bom-forbidden", "UTF-8 byte-order mark is forbidden")
    try:
        text = raw_bytes.decode("utf-8", errors="strict")
    except UnicodeDecodeError as exc:
        raise CanonRefusal("invalid-utf8", f"invalid UTF-8: {exc}") from exc
    return _StrictJsonParser(text).parse()


# ============================================================================
# Step 3 & 4: Schema-directed canonicalization of frob.design/1
# ============================================================================

def _expect_obj(val: Any, ctx: str) -> Dict[str, Any]:
    if not isinstance(val, dict):
        raise CanonRefusal("invalid-type", f"{ctx}: expected object")
    return val


def _expect_list(val: Any, ctx: str) -> List[Any]:
    if not isinstance(val, list):
        raise CanonRefusal("invalid-type", f"{ctx}: expected array")
    return val


def _expect_str(val: Any, ctx: str) -> str:
    if not isinstance(val, str):
        raise CanonRefusal("invalid-type", f"{ctx}: expected string")
    return val


def _expect_bool(val: Any, ctx: str) -> bool:
    if not isinstance(val, bool):
        raise CanonRefusal("invalid-type", f"{ctx}: expected boolean")
    return val


def _check_allowed_keys(obj: Dict[str, Any], allowed: frozenset[str], ctx: str) -> None:
    for k in obj:
        if k not in allowed:
            raise CanonRefusal("unknown-field", f"{ctx}: unknown member `{k}`")


def _expect_int53(val: Any, ctx: str) -> int:
    if not isinstance(val, RawNumber):
        raise CanonRefusal("invalid-type", f"{ctx}: expected JSON integer")
    if not INT_LEXEME_RE.match(val.lexeme):
        raise CanonRefusal("bare-number-for-quantity", f"{ctx}: non-integer JSON number `{val.lexeme}`")
    iv = int(val.lexeme)
    if iv == 0:
        return 0
    if abs(iv) > MAX_SAFE_INT53:
        raise CanonRefusal(
            "wide-integer-as-number",
            f"{ctx}: integer {iv} exceeds +/-{MAX_SAFE_INT53}; must travel as string",
        )
    return iv


def _canon_exact_scalar_or_int53(val: Any, ctx: str) -> Any:
    """Axis component or exact scalar: small int as int53, wide int or rational as string."""
    if isinstance(val, RawNumber):
        return _expect_int53(val, ctx)
    if isinstance(val, str):
        try:
            frac = parse_exact_magnitude(val)
        except UnitError as exc:
            raise CanonRefusal(exc.reason_class, f"{ctx}: {exc.detail}") from exc
        if frac.denominator == 1:
            return str(frac.numerator)
        return f"{frac.numerator}/{frac.denominator}"
    raise CanonRefusal("invalid-type", f"{ctx}: expected integer or exact rational string")


def _canon_rational_str(val: Any, ctx: str) -> str:
    """Exact rational or wide integer carried strictly as a string (Rule 5)."""
    if isinstance(val, RawNumber):
        raise CanonRefusal("bare-number-for-quantity", f"{ctx}: rational/wide value must be string")
    s = _expect_str(val, ctx)
    try:
        frac = parse_exact_magnitude(s)
    except UnitError as exc:
        raise CanonRefusal(exc.reason_class, f"{ctx}: {exc.detail}") from exc
    return shortest_decimal(frac)


def _canon_qty(
    val: Any,
    ctx: str,
    expected_dim: Optional[str] = None,
    *,
    allow_dimensionless: bool = False,
    temp_is_diff: bool = False,
    mode: str = "ref",
) -> Any:
    """Canonicalize a bare quantity string or annotated quantity object (Rules 2-4)."""
    if isinstance(val, RawNumber):
        raise CanonRefusal(
            "bare-number-for-quantity",
            f"{ctx}: bare JSON number `{val.lexeme}` where a quantity string is required",
        )
    if isinstance(val, str):
        try:
            canon_s, _ = canonicalize_quantity_string(
                val,
                expected_dim,
                allow_dimensionless=allow_dimensionless,
                temp_is_diff=temp_is_diff,
                mode=mode,
            )
            return canon_s
        except UnitError as exc:
            raise CanonRefusal(exc.reason_class, f"{ctx}: {exc.detail}") from exc
    if isinstance(val, dict):
        _check_allowed_keys(
            val, frozenset({"value", "provenance", "uncertainty", "source"}), ctx
        )
        if "value" not in val:
            raise CanonRefusal("missing-field", f"{ctx}: annotated quantity missing `value`")
        raw_v = val["value"]
        if isinstance(raw_v, RawNumber):
            raise CanonRefusal(
                "bare-number-for-quantity",
                f"{ctx}.value: bare JSON number `{raw_v.lexeme}` where quantity string required",
            )
        val_str = _expect_str(raw_v, f"{ctx}.value")
        try:
            canon_v, dim = canonicalize_quantity_string(
                val_str,
                expected_dim,
                allow_dimensionless=allow_dimensionless,
                temp_is_diff=temp_is_diff,
                mode=mode,
            )
        except UnitError as exc:
            raise CanonRefusal(exc.reason_class, f"{ctx}.value: {exc.detail}") from exc

        prov = "designed"
        if "provenance" in val:
            prov = _expect_str(val["provenance"], f"{ctx}.provenance")
            if prov not in VALID_PROVENANCES:
                raise CanonRefusal(
                    "invalid-value", f"{ctx}.provenance: unknown provenance `{prov}`"
                )

        unc_canon: Optional[str] = None
        if "uncertainty" in val:
            raw_u = val["uncertainty"]
            if isinstance(raw_u, RawNumber):
                raise CanonRefusal(
                    "bare-number-for-quantity",
                    f"{ctx}.uncertainty: bare number where quantity string required",
                )
            u_str = _expect_str(raw_u, f"{ctx}.uncertainty")
            try:
                unc_canon, _ = canonicalize_quantity_string(
                    u_str,
                    dim if dim != "dimensionless" else None,
                    allow_dimensionless=(dim == "dimensionless"),
                    temp_is_diff=(dim == "temperature"),
                    mode=mode,
                )
            except UnitError as exc:
                raise CanonRefusal(exc.reason_class, f"{ctx}.uncertainty: {exc.detail}") from exc

        src: Optional[str] = None
        if "source" in val:
            src = _expect_str(val["source"], f"{ctx}.source")

        # Rule 2 / Step 3: annotated quantity with provenance `designed` and nothing else collapses to bare string.
        if prov == "designed" and unc_canon is None and src is None:
            return canon_v

        out: Dict[str, Any] = {"provenance": prov, "value": canon_v}
        if unc_canon is not None:
            out["uncertainty"] = unc_canon
        if src is not None:
            out["source"] = src
        return out

    raise CanonRefusal("invalid-type", f"{ctx}: expected quantity string or annotated quantity object")


def _canon_vec3_qty(val: Any, ctx: str, dim: str, mode: str) -> List[Any]:
    arr = _expect_list(val, ctx)
    if len(arr) != 3:
        raise CanonRefusal("invalid-type", f"{ctx}: expected 3-element quantity array")
    return [_canon_qty(x, f"{ctx}[{i}]", dim, mode=mode) for i, x in enumerate(arr)]


def _canon_vec3_axis(val: Any, ctx: str) -> List[Any]:
    arr = _expect_list(val, ctx)
    if len(arr) != 3:
        raise CanonRefusal("invalid-type", f"{ctx}: expected 3-element axis array")
    return [_canon_exact_scalar_or_int53(x, f"{ctx}[{i}]") for i, x in enumerate(arr)]


def _canon_frame(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(obj, frozenset({"xyz", "rot"}), ctx)
    out: Dict[str, Any] = {}
    if "xyz" in obj:
        out["xyz"] = _canon_vec3_qty(obj["xyz"], f"{ctx}.xyz", "length", mode)
    else:
        out["xyz"] = ["0 m", "0 m", "0 m"]
    if "rot" in obj:
        rot = _expect_obj(obj["rot"], f"{ctx}.rot")
        _check_allowed_keys(rot, frozenset({"axis", "angle"}), f"{ctx}.rot")
        out["rot"] = {
            "axis": _canon_vec3_axis(rot["axis"], f"{ctx}.rot.axis"),
            "angle": _canon_qty(rot["angle"], f"{ctx}.rot.angle", "angle", mode=mode),
        }
    elif mode != "no-defaults":
        out["rot"] = {
            "angle": DEFAULT_FRAME_ROT["angle"],
            "axis": list(DEFAULT_FRAME_ROT["axis"]),
        }
    return out


def _canon_region(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    if "mesh" in obj:
        _check_allowed_keys(obj, frozenset({"mesh", "unit"}), ctx)
        mesh_h = _expect_str(obj["mesh"], f"{ctx}.mesh")
        if not HEX64_RE.match(mesh_h):
            raise CanonRefusal("invalid-hash", f"{ctx}.mesh: expected b3:<64 hex>")
        out: Dict[str, Any] = {"mesh": mesh_h}
        if "unit" in obj:
            out["unit"] = _expect_str(obj["unit"], f"{ctx}.unit")
        return out
    if len(obj) != 1:
        raise CanonRefusal("unknown-field", f"{ctx}: region must have exactly one shape key")
    key, inner = next(iter(obj.items()))
    if key == "box":
        return {"box": _canon_vec3_qty(inner, f"{ctx}.box", "length", mode)}
    if key == "ellipsoid":
        return {"ellipsoid": _canon_vec3_qty(inner, f"{ctx}.ellipsoid", "length", mode)}
    if key in ("cylinder", "capsule"):
        arr = _expect_list(inner, f"{ctx}.{key}")
        if len(arr) != 2:
            raise CanonRefusal("invalid-type", f"{ctx}.{key}: expected [radius, length]")
        return {
            key: [
                _canon_qty(arr[0], f"{ctx}.{key}[0]", "length", mode=mode),
                _canon_qty(arr[1], f"{ctx}.{key}[1]", "length", mode=mode),
            ]
        }
    if key == "sphere":
        return {"sphere": _canon_qty(inner, f"{ctx}.sphere", "length", mode=mode)}
    if key in ("union", "difference"):
        arr = _expect_list(inner, f"{ctx}.{key}")
        placed = []
        for idx, item in enumerate(arr):
            p = _expect_obj(item, f"{ctx}.{key}[{idx}]")
            _check_allowed_keys(p, frozenset({"of", "at"}), f"{ctx}.{key}[{idx}]")
            po: Dict[str, Any] = {"of": _canon_region(p["of"], f"{ctx}.{key}[{idx}].of", mode)}
            if "at" in p:
                po["at"] = _canon_frame(p["at"], f"{ctx}.{key}[{idx}].at", mode)
            placed.append(po)
        return {key: placed}
    raise CanonRefusal("unknown-field", f"{ctx}: unknown region shape `{key}`")


def _canon_fill(val: Any, ctx: str, mode: str) -> Any:
    if isinstance(val, str):
        if val not in ("solid", "declared", "massless"):
            raise CanonRefusal("unknown-field", f"{ctx}: unknown fill `{val}`")
        return val
    obj = _expect_obj(val, ctx)
    if len(obj) != 1:
        raise CanonRefusal("unknown-field", f"{ctx}: fill object must have one key")
    k, v = next(iter(obj.items()))
    if k == "shell":
        return {"shell": _canon_qty(v, f"{ctx}.shell", "length", mode=mode)}
    if k == "lattice":
        return {"lattice": _canon_qty(v, f"{ctx}.lattice", None, allow_dimensionless=True, mode=mode)}
    if k == "solid":
        inner = _expect_obj(v, f"{ctx}.solid")
        _check_allowed_keys(inner, frozenset({"intended"}), f"{ctx}.solid")
        if _expect_bool(inner.get("intended"), f"{ctx}.solid.intended") is not True:
            raise CanonRefusal("invalid-value", f"{ctx}.solid.intended must be true")
        return {"solid": {"intended": True}}
    raise CanonRefusal("unknown-field", f"{ctx}: unknown fill key `{k}`")


def _canon_body(
    val: Any,
    ctx: str,
    mode: str,
    *,
    extra_allowed: frozenset[str] = frozenset(),
    is_payload: bool = False,
    is_end_effector: bool = False,
) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    allowed = frozenset(
        {"body", "region", "fill", "material", "mass_properties", "collision", "visual", "ports"}
    ) | extra_allowed
    _check_allowed_keys(obj, allowed, ctx)

    out: Dict[str, Any] = {}
    if "body" in obj:
        out["body"] = _expect_str(obj["body"], f"{ctx}.body")
    if "region" in obj:
        out["region"] = _canon_region(obj["region"], f"{ctx}.region", mode)
    if "fill" in obj:
        out["fill"] = _canon_fill(obj["fill"], f"{ctx}.fill", mode)
    if "material" in obj:
        out["material"] = _expect_str(obj["material"], f"{ctx}.material")
    if "mass_properties" in obj:
        mp = _expect_obj(obj["mass_properties"], f"{ctx}.mass_properties")
        _check_allowed_keys(mp, frozenset({"mass", "com", "inertia"}), f"{ctx}.mass_properties")
        mp_out: Dict[str, Any] = {}
        if "mass" in mp:
            mp_out["mass"] = _canon_qty(mp["mass"], f"{ctx}.mass_properties.mass", "mass", mode=mode)
        if "com" in mp:
            mp_out["com"] = _canon_vec3_qty(mp["com"], f"{ctx}.mass_properties.com", "length", mode)
        if "inertia" in mp:
            inert = _expect_obj(mp["inertia"], f"{ctx}.mass_properties.inertia")
            _check_allowed_keys(
                inert, frozenset({"xx", "yy", "zz", "xy", "xz", "yz"}), f"{ctx}.mass_properties.inertia"
            )
            mp_out["inertia"] = {
                ik: _canon_qty(iv, f"{ctx}.mass_properties.inertia.{ik}", "inertia", mode=mode)
                for ik, iv in inert.items()
            }
        out["mass_properties"] = mp_out
    if "collision" in obj:
        c = obj["collision"]
        if isinstance(c, str):
            if c != "region":
                raise CanonRefusal("invalid-value", f"{ctx}.collision: unknown token `{c}`")
            out["collision"] = "region"
        else:
            out["collision"] = _canon_region(c, f"{ctx}.collision", mode)
    elif mode != "no-defaults":
        out["collision"] = DEFAULT_BODY_COLLISION

    if "visual" in obj:
        vis = _expect_str(obj["visual"], f"{ctx}.visual")
        if not HEX64_RE.match(vis):
            raise CanonRefusal("invalid-hash", f"{ctx}.visual: expected b3:<64 hex>")
        out["visual"] = vis

    if "ports" in obj:
        ports = _expect_obj(obj["ports"], f"{ctx}.ports")
        out["ports"] = {
            pname: _canon_frame(pframe, f"{ctx}.ports.{pname}", mode)
            for pname, pframe in ports.items()
        }

    if is_end_effector and "contact" in obj:
        out["contact"] = _canon_contact(obj["contact"], f"{ctx}.contact", mode)

    if is_payload:
        if "mount" in obj:
            if isinstance(obj["mount"], str):
                out["mount"] = obj["mount"]
            else:
                out["mount"] = _canon_frame(obj["mount"], f"{ctx}.mount", mode)
        if "removable" in obj:
            out["removable"] = _expect_bool(obj["removable"], f"{ctx}.removable")
        elif mode != "no-defaults":
            out["removable"] = DEFAULT_PAYLOAD_REMOVABLE

    return out


def _canon_contact(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(
        obj,
        frozenset({"law", "interface", "torsion_radius", "roller_angle", "run_length", "drive"}),
        ctx,
    )
    law = _expect_str(obj.get("law"), f"{ctx}.law")
    if law not in VALID_CONTACT_LAWS:
        raise CanonRefusal("unknown-field", f"{ctx}.law: unknown contact law `{law}`")
    out: Dict[str, Any] = {"law": law}
    if "interface" in obj:
        out["interface"] = _expect_str(obj["interface"], f"{ctx}.interface")
    if "torsion_radius" in obj:
        out["torsion_radius"] = _canon_qty(
            obj["torsion_radius"], f"{ctx}.torsion_radius", "length", mode=mode
        )
    if "roller_angle" in obj:
        out["roller_angle"] = _canon_qty(
            obj["roller_angle"], f"{ctx}.roller_angle", "angle", mode=mode
        )
    if "run_length" in obj:
        out["run_length"] = _canon_qty(obj["run_length"], f"{ctx}.run_length", "length", mode=mode)
    if "drive" in obj:
        out["drive"] = _expect_str(obj["drive"], f"{ctx}.drive")
    return out


def _canon_joint(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(
        obj,
        frozenset(
            {"class", "axis", "pitch", "screws", "origin", "limits", "home", "damping", "friction", "backlash"}
        ),
        ctx,
    )
    jclass = _expect_str(obj.get("class"), f"{ctx}.class")
    if jclass not in VALID_JOINT_CLASSES:
        raise CanonRefusal("invalid-value", f"{ctx}.class: unknown joint class `{jclass}`")
    out: Dict[str, Any] = {"class": jclass}
    if "axis" in obj:
        out["axis"] = _canon_vec3_axis(obj["axis"], f"{ctx}.axis")
    if "pitch" in obj:
        out["pitch"] = _canon_qty(obj["pitch"], f"{ctx}.pitch", "length", mode=mode)
    if "screws" in obj:
        screws = _expect_list(obj["screws"], f"{ctx}.screws")
        s_out = []
        for idx, sc in enumerate(screws):
            so = _expect_obj(sc, f"{ctx}.screws[{idx}]")
            _check_allowed_keys(
                so, frozenset({"axis", "point", "pitch", "translation"}), f"{ctx}.screws[{idx}]"
            )
            entry: Dict[str, Any] = {}
            if "axis" in so:
                entry["axis"] = _canon_vec3_axis(so["axis"], f"{ctx}.screws[{idx}].axis")
            if "point" in so:
                entry["point"] = _canon_vec3_qty(so["point"], f"{ctx}.screws[{idx}].point", "length", mode)
            if "pitch" in so:
                entry["pitch"] = _canon_qty(so["pitch"], f"{ctx}.screws[{idx}].pitch", "length", mode=mode)
            if "translation" in so:
                entry["translation"] = _canon_vec3_axis(
                    so["translation"], f"{ctx}.screws[{idx}].translation"
                )
            s_out.append(entry)
        out["screws"] = s_out
    if "origin" in obj:
        out["origin"] = _canon_frame(obj["origin"], f"{ctx}.origin", mode)
    elif mode != "no-defaults":
        out["origin"] = {
            "rot": {
                "angle": DEFAULT_JOINT_ORIGIN["rot"]["angle"],
                "axis": list(DEFAULT_JOINT_ORIGIN["rot"]["axis"]),
            },
            "xyz": list(DEFAULT_JOINT_ORIGIN["xyz"]),
        }
    if "limits" in obj:
        lim = obj["limits"]
        if isinstance(lim, str):
            if lim != "unbounded":
                raise CanonRefusal("invalid-value", f"{ctx}.limits: unknown token `{lim}`")
            out["limits"] = "unbounded"
        elif isinstance(lim, dict):
            _check_allowed_keys(lim, frozenset({"swing", "twist"}), f"{ctx}.limits")
            lo_out: Dict[str, Any] = {}
            if "swing" in lim:
                lo_out["swing"] = _canon_qty(lim["swing"], f"{ctx}.limits.swing", "angle", mode=mode)
            if "twist" in lim:
                tw = _expect_list(lim["twist"], f"{ctx}.limits.twist")
                lo_out["twist"] = [
                    _canon_qty(t, f"{ctx}.limits.twist[{i}]", "angle", mode=mode)
                    for i, t in enumerate(tw)
                ]
            out["limits"] = lo_out
        else:
            arr = _expect_list(lim, f"{ctx}.limits")
            lim_dim = "length" if jclass == "P" else ("angle" if jclass in ("R", "H") else None)
            out["limits"] = [
                _canon_qty(x, f"{ctx}.limits[{i}]", lim_dim, mode=mode)
                for i, x in enumerate(arr)
            ]
    if "home" in obj:
        h = obj["home"]
        if isinstance(h, list):
            out["home"] = [_canon_qty(x, f"{ctx}.home[{i}]", None, mode=mode) for i, x in enumerate(h)]
        else:
            out["home"] = _canon_qty(h, f"{ctx}.home", None, mode=mode)
    for opt in ("damping", "friction", "backlash"):
        if opt in obj:
            out[opt] = _canon_qty(obj[opt], f"{ctx}.{opt}", None, mode=mode)
    return out


def _canon_transmission(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    if len(obj) != 1:
        raise CanonRefusal("unknown-field", f"{ctx}: transmission must have one kind key")
    kind, inner = next(iter(obj.items()))
    if kind not in ("gear", "belt", "screw", "linkage", "cable", "coupling"):
        raise CanonRefusal("unknown-field", f"{ctx}: unknown transmission kind `{kind}`")
    t_obj = _expect_obj(inner, f"{ctx}.{kind}")
    _check_allowed_keys(
        t_obj,
        frozenset({"ratio", "efficiency", "backlash", "compliance", "joints", "matrix"}),
        f"{ctx}.{kind}",
    )
    t_out: Dict[str, Any] = {}
    if "ratio" in t_obj:
        t_out["ratio"] = _canon_qty(
            t_obj["ratio"], f"{ctx}.{kind}.ratio", None, allow_dimensionless=True, mode=mode
        )
    if "efficiency" in t_obj:
        eff = t_obj["efficiency"]
        if isinstance(eff, list):
            t_out["efficiency"] = [
                _canon_qty(x, f"{ctx}.{kind}.efficiency[{i}]", None, allow_dimensionless=True, mode=mode)
                for i, x in enumerate(eff)
            ]
        else:
            t_out["efficiency"] = _canon_qty(
                eff, f"{ctx}.{kind}.efficiency", None, allow_dimensionless=True, mode=mode
            )
    if "backlash" in t_obj:
        t_out["backlash"] = _canon_qty(t_obj["backlash"], f"{ctx}.{kind}.backlash", None, mode=mode)
    if "compliance" in t_obj:
        t_out["compliance"] = _canon_qty(t_obj["compliance"], f"{ctx}.{kind}.compliance", None, mode=mode)
    if "joints" in t_obj:
        t_out["joints"] = [_expect_str(j, f"{ctx}.{kind}.joints[{i}]") for i, j in enumerate(_expect_list(t_obj["joints"], f"{ctx}.{kind}.joints"))]
    if "matrix" in t_obj:
        mat = _expect_list(t_obj["matrix"], f"{ctx}.{kind}.matrix")
        t_out["matrix"] = [
            [_canon_rational_str(c, f"{ctx}.{kind}.matrix[{r}][{ci}]") for ci, c in enumerate(_expect_list(row, f"{ctx}.{kind}.matrix[{r}]"))]
            for r, row in enumerate(mat)
        ]
    return {kind: t_out}


def _canon_actuator(val: Any, ctx: str, mode: str) -> Any:
    if isinstance(val, str):
        return val
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(obj, frozenset({"card", "override"}), ctx)
    out: Dict[str, Any] = {"card": _expect_str(obj["card"], f"{ctx}.card")}
    if "override" in obj:
        ov = _expect_obj(obj["override"], f"{ctx}.override")
        out["override"] = {
            ok: _canon_qty(ov_v, f"{ctx}.override.{ok}", None, allow_dimensionless=True, mode=mode)
            for ok, ov_v in ov.items()
        }
    return out


def _canon_link(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(
        obj,
        frozenset({"joint", "actuator", "transmission", "body", "foot", "wheel", "track"}),
        ctx,
    )
    out: Dict[str, Any] = {}
    if "joint" in obj:
        out["joint"] = _canon_joint(obj["joint"], f"{ctx}.joint", mode)
    if "actuator" in obj:
        out["actuator"] = _canon_actuator(obj["actuator"], f"{ctx}.actuator", mode)
    if "transmission" in obj:
        out["transmission"] = _canon_transmission(obj["transmission"], f"{ctx}.transmission", mode)
    if "body" in obj:
        out["body"] = _canon_body(obj["body"], f"{ctx}.body", mode)
    for ee in ("foot", "wheel", "track"):
        if ee in obj:
            out[ee] = _canon_body(
                obj[ee],
                f"{ctx}.{ee}",
                mode,
                extra_allowed=frozenset({"contact"}),
                is_end_effector=True,
            )
    return out


def _canon_constructor(val: Any, ctx: str, mode: str) -> Any:
    """Canonicalize constructor terms while keeping them strictly UNEXPANDED (Rule 7)."""
    if isinstance(val, list):
        return [_canon_constructor(item, f"{ctx}[{i}]", mode) for i, item in enumerate(val)]
    obj = _expect_obj(val, ctx)
    out: Dict[str, Any] = {}

    if "mirror" in obj:
        _check_allowed_keys(obj, frozenset({"mirror", "of"}), ctx)
        m = obj["mirror"]
        if isinstance(m, str):
            if m not in VALID_MIRROR_PLANES:
                raise CanonRefusal("invalid-value", f"{ctx}.mirror: unknown plane `{m}`")
            out["mirror"] = m
        else:
            mo = _expect_obj(m, f"{ctx}.mirror")
            _check_allowed_keys(mo, frozenset({"normal", "offset"}), f"{ctx}.mirror")
            out["mirror"] = {
                "normal": _canon_vec3_axis(mo["normal"], f"{ctx}.mirror.normal"),
                "offset": _canon_qty(mo["offset"], f"{ctx}.mirror.offset", "length", mode=mode),
            }
        out["of"] = _canon_constructor(obj["of"], f"{ctx}.of", mode)
        return out

    if "radial" in obj:
        _check_allowed_keys(obj, frozenset({"radial", "axis", "of"}), ctx)
        out["radial"] = _expect_int53(obj["radial"], f"{ctx}.radial")
        ax = obj["axis"]
        if isinstance(ax, str):
            if ax not in VALID_RADIAL_AXES:
                raise CanonRefusal("invalid-value", f"{ctx}.axis: unknown axis `{ax}`")
            out["axis"] = ax
        else:
            axo = _expect_obj(ax, f"{ctx}.axis")
            _check_allowed_keys(axo, frozenset({"dir", "point"}), f"{ctx}.axis")
            out["axis"] = {
                "dir": _canon_vec3_axis(axo["dir"], f"{ctx}.axis.dir"),
                "point": _canon_vec3_qty(axo["point"], f"{ctx}.axis.point", "length", mode),
            }
        out["of"] = _canon_constructor(obj["of"], f"{ctx}.of", mode)
        return out

    if "serial" in obj:
        _check_allowed_keys(obj, frozenset({"serial", "of"}), ctx)
        out["serial"] = _expect_int53(obj["serial"], f"{ctx}.serial")
        of_val = obj["of"]
        if isinstance(of_val, dict) and any(
            k in of_val for k in ("joint", "body", "foot", "wheel", "track")
        ):
            out["of"] = _canon_link(of_val, f"{ctx}.of", mode)
        else:
            out["of"] = _canon_constructor(of_val, f"{ctx}.of", mode)
        return out

    if "scale" in obj:
        _check_allowed_keys(obj, frozenset({"scale", "of"}), ctx)
        out["scale"] = _canon_qty(
            obj["scale"], f"{ctx}.scale", None, allow_dimensionless=True, mode=mode
        )
        out["of"] = _canon_constructor(obj["of"], f"{ctx}.of", mode)
        return out

    if "replace" in obj:
        _check_allowed_keys(obj, frozenset({"replace", "with", "of"}), ctx)
        out["replace"] = _expect_str(obj["replace"], f"{ctx}.replace")
        out["with"] = _canon_link(obj["with"], f"{ctx}.with", mode)
        out["of"] = _canon_constructor(obj["of"], f"{ctx}.of", mode)
        return out

    if "closure" in obj:
        _check_allowed_keys(obj, frozenset({"closure", "joint"}), ctx)
        pair = _expect_list(obj["closure"], f"{ctx}.closure")
        if len(pair) != 2:
            raise CanonRefusal("invalid-type", f"{ctx}.closure: expected 2 port refs")
        out["closure"] = [_expect_str(pair[0], f"{ctx}.closure[0]"), _expect_str(pair[1], f"{ctx}.closure[1]")]
        out["joint"] = _canon_joint(obj["joint"], f"{ctx}.joint", mode)
        return out

    if "use" in obj:
        _check_allowed_keys(obj, frozenset({"use", "module", "at", "bind", "rename"}), ctx)
        if isinstance(obj["use"], str):
            out["use"] = obj["use"]
        else:
            uo = _expect_obj(obj["use"], f"{ctx}.use")
            _check_allowed_keys(uo, frozenset({"module", "at", "bind", "rename"}), f"{ctx}.use")
            inner_u: Dict[str, Any] = {}
            if "module" in uo:
                inner_u["module"] = _expect_str(uo["module"], f"{ctx}.use.module")
            if "at" in uo:
                inner_u["at"] = _expect_str(uo["at"], f"{ctx}.use.at")
            if "bind" in uo:
                b_obj = _expect_obj(uo["bind"], f"{ctx}.use.bind")
                inner_u["bind"] = {
                    bk: _canon_bind_val(bv, f"{ctx}.use.bind.{bk}", mode)
                    for bk, bv in b_obj.items()
                }
            if "rename" in uo:
                r_obj = _expect_obj(uo["rename"], f"{ctx}.use.rename")
                inner_u["rename"] = {rk: _expect_str(rv, f"{ctx}.use.rename.{rk}") for rk, rv in r_obj.items()}
            out["use"] = inner_u
        if "module" in obj:
            out["module"] = _expect_str(obj["module"], f"{ctx}.module")
        if "at" in obj:
            out["at"] = _expect_str(obj["at"], f"{ctx}.at")
        if "bind" in obj:
            b_obj = _expect_obj(obj["bind"], f"{ctx}.bind")
            out["bind"] = {
                bk: _canon_bind_val(bv, f"{ctx}.bind.{bk}", mode) for bk, bv in b_obj.items()
            }
        if "rename" in obj:
            r_obj = _expect_obj(obj["rename"], f"{ctx}.rename")
            out["rename"] = {rk: _expect_str(rv, f"{ctx}.rename.{rk}") for rk, rv in r_obj.items()}
        return out

    if "attach" in obj or "chain" in obj:
        _check_allowed_keys(obj, frozenset({"attach", "chain", "serial", "of", "use"}), ctx)
        if "attach" in obj:
            out["attach"] = _expect_str(obj["attach"], f"{ctx}.attach")
        if "chain" in obj:
            ch = _expect_list(obj["chain"], f"{ctx}.chain")
            out["chain"] = [_canon_link(lnk, f"{ctx}.chain[{i}]", mode) for i, lnk in enumerate(ch)]
        if "of" in obj:
            out["of"] = _canon_constructor(obj["of"], f"{ctx}.of", mode)
        return out

    raise CanonRefusal("unknown-field", f"{ctx}: unknown constructor form with keys {sorted(obj.keys())}")


def _canon_bind_val(val: Any, ctx: str, mode: str) -> Any:
    if isinstance(val, RawNumber):
        raise CanonRefusal("bare-number-for-quantity", f"{ctx}: bare number in binding")
    if isinstance(val, dict):
        return _canon_qty(val, ctx, None, allow_dimensionless=True, mode=mode)
    s = _expect_str(val, ctx)
    # Try quantity if it contains a space (e.g. "210 mm"); otherwise keep identifier string (e.g. "qdd-l").
    if " " in s:
        return _canon_qty(s, ctx, None, allow_dimensionless=False, mode=mode)
    return s


def _canon_power_bus_one(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(
        obj,
        frozenset(
            {
                "id",
                "battery",
                "nominal",
                "current_max",
                "regen",
                "harness_resistance",
                "brownout",
                "sink",
                "members",
            }
        ),
        ctx,
    )
    out: Dict[str, Any] = {}
    if "id" in obj:
        out["id"] = _expect_str(obj["id"], f"{ctx}.id")
    if "battery" in obj:
        out["battery"] = _expect_str(obj["battery"], f"{ctx}.battery")
    if "nominal" in obj:
        out["nominal"] = _canon_qty(obj["nominal"], f"{ctx}.nominal", "voltage", mode=mode)
    if "current_max" in obj:
        out["current_max"] = _canon_qty(obj["current_max"], f"{ctx}.current_max", "current", mode=mode)
    if "regen" in obj:
        reg = _expect_str(obj["regen"], f"{ctx}.regen")
        if reg not in VALID_REGEN:
            raise CanonRefusal("invalid-value", f"{ctx}.regen: unknown regen `{reg}`")
        out["regen"] = reg
    if "harness_resistance" in obj:
        out["harness_resistance"] = _canon_qty(
            obj["harness_resistance"], f"{ctx}.harness_resistance", "resistance", mode=mode
        )
    if "brownout" in obj:
        out["brownout"] = _canon_qty(obj["brownout"], f"{ctx}.brownout", "voltage", mode=mode)
    if "sink" in obj:
        sk = _expect_obj(obj["sink"], f"{ctx}.sink")
        _check_allowed_keys(sk, frozenset({"conductance"}), f"{ctx}.sink")
        out["sink"] = {
            "conductance": _canon_qty(
                sk["conductance"], f"{ctx}.sink.conductance", "thermal_conductance", mode=mode
            )
        }
    if "members" in obj:
        m = obj["members"]
        if isinstance(m, str):
            out["members"] = m
        else:
            out["members"] = [_expect_str(x, f"{ctx}.members[{i}]") for i, x in enumerate(_expect_list(m, f"{ctx}.members"))]
    elif mode != "no-defaults":
        out["members"] = DEFAULT_BUS_MEMBERS
    return out


def _canon_sensor(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(
        obj, frozenset({"id", "card", "parent", "mount", "rate", "clock", "fov", "latency"}), ctx
    )
    out: Dict[str, Any] = {}
    for sk in ("id", "card", "parent", "clock"):
        if sk in obj:
            out[sk] = _expect_str(obj[sk], f"{ctx}.{sk}")
    if "mount" in obj:
        out["mount"] = _canon_frame(obj["mount"], f"{ctx}.mount", mode)
    if "rate" in obj:
        out["rate"] = _canon_qty(obj["rate"], f"{ctx}.rate", "frequency", mode=mode)
    if "fov" in obj:
        fov = obj["fov"]
        if isinstance(fov, list):
            out["fov"] = [_canon_qty(x, f"{ctx}.fov[{i}]", "angle", mode=mode) for i, x in enumerate(fov)]
        else:
            out["fov"] = _canon_qty(fov, f"{ctx}.fov", "angle", mode=mode)
    if "latency" in obj:
        out["latency"] = _canon_qty(obj["latency"], f"{ctx}.latency", "time", mode=mode)
    return out


def _canon_budgets(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(
        obj,
        frozenset({"mass_max", "cost_max", "power_peak_max", "energy_min", "envelope_max", "payload_min"}),
        ctx,
    )
    out: Dict[str, Any] = {}
    if "mass_max" in obj:
        out["mass_max"] = _canon_qty(obj["mass_max"], f"{ctx}.mass_max", "mass", mode=mode)
    if "cost_max" in obj:
        out["cost_max"] = _canon_qty(obj["cost_max"], f"{ctx}.cost_max", "currency", mode=mode)
    if "power_peak_max" in obj:
        out["power_peak_max"] = _canon_qty(
            obj["power_peak_max"], f"{ctx}.power_peak_max", "power", mode=mode
        )
    if "energy_min" in obj:
        out["energy_min"] = _canon_qty(obj["energy_min"], f"{ctx}.energy_min", "energy", mode=mode)
    if "envelope_max" in obj:
        out["envelope_max"] = _canon_vec3_qty(obj["envelope_max"], f"{ctx}.envelope_max", "length", mode)
    if "payload_min" in obj:
        out["payload_min"] = _canon_qty(obj["payload_min"], f"{ctx}.payload_min", "mass", mode=mode)
    return out


def _canon_lever(val: Any, ctx: str, mode: str) -> Dict[str, Any]:
    obj = _expect_obj(val, ctx)
    _check_allowed_keys(
        obj, frozenset({"id", "path", "bounds", "choices", "step", "manifold", "prior", "locked"}), ctx
    )
    out: Dict[str, Any] = {}
    for sk in ("id", "path"):
        if sk in obj:
            out[sk] = _expect_str(obj[sk], f"{ctx}.{sk}")
    if "bounds" in obj:
        b = _expect_list(obj["bounds"], f"{ctx}.bounds")
        if len(b) != 2:
            raise CanonRefusal("invalid-type", f"{ctx}.bounds: expected [lo, hi]")
        out["bounds"] = [
            _canon_qty(b[0], f"{ctx}.bounds[0]", None, allow_dimensionless=True, mode=mode),
            _canon_qty(b[1], f"{ctx}.bounds[1]", None, allow_dimensionless=True, mode=mode),
        ]
    if "choices" in obj:
        out["choices"] = [
            _expect_str(c, f"{ctx}.choices[{i}]")
            for i, c in enumerate(_expect_list(obj["choices"], f"{ctx}.choices"))
        ]
    if "step" in obj:
        out["step"] = _canon_qty(obj["step"], f"{ctx}.step", None, allow_dimensionless=True, mode=mode)
    if "manifold" in obj:
        mf = _expect_str(obj["manifold"], f"{ctx}.manifold")
        if mf not in VALID_MANIFOLDS:
            raise CanonRefusal("invalid-value", f"{ctx}.manifold: unknown manifold `{mf}`")
        out["manifold"] = mf
    if "prior" in obj:
        out["prior"] = _canon_generic_strict(obj["prior"], f"{ctx}.prior", mode)
    if "locked" in obj:
        out["locked"] = _expect_bool(obj["locked"], f"{ctx}.locked")
    elif mode != "no-defaults":
        out["locked"] = DEFAULT_LEVER_LOCKED
    return out


def _canon_generic_strict(val: Any, ctx: str, mode: str) -> Any:
    """Generic strict canonicalizer for structured metadata subblocks (`init`, `reduced`, `prior`)."""
    if isinstance(val, bool) or isinstance(val, str):
        return val
    if isinstance(val, RawNumber):
        return _expect_int53(val, ctx)
    if isinstance(val, list):
        return [_canon_generic_strict(x, f"{ctx}[{i}]", mode) for i, x in enumerate(val)]
    if isinstance(val, dict):
        return {k: _canon_generic_strict(v, f"{ctx}.{k}", mode) for k, v in val.items()}
    raise CanonRefusal("invalid-type", f"{ctx}: unsupported value")


def canonicalize_document(doc: Any, mode: str = "ref") -> Dict[str, Any]:
    if mode not in VALID_MODES:
        raise CanonRefusal("invalid-mode", f"unknown mode `{mode}`")
    root = _expect_obj(doc, "/")
    allowed_top = frozenset(
        {
            "schema",
            "name",
            "meta",
            "root",
            "limbs",
            "power_bus",
            "sensors",
            "payloads",
            "budgets",
            "levers",
            "locks",
            "symmetry",
            "reduced",
            "init",
        }
    )
    _check_allowed_keys(root, allowed_top, "/")

    if "schema" not in root:
        raise CanonRefusal("unsupported-schema", "missing required `schema`")
    schema = _expect_str(root["schema"], "/schema")
    if schema != "frob.design/1":
        raise CanonRefusal("unsupported-schema", f"expected `frob.design/1`, got `{schema}`")
    if "name" not in root:
        raise CanonRefusal("missing-field", "missing required `name`")

    out: Dict[str, Any] = {
        "schema": schema,
        "name": _expect_str(root["name"], "/name"),
    }

    if "meta" in root:
        meta = _expect_obj(root["meta"], "/meta")
        _check_allowed_keys(meta, frozenset({"author", "licence", "source"}), "/meta")
        out["meta"] = {k: _expect_str(v, f"/meta/{k}") for k, v in meta.items()}

    if "root" in root:
        out["root"] = _canon_body(root["root"], "/root", mode)

    if "limbs" in root:
        out["limbs"] = _canon_constructor(root["limbs"], "/limbs", mode)

    if "power_bus" in root:
        pb = root["power_bus"]
        if isinstance(pb, list):
            out["power_bus"] = [
                _canon_power_bus_one(b, f"/power_bus[{i}]", mode) for i, b in enumerate(pb)
            ]
        else:
            out["power_bus"] = _canon_power_bus_one(pb, "/power_bus", mode)

    if "sensors" in root:
        out["sensors"] = [
            _canon_sensor(s, f"/sensors[{i}]", mode)
            for i, s in enumerate(_expect_list(root["sensors"], "/sensors"))
        ]

    if "payloads" in root:
        out["payloads"] = [
            _canon_body(
                p,
                f"/payloads[{i}]",
                mode,
                extra_allowed=frozenset({"mount", "removable"}),
                is_payload=True,
            )
            for i, p in enumerate(_expect_list(root["payloads"], "/payloads"))
        ]

    if "budgets" in root:
        out["budgets"] = _canon_budgets(root["budgets"], "/budgets", mode)

    if "levers" in root:
        out["levers"] = [
            _canon_lever(lv, f"/levers[{i}]", mode)
            for i, lv in enumerate(_expect_list(root["levers"], "/levers"))
        ]

    if "locks" in root:
        out["locks"] = [
            _expect_str(lk, f"/locks[{i}]")
            for i, lk in enumerate(_expect_list(root["locks"], "/locks"))
        ]

    if "symmetry" in root:
        sym = _expect_obj(root["symmetry"], "/symmetry")
        _check_allowed_keys(sym, frozenset({"assert_mech", "assert_obs"}), "/symmetry")
        out["symmetry"] = {
            sk: _canon_generic_strict(sv, f"/symmetry/{sk}", mode) for sk, sv in sym.items()
        }

    if "reduced" in root:
        out["reduced"] = _canon_generic_strict(root["reduced"], "/reduced", mode)

    if "init" in root:
        out["init"] = _canon_generic_strict(root["init"], "/init", mode)

    return out


# ============================================================================
# Step 5: Canonical serializer (explicit code-point sort, minimal \u00xx escapes)
# ============================================================================

def _escape_string(s: str) -> str:
    """Shared writer Rule 3: minimal escapes `\"`, `\\`, and `\\u00xx` for < 0x20; 0x7F and non-ASCII raw."""
    parts: List[str] = ['"']
    for ch in s:
        cp = ord(ch)
        if ch == '"':
            parts.append('\\"')
        elif ch == "\\":
            parts.append("\\\\")
        elif cp < 0x20:
            parts.append(f"\\u{cp:04x}")
        else:
            parts.append(ch)
    parts.append('"')
    return "".join(parts)


def _key_sort_fn(mode: str):
    if mode == "utf16-keys":
        # Planted mutant 1: sort keys by UTF-16 code units (BMP U+E000..U+FFFF after U+10000+).
        return lambda kv: kv[0].encode("utf-16-be")
    # Rule 1: sort keys by Unicode code point (Python str order == UTF-8 byte order).
    return lambda kv: kv[0]


def serialize_canonical(val: Any, mode: str = "ref") -> bytes:
    out: List[str] = []
    key_fn = _key_sort_fn(mode)

    def _emit(v: Any) -> None:
        if isinstance(v, bool):
            out.append("true" if v else "false")
        elif isinstance(v, int):
            out.append("0" if v == 0 else str(v))
        elif isinstance(v, str):
            out.append(_escape_string(v))
        elif isinstance(v, list):
            out.append("[")
            for idx, item in enumerate(v):
                if idx > 0:
                    out.append(",")
                _emit(item)
            out.append("]")
        elif isinstance(v, dict):
            out.append("{")
            for idx, (k, child) in enumerate(sorted(v.items(), key=key_fn)):
                if idx > 0:
                    out.append(",")
                out.append(_escape_string(k))
                out.append(":")
                _emit(child)
            out.append("}")
        else:
            raise CanonRefusal("invalid-type", f"cannot serialize {type(v).__name__}")

    _emit(val)
    return "".join(out).encode("utf-8")


# ============================================================================
# Step 6: End-to-end entry point + CLI
# ============================================================================

def canonicalize_bytes(raw_bytes: bytes, mode: str = "ref") -> Tuple[bytes, str]:
    """Canonicalize raw `.fbot` bytes into `(canonical_utf8_bytes, design_id)`."""
    parsed = parse_raw_json(raw_bytes)
    canon_tree = canonicalize_document(parsed, mode=mode)
    canon_bytes = serialize_canonical(canon_tree, mode=mode)
    did = compute_design_id(canon_bytes)
    return canon_bytes, did


def _main(argv: Sequence[str]) -> int:
    mode = "ref"
    raw_out = False
    path: Optional[str] = None
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg == "--mode":
            if i + 1 >= len(argv):
                sys.stderr.write("error: --mode requires a value\n")
                return 2
            mode = argv[i + 1]
            i += 2
        elif arg == "--raw-bytes":
            raw_out = True
            i += 1
        elif arg in ("-h", "--help"):
            sys.stdout.write(
                "usage: canon.py [--mode ref|utf16-keys|bin64-roundtrip|bin64-exact|no-defaults] [--raw-bytes] [file]\n"
            )
            return 0
        elif arg.startswith("-"):
            sys.stderr.write(f"error: unknown option {arg}\n")
            return 2
        else:
            path = arg
            i += 1

    if mode not in VALID_MODES:
        sys.stderr.write(f"error: invalid mode `{mode}`\n")
        return 2

    if path is not None:
        with open(path, "rb") as fh:
            raw_bytes = fh.read()
    else:
        raw_bytes = sys.stdin.buffer.read()

    try:
        canon_bytes, did = canonicalize_bytes(raw_bytes, mode=mode)
    except CanonRefusal as ref:
        payload = {"detail": ref.detail, "reason": ref.reason, "status": "refused"}
        sys.stdout.write(json.dumps(payload, sort_keys=True, separators=(",", ":")) + "\n")
        return 3

    if raw_out:
        sys.stdout.buffer.write(canon_bytes)
        return 0

    payload = {
        "canon_hex": canon_bytes.hex(),
        "design_id": did,
        "status": "ok",
    }
    sys.stdout.write(json.dumps(payload, sort_keys=True, separators=(",", ":")) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(_main(sys.argv[1:]))
