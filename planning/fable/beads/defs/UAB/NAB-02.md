---
key: "NAB-02"
title: "frob-conformance: the glossary lint: every term of art used in a CONTRACT.md or verb description is defined"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "gauntlet", "frob-conformance", "solid", "test"]
parent: "WP-01"
deps: ["BD-004", "BD-025", "N32a-02", "NAB-01", "STD-TEST"]
needs: []
related: ["NAB-03", "NAB-04", "NAB-05", "NAB-06", "NAB-07", "NJL-01"]
gates: []
plan_refs: ["Appendix A", "§0.2", "§0.7", "§34.5", "§34.6"]
---
## Background

Every crate of FrankenRobotics ships a `CONTRACT.md` with ten sections: purpose, inputs, outputs, invariants,
refusals, determinism class, evidence class of outputs, validity domain, oracles, known gaps. A contract is
read by an agent that did not write the crate, so every term of art in it must have one definition that the
agent can find. The glossary registry (`registries/glossary.toml`, NAB-01) holds those definitions. This bead is
the check that the two stay joined: a contract that uses a term of art the registry does not define fails, and
so does a contract that uses a two-sense word without the sense being decidable.

It is also the mechanical half of the plain-names rule. The rule: a label from a richer theory is used only
when the mechanism uses that theory's theorems ("a least-squares pose graph is a least-squares pose graph").
A review enforces the judgment; this lint enforces the list: a small registry of ornamental labels that were
considered and rejected (decorated cospans, sub-PROP, Futamura projection, Pearl-causal model, Wasserstein
robustness, cellular sheaf, conformal geometric algebra, max-plus cycle means for balancing two machines)
may not appear in a contract or a verb description unless the term has a glossary record that names the
theorem it relies on.

Why a test and not an `xtask` check (DESIGN): `xtask` holds exactly six checks and a seventh needs a named
defect class and the owner's approval. `check-contracts` validates the presence of the ten sections and the
machine-readable `contract.toml`; it does not grade prose. This lint reads prose, so it lives in
`frob-conformance` and runs with `cargo test -p frob-conformance --test glossary_lint`.

Alternatives rejected: (a) a spell-checker style scan for every capitalised phrase, which produces noise on
type names and proper nouns and is switched off within a week; (b) requiring authors to tag terms by hand in
contracts (`{term}`), which makes contracts unreadable and still misses untagged uses. The design below scans
for known terms and for a short, explicit set of term-shaped patterns.

## Scope

In: the scanner, the candidate-term rules, the sense rule and its `[glossary]` key in the contract schema, the
ornamental-label list, the allow file, the report format, the test target. Out: the registry and generator (NAB-01); the term records (NAB-03 to NAB-07); the
evidence-label and wording lints (N00-01); the identifier lint (N00-03); grading whether a definition is right
(the exit review).

## Technical approach

