#![forbid(unsafe_code)]

//! Airlock feature-group registry, compile-time `BUILT_WITH` set, and typed
//! `GroupAbsent` error (`N33a-11`, §33.3, §33.9, §33.16, INV-SUITE-01).
//!
//! Determinism class D0; zero `unsafe`; zero stdout/stderr printing (rule L-7);
//! names no volatile FrankenSim types in any public signature.

use std::error::Error;
use std::fmt;

use frob_core::refusal::Refusal;

use crate::refuse::group_absent_refusal;

/// The nine runtime airlock feature groups of `frob-fsim`.
///
/// The dev-only `harness` group, the `optimize-race` M1 sub-feature, the build
/// modes (`mode-inproc`, `mode-remote`), and the non-group `conformance`
/// feature are intentionally absent from `Group`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    Referee,
    Geometry,
    Ledger,
    Optimize,
    Stats,
    Spectral,
    Actuation,
    Structure,
    Multiphysics,
}

impl Group {
    /// All nine runtime airlock groups in canonical declaration order.
    pub const ALL: &'static [Self] = &[
        Self::Referee,
        Self::Geometry,
        Self::Ledger,
        Self::Optimize,
        Self::Stats,
        Self::Spectral,
        Self::Actuation,
        Self::Structure,
        Self::Multiphysics,
    ];

    /// Canonical Cargo feature name for this group.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Referee => "referee",
            Self::Geometry => "geometry",
            Self::Ledger => "ledger",
            Self::Optimize => "optimize",
            Self::Stats => "stats",
            Self::Spectral => "spectral",
            Self::Actuation => "actuation",
            Self::Structure => "structure",
            Self::Multiphysics => "multiphysics",
        }
    }

    /// Parse a canonical group name into a [`Group`].
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "referee" => Some(Self::Referee),
            "geometry" => Some(Self::Geometry),
            "ledger" => Some(Self::Ledger),
            "optimize" => Some(Self::Optimize),
            "stats" => Some(Self::Stats),
            "spectral" => Some(Self::Spectral),
            "actuation" => Some(Self::Actuation),
            "structure" => Some(Self::Structure),
            "multiphysics" => Some(Self::Multiphysics),
            _ => None,
        }
    }

    /// Refusal pointer for a missing compiled-in group (`foundation/<group>/compiled`).
    #[must_use]
    pub fn compiled_pointer(self) -> String {
        format!("foundation/{}/compiled", self.as_str())
    }

    /// Return `true` iff this binary was compiled with this feature group enabled.
    #[must_use]
    pub fn is_compiled(self) -> bool {
        BUILT_WITH.contains(&self)
    }

    /// Look up this group's static metadata descriptor in [`GROUP_REGISTRY`].
    #[must_use]
    pub fn descriptor(self) -> &'static GroupDescriptor {
        group_descriptor(self.as_str()).expect("every Group variant has a GROUP_REGISTRY entry")
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Compile-time set of runtime airlock groups enabled in this build of `frob-fsim`.
///
/// Identical in debug and release builds for the same Cargo feature set; excludes
/// `mode-inproc`, `mode-remote`, `conformance`, `harness`, and `optimize-race`.
pub const BUILT_WITH: &[Group] = &[
    #[cfg(feature = "referee")]
    Group::Referee,
    #[cfg(feature = "geometry")]
    Group::Geometry,
    #[cfg(feature = "ledger")]
    Group::Ledger,
    #[cfg(feature = "optimize")]
    Group::Optimize,
    #[cfg(feature = "stats")]
    Group::Stats,
    #[cfg(feature = "spectral")]
    Group::Spectral,
    #[cfg(feature = "actuation")]
    Group::Actuation,
    #[cfg(feature = "structure")]
    Group::Structure,
    #[cfg(feature = "multiphysics")]
    Group::Multiphysics,
];

/// Whether the M1 `optimize-race` sub-feature (`fs-race`, `fs-eproc`) is compiled in
/// (enabled directly or implied by the full `optimize` group; N33a-11 Review Note 1).
pub const BUILT_WITH_OPTIMIZE_RACE: bool = cfg!(feature = "optimize-race");

/// Render `BUILT_WITH` as a canonical comma-separated list (or `"none"` when empty).
#[must_use]
pub fn built_with_names() -> String {
    if BUILT_WITH.is_empty() {
        return "none".to_owned();
    }
    let mut out = String::new();
    for (idx, group) in BUILT_WITH.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push_str(group.as_str());
    }
    out
}

