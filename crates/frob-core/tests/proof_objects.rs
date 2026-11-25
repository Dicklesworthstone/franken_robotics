//! Unit and property tests for `frob-core::proof` (`N25b-01`, KA-624, CX-108).
//! No mocks; pure data, exact rationals/intervals, deterministic D0 checks.

use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::evidence::EvidenceClass;
use frob_core::id::hash_blob;
use frob_core::proof::{
    AliasPairPayload, AliasScope, Assumption, AssumptionValue, Axis, Binder, Bound, BoundEnd,
    BoxOutcome, Cell, CellEndForces, CellPolarity, CellRef, ChartId, Comparator,
    CompositionPayload, ConicFarkasPayload, DualVector, Equation, Establishes, ExactQty, Expected,
    ExprBuilder, ExprDag, ExprNode, FieldPath, FiniteGamePayload, FirstMoment, FrameSpec, Horizon,
    ImplicitDep, Interval, IntervalBoxPayload, LedgerEntry, LoadVertexWitness, LpFarkasPayload,
    LpLaw, LpPrimalPayload, ModeId, MonotoneTransportPayload, NodeId, OrderedPorts, PartitionCell,
    PartitionPayload, Polarity, PolicyClassDigest, PolicyScope, PopulationId, PortForceVector,
    PortId, ProductUnreachablePayload, ProofError, ProofKind, ProofObject, ProofPayload, ProofRef,
    Ratio, SampleCoveragePayload, SosGramPayload, SosMultiplier, StageLink, Support,
    SymmetryTransportPayload, TaylorBoxPayload, TopologyId, TorqueVec, TraceWitnessPayload, Unit,
    Var, vertex_force_values,
};
use frob_core::registry::toml as strict_toml;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("locate workspace root from CARGO_MANIFEST_DIR")
        .to_path_buf()
}

fn stance_prefix() -> Vec<Binder> {
    vec![
        Binder::Exists {
            var: Var::new("policy").unwrap(),
            scope: PolicyScope::PerNominalDesign,
        },
        Binder::ForAll { var: Var::new("world").unwrap() },
        Binder::ForAll { var: Var::new("execution").unwrap() },
    ]
}

fn build_ka624_lp_primal(port_names: [&str; 4]) -> ProofObject {
    let frame = FrameSpec::right_handed_z_up("base");
    let meters = Unit::meters();
    let newtons = Unit::newtons();
    let nm = Unit::newton_meters();

    let ports = OrderedPorts::new(
        port_names
            .iter()
            .map(|p| PortId::new(*p).unwrap())
            .collect(),
    )
    .unwrap();

    let x_half = Ratio::new(1, 3).unwrap();
    let y_half = Ratio::new(3, 14).unwrap();
    let contact_coords = vec![
        [
            ExactQty::new(x_half, meters.clone(), Some(frame.clone())),
            ExactQty::new(y_half, meters.clone(), Some(frame.clone())),
        ],
        [
            ExactQty::new(x_half, meters.clone(), Some(frame.clone())),
            ExactQty::new(y_half.neg_exact(), meters.clone(), Some(frame.clone())),
        ],
        [
            ExactQty::new(x_half.neg_exact(), meters.clone(), Some(frame.clone())),
            ExactQty::new(y_half, meters.clone(), Some(frame.clone())),
        ],
        [
            ExactQty::new(x_half.neg_exact(), meters.clone(), Some(frame.clone())),
            ExactQty::new(y_half.neg_exact(), meters, Some(frame.clone())),
        ],
    ];

    let cap = Ratio::from_int(160);
    let capacities = PortForceVector::new(
        ports.clone(),
        vec![cap, cap, cap, cap],
        newtons.clone(),
        frame.clone(),
    )
    .unwrap();

    let w_qty = ExactQty::new(Ratio::from_int(400), newtons.clone(), Some(frame.clone()));
    let vertex_specs: [(i128, i128, [i128; 4]); 4] = [
        (60, 30, [160, 130, 110, 0]),
        (60, -30, [130, 160, 0, 110]),
        (-60, 30, [110, 0, 160, 130]),
        (-60, -30, [0, 110, 130, 160]),
    ];

    let vertices = vertex_specs
        .iter()
        .map(|&(px, py, fs)| {
            let fm = FirstMoment::planar(
                Ratio::from_int(px),
                Ratio::from_int(py),
                nm.clone(),
                frame.clone(),
            );
            let torque = fm.to_support_torque_z_up();
            let forces = PortForceVector::new(
                ports.clone(),
                fs.iter().map(|&f| Ratio::from_int(f)).collect(),
                newtons.clone(),
                frame.clone(),
            )
            .unwrap();
            LoadVertexWitness {
                load_weight: w_qty.clone(),
                first_moment: fm,
                support_torque: torque,
                forces,
            }
        })
        .collect();

    let px_bound = Bound::new(
        Var::new("P_x").unwrap(),
        Ratio::from_int(-60),
        BoundEnd::Closed,
        Ratio::from_int(60),
        BoundEnd::Closed,
    )
    .unwrap();
    let py_bound = Bound::new(
        Var::new("P_y").unwrap(),
        Ratio::from_int(-30),
        BoundEnd::Closed,
        Ratio::from_int(30),
        BoundEnd::Closed,
    )
    .unwrap();

    let cell = Cell::new(
        TopologyId::new("quadruped.stance").unwrap(),
        ChartId::new("stance.load.moment").unwrap(),
        vec![px_bound.clone(), py_bound.clone()],
        vec![nm.clone(), nm],
    )
    .unwrap();

    let establishes = Establishes::new(
        stance_prefix(),
        Horizon::Static,
        PolicyClassDigest::new(hash_blob(b"ka-624-stance-adjustable")),
        cell,
    )
    .unwrap();

    let mut eb = ExprBuilder::new();
    let f0 = eb.var("f_pp", newtons.clone());
    let f1 = eb.var("f_pm", newtons.clone());
    let f2 = eb.var("f_mp", newtons.clone());
    let f3 = eb.var("f_mm", newtons.clone());
    let sum = eb.sum(vec![f0, f1, f2, f3]);
    let w = eb.var("W", newtons);
    let neg_w = eb.neg(w);
    let root = eb.sum(vec![sum, neg_w]);
    let eq = Equation::new("Σ f_i − W = 0", eb.build(root).unwrap()).unwrap();

    let payload = LpPrimalPayload {
        ports,
        contact_coords,
        capacities,
        authority: "independent-normal-force".to_owned(),
        observed_before_decision: vec!["W".to_owned(), "P".to_owned()],
        vertices,
        cell_end_forces: Vec::new(),
        law_class: "adjustable".to_owned(),
        law: LpLaw::MultilinearVertex {
            vars: vec![Var::new("P_x").unwrap(), Var::new("P_y").unwrap()],
        },
        bounds_used: vec![px_bound, py_bound],
        frame: frame.clone(),
    };

    let assumptions = vec![
        Assumption::new(
            "stance.weight",
            AssumptionValue::Ratio(Ratio::from_int(400)),
            Axis::World,
            Polarity::Shrink,
        )
        .unwrap(),
        Assumption::new(
            "actuator.normal_cap",
            AssumptionValue::Ratio(cap),
            Axis::Policy,
            Polarity::Grow,
        )
        .unwrap(),
    ];
    let support = Support::new(
        vec![
            FieldPath::new("stance.feet.xy").unwrap(),
            FieldPath::new("actuator.normal_cap").unwrap(),
        ],
        vec![ImplicitDep::GravityFrame(frame)],
    )
    .unwrap();

    ProofObject::new(
        ProofRef::new("p0").unwrap(),
        ProofKind::LpPrimal,
        CellRef::new("c0").unwrap(),
        establishes,
        ProofPayload::LpPrimal(payload),
        Vec::new(),
        vec![eq],
        assumptions,
        support,
        Expected::inner_certified(),
    )
    .unwrap()
}

