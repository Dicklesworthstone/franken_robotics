//! Rust and shared-dialect (CUDA/Metal) emitters for `frob.kir/1` programs (`BD-018`).
//!
//! Enforces strictly one arithmetic operation per SSA statement, bit-exact constant
//! encoding (`f32::from_bits` / `FROB_F32_BITS`), zero decimal floating literals,
//! zero `mul_add`, zero platform transcendentals, and explicit refusal of `F64` on
//! the dialect target. Determinism D0; zero `unsafe`; prints nothing (L-7).

use std::{collections::BTreeSet, fmt::Write as _};

use crate::ir::{CmpOp, KIR_SCHEMA, Op, Program, Ty};

/// Convert a kernel name like `K-cay-step` into a C/Rust snake_case identifier `k_cay_step`.
#[must_use]
pub fn snake_ident(name: &str) -> String {
    let mut ident: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    if !ident.starts_with(|c: char| c.is_ascii_alphabetic()) {
        ident.insert_str(0, "k_");
    }
    ident
}

fn validate_ports(prog: &Program) -> Result<(), String> {
    const RESERVED: &str = "
        abstract alignas alignof and and_eq as asm async atomic auto await become bitand bitor bool
        box break case catch char char8_t char16_t char32_t class co_await co_return co_yield compl
        concept const const_cast consteval constexpr constinit continue crate decltype default delete
        device do double dyn else enum explicit export extern false final float fn for friend gen
        goto if impl inline int kernel let long loop macro match mod move mut mutable namespace new
        noexcept not not_eq nullptr operator or or_eq override priv protected pub public ref register
        reinterpret_cast requires return safe self Self short signed sizeof static static_assert
        static_cast struct super switch template this thread thread_local threadgroup throw trait true
        try type typedef typeid typename typeof union unsafe unsized unsigned use using virtual void
        volatile wchar_t where while xor xor_eq yield constant vertex fragment packed_f32 f32 u32
    ";
    let mut seen = BTreeSet::new();
    for port in prog
        .inputs
        .iter()
        .chain(prog.outputs.iter().map(|(p, _)| p))
    {
        let name = port.name.as_str();
        if !name.starts_with(|c: char| c.is_ascii_alphabetic())
            || !name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_')
            || name.starts_with("FROB_")
            || RESERVED.split_ascii_whitespace().any(|key| key == name)
        {
            return Err(format!("invalid or reserved port name `{name}`"));
        }
        if !seen.insert(name) {
            return Err(format!("duplicate port name `{name}`"));
        }
    }
    Ok(())
}

