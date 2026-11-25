#![deny(unsafe_code)]

//! Unit and property tests for `KernelBackend`, `Receipt` (`AE-01`..`AE-12`),
//! `BatchCpu`, `Harness`, and `K-selftest` (`BD-014`).

use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use frob_core::id::hash_blob;
use frob_core::refusal::RefusalCode;
use frob_core::registry::toml::{self, Fields};
use frob_kernels::backend::{
    BufferLayout, Cx, KernelArgs, KernelBackend, KernelEntry, KernelLibrary, KernelSource,
    SLAB_ALIGNMENT_BYTES, SlabId, SlabSpec, Stage, StageBinding,
};
use frob_kernels::cpu::BatchCpu;
use frob_kernels::harness::{Harness, HarnessInput, HarnessOutputSpec};
use frob_kernels::manifest::{
    CapExpr, DeclaredOutput, ElemType, ShapeClassCaps, SlabBinding, StrideUnit,
};
use frob_kernels::receipt::{DOCTRINE_FIELDS, ReceiptError};
use frob_kernels::selftest::{
    SelftestInputs, k_selftest_entry, k_selftest_manifest, k_selftest_scalar_reference,
    k_selftest_seeded_inputs,
};

const DOCTRINE_FIXTURE: &str = include_str!("fixtures/accelerator_doctrine_fields.toml");

fn shape_for(worlds: usize) -> ShapeClassCaps {
    ShapeClassCaps {
        name: format!("selftest-w{worlds}"),
        nv_cap: 12,
        link_cap: 13,
        contact_cap: 24,
        row_cap: 72,
        sample_cap: 16,
        actuator_cap: 12,
        worlds: worlds as u64,
        extra: BTreeMap::new(),
    }
}

fn run_selftest(workers: usize, inputs: &SelftestInputs) -> frob_kernels::harness::HarnessOutput {
    let total = inputs.total_lanes();
    let class = shape_for(total);
    let mut cpu = BatchCpu::new(workers);
    let h_inputs = [
        HarnessInput {
            name: "a".into(),
            elem: ElemType::F32,
            len: total,
            bytes: inputs.a_bytes(),
        },
        HarnessInput {
            name: "b".into(),
            elem: ElemType::F32,
            len: total,
            bytes: inputs.b_bytes(),
        },
        HarnessInput {
            name: "x".into(),
            elem: ElemType::U32,
            len: total,
            bytes: inputs.x_bytes(),
        },
        HarnessInput {
            name: "y".into(),
            elem: ElemType::U32,
            len: total,
            bytes: inputs.y_bytes(),
        },
    ];
    let h_outputs = [
        HarnessOutputSpec {
            name: "out_f".into(),
            elem: ElemType::F32,
            len: total,
        },
        HarnessOutputSpec {
            name: "out_u".into(),
            elem: ElemType::U32,
            len: total,
        },
    ];
    Harness::run(
        &mut cpu,
        k_selftest_entry(),
        &h_inputs,
        &h_outputs,
        &class,
        &Cx::new(),
    )
    .expect("K-selftest harness run succeeds")
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
        bad.clear_ae_field(k);
        let err = bad.validate().expect_err("missing AE field must fail");
        let expected_id = format!("AE-{k:02}");
        assert!(
            matches!(err, ReceiptError::MissingField { id, .. } if id == expected_id),
            "expected MissingField naming {expected_id}, got {err:?}"
        );
        assert!(err.to_string().contains(&expected_id));
    }

    // Agent-boundary refusal for absent fingerprint -> FROB-REF-606.
    let err606 = base
        .validate_agent_boundary("batch-cpu")
        .expect_err("missing fingerprint must refuse at agent boundary");
    assert_eq!(err606.code, RefusalCode::FrobRef606);
    assert!(err606.repairs[0].action.contains("frob spark fingerprint"));
}

#[test]
fn undeclared_alias_refused() {
    let class = shape_for(64);
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
    // Plant undeclared alias: `out_f` writes to the same backing slab as input `a`.
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
    assert_eq!(err.code, RefusalCode::FrobRef603);
    assert!(
        err.message.contains("K-selftest"),
        "error must name kernel: {}",
        err.message
    );
    assert!(
        err.message.contains("slab-shared") && err.message.contains("out_f"),
        "error must name aliased buffer/arg: {}",
        err.message
    );
}

