---
key: "NAB-16"
title: "e2e scripts/e2e/sources.sh: source register counted, archive checked, citation lint shown to bite"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "test", "e2e", "review-notes-open"]
parent: "WP-01"
deps: ["N40-01", "N40-02", "NAB-09", "NAB-10", "NAB-11", "NAB-12", "NAB-13", "NAB-14", "NAB-15", "STD-E2E"]
needs: []
related: []
gates: []
plan_refs: ["Appendix H", "§0.1", "§38"]
---
## Background

FrankenRobotics keeps every outside fact it relies on in one register, `registries/sources.toml`: local files
read in sibling repositories, public pages and papers with a mark that says how each was read (P primary text,
S summary, F fetched with no record of which, R search summary, M recalled, U unmarked), standard references
cited from memory, 25 small programs run at plan time, and a list of 13 things that were explicitly not
verified. A lint in `frob-conformance` refuses a gate, a contract's oracle or a close reason that cites a
source which still needs a reading, and refuses reliance on an unverified item.

This bead is the end-to-end script for that capability. It runs the pipeline on the real tree from the command
line, checks the register's known counts, checks the planning archive against its recorded digests, and plants
the defects the lint exists to catch. Tier: Solid; a test bead.

## Scope

In: `scripts/e2e/sources.sh`, its planted fixtures and its log. Out: the register, the records and the lint
(NAB-09 to NAB-15); re-reading sources and re-running checks (N00-34, the plan-time pilot ledger N38a-10 and the pilot re-run beads).

## Technical approach

Plain shell, sourcing `scripts/e2e/lib.sh`, idempotent, offline (no URL is fetched), working on a scratch copy
under the harness's scratch root `target/e2e-scratch/sources/<run_id>/`, passed to `xtask` by `--root <dir>`
and to the conformance tests by `FROB_LINT_ROOT=<dir>` (NAB-02 step 1; NAB-10 step 6). Header:
`bead=NAB-16 milestone=M0 labels=gauntlet,sources plants=pass-on-unread,unknown-source,bare-url,plan-check-not-reproduced,relies-on-unverified,by-eye,mark-raised,reread-edited,fabricated-url,archive-unknown-revision`.

1. `e2e_init sources NAB-16`; copy `registries/`, `crates/frob-conformance/registry/`, contracts and `docs/`
   into the scratch tree.
2. **Register loads and counts.** `e2e_cmd -- cargo test -p frob-core --test registry_sources` and
   `e2e_cmd -- cargo test -p frob-conformance --test sources_content -- --nocapture`. Assertions:
   `public_urls == 147` (49 + 98); `public_brackets == 163` (39 P, 92 F, 30 S, 2 U); `public_readings == 174`
   with `P == 43`, `F == 95`, `S == 34`, `U == 2`, `R == 0`, `M == 0` (the split, versioned and shared-bracket
   rules of NAB-12 and NAB-13); `topics_public == 13`; `references == 43` with `M == 43`; `plan_checks == 25`;
   `unverified == 13`; `local_topics == 13`; `commits_recorded == 5`.
3. **Flags.** Every non-P reading reports `reread_before_gate = true`; the count of public readings that may be
   cited without a reading equals the 43 public P readings plus those with confirmed re-reads (0 at plan time);
   no local reading (before its re-read at the stack-lock commit), reference reading or plan-time check (before
   its re-run) may be cited; and the script logs the number of open reading tasks as a `metric`.
4. **Archive.** Compute `shasum -a 256` of the single `.zip` under `planning/`; `e2e_assert` that it equals
   exactly one of the four recorded revisions and log which. Then extract two revisions (NAB-15 step 1), each
   into its own new, empty scratch directory that holds nothing else: the **cited** revision, written from
   `git show a635b91:planning/astra-10x-reproduction.zip` and asserted to have the digest of the `a635b91` row
   (when `git cat-file -e a635b91` fails, as in a shallow clone, the script runs `git fetch --unshallow` once
   and, if the commit is still absent, fails with that reason logged; it never substitutes the working-tree
   file), and the working-tree revision. In each, run `e2e_cmd -- python3 -I -B verify.py` (the runner checks the inventory of
   released file names and the SHA-256 of each listed file, then replays the standard-library checks; it
   expects to be started from the directory that holds its README). Then run three of its inner programs in each
   revision and compare the cited revision's printed counts with the recorded values, and the working-tree
   revision's with the cited revision's: `mor/repro/P01-inverse.py` (stance family: 11,520 of
   11,520, half-width 3/14 m), `mor/repro/P05-docking-family.py` (561 cells: 344 admitted, 217 excluded) and
   the promotion-rule enumeration `sill/promotion_rule.py` (2,145 pairs: 521, 521, 1,103, 0; its recorded
   output `sill/promotion_rule.out` is compared byte for byte). Then
   `e2e_expect_caught archive-negative -- python3 -I -B verify_negative.py`. Plant `archive-unknown-revision`:
   one byte appended to a scratch copy of the archive → the digest step fails `src.archive-unknown-revision`.
