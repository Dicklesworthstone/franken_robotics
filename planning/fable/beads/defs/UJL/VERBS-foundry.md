---
key: "VERBS-foundry"
title: "frob-verbs registry: cluster `foundry`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "tether", "foundry", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-62"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: ["N31-REF", "N31a-51", "N31ap3-51", "N31b-45", "N33c-32", "N34-24"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§31.5", "§31.11", "§31.12", "§31.15", "§31.16", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `foundry`. The cluster covers structure and manufacture: load
cases from telemetry, beam-rung sizing, topology optimization with the loads-mass loop, the independent check,
tolerance allocation, the build packet and, as a Moonshot, lifecycle runs. It has seven verbs (2 `read`, 5 `run`);
the handlers live in `frob-foundry`, `frob-lifecycle`. The bead carries every row completely, with the three checks
that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an end-to-end
script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the seven registry entries of cluster `foundry`, entered here and nowhere else; the MCP tool names; the cluster's
conformance script; the help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the
implementing beads named below); the behaviour common to the cluster with the real handlers behind the rows: its
contract, the named-fixture option `--fixture`, the script preamble and the reasons for the refusal codes added to
the plan's lists are the Foundry unit's surface bead N31a-51, which depends on this bead; the tests of that
contract through the real handlers (the bounded wait and the job handle with a progress denominator per verb,
idempotent replay, cost estimate and admission, cancellation at each verb's boundary, the envelope's evidence and
conditions, two worked examples per verb in the generated documentation) are N31ap3-51, for the six verbs whose
handlers exist at M5, with `foundry.lifecycle`'s share run in its feature lane by N31a-59 on N31ap3-51's library.
N31a-51 repeats the rows in a table and enters none; the two beads state the same kinds, schema names, scopes and
refusal lists. Also out:
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `foundry.check` | run | M5 | F | Independent stress, buckling, modal, fatigue and manufacturability check of a geometry | §31.15 |
| `foundry.export` | run | M5 | F | Write geometry and the build packet; `--with-firmware` under its feature | §31.12 |
| `foundry.lifecycle` | run | M8 | M | A lifecycle run under the feature `lifecycle` | §31.15 |
| `foundry.loads` | run | M4 | F | Run or read a load campaign; envelope cases, spectra, the free-body residual | §31.15 |
| `foundry.optimize` | run | M5 | F | Topology optimization of link classes with the loads–mass loop | §31.5 |
| `foundry.size` | read | M4 | F | Beam-rung sizing and the structural margin of A-15 | §31.15 |
| `foundry.tolerance` | read | M5 | F | Sensitivity-driven tolerance allocation, stacks and the band-extreme check | §31.11 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

