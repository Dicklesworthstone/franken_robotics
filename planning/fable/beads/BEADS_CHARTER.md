# Beads charter: turning the FrankenRobotics plan into beads

Every bead drafter and every bead reviewer reads this file first and follows it exactly.

## 1. What we are doing

The plan `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` (version 1.1,
about 37,000 lines) is being converted into a complete, granular set of beads (work items) with tasks, subtasks and
a dependency structure. The owner's instruction, verbatim in substance: take ALL of the plan and elaborate on it to
create a comprehensive and granular set of beads, with detailed comments, so that the whole thing is totally
self-contained and self-documenting, including relevant background, reasoning and justification, considerations,
anything a future self would want to know about the goals, intentions and thought process and how each piece
serves the over-arching goals of the project. **The beads must be so detailed that nobody ever needs to consult the
plan again.**

Two consequences bind everything below:

- **DO NOT OVERSIMPLIFY. DO NOT LOSE ANY FEATURE OR FUNCTIONALITY.** Every mechanism, gate, kill gate, known
  answer, counterexample, refusal, verb, invariant, proof obligation, decision and open question in the plan lands
  in a bead.
- **Tests are part of the beads.** Comprehensive unit tests and end-to-end test scripts with great, detailed
  logging, so that after implementation we can be sure everything works.

The project is at the planning stage (`/Users/jemanuel/projects/franken_robotics/AGENTS.md`): beads describe work;
nobody executes them yet. Beads are created and modified only with the `br` tool. Drafters do not run `br`: they
write bead definition files in the format of §4, and the lead loads them with `br`.

## 2. Where things are

| Thing | Path |
|---|---|
| The plan (find a chapter with `grep -n '^## ' <file>`; chapters are `## N. Title`, appendices `## Appendix X.`) | `/Users/jemanuel/projects/franken_robotics/COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS.md` |
| The seed beads of §45, as data: `bd`, `milestone`, `title`, `deps`, `done_when`, `slice` | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/seed.json` |
| Which slice owns which chapters and seed beads | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/assignment.json` |
| The owner's ruling on siblings and the toolchain (binding; the plan already reflects it) | `/Users/jemanuel/projects/franken_robotics/planning/fable/blend/REVISION_2.md` |
| Your output directory | `/Users/jemanuel/projects/franken_robotics/planning/fable/beads/defs/<your slice>/` |

Useful parts of the plan for every drafter: §0.2 (what "implemented" means: eight conditions every capability
bead inherits), §8 (the seven claims and the claim product), §38 (the verification tiers G0–G5 and the guards
against false progress), §43 (milestones M0–M8), §44 (work packages WP-01 to WP-80, with crates, milestone and
gates), §45 (the seed beads; §45.12 lists 66 subsections that were added late and have no seed bead yet), §40 (the
gate register), Appendix M (known answers KA-001 onward, grouped by chapter), Appendix N (counterexamples CX-001
onward), Appendix O (refusal codes), Appendix F (upstream work items in sibling repositories), Appendix J (verbs),
Appendix L (worked artifacts).

## 3. What a slice must produce

You own one or more chapters and the seed beads assigned to your slice. Produce:

1. **One bead file for every seed bead assigned to you** (key `BD-nnn`). Keep its number, its milestone and its
   dependencies; you may sharpen its title and you must expand its one-line done-condition into a full bead.
2. **New beads for everything in your chapters that no seed bead covers.** Walk every subsection of your chapters
   (including the late subsections listed in §45.12) and every gate, kill gate, known answer, counterexample,
   refusal code, verb, invariant, proof obligation, architecture decision and open question in them. Anything not
   already carried by a bead gets a bead, or is added to the bead where it belongs. Split large mechanisms into
   subtasks: the core mechanism; its refusals and edge cases; its verb or surface; its tests. New keys are
   `N<chapter>-<nn>`, for example `N18-03`.