/// Typed error returned by disabled group stub constructors (`N33a-11`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupAbsent {
    pub group: Group,
}

impl GroupAbsent {
    /// Construct a `GroupAbsent` error for `group`.
    #[must_use]
    pub const fn new(group: Group) -> Self {
        Self { group }
    }

    /// Convert this typed error into the structured `FROB-REF-790` refusal value.
    #[must_use]
    pub fn into_refusal(self) -> Refusal {
        group_absent_refusal(self.group)
    }
}

impl fmt::Display for GroupAbsent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "airlock feature group `{}` is not compiled into this binary (FROB-REF-790)",
            self.group
        )
    }
}

impl Error for GroupAbsent {}

/// Static metadata record mirroring `[features.<name>]` in `crates/frob-fsim/contract.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupDescriptor {
    pub name: &'static str,
    pub group: Option<Group>,
    pub parent_group: Option<&'static str>,
    pub default_on: bool,
    pub dev_only: bool,
    pub traits: &'static [&'static str],
    pub crates: &'static [&'static str],
    pub consumers: &'static [&'static str],
    pub test_consumers: &'static [&'static str],
    pub first_milestone: &'static str,
    pub gate: &'static str,
}

/// Complete machine-readable airlock group registry (ten primary groups plus the
/// `optimize-race` M1 sub-feature from Review Note 1; `harness` includes test-only
/// `fs-ir` per Review Note 2).
pub const GROUP_REGISTRY: &[GroupDescriptor] = &[
    GroupDescriptor {
        name: "referee",
        group: Some(Group::Referee),
        parent_group: None,
        default_on: true,
        dev_only: false,
        traits: &[
            "RefereeDynamics",
            "RefereeContact",
            "Integrator",
            "PortSystem",
        ],
        crates: &[
            "fs-mbd",
            "fs-mbd-material",
            "fs-contact",
            "fs-tribo",
            "fs-time",
            "fs-motion",
            "fs-ports",
            "fs-system",
        ],
        consumers: &[
            "frob-model",
            "frob-dyn",
            "frob-contact",
            "frob-analyze",
            "frob-verbs",
            "frob-conformance",
        ],
        test_consumers: &["frob-conformance"],
        first_milestone: "M0",
        gate: "INT-FSIM-01",
    },
    GroupDescriptor {
        name: "geometry",
        group: Some(Group::Geometry),
        parent_group: None,
        default_on: true,
        dev_only: false,
        traits: &["GeometryKernel", "SceneQuery", "MeshExchange"],
        crates: &[
            "fs-query",
            "fs-geom",
            "fs-rep-sdf",
            "fs-rep-mesh",
            "fs-rep-frep",
            "fs-scene",
            "fs-topo",
            "fs-io",
        ],
        consumers: &[
            "frob-contact",
            "frob-scene",
            "frob-foundry",
            "frob-conformance",
        ],
        test_consumers: &["frob-conformance"],
        first_milestone: "M0",
        gate: "INT-FSIM-01",
    },
    GroupDescriptor {
        name: "ledger",
        group: Some(Group::Ledger),
        parent_group: None,
        default_on: true,
        dev_only: false,
        traits: &["Ledger", "Session", "Recompute"],
        crates: &["fs-ledger", "fs-session", "fs-package", "fs-recompute"],
        consumers: &["frob-chronicle", "frob-verbs"],
        test_consumers: &["frob-conformance"],
        first_milestone: "M0",
        gate: "INT-FSQL-01",
    },
    GroupDescriptor {
        name: "optimize",
        group: Some(Group::Optimize),
        parent_group: None,
        default_on: false,
        dev_only: false,
        traits: &["Optimizer", "Archive", "Racing"],
        crates: &[
            "fs-dfo",
            "fs-bo",
            "fs-ascent",
            "fs-archive",
            "fs-opt",
            "fs-sos",
            "fs-race",
            "fs-eproc",
            "fs-tropical",
            "fs-symmetry",
        ],
        consumers: &[
            "frob-opt",
            "frob-arena",
            "frob-dyn",
            "frob-mission",
            "frob-analyze",
            "frob-nemesis",
        ],
        test_consumers: &["frob-conformance"],
        first_milestone: "M3",
        gate: "INT-FSIM-14",
    },
    GroupDescriptor {
        name: "optimize-race",
        group: None,
        parent_group: Some("optimize"),
        default_on: false,
        dev_only: false,
        traits: &["Racing"],
        crates: &["fs-race", "fs-eproc"],
        consumers: &[
            "frob-opt",
            "frob-arena",
            "frob-dyn",
            "frob-mission",
            "frob-analyze",
            "frob-nemesis",
        ],
        test_consumers: &["frob-conformance"],
        first_milestone: "M1",
        gate: "INT-FSIM-14",
    },
    GroupDescriptor {
        name: "stats",
        group: Some(Group::Stats),
        parent_group: None,
        default_on: false,
        dev_only: false,
        traits: &[
            "Uncertainty",
            "ValueOfInformation",
            "Surrogate",
            "Assimilation",
        ],
        crates: &[
            "fs-uq",
            "fs-voi",
            "fs-surrogate",
            "fs-assimilate",
            "fs-robust",
            "fs-regime",
        ],
        consumers: &[
            "frob-nemesis",
            "frob-arena",
            "frob-ground",
            "frob-opt",
            "frob-mission",
            "frob-dyn",
        ],
        test_consumers: &["frob-conformance"],
        first_milestone: "M4",
        gate: "INT-FSIM-14",
    },
    GroupDescriptor {
        name: "spectral",
        group: Some(Group::Spectral),
        parent_group: None,
        default_on: false,
        dev_only: false,
        traits: &["Spectral"],
        crates: &["fs-spectral"],
        consumers: &["frob-gait", "frob-lifecycle"],
        test_consumers: &["frob-conformance"],
        first_milestone: "M3",
        gate: "INT-FSIM-13",
    },
    GroupDescriptor {
        name: "actuation",
        group: Some(Group::Actuation),
        parent_group: None,
        default_on: false,
        dev_only: false,
        traits: &[
            "Magnetics",
            "ThermalNetwork",
            "CurrentLoop",
            "PortHamiltonian",
        ],
        crates: &["fs-em", "fs-thermal", "fs-control", "fs-phs", "fs-matdb"],
        consumers: &["frob-actuate"],
        test_consumers: &["frob-conformance"],
        first_milestone: "M1",
        gate: "INT-FSIM-09",
    },
    GroupDescriptor {
        name: "structure",
        group: Some(Group::Structure),
        parent_group: None,
        default_on: false,
        dev_only: false,
        traits: &[
            "StructuralOpt",
            "ToleranceAllocation",
            "AsBuilt",
            "FabricationRules",
        ],
        crates: &[
            "fs-solid",
            "fs-topopt",
            "fs-topols",
            "fs-toleralloc",
            "fs-asbuilt",
            "fs-geocon",
            "fs-fab",
            "fs-adjoint",
        ],
        consumers: &["frob-foundry", "frob-ground"],
        test_consumers: &["frob-conformance"],
        first_milestone: "M5",
        gate: "INT-FSIM-15",
    },
    GroupDescriptor {
        name: "multiphysics",
        group: Some(Group::Multiphysics),
        parent_group: None,
        default_on: false,
        dev_only: false,
        traits: &["FluidIsland", "CoupledIsland"],
        crates: &["fs-lbm", "fs-couple"],
        consumers: &["frob-scene"],
        test_consumers: &["frob-conformance"],
        first_milestone: "M6",
        gate: "INT-FSIM-17",
    },
    GroupDescriptor {
        name: "harness",
        group: None,
        parent_group: None,
        default_on: false,
        dev_only: true,
        traits: &[],
        crates: &[
            "fs-roofline",
            "fs-detaudit",
            "fs-propcheck",
            "fs-render",
            "fs-la",
            "fs-ir",
        ],
        consumers: &[],
        test_consumers: &["frob-fsim", "frob-conformance"],
        first_milestone: "M0",
        gate: "INT-FSIM-01",
    },
];

/// Return all static descriptors in [`GROUP_REGISTRY`].
#[must_use]
pub const fn all_group_descriptors() -> &'static [GroupDescriptor] {
    GROUP_REGISTRY
}

/// Look up a group or sub-feature descriptor by canonical feature name.
#[must_use]
pub fn group_descriptor(name: &str) -> Option<&'static GroupDescriptor> {
    GROUP_REGISTRY.iter().find(|d| d.name == name)
}
