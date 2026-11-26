#![deny(unsafe_code)]

//! End-to-end harness integration lane for `BD-014` (`K-selftest` on `BatchCpu`
//! across 1, 2, 8, and 36 workers), emitting STD-E2E `frob.e2elog/1` NDJSON to
//! `target/e2e-logs/spark_harness_selftest/<run_id>.ndjson`.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use frob_core::canon::{CanonError, CanonWriter, Object};
use frob_core::id::hash_blob;
use frob_core::registry::refusals::RefusalRegistry;
use frob_kernels::backend::Cx;
use frob_kernels::cpu::BatchCpu;
use frob_kernels::harness::Harness;
use frob_kernels::selftest::{
    k_selftest_entry, k_selftest_output_specs, k_selftest_scalar_reference,
    k_selftest_seeded_inputs, k_selftest_shape_class,
};

const SCRIPT_NAME: &str = "spark_harness_selftest";
const BEAD_ID: &str = "BD-014";
const SEED_STR: &str = "1";

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

fn now_utc_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (yoe as i64 + era * 400 + i64::from(m <= 2), m, d)
}

fn format_utc_rfc3339_ms(epoch_ms: i64) -> String {
    let (secs, ms) = (epoch_ms.div_euclid(1000), epoch_ms.rem_euclid(1000));
    let (y, m, d) = civil_from_days(secs.div_euclid(86_400));
    let rem = secs.rem_euclid(86_400);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}.{ms:03}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

