//! Append-only on-disk store for the FrankenRobotics ten-fold harness
//! (bead `N39a-01`, §39.1).
//!
//! Deliberately lives OUTSIDE the scorer boundary (`BOUNDARY_FILES`): no file
//! under `score/` may import `crate::tenfold::store`.
//!
//! Writes are append-only: a new file is written under a temporary name in the
//! target directory and renamed into place; a second write of identical bytes
//! is a no-op, while a second write of different bytes is refused with
//! `store-overwrite` (exit 9). Never deletes any file or directory.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::tenfold::ids::{Arm, CampaignIndex, FamilyId, GateId, MissionHash, StratumId};
use crate::tenfold::refusal::TenfoldRefusal;

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

/// Outcome of an append-only [`write_once`] call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteOutcome {
    Created,
    IdenticalNoOp,
}

/// On-disk layout for one ten-fold gate directory (`tenfold/<gate>/`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateStore {
    root: PathBuf,
    gate: GateId,
}

impl GateStore {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>, gate: GateId) -> Self {
        Self {
            root: root.into(),
            gate,
        }
    }

    #[must_use]
    pub fn gate(&self) -> &GateId {
        &self.gate
    }

    #[must_use]
    pub fn gate_dir(&self) -> PathBuf {
        self.root.join("tenfold").join(self.gate.as_string())
    }

    #[must_use]
    pub fn roles_path(&self) -> PathBuf {
        self.gate_dir().join("roles.json")
    }

    #[must_use]
    pub fn seal_path(&self) -> PathBuf {
        self.gate_dir().join("seal.ndjson")
    }

    #[must_use]
    pub fn comments_path(&self) -> PathBuf {
        self.gate_dir().join("comments.ndjson")
    }

    #[must_use]
    pub fn strata_path(&self) -> PathBuf {
        self.gate_dir().join("strata.json")
    }

    #[must_use]
    pub fn families_path(&self) -> PathBuf {
        self.gate_dir().join("families.json")
    }

    #[must_use]
    pub fn preanalysis_path(&self) -> PathBuf {
        self.gate_dir().join("preanalysis.json")
    }

    #[must_use]
    pub fn public_card_path(&self, mission_hash: &MissionHash) -> PathBuf {
        self.gate_dir()
            .join("cards")
            .join(format!("{}.json", mission_hash.hex()))
    }

    #[must_use]
    pub fn sealed_card_path(&self, mission_hash: &MissionHash) -> PathBuf {
        self.gate_dir()
            .join("sealed")
            .join(format!("{}.json", mission_hash.hex()))
    }

    #[must_use]
    pub fn freeze_path(&self, arm: Arm, stratum: StratumId) -> PathBuf {
        self.gate_dir()
            .join("freeze")
            .join(format!("{}-{}.json", arm.as_str(), stratum.as_str()))
    }

    #[must_use]
    pub fn campaign_dir(
        &self,
        stratum: StratumId,
        family: &FamilyId,
        campaign: CampaignIndex,
        arm: Arm,
    ) -> PathBuf {
        self.gate_dir()
            .join("campaigns")
            .join(stratum.as_str())
            .join(family.hex())
            .join(campaign.0.to_string())
            .join(arm.as_str())
    }

    #[must_use]
    pub fn campaign_record_path(
        &self,
        stratum: StratumId,
        family: &FamilyId,
        campaign: CampaignIndex,
        arm: Arm,
    ) -> PathBuf {
        self.campaign_dir(stratum, family, campaign, arm)
            .join("campaign.json")
    }

    #[must_use]
    pub fn intervals_path(
        &self,
        stratum: StratumId,
        family: &FamilyId,
        campaign: CampaignIndex,
        arm: Arm,
    ) -> PathBuf {
        self.campaign_dir(stratum, family, campaign, arm)
            .join("intervals.ndjson")
    }

    #[must_use]
    pub fn ledger_path(
        &self,
        stratum: StratumId,
        family: &FamilyId,
        campaign: CampaignIndex,
        arm: Arm,
    ) -> PathBuf {
        self.campaign_dir(stratum, family, campaign, arm)
            .join("ledger.json")
    }

    #[must_use]
    pub fn transcript_path(
        &self,
        stratum: StratumId,
        family: &FamilyId,
        campaign: CampaignIndex,
        arm: Arm,
    ) -> PathBuf {
        self.campaign_dir(stratum, family, campaign, arm)
            .join("transcript.ndjson")
    }

    #[must_use]
    pub fn receipt_path(
        &self,
        stratum: StratumId,
        family: &FamilyId,
        campaign: CampaignIndex,
        arm: Arm,
        receipt_index: u32,
    ) -> PathBuf {
        self.campaign_dir(stratum, family, campaign, arm)
            .join("receipts")
            .join(format!("{receipt_index}.json"))
    }

    #[must_use]
    pub fn report_path(&self) -> PathBuf {
        self.gate_dir().join("report.json")
    }

    /// Create the standard gate store directory tree idempotently.
    pub fn init_dirs(&self) -> Result<(), TenfoldRefusal> {
        let base = self.gate_dir();
        for sub in ["cards", "sealed", "freeze", "campaigns"] {
            let p = base.join(sub);
            fs::create_dir_all(&p).map_err(|e| {
                TenfoldRefusal::store_overwrite(
                    p.display().to_string(),
                    format!("create_dir_all failed: {e}"),
                    "writable gate store directory",
                )
            })?;
        }
        Ok(())
    }

    /// Write `bytes` once to `target`: creates parent directories, writes to a
    /// temporary file and renames into place; refuses differing bytes with
    /// `store-overwrite` (exit 9) and treats identical bytes as a no-op.
    pub fn write_once(&self, target: &Path, bytes: &[u8]) -> Result<WriteOutcome, TenfoldRefusal> {
        write_once(target, bytes)
    }
}