#[test]
fn plan_rejects_bad_binding() {
    let class = shape_for(32);
    let mut cpu = BatchCpu::new(1);
    let compiled = cpu
        .compile(&KernelLibrary {
            entries: vec![k_selftest_entry()],
            shape_class: class.clone(),
            declared_outputs: Vec::new(),
        })
        .expect("compile succeeds");
    let kid = compiled.id_for("K-selftest").unwrap();

    // Allocate an f32 slab for `x` (which manifest requires to be u32).
    let bufs = cpu
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
    assert_eq!(
        (bufs.get(SlabId(0)).unwrap().as_bytes().as_ptr() as usize) % SLAB_ALIGNMENT_BYTES,
        0
    );

    let stage = Stage {
        name: "bad-stage".into(),
        kernel_id: kid,
        bindings: vec![
            StageBinding {
                arg: "a".into(),
                slab: SlabId(0),
            },
            StageBinding {
                arg: "b".into(),
                slab: SlabId(1),
            },
            StageBinding {
                arg: "x".into(),
                slab: SlabId(2),
            },
            StageBinding {
                arg: "y".into(),
                slab: SlabId(3),
            },
            StageBinding {
                arg: "out_f".into(),
                slab: SlabId(4),
            },
            StageBinding {
                arg: "out_u".into(),
                slab: SlabId(5),
            },
        ],
    };
    let err = cpu
        .plan(&[stage], &class)
        .expect_err("binding f32 slab to u32 arg must fail in plan");
    assert_eq!(err.code, RefusalCode::FrobRef603);
    assert!(err.message.contains("x"));
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

    // Negative control: planted arrival-order collection of K-selftest chunks
    // (worker 1 completes before worker 0) changes the digest vs fixed-index placement.
    let bytes_ref = &ref_run.slabs["out_u"];
    let half = bytes_ref.len() / 2;
    let mut arrival_bytes = Vec::with_capacity(bytes_ref.len());
    arrival_bytes.extend_from_slice(&bytes_ref[half..]);
    arrival_bytes.extend_from_slice(&bytes_ref[..half]);
    assert_ne!(
        hash_blob(&arrival_bytes),
        ref_run.digests["out_u"],
        "planted arrival-order dependence must break worker_count_invariance"
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
    let class = shape_for(64);
    let mut m2 = k_selftest_manifest();
    m2.name = "K-selftest-second".into();
    m2.id = m2.compute_id().unwrap();
    let e2 = KernelEntry {
        manifest: m2,
        source: KernelSource::CpuSymbol {
            symbol_name: "test::sentinel_second".into(),
            func: sentinel_second,
        },
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

    let mut bufs = cpu
        .allocate(&BufferLayout {
            slabs: vec![
                SlabSpec::new("a", ElemType::F32, 64),
                SlabSpec::new("b", ElemType::F32, 64),
                SlabSpec::new("x", ElemType::U32, 64),
                SlabSpec::new("y", ElemType::U32, 64),
                SlabSpec::new("out_f", ElemType::F32, 64),
                SlabSpec::new("out_u", ElemType::U32, 64),
            ],
        })
        .expect("allocate");
    let inputs = k_selftest_seeded_inputs(64, 0, 1);
    bufs.upload(SlabId(0), &inputs.a_bytes()).unwrap();
    bufs.upload(SlabId(1), &inputs.b_bytes()).unwrap();
    bufs.upload(SlabId(2), &inputs.x_bytes()).unwrap();
    bufs.upload(SlabId(3), &inputs.y_bytes()).unwrap();

    let mk_bindings = || {
        vec![
            StageBinding {
                arg: "a".into(),
                slab: SlabId(0),
            },
            StageBinding {
                arg: "b".into(),
                slab: SlabId(1),
            },
            StageBinding {
                arg: "x".into(),
                slab: SlabId(2),
            },
            StageBinding {
                arg: "y".into(),
                slab: SlabId(3),
            },
            StageBinding {
                arg: "out_f".into(),
                slab: SlabId(4),
            },
            StageBinding {
                arg: "out_u".into(),
                slab: SlabId(5),
            },
        ]
    };
    let plan = cpu
        .plan(
            &[
                Stage {
                    name: "stage-1".into(),
                    kernel_id: k1,
                    bindings: mk_bindings(),
                },
                Stage {
                    name: "stage-2".into(),
                    kernel_id: k2,
                    bindings: mk_bindings(),
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
    assert_eq!(
        SECOND_RUNS.load(Ordering::SeqCst),
        0,
        "second launch must not execute"
    );

    // Buffers are no longer borrowed and can be read back cleanly.
    let mut out_f = vec![0u8; 64 * 4];
    cpu.read_back(&bufs, SlabId(4), &mut out_f)
        .expect("read_back succeeds after drain");
    assert_eq!(out_f, k_selftest_scalar_reference(&inputs).out_f_bytes());
}

#[test]
fn read_back_only_after_completion() {
    // Compile-time enforcement lives in the doc-test on `backend.rs`;
    // also verify unavailable backend raises FROB-REF-601 (exit 6).
    let mut cpu = BatchCpu::new(1).with_unavailable(true);
    let err = cpu
        .allocate(&BufferLayout {
            slabs: vec![SlabSpec::new("a", ElemType::F32, 8)],
        })
        .expect_err("unavailable backend refuses");
    assert_eq!(err.code, RefusalCode::FrobRef601);
}

#[test]
fn inv_spark_02_every_launch_receipt_validates() {
    for (seed, lanes, workers) in [(1u64, 17usize, 1usize), (99, 257, 4), (42, 1_024, 8)] {
        let inputs = k_selftest_seeded_inputs(lanes, 3, seed);
        let res = run_selftest(workers, &inputs);
        res.receipt
            .validate()
            .unwrap_or_else(|e| panic!("seed={seed} lanes={lanes} w={workers}: {e}"));
    }
}
