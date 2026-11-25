//! CLI example emitting the two KA-624 stance proof-object specimens (`lp-primal` and `lp-farkas`)
//! as deterministic canonical JSON (`N25b-01`).
//!
//! Usage:
//!   `cargo run -p frob-core --example proof_specimens -- --json [--swap-ports]`

use std::fmt::Write as _;
use std::process::ExitCode;

use frob_core::id::hash_blob;
use frob_core::proof::{
    Assumption, AssumptionValue, Axis, Binder, Bound, BoundEnd, Cell, CellRef, ChartId, DualVector,
    Equation, Establishes, ExactQty, Expected, ExprBuilder, FirstMoment, FieldPath, FrameSpec,
    Horizon, ImplicitDep, LoadVertexWitness, LpFarkasPayload, LpLaw, LpPrimalPayload, OrderedPorts,
    Polarity, PolicyClassDigest, PolicyScope, PortForceVector, PortId, ProofKind, ProofObject,
    ProofPayload, ProofRef, Ratio, Support, TopologyId, Unit, Var, vertex_force_values,
};

fn build_stance_lp_primal(swap_ports: bool) -> ProofObject {
    let frame = FrameSpec::right_handed_z_up("base");
    let meters = Unit::meters();
    let newtons = Unit::newtons();
    let nm = Unit::newton_meters();

    let port_names: [&str; 4] = if swap_ports {
        ["foot.pm", "foot.pp", "foot.mp", "foot.mm"]
    } else {
        ["foot.pp", "foot.pm", "foot.mp", "foot.mm"]
    };
    let ports = OrderedPorts::new(
        port_names
            .iter()
            .map(|p| PortId::new(*p).expect("port id"))
            .collect(),
    )
    .expect("ordered ports");

    let x_half = Ratio::new(1, 3).expect("1/3");
    let y_half = Ratio::new(3, 14).expect("3/14");
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

    let cap_160 = Ratio::from_int(160);
    let capacities = PortForceVector::new(
        ports.clone(),
        vec![cap_160, cap_160, cap_160, cap_160],
        newtons.clone(),
        frame.clone(),
    )
    .expect("capacities");

    let vertex_specs: [(i128, i128, [i128; 4]); 4] = [
        (60, 30, [160, 130, 110, 0]),
        (60, -30, [130, 160, 0, 110]),
        (-60, 30, [110, 0, 160, 130]),
        (-60, -30, [0, 110, 130, 160]),
    ];

    let weight_400 = ExactQty::new(Ratio::from_int(400), newtons.clone(), Some(frame.clone()));
    let vertices = vertex_specs
        .iter()
        .map(|&(px, py, fs)| {
            let fm = FirstMoment::planar(
                Ratio::from_int(px),
                Ratio::from_int(py),
                nm.clone(),
                frame.clone(),
            );
            let support_torque = fm.to_support_torque_z_up();
            let forces = PortForceVector::new(
                ports.clone(),
                fs.iter().map(|&f| Ratio::from_int(f)).collect(),
                newtons.clone(),
                frame.clone(),
            )
            .expect("vertex forces");
            LoadVertexWitness {
                load_weight: weight_400.clone(),
                first_moment: fm,
                support_torque,
                forces,
            }
        })
        .collect();

    let px_bound = Bound::new(
        Var::new("P_x").expect("var"),
        Ratio::from_int(-60),
        BoundEnd::Closed,
        Ratio::from_int(60),
        BoundEnd::Closed,
    )
    .expect("P_x bound");
    let py_bound = Bound::new(
        Var::new("P_y").expect("var"),
        Ratio::from_int(-30),
        BoundEnd::Closed,
        Ratio::from_int(30),
        BoundEnd::Closed,
    )
    .expect("P_y bound");

    let cell = Cell::new(
        TopologyId::new("quadruped.stance").expect("topology"),
        ChartId::new("stance.load.moment").expect("chart"),
        vec![px_bound.clone(), py_bound.clone()],
        vec![nm.clone(), nm],
    )
    .expect("cell");

    let prefix = vec![
        Binder::Exists {
            var: Var::new("policy").expect("var"),
            scope: PolicyScope::PerNominalDesign,
        },
        Binder::ForAll { var: Var::new("world").expect("var") },
        Binder::ForAll { var: Var::new("execution").expect("var") },
    ];
    let establishes = Establishes::new(
        prefix,
        Horizon::Static,
        PolicyClassDigest::new(hash_blob(b"ka-624-stance-adjustable")),
        cell,
    )
    .expect("establishes");

    let mut eb = ExprBuilder::new();
    let f0 = eb.var("f_pp", newtons.clone());
    let f1 = eb.var("f_pm", newtons.clone());
    let f2 = eb.var("f_mp", newtons.clone());
    let f3 = eb.var("f_mm", newtons.clone());
    let sum_f = eb.sum(vec![f0, f1, f2, f3]);
    let w = eb.var("W", newtons);
    let neg_w = eb.neg(w);
    let balance = eb.sum(vec![sum_f, neg_w]);
    let eq_weight = Equation::new(
        "f_pp + f_pm + f_mp + f_mm − W = 0",
        eb.build(balance).expect("dag"),
    )
    .expect("equation");

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
            vars: vec![Var::new("P_x").expect("var"), Var::new("P_y").expect("var")],
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
        .expect("assumption"),
        Assumption::new(
            "actuator.normal_cap",
            AssumptionValue::Ratio(cap_160),
            Axis::Policy,
            Polarity::Grow,
        )
        .expect("assumption"),
    ];
    let support = Support::new(
        vec![
            FieldPath::new("stance.feet.xy").expect("field"),
            FieldPath::new("actuator.normal_cap").expect("field"),
        ],
        vec![ImplicitDep::GravityFrame(frame)],
    )
    .expect("support");

    ProofObject::new(
        ProofRef::new("p0").expect("proof ref"),
        ProofKind::LpPrimal,
        CellRef::new("c0").expect("cell ref"),
        establishes,
        ProofPayload::LpPrimal(payload),
        Vec::new(),
        vec![eq_weight],
        assumptions,
        support,
        Expected::inner_certified(),
    )
    .expect("lp-primal proof object")
}

