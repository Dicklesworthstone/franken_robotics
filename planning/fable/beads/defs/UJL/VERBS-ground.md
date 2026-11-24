---
key: "VERBS-ground"
title: "frob-verbs registry: cluster `ground`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "ground", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "N30-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: ["BD-387", "BD-390", "BD-391", "BD-392", "BD-395", "FD-FSIM-23", "N30a-34", "N30a-47", "N30b-31", "N37-REF"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§30.5", "§30.9", "§30.11", "§30.16", "§30.19", "§32.4", "§32.6", "§32.10", "§32.12", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `ground`. The cluster covers calibration against reality:
admitting logs and measurements as data, identifiability and excitation design, staged identification, frozen
predictions scored once on sealed data, conformal reality bands per calibrated cell, and the replication kit. It has
seven verbs (2 `read`, 3 `run`, 2 `mutate`); the handlers live in `frob-ground`. The bead carries every row
completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match the list;
every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

Kinds: `read` returns stored or cheaply computed facts and changes nothing; `run` spends compute, takes an
idempotency key and a budget, and after a bounded synchronous wait returns a job handle with exit code 8; `mutate`
changes a named object in the caller's namespace and takes an idempotency key and an expected parent; `admin`
cancels, stops, evicts or deletes and needs a capability scope. Tiers: S Solid, F Frontier (with a pilot and a kill
gate), M Moonshot; `S/F` is a Solid core with Frontier extensions.

DESIGN: a registry bead per cluster exists beside the implementing beads because a row missing from the registry,
spelled twice, or served with different refusal lists on the CLI and over MCP is a surface defect that no mechanism
test catches. Rejected: rows left to the implementing beads alone (nothing checks their union against the
inventory); one bead for all the rows (too large to review).

The Ground unit drafted the same seven entries a second time (N30b-13, now superseded by this bead), from the
chapter's side. This is the one kept, because the registry of record for a cluster is its registry bead, and it
carries the content of both. Three things that draft corrected are carried in the rows below. The chapter's verb
table lists refusal numbers from an earlier numbering (840 to 850, which the registry assigns to other
subsystems), and its per-verb lists are shorter than the raising sites the Ground refusal registry (N30-REF)
records; the rows here use the registry's codes and its lists. Two of the seven verbs are not in the task
cluster `reality` that the registry shows an agent for calibration work, and a lint requires every verb to sit
in at least one. And one verb is registered as `read` while one of its forms changes state.

## Scope

In: the seven registry entries of cluster `ground`, complete and consistent: kinds; first milestones; input and
output schema names; capabilities; refusal lists; cost models; examples; the MCP tool names; task-cluster
membership; long-verb behaviour as the rows declare it; the `retire` action of `ground.band`; the cluster's
conformance script; the help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the
handlers: BD-387 `ground.ingest`, BD-390 `ground.excite`, BD-391 `ground.identify`, BD-392 `ground.predict`,
N30b-06 `ground.score`, BD-393 `ground.band`, BD-396 and N30b-12 `ground.kit`); the refusal rows themselves
(N30-REF); the catalogue row of the scope `ground:retire` (N32bx-02); the entry schema and lints (NJL-01); the
conformance harness (NJL-02); the alias lint (NJL-04); the definition of the task clusters (NJL-05, NJL-06);
generation of the surfaces (the Tether seed beads); the end-to-end run of all seven verbs on the rehearsal
(N30b-31) and of the refusal lists on planted violations (N30b-37).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `ground.band` | read | M5 | F | Issue or refuse a reality band; cells, coverage accounting, monitor states; the action `retire` needs a capability | §30.16 |
| `ground.excite` | read | M4 | F | Identifiability report and excitation plan for a design, a card set and a rig description; needs no data and never returns a fit | §30.16 |
| `ground.identify` | run | M5 | F | Staged identification; writes candidate card versions and the calibration record | §30.9 |
| `ground.ingest` | mutate | M1 for the row and the `reality:` line; M5 for admission of logs, bench measurements and scans | F | Admit a log, a bench measurement or a scan; fit clocks; seal splits; feed the monitors of calibrated cells | §30.5 |
| `ground.kit` | run | M5 | F | Build or check a replication kit | §30.16 |
| `ground.predict` | mutate | M5 | F | Freeze a prediction for a planned real trial | §30.11 |
| `ground.score` | run | M5 | F | Open sealed data once, score, retire the holdout, admit or reject the posterior | §30.11 |

