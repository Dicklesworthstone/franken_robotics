#![forbid(unsafe_code)]

//! Forced-kill crash-consistency harness for `BD-009` (`INT-FSQL-01`).
//!
//! Spawns `ledger_kill_probe` over two pipes, sends a real OS `SIGKILL` at the
//! seed's deterministic kill point (and during recovery for seeds `90..=99`),
//! reopens the ledger, verifies `integrity_check`, exact prefix `m in {a, a+1}`,
//! zero partial transactions, recovery report consistency, and equality of the
//! resumed chain digest with the uninterrupted reference.
//!
//! All scratch files stay under `${BIG_SCRATCH:-$TMPDIR}/frob-BrownHeron/` and
//! are never deleted (`COMMON.md` 9b). Row payloads are never logged.

#[path = "ledger_kill_probe/runner.rs"]
mod probe_runner;

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use frob_core::id::Blake3;
use frob_fsim::ledger::probe::{
    CHAIN_HEAD_KEY, DEFAULT_ROWS_PER_TXN, DEFAULT_TXN_COUNT, GOLDEN_SEED_0_N200_K8,
    GOLDEN_SEED_17_N200_K8, KillPhase, KillPoint, chain_key, chain_step, decode_chain_head,
    kill_point_for_seed, reference_chain_digest, row_key, row_value, table_for_row, zero_chain,
};
use frob_fsim::ledger::{FsqliteLedger, OpenOptions, RecoveryReport, SchemaVersion, TableId};

const CHILD_ARGS_ENV: &str = "FROB_LEDGER_KILL_PROBE_ARGS";
const CHILD_ARG_SEP: &str = "\x1f";

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

/// Build a `Command` that spawns a real separate OS child process executing
/// `ledger_kill_probe` logic via `--exact ledger_kill_probe_child --ignored --nocapture`
/// so `cargo test -p frob-fsim --test ledger_forced_kill -- --ignored forced_kill_100`
/// works standalone without declaring a product `[[bin]]` on `frob-fsim`.
fn probe_command(args: &[&str]) -> Command {
    let exe = std::env::current_exe().expect("resolve current test executable");
    let mut cmd = Command::new(exe); // ubs:ignore — re-execs own compiled test binary from std::env::current_exe()
    cmd.args([
        "--exact",
        "ledger_kill_probe_child",
        "--ignored",
        "--nocapture",
    ]);
    cmd.env(CHILD_ARGS_ENV, args.join(CHILD_ARG_SEP));
    cmd
}

#[test]
#[ignore = "internal child entrypoint spawned by ledger_forced_kill trials"]
fn ledger_kill_probe_child() {
    let Ok(raw) = std::env::var(CHILD_ARGS_ENV) else {
        return;
    };
    let argv: Vec<String> = raw.split(CHILD_ARG_SEP).map(ToOwned::to_owned).collect();
    match probe_runner::run_from_args(argv) {
        Ok(()) => std::process::exit(0),
        Err(msg) => {
            let _ = writeln!(std::io::stderr().lock(), "ledger_kill_probe error: {msg}");
            std::process::exit(1);
        }
    }
}

