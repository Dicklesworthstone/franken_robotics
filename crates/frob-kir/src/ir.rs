//! Straight-line kernel IR (`frob.kir/1`), bounded CSE, exact physical-dimension
//! checker (`FROB-REF-250`, T-09, PO-SPARK-01), canonical serialization, and
//! bit-exact reference interpreter (`BD-018`).
//!
//! Determinism D0; zero `unsafe`; prints nothing (L-7).

use std::collections::BTreeMap;
use std::fmt;

use frob_core::canon::{CanonError, CanonWriter, CanonicalBytes};
use frob_core::evidence::DetClass;
use frob_core::id::{Blake3, SchemaName, identity};
use frob_core::json::{Json, Limits, Spanned, admit};
use frob_core::qty::{AngularVelocity, Length, QuantityAlias, Ratio, Time, Velocity};
use frob_core::refusal::{Refusal, RefusalCode, Repair, RetryPolicy};

/// Schema identifier of the v0 straight-line kernel IR.
pub const KIR_SCHEMA: &str = "frob.kir/1";

/// Base dimension vector length sourced from `frob_core::qty` (never hard-coded).
pub const DIM_LEN: usize = Length::DIMENSION.len();

/// Default bounded hash-consing CSE table capacity (BD-018).
pub const DEFAULT_CSE_LIMIT: usize = 65_536;

/// Physical dimension as an exponent vector over `frob_core::qty` base dimensions
/// `[m, kg, s, K, A, mol]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dim(pub [i8; DIM_LEN]);

impl Dim {
    pub const DIMENSIONLESS: Self = Self(Ratio::DIMENSION);
    pub const LENGTH: Self = Self(Length::DIMENSION);
    pub const TIME: Self = Self(Time::DIMENSION);
    pub const VELOCITY: Self = Self(Velocity::DIMENSION);
    pub const ANGULAR_VELOCITY: Self = Self(AngularVelocity::DIMENSION);

    #[must_use]
    pub fn add(self, rhs: Self) -> Option<Self> {
        let mut out = [0i8; DIM_LEN];
        for i in 0..DIM_LEN {
            out[i] = self.0[i].checked_add(rhs.0[i])?;
        }
        Some(Self(out))
    }

    #[must_use]
    pub fn sub(self, rhs: Self) -> Option<Self> {
        let mut out = [0i8; DIM_LEN];
        for i in 0..DIM_LEN {
            out[i] = self.0[i].checked_sub(rhs.0[i])?;
        }
        Some(Self(out))
    }

    #[must_use]
    pub fn halve_even(self) -> Option<Self> {
        let mut out = [0i8; DIM_LEN];
        for i in 0..DIM_LEN {
            if self.0[i] % 2 != 0 {
                return None;
            }
            out[i] = self.0[i] / 2;
        }
        Some(Self(out))
    }
}

impl fmt::Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ty {
    F32,
    F64,
    Bool,
}

impl Ty {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::Bool => "bool",
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "bool" => Ok(Self::Bool),
            _ => Err(format!("unknown kir Ty `{s}`")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CmpOp {
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,
}

impl CmpOp {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lt => "lt",
            Self::Le => "le",
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Ge => "ge",
            Self::Gt => "gt",
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "lt" => Ok(Self::Lt),
            "le" => Ok(Self::Le),
            "eq" => Ok(Self::Eq),
            "ne" => Ok(Self::Ne),
            "ge" => Ok(Self::Ge),
            "gt" => Ok(Self::Gt),
            _ => Err(format!("unknown CmpOp `{s}`")),
        }
    }
}

/// SSA node index in a [`Program`].
pub type NodeId = usize;

/// Straight-line arithmetic operator (ADR-SPARK-02: no trig/exp/log/pow/rsqrt/fma).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Input { port: usize },
    Const { bits: u64, ty: Ty, dim: Dim },
    Add(NodeId, NodeId),
    Sub(NodeId, NodeId),
    Mul(NodeId, NodeId),
    Div(NodeId, NodeId),
    Sqrt(NodeId),
    Neg(NodeId),
    Abs(NodeId),
    Min(NodeId, NodeId),
    Max(NodeId, NodeId),
    Cmp(CmpOp, NodeId, NodeId),
    Select(NodeId, NodeId, NodeId),
}

