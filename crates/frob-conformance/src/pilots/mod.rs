#![deny(unsafe_code)]

//! Pilot register (`crates/frob-conformance/registry/pilots.toml`, schema
//! `frob.gauntlet.pilot/1`), frozen protocols, funding rule, completeness check,
//! Moonshot cap, and Markdown printout (`N38a-09`, `GATE-GAUNT-08`, §38.5).
//!
//! Determinism class D0; zero `unsafe`; library functions print nothing (rule L-7).
//! The CLI driver [`run_cli`] is invoked exclusively through the declared example
//! target `cargo run -p frob-conformance --example pilots -- ...`.

pub mod complete;
pub mod funding;
pub mod printout;
pub mod schema;
pub mod validate;

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use frob_core::canon::{CanonError, Canonical, Value as CanonValue, to_canonical};
use frob_core::id::hash_blob;
use frob_core::testkit::registry as testkit_registry;

use crate::gates_v0::GateRegistry;
use crate::reasons::HarnessReason;

pub use complete::{
    CompletenessReport, MissingPilot, UngatedMultiMechanismWarning, UnregisteredSeedRow,
    check_completeness, check_completeness_with_expected,
};
pub use funding::{
    AllPackagesFunding, BeadFunding, FallbackResolution, GateGaunt08Scan, GateGaunt08Violation,
    MechanismFunding, MechanismVerdict, PILOT_FUNDING_REASONS, PackageFunding,
    PackageFundingStatus, PackageMeta, RerunStatus, StatusCounts, WorkspacePackages,
    check_moonshot_status_change, evaluate_record, fundable_all_in, fundable_bead_in,
    fundable_mechanism_in, fundable_package_in, moonshots_running_by_milestone,
    pilot_result_supports_gate, rerun_status_in, resolve_fallback, scan_gate_gaunt_08,
};
pub use printout::{check_printout_matches, render_pilots_markdown};
pub use schema::{
    CONVENTION_15_RULE, CeilingSpec, CostSpec, ExpectedPilotRow, FallbackAlso, FallbackSpec,
    MOONSHOT_RUNNING_CAP, PILOTS_EXPECTED_PATH, PILOTS_EXPECTED_SCHEMA, PILOTS_PATH,
    PILOTS_PRINTOUT_PATH, PILOTS_SCHEMA, PilotLoadError, PilotRecord, PilotRegister, PilotResult,
    PilotSite, PilotStatus, PilotTier, PilotsExpected, ProtocolSpec, RECEIPTS_PILOTS_DIR,
    ResultValue, ToolSpec, is_plan_time_ledger_id, is_valid_pilot_id, load_pilots_expected,
    normalize_blake3_hex, parse_pilots_expected,
};
pub use validate::{
    LoadOptions, PILOT_LOAD_REASONS, effective_status, load_register, load_register_with_options,
    parse_and_validate, sampling_allowance_in, validate_register,
};

/// Locate the active workspace root, honoring `FROB_PILOTS_ROOT`,
/// `FROB_REGISTRY_ROOT`, and `FROB_CONFORMANCE_ROOT` before falling back to
/// `frob_core::testkit::registry::root()`.
#[must_use]
pub fn workspace_root() -> PathBuf {
    for var in [
        "FROB_PILOTS_ROOT",
        "FROB_REGISTRY_ROOT",
        "FROB_CONFORMANCE_ROOT",
    ] {
        if let Some(ov) = env::var_os(var)
            && !ov.is_empty()
        {
            return PathBuf::from(ov);
        }
    }
    testkit_registry::root().expect("locate workspace root via franken-stack.lock")
}

/// Rule 5b: return the frozen `protocol.sampling_episodes_per_design` for a
/// registered, non-void pilot record in the active workspace, or `None` otherwise.
#[must_use]
pub fn sampling_allowance(id: &str) -> Option<u32> {
    let root = workspace_root();
    sampling_allowance_at(&root, id)
}

/// Rule 5b: return the frozen `protocol.sampling_episodes_per_design` at `root`.
#[must_use]
pub fn sampling_allowance_at(root: &Path, id: &str) -> Option<u32> {
    let reg = load_register(root).ok()?;
    sampling_allowance_in(&reg, id, Some(root), None)
}

/// Live plan-time ledger status lookup at the active workspace (`N38a-09` / `N38a-10`).
#[must_use]
pub fn rerun_status(id: &str) -> Option<RerunStatus> {
    let root = workspace_root();
    let reg = load_register(&root).ok()?;
    rerun_status_in(&reg, id, Some(&root), None)
}

