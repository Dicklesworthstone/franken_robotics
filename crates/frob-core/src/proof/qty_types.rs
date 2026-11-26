//! Physical units, frame/sign conventions, distinct `FirstMoment` vs `TorqueVec`
//! types, and port-bound force vectors for proof objects (`N25b-01`, rules 2 & 3).
//!
//! Zero floating-point fields (`f32` / `f64`), zero `unsafe`, determinism D0.

use std::collections::BTreeSet;
use std::marker::PhantomData;

use crate::proof::num::{CanonicalEncoder, ProofError, Ratio};

/// Physical unit symbol carried on exact proof quantities (e.g. `"m"`, `"N"`, `"N·m"`, `"s"`, `"1"`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unit {
    symbol: String,
}

impl Unit {
    pub fn new(symbol: impl Into<String>) -> Result<Self, ProofError> {
        let symbol = symbol.into();
        if symbol.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "unit" });
        }
        Ok(Self { symbol })
    }

    #[must_use]
    pub fn meters() -> Self {
        Self { symbol: "m".to_owned() }
    }

    #[must_use]
    pub fn newtons() -> Self {
        Self { symbol: "N".to_owned() }
    }

    #[must_use]
    pub fn newton_meters() -> Self {
        Self { symbol: "N·m".to_owned() }
    }

    #[must_use]
    pub fn seconds() -> Self {
        Self { symbol: "s".to_owned() }
    }

    #[must_use]
    pub fn dimensionless() -> Self {
        Self { symbol: "1".to_owned() }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.symbol
    }

    pub(super) fn is_newton_meters(&self) -> bool {
        matches!(self.as_str(), "N·m" | "N*m")
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"unit");
        enc.str(&self.symbol);
    }
}

/// Handedness of a coordinate frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Handedness {
    RightHanded,
    LeftHanded,
}

impl Handedness {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RightHanded => "right-handed",
            Self::LeftHanded => "left-handed",
        }
    }
}

/// Axis orientation convention of a coordinate frame.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AxisConvention {
    ZUp,
    YUp,
    Custom(String),
}

impl AxisConvention {
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::ZUp => enc.tag(b"z-up"),
            Self::YUp => enc.tag(b"y-up"),
            Self::Custom(s) => {
                enc.tag(b"custom-axes");
                enc.str(s);
            }
        }
    }
}

/// Sign convention for rotational and support moments.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SignConvention {
    RightHandScrew,
    SupportFirstMoment,
    Custom(String),
}

impl SignConvention {
    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        match self {
            Self::RightHandScrew => enc.tag(b"rh-screw"),
            Self::SupportFirstMoment => enc.tag(b"support-first-moment"),
            Self::Custom(s) => {
                enc.tag(b"custom-sign");
                enc.str(s);
            }
        }
    }
}

/// Complete physical frame specification: origin, axes, handedness, and sign convention (Rule 2).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameSpec {
    origin: String,
    axes: AxisConvention,
    handedness: Handedness,
    sign_convention: SignConvention,
}

impl FrameSpec {
    pub fn new(
        origin: impl Into<String>,
        axes: AxisConvention,
        handedness: Handedness,
        sign_convention: SignConvention,
    ) -> Result<Self, ProofError> {
        let origin = origin.into();
        if origin.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "frame.origin" });
        }
        Ok(Self { origin, axes, handedness, sign_convention })
    }

    /// Standard right-handed `+z` up world/base support frame.
    #[must_use]
    pub fn right_handed_z_up(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
            axes: AxisConvention::ZUp,
            handedness: Handedness::RightHanded,
            sign_convention: SignConvention::RightHandScrew,
        }
    }

    #[must_use]
    pub fn origin(&self) -> &str {
        &self.origin
    }

    #[must_use]
    pub const fn axes(&self) -> &AxisConvention {
        &self.axes
    }

    #[must_use]
    pub const fn handedness(&self) -> Handedness {
        self.handedness
    }

    #[must_use]
    pub const fn sign_convention(&self) -> &SignConvention {
        &self.sign_convention
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"frame");
        enc.str(&self.origin);
        self.axes.encode_canonical(enc);
        enc.str(self.handedness.as_str());
        self.sign_convention.encode_canonical(enc);
    }
}

