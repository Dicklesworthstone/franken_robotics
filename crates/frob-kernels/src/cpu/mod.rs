//! `batch-cpu` [`KernelBackend`] implementation (`BD-014`).
//!
//! Executes validated Rust kernel symbols over 256-byte-aligned `Vec`-backed
//! slabs using deterministic fixed logical-index partitioning across worker
//! threads (zero `unsafe`; zero `mul_add`; prints nothing per L-7).

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use frob_core::evidence::DetClass;
use frob_core::refusal::Refusal;

use crate::backend::{
    BackendIdentity, BufferLayout, BufferSet, CompiledLibrary, CpuKernelFn, Cx, ExpectedSlab,
    KernelArgs, KernelBackend, KernelLibrary, PlannedLaunch, SlabId, SlabSpec, Stage, StepPlan,
    next_backend_tag, refusal_601, refusal_603, refusal_802,
};
use crate::manifest::{
    BackendLimits, DeclaredOutput, KernelClass, KernelId, KernelManifest, ShapeClassCaps,
    SlabBinding,
};
use crate::receipt::{CancelOutcome, PhaseTimes, Receipt};

type BoundaryHook = Box<dyn FnMut(usize, &Cx) + Send>;

struct CompiledCpuKernel {
    manifest: KernelManifest,
    symbol_name: String,
    func: CpuKernelFn,
}

/// Deterministic CPU backend implementing [`KernelBackend`] (`BD-014`).
pub struct BatchCpu {
    backend_tag: u64,
    workers: usize,
    compiled: BTreeMap<KernelId, CompiledCpuKernel>,
    last_allocation_id: Option<u64>,
    allocated_specs: Vec<SlabSpec>,
    last_compile_s: f64,
    last_allocate_s: f64,
    last_upload_s: f64,
    in_launch: bool,
    unavailable: bool,
    boundary_hook: Option<BoundaryHook>,
}

impl BatchCpu {
    /// Construct a `batch-cpu` backend with `workers` parallel worker lanes (`>= 1`).
    #[must_use]
    pub fn new(workers: usize) -> Self {
        Self {
            backend_tag: next_backend_tag(),
            workers: workers.max(1),
            compiled: BTreeMap::new(),
            last_allocation_id: None,
            allocated_specs: Vec::new(),
            last_compile_s: 0.0,
            last_allocate_s: 0.0,
            last_upload_s: 0.0,
            in_launch: false,
            unavailable: false,
            boundary_hook: None,
        }
    }

    /// Configured worker count.
    #[must_use]
    pub const fn workers(&self) -> usize {
        self.workers
    }

    /// Fault-injection helper for tests: make this backend refuse with `FROB-REF-601`.
    #[must_use]
    pub const fn with_unavailable(mut self, unavailable: bool) -> Self {
        self.unavailable = unavailable;
        self
    }

    /// Attach a launch-boundary callback invoked after launch `idx` finishes
    /// (between launches, never inside a kernel). Used to test deterministic
    /// cancellation and drain (`AE-10 = cancelled-drained`).
    pub fn set_boundary_hook(&mut self, hook: impl FnMut(usize, &Cx) + Send + 'static) {
        self.boundary_hook = Some(Box::new(hook));
    }

    fn check_available(&self) -> Result<(), Refusal> {
        if self.unavailable {
            return Err(refusal_601(
                "/device/batch-cpu",
                "batch-cpu backend marked unavailable",
            ));
        }
        Ok(())
    }

