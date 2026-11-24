---
key: "VERBS-mission"
title: "frob-verbs registry: cluster `mission`, nine rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "augur", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: ["BD-133", "N12-REF", "N19-REF", "N24-REF", "N25-REF", "N25e-45", "N28-REF", "N28c-29", "N34-24", "N35x-02"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§25.3", "§25.3.1", "§25.3.2", "§25.8", "§25.9", "§25.11", "§25.14", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `mission`. The cluster covers mission queries and inverse
design: a quantified statement about a design family, a world family, an observation channel and a policy class,
answered with inner and outer design sets, witnesses, exclusions, unresolved cells and a certificate bundle. It has
nine verbs (2 `read`, 6 `run`, 1 `mutate`); the handlers live in `frob-mission`, `frob-spec`. The bead carries every
row completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match the
list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

Kinds: `read` returns stored or cheaply computed facts and changes nothing; `run` spends compute, takes an
idempotency key and a budget, and after a bounded synchronous wait returns a job handle with exit code 8; `mutate`
changes a named object in the caller's namespace and takes an idempotency key and an expected parent; `admin`
cancels, stops, evicts or deletes and needs a capability scope. Tiers: S Solid, F Frontier (with a pilot and a kill
gate), M Moonshot; `S/F` is a Solid core with Frontier extensions.

DESIGN: a registry bead per cluster exists beside the implementing beads because a row missing from the registry,
spelled twice, or served with different refusal lists on the CLI and over MCP is a surface defect that no mechanism
test catches. Rejected: rows left to the implementing beads alone (nothing checks their union against the
inventory); one bead for all the rows (too large to review).

## Scope

In: the nine registry entries of cluster `mission`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `mission.distinguish` | run | M4 | F | The smallest admissible intervention or instrument that separates alternatives | §25.14 |
| `mission.refute` | run | M1 | F | A scoped impossibility bound, or a witness defeating one design and controller; the smallest relaxing change | §25.3.1, §25.9 |
| `mission.revalidate` | run | M4 | F | Surviving certificates, invalidated regions, reruns and their cost after an edit (one engine with `result.revalidate`) | §25.8 |
| `mission.separate` | run | M4 | F | A minimized, admissible pair of worlds in which a design ordering or a diagnosis changes | §25.9 |
| `mission.show` | read | M1 | S | A stored answer by set (`inner`, `outer`, `excluded`, `unresolved`), ordered and paged | §32.6 |
| `mission.synthesize` | run | M1 | F | Inner and outer design sets with witnesses, exclusions, unresolved cells and cost bounds | §25.3 |
| `mission.transport` | run | M8 | F/M | Move a certified claim through a map between task interfaces with a bounded error, or return a counterexample naming the lost mode [F]; static core at M4. A skill artifact with a learned residual is `skill.transport` | §25.3.1, §25.11 |
| `mission.new` | mutate | M1 | S | Create a mission query from a document or a template | §25.3.2, §32.6 |
| `mission.check` | read | M1 | S | Admit a mission query: refuse an empty or inconsistent domain and echo the quantifier order | §25.3.2, §32.6 |

**Forms that answer before the row's milestone.** `first_milestone` is the value the verb appendix prints, for
every row (the rule BD-137, NJL-01 and every registry bead share: the registry's per-milestone verb counts are
known answers pinned to the appendix). What a build serves is decided by the handler: a form that lands before
the row's milestone answers from then on, and `frob capabilities --json` lists it as implemented with its
restriction named; a form that has not landed answers `FROB-REF-724`, exit 5. Four verbs of this cluster answer
early (requests of round 1 from U25a, U25b and U25c, checked against the delivering beads and the gate register):

| Verb | Row's milestone | Answers from | Restriction named by `frob capabilities` | Complete at |
|---|---|---|---|---|
| `mission.distinguish` | M4 | M1 (BD-135: enumerated worlds and the alias witness) | `form: "enumerated-worlds"` | M3 probe design by decision partition (BD-291), driven through the public verb by the pilot rows of GATE-AUGUR-11 (N25c-13) for KILL-AUGUR-PROBE; M4 full, GATE-AUGUR-11 |
| `mission.separate` | M4 | M3 (BD-293; GATE-AUGUR-13 is due at M3 in the gate register) | none | M3 |
| `mission.revalidate` | M4 | M3 (N25a-16; GATE-AUGUR-12 at M3) | none | M3; the M4 verdict kinds through the shared engine (BD-347) |
| `mission.transport` | M8 | M4 (N25c-02: the static core; a request for the dynamic relation returns `FROB-REF-724`, exit 5, with the static core named as the available scope) | the static core (the handler words it) | M8 (BD-436) |

The early forms are what the gate scripts of M1, M3 and M4 call, so the conformance script of this bead accepts a
result for each of them from the milestone in the "Answers from" column (step 4 reads the capability report, not
the field).

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.

**`mission.distinguish`** (MCP tool `mission_distinguish`; owner `frob-mission`)
- Inputs: mission (a `.fmission` file or `q:…`); alternatives (designs, diagnoses or laws; `--between`);
  `--grammar` (from M4, N25c-17: candidate probes compiled from the experiment grammar, returned as
  `ProbeProposal` records); `--scope ignore` (from M4, N25e-38: every nuisance parameter evaluated
  independently and every bound marked `scope: undeclared`, the comparison form; it can only widen bounds).
- Outputs: the smallest admissible intervention or instrument whose readings separate the alternatives, or two
  witness mechanisms with opposing margins; when every admitted world implies the same decision, the statement that
  identification is not needed. A probe verdict is a conditional witness whose condition (the admitted mechanism
  set) is in the certificate and in the answer's assumptions (BD-291). Evidence: *certificate*.
