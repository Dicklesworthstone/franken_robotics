//! Canonical expression DAG and independently reviewable primitive equations (`N25b-01`, rule 6).
//!
//! Node kinds: exact constant, typed variable, sum, product, negation, quotient,
//! nonnegative integer power, square root, absolute value, minimum, maximum.
//! Digests are BLAKE3 over a canonical post-order encoding rooted at `root`, so two
//! producers that build structurally identical expressions in different node insertion
//! orders produce identical digests.

use std::collections::BTreeMap;

use crate::id::Blake3;
use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};
use crate::proof::qty_types::Unit;

/// Index of a node inside an [`ExprDag`] arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub usize);

/// Expression DAG node (`N25b-01` rule 6).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExprNode {
    Const(Ratio),
    Var { name: String, unit: Unit },
    Sum(Vec<NodeId>),
    Product(Vec<NodeId>),
    Neg(NodeId),
    Quotient(NodeId, NodeId),
    Pow(NodeId, u32),
    Sqrt(NodeId),
    Abs(NodeId),
    Min(Vec<NodeId>),
    Max(Vec<NodeId>),
}

/// Acyclic directed expression graph with canonical post-order BLAKE3 digest.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprDag {
    nodes: Vec<ExprNode>,
    root: NodeId,
}

impl ExprDag {
    /// Construct and validate an expression DAG: every child index must strictly precede
    /// its parent (`child < parent`, enforcing acyclicity), variable names must be non-empty,
    /// and variadic nodes (`Sum`, `Product`, `Min`, `Max`) must be non-empty.
    pub fn new(nodes: Vec<ExprNode>, root: NodeId) -> Result<Self, ProofError> {
        if nodes.is_empty() || root.0 >= nodes.len() {
            return Err(ProofError::InvalidExprDag {
                detail: "DAG must be non-empty with root in bounds".to_owned(),
            });
        }
        for (idx, node) in nodes.iter().enumerate() {
            match node {
                ExprNode::Const(_) => {}
                ExprNode::Var { name, .. } => {
                    if name.trim().is_empty() {
                        return Err(ProofError::InvalidExprDag {
                            detail: format!("node {idx} has empty variable name"),
                        });
                    }
                }
                ExprNode::Sum(cs) | ExprNode::Product(cs) | ExprNode::Min(cs) | ExprNode::Max(cs) => {
                    if cs.is_empty() {
                        return Err(ProofError::InvalidExprDag {
                            detail: format!("variadic node {idx} has zero children"),
                        });
                    }
                    for c in cs {
                        if c.0 >= idx {
                            return Err(ProofError::InvalidExprDag {
                                detail: format!("node {idx} references non-preceding child {}", c.0),
                            });
                        }
                    }
                }
                ExprNode::Neg(a) | ExprNode::Pow(a, _) | ExprNode::Sqrt(a) | ExprNode::Abs(a) => {
                    if a.0 >= idx {
                        return Err(ProofError::InvalidExprDag {
                            detail: format!("node {idx} references non-preceding child {}", a.0),
                        });
                    }
                }
                ExprNode::Quotient(a, b) => {
                    if a.0 >= idx || b.0 >= idx {
                        return Err(ProofError::InvalidExprDag {
                            detail: format!("quotient node {idx} references non-preceding child"),
                        });
                    }
                }
            }
        }
        Ok(Self { nodes, root })
    }

    #[must_use]
    pub fn nodes(&self) -> &[ExprNode] {
        &self.nodes
    }

    #[must_use]
    pub const fn root(&self) -> NodeId {
        self.root
    }

