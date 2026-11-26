#![deny(unsafe_code)]

//! Prelude v0, dialect-cleanliness, and planted negative-control tests
//! (`cargo test -p frob-kernels prelude`, `cargo test -p frob-kernels dialect_sources_are_clean`).

use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use frob_core::id::hash_blob;
use frob_core::refusal::{RefusalCode, RetryPolicy};
use frob_core::registry::toml::{self, Fields, Value};
use frob_kernels::dialect::{
    BackendTarget, METAL_MAX_ARGUMENTS, PRELUDE_CUDA, PRELUDE_METAL, PRELUDE_VERSION,
    SKELETON_K_PROBE_CONTRACT, SKELETON_K_SELFTEST, assemble_and_lint, assemble_unit,
    body_after_prelude, expand_f32_bits_macro, frob_f32_bits, parse_c_u32_literal,
    planted_contraction_options,
};
use frob_kir::dialect_lint::{
    CompileOptions, DialectUnit, MetalMathMode, PRELUDE_BOUNDARY_MARKER, SourceFile,
    collect_findings, lint,
};

fn dialect_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("dialect")
}

fn bad_fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bad_dialect")
}

fn collect_headers(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect_headers(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("h") {
            out.push(p);
        }
    }
}

#[test]
fn dialect_sources_are_clean() {
    let root = dialect_root();
    let mut headers = Vec::new();
    collect_headers(&root, &mut headers);
    assert!(
        headers.len() >= 4,
        "expected at least prelude_cuda.h, prelude_metal.h, k_selftest.h, k_probe_contract.h"
    );

    let shape = SourceFile::new("shape.h", "#define NV_CAP 64u\n");
    let mut preludes_checked = 0usize;
    let mut sources_checked = 0usize;

    for path in &headers {
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let text = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));

        if rel == "prelude_cuda.h" {
            let unit = DialectUnit::default()
                .with_shape_constants(shape.clone())
                .with_prelude(SourceFile::new(&rel, text));
            let report = lint(&unit, &BackendTarget::Cuda.production_options())
                .unwrap_or_else(|r| panic!("{rel} failed lint: {r}"));
            assert!(report.findings.is_empty());
            preludes_checked += 1;
        } else if rel == "prelude_metal.h" {
            let unit = DialectUnit::default()
                .with_shape_constants(shape.clone())
                .with_prelude(SourceFile::new(&rel, text));
            let report = lint(&unit, &BackendTarget::Metal.production_options())
                .unwrap_or_else(|r| panic!("{rel} failed lint: {r}"));
            assert!(report.findings.is_empty());
            preludes_checked += 1;
        } else {
            for target in [BackendTarget::Cuda, BackendTarget::Metal] {
                let (_, _, report) = assemble_and_lint(
                    target,
                    Some(shape.clone()),
                    vec![SourceFile::new(&rel, text.clone())],
                )
                .unwrap_or_else(|r| panic!("{rel} under {target:?} failed lint: {r}"));
                assert!(report.findings.is_empty());
            }
            sources_checked += 1;
        }
    }

    assert_eq!(preludes_checked, 2, "must lint both CUDA and Metal preludes");
    assert!(sources_checked >= 2, "must lint checked-in skeletons");
}

