//! Independence source-scan tests and standalone `frob-verify` CLI tests (BD-026).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use frob_cert::blake3::hash_hex;
use frob_cert::json::parse_canonical;
use frob_cert::limits::{
    COMPILED_LIMITS, RULE_INPUT_UNREADABLE, RULE_JSON_KEY_ORDER, RULE_KIND_UNKNOWN,
};

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

/// Strip `//` line comments and `/* ... */` block comments so token checks operate on code.
fn strip_comments(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(bytes.len());
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn contains_ident(haystack: &str, ident: &str) -> bool {
    let bytes = haystack.as_bytes();
    let needle = ident.as_bytes();
    if needle.is_empty() || bytes.len() < needle.len() {
        return false;
    }
    for start in 0..=bytes.len() - needle.len() {
        if &bytes[start..start + needle.len()] == needle {
            let before_ok = start == 0 || !is_ident_char(bytes[start - 1]);
            let after_pos = start + needle.len();
            let after_ok = after_pos == bytes.len() || !is_ident_char(bytes[after_pos]);
            if before_ok && after_ok {
                return true;
            }
        }
    }
    false
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[test]
fn source_scan_forbid_unsafe_and_forbidden_imports() {
    let root = crate_root();

    // 1. Crate roots forbid unsafe code.
    let lib_src = fs::read_to_string(root.join("src/lib.rs")).unwrap();
    let bin_src = fs::read_to_string(root.join("src/bin/frob-verify.rs")).unwrap();
    assert!(
        lib_src.trim_start().starts_with("#![forbid(unsafe_code)]"),
        "src/lib.rs must begin with #![forbid(unsafe_code)]"
    );
    assert!(
        bin_src.trim_start().starts_with("#![forbid(unsafe_code)]"),
        "src/bin/frob-verify.rs must begin with #![forbid(unsafe_code)]"
    );

    // 2. Scan every source file under src/.
    let mut rs_files = Vec::new();
    collect_rs_files(&root.join("src"), &mut rs_files);
    assert!(rs_files.len() >= 8);

    for path in &rs_files {
        let raw = fs::read_to_string(path).unwrap();
        let code = strip_comments(&raw);
        let rel = path.strip_prefix(&root).unwrap();
        let is_bin = rel == Path::new("src/bin/frob-verify.rs");

        // Zero unsafe keyword anywhere (except inside forbid(unsafe_code)).
        let without_forbid = code.replace("#![forbid(unsafe_code)]", "");
        assert!(
            !contains_ident(&without_forbid, "unsafe"),
            "forbidden `unsafe` found in {}",
            rel.display()
        );

        // Zero floating-point types anywhere in src/.
        assert!(
            !contains_ident(&code, "f32") && !contains_ident(&code, "f64"),
            "forbidden floating-point type found in {}",
            rel.display()
        );

        // Zero network or subprocess spawning anywhere in src/.
        assert!(
            !code.contains("std::net") && !code.contains("net::"),
            "forbidden std::net reference in {}",
            rel.display()
        );
        assert!(
            !code.contains("Command"),
            "forbidden std::process::Command reference in {}",
            rel.display()
        );

        if !is_bin {
            assert!(
                !code.contains("std::env") && !code.contains("std::process"),
                "forbidden std::env or std::process in library module {}",
                rel.display()
            );
        } else {
            // Binary may only call std::env::args_os() for CLI arguments; never read env vars.
            for forbidden_env in [
                "env::var",
                "env::vars",
                "current_dir",
                "temp_dir",
                "home_dir",
                "current_exe",
                "set_var",
                "remove_var",
            ] {
                assert!(
                    !code.contains(forbidden_env),
                    "forbidden environment call `{forbidden_env}` in {}",
                    rel.display()
                );
            }
        }
    }

    // 3. Cargo.toml has zero dependencies and zero workspace inheritance.
    let cargo_toml = fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert!(
        !cargo_toml.contains("workspace = true") && !cargo_toml.contains("workspace=true"),
        "crates/frob-cert/Cargo.toml must not inherit any workspace keys"
    );
    let mut section = String::new();
    for line in cargo_toml.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = trimmed.to_string();
            continue;
        }
        if matches!(
            section.as_str(),
            "[dependencies]" | "[dev-dependencies]" | "[build-dependencies]"
        ) {
            panic!("crates/frob-cert/Cargo.toml must keep {section} empty, found: {trimmed}");
        }
    }

    // 4. rust-toolchain.toml names a pinned stable release (not nightly/beta).
    let toolchain = fs::read_to_string(root.join("rust-toolchain.toml")).unwrap();
    let channel_line = toolchain
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with("channel"))
        .expect("rust-toolchain.toml must declare channel");
    assert!(channel_line.contains("\"1."), "channel line: {channel_line}");
    assert!(
        !channel_line.contains("nightly") && !channel_line.contains("beta"),
        "channel must be a stable release: {channel_line}"
    );

    // 5. CONTRACT.md exists with all 10 required sections and determinism class D0.
    let contract = fs::read_to_string(root.join("CONTRACT.md")).unwrap();
    for required in [
        "Purpose",
        "Inputs",
        "Outputs",
        "Invariants",
        "Refusals",
        "Determinism",
        "Evidence",
        "Validity",
        "Oracles",
        "Known",
        "D0",
    ] {
        assert!(
            contract.contains(required),
            "CONTRACT.md missing required section/term {required:?}"
        );
    }
}

