---
key: "N41-OPEN5"
title: "registries/open.toml, part 5: open questions of Augur, Nemesis and Autopsy (20 rows, one settled)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "augur", "nemesis", "autopsy", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-03"]
needs: ["§25 to §27: the decision and experiment beads named in the last column are owned by the chapter units", "§27.18: decision or experiment beads for OPEN-AUTOPSY-3 and OPEN-AUTOPSY-4, which block nothing and are carried in the considerations of the autopsy beads named in the table"]
related: ["N41-OPEN4", "N41-OPEN6", "N25d-45", "N25d-48", "N25d-49", "N26b-21", "N27b-13", "N40-35"]
gates: []
plan_refs: ["§42.3", "§42.2", "§25.28", "§26.17", "§27.18"]
---
## Background

Fifth part of the open-question register (rule and schema in N41-03; how to read a row in N41-OPEN1). The
subsystems:

- **Augur**: mission queries, which return for a family of designs and worlds an inner set (certified to succeed),
  an outer set, witnesses, exclusions and an unresolved remainder, with certificates checked by the offline
  auditor;
- **Nemesis**: adversarial search inside legality bounds, shrinking of failing worlds, rare-failure bounds;
- **Autopsy**: attribution of a failure by counterfactual forks with a placebo.

The owner's questions here: OPEN-AUGUR-4 (whether a quantifier-elimination or satisfiability engine is admitted as
a production dependency or only as a comparator; default comparator only, at the cost that some family cells stay
unresolved that a general solver would close; KILL-AUGUR-TIE depends on it), and the compute questions
OPEN-NEMESIS-3 (a certified failure rate of 10⁻⁶ exceeds the one-hour job rule; default certify to 10⁻⁵ and report
10⁻⁶ as `Estimated`) and OPEN-AUGUR-10 (the Moonshot slices; default none is funded before its pilot passes).

## Scope

In: the twenty rows below as records of `registries/open.toml`. Out: the schema and loader (N41-03); the decisions
(the beads in the last column); the chapter's own register and defaults-in-force battery for the Augur rows
(N25d-48), which reads the same records.

## Technical approach

