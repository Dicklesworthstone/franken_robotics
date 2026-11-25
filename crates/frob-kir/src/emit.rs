//! Rust and shared-dialect (CUDA/Metal) emitters for `frob.kir/1` programs (`BD-018`).
//!
//! Enforces strictly one arithmetic operation per SSA statement, bit-exact constant
//! encoding (`f32::from_bits` / `FROB_F32_BITS`), zero decimal floating literals,
//! zero `mul_add`, zero platform transcendentals, and explicit refusal of `F64` on
//! the dialect target. Determinism D0; zero `unsafe`; prints nothing (L-7).

use std::fmt::Write as _;

use crate::ir::{CmpOp, KIR_SCHEMA, Op, Program, Ty};

/// Convert a kernel name like `K-cay-step` into a C/Rust snake_case identifier `k_cay_step`.
#[must_use]
pub fn snake_ident(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

/// Emit a deterministic Rust function `<snake>_body(...)` with one operation per `let` binding.
pub fn emit_rust(prog: &Program) -> Result<String, String> {
    prog.typecheck().map_err(|r| r.message)?;
    let hash = prog.content_hash().map_err(|e| e.to_string())?.to_hex();
    let fn_name = format!("{}_body", snake_ident(&prog.name));

    let mut out = String::new();
    let _ = writeln!(
        out,
        "// @generated from {KIR_SCHEMA} program `{}` (hash: {hash})",
        prog.name
    );
    let _ = writeln!(out, "// Do not edit by hand (INV-SPARK-08).");
    let _ = write!(out, "#[inline]\npub fn {fn_name}(");
    for (i, inp) in prog.inputs.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{}: {}", inp.name, inp.ty.as_str());
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
            Op::Input { port } => prog.inputs[port].name.clone(),
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
            Op::Abs(a) => format!("v{a}.abs()"),
            Op::Min(a, b) => match node.ty {
                Ty::F32 => format!("crate::ir::ieee_min_f32(v{a}, v{b})"),
                Ty::F64 => format!(
                    "if v{a} != v{a} {{ v{a} }} else if v{b} != v{b} {{ v{b} }} else if v{a} < v{b} {{ v{a} }} else if v{b} < v{a} {{ v{b} }} else if v{a}.to_bits() == 0x8000_0000_0000_0000u64 {{ v{a} }} else {{ v{b} }}"
                ),
                Ty::Bool => return Err("min on bool unsupported".into()),
            },
            Op::Max(a, b) => match node.ty {
                Ty::F32 => format!("crate::ir::ieee_max_f32(v{a}, v{b})"),
                Ty::F64 => format!(
                    "if v{a} != v{a} {{ v{a} }} else if v{b} != v{b} {{ v{b} }} else if v{a} > v{b} {{ v{a} }} else if v{b} > v{a} {{ v{b} }} else if v{a}.to_bits() == 0u64 {{ v{a} }} else {{ v{b} }}"
                ),
                Ty::Bool => return Err("max on bool unsupported".into()),
            },
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

/// Emit a deterministic shared-dialect (`FROB_FN`) body compatible with both CUDA and Metal.
///
/// Refuses any `Ty::F64` program (Metal has no double precision; never silently narrows).
pub fn emit_dialect(prog: &Program) -> Result<String, String> {
    prog.typecheck().map_err(|r| r.message)?;
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
        "/* @generated from {KIR_SCHEMA} program `{}` (hash: {hash}) */",
        prog.name
    );
    let _ = writeln!(out, "/* Do not edit by hand (INV-SPARK-08). */");
    let _ = write!(out, "FROB_FN void {fn_name}(");
    let mut first = true;
    for inp in &prog.inputs {
        if !first {
            out.push_str(", ");
        }
        first = false;
        let cty = dialect_ty(inp.ty);
        let _ = write!(out, "{cty} {}", inp.name);
    }
    for (p, _) in &prog.outputs {
        if !first {
            out.push_str(", ");
        }
        first = false;
        let cty = dialect_ty(p.ty);
        let _ = write!(out, "FROB_PTR({cty}) out_{}", p.name);
    }
    out.push_str(") {\n");

    for (idx, node) in prog.nodes.iter().enumerate() {
        let cty = dialect_ty(node.ty);
        match node.op {
            Op::Input { port } => {
                let _ = writeln!(out, "    const {cty} v{idx} = {};", prog.inputs[port].name);
            }
            Op::Const {
                bits, ty: Ty::F32, ..
            } => {
                let _ = writeln!(
                    out,
                    "    const f32 v{idx} = FROB_F32_BITS(0x{:08x}u);",
                    bits as u32
                );
            }
            Op::Const {
                bits, ty: Ty::Bool, ..
            } => {
                let b = if bits != 0 { "1u" } else { "0u" };
                let _ = writeln!(out, "    const u32 v{idx} = {b};");
            }
            Op::Const { ty: Ty::F64, .. } => unreachable!("F64 refused above"),
            Op::Add(a, b) => {
                let _ = writeln!(out, "    const f32 v{idx} = v{a} + v{b};");
            }
            Op::Sub(a, b) => {
                let _ = writeln!(out, "    const f32 v{idx} = v{a} - v{b};");
            }
            Op::Mul(a, b) => {
                let _ = writeln!(out, "    const f32 v{idx} = v{a} * v{b};");
            }
            Op::Div(a, b) => {
                let _ = writeln!(out, "    const f32 v{idx} = v{a} / v{b};");
            }
            Op::Sqrt(a) => {
                let _ = writeln!(out, "    const f32 v{idx} = FROB_SQRT(v{a});");
            }
            Op::Neg(a) => {
                let _ = writeln!(out, "    const f32 v{idx} = -v{a};");
            }
            Op::Abs(a) => {
                let _ = writeln!(out, "    const f32 v{idx}_neg = -v{a};");
                let _ = writeln!(
                    out,
                    "    const f32 v{idx} = FROB_SELECT(v{a} < FROB_F32_BITS(0x00000000u), v{idx}_neg, v{a});"
                );
            }
            Op::Min(a, b) => {
                // Explicit NaN-propagating, signed-zero-exact select sequence (zero built-ins).
                let _ = writeln!(
                    out,
                    "    const f32 v{idx}_ab = FROB_SELECT(v{a} < v{b}, v{a}, v{b});"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{idx}_nz = FROB_SELECT(v{b} == FROB_F32_BITS(0x00000000u), v{a}, v{idx}_ab);"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{idx}_eq = FROB_SELECT(v{a} == v{b}, v{idx}_nz, v{idx}_ab);"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{idx}_nb = FROB_SELECT(v{b} != v{b}, v{b}, v{idx}_eq);"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{idx} = FROB_SELECT(v{a} != v{a}, v{a}, v{idx}_nb);"
                );
            }
            Op::Max(a, b) => {
                let _ = writeln!(
                    out,
                    "    const f32 v{idx}_ab = FROB_SELECT(v{a} > v{b}, v{a}, v{b});"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{idx}_pz = FROB_SELECT(v{b} == FROB_F32_BITS(0x00000000u), v{b}, v{idx}_ab);"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{idx}_eq = FROB_SELECT(v{a} == v{b}, v{idx}_pz, v{idx}_ab);"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{idx}_nb = FROB_SELECT(v{b} != v{b}, v{b}, v{idx}_eq);"
                );
                let _ = writeln!(
                    out,
                    "    const f32 v{idx} = FROB_SELECT(v{a} != v{a}, v{a}, v{idx}_nb);"
                );
            }
            Op::Cmp(op, a, b) => {
                let _ = writeln!(
                    out,
                    "    const u32 v{idx} = (v{a} {} v{b}) ? 1u : 0u;",
                    cmp_sym(op)
                );
            }
            Op::Select(p, a, b) => {
                let _ = writeln!(
                    out,
                    "    const f32 v{idx} = FROB_SELECT(v{p} != 0u, v{a}, v{b});"
                );
            }
        }
    }

    for (p, nid) in &prog.outputs {
        let _ = writeln!(out, "    *out_{} = v{nid};", p.name);
    }
    out.push_str("}\n");
    Ok(out)
}

/// Static audit enforcing BD-018's one-operation-per-statement rule on emitted Rust or dialect text.
/// Rejects compound expressions like `a * b + c` that would permit compiler contraction.
pub fn check_one_op_per_statement(source: &str) -> Result<(), String> {
    for (line_no, raw) in source.lines().enumerate() {
        let line = raw.trim();
        if !(line.starts_with("let v")
            || line.starts_with("const f32 v")
            || line.starts_with("const u32 v"))
        {
            continue;
        }
        let Some((_, rhs)) = line.split_once('=') else {
            continue;
        };
        let rhs = rhs.trim().trim_end_matches(';');
        if rhs.starts_with("if ")
            || rhs.starts_with("FROB_SELECT(")
            || rhs.starts_with("crate::ir::ieee_")
        {
            continue;
        }
        let op_count = rhs
            .split_whitespace()
            .filter(|tok| matches!(*tok, "+" | "-" | "*" | "/"))
            .count();
        if op_count > 1 {
            return Err(format!(
                "line {}: multiple arithmetic operators ({op_count}) in one statement: `{line}`",
                line_no + 1
            ));
        }
    }
    Ok(())
}

/// Static audit ensuring no decimal floating-point literal appears in emitted Rust or dialect code.
pub fn check_no_decimal_float_literals(source: &str) -> Result<(), String> {
    for (line_no, raw) in source.lines().enumerate() {
        let line = raw.trim();
        if line.starts_with("//") || line.starts_with("/*") {
            continue;
        }
        let bytes = line.as_bytes();
        for i in 1..bytes.len().saturating_sub(1) {
            if bytes[i] == b'.' && bytes[i - 1].is_ascii_digit() && bytes[i + 1].is_ascii_digit() {
                return Err(format!(
                    "line {}: decimal float literal forbidden: `{line}`",
                    line_no + 1
                ));
            }
        }
    }
    Ok(())
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