3. **Registry beads, one set per chapter**, so that nothing enumerable is lost:
   - `N<chapter>-KA`: the known-answer registry of the chapter: every Appendix M row whose section is in your
     chapter, each with its KA number, the case, the truth with its formula or value, the tolerance, the tier and
     its negative control, in a compact table. If the chapter has more than about 60, split into `-KA1`, `-KA2`.
   - `N<chapter>-CX`: the counterexamples of Appendix N that belong to your chapter, each with the tempting
     shortcut, the case with its numbers and the rule it protects. These seed the negative-control registry.
   - `N<chapter>-REF`: the refusal codes your chapter defines (Appendix O and the chapter's refusal table): code,
     meaning, trigger, ranked repairs, exit code.
   Mechanism beads cite KA and CX numbers in their test plans and also state the values they need inline.
4. **End-to-end test beads**: for each user-visible capability of your chapters (a verb, a pipeline, a gate that is
   measured on a running system) a bead for its end-to-end script, per §6 below. One per capability, not one per
   mechanism.
5. **Pilot beads**: every Frontier [F] or Moonshot [M] mechanism names a paper-scale pilot with a falsifier that
   must run before its work is funded (§0.7 of the plan). Each such pilot is its own bead, and the mechanism's
   bead depends on it.
6. **Identifiers that must not be lost.** Each of these, where your chapters define it, is carried by a bead:
   - invariants (`INV-…`) and proof obligations (`PO-…`): named, with their statement, in the test plan of the
     bead whose mechanism they constrain (a property test, a certificate or a proof); one that no mechanism bead
     carries gets its own test bead;
   - architecture decisions (`ADR-…`): stated with their reason and the rejected alternatives in the Background
     of the bead they govern;
   - open questions (`OPEN-…`): a decision bead (label `owner-decision`) or an experiment bead for each one that
     blocks work, stating the fail-closed default that holds until it is decided; the others in the
     Considerations of the bead they concern;
   - kill gates (`KILL-…`): a decision bead each (label `kill-gate`): the trigger with its number, the milestone
     at which it is decided, the measurement that decides it, the fallback, and what changes downstream if it
     fires. Beads that only exist on one branch of the decision depend on it;
   - bets (`BET-…`), risks (`RISK-…`), analyses (`A-…`), constructions (`C-…`), theorem cards (`T-…`): named
     where the bead serves, decides or relies on them.
7. **A coverage file** `coverage.md` in your output directory: (a) a table with one row per subsection of your
   chapters (`§N.k` and its title) and the bead keys that carry it; (b) a table with one row per gate, kill gate
   and admission gate defined in your chapters and the bead keys that measure or decide it; (c) counts of known
   answers, counterexamples and refusal codes carried; (d) "needs from other slices": things your beads depend on
   that another slice owns and for which you could not name a seed key; (e) anything in the plan you found
   contradictory or impossible to turn into work, with what you did about it.

## 4. Bead file format

One file per bead: `<key>.md` in your output directory. A front matter block, then the body. The front matter is
parsed by a simple reader: one `name: value` per line, strings in double quotes, lists as JSON arrays on one line.

```
---
key: "BD-047"
title: "frob-contact: collision primitives and contact sets from certified convex separation"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "traction", "frob-contact", "solid"]
parent: "WP-10"
deps: ["BD-036", "N16-02"]
needs: []
gates: ["GATE-TRAC-02"]
plan_refs: ["§18.2", "§18.7"]
---
## Background
...
## Scope
...
## Technical approach
...
## Acceptance criteria
- [ ] ...
## Test plan
### Unit tests
### End-to-end script and logging
### Known answers and negative controls
## Considerations
## References
```

Field rules:

- `key`: `BD-nnn` for a seed bead; your brief's prefix plus a two-digit number for new beads (`N18a-03`);
  `N<chapter>-KA`/`-CX`/`-REF` for registries. Reserved keys you may depend on but must not write unless your
  brief says you own them: `EPIC-PROGRAM` (the root epic), `EPIC-M0` … `EPIC-M8` (milestone epics), `WP-01` …
  `WP-80` (work-package epics), `STD-E2E` (the end-to-end harness and structured logging library), `STD-TEST`
  (unit-test conventions and the negative-control registry), `SPEC-C-01` … `SPEC-C-40` (the specification bead of
  construction C-nn: its mathematics as a contract, with law tests), `CARD-T-01` … `CARD-T-35` (theorem card
  T-nn), `FD-…` (an upstream work item of Appendix F, by its identifier, for example `FD-FMCP-01`). A key contains
  only letters, digits and hyphens.
- `title`: names the artifact and the outcome, at most about 110 characters, no trailing period. Start with the
  crate or surface where one applies (`frob-contact: …`, `verb sim.run: …`, `upstream frankensim: …`).
- `type`: `epic` (a container), `feature` (a capability a user or agent can observe), `task` (everything else:
  infrastructure, experiments, pilots, tests, reviews, registries).
- `priority`: 0 critical, 1 high, 2 medium, 3 low, 4 backlog. Default by milestone: M0 → 0; M1 → 1; M2 → 1;
  M3 → 2; M4 → 2; M5 → 3; M6 → 3; M7 → 2; M8 → 4. A Moonshot is 4 whatever its milestone. A kill-gate decision
  or an owner decision that blocks others is one step more urgent than its milestone.
- `milestone`: `M0` … `M8`, following the plan (§43 and the gate's milestone in §40; where a chapter and §40
  differ, §40 governs).
- `labels`: always the milestone in lower case (`m2`), the subsystem name in lower case (`marrow`, `skeleton`,
  `sinew`, `kinesis`, `phylum`, `traction`, `habitat`, `sensorium`, `spark`, `reflex`, `cortex`, `arena`,
  `augur`, `nemesis`, `autopsy`, `atelier`, `chronicle`, `ground`, `foundry`, `tether`, `lemma`, `gauntlet`,
  `suite`, `workspace`, `safety`, `tenfold`, `delivery`), the crate where one applies (`frob-contact`), the tier
  (`solid`, `frontier`, `moonshot`), and any of: `test`, `e2e`, `pilot`, `registry`, `kill-gate`, `exit-review`,
  `upstream` (work in a sibling repository), `owner-decision`, `verb`, `gate`.
- `parent`: the work package this bead belongs to (`WP-nn`, from §44, chosen by crate and milestone), or the
  milestone epic `EPIC-Mn` if no work package fits.
- `deps`: keys of beads that must be finished before this one can start (blocking dependencies). Keep every
  dependency the seed lists. Add the others the work really has. A bead never depends on its own parent epic.
  Avoid depending on a later milestone's bead. Every end-to-end bead depends on `STD-E2E`.
- `needs`: free-text descriptions of prerequisites owned by another unit for which you do not know a key, for
  example `"§18.9: the four-dimensional cone rows in the batched solve"`. Always start with the plan subsection.
  The lead resolves them to keys. Prefer a real key: look in `seed_index.md` first.
- `related`: optional keys of beads that are connected but do not block (same fixture, sibling mechanism, the
  package a package follows). Never blocks readiness.
- `gates`: the gate and kill-gate identifiers this bead measures, passes or decides.
- `plan_refs`: plan sections the content came from. Provenance only.

## 5. The body: what each section must contain

A bead must be implementable by an agent who has never seen the plan. Write "what and why and how", not pointers.
Never write "see §18.3" as a substitute for content. Keep the plan's evidence labels where they matter: FACT (with
the path or source), DESIGN (with its reason), HYPOTHESIS (with the test that settles it), TARGET (a number we
intend, not a measurement), OPEN (with its fail-closed default). Mathematics in Unicode, never LaTeX. No
attribution to other plans, models or planning exercises. Plain engineering prose; no praise.