| ID | Question | Fail-closed default | Owner | Blocks, or decided by | Deadline | Decided in |
|---|---|---|---|---|---|---|
| OPEN-AUGUR-1 | Whether mission-file predicates are written in the requirement language of task files or reference its terms by identifier | Reference by identifier, so that there is one grammar | Arena lead | GATE-AUGUR-01 | M1 | N25d-42 |
| OPEN-AUGUR-2 | Whether the linear programs are solved exactly over ℚ or in floating point with rational reconstruction | Floating point in the cone solver, basis reconstruction over ℚ, exact check; a failed reconstruction leaves the cell unresolved | Augur lead | GATE-AUGUR-02, from the reconstruction failure rate | M1 | N25d-43 |
| OPEN-AUGUR-3 | The measured sparsity of the constraint-to-parameter graph for the robot classes, and the coupled dimension that linkage closure adds | Region certificates are refused above six parameters | Augur lead | GATE-AUGUR-09 | M3 | N25b-08 (also N25e-27) |
| OPEN-AUGUR-4 | Whether a quantifier-elimination or satisfiability-modulo-theories engine is a production dependency or only a comparator outside the product | Comparator only; in-tree elimination for small linear systems | Owner | Before the M3 partial gate; KILL-AUGUR-TIE depends on it | M3 | N25d-45 |
| OPEN-AUGUR-5 | Sound reachable tubes for contact-rich full-order cells, which no sibling provides | Constructive inclusion is limited to static, quasi-static, finite-game and low-dimensional smooth cells; contact-rich cells stay unresolved with `Tested` trajectories | Augur lead | Revisited at M4 | M4 | N25d-44 |
| OPEN-AUGUR-6 | Where the standard-library-only checking code lives, and who writes bundles | Settled by architecture decision ADR-WS-02: `frob-cert` defines and reads the bundle, `frob-lemma` writes it, `frob-verbs` runs the check before an answer returns | none | Settled; tombstoned | closed | BD-026 and BD-027 carry the decision |
| OPEN-AUGUR-7 | The exit code of a mission query that stops on budget with certified cells | Exit 2 with `resolution-unmet`; the certified cells and the unresolved remainder are returned, each cell complete in itself | Tether lead | The registry entry of `mission.synthesize` at M1 | M1 | N25d-46 |
| OPEN-AUGUR-8 | Who reviews the primitive equations in a bundle, and whether that review is recorded in the bundle | Unreviewed bundles say so in their flags and cannot be cited under the Proof claim outside the plan's fixtures | Lemma lead | Citing a bundle under the Proof claim | M1 | N25b-02 |
| OPEN-AUGUR-9 | Whether version 1 of the mission file expresses several decision makers with separate histories | The field accepts one decision maker; more than one is refused until the fifth delivery slice | Augur lead | M8 | M8 | N25d-47 |
| OPEN-AUGUR-10 | The compute and calendar envelope for the Moonshot slices (facility co-design, shared-controller information, geometric-phase locomotion, symmetry at contact) | None is funded before its pilot passes | Owner | The M6 and M8 epics | M6 | N25d-49 |
| OPEN-AUGUR-11 | Whether a measure on unresolved sets is standardized (volume in a named chart) | No measure; the caller supplies one in `resolution`, and probability mass is reported only under a supplied weighting | Augur lead | Nothing | none | considerations of N25a-33; N25d-48 |
| OPEN-NEMESIS-1 | Sources for the legality bounds of interiors | Bounds are `assumed`, and counterexamples say so | Project lead, with OPEN-HAB-1 | Claims that a counterexample is realistic, not its validity | M3 | N26b-19 |
| OPEN-NEMESIS-2 | The distress score when the requirement margin is flat until shortly before failure | No splitting for that requirement; crude sampling | Nemesis lead | GATE-NEMESIS-05 | M3 | N26b-20 |
| OPEN-NEMESIS-3 | The compute envelope for a 10⁻⁶ claim, which exceeds the one-hour job rule | Certify to 10⁻⁵ and report 10⁻⁶ as `Estimated` | Project lead; asked of the owner at M3 | Any certified rate below 10⁻⁵ | M3 | N26b-21 |
| OPEN-NEMESIS-4 | The sample caps of 10⁶ in the siblings `fs-eproc` and `fs-uq`, and the absence of splitting upstream | Our own exact binomial bound for larger counts; upstream work items FD-FSIM-18 and FD-FSIM-19 are open and no milestone waits on them | Airlock lead | Nothing | none | N26b-22 |
| OPEN-NEMESIS-5 | How a hard-world set is carried across a change of robot class or task version | A set belongs to one class and one task version; worlds are re-promoted before they enter another | Arena lead | Nothing | M4 | N26b-23 |
| OPEN-AUTOPSY-1 | Default replicate count and perturbation sizes | 8 replicates at 10⁻¹², and 8 at 10⁻⁷ for interventions that change the outcome | Autopsy lead | The attribution pilot | M3 | N27b-13 |
| OPEN-AUTOPSY-2 | Whether one-at-a-time contrasts suffice, which would make pairs optional | Singles and pairs | Autopsy lead | GATE-AUTOPSY-03 | M3 | N27b-13 |
| OPEN-AUTOPSY-3 | Lyapunov exponents of our own controllers and policies on our tiers, which set the usable fork horizon | Leads of at most 2 s, with the growth of placebo separation reported in every attribution | Cortex lead | Measured at M3; blocks nothing | M3 | considerations of N27b-22 and N27b-23 |
| OPEN-AUTOPSY-4 | The size of the snapshot reservoir on each device | Sized by the budget estimator; every other fork point is reconstructed by replay | Chronicle lead | Nothing | none | considerations of N27b-27 |

Status at entry: nineteen `open`, one `closed` (OPEN-AUGUR-6, by ADR-WS-02; the identifier is retired).

## Acceptance criteria

- [ ] `registries/open.toml` contains the twenty records with no empty required field; OPEN-AUGUR-6 is tombstoned.
- [ ] The owner rows (OPEN-AUGUR-4, OPEN-AUGUR-10, and OPEN-NEMESIS-3 as asked of the owner) carry the cost of
      their defaults.
- [ ] Every blocking row names a decision or experiment bead that exists; the four non-blocking rows name the bead
      that carries them.
- [ ] The Augur chapter's own register battery and this registry read the same records (one file, no second
      copy).

## Test plan

### Unit tests
`crates/frob-conformance/tests/open_part5.rs`: `twenty_rows_load`; `ids_match_expected_set` (AUGUR-1 to -11,
NEMESIS-1 to -5, AUTOPSY-1 to -4); `augur_6_is_tombstoned`; `decision_beads_resolve`; `owner_rows`;
`defaults_quote_known_numbers` (six parameters; exit 2; 10⁻⁵ and 10⁻⁶; 10⁶ caps; 8 replicates at 10⁻¹² and 10⁻⁷;
2 s).

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04).

### Known answers and negative controls
Known answers: 20 rows, 1 closed. Negative controls: a certified failure rate of 10⁻⁶ recorded while
OPEN-NEMESIS-3 is at its default; a region certificate over seven parameters while OPEN-AUGUR-3 is at its default;
a second question under OPEN-AUGUR-6.

## Considerations

- OPEN-AUGUR-4 decides whether the product can ever close cells that only a general solver closes; the comparator
  arm exists either way, which is what lets the tie rule be evaluated.
- OPEN-AUGUR-8's default is deliberately uncomfortable: an unreviewed bundle carries a flag that prevents its
  citation as proof, so the review question cannot be forgotten.

## References

§42.3 (rows for Augur, Nemesis, Autopsy), §42.2 (owner rows), §42.4 (closed questions).
