//! Gauntlet verification crate for FrankenRobotics (`BD-025`, `STD-E2E`).
//!
//! - `compendium`: exact mathematical oracle library, registry validator, and doc renderer (`N12-41`).
//! - `e2e`: `std`-only end-to-end log tooling (`STD-E2E`).
//! - `gates_v0`: v0 reader for `registries/gates.toml` (`frob.gates/1`, `BD-025`).
//! - `hypotheses`: hypothesis verdict register (`registry/hypotheses.toml`, `frob.hypothesis/1`),
//!   append-only change rule, wording lint, scope binding, open disagreements, and
//!   `frob.prove.effects.out/1` certificate-effect read-out (`N00-10`).
//! - `controls`: negative-control registry validator, serializer, and `GATE-GAUNT-01`
//!   registry check (`BD-025`, `N38b-04`).
//! - `control_runner`: negative-control runner for all five control kinds (`mutation`,
//!   `planted-input`, `planted-design`, `planted-world`, `a-a`), annex driver hook
//!   `DRIVERS`, run history (`control_history.ndjson`), suspect-control review
//!   (`control_reviews.toml`), permanent-control procedure, and mutation guard (`N38b-04`).
//! - `design_golden`: independent golden corpus harness for `frob.design/1` canonical
//!   bytes and `DesignId` (`N14a-26`).
//! - `oracles`: oracle register (`registry/oracles.toml`), role grammar, pin-move audit,
//!   pinned fixtures, and disagreement findings (`N38a-04`).
//! - `perf`: performance-program conformance lints and KA-815 workload checks (`N36-02`).
//! - `pilots`: pilot register (`registry/pilots.toml`, `frob.gauntlet.pilot/1`), seed list
//!   (`registry/pilots_expected.toml`), frozen protocols, funding rule, completeness check,
//!   Moonshot cap, and `docs/delivery/pilots.md` printout (`N38a-09`, `GATE-GAUNT-08`).
//! - `reasons`: closed Gauntlet harness refusal vocabulary (`registry/reasons.toml`),
//!   existing public `FROB-REF-nnn` code references, and source/log scanners (`N38-REF`).
//! - `register`: base gate-register record types (`Claim`, `Status`, `GateBase`, `PartBase`),
//!   D0 TOML printer, and plan-time/revision count assertions (`N38b-18`; `frob-core`-only).
//! - `tenfold`: ten-fold comparative benchmark harness skeleton, five harness records,
//!   canonical bytes, append-only store, and dependency-free scorer boundary (`N39a-01`).
//! - `tiers`: Gauntlet conformance tiers `G0`–`G5`, execution sites, deterministic selection
//!   with tier filter extensions, same-run control execution, and hash-named canonical receipts
//!   (`frob.gauntlet.receipt/1`, `N38a-02`).

#[cfg(all(feature = "conformance", not(debug_assertions)))]
compile_error!(
    "frob-conformance: feature `conformance` (and its negative-control mutation features) \
     is forbidden in release builds (`not(debug_assertions)`); see N38b-04"
);

pub mod admission;
pub mod compendium;
pub mod control_runner;
pub mod controls;
pub mod design_golden;
pub mod e2e;
pub mod gates_v0;
pub mod hypotheses;
pub mod oracles;
pub mod perf;
pub mod pilots;
pub mod reasons;
pub mod register;
pub mod tenfold;
pub mod tiers;