Two rows answer later than their printed milestones, and the reason is the same for both: the gate register lands
Ground at M5. Their `first_milestone` keeps the appendix's value (the rule BD-137, NJL-01 and every registry bead
share; BD-137 names `ground.ingest` among the verbs whose seed bead lands later than the appendix), and the
handler decides what a build serves. `ground.excite` is printed M4; its handler (BD-390) and its gate are at M5,
and the gate may be recorded at M4, so the row says M4 and the verb answers `FROB-REF-724` until a build carries
BD-390. `ground.ingest` is printed M1 "for admission and the `reality:` line"; its admission handler (BD-387) is
at M5, so at M1 the row exists and the verb answers not-implemented, and the `reality:` line is carried by every
other verb's envelope. Both are declared late rows of NJL-02's closed list `LATE_ROWS` (lands M5), so the
capability-level coverage check accepts their 724 answer until M5 and not after. (Correction, round 1: an earlier text set `ground.excite` to M5, which contradicted the
shared rule and the per-milestone counts pinned to the appendix.)

**Contract of each verb.** Every output of this cluster has evidence class `Estimated`, fixed in its output
schema (the Ground chapter's decision that every Ground output has class Estimated, and the default of open
question OPEN-GROUND-5: nobody can promote an estimated candidate). The registry beads' usual labels are
narrowed accordingly for this cluster (the first draft of these rows had left `ground.excite` at class Tested
and the two reality verbs at "Tested or Estimated"):
- *reality*: a Reality claim only for a calibrated cell with a frozen prediction scored on sealed data; otherwise
  `reality: no-claim`; class Estimated. A band never raises an evidence class.