#[test]
fn prelude_expansions_differ_only_in_prelude() {
    for (name, src) in [
        ("skeletons/k_selftest.h", SKELETON_K_SELFTEST),
        ("skeletons/k_probe_contract.h", SKELETON_K_PROBE_CONTRACT),
    ] {
        let (cuda_src, cuda_digest, cuda_rep) = assemble_and_lint(
            BackendTarget::Cuda,
            None,
            vec![SourceFile::new(name, src)],
        )
        .unwrap_or_else(|r| panic!("{name} CUDA failed lint: {r}"));
        let (metal_src, metal_digest, metal_rep) = assemble_and_lint(
            BackendTarget::Metal,
            None,
            vec![SourceFile::new(name, src)],
        )
        .unwrap_or_else(|r| panic!("{name} Metal failed lint: {r}"));

        assert!(cuda_rep.findings.is_empty());
        assert!(metal_rep.findings.is_empty());
        assert_ne!(cuda_src, metal_src, "CUDA and Metal preludes must differ");

        let cuda_body = body_after_prelude(&cuda_src).expect("boundary marker present in CUDA");
        let metal_body = body_after_prelude(&metal_src).expect("boundary marker present in Metal");
        assert_eq!(
            cuda_body, metal_body,
            "{name}: kernel text after prelude must be byte-identical"
        );
        assert_eq!(
            cuda_digest, metal_digest,
            "{name}: post-prelude BLAKE3 digests must match"
        );
        assert_eq!(hash_blob(cuda_body.as_bytes()), cuda_digest);
    }
}

#[test]
fn prelude_f32_bits_evaluates_0_25_and_neg_zero() {
    assert_eq!(PRELUDE_VERSION, 0);
    assert!(PRELUDE_CUDA.contains("#define FROB_PRELUDE_VERSION 0"));
    assert!(PRELUDE_METAL.contains("#define FROB_PRELUDE_VERSION 0"));

    // Verify both preludes define FROB_F32_BITS as exact bit reinterpretation (no arithmetic).
    for (lit, expected_bits, expected_val, neg_sign) in [
        ("0x3e800000u", 0x3e80_0000u32, 0.25_f32, false),
        ("0x80000000u", 0x8000_0000u32, -0.0_f32, true),
    ] {
        let cuda_exp = expand_f32_bits_macro(PRELUDE_CUDA, lit)
            .expect("prelude_cuda.h must define FROB_F32_BITS(b)");
        let metal_exp = expand_f32_bits_macro(PRELUDE_METAL, lit)
            .expect("prelude_metal.h must define FROB_F32_BITS(b)");

        assert_eq!(cuda_exp, format!("__uint_as_float((u32)({lit}))"));
        assert_eq!(metal_exp, format!("as_type<f32>((u32)({lit}))"));

        let parsed = parse_c_u32_literal(lit).expect("literal parses");
        assert_eq!(parsed, expected_bits);
        let val = frob_f32_bits(parsed);
        assert_eq!(val.to_bits(), expected_bits);
        assert_eq!(val, expected_val);
        assert_eq!(val.is_sign_negative(), neg_sign);
    }

    // Signed zero is bit-distinct from +0.0.
    assert_ne!(frob_f32_bits(0x8000_0000).to_bits(), 0.0_f32.to_bits());
}

#[test]
fn expanded_sources_use_only_the_inlined_target_prelude() {
    for target in [BackendTarget::Cuda, BackendTarget::Metal] {
        let name = target.prelude_name();
        let source = format!(
            "#include \"{name}\"\n/* café\n*/ %:inc\\\r\nlude \"{name}\" // redundant\nf32 probe(f32 x) {{ return x; }}\n"
        );
        let (expanded, digest, _) =
            assemble_and_lint(target, None, vec![SourceFile::new("probe.h", source)]).unwrap();
        assert!(expanded.starts_with(target.prelude_source()));
        let body = body_after_prelude(&expanded).unwrap();
        assert_eq!(body, "\n/* café\n*/ \r\n\nf32 probe(f32 x) { return x; }\n");
        assert_eq!(digest, hash_blob(body.as_bytes()));
    }
    for (target, path, prefix) in [
        (BackendTarget::Cuda, "x/prelude_cuda.h", ""),
        (BackendTarget::Metal, "../prelude_metal.h", ""),
        (BackendTarget::Cuda, "prelude_metal.h", ""),
        (BackendTarget::Metal, "prelude_cuda.h", ""),
        (
            BackendTarget::Cuda,
            "x/prelude_cuda.h",
            "const char* text = R\"tag(\n/* raw text\n)tag\";\n",
        ),
    ] {
        let source = SourceFile::new("probe.h", format!("{prefix}#include \"{path}\"\n"));
        let options = target.production_options();
        let unit = assemble_unit(target, None, vec![source.clone()]);
        let findings = collect_findings(&unit, &options);
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].rule, 1);
        assert_eq!(findings[0].token, format!("#include \"{path}\""));
        assert_eq!(
            assemble_and_lint(target, None, vec![source])
                .unwrap_err()
                .code,
            RefusalCode::FrobRef251
        );
    }
}

