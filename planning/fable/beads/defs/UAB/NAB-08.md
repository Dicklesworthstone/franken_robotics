---
key: "NAB-08"
title: "e2e scripts/e2e/glossary.sh: registry to generated document to contract lint, with planted defects caught"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "gauntlet", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-01"
deps: ["STD-E2E", "NAB-01", "NAB-02", "NAB-03", "NAB-04", "NAB-05", "NAB-06", "NAB-07"]
needs: []
related: ["N00-08", "N00-03"]
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

In: `scripts/e2e/glossary.sh`, its planted-defect fixtures, its log. Out: the registry, generator and lint
themselves (NAB-01, NAB-02); the term records (NAB-03 to NAB-07).

## Technical approach

Plain shell, sourcing `scripts/e2e/lib.sh` (the shared helper of STD-E2E), idempotent, seed-free (nothing is
random), working on a scratch copy of the tree under `target/e2e-work/glossary/` so that planted defects never
touch the working tree. Header: `bead=NAB-08 milestone=M0 labels=workspace,glossary`.

Steps:

1. `e2e_init glossary NAB-08`; copy `registries/`, `docs/GLOSSARY.md`, every `CONTRACT.md` and `contract.toml`
   into the scratch tree.
2. **Registry loads.** `e2e_cmd -- cargo test -p frob-core --test registry_glossary`. Assert exit 0.
3. **Counts.** `e2e_cmd -- cargo test -p frob-conformance --test glossary_content -- --nocapture` and read the
   printed counts. `e2e_assert_eq total 337`; per range 73, 59, 61, 66, 78; `e2e_assert_eq subsystems 23`;
   `e2e_assert_eq two_sense_terms 6` (Archetype, Arena, Campaign, Cell, Envelope, Tier) and
   `e2e_assert_eq sense_records 13` (2 + 2 + 2 + 3 + 2 + 2).
4. **Generated document is current.** `e2e_cmd -- cargo xtask check-generated --json`; assert no `gen.differs`
   finding for `docs/GLOSSARY.md`; assert the document has 337 table rows and the marker line.
5. **Regeneration is deterministic.** Run `check-generated --write` twice in the scratch tree;
   `e2e_assert_digest_eq` on the two outputs.
6. **Lint passes on the real tree.** `e2e_cmd -- cargo test -p frob-conformance --test glossary_lint`; emit
   `metric` lines `uses_resolved`, `undefined` (threshold `== 0`), `ambiguous` (`== 0`), `ornamental` (`== 0`),
   `unused` (reported, no threshold), and the lint's wall time against its 5 s target.
7. **Planted defects** (each through `e2e_expect_caught <name> -- <check>`, in the scratch tree, restored after
   each):
   - `hand-edit`: change one character of `docs/GLOSSARY.md` → `check-generated` reports `gen.differs`.
   - `deleted-term`: remove the `commit-fence` record → the lint fails on the first contract that uses
     "commit fence" with `gloss.undefined-term` (skipped with a recorded `skip` while no contract uses it).
   - `undefined-label`: append "Outputs carry the label `rate-biased`." to a contract → `gloss.undefined-term`.
   - `ambiguous-sense`: append "Each cell is refined once." to the contract of a crate that owns no sense of
     `Cell` → `gloss.ambiguous-sense`.
   - `ornamental`: append "The fusion is a cellular sheaf." → `gloss.ornamental-label`.
   - `unsorted`: swap two adjacent records → loader error `gloss.unsorted`.
   - `missing-subsystem`: delete the `Reflex` record → the subsystem completeness test fails.
8. `e2e_step summary`: the helper writes the `summary` line with assertions passed and failed.

Exit status: non-zero if any assertion fails or any planted defect is not caught.

## Acceptance criteria

- [ ] `scripts/e2e/glossary.sh` exits 0 on a clean tree and writes
      `target/e2e-logs/glossary/<utc-timestamp>.ndjson` and a transcript beside it.
- [ ] With `E2E_PLANT_FAILURE=<name>` for each of the seven names, the corresponding check fails and the script
      records it as caught; with the lint disabled (a test hook that returns success unconditionally) the
      script itself exits non-zero, which shows that the catch is not vacuous.
- [ ] The log's `summary` line reports at least 20 assertions and no gate (the glossary measures no gate).
- [ ] The script runs in under 60 s on the CPU tier on a warm build (TARGET) and leaves the working tree
      unchanged (`git status --porcelain` is empty before and after).

## Test plan

### Unit tests

None of its own: the script exercises the unit tests of NAB-01 to NAB-07. One self-test,
`scripts/e2e/selftest/st_glossary_plant.sh`, checks that the planting helper restores the scratch tree after
each defect (digest of the scratch tree before the first plant equals the digest after the last).

### End-to-end script and logging

This bead is the script. Every log line carries `ts`, `script`, `bead`, `step`, `event`; `cmd` lines carry the
exact command line, exit code and wall time; `assert` lines carry the name, observed and expected values;
`metric` lines carry the value, unit and threshold; state comparisons carry the BLAKE3 digests of
`registries/glossary.toml` and `docs/GLOSSARY.md`; a failed assertion carries the first differing value and the
path of the scratch tree. The envelope `evidence` block is absent because no `frob` verb is called; the helper
records `evidence: null` and the reason `no-verb`. The glossary holds no private-scene data, and the log
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
- Before any crate has a contract the lint steps find nothing to scan; the script then records `skip` for the
  defects that need a contract, and the exit review of the first milestone with contracts re-runs it.

## References

Appendix A; §0.7 (a gate that nothing can fail validates nothing); §34.6 (`check-generated`); the testing
standard of the conversion charter (structured logs, negative controls).
