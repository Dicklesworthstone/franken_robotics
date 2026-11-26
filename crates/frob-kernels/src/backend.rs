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
//!
//! [`BufferSet`] is not `Clone`, preventing accidental aliasing or misaligned
//! copies of 256-byte-aligned backing storage:
//!
//! ```compile_fail
//! use frob_kernels::backend::BufferSet;
//! fn bad_clone(bufs: &BufferSet) -> BufferSet {
//!     bufs.clone()
//! }
//! ```
//!
//! A caller cannot inspect a host slab directly:
//!
//! ```compile_fail,E0624
//! use frob_kernels::backend::{BufferSet, SlabId};
//! fn unchecked_read(bufs: &BufferSet) -> &[u8] {
//!     bufs.cpu_slab(SlabId(0)).unwrap().as_bytes()
//! }
//! ```

use std::collections::BTreeMap;
use std::fmt;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use frob_core::refusal::{Refusal, RefusalCode, RefusalFingerprint, Repair, RetryPolicy};

use crate::manifest::{
    DeclaredOutput, ElemType, KernelId, KernelManifest, ProgramFootprint, ShapeClassCaps,
    SlabBinding,
};
use crate::receipt::Receipt;

/// Required slab base alignment in bytes (`BD-014`).
pub const SLAB_ALIGNMENT_BYTES: usize = 256;

static NEXT_ALLOCATION_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_BACKEND_TAG: AtomicU64 = AtomicU64::new(1);

pub(crate) fn next_backend_tag() -> u64 {
    NEXT_BACKEND_TAG.fetch_add(1, Ordering::Relaxed)
}

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
///
/// Intentionally not `Clone` and with private metadata so alignment offsets and
/// validated element types cannot be invalidated after allocation.
pub(crate) struct AlignedSlab {
    spec: SlabSpec,
    raw: Vec<u8>,
    offset: usize,
    byte_len: usize,
}

impl AlignedSlab {
    #[must_use]
    pub(crate) fn new(spec: SlabSpec) -> Self {
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
    pub const fn spec(&self) -> &SlabSpec {
        &self.spec
    }

    #[must_use]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.raw[self.offset..self.offset + self.byte_len]
    }

    pub(crate) fn as_bytes_mut(&mut self) -> &mut [u8] {
        let end = self.offset + self.byte_len;
        &mut self.raw[self.offset..end]
    }

    #[must_use]
    pub const fn byte_len(&self) -> usize {
        self.byte_len
    }
}

/// Environment slab handle set returned by [`KernelBackend::allocate`].
///
/// Intentionally not `Clone`, with immutable descriptors and per-slab written
/// tracking so output slabs cannot be read before their producer completes and
/// host-to-device transfers always flow through [`KernelBackend::upload`].
///
/// GPU annex backends (`BD-015` CUDA, `BD-016` Metal) construct a device-backed
/// handle set with [`BufferSet::for_device`] (zero host slab allocation) and
/// index their native device buffers by [`BufferSet::allocation_id`].
pub struct BufferSet {
    allocation_id: u64,
    specs: Vec<SlabSpec>,
    written: Vec<bool>,
    cpu_slabs: Option<Vec<AlignedSlab>>,
}

impl fmt::Debug for BufferSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufferSet")
            .field("allocation_id", &self.allocation_id)
            .field("specs", &self.specs)
            .field("written", &self.written)
            .finish_non_exhaustive()
    }
}

impl BufferSet {
    #[must_use]
    pub(crate) fn new_cpu(layout: &BufferLayout) -> Self {
        let specs = layout.slabs.clone();
        let written = vec![false; specs.len()];
        let slabs = specs.iter().cloned().map(AlignedSlab::new).collect();
        Self {
            allocation_id: NEXT_ALLOCATION_ID.fetch_add(1, Ordering::Relaxed),
            specs,
            written,
            cpu_slabs: Some(slabs),
        }
    }

    /// Construct a device-backed handle set for a GPU annex (`BD-015`/`BD-016`).
    ///
    /// Allocates no CPU backing slabs; the annex stores native device buffers
    /// keyed by [`BufferSet::allocation_id`].
    #[must_use]
    pub fn for_device(layout: &BufferLayout) -> Self {
        let specs = layout.slabs.clone();
        let written = vec![false; specs.len()];
        Self {
            allocation_id: NEXT_ALLOCATION_ID.fetch_add(1, Ordering::Relaxed),
            specs,
            written,
            cpu_slabs: None,
        }
    }

