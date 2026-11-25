#![forbid(unsafe_code)]

//! Shared child-process probe logic for `BD-009` (`INT-FSQL-01`).
//!
//! Invoked both by the standalone `ledger_kill_probe` example binary and by
//! `ledger_forced_kill` child processes spawned via `std::env::current_exe()`.
//! Ships in no product binary.

use std::io::{self, Read, Write};
use std::path::PathBuf;

use frob_core::id::Blake3;
use frob_fsim::ledger::probe::{
    CHAIN_HEAD_KEY, DEFAULT_ROWS_PER_TXN, DEFAULT_TXN_COUNT, KillPhase, KillPoint, ROW_VALUE_LEN,
    chain_key, chain_step, encode_chain_head, kill_point_for_seed, row_key, row_value,
    table_for_row, zero_chain,
};
use frob_fsim::ledger::{FsqliteLedger, Ledger, LedgerTxn, OpenOptions, SchemaVersion, TableId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    RunKill,
    Resume,
    RecoverKill,
    Uninterrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Plant {
    None,
    SplitWriter,
    RowSwap,
}

struct Args {
    db: PathBuf,
    seed: u64,
    txns: u64,
    rows: u32,
    mode: Mode,
    start_from: u64,
    plant: Plant,
}

fn parse_args<I>(args: I) -> Result<Args, String>
where
    I: IntoIterator<Item = String>,
{
    let mut db: Option<PathBuf> = None;
    let mut seed: Option<u64> = None;
    let mut txns: u64 = DEFAULT_TXN_COUNT;
    let mut rows: u32 = DEFAULT_ROWS_PER_TXN;
    let mut mode: Option<Mode> = None;
    let mut start_from: u64 = 1;
    let mut plant = Plant::None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--db" => {
                let v = iter.next().ok_or("--db requires a path")?;
                db = Some(PathBuf::from(v));
            }
            "--seed" => {
                let v = iter.next().ok_or("--seed requires u64")?;
                seed = Some(v.parse::<u64>().map_err(|e| format!("bad --seed: {e}"))?);
            }
            "--txns" => {
                let v = iter.next().ok_or("--txns requires u64")?;
                txns = v.parse::<u64>().map_err(|e| format!("bad --txns: {e}"))?;
            }
            "--rows" => {
                let v = iter.next().ok_or("--rows requires u32")?;
                rows = v.parse::<u32>().map_err(|e| format!("bad --rows: {e}"))?;
            }
            "--mode" => {
                let v = iter.next().ok_or("--mode requires a value")?;
                mode = Some(match v.as_str() {
                    "run-kill" => Mode::RunKill,
                    "resume" => Mode::Resume,
                    "recover-kill" => Mode::RecoverKill,
                    "uninterrupted" => Mode::Uninterrupted,
                    other => return Err(format!("unknown --mode `{other}`")),
                });
            }
            "--start-from" => {
                let v = iter.next().ok_or("--start-from requires u64")?;
                start_from = v
                    .parse::<u64>()
                    .map_err(|e| format!("bad --start-from: {e}"))?;
            }
            "--plant" => {
                let v = iter.next().ok_or("--plant requires a value")?;
                plant = match v.as_str() {
                    "none" => Plant::None,
                    "split-writer" => Plant::SplitWriter,
                    "row-swap" => Plant::RowSwap,
                    other => return Err(format!("unknown --plant `{other}`")),
                };
            }
            other => return Err(format!("unknown argument `{other}`")),
        }
    }

    Ok(Args {
        db: db.ok_or("missing required --db")?,
        seed: seed.ok_or("missing required --seed")?,
        txns,
        rows,
        mode: mode.ok_or("missing required --mode")?,
        start_from,
        plant,
    })
}

fn emit_line(out: &mut impl Write, line: &str) -> Result<(), String> {
    writeln!(out, "{line}").map_err(|e| format!("write stdout: {e}"))?;
    out.flush().map_err(|e| format!("flush stdout: {e}"))
}

fn wait_for_kill_signal() {
    let mut stdin = io::stdin().lock();
    let mut buf = [0u8; 64];
    loop {
        match stdin.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
    }
}

fn load_prev_chain(ledger: &FsqliteLedger, seed: u64, start_from: u64) -> Result<Blake3, String> {
    if start_from <= 1 {
        return Ok(zero_chain());
    }
    let prev_idx = start_from - 1;
    let raw = ledger
        .get_committed(TableId::Meta, &chain_key(seed, prev_idx))
        .map_err(|e| format!("read prev chain t{prev_idx}: {e}"))?
        .ok_or_else(|| format!("missing committed chain entry for transaction {prev_idx}"))?;
    if raw.len() != 32 {
        return Err(format!(
            "corrupt prev chain length {} for transaction {prev_idx}",
            raw.len()
        ));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&raw);
    Ok(Blake3::from_bytes(arr))
}