fn build_stance_lp_farkas() -> ProofObject {
    let frame = FrameSpec::right_handed_z_up("base");
    let newtons = Unit::newtons();
    let inv_m = Unit::new("m^-1").expect("unit");

    let r1 = Ratio::one();
    let rm1 = Ratio::from_int(-1);
    let matrix_a = vec![
        vec![r1, r1, r1, r1],
        vec![r1, r1, rm1, rm1],
        vec![r1, rm1, r1, rm1],
    ];
    let rhs_w = vec![
        Ratio::from_int(400),
        Ratio::from_int(180),
        Ratio::from_int(150),
    ];
    let cap_160 = Ratio::from_int(160);
    let capacities = vec![cap_160, cap_160, cap_160, cap_160];
    let half = Ratio::new(1, 2).expect("1/2");
    let dual_y = DualVector::Constant(vec![Ratio::zero(), half, half]);

    let mut eb = ExprBuilder::new();
    let c90 = eb.constant(Ratio::from_int(90));
    let c15 = eb.constant(Ratio::from_int(15));
    let v = eb.var("v", inv_m.clone());
    let term = eb.product(vec![c15, v]);
    let lhs = eb.sum(vec![c90, term]);
    let c160 = eb.constant(cap_160);
    let neg_160 = eb.neg(c160);
    let margin = eb.sum(vec![lhs, neg_160]);
    let sign_dag = eb.build(margin).expect("sign dag");

    let eq_dual = Equation::new("90 + 15·v − 160 > 0", sign_dag.clone()).expect("eq");

    let v_bound = Bound::new(
        Var::new("v").expect("var"),
        Ratio::new(14, 3).expect("14/3"),
        BoundEnd::Open,
        Ratio::from_int(6),
        BoundEnd::Closed,
    )
    .expect("v bound");

    let cell = Cell::new(
        TopologyId::new("quadruped.stance").expect("topology"),
        ChartId::new("stance.inv_half_width").expect("chart"),
        vec![v_bound],
        vec![inv_m],
    )
    .expect("cell");

    let prefix = vec![
        Binder::Exists {
            var: Var::new("policy").expect("var"),
            scope: PolicyScope::PerNominalDesign,
        },
        Binder::ForAll { var: Var::new("world").expect("var") },
        Binder::ForAll { var: Var::new("execution").expect("var") },
    ];
    let establishes = Establishes::new(
        prefix,
        Horizon::Static,
        PolicyClassDigest::new(hash_blob(b"ka-624-stance-adjustable")),
        cell,
    )
    .expect("establishes");

    let payload = LpFarkasPayload {
        matrix_a,
        rhs_w,
        capacities,
        dual_y,
        sign_proof: "90 + 15·v > 160 for all v > 14/3".to_owned(),
        sign_proof_dag: Some(sign_dag),
        unit: newtons,
        frame: frame.clone(),
    };

    let assumptions = vec![
        Assumption::new(
            "actuator.normal_cap",
            AssumptionValue::Ratio(cap_160),
            Axis::Policy,
            Polarity::Shrink,
        )
        .expect("assumption"),
    ];
    let support = Support::new(
        vec![
            FieldPath::new("stance.inv_half_width").expect("field"),
            FieldPath::new("actuator.normal_cap").expect("field"),
        ],
        vec![ImplicitDep::GravityFrame(frame)],
    )
    .expect("support");

    ProofObject::new(
        ProofRef::new("p1").expect("proof ref"),
        ProofKind::LpFarkas,
        CellRef::new("c1").expect("cell ref"),
        establishes,
        ProofPayload::LpFarkas(payload),
        Vec::new(),
        vec![eq_dual],
        assumptions,
        support,
        Expected::excluded_certified(),
    )
    .expect("lp-farkas proof object")
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn hex_bytes(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}

fn format_prefix(obj: &ProofObject) -> Vec<String> {
    obj.establishes()
        .prefix()
        .iter()
        .map(|b| match b {
            Binder::Exists { var, scope } => {
                format!("exists:{}:{}", var.as_str(), scope.as_str())
            }
            Binder::ForAll { var } => format!("forall:{}", var.as_str()),
            Binder::Prob { var, population, at_least, confidence } => {
                format!(
                    "prob:{}:{}:{at_least}:{confidence}",
                    var.as_str(),
                    population.as_str()
                )
            }
        })
        .collect()
}

fn format_string_array(items: &[String]) -> String {
    let mut out = String::from("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&json_escape(item));
    }
    out.push(']');
    out
}

fn format_cell_json(obj: &ProofObject) -> String {
    let cell = obj.establishes().cell();
    let bounds: Vec<String> = cell
        .bounds()
        .iter()
        .map(|b| {
            format!(
                "{{\"lower\":{},\"lower_end\":{},\"upper\":{},\"upper_end\":{},\"var\":{}}}",
                json_escape(&b.lower().to_string()),
                json_escape(b.lower_end().as_str()),
                json_escape(&b.upper().to_string()),
                json_escape(b.upper_end().as_str()),
                json_escape(b.var().as_str()),
            )
        })
        .collect();
    let units: Vec<String> = cell.units().iter().map(|u| u.as_str().to_owned()).collect();
    format!(
        "{{\"bounds\":[{}],\"chart\":{},\"topology\":{},\"units\":{}}}",
        bounds.join(","),
        json_escape(cell.chart().as_str()),
        json_escape(cell.topology().as_str()),
        format_string_array(&units),
    )
}

fn format_lp_primal_json(obj: &ProofObject) -> String {
    let ProofPayload::LpPrimal(p) = obj.payload() else {
        unreachable!("expected lp-primal payload");
    };
    let forces: Vec<String> = p
        .vertices
        .iter()
        .map(|v| {
            vertex_force_values(v)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect();
    let eq_digests: Vec<String> = obj
        .primitive_equations()
        .iter()
        .map(|e| e.dag_digest().to_hex())
        .collect();
    let enc = obj.canonical_bytes();
    format!(
        "{{\"cell\":{},\"encoding_digest\":{},\"encoding_hex\":{},\"equation_digests\":{},\"forces\":{},\"kind\":{},\"prefix\":{}}}",
        format_cell_json(obj),
        json_escape(&obj.canonical_digest().to_hex()),
        json_escape(&hex_bytes(&enc)),
        format_string_array(&eq_digests),
        format_string_array(&forces),
        json_escape(obj.kind().as_str()),
        format_string_array(&format_prefix(obj)),
    )
}

fn format_lp_farkas_json(obj: &ProofObject) -> String {
    let ProofPayload::LpFarkas(p) = obj.payload() else {
        unreachable!("expected lp-farkas payload");
    };
    let dual_y = match &p.dual_y {
        DualVector::Constant(v) => v.iter().map(ToString::to_string).collect::<Vec<_>>().join(","),
        DualVector::RationalInChart(v) => {
            v.iter().map(|d| d.digest().to_hex()).collect::<Vec<_>>().join(",")
        }
    };
    let cell_ends = obj
        .establishes()
        .cell()
        .bounds()
        .first()
        .map(|b| format!("{},{}", b.lower_end().as_str(), b.upper_end().as_str()))
        .unwrap_or_default();
    let eq_digests: Vec<String> = obj
        .primitive_equations()
        .iter()
        .map(|e| e.dag_digest().to_hex())
        .collect();
    let enc = obj.canonical_bytes();
    format!(
        "{{\"cell\":{},\"cell_ends\":{},\"dual_y\":{},\"encoding_digest\":{},\"encoding_hex\":{},\"equation_digests\":{},\"kind\":{},\"prefix\":{}}}",
        format_cell_json(obj),
        json_escape(&cell_ends),
        json_escape(&dual_y),
        json_escape(&obj.canonical_digest().to_hex()),
        json_escape(&hex_bytes(&enc)),
        format_string_array(&eq_digests),
        json_escape(obj.kind().as_str()),
        format_string_array(&format_prefix(obj)),
    )
}

fn main() -> ExitCode {
    let mut swap_ports = false;
    let mut json_mode = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--json" => json_mode = true,
            "--swap-ports" => swap_ports = true,
            other => {
                eprintln!("unknown argument: {other}");
                return ExitCode::from(2);
            }
        }
    }
    let _ = json_mode;

    let lp_primal = build_stance_lp_primal(swap_ports);
    let lp_farkas = build_stance_lp_farkas();

    println!(
        "{{\"lp_farkas\":{},\"lp_primal\":{}}}",
        format_lp_farkas_json(&lp_farkas),
        format_lp_primal_json(&lp_primal),
    );
    ExitCode::SUCCESS
}
