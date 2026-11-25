#![deny(unsafe_code)]

//! Unit and property tests for `frob.kernel/1` kernel manifest, stage signature,
//! static/binding validation, and `registries/kernels.toml` provenance (`N21b-01`).
//!
//! All test manifests constructed here are labelled `synthetic`.

use std::fs;
use std::path::PathBuf;

use frob_core::evidence::DetClass;
use frob_core::id::{Blake3, hash_blob};
use frob_kernels::manifest::{
    ArgDecl, ArgLayout, ArgView, BackendLimits, CapExpr, DeclaredOutput, DoctorKernelsReport,
    ElemKind, ElemType, Footprint, FootprintEvidence, IdentityDecl, KernelClass, KernelId,
    KernelManifest, KernelOrigin, KernelRegistry, KernelRole, LaneScope, ManifestError,
    NeutralSpec, NumericalContract, PrecisionProfile, ProgramFootprint, ReductionConsumer,
    ResourceBounds, ShapeClassCaps, SlabBinding, StageSignature, StageWriteEpoch, StrideUnit,
};

fn zero_id() -> KernelId {
    KernelId(Blake3::from_bytes([0u8; 32]))
}

fn synthetic_quat_manifest() -> KernelManifest {
    let extent = CapExpr::row_cap(4);
    KernelManifest {
        id: zero_id(),
        name: "K-synthetic-quat".into(),
        class: KernelClass::Float,
        role: KernelRole::Ordinary,
        args: vec![
            ArgDecl {
                id: "q_in".into(),
                elem: ElemType::F32,
                view: ArgView::Value,
                shape: vec![extent],
                dim: [0; 6],
                kind: ElemKind::Quaternion,
                components: vec!["w".into(), "x".into(), "y".into(), "z".into()],
                state: true,
            },
            ArgDecl {
                id: "q_out".into(),
                elem: ElemType::F32,
                view: ArgView::Value,
                shape: vec![extent],
                dim: [0; 6],
                kind: ElemKind::Quaternion,
                components: vec!["w".into(), "x".into(), "y".into(), "z".into()],
                state: true,
            },
        ],
        determinism: DetClass::D2PerBackend,
        identities: vec![IdentityDecl {
            name: "T-01".into(),
            statement: "cay(Omega)^T * cay(Omega) = I".into(),
        }],
        derivative: None,
        cpu_reference: "frob_kernels::cpu::synthetic_quat".into(),
        signature: StageSignature {
            reads: vec![Footprint {
                arg: "q_in".into(),
                lanes: LaneScope::OwnWorld,
                lo: CapExpr::constant(0),
                hi: extent,
            }],
            writes: vec![Footprint {
                arg: "q_out".into(),
                lanes: LaneScope::OwnWorld,
                lo: CapExpr::constant(0),
                hi: extent,
            }],
            may_alias: vec![],
            layout: vec![
                ArgLayout {
                    arg: "q_in".into(),
                    elem: ElemType::F32,
                    extents: vec![extent],
                    strides: vec![1],
                    stride_unit: StrideUnit::Elements,
                    stride_signed: false,
                    origin: CapExpr::constant(0),
                    alloc_extent: extent,
                },
                ArgLayout {
                    arg: "q_out".into(),
                    elem: ElemType::F32,
                    extents: vec![extent],
                    strides: vec![1],
                    stride_unit: StrideUnit::Elements,
                    stride_signed: false,
                    origin: CapExpr::constant(0),
                    alloc_extent: extent,
                },
            ],
            epoch_read: -1,
            epoch_write: 0,
            resources: ResourceBounds {
                live_values: 16,
                stack_bytes: 0,
                max_loop_bound: 64,
                buffer_args: 2,
            },
            numerics: NumericalContract {
                precision: PrecisionProfile::F32,
                error_bound: None,
                deferral_threshold: None,
                exact_fallback: None,
                neutrals: vec![],
                padded_reduction_outputs: vec![],
            },
            feeds_state: true,
            footprint_evidence: FootprintEvidence::Lint,
        },
    }
    .seal()
    .expect("seal synthetic manifest")
}