#[test]
fn stance_lp_primal_object_builds() {
    let obj = build_ka624_lp_primal(["foot.pp", "foot.pm", "foot.mp", "foot.mm"]);

    assert_eq!(obj.id().as_str(), "p0");
    assert_eq!(obj.kind(), ProofKind::LpPrimal);
    assert_eq!(obj.kind().as_str(), "lp-primal");
    assert_eq!(obj.cell().as_str(), "c0");
    assert_eq!(obj.expects(), Expected::inner_certified());
    assert_eq!(obj.establishes().prefix().len(), 3);
    assert_eq!(obj.primitive_equations().len(), 1);
    assert_eq!(obj.assumptions().len(), 2);
    assert_eq!(obj.support().fields().len(), 2);
    assert_eq!(obj.support().implicit().len(), 1);

    let ProofPayload::LpPrimal(p) = obj.payload() else {
        panic!("expected LpPrimal payload");
    };
    let port_strs: Vec<&str> = p.ports.as_slice().iter().map(PortId::as_str).collect();
    assert_eq!(port_strs, vec!["foot.pp", "foot.pm", "foot.mp", "foot.mm"]);
    assert_eq!(p.authority, "independent-normal-force");
    assert_eq!(p.observed_before_decision, vec!["W", "P"]);
    assert_eq!(p.law_class, "adjustable");
    assert_eq!(
        p.law,
        LpLaw::MultilinearVertex {
            vars: vec![Var::new("P_x").unwrap(), Var::new("P_y").unwrap()],
        }
    );
    assert_eq!(p.vertices.len(), 4);
    assert_eq!(
        vertex_force_values(&p.vertices[0]),
        &[160, 130, 110, 0].map(Ratio::from_int)
    );
    assert_eq!(
        vertex_force_values(&p.vertices[1]),
        &[130, 160, 0, 110].map(Ratio::from_int)
    );
    assert_eq!(
        vertex_force_values(&p.vertices[2]),
        &[110, 0, 160, 130].map(Ratio::from_int)
    );
    assert_eq!(
        vertex_force_values(&p.vertices[3]),
        &[0, 110, 130, 160].map(Ratio::from_int)
    );
    assert_eq!(p.vertices[0].first_moment.x(), Ratio::from_int(60));
    assert_eq!(p.vertices[0].first_moment.y(), Ratio::from_int(30));
    assert_eq!(p.vertices[0].support_torque.x(), Ratio::from_int(30));
    assert_eq!(p.vertices[0].support_torque.y(), Ratio::from_int(-60));
    assert_eq!(p.vertices[0].support_torque.z(), Ratio::zero());
}