impl Op {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Input { .. } => "input",
            Self::Const { .. } => "const",
            Self::Add(..) => "add",
            Self::Sub(..) => "sub",
            Self::Mul(..) => "mul",
            Self::Div(..) => "div",
            Self::Sqrt(_) => "sqrt",
            Self::Neg(_) => "neg",
            Self::Abs(_) => "abs",
            Self::Min(..) => "min",
            Self::Max(..) => "max",
            Self::Cmp(..) => "cmp",
            Self::Select(..) => "select",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
    pub name: String,
    pub ty: Ty,
    pub dim: Dim,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub op: Op,
    pub ty: Ty,
    pub dim: Dim,
    pub expr_name: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OpCounts {
    pub inputs: u32,
    pub consts: u32,
    pub add_sub: u32,
    pub mul: u32,
    pub div: u32,
    pub sqrt: u32,
    pub unary: u32,
    pub min_max: u32,
    pub cmp: u32,
    pub select: u32,
}

impl OpCounts {
    #[must_use]
    pub fn from_nodes(nodes: &[Node]) -> Self {
        let mut c = Self::default();
        for n in nodes {
            match n.op {
                Op::Input { .. } => c.inputs += 1,
                Op::Const { .. } => c.consts += 1,
                Op::Add(..) | Op::Sub(..) => c.add_sub += 1,
                Op::Mul(..) => c.mul += 1,
                Op::Div(..) => c.div += 1,
                Op::Sqrt(_) => c.sqrt += 1,
                Op::Neg(_) | Op::Abs(_) => c.unary += 1,
                Op::Min(..) | Op::Max(..) => c.min_max += 1,
                Op::Cmp(..) => c.cmp += 1,
                Op::Select(..) => c.select += 1,
            }
        }
        c
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramMeta {
    pub math_mode: String,
    pub op_counts: OpCounts,
    pub cpu_reference: String,
    pub determinism: DetClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub name: String,
    pub inputs: Vec<Port>,
    pub outputs: Vec<(Port, NodeId)>,
    pub nodes: Vec<Node>,
    pub meta: ProgramMeta,
}

/// Bounded hash-consing builder for straight-line [`Program`]s.
///
/// Only exact commutative normalization (`Add`, `Mul`, `Min`, `Max` sort operand ids)
/// is applied; no reassociation, distribution, or inexact constant folding occurs.
#[derive(Debug, Clone)]
pub struct ProgramBuilder {
    name: String,
    cpu_reference: String,
    determinism: DetClass,
    cse_limit: usize,
    inputs: Vec<Port>,
    outputs: Vec<(Port, NodeId)>,
    nodes: Vec<Node>,
    cse: BTreeMap<(Op, Ty, Dim), NodeId>,
}

impl ProgramBuilder {
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        cpu_reference: impl Into<String>,
        determinism: DetClass,
    ) -> Self {
        Self {
            name: name.into(),
            cpu_reference: cpu_reference.into(),
            determinism,
            cse_limit: DEFAULT_CSE_LIMIT,
            inputs: Vec::new(),
            outputs: Vec::new(),
            nodes: Vec::new(),
            cse: BTreeMap::new(),
        }
    }

    #[must_use]
    pub const fn with_cse_limit(mut self, limit: usize) -> Self {
        self.cse_limit = limit;
        self
    }

    pub fn input(&mut self, name: impl Into<String>, ty: Ty, dim: Dim) -> Result<NodeId, Refusal> {
        let port = self.inputs.len();
        let nm = name.into();
        self.inputs.push(Port {
            name: nm.clone(),
            ty,
            dim,
        });
        self.push_node(Op::Input { port }, ty, dim, nm)
    }

    pub fn const_f32(&mut self, value: f32, dim: Dim) -> Result<NodeId, Refusal> {
        let bits = u64::from(value.to_bits());
        self.push_node(
            Op::Const {
                bits,
                ty: Ty::F32,
                dim,
            },
            Ty::F32,
            dim,
            format!("const_f32_{bits:08x}"),
        )
    }

    pub fn const_f64(&mut self, value: f64, dim: Dim) -> Result<NodeId, Refusal> {
        let bits = value.to_bits();
        self.push_node(
            Op::Const {
                bits,
                ty: Ty::F64,
                dim,
            },
            Ty::F64,
            dim,
            format!("const_f64_{bits:016x}"),
        )
    }

    pub fn add(
        &mut self,
        a: NodeId,
        b: NodeId,
        expr: impl Into<String>,
    ) -> Result<NodeId, Refusal> {
        let (x, y) = sort_pair(a, b);
        self.bin_same_dim(Op::Add(x, y), a, b, expr.into())
    }

    pub fn sub(
        &mut self,
        a: NodeId,
        b: NodeId,
        expr: impl Into<String>,
    ) -> Result<NodeId, Refusal> {
        self.bin_same_dim(Op::Sub(a, b), a, b, expr.into())
    }

    pub fn mul(
        &mut self,
        a: NodeId,
        b: NodeId,
        expr: impl Into<String>,
    ) -> Result<NodeId, Refusal> {
        let (x, y) = sort_pair(a, b);
        let na = &self.nodes[a];
        let nb = &self.nodes[b];
        let ty = na.ty;
        let dim = na.dim.add(nb.dim).unwrap_or(na.dim);
        self.push_node(Op::Mul(x, y), ty, dim, expr.into())
    }

    pub fn div(
        &mut self,
        a: NodeId,
        b: NodeId,
        expr: impl Into<String>,
    ) -> Result<NodeId, Refusal> {
        let na = &self.nodes[a];
        let nb = &self.nodes[b];
        let ty = na.ty;
        let dim = na.dim.sub(nb.dim).unwrap_or(na.dim);
        self.push_node(Op::Div(a, b), ty, dim, expr.into())
    }

    pub fn sqrt(&mut self, a: NodeId, expr: impl Into<String>) -> Result<NodeId, Refusal> {
        let na = &self.nodes[a];
        let ty = na.ty;
        let expr_s = expr.into();
        let dim = match na.dim.halve_even() {
            Some(d) => d,
            None => {
                return Err(dim_refusal(
                    &self.name,
                    self.nodes.len(),
                    &expr_s,
                    na.dim,
                    Dim::DIMENSIONLESS,
                    "sqrt requires even dimension exponents",
                ));
            }
        };
        self.push_node(Op::Sqrt(a), ty, dim, expr_s)
    }

    pub fn neg(&mut self, a: NodeId, expr: impl Into<String>) -> Result<NodeId, Refusal> {
        let (ty, dim) = (self.nodes[a].ty, self.nodes[a].dim);
        self.push_node(Op::Neg(a), ty, dim, expr.into())
    }

    pub fn abs(&mut self, a: NodeId, expr: impl Into<String>) -> Result<NodeId, Refusal> {
        let (ty, dim) = (self.nodes[a].ty, self.nodes[a].dim);
        self.push_node(Op::Abs(a), ty, dim, expr.into())
    }

    pub fn min(
        &mut self,
        a: NodeId,
        b: NodeId,
        expr: impl Into<String>,
    ) -> Result<NodeId, Refusal> {
        let (x, y) = sort_pair(a, b);
        self.bin_same_dim(Op::Min(x, y), a, b, expr.into())
    }

    pub fn max(
        &mut self,
        a: NodeId,
        b: NodeId,
        expr: impl Into<String>,
    ) -> Result<NodeId, Refusal> {
        let (x, y) = sort_pair(a, b);
        self.bin_same_dim(Op::Max(x, y), a, b, expr.into())
    }

    pub fn cmp(
        &mut self,
        op: CmpOp,
        a: NodeId,
        b: NodeId,
        expr: impl Into<String>,
    ) -> Result<NodeId, Refusal> {
        let expr_s = expr.into();
        let (da, db) = (self.nodes[a].dim, self.nodes[b].dim);
        if da != db {
            return Err(dim_refusal(
                &self.name,
                self.nodes.len(),
                &expr_s,
                da,
                db,
                "comparison operands have different dimensions",
            ));
        }
        self.push_node(Op::Cmp(op, a, b), Ty::Bool, Dim::DIMENSIONLESS, expr_s)
    }

    pub fn select(
        &mut self,
        pred: NodeId,
        a: NodeId,
        b: NodeId,
        expr: impl Into<String>,
    ) -> Result<NodeId, Refusal> {
        let expr_s = expr.into();
        let (da, db) = (self.nodes[a].dim, self.nodes[b].dim);
        if da != db {
            return Err(dim_refusal(
                &self.name,
                self.nodes.len(),
                &expr_s,
                da,
                db,
                "select arms have different dimensions",
            ));
        }
        let ty = self.nodes[a].ty;
        self.push_node(Op::Select(pred, a, b), ty, da, expr_s)
    }

    pub fn output(&mut self, name: impl Into<String>, ty: Ty, dim: Dim, node: NodeId) {
        self.outputs.push((
            Port {
                name: name.into(),
                ty,
                dim,
            },
            node,
        ));
    }

    /// Finalize and typecheck (dimension homogeneity, SSA order, types).
    pub fn build(self) -> Result<Program, Refusal> {
        let op_counts = OpCounts::from_nodes(&self.nodes);
        let prog = Program {
            name: self.name,
            inputs: self.inputs,
            outputs: self.outputs,
            nodes: self.nodes,
            meta: ProgramMeta {
                math_mode: "Ieee754Safe".into(),
                op_counts,
                cpu_reference: self.cpu_reference,
                determinism: self.determinism,
            },
        };
        prog.typecheck()?;
        Ok(prog)
    }

    fn bin_same_dim(
        &mut self,
        op: Op,
        a: NodeId,
        b: NodeId,
        expr: String,
    ) -> Result<NodeId, Refusal> {
        let (da, db) = (self.nodes[a].dim, self.nodes[b].dim);
        if da != db {
            return Err(dim_refusal(
                &self.name,
                self.nodes.len(),
                &expr,
                da,
                db,
                "operands have incompatible physical dimensions",
            ));
        }
        let ty = self.nodes[a].ty;
        self.push_node(op, ty, da, expr)
    }

    fn push_node(
        &mut self,
        op: Op,
        ty: Ty,
        dim: Dim,
        expr_name: String,
    ) -> Result<NodeId, Refusal> {
        if let Some(&existing) = self.cse.get(&(op, ty, dim)) {
            return Ok(existing);
        }
        if self.nodes.len() >= self.cse_limit {
            return Err(Refusal::new(
                RefusalCode::FrobRef250,
                format!("frob-kir CSE node budget exceeded ({})", self.cse_limit),
                format!("/{}/nodes", self.name),
                self.nodes.len().to_string(),
                format!("<= {}", self.cse_limit),
                "nodes",
                RetryPolicy::AfterChange,
                vec![
                    Repair::new("Split the kernel body into smaller derived stages"),
                    Repair::new("Raise the builder CSE limit if justified by RISK-07"),
                ],
            ));
        }
        let id = self.nodes.len();
        self.nodes.push(Node {
            op,
            ty,
            dim,
            expr_name,
        });
        self.cse.insert((op, ty, dim), id);
        Ok(id)
    }
}

#[inline]
const fn sort_pair(a: NodeId, b: NodeId) -> (NodeId, NodeId) {
    if a <= b { (a, b) } else { (b, a) }
}

fn dim_refusal(kernel: &str, node: usize, expr: &str, obs: Dim, req: Dim, msg: &str) -> Refusal {
    Refusal::new(
        RefusalCode::FrobRef250,
        format!("kernel `{kernel}` node {node} (`{expr}`): {msg}"),
        format!("/{kernel}/node/{node}"),
        format!("expr={expr} dim={obs}"),
        format!("equal exponent vectors (expected {req})"),
        "dim",
        RetryPolicy::AfterChange,
        vec![
            Repair::new(format!(
                "Fix the derivation at `{kernel}` node {node} (`{expr}`)"
            )),
            Repair::new("Declare the missing conversion constant with its physical dimension"),
        ],
    )
}

impl Program {
    /// Independent exact type & dimension verification (T-09, PO-SPARK-01).
    pub fn typecheck(&self) -> Result<(), Refusal> {
        let mut inferred_ty: Vec<Ty> = Vec::with_capacity(self.nodes.len());
        let mut inferred_dim: Vec<Dim> = Vec::with_capacity(self.nodes.len());

        for (idx, node) in self.nodes.iter().enumerate() {
            let check_ref = |r: NodeId| -> Result<(Ty, Dim), Refusal> {
                if r >= idx {
                    return Err(dim_refusal(
                        &self.name,
                        idx,
                        &node.expr_name,
                        node.dim,
                        node.dim,
                        "SSA order violation: operand index >= node index",
                    ));
                }
                Ok((inferred_ty[r], inferred_dim[r]))
            };
            let (ty, dim) = match node.op {
                Op::Input { port } => {
                    let p = self.inputs.get(port).ok_or_else(|| {
                        dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            node.dim,
                            node.dim,
                            "input port out of range",
                        )
                    })?;
                    (p.ty, p.dim)
                }
                Op::Const { ty, dim, .. } => (ty, dim),
                Op::Add(a, b) | Op::Sub(a, b) | Op::Min(a, b) | Op::Max(a, b) => {
                    let (ta, da) = check_ref(a)?;
                    let (tb, db) = check_ref(b)?;
                    if ta != tb || ta == Ty::Bool {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "numeric operand type mismatch",
                        ));
                    }
                    if da != db {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "inhomogeneous sum/difference/min/max",
                        ));
                    }
                    (ta, da)
                }
                Op::Mul(a, b) => {
                    let (ta, da) = check_ref(a)?;
                    let (tb, db) = check_ref(b)?;
                    if ta != tb || ta == Ty::Bool {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "mul operand type mismatch",
                        ));
                    }
                    let d = da.add(db).ok_or_else(|| {
                        dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "dimension exponent overflow",
                        )
                    })?;
                    (ta, d)
                }
                Op::Div(a, b) => {
                    let (ta, da) = check_ref(a)?;
                    let (tb, db) = check_ref(b)?;
                    if ta != tb || ta == Ty::Bool {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "div operand type mismatch",
                        ));
                    }
                    let d = da.sub(db).ok_or_else(|| {
                        dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "dimension exponent overflow",
                        )
                    })?;
                    (ta, d)
                }
                Op::Sqrt(a) => {
                    let (ta, da) = check_ref(a)?;
                    if ta == Ty::Bool {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            Dim::DIMENSIONLESS,
                            "sqrt on bool",
                        ));
                    }
                    let d = da.halve_even().ok_or_else(|| {
                        dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            Dim::DIMENSIONLESS,
                            "sqrt of odd dimension exponent",
                        )
                    })?;
                    (ta, d)
                }
                Op::Neg(a) | Op::Abs(a) => {
                    let (ta, da) = check_ref(a)?;
                    if ta == Ty::Bool {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            da,
                            "neg/abs on bool",
                        ));
                    }
                    (ta, da)
                }
                Op::Cmp(_, a, b) => {
                    let (ta, da) = check_ref(a)?;
                    let (tb, db) = check_ref(b)?;
                    if ta != tb || ta == Ty::Bool {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "cmp operand type mismatch",
                        ));
                    }
                    if da != db {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "cmp operands have different dimensions",
                        ));
                    }
                    (Ty::Bool, Dim::DIMENSIONLESS)
                }
                Op::Select(p, a, b) => {
                    let (tp, _) = check_ref(p)?;
                    let (ta, da) = check_ref(a)?;
                    let (tb, db) = check_ref(b)?;
                    if tp != Ty::Bool || ta != tb {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "select type mismatch",
                        ));
                    }
                    if da != db {
                        return Err(dim_refusal(
                            &self.name,
                            idx,
                            &node.expr_name,
                            da,
                            db,
                            "select arms have different dimensions",
                        ));
                    }
                    (ta, da)
                }
            };
            if ty != node.ty || dim != node.dim {
                return Err(dim_refusal(
                    &self.name,
                    idx,
                    &node.expr_name,
                    dim,
                    node.dim,
                    "annotated node type/dim disagrees with inferred",
                ));
            }
            inferred_ty.push(ty);
            inferred_dim.push(dim);
        }

        for (port, nid) in &self.outputs {
            if *nid >= self.nodes.len() {
                return Err(dim_refusal(
                    &self.name,
                    *nid,
                    &port.name,
                    Dim::DIMENSIONLESS,
                    port.dim,
                    "output node index out of range",
                ));
            }
            let (ty, dim) = (inferred_ty[*nid], inferred_dim[*nid]);
            if ty != port.ty || dim != port.dim {
                return Err(dim_refusal(
                    &self.name,
                    *nid,
                    &port.name,
                    dim,
                    port.dim,
                    "output port dimension/type mismatch",
                ));
            }
        }
        Ok(())
    }

    /// Canonical `frob.kir/1` byte serialization (D0, identical across Linux and macOS).
    pub fn to_canonical_json(&self) -> Result<CanonicalBytes, CanonError> {
        let mut w = CanonWriter::new();
        w.value()?.object(|o| {
            o.member("inputs", |iv| {
                iv.array(|ia| {
                    for p in &self.inputs {
                        write_port(p, ia)?;
                    }
                    Ok(())
                })
            })?;
            o.member("meta", |mv| {
                mv.object(|mo| {
                    mo.str("cpu_reference", &self.meta.cpu_reference)?;
                    mo.str("determinism", self.meta.determinism.as_str())?;
                    mo.str("math_mode", &self.meta.math_mode)?;
                    mo.member("op_counts", |ov| {
                        let c = self.meta.op_counts;
                        ov.object(|oo| {
                            oo.int53("add_sub", i64::from(c.add_sub))?;
                            oo.int53("cmp", i64::from(c.cmp))?;
                            oo.int53("consts", i64::from(c.consts))?;
                            oo.int53("div", i64::from(c.div))?;
                            oo.int53("inputs", i64::from(c.inputs))?;
                            oo.int53("min_max", i64::from(c.min_max))?;
                            oo.int53("mul", i64::from(c.mul))?;
                            oo.int53("select", i64::from(c.select))?;
                            oo.int53("sqrt", i64::from(c.sqrt))?;
                            oo.int53("unary", i64::from(c.unary))?;
                            Ok(())
                        })
                    })?;
                    Ok(())
                })
            })?;
            o.str("name", &self.name)?;
            o.member("nodes", |nv| {
                nv.array(|na| {
                    for n in &self.nodes {
                        na.push(|ev| {
                            ev.object(|no| {
                                no.member("dim", |dv| write_dim(n.dim, dv))?;
                                no.str("expr_name", &n.expr_name)?;
                                no.str("op", n.op.tag())?;
                                match n.op {
                                    Op::Input { port } => no.int53("port", port as i64)?,
                                    Op::Const { bits, .. } => {
                                        no.str("bits_hex", &format!("{bits:016x}"))?
                                    }
                                    Op::Add(a, b)
                                    | Op::Sub(a, b)
                                    | Op::Mul(a, b)
                                    | Op::Div(a, b)
                                    | Op::Min(a, b)
                                    | Op::Max(a, b) => {
                                        no.member("args", |av| {
                                            av.array(|aa| {
                                                aa.push_int53(a as i64)?;
                                                aa.push_int53(b as i64)?;
                                                Ok(())
                                            })
                                        })?;
                                    }
                                    Op::Sqrt(a) | Op::Neg(a) | Op::Abs(a) => {
                                        no.member("args", |av| {
                                            av.array(|aa| aa.push_int53(a as i64))
                                        })?;
                                    }
                                    Op::Cmp(cop, a, b) => {
                                        no.str("cmp", cop.as_str())?;
                                        no.member("args", |av| {
                                            av.array(|aa| {
                                                aa.push_int53(a as i64)?;
                                                aa.push_int53(b as i64)?;
                                                Ok(())
                                            })
                                        })?;
                                    }
                                    Op::Select(p, a, b) => {
                                        no.member("args", |av| {
                                            av.array(|aa| {
                                                aa.push_int53(p as i64)?;
                                                aa.push_int53(a as i64)?;
                                                aa.push_int53(b as i64)?;
                                                Ok(())
                                            })
                                        })?;
                                    }
                                }
                                no.str("ty", n.ty.as_str())?;
                                Ok(())
                            })
                        })?;
                    }
                    Ok(())
                })
            })?;
            o.member("outputs", |ov| {
                ov.array(|oa| {
                    for (p, nid) in &self.outputs {
                        oa.push(|ev| {
                            ev.object(|po| {
                                po.member("dim", |dv| write_dim(p.dim, dv))?;
                                po.str("name", &p.name)?;
                                po.int53("node", *nid as i64)?;
                                po.str("ty", p.ty.as_str())?;
                                Ok(())
                            })
                        })?;
                    }
                    Ok(())
                })
            })?;
            o.str("schema", KIR_SCHEMA)?;
            Ok(())
        })?;
        w.finish()
    }

    /// Domain-separated BLAKE3 content hash of this `frob.kir/1` program.
    pub fn content_hash(&self) -> Result<Blake3, CanonError> {
        let bytes = self.to_canonical_json()?;
        let schema = SchemaName::parse(KIR_SCHEMA).expect("valid frob.kir/1 schema name");
        Ok(identity(&schema, &bytes))
    }

    /// Parse a `frob.kir/1` program from canonical or admitted JSON bytes.
    pub fn parse_json(bytes: &[u8]) -> Result<Self, String> {
        let adm = admit(bytes, &Limits::default()).map_err(|e| e.to_string())?;
        let root = obj(adm.root(), "program")?;
        let schema = s_req(&root, "schema")?;
        if schema != KIR_SCHEMA {
            return Err(format!("expected schema `{KIR_SCHEMA}`, got `{schema}`"));
        }
        let name = s_req(&root, "name")?.to_owned();
        let mut inputs = Vec::new();
        for item in a_req(&root, "inputs")? {
            let po = obj(item, "input")?;
            inputs.push(Port {
                name: s_req(&po, "name")?.to_owned(),
                ty: Ty::parse(s_req(&po, "ty")?)?,
                dim: parse_dim(f_req(&po, "dim")?)?,
            });
        }
        let mut outputs = Vec::new();
        for item in a_req(&root, "outputs")? {
            let po = obj(item, "output")?;
            let nid = u_req(&po, "node")?;
            outputs.push((
                Port {
                    name: s_req(&po, "name")?.to_owned(),
                    ty: Ty::parse(s_req(&po, "ty")?)?,
                    dim: parse_dim(f_req(&po, "dim")?)?,
                },
                nid,
            ));
        }
        let mut nodes = Vec::new();
        for item in a_req(&root, "nodes")? {
            let no = obj(item, "node")?;
            let ty = Ty::parse(s_req(&no, "ty")?)?;
            let dim = parse_dim(f_req(&no, "dim")?)?;
            let expr_name = s_req(&no, "expr_name")?.to_owned();
            let op = match s_req(&no, "op")? {
                "input" => Op::Input {
                    port: u_req(&no, "port")?,
                },
                "const" => {
                    let hex = s_req(&no, "bits_hex")?;
                    let bits =
                        u64::from_str_radix(hex, 16).map_err(|e| format!("bits_hex: {e}"))?;
                    Op::Const { bits, ty, dim }
                }
                "add" => {
                    let a = args2(&no)?;
                    Op::Add(a.0, a.1)
                }
                "sub" => {
                    let a = args2(&no)?;
                    Op::Sub(a.0, a.1)
                }
                "mul" => {
                    let a = args2(&no)?;
                    Op::Mul(a.0, a.1)
                }
                "div" => {
                    let a = args2(&no)?;
                    Op::Div(a.0, a.1)
                }
                "min" => {
                    let a = args2(&no)?;
                    Op::Min(a.0, a.1)
                }
                "max" => {
                    let a = args2(&no)?;
                    Op::Max(a.0, a.1)
                }
                "sqrt" => Op::Sqrt(args1(&no)?),
                "neg" => Op::Neg(args1(&no)?),
                "abs" => Op::Abs(args1(&no)?),
                "cmp" => {
                    let cop = CmpOp::parse(s_req(&no, "cmp")?)?;
                    let a = args2(&no)?;
                    Op::Cmp(cop, a.0, a.1)
                }
                "select" => {
                    let arr = a_req(&no, "args")?;
                    if arr.len() != 3 {
                        return Err("select requires 3 args".into());
                    }
                    Op::Select(sp_usize(&arr[0])?, sp_usize(&arr[1])?, sp_usize(&arr[2])?)
                }
                other => return Err(format!("unknown op `{other}`")),
            };
            nodes.push(Node {
                op,
                ty,
                dim,
                expr_name,
            });
        }
        let mo = obj(f_req(&root, "meta")?, "meta")?;
        let math_mode = s_req(&mo, "math_mode")?.to_owned();
        let cpu_reference = s_req(&mo, "cpu_reference")?.to_owned();
        let det_s = s_req(&mo, "determinism")?;
        let determinism = DetClass::parse(det_s).ok_or_else(|| format!("bad det `{det_s}`"))?;
        let op_counts = OpCounts::from_nodes(&nodes);
        Ok(Self {
            name,
            inputs,
            outputs,
            nodes,
            meta: ProgramMeta {
                math_mode,
                op_counts,
                cpu_reference,
                determinism,
            },
        })
    }

    /// Bit-exact reference interpreter for `F32`/`Bool` programs.
    pub fn eval_f32(&self, inputs: &[f32]) -> Result<Vec<f32>, String> {
        if inputs.len() != self.inputs.len() {
            return Err(format!(
                "expected {} inputs, got {}",
                self.inputs.len(),
                inputs.len()
            ));
        }
        #[derive(Clone, Copy)]
        enum Val {
            F32(f32),
            Bool(bool),
        }
        let mut vals: Vec<Val> = Vec::with_capacity(self.nodes.len());
        let get_f = |vals: &[Val], i: NodeId| -> Result<f32, String> {
            match vals[i] {
                Val::F32(v) => Ok(v),
                Val::Bool(_) => Err("expected f32".into()),
            }
        };
        let get_b = |vals: &[Val], i: NodeId| -> Result<bool, String> {
            match vals[i] {
                Val::Bool(v) => Ok(v),
                Val::F32(_) => Err("expected bool".into()),
            }
        };
        for n in &self.nodes {
            let v = match n.op {
                Op::Input { port } => Val::F32(inputs[port]),
                Op::Const {
                    bits, ty: Ty::F32, ..
                } => Val::F32(f32::from_bits(bits as u32)),
                Op::Const {
                    bits, ty: Ty::Bool, ..
                } => Val::Bool(bits != 0),
                Op::Const { ty: Ty::F64, .. } => {
                    return Err("eval_f32 does not run f64 constants".into());
                }
                Op::Add(a, b) => Val::F32(get_f(&vals, a)? + get_f(&vals, b)?),
                Op::Sub(a, b) => Val::F32(get_f(&vals, a)? - get_f(&vals, b)?),
                Op::Mul(a, b) => Val::F32(get_f(&vals, a)? * get_f(&vals, b)?),
                Op::Div(a, b) => Val::F32(get_f(&vals, a)? / get_f(&vals, b)?),
                Op::Sqrt(a) => Val::F32(get_f(&vals, a)?.sqrt()),
                Op::Neg(a) => Val::F32(-get_f(&vals, a)?),
                Op::Abs(a) => Val::F32(get_f(&vals, a)?.abs()),
                Op::Min(a, b) => Val::F32(ieee_min_f32(get_f(&vals, a)?, get_f(&vals, b)?)),
                Op::Max(a, b) => Val::F32(ieee_max_f32(get_f(&vals, a)?, get_f(&vals, b)?)),
                Op::Cmp(op, a, b) => {
                    let (x, y) = (get_f(&vals, a)?, get_f(&vals, b)?);
                    let r = match op {
                        CmpOp::Lt => x < y,
                        CmpOp::Le => x <= y,
                        CmpOp::Eq => x == y,
                        CmpOp::Ne => x != y,
                        CmpOp::Ge => x >= y,
                        CmpOp::Gt => x > y,
                    };
                    Val::Bool(r)
                }
                Op::Select(p, a, b) => {
                    let cond = get_b(&vals, p)?;
                    let (x, y) = (get_f(&vals, a)?, get_f(&vals, b)?);
                    Val::F32(if cond { x } else { y })
                }
            };
            vals.push(v);
        }
        let mut out = Vec::with_capacity(self.outputs.len());
        for (_, nid) in &self.outputs {
            out.push(get_f(&vals, *nid)?);
        }
        Ok(out)
    }
}