Class ceiling of this cluster. A Foundry or lifecycle result is at most class Estimated, whatever its label says
(the chapter's invariant INV-FOUNDRY-06; the one exception is the firmware instance certificates listed inside an
export's result, which are Certified). The labels *static* and *run* below therefore give the tier and the
determinism of a block, and its class is Estimated: a first draft of this bead let `foundry.check`, `foundry.size`
and `foundry.tolerance` read class Tested through the *static* label, and `foundry.loads` through *run*, which the
invariant forbids and which disagreed with N31a-51.

**`foundry.check`** (MCP tool `foundry_check`; owner `frob-foundry`)
- Inputs (flag spellings fixed by the implementing beads named): the positional geometry (a content hash, a part
  of a design, a design with `--link <class>` for one link class, the flag of the registry example; or a packet
  `.fbuild` or firmware directory for the mechanical pre-review), or `--fixture <name>` in its place (the
  cluster's named fixtures, resolved through `crates/frob-foundry/tests/fixtures/index.toml`; a fixture and a
  positional together are refused; N31a-51); the load set, `--campaign <spec|id>` or a declared load set
  `--loads <file>` (N31b-16, N31b-36), restricted with `--cases <set>`; `--load <N>` (the magnitude in newtons of
  a fixture's single declared load case; N31a-19); the structural criteria `--stress`, `--modes` or `--all`
  (N31a-19); the halves `--structure` (the structural half only) and `--fab` (the manufacturability half only),
  both halves when neither is given (BD-405); `--process <card>` (the process card; a part with no declared
  process is a schema refusal; N31a-26); `--require fatigue` (a fatigue `no-claim` becomes `FROB-REF-163`;
  N31a-22); `--strict-data` (refuse a required card field, or a stress-life curve, of provenance `assumed`;
  N31a-22, N31b-27); `--for packet` (the referee check of fatigue spectra on keyed windows, N31b-36);
  `--exploratory` (a run on a design with no qualification verdict, labelled `exploratory`, which cannot enter
  a packet; N31b-36); `--review-record <file>` (import of the review record `frob.review/1` against a packet;
  N31b-33); `--firmware-receipt <file>` (under the firmware feature: import of the hardware timing receipt
  against a packet or firmware directory; N31a-62); the idempotency key `--key`.
- Outputs: stress, buckling, modal, fatigue and manufacturability results on an independent mesh; the governing case
  and location; for a packet, the pre-review items M1 to M6, each pass or fail with the offending file, and the
  manifest view (`review`, firmware receipts). Evidence: *static* in tier and determinism, class Estimated.
- Chapter refusals: `FROB-REF-163` (a check whose material card lacks the field or provenance it needs; also a
  required fatigue criterion that reads `no-claim`); `FROB-REF-261` (an optimized part that fails the independent
  check); and, added to the plan's list (DESIGN), `FROB-REF-860` (the argument is a packet without its flags),
  `FROB-REF-169` (a symmetric class's load set that does not cover every instance: the equal-limb rule of N31a-11
  step 6, naming the missing limbs), `FROB-REF-113` (under `--strict-data`, a required card field or a
  stress-life curve of provenance `assumed`, naming the card and field path; the program's strict-data code;
  N31a-22, N31b-27) and `FROB-REF-890` (the interim code of `firmware-receipt-mismatch` with `--firmware-receipt`,
  raised with `detail: "firmware-receipt-mismatch"` behind the named constant, row 9 of N31b-45; N31a-62).
- Airlock requirement: `needs_groups = ["structure"]` (the independent check runs on `fs-solid` and the
  manufacturability half on `fs-fab` and `fs-geocon`, all in the `structure` feature group of N33a-11; without it
  every wrapper refuses, N31a-04), so the generator appends `FROB-REF-790` (exit 5).
- Pending, uncoded (N31b-45's held rule, DESIGN of that bead): `review-record-packet-mismatch` with
  `--review-record` (row 4 of N31b-45; N31b-33) has no fitting interim: the import refuses with exit 3, nothing
  written, and a `detail` naming the refusal behind its named constant, and N31b-33 stays open on that point
  until the allocation ledger assigns the number, which then joins this list. (`firmware-receipt-mismatch`, row
  9, is raised meanwhile under its interim 890, listed above; when its number is assigned the constant changes
  and the list takes the new code in the same change.)
- Implemented by: BD-405 (manufacturability) and BD-402 (the independent stress, buckling, modal and fatigue
  check), with the component beads N31a-51 lists (N31a-19 to N31a-22, N31a-26 to N31a-28).
- Extended by: N31b-32 (the mechanical pre-review of a packet), N31b-33 (`--review-record`), N31b-36 (`--for
  packet`, `--exploratory`), N31b-27 (the fatigue admission of printed parts under `--strict-data`), N31a-62
  (`--firmware-receipt`, Moonshot, M8).

**`foundry.export`** (MCP tool `foundry_export`; owner `frob-foundry`)
- Inputs (flag spellings fixed by the implementing beads named): the positional design, or `--fixture <name>`
  in its place (N31a-51); `--packet <dir>` (the build packet's directory under the session's export root) with
  `--profile reduced` (the KILL-PACKET profile: `parts.json`, `mass.json` and `geometry/*.stl` only, forced when
  the kill switch is set; N31a-43, BD-408) and `--replace <identity>` (replace an existing packet of another
  content whose identity is named; without it a different packet at the target is a conflict, exit 7;
  N31a-36); or `--geometry-only` (loose geometry files, labelled, never a packet; N31a-34 step 7) with
  `--part <part>` (default all made parts), `--format step|stl|3mf|glb` (default all four), `--out <dir>` (the
  target directory under the session's export root, default the root itself) and `--exploratory` (loose
  geometry of a design with no qualification verdict, labelled; the only export that accepts it; BD-408);
  `--decimate <hausdorff tolerance>` (a part above the triangle bound is decimated under the stated two-sided
  Hausdorff tolerance and re-checked as a new geometry; N31a-35); `--priced` (prices on the parts list, from a
  pinned and licensed catalogue only); `--strict-data` (a priced list whose catalogue is marked `assumed` is
  refused with 818; N31a-39); `--with-firmware` (its own feature, Moonshot) with `--controller <name>` and
  `--firmware params-only` (the KILL-FIRMWARE fallback output on request: parameters as exact rationals, the
  Q-format table and the vector manifest, no `src/`; N31a-46 step 6); the idempotency key `--key`.
  Forbidden keys: the input schema `frob.foundry.export.in/1` declares the seven keys `execution_authorized`,
  `hardware_safety_signoff`, `sim_to_real_certified`, `fabrication_released`, `released`, `authorized` and
  `signoff` as a `forbidden_keys` set mapped to `FROB-REF-860`, tested at any depth before the unknown-parameter
  rule, so a request naming a flag answers 860 and not 802 (N31a-37; the ordering rule is N32a-04's).
- Outputs: geometry files and the build packet `.fbuild` with its four flags, all false (the three
  no-authorization flags and `fabrication_released: false`). Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-166` (optimization or packet export for a design with no qualification verdict);
  `FROB-REF-818` (a priced parts list without a pinned, licensed catalogue); `FROB-REF-824` (a STEP export above its
  bounds or failing bit-exact replay); `FROB-REF-827` (a fixed-point obligation refuted or undecided within budget;
  raised only with `--with-firmware`); `FROB-REF-860` (an export without the flags, or any attempt to produce a true
  flag); and, added to the plan's list (DESIGN), `FROB-REF-261` (a part that failed the independent check cannot
  enter a packet); `FROB-REF-169` (a symmetric class's load set that does not cover every instance); from the packet
  preconditions other beads register: `FROB-REF-820` (the interim code of `provisional-load-case-in-packet`: a
  provisional envelope case cannot enter a packet; N31b-36), `FROB-REF-830` (a `model-conditional` selection
  offered for prototype spend, which a build packet is) and `FROB-REF-819` (a `tier-sensitive` selection cited
  above its claim product; both N31ax-01).
- Export door (`exports = true`). The verb writes bytes outside the store, so it is routed through the one
  export door of N37-18 and its row carries the door's codes, as every exporting row does (N37-REF's list test
  requires 731, 733 and 816 of each): `FROB-REF-731` (the design or an asset it derives from is private, and
  the call lacks the private scope and flag); `FROB-REF-733` (the licence set of the artifact forbids this
  export: a `research-only`, `proprietary` or `unknown` asset in the design's lineage); `FROB-REF-816` (the
  door's flags check for an artifact that is not a packet: the loose geometry of `--geometry-only` handed over
  without its three no-authorization flags; for a packet the chapter's own 860 answers, since the chapter
  governs its own artifact, N31-REF); `FROB-REF-734` (the `--packet` directory or a geometry output path
  outside the session's export root, by the path jail). The publish scope the door checks is the row's
  `export:packet`.
- Pending, uncoded (N31b-45's held rule): `packet-inconsistent` (a cross-consistency or cross-export identity
  failure blocks the write; row 10 of N31b-45; N31a-43) has no fitting interim: the assembler refuses with exit 3,
  nothing written, and a `detail` naming the refusal behind its named constant, and N31a-43 stays open on that
  point until the ledger assigns the number, which then joins this list. `firmware-controller-not-exportable`
  (with `--with-firmware`: a controller whose kernel-IR program leaves the exportable set; a capability
  refusal, exit 5; N31a-44) is row 11 of N31b-45: filed in the ledger by N31b-45 in family `7xx`, subfamily `72x`,
  with no proposal and no interim; N31a-44 refuses with exit 5, nothing emitted, and `detail:
  "firmware-controller-not-exportable"` behind `FIRMWARE_CONTROLLER_NOT_EXPORTABLE`, keeps that point open until
  the ledger assigns the number, and the number then joins this list.
- Implemented by: BD-408 (the packet) and BD-407 (geometry export), with N31a-34 to N31a-43; firmware N31a-44 to
  N31a-46; the prototype-spend precondition N31ax-01.

**`foundry.lifecycle`** (MCP tool `foundry_lifecycle`; owner `frob-lifecycle`)
- Inputs (spellings fixed by N31a-50 step 6): the positional design, or `--fixture <name>` in its place (the
  cluster's named fixtures; the lifecycle toys of N31a-47 and the gate fixtures are entries of kind `lifecycle`
  in the index, each naming its duty and laws); duty cycle (`--duty <profile>`); horizon (`--horizon-h <H>`);
  `--ages <list>` (the ages at which aged model variants are written, default 0, ¼, ½ and 1 × horizon);
  `--laws wear,fatigue,insulation,cell` (the lifecycle laws); `--structural-candidates <set>` (the mass against
  service-duration front over a link class's candidates); `--rare-rates <file>` (rare-overload rates from
  adversarial search; without it the label `rare overloads: not included`); `--mode time-axis|hand-specified`
  (default `time-axis`); `--wear-states <file>` (a table of backlash per joint, f_Q and f_R; required with
  `hand-specified`, refused with any other mode by the unknown-argument rule); optional `--strict-data` (refuse
  a law resting on a card coefficient of provenance class `assumed`); the idempotency key `--key`.
- Outputs: a lifecycle run under the feature `lifecycle`: the threshold table, the first threshold, the aged
  variant set (a set handle of `ModelId`s that `eval.run` and `nemesis.search` accept as `--variants <set>`), the
  front when requested, the run record; class at most Estimated. Evidence: *screening*.
- Chapter refusals: `FROB-REF-831` (a lifecycle law outside its validity domain, or cited above class Estimated);
  and, added to the plan's list (DESIGN), `FROB-REF-169` (window traces a lifecycle metric needs are missing) and
  `FROB-REF-113` (under `--strict-data`, a law resting on a card field of class `assumed`; the design language's
  strict-data code, naming the card and field; N31a-50). The appended `FROB-REF-724` (exit 5) carries two
  statuses here (N31a-50): `not-in-build` when the binary lacks the feature `lifecycle`, and `killed` when the
  KILL-LIFECYCLE decision has fired and a time axis is requested (the reason, the fallback and its ADR named; the
  repair `--mode hand-specified`).
- Implemented by: N31a-50 with N31a-47 to N31a-49 (the work of the Moonshot epic BD-423, M6). Gate lane: N31a-59.

**`foundry.loads`** (MCP tool `foundry_loads`; owner `frob-foundry`)
- Inputs (spellings fixed by BD-400 step 8): the positional design, or `--fixture <name>` in its place (a
  registered known-answer fixture; N31a-51); runs or a campaign at telemetry level T2 (`--campaign <spec file |
  campaign id>`, run or read; N31a-06); `--tier <tier>` and `--precision f32|f64` (override the specification's
  tier for a parity run; `batch-cpu` runs in `f64` unless `--precision f32`); `--links <selector>`; `--variants
  all|nominal-only` (default `all`: the four contact variants; `nominal-only` labels every case `model-form:
  unchecked` and admits the result as exploratory only; N31a-12); `--transient impact-windows` (the transient
  path for impact windows when the frequency separation fails; N31a-12); `--declared statics [--dynamic-factor
  <k>] [--patterns <list>]` (declared load sets from statics with a stated dynamic factor, the KILL-LOADS
  fallback; N31a-13); `--for packet` (the referee
  re-extraction of every envelope case, rule 7 of the load cases: a replay window, both tier values, a label and a
  replay address per case; N31b-35); `--admit packet` (a read of the load set's admissibility for a packet, the
  same function `foundry.export` calls before it writes; N31b-36); the idempotency key `--key`.
- Outputs: envelope load cases (one instant each), load spectra, the free-body residual; with `--for packet` the
  re-extraction records; with `--admit packet` the admissibility verdict per case. Evidence: *run* in tier and
  determinism, class Estimated.
- Chapter refusals: `FROB-REF-167` (a load case that is not one instant, or a campaign failing the free-body check);
  `FROB-REF-169` (telemetry without the traces or samples a load metric needs); `FROB-REF-262` (quasi-static
  superposition where the frequency separation fails); and, from the trust-ladder steps of the load cases,
  `FROB-REF-821` (re-extraction with `--for packet` against a wrench envelope that is not frozen; N31b-35) and
  `FROB-REF-820` (the interim code of `provisional-load-case-in-packet` under `--admit packet`, raised with its
  `detail`; N31b-36). A campaign whose Chronicle chunk lacks samples is answered with 169, the upstream
  `FROB-REF-888` kept in the payload (N31ap1-51). The appended `FROB-REF-724` (exit 5) is also the KILL-LOADS
  answer: with the switch set, `--campaign` refuses with 724, a message naming the fallback and `--declared
  statics` as the offered repair (N31a-13).
- Implemented by: BD-400 with N31a-06 to N31a-13. Extended by: N31b-35 (`--for packet`), N31b-36 (`--admit
  packet`).

**`foundry.optimize`** (MCP tool `foundry_optimize`; owner `frob-foundry`)
- Inputs (spellings fixed by BD-403 step 9): the positional design, or `--fixture <name>` in its place
  (`twobar`, `planted-gain`, the extraction battery of N31a-18 and the other registered fixtures; N31a-51); link
  classes (`--links <class>[,<class>…]`, default all classes); the load campaign (`--campaign <spec|id>`) or a
  declared load set (`--loads <file>`); `--rung beam|topology` (default `topology` when the `structure` group is
  compiled and KILL-TOPOPT is not in force, otherwise `beam`, the defaulted choice named in the result with its
  reason); `--mode coarse|finalist` (default `coarse`); `--plate` (shape a plate part in two dimensions;
  N31a-16); `--process <card>` per part; `--loop` (the loads–mass loop of BD-404) with `--outer actuators` (the
  outer actuator re-selection with revalidation, run only on request; N31a-25); `--strict-data` (N31a-22); a
  qualification verdict or `--exploratory`; the idempotency key `--key`.
- Outputs: optimized link geometry with the loads-mass loop record and convergence figure. Evidence: *screening*.
- Chapter refusals: `FROB-REF-164` (an empty design region, or ports with no load path inside the envelope);
  `FROB-REF-166` (optimization or packet export for a design with no qualification verdict); `FROB-REF-260` (a
  loads–mass loop that diverges, cycles or exceeds its iteration budget); and, added to the plan's list (DESIGN),
  `FROB-REF-261` (an optimized part that fails the independent check), from the campaigns the loop runs,
  `FROB-REF-167`, `FROB-REF-169` and `FROB-REF-262` with the meanings given under `foundry.loads`, and
  `FROB-REF-113` (under `--strict-data`, a required material-card field of provenance `assumed`; N31a-22).
- Airlock requirement: `needs_groups = [{ groups = ["structure"], when = "foundry-rung-topology" }]` (DESIGN
  predicate name): the topology rung runs on `fs-topopt` and `fs-topols`; an explicit `--rung topology` in a
  binary without the group is refused with the appended `FROB-REF-790` (exit 5), while a defaulted rung falls
  back to `beam` with the reason `feature-absent` (BD-403).
- Implemented by: BD-403 (the optimization) and BD-404 (inertia feedback and the convergence test), with N31a-05,
  N31a-14 to N31a-17 and N31a-23 to N31a-25.

**`foundry.size`** (MCP tool `foundry_size`; owner `frob-foundry`)
- Inputs (BD-401 step 6): the positional design, or `--fixture <name>` in its place (`cantilever`,
  `cantilever-thinned`, `assembled-peaks` and the other registered fixtures; N31a-51); a link selector; the load
  set (BD-401 step 6): `--loads <campaign id | file>` (the envelope cases of a load campaign, or a declared load
  set), `--loads statics` (the bounded statics set) or `--loads statics:<factor>` (the same set with one load
  factor for every port); with no `--loads`, the bounded statics set of the stage-6 member-sizing check (N31b-39)
  with the class load factors of the design's archetype card (each foot, wheel or end-effector port carries the
  robot's weight times 3 for legged feet, 2 for wheels, 1.5 for manipulator ports with the rated payload added,
  in the worst direction the port admits), labelled `load set: bounded-statics` with the factors used, so the
  registry example runs as printed; a fixture brings its own declared case and needs no `--loads`;
  `--load <N>` (the magnitude in newtons of a fixture's single declared load case;
  N31a-19); the section family and safety factors; `--fe` (the full finite-element form); `--diagram` (the
  section-force diagram along each member; N31a-60); `--strict-data` (N31a-22). A `read`: it takes no key and
  needs no qualification verdict (166 is not on its list).
- Outputs: beam-rung sizing per link and the structural margin of analysis A-15. Evidence: *static* in tier and
  determinism, class Estimated.
- Chapter refusals: `FROB-REF-163` (a check whose material card lacks the field or provenance it needs); and, added
  to the plan's list (DESIGN), `FROB-REF-167` (a load set containing a case that is not one instant),
  `FROB-REF-169` (a symmetric class's load set that does not cover every instance, naming the missing limbs: the
  equal-limb rule of N31a-11 step 6, which the loads script N31a-53 asserts through this verb) and `FROB-REF-113`
  (under `--strict-data`, a required material-card field of provenance `assumed`; N31a-22).
- Airlock requirement: `needs_groups = [{ groups = ["structure"], when = "foundry-size-fe" }]` (DESIGN
  predicate name): the beam form answers in any build, and `--fe` (on `fs-solid`) without the `structure` group
  is refused with the appended `FROB-REF-790` (exit 5; BD-401).
- Implemented by: BD-401. Extended by: N31a-60 (`--diagram`).

**`foundry.tolerance`** (MCP tool `foundry_tolerance`; owner `frob-foundry`)
- Inputs (BD-406, spellings of N31a-31 to N31a-33): the positional design, or `--fixture <name>` in its place
  (N31a-51); the quantities of interest (`--qoi <name>`) with their specification margins and targets (the target
  metric bands); cost coefficients (default equal); `--k <k>` (default 3); correlation groups; `--method
  auto|statistical|worst-case` (default `auto`: the statistical allocation with worst-case boxes for the
  robust-path features; `statistical` refuses rather than boxes; `worst-case` applies the worst-case closed form
  to every feature; N31a-32); `--verify monte-carlo [--samples <N>] --seed <s>` (the held-out coverage check;
  N31a-33). A `read`: it creates no job and writes no file (the report reaches disk only inside a packet).
- Outputs: sensitivity-driven tolerance allocation, stacks, the band-extreme check. Evidence: *static* in tier and
  determinism, class Estimated.
- Chapter refusals: `FROB-REF-165` (an allocation with a zero or unknown sensitivity, or a failed band-extreme
  check).
- Airlock requirement: `needs_groups = ["structure"]` (the allocation runs on `fs-toleralloc`, in the `structure`
  group of N33a-11), so the generator appends `FROB-REF-790` (exit 5).
- Implemented by: BD-406 with N31a-31 to N31a-33.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `foundry.check` | `foundry:run` | `frob.foundry.check.out/1` | `make`, `structure` | `frob foundry check quad-a --link thigh --key fc-1 --json` |
| `foundry.export` | `foundry:run`, `export:packet` | `frob.foundry.export.out/1` | `make` | `frob foundry export quad-a --packet out/quad-a --key fe-1 --json` |
| `foundry.lifecycle` | `foundry:run` | `frob.foundry.lifecycle.out/1` | `structure` | `frob foundry lifecycle quad-a --duty kitchen-shift --key lc-1 --json` |
| `foundry.loads` | `foundry:run` | `frob.foundry.loads.out/1` | `structure` | `frob foundry loads quad-a --campaign c:14 --key fl-1 --json` |
| `foundry.optimize` | `foundry:run` | `frob.foundry.optimize.out/1` | `make` | `frob foundry optimize quad-a --links thigh,shank --key fo-1 --json` |
| `foundry.size` | `foundry:read` | `frob.foundry.size.out/1` | `make`, `structure` | `frob foundry size quad-a --json` |
| `foundry.tolerance` | `foundry:read` | `frob.foundry.tolerance.out/1` | `make`, `structure` | `frob foundry tolerance quad-a --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. `foundry.export` has `exports = true` (it writes the packet
or loose geometry outside the store, through the export door of N37-18); no other row of the cluster has it.
`needs_groups` is `["structure"]` on `foundry.check` and `foundry.tolerance`, the predicate form stated in the
blocks on `foundry.size` (`--fe`) and `foundry.optimize` (`--rung topology`), and empty on `foundry.loads`,
`foundry.export` and `foundry.lifecycle` (a loads build without the transient path answers 262, not 790,
N31a-12; the lifecycle feature's absence is 724, N31a-50). The two predicates, `foundry-size-fe` and
`foundry-rung-topology`, are registered in `frob_verbs::admission::predicates` by the beads that implement the
options (BD-401, BD-403), which the lint `needs-groups-known` of N33c-32 requires. The fields `needs_groups`
(N33c-32, M0) and `params` (N34-24, M1) exist in the closed entry schema before this M4 bead enters its rows;
both are linked as related, not as dependencies, because a registry bead depends only on the registry skeleton.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "foundry.check"
cluster = "foundry"
kind = "run"
tier = "F"
capabilities = ["foundry:run"]
input_schema = "frob.foundry.check.in/1"
output_schema = "frob.foundry.check.out/1"
refusals = ["FROB-REF-113", "FROB-REF-163", "FROB-REF-169", "FROB-REF-261", "FROB-REF-860", "FROB-REF-890"]   # 890: interim of row 9; row 4 enters when the ledger numbers it
needs_groups = ["structure"]
# params: the packet positional and --loads, --review-record, --firmware-receipt (file paths), as stated under the conventions
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M5"
task_clusters = ["make", "structure"]
examples = ["frob foundry check quad-a --link thigh --key fc-1 --json"]
owner = "frob-foundry"
purpose = "Independent stress, buckling, modal, fatigue and manufacturability check of a geometry"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator (NJL-01) adds the common input properties of the kind. `run`: `sync_wait_ms` (0 to the row's
   `wait.mcp_max_ms`, at most 30,000; MCP default 5,000; the CLI waits to completion), `idempotency_key` (CLI
   `--key`), `budget`, `estimate_only` (CLI `--estimate`), `on_cancel` (`leave`, the default, or `cancel_job`;
   `leave_running` is 802) and `envelope_scope` (CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`: the
   approved compute-envelope scope the run belongs to; a launcher's token comes first and a citation that
   contradicts it is refused; with neither, nor a scope the verb derives from its own inputs, the run is `adhoc`,
   class E0; N36-20). `mutate`: `idempotency_key` and `expected_parent`. `idempotency_key` is in no JSON-schema
   `required` array: the annotation `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the
   rule, and the admission transaction refuses a missing key with 805 on both surfaces. The generator appends the
   surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build)
   and 802 (schema failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761
   (above the compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`;
   725, 803, 804 (stale parent) and 805 for `mutate`. The rest is NJL-01's table of variations driven by fields
   and tables, cited here and not copied: among them 806 and 807 (with `cursor`, `limit`, `follow`) for a
   paginated read; 808 (an integer outside ±(2⁵³ − 1)) for every entry whose input has an `int53` field, whatever
   its kind (every `run` entry has one, `sync_wait_ms`); no 805 for a `key_optional` entry; `estimate_only` on a
   non-`run` entry with `estimable = true`; the property `action` for an entry with `actions`; 790 for a non-empty
   `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a path to a file
   (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`); and the later variations 10 to 16 (`override`
   with `override_reason`, `from_proposal`, `machine`, `engine`, `no_memo` for `memo = true`, the reality guard's
   refusals for `reports = true` or an artifact reader, `diagnostic_override` with the codes of the
   `[[admission_check]]` rows), each landing with the bead NJL-01's table names.
   In this cluster the inputs that accept a path to a file are `params` entries of kind `artifact-input<F>` for
   the format the file holds, so their rows get 891, 734 and 828: `foundry.check` (`--loads`,
   `--review-record`, `--firmware-receipt`; and its positional when it names a packet directory, of kind
   `artifact-identity<fbuild>`, which adds 892: a view offered as a packet's identity), `foundry.optimize` and
   `foundry.size` (`--loads` given as a file), `foundry.loads` (`--campaign` given as a specification file) and
   `foundry.lifecycle` (`--rare-rates`, `--wear-states`). `foundry.export`'s `--packet` and `--out` are export
   targets, whose 734 its own row lists with the export door's codes; `foundry.tolerance` takes names only.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x, 79x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M4); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Seven verbs, the limit for one registry cluster of this chapter.
- Refusal codes. The chapter's verb table prints chapter-local numbers that predate the refusal registry (160 to
  165, 260 to 262, 860 to 864). The registry codes used here are those of the chapter's refusal table, matched by
  meaning: 160 → `FROB-REF-166`, 161 → 167, 162 → 169, 163 → 163, 164 → 164, 165 → 165, 260 → 260, 261 → 261, 262 →
  262, 860 → 860, 861 → 818, 862 → 824, 863 → 827, 864 → 831.
- Refusal lists beyond the plan's. The plan's verb table gives, after that renumbering, `foundry.loads` 167, 169,
  262; `foundry.size` 163; `foundry.optimize` 166, 164, 260; `foundry.check` 163, 261; `foundry.tolerance` 165;
  `foundry.export` 166, 860, 818, 824, 827; `foundry.lifecycle` 831. The rows here add the codes marked DESIGN in
  the blocks above (167, 169 and 113 on `foundry.size`; 261, 167, 169, 262 and 113 on `foundry.optimize`; 860,
  169, 113 and 890 on `foundry.check`; 261, 169, 820, 830 and 819 on `foundry.export`; 169 and 113 on
  `foundry.lifecycle`; 821 and 820 on `foundry.loads`), because the implementing beads raise them and a verb must
  not return a refusal its row does not list. The reasons are given in N31a-51, which first proposed most of them,
  in N31a-22 (113 under `--strict-data` on the three verbs that read material cards), and in the chapter refusal
  registry N31-REF, whose Table 2 is compared with these rows code by code. The held refusals follow N31b-45's
  rule: a held row with a fitting interim is listed by its interim code (820 for row 3 on `foundry.loads` and
  `foundry.export`; 890 for row 9 on `foundry.check`), and the rows with none (`review-record-packet-mismatch` on
  `foundry.check`, `packet-inconsistent` on `foundry.export`, and `firmware-receipt-mismatch`'s successor number)
  join the lists with their numbers when the ledger assigns them; when an interim is retired the row's list
  takes the assigned code in the same change. `firmware-controller-not-exportable` on `foundry.export`
  (N31a-44), row 11 of N31b-45 (`72x`, exit 5, no interim), is held the same way.
- Options. Every option a mechanism bead defines for a verb of this cluster is named in that verb's block, with
  the bead that fixes its spelling; an end-to-end script calls only these (KNOWN_DRIFTS 8). The named-fixture
  option `--fixture <name>` is on every row (N31a-51).
- Every exported artifact carries the no-authorization flags, all false, and a build packet adds a fourth,
  `fabrication_released: false`; an export without the flags, or any attempt to produce a true flag, is refused
  (`FROB-REF-860`).
- Milestones as the inventory prints them: `foundry.loads` and `foundry.size` M4, the rest M5, `foundry.lifecycle`
  M8. The seed beads place `foundry.size` and `foundry.loads` at M5 (BD-401, BD-400) and the lifecycle epic at M6
  (BD-423); until a verb's bead lands it answers the not-implemented refusal. `foundry.size` and `foundry.loads`
  are declared late rows of NJL-02's closed list `LATE_ROWS` (first milestone M4, lands M5): from M4 to the
  milestone before M5 the coverage check accepts their `FROB-REF-724` answer and records the level `late-724`
  with the bead; from M5 the ordinary rule applies.
- The export door's codes on `foundry.export` (731, 733, 816 and 734) are the safety chapter's, defined in
  N37-REF and raised by N37-18; N31-REF names 816 among the codes of other chapters that Foundry verbs raise
  (its Table 4), not in its Table 2, so the Table 2 comparison below excepts these four.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "foundry"` (2 `read`, 5 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster foundry` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_foundry.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/foundry/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_foundry` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `foundry.tolerance` is `read` at M5).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`foundry.check`: 113, 163, 169, 261, 860, 890;
  `foundry.export`: 166, 169, 261, 731, 733, 734, 816, 818, 819, 820, 824, 827, 830, 860, with `exports = true`;
  `foundry.lifecycle`: 113, 169, 831;
  `foundry.loads`: 167, 169, 262, 820, 821; `foundry.size`: 113, 163, 167, 169; `foundry.optimize`: 113, 164, 166,
  167, 169, 260, 261, 262; `foundry.tolerance`: 165); the appended codes equal the rule for its kind and fields
  (790 on `foundry.check`, `foundry.tolerance`, `foundry.size` and `foundry.optimize`; 891, 734 and 828 on the
  rows with file `params`, and 892 on `foundry.check`).
- `table2_equality`: the seven lists equal N31-REF's Table 2 code for code (113 on `foundry.size`,
  `foundry.check`, `foundry.optimize` and `foundry.lifecycle` and 890 on `foundry.check` included), two kinds of
  entry excepted: the uncoded names (`review-record-packet-mismatch`, `packet-inconsistent`,
  `firmware-controller-not-exportable`) while their numbers are held or not yet assigned; and the four
  export-door codes of `foundry.export` (731, 733, 734, 816), as codes of another chapter (N31-REF's Table 4). A
  planted removal of 821 from `foundry.loads` fails with the verb and the code, and so does a planted removal of
  113 from `foundry.size`.
- `needs_groups_rows`: `foundry.check` and `foundry.tolerance` carry `["structure"]`, `foundry.size` and
  `foundry.optimize` the predicate forms `foundry-size-fe` and `foundry-rung-topology`, the other three none; each
  of the four has 790 in its generated list and the other three do not; a planted `needs_groups = ["gpu-cuda"]`
  fails `needs-groups-known`.
- `export_forbidden_keys`: the generated `frob.foundry.export.in/1` declares the seven flag keys as
  `forbidden_keys` mapped to 860; a request `{"released": true}` nested one level deep is refused with 860 and
  not 802 (the ordering rule of the request pipeline, N32a-04, M0: a schema-declared forbidden key answers with
  its declared code ahead of 802 at every depth; N31a-37's handler carries no second check).
- `held_refusals_follow_n31b45`: the interim list served by `frob --json schema --refusals` (VERBS-operator's form) names 820 for
  `provisional-load-case-in-packet` and 890 for `firmware-receipt-mismatch`, each with its `detail`, and lists
  `review-record-packet-mismatch`, `packet-inconsistent` and `firmware-controller-not-exportable` (exit 5) with no
  interim; each interim code is in the list of the verb that raises it.
- `export_door_row`: `foundry.export` is the only row of the cluster with `exports = true`, and it lists 731,
  733 and 816 (N37-REF's list test) and 734; a planted `exports = true` on `foundry.check` without the three
  codes fails.
- `class_ceiling_estimated`: no row of the cluster declares an evidence class above Estimated.
- `mcp_names_one_to_one`: the tool names are the mapped names (`foundry_check`, `foundry_export`,
  `foundry_lifecycle`, `foundry_loads`, `foundry_optimize`, `foundry_size`, `foundry_tolerance`), with no collision
  against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`,
  `budget`, `estimate_only`, `on_cancel`, `envelope_scope`) and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_foundry.sh` (header: `bead=VERBS-foundry milestone=M4 labels=tether,verbs,verb:foundry.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field,dropped-option`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance foundry`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
foundry` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster foundry`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1; (8) per verb, `frob <verb> --help --json` lists
every option its block names (`--fixture` on all seven; for example `--variants`, `--transient`, `--declared` on
`foundry.loads`, `--outer` on `foundry.optimize`, `--replace`, `--profile`, `--decimate`, `--firmware` on
`foundry.export`), logged as `options_expected` and `options_found` with the first missing name on failure; and,
once `foundry.export` is implemented, a call carrying a nested `released: true` key (its own key) answers
`e2e_expect_refusal FROB-REF-860 3`, never 802.

Log: `target/e2e-logs/verbs_foundry/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (2 `read`, 5 `run`); the tables above. KA-755 (the exit-code table on
  malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's
  tables; exact) is exercised for every verb by steps (6) and (7).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`);
  `dropped-option` (`--outer` removed from `foundry.optimize`'s input schema: step (8) names it). A verb answering
  with two envelopes or exit code 1 fails step (7).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Optimization or packet export for a design with no qualification verdict is refused (`FROB-REF-166`) unless
  `--exploratory` is given, and an exploratory result cannot enter a packet.
- `foundry.export --with-firmware` is a Moonshot option under its own feature; it is an option of the verb, not a
  separate row.
- Closing rule: the bead closes at M4, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M4, M5, M8. The
  per-milestone surface beads (BD-376, BD-409) re-run this script as verbs land; that is how GATE-TETHER-02 is
  re-measured.
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

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §31.5, §31.11, §31.12, §31.15, §31.16, §32.4,
§32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
