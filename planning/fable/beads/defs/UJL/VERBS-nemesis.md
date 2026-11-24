---
key: "VERBS-nemesis"
title: "frob-verbs registry: cluster `nemesis`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "nemesis", "frob-nemesis", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-26"
deps: ["BD-005", "BD-022", "BD-137", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: ["BD-309", "N25-REF", "N25e-07", "N26-REF", "N28-REF", "N28c-02", "N31a-50"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§26.4", "§26.5", "§26.7", "§26.8", "§26.9", "§26.10", "§26.11", "§26.12", "§32.4", "§32.6", "§32.12", "§32.12.3", "§32.18", "§32.21 (GATE-TETHER-02)", "§0.2"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `nemesis`. The cluster covers adversarial worlds and rare
failures: search inside a declared legality profile, the solvability oracle, shrinking a failure to a minimal
counterexample world, a bound on a failure probability, the audit of a screening stage, covering scene sets and the
hard-world set. It has seven verbs (2 `read`, 5 `run`); the handlers live in `frob-nemesis`. The bead carries every
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

In: the seven registry entries of cluster `nemesis`, each with its schemas, refusal list, capability scopes, cost
model, wait defaults, task clusters and example; the schema files of the seven verbs; the MCP tool names; the
cluster's conformance script; the help-schema snapshots; each verb's answer before its milestone; the generated
agent documentation of the seven. Out: behaviour (the handlers of the implementing beads named below); the
meanings and repairs of the refusal codes (N26-REF); the entry schema and lints (NJL-01); the conformance harness
(NJL-02); the alias lint (NJL-04); the task-cluster tables (NJL-05, NJL-06); generation of the surfaces and the
envelope (the Tether seed beads).

This is the one bead that enters the `nemesis.*` rows. A second draft of the same rows, written by the Nemesis
chapter's unit (N26b-13), is merged into it and superseded; where the two differed, the settled value is marked
below.

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `nemesis.audit` | run | M4 | S | Audit a screening stage with stratified discards and a planted winner; the one screening audit, applied by the cascade of §28.12 | §26.10 |
| `nemesis.bound` | run | M3 | S (the splitting method is F) | Certify, refute or estimate a failure probability against a target | §26.9 |
| `nemesis.cover` | read | M3 | S | Select a covering scene set and return it with its certificate | §26.11 |
| `nemesis.hardset` | read, run | M3 (action `carry` M4) | S | Show, add to, retire from and run the hard-world set of a class; carry it to another robot class or task version | §26.8 |
| `nemesis.oracle` | run | M3 | S | Classify one world for one candidate: a witness, or the failed necessary condition | §26.5 |
| `nemesis.search` | run | M3 | F | Adversarial worlds inside a declared legality profile, with the baseline arm's count at equal budget | §26.4 |
| `nemesis.shrink` | run | M3 | F | Shrink a promoted failure to a minimal counterexample world and write the `.fmcw` | §26.7 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

**`nemesis.audit`** (MCP tool `nemesis_audit`; owner `frob-nemesis`)
- Inputs: the default action `run` (no `--action`) or `--action prepare`; the campaign (positional or `--campaign`, the
  same argument; both with different values is 802); `--stage <n>`. `prepare`: `--share` (default 1/10),
  `--n-audit`, `--delta` (default 0.05), `--tolerance` (default 0.05), `--top-k`, `--plant <design> --poorly
  <kind>`. `run`: `--reveal <file>`, budget; the offline form `--stage-record <file> --outcomes <file>` audits a
  self-contained stage record (`frob.nemesis.stage/1`) and is marked `origin: synthetic` unless it resolves to a
  real campaign.
- Outputs: `run`: the audit record `frob.nemesis.audit/1` (strata with size, audited count, false discards, rate,
  upper bound and verdict; merges; the stage-level bound; the plant's outcome; the label; the bypass list; the
  calibration artifact). `prepare`: the commitment hash written into the campaign manifest (a repeated `prepare`
  with the same key returns the same commitment); a stage that already has a score record returns `status:
  "not-run"`, reason `scores-exist-before-commitment`, exit 2. Evidence: *study* (class Tested at confidence
  1 − δ, scoped to the audited stage's tier, population and generator identity).
- Chapter refusals: `FROB-REF-546` (calibration from survivors, or an audit episode cancelled or re-weighted;
  raised when a design outside the committed draw is offered as audited); `FROB-REF-566` (a planted winner's
  rank is not on record from a full evaluation; raised by `prepare`).
- Scope: `nemesis:run`; revealing the planted winner (`--reveal`) also needs the campaign administration scope.
- Implemented by: BD-345, N26b-05 (the two phases and the offline form), N28b-29.

**`nemesis.bound`** (MCP tool `nemesis_bound`; owner `frob-nemesis`)
- Inputs: candidate; `--task` (the failure predicate is the task's requirement); `--tier` (required, no default:
  the scope is part of the claim); `--target <p⋆>`; `--delta` (default 1/20); `--mode` (`certify`, `refute`,
  `estimate`); budget; optional `--proposal <id>` or `--shift … --epsilon …` (default ε = 1/10); optional
  `--episodes <run set>` (offered episodes, admitted only if drawn nominally or with recorded bounded weights);
  optional `--finalist` (a flag: the caller's attestation, itself recorded, that a candidate not drawn from a
  recorded campaign's finalists is a finalist; N26a-26's finalists-only rule admits a candidate that is a
  finalist of a recorded campaign, or a request that carries this flag).
- Outputs: a bound on the failure probability against the target, with the method and effective sample size, or the
  bound reachable within budget; a `fallback` block when the decision policy fell back to nominal sampling.
  Evidence: *study*.
- Chapter refusals: `FROB-REF-549` (a rare-failure bound over battery-selected or search-selected episodes);
  `FROB-REF-560` (a bound was offered episodes that were not drawn nominally or with recorded bounded weights);
  `FROB-REF-563` (importance sampling outside its admitted law, or with a proposal fitted on the run it is applied
  to); `FROB-REF-568` (the failure predicate changed after sampling began); `FROB-REF-569` (splitting requested
  where no randomness enters after the fork); `FROB-REF-760` (the episodes a claim needs at its level exceed the
  budget; the bound reachable within budget is attached); `FROB-REF-840` (a world address in the confirmation
  namespace without the qualification administration scope). Of the two codes for selected episodes, 560 is the
  one the bound's own admission rule raises and 549 is the co-design cascade's code for the same rule one layer
  up; both are on the row because a call made by the cascade surfaces 549. 569 on this row is the decision
  policy's fallback trigger (b) (BD-276): when the policy's exploration step asks the splitting stage (BD-277) for
  splitting on a task with no per-step randomness after the fork, the bound completes by nominal sampling and the
  code is recorded in the result's `fallback` block; it is not returned as the verb's exit. It is listed because
  it appears in the verb's envelope, and the lint requires every code a verb emits on its row. Three refusals
  that the plan states without a number are on the row under the numbers the allocation ledger assigns their
  slugs (N26-REF's table; `registries/refusal_allocations.toml`, field `assigned`, filled by N35x-02's run at
  M0, before the row is entered; this bead prints none): `rare-bound-not-finalist` (family `5xx`, exit 3: the
  candidate is not a finalist of a recorded campaign and the request carries no `--finalist`, for example a
  request made from inside a screening rung; N26a-26); `certify-by-importance-sampling` (family `5xx`, exit 3:
  `--mode certify` with `--proposal`, `--shift` or `--epsilon`, refused at admission with zero episodes drawn;
  repairs: `--mode refute` or `--mode estimate`, or drop the proposal and certify by nominal sampling; BD-276);
  `importance-over-sibling-cap` (sub-family `76x`, exit 4: a bounded-weight run planned above the 10⁶
  evaluations of the pinned sibling's importance execution, refused at admission with the bound reachable at
  the cap attached; BD-276).
- Implemented by: BD-276 (the bound and its decision policy), N26a-26 (the decision policy's admission: the
  finalists-only rule and `--finalist`), N26a-23 (the lane firewall that raises 560, 563
  and 568), BD-277 (the splitting exploration step, which records 569).

**`nemesis.cover`** (MCP tool `nemesis_cover`; owner `frob-nemesis`)
- Inputs: robot class (`--class`) and task family (`--task-family`), which fix the catalogues read; optional
  reference set of designs (`--reference`); a size, either `--slots K` (cardinality mode) or `--budget-seconds T`
  (cost mode); optional `--incidence <inc:… | file>` (a stored or supplied incidence of hazards; with it the
  class and the task family are read from the incidence's own header and `--class` and `--task-family` may be
  omitted, and given together with it they must equal the header's, a difference being `FROB-REF-802` naming the
  option; without `--incidence` both are required; BD-278); `--prefixes`
  (every prefix's certificate); `--algorithm lazy|plain` (default `lazy`); `--greedy-only` (K greedy slots and no
  held-out slot, marked `no-held-out-slot`, not adoptable as the `covering` battery); `--require-rule table` (pins
  the decision core DC-10's rule, so that a missing or stale core table is refused instead of completing on the
  core's fallback; N28c-02's surface rule for a governed verb, called by canary audits and replay scenarios).
- Outputs: a covering scene set with its coverage certificate. It changes nothing: adoption into the `covering`
  battery is a membership event of the battery mechanism that cites the selection and its certificate. Exit 0 for
  a selection with an accepted certificate; exit 2 for the fallback or a flag (`held-out-pool-short`,
  `certificate-check-failed`, fewer scenes than slots). Evidence: *static*.
- Chapter refusals: `FROB-REF-565` (coverage selection on a stale incidence); `FROB-REF-871` (from M3: with
  `--require-rule table`, the DC-10 core table is missing, stale against its contract or not the registry version,
  exit 3; on a normal call the verb completes on the core's fallback and carries 871 in `warnings` and in its
  decision record; Atelier's code, whose N28-REF row names this verb through DC-10).
- Implemented by: BD-278. Governed by: DC-10 (BD-309, policy `nemesis.dc10.cover`); the core-table check and
  `--require-rule` are N28c-02's.

**`nemesis.hardset`** (MCP tool `nemesis_hardset`; owner `frob-nemesis`)
- Inputs: class (`--class`) with task (`--task <t:…@version>`), or set (`--set hs:…`): one set exists per task
  family, task version and robot class, so `--class` with `--task` selects the current set of that class and
  task version, the default call with `--class <class>` alone lists the class's sets (identity, task, version,
  world count), and a run action whose set is not determined is `FROB-REF-802` naming the missing `--task`
  (BD-275); the action is selected only with `--action <name>` (NJL-01 action rule 1): the default, `show`
  (read; paginated), is the call with no `--action` and has no name to pass; the run actions are `--action add
  <.fmcw>`, `--action retire <world> --reason … --decision …`, `--action run <design>`, `--action carry --from
  <hs:…> --to-class <C′> | --to-task <t:…@v′> --candidate <design> [--worlds w:…,…]` and `--action carry
  --refresh` (the re-promotion of a set's members in place after a kernel-version change). An action word in a
  positional place (`frob nemesis hardset add …`) or `--action show` is `FROB-REF-802`.
- Outputs: `frob.hardset/1`: the hard-world set of the class (identity, counts by failure class, each world with
  its failure mode, witness class and candidate, the retired list, the version lineage); for `run`, a scorecard
  with the standard outcome columns and two denominators, verdicts labelled as regression results; for `carry`,
  per source world one of `carried` (new world identity; same or different mechanism), `does-not-fail`,
  `illegal-under-target-profile`, `infeasible-for-candidate`, `not-excluded`, `artifact-candidate`, with the
  carried entries recording `carried_from` and the source set untouched. Evidence: *run*.
- Chapter refusals: `FROB-REF-567` (a hard-world or search-selected world offered as a confirmation sample; the
  qualification lane's own verb raises the same code when such a world is offered to it); `FROB-REF-560`
  (episodes of the set offered to a bound: they were not drawn nominally; one of the set's refused uses, BD-275).
  And, under the number the allocation ledger assigns its slug (N26-REF's table; N35x-02's run at M0; this bead
  prints none): `hard-world-set-inadmissible` (family `3xx`, exit 3: an `add` whose admission check fails; the
  refusal carries its pointer and names the failed check; BD-275).
- Scopes by action: `show` needs `nemesis:read`; `run` needs `nemesis:run`; `add`, `retire` and `carry` need
  `nemesis:write` (`carry` ends in the target set's `add`).
- Implemented by: BD-275. Extended by: N26b-23 (action `carry`, first milestone M4; before M4 a `carry` request
  answers `FROB-REF-724`, exit 5; progress denominator: source worlds).

**`nemesis.oracle`** (MCP tool `nemesis_oracle`; owner `frob-nemesis`)
- Inputs: candidate; `--task`; `--world <world.json | w:… | run id>`; `--legality <profile>`; optional `--tier`,
  `--class-level` (the question for every body of the class), `--require-cited` (the demand BD-270 defines: a
  touched bound of provenance `assumed` is refused with `FROB-REF-361`), `--step-plan <plan>` (the step plan of
  the witness runs on the request's tier, default the tier's production plan; a plan carrying a conformance-only
  option of N16b-15 is admitted only by a `conformance` build for a session holding the conformance scope, as
  that bead's guard states), budget (`--budget steps=N`). (BD-271.)
- Outputs: class `solvable-causal` or `plant-feasible` with a witness controller and margin, or the failed necessary
  condition. Evidence: *run*.
- Chapter refusals: `FROB-REF-360` (a world or edit lies outside the legality profile; the violated bound is named);
  `FROB-REF-361` (the request demands `cited` bounds and the profile has an `assumed` bound on a touched
  quantity); `FROB-REF-362` (an edit does not declare the causal paths it changes); `FROB-REF-840` (confirmation
  namespace).
- Implemented by: BD-271.

**`nemesis.search`** (MCP tool `nemesis_search`; owner `frob-nemesis`)
- Inputs: candidate; `--task`; `--legality <profile>`; `--method` (`auto` the default, `card-uniform`,
  `cross-entropy`, `descent`, `grammar`, `splitting`, `given`, `exploration`; `exploration` is DESIGN of BD-273:
  the exploration lane as one search arm, splitting and cross-entropy only, half of the search arm each, on the
  one shared failure archive with the restart rule, the arm GATE-NEMESIS-05 measures, N26a-31; it needs what
  `splitting` needs); budget; optional `--first` (each arm stops at its
  own first promoted failure); `--worlds <file>` (the newline-delimited file of world documents the `given` arm
  reads: required with `--method given` and refused with any other method, `FROB-REF-802`); `--require-cited`
  (the demand BD-270 defines: a world touching a bound of provenance `assumed` is refused with `FROB-REF-361`, a
  `given` document individually; the methods' own proposals are confined to `cited` or `card` bounds, and a
  request that leaves no searchable quantity is 361 before any episode); `--seed <u64str>` (the campaign seed,
  default 0, echoed in the result); `--tier <tier>`; `--mission <query>` (the mission-bound form the written
  mission session of §32.16 calls, `--mission q:…`); `--step-plan <plan>` (the step plan of the campaign's own
  evaluations, default the tier's production plan; a plan carrying a conformance-only option of N16b-15,
  `penetration-recovery` or `pyramid4` as base cone, is admitted only by a `conformance` build for a session
  holding the conformance scope, its label printed in the result, and the artifact guard's replays always run
  under the production plan); `--variants <set>` (a set of model variants of the candidate, such as the aged
  variants `foundry.lifecycle` returns, each variant one model slot and worlds paired by address; added by
  N31a-50 at M6, before which the input answers `FROB-REF-724`); `--fill licensed` (from M3, N25e-07: default
  off; with it licensed monotone fills between evaluated points are attempted and each fill appears as a `proved`
  point with its proof reference; without it no verdict is interpolated). (BD-273, which fixes the spellings.)
- Outputs: worlds searched, classified candidate failures, the worst world, the baseline arm's failure count at
  equal budget, the artifact guard's result. The one-arm campaigns (BD-273): under `--method card-uniform` the
  baseline receives the whole budget and the result reports `baseline_arm` and `steps.baseline` only; under
  `--method given` the caller's worlds are evaluated in file order, nothing else is drawn, the result reports
  `baseline_arm: null` with the reason `given-worlds-only`, and it carries `refused`: one entry per document
  turned away, `{line, code, pointer, bound, observed, required}` with code 360, 361 or 362 (the verb still
  completes; a refused document is not a refusal of the request). "None found" is a search result and not a
  certificate. The searched grid is reported with point statuses (N25e-07): a failing world found at one parameter
  value says nothing about its neighbours. Evidence: *run*.
- Chapter refusals: `FROB-REF-360` (a world or edit lies outside the legality profile; the violated bound is named);
  `FROB-REF-361` (the request demands `cited` bounds and the profile has an `assumed` bound on a touched quantity);
  `FROB-REF-362` (an edit does not declare the causal paths it changes); `FROB-REF-569` (method `splitting` or
  `exploration` requested where no randomness enters after the fork, refused before any episode); `FROB-REF-840`
  (confirmation namespace); `FROB-REF-852`
  (from M3, N25e-07: `--fill licensed` on a sweep with no monotonicity licence, such as a dynamic rollout, whose
  verdicts have no monotonicity theorem by default; repairs ranked: evaluate the missing values, supply the
  monotonicity proof and register a licence, accept `unresolved`; Augur's code, N25-REF). `--method
  splitting` or `--method exploration` in a build without the splitting explorer (before BD-277, or after
  KILL-RARE removes it) is the appended `FROB-REF-724`, exit 5, zero episodes, naming the splitting explorer
  or KILL-RARE.
- Implemented by: BD-273 (`exploration` among them). Extended by: BD-277 (the splitting method, which raises
  569, and the completion of the `exploration` arm: it registers the explorer and asserts the half-and-half
  split on one archive); N31a-50 (`--variants`, M6); N25e-07 (`--fill licensed` and 852, M3).

**`nemesis.shrink`** (MCP tool `nemesis_shrink`; owner `frob-nemesis`)
- Inputs: a promoted failing world (`w:…` or a run's failure); optional `--out <dir>`, `--max-tests` (default
  400), `--no-pair-check`, `--tier`; budget.
- Outputs: a `.fmcw` (`frob.mcw/1`): the minimal edits, minimality kind, tests run, seeds failing, passing
  neighbours, the oracle's class. Evidence: *run*.
- Chapter refusals: `FROB-REF-360` (a world or edit lies outside the legality profile; the violated bound is named);
  `FROB-REF-564` (shrinking requested for a failure that does not replay, or that the guard labelled an artifact
  candidate); `FROB-REF-840` (confirmation namespace); and, for `--out`, the export door's codes: `FROB-REF-816`
  (the no-authorization flags absent from the `.fmcw`'s manifest), `FROB-REF-731` (the privacy mark of a scene the
  world derives from, against scope and flag), `FROB-REF-733` (its licence set against the export kind),
  `FROB-REF-734` (an `--out` path outside the session's export root).
- Writing outside the store (DESIGN, decided against N37-18's rule). Without `--out` the `.fmcw` is
  stored as an artifact of the caller's namespace and nothing leaves the store. With `--out <dir>` the file is
  written outside the store, and N37-18 admits no exception for a writer outside the store: the row carries
  `exports = true`, the write takes an `ExportGrant` from `frob_verbs::export::admit` under export kind `run` (a
  derived view of a run's failing world), and the option's publish scope is `option_capabilities = { out =
  ["export:report"] }`, so a call without `--out` needs no publish scope. Reason: a minimal counterexample world
  is derived from its scene and would otherwise leave the machine without the scene's privacy mark or licence.
- Implemented by: BD-274 (its `--out` routed through N37-18's export door).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `nemesis.audit` | `nemesis:run` | `frob.nemesis.audit.out/1` | `ladder` | `frob nemesis audit camp:12 --stage 3 --key aud-1 --json` |
| `nemesis.bound` | `nemesis:run` | `frob.nemesis.bound.out/1` | `attack` | `frob nemesis bound quad-tray --task t:kitchen-tray --tier batch-cpu --target 1e-3 --mode certify --key nb-1 --json` |
| `nemesis.cover` | `nemesis:read` | `frob.nemesis.cover.out/1` | `batteries` | `frob nemesis cover --class quad-mammal --task-family payload-transport --slots 20 --json` |
| `nemesis.hardset` | `nemesis:read` | `frob.hardset/1` | `attack`, `batteries` | `frob nemesis hardset --class kitchen-commercial --json` |
| `nemesis.oracle` | `nemesis:run` | `frob.nemesis.oracle.out/1` | `attack` | `frob nemesis oracle quad-tray --task t:kitchen-tray --world w:3fa9 --legality kitchen-commercial/default --key or-1 --json` |
| `nemesis.search` | `nemesis:run` | `frob.nemesis.search.out/1` | `attack` | `frob nemesis search quad-tray --task t:kitchen-tray --legality kitchen-commercial/default --budget gpu_s=240 --key nem-1 --json` |
| `nemesis.shrink` | `nemesis:run` | `frob.nemesis.shrink.out/1` | `attack` | `frob nemesis shrink w:9d04 --key shr-1 --json` |

Cost model of each row (the text of its `cost_model` field; for a `run` verb it is what `--estimate` prices):

| Verb | Cost model |
|---|---|
| `nemesis.audit` | a tenth of the audited stage's spend |
| `nemesis.bound` | n nominal episodes; 29,956 for a target of 10⁻⁴ at δ = 0.05 |
| `nemesis.cover` | static: under 1 s for 10⁴ scenes and 300 modes; no episodes |
| `nemesis.hardset` | `show`: static; `run`: one episode per world of the set; `carry`: per source world 64 episodes, six guard episodes, a witness search of up to 16 failing-episode equivalents and a shrink |
| `nemesis.oracle` | up to 16 failing-episode equivalents plus 6 guard episodes per witness |
| `nemesis.search` | budgeted; half of the budget goes to the baseline arm |
| `nemesis.shrink` | at most `max-tests` × 64 episodes |

Settled differences between the two drafts of these rows (DESIGN where the plan is silent):
- *Output schema names.* `frob.<verb name>.out/1`, the form of the registry's entry example (the other draft had
  `frob.nemesis.cover/1` and the like, without `.out`). `nemesis.hardset` returns the artifact schema
  `frob.hardset/1` with a scorecard for `run`.
- *Task cluster of `nemesis_cover`.* The plan's cluster table assigns it nowhere, and `attack` and `ladder` are
  full at seven tools. The other draft proposed the cluster `audit` pending a ruling by the owner of the cluster
  table; that owner (NJL-05) has added the cluster `batteries` (`nemesis_cover`, `nemesis_hardset`, `eval_run`,
  `eval_scorecard`, `eval_items`, `task_list`, `scene_list`), so the row says `batteries` and the proposal is
  withdrawn.
- *Refusal lists.* Each list above is the union of the two drafts, which had each omitted codes the other
  carried (840 on the four verbs that take world addresses, 569 on `search`, 361 and 362 on `oracle`, 560 on
  `hardset`; 546 on `audit` and 549 on `bound`). The twelve codes the Nemesis chapter defines (360, 361, 362,
  560, 563, 564, 565, 566, 567, 568, 569, 760) each appear on at least one row.

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. A verb printed with two kinds has `kind` equal to its default
action and an `actions` list for the others, each with its own kind and scope: `nemesis.hardset`: `add` (run,
scope `nemesis:write`), `retire` (run, `nemesis:write`), `run` (run, `nemesis:run`), `carry` (run,
`nemesis:write`, first milestone M4, owner N26b-23). One kind with a second action is listed the same way:
`nemesis.audit`: `run` is the default and `prepare` (run, `nemesis:run`, M4, owner N26b-05) is in `actions`.
`nemesis.hardset` has `paginated = true` for `show` (the generator appends 806 and 807).

Wait and progress (run verbs). Wait to completion on the command line; 5,000 ms by default and at most 30,000 ms
on MCP, after which the envelope carries a job handle (exit 8) with progress that always has a denominator:
worlds for `search`, witness runs for `oracle`, tests for `shrink`, episodes for `bound`, lane members for
`audit`, worlds for `hardset run`, source worlds for `hardset carry`.

Schema files. One input and one output schema file per verb under `schemas/`, generated from the handler's
request and result types and referenced by name in the row; `frob schema nemesis.cover` prints them. The output
schemas are those the implementing beads define; this bead creates the files with the fields those beads list
and a `"$comment"` naming the owning bead, and the generator check keeps them in step.

Agent documentation. The generated `robot-docs` entry of each verb states its one-line purpose from the rows
table, its lane (discovery data: nothing these verbs return is a confirmation sample), its refusal list with
first repairs, and one worked example.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "nemesis.audit"
cluster = "nemesis"
kind = "run"
tier = "S"
capabilities = ["nemesis:run"]
input_schema = "frob.nemesis.audit.in/1"
output_schema = "frob.nemesis.audit.out/1"
refusals = ["FROB-REF-546", "FROB-REF-566"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["ladder"]
examples = ["frob nemesis audit camp:12 --stage 3 --key aud-1 --json"]
owner = "frob-nemesis"
purpose = "Audit a screening stage with stratified discards and a planted winner; the one screening audit, applied by the cascade of §28.12"
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
   In this cluster the inputs that accept a path to a file are `params` entries of kind
   `artifact-input<F>` for the format the file holds, so their rows get 891, 734 and 828: `nemesis.audit`
   (`--reveal`, `--stage-record`, `--outcomes`), `nemesis.cover` (`--incidence` given as a file),
   `nemesis.hardset` (`add <.fmcw>`), `nemesis.oracle` (`--world` given as a file) and `nemesis.search`
   (`--worlds`). `nemesis.shrink`'s `--out` is an export path, whose 734 its own row lists with the export door's
   codes; `nemesis.bound` takes identifiers only.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M3); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Seven verbs. `nemesis.hardset` has two kinds: `show` is a `read`; `add`, `retire`, `run` and `carry` are `run`
  actions. Carrying is offered through re-promotion only until OPEN-NEMESIS-5 is ruled (N26b-23).
- Only solvable worlds count: no world without a `solvable-causal` or `plant-feasible` witness enters the hard-world
  set or a counterexample count (INV-NEMESIS-01). A failure and its witness count only after the artifact guard.
- "None found" is a search result and not a certificate; the envelope of an empty search says so in its result.
- `nemesis.audit` is the one screening audit; the co-design cascade applies it and registers no audit verb of its
  own.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "nemesis"` (2 `read`, 5 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster nemesis` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_nemesis.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/nemesis/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The generated input schemas carry the inputs the implementing beads fix: on `nemesis.search` `worlds`,
      `require_cited`, `seed` (`u64str`), `tier`, `mission`, `step_plan`, `fill` and, from M6, `variants`; on
      `nemesis.oracle` `require_cited` and `step_plan`; on `nemesis.hardset` `task`; on `nemesis.cover` `class`
      and `task_family` optional when `incidence` is given, and `require_rule`; on `nemesis.bound` the flag
      `finalist`; `nemesis.search`'s `method` has the eight values of its block, `exploration` among them;
      `nemesis.shrink` has `exports = true` and `option_capabilities = { out = ["export:report"] }`.
- [ ] The two-kind verb exposes `action`; an `admin` or `run` action without its scope is `FROB-REF-720`, exit 5,
      and the `read` action still answers.
- [ ] The rows' kinds and first milestones equal the rows table (`search`, `oracle`, `shrink`, `bound` run at
      M3; `audit` run at M4 with the action `prepare`; `cover` read at M3; `hardset` read and run at M3, with the
      action `carry` at M4 answering `FROB-REF-724` before M4).
- [ ] Every refusal code named in a row resolves in `registries/refusals.toml`; the twelve codes the Nemesis
      chapter defines (360, 361, 362, 560, 563, 564, 565, 566, 567, 568, 569, 760) each appear on at least one
      row.
- [ ] Each verb has an input schema file, an output schema file and at least one example that parses against the
      schema.
- [ ] `frob capabilities --json` lists the seven verbs with their kinds; the served MCP tool list equals the
      registry under the name map (`nemesis.cover` ↔ `nemesis_cover`); no task cluster exceeds seven tools, and
      `nemesis_cover` sits in `batteries`.
- [ ] In an M3 build `frob nemesis audit …` exits 5 with `FROB-REF-724` and never with an unknown-verb error.
- [ ] Every run verb's job handle reports progress with the denominator of its row.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_nemesis` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `nemesis.shrink` is `run` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`nemesis.hardset` returns `frob.hardset/1`).
- `refusal_lists`: each row's chapter codes equal its block (`nemesis.audit`: 546, 566; `nemesis.bound`: 549, 560,
  563, 568, 569, 760, 840; `nemesis.cover`: 565, 871; `nemesis.hardset`: 567, 560; `nemesis.oracle`: 360, 361, 362,
  840; `nemesis.search`: 360, 361, 362, 569, 840, 852; `nemesis.shrink`: 360, 564, 840, 816, 731, 733, 734);
  beside them the refusals that the allocation ledger numbers, each compared by reading the `assigned` field of
  its slug's row in `registries/refusal_allocations.toml`, never by a typed number: `rare-bound-not-finalist`,
  `certify-by-importance-sampling` and `importance-over-sibling-cap` on `nemesis.bound`,
  `hard-world-set-inadmissible` on `nemesis.hardset` (a planted row without one of them fails, and so does a
  slug whose ledger row has no assigned number); the
  appended codes equal the rule for its kind and fields (NJL-01): 761 on every `run` row; 806 and 807 on
  `nemesis.hardset`; 891, 734 and 828 on `nemesis.audit`, `nemesis.cover`, `nemesis.hardset`, `nemesis.oracle`
  and `nemesis.search` (their file inputs), and on no other row.
- `inputs_from_implementing_beads`: `nemesis.search` without `--worlds` under `--method given`, and with
  `--worlds` under another method, is 802; its schema has `require_cited`, `seed` (type `u64str`), `tier`,
  `mission`, `step_plan`, `variants`, `fill` (the one value `licensed`), and `method` accepts exactly `auto`,
  `card-uniform`, `cross-entropy`, `descent`, `grammar`, `splitting`, `given` and `exploration` (another value is
  802); `nemesis.bound` has the boolean `finalist`; `nemesis.oracle` has `require_cited` and
  `step_plan`; `nemesis.cover` has `require_rule` (the one value `table`); `nemesis.hardset --action
  run <d>` with `--class` and neither `--task` nor `--set` is 802 naming `task`; `nemesis.cover --incidence <file>`
  without `--class` passes the schema and with a differing `--class` is 802; `nemesis.shrink` routes `--out`
  through the export door (a planted handler writing `--out` without a grant fails the `exports = true` lint).
- `actions_enumerated`: `nemesis.hardset` lists `add`, `retire`, `run`, `carry` with their kinds, scopes and
  milestones (`carry` M4, owner N26b-23); `nemesis.audit` lists `prepare`; a planted fifth hardset action fails.
  Each action is selected only with `--action <name>`: `frob nemesis hardset add <f>.fmcw` (an action word in a
  positional place) and `frob nemesis hardset --action show` (the default has no name) are 802.
- `refusal_lists_resolve_and_cover_twelve_codes`: every code is in the refusal registry and each of the twelve
  chapter codes is on at least one row.
- `schemas_exist_and_examples_parse` (one example per verb); `run_verbs_have_wait_defaults_and_progress_units`.
- `nemesis_cover_cluster_is_batteries`; `task_clusters_at_most_seven`.
- `not_implemented_before_milestone`: with the handler table empty for `audit` in the M3 feature set the verb
  answers `FROB-REF-724`.
- `scopes_declared`: a session without `nemesis:run` calling `nemesis.oracle` gets the missing-scope refusal;
  `hardset add` and `hardset carry` without `nemesis:write` are refused and `hardset show` still answers.
- `mcp_names_one_to_one`: the tool names are the mapped names (`nemesis_audit`, `nemesis_bound`, `nemesis_cover`,
  `nemesis_hardset`, `nemesis_oracle`, `nemesis_search`, `nemesis_shrink`), with no collision against any other
  cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of convention 2 (`envelope_scope` among them) and refuses
  `sync_wait_ms = 30001` with code 802; an
  `admin` row or action without its scope is refused with code 720; a verb whose capability is absent returns code
  724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_nemesis.sh` (header: `bead=VERBS-nemesis milestone=M3 labels=tether,verbs,verb:nemesis.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field,dropped-option`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance nemesis`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
nemesis` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster nemesis`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1; (8) in an M3 build `frob nemesis hardset --action
carry --from <hs> --to-class <C> --candidate <d> --key c-1 --json` answers `FROB-REF-724`, exit 5, one envelope,
and `frob nemesis audit camp:12 --action prepare --stage 3 --key ap-1 --json` the same; (9) the schema rules of
the inputs the implementing beads fix, each a refusal before any episode: `frob nemesis search quad-tray --task
t:kitchen-tray --legality kitchen-commercial/default --method given --budget gpu_s=10 --key g-1 --json` without `--worlds`
(`e2e_expect_refusal FROB-REF-802 3`, pointer `worlds`); `frob nemesis search … --method descent --worlds
w.ndjson --key g-2 --json` (802); `frob nemesis shrink w:9d04 --out /etc --key shr-2 --json`
(`e2e_expect_refusal FROB-REF-734 5`) and, from a session without `export:report`, `--out <export root>/m`
(`e2e_expect_refusal FROB-REF-720 5`); `frob nemesis oracle quad-tray --task t:kitchen-tray --world /etc/hosts
--legality kitchen-commercial/default --key or-2 --json` (734); (10) for each verb, every option named in its block
appears in `frob <verb> --help --json` (logged as `option_present` per option with the block's bead).

The chapter's refusal script `scripts/e2e/nemesis_refusals.sh` (N26b-14) begins with the same surface check for
the seven verbs (`frob capabilities --json`, `frob schema <verb>`, the MCP `tools/list`, with an empty diff against
the registry) before it exercises the refusals; each implementing bead's own script exercises its verb.

Log: `target/e2e-logs/verbs_nemesis/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
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
  `dropped-option` (`--fill` removed from the `nemesis.search` entry: step (10) fails naming the verb, the option
  and N25e-07). A verb answering with two envelopes or exit code 1 fails step (7).
- Planted defects the unit tests must catch: a row whose kind is `read` for a verb that spends compute (the kind
  test); a refusal code on a row that is absent from the refusal registry; an eighth tool in a task cluster; a
  verb present in the handler table and absent from the registry (the parity diff). The verb table of the Nemesis
  chapter is the known answer for kinds and milestones. Regeneration of the registry's generated files is
  idempotent.
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Hard-world and search-selected worlds are discovery data; offering one as a confirmation sample is refused
  (`FROB-REF-567`).
- A certified failure rate of 10⁻⁶ needs a compute envelope above the default (OPEN-NEMESIS-3); without approval the
  verb certifies to 10⁻⁵ and reports 10⁻⁶ as Estimated.
- Closing rule: the bead closes at M3, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M3, M4. The per-milestone
  surface beads (BD-312, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Additions that the implementing beads need beyond the plan's verb notes are DESIGN of this registry, each
  written into its verb's block above: the methods `given` and `exploration` (BD-273) and the option `--first`
  on `nemesis.search`; `--episodes` and `--finalist` (N26a-26) on `nemesis.bound`; the two phases `prepare` and
  `run` on `nemesis.audit`;
  `--incidence`, `--algorithm` and `--greedy-only` on `nemesis.cover`; the action `carry` on `nemesis.hardset`
  (N26b-23); `--worlds`, `--require-cited`, `--seed`, `--tier`, `--mission` and `--step-plan` on `nemesis.search`
  (BD-273) and `--variants` (N31a-50); `--require-cited` and `--step-plan` on `nemesis.oracle` (BD-271);
  `--task` on `nemesis.hardset` (BD-275); the header-read class and family of `nemesis.cover --incidence`
  (BD-278); the export treatment of `nemesis.shrink --out` (N37-18); `--fill licensed` and 852 on
  `nemesis.search` (N25e-07, round 3); `--require-rule table` and 871 on `nemesis.cover` (N28c-02 and the DC-10
  record of BD-309, round 3, the same form as `eval.promote` in VERBS-eval).
- A change to a row after its first milestone is a schema change with a version bump of the affected schema,
  never an in-place edit. A registered verb never answers "unknown verb".
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

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §26.4, §26.5, §26.7, §26.8, §26.9, §26.10,
§26.11, §26.12, §32.4, §32.6, §32.12.3, §32.16 (the mission session's `--mission`), §32.18, §32.21; GATE-TETHER-02;
INV-TETHER-01, -02; KA-755; NJL-01 (appended codes), NJL-04 (alias lint); N37-18 (the export door); BD-270,
BD-271, BD-273, BD-275, BD-278, N16b-15, N31a-50.