fn scratch_dir(tag: &str) -> PathBuf {
    let base = std::env::var_os("BIG_SCRATCH")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let seq = SCRATCH_SEQ.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let dir = base.join("frob-BrownHeron").join(format!(
        "bd009-harness-{tag}-{}-{nanos}-{seq}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("create forced-kill scratch dir");
    dir
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlantMode {
    None,
    SplitWriter,
    RowSwap,
}

impl PlantMode {
    const fn as_arg(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SplitWriter => "split-writer",
            Self::RowSwap => "row-swap",
        }
    }
}

struct ChildGuard(Option<Child>);

impl ChildGuard {
    fn new(child: Child) -> Self {
        Self(Some(child))
    }

    fn kill_and_wait(&mut self) -> Result<(), String> {
        let Some(mut child) = self.0.take() else {
            return Ok(());
        };
        child.kill().map_err(|e| format!("SIGKILL child: {e}"))?;
        let _ = child
            .wait()
            .map_err(|e| format!("wait killed child: {e}"))?;
        Ok(())
    }

    fn wait_success(&mut self) -> Result<(), String> {
        let Some(mut child) = self.0.take() else {
            return Ok(());
        };
        let status = child.wait().map_err(|e| format!("wait child: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("child exited non-zero: {status}"))
        }
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrialOutcome {
    seed: u64,
    kill_point: KillPoint,
    last_acked_a: u64,
    recovered_m: u64,
    recovery_report: RecoveryReport,
    integrity_clean: bool,
    final_chain_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TrialFailure {
    PartialTransaction {
        txn_idx: u64,
        present_rows: u32,
        expected_rows: u32,
    },
    NonPrefixGap {
        missing_before: u64,
        present_at: u64,
    },
    PrefixOutOfRange {
        last_acked_a: u64,
        recovered_m: u64,
        phase: KillPhase,
    },
    ChainMismatch {
        txn_idx: u64,
        detail: String,
    },
    RowMismatch {
        txn_idx: u64,
        row_idx: u32,
    },
    IntegrityNotClean {
        detail: String,
    },
    RecoveryInconsistent {
        report: RecoveryReport,
        last_acked_a: u64,
        recovered_m: u64,
    },
    HarnessError(String),
}

fn verify_prefix_and_chain(
    ledger: &FsqliteLedger,
    seed: u64,
    n_txns: u64,
    k_rows: u32,
    last_acked_a: u64,
    phase: KillPhase,
    plant: PlantMode,
    skip_chain_check: bool,
) -> Result<u64, TrialFailure> {
    let mut recovered_m: u64 = 0;
    let mut saw_absent = false;
    let mut running_chain = zero_chain();

    for txn_idx in 1..=n_txns {
        let mut present_rows: Vec<Vec<u8>> = Vec::with_capacity(k_rows as usize);
        let mut row_count: u32 = 0;
        for r in 0..k_rows {
            let got = ledger
                .get_committed(table_for_row(r), &row_key(seed, txn_idx, r))
                .map_err(|e| TrialFailure::HarnessError(format!("get t{txn_idx} r{r}: {e}")))?;
            if let Some(bytes) = got {
                row_count += 1;
                present_rows.push(bytes);
            }
        }
        let chain_entry = ledger
            .get_committed(TableId::Meta, &chain_key(seed, txn_idx))
            .map_err(|e| TrialFailure::HarnessError(format!("get chain t{txn_idx}: {e}")))?;

        if row_count == 0 && chain_entry.is_none() {
            saw_absent = true;
            continue;
        }

        if saw_absent {
            return Err(TrialFailure::NonPrefixGap {
                missing_before: recovered_m + 1,
                present_at: txn_idx,
            });
        }

        if row_count != k_rows || chain_entry.is_none() {
            return Err(TrialFailure::PartialTransaction {
                txn_idx,
                present_rows: row_count,
                expected_rows: k_rows,
            });
        }

        // Complete transaction present: check deterministic payload & chain when enabled.
        if !skip_chain_check {
            let slices: Vec<&[u8]> = present_rows.iter().map(Vec::as_slice).collect();
            let recomputed = chain_step(running_chain, &slices);
            let stored_raw = chain_entry.as_deref().unwrap_or(&[]);
            if stored_raw.len() != 32 {
                return Err(TrialFailure::ChainMismatch {
                    txn_idx,
                    detail: format!("stored chain digest length {} != 32", stored_raw.len()),
                });
            }
            if stored_raw != recomputed.as_bytes() {
                return Err(TrialFailure::ChainMismatch {
                    txn_idx,
                    detail: format!(
                        "stored chain {} disagreed with recomputed {} from read rows",
                        Blake3::from_bytes(stored_raw.try_into().unwrap_or([0u8; 32])).to_hex(),
                        recomputed.to_hex()
                    ),
                });
            }

            if plant == PlantMode::None {
                for (r_idx, got_bytes) in present_rows.iter().enumerate() {
                    let expected = row_value(seed, txn_idx, r_idx as u32);
                    if got_bytes.as_slice() != expected.as_slice() {
                        return Err(TrialFailure::RowMismatch {
                            txn_idx,
                            row_idx: r_idx as u32,
                        });
                    }
                }
            }

            running_chain = recomputed;
        }

        recovered_m = txn_idx;
    }

    // Verify prefix bound m in {a, a + 1} and exact phase correspondence.
    let valid_m = match phase {
        KillPhase::BeforeBegin | KillPhase::InsideAfterPut { .. } => recovered_m == last_acked_a,
        KillPhase::AfterCommitBeforeAck => recovered_m == last_acked_a + 1,
    };
    if !valid_m {
        return Err(TrialFailure::PrefixOutOfRange {
            last_acked_a,
            recovered_m,
            phase,
        });
    }

    if !skip_chain_check && recovered_m > 0 {
        let head_raw = ledger
            .get_committed(TableId::Meta, CHAIN_HEAD_KEY)
            .map_err(|e| TrialFailure::HarnessError(format!("read chain head: {e}")))?
            .ok_or_else(|| TrialFailure::ChainMismatch {
                txn_idx: recovered_m,
                detail: "missing CHAIN_HEAD_KEY after non-empty recovery".to_owned(),
            })?;
        let Some((head_idx, head_digest)) = decode_chain_head(&head_raw) else {
            return Err(TrialFailure::ChainMismatch {
                txn_idx: recovered_m,
                detail: "malformed CHAIN_HEAD_KEY bytes".to_owned(),
            });
        };
        if head_idx != recovered_m || head_digest != running_chain {
            return Err(TrialFailure::ChainMismatch {
                txn_idx: recovered_m,
                detail: format!(
                    "CHAIN_HEAD_KEY ({head_idx}, {}) != ({recovered_m}, {})",
                    head_digest.to_hex(),
                    running_chain.to_hex()
                ),
            });
        }
    }

    Ok(recovered_m)
}

fn kill_recovery_child(db: &Path, seed: u64, n_txns: u64, k_rows: u32) -> Result<(), TrialFailure> {
    let seed_s = seed.to_string();
    let txns_s = n_txns.to_string();
    let rows_s = k_rows.to_string();
    let mut child = probe_command(&[
        "--db",
        db.to_str().unwrap_or(""),
        "--seed",
        &seed_s,
        "--txns",
        &txns_s,
        "--rows",
        &rows_s,
        "--mode",
        "recover-kill",
    ])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .map_err(|e| TrialFailure::HarnessError(format!("spawn recover-kill child: {e}")))?;

    let _stdin = child.stdin.take();
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| TrialFailure::HarnessError("recover-kill stdout missing".to_owned()))?;
    let mut guard = ChildGuard::new(child);

    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    let wait_for_at_recover = seed % 2 == 1;

    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .map_err(|e| TrialFailure::HarnessError(format!("read recover-kill stdout: {e}")))?;
        if n == 0 {
            return Err(TrialFailure::HarnessError(
                "recover-kill child exited before SIGKILL".to_owned(),
            ));
        }
        let trimmed = line.trim();
        if trimmed == "RECOVER_BEGIN" {
            if !wait_for_at_recover {
                // Deterministic micro-offset inside open/recovery before SIGKILL.
                let delay_ms = (seed.saturating_sub(90) % 10) * 2;
                if delay_ms > 0 {
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                guard.kill_and_wait().map_err(TrialFailure::HarnessError)?;
                return Ok(());
            }
        } else if trimmed.starts_with("AT_RECOVER") {
            guard.kill_and_wait().map_err(TrialFailure::HarnessError)?;
            return Ok(());
        }
    }
}

fn run_single_trial(
    seed: u64,
    n_txns: u64,
    k_rows: u32,
    plant: PlantMode,
    skip_chain_check: bool,
    kill_during_recovery: bool,
    run_resume: bool,
) -> Result<TrialOutcome, TrialFailure> {
    let dir = scratch_dir(&format!("s{seed}-{}", plant.as_arg()));
    let db = dir.join("ledger.db");
    let expected_kp = kill_point_for_seed(seed, n_txns, k_rows);

    // Steps 1-3: spawn child, read pipe until AT_KILL, SIGKILL immediately, drain pipe for `a`.
    let seed_s = seed.to_string();
    let txns_s = n_txns.to_string();
    let rows_s = k_rows.to_string();
    let mut child = probe_command(&[
        "--db",
        db.to_str().unwrap_or(""),
        "--seed",
        &seed_s,
        "--txns",
        &txns_s,
        "--rows",
        &rows_s,
        "--mode",
        "run-kill",
        "--plant",
        plant.as_arg(),
    ])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .map_err(|e| TrialFailure::HarnessError(format!("spawn run-kill child: {e}")))?;

    let _stdin = child.stdin.take();
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| TrialFailure::HarnessError("child stdout missing".to_owned()))?;
    let mut guard = ChildGuard::new(child);

    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    let mut last_acked_a: u64 = 0;
    let mut killed = false;

    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .map_err(|e| TrialFailure::HarnessError(format!("read child stdout: {e}")))?;
        if n == 0 {
            break;
        }
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("C ") {
            if let Ok(idx) = rest.parse::<u64>() {
                last_acked_a = idx;
            }
        } else if let Some(rest) = trimmed.strip_prefix("AT_KILL ") {
            let expected_ann = format!("{} {}", expected_kp.txn_idx, expected_kp.phase.label());
            if rest != expected_ann {
                return Err(TrialFailure::HarnessError(format!(
                    "seed {seed} kill announcement mismatch: expected `{expected_ann}`, got `{rest}`"
                )));
            }
            guard.kill_and_wait().map_err(TrialFailure::HarnessError)?;
            killed = true;
            // Drain stdout to EOF after child death to read any buffered `C a` lines.
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        if let Some(c_rest) = line.trim().strip_prefix("C ")
                            && let Ok(idx) = c_rest.parse::<u64>()
                        {
                            last_acked_a = idx;
                        }
                    }
                }
            }
            break;
        }
    }

    if !killed {
        return Err(TrialFailure::HarnessError(format!(
            "seed {seed} child exited before announcing AT_KILL"
        )));
    }

    // Step 6 (seeds 90..=99): kill a second child during recovery before parent reopens.
    if kill_during_recovery {
        kill_recovery_child(&db, seed, n_txns, k_rows)?;
    }

    // Step 4: reopen in parent and check assertions 4a, 4b, 4c.
    let reopened = FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default())
        .map_err(|e| TrialFailure::HarnessError(format!("reopen after kill failed: {e}")))?;
    let recovery_report = reopened.recovered();
    let integrity = reopened
        .integrity_check()
        .map_err(|e| TrialFailure::IntegrityNotClean {
            detail: format!("integrity_check error: {e}"),
        })?;
    if !integrity.is_clean() {
        return Err(TrialFailure::IntegrityNotClean {
            detail: format!("{integrity:?}"),
        });
    }

    let recovered_m = verify_prefix_and_chain(
        &reopened,
        seed,
        n_txns,
        k_rows,
        last_acked_a,
        expected_kp.phase,
        plant,
        skip_chain_check,
    )?;

    // Assertion 4c: recovery report consistency with observed prefix.
    if recovered_m == last_acked_a + 1 && recovery_report == RecoveryReport::RolledBackIncompleteTxn
    {
        return Err(TrialFailure::RecoveryInconsistent {
            report: recovery_report,
            last_acked_a,
            recovered_m,
        });
    }

    drop(reopened);

    if !run_resume {
        return Ok(TrialOutcome {
            seed,
            kill_point: expected_kp,
            last_acked_a,
            recovered_m,
            recovery_report,
            integrity_clean: true,
            final_chain_hex: String::new(),
        });
    }

    // Step 5: resume at m + 1 through N uninterrupted and verify final chain == reference.
    let start_from_s = (recovered_m + 1).to_string();
    let mut resume_child = probe_command(&[
        "--db",
        db.to_str().unwrap_or(""),
        "--seed",
        &seed_s,
        "--txns",
        &txns_s,
        "--rows",
        &rows_s,
        "--mode",
        "resume",
        "--start-from",
        &start_from_s,
        "--plant",
        plant.as_arg(),
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .map_err(|e| TrialFailure::HarnessError(format!("spawn resume child: {e}")))?;

    let resume_stdout = resume_child
        .stdout
        .take()
        .ok_or_else(|| TrialFailure::HarnessError("resume stdout missing".to_owned()))?;
    let mut resume_guard = ChildGuard::new(resume_child);

    let mut final_hex = String::new();
    for line_res in BufReader::new(resume_stdout).lines() {
        let l =
            line_res.map_err(|e| TrialFailure::HarnessError(format!("read resume stdout: {e}")))?;
        if let Some(hex) = l.trim().strip_prefix("DONE ") {
            final_hex = hex.to_owned();
        }
    }
    resume_guard
        .wait_success()
        .map_err(TrialFailure::HarnessError)?;

    let final_ledger = FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default())
        .map_err(|e| TrialFailure::HarnessError(format!("reopen final ledger: {e}")))?;
    let final_integrity =
        final_ledger
            .integrity_check()
            .map_err(|e| TrialFailure::IntegrityNotClean {
                detail: format!("final integrity_check error: {e}"),
            })?;
    if !final_integrity.is_clean() {
        return Err(TrialFailure::IntegrityNotClean {
            detail: format!("final integrity_check not clean: {final_integrity:?}"),
        });
    }

    let final_m = verify_prefix_and_chain(
        &final_ledger,
        seed,
        n_txns,
        k_rows,
        n_txns,
        KillPhase::BeforeBegin,
        plant,
        skip_chain_check,
    )?;
    if final_m != n_txns {
        return Err(TrialFailure::PrefixOutOfRange {
            last_acked_a: n_txns,
            recovered_m: final_m,
            phase: KillPhase::BeforeBegin,
        });
    }

    if !skip_chain_check {
        let expected_ref = reference_chain_digest(seed, n_txns, k_rows).to_hex();
        if final_hex != expected_ref {
            return Err(TrialFailure::ChainMismatch {
                txn_idx: n_txns,
                detail: format!("resumed final chain {final_hex} != reference {expected_ref}"),
            });
        }
    }

    Ok(TrialOutcome {
        seed,
        kill_point: expected_kp,
        last_acked_a,
        recovered_m,
        recovery_report,
        integrity_clean: true,
        final_chain_hex: final_hex,
    })
}