    /// Canonical post-order encoding rooted at `self.root` (independent of arena insertion
    /// order and dead unreachable nodes; deduplicates structurally equal sub-expressions).
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut enc = CanonicalEncoder::new();
        self.encode_canonical(&mut enc);
        enc.finish()
    }

    /// Domain-separated BLAKE3 digest of the canonical post-order encoding (`frob.proof.expr/1`).
    #[must_use]
    pub fn digest(&self) -> Blake3 {
        let bytes = self.canonical_bytes();
        Blake3::from_bytes(*fs_blake3::hash_domain("frob.proof.expr/1", &bytes).as_bytes())
    }

    /// Emit canonical post-order bytes into `enc`.
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        let mut post_nodes: Vec<CanonNode> = Vec::new();
        let mut structural_intern: BTreeMap<CanonNode, usize> = BTreeMap::new();
        let mut memo: BTreeMap<usize, usize> = BTreeMap::new();
        let root_id = self.canon_visit(
            self.root.0,
            &mut memo,
            &mut structural_intern,
            &mut post_nodes,
        );

        enc.tag(b"expr_dag/1");
        enc.usize(post_nodes.len());
        for node in &post_nodes {
            node.encode(enc);
        }
        enc.usize(root_id);
    }

    fn canon_visit(
        &self,
        idx: usize,
        memo: &mut BTreeMap<usize, usize>,
        intern: &mut BTreeMap<CanonNode, usize>,
        out: &mut Vec<CanonNode>,
    ) -> usize {
        if let Some(&cid) = memo.get(&idx) {
            return cid;
        }
        let Some(node) = self.nodes.get(idx) else {
            return 0;
        };
        let canon = match node {
            ExprNode::Const(r) => CanonNode::Const(*r),
            ExprNode::Var { name, unit } => CanonNode::Var(name.clone(), unit.as_str().to_owned()),
            ExprNode::Sum(cs) => {
                let kids = cs.iter().map(|c| self.canon_visit(c.0, memo, intern, out)).collect();
                CanonNode::Sum(kids)
            }
            ExprNode::Product(cs) => {
                let kids = cs.iter().map(|c| self.canon_visit(c.0, memo, intern, out)).collect();
                CanonNode::Product(kids)
            }
            ExprNode::Neg(a) => CanonNode::Neg(self.canon_visit(a.0, memo, intern, out)),
            ExprNode::Quotient(a, b) => {
                let ca = self.canon_visit(a.0, memo, intern, out);
                let cb = self.canon_visit(b.0, memo, intern, out);
                CanonNode::Quotient(ca, cb)
            }
            ExprNode::Pow(a, e) => CanonNode::Pow(self.canon_visit(a.0, memo, intern, out), *e),
            ExprNode::Sqrt(a) => CanonNode::Sqrt(self.canon_visit(a.0, memo, intern, out)),
            ExprNode::Abs(a) => CanonNode::Abs(self.canon_visit(a.0, memo, intern, out)),
            ExprNode::Min(cs) => {
                let kids = cs.iter().map(|c| self.canon_visit(c.0, memo, intern, out)).collect();
                CanonNode::Min(kids)
            }
            ExprNode::Max(cs) => {
                let kids = cs.iter().map(|c| self.canon_visit(c.0, memo, intern, out)).collect();
                CanonNode::Max(kids)
            }
        };
        let cid = if let Some(&existing) = intern.get(&canon) {
            existing
        } else {
            let next = out.len();
            intern.insert(canon.clone(), next);
            out.push(canon);
            next
        };
        memo.insert(idx, cid);
        cid
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CanonNode {
    Const(Ratio),
    Var(String, String),
    Sum(Vec<usize>),
    Product(Vec<usize>),
    Neg(usize),
    Quotient(usize, usize),
    Pow(usize, u32),
    Sqrt(usize),
    Abs(usize),
    Min(Vec<usize>),
    Max(Vec<usize>),
}

impl CanonNode {
    fn encode(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::Const(r) => {
                enc.tag(b"c");
                r.encode_canonical(enc);
            }
            Self::Var(name, unit) => {
                enc.tag(b"v");
                enc.str(name);
                enc.str(unit);
            }
            Self::Sum(cs) => {
                enc.tag(b"sum");
                enc.usize(cs.len());
                for &c in cs { enc.usize(c); }
            }
            Self::Product(cs) => {
                enc.tag(b"prod");
                enc.usize(cs.len());
                for &c in cs { enc.usize(c); }
            }
            Self::Neg(a) => { enc.tag(b"neg"); enc.usize(*a); }
            Self::Quotient(a, b) => { enc.tag(b"quot"); enc.usize(*a); enc.usize(*b); }
            Self::Pow(a, e) => { enc.tag(b"pow"); enc.usize(*a); enc.u32(*e); }
            Self::Sqrt(a) => { enc.tag(b"sqrt"); enc.usize(*a); }
            Self::Abs(a) => { enc.tag(b"abs"); enc.usize(*a); }
            Self::Min(cs) => {
                enc.tag(b"min");
                enc.usize(cs.len());
                for &c in cs { enc.usize(c); }
            }
            Self::Max(cs) => {
                enc.tag(b"max");
                enc.usize(cs.len());
                for &c in cs { enc.usize(c); }
            }
        }
    }
}

/// Builder helper for constructing valid acyclic [`ExprDag`] instances ergonomically.
#[derive(Debug, Clone, Default)]
pub struct ExprBuilder {
    nodes: Vec<ExprNode>,
}

impl ExprBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn push(&mut self, node: ExprNode) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(node);
        id
    }

    pub fn constant(&mut self, r: Ratio) -> NodeId {
        self.push(ExprNode::Const(r))
    }

    pub fn var(&mut self, name: impl Into<String>, unit: Unit) -> NodeId {
        self.push(ExprNode::Var { name: name.into(), unit })
    }

    pub fn sum(&mut self, items: Vec<NodeId>) -> NodeId {
        self.push(ExprNode::Sum(items))
    }

    pub fn product(&mut self, items: Vec<NodeId>) -> NodeId {
        self.push(ExprNode::Product(items))
    }

    pub fn neg(&mut self, a: NodeId) -> NodeId {
        self.push(ExprNode::Neg(a))
    }

    pub fn quotient(&mut self, a: NodeId, b: NodeId) -> NodeId {
        self.push(ExprNode::Quotient(a, b))
    }

    pub fn pow(&mut self, a: NodeId, exp: u32) -> NodeId {
        self.push(ExprNode::Pow(a, exp))
    }

    pub fn sqrt(&mut self, a: NodeId) -> NodeId {
        self.push(ExprNode::Sqrt(a))
    }

    pub fn abs(&mut self, a: NodeId) -> NodeId {
        self.push(ExprNode::Abs(a))
    }

    pub fn min(&mut self, items: Vec<NodeId>) -> NodeId {
        self.push(ExprNode::Min(items))
    }

    pub fn max(&mut self, items: Vec<NodeId>) -> NodeId {
        self.push(ExprNode::Max(items))
    }

    pub fn build(self, root: NodeId) -> Result<ExprDag, ProofError> {
        ExprDag::new(self.nodes, root)
    }
}

/// Independently reviewable primitive equation in Unicode text and canonical [`ExprDag`] form.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Equation {
    text: String,
    dag: ExprDag,
}

impl Equation {
    pub fn new(text: impl Into<String>, dag: ExprDag) -> Result<Self, ProofError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "equation.text" });
        }
        Ok(Self { text, dag })
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub const fn dag(&self) -> &ExprDag {
        &self.dag
    }

    #[must_use]
    pub fn dag_digest(&self) -> Blake3 {
        self.dag.digest()
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"eq");
        enc.str(&self.text);
        self.dag.encode_canonical(enc);
    }
}
