//! Plain claim-context types (`Establishes`, `Binder`, `PolicyScope`, `Horizon`, `Cell`)
//! owned by `frob-core` at layer 0 (`N25b-01`, rule 5).
//!
//! Zero floating-point fields (`f32` / `f64`), zero `unsafe`, determinism D0.

use crate::id::Blake3;
use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};
use crate::proof::qty_types::{ExactQty, Unit};

macro_rules! define_plain_name {
    ($($name:ident => $field:literal),+ $(,)?) => {$(
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ProofError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(ProofError::EmptyIdentifier { field: $field });
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
                enc.str(&self.0);
            }
        }
    )+};
}

define_plain_name! {
    ProofRef => "proof_ref",
    CellRef => "cell_ref",
    Var => "var",
    PopulationId => "population_id",
    TopologyId => "topology_id",
    ChartId => "chart_id",
    ModeId => "mode_id",
}

/// Content digest of the admitted policy class `Pi_Y`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyClassDigest(pub Blake3);

impl PolicyClassDigest {
    #[must_use]
    pub const fn new(digest: Blake3) -> Self {
        Self(digest)
    }

    #[must_use]
    pub const fn digest(self) -> Blake3 {
        self.0
    }
}

/// Scope of an existential policy quantifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PolicyScope {
    PerNominalDesign,
    PerCell,
    Family,
}

impl PolicyScope {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerNominalDesign => "per-nominal-design",
            Self::PerCell => "per-cell",
            Self::Family => "family",
        }
    }
}

/// One quantifier binder in `Establishes.prefix`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Binder {
    Exists {
        var: Var,
        scope: PolicyScope,
    },
    ForAll {
        var: Var,
    },
    Prob {
        var: Var,
        population: PopulationId,
        at_least: Ratio,
        confidence: Ratio,
    },
}

impl Binder {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        if let Self::Prob { at_least, confidence, .. } = self {
            let z = Ratio::zero();
            let one = Ratio::one();
            if *at_least < z || *at_least > one {
                return Err(ProofError::InvalidCell {
                    detail: format!("prob binder at_least={at_least} outside [0, 1]"),
                });
            }
            if *confidence < z || *confidence > one {
                return Err(ProofError::InvalidCell {
                    detail: format!("prob binder confidence={confidence} outside [0, 1]"),
                });
            }
        }
        Ok(())
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::Exists { var, scope } => {
                enc.tag(b"exists");
                var.encode_canonical(enc);
                enc.str(scope.as_str());
            }
            Self::ForAll { var } => {
                enc.tag(b"forall");
                var.encode_canonical(enc);
            }
            Self::Prob { var, population, at_least, confidence } => {
                enc.tag(b"prob");
                var.encode_canonical(enc);
                population.encode_canonical(enc);
                at_least.encode_canonical(enc);
                confidence.encode_canonical(enc);
            }
        }
    }
}

/// Time or termination horizon established by a proof object.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Horizon {
    Static,
    Time(ExactQty),
    Ticks(u64),
    TerminalModes(Vec<ModeId>),
}

impl Horizon {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        match self {
            Self::Static | Self::Ticks(_) => Ok(()),
            Self::Time(q) => {
                if q.value().is_neg() {
                    return Err(ProofError::InvalidCell {
                        detail: format!("horizon time must be non-negative, got {}", q.value()),
                    });
                }
                Ok(())
            }
            Self::TerminalModes(modes) => {
                if modes.is_empty() {
                    return Err(ProofError::InvalidCell {
                        detail: "TerminalModes horizon must list at least one mode".to_owned(),
                    });
                }
                Ok(())
            }
        }
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::Static => enc.tag(b"static"),
            Self::Time(q) => {
                enc.tag(b"time");
                q.encode_canonical(enc);
            }
            Self::Ticks(n) => {
                enc.tag(b"ticks");
                enc.u64(*n);
            }
            Self::TerminalModes(modes) => {
                enc.tag(b"modes");
                enc.usize(modes.len());
                for m in modes {
                    m.encode_canonical(enc);
                }
            }
        }
    }
}

/// Endpoint openness of a parameter bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundEnd {
    Open,
    Closed,
}

impl BoundEnd {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

/// Parameter interval bound with explicit open/closed ends over exact rationals.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bound {
    var: Var,
    lower: Ratio,
    lower_end: BoundEnd,
    upper: Ratio,
    upper_end: BoundEnd,
}