    #[must_use]
    pub const fn allocation_id(&self) -> u64 {
        self.allocation_id
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.specs.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.specs.is_empty()
    }

    #[must_use]
    pub fn spec(&self, id: SlabId) -> Option<&SlabSpec> {
        self.specs.get(id.0)
    }

    #[must_use]
    pub fn is_written(&self, id: SlabId) -> bool {
        self.written.get(id.0).copied().unwrap_or(false)
    }

    /// Mark a device slab written after an upload or completed producer launch.
    pub fn mark_written(&mut self, id: SlabId) -> Result<(), Refusal> {
        if self.cpu_slabs.is_some() {
            return Err(refusal_802(
                format!("/slab/{}", id.0),
                "CPU slab completion is owned by BatchCpu",
                "completed BatchCpu upload or launch",
            ));
        }
        self.record_completion(id)
    }

    pub(crate) fn record_completion(&mut self, id: SlabId) -> Result<(), Refusal> {
        let slot = self.written.get_mut(id.0).ok_or_else(|| {
            refusal_802(
                format!("/slab/{}", id.0),
                format!("slab {} out of range (len {})", id.0, self.specs.len()),
                "allocated slab id",
            )
        })?;
        *slot = true;
        Ok(())
    }

    /// Read-only view of an aligned host slab when this set belongs to `batch-cpu`.
    #[must_use]
    pub(crate) fn cpu_slab(&self, id: SlabId) -> Option<&AlignedSlab> {
        self.cpu_slabs.as_deref()?.get(id.0)
    }

    pub(crate) fn cpu_slabs_mut(&mut self) -> Option<&mut [AlignedSlab]> {
        self.cpu_slabs.as_deref_mut()
    }
}

#[test]
fn host_slab_storage_is_aligned() {
    let slab = AlignedSlab::new(SlabSpec::new("x", ElemType::F32, 32));
    assert!((slab.as_bytes().as_ptr() as usize).is_multiple_of(SLAB_ALIGNMENT_BYTES));
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
    pub(crate) fn new(
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
        let off = ((lane - *start_lane) as usize) * 4;
        buf[off..off + 4].copy_from_slice(&val.to_le_bytes());
    }

    /// Write one `u32` element at global `lane` (within this worker's chunk) to output `name`.
    pub fn write_u32(&mut self, name: &str, lane: u32, val: u32) {
        let (buf, start_lane) = self.outputs.get_mut(name).expect("output arg bound");
        let off = ((lane - *start_lane) as usize) * 4;
        buf[off..off + 4].copy_from_slice(&val.to_le_bytes());
    }
}

/// Function pointer signature for a `batch-cpu` kernel symbol.
pub type CpuKernelFn = fn(&mut KernelArgs<'_>, Range<u32>);

/// Rust function symbol used by `batch-cpu`.
#[derive(Clone)]
pub struct CpuKernelSymbol {
    pub symbol_name: String,
    pub func: CpuKernelFn,
}

/// One manifest + multi-backend sources + optional KIR footprint evidence inside a [`KernelLibrary`].
///
/// Carrying optional `cpu_symbol` and `dialect_source` side by side allows one
/// [`KernelEntry`] (such as `K-selftest` or `K-cay-step`) to execute on
/// `batch-cpu`, CUDA (`BD-015`), and Metal (`BD-016`) through the same
/// [`crate::harness::Harness::run`] call by swapping only `&mut dyn KernelBackend`.
#[derive(Clone)]
pub struct KernelEntry {
    pub manifest: KernelManifest,
    pub cpu_symbol: Option<CpuKernelSymbol>,
    pub dialect_source: Option<String>,
    /// Required when `manifest.signature.footprint_evidence` is `FootprintEvidence::KirProgram`.
    pub program_footprint: Option<ProgramFootprint>,
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

/// Validated slab expectation sealed inside a [`StepPlan`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedSlab {
    pub slab: SlabId,
    pub elem: ElemType,
    pub byte_len: usize,
}

/// One validated, immutable launch inside a [`StepPlan`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedLaunch {
    stage_name: String,
    kernel_id: KernelId,
    lanes: u32,
    input_bindings: Vec<(String, SlabId)>,
    output_bindings: Vec<(String, SlabId)>,
}

