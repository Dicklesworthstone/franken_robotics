---
key: "STD-E2E"
title: "scripts/e2e: end-to-end harness, structured NDJSON log, runner, linter, gate report and self-test"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "test", "e2e", "delivery"]
parent: "WP-14"
deps: []
needs: []
related: ["STD-TEST", "BD-001", "BD-022", "BD-025", "BD-004", "BD-095"]
gates: []
plan_refs: ["§38", "§38.11", "§32.4", "§32.5", "§37.6", "§29.14", "§34.6", "§34.7", "§0.2"]
---
## Background

FrankenRobotics (a robotics simulation and design system whose every capability is reached through one command-line
binary, `frob`, with a daemon `frobd` and an MCP server `frob-mcp`) proves that a capability works from the user's
side with end-to-end scripts: plain shell that drives the real binaries on real fixtures. Every other bead of the
program writes such a script and depends on this bead for the shared helper, the log format and the tools that read
the logs. The harness belongs to the Gauntlet, the verification subsystem (crate `frob-conformance`, layer 7, tier
Solid), whose first duty is that nothing records a pass it did not earn.

Three rules of the program shape the design. (1) "A gate that no deficient model can fail validates nothing": a
script must be able to fail, so the harness has its own negative controls and can verify each script's planted
failures. (2) No self-certification: a harness driven through the surface it judges is a path to it, so the harness
is not a `frob` verb, is not an `xtask` subcommand (`xtask` holds exactly six checks and runs no physics), and
never writes a gate's status; it produces evidence that a fresh-context recording session reads. (3) Private places
stay private: anything derived from a scan of a real home or workplace carries the mark `private`, and no log may
contain such data.

DESIGN decisions and rejected alternatives.
- One compiled companion, `frob-e2e`, writes and reads every log line. Reason: shell cannot escape JSON, time a
  command in milliseconds portably (macOS ships bash 3.2 and BSD `date`), extract a field of a JSON envelope or
  compare a rational; and one writer means the schema cannot drift from the logs. Rejected: `jq` plus `printf`
  (escaping defects, tool-version drift, nothing validated at write time); a Python helper (no interpreter in this
  repository outside one optional binding); logging from inside `frob` (the judged surface would write its own
  evidence, and library crates do not print).
- `frob-e2e` uses the standard library only and carries its own small strict JSON reader and writer. Reason: it
  must exist before `frob-core` does, and a harness that parses an envelope with the same reader that wrote it can
  share that reader's defect (the same argument that keeps the offline auditor free of dependencies).
- Script metadata lives in a header inside each script, not in a second manifest. Reason: two copies of a fact are
  two chances to disagree.

Terms. *Envelope*: the single JSON object `frob --json` writes to standard output (`schema: "frob.envelope/1"`),
with `ok`, `exit`, `result`, `evidence`, `bound`, `cost`, and `refusal` when `ok` is false. *Evidence block*: the
envelope's `evidence` object (claim kinds it may be cited for, evidence class Proved/Certified/Tested/Estimated,
tier, determinism class D0 to D4, model identity, `reality`). *Gate*: a numbered pass criterion (`GATE-…`) whose
status lives in `registries/gates.toml`. *Negative control*: a planted defect a test must catch.

## Scope