#[test]
fn stance_lp_farkas_object_builds() {
    let frame = FrameSpec::right_handed_z_up("base");
    let inv_m = Unit::new("m^-1").unwrap();
    let r1 = Ratio::one();
    let rm1 = Ratio::from_int(-1);
    let half = Ratio::new(1, 2).unwrap();

    let mut eb = ExprBuilder::new();
    let c90 = eb.constant(Ratio::from_int(90));
    let c15 = eb.constant(Ratio::from_int(15));
    let v = eb.var("v", inv_m.clone());
    let prod = eb.product(vec![c15, v]);
    let sum = eb.sum(vec![c90, prod]);
    let dag = eb.build(sum).unwrap();

    let bound = Bound::new(
        Var::new("v").unwrap(),
        Ratio::new(14, 3).unwrap(),
        BoundEnd::Open,
        Ratio::from_int(6),
        BoundEnd::Closed,
    )
    .unwrap();
    let cell = Cell::new(
        TopologyId::new("quadruped.stance").unwrap(),
        ChartId::new("stance.inv_half_width").unwrap(),
        vec![bound],
        vec![inv_m],
    )
    .unwrap();
    let establishes = Establishes::new(
        stance_prefix(),
        Horizon::Static,
        PolicyClassDigest::new(hash_blob(b"ka-624-stance-adjustable")),
        cell,
    )
    .unwrap();

    let payload = LpFarkasPayload {
        matrix_a: vec![
            vec![r1, r1, r1, r1],
            vec![r1, r1, rm1, rm1],
            vec![r1, rm1, r1, rm1],
        ],
        rhs_w: vec![
            Ratio::from_int(400),
            Ratio::from_int(180),
            Ratio::from_int(150),
        ],
        capacities: vec![Ratio::from_int(160); 4],
        dual_y: DualVector::Constant(vec![Ratio::zero(), half, half]),
        sign_proof: "90 + 15·v > 160 for all v > 14/3".to_owned(),
        sign_proof_dag: Some(dag.clone()),
        unit: Unit::newtons(),
        frame: frame.clone(),
    };

    let support = Support::new(
        vec![FieldPath::new("stance.inv_half_width").unwrap()],
        vec![ImplicitDep::GravityFrame(frame)],
    )
    .unwrap();
    let obj = ProofObject::new(
        ProofRef::new("p1").unwrap(),
        ProofKind::LpFarkas,
        CellRef::new("c1").unwrap(),
        establishes,
        ProofPayload::LpFarkas(payload),
        Vec::new(),
        vec![Equation::new("90 + 15·v > 160", dag).unwrap()],
        Vec::new(),
        support,
        Expected::excluded_certified(),
    )
    .unwrap();

    assert_eq!(obj.kind(), ProofKind::LpFarkas);
    assert_eq!(obj.expects(), Expected::excluded_certified());
    let b0 = &obj.establishes().cell().bounds()[0];
    assert_eq!(b0.lower(), Ratio::new(14, 3).unwrap());
    assert_eq!(b0.lower_end(), BoundEnd::Open);
    assert_eq!(b0.upper(), Ratio::from_int(6));
    assert_eq!(b0.upper_end(), BoundEnd::Closed);

    let ProofPayload::LpFarkas(p) = obj.payload() else {
        panic!("expected LpFarkas payload");
    };
    assert_eq!(p.matrix_a.len(), 3);
    assert_eq!(p.dual_y, DualVector::Constant(vec![Ratio::zero(), half, half]));
    assert_eq!(p.sign_proof, "90 + 15·v > 160 for all v > 14/3");
}

#[test]
fn first_moment_is_not_a_torque() {
    // Negative-control assertion: under `--features nc-torque-alias-first-moment`,
    // `TorqueVec` is aliased to `FirstMoment` so their `TypeId`s collide and this fails.
    assert_ne!(
        TypeId::of::<FirstMoment>(),
        TypeId::of::<TorqueVec>(),
        "FirstMoment and TorqueVec must be distinct Rust types (N25b-01 rule 2)"
    );

    // Right-handed +z up support torque mapping: P = (60, 30) N·m -> (30, -60, 0) N·m.
    let frame = FrameSpec::right_handed_z_up("base");
    let p = FirstMoment::planar(
        Ratio::from_int(60),
        Ratio::from_int(30),
        Unit::newton_meters(),
        frame,
    );
    let tau: TorqueVec = p.to_support_torque_z_up();
    assert_eq!(tau.x(), Ratio::from_int(30));
    assert_eq!(tau.y(), Ratio::from_int(-60));
    assert_eq!(tau.z(), Ratio::zero());

    // Confirm compile-fail doctests with pinned rustc error codes exist in qty_types.rs.
    let qty_src = fs::read_to_string(
        workspace_root().join("crates/frob-core/src/proof/qty_types.rs"),
    )
    .expect("read qty_types.rs");
    assert!(
        qty_src.contains("compile_fail,E0308"),
        "qty_types.rs must carry pinned compile_fail,E0308 doctest for FirstMoment -> TorqueVec"
    );
    assert!(
        qty_src.contains("compile_fail,E0451"),
        "qty_types.rs must carry pinned compile_fail,E0451 doctest for PortForceVector"
    );
}

#[test]
fn port_order_is_identity() {
    // CX-108: swapping two ports with identical numeric values yields a distinct canonical encoding.
    let a = build_ka624_lp_primal(["foot.pp", "foot.pm", "foot.mp", "foot.mm"]);
    let b = build_ka624_lp_primal(["foot.pm", "foot.pp", "foot.mp", "foot.mm"]);
    assert_ne!(a.canonical_bytes(), b.canonical_bytes());
    assert_ne!(a.canonical_digest(), b.canonical_digest());

    // Rebuilding with identical port order produces byte-identical encodings.
    let a2 = build_ka624_lp_primal(["foot.pp", "foot.pm", "foot.mp", "foot.mm"]);
    assert_eq!(a.canonical_bytes(), a2.canonical_bytes());
    assert_eq!(a.canonical_digest(), a2.canonical_digest());

    // Length mismatch between ordered ports and force values is rejected.
    let ports = OrderedPorts::new(vec![
        PortId::new("foot.pp").unwrap(),
        PortId::new("foot.pm").unwrap(),
    ])
    .unwrap();
    let err = PortForceVector::new(
        ports,
        vec![Ratio::from_int(160)],
        Unit::newtons(),
        FrameSpec::right_handed_z_up("base"),
    )
    .unwrap_err();
    assert_eq!(
        err,
        ProofError::PortLengthMismatch { expected: 2, observed: 1 }
    );
}

