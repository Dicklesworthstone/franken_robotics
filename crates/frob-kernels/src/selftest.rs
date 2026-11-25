//! Self-test kernel `K-selftest` (`BD-014`).
//!
//! For each lane `i`:
//! - `out_f[i] = a[i] + b[i]` in `f32` (plain IEEE-754 addition, no `mul_add`;
//!   padding lanes hold `-0.0` so `-0.0 + (-0.0) = -0.0`).
//! - `out_u[i] = x[i].wrapping_add(y[i])` in `u32` (padding lanes hold `0`).

use std::ops::Range;

use frob_core::evidence::DetClass;
use frob_core::id::Blake3;

use crate::backend::{KernelArgs, KernelEntry, KernelSource};
use crate::manifest::{
    ArgDecl, ArgLayout, ArgView, CapExpr, ElemKind, ElemType, Footprint, FootprintEvidence,
    KernelClass, KernelId, KernelManifest, KernelRole, LaneScope, NumericalContract,
    PrecisionProfile, ResourceBounds, StageSignature, StrideUnit,
};

pub const K_SELFTEST_NAME: &str = "K-selftest";
pub const K_SELFTEST_SYMBOL: &str = "frob_kernels::selftest::k_selftest_cpu";

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
        let af = ((h0 as i32) as f32) * 0.000_000_25_f32;
        let bf = (((h0 >> 32) as i32) as f32) * 0.000_000_25_f32;
        a.push(af);
        b.push(bf);
        x.push(h1 as u32);
        y.push((h1 >> 32) as u32);
    }
    for _ in 0..pad_lanes {
        a.push(-0.0_f32);
        b.push(-0.0_f32);
        x.push(0);
        y.push(0);
    }
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

/// Complete `batch-cpu` [`KernelEntry`] for `K-selftest`.
#[must_use]
pub fn k_selftest_entry() -> KernelEntry {
    KernelEntry {
        manifest: k_selftest_manifest(),
        source: KernelSource::CpuSymbol {
            symbol_name: K_SELFTEST_SYMBOL.into(),
            func: k_selftest_cpu,
        },
        compile_bindings: Vec::new(),
    }
}

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

#[must_use]
pub fn f32_slice_to_bytes(vals: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vals.len() * 4);
    for &v in vals {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

#[must_use]
pub fn u32_slice_to_bytes(vals: &[u32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vals.len() * 4);
    for &v in vals {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}
