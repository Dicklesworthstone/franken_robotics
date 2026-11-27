#![forbid(unsafe_code)]

//! In-process `RefereeDynamics` adapter over FrankenSim's articulated dynamics (`BD-007`).
//!
//! Compiled strictly under `#[cfg(feature = "referee")]`. No volatile FrankenSim type
//! appears in any public signature; all upstream errors convert into [`RefereeRefusal`]
//! carrying the upstream reason text without retrying on weakened inputs.

use fs_ga::{Quat, Se3, So3, Twist, Vec3, Wrench};
use fs_mbd::articulated::motion::{
    FreeFloatingMotionLoads, FreeFloatingMotionState, step_free_floating,
};
use fs_mbd::articulated::{
    ArticulatedModel, BaseState, FreeFloatingBaseState, JointKind, forward_kinematics,
    free_floating_forward_dynamics, free_floating_momentum,
};
use fs_mbd::robot_models::{CatalogRobotModel, unitree_g1_29dof};

use super::{
    CatalogRobot, CoordEntry, CoordKind, CoordLayout, ModelFacts, ORIENTATION_PARAMETERIZATION,
    Rational, RefereeDynamics, RefereeModel, RefereeRefusal, RefereeRefusalKind, RefereeState,
    STANDARD_GRAVITY_M_S2,
};

const BASE_POSITION_COORDS: usize = 3;
const BASE_QUAT_COORDS: usize = 4;
const BASE_Q_COORDS: usize = BASE_POSITION_COORDS + BASE_QUAT_COORDS;
const BASE_V_COORDS: usize = 6;

/// Private inner representation wrapped by [`RefereeModel`].
#[derive(Debug, Clone)]
pub(super) struct RefereeModelInner {
    catalog: CatalogRobotModel,
    facts: ModelFacts,
}

impl RefereeModelInner {
    fn from_catalog(catalog: CatalogRobotModel) -> Result<Self, RefereeRefusal> {
        let facts = build_model_facts(&catalog)?;
        Ok(Self { catalog, facts })
    }

    pub(super) fn facts(&self) -> &ModelFacts {
        &self.facts
    }

    fn model(&self) -> &ArticulatedModel {
        self.catalog.model()
    }
}

/// In-process referee forward-dynamics and contact-free stepping adapter (`BD-007`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InProcRefereeDynamics;

impl InProcRefereeDynamics {
    /// Construct a new in-process referee dynamics adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Return world-Z centre of mass (m) and total linear momentum (kg m/s).
    pub fn vertical_observables(
        &self,
        model: &RefereeModel,
        state: &RefereeState,
    ) -> Result<(f64, f64), RefereeRefusal> {
        let inner = model.inner();
        let (base, q, v) = validate_and_unpack(
            inner.facts(),
            state,
            &vec![0.0; inner.facts().actuators as usize],
        )?;
        let model = inner.model();
        let upstream = |error: fs_mbd::articulated::ArticulatedError| {
            RefereeRefusal::new(RefereeRefusalKind::Upstream, error.to_string())
        };
        let kinematics = forward_kinematics(
            model,
            BaseState::prescribed(base.world_from_base, base.twist_body, Twist::zero()),
            q,
            v,
        )
        .map_err(upstream)?;
        let mut weighted_z = 0.0;
        for (link, pose) in model.links().iter().zip(&kinematics.world_from_link) {
            let inertia = link.inertia();
            let center = pose
                .transform_point(inertia.center_of_mass())
                .map_err(|error| {
                    RefereeRefusal::new(RefereeRefusalKind::Upstream, error.to_string())
                })?;
            weighted_z += inertia.mass() * center.z;
        }
        let momentum = free_floating_momentum(model, base, q, v).map_err(upstream)?;
        Ok((
            weighted_z / inner.facts().mass_kg,
            momentum.linear_momentum_world.z,
        ))
    }
}