    fn validate_launch_buffers(&self, plan: &StepPlan, bufs: &BufferSet) -> Result<(), Refusal> {
        if plan.backend_tag() != Some(self.backend_tag) {
            return Err(refusal_802(
                "/launch/plan",
                "step plan was not created by this BatchCpu instance",
                "StepPlan produced by this BatchCpu::plan",
            ));
        }
        if plan.allocation_id() != bufs.allocation_id() {
            return Err(refusal_802(
                "/launch/buffers",
                format!(
                    "BufferSet allocation {} does not match StepPlan allocation {}",
                    bufs.allocation_id(),
                    plan.allocation_id()
                ),
                "BufferSet returned by the matching BatchCpu::allocate",
            ));
        }
        for exp in plan.expected_slabs() {
            let Some(slab) = bufs.cpu_slab(exp.slab) else {
                return Err(refusal_802(
                    format!("/slab/{}", exp.slab.0),
                    format!("slab {} missing CPU backing in BufferSet", exp.slab.0),
                    "CPU-backed BufferSet matching StepPlan",
                ));
            };
            if slab.spec().elem != exp.elem || slab.byte_len() != exp.byte_len {
                return Err(refusal_802(
                    format!("/slab/{}", exp.slab.0),
                    format!(
                        "slab `{}` descriptor mismatch: got ({:?}, {} B), plan expects ({:?}, {} B)",
                        slab.spec().name,
                        slab.spec().elem,
                        slab.byte_len(),
                        exp.elem,
                        exp.byte_len
                    ),
                    "exact ElemType and byte length validated by plan",
                ));
            }
        }
        Ok(())
    }

    fn execute_stage(
        &self,
        kernel: &CompiledCpuKernel,
        launch: &PlannedLaunch,
        bufs: &mut BufferSet,
    ) -> Result<usize, Refusal> {
        let total = launch.lanes() as usize;
        let output_ids: BTreeSet<SlabId> =
            launch.output_bindings().iter().map(|(_, id)| *id).collect();
        let input_ids: BTreeSet<SlabId> =
            launch.input_bindings().iter().map(|(_, id)| *id).collect();

        if !input_ids.is_disjoint(&output_ids) {
            return Err(refusal_603(
                &kernel.manifest.name,
                "batch-cpu requires disjoint input and output slabs per stage",
            ));
        }

        let slabs = bufs.cpu_slabs_mut().ok_or_else(|| {
            refusal_802(
                "/launch/buffers",
                "BufferSet lacks CPU backing slabs",
                "BufferSet from BatchCpu::allocate",
            )
        })?;

        let mut raw_inputs: BTreeMap<SlabId, &[u8]> = BTreeMap::new();
        let mut raw_outputs: BTreeMap<SlabId, (&mut [u8], usize)> = BTreeMap::new();
        for (idx, slab) in slabs.iter_mut().enumerate() {
            let sid = SlabId(idx);
            let elem_bytes = slab.spec().elem.byte_size();
            if output_ids.contains(&sid) {
                raw_outputs.insert(sid, (slab.as_bytes_mut(), elem_bytes));
            } else if input_ids.contains(&sid) {
                raw_inputs.insert(sid, slab.as_bytes());
            }
        }

        let mut input_map: BTreeMap<&str, &[u8]> = BTreeMap::new();
        for (arg, id) in launch.input_bindings() {
            input_map.insert(arg.as_str(), raw_inputs[id]);
        }

        let eff_workers = if total == 0 {
            1
        } else {
            self.workers.min(total)
        };
        let ranges: Vec<(usize, usize)> = (0..eff_workers)
            .map(|w| ((w * total) / eff_workers, ((w + 1) * total) / eff_workers))
            .collect();

        let mut per_worker_outputs: Vec<BTreeMap<&str, (&mut [u8], u32)>> =
            (0..eff_workers).map(|_| BTreeMap::new()).collect();
        for (arg, id) in launch.output_bindings() {
            let (mut rest, elem_bytes) = raw_outputs.remove(id).ok_or_else(|| {
                refusal_603(
                    &kernel.manifest.name,
                    format!("duplicate output binding on slab {}", id.0),
                )
            })?;
            for (w, &(start, end)) in ranges.iter().enumerate() {
                let chunk_bytes = (end - start) * elem_bytes;
                let (head, tail) = rest.split_at_mut(chunk_bytes);
                per_worker_outputs[w].insert(arg.as_str(), (head, start as u32));
                rest = tail;
            }
        }

        let func = kernel.func;
        if eff_workers == 1 {
            let out_map = per_worker_outputs.into_iter().next().unwrap_or_default();
            let mut args = KernelArgs::new(input_map, out_map);
            func(&mut args, 0..launch.lanes());
        } else {
            let inputs_ref = &input_map;
            std::thread::scope(|scope| {
                for ((start, end), out_map) in ranges.into_iter().zip(per_worker_outputs) {
                    let inputs_clone = inputs_ref.clone();
                    scope.spawn(move || {
                        let mut args = KernelArgs::new(inputs_clone, out_map);
                        func(&mut args, (start as u32)..(end as u32));
                    });
                }
            });
        }

        for &sid in &output_ids {
            bufs.mark_written(sid)?;
        }
        Ok(eff_workers)
    }
}

