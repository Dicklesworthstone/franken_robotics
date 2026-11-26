#![deny(unsafe_code)]

//! Unit and property tests for `KernelBackend`, `Receipt` (`AE-01`..`AE-12`),
//! `BatchCpu`, `Harness`, and `K-selftest` (`BD-014`).

use std::ops::Range;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};

use frob_core::evidence::DetClass;
use frob_core::id::{Blake3, hash_blob};
use frob_core::refusal::{Refusal, RefusalCode};
use frob_core::registry::toml::{self, Fields};
use frob_kernels::backend::{
    BufferLayout, BufferSet, CpuKernelSymbol, Cx, KernelArgs, KernelBackend, KernelEntry,
    KernelLibrary, SlabId, SlabSpec, Stage, StageBinding,
};
use frob_kernels::cpu::BatchCpu;
use frob_kernels::dialect::{BackendTarget, assemble_and_lint};
use frob_kernels::harness::{Harness, HarnessOutput};
use frob_kernels::manifest::{
    CapExpr, DeclaredOutput, ElemType, FootprintEvidence, ProgramFootprint, SlabBinding, StrideUnit,
};
use frob_kernels::receipt::{DOCTRINE_FIELDS, Receipt, ReceiptError};
use frob_kernels::selftest::{
    K_SELFTEST_DIALECT_SOURCE, SelftestInputs, k_selftest_entry, k_selftest_manifest,
    k_selftest_output_specs, k_selftest_scalar_reference, k_selftest_seeded_inputs,
    k_selftest_shape_class,
};
use frob_kir::dialect_lint::SourceFile;

const DOCTRINE_FIXTURE: &str = include_str!("fixtures/accelerator_doctrine_fields.toml");

fn run_selftest(workers: usize, inputs: &SelftestInputs) -> HarnessOutput {
    let total = inputs.total_lanes();
    let mut cpu = BatchCpu::new(workers);
    Harness::run(
        &mut cpu,
        k_selftest_entry(),
        &inputs.harness_inputs(),
        &k_selftest_output_specs(total),
        &k_selftest_shape_class(total),
        &Cx::new(),
    )
    .expect("K-selftest harness run succeeds")
}

fn selftest_layout(lanes: usize) -> BufferLayout {
    BufferLayout {
        slabs: vec![
            SlabSpec::new("a", ElemType::F32, lanes),
            SlabSpec::new("b", ElemType::F32, lanes),
            SlabSpec::new("x", ElemType::U32, lanes),
            SlabSpec::new("y", ElemType::U32, lanes),
            SlabSpec::new("out_f", ElemType::F32, lanes),
            SlabSpec::new("out_u", ElemType::U32, lanes),
        ],
    }
}

fn selftest_bindings() -> Vec<StageBinding> {
    ["a", "b", "x", "y", "out_f", "out_u"]
        .into_iter()
        .enumerate()
        .map(|(idx, arg)| StageBinding {
            arg: arg.into(),
            slab: SlabId(idx),
        })
        .collect()
}

fn clear_ae_field(receipt: &mut Receipt, index_1_based: u8) {
    match index_1_based {
        1 => receipt.ae01_device_identity = None,
        2 => receipt.ae02_driver_and_runtime_version = None,
        3 => receipt.ae03_compiler_and_backend_identity = None,
        4 => receipt.ae04_kernel_source_identity = None,
        5 => receipt.ae05_kernel_and_workload_identity = None,
        6 => receipt.ae06_cpu_machine_identity_and_measured_axes = None,
        7 => receipt.ae07_reduction_and_determinism_policy = None,
        8 => receipt.ae08_cpu_versus_device_numerical_comparison = None,
        9 => receipt.ae09_permanent_cpu_reference = None,
        10 => receipt.ae10_cancellation_drain_and_finalize_outcome = None,
        11 => receipt.ae11_phase_wall_time_and_energy_attribution = None,
        12 => receipt.ae12_go_or_no_go_decision = None,
        _ => unreachable!("AE field index must be in 1..=12"),
    }
}