/// Exact rational physical quantity tagged with unit and optional frame.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExactQty {
    value: Ratio,
    unit: Unit,
    frame: Option<FrameSpec>,
}

impl ExactQty {
    #[must_use]
    pub const fn new(value: Ratio, unit: Unit, frame: Option<FrameSpec>) -> Self {
        Self { value, unit, frame }
    }

    #[must_use]
    pub const fn value(&self) -> Ratio {
        self.value
    }

    #[must_use]
    pub const fn unit(&self) -> &Unit {
        &self.unit
    }

    #[must_use]
    pub const fn frame(&self) -> Option<&FrameSpec> {
        self.frame.as_ref()
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"eqty");
        self.value.encode_canonical(enc);
        self.unit.encode_canonical(enc);
        match &self.frame {
            Some(fr) => {
                enc.bool(true);
                fr.encode_canonical(enc);
            }
            None => enc.bool(false),
        }
    }
}

/// Phantom tag for a first moment `(sum x_i * f_i, sum y_i * f_i, 0)` (not a torque).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FirstMomentTag;

/// Phantom tag for a 3-axis torque `(tau_x, tau_y, tau_z)` (not a first moment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TorqueTag;

/// Dimensioned, frame-typed 3-vector distinguished at compile time by `Tag`.
///
/// A [`FirstMoment`] and a [`TorqueVec`] are distinct Rust types (Rule 2, CX-108):
///
/// ```compile_fail,E0308
/// use frob_core::proof::{FirstMoment, FrameSpec, Ratio, TorqueVec, Unit};
/// let p = FirstMoment::planar(
///     Ratio::from_int(60),
///     Ratio::from_int(30),
///     Unit::newton_meters(),
///     FrameSpec::right_handed_z_up("base"),
/// );
/// let _wrong: TorqueVec = p;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedMoment3<Tag> {
    x: Ratio,
    y: Ratio,
    z: Ratio,
    unit: Unit,
    frame: FrameSpec,
    _tag: PhantomData<Tag>,
}

impl<Tag> TypedMoment3<Tag> {
    #[must_use]
    pub const fn new(x: Ratio, y: Ratio, z: Ratio, unit: Unit, frame: FrameSpec) -> Self {
        Self { x, y, z, unit, frame, _tag: PhantomData }
    }

    #[must_use]
    pub const fn planar(x: Ratio, y: Ratio, unit: Unit, frame: FrameSpec) -> Self {
        Self::new(x, y, Ratio::zero(), unit, frame)
    }

    #[must_use]
    pub const fn x(&self) -> Ratio {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> Ratio {
        self.y
    }

    #[must_use]
    pub const fn z(&self) -> Ratio {
        self.z
    }

    #[must_use]
    pub const fn unit(&self) -> &Unit {
        &self.unit
    }

    #[must_use]
    pub const fn frame(&self) -> &FrameSpec {
        &self.frame
    }

    pub fn encode_canonical(&self, tag: &[u8], enc: &mut CanonicalEncoder) {
        enc.tag(tag);
        self.x.encode_canonical(enc);
        self.y.encode_canonical(enc);
        self.z.encode_canonical(enc);
        self.unit.encode_canonical(enc);
        self.frame.encode_canonical(enc);
    }
}

/// First moment of vertical support forces `(P_x, P_y, 0) = (sum x_i * f_i, sum y_i * f_i, 0)`.
pub type FirstMoment = TypedMoment3<FirstMomentTag>;

/// Three-axis torque `(tau_x, tau_y, tau_z)` — distinct from [`FirstMoment`] unless the
/// test-only negative-control feature `nc-torque-alias-first-moment` is enabled.
#[cfg(not(feature = "nc-torque-alias-first-moment"))]
pub type TorqueVec = TypedMoment3<TorqueTag>;

/// Negative-control mutant (`N25b-01`): aliases `TorqueVec` to `FirstMoment`.
#[cfg(feature = "nc-torque-alias-first-moment")]
pub type TorqueVec = FirstMoment;

impl FirstMoment {
    /// Convert a support first moment `P = (P_x, P_y, 0)` in right-handed `+z` up coordinates
    /// into its physical support torque `(P_y, -P_x, 0)` (Rule 2).
    ///
    /// Example: `P = (60, 30) N·m` maps to torque `(30, -60, 0) N·m`.
    pub fn to_support_torque_z_up(&self) -> Result<TorqueVec, ProofError> {
        if self.frame.axes != AxisConvention::ZUp
            || self.frame.handedness != Handedness::RightHanded
            || self.frame.sign_convention != SignConvention::RightHandScrew
            || self.frame.origin.trim().is_empty()
            || !self.unit.is_newton_meters()
            || !self.z.is_zero()
        {
            return Err(ProofError::InvalidPayload {
                kind: "lp-primal",
                detail: "support torque requires a planar N·m first moment in a right-handed +z-up frame with the right-hand screw convention".to_owned(),
            });
        }
        Ok(TorqueVec::new(
            self.y,
            self.x.neg_exact()?,
            Ratio::zero(),
            self.unit.clone(),
            self.frame.clone(),
        ))
    }
}

/// Validated port identifier (e.g. `"foot.pp"`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortId(String);

impl PortId {
    pub fn new(id: impl Into<String>) -> Result<Self, ProofError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "port_id" });
        }
        Ok(Self(id))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Ordered, non-empty, duplicate-free port sequence. Reordering yields a distinct value (Rule 3, CX-108).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderedPorts {
    ports: Vec<PortId>,
}