/// Append-only atomic write-once helper (`N39a-01`).
///
/// If `target` exists:
/// - identical bytes -> `Ok(WriteOutcome::IdenticalNoOp)`
/// - different bytes -> `Err(TenfoldRefusal { reason: StoreOverwrite, .. })` (exit 9)
///
/// If `target` does not exist: writes to a unique temporary file in the same
/// parent directory and renames it into place. Never deletes files.
pub fn write_once(target: &Path, bytes: &[u8]) -> Result<WriteOutcome, TenfoldRefusal> {
    let target_label = target.display().to_string();
    if target.exists() {
        let existing = fs::read(target).map_err(|e| {
            TenfoldRefusal::store_overwrite(
                &target_label,
                format!("unreadable existing artifact: {e}"),
                "readable existing artifact or absent path",
            )
        })?;
        if existing == bytes {
            return Ok(WriteOutcome::IdenticalNoOp);
        }
        let existing_hash = frob_cert::blake3::hash_hex(&existing);
        let offered_hash = frob_cert::blake3::hash_hex(bytes);
        return Err(TenfoldRefusal::store_overwrite(
            target_label,
            format!("b3:{offered_hash} ({} bytes)", bytes.len()),
            format!(
                "identical bytes matching existing b3:{existing_hash} ({} bytes)",
                existing.len()
            ),
        ));
    }

    let parent = target.parent().ok_or_else(|| {
        TenfoldRefusal::store_overwrite(
            &target_label,
            "target path has no parent directory",
            "file path inside a directory",
        )
    })?;
    fs::create_dir_all(parent).map_err(|e| {
        TenfoldRefusal::store_overwrite(
            &target_label,
            format!("failed creating parent dir: {e}"),
            "writable store directory",
        )
    })?;

    let fname = target
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("record");
    let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp_path = parent.join(format!(".{fname}.tmp.{}.{seq}", std::process::id()));

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp_path)
        .map_err(|e| {
            TenfoldRefusal::store_overwrite(
                &target_label,
                format!("failed creating staging file {}: {e}", tmp_path.display()),
                "fresh staging file",
            )
        })?;
    file.write_all(bytes).map_err(|e| {
        TenfoldRefusal::store_overwrite(
            &target_label,
            format!("failed writing staging file: {e}"),
            "complete byte write",
        )
    })?;
    file.sync_all().map_err(|e| {
        TenfoldRefusal::store_overwrite(
            &target_label,
            format!("failed syncing staging file: {e}"),
            "durable staging file",
        )
    })?;
    drop(file);

    // Re-check target immediately before rename so a concurrent writer's
    // differing bytes are never silently overwritten.
    if target.exists() {
        let existing = fs::read(target).map_err(|e| {
            TenfoldRefusal::store_overwrite(
                &target_label,
                format!("unreadable existing artifact on re-check: {e}"),
                "readable existing artifact",
            )
        })?;
        if existing == bytes {
            return Ok(WriteOutcome::IdenticalNoOp);
        }
        return Err(TenfoldRefusal::store_overwrite(
            target_label,
            "concurrent write with differing bytes",
            "identical bytes",
        ));
    }

    fs::rename(&tmp_path, target).map_err(|e| {
        TenfoldRefusal::store_overwrite(
            &target_label,
            format!("atomic rename failed: {e}"),
            "atomic move of staged file into store path",
        )
    })?;
    Ok(WriteOutcome::Created)
}
