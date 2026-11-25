//! Kernel manifest `frob.kernel/1`, stage signature, static/binding validation,
//! and `registries/kernels.toml` provenance loader (`N21b-01`).
//!
//! Enforces INV-SPARK-04 (per-world footprints / Stat isolation), PO-SPARK-03
//! (resource bounds), PO-SPARK-04 (IR load/store containment), and explicit
//! stride unit/signedness checks (KA-532, CX-218). Determinism D0; zero
//! `unsafe`; prints nothing (L-7).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use frob_core::canon::{CanonError, CanonWriter, Canonical, CanonicalBytes, Value as CVal};
use frob_core::evidence::DetClass;
use frob_core::id::{Blake3, SchemaName, hash_blob, identity};
use frob_core::json::{Json, Limits, Spanned, admit};
use frob_core::registry::toml::{self, Fields, Value as TVal};

use crate::dialect::METAL_MAX_ARGUMENTS;

/// Schema identifier for kernel manifests.
pub const KERNEL_MANIFEST_SCHEMA: &str = "frob.kernel/1";
/// Schema identifier for `registries/kernels.toml`.
pub const KERNEL_REGISTRY_SCHEMA: &str = "frob.kernels/1";

/// Content-addressed identity of a [`KernelManifest`] (`BLAKE3` over canonical JSON without `id`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KernelId(pub Blake3);

impl KernelId {
    #[must_use]
    pub fn to_hex(self) -> String {
        self.0.to_hex()
    }

    pub fn from_hex(s: &str) -> Result<Self, ManifestError> {
        Blake3::from_hex(s)
            .map(Self)
            .map_err(|_| ManifestError::Parse(format!("invalid 64-hex KernelId `{s}`")))
    }
}

impl fmt::Display for KernelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KernelClass {
    Float,
    Exact,
    Guarded,
    Stat,
}

impl KernelClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Float => "Float",
            Self::Exact => "Exact",
            Self::Guarded => "Guarded",
            Self::Stat => "Stat",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "Float" => Ok(Self::Float),
            "Exact" => Ok(Self::Exact),
            "Guarded" => Ok(Self::Guarded),
            "Stat" => Ok(Self::Stat),
            _ => Err(ManifestError::Parse(format!("unknown KernelClass `{s}`"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KernelRole {
    Ordinary,
    StrictPredicate,
    Digest,
    NegativeControl,
}

impl KernelRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ordinary => "Ordinary",
            Self::StrictPredicate => "StrictPredicate",
            Self::Digest => "Digest",
            Self::NegativeControl => "NegativeControl",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "Ordinary" => Ok(Self::Ordinary),
            "StrictPredicate" => Ok(Self::StrictPredicate),
            "Digest" => Ok(Self::Digest),
            "NegativeControl" => Ok(Self::NegativeControl),
            _ => Err(ManifestError::Parse(format!("unknown KernelRole `{s}`"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElemType {
    F32,
    F64,
    U32,
    I32,
    Bool,
}

impl ElemType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::U32 => "u32",
            Self::I32 => "i32",
            Self::Bool => "bool",
        }
    }

    #[must_use]
    pub const fn byte_size(self) -> usize {
        match self {
            Self::F32 | Self::U32 | Self::I32 => 4,
            Self::F64 => 8,
            Self::Bool => 1,
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "u32" => Ok(Self::U32),
            "i32" => Ok(Self::I32),
            "bool" => Ok(Self::Bool),
            _ => Err(ManifestError::Parse(format!("unknown ElemType `{s}`"))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArgView {
    Value,
    Bits,
}

impl ArgView {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Value => "Value",
            Self::Bits => "Bits",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "Value" => Ok(Self::Value),
            "Bits" => Ok(Self::Bits),
            _ => Err(ManifestError::Parse(format!("unknown ArgView `{s}`"))),
        }
    }
}

/// Closed semantic kind set for buffer arguments (read by N10a-03 D-3 doctrine lint).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElemKind {
    Quaternion,
    UnitComplex,
    TurnCount,
    Vector,
    Twist,
    Wrench,
    Scalar,
    Integer,
    Angle,
    EulerTriple,
    RollPitchYaw,
    AxisAngle,
}

impl ElemKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Quaternion => "quaternion",
            Self::UnitComplex => "unit-complex",
            Self::TurnCount => "turn-count",
            Self::Vector => "vector",
            Self::Twist => "twist",
            Self::Wrench => "wrench",
            Self::Scalar => "scalar",
            Self::Integer => "integer",
            Self::Angle => "angle",
            Self::EulerTriple => "euler-triple",
            Self::RollPitchYaw => "roll-pitch-yaw",
            Self::AxisAngle => "axis-angle",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "quaternion" => Ok(Self::Quaternion),
            "unit-complex" => Ok(Self::UnitComplex),
            "turn-count" => Ok(Self::TurnCount),
            "vector" => Ok(Self::Vector),
            "twist" => Ok(Self::Twist),
            "wrench" => Ok(Self::Wrench),
            "scalar" => Ok(Self::Scalar),
            "integer" => Ok(Self::Integer),
            "angle" => Ok(Self::Angle),
            "euler-triple" => Ok(Self::EulerTriple),
            "roll-pitch-yaw" => Ok(Self::RollPitchYaw),
            "axis-angle" => Ok(Self::AxisAngle),
            _ => Err(ManifestError::Parse(format!("unknown ElemKind `{s}`"))),
        }
    }
}

/// Non-negative affine expression over shape-class capacities.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapExpr {
    pub constant: u64,
    pub nv_cap: u64,
    pub link_cap: u64,
    pub contact_cap: u64,
    pub row_cap: u64,
    pub sample_cap: u64,
    pub actuator_cap: u64,
    pub worlds: u64,
}

impl CapExpr {
    #[must_use]
    pub const fn constant(c: u64) -> Self {
        Self {
            constant: c,
            nv_cap: 0,
            link_cap: 0,
            contact_cap: 0,
            row_cap: 0,
            sample_cap: 0,
            actuator_cap: 0,
            worlds: 0,
        }
    }

    #[must_use]
    pub const fn worlds(coeff: u64) -> Self {
        Self {
            worlds: coeff,
            ..Self::constant(0)
        }
    }

    #[must_use]
    pub const fn row_cap(coeff: u64) -> Self {
        Self {
            row_cap: coeff,
            ..Self::constant(0)
        }
    }

    #[must_use]
    pub const fn nv_cap(coeff: u64) -> Self {
        Self {
            nv_cap: coeff,
            ..Self::constant(0)
        }
    }

    #[must_use]
    pub fn eval(self, class: &ShapeClassCaps) -> Option<u64> {
        let mut acc = self.constant;
        for (coeff, cap) in [
            (self.nv_cap, class.nv_cap),
            (self.link_cap, class.link_cap),
            (self.contact_cap, class.contact_cap),
            (self.row_cap, class.row_cap),
            (self.sample_cap, class.sample_cap),
            (self.actuator_cap, class.actuator_cap),
            (self.worlds, class.worlds),
        ] {
            acc = acc.checked_add(coeff.checked_mul(cap)?)?;
        }
        Some(acc)
    }
}

/// Concrete capacity tuple for one shape class; extra unused capacities are ignored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeClassCaps {
    pub name: String,
    pub nv_cap: u64,
    pub link_cap: u64,
    pub contact_cap: u64,
    pub row_cap: u64,
    pub sample_cap: u64,
    pub actuator_cap: u64,
    pub worlds: u64,
    pub extra: BTreeMap<String, u64>,
}

