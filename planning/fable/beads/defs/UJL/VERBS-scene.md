---
key: "VERBS-scene"
title: "frob-verbs registry: cluster `scene`, thirteen rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "tether", "habitat", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-43"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: ["N19-REF", "N19c-42", "N34-24"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§19.3", "§19.4", "§19.5", "§19.6", "§19.7", "§19.8", "§19.11", "§19.12", "§19.13", "§19.14", "§19.15", "§37.6", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `scene`. The cluster covers scenes: the catalog, compiling a
pack from source geometry, generating terrain and interiors, importing assets and real places, acceptance, hazards,
clearance fits, retrofits, splits for train and test hygiene, the world bank on a device, probe requests and purging
a private pack. It has thirteen verbs (5 `read`, 6 `run`, 1 `mutate`, 1 `admin`); the handlers live in `frob-scene`,
except that of `scene.purge`, which lives in `frob-verbs` (N37-19).
The bead carries every row completely, with the three checks that keep the rows true: the generated CLI commands and
MCP tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is
stable.

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

In: the thirteen registry entries of cluster `scene`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. This bead is the only one that enters rows of
this cluster: the Habitat unit's family bead N19a-02 was drafted with twelve of the same rows and now enters
none. Out: behaviour (the implementing beads named below); Habitat's handler map and dispatch table, the test
that handlers and rows agree one to one, and the progress model of the four verbs that can run long (N19a-02,
which depends on this bead);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `scene.accept` | run | M3 | S | The acceptance report of a pack | §19.5 |
| `scene.bank` | read, admin | M3 | S | The resident set on a device, an admission estimate, eviction | §19.15 |
| `scene.compile` | run | M2 | S | A pack from source geometry, with compile receipts | §19.4 |
| `scene.fit` | run | M3 | F | A feasibility witness, an impossibility cut or `unresolved` for a footprint family and a route | §19.11 |
| `scene.gen` | run | M2 | S | A pack from (family, parameters, seed): terrain at M2, interiors at M3 | §19.6, §19.7 |
| `scene.hazards` | read | M3 | S | The hazard registry of an archetype with the provenance of every dimension | §19.8 |
| `scene.import` | run | M3 | F | A pack with a loss report, a scale label and an unknown mask: assets at M3, real places at M5 | §19.13 |
| `scene.list` | read | M2 | S | The scene catalog | §19.3 |
| `scene.probe-request` | read | M5 | F | The measurements that would settle an unresolved answer | §19.13 |
| `scene.purge` | admin | M3 | S | Remove a private scene pack and the lineage closure of everything derived from it | §37.6 |
| `scene.retrofit` | run | M3 | F | The cheapest building amendment that flips a failing verdict, beside the cheapest redesign | §19.12 |
| `scene.show` | read | M2 | S | A manifest, the hazards drawn and the feasibility tag | §19.3 |
| `scene.split` | mutate, read | M3 | S | A split over scene groups, with feasibility counts | §19.14 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *operational*: no claim: an operational answer about the session, the store, a job or a device; `evidence.claims`
  is empty.
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.

**Worded refusals of this cluster.** Six rows of the decision bead N19c-42 (its rows 4 to 9, all raised by
handlers of this cluster) have their numbers assigned by the allocation ledger (`registries/refusal_allocations.toml`,
N35-33, N35x-02), not typed by any bead. A row below cites rows 4 and 7 to 9 by slug, and rows 5 and 6 by the numbers
N19c-42 proposes and N35x-02 holds for it (`FROB-REF-324`, `FROB-REF-325`; N19c-42 step 3 allows this), each raised
through its constant of `crates/frob-scene/src/refusal/late.rs`; the row's `refusals` field receives the assigned
number in the change that enters the registry row (N19c-42, M2), as VERBS-report does for row 9. Until a row is
registered its code does not compile, so no handler can raise it and the conformance rule "a verb raises only codes
of its list" holds at every step. The withdrawn numbers 323 and 326 are cited nowhere.

| Slug (N19c-42 row) | Constant | Family, exit | Rows that list it |
|---|---|---|---|
| `scene-pack-identity-mismatch` (4) | `IDENTITY_MISMATCH` | `9xx`, 9 | `scene.accept`, `scene.bank`, `scene.compile`, `scene.fit`, `scene.gen`, `scene.list`, `scene.show` |
| `scene-split-group-unmatched` (5; `FROB-REF-324`, proposed) | `SPLIT_GROUP_UNMATCHED` | `3xx`, 3 | `scene.split` |
| `scene-split-group-cited` (6; `FROB-REF-325`, proposed) | `SPLIT_GROUP_CITED` | `3xx`, 3 | `scene.split` |
| `scene-split-count-mismatch` (7) | `SPLIT_COUNT_MISMATCH` | `9xx`, 9 | `scene.split` |
| `spawn-region-no-admissible-pose` (8; cited by slug) | `SPAWN_NO_ADMISSIBLE_POSE` | `3xx`, 3 | `scene.compile`, `scene.gen` |
| `retrofit-cost-on-assumed-prices` (9) | `RETROFIT_COST_ASSUMED_PRICES` | `3xx`, 3 | `scene.retrofit` |

A fifth worded refusal is BD-227's `hazard-registry-invalid` (a hazard registry file that fails to load; a ledger
request in `3xx`), raised behind the constant `HAZARD_REGISTRY_INVALID` on the interim code `FROB-REF-802` until
the ledger assigns it; `scene.hazards` and `scene.gen` (interior families) list its number from then. Four more
are cited by slug in the same way. N19b-02's `asset-source-not-default-pipeline` (a draw or an import from an
asset-source row with `default_pipeline = false` without `--user-supplied`; family `3xx`, exit 3, nothing written;
ranked repairs: re-run with `--user-supplied`, with `--licence-ack` for a `research-only` or `proprietary` row;
choose a default-pipeline source) is raised behind the constant `ASSET_SOURCE_NOT_DEFAULT` of
`crates/frob-scene/src/refusal/late.rs`, in the ledger's interim form `FROB-REF-802` with `detail:
"asset-source-not-default-pipeline"`; `scene.import` (both forms) and `scene.gen` (the populated-interior draw of
`--populated`, BD-399, M5) list it. `import-articulation-interpenetrates` (the ledger row of N35x-02 for the joint
sweep of the articulated import profiles of NEG-24 and NEG-20; no interim code) is listed by `scene.import`. N19b-44's
`scene-edit-invalid` (an edit list of the derived form of `scene.compile` that cannot be applied to its base:
`detail` one of `UnresolvedTarget`, `ParamOutOfRange`, `Conflict` or `UndeclaredAspect`, `pointer` the offending
edit; family `3xx`, exit 3) is raised behind the constant `SCENE_EDIT_INVALID` of the same file; `scene.compile`
lists it. NEG-23's `scan-profile-absent` (a consumer would read the geometry of a scan pack imported without its
scan profile, `provenance.scan_profile = absent`; family `3xx`, exit 3; `pointer` the pack and
`provenance.scan_profile`; ranked repairs: re-import the scan with the scan profile, from M5 `--kind place
--profile hm3d-scan`; use another scene) is raised by the pack reader's geometry accessors behind the constant
`SCAN_PROFILE_ABSENT`, with the ledger's interim `FROB-REF-305` and `detail: "scan-profile-absent"` only in a
build that predates the allocation; the rows of this cluster that read a pack's geometry list it: `scene.compile`
(the derived form on a scan base), `scene.fit` and `scene.show` (its `--probe`, `--route` and `--corridor`
forms). Reading a scan pack's manifest, licence entry and loss report (`scene.show` without those forms,
`scene.list`) is not refused (NEG-23). Row 2 of
N19c-42 (`scene-variant-not-resident`, proposed 319) is on no row of this cluster: its only emit sites are job
admission and the reset of a run (N19c-22), so it belongs to the run verbs' rows, as N19c-42's step 6 states
(DESIGN, the rule VERBS-control applied when it removed a code its handler cannot raise).

The pack reader's decode refusal `FROB-REF-303` (N19-REF: stage 1 or any pack read; a stored pack that cannot be
decoded as written, BD-162: a missing or mistyped manifest member, a blob that is missing or over the import
profile's decode limits, a stored value outside its stated range, a name that does not resolve, a count that
differs from the manifest's) is listed by the seven rows that list `scene-pack-identity-mismatch`, which are the
rows that open a stored pack through that reader, and by `scene.import`.

**Parameters that accept a path to a file** (NJL-01's `params` rule appends `FROB-REF-734`, the path jail, and
`FROB-REF-828`, the dispatcher's sniff of N37-03, to every such parameter; N34-24 fixes the kind). Each is declared
in the row's `params` field with the kind given here. DESIGN where the implementing bead did not name the kind: an
asset, a scan, a constructive-solid source (`*.src`, which is not a registered format) or a plain list is
`foreign-file` (appends 893); a document of this project with its own schema is `artifact-input<F>` of that format
(appends 891), and the bead that defines the document enters the format's row in `registries/formats.toml`, which
N34-27's rule `fmt.param-without-row` requires of every schema a verb parameter names: BD-165
(`frob.scene.grid/1`), N19b-44 (`FMT-FEDIT`, `frob.scene.edit/1`), N19c-06 (`FMT-FSURVEY`, `frob.scene.survey/1`)
and N21c-09 (`frob.scene.recipe/1`).

| Row | Parameter | Kind |
|---|---|---|
| `scene.accept` | `source` (preview form) | `foreign-file` |
| `scene.bank` | `recipe` | `artifact-input<frob.scene.recipe/1>` |
| `scene.compile` | `source` (positional); `edits` (a list of N19b-44's `frob.scene.edit/1` edits) | `foreign-file`; `artifact-input<frob.scene.edit/1>` |
| `scene.gen` | `grid` (BD-165's curriculum grid specification, schema `frob.scene.grid/1`); `recipe`; `survey` | `artifact-input<frob.scene.grid/1>`; `artifact-input<frob.scene.recipe/1>`; `artifact-input<frob.scene.survey/1>` |
| `scene.hazards` | `uncovered_by`, `incidence` (the `@<file>` form of the scene list) | `foreign-file` |
| `scene.import` | `path` (positional); `licence_ack`; `landmarks`; `declare` | `foreign-file` each |
| `scene.show` | `source`; `probe` (the `<points.json>` form) | `foreign-file` each |

The other six rows take identifiers and names only.

**`scene.accept`** (MCP tool `scene_accept`; owner `frob-scene`)
- Inputs (BD-228, which fixes the spellings): the scene pack (positional `<scene>`); `--only <checks>` (names from
  one closed list: `registration`, `enclosure-selftest`, `deviation`, `maps`, `feasibility`, `board`, `gauges`,
  `normals`, `support`, `silhouette`, `ripple`, `continuity`, `leak`; an unknown name is `FROB-REF-802`; a subset
  yields a report marked incomplete); `--tiers <list>` (of `referee`, `batch-cpu`, `cuda`, `metal`: the tiers whose
  measurement part is measured); `--waive region=<id> --reason <text>` (only the collision-to-visual deviation may
  be waived, and only here); the preview form `--source <path> --preview [--ray <ox,oy,oz>/<dx,dy,dz>]…` in place
  of a pack (exclusive with `<scene>`, `FROB-REF-802` otherwise), which runs the four-point registration and the
  first hit of each ray on uncompiled geometry and yields a report that is never complete, never stored beside a
  pack and binds no run; idempotency key; budget (the bounded wait is the generated `sync_wait_ms`,
  `--sync-wait-ms` on the command line).
- Outputs: the acceptance report: every section and fixture with its value and threshold, waivers, completeness
  (identity part and per-tier measurement part). Exit 0 complete with nothing listed, 2 with findings or waivers, 3
  on a refusal, 8 still running. Evidence: *static*.
- Chapter refusals: `FROB-REF-312` (a realism claim cites a scene with `assumed` dimensions); `FROB-REF-317`
  (collision-to-visual deviation exceeds the pack threshold with no waiver: raised here and never by
  `scene.compile`, BD-228 item 1); `FROB-REF-302` (the preview form on a reflected or non-rigid source, as compile
  stage 2 refuses it); `FROB-REF-303` (the pack read cannot be decoded as written: the pack reader's code,
  BD-162); `scene-pack-identity-mismatch` (the recomputed identity part's digest differs from the
  manifest's `acceptance` field: the pack is corrupt, never re-labelled; BD-228 item 4).
- Implemented by: BD-228.

**`scene.bank`** (MCP tool `scene_bank`; owner `frob-scene`)
- Inputs (BD-236, which fixes the spellings): `--device <D>` (a device key of the hardware fingerprint: `cpu`,
  `cuda:0`, `metal:0`; an unknown key is `FROB-REF-802`; required by the admin actions `admit`, `evict`, `pin` and
  `unpin`, which are `FROB-REF-802` without it, so that nothing is admitted to or evicted from a device the
  caller did not name; optional for `show` and `estimate`, which without it answer for every device of the
  fingerprint, one block per device in fingerprint order); the action, selected only with `--action <name>` (NJL-01
  action rule 1), the scene ids being positional inputs: the default, `show` (read), is the call with no
  `--action` (`frob scene bank --device D`) and has no name to pass; `--action estimate <scene_id>… | --recipe
  <file>…` (read); `--action admit <scene_id>… | --recipe <file>… [--allow-evict]`, `--action evict <scene_id>… |
  --free <bytes>`, `--action pin <scene_id>…` and `--action unpin <scene_id>…` (admin, scope `scene:admin`), for
  example `frob scene bank --action admit s:kitchen-17 --device cpu`. An action word in a positional place (`frob
  scene bank admit …`) is `FROB-REF-802`. `--segments` on the default call adds the per-segment statistics and the
  cross-segment duplication (N19c-20). `--recipe <file>` names a `frob.scene.recipe/1` record (N21c-09): the bank
  uses the stored pack whose recomputed identity equals the recipe's `expected`, or regenerates it locally through
  `scene.gen --recipe`'s path and admits it only if the identities agree (N19c-23). Admin actions take no
  idempotency key: each is idempotent by effect (admitting a resident pack, evicting an absent one, a repeated pin,
  an unpin of an unpinned pack are no-ops reported as such, with a ledger row marked no-op). The read action is
  the default `show` (BD-236, and N19c-14, N19c-20, N19c-21 and N19c-24, which call it); `status` is the name of the
  variant of its output and not an action, so `scene bank status`, `--action status` and `--action show` are
  `FROB-REF-802`.
- Outputs: one schema `frob.scene.bank.out/1`, a tagged union whose `kind` is `status` (`show`: per resident pack
  `scene_id`, family, spacing, marginal and shared bytes, pins, last use, compile seconds; totals; the cap, the
  measured working-set limit, headroom; deduplication), `estimate` (per pack the estimate in order with the
  cumulative headroom and which would be refused) or `change` (`admit`, `evict`, `pin`, `unpin`: what changed, the
  ledger rows, and `not_evicted` with reason `pinned` and each holder; an eviction short of its target exits 2).
  Evidence: *operational*.
- Chapter refusals: `FROB-REF-314` (world-bank admission would exceed the device cap; nothing is allocated);
  `FROB-REF-303` (a stored pack that cannot be decoded as written; with `--recipe`: the local generator pin or
  compiler version differs from the recipe's, N21c-09 step 1); `scene-pack-identity-mismatch` (with `--recipe`: the
  regenerated `SceneId` differs from `expected`, or a
  stored pack fails its identity check, N19c-23 step 5).
- Implemented by: BD-236. Extended by: N19c-20 (`--segments` on the default `show`), N19c-21 (the eviction policy
  behind `--action evict --free` and `--allow-evict`), N19c-23 (`--recipe` with the identity check).

**`scene.compile`** (MCP tool `scene_compile`; owner `frob-scene`)
- Inputs (BD-163, which fixes the spellings): the source as the positional argument (a constructive-solid document
  `*.src`, or an asset file or directory), or the derived form `--base <SceneId> --edits <file>` (N19b-44: a base
  pack and an edit list; the two together, exclusive with a source path, `FROB-REF-802` otherwise); `--name
  <catalog name>`; `--fine <m>`, `--coarse <m>`, `--truncation <m>` (defaults 0.02, 0.16, 0.08); `--overlay-capacity
  <n>` (default 8); `--profile <import profile>` (default `default`); `--repair none|fill` (default `none`; `fill`
  requests the explicit repair pipeline of stage 4, each closed hole recorded as `derived: hole-fill`);
  `--brick-layout <n>` (8 is the only value a default build admits; N19c-31's experiment build adds 9); `--workers
  <n>` (operator-only, outside every identity, echoed in the receipt); idempotency key; budget. There is no
  `--seed`: a compile is a deterministic function of its source and settings.
- Outputs: `scene_id`, the manifest summary, the eleven stage receipts, the acceptance status and the evidence
  block; for the derived form the `lineage` (parent, canonical edit list) and `aspects` (five digests) blocks.
  Exit 0; 2 when the pack compiled with findings (palette overflow, skipped stages, an over-threshold deviation that
  acceptance must waive); 8 still running. Evidence: *static*.
- Chapter refusals: `FROB-REF-301` (source geometry is not watertight or not consistently oriented); `FROB-REF-302`
  (the transform class of an import is not established: a reflection or affine map is not excluded, or winding
  disagrees with the determinant); `FROB-REF-303` (a decode limit is exceeded or a reference does not resolve);
  `FROB-REF-306` (a feature below voxel scale has no overlay, or an overlay index cell is over capacity);
  `FROB-REF-320` (a thin feature with no route; Traction's code, raised at compile); `FROB-REF-315` (a movable object
  has no declared behaviour) and `FROB-REF-316` (a surface region has no optical card), raised by the fixture and
  optical stages from M3; `spawn-region-no-admissible-pose` (stage 9 finds a spawn region with no admissible pose,
  N19a-19); `scene-pack-identity-mismatch` (the freeze stage's self-check, N19a-20); for the derived form,
  `scene-edit-invalid` (N19b-44, M3: the edit list cannot be applied to its base: a target that does not resolve,
  a parameter outside its kind's range, two edits that set one field differently, or an edit that changes an
  aspect it does not declare; nothing is written), and `scan-profile-absent` (a base that is a scan pack imported
  without its scan profile, NEG-23). Not `FROB-REF-317`: stage 10
  reports an over-threshold deviation as a finding and the refusal and its waiver are `scene.accept`'s (BD-228
  item 1, as BD-163 states).
- Implemented by: BD-163. Extended by: N19b-44 (the derived form; `lineage` and `aspects`; `scene-edit-invalid`),
  N19c-31 (layout 9 in its experiment build only).

**`scene.fit`** (MCP tool `scene_fit`; owner `frob-scene`)
- Inputs (N19b-32): `--scene` (a name or `SceneId`, echoed resolved in `bound`); `--footprint <family>` (an outline
  and reference point, for example `rect:0.60x1.30`) or `--design` (from which footprint, envelope and
  capabilities are derived); `--margin` (the margin w to decide, default 0; the answer reports w_in and w_out);
  envelope and capabilities (top and step-over heights; step height, slope, gap, push torque and force,
  reversal); the route (`--route <name>` of the scene's route graph, or `--from <pose or region> --to
  <region>…`); fixture modes (or `as-declared`; doors `open`, `closed`, `openable`); `refine` (`max_level`
  default 4, `max_cells`, or `none`); optional `require_label kinematic`; `emit` (keep the `.fclear` artifacts,
  write a certificate bundle); optional `fallback` (`none` by default, or `heuristic`: the route-graph
  traversability check of the KILL-FCLEAR fallback, labelled `heuristic`, with the tag reduced to `unresolved`
  wherever it and the brute-force grid disagree; an additive field of `frob.scene.fit.in/1` added by N19b-36);
  idempotency key; budget.
- Outputs: a feasibility witness path, an impossibility cut, or `unresolved` with what to refine. Evidence:
  *certificate*.
- Chapter refusals: `FROB-REF-304` (an ingested scene whose scale label is `relative-only`: scale authority is
  missing); `FROB-REF-305` (a required route, spawn or contact lies in an unknown region); `FROB-REF-308` (a
  clearance query is outside the domain of the complex: the footprint changes shape along the route, a height or
  step capability is missing); `FROB-REF-891` (a stored `.fclear` at an unsupported format version);
  `FROB-REF-303` (the `.fclear` reader of N19b-27: a stored complex whose blob reference does not resolve or whose
  blob length differs from the lattice arithmetic nx·ny·headings·4 bytes; and the pack reader's code for a scene
  pack that cannot be decoded as written, BD-162); `scene-pack-identity-mismatch` (a stored `.fclear` or the scene
  pack it was built on whose recomputed identity differs from the stated one, raised by the `.fclear` reader of
  N19b-27 with `artifact: fclear`); `scan-profile-absent` (the scene is a scan pack imported without its scan
  profile, NEG-23).
- Implemented by: N19b-32, BD-398. Extended by: N19b-36 (`fallback`; the help-schema snapshot changes additively
  when it lands).

**`scene.gen`** (MCP tool `scene_gen`; owner `frob-scene`)
- Inputs (BD-165, which fixes the spellings; one spelling for every caller): `--family <name>`, where a terrain
  family is `terrain/<family>` (`flat`, `slope`, `stairs`, `step`, `gap`, `stones`, `rough`, `rubble`, `ramp`,
  `curb`, and `mixed-route`) and an interior family is `warehouse`, `kitchen-commercial/galley` (BD-225), `home`,
  `apartment`, `office`, `plant` or `retail` (BD-226); `--difficulty <d>` (d in [0, 1], mapped to parameters by
  N19a-28's maps); `--param <name>=<value>` (repeatable: one family parameter; outside its registered range, or a
  violated validity constraint, is `FROB-REF-802` naming the field); exactly one of `--seed <s>`, the batch form
  `--seeds <a>..<b>` (N19a-26: both ends inclusive, a ≤ b; one job and one key for the range) and the world form
  `--address <seed>:<world>:<episode>` (N19cx-02: the pack of that world, from its block address and orbit slot),
  `FROB-REF-802` for two of them or none; or, in place of all of the above, `--recipe <file>` (N21c-09: family,
  parameters and seed read from a `frob.scene.recipe/1` record, which may not also be given: `FROB-REF-802`);
  `--grid <grid.json>` (a curriculum grid specification, schema `frob.scene.grid/1`, an `artifact-input` whose
  format row BD-165 enters in `registries/formats.toml`: `schema`, `levels` (R, an integer ≥ 2), `columns` (each a
  `terrain/<family>` name or a seed variant `<family>@<seed>`), `tile_m` (the tile side in metres as an exact
  decimal string); an unknown or missing field is `FROB-REF-802` with the pointer at it); `--brick-layout <n>`
  (as on `scene.compile`: 8 is the only value a default build admits, 9 only N19c-31's experiment build, Cargo
  feature `experiment-brick-apron`, and any other value `FROB-REF-802`; a compile setting, so part of the
  `SceneId`); `--survey <file>` (N19c-06: a
  `frob.scene.survey/1` document whose surveyed dimensions are pinned; its archetype must equal `--family`, and a
  malformed survey or a dimension without an uncertainty is `FROB-REF-802` with the offending line; a survey
  without `synthetic = true` whose path is not beneath the private root is `FROB-REF-734`); `--populated` and
  `--assets <class>` (BD-399, Frontier, behind the `frontier-dressing` feature: populate the interior from asset
  libraries drawing no class stricter than `redistributable` or `attribution`, another value `FROB-REF-802`;
  without the feature, or after KILL-DRESSING fires, `--populated` is `FROB-REF-724`, exit 5); `--actors` (N38a-33,
  M6, Frontier, behind the `frontier-actors` feature: the generator adds the archetype's kinematic actor
  declarations, drawn from streams of their own after the scene is otherwise complete, so the result is the proxy
  scene of the same seed with actors added, a different `SceneId` in the same split group; interior families only,
  `FROB-REF-802` on a `terrain/…` family; `FROB-REF-724`, exit 5, without the feature or after KILL-DRESSING fires
  for actors; it combines with `--seeds`, `--address` and `--populated`); `--workers <n>`
  (operator-only, outside every identity); idempotency key; budget. Family parameters set through `--param` that
  other beads define: `impossible_quota` (N19a-26, in [0, 1), the seven interior families), `hazards.exclude=<row
  identifier,…>` (N19a-38: withholds hazard rows; a structural row is refused as an out-of-range parameter), the
  mixed-route parameters (N19a-29: `interior`, `exterior`, `approach_m`, `window_m`, `swing`, `reversed`, `wet`,
  `outdoor_level`, `indoor_level`, `door_clear_width_m`, `sill_m`), `units` (BD-226, `retail`). (Round 3: the
  example's earlier form `frob scene gen terrain:stairs …`, a positional family with a colon, parsed against no
  bead's spelling and is replaced.)
- Outputs: a scene pack identified by (family, parameters, seed): the manifest `frob.scene/1` with `scene_id`,
  family, seed, group, field settings, counts, provenance (generator name and version, `dimensions_assumed`),
  acceptance hash, the constraint report and `derivations` (the number of derivations evaluated: exactly one per
  seed, BD-225); for `--seeds`, the batch identifier `batch:<digest>` and the batch manifest (per seed the
  `SceneId`, and the count record `witnessed`, `impossible_intended`, `impossible_incidental`, `unresolved`,
  `total`). Terrain at M2, interiors at M3. Evidence: *static*.
- Chapter refusals: `FROB-REF-315` (a movable object has no declared behaviour); `FROB-REF-316` (a surface region
  has no optical card) (both from the interior generators at M3); `FROB-REF-303` (a decode limit from the compiler;
  with `--recipe`, a local generator pin or compiler version that differs from the recipe's, N21c-09);
  `FROB-REF-306` (an overlay-capacity limit from the compiler); `FROB-REF-320` (a thin feature with no route);
  `spawn-region-no-admissible-pose` (stage 9: a generator that provokes it has broken its own grammar);
  `scene-pack-identity-mismatch` (with `--recipe`: the regenerated `SceneId` differs from the recipe's `expected`,
  N21c-09's emit site); from its assignment, `hazard-registry-invalid` for an interior family whose hazard registry
  does not load (BD-227); `asset-source-not-default-pipeline` (from M5, the populated-interior draw of
  `--populated`, BD-399: an asset class drawn from a source row with `default_pipeline = false`; N19b-02's filter;
  nothing is written).
- Implemented by: BD-165 (terrain, M2). Extended by: BD-225 (`warehouse`, `kitchen-commercial/galley`, the output
  field `derivations`), BD-226 (the other five interior families), N19a-26 (`--seeds`, `impossible_quota`), N19a-29
  (`terrain/mixed-route` completed at M3), N19a-38 (`hazards.exclude`), N19c-06 (`--survey`), N19cx-02
  (`--address`), N21c-09 (`--recipe`), N19c-31 (`--brick-layout 9` in its experiment build only), BD-399
  (`--populated`, `--assets`, M5, Frontier), N38a-33 (`--actors`, M6, Frontier).

**`scene.hazards`** (MCP tool `scene_hazards`; owner `frob-scene`)
- Inputs (BD-227, which fixes the spellings): `<archetype>` (positional: one of the seven registry names `home`,
  `apartment`, `office`, `warehouse`, `manufacturing-plant`, `retail-store`, `commercial-kitchen`; a generator
  family name in its place is `FROB-REF-802` with the pairing hint); `--slug <slug>` (one row; an unknown slug is
  `FROB-REF-802` listing the archetype's slugs); `--uncovered-by <scenes>` and `--incidence <scenes>` (N19a-38:
  the rows no scene of the set contains, split into `never-drawn` and `not-represented`; the incidence export in
  canonical order with its digest; `<scenes>` is a comma-separated list of names or `SceneId`s, or `@<file>` with
  one per line under the session's roots; a scene of another archetype, or an unknown scene, is `FROB-REF-802`);
  `--feature-law [--param <name>=<value>]…` (N19ax-04: the exact marginals of each discrete feature of the paired
  generator family at the given family parameters, the spelling `scene.gen` uses). Every form compiles nothing and
  takes no key.
- Outputs: the hazard registry of the archetype with the provenance of every row: per row its slug, identifier,
  address, name, provenance (`assumed`, `norm: <citation>` or `measured: <dataset>`; the earlier text printed
  `cited`, which no bead uses), dimension entries (drawn: range, unit, distribution; fixed: value), mechanisms with
  tier, `solid_alternative`, and the failures it provokes; the counts by provenance `{assumed, norm, measured}`; the
  generator family the archetype pairs with. Evidence: *pass-through* (the `--feature-law` form computes its answer:
  class `Tested`, exact).
- Chapter refusals: none of its own; from its assignment, `hazard-registry-invalid` (a registry file that fails to
  load: pointer the file, the slug and the field), on the interim `FROB-REF-802` until then.
- Implemented by: BD-227. Extended by: N19a-38 (`--uncovered-by`, `--incidence`), N19ax-04 (`--feature-law`).

**`scene.import`** (MCP tool `scene_import`; owner `frob-scene`)
- Inputs: `--kind asset|place` (required: `asset`, one object or furnished fragment, from M3; `place`, a scan of a
  real place, from M5; no value or another is `FROB-REF-802`) and the path of the asset or scan (positional, a
  `foreign-file` parameter, N34-24, so the parameter rule of NJL-01 appends 734, 828 and 893).
  - The `asset` form (N19a-46, which fixes the spellings): `--source <key>` (required: a key of the asset-source
    register, or `user-supplied`); `--user-supplied` (the caller holds the file under the source's terms; required
    for a row with `default_pipeline = false`); `--licence-ack <file>` (hashed into the licence entry, never copied;
    required with `--user-supplied` for a `research-only` or `proprietary` row); `--licence-class <class>` (the
    declared class for `user-supplied`; never less restrictive than a named row's); `--object-licence <SPDX id or
    "unknown">`, `--object-origin <URL>`, `--attribution <text>` (for a `per-object` row, NEG-26); `--profile
    <name>` (a named record of `registries/import-profiles.toml`, N19a-10: `asset-default`, N19a-46's default;
    the row beads add `scene-collection` (NEG-22), `scan-store` (NEG-23: a scan stored without geometry at M3),
    `articulated-object` (NEG-24) and `generic` (NEG-25) at M3, and `populated-interior` (NEG-18), `small-rigid`
    (NEG-19), `kitchen-fixture` and `kitchen-layout` (NEG-20) at M5; another name is `FROB-REF-802`); `--behaviour
    static|articulated|free|prescribed` (required for anything that can move); `--unit <unit>` (only when the
    format declares none); `--scale <factor>` or `--target-dimension <axis>=<length>` (a normalized source, NEG-24);
    `--card <region>=<card>` (repeatable measured cards); `--repair none|fill`. A flag the named row does not use
    is `FROB-REF-802` naming the flag and the row.
  - The `place` form (BD-398, which fixes the spellings): `--from twin|fdgr|mesh` (the adapter of N19c-04, default
    `twin`; `fdgr` reads a geometry generation through N33c-25's schema module; `mesh` needs `--declare`);
    `--source`, `--user-supplied`, `--licence-ack` (optional for the owner's own capture; required by the asset
    form's rules when the scan comes from a dataset row, for example `--source hm3d --user-supplied --licence-ack
    <file> --profile hm3d-scan`, NEGp1-52); `--profile <name>` (default `large-building`); `--landmarks <file>`
    (landmark correspondences when the package does not carry them); `--declare <file>` (the operator declaration:
    units, frame, handedness, optionally a scale witness and the answered probe requests, which close those
    requests on the new pack, N19c-05); `--name <name>` (a mutable name in the private catalog); `--repair
    none|fill`. There is no per-call private root: the pack is written beneath `roots.private` of the owner-held
    safety configuration (N37-02, BD-305), and with none configured the import is `FROB-REF-734` with `observed =
    root-unset`.
  - Both forms: `--private` (BD-305: raises the root mark of the new pack to `Private`; nothing lowers a mark);
    idempotency key; budget. `--repair` takes `none|fill` on both forms (N19a-46, BD-398: BD-163's values for the
    same stage-4 pipeline).
- Outputs: a scene pack with a loss report (`frob.scene.import.loss/1`), a scale label and an unknown mask; the
  stage receipts, the licence entry and the acceptance status (`incomplete` until `scene.accept` has run). Exit 0;
  2 when written with findings (assumed cards, repaired holes, parts tagged `optical-only`); 8 still running.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-302` (the transform class of an import is not established: a reflection or affine map
  is not excluded, or winding disagrees with the determinant); `FROB-REF-303` (a decode limit is exceeded or a
  reference does not resolve); `FROB-REF-304` (scale authority is below what the task requires); `FROB-REF-301` (a
  collidable part that is not watertight or consistently oriented, unless `--repair fill`); `FROB-REF-315` (a part
  that can move with no declared behaviour); `FROB-REF-317` (an imported asset's collision-to-visual deviation above
  the pack threshold with no waiver, N19a-46 step 8); `FROB-REF-828` (a file the importer's reader opens carries
  code or is a weight format outside the allowlist: N37-03 sniffs every file before its reader, including each
  member of a scan directory and each file an asset references, which the parameter rule does not reach; NJL-01
  lets a chapter list name 828 for an importer's reader, and the full list is a set); `FROB-REF-832` (an import over
  its size, depth or polygon limit); `asset-source-not-default-pipeline` (both forms: N19b-02's default-pipeline
  filter refuses a source row with `default_pipeline = false` without `--user-supplied`; nothing is written);
  `import-articulation-interpenetrates` (the asset form under the articulated profiles, `articulated-object` of
  NEG-24 at M3 and `kitchen-fixture` of NEG-20 at M5: a joint swept kinematically from its lower to its upper limit
  and back, in steps of at most 1° or 5 mm, puts a part into its own fixture, frame or another part deeper than
  the recorded fitting or decomposition error; the part and the joint value are named and nothing is imported;
  family `3xx`, exit 3, retry `after-change`; the allocation ledger's row of N35x-02, registered by N19-REF, with
  no proposal and no interim code). (301,
  315 and 317 are the asset form's, added in round 3 from N19a-46's pipeline; N19a-46 assigns every region a card,
  so it never raises 316.)