impl KernelBackend for BatchCpu {
    fn compile(&mut self, lib: &KernelLibrary) -> Result<CompiledLibrary, Refusal> {
        self.check_available()?;
        let t0 = Instant::now();
        let mut staged: Vec<(KernelId, CompiledCpuKernel)> = Vec::with_capacity(lib.entries.len());
        let mut ids: Vec<(String, KernelId)> = Vec::with_capacity(lib.entries.len());

        for entry in &lib.entries {
            entry
                .manifest
                .validate_static(
                    std::slice::from_ref(&lib.shape_class),
                    entry.program_footprint.as_ref(),
                    false,
                )
                .map_err(|e| refusal_603(&entry.manifest.name, e.to_string()))?;
            if !entry.compile_bindings.is_empty() {
                entry
                    .manifest
                    .validate_binding(
                        &entry.compile_bindings,
                        &lib.shape_class,
                        BackendLimits::batch_cpu(),
                        &lib.declared_outputs,
                        &[],
                    )
                    .map_err(|e| refusal_603(&entry.manifest.name, e.to_string()))?;
            }
            let Some(sym) = &entry.cpu_symbol else {
                return Err(refusal_603(
                    &entry.manifest.name,
                    "batch-cpu requires KernelEntry.cpu_symbol",
                ));
            };
            let kid = entry.manifest.id;
            ids.push((entry.manifest.name.clone(), kid));
            staged.push((
                kid,
                CompiledCpuKernel {
                    manifest: entry.manifest.clone(),
                    symbol_name: sym.symbol_name.clone(),
                    func: sym.func,
                },
            ));
        }

        for (kid, ck) in staged {
            self.compiled.insert(kid, ck);
        }
        self.last_compile_s = t0.elapsed().as_secs_f64();
        Ok(CompiledLibrary { kernel_ids: ids })
    }

    fn allocate(&mut self, layout: &BufferLayout) -> Result<BufferSet, Refusal> {
        self.check_available()?;
        let t0 = Instant::now();
        let bufs = BufferSet::new_cpu(layout);
        self.last_allocation_id = Some(bufs.allocation_id());
        self.allocated_specs = layout.slabs.clone();
        self.last_allocate_s = t0.elapsed().as_secs_f64();
        self.last_upload_s = 0.0;
        Ok(bufs)
    }

    fn upload(&mut self, bufs: &mut BufferSet, slab: SlabId, bytes: &[u8]) -> Result<(), Refusal> {
        self.check_available()?;
        if self.in_launch {
            return Err(refusal_802(
                format!("/slab/{}", slab.0),
                "upload called while launch is active",
                "upload only between launches",
            ));
        }
        let t0 = Instant::now();
        let slabs = bufs.cpu_slabs_mut().ok_or_else(|| {
            refusal_802(
                "/upload/buffers",
                "BufferSet lacks CPU backing slabs",
                "BufferSet from BatchCpu::allocate",
            )
        })?;
        let target = slabs.get_mut(slab.0).ok_or_else(|| {
            refusal_802(
                format!("/slab/{}", slab.0),
                format!("slab {} out of range", slab.0),
                "allocated slab id",
            )
        })?;
        if bytes.len() != target.byte_len() {
            return Err(refusal_802(
                format!("/slab/{}", target.spec().name),
                format!(
                    "upload size mismatch: got {} bytes, slab `{}` requires {}",
                    bytes.len(),
                    target.spec().name,
                    target.byte_len()
                ),
                "exact slab byte length",
            ));
        }
        target.as_bytes_mut().copy_from_slice(bytes);
        bufs.mark_written(slab)?;
        self.last_upload_s += t0.elapsed().as_secs_f64();
        Ok(())
    }