impl ShapeClassCaps {
    /// Default admitted M0 shape classes (DoF buckets 12, 18, 24, 36, 48, 64).
    #[must_use]
    pub fn default_classes() -> Vec<Self> {
        [12u64, 18, 24, 36, 48, 64]
            .into_iter()
            .map(|nv| Self {
                name: format!("dof-{nv}"),
                nv_cap: nv,
                link_cap: nv + 1,
                contact_cap: nv * 2,
                row_cap: ((nv * 6 + 7) / 8) * 8,
                sample_cap: 16,
                actuator_cap: nv,
                worlds: 64,
                extra: BTreeMap::new(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgDecl {
    pub id: String,
    pub elem: ElemType,
    pub view: ArgView,
    pub shape: Vec<CapExpr>,
    /// Exponent vector over `frob_core::qty` base dimensions `[m, kg, s, K, A, mol]`.
    pub dim: [i8; 6],
    pub kind: ElemKind,
    pub components: Vec<String>,
    pub state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdentityDecl {
    pub name: String,
    pub statement: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LaneScope {
    OwnWorld,
    AllWorlds,
}

impl LaneScope {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnWorld => "OwnWorld",
            Self::AllWorlds => "AllWorlds",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "OwnWorld" => Ok(Self::OwnWorld),
            "AllWorlds" => Ok(Self::AllWorlds),
            _ => Err(ManifestError::Parse(format!("unknown LaneScope `{s}`"))),
        }
    }
}

/// Half-open element range `[lo, hi)` accessed per lane on argument `arg`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Footprint {
    pub arg: String,
    pub lanes: LaneScope,
    pub lo: CapExpr,
    pub hi: CapExpr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StrideUnit {
    Bytes,
    Elements,
}

impl StrideUnit {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bytes => "Bytes",
            Self::Elements => "Elements",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "Bytes" => Ok(Self::Bytes),
            "Elements" => Ok(Self::Elements),
            _ => Err(ManifestError::Parse(format!("unknown StrideUnit `{s}`"))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgLayout {
    pub arg: String,
    pub elem: ElemType,
    pub extents: Vec<CapExpr>,
    pub strides: Vec<i64>,
    pub stride_unit: StrideUnit,
    pub stride_signed: bool,
    pub origin: CapExpr,
    pub alloc_extent: CapExpr,
}

impl ArgLayout {
    /// Compute the element index at multi-index `coords` using this layout's own unit.
    #[must_use]
    pub fn element_offset(&self, coords: &[usize], class: &ShapeClassCaps) -> Option<usize> {
        if coords.len() != self.strides.len() {
            return None;
        }
        let mut acc = i128::from(self.origin.eval(class)?);
        let elem_bytes = i128::from(self.elem.byte_size() as u64);
        for (&idx, &stride) in coords.iter().zip(&self.strides) {
            let delta = i128::from(idx as u64) * i128::from(stride);
            let elem_delta = match self.stride_unit {
                StrideUnit::Elements => delta,
                StrideUnit::Bytes => {
                    if delta % elem_bytes != 0 {
                        return None;
                    }
                    delta / elem_bytes
                }
            };
            acc += elem_delta;
        }
        usize::try_from(acc).ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceBounds {
    pub live_values: u32,
    pub stack_bytes: u32,
    pub max_loop_bound: u32,
    pub buffer_args: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReductionConsumer {
    Sum,
    Product,
    Min,
    Max,
    Factorization,
}

impl ReductionConsumer {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sum => "sum",
            Self::Product => "product",
            Self::Min => "min",
            Self::Max => "max",
            Self::Factorization => "factorization",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "sum" => Ok(Self::Sum),
            "product" => Ok(Self::Product),
            "min" => Ok(Self::Min),
            "max" => Ok(Self::Max),
            "factorization" => Ok(Self::Factorization),
            _ => Err(ManifestError::Parse(format!(
                "unknown ReductionConsumer `{s}`"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NeutralSpec {
    pub arg: String,
    pub consumer: ReductionConsumer,
    /// IEEE-754 bit pattern of the neutral element (`f64::to_bits` or zero-extended `f32::to_bits`).
    pub value_bits: Option<u64>,
    pub identity_rows: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrecisionProfile {
    F32,
    F64,
    ExactU32,
}

impl PrecisionProfile {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::ExactU32 => "exact-u32",
        }
    }

    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        match s {
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "exact-u32" => Ok(Self::ExactU32),
            _ => Err(ManifestError::Parse(format!(
                "unknown PrecisionProfile `{s}`"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumericalContract {
    pub precision: PrecisionProfile,
    pub error_bound: Option<f64>,
    pub deferral_threshold: Option<f64>,
    pub exact_fallback: Option<String>,
    pub neutrals: Vec<NeutralSpec>,
    pub padded_reduction_outputs: Vec<(String, ReductionConsumer)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FootprintEvidence {
    Lint,
    KirProgram(Blake3),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StageSignature {
    pub reads: Vec<Footprint>,
    pub writes: Vec<Footprint>,
    pub may_alias: Vec<(String, String)>,
    pub layout: Vec<ArgLayout>,
    /// Ownership epoch offset read (`-1` for committed `e - 1`, `0` for `e`).
    pub epoch_read: i64,
    /// Ownership epoch offset written (`0` for `e`).
    pub epoch_write: i64,
    pub resources: ResourceBounds,
    pub numerics: NumericalContract,
    pub feeds_state: bool,
    pub footprint_evidence: FootprintEvidence,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KernelManifest {
    pub id: KernelId,
    pub name: String,
    pub class: KernelClass,
    pub role: KernelRole,
    pub args: Vec<ArgDecl>,
    pub determinism: DetClass,
    pub identities: Vec<IdentityDecl>,
    pub derivative: Option<KernelId>,
    pub cpu_reference: String,
    pub signature: StageSignature,
}

/// Program-derived load/store sets and resource counts checked by static rule 7 (PO-SPARK-03/04).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramFootprint {
    pub reads: Vec<Footprint>,
    pub writes: Vec<Footprint>,
    pub resources: ResourceBounds,
}

/// Backend resource limits checked by binding rule 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendLimits {
    pub name: &'static str,
    pub max_buffer_args: u32,
    pub max_stack_bytes: u32,
}

impl BackendLimits {
    #[must_use]
    pub const fn metal() -> Self {
        Self {
            name: "gpu-metal",
            max_buffer_args: METAL_MAX_ARGUMENTS as u32,
            max_stack_bytes: 65_536,
        }
    }

    #[must_use]
    pub const fn cuda() -> Self {
        Self {
            name: "gpu-cuda",
            max_buffer_args: 128,
            max_stack_bytes: 65_536,
        }
    }

    #[must_use]
    pub const fn batch_cpu() -> Self {
        Self {
            name: "batch-cpu",
            max_buffer_args: 256,
            max_stack_bytes: 1_048_576,
        }
    }
}

/// Runtime slab descriptor bound to one kernel argument in a stage plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlabBinding {
    pub arg: String,
    pub slab_id: String,
    pub elem: ElemType,
    pub extents: Vec<CapExpr>,
    pub strides: Vec<i64>,
    pub stride_unit: StrideUnit,
    pub stride_signed: bool,
    pub origin: CapExpr,
    pub alloc_extent: CapExpr,
    /// Offset in elements within the backing slab where this binding starts.
    pub slab_offset: u64,
    /// True if this slab is written exclusively by `Stat` kernels.
    pub from_stat_kernel: bool,
}

/// Declared output range on a slab for binding rule 3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredOutput {
    pub arg: String,
    pub slab_id: String,
    pub lo: u64,
    pub hi: u64,
}

/// Epoch write record of another kernel in the same launch (binding rule 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageWriteEpoch {
    pub slab_id: String,
    pub epoch_write: i64,
    pub ordered_before: bool,
}

/// Typed registration, static, and binding validation errors (build/load time, not `FROB-REF`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    Parse(String),
    UndeclaredAlias {
        kernel: String,
        arg_a: String,
        arg_b: String,
        slab: String,
    },
    FootprintOutOfRange {
        kernel: String,
        arg: String,
        detail: String,
    },
    CrossWorldFootprint {
        kernel: String,
        detail: String,
    },
    ClassMismatch {
        kernel: String,
        detail: String,
    },
    ResourceMismatch {
        kernel: String,
        detail: String,
    },
    NeutralMissing {
        kernel: String,
        arg: String,
        detail: String,
    },
    LoadStoreNotContained {
        kernel: String,
        arg: String,
        detail: String,
    },
    LayoutMismatch {
        kernel: String,
        arg: String,
        expected: String,
        observed: String,
    },
    Registry(String),
}

impl ManifestError {
    #[must_use]
    pub const fn variant_name(&self) -> &'static str {
        match self {
            Self::Parse(_) => "Parse",
            Self::UndeclaredAlias { .. } => "UndeclaredAlias",
            Self::FootprintOutOfRange { .. } => "FootprintOutOfRange",
            Self::CrossWorldFootprint { .. } => "CrossWorldFootprint",
            Self::ClassMismatch { .. } => "ClassMismatch",
            Self::ResourceMismatch { .. } => "ResourceMismatch",
            Self::NeutralMissing { .. } => "NeutralMissing",
            Self::LoadStoreNotContained { .. } => "LoadStoreNotContained",
            Self::LayoutMismatch { .. } => "LayoutMismatch",
            Self::Registry(_) => "Registry",
        }
    }
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(m) => write!(f, "manifest parse error: {m}"),
            Self::UndeclaredAlias {
                kernel,
                arg_a,
                arg_b,
                slab,
            } => {
                write!(
                    f,
                    "undeclared alias in `{kernel}`: args `{arg_a}` and `{arg_b}` overlap on slab `{slab}`"
                )
            }
            Self::FootprintOutOfRange {
                kernel,
                arg,
                detail,
            } => {
                write!(
                    f,
                    "footprint out of range in `{kernel}` arg `{arg}`: {detail}"
                )
            }
            Self::CrossWorldFootprint { kernel, detail } => {
                write!(
                    f,
                    "cross-world / stat isolation violation in `{kernel}`: {detail}"
                )
            }
            Self::ClassMismatch { kernel, detail } => {
                write!(f, "class/role mismatch in `{kernel}`: {detail}")
            }
            Self::ResourceMismatch { kernel, detail } => {
                write!(f, "resource mismatch in `{kernel}`: {detail}")
            }
            Self::NeutralMissing {
                kernel,
                arg,
                detail,
            } => {
                write!(
                    f,
                    "neutral element invalid/missing in `{kernel}` arg `{arg}`: {detail}"
                )
            }
            Self::LoadStoreNotContained {
                kernel,
                arg,
                detail,
            } => {
                write!(
                    f,
                    "load/store not contained in `{kernel}` arg `{arg}`: {detail}"
                )
            }
            Self::LayoutMismatch {
                kernel,
                arg,
                expected,
                observed,
            } => {
                write!(
                    f,
                    "layout mismatch in `{kernel}` arg `{arg}`: expected {expected}, observed {observed}"
                )
            }
            Self::Registry(m) => write!(f, "kernel registry error: {m}"),
        }
    }
}

impl std::error::Error for ManifestError {}

impl From<CanonError> for ManifestError {
    fn from(e: CanonError) -> Self {
        Self::Parse(format!("canonical JSON error: {e}"))
    }
}

// --- Canonical JSON Serialization & KernelId ---

impl Canonical for CapExpr {
    fn write(&self, w: CVal<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.int53(
                "actuator_cap",
                i64::try_from(self.actuator_cap)
                    .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
            )?;
            o.int53(
                "constant",
                i64::try_from(self.constant)
                    .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
            )?;
            o.int53(
                "contact_cap",
                i64::try_from(self.contact_cap)
                    .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
            )?;
            o.int53(
                "link_cap",
                i64::try_from(self.link_cap)
                    .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
            )?;
            o.int53(
                "nv_cap",
                i64::try_from(self.nv_cap)
                    .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
            )?;
            o.int53(
                "row_cap",
                i64::try_from(self.row_cap)
                    .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
            )?;
            o.int53(
                "sample_cap",
                i64::try_from(self.sample_cap)
                    .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
            )?;
            o.int53(
                "worlds",
                i64::try_from(self.worlds)
                    .map_err(|_| CanonError::Int53OutOfRange { value: i64::MAX })?,
            )?;
            Ok(())
        })
    }
}

impl Canonical for Footprint {
    fn write(&self, w: CVal<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("arg", &self.arg)?;
            o.member_term("hi", &self.hi)?;
            o.str("lanes", self.lanes.as_str())?;
            o.member_term("lo", &self.lo)?;
            Ok(())
        })
    }
}

fn write_manifest_body(
    m: &KernelManifest,
    include_id: Option<KernelId>,
    w: CVal<'_>,
) -> Result<(), CanonError> {
    let mut args = m.args.clone();
    args.sort_by(|a, b| a.id.cmp(&b.id));
    let mut identities = m.identities.clone();
    identities.sort();
    let mut reads = m.signature.reads.clone();
    reads.sort();
    let mut writes = m.signature.writes.clone();
    writes.sort();
    let mut may_alias: Vec<(String, String)> = m
        .signature
        .may_alias
        .iter()
        .map(|(a, b)| {
            if a <= b {
                (a.clone(), b.clone())
            } else {
                (b.clone(), a.clone())
            }
        })
        .collect();
    may_alias.sort();
    may_alias.dedup();
    let mut layout = m.signature.layout.clone();
    layout.sort_by(|a, b| a.arg.cmp(&b.arg));
    let mut neutrals = m.signature.numerics.neutrals.clone();
    neutrals.sort();
    let mut padded = m.signature.numerics.padded_reduction_outputs.clone();
    padded.sort();

    w.object(|o| {
        o.member("args", |v| {
            v.array(|arr| {
                for a in &args {
                    arr.push(|ev| {
                        ev.object(|ao| {
                            ao.member("components", |cv| {
                                cv.array(|ca| {
                                    for c in &a.components {
                                        ca.push_str(c);
                                    }
                                    Ok(())
                                })
                            })?;
                            ao.member("dim", |dv| {
                                dv.array(|da| {
                                    for d in a.dim {
                                        da.push_int53(i64::from(d))?;
                                    }
                                    Ok(())
                                })
                            })?;
                            ao.str("elem", a.elem.as_str())?;
                            ao.str("id", &a.id)?;
                            ao.str("kind", a.kind.as_str())?;
                            ao.member("shape", |sv| {
                                sv.array(|sa| {
                                    for expr in &a.shape {
                                        sa.push_term(expr)?;
                                    }
                                    Ok(())
                                })
                            })?;
                            ao.bool("state", a.state)?;
                            ao.str("view", a.view.as_str())?;
                            Ok(())
                        })
                    })?;
                }
                Ok(())
            })
        })?;
        o.str("class", m.class.as_str())?;
        o.str("cpu_reference", &m.cpu_reference)?;
        match m.derivative {
            Some(d) => o.str("derivative", &d.to_hex())?,
            None => o.null("derivative")?,
        }
        o.str("determinism", m.determinism.as_str())?;
        if let Some(kid) = include_id {
            o.str("id", &kid.to_hex())?;
        }
        o.member("identities", |iv| {
            iv.array(|ia| {
                for id_decl in &identities {
                    ia.push(|ev| {
                        ev.object(|io| {
                            io.str("name", &id_decl.name)?;
                            io.str("statement", &id_decl.statement)?;
                            Ok(())
                        })
                    })?;
                }
                Ok(())
            })
        })?;
        o.str("name", &m.name)?;
        o.str("role", m.role.as_str())?;
        o.str("schema", KERNEL_MANIFEST_SCHEMA)?;
        o.member("signature", |sv| {
            sv.object(|so| {
                so.int53("epoch_read", m.signature.epoch_read)?;
                so.int53("epoch_write", m.signature.epoch_write)?;
                so.bool("feeds_state", m.signature.feeds_state)?;
                match m.signature.footprint_evidence {
                    FootprintEvidence::Lint => so.str("footprint_evidence", "lint")?,
                    FootprintEvidence::KirProgram(h) => so.member("footprint_evidence", |fv| {
                        fv.object(|fo| {
                            fo.str("kir_program", &h.to_hex())?;
                            Ok(())
                        })
                    })?,
                }
                so.member("layout", |lv| {
                    lv.array(|la| {
                        for l in &layout {
                            la.push(|ev| {
                                ev.object(|lo| {
                                    lo.member_term("alloc_extent", &l.alloc_extent)?;
                                    lo.str("arg", &l.arg)?;
                                    lo.str("elem", l.elem.as_str())?;
                                    lo.member("extents", |xv| {
                                        xv.array(|xa| {
                                            for e in &l.extents {
                                                xa.push_term(e)?;
                                            }
                                            Ok(())
                                        })
                                    })?;
                                    lo.member_term("origin", &l.origin)?;
                                    lo.bool("stride_signed", l.stride_signed)?;
                                    lo.str("stride_unit", l.stride_unit.as_str())?;
                                    lo.member("strides", |stv| {
                                        stv.array(|sta| {
                                            for &s in &l.strides {
                                                sta.push_int53(s)?;
                                            }
                                            Ok(())
                                        })
                                    })?;
                                    Ok(())
                                })
                            })?;
                        }
                        Ok(())
                    })
                })?;
                so.member("may_alias", |mv| {
                    mv.array(|ma| {
                        for (a, b) in &may_alias {
                            ma.push(|ev| {
                                ev.array(|pa| {
                                    pa.push_str(a);
                                    pa.push_str(b);
                                    Ok(())
                                })
                            })?;
                        }
                        Ok(())
                    })
                })?;
                so.member("numerics", |nv| {
                    nv.object(|no| {
                        match m.signature.numerics.deferral_threshold {
                            Some(x) => no.f64_shortest("deferral_threshold", x)?,
                            None => no.null("deferral_threshold")?,
                        }
                        match m.signature.numerics.error_bound {
                            Some(x) => no.f64_shortest("error_bound", x)?,
                            None => no.null("error_bound")?,
                        }
                        match &m.signature.numerics.exact_fallback {
                            Some(s) => no.str("exact_fallback", s)?,
                            None => no.null("exact_fallback")?,
                        }
                        no.member("neutrals", |nev| {
                            nev.array(|nea| {
                                for n in &neutrals {
                                    nea.push(|ev| {
                                        ev.object(|neo| {
                                            neo.str("arg", &n.arg)?;
                                            neo.str("consumer", n.consumer.as_str())?;
                                            neo.bool("identity_rows", n.identity_rows)?;
                                            match n.value_bits {
                                                Some(b) => {
                                                    neo.str("value_bits_hex", &format!("{b:016x}"))?
                                                }
                                                None => neo.null("value_bits_hex")?,
                                            }
                                            Ok(())
                                        })
                                    })?;
                                }
                                Ok(())
                            })
                        })?;
                        no.member("padded_reduction_outputs", |pv| {
                            pv.array(|pa| {
                                for (arg, cons) in &padded {
                                    pa.push(|ev| {
                                        ev.object(|po| {
                                            po.str("arg", arg)?;
                                            po.str("consumer", cons.as_str())?;
                                            Ok(())
                                        })
                                    })?;
                                }
                                Ok(())
                            })
                        })?;
                        no.str("precision", m.signature.numerics.precision.as_str())?;
                        Ok(())
                    })
                })?;
                so.member("reads", |rv| {
                    rv.array(|ra| {
                        for fp in &reads {
                            ra.push_term(fp)?;
                        }
                        Ok(())
                    })
                })?;
                so.member("resources", |rv| {
                    rv.object(|ro| {
                        ro.int53("buffer_args", i64::from(m.signature.resources.buffer_args))?;
                        ro.int53("live_values", i64::from(m.signature.resources.live_values))?;
                        ro.int53(
                            "max_loop_bound",
                            i64::from(m.signature.resources.max_loop_bound),
                        )?;
                        ro.int53("stack_bytes", i64::from(m.signature.resources.stack_bytes))?;
                        Ok(())
                    })
                })?;
                so.member("writes", |wv| {
                    wv.array(|wa| {
                        for fp in &writes {
                            wa.push_term(fp)?;
                        }
                        Ok(())
                    })
                })?;
                Ok(())
            })
        })?;
        Ok(())
    })
}

impl KernelManifest {
    /// Compute the deterministic [`KernelId`] (BLAKE3 over canonical JSON without `id`).
    pub fn compute_id(&self) -> Result<KernelId, ManifestError> {
        let mut w = CanonWriter::new();
        write_manifest_body(self, None, w.value()?)?;
        let bytes = w.finish()?;
        let schema = SchemaName::parse(KERNEL_MANIFEST_SCHEMA)
            .map_err(|e| ManifestError::Parse(format!("schema name error: {e}")))?;
        Ok(KernelId(identity(&schema, &bytes)))
    }

    /// Recompute and seal `self.id`.
    pub fn seal(mut self) -> Result<Self, ManifestError> {
        self.id = self.compute_id()?;
        Ok(self)
    }

    /// Serialize to canonical JSON (`frob.kernel/1`, including computed `id`).
    pub fn to_canonical_json(&self) -> Result<CanonicalBytes, ManifestError> {
        let kid = self.compute_id()?;
        let mut w = CanonWriter::new();
        write_manifest_body(self, Some(kid), w.value()?)?;
        Ok(w.finish()?)
    }

    /// Parse a `frob.kernel/1` manifest from JSON bytes and verify `id` if present.
    pub fn parse_json(bytes: &[u8]) -> Result<Self, ManifestError> {
        let adm = admit(bytes, &Limits::default())
            .map_err(|e| ManifestError::Parse(format!("JSON admission failed: {e}")))?;
        let root = obj_map(adm.root(), "manifest")?;
        let schema = req_str(&root, "schema", "manifest")?;
        if schema != KERNEL_MANIFEST_SCHEMA {
            return Err(ManifestError::Parse(format!(
                "expected schema `{KERNEL_MANIFEST_SCHEMA}`, got `{schema}`"
            )));
        }
        let name = req_str(&root, "name", "manifest")?.to_owned();
        let class = KernelClass::parse(req_str(&root, "class", "manifest")?)?;
        let role = KernelRole::parse(req_str(&root, "role", "manifest")?)?;
        let det_s = req_str(&root, "determinism", "manifest")?;
        let determinism = DetClass::parse(det_s)
            .ok_or_else(|| ManifestError::Parse(format!("unknown determinism `{det_s}`")))?;
        let cpu_reference = req_str(&root, "cpu_reference", "manifest")?.to_owned();
        let derivative = match &req_field(&root, "derivative", "manifest")?.value {
            Json::Null => None,
            Json::Str(s) => Some(KernelId::from_hex(s)?),
            _ => {
                return Err(ManifestError::Parse(
                    "`derivative` must be null or 64-hex string".into(),
                ));
            }
        };
        let mut identities = Vec::new();
        for item in req_arr(&root, "identities", "manifest")? {
            let io = obj_map(item, "identity")?;
            identities.push(IdentityDecl {
                name: req_str(&io, "name", "identity")?.to_owned(),
                statement: req_str(&io, "statement", "identity")?.to_owned(),
            });
        }
        let mut args = Vec::new();
        for item in req_arr(&root, "args", "manifest")? {
            let ao = obj_map(item, "arg")?;
            let id = req_str(&ao, "id", "arg")?.to_owned();
            let elem = ElemType::parse(req_str(&ao, "elem", "arg")?)?;
            let view = ArgView::parse(req_str(&ao, "view", "arg")?)?;
            let kind = ElemKind::parse(req_str(&ao, "kind", "arg")?)?;
            let components = req_arr(&ao, "components", "arg")?
                .iter()
                .map(|c| {
                    c.value
                        .as_str()
                        .map(str::to_owned)
                        .ok_or_else(|| ManifestError::Parse("component must be string".into()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let state = req_bool(&ao, "state", "arg")?;
            let shape = req_arr(&ao, "shape", "arg")?
                .iter()
                .map(parse_cap_expr)
                .collect::<Result<Vec<_>, _>>()?;
            let dim_arr = req_arr(&ao, "dim", "arg")?;
            if dim_arr.len() != 6 {
                return Err(ManifestError::Parse(format!(
                    "arg `{id}` dim must have 6 exponents"
                )));
            }
            let mut dim = [0i8; 6];
            for (slot, sp) in dim.iter_mut().zip(dim_arr) {
                let n = sp
                    .as_int53()
                    .map_err(|e| ManifestError::Parse(format!("dim int: {e}")))?;
                *slot = i8::try_from(n)
                    .map_err(|_| ManifestError::Parse(format!("dim exponent out of i8: {n}")))?;
            }
            args.push(ArgDecl {
                id,
                elem,
                view,
                shape,
                dim,
                kind,
                components,
                state,
            });
        }

        let so = obj_map(req_field(&root, "signature", "manifest")?, "signature")?;
        let reads = req_arr(&so, "reads", "signature")?
            .iter()
            .map(parse_footprint)
            .collect::<Result<Vec<_>, _>>()?;
        let writes = req_arr(&so, "writes", "signature")?
            .iter()
            .map(parse_footprint)
            .collect::<Result<Vec<_>, _>>()?;
        let mut may_alias = Vec::new();
        for pair in req_arr(&so, "may_alias", "signature")? {
            let items = pair
                .value
                .as_array()
                .ok_or_else(|| ManifestError::Parse("may_alias item must be array".into()))?;
            if items.len() != 2 {
                return Err(ManifestError::Parse(
                    "may_alias pair must have length 2".into(),
                ));
            }
            let a = items[0]
                .value
                .as_str()
                .ok_or_else(|| ManifestError::Parse("may_alias[0] string".into()))?;
            let b = items[1]
                .value
                .as_str()
                .ok_or_else(|| ManifestError::Parse("may_alias[1] string".into()))?;
            may_alias.push((a.to_owned(), b.to_owned()));
        }
        let mut layout = Vec::new();
        for item in req_arr(&so, "layout", "signature")? {
            let lo = obj_map(item, "layout")?;
            layout.push(ArgLayout {
                arg: req_str(&lo, "arg", "layout")?.to_owned(),
                elem: ElemType::parse(req_str(&lo, "elem", "layout")?)?,
                extents: req_arr(&lo, "extents", "layout")?
                    .iter()
                    .map(parse_cap_expr)
                    .collect::<Result<Vec<_>, _>>()?,
                strides: req_arr(&lo, "strides", "layout")?
                    .iter()
                    .map(|s| {
                        s.as_int53()
                            .map_err(|e| ManifestError::Parse(format!("stride int: {e}")))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                stride_unit: StrideUnit::parse(req_str(&lo, "stride_unit", "layout")?)?,
                stride_signed: req_bool(&lo, "stride_signed", "layout")?,
                origin: parse_cap_expr(req_field(&lo, "origin", "layout")?)?,
                alloc_extent: parse_cap_expr(req_field(&lo, "alloc_extent", "layout")?)?,
            });
        }
        let epoch_read = req_int53(&so, "epoch_read", "signature")?;
        let epoch_write = req_int53(&so, "epoch_write", "signature")?;
        let feeds_state = req_bool(&so, "feeds_state", "signature")?;
        let footprint_evidence = match &req_field(&so, "footprint_evidence", "signature")?.value {
            Json::Str(s) if s == "lint" => FootprintEvidence::Lint,
            Json::Obj(_) => {
                let fo = obj_map(
                    req_field(&so, "footprint_evidence", "signature")?,
                    "footprint_evidence",
                )?;
                let hex = req_str(&fo, "kir_program", "footprint_evidence")?;
                let h = Blake3::from_hex(hex)
                    .map_err(|_| ManifestError::Parse(format!("bad kir_program hash `{hex}`")))?;
                FootprintEvidence::KirProgram(h)
            }
            _ => return Err(ManifestError::Parse("invalid footprint_evidence".into())),
        };
        let ro = obj_map(req_field(&so, "resources", "signature")?, "resources")?;
        let resources = ResourceBounds {
            live_values: req_u32(&ro, "live_values", "resources")?,
            stack_bytes: req_u32(&ro, "stack_bytes", "resources")?,
            max_loop_bound: req_u32(&ro, "max_loop_bound", "resources")?,
            buffer_args: req_u32(&ro, "buffer_args", "resources")?,
        };
        let no = obj_map(req_field(&so, "numerics", "signature")?, "numerics")?;
        let precision = PrecisionProfile::parse(req_str(&no, "precision", "numerics")?)?;
        let error_bound = opt_f64(&no, "error_bound", "numerics")?;
        let deferral_threshold = opt_f64(&no, "deferral_threshold", "numerics")?;
        let exact_fallback = match &req_field(&no, "exact_fallback", "numerics")?.value {
            Json::Null => None,
            Json::Str(s) => Some(s.clone()),
            _ => {
                return Err(ManifestError::Parse(
                    "exact_fallback must be null or string".into(),
                ));
            }
        };
        let mut neutrals = Vec::new();
        for item in req_arr(&no, "neutrals", "numerics")? {
            let neo = obj_map(item, "neutral")?;
            let value_bits = match &req_field(&neo, "value_bits_hex", "neutral")?.value {
                Json::Null => None,
                Json::Str(hex) => Some(
                    u64::from_str_radix(hex, 16)
                        .map_err(|_| ManifestError::Parse(format!("bad value_bits_hex `{hex}`")))?,
                ),
                _ => {
                    return Err(ManifestError::Parse(
                        "value_bits_hex must be null or hex string".into(),
                    ));
                }
            };
            neutrals.push(NeutralSpec {
                arg: req_str(&neo, "arg", "neutral")?.to_owned(),
                consumer: ReductionConsumer::parse(req_str(&neo, "consumer", "neutral")?)?,
                value_bits,
                identity_rows: req_bool(&neo, "identity_rows", "neutral")?,
            });
        }
        let mut padded_reduction_outputs = Vec::new();
        for item in req_arr(&no, "padded_reduction_outputs", "numerics")? {
            let po = obj_map(item, "padded_reduction_output")?;
            padded_reduction_outputs.push((
                req_str(&po, "arg", "padded_reduction_output")?.to_owned(),
                ReductionConsumer::parse(req_str(&po, "consumer", "padded_reduction_output")?)?,
            ));
        }

        let mut manifest = Self {
            id: KernelId(Blake3::from_bytes([0u8; 32])),
            name,
            class,
            role,
            args,
            determinism,
            identities,
            derivative,
            cpu_reference,
            signature: StageSignature {
                reads,
                writes,
                may_alias,
                layout,
                epoch_read,
                epoch_write,
                resources,
                numerics: NumericalContract {
                    precision,
                    error_bound,
                    deferral_threshold,
                    exact_fallback,
                    neutrals,
                    padded_reduction_outputs,
                },
                feeds_state,
                footprint_evidence,
            },
        };
        let computed = manifest.compute_id()?;
        if let Some(sp) = root.get("id") {
            let declared_hex = sp
                .value
                .as_str()
                .ok_or_else(|| ManifestError::Parse("`id` must be string".into()))?;
            let declared = KernelId::from_hex(declared_hex)?;
            if declared != computed {
                return Err(ManifestError::Parse(format!(
                    "manifest `{}` declared id {declared} != computed {computed}",
                    manifest.name
                )));
            }
        }
        manifest.id = computed;
        Ok(manifest)
    }

    // --- Static Validation (Rules 1..=7) ---

    /// Run all seven static validation rules across `classes`.
    pub fn validate_static(
        &self,
        classes: &[ShapeClassCaps],
        program: Option<&ProgramFootprint>,
        allow_negative_control: bool,
    ) -> Result<(), ManifestError> {
        let arg_map: BTreeMap<&str, &ArgDecl> =
            self.args.iter().map(|a| (a.id.as_str(), a)).collect();
        if arg_map.len() != self.args.len() {
            return Err(ManifestError::Parse(format!(
                "duplicate argument id in manifest `{}`",
                self.name
            )));
        }
        let layout_map: BTreeMap<&str, &ArgLayout> = self
            .signature
            .layout
            .iter()
            .map(|l| (l.arg.as_str(), l))
            .collect();

        // Rule 1: every arg appears in reads or writes; no undeclared arg named.
        let mut referenced = BTreeSet::new();
        for fp in self.signature.reads.iter().chain(&self.signature.writes) {
            if !arg_map.contains_key(fp.arg.as_str()) {
                return Err(ManifestError::FootprintOutOfRange {
                    kernel: self.name.clone(),
                    arg: fp.arg.clone(),
                    detail: "footprint names undeclared argument".into(),
                });
            }
            referenced.insert(fp.arg.as_str());
        }
        for lay in &self.signature.layout {
            if !arg_map.contains_key(lay.arg.as_str()) {
                return Err(ManifestError::LayoutMismatch {
                    kernel: self.name.clone(),
                    arg: lay.arg.clone(),
                    expected: "declared argument".into(),
                    observed: "undeclared argument in layout".into(),
                });
            }
        }
        for (a, b) in &self.signature.may_alias {
            if !arg_map.contains_key(a.as_str()) || !arg_map.contains_key(b.as_str()) {
                return Err(ManifestError::FootprintOutOfRange {
                    kernel: self.name.clone(),
                    arg: format!("{a},{b}"),
                    detail: "may_alias names undeclared argument".into(),
                });
            }
        }
        for arg in &self.args {
            if !referenced.contains(arg.id.as_str()) {
                return Err(ManifestError::FootprintOutOfRange {
                    kernel: self.name.clone(),
                    arg: arg.id.clone(),
                    detail: "buffer argument appears in neither reads nor writes".into(),
                });
            }
            if !layout_map.contains_key(arg.id.as_str()) {
                return Err(ManifestError::LayoutMismatch {
                    kernel: self.name.clone(),
                    arg: arg.id.clone(),
                    expected: "declared ArgLayout entry".into(),
                    observed: "missing".into(),
                });
            }
        }

        // Rule 2: 0 <= lo <= hi <= alloc_extent for every footprint and admitted shape class.
        for class in classes {
            for fp in self.signature.reads.iter().chain(&self.signature.writes) {
                let lay = layout_map[fp.arg.as_str()];
                let lo = fp
                    .lo
                    .eval(class)
                    .ok_or_else(|| ManifestError::FootprintOutOfRange {
                        kernel: self.name.clone(),
                        arg: fp.arg.clone(),
                        detail: format!("lo overflow on class `{}`", class.name),
                    })?;
                let hi = fp
                    .hi
                    .eval(class)
                    .ok_or_else(|| ManifestError::FootprintOutOfRange {
                        kernel: self.name.clone(),
                        arg: fp.arg.clone(),
                        detail: format!("hi overflow on class `{}`", class.name),
                    })?;
                let alloc = lay.alloc_extent.eval(class).ok_or_else(|| {
                    ManifestError::FootprintOutOfRange {
                        kernel: self.name.clone(),
                        arg: fp.arg.clone(),
                        detail: format!("alloc_extent overflow on class `{}`", class.name),
                    }
                })?;
                if lo > hi || hi > alloc {
                    return Err(ManifestError::FootprintOutOfRange {
                        kernel: self.name.clone(),
                        arg: fp.arg.clone(),
                        detail: format!(
                            "class `{}` footprint [{lo}, {hi}) exceeds alloc_extent {alloc}",
                            class.name
                        ),
                    });
                }
            }
        }

        // Rule 3: AllWorlds only on Stat; Stat never feeds state (INV-SPARK-04).
        for fp in self.signature.reads.iter().chain(&self.signature.writes) {
            if fp.lanes == LaneScope::AllWorlds && self.class != KernelClass::Stat {
                return Err(ManifestError::CrossWorldFootprint {
                    kernel: self.name.clone(),
                    detail: format!(
                        "arg `{}` declares LaneScope::AllWorlds on non-Stat class {:?}",
                        fp.arg, self.class
                    ),
                });
            }
        }
        if self.class == KernelClass::Stat && self.signature.feeds_state {
            return Err(ManifestError::CrossWorldFootprint {
                kernel: self.name.clone(),
                detail: "Stat kernel must have feeds_state = false (INV-SPARK-04)".into(),
            });
        }

        // Rule 4: Exact / Guarded / role constraints.
        match self.class {
            KernelClass::Exact => {
                if self.determinism != DetClass::D0Exact {
                    return Err(ManifestError::ClassMismatch {
                        kernel: self.name.clone(),
                        detail: format!(
                            "Exact kernel requires D0 determinism, got {}",
                            self.determinism
                        ),
                    });
                }
                for a in &self.args {
                    let ok = matches!(a.elem, ElemType::U32 | ElemType::Bool)
                        || (matches!(a.elem, ElemType::F32 | ElemType::F64)
                            && a.view == ArgView::Bits);
                    if !ok {
                        return Err(ManifestError::ClassMismatch {
                            kernel: self.name.clone(),
                            detail: format!(
                                "Exact kernel arg `{}` has elem {:?} view {:?}; requires u32, bool, or float with Bits view",
                                a.id, a.elem, a.view
                            ),
                        });
                    }
                }
            }
            KernelClass::Guarded => {
                let num = &self.signature.numerics;
                let (Some(eps), Some(tau), Some(fb)) =
                    (num.error_bound, num.deferral_threshold, &num.exact_fallback)
                else {
                    return Err(ManifestError::ClassMismatch {
                        kernel: self.name.clone(),
                        detail: "Guarded kernel requires error_bound, deferral_threshold, and exact_fallback".into(),
                    });
                };
                if !eps.is_finite() || !tau.is_finite() || eps < 0.0 || tau < eps || fb.is_empty() {
                    return Err(ManifestError::ClassMismatch {
                        kernel: self.name.clone(),
                        detail: format!(
                            "Guarded kernel invalid bounds eps={eps}, tau={tau}, fallback=`{fb}`"
                        ),
                    });
                }
            }
            KernelClass::Float | KernelClass::Stat => {}
        }
        if self.class == KernelClass::Float
            && matches!(self.role, KernelRole::StrictPredicate | KernelRole::Digest)
        {
            return Err(ManifestError::ClassMismatch {
                kernel: self.name.clone(),
                detail: format!(
                    "role {:?} with class Float is refused (FROB-REF-252 build-time rule)",
                    self.role
                ),
            });
        }
        if self.role == KernelRole::NegativeControl && !allow_negative_control {
            return Err(ManifestError::ClassMismatch {
                kernel: self.name.clone(),
                detail: "NegativeControl manifest allowed only in conformance registry".into(),
            });
        }

        // Rule 5: buffer_args matches args.len() and program resources match.
        if self.signature.resources.buffer_args as usize != self.args.len() {
            return Err(ManifestError::ResourceMismatch {
                kernel: self.name.clone(),
                detail: format!(
                    "resources.buffer_args={} != args.len()={}",
                    self.signature.resources.buffer_args,
                    self.args.len()
                ),
            });
        }
        if let Some(prog) = program
            && !resources_equal(prog.resources, self.signature.resources)
        {
            return Err(ManifestError::ResourceMismatch {
                kernel: self.name.clone(),
                detail: format!(
                    "IR resources {:?} != manifest resources {:?}",
                    prog.resources, self.signature.resources
                ),
            });
        }

        // Rule 6: neutral elements for padded outputs consumed by reductions.
        for (arg, consumer) in &self.signature.numerics.padded_reduction_outputs {
            let spec = self
                .signature
                .numerics
                .neutrals
                .iter()
                .find(|n| &n.arg == arg && n.consumer == *consumer)
                .ok_or_else(|| ManifestError::NeutralMissing {
                    kernel: self.name.clone(),
                    arg: arg.clone(),
                    detail: format!("missing neutral specification for {:?} consumer", consumer),
                })?;
            let prec = self.signature.numerics.precision;
            if !neutral_is_valid(*consumer, prec, spec) {
                return Err(ManifestError::NeutralMissing {
                    kernel: self.name.clone(),
                    arg: arg.clone(),
                    detail: format!(
                        "invalid neutral for {:?} ({:?}): bits={:?}, identity_rows={}",
                        consumer, prec, spec.value_bits, spec.identity_rows
                    ),
                });
            }
        }

        // Rule 7: IR load/store containment (PO-SPARK-04).
        if let Some(prog) = program {
            for class in classes {
                check_containment(
                    &self.name,
                    "read",
                    &prog.reads,
                    &self.signature.reads,
                    class,
                )?;
                check_containment(
                    &self.name,
                    "write",
                    &prog.writes,
                    &self.signature.writes,
                    class,
                )?;
            }
        }

        Ok(())
    }

    // --- Binding Validation (Rules 1..=5) ---

    /// Validate concrete slab bindings for one launch stage under `class` and `limits`.
    pub fn validate_binding(
        &self,
        bindings: &[SlabBinding],
        class: &ShapeClassCaps,
        limits: BackendLimits,
        declared_outputs: &[DeclaredOutput],
        prior_stage_writes: &[StageWriteEpoch],
    ) -> Result<(), ManifestError> {
        // Rule 5: backend limits checked before compilation/launch.
        if self.signature.resources.buffer_args > limits.max_buffer_args {
            return Err(ManifestError::ResourceMismatch {
                kernel: self.name.clone(),
                detail: format!(
                    "buffer_args={} exceeds backend `{}` limit {}",
                    self.signature.resources.buffer_args, limits.name, limits.max_buffer_args
                ),
            });
        }
        if self.signature.resources.stack_bytes > limits.max_stack_bytes {
            return Err(ManifestError::ResourceMismatch {
                kernel: self.name.clone(),
                detail: format!(
                    "stack_bytes={} exceeds backend `{}` limit {}",
                    self.signature.resources.stack_bytes, limits.name, limits.max_stack_bytes
                ),
            });
        }

        let bind_map: BTreeMap<&str, &SlabBinding> =
            bindings.iter().map(|b| (b.arg.as_str(), b)).collect();
        let arg_map: BTreeMap<&str, &ArgDecl> =
            self.args.iter().map(|a| (a.id.as_str(), a)).collect();

        // Rule 1: layout match (including stride unit and signedness) + Stat-output isolation.
        for lay in &self.signature.layout {
            let bind =
                bind_map
                    .get(lay.arg.as_str())
                    .ok_or_else(|| ManifestError::LayoutMismatch {
                        kernel: self.name.clone(),
                        arg: lay.arg.clone(),
                        expected: "bound slab".into(),
                        observed: "unbound".into(),
                    })?;
            let decl =
                arg_map
                    .get(lay.arg.as_str())
                    .ok_or_else(|| ManifestError::LayoutMismatch {
                        kernel: self.name.clone(),
                        arg: lay.arg.clone(),
                        expected: "declared arg".into(),
                        observed: "missing".into(),
                    })?;
            if bind.from_stat_kernel && self.class != KernelClass::Stat {
                return Err(ManifestError::CrossWorldFootprint {
                    kernel: self.name.clone(),
                    detail: format!(
                        "Stat-produced slab `{}` bound to non-Stat kernel arg `{}` (INV-SPARK-04)",
                        bind.slab_id, bind.arg
                    ),
                });
            }
            let elem_ok = bind.elem == lay.elem
                && (decl.view == ArgView::Value
                    || matches!(bind.elem, ElemType::F32 | ElemType::F64));
            let layout_ok = elem_ok
                && bind.extents == lay.extents
                && bind.strides == lay.strides
                && bind.stride_unit == lay.stride_unit
                && bind.stride_signed == lay.stride_signed
                && bind.origin == lay.origin
                && bind.alloc_extent == lay.alloc_extent;
            if !layout_ok {
                return Err(ManifestError::LayoutMismatch {
                    kernel: self.name.clone(),
                    arg: lay.arg.clone(),
                    expected: format!(
                        "elem={:?} unit={:?} signed={} strides={:?} origin={:?} alloc={:?}",
                        lay.elem,
                        lay.stride_unit,
                        lay.stride_signed,
                        lay.strides,
                        lay.origin,
                        lay.alloc_extent
                    ),
                    observed: format!(
                        "elem={:?} unit={:?} signed={} strides={:?} origin={:?} alloc={:?}",
                        bind.elem,
                        bind.stride_unit,
                        bind.stride_signed,
                        bind.strides,
                        bind.origin,
                        bind.alloc_extent
                    ),
                });
            }
        }

        // Compute concrete read/write ranges per arg on backing slabs.
        let mut arg_ranges: BTreeMap<&str, (String, u64, u64, bool)> = BTreeMap::new();
        for fp in &self.signature.reads {
            let bind = bind_map[fp.arg.as_str()];
            let lo = bind
                .slab_offset
                .saturating_add(fp.lo.eval(class).unwrap_or(u64::MAX));
            let hi = bind
                .slab_offset
                .saturating_add(fp.hi.eval(class).unwrap_or(u64::MAX));
            arg_ranges
                .entry(fp.arg.as_str())
                .and_modify(|e| {
                    e.1 = e.1.min(lo);
                    e.2 = e.2.max(hi);
                })
                .or_insert((bind.slab_id.clone(), lo, hi, false));
        }
        for fp in &self.signature.writes {
            let bind = bind_map[fp.arg.as_str()];
            let lo = bind
                .slab_offset
                .saturating_add(fp.lo.eval(class).unwrap_or(u64::MAX));
            let hi = bind
                .slab_offset
                .saturating_add(fp.hi.eval(class).unwrap_or(u64::MAX));
            arg_ranges
                .entry(fp.arg.as_str())
                .and_modify(|e| {
                    e.1 = e.1.min(lo);
                    e.2 = e.2.max(hi);
                    e.3 = true;
                })
                .or_insert((bind.slab_id.clone(), lo, hi, true));
        }

        // Rule 2: overlapping ranges on same slab where at least one is written must be in may_alias.
        let allowed_alias: BTreeSet<(&str, &str)> = self
            .signature
            .may_alias
            .iter()
            .flat_map(|(a, b)| [(a.as_str(), b.as_str()), (b.as_str(), a.as_str())])
            .collect();
        let entries: Vec<(&str, &(String, u64, u64, bool))> =
            arg_ranges.iter().map(|(&k, v)| (k, v)).collect();
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (arg_a, (slab_a, lo_a, hi_a, wr_a)) = entries[i];
                let (arg_b, (slab_b, lo_b, hi_b, wr_b)) = entries[j];
                if slab_a == slab_b && (*wr_a || *wr_b) && *lo_a < *hi_b && *lo_b < *hi_a {
                    if !allowed_alias.contains(&(arg_a, arg_b)) {
                        return Err(ManifestError::UndeclaredAlias {
                            kernel: self.name.clone(),
                            arg_a: arg_a.to_owned(),
                            arg_b: arg_b.to_owned(),
                            slab: slab_a.clone(),
                        });
                    }
                }
            }
        }

        // Rule 3: write footprint mapped through binding must lie inside plan's declared output range.
        for fp in &self.signature.writes {
            let bind = bind_map[fp.arg.as_str()];
            let lo = bind
                .slab_offset
                .saturating_add(fp.lo.eval(class).unwrap_or(u64::MAX));
            let hi = bind
                .slab_offset
                .saturating_add(fp.hi.eval(class).unwrap_or(u64::MAX));
            let covered = declared_outputs.iter().any(|out| {
                out.arg == fp.arg && out.slab_id == bind.slab_id && out.lo <= lo && hi <= out.hi
            });
            if !covered {
                return Err(ManifestError::UndeclaredAlias {
                    kernel: self.name.clone(),
                    arg_a: fp.arg.clone(),
                    arg_b: "<undeclared-output>".into(),
                    slab: bind.slab_id.clone(),
                });
            }
        }

        // Rule 4: epoch consistency — reading epoch e - 1 cannot bind a slab written at epoch e unordered.
        for fp in &self.signature.reads {
            let bind = bind_map[fp.arg.as_str()];
            for w in prior_stage_writes {
                if w.slab_id == bind.slab_id
                    && w.epoch_write > self.signature.epoch_read
                    && !w.ordered_before
                {
                    return Err(ManifestError::ClassMismatch {
                        kernel: self.name.clone(),
                        detail: format!(
                            "epoch conflict on slab `{}`: reads epoch {}, unordered stage writes epoch {}",
                            bind.slab_id, self.signature.epoch_read, w.epoch_write
                        ),
                    });
                }
            }
        }

        Ok(())
    }
}

fn neutral_is_valid(
    consumer: ReductionConsumer,
    prec: PrecisionProfile,
    spec: &NeutralSpec,
) -> bool {
    match consumer {
        ReductionConsumer::Factorization => spec.identity_rows,
        ReductionConsumer::Sum => match prec {
            PrecisionProfile::F32 => spec.value_bits == Some(u64::from((-0.0_f32).to_bits())),
            PrecisionProfile::F64 => spec.value_bits == Some((-0.0_f64).to_bits()),
            PrecisionProfile::ExactU32 => spec.value_bits == Some(0),
        },
        ReductionConsumer::Product => match prec {
            PrecisionProfile::F32 => spec.value_bits == Some(u64::from(1.0_f32.to_bits())),
            PrecisionProfile::F64 => spec.value_bits == Some(1.0_f64.to_bits()),
            PrecisionProfile::ExactU32 => spec.value_bits == Some(1),
        },
        ReductionConsumer::Min => match prec {
            PrecisionProfile::F32 => spec.value_bits == Some(u64::from(f32::INFINITY.to_bits())),
            PrecisionProfile::F64 => spec.value_bits == Some(f64::INFINITY.to_bits()),
            PrecisionProfile::ExactU32 => spec.value_bits == Some(u64::from(u32::MAX)),
        },
        ReductionConsumer::Max => match prec {
            PrecisionProfile::F32 => {
                spec.value_bits == Some(u64::from(f32::NEG_INFINITY.to_bits()))
            }
            PrecisionProfile::F64 => spec.value_bits == Some(f64::NEG_INFINITY.to_bits()),
            PrecisionProfile::ExactU32 => spec.value_bits == Some(0),
        },
    }
}

fn check_containment(
    kernel: &str,
    dir: &str,
    prog_fps: &[Footprint],
    decl_fps: &[Footprint],
    class: &ShapeClassCaps,
) -> Result<(), ManifestError> {
    for p in prog_fps {
        let p_lo =
            p.lo.eval(class)
                .ok_or_else(|| ManifestError::LoadStoreNotContained {
                    kernel: kernel.to_owned(),
                    arg: p.arg.clone(),
                    detail: format!("{dir} lo overflow on class `{}`", class.name),
                })?;
        let p_hi =
            p.hi.eval(class)
                .ok_or_else(|| ManifestError::LoadStoreNotContained {
                    kernel: kernel.to_owned(),
                    arg: p.arg.clone(),
                    detail: format!("{dir} hi overflow on class `{}`", class.name),
                })?;
        if p_lo == p_hi {
            continue;
        }
        let contained = decl_fps.iter().any(|d| {
            d.arg == p.arg
                && (d.lanes == p.lanes || d.lanes == LaneScope::AllWorlds)
                && d.lo.eval(class).is_some_and(|lo| lo <= p_lo)
                && d.hi.eval(class).is_some_and(|hi| p_hi <= hi)
        });
        if !contained {
            return Err(ManifestError::LoadStoreNotContained {
                kernel: kernel.to_owned(),
                arg: p.arg.clone(),
                detail: format!(
                    "program {dir} [{p_lo}, {p_hi}) on class `{}` is not contained in declared {dir} footprints",
                    class.name
                ),
            });
        }
    }
    Ok(())
}

// --- Registry Loader (registries/kernels.toml) & Doctor Section ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelOrigin {
    Derived {
        derivation_hash: String,
        source_hash: String,
    },
    Skeleton,
    Handwritten {
        reason: String,
        adr: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweringRef {
    pub arg: String,
    pub model: String,
    pub field_path: String,
    pub lowering_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelRegistryRow {
    pub name: String,
    pub manifest_path: String,
    pub id: KernelId,
    pub origin: KernelOrigin,
    pub lowers: Vec<LoweringRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelRegistry {
    pub digest: Blake3,
    pub rows: Vec<KernelRegistryRow>,
}

impl KernelRegistry {
    /// Parse `registries/kernels.toml` strictly via `frob_core::registry::toml`.
    /// Requires `origin` on every row; accepts missing `lowers` entries so N10a-03 D-1 can report them.
    pub fn parse_toml(bytes: &[u8]) -> Result<Self, ManifestError> {
        let root = toml::parse(bytes).map_err(|e| ManifestError::Registry(e.to_string()))?;
        let mut top = Fields::new(root, "registries/kernels.toml");
        let schema = top
            .require_str("schema")
            .map_err(|e| ManifestError::Registry(e.to_string()))?;
        if schema != KERNEL_REGISTRY_SCHEMA {
            return Err(top.fail(ManifestError::Registry(format!(
                "expected schema `{KERNEL_REGISTRY_SCHEMA}`, got `{schema}`"
            ))));
        }
        let mut rows = Vec::new();
        if let Some(arr) = top
            .opt_array("kernel")
            .map_err(|e| ManifestError::Registry(e.to_string()))?
        {
            for (idx, sp) in arr.into_iter().enumerate() {
                let tbl = match sp.value {
                    TVal::Table(t) => t,
                    _ => {
                        return Err(top.fail(ManifestError::Registry(format!(
                            "kernel[{idx}] must be a table"
                        ))));
                    }
                };
                let ctx = format!("kernel[{idx}]");
                let mut f = Fields::new(tbl, &ctx);
                let name = f
                    .require_str("name")
                    .map_err(|e| ManifestError::Registry(e.to_string()))?;
                let manifest_path = f
                    .require_str("manifest")
                    .map_err(|e| ManifestError::Registry(e.to_string()))?;
                let id_s = f
                    .require_str("id")
                    .map_err(|e| ManifestError::Registry(e.to_string()))?;
                let id = match KernelId::from_hex(&id_s) {
                    Ok(id) => id,
                    Err(e) => return Err(f.fail(e)),
                };

                // `origin` is required; error must name the kernel.
                let origin = match parse_origin(&mut f, &name) {
                    Ok(o) => o,
                    Err(e) => return Err(f.fail(e)),
                };

                let mut lowers = Vec::new();
                if let Some(l_arr) = f
                    .opt_array("lowers")
                    .map_err(|e| ManifestError::Registry(e.to_string()))?
                {
                    for (li, lsp) in l_arr.into_iter().enumerate() {
                        let ltbl = match lsp.value {
                            TVal::Table(t) => t,
                            _ => {
                                return Err(f.fail(ManifestError::Registry(format!(
                                    "kernel `{name}` lowers[{li}] must be table"
                                ))));
                            }
                        };
                        let mut lf = Fields::new(ltbl, &format!("{name}.lowers[{li}]"));
                        let arg = lf
                            .require_str("arg")
                            .map_err(|e| ManifestError::Registry(e.to_string()))?;
                        let model = lf
                            .require_str("model")
                            .map_err(|e| ManifestError::Registry(e.to_string()))?;
                        let field_path = lf
                            .require_str("field_path")
                            .map_err(|e| ManifestError::Registry(e.to_string()))?;
                        let lowering_hash = lf
                            .require_str("lowering_hash")
                            .map_err(|e| ManifestError::Registry(e.to_string()))?;
                        lf.finish()
                            .map_err(|e| ManifestError::Registry(e.to_string()))?;
                        lowers.push(LoweringRef {
                            arg,
                            model,
                            field_path,
                            lowering_hash,
                        });
                    }
                }
                f.finish()
                    .map_err(|e| ManifestError::Registry(e.to_string()))?;
                rows.push(KernelRegistryRow {
                    name,
                    manifest_path,
                    id,
                    origin,
                    lowers,
                });
            }
        }
        top.finish()
            .map_err(|e| ManifestError::Registry(e.to_string()))?;
        Ok(Self {
            digest: hash_blob(bytes),
            rows,
        })
    }
}

fn parse_origin(f: &mut Fields, kernel: &str) -> Result<KernelOrigin, ManifestError> {
    let Some(sp) = f.take("origin") else {
        return Err(ManifestError::Registry(format!(
            "kernel `{kernel}` is missing required `origin` provenance field"
        )));
    };
    match sp.value {
        TVal::Str(s) if s == "skeleton" => Ok(KernelOrigin::Skeleton),
        TVal::Str(s) => Err(ManifestError::Registry(format!(
            "kernel `{kernel}` has invalid string origin `{s}` (expected `skeleton`)"
        ))),
        TVal::Table(origin_tbl) => {
            let mut of = Fields::new(origin_tbl, &format!("{kernel}.origin"));
            let kind = of.require_str("kind").map_err(|_| {
                ManifestError::Registry(format!("kernel `{kernel}` origin missing `kind`"))
            })?;
            let res = match kind.as_str() {
                "skeleton" => KernelOrigin::Skeleton,
                "derived" => {
                    let derivation_hash = of
                        .require_str("derivation_hash")
                        .map_err(|e| ManifestError::Registry(format!("kernel `{kernel}`: {e}")))?;
                    let source_hash = of
                        .require_str("source_hash")
                        .map_err(|e| ManifestError::Registry(format!("kernel `{kernel}`: {e}")))?;
                    KernelOrigin::Derived {
                        derivation_hash,
                        source_hash,
                    }
                }
                "handwritten" => {
                    let reason = of
                        .require_str("reason")
                        .map_err(|e| ManifestError::Registry(format!("kernel `{kernel}`: {e}")))?;
                    let adr = of
                        .require_str("adr")
                        .map_err(|e| ManifestError::Registry(format!("kernel `{kernel}`: {e}")))?;
                    KernelOrigin::Handwritten { reason, adr }
                }
                other => {
                    return Err(of.fail(ManifestError::Registry(format!(
                        "kernel `{kernel}` unknown origin kind `{other}`"
                    ))));
                }
            };
            of.finish()
                .map_err(|e| ManifestError::Registry(format!("kernel `{kernel}`: {e}")))?;
            Ok(res)
        }
        _ => Err(ManifestError::Registry(format!(
            "kernel `{kernel}` has invalid `origin` field"
        ))),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorKernelFinding {
    pub kernel: String,
    pub rule: String,
    pub error: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorKernelsReport {
    pub registry_path: String,
    pub registry_digest: String,
    pub manifest_count: usize,
    pub findings: Vec<DoctorKernelFinding>,
    pub exit_code: u8,
}

impl DoctorKernelsReport {
    /// Validate `kernels.toml` inside `registry_dir` and every manifest it references.
    /// Exits 0 with zero findings when all validate, or 2 with one finding per failure.
    #[must_use]
    pub fn check_registry_dir(registry_dir: &Path) -> Self {
        let reg_path = registry_dir.join("kernels.toml");
        let bytes = match fs::read(&reg_path) {
            Ok(b) => b,
            Err(e) => {
                return Self {
                    registry_path: reg_path.display().to_string(),
                    registry_digest: String::new(),
                    manifest_count: 0,
                    findings: vec![DoctorKernelFinding {
                        kernel: "<registry>".into(),
                        rule: "registry-load".into(),
                        error: "Registry".into(),
                        detail: format!("failed reading {}: {e}", reg_path.display()),
                    }],
                    exit_code: 2,
                };
            }
        };
        let digest = hash_blob(&bytes).to_hex();
        let reg = match KernelRegistry::parse_toml(&bytes) {
            Ok(r) => r,
            Err(e) => {
                return Self {
                    registry_path: reg_path.display().to_string(),
                    registry_digest: digest,
                    manifest_count: 0,
                    findings: vec![DoctorKernelFinding {
                        kernel: "<registry>".into(),
                        rule: "registry-parse".into(),
                        error: e.variant_name().into(),
                        detail: e.to_string(),
                    }],
                    exit_code: 2,
                };
            }
        };
        let classes = ShapeClassCaps::default_classes();
        let mut findings = Vec::new();
        for row in &reg.rows {
            let m_path = registry_dir.join(&row.manifest_path);
            let m_bytes = match fs::read(&m_path) {
                Ok(b) => b,
                Err(e) => {
                    findings.push(DoctorKernelFinding {
                        kernel: row.name.clone(),
                        rule: "manifest-read".into(),
                        error: "Parse".into(),
                        detail: format!("failed reading {}: {e}", m_path.display()),
                    });
                    continue;
                }
            };
            let manifest = match KernelManifest::parse_json(&m_bytes) {
                Ok(m) => m,
                Err(e) => {
                    findings.push(DoctorKernelFinding {
                        kernel: row.name.clone(),
                        rule: "manifest-parse".into(),
                        error: e.variant_name().into(),
                        detail: e.to_string(),
                    });
                    continue;
                }
            };
            if manifest.id != row.id {
                findings.push(DoctorKernelFinding {
                    kernel: row.name.clone(),
                    rule: "manifest-id".into(),
                    error: "Registry".into(),
                    detail: format!("row id {} != manifest id {}", row.id, manifest.id),
                });
                continue;
            }
            if let Err(e) = manifest.validate_static(&classes, None, false) {
                findings.push(DoctorKernelFinding {
                    kernel: row.name.clone(),
                    rule: "static-validation".into(),
                    error: e.variant_name().into(),
                    detail: e.to_string(),
                });
            }
        }
        let exit_code = if findings.is_empty() { 0 } else { 2 };
        Self {
            registry_path: reg_path.display().to_string(),
            registry_digest: digest,
            manifest_count: reg.rows.len(),
            findings,
            exit_code,
        }
    }

    pub fn to_canonical_json(&self) -> Result<CanonicalBytes, ManifestError> {
        let mut w = CanonWriter::new();
        w.value()?.object(|o| {
            o.int53("exit_code", i64::from(self.exit_code))?;
            o.member("findings", |fv| {
                fv.array(|fa| {
                    for f in &self.findings {
                        fa.push(|ev| {
                            ev.object(|fo| {
                                fo.str("detail", &f.detail)?;
                                fo.str("error", &f.error)?;
                                fo.str("kernel", &f.kernel)?;
                                fo.str("rule", &f.rule)?;
                                Ok(())
                            })
                        })?;
                    }
                    Ok(())
                })
            })?;
            o.int53("manifest_count", self.manifest_count as i64)?;
            o.str("registry_digest", &self.registry_digest)?;
            o.str("registry_path", &self.registry_path)?;
            Ok(())
        })?;
        Ok(w.finish()?)
    }
}

// --- JSON Helper Parsers ---

fn obj_map<'a>(
    sp: &'a Spanned<'a>,
    ctx: &str,
) -> Result<BTreeMap<&'a str, &'a Spanned<'a>>, ManifestError> {
    let members = sp
        .value
        .as_object()
        .ok_or_else(|| ManifestError::Parse(format!("{ctx} must be a JSON object")))?;
    let mut map = BTreeMap::new();
    for m in members {
        map.insert(m.name.as_str(), &m.value);
    }
    Ok(map)
}

fn req_field<'a>(
    map: &BTreeMap<&'a str, &'a Spanned<'a>>,
    key: &str,
    ctx: &str,
) -> Result<&'a Spanned<'a>, ManifestError> {
    map.get(key)
        .copied()
        .ok_or_else(|| ManifestError::Parse(format!("{ctx} missing required field `{key}`")))
}

fn req_str<'a>(
    map: &BTreeMap<&'a str, &'a Spanned<'a>>,
    key: &str,
    ctx: &str,
) -> Result<&'a str, ManifestError> {
    req_field(map, key, ctx)?
        .value
        .as_str()
        .ok_or_else(|| ManifestError::Parse(format!("{ctx}.{key} must be string")))
}

fn req_bool(
    map: &BTreeMap<&str, &Spanned<'_>>,
    key: &str,
    ctx: &str,
) -> Result<bool, ManifestError> {
    req_field(map, key, ctx)?
        .value
        .as_bool()
        .ok_or_else(|| ManifestError::Parse(format!("{ctx}.{key} must be bool")))
}

fn req_arr<'a>(
    map: &BTreeMap<&'a str, &'a Spanned<'a>>,
    key: &str,
    ctx: &str,
) -> Result<&'a [Spanned<'a>], ManifestError> {
    req_field(map, key, ctx)?
        .value
        .as_array()
        .ok_or_else(|| ManifestError::Parse(format!("{ctx}.{key} must be array")))
}

fn req_int53(
    map: &BTreeMap<&str, &Spanned<'_>>,
    key: &str,
    ctx: &str,
) -> Result<i64, ManifestError> {
    req_field(map, key, ctx)?
        .as_int53()
        .map_err(|e| ManifestError::Parse(format!("{ctx}.{key} int53: {e}")))
}

fn req_u64(map: &BTreeMap<&str, &Spanned<'_>>, key: &str, ctx: &str) -> Result<u64, ManifestError> {
    let n = req_int53(map, key, ctx)?;
    u64::try_from(n)
        .map_err(|_| ManifestError::Parse(format!("{ctx}.{key} must be non-negative, got {n}")))
}

fn req_u32(map: &BTreeMap<&str, &Spanned<'_>>, key: &str, ctx: &str) -> Result<u32, ManifestError> {
    let n = req_int53(map, key, ctx)?;
    u32::try_from(n).map_err(|_| ManifestError::Parse(format!("{ctx}.{key} must fit u32, got {n}")))
}

fn opt_f64(
    map: &BTreeMap<&str, &Spanned<'_>>,
    key: &str,
    ctx: &str,
) -> Result<Option<f64>, ManifestError> {
    let sp = req_field(map, key, ctx)?;
    if matches!(sp.value, Json::Null) {
        return Ok(None);
    }
    let (_, val) = sp
        .as_f64_lexeme()
        .map_err(|e| ManifestError::Parse(format!("{ctx}.{key} f64: {e}")))?;
    Ok(Some(val))
}

fn parse_cap_expr(sp: &Spanned<'_>) -> Result<CapExpr, ManifestError> {
    let o = obj_map(sp, "cap_expr")?;
    Ok(CapExpr {
        constant: req_u64(&o, "constant", "cap_expr")?,
        nv_cap: req_u64(&o, "nv_cap", "cap_expr")?,
        link_cap: req_u64(&o, "link_cap", "cap_expr")?,
        contact_cap: req_u64(&o, "contact_cap", "cap_expr")?,
        row_cap: req_u64(&o, "row_cap", "cap_expr")?,
        sample_cap: req_u64(&o, "sample_cap", "cap_expr")?,
        actuator_cap: req_u64(&o, "actuator_cap", "cap_expr")?,
        worlds: req_u64(&o, "worlds", "cap_expr")?,
    })
}

fn parse_footprint(sp: &Spanned<'_>) -> Result<Footprint, ManifestError> {
    let o = obj_map(sp, "footprint")?;
    Ok(Footprint {
        arg: req_str(&o, "arg", "footprint")?.to_owned(),
        lanes: LaneScope::parse(req_str(&o, "lanes", "footprint")?)?,
        lo: parse_cap_expr(req_field(&o, "lo", "footprint")?)?,
        hi: parse_cap_expr(req_field(&o, "hi", "footprint")?)?,
    })
}

fn resources_equal(lhs: ResourceBounds, rhs: ResourceBounds) -> bool {
    lhs == rhs
}