- Chapter refusals: `FROB-REF-847` (an observation has no arrival instant, or a policy uses information before it
  arrives); `FROB-REF-859` (an experiment or world edit cannot be realized through the declared interfaces, or does
  not declare the causal path it changes); `FROB-REF-854` (from M3, BD-291: a query whose `archive` applies a
  stored probe's measurement contract, an earlier answer `a:…`, to a target whose mechanism set is not that
  certificate's condition; first repair `mission.revalidate`); `FROB-REF-850` (from M4, N25e-38: a counterexample
  assembled from independent extremes of a shared or undeclared-scope parameter, raised under 850 with `detail:
  unshared-witness` as the interim of N25e-45's row 4, which the ledger holds, N35x-02). With 847, the other
  admission codes, raised when the mission is admitted again before the verb answers (the admission rule in
  the notes; meanings as on `mission.check`): `FROB-REF-846`, `FROB-REF-855`, `FROB-REF-856`, `FROB-REF-868`,
  `FROB-REF-874`, `FROB-REF-877`, `FROB-REF-884` and `FROB-REF-887`.
- Implemented by: BD-135 (the reduced form at M1: enumerated worlds and the alias witness), BD-291 (probe design by
  decision partition, conditional witnesses and the direct test, M3; full at M4 with GATE-AUGUR-11). Extended by:
  N25c-17 (`--grammar`, M4), N25e-38 (`--scope`, the witness rule, M4).

**`mission.refute`** (MCP tool `mission_refute`; owner `frob-mission`)
- Inputs: mission (a `.fmission` file or `q:…`); the scope of the question, one of: a cell (`--cell`), one design
  and controller (`--design`, `--policy`), or neither, the whole-family form (from M3, N25e-11: the screen runs on
  the whole family); optional `--from a:…` (an answer or proof supplied as evidence); `--set key=value` (a
  one-field amendment that produces a new `MissionId`; BD-135 step 2); `--reuse <stored screen, classifier or
  bundle>` (N25c-21, M1: the artifact offered as evidence, checked for eligibility first); `--card
  <sensor>=<card>` and `--reads <n>` (from M3, N25e-34: `--card`, repeatable, binds a card, a card name or a
  `.fcard` file, to a sensor as the design names it, the one grammar `sense.alias` has in VERBS-sense and
  BD-134; the card's `error_dependence` governs the probe's reads; a value without `<sensor>=` fails the
  schema, 802; `--reads` is the number of reads before commitment); `--screen monomial` and
  `--weights equal|centre-share|<w,…>` (from M3,
  N25e-11: only the monomial screen, returning `result.screen` without paving; explicit weights are positive
  rationals summing to 1 with a common denominator of at most 16, otherwise 802); `--utility` (from M4,
  N25e-15); `--baseline <design>`, `--baseline-policy <policy>` and `--paired-plan <plan>` (from M4, BD-359:
  product-system dominance against a baseline, and the per-address outcome table evaluated under a registered
  paired plan); `--split-cap <k>`, `--region per-constraint` and `--boxes <n>` (from M4, N25e-24, the region
  spellings BD-358 gives `mission.synthesize`); `--scope ignore` and `--witness independent-extremes` (from M4,
  N25e-38); budget; `--estimate`.
- Outputs: a scoped impossibility bound with its proof, or a witness that defeats the design and controller; the
  binding constraint; the smallest relaxing change. For the whole family (N25e-11), the excluded family with its
  weights, the product, the binding pair and the repair, or the statement that the screen does not exclude the
  family. With `--utility`, Ū_d with its binding constraint, the relaxation's `encloses` list, the certificate
  reference and the label `body-limited` when Ū_d < u_req. With `--baseline`, the dominance verdict on shared
  inputs (BD-359); with `--paired-plan`, the table's digest, the closed fraction and the paired verdict. With the
  region form, per constraint S, ω, the three volume shares and the driver's counters (N25e-24). On a sensing
  mission the guaranteed value under the card's law, with the law named in the assumptions (N25e-34).
  Evidence: *certificate*.
- Chapter refusals: `FROB-REF-853` (a minimum is claimed without a matching lower bound, or an unattained infimum is
  reported as attained); `FROB-REF-868` (the `.fmission` file is not valid: unknown field, missing unit, misspelt
  parameter, number not an exact string); `FROB-REF-874` (the world family is empty, or its scope changed after
  candidates were tested); `FROB-REF-881` (evidence or a certificate establishes a different quantifier prefix than
  the query's, including clairvoyant evidence offered for a robust claim). The codes of BD-133's rule-to-refusal
  mapping, raised with exit 3 when evidence supplied by the caller (an answer passed with `--from a:…` or
  `--reuse`, a supplied proof, a rival producer's bundle) fails the check before the answer returns (BD-135
  step 1; N25-REF's note on 848 to 851): `FROB-REF-848` (a proof kind unknown, at an unknown version, disabled
  or out of scope); `FROB-REF-849` (malformed certificate arithmetic); `FROB-REF-850` (a frame, unit, sign or port
  order that does not match its definitions); `FROB-REF-851` (an authority the design does not declare, or an
  omitted active bound); `FROB-REF-852` (a monotone or symmetry transport without a side condition; its raising
  site arrives at M3); `FROB-REF-854` (a classifier, screen or certificate reused outside its eligibility, also
  raised by `--reuse`, N25c-21); `FROB-REF-833` (a flag missing or true); `FROB-REF-834` (a class above the weakest
  input); and the auditor's own `FROB-REF-826` (a scope or cell end that differs from what the proof establishes)
  and `FROB-REF-829` (another side condition of the kind's rule missing), the two rule areas with no mission
  row. Raised by the options above: `FROB-REF-856` (from M4, N25e-24: `--region joint`, a joint paving, or
  `--split-cap` above 6); `FROB-REF-850` also as the interim of N25e-45's row 4 (`detail: unshared-witness`,
  N25e-38: `--witness independent-extremes`, or any refutation, built from independent extremes of a shared
  parameter). The other admission codes, raised when the mission is admitted again before the verb answers
  (the admission rule in the notes; meanings as on `mission.check`): `FROB-REF-846`, `FROB-REF-847`,
  `FROB-REF-855`, `FROB-REF-877`, `FROB-REF-884` and `FROB-REF-887`, and 856 from M1 on its admission trigger
  (an `enclosed` query that names a joint paving). From M6 (N25c-44, the inverse holonomy query, selected by the
  gait block of the `.fmission`): the code of the ledger slug `connection-outside-stratum` (N25e-45's row 6,
  raised through the constant `frob_core::refusal::augur_late::CONNECTION_OUTSIDE_STRATUM`; family `1xx`; the
  number is the one N35x-02's run assigns at M0, so the row lists the assigned number, no interim exists and
  this bead prints none; exit 3), with `detail: stratum-crossing` (the shape bounds of the gait block cross a
  contact-mode boundary of the stratum) or `detail: rank-only-displacement` (a `Certified` finite displacement
  is wanted and the only evidence offered, for example an answer passed with `--from`, is accessibility rank).
- Implemented by: BD-135, BD-290. Extended by: N25c-21 (`--reuse`, M1), N25e-11 (the whole-family form,
  `--screen`, `--weights`, M3), N25e-34 (`--card`, `--reads`, M3), N25e-15 (`--utility`, M4), BD-359
  (`--baseline`, `--baseline-policy`, `--paired-plan`, M4), N25e-24 (`--split-cap`, `--region`, `--boxes`, M4),
  N25e-38 (`--scope`, `--witness`, M4), N25c-44 (the inverse holonomy query on a mission with a gait block,
  behind the feature `holonomy-query`, M6).

**`mission.revalidate`** (MCP tool `mission_revalidate`; owner `frob-mission`)
- Inputs: mission answer (`a:…`); exactly one change form: the edit (`--after-edit e:…`: mission, design family,
  card or world), `--against q:…` (an amended query), or `--since <identity>` (from M3, N28c-29: every change
  since that identity, the change form `result.revalidate` shares through the one engine); two of them together,
  or none, is `FROB-REF-802` with pointer `/change`, as on `result.revalidate` (N28b-32).
- Outputs: surviving certificates, invalidated regions, required reruns and their cost (one engine with
  `result.revalidate`); witness re-checks; feasibility and optimality obligations kept apart. Evidence:
  *certificate*.
- Chapter refusals: `FROB-REF-835` (a verdict is reused after a change without a recorded implication (D-11));
  `FROB-REF-854` (a classifier, screen or certificate is reused outside its eligibility: support, deadline, error
  units or tolerance changed); `FROB-REF-874` (the world scope changed and no revalidation was requested);
  `FROB-REF-881` (evidence or a certificate establishes a different quantifier prefix than the query's);
  `FROB-REF-870` (from M3: reuse of a verdict with no licence, or under a rule the registry does not know; Atelier's
  code, whose N28-REF row names this verb, listed on both revalidation verbs by N28c-29).
- Implemented by: BD-292 (the engine), N25a-16 (the verb, M3, GATE-AUGUR-12). Extended by: N28c-29 (`--since`,
  870 and the one-engine relation with `result.revalidate`, M3).

**`mission.separate`** (MCP tool `mission_separate`; owner `frob-mission`)
- Inputs (BD-293 step 1): the canonical form, a stored separation query `q:…` (N25b-22: decision, base world,
  legality set, smallness, budget, the two frozen policies) with `--designs <a>,<b>` naming the two rivals; or
  the inline form, which builds the same query in one call: `--a <design>`, `--policy-a <policy>`, `--b <design>`,
  `--policy-b <policy>`, `--task <task>`, the decision as one of `--requirement <atom>` or `--metric <name>`, each
  with `--margin <value>`, or `--diagnosis <label>`, then `--base-world <world>` and `--legality <set>`; budget;
  `--estimate` (the cost of search, reproduction and minimization separately).
- Outputs: a minimized, admissible, executable pair of worlds in which the ordering or diagnosis changes, with the
  edit that removes the effect. Evidence: *run*.
- Chapter refusals: `FROB-REF-360` (a world or edit lies outside the legality profile; the violated bound is named);
  `FROB-REF-362` (an edit does not declare the causal paths it changes); `FROB-REF-859` (an experiment or world edit
  cannot be realized through the declared interfaces, or does not declare the causal path it changes). And two
  refusals of the separation query's admission (N25b-22) that the plan states without a number: each is on the
  row under the number the allocation ledger assigns its slug (`registries/refusal_allocations.toml`, field
  `assigned`, filled by N35x-02's run at M0, before the row is entered; this bead prints neither number), is
  raised through one named constant of N25b-22 and exits 3 with the field as pointer:
  `world-edit-scale-undeclared` (family `3xx`: a world edit on a field with no declared scale; the field is
  named and no scale is invented) and `decision-names-shaping-reward` (family `5xx`: the decision names a
  shaping reward rather than a hard-limit monitor; first repair: name the hard-limit monitor the reward shapes).
- Implemented by: BD-293 (M3, GATE-AUGUR-13), with the query's admission by N25b-22.

**`mission.show`** (MCP tool `mission_show`; owner `frob-mission`; `paginated = true`: the generator adds `cursor`,
`limit`, `follow` and appends 806 and 807)
- Inputs: mission; answer (`--answer a:…`); `--set` (`inner`, `outer`, `excluded`, `unresolved`; from M8,
  N25c-33, the further parts of a facility answer: `strategies`, `per_budget`, `ties`, `acceptance` and `unmet`,
  `excluded` then holding the alternatives with their obstructions; while N25c-33 is not in the build these five
  values answer `FROB-REF-724`, exit 5, by the rule for registered options in the notes); `--order
  canonical|cost` (default `canonical`, BD-135 step 9; `cost` orders cells by their cost bound, ascending, ties
  in canonical order, and cells with no cost bound after the bounded ones in canonical order: DESIGN of BD-135
  step 9, the form N32b-16 calls with `--limit 3`); the generated `--limit`, `--cursor`
  and `--follow` (there is no `--page-size`); the membership selector `--at <design or box>` (from M4: answered
  from the stored per-constraint sets, by implicit membership, without re-running the answer).
- Outputs: the cells of the set, ordered canonically and paged against the committed frontier, with measure and
  proofs; for `outer`, the definition ("domain minus excluded cells"), the cells, `inner_within`,
  `unresolved_within`, and a measure only when the query's `resolution` names one. With `--at`: the membership of
  that design or box in each stored per-constraint set, with the certificate that decides it or `unresolved`. With
  a facility part (N25c-33): the rows of that part of the stored answer, paged the same way: the surviving
  strategies, one report per declared change budget, the smallest survey that settles each tie, one dispatchable
  acceptance mission per survivor, or the finding that every allowed alternative leaves a mission unmet. No
  unresolved cell is printed as a verdict. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-135. Extended by: N25b-10 and BD-358 (`--at`, region answers from per-constraint sets, M4;
  used by N25b-11 step 8), N25c-33 (the five facility values of `--set`, M8).

**`mission.synthesize`** (MCP tool `mission_synthesize`; owner `frob-mission`)
- Inputs, by the bead that fixes each spelling and the milestone from which it answers:
  - Core (BD-135, M1): mission (a `.fmission` file or `q:…`); optional `--design`, `--policy`, `--cell`,
    `--exists`, `--paving`, `--from a:…`, `--strategy`, `--set key=value` (a one-field amendment that produces a
    new `MissionId`); budget; `--estimate`.
  - M1: `--reuse <stored screen, classifier or bundle>` (N25c-21); `--enclosure <width>` (N25e-31: the width of
    the optimum's enclosure, default 10⁻⁹ in the objective's unit); `--lowering primary|second|<lowering name>`
    (N25d-34's two roles of PO-AUGUR-01, and the registered name of a template's lowering, `scenario-tree` for
    the docking template, N25c-20; default `primary`; the answer records the lowering that ran).
  - M3: `--card <sensor>=<card>` (repeatable; the grammar of `mission.refute` above) and `--reads <n>`
    (N25e-34); `--transport-pairs <n>` (BD-288 step 8: the
    harness's pair budget per direction, default 10000; 0 is `FROB-REF-802` while `frontier-transport` is on,
    except together with `--transport-unchecked`); `--fill licensed` (N25e-07: default off; licensed monotone
    fills appear as `proved` points with their proof reference). `--transport-unchecked` (BD-288) exists only in
    a build with the cargo feature `test-switches`: it is not in the release schema or the snapshot, and a
    release build answers it as an unknown parameter, 802 (see the notes).
  - M4: `--region per-constraint` and `--boxes <n>` (BD-358; `--region joint` is not a mode and is refused with
    856); `--split-cap <k>` (N25e-24, default 6); `--scheduler fifo|opportunity-loss|largest-unresolved|hinted`
    with `--hints <order.json>` (N25b-12: the hints are required for `hinted` and refused with 802 otherwise;
    the default is `opportunity-loss` in a build with the `stats` feature and `largest-unresolved` without it,
    and the cost block names the scheduler that ran); `--cuts on|off` and `--quotient on|off` (BD-357: default
    the decision in force; `on` for a fired clause is 724 naming the decision record); `--family-order
    ranked|fixed|reversed` (N25b-15); `--tier batch-cpu|gpu-cuda|gpu-metal`, `--placement
    auto|one-card|two-shards|host` and `--kernel-class exact-integer|guarded` (N25c-09; the placement receipt
    records the requested and the actual placement). `--strategy` takes exactly `codesign`, `refine`, `linkage`,
    `continuation` and the reserved `placement` (N25d-06: another value is 802; a strategy whose feature is not
    compiled or not admitted is 724 with pointer `/strategy`). Lane options: `--catalog <set>` (BD-360),
    `--tolerance <width>` and `--candidate <resource>=<value>` (N25e-42) with `codesign`; `--threads <n>`
    (default 8), `--knots <N>` (default 50) and `--seed <n>` (BD-362) with `refine`; `--motions <library>`,
    `--ratios catalog|continuous` and `--objective energy|first-moment:<component>` (N25e-20) with `placement`.
  - M6: `--atlas <id|new>` (N25c-47, behind the feature `shape-atlas`). A mission whose `.fmission` carries a
    gait block selects the inverse holonomy query (N25c-44, behind the feature `holonomy-query`); it adds no
    option.
  - M8: `--family <id>` with `--strategy continuation` (N25d-23: the mechanism family whose generic-solve
    artifact is used, for example `stewart-gough-fk` in N25d-24; feature-gated); `--derive-sets` (N25c-41,
    behind `labels-from-contact`). The facility query (N25c-29 to N25c-33) adds no option: it is selected by
    the mission itself (a `world` compiled from a scene by `mission.new --from-scene`, several entries in
    `policy_class.decision_makers`, a declared stage chain, admitted edits in `design_domain`, `objective.claim:
    pareto`), and its parts are read with `mission.show --set`.
  A registered option whose feature is not in the build answers `FROB-REF-724`, exit 5, status `not-in-build`,
  with the pointer at the option and the missing feature named in the first repair (DESIGN, see the notes).
- Outputs: inner cells with witnesses, excluded cells with proofs, unresolved cells, cost bounds, the next test, the
  certificate bundle root. From the extending beads: `cost_bounds.optimum` with its enclosure (N25e-31); per
  direction the transports issued, the pairs tried and `rejected_by` (BD-288); point statuses for cells built from
  a sweep (N25e-07); `result.region` (BD-358); `result.candidates[]`, each with `verdict: null` unless a cell with
  a checked proof decides it (N25d-06); `result.sizing` (N25e-42). Evidence: *certificate*.
- Chapter refusals: `FROB-REF-846` (a chance constraint lacks its population, event or confidence, or marginal stage
  rates are multiplied without a dependence assumption); `FROB-REF-847` (an observation has no arrival instant, or a
  policy uses information before it arrives); `FROB-REF-853` (a minimum is claimed without a matching lower bound,
  or an unattained infimum is reported as attained); `FROB-REF-855` (evidence profiles are mixed inside one cell, or
  a probability mass is requested without a supplied weighting); `FROB-REF-856` (a region certificate is requested
  over more parameters than the admitted limit, or a joint paving is requested); `FROB-REF-857` (a synthesis system
  is outside the symbolic caps or is not zero-dimensional); `FROB-REF-868` (the `.fmission` file is not valid:
  unknown field, missing unit, misspelt parameter, number not an exact string); `FROB-REF-874` (the world family is
  empty, or its scope changed after candidates were tested); `FROB-REF-877` (the predicates conflict with the world
  family for every design); `FROB-REF-881` (an answer supplied with `--from a:…`, or a proof, establishes another
  quantifier prefix than the query's, including clairvoyant evidence offered for a robust claim; raised by the
  prefix check of N25a-03, asserted by BD-131); `FROB-REF-884` (no quantifier prefix, or a hidden parameter with no
  persistence declaration); `FROB-REF-887` (a policy is indexed by a parameter the controller cannot observe).
  The codes of BD-133's rule-to-refusal mapping, raised with exit 3 when evidence supplied by the caller (an
  answer passed with `--from a:…` or `--reuse`, a supplied proof, a rival producer's bundle) fails the check
  before the answer returns (BD-135 step 1): `FROB-REF-848`, `FROB-REF-849`, `FROB-REF-850`, `FROB-REF-851`,
  `FROB-REF-852`, `FROB-REF-854`, `FROB-REF-833` and `FROB-REF-834`, and the auditor's own `FROB-REF-826` and
  `FROB-REF-829` for the two rule areas with no mission row (meanings as on `mission.refute`). 854 is also raised
  by `--reuse` outside the artifact's eligibility (N25c-21, M1) and by a certificate reused outside its reading
  set, for example a nominal answer offered with `--from` for another reading set (N25c-14, M3); 852 is also
  raised by `--fill licensed` on a sweep with no licence, such as a dynamic rollout (N25e-07, M3). Passed through
  from another chapter: `FROB-REF-263` (from M4, BD-362: the `refine` lane's derivative request refused by the
  labelled-derivative machinery near grazing in the initial motion; N12-REF's code). Interims of N25e-45, held
  by the ledger (N35x-02): 868 with `detail: law-class-missing` (row 2, at admission), 853 with `detail:
  sizing-witness-missing` (row 5, N25e-42) and 857 with `detail: synthesis-out-of-scope` (row 7, BD-362).
  From M6 (N25c-44, the inverse holonomy query): the code of the ledger slug `connection-outside-stratum`, as
  on `mission.refute` (the `1xx` number N35x-02's run assigns at M0; exit 3; `detail: stratum-crossing` or
  `rank-only-displacement`). From M8, on a facility query: `FROB-REF-858` (stage composition fails on a stage
  chain the query declares: an exit set not inside the next entry set, a clock mismatch, or a resource spent
  twice; N25c-30; a chain that the schedule search proposes and that fails composition is discarded with that
  reason and is not a refusal) and `FROB-REF-859` (a world edit in an edit set of the query cannot be realized
  through the declared interfaces, or does not declare the causal path it changes; N25c-31).
- Implemented by: BD-135. Extended by: N25c-21 (`--reuse`, M1), N25e-31 (`--enclosure`, M1), N25c-20 and N25d-34
  (`--lowering`, M1), N25e-34 (`--card`, `--reads`, M3), BD-288 (transports, `--transport-pairs`,
  `--transport-unchecked`, M3), N25e-07 (`--fill`, M3), N25c-14 (854 on a reused certificate, M3), BD-358
  (`--region`, `--boxes`, M4), N25e-24 (`--split-cap`, M4), N25b-12 (`--scheduler`, `--hints`, M4), BD-357
  (`--cuts`, `--quotient`, M4), N25b-15 (`--family-order`, M4), N25c-09 (`--tier`, `--placement`,
  `--kernel-class`, M4), N25d-06 (strategy dispatch, M4), BD-360, BD-361, BD-362, N25e-20 and N25e-42 (the
  `codesign`, `linkage`, `refine` and `placement` lanes and their options, M4), N25c-47 (`--atlas`, M6), N25c-44
  (the inverse holonomy query on a gait block, M6), N25d-23 (`continuation`, `--family`, M8), N25c-41
  (`--derive-sets`, M8), and the facility query (M8): N25c-29 (the site model of a query made with `mission.new
  --from-scene`), N25c-30 (stage composition and schedules across decision makers; 858), N25c-31 (the priced
  world-edit grammar; 859), N25c-32 (operation over time) and N25c-33 (the counterfactual outputs, read with
  `mission.show --set`).

**`mission.transport`** (MCP tool `mission_transport`; owner `frob-mission`)
- Inputs: a certified claim (`a:…`), a claims document (`<claims.fmission>`, N25e-02) or a skill segment (a
  `.fskill` file); the source and target (`--from`, `--to`): design references `d:…` (the static core) or, from
  M4, finite models `*.fkernel.json` (schema `frob.mission.fkernel/1`, N25e-02); a map between task interfaces
  (`--interface <file>`; optional for finite models); from M4 (N25e-02): `--claims <id,…|all>` (the claims of the
  document to transport), `--certificate <bundle>` (a caller-built transport certificate, checked before use and
  never trusted) and `--relation <relation.json>` (the relation table for finite models; default equality of
  labels); `--relation compute` (N25c-03: the greatest relation on the finite abstraction, computed by the
  checker; accepted by name from M4 and refused with 724 and the static core named until N25c-03 enables it at
  M8 behind the feature `transport-relation`, and again after KILL-AUGUR-TRANSPORT fires); `--lowering
  vertex|facets` (N25c-02, M4: the second lowering of PO-AUGUR-01; vertex by default); `--coupling-source
  paired-runs` with `--pairs <n>` and `--level <α>` (from M8, N25e-04: the coupling budget estimated from sealed
  paired runs; DESIGN spelling, see the notes); budget; `--estimate`.
- Outputs: the transported claim with a bounded error, or a counterexample naming the contact mode where authority
  is lost; static core at M4 (per contact mode: does the target's reachable wrench set cover the interface's
  requirement, with `lp-primal` and `lp-farkas` proofs). Evidence: *certificate* (covered and lost-authority cells
  `Certified`, tier `static`; a replayed counterexample is a trace witness, class `Tested`; the answer's class is
  the weakest among its cells); `authorization` with the three flags false. From M4, `result.claims[]` with `id`,
  `kind`, `source_value`, the coupling budget (`total`, `per_step`, `source`, `class`), `transported`, `threshold`,
  `verdict` and the class per claim (N25e-02); from M8, per skill n, k, p̄, slack, the verdict, the study receipt
  and the broken pairs as replay addresses (N25e-04).
- Chapter refusals: `FROB-REF-444` (a transported skill does not refine its specification, or a refinement that
  selects only the stable branch of a set-valued successor); `FROB-REF-847` (an observable without an arrival
  instant, or used before it arrives); `FROB-REF-850` (a frame, unit, sign or port order that does not match its
  definitions, for example a first moment passed as a torque); `FROB-REF-851` (an authority the design does not
  declare); `FROB-REF-852` (a monotone or symmetry transport lacks a side condition); `FROB-REF-858` (stage
  composition fails: an exit set not inside the next entry set, a clock mismatch, or a resource spent twice). A
  transported chance claim without a coupling budget, or one marked `universal`, is refused under the coupling
  rule (N25e-02): N25e-45's row 1, held by the ledger (N35x-02) and raised meanwhile as 852 with `detail:
  coupling-budget-missing`. From another chapter: `FROB-REF-843` (from M8, N25e-04: a paired run whose world
  address appears in either controller's training or tuning lineage; the qualification lane's code, N24-REF).
- Implemented by: N25c-02 (the static core, M4), N25e-02 (claim kinds, coupling budgets, finite models, M4); the M8
  form, children of the epic BD-436: N25c-03 (the computed relation, released only if KILL-AUGUR-TRANSPORT does
  not fire) and N25e-04 (paired-run coupling budgets).

**`mission.new`** (MCP tool `mission_new`; owner `frob-spec`)
- Inputs: a `.fmission` document as the positional argument, or a template: `--template <name>[@<version>]`
  (BD-126's `stance-family`, `docking` and `pusher` at M1; `delivery-and-placement` at M4, N25ax-01) with the new
  query's name as the positional argument and its parameters as `--set key=value`, repeatable (`--set
  catalog=full` answers 724 where the at-scale mission work is not in the build, N25ax-01); from M8,
  `--from-scene <scene>` (N25c-29: the query's `world` field is the compiled site model, whose digest enters
  `MissionId`).
- Outputs: `MissionId` and the identities every mutable name was bound to; `bound.family` names the template with
  its content hash. Evidence: *static*.
- Chapter refusals: `FROB-REF-868` (the `.fmission` file is not valid: unknown field, missing unit, misspelt
  parameter, number not an exact string). From another chapter: `FROB-REF-304` (from M8, N25c-29: the scene's
  scale authority is below what the task requires, for example a `relative-only` scene for a metric task; the
  scene chapter's code, N19-REF).
- Implemented by: BD-126. Extended by: N25ax-01 (the `delivery-and-placement` template, M4), N25c-29
  (`--from-scene`, M8).

**`mission.check`** (MCP tool `mission_check`; owner `frob-spec`)
- Inputs: mission.
- Outputs: domain `nonempty` or the refusal; topologies; free parameters; the quantifier order echoed; the fragment
  (whether an exact answer is available); horizon. Evidence: *static*.
- Chapter refusals: `FROB-REF-868` (the `.fmission` file is not valid: unknown field, missing unit, misspelt
  parameter, number not an exact string); `FROB-REF-874` (the world family is empty, or its scope changed after
  candidates were tested); `FROB-REF-877` (the predicates conflict with the world family for every design);
  `FROB-REF-884` (no quantifier prefix, or a hidden parameter with no persistence declaration); and from the
  semantic checks of admission: `FROB-REF-846` (a chance constraint lacks its population, event or confidence,
  or marginal stage rates are multiplied without a dependence assumption); `FROB-REF-847` (an observation field
  has no arrival instant, or a policy input is used before it arrives); `FROB-REF-855` (evidence profiles are
  mixed inside one cell, or a probability mass is requested without a supplied weighting); `FROB-REF-856` (its
  admission trigger: an `enclosed` query that names a joint paving, or more split parameters than the limit of
  six); `FROB-REF-887` (a policy is indexed by a parameter the controller cannot observe, or its kind is
  `clairvoyant` without the label `relaxation: exclusion-only`). The checks run in the order of BD-126's table
  and the first failure refuses, exit 3. More than one decision maker is the appended `FROB-REF-724`, exit 5
  (N25a-02: the fail-closed default of OPEN-AUGUR-9, until the multi-robot slice).
- Implemented by: BD-126 (the verb, `frob_mission::admit` and its checks 1 to 3: 868 and 874). Extended by:
  N25a-02 (checks 4 to 13: 884, 847, 887, 855, 856, the semantic uses of 868 and 874, and 724 for more than one
  decision maker), N25a-11 (check 14: 846), N25b-07 (check 15: 877).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `mission.distinguish` | `mission:run` | `frob.mission.distinguish.out/1` | `mission` | `frob mission distinguish q:5b1e --between c:0204,c:0007 --key dis-1 --json` |
| `mission.refute` | `mission:run` | `frob.mission.refute.out/1` | `mission` | `frob mission refute q:3c9d --design v=9,r=5/2 --key ref-1 --json` |
| `mission.revalidate` | `mission:run` | `frob.mission.revalidate.out/1` | `transfer` | `frob mission revalidate a:0e51 --after-edit e:5d1c --key mrv-1 --json` |
| `mission.separate` | `mission:run` | `frob.mission.separate.out/1` | `attack` | `frob mission separate q:5b1e --designs d:a3f1,d:b77e --key sep-1 --json` |
| `mission.show` | `mission:read` | `frob.mission.show.out/1` | `mission`, `transfer` | `frob mission show q:3c9d --answer a:0e51 --set outer --json` |
| `mission.synthesize` | `mission:run` | `frob.mission.synthesize.out/1` | `mission` | `frob mission synthesize q:3c9d --budget cpu_core_s=60 --key syn-1 --sync-wait-ms 2000 --json` |
| `mission.transport` | `mission:run` | `frob.mission.transport.out/1` | `transfer` | `frob mission transport a:8b27 --from d:41aa --to d:52bc --interface tray.json --key tr-1 --json` |
| `mission.new` | `mission:write` | `frob.mission.new.out/1` | `mission` | `frob mission new tray-stance.fmission --key mis-1 --json` |
| `mission.check` | `mission:read` | `frob.mission.check.out/1` | `mission` | `frob mission check q:3c9d --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "mission.distinguish"
cluster = "mission"
kind = "run"
tier = "F"
capabilities = ["mission:run"]
input_schema = "frob.mission.distinguish.in/1"
output_schema = "frob.mission.distinguish.out/1"
refusals = ["FROB-REF-846", "FROB-REF-847", "FROB-REF-850", "FROB-REF-854", "FROB-REF-855", "FROB-REF-856",
            "FROB-REF-859", "FROB-REF-868", "FROB-REF-874", "FROB-REF-877", "FROB-REF-884", "FROB-REF-887"]
            # the nine admission codes (846, 847, 855, 856, 868, 874, 877, 884, 887) and 859;
            # 854 from M3 (BD-291); 850 from M4 (N25e-38, interim of N25e-45 row 4)
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["mission"]
examples = ["frob mission distinguish q:5b1e --between c:0204,c:0007 --key dis-1 --json"]
owner = "frob-mission"
purpose = "The smallest admissible intervention or instrument that separates alternatives"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind and appends the surface-wide refusals. This item
   mirrors the kinds table of NJL-01, which governs wherever the two differ. Common input properties: `run`:
   `sync_wait_ms` (0 to 30,000; MCP default 5,000; the command line waits to completion), `idempotency_key`
   (`--key`), `budget`, `estimate_only` (`--estimate`), `on_cancel` (`leave`, the default, or `cancel_job`) and
   `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved compute-envelope scope the
   job cites; a job's scope is the token of a launcher it runs under, then a cited `envelope_scope`, then a
   scope the verb derives from its own inputs, otherwise `adhoc`, whose class in force is E0); `mutate`:
   `idempotency_key` and `expected_parent`; `read` and `admin`: none. `idempotency_key` is never in the
   JSON-schema `required` array: the annotation `x-frob-key` (`"required"`, or `"optional"` on a `key_optional`
   entry) carries the rule, and a call without a key is refused with 805 by the admission transaction (N32a-09)
   on both surfaces, never as a schema failure. Appended by kind: `FROB-REF-720` (scope missing), 723 (token
   invalid), 724 (not implemented in this build) and 802 (schema failure, including an unknown parameter) for
   every verb; 701 to 704 (budget and admission), 761 (above the compute envelope in force), 803 (key reused
   with different content) and 805 (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`.
   Appended by field, whatever the kind: 806 and 807, with the inputs `cursor`, `limit` and `follow`, for
   `paginated = true`; 808 (an integer outside ±(2⁵³ − 1)) for an entry with an `int53` input property, which
   every `run` entry has in `sync_wait_ms` and an entry without one never gets; no 805 for a `key_optional`
   entry; `estimate_only` for a non-`run` entry with `estimable = true`; 790 for a non-empty `needs_groups`; 891
   to 893 for artifact `params`; 734 and 828 for a parameter that accepts a path to a file
   (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`). The later variations of NJL-01's table (10
   onward: the inputs `override` and `override_reason`, `from_proposal`, `machine` and `engine`, which the beads
   named there add to some or all `run` entries) are not copied here: that table is the one list.
   The parameters of this cluster that accept a path to a file are declared in `params` (N34-24), so the
   generator appends 891, 734 and 828 to their rows: the mission positional when it is a `.fmission` file (every
   verb but `mission.show` and `mission.revalidate`, which take stored identities); `mission.synthesize`'s
   `--hints`; the card of `--card <sensor>=<card>` on `mission.refute` and `mission.synthesize` when it is a
   `.fcard` file; `mission.refute`'s `--paired-plan`; `mission.transport`'s `--interface`,
   `--relation <relation.json>`, `--certificate`, a `.fskill` positional and the finite models given to `--from`
   and `--to`; and a design given as a `.fdesign` path to `mission.refute` or `mission.distinguish`.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Nine verbs: the six of the mission chapter (`synthesize`, `refute`, `distinguish`, `separate`, `transport`,
  `revalidate`), `show` for stored answers, and `new` and `check` for admission.
- Admission (BD-126's rule: creation stores a valid document, and admission is the read verb). `mission.new`
  runs check 1 of `frob_mission::admit` (the file is valid, 868) before it stores anything and lists no other
  admission code. `mission.check` runs all fifteen checks and raises the nine admission codes: 846, 847, 855,
  856 (on its admission trigger), 868, 874, 877, 884 and 887. A stored query may never have been checked, so
  every verb that takes a mission and answers on it (`mission.synthesize`, `mission.refute`,
  `mission.distinguish`) admits it again before it answers, and those three rows list the same nine codes. 853
  is not an admission code: no check of `admit` raises it; `mission.synthesize` and `mission.refute` raise it
  when an answer, or evidence supplied with `--from` or `--reuse`, claims a minimum without a matching lower
  bound.
- A point query is `mission.synthesize` with every design parameter bound; an existence query returns `true` with a
  witness, `false` with an exclusion of the whole domain, or `unknown` with the reason. Binding the body is a
  special case, not a different verb.
- The inner set holds only cells with a constructive witness; the outer set is the domain minus the excluded cells
  and is not listed as cells of its own. A minimum is stated only with a matching lower bound (`FROB-REF-853`).
- Two verbs transport and their scopes differ: `mission.transport` moves a certified claim through an interface map
  with a bounded error (Frontier; static core at M4; beyond it Moonshot, M8); `skill.transport` moves a skill
  artifact with a learned residual (Moonshot).
- `mission.revalidate` and `result.revalidate` are two verbs over one engine.
- Every `run` example carries `--key`: the conformance script runs the examples verbatim, and a `run` call without
  a key is `FROB-REF-805`.
- Options (round 3). Every option a mechanism bead or a script calls on a verb of this cluster is in its block,
  with the bead that fixes the spelling and the milestone from which it answers (convention 8 of the program:
  scripts call only what a row carries). The handler fixes behaviour; the snapshot pins the spelling.
- Feature-gated options (DESIGN, following NJL-01's rule for verbs and N25d-06's for strategies). A registered
  option is in the generated schema in every build, so the schema and the snapshot stay stable across builds;
  when its feature is not compiled or not admitted, or its kill gate has fired, it answers `FROB-REF-724`, exit 5,
  status `not-in-build` or `killed`, with the pointer at the option and the missing feature named in the first
  repair. An option that is not registered is 802. This applies to `--atlas` (`shape-atlas`), `--family` and
  `continuation`, `--derive-sets` (`labels-from-contact`, N25c-41),
  `--relation compute` (`transport-relation`, KILL-AUGUR-TRANSPORT), `--cuts on` and `--quotient on` on a fired
  clause, `--set catalog=full` on `mission.new`, and `--coupling-source paired-runs` with `--pairs` and `--level`
  after either clause of KILL-COUPLING fires: the finite clause, decided at M4 (N25ex-01), or the tray clause,
  decided at M8 (N25e-06); the first repair then names the fallback (`mission.transport` carries universal
  claims only, and a chance or expected-value claim is measured on the target with `eval.qualify`), and the
  compute `budget` is untouched. The same rule holds for a registered value and for a registered block of the
  mission document: the five facility values of `mission.show --set` while N25c-33 is not in the build, and a
  gait block in a build without `holonomy-query` (pointer at the block; status `killed` after KILL-AUGUR-PHASE
  fires; N25c-44). A scheduler that needs `stats` is not refused: the default changes and the cost
  block names the scheduler that ran (N25b-12).
- Test-build switches (DESIGN). `--transport-unchecked` (BD-288) writes transports without the proposer's
  pre-check, for testing the auditor alone, and must never reach a release: it is generated only in a build with
  the cargo feature `test-switches`, it is absent from the release schema and from the help snapshot, and a
  release build answers it as an unknown parameter, 802, as BD-288 states. The entry marks it with the field
  the entry schema has for an input that exists only outside a release build, `conformance_only =
  ["transport_unchecked"]` (N32a-02, lint `RL-24`): the build script removes the named property from the
  embedded input schema of every build without the `frob-verbs` feature `conformance`, before anything is
  generated, so the release surfaces and the snapshot never see it and the registry digest is the same in both
  builds. BD-288's `test-switches` build is a build with that feature on: the `frob-mission` feature
  `test-switches` is enabled exactly when `frob-verbs/conformance` is, both through `gauntlet-mutations` (BD-025's
  umbrella), so that the schema and the handler agree (a handler without the property, or a property without the
  handler, is never built).
- Coupling-budget source (DESIGN). N25e-04 spelled its option `--budget paired-runs`; `budget` is a generated
  common property of every `run` verb (the compute budget, `--budget cpu_core_s=60`), so the same spelling cannot
  carry the coupling budget's source. The row registers `--coupling-source paired-runs` with `--pairs <n>` and
  `--level <α>` (the source names the variant `CommonNoisePairedRuns` of N25e-02's `BudgetSource`); without it
  the source is the exact computation for finite models, or none. N25e-04, N25e-06 and N25ex-01 use this
  spelling.
- `--split-cap`. The schema states only the minimum (1): `--split-cap 0` fails the schema (802), and a cap above 6
  is the domain refusal 856 naming ADR-AUGUR-06's bisection limit (N25e-24, BD-358), which a schema maximum
  would turn into 802.
- Codes of other chapters on these rows: 263 (N12-REF) passed through by `mission.synthesize`'s `refine` lane;
  304 (N19-REF) on `mission.new --from-scene`; 843 (N24-REF) on `mission.transport`'s paired runs; 870 (N28-REF)
  on `mission.revalidate`. Each is listed because the handler raises it, and a verb never returns a code its
  row does not list; the row is the record of which verb returns the code, and the owning registry's row names
  the bead that raises it.
- `mission.revalidate` and `result.revalidate` share one engine (N28c-29): the same change forms (`--after-edit`,
  `--since`; `mission.revalidate` also `--against`), the same refusal 870, and the agreement test of N28c-29.
  The relation is an entry field: `same_engine_as = "result.revalidate"` on this row and the converse on the
  other, the field N28c-29 adds to both entries at M3 by the entry schema's rule for a field a later bead adds
  (its row in N32a-02's field table, with its lint); the generated documentation prints the relation from it.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly nine entries with `cluster = "mission"` (2 `read`, 6 `run`, 1 `mutate`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster mission` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 9, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_mission.sh` runs each verb of the cluster (nine), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/mission/<verb>.help.json` byte for byte for each of
      the nine, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_mission` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 9 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `mission.check` is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`mission.distinguish`: 846, 847, 850, 854, 855,
  856, 859, 868, 874, 877, 884, 887; `mission.refute`: 826, 829, 833, 834, 846, 847, 848, 849, 850, 851, 852,
  853, 854, 855, 856, 868, 874, 877, 881, 884, 887; `mission.revalidate`: 835, 854, 870, 874, 881;
  `mission.separate`: 360, 362, 859; `mission.show`: none; `mission.synthesize`: 263, 826, 829, 833, 834, 846,
  847, 848, 849, 850, 851, 852, 853, 854, 855, 856, 857, 858, 859, 868, 874, 877, 881, 884, 887;
  `mission.transport`: 444, 843, 847, 850, 851, 852, 858; `mission.new`: 304, 868; `mission.check`: 846, 847,
  855, 856, 868, 874, 877, 884, 887); beside them the refusals that the allocation ledger numbers, each
  compared by reading the `assigned` field of its slug's row in `registries/refusal_allocations.toml`, never
  by a typed number: `connection-outside-stratum` on `mission.synthesize` and `mission.refute`,
  `world-edit-scale-undeclared` and `decision-names-shaping-reward` on `mission.separate` (a planted row
  without one of them fails, and so does a slug whose ledger row has no assigned number); all nine admission
  codes are on each of the four rows `mission.check`, `mission.synthesize`, `mission.refute` and
  `mission.distinguish`, and a planted `mission.refute` row without 884 fails; the appended codes equal the
  rule for its kind and fields (`mission.show`
  is paginated and also gets 806 and 807; the rows with file-path `params` get 891, 734 and 828); every listed
  code exists in `registries/refusals.toml`, and for the codes of other registries (263, 304, 843, 870) the
  generated-artifact check reports a missing raiser entry as `pending` with the owning registry, never as passed.
- `options_by_row`: the generated input schemas accept exactly the options of each block, among them
  (`mission.synthesize`) `--reuse`, `--enclosure`, `--lowering`, `--card`, `--reads`, `--transport-pairs`,
  `--fill`, `--region`, `--boxes`, `--split-cap`, `--scheduler`, `--hints`, `--cuts`, `--quotient`,
  `--family-order`, `--tier`, `--placement`, `--kernel-class`, `--catalog`, `--tolerance`, `--candidate`,
  `--threads`, `--knots`, `--seed`, `--motions`, `--ratios`, `--objective`, `--atlas`, `--family`,
  `--derive-sets`; (`mission.refute`) `--set`, `--reuse`, `--card`, `--reads`, `--screen`, `--weights`,
  `--utility`, `--baseline`, `--baseline-policy`, `--paired-plan`, `--split-cap`, `--region`, `--boxes`,
  `--scope`, `--witness`; (`mission.distinguish`) `--between`, `--grammar`, `--scope`; (`mission.revalidate`)
  `--after-edit`, `--against`, `--since`; (`mission.separate`) `--designs` and the inline flags of BD-293;
  (`mission.show`) `--answer`, `--set`, `--order`, `--at`; (`mission.transport`) `--from`, `--to`,
  `--interface`, `--claims`, `--certificate`, `--relation`, `--lowering`, `--coupling-source`, `--pairs`,
  `--level`; (`mission.new`) `--template`, `--set`, `--from-scene`. Spellings that beads once used and the rows
  do not carry are 802: `--page-size`, `--idempotency-key`, `--budget-wall-s`, and `--budget paired-runs` on
  `mission.transport` (a compute-budget parse failure).
- `card_one_grammar`: on `mission.refute` and on `mission.synthesize`, `--card probe=independent.fcard` parses,
  `--card` given twice for two sensors parses to two bindings, and the bare `--card independent.fcard` is 802
  with pointer `/card`; the property's schema equals that of `sense.alias`'s `card`.
- `show_set_values`: `mission.show --set` accepts exactly `inner`, `outer`, `excluded`, `unresolved`,
  `strategies`, `per_budget`, `ties`, `acceptance` and `unmet`; another value is 802; in a build without N25c-33
  each of the last five answers 724, exit 5, pointer `/set`.
- `feature_gated_options`: in a build without `shape-atlas`, `mission.synthesize --atlas new` answers 724, exit 5,
  status `not-in-build`, pointer `/atlas`; the same for `--derive-sets` without `labels-from-contact` and for
  `--relation compute` before M8; an unregistered option is 802; the help schema is byte-identical with and
  without the features. (The `killed` answers after a kill gate fires, and the gait block without
  `holonomy-query`, are tested by the beads that own them: N25ex-01 and N25e-06 for `--coupling-source`,
  N25c-44 for the gait block.)
- `transport_unchecked_test_only`: the entry lists `transport_unchecked` in `conformance_only`; a release
  build's schema has no `transport_unchecked` property and answers `--transport-unchecked` with 802; a
  `test-switches` build (the `frob-verbs` feature `conformance` on) accepts it and marks the answer
  `unchecked-transports`; the help snapshot does not contain it.
- `revalidate_one_change_form`: `mission.revalidate` with `--since` and `--after-edit` together, or with none of
  the three forms, is 802 with pointer `/change`.
- `split_cap_bounds`: `--split-cap 0` is 802; `--split-cap 7` is 856; `--split-cap 6` is accepted.
- `early_forms_not_remilestoned`: `first_milestone` equals the appendix for every row (`mission.distinguish`,
  `mission.separate`, `mission.revalidate` M4; `mission.transport` M8); in an M1 build `mission.distinguish`
  answers and `capabilities` lists it implemented with `form: "enumerated-worlds"`; in an M3 build
  `mission.separate` and `mission.revalidate` answer; in an M4 build `mission.transport` answers with the static
  core named as its restriction and a dynamic-relation request is 724.
- `run_examples_have_keys`: every `run` and `mutate` example carries `--key`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`mission_distinguish`, `mission_refute`,
  `mission_revalidate`, `mission_separate`, `mission_show`, `mission_synthesize`, `mission_transport`,
  `mission_new`, `mission_check`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of convention 2 (`envelope_scope` among them) and
  refuses `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; a verb
  whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_mission.sh` (header: `bead=VERBS-mission milestone=M1 labels=tether,verbs,verb:mission.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field,dropped-option`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance mission`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the nine verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
mission` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster mission`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1; (8) for each verb, every option named in its block
appears in `frob <verb> --help --json` (logged as `option_present` per option, with the block's bead as the
source), and in a build without a gated option's feature the option answers 724 with the feature named.

Log: `target/e2e-logs/verbs_mission/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 9
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 9 verbs (2 `read`, 6 `run`, 1 `mutate`); the tables above. KA-755 (the exit-code
  table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the
  surface's tables; exact) is exercised for every verb by steps (6) and (7).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`);
  `dropped-option` (`--reuse` removed from the `mission.refute` entry: step (8) fails naming the verb, the option
  and N25c-21). A verb answering with two envelopes or exit code 1 fails step (7).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Every `run` verb accepts `--estimate`, which returns the cost of admission, lowering, proposing and checking
  separately and runs nothing; checker time is a budget line of its own.
- Kill gate KILL-AUGUR-FALSECERT (a false certificate is accepted): the false-certificate procedure disables the
  proof kind, withdraws bundles and downgrades answers; the rows stay registered.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Rows read M1, M4 and M8; forms answer from M1 (reduced
  `mission.distinguish`), M3 (`mission.separate`, `mission.revalidate`) and M4 (the static core of
  `mission.transport`). The per-milestone surface beads (BD-137, BD-312, BD-376) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`read` → `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` →
  `<cluster>:write` or a named write scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope),
  plus `export:*` when the row exports and `private:*` where privacy applies, never a scope named after the
  object read or the engine spent on; N32b-01's catalogue generates the forms. `<cluster>:run` and
  `<cluster>:write` imply `<cluster>:read`, closed when a set is stored (at minting and at derivation,
  `Scopes::close_implied`), so checks stay plain set inclusion.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §25.3, §25.3.1, §25.3.2,
§25.8, §25.9, §25.11, §25.14, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
