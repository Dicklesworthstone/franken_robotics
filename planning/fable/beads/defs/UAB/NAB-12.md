---
key: "NAB-12"
title: "registries/sources.toml: public sources on incumbent engines, GPU toolchains, arithmetic and hardware"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tenfold", "gauntlet", "delivery", "solid", "registry"]
parent: "WP-34"
deps: ["NAB-09"]
needs: []
related: ["NAB-13", "NAB-14"]
gates: []
plan_refs: ["Appendix H.3", "Appendix H.1", "§1.1", "§36.3"]
---
## Background

The claims of FrankenRobotics are comparative: throughput is a parity requirement against incumbent GPU
simulators, training time is a ratio to an incumbent measured on the same card, and the ten-fold test compares
the time to an accepted design decision with the strongest rival a competent engineer can compose from
existing systems. Those comparisons rest on what the incumbents document and publish. At plan time the
incumbents were **read, not run**: MuJoCo and MuJoCo Warp at 4.0.1, MJX, MuJoCo Playground, mjlab, Newton at
1.8.0, Warp at 1.19.0, Isaac Lab at 3.0.0 with Isaac Sim 6.2.0, Genesis at 1.4.7, ManiSkill at 3.0.1 and Drake
at 1.58.0. The GPU toolchain facts (CUDA bindings, Metal behaviour, single-precision rounding, memory
bandwidth) and the arithmetic references (three-precision iterative refinement) were read the same way.

This bead enters those sources into the source register (`registries/sources.toml`, schema `frob.sources/1`,
NAB-09), each with the mark that says how its text reached the reader: **P** read from the primary text, **S**
through a summarizing fetch, **F** fetched with no record of which (treated as S), **R** a search summary only,
**M** recalled, **U** no mark recorded. Only a P reading may be cited by a gate without a further reading, so
every S and F reading below is a reading task for the first gate that cites it (the citation lint of NAB-10
lists them). Tier: Solid.

## Scope

In: the public-source records of four topics: (1) MuJoCo, MJX, MuJoCo Warp, Playground, mjlab; (2) Newton,
Warp, Isaac Lab, Isaac Sim; (3) Genesis, ManiSkill, Drake; (4) GPU toolchains, arithmetic and hardware. Out: the
other nine topics (NAB-13); the schema (NAB-09); the lint (NAB-10); the re-reads (N00-34); executing incumbents
(BD-039, BD-040).

## Technical approach

**Transcription rule.**

1. One `[[source]]` per URL or per named document, `kind = "public"`, `topic` = one of the four group names,
   `read_on = "2026-11-21"` unless the text gives another date.
2. Each bracket after a source becomes one `reading`: the letter is the `mark`; text after a colon or after
   "at" is `covers` and `version` ("P at 4.0.1: the 3.7.0 entry on electrical and thermal actuators" →
   `mark = "P"`, `version = "4.0.1"`, `covers = "the 3.7.0 entry on electrical and thermal actuators"`).
   "P for X; S elsewhere" becomes two readings (P covering X, S covering "elsewhere"); "P; also read at tag
   v1.8.0 with …" becomes two P readings, the second with `version = "1.8.0"`. In these four topics that rule
   applies three times: the Genesis README, the mixed-batch paper and the Newton determinism page, so the 61
   brackets become 64 readings.
3. A source named without a URL (a page of a versioned documentation set, a file at a release tag, a support
   page by number) has `locator = ""`, `unlocated = true` and a `title` that names it exactly; the first
   re-read records its URL. At plan time the text says explicitly "no URL was recorded" for three: the MuJoCo
   Warp nightly benchmark page; the Isaac Sim 6.2.0 documentation pages (Warehouse Creator, agent interface)
   with the community server's package description; and the two retail-scene generators of NAB-13. A phrase
   that names several parts of one versioned documentation set or release without URLs is one record titled
   by the whole phrase, with one reading (the Isaac Sim 6.2.0 Warehouse Creator and agent-interface pages; the
   Isaac Lab 3.0.0 release notes, benchmark table and G1 configuration; the Newton 1.8.0 solver class
   documentation and test suite), so these topics hold no shared-bracket copies (contrast NAB-13 rule (c)).
4. `licence = "n/a"` for a document that is only cited for a fact. The licence of an incumbent *package* that
   the incumbent harness installs is recorded by that harness with its pinned revision, not here.
5. Figures that gates or the incumbent capability register quote are entered as typed `figures` on their
   reading, with hardware and how they were read:

| Figure | Value | Hardware or setting | How read | Source |
|---|---|---|---|---|
| Humanoid (G1) physics throughput | 2.55 M world-steps/s at 8,192 worlds | RTX 6000 Ada | raw text (P) | MuJoCo Warp nightly benchmark (no URL recorded) |
| Memory per humanoid world | 88.4 kB per G1 world | — | raw text (P) | the same |
| Quadruped joystick training time | about 410 s on one RTX 4090 (± 20 s) | RTX 4090 | figure read by eye from a rendered plot | the Playground paper, Fig. 13 |
| mjlab nightly figures | at 4,096 environments | RTX 5090 | raw page (P), updated 2026-11-21 | mjlab nightly |
| Event trace of a humanoid step | 262 ns per world-step at 8,192 worlds: 53 ns in recursion stages, 124 ns in the solve, 38 ns in constraint construction | — | fetched (F) | MuJoCo Warp pull request 1119 |
| Cold and cached kernel compile times | about 96 s cold; under 1 s cached | — | fetched (F) | MuJoCo Warp pull request 989 |
| CPU rates of the classic humanoid | 650 k steps/s on an M3 Max; 1.8 M steps/s on a 64-core 3995WX | CPU | raw text (P) | the MJX documentation page |
| Mixed batch of three body plans | Table A.5 | — | raw text (P) for the abstract and Table A.5; S elsewhere | arXiv 2603.16536v1 |
| A Newton pull request | closed unmerged on 2026-11-02 | — | fetched (F) | Newton pull request 4312 |
| Memory bandwidth of this chip configuration | as stated on the page | Apple M5 family | raw text (P) | Apple support page 128107 |
| Metal single-precision arithmetic | no control of rounding or subnormal handling | — | summary (S) | an Apple developer forum thread |

6. Every reading that is not P has `reread_before_gate = true`. The by-eye figure keeps its uncertainty and may
   never be cited without it.

**The sources as recorded at plan time** (dates: fetched or read on 2026-11-21 unless a line says otherwise; a
source that two records marked differently is listed once, with each mark and what it covers).

**Incumbent engines: MuJoCo, MJX, MuJoCo Warp, Playground, mjlab.**

- https://mujoco.readthedocs.io/en/latest/changelog.html [P at 4.0.1: the 3.7.0 entry on electrical and thermal
  actuators]; the MuJoCo XML reference at 4.0.1 [P: the energy-flag sentence and five sensors]; the model-editing page
  [P].
- https://mujoco.readthedocs.io/en/latest/mjx.html [P: CPU rates on an M3 Max and a 64-core 3995WX];
  https://raw.githubusercontent.com/google-deepmind/mujoco/main/doc/mjx.rst [F].
- https://github.com/google-deepmind/mujoco_warp [F]; https://mujoco.readthedocs.io/en/latest/mjwarp/index.html [F];
  https://mujoco.readthedocs.io/en/latest/_sources/mjwarp/index.rst.txt [P: batched model fields, per-world assets, the
  determinism answer, the 60-degree-of-freedom note]; https://mujoco.readthedocs.io/en/latest/mjwarp/ [F].
- https://github.com/google-deepmind/mujoco_warp/pull/1119 [F: event trace, humanoid, 8,192 worlds];
  https://github.com/google-deepmind/mujoco_warp/pull/1014 [F]; https://github.com/google-deepmind/mujoco_warp/pull/989
  [F: cold and cached compile times]; https://github.com/google-deepmind/mujoco_warp/issues/562 [S].
- MuJoCo Warp 4.0.1 source at its release tag [P: `DynType.DCMOTOR` with a temperature state]; the MuJoCo Warp nightly
  benchmark [P: 2.55 M G1 world-steps/s at 8,192 worlds on an RTX 6000 Ada; 88.4 kB per G1 world]. No URL was recorded
  for the benchmark page.
- https://arxiv.org/abs/2502.08844 (MuJoCo Playground) [P: three sentences from the PDF text; Fig. 13 rendered and read
  by eye, about ±20 s]; https://arxiv.org/html/2502.08844 [S].
- https://mujocolab.github.io/mjlab/nightly/ [P: raw page, updated 2026-11-21; RTX 5090, 4,096 environments].

**Incumbent engines: Newton, Warp, Isaac Lab, Isaac Sim.**

- https://github.com/newton-physics/newton [F]; https://newton-physics.github.io/newton/stable/solvers/index.html [F];
  https://newton-physics.github.io/newton/latest/concepts/determinism.html [P; also read at tag v1.8.0 with its
  per-solver table]; https://newton-physics.github.io/newton/latest/concepts/worlds.html [S];
  https://newton-physics.github.io/newton/1.2.0/api/newton_solvers.html [S];
  https://github.com/newton-physics/newton/pull/4312 [F: closed unmerged on 2026-11-02]; the Newton 1.8.0 solver class
  documentation and test suite at the tag [P].