- **Background.** Why this work exists: the problem it solves, where it sits in the system (subsystem, crate,
  layer, tier Solid/Frontier/Moonshot), which goal, bet or North-Star scenario it serves, and the reasoning behind
  the design decision, including the alternatives that were rejected and why. Enough context that the bead stands
  alone: define every term of art the bead uses.
- **Scope.** What is in this bead and what is explicitly out (and which bead has it).
- **Technical approach.** The design in full: types and data layouts (Rust sketches are fine and are labelled
  DESIGN), algorithms as numbered steps, formulas with every symbol defined, constants and thresholds, file and
  module layout, the verb or API surface with inputs, outputs and schema names, refusal codes with their meanings
  and repairs, determinism class, evidence and claim class of outputs, what is recorded in the ledger. Facts about
  sibling libraries the work relies on, with paths. Where the work is in a sibling repository, say which
  repository, under whose rules, and the interim posture and switch-over rule.
- **Acceptance criteria.** A checklist (`- [ ]` lines) of observable conditions, each checkable by a command or a
  test: the gate thresholds with their numbers (TARGET), the negative controls that must be caught, the artifacts
  that must exist (code in its crate, `CONTRACT.md` with its ten sections, known-answer tests against a pinned
  oracle, evidence on outputs, a CPU reference path, reachability through `frob --json`, cancellation tested, the
  gate recorded with measured values). Include the seed bead's done-condition.