    fn plan(&mut self, stages: &[Stage], class: &ShapeClassCaps) -> Result<StepPlan, Refusal> {
        self.check_available()?;
        let alloc_id = self.last_allocation_id.ok_or_else(|| {
            refusal_802(
                "/plan",
                "plan called before allocate",
                "call allocate before plan",
            )
        })?;

        let mut launches = Vec::with_capacity(stages.len());
        let mut expected_map: BTreeMap<SlabId, ExpectedSlab> = BTreeMap::new();

        for stage in stages {
            let ck = self.compiled.get(&stage.kernel_id).ok_or_else(|| {
                refusal_603(
                    &stage.name,
                    format!("uncompiled kernel id {}", stage.kernel_id),
                )
            })?;

            let mut slab_bindings = Vec::with_capacity(stage.bindings.len());
            let mut declared_outputs = Vec::new();
            let write_args: BTreeSet<&str> = ck
                .manifest
                .signature
                .writes
                .iter()
                .map(|f| f.arg.as_str())
                .collect();
            let read_args: BTreeSet<&str> = ck
                .manifest
                .signature
                .reads
                .iter()
                .map(|f| f.arg.as_str())
                .collect();

            let mut input_bindings = Vec::new();
            let mut output_bindings = Vec::new();
            let mut seen_out_slabs = BTreeSet::new();
            let mut seen_in_slabs = BTreeSet::new();

            for sb in &stage.bindings {
                let spec = self.allocated_specs.get(sb.slab.0).ok_or_else(|| {
                    refusal_802(
                        format!("/stage/{}/arg/{}", stage.name, sb.arg),
                        format!(
                            "stage `{}` binds arg `{}` to unallocated slab {}",
                            stage.name, sb.arg, sb.slab.0
                        ),
                        "allocated slab id",
                    )
                })?;
                let lay = ck
                    .manifest
                    .signature
                    .layout
                    .iter()
                    .find(|l| l.arg == sb.arg)
                    .ok_or_else(|| {
                        refusal_603(
                            &ck.manifest.name,
                            format!(
                                "stage `{}` binds undeclared arg `{}` on kernel `{}`",
                                stage.name, sb.arg, ck.manifest.name
                            ),
                        )
                    })?;
                let req_elems = lay.alloc_extent.eval(class).ok_or_else(|| {
                    refusal_603(
                        &ck.manifest.name,
                        format!(
                            "arg `{}` alloc_extent overflowed for class `{}`",
                            sb.arg, class.name
                        ),
                    )
                })?;
                if (spec.len as u64) < req_elems {
                    return Err(refusal_603(
                        &ck.manifest.name,
                        format!(
                            "slab `{}` for arg `{}` has len {} < required alloc_extent {}",
                            spec.name, sb.arg, spec.len, req_elems
                        ),
                    ));
                }
                let slab_label = format!("slab-{}", sb.slab.0);
                slab_bindings.push(SlabBinding {
                    arg: sb.arg.clone(),
                    slab_id: slab_label.clone(),
                    elem: spec.elem,
                    extents: lay.extents.clone(),
                    strides: lay.strides.clone(),
                    stride_unit: lay.stride_unit,
                    stride_signed: lay.stride_signed,
                    origin: lay.origin,
                    alloc_extent: lay.alloc_extent,
                    slab_offset: 0,
                    from_stat_kernel: spec.from_stat_kernel,
                });
                if read_args.contains(sb.arg.as_str()) {
                    seen_in_slabs.insert(sb.slab);
                    input_bindings.push((sb.arg.clone(), sb.slab));
                }
                if write_args.contains(sb.arg.as_str()) {
                    if !seen_out_slabs.insert(sb.slab) {
                        return Err(refusal_603(
                            &ck.manifest.name,
                            format!(
                                "stage `{}` binds multiple outputs to slab {}",
                                stage.name, sb.slab.0
                            ),
                        ));
                    }
                    output_bindings.push((sb.arg.clone(), sb.slab));
                    declared_outputs.push(DeclaredOutput {
                        arg: sb.arg.clone(),
                        slab_id: slab_label,
                        lo: 0,
                        hi: spec.len as u64,
                    });
                }
                expected_map.entry(sb.slab).or_insert(ExpectedSlab {
                    slab: sb.slab,
                    elem: spec.elem,
                    byte_len: spec.byte_len(),
                });
            }

            if !seen_in_slabs.is_disjoint(&seen_out_slabs) {
                return Err(refusal_603(
                    &ck.manifest.name,
                    format!(
                        "stage `{}` aliases an input and output slab (disjoint slabs required on batch-cpu)",
                        stage.name
                    ),
                ));
            }

            ck.manifest
                .validate_binding(
                    &slab_bindings,
                    class,
                    BackendLimits::batch_cpu(),
                    &declared_outputs,
                    &[],
                )
                .map_err(|e| refusal_603(&ck.manifest.name, e.to_string()))?;

            let lanes = u32::try_from(class.worlds).map_err(|_| {
                refusal_603(
                    &ck.manifest.name,
                    format!("world count {} exceeds u32::MAX", class.worlds),
                )
            })?;
            launches.push(PlannedLaunch::new(
                stage.name.clone(),
                stage.kernel_id,
                lanes,
                input_bindings,
                output_bindings,
            ));
        }

        Ok(StepPlan::new_cpu(
            self.backend_tag,
            alloc_id,
            class.clone(),
            launches,
            expected_map.into_values().collect(),
        ))
    }

