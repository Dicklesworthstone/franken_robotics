//! Canonical referee run identity and transactional records (BD-024).

use std::io::{self, Read};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use frob_core::canon::{CanonError, Canonical, CanonicalBytes, Value, to_canonical};
use frob_core::graph_id::TraceId;
use frob_core::id::{Blake3, hash_blob};
use frob_core::stack::BUILT_UNDER;
pub use frob_fsim::ledger::{FsqliteLedger, LedgerRefusal, TableId, Txn};
use frob_fsim::ledger::{OpenOptions, SchemaVersion};

pub const CHUNK_STEPS: i64 = 256;
pub const TICKS_PER_SECOND: i64 = 480;
pub const DEFAULT_STEPS: i64 = 2 * TICKS_PER_SECOND;

pub struct Manifest<'a> {
    pub model_id: &'a str,
    pub foundation_pin: &'a str,
    pub steps: i64,
    pub seed: u64,
    pub perturbed: bool,
}

impl Canonical for Manifest<'_> {
    fn write(&self, value: Value<'_>) -> Result<(), CanonError> {
        value.object(|o| {
            o.str("design", "g1")?;
            o.str("model_id", self.model_id)?;
            o.str("tier", "referee")?;
            o.array("h", |a| {
                a.push_int53(1)?;
                a.push_int53(TICKS_PER_SECOND)
            })?;
            o.int53("steps", self.steps)?;
            o.int53("chunk_steps", CHUNK_STEPS)?;
            o.u64str("seed", self.seed)?;
            o.str("foundation_pin", self.foundation_pin)?;
            o.str("kernel", "frob-fsim/referee:step_free")?;
            o.str("stack_receipt", &BUILT_UNDER.receipt.to_hex())?;
            o.bool("execution_authorized", false)?;
            o.bool("hardware_safety_signoff", false)?;
            o.bool("sim_to_real_certified", false)?;
            if self.perturbed {
                o.str("perturb_ulp", "q0")?;
            }
            Ok(())
        })
    }
}

impl Manifest<'_> {
    pub fn digest(&self) -> Result<Blake3, CanonError> {
        Ok(hash_blob(to_canonical(self)?.as_bytes()))
    }

    pub fn run_row(&self, run_id: &str, state: &str) -> Result<CanonicalBytes, CanonError> {
        canonical(|v| {
            v.object(|o| {
                o.str("run_id", run_id)?;
                o.str("manifest_hash", &self.digest()?.to_hex())?;
                o.member_term("manifest", self)?;
                o.str("state", state)
            })
        })
    }
}

pub fn open(path: &Path) -> Result<FsqliteLedger, LedgerRefusal> {
    FsqliteLedger::open(path, SchemaVersion::new(1), OpenOptions::default())
}

pub fn episode_key(run_id: &str) -> String {
    format!("{run_id}/0/0")
}

pub fn request_key(principal: &str, key: &str) -> Result<String, CanonError> {
    let bytes = canonical(|v| {
        v.array(|a| {
            a.push_str(principal);
            a.push_str("sim.run");
            a.push_str(key);
            Ok(())
        })
    })?;
    Ok(format!("sim.run/key/{}", hash_blob(bytes.as_bytes())))
}

pub fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

pub fn new_trace(start_ms: u64) -> io::Result<TraceId> {
    let mut random = [0; 16];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut random)?;
    Ok(TraceId::from_parts(start_ms, u128::from_be_bytes(random)))
}

pub fn execution_row(
    run_id: &str,
    trace: TraceId,
    end_ms: u64,
) -> Result<CanonicalBytes, CanonError> {
    canonical(|v| {
        v.object(|o| {
            o.str("run_id", run_id)?;
            o.str("trace", &trace.to_string())?;
            o.u64str("start_ms", trace.timestamp_ms())?;
            o.u64str("end_ms", end_ms.max(trace.timestamp_ms()))
        })
    })
}

fn canonical(
    f: impl Fn(Value<'_>) -> Result<(), CanonError>,
) -> Result<CanonicalBytes, CanonError> {
    struct Term<F>(F);
    impl<F: Fn(Value<'_>) -> Result<(), CanonError>> Canonical for Term<F> {
        fn write(&self, value: Value<'_>) -> Result<(), CanonError> {
            (self.0)(value)
        }
    }
    to_canonical(&Term(f))
}
