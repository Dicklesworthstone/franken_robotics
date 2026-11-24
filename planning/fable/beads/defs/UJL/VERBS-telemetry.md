---
key: "VERBS-telemetry"
title: "frob-verbs registry: cluster `telemetry`, nine rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "chronicle", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§29.4", "§29.5", "§29.7", "§29.8", "§29.9", "§29.10", "§29.12", "§20.9", "§32.15", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `telemetry`. The cluster covers reading recorded runs: a
token-budgeted summary, columns by stream and range, the energy ledger, failures by class, histograms, the first
divergence of two runs, stills with overlays, the quantity under a pixel, and the self-contained HTML replay. It has
nine verbs (9 `read`); the handlers live in `frob-chronicle`. The bead carries every row completely, with the three
checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an
end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the nine registry entries of cluster `telemetry`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `telemetry.divergence` | read | M2 | S | First divergence between two runs, in exact mode along the digest chain or in tolerance mode | §29.8 |
| `telemetry.energy` | read | M1 | S | The energy ledger and the created-energy audit | §29.9 |
| `telemetry.export-html` | read | M2 | S | A replay as one self-contained HTML file | §29.12; viewer text in §32.15, §32.24 |
| `telemetry.failures` | read | M1 | S | Failures by class, with first offending events | §29.10 |
| `telemetry.histogram` | read | M2 | S | Deterministic integer log-bin histograms and quantiles | §29.7 |
| `telemetry.inspect-pixel` | read | M2 | S | The quantity under a pixel with its unit, channel and world address | §32.15 |
| `telemetry.query` | read | M1 | S | Columns by stream and range, paged against a committed frontier | §29.4, §29.5 |
| `telemetry.stills` | read | M2 | S | A still or a contact sheet with overlays | §20.9 |
| `telemetry.summary` | read | M1 | S | A token-budgeted run summary with handles for drill-down | §29.12 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`telemetry.divergence`** (MCP tool `telemetry_divergence`; owner `frob-chronicle`)
- Inputs (BD-206): two runs (positional); `--world <w>`; `--mode exact|tolerance` (default exact when the pair is in
  an exact class against each other, tolerance otherwise; exact on a pair that is not is `FROB-REF-802` with the
  classes named); `--from <tick>`, `--to <tick>`; `--scales <profile>`; `--no-replay` (answer from stored data
  alone: a chunk-resolution interval or `unresolved`, never a guessed tick).
- Outputs: the first divergent tick, stage and channel; exit 2 when the runs part or coverage gaps limit the
  answer. Evidence: *pass-through* (the claim product of the two records; the report's `caveat` says it adds no
  claim).
- Cost: static, except that a per-tick capture replay that needs a device runs as a job, and the verb then
  returns exit 8 with a job handle and is re-read afterwards (BD-206; the one exception to a `read` row's static
  cost, stated in its `cost_model`; the row carries no `wait` field).
- Chapter refusals: `FROB-REF-840` (a sealed confirmation run); `FROB-REF-888` (a requested range has no samples at
  all); `FROB-REF-894` (a per-tick capture replay would restore a snapshot of another numerical profile as a
  continuation; repair: a comparison replay, which the verb then runs in tolerance mode).
- Implemented by: BD-206.

**`telemetry.energy`** (MCP tool `telemetry_energy`; owner `frob-chronicle`)
- Inputs (N24a-37, with the filters written as `telemetry.query` writes them, so that a handle from one verb is a
  valid call of the other): `<run>…` (a run or a set of runs) `[--world <set>] [--episode <set>] [--from
  <instant>] [--to <instant>] [--by <axis>] [--per-task] [--cot] [--budget <tokens>]`, where a set is a list of
  indices and inclusive ranges (`0-36`), an instant is a tick or a simulated time with unit `s` (an exact
  rational, `--from 2s --to 4s`) mapped inward to the physics ticks, and the axis of `--by` is `actuator`,
  `joint`, `limb` or `phase`.