#[test]
fn composed_source_lint_checks_file_boundaries() {
    for target in [BackendTarget::Cuda, BackendTarget::Metal] {
        for (left, right) in [
            ("%\\", ":include \"nonexistent.h\"\n"),
            (
                "const char* text = R\"tag(\n",
                "/* raw text\n)tag\";\n#include \"nonexistent.h\"\n",
            ),
        ] {
            let sources = vec![
                SourceFile::new("left.h", left),
                SourceFile::new("right.h", right),
            ];
            let unit = assemble_unit(target, None, sources.clone());
            let findings = collect_findings(&unit, &target.production_options());
            assert_eq!(findings.len(), 1, "{target:?}: {findings:?}");
            assert_eq!(findings[0].rule, 1);
            assert_eq!(findings[0].token, "#include \"nonexistent.h\"");
            assert_eq!(
                assemble_and_lint(target, None, sources).unwrap_err().code,
                RefusalCode::FrobRef251
            );
        }

        let first = "f32 first(f32 x) { return x; }\n";
        let second = "f32 second(f32 x) { return first(x); }\n";
        let sources = vec![
            SourceFile::new("left.h", first),
            SourceFile::new("right.h", second),
        ];
        let (expanded, digest, _) = assemble_and_lint(target, None, sources).unwrap();
        assert_eq!(
            expanded,
            format!(
                "{}{PRELUDE_BOUNDARY_MARKER}\n{first}{second}",
                target.prelude_source()
            )
        );
        assert_eq!(digest, hash_blob(format!("{first}{second}").as_bytes()));
    }
}

#[test]
fn composed_source_lint_checks_prelude_boundary() {
    for target in [BackendTarget::Cuda, BackendTarget::Metal] {
        let options = target.production_options();
        let unit = DialectUnit::single_source(
            "probe.h",
            "/* raw text\n)tag\";\n#include \"nonexistent.h\"\n",
        )
        .with_prelude(SourceFile::new(
            target.prelude_name(),
            "const char* text = R\"tag(\n",
        ));
        let findings = collect_findings(&unit, &options);
        assert_eq!(findings.len(), 1, "{target:?}: {findings:?}");
        assert_eq!(findings[0].rule, 1);
        assert_eq!(findings[0].token, "#include \"nonexistent.h\"");
        assert_eq!(
            lint(&unit, &options).unwrap_err().code,
            RefusalCode::FrobRef251
        );

        let shape = "#define CAPACITY 4\n";
        let body = "f32 probe(f32 x) { return x; }\n";
        let source = format!("#include \"{}\"\n{body}", target.prelude_name());
        let (expanded, digest, _) = assemble_and_lint(
            target,
            Some(SourceFile::new("shape.h", shape)),
            vec![SourceFile::new("probe.h", source)],
        )
        .unwrap();
        assert_eq!(
            expanded,
            format!(
                "{shape}{}{PRELUDE_BOUNDARY_MARKER}\n\n{body}",
                target.prelude_source()
            )
        );
        assert_eq!(digest, hash_blob(format!("\n{body}").as_bytes()));
    }
}

#[test]
fn raw_literals_in_directives_preserve_following_include_findings() {
    let tail = "/* raw text\n)tag\"\n#include \"bd013-review-missing-header.h\"\n";
    for target in [BackendTarget::Cuda, BackendTarget::Metal] {
        let options = target.production_options();
        for prefix in ["R", "u8R", "uR", "UR", "LR"] {
            let head = format!("#define FROB_PROBE_TEXT {prefix}\"tag(\n");
            let split = DialectUnit::single_source("probe.h", tail)
                .with_prelude(SourceFile::new(target.prelude_name(), &head));
            let single = assemble_unit(
                target,
                None,
                vec![SourceFile::new("probe.h", format!("{head}{tail}"))],
            );
            for unit in [split, single] {
                assert_rule_1(
                    &unit,
                    &options,
                    "#include \"bd013-review-missing-header.h\"",
                );
            }
            let open = DialectUnit::single_source("probe.h", head);
            assert_rule_1(&open, &options, "raw string literal");
        }
    }
}