- *static*: tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is attached;
  `reality: no-claim`; class Estimated in this cluster.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`; the class is Estimated at most.
Exporting forms also carry `authorization` with the three flags false.

**`ground.band`** (MCP tool `ground_band`; owner `frob-ground`)
- Inputs, by form: the band (`--cell <c>` or `--design <built design>` with `--task-family`; `--metric <m>` named
  with its unit; `--level <1−α>` as an exact decimal; optional `--sided two|lower`, `--for <result>`); `--list`
  (cells with status, n per metric, levels available, coverage accounting); `--status [--cell <c>]` (monitor
  states with their power facts); the action `retire` (`--action retire --cell <c> --reason <text>`, needs the
  capability `ground:retire`; below).
- Outputs: a conformal reality band for the cell, or a refusal with samples present and samples needed; cells,
  coverage accounting, monitor states. Evidence: *reality*.
- Chapter refusals: `FROB-REF-889` (a band for an uncalibrated cell, or with too few calibration points);
  `FROB-REF-895` (a band for a design that was never built); `FROB-REF-897` (use of a retired calibration: The
  retirement reason; recalibrate); `FROB-REF-814` (pooling across cells, across `ModelId`s, or of synthetic with
  real data, in a calibration set or a monitor stream).
- Implemented by: BD-393.

**`ground.excite`** (MCP tool `ground_excite`; owner `frob-ground`)
- Inputs (BD-390 step 1): the design as the positional argument; `--cards <set>`; `--rig <rig>` (a rig file
  `frob.ground.rig/1` or the name of a rig stored in the workspace); optional `--params <names|stage:…>`
  (without it the analysis covers every field of the named cards that carries a prior uncertainty),
  `--budget-s <B>`, `--decision <partition.toml>` and `--seed <S>`. It needs no data.
- Outputs: identifiability report and an excitation plan. Evidence: *static*.
- Chapter refusals: `FROB-REF-879` (a fit on a parameter set that fails identifiability).
- Implemented by: BD-390.

**`ground.identify`** (MCP tool `ground_identify`; owner `frob-ground`)
- Inputs (BD-391 step 3): the dataset as the positional argument (it names its own cell); optional `--stage
  actuators|contact|inertia|sensors` (the fixed order, actuators first; without it the earliest stage with no
  record for this cell and lineage), `--params <names>` (the names `ground.excite`'s identifiability report
  prints), `--combination` (the identified combination of an aliased set in place of its members),
  `--keep-prior <stage>` (an earlier stage kept at its prior; N30a-20), `--law-class <class>` (the law class the
  fit is restricted to, one of the classes of N30a-18's `registries/ground/omitted_laws.toml`, for example
  `coulomb-viscous`: Coulomb plus viscous friction, τ_s = τ_c, no Stribeck term; a class that registry does not
  list is 802; without the option, the stage's full law class; the class fitted is named in the calibration
  record's `conditional_on`; GATE-GROUND-04's omitted-law clause runs through it, N30a-26), `--joints <list>`,
  `--windows <def | raw>` (a window definition is the windowed route; `raw` sends the raw rows through the
  streaming entry of `fs-assimilate`, added by FD-FSIM-23 at M5, and at a pin whose airlock has no streaming entry
  answers `FROB-REF-724` naming FD-FSIM-23, exit 5, before any fit; the calibration record names the route,
  `windowed` or `streaming`), `--cell <cell>` (a cross-check that must equal the dataset's cell); the idempotency
  key `--key`.
- Outputs: candidate card versions and the calibration record. Evidence: *screening*.
- Chapter refusals: `FROB-REF-814` (pooling across cells, across `ModelId`s, or of synthetic with real data);
  `FROB-REF-879` (a fit on a parameter set that fails identifiability).
- Implemented by: BD-391 (`--law-class` included).
- Extended by: FD-FSIM-23 (`--windows raw`, M5).

**`ground.ingest`** (MCP tool `ground_ingest`; owner `frob-ground`)
- Inputs (BD-387 step 6): a log, a bench measurement or a scan, as data, with its manifest (licence, provenance,
  rights, uncertainty), as the positional bundle; `--design <design>` and `--task-family <tf>` (with the manifest
  they determine the cell key; a bench measurement or a scan of one part, which has no task, says
  `--task-family tf:none`; a call that leaves a component of the cell undetermined is refused under 873 with
  the missing components named); the idempotency key `--key`; with a scan, optional `--emit-variant` (N30a-47:
  build the as-built model variant from the registered scan and return its identity).
- Outputs: dataset identity (`frob.ground.dataset/1`), `cell`, `origin`, `kind`, the split (counts per part and
  which are sealed), `sealed_commitment`, `private`, `already_admitted` (a bundle already admitted under another
  key returns the existing receipt and writes nothing), fitted clocks, and `reality` (unchanged by admission).
  For a scan (BD-395) the result adds `registration` (transform, scale, flags), `screen` and `quantities`, and
  with `--emit-variant` `variant`: the as-built variant's identity, a new `ModelId` with provenance `as-built`
  (N30a-47).
  An under-determined clock fit does not undo admission: the call completes with exit 2 and carries the full
  refusal object of 876 in `clocks[i].refused`, with time-based metrics reported unavailable (BD-389; N30-REF).
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-812` (an as-built registration that is ill-posed or below its noise screen);
  `FROB-REF-873` (a dataset lacking licence, provenance, rights or measurement uncertainty, or with an
  undetermined cell or an invalid split); `FROB-REF-876` (clock alignment under-determined, or a scored instant
  outside the bracket; at this verb carried inside an exit-2 result as above); `FROB-REF-814` (pooling across
  cells, across `ModelId`s, or of synthetic with real data); `FROB-REF-886` (a spent holdout chunk declared as
  holdout or validation in a later dataset); `FROB-REF-883` (a bundle that declares unsealed, as `calibration`
  or in a part missing from `sealed`, a chunk hash an earlier admission committed as sealed and that has not
  been scored: admitting it would open a read path with no frozen prediction; BD-387). Raised by the dispatcher
  and the importer on its file arguments, and listed because N37-REF's list test requires every importing verb
  to list them: `FROB-REF-828` (a file whose leading bytes are an executable, script, pickle or archive
  signature; N37-03's sniff) and `FROB-REF-832` (a scan or bundle over a limit of the import profile in force;
  N37-05). Raised by the handler and listed as a safety code: `FROB-REF-731` (a private-marked bundle admitted
  without the private scope and flag; exit 5; BD-387).
