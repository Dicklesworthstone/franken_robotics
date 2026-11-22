---
key: "NAB-01"
title: "registries/glossary.toml: the term registry (schema frob.glossary/1) and the generated docs/GLOSSARY.md"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["BD-002", "BD-004", "BD-005"]
needs: ["§0.3: the identifier-prefix table, which owns the prefixes (BET, GOAL, NS, C, T, A, DC, GATE, KILL, INT, INV, PO, OPEN, ADR, FD, WP, BD, GB, KA, H, N, FMT, FROB-REF) that the glossary does not redefine (N00-03)", "§34.6: the generator table of `xtask check-generated`, to which this bead adds one row (BD-004)"]
related: ["NAB-02", "NAB-03", "NAB-04", "NAB-05", "NAB-06", "NAB-07", "NAB-08", "N00-03", "N00-08"]
gates: []
plan_refs: ["Appendix A", "§0.3", "§0.5", "§0.7", "§34.1", "§34.6"]
---
## Background

FrankenRobotics is a robotics simulation and design system written for agents, and it uses about 340 terms of
art: names of subsystems (Marrow, Skeleton, Phylum), names of mechanisms (trust ladder, commit fence, audit
lane), labels that appear in tool output (`provisional`, `tier-sensitive`, `model-conditional`), and ordinary
words used in a restricted sense (cell, campaign, envelope, tier). Several ordinary words carry two or three
senses in different subsystems: a *cell* is a cube of a clearance complex in Habitat, a box of a design family in
Augur and the scope of a Reality result in Ground. An implementing agent that meets one of these words in a
`CONTRACT.md` needs one authoritative line that says what it means here and which chapter owns the definition.

The glossary is that line, kept as data. This bead builds the registry file, its schema, its loader and the
generator that renders `docs/GLOSSARY.md`. The terms themselves are carried by NAB-03 to NAB-07 (five letter
ranges, 337 records at plan time); the lint that checks contracts against the registry is NAB-02.

Design decisions, with reasons (DESIGN):

- **Data first, document generated.** The glossary is a TOML registry and the Markdown file is generated from
  it and never edited (rule L-8 of the workspace: generated files are never edited; `xtask check-generated`
  regenerates and compares). Rejected: a hand-edited Markdown glossary, because the lint of NAB-02 needs term
  identities, senses and aliases as data, and a second copy would drift.
- **No new check.** `xtask` has exactly six subcommands, and a seventh needs a named defect class and the owner's
  approval. The generator is one more row in `check-generated`'s generator table; the term lint is a test in
  `frob-conformance` (NAB-02).
- **The defining chapter governs.** A glossary row is an abbreviation. Where a row and the contract of the crate
  that owns the term differ, the contract is right and the row is corrected. The registry therefore records, for
  every term, where its definition lives.
- **A registry must have a reader.** The hygiene rule of the program is that every registry exists because a
  named verb reads it or a named check uses it, and one that nothing reads is deleted. Readers of this registry:
  the generator (`check-generated`), the contract lint (NAB-02), and the plain-names review rule (a term from a
  richer theory is used only where a theorem of that theory is used), which looks a new term up here first.

Tier: Solid. Subsystem: document control under the workspace.

## Scope

In: `registries/glossary.toml` (the file, empty of terms until NAB-03 to NAB-07 land), its JSON-Schema, the
loader, the canonical ordering rule, the generator and its row in `check-generated`, the structural tests of the
registry. Out: the term records (NAB-03 to NAB-07); the lint over `CONTRACT.md` files and verb descriptions
(NAB-02); the end-to-end script (NAB-08); the identifier-prefix table (N00-03).

## Technical approach

**1. File and schema.** `registries/glossary.toml`, schema name `frob.glossary/1`, JSON-Schema at
`schemas/registries/glossary.schema.json` (generated with the other registry schemas). One `[[term]]` table per
row:

```toml
schema = "frob.glossary/1"

[[term]]
id          = "cell-mission"          # slug: lower-case ASCII, digits, hyphens; unique in the file
term        = "Cell"                  # as printed; a code literal keeps its backticks out of this field
sense       = "mission"               # "" for a term with one sense; required and distinct when a term repeats
code        = false                   # true when the term is a literal printed in backticks (`provisional`)
kind        = "concept"               # concept | subsystem | label | binary | class | quantity | record
meaning     = "A box of a design family's parameter space that carries one verdict and one certificate"
defined_in  = ["§25.3"]               # provenance: the chapter subsection that owns the definition
constructs  = []                      # constructions, doctrine rules, gates or bets the row names ("C-17", "D-13")
aliases     = ["cells"]               # inflections and alternative spellings the lint accepts for this term
owner_crate = "frob-mission"          # crate whose CONTRACT.md is authoritative once it exists; "" before
design_values = false                 # true when the row quotes a number; the number is then a DESIGN value
```