/// Funding rule for one mechanism in one work package at the active workspace.
pub fn fundable_mechanism(
    work_package: &str,
    mechanism: &str,
) -> Result<MechanismFunding, PilotLoadError> {
    let root = workspace_root();
    fundable_mechanism_at(&root, work_package, mechanism)
}

/// Funding rule for one mechanism in one work package at `root`.
pub fn fundable_mechanism_at(
    root: &Path,
    work_package: &str,
    mechanism: &str,
) -> Result<MechanismFunding, PilotLoadError> {
    let reg = load_register(root)?;
    let gates = GateRegistry::load(root).ok();
    let ws = WorkspacePackages::load(root);
    Ok(fundable_mechanism_in(
        &reg,
        work_package,
        mechanism,
        gates.as_ref(),
        Some(root),
        None,
        &ws,
    ))
}

/// Funding rule for a work package at the active workspace.
pub fn fundable(work_package: &str) -> Result<PackageFunding, PilotLoadError> {
    let root = workspace_root();
    fundable_at(&root, work_package)
}

/// Funding rule for a work package at `root`.
pub fn fundable_at(root: &Path, work_package: &str) -> Result<PackageFunding, PilotLoadError> {
    let reg = load_register(root)?;
    let gates = GateRegistry::load(root).ok();
    let ws = WorkspacePackages::load(root);
    Ok(fundable_package_in(
        &reg,
        work_package,
        gates.as_ref(),
        Some(root),
        None,
        &ws,
    ))
}

/// Funding rule for a single bead key at the active workspace.
pub fn fundable_bead(key: &str) -> Result<BeadFunding, PilotLoadError> {
    let root = workspace_root();
    fundable_bead_at(&root, key)
}

/// Funding rule for a single bead key at `root`.
pub fn fundable_bead_at(root: &Path, key: &str) -> Result<BeadFunding, PilotLoadError> {
    let reg = load_register(root)?;
    let gates = GateRegistry::load(root).ok();
    let ws = WorkspacePackages::load(root);
    Ok(fundable_bead_in(
        &reg,
        key,
        None,
        gates.as_ref(),
        Some(root),
        None,
        &ws,
    ))
}

/// Evaluate every upper-tier and pilot-bearing work package at `root`.
pub fn fundable_all_at(root: &Path) -> Result<AllPackagesFunding, PilotLoadError> {
    let reg = load_register(root)?;
    let gates = GateRegistry::load(root).ok();
    let ws = WorkspacePackages::load(root);
    Ok(fundable_all_in(&reg, gates.as_ref(), Some(root), None, &ws))
}

/// Run the completeness check at the active workspace (`pilots::complete`).
pub fn complete() -> Result<CompletenessReport, PilotLoadError> {
    let root = workspace_root();
    complete_at(&root)
}

/// Run the completeness check at `root`.
pub fn complete_at(root: &Path) -> Result<CompletenessReport, PilotLoadError> {
    let reg = load_register(root)?;
    check_completeness(&reg, root)
}

// ============================================================================
// CLI driver for `cargo run -p frob-conformance --example pilots -- ...`
// ============================================================================

/// Execute the `examples/pilots.rs` CLI.
///
/// Exit codes follow `registry/reasons.toml` (`N38-REF`):
/// - `0`: query succeeded (funding answers, completeness pass, printout match).
/// - `3`: refusal (`moonshot-cap`, `--require-fundable` blocked, or invalid CLI usage).
/// - `101`: register load error (`reason=<load-reason>`) or completeness failure (`reason=missing`).
pub fn run_cli<W1: Write, W2: Write>(
    args: &[String],
    stdout: &mut W1,
    stderr: &mut W2,
) -> Result<(), u8> {
    let Some(sub) = args.first().map(String::as_str) else {
        let _ = writeln!(
            stderr,
            "usage: pilots <fundable|complete|check|set-status|printout|hash> [options]"
        );
        return Err(3);
    };

    match sub {
        "fundable" => cli_fundable(&args[1..], stdout, stderr),
        "complete" => cli_complete(&args[1..], stdout, stderr),
        "check" => cli_check(&args[1..], stdout, stderr),
        "set-status" => cli_set_status(&args[1..], stdout, stderr),
        "printout" => cli_printout(&args[1..], stdout, stderr),
        "hash" => cli_hash(&args[1..], stdout, stderr),
        "-h" | "--help" | "help" => {
            let _ = writeln!(
                stdout,
                "usage:\n  pilots fundable (--all | --package <WP-nn> | --bead <key>) [--json] [--require-fundable] [--root <dir>]\n  pilots complete [--json] [--root <dir>]\n  pilots check [--prev <path>] [--json] [--root <dir>]\n  pilots set-status --id <PILOT-ID> --status <status> [--milestone <Mn>] [--write] [--json] [--root <dir>]\n  pilots printout [--write | --check] [--root <dir>]\n  pilots hash <path>"
            );
            Ok(())
        }
        other => {
            let _ = writeln!(stderr, "pilots: unknown subcommand `{other}`");
            Err(3)
        }
    }
}

