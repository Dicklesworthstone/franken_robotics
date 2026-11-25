#![deny(unsafe_code)]

//! End-to-end harness integration lane for `BD-014` (`K-selftest` on `BatchCpu`
//! across 1, 2, 8, and 36 workers), logging structured STD-E2E NDJSON to
//! `target/e2e-logs/spark_harness_selftest/<utc-timestamp>.ndjson`.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use frob_kernels::backend::Cx;
use frob_kernels::cpu::BatchCpu;
use frob_kernels::harness::{Harness, HarnessInput, HarnessOutputSpec};
use frob_kernels::manifest::{ElemType, ShapeClassCaps};
use frob_kernels::selftest::{
    k_selftest_entry, k_selftest_scalar_reference, k_selftest_seeded_inputs,
};

fn utc_stamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}Z")
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
            c => out.push(c),
        }
    }
    out.push('"');
    out
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

#[test]
fn spark_harness_selftest_e2e() {
    let lanes = 4_096usize;
    let seed = 1u64;
    let inputs = k_selftest_seeded_inputs(lanes, 0, seed);
    let expected = k_selftest_scalar_reference(&inputs);
    let expected_f = expected.out_f_bytes();
    let expected_u = expected.out_u_bytes();

    let class = ShapeClassCaps {
        name: "selftest-4096".into(),
        nv_cap: 12,
        link_cap: 13,
        contact_cap: 24,
        row_cap: 72,
        sample_cap: 16,
        actuator_cap: 12,
        worlds: lanes as u64,
        extra: BTreeMap::new(),
    };
    let h_inputs = [
        HarnessInput {
            name: "a".into(),
            elem: ElemType::F32,
            len: lanes,
            bytes: inputs.a_bytes(),
        },
        HarnessInput {
            name: "b".into(),
            elem: ElemType::F32,
            len: lanes,
            bytes: inputs.b_bytes(),
        },
        HarnessInput {
            name: "x".into(),
            elem: ElemType::U32,
            len: lanes,
            bytes: inputs.x_bytes(),
        },
        HarnessInput {
            name: "y".into(),
            elem: ElemType::U32,
            len: lanes,
            bytes: inputs.y_bytes(),
        },
    ];
    let h_outputs = [
        HarnessOutputSpec {
            name: "out_f".into(),
            elem: ElemType::F32,
            len: lanes,
        },
        HarnessOutputSpec {
            name: "out_u".into(),
            elem: ElemType::U32,
            len: lanes,
        },
    ];

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf();
    let log_dir = repo_root
        .join("target")
        .join("e2e-logs")
        .join("spark_harness_selftest");
    fs::create_dir_all(&log_dir).expect("create e2e log dir");
    let log_path = log_dir.join(format!("{}.ndjson", utc_stamp()));
    let mut log = File::create(&log_path).expect("create ndjson log");

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut ref_f_digest: Option<String> = None;
    let mut ref_u_digest: Option<String> = None;

    for workers in [1usize, 2, 8, 36] {
        let t0 = Instant::now();
        let mut cpu = BatchCpu::new(workers);
        let res = Harness::run(
            &mut cpu,
            k_selftest_entry(),
            &h_inputs,
            &h_outputs,
            &class,
            &Cx::new(),
        )
        .expect("harness run succeeds");
        let wall_ms = t0.elapsed().as_secs_f64() * 1_000.0;
        let receipt_ok = res.receipt.validate().is_ok();
        let f_diff = first_diff_lane(&res.slabs["out_f"], &expected_f);
        let u_diff = first_diff_lane(&res.slabs["out_u"], &expected_u);
        let f_ok = f_diff.is_none();
        let u_ok = u_diff.is_none();
        if !f_ok || !u_ok {
            let act_path = log_dir.join(format!("fail_w{workers}_actual.bin"));
            let exp_path = log_dir.join(format!("fail_w{workers}_expected.bin"));
            let _ = fs::write(&act_path, &res.slabs["out_f"]);
            let _ = fs::write(&exp_path, &expected_f);
            let fmt_opt = |o: Option<usize>| o.map_or_else(|| "null".to_owned(), |i| i.to_string());
            let _ = writeln!(
                log,
                "{{\"ts\":{},\"script\":\"spark_harness_selftest\",\"bead\":\"BD-014\",\"step\":\"workers_{workers}\",\"event\":\"mismatch\",\"first_diff_f\":{},\"first_diff_u\":{},\"actual_dump\":{},\"expected_dump\":{}}}",
                json_escape(&utc_stamp()),
                fmt_opt(f_diff),
                fmt_opt(u_diff),
                json_escape(&act_path.display().to_string()),
                json_escape(&exp_path.display().to_string()),
            );
        }
        let f_hex = res.digests["out_f"].to_hex();
        let u_hex = res.digests["out_u"].to_hex();

        let digest_ok = match (&ref_f_digest, &ref_u_digest) {
            (None, None) => {
                ref_f_digest = Some(f_hex.clone());
                ref_u_digest = Some(u_hex.clone());
                true
            }
            (Some(rf), Some(ru)) => rf == &f_hex && ru == &u_hex,
            _ => false,
        };
        for ok in [receipt_ok, f_ok, u_ok, digest_ok] {
            if ok {
                passed += 1;
            } else {
                failed += 1;
            }
        }

        let ae_json: Vec<String> = res
            .receipt
            .ae_entries()
            .iter()
            .map(|(id, _, val)| format!("{}:{}", json_escape(id), json_escape(val.unwrap_or(""))))
            .collect();
        writeln!(
            log,
            "{{\"ts\":{},\"script\":\"spark_harness_selftest\",\"bead\":\"BD-014\",\"step\":\"workers_{workers}\",\"event\":\"run\",\"workers\":{workers},\"wall_ms\":{wall_ms:.3},\"out_f_digest\":{},\"out_u_digest\":{},\"evidence\":{{{}}}}}",
            json_escape(&utc_stamp()),
            json_escape(&f_hex),
            json_escape(&u_hex),
            ae_json.join(","),
        )
        .expect("write run line");

        assert!(receipt_ok, "workers={workers}: receipt invalid");
        assert!(f_ok, "workers={workers}: out_f diverged from scalar loop");
        assert!(u_ok, "workers={workers}: out_u diverged from scalar loop");
        assert!(
            digest_ok,
            "workers={workers}: digest diverged across worker counts"
        );
    }

    writeln!(
        log,
        "{{\"ts\":{},\"script\":\"spark_harness_selftest\",\"bead\":\"BD-014\",\"step\":\"summary\",\"event\":\"summary\",\"passed\":{passed},\"failed\":{failed}}}",
        json_escape(&utc_stamp()),
    )
    .expect("write summary line");

    assert_eq!(failed, 0);
    assert_eq!(passed, 16);
}