In: `scripts/e2e/lib.sh`; `scripts/e2e/run-all.sh`; the tool `frob-e2e` (emit, run, get, cmp, lint, report,
transcript, schema, list); the log schema `schemas/e2e-log.schema.json` (`frob.e2elog/1`); the directory layout;
redaction; the self-test with its negative controls. Out: unit-test conventions and the registries (STD-TEST); the
per-bead delivery log `frob.beadlog/1` and the artifact graph (BD-095); recording a gate in `registries/gates.toml`
(the Gauntlet's status accounting, drafted with BD-025); each capability's own script (its bead).

## Technical approach

**Files (DESIGN).**
```
scripts/e2e/lib.sh            sourced helper            scripts/e2e/run-all.sh     runner
scripts/e2e/<name>.sh         one script per capability scripts/e2e/selftest.sh    harness self-test
scripts/e2e/selftest/st_*.sh  fixture scripts of the self-test
crates/frob-conformance/src/bin/frob-e2e.rs, src/e2e/{json,line,lint,report,sha256}.rs   the tool (std only)
schemas/e2e-log.schema.json   printed by `frob-e2e schema`; compared with a fresh print by the self-test
```
Scripts are bash, start with `#!/usr/bin/env bash`, use no bash feature newer than 3.2, and run on arm64 macOS
and x86-64 Linux. `lib.sh` builds the tool on first use with `cargo build -p frob-conformance --bin frob-e2e`
through the build-offload shim (cargo is never invoked by absolute path) and caches the path in `FROB_E2E_TOOL`.

**Script header.** Lines beginning `# e2e:` with `key=value` pairs, read by `frob-e2e list`:
`bead` (the owning bead key), `milestone` (M0…M8), `labels` (comma list), `gates` (comma list), `seed` (decimal
u64), `timeout_s` (default 600; above 3600 needs `--approved-envelope <id>`, because no run may exceed one hour,
16 GiB or 8 cores without the owner's written approval), `needs` (`cpu`, `cuda`, `metal`, `frobd`, `mcp`,
`second-host`), `plants` (comma list of planted failures the script supports).

**Helper functions (`lib.sh`).** All are prefixed `e2e_`; each writes one or more log lines through the tool.
1. `e2e_init <script> <bead>`: resolves binaries (`FROB_BIN` default `frob` on PATH then `target/release/frob`;
   `FROBD_BIN`, `FROB_MCP_BIN`), creates the log and scratch directories, fixes the run identifier and the seed
   (`FROB_E2E_SEED`, alias `E2E_SEED`, default the header's), installs an EXIT trap that always writes the
   `summary`, and emits the script-level `start`.
2. `e2e_step <id> <title>`: closes the previous step (`end`) and opens a new one (`start` with `step` set).
3. `e2e_cmd [--name n] [--expect-exit 0,2] -- <command…>`: runs the command with standard output and error
   captured to the artifacts directory; emits `cmd`; sets `E2E_LAST_EXIT`, `E2E_LAST_STDOUT`; if the exit code is
   not expected, records a failed assertion `exit:<name>`. Default expected exit is 0. Exit 8 ("accepted and still
   running") is never treated as success unless listed.
4. `e2e_get <json-pointer>`: prints a field of the last envelope (RFC 6901 pointer); a missing field fails.
5. Assertions, each emitting `assert` with `status` `pass` or `fail`: `e2e_assert_eq name observed expected`
   (byte equality); `e2e_assert_num name observed op threshold [unit]` (op in `<`, `<=`, `>`, `>=`, `==`;
   decimal strings compared exactly as rationals, never through binary floating point); `e2e_assert_close name
   observed expected abs|rel tol [unit]`; `e2e_assert_rational name observed expected` (`p/q` reduced and
   compared); `e2e_assert_digest_eq name a b`; `e2e_assert_json name pointer expected` (on the last envelope);
   `e2e_assert_file name path`; `e2e_assert_repairs_order name code1,code2,…` (ranked repairs of the last refusal).
   Optional trailing tags `--gate GATE-…`, `--ka KA-nnn`, `--cx CX-nnn`, `--nc NC-nnnn`, `--mr MR-nn`.
6. `e2e_expect_refusal <FROB-REF-nnn> [exit] -- <command…>`: runs the command; passes only if the envelope has
   `ok: false`, the code matches and the exit code matches (default 3); emits `refusal` (`expected: true`) and an
   `assert`. A success envelope fails it. Any refusal met outside this function is logged with `expected: false`
   and counts as a failed assertion.
7. `e2e_expect_caught <control> -- <command…>`: the negative-control form. The command is a check run against a
   planted defect and must exit non-zero or refuse; emits `assert` with `kind: "negative-control"` and
   `caught: true|false`. A control that is not caught fails the script.
8. `e2e_metric name value unit [op threshold] [--gate GATE-…] [--interval lo hi] [--n count]`: emits `metric`,
   and, when a threshold is given, an `assert` on it. A measured value is always logged against its threshold.
9. `e2e_gate GATE-…`: declares a gate touched; `e2e_skip reason`: emits `assert` with `status: "skip"`.
10. `e2e_plant <name>`: true when `E2E_PLANT_FAILURE` equals `<name>`; the script then injects that failure.

**Exit codes of a script.** 0: at least one assertion ran, none failed, no control was missed. 1: an assertion
failed, a control was missed, or an unexpected refusal occurred. 2: harness error (tool missing, log not
writable, bad usage, zero assertions: a script that asserts nothing is not a pass). 77: skipped as a whole (a
needed device or binary is absent); a skip is never counted as a pass.

**Idempotence and seeds.** A script works only inside `target/e2e-scratch/<script>/<run_id>/` (its own project
root, registries copy and ledger, passed to `frob` by `--root`), never edits the repository's registries, and
gives the same assertions on every run. Every random choice comes from the declared seed, which is passed to
`frob` explicitly and repeated on every log line. `E2E_SMALL=1` (alias `E2E_QUICK`) shrinks sizes; the log then
says `scale: "small"` and `gates_claimable: false`. `E2E_DEVICES` lists devices (`cpu,cuda:0,cuda:1,metal`).
`E2E_DRY_RUN=1` prints commands and exits 77. `FROB_E2E_SECOND_HOST` names the host of the cross-platform lane.

**Layout.**
```
target/e2e-logs/<script>/<utc>.ndjson            <utc> = YYYYMMDDTHHMMSSZ, suffixed -<pid> on collision
target/e2e-logs/<script>/<utc>.transcript.txt    human-readable, written line by line with the log
target/e2e-logs/<script>/<utc>.artifacts/        <step>.<seq>.stdout, .stderr, files a failure needs
target/e2e-logs/<script>/LATEST                  text file naming the newest <utc> (no symbolic link)
target/e2e-logs/_runs/<utc>.run.ndjson           runner index: one line per script
target/e2e-report/<utc>/gates.json, gates.md     the gate report
```
The transcript shows, per line: time, step, event, the command as typed, exit code and wall time, each assertion
as `PASS`/`FAIL name: observed vs expected (tolerance, unit)`, each metric with its threshold, each refusal with
its code and first repair, and the summary block.

**Log schema `frob.e2elog/1`.** One JSON object per line, UTF-8, no line longer than 64 KiB.

| Field | On | Type and meaning |
|---|---|---|
| `schema`, `ts`, `seq`, `run_id`, `script`, `bead`, `step`, `event`, `seed` | every line | `"frob.e2elog/1"`; RFC 3339 UTC with milliseconds; integer from 0, strictly increasing; `<utc>-<8 hex>`; script name; bead key; step id (`""` at script level); one of `start`, `cmd`, `assert`, `metric`, `refusal`, `end`, `summary`; decimal string |
| `commit`, `dirty`, `toolchain`, `frob_version`, `host{os,arch,fingerprint}`, `scale`, `devices`, `plant`, `argv` | script `start` | Provenance of the run; `fingerprint` is the hardware fingerprint digest, never a host name |
| `title` | step `start` | Step title |
| `argv`, `cmdline`, `cwd`, `exit`, `expected_exit`, `wall_ms`, `stdout_path`, `stderr_path`, `stdout_sha256` | `cmd` | The exact command (array and one shell-quoted string), directory relative to the scratch root, codes, wall time, capture paths and digest |
| `envelope{schema,verb,ok,exit}`, `evidence{claims,class,tier,determinism,identities,seed,reality}`, `cost`, `job` | `cmd` when standard output is an envelope | The evidence block copied verbatim; `identities` is the envelope's `bound` block plus `model_id` |
| `name`, `kind`, `status`, `observed`, `expected`, `op`, `tolerance`, `unit`, `gate`, `ka`, `cx`, `nc`, `mr`, `caught` | `assert` | `kind` in `eq`, `num`, `close`, `rational`, `digest`, `json`, `file`, `exit`, `refusal`, `negative-control`, `determinism`; `status` in `pass`, `fail`, `skip` |
| `first_diff{path,observed,expected}`, `repro{cmd_seq,artifacts}` | failed `assert` | The first differing value and what is needed to reproduce: the `seq` of the command and artifact paths |
| `name`, `value`, `unit`, `threshold{op,value}`, `interval`, `n`, `gate` | `metric` | `value` is a JSON number, or a string for exact rationals and integers above 2⁵³ |
| `code`, `exit`, `message`, `pointer`, `observed`, `required`, `unit`, `retry`, `repairs`, `expected` | `refusal` | Copied from the envelope's `refusal`; `repairs` in rank order |
| `wall_ms`, `asserts_passed`, `asserts_failed`, `exit` | `end` | Per step; `exit` only at script level |
| `asserts_passed`, `asserts_failed`, `asserts_skipped`, `controls_caught`, `controls_missed`, `refusals_expected`, `refusals_unexpected`, `gates[{id,measured_ok,controls_caught}]`, `gates_claimable`, `exit`, `wall_ms`, `body_sha256` | `summary` (last line) | Counts, gates touched, and the SHA-256 of every byte before this line |

**Redaction.** (1) When an envelope carries the mark `private` (the mark is the join over the lineage of its
inputs), the `cmd` line keeps only verb, codes, wall time, identities and the evidence block; captured output is
written under the private root, not under `target/`, and only its digest is logged. (2) An argument or path that
resolves beneath `FROB_PRIVATE_ROOT` is logged as `<private:` + the first 12 hex digits of its SHA-256 + `>`.
(3) The writer refuses, at any depth, the member names `voxels`, `occupancy`, `points`, `mesh`, `image`, `pixels`,
`depth_map`, `rgb`, `scan`, `floorplan`, `pose_trace`, and any string above 4,096 bytes. (4) The linter repeats
(2) and (3) on finished logs and searches them for the canary values planted by the self-test.

**Linter (`frob-e2e lint <log>…`).** Fails when: a line is not valid against the schema; `seq` is not contiguous;
the first line is not a script `start` or the last is not a `summary`; the summary's counts differ from a recount
of the `assert` and `refusal` lines; `body_sha256` does not match; a `cmd` line lacks its exit code or wall time;
a failed assertion lacks `first_diff` or `repro`; a `metric` tagged with a gate has no threshold; the log claims
a gate with `scale: "small"`; a gate is touched and `controls_caught` for it is zero while the summary says
`measured_ok: true` (reported as `no-control`, never as pass); a redaction rule is broken.

**Runner (`run-all.sh`).** `--milestone M1` (exact), `--through M2` (cumulative), `--label a,b` (all must match),
`--gate`, `--bead`, `--script`, `--list`, `--small`, `--devices`, `--fail-fast`, `--verify-plants`. It runs the
selected scripts one at a time under their timeouts (the process group is killed on expiry and when the runner is
interrupted), lints each log, writes the run index and prints one line per script. Exit 0 only if every selected
script passed and linted; 1 if any failed; 2 on harness error or when the selection is empty. `--verify-plants`
runs each script once per declared plant with `E2E_PLANT_FAILURE=<plant>` and requires exit 1 with a failed
assertion that names the plant.

**Gate report (`frob-e2e report`).** Reads the headers, the newest log of each script and, when present,
`registries/gates.toml`; writes one row per gate: milestone, scripts that touch it, last run, result (`pass`,
`fail`, `not-run`, `small-only`, `no-control`), measured values with thresholds, controls caught. It is a view. It
never writes `registries/gates.toml`: a status is entered by a session that did not implement the capability.

## Acceptance criteria

- [ ] `scripts/e2e/lib.sh`, `run-all.sh`, `selftest.sh` and the tool exist; `frob-e2e schema` prints exactly
      `schemas/e2e-log.schema.json`; a source scan shows that the tool's modules (`src/e2e/` and the binary)
      import nothing outside the standard library.
- [ ] A script whose assertion fails exits 1 and its summary reports `asserts_failed ≥ 1` (negative control).
- [ ] A script that logs a private field, a path under the private root or the canary fails the linter (negative
      control); the same script without the leak passes.
- [ ] A script with zero assertions exits 2; a wholly skipped script exits 77 and is reported `not-run`.
- [ ] An unexpected refusal fails the script; `e2e_expect_refusal` fails on a success envelope and on a wrong code
      or exit code.
- [ ] Every line of every self-test log validates; the summary counts equal a recount; `body_sha256` matches.
- [ ] `run-all.sh` with an empty selection exits 2; `--verify-plants` reports a script whose plant does not make
      it fail.
- [ ] Two runs of one script with one seed produce logs that are equal after removing `ts`, `run_id`, `wall_ms`,
      capture paths and `body_sha256`.
- [ ] The gate report lists a gate with a passing value and no caught control as `no-control`.
- [ ] When the `frob` binary exists, the live stage of the self-test runs `frob doctor --json` and an unknown verb
      and logs the envelope's evidence block and the refusal (`FROB-REF-801`, exit 3); until then that stage is
      logged as a skip.

## Test plan

### Unit tests

`crates/frob-conformance/tests/e2e_tool.rs` and inline modules (no mocks; real files in a temporary directory):
- `json_roundtrip_escapes`: strings with `"`, `\`, newline, U+0001 and U+1F600 survive write then read.
- `json_duplicate_member_refused`: `{"a":1,"a":2}` is refused.
- `cmp_decimal_exact`: `0.1 + 0.2` is not used; `"0.30" == "0.3"` true; `"1e-3" < "0.0011"` true.
- `cmp_rational`: `"6/8"` equals `"3/4"`; `"27/32"` differs from `"27/33"`.
- `cmp_close_rel`: observed 1.0000004, expected 1, rel 5e-7 passes; rel 3e-7 fails with `first_diff`.
- `sha256_vectors`: the empty string and `"abc"` give the published digests.
- `emit_refuses_private_member`: a line with member `voxels` is refused; a 5,000-byte string is refused.
- `redact_private_path`: a path under the private root becomes `<private:…>` with 12 hex digits.
- `lint_seq_gap`, `lint_summary_recount`, `lint_missing_summary`, `lint_small_claims_gate`, `lint_no_control`:
  one planted defect each; each must be reported and named.
- `header_parse`: the header of a fixture script yields bead, milestone, labels, gates, seed, timeout, plants.
- `e2e_summary_counts_match`, `e2e_skip_is_not_pass`, `e2e_script_idempotent`,
  `expect_refusal_matches_code_and_exit`, `expect_refusal_fails_on_success_envelope`: driven through `lib.sh`.

### End-to-end script and logging

`scripts/e2e/selftest.sh` (header: `bead=STD-E2E milestone=M0 labels=gauntlet,harness seed=1`). It runs the fixture
scripts and asserts their outcomes: `st_pass` (exit 0); `st_fail_assert` (exit 1, one failed assertion with
`first_diff`); `st_private_leak` (script exit 0, linter fails); `st_no_assert` (exit 2); `st_skip_only` (exit
77); `st_unexpected_refusal` (exit 1); `st_plant` (exit 0 unplanted, exit 1 with `E2E_PLANT_FAILURE=boundary`);
`st_killed` (killed by signal 9: no summary, linter fails). It then runs `run-all.sh --label harness` and
`frob-e2e report`, and writes its own log at `target/e2e-logs/selftest/<utc>.ndjson` with every command, exit
code, wall time, each expected-versus-observed outcome and the final `summary`.

### Known answers and negative controls

Known answers: SHA-256 test vectors; the summary of `st_fail_assert` is exactly `asserts_passed 2, asserts_failed
1`. Negative controls (each must be caught): the failing assertion; the private field; the canary
`FROB-PRIVATE-CANARY-7f3a`; a hand-edited summary count; a removed line (sequence gap); a gate claimed in small
mode; a gate with no caught control. Determinism lane: the idempotence criterion above, twice on one machine and
on both operating systems.

## Considerations

- No blocking dependency is declared, on purpose: every other bead depends on this one, and an edge to any seed
  bead could close a cycle through that bead's own script. It lands in the first implementation session. If the
  crate `frob-conformance` (BD-025) does not exist yet, this bead creates only the binary target and its modules.
- `schemas/` is otherwise generated from the format and verb registries; this schema is a harness record, not a
  product format, and is generated by the tool. The generated-file check (BD-004) is told of that generator.
- The harness must stay small: the tool's cap is 2,500 lines inside the crate's cap of 24,000.
- Timings are host-dependent; a script never asserts on wall time unless its gate is a performance gate, in which
  case it logs the full disclosure block as metrics.
- Logs are evidence for a recorder, not a verdict: a pass in a log becomes a gate status only when a
  fresh-context session enters it with a caught control (GATE-GAUNT-01, GATE-GAUNT-10).

## References

Charter §6; plan §38 (preamble, §38.11), §32.4, §32.5, §37.6, §29.14, §34.6, §34.7; GATE-GAUNT-01, GATE-GAUNT-10;
ADR-GAUNT-01, ADR-GAUNT-03.
