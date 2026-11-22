---
key: "NAB-14"
title: "registries/sources.toml: standard references cited from memory, and the list of what was not verified"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["NAB-09"]
needs: ["Appendix K.5: the audit of the eighteen planning claims that were never audited (N00-31)", "§21.16, §21.26: the hardware fingerprint and its rate probes, which measure the unverified hardware items (BD-029, N21c-01)", "Appendix G, §19.8, §19.12: the hazard registries and the retrofit catalogue, whose dimensions and prices are `assumed` (BD-227, BD-369)"]
related: ["NAB-10", "NAB-12", "NAB-13", "NAB-16", "N00-31", "N00-34", "N00-35", "BD-017", "BD-029", "BD-033", "BD-039", "BD-142", "BD-311", "BD-380", "BD-399"]
gates: []
plan_refs: ["Appendix H.4", "Appendix H.6", "§0.1", "§0.7"]
---
## Background

Two parts of the planning record of FrankenRobotics are honest admissions, and a system that claims to be
honest about evidence has to carry them as data.

**Standard references cited from background knowledge and not re-fetched.** The mathematics of the system
(spatial-algebra rigid-body dynamics, Lie-group integrators, convex contact, invariant filtering, max-plus gait
timing, interval methods, α-theory) rests on textbooks and classic papers. They were cited from memory. Each
MUST be confirmed against its primary source before a gate cites it. In the register's vocabulary they carry
mark **M** (recalled, not re-read).

**What was not verified, and was not relied on.** Thirteen kinds of statement were explicitly left unverified
at plan time: absolute incumbent throughput on the hardware this project owns, several properties of that
hardware, the licences of some data sets, every hazard dimension and retrofit price, the unexecuted parts of
five reported foundation defects, any result of the proof checker, and any statement about a physical robot.
The specification does not depend on them. The danger is that a later contract, gate or report quietly starts
to. The register therefore holds them as `[[unverified]]` records, each naming the gate, open question or bead
that will measure or read it, and the citation lint (NAB-10) fails any reliance on one before then.

This bead enters both parts into `registries/sources.toml` (schema `frob.sources/1`, NAB-09). Tier: Solid.

## Scope

In: the `SRC-REF-…` records (43 listed items in five groups) and the thirteen `[[unverified]]` records with
their measuring gates. Out: confirming the references (each consuming bead does it before its gate cites one;
the theorem-card and specification beads own most); measuring the unverified items (the beads named in the
table); the lint (NAB-10).

## Technical approach

**1. Standard references.** One `[[source]]` per listed item, `kind = "reference"`, `locator = ""`,
`unlocated = true`, one `reading` with `mark = "M"` and `covers` = what the specification uses it for, and
`group` = one of: `carried` (the 18 references of the first design), `mission-and-synthesis` (14),
`numerics` (4), `scene-and-sensing` (6), `control` (1). `licence = "n/a"`. A confirmation is a `reread` entry
with the edition, chapter or equation number, and `result = "confirmed"`; the citation lint treats an M reading
without one as uncitable by a gate.

Two notes are carried on records. The published technical specification for collaborative robots that states
transient contact limits through transferred energy is named **as context only**: no conformity is assessed
against it, and the energy-budget safety tier must never be described as conforming to it. The solution counts
(the nine-point four-bar problem, Stewart–Gough forward kinematics, the general 6R inverse kinematics) are
numbers that known-answer tests pin; each is confirmed against its primary source before the test's gate is
recorded.

The references as listed at plan time:

- Carried: Featherstone, *Rigid Body Dynamics Algorithms*; Park, Bobrow and Ploen on Lie-group robot dynamics; Murray,
  Li and Sastry; Lynch and Park; Hervé on displacement subgroups; Iserles, Munthe-Kaas, Nørsett and Zanna on Lie-group
  methods; Marsden and West on variational integrators; Lee, Leok and McClamroch; Wensing, Kim and Slotine on
  physically consistent inertia; Todorov on convex contact; Bloch, Krishnaprasad, Marsden and Murray on nonholonomic
  systems; Hatton and Choset; Buono and Golubitsky; Lopes et al. on max-plus gait models; Hartley et al. on
  contact-aided invariant filtering; Bullo and Lewis; Lee, Liu, Park and Srinivasa on linear-time variational
  integrators; Kobilarov, Crane and Desbrun on Cayley-based Lie-group integrators.
