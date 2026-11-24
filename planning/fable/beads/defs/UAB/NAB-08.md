---
key: "NAB-08"
title: "e2e scripts/e2e/glossary.sh: registry to generated document to contract lint, with planted defects caught"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "gauntlet", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-01"
deps: ["N00-03", "NAB-01", "NAB-02", "NAB-03", "NAB-04", "NAB-05", "NAB-06", "NAB-07", "STD-E2E"]
needs: []
related: []
gates: []
plan_refs: ["Appendix A", "§0.7", "§34.6", "§38"]
---
## Background

The glossary of FrankenRobotics is a small pipeline: a registry (`registries/glossary.toml`, 337 term records)
is rendered into `docs/GLOSSARY.md` by a generator that `xtask check-generated` re-runs and compares, and a lint
in `frob-conformance` checks that every term of art used in a crate's `CONTRACT.md`, a verb description or a
refusal meaning is defined, that two-sense words are decidable, and that rejected ornamental labels do not
return. Each piece has unit tests. This bead is the end-to-end script that drives the pipeline from the user's
side on the real tree, proves the checks can fail by planting defects, and leaves a structured log. The rule it
serves: a check that nothing can fail validates nothing.

Tier: Solid. This is a test bead; it adds no mechanism.

## Scope

In: `scripts/e2e/glossary.sh`, its planted-defect fixtures, its log, and the module `whole_registry` of
`crates/frob-conformance/tests/glossary_content.rs`: the checks that need all five letter ranges at once (the
counts NAB-01 lists as known answers and the subsystem completeness check), which no range bead can pass alone
because the five are parallel. Out: the registry, generator and lint themselves (NAB-01, NAB-02); the term
records and their per-range tests (NAB-03 to NAB-07).

## Technical approach

Plain shell, sourcing `scripts/e2e/lib.sh` (the shared helper of STD-E2E), idempotent, seed-free (nothing is
random), working on a scratch copy of the tree in the run's working directory
`target/e2e-scratch/glossary/<run_id>/work/tree/` so that planted defects never touch the working tree. The
scratch tree is passed to `xtask` by `--root <dir>` and to the lint and to the `whole_registry` module by
`FROB_LINT_ROOT=<dir>` (NAB-02 step 1; the module reads the registry under that root when the variable is set,
as the lint does). Header lines (`# e2e:` key=value): `bead=NAB-08 milestone=M0 labels=workspace,glossary
plants=hand-edit,deleted-term,undefined-label,ambiguous-sense,ornamental,unsorted,missing-subsystem`.

Steps:

1. `e2e_init glossary NAB-08`; copy every tracked file of the tree (`git ls-files`, so `target/` is never
   copied) into the scratch tree: `check-generated` reads every generator row's inputs and outputs, and a copy
   holding only the glossary's files would fail it with `gen.missing` whether or not a defect is planted. If the
   real tree has no `CONTRACT.md` yet (early M0), copy NAB-02's clean fixture
   contract (`crates/frob-conformance/tests/fixtures/glossary/clean/CONTRACT.md`, labelled `synthetic`, whose
   Outputs section prints `` `provisional` `` as a label value) into the scratch tree as
   `crates/e2e-probe/CONTRACT.md` and log `probe_contract: true`, so that every contract-dependent defect of
   step 7 is exercised from the first day instead of being skipped.