/// Explicit NaN-propagating, signed-zero-exact `min(a, b)`:
/// NaN in either operand yields NaN; `min(-0.0, +0.0) = -0.0`.
#[inline]
#[must_use]
pub fn ieee_min_f32(a: f32, b: f32) -> f32 {
    if a != a {
        a
    } else if b != b {
        b
    } else if a < b {
        a
    } else if b < a {
        b
    } else if a.to_bits() == (-0.0_f32).to_bits() {
        a
    } else {
        b
    }
}

/// Explicit NaN-propagating, signed-zero-exact `max(a, b)`:
/// NaN in either operand yields NaN; `max(-0.0, +0.0) = +0.0`.
#[inline]
#[must_use]
pub fn ieee_max_f32(a: f32, b: f32) -> f32 {
    if a != a {
        a
    } else if b != b {
        b
    } else if a > b {
        a
    } else if b > a {
        b
    } else if a.to_bits() == (0.0_f32).to_bits() {
        a
    } else {
        b
    }
}

fn write_dim(d: Dim, v: frob_core::canon::Value<'_>) -> Result<(), CanonError> {
    v.array(|a| {
        for x in d.0 {
            a.push_int53(i64::from(x))?;
        }
        Ok(())
    })
}

fn write_port(p: &Port, arr: &mut frob_core::canon::Array<'_>) -> Result<(), CanonError> {
    arr.push(|ev| {
        ev.object(|po| {
            po.member("dim", |dv| write_dim(p.dim, dv))?;
            po.str("name", &p.name)?;
            po.str("ty", p.ty.as_str())?;
            Ok(())
        })
    })
}