- **Test plan.** Three parts.
  - *Unit tests*: named tests, what each asserts, with inputs and expected values. No mocks and no fakes: tests
    run the real component; a synthetic fixture is labelled `synthetic`.
  - *End-to-end script and logging*: the script path (`scripts/e2e/<name>.sh`), what it runs from the user's
    side (the `frob` commands), what it asserts, and what it logs (§6).
  - *Known answers and negative controls*: the KA and CX numbers with their values, the planted failures the test
    must catch, the metamorphic relations that apply, the determinism lanes (twice on one machine; across worker
    counts; debug and release; across platforms and backends, each against its class).
- **Considerations.** Risks and failure modes; the kill gate with its trigger and its fallback, named in advance;
  performance and memory envelopes; security, privacy and licence points; interactions with other subsystems;
  open questions with their owner and fail-closed default; what would make us revisit the design.
- **References.** One or two lines: plan sections, gate, bet, construction, theorem-card, analysis and risk
  identifiers. Provenance only.

Size: a task or feature bead body is typically 60 to 130 lines; an epic 30 to 70; a registry bead as long as its
table needs. A unit typically yields 20 to 45 beads: enough that no bead is more than about five days of work and
nothing in the unit's text is left without a bead, and no more. Do not pad, and do not compress away content: if a mechanism needs 200 lines to be implementable,
either write them or split the bead.

## 6. The testing standard every bead follows

- **Unit tests** live beside the code (`crates/<crate>/tests/` and inline modules), run with `cargo test -p
  <crate>`, use no mocks, and include for every law, invariant and proof obligation a property test, and for every
  gate at least one negative control: a planted defect the test must catch (the plan's rule: a gate that nothing
  can fail validates nothing).
- **End-to-end scripts** live in `scripts/e2e/`, are plain shell driving the real `frob` binary (and `frobd`,
  `frob-mcp` where relevant) on real fixtures, exit non-zero on failure, and are idempotent and seed-explicit.
- **Logging.** Every script sources the shared helper from `STD-E2E` and writes structured newline-delimited JSON
  to `target/e2e-logs/<script>/<utc-timestamp>.ndjson` and a human-readable transcript beside it. Every line has:
  `ts`, `script`, `bead`, `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), the exact command
  line, exit code, wall time, the envelope's `evidence` block (claim, class, tier, determinism, identities, seed),
  measured values with units and their thresholds, digests where state is compared, and on failure the first
  differing value and the artifact paths needed to reproduce. A final `summary` line gives counts of assertions
  passed and failed and the gates touched. Logs never contain private-scene data.
- **Evidence.** A test that claims a gate records the measured value against the threshold; a bead closes with a
  typed reason that cites its own test and that test's output; exit reviews are done by a fresh-context session
  that did not implement the work.

## 7. Dependencies, hierarchy and granularity

- Hierarchy: milestone epic (`EPIC-Mn`) → work-package epic (`WP-nn`) → task and feature beads → their test and
  pilot beads. Hierarchy is expressed with `parent`, never with `deps`.
- Dependencies express real order of work. Prefer shallow, wide graphs: do not chain beads that could proceed in
  parallel. A test bead depends on what it tests. A mechanism that needs a decision depends on the decision bead.
  A Frontier or Moonshot mechanism depends on its pilot bead.
- One bead is one coherent piece of work, roughly one to five days for one agent. A bead that mixes two crates, or
  a mechanism and an unrelated verb, is split.

## 8. When you finish

Before replying, check your own output: every file parses (front matter between two `---` lines, every field
present, lists valid JSON), every key is unique and matches its file name, every `deps` and `parent` key is a
seed key, a reserved key or one of your own keys, every seed bead in your brief has a file, every subsection in
your brief appears in `coverage.md`, and no body says "see §…" in place of content.

Reply with: the number of bead files written (seed beads and new beads separately), the number of subsections,
gates, known answers, counterexamples and refusal codes covered, the largest and smallest bead in lines, and any
seed bead you could not write and why.