- Added by the mission and synthesis chapters: Bretl and Lall on static equilibrium with friction; Orsolino et al. on
  feasible regions; Fadini et al. on co-design; the Flow* reachability tool; the Scenic scenario language;
  decision-region determination; ASID; saltation matrices; contact-size dependence in gel indentation; Merlet, and
  Neumaier, on interval methods in design; the nine-point, Stewart–Gough and 6R solution counts; Burmester theory;
  Smale's α-theory.
- Added by the numerics chapters: Demmel, van der Sluis and Higham on scaling; Sterbenz's lemma; Schwartz–Zippel
  identity testing; the Clopper–Pearson interval.
- Added by the scene and sensing chapters: the squeeze-film formula; viscous-spreading exponents; linear sloshing
  frequencies; the right-angle corridor result; small-time local controllability of car-like bases; the glTF camera
  convention.
- Added by the control chapter: a published technical specification for collaborative robots that states transient
  contact limits through transferred energy. It is named as context only. No conformity is assessed against it (§22.11).

**2. The not-verified list.** `[[unverified]]` records in the same file:

```toml
[[unverified]]
id          = "NV-01"
statement   = "Absolute MuJoCo Warp or Newton throughput on an RTX 4090"
why         = "The published absolute figures are for an RTX 6000 Ada and an RTX 5090"
measured_by = ["GATE-SPARK-21"]          # gates, open questions or beads that settle it
status      = "unverified"               # unverified | measured | read | withdrawn
record      = ""                         # gate record or source id once settled
```

| Id | Not verified, and not relied on | Settled by |
|---|---|---|
| NV-01 | Absolute MuJoCo Warp or Newton throughput on an RTX 4090. The published absolute figures are for an RTX 6000 Ada and an RTX 5090 | The parity baseline measured on host `trj` at M0 and M1 (GATE-SPARK-21; BD-039, BD-040) |
| NV-02 | The RTX 4090's double-precision ratio from a primary source, its memory bandwidth, and peer access between the two cards | The open question OPEN-SPARK-6; the hardware fingerprint and its rate probes (BD-029, N21c-01) |
| NV-03 | Whether Metal's single-precision division and square root are correctly rounded and whether subnormals are flushed. 64-bit integer atomics are reported absent on Metal | GATE-SPARK-01 and GATE-SPARK-07 measure the first two (BD-017, BD-029); the atomics are re-run by the fingerprint (GATE-SPARK-23) |
| NV-04 | Bandwidth, launch costs and compile times of both machines. Only core counts, memory sizes and the two Metal limits are recorded | The fingerprint rate probes (N21c-01) |
| NV-05 | Rigid-body simulation by an incumbent on Apple silicon at stated throughput. A search found no benchmark | The parity baseline on this Mac (BD-142) |
| NV-06 | The reference-robot figures used as published anchors for Parity at M3. They are at search-summary level (mark R) | The published-anchor comparison (BD-311), after the re-read review (N00-34) |
| NV-07 | Licences of ProcTHOR-10K data, Replica and YCB, and of the retail and warehouse generators | The asset-licence register and asset-populated interiors (N19b-02, BD-399) |
| NV-08 | Ratification status of glTF physics extensions | The import and export work; until read, no importer claims conformance to an extension |
| NV-09 | Every hazard dimension of the hazard registries and every retrofit price. No norm, regulation, standard or price list was read | Each value stays `assumed` with its provenance field (BD-227, BD-369; OPEN-HAB-1, OPEN-HAB-4) |
| NV-10 | The reported numbers of the five foundation defects beyond what reading the source confirms: the ratio 1.582459, the zero-translation run and the wrong in-bounds read were not re-executed | The foundation-defect admission tests (BD-033, BD-034, BD-035, BD-036) |
| NV-11 | Sources marked M or U, and the 18 claims the second audit left pending | The audit of the eighteen claims (N00-31) and the re-read review (N00-34) |
| NV-12 | Any Lean result. No theorem card has been checked; the proof checker's module status is taken from its own receipt | The proof-checker admission and the first sealed capsules (BD-380, BD-381) |
| NV-13 | Any statement about a physical robot | The Reality lane, which is an import lane resting at `no-claim` until data is admitted (BD-387, BD-397) |