impl RefereeDynamics for InProcRefereeDynamics {
    fn adapter_name(&self) -> &'static str {
        "inproc-referee-articulated-v0"
    }

    fn catalog_model(&self, which: CatalogRobot) -> Result<RefereeModel, RefereeRefusal> {
        let catalog = match which {
            CatalogRobot::G1 => unitree_g1_29dof().map_err(|e| {
                RefereeRefusal::new(
                    RefereeRefusalKind::Upstream,
                    format!("upstream catalog_model(G1) refused: {e}"),
                )
            })?,
        };
        let inner = RefereeModelInner::from_catalog(catalog)?;
        Ok(RefereeModel::from_inner(inner))
    }

    fn reference_state(&self, m: &RefereeModel) -> RefereeState {
        let facts = m.inner().facts();
        let mut q = vec![0.0_f64; facts.nq as usize];
        // Floating-base identity pose: position [0, 0, 0], unit quaternion [w=1, x=0, y=0, z=0].
        q[3] = 1.0;
        let v = vec![0.0_f64; facts.nv as usize];
        RefereeState { q, v }
    }

    fn forward_dynamics(
        &self,
        m: &RefereeModel,
        s: &RefereeState,
        tau: &[f64],
    ) -> Result<Vec<f64>, RefereeRefusal> {
        let inner = m.inner();
        let (base, q_joints, v_joints) = validate_and_unpack(inner.facts(), s, tau)?;
        let external = vec![Wrench::default(); inner.model().link_count()];
        let gravity = Vec3::new(0.0, 0.0, -STANDARD_GRAVITY_M_S2);
        let fd = free_floating_forward_dynamics(
            inner.model(),
            base,
            q_joints,
            v_joints,
            tau,
            gravity,
            &external,
        )
        .map_err(|e| {
            RefereeRefusal::new(
                RefereeRefusalKind::Upstream,
                format!("upstream free_floating_forward_dynamics refused: {e}"),
            )
        })?;

        let mut qdd = Vec::with_capacity(inner.facts().nv as usize);
        let ba = fd.base_spatial_acceleration_body;
        qdd.extend_from_slice(&[
            ba.angular.x,
            ba.angular.y,
            ba.angular.z,
            ba.linear.x,
            ba.linear.y,
            ba.linear.z,
        ]);
        qdd.extend_from_slice(&fd.generalized_acceleration);
        Ok(qdd)
    }

    fn step_free(
        &self,
        m: &RefereeModel,
        s: &RefereeState,
        tau: &[f64],
        h: Rational,
    ) -> Result<RefereeState, RefereeRefusal> {
        let inner = m.inner();
        let dt = h.to_f64().ok_or_else(|| {
            RefereeRefusal::new(
                RefereeRefusalKind::InvalidStep,
                format!(
                    "step duration `{}/{}` must be strictly positive and finite",
                    h.num(),
                    h.den()
                ),
            )
        })?;
        let (base, q_joints, v_joints) = validate_and_unpack(inner.facts(), s, tau)?;
        let motion_state = FreeFloatingMotionState {
            base,
            joint_position: q_joints.to_vec(),
            joint_velocity: v_joints.to_vec(),
        };
        let external = vec![Wrench::default(); inner.model().link_count()];
        let loads = FreeFloatingMotionLoads {
            generalized_force: tau,
            external_body_wrench: &external,
            gravity_world: Vec3::new(0.0, 0.0, -STANDARD_GRAVITY_M_S2),
        };
        let step =
            step_free_floating(inner.model(), &motion_state, loads, dt, || false).map_err(|e| {
                RefereeRefusal::new(
                    RefereeRefusalKind::Upstream,
                    format!("upstream step_free_floating refused: {e}"),
                )
            })?;
        Ok(pack_state(&step.state_after))
    }

    fn describe(&self, m: &RefereeModel) -> ModelFacts {
        m.inner().facts().clone()
    }
}

fn validate_and_unpack<'a>(
    facts: &ModelFacts,
    s: &'a RefereeState,
    tau: &[f64],
) -> Result<(FreeFloatingBaseState, &'a [f64], &'a [f64]), RefereeRefusal> {
    let expected_nq = facts.nq as usize;
    let expected_nv = facts.nv as usize;
    let expected_act = facts.actuators as usize;

    if s.q.len() != expected_nq {
        return Err(RefereeRefusal::length_mismatch("q", expected_nq, s.q.len()));
    }
    if s.v.len() != expected_nv {
        return Err(RefereeRefusal::length_mismatch("v", expected_nv, s.v.len()));
    }
    if tau.len() != expected_act {
        return Err(RefereeRefusal::length_mismatch(
            "tau",
            expected_act,
            tau.len(),
        ));
    }

    for (index, &val) in s.q.iter().enumerate() {
        if !val.is_finite() {
            return Err(RefereeRefusal::non_finite("q", index));
        }
    }
    for (index, &val) in s.v.iter().enumerate() {
        if !val.is_finite() {
            return Err(RefereeRefusal::non_finite("v", index));
        }
    }
    for (index, &val) in tau.iter().enumerate() {
        if !val.is_finite() {
            return Err(RefereeRefusal::non_finite("tau", index));
        }
    }

    let pos = Vec3::new(s.q[0], s.q[1], s.q[2]);
    let rot = So3::try_from_quat(Quat {
        w: s.q[3],
        x: s.q[4],
        y: s.q[5],
        z: s.q[6],
    })
    .map_err(|e| {
        RefereeRefusal::new(
            RefereeRefusalKind::Upstream,
            format!("invalid base orientation quaternion: {e}"),
        )
    })?;
    let pose = Se3::from_parts(rot, pos).map_err(|e| {
        RefereeRefusal::new(
            RefereeRefusalKind::Upstream,
            format!("invalid base SE(3) pose: {e}"),
        )
    })?;
    let twist = Twist::new(
        Vec3::new(s.v[0], s.v[1], s.v[2]),
        Vec3::new(s.v[3], s.v[4], s.v[5]),
    );

    Ok((
        FreeFloatingBaseState::new(pose, twist),
        &s.q[BASE_Q_COORDS..],
        &s.v[BASE_V_COORDS..],
    ))
}

