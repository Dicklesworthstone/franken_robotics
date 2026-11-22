---
key: "STD-TEST"
title: "unit-test conventions, the test kit, registry loaders, determinism lanes and the no-control-no-pass rule"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "frob-core", "solid", "test", "registry", "delivery"]
parent: "WP-14"
deps: []
needs: ["§38.11: the record format of a negative control in `crates/frob-conformance/registry/controls.toml` and the seeding of that file from the counterexample catalogue (drafted with BD-025); this bead reads the file and restates the fields it reads", "§38.13: the recorder that writes a status into `registries/gates.toml` and refuses `pass` without a caught control; this bead supplies the evidence lines it reads"]
related: ["STD-E2E", "BD-005", "BD-025", "N38a-12", "N38a-14", "N38a-35"]
gates: ["GATE-GAUNT-01", "GATE-GAUNT-07"]
plan_refs: ["§38.2", "§38.6", "§38.7", "§38.11", "§35.1", "§35.3", "§35.9", "§34.3", "§34.10", "§0.2", "§0.7"]
---
## Background

Every bead of FrankenRobotics (a robotics simulation and design system in Rust) carries unit tests, and the program's
verification subsystem, the Gauntlet (crate `frob-conformance`), decides whether a gate may record a pass. This
bead fixes what all those unit tests have in common, so that a test written in one crate can be counted, joined to
the registries and trusted by a reviewer who has never seen that crate.

