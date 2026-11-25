//! Dirty-tree PASS guard for gate recording (bead `N33a-05`, INV-SUITE-03).
//!
//! Single authoritative implementation called by `check-stack`, the gate
//! runner, the admission validator, and exit reviews. A gate result may be
//! recorded as `pass` only when:
//! 1. The cited resolution receipt file exists and its bytes hash to its name.
//! 2. This repository's own tree was clean (`repo_dirty == false`).
//! 3. Every sibling entry in the receipt has `dirty == false`.
//! 4. `duplicates` holds no `asupersync` entry.
//!
//! Otherwise [`may_record_pass`] returns [`GuardRefusal`] with
//! `reason == GUARD_REASON_DIRTY_STACK` (`"dirty-stack"`). Pure (D0), zero
//! `unsafe`, prints nothing (rule L-7), and writes nothing.

use std::fmt;
use std::path::Path;

use crate::stack::receipt::{ReceiptError, StackReceipt, load_and_verify_receipt};

/// Standard refusal reason attached to every gate-recording refusal (`N33a-05`).
pub const GUARD_REASON_DIRTY_STACK: &str = "dirty-stack";

/// Specific condition that prevented a receipt from qualifying for a `pass` record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardRefusalKind {
    MissingReceipt { path: String },
    TamperedReceipt { detail: String },
    DirtyRepo,
    DirtySiblings { fds: Vec<String> },
    AsupersyncDuplicate { entries: Vec<String> },
}

/// Refusal returned by [`may_record_pass`] or [`may_record_pass_receipt`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardRefusal {
    /// Always `"dirty-stack"` (`GUARD_REASON_DIRTY_STACK`).
    pub reason: &'static str,
    pub kind: GuardRefusalKind,
    pub message: String,
}

impl GuardRefusal {
    #[must_use]
    pub fn new(kind: GuardRefusalKind, message: impl Into<String>) -> Self {
        Self {
            reason: GUARD_REASON_DIRTY_STACK,
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for GuardRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.reason, self.message)
    }
}

impl std::error::Error for GuardRefusal {}

/// Verify that the receipt file at `receipt_path` exists, hashes to its
/// `<64-hex>.json` filename, is canonical, was captured on a clean repository
/// and clean siblings, and contains no `asupersync` duplicate.
pub fn may_record_pass(receipt_path: &Path) -> Result<(), GuardRefusal> {
    if !receipt_path.is_file() {
        return Err(GuardRefusal::new(
            GuardRefusalKind::MissingReceipt {
                path: receipt_path.display().to_string(),
            },
            format!("cited receipt does not exist: {}", receipt_path.display()),
        ));
    }

    let (receipt, _, _) = load_and_verify_receipt(receipt_path).map_err(|err| match err {
        ReceiptError::Io(detail) => GuardRefusal::new(
            GuardRefusalKind::MissingReceipt {
                path: receipt_path.display().to_string(),
            },
            detail,
        ),
        other => GuardRefusal::new(
            GuardRefusalKind::TamperedReceipt {
                detail: other.to_string(),
            },
            format!("receipt integrity check failed: {other}"),
        ),
    })?;

    may_record_pass_receipt(&receipt)
}

/// Evaluate the dirty-tree and duplicate-asupersync PASS guard on an already
/// verified [`StackReceipt`].
pub fn may_record_pass_receipt(receipt: &StackReceipt) -> Result<(), GuardRefusal> {
    if receipt.repo_dirty {
        return Err(GuardRefusal::new(
            GuardRefusalKind::DirtyRepo,
            "repository working tree was dirty when this receipt was recorded (INV-SUITE-03)",
        ));
    }

    let dirty_fds: Vec<String> = receipt
        .siblings
        .iter()
        .filter(|s| s.dirty)
        .map(|s| s.fd.clone())
        .collect();
    if !dirty_fds.is_empty() {
        return Err(GuardRefusal::new(
            GuardRefusalKind::DirtySiblings {
                fds: dirty_fds.clone(),
            },
            format!(
                "sibling checkout(s) were dirty when this receipt was recorded: {} (INV-SUITE-03)",
                dirty_fds.join(", ")
            ),
        ));
    }

    let asup_dups: Vec<String> = receipt
        .duplicates
        .iter()
        .filter(|d| is_asupersync_duplicate(d))
        .cloned()
        .collect();
    if !asup_dups.is_empty() {
        return Err(GuardRefusal::new(
            GuardRefusalKind::AsupersyncDuplicate {
                entries: asup_dups.clone(),
            },
            format!(
                "receipt records duplicate asupersync package(s): {} (blocks PASS under INV-SUITE-03)",
                asup_dups.join(", ")
            ),
        ));
    }

    Ok(())
}