#[test]
fn manifest_roundtrip_and_id() {
    let m = synthetic_quat_manifest();
    let bytes1 = m.to_canonical_json().expect("serialize 1");
    let parsed = KernelManifest::parse_json(bytes1.as_bytes()).expect("parse");
    let bytes2 = parsed.to_canonical_json().expect("serialize 2");
    assert_eq!(bytes1.as_bytes(), bytes2.as_bytes());
    assert_eq!(parsed.id, m.id);

    // Metamorphic: permuting argument declaration order leaves KernelId unchanged.
    let mut permuted = m.clone();
    permuted.args.reverse();
    permuted.signature.layout.reverse();
    assert_eq!(permuted.compute_id().unwrap(), m.id);

    // Changing one footprint bound changes KernelId.
    let mut changed = m.clone();
    changed.signature.writes[0].hi = CapExpr::row_cap(2);
    assert_ne!(changed.compute_id().unwrap(), m.id);
}

#[test]
fn arg_meaning_and_provenance_fields() {
    let m = synthetic_quat_manifest();
    let m_json = m.to_canonical_json().unwrap();
    let parsed = KernelManifest::parse_json(m_json.as_bytes()).unwrap();
    assert_eq!(parsed.args[0].kind, ElemKind::Quaternion);
    assert_eq!(parsed.args[0].components, vec!["w", "x", "y", "z"]);
    assert!(parsed.args[0].state);

    // Missing `kind` must fail to parse.
    let raw = m_json.as_str().replace(r#""kind":"quaternion","#, "");
    assert!(matches!(
        KernelManifest::parse_json(raw.as_bytes()),
        Err(ManifestError::Parse(_))
    ));

    // Changing `kind` to `vector` changes KernelId.
    let mut vec_kind = m.clone();
    vec_kind.args[0].kind = ElemKind::Vector;
    assert_ne!(vec_kind.compute_id().unwrap(), m.id);

    // Registry row with `origin = derived { ... }` and `lowers` round-trips;
    // changing `derivation_hash` or `lowering_hash` leaves KernelId unchanged.
    let id_hex = m.id.to_hex();
    let toml_a = format!(
        r#"schema = "frob.kernels/1"
[[kernel]]
name = "K-synthetic-quat"
manifest = "k_synthetic_quat.json"
id = "{id_hex}"
origin = {{ kind = "derived", derivation_hash = "aaaa", source_hash = "bbbb" }}
lowers = [{{ arg = "q_in", model = "frob.design/1", field_path = "links[].joint.rotation", lowering_hash = "1111" }}]
"#
    );
    let toml_b = format!(
        r#"schema = "frob.kernels/1"
[[kernel]]
name = "K-synthetic-quat"
manifest = "k_synthetic_quat.json"
id = "{id_hex}"
origin = {{ kind = "derived", derivation_hash = "cccc", source_hash = "bbbb" }}
lowers = [{{ arg = "q_in", model = "frob.design/1", field_path = "links[].joint.rotation", lowering_hash = "2222" }}]
"#
    );
    let reg_a = KernelRegistry::parse_toml(toml_a.as_bytes()).expect("parse reg_a");
    let reg_b = KernelRegistry::parse_toml(toml_b.as_bytes()).expect("parse reg_b");
    assert_eq!(reg_a.rows[0].id, m.id);
    assert_eq!(reg_b.rows[0].id, m.id);
    assert!(matches!(reg_a.rows[0].origin, KernelOrigin::Derived { .. }));

    // Row without `origin` fails to load and names the kernel.
    let no_origin = format!(
        r#"schema = "frob.kernels/1"
[[kernel]]
name = "K-missing-origin"
manifest = "k.json"
id = "{id_hex}"
"#
    );
    let err =
        KernelRegistry::parse_toml(no_origin.as_bytes()).expect_err("missing origin must fail");
    assert!(
        err.to_string().contains("K-missing-origin"),
        "error must name kernel: {err}"
    );

    // Row whose state argument has no `lowers` entry still loads (so N10a-03 D-1 lint reports it).
    let no_lowers = format!(
        r#"schema = "frob.kernels/1"
[[kernel]]
name = "K-synthetic-quat"
manifest = "k_synthetic_quat.json"
id = "{id_hex}"
origin = {{ kind = "skeleton" }}
"#
    );
    let reg_nl = KernelRegistry::parse_toml(no_lowers.as_bytes()).expect("missing lowers loads");
    assert!(reg_nl.rows[0].lowers.is_empty());
}

#[test]
fn footprint_eval_all_classes() {
    let m = synthetic_quat_manifest();
    let classes = ShapeClassCaps::default_classes();
    assert_eq!(classes.len(), 6);
    m.validate_static(&classes, None, false)
        .expect("valid on all default classes");

    // Metamorphic: adding an unused capacity leaves validation unchanged.
    let mut extended = classes.clone();
    for c in &mut extended {
        c.extra.insert("unused_cap".into(), 999);
    }
    m.validate_static(&extended, None, false)
        .expect("valid with unused extra cap");

    // Static rule 2 violation: footprint hi exceeds alloc_extent.
    let mut bad = m.clone();
    bad.signature.writes[0].hi = CapExpr::row_cap(5);
    assert!(matches!(
        bad.validate_static(&classes, None, false),
        Err(ManifestError::FootprintOutOfRange { .. })
    ));

    // Static rule 1 violation: unreferenced argument.
    let mut unref = m.clone();
    unref.signature.reads.clear();
    assert!(matches!(
        unref.validate_static(&classes, None, false),
        Err(ManifestError::FootprintOutOfRange { .. })
    ));
}

#[test]
fn undeclared_alias_refused() {
    let m = synthetic_quat_manifest();
    let class = ShapeClassCaps::default_classes().remove(0);
    let extent = CapExpr::row_cap(4);
    let hi = extent.eval(&class).unwrap();
    let bindings = vec![
        SlabBinding {
            arg: "q_in".into(),
            slab_id: "slab_shared".into(),
            elem: ElemType::F32,
            extents: vec![extent],
            strides: vec![1],
            stride_unit: StrideUnit::Elements,
            stride_signed: false,
            origin: CapExpr::constant(0),
            alloc_extent: extent,
            slab_offset: 0,
            from_stat_kernel: false,
        },
        SlabBinding {
            arg: "q_out".into(),
            slab_id: "slab_shared".into(),
            elem: ElemType::F32,
            extents: vec![extent],
            strides: vec![1],
            stride_unit: StrideUnit::Elements,
            stride_signed: false,
            origin: CapExpr::constant(0),
            alloc_extent: extent,
            slab_offset: 0,
            from_stat_kernel: false,
        },
    ];
    let outputs = vec![DeclaredOutput {
        arg: "q_out".into(),
        slab_id: "slab_shared".into(),
        lo: 0,
        hi,
    }];
    let err = m
        .validate_binding(&bindings, &class, BackendLimits::batch_cpu(), &outputs, &[])
        .expect_err("undeclared alias must be refused");
    assert!(matches!(
        err,
        ManifestError::UndeclaredAlias { ref kernel, ref slab, .. }
        if kernel == "K-synthetic-quat" && slab == "slab_shared"
    ));

    // Binding rule 3: writing an undeclared output slab is also refused.
    assert!(matches!(
        m.validate_binding(&bindings, &class, BackendLimits::batch_cpu(), &[], &[]),
        Err(ManifestError::UndeclaredAlias { .. })
    ));
}

#[test]
fn declared_alias_accepted() {
    let mut m = synthetic_quat_manifest();
    m.signature.may_alias.push(("q_in".into(), "q_out".into()));
    let class = ShapeClassCaps::default_classes().remove(0);
    let extent = CapExpr::row_cap(4);
    let hi = extent.eval(&class).unwrap();
    let bindings = vec![
        SlabBinding {
            arg: "q_in".into(),
            slab_id: "slab_shared".into(),
            elem: ElemType::F32,
            extents: vec![extent],
            strides: vec![1],
            stride_unit: StrideUnit::Elements,
            stride_signed: false,
            origin: CapExpr::constant(0),
            alloc_extent: extent,
            slab_offset: 0,
            from_stat_kernel: false,
        },
        SlabBinding {
            arg: "q_out".into(),
            slab_id: "slab_shared".into(),
            elem: ElemType::F32,
            extents: vec![extent],
            strides: vec![1],
            stride_unit: StrideUnit::Elements,
            stride_signed: false,
            origin: CapExpr::constant(0),
            alloc_extent: extent,
            slab_offset: 0,
            from_stat_kernel: false,
        },
    ];
    let outputs = vec![DeclaredOutput {
        arg: "q_out".into(),
        slab_id: "slab_shared".into(),
        lo: 0,
        hi,
    }];
    m.validate_binding(&bindings, &class, BackendLimits::batch_cpu(), &outputs, &[])
        .expect("declared alias must be accepted");

    // Binding rule 4: epoch conflict refused when unordered, accepted when ordered_before.
    let bad_epoch = vec![StageWriteEpoch {
        slab_id: "slab_shared".into(),
        epoch_write: 0,
        ordered_before: false,
    }];
    assert!(matches!(
        m.validate_binding(
            &bindings,
            &class,
            BackendLimits::batch_cpu(),
            &outputs,
            &bad_epoch
        ),
        Err(ManifestError::ClassMismatch { .. })
    ));
    let ok_epoch = vec![StageWriteEpoch {
        slab_id: "slab_shared".into(),
        epoch_write: 0,
        ordered_before: true,
    }];
    m.validate_binding(
        &bindings,
        &class,
        BackendLimits::batch_cpu(),
        &outputs,
        &ok_epoch,
    )
    .expect("ordered prior write accepted");
}

#[test]
fn cross_world_footprint_refused() {
    let mut m = synthetic_quat_manifest();
    m.signature.reads[0].lanes = LaneScope::AllWorlds;
    let classes = ShapeClassCaps::default_classes();
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::CrossWorldFootprint { .. })
    ));

    // Same AllWorlds footprint on a Stat kernel with feeds_state = false is accepted.
    m.class = KernelClass::Stat;
    m.signature.feeds_state = false;
    m.validate_static(&classes, None, false)
        .expect("Stat with AllWorlds accepted");

    // Stat kernel claiming feeds_state = true is refused.
    m.signature.feeds_state = true;
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::CrossWorldFootprint { .. })
    ));
}