2. **Registry loads.** `e2e_cmd -- cargo test -p frob-core --test registry_glossary`. Assert exit 0.
3. **Counts.** `e2e_cmd -- cargo test -p frob-conformance --test glossary_content` (every range module and
   `whole_registry`; exit 0), then `FROB_GLOSSARY_COUNTS="$PWD/counts.json" e2e_cmd -- cargo test -p
   frob-conformance --test glossary_content -- whole_registry::print_counts` (an absolute path, because a test
   binary runs in its crate's directory), which writes one JSON object (`frob.glossary-counts`) to that file in
   `work/`; the script reads each field with `e2e_get --file counts.json /<field>`:
   `e2e_assert_eq total "$total" 337`; `per_range` 73, 59, 61, 66, 78 (one `e2e_assert_eq` each); `subsystems` 23; `two_sense_terms` 6 (Archetype, Arena, Campaign, Cell, Envelope,
   Tier); `sense_records` 13 (2 + 2 + 2 + 3 + 2 + 2: records whose term occurs in two or more records, so
   `Battery (evaluation)` is not counted); `code_literals` 11; `multi_name` 5; `structural` 4.
4. **Generated document is current.** `e2e_cmd -- cargo xtask check-generated --json`; assert no `gen.differs`
   finding for `docs/GLOSSARY.md`; assert the document has 337 table rows and the marker line.
5. **Regeneration is deterministic.** Run `check-generated --write` twice in the scratch tree;
   `e2e_assert_digest_eq` on the two outputs.
6. **Lint passes on the real tree.** `e2e_cmd -- cargo test -p frob-conformance --test glossary_lint`; emit
   `metric` lines `uses_resolved`, `undefined` (threshold `== 0`), `ambiguous` (`== 0`), `ornamental` (`== 0`),
   `unused` (reported, no threshold), and the lint's wall time against its 5 s target.
7. **Planted defects.** First the baseline: on the unplanted scratch tree `cargo xtask check-generated --root
   <tree> --json` exits 0, the lint with `FROB_LINT_ROOT=<tree>` exits 0, and `whole_registry` with
   `FROB_LINT_ROOT=<tree>` exits 0 (three asserts), so that each catch below is caused by its plant and not by
   the copy. Then each defect through `e2e_expect_caught <name> -- <check>`, in the scratch tree, restored after
   each, and the finding named below asserted in the check's output:
   - `hand-edit`: change one character of `docs/GLOSSARY.md` → `check-generated` reports `gen.differs`.
   - `deleted-term`: remove the `provisional` record → the lint fails with `gloss.undefined-term` on the first
     contract whose Outputs section prints `` `provisional` `` as a label value (the probe contract of step 1
     guarantees one). A deleted term used only in plain prose is invisible to the scanner (NAB-02, negative
     control 7), so the plant uses a label value.
   - `undefined-label`: append "Outputs carry the label `rate-biased`." to the Outputs section of a contract →
     `gloss.undefined-term`.
   - `ambiguous-sense`: append "Each cell is refined once." to the contract of a crate that owns no sense of
     `Cell` (the probe contract owns none) → `gloss.ambiguous-sense`.
   - `ornamental`: append "The fusion is a cellular sheaf." → `gloss.ornamental-label`.
   - `unsorted`: swap two adjacent records → `whole_registry` (which loads through NAB-01's loader) fails with
     the loader error `gloss.unsorted` naming the pair.
   - `missing-subsystem`: delete the `Reflex` record → `whole_registry::all_23_subsystems_defined` fails naming
     `Reflex`.
8. The EXIT trap installed by `e2e_init` writes the `summary` line with assertions passed and failed and the
   controls caught and missed.

**Plants under `E2E_PLANT_FAILURE`.** With `E2E_PLANT_FAILURE=<name>` (one of the seven header plants,
`e2e_plant <name>`), the script applies that defect to the scratch tree before the baseline of step 7, so that
the baseline assertion of the check the plant targets fails, its name carrying the plant's name
(`baseline:<check>:<plant>`), and the script exits 1: the form the runner's `--verify-plants` requires
(STD-E2E). In a normal run the seven defects are caught by `e2e_expect_caught` and the script exits 0.

Exit status: non-zero if any assertion fails or any planted defect is not caught.

## Acceptance criteria

- [ ] `scripts/e2e/glossary.sh` exits 0 on a clean tree and writes
      `target/e2e-logs/glossary/<utc-timestamp>.ndjson` and a transcript beside it.
- [ ] In a normal run each of the seven planted defects is caught with its named finding, after a baseline in
      which the same three checks pass on the unplanted scratch tree. With `E2E_PLANT_FAILURE=<name>` for each
      of the seven names the script exits 1 with a failed baseline assertion naming the plant, and
      `scripts/e2e/run-all.sh --script glossary --verify-plants` exits 0.
- [ ] `whole_registry` passes on the real tree once NAB-03 to NAB-07 have landed: 337 records, 73, 59, 61, 66
      and 78 per range, 23 subsystems equal to the subsystem table of N00-03, 6 two-sense terms with 13 sense
      records, 11 code-literal records, 5 multi-name records, 4 structural records.
- [ ] The log's `summary` line reports at least 20 assertions and no gate (the glossary measures no gate).
- [ ] The script runs in under 60 s on the CPU tier on a warm build (TARGET) and leaves the working tree
      unchanged (`git status --porcelain` is empty before and after).

## Test plan

### Unit tests

The script exercises the unit tests of NAB-01 to NAB-07. This bead adds the module `whole_registry` of
`crates/frob-conformance/tests/glossary_content.rs` (it reads `registries/glossary.toml` through NAB-01's
loader, under `FROB_LINT_ROOT` when set, and `registries/subsystems.toml` of N00-03):

- `whole_registry_count_is_337`, and `per_range_counts`: 73, 59, 61, 66, 78 by first letter of the sort key
  (A–C, D–F, G–M, N–R, S–Z).
- `all_23_subsystems_defined`: the 23 names of the subsystem table each resolve to a record of kind
  `subsystem`, and no other record has that kind.
- `two_sense_terms_and_sense_records`: 6 terms with two or more records, 13 records among them, each with a
  distinct non-empty `sense`.
- `code_literals_are_eleven`; `structural_records_are_four` (`ADR`, `Stratum`, `Witness`, `World`).
- `multi_name_records_are_five`: the term cells of the five frozen fixtures `tests/fixtures/glossary/*.tsv`
  that print several names (separated by ", " or "; ") are five, and every further name of each resolves by
  `lookup` to the record of its first name.
- `print_counts`: when `FROB_GLOSSARY_COUNTS` names a file, writes to it one JSON object
  `{"schema":"frob.glossary-counts","total":…,"per_range":[…],"subsystems":…,"two_sense_terms":…,
  "sense_records":…,"code_literals":…,"multi_name":…,"structural":…}`, which step 3 reads; without the variable
  it writes nothing and passes.

One self-test, `scripts/e2e/selftest/st_glossary_plant.sh`, checks that the planting helper restores the
scratch tree after each defect (digest of the scratch tree before the first plant equals the digest after the
last).

### End-to-end script and logging

This bead is the script. Every log line carries `ts`, `script`, `bead`, `step`, `event`; `cmd` lines carry the
exact command line, exit code and wall time; `assert` lines carry the name, observed and expected values;
`metric` lines carry the value, unit and threshold; state comparisons carry the BLAKE3 digests of
`registries/glossary.toml` and `docs/GLOSSARY.md`; a failed assertion carries the first differing value and the
path of the scratch tree. No `cmd` line carries an `evidence` block, because no `frob` verb is called and
STD-E2E's log schema writes that block only when a command's standard output is an envelope. The glossary holds no private-scene data, and the log
quotes at most the offending phrase and line number of a contract.

### Known answers and negative controls

Known answers: 337 records; 73, 59, 61, 66 and 78 per range; 23 subsystems; 6 two-sense terms with 13 sense
records; 11 code-literal rows; 5 multi-name rows; 4 structural rows. Negative controls: the seven planted
defects of step 7. Determinism lanes: the script twice on one machine gives identical assertion results and
identical digests of the generated document; the generated document's digest is identical on macOS (Apple
silicon) and Linux (x86-64).

## Considerations

- The script depends on `cargo` through the build-offload shim like every other; it never invokes a compiler
  directly.
- Before any crate has a contract the lint would find nothing to scan; the probe contract of step 1 keeps the
  contract-dependent defects live, and the `uses_resolved` metric of step 6 is then reported against the
  probe alone (logged with `probe_contract: true`, so nobody mistakes it for coverage of real contracts).

## References

Appendix A; §0.7 (a gate that nothing can fail validates nothing); §34.6 (`check-generated`); the testing
standard of the conversion charter (structured logs, negative controls).
