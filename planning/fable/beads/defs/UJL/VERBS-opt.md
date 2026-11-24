---
key: "VERBS-opt"
title: "frob-verbs registry: cluster `opt`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M4"
labels: ["m4", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-62"
deps: ["BD-022", "N14-REF", "N16-REF", "N17-REF", "N20-REF", "N23-REF", "N24-REF", "N28-REF", "N29-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.8", "§28.11", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)", "§19.12 (step 4)", "§20.24", "§16.21"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `opt`. The cluster covers optimization campaigns: starting an
optimization or a race, its status, the quality-diversity archive, and the six-stage co-design cascade. It has four
verbs (2 `read`, 2 `run`); the handlers live in `frob-opt`. The bead carries every row completely, with the three
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

In: the four registry entries of cluster `opt`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `opt.archive` | read | M4 | S | The quality-diversity archive of a campaign | §28.8 |
| `opt.cascade` | run | M4 | F | The six-stage co-design cascade, with a control in every stage; `solve.run` calls it | §28.8, §28.11 |
| `opt.start` | run | M4 | S | Start an optimization or a race; `--family` evaluates a family per dispatch | §28.8 |
| `opt.status` | read | M4 | S | Progress, survivors and spend of a campaign | §28.8 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.

**`opt.archive`** (MCP tool `opt_archive`; owner `frob-opt`; `paginated = true`: a long cell list is paged against
a committed frontier, and the generator appends 806 and 807)
- Inputs: `--campaign <id>`; optional `--top <n>`, `--stratum <id>`.
- Outputs: the campaign map's elites, meant for choosing what to do next: per elite its cell, descriptors with
  units, provisional score, selection standing, lineage link (the edit path from the campaign's root design) and
  the generation it entered; the coverage and quality summaries. It holds no number that `phylum.map` on the same
  campaign does not. A `warnings` entry states the bias when the entries were scored under one shared policy.
  Evidence: *pass-through* (class Tested, the weakest tier and determinism class among the entries, standing
  `provisional`).
- Chapter refusals: none of its own.
- Implemented by: BD-342, N17b-18 (the store is N17b-29's).

**`opt.cascade`** (MCP tool `opt_cascade`; owner `frob-opt`)
- Inputs: `--spec <mission or task>` (a registered task, or a task or mission specification file);
  `--population <set>` (the candidate designs); budget; `--seed`; `--allocation fixed|conformal`; `--stage <n>`
  with `--campaign <id>` (one stage on a stored campaign); `--manifest <id>` (run from a `solve.run` manifest,
  BD-372); `--plan-only` (the campaign plan and the mode of every rung and stratum, nothing run); optional
  `--finalists-from <rung>` (choose finalists from a named rung; refused with 545 when that rung is not
  `selective` in a stratum); `--estimate` (the cost model per stage); `--mutant <name>` (conformance builds
  only: the entry lists it in `conformance_only = ["mutant"]`, N32a-02, so a release build's schema has no such
  property and answers it as an unknown parameter, 802; loaded as every `--mutant` is, by the one loader of
  N21b-42 with the mutants of N35-15, so the receipt and envelope carry `conformance_mutant` and the run is
  stored as non-citable; BD-346 defines `skip-stage5`, which replaces stage 5 by a no-op so that the ranking is
  `unchecked`, and BD-346 and N28b-28 call `--mutant skip-stage5` as GATE-ATEL-10's negative control).
- Outputs: the six-stage co-design cascade's ranked finalists with verdicts and typing (`model-robust` or
  `model-conditional`), and per stage its control, audit and spend; the phase ledger; the fallbacks that fired;
  discards by stage with reasons. Progress events carry denominators; exit 2 when labels are present. Evidence:
  *study*.
- Chapter refusals: `FROB-REF-537` (a rung used to select or discard without a rank-agreement record that meets its
  threshold); `FROB-REF-545` (a rung asked to select in a stratum whose mode is not `selective`); `FROB-REF-704` (a
  campaign-class job without an approved compute envelope); `FROB-REF-871` (a decision-core table that is missing,
  stale against its contract, or not the registry version); and four codes the cascade passes on from the stages
  it runs, which the studio's refusal registry N28-REF lists under this verb: `FROB-REF-546` (calibration from
  survivors, or an audit-lane episode cancelled or re-weighted: the audit lane N28b-13, for example a stage run
  with `--stage` on a stored campaign whose calibration set holds a design the keyed audit draw did not),
  `FROB-REF-548` (ledger rows or calibration pooled across two model identities: the rank-agreement ledger
  N28b-22, for example a stage run on a stored campaign after a kernel-version change), `FROB-REF-549` (the
  stage-5 bound requested over battery-selected or search-selected episodes: BD-276 raises it for the cascade,
  where a direct `nemesis.bound` call answers 560) and `FROB-REF-870` (stage 2: an inherited verdict carried
  across an edit without a grade and a licence, refused by BD-347's consumer check `revalidate::require_licence`,
  which BD-352 calls before the stage uses the verdict, for example `--stage 2 --campaign <id>` on a stored
  campaign whose inherited verdicts were never revalidated; repair: `frob result revalidate <id>`). A verb must
  not return a code its row does not list, so the row lists them; N28-REF's generated-artifact check compares
  its verbs columns with these lists. The
  cascade types a finalist `model-conditional` and raises neither of the two codes that guard such a result: it
  sets the condition that the citation check reads (BD-346). One request raises one code (N16b-04's citation
  check, precedence in N28-REF): a `model-conditional` result offered to a build packet, for promotion or for
  prototype budget is refused with `FROB-REF-830` (the spend code, raised by the receiving verb, for example
  `eval.promote` or `foundry.export`), never 557; a `model-conditional` ranking presented as a design comparison
  without the variant that flips it (for example `report campaign --compact` dropping the condition columns) is
  refused with `FROB-REF-557` by the verb that presents it (`report.design`, `report.campaign`). Neither code is
  in this row.
- Implemented by: BD-352 (with BD-347's consumer check for 870 at stage 2). Extended by: BD-346 (the
  conformance-build mutant `skip-stage5` of `--mutant`).

**`opt.start`** (MCP tool `opt_start`; owner `frob-opt`)
- Inputs: exactly one of `--task <id>` (a registered task) or `--spec <file>` (a task or mission specification
  file; both, or neither, is 802), except under `--objective flip-verdict`, which takes neither (below);
  `--design <id>`; `--levers <list>`; `--method` (`cma-es`, `tr-bo`, `mf-bo`, `riemann-qn`, the four adapters of
  N28b-07; a method not admitted in the build is the appended 724, exit 5); `--objective <metric:min|max>`;
  `--population <n>` (default: the batch capacity); `--generations <g>`; `--race alpha=…,rounds=…`; `--seed`;
  budget (`gpu_s`, `cpu_core_s`); `--override-alias diagnosis` (the third repair of 445, N20a-41: the run is
  admitted on a mission with a standing alias witness, every result is labelled `diagnostic: standing alias
  witness` and is barred from promotion and from being cited as evidence that the mission is solvable).
  `--family <kind>` evaluates a family per dispatch (N28b-09, [F]): `sweep` (`--sweep <lever>=<lo>:<hi>:<n>`, one
  or two levers), `stencil` (2P + 1 designs on common random draws; a gradient estimate per lever), `generation`
  (the candidates of one `ask`), with the design named by `--design` as in every other call (never positionally:
  one spelling for agents and the CLI).
  `--objective flip-verdict` (N28bx-01) searches for the cheapest priced design edit that flips a named failing
  verdict: `--verdict <id>` (required with it; the task, the design and the world addresses are the verdict's, so
  `--task`, `--spec` and `--design` are not given), optional `--max-edits <n>` (1 or 2, default 1; above 2 is 802)
  and `--max-price <amount>` (the largest price delta tried).
  `--gradient adjoint` ([M], M6, N16c-26): projected steps along the adjoint design gradient with the lever
  manifold's retraction, taken only when the gradient is labelled `smooth`, `uninformative: false` and its
  agreement cosine is at least 0.65; otherwise the generation takes the rank-based update; every step's end point
  is re-evaluated (a gradient is class `Estimated`). Until N16c-26's handler is in the build, and after
  KILL-STEPADJOINT falls back, the option answers the appended 724, exit 5. There is no horizon input: each
  generation's gradient uses K = min(the task's horizon in control intervals, 32), the default of
  `phylum.gradient`'s `--horizon`, and N16c-26's agreement guard halves it (K → max(8, ⌈K/2⌉)) while the
  agreement cosine is below 0.65. Before the first generation the step-buffer store for K and the generation's
  worlds is computed; a store over the device budget is refused at admission, before any world steps, with the
  refusal of the ledger slug `adjoint-store-over-budget` (below), repairs (1) a smaller `--population`, (2)
  drop `--gradient adjoint` (the rank-based update alone). The run never falls back silently to the rank-based
  update, because the caller asked for a method the run would then not use.
  The `--spec` file may carry a section `sensing` ([F], N20b-36, built only on the fund branch of the visibility
  pilot and removed if KILL-VISIBILITY fires): the movable sensor-mount and payload-bay levers with their bounds
  and the objective and constraint terms (worst lead margin, worst blind-interval excess, route blind dwell,
  route minimum of the observability Gramian index), searched as a multi-objective problem or a scalarization the
  section declares; the levers and objectives are the section's, so `--levers` and `--objective` are not given.
- Outputs: `campaign_id` and the job handle (exit 8 while running). With `--family`: the family table (paired
  scores by parameter value; or gradient estimates with intervals, divergence steps and the label `Estimated`,
  `truncated-horizon` where pairs diverged; or ranks and scores with `Cancelled` members) and the designs decided
  per dispatch. With `--objective flip-verdict`: the result `frob.opt.flip/1`, reported through `opt.status`
  (claim `Minimum`, `FeasibleCandidate {gap}`, `NoneFound` or `NothingToFlip`; the edit with its price delta and
  provenance, the new `DesignId`, the controller route and any new policy, the candidates tried and why each
  failed); exit 0 when an edit is found, 2 when none is or the verdict is not a failure (`NothingToFlip`, a
  finding and not a refusal); `data_flags: ["prices: assumed"]` whenever a price is assumed. With a `sensing`
  section: the recommended patch of typed edits with each objective before and after, the constraint margins,
  Γ_obs before and after, closed-loop success under estimated and privileged state on the confirmation set, the
  static metrics labelled screening, and the statement that no estimator bound is claimed. Evidence: *study*.
- Chapter refusals: `FROB-REF-111` (a lever outside its bounds, or a variable that is not a declared lever);
  `FROB-REF-445` (a training or optimization run is requested on a mission with a standing alias witness for its
  interface, found by N20a-41's lookup `alias::standing_witness`; the witness is attached; repairs: read the
  witness, apply a separating change, `--override-alias diagnosis`); `FROB-REF-540` (gradient unreliable:
  non-smooth metric, horizon above the admitted cap, pairs diverged early, or an uncontrolled determinism class;
  `--family stencil`); `FROB-REF-541` (a family member outside the family policy's generator support; passed
  through with the member named while the rest is scored); `FROB-REF-542` (a race under an uncontrolled
  determinism class, or with arms in different classes); `FROB-REF-543` (a race on a metric with no declared
  span); `FROB-REF-545` (`--method mf-bo` with a fidelity rung whose mode in the campaign's stratum is not
  `selective`, an unmeasured stratum counting as `uninformative`; raised at admission before any world is
  created, BD-341 under N28b-07's multi-fidelity rule; repairs: run the audit lane and the rank-agreement
  measurement for the stratum, or use a single-fidelity method such as `tr-bo`); `FROB-REF-556` (a pairing keyed
  by event order or by a training counter); `FROB-REF-704` (a campaign-class job without an approved compute
  envelope); `FROB-REF-761` (the campaign needs an envelope class above the one in force); `FROB-REF-869` (under
  `--objective flip-verdict`: the named verdict's citation does not resolve, its evidence closure dangling or
  tombstoned, N28bx-01). And two refusals that the plan states without a number, each on the row under the
  number the allocation ledger assigns its slug (`registries/refusal_allocations.toml`, field `assigned`, filled
  by N35-33's first-round run at M0, before the row is entered; this bead prints neither number), raised behind
  one named constant: `foil-as-candidate` (N24-REF's table; a base design or a realized candidate that carries
  the foil marker `foil: true`, found by N24c-23's one candidate-admission function; exit 3, `retry: never` for
  that design; repair: none for the foil, submit a real design; if KILL-FOIL falls back, N24c-26, it is
  retired, never reused, and leaves the row) and `adjoint-store-over-budget` (N16-REF's table; from M6,
  N16c-26: under `--gradient adjoint`, the step-buffer store for K control intervals and the generation's
  worlds exceeds the device budget, refused at admission with no world stepped; exit by the family exit rule
  of the family the ledger files it in, BD-005; the two repairs above).
- Implemented by: BD-341 (with N28b-07's adapters and multi-fidelity rule). Extended by: N28b-09 (`--family`),
  N28bx-01 (`--objective flip-verdict`), N20b-36 (the `sensing` section), N16c-26 (`--gradient adjoint`, its
  horizon rule and the admission refusal `adjoint-store-over-budget`, M6).

**`opt.status`** (MCP tool `opt_status`; owner `frob-opt`)
- Inputs: campaign.
- Outputs: state, generation, evaluations (started, completed, cancelled), survivors with lever values and score
  intervals, brackets, the best provisional candidate with its `DesignId`, the optimizer state hash of the
  generation, spend by tier in device-seconds against the budget, the envelope class in force, warnings and
  `next_actions`; the optional block `race_ledger` (the race ledger records and the campaign roll-up with the
  pooled saving), absent until N28b-08 lands; for a campaign started with `--objective flip-verdict`, its
  progress (candidates tried of those generated) and, when done, the result `frob.opt.flip/1` (N28bx-01). Every
  number carries the claim product and is labelled `provisional`. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-341. Extended by: N28b-08 (`race_ledger`), N28bx-01 (the flip result).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `opt.archive` | `opt:read` | `frob.opt.archive.out/1` | `search` | `frob opt archive --campaign camp:12 --top 3 --json` |
| `opt.cascade` | `opt:run` | `frob.opt.cascade.out/1` | `ladder` | `frob opt cascade --spec stairs-0.4 --population seeded-81 --budget gpu_s=3600 --seed 3 --key cas-1 --json` |
| `opt.start` | `opt:run` | `frob.opt.start.out/1` | `search` | `frob opt start --task stairs-0.4 --design quad-a --levers thigh,knee_ratio --method cma-es --population 16 --budget gpu_s=600 --key opt-1 --json` |
| `opt.status` | `opt:read` | `frob.opt.status.out/1` | `search` | `frob opt status camp:12 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "opt.cascade"
cluster = "opt"
kind = "run"
tier = "F"
capabilities = ["opt:run"]
input_schema = "frob.opt.cascade.in/1"
output_schema = "frob.opt.cascade.out/1"
refusals = ["FROB-REF-537", "FROB-REF-545", "FROB-REF-546", "FROB-REF-548", "FROB-REF-549", "FROB-REF-704", "FROB-REF-870",
            "FROB-REF-871"]
conformance_only = ["mutant"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["ladder"]
examples = ["frob opt cascade --spec stairs-0.4 --population seeded-81 --budget gpu_s=3600 --seed 3 --key cas-1 --json"]
owner = "frob-opt"
purpose = "The six-stage co-design cascade, with a control in every stage; `solve.run` calls it"
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
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M4); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `opt.cascade` is the mechanism; `solve.run` is the agent-facing orchestration verb that calls it and adds no
  evidence of its own. If kill gate KILL-SOLVE fires and the `solve.run` row is withdrawn (VERBS-solve), the
  tool `opt_cascade` takes the vacated seat in the task cluster `search` (NJL-05's rule): the row's
  `task_clusters` becomes `["ladder", "search"]` in the same change, an additive change to its help snapshot.
- Every stage of the cascade carries a control, and its screening audit is `nemesis.audit`.
- Agreement with the studio's refusal registry (N28-REF, whose verbs columns are compared with these lists).
  `opt.cascade` carries every code N28-REF names for it, 870 at stage 2 among them (537 belongs to the learning
  chapter's registry). `opt.start` lists the two N28-REF names for it: 545 (BD-341 under N28b-07's
  multi-fidelity rule, `--method mf-bo`) and 543 (BD-341: an e-process race on a metric with no declared span,
  a race being an `anytime` study).
- Spellings settled: the archive is read with `--campaign` and no action word (N17b-18 and BD-342 now write
  `frob opt archive --campaign …`); the cascade takes its brief by `--spec` (BD-352), the population by
  `--population` (N28b-28); the former example's `--space` is no input of any implementing bead; the design of
  every `opt.start` call, `--family` included, is `--design` (N28b-09).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "opt"` (2 `read`, 2 `run`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster opt` exits 0 in process (`frob-mcp` over stdio) and through `frobd --mcp`:
      registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_opt.sh` runs each verb of the cluster (four), and from
      a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/opt/<verb>.help.json` byte for byte for each of the
      four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The script's run leaves a receipt with the measured counts (registry, CLI and MCP each 4; every difference
      list 0) and the controls caught. This bead records no gate value: the M4 surface bead BD-376 records
      GATE-TETHER-02's M4 part from that receipt with `cargo run -p frob-conformance --example record_gate --
      --gate GATE-TETHER-02 --receipt <hash> --part M4`, in a session that did not write the rows (N38b-19).

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_opt` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `opt.status`
  is `read` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`opt.archive`: none; `opt.cascade`: 537, 545, 546,
  548, 549, 704, 870, 871; `opt.start`: 111, 445, 540, 541, 542, 543, 545, 556, 704, 761, 869; `opt.status`:
  none); beside them, on `opt.start`, the refusals that the allocation ledger numbers, each compared by reading
  the `assigned` field of its slug's row in `registries/refusal_allocations.toml`, never by a typed number:
  `foil-as-candidate` and `adjoint-store-over-budget` (a planted row without one of them fails, and so does a
  slug whose ledger row has no assigned number); 602 and 557 and 830 are in no row of this cluster; the appended
  codes equal the rule for its kind and fields (`opt.archive` is paginated and also gets 806 and 807).
- `cascade_mutant_conformance_only`: the `opt.cascade` entry lists `mutant` in `conformance_only`; a release
  build's schema has no `mutant` property and answers `frob opt cascade … --mutant skip-stage5` with 802; a build
  with the `frob-verbs` feature `conformance` (through `gauntlet-mutations`) accepts it; the help snapshot does
  not contain it.
- `inputs_by_row`: `opt.start` accepts `--family sweep|stencil|generation` with `--design`, `--override-alias
  diagnosis`, `--gradient adjoint` and `--objective flip-verdict --verdict v:1 --max-edits 2 --max-price 400`
  without `--task` or `--spec`; it refuses with 802 `--task` with `--spec` together, `--objective flip-verdict`
  without `--verdict`, `--max-edits 3`, a positional design, and `--horizon` (no such input: the adjoint
  horizon is K = min(task horizon, 32)); `opt.cascade` accepts `--allocation`,
  `--stage`, `--manifest`, `--plan-only` and `--finalists-from`; `opt.archive` accepts `--campaign`, `--top`,
  `--stratum`; a planted `--space` fails.
- `mcp_names_one_to_one`: the tool names are the mapped names (`opt_archive`, `opt_cascade`, `opt_start`,
  `opt_status`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of convention 2 (`envelope_scope` among them) and refuses
  `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_opt.sh` (header: `bead=VERBS-opt milestone=M4 labels=tether,verbs,verb:opt.* gates=GATE-TETHER-02
seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper and
`scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance opt`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster opt`
over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check --cluster
opt`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob
job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each implemented
verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`,
`evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_opt/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
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

- Campaign-class jobs need an approved compute envelope (`FROB-REF-704`).
- A training or optimization run on a mission with a standing alias witness for its interface is refused
  (`FROB-REF-445`).
- Closing rule: the bead closes at M4, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-376) re-run this script as
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

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §28.8, §28.11, §32.4, §32.6, §32.12.3, §32.18,
§32.21; §19.12 step 4 (flip-verdict), §20.24 (sensing section), §16.21 (adjoint steps); GATE-TETHER-02;
INV-TETHER-01, -02; KA-755.