impl OrderedPorts {
    pub fn new(ports: Vec<PortId>) -> Result<Self, ProofError> {
        if ports.is_empty() {
            return Err(ProofError::EmptyIdentifier { field: "ordered_ports" });
        }
        let mut seen = BTreeSet::new();
        for p in &ports {
            if !seen.insert(p.as_str()) {
                return Err(ProofError::DuplicatePort { port: p.as_str().to_owned() });
            }
        }
        Ok(Self { ports })
    }

    #[must_use]
    pub fn as_slice(&self) -> &[PortId] {
        &self.ports
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.ports.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ports.is_empty()
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"ports");
        enc.usize(self.ports.len());
        for p in &self.ports {
            enc.str(p.as_str());
        }
    }
}

/// Force vector bound to an [`OrderedPorts`] sequence, unit, and frame (Rule 3, CX-108).
///
/// Cannot be constructed without supplying its ordered port list:
///
/// ```compile_fail,E0451
/// use frob_core::proof::{FrameSpec, OrderedPorts, PortForceVector, PortId, Ratio, Unit};
/// let ports = OrderedPorts::new(vec![PortId::new("foot.pp").unwrap()]).unwrap();
/// let _ = PortForceVector {
///     ports,
///     values: vec![Ratio::from_int(160)],
///     unit: Unit::newtons(),
///     frame: FrameSpec::right_handed_z_up("base"),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortForceVector {
    ports: OrderedPorts,
    values: Vec<Ratio>,
    unit: Unit,
    frame: FrameSpec,
}

impl PortForceVector {
    pub fn new(
        ports: OrderedPorts,
        values: Vec<Ratio>,
        unit: Unit,
        frame: FrameSpec,
    ) -> Result<Self, ProofError> {
        if ports.len() != values.len() {
            return Err(ProofError::PortLengthMismatch {
                expected: ports.len(),
                observed: values.len(),
            });
        }
        Ok(Self { ports, values, unit, frame })
    }

    #[must_use]
    pub const fn ports(&self) -> &OrderedPorts {
        &self.ports
    }

    #[must_use]
    pub fn values(&self) -> &[Ratio] {
        &self.values
    }

    #[must_use]
    pub const fn unit(&self) -> &Unit {
        &self.unit
    }

    #[must_use]
    pub const fn frame(&self) -> &FrameSpec {
        &self.frame
    }

    pub fn encode_canonical(&self, enc: &mut CanonicalEncoder) {
        enc.tag(b"port_forces");
        self.ports.encode_canonical(enc);
        enc.usize(self.values.len());
        for &v in &self.values {
            v.encode_canonical(enc);
        }
        self.unit.encode_canonical(enc);
        self.frame.encode_canonical(enc);
    }
}