fn cli_fundable<W1: Write, W2: Write>(
    args: &[String],
    stdout: &mut W1,
    stderr: &mut W2,
) -> Result<(), u8> {
    let mut all = false;
    let mut package: Option<String> = None;
    let mut bead: Option<String> = None;
    let mut as_json = false;
    let mut require_fundable = false;
    let mut root_arg: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--all" => all = true,
            "--json" => as_json = true,
            "--require-fundable" => require_fundable = true,
            "--package" => {
                i += 1;
                package = args.get(i).cloned();
            }
            "--bead" => {
                i += 1;
                bead = args.get(i).cloned();
            }
            "--root" => {
                i += 1;
                root_arg = args.get(i).map(PathBuf::from);
            }
            other => {
                let _ = writeln!(stderr, "pilots fundable: unknown option `{other}`");
                return Err(3);
            }
        }
        i += 1;
    }

    let root = root_arg.unwrap_or_else(workspace_root);
    let reg = match load_register(&root) {
        Ok(r) => r,
        Err(e) => {
            let _ = writeln!(stderr, "{e}");
            return Err(101);
        }
    };
    let gates = GateRegistry::load(&root).ok();
    let ws = WorkspacePackages::load(&root);

    if all {
        let report = fundable_all_in(&reg, gates.as_ref(), Some(&root), None, &ws);
        if as_json {
            emit_canon(stdout, &AllPackagesJson(&report))?;
        } else {
            let _ = writeln!(
                stdout,
                "packages={} fundable={} partial={} refused={}",
                report.packages.len(),
                report.fundable_count,
                report.partial_count,
                report.refused_count
            );
            for pkg in &report.packages {
                let _ = writeln!(stdout, "{} {}", pkg.work_package, pkg.status);
            }
        }
        if require_fundable && (report.partial_count > 0 || report.refused_count > 0) {
            let first_reason = report
                .packages
                .iter()
                .flat_map(|p| p.mechanisms.iter())
                .find_map(|m| m.verdict.reason())
                .unwrap_or(HarnessReason::Missing);
            let _ = writeln!(
                stderr,
                "{}",
                first_reason.format_failure_line("one or more packages are not fundable")
            );
            return Err(3);
        }
        return Ok(());
    }

    if let Some(wp) = package {
        let pkg = fundable_package_in(&reg, &wp, gates.as_ref(), Some(&root), None, &ws);
        if as_json {
            emit_canon(stdout, &PackageFundingJson(&pkg))?;
        } else {
            let _ = writeln!(stdout, "{} {}", pkg.work_package, pkg.status);
            for m in &pkg.mechanisms {
                match &m.verdict {
                    MechanismVerdict::Fundable { provisional, .. } => {
                        let _ = writeln!(
                            stdout,
                            "  {} Fundable (provisional={provisional})",
                            m.mechanism
                        );
                    }
                    MechanismVerdict::Blocked { reason, detail, .. } => {
                        let _ = writeln!(
                            stdout,
                            "  {} Blocked reason={} {}",
                            m.mechanism,
                            reason.as_str(),
                            detail
                        );
                    }
                }
            }
        }
        if require_fundable && pkg.status != PackageFundingStatus::Fundable {
            let reason = pkg
                .mechanisms
                .iter()
                .find_map(|m| m.verdict.reason())
                .unwrap_or(HarnessReason::Missing);
            let _ = writeln!(
                stderr,
                "{}",
                reason.format_failure_line(&format!(
                    "work package `{}` is `{}`",
                    pkg.work_package, pkg.status
                ))
            );
            return Err(3);
        }
        return Ok(());
    }

    if let Some(key) = bead {
        let bf = fundable_bead_in(&reg, &key, None, gates.as_ref(), Some(&root), None, &ws);
        if as_json {
            emit_canon(stdout, &BeadFundingJson(&bf))?;
        } else if bf.fundable {
            let _ = writeln!(
                stdout,
                "{} Fundable note={}",
                bf.bead,
                bf.note.as_deref().unwrap_or("none")
            );
        } else {
            let r = bf.reason.unwrap_or(HarnessReason::Missing);
            let _ = writeln!(
                stdout,
                "{} Blocked reason={} {}",
                bf.bead,
                r.as_str(),
                bf.detail
            );
        }
        if require_fundable && !bf.fundable {
            let r = bf.reason.unwrap_or(HarnessReason::Missing);
            let _ = writeln!(stderr, "{}", r.format_failure_line(&bf.detail));
            return Err(3);
        }
        return Ok(());
    }

    let _ = writeln!(
        stderr,
        "pilots fundable: specify one of --all, --package <WP-nn>, or --bead <key>"
    );
    Err(3)
}

