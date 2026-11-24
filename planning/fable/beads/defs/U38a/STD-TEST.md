---
key: "STD-TEST"
title: "unit-test conventions, test kit, registry loaders, determinism lanes and the no-control-no-pass rule"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "frob-core", "solid", "test", "registry", "delivery", "review-notes-open"]
parent: "WP-14"
deps: ["STD-E2E"]
needs: []
related: ["BD-025", "N38a-02", "N38a-12", "N38a-14", "N38a-40", "N38b-04", "N38b-05"]
gates: ["GATE-GAUNT-01"]
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
  default; enabled in dev-dependencies, and as a normal dependency only by `frob-conformance`, whose commands read
  the registries through the one parser of section 5 and on which nothing depends). Reason: every crate already depends on `frob-core`; the crate list is
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

In: the written conventions; `frob_core::testkit`; the one parser of each of the known-answer, negative-control
and metamorphic-relation registries and of the counterexample catalogue, whose types carry every field their
schema owners define; the outcomes of a control and of a gate line; the rule on where numbers come from; the
determinism lanes; the evidence stream of unit tests; the lint that enforces the conventions and the rule on
gates. Out: the end-to-end harness (STD-E2E); the content, schema and validation of each registry (N38a-12 known
answers with the eligibility of a row, N38a-14 relations, BD-025 controls, N38b-05 the counterexample catalogue);
running controls and deciding `stale` (N38b-04); issuing numbers (N38a-40 for known answers and
counterexamples, N35-33 and N35x-02 for refusal codes); the tier runners and receipts (N38a-02); recording a gate's
value in `registries/gates.toml` (N38b-19).

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
| `mr_<id>_` | Metamorphic relation MR-nn, or carried relation MR-Cnn (`mr_c01_`) | `mr_06_similarity_rescaling_bits` |
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
`crates/frob-conformance/registry/known_answers.toml`, `controls.toml`, `relations.toml`, `counterexamples.toml`
(and `pilots.toml`, `comparators.toml` and `allocations.toml`, read by the beads that own them: N38a-09, N38a-04,
N38a-40). The names `registries/known_answers.toml` and `registries/negative_controls.toml` used
in some documents are logical aliases that the loader resolves to the first two files; a second physical copy,
or a chapter's private file or private row format, is a lint failure. `testkit::registry::root()` finds the
workspace root by walking up to the directory holding `franken-stack.lock`. Each type below carries every field
its schema owner defines, the optional ones included (as `Option` or an empty default), so that the owner's
validation can check its rules on the parsed rows; an unknown field is a load error naming the file and row.
- `KnownAnswers::get("KA-501") -> Row { id, section, owner, fixture, same_as, case, truth{kind, value},
  tolerance, tier, sites, control{text, nc}, derivation, label, due, printed, slug }` (N38a-12's row; `sites` is
  a list, `nc` a list of NC identifiers, `owner` the gate, proof obligation or invariant that owns the row).
  `Tolerance` is `Exact | Abs(q) | Rel(q) | PrintedDigits(n) | Gate(id) | ToSet`. `testkit::assert_ka(&row,
  site, observed)` compares under the row's tolerance and reports the site; under `ToSet`, or when the control
  names no NC (`text` is `none stated` or `nc` is empty), it runs the comparison and reports `obligation` with the
  reason (`tolerance`, `control`), never `pass`. The other states of a row (`obligation: derivation`,
  `obligation: rerun`, `flagged: unreconciled`, `boundary-only`, `conditional (<decision>)` from the derivation
  front matter's `conditional_on`) need the derivation files, the pilot register and
  the reconciliation table, so they are N38a-12's computed eligibility, applied by the tier runner to these
  lines; the kit never turns an ineligible row into a gate's pass because only `GateReport` prints one (§8).
- `Controls::for_gate("GATE-ARENA-04")` and `Controls::all() -> Vec<Control { id, seed, gate, kind, target,
  defect, expect, last_seen, feature, fixture, due, permanent, state, removed_check }>` (BD-025's record):
  `gate` is a list; `kind` is `mutation | planted-input | planted-design | planted-world | a-a`; `last_seen` is
  `Option<{caught, session, date}>`, written by the recorder (N38b-19) only; `state` is `Runnable` (default) or
  `Obligation` (written by N38b-05's seeding); `feature`, `fixture`, `due`, `removed_check` are optional strings
  and `permanent` a Boolean, default false.
  `testkit::control(id, || -> ControlOutcome) -> ControlOutcome` runs the planted check and records the outcome:
  `Caught`, `Missed` or `Stale` (the plant cannot be built or found: N38b-04 decides it). A check that fails, but
  not by the observable the record's `expect` states, is reported `Missed` (N38b-04's caught-unexpected rule: a
  control must fail for the stated reason). On a record whose `state` is `Obligation` it returns `Obligation`
  without running the closure; an obligation counts toward nothing.
- `Relations::get("MR-06") -> RelationRow { id, statement, source, sites, compare, bound, mutations, entry, owner,
  due, generated_from }` (N38a-14's record; the type is named `RelationRow` because `Relation` is N38a-14's trait
  for a relation in code). `compare` is `Bits | Exact | Digest | Bound | Order | LeadingOrder`; `mutations` is a
  list of `{name, text, control, feature}`, read from the record's `mutation`, which is one table or an array of
  tables (MR-01 and MR-08 name two each); `id` is `MR-nn` or, for the eleven carried relations, `MR-Cnn`;
  `generated_from` is optional (`Option<String>`, the contract statement `C-nn.Sk` the relation was generated
  from; N38a-14 validates its form).
- `Counterexamples::get("CX-262") -> Counterexample { id, shortcut, case, numeric, plan_case, case_supplied_by,
  rule, rule_ids, defined_in, same_as, label, rerun, rerun_receipt, control_kind, plant, due, slug }` (N38b-05's
  catalogue record, `registry/counterexamples.toml`; `rerun` is present on FACT entries only; `due` is an optional
  milestone). `plant` is a list of plant records `Plant { id, kind, target, defect, expect, gates, feature,
  fixture }`, `id` required and the rest optional; the loader accepts a plain string as the one-plant short form,
  `plant = "SPEC-C-01:cx-019"` reading as `[Plant { id: "SPEC-C-01:cx-019", .. }]`, and an empty list on an
  obligation entry.
- Identifiers are append-only: a loader error is raised on a reused or renumbered KA, CX, NC or MR number.
- One reader per file (DESIGN): these are the only parsers of the four files. Each registry's schema validation
  (N38a-12 for known answers, N38a-14 for relations, BD-025 for controls, N38b-05 for the counterexample
  catalogue, whose counts and join run on these rows) is a function of `frob-conformance` that runs on the rows
  this parser returns, so no registry has a second reader that could disagree with this one.
- **Where numbers come from.** A bead never invents a number. A known-answer or counterexample number beyond the
  plan-time ranges (KA-835, CX-384) comes only from a block issued in N38a-40's `allocations.toml` (its validator
  refuses any other with `unallocated-number`); a negative-control number is appended in order by N38b-05's
  seeding tool or by the bead that registers the control under BD-025's rules; a relation number is N38a-14's;
  a refusal code comes from the allocation ledger N35-33 and the code bead N35x-02. A row that the plan states
  without a number is requested of those beads, never written with a guessed one.

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
rate of discordant task decisions is at most 1% (with none discordant the exact bound is 1 − 0.05^(1/n), at most
1% from n = 299; the plan's "about 3/n, so at least 300 worlds" is the rounded form), and on the net rate at most
0.5%.

**7. Evidence stream.** When `FROB_TEST_EVIDENCE_DIR` is set, each testkit call appends one line
(`frob.testevidence/1`) to `<dir>/<crate>.ndjson`: `ts, crate, test, kind (ka|cx|nc|mr|inv|po|det|law|gate), id,
status (pass|fail|obligation|caught|missed|stale|skip|not-eligible|no-claim), reason, observed, expected,
tolerance, unit, tier, site, seed, determinism, synthetic`. `reason` is set on `obligation` (`tolerance`,
`control`), on `not-eligible` (`no-control`, `stale-control`) and on `no-claim` (the entry's reason). The tier runners and the recorder read these lines; a test never
writes a gate status.

**8. No caught control, no pass.** `testkit::GateReport::new(gate)` collects `.measured(name, value, unit,
threshold)` and `.control(id, outcome)` and ends with `.finish() -> GateLine`. `finish` is the only function in
the workspace that can produce the text `pass` for a gate. In this order: it returns `Fail` if any measurement
misses its threshold or any control is `Missed`; `NotEligible(StaleControl)` if any control is `Stale`;
`NotEligible(NoControl)` if no control was caught in this report (none ran, or every one was an `Obligation`);
and `Pass` otherwise. They print as `fail`, `not-eligible: control NC-nnnn stale`, `not-eligible: no control
caught in this run` and `pass` (the control runner N38b-04 and the receipts of N38a-02 use these texts).
`GateLine` has a fourth form, `NoClaim(reason)`, printed `no-claim: <reason>` (N38a-02's fourth gate line): it is
produced only for a gate whose record has `claim = "reality"`, from an entry that returns `NoClaim{reason}` (the
Reality lane's entry, BD-397, for example `no-claim: no-dataset-admitted`), given to the report as
`.no_claim(reason)` with no measurement and no control; it carries no control outcome and no measured value and is
never a pass. `GateReport` never prints it for any other gate: the runner, which reads the gate's claim, fails
the run with `no-claim-on-non-reality` otherwise. The recorder N38b-19 records the line as status `no-claim` with
the reason in `detail`. `GateLine` has no public constructor.

**9. The enforcing tool.** `crates/frob-conformance/tests/std_test_lint.rs`, run as
`cargo test -p frob-conformance --test std_test_lint`. It fails when: any manifest names a forbidden mocking
crate; a test is `#[ignore]` without a reason string; a `ka_`, `cx_`, `nc_` or `mr_` test names an identifier
absent from its registry; a registry row due at or before the current milestone has no bound test and no runner
entry; a fixture file lacks its kind header; a gate identifier is printed with `pass` outside `GateReport`; two
physical copies of a registry exist; an evidence line carries `kind: gate, status: pass` without a `caught` line
for the same gate and run.

## Acceptance criteria

- [ ] `docs/testing/CONVENTIONS.md` states sections 1 to 8 and is cited under "oracles" from the `CONTRACT.md`
      of every crate that exists when this bead closes; a crate created later cites it from its skeleton bead's
      contract.
- [ ] `frob_core::testkit` builds only under `test-support`; `cargo tree -e normal` of every binary outside
      `frob-conformance` (the product binaries and the workers) shows the feature off; within `frob-conformance`
      only, it is a normal dependency; the module uses the standard library and `frob-core` only.
- [ ] `prop::check` is reproducible: the same seed and case give the same input on both machines; a planted
      failing property is found, shrunk to its minimal input and written to the regressions file.
- [ ] The four loaders read the registries; a duplicate or renumbered identifier is a load error; an unknown
      field is a load error naming file and row; every field of the specimen records of N38a-12, BD-025, N38a-14
      and N38b-05 (the optional ones included) is present in the parsed type.
- [ ] `assert_ka` on a row with tolerance `to set`, or whose control names no NC, reports `obligation` with its
      reason.
- [ ] `control()` returns `Obligation` on an obligation record without running the check, and reports a check
      that fails for a reason other than `expect` as `Missed`.
- [ ] `GateReport` with no control prints `not-eligible: no control caught in this run`; with only obligation
      controls the same; with a stale control prints `not-eligible: control NC-nnnn stale` although another was
      caught; with a missed control prints `fail` although every measurement passed; with a caught control and
      passing measurements prints `pass`; given `.no_claim("no-dataset-admitted")` prints `no-claim:
      no-dataset-admitted`, with no control line and no measured value, and never `pass`.
- [ ] The lint catches each of its eight planted violations and passes on the clean tree.
- [ ] The lanes `twice`, `workers` and `backends` (its decision-level test) each have a passing example and a
      caught negative control (below). The lanes `profiles` and `platforms` need a second build or a second host;
      their helpers are tested here on recorded digest pairs (equal pair accepted, differing pair reported with
      the first differing checkpoint), and their planted defects run in tier G5 (`profiles_catches_contraction` of
      N38a-35; the `arch` lane of N35-03).

## Test plan

### Unit tests

In `crates/frob-core/tests/testkit.rs` (feature `test-support`), real files, no mocks:
- `prop_draw_is_address_function`: draw (seed 7, case 3, stream 1, index 0) is unchanged when stream 2 is added.
- `prop_shrinks_to_minimal`: the property "x < 1000" on u64 fails and shrinks to exactly 1000.
- `prop_vector_shrink`: "no element equals 7" on vectors shrinks to the single-element vector `[7]`.
- `prop_replay_line_reproduces`: running with the printed seed and case reproduces the failure first.
- `ka_tolerance_kinds`: `Exact` rejects 344 against 345; `Abs(0.1)` accepts 33.49 against 33.52 and rejects
  34.08; `Rel(0.02)` accepts 0.985 against 1 and rejects 0.975; the bound is inclusive and compared exactly,
  never through a rounded difference: `Rel(0.25)` accepts 0.75 against 1 (exact in binary), while the `f64`
  value nearest 0.98 lies below 0.98 and is rejected under `Rel(0.02)`; `PrintedDigits` accepts 0.9207 against
  0.92074.
- `ka_obligation_states`: `ToSet` yields `obligation` with reason `tolerance`; `control.text = "none stated"`, and
  a control with text but `nc = []`, yield `obligation` with reason `control`.
- `ka_row_full_fields` (a `synthetic` registry string): a row with `owner`, `same_as = "KA-010"`, `sites = ["host",
  "cuda"]`, `printed` and `slug` set parses with each field; `assert_ka` reports the site it is given; a row with
  an extra field `site` (the retired singular) is a load error naming the row.
- `controls_carry_every_field`: a record with `last_seen`, `feature`, `fixture`, `due`, `permanent = true`, `state
  = "obligation"` and `removed_check` parses with each; one with none of them parses with `last_seen = None`,
  `permanent = false`, `state = Runnable`; `gate = ["GATE-A-01", "GATE-B-02"]` is returned by `for_gate` of
  either.
- `control_outcomes`: a check returning `Caught`, `Missed` or `Stale` is recorded with that status; an
  `Obligation` record does not call its closure (a closure that panics is used to prove it) and returns
  `Obligation`.
- `relation_mutation_table_or_array`: a record with one `mutation` table gives `mutations` of length 1, one with
  an array of two gives 2; `compare = "leading-order"` parses to `LeadingOrder`; `MR-C01` loads and binds the test
  prefix `mr_c01_`; a record with `generated_from = "C-21.S3"` parses with it, one without it gives `None`.
- `counterexample_fields`: a DESIGN entry and a FACT entry with `rerun = false` parse with every field; `rerun`
  on a DESIGN entry is absent; an entry with `due = "M2"` parses with it, one without gives `None`; `plant =
  "N13-18:M2"` and `plant = [{ id = "N13-18:M2" }]` give equal values; a plant table carrying all eight fields
  parses with each, one without `id` is a load error naming the row.
- `registry_of_record_loads`: the four committed files load without error and the test prints one line per file
  with its row count; a file that does not exist yet (its owner has not entered rows) loads as empty and is named
  as such, not as an error.
- `registry_append_only`: a file with KA-010 twice, or with KA-012 before KA-011, is refused; the same for CX, NC
  and MR identifiers.
- `gate_report_no_control`, `gate_report_only_obligations`, `gate_report_stale_control`,
  `gate_report_missed_control`, `gate_report_pass`: the outcomes of section 8, including precedence (a missed and
  a stale control together give `fail`; a caught and a stale control give `not-eligible`).
- `gate_report_no_claim`: a report given `.no_claim("no-dataset-admitted")` finishes `NoClaim` and prints
  `no-claim: no-dataset-admitted`; its evidence line has status `no-claim` with that reason, no control and no
  observed value; no input makes it print `pass`. (The runner's refusal of `NoClaim` on a gate whose claim is not
  `reality`, `no-claim-on-non-reality`, is N38a-02's `no_claim_only_for_reality`.)
- `gate_line_has_no_constructor`: a compile-fail test builds `GateLine` by hand and must not compile.
- `det_twice_catches_time_seeded_value`: a function that mixes the wall clock into a digest fails lane `twice`.
- `det_workers_catches_arrival_order_sum`: a sum taken in arrival order fails lane `workers` for 1 against 8.
- `det_backends_d3_decision`: states 1.0000001 and 0.9999999 around a threshold of 1 are inside the envelope
  and are reported as a discordant decision.
- `det_pair_helper_reports_first_diff`: for the `profiles` and `platforms` lanes, a recorded pair of equal digest
  lists is accepted and a pair differing at checkpoint 3 is reported at checkpoint 3 with its quantity.

### End-to-end script and logging

`scripts/e2e/std_test_lint.sh` (header: `bead=STD-TEST milestone=M0 labels=gauntlet,harness
gates=GATE-GAUNT-01`). In a scratch copy of the workspace it plants, one at a time, the eight violations of
section 9, runs the lint, and uses `e2e_expect_caught` for each; then it runs the lint on the clean tree; then it
loads the four registries of record (`cargo test -p frob-core --features test-support --test testkit --
registry_of_record_loads`, which prints one count line per file). It logs through the STD-E2E helper to
`target/e2e-logs/std_test_lint/<utc>.ndjson`: each command with exit code and wall time, one `assert` per plant
(`kind: negative-control`, `caught`), the count of bound tests per prefix, the row count of each registry and
the number of known-answer rows in each `obligation` reason and of controls in state `obligation` as `metric`
lines, and the `summary`.

### Known answers and negative controls

Known answers used as fixtures of the kit: KA-501 (the docking grid: 344 admitted, 217 excluded, nine on the
boundary of 561 cells; exact); the wheel-loop spectral radius 0.9207 against the mutant's 1.497 (control format
example NC-0412, seed CX-262). Negative controls: the eight lint plants; the hand-written `pass` evidence line;
the three determinism plants; a `GateReport` fed one caught and one stale control (it must not print `pass`); an
obligation control whose closure would panic (it must not run). Metamorphic relation on the kit itself: permuting the order in which cases run
changes no drawn input. Determinism lanes: the kit's own draws under `twice` and `platforms`, class D0.

## Considerations

- The only blocking dependency is the end-to-end harness (STD-E2E), on purpose: most beads depend on this one,
  so an edge to a seed bead could close a cycle. The module lands with `frob-core` (BD-005); if `frob-core` does
  not exist yet, the conventions document and the lint land first and the kit follows in the same session.
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