#[test]
fn stat_output_cannot_feed_state() {
    let m = synthetic_quat_manifest();
    let class = ShapeClassCaps::default_classes().remove(0);
    let extent = CapExpr::row_cap(4);
    let hi = extent.eval(&class).unwrap();
    let bindings = vec![
        SlabBinding {
            arg: "q_in".into(),
            slab_id: "stat_slab".into(),
            elem: ElemType::F32,
            extents: vec![extent],
            strides: vec![1],
            stride_unit: StrideUnit::Elements,
            stride_signed: false,
            origin: CapExpr::constant(0),
            alloc_extent: extent,
            slab_offset: 0,
            from_stat_kernel: true,
        },
        SlabBinding {
            arg: "q_out".into(),
            slab_id: "state_out".into(),
            elem: ElemType::F32,
            extents: vec![extent],
            strides: vec![1],
            stride_unit: StrideUnit::Elements,
            stride_signed: false,
            origin: CapExpr::constant(0),
            alloc_extent: extent,
            slab_offset: 0,
            from_stat_kernel: false,
        },
    ];
    let outputs = vec![DeclaredOutput {
        arg: "q_out".into(),
        slab_id: "state_out".into(),
        lo: 0,
        hi,
    }];
    assert!(matches!(
        m.validate_binding(&bindings, &class, BackendLimits::batch_cpu(), &outputs, &[]),
        Err(ManifestError::CrossWorldFootprint { .. })
    ));
}