#[test]
fn rational_reduced_on_construction() {
    let r = Ratio::new(6, 28).unwrap();
    assert_eq!(r.numer(), 3);
    assert_eq!(r.denom(), 14);
    assert_eq!(r.to_string(), "3/14");

    let r_neg_both = Ratio::new(-6, -28).unwrap();
    assert_eq!(r_neg_both, r);

    let r_neg_den = Ratio::new(6, -28).unwrap();
    assert_eq!(r_neg_den.numer(), -3);
    assert_eq!(r_neg_den.denom(), 14);

    let z = Ratio::new(0, -42).unwrap();
    assert_eq!(z.numer(), 0);
    assert_eq!(z.denom(), 1);

    assert_eq!(Ratio::new(1, 0), Err(ProofError::ZeroDenominator));

    // Overflow-free ordering for large numerators/denominators.
    let big_a = Ratio::new(1_000_000_000_000_000_000_i128, 3).unwrap();
    let big_b = Ratio::new(1_000_000_000_000_000_001_i128, 3).unwrap();
    assert!(big_a < big_b);
}

#[test]
fn expr_digest_structural() {
    let u = Unit::dimensionless();

    // Build (a + b) * c in order 1: a, b, a+b, c, (a+b)*c.
    let mut b1 = ExprBuilder::new();
    let a1 = b1.var("a", u.clone());
    let bb1 = b1.var("b", u.clone());
    let s1 = b1.sum(vec![a1, bb1]);
    let c1 = b1.var("c", u.clone());
    let p1 = b1.product(vec![s1, c1]);
    let dag1 = b1.build(p1).unwrap();

    // Build (a + b) * c in order 2: dead node, c, b, a, dead sum, a+b, (a+b)*c.
    let mut b2 = ExprBuilder::new();
    let _dead = b2.constant(Ratio::from_int(999));
    let c2 = b2.var("c", u.clone());
    let bb2 = b2.var("b", u.clone());
    let a2 = b2.var("a", u.clone());
    let _dead_sum = b2.sum(vec![_dead, c2]);
    let s2 = b2.sum(vec![a2, bb2]);
    let p2 = b2.product(vec![s2, c2]);
    let dag2 = b2.build(p2).unwrap();

    assert_eq!(dag1.canonical_bytes(), dag2.canonical_bytes());
    assert_eq!(dag1.digest(), dag2.digest());

    // Changing constant 15 to 16 changes the digest.
    let make_linear = |k: i128| {
        let mut eb = ExprBuilder::new();
        let c90 = eb.constant(Ratio::from_int(90));
        let ck = eb.constant(Ratio::from_int(k));
        let v = eb.var("v", Unit::new("m^-1").unwrap());
        let prod = eb.product(vec![ck, v]);
        let sum = eb.sum(vec![c90, prod]);
        eb.build(sum).unwrap()
    };
    assert_ne!(make_linear(15).digest(), make_linear(16).digest());

    // Non-preceding child index (cycle/forward ref) is rejected.
    let bad = ExprDag::new(vec![ExprNode::Neg(NodeId(0))], NodeId(0));
    assert!(matches!(bad, Err(ProofError::InvalidExprDag { .. })));
}

#[test]
fn support_must_not_be_empty() {
    let base = build_ka624_lp_primal(["foot.pp", "foot.pm", "foot.mp", "foot.mm"]);

    // Negative-control assertion: under `--features nc-allow-empty-support`, this returns Ok and fails.
    let res = ProofObject::new(
        base.id().clone(),
        base.kind(),
        base.cell().clone(),
        base.establishes().clone(),
        base.payload().clone(),
        Vec::new(),
        base.primitive_equations().to_vec(),
        base.assumptions().to_vec(),
        Support::empty(),
        base.expects(),
    );
    assert_eq!(
        res,
        Err(ProofError::EmptySupport { kind: "lp-primal" }),
        "non-partition proof object with empty Support must be refused (N25b-01 rule 7)"
    );

    // `partition` is explicitly allowed to carry an empty support.
    let region = base.establishes().cell().clone();
    let part = PartitionPayload {
        region: region.clone(),
        sub_cells: vec![PartitionCell {
            cell: region,
            proof: ProofRef::new("p0").unwrap(),
        }],
    };
    let ok = ProofObject::new(
        ProofRef::new("p_part").unwrap(),
        ProofKind::Partition,
        CellRef::new("c_part").unwrap(),
        base.establishes().clone(),
        ProofPayload::Partition(part),
        vec![ProofRef::new("p0").unwrap()],
        Vec::new(),
        Vec::new(),
        Support::empty(),
        Expected::inner_certified(),
    );
    assert!(ok.is_ok(), "partition with empty Support must be accepted");
}

