# Dependency Admission Ledger (`DEPENDENCY_ADMISSION.md`)

A foundation (a sibling library or external crate consumed by FrankenRobotics) is not used above the airlock crate until its admission gate has a record in this file.

## Admission Rules

1. **Validity rule.** An admission record is valid only if it cites a commit of this repository whose tracked `Cargo.lock` contains the foundation, together with the exact command, its output (and content-addressed output hash), and the stack resolution receipt hash.
2. **One record per admission gate run, passing or failing.** Every gate attempt is recorded honestly; a failing or blocked observation is a record, and no gate `PASS` is recorded while any sibling checkout or the repository working tree is dirty (such observations are recorded as `development build`).
3. **Fields shown by each record.** Gate identifier, foundation identifier, the commit of this repository whose tracked `Cargo.lock` contains the foundation (or core skeleton for bootstrap), the command, its exit code and output excerpt/hash, the resolution receipt hash, measured values against `TARGET` thresholds, and date.
4. **Never inherited.** An admission is never inherited: a crate or feature group first used at a later milestone gets its own record then, at the pin of that day.

## Admission Status Table

| Gate | Foundation | Status | Measured vs Threshold | Receipt | Date |
|---|---|---|---|---|---|
| `INT-TOOLCHAIN-01` | `WORKSPACE` | not run | — | — | — |
| `INT-FSIM-01` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-02` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-03` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-04` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-05` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-06` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-07` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-08` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-09` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-10` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-11` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-13` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-14` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-15` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-16` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-17` | `FD-FSIM` | not run | — | — | — |
| `INT-FSIM-18` | `FD-FSIM` | not run | — | — | — |
| `INT-ASUP-01` | `FD-ASUP` | not run | — | — | — |
| `INT-ASUP-02` | `FD-ASUP` | not run | — | — | — |
| `INT-FSQL-01` | `FD-FSQL` | not run | — | — | — |
| `INT-FSQL-02` | `FD-FSQL` | not run | — | — | — |
| `INT-FT-01` | `FD-FT` | not run | — | — | — |
| `INT-FT-02` | `FD-FT` | not run | — | — | — |
| `INT-CUDA-01` | `FD-CUDA` | not run | — | — | — |
| `INT-FSYM-01` | `FD-FSYM` | not run | — | — | — |
| `INT-FSYM-02` | `FD-FSYM` | not run | — | — | — |
| `INT-FLN-01` | `FD-FLN` | not run | — | — | — |
| `INT-FLN-02` | `FD-FLN` | not run | — | — | — |
| `INT-FMCP-01` | `FD-FMCP` | not run | — | — | — |
| `INT-FNP-01` | `FD-FNP` | not run | — | — | — |
| `INT-FNP-02` | `FD-FNP` | not run | — | — | — |
| `INT-FTUI-01` | `FD-FTUI` | not run | — | — | — |
| `INT-F3D-01` | `FD-F3D` | not run | — | — | — |
| `INT-FDGR-01` | `FD-FDGR` | not run | — | — | — |
| `INT-FJAX-01` | `FD-FJAX` | not run | — | — | — |
| `INT-FTX-01` | `FD-FTX` | not run | — | — | — |
