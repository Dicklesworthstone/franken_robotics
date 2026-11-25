#![forbid(unsafe_code)]

//! FrankenSim airlock crate (`BD-006`, ADR-SUITE-01, INV-SUITE-01, `INT-FSIM-01`).
//!
//! Confines FrankenSim's volatile path dependencies (`fs-contact`, `fs-ga`,
//! `fs-mbd`, `fs-query`, `fs-time`) behind FrankenRobotics-owned APIs. No
//! FrankenSim crate or type is publicly re-exported from this crate.
//!
//! Determinism class D0; zero `unsafe`; zero stdout/stderr printing (rule L-7);
//! zero fallback default for the FrankenSim build-identity environment variable.

/// Sorted list of the five FrankenSim crates admitted in `INT-FSIM-01` (`BD-006`).
pub const ADMITTED_FRANKENSIM_CRATES: [&str; 5] = [
    "fs-contact",
    "fs-ga",
    "fs-mbd",
    "fs-query",
    "fs-time",
];

/// FrankenSim commit pinned in `franken-stack.lock` when `frob-fsim` was created.
pub const FRANKENSIM_PIN_REV: &str = "08e9195c6faea3363377b7fdfb6d6b07f5b552f3";

/// Which build-identity environment variable was present at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildIdentitySource {
    /// Operator-observed dependency graph receipt (`FRANKENSIM_DEPGRAPH_RECEIPT`).
    Receipt,
    /// Workspace development equivalence-class salt (`FRANKENSIM_DEPGRAPH_SALT`).
    DevSalt,
}

/// Compile-time FrankenSim build-identity evidence observed by `frob-fsim`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildIdentity {
    pub source: BuildIdentitySource,
    pub env_var: &'static str,
    pub value: &'static str,
}

/// Return the compile-time build-identity variable supplied by the build
/// environment, with receipt taking precedence over development salt (matching
/// `fs-la/build.rs`). Returns `None` when neither variable was set; never
/// synthesizes a fallback default.
#[must_use]
pub const fn build_identity() -> Option<BuildIdentity> {
    if let Some(receipt) = option_env!("FRANKENSIM_DEPGRAPH_RECEIPT")
        && !receipt.is_empty()
    {
        return Some(BuildIdentity {
            source: BuildIdentitySource::Receipt,
            env_var: "FRANKENSIM_DEPGRAPH_RECEIPT",
            value: receipt,
        });
    }
    if let Some(salt) = option_env!("FRANKENSIM_DEPGRAPH_SALT")
        && !salt.is_empty()
    {
        return Some(BuildIdentity {
            source: BuildIdentitySource::DevSalt,
            env_var: "FRANKENSIM_DEPGRAPH_SALT",
            value: salt,
        });
    }
    None
}

/// Result of exercising one admitted FrankenSim crate through the airlock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrateSmokeResult {
    pub crate_name: &'static str,
    pub item_path: &'static str,
    pub passed: bool,
}

/// Exercise `fs-contact` via `fs_contact::ballistic::first_ballistic_contact`.
///
/// Initial gap 2.0 m closing at -1.0 m/s with zero acceleration over a 5.0 s
/// horizon must yield a closing contact at t = 2.0 s and relative speed -1.0 m/s.
#[must_use]
pub fn smoke_fs_contact() -> CrateSmokeResult {
    let input = fs_contact::ballistic::BallisticContactInput {
        gap: 2.0,
        relative_normal_speed: -1.0,
        relative_normal_acceleration: 0.0,
        duration_s: 5.0,
    };
    let passed = matches!(
        fs_contact::ballistic::first_ballistic_contact(input),
        Ok(Some(hit))
            if (hit.time_s - 2.0).abs() <= 1e-12
                && (hit.relative_normal_speed + 1.0).abs() <= 1e-12
    );
    CrateSmokeResult {
        crate_name: "fs-contact",
        item_path: "fs_contact::ballistic::first_ballistic_contact",
        passed,
    }
}

