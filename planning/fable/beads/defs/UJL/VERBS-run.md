---
key: "VERBS-run"
title: "frob-verbs registry: cluster `run`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "chronicle", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "N18-REF", "N21-REF", "N29-REF", "N30-REF", "N34-REF", "N35-REF", "N37-REF", "N39-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§29.3", "§29.8", "§27.3", "§35.2", "§19.21", "§36.9", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `run`. The cluster covers recorded runs: a run's manifest and
standing, the list of runs, export of the episode bundle, and forks from a checkpoint. It has four verbs (2 `read`,
2 `run`); the handlers live in `frob-chronicle`. The bead carries every row completely, with the three checks that
keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an end-to-end script;
the schema printed by `frob <verb> --help --json` is stable.

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

In: the four registry entries of cluster `run`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `run.export` | run | M1 | S | Export a `.frun` bundle or a view of it | §29.3 |
| `run.fork` | run | M2 | S | A new branch from a run and a checkpoint with a change set; returns its class (one of the five of §27.4), the earliest valid tick and the null-replay result; `--classify-only` restores nothing | §29.8 |
| `run.show` | read | M1 | S | A run's manifest, standing and summary | §32.6 |
| `run.list` | read | M1 | S | Runs in the caller's namespace | §32.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**`run.export`** (MCP tool `run_export`; owner `frob-chronicle`)
- Inputs: run; `--out <path>` (under the session's export root; a directory receives
  `<first 16 hex digits of the RunId>.frun`, or `….<view>` for a view); optional filters `--worlds`, `--episodes`,
  `--streams` (and a tick sub-range, only with `--view`); `--view arrays|summary` (absent: the canonical bundle);
  `--at-frontier <seq>`; idempotency key.
- Outputs: the exported bundle or view under the session's export root, with its digest; a view's header carries
  `view_of: <bundle hash>`; the `authorization` block with the three no-authorization flags, all `false`
  (N29a-02), so the row sets `exports = true`, as `artifact.export` and `design.export` do. Evidence:
  *pass-through*.
- Chapter refusals: `FROB-REF-731` (a private object, or anything derived from one, without the private scope and
  flag); `FROB-REF-733` (the licence set of the artifact forbids this export); `FROB-REF-734` (the destination:
  `..`, an absolute path or a link that leaves the export root); `FROB-REF-816` (an artifact whose authorization
  flags are missing or not `false`); `FROB-REF-840` (a sealed confirmation run); `FROB-REF-883` (a read of sealed
  data with no frozen prediction: the export would reach a sealed validation or holdout chunk of a grounding
  dataset, whose only read path takes a score token that `ground.score` alone mints; BD-387's read guard, which
  refuses `artifact.read` and `telemetry.query` the same way; N30-REF); `FROB-REF-888` (a view or summary computed
  over an interval with a recorded gap); `FROB-REF-892` (a
  view offered where an identity is required). An integrity fault is exit 9.
- Implemented by: N29a-02. (Correction, round 1: the row also named BD-097, whose scope puts `run.export` of a
  whole bundle Out, to N29a-02.)