- Outputs: the energy ledger by line, joint, limb and phase, the numerical residual, the created-energy audit flags;
  with `--per-task`, the figure conditional on completion with its `non_physical` block. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-865` (a per-task figure without the `non_physical` block while excluded attempts
  exist: a summary statistic that omits what it excluded); `FROB-REF-888` (a figure over an interval that contains
  dropped telemetry).
- Implemented by: BD-094, N24a-37.

**`telemetry.export-html`** (MCP tool `telemetry_export_html`; owner `frob-chronicle`)
- Inputs (N24a-41): run (positional); `--world <address>`; optional `--episode`, `--window <t0>:<t1>`, `--fps`,
  `--overlay <list>`, `--forks <list>`, `--bodies <subtree>`, `--cap <bytes>`; `--out <file>` (under the
  session's export root); or `--check <file>` (re-check a written packet).
- Outputs: one self-contained HTML replay file (`frob.replay-packet/1`; at most 15 MB, no network fetch; its header
  repeats the three no-authorization flags), its path or artifact handle, its byte size against the cap computed
  before writing, and the size table. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-731` (a private object, or anything derived from one, without the private scope and
  flag); `FROB-REF-733` (the licence set of the artifact forbids this export); `FROB-REF-816` (an artifact whose
  authorization flags are missing or not `false`); `FROB-REF-734` (`--out` outside the session's export root, by
  the path jail; N37-REF's list test requires it of every verb with a path argument); and the refusals of the
  implementing bead N24a-41, none of which leaves a partial file: `FROB-REF-562` (a fork overlay for a run whose
  determinism class does not license replay), `FROB-REF-840` (a run in the blinded confirmation lane before its
  campaign closes). A fork overlay before the harness gates have passed is the appended `FROB-REF-724`. Two
  refusals of N24a-41 are N32-REF ledger requests held under N35x-01 (rule 8 of N35-33: the `8xx` family's free
  numbers are held), both `8xx`, exit 3, with no interim code: `packet-over-byte-cap` (a packet over its byte
  cap; carries the size table and the three reductions: half the frame rate, fewer forks, bodies restricted to a
  named subtree) and `packet-frames-not-retained` (frames requested from a chunk that is neither stored nor
  pinned; names the chunk keys and their retention state). Neither has a number; the row's `refusals` receives
  each number in the change that registers it, and no bead types one. The row has `exports = true`.
- Implemented by: BD-191, N24a-41.

**`telemetry.failures`** (MCP tool `telemetry_failures`; owner `frob-chronicle`)
- Inputs (BD-094): a run (or a set); optional filters (class, kind, world range); a token budget.
- Outputs: failures by class and kind with the denominator (episodes planned) and the frontier the counts were
  computed at; per class the first offending events with drill-down handles; the first-cause table. Evidence:
  *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-094.

**`telemetry.histogram`** (MCP tool `telemetry_histogram`; owner `frob-chronicle`)
- Inputs (N29a-15): run (positional); `--channel <name>` (a declared channel; an unknown name is `FROB-REF-802`
  with the nearest declared names); `--scope run|world|traced` (default `run`; `world` with `--world <address>`);
  `--quantile <p>` (repeatable; a decimal or rational probability in (0, 1]); `--bins` (include the sparse bin
  list; default off above 64 non-empty bins, where the list is paged); `--against <run>` (a side-by-side answer);
  `--method` (default `histogram`); `--at-frontier` (an optional pin); `--resolution` (a relative width in (0, 1]
  that changes no number and adds a `next_actions` entry when an interval is wider). Paginated (`paginated = true`
  for the bin list; 806 and 807 are appended, a cursor mismatch reaching the agent as 806 with the store's 896 as
  its cause).
- Outputs: deterministic integer log-bin histograms and quantiles, each quantile as the bounds of the bin that
  holds it with its count; `cursor` and `frontier` when the bin list is paged. Exit 2 when a requested quantile
  lies in an overflow bin or a count is saturated. Evidence: *pass-through* (determinism D0 for the aggregate).
- Chapter refusals: `FROB-REF-840` (a sealed confirmation run); `FROB-REF-888` (the channel has gaps in the
  requested scope); `FROB-REF-898` (an order-dependent aggregate requested on the evidence path: any `--method`
  other than `histogram`).
- Implemented by: BD-207, N29a-15.

**`telemetry.inspect-pixel`** (MCP tool `telemetry_inspect_pixel`; owner `frob-chronicle`)
- Inputs (N24a-40): a still, given as its `still_id` (positional) or as `<run> --world <address>` with the frame,
  view, overlay and size options of `telemetry.stills` (`--episode`, `--at`, `--views`, `--overlay <list>`,
  `--size`) and their defaults, which address exactly the still that verb returns; `--pixel <x>,<y>`
  (repeatable; image coordinates, sheet coordinates on a contact sheet); `--overlay <name>` (one overlay of the
  still); `--grid <n>` (n ≤ 16: the identifier kind and name at the centres of an n × n grid per tile, with a
  histogram of names by pixel count).
- Outputs: the quantity under the pixel with its unit, channel and world address: the identifier (link, fixture,
  region or contact, or `none` with a reason), `range_m` and the hit point with its frame, normal, tick and camera
  pose; per overlay the stored value from the overlay value buffer (`null` with reason `no-element` where the
  pixel shows none) and a drill-down handle into `telemetry.query` that returns the same stored integer.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-731` (a still of a private scene without the private read scope); `FROB-REF-840` (a
  still of a run in the blinded confirmation lane before its campaign closes); `FROB-REF-888` (`detail:
  still-collected`: a `still_id` whose buffers were collected by retention and cannot be redrawn, the trace being
  collected too; exit 3, naming the tombstone: the `still_id`, the collection's ledger sequence and the chunk
  keys that would have redrawn it; N24a-40). A pixel outside the image is the appended 802.
- Implemented by: BD-191, N24a-40.

**`telemetry.query`** (MCP tool `telemetry_query`; owner `frob-chronicle`)
- Inputs (N24a-36's one spelling): `<run> [--world <set>] [--episode <set>] [--stream <name>] [--channels
  <list>] [--from <instant>] [--to <instant>] [--period <duration>] [--reduce <rule>] [--aggregate <names>]
  [--traced-worlds]`: world and episode sets of indices and inclusive ranges; one of the nine streams (omitted
  when every named channel belongs to one stream); declared channel names (`--channel` is not a second
  spelling); a closed range whose instants are ticks or simulated times with unit `s`, mapped inward (`--ticks`
  is retired); the target sample period; an explicit reduction (`sample`, `envelope`, `mean`, `sum`, `count`,
  `last`, `min`, `max`); `--aggregate` over the registered integer aggregates; `--traced-worlds`. Paginated
  (`paginated = true`): `--limit` (the page size in rows, further bounded by the caller's token budget; there
  is no `--page-size`), `--cursor` and `--follow` are the common properties the generator adds, and the appended
  list adds 806 and 807.
- Outputs: columns by stream and range, downsampled by channel kind, with gaps marked by interval and reason,
  paged against a committed frontier. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-888` (a metric computed over missing samples); `FROB-REF-898` (an aggregate outside
  the registered integer aggregates, through `--aggregate`, N24a-36); `FROB-REF-883` (a read of a sealed chunk
  without a score token: BD-387's read guard, M5, N30-REF). A reduction the column's kind does not admit is the
  appended 802 with `detail: reduction-not-admitted`, naming the column, its kind and the admitted rules. A
  cursor used with a different pin, filter or order reaches the agent as the appended 806, with the store's
  `FROB-REF-896` as its cause (N29-REF's `surfaces_as`), so 896 is not in the row's list.
- Implemented by: BD-094, N24a-36. Extended by: BD-387 (M5: 883).

**`telemetry.stills`** (MCP tool `telemetry_stills`; owner `frob-chronicle`)
- Inputs (BD-191): run (positional); `--world <address>`; `--episode <n>`; `--at <selector>` (`failure`, the
  default when the episode has a terminal failure; `keyframes`, the default otherwise; `first-cause`, `decided`,
  `binding`, `event:<class>`, `t=<seconds>`, `tick=<n>`); `--views <list>` (`orbit`, `top`, `side`, `follow`,
  `sensor:<name>`; default the first four); `--overlay <list>` (`contact-cones`, `energy`, `actuator`,
  `balance`, `sensing`; from M3 `film`, `clearance`, `ghost`, `world-diff` and `reality-band`, N24ap2-60, whose
  data do not exist at M2); `--size <W>x<H>`
  (default 320 × 240, at most 1,920 × 1,080); `--sheet | --single`; `--max-tiles <n>` (default 16); `--depth`.
  From M3 (N24ap2-60): `--fork <fork path>` (repeatable, at most four; the forks `ghost` draws) and `--band
  <result handle>` (a stored `ground.band` result that `reality-band` draws). From M6, Frontier (N20ap2-01):
  `--quality standard|hero` (default `standard`) and `--spp <n>` (samples per pixel, default 256, accepted only
  with `hero`; otherwise 802).
- Outputs: a still or a contact sheet (PNG) with overlays: `still_id`, the image handle (also a `frob://`
  resource over MCP), the identifier and range buffer handles, per tile its view, tick, simulated time, event and
  camera, the legend per overlay, `drawn_from: "collision-field"`, the privacy mark; `requested_tick` and
  `drawn_tick` where the nearest stored tick is drawn. With `hero`, an EXR and a PNG preview with a
  `RenderIdentity` header, never an observation (N20ap2-01). Evidence: *pass-through*.
- Cost: static, except that a sheet larger than the wait budget runs as a job and the verb returns exit 8 with a
  job handle (BD-191; the same exception as `telemetry.divergence`, stated in its `cost_model`; the row carries no
  `wait` field).
- Chapter refusals: `FROB-REF-562` (a `ghost` for a run whose determinism class does not license replay,
  N24ap2-60); `FROB-REF-731` (a private object, or anything derived from one, without the private scope and flag);
  `FROB-REF-840` (a still of a run in the blinded confirmation lane, for a session without the administrative
  capability, BD-191); `FROB-REF-888` (exit 3, retry `after-change`: an untraced world, one with nothing stored,
  whose block names the sampling rule and the manifest field that would have retained it; or an overlay whose
  channel the run declares but did not retain for that world at the drawn instant, naming the channel and the
  manifest field; BD-191). `--single` over `--max-tiles`, a fifth `--fork` and `--spp` without `hero` are the
  appended 802, and so is an overlay whose channel the run does not declare at all (`sensing` on a run with no
  sensor; from M3 `film` on a scene with no film), with `pointer` `/overlay` and `available`, the list of
  overlays this run can draw (BD-191); an overlay id whose bead is not in the build (`film`, `clearance`,
  `ghost`, `world-diff` and `reality-band` before N24ap2-60) answers the appended 724, reason `not-linked`, whose
  `mechanism` names the overlay, with the list of available overlays (BD-191, convention 5). No overlay is ever
  drawn blank.
- Airlock requirement (N20ap2-01): `needs_groups = [{ groups = ["tracer"], when = "stills-quality-hero" }]`
  (N33c-32's predicate form; DESIGN predicate name, registered by N20ap2-01), so a binary without the group
  answers `--quality hero` with the appended `FROB-REF-790`, exit 5, at admission.
- Implemented by: BD-191. Extended by: N24ap2-60 (M3: `film`, `clearance`, `ghost`, `world-diff`,
  `reality-band`, `--fork`, `--band`, 562), N20ap2-01 (M6: `--quality`, `--spp`, the `tracer` group).

**`telemetry.summary`** (MCP tool `telemetry_summary`; owner `frob-chronicle`)
- Inputs (N24a-35): a run or job (positional); an optional world filter; `--budget <tokens>` (default 800, at least
  120: a smaller budget is the appended 802 naming the floor); `--encoding json|compact`. From M2, `--digests`
  (BD-187: the digest table, per world address the digest of every stream the run recorded by kind, a stream not
  recorded listed `absent`; GATE-SENS-02 is decided on it). From M8, `--table <name>` with the one value
  `tile-ledger` (N16c-20: one row per tile with its ending clause, islands, groups, uncertified pairs and running
  totals; read by N16c-21).
- Outputs: a token-budgeted hierarchical summary with denominators, the frontier and handles for drill-down; with
  `--digests`, the digest table; with `--table tile-ledger`, the tile ledger. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-094, N24a-35. Extended by: BD-187 (M2: `--digests`), N16c-20 (M8: `--table tile-ledger`).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `telemetry.divergence` | `telemetry:read` | `frob.telemetry.divergence.out/1` | `runs`, `repair` | `frob telemetry divergence r:19e2 r:5c11 --json` |
| `telemetry.energy` | `telemetry:read` | `frob.telemetry.energy.out/1` | `diagnose` | `frob telemetry energy r:19e2 --json` |
| `telemetry.export-html` | `telemetry:read`, `export:report` | `frob.telemetry.export-html.out/1` | `runs` | `frob telemetry export-html r:19e2 --world 311 --out replay.html --json` |
| `telemetry.failures` | `telemetry:read` | `frob.telemetry.failures.out/1` | `simulate` | `frob telemetry failures r:19e2 --json` |
| `telemetry.histogram` | `telemetry:read` | `frob.telemetry.histogram.out/1` | `runs` | `frob telemetry histogram r:19e2 --channel tilt --json` |
| `telemetry.inspect-pixel` | `telemetry:read` | `frob.telemetry.inspect-pixel.out/1` | `diagnose` | `frob telemetry inspect-pixel r:19e2 --world 311 --at failure --pixel 412,230 --json` |
| `telemetry.query` | `telemetry:read` | `frob.telemetry.query.out/1` | `diagnose` | `frob telemetry query r:19e2 --stream contact --from 10s --to 12s --json` |
| `telemetry.stills` | `telemetry:read` | `frob.telemetry.stills.out/1` | `simulate`, `places` | `frob telemetry stills r:19e2 --world 311 --at failure --overlay contact-cones --json` |
| `telemetry.summary` | `telemetry:read` | `frob.telemetry.summary.out/1` | `simulate`, `learn` | `frob telemetry summary r:19e2 --budget 600 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. `paginated = true` on `telemetry.query` and
`telemetry.histogram`. `exports = true` on `telemetry.export-html` only. The `cost_model` of
`telemetry.divergence` and of `telemetry.stills` names its job case (a capture replay on a device; a sheet larger
than the wait budget). `needs_groups` is the predicate form on `telemetry.stills` (above) and empty on the other
eight rows.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "telemetry.divergence"
cluster = "telemetry"
kind = "read"
tier = "S"
capabilities = ["telemetry:read"]
input_schema = "frob.telemetry.divergence.in/1"
output_schema = "frob.telemetry.divergence.out/1"
refusals = ["FROB-REF-840", "FROB-REF-888", "FROB-REF-894"]
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; a per-tick capture replay that needs a device runs as a job and the verb returns exit 8 with a job handle (BD-206)"
first_milestone = "M2"
task_clusters = ["runs", "repair"]
examples = ["frob telemetry divergence r:19e2 r:5c11 --json"]
owner = "frob-chronicle"
purpose = "First divergence between two runs, in exact mode along the digest chain or in tolerance mode"
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them (`run`:
   `sync_wait_ms` from 0 to the row's `wait.mcp_max_ms`, 30,000, with MCP default 5,000; `idempotency_key`, CLI
   `--key`; `budget`; `estimate_only`, CLI `--estimate`; `on_cancel`, `leave` (the default: the job keeps
   running) or `cancel_job`; `envelope_scope`, CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`, the
   approved compute-envelope scope the job cites (NJL-01; a citation that contradicts the token of the launcher
   the job runs under is 761 with pointer `/envelope_scope`, N35-REF); `mutate`: `idempotency_key` and
   `expected_parent`). `idempotency_key` is never in the JSON-schema `required` array: the annotation
   `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the rule, and the admission
   transaction refuses a missing key with 805 on both surfaces. The surface-wide refusals appended by kind:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. The properties and codes driven by fields are those of NJL-01's
   variation table: `cursor`, `limit` and `follow` with 806 and 807 for a paginated entry or a paginated action;
   808 (an integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property at any
   depth, whatever its kind (every `run` entry has one, `sync_wait_ms`); `estimate_only` on a non-`run` entry
   with `estimable = true`; no 805 for a `key_optional` entry; the `action` property, with each action's codes,
   for an entry with `actions`; 790 for a non-empty `needs_groups`; 891 for an `artifact-input<F>`,
   `artifact-ref<F>` or `artifact-identity<F>` parameter, and 892 as well for `artifact-identity<F>`; 893 for a
   `foreign-file` parameter; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The later variations of that table (10 to 13) land with their beads
   and are read there, not copied here.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).
5. Options (DESIGN). An option or option value a row lists whose
   implementing bead is not in the build (`telemetry.summary --digests` and `--table`, the M3 overlays and
   `--fork`, `--band`, `--quality`) is in the input schema and answers one envelope `FROB-REF-724`, exit 5, naming
   it, never 802 and never a blank drawing (BD-191's `later_overlay_ids_refused_until_built`). A value outside an
   option's domain stays 802. `--quality hero` in a build whose implementing bead is present but whose binary
   lacks the `tracer` group is 790, not 724.

**Notes for this cluster.**

- Nine verbs, all `read`. Paginated answers are pinned to a committed frontier; a cursor used with changed filters
  is refused (`FROB-REF-806`) and one whose frontier was collected is refused (`FROB-REF-807`).
- `telemetry.export-html` belongs to this cluster; the viewer it packages is specified with the surface. One HTML
  file of at most 15 MB (TARGET), no network fetch.
- `telemetry.inspect-pixel` exists so that an agent that cannot see still reads every overlay: it returns the
  quantity under a pixel with its unit, channel and world address.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly nine entries with `cluster = "telemetry"` (9 `read`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster telemetry` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 9, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_telemetry.sh` runs each verb of the cluster (nine),
      and from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/telemetry/<verb>.help.json` byte for byte for each
      of the nine, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_telemetry` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 9 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `telemetry.summary` is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`telemetry.divergence`: 840, 888, 894;
  `telemetry.energy`: 865, 888; `telemetry.export-html`: 562, 731, 733, 734, 816, 840, with `exports = true`,
  and the numbers of `packet-over-byte-cap` and `packet-frames-not-retained` read from
  `registries/refusal_allocations.toml` by slug once the ledger assigns them, none typed in the test;
  `telemetry.failures`: none; `telemetry.histogram`: 840, 888, 898; `telemetry.inspect-pixel`: 731, 840, 888;
  `telemetry.query`: 883, 888, 898, and no 896; `telemetry.stills`: 562, 731, 840, 888; `telemetry.summary`:
  none); the appended codes equal the rule for its kind and fields (`telemetry.query` and `telemetry.histogram`
  paginated, with 806 and 807; `telemetry.stills` with 790 from its `needs_groups`; 808 on every row whose
  generated schema has an `int53` property).
- `option_rows`: the inputs of the blocks are properties of the generated schemas (`telemetry.histogram`:
  `channel`, `scope`, `world`, `quantile` as an array, `bins`, `against`, `method`, `at_frontier`, `resolution`;
  `telemetry.stills`: the BD-191 options with `sheet` and `single` exclusive, `fork` with `maxItems = 4`, `band`,
  `quality`, `spp`; `telemetry.inspect-pixel`: the positional `still_id` or `run` with `world`, `pixel` as an
  array, `overlay`, `grid` with `maximum = 16`; `telemetry.summary`: `budget` with `minimum = 120`, `encoding`,
  `digests`, `table` with the one value `tile-ledger`; `telemetry.query`: `world`, `episode`, `stream`,
  `channels`, `from`, `to`, `period`, `reduce` with its eight values, `aggregate`, `traced_worlds`, and the
  generated `limit`, `cursor` and `follow`, with no `ticks`, `channel` or `page_size`; `telemetry.energy`:
  `world`, `episode`, `from`, `to`, `by` with its four values, `per_task`, `cot`, `budget`); an option of an
  absent bead answers 724 (convention 5), and `telemetry.stills --overlay film` in a build without N24ap2-60 is
  724 naming the overlay, not 802.
- `needs_groups_stills`: `telemetry.stills` carries `[{ groups = ["tracer"], when = "stills-quality-hero" }]`,
  passes `needs-groups-known` once N20ap2-01 registers the predicate, and its generated list contains 790; the
  other eight rows have an empty list and no 790.
- `divergence_job_case`: `telemetry.divergence` is `read` with no `wait` field and its `cost_model` names the job
  case; `--no-replay` never returns exit 8.
- `mcp_names_one_to_one`: the tool names are the mapped names (`telemetry_divergence`, `telemetry_energy`,
  `telemetry_export_html`, `telemetry_failures`, `telemetry_histogram`, `telemetry_inspect_pixel`,
  `telemetry_query`, `telemetry_stills`, `telemetry_summary`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_telemetry.sh` (header: `bead=VERBS-telemetry milestone=M1 labels=tether,verbs,verb:telemetry.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance telemetry`. Steps: (1) `frob capabilities
--json` and `frob robot-docs --json` list the nine verbs with kind and milestone; (2) `frob-surface-diff parity
--cluster telemetry` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff
snapshot --check --cluster telemetry`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2),
a job handle (exit 8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is
absent; (5) for each implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`,
`exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a
misspelt parameter (`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_telemetry/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 9
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 9 verbs (9 `read`); the tables above. KA-755 (the exit-code table on malformed,
  unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's tables;
  exact) is exercised for every verb by steps (6) and (7).
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

- Kill gate KILL-REPLAYPACK: fallback, `run.export` to the episode file, stills with overlays and the live viewer;
  `telemetry.export-html` then answers the not-implemented refusal.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M2. The per-milestone
  surface beads (BD-137, BD-222) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
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
- Round 3 (settled requests). Applied: S51 and S91 (`--digests`; `--table tile-ledger`), S60 (the histogram
  inputs, `paginated = true`, 840 and 888; the stills inputs of BD-191 and N24ap2-60 with 840 and 562; the
  inspect-pixel still-id form, `--overlay`, `--grid`, 731 and 840), S87 (`--quality`, `--spp`, the `tracer`
  group). In corrected form: BD-187 is listed under "Extended by", not "Implemented by" (it adds an option to a
  verb that BD-094 and N24a-35 implement); 790 and 802 of S87 are appended codes (by `needs_groups` and for every
  verb), not chapter codes; the `tracer` need is the predicate form of N33c-32, which N20ap2-01 calls
  argument-dependent. Found in the review and applied: 865 and 888 on `telemetry.energy` (N24a-37); 898 on
  `telemetry.query` (N24a-36's `--aggregate`); the inputs of energy, failures and query as their beads define
  them; the job case of `telemetry.stills` (BD-191 §9).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §29.4, §29.5, §29.7, §29.8, §29.9, §29.10,
§29.12, §20.9, §32.15, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