mod ledger {
    use super::*;

    #[test]
    #[ignore = "BD-009 gate command runs 100 forced-kill trials (seeds 0..99, 10 kills during recovery)"]
    fn forced_kill_100() {
        let mut passed: u64 = 0;
        let mut failed: u64 = 0;
        let mut first_err: Option<(u64, TrialFailure)> = None;

        for seed in 0u64..100 {
            let kill_during_recovery = seed >= 90;
            match run_single_trial(
                seed,
                DEFAULT_TXN_COUNT,
                DEFAULT_ROWS_PER_TXN,
                PlantMode::None,
                false,
                kill_during_recovery,
                true,
            ) {
                Ok(out) => {
                    passed += 1;
                    // Structured trial log line (never includes row payloads).
                    eprintln!(
                        "trial seed={} kill_txn={} kill_phase={} kill_during_recovery={} a={} m={} recovered={} integrity_clean={} final_digest={}",
                        out.seed,
                        out.kill_point.txn_idx,
                        out.kill_point.phase,
                        kill_during_recovery,
                        out.last_acked_a,
                        out.recovered_m,
                        out.recovery_report,
                        out.integrity_clean,
                        out.final_chain_hex,
                    );
                }
                Err(err) => {
                    failed += 1;
                    eprintln!("trial seed={seed} FAILED: {err:?}");
                    if first_err.is_none() {
                        first_err = Some((seed, err));
                    }
                }
            }
        }

        eprintln!("metric passed={passed} failed={failed} threshold=100");
        assert_eq!(
            (passed, failed),
            (100, 0),
            "forced_kill_100 must pass 100 of 100 trials; first failure: {first_err:?}"
        );
    }