#[test]
fn exact_class_requires_u32() {
    let mut m = synthetic_quat_manifest();
    m.class = KernelClass::Exact;
    m.determinism = DetClass::D0Exact;
    m.signature.numerics.precision = PrecisionProfile::ExactU32;
    let classes = ShapeClassCaps::default_classes();
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::ClassMismatch { .. })
    ));
}

#[test]
fn exact_reads_float_bits() {
    let extent = CapExpr::row_cap(4);
    let mut m = KernelManifest {
        id: zero_id(),
        name: "K-synthetic-x-digest".into(),
        class: KernelClass::Exact,
        role: KernelRole::Digest,
        args: vec![
            ArgDecl {
                id: "state_f32".into(),
                elem: ElemType::F32,
                view: ArgView::Bits,
                shape: vec![extent],
                dim: [0; 6],
                kind: ElemKind::Quaternion,
                components: vec!["w".into(), "x".into(), "y".into(), "z".into()],
                state: true,
            },
            ArgDecl {
                id: "digest_words".into(),
                elem: ElemType::U32,
                view: ArgView::Value,
                shape: vec![CapExpr::constant(8)],
                dim: [0; 6],
                kind: ElemKind::Integer,
                components: vec!["word".into()],
                state: false,
            },
        ],
        determinism: DetClass::D0Exact,
        identities: vec![],
        derivative: None,
        cpu_reference: "frob_kernels::cpu::synthetic_digest".into(),
        signature: StageSignature {
            reads: vec![Footprint {
                arg: "state_f32".into(),
                lanes: LaneScope::OwnWorld,
                lo: CapExpr::constant(0),
                hi: extent,
            }],
            writes: vec![Footprint {
                arg: "digest_words".into(),
                lanes: LaneScope::OwnWorld,
                lo: CapExpr::constant(0),
                hi: CapExpr::constant(8),
            }],
            may_alias: vec![],
            layout: vec![
                ArgLayout {
                    arg: "state_f32".into(),
                    elem: ElemType::F32,
                    extents: vec![extent],
                    strides: vec![1],
                    stride_unit: StrideUnit::Elements,
                    stride_signed: false,
                    origin: CapExpr::constant(0),
                    alloc_extent: extent,
                },
                ArgLayout {
                    arg: "digest_words".into(),
                    elem: ElemType::U32,
                    extents: vec![CapExpr::constant(8)],
                    strides: vec![1],
                    stride_unit: StrideUnit::Elements,
                    stride_signed: false,
                    origin: CapExpr::constant(0),
                    alloc_extent: CapExpr::constant(8),
                },
            ],
            epoch_read: -1,
            epoch_write: 0,
            resources: ResourceBounds {
                live_values: 16,
                stack_bytes: 0,
                max_loop_bound: 64,
                buffer_args: 2,
            },
            numerics: NumericalContract {
                precision: PrecisionProfile::ExactU32,
                error_bound: None,
                deferral_threshold: None,
                exact_fallback: None,
                neutrals: vec![],
                padded_reduction_outputs: vec![],
            },
            feeds_state: false,
            footprint_evidence: FootprintEvidence::Lint,
        },
    }
    .seal()
    .unwrap();

    let classes = ShapeClassCaps::default_classes();
    m.validate_static(&classes, None, false)
        .expect("Exact with Bits view validates");

    // Binds directly to an f32 slab with zero copy.
    let class = &classes[0];
    let bindings = vec![
        SlabBinding {
            arg: "state_f32".into(),
            slab_id: "state_slab".into(),
            elem: ElemType::F32,
            extents: vec![extent],
            strides: vec![1],
            stride_unit: StrideUnit::Elements,
            stride_signed: false,
            origin: CapExpr::constant(0),
            alloc_extent: extent,
            slab_offset: 0,
            from_stat_kernel: false,
        },
        SlabBinding {
            arg: "digest_words".into(),
            slab_id: "digest_slab".into(),
            elem: ElemType::U32,
            extents: vec![CapExpr::constant(8)],
            strides: vec![1],
            stride_unit: StrideUnit::Elements,
            stride_signed: false,
            origin: CapExpr::constant(0),
            alloc_extent: CapExpr::constant(8),
            slab_offset: 0,
            from_stat_kernel: false,
        },
    ];
    let outputs = vec![DeclaredOutput {
        arg: "digest_words".into(),
        slab_id: "digest_slab".into(),
        lo: 0,
        hi: 8,
    }];
    m.validate_binding(&bindings, class, BackendLimits::batch_cpu(), &outputs, &[])
        .expect("f32 slab binds to f32 Bits arg without copy");

    // Removing the Bits view fails with ClassMismatch.
    m.args[0].view = ArgView::Value;
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::ClassMismatch { .. })
    ));

    // Role Digest with class Float also refused (FROB-REF-252 rule).
    m.class = KernelClass::Float;
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::ClassMismatch { .. })
    ));
}

