//! Self-test kernel `K-selftest` (`BD-014`).
//!
//! For each lane `i`:
//! - `out_f[i] = a[i] + b[i]` in `f32` (plain IEEE-754 addition, no `mul_add`;
//!   padding lanes hold `-0.0` so `-0.0 + (-0.0) = -0.0`).
//! - `out_u[i] = x[i].wrapping_add(y[i])` in `u32` (padding lanes hold `0`).

use std::collections::BTreeMap;
use std::ops::Range;

use frob_core::evidence::DetClass;
use frob_core::id::Blake3;

use crate::backend::{CpuKernelSymbol, KernelArgs, KernelEntry};
use crate::harness::{HarnessInput, HarnessOutputSpec};
use crate::manifest::{
    ArgDecl, ArgLayout, ArgView, CapExpr, ElemKind, ElemType, Footprint, FootprintEvidence,
    KernelClass, KernelId, KernelManifest, KernelRole, LaneScope, NumericalContract,
    PrecisionProfile, ResourceBounds, ShapeClassCaps, StageSignature, StrideUnit,
};

pub const K_SELFTEST_NAME: &str = "K-selftest";
pub const K_SELFTEST_SYMBOL: &str = "frob_kernels::selftest::k_selftest_cpu";

/// Shared-dialect source for `K-selftest` consumed by CUDA (`BD-015`) and Metal (`BD-016`).
pub const K_SELFTEST_DIALECT_SOURCE: &str = "\
FROB_KERNEL(k_selftest,
    FROB_BUF_RO(f32, a, 0),
    FROB_BUF_RO(f32, b, 1),
    FROB_BUF_RO(u32, x, 2),
    FROB_BUF_RO(u32, y, 3),
    FROB_BUF(f32, out_f, 4),
    FROB_BUF(u32, out_u, 5)
)
    const u32 i = frob_gid;
    if (i < FROB_WORLDS) {
        out_f[i] = a[i] + b[i];
        out_u[i] = x[i] + y[i];
    }
FROB_KERNEL_END
";

/// Seeded inputs for `K-selftest`, including explicit padding lanes.
#[derive(Debug, Clone, PartialEq)]
pub struct SelftestInputs {
    pub active_lanes: usize,
    pub pad_lanes: usize,
    pub a: Vec<f32>,
    pub b: Vec<f32>,
    pub x: Vec<u32>,
    pub y: Vec<u32>,
}

impl SelftestInputs {
    #[must_use]
    pub fn total_lanes(&self) -> usize {
        self.active_lanes + self.pad_lanes
    }

    #[must_use]
    pub fn a_bytes(&self) -> Vec<u8> {
        f32_slice_to_bytes(&self.a)
    }

    #[must_use]
    pub fn b_bytes(&self) -> Vec<u8> {
        f32_slice_to_bytes(&self.b)
    }

    #[must_use]
    pub fn x_bytes(&self) -> Vec<u8> {
        u32_slice_to_bytes(&self.x)
    }

    #[must_use]
    pub fn y_bytes(&self) -> Vec<u8> {
        u32_slice_to_bytes(&self.y)
    }

    /// Build the four [`HarnessInput`] descriptors (`a`, `b`, `x`, `y`) for [`crate::harness::Harness::run`].
    #[must_use]
    pub fn harness_inputs(&self) -> [HarnessInput; 4] {
        let total = self.total_lanes();
        [
            HarnessInput {
                name: "a".into(),
                elem: ElemType::F32,
                len: total,
                bytes: self.a_bytes(),
            },
            HarnessInput {
                name: "b".into(),
                elem: ElemType::F32,
                len: total,
                bytes: self.b_bytes(),
            },
            HarnessInput {
                name: "x".into(),
                elem: ElemType::U32,
                len: total,
                bytes: self.x_bytes(),
            },
            HarnessInput {
                name: "y".into(),
                elem: ElemType::U32,
                len: total,
                bytes: self.y_bytes(),
            },
        ]
    }
}