fn compute_txn_rows(
    seed: u64,
    txn_idx: u64,
    k_rows: u32,
    prev_chain: Blake3,
    plant: Plant,
) -> (Vec<[u8; ROW_VALUE_LEN]>, Blake3) {
    let mut vals: Vec<[u8; ROW_VALUE_LEN]> =
        (0..k_rows).map(|r| row_value(seed, txn_idx, r)).collect();
    let slices: Vec<&[u8]> = vals.iter().map(|v| v.as_slice()).collect();
    let next_chain = chain_step(prev_chain, &slices);

    // Planted defect 2 (`--plant row-swap`): exchange two rows of transaction 1
    // AFTER the chain digest has been computed from the unswapped rows.
    if plant == Plant::RowSwap && txn_idx == 1 && vals.len() >= 3 {
        vals.swap(1, 2);
    }

    (vals, next_chain)
}

fn run_transactions(
    ledger: &mut FsqliteLedger,
    args: &Args,
    kill_point: Option<KillPoint>,
    out: &mut impl Write,
) -> Result<Blake3, String> {
    let mut chain = load_prev_chain(ledger, args.seed, args.start_from)?;

    for txn_idx in args.start_from..=args.txns {
        let at_this_txn = kill_point.filter(|kp| kp.txn_idx == txn_idx);

        if kill_point.is_some() {
            emit_line(out, &format!("B {txn_idx}"))?;
        }

        if let Some(kp) = at_this_txn
            && kp.phase == KillPhase::BeforeBegin
        {
            emit_line(out, &format!("AT_KILL {} {}", kp.txn_idx, kp.phase.label()))?;
            wait_for_kill_signal();
            return Ok(chain);
        }

        let (vals, next_chain) = compute_txn_rows(args.seed, txn_idx, args.rows, chain, args.plant);

        if args.plant == Plant::SplitWriter {
            // Planted defect 1 (`--plant split-writer`): commit each logical
            // transaction as two separate auto-committed halves of k/2 rows.
            let half = (args.rows / 2).max(1);

            let mut txn1 = ledger
                .begin()
                .map_err(|e| format!("split begin half 1 at t{txn_idx}: {e}"))?;
            for r in 0..half {
                txn1.put(
                    table_for_row(r),
                    &row_key(args.seed, txn_idx, r),
                    &vals[r as usize],
                )
                .map_err(|e| format!("split put half 1 t{txn_idx} r{r}: {e}"))?;
                if let Some(kp) = at_this_txn
                    && let KillPhase::InsideAfterPut { put_count } = kp.phase
                    && r + 1 == put_count
                    && put_count < half
                {
                    emit_line(out, &format!("AT_KILL {} {}", kp.txn_idx, kp.phase.label()))?;
                    wait_for_kill_signal();
                    return Ok(chain);
                }
            }
            txn1.commit()
                .map_err(|e| format!("split commit half 1 at t{txn_idx}: {e}"))?;

            if let Some(kp) = at_this_txn
                && let KillPhase::InsideAfterPut { put_count } = kp.phase
                && put_count == half
            {
                emit_line(out, &format!("AT_KILL {} {}", kp.txn_idx, kp.phase.label()))?;
                wait_for_kill_signal();
                return Ok(chain);
            }

            let mut txn2 = ledger
                .begin()
                .map_err(|e| format!("split begin half 2 at t{txn_idx}: {e}"))?;
            for r in half..args.rows {
                txn2.put(
                    table_for_row(r),
                    &row_key(args.seed, txn_idx, r),
                    &vals[r as usize],
                )
                .map_err(|e| format!("split put half 2 t{txn_idx} r{r}: {e}"))?;
                if let Some(kp) = at_this_txn
                    && let KillPhase::InsideAfterPut { put_count } = kp.phase
                    && r + 1 == put_count
                {
                    emit_line(out, &format!("AT_KILL {} {}", kp.txn_idx, kp.phase.label()))?;
                    wait_for_kill_signal();
                    return Ok(chain);
                }
            }
            txn2.put(
                TableId::Meta,
                &chain_key(args.seed, txn_idx),
                next_chain.as_bytes(),
            )
            .map_err(|e| format!("split put chain t{txn_idx}: {e}"))?;
            let head = encode_chain_head(txn_idx, next_chain);
            txn2.put(TableId::Meta, CHAIN_HEAD_KEY, &head)
                .map_err(|e| format!("split put head t{txn_idx}: {e}"))?;
            txn2.commit()
                .map_err(|e| format!("split commit half 2 at t{txn_idx}: {e}"))?;
        } else {
            let mut txn = ledger
                .begin()
                .map_err(|e| format!("begin t{txn_idx}: {e}"))?;
            for (r_idx, val) in vals.iter().enumerate() {
                let r = r_idx as u32;
                txn.put(table_for_row(r), &row_key(args.seed, txn_idx, r), val)
                    .map_err(|e| format!("put t{txn_idx} r{r}: {e}"))?;
                if let Some(kp) = at_this_txn
                    && let KillPhase::InsideAfterPut { put_count } = kp.phase
                    && r + 1 == put_count
                {
                    emit_line(out, &format!("AT_KILL {} {}", kp.txn_idx, kp.phase.label()))?;
                    wait_for_kill_signal();
                    return Ok(chain);
                }
            }
            txn.put(
                TableId::Meta,
                &chain_key(args.seed, txn_idx),
                next_chain.as_bytes(),
            )
            .map_err(|e| format!("put chain t{txn_idx}: {e}"))?;
            let head = encode_chain_head(txn_idx, next_chain);
            txn.put(TableId::Meta, CHAIN_HEAD_KEY, &head)
                .map_err(|e| format!("put head t{txn_idx}: {e}"))?;
            txn.commit()
                .map_err(|e| format!("commit t{txn_idx}: {e}"))?;
        }

        chain = next_chain;

        if let Some(kp) = at_this_txn
            && kp.phase == KillPhase::AfterCommitBeforeAck
        {
            emit_line(out, &format!("AT_KILL {} {}", kp.txn_idx, kp.phase.label()))?;
            wait_for_kill_signal();
            return Ok(chain);
        }

        if kill_point.is_some() {
            emit_line(out, &format!("C {txn_idx}"))?;
        }
    }

    Ok(chain)
}

