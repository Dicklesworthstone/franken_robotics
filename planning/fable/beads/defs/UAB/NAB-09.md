---
key: "NAB-09"
title: "registries/sources.toml: the source register (schema frob.sources/1), reading marks, licence, re-read rule"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "gauntlet", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["BD-002", "BD-004", "BD-005"]
needs: []
related: ["NAB-23"]
gates: []
plan_refs: ["Appendix H (preamble)", "Appendix H.1", "§0.1", "§0.6", "§0.7"]
---
## Background

FrankenRobotics is at the planning stage: **nothing in the specification was compiled, run or measured as
FrankenRobotics, and no crate of the project exists.** Every statement about the system is a DESIGN decision, a
HYPOTHESIS or a TARGET. Every FACT the specification relies on is one of three things: a reading of local
source or documentation in a sibling repository, a reading of a public source (documentation, a paper, a pull
request, a benchmark page), or the result of a small program run at plan time on a small model. The first rule
of the program for outside numbers is: *an external number is a FACT about its source, not about this system*.
It is registered with where it was read, the version or date it was read at, how it was read, and it is re-read
from the primary source, or re-run at a pinned commit, before a gate cites it.

This bead builds the register that makes the rule mechanical: `registries/sources.toml`. The other beads of the
set fill it (NAB-11 local sources, NAB-12 and NAB-13 public sources, NAB-14 standard references and the list of
things not verified, NAB-15 the plan-time checks), and NAB-10 is the citation lint that reads it.

**How a text reached its reader is recorded as a mark.** The mark says how the text was read, not how reliable
the source is:

| Mark | Meaning |
|---|---|
| P | Read from the primary text: a PDF converted to text, a raw file at a release tag, or a raw documentation page |
| S | Reported through a summary: the page was read through a summarizing fetch tool, and quotes are short for that reason |
| F | Fetched during planning; the record does not say whether the text was raw or summarized. Treated as S |
| R | A search-engine summary only; the page was not opened |
| M | Recalled, not re-read |
| U | Cited with no mark recorded |

Two rules govern marks. (1) Where two records disagree about a source, the weaker mark is kept, except that a
source re-read by an independent audit is P for the figures that audit quotes. (2) **Only a P reading may be
cited by a gate without a further reading; every other mark is a reading task that the gate's bead performs
first.**

Design decisions (DESIGN): one register for all four kinds of source, because a gate's citation should resolve
the same way whatever it cites; a mark per *reading* and not per source, because several sources were read
partly from the primary text and partly through a summary; licence recorded per source, because some sources
are data or assets whose licence decides whether they may be used at all. Rejected: citing URLs inline in
contracts, which cannot carry a mark, a version or a re-read history. Tier: Solid.

## Scope

In: the file, its schema, the loader, the mark order, the re-read record, the generated summary counts. Out: the
records (NAB-11 to NAB-15); the lint over gates, contracts and close reasons (NAB-10); re-reading itself (N00-34
for published figures, N00-16 for sibling-code facts, the plan-time pilot ledger N38a-10 with N00-35 and the pilot re-run beads for plan-time checks).

## Technical approach

**1. File and schema.** `registries/sources.toml`, schema `frob.sources/1`, JSON-Schema at
`schemas/registries/sources.schema.json`, hand-written by this bead like every registry-file schema (not a
`check-generated` output, BD-004). It names every field below, including the kind-specific fields the record
beads use (`role`, NAB-11; `group` and `context_only`, NAB-14; `cited_in`, `scripts_retained`, `revisions`,
`cited_revision` and `limits`, NAB-15); a field outside it fails `src.schema`.