impl PlannedLaunch {
    #[must_use]
    pub fn new(
        stage_name: String,
        kernel_id: KernelId,
        lanes: u32,
        input_bindings: Vec<(String, SlabId)>,
        output_bindings: Vec<(String, SlabId)>,
    ) -> Self {
        Self {
            stage_name,
            kernel_id,
            lanes,
            input_bindings,
            output_bindings,
        }
    }

    #[must_use]
    pub fn stage_name(&self) -> &str {
        &self.stage_name
    }

    #[must_use]
    pub const fn kernel_id(&self) -> KernelId {
        self.kernel_id
    }

    #[must_use]
    pub const fn lanes(&self) -> u32 {
        self.lanes
    }

    #[must_use]
    pub fn input_bindings(&self) -> &[(String, SlabId)] {
        &self.input_bindings
    }

    #[must_use]
    pub fn output_bindings(&self) -> &[(String, SlabId)] {
        &self.output_bindings
    }
}

/// Sealed, immutable list of validated launches bound to one backend allocation
/// and shape class.
///
/// Fields are private so callers cannot mutate validated bindings or bypass
/// `N21b-01` alias/type checks between [`KernelBackend::plan`] and
/// [`KernelBackend::launch`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepPlan {
    backend_tag: Option<u64>,
    allocation_id: u64,
    class: ShapeClassCaps,
    launches: Vec<PlannedLaunch>,
    expected_slabs: Vec<ExpectedSlab>,
}

impl StepPlan {
    #[must_use]
    pub(crate) fn new_cpu(
        backend_tag: u64,
        allocation_id: u64,
        class: ShapeClassCaps,
        launches: Vec<PlannedLaunch>,
        expected_slabs: Vec<ExpectedSlab>,
    ) -> Self {
        Self {
            backend_tag: Some(backend_tag),
            allocation_id,
            class,
            launches,
            expected_slabs,
        }
    }

    /// Construct a validated plan from a GPU annex backend (`BD-015`/`BD-016`).
    ///
    /// Never accepted by [`crate::cpu::BatchCpu`], which requires its own
    /// private backend tag.
    #[must_use]
    pub fn for_device(
        allocation_id: u64,
        class: ShapeClassCaps,
        launches: Vec<PlannedLaunch>,
        expected_slabs: Vec<ExpectedSlab>,
    ) -> Self {
        Self {
            backend_tag: None,
            allocation_id,
            class,
            launches,
            expected_slabs,
        }
    }

    #[must_use]
    pub(crate) const fn backend_tag(&self) -> Option<u64> {
        self.backend_tag
    }

    #[must_use]
    pub const fn allocation_id(&self) -> u64 {
        self.allocation_id
    }

    #[must_use]
    pub const fn class(&self) -> &ShapeClassCaps {
        &self.class
    }

    #[must_use]
    pub fn launches(&self) -> &[PlannedLaunch] {
        &self.launches
    }

    #[must_use]
    pub fn expected_slabs(&self) -> &[ExpectedSlab] {
        &self.expected_slabs
    }
}

/// Core seam implemented by `batch-cpu` (core), CUDA (annex), and Metal (annex) (`BD-014`).
pub trait KernelBackend {
    fn compile(&mut self, lib: &KernelLibrary) -> Result<CompiledLibrary, Refusal>;
    fn allocate(&mut self, layout: &BufferLayout) -> Result<BufferSet, Refusal>;
    fn upload(&mut self, bufs: &mut BufferSet, slab: SlabId, bytes: &[u8]) -> Result<(), Refusal>;
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
        "available device",
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

/// Construct a `FROB-REF-802` (invalid caller argument/contract error, exit 3)
/// refusal for slab index, buffer mismatch, unwritten-slab readback, or
/// re-entrant host calls (distinct from `601` device loss and `603` compiler diagnostics).
#[must_use]
pub fn refusal_802(
    pointer: impl Into<String>,
    message: impl Into<String>,
    required: impl Into<String>,
) -> Refusal {
    let msg = message.into();
    Refusal::new(
        RefusalCode::FrobRef802,
        msg.clone(),
        pointer,
        msg,
        required,
        "contract",
        RetryPolicy::AfterChange,
        vec![Repair::new(
            "Match BufferSet, StepPlan, slab indices, and completion order to the backend contract",
        )],
    )
}