    fn launch(
        &mut self,
        plan: &StepPlan,
        bufs: &mut BufferSet,
        cx: &Cx,
    ) -> Result<Receipt, Refusal> {
        self.check_available()?;
        if self.in_launch {
            return Err(refusal_802(
                "/launch",
                "re-entrant launch while previous launch is active",
                "sequential non-re-entrant launch",
            ));
        }
        self.validate_launch_buffers(plan, bufs)?;

        self.in_launch = true;
        let t0 = Instant::now();
        let mut outcome = CancelOutcome::Completed;
        let mut executed: u32 = 0;
        let mut max_workers_used: usize = 0;
        let mut kernel_ids = Vec::new();
        let mut kernel_names = Vec::new();
        let mut symbols = Vec::new();
        let mut det_claimed = DetClass::D0Exact;
        let mut det_attained = DetClass::D0Exact;

        let mut launch_err: Option<Refusal> = None;
        for (idx, step) in plan.launches().iter().enumerate() {
            if cx.is_cancelled() {
                outcome = CancelOutcome::CancelledDrained;
                break;
            }
            let Some(ck) = self.compiled.get(&step.kernel_id()) else {
                launch_err = Some(refusal_603(
                    step.stage_name(),
                    format!("missing compiled kernel {}", step.kernel_id()),
                ));
                break;
            };
            let used = match self.execute_stage(ck, step, bufs) {
                Ok(w) => w,
                Err(e) => {
                    launch_err = Some(e);
                    break;
                }
            };
            max_workers_used = max_workers_used.max(used);
            kernel_ids.push(step.kernel_id().to_string());
            kernel_names.push(ck.manifest.name.clone());
            symbols.push(ck.symbol_name.clone());
            det_claimed = det_claimed.meet(ck.manifest.determinism);
            let backend_cap = if ck.manifest.class == KernelClass::Exact {
                DetClass::D0Exact
            } else {
                DetClass::D2PerBackend
            };
            det_attained = det_attained.meet(ck.manifest.determinism.meet(backend_cap));
            executed += 1;

            if let Some(hook) = self.boundary_hook.as_mut() {
                hook(idx, cx);
            }
            if cx.is_cancelled() {
                outcome = CancelOutcome::CancelledDrained;
                break;
            }
        }
        self.in_launch = false;

        if let Some(err) = launch_err {
            return Err(err);
        }

        let launch_s = t0.elapsed().as_secs_f64();
        let phase_times = PhaseTimes {
            compile_s: self.last_compile_s,
            allocate_s: self.last_allocate_s,
            upload_s: self.last_upload_s,
            launch_s,
            read_back_s: 0.0,
        };
        let wall_seconds = phase_times.total_s();
        let id = self.identity();

        let ae04 = if kernel_ids.is_empty() {
            "none (cancelled before launch)".to_owned()
        } else {
            kernel_ids.join(",")
        };
        let ae05 = format!(
            "kernels=[{}] shape={} worlds={}",
            kernel_names.join(","),
            plan.class().name,
            plan.class().worlds
        );
        let ae09 = if symbols.is_empty() {
            "none (cancelled before launch)".to_owned()
        } else {
            symbols.join(",")
        };

        Ok(Receipt {
            ae01_device_identity: Some(id.device),
            ae02_driver_and_runtime_version: Some(format!("{} / {}", id.driver, id.runtime)),
            ae03_compiler_and_backend_identity: Some(id.compiler),
            ae04_kernel_source_identity: Some(ae04),
            ae05_kernel_and_workload_identity: Some(ae05),
            ae06_cpu_machine_identity_and_measured_axes: Some(format!(
                "arch={} os={} workers_used={} workers_configured={} vector_width=1",
                std::env::consts::ARCH,
                std::env::consts::OS,
                max_workers_used,
                self.workers
            )),
            ae07_reduction_and_determinism_policy: Some(format!(
                "logical-index disjoint lane partition, no atomics, claimed={} attained={}",
                det_claimed.as_str(),
                det_attained.as_str()
            )),
            ae08_cpu_versus_device_numerical_comparison: Some("self".to_owned()),
            ae09_permanent_cpu_reference: Some(ae09),
            ae10_cancellation_drain_and_finalize_outcome: Some(outcome.as_str().to_owned()),
            ae11_phase_wall_time_and_energy_attribution: Some(phase_times.format_ae11()),
            ae12_go_or_no_go_decision: Some("not-assessed".to_owned()),
            placements: Vec::new(),
            fingerprint_digest: None,
            determinism_claimed: det_claimed,
            determinism_attained: det_attained,
            shape_class: plan.class().clone(),
            launch_count: executed,
            retries: 0,
            device_seconds: launch_s,
            wall_seconds,
            phase_times,
            citable: false,
        })
    }