/// Emit a deterministic Rust function `<snake>_body(...)` with one operation per `let` binding.
pub fn emit_rust(prog: &Program) -> Result<String, String> {
    prog.typecheck().map_err(|r| r.message)?;
    validate_ports(prog)?;
    let hash = prog.content_hash().map_err(|e| e.to_string())?.to_hex();
    let fn_name = format!("{}_body", snake_ident(&prog.name));

    let mut out = String::new();
    let _ = writeln!(
        out,
        "// @generated from {KIR_SCHEMA} program `{}` (hash: {hash})",
        prog.name.escape_default()
    );
    let _ = writeln!(out, "// Do not edit by hand (INV-SPARK-08).");
    let _ = write!(out, "#[rustfmt::skip]\n#[inline]\npub fn {fn_name}(");
    for (i, inp) in prog.inputs.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "arg{i}: {}", inp.ty.as_str());
    }
    out.push_str(") -> (");
    for (i, (p, _)) in prog.outputs.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(p.ty.as_str());
    }
    if prog.outputs.len() == 1 {
        out.push(',');
    }
    out.push_str(") {\n");

    for (idx, node) in prog.nodes.iter().enumerate() {
        let ty = node.ty.as_str();
        let rhs = match node.op {
            Op::Input { port } => format!("arg{port}"),
            Op::Const {
                bits, ty: Ty::F32, ..
            } => format!("f32::from_bits(0x{:08x}u32)", bits as u32),
            Op::Const {
                bits, ty: Ty::F64, ..
            } => format!("f64::from_bits(0x{bits:016x}u64)"),
            Op::Const {
                bits, ty: Ty::Bool, ..
            } => {
                if bits != 0 {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            Op::Add(a, b) => format!("v{a} + v{b}"),
            Op::Sub(a, b) => format!("v{a} - v{b}"),
            Op::Mul(a, b) => format!("v{a} * v{b}"),
            Op::Div(a, b) => format!("v{a} / v{b}"),
            Op::Sqrt(a) => format!("{ty}::sqrt(v{a})"),
            Op::Neg(a) => format!("-v{a}"),
            Op::Abs(a) => format!(
                "if v{a}.is_nan() {{ {} }} else {{ v{a}.abs() }}",
                rust_nan(node.ty)
            ),
            Op::Min(a, b) | Op::Max(a, b) => {
                let (cmp, sign) = if matches!(node.op, Op::Min(..)) {
                    ("<", "negative")
                } else {
                    (">", "positive")
                };
                let nan = rust_nan(node.ty);
                format!(
                    "if v{a}.is_nan() || v{b}.is_nan() {{ {nan} }} else if v{a} {cmp} v{b} || (v{a} == {ty}::from_bits(0) && v{b} == {ty}::from_bits(0) && v{a}.is_sign_{sign}()) {{ v{a} }} else {{ v{b} }}"
                )
            }
            Op::Cmp(op, a, b) => format!("v{a} {} v{b}", cmp_sym(op)),
            Op::Select(p, a, b) => format!("if v{p} {{ v{a} }} else {{ v{b} }}"),
        };
        let _ = writeln!(out, "    let v{idx}: {ty} = {rhs};");
    }

    out.push_str("    (");
    for (i, (_, nid)) in prog.outputs.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "v{nid}");
    }
    if prog.outputs.len() == 1 {
        out.push(',');
    }
    out.push_str(")\n}\n");
    Ok(out)
}

fn rust_nan(ty: Ty) -> &'static str {
    match ty {
        Ty::F32 => "f32::from_bits(0x7fc00000u32)",
        Ty::F64 => "f64::from_bits(0x7ff8000000000000u64)",
        Ty::Bool => unreachable!("numeric node already typechecked"),
    }
}

