---
key: "STD-E2E"
title: "scripts/e2e: end-to-end harness, structured NDJSON log, runner, linter, gate report and self-test"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "test", "e2e", "delivery"]
parent: "WP-14"
deps: ["BD-001"]
needs: []
related: ["STD-TEST", "BD-228", "N38a-02", "N38b-19", "N32ax-03", "N32a-14", "N34-24", "N34p3-01", "N37-02", "N32b-01", "BD-025"]
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

In: `scripts/e2e/lib.sh`; `scripts/e2e/run-all.sh`; the tool `frob-e2e` (emit, run, get, cmp, check, lint,
report, transcript, schema, list); the log schema `schemas/e2e-log.schema.json` (`frob.e2elog/1`); the directory
layout and the isolation of a run (its own state root); the rule for idempotency keys in scripts and the helper
that issues them; the meaning of a fixture reference `@fixture:<name>` and the helper that logs one; the helper
that grants a principal in the run's `safety.toml` and opens a root session; the helper that runs a Gauntlet tier
and logs its receipt; the registry of script checks and the test target `e2e_scripts`; the rule for a bead's own
helper library; redaction; the self-test with its negative controls; the `[[bin]]` entry of `frob-e2e` and its row
in `CONTRACT.md`. Out: unit-test conventions and the registries (STD-TEST); the per-bead delivery log
`frob.beadlog/1` (N29a-09) and the artifact graph (BD-095); recording a gate's value (the one recorder N38b-19,
through the `record_gate` example, from a receipt of a tier run, N38a-02); the resolution of `@fixture:` inside
`frob` (N34p3-01); the state root's contents (N32a-14), the safety configuration and its grant tables (N37-02),
minting and verifying tokens (N32b-01), and the scratch registries root (N32ax-03); the rest of the crate's
manifest and `CONTRACT.md` (BD-025); each capability's own script (its bead).

## Technical approach

**Files (DESIGN).**
```
scripts/e2e/lib.sh            sourced helper            scripts/e2e/run-all.sh     runner
scripts/e2e/<name>.sh         one script per capability scripts/e2e/selftest.sh    harness self-test
scripts/e2e/selftest/st_*.sh  fixture scripts of the self-test
scripts/e2e/lib/<name>.sh     a bead's own helper library (below)
crates/frob-conformance/src/bin/frob-e2e.rs, src/e2e/{json,line,lint,report,sha256}.rs   the tool (std only)
crates/frob-conformance/src/e2e/checks/{mod,<name>}.rs   script checks run by `frob-e2e check` (std only)
crates/frob-conformance/tests/e2e_tool.rs, tests/e2e_scripts.rs   the tool's tests; the checks' tests
crates/frob-conformance/Cargo.toml   the tool's `[[bin]]` entry; CONTRACT.md   its row in the binary table
schemas/e2e-log.schema.json   printed by `frob-e2e schema`; compared with a fresh print by the self-test
```
**The binary's declaration.** The crate's manifest sets `autobins = false` (BD-025), so a binary exists only by
declaration. This bead writes the entry `[[bin]] name = "frob-e2e", path = "src/bin/frob-e2e.rs"` and the
binary's row in `CONTRACT.md`'s binary table (owning bead STD-E2E; reads script headers, logs and the captured
output of the commands it runs; writes logs, transcripts, artifacts, the gate report and the schema print; writes
no registry). When the crate does not exist yet, this bead creates the manifest with `autobins = false`, no
`default-run` and that one entry, and `CONTRACT.md` with that one table; BD-025 completes both and its test
`binaries_declared` checks the entry and the row.
Scripts are bash, start with `#!/usr/bin/env bash`, use no bash feature newer than 3.2, and run on arm64 macOS
and x86-64 Linux. `lib.sh` builds the tool on first use with `cargo build -p frob-conformance --bin frob-e2e`
through the build-offload shim (cargo is never invoked by absolute path) and caches the path in `FROB_E2E_TOOL`.

