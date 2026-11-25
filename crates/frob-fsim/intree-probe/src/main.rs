#![forbid(unsafe_code)]

//! In-tree FrankenSim oracle probe for `BD-007` (`INT-FSIM-01` G1 digest capture).
//!
//! Runs inside a read-only `git archive` export of FrankenSim at the pinned commit
//! under `apps/frob-digest-probe/`, invoking `fs_mbd` directly (no airlock) and
//! hashing with the exact byte-for-byte `digest.rs` source file from `frob-fsim`.

#[allow(dead_code)]
mod digest;

use std::env;
use std::process;

use fs_ga::{Se3, Twist, Vec3, Wrench};
use fs_mbd::articulated::motion::{
    FreeFloatingMotionLoads, FreeFloatingMotionState, step_free_floating,
};
use fs_mbd::articulated::{FreeFloatingBaseState, JointKind, free_floating_forward_dynamics};
use fs_mbd::robot_models::unitree_g1_29dof;

const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;
const ROLLOUT_STEPS: usize = 960;
const ROLLOUT_DT: f64 = 1.0_f64 / 480.0_f64;

fn pack_state(state: &FreeFloatingMotionState) -> (Vec<f64>, Vec<f64>) {
    let pos = state.base.world_from_base.translation();
    let rot = state.base.world_from_base.rotation();
    let quat = *rot.as_quat();
    let twist = state.base.twist_body;

    let mut q = Vec::with_capacity(7 + state.joint_position.len());
    q.extend_from_slice(&[pos.x, pos.y, pos.z, quat.w, quat.x, quat.y, quat.z]);
    q.extend_from_slice(&state.joint_position);

    let mut v = Vec::with_capacity(6 + state.joint_velocity.len());
    v.extend_from_slice(&[
        twist.angular.x,
        twist.angular.y,
        twist.angular.z,
        twist.linear.x,
        twist.linear.y,
        twist.linear.z,
    ]);
    v.extend_from_slice(&state.joint_velocity);
    (q, v)
}

fn main() {
    let diagnostic = env::args().any(|a| a == "--diagnostic");

    let catalog = match unitree_g1_29dof() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("frob-digest-probe: unitree_g1_29dof failed: {e}");
            process::exit(1);
        }
    };
    let model = catalog.model();
    let links = u32::try_from(model.link_count()).expect("link_count fits u32");
    let dof = model.dof_count();
    let nq = 7 + dof;
    let nv = 6 + dof;

    let mut mass_kg = 0.0_f64;
    let mut link_names = Vec::with_capacity(model.link_count());
    let mut joint_kinds = Vec::with_capacity(dof);
    for link in model.links() {
        mass_kg += link.inertia().mass();
        link_names.push(link.name());
        let kind = link.joint().kind();
        if kind != JointKind::Fixed {
            joint_kinds.push(match kind {
                JointKind::Revolute => "revolute",
                JointKind::Prismatic => "prismatic",
                JointKind::Helical => "helical",
                JointKind::Fixed => "fixed",
            });
        }
    }
    let joint_names: Vec<&str> = catalog.joints().iter().map(|j| j.name).collect();

    let base_ref = FreeFloatingBaseState::stationary(Se3::identity());
    let q_zero = vec![0.0_f64; dof];
    let v_zero = vec![0.0_f64; dof];
    let tau_zero = vec![0.0_f64; dof];
    let external = vec![Wrench::default(); model.link_count()];
    let gravity = Vec3::new(0.0, 0.0, -STANDARD_GRAVITY_M_S2);

    let init_state = FreeFloatingMotionState {
        base: base_ref,
        joint_position: q_zero.clone(),
        joint_velocity: v_zero.clone(),
    };
    let (q0, v0) = pack_state(&init_state);

    let fd = match free_floating_forward_dynamics(
        model,
        base_ref,
        &q_zero,
        &v_zero,
        &tau_zero,
        gravity,
        &external,
    ) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("frob-digest-probe: free_floating_forward_dynamics failed: {e}");
            process::exit(1);
        }
    };
    let ba: Twist = fd.base_spatial_acceleration_body;
    let mut qdd = Vec::with_capacity(nv);
    qdd.extend_from_slice(&[
        ba.angular.x,
        ba.angular.y,
        ba.angular.z,
        ba.linear.x,
        ba.linear.y,
        ba.linear.z,
    ]);
    qdd.extend_from_slice(&fd.generalized_acceleration);

    let fd_digest = digest::forward_dynamics_digest_hex(links, &q0, &v0, &qdd);

    let mut cur = init_state;
    let loads = FreeFloatingMotionLoads {
        generalized_force: &tau_zero,
        external_body_wrench: &external,
        gravity_world: gravity,
    };
    for step_idx in 0..ROLLOUT_STEPS {
        match step_free_floating(model, &cur, loads, ROLLOUT_DT, || false) {
            Ok(step) => cur = step.state_after,
            Err(e) => {
                eprintln!(
                    "frob-digest-probe: step_free_floating failed at step {step_idx}: {e}"
                );
                process::exit(1);
            }
        }
    }
    let (q_end, v_end) = pack_state(&cur);
    let rollout_2s_digest = digest::state_digest_hex(links, &q_end, &v_end);

    println!("target_arch={}", env::consts::ARCH);
    println!("target_os={}", env::consts::OS);
    println!("links={links}");
    println!("nq={nq}");
    println!("nv={nv}");
    println!("actuators={dof}");
    println!("mass_kg={mass_kg:.8}");
    println!("orientation_parameterization=unit-quaternion-wxyz");
    println!("gravity_world_m_s2=0.0,0.0,-{STANDARD_GRAVITY_M_S2}");
    println!("rollout_steps={ROLLOUT_STEPS}");
    println!("rollout_h=1/480");
    println!("fd_digest={fd_digest}");
    println!("rollout_2s_digest={rollout_2s_digest}");
    println!("link_names={}", link_names.join(","));
    println!("joint_names={}", joint_names.join(","));
    println!("joint_kinds={}", joint_kinds.join(","));

    if diagnostic {
        for (i, &x) in qdd.iter().enumerate() {
            println!("diag.qdd[{i}]={:016x} ({x:.17e})", x.to_bits());
        }
        for (i, &x) in q_end.iter().enumerate() {
            println!("diag.q_end[{i}]={:016x} ({x:.17e})", x.to_bits());
        }
        for (i, &x) in v_end.iter().enumerate() {
            println!("diag.v_end[{i}]={:016x} ({x:.17e})", x.to_bits());
        }
    }
}