#[test]
fn trace_witness_certified_refused_and_verdict_rules_enforced() {
    let base = build_ka624_lp_primal(["foot.pp", "foot.pm", "foot.mp", "foot.mm"]);
    let trace_payload = ProofPayload::TraceWitness(TraceWitnessPayload {
        replay_address: "replay://episode/e0".to_owned(),
        trace_digest: hash_blob(b"trace-0"),
        monitor_verdict: "violated:tip-over".to_owned(),
        separates_design: None,
    });

    // AC 5: trace-witness cannot claim Certified (neither Excluded nor Inner).
    for bad_expected in [Expected::excluded_certified(), Expected::inner_certified()] {
        let err = ProofObject::new(
            ProofRef::new("p_tw").unwrap(),
            ProofKind::TraceWitness,
            CellRef::new("c_tw").unwrap(),
            base.establishes().clone(),
            trace_payload.clone(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            base.support().clone(),
            bad_expected,
        )
        .unwrap_err();
        assert_eq!(
            err,
            ProofError::ClassExceedsKindCap {
                kind: "trace-witness",
                requested: "Certified",
                max_allowed: "Tested",
            }
        );
    }

    // trace-witness at Tested succeeds.
    let ok_tw = ProofObject::new(
        ProofRef::new("p_tw").unwrap(),
        ProofKind::TraceWitness,
        CellRef::new("c_tw").unwrap(),
        base.establishes().clone(),
        trace_payload,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        base.support().clone(),
        Expected::excluded_tested(),
    );
    assert!(ok_tw.is_ok());

    // lp-primal cannot claim Excluded.
    let wrong_pol = ProofObject::new(
        base.id().clone(),
        ProofKind::LpPrimal,
        base.cell().clone(),
        base.establishes().clone(),
        base.payload().clone(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        base.support().clone(),
        Expected::excluded_certified(),
    )
    .unwrap_err();
    assert_eq!(
        wrong_pol,
        ProofError::VerdictKindMismatch {
            kind: "lp-primal",
            expected_polarity: "excluded",
        }
    );

    // Establishes refuses empty quantifier prefix (Rule 5: no default prefix).
    let empty_prefix = Establishes::new(
        Vec::new(),
        Horizon::Static,
        PolicyClassDigest::new(hash_blob(b"pi")),
        base.establishes().cell().clone(),
    );
    assert_eq!(empty_prefix, Err(ProofError::EmptyPrefix));
}

fn parse_mission_kinds_from_registry_doc(md: &str) -> BTreeSet<String> {
    let mut in_section_1 = false;
    let mut out = BTreeSet::new();
    for line in md.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## 1.") {
            in_section_1 = true;
            continue;
        }
        if in_section_1 && trimmed.starts_with("## ") {
            break;
        }
        if !in_section_1 || !trimmed.starts_with('|') {
            continue;
        }
        let cols: Vec<&str> = trimmed.split('|').map(str::trim).collect();
        // Table columns: ["", "`kind`", "Version", "Cap", "Tier", "Scope", ...]
        if cols.len() < 7 || cols[1] == "Kind" || cols[1].starts_with("---") {
            continue;
        }
        let kind = cols[1].trim_matches('`');
        let scope_tokens: Vec<&str> = cols[5].split(',').map(str::trim).collect();
        if scope_tokens.contains(&"mission") {
            out.insert(kind.to_owned());
        }
    }
    out
}

