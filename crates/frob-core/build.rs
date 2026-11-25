//! Build-time stack identity and FrankenSim build-identity mode embedding for
//! `frob-core` (`N33a-06`, `N33a-08`, §33.2, §33.5).
//!
//! Copies the two one-line 64-hex hashes written by `xtask check-stack --write-receipt`
//! (`receipts/HEAD` and `receipts/LOCK_DIGEST`) into compile-time environment variables,
//! and resolves `FROB_BUILD_IDENTITY_MODE` (`dev-salt` or `depgraph-receipt`) fail-closed
//! from `FRANKENSIM_DEPGRAPH_SALT` and `FRANKENSIM_DEPGRAPH_RECEIPT`.
//! Performs no hashing and reads no sibling repositories.

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process;

const REMEDY: &str = "FROB_STACK_DEV=1 cargo xtask check-stack --write-receipt";
const ZERO_HEX: &str = "0000000000000000000000000000000000000000000000000000000000000000";

const ENV_SALT: &str = "FRANKENSIM_DEPGRAPH_SALT";
const ENV_RECEIPT: &str = "FRANKENSIM_DEPGRAPH_RECEIPT";
const MODE_DEV_SALT: &str = "dev-salt";
const MODE_DEPGRAPH_RECEIPT: &str = "depgraph-receipt";

/// Precedence fact for `frankensim/crates/fs-la/build.rs` when both
/// `FRANKENSIM_DEPGRAPH_SALT` and `FRANKENSIM_DEPGRAPH_RECEIPT` are set
/// (`N33a-08` Mode 2 step 2). Remains `None` until recorded as a re-read fact
/// in the `INT-FSIM-01` admission record.
const RECEIPT_WINS_WHEN_BOTH: Option<bool> = None;

fn workspace_root() -> PathBuf {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap_or_else(|| {
        eprintln!("frob-core/build.rs: CARGO_MANIFEST_DIR is not set");
        process::exit(1);
    });
    Path::new(&manifest_dir)
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| {
            eprintln!(
                "frob-core/build.rs: cannot locate workspace root from CARGO_MANIFEST_DIR={}",
                Path::new(&manifest_dir).display()
            );
            process::exit(1);
        })
        .to_path_buf()
}

fn is_valid_64_lower_hex(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn read_hash_file(path: &Path, label: &str, dev: bool) -> String {
    match fs::read_to_string(path) {
        Ok(raw) => {
            let trimmed = raw.trim();
            if !is_valid_64_lower_hex(trimmed) {
                eprintln!(
                    "frob-core/build.rs: `{label}` ({}) must contain exactly 64 lower-case hex digits, got `{trimmed}`.\n\
                     Regenerate with: {REMEDY}",
                    path.display()
                );
                process::exit(1);
            }
            trimmed.to_owned()
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            if dev {
                ZERO_HEX.to_owned()
            } else {
                eprintln!(
                    "frob-core/build.rs: required stack identity file `{label}` ({}) is missing.\n\
                     There is no unknown stack identity. To bootstrap or refresh receipts, run:\n\
                       {REMEDY}",
                    path.display()
                );
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!(
                "frob-core/build.rs: failed reading `{label}` ({}): {err}\n\
                 Regenerate with: {REMEDY}",
                path.display()
            );
            process::exit(1);
        }
    }
}

fn read_build_identity_var(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(v) if !v.is_empty() => Some(v),
        Ok(_) => {
            eprintln!(
                "frob-core/build.rs: `{name}` is set to an empty string; refusing fail-closed"
            );
            process::exit(1);
        }
        Err(env::VarError::NotPresent) => None,
        Err(env::VarError::NotUnicode(_)) => {
            eprintln!("frob-core/build.rs: `{name}` must be valid UTF-8; refusing fail-closed");
            process::exit(1);
        }
    }
}

fn resolve_build_identity_mode(
    salt: Option<&str>,
    receipt: Option<&str>,
    receipt_wins_when_both: Option<bool>,
) -> Result<&'static str, String> {
    let has_salt = salt.is_some_and(|s| !s.is_empty());
    let has_receipt = receipt.is_some_and(|s| !s.is_empty());
    match (has_salt, has_receipt) {
        (true, false) => Ok(MODE_DEV_SALT),
        (false, true) => Ok(MODE_DEPGRAPH_RECEIPT),
        (false, false) => Err(format!(
            "frob-core/build.rs: neither `{ENV_SALT}` nor `{ENV_RECEIPT}` is set; \
             failing closed. Build inside a workspace whose `.cargo/config.toml` sets \
             `{ENV_SALT}`, or export `{ENV_RECEIPT}` for an evidence-bearing build."
        )),
        (true, true) => match receipt_wins_when_both {
            Some(true) => Ok(MODE_DEPGRAPH_RECEIPT),
            Some(false) | None => Err(format!(
                "frob-core/build.rs: both `{ENV_SALT}` and `{ENV_RECEIPT}` are set while \
                 `RECEIPT_WINS_WHEN_BOTH` is `{receipt_wins_when_both:?}`; refusing ambiguous \
                 build identity until the receipt form and precedence are recorded as re-read \
                 facts in the INT-FSIM-01 admission record."
            )),
        },
    }
}

