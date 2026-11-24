---
key: "VERBS-autopsy"
title: "frob-verbs registry: cluster `autopsy`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "autopsy", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-26"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§27.5", "§27.6", "§27.7", "§27.8", "§27.12", "§27.14", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `autopsy`. The cluster covers counterfactual debugging:
interventions on a fork of a failed run, placebo forks, interventional contrasts and minimal repair sets. It answers
which change rescues a failing run, as a cause in the model. It has five verbs (2 `read`, 3 `run`); the handlers
live in `frob-autopsy`. The bead carries every row completely, with the three checks that keep the rows true: the
generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by
`frob <verb> --help --json` is stable.

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

In: the five registry entries of cluster `autopsy`, every field of each (this bead is the only place where the
rows are specified); the MCP tool names; the cluster's conformance script; parity of the CLI and MCP for the five
verbs; the help-schema snapshots; each verb's answer before its milestone.

Out, with the bead that has it: behaviour (the implementing beads named below); the checks of these rows that need
the handlers (N27b-16, which depends on this bead and enters no row: each registry example executes against the
fixtures, each refusal list covers what the verb's tests raise, the output schemas validate the worked artifacts,
the names `autopsy.fork`, `autopsy.divergence`, `autopsy.explain` and `autopsy.replay` are absent and
`frob autopsy fork` is answered by the unknown-verb refusal with `run.fork` as its repair; where such a check
shows a row to be wrong, the row is corrected here); the M3 verb sweep, the registry freeze and the M3 parity
record (BD-312, which depends on this bead and on N27b-16); the entry schema and lints (NJL-01); the conformance
harness (NJL-02); the alias lint (NJL-04); task clusters (NJL-05, NJL-06); generation of the surfaces (the Tether
seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `autopsy.attribute` | read | M3 | S | Contrasts, flip rates, pairs, labels and an optional Shapley summary for a fork | §27.6, §27.7 |
| `autopsy.intervene` | run | M3 | S | Run interventions, placebo forks, replicates and coalitions from a fork handle made by `run.fork` | §27.5 |
| `autopsy.repair` | run | M3 | S | Minimal repair sets, by shrinking the intervention set | §27.7 |
| `autopsy.rerun` | run | M3 | S | Paired re-run from the episode start for design-parameter and structural edits | §27.8 |
| `autopsy.show` | read | M3 | S | The transcript of an attribution session, token-budgeted | §27.12 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *model cause*: class Estimated, with scope "a cause in the model at this tier, among the listed factors"; never an
  established cause and never a cause in the physical robot.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`autopsy.attribute`** (MCP tool `autopsy_attribute`; owner `frob-autopsy`)
- Inputs: fork handle; optional `ledger` (the `ledger` field of an intervene result: the intervention set to read;
  by default the latest, BD-280); `forks` (several forks: one attribution per fork and a pooled table); `lane`
  (`auto`, the default: the latest set whatever lane ran it; `f64` or `tier-f32`: the latest set that ran in that
  lane); `shapley` (`auto`, the default, `never` or `force`: the Shapley summary is computed on a budget, after the
  contrasts; `force`, BD-281, prints one only from coalitions already run, since this verb runs no fork);
  `budget` in tokens.
- Outputs: `frob.autopsy.attribution/1`: contrasts, flip rates at two perturbation scales, pairs with interaction,
  labels (N27b-06's spellings: `but-for`, `contributing`, `joint`, `alternative`, `precedes-snapshot`,
  `precision-sensitive`, `chaotic: unattributable`, `marginal`, `tier-specific: <variant>`, `none`), the ledger
  read, placebo rate, refused interventions; optional Shapley summary. Evidence: *model cause*.
- Chapter refusals: `FROB-REF-553` (a comparison that mixes study types, or re-reads fixed-n data as a sequential
  study: raised for a pooled read over sets of both test types); `FROB-REF-562` (attribution or a
  counterfactual fork was requested below class D2); `FROB-REF-583` (the null fork does not reproduce its parent);
  `FROB-REF-584` (attribution requested for a failure that neither lane admits, or under an uncontrolled determinism
  class); `FROB-REF-840` (the run is a sealed confirmation run).
- Implemented by: BD-280. Extended by: BD-281 (the value `force` of `shapley`).

**`autopsy.intervene`** (MCP tool `autopsy_intervene`; owner `frob-autopsy`)
- Inputs: fork handle from `run.fork`, or instead `forks` (several fork handles) or `cluster`
  (`<run>[:<failure class>]`: every failing world of a run, or of one failure class), the bulk form on a device
  (N27b-27); in a conformance build the fork argument also accepts `fixture:host/<name>[:<case>]` (N27b-03's host
  fixtures; absent from a release build); `do` list (one declared mechanism each, at most 12 factors), optional:
  with none, the verb runs the null fork and the placebo forks only (1 + P forks), which is how an agent measures
  a failure's placebo flip rate before choosing factors; `placebo` (default 64); `replicates` (default 8);
  `coalitions` (`pairs`, the default under `test = fixed`, per OPEN-AUTOPSY-2's default "singles and pairs",
  whose decision N27b-13 may make `none` the default; `none`; `all` and `sampled`, BD-281; under `test =
  sequential` `none` is the default and the only value admitted); `shapley` (`auto`, the default, `never` or
  `force`, BD-281); `shapley_budget` (the forks the sampled summary may buy, default 512, BD-281); `horizon` (a
  duration or `episode`; default one second past the parent's failure time); `series_1e7` (`auto`, `always`,
  `never`); `replicate_by` (`perturb` or `rekey:<stream>`); `test` (`fixed`, the default, or `sequential`: forks
  bought in rounds, the test type declared in the ledger header before any fork runs); `max_pairs` (the round
  limit of the sequential mode, default 64); `allow_unreachable` (admit a request whose rescue rule cannot be
  reached at the requested replicates, and record `unreachable: true`); `lane` (`auto`, the default: the
  precision route decides; `f64`; `tier-f32`; BD-279, with the lane's half of `auto` and `tier-f32` from
  N27b-24); `reuse[]` (checkpoint fields to reuse, spelled as on `run.fork`, empty by default, BD-279);
  `continue_after_variant` (default false, N27b-25); `diagnostic_force`; `plan_only`. CLI flags are the
  kebab-case spellings (`--test`, `--max-pairs`, `--allow-unreachable`, `--series-1e-7`, `--shapley-budget`,
  `--reuse`, `--continue-after-variant`, `--forks`, `--cluster`, …).
- Outputs: `{fork, forks_run, series (counts), placebo {n, flips, rate}, refused [{do, code,
  earliest_valid_fork_tick, forced_by}], ledger, tier, precision, lane}`: forks run, by kind; refused interventions
  with the earliest valid tick; the null-replay result; the fork ledger `frob.autopsy.forkledger/1` as a stored
  artifact; `next_actions` naming `autopsy.attribute` and, for each 581, the `autopsy.rerun` command. Evidence:
  *run*.
- Chapter refusals: `FROB-REF-553` (the two study types mixed on one intervention set, or a sequential set re-read
  as fixed: the ledger header fixes the type before any fork runs); `FROB-REF-562` (attribution or a
  counterfactual fork was requested below class D2);
  `FROB-REF-580` (a continuation whose physical, clock, sensor or policy contract does not match the checkpoint);
  `FROB-REF-581` (a structural or design-parameter intervention requested at a mid-episode tick; the earliest valid
  tick and the forcing fields are returned); `FROB-REF-582` (state from one model grafted onto another);
  `FROB-REF-583` (the null fork does not reproduce its parent); `FROB-REF-584` (a failure that neither lane admits,
  or an uncontrolled determinism class: BD-279 reads the precision route of N27b-07 at admission, and under
  `lane = f64`, or `auto` without the lane, a failure double precision does not reproduce is refused with it);
  `FROB-REF-586` (an intervention changes more than one declared mechanism); `FROB-REF-840` (the run is a sealed
  confirmation run).
- Implemented by: BD-279; N27bp2-01 (`test = sequential`, `max_pairs`, and the 553 it raises). Extended by:
  N27b-24 (`--lane tier-f32` and the lane's half of `auto`); BD-281 (`shapley`, `shapley_budget`, the
  `coalitions` values `all` and `sampled`); N27b-27 (`forks`, `cluster`); N27b-25 (`continue_after_variant`). An
  unreachable request without
  `allow_unreachable`, more than 12 factors and sequential-mode coalitions are `FROB-REF-802` (pointing at
  `replicates`, `do` and `coalitions`), which the generator appends to every row.

**`autopsy.repair`** (MCP tool `autopsy_repair`; owner `frob-autopsy`)
- Inputs: fork handle (its admitted interventions are the candidate set); optional `ledger` (the intervention set
  to shrink, by default the latest; the same rule as `autopsy.attribute`, N27b-14); `max_tests` (the forks the
  shrinker may buy); `include_diagnostic` (show diagnostic factors beside the repair sets; never inside them).
- Outputs: 1-minimal repair sets, each with rescue counts; diagnostic factors kept apart from repairs. Evidence:
  *model cause*.
- Chapter refusals: `FROB-REF-562` (the fork's attribution was requested below class D2); `FROB-REF-583` (the null
  fork does not reproduce its parent); `FROB-REF-584` (attribution requested for a failure that neither lane admits,
  or under an uncontrolled determinism class); `FROB-REF-586` (an intervention of the candidate set changes more
  than one declared mechanism); `FROB-REF-840` (the run is a sealed confirmation run).
- Implemented by: N27b-14 (BD-284 records a repair move in the transcript and implements no part of this verb).

**`autopsy.rerun`** (MCP tool `autopsy_rerun`; owner `frob-autopsy`)
- Inputs: run; one typed edit, either `edit` (a typed design edit: design parameter or structural) or `do` (`<factor>
  := <value>`); worlds (`failing`, `failing,sample:<n>` or an explicit list); `controller` (`keep`, `resynth`,
  `mpc` or `family:<id>`); `placebo` (`on` or `off`); `replicates`; `tier`.
- Outputs: paired outcomes from the episode start for the edited and original design, on common world addresses.
  Evidence: *study*.
- Chapter refusals: `FROB-REF-529` (the edited model has no complete settled-pose cache for the task: fewer than
  64 of its spawn draws came to rest when the re-run settled it, so no paired reset exists; the reset cache's
  refusal, N24a-16, which every verb that starts episodes can return; the failed draws and their reasons are
  returned and no episode is stepped); `FROB-REF-562` (the recorded run is below determinism class D2, so the
  arms cannot be paired exactly); `FROB-REF-587` (a design-parameter re-run with a policy that cannot act on the
  edited body without retraining); `FROB-REF-840` (the run is a sealed confirmation run); and the chapter codes
  of the row `design.edit` (the list of VERBS-design: `FROB-REF-101`, 102, 103, 105, 106, 107, 108, 111, 114,
  121, 123, 125 and 126), entered explicitly, because the verb applies a typed edit before it runs. A code that
  joins `design.edit`'s list later joins this row in the same change: from M3 the code of the ledger slug
  `sized-fill-unsatisfiable` (a `resolve-sized` edit whose link no stock wall carries; N31b-41, row 1 of
  N31b-45), under the number the allocation ledger holds for it; this bead writes no number.
  `FROB-REF-581` is not in this row: a re-run always starts at the episode start, so it cannot be asked for
  a mid-episode tick; 581 is what `run.fork` and `autopsy.intervene` return for such a request, and its first
  repair names this verb.
- Implemented by: BD-283.

**`autopsy.show`** (MCP tool `autopsy_show`; owner `frob-autopsy`)
- Inputs: session, fork or run handle; token budget (default 600); `format` (`summary`, the default; `full`, the
  transcript paginated by entries; `script`, the canonical commands that re-execute the session in a caller's
  namespace; `ghost`, the counterfactual-ghost feed of forks paired with their parent on the fork tick, for the
  viewer, with `max_forks` (default 16) and `include_forced` (show `diagnostic-forced` children), N27b-17;
  `crosscheck`, the view of the single-precision lane's agreement ledger `frob.autopsy.crosscheck/1`, N27b-26);
  `compare` (a second session: entry-by-entry comparison, exit 0 when identical and 2 when not); cursor
  (paginated).
- Outputs: the transcript of the attribution session with handles for drill-down. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-840` (the session's run is a sealed confirmation run); none of the Autopsy chapter's
  own.
- Implemented by: BD-284. Extended by: N27b-17 (`format = ghost`, `max_forks`, `include_forced`); N27b-26
  (`format = crosscheck`).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `autopsy.attribute` | `autopsy:read` | `frob.autopsy.attribution/1` | `diagnose` | `frob autopsy attribute f:a41c --json` |
| `autopsy.intervene` | `autopsy:run` | `frob.autopsy.intervene.out/1` | `diagnose` | `frob autopsy intervene f:a41c --do 'scene./floor/patch[galley-2]/film := dry' --placebo 64 --replicates 8 --coalitions pairs --key aut-1 --json` |
| `autopsy.repair` | `autopsy:run` | `frob.autopsy.repair/1` | `repair` | `frob autopsy repair f:a41c --key rep-1 --json` |
| `autopsy.rerun` | `autopsy:run` | `frob.autopsy.rerun/1` | `repair` | `frob autopsy rerun r:19e2 --edit 'set-param foot.radius 0.030 m' --worlds failing,sample:64 --key rr-1 --json` |
| `autopsy.show` | `autopsy:read` | `frob.autopsy.transcript/1` | `diagnose`, `repair` | `frob autopsy show f:a41c --budget 600 --json` |

Output schema names. Two are fixed by the Autopsy specification: the attribution object is
`frob.autopsy.attribution/1` and the recorded session is `frob.autopsy.transcript/1`, which is what
`autopsy.show` returns. DESIGN for the other three: `frob.autopsy.intervene.out/1` by the rule
`frob.<verb name>.out/1`; `frob.autopsy.repair/1` and `frob.autopsy.rerun/1` because the result objects are
defined under those names by the beads that build them (N27b-14 and BD-283) and are stored and cited as objects
of their own, like the attribution; a second name with `.out` for the same object would give one schema two
identifiers. N27b-16 validates the worked artifacts against exactly these five names.

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column; `paginated = true` on `autopsy.show` (its `full` form pages
by entries). The CLI spells the idempotency key `--key` for every `run` verb (the generated common flag).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "autopsy.intervene"
cluster = "autopsy"
kind = "run"
tier = "S"
capabilities = ["autopsy:run"]
input_schema = "frob.autopsy.intervene.in/1"
output_schema = "frob.autopsy.intervene.out/1"
refusals = ["FROB-REF-553", "FROB-REF-562", "FROB-REF-580", "FROB-REF-581", "FROB-REF-582", "FROB-REF-583", "FROB-REF-584", "FROB-REF-586", "FROB-REF-840"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval; the cost scales as forks × horizon in world-seconds (689 forks for 12 factors with pairs, before the second series)"
first_milestone = "M3"
task_clusters = ["diagnose"]
examples = ["frob autopsy intervene f:a41c --do 'scene./floor/patch[galley-2]/film := dry' --placebo 64 --replicates 8 --coalitions pairs --key aut-1 --json"]
owner = "frob-autopsy"
purpose = "Run interventions, placebo forks, replicates and coalitions from a fork handle made by `run.fork`"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved compute
   envelope scope the job cites; its schema, its place in the scope order and its refusals are NJL-01's);
   `mutate`: `idempotency_key`, `expected_parent`). `idempotency_key` is in no JSON-schema `required` array: the
   annotation `x-frob-key` (`"required"`, or `"optional"` on a `key_optional` entry) carries the rule, and a
   missing key is refused with 805 by the admission transaction on both surfaces. The generator appends the
   surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this
   build) and 802 (schema failure, including an unknown parameter) for every verb; 701 to 704 (budget and
   admission), 761 (above the compute envelope in force; N36-20's `admit_run`), 803 (key reused with different
   content) and 805 (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. The other
   properties and codes are added by field, as the variation table of NJL-01 lists them (that table is the one
   list; its variations 10 to 16, which later beads land, are read there and not copied here): 806 and 807 for
   paginated reads; 808 (an integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53`
   property and for no entry by kind (every `run` entry has one in `sync_wait_ms`, every paginated entry in
   `limit`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790
   for a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a
   path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`). No row of this cluster has such a
   parameter (every input is a handle, an identifier or a value), so none gets 734 or 828.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose. A code whose registry row states another exit keeps its row's exit:
   `FROB-REF-840`, which every row of this cluster lists, is exit 5 (N24-REF: the sealed data lies outside the
   session's scopes).
4. Rows are entered when the cluster is first registered (M3); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Autopsy registers no fork verb and no divergence verb. A fork point is made with `run.fork`, which returns the
  class of a proposed change set (one of five fork classes), the earliest valid tick and the null-replay result; the
  first divergence of two runs is read with `telemetry.divergence`.
- An attribution is class Estimated and is a cause in the model at the tier it ran on. Citing it as an established
  cause, or as a cause in the physical robot, is refused (`FROB-REF-880`). That code is in no row of this cluster:
  it is raised where the claim is cited, by the report and evidence verbs that cite (the citation guard N27b-15),
  and is listed in their rows.
- What a row's `refusals` lists (DESIGN, the same statement in N27b-16): the codes the verb's own handler can
  return, as the implementing bead raises them, so that the generated refusal table and the check "listed covers
  raised" speak of what a call can answer. A code raised elsewhere about the same subject is named in these notes
  and not in the row (581 for `autopsy.rerun`; 880 for `autopsy.attribute`; 866 for the bulk form of
  `autopsy.intervene`). Reason: two drafts of these rows listed different codes for three verbs, one by subject
  and one by raising site, and a row can hold only one.
- `FROB-REF-866` (a speed experiment without its computed ceiling, or with a ceiling below its goal) concerns
  the bulk form (`--forks`, `--cluster`) but is not raised by the verb: N27b-27 registers its speed experiment as
  a record of the method-experiment register (`crates/frob-conformance/registry/experiments.toml`), whose
  admission check (N38a-07, through N36-05's `admit_experiment`, the one function that raises the code) refuses
  the record with 866 before the experiment's runs. A bulk `autopsy.intervene` call carries no ceiling and is
  never refused with 866.
- Every verb of the cluster can expose a world of the run it reads, so every row lists the sealed-read refusal of
  the blinded lane, `FROB-REF-840` (a session without `qual:admin` asks for sealed confirmation data before
  close), which the M3 verb sweep (BD-312) requires of every such verb.
- The task cluster `diagnose` is the plan's: `explain_run`, `autopsy_intervene`, `autopsy_attribute`,
  `autopsy_show`, `telemetry_query`, `telemetry_inspect_pixel`, `telemetry_energy`. `autopsy.repair` and
  `autopsy.rerun` are not in it; they sit in the further task cluster `repair` ("turn a diagnosis into a tested
  edit"), which NJL-05 declares: `autopsy_repair`, `autopsy_rerun`, `autopsy_show`, `run_fork`,
  `telemetry_divergence`, `design_edit`, `result_revalidate`.
- Nothing is attributed unless the empty intervention replays its parent bit for bit (INV-AUTOPSY-01; refusal
  `FROB-REF-583`), and no factor is named when the placebo flip rate exceeds 0.2.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly five entries with `cluster = "autopsy"` (2 `read`, 3 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster autopsy` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 5, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_autopsy.sh` runs each verb of the cluster (five), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/autopsy/<verb>.help.json` byte for byte for each of
      the five, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The generated input schemas carry every input of the blocks above with its default: on `autopsy.intervene`
      `coalitions` (default `pairs` under `test = fixed`), `lane` (default `auto`), `shapley`, `shapley_budget`
      (512), `reuse`, `forks`, `cluster`, `continue_after_variant` (false); on `autopsy.attribute` `ledger`,
      `lane` (default `auto`) and `shapley` with `force`; on `autopsy.repair` `ledger`; on `autopsy.show` the
      formats `ghost` and `crosscheck` with `max_forks` (16) and `include_forced`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_autopsy` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 5 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `autopsy.show` is `read` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`autopsy.attribute` returns `frob.autopsy.attribution/1`; `autopsy.show` returns
  `frob.autopsy.transcript/1`).
- `refusal_lists`: each row's chapter codes equal its block (`autopsy.attribute`: 553, 562, 583, 584, 840;
  `autopsy.intervene`: 553, 562, 580, 581, 582, 583, 584, 586, 840; `autopsy.repair`: 562, 583, 584, 586, 840;
  `autopsy.rerun`: 529, 562, 587, 840 and the thirteen `design.edit` codes 101, 102, 103, 105, 106, 107, 108,
  111, 114, 121, 123, 125, 126, and from the change that enters it in `design.edit`'s row the code of
  `sized-fill-unsatisfiable`, which the test reads from the allocation ledger and requires on both rows or on
  neither; `autopsy.show`: 840); no row lists 880; the appended
  codes equal the rule for its kind and fields (NJL-01): {720, 723, 724, 802, 808} for the two `read` rows (808
  because each has an `int53` input, the token budget), with 806 and 807 for `autopsy.show` (paginated); {701,
  702, 703, 704, 720, 723, 724, 761, 802, 803, 805, 808} for the three `run` rows (808 through `sync_wait_ms`;
  what the variations landing at M4 add to every `run` entry is asserted by their own beads); no row has 734 or
  828.
- `mcp_names_one_to_one`: the tool names are the mapped names (`autopsy_attribute`, `autopsy_intervene`,
  `autopsy_repair`, `autopsy_rerun`, `autopsy_show`), with no collision against any other cluster.
- `intervene_inputs`: the generated `autopsy.intervene` input schema has `do` optional and the properties `test`
  (`fixed` | `sequential`, default `fixed`), `max_pairs` (default 64), `allow_unreachable`, `coalitions`
  (`pairs` | `none` | `all` | `sampled`, default `pairs`; under `test = sequential` a value other than `none` is
  802 pointing at `coalitions`), `shapley` (`auto` | `never` | `force`, default `auto`), `shapley_budget` (default
  512), `lane` (`auto` | `f64` | `tier-f32`, default `auto`), `reuse` (an array, empty by default), `forks`,
  `cluster`, `continue_after_variant` (default false); a fork argument `fixture:host/doubling-map-40:7` is
  accepted only in a build with the `conformance` feature and is 802 otherwise.
- `attribute_repair_show_inputs`: `autopsy.attribute` has `ledger`, `lane` (default `auto`) and `shapley`
  (`auto` | `never` | `force`); `autopsy.repair` has `ledger`; `autopsy.show` has `format` (`summary` | `full` |
  `script` | `ghost` | `crosscheck`, default `summary`), `max_forks` (default 16), `include_forced`, and
  `paginated = true`; the attribution output schema's label enumeration is exactly N27b-06's ten spellings and has
  no `necessary`.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool;
  `diagnose` is exactly the seven tools named in the notes; each example parses against the generated CLI and its
  command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties (`sync_wait_ms`, `idempotency_key`, `budget`,
  `estimate_only`, `on_cancel`, `envelope_scope`), with `idempotency_key` outside the schema's `required` array
  and annotated `x-frob-key: "required"`, and refuses `sync_wait_ms = 30001` with code 802; a
  `read` row has no `idempotency_key` property, so a key passed to `autopsy.attribute` or `autopsy.show` is an
  unknown parameter (802); a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_autopsy.sh` (header: `bead=VERBS-autopsy milestone=M3 labels=tether,verbs,verb:autopsy.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance autopsy`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the five verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
autopsy` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster autopsy`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_autopsy/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 5
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 5 verbs (2 `read`, 3 `run`); the tables above. KA-755 (the exit-code table on
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

- Attribution needs determinism class D2 or better (`FROB-REF-562`); on a placement that attains only D3 the
  cluster's `run` verbs refuse and name the placement that would do.
- Closing rule: the bead closes at M3, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-312) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  DESIGN: the rows list `autopsy:run` and `autopsy:read` only. A second draft of these rows also listed `run:read`
  (read access to the parent run) on the `run` verbs; it is not listed, because access to the run is decided by
  the token's namespace and by the sealed-read check, and a row that listed another cluster's scope would be the
  only one of its kind in the registry. If BD-258 rules that the scope of the object read must be listed, the
  five rows gain `run:read` together.
- If KILL-AUTOPSY fires, `--coalitions` disappears from the input schema of `autopsy.intervene`, which is a new
  schema version (`frob.autopsy.intervene.in/2`), not a silent change; the registry example loses the flag in the
  same change.
- `--lane` on `autopsy.intervene` and `autopsy.attribute` takes `auto | f64 | tier-f32` with `auto` the default
  (BD-279, BD-280). Until the single-precision lane lands (N27b-24) only `tier-f32` is refused with
  `FROB-REF-724` (not implemented in this build), which the generator appends to every row; `auto` is then the
  double-precision procedure, with `f64` its explicit form. If KILL-ENSEMBLE fires, `tier-f32` leaves the schema
  (a new schema version) and `auto` stays the default (N27b-24).
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §27.5, §27.6, §27.7, §27.8, §27.12, §27.14,
§32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755; FROB-REF-529, -553, -562,
-580 to -584, -586, -587, -840, -880, -761; OPEN-AUTOPSY-2 (N27b-13); the inputs' sources BD-279, BD-280, BD-281,
N27b-03, N27b-06 (label spellings), N27b-14, N27b-17, N27b-24, N27b-25, N27b-26, N27b-27, N27bp2-01; NJL-01
(appended codes).