- Implemented by: BD-398 (the `place` form, M5).
- Extended by: N19a-46 (the `asset` form, M3: a reduced form that answers before the `place` form and is listed by
  name in the capability report, NJL-01 and NJL-03; `--kind place` answers `FROB-REF-724`, exit 5, until BD-398);
  BD-305 (`--private`).

**`scene.list`** (MCP tool `scene_list`; owner `frob-scene`)
- Inputs (BD-163, which fixes the spellings; paginated): `--family <name>`, `--group <group>` (filters); `--limit
  <n>` and the cursor (the paginated read's common properties); `--batch <id> --counts` (N19a-26, M3: the count
  record of a `scene.gen --seeds` batch, a recount and the stored audit's verdict, compiling nothing; a failed audit
  is a finding, exit 2, naming the missing or swapped seed); `--private` (BD-305: the explicit private listing, the
  only way a session sees its private packs; `option_capabilities = { private = ["private:read"] }`, so the option
  without that scope is `FROB-REF-720` while a call without it needs only `scene:read`; the envelope of a private
  listing is itself marked private).
- Outputs: the scene catalog, one row per pack with the columns BD-163 fixes: `scene_id`, catalog name, family,
  seed, group, spacings, band-brick count, bytes, acceptance status, feasibility tag, scale label and
  `dimensions_assumed`, and the privacy-mark column (BD-305); private packs are absent without `--private`, and no
  count of withheld rows is printed; for `--batch … --counts` the count record (`witnessed`,
  `impossible_intended`, `impossible_incidental`, `unresolved`, `total`), the recount and the audit verdict.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-303` (a listed pack whose manifest cannot be decoded as written: the pack reader's
  code, BD-162); `scene-pack-identity-mismatch` (a listed pack, or a batch member under `--counts`, whose
  recomputed identity differs from the stated one).
- Implemented by: BD-163. Extended by: N19a-26 (`--batch`, `--counts`), BD-305 (`--private` and the visibility
  filter every listing calls).

**`scene.probe-request`** (MCP tool `scene_probe_request`; owner `frob-scene`)
- Inputs (N19c-05, which fixes the spellings): `<ref>` (positional: an answer reference `a:…`, mission cell or fit,
  or a scene); `--cell c:…` (narrows a mission answer to one cell); `--status open|closed|all` (filters a scene's
  standing requests). The drafts' `--scene <id> --for <answer-ref>` and `<scene> --answer a:…` are not spellings.
- Outputs: the list of `frob.scene.probe-request/1` records that would settle the answer (target box or features,
  quantity, accuracy, threshold, status, the link to the pack that closed it). Exit 0 with requests, 0 with an empty
  list for a scene with nothing unresolved, 2 with the finding "nothing to settle" for an answer that is not
  unresolved. A read that derives: requests absent for an answer are computed and their record written, which is
  idempotent. For a private pack the result is stored under the private root and the envelope carries the privacy
  mark. Evidence: *screening* (class `Estimated`), or *certificate* (class `Certified` relative to the stated
  bounds) where the limiting answer is a certificate (N19c-05 item 5).
- Chapter refusals: none of its own.
- Implemented by: N19c-05. (Round 3: N25c-19 is superseded by N19c-05 and left the list; BD-291 is the
  distinguishing-experiment bead, which names `scene.probe-request` as N19c-05's and is not an implementing bead.)

**`scene.purge`** (MCP tool `scene_purge`; owner `frob-verbs`: N37-19 builds the handler there, test
`-p frob-verbs --test scene_purge`, over the store functions of `frob-chronicle`, test `-p frob-chronicle --test
purge`; the other twelve handlers are the dispatch table of `frob-scene`, N19a-02)
- Inputs (N37-19, which fixes the spellings): `<scene>` (a `SceneId` of a private pack; a pack not marked private
  is `FROB-REF-802`, "not a private scene pack; use the store's collector"); `--dry-run` (Boolean, default true: the
  closure and its digest, nothing removed; the real call is `--dry-run=false`); `--confirm <digest>` (the closure
  digest a dry run returned; required when not a dry run, and a missing or stale digest is `FROB-REF-802` with the
  new digest); `--withdraw-claims` (Boolean, default false: withdraw the live claims whose evidence closure holds a
  member, and release their pins, inside the purge). No idempotency key (DESIGN, the `admin` kind of NJL-01, which
  gives `admin` no key and no 805, as BD-236 does for the bank's admin actions): the confirm digest is the guard
  against an unintended call, and a repeat of a completed purge finds the tombstones and returns the stored receipt
  as a no-op, so the call is idempotent by effect. N37-19's restated row is this one (no key, no 805, and the
  surface codes 720 and 802 appended by the generator, not in the chapter list), and N37-21's script calls the verb
  without a key.
- Outputs: the closure (the pack and everything derived from it, by lineage), the closure digest, the claims
  withdrawn and pins released, the tombstones, and the purge receipt; for a dry run the same without removal. The
  MCP tool is annotated destructive. Evidence: *operational*.
- Chapter refusals: `FROB-REF-813` (a deletion that would remove a pinned object: a member of the closure is in the
  evidence closure of a live claim and `--withdraw-claims` was not given; repair "repeat with `--withdraw-claims`,
  or withdraw them first"). A session without `private:read` is refused with `FROB-REF-720` (a listed scope is
  missing) and learns nothing about whether the scene exists, so `FROB-REF-731` is not raised here.
- Implemented by: N37-19. Extended by: BD-305 (the privacy mark and private root the purge reads).

**`scene.retrofit`** (MCP tool `scene_retrofit`; owner `frob-scene`)
- Inputs (N19b-40, which fixes the spellings): `--evidence <ref>` (a claim or artifact identity: a checked cut, a
  `.fmcw`, or an alias witness; an identifier, not a path); `--permit <class>` (the highest install class the
  mission allows, default `renter-safe`); `--max-kits <n>`; `--compare-redesign` (default true); `--tier <tier>`
  (the re-decision tier, default the tier that produced the failing verdict); `--plan-only`; `--apply
  <entry>:<param>=<value>,… --at <feature> --scene <id>` (apply one named kit and return the variant, no search);
  `--strict-data` (refuse a cost verdict on `assumed` prices); `--facility` (N19bx-01, M8: adds to the `--apply`
  report the as-installed band check, with the violating vertex, and the annualized facility terms; a missing
  facility sub-table is `Unpriced { missing }`, never zero; without it the report is unchanged); idempotency key;
  budget. The retrofit catalog is read from the registry, not passed. (Round 3: the example's positional evidence,
  `frob scene retrofit w:3fa9 …`, is not one of N19b-40's spellings and is replaced by `--evidence w:3fa9`.)
- Outputs: the cheapest building amendment that flips the verdict with the policy frozen, beside the cheapest
  redesign: both arms' flip verdicts, kits with prices and their provenance (`prices: assumed` where they are), the
  comparison, the promotion status; for `--apply` the variant scene. Evidence: *study* (claim `DesignComparison`,
  never a certified class).
- Chapter refusals: `FROB-REF-313` (a retrofit violates a constraint or an install class the mission does not
  permit); `retrofit-cost-on-assumed-prices` (constant `RETROFIT_COST_ASSUMED_PRICES`: a cost verdict under
  `--strict-data` while a compared price is `assumed`; the flip verdicts are returned in the refusal's detail).
- Implemented by: BD-369, N19b-40. Extended by: N19bx-01 (`--facility`, M8).

**`scene.show`** (MCP tool `scene_show`; owner `frob-scene`)
- Inputs (BD-163, which fixes the base spellings): `<scene_id>` (positional); `--receipts` (the eleven stage
  receipts); `--probe x,y,z` (repeatable) or `--probe <points.json>` (distance and region at points); `--bytes` (the
  whole byte recount record of N19c-16, beside the byte fields printed by default); `--source <path>` in place of a
  `scene_id`, with `--probe` (N19c-38: an uncompiled source read through compile stages 1 and 2 only, writing
  nothing and returning no `SceneId`). Forms added by extending beads:
  - the field form `--field <task> --draw <name> --address <seed>:<world>:<episode> [--at x,y,z]` (N19ax-03: the
    value of a drawn field at a block address, or along the task's channel);
  - `--draws` (N19ax-04: the draws of the pack's discrete features with their law);
  - the route form `--route <a>,<b>|shortest`, with `--corridor <half-width> --at x,y,z,yaw --footprint <design>`
    to evaluate the corridor margin at one pose (N19ax-02);
  - the family forms `--family <name>` in place of a `scene_id` (N19cx-01: the descriptor, `family_id`, declared and
    effective law groups) and `--family <name> --address <seed>:<world>:<episode> [--image <element>]` (N19cx-02:
    the orbit slot, or the mapped address with its certificate; no image is a `detail` with exit 0, not a refusal).
  `--at` is one option with the grammar `x,y,z[,yaw]` (DESIGN: N19ax-03 passes a point and N19ax-02 a pose under the
  same name; the field form refuses a yaw and the corridor form requires one, `FROB-REF-802` either way). There is
  no `--hazards` option: the hazards drawn are part of the default output.
- Outputs: the manifest, the hazards drawn and the feasibility tag; the byte fields; with the derived form's packs
  the `lineage` and `aspects` blocks (N19b-44); with `--route`, the resolved node list, segment lengths and slabs, and
  the block `pinch_zones` (N19ax-01); the probe answers as `WorldDistanceBound`s with `resolved` and the method (an
  interval unless resolved). Evidence: *pass-through* (the probe, field and family forms compute their answer: class
  `Tested`, or `Certified` where interval arithmetic or a certificate bounds it).
- Chapter refusals: `FROB-REF-303` (a stored pack that cannot be decoded as written: a missing or mistyped
  manifest member, a missing blob, a stored value outside its stated range, a name that does not resolve; the pack
  reader's code, BD-162, whose script asserts it from `frob scene show <SceneId>` on a planted pack);
  `FROB-REF-731` (a private object, or anything derived from one, without the private scope and flag);
  `scene-pack-identity-mismatch` (the pack read differs from its stated identity); `scan-profile-absent` (the
  `--probe`, `--route` or `--corridor` form on a scan pack imported without its scan profile, NEG-23).
- Implemented by: BD-163. Extended by: N19a-18 (`--receipts`), N19c-16 (`--bytes`), N19c-38 (`--source`), N19ax-01
  (`pinch_zones`), N19ax-02 (the route and corridor form), N19ax-03 (the field form), N19ax-04 (`--draws`),
  N19cx-01 and N19cx-02 (the family forms), N19b-44 (`lineage`, `aspects`).

**`scene.split`** (MCP tool `scene_split`; owner `frob-scene`)
- Inputs (BD-235, which fixes the spellings): the operation, selected only with `--action <name>` (NJL-01 action
  rule 1); the default, `create`, is the call with no `--action` and has no name to pass:
  - `--name <N> --group-by building --rule '<selector>=<part>'… [--default <part>]` (mutate, the default action
    `create`: a new split; the rule grammar is BD-235's, for example `'hash-bucket:0..7000=train'`; a call without
    `--name` is `FROB-REF-802`);
  - `--action add --split <S> <scene_id>… | --family <F> --seeds <a>..<b> [--to <part>]` (mutate: classify scenes
    by the rules, or place them explicitly with `--to`; all or nothing);
  - `--action move-group --split <S> <group> --to <part>` (mutate: moves a whole group and returns the successor
    split; no operation moves one member of a multi-scene group);
  - `--action show --split <S>`, `--action check --split <S> [--deep]` (re-derive groups, re-check straddles,
    recount; `--deep` is N19c-11's deep check, whose finding `possible-ungrouped-variant` exits 2 and never merges
    groups), `--action part-of --split <S> <scene_id>` and `--action audit --split <S> --runs <run-set> [--rule
    <i>]` (N19c-13: recompute `part_of` for every episode of the run set from the stored values of the group's
    base scene) (read, scope `scene:read`, no key, no parent).
  An action word in a positional place (`frob scene split create …`, `frob scene split show …`) and `--action
  create` are `FROB-REF-802`.
  `--rule` is one option whose input schema is typed by action: under `create` a repeated `'<selector>=<part>'`
  string (rules kept in the order given, first match wins), under `audit` one 1-based integer index into the
  split's rules in that order (an index outside them is `FROB-REF-802`). The selectors are BD-235's,
  `family:<name>` (a trailing `/*` matches a prefix), `variant:<name>`, `seed:<lo>..<hi>` (inclusive, decimal or
  `0x` hexadecimal), `param:<name>:<lo>..<hi>` (inclusive quantities with units), `building:<id>`,
  `group:<GroupId>` and `hash-bucket:<lo>..<hi>[@<salt>]` (half-open, 0 ≤ lo < hi ≤ 10,000), and N19c-13's
  `episode:<name>:<lo>..<hi>` over a per-episode randomized quantity (inclusive; from M3); no selector reads the
  feasibility tag, and a rule that does not parse, a part not in the list or an empty range is `FROB-REF-802`.
  The writing operations take `--key` and `--expected-parent`; for `create` the parent is the empty parent, which is
  its default, so `create` is called without `--expected-parent` (as N19c-11 and N19c-15 call it).
- Outputs: one schema `frob.scene.split.out/1` (the registry's name; BD-235's earlier `frob.scene.split.result/1` is
  not used): for `create` the `split_id` and empty counts; for `add` per scene its group and part, the updated
  counts and the per-scene refusals; for `move-group` the successor split; for the reads the manifest
  (`frob.scene.split/1`), the check's findings and recount, the part of a scene, or the audit's per-episode verdicts.
  Evidence: *static*.
- Chapter refusals: `FROB-REF-311` (a scene group straddles a split: Move the whole group); `FROB-REF-324` (row 5,
  `scene-split-group-unmatched`: on `add`, the scene's group matches no rule and the split has no default part);
  `FROB-REF-325` (row 6, `scene-split-group-cited`: `move-group` out of a held-out or development part that a frozen
  study or an open qualification request scored on); `scene-split-count-mismatch` (row 7, exit 9: on any read of a
  split, `check` or `audit`, the stored counts differ from a recount).
- Implemented by: BD-235. Extended by: N19c-11 (`--action check --deep`), N19c-13 (`--action audit`, the stored
  resolved values and the recount).

A row of kind `mutate` whose action list holds read actions is NJL-01's form (`actions = [{name, kind,
capabilities}]`): `scene.split` has `kind = "mutate"` for its default action `create` and the actions `add`,
`move-group` (mutate, `scene:write`) and `show`, `check`, `part-of`, `audit` (read, `scene:read`), each linted by
its own kind (BD-258 step 3), so a read action takes no key and gets no 805.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `scene.accept` | `scene:run` | `frob.scene.accept.out/1` | `places-check` | `frob scene accept s:kitchen-17 --key acc-1 --json` |
| `scene.bank` | `scene:read` | `frob.scene.bank.out/1` | `housekeeping` | `frob scene bank --device cuda:0 --json` |
| `scene.compile` | `scene:run` | `frob.scene.compile.out/1` | `places` | `frob scene compile stairs.glb --name stairs-a --key sc-1 --json` |
| `scene.fit` | `scene:run` | `frob.scene.fit.out/1` | `places-check` | `frob scene fit --design quad-a --scene s:kitchen-17 --route galley --key fit-1 --json` |
| `scene.gen` | `scene:run` | `frob.scene.gen.out/1` | `places` | `frob scene gen --family terrain/stairs --difficulty 0.4 --seed 11 --key sg-1 --json` |
| `scene.hazards` | `scene:read` | `frob.scene.hazards.out/1` | `places-check` | `frob scene hazards commercial-kitchen --json` |
| `scene.import` | `scene:import` | `frob.scene.import.out/1` | `places` | `frob scene import --kind asset assets/in/mug.glb --source frob-scene --profile asset-default --behaviour free --key imp-1 --json` |
| `scene.list` | `scene:read` | `frob.scene.list.out/1` | `orient`, `places`, `batteries` | `frob scene list --json` |
| `scene.probe-request` | `scene:read` | `frob.scene.probe-request.out/1` | `places-check` | `frob scene probe-request a:0e51 --cell c:0204 --json` |
| `scene.purge` | `scene:admin`, `private:read` | `frob.scene.purge.out/1` | `housekeeping` | `frob scene purge s:galley-real --json` |
| `scene.retrofit` | `scene:run` | `frob.scene.retrofit.out/1` | `places-check` | `frob scene retrofit --evidence w:3fa9 --key rt-1 --json` |
| `scene.show` | `scene:read` | `frob.scene.show.out/1` | `attack`, `places` | `frob scene show s:kitchen-17 --json` |
| `scene.split` | `scene:write` | `frob.scene.split.out/1` | `places-check` | `frob scene split --name galley-holdout --group-by building --rule 'hash-bucket:0..7000=train' --default held-out --key sp-1 --json` |

The examples changed in round 3 (`scene.gen`, `scene.hazards`, `scene.import`, `scene.retrofit`, `scene.split`)
replace forms that parsed against no implementing bead's spelling or were refused at their milestone: a positional
`terrain:stairs`; the generator family name `kitchen-commercial` where `scene.hazards` takes the registry name
`commercial-kitchen`; `--source google-scanned-objects`, which is not a default-pipeline row until its M5
admission reading (NEG-19), for a repository-authored mug whose source key is `frob-scene` (N19a-46); a positional
evidence reference; and a `scene.split` call with no `--name` that passed `--family`, an input of `--action add`
only. The `scene.split` example (the default action `create`) gives a default part so that it classifies every
group (part names are BD-235's `train`, `development`, `held-out`).

One verb answers later than its printed milestone: `scene.retrofit` is printed at M3 in the inventory and in the
chapter's verb table, and the gate register moves the retrofit catalog and its gate GATE-HAB-11 to M4 (the register
governs when the gate is due); its implementing beads (BD-369, N19b-40) are M4 beads. The row keeps the appendix's
`first_milestone = "M3"` (the rule BD-137, NJL-01 and every registry bead share: the field is pinned to the
appendix, whose per-milestone counts are known answers, and the handler decides what a build serves), so in an M3
build the verb answers `FROB-REF-724`, exit 5, and `frob capabilities --json` lists it as absent until a build
carries BD-369. (Correction, round 1: an earlier text entered the row at M4.) The row is one of the declared late
rows of NJL-02's closed list `LATE_ROWS` (BD-369 and N19b-40, lands M4), so the capability level of the coverage
check accepts its 724 answer in M3 builds and not after.

`scene.import` is not in `LATE_ROWS` (NJL-02's list holds rows, not forms of a row): from M3 the build lists the row
implemented with the reduced `asset` form (N19a-46), and NJL-02 checks a row listed implemented by the ordinary
rule: its registry example must then pass. The example is therefore the asset import of N19a-46 (an earlier text
used a real-place scan, `frob scene import galley-scan/ --name galley-real`, which answers 724 until M5 and would
have failed the ordinary rule from M3; it also lacked the required `kind`). The cluster script additionally calls
`--kind place` and expects `FROB-REF-724`, exit 5, until the capability report lists the `place` form.

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column; `params` lists the path parameters of the table above with
their kinds. A verb printed with two kinds has `kind` equal to its default action and an `actions` list for the
others, each with its own kind and scope: `scene.bank` (default `show`, read): `estimate` (read, `scene:read`),
`admit`, `evict`, `pin`, `unpin` (admin, `scene:admin`); `scene.split` (default `create`, mutate): `add`,
`move-group` (mutate, `scene:write`), `show`, `check`, `part-of`, `audit` (read, `scene:read`). (Round 3: the
bank's list held `evict` alone, while BD-236 implements `admit`, `pin` and `unpin` too and N19c-20 to N19c-24 call
`admit`.) `scene.list` carries `option_capabilities = { private = ["private:read"] }` for its explicit private
listing (BD-305); `scene.import`'s `--private` needs no scope, because it can only raise a mark.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "scene.accept"
cluster = "scene"
kind = "run"
tier = "S"
capabilities = ["scene:run"]
input_schema = "frob.scene.accept.in/1"
output_schema = "frob.scene.accept.out/1"
refusals = ["FROB-REF-302", "FROB-REF-303", "FROB-REF-312", "FROB-REF-317"]  # + the number assigned to scene-pack-identity-mismatch
params = { source = "foreign-file" }
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["places-check"]
examples = ["frob scene accept s:kitchen-17 --key acc-1 --json"]
owner = "frob-scene"
purpose = "The acceptance report of a pack"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved
   compute-envelope scope the job cites; N36-20 takes a job's scope from a launcher's token, then from this
   input, then from the verb's own inputs, and otherwise `adhoc`, whose class in force is E0, and a citation that
   contradicts the launcher's token is refused); `mutate`: `idempotency_key`, `expected_parent`).
   `idempotency_key` is in no JSON-schema `required` array: the annotation `x-frob-key` (`"required"`, or
   `"optional"` on a `key_optional` entry) carries the rule, and the admission transaction (N32a-09, from M1)
   refuses a missing key with 805 on both surfaces. The generator appends the surface-wide refusals:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. What a field or a registry table adds follows NJL-01's variation
   table, which is the one list of what the generator adds: 806 and 807 for `paginated = true`; 808 for an entry
   with an `int53` input at any depth, whatever its kind (every `run` entry has one, `sync_wait_ms`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790 for
   a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a path
   to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`). Its later variations, 10 to 16
   (`override` with `override_reason`, `from_proposal`, `machine`, `engine`, `no_memo`, the reality guard's seven
   codes, and `diagnostic_override` with the admission checks' codes), are read from that table and not copied
   here.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M2); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Thirteen verbs, the largest registry cluster; agents see them through the task clusters `places`, `places-check`,
  `housekeeping` and `batteries`.
- `scene.bank` has two kinds: the resident set and an admission estimate are `read`; admission, eviction, pin and
  unpin are `admin` actions, each idempotent by effect and taking no key. `scene.split` has two kinds the other way
  round: its default `create` and the changes are `mutate`, its inspections `read`.
- `scene.purge` removes a private scene pack and the lineage closure of everything derived from it. An object
  derived from a private scene is not listed and is refused without the private scope (`FROB-REF-731`, on
  `scene.show` and the other reads; `scene.purge` itself requires `private:read`, so its caller meets 720 instead).
- Where an implementing bead and this registry disagreed on a spelling in round 3, the row follows the implementing
  bead (the read action of `scene.bank` is its default `show`, not `status`; `--family terrain/<family>`; `--repair
  none|fill`), and every caller uses that form (KNOWN_DRIFTS 12).
- `scene.gen` first answers at M2 for terrain and at M3 for interiors; `scene.import` at M3 for assets and at M5 for
  real places; `scene.probe-request` at M5.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly thirteen entries with `cluster = "scene"` (5 `read`, 6 `run`, 1
      `mutate`, 1 `admin`), each with every field of the entry schema, an example and a task-cluster list; the alias
      lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster scene` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 13, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_scene.sh` runs each verb of the cluster (thirteen),
      and from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/scene/<verb>.help.json` byte for byte for each of
      the thirteen, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The two-kind verbs expose `action`, selected only with `--action <name>` (the default with none):
      `scene.bank` (`show` default; `estimate`; `admit`, `evict`, `pin`, `unpin`) and `scene.split` (`create`
      default; `add`, `move-group`; `show`, `check`, `part-of`, `audit`); an action word in a positional place is
      `FROB-REF-802`; an `admin` action without its scope is `FROB-REF-720`, exit 5, and the `read` action still
      answers; a `read` action of `scene.split` without a key answers (no 805), and a `mutate` action without one
      is 805.
- [ ] Every input named in a row's Inputs is a property of its generated input schema with the spelling given, and
      every exclusive pair or group named there (`--seed`/`--seeds`/`--address`/`--recipe`; `--base --edits` against
      a source path; `--source --preview` against a pack; `--source` against a `scene_id` on `scene.show`) is
      refused with `FROB-REF-802` when both are given; each path parameter of the parameters table is declared in
      `params` with its kind, and the row's full refusal list holds 734 and 828 and the code of its kind.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_scene` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 13 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `scene.split` is `mutate` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`scene.accept`: 302, 303, 312, 317; `scene.bank`: 303,
  314; `scene.compile`: 301, 302, 303, 306, 315, 316, 320; `scene.fit`: 303, 304, 305, 308, 891; `scene.gen`: 303,
  306, 315, 316, 320; `scene.hazards`: none; `scene.import`: 301, 302, 303, 304, 315, 317, 828, 832; `scene.list`:
  303; `scene.probe-request`: none; `scene.purge`: 813; `scene.retrofit`: 313; `scene.show`: 303, 731;
  `scene.split`: 311, 324, 325), plus, once N19c-42's rows are registered, the assigned numbers of the worded
  refusals table (`scene-pack-identity-mismatch` on seven rows, `scene-split-count-mismatch` on `scene.split`,
  `spawn-region-no-admissible-pose` on `scene.compile` and `scene.gen`, `retrofit-cost-on-assumed-prices` on
  `scene.retrofit`), and the assigned numbers of `hazard-registry-invalid` (BD-227) on `scene.hazards` and
  `scene.gen`, `asset-source-not-default-pipeline` (N19b-02) on `scene.import` and `scene.gen`,
  `import-articulation-interpenetrates` (NEG-24, NEG-20) on `scene.import`, `scene-edit-invalid`
  (N19b-44) on `scene.compile` and `scan-profile-absent` (NEG-23) on `scene.compile`, `scene.fit` and
  `scene.show`; every row that lists `scene-pack-identity-mismatch` also lists 303; a planted row that
  lists 319 on `scene.bank` fails; the appended codes equal the rule for its kind and fields (NJL-01), including
  734, 828 and 891 or 893 for every `params` entry of the parameters table.
- `scene_gen_forms`: `--family terrain/stairs` parses and the positional `terrain:stairs` is 802; exactly one of
  `--seed`, `--seeds`, `--address` and `--recipe` (two of them, or `--recipe` with `--family`, is 802); `--seeds
  9..0` is 802; in a build without `frontier-dressing`, `--populated` is 724, exit 5; `--grid` with `levels = 1`
  is 802 with the pointer at `levels`; in a default build `--brick-layout 8` parses and `--brick-layout 9` is 802;
  in a build without `frontier-actors`, `--actors` is 724, exit 5, and in a build with it `--family terrain/stairs
  --actors` is 802.
- `scene_split_rule_by_action`: under the default `create`, `--rule 'episode:wetness:0.8..1.0=held-out'` and
  `--rule 'hash-bucket:0..7000=train'` parse as rule strings; `--action audit --rule 2` parses as an index and
  `--action audit --rule 'seed:1..5=train'` is 802; the generated input schema types `rule` by action.
- `scene_bank_and_split_actions`: the action lists, kinds and scopes of the remaining-fields paragraph; `scene bank
  --device D` runs the default `show`, and `scene bank --action show --device D` and `scene bank show --device D`
  are 802 (the default has no name to pass); `--action status` is 802 (it is the output variant, not an action);
  `scene bank --action admit s:kitchen-17 --device D` parses with the scene id positional, and `scene bank admit
  s:kitchen-17 --device D` and `scene split create --name n` (action words in a positional place) are 802.
- `scene_show_forms`: `--at 1,2,3` with `--field` parses and `--at 1,2,3,0.5` with `--field` is 802; with
  `--corridor`, `--at` without a yaw is 802; `--source` with a `scene_id`, or `--family` with a `scene_id`, is 802.
- `scene_fit_inputs`: `scene.fit` accepts `--footprint` or `--design` (both is 802), `--route` or `--from`/`--to`,
  `--margin`, and `fallback` with the values `none` and `heuristic` (another value is 802).
- `scene_import_forms`: `scene.import` requires `kind` with the values `asset` and `place` (another value, or none,
  is 802); its full refusal list (chapter and appended) holds 734, 828 and 893, with 828 once; in a build whose
  capability report lists only the `asset` form, `--kind place` is 724, exit 5, one envelope, and the row's example
  is an asset import.
- `mcp_names_one_to_one`: the tool names are the mapped names (`scene_accept`, `scene_bank`, `scene_compile`,
  `scene_fit`, `scene_gen`, `scene_hazards`, `scene_import`, `scene_list`, `scene_probe_request`, `scene_purge`,
  `scene_retrofit`, `scene_show`, `scene_split`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of convention 2 and refuses `sync_wait_ms = 30001`
  with code 802; a `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with
  code 805; an
  `admin` row or action without its scope is refused with code 720; a verb whose capability is absent returns code
  724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_scene.sh` (header: `bead=VERBS-scene milestone=M2 labels=tether,verbs,verb:scene.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance scene`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the thirteen verbs with kind and milestone; (2) `frob-surface-diff parity
--cluster scene` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot
--check --cluster scene`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle
(exit 8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for
each implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` verbs and the `mutate` actions of `scene.split`, one without a
key (`e2e_expect_refusal FROB-REF-805 3`); (7) one envelope per call and no exit code 1; (8) `frob scene import --kind place
scan-min/ --key si-2 --json` (run, like every example, in `fixtures/verbs/scene/`, NJL-02, which holds
`assets/in/mug.glb` for the row's example and a three-file scan directory `scan-min/` that this bead commits) expecting `FROB-REF-724`, exit 5, while the capability report lists the
`place` form absent, and a result once it lists it (logged with the report's form list); (9) once `scene.bank` and
`scene.split` are implemented, `frob scene bank --device cpu --json` (the default `show`) and `frob scene bank
--action show --device cpu --json` (`e2e_expect_refusal FROB-REF-802 3`), `frob scene bank --action admit
s:kitchen-17 --device cpu --json` from a session without `scene:admin` (`e2e_expect_refusal FROB-REF-720 5`, then
the default `show` still answers), and `frob scene split --action show --split <the example's split> --json` with no
key (exit 0, no 805). Every option a step uses is in the row's Inputs (KNOWN_DRIFTS 8); a
step that needs a verb not yet in the build answers 724 and is logged as such.

Log: `target/e2e-logs/verbs_scene/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 13 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 13 verbs (5 `read`, 6 `run`, 1 `mutate`, 1 `admin`); the tables above. KA-755 (the
  exit-code table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal
  code of the surface's tables; exact) is exercised for every verb by steps (6) and (7).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`). A
  verb answering with two envelopes or exit code 1 fails step (7).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Kill gate KILL-INGEST (real-place ingestion cannot reach a scale label of `witnessed` or better): fallback, a real
  place is authored in the interior grammar from a tape-measure survey and the pack says it is a proxy;
  `scene.import` keeps its asset form.
- Closing rule: the bead closes at M2, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M2, M3, M5. The
  per-milestone surface beads (BD-222, BD-312, BD-409) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`read` → `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` →
  `<cluster>:write` or a named write scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope),
  plus `export:*` when the row exports and `private:*` where privacy applies, never a scope named after the
  object read or the engine spent on; N32b-01's catalogue generates the forms. `<cluster>:run` and
  `<cluster>:write` imply `<cluster>:read`, closed when a set is stored (at minting and at derivation,
  `Scopes::close_implied`), so checks stay plain set inclusion.
  Inputs above give the spellings the implementing beads fix (round 3 entered them for every row, so that a script
  can follow KNOWN_DRIFTS 8); the snapshot then pins them (additive changes keep the schema's major version;
  anything else raises it). An option a mechanism bead adds later is requested of this bead and entered here before
  a script calls it.

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §19.3, §19.4, §19.5, §19.6, §19.7, §19.8,
§19.11, §19.12, §19.13, §19.14, §19.15, §37.6, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02;
INV-TETHER-01, -02; KA-755.