fn obj<'a>(sp: &'a Spanned<'a>, ctx: &str) -> Result<BTreeMap<&'a str, &'a Spanned<'a>>, String> {
    let ms = sp
        .value
        .as_object()
        .ok_or_else(|| format!("{ctx} must be object"))?;
    let mut m = BTreeMap::new();
    for mem in ms {
        m.insert(mem.name.as_str(), &mem.value);
    }
    Ok(m)
}

fn f_req<'a>(m: &BTreeMap<&'a str, &'a Spanned<'a>>, k: &str) -> Result<&'a Spanned<'a>, String> {
    m.get(k)
        .copied()
        .ok_or_else(|| format!("missing field `{k}`"))
}

fn s_req<'a>(m: &BTreeMap<&'a str, &'a Spanned<'a>>, k: &str) -> Result<&'a str, String> {
    f_req(m, k)?
        .value
        .as_str()
        .ok_or_else(|| format!("`{k}` must be string"))
}

fn a_req<'a>(m: &BTreeMap<&'a str, &'a Spanned<'a>>, k: &str) -> Result<&'a [Spanned<'a>], String> {
    f_req(m, k)?
        .value
        .as_array()
        .ok_or_else(|| format!("`{k}` must be array"))
}

fn sp_usize(sp: &Spanned<'_>) -> Result<usize, String> {
    let n = sp.as_int53().map_err(|e| e.to_string())?;
    usize::try_from(n).map_err(|_| format!("negative index {n}"))
}

