---
key: "NAB-13"
title: "registries/sources.toml: public sources on learning, symmetry, gaits, missions, statistics, reality, assets"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["NAB-09"]
needs: []
related: ["NAB-14"]
gates: []
plan_refs: ["Appendix H.3", "Appendix H.1", "§0.8"]
---
## Background

Beyond the incumbent engines (NAB-12), the specification of FrankenRobotics leans on published research in
nine areas. The hypothesis register (which decides what the program may claim and build) quotes ranges from
these papers: how well one policy transfers across bodies, what a parent-to-child policy inheritance costs,
how fast full-order gait orbits solve, what sequential tests and rare-event estimators have been measured to
gain, how conformal bands behave, what topology optimization saves. Every such range is a HYPOTHESIS about this
system and a FACT only about its source. This bead enters the sources into the register
(`registries/sources.toml`, schema `frob.sources/1`, NAB-09), each with its mark: **P** read from the primary
text, **S** through a summarizing fetch, **F** fetched with no record of which (treated as S), **R** a search
summary only, **M** recalled, **U** no mark recorded.

Most of these sources carry mark F or S: 85 of the 102 bracketed readings. That is the honest state of the
planning record, and it is why the citation lint (NAB-10) refuses a gate that cites one of them before it has
been re-read. Tier: Solid.

## Scope

In: the public-source records of nine topics: training time, sampling control and agent-driven design loops;
family policies, inheritance, rank agreement and population co-design; symmetry, equivariance and
geometric-algebra policies; gait orbits, continuation and certificates for control; mission queries,
co-design, synthesis and region certificates; studies, rare events, requirements and attribution; reality,
calibration, structure and living worlds; estimation and gradients through contact; robot models and scene
assets. Out: the incumbent and toolchain topics (NAB-12); standard references and the not-verified list
(NAB-14); the schema and the lint (NAB-09, NAB-10).

## Technical approach

**Transcription rule** (as in NAB-12). One `[[source]]` per URL or named document, `kind = "public"`, `topic` =
the group name, `read_on = "2026-11-21"`. Each bracket becomes a `reading` with its `mark`; text after a colon
is `covers`; "P for X; F elsewhere" becomes two readings. A bare arXiv identifier (`arXiv 2510.25850`) is
stored with `locator = "https://arxiv.org/abs/<id>"` and `locator_derived = true`, because the identifier
determines the address. A named source with no address (`a symmetry library's README`; the two retail-scene
generators, for which the text says no URLs were recorded) has `unlocated = true`. The one address printed
without a scheme, `raw.githubusercontent.com/LeCAR-Lab/dial-mpc/main/dial_mpc/examples/` [P], is stored with
`https://` prefixed and `locator_derived = true`; it is a record of its own and not one of the 98 URLs counted
below. The register therefore holds 106 records for these topics: 98 URL records, 4 derived arXiv records, this
one derived address and 3 unlocated records.

**Readings versus brackets** (DESIGN, so that the counts below are reproducible). One bracket can yield more
than one reading: (a) a split bracket ("P for X; F elsewhere", "P for X; S elsewhere") yields two readings,
three times in these topics (arXiv 2511.19204v3, 2409.06366v4, 2505.05753v2); (b) the versioned bracket of
arXiv 2510.25850 yields two P readings (versions 2 and 3); (c) a bracket after two sources joined by "and"
gives each of the two records its own copy of the reading, four times (the abs and pdf URLs of 1512.08055; the
abs and html URLs of 2107.07511; the bare identifiers 2603.13616 and 2503.10966; the two retail-scene generators,
RoboBenchMart and MarketGen, which are two unlocated records). The 102 brackets therefore become 110 readings.

**Special cases.**

- Version-specific readings: `arXiv 2510.25850`, versions 1 to 3 [P]: the per-round wall-clock statement
  survives in version 3; the minutes-and-dollars figure is in version 2 only. Two readings, each with its
  `version`, so that a citation of the dollars figure cannot silently resolve to version 3.
- Two sources carry mark **U** (no mark recorded): `https://arxiv.org/abs/2610.14064` and
  `https://arxiv.org/abs/2202.13415`. They are uncitable by a gate and are listed first among the reading
  tasks.
- Figures entered as typed `figures`. Where the planning record names what was read without stating the value,
  the record's `value` is left empty and the first re-read fills it:

