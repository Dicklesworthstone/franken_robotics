# ADR-GAUNT-04: Reconciliation of the Gate Register Status Set (`frob.gates/1`)

- **Status:** Accepted
- **Date:** 2026-11-24
- **Beads:** `N38b-18` (base record), `N40-01` (register layer, validator, write path)
- **Schema:** `schemas/registries/gates.schema.json` (`frob.gates/1`)
- **Plan references:** §40.1, §38.13, §38.14 (`INV-GAUNT-02`, `-05`, `-06`, `-07`), §38.1 (`ADR-GAUNT-01`, `-02`, `-03`)

## Context

Two sections of the comprehensive plan described the status values of a gate row in `registries/gates.toml` using overlapping but non-identical lists:

1. **The gate-register chapter (§40.1)** listed five human-readable statuses: `not run`, `pass`, `fail`, `killed: fallback adopted`, and `blocked: <open question>`.
2. **The Gauntlet schema (§38.13)** listed six machine tokens: `not-run`, `pass`, `fail`, `killed`, `no-claim`, and `superseded`.

Furthermore, 40 convergence gates (including `GATE-MARROW-01`, `GATE-HAB-01`, `GATE-HAB-13`, `GATE-ARENA-01`, and `GATE-LEMMA-03` per `N40-02` step 5) plus 3 admission gates (`INT-FSIM-07`, `INT-FMCP-01`, `INT-FDGR-01`) carry distinct requirements due at multiple milestone exits. Storing a single status on `[[gate]]` would force either a premature `pass` before later parts run or an inaccurate `not-run` after earlier parts pass.

## Decision

1. **Closed seven-value status union (`Status`):** `registries/gates.toml` (`frob.gates/1`) uses the exact reconciled union of seven machine tokens on every `[[gate.part]]`:
   - `not-run` — printed `"not run"`; open, never counted as passed. A gate with zero recorded parts derives `not-run`.
   - `pass` — requires non-empty `measured` (never byte-equal to `threshold`), `commit`, `command`, `receipt` (resolving under `receipts/gates/<hash>.json` and hashing to `<hash>`, including receipt-set members), `fingerprint`, `seed`, resolving `claim_id` (`clm:<64 hex>`) and `evidence`, non-empty `controls` with every due control caught in the same run, and `recorded.session` absent from `recorded.implemented_by`.
   - `fail` — a first-class reproducible record requiring non-empty `measured` (not equal to `threshold`), `commit`, `command`, and a resolving committed `receipt` under `receipts/gates/<hash>.json`.
   - `killed` — printed `"killed: fallback adopted (<fallback>)"`; requires the triggering kill gate (or for a `program` row with no kill gate, the governing `BET-…`), the adopted fallback, and the recording `ADR-…` in `detail` (or `kill_gate`/`fallback`/`detail`).
   - `blocked` — printed `"blocked: OPEN-…"` or `"blocked: claim-reading: <bead>"`; requires either an `OPEN-…` identifier or `claim-reading: <bead>` in `detail` (the only two admitted blocking causes).
   - `no-claim` — terminal status valid **only** on gates with `claim = "reality"` (e.g. `needs = "import-lane"` gates such as `GATE-GROUND-10`), requiring an explicit reason in `detail` (such as `"no-dataset-admitted"`).
   - `superseded` — terminal replacement status requiring the existing successor gate identifier in `detail`.
2. **Per-part status placement:** `status`, `measured`, `evidence`, `recorded`, and register-layer provenance (`scope`, `commit`, `command`, `receipt`, `fingerprint`, `seed`, `detail`) live exclusively on `[[gate.part]]`. A gate's status is derived (`not-run` when no part is recorded or due; otherwise the status of its latest due part) and is never stored on `[[gate]]`.
3. **Kill rows carry no measurement fields:** Rows with `kind = "kill"` decide and measure nothing; they carry `trigger`, `fallback`, `decided`, `mode`, `reads`, `decision` (and optionally `reread`, `scope_kind`), and must never carry `claim`, `claim_id`, `threshold`, `tier`, `controls`, or `[[gate.part]]`.
4. **Enforced by one validator and write path:** All nine register rules and all ten write-time refusals (`N38-REF` reason vocabulary) are enforced exclusively in `frob_conformance::register::{validate,write}` so every generated view (README table, exit query, dashboard, release rubric) reads one reconciled source of truth.

## Consequences

- `BaseDocument` (`N38b-18`) and `Register` (`N40-01`) share one `Claim` enum, one `Status` enum, and one schema file (`schemas/registries/gates.schema.json`).
- No boolean `passed` flag can exist; unrun work always renders `"not run"`.
- `GATE-GROUND-10` (`needs = "import-lane"`, `claim = "reality"`) transitions cleanly from `blocked` (`OPEN-GROUND-1`) to `no-claim` (`no-dataset-admitted`) and never enters any milestone exit's due set.