#[must_use]
pub fn is_asupersync_duplicate(entry: &str) -> bool {
    entry == "asupersync" || entry.starts_with("asupersync ")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::*;
    use crate::stack::receipt::{
        CommandEntry, DriftClass, LockfileEntry, OwnLockEntry, STACK_RECEIPT_SCHEMA, SiblingEntry,
        write_receipt,
    };

    const SAMPLE_REV: &str = "d02ad9c170d5932667d15359cd362d3198ac4806";
    const SAMPLE_HASH: &str =
        "d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4";

    fn fresh_dir(tag: &str) -> PathBuf {
        static CTR: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "frob_xtask_guard_{tag}_{}_{}",
            std::process::id(),
            CTR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create guard fixture dir");
        dir
    }

    fn clean_sample_receipt() -> StackReceipt {
        let mut build_env = BTreeMap::new();
        build_env.insert(
            "FRANKENSIM_DEPGRAPH_SALT".to_owned(),
            "synthetic-v1".to_owned(),
        );
        StackReceipt {
            schema: STACK_RECEIPT_SCHEMA.to_owned(),
            repo_rev: SAMPLE_REV.to_owned(),
            repo_dirty: false,
            repo_tree_hash: None,
            toolchain: "nightly-2026-08-31".to_owned(),
            siblings: vec![SiblingEntry {
                fd: "FD-FSIM".to_owned(),
                rev: SAMPLE_REV.to_owned(),
                dirty: false,
                tree_hash: None,
                features: vec!["referee".to_owned()],
                own_lock: OwnLockEntry {
                    file: None,
                    hash: None,
                    agrees: true,
                    class: DriftClass::OnPin,
                    reason: None,
                },
            }],
            lockfiles: vec![LockfileEntry {
                workspace: "core".to_owned(),
                hash: SAMPLE_HASH.to_owned(),
                packages: 2,
                registry: 0,
            }],
            patches: vec!["asupersync -> ../asupersync".to_owned()],
            profile_overrides: vec!["fsqlite*: opt-level 1".to_owned()],
            build_env,
            duplicates: Vec::new(),
            commands: vec![CommandEntry {
                argv: vec!["cargo".to_owned(), "tree".to_owned(), "-d".to_owned()],
                exit: 0,
                output_hash: SAMPLE_HASH.to_owned(),
            }],
        }
    }

    fn store_receipt(tag: &str, receipt: &StackReceipt) -> PathBuf {
        let dir = fresh_dir(tag);
        let canon = receipt.to_canonical_bytes().expect("canonicalize receipt");
        let (path, _) = write_receipt(&dir, &canon).expect("write receipt");
        path
    }

    #[test]
    fn guard_accepts_clean_receipt() {
        let path = store_receipt("clean", &clean_sample_receipt());
        assert_eq!(may_record_pass(&path), Ok(()));
    }

    #[test]
    fn guard_refuses_dirty_receipt() {
        // Case 1: dirty sibling checkout.
        let mut dirty_sib = clean_sample_receipt();
        dirty_sib.siblings[0].dirty = true;
        dirty_sib.siblings[0].tree_hash = Some(SAMPLE_HASH.to_owned());
        let p_sib = store_receipt("dirty_sib", &dirty_sib);
        let err_sib = may_record_pass(&p_sib).expect_err("dirty sibling must be refused");
        assert_eq!(err_sib.reason, GUARD_REASON_DIRTY_STACK);
        assert!(matches!(
            err_sib.kind,
            GuardRefusalKind::DirtySiblings { ref fds } if fds == &["FD-FSIM"]
        ));

        // Case 2: dirty repository working tree.
        let mut dirty_repo = clean_sample_receipt();
        dirty_repo.repo_dirty = true;
        dirty_repo.repo_tree_hash = Some(SAMPLE_HASH.to_owned());
        let p_repo = store_receipt("dirty_repo", &dirty_repo);
        let err_repo = may_record_pass(&p_repo).expect_err("dirty repo must be refused");
        assert_eq!(err_repo.reason, GUARD_REASON_DIRTY_STACK);
        assert_eq!(err_repo.kind, GuardRefusalKind::DirtyRepo);
    }

    #[test]
    fn guard_refuses_tampered_receipt() {
        let path = store_receipt("tampered", &clean_sample_receipt());
        let mut raw = fs::read(&path).expect("read stored receipt");
        raw.push(b' ');
        fs::write(&path, &raw).expect("tamper stored receipt");
        let err = may_record_pass(&path).expect_err("tampered receipt must be refused");
        assert_eq!(err.reason, GUARD_REASON_DIRTY_STACK);
        assert!(matches!(err.kind, GuardRefusalKind::TamperedReceipt { .. }));
    }

    #[test]
    fn guard_refuses_missing_receipt() {
        let missing = fresh_dir("missing").join(format!("{SAMPLE_HASH}.json"));
        let err = may_record_pass(&missing).expect_err("missing receipt must be refused");
        assert_eq!(err.reason, GUARD_REASON_DIRTY_STACK);
        assert!(matches!(err.kind, GuardRefusalKind::MissingReceipt { .. }));
    }

    #[test]
    fn guard_refuses_asupersync_duplicate() {
        let mut with_asup = clean_sample_receipt();
        with_asup.duplicates = vec![
            "asupersync 0.5.0".to_owned(),
            "asupersync 0.6.2".to_owned(),
        ];
        let p = store_receipt("asup_dup", &with_asup);
        let err = may_record_pass(&p).expect_err("asupersync duplicate must be refused");
        assert_eq!(err.reason, GUARD_REASON_DIRTY_STACK);
        assert!(matches!(
            err.kind,
            GuardRefusalKind::AsupersyncDuplicate { ref entries } if entries.len() == 2
        ));

        // Non-asupersync duplicates alone do not trigger the asupersync guard.
        let mut other_dup = clean_sample_receipt();
        other_dup.duplicates = vec!["syn 1.0.109".to_owned(), "syn 2.0.90".to_owned()];
        let p_other = store_receipt("other_dup", &other_dup);
        assert_eq!(may_record_pass(&p_other), Ok(()));
    }
}