fn cli_complete<W1: Write, W2: Write>(
    args: &[String],
    stdout: &mut W1,
    stderr: &mut W2,
) -> Result<(), u8> {
    let mut as_json = false;
    let mut root_arg: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => as_json = true,
            "--root" => {
                i += 1;
                root_arg = args.get(i).map(PathBuf::from);
            }
            other => {
                let _ = writeln!(stderr, "pilots complete: unknown option `{other}`");
                return Err(3);
            }
        }
        i += 1;
    }

    let root = root_arg.unwrap_or_else(workspace_root);
    let report = match complete_at(&root) {
        Ok(r) => r,
        Err(e) => {
            let _ = writeln!(stderr, "{e}");
            return Err(101);
        }
    };

    if as_json {
        emit_canon(stdout, &CompletenessJson(&report))?;
    } else {
        for w in &report.warnings {
            let _ = writeln!(stdout, "{w}");
        }
        for s in &report.seed_unregistered {
            let _ = writeln!(stdout, "{s}");
        }
        for m in &report.missing {
            let _ = writeln!(stdout, "{m}");
        }
    }

    if !report.is_ok() {
        let first = &report.missing[0];
        let _ = writeln!(stderr, "{first}");
        return Err(101);
    }
    Ok(())
}

fn cli_check<W1: Write, W2: Write>(
    args: &[String],
    stdout: &mut W1,
    stderr: &mut W2,
) -> Result<(), u8> {
    let mut as_json = false;
    let mut root_arg: Option<PathBuf> = None;
    let mut prev_arg: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => as_json = true,
            "--root" => {
                i += 1;
                root_arg = args.get(i).map(PathBuf::from);
            }
            "--prev" => {
                i += 1;
                prev_arg = args.get(i).map(PathBuf::from);
            }
            other => {
                let _ = writeln!(stderr, "pilots check: unknown option `{other}`");
                return Err(3);
            }
        }
        i += 1;
    }

    let root = root_arg.unwrap_or_else(workspace_root);
    let prev_reg = if let Some(ref pp) = prev_arg {
        let bytes = match fs::read(pp) {
            Ok(b) => b,
            Err(e) => {
                let _ = writeln!(stderr, "pilots check: read {}: {e}", pp.display());
                return Err(101);
            }
        };
        match parse_and_validate(
            &bytes,
            &pp.to_string_lossy(),
            LoadOptions::default().with_receipt_checks(false),
        ) {
            Ok(r) => Some(r),
            Err(e) => {
                let _ = writeln!(stderr, "{e}");
                return Err(101);
            }
        }
    } else {
        None
    };

    let mut opts = LoadOptions::for_root(&root);
    if let Some(ref p) = prev_reg {
        opts = opts.with_previous(p);
    }
    let reg = match load_register_with_options(&root, opts) {
        Ok(r) => r,
        Err(e) => {
            let _ = writeln!(stderr, "{e}");
            return Err(101);
        }
    };
    let counts = StatusCounts::from_register(&reg, Some(&root), prev_reg.as_ref());
    if as_json {
        emit_canon(stdout, &StatusCountsJson(&counts))?;
    } else {
        let _ = writeln!(
            stdout,
            "ok pilots={} registered={} running={} falsifier-rejected={} falsifier-holds={} void={}",
            counts.total(),
            counts.registered,
            counts.running,
            counts.falsifier_rejected,
            counts.falsifier_holds,
            counts.void
        );
    }
    Ok(())
}