- A large bundle is admitted by chunked upload under the job protocol; the verb stays `mutate`.
- Implemented by: BD-387.
- Extended by: BD-395 (the scan form), N30a-47 (`--emit-variant`).

**`ground.kit`** (MCP tool `ground_kit`; owner `frob-ground`)
- Inputs: the form, `build` or `check`, as a positional argument (two forms of one verb, not two verbs, so that
  the cluster stays inside its limit of seven). `build`: `--result <score or band identifier>` (a rejected
  candidate, a `ranking-falsified` score or a retired cell are accepted: losing results ship too) and `--out <dir>`
  under the session's export root, required (a `build` without it is 802, pointer `/out`). `check`: the kit
  directory and optional `--store <empty dir>` (the self-test replays the kit against an empty store); `--out`
  with `check` is 802.
- Capabilities, by form (DESIGN, the form N37-18 gives an option that writes outside the store, as
  `nemesis.shrink` has it): the row carries `capabilities = ["ground:run"]` and `option_capabilities = { out =
  ["export:packet"] }`. `build`, which always has `--out`, needs `ground:run` and `export:packet`; `check`
  writes nothing outside the store and needs `ground:run` alone, so a third party checking a kit holds no export
  scope.
- Outputs: a replication kit (its identifier, the hash of the manifest; the file list with hashes; substitutions
  made; the classes of the expected outputs), or the check of one. There is no override flag for private inputs.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-899` (a private or non-redistributable input to a kit). The `build` form writes
  outside the store, so it goes through the export door (N37-18, a dependency of BD-396) and the path jail, and
  the row lists their codes, as N37-REF's list test requires of every exporting verb with a path argument:
  `FROB-REF-816` (an artifact whose no-authorization flags are missing or not `false`), `FROB-REF-731` (a private
  object without the private scope and flag), `FROB-REF-733` (a licence that forbids the export) and
  `FROB-REF-734` (`--out` outside the session's export root). 899 is the kit's own, earlier check on the
  lineage, so a private input is refused before the door is reached.
- Implemented by: BD-396.
- Extended by: N30b-12 (`ground.kit check`, the kit's self-test).

**`ground.predict`** (MCP tool `ground_predict`; owner `frob-ground`)
- Inputs (BD-392 step 2): `--dataset <id>` and `--part validation|holdout` (the planned real trial: the sealed
  part the prediction is for); `--cell <cell>`; `--model <id>`; `--scene <id>`; `--task <id>`; `--metric
  <name>:<level>` (repeatable); `--decision <spec>`; `--study fixed-n:<n>|anytime`; optional `--supersedes <id>`
  (the earlier version, which is also the expected parent); the idempotency key `--key`. There is no `--trial`
  option (an earlier example used one).
- Outputs: a frozen prediction record with its hash, the ledger sequence, the drawn trial conditions, the
  intervals and the pre-analysis of reachable verdicts. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-883` (a read of sealed data with no frozen prediction, or a prediction changed after
  freezing). Consumed from other subsystems and passed through unchanged, not listed as this cluster's own:
  `FROB-REF-532`, `-552` and `-102`.
- Implemented by: BD-392.

**`ground.score`** (MCP tool `ground_score`; owner `frob-ground`)
- Inputs: the frozen prediction as the positional argument (it names the sealed part it is scored on, which the
  call opens once); optional `--then calibration|regression` (what the spent part becomes, default `calibration`:
  unsealed into the cell's calibration pool and a holdout for no lineage; `regression`: kept sealed in the
  regression set, and a lineage that has not scored it may score it once; N30a-34); the idempotency key `--key`.