| Figure | Value | How read | Source |
|---|---|---|---|
| Designs explored by clustering similar morphologies, and simulation steps saved | about 780 times more designs; 78% fewer steps | P for these two figures | arXiv 2505.21665 |
| Wall-clock saved by sharing specialist policies across candidates | 67.4% (1/(1 − 0.674) = 3.07 times) | P, abstract | arXiv 2606.07424 |
| An accuracy figure quoted from the paper | 77.7% over 300 samples | S | arXiv 2305.07766 (ar5iv) |
| Two percentages quoted from the abstract | 46% and 75% | P, abstract | arXiv 2604.10351 |
| Robot and step counts | as in the paper | P for robot and step counts; F elsewhere | arXiv 2409.06366v4 |
| Step and device counts | as in the paper | P for step and device counts; F elsewhere | arXiv 2505.05753v2 |
| Hardware and sample counts | as in the paper | P for hardware and sample counts; S elsewhere | arXiv 2511.19204v3 |

- Licences: sources of the last topic are data and model collections that may be *used*, not only cited
  (MuJoCo Menagerie, Infinigen, RoboCasa, HSSD, HM3D). Their records carry `licence = "unknown"` and
  `licence_verified = false` at plan time; the asset-licence register is the authority, and a scene pack that
  joins an asset with an unverified licence is refused there. All other records carry `licence = "n/a"`.

**The sources as recorded at plan time** (dates: fetched or read on 2026-11-21 unless a line says otherwise).

**Training time, sampling control and agent-driven design loops.**

- https://arxiv.org/pdf/2109.11978 (Rudin et al.) [F].
- https://arxiv.org/html/2409.15610v1 [S]; `raw.githubusercontent.com/LeCAR-Lab/dial-mpc/main/dial_mpc/examples/` [P];
  https://arxiv.org/html/2511.19204v3 [P for hardware and sample counts; S elsewhere];
  https://ar5iv.labs.arxiv.org/html/2212.00541 [P]; https://arxiv.org/html/2409.10469v1 [S];
  https://arxiv.org/abs/2610.14064 [U].
- arXiv 2510.25850, versions 1 to 3 [P: the per-round wall-clock statement survives in version 3; the minutes and
  dollars figure is in version 2 only].
- https://arxiv.org/abs/1803.07055 [F].

**Family policies, inheritance, rank agreement and population co-design.**

- https://arxiv.org/html/2409.06366v4 [P for robot and step counts; F elsewhere]; https://arxiv.org/abs/2610.07597 [S];
  https://arxiv.org/html/2508.03660 [F]; https://ar5iv.labs.arxiv.org/html/2203.11931 [F];
  https://arxiv.org/html/2505.05753v2 [P for step and device counts; F elsewhere]; https://arxiv.org/html/2509.23745v1
  [F]; https://arxiv.org/html/2602.05791v3 [F]; https://arxiv.org/abs/2507.22653v2 [F];
  https://ar5iv.labs.arxiv.org/html/2207.11773 [F]; https://arxiv.org/html/2512.00971 [F];
  https://www.arxiv.org/abs/2510.08768 [F]; arXiv 2611.02462 [P, abstract].
- https://pmc.ncbi.nlm.nih.gov/articles/PMC11338828/ (loss of plasticity) [F]; https://arxiv.org/html/2604.15854 [F];
  https://arxiv.org/html/2309.13099 [F]; https://arxiv.org/html/2608.23100 [F].
- https://arxiv.org/html/2606.00702 [F]; https://arxiv.org/abs/2502.10862 [F];
  https://ar5iv.labs.arxiv.org/html/2003.04276 [F]; https://arxiv.org/html/2503.05696v4 [F];
  https://arxiv.org/abs/2210.02750 [F]; https://arxiv.org/html/2505.21665 [P for the 780-fold and 78% figures];
  https://arxiv.org/abs/2606.07424 [P, abstract: the 67.4% wall-clock figure]; https://arxiv.org/html/2411.16206v2 [F];
  https://arxiv.org/abs/2310.15418 [F].
- https://arxiv.org/html/2410.10674v1 (chaos in learned continuous control) [S].

**Symmetry, equivariance and geometric-algebra policies.**

- https://arxiv.org/abs/2402.15552 [F]; https://www.doi.org/10.1038/44416 [F]; a symmetry library's README [P].
- https://arxiv.org/html/2507.05695v4 [F]; https://ar5iv.labs.arxiv.org/html/2010.01856 [F];
  https://ar5iv.labs.arxiv.org/html/2408.06316 [F]; https://ar5iv.labs.arxiv.org/html/2305.18951 [F];
  https://arxiv.org/html/2410.23179v2 [F]; https://arxiv.org/html/2403.17320 [F]; https://arxiv.org/abs/2305.18415 [F];
  https://arxiv.org/html/2512.00727v1 [F].