fn cli_set_status<W1: Write, W2: Write>(
    args: &[String],
    stdout: &mut W1,
    stderr: &mut W2,
) -> Result<(), u8> {
    let mut id: Option<String> = None;
    let mut status_raw: Option<String> = None;
    let mut milestone: Option<String> = None;
    let mut write_back = false;
    let mut as_json = false;
    let mut root_arg: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--id" => {
                i += 1;
                id = args.get(i).cloned();
            }
            "--status" => {
                i += 1;
                status_raw = args.get(i).cloned();
            }
            "--milestone" => {
                i += 1;
                milestone = args.get(i).cloned();
            }
            "--write" => write_back = true,
            "--json" => as_json = true,
            "--root" => {
                i += 1;
                root_arg = args.get(i).map(PathBuf::from);
            }
            other => {
                let _ = writeln!(stderr, "pilots set-status: unknown option `{other}`");
                return Err(3);
            }
        }
        i += 1;
    }

    let Some(pilot_id) = id else {
        let _ = writeln!(stderr, "pilots set-status: missing `--id <PILOT-ID>`");
        return Err(3);
    };
    let Some(st_raw) = status_raw else {
        let _ = writeln!(stderr, "pilots set-status: missing `--status <status>`");
        return Err(3);
    };
    let Some(new_status) = PilotStatus::parse(&st_raw) else {
        let _ = writeln!(stderr, "pilots set-status: unknown status `{st_raw}`");
        return Err(3);
    };

    let root = root_arg.unwrap_or_else(workspace_root);
    let mut reg = match load_register(&root) {
        Ok(r) => r,
        Err(e) => {
            let _ = writeln!(stderr, "{e}");
            return Err(101);
        }
    };
    let ws = WorkspacePackages::load(&root);

    if let Err(e) = check_moonshot_status_change(
        &reg,
        &pilot_id,
        new_status,
        milestone.as_deref(),
        Some(&root),
        None,
        &ws,
    ) {
        let _ = writeln!(stderr, "{e}");
        return Err(3);
    }

    if let Some(rec) = reg.get_mut(&pilot_id) {
        rec.status = new_status;
    }
    if write_back {
        let out_path = root.join(PILOTS_PATH);
        if let Err(e) = fs::write(&out_path, reg.to_canonical_toml()) {
            let _ = writeln!(stderr, "pilots set-status: write failed: {e}");
            return Err(101);
        }
    }

    if as_json {
        let resp = SetStatusJson {
            id: &pilot_id,
            status: new_status.as_str(),
            written: write_back,
        };
        emit_canon(stdout, &resp)?;
    } else {
        let _ = writeln!(stdout, "ok {pilot_id} status={}", new_status.as_str());
    }
    Ok(())
}

fn cli_printout<W1: Write, W2: Write>(
    args: &[String],
    stdout: &mut W1,
    stderr: &mut W2,
) -> Result<(), u8> {
    let mut write_file = false;
    let mut check_only = false;
    let mut root_arg: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--write" => write_file = true,
            "--check" => check_only = true,
            "--root" => {
                i += 1;
                root_arg = args.get(i).map(PathBuf::from);
            }
            other => {
                let _ = writeln!(stderr, "pilots printout: unknown option `{other}`");
                return Err(3);
            }
        }
        i += 1;
    }
    let root = root_arg.unwrap_or_else(workspace_root);
    if check_only {
        match check_printout_matches(&root) {
            Ok(()) => {
                let _ = writeln!(stdout, "ok {PILOTS_PRINTOUT_PATH}");
                return Ok(());
            }
            Err(e) => {
                let _ = writeln!(stderr, "{e}");
                return Err(101);
            }
        }
    }
    let reg = match load_register(&root) {
        Ok(r) => r,
        Err(e) => {
            let _ = writeln!(stderr, "{e}");
            return Err(101);
        }
    };
    let rendered = render_pilots_markdown(&reg, Some(&root), None);
    if write_file {
        let path = root.join(PILOTS_PRINTOUT_PATH);
        if let Some(parent) = path.parent()
            && let Err(e) = fs::create_dir_all(parent)
        {
            let _ = writeln!(stderr, "pilots printout: mkdir failed: {e}");
            return Err(101);
        }
        if let Err(e) = fs::write(&path, rendered.as_bytes()) {
            let _ = writeln!(stderr, "pilots printout: write failed: {e}");
            return Err(101);
        }
        let _ = writeln!(stdout, "wrote {PILOTS_PRINTOUT_PATH}");
    } else {
        let _ = write!(stdout, "{rendered}");
    }
    Ok(())
}