**`run.fork`** (MCP tool `run_fork`; owner `frob-chronicle`)
- Inputs: run; `--world <w>`; the fork point, `--at <tick or time>` or `--before failure [--dt <lead>]` (one of
  the two); the change set, `--change '<factor> := <value>'` (repeatable; the repeated options form one change
  set, classified change by change, the set's class the maximum, and run as one child with shared streams and no
  perturbation); `--reuse <checkpoint field>` (repeatable; the warm-start graft, the way 582 is reached; a field
  the change leaves valid is accepted and changes nothing); `--classify-only`; `--children <plan file>` (a batch
  of different children: the child plan `frob.fork.children/1`, a JSON array of child specifications in the
  canonical JSON of `ChildSpec`, N27a-01, omitted fields taking the defaults: no change, no perturbation, series
  `null`, index 0, `shared` streams, the request's horizon; the file is read through the path jail, so the
  generator appends 734 and 828 for it as a file parameter); `--co-tenants <seed>` (the children placed among as
  many unrelated filler worlds in a slot order permuted by the seed; it changes no address, stream key or digest);
  `--horizon <time>` or `--to-horizon` (one of the two: a horizon, or the episode's end, N27a-03's
  `ToEpisodeEnd`; default N27a-03's, the failure time plus 1 s); `--determinism
  {reference-replay|device-replay|cross-backend|fast-training}`, `--precision {default|stiff|f32|f64}` and
  `--on-degraded {refuse|continue-flagged}` (N35-04's enumerations, parsed by its shared parser and admission;
  see the notes); `--field-reuse auto|require|off` (default `auto`) and `--no-field-reuse` (§19.21's spelling of
  mode `off`) ([M], from M6, N19c-35; until then each answers `FROB-REF-724`); idempotency key. The plan's size
  is bounded by the free slots of the resident environment and by the budget through the ordinary admission
  estimate (the appended 701).
- Outputs: the fork handle (`fork`, with `parent`, `requested`, `tick`, `source`); `classified`, the class of each
  proposed change (one of the five fork classes) with its earliest valid tick, forcing fields and labels; the
  null-replay result (`bitwise`, or `mismatch` with the first differing tick and stage); `double_precision`
  (`decision-equivalent`, `not-equivalent` or `unresolved`, with both failure times, for a parent recorded in
  single precision; `same-precision` otherwise); `children` with their addresses; `refused`, the specifications
  of a plan that would be refused alone (581, 582), each with its code and fields, while the others run;
  `horizon_s`; from M6 the receipt's `field_reuse` block (N19c-35: `enabled`, `mode`, the measured a, d and b,
  the tile counts, `store_bytes`, `cost_ratio`, `reason`). With `--classify-only` no child carrying a change is
  restored; the fork point is still opened and its null fork run. Exit 0 complete with nothing to flag; 2 when
  the null replay is a `mismatch` on a call that asked for no child, when the double-precision check is
  `not-equivalent` or `unresolved`, or when some specifications of a plan were refused and the others ran; 8 with
  a job handle while children still run; 3 on the chapter refusals below except 604 (when no specification of a
  plan can run, the first refusal is the envelope's). Evidence: *run* (the parent's tier and determinism class,
  an empty `claims` list: a fork is a replayable experiment, not a claim about a design).
- Chapter refusals: `FROB-REF-245` (a half-precision value of `--precision`, refused at N35-04's admission step 1);
  `FROB-REF-247` (an N35-04 option not applicable to a fork: `--determinism fast-training`, which only training
  verbs take, or `cross-backend` without two named placements; the class check of N35-04's step 4 is 562 on this
  verb, by N35-REF's division); `FROB-REF-248` (a child world ill-conditioned beyond its precision profile under
  `--on-degraded refuse`; nothing partial is published); `FROB-REF-562` (attribution or a counterfactual fork was
  requested below class D2, including a device family whose batch-independence record is not current);
  `FROB-REF-580` (a continuation whose physical, clock, sensor or policy contract does not match the checkpoint);
  `FROB-REF-581` (a structural or design-parameter intervention requested at a mid-episode tick; the earliest valid
  tick and the forcing fields are returned; `--classify-only` reads the classification without raising it);
  `FROB-REF-582` (state from one model grafted onto another: a `--reuse` field the change set invalidates, or a
  checkpoint of a run under another `ModelId`); `FROB-REF-583` (children requested, by the request's own change set
  without `--classify-only` or by a `--children` plan, from a fork point whose null fork does not reproduce its
  parent; a call that only opens the point, bare or with `--classify-only`, completes with exit 2 and files the
  mismatch in the divergence ledger); `FROB-REF-604` (a `--precision` width the parent's tier has no step for;
  exit 6, zero worlds stepped; N35-04's admission step 2); `FROB-REF-840` (a sealed confirmation run);
  `FROB-REF-894` (a snapshot restored under another numerical profile and labelled a continuation). From M6: `FROB-REF-322` (`--field-reuse require` when a precondition of reuse fails, named in
  `failed_precondition`: `determinism-class` and `no-stencil` from N19c-34's emit site, `store-over-budget`,
  `a-below-threshold` and `version-mismatch` from N19c-35; exit 3, no world run, first repair mode `auto`; the
  number is the one N19c-42 proposes and the ledger assigns, read through N19c-42's constant
  `FIELD_REUSE_PRECONDITIONS`).
- Implemented by: BD-205. Extended by: N35-04 (the shared parser and admission of `--determinism`, `--precision`
  and `--on-degraded`, which BD-205's handler calls), N19c-35 (`--field-reuse`, `--no-field-reuse`, the
  `field_reuse` block; M6). Used by: N19c-34, N19c-36, N19c-37 (the field-reuse scripts), N35-31 (the M2
  licence steps).

**`run.show`** (MCP tool `run_show`; owner `frob-chronicle`)
- Inputs: run; optional `--budget <tokens>` (the summary's size, default 600); `--receipt` (the run's receipt in
  full; for a fork, with its `field_reuse` block, N19c-35); `--phase-ledger` (the run's phase ledger from
  N21a-27's step profile record, DESIGN name `frob.stepprofile/1`: wall time per phase of the job from submission
  to completion, and the share a of step time spent in field tile updates that the field-reuse rule reads, the
  profile's `field_tile_share`).
- Outputs: the manifest (identities, tier, determinism class, effort profile, chunk length, channel schemas, seed
  as a string); the standing (`provisional` or `established`; state `admitted`, `running`, `complete`, `failed`,
  `cancelled` or `determinism-violated`; the terminal receipt with its four counts; `frontier`; the gap count);
  the summary sized to the budget; progress always "k of n" with its unit; with `--receipt` the receipt block,
  with `--phase-ledger` the phase ledger, in one of two forms (N21a-27): for a run recorded with `sim.run
  --profile-kernels`, the run's `frob.stepprofile/1` (the phase ledger, the per-kernel rows and the derived
  figures, `field_tile_share` among them), exit 0; for a run recorded without it, the job-level phase ledger
  from the run's receipt with `profiling: false`, and the per-kernel rows, the stage shares and every derived
  figure that needs per-kernel time (`field_tile_share` included) as `unavailable` with the reason `run recorded
  without --profile-kernels`, never estimated, exit 2 (done with findings), the next action naming the same run
  request with `--profile-kernels`; the bytes-touched table, which does not depend on timing, is printed in both
  cases. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-840` (a sealed confirmation run).
- Implemented by: BD-097. Extended by: N19c-35 (`--receipt` with the `field_reuse` block), N21a-27
  (`--phase-ledger`, M2: it prints that bead's step profile; see the notes). Used by: N19c-35 (the
  enablement rule a ≥ 0.8 and the receipt), N19cp1-02 (stage A of KILL-FARFIELD).

**`run.list`** (MCP tool `run_list`; owner `frob-chronicle`)
- Inputs: optional filters `--state`, `--tier`, `--design`, `--task`, `--since`; `--limit`; cursor
  (`paginated = true`: the generator appends 806 and 807).
- Outputs: runs in the caller's namespace, ordered by (start instant, run identifier): identifier, state, tier,
  world and episode counts, start instant, design and task identities. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-097.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `run.export` | `run:run`, `export:report` | `frob.run.export.out/1` | `runs` | `frob run export r:19e2 --out exports/ --key rx-1 --json` |
| `run.fork` | `run:run` | `frob.run.fork.out/1` | `simulate`, `repair` | `frob run fork r:19e2 --world w:3fa9 --before failure --dt 1.0s --classify-only --key fk-1 --json` |
| `run.show` | `run:read` | `frob.run.show.out/1` | `runs` | `frob run show r:19e2 --json` |
| `run.list` | `run:read` | `frob.run.list.out/1` | `runs` | `frob run list --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. `params` (N34-24's kinds): `run.fork` declares `{ children =
"artifact-input<frob.fork.children/1>" }` (DESIGN, the rule VERBS-scene states: a document of this project with
its own schema is `artifact-input<F>` of that format, so NJL-01's variations 7 and 9 append 891, 734 and 828, and
the bead that defines the document, BD-205, enters its row in `registries/formats.toml`, which N34-27's rule
`fmt.param-without-row` requires of every schema a verb parameter names). The other rows take identifiers,
names and the export destination `--out`, whose 734 is in `run.export`'s chapter list.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "run.export"
cluster = "run"
kind = "run"
tier = "S"
capabilities = ["run:run", "export:report"]
input_schema = "frob.run.export.in/1"
output_schema = "frob.run.export.out/1"
refusals = ["FROB-REF-731", "FROB-REF-733", "FROB-REF-734", "FROB-REF-816", "FROB-REF-840", "FROB-REF-883", "FROB-REF-888", "FROB-REF-892"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M1"
task_clusters = ["runs"]
examples = ["frob run export r:19e2 --out exports/ --key rx-1 --json"]
owner = "frob-chronicle"
purpose = "Export a `.frun` bundle or a view of it"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
exports = true
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
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `run.fork` is the one registered verb for forks and their classification: it returns the branch, the inherited
  prefix, the restart time and the fork's class; `--classify-only` restores nothing. Attribution over a fork is
  `autopsy.intervene`.
- A continuation whose physical, clock, sensor or policy contract does not match the checkpoint is refused
  (`FROB-REF-580`); a snapshot restored under another numerical profile may not be labelled a continuation
  (`FROB-REF-894`).
- N35-04's options on a fork. The null fork runs on the originating backend and profile (BD-205, step 4), and a
  snapshot is bound to its `ProfileRecord` (N35-04), so a profile resolved from `--determinism` and `--precision`
  that differs from the parent's makes the restore a comparison, never a continuation, and a fork labelled a
  continuation under it is 894 (N29-REF; N27-REF divides 894 from 580). The options' defaults on a fork are the
  parent's recorded `ProfileRecord` (its determinism profile, its precision profile and width, and its
  degraded-world policy), not N35-04's per-tier defaults (BD-205: the parent's recorded profile is the default
  that keeps a bare fork a continuation; N35-04's per-tier default would not for a parent run under `stiff`).
  `--on-degraded` is not part of the pins and may differ from the parent's. BD-205's refusal table lists 245,
  247, 248 and 604, raised in the admission order 245, 247 and 604, then the licence (562), then the profile
  comparison (894), then classification.
- Codes whose registry lands after this bead. The cluster is registered at M1, and RL-06 (N32a-02) makes a code
  absent from `refusals.toml` a load-time error, so the M1 entry of `run.fork` lists only codes already entered:
  245 (N18-REF), 247, 248 and 562 (N35-REF), 604 (N21-REF), 840 (N39-REF), 894 (N29-REF). 580, 581, 582 and 583
  enter with N27-REF at M2 and are appended to the row's list in BD-205's change (BD-205 reaches N27-REF through
  N27a-02); 322 is appended in N19c-35's change (M6) under the number the ledger assigns (N19c-42). The block
  above is the row's complete list once those changes land; VERBS-prove stages 845 the same way. `run.export`'s
  883 needs no staging: N30-REF enters it at M1 and is a dependency of this bead, so the M1 entry lists it,
  although the sealed store that raises it lands with BD-387 at M5.
- Field reuse is Moonshot and adds no verb: it is reported by `run.fork` and read back by `run.show --receipt`.
  If KILL-FARFIELD fires, the `field_reuse` block stays in the receipt as `enabled: false, reason: "withdrawn"`
  and `--no-field-reuse` stays an accepted no-op (N19c-37), so neither option leaves the schema.
- `run.show --phase-ledger` is called by N19cp1-02's script, and N19cp1-02 cannot wait on N19c-35,
  which waits on it (N19c-35 → N19c-34 → N19cp1-02; KNOWN_DRIFTS 7). The option therefore belongs to the bead of
  the step profile it prints, N21a-27 (M2, a dependency of N19cp1-02), which builds it with the share
  a among its fields (`field_tile_share`); N19c-35 reads a from it and adds `--receipt`. The share is a per-kernel
  figure, so a caller that needs a records the run with `sim.run --profile-kernels` (VERBS-sim); on a run
  recorded without it the option prints a as `unavailable` and exits 2 (the row's Outputs).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "run"` (2 `read`, 2 `run`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster run` exits 0 in process (`frob-mcp` over stdio) and through `frobd --mcp`:
      registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_run.sh` runs each verb of the cluster (four), and from
      a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/run/<verb>.help.json` byte for byte for each of the
      four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The script's run leaves a receipt with the measured counts (registry, CLI and MCP each 4; every difference
      list 0) and the controls caught. This bead records no gate value: the per-milestone surface beads (BD-137 at
      M1, then BD-222) record GATE-TETHER-02's part for their milestone from such a receipt with `cargo run -p
      frob-conformance --example record_gate -- --gate GATE-TETHER-02 --receipt <hash> --part <M>`, in a session
      that did not write the rows (N38b-19).

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_run` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `run.list`
  is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`run.export`: 731, 733, 734, 816, 840, 883, 888, 892;
  `run.fork`: 245, 247, 248, 562, 604, 840, 894 at M1, with 580, 581, 582 and 583 from BD-205's change and 322
  from N19c-35's; `run.show`: 840; `run.list`: none); the appended codes equal the rule for its kind and fields
  (`run.list` is paginated and also gets 806 and 807; `run.fork`'s `--children` is declared
  `artifact-input<frob.fork.children/1>` in `params` and gets 891, 734 and 828); `run.export` has `exports = true`
  and no other row of the cluster has it.
- `inputs_by_row`: `run.fork` accepts `--world`, exactly one of `--at` and `--before failure` (with `--dt` only
  after `--before`), `--change` and `--reuse` repeated, `--classify-only`, `--children`, `--co-tenants`, at most
  one of `--horizon` and `--to-horizon`, `--determinism`, `--precision` and `--on-degraded` with N35-04's
  enumerations (a misspelt value is 802), and from N19c-35's change `--field-reuse` with its three modes and
  `--no-field-reuse`; `run.show` accepts `--budget`, `--receipt` and `--phase-ledger`; `run.list` accepts the
  filters `--state`, `--tier`, `--design`, `--task`, `--since` and `--limit`.
- `sealed_runs_840`: every verb of the cluster that takes a run lists 840 (§24: a confirmation run is sealed).
- `mcp_names_one_to_one`: the tool names are the mapped names (`run_export`, `run_fork`, `run_show`, `run_list`),
  with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of convention 2 and refuses `sync_wait_ms = 30001` with
  code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_run.sh` (header: `bead=VERBS-run milestone=M1 labels=tether,verbs,verb:run.* gates=GATE-TETHER-02
seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper and
`scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance run`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster run`
over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster
run`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob
job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented
verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`,
`evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_run/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 4 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 4 verbs (2 `read`, 2 `run`); the tables above. KA-755 (the exit-code table on
  malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's
  tables; exact) is exercised for every verb by steps (6) and (7).
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

- `run.export` is not named in any seed bead's title; a new bead of the Chronicle unit (N29a-02) carries it, with
  the artifact-verbs bead (BD-097).
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M2 (the field-reuse
  options of `run.fork` at M6, answering 724 before). The per-milestone
  surface beads (BD-137, BD-222) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`read` → `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` →
  `<cluster>:write`; `admin` → `<cluster>:admin` or a named admin scope), plus `export:*` when the row exports
  (`run.export`'s `export:report`), never a scope named after the object read; N32b-01's catalogue generates the
  forms, and `<cluster>:run` and `<cluster>:write` imply `<cluster>:read` (`Scopes::close_implied`).
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §29.3, §29.8, §27.3 (the
fork horizon), §35.2 (determinism and precision profiles), §19.21 (field reuse and its receipt), §36.9 (where time goes: the M2
step profile), §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