The conventions exist because sibling projects recorded how agent swarms produce the appearance of progress: dozens
of work items closed within seconds on one shared reason, serving figures from a stand-in. The response is
mechanical. A test runs the real component. A fixture that is not the real thing says `synthetic`. A known answer
is a numbered row with a derivation. Every gate has a planted failure it must catch, and a gate whose control was
not caught in the same run cannot print as passed (the program's rule: "a gate that no deficient model can fail
validates nothing").

DESIGN decisions and rejected alternatives.
- The shared test code is a module of `frob-core`, `frob_core::testkit`, behind the feature `test-support` (off by
  default, enabled only in dev-dependencies). Reason: every crate already depends on `frob-core`; the crate list is
  closed (a new crate needs a distinct dependency, device target or proof boundary); and `frob-conformance`
  depends on everything, so nothing may depend on it. Rejected: a new `frob-testkit` crate (violates the closed
  list); copying helpers per crate (they would drift).
- No third-party test library (no property-testing crate, no mocking crate). Reason: core dependencies are the
  standard library and the suite's own foundations, and the generator must follow the program's rule that a
  random draw is a function of an address, never of a sequence.
- Tests are bound to registry rows by name, not by a procedural macro. Reason: a procedural macro needs its own
  crate; a name is checkable from `cargo test -- --list`.

Terms. *Known answer* (KA-nnn): an analytic or synthetic truth with tolerance, tier and negative control.
*Counterexample* (CX-nnn): the smallest case that breaks a tempting shortcut. *Negative control* (NC-nnnn): a
registered planted defect attached to gates. *Metamorphic relation* (MR-nn): a relation that holds without knowing
the answer. *Determinism class*: D0 exact; D1 bitwise across backends; D2 bitwise per backend and independent of
batch composition; D3 inside a tolerance, with agreement of task decisions; D4 unspecified, never evidence.

## Scope

In: the written conventions; `frob_core::testkit`; loaders for the known-answer, negative-control and
metamorphic-relation registries; the determinism lanes; the evidence stream of unit tests; the lint that enforces
the conventions and the rule on gates. Out: the end-to-end harness (STD-E2E); the content and schema ownership of
each registry (N38a-12 known answers, N38a-14 relations, BD-025 controls); the tier runners (N38a-02); writing
`registries/gates.toml`.

## Technical approach

**1. Where tests live.** Inline `#[cfg(test)] mod tests` for private logic; `crates/<crate>/tests/*.rs` for the
public surface; `crates/<crate>/tests/conformance.rs` for the crate's contract. All run with
`cargo test -p <crate>` through the build-offload shim. A test finishes inside the default machine envelope
(16 GiB, 8 cores, one hour); a longer one is a tier test of `frob-conformance` with its budget named.

**2. Names.** `snake_case`, subject first: `<subject>_<behaviour>[_<condition>]`. Bound tests use fixed prefixes:

| Prefix | Bound to | Example |
|---|---|---|
| `ka_<nnn>_` | Known answer KA-nnn | `ka_501_docking_grid_344_217_9` |
| `cx_<nnn>_` | Counterexample CX-nnn, shown to break the shortcut | `cx_262_euler_order_spectral_radius` |
| `nc_<nnnn>_` | Negative control NC-nnnn, shown caught | `nc_0412_wheel_loop_mutation_caught` |
| `mr_<nn>_` | Metamorphic relation MR-nn | `mr_06_similarity_rescaling_bits` |
| `inv_<area>_<nn>_`, `po_<area>_<nn>_` | An invariant or proof obligation | `inv_kin_04_batch_independence` |
| `law_` | An algebraic law (tier G0) | `law_se3_adjoint_homomorphism` |
| `det_<lane>_` | A determinism lane | `det_twice_referee_digest` |
| `refusal_<code>_` | A refusal code | `refusal_532_unreachable_verdict` |

**3. No mocks and no fakes.** A test calls the real function, crate or binary. Forbidden: mocking crates
(`mockall`, `mockito`, `wiremock`, `faux`, `double`), a trait implementation written only to stand in for a
collaborator under test, and a test that asserts on a stub's own canned answer. Sanctioned stand-ins are named and
labelled: a `synthetic` fixture (data constructed to have a known answer); the simulated device of the daemon
tests (outcomes a pure function of the world address, kept honest by dual runs against the CPU tier, labelled
`simulated`); a development oracle (labelled `development-oracle`). A fixture file begins with a header naming its
kind: `synthetic`, `oracle` (with pin, date and capture script) or `recorded`.

**4. Property tests.** `testkit::prop::check(name, cases, |g| …)`.
1. Base seed = first 8 bytes of BLAKE3(crate name ‖ 0x00 ‖ test name), XOR the value of `FROB_TEST_SEED` (default 0).
2. Draw `k` of stream `s` in case `c` is `mix(seed, c, s, k)`, a counter-based 64-bit hash (SplitMix64 finalizer
   applied to the four words combined by multiply-xorshift): adding a stream changes no other stream, and cases
   can run in any order.
3. Default 256 cases; `FROB_PROP_CASES` scales it; a gate that states a count (10⁴, 10⁶, 10⁹) is a tier test that
   passes the count explicitly.
4. On failure, shrink deterministically, at most 1,000 steps: integers halve toward 0; floats try 0, ±1, then
   halve the exponent distance; vectors drop halves, then single elements, then shrink elements; tuples shrink
   left to right. The report prints the seed, the case index, the original and the shrunk input, and a replay line
   `FROB_TEST_SEED=<s> FROB_PROP_CASE=<c> cargo test -p <crate> <name>`.
5. A found failure is appended as `(seed, case)` to `crates/<crate>/tests/regressions/<name>.txt`, committed, and
   replayed first on every run.
6. Every law, invariant and proof obligation has a property test; boundary sets (identity, zero, signed zero,
   largest finite, values straddling a comparison) are enumerated before random cases.

**5. Registry loaders.** The registries are data files of the conformance crate:
`crates/frob-conformance/registry/known_answers.toml`, `controls.toml`, `relations.toml` (and `pilots.toml`,
`comparators.toml`, used by other beads). The names `registries/known_answers.toml` and
`registries/negative_controls.toml` used in some documents are logical aliases that the loader resolves to those
files; a second physical copy is a lint failure. `testkit::registry::root()` finds the workspace root by walking
up to the directory holding `franken-stack.lock`.
- `KnownAnswers::get("KA-501") -> Row { id, section, case, truth, tolerance, tier, site, control, derivation,
  fixture, due }`. `Tolerance` is `Exact | Abs(q) | Rel(q) | PrintedDigits(n) | Gate(id) | ToSet`.
  `testkit::assert_ka(&row, observed)` compares under the row's tolerance; under `ToSet`, or when `control` is
  `none stated`, it runs the comparison and reports `obligation`, never `pass`.
- `Controls::for_gate("GATE-ARENA-04") -> Vec<Control { id, seed, gate, kind, target, defect, expect }>`; `kind`
  is `mutation | planted-input | planted-design | planted-world | a-a`.
  `testkit::control(id, || -> Caught | Missed)` runs the planted check and records the outcome.
- `Relations::get("MR-06") -> Relation { id, statement, source, tiers, mutation, equality }`.
- Identifiers are append-only: a loader error is raised on a reused or renumbered KA, CX, NC or MR number.

**6. Determinism lanes.** `testkit::det::lane(lane, class, run) -> LaneResult`, where `run` returns the digests of
declared state at checkpoints. Five lanes, each judged against the declared class of the thing under test:

| Lane | What varies | D0, D1, D2 | D3 | D4 |
|---|---|---|---|---|
| `twice` | Nothing: two runs on one machine | Identical bytes | Inside the envelope, decisions agree | Not asserted, labelled |
| `workers` | Worker count 1, 2, 8; shard count on devices | Identical bytes | As above | Not asserted |
| `profiles` | Debug and release builds | Identical bytes (no fused multiply-add, no fast-math) | As above | Not asserted |
| `platforms` | arm64 macOS and x86-64 Linux | D0 identical; referee `f64` identical only where the upstream cross-ISA work item has landed, otherwise judged as D3 | As above | Not asserted |
| `backends` | `batch-cpu` in `f32`, CUDA, Metal | D0 identical; D1 identical where the IEEE probe gate has passed for the device family; D2 is not a cross-backend claim | Envelope plus the decision-level test | Not asserted |

A mismatch prints the first differing checkpoint, step and quantity. A result never claims a class higher than the
weakest of its inputs, and a class is never raised by relabelling, only by running again under a stricter profile.
The decision-level test under D3 (thresholds are TARGETs): the one-sided 95% exact binomial upper bound on the
rate of discordant task decisions is at most 1% (so at least 300 worlds are needed), and on the net rate at most
0.5%.

**7. Evidence stream.** When `FROB_TEST_EVIDENCE_DIR` is set, each testkit call appends one line
(`frob.testevidence/1`) to `<dir>/<crate>.ndjson`: `ts, crate, test, kind (ka|cx|nc|mr|inv|po|det|law|gate), id,
status (pass|fail|obligation|caught|missed|skip), observed, expected, tolerance, unit, tier, site, seed,
determinism, synthetic`. The tier runners and the recorder read these lines; a test never writes a gate status.

**8. No caught control, no pass.** `testkit::GateReport::new(gate)` collects `.measured(name, value, unit,
threshold)` and `.control(id, outcome)` and ends with `.finish() -> GateLine`. `finish` is the only function in
the workspace that can produce the text `pass` for a gate. It returns `Fail` if any measurement misses its
threshold or any control is `Missed`, `NoControl` if no control ran in this report, and `Pass` otherwise.
`NoControl` prints as `not-eligible: no control caught in this run`. `GateLine` has no public constructor.

**9. The enforcing tool.** `crates/frob-conformance/tests/std_test_lint.rs`, run as
`cargo test -p frob-conformance --test std_test_lint`. It fails when: any manifest names a forbidden mocking
crate; a test is `#[ignore]` without a reason string; a `ka_`, `cx_`, `nc_` or `mr_` test names an identifier
absent from its registry; a registry row due at or before the current milestone has no bound test and no runner
entry; a fixture file lacks its kind header; a gate identifier is printed with `pass` outside `GateReport`; two
physical copies of a registry exist; an evidence line carries `kind: gate, status: pass` without a `caught` line
for the same gate and run.

## Acceptance criteria

- [ ] `docs/testing/CONVENTIONS.md` states sections 1 to 8 and is cited from every crate's `CONTRACT.md` under
      "oracles".
- [ ] `frob_core::testkit` builds only under `test-support`; `cargo tree -e normal` of any binary shows the
      feature off; the module uses the standard library and `frob-core` only.
- [ ] `prop::check` is reproducible: the same seed and case give the same input on both machines; a planted
      failing property is found, shrunk to its minimal input and written to the regressions file.
- [ ] The three loaders read the registries; a duplicate or renumbered identifier is a load error.
- [ ] `assert_ka` on a row with tolerance `to set` or control `none stated` reports `obligation`.
- [ ] `GateReport` with no control prints `not-eligible`; with a missed control prints `fail` although every
      measurement passed; with a caught control and passing measurements prints `pass`.
- [ ] The lint catches each of its eight planted violations and passes on the clean tree.
- [ ] Each determinism lane has a passing example and a caught negative control (below).

## Test plan

### Unit tests

In `crates/frob-core/tests/testkit.rs` (feature `test-support`), real files, no mocks:
- `prop_draw_is_address_function`: draw (seed 7, case 3, stream 1, index 0) is unchanged when stream 2 is added.
- `prop_shrinks_to_minimal`: the property "x < 1000" on u64 fails and shrinks to exactly 1000.
- `prop_vector_shrink`: "no element equals 7" on vectors shrinks to the single-element vector `[7]`.
- `prop_replay_line_reproduces`: running with the printed seed and case reproduces the failure first.
- `ka_tolerance_kinds`: `Exact` rejects 344 against 345; `Abs(0.1)` accepts 33.49 against 33.52 and rejects
  34.08; `Rel(0.02)` accepts 0.98 against 1; `PrintedDigits` accepts 0.9207 against 0.92074.
- `ka_obligation_states`: `ToSet` and `none stated` yield `obligation`.
- `registry_append_only`: a file with KA-010 twice, or with KA-012 before KA-011, is refused.
- `gate_report_no_control`, `gate_report_missed_control`, `gate_report_pass`: the three outcomes of section 8.
- `gate_line_has_no_constructor`: a compile-fail test builds `GateLine` by hand and must not compile.
- `det_twice_catches_time_seeded_value`: a function that mixes the wall clock into a digest fails lane `twice`.
- `det_workers_catches_arrival_order_sum`: a sum taken in arrival order fails lane `workers` for 1 against 8.
- `det_backends_d3_decision`: states 1.0000001 and 0.9999999 around a threshold of 1 are inside the envelope
  and are reported as a discordant decision.

### End-to-end script and logging

`scripts/e2e/std_test_lint.sh` (header: `bead=STD-TEST milestone=M0 labels=gauntlet,harness
gates=GATE-GAUNT-01`). In a scratch copy of the workspace it plants, one at a time, the eight violations of
section 9, runs the lint, and uses `e2e_expect_caught` for each; then it runs the lint on the clean tree. It logs
through the STD-E2E helper to `target/e2e-logs/std_test_lint/<utc>.ndjson`: each command with exit code and wall
time, one `assert` per plant (`kind: negative-control`, `caught`), the count of bound tests per prefix as
`metric` lines, and the `summary`.

### Known answers and negative controls

Known answers used as fixtures of the kit: KA-501 (the docking grid: 344 admitted, 217 excluded, nine on the
boundary of 561 cells; exact); the wheel-loop spectral radius 0.9207 against the mutant's 1.497 (control format
example NC-0412, seed CX-262). Negative controls: the eight lint plants; the hand-written `pass` evidence line;
the three determinism plants. Metamorphic relation on the kit itself: permuting the order in which cases run
changes no drawn input. Determinism lanes: the kit's own draws under `twice` and `platforms`, class D0.

## Considerations

- No blocking dependency is declared, on purpose: most beads depend on this one, so an edge to a seed bead could
  close a cycle. The module lands with `frob-core` (BD-005); if `frob-core` does not exist yet, the conventions
  document and the lint land first and the kit follows in the same session.
- The kit must stay small (cap 1,500 lines inside `frob-core`'s cap of 8,000); it is test support, not a product
  feature, and `xtask` gains no check for it (six checks only): the lint is a test target of the conformance crate.
- Open: the tolerances of 133 known-answer rows and the controls of 325 rows are not stated in the plan; the kit
  reports those rows as obligations until their owners register the missing part (default: not counted).
- BLAKE3 for seed derivation comes from `frob-core`'s hashing; the generator is not cryptographic and is not used
  for any product randomness.
- A reused close reason across many beads is a failure signal; a bead closes citing its own test and its output.

## References

Charter §6; plan §38.2, §38.6, §38.7, §38.11, §35.1, §35.3, §35.4, §35.9, §34.3, §34.10; GATE-GAUNT-01,
GATE-GAUNT-07; INV-GAUNT-02, INV-GAUNT-07; ADR-GAUNT-02, ADR-GAUNT-03.