#[test]
fn prelude_exemption_requires_checked_in_bytes() {
    for target in [BackendTarget::Cuda, BackendTarget::Metal] {
        let options = target.production_options();
        let original = target.prelude_source();
        let altered = original.replacen("FrankenRobotics", "frankenRobotics", 1);
        assert_eq!(original.len(), altered.len());
        let changed = original
            .bytes()
            .zip(altered.bytes())
            .filter(|(a, b)| a != b);
        assert_eq!(changed.count(), 1);
        let expected = match target {
            BackendTarget::Cuda => "#ifndef",
            BackendTarget::Metal => "#include <metal_stdlib>",
        };
        let check = |text, token| {
            let unit =
                DialectUnit::default().with_prelude(SourceFile::new(target.prelude_name(), text));
            assert_rule_1(&unit, &options, token);
        };
        check(altered.clone(), expected);

        // The checked-in CUDA header has no include to retain in the one-byte plant.
        if target == BackendTarget::Cuda {
            check(
                format!("{altered}\n#include <unexpected.h>\n"),
                "#include <unexpected.h>",
            );
        }
    }
}

fn assert_rule_1(unit: &DialectUnit, options: &CompileOptions, token: &str) {
    let findings = collect_findings(unit, options);
    assert!(
        findings.iter().any(|f| f.rule == 1 && f.token == token),
        "{options:?} {token}: {findings:?}"
    );
    assert_eq!(
        lint(unit, options).unwrap_err().code,
        RefusalCode::FrobRef251
    );
}

#[test]
fn cuda_unsigned_atomics_reject_float_pointer() {
    // Host C++ syntax check with CUDA's relevant overloads. This launches no kernel.
    let compile = |body: &str| {
        let source = format!(
            "#define __device__\n\
             unsigned atomicAdd(unsigned*, unsigned);\n\
             float atomicAdd(float*, float);\n\
             unsigned atomicMax(unsigned*, unsigned);\n\
             unsigned atomicMin(unsigned*, unsigned);\n\
             unsigned atomicOr(unsigned*, unsigned);\n\
             {PRELUDE_CUDA}\n{body}"
        );
        let mut compiler = Command::new("c++")
            .args(["-std=c++17", "-x", "c++", "-fsyntax-only", "-"])
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start host C++ compiler");
        compiler
            .stdin
            .take()
            .expect("compiler stdin")
            .write_all(source.as_bytes())
            .expect("send CUDA prelude and pointer probe");
        compiler.wait_with_output().expect("C++ syntax result")
    };
    let good = compile(
        "void k(frob_atomic_u32* p) {\
         FROB_ATOMIC_ADDW_U32(p, 1u); FROB_ATOMIC_MAX_U32(p, 1u);\
         FROB_ATOMIC_MIN_U32(p, 1u); FROB_ATOMIC_OR_U32(p, 1u); }",
    );
    assert!(
        good.status.success(),
        "{}",
        String::from_utf8_lossy(&good.stderr)
    );
    let bad = compile("void k(float* p) { FROB_ATOMIC_ADDW_U32(p, 1u); }");
    let diagnostic = String::from_utf8_lossy(&bad.stderr);
    assert!(
        !bad.status.success() && diagnostic.contains("frob_atomic_u32_ptr"),
        "{diagnostic}"
    );

    let overload = assemble_unit(
        BackendTarget::Cuda,
        None,
        vec![SourceFile::new(
            "atomic_overload.h",
            "FROB_FN float* frob_atomic_u32_ptr(float* p) { return p; }\n\
             FROB_FN void probe(FROB_PTR(float) p) { FROB_ATOMIC_ADDW_U32(p, 1u); }\n",
        )],
    );
    let opts = BackendTarget::Cuda.production_options();
    let findings = collect_findings(&overload, &opts);
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].rule, 2);
    assert_eq!(findings[0].token, "frob_atomic_u32_ptr");
    assert_eq!(
        lint(&overload, &opts).unwrap_err().code,
        RefusalCode::FrobRef251
    );
}