- Outputs: the score, the holdout retired, the posterior admitted or rejected. Evidence: *reality*.
- Chapter refusals: `FROB-REF-883` (a read of sealed data with no frozen prediction, or a prediction changed after
  freezing); `FROB-REF-886` (reuse of a spent holdout: Acquire fresh sealed data); `FROB-REF-876` (a scored
  instant outside the clock bracket).
- Implemented by: BD-392, N30b-06; `--then` by N30a-34.

**The per-verb refusal lists, in one place** (DESIGN: where the chapter's verb table and the Ground refusal
registry differ, the refusal registry governs, because the generated-artifact check compares the two registries
in both directions and the refusal rows record every raising site; the first draft of these rows had copied the
chapter's shorter lists): `ground.ingest`: 873, 876, 812, 814, 886, 883; `ground.excite`: 879;
`ground.identify`: 879, 814; `ground.predict`: 883 (consumed: 532, 552, 102); `ground.score`: 883, 886, 876;
`ground.band`: 889, 895, 897, 814; `ground.kit`: 899. To these the rows add the safety codes that N37-REF's
list test requires, or that the handler raises, and the Ground registry does not carry: 828, 832 and 731 on
`ground.ingest` (an importing verb whose handler refuses a private bundle without the private scope, BD-387)
and 816, 731, 733 and 734 on `ground.kit` (an exporting verb with a path argument). The schema refusal (`-802`) is appended to every
row by kind; no verb raises a code its row does not list (NJL-01, RL-06). (An earlier text had every verb
"pass through" 731 and 733 without listing them.)

**The `retire` action of `ground.band`.** One entry form (DESIGN, the form the scope bead N32bx-02 fixes; the
two drafts of these rows had called it "a second parameter set" and "an action of kind `admin`", which are the
same thing): the entry keeps `kind = "read"` and `capabilities = ["ground:read"]`, and declares
`actions = [{ name = "retire", kind = "admin", capabilities = ["ground:read", "ground:retire"] }]`. The action
is selected by the one input property `action` on both surfaces (NJL-01; no per-action flag): on the command
line `frob ground band --action retire --cell <id> --reason "<text>" --json`; over MCP `action: "retire"` of the
tool `ground_band`, shown as a separate optional group. It is of kind `admin`, so the generator gives it no
idempotency key, and no form of `ground.band` has one (`--key` on this tool is 802). The action is idempotent by
its effect, as `job.cancel` is: a retire of a cell already retired returns the stored retirement record with
`already: retired` and writes nothing, so a repeated call is safe without a key. The registry lint that forbids
state change in `read` verbs lists this action as its one exemption for the cluster. The three read forms of
the verb (issue or refuse a band; list cells and coverage accounting; list monitor states) need only
`ground:read`.

**Schemas.** `frob.ground.<verb>.in/1` and `frob.ground.<verb>.out/1` for each verb. The stored record schemas
they reference: `frob.ground.dataset/1`, `frob.ground.clockfit/1`, `frob.ground.calibration/1`,
`frob.ground.prediction/1`, `frob.ground.score/1`, `frob.ground.band/1`, `frob.ground.monitor/1`,
`frob.ground.asbuilt/1`, `frob.ground.kit/1`. Every output schema carries `evidence` with `class` fixed to
`Estimated`, `reality`, and for exporting forms `authorization` with the three flags false.

**Long verbs.** `ground.identify`, `ground.score` and `ground.kit` take the bounded synchronous wait and
otherwise return a job handle with exit code 8; the job completes with the same envelope a synchronous call
returns; progress always has a denominator (rigs, joints, calls replayed).

**Cost models** (the content of each row's `cost_model` line, under the rule below): `ground.ingest`: static,
linear in bundle bytes; `ground.excite`: static, seconds; `ground.identify`: estimated, minutes per joint;
`ground.predict`: static, one simulated trial per registered trial; `ground.score`: estimated, seconds per
trial; `ground.band`: static; `ground.kit`: estimated, minutes to an hour for a check.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `ground.band` | `ground:read`; `ground:retire` for the action `retire` | `frob.ground.band.out/1` | `reality`, `calibrate` | `frob ground band --cell c:9f2c41 --metric crossing_time_s --level 0.90 --json` |
| `ground.excite` | `ground:read` | `frob.ground.excite.out/1` | `calibrate` | `frob ground excite cart-a --cards drive-60 --rig stand-A --json` |
| `ground.identify` | `ground:run` | `frob.ground.identify.out/1` | `reality`, `calibrate` | `frob ground identify ds:stand-A-01 --key gi-1 --json` |
| `ground.ingest` | `ground:import` | `frob.ground.ingest.out/1` | `reality`, `calibrate` | `frob ground ingest stand-A-01/ --design cart-a --task-family tf:none --key gin-1 --json` |
| `ground.kit` | `ground:run`; `export:packet` with `--out` (`build`) | `frob.ground.kit.out/1` | `calibrate` | `frob ground kit build --result sc:4e1 --out kit/ --key gk-1 --json` |
| `ground.predict` | `ground:write` | `frob.ground.predict.out/1` | `reality` | `frob ground predict --dataset ds:sill-day-3 --part validation --cell c:9f2c41 --model cart-a --scene sill-12mm --task sill-crossing --metric crossing_time_s:0.90 --decision "crossing_time_s below 4.5" --study fixed-n:20 --key gp-1 --json` |
| `ground.score` | `ground:run` | `frob.ground.score.out/1` | `reality` | `frob ground score pred:b3:11a0 --key gs-1 --json` |

Each example runs on the synthetic rehearsal once its handler has landed (N30b-31), whose fixtures carry the
names the examples use (`cart-a`, `stand-A-01`, `ds:sill-day-3`, `c:9f2c41`, `sill-12mm`, `sill-crossing`):
N30b-31 stores the rehearsal objects under these names, adds the rows of `fixtures/verbs/identities.toml` for
the five abbreviated identities (`ds:stand-A-01`, `ds:sill-day-3`, `c:9f2c41`, `pred:b3:11a0`, `sc:4e1`), and
logs an example that cannot run as a skip with its reason. Two examples were corrected in round 3:
`ground.ingest` now carries `--design` and `--task-family`, without which BD-387 cannot form the cell and
refuses with 873 (a drive-stand bench bundle has no task, so `tf:none`); `ground.predict` used a `--trial`
option that BD-392 does not define, and now gives the handler's required inputs. The band example is the
chapter's form, with the cell identified by its hash, the metric named with its unit and the level given (the
first draft of these rows had `--metric crossing_time --cell cart/sill-12mm`; one example per row, and this is
the one). Capabilities follow the scope scheme of the surface (`<cluster>:read`, `<cluster>:run`, the named
scope `ground:import` for admission, `export:packet` for the kit's `build` form, and `<cluster>:write` for the
one `mutate` verb that is not an import); the Ground unit's draft had proposed `ground:write` for `ingest` and
`identify`, a separate `ground:score` for `ground.score`, and "the session's export scope" for the kit. They are
not adopted: the scope catalogue of BD-258, which fixes the names, has no separate scope for opening sealed
data, and `ground.score` needs `ground:run`.

Task clusters. The principal task cluster for calibration work is `reality`: `ground_ingest`, `ground_identify`,
`ground_predict`, `ground_score`, `ground_band`, `predict_report`, `report_run` (seven tools).
`ground_excite` and `ground_kit` are not in it; they sit in the task cluster `calibrate` that the task-cluster
bead adds (NJL-05: `ground_excite`, `ground_kit`, `ground_ingest`, `ground_identify`, `sense_calibrate`,
`analyze_identifiability`, `ground_band`), so that the lint passes and an agent that is planning a bench session
or packaging a result sees the tools for that. (The Ground unit's draft had proposed a cluster `reality-plan`
for the same two verbs; there is one definition of the task clusters, NJL-05's.)

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. `ground.kit` has `exports = true` (its `build` form carries
the three no-authorization flags) and `option_capabilities = { out = ["export:packet"] }`, which BD-258's lint
accepts as the place of an exporting row's publish scope.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "ground.identify"
cluster = "ground"
kind = "run"
tier = "F"
capabilities = ["ground:run"]
input_schema = "frob.ground.identify.in/1"
output_schema = "frob.ground.identify.out/1"
refusals = ["FROB-REF-814", "FROB-REF-879"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M5"
task_clusters = ["reality", "calibrate"]
examples = ["frob ground identify ds:stand-A-01 --key gi-1 --json"]
owner = "frob-ground"
purpose = "Staged identification; writes candidate card versions and the calibration record"
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
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Seven verbs, the limit for one registry cluster of this chapter.
- Refusal codes. The chapter's verb table prints chapter-local numbers that predate the refusal registry (840 to
  850), which collide with the qualification codes. The registry codes used here are those of the chapter's refusal
  table, matched by meaning: 840 → `FROB-REF-873`, 841 → 876, 842 → 879, 843 → 883, 844 → 886, 845 → 889, 846 → 895,
  847 → 897, 848 → 899, 849 → 812, 850 → 814.
- `ground.ingest` is registered at M1. Its admission handler (BD-387) is at M5: at M1 the entry exists and answers
  not-implemented (`FROB-REF-724`), and the `reality: no-claim` line that every report prints is carried by every
  other verb's envelope. (The two drafts of these rows read the appendix's "M1 for admission and the `reality:`
  line" differently; this is the one reading, which follows from the handler's milestone.)
- `ground.excite` is registered with first milestone M4 (the appendix's) and answers from the build that carries
  BD-390 (M5; its gate may be recorded at M4).
- Imported text is data and is never interpreted as instructions; every echo of it carries the untrusted mark.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "ground"` (2 `read`, 3 `run`, 2 `mutate`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster ground` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_ground.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/ground/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] No entry lists a refusal code in 840 to 850; the lists equal those of the refusal registry in both
      directions (`xtask check-generated`).
- [ ] `frob capabilities --json` lists the seven with the kinds and milestones above; the cluster has exactly
      seven verbs and a lint fails on an eighth.
- [ ] The MCP surface serves seven tools for the cluster with the mapped names; the task clusters `reality` and
      `calibrate` have at most seven tools each; every `ground` verb is in at least one.
- [ ] Every output schema of the cluster fixes `evidence.class` to `Estimated`. Each example in the registry
      parses against its input schema now; once its handler has landed it runs on the rehearsal and returns an
      envelope that validates against its output schema with `evidence.class = "Estimated"` and
      `evidence.reality = "synthetic"` (asserted by the end-to-end run N30b-31).
- [ ] The action `retire` of `ground.band` without `ground:retire` is refused with the capability refusal; the
      three read forms need only `ground:read`. `ground.kit check` is admitted with `ground:run` alone; `build`
      needs `export:packet` as well.
- [ ] From their milestone on, `identify`, `score` and `kit` return exit code 8 with a job handle when
      `sync_wait_ms` elapses, and the job completes with the same envelope a synchronous call returns (re-run by
      the M5 surface bead BD-409).
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_ground` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `ground.score` is `run` at M5, `ground.excite` is `read` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table; the stored record schemas resolve.
- `refusal_lists`: each row's chapter codes equal its block (`ground.band`: 889, 895, 897, 814; `ground.excite`:
  879; `ground.identify`: 814, 879; `ground.ingest`: 873, 876, 812, 814, 886, 883, 828, 832, 731; `ground.kit`: 899, 816, 731, 733, 734;
  `ground.predict`: 883; `ground.score`: 883, 886, 876); the appended codes equal the rule for its kind and fields;
  `refusal_lists_match_registry` (both directions against `registries/refusals.toml`); `no_stale_840_block`.
- `options_by_row`: the generated input schemas accept the options of each block (`ground.ingest`: `--design`,
  `--task-family`; `ground.excite`: `--cards`, `--rig`, `--params`, `--budget-s`, `--decision`, `--seed`;
  `ground.identify`: `--stage`, `--params`, `--combination`, `--keep-prior`, `--law-class`, `--joints`,
  `--windows` (a definition or `raw`), `--cell`; `ground.ingest` also `--emit-variant`; `ground.predict`:
  `--dataset`, `--part`, `--cell`, `--model`, `--scene`, `--task`, `--metric`, `--decision`, `--study`,
  `--supersedes`; `ground.score`: `--then`; `ground.band`: `--cell`, `--design`, `--task-family`, `--metric`,
  `--level`, `--sided`, `--for`, `--list`, `--status`, `--action` (`retire`), `--reason`; `ground.kit`:
  `--result`, `--out`, `--store`); `frob ground predict --trial x` is 802; `ground.identify --stage gearbox` is
  802; `ground.score --then archive` is 802; `ground.band --retire` and `ground.band --action retire --key k` are
  802.
- `kit_scopes_by_form`: `ground.kit build` without `export:packet` is refused with 720; `ground.kit check` with
  `ground:run` alone is admitted; `build` without `--out` and `check` with `--out` are 802.
- `ingest_clock_refusal_is_exit_2`: on the one-pulse clock fixture `ground.ingest` completes with exit 2, the
  dataset admitted and `clocks[0].refused.code = "FROB-REF-876"`; the envelope's own `refusal` is empty.
- `kit_forms`: `ground.kit` accepts exactly the positional forms `build` and `check`; a missing form, or a third
  (`emit`), is `FROB-REF-802`; a planted `--allow-private` property fails the schema test (`kit_has_no_override`).
- `output_class_fixed_estimated`: every output schema of the cluster has `evidence.class` fixed.
- `retire_action_needs_capability`: the action is refused without `ground:retire`; the read forms are not.
- `every_ground_verb_in_a_task_cluster`; `unimplemented_handler_returns_724`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`ground_band`, `ground_excite`, `ground_identify`,
  `ground_ingest`, `ground_kit`, `ground_predict`, `ground_score`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`,
  `budget`, `estimate_only`, `on_cancel`, `envelope_scope`) and refuses `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; a verb
  whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_ground.sh` (header: `bead=VERBS-ground milestone=M1 labels=tether,verbs,verb:ground.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance ground`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
ground` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster ground`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_ground/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (2 `read`, 3 `run`, 2 `mutate`); the tables above. KA-755 (the exit-code
  table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the
  surface's tables; exact) is exercised for every verb by steps (6) and (7).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`). A
  verb answering with two envelopes or exit code 1 fails step (7).
- Further negative controls on planted registries: `ground.band` listing `FROB-REF-846` (the stale-number lint
  fails); an eighth `ground` verb (the cluster-limit lint fails); a task-cluster assignment that omits
  `ground_excite` (the membership lint fails); an output schema whose class is free (the fixed-class test fails).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Without imported data every verb of the cluster still answers: `ground.band` returns `reality: no-claim` with the
  simulation's own interval. Nothing in the program waits for a bench.
- `ground.band --action retire` needs a capability scope and is the only mutating action of a `read` verb in this
  cluster; it is modelled as an action of kind `admin` on a `read` entry (the form given above), idempotent by
  its effect and without a key.
- The class of every output of this cluster is fixed in the schema, which is how open question OPEN-GROUND-5
  (who could promote an estimated candidate: nobody, by default) is enforced at the surface.
- Naming: the registry holds no second spelling. `ground.kit` has two forms (`build`, `check`) as a positional
  argument, not two verbs, to stay inside the limit of seven.
- `scripts/e2e/ground_rehearsal.sh` (bead N30b-31) calls all seven verbs in the order of the procedure through
  the CLI and, for three of them, through `frob-mcp`; `scripts/e2e/ground_refusals.sh` (bead N30b-37) covers the
  refusal lists. Both read the rows of this bead.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Handlers of this cluster land at M5 (`ground.excite` may be
  exercised from M4). The
  per-milestone surface beads (BD-137, BD-376, BD-409) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
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

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §30.5, §30.9, §30.11, §30.16, §30.19, §32.4,
§32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