fn pack_state(state: &FreeFloatingMotionState) -> RefereeState {
    let pos = state.base.world_from_base.translation();
    let rot = state.base.world_from_base.rotation();
    let quat = *rot.as_quat();
    let twist = state.base.twist_body;

    let mut q = Vec::with_capacity(BASE_Q_COORDS + state.joint_position.len());
    q.extend_from_slice(&[pos.x, pos.y, pos.z, quat.w, quat.x, quat.y, quat.z]);
    q.extend_from_slice(&state.joint_position);

    let mut v = Vec::with_capacity(BASE_V_COORDS + state.joint_velocity.len());
    v.extend_from_slice(&[
        twist.angular.x,
        twist.angular.y,
        twist.angular.z,
        twist.linear.x,
        twist.linear.y,
        twist.linear.z,
    ]);
    v.extend_from_slice(&state.joint_velocity);

    RefereeState { q, v }
}

fn build_model_facts(catalog: &CatalogRobotModel) -> Result<ModelFacts, RefereeRefusal> {
    let model = catalog.model();
    let links_usize = model.link_count();
    let dof_usize = model.dof_count();
    let links = u32::try_from(links_usize).map_err(|_| {
        RefereeRefusal::new(
            RefereeRefusalKind::Upstream,
            format!("link count {links_usize} overflows u32"),
        )
    })?;
    let actuators = u32::try_from(dof_usize).map_err(|_| {
        RefereeRefusal::new(
            RefereeRefusalKind::Upstream,
            format!("dof count {dof_usize} overflows u32"),
        )
    })?;
    let nq = u32::try_from(BASE_Q_COORDS + dof_usize).map_err(|_| {
        RefereeRefusal::new(
            RefereeRefusalKind::Upstream,
            format!("nq {} overflows u32", BASE_Q_COORDS + dof_usize),
        )
    })?;
    let nv = u32::try_from(BASE_V_COORDS + dof_usize).map_err(|_| {
        RefereeRefusal::new(
            RefereeRefusalKind::Upstream,
            format!("nv {} overflows u32", BASE_V_COORDS + dof_usize),
        )
    })?;

    let mut mass_kg = 0.0_f64;
    let mut link_names = Vec::with_capacity(links_usize);
    let mut joint_kinds = Vec::with_capacity(dof_usize);
    for link in model.links() {
        mass_kg += link.inertia().mass();
        link_names.push(link.name().to_owned());
        let kind = link.joint().kind();
        if kind != JointKind::Fixed {
            joint_kinds.push(kind);
        }
    }

    let mut position = Vec::with_capacity(nq as usize);
    for (idx, name) in ["base_x", "base_y", "base_z"].iter().enumerate() {
        position.push(CoordEntry {
            index: idx as u32,
            name: (*name).to_owned(),
            kind: CoordKind::BasePosition,
            unit: "m",
        });
    }
    for (offset, name) in ["base_qw", "base_qx", "base_qy", "base_qz"]
        .iter()
        .enumerate()
    {
        position.push(CoordEntry {
            index: (BASE_POSITION_COORDS + offset) as u32,
            name: (*name).to_owned(),
            kind: CoordKind::BaseOrientationQuatWxyz,
            unit: "1",
        });
    }

    let mut velocity = Vec::with_capacity(nv as usize);
    for (idx, name) in ["base_wx_body", "base_wy_body", "base_wz_body"]
        .iter()
        .enumerate()
    {
        velocity.push(CoordEntry {
            index: idx as u32,
            name: (*name).to_owned(),
            kind: CoordKind::BaseAngularVelocityBody,
            unit: "rad/s",
        });
    }
    for (offset, name) in ["base_vx_body", "base_vy_body", "base_vz_body"]
        .iter()
        .enumerate()
    {
        velocity.push(CoordEntry {
            index: (3 + offset) as u32,
            name: (*name).to_owned(),
            kind: CoordKind::BaseLinearVelocityBody,
            unit: "m/s",
        });
    }

    let mut actuator_names = Vec::with_capacity(dof_usize);
    for (j_idx, (joint_meta, j_kind)) in catalog.joints().iter().zip(joint_kinds.iter()).enumerate()
    {
        let (coord_kind, pos_unit, vel_unit) = match j_kind {
            JointKind::Revolute => (CoordKind::RevoluteJoint, "rad", "rad/s"),
            JointKind::Prismatic => (CoordKind::PrismaticJoint, "m", "m/s"),
            JointKind::Helical => (CoordKind::HelicalJoint, "rad", "rad/s"),
            JointKind::Fixed => continue,
        };
        actuator_names.push(joint_meta.name.to_owned());
        position.push(CoordEntry {
            index: (BASE_Q_COORDS + j_idx) as u32,
            name: joint_meta.name.to_owned(),
            kind: coord_kind,
            unit: pos_unit,
        });
        velocity.push(CoordEntry {
            index: (BASE_V_COORDS + j_idx) as u32,
            name: format!("{}_vel", joint_meta.name),
            kind: coord_kind,
            unit: vel_unit,
        });
    }

    Ok(ModelFacts {
        links,
        nq,
        nv,
        actuators,
        layout: CoordLayout {
            orientation_parameterization: ORIENTATION_PARAMETERIZATION,
            position,
            velocity,
            actuators: actuator_names,
            link_names,
        },
        mass_kg,
    })
}