#[test]
fn selftest_matches_scalar_loop() {
    let inputs = k_selftest_seeded_inputs(4_096, 16, 1);
    let expected = k_selftest_scalar_reference(&inputs);
    let res = run_selftest(4, &inputs);

    assert_eq!(res.slabs["out_f"], expected.out_f_bytes());
    assert_eq!(res.slabs["out_u"], expected.out_u_bytes());
    res.receipt.validate().expect("receipt valid");
    assert!(
        !res.receipt.citable,
        "M0 batch-cpu receipt without fingerprint is citable=false"
    );
    assert_eq!(res.receipt.determinism_claimed, DetClass::D2PerBackend);
    assert_eq!(res.receipt.determinism_attained, DetClass::D2PerBackend);
    assert!(
        res.receipt
            .ae06_cpu_machine_identity_and_measured_axes
            .as_deref()
            .is_some_and(|s| s.contains("workers_used=4") && s.contains("workers_configured=4"))
    );

    // Padding lanes: -0.0 + (-0.0) = -0.0 (exact sign bit 0x8000_0000) and 0 + 0 = 0.
    let pad_off = 4_096 * 4;
    assert_eq!(
        &res.slabs["out_f"][pad_off..pad_off + 4],
        &(-0.0_f32).to_le_bytes()
    );
    assert_eq!(
        &res.slabs["out_u"][pad_off..pad_off + 4],
        &0u32.to_le_bytes()
    );

    // The shared dialect source carried by k_selftest_entry lints cleanly for both CUDA and Metal.
    let entry = k_selftest_entry();
    let dialect_src = entry.dialect_source.as_deref().expect("dialect source");
    assert_eq!(dialect_src, K_SELFTEST_DIALECT_SOURCE);
    let shape_defs = Some(SourceFile::new("shape.h", "#define FROB_WORLDS 4112u\n"));
    let (_, cuda_digest, _) = assemble_and_lint(
        BackendTarget::Cuda,
        shape_defs.clone(),
        vec![SourceFile::new("k_selftest_bd014.h", dialect_src)],
    )
    .expect("CUDA dialect lint clean");
    let (_, metal_digest, _) = assemble_and_lint(
        BackendTarget::Metal,
        shape_defs,
        vec![SourceFile::new("k_selftest_bd014.h", dialect_src)],
    )
    .expect("Metal dialect lint clean");
    assert_eq!(cuda_digest, metal_digest);
}

#[test]
fn selftest_wraps() {
    let inputs = SelftestInputs {
        active_lanes: 4,
        pad_lanes: 0,
        a: vec![-0.0, 1.5, -2.25, 0.0],
        b: vec![-0.0, 0.5, 2.25, -0.0],
        x: vec![0xFFFF_FFFF, 0xFFFF_FFFE, 0, 42],
        y: vec![1, 5, 0, 0xFFFF_FFFF],
    };
    let res = run_selftest(1, &inputs);
    let u0 = u32::from_le_bytes(res.slabs["out_u"][0..4].try_into().unwrap());
    let u1 = u32::from_le_bytes(res.slabs["out_u"][4..8].try_into().unwrap());
    assert_eq!(u0, 0, "0xFFFF_FFFF +w 1 must wrap to 0");
    assert_eq!(u1, 3, "0xFFFF_FFFE +w 5 must wrap to 3");

    let f0_bits = u32::from_le_bytes(res.slabs["out_f"][0..4].try_into().unwrap());
    assert_eq!(
        f0_bits,
        (-0.0_f32).to_bits(),
        "-0.0 + (-0.0) preserves -0.0 bit pattern"
    );
}

#[test]
fn receipt_has_twelve_fields() {
    let root = toml::parse(DOCTRINE_FIXTURE.as_bytes()).expect("doctrine fixture parses");
    let mut top = Fields::new(root, "accelerator_doctrine_fields.toml");
    assert_eq!(
        top.require_str("schema").unwrap(),
        "frob.accelerator_doctrine_fields/1"
    );
    let commit = top.require_str("frankensim_commit").unwrap();
    assert_eq!(commit.len(), 40, "pinned FrankenSim commit hash");
    let _src = top.require_str("source_path").unwrap();
    let fields = top.require_array("field").unwrap();
    top.finish().unwrap();
    assert_eq!(fields.len(), 12);

    let inputs = k_selftest_seeded_inputs(64, 0, 1);
    let res = run_selftest(1, &inputs);
    let entries = res.receipt.ae_entries();
    for (idx, sp) in fields.into_iter().enumerate() {
        let mut f = Fields::new(sp.value.as_table().unwrap().clone(), "field");
        let id = f.require_str("id").unwrap();
        let name = f.require_str("name").unwrap();
        f.finish().unwrap();
        assert_eq!(DOCTRINE_FIELDS[idx], (id.as_str(), name.as_str()));
        assert_eq!(entries[idx].0, id);
        assert_eq!(entries[idx].1, name);
        assert!(
            entries[idx].2.is_some_and(|v| !v.trim().is_empty()),
            "{id} ({name}) must be non-empty"
        );
    }
}