**1. Inputs scanned.** (a) every `crates/*/CONTRACT.md`, `surface/*/CONTRACT.md`, `spark/*/CONTRACT.md`,
`derive/*/CONTRACT.md`; (b) the prose fields of each entry of `registries/verbs.toml` (`purpose` and the
when-to-use sentence, from which the tool description is generated; the verb registry's schema names them); (c) the
`meaning` fields of `registries/refusals.toml`; (d) `docs/*.md` except `docs/delivery/**` and generated files. A
generated file is one whose first line is the generator marker that `check-generated` requires (N34p4-03); the
lint skips it whole, so a printout that quotes a rejected label (the killed-idea register's document) is not
scanned.
Fenced code blocks and inline code spans that are not registered code terms are skipped. The tree that is walked
is the workspace root, or the directory named by the environment variable `FROB_LINT_ROOT` when it is set
(DESIGN: the end-to-end script of NAB-08 runs the lint on a scratch copy holding its planted defects; the
real tree is never edited to plant a defect).

**2. What counts as a use of a term of art.** A token sequence is a *use* when one of these holds:

1. It matches a registered `term` or `alias`, case-insensitively, at word boundaries, longest match first
   (so `minimal counterexample world` is matched before `world`).
2. It is an inline code span whose text is a registered code term (`` `provisional` ``, `` `tier-sensitive` ``).
3. It is a *candidate*: a phrase the contract itself marks as defined vocabulary, by any of three patterns:
   bold on first use (`**audit lane**`), the form "a/an/the X (…)" followed by "is" within the Purpose or
   Invariants section where X is two to four words in title or lower case, or an inline code span of lower-case
   letters and hyphens that appears in a sentence of the Outputs or Refusals section as a label value
   (`` `capacity-biased` ``).

Uses of kinds 1 and 2 resolve by construction. A candidate of kind 3 that resolves to no record is the finding
`gloss.undefined-term` (path, line, the phrase, the nearest registered terms by edit distance).

**3. The sense rule.** When a use matches a `term` that has two or more senses, the lint decides the sense by,
in order: an explicit qualifier in the text (`cell (mission)`, `mission cell`, `clearance cell`); the
`owner_crate` of exactly one sense equal to the crate whose contract is being read; the contract's
`[glossary] senses = { cell = "mission" }` table in `contract.toml`. If none decides, the finding is
`gloss.ambiguous-sense` with the candidate senses. A crate that genuinely uses two senses declares both in
`contract.toml` and qualifies each use. This bead adds the optional table `[glossary]`, with its one key `senses`
(a map from a registered term to one of its senses), to `schemas/contract.schema.json`: under the rule of
`check-contracts` (N34-08) the bead that introduces a `contract.toml` key adds it to the schema, and a contract
using an absent key fails `contract.schema`.

**4. The ornamental-label list.** `registries/glossary.toml` carries `[[ornamental]]` records: `label`, `plain`
(the plain name to use), `allowed_if` (the theorem whose use would license the label). The registry schema is
hand-written (NAB-01), so this bead adds the `[[ornamental]]` table, with exactly these three string fields, to
`schemas/registries/glossary.schema.json` and to NAB-01's loader in the same commit (an unknown field there fails
`gloss.schema`). A use of a `label`
outside a record that satisfies `allowed_if` is `gloss.ornamental-label`, with the plain name printed as the
repair. Seed records (eight): decorated cospan → composition over typed ports; sub-PROP → the same; Futamura
projection → build-time partial evaluation; Pearl-causal model → interventions on a forkable simulator;
Wasserstein robustness → a worst case over a bounded set; cellular sheaf → a least-squares pose graph;
conformal geometric algebra → primitive intersection tests; max-plus cycle mean for balancing two machines → a
role table for two machines.

**5. Reverse direction (warning, not failure).** A record that no scanned file uses is listed as
`gloss.unused-term`. It is a warning because a term may be defined before the crate that uses it exists; at a
milestone exit the list is reviewed and a term with no use and no planned user is removed (the registry-must-
have-a-reader rule applied to rows).

**6. The allow file.** `crates/frob-conformance/registry/glossary_allow.toml`: `[[allow]] path, phrase, reason,
expires` for a candidate phrase that is not a term of art (a proper noun, a quoted external term). An entry
without `reason`, or past its `expires` milestone, fails. The allow file is itself bounded: more than 40 entries
fails `gloss.allow-file-too-large`, because a long allow list means rule 3 of step 2 is wrong and should be
tightened instead.

**7. Report.** A table per file: uses resolved (count), undefined candidates, ambiguous senses, ornamental
labels; JSON form `frob.glossary-lint/1` for the end-to-end script. Exit status of the test: failed if any
finding of the three failing kinds exists. The lint's own budget: under 5 s on the full tree (TARGET).

**8. Determinism and evidence.** D0: a pure function of the scanned bytes and the registry. The lint records
nothing in the ledger and claims nothing about the system; it is hygiene.

## Acceptance criteria

- [ ] `cargo test -p frob-conformance --test glossary_lint` passes on the clean tree with every `CONTRACT.md`
      present at that time, and prints the per-file table.
- [ ] Each of the seven planted fixtures of the test plan fails with its named finding and the path and line.
- [ ] A two-sense term used in a crate that owns one sense resolves without annotation; used in a third crate
      without a qualifier it fails `gloss.ambiguous-sense`.
- [ ] `schemas/contract.schema.json` admits `[glossary] senses`; a fixture `contract.toml` carrying it passes
      `cargo xtask check-contracts`, and one carrying `[glossary] sense` (misspelled) fails `contract.schema`.
- [ ] Each of the eight ornamental labels is refused in a contract with its plain name as the repair, and the
      same text is accepted inside a generated document (a fixture file whose first line is the generator
      marker, as the killed-idea register's printout will be; that printout is NAB-32's and is not built here).
- [ ] The allow file holds no entry without a reason, and a 41st entry fails.
- [ ] The lint is not an `xtask` subcommand, and the six-subcommand test still passes.
- [ ] The lint finishes in under 5 s on the full tree (TARGET), with the measured time in its report.

## Test plan

### Unit tests

`crates/frob-conformance/tests/glossary_lint.rs`, driven by fixture trees under
`crates/frob-conformance/tests/fixtures/glossary/` (real files, labelled `synthetic`):

- `clean_contract_passes`: a ten-section contract using `commit fence`, `base interval` and `` `provisional` ``
  resolves three uses and reports no finding.
- `undefined_bold_term_fails`: `**shadow lane**` in Purpose → `gloss.undefined-term`, nearest `audit lane`.
- `undefined_label_value_fails`: `` `rate-biased` `` in Outputs → `gloss.undefined-term`.
- `longest_match_wins`: `minimal counterexample world` counts once, and not also as `world`.
- `sense_from_owner_crate`: `cell` in `frob-mission`'s contract resolves to `Cell (mission)`.
- `ambiguous_sense_fails`: `tier` in a contract of `frob-chronicle` with no qualifier →
  `gloss.ambiguous-sense` listing `maturity` and `physics`.
- `qualifier_resolves`: `physics tier` and `tier (maturity)` both resolve.
- `senses_table_decides_and_validates`: `tier` in a fixture crate whose `contract.toml` has
  `[glossary] senses = { tier = "physics" }` resolves to `Tier (physics)`, and that `contract.toml` validates
  against `schemas/contract.schema.json`; a sense that the term does not have fails `gloss.ambiguous-sense`.
- `ornamental_label_fails_with_plain_name`: "a cellular sheaf over the sensor graph" →
  `gloss.ornamental-label`, repair "a least-squares pose graph"; then each of the eight seed labels in turn, each
  with its own `plain` as the repair.
- `generated_file_is_exempt`: a fixture `docs/decisions/killed-ideas.md` whose first line is the generator marker
  and which quotes "cellular sheaf" produces no finding; the same file with the marker line removed produces
  `gloss.ornamental-label`.
- `code_block_is_skipped`: an undefined phrase inside a fenced block produces no finding.
- `prose_definition_absent_fails`: "The shadow budget (…) is the share of …" in Invariants, with no record for
  `shadow budget` → `gloss.undefined-term` (negative control 6).
- `deleted_registry_term_fails`: the clean fixture contract, whose Outputs section prints `` `provisional` `` as a
  label value, checked against a copy of the registry without the `provisional` record → `gloss.undefined-term`
  on that line (negative control 7). A deleted term is caught only where the contract uses it in one of the
  three candidate forms of step 2, rule 3; a plain-prose use of a deleted term is invisible to the scanner,
  which is why NAB-01's change rule removes a term only after this lint, run with the term still present,
  reports it as `gloss.unused-term`.
- `allow_entry_requires_reason`, `expired_allow_entry_fails`, `forty_one_allow_entries_fail`.
- `unused_term_is_a_warning_only`.

### End-to-end script and logging

`scripts/e2e/glossary.sh` (NAB-08) runs the lint on the real tree and then on each planted tree through
`e2e_expect_caught`. Logged per STD-E2E: the command line, exit code, wall time, per-file counts as `metric`
lines (`uses_resolved`, `undefined`, `ambiguous`, `ornamental`, `unused`), the digest of the registry, and for a
failure the path, line and phrase of the first finding.

### Known answers and negative controls

Known answers: on the fixture contract of `clean_contract_passes` exactly three uses resolve; on the five-sense
fixture (two `Archetype`, three `Cell`) five records are reachable by qualifier. Negative controls (each must be
caught, and the lint must be shown to fail without them being special-cased): (1) an undefined bold term; (2)
an undefined label value; (3) an unqualified two-sense word; (4) an ornamental label; (5) an allow entry with no
reason; (6) a contract that defines a term in prose that contradicts nothing but is absent from the registry;
(7) a registry from which one used term was deleted (the lint must then fail on the contract that uses it).
Metamorphic relation: reordering the sections of a contract changes no finding. Determinism lanes: twice on one
machine and on both platforms, identical report bytes.

## Considerations

- False positives are the failure mode that kills a lint. The candidate rule is deliberately narrow (three
  patterns); widening it needs evidence from real contracts, recorded with the change.
- The lint cannot tell whether a definition is true. The defining contract governs; the exit review reads new
  terms.
- Interaction: N00-01 lints evidence labels and wording in the same files, N00-03 lints identifiers. The three
  share one file walker so the tree is read once.
- A contract in a sibling repository is not scanned; siblings keep their own vocabulary.
- Revisit if more than 5% of findings over one milestone are allow-listed: the rules are then wrong.

## References

Appendix A; §0.2 (the ten sections); §0.7 (plain names; process is never the product); §34.5, §34.6; doctrine
rule D-2 (vocabulary is bound to the theorems used).
