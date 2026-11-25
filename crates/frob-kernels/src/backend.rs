//! `KernelBackend` trait and companion seam types (`BD-014`, `ADR-SPARK-01`,
//! `INV-SPARK-01`, `INV-SPARK-03`).
//!
//! No GPU crate or GPU type appears in this module; every backend (`batch-cpu`
//! in core, CUDA/Metal in annexes) implements [`KernelBackend`]. The trait
//! offers no allocation method other than [`KernelBackend::allocate`], making
//! `INV-SPARK-03` ("no device allocation after environment creation") a property
//! of the interface.
//!
//! # Compile-fail interface invariants
//!
//! A kernel body receives [`KernelArgs`] and has no allocation hook (`INV-SPARK-03`):
//!
//! ```compile_fail
//! use frob_kernels::backend::KernelArgs;
//! fn bad_kernel(args: &mut KernelArgs<'_>) {
//!     let _ = args.allocate_slab("illegal", 64);
//! }
//! ```
//!
//! Calling [`KernelBackend::read_back`] while [`BufferSet`] is mutably borrowed
//! by [`KernelBackend::launch`] is rejected by the borrow checker:
//!
//! ```compile_fail
//! use frob_kernels::backend::{BufferSet, Cx, KernelBackend, SlabId, StepPlan};
//! fn bad_read_during_launch(
//!     backend: &mut dyn KernelBackend,
//!     plan: &StepPlan,
//!     bufs: &mut BufferSet,
//!     cx: &Cx,
//!     out: &mut [u8],
//! ) {
//!     let b_mut: &mut BufferSet = bufs;
//!     let _ = backend.read_back(bufs, SlabId(0), out);
//!     let _ = backend.launch(plan, b_mut, cx);
//! }
//! ```

use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use frob_core::refusal::{Refusal, RefusalCode, RefusalFingerprint, Repair, RetryPolicy};

use crate::manifest::{
    DeclaredOutput, ElemType, KernelId, KernelManifest, ShapeClassCaps, SlabBinding,
};
use crate::receipt::Receipt;

/// Required slab base alignment in bytes (`BD-014`).
pub const SLAB_ALIGNMENT_BYTES: usize = 256;

/// Cancellation capability context observed between kernel launches (`BD-014`).
///
/// Lightweight synchronous token matching `frob-verbs::dispatch::DispatchCx`
/// until `BD-008` (`INT-ASUP-01`) wires `asupersync::Cx` into core crates.
#[derive(Debug, Clone, Default)]
pub struct Cx {
    cancelled: Arc<AtomicBool>,
}

impl Cx {
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Signal cancellation; observed at the next kernel-launch boundary.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Return `true` if cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

/// Static identity of a [`KernelBackend`] implementation (`AE-01`..`AE-03`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendIdentity {
    pub family: String,
    pub device: String,
    pub driver: String,
    pub runtime: String,
    pub compiler: String,
}

/// Opaque index of one allocated slab inside a [`BufferSet`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SlabId(pub usize);

/// Allocation specification for one named slab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlabSpec {
    pub name: String,
    pub elem: ElemType,
    pub len: usize,
    pub alignment: usize,
    pub from_stat_kernel: bool,
}

impl SlabSpec {
    #[must_use]
    pub fn new(name: impl Into<String>, elem: ElemType, len: usize) -> Self {
        Self {
            name: name.into(),
            elem,
            len,
            alignment: SLAB_ALIGNMENT_BYTES,
            from_stat_kernel: false,
        }
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.len.saturating_mul(self.elem.byte_size())
    }
}

/// Complete slab layout allocated once at environment creation (`INV-SPARK-03`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferLayout {
    pub slabs: Vec<SlabSpec>,
}

/// One 256-byte-aligned backing slab allocated in safe Rust (zero `unsafe`).
#[derive(Debug, Clone)]
pub struct AlignedSlab {
    pub spec: SlabSpec,
    raw: Vec<u8>,
    offset: usize,
    byte_len: usize,
}