/// Scalar reference outputs for bit-for-bit verification of `K-selftest`.
#[derive(Debug, Clone, PartialEq)]
pub struct SelftestOutputs {
    pub out_f: Vec<f32>,
    pub out_u: Vec<u32>,
}

impl SelftestOutputs {
    #[must_use]
    pub fn out_f_bytes(&self) -> Vec<u8> {
        f32_slice_to_bytes(&self.out_f)
    }

    #[must_use]
    pub fn out_u_bytes(&self) -> Vec<u8> {
        u32_slice_to_bytes(&self.out_u)
    }
}

/// Output slab specifications (`out_f`, `out_u`) for `K-selftest`.
#[must_use]
pub fn k_selftest_output_specs(total_lanes: usize) -> [HarnessOutputSpec; 2] {
    [
        HarnessOutputSpec {
            name: "out_f".into(),
            elem: ElemType::F32,
            len: total_lanes,
        },
        HarnessOutputSpec {
            name: "out_u".into(),
            elem: ElemType::U32,
            len: total_lanes,
        },
    ]
}

/// Shape class matching `total_lanes` worlds for `K-selftest`.
#[must_use]
pub fn k_selftest_shape_class(total_lanes: usize) -> ShapeClassCaps {
    ShapeClassCaps {
        name: format!("selftest-w{total_lanes}"),
        nv_cap: 12,
        link_cap: 13,
        contact_cap: 24,
        row_cap: 72,
        sample_cap: 16,
        actuator_cap: 12,
        worlds: total_lanes as u64,
        extra: BTreeMap::new(),
    }
}

/// `batch-cpu` kernel symbol for `K-selftest`.
pub fn k_selftest_cpu(args: &mut KernelArgs<'_>, lanes: Range<u32>) {
    for i in lanes {
        let af = args.read_f32("a", i);
        let bf = args.read_f32("b", i);
        args.write_f32("out_f", i, af + bf);

        let xu = args.read_u32("x", i);
        let yu = args.read_u32("y", i);
        args.write_u32("out_u", i, xu.wrapping_add(yu));
    }
}

/// Scalar Rust reference loop for `K-selftest` (oracle for `BD-014`).
#[must_use]
pub fn k_selftest_scalar_reference(inputs: &SelftestInputs) -> SelftestOutputs {
    let n = inputs.total_lanes();
    let mut out_f = Vec::with_capacity(n);
    let mut out_u = Vec::with_capacity(n);
    for i in 0..n {
        out_f.push(inputs.a[i] + inputs.b[i]);
        out_u.push(inputs.x[i].wrapping_add(inputs.y[i]));
    }
    SelftestOutputs { out_f, out_u }
}