#[test]
fn guarded_requires_bound_and_fallback() {
    let classes = ShapeClassCaps::default_classes();
    let mut m = synthetic_quat_manifest();
    m.class = KernelClass::Guarded;
    m.determinism = DetClass::D0Exact;
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::ClassMismatch { .. })
    ));

    m.signature.numerics.error_bound = Some(1e-6);
    m.signature.numerics.deferral_threshold = Some(1e-5);
    // Missing exact_fallback still fails.
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::ClassMismatch { .. })
    ));

    // tau < eps fails.
    m.signature.numerics.exact_fallback = Some("K-exact-fallback".into());
    m.signature.numerics.deferral_threshold = Some(1e-7);
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::ClassMismatch { .. })
    ));

    // Valid Guarded manifest passes.
    m.signature.numerics.deferral_threshold = Some(1e-5);
    m.validate_static(&classes, None, false)
        .expect("valid Guarded passes");
}

#[test]
fn neutral_elements_checked() {
    let classes = ShapeClassCaps::default_classes();
    let mut m = synthetic_quat_manifest();
    m.signature.numerics.padded_reduction_outputs = vec![("q_out".into(), ReductionConsumer::Sum)];

    // Missing neutral fails.
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::NeutralMissing { .. })
    ));

    // +0.0 for sum fails (must be -0.0).
    m.signature.numerics.neutrals = vec![NeutralSpec {
        arg: "q_out".into(),
        consumer: ReductionConsumer::Sum,
        value_bits: Some(u64::from((0.0_f32).to_bits())),
        identity_rows: false,
    }];
    assert!(matches!(
        m.validate_static(&classes, None, false),
        Err(ManifestError::NeutralMissing { .. })
    ));

    // -0.0 for sum passes.
    m.signature.numerics.neutrals[0].value_bits = Some(u64::from((-0.0_f32).to_bits()));
    m.validate_static(&classes, None, false)
        .expect("-0.0 sum neutral accepted");
}