impl AlignedSlab {
    #[must_use]
    pub fn new(spec: SlabSpec) -> Self {
        let align = spec.alignment.max(SLAB_ALIGNMENT_BYTES);
        let byte_len = spec.byte_len();
        let raw = vec![0u8; byte_len.saturating_add(align)];
        let addr = raw.as_ptr() as usize;
        let rem = addr % align;
        let offset = if rem == 0 { 0 } else { align - rem };
        debug_assert!((raw[offset..offset + byte_len].as_ptr() as usize).is_multiple_of(align));
        Self {
            spec,
            raw,
            offset,
            byte_len,
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.raw[self.offset..self.offset + self.byte_len]
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let end = self.offset + self.byte_len;
        &mut self.raw[self.offset..end]
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.byte_len
    }
}

/// Environment slab set returned by [`KernelBackend::allocate`].
#[derive(Debug, Clone)]
pub struct BufferSet {
    slabs: Vec<AlignedSlab>,
}

impl BufferSet {
    #[must_use]
    pub fn allocate_aligned(layout: &BufferLayout) -> Self {
        let slabs = layout.slabs.iter().cloned().map(AlignedSlab::new).collect();
        Self { slabs }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.slabs.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.slabs.is_empty()
    }

    #[must_use]
    pub fn get(&self, id: SlabId) -> Option<&AlignedSlab> {
        self.slabs.get(id.0)
    }

    pub fn get_mut(&mut self, id: SlabId) -> Option<&mut AlignedSlab> {
        self.slabs.get_mut(id.0)
    }

    /// Write initial host bytes into a slab prior to launch.
    pub fn upload(&mut self, id: SlabId, bytes: &[u8]) -> Result<(), Refusal> {
        let slab = self.slabs.get_mut(id.0).ok_or_else(|| {
            refusal_601(
                format!("/slab/{}", id.0),
                format!("slab {} out of range", id.0),
            )
        })?;
        if bytes.len() != slab.byte_len() {
            return Err(refusal_603(
                &slab.spec.name,
                format!(
                    "upload size mismatch: got {} bytes, slab `{}` requires {}",
                    bytes.len(),
                    slab.spec.name,
                    slab.byte_len()
                ),
            ));
        }
        slab.as_bytes_mut().copy_from_slice(bytes);
        Ok(())
    }

    pub(crate) fn slabs_mut(&mut self) -> &mut [AlignedSlab] {
        &mut self.slabs
    }
}

/// Safe, non-allocating argument view passed to a `batch-cpu` kernel chunk.
///
/// Input slabs are shared read-only slices over the full lane range; output
/// slabs are disjoint mutable slices covering exactly `lane_range`.
pub struct KernelArgs<'a> {
    inputs: BTreeMap<&'a str, &'a [u8]>,
    outputs: BTreeMap<&'a str, (&'a mut [u8], u32)>,
}

impl<'a> KernelArgs<'a> {
    #[must_use]
    pub fn new(
        inputs: BTreeMap<&'a str, &'a [u8]>,
        outputs: BTreeMap<&'a str, (&'a mut [u8], u32)>,
    ) -> Self {
        Self { inputs, outputs }
    }

    /// Read one `f32` element at global `lane` from input argument `name`.
    #[must_use]
    pub fn read_f32(&self, name: &str, lane: u32) -> f32 {
        let bytes = self.inputs[name];
        let off = (lane as usize) * 4;
        f32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]])
    }

    /// Read one `u32` element at global `lane` from input argument `name`.
    #[must_use]
    pub fn read_u32(&self, name: &str, lane: u32) -> u32 {
        let bytes = self.inputs[name];
        let off = (lane as usize) * 4;
        u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]])
    }

    /// Write one `f32` element at global `lane` (within this worker's chunk) to output `name`.
    pub fn write_f32(&mut self, name: &str, lane: u32, val: f32) {
        let (buf, start_lane) = self.outputs.get_mut(name).expect("output arg bound");
        let local = (lane - *start_lane) as usize;
        let off = local * 4;
        buf[off..off + 4].copy_from_slice(&val.to_le_bytes());
    }

    /// Write one `u32` element at global `lane` (within this worker's chunk) to output `name`.
    pub fn write_u32(&mut self, name: &str, lane: u32, val: u32) {
        let (buf, start_lane) = self.outputs.get_mut(name).expect("output arg bound");
        let local = (lane - *start_lane) as usize;
        let off = local * 4;
        buf[off..off + 4].copy_from_slice(&val.to_le_bytes());
    }
}