**Script header.** Lines beginning `# e2e:` with `key=value` pairs, read by `frob-e2e list`:
`bead` (the owning bead key), `milestone` (M0…M8), `labels` (comma list), `gates` (comma list), `seed` (decimal
u64; 0 when absent, the default of STD-TEST's `FROB_TEST_SEED`), `timeout_s` (default 600; above 3600 needs `--approved-envelope <id>`, because no run may exceed one hour,
16 GiB or 8 cores without the owner's written approval), `needs` (`cpu`, `cuda`, `metal`, `frobd`, `mcp`,
`second-host`), `plants` (comma list of planted failures the script supports).

**Helper functions (`lib.sh`).** All are prefixed `e2e_`; each writes one or more log lines through the tool.
1. `e2e_init <script> <bead>`: resolves binaries (`FROB_BIN` default `frob` on PATH then `target/release/frob`;
   `FROBD_BIN`, `FROB_MCP_BIN`), creates the log and scratch directories (below), exports `FROB_STATE` as the
   run's own state root and makes `work/` the working directory, fixes the run identifier and the seed
   (`FROB_E2E_SEED`, alias `E2E_SEED`, default the header's), installs an EXIT trap that always writes the
   `summary`, and emits the script-level `start`.
2. `e2e_step <id> <title>`: closes the previous step (`end`) and opens a new one (`start` with `step` set).
3. `e2e_cmd [--name n] [--expect-exit 0,2] -- <command…>`: runs the command with standard output and error
   captured to the artifacts directory; emits `cmd`; sets `E2E_LAST_EXIT`, `E2E_LAST_STDOUT`; if the exit code is
   not expected, records a failed assertion `exit:<name>`. Default expected exit is 0. Exit 8 ("accepted and still
   running") is never treated as success unless listed. The command is run and logged exactly as written: the
   helper never adds, removes or rewrites an argument. An unexpected `FROB-REF-805` (a run or mutate verb called
   without an idempotency key) is logged as any unexpected refusal, and the transcript adds the line `hint: pass
   --key "$(e2e_key <label>)"`.
4. `e2e_get [--file <path>] <json-pointer>`: prints a field of the last envelope, or of the JSON file named
   (`E2E_LAST_RECEIPT`, for instance), by RFC 6901 pointer; a missing field fails.
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
11. `e2e_scene_ready <scene_id> [--tiers …]` (added by BD-228, M3, in the change that switches on the refusal
    `FROB-REF-318` for runs on packs without a complete acceptance report; listed here so the helper's function
    list stays the single record): when `frob capabilities` lists `scene.accept` it runs
    `frob --json scene accept <scene_id> --key e2e-ready-<run_id>-<scene_id>` (with the tiers given; `scene.accept`
    is a `run` verb, and one key per script run and pack makes a repeated call in one run the same idempotent
    request), emits a `cmd` line and logs the report digest and `complete` (a pack whose report stays incomplete
    then meets `FROB-REF-318` in the run, which fails the script as an unexpected refusal); before that verb exists
    it is a logged no-op, written as one `assert` with `status: "skip"` naming the missing verb, as `e2e_skip`
    writes it, never a silent pass. Every script that binds a compiled pack in a run verb calls it before the run;
    the same change adds the linter rule `scene-ready-before-run` (below). This bead reserves the name, the list
    entry and the rule's name; BD-228 writes and tests the function and the rule.
12. `e2e_key <label> [--again]`: prints the idempotency key `e2e-<script>-<label>` for a script to pass with
    `--key` (label: `[a-z0-9][a-z0-9-]*`). It contains no run identifier and no time, so two runs of a script send
    the same keys and their logs compare equal; keys never meet across runs because every run has its own state
    root. Issuing one label twice in a run is a harness error (exit 2) unless `--again` is given, which a script
    uses only when it replays a key on purpose (to check an idempotent return, or `FROB-REF-803` with changed
    content); two runs that a script compares therefore always carry distinct keys.
13. `e2e_fixture <name> [<file>] [--copy]`: prints the path that `@fixture:<name>` (below) denotes, or of the
    file `<file>` inside it, and emits an `assert` of kind `file` whose `observed` is the SHA-256 of the file (of
    the sorted list of `path digest` lines for a directory). A missing name or file is a harness error (exit 2),
    not a failed assertion. With `--copy` it copies the fixture into `work/fixtures/<name>/` and prints that path,
    for a script that must change it; a script never writes into the repository's `fixtures/`. The helper's root
    is `E2E_FIXTURES` (default: the repository's `fixtures/`; the self-test points it at
    `scripts/e2e/selftest/fixtures/`); a script that also passes the reference to `frob` keeps the default, so the
    path it logs is the path `frob` resolves.
14. `e2e_tier <target> <filter> [--mutations]`: runs `FROB_TEST_SEED=<seed> cargo test -p frob-conformance --test
    <target> [--features gauntlet-mutations] -- <filter>` through the build-offload shim, with the script's seed.
    `<target>` must be one of `g0_laws`, `g1_known_answers`, `g2_parity`, `g3_metamorphic`, `g4_chaos`,
    `g5_determinism` (anything else, `g1` included, is a harness error, exit 2). It emits the `cmd` line with the
    field `receipt`: the file name, which is the BLAKE3 hash of its bytes, of the receipt `frob.gauntlet.receipt/1`
    the run wrote under `target/gauntlet/receipts/` (found as the one name that appeared during the run; none or
    more than one is a failed assertion `receipt:<target>`), and sets `E2E_LAST_RECEIPT` to its path, so the script
    can assert on its `gates` and `controls` with `e2e_get --file`. It never records the value: the hash is what
    the gate's one recorder cites with `record_gate`.
15. `e2e_check <name> [--gate …] [--ka …] [--cx …] [--nc …] [--mr …] -- <args…>`: runs `frob-e2e check <name>
    <args…>` (a script check, below) and emits an `assert` of kind `check` with the check's `observed`,
    `expected` and `first_diff`.
16. `e2e_registries_scratch`: copies the repository's `registries/` into the run's `registries/`, exports
    `FROB_REGISTRIES_ROOT` at it, and marks the run: the summary then says `registries: "override"` and
    `gates_claimable: false`. It is the only way a script changes a registry (a script that plants a defective row,
    for instance). Every envelope then carries the warning `registries: override` and every receipt the kind
    `override`, which the recorder's `assert_of_record` rejects (N32ax-03), so such a run can never be cited for a
    gate.
17. `e2e_session`: the grant and the root session a script that drives `frobd` needs (a root token is minted only
    from a principal's grant, N32b-01, and from M2 a call without a token is `FROB-REF-723`). Three forms.
    `e2e_session grant <principal> --scopes <list> [--namespace <ns>] [--trace-bytes <n>] [--max-lifetime-s <n>]`
    writes the table `[principal.<principal>]` of N37-02 into the run's `state/safety.toml` (`$FROB_STATE`;
    created with mode 0600, a regular file owned by the user, as N37-02 requires), with `scopes` the list as
    given (catalogue scopes and budget scopes in the grant's form, such as `cpu:core_seconds=600`) and the other
    keys when given; it is called before the script starts `frobd`, because the daemon reads the tables once at
    start, so a call while the run's daemon socket is live, or for a principal whose table exists, is a harness
    error (exit 2). The helper checks no scope name (it reads no registry); `frobd`'s start-up check does.
    `e2e_session open <label> --principal <id> [--scopes <list>] [--budget <map>] [--namespace <ns>]` runs
    `frob --json session open --principal <id> [--scopes …] [--budget …] [--namespace …] --key "$(e2e_key
    <label>)"` with `FROB_SESSION_TOKEN` removed from that command's environment (a root call presenting a token
    is `FROB-REF-723`), through `frob-e2e run --secret /result/token`: the tool writes the token to
    `state/sessions/<label>.token` (mode 0600) and replaces it, in the captured output, the `cmd` line and the
    transcript, with `<token:` + the first 12 hex digits of its SHA-256 + `>`; the helper then exports
    `FROB_SESSION_TOKEN` from that file. `e2e_session use <label>` exports the token of a session the run opened
    (a script that drives two agents switches with it; an unknown label is a harness error, exit 2). An envelope
    without `/result/token` (a build before tokens, M2) leaves the variable unset and is logged as `e2e_skip`
    writes it, naming the missing token; when `frob capabilities` does not list `session.open` the open is a
    logged skip naming the missing verb, never a silent pass. A script writes a grant or a token file only
    through this helper; the exceptions are the scripts of the beads that own the file and the minting (N37-02,
    N32b-01), which plant malformed tables on purpose.

The `e2e_` prefix is reserved for this list. **A bead's own helper library** (as N00-29's `e2e_demo_run` in
`scripts/e2e/lib/demo.sh`, and NJL-02's `scripts/e2e/verbs_lib.sh`, kept where its 44 users name it) is sourced
after `lib.sh`, is named in its owning bead's scope, writes log lines only through the functions above, and
prefixes its functions with its own name (`verbs_`) or with `e2e_<name>_` (`e2e_demo_`); a new library goes in
`scripts/e2e/lib/<name>.sh`. A function a library needs in every script is requested here instead.

**Idempotency keys (DESIGN, the rule of the program's conventions: run verbs take a key).** Every call of a `run`
or `mutate` verb in a script passes `--key`, written with `e2e_key`; `frob` is never wrapped to supply one.
Reason: the key is part of the request a user types and of what the admission transaction digests (N32a-09:
reusing a key with different resolved content is `FROB-REF-803`); a wrapper that inserted keys would make the log
show a command no user can reproduce, would need the verb registry to know which verbs take one (the tool reads
no registry), and would hide the replays a script makes on purpose. Rejected: a key derived by the helper from
script, step and seed (it would collide when one step runs two runs of one verb, and silently return the first
one's stored receipt). The exception is a verb whose row says `key_optional` (`session.focus`, `study.plan`).

**Fixture references (DESIGN).** In a script, `@fixture:<name>` (segments `[a-z0-9][a-z0-9._-]*` joined by `/`,
with an optional trailing `/`; no empty, `.` or `..` segment; at most 255 bytes) denotes the repository's
`fixtures/<name>`: the file of that path, or, for a directory, the
one file in it whose format is the one the argument takes (for a list-valued argument such as `--designs`, every
such file, sorted by name), and the directory itself for an argument that takes one (`@fixture:cortex/`). Scripts
pass the reference verbatim, as a user would, from M0; `frob` resolves it with N34p3-01's resolver (M0) for any
argument of the parameter kinds `artifact-input<F>`, `artifact-identity<F>` and `foreign-file` and echoes the
resolution in the envelope's `bound.fixtures` (`param`, `reference`, `path` relative to the workspace, `digest`;
ADR-TETHER-11), which the `cmd` line logs. That digest is BLAKE3, so a script never compares it with the SHA-256
that `e2e_fixture` logs; it compares `bound.fixtures` paths, or BLAKE3 with BLAKE3. A malformed, unknown,
ambiguous or empty reference, and a missing fixtures root, are refused before the request digest with
N34p3-01's refusal `fixture-reference-unresolved` (reported as FROB-REF-802, exit 3, `observed` one of
`fixture-name-invalid`, `fixtures-root-absent`, `fixture-unknown:<name>`, `fixture-outside-root:<name>`,
`fixture-ambiguous:<name>:<count>`, `fixture-empty:<name>`), which a script expects with `e2e_expect_refusal
FROB-REF-802 3`. The fixtures directory is the
workspace's, whose path the build records as it records `registries/` (N32ax-03), and it is read only. Reason: 57
beads already write the form, and `frob`'s own resolution makes a script's command the command a user copies; the
helper `e2e_fixture` exists for the path and digest a script needs itself. The namespace is distinct from the
arena fixture names `fixture:<name>` of N24b-03 (no `@`, resolved in the arena registry), and from `@corpus:`,
`@archive:` and `@workload:`, which their own beads resolve.

**Script checks.** A check a shell script cannot make (ranks in descending order, every numeric leaf of an
envelope, a seeded variant generator) is a function in `crates/frob-conformance/src/e2e/checks/<name>.rs`,
registered by name in `checks/mod.rs`, using the tool's JSON reader and the standard library only, at most 300
lines each. `frob-e2e check <name> <args…>` runs it and prints one JSON object `{ok, observed, expected,
first_diff}`, exit 0 or 1. Its tests are in the test target `e2e_scripts` (`cargo test -p frob-conformance --test
e2e_scripts`), which this bead creates with the built-in check `single-envelope` (standard output holds exactly
one JSON object); the bead that adds a check adds its tests there, named `<check>_…`.

**Exit codes of a script.** 0: at least one assertion ran, none failed, no control was missed. 1: an assertion
failed, a control was missed, or an unexpected refusal occurred. 2: harness error (tool missing, log not
writable, bad usage, zero assertions: a script that asserts nothing is not a pass). 77: skipped as a whole (a
needed device or binary is absent); a skip is never counted as a pass.

**Isolation, idempotence and seeds.** A script works only inside `target/e2e-scratch/<script>/<run_id>/`:
`state/` (mode 0700) is the state root `FROB_STATE` that `e2e_init` exports (the ledger, the store, the keys and
the daemon's socket live there, N32a-14), so a run never meets the user's state of record nor another run's;
`work/` is the working directory and the place for files the script writes; `registries/` exists only after
`e2e_registries_scratch`. `frob` has no `--root` option and reads no `FROB_HOME`: a script that needs its own
state uses this root, and nothing else. Registries of record are read in place and never edited; a script
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
target/e2e-scratch/<script>/<run_id>/state/      FROB_STATE of the run (mode 0700)
target/e2e-scratch/<script>/<run_id>/work/       working directory; work/fixtures/ for copied fixtures
target/e2e-scratch/<script>/<run_id>/registries/ only after e2e_registries_scratch
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
| `argv`, `cmdline`, `cwd`, `exit`, `expected_exit`, `wall_ms`, `stdout_path`, `stderr_path`, `stdout_sha256`, `receipt` | `cmd` | The exact command (array and one shell-quoted string), directory relative to the scratch root, codes, wall time, capture paths and digest; `receipt` (from `e2e_tier` only) the hash of the tier run's receipt |
| `envelope{schema,verb,ok,exit}`, `evidence{claims,class,tier,determinism,identities,seed,reality}`, `cost`, `job` | `cmd` when standard output is an envelope | The evidence block copied verbatim; `identities` is the envelope's `bound` block plus `model_id` |
| `name`, `kind`, `status`, `observed`, `expected`, `op`, `tolerance`, `unit`, `gate`, `ka`, `cx`, `nc`, `mr`, `caught` | `assert` | `kind` in `eq`, `num`, `close`, `rational`, `digest`, `json`, `file`, `exit`, `refusal`, `negative-control`, `determinism`, `check`; `status` in `pass`, `fail`, `skip` |
| `first_diff{path,observed,expected}`, `repro{cmd_seq,artifacts}` | failed `assert` | The first differing value and what is needed to reproduce: the `seq` of the command and artifact paths |
| `name`, `value`, `unit`, `threshold{op,value}`, `interval`, `n`, `gate` | `metric` | `value` is a JSON number, or a string for exact rationals and integers above 2⁵³ |
| `code`, `exit`, `message`, `pointer`, `observed`, `required`, `unit`, `retry`, `repairs`, `expected` | `refusal` | Copied from the envelope's `refusal`; `repairs` in rank order |
| `wall_ms`, `asserts_passed`, `asserts_failed`, `exit` | `end` | Per step; `exit` only at script level |
| `asserts_passed`, `asserts_failed`, `asserts_skipped`, `controls_caught`, `controls_missed`, `refusals_expected`, `refusals_unexpected`, `gates[{id,measured_ok,controls_caught}]`, `gates_claimable`, `registries`, `receipts`, `exit`, `wall_ms`, `body_sha256` | `summary` (last line) | Counts, gates touched, `registries` (`of-record`, or `override` after `e2e_registries_scratch`), the receipt hashes of the run's tier runs, and the SHA-256 of every byte before this line |

**Redaction.** (1) When an envelope carries the mark `private` (the mark is the join over the lineage of its
inputs), the `cmd` line keeps only verb, codes, wall time, identities and the evidence block; captured output is
written under the private root, not under `target/`, and only its digest is logged. (2) An argument or path that
resolves beneath `FROB_PRIVATE_ROOT` is logged as `<private:` + the first 12 hex digits of its SHA-256 + `>`.
(3) The writer refuses, at any depth, the member names `voxels`, `occupancy`, `points`, `mesh`, `image`, `pixels`,
`depth_map`, `rgb`, `scan`, `floorplan`, `pose_trace`, and any string above 4,096 bytes. (4) The linter repeats
(2) and (3) on finished logs and searches them for the canary values planted by the self-test. (5) A session token
(`e2e_session open`) reaches no log line, transcript or capture: only `state/sessions/<label>.token` holds it,
and everything else carries `<token:` + 12 hex digits + `>`; the linter searches finished logs and captures for
the token canary planted by the self-test.

**Linter (`frob-e2e lint <log>…`).** Fails when: a line is not valid against the schema; `seq` is not contiguous;
the first line is not a script `start` or the last is not a `summary`; the summary's counts differ from a recount
of the `assert` and `refusal` lines; `body_sha256` does not match; a `cmd` line lacks its exit code or wall time;
a failed assertion lacks `first_diff` or `repro`; a `metric` tagged with a gate has no threshold; the log claims
a gate with `scale: "small"` or with `registries: "override"` (`scratch-registries`); a gate is touched and
`controls_caught` for it is zero while the summary says `measured_ok: true` (reported as `no-control`, never as
pass); a `cmd` line runs the `record_gate` example or writes `registries/gates.toml` (`self-recording`: a script
produces a receipt and never records a value; the one exemption is the recorder's own tests, the scripts whose
header `bead` is N38b-19 or N38b-21, which drive the recorder from the user's side); a `cmd` line of `e2e_tier`
lacks its `receipt`; a redaction rule is broken. From BD-228 (M3), `scene-ready-before-run`: a `cmd` line runs one of the five run verbs on a compiled
pack with no earlier `e2e_scene_ready` line for that pack.

**Runner (`run-all.sh`).** `--milestone M1` (exact), `--through M2` (cumulative), `--label a,b` (all must match),
`--gate`, `--bead`, `--script`, `--list`, `--small`, `--devices`, `--fail-fast`, `--verify-plants`. It runs the
selected scripts one at a time under their timeouts (the process group is killed on expiry and when the runner is
interrupted), lints each log, writes the run index and prints one line per script. Exit 0 only if every selected
script passed and linted; 1 if any failed; 2 on harness error or when the selection is empty. `--verify-plants`
runs each script once per declared plant with `E2E_PLANT_FAILURE=<plant>` and requires exit 1 with a failed
assertion that names the plant.

**Gate report (`frob-e2e report`).** Reads the headers and the newest log of each script; writes one row per gate
named in a header or a log: milestone (the header's), scripts that touch it, last run, result (`pass`, `fail`,
`not-run`, `small-only`, `no-control`), measured values with thresholds, controls caught. It reads no registry:
the tool has a JSON reader only, and `registries/gates.toml` has one loader (N40-01); listing the register's gates
that no script touches, and joining these rows with recorded statuses, is the exit report's (N40-04), which reads
the register through that loader. It is a view, and it lists the receipt hashes each gate's scripts produced so
that the recorder can find them. It never writes `registries/gates.toml`: a value is entered only by the gate's
one recorder (N38b-19), in a session that did not implement the capability, from a receipt with `record_gate`.

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
- [ ] Two runs of one script with one seed produce logs that are equal after removing `ts`, `wall_ms` and
      `body_sha256` and replacing every occurrence of the run identifier (in `run_id`, `argv`, `cmdline`, capture
      paths and `cwd`) with one placeholder.
- [ ] `e2e_init` exports `FROB_STATE` under the run's scratch directory with mode 0700 and the working directory
      is `work/`; two concurrent runs of one script share no file.
- [ ] `e2e_key` gives the same key for one label on two runs, and a second issue of one label in a run without
      `--again` exits 2; `e2e_cmd` logs `argv` exactly as given.
- [ ] `e2e_fixture` logs the path and SHA-256 of an existing fixture, exits 2 on a missing one, and with `--copy`
      leaves the repository's `fixtures/` byte-identical.
- [ ] `e2e_session grant` writes the `[principal.<id>]` table into the run's `state/safety.toml` (mode 0600) and
      exits 2 for a principal already granted and while the run's daemon socket is live; `frob-e2e run --secret
      /result/token` on an envelope carrying a token leaves the token only in `state/sessions/<label>.token`
      (mode 0600) and logs `<token:…>` in its place; `e2e_session use` of an unknown label exits 2.
- [ ] The manifest of `frob-conformance` declares `frob-e2e` by a `[[bin]]` entry with explicit name and path
      under `autobins = false`, `cargo build -p frob-conformance --bin frob-e2e` builds it, and `CONTRACT.md`'s
      binary table has its row.
- [ ] `e2e_tier g1 x` exits 2 and `e2e_tier g1_known_answers x` builds the documented command line (dry run).
      That a real tier run's receipt hash reaches `receipt` and the summary's `receipts` is checked by N38a-02's
      script, which runs its tier targets through this helper (the tiers do not exist before it).
- [ ] A log with a `record_gate` command fails the linter as `self-recording`; a log with `registries: "override"`
      that claims a gate fails as `scratch-registries`.
- [ ] `frob-e2e check single-envelope` passes on a file holding one JSON object and fails, with `first_diff`, on
      one holding two; `cargo test -p frob-conformance --test e2e_scripts` runs.
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
- `lint_self_recording` (a `cmd` line running `cargo run -p frob-conformance --example record_gate` fails; the
  same line in a log whose `bead` is N38b-19 passes),
  `lint_scratch_registries` (a gate claimed under `registries: "override"`), `lint_tier_without_receipt`: one
  planted log each, reported by name.
- `header_parse`: the header of a fixture script yields bead, milestone, labels, gates, seed, timeout, plants;
  `header_seed_default_zero`: a header without `seed` yields 0.
- `check_single_envelope` (in `e2e_scripts`): one object passes; two objects, and one object followed by text,
  fail with `first_diff` naming the byte offset; `check_registry_names_unique`: two checks under one name fail to
  register.
- Driven through `lib.sh` on fixture scripts: `e2e_summary_counts_match`, `e2e_skip_is_not_pass`,
  `e2e_script_idempotent` (the run-identifier substitution of the acceptance criterion),
  `expect_refusal_matches_code_and_exit`, `expect_refusal_fails_on_success_envelope`; `state_root_isolated`
  (`FROB_STATE` under the scratch directory, mode 0700, cwd `work/`); `key_label_deterministic` (one label gives
  `e2e-st_key-a` on two runs); `key_label_reuse_refused` (exit 2 without `--again`, the same key with it);
  `cmd_argv_verbatim` (a command with spaces, quotes and an empty argument is logged as given, nothing added);
  `refusal_805_hint` (a fixture envelope refusing 805 puts the `--key` hint in the transcript);
  `fixture_resolves_and_digests` (a file and a directory under a temporary fixtures root; directory digest over
  sorted lines); `fixture_missing_is_harness_error`; `fixture_copy_leaves_source` (source bytes unchanged);
  `tier_target_refused` (`g1`, `g7`, `e2e_scripts` exit 2; the six names accepted in dry run);
  `registries_scratch_marks_run` (summary `registries: "override"`, `gates_claimable: false`);
  `session_grant_writes_table` (`[principal.agent-a]` with the scopes as given, mode 0600; a second grant of
  `agent-a` exits 2; a grant while a socket listens at the run's daemon socket path exits 2);
  `session_token_redacted` (`frob-e2e run --secret /result/token` on a fixture envelope: the capture, the `cmd`
  line and the transcript hold `<token:` + 12 hex digits + `>`, the token file holds the value with mode 0600);
  `session_use_unknown_label` (exit 2).

### End-to-end script and logging

`scripts/e2e/selftest.sh` (header: `bead=STD-E2E milestone=M0 labels=gauntlet,harness seed=1`). It runs the fixture
scripts and asserts their outcomes: `st_pass` (exit 0); `st_fail_assert` (exit 1, one failed assertion with
`first_diff`); `st_private_leak` (script exit 0, linter fails); `st_no_assert` (exit 2); `st_skip_only` (exit
77); `st_unexpected_refusal` (exit 1); `st_plant` (exit 0 unplanted, exit 1 with `E2E_PLANT_FAILURE=boundary`);
`st_killed` (killed by signal 9: no summary, linter fails); `st_self_record` (logs a `record_gate` command: linter
fails as `self-recording`); `st_key_reuse` (issues one key label twice: exit 2); `st_fixture` (resolves a
fixture of `scripts/e2e/selftest/fixtures/`, logs its digest, then a missing one: exit 2); `st_token_leak`
(prints the token canary `FROB-TOKEN-CANARY-5c1e` into a log line: script exit 0, linter fails). It then runs
`run-all.sh --label harness` and
`frob-e2e report`, and writes its own log at `target/e2e-logs/selftest/<utc>.ndjson` with every command, exit
code, wall time, each expected-versus-observed outcome and the final `summary`.

### Known answers and negative controls

Known answers: SHA-256 test vectors; the summary of `st_fail_assert` is exactly `asserts_passed 2, asserts_failed
1`. Negative controls (each must be caught): the failing assertion; the private field; the canary
`FROB-PRIVATE-CANARY-7f3a`; a hand-edited summary count; a removed line (sequence gap); a gate claimed in small
mode; a gate claimed under scratch registries; a gate with no caught control; a script that records a gate
itself; a reused key label; a missing fixture; the token canary in a log. Determinism lane: the idempotence criterion above, twice on one
machine and on both operating systems.

## Considerations

- The only blocking dependency is the workspace skeleton (BD-001): `frob-e2e` is built by Cargo inside that
  workspace, and BD-001 has no dependency of its own. Nothing else is declared, on purpose: every other bead
  depends on this one, and an edge to any other seed bead could close a cycle through that bead's own script.
  It lands in the first implementation session. If the crate `frob-conformance` (BD-025) does not exist yet,
  this bead creates only the binary target and its modules.
- `schemas/` is otherwise generated from the format and verb registries; this schema is a harness record, not a
  product format, and is generated by the tool. The generated-file check (BD-004) is told of that generator.
- The harness must stay small: the tool's cap is 2,500 lines inside the crate's cap of 24,000; script checks are
  outside the tool's cap (at most 300 lines each) and inside the crate's.
- Before the admission transaction exists (N32a-09, M1) a run or mutate call without `--key` is not refused, so
  the defect is silent until then; from M1 it is an unexpected `FROB-REF-805`, which fails the script with the
  hint above. Each bead's script carries its own keys (the program's conventions, item 5).
- No bead defines a `frob --root <dir>` option or a `FROB_HOME` variable, and `frob` reads neither: a script's
  state is the run's `FROB_STATE`, and a script that needs an empty store gets one from `e2e_init`.
- Variables the tool reads (`E2E_*`, `FROB_E2E_*`, `FROB_BIN` and the other binary paths) are read by `lib.sh` and
  `frob-e2e`, never by `frob`; the variables `frob` reads are the reviewed list of its environment modules
  (N37-02), which includes `FROB_STATE`.
- Timings are host-dependent; a script never asserts on wall time unless its gate is a performance gate, in which
  case it logs the full disclosure block as metrics.
- Logs are evidence for a recorder, not a verdict: a pass in a log becomes a gate status only when a
  fresh-context session enters it with a caught control (GATE-GAUNT-01, GATE-GAUNT-10).

## References

Charter §6; plan §38 (preamble, §38.11), §32.4, §32.5, §37.6, §29.14, §34.6, §34.7; GATE-GAUNT-01, GATE-GAUNT-10;
ADR-GAUNT-01, ADR-GAUNT-03; ADR-TETHER-11 (names resolved once and echoed in `bound`); the program's conventions
(running the Gauntlet, run verbs take a key, one recorder per gate).