#[test]
fn stride_convention_2x2() {
    // KA-532 / CX-218: 2x2 f32 view has byte strides (8, 4) vs element strides (2, 1).
    let buf: Vec<u32> = (0..16).collect();
    let class = ShapeClassCaps::default_classes().remove(0);

    let elem_layout = ArgLayout {
        arg: "mat".into(),
        elem: ElemType::F32,
        extents: vec![CapExpr::constant(2), CapExpr::constant(2)],
        strides: vec![2, 1],
        stride_unit: StrideUnit::Elements,
        stride_signed: false,
        origin: CapExpr::constant(0),
        alloc_extent: CapExpr::constant(16),
    };
    let idx_elem = elem_layout.element_offset(&[1, 1], &class).unwrap();
    assert_eq!(
        idx_elem, 3,
        "KA-532: element strides (2, 1) at [1, 1] address element 3"
    );
    assert_eq!(buf[idx_elem], 3);

    // Planted mutant (CX-218): ignoring stride unit and copying byte strides (8, 4) verbatim as element strides.
    let mutant_idx = (1 * 8 + 1 * 4) as usize;
    assert_eq!(
        buf[mutant_idx], 12,
        "CX-218 mutant reads element 12 in bounds"
    );

    let m = KernelManifest {
        id: zero_id(),
        name: "K-synthetic-2x2".into(),
        class: KernelClass::Float,
        role: KernelRole::Ordinary,
        args: vec![ArgDecl {
            id: "mat".into(),
            elem: ElemType::F32,
            view: ArgView::Value,
            shape: vec![CapExpr::constant(2), CapExpr::constant(2)],
            dim: [0; 6],
            kind: ElemKind::Scalar,
            components: vec!["v".into()],
            state: false,
        }],
        determinism: DetClass::D2PerBackend,
        identities: vec![],
        derivative: None,
        cpu_reference: "frob_kernels::cpu::synthetic_2x2".into(),
        signature: StageSignature {
            reads: vec![Footprint {
                arg: "mat".into(),
                lanes: LaneScope::OwnWorld,
                lo: CapExpr::constant(0),
                hi: CapExpr::constant(4),
            }],
            writes: vec![],
            may_alias: vec![],
            layout: vec![elem_layout],
            epoch_read: -1,
            epoch_write: 0,
            resources: ResourceBounds {
                live_values: 4,
                stack_bytes: 0,
                max_loop_bound: 4,
                buffer_args: 1,
            },
            numerics: NumericalContract {
                precision: PrecisionProfile::F32,
                error_bound: None,
                deferral_threshold: None,
                exact_fallback: None,
                neutrals: vec![],
                padded_reduction_outputs: vec![],
            },
            feeds_state: false,
            footprint_evidence: FootprintEvidence::Lint,
        },
    }
    .seal()
    .unwrap();

    // Byte-strided slab (8, 4) refused with LayoutMismatch naming both layouts.
    let byte_binding = vec![SlabBinding {
        arg: "mat".into(),
        slab_id: "s0".into(),
        elem: ElemType::F32,
        extents: vec![CapExpr::constant(2), CapExpr::constant(2)],
        strides: vec![8, 4],
        stride_unit: StrideUnit::Bytes,
        stride_signed: true,
        origin: CapExpr::constant(0),
        alloc_extent: CapExpr::constant(16),
        slab_offset: 0,
        from_stat_kernel: false,
    }];
    let err = m
        .validate_binding(&byte_binding, &class, BackendLimits::batch_cpu(), &[], &[])
        .expect_err("byte strides (8, 4) vs element strides (2, 1) must be refused");
    assert!(matches!(
        err,
        ManifestError::LayoutMismatch { ref expected, ref observed, .. }
        if expected.contains("Elements") && observed.contains("Bytes")
    ));

    // Element-strided slab (2, 1) binds cleanly.
    let elem_binding = vec![SlabBinding {
        arg: "mat".into(),
        slab_id: "s0".into(),
        elem: ElemType::F32,
        extents: vec![CapExpr::constant(2), CapExpr::constant(2)],
        strides: vec![2, 1],
        stride_unit: StrideUnit::Elements,
        stride_signed: false,
        origin: CapExpr::constant(0),
        alloc_extent: CapExpr::constant(16),
        slab_offset: 0,
        from_stat_kernel: false,
    }];
    m.validate_binding(&elem_binding, &class, BackendLimits::batch_cpu(), &[], &[])
        .expect("matching element-strided view binds");
}