    #[test]
    fn forced_kill_reproduces() {
        let run_a = run_single_trial(
            17,
            DEFAULT_TXN_COUNT,
            DEFAULT_ROWS_PER_TXN,
            PlantMode::None,
            false,
            false,
            true,
        )
        .expect("seed 17 first run must pass");
        let run_b = run_single_trial(
            17,
            DEFAULT_TXN_COUNT,
            DEFAULT_ROWS_PER_TXN,
            PlantMode::None,
            false,
            false,
            true,
        )
        .expect("seed 17 second run must pass");

        assert_eq!(
            run_a.kill_point, run_b.kill_point,
            "kill point must reproduce"
        );
        assert_eq!(
            run_a.last_acked_a, run_b.last_acked_a,
            "acked index a must reproduce"
        );
        assert_eq!(
            run_a.recovered_m, run_b.recovered_m,
            "recovered m must reproduce"
        );
        assert_eq!(
            run_a.final_chain_hex, run_b.final_chain_hex,
            "final chain value must reproduce"
        );
        assert_eq!(
            run_a.final_chain_hex, GOLDEN_SEED_17_N200_K8,
            "seed 17 resumed chain must equal golden reference"
        );
    }

    #[test]
    fn control_split_writer_caught() {
        let mut caught_partial: Vec<(u64, u64, u32)> = Vec::new();

        for seed in 0u64..100 {
            let res = run_single_trial(
                seed,
                DEFAULT_TXN_COUNT,
                DEFAULT_ROWS_PER_TXN,
                PlantMode::SplitWriter,
                false,
                false,
                false,
            );
            if let Err(TrialFailure::PartialTransaction {
                txn_idx,
                present_rows,
                expected_rows,
            }) = res
            {
                assert_eq!(
                    present_rows,
                    expected_rows / 2,
                    "split-writer partial transaction at seed {seed} txn {txn_idx} must leave k/2 rows"
                );
                caught_partial.push((seed, txn_idx, present_rows));
            }
        }

        assert!(
            !caught_partial.is_empty(),
            "negative control FAILED: --plant split-writer must leave a k/2 partial transaction in at least one of 100 trials"
        );
        let (first_seed, first_txn, first_rows) = caught_partial[0];
        eprintln!(
            "control split-writer caught_trials={} first_seed={} first_txn={} present_rows={}",
            caught_partial.len(),
            first_seed,
            first_txn,
            first_rows
        );
    }