#[test]
fn proof_kinds_match_cert_kinds_registry_doc() {
    // AC 1: read docs/registries/cert-kinds.md as a plain file (never a code dependency on frob-cert).
    let doc_path = workspace_root().join("docs/registries/cert-kinds.md");
    let doc = fs::read_to_string(&doc_path).expect("read docs/registries/cert-kinds.md");
    let doc_mission_kinds = parse_mission_kinds_from_registry_doc(&doc);
    assert_eq!(
        doc_mission_kinds.len(),
        17,
        "frob.cert.kinds/1 must list 17 mission-scope kinds, got {doc_mission_kinds:?}"
    );

    let enum_kinds: BTreeSet<String> = ProofKind::ALL
        .iter()
        .map(|k| k.as_str().to_owned())
        .collect();
    assert_eq!(enum_kinds.len(), 15, "ProofKind::ALL must have 15 unique kinds");

    let pending_kinds: BTreeSet<String> = ProofKind::PENDING_MISSION_KINDS
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    assert_eq!(pending_kinds.len(), 2);

    // No pending kind may already be an enum variant.
    let overlap: Vec<_> = enum_kinds.intersection(&pending_kinds).collect();
    assert!(
        overlap.is_empty(),
        "PENDING_MISSION_KINDS overlaps ProofKind::ALL: {overlap:?}"
    );

    // Union must equal mission-scope rows of cert-kinds.md verbatim.
    let union: BTreeSet<String> = enum_kinds.union(&pending_kinds).cloned().collect();
    assert_eq!(
        union, doc_mission_kinds,
        "ProofKind::ALL U PENDING_MISSION_KINDS must equal mission-scope rows of cert-kinds.md"
    );

    // Every ProofKind round-trips through parse(as_str()).
    for &k in ProofKind::ALL {
        assert_eq!(ProofKind::parse(k.as_str()), Some(k));
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .expect("read dir")
        .map(|e| e.expect("dir entry"))
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let p = entry.path();
        if p.is_dir() {
            collect_rs_files(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

fn strip_comments_and_strings(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut i = 0usize;
    let mut block_depth = 0usize;
    while i < bytes.len() {
        if block_depth > 0 {
            if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                block_depth += 1;
                i += 2;
            } else if i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                block_depth -= 1;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            out.push('\n');
            continue;
        }
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            block_depth = 1;
            i += 2;
            continue;
        }
        if bytes[i] == b'"' {
            i += 1;
            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push(' ');
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

#[test]
fn no_float_fields_in_proof_source() {
    // AC 2: source scan over all crates/frob-core/src/proof/**/*.rs verifying zero f32/f64 tokens.
    let proof_dir = workspace_root().join("crates/frob-core/src/proof");
    let mut files = Vec::new();
    collect_rs_files(&proof_dir, &mut files);
    assert!(files.len() >= 20, "expected >= 20 proof source files, got {}", files.len());

    let mut offenders = Vec::new();
    for path in files {
        let src = fs::read_to_string(&path).expect("read proof source");
        let code = strip_comments_and_strings(&src);
        for tok in code.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
            if tok == "f32" || tok == "f64" {
                offenders.push(format!("{}: found `{tok}`", path.display()));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "floating-point types forbidden in frob-core::proof: {offenders:?}"
    );
}

#[test]
fn frob_core_no_new_deps_and_frob_cert_independent() {
    // AC 6: frob-core adds no dependencies; frob-cert has zero dependency on frob-core.
    let core_toml = fs::read(workspace_root().join("crates/frob-core/Cargo.toml"))
        .expect("read frob-core/Cargo.toml");
    let core_root = strict_toml::parse(&core_toml).expect("parse frob-core/Cargo.toml");
    let core_deps = core_root
        .get("dependencies")
        .and_then(|v| v.as_table())
        .expect("frob-core [dependencies]");
    let dep_names: BTreeSet<&str> = core_deps.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(
        dep_names,
        BTreeSet::from(["fs-blake3", "fs-qty"]),
        "frob-core must gain no new dependencies"
    );

    let cert_toml = fs::read(workspace_root().join("crates/frob-cert/Cargo.toml"))
        .expect("read frob-cert/Cargo.toml");
    let cert_root = strict_toml::parse(&cert_toml).expect("parse frob-cert/Cargo.toml");
    if let Some(cert_deps) = cert_root.get("dependencies").and_then(|v| v.as_table()) {
        assert!(
            cert_deps.get("frob-core").is_none(),
            "frob-cert must never depend on frob-core (rule L-9)"
        );
    }

    // AC 7: CONTRACT.md lists proof module under Outputs with class "none: data only".
    let contract_md = fs::read_to_string(workspace_root().join("crates/frob-core/CONTRACT.md"))
        .expect("read CONTRACT.md");
    let outputs_sec = contract_md
        .split("## Outputs")
        .nth(1)
        .and_then(|s| s.split("## ").next())
        .expect("CONTRACT.md ## Outputs section");
    assert!(
        outputs_sec.contains("proof") && outputs_sec.contains("none: data only"),
        "CONTRACT.md ## Outputs must list proof module with class \"none: data only\""
    );
}

/// Deterministic splitmix64 PRNG for zero-dependency property tests.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn ratio(&mut self, max_abs: i128) -> Ratio {
        let span = u64::try_from(max_abs).unwrap_or(100);
        let raw = i128::from(self.next_u64() % (2 * span + 1)) - max_abs;
        let den = i128::from((self.next_u64() % 17) + 1);
        Ratio::new(raw, den).unwrap()
    }

    fn unit_ratio(&mut self) -> Ratio {
        let den = i128::from((self.next_u64() % 20) + 1);
        let num = i128::from(self.next_u64() % u64::try_from(den + 1).unwrap());
        Ratio::new(num, den).unwrap()
    }

    fn interval(&mut self) -> Interval {
        let a = self.ratio(50);
        let b = self.ratio(50);
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        Interval::new(lo, hi, "frob.iv/mpfi", 64).unwrap()
    }

    fn bound(&mut self, name: &str) -> Bound {
        let a = self.ratio(25);
        let b = self.ratio(25);
        let (lo, hi) = if a < b {
            (a, b)
        } else if a > b {
            (b, a)
        } else {
            return Bound::new(
                Var::new(name).unwrap(),
                a,
                BoundEnd::Closed,
                a,
                BoundEnd::Closed,
            )
            .unwrap();
        };
        let lo_end = if self.next_u64() % 2 == 0 {
            BoundEnd::Closed
        } else {
            BoundEnd::Open
        };
        let hi_end = if self.next_u64() % 2 == 0 {
            BoundEnd::Closed
        } else {
            BoundEnd::Open
        };
        Bound::new(Var::new(name).unwrap(), lo, lo_end, hi, hi_end).unwrap()
    }
}

fn random_payload_and_expected(
    rng: &mut Rng,
    kind: ProofKind,
    cell: &Cell,
    dag: &ExprDag,
) -> (ProofPayload, Expected) {
    let frame = FrameSpec::right_handed_z_up("prop_base");
    let n_unit = Unit::newtons();
    let m_unit = Unit::meters();
    let nm_unit = Unit::newton_meters();
    match kind {
        ProofKind::LpPrimal => {
            let ports = OrderedPorts::new(vec![
                PortId::new("p.a").unwrap(),
                PortId::new("p.b").unwrap(),
            ])
            .unwrap();
            let f_a = rng.ratio(100);
            let f_b = rng.ratio(100);
            let fm = FirstMoment::planar(rng.ratio(40), rng.ratio(40), nm_unit, frame.clone());
            let tau = fm.to_support_torque_z_up();
            let vtx = LoadVertexWitness {
                load_weight: ExactQty::new(rng.ratio(200), n_unit.clone(), Some(frame.clone())),
                first_moment: fm,
                support_torque: tau,
                forces: PortForceVector::new(
                    ports.clone(),
                    vec![f_a, f_b],
                    n_unit.clone(),
                    frame.clone(),
                )
                .unwrap(),
            };
            let end = CellEndForces {
                chart_var: Var::new("s").unwrap(),
                lower_end_forces: PortForceVector::new(
                    ports.clone(),
                    vec![f_a, f_b],
                    n_unit.clone(),
                    frame.clone(),
                )
                .unwrap(),
                upper_end_forces: PortForceVector::new(
                    ports.clone(),
                    vec![f_b, f_a],
                    n_unit.clone(),
                    frame.clone(),
                )
                .unwrap(),
            };
            let p = LpPrimalPayload {
                ports: ports.clone(),
                contact_coords: vec![
                    [
                        ExactQty::new(rng.ratio(5), m_unit.clone(), None),
                        ExactQty::new(rng.ratio(5), m_unit.clone(), None),
                    ],
                    [
                        ExactQty::new(rng.ratio(5), m_unit.clone(), None),
                        ExactQty::new(rng.ratio(5), m_unit, None),
                    ],
                ],
                capacities: PortForceVector::new(
                    ports,
                    vec![Ratio::from_int(200), Ratio::from_int(200)],
                    n_unit,
                    frame.clone(),
                )
                .unwrap(),
                authority: "independent-normal-force".to_owned(),
                observed_before_decision: vec!["W".to_owned()],
                vertices: vec![vtx],
                cell_end_forces: vec![end],
                law_class: "adjustable".to_owned(),
                law: LpLaw::Affine { vars: vec![Var::new("s").unwrap()] },
                bounds_used: vec![rng.bound("s")],
                frame,
            };
            (ProofPayload::LpPrimal(p), Expected::inner_certified())
        }
        ProofKind::LpFarkas => {
            let p = LpFarkasPayload {
                matrix_a: vec![vec![rng.ratio(5), rng.ratio(5)]],
                rhs_w: vec![rng.ratio(50)],
                capacities: vec![Ratio::from_int(100), Ratio::from_int(100)],
                dual_y: DualVector::Constant(vec![rng.ratio(5)]),
                sign_proof: "y^T w > max_f".to_owned(),
                sign_proof_dag: Some(dag.clone()),
                unit: n_unit,
                frame,
            };
            (ProofPayload::LpFarkas(p), Expected::excluded_certified())
        }
        ProofKind::ConicFarkas => {
            let p = ConicFarkasPayload {
                dual_y: vec![rng.ratio(10), rng.ratio(10), rng.ratio(10)],
                cone_dims: vec![3],
                separation_margin: rng.ratio(10),
                unit: n_unit,
            };
            (ProofPayload::ConicFarkas(p), Expected::excluded_certified())
        }
        ProofKind::IntervalBox => {
            let holds = rng.next_u64() % 2 == 0;
            let (outcome, exp) = if holds {
                (BoxOutcome::Holds, Expected::inner_certified())
            } else {
                (BoxOutcome::Fails, Expected::excluded_certified())
            };
            let p = IntervalBoxPayload {
                expr_digest: dag.digest(),
                box_bounds: vec![rng.bound("x0")],
                split_set: vec![Var::new("x0").unwrap()],
                enclosure: rng.interval(),
                outcome,
            };
            (ProofPayload::IntervalBox(p), exp)
        }
        ProofKind::TaylorBox => {
            let holds = rng.next_u64() % 2 == 0;
            let (outcome, exp) = if holds {
                (BoxOutcome::Holds, Expected::inner_certified())
            } else {
                (BoxOutcome::Fails, Expected::excluded_certified())
            };
            let p = TaylorBoxPayload {
                expr_digest: dag.digest(),
                box_bounds: vec![rng.bound("x0")],
                split_set: vec![Var::new("x0").unwrap()],
                coefficients: vec![(vec![0], rng.ratio(20)), (vec![1], rng.ratio(20))],
                remainder: rng.interval(),
                enclosure: rng.interval(),
                outcome,
            };
            (ProofPayload::TaylorBox(p), exp)
        }
        ProofKind::SosGram => {
            let d = rng.ratio(10);
            let p = SosGramPayload {
                box_bounds: vec![rng.bound("x0")],
                monomial_basis: vec![vec![0], vec![1]],
                gram_matrix: vec![
                    vec![Ratio::one(), d],
                    vec![d, Ratio::from_int(2)],
                ],
                multipliers: vec![SosMultiplier {
                    var: Var::new("x0").unwrap(),
                    gram: vec![vec![Ratio::one()]],
                }],
            };
            (ProofPayload::SosGram(p), Expected::inner_certified())
        }
        ProofKind::FiniteGame => {
            let mut table = BTreeMap::new();
            table.insert(vec!["obs0".to_owned()], vec![rng.ratio(10)]);
            let p = FiniteGamePayload {
                policy_table: table,
                worlds: vec!["w0".to_owned(), "w1".to_owned()],
                error_vertices: vec![vec![rng.ratio(5)]],
                command_bounds: vec![rng.bound("u0")],
                terminal_predicate: Equation::new("g(x) >= 0", dag.clone()).unwrap(),
                comparator: Comparator::Ge,
            };
            (ProofPayload::FiniteGame(p), Expected::inner_certified())
        }
        ProofKind::AliasPair => {
            let p = AliasPairPayload {
                world_a: "w_a".to_owned(),
                world_b: "w_b".to_owned(),
                observation_set_a: vec!["pkt0".to_owned()],
                observation_set_b: vec!["pkt0".to_owned()],
                success_set_a: vec!["u_left".to_owned()],
                success_set_b: vec!["u_right".to_owned()],
                scope: AliasScope::AllAdmittedProbes,
            };
            (ProofPayload::AliasPair(p), Expected::excluded_certified())
        }
        ProofKind::MonotoneTransport => {
            let sign = if rng.next_u64() % 2 == 0 { 1 } else { -1 };
            let p = MonotoneTransportPayload {
                direction: vec![(Var::new("x0").unwrap(), sign)],
                sign_enclosures: vec![(Var::new("x0").unwrap(), rng.interval())],
                source_certificates: vec![ProofRef::new("p_src").unwrap()],
                model_stratum: "stratum.rigid".to_owned(),
            };
            (ProofPayload::MonotoneTransport(p), Expected::inner_certified())
        }
        ProofKind::SymmetryTransport => {
            let sign = if rng.next_u64() % 2 == 0 { 1 } else { -1 };
            let p = SymmetryTransportPayload {
                signed_permutation: vec![(1, sign), (0, -sign)],
                mission_invariant: "inv.mission".to_owned(),
                world_set_invariant: "inv.worlds".to_owned(),
                policy_class_invariant: "inv.policy".to_owned(),
                source_certificate: ProofRef::new("p_src").unwrap(),
            };
            (ProofPayload::SymmetryTransport(p), Expected::inner_certified())
        }
        ProofKind::Partition => {
            let p = PartitionPayload {
                region: cell.clone(),
                sub_cells: vec![PartitionCell {
                    cell: cell.clone(),
                    proof: ProofRef::new("p_sub0").unwrap(),
                }],
            };
            (ProofPayload::Partition(p), Expected::inner_certified())
        }
        ProofKind::Composition => {
            let p = CompositionPayload {
                stage_links: vec![StageLink {
                    from_stage: "approach".to_owned(),
                    to_stage: "grasp".to_owned(),
                    inclusion_proof: ProofRef::new("p_inc").unwrap(),
                }],
                clock_ledger: vec![LedgerEntry {
                    name: "wall".to_owned(),
                    spent: rng.unit_ratio(),
                    limit: Ratio::one(),
                    unit: Unit::seconds(),
                }],
                resource_ledger: vec![LedgerEntry {
                    name: "energy".to_owned(),
                    spent: rng.unit_ratio(),
                    limit: Ratio::one(),
                    unit: Unit::new("J").unwrap(),
                }],
            };
            (ProofPayload::Composition(p), Expected::inner_certified())
        }
        ProofKind::ProductUnreachable => {
            let p = ProductUnreachablePayload {
                outcome_a: "A.ok".to_owned(),
                outcome_b: "B.fail".to_owned(),
                joint_enclosure: vec![rng.interval()],
                coverage_argument: "disjoint boxes".to_owned(),
            };
            (ProofPayload::ProductUnreachable(p), Expected::excluded_certified())
        }
        ProofKind::TraceWitness => {
            let p = TraceWitnessPayload {
                replay_address: format!("replay://ep/{}", rng.next_u64() % 1000),
                trace_digest: hash_blob(&rng.next_u64().to_le_bytes()),
                monitor_verdict: "violated".to_owned(),
                separates_design: Some("d_alt".to_owned()),
            };
            (ProofPayload::TraceWitness(p), Expected::excluded_tested())
        }
        ProofKind::SampleCoverage => {
            let trials = (rng.next_u64() % 500) + 1;
            let successes = rng.next_u64() % (trials + 1);
            let p = SampleCoveragePayload {
                successes,
                trials,
                population: PopulationId::new("pop.kitchen").unwrap(),
                method: "clopper-pearson".to_owned(),
                bound_ratio: rng.unit_ratio(),
                confidence: rng.unit_ratio(),
            };
            (
                ProofPayload::SampleCoverage(p),
                Expected::new(CellPolarity::Inner, EvidenceClass::Estimated),
            )
        }
    }
}

#[test]
fn all_fifteen_kinds_property_roundtrip() {
    let mut rng = Rng(0x5EED_025B_0001_CAFE);

    for trial in 0..32u64 {
        for &kind in ProofKind::ALL {
            let b0 = rng.bound("x0");
            let cell = Cell::new(
                TopologyId::new(format!("top.{trial}")).unwrap(),
                ChartId::new(format!("chart.{trial}")).unwrap(),
                vec![b0],
                vec![Unit::meters()],
            )
            .unwrap();
            let horizon = match trial % 4 {
                0 => Horizon::Static,
                1 => Horizon::Time(ExactQty::new(rng.unit_ratio(), Unit::seconds(), None)),
                2 => Horizon::Ticks(trial + 1),
                _ => Horizon::TerminalModes(vec![ModeId::new("done").unwrap()]),
            };
            let establishes = Establishes::new(
                stance_prefix(),
                horizon,
                PolicyClassDigest::new(hash_blob(&trial.to_le_bytes())),
                cell.clone(),
            )
            .unwrap();

            let mut eb = ExprBuilder::new();
            let k_node = eb.constant(rng.ratio(30));
            let x_node = eb.var("x0", Unit::meters());
            let sum_node = eb.sum(vec![k_node, x_node]);
            let dag = eb.build(sum_node).unwrap();
            let eqs = vec![Equation::new(format!("eq_{trial}"), dag.clone()).unwrap()];

            let (payload, expected) = random_payload_and_expected(&mut rng, kind, &cell, &dag);
            let side_conditions = vec![ProofRef::new(format!("sc_{trial}")).unwrap()];
            let assumptions = vec![
                Assumption::new(
                    format!("a_{trial}"),
                    AssumptionValue::Ratio(rng.ratio(10)),
                    Axis::World,
                    Polarity::Shrink,
                )
                .unwrap(),
            ];
            let support = Support::new(
                vec![FieldPath::new(format!("param.{trial}")).unwrap()],
                vec![ImplicitDep::ModelVersion(format!("v{trial}"))],
            )
            .unwrap();
            let id = ProofRef::new(format!("p_{trial}_{}", kind.as_str())).unwrap();
            let cell_ref = CellRef::new(format!("c_{trial}")).unwrap();

            let obj = ProofObject::new(
                id.clone(),
                kind,
                cell_ref.clone(),
                establishes.clone(),
                payload.clone(),
                side_conditions.clone(),
                eqs.clone(),
                assumptions.clone(),
                support.clone(),
                expected,
            )
            .expect("valid random proof object must construct");

            // Every accessor returns the exact input unchanged and in order.
            assert_eq!(obj.id(), &id);
            assert_eq!(obj.kind(), kind);
            assert_eq!(obj.cell(), &cell_ref);
            assert_eq!(obj.establishes(), &establishes);
            assert_eq!(obj.payload(), &payload);
            assert_eq!(obj.side_conditions(), side_conditions.as_slice());
            assert_eq!(obj.primitive_equations(), eqs.as_slice());
            assert_eq!(obj.assumptions(), assumptions.as_slice());
            assert_eq!(obj.support(), &support);
            assert_eq!(obj.expects(), expected);

            // Canonical encoding and digest are deterministic.
            assert_eq!(obj.canonical_bytes(), obj.canonical_bytes());
            assert_eq!(obj.canonical_digest(), obj.canonical_digest());
        }
    }
}
