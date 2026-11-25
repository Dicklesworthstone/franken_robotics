//! Device launch receipt carrying FrankenSim's twelve verbatim evidence fields
//! (`AE-01`..`AE-12`, `frankensim/docs/ACCELERATOR_DOCTRINE.md`) beside
//! FrankenRobotics execution metadata (`BD-014`, `INV-SPARK-02`).
//!
//! Deliberately carries no `exploratory` flag: until `OPEN-SPARK-1` is ratified,
//! `evidence.device.exploratory` is attached at the agent boundary (`N21a-02`)
//! so stored receipts never need relabelling. Zero `unsafe`; prints nothing (L-7).

use std::fmt;

use frob_core::evidence::DetClass;
use frob_core::refusal::{Refusal, RefusalCode, Repair, RetryPolicy};

use crate::manifest::ShapeClassCaps;

/// Verbatim `AE-01`..`AE-12` field IDs and names from FrankenSim's
/// `docs/ACCELERATOR_DOCTRINE.md` (commit `83c260121532a6a16d5b014326f9a3f937e14e94`).
pub const DOCTRINE_FIELDS: [(&str, &str); 12] = [
    ("AE-01", "device identity"),
    ("AE-02", "driver and runtime version"),
    ("AE-03", "compiler and backend identity"),
    ("AE-04", "kernel source identity"),
    ("AE-05", "kernel and workload identity"),
    ("AE-06", "CPU machine identity and measured axes"),
    ("AE-07", "reduction and determinism policy"),
    ("AE-08", "CPU-versus-device numerical comparison"),
    ("AE-09", "permanent CPU reference"),
    ("AE-10", "cancellation, drain, and finalize outcome"),
    ("AE-11", "phase wall-time and energy attribution"),
    ("AE-12", "go or no-go decision"),
];

/// Terminal outcome recorded in `AE-10` (`cancellation, drain, and finalize outcome`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelOutcome {
    Completed,
    CancelledDrained,
    Failed,
}

impl CancelOutcome {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::CancelledDrained => "cancelled-drained",
            Self::Failed => "failed",
        }
    }
}

/// Wall-time attribution across harness phases (`AE-11`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PhaseTimes {
    pub compile_s: f64,
    pub allocate_s: f64,
    pub launch_s: f64,
    pub read_back_s: f64,
}

impl PhaseTimes {
    #[must_use]
    pub fn format_ae11(self) -> String {
        format!(
            "compile={:.6}s allocate={:.6}s launch={:.6}s read_back={:.6}s energy=unmeasured-gap",
            self.compile_s, self.allocate_s, self.launch_s, self.read_back_s
        )
    }
}

/// Per-stage placement record (empty until `N21b-28` populates it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagePlacement {
    pub stage: String,
    pub backend: String,
}

/// Receipt schema validation failure (`INV-SPARK-02`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptError {
    MissingField {
        id: &'static str,
        name: &'static str,
    },
}

impl fmt::Display for ReceiptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField { id, name } => {
                write!(
                    f,
                    "device receipt missing required evidence field {id} ({name})"
                )
            }
        }
    }
}

impl std::error::Error for ReceiptError {}

/// Execution receipt returned by every `KernelBackend::launch` (`BD-014`).
///
/// Local placeholder with verbatim FrankenSim doctrine field names until
/// upstream `FD-FSIM-09` provides a shared device receipt type in FrankenSim.
#[derive(Debug, Clone, PartialEq)]
pub struct Receipt {
    pub ae01_device_identity: Option<String>,
    pub ae02_driver_and_runtime_version: Option<String>,
    pub ae03_compiler_and_backend_identity: Option<String>,
    pub ae04_kernel_source_identity: Option<String>,
    pub ae05_kernel_and_workload_identity: Option<String>,
    pub ae06_cpu_machine_identity_and_measured_axes: Option<String>,
    pub ae07_reduction_and_determinism_policy: Option<String>,
    pub ae08_cpu_versus_device_numerical_comparison: Option<String>,
    pub ae09_permanent_cpu_reference: Option<String>,
    pub ae10_cancellation_drain_and_finalize_outcome: Option<String>,
    pub ae11_phase_wall_time_and_energy_attribution: Option<String>,
    pub ae12_go_or_no_go_decision: Option<String>,
    pub placements: Vec<StagePlacement>,
    pub fingerprint_digest: Option<String>,
    pub determinism_claimed: DetClass,
    pub determinism_attained: DetClass,
    pub shape_class: ShapeClassCaps,
    pub launch_count: u32,
    pub retries: u32,
    pub device_seconds: f64,
    pub wall_seconds: f64,
    pub phase_times: PhaseTimes,
    pub citable: bool,
}