/// Emit a deterministic shared-dialect (`FROB_FN`) body compatible with both CUDA and Metal.
///
/// Refuses any `Ty::F64` program (Metal has no double precision; never silently narrows).
pub fn emit_dialect(prog: &Program) -> Result<String, String> {
    prog.typecheck().map_err(|r| r.message)?;
    validate_ports(prog)?;
    for p in prog
        .inputs
        .iter()
        .chain(prog.outputs.iter().map(|(p, _)| p))
    {
        if p.ty == Ty::F64 {
            return Err(format!(
                "dialect emitter refuses F64 port `{}` in `{}`",
                p.name, prog.name
            ));
        }
    }
    for (idx, n) in prog.nodes.iter().enumerate() {
        if n.ty == Ty::F64 {
            return Err(format!(
                "dialect emitter refuses F64 node {idx} in `{}`",
                prog.name
            ));
        }
    }
    let hash = prog.content_hash().map_err(|e| e.to_string())?.to_hex();
    let fn_name = format!("{}_body", snake_ident(&prog.name));

    let mut out = String::new();
    let _ = writeln!(
        out,
        "/* @generated from {KIR_SCHEMA} program `{fn_name}` (hash: {hash}) */"
    );
    let _ = writeln!(out, "/* Do not edit by hand (INV-SPARK-08). */");
    let _ = write!(out, "FROB_FN void {fn_name}(");
    let mut first = true;
    for (i, inp) in prog.inputs.iter().enumerate() {
        if !first {
            out.push_str(", ");
        }
        first = false;
        let cty = dialect_ty(inp.ty);
        let _ = write!(out, "{cty} arg{i}");
    }
    for (i, (p, _)) in prog.outputs.iter().enumerate() {
        if !first {
            out.push_str(", ");
        }
        first = false;
        let cty = dialect_ty(p.ty);
        let _ = write!(out, "FROB_PTR({cty}) out{i}");
    }
    out.push_str(") {\n");

    for (i, node) in prog.nodes.iter().enumerate() {
        let cty = dialect_ty(node.ty);
        let zero = "FROB_F32_BITS(0x00000000u)";
        let nan = "FROB_F32_BITS(0x7fc00000u)";
        let rhs = match node.op {
            Op::Input { port } => format!("arg{port}"),
            Op::Const {
                bits, ty: Ty::F32, ..
            } => format!("FROB_F32_BITS(0x{:08x}u)", bits as u32),
            Op::Const {
                bits, ty: Ty::Bool, ..
            } => format!("{bits}u"),
            Op::Const { ty: Ty::F64, .. } => unreachable!("F64 refused above"),
            Op::Add(a, b) => format!("v{a} + v{b}"),
            Op::Sub(a, b) => format!("v{a} - v{b}"),
            Op::Mul(a, b) => format!("v{a} * v{b}"),
            Op::Div(a, b) => format!("v{a} / v{b}"),
            Op::Sqrt(a) => format!("FROB_SQRT(v{a})"),
            Op::Neg(a) => format!("-v{a}"),
            Op::Abs(a) => {
                let _ = writeln!(out, "    const f32 v{i}_neg = -v{a};");
                let _ = writeln!(
                    out,
                    "    const f32 v{i}_mag = FROB_SELECT(v{a} < {zero}, v{i}_neg, v{a});"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{i}_abs = FROB_SELECT(v{a} == {zero}, {zero}, v{i}_mag);"
                );
                format!("FROB_SELECT(v{a} != v{a}, {nan}, v{i}_abs)")
            }
            Op::Min(a, b) | Op::Max(a, b) => {
                // With IEEE round-to-nearest, adding zeros keeps a negative sign
                // only if both inputs are negative. Negation reverses that rule.
                let min = matches!(node.op, Op::Min(..));
                let cmp = if min { "<" } else { ">" };
                let _ = writeln!(
                    out,
                    "    const f32 v{i}_za = FROB_SELECT(v{a} == {zero}, v{a}, {zero});"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{i}_zb = FROB_SELECT(v{b} == {zero}, v{b}, {zero});"
                );
                if min {
                    let _ = writeln!(
                        out,
                        "    const f32 v{i}_na = -v{i}_za;\n    const f32 v{i}_nb = -v{i}_zb;\n    const f32 v{i}_sum = v{i}_na + v{i}_nb;\n    const f32 v{i}_z = -v{i}_sum;"
                    );
                } else {
                    let _ = writeln!(out, "    const f32 v{i}_z = v{i}_za + v{i}_zb;");
                }
                let _ = writeln!(
                    out,
                    "    const f32 v{i}_ab = FROB_SELECT(v{a} {cmp} v{b}, v{a}, v{b});"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{i}_ordered = FROB_SELECT(v{a} == {zero} && v{b} == {zero}, v{i}_z, v{i}_ab);"
                );
                format!("FROB_SELECT(v{a} != v{a} || v{b} != v{b}, {nan}, v{i}_ordered)")
            }
            Op::Cmp(op, a, b) => format!("FROB_SELECT(v{a} {} v{b}, 1u, 0u)", cmp_sym(op)),
            Op::Select(p, a, b) => format!("FROB_SELECT(v{p} != 0u, v{a}, v{b})"),
        };
        let _ = writeln!(out, "    const {cty} v{i} = {rhs};");
    }

    for (i, (_, nid)) in prog.outputs.iter().enumerate() {
        let _ = writeln!(out, "    *out{i} = v{nid};");
    }
    out.push_str("}\n");
    Ok(out)
}

const fn dialect_ty(ty: Ty) -> &'static str {
    match ty {
        Ty::F32 => "f32",
        Ty::Bool => "u32",
        Ty::F64 => "f64",
    }
}

const fn cmp_sym(op: CmpOp) -> &'static str {
    match op {
        CmpOp::Lt => "<",
        CmpOp::Le => "<=",
        CmpOp::Eq => "==",
        CmpOp::Ne => "!=",
        CmpOp::Ge => ">=",
        CmpOp::Gt => ">",
    }
}