fn format_utc_compact(epoch_ms: i64) -> String {
    let secs = epoch_ms.div_euclid(1000);
    let (y, m, d) = civil_from_days(secs.div_euclid(86_400));
    let rem = secs.rem_euclid(86_400);
    format!(
        "{y:04}{m:02}{d:02}T{:02}{:02}{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

fn first_diff_lane(lhs: &[u8], rhs: &[u8]) -> Option<usize> {
    let lanes = lhs.len().min(rhs.len()) / 4;
    for lane in 0..lanes {
        let off = lane * 4;
        if lhs[off..off + 4] != rhs[off..off + 4] {
            return Some(lane);
        }
    }
    if lhs.len() != rhs.len() {
        Some(lanes)
    } else {
        None
    }
}

struct E2eLog {
    file: File,
    run_id: String,
    seq: u64,
    dir: PathBuf,
}

impl E2eLog {
    fn open(dir: PathBuf) -> Self {
        fs::create_dir_all(&dir).expect("create e2e log dir");
        let stamp = format_utc_compact(now_utc_ms());
        let pid = std::process::id();
        loop {
            let n = RUN_COUNTER.fetch_add(1, Ordering::Relaxed);
            let run_id = format!("{stamp}-p{pid}-{n}");
            let path = dir.join(format!("{run_id}.ndjson"));
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(file) => {
                    return Self {
                        file,
                        run_id,
                        seq: 0,
                        dir,
                    };
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("create e2e log {}: {e}", path.display()),
            }
        }
    }

    fn emit<F>(&mut self, step: &str, event: &str, extra: F)
    where
        F: FnOnce(&mut Object<'_>) -> Result<(), CanonError>,
    {
        let seq = self.seq;
        self.seq += 1;
        let ts = format_utc_rfc3339_ms(now_utc_ms());
        let mut w = CanonWriter::new();
        w.value()
            .expect("root value")
            .object(|o| {
                o.str("schema", "frob.e2elog/1")?;
                o.str("ts", &ts)?;
                o.int53("seq", seq as i64)?;
                o.str("run_id", &self.run_id)?;
                o.str("script", SCRIPT_NAME)?;
                o.str("bead", BEAD_ID)?;
                o.str("step", step)?;
                o.str("seed", SEED_STR)?;
                o.str("event", event)?;
                extra(o)
            })
            .expect("serialize e2e log line");
        let bytes = w.finish().expect("finish e2e line");
        self.file
            .write_all(bytes.as_bytes())
            .expect("write log line");
        self.file.write_all(b"\n").expect("write newline");
    }

    fn path(&self) -> PathBuf {
        self.dir.join(format!("{}.ndjson", self.run_id))
    }

    fn body_sha256(&self) -> String {
        let mut command = if cfg!(target_os = "macos") {
            let mut shasum = Command::new("shasum");
            shasum.args(["-a", "256"]);
            shasum
        } else {
            Command::new("sha256sum")
        };
        let output = command.arg(self.path()).output().expect("hash log body");
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stdout).expect("SHA-256 output");
        let digest = text.split_whitespace().next().expect("body digest");
        assert_eq!(digest.len(), 64);
        digest.to_owned()
    }

    fn dump_mismatch(
        &self,
        workers: usize,
        slab_name: &str,
        actual: &[u8],
        expected: &[u8],
    ) -> (PathBuf, PathBuf) {
        let act = self
            .dir
            .join(format!("{}_w{workers}_{slab_name}_actual.bin", self.run_id));
        let exp = self.dir.join(format!(
            "{}_w{workers}_{slab_name}_expected.bin",
            self.run_id
        ));
        let _ = fs::write(&act, actual);
        let _ = fs::write(&exp, expected);
        (act, exp)
    }
}

fn emit_assert(
    log: &mut E2eLog,
    step: &str,
    name: &str,
    kind: &str,
    ok: bool,
    observed: &str,
    expected: &str,
    diff_artifacts: Option<(&Path, &Path)>,
) {
    log.emit(step, "assert", |o| {
        o.str("name", name)?;
        o.str("kind", kind)?;
        o.str("status", if ok { "pass" } else { "fail" })?;
        o.str("observed", observed)?;
        o.str("expected", expected)?;
        if !ok {
            o.member("first_diff", |v| {
                v.object(|d| {
                    d.str("path", name)?;
                    d.str("observed", observed)?;
                    d.str("expected", expected)
                })
            })?;
            o.member("repro", |v| {
                v.object(|r| {
                    r.int53("cmd_seq", 0)?;
                    r.member("artifacts", |av| {
                        av.array(|a| {
                            if let Some((act, exp)) = diff_artifacts {
                                a.push(|s| {
                                    s.str(&act.display().to_string());
                                    Ok(())
                                })?;
                                a.push(|s| {
                                    s.str(&exp.display().to_string());
                                    Ok(())
                                })?;
                            }
                            Ok(())
                        })
                    })
                })
            })?;
        }
        Ok(())
    });
}

#[test]
fn spark_harness_selftest_e2e() {
    run_harness_selftest(None);
}

#[test]
fn spark_harness_logs_refusal_as_failed_assertion() {
    run_harness_selftest(Some(2));
}

fn run_harness_selftest(refusal_workers: Option<usize>) {
    let lanes = 4_096usize;
    let inputs = k_selftest_seeded_inputs(lanes, 0, 1);
    let expected = k_selftest_scalar_reference(&inputs);
    let expected_f = expected.out_f_bytes();
    let expected_u = expected.out_u_bytes();
    let h_inputs = inputs.harness_inputs();
    let h_outputs = k_selftest_output_specs(lanes);
    let class = k_selftest_shape_class(lanes);

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf();
    let log_dir = repo_root.join("target").join("e2e-logs").join(SCRIPT_NAME);
    let mut log = E2eLog::open(log_dir);

    let host_fp =
        hash_blob(format!("{}:{}", std::env::consts::OS, std::env::consts::ARCH).as_bytes())
            .to_hex();
    log.emit("", "start", |o| {
        o.str("commit", "workspace")?;
        o.bool("dirty", false)?;
        o.str("toolchain", "rust-toolchain.toml")?;
        o.str("frob_version", env!("CARGO_PKG_VERSION"))?;
        o.str("scale", "full")?;
        o.str(
            "plant",
            if refusal_workers.is_some() {
                "unavailable-backend"
            } else {
                ""
            },
        )?;
        o.member("devices", |v| {
            v.array(|a| {
                a.push(|s| {
                    s.str("batch-cpu");
                    Ok(())
                })
            })
        })?;
        o.member("argv", |v| {
            v.array(|a| {
                for tok in [
                    "cargo",
                    "test",
                    "-p",
                    "frob-kernels",
                    "--release",
                    "--test",
                    "harness_e2e",
                ] {
                    a.push(|s| {
                        s.str(tok);
                        Ok(())
                    })?;
                }
                Ok(())
            })
        })?;
        o.member("host", |v| {
            v.object(|h| {
                h.str("os", std::env::consts::OS)?;
                h.str("arch", std::env::consts::ARCH)?;
                h.str("fingerprint", &host_fp[..16])
            })
        })
    });

    let mut passed = 0u64;
    let mut failed = 0u64;
    let mut refusals_unexpected = 0u64;
    let mut ref_f_digest: Option<String> = None;
    let mut ref_u_digest: Option<String> = None;
    let mut failures: Vec<String> = Vec::new();

    for workers in [1usize, 2, 8, 36] {
        let step = format!("workers_{workers}");
        let t0 = Instant::now();
        let mut step_passed = 0u64;
        let mut step_failed = 0u64;

        log.emit(&step, "start", |o| {
            o.str(
                "title",
                &format!("Run K-selftest on BatchCpu with {workers} workers"),
            )
        });

        let mut cpu = BatchCpu::new(workers).with_unavailable(refusal_workers == Some(workers));
        match Harness::run(
            &mut cpu,
            k_selftest_entry(),
            &h_inputs,
            &h_outputs,
            &class,
            &Cx::new(),
        ) {
            Err(refusal) => {
                refusals_unexpected += 1;
                step_failed += 1;
                failed += 1;
                failures.push(format!(
                    "workers={workers}: unexpected refusal {}",
                    refusal.code.as_str()
                ));
                let exit_code = RefusalRegistry::load()
                    .ok()
                    .and_then(|reg| {
                        reg.lookup(refusal.code.as_str(), "harness_e2e")
                            .ok()
                            .map(|row| i64::from(row.exit))
                    })
                    .unwrap_or(1);
                log.emit(&step, "refusal", |o| {
                    o.str("code", refusal.code.as_str())?;
                    o.str("reason", &refusal.message)?;
                    o.int53("exit", exit_code)?;
                    o.bool("expected", false)
                });
                emit_assert(
                    &mut log,
                    &step,
                    "harness.run",
                    "refusal",
                    false,
                    refusal.code.as_str(),
                    "successful harness run",
                    None,
                );
            }
            Ok(res) => {
                let wall_ms = t0.elapsed().as_millis() as i64;
                let f_hex = res.digests["out_f"].to_hex();
                let u_hex = res.digests["out_u"].to_hex();

                log.emit(&step, "metric", |o| {
                    o.str("name", "harness_wall_ms")?;
                    o.int53("value", wall_ms)?;
                    o.str("unit", "ms")?;
                    o.int53("workers", workers as i64)?;
                    o.str("out_f_digest", &f_hex)?;
                    o.str("out_u_digest", &u_hex)?;
                    o.member("evidence", |ev| {
                        ev.object(|eo| {
                            for (id, _, val) in res.receipt.ae_entries() {
                                eo.str(id, val.unwrap_or(""))?;
                            }
                            Ok(())
                        })
                    })
                });

                let receipt_ok = res.receipt.validate().is_ok();
                emit_assert(
                    &mut log,
                    &step,
                    "receipt_valid",
                    "check",
                    receipt_ok,
                    if receipt_ok { "valid" } else { "invalid" },
                    "valid",
                    None,
                );

                let f_diff = first_diff_lane(&res.slabs["out_f"], &expected_f);
                let f_ok = f_diff.is_none();
                let f_dumps = (!f_ok)
                    .then(|| log.dump_mismatch(workers, "out_f", &res.slabs["out_f"], &expected_f));
                emit_assert(
                    &mut log,
                    &step,
                    "out_f_bit_equal",
                    "eq",
                    f_ok,
                    &f_diff.map_or_else(|| "equal".into(), |i| format!("first_diff_lane={i}")),
                    "equal",
                    f_dumps.as_ref().map(|(a, e)| (a.as_path(), e.as_path())),
                );

                let u_diff = first_diff_lane(&res.slabs["out_u"], &expected_u);
                let u_ok = u_diff.is_none();
                let u_dumps = (!u_ok)
                    .then(|| log.dump_mismatch(workers, "out_u", &res.slabs["out_u"], &expected_u));
                emit_assert(
                    &mut log,
                    &step,
                    "out_u_bit_equal",
                    "eq",
                    u_ok,
                    &u_diff.map_or_else(|| "equal".into(), |i| format!("first_diff_lane={i}")),
                    "equal",
                    u_dumps.as_ref().map(|(a, e)| (a.as_path(), e.as_path())),
                );

                let digest_ok = match (&ref_f_digest, &ref_u_digest) {
                    (None, None) => {
                        ref_f_digest = Some(f_hex.clone());
                        ref_u_digest = Some(u_hex.clone());
                        true
                    }
                    (Some(rf), Some(ru)) => rf == &f_hex && ru == &u_hex,
                    _ => false,
                };
                emit_assert(
                    &mut log,
                    &step,
                    "worker_digest_invariant",
                    "digest",
                    digest_ok,
                    &format!("{f_hex}:{u_hex}"),
                    &format!(
                        "{}:{}",
                        ref_f_digest.as_deref().unwrap_or(""),
                        ref_u_digest.as_deref().unwrap_or("")
                    ),
                    None,
                );

                for (label, ok) in [
                    ("receipt invalid", receipt_ok),
                    ("out_f diverged from scalar loop", f_ok),
                    ("out_u diverged from scalar loop", u_ok),
                    ("digest diverged across worker counts", digest_ok),
                ] {
                    if ok {
                        step_passed += 1;
                        passed += 1;
                    } else {
                        step_failed += 1;
                        failed += 1;
                        failures.push(format!("workers={workers}: {label}"));
                    }
                }
            }
        }

        let step_ms = t0.elapsed().as_millis() as i64;
        log.emit(&step, "end", |o| {
            o.int53("wall_ms", step_ms)?;
            o.int53("asserts_passed", step_passed as i64)?;
            o.int53("asserts_failed", step_failed as i64)
        });
    }

    // Always emit the summary record before asserting so failure evidence is never lost.
    let body_sha256 = log.body_sha256();
    log.emit("", "summary", |o| {
        o.str("body_sha256", &body_sha256)?;
        o.int53("asserts_passed", passed as i64)?;
        o.int53("asserts_failed", failed as i64)?;
        o.int53("asserts_skipped", 0)?;
        o.int53("controls_caught", 0)?;
        o.int53("controls_missed", 0)?;
        o.int53("refusals_expected", 0)?;
        o.int53("refusals_unexpected", refusals_unexpected as i64)?;
        o.str("registries", "of-record")?;
        o.bool("gates_claimable", false)?;
        o.member("gates", |v| v.array(|_| Ok(())))
    });

    let path = log.path();
    println!("BD-014 harness log: {}", path.display());
    let log_text = fs::read_to_string(&path).expect("read completed harness log");
    let mut lines = log_text.lines();
    let summary = lines.next_back().expect("summary");
    assert!(summary.contains(&format!("\"body_sha256\":\"{body_sha256}\"")));
    let refusal_asserts = lines
        .filter(|line| {
            line.contains("\"event\":\"assert\"")
                && line.contains("\"name\":\"harness.run\"")
                && line.contains("\"status\":\"fail\"")
                && line.contains("\"observed\":\"FROB-REF-601\"")
        })
        .count();
    let expected_refusals = u64::from(refusal_workers.is_some());
    assert_eq!(refusal_asserts as u64, expected_refusals);
    assert_eq!(
        (passed, failed, refusals_unexpected, failures.len() as u64),
        (
            16 - 4 * expected_refusals,
            expected_refusals,
            expected_refusals,
            expected_refusals
        ),
        "unexpected harness results: {failures:?}"
    );
}