5. **Summary block is current.** `e2e_cmd -- cargo xtask check-generated --json`; no `gen.differs` for
   `docs/SOURCES.md`.
6. **Citation lint on the real tree.** `e2e_cmd -- cargo test -p frob-conformance --test citations`; `metric`
   lines per finding kind (all zero on a clean tree except the warning `cite.needs-reading`, whose count is
   logged with no threshold).
7. **Planted defects** (each through `e2e_expect_caught`, restored afterwards):
   - `pass-on-unread`: a gate record with status `pass` citing an F reading → `cite.pass-on-unread`.
   - `unknown-source`: a citation `SRC-PUB-999` → `cite.unknown-source`.
   - `bare-url`: an unregistered URL in a contract → `cite.unregistered-url`.
   - `plan-check-not-reproduced`: a gate citing `SRC-CHK-18` while the ledger says `not-run`.
   - `relies-on-unverified`: a contract sentence stating an absolute incumbent throughput on an RTX 4090 as
     FACT → `cite.relies-on-unverified` (NV-01).
   - `by-eye`: the 410 s training figure cited without "± 20 s" → `cite.by-eye-without-uncertainty`.
   - `mark-raised`: an F mark changed to P with no `reread` entry → the frozen-mark comparison fails.
   - `reread-edited`: an existing `reread` entry altered → the loader fails `src.reread-edited`.
   - `fabricated-url`: a URL given to the unlocated benchmark page → `no_url_is_fabricated` fails.
8. **Two positive paths.** (a) Append a `reread` entry with `result = "confirmed"` to the F reading of the
   `pass-on-unread` plant and re-run the lint: the same gate record is now clean. (b) In the scratch copy, give
   one part of GATE-SPARK-21 a recorded value (a scratch fixture row labelled `synthetic`; the real gate register
   is never written, and the script records no gate) and set NV-01 to `status = "measured"` with that record:
   the `relies-on-unverified` contract of step 7 is now clean (NAB-14's acceptance). Both show that the lint
   can be satisfied only through the registers.
9. `summary`.

## Acceptance criteria

- [ ] The script exits 0 on a clean tree and writes `target/e2e-logs/sources/<utc-timestamp>.ndjson` with a
      transcript beside it.
- [ ] All nine planted defects are caught, and both positive paths of step 8 pass.
- [ ] The archive digest is matched to a recorded revision and logged; with a byte appended to a scratch copy
      of the archive the step fails `src.archive-unknown-revision`.
- [ ] The script fetches nothing from the network (it runs with networking disabled in CI) and leaves the
      working tree unchanged.
- [ ] Under 120 s on a warm build on the CPU tier (TARGET), excluding the first build of the test binaries.

## Test plan

### Unit tests

None of its own; the script drives the unit tests of NAB-09 to NAB-15. Self-test
`scripts/e2e/selftest/st_sources_restore.sh`: after the nine plants the scratch tree's digest equals its digest
before the first.

### End-to-end script and logging

This bead is the script. Per STD-E2E every line carries `ts`, `script`, `bead`, `step`, `event`; `cmd` lines
the exact command line, exit code and wall time; `assert` and `metric` lines the observed value, the expected
value or threshold and the unit; digests of `registries/sources.toml`, of the archive and of
`docs/SOURCES.md`; on failure the first differing value, the source id, the mark and the scratch path. No
`frob` verb is called, so the `evidence` block is recorded as absent with reason `no-verb`. The final `summary`
line lists assertions passed and failed and touches no gate.

### Known answers and negative controls

Known answers: 147 URLs; 163 public brackets (39 P, 92 F, 30 S, 2 U) entered as 174 readings (43 P, 95 F,
34 S, 2 U); 13 public topics; 43 references; 25
plan-time checks; 13 unverified items; four archive revisions (230,188; 241,568; 258,254; 275,178 bytes). The
archive checks pin 11,520 of 11,520, 344 + 217 = 561 and 521 + 521 + 1,103 + 0 = 2,145. Negative controls: the
nine plants of step 7 and the archive's own negative runner. Determinism lanes: twice on one machine, identical
assertion results and digests; on macOS and Linux the same counts (the archive's Python checks use the standard
library only).

## Considerations

- The archive is extracted into its own new, empty directory and run there with `python3 -I -B` (isolated
  mode, no bytecode written); that directory is never on the import path of anything else and nothing from it
  runs during a build.
- The script does not record GATE-GAUNT-08; it checks that the register and the archive are in the state the
  re-run beads expect.
- If a sibling repository is not checked out, the local-path existence step records `skip` with the repository
  name.

## References

Appendix H (all parts); §0.1 rule 1; §38 (negative controls; a gate that nothing can fail validates nothing).