#[test]
fn receipt_missing_field_rejected() {
    let inputs = k_selftest_seeded_inputs(32, 0, 7);
    let base = run_selftest(1, &inputs).receipt;
    base.validate().expect("complete receipt validates");

    // Twelve negative controls: deleting AE-k fails and names AE-k.
    for k in 1u8..=12 {
        let mut bad = base.clone();
        clear_ae_field(&mut bad, k);
        let err = bad.validate().expect_err("missing AE field must fail");
        let expected_id = format!("AE-{k:02}");
        assert!(
            matches!(err, ReceiptError::MissingField { id, .. } if id == expected_id),
            "expected MissingField naming {expected_id}, got {err:?}"
        );
        assert!(err.to_string().contains(&expected_id));
    }

    // Agent-boundary refusal for absent fingerprint -> FROB-REF-606 (digest: null).
    let err606 = base
        .validate_agent_boundary("batch-cpu")
        .expect_err("missing fingerprint must refuse at agent boundary");
    assert_eq!(err606.code, RefusalCode::FrobRef606);
    let wire = Refusal::from_json(&err606.to_json()).expect("606 with a valid fingerprint block");
    assert_eq!(
        wire.fingerprint
            .as_ref()
            .map(|fp| (fp.device.as_str(), fp.digest.as_deref())),
        Some(("batch-cpu", None))
    );
    assert!(err606.repairs[0].action.contains("frob spark fingerprint"));

    // Fail-closed negative control: setting an arbitrary nonblank digest on an
    // M0 non-citable/unplaced receipt must STILL be refused with FROB-REF-606.
    let mut forged = base;
    forged.fingerprint_digest = Some("deadbeef".into());
    let err_stale = forged
        .validate_agent_boundary("batch-cpu")
        .expect_err("non-citable/unverified digest must remain fail-closed");
    assert_eq!(err_stale.code, RefusalCode::FrobRef606);
    assert_eq!(
        err_stale
            .fingerprint
            .as_ref()
            .map(|fp| (fp.device.as_str(), fp.digest.as_deref())),
        Some(("batch-cpu", None))
    );
    assert_eq!(err_stale.observed, "deadbeef");
}

#[test]
fn undeclared_alias_refused() {
    let class = k_selftest_shape_class(64);
    let extent = CapExpr::worlds(1);
    let mk_bind = |arg: &str, slab_id: &str, elem: ElemType| SlabBinding {
        arg: arg.into(),
        slab_id: slab_id.into(),
        elem,
        extents: vec![extent],
        strides: vec![1],
        stride_unit: StrideUnit::Elements,
        stride_signed: false,
        origin: CapExpr::constant(0),
        alloc_extent: extent,
        slab_offset: 0,
        from_stat_kernel: false,
    };
    let mut entry = k_selftest_entry();
    entry.compile_bindings = vec![
        mk_bind("a", "slab-shared", ElemType::F32),
        mk_bind("b", "slab-b", ElemType::F32),
        mk_bind("x", "slab-x", ElemType::U32),
        mk_bind("y", "slab-y", ElemType::U32),
        mk_bind("out_f", "slab-shared", ElemType::F32),
        mk_bind("out_u", "slab-out-u", ElemType::U32),
    ];
    let lib = KernelLibrary {
        entries: vec![entry],
        shape_class: class,
        declared_outputs: vec![
            DeclaredOutput {
                arg: "out_f".into(),
                slab_id: "slab-shared".into(),
                lo: 0,
                hi: 64,
            },
            DeclaredOutput {
                arg: "out_u".into(),
                slab_id: "slab-out-u".into(),
                lo: 0,
                hi: 64,
            },
        ],
    };

    let mut cpu = BatchCpu::new(1);
    let err = cpu
        .compile(&lib)
        .expect_err("undeclared write alias on `a` must refuse at compile");
    assert_eq!(err.code, RefusalCode::FrobRef802);
    assert!(err.message.contains("K-selftest"));
    assert!(err.message.contains("slab-shared") && err.message.contains("out_f"));
}