fn u_req(m: &BTreeMap<&str, &Spanned<'_>>, k: &str) -> Result<usize, String> {
    sp_usize(f_req(m, k)?)
}

fn args1(m: &BTreeMap<&str, &Spanned<'_>>) -> Result<NodeId, String> {
    let arr = a_req(m, "args")?;
    if arr.len() != 1 {
        return Err("expected 1 arg".into());
    }
    sp_usize(&arr[0])
}

fn args2(m: &BTreeMap<&str, &Spanned<'_>>) -> Result<(NodeId, NodeId), String> {
    let arr = a_req(m, "args")?;
    if arr.len() != 2 {
        return Err("expected 2 args".into());
    }
    Ok((sp_usize(&arr[0])?, sp_usize(&arr[1])?))
}

fn parse_dim(sp: &Spanned<'_>) -> Result<Dim, String> {
    let arr = sp
        .value
        .as_array()
        .ok_or_else(|| "dim must be array".to_owned())?;
    if arr.len() != DIM_LEN {
        return Err(format!("dim length must be {DIM_LEN}"));
    }
    let mut out = [0i8; DIM_LEN];
    for (slot, item) in out.iter_mut().zip(arr) {
        let n = item.as_int53().map_err(|e| e.to_string())?;
        *slot = i8::try_from(n).map_err(|_| format!("dim out of i8: {n}"))?;
    }
    Ok(Dim(out))
}
