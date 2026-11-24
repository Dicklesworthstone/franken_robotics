---
key: "NEG-27"
title: "prior-art register for interior generation (retail, warehouse; not read) and the no-novelty claim check"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "habitat", "frob-scene", "frob-conformance", "solid", "test", "asset", "registry"]
parent: "WP-23"
deps: ["BD-139", "BD-225", "BD-226", "BD-302", "N19b-02", "NAB-12", "NAB-13", "NAB-14"]
needs: []
related: ["NEGp1-51"]
gates: []
plan_refs: ["Appendix G.2 (rows 11, 12 and the correction)", "§19.7", "Appendix H.3", "Appendix H.6"]
---
## Background

An earlier text of the specification suggested that generating interior scenes for robot evaluation was new.
It is not, and the specification says so in a correction. FACT (public sources read on 2026-11-21, to be
re-read from the primary source before a gate cites them): generators or builders of interior scenes are
published for homes (Infinigen Indoors, ProcTHOR), kitchens (RoboCasa), retail stores (RoboBenchMart,
MarketGen) and warehouses (the Isaac Sim Warehouse Creator, which turns a two-dimensional plan into a
warehouse and has a headless interface). None was found for manufacturing plants or commercial kitchens;
that is an absence found by a bounded search, not evidence that none exists. What this project adds to a
generated interior is four things: the hazard list with dimensions and provenance, the optical map, the
feasibility tag with its cut, and the acceptance report.

Three of those prior-art items have rows in the asset table with licence "not read": the two retail
generators (known from abstracts only; no URL was recorded) and the warehouse builder of Isaac Sim 6.2.0
(documentation pages read; licence not read). Their use is "reference only, as prior art". This bead gives
them their register rows, keeps them out of the tree, and installs the check that no document, report or
verb output of the project claims novelty for interior generation.

## Scope

In: three register rows of kind prior art; the absence checks; the claim check with its permitted and
forbidden wordings; the prior-art note shipped in the scene crate's contract. Out: the licence machinery
(N19b-02); the project's own generators (NEG-17); the four additions themselves (hazard registries, optical
maps, feasibility tags, scene acceptance, each owned by the scene unit).

## Technical approach

**Register rows.**

| Key | Licence | Class | `read` | Kind | Use | Source-register status |
|---|---|---|---|---|---|---|
| `robobenchmart` | not read | `unknown` | false | prior-art | reference only, retail interiors | abstract read; no URL recorded |
| `marketgen` | not read | `unknown` | false | prior-art | reference only, retail interiors | abstract read; no URL recorded |
| `isaac-sim-warehouse-creator-6.2.0` | not read | `unknown` | false | prior-art | reference only, warehouse interiors | documentation pages read |

All three: `default_pipeline = false`, `redistribute = "never"`. First task: record a primary URL for each of
the two retail generators in the source register (none was recorded), or record that none could be found.

**Absence checks.** No file with one of these source keys in the tree; no dependency, container or script
references the tools; no import profile names them; a user-supplied import naming one of them is classed
`unknown` and is unexportable: its pack is refused by the pack export check with FROB-REF-310 and anything else
derived from it at the export door with FROB-REF-733 or FROB-REF-731 (N19b-02's division; the codes are raised
by those beads, not here).

**Claim check (`no_novelty_claim_for_interior_generation`).** A lint over `README`, `docs/`, crate contracts,
verb descriptions, report templates and schema examples:

- Forbidden: statements that interior generation, procedural interiors or generated scenes for robot
  evaluation are new, first, novel or unique to this project (a pattern list with the usual synonyms, scoped
  to sentences that mention interiors, scenes or archetypes).
- Permitted and encouraged: the four additions named above, each stated as what is added to a generated
  interior.
- Required wording where absence is mentioned: any sentence saying no generator exists for manufacturing
  plants or commercial kitchens must carry the qualifier that this is an absence found by a bounded search
  on a stated date.
- Reports: the scene section of a design, run or campaign report prints the scene's generator identity and
  never a novelty statement; the report templates are part of the lint's input. The templates and the report
  verbs are BD-302's (a dependency, so that this rule is checked on reports that exist); this bead adds the
  lint and, where a template's scene section lacks the generator identity, the line that prints it.

**Prior-art note.** `crates/frob-scene/CONTRACT.md` carries a short section listing the six prior-art items
by scene type with their source-register marks and the sentence on bounded search, so that the claim's
boundary travels with the code.

## Acceptance criteria

- [ ] The three rows exist with `read = false`, class `unknown`; the source register has a URL or a recorded
      "not found" for each retail generator.
- [ ] The absence checks pass and each fails on a planted violation.
- [ ] The claim lint passes on the tree, fails on a planted "first procedural interior generator" sentence
      and on an unqualified absence sentence, and accepts the four-additions wording.
- [ ] The contract section exists and lists six items across four scene types, with the two scene types for
      which none was found.

## Test plan

### Unit tests

- `prior_art_rows_valid`: three rows, `read = false`.
- `prior_art_absent_from_tree`, `prior_art_not_a_dependency`.
- `claim_lint_rejects_novelty`: five planted phrasings.
- `claim_lint_requires_bounded_search_qualifier`.
- `claim_lint_accepts_four_additions`.
- `report_templates_have_no_novelty_statement`.

### End-to-end script and logging

`scripts/e2e/assets_licences.sh` (the M3 scene-asset loop, NEGp1-51): the absence scan and the claim lint over
the tree and over a freshly rendered `frob report design` and `frob report run`; log lines carry the rows'
keys and flags, lint hits (zero expected) with file and line, and the generator identity printed by each
report.

### Known answers and negative controls

The correction itself is the rule under test. Controls: the planted novelty sentences; an absence sentence
without its qualifier; a prior-art tool referenced from a script. Determinism: lints are pure functions of
the tree.

## Considerations

- A lint cannot judge every sentence; it catches the common phrasings, and review catches the rest. Its value
  is that the boundary of the claim is written down where a contributor will meet it.
- If a generator for manufacturing plants or commercial kitchens is found later, the note is amended with
  its date and the absence sentence is removed everywhere the lint finds it.

## References

Appendix G.2 (rows 11, 12 and the correction); §19.7; Appendix H.3; Appendix H.6.