- https://arxiv.org/html/2603.16536v1 (the mixed-batch solver paper) [P for its abstract and Table A.5; S elsewhere].
- https://nvidia.github.io/warp/latest/user_guide/execution_and_performance/deterministic_execution.html [F]; Warp
  1.19.0 at its tag [P].
- https://arxiv.org/abs/2511.04831 (Isaac Lab) [F]; https://docs.nvidia.com/isaaclab/isaac-lab/reproducibility-and-determinism.md
  [P]; https://docs.nvidia.com/isaaclab/isaac-lab/reproducibility-and-determinism [S];
  https://docs.nvidia.com/isaaclab/isaac-lab/spawning-multiple-assets.md [P];
  https://isaac-sim.github.io/IsaacLab/main/source/how-to/multi_asset_spawning.html [F]; the Isaac Lab 3.0.0 release
  notes, benchmark table and G1 configuration at the tag [P].
- Isaac Sim 6.2.0 documentation: the Warehouse Creator page and the agent-interface page [P]; a community server's
  package description [P, searched for budget, quota, provenance and refusal terms]. No URLs were recorded.

**Incumbent engines: Genesis, ManiSkill, Drake.**

- https://github.com/Genesis-Embodied-AI/genesis-world [F];
  https://raw.githubusercontent.com/Genesis-Embodied-AI/Genesis/main/README.md [P at 1.4.7 for three README sentences;
  S elsewhere]; https://stoneztao.substack.com/p/the-new-hyped-genesis-simulator-is [F].
- https://arxiv.org/html/2410.00425v2 (ManiSkill3) [S]; `advanced.md` at v3.0.1 [P].
- https://drake.mit.edu/release_notes/release_notes.html [F]; https://arxiv.org/abs/2110.10107 [F]; a Drake header at
  v1.58.0 [P].

**GPU toolchains, arithmetic and hardware.**

- https://docs.rs/crate/cudarc/latest [F];
  https://developer.nvidia.com/blog/introducing-cuda-rust-two-tracks-for-writing-gpu-kernels/ [F].
- https://developer.apple.com/videos/play/wwdc2025/205/ [F]; https://developer.apple.com/forums/thread/844598 [S: no
  control of single-precision rounding or subnormal handling]; https://developer.apple.com/documentation/metal/mtlstoragemode/shared.md
  [P]; https://docs.rs/objc2-metal [F].
- https://www.apple.com/newsroom/2025/10/apple-unleashes-m5-the-next-big-leap-in-ai-performance-for-apple-silicon/ [F];
  Apple support page 128107 [P: the memory-bandwidth figure for this chip configuration].
- https://github.com/gfx-rs/wgpu/releases [F]; https://github.com/shader-slang/slang [F].
- https://eprints.maths.manchester.ac.uk/2629/1/cahi18.pdf (Carson and Higham, three-precision refinement) [P];
  https://pmc.ncbi.nlm.nih.gov/articles/PMC7735315/ [S].
- https://www.roboticsproceedings.org/rss14/p38.pdf (Carpentier and Mansard) [F];
  https://people.csail.mit.edu/devadas/pubs/iros2019.pdf (Neuman et al.) [F]; https://arxiv.org/pdf/2109.06976 [F];
  https://arxiv.org/pdf/1709.03799 (Giftthaler et al.) [F]; https://arxiv.org/pdf/2006.03785 (Rosa and Lynch) [F].

## Acceptance criteria

- [ ] The register holds one record for each of the 49 URLs of these four topics and one for each document
      named without a URL, each with its readings and marks exactly as above.
- [ ] Mark counts over the 61 brackets of these topics: 24 P, 29 F, 8 S, no R, M or U; over the 64 readings the
      register holds after the split rule: 25 P, 29 F, 10 S.
- [ ] The eleven figures of the table are typed `figures` with hardware and `how`; the 410 s figure has
      `how = "figure-by-eye"` and `uncertainty = "± 20 s"`.