#[test]
fn metal_argument_limit() {
    let class = ShapeClassCaps::default_classes().remove(0);
    for (count, metal_ok) in [(31usize, true), (32usize, false)] {
        let mut args = Vec::with_capacity(count);
        let mut reads = Vec::with_capacity(count);
        let mut layout = Vec::with_capacity(count);
        let mut bindings = Vec::with_capacity(count);
        for i in 0..count {
            let name = format!("a{i}");
            let ext = CapExpr::constant(4);
            args.push(ArgDecl {
                id: name.clone(),
                elem: ElemType::F32,
                view: ArgView::Value,
                shape: vec![ext],
                dim: [0; 6],
                kind: ElemKind::Scalar,
                components: vec!["x".into()],
                state: false,
            });
            reads.push(Footprint {
                arg: name.clone(),
                lanes: LaneScope::OwnWorld,
                lo: CapExpr::constant(0),
                hi: ext,
            });
            layout.push(ArgLayout {
                arg: name.clone(),
                elem: ElemType::F32,
                extents: vec![ext],
                strides: vec![1],
                stride_unit: StrideUnit::Elements,
                stride_signed: false,
                origin: CapExpr::constant(0),
                alloc_extent: ext,
            });
            bindings.push(SlabBinding {
                arg: name,
                slab_id: format!("slab_{i}"),
                elem: ElemType::F32,
                extents: vec![ext],
                strides: vec![1],
                stride_unit: StrideUnit::Elements,
                stride_signed: false,
                origin: CapExpr::constant(0),
                alloc_extent: ext,
                slab_offset: 0,
                from_stat_kernel: false,
            });
        }
        let m = KernelManifest {
            id: zero_id(),
            name: format!("K-synthetic-args-{count}"),
            class: KernelClass::Float,
            role: KernelRole::Ordinary,
            args,
            determinism: DetClass::D2PerBackend,
            identities: vec![],
            derivative: None,
            cpu_reference: "frob_kernels::cpu::synthetic_args".into(),
            signature: StageSignature {
                reads,
                writes: vec![],
                may_alias: vec![],
                layout,
                epoch_read: -1,
                epoch_write: 0,
                resources: ResourceBounds {
                    live_values: 8,
                    stack_bytes: 0,
                    max_loop_bound: 4,
                    buffer_args: count as u32,
                },
                numerics: NumericalContract {
                    precision: PrecisionProfile::F32,
                    error_bound: None,
                    deferral_threshold: None,
                    exact_fallback: None,
                    neutrals: vec![],
                    padded_reduction_outputs: vec![],
                },
                feeds_state: false,
                footprint_evidence: FootprintEvidence::Lint,
            },
        }
        .seal()
        .unwrap();

        // Always accepted on batch-cpu.
        m.validate_binding(&bindings, &class, BackendLimits::batch_cpu(), &[], &[])
            .expect("batch-cpu accepts 31 and 32 args");
        // Metal accepts 31 and refuses 32.
        let metal_res = m.validate_binding(&bindings, &class, BackendLimits::metal(), &[], &[]);
        assert_eq!(
            metal_res.is_ok(),
            metal_ok,
            "Metal arg limit for {count} args"
        );
    }
}

#[test]
fn load_store_containment() {
    let classes = ShapeClassCaps::default_classes();
    let m = synthetic_quat_manifest();
    let ok_prog = ProgramFootprint {
        reads: vec![Footprint {
            arg: "q_in".into(),
            lanes: LaneScope::OwnWorld,
            lo: CapExpr::constant(0),
            hi: CapExpr::row_cap(4),
        }],
        writes: vec![Footprint {
            arg: "q_out".into(),
            lanes: LaneScope::OwnWorld,
            lo: CapExpr::constant(0),
            hi: CapExpr::row_cap(4),
        }],
        resources: m.signature.resources,
    };
    m.validate_static(&classes, Some(&ok_prog), false)
        .expect("contained IR passes");

    // Store one element beyond declared write footprint is refused (PO-SPARK-04).
    let mut bad_prog = ok_prog.clone();
    bad_prog.writes[0].hi = CapExpr {
        constant: 1,
        row_cap: 4,
        ..CapExpr::constant(0)
    };
    assert!(matches!(
        m.validate_static(&classes, Some(&bad_prog), false),
        Err(ManifestError::LoadStoreNotContained { .. })
    ));

    // Narrowing declared manifest write by 1 element refuses the original program.
    let mut narrowed = m.clone();
    narrowed.signature.writes[0].hi = CapExpr::row_cap(3);
    assert!(matches!(
        narrowed.validate_static(&classes, Some(&ok_prog), false),
        Err(ManifestError::LoadStoreNotContained { .. })
    ));
}