impl Receipt {
    /// Ordered view of the twelve doctrine fields `(AE-id, verbatim name, value)`.
    #[must_use]
    pub fn ae_entries(&self) -> [(&'static str, &'static str, Option<&str>); 12] {
        [
            (
                DOCTRINE_FIELDS[0].0,
                DOCTRINE_FIELDS[0].1,
                self.ae01_device_identity.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[1].0,
                DOCTRINE_FIELDS[1].1,
                self.ae02_driver_and_runtime_version.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[2].0,
                DOCTRINE_FIELDS[2].1,
                self.ae03_compiler_and_backend_identity.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[3].0,
                DOCTRINE_FIELDS[3].1,
                self.ae04_kernel_source_identity.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[4].0,
                DOCTRINE_FIELDS[4].1,
                self.ae05_kernel_and_workload_identity.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[5].0,
                DOCTRINE_FIELDS[5].1,
                self.ae06_cpu_machine_identity_and_measured_axes.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[6].0,
                DOCTRINE_FIELDS[6].1,
                self.ae07_reduction_and_determinism_policy.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[7].0,
                DOCTRINE_FIELDS[7].1,
                self.ae08_cpu_versus_device_numerical_comparison.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[8].0,
                DOCTRINE_FIELDS[8].1,
                self.ae09_permanent_cpu_reference.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[9].0,
                DOCTRINE_FIELDS[9].1,
                self.ae10_cancellation_drain_and_finalize_outcome.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[10].0,
                DOCTRINE_FIELDS[10].1,
                self.ae11_phase_wall_time_and_energy_attribution.as_deref(),
            ),
            (
                DOCTRINE_FIELDS[11].0,
                DOCTRINE_FIELDS[11].1,
                self.ae12_go_or_no_go_decision.as_deref(),
            ),
        ]
    }

    /// Clear field `AE-k` (`k` in `1..=12`); used by negative-control tests.
    pub fn clear_ae_field(&mut self, index_1_based: u8) {
        match index_1_based {
            1 => self.ae01_device_identity = None,
            2 => self.ae02_driver_and_runtime_version = None,
            3 => self.ae03_compiler_and_backend_identity = None,
            4 => self.ae04_kernel_source_identity = None,
            5 => self.ae05_kernel_and_workload_identity = None,
            6 => self.ae06_cpu_machine_identity_and_measured_axes = None,
            7 => self.ae07_reduction_and_determinism_policy = None,
            8 => self.ae08_cpu_versus_device_numerical_comparison = None,
            9 => self.ae09_permanent_cpu_reference = None,
            10 => self.ae10_cancellation_drain_and_finalize_outcome = None,
            11 => self.ae11_phase_wall_time_and_energy_attribution = None,
            12 => self.ae12_go_or_no_go_decision = None,
            _ => {}
        }
    }

    /// `INV-SPARK-02`: every one of `AE-01`..`AE-12` must be present and non-empty.
    pub fn validate(&self) -> Result<(), ReceiptError> {
        for (id, name, value) in self.ae_entries() {
            let ok = value.is_some_and(|s| !s.trim().is_empty());
            if !ok {
                return Err(ReceiptError::MissingField { id, name });
            }
        }
        Ok(())
    }

    /// Update `read_back_s` and refresh `AE-11` consistently.
    pub fn record_read_back_seconds(&mut self, read_back_s: f64) {
        self.phase_times.read_back_s = read_back_s;
        self.wall_seconds = self.phase_times.compile_s
            + self.phase_times.allocate_s
            + self.phase_times.launch_s
            + self.phase_times.read_back_s;
        self.ae11_phase_wall_time_and_energy_attribution = Some(self.phase_times.format_ae11());
    }

    /// Agent-boundary evidence admission (`INV-SPARK-02`): requires all twelve
    /// doctrine fields plus a current device fingerprint digest, refusing an
    /// absent fingerprint with `FROB-REF-606` (exit 6).
    pub fn validate_agent_boundary(&self, device: &str) -> Result<(), Refusal> {
        if let Err(err) = self.validate() {
            return Err(Refusal::new(
                RefusalCode::FrobRef990,
                err.to_string(),
                "/receipt",
                err.to_string(),
                "all twelve AE-01..AE-12 evidence fields present",
                "fields",
                RetryPolicy::Never,
                vec![Repair::new("frob doctor")],
            ));
        }
        let has_fp = self
            .fingerprint_digest
            .as_deref()
            .is_some_and(|s| !s.trim().is_empty());
        if !has_fp {
            return Err(Refusal::new(
                RefusalCode::FrobRef606,
                format!("fingerprint missing for device `{device}`"),
                format!("/device/{device}/fingerprint"),
                "null",
                "current hardware fingerprint digest",
                "blake3-hex",
                RetryPolicy::AfterChange,
                vec![
                    Repair::new("frob spark fingerprint"),
                    Repair::new("frob spark check"),
                    Repair::new("Run on batch-cpu"),
                ],
            ));
        }
        Ok(())
    }
}
