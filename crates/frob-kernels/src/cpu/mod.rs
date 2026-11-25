//! `batch-cpu` [`KernelBackend`] implementation (`BD-014`).
//!
//! Executes validated Rust kernel symbols over 256-byte-aligned `Vec`-backed
//! slabs using deterministic fixed logical-index partitioning across worker
//! threads (determinism class `D2`, or `D0` for exact-integer kernels; zero
//! `unsafe`; zero `mul_add`; prints nothing per L-7).

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use frob_core::evidence::DetClass;
use frob_core::refusal::Refusal;

use crate::backend::{
    BackendIdentity, BufferLayout, BufferSet, CompiledLibrary, CpuKernelFn, Cx, KernelArgs,
    KernelBackend, KernelLibrary, KernelSource, PlannedLaunch, SlabId, SlabSpec, Stage, StepPlan,
    refusal_601, refusal_603,
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
    workers: usize,
    compiled: BTreeMap<KernelId, CompiledCpuKernel>,
    allocated_specs: Vec<SlabSpec>,
    last_compile_s: f64,
    last_allocate_s: f64,
    in_launch: bool,
    unavailable: bool,
    boundary_hook: Option<BoundaryHook>,
}

impl BatchCpu {
    /// Construct a `batch-cpu` backend with `workers` parallel worker lanes (`>= 1`).
    #[must_use]
    pub fn new(workers: usize) -> Self {
        Self {
            workers: workers.max(1),
            compiled: BTreeMap::new(),
            allocated_specs: Vec::new(),
            last_compile_s: 0.0,
            last_allocate_s: 0.0,
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

    fn execute_stage(
        &self,
        kernel: &CompiledCpuKernel,
        launch: &PlannedLaunch,
        bufs: &mut BufferSet,
    ) -> Result<(), Refusal> {
        let total = launch.lanes as usize;
        let output_ids: BTreeSet<SlabId> =
            launch.output_bindings.iter().map(|(_, id)| *id).collect();
        let input_ids: BTreeSet<SlabId> = launch.input_bindings.iter().map(|(_, id)| *id).collect();

        // Check bounds up front before borrowing slab storage.
        for (arg, id) in launch
            .input_bindings
            .iter()
            .chain(launch.output_bindings.iter())
        {
            let Some(slab) = bufs.get(*id) else {
                return Err(refusal_601(
                    format!("/slab/{}", id.0),
                    format!("slab {} for arg `{arg}` out of range", id.0),
                ));
            };
            let needed = total.saturating_mul(slab.spec.elem.byte_size());
            if slab.byte_len() < needed {
                return Err(refusal_603(
                    &kernel.manifest.name,
                    format!(
                        "slab `{}` has {} bytes, launch requires {} bytes",
                        slab.spec.name,
                        slab.byte_len(),
                        needed
                    ),
                ));
            }
        }

        // Snapshot any aliased read-write inputs so reads and writes never alias
        // mutable references (safe Rust, zero unsafe).
        let aliased_ids: BTreeSet<SlabId> = input_ids.intersection(&output_ids).copied().collect();
        let mut aliased_snapshots: BTreeMap<SlabId, Vec<u8>> = BTreeMap::new();
        for &id in &aliased_ids {
            if let Some(slab) = bufs.get(id) {
                aliased_snapshots.insert(id, slab.as_bytes().to_vec());
            }
        }

        // Partition BufferSet slabs into disjoint read-only inputs and mutable outputs.
        let mut raw_inputs: BTreeMap<SlabId, &[u8]> = BTreeMap::new();
        let mut raw_outputs: BTreeMap<SlabId, (&mut [u8], usize)> = BTreeMap::new();
        for (idx, slab) in bufs.slabs_mut().iter_mut().enumerate() {
            let sid = SlabId(idx);
            let elem_bytes = slab.spec.elem.byte_size();
            if output_ids.contains(&sid) {
                raw_outputs.insert(sid, (slab.as_bytes_mut(), elem_bytes));
            } else if input_ids.contains(&sid) {
                raw_inputs.insert(sid, slab.as_bytes());
            }
        }

        let mut input_map: BTreeMap<&str, &[u8]> = BTreeMap::new();
        for (arg, id) in &launch.input_bindings {
            let slice = if let Some(snap) = aliased_snapshots.get(id) {
                snap.as_slice()
            } else {
                raw_inputs[id]
            };
            input_map.insert(arg.as_str(), slice);
        }

        let eff_workers = self.workers.min(total.max(1));
        if eff_workers <= 1 || total == 0 {
            let mut out_map: BTreeMap<&str, (&mut [u8], u32)> = BTreeMap::new();
            for (arg, id) in &launch.output_bindings {
                let (slice, _) = raw_outputs.remove(id).ok_or_else(|| {
                    refusal_603(
                        &kernel.manifest.name,
                        format!("duplicate output binding on slab {}", id.0),
                    )
                })?;
                out_map.insert(arg.as_str(), (slice, 0));
            }
            let mut args = KernelArgs::new(input_map, out_map);
            (kernel.func)(&mut args, 0..launch.lanes);
            return Ok(());
        }

        // Deterministic fixed logical-index partition across `eff_workers` threads.
        let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(eff_workers);
        for w in 0..eff_workers {
            let start = (w * total) / eff_workers;
            let end = ((w + 1) * total) / eff_workers;
            ranges.push((start, end));
        }

        let mut per_worker_outputs: Vec<BTreeMap<&str, (&mut [u8], u32)>> =
            (0..eff_workers).map(|_| BTreeMap::new()).collect();
        for (arg, id) in &launch.output_bindings {
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
        Ok(())
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
                .validate_static(std::slice::from_ref(&lib.shape_class), None, false)
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
            let (symbol_name, func) = match &entry.source {
                KernelSource::CpuSymbol { symbol_name, func } => (symbol_name.clone(), *func),
                KernelSource::DialectText(_) => {
                    return Err(refusal_603(
                        &entry.manifest.name,
                        "batch-cpu requires KernelSource::CpuSymbol, not DialectText",
                    ));
                }
            };
            let kid = entry.manifest.id;
            ids.push((entry.manifest.name.clone(), kid));
            staged.push((
                kid,
                CompiledCpuKernel {
                    manifest: entry.manifest.clone(),
                    symbol_name,
                    func,
                },
            ));
        }

        // Commit atomically only after every manifest and binding passes.
        for (kid, ck) in staged {
            self.compiled.insert(kid, ck);
        }
        self.last_compile_s = t0.elapsed().as_secs_f64();
        Ok(CompiledLibrary { kernel_ids: ids })
    }

    fn allocate(&mut self, layout: &BufferLayout) -> Result<BufferSet, Refusal> {
        self.check_available()?;
        let t0 = Instant::now();
        let bufs = BufferSet::allocate_aligned(layout);
        self.allocated_specs = layout.slabs.clone();
        self.last_allocate_s = t0.elapsed().as_secs_f64();
        Ok(bufs)
    }

    fn plan(&mut self, stages: &[Stage], class: &ShapeClassCaps) -> Result<StepPlan, Refusal> {
        self.check_available()?;
        let mut launches = Vec::with_capacity(stages.len());

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

            for sb in &stage.bindings {
                let spec = self.allocated_specs.get(sb.slab.0).ok_or_else(|| {
                    refusal_603(
                        &ck.manifest.name,
                        format!(
                            "stage `{}` binds arg `{}` to unallocated slab {}",
                            stage.name, sb.arg, sb.slab.0
                        ),
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
                    input_bindings.push((sb.arg.clone(), sb.slab));
                }
                if write_args.contains(sb.arg.as_str()) {
                    output_bindings.push((sb.arg.clone(), sb.slab));
                    declared_outputs.push(DeclaredOutput {
                        arg: sb.arg.clone(),
                        slab_id: slab_label,
                        lo: 0,
                        hi: spec.len as u64,
                    });
                }
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
            launches.push(PlannedLaunch {
                stage_name: stage.name.clone(),
                kernel_id: stage.kernel_id,
                lanes,
                input_bindings,
                output_bindings,
            });
        }

        Ok(StepPlan {
            launches,
            class: class.clone(),
        })
    }

    fn launch(
        &mut self,
        plan: &StepPlan,
        bufs: &mut BufferSet,
        cx: &Cx,
    ) -> Result<Receipt, Refusal> {
        self.check_available()?;
        if self.in_launch {
            return Err(refusal_601(
                "/launch",
                "re-entrant launch while previous launch is active",
            ));
        }
        self.in_launch = true;
        let t0 = Instant::now();
        let mut outcome = CancelOutcome::Completed;
        let mut executed: u32 = 0;
        let mut kernel_ids = Vec::new();
        let mut kernel_names = Vec::new();
        let mut symbols = Vec::new();
        let mut all_exact = true;

        let mut launch_err: Option<Refusal> = None;
        for (idx, step) in plan.launches.iter().enumerate() {
            if cx.is_cancelled() {
                outcome = CancelOutcome::CancelledDrained;
                break;
            }
            let Some(ck) = self.compiled.get(&step.kernel_id) else {
                launch_err = Some(refusal_603(
                    &step.stage_name,
                    format!("missing compiled kernel {}", step.kernel_id),
                ));
                break;
            };
            kernel_ids.push(step.kernel_id.to_string());
            kernel_names.push(ck.manifest.name.clone());
            symbols.push(ck.symbol_name.clone());
            if ck.manifest.class != KernelClass::Exact {
                all_exact = false;
            }

            if let Err(e) = self.execute_stage(ck, step, bufs) {
                launch_err = Some(e);
                break;
            }
            executed += 1;

            if let Some(hook) = self.boundary_hook.as_mut() {
                hook(idx, cx);
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
            launch_s,
            read_back_s: 0.0,
        };
        let wall_seconds = phase_times.compile_s + phase_times.allocate_s + phase_times.launch_s;
        let id = self.identity();
        let det = if all_exact && executed > 0 {
            DetClass::D0Exact
        } else {
            DetClass::D2PerBackend
        };

        let ae04 = if kernel_ids.is_empty() {
            "none (cancelled before launch)".to_owned()
        } else {
            kernel_ids.join(",")
        };
        let ae05 = format!(
            "kernels=[{}] shape={} worlds={}",
            kernel_names.join(","),
            plan.class.name,
            plan.class.worlds
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
                "arch={} workers={} vector_width=1",
                std::env::consts::ARCH,
                self.workers
            )),
            ae07_reduction_and_determinism_policy: Some(format!(
                "fixed-order trees, no atomics, class {}",
                det.as_str()
            )),
            ae08_cpu_versus_device_numerical_comparison: Some("self".to_owned()),
            ae09_permanent_cpu_reference: Some(ae09),
            ae10_cancellation_drain_and_finalize_outcome: Some(outcome.as_str().to_owned()),
            ae11_phase_wall_time_and_energy_attribution: Some(phase_times.format_ae11()),
            ae12_go_or_no_go_decision: Some("not-assessed".to_owned()),
            placements: Vec::new(),
            fingerprint_digest: None,
            determinism_claimed: det,
            determinism_attained: det,
            shape_class: plan.class.clone(),
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
            return Err(refusal_601(
                format!("/slab/{}", slab.0),
                "read_back called while launch is active",
            ));
        }
        let s = bufs.get(slab).ok_or_else(|| {
            refusal_601(
                format!("/slab/{}", slab.0),
                format!("slab {} out of range", slab.0),
            )
        })?;
        if out.len() != s.byte_len() {
            return Err(refusal_603(
                &s.spec.name,
                format!(
                    "read_back buffer length {} != slab byte length {}",
                    out.len(),
                    s.byte_len()
                ),
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
            compiler: "rustc-batch-cpu-no-fma".to_owned(),
        }
    }
}