#[test]
fn kir_program_footprint_evidence_admitted_and_required() {
    let class = k_selftest_shape_class(32);
    let prog_id = hash_blob(b"frob.kir.test-program/1");

    let mut manifest = k_selftest_manifest();
    manifest.name = "K-selftest-kir".into();
    manifest.signature.footprint_evidence = FootprintEvidence::KirProgram(prog_id);
    manifest.id = manifest.compute_id().unwrap();

    // Negative control: KirProgram manifest without ProgramFootprint is refused.
    let mut missing_entry = k_selftest_entry();
    missing_entry.manifest = manifest.clone();
    missing_entry.program_footprint = None;
    let mut cpu = BatchCpu::new(1);
    let err = cpu
        .compile(&KernelLibrary {
            entries: vec![missing_entry],
            shape_class: class.clone(),
            declared_outputs: Vec::new(),
        })
        .expect_err("KirProgram manifest without ProgramFootprint must fail compile");
    assert_eq!(err.code, RefusalCode::FrobRef802);
    assert!(err.message.contains("K-selftest-kir"));

    // Positive control: supplying matching ProgramFootprint compiles cleanly.
    let mut good_entry = k_selftest_entry();
    good_entry.manifest = manifest.clone();
    good_entry.program_footprint = Some(ProgramFootprint {
        program_id: prog_id,
        reads: manifest.signature.reads.clone(),
        writes: manifest.signature.writes.clone(),
        resources: manifest.signature.resources,
    });
    let compiled = cpu
        .compile(&KernelLibrary {
            entries: vec![good_entry],
            shape_class: class,
            declared_outputs: Vec::new(),
        })
        .expect("KirProgram manifest with matching ProgramFootprint compiles");
    assert_eq!(compiled.id_for("K-selftest-kir"), Some(manifest.id));
}

#[test]
fn plan_rejects_bad_binding() {
    let class = k_selftest_shape_class(32);
    let mut cpu = BatchCpu::new(1);
    let compiled = cpu
        .compile(&KernelLibrary {
            entries: vec![k_selftest_entry()],
            shape_class: class.clone(),
            declared_outputs: Vec::new(),
        })
        .expect("compile succeeds");
    let kid = compiled.id_for("K-selftest").unwrap();

    let _bufs = cpu
        .allocate(&BufferLayout {
            slabs: vec![
                SlabSpec::new("a", ElemType::F32, 32),
                SlabSpec::new("b", ElemType::F32, 32),
                SlabSpec::new("x_wrong_f32", ElemType::F32, 32),
                SlabSpec::new("y", ElemType::U32, 32),
                SlabSpec::new("out_f", ElemType::F32, 32),
                SlabSpec::new("out_u", ElemType::U32, 32),
            ],
        })
        .expect("allocate succeeds");
    let stage = Stage {
        name: "bad-stage".into(),
        kernel_id: kid,
        bindings: selftest_bindings(),
    };
    let err = cpu
        .plan(&[stage], &class)
        .expect_err("binding f32 slab to u32 arg must fail in plan");
    assert_eq!(err.code, RefusalCode::FrobRef802);
    let lower = err.message.to_ascii_lowercase();
    assert!(
        err.message.contains("x"),
        "error must name arg x: {}",
        err.message
    );
    assert!(
        lower.contains("f32") && lower.contains("u32"),
        "error must name observed and expected element types: {}",
        err.message
    );
}

