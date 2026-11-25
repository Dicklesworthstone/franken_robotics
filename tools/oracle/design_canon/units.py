#!/usr/bin/env python3
"""Independent exact rational unit table and quantity printer for N14a-26.

Written directly from §14.4 (rules 2-4) and the `frob.design/1` unit rules using
only Python's exact `fractions.Fraction` and `decimal.Decimal` (for the planted
binary64 mutant modes only). Shares no unit table or code with `frob-core` or
`frob-design`.
"""

from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal
from fractions import Fraction
import re
from typing import Dict, Optional, Tuple


class UnitError(ValueError):
    """Refusal error raised when a quantity string or unit token is invalid."""

    def __init__(self, reason_class: str, detail: str) -> None:
        super().__init__(detail)
        self.reason_class = reason_class
        self.detail = detail


@dataclass(frozen=True)
class UnitSpec:
    dimension: str
    canonical_unit: str
    scale: Fraction
    kind: str  # "si", "angle_turn", "angle_rad", "temp_abs", "currency", "dimensionless"
    offset: Fraction = Fraction(0, 1)


# Explicit coherent-SI unit table (independent copy by design, N14a-26).
UNIT_TABLE: Dict[str, UnitSpec] = {
    # Length -> m
    "m": UnitSpec("length", "m", Fraction(1, 1), "si"),
    "mm": UnitSpec("length", "m", Fraction(1, 1000), "si"),
    "cm": UnitSpec("length", "m", Fraction(1, 100), "si"),
    "dm": UnitSpec("length", "m", Fraction(1, 10), "si"),
    "um": UnitSpec("length", "m", Fraction(1, 1_000_000), "si"),
    "nm": UnitSpec("length", "m", Fraction(1, 1_000_000_000), "si"),
    "km": UnitSpec("length", "m", Fraction(1000, 1), "si"),
    "in": UnitSpec("length", "m", Fraction(254, 10_000), "si"),
    "ft": UnitSpec("length", "m", Fraction(3048, 10_000), "si"),
    # Mass -> kg
    "kg": UnitSpec("mass", "kg", Fraction(1, 1), "si"),
    "g": UnitSpec("mass", "kg", Fraction(1, 1000), "si"),
    "mg": UnitSpec("mass", "kg", Fraction(1, 1_000_000), "si"),
    "t": UnitSpec("mass", "kg", Fraction(1000, 1), "si"),
    "lb": UnitSpec("mass", "kg", Fraction(45_359_237, 100_000_000), "si"),
    # Time -> s
    "s": UnitSpec("time", "s", Fraction(1, 1), "si"),
    "ms": UnitSpec("time", "s", Fraction(1, 1000), "si"),
    "us": UnitSpec("time", "s", Fraction(1, 1_000_000), "si"),
    "ns": UnitSpec("time", "s", Fraction(1, 1_000_000_000), "si"),
    "min": UnitSpec("time", "s", Fraction(60, 1), "si"),
    "h": UnitSpec("time", "s", Fraction(3600, 1), "si"),
    # Angles: deg/turn -> turn ratio; rad -> rad unchanged
    "deg": UnitSpec("angle", "turn", Fraction(1, 360), "angle_turn"),
    "turn": UnitSpec("angle", "turn", Fraction(1, 1), "angle_turn"),
    "rad": UnitSpec("angle", "rad", Fraction(1, 1), "angle_rad"),
    # Frequency -> Hz
    "Hz": UnitSpec("frequency", "Hz", Fraction(1, 1), "si"),
    "mHz": UnitSpec("frequency", "Hz", Fraction(1, 1000), "si"),
    "kHz": UnitSpec("frequency", "Hz", Fraction(1000, 1), "si"),
    "MHz": UnitSpec("frequency", "Hz", Fraction(1_000_000, 1), "si"),
    # Voltage -> V
    "V": UnitSpec("voltage", "V", Fraction(1, 1), "si"),
    "mV": UnitSpec("voltage", "V", Fraction(1, 1000), "si"),
    "kV": UnitSpec("voltage", "V", Fraction(1000, 1), "si"),
    # Current -> A
    "A": UnitSpec("current", "A", Fraction(1, 1), "si"),
    "mA": UnitSpec("current", "A", Fraction(1, 1000), "si"),
    "kA": UnitSpec("current", "A", Fraction(1000, 1), "si"),
    # Resistance -> ohm
    "ohm": UnitSpec("resistance", "ohm", Fraction(1, 1), "si"),
    "mohm": UnitSpec("resistance", "ohm", Fraction(1, 1000), "si"),
    "kohm": UnitSpec("resistance", "ohm", Fraction(1000, 1), "si"),
    # Power -> W
    "W": UnitSpec("power", "W", Fraction(1, 1), "si"),
    "mW": UnitSpec("power", "W", Fraction(1, 1000), "si"),
    "kW": UnitSpec("power", "W", Fraction(1000, 1), "si"),
    # Energy -> J
    "J": UnitSpec("energy", "J", Fraction(1, 1), "si"),
    "kJ": UnitSpec("energy", "J", Fraction(1000, 1), "si"),
    "MJ": UnitSpec("energy", "J", Fraction(1_000_000, 1), "si"),
    "Wh": UnitSpec("energy", "J", Fraction(3600, 1), "si"),
    "kWh": UnitSpec("energy", "J", Fraction(3_600_000, 1), "si"),
    # Thermal conductance -> W/K
    "W/K": UnitSpec("thermal_conductance", "W/K", Fraction(1, 1), "si"),
    "mW/K": UnitSpec("thermal_conductance", "W/K", Fraction(1, 1000), "si"),
    # Temperature -> K (degC affine for absolute temperature only)
    "K": UnitSpec("temperature", "K", Fraction(1, 1), "si"),
    "degC": UnitSpec("temperature", "K", Fraction(1, 1), "temp_abs", Fraction(27315, 100)),
    # Density -> kg/m^3
    "kg/m^3": UnitSpec("density", "kg/m^3", Fraction(1, 1), "si"),
    "g/cm^3": UnitSpec("density", "kg/m^3", Fraction(1000, 1), "si"),
    # Force -> N
    "N": UnitSpec("force", "N", Fraction(1, 1), "si"),
    "mN": UnitSpec("force", "N", Fraction(1, 1000), "si"),
    "kN": UnitSpec("force", "N", Fraction(1000, 1), "si"),
    # Torque -> N*m
    "N*m": UnitSpec("torque", "N*m", Fraction(1, 1), "si"),
    "mN*m": UnitSpec("torque", "N*m", Fraction(1, 1000), "si"),
    # Pressure -> Pa
    "Pa": UnitSpec("pressure", "Pa", Fraction(1, 1), "si"),
    "kPa": UnitSpec("pressure", "Pa", Fraction(1000, 1), "si"),
    "MPa": UnitSpec("pressure", "Pa", Fraction(1_000_000, 1), "si"),
    "bar": UnitSpec("pressure", "Pa", Fraction(100_000, 1), "si"),
    # Velocity -> m/s, rad/s
    "m/s": UnitSpec("linear_velocity", "m/s", Fraction(1, 1), "si"),
    "mm/s": UnitSpec("linear_velocity", "m/s", Fraction(1, 1000), "si"),
    "rad/s": UnitSpec("angular_velocity", "rad/s", Fraction(1, 1), "si"),
    # Inertia -> kg*m^2
    "kg*m^2": UnitSpec("inertia", "kg*m^2", Fraction(1, 1), "si"),
    "g*cm^2": UnitSpec("inertia", "kg*m^2", Fraction(1, 10_000_000), "si"),
    # Damping -> N*m*s/rad, N*s/m
    "N*m*s/rad": UnitSpec("rotational_damping", "N*m*s/rad", Fraction(1, 1), "si"),
    "N*s/m": UnitSpec("linear_damping", "N*s/m", Fraction(1, 1), "si"),
}

