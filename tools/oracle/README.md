# Oracle Capture Scripts (`tools/oracle/`)

Every fixture in `tests/fixtures/oracle/` and `crates/frob-conformance/fixtures/`
is produced exclusively by a capture script under `tools/oracle/<oracle>/<fixture>.sh`
(or the oracle's own language, run out of process) — never regenerated from any
FrankenRobotics build product (`INV-GAUNT-01`, `N38a-04`, `GATE-GAUNT-05`).

## Script contract

Every capture script under `tools/oracle/`:
1. Accepts `--pin <pin>` and `--out <fixture.toml>`.
2. Prints the installed oracle tool's version string.
3. Refuses to run (non-zero exit) when the installed version or pin differs from `--pin`.
4. Declares no workspace build product (`target/`, `crates/`, `frob`, `frobd`, `frob-mcp`, `frob-verify`) in its inputs (`# inputs:`).
5. Writes a `kind = "oracle"` fixture with `oracle`, `pin`, `captured`, `script` (path + SHA-256), `inputs` (paths + SHA-256), `units`, `tolerance_note`, and whole-file `bytes_sha256`.

## Registered scripts

| Script | Oracle | Fixture | Reproduction command (from an empty machine) |
|---|---|---|---|
| `tools/oracle/analytic/pendulum_period.sh` | `analytic-derivations` | `crates/frob-conformance/fixtures/pendulum_period.toml` | `tools/oracle/analytic/pendulum_period.sh --pin "docs/derivations/" --out crates/frob-conformance/fixtures/pendulum_period.toml` |