#[test]
fn launch_rejects_mismatched_buffer_set() {
    let class = k_selftest_shape_class(32);
    let mut cpu = BatchCpu::new(2);
    let compiled = cpu
        .compile(&KernelLibrary {
            entries: vec![k_selftest_entry()],
            shape_class: class.clone(),
            declared_outputs: Vec::new(),
        })
        .expect("compile succeeds");
    let kid = compiled.id_for("K-selftest").unwrap();

    let _bufs_a = cpu.allocate(&selftest_layout(32)).expect("allocate A");
    let plan_a = cpu
        .plan(
            &[Stage {
                name: "stage-a".into(),
                kernel_id: kid,
                bindings: selftest_bindings(),
            }],
            &class,
        )
        .expect("plan A");

    // Allocate a second BufferSet with identical byte lengths (all 4-byte elems)
    // but swapped element type on `out_u` (`F32` instead of `U32`).
    let mut bufs_b = cpu
        .allocate(&BufferLayout {
            slabs: vec![
                SlabSpec::new("a", ElemType::F32, 32),
                SlabSpec::new("b", ElemType::F32, 32),
                SlabSpec::new("x", ElemType::U32, 32),
                SlabSpec::new("y", ElemType::U32, 32),
                SlabSpec::new("out_f", ElemType::F32, 32),
                SlabSpec::new("out_u_wrong_f32", ElemType::F32, 32),
            ],
        })
        .expect("allocate B");
    let inputs = k_selftest_seeded_inputs(32, 0, 1);
    cpu.upload(&mut bufs_b, SlabId(0), &inputs.a_bytes())
        .unwrap();
    cpu.upload(&mut bufs_b, SlabId(1), &inputs.b_bytes())
        .unwrap();
    cpu.upload(&mut bufs_b, SlabId(2), &inputs.x_bytes())
        .unwrap();
    cpu.upload(&mut bufs_b, SlabId(3), &inputs.y_bytes())
        .unwrap();

    let err = cpu
        .launch(&plan_a, &mut bufs_b, &Cx::new())
        .expect_err("launch with mismatched BufferSet must refuse before writes");
    assert_eq!(err.code, RefusalCode::FrobRef802);
    assert!(
        !bufs_b.is_written(SlabId(4)) && !bufs_b.is_written(SlabId(5)),
        "mismatched BufferSet outputs must remain unwritten"
    );
    let mut readback = [0xa5; 32 * 4];
    cpu.read_back(&bufs_b, SlabId(4), &mut readback)
        .expect_err("refused launch exposes no output bytes");
    assert_eq!(readback, [0xa5; 32 * 4]);
}

#[test]
fn worker_count_invariance() {
    let inputs = k_selftest_seeded_inputs(4_096, 0, 1);
    let ref_run = run_selftest(1, &inputs);
    for workers in [2usize, 8, 36] {
        let run = run_selftest(workers, &inputs);
        assert_eq!(
            run.digests["out_f"], ref_run.digests["out_f"],
            "out_f digest mismatch at workers={workers}"
        );
        assert_eq!(
            run.digests["out_u"], ref_run.digests["out_u"],
            "out_u digest mismatch at workers={workers}"
        );
    }

    // Real multi-threaded arrival-order mutant: two worker threads compute their
    // own chunks of K-selftest and send them through a channel; worker 0 waits
    // until worker 1 has sent its chunk, so arrival-order assembly appends
    // worker 1 ahead of worker 0 and breaks worker-count invariance.
    let total = inputs.total_lanes();
    let half = total / 2;
    let arrival_digest = |workers: usize| -> Blake3 {
        if workers == 1 {
            let mut out = Vec::with_capacity(total * 4);
            for i in 0..total {
                out.extend_from_slice(&inputs.x[i].wrapping_add(inputs.y[i]).to_le_bytes());
            }
            return hash_blob(&out);
        }
        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let (gate_tx, gate_rx) = mpsc::channel::<()>();
        let x_ref = &inputs.x;
        let y_ref = &inputs.y;
        std::thread::scope(|s| {
            let tx0 = tx.clone();
            s.spawn(move || {
                let mut chunk = Vec::with_capacity(half * 4);
                for i in 0..half {
                    chunk.extend_from_slice(&x_ref[i].wrapping_add(y_ref[i]).to_le_bytes());
                }
                let _ = gate_rx.recv();
                tx0.send(chunk).unwrap();
            });
            s.spawn(move || {
                let mut chunk = Vec::with_capacity((total - half) * 4);
                for i in half..total {
                    chunk.extend_from_slice(&x_ref[i].wrapping_add(y_ref[i]).to_le_bytes());
                }
                tx.send(chunk).unwrap();
                gate_tx.send(()).unwrap();
            });
        });
        let mut assembled = Vec::with_capacity(total * 4);
        for chunk in rx {
            assembled.extend_from_slice(&chunk);
        }
        hash_blob(&assembled)
    };
    assert_eq!(arrival_digest(1), ref_run.digests["out_u"]);
    assert_ne!(
        arrival_digest(2),
        arrival_digest(1),
        "planted arrival-order worker mutant must break worker-count invariance"
    );
}