A record's `status` changes only with a `record`: a gate record (`measured`), a source id with a P reading or a
confirmed re-read (`read`), or an ADR (`withdrawn`). The citation lint reads `status`.

**3. Mark note (a finding of the conversion).** The mark table defines R and M, and NV-11 speaks of "sources
marked M or U" among the public sources; yet no public-source reading carries R or M (the public register has
39 P, 92 F, 30 S and 2 U). The recalled sources are the standard references of this bead, which is why they
are entered with mark M, and the search-summary figures are those of NV-06, entered with mark R when the
published-anchor bead registers them.

## Acceptance criteria

- [ ] 43 `SRC-REF-…` records exist in five groups (18, 14, 4, 6, 1), each with mark M and `unlocated = true`.
- [ ] The collaborative-robot specification's record carries `context_only = true`, and a test fails if any
      contract, report template or verb description contains "conform" within the same sentence as its title.
- [ ] 13 `[[unverified]]` records exist with the statements and settling gates of the table; each `measured_by`
      identifier exists in the gate register, the open-question list or the bead graph.
- [ ] With every record at `status = "unverified"`, the citation lint fails a planted contract that states NV-01
      as fact, and passes it after GATE-SPARK-21 has a recorded value and the record is set to `measured`.
- [ ] No reference is cited by a recorded gate without a `reread` entry naming edition and location.

## Test plan

### Unit tests

`crates/frob-conformance/tests/sources_content.rs`, module `references_and_unverified`:

- `forty_three_references_in_five_groups`; `all_references_have_mark_m`.
- `context_only_reference_is_flagged`; `conformity_wording_is_refused` (a fixture sentence "conforms to the
  collaborative-robot specification" fails).
- `thirteen_unverified`; `each_unverified_names_a_settler`; `settlers_resolve` (gate ids against
  `registries/gates.toml`, open questions against the open-question list).
- `status_change_requires_record`: `status = "measured"` with empty `record` is refused.
- `nv06_anchor_figures_are_mark_r` (once BD-311 registers them).
- `public_register_has_no_r_or_m_reading` (the finding of step 3, pinned so a later change is noticed).

### End-to-end script and logging

`scripts/e2e/sources.sh` (NAB-16) prints the unverified list with statuses and runs the planted reliance cases.
Logged per STD-E2E: per record a line with id, status and settler; counts by status as `metric` lines.

### Known answers and negative controls

Known answers: 43 references (18 + 14 + 4 + 6 + 1); 13 unverified items; at plan time all 13 have status
`unverified`. The α-test constant quoted with Smale's α-theory elsewhere in the specification is
(13 − 3√17)/4 ≈ 0.157671; the reference record stores the constant so that its confirmation has something to
compare. Negative controls: a gate threshold that quotes an absolute RTX 4090 incumbent figure
(`cite.relies-on-unverified`); a hazard dimension whose provenance was changed from `assumed` to `cited` with
no source id; a report template that claims a Lean theorem "checked" while NV-12 is `unverified`.

## Considerations

- The list is what makes the honest-status table honest: a status line may say "not verified" and point here.
- A standard reference being famous does not make a recalled equation or sign convention right. Confirmation is
  cheap at the bead that first uses it, and expensive after three crates have copied the recalled form.
- Revisit the list at every milestone exit (the sweeps NAB-23 to NAB-31): an item settled by a gate moves to
  `measured`; a new unverified reliance found by a review is appended as `NV-14` onward.

## References

Appendix H.4 and H.6; §0.1 (labels and the three rules for outside numbers); §0.7 (a gate needs only what a
simulator has).