```toml
schema = "frob.sources/1"
read_on_default = "2026-11-21"                # the date of the plan-time readings
local_root = "/Users/jemanuel/projects/"      # the one absolute prefix of every local locator; a replication kit
                                              # rewrites this key alone (NAB-11)

[[source]]
id        = "SRC-PUB-027"                     # SRC-LOC-nnn local file or crate; SRC-PUB-nnn public; SRC-REF-nnn
                                              # standard reference; SRC-CHK-nn plan-time check; SRC-MACH-nn machine probe;
                                              # SRC-REC-nn planning record. Never renumbered; a retired id is tombstoned.
kind      = "public"                          # local | public | reference | plan-check | machine | planning-record
topic     = "incumbent-engines-mujoco"        # grouping used by the generated summary
title     = "MuJoCo Warp documentation, index page source"
locator   = "https://mujoco.readthedocs.io/en/latest/_sources/mjwarp/index.rst.txt"
                                              # a URL; or "<repo>:<path>" under the projects directory; or "" when
                                              # no URL was recorded (then `unlocated = true`)
repo      = ""                                # sibling repository name for a local source
role      = ""                                # local sources only: linked-candidate | document | build-fact |
                                              # defect-report | pattern (NAB-11); "" for other kinds
commit    = ""                                # commit the reading was made at, where one was recorded
version   = ""                                # release tag or version the text was read at ("4.0.1", "v1.8.0")
read_on   = "2026-11-21"
licence   = "unknown"                         # SPDX id, "owner" (an owner's own repository), "n/a" (a fact, not a work
                                              # we redistribute), or "unknown"
licence_verified = false
unlocated = false
locator_derived = false                       # true when the locator was built from a bare identifier (arXiv id)
group     = ""                                # reference records only: carried | mission-and-synthesis | numerics |
                                              # scene-and-sensing | control (NAB-14)
context_only = false                          # true: named as context; no conformity may be claimed against it
cited_in  = []                                # plan-check records only: the chapters that cite the check (NAB-15)
scripts_retained = true                       # plan-check records only: false when the check's script was not kept
revisions = []                                # the archive record SRC-REC-05 only (NAB-15): { commit, bytes, sha256,
                                              #   entries } per revision, entries = 0 when not counted
cited_revision = ""                           # the archive record only: the commit of the revision the record cites
limits    = []                                # the archive record only: the stored limits text, one string per entry

  [[source.reading]]                          # one or more; each has its own mark
  mark    = "P"                               # P | S | F | R | M | U
  covers  = "batched model fields, per-world assets, the determinism answer, the 60-degree-of-freedom note"
  supports = ["§1.1 incumbent audit: per-world parameters", "§17 topology as data"]   # what the reading is used for
  figures = []                                # typed figures: { name, value, unit, hardware, how = "raw-text" |
                                              #   "figure-by-eye" | "derived" | "executed", uncertainty }
                                              #   ("executed": the output of a program run at plan time)

  [[source.reread]]                           # appended by a re-read; never edited
  date       = ""
  session    = ""
  at_version = ""                             # or the commit, for a local source
  result     = "confirmed"                    # confirmed | corrected | not-found | changed
  note       = ""
  record     = ""                             # ledger reference of the reading note

[[unverified]]                                # a statement the plan explicitly did not verify (NAB-14)
id          = "NV-01"                         # NV-nn, append-only
statement   = "Absolute MuJoCo Warp or Newton throughput on an RTX 4090"
why         = "The published absolute figures are for an RTX 6000 Ada and an RTX 5090"
measured_by = ["GATE-SPARK-21"]               # gates, open questions or beads that settle it
status      = "unverified"                    # unverified | measured | read | withdrawn; changes only with `record`
record      = ""                              # gate record, source id or ADR once settled

[[tombstone]]                                 # a retired id: never reused, never renumbered
id     = "SRC-PUB-099"
reason = "duplicate of SRC-PUB-027"

[history]                                     # one digest appended with each reread entry, in file order
reread_digests = ["<blake3 of the canonical bytes of each reread entry>"]
                                              # the loader recomputes and compares (src.reread-edited); an edit
                                              # that also rewrites a digest shows in the diff of an append-only list
```

**2. Derived fields (computed by the loader, not stored).**