#[test]
fn cancel_between_launches() {
    static SECOND_RUNS: AtomicUsize = AtomicUsize::new(0);
    fn sentinel_second(args: &mut KernelArgs<'_>, lanes: Range<u32>) {
        SECOND_RUNS.fetch_add(1, Ordering::SeqCst);
        for i in lanes {
            args.write_f32("out_f", i, 999.0);
            args.write_u32("out_u", i, 999);
        }
    }

    SECOND_RUNS.store(0, Ordering::SeqCst);
    let class = k_selftest_shape_class(64);
    let mut m2 = k_selftest_manifest();
    m2.name = "K-selftest-second".into();
    m2.id = m2.compute_id().unwrap();
    let e2 = KernelEntry {
        manifest: m2,
        cpu_symbol: Some(CpuKernelSymbol {
            symbol_name: "test::sentinel_second".into(),
            func: sentinel_second,
        }),
        dialect_source: None,
        program_footprint: None,
        compile_bindings: Vec::new(),
    };

    let mut cpu = BatchCpu::new(2);
    let compiled = cpu
        .compile(&KernelLibrary {
            entries: vec![k_selftest_entry(), e2],
            shape_class: class.clone(),
            declared_outputs: Vec::new(),
        })
        .expect("compile succeeds");
    let k1 = compiled.id_for("K-selftest").unwrap();
    let k2 = compiled.id_for("K-selftest-second").unwrap();

    let mut bufs = cpu.allocate(&selftest_layout(64)).expect("allocate");
    let inputs = k_selftest_seeded_inputs(64, 0, 1);
    cpu.upload(&mut bufs, SlabId(0), &inputs.a_bytes()).unwrap();
    cpu.upload(&mut bufs, SlabId(1), &inputs.b_bytes()).unwrap();
    cpu.upload(&mut bufs, SlabId(2), &inputs.x_bytes()).unwrap();
    cpu.upload(&mut bufs, SlabId(3), &inputs.y_bytes()).unwrap();

    let plan = cpu
        .plan(
            &[
                Stage {
                    name: "stage-1".into(),
                    kernel_id: k1,
                    bindings: selftest_bindings(),
                },
                Stage {
                    name: "stage-2".into(),
                    kernel_id: k2,
                    bindings: selftest_bindings(),
                },
            ],
            &class,
        )
        .expect("plan");

    let cancelled_after = Arc::new(AtomicUsize::new(0));
    let flag = Arc::clone(&cancelled_after);
    cpu.set_boundary_hook(move |idx, cx| {
        if idx == 0 {
            flag.fetch_add(1, Ordering::SeqCst);
            cx.cancel();
        }
    });

    let cx = Cx::new();
    let receipt = cpu
        .launch(&plan, &mut bufs, &cx)
        .expect("launch drains cleanly");
    receipt
        .validate()
        .expect("cancelled-drained receipt still valid");
    assert_eq!(
        receipt
            .ae10_cancellation_drain_and_finalize_outcome
            .as_deref(),
        Some("cancelled-drained")
    );
    assert_eq!(receipt.launch_count, 1);
    assert_eq!(cancelled_after.load(Ordering::SeqCst), 1);
    assert_eq!(SECOND_RUNS.load(Ordering::SeqCst), 0);

    let mut out_f = vec![0u8; 64 * 4];
    cpu.read_back(&bufs, SlabId(4), &mut out_f)
        .expect("read_back succeeds after drain");
    assert_eq!(out_f, k_selftest_scalar_reference(&inputs).out_f_bytes());

    // Final-boundary cancellation: cancelling in the hook of a single-stage plan
    // also settles AE-10 to cancelled-drained.
    let single_plan = cpu
        .plan(
            &[Stage {
                name: "stage-only".into(),
                kernel_id: k1,
                bindings: selftest_bindings(),
            }],
            &class,
        )
        .expect("single plan");
    cpu.set_boundary_hook(|_, cx| cx.cancel());
    let final_receipt = cpu
        .launch(&single_plan, &mut bufs, &Cx::new())
        .expect("final boundary cancel drains cleanly");
    assert_eq!(
        final_receipt
            .ae10_cancellation_drain_and_finalize_outcome
            .as_deref(),
        Some("cancelled-drained")
    );
    assert_eq!(final_receipt.launch_count, 1);
}