impl Bound {
    pub fn new(
        var: Var,
        lower: Ratio,
        lower_end: BoundEnd,
        upper: Ratio,
        upper_end: BoundEnd,
    ) -> Result<Self, ProofError> {
        if lower > upper {
            return Err(ProofError::InvalidCell {
                detail: format!("bound for `{}` has lower {lower} > upper {upper}", var.as_str()),
            });
        }
        if lower == upper && (lower_end == BoundEnd::Open || upper_end == BoundEnd::Open) {
            return Err(ProofError::InvalidCell {
                detail: format!("degenerate bound for `{}` must have closed ends", var.as_str()),
            });
        }
        Ok(Self { var, lower, lower_end, upper, upper_end })
    }

    #[must_use]
    pub const fn var(&self) -> &Var {
        &self.var
    }

    #[must_use]
    pub const fn lower(&self) -> Ratio {
        self.lower
    }

    #[must_use]
    pub const fn lower_end(&self) -> BoundEnd {
        self.lower_end
    }

    #[must_use]
    pub const fn upper(&self) -> Ratio {
        self.upper
    }

    #[must_use]
    pub const fn upper_end(&self) -> BoundEnd {
        self.upper_end
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"bound");
        self.var.encode_canonical(enc);
        self.lower.encode_canonical(enc);
        enc.str(self.lower_end.as_str());
        self.upper.encode_canonical(enc);
        enc.str(self.upper_end.as_str());
    }
}

/// Plain cell description: topology, chart, exact parameter bounds, and matching units.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    topology: TopologyId,
    chart: ChartId,
    bounds: Vec<Bound>,
    units: Vec<Unit>,
}

impl Cell {
    pub fn new(
        topology: TopologyId,
        chart: ChartId,
        bounds: Vec<Bound>,
        units: Vec<Unit>,
    ) -> Result<Self, ProofError> {
        if bounds.len() != units.len() {
            return Err(ProofError::InvalidCell {
                detail: format!(
                    "bounds length ({}) must equal units length ({})",
                    bounds.len(),
                    units.len()
                ),
            });
        }
        Ok(Self { topology, chart, bounds, units })
    }

    #[must_use]
    pub const fn topology(&self) -> &TopologyId {
        &self.topology
    }

    #[must_use]
    pub const fn chart(&self) -> &ChartId {
        &self.chart
    }

    #[must_use]
    pub fn bounds(&self) -> &[Bound] {
        &self.bounds
    }

    #[must_use]
    pub fn units(&self) -> &[Unit] {
        &self.units
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"cell");
        self.topology.encode_canonical(enc);
        self.chart.encode_canonical(enc);
        enc.usize(self.bounds.len());
        for b in &self.bounds {
            b.encode_canonical(enc);
        }
        enc.usize(self.units.len());
        for u in &self.units {
            u.encode_canonical(enc);
        }
    }
}

/// What a proof object establishes; compared against the mission query by the auditor.
///
/// Intentionally does NOT implement `Default` (`N25b-01` rule 5): callers must pass
/// `prefix` explicitly from the query being answered.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Establishes {
    prefix: Vec<Binder>,
    horizon: Horizon,
    policy_class: PolicyClassDigest,
    cell: Cell,
}

impl Establishes {
    pub fn new(
        prefix: Vec<Binder>,
        horizon: Horizon,
        policy_class: PolicyClassDigest,
        cell: Cell,
    ) -> Result<Self, ProofError> {
        if prefix.is_empty() {
            return Err(ProofError::EmptyPrefix);
        }
        for b in &prefix {
            b.validate_shape()?;
        }
        horizon.validate_shape()?;
        Ok(Self { prefix, horizon, policy_class, cell })
    }

    #[must_use]
    pub fn prefix(&self) -> &[Binder] {
        &self.prefix
    }

    #[must_use]
    pub const fn horizon(&self) -> &Horizon {
        &self.horizon
    }

    #[must_use]
    pub const fn policy_class(&self) -> PolicyClassDigest {
        self.policy_class
    }

    #[must_use]
    pub const fn cell(&self) -> &Cell {
        &self.cell
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"establishes");
        enc.usize(self.prefix.len());
        for b in &self.prefix {
            b.encode_canonical(enc);
        }
        self.horizon.encode_canonical(enc);
        enc.blake3(self.policy_class.digest());
        self.cell.encode_canonical(enc);
    }
}