- `effective_mark(reading)`: `F` is treated as `S`. Strength order, strongest first: P, S (and F), R, M, U.
- `weakest_mark(source)`: the weakest effective mark over its readings.
- `may_cite_without_reading(reading)`: true iff the reading's mark is P, or a `reread` with `result =
  "confirmed"` exists at a recorded version or commit and covers the same figures.
- `reread_before_gate(reading)` = not `may_cite_without_reading(reading)`. This is the flag the citation lint
  and the re-read reviews consume.
- For `kind = "local"`: the reading is direct, but the statement is "a FACT about the commit it was read at";
  `may_cite_without_reading` requires a `reread` at the commit recorded in the stack lock.
- For `kind = "plan-check"`: `may_cite_without_reading` requires the re-run record of the plan-time check ledger
  to say `reproduced`.

**3. Loader and errors.** `frob_core::registry::sources::load`. Typed errors: `src.schema`, `src.duplicate-id`,
`src.no-reading`, `src.bad-mark`, `src.unlocated-without-flag` (empty `locator` with `unlocated = false`),
`src.local-path-missing-repo`, `src.unverified-status-without-record` (an `[[unverified]]` record whose
`status` is not `unverified` and whose `record` is empty), `src.reread-edited` (a `reread` entry differs from the previous commit's copy of
the same entry; checked against a digest list kept in the file's `[history]` table), `src.id-reused`.

**4. Mark-merge rule as code.** `merge_marks(a, b, audited_figures)`: returns the weaker of the two, except that
a reading whose `covers` is listed in `audited_figures` is P. Used when two records of one source are merged on
import.

**5. Generated summary.** `xtask::generate::sources_summary` writes the block between markers in
`docs/SOURCES.md`: counts per kind, per topic and per mark; the list of unlocated sources; the list of readings
with `reread_before_gate = true` grouped by the gate or bead that cites them. It is one more row of the
`check-generated` generator table, not a new check.

**6. Readers (the registry-must-have-a-reader rule).** The citation lint (NAB-10); the evidence-label lint
(N00-01), which resolves every external figure to a source id; the incumbent capability register (N00-13),
whose rows carry source keys; the review that re-reads published figures (N00-34); the asset-licence join for
scenes.

**7. Determinism and evidence.** The loader and the summary generator are D0. A source record makes no claim
about FrankenRobotics; it is what a claim of kind Parity or Known answer cites.

## Acceptance criteria

- [ ] `registries/sources.toml` validates and loads; with NAB-11 to NAB-15 landed every record has at least one
      reading and a kind.
- [ ] `effective_mark` treats F as S; `weakest_mark` and `merge_marks` follow the order P, S/F, R, M, U, with the
      audit exception.
- [ ] `reread_before_gate` is true for every reading that is not P and has no confirmed re-read, for every
      local source not re-read at the stack-lock commit, and for every plan-time check not reproduced.
- [ ] An empty locator without `unlocated = true` is refused; the plan-time sources with no recorded URL load
      only with the flag set.
- [ ] `docs/SOURCES.md`'s generated block is current under `check-generated`, and `xtask` still has six
      subcommands.
- [ ] A `reread` entry cannot be edited or removed without the loader failing.

## Test plan

### Unit tests

`crates/frob-core/tests/registry_sources.rs` (real files in a temporary directory):

- `f_is_treated_as_s`: a reading marked F has `effective_mark = S` and `reread_before_gate = true`.
- `p_reading_may_be_cited`: mark P → `may_cite_without_reading = true`.
- `mixed_readings`: a source with readings P (covers "abstract and Table A.5") and S (covers "elsewhere") has
  `weakest_mark = S`; a citation of Table A.5 needs no reading, a citation of another figure does.
- `merge_keeps_weaker_unless_audited`: `merge_marks(P, R, [])` = R; with the figure in `audited_figures` = P.
- `confirmed_reread_lifts_flag`: an S reading with a `reread { result = "confirmed", at_version = "4.0.1" }`
  covering the figure → `reread_before_gate = false`; `result = "corrected"` leaves the old figure uncitable
  and requires a new reading record.
- `local_needs_stack_lock_commit`: a local reading re-read at a commit other than the stack lock's stays
  flagged.
- `unlocated_requires_flag`; `duplicate_id_refused`; `reread_edit_refused`.
- `kind_specific_fields_load`: a `plan-check` record with `cited_in`, `scripts_retained = false` and a figure
  with `how = "executed"`, and a `planning-record` with two `revisions`, a `cited_revision` and three `limits`,
  load and validate against the hand-written schema; a record carrying an undeclared field `note` fails
  `src.schema` in the loader and in schema validation.
- `summary_counts`: on a five-record fixture the generated block prints the expected counts per mark.

### End-to-end script and logging

`scripts/e2e/sources.sh` (NAB-16) loads the real register, prints the summary and runs the citation lint. This
bead's assertions there: the register loads; the per-mark counts equal the known answers of NAB-11 to NAB-15;
every `unlocated` source is listed. Logged per STD-E2E with the digest of `registries/sources.toml`.

### Known answers and negative controls

Known answers: six marks; strength order P > S = F > R > M > U; on the fixture of `mixed_readings` exactly one
of two citations needs a reading. Negative controls: a reading with mark "X" (`src.bad-mark`); a source with no
reading; an F reading presented as citable; an edited `reread` entry; a retired id reused for a new source.
Determinism: the summary block is byte-identical across two runs and both platforms.

## Considerations

- The register records how a text was read on 2026-11-21. Pages move and papers are revised: a re-read that
  finds a changed figure appends `result = "changed"` with the new value as a new reading; the old reading is
  kept, flagged, and every citation of it is listed for review.
- Privacy and licence: locators of local sources are paths inside the owner's projects directory; the register
  is not exported in any replication kit without the path prefix being rewritten to repository-relative form.
- The register is not a bibliography manager. If it needs fields beyond those above, the question is whether a
  gate reads them.
- OPEN (owner: Gauntlet): whether `docs/SOURCES.md` should print full records or only the summary block.
  Fail-closed default: the summary block only.

## References

Appendix H preamble and H.1; §0.1 rule 1 (an external number is a FACT about its source); §0.6; §0.7.