fn cli_hash<W1: Write, W2: Write>(
    args: &[String],
    stdout: &mut W1,
    stderr: &mut W2,
) -> Result<(), u8> {
    let Some(path) = args.first() else {
        let _ = writeln!(stderr, "pilots hash: missing <path>");
        return Err(3);
    };
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            let _ = writeln!(stderr, "pilots hash: read `{path}`: {e}");
            return Err(3);
        }
    };
    let _ = writeln!(stdout, "{}", hash_blob(&bytes).to_hex());
    Ok(())
}

fn emit_canon<W: Write>(stdout: &mut W, value: &impl Canonical) -> Result<(), u8> {
    let bytes = to_canonical(value).map_err(|_| 101u8)?;
    let _ = writeln!(stdout, "{}", bytes.as_str());
    Ok(())
}

// ---- Canonical JSON serializers (`frob_core::canon`) ----

struct SetStatusJson<'a> {
    id: &'a str,
    status: &'a str,
    written: bool,
}

impl Canonical for SetStatusJson<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        w.object(|o| {
            o.str("id", self.id)?;
            o.bool("ok", true)?;
            o.str("status", self.status)?;
            o.bool("written", self.written)?;
            Ok(())
        })
    }
}

struct StatusCountsJson<'a>(&'a StatusCounts);

impl Canonical for StatusCountsJson<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        let c = self.0;
        w.object(|o| {
            o.int53("differs", c.differs as i64)?;
            o.int53("falsifier_holds", c.falsifier_holds as i64)?;
            o.int53("falsifier_rejected", c.falsifier_rejected as i64)?;
            o.int53("inconclusive", c.inconclusive as i64)?;
            o.int53("registered", c.registered as i64)?;
            o.int53("reproduced", c.reproduced as i64)?;
            o.int53("running", c.running as i64)?;
            o.int53("total", c.total() as i64)?;
            o.int53("void", c.void as i64)?;
            o.int53("withdrawn", c.withdrawn as i64)?;
            Ok(())
        })
    }
}

struct MechanismFundingJson<'a>(&'a MechanismFunding);

impl Canonical for MechanismFundingJson<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        let m = self.0;
        w.object(|o| {
            match &m.verdict {
                MechanismVerdict::Fundable {
                    provisional,
                    queued_for_referee,
                    supports_gate,
                    note,
                } => {
                    o.str("detail", "")?;
                    o.null("fallback")?;
                    o.bool("fundable", true)?;
                    o.null("kill_gate")?;
                    o.str("mechanism", &m.mechanism)?;
                    match note {
                        Some(n) => o.str("note", n)?,
                        None => o.null("note")?,
                    }
                    o.array("pilot_ids", |a| {
                        for pid in &m.pilot_ids {
                            a.push_str(pid);
                        }
                        Ok(())
                    })?;
                    o.bool("provisional", *provisional)?;
                    o.bool("queued_for_referee", *queued_for_referee)?;
                    o.null("reason")?;
                    o.bool("supports_gate", *supports_gate)?;
                }
                MechanismVerdict::Blocked {
                    reason,
                    fallback,
                    kill_gate,
                    detail,
                } => {
                    o.str("detail", detail)?;
                    match fallback {
                        Some(fb) => o.str("fallback", fb)?,
                        None => o.null("fallback")?,
                    }
                    o.bool("fundable", false)?;
                    match kill_gate {
                        Some(kg) => o.str("kill_gate", kg)?,
                        None => o.null("kill_gate")?,
                    }
                    o.str("mechanism", &m.mechanism)?;
                    o.null("note")?;
                    o.array("pilot_ids", |a| {
                        for pid in &m.pilot_ids {
                            a.push_str(pid);
                        }
                        Ok(())
                    })?;
                    o.bool("provisional", false)?;
                    o.bool("queued_for_referee", false)?;
                    o.str("reason", reason.as_str())?;
                    o.bool("supports_gate", false)?;
                }
            }
            Ok(())
        })
    }
}

struct PackageFundingJson<'a>(&'a PackageFunding);