**Gait orbits, continuation and certificates for control.**

- https://arxiv.org/pdf/1807.06614 (Hereid et al.) [F]; https://arxiv.org/pdf/1909.08124 (Ma, Hamed, Ames) [F];
  https://arxiv.org/pdf/2410.09512 (Raff, Flaßkamp, Remy) [F]; https://arxiv.org/pdf/1809.07279 (Gong et al.) [F];
  https://arxiv.org/pdf/1909.08560 (Ma and Ames) [F].
- https://groups.csail.mit.edu/robotics-center/public_papers/Majumdar14a.pdf [F]; https://arxiv.org/pdf/1010.2247 [F];
  https://arxiv.org/pdf/1601.04037 [F]; https://arxiv.org/pdf/2404.07956 [F]; https://arxiv.org/pdf/2006.09921 [F];
  https://arxiv.org/pdf/1909.09693 [F]; https://arxiv.org/pdf/2011.00032 [F]; https://arxiv.org/pdf/2109.09047 [F].

**Mission queries, co-design, synthesis and region certificates.**

- https://arxiv.org/abs/1512.08055 and https://arxiv.org/pdf/1512.08055 (monotone co-design) [F];
  https://arxiv.org/pdf/2011.10756 [F]; https://arxiv.org/pdf/2103.04660 [F];
  https://la.disneyresearch.com/wp-content/uploads/Joint-Optimization-of-Robot-Design-and-Motion-Parameters-using-the-Implicit-Function-Theorem-Paper.pdf
  [F, in part].
- https://arxiv.org/pdf/0705.1284 (Chablat, Wenger, Merlet) [F];
  https://ptolemy.berkeley.edu/projects/robotics/~ronf/PAPERS/mplecnik-idetc16.pdf (Plecnik and Fearing) [F];
  https://arxiv.org/abs/math/9803150 (linkage universality) [F].

**Studies, rare events, requirements and attribution.**

- arXiv 2603.13616 and arXiv 2503.10966 (anytime-valid and sequential comparison of robot policies) [P, full text].
- https://arxiv.org/html/2405.15771v2 [S]; https://ar5iv.labs.arxiv.org/html/2008.10581 [S];
  https://arxiv.org/abs/1811.00145 [S]; https://arxiv.org/pdf/2003.07739 [P: the quote was extracted from the PDF
  text]; https://arxiv.org/abs/1505.02674 [S].
- https://ar5iv.arxiv.org/html/2204.04483 (causal models of robot failures) [S].
- https://arxiv.org/html/2312.04528v2 [S]; https://ar5iv.arxiv.org/html/2201.09863 [S];
  https://ar5iv.labs.arxiv.org/html/2302.14545 [S].
- https://arxiv.org/abs/2510.00225 [S]; https://ar5iv.labs.arxiv.org/html/2110.00339 [S];
  https://ar5iv.labs.arxiv.org/html/2111.12679 [S]; https://ar5iv.labs.arxiv.org/html/2305.07766 [S: 77.7% accuracy
  over 300 samples]; https://arxiv.org/abs/2607.00442 [S].

**Reality, calibration, structure and living worlds.**

- https://arxiv.org/abs/2604.10351 [P, abstract: 46% and 75%]; https://arxiv.org/abs/2503.01255 [F];
  https://arxiv.org/abs/2508.02194 [F]; https://arxiv.org/abs/2509.06342 [F].
- https://arxiv.org/abs/2107.07511 and https://arxiv.org/html/2107.07511v6 (conformal prediction) [S];
  https://arxiv.org/abs/2510.04354 [S]; https://suresim-robot-eval.github.io [S]; https://arxiv.org/abs/2202.13415 [U].
- https://arxiv.org/html/2606.26643v1 [S]; https://www.atlantis-press.com/article/15203.pdf [P: the quote was
  extracted from the PDF text].
- https://arxiv.org/abs/2509.19105 [S].

**Estimation and gradients through contact.**

- https://arxiv.org/abs/1410.1465 [F]; https://arxiv.org/abs/2010.14666 [F]; https://arxiv.org/abs/2202.00817 [F];
  https://arxiv.org/abs/2109.05143 [F].