ISO_CURRENCIES = frozenset({"USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "CNY"})

_DECIMAL_RE = re.compile(r"^([+-]?)(\d+)(?:\.(\d+))?(?:[eE]([+-]?\d+))?$")
_RATIO_RE = re.compile(r"^([+-]?)(\d+)/(\d+)$")


def parse_exact_magnitude(text: str) -> Fraction:
    """Parse an exact decimal, scientific notation, or ratio into a reduced Fraction."""
    if not text:
        raise UnitError("invalid-quantity", "empty magnitude")
    rm = _RATIO_RE.match(text)
    if rm:
        sign_s, num_s, den_s = rm.groups()
        den = int(den_s)
        if den == 0:
            raise UnitError("invalid-quantity", f"zero denominator in `{text}`")
        num = int(num_s)
        if sign_s == "-":
            num = -num
        return Fraction(num, den)
    dm = _DECIMAL_RE.match(text)
    if dm:
        sign_s, int_s, frac_s, exp_s = dm.groups()
        frac_s = frac_s or ""
        exp = int(exp_s) if exp_s is not None else 0
        digits = int(int_s + frac_s)
        if sign_s == "-":
            digits = -digits
        scale_exp = exp - len(frac_s)
        if scale_exp >= 0:
            return Fraction(digits * (10 ** scale_exp), 1)
        return Fraction(digits, 10 ** (-scale_exp))
    raise UnitError("invalid-quantity", f"invalid exact magnitude `{text}`")


def shortest_decimal(frac: Fraction) -> str:
    """Print a reduced Fraction as its shortest terminating decimal, or `p/q`."""
    if frac == 0:
        return "0"
    neg = frac < 0
    num = -frac.numerator if neg else frac.numerator
    den = frac.denominator

    a = 0
    rem = den
    while rem % 2 == 0:
        rem //= 2
        a += 1
    b = 0
    while rem % 5 == 0:
        rem //= 5
        b += 1

    if rem != 1:
        signed_num = -num if neg else num
        return f"{signed_num}/{den}"

    k = max(a, b)
    scaled = num * (5 ** (k - b)) * (2 ** (k - a))
    if k == 0:
        out = str(scaled)
    else:
        s = str(scaled).rjust(k + 1, "0")
        whole = s[:-k]
        frac_part = s[-k:].rstrip("0")
        out = f"{whole}.{frac_part}" if frac_part else whole
    return f"-{out}" if neg else out


def format_turn_ratio(frac: Fraction) -> str:
    """Format an AngleTurn value as a reduced ratio of a turn (always ratio, even if terminating)."""
    if frac == 0:
        return "0 turn"
    if frac.denominator == 1:
        return f"{frac.numerator} turn"
    return f"{frac.numerator}/{frac.denominator} turn"


def _format_binary64_roundtrip(frac: Fraction) -> str:
    """Planted wrong printer 2a: route magnitude through IEEE-754 binary64, shortest round-trip."""
    val = float(frac)
    if val == 0.0:
        return "0"
    # Python's repr(float) uses David Gay's shortest round-trip decimal.
    s = repr(val)
    if "e" in s or "E" in s:
        # Format small/large binary64 round-trip cleanly without exponent.
        d = Decimal(s)
        txt = format(d, "f")
        if "." in txt:
            txt = txt.rstrip("0").rstrip(".")
        return txt
    if s.endswith(".0"):
        return s[:-2]
    return s


def _format_binary64_exact(frac: Fraction) -> str:
    """Planted wrong printer 2b: route magnitude through IEEE-754 binary64, print exact double value."""
    val = float(frac)
    if val == 0.0:
        return "0"
    d = Decimal.from_float(val)
    txt = format(d, "f")
    if "." in txt:
        txt = txt.rstrip("0").rstrip(".")
    return txt


def resolve_unit(token: str) -> UnitSpec:
    if token in UNIT_TABLE:
        return UNIT_TABLE[token]
    if token in ISO_CURRENCIES:
        return UnitSpec(f"currency:{token}", token, Fraction(1, 1), "currency")
    raise UnitError("unknown-unit", f"unknown unit token `{token}`")


def canonicalize_quantity_string(
    raw: str,
    expected_dim: Optional[str] = None,
    *,
    allow_dimensionless: bool = False,
    temp_is_diff: bool = False,
    mode: str = "ref",
) -> Tuple[str, str]:
    """Canonicalize a bare quantity string and return `(canonical_str, dimension)`."""
    if not isinstance(raw, str) or not raw:
        raise UnitError("invalid-quantity", "quantity must be a non-empty string")
    if raw != raw.strip():
        raise UnitError("invalid-quantity", f"leading or trailing whitespace in `{raw}`")

    parts = raw.split(" ")
    if len(parts) == 1:
        if not allow_dimensionless:
            raise UnitError("unknown-unit", f"missing unit token in `{raw}`")
        mag = parse_exact_magnitude(parts[0])
        if expected_dim is not None and expected_dim != "dimensionless":
            raise UnitError(
                "unknown-unit",
                f"dimensionless quantity `{raw}` in `{expected_dim}` field",
            )
        return _print_scalar(mag, mode), "dimensionless"

    if len(parts) != 2 or not parts[0] or not parts[1]:
        raise UnitError("invalid-quantity", f"malformed quantity `{raw}`")

    mag_text, unit_tok = parts[0], parts[1]
    mag = parse_exact_magnitude(mag_text)
    spec = resolve_unit(unit_tok)

    if spec.kind == "temp_abs" and temp_is_diff:
        raise UnitError("unknown-unit", f"affine `{unit_tok}` forbidden in temperature-difference field")

    if expected_dim is not None:
        if expected_dim == "currency":
            if not spec.dimension.startswith("currency:"):
                raise UnitError(
                    "unknown-unit",
                    f"expected currency unit, got `{unit_tok}` ({spec.dimension})",
                )
        elif expected_dim != spec.dimension:
            raise UnitError(
                "unknown-unit",
                f"unit `{unit_tok}` has dimension `{spec.dimension}`, expected `{expected_dim}`",
            )

    scaled = mag * spec.scale + spec.offset
    if spec.kind == "angle_turn":
        return format_turn_ratio(scaled), spec.dimension
    if spec.kind == "angle_rad":
        return f"{_print_scalar(scaled, mode)} rad", spec.dimension
    return f"{_print_scalar(scaled, mode)} {spec.canonical_unit}", spec.dimension


def _print_scalar(val: Fraction, mode: str) -> str:
    if mode == "bin64-roundtrip":
        return _format_binary64_roundtrip(val)
    if mode == "bin64-exact":
        return _format_binary64_exact(val)
    return shortest_decimal(val)