/// Exercise `fs-ga` via `fs_ga::Vec3::cross` and `fs_ga::Quat::identity().rotate`.
///
/// Right-handed basis product `e_x × e_y` rotated by the identity quaternion
/// must equal `e_z = (0, 0, 1)`.
#[must_use]
pub fn smoke_fs_ga() -> CrateSmokeResult {
    let ex = fs_ga::Vec3::new(1.0, 0.0, 0.0);
    let ey = fs_ga::Vec3::new(0.0, 1.0, 0.0);
    let ez = fs_ga::Quat::identity().rotate(ex.cross(ey));
    let passed = ez.x.abs() <= 1e-12 && ez.y.abs() <= 1e-12 && (ez.z - 1.0).abs() <= 1e-12;
    CrateSmokeResult {
        crate_name: "fs-ga",
        item_path: "fs_ga::Vec3::cross + fs_ga::Quat::rotate",
        passed,
    }
}

/// Exercise `fs-mbd` via `fs_mbd::Vec3::dot` and `fs_mbd::Vec3::cross`.
///
/// `(1, 2, 3) · (4, -5, 6) = 12` and `(1, 0, 0) × (0, 1, 0) = (0, 0, 1)`.
#[must_use]
pub fn smoke_fs_mbd() -> CrateSmokeResult {
    let a = fs_mbd::Vec3::new(1.0, 2.0, 3.0);
    let b = fs_mbd::Vec3::new(4.0, -5.0, 6.0);
    let dot_ok = (a.dot(b) - 12.0).abs() <= 1e-12;
    let ez = fs_mbd::Vec3::new(1.0, 0.0, 0.0).cross(fs_mbd::Vec3::new(0.0, 1.0, 0.0));
    let cross_ok = ez.x.abs() <= 1e-12 && ez.y.abs() <= 1e-12 && (ez.z - 1.0).abs() <= 1e-12;
    CrateSmokeResult {
        crate_name: "fs-mbd",
        item_path: "fs_mbd::Vec3::dot + fs_mbd::Vec3::cross",
        passed: dot_ok && cross_ok,
    }
}

/// Exercise `fs-query` via `fs_query::closest_triangle_point`.
///
/// Query point `(0.25, 0.25, 2.0)` directly above the unit xy-triangle
/// `[(0,0,0), (1,0,0), (0,1,0)]` projects to `(0.25, 0.25, 0.0)` at distance `2.0`.
#[must_use]
pub fn smoke_fs_query() -> CrateSmokeResult {
    let tri = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    let passed = matches!(
        fs_query::closest_triangle_point([0.25, 0.25, 2.0], tri),
        Ok(pt)
            if (pt.distance - 2.0).abs() <= 1e-12
                && (pt.point[0] - 0.25).abs() <= 1e-12
                && (pt.point[1] - 0.25).abs() <= 1e-12
                && pt.point[2].abs() <= 1e-12
    );
    CrateSmokeResult {
        crate_name: "fs-query",
        item_path: "fs_query::closest_triangle_point",
        passed,
    }
}

/// Exercise `fs-time` via `fs_time::verlet_step`.
///
/// One unit-mass harmonic-oscillator step (`F(q) = -q`) from `q = 1, p = 0`
/// with step `h = 0.1` yields exact Stormer-Verlet state `q_1 = 0.995` and
/// `p_1 = -0.09975`.
#[must_use]
pub fn smoke_fs_time() -> CrateSmokeResult {
    let mut q = [1.0_f64];
    let mut p = [0.0_f64];
    let mut scratch = [0.0_f64];
    fs_time::verlet_step(
        &mut q,
        &mut p,
        0.1,
        &|pos, force| {
            force[0] = -pos[0];
        },
        &mut scratch,
    );
    let passed = (q[0] - 0.995).abs() <= 1e-12 && (p[0] + 0.09975).abs() <= 1e-12;
    CrateSmokeResult {
        crate_name: "fs-time",
        item_path: "fs_time::verlet_step",
        passed,
    }
}

/// Run all five admitted FrankenSim crate smoke checks in canonical sorted order.
#[must_use]
pub fn smoke_all_admitted() -> [CrateSmokeResult; 5] {
    [
        smoke_fs_contact(),
        smoke_fs_ga(),
        smoke_fs_mbd(),
        smoke_fs_query(),
        smoke_fs_time(),
    ]
}