/// Function pointer signature for a `batch-cpu` kernel symbol.
pub type CpuKernelFn = fn(&mut KernelArgs<'_>, Range<u32>);

/// Kernel implementation attached to a [`KernelManifest`].
#[derive(Clone)]
pub enum KernelSource {
    CpuSymbol {
        symbol_name: String,
        func: CpuKernelFn,
    },
    DialectText(String),
}

/// One manifest + source + compile-time binding view inside a [`KernelLibrary`].
#[derive(Clone)]
pub struct KernelEntry {
    pub manifest: KernelManifest,
    pub source: KernelSource,
    /// Optional compile-time slab bindings checked against `N21b-01` alias rules during `compile`.
    pub compile_bindings: Vec<SlabBinding>,
}

/// Library of kernels submitted to [`KernelBackend::compile`].
#[derive(Clone)]
pub struct KernelLibrary {
    pub entries: Vec<KernelEntry>,
    pub shape_class: ShapeClassCaps,
    pub declared_outputs: Vec<DeclaredOutput>,
}

/// Handle returned by [`KernelBackend::compile`] listing validated kernel IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledLibrary {
    pub kernel_ids: Vec<(String, KernelId)>,
}

impl CompiledLibrary {
    #[must_use]
    pub fn id_for(&self, name: &str) -> Option<KernelId> {
        self.kernel_ids
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
    }
}

/// Argument-to-slab binding inside a [`Stage`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageBinding {
    pub arg: String,
    pub slab: SlabId,
}

/// One stage of a batched step submitted to [`KernelBackend::plan`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage {
    pub name: String,
    pub kernel_id: KernelId,
    pub bindings: Vec<StageBinding>,
}

/// One validated launch inside a [`StepPlan`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedLaunch {
    pub stage_name: String,
    pub kernel_id: KernelId,
    pub lanes: u32,
    pub input_bindings: Vec<(String, SlabId)>,
    pub output_bindings: Vec<(String, SlabId)>,
}

/// Static ordered list of launches with fixed grid sizes for one shape class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepPlan {
    pub launches: Vec<PlannedLaunch>,
    pub class: ShapeClassCaps,
}

/// Core seam implemented by `batch-cpu` (core), CUDA (annex), and Metal (annex) (`BD-014`).
pub trait KernelBackend {
    fn compile(&mut self, lib: &KernelLibrary) -> Result<CompiledLibrary, Refusal>;
    fn allocate(&mut self, layout: &BufferLayout) -> Result<BufferSet, Refusal>;
    fn plan(&mut self, stages: &[Stage], class: &ShapeClassCaps) -> Result<StepPlan, Refusal>;
    fn launch(
        &mut self,
        plan: &StepPlan,
        bufs: &mut BufferSet,
        cx: &Cx,
    ) -> Result<Receipt, Refusal>;
    fn read_back(&mut self, bufs: &BufferSet, slab: SlabId, out: &mut [u8]) -> Result<(), Refusal>;
    fn identity(&self) -> BackendIdentity;
}

/// Construct a `FROB-REF-601` (device unavailable/lost, exit 6) refusal.
#[must_use]
pub fn refusal_601(pointer: impl Into<String>, message: impl Into<String>) -> Refusal {
    let msg = message.into();
    Refusal::new(
        RefusalCode::FrobRef601,
        msg.clone(),
        pointer,
        msg,
        "available device and valid slab/launch state",
        "device",
        RetryPolicy::Backoff,
        vec![
            Repair::new("Rerun on another device"),
            Repair::new("Rerun on batch-cpu under mixed-allowed"),
            Repair::new("Resume from the last committed epoch when the device returns"),
        ],
    )
    .with_fingerprint(RefusalFingerprint::new("batch-cpu", "batch-cpu-host-m0"))
}

/// Construct a `FROB-REF-603` (kernel compile/validation failure, exit 6) refusal
/// carrying verbatim diagnostics.
#[must_use]
pub fn refusal_603(kernel: impl Into<String>, diagnostics: impl Into<String>) -> Refusal {
    let k = kernel.into();
    let diag = diagnostics.into();
    Refusal::new(
        RefusalCode::FrobRef603,
        format!("kernel `{k}` compile/validation failed: {diag}"),
        format!("/kernel/{k}"),
        diag,
        "valid kernel manifest, signature, and backend source",
        "diagnostics",
        RetryPolicy::AfterChange,
        vec![
            Repair::new("Report the verbatim compiler diagnostics and source hash"),
            Repair::new("Use the generic kernel if a specialized kernel failed"),
        ],
    )
    .with_fingerprint(RefusalFingerprint::new("batch-cpu", "batch-cpu-host-m0"))
}