- [ ] Every unlocated source has `unlocated = true` and an exact title; none has an invented URL.
- [ ] Every non-P reading reports `reread_before_gate = true` through NAB-09's loader, and every reading's mark
      equals the frozen bracket list of this bead (`marks_match_frozen_list`). (Listing each such reading under
      the earliest gate that cites it is NAB-10's reading-task report, which comes after this bead.)
- [ ] Every record of these topics has an `id` of the form `SRC-PUB-nnn`, so that the source keys of the
      incumbent capability register resolve to them. (That every key of that register resolves is the test
      `sources_resolve` of N00-13, which depends on this bead; it is not this bead's close condition.)

## Test plan

### Unit tests

`crates/frob-conformance/tests/sources_content.rs`, module `public_engines`:

- `forty_nine_urls`: the frozen URL list of this bead equals the set of non-empty `locator` values of the four
  topics.
- `mark_counts`: readings of the four topics number 64 (18, 20, 9 and 17 per topic) with marks 25 P, 29 F,
  10 S; the frozen bracket list of this bead gives 61 brackets with 24 P, 29 F, 8 S.
- `split_marks_are_two_readings`: the Genesis README (P at 1.4.7 for three README sentences; S elsewhere), the
  mixed-batch paper (P for its abstract and Table A.5; S elsewhere) and the Newton determinism page (P; also read
  at tag v1.8.0 with its per-solver table) each have the readings the text states.
- `pinned_versions`: 4.0.1 (MuJoCo, MuJoCo Warp), 1.8.0 and 1.2.0 (Newton), 1.19.0 (Warp), 3.0.0 (Isaac Lab),
  6.2.0 (Isaac Sim), 1.4.7 (Genesis), 3.0.1 (ManiSkill), 1.58.0 (Drake) appear as `version` values.
- `by_eye_figure_has_uncertainty`; `benchmark_page_is_unlocated`.
- `figure_values`: 2.55 M world-steps/s at 8,192 worlds; 88.4 kB; 262 ns; about 96 s.
- `marks_match_frozen_list`: each reading of the four topics has the mark of its bracket in the frozen
  plan-time list (`tests/fixtures/sources/public_engines_marks.tsv`); a stronger mark is accepted only when the
  reading has a `reread` entry with `result = "confirmed"`. This is the frozen-mark comparison that NAB-16's
  plant `mark-raised` must trip.
- `no_url_is_fabricated`: every `locator` that is a URL appears verbatim in this bead's text.

### End-to-end script and logging

`scripts/e2e/sources.sh` (NAB-16) prints the per-topic and per-mark counts and the list of reading tasks for
these topics. It does not fetch any URL (the script is offline; re-reading is the work of N00-34). Logged per
STD-E2E: counts as `metric` lines with expected values; the digest of the register.

### Known answers and negative controls

Known answers: 49 URLs (14, 13, 6 and 16 per topic); 61 brackets (18, 18, 8 and 17 per topic) with marks
24 P, 29 F, 8 S; 64 readings after the split rule (18, 20, 9 and 17) with marks 25 P, 29 F, 10 S. Arithmetic pins on the figures: 262 ns per world-step is 1 ÷ 262 ns = 3.82 M
world-steps/s for that trace, of which the stages a design-specialized recursion could touch are 53 ns (20%),
the solve 124 ns (47%) and constraint construction 38 ns; 8,192 worlds × 88.4 kB = 724 MB for the humanoid
batch. The test `figures_have_units` requires a non-empty `unit` on every figure whose value is a number, and a
non-empty `hardware` on every figure (`"n/a"` where the table prints "—"); a fixture figure with a number and
no unit is refused. Negative
controls: a record whose mark was raised from F to P without a `reread` entry (must fail
`marks_match_frozen_list`); a fabricated URL for the benchmark page (must fail `no_url_is_fabricated`); the 410 s
figure stored without its uncertainty (must fail `by_eye_figure_has_uncertainty`). A gate that cites the 410 s
figure without "± 20 s" is refused by NAB-10's `cite.by-eye-without-uncertainty`, which comes after this bead;
that control is NAB-16's plant `by-eye`.

## Considerations

- Incumbent versions move quickly. A record is a statement about a pin; a new version is a new reading
  appended, never an edit.
- "Not found" is not "does not exist": several incumbent capabilities were searched for and not found; those
  are in the documented-absences list (N00-14), not here.
- No absolute incumbent throughput on an RTX 4090 was read from any source; the published figures are for an
  RTX 6000 Ada and an RTX 5090. The parity baseline is measured on this project's own hardware (GATE-SPARK-21);
  the not-verified list (NAB-14) records the gap.
- Security: the register holds URLs; nothing fetches them automatically in a build or a test.

## References

Appendix H.3 (first four topics) and H.1; §1.1 (the incumbent audit); §36.3 (targets as ratios to the measured
incumbent).
