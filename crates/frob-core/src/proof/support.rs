//! Assumptions, polarity axes, and dependency-support records (`N25b-01`, rule 7).
//!
//! Zero floating-point fields (`f32` / `f64`), zero `unsafe`, determinism D0.

use crate::id::Blake3;
use crate::proof::num::{CanonicalEncoder, Interval, ProofError, Ratio};
use crate::proof::qty_types::FrameSpec;

/// Axis along which an assumption acts (`N25b-01`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    World,
    Policy,
    Acceptance,
}

impl Axis {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Policy => "policy",
            Self::Acceptance => "acceptance",
        }
    }
}

/// Direction of assumption relaxation under which a certificate survives (`C-18`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Polarity {
    Grow,
    Shrink,
}

impl Polarity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Grow => "grow",
            Self::Shrink => "shrink",
        }
    }
}

/// Exact value attached to a proof assumption (no floating-point variants).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssumptionValue {
    Ratio(Ratio),
    Interval(Interval),
    Text(String),
    Bool(bool),
    Digest(Blake3),
}

impl AssumptionValue {
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::Ratio(r) => { enc.tag(b"av_rat"); r.encode_canonical(enc); }
            Self::Interval(iv) => { enc.tag(b"av_iv"); iv.encode_canonical(enc); }
            Self::Text(s) => { enc.tag(b"av_txt"); enc.str(s); }
            Self::Bool(b) => { enc.tag(b"av_bool"); enc.bool(*b); }
            Self::Digest(d) => { enc.tag(b"av_b3"); enc.blake3(*d); }
        }
    }
}

/// Named assumption carried by a proof object with its axis and survival polarity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Assumption {
    id: String,
    value: AssumptionValue,
    axis: Axis,
    survives: Polarity,
}

impl Assumption {
    pub fn new(
        id: impl Into<String>,
        value: AssumptionValue,
        axis: Axis,
        survives: Polarity,
    ) -> Result<Self, ProofError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "assumption.id" });
        }
        Ok(Self { id, value, axis, survives })
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn value(&self) -> &AssumptionValue {
        &self.value
    }

    #[must_use]
    pub const fn axis(&self) -> Axis {
        self.axis
    }

    #[must_use]
    pub const fn survives(&self) -> Polarity {
        self.survives
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"assumption");
        enc.str(&self.id);
        self.value.encode_canonical(enc);
        enc.str(self.axis.as_str());
        enc.str(self.survives.as_str());
    }
}

/// Non-empty field path naming a parameter or model field used by a certificate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldPath(String);

impl FieldPath {
    pub fn new(path: impl Into<String>) -> Result<Self, ProofError> {
        let path = path.into();
        if path.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "field_path" });
        }
        Ok(Self(path))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.str(&self.0);
    }
}

/// Implicit dependency used by a certificate without naming a parameter (`N25b-01` rule 7).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplicitDep {
    NoiseModel(String),
    GravityFrame(FrameSpec),
    CalibratedRenderer(String),
    TorqueDerating(String),
    PreprocessingGraph(String),
    Clock(String),
    Card(String),
    ModelVersion(String),
}

impl ImplicitDep {
    pub fn validate_shape(&self) -> Result<(), ProofError> {
        match self {
            Self::GravityFrame(_) => Ok(()),
            Self::NoiseModel(s)
            | Self::CalibratedRenderer(s)
            | Self::TorqueDerating(s)
            | Self::PreprocessingGraph(s)
            | Self::Clock(s)
            | Self::Card(s)
            | Self::ModelVersion(s) => {
                if s.trim().is_empty() {
                    Err(ProofError::EmptyIdentifier { field: "implicit_dep" })
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::NoiseModel(s) => { enc.tag(b"imp_noise"); enc.str(s); }
            Self::GravityFrame(f) => { enc.tag(b"imp_grav"); f.encode_canonical(enc); }
            Self::CalibratedRenderer(s) => { enc.tag(b"imp_rend"); enc.str(s); }
            Self::TorqueDerating(s) => { enc.tag(b"imp_derate"); enc.str(s); }
            Self::PreprocessingGraph(s) => { enc.tag(b"imp_prep"); enc.str(s); }
            Self::Clock(s) => { enc.tag(b"imp_clock"); enc.str(s); }
            Self::Card(s) => { enc.tag(b"imp_card"); enc.str(s); }
            Self::ModelVersion(s) => { enc.tag(b"imp_mver"); enc.str(s); }
        }
    }
}

/// Dependency support actually used by a proof object (`N25b-01` rule 7).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Support {
    fields: Vec<FieldPath>,
    implicit: Vec<ImplicitDep>,
}

impl Support {
    pub fn new(fields: Vec<FieldPath>, implicit: Vec<ImplicitDep>) -> Result<Self, ProofError> {
        for dep in &implicit {
            dep.validate_shape()?;
        }
        Ok(Self { fields, implicit })
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self { fields: Vec::new(), implicit: Vec::new() }
    }

    #[must_use]
    pub fn fields(&self) -> &[FieldPath] {
        &self.fields
    }

    #[must_use]
    pub fn implicit(&self) -> &[ImplicitDep] {
        &self.implicit
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty() && self.implicit.is_empty()
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"support");
        enc.usize(self.fields.len());
        for f in &self.fields {
            f.encode_canonical(enc);
        }
        enc.usize(self.implicit.len());
        for d in &self.implicit {
            d.encode_canonical(enc);
        }
    }
}