#[test]
fn po_spark_04_random_containment_property() {
    let classes = ShapeClassCaps::default_classes();
    let mut rng = 0x5041_524B_0004_0001u64;
    let mut next = || {
        rng = rng.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        rng >> 32
    };
    for _ in 0..512 {
        let d_lo = next() % 8;
        let d_hi = d_lo + 1 + (next() % 16);
        let p_lo = next() % 12;
        let p_hi = p_lo + 1 + (next() % 16);
        let alloc = 64u64;
        let mut m = synthetic_quat_manifest();
        m.signature.layout[0].alloc_extent = CapExpr::constant(alloc);
        m.signature.layout[1].alloc_extent = CapExpr::constant(alloc);
        m.signature.reads[0].lo = CapExpr::constant(d_lo);
        m.signature.reads[0].hi = CapExpr::constant(d_hi);
        m.signature.writes[0].lo = CapExpr::constant(0);
        m.signature.writes[0].hi = CapExpr::constant(4);

        let prog = ProgramFootprint {
            reads: vec![Footprint {
                arg: "q_in".into(),
                lanes: LaneScope::OwnWorld,
                lo: CapExpr::constant(p_lo),
                hi: CapExpr::constant(p_hi),
            }],
            writes: vec![Footprint {
                arg: "q_out".into(),
                lanes: LaneScope::OwnWorld,
                lo: CapExpr::constant(0),
                hi: CapExpr::constant(4),
            }],
            resources: m.signature.resources,
        };
        let expected_ok = d_lo <= p_lo && p_hi <= d_hi;
        let got_ok = m.validate_static(&classes, Some(&prog), false).is_ok();
        assert_eq!(
            got_ok, expected_ok,
            "PO-SPARK-04 mismatch for decl=[{d_lo},{d_hi}) prog=[{p_lo},{p_hi})"
        );
    }
}

#[test]
fn doctor_kernels_section_real_and_planted() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf();
    let real_report = DoctorKernelsReport::check_registry_dir(&repo_root.join("registries"));
    assert_eq!(
        real_report.exit_code, 0,
        "real registries/kernels.toml must pass: {real_report:?}"
    );
    assert!(real_report.findings.is_empty());
    let json = real_report.to_canonical_json().expect("canonical json");
    assert!(json.as_str().contains("\"exit_code\":0"));

    // Planted bad manifest in a scratch directory under TMPDIR (never deleted per owner rule).
    let scratch = std::env::temp_dir().join(format!(
        "frob-AzureCarp-doctor-kernels-{}",
        hash_blob(b"n21b-01-planted")
            .to_hex()
            .get(..12)
            .unwrap_or("0")
    ));
    fs::create_dir_all(&scratch).expect("create scratch dir");
    let mut bad = synthetic_quat_manifest();
    bad.signature.reads[0].lanes = LaneScope::AllWorlds;
    let bad = bad.seal().unwrap();
    fs::write(
        scratch.join("bad.json"),
        bad.to_canonical_json().unwrap().as_bytes(),
    )
    .unwrap();
    let bad_toml = format!(
        r#"schema = "frob.kernels/1"
[[kernel]]
name = "K-synthetic-quat"
manifest = "bad.json"
id = "{}"
origin = {{ kind = "skeleton" }}
"#,
        bad.id.to_hex()
    );
    fs::write(scratch.join("kernels.toml"), bad_toml).unwrap();

    let bad_report = DoctorKernelsReport::check_registry_dir(&scratch);
    assert_eq!(bad_report.exit_code, 2);
    assert_eq!(bad_report.findings.len(), 1);
    assert_eq!(bad_report.findings[0].kernel, "K-synthetic-quat");
    assert_eq!(bad_report.findings[0].error, "CrossWorldFootprint");
}