Field rules:

1. `id` is unique. `(term, sense)` is unique, compared case-insensitively.
2. A `term` that occurs in two or more records has a non-empty, distinct `sense` in each (for example `Cell` with
   senses `clearance`, `mission`, `reality`; `Tier` with `maturity`, `physics`).
3. A row that names several terms defined together (`Adjoint, coadjoint`; `Cold run, warm campaign`;
   `Degraded`, `Faulted`; the five binaries; `Toolchain T; toolchain train`) is one record whose `term` is the
   first name and whose `aliases` list the others; each alias resolves to the record.
4. `defined_in` is non-empty except for the records marked `structural = true`, which are defined by the glossary
   itself (at plan time four: `ADR`, `Stratum`, `Witness`, `World`).
5. `meaning` is one sentence or two, at most 320 characters, plain prose, Unicode mathematics, no LaTeX.
6. A number inside `meaning` requires `design_values = true`; the number is a DESIGN value of the defining
   section unless the text says FACT or TARGET.
7. An identifier prefix (`GATE-`, `KILL-`, `BD-` and the others of the prefix table) is not a term. The thing a
   prefix names may be a term where it needs a definition (`Kill gate`, `Convergence gate`, `Known answer`).

**2. Canonical order.** Records are stored and rendered in alphabetical order of a sort key: `term` with
backticks removed, lower-cased, compared by Unicode code point; ties by `sense`. The loader refuses a file that
is out of order (`gloss.unsorted`, naming the first pair), so two agents adding terms cannot produce two
orderings.

**3. Loader.** `frob_core::registry::glossary::load(path) -> Result<Glossary, RegistryError>` (in the registry
reader module the other registries use), with `Glossary::lookup(&str) -> Vec<&Term>` (case-insensitive, over
`term` and `aliases`; more than one hit means the caller must give a sense) and `Glossary::iter()`. Errors are
typed: `gloss.schema`, `gloss.duplicate-id`, `gloss.duplicate-term-sense`, `gloss.missing-sense`,
`gloss.unsorted`, `gloss.no-definition`, `gloss.meaning-too-long`, `gloss.number-unlabelled`.

**4. Generator.** `xtask::generate::glossary`, a pure function from `registries/glossary.toml` to
`docs/GLOSSARY.md`, added to the generator table of `check-generated` with mode "regenerate, compare bytes". The
output is deterministic (sorted rows, `\n` endings, a first-line marker naming the generator and its input):

```
<!-- generated by xtask generate glossary from registries/glossary.toml; do not edit -->
# Glossary
<the five standing rules, as prose>
| Term | Meaning | Defined in |
|---|---|---|
| Acceptance predicate | The Boolean predicate A on executions … | C-17, C-30, §24.5 |
| Archetype (design) | One of the eighteen design templates … | §14.10 |
| `provisional` | The label of a fast-tier result that has not been promoted | §16.8 |
```

A term with a sense prints as `Term (sense)`; a code term prints in backticks; a multi-name record prints its
names joined as the plan-time row does. `cargo xtask check-generated --write` rewrites the file; a hand edit
fails `gen.differs` with the first differing line.

**5. The five standing rules** (rendered at the head of the document, and enforced by the loader where they are
mechanical): every term of art has one line with the section that defines it; the defining section governs
where the glossary abbreviates; terms are in alphabetical order; a term in two senses has two rows; identifier
prefixes are registered in the prefix table and appear here only where the thing they name needs a definition;
numbers quoted in a row are DESIGN values of the cited section unless the row says otherwise.

**6. Change rule.** Adding a term is an ordinary change with the contract that introduces it. Changing a
`meaning` requires the defining contract to change in the same commit. Removing a term requires that the lint of
NAB-02 finds no remaining use. Determinism class of the generator: D0 (exact; bytes are a function of the
registry). Evidence class: not applicable; the registry makes no claim about the system.

## Acceptance criteria

- [ ] `registries/glossary.toml` exists, validates against `schemas/registries/glossary.schema.json`, and loads
      through `frob_core::registry::glossary::load`.