    #[test]
    fn control_row_swap_caught() {
        // 1. With chain verification in force, --plant row-swap MUST fail on txn 1.
        let caught = run_single_trial(
            17,
            DEFAULT_TXN_COUNT,
            DEFAULT_ROWS_PER_TXN,
            PlantMode::RowSwap,
            false,
            false,
            true,
        )
        .expect_err("chain verification must catch --plant row-swap");
        assert!(
            matches!(caught, TrialFailure::ChainMismatch { txn_idx: 1, .. }),
            "expected ChainMismatch on swapped transaction 1, got: {caught:?}"
        );

        // 2. With --skip-chain-check, the exact same planted row swap passes,
        //    proving the chain assertion is what catches the defect.
        let skipped = run_single_trial(
            17,
            DEFAULT_TXN_COUNT,
            DEFAULT_ROWS_PER_TXN,
            PlantMode::RowSwap,
            true,
            false,
            true,
        )
        .expect("with skip_chain_check=true, --plant row-swap must pass structural prefix check");
        assert!(skipped.integrity_clean);
    }

    #[test]
    fn uninterrupted_reference_digest() {
        // Pure reference computation matches checked-in golden digest (verified by
        // tests/admission/recompute_int_fsql_01_chain.py against b3sum 1.8.7).
        let pure_hex = reference_chain_digest(0, DEFAULT_TXN_COUNT, DEFAULT_ROWS_PER_TXN).to_hex();
        assert_eq!(pure_hex, GOLDEN_SEED_0_N200_K8);

        // Real ledger uninterrupted run produces the exact same digest on disk.
        let dir = scratch_dir("uninterrupted-seed0");
        let db = dir.join("ledger.db");
        let txns_s = DEFAULT_TXN_COUNT.to_string();
        let rows_s = DEFAULT_ROWS_PER_TXN.to_string();
        let output = probe_command(&[
            "--db",
            db.to_str().unwrap_or(""),
            "--seed",
            "0",
            "--txns",
            &txns_s,
            "--rows",
            &rows_s,
            "--mode",
            "uninterrupted",
        ])
        .output()
        .expect("run uninterrupted probe");
        assert!(
            output.status.success(),
            "uninterrupted probe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        let done_hex = stdout
            .lines()
            .find_map(|l| l.trim().strip_prefix("DONE "))
            .expect("uninterrupted probe must emit DONE <hex>");
        assert_eq!(done_hex, GOLDEN_SEED_0_N200_K8);

        let reopened = FsqliteLedger::open(&db, SchemaVersion::M0, OpenOptions::default())
            .expect("reopen uninterrupted ledger");
        assert!(reopened.integrity_check().expect("integrity").is_clean());
        let head_raw = reopened
            .get_committed(TableId::Meta, CHAIN_HEAD_KEY)
            .expect("read head")
            .expect("head present");
        let (head_idx, head_digest) = decode_chain_head(&head_raw).expect("decode head");
        assert_eq!(head_idx, DEFAULT_TXN_COUNT);
        assert_eq!(head_digest.to_hex(), GOLDEN_SEED_0_N200_K8);
    }
}