#[test]
fn cancelled_before_launch_withholds_output() {
    let inputs = k_selftest_seeded_inputs(32, 0, 5);
    let total = inputs.total_lanes();
    let cx = Cx::new();
    cx.cancel();

    let mut cpu = BatchCpu::new(2);
    let res = Harness::run(
        &mut cpu,
        k_selftest_entry(),
        &inputs.harness_inputs(),
        &k_selftest_output_specs(total),
        &k_selftest_shape_class(total),
        &cx,
    )
    .expect("pre-cancelled harness returns drained receipt");
    res.receipt.validate().expect("cancelled receipt valid");
    assert_eq!(
        res.receipt
            .ae10_cancellation_drain_and_finalize_outcome
            .as_deref(),
        Some("cancelled-drained")
    );
    assert_eq!(res.receipt.launch_count, 0);
    assert!(
        res.slabs.is_empty() && res.digests.is_empty(),
        "cancelled-before-launch must withhold output bytes and digests"
    );
}

#[test]
fn read_back_only_after_completion() {
    let mut cpu = BatchCpu::new(1);
    let layout = selftest_layout(16);
    let mut bufs = cpu.allocate(&layout).expect("allocate");
    let output = SlabId(4);
    let err = bufs
        .mark_written(output)
        .expect_err("callers cannot mark CPU output complete");
    assert_eq!(err.code, RefusalCode::FrobRef802);
    assert!(!bufs.is_written(output));
    assert!(!format!("{bufs:?}").contains("raw:"));
    let mut out = [0xa5; 16 * 4];
    let err = cpu
        .read_back(&bufs, output, &mut out)
        .expect_err("read_back of unwritten output slab before launch must refuse");
    assert_eq!(err.code, RefusalCode::FrobRef802);
    assert!(err.message.contains("out_f"));
    assert_eq!(out, [0xa5; 16 * 4]);

    let mut device_bufs = BufferSet::for_device(&layout);
    device_bufs
        .mark_written(output)
        .expect("record device completion");
    assert!(device_bufs.is_written(output));
}

#[test]
fn unavailable_backend_refuses_601() {
    let mut cpu = BatchCpu::new(1).with_unavailable(true);
    let err = cpu
        .allocate(&BufferLayout {
            slabs: vec![SlabSpec::new("a", ElemType::F32, 8)],
        })
        .expect_err("unavailable backend refuses with FROB-REF-601");
    assert_eq!(err.code, RefusalCode::FrobRef601);
}

#[test]
fn inv_spark_02_every_launch_receipt_validates() {
    for (seed, lanes, workers) in [
        (1u64, 17usize, 1usize),
        (7, 64, 2),
        (99, 257, 4),
        (42, 1_024, 8),
    ] {
        let inputs = k_selftest_seeded_inputs(lanes, 3, seed);
        let total = inputs.total_lanes();
        let class = k_selftest_shape_class(total);

        // Direct KernelBackend::launch validation (without Harness's own check).
        let mut cpu = BatchCpu::new(workers);
        let compiled = cpu
            .compile(&KernelLibrary {
                entries: vec![k_selftest_entry()],
                shape_class: class.clone(),
                declared_outputs: Vec::new(),
            })
            .expect("compile");
        let kid = compiled.id_for("K-selftest").unwrap();
        let mut bufs = cpu.allocate(&selftest_layout(total)).expect("allocate");
        cpu.upload(&mut bufs, SlabId(0), &inputs.a_bytes()).unwrap();
        cpu.upload(&mut bufs, SlabId(1), &inputs.b_bytes()).unwrap();
        cpu.upload(&mut bufs, SlabId(2), &inputs.x_bytes()).unwrap();
        cpu.upload(&mut bufs, SlabId(3), &inputs.y_bytes()).unwrap();
        let plan = cpu
            .plan(
                &[Stage {
                    name: "direct".into(),
                    kernel_id: kid,
                    bindings: selftest_bindings(),
                }],
                &class,
            )
            .expect("plan");
        let receipt = cpu
            .launch(&plan, &mut bufs, &Cx::new())
            .expect("direct launch");
        receipt
            .validate()
            .unwrap_or_else(|e| panic!("seed={seed} lanes={lanes} w={workers}: {e}"));
    }
}