    fn read_back(&mut self, bufs: &BufferSet, slab: SlabId, out: &mut [u8]) -> Result<(), Refusal> {
        self.check_available()?;
        if self.in_launch {
            return Err(refusal_802(
                format!("/slab/{}", slab.0),
                "read_back called while launch is active",
                "read_back only after launch completion",
            ));
        }
        let s = bufs.cpu_slab(slab).ok_or_else(|| {
            refusal_802(
                format!("/slab/{}", slab.0),
                format!("slab {} out of range or not CPU-backed", slab.0),
                "allocated CPU slab id",
            )
        })?;
        if !bufs.is_written(slab) {
            return Err(refusal_802(
                format!("/slab/{}", s.spec().name),
                format!(
                    "read_back refused for slab `{}` before its producer completed",
                    s.spec().name
                ),
                "completed upload or producer launch before read_back",
            ));
        }
        if out.len() != s.byte_len() {
            return Err(refusal_802(
                format!("/slab/{}", s.spec().name),
                format!(
                    "read_back buffer length {} != slab byte length {}",
                    out.len(),
                    s.byte_len()
                ),
                "exact slab byte length",
            ));
        }
        out.copy_from_slice(s.as_bytes());
        Ok(())
    }

    fn identity(&self) -> BackendIdentity {
        BackendIdentity {
            family: "batch-cpu".to_owned(),
            device: format!("cpu-{}-{}", std::env::consts::ARCH, std::env::consts::OS),
            driver: format!("os-{}", std::env::consts::OS),
            runtime: format!("std-thread-scope-w{}", self.workers),
            compiler: format!("batch-cpu (frob-kernels v{})", env!("CARGO_PKG_VERSION")),
        }
    }
}