#[test]
fn verify_reads_only_named_files() {
    let base = std::env::var_os("BIG_SCRATCH")
        .or_else(|| std::env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let scratch = base
        .join("frob-BrownHeron")
        .join(format!("test-verify-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&scratch).unwrap();

    // Named canonical bundle + unreadable sibling file in the same directory.
    let bundle_bytes = br#"{"a":2,"b":1}"#;
    let bundle_path = scratch.join("bundle.json");
    let sibling_path = scratch.join("unreadable_sibling.json");
    let reser_path = scratch.join("reserialized.json");
    fs::write(&bundle_path, bundle_bytes).unwrap();
    fs::write(&sibling_path, br#"{"secret":true}"#).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&sibling_path, fs::Permissions::from_mode(0o000)).unwrap();
    }

    let bin = env!("CARGO_BIN_EXE_frob-verify");
    let output = Command::new(bin)
        .env_clear()
        .current_dir(&scratch)
        .arg("bundle.json")
        .arg("--reserialize")
        .arg(&reser_path)
        .output()
        .unwrap();

    // Restore permissions on sibling file (never delete files; leave scratch readable).
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&sibling_path, fs::Permissions::from_mode(0o644));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(0),
        "expected exit 0, stdout={stdout:?}, stderr={stderr:?}"
    );
    assert!(stdout.contains("accept"), "stdout={stdout:?}");
    assert!(stdout.contains(&hash_hex(bundle_bytes)), "stdout={stdout:?}");
    assert!(stdout.contains("limits:"), "stdout={stdout:?}");
    assert!(stdout.contains("trusted base:"), "stdout={stdout:?}");

    // --reserialize output is byte-identical to input.
    let reser_bytes = fs::read(&reser_path).unwrap();
    assert_eq!(reser_bytes, bundle_bytes);

    // --json mode emits valid canonical JSON.
    let json_out = Command::new(bin)
        .env_clear()
        .arg("--json")
        .arg(&bundle_path)
        .output()
        .unwrap();
    assert_eq!(json_out.status.code(), Some(0));
    let json_line = String::from_utf8(json_out.stdout).unwrap();
    let trimmed = json_line.trim_end_matches('\n');
    let parsed_report = parse_canonical(trimmed.as_bytes(), &COMPILED_LIMITS).unwrap();
    assert_eq!(
        parsed_report.get("overall").and_then(|v| v.as_str()),
        Some("accept")
    );

    // Unknown proof kind -> reject with kind.unknown, exit 3.
    let unknown_path = scratch.join("unknown_kind.frobcert");
    fs::write(&unknown_path, br#"{"proof":{"kind":"mystery-proof/1"}}"#).unwrap();
    let unknown_out = Command::new(bin)
        .env_clear()
        .arg(&unknown_path)
        .output()
        .unwrap();
    assert_eq!(unknown_out.status.code(), Some(3));
    let u_stdout = String::from_utf8_lossy(&unknown_out.stdout);
    assert!(u_stdout.contains("reject"));
    assert!(u_stdout.contains(RULE_KIND_UNKNOWN));

    // Non-canonical JSON -> reject with json.key-order, exit 3.
    let noncanon_path = scratch.join("noncanon.json");
    fs::write(&noncanon_path, br#"{"b":1,"a":2}"#).unwrap();
    let noncanon_out = Command::new(bin)
        .env_clear()
        .arg(&noncanon_path)
        .output()
        .unwrap();
    assert_eq!(noncanon_out.status.code(), Some(3));
    let nc_stdout = String::from_utf8_lossy(&noncanon_out.stdout);
    assert!(nc_stdout.contains("reject"));
    assert!(nc_stdout.contains(RULE_JSON_KEY_ORDER));

    // Missing file -> reject with input.unreadable, exit 3.
    let missing_out = Command::new(bin)
        .env_clear()
        .arg(scratch.join("does_not_exist.frobcert"))
        .output()
        .unwrap();
    assert_eq!(missing_out.status.code(), Some(3));
    let m_stdout = String::from_utf8_lossy(&missing_out.stdout);
    assert!(m_stdout.contains("reject"));
    assert!(m_stdout.contains(RULE_INPUT_UNREADABLE));
}