- [ ] After NAB-03 to NAB-07 land, the registry holds 337 records: the 336 rows of the plan-time glossary and the
      one row it omitted (`Reflex`).
- [ ] `docs/GLOSSARY.md` is produced by the generator, carries the marker line, and `cargo xtask
      check-generated` passes on a clean tree and fails with `gen.differs` after a one-character hand edit.
- [ ] `xtask` still has exactly six subcommands (the six-subcommand test passes).
- [ ] Each loader error of step 3 is produced by its planted fixture.
- [ ] `cargo xtask check-generated --write` followed by `check-generated` exits 0, and running `--write` twice
      leaves the file byte-identical.

## Test plan

### Unit tests

`crates/frob-core/tests/registry_glossary.rs` (real files in a temporary directory; no mocks):

- `loads_minimal_registry`: a file with three records loads; `lookup("cells")` returns the record whose
  `aliases` contain `cells`.
- `duplicate_term_without_sense_is_refused`: two records `Cell` with empty `sense` → `gloss.missing-sense`.
- `duplicate_term_sense_is_refused`: `Tier`/`physics` twice → `gloss.duplicate-term-sense`.
- `unsorted_file_is_refused`: `Bead` before `Battery` → `gloss.unsorted` naming that pair.
- `sort_key_ignores_backticks_and_case`: `` `lockstep` `` sorts between `Load case` and `Marrow`; `` `unknown` ``
  between `Tube table` and `Unresolved cell`.
- `multi_name_record_resolves_every_alias`: `lookup("coadjoint")` and `lookup("Faulted")` each return one record.
- `number_without_design_flag_is_refused`: a meaning containing "8 m" with `design_values = false` →
  `gloss.number-unlabelled`.
- `meaning_longer_than_320_is_refused`.
- `structural_records_may_omit_definition`: `Stratum` with `structural = true` and empty `defined_in` loads; the
  same record without the flag → `gloss.no-definition`.

`xtask/tests/generate_glossary.rs`: `output_is_deterministic` (two runs, equal bytes);
`sense_and_code_terms_render` (`Cell (mission)`, `` `provisional` ``); `marker_line_present`;
`hand_edit_is_caught` (one changed byte → `gen.differs` with line number).

### End-to-end script and logging

`scripts/e2e/glossary.sh` (owned by NAB-08) runs `cargo xtask check-generated --json` and the registry tests on
the real tree. This bead's part: the script asserts the record count, the marker line and the determinism of
`--write`. Logged per STD-E2E: each `cmd` with exit code and wall time, an `assert` per check with observed and
expected values, the BLAKE3 digests of `registries/glossary.toml` and `docs/GLOSSARY.md`, and on failure the
first differing line.

### Known answers and negative controls

Known answers at plan time: 337 records; 14 records whose term carries a sense qualifier in parentheses
(`Archetype` twice, `Arena (device)`, `Battery (evaluation)`, `Campaign` twice, `Cell` three times, `Envelope`
twice, `Instants (seven)`, `Tier` twice); 11 code-literal rows; 5 multi-name rows; 4 structural rows; 23
subsystem names (the twenty-one that own a chapter, `Gauntlet` and `Airlock`). Negative controls (each must be
caught): a planted duplicate `id`; a planted out-of-order pair; a hand edit of `docs/GLOSSARY.md`; a record
whose `sense` repeats. Determinism lanes: the generator twice on one machine and on both platforms (macOS on
Apple silicon, Linux on x86-64) produces identical bytes.

## Considerations

- Process is never the product: the loader, generator and tests together should stay under about 400 lines. If
  the glossary grows a second consumer that needs more structure, that is a new decision, not a quiet extension.
- The registry records provenance as plan subsections because no crate contract exists yet. As each crate's
  `CONTRACT.md` lands, `owner_crate` is filled and the contract becomes the authority; the subsection reference
  stays as provenance.
- Risk: the glossary drifts from the contracts. NAB-02's lint closes one direction (a contract uses an undefined
  term); the change rule of step 6 closes the other.
- OPEN (owner: Tether): whether `frob robot-docs` should print glossary definitions on request. Fail-closed
  default: it does not; agents read `docs/GLOSSARY.md`.

## References

Appendix A (preamble and rules); §0.3 (prefixes); §0.5 (subsystem names); §0.7 (registries need readers; plain
names); §34.6 (the six checks, rule L-8).