**Robot models and scene assets.**

- https://github.com/google-deepmind/mujoco_menagerie [F]; https://github.com/princeton-vl/infinigen [F];
  https://github.com/robocasa/robocasa [F]; https://huggingface.co/datasets/hssd/hssd-hab [F];
  https://aihabitat.org/datasets/hm3d/ [F]; two retail-scene generators (RoboBenchMart, MarketGen) [P, abstracts]. No
  URLs were recorded for the two generators.

## Acceptance criteria

- [ ] The register holds one record for each of the 98 URLs of these nine topics, one for each of the four bare
      arXiv identifiers, one for the scheme-less dial-mpc address, and one for each of the three named sources
      without an address: 106 records.
- [ ] Mark counts over the 102 brackets of these topics: 15 P, 63 F, 22 S, 2 U; over the 110 readings the
      register holds after the rule above: 18 P, 66 F, 24 S, 2 U.
- [ ] The two U sources are present, uncitable, and first in the reading-task report.
- [ ] `arXiv 2510.25850` has separate readings for versions 2 and 3 with what each covers.
- [ ] The seven figures of the table are typed; the 67.4% figure's derived ratio is recorded as derived.
- [ ] The five asset collections carry `licence_verified = false`.
- [ ] Every "what survives" figure of the hypothesis register that names a published source resolves to a
      record of this bead or of NAB-12 (tested where the hypothesis records are entered: N00-11's resolution
      check, which depends on this bead).

## Test plan

### Unit tests

`crates/frob-conformance/tests/sources_content.rs`, module `public_research`:

- `ninety_eight_urls`: the frozen URL list of this bead equals the set of locators of the nine topics with
  `locator_derived = false` and `unlocated = false`.
- `record_count_is_106`: 98 URL records, 4 derived arXiv records, 1 derived scheme-less record (the dial-mpc
  examples, one P reading), 3 unlocated records.
- `per_topic_url_counts`: 7, 25, 10, 13, 8, 14, 12, 4, 5.
- `mark_counts`: 110 readings, per topic 11, 28, 11, 13, 8, 16, 12, 4, 7, with marks 18 P, 66 F, 24 S, 2 U;
  the frozen bracket list gives 102 brackets with 15 P, 63 F, 22 S, 2 U.
- `shared_bracket_copies`: the abs and pdf records of 1512.08055 each hold one F reading; RoboBenchMart and
  MarketGen are two unlocated records, each with one P reading covering "abstract".
- `u_sources_are_two`; `u_source_is_never_citable` (with NAB-10's grading function).
- `versioned_readings_do_not_merge`: a citation `#minutes-and-dollars` resolves to version 2 only.
- `derived_ratio`: 1/(1 − 0.674) = 3.067…, stored as 3.07 with `how = "derived"`.
- `asset_licences_unverified`.
- `no_url_is_fabricated`: every URL locator appears verbatim in this bead's text, and each derived locator is
  built from an arXiv identifier or a scheme-less address that appears in it.

### End-to-end script and logging

`scripts/e2e/sources.sh` (NAB-16) prints the per-topic and per-mark counts and the reading tasks. Offline; no
URL is fetched. Logged per STD-E2E as `metric` lines with expected values.

### Known answers and negative controls

Known answers: 98 URLs; 106 records; 102 brackets (15 P, 63 F, 22 S, 2 U); 110 readings (18 P, 66 F, 24 S, 2 U); together
with NAB-12 the public register has 147 URLs, 163 brackets (39 P, 92 F, 30 S, 2 U) and 174 readings (43 P,
95 F, 34 S, 2 U), and no reading marked R or M. Negative controls: a hypothesis
range restated as a gate threshold with an F citation (NAB-10 must fail the gate once it records a value); a
U source given mark S without a `reread` entry (the frozen-mark comparison fails); the dollars figure cited
against version 3.

## Considerations

- A paper is not a measurement of this system. The register lets a hypothesis cite an unread source; it never
  lets a gate do so.
- Preprints are revised and renumbered; the version suffix in a locator is part of the record. Where the
  plan-time locator has no version, the first re-read records the version it found.
- The planning record's own limits apply: web sources were often read through a summarizing fetch, so quoted
  figures must be re-read from the primary source before a gate cites them. That task is owned by the re-read
  review and listed per gate by the citation lint.

## References

Appendix H.3 (topics five to thirteen) and H.1; §0.8 (the hypothesis register's sources).
