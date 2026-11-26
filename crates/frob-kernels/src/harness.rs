//! Single-kernel execution harness (`BD-014`).
//!
//! [`Harness::run`] drives any [`KernelBackend`] (`batch-cpu`, CUDA, Metal)
//! through one uniform compile → allocate → upload → plan → launch → read-back
//! → receipt-validation sequence.

use std::collections::BTreeMap;
use std::time::Instant;

use frob_core::id::{Blake3, hash_blob};
use frob_core::refusal::{Refusal, RefusalCode, Repair, RetryPolicy};

use crate::backend::{
    BufferLayout, Cx, KernelBackend, KernelEntry, KernelLibrary, SlabId, SlabSpec, Stage,
    StageBinding,
};
use crate::manifest::{ElemType, ShapeClassCaps};
use crate::receipt::Receipt;

/// One named host input slab uploaded by [`Harness::run`].
#[derive(Debug, Clone)]
pub struct HarnessInput {
    pub name: String,
    pub elem: ElemType,
    pub len: usize,
    pub bytes: Vec<u8>,
}

/// One named output slab allocated and read back by [`Harness::run`].
#[derive(Debug, Clone)]
pub struct HarnessOutputSpec {
    pub name: String,
    pub elem: ElemType,
    pub len: usize,
}

/// Read-back bytes, BLAKE3 digests, and validated [`Receipt`] from [`Harness::run`].
#[derive(Debug, Clone)]
pub struct HarnessOutput {
    pub slabs: BTreeMap<String, Vec<u8>>,
    pub digests: BTreeMap<String, Blake3>,
    pub receipt: Receipt,
}

/// Uniform single-kernel execution harness shared across CPU and GPU backends.
pub struct Harness;

impl Harness {
    /// Execute one kernel on `backend` with seeded `inputs`, validate the
    /// returned receipt (`INV-SPARK-02`), and return output bytes and digests.
    ///
    /// If cancellation drains before any launch executes (`receipt.launch_count == 0`),
    /// outputs are withheld (`slabs` and `digests` are empty) rather than reading
    /// or hashing unwritten memory, while the validated `cancelled-drained` receipt
    /// is preserved.
    pub fn run(
        backend: &mut dyn KernelBackend,
        entry: KernelEntry,
        inputs: &[HarnessInput],
        outputs: &[HarnessOutputSpec],
        class: &ShapeClassCaps,
        cx: &Cx,
    ) -> Result<HarnessOutput, Refusal> {
        let kernel_name = entry.manifest.name.clone();
        let lib = KernelLibrary {
            entries: vec![entry],
            shape_class: class.clone(),
            declared_outputs: Vec::new(),
        };
        let compiled = backend.compile(&lib)?;
        let kernel_id = compiled.id_for(&kernel_name).ok_or_else(|| {
            Refusal::new(
                RefusalCode::FrobRef990,
                "compiled library missing entry for submitted kernel",
                "/compile/kernel",
                &kernel_name,
                "compiled kernel ID",
                "kernel",
                RetryPolicy::Never,
                vec![Repair::new("frob doctor")],
            )
        })?;

        let mut specs = Vec::with_capacity(inputs.len() + outputs.len());
        let mut bindings = Vec::with_capacity(inputs.len() + outputs.len());
        let mut input_ids = Vec::with_capacity(inputs.len());
        let mut output_ids = Vec::with_capacity(outputs.len());

        for inp in inputs {
            let sid = SlabId(specs.len());
            specs.push(SlabSpec::new(&inp.name, inp.elem, inp.len));
            bindings.push(StageBinding {
                arg: inp.name.clone(),
                slab: sid,
            });
            input_ids.push((sid, inp.bytes.as_slice()));
        }
        for out in outputs {
            let sid = SlabId(specs.len());
            specs.push(SlabSpec::new(&out.name, out.elem, out.len));
            bindings.push(StageBinding {
                arg: out.name.clone(),
                slab: sid,
            });
            output_ids.push((out.name.clone(), sid, out.len * out.elem.byte_size()));
        }

        let mut bufs = backend.allocate(&BufferLayout { slabs: specs })?;
        for (sid, bytes) in input_ids {
            backend.upload(&mut bufs, sid, bytes)?;
        }

        let stage = Stage {
            name: kernel_name,
            kernel_id,
            bindings,
        };
        let plan = backend.plan(&[stage], class)?;
        let mut receipt = backend.launch(&plan, &mut bufs, cx)?;

        let mut slabs = BTreeMap::new();
        let mut digests = BTreeMap::new();
        if receipt.launch_count > 0 {
            let t_rb = Instant::now();
            for (name, sid, byte_len) in output_ids {
                let mut host = vec![0u8; byte_len];
                backend.read_back(&bufs, sid, &mut host)?;
                let digest = hash_blob(&host);
                digests.insert(name.clone(), digest);
                slabs.insert(name, host);
            }
            receipt.record_read_back_seconds(t_rb.elapsed().as_secs_f64());
        }

        receipt.validate().map_err(|e| {
            Refusal::new(
                RefusalCode::FrobRef990,
                e.to_string(),
                "/receipt",
                e.to_string(),
                "all twelve AE-01..AE-12 evidence fields present",
                "fields",
                RetryPolicy::Never,
                vec![Repair::new("frob doctor")],
            )
        })?;

        Ok(HarnessOutput {
            slabs,
            digests,
            receipt,
        })
    }
}