/// Deterministically generate seeded `K-selftest` inputs from `(seed, lane)`.
///
/// Padding lanes `[active_lanes .. active_lanes + pad_lanes)` hold `-0.0_f32`
/// in `a`/`b` and `0_u32` in `x`/`y`.
#[must_use]
pub fn k_selftest_seeded_inputs(
    active_lanes: usize,
    pad_lanes: usize,
    seed: u64,
) -> SelftestInputs {
    let total = active_lanes + pad_lanes;
    let mut a = Vec::with_capacity(total);
    let mut b = Vec::with_capacity(total);
    let mut x = Vec::with_capacity(total);
    let mut y = Vec::with_capacity(total);

    for lane in 0..active_lanes {
        let h0 = splitmix64(seed ^ (lane as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let h1 = splitmix64(h0 ^ 0xBF58_476D_1CE4_E5B9);
        a.push(((h0 as i32) as f32) * 0.000_000_25_f32);
        b.push((((h0 >> 32) as i32) as f32) * 0.000_000_25_f32);
        x.push(h1 as u32);
        y.push((h1 >> 32) as u32);
    }
    a.resize(total, -0.0_f32);
    b.resize(total, -0.0_f32);
    x.resize(total, 0);
    y.resize(total, 0);
    SelftestInputs {
        active_lanes,
        pad_lanes,
        a,
        b,
        x,
        y,
    }
}

/// Valid `frob.kernel/1` manifest for `K-selftest`.
#[must_use]
pub fn k_selftest_manifest() -> KernelManifest {
    let arg_decl = |id: &str, elem: ElemType, kind: ElemKind, state: bool| ArgDecl {
        id: id.into(),
        elem,
        view: ArgView::Value,
        shape: vec![CapExpr::worlds(1)],
        dim: [0; 6],
        kind,
        components: vec![id.into()],
        state,
    };
    let layout = |arg: &str, elem: ElemType| ArgLayout {
        arg: arg.into(),
        elem,
        extents: vec![CapExpr::worlds(1)],
        strides: vec![1],
        stride_unit: StrideUnit::Elements,
        stride_signed: false,
        origin: CapExpr::constant(0),
        alloc_extent: CapExpr::worlds(1),
    };
    let fp = |arg: &str| Footprint {
        arg: arg.into(),
        lanes: LaneScope::OwnWorld,
        lo: CapExpr::constant(0),
        hi: CapExpr::worlds(1),
    };

    let mut manifest = KernelManifest {
        id: KernelId(Blake3::from_bytes([0u8; 32])),
        name: K_SELFTEST_NAME.into(),
        class: KernelClass::Float,
        role: KernelRole::Ordinary,
        args: vec![
            arg_decl("a", ElemType::F32, ElemKind::Scalar, false),
            arg_decl("b", ElemType::F32, ElemKind::Scalar, false),
            arg_decl("out_f", ElemType::F32, ElemKind::Scalar, true),
            arg_decl("out_u", ElemType::U32, ElemKind::Integer, true),
            arg_decl("x", ElemType::U32, ElemKind::Integer, false),
            arg_decl("y", ElemType::U32, ElemKind::Integer, false),
        ],
        determinism: DetClass::D2PerBackend,
        identities: Vec::new(),
        derivative: None,
        cpu_reference: K_SELFTEST_SYMBOL.into(),
        signature: StageSignature {
            reads: vec![fp("a"), fp("b"), fp("x"), fp("y")],
            writes: vec![fp("out_f"), fp("out_u")],
            may_alias: Vec::new(),
            layout: vec![
                layout("a", ElemType::F32),
                layout("b", ElemType::F32),
                layout("out_f", ElemType::F32),
                layout("out_u", ElemType::U32),
                layout("x", ElemType::U32),
                layout("y", ElemType::U32),
            ],
            epoch_read: 0,
            epoch_write: 0,
            resources: ResourceBounds {
                live_values: 8,
                stack_bytes: 0,
                max_loop_bound: 1,
                buffer_args: 6,
            },
            numerics: NumericalContract {
                precision: PrecisionProfile::F32,
                error_bound: None,
                deferral_threshold: None,
                exact_fallback: None,
                neutrals: Vec::new(),
                padded_reduction_outputs: Vec::new(),
            },
            feeds_state: true,
            footprint_evidence: FootprintEvidence::Lint,
        },
    };
    manifest.id = manifest.compute_id().expect("K-selftest manifest hashes");
    manifest
}

/// Complete multi-backend [`KernelEntry`] for `K-selftest` (CPU symbol + shared dialect source).
#[must_use]
pub fn k_selftest_entry() -> KernelEntry {
    KernelEntry {
        manifest: k_selftest_manifest(),
        cpu_symbol: Some(CpuKernelSymbol {
            symbol_name: K_SELFTEST_SYMBOL.into(),
            func: k_selftest_cpu,
        }),
        dialect_source: Some(K_SELFTEST_DIALECT_SOURCE.into()),
        program_footprint: None,
        compile_bindings: Vec::new(),
    }
}

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

fn f32_slice_to_bytes(vals: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vals.len() * 4);
    for &v in vals {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn u32_slice_to_bytes(vals: &[u32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vals.len() * 4);
    for &v in vals {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}