#[test]
fn prelude_contraction_detector_and_planted_options() {
    // Static half: K-probe-contract is lint-clean under production CUDA and Metal options.
    for target in [BackendTarget::Cuda, BackendTarget::Metal] {
        let (_, _, report) = assemble_and_lint(
            target,
            None,
            vec![SourceFile::new(
                "skeletons/k_probe_contract.h",
                SKELETON_K_PROBE_CONTRACT,
            )],
        )
        .unwrap_or_else(|r| panic!("K-probe-contract failed lint on {target:?}: {r}"));
        assert!(report.findings.is_empty());
    }

    // Reference arithmetic for KA-526 detector and GB-25 regrouping pair.
    let a = frob_f32_bits(0x3f80_0800); // 1 + 2^-12
    let c = frob_f32_bits(0x3f80_1000); // 1 + 2^-11
    assert_eq!(a, 1.0_f32 + 2.0_f32.powi(-12));
    assert_eq!(c, 1.0_f32 + 2.0_f32.powi(-11));
    let unfused = (a * a) - c;
    let fused = a.mul_add(a, -c);
    assert_eq!(unfused.to_bits(), 0x0000_0000);
    assert_eq!(fused.to_bits(), 0x3380_0000); // 2^-24

    let ra = frob_f32_bits(0x4cbe_bc20); // 10^8
    let rb = frob_f32_bits(0xccbe_bc20); // -10^8
    let rc = frob_f32_bits(0x3f80_0000); // 1.0
    assert_eq!(ra, 100_000_000.0_f32);
    assert_eq!(rb, -100_000_000.0_f32);
    assert_eq!(((ra + rb) + rc).to_bits(), 0x3f80_0000);
    assert_eq!((ra + (rb + rc)).to_bits(), 0x0000_0000);

    // Planted contraction-permitting compile-option sets (in-memory + checked-in TOML fixture).
    for (label, opts) in planted_contraction_options() {
        let target = match opts {
            CompileOptions::Cuda { .. } => BackendTarget::Cuda,
            CompileOptions::Metal { .. } => BackendTarget::Metal,
        };
        let u = assemble_unit(
            target,
            None,
            vec![SourceFile::new(
                "skeletons/k_probe_contract.h",
                SKELETON_K_PROBE_CONTRACT,
            )],
        );
        let findings = collect_findings(&u, &opts);
        assert!(
            !findings.is_empty() && findings.iter().all(|f| f.rule == 4),
            "planted option fixture `{label}` must fail rule 4, got {findings:?}"
        );
        let err = lint(&u, &opts).unwrap_err();
        assert_eq!(err.code, RefusalCode::FrobRef251);
        assert_eq!(err.retry, RetryPolicy::AfterChange);
    }

    // Also parse and test the checked-in `contraction_options.toml` fixture.
    let toml_path = bad_fixtures_root().join("contraction_options.toml");
    let toml_bytes = fs::read(&toml_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", toml_path.display()));
    let root = toml::parse(&toml_bytes).expect("contraction_options.toml parses");
    let mut f = Fields::new(root, "contraction_options.toml");
    assert_eq!(
        f.require_str("schema").unwrap(),
        "frob.dialect.planted_options/1"
    );
    let fixtures = f.require_array("fixture").unwrap();
    f.finish().unwrap();
    assert!(fixtures.len() >= 4);

    for sp in fixtures {
        let Value::Table(tbl) = sp.value else {
            panic!("fixture entry must be a table");
        };
        let mut row = Fields::new(tbl, "fixture");
        let name = row.require_str("name").unwrap();
        let backend = row.require_str("backend").unwrap();
        let expect_refusal = row.require_str("expect_refusal").unwrap();
        let expect_rule = u8::try_from(row.require_int("expect_rule").unwrap()).unwrap();
        let opts = match backend.as_str() {
            "cuda" => {
                let raw = row.require_array("options").unwrap();
                let options = raw
                    .into_iter()
                    .map(|x| x.value.as_str().expect("option string").to_owned())
                    .collect();
                CompileOptions::Cuda { options }
            }
            "metal" => {
                let mode = row.require_str("math_mode").unwrap();
                let math_mode = match mode.as_str() {
                    "safe" => MetalMathMode::Safe,
                    "fast" => MetalMathMode::Fast,
                    other => panic!("unexpected math_mode `{other}`"),
                };
                CompileOptions::Metal { math_mode }
            }
            other => panic!("unexpected backend `{other}`"),
        };
        row.finish().unwrap();

        let target = match opts {
            CompileOptions::Cuda { .. } => BackendTarget::Cuda,
            CompileOptions::Metal { .. } => BackendTarget::Metal,
        };
        let fixture_unit = assemble_unit(
            target,
            None,
            vec![SourceFile::new(
                "skeletons/k_probe_contract.h",
                SKELETON_K_PROBE_CONTRACT,
            )],
        );
        let err = lint(&fixture_unit, &opts)
            .expect_err(&format!("fixture `{name}` must be refused under rule 4"));
        assert_eq!(err.code.as_str(), expect_refusal, "fixture `{name}`");
        let findings = collect_findings(&fixture_unit, &opts);
        assert!(
            findings.iter().any(|x| x.rule == expect_rule),
            "fixture `{name}` expected rule {expect_rule}, got {findings:?}"
        );
    }
}

#[test]
fn prelude_metal_msl_3_1_and_buffer_limit() {
    // Contraction-off pragmas are present inside prelude_metal.h.
    assert!(PRELUDE_METAL.contains("#pragma clang fp contract(off)"));
    assert!(PRELUDE_METAL.contains("#pragma STDC FP_CONTRACT OFF"));
    assert!(PRELUDE_METAL.contains("#include <metal_stdlib>"));
    assert!(PRELUDE_METAL.contains("using namespace metal;"));

    // No construct newer than MSL 3.1 appears in prelude_metal.h or expanded skeletons.
    const POST_MSL_3_1_TOKENS: &[&str] = &[
        "bfloat",
        "bfloat16_t",
        "cooperative_tensor",
        "mesh_grid_properties",
        "atomic_ulong",
        "metal4",
    ];
    for (name, src) in [
        ("k_selftest.h", SKELETON_K_SELFTEST),
        ("k_probe_contract.h", SKELETON_K_PROBE_CONTRACT),
    ] {
        let (expanded, _, _) = assemble_and_lint(
            BackendTarget::Metal,
            None,
            vec![SourceFile::new(name, src)],
        )
        .unwrap();
        for banned in POST_MSL_3_1_TOKENS {
            assert!(
                !expanded.contains(banned),
                "Metal expansion of {name} must not contain post-MSL-3.1 construct `{banned}`"
            );
        }

        // Verify every FROB_BUF / FROB_BUF_RO index anywhere in source stays strictly below METAL_MAX_ARGUMENTS (31).
        let indices = extract_frob_buf_indices(src);
        assert!(!indices.is_empty(), "{name}: expected at least one FROB_BUF* binding");
        for idx in indices {
            assert!(
                idx < METAL_MAX_ARGUMENTS,
                "{name}: buffer index {idx} must be < {METAL_MAX_ARGUMENTS}"
            );
        }
    }

    // Verify inline single-line FROB_BUF(..., 31) is also detected (not just line-leading).
    let inline_bad = "FROB_KERNEL(k_too_many, FROB_BUF(f32, a, 0), FROB_BUF(f32, b, 31)) FROB_KERNEL_END";
    let inline_indices = extract_frob_buf_indices(inline_bad);
    assert_eq!(inline_indices, vec![0, 31]);
    assert!(inline_indices.iter().any(|&i| i >= METAL_MAX_ARGUMENTS));
}

fn extract_frob_buf_indices(src: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for needle in ["FROB_BUF(", "FROB_BUF_RO("] {
        let mut rest = src;
        while let Some(pos) = rest.find(needle) {
            let after = &rest[pos + needle.len()..];
            if let Some(close) = after.find(')') {
                let inner = &after[..close];
                let idx_str = inner.split(',').nth(2).expect("buffer index arg").trim();
                let idx: usize = idx_str
                    .parse()
                    .unwrap_or_else(|e| panic!("non-integer buffer index `{idx_str}`: {e}"));
                out.push(idx);
                rest = &after[close + 1..];
            } else {
                break;
            }
        }
    }
    out.sort_unstable();
    out
}

#[test]
fn prelude_bad_dialect_fixtures_all_rejected() {
    let dir = bad_fixtures_root();
    let mut files = Vec::new();
    collect_headers(&dir, &mut files);
    assert!(
        files.len() >= 9,
        "expected planted bad dialect fixtures for rules 1, 2, 3, 4, 6"
    );

    let mut covered_rules = BTreeSet::new();

    for path in files {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("fixture.h");
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));

        assert!(
            text.lines().next().unwrap_or("").contains("synthetic"),
            "{name} must declare synthetic kind header on line 1 (STD-TEST)"
        );

        let expect_line = text
            .lines()
            .find(|l| l.trim_start().starts_with("// expect:"))
            .unwrap_or_else(|| panic!("{name} missing `// expect: FROB-REF-251 rules=...` line"));
        assert!(expect_line.contains("FROB-REF-251"));
        let rules_part = expect_line
            .split("rules=")
            .nth(1)
            .unwrap_or_else(|| panic!("{name} missing `rules=` in expect header"))
            .trim();
        let expected_rules: BTreeSet<u8> = rules_part
            .split(',')
            .map(|r| {
                r.trim()
                    .parse::<u8>()
                    .unwrap_or_else(|e| panic!("{name}: bad rule `{r}`: {e}"))
            })
            .collect();

        for target in [BackendTarget::Cuda, BackendTarget::Metal] {
            let unit = assemble_unit(target, None, vec![SourceFile::new(name, text.clone())]);
            let opts = target.production_options();
            let err = lint(&unit, &opts)
                .expect_err(&format!("fixture `{name}` must be refused under {target:?}"));
            assert_eq!(err.code, RefusalCode::FrobRef251, "fixture `{name}`");
            assert_eq!(err.retry, RetryPolicy::AfterChange, "fixture `{name}`");

            let findings = collect_findings(&unit, &opts);
            let observed_rules: BTreeSet<u8> = findings.iter().map(|f| f.rule).collect();
            assert_eq!(
                observed_rules, expected_rules,
                "fixture `{name}` ({target:?}) rule mismatch: findings={findings:?}"
            );
        }

        covered_rules.extend(expected_rules);
    }

    for required_rule in [1u8, 2, 3, 4, 6] {
        assert!(
            covered_rules.contains(&required_rule),
            "bad_dialect fixtures must cover rule {required_rule}"
        );
    }
}

#[test]
fn parse_c_u32_literal_rejects_signed_hex_and_decimal() {
    assert_eq!(frob_kernels::dialect::parse_c_u32_literal("0x3f800000u"), Some(0x3f80_0000));
    assert_eq!(frob_kernels::dialect::parse_c_u32_literal("42U"), Some(42));
    assert_eq!(frob_kernels::dialect::parse_c_u32_literal("0x+1"), None);
    assert_eq!(frob_kernels::dialect::parse_c_u32_literal("+1"), None);
    assert_eq!(frob_kernels::dialect::parse_c_u32_literal("0x"), None);
}