fn main() {
    let root = workspace_root();
    let head_path = root.join("receipts").join("HEAD");
    let lock_digest_path = root.join("receipts").join("LOCK_DIGEST");
    let stack_lock_path = root.join("franken-stack.lock");

    // Register every input that determines BUILT_UNDER and BUILD_IDENTITY_MODE so Cargo
    // reruns when any changes.
    println!("cargo:rerun-if-changed={}", head_path.display());
    println!("cargo:rerun-if-changed={}", lock_digest_path.display());
    println!("cargo:rerun-if-changed={}", stack_lock_path.display());
    println!("cargo:rerun-if-env-changed=FROB_STACK_DEV");
    println!("cargo:rerun-if-env-changed={ENV_SALT}");
    println!("cargo:rerun-if-env-changed={ENV_RECEIPT}");

    if let Ok(marker_path) = env::var("FROB_STACK_BUILD_MARKER")
        && !marker_path.is_empty()
        && let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&marker_path)
    {
        let _ = writeln!(f, "ran");
    }

    let dev = match env::var("FROB_STACK_DEV") {
        Ok(v) if v == "1" => true,
        Ok(v) if v.is_empty() || v == "0" => false,
        Err(env::VarError::NotPresent) => false,
        Ok(other) => {
            eprintln!(
                "frob-core/build.rs: unsupported FROB_STACK_DEV=`{other}` (use `FROB_STACK_DEV=1` or unset it)"
            );
            process::exit(1);
        }
        Err(env::VarError::NotUnicode(_)) => {
            eprintln!("frob-core/build.rs: FROB_STACK_DEV must be valid UTF-8 (`1` or unset)");
            process::exit(1);
        }
    };

    let receipt_hex = read_hash_file(&head_path, "receipts/HEAD", dev);
    let lock_hex = read_hash_file(&lock_digest_path, "receipts/LOCK_DIGEST", dev);

    let salt = read_build_identity_var(ENV_SALT);
    let depgraph_receipt = read_build_identity_var(ENV_RECEIPT);
    let build_identity_mode = match resolve_build_identity_mode(
        salt.as_deref(),
        depgraph_receipt.as_deref(),
        RECEIPT_WINS_WHEN_BOTH,
    ) {
        Ok(mode) => mode,
        Err(msg) => {
            eprintln!("{msg}");
            process::exit(1);
        }
    };

    println!("cargo:rustc-env=FROB_STACK_RECEIPT_HEX={receipt_hex}");
    println!("cargo:rustc-env=FROB_STACK_LOCK_HEX={lock_hex}");
    println!(
        "cargo:rustc-env=FROB_STACK_DEV_FLAG={}",
        if dev { "true" } else { "false" }
    );
    println!("cargo:rustc-env=FROB_BUILD_IDENTITY_MODE={build_identity_mode}");
}