/// Run the forced-kill probe from CLI-style arguments (`--db`, `--seed`, `--mode`, ...).
pub fn run_from_args<I>(raw_args: I) -> Result<(), String>
where
    I: IntoIterator<Item = String>,
{
    let args = parse_args(raw_args)?;
    let mut stdout = io::stdout().lock();

    match args.mode {
        Mode::RecoverKill => {
            emit_line(&mut stdout, "RECOVER_BEGIN")?;
            let ledger = FsqliteLedger::open(&args.db, SchemaVersion::M0, OpenOptions::default())
                .map_err(|e| format!("recover-kill open failed: {e}"))?;
            let report = ledger
                .integrity_check()
                .map_err(|e| format!("recover-kill integrity_check failed: {e}"))?;
            if !report.is_clean() {
                return Err(format!("recover-kill integrity not clean: {report:?}"));
            }
            emit_line(
                &mut stdout,
                &format!("AT_RECOVER {}", ledger.recovered().as_str()),
            )?;
            wait_for_kill_signal();
            Ok(())
        }
        Mode::RunKill => {
            let mut ledger =
                FsqliteLedger::open(&args.db, SchemaVersion::M0, OpenOptions::default())
                    .map_err(|e| format!("run-kill open failed: {e}"))?;
            let kp = kill_point_for_seed(args.seed, args.txns, args.rows);
            let final_chain = run_transactions(&mut ledger, &args, Some(kp), &mut stdout)?;
            emit_line(&mut stdout, &format!("DONE {}", final_chain.to_hex()))?;
            Ok(())
        }
        Mode::Resume => {
            let mut ledger =
                FsqliteLedger::open(&args.db, SchemaVersion::M0, OpenOptions::default())
                    .map_err(|e| format!("resume open failed: {e}"))?;
            let final_chain = run_transactions(&mut ledger, &args, None, &mut stdout)?;
            emit_line(&mut stdout, &format!("DONE {}", final_chain.to_hex()))?;
            Ok(())
        }
        Mode::Uninterrupted => {
            let mut ledger =
                FsqliteLedger::open(&args.db, SchemaVersion::M0, OpenOptions::default())
                    .map_err(|e| format!("uninterrupted open failed: {e}"))?;
            let final_chain = run_transactions(&mut ledger, &args, None, &mut stdout)?;
            emit_line(&mut stdout, &format!("DONE {}", final_chain.to_hex()))?;
            Ok(())
        }
    }
}