impl Canonical for PackageFundingJson<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        let p = self.0;
        w.object(|o| {
            o.array("mechanisms", |a| {
                for m in &p.mechanisms {
                    a.push_term(&MechanismFundingJson(m))?;
                }
                Ok(())
            })?;
            o.str("status", p.status.as_str())?;
            o.str("work_package", &p.work_package)?;
            Ok(())
        })
    }
}

struct AllPackagesJson<'a>(&'a AllPackagesFunding);

impl Canonical for AllPackagesJson<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        let r = self.0;
        w.object(|o| {
            o.object("counts", |co| {
                co.int53("fundable", r.fundable_count as i64)?;
                co.int53("partial", r.partial_count as i64)?;
                co.int53("refused", r.refused_count as i64)?;
                co.int53("total", r.packages.len() as i64)?;
                Ok(())
            })?;
            o.object("moonshots_running_by_milestone", |mo| {
                for (ms, cnt) in &r.moonshots_running_by_milestone {
                    mo.int53(ms, *cnt as i64)?;
                }
                Ok(())
            })?;
            o.array("packages", |a| {
                for pkg in &r.packages {
                    a.push_term(&PackageFundingJson(pkg))?;
                }
                Ok(())
            })?;
            o.str("schema", "frob.gauntlet.pilot.funding/1")?;
            o.member_term("status_counts", &StatusCountsJson(&r.status_counts))?;
            Ok(())
        })
    }
}

struct BeadFundingJson<'a>(&'a BeadFunding);

impl Canonical for BeadFundingJson<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        let b = self.0;
        w.object(|o| {
            o.str("bead", &b.bead)?;
            o.str("detail", &b.detail)?;
            match &b.fallback {
                Some(fb) => o.str("fallback", fb)?,
                None => o.null("fallback")?,
            }
            o.bool("fundable", b.fundable)?;
            match &b.kill_gate {
                Some(kg) => o.str("kill_gate", kg)?,
                None => o.null("kill_gate")?,
            }
            o.array("mechanisms", |a| {
                for m in &b.mechanisms {
                    a.push_term(&MechanismFundingJson(m))?;
                }
                Ok(())
            })?;
            match &b.note {
                Some(n) => o.str("note", n)?,
                None => o.null("note")?,
            }
            o.bool("provisional", b.provisional)?;
            o.bool("queued_for_referee", b.queued_for_referee)?;
            match b.reason {
                Some(r) => o.str("reason", r.as_str())?,
                None => o.null("reason")?,
            }
            match &b.work_package {
                Some(wp) => o.str("work_package", wp)?,
                None => o.null("work_package")?,
            }
            Ok(())
        })
    }
}

struct CompletenessJson<'a>(&'a CompletenessReport);

impl Canonical for CompletenessJson<'_> {
    fn write(&self, w: CanonValue<'_>) -> Result<(), CanonError> {
        let c = self.0;
        w.object(|o| {
            o.array("missing", |a| {
                for m in &c.missing {
                    a.push_object(|mo| {
                        mo.str("mechanism", &m.mechanism)?;
                        mo.str("source", &m.source)?;
                        mo.str("tier", m.tier.as_str())?;
                        match &m.work_package {
                            Some(wp) => mo.str("work_package", wp)?,
                            None => mo.null("work_package")?,
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            o.bool("ok", c.is_ok())?;
            o.array("seed_unregistered", |a| {
                for s in &c.seed_unregistered {
                    a.push_object(|so| {
                        match &s.falsifier_threshold {
                            Some(t) => so.str("falsifier_threshold", t)?,
                            None => so.null("falsifier_threshold")?,
                        }
                        so.str("identifier", &s.identifier)?;
                        so.str("kill_gate", &s.kill_gate)?;
                        so.str("mechanism", &s.mechanism)?;
                        match &s.pilot {
                            Some(p) => so.str("pilot", p)?,
                            None => so.null("pilot")?,
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            o.array("warnings", |a| {
                for warn in &c.warnings {
                    a.push_object(|wo| {
                        wo.str("mechanism", &warn.mechanism)?;
                        wo.array("other_mechanisms", |oa| {
                            for om in &warn.other_mechanisms {
                                oa.push_str(om);
                            }
                            Ok(())
                        })?;
                        wo.str("pilot_bead", &warn.pilot_bead)?;
                        wo.str("pilot_id", &warn.pilot_id)?;
                        wo.str("work_package", &warn.work_package)?;
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
            Ok(())
        })
    }
}
