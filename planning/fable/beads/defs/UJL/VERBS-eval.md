---
key: "VERBS-eval"
title: "frob-verbs registry: cluster `eval`, ten rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "arena", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§24.8", "§24.12", "§24.13", "§24.21", "§16.8", "§22.3", "§23.16", "§28.6", "§28.10", "§28.20", "§35.3", "§32.6", "§32.12.3", "§32.16", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `eval`. The cluster covers evaluation: running a battery,
scorecards with outcome columns, the pre-analysis of a fixed-n rule, paired comparison, promotion of a fast-tier
result to the referee, the blinded qualification lane and its ledger, exploitation detectors, rank agreement between
rungs, and the item bank. It has ten verbs (4 `read`, 6 `run`); the handlers live in `frob-arena`, `frob-dyn`,
`frob-learn`, `frob-opt`. The bead carries every row completely, with the three checks that keep the rows true: the
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

In: the ten registry entries of cluster `eval`, with every input, action, output and refusal that the implementing
and extending beads define; the MCP tool names; the cluster's conformance script; the help-schema snapshots; each
verb's answer before its milestone. Out: behaviour (the implementing beads named below); the entry schema and lints
(NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters (NJL-05, NJL-06); generation of
the surfaces (the Tether seed beads); the numbers of refusals still named by slug (the allocation ledger, N35-33 and
N35x-02).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `eval.compare` | run | M3 | S | Plan and submit a paired study on common world addresses; the reachability pre-analysis is `eval.preanalyze` | §28.6 |
| `eval.detect` | run | M3 | S | The exploitation detectors, on one result | §23.16 |
| `eval.ledger` | read | M3 | S | A qualification campaign's index, levels spent and receipts; nothing sealed | §24.13 |
| `eval.preanalyze` | read | M1 | S | Reachable verdicts and power of a fixed-n rule at a given n and level | §24.12 |
| `eval.promote` | run | M3 | S | Apply the promotion rule and the tier correction to a fast-tier result | §16.8 |
| `eval.qualify` | run | M3 | S | Submit a committed request to the blinded lane; returns a receipt and nothing per world | §24.13 |
| `eval.rank-agreement` | run | M4 | F | Measure rank agreement between two rungs in a stratum and write the ledger record | §28.10 |
| `eval.run` | run | M2 | S | Run a battery and record verdicts | §24.8 |
| `eval.scorecard` | read | M3 | S | Metrics by scene class with the outcome columns and two denominators | §24.8 |
| `eval.items` | read | M3 | F | The item bank of a battery and the sampling weights a close comparison used | §24.21 |

Tiers follow the plan's capability tables: the rank-agreement ledger is `[F]` (§28, with KILL-RANK) and the item
bank is `[F]` (§24.21, with KILL-ITEMBANK); an earlier draft printed `S` for both. Four rows answer a reduced form
before their `first_milestone` (convention 4): `eval.compare` from M1 (N33b-19, the CPU referee form), `eval.run`
and `eval.scorecard` from M1 (N24c-04, the referee form), and `eval.rank-agreement` from M3 (BD-265, the tier-pair
form, and N23b-35, the fixture form).

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *fidelity*: a Fidelity claim (the faster tier tracks the more trusted tier within a frozen envelope); class
  Tested; directional.
- *qualification*: a Design comparison of class Tested at the referee tier, `qualified` only by a receipt whose
  decision is `promoted`; the receipt carries the three no-authorization flags, all false.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.

Each block lists the chapter refusals by number (the row's `refusals` field) and, separately, the pending refusals
that the plan names without a number (convention 5).

**`eval.compare`** (MCP tool `eval_compare`; owner `frob-opt`)
- Inputs:
  - The arms: two designs (positional), or more than two with `--bracket` (N25e-15: the comparison whose level
    split α/(2M) counts the M designs). Instead of designs, the cross-decimation form `--run <coarse run>
    --across decimation` (N16c-44: the run's deadline verdicts against the 2d arm that the step-refinement helper
    builds inside the comparison).
  - `task`; optional `scene`; optional `battery` (`--battery <b>`: the world sample is drawn from the battery's
    items, N24c-31, N24c-35).
  - `metric` (repeatable; default the task's primary metric); `margin`; `level` (default 0.05); `max_pairs`;
    `budget`; the idempotency key.
  - `backend` (BD-262): one placement for every arm (`referee` at M1; for example `batch-cpu` or
    `strict:gpu-cuda` from M3), or per arm, `--backend <i>=<placement>` repeated, with `i` the arm's position from
    1 (DESIGN: N35-08's per-arm placement under the one input that names a placement; an arm not named takes the
    default).
  - `controller_arm` (`shared` by default, `retuned-equal-resources` with the resources per arm, or
    `inherits-weights`; the field of `study.plan`, N33b-19).
  - At most one study-type block (N33b-19, ADR-ATEL-02), with the fields of `study.plan`: `anytime { null,
    margin, span, betting_rule, level, planning_effect? }` or `fixed_n { n, rule, level, margin,
    declared_verdicts }`. On the command line `--fixed-n <n> --rule … --declared-verdicts <tokens>` selects the
    fixed-n block (`--declared-verdicts` is the flag of `eval.preanalyze`) and `--margin` and `--level` fill the
    block given; over MCP the block is the property `anytime` or `fixed_n`. Without a block the verb builds an
    anytime plan with defaults: level 0.05, the task's declared margin and span per metric, a shared controller
    arm.
  - `claim` (`global`, `named` or `all`; N28a-20).
  - `pairing` (`exact`, `by-address` or `unpaired`; N35-07's `Asked`; default `exact`).
  - `sampling` (`nominal`, the default; `item-bank`, N24c-31; `stratified`, N24c-35 on KILL-ITEMBANK's fallback
    branch), with `epsilon` (in (0, 1], default 0.1), `fit <version>` (the item-bank fit; default the stored fit,
    refitted when the row set has grown by the declared factor, N24c-30), `explain-variance` (the exact per-draw
    variance; draws nothing) and `sequence` (the study's sequence choice; its range-driven value is refused with
    the item bank).
  - `state` (`privileged` or `estimated`; §22.3: every evaluation can run either way; both arms run under the one
    state named; DESIGN default `privileged`, the estimated twin being a second comparison under its own key;
    N20a-36 and N20b-37 call `--state estimated`).
  - `auto_causal` (`--auto-causal on|off`, default `on`; N20b-12: an arm that is a policy is admitted only with a
    passing causal-policy receipt for its interface, and with `on` the verb runs the test first when the budget
    allows).
  - `bracket` (`--bracket`, or `--bracket advisory`, which prints labels and takes no action), `scenario <w>` (one
    scenario of the world sample) and `relaxation <name>` (a relaxation that `frob-analyze` registers for the
    task); N25e-15.
  - `budget_curve` (`--budget-curve`; N28c-38).
  - `no_memo` (`--no-memo`; BD-111, BD-220: the memo bypass, generated from the row's `memo = true` by NJL-01's
    variation 14 and not declared by the row. The call is not served from the memo graph in any store mode, its
    nodes are still stored, and the envelope's memo disposition is `NoMemo`; the e-value trajectory equals the
    memoized one bit for bit, which BD-350's IT-9 checks).
  - The world sample has no seed input (DESIGN, with the reason). A study's address list (world, replicate) is
    drawn from the plan key (`seed_protocol.addresses = "philox:plan-key"`, the plan record `frob.plan/1`) and
    written to the ledger with the plan before any episode (BD-262 step 2), so `--run-seed`, the seed of
    `eval.run`'s single-task form, is FROB-REF-802 here. Reason: a caller who could name the seed of a comparison
    could choose its worlds after seeing outcomes on them; a study is reproduced from its plan, never from a seed.
- Outputs: a job; on completion the verdict per metric (`settled`, `tied`, `inconclusive`, `incomparable` or
  `infeasible`), the e-value and the confidence sequence, pairs used with the accounting identity, the tier; per
  metric the pairing label (`paired-exact`, `paired-by-address` with the realized pair correlation beside the
  verdict, or `unpaired`; N35-08); with a sampling mode, the sampling record (fit version, ε, q, the widened span,
  per-stratum differences and the realised saving factor); with `--bracket`, per design both ends (value, level,
  class, n) and the label, for a dominated design the competitor and both certificates, and the comparison's α, M
  and shared world sample; with `--across decimation`, per requirement the table, the differing addresses with
  margins and the label; with `--budget-curve`, the scores of each design by budget (N28c-38). Evidence: *study*.
- Chapter refusals: `FROB-REF-464` (`--state estimated` on a design without a sensor its filter needs; N22-REF);
  `FROB-REF-502` (an `in-sample` result offered as an arm; N24a-31); `FROB-REF-542` (a paired study under an
  uncontrolled determinism class, or with arms in different classes: a D4 arm, arms of different classes, or D3
  without a decision-level equivalence record; `pairing::admit`, which runs first); `FROB-REF-543` (an `anytime`
  study on a metric with no declared span); `FROB-REF-551` (with `--bracket`, a stop requested on a relaxation that
  does not enclose a declared state or energy source; N25e-15); `FROB-REF-553` (a request with both study blocks, or
  one that mixes study types); `FROB-REF-555` (a `fixed_n` block whose declared verdicts include an unreachable one);
  `FROB-REF-556` (a pairing keyed by event order or by a training counter); `FROB-REF-561` (the determinism licence
  of N35-07, after `pairing::admit`, for the cases 542 does not cover: an arm of unrecorded class, a pair on
  different device families demanded `exact` without class D1, a D3 pair demanded `exact`); `FROB-REF-843` (an
  item-bank refit whose row set holds a sealed confirmation row; N24c-30); `FROB-REF-890` (`stack-mismatch`: arms
  recorded under different stack receipts; N33c-31).
- Pending refusals (convention 5): `exposure-table-untested` (N24c-37, the study-admission check shared with
  `study.plan`); `coupling-choice-after-draw` (N24c-40, in the same admission check: a study on a task with a
  locked coupling choice whose address list was written to the ledger before the lock record; exit 3; repair:
  draw fresh addresses); `foil-as-candidate` (N24c-23: a foil body offered as an arm, refused by the one
  candidate-admission function; exit 3); `item-bank-in-fixed-n`, `item-bank-with-tier-correction` and
  `item-bank-needs-variance-adaptive` (N24c-31); `stratified-sampling-not-admitted` (N24c-35, a `fixed_n` study);
  `sampling-mode-not-built` (N24c-35, `--sampling item-bank` after KILL-ITEMBANK's fallback withdrew the mode, when
  the four item-bank refusals are retired); `causal-receipt-missing` (N20b-12, 5xx, exit 3).
- Implemented by: N33b-19 (M1: the CPU referee form, calling `eval.preanalyze` for a `fixed_n` block), BD-262 (M3:
  the device form, the options `metric`, `margin`, `max_pairs` and `backend`, and `state` on both forms through
  the shared plan path, with 464 raised by N22a-03's run admission), BD-124 (the study types).
- Extended by: BD-111 and BD-220 (`--no-memo`), N16c-44 (`--run`, `--across decimation`), N20b-12
  (`--auto-causal`), N24a-31 (502), N24c-23, N24c-30 and N24c-31 (the item bank), N24c-35 (`--sampling
  stratified`), N24c-37 (the exposure check), N24c-40 (`coupling-choice-after-draw`), N25e-15 (`--bracket`,
  `--scenario`, `--relaxation`, more than two designs, 551), N28a-20 (`--claim`), N28c-38 (`--budget-curve`),
  N33c-31 (890), N35-07 (`--pairing`, 561), N35-08 (the pairing report, the per-arm placement).

**`eval.detect`** (MCP tool `eval_detect`; owner `frob-learn`)
- Inputs: `result` (positional: the identity of an evaluation result: design, policy, task, split, tier, step plan,
  seeds); `detectors` (default all ten; a subset by flag name, for example `--detectors
  dt-sensitive,energy-created`); `pairs_budget` (per arm, default 32); `tiers` (for the tier detector; default the
  next tier up); `state` is inherited from the result; the idempotency key, budget and bounded wait.
- Outputs: the exploitation detectors that fired, with the evidence for each. Evidence: *screening*.
- Chapter refusals: `FROB-REF-561` (an arm of class D4 or of unrecorded class, refused by the determinism licence
  for paired statistics); `FROB-REF-867` (a result of a D4-trained policy with no re-evaluation on a stricter
  profile; N23-REF). A budget that cannot cover 16 pairs per arm is the kind's 701.
- Implemented by: BD-267. The `dt-sensitive`, `tolerance-sensitive` and `stiffness-sensitive` detectors are the
  first repair of `randomize-numerical-parameter` (FROB-REF-572, raised by task admission, not by this verb).

**`eval.ledger`** (MCP tool `eval_ledger`; owner `frob-arena`)
- Inputs (the read form, `eval:read`): a campaign id (positional), or a claim family (`--task`,
  `--baseline-class`, `--world-family`), which resolves to the family's open campaign, or to its latest closed one
  when none is open; `--read-sealed <name>` (BD-261: one sealed object of a battery campaign; see the firewall
  below).
- Actions (the entry keeps `kind = "read"` and declares the others, NJL-01's form for verbs with two kinds; an
  action is selected only by the input property `action`, on the command line `--action <name>` and over MCP
  `"action": "<name>"`, with no per-action flag; each action is of kind `admin`, so the generator gives it no
  idempotency key, no form of the verb has one, and `--key` on this tool is 802; each action is idempotent by its
  effect, as `ground.band`'s `retire` is: a repeated `open` finds the open campaign of its claim family or battery
  label and answers `campaign-already-open`, exit 7, naming it, with the index sequence untouched; a repeated
  `close` changes nothing and answers `campaign-not-open`, exit 3; a `seal` of a name already sealed in the
  campaign writes nothing, and with the same bytes returns the stored name and digest):
  `actions = [{ name = "open", kind = "admin", capabilities = ["eval:read", "qual:admin"] }, { name = "close", kind
  = "admin", capabilities = ["eval:read", "qual:admin"] }, { name = "seal", kind = "admin", capabilities =
  ["eval:read", "qual:admin"] }]`.
  - `open` (N24b-15): `frob eval ledger --action open --task <TaskId> [--baseline-class <class>] --world-family
    <family> [--slug <name>] [--alpha 0.05] [--clusters-per-request <n>] [--tier-plan '<json>'] [--max-requests
    <n> [--level-schedule bonferroni]] --json`, or a battery campaign, `frob eval ledger --action open --kind
    battery --label <name> --json` (no index and no levels). `--baseline-class` is absent for an absolute claim
    family; the slug defaults to the task's short name.
  - `close` (N24b-15): `frob eval ledger <campaign> --action close --json`; triggers N24b-17's reveal and
    retirement.
  - `seal` (BD-261): `frob eval ledger <campaign> --action seal --name <name> --file <path> --json`, only while the
    campaign is open; `name` (the object's name, required) and `file` are the seal action's own input properties,
    and either given with another action or the read form is 802; `--file` accepts a path, so 734 and 828 are
    appended by field.
- Outputs: `campaign`, `kind`, `state`, `claim_family`, `predecessor`, `alpha_total`, `level_schedule`, `index`
  (the last committed), `alpha_spent`, `next_alpha_i`, the rows `[{index, alpha_i, outcome, receipt_hash,
  units_used}]`, `max_requests` if set; after close also the revealed key's identifier and the location of the
  cluster derivation. Nothing sealed: no world address, no per-unit outcome, no trace reference. `open` returns the
  new `frob.qualify.campaign/1` record (its `key_commitment` written by N24b-17 in the same step); `close` returns
  the closed record; `seal` returns the object's name and digest; `--read-sealed` returns the object's bytes, an
  output field the static audit classifies as sealed until close. Evidence: *pass-through* (the arithmetic record:
  class `Tested`, tier `static`, determinism `exact`).
- The firewall on `--read-sealed` (DESIGN, with the reason): it is an input of the read form and not an action,
  because its kind is the entry's own (`read`) and after close it answers with `eval:read` alone (BD-261); before
  close the dispatcher's sealed-object check (BD-261, layer 1) admits only a session holding `qual:admin`, writes
  each such read to the ledger as `sealed-admin-read {session, campaign, name, ledger_seq}`, and refuses every
  other session with FROB-REF-840, exit 5, logged as a `sealed-read-attempt`.
- Chapter refusals: `FROB-REF-840` (sealed confirmation data or a sealed object asked for by a session without
  `qual:admin` before close); `FROB-REF-886` (`open` on a retired family; N24b-17). An action without
  `qual:admin` is the appended 720, exit 5.
- Pending refusals (convention 5): `campaign-already-open` (8xx, exit 7: a second campaign on a claim family, or
  on a battery label, that has one open, or more open campaigns than `families_per_task_family`); `qualification-undecided` (8xx, exit 3:
  `open` on a non-synthetic claim family while OPEN-ARENA-3 is undecided); `campaign-not-open` (8xx, exit 3: `close`
  or `seal` of a campaign that is not open).
- Implemented by: N24b-15 (the campaign object, the index, the level ledger, `open` and `close`), BD-259 (the
  lane), BD-261 (`seal`, `--read-sealed` and the firewall).

**`eval.preanalyze`** (MCP tool `eval_preanalyze`; owner `frob-arena`)
- Inputs, by mode (one mode per request; inputs of two modes together are FROB-REF-802):
  - fixed-n (N24b-06): `rule`, `n`, `level` (a rational string, default 0.05), `margin`, optional
    `declared_verdicts` (tokens `promote`, `inferior`, `practical-tie`, `inconclusive`), optional `alternatives`
    (pairs of decimal strings), optional `first_reachable_up_to` (the cap of the search for the smallest n per
    verdict);
  - cadence (N23b-34): `cadence <looks>x<episodes>` (for example `50x30`), `target <p₀>`, one or more
    `true_rate <p>`, and the same `level` (no `--alpha`);
  - ten-fold plan (N39b-34): `plan <file>` (a ten-fold study plan; a path, so 734 and 828 are appended by field).
- Outputs: `frob.eval.preanalyze.out/1`. In the fixed-n mode its result is the record `frob.preanalysis/1` of
  N24b-06 unchanged (`rule`, `n`, `margin`, `level`, `interval_level`, `outcomes`, `counts` per verdict,
  `reachable`, `first_reachable`, `min_b_for_prefer`, `concordance_interval`, `power`, `hash`: the pre-analysis
  hash that studies and qualification requests cite); in the cadence mode the member `cadence` (shape
  `frob.eval.preanalyze.cadence/1`: per true rate the point-score, unadjusted-bound and attainment crossing
  probabilities, `min_all_success_episodes`, `attainment_reachable_from_look`); in the plan mode the reachable
  verdicts per cell and the necessary family count. DESIGN: one output schema, with the stored record embedded, so
  that a plan's `preanalysis.hash` equals the hash this verb prints. Evidence: *static*.
- Chapter refusals: `FROB-REF-532` (a fixed-n study has no pre-analysis, or names a verdict it cannot reach; the
  reachable set and the first n at which the missing verdict becomes reachable are attached).
- Pending refusals (convention 5): `suite-too-small-for-population-claim` (5xx, exit 3: a ten-fold plan declares a
  population result with fewer independent families than a valid one-sided bound needs; N39-REF, N39b-34).
- Implemented by: N24b-06, BD-124, N33b-19.
- Extended by: N23b-34 (the cadence mode), N39b-34 (ten-fold study plans).

**`eval.promote`** (MCP tool `eval_promote`; owner `frob-opt`)
- Inputs: `result` (a fast-tier result; marked `accepts = "verdict"`, so it passes through
  `frob_verbs::require_establishable` and an advisory output offered here is FROB-REF-872; N28c-12); `tier`
  (default `referee`); `adjudicator` (DESIGN: `auto`, the default, is BD-263's step 3: `referee` where the referee
  completes the stratum's scenarios, otherwise `batch-cpu:strict`, recorded; `referee` or `batch-cpu:strict` names
  it, so that N16c-09 can promote the same results under each adjudicator law); `law` (DESIGN: the strict
  adjudicator's contact law, `--law CL-1|CL-H`, spelled as `sim.run`'s `--law` spells the law table's rows, so a
  lowercase `cl-1` or `cl-h` is FROB-REF-802; default the law the `strict` profile runs by default, CL-1 with
  located events until OPEN-KIN-4 is decided; `--law` with `--adjudicator referee` is FROB-REF-802, because the
  referee runs its own law; every `established` label names its adjudicator and law, `batch-cpu:strict/CL-1`);
  `method` (`paired`, `dr`, `multilevel`, `segment`);
  `quantity` (default the result's primary metric); `budget_pairs` (default 32 for `paired`, 256 higher-tier
  episodes for `dr`); `require_rule` (`--require-rule table`: pins the decision core's rule, so a missing or stale
  core table is refused instead of completing on the fallback; N28c-02, called by N28c-05, N28c-21 and BD-349).
- Outputs: the promoted result with standing `established { adjudicator }` or the reason it is not
  (`provisional: tier-sensitive` with both numbers, `provisional: referee-unavailable`); ρ and the higher-tier-only
  count it replaced; for a task with a deadline requirement, the cross-decimation block of N16c-44 (a labelled
  deadline verdict is not promoted). Evidence: *fidelity*.
- Chapter refusals: `FROB-REF-171` (`under-built-promotion`: a design, or a comparison with such a design as an arm,
  labelled `structure-unchecked` because a link is under-built under measured loads; N31b-40, loaded by N31b-45);
  `FROB-REF-246` (with `--law`, a law not admitted on the strict adjudicator's cell); `FROB-REF-502` (an `in-sample`
  result; N24a-31); `FROB-REF-548` (pooling evidence or calibration across two model identities);
  `FROB-REF-561` (the determinism licence of N35-07 on the promotion's paired statistic, for what 825 does not
  answer: the fast-tier result and its higher-tier re-evaluation paired across device families without class D1,
  and arms that cannot be paired, which includes a solve-path mismatch between the two that the request did not
  declare, BD-263); `FROB-REF-811` (a tier-corrected estimate across identities, with no audited
  address, or below the effective-sample floor); `FROB-REF-817` (sampling-controller episodes offered as the
  scoring source of a promotion; raised at admission by `bind_ahead`, N22a-25, N22-REF); `FROB-REF-820` (label
  misuse: a result that is not `established` cited as established, or a `model-conditional` ranking used to
  authorize spend; also a result labelled `experimental-execution`, for example a run under `--solve-queues` on a
  backend where GATE-KIN-08 has not passed, offered as promotion evidence, N16c-15 and BD-263); `FROB-REF-821`
  (envelope not frozen: promotion requested for a quantity whose envelope is not
  hashed in the ledger; its repair, freezing the envelope, is `eval.run --freeze-envelopes`); `FROB-REF-825` (a
  stored result of class D4 or unrecorded offered as the `result`: the evidence boundary of N37-28, which runs in
  the dispatcher before the handler and is therefore the one code for that input on this verb; N35-07's rule for
  the promotion use names the same code); `FROB-REF-830` (a `model-conditional`
  result offered for promotion, build or prototype budget); `FROB-REF-871` (with `--require-rule table`, a
  decision-core table missing, stale or not the registry version; on a normal call the verb completes on the core's
  fallback and carries 871 in `warnings`; N28c-02); `FROB-REF-872` (an advisory output offered as the `result`;
  N28c-12); `FROB-REF-890` (`stack-mismatch`; N33c-31).
- Pending refusals (convention 5): `foil-as-candidate` (N24c-23); `wins-outside-intent-unresolved` (N24c-21).
- Implemented by: BD-263 (the decision logic in `frob-opt::promote`, layer 4, with the inputs `adjudicator` and
  `law` from M3; `frob-dyn` contributes only the pure pair comparison). N16c-09 (the adjudicator-law comparison,
  M5) calls the two inputs and adds nothing to the row.
- Extended by: N16c-15 (the `experimental-execution` case of 820), N16c-44 (deadline verdicts across
  decimations), N22a-25 (817), N24a-31 (502), N28c-02 (`--require-rule`, 871), N28c-12 (872), N31b-40 (171),
  N33c-31 (890), N37-28 (825).

**`eval.qualify`** (MCP tool `eval_qualify`; owner `frob-arena`)
- Inputs: `design` (resolved to `candidate: {design, controller}` with the design's frozen controller record, the
  long form a caller may give instead, `--candidate design=<d>,controller=<c>`); `task`; `claim`; `campaign` or claim
  family; or, in place of `task`, `claim` and the campaign, `mission` (DESIGN, the mission-bound form of the
  written session of §32.16, `frob eval qualify carrier-b --mission q:5b1e… --key qual-1 --json`, implemented by
  BD-259: the mission query resolves to its task and its requirement as `task` and `claim`, and to the open
  campaign of that claim family; the resolved `task` and `claim` enter the commitment beside `mission`; a mission whose predicates do
  not reference the requirement terms of exactly one task is `claim-family-mismatch`, pointer `/mission`;
  `mission` given together with `task` or `claim` is FROB-REF-802); optional `baseline` (absent for an
  absolute claim), `study`, `tier_plan`, `clusters` and `budget` (defaults filled from the claim and the task's
  registered rule and echoed in the commitment); `dry` (the reachability preview, which commits nothing and spends
  no level); `sampling` (accepted only to be refused unless `nominal`: the blinded lane draws from p on fresh sealed
  clusters; N24c-32, N24c-35); and the common `sync_wait_ms` and idempotency key. The minimal call names `design`,
  `task`, `claim`, the key and `sync_wait_ms`, and must succeed.
- Outputs: `frob.qualify.receipt/1`: decision, index, level spent, units used, receipt hash; nothing per world;
  progress events carry only episodes done of total. Evidence: *qualification*.
- Chapter refusals (checked before commitment, so a refused request advances no index and spends no level, except
  where stated): `FROB-REF-502` (an `in-sample` input, at the commit step; N24a-31); `FROB-REF-532` (a fixed-n study
  with no pre-analysis, or naming a verdict it cannot reach); `FROB-REF-533` (paired statistics under an
  uncontrolled determinism class, a D4 arm included; `study::admit`, N24b-16); `FROB-REF-543` and `FROB-REF-556`
  (the pairing rules, through `study::admit`); `FROB-REF-552` (a request that mixes study types);
  `FROB-REF-567` (a hard-world or search-selected world offered as a confirmation sample); `FROB-REF-817`
  (sampling-controller episodes offered as the scoring source; N22a-25); `FROB-REF-825` (a stored D4 or unrecorded
  result offered as the candidate's evidence, or a tier plan naming `fast-training`; N24b-16, N37-28);
  `FROB-REF-840` (a session without `qual:admin` asks for sealed confirmation data before close); `FROB-REF-841` (a
  confirmation request was not committed first, or differs from its commitment; exit 3; a reused idempotency key
  with a different request is not 841 but the kind's 803, exit 7, with no index assigned, N24b-16);
  `FROB-REF-842` (the units the request needs at its level exceed its budget; the reachable
  verdicts are attached); `FROB-REF-843` (the candidate's lineage contains data from the sealed family);
  `FROB-REF-844` (a cluster would be used twice); `FROB-REF-845` (a result cited as qualified without a promoting
  receipt; raised by the standing check N24b-19). A cancelled, budget-stopped or faulted request ends
  `inconclusive` with the level spent and the clusters consumed. Scope (720), budget (701) and key (803, 805) codes
  are the kind's.
- Pending refusals (convention 5): `campaign-not-open` and `claim-family-mismatch` (N24b-16, 8xx, exit 3);
  `campaign-request-cap` (N24b-15, 8xx, exit 3); `task-audit-missing` and `task-audit-unresolved` (N24c-22);
  `wins-outside-intent-unresolved` (N24c-21); `foil-as-candidate` (N24c-23); `item-bank-in-qualification` (N24c-31)
  and `stratified-sampling-not-admitted` (N24c-35) for `--sampling`, and `sampling-mode-not-built` for `--sampling
  item-bank` after KILL-ITEMBANK's fallback.
- Implemented by: BD-259 (the lane and the mission-bound form; 840 by the firewall BD-261, 845 by N24b-19; the
  admission checks by N24b-16).
- Extended by: N22a-25 (817), N24a-31 (502), N24c-21, N24c-22, N24c-23, N24c-32 and N24c-35 (the pending
  refusals above), N37-28 (825).

**`eval.rank-agreement`** (MCP tool `eval_rank_agreement`; owner `frob-opt`)
- Inputs: `designs` (a design set, drawn by a keyed stream before any score exists); `pair`, two controller rungs
  or a tier pair (`tier:batch-cpu,referee`); `class`; `task` (the task family); `episodes_per_design`;
  `controller`; stratum; `k` and `m` (the requested cut: m finalists wanted, k designs kept; N23b-35); `band` (the
  rank band τ is computed on, for example `true-top-10`; N23b-35); `fixture` (a registered synthetic score table,
  for example `forty-design-winner-third`: runs nothing, tier `static`, every output labelled `synthetic`;
  N23b-35, GATE-CORTEX-21); budget; the idempotency key.
- Outputs: the rank-agreement record (τ with interval, strata, `mode` and permitted use per stratum) written to the
  ledger; one vocabulary for tier-pair and rung-pair rows; for a requested cut (m, k), `tau_band` with its
  bootstrap interval, `top_k_recall`, `containment_rank`, `regret` and `admissible: true|false` (N23b-35; a cut
  below the containment rank is refused by the allocation entry, `opt.cascade`, not here). Evidence: *study*.
- Chapter refusals: `FROB-REF-537` (a rung used to select or discard without a rank-agreement record that meets its
  threshold); `FROB-REF-545` (a rung asked to select in a stratum whose mode is not `selective`); `FROB-REF-546`
  (calibration from survivors: a design set holding a design not drawn by the keyed stream; N28b-13, BD-328's
  `survivors_calibration_forbidden`); `FROB-REF-548` (pooling evidence or calibration across two model identities).
- Implemented by: BD-328 (rung pairs, M4), BD-265 (tier pairs, M3), N23b-35 (the winner-check report fields and
  the fixture form, M3). The row's `first_milestone` keeps the appendix's M4: from M3 the verb answers a tier pair
  and the fixture form and is listed in `frob capabilities` as implemented with `form: "tier-pair"`; a rung pair
  before BD-328 answers `FROB-REF-724` naming the form (NJL-01, NJL-03).

**`eval.run`** (MCP tool `eval_run`; owner `frob-arena`; the policy form's path is `frob-learn`'s, N23b-09)
- The candidate:
  - `design` (positional), or `model` with `controller` (N24c-04's form). DESIGN: in the battery form the design
    may be omitted when `class` is given; the battery then runs on each reference robot of the class on file in
    the reference-robot asset tree (`assets/robots/`, NEG-15), resolved at run time and recorded in the run record,
    as N24b-01 resolves the design list of `smoke` (a battery is a population of scenarios, and the designs are an
    argument of the run, never part of `BatteryId`); neither a design nor a class is FROB-REF-802. In the policy form
    a missing design is the policy's design parent (N23b-09).
  - `controller`: `baseline` (the default), `baseline+residual:<policy>` (N22a-16; `baseline+residual:zero` is the
    conformance selector), a `ControllerId`, `fixture:<name>`, or an exported policy (N24a-31); the run header
    carries the bound `ControllerId`.
  - `policy` (N23b-09): `<family | checkpoint>` evaluated in place of a controller, bound to the design by role
    through N23a-13's matcher; a bound family policy (BD-321, N23bx-02); the control forms `zero:<spec>`,
    `random:<spec>:<seed>` and `masked:<spec>:<role>` (N23b-15; a malformed form is FROB-REF-802). `controller` and
    `policy` together are FROB-REF-802 (DESIGN: one candidate per run). A *policy family* (N23b-09) is the set of
    stored policies whose training requests (`frob.train.start.in/1`, the idempotency key removed) are equal
    except for the seed; it is named by any one member, its identity `pf:<digest>` is the digest of that request
    with the seed removed and is printed in the report as `policy_family`, and a single checkpoint is a family of
    one.
- The population, exactly one of:
  - `battery` (`--battery <name>`, with optional `class`), resolved once by `battery::resolve(name, class)` before
    the request is digested, the `BatteryId` echoed in `bound` (N24b-01, N24c-04); the world addresses are the
    items' own `(run_seed, world, episode)` triples, which the manifest stores and `ItemId` hashes (N24b-01), so
    the battery form takes no seed and no world count;
  - the single-task form: `task` with `worlds <n>` and `run_seed <s>` (N24c-04);
  - the policy form: `task` with exactly one of `split` (`held-out` or a split id) and `stream dev` (the registered
    development stream, N23b-33), and `seeds <S>` (the number of training seeds of the family to evaluate: the S
    lowest seeds on file; when fewer are on file those are evaluated, the count is recorded, and a report on
    fewer than five is labelled `under-seeded`);
    `split` and `stream` together, or a task with neither under `--policy`, is FROB-REF-802 (N23b-09). With
    `--policy` a battery (`--battery <name>` in place of `task` and its split or stream) is also a valid
    population, N23-REF's role-matched loading case: the policy is bound by role and the run returns the battery
    form's outputs (per-scenario verdicts and the scorecard handle); the five-seed report is the task form's.
  `run_seed` (`--run-seed`) is the one spelling of the seed (N24c-04); `--seed` is not an alias. `--run-seed` or
  `--worlds` beside `--battery` is FROB-REF-802 with the pointer at the input (DESIGN: a seed that changed the
  addresses would change the population under the same `BatteryId`, and one that changed nothing would be an input
  that is silently ignored; repair: omit it, since two runs of one battery are on the same addresses by
  construction, which is what tier pairing and the determinism lanes need).
- Task start (N15a-29, M1): `thermal_start` (`--thermal-start cold|soaked:<T>|steady`) replaces, for this run, the
  thermal start the task declares for evaluation (the field `thermal_start` of its `reset` section; default
  `steady`). An override never edits a task: it yields a derived task whose canonical bytes carry the overriding
  value and whose `TaskId` is new, and the envelope's `bound` block lists both identities. The resolved start per
  actuator is recorded with the episode; at actuator level L1 the result carries the flag `thermal-start: no
  thermal state at L1`.
- Placement and step plan: `tier` (`referee` at M1; `batch-cpu`, `gpu-cuda`, `gpu-metal` from M2); `profile`
  (`lockstep`, `strict`, `referee`), `law`, `step <duration>`, `newton_iterations <n>` and `warm_start on|off`
  (N16bp2-01: forwarded to the run's step plan with `sim.run`'s spellings; a changed option is a different
  `StepPlanId` and therefore a different stratum); `solve_path dense|ancestor` (N16c-38); `solve_queues
  off|auto|<q0,q1,q2>` (N16c-15, M2: forwarded as `solve_path` is, with `sim.run`'s spelling and values; it is an
  execution option outside `StepPlanId`, so it makes no new stratum, and the receipt records it, `execution:
  "queued"`; outside `--profile lockstep` it is FROB-REF-802, pointer `execution.solve_queues`; a queued run on a
  backend where GATE-KIN-08 has not passed is labelled `experimental-execution`); `determinism` (the
  profiles of N35-04; `fast-training` is selectable only on the training verb), `precision default|stiff|f32|f64`
  and `on_degraded refuse|continue-flagged` (default `refuse`), parsed by N35-04's shared admission;
  `on_overflow refuse|reset|continue-flagged` (default `refuse` for evidence-bearing evaluation, N21b-23;
  GATE-KIN-07 runs `continue-flagged`, N16b-03).
- Recording: `telemetry T0|T1|T2|T3|task-all` (forwarded to the run by the verb's handler, N24c-04 on the referee
  and N24cp3-01 on the batched tiers, with `sim.run`'s spelling and default, `T1`: the highest level recorded; it
  enters the manifest only, so physics digests, `StepPlanId` and the stratum are unchanged; a lowercase level is
  FROB-REF-802). On the batched tiers the run records the selection `task-all` whatever the level, because the
  host verdicts read it (N24cp3-01, N24cx-01). A metric over a stream the level did not retain is refused by
  `eval.scorecard` with FROB-REF-888 and is never printed as zero (N24a-21: the fallback count of a run at `T0`).
- Conformance builds only: `conformance_plant <file>` (`--conformance-plant`; N29a-25's plant file, schema
  `frob.conformance-plant/1`, the same input as on `sim.run`; N24cp3-01). It is absent from a release build's
  schema, where it is FROB-REF-802.
- Fidelity: `adjudicate referee-only|patch-reduction|strict-fallback` (N16a-05's adjudication policies) and
  `record coverage` (writes the `referee_class_coverage` row), both BD-213; `freeze_envelopes` (when the run has
  completed every address of a stratum on both tiers, the job freezes that stratum's fidelity envelope and lists
  the new `EnvelopeId`s; a stratum still incomplete is `not-frozen: incomplete`; the option needs the scope
  `fidelity:admin`, checked by the handler: without it the run proceeds and the option is refused with
  `FROB-REF-720`, exit 5; N16bp1-01).
- Arena and learning: `in_sample` (N24a-31: an evaluation that reuses training scenes runs as a diagnostic, every
  scorecard row labelled `in-sample`); `state privileged|estimated` (N23b-09: default both, issued as the twin at
  the same addresses; one value labels the report `privileged-only` or estimated-only); `randomization
  fixed|adaptive|training-distribution` (N23b-04; default `fixed`, the fixed nominal law; `adaptive` is refused as
  an evaluation before any episode runs, with `evaluation-under-adaptive-law`; `training-distribution` is the
  diagnostic that refusal's second repair names: the episodes are drawn, at the run's world addresses, from the
  policy's own training law, which is the adaptive law in force at its last update as its manifest's
  `trained_on.randomization` records it, or the nominal law for a policy trained with fixed randomization; every
  result is labelled `training-distribution`, its `evidence.claims` is empty, and it cannot be attached to a gate
  record, cited in a comparison or reported as a return); `auto_causal on|off`
  (default `on`; N20b-12).
- Safety and model variants: `shield <ShieldId>` (BD-338: the `ActionFilter` stage enters the run and its
  identity; an unknown `ShieldId` is FROB-REF-802); `shield_budget <EnergyBudgetId>` (BD-337: the specialized
  energy monitor runs inside the evaluation; an unknown or unadmitted budget is FROB-REF-802); `variants <set>`
  (N31a-50, M6: a set handle of aged `ModelId`s run as per-world variants in one batch on the same world addresses).
- The idempotency key and budget (two runs that a test compares carry distinct keys).
- `no_memo` (`--no-memo`; BD-111, BD-220: generated from the row's `memo = true` by NJL-01's variation 14 and not
  declared by the row; nothing is served from the memo graph in any store mode, every node is computed and still
  stored, and the envelope's memo disposition is `NoMemo`).
- Outputs: per scenario a `frob.verdict/1` record in the ledger and the episode in a `.frun`, in address order; the
  run identifier, counts per outcome column and the battery identity (`battery: {id, items}`); a scorecard handle;
  in the policy form the report `frob.evalreport/1` (interquartile mean, stratified bootstrap interval, pass-rate
  matrix, estimated-state twin, compute block; N23b-09); after a battery run on two tiers, the block `fidelity:
  {pairs_written, unpaired_by_cause, strata, adjudicators, intrinsic_addresses, tier_correction}` (N16bp2-01),
  where `tier_correction: [{ stratum, claim }]` has one element per tier-correction row that the run's pairs
  updated and `claim` is the claim identifier a caller passes to `evidence.trace` to read that row (N29b-07; an
  additive member, empty when no row was updated); with
  `freeze_envelopes`, the frozen envelopes; with `variants`, one result column per variant on equal addresses;
  with a `thermal_start` override, the task's and the derived task's `TaskId`s in `bound`.
  Evidence: *run*.
- Chapter refusals: `FROB-REF-103` (a battery manifest with an unknown, misspelt or duplicate key, at
  `battery::resolve`; nothing runs; N24b-01, N24c-04); `FROB-REF-132` (no admissible thermal equilibrium for the
  task's thermal start when the task meets the design: the temperature, or the margin to the tangency, and the
  largest sustainable load are returned, and no start is substituted; N15a-15, N15a-29);
  `FROB-REF-222` (`--profile strict` or `referee` on a GPU tier); `FROB-REF-245` (a half-precision
  spelling of `--precision`); `FROB-REF-246` (a law not admitted on the tier and profile); `FROB-REF-247` (the
  determinism class required is not attainable on the placement); `FROB-REF-248` (a world whose solve is
  ill-conditioned beyond its precision profile under `on_degraded refuse`, nothing partial published; N35-04,
  raised through the M2 tier form, N24cp3-01); `FROB-REF-421` (no synthesis route and no
  admitted fallback, when the run needs a baseline controller; N22-REF); `FROB-REF-457` (a shield whose margin key
  differs from the run's; BD-338); `FROB-REF-464` (`--state estimated` on a design without a sensor its filter
  needs; N22-REF); `FROB-REF-502` (an evaluation reuses a scene from the policy's training manifest, without
  `--in-sample`; N24a-31); `FROB-REF-522` (a learned policy emits a non-finite action during the job; N23-REF);
  `FROB-REF-529` (no settled-pose cache for this model and task); `FROB-REF-531` (the policy's mapping table does not
  match the design by role; N23a-13); `FROB-REF-534` (a family policy applied to a design outside its class or its
  pretraining distribution; BD-320); `FROB-REF-601` (device unavailable or lost) and `FROB-REF-602` (the memory
  estimate exceeds the device budget), on a GPU tier; `FROB-REF-604` (a precision with no step kernel on the tier);
  `FROB-REF-608` (`Faulted` worlds above the evaluation threshold, which is 0; N21b-23), on a GPU tier;
  `FROB-REF-610` (capacity overflow under `refuse` on a GPU tier; N21b-23) and `FROB-REF-722` (the same on
  `batch-cpu`; KNOWN_DRIFTS 6: the two are not merged); `FROB-REF-726` (a living-world mechanism not admitted for
  the run's purpose or tier; N32-REF); `FROB-REF-867` (a D4-trained policy cited before a re-evaluation under a D2
  or D3 profile exists; N23-REF). `FROB-REF-861`'s first repair is this verb, `frob eval run --policy <id>
  --determinism device-replay` (N35-06), which is admitted.
- Pending refusals (convention 5): `battery-manifest-invalid` (N24b-01, returned through this verb by N24c-04);
  `evaluation-under-adaptive-law` (N23-REF, N23b-04: `--randomization adaptive`; 5xx, exit 3, `pointer`
  `randomization`; repairs: 1. evaluate under the fixed nominal law, 2. run the same request as the diagnostic
  `--randomization training-distribution`, which is never an evaluation); `causal-receipt-missing` (N20b-12,
  5xx, exit 3); `coupling-choice-after-draw` (N24c-40: a run on a task with a locked coupling choice whose
  `run_seed` is the pilot seed that the choice recorded as consumed; exit 3; repair: draw fresh addresses).
- Implemented by: N24c-04 (M1: the referee form), N24cp3-01 (M2: the tier form, `batch-cpu` and the GPU tiers
  with N35-04's shared profile admission, N21b-23's fault and overflow policy and `conformance_plant`), BD-125.
- Extended by: BD-111 and BD-220 (`--no-memo`), BD-213, BD-320 (534), BD-337, BD-338, N15a-29 (`--thermal-start`, 132), N16bp1-01
  (`--freeze-envelopes`), N16bp2-01 (the step-plan options and the `fidelity` block), N16c-15 (`--solve-queues`),
  N16c-38, N20b-12, N21b-23, N22a-16, N23b-04, N23b-09, N23b-15, N24a-31,
  N24b-01, N24c-40 (`coupling-choice-after-draw`), N29b-07 (`fidelity.tier_correction`), N31a-50, N35-04.

**`eval.scorecard`** (MCP tool `eval_scorecard`; owner `frob-arena`)
- Inputs: a run of `eval.run`, or several runs of one battery, or a battery and a design (the latest run); optional
  reference run (the ratio to reference of each metric; N24c-04). The several-runs form names the runs as
  positionals, `frob eval scorecard <run-1> <run-2> --json`, and pools their episodes into one scorecard; runs of
  two different `BatteryId`s are the appended FROB-REF-802 (pointer at the first run of the second battery;
  N24c-04). Optional `quantile` (`--quantile <p>`, a probability in (0, 1): each metric's distribution also
  reports that quantile under N24a-17's sample-count rule, served by the verb's handler, N24c-04; an upper-tail
  quantile p needs n ≥ ln(0.05)/ln(p) independent episodes, 299 for p = 0.99, and below that the cell is an
  answer and not a refusal, `{status: "not-observable", n, p_max, n_needed}` with p_max(n) = 0.05^(1/n) and no
  value, the envelope `ok` with exit 0, and `next_actions[0]` naming the run that would add the missing
  episodes).
- Outputs: `frob.scorecard/1` (N24a-22's schema): battery identity and size, design, controller, tier, the seven
  outcome columns (`violated` by failure class, reasons for `undetermined`, refusal codes for `refused`, witness
  references for the infeasible columns), the two denominators with the success rate and its interval for each,
  metrics by scene class (value, unit, interval, n, ratio to reference), alarms, and `hazards_drawn_unguarded` and
  `hazards_uncovered`, each with the list of row identifiers behind it (NEG-29; an additive change whose help-schema
  snapshot is updated in the same change); labels `in-sample` and `baseline-only: unranked` where they apply; in
  the metadata block, each metric's `smoothness`, the attribute of its record in `registries/metrics.toml`
  (N24ax-04, also additive), so that an agent can see which metrics a gradient verb will accept.
  Evidence: *pass-through* (the claim product of the episodes; `provisional` on fast tiers).
- Chapter refusals: `FROB-REF-502` (the check repeated on stored runs against the manifests as they are now;
  N24a-31); `FROB-REF-845` (a result is cited as qualified without a promoting receipt); `FROB-REF-861` (a number
  observed during a `fast-training` run, such as a learning curve point or a training return, attached to the
  scorecard, or a policy trained under that profile cited there with no re-evaluation under `device-replay` or
  stricter; the citation check `cite_check` of N35-06, wired by BD-269, whose planted case 6 is this verb's;
  first repair `frob eval run --policy <id> --determinism device-replay`);
  `FROB-REF-865` (a summary statistic that omits what it excluded); `FROB-REF-888` (a metric over missing
  samples: a channel series with a recorded gap, or a stream the run's telemetry level did not retain, such as
  the Control stream behind the fallback count; N24a-17, N24a-21; repair: re-run with the channel retained);
  `FROB-REF-890` (`stack-mismatch`: the several-runs form pooling runs recorded under different stack receipts;
  from M1; N33c-31).
- Implemented by: BD-254 (M3: the full form), N24c-04 (M1: the referee form).
- Extended by: NEG-29 (the hazard counts), N22c-51 (the label `baseline-only: unranked`), N24a-31 (502 and the
  label `in-sample`), N24ax-04 (`smoothness`), N24b-19 (845), N33c-31 (890), N35-06 and BD-269 (861), N35-28
  (865).

**`eval.items`** (MCP tool `eval_items`; owner `frob-arena`; paginated: `paginated = true`, so the generator adds
`cursor`, `limit`, `follow` and appends 806 and 807)
- Inputs: battery (version); `class` (selects the bank, which is keyed by class and battery version); optional
  `fit <version>` (default the latest; old fit versions stay readable so old comparisons replay); optional
  `comparison` (`--comparison <id>`: one stored paired comparison that sampled from the item bank, the record
  `eval.compare --sampling item-bank` writes and `explain.compare` reads. DESIGN: the input is the comparison's
  identifier and not a pair of designs, because two designs can be compared many times, on other tasks, fits and
  keys, and the weights belong to one comparison; N24c-30, called by N24c-32's `items_for_comparison` step).
- Outputs: the fit version; per item its identity, p(i), row count, a_i, b_i (or μ_i, s_i) and discrimination
  ‖a_i‖; with `comparison`, the sampling weights it used, q(i), w(i) = p(i)/q(i) and the number of draws; sorted by
  ‖a_i‖ descending by default, paginated against a committed frontier. Nothing sealed is returned; the verb never
  refits. Evidence: *pass-through*.
- Chapter refusals: none of its own (the sealed-row refusal of a fit, FROB-REF-843, is raised by the refit inside
  `eval.compare`).
- Implemented by: N24c-30. On KILL-ITEMBANK's fallback branch (N24c-35) the verb is unregistered with its row marked
  withdrawn and the decision record cited, in the same change as the withdrawal (a breaking snapshot change).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `eval.compare` | `eval:run` | `frob.eval.compare.out/1` | `studies` | `frob eval compare quad-a@prev quad-a --task stairs-0.4 --key cmp-1 --json` |
| `eval.detect` | `eval:run` | `frob.eval.detect.out/1` | `inherit` | `frob eval detect r:19e2 --key det-1 --json` |
| `eval.ledger` | `eval:read` | `frob.eval.ledger.out/1` | `qualify` | `frob eval ledger q:kitchen-tray/1 --json` |
| `eval.preanalyze` | `eval:read` | `frob.eval.preanalyze.out/1` | `studies` | `frob eval preanalyze --rule paired-clopper-pearson/1 --n 64 --margin 0.05 --level 0.05 --json` |
| `eval.promote` | `eval:run` | `frob.eval.promote.out/1` | `compare`, `fidelity` | `frob eval promote r:19e2 --tier referee --key pro-1 --json` |
| `eval.qualify` | `qual:run` | `frob.qualify.receipt/1` | `qualify` | `frob eval qualify quad-tray --task t:kitchen-tray --claim 'failure probability of delivered at most 0.05' --key qual-1 --json` |
| `eval.rank-agreement` | `eval:run` | `frob.eval.rank-agreement.out/1` | `fidelity`, `inherit`, `ladder` | `frob eval rank-agreement --designs @set:40 --pair tier:gpu-cuda,referee --key ra-1 --json` |
| `eval.run` | `eval:run` | `frob.eval.run.out/1` | `learn`, `batteries` | `frob eval run quad-a --battery standard --key ev-1 --json` |
| `eval.scorecard` | `eval:read` | `frob.scorecard/1` | `qualify`, `batteries` | `frob eval scorecard quad-a --json` |
| `eval.items` | `eval:read` | `frob.eval.items.out/1` | `audit`, `batteries` | `frob eval items --battery standard --class quadruped --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block (numbers only; pending slugs are not entered, convention 5); `idempotent` is `true` for `read` and
`admin` and `false` for `run` and `mutate` (which take an idempotency key); `cost_model` is `static: …` for `read`,
`mutate` and `admin` and the `--estimate` cost model for `run`; a `run` row carries `wait = { cli =
"to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the crate of the block; `purpose` is the Purpose
column. `eval.ledger` carries the `actions` list of its block, `eval.items` carries `paginated = true`, and the
`result` parameter of `eval.promote` carries `accepts = "verdict"`. `eval.compare` and `eval.run` have `memo =
true` (BD-111's field, set in its change at M1): NJL-01's variation 14 generates their `no_memo`, and neither row
declares it as an input of its own (a row that does fails the lint `common-property-shadowed`).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "eval.compare"
cluster = "eval"
kind = "run"
tier = "S"
capabilities = ["eval:run"]
input_schema = "frob.eval.compare.in/1"
output_schema = "frob.eval.compare.out/1"
refusals = ["FROB-REF-464", "FROB-REF-502", "FROB-REF-542", "FROB-REF-543", "FROB-REF-551", "FROB-REF-553",
            "FROB-REF-555", "FROB-REF-556", "FROB-REF-561", "FROB-REF-843", "FROB-REF-890"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["studies"]
examples = ["frob eval compare quad-a@prev quad-a --task stairs-0.4 --key cmp-1 --json"]
owner = "frob-opt"
purpose = "Plan and submit a paired study on common world addresses; the reachability pre-analysis is `eval.preanalyze`"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` with the values `leave`, the default,
   and `cancel_job`, and `envelope_scope`, on the command line `--envelope-scope <gate:…|campaign:…|pretrain:…>`,
   the approved compute envelope the job cites, KNOWN_DRIFTS 14; `mutate`: `idempotency_key`, `expected_parent`).
   `idempotency_key` is never in the JSON-schema `required` array: the annotation `x-frob-key` (`"required"`, or
   `"optional"` on a `key_optional` entry) carries the rule, and a missing key is refused by the admission
   transaction with 805 on both surfaces. The generator appends the surface-wide refusals: `FROB-REF-720` (scope
   missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema failure, including an
   unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the compute envelope in
   force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803, 804 (stale parent)
   and 805 for `mutate`. The codes appended by field follow NJL-01's variation table: 806 and 807 for a paginated
   read; 808 (an integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property,
   whatever its kind, which every `run` entry has through `sync_wait_ms`; a `key_optional` entry is not given
   805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790 for a non-empty `needs_groups`; 891 to
   893 for artifact `params`; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The variations that later beads land (10 to 16 of that table: the
   loop-order override, `from_proposal`, `machine`, `engine`, `no_memo` for `memo = true`, the reality guard's
   refusals for `reports = true` or an artifact reader, `diagnostic_override` with the codes of the
   `[[admission_check]]` rows) are read from NJL-01's table and are not copied here.
   An action of an entry with two kinds gets the common properties, the scope and the appended refusals of the
   action's kind (NJL-01).
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose. A code whose registry row states another exit (FROB-REF-840, exit 5;
   `campaign-already-open`, exit 7) keeps its row's exit.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone). The early forms of
   this cluster: `eval.compare` `form: "cpu-referee"` (M1), `eval.run` and `eval.scorecard` `form: "referee"`
   (M1), `eval.rank-agreement` `form: "tier-pair"` (M3).
5. Pending refusals. A refusal that the plan names without a number is raised under its slug with the exit its
   registry row states (exit 3 unless stated), and is on no row's `refusals` list until the allocation ledger
   (N35-33, N35x-02) gives it a number; it enters the list in the same change, an additive snapshot change. The
   conformance test accepts a slug raised by a verb only while the slug is pending in the ledger.
6. An option of a row is spelled once, in this bead. A mechanism bead that implements or uses an option of a verb
   of this cluster states it in the row's form, and a script calls only options of the row (KNOWN_DRIFTS 8, 12).

**Notes for this cluster.**

- Handlers live in four crates: `frob-arena` (run, scorecard, preanalyze, qualify, ledger, items), `frob-opt`
  (compare, rank-agreement, promote), `frob-learn` (detect, and the policy path of `eval.run`) and `frob-dyn`
  (the pure pair comparison inside promotion, and the step-plan forwarding of N16bp2-01 reaches its step planner).
  `eval.promote` is `frob-opt`'s by the layer rule (BD-263: no estimator or scheduler lives in `frob-dyn`); an
  earlier draft gave it to `frob-dyn`.
- Three output schemas are named by the components that define the objects, as stored and cited objects of their
  own: `frob.qualify.receipt/1` (`eval.qualify`), `frob.scorecard/1` (`eval.scorecard`, N24a-22 and BD-254) and,
  embedded unchanged in `frob.eval.preanalyze.out/1` and in study plans, `frob.preanalysis/1` (N24b-06); a second
  name for the same object would give one schema two identifiers.
- `eval.compare` is the one-call form of the two-phase study: it plans and submits an anytime-valid paired study
  with defaults, or the study its block declares. `eval.preanalyze` is called by `study.plan` and by `eval.compare`
  for a fixed-n study.
- One situation, one code, one verb (N28-REF's layer rule and N35x-02's pair table). Through `eval.compare`, whose
  `pairing::admit` runs first, a mixed-type request is 553, an unreachable declared verdict 555 and an uncontrolled
  determinism class 542; through the Arena study path (`eval.qualify`, `study.plan`) the same defects are 552, 532
  and 533. 561 is the determinism licence's code (N35-07): on `eval.compare` it answers only what 542 does not
  cover (DESIGN, below); on `eval.detect` it answers a D4 or unrecorded arm; on `eval.promote` it answers only
  what 825 does not cover (DESIGN, below). 825 is the evidence boundary's code for a stored result of class D4 or
  unrecorded offered to promotion or qualification. 861 is the citation check's code and in this cluster is
  listed by `eval.scorecard` alone: the same training-time number offered to `eval.compare` as an arm is 542, to
  `eval.promote` or `eval.qualify` 825, and a D4-trained policy's result at `eval.detect` 867.
- `eval.qualify` returns a receipt and nothing per world; no scope in a discovery token can read sealed clusters
  before the campaign closes (`FROB-REF-840`). Its `--dry` form commits nothing and spends no level.
- `eval.rank-agreement` is the one registered name for the rank-agreement record, between controller rungs and
  between tier pairs.
- The unit-level batteries of the learning chapter are Gauntlet entries, not runs of `eval.run` (KNOWN_DRIFTS 1):
  `cortex-ppo`, `cortex-offpolicy`, `cortex-env` and `cortex-verdict` run as `FROB_TEST_SEED=<seed> cargo test -p
  frob-conformance --test g1_known_answers -- <row>`, and `cortex-grad` in the parity tier, `FROB_TEST_SEED=<seed>
  cargo test -p frob-conformance --test g2_parity -- <row>` (GATE-CORTEX-02 is of kind Parity; BD-196).
  `eval.run` evaluates a candidate on a battery
  of scenarios. The row therefore has no design-less conformance form and no `--rules`, `--backend` or `--impl`
  input.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly ten entries with `cluster = "eval"` (4 `read`, 6 `run`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster eval` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 10, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_eval.sh` runs each verb of the cluster (ten), and from
      a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/eval/<verb>.help.json` byte for byte for each of the
      ten, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified `breaking`
      and refused. The snapshots carry every input of the blocks above (for example `eval.run`'s `policy`,
      `run_seed`, `on_overflow`, `freeze_envelopes`; `eval.compare`'s `anytime`, `fixed_n`, `pairing`, `backend`;
      `eval.ledger`'s actions and `read_sealed`) and the added output members (`eval.scorecard`'s hazard counts,
      `eval.preanalyze`'s `cadence`).
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`; no chapter-28 code is listed on
      a verb that N28-REF's verbs column does not name (`verbs_lists_agree`), and every parameter marked `accepts =
      "verdict"` belongs to a row that lists 872 (N28c-12).
- [ ] `eval.ledger`'s actions `open`, `close` and `seal` require `qual:admin` (720, exit 5, without it), are
      selected only with `--action` (MCP `action`), take no key (`--key` is 802) and change nothing when repeated,
      and `--read-sealed` before close from a session without `qual:admin` is 840, exit 5.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_eval` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 10 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example `eval.items`
  is `read` at M3 with tier F, and `eval.rank-agreement` is `run` at M4 with tier F).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table (`eval.qualify` returns `frob.qualify.receipt/1`).
- `refusal_lists`: each row's chapter codes equal its block (`eval.compare`: 464, 502, 542, 543, 551, 553, 555, 556,
  561, 843, 890; `eval.detect`: 561, 867; `eval.ledger`: 840, 886; `eval.preanalyze`: 532; `eval.promote`: 171,
  246, 502, 548, 561, 811, 817, 820, 821, 825, 830, 871, 872, 890; `eval.qualify`: 502, 532, 533, 543, 552, 556, 567,
  817, 825, 840 to 845; `eval.rank-agreement`: 537, 545, 546, 548; `eval.run`: 103, 132, 222, 245, 246, 247, 248,
  421, 457, 464, 502, 522, 529, 531, 534, 601, 602, 604, 608, 610, 722, 726, 867; `eval.scorecard`: 502, 845, 861,
  865, 888, 890; `eval.items`: none); the appended codes equal the rule for its kind and fields (`eval.items` is
  paginated and also gets 806, 807 and, through its `int53` property `limit`, 808; every `run` row gets 808 through
  `sync_wait_ms`; `eval.ledger`'s `--file` and `eval.preanalyze`'s `--plan` get 734 and 828; each action gets the
  codes of its kind).
- `pending_slugs_uncoded`: each pending slug of the blocks above is raised under its slug with its row's exit code
  and appears on no row's `refusals` list; a planted slug on a list fails, and so does a planted number for a slug
  that the ledger still marks pending.
- `mcp_names_one_to_one`: the tool names are the mapped names (`eval_compare`, `eval_detect`, `eval_ledger`,
  `eval_preanalyze`, `eval_promote`, `eval_qualify`, `eval_rank_agreement`, `eval_run`, `eval_scorecard`,
  `eval_items`), with no collision against any other cluster.
- `owners_and_options`: `eval.promote`'s owner is `frob-opt` and its `result` parameter is marked `accepts =
  "verdict"`; `eval.scorecard`'s output schema is `frob.scorecard/1`; the generated `eval.run` input schema has
  `freeze_envelopes`, `policy`, `run_seed`, `on_overflow` (three values, default `refuse`), `determinism`,
  `precision`, `on_degraded`, `state`, `shield`, `shield_budget`, `thermal_start`, `solve_queues`, `telemetry`
  (upper-case levels; `t1` is 802) and `randomization` (`fixed`, `adaptive`, `training-distribution`; default
  `fixed`), `no_memo` (generated from `memo = true`), `conformance_plant` only in a conformance build's schema, and no `seed`, `workers`, `rules` or `impl`;
  `--battery smoke --run-seed 17` is 802 with pointer `/run_seed`; `eval.compare`'s has `anytime` and `fixed_n` (at
  most one), `pairing`, `backend`, `sampling`, `claim` and `no_memo`, and `--run-seed` there is 802;
  `eval.promote`'s `law` accepts `CL-1` and `CL-H` and refuses `cl-1` with 802; `eval.qualify`'s has `design`,
  `task`, `claim`, `mission`, `dry` and `sampling`; `eval.scorecard`'s has `quantile`; `eval.items`'s has
  `comparison`; `eval.preanalyze`'s has the fixed-n, cadence and plan inputs, and a request mixing two modes is 802.
- `ledger_actions`: `eval.ledger` keeps `kind = "read"` and declares exactly the actions `open`, `close` and `seal`,
  each of kind `admin` with capabilities `["eval:read", "qual:admin"]`; an action call without `qual:admin` is 720,
  exit 5; the generated schema has no `idempotency_key` and `--key` with any form is 802; `--open`, `--close` and
  `--seal` are 802 (no per-action flag); `name` and `file` belong to `seal` alone, and `name` with `open` is 802;
  `read_sealed` is an input of the read form, not an action.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties (`sync_wait_ms`, `idempotency_key`, `budget`,
  `estimate_only`, `on_cancel`, `envelope_scope`), with `idempotency_key` outside the schema's `required` array and
  annotated `x-frob-key: "required"`, and refuses `sync_wait_ms = 30001` with code 802; a verb whose capability is
  absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_eval.sh` (header: `bead=VERBS-eval milestone=M1 labels=tether,verbs,verb:eval.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance eval`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the ten verbs with kind and milestone, and the early forms named in convention 4 from
their milestones; (2) `frob-surface-diff parity --cluster eval` over stdio and through `frobd --mcp`, every
difference list empty; (3) `frob-surface-diff snapshot --check --cluster eval`; (4) each verb's registry example
verbatim, expecting a result (exit 0 or 2), a job handle (exit 8, then `frob job wait`) or `FROB-REF-724` exactly
when the capability report says the verb is absent; (5) for each implemented verb the same call as an MCP tool, the
two envelopes agreeing on `verb`, `ok`, `exit`, `result`, `refusal`, `evidence`, `bound`, `warnings` and
`next_actions`; (6) per verb one call with a misspelt parameter (`e2e_expect_refusal FROB-REF-802 3`) and, for
`run` and `mutate` verbs, one without a key (`e2e_expect_refusal FROB-REF-805 3`); (7) one envelope per call and no
exit code 1; (8) from M3, `frob eval ledger --action open --kind battery --label verbs-e2e --json` from a
session without `qual:admin` (`e2e_expect_refusal FROB-REF-720 5`), and the same action as the MCP property
`action: "open"` with the same answer.

Log: `target/e2e-logs/verbs_eval/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 10 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 10 verbs (4 `read`, 6 `run`); the tables above. KA-755 (the exit-code table on
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

- Open naming decision (owner decision recorded with bead N16b-27): the Kinesis chapter's verb table also prints
  `fidelity.rank` for the tier-pair measurement. Fail-closed default: only `eval.rank-agreement` is registered,
  accepting a rung pair or a tier pair.
- Every study a verb of this cluster submits is declared (anytime-valid or fixed-n, never both); a request that
  mixes types is refused, with 553 on the comparison path (`eval.compare`) and 552 on the Arena study path
  (`eval.qualify`). `eval.run`, `eval.detect` and `eval.rank-agreement` submit no study and take no study block.
- DESIGN (542 and 561 on `eval.compare`). The plan prints both for a D4 arm in a paired comparison (§28.20 and the
  class-licence table of §35.3). N28-REF's layer rule, N35x-02's pair table ("542: the comparison verb") and the
  device form BD-262 (which asserts 542 for a D4 arm) give the comparison verb 542, raised by `pairing::admit`,
  which runs first. The licence check of N35-07 runs after it and keeps 561 for what 542 does not cover. Reason: one
  situation gets one code on one verb, and the licence's own cases (an unrecorded class, a cross-family or D3 pair
  demanded `exact`) stay reachable. N35-07's division table, N37-28's table of paths, N35-REF's row for the codes
  561, 542, 533 and 825, and N37-REF's 825 row state the same division.
- DESIGN (825 and 561 on `eval.promote`). The plan gives a result of class D4 offered to promotion to the evidence
  boundary (§37.11, 825) and a D4 arm of a paired statistic to the numerics licence (§35.3, 561). N37-28's check
  runs in the dispatcher before the handler, so a stored result of class D4 or unrecorded offered as `result` is
  825 and the handler's licence check never sees it. The licence keeps 561 for what the boundary does not answer:
  the fast-tier result and its higher-tier re-evaluation paired across device families without class D1, and arms
  that cannot be paired (a solve-path mismatch the request did not declare, BD-263). Reason: one situation gets
  one code on one verb, and the code is that of the site that raises it first.
- DESIGN (the battery form without a design). Five beads run `frob eval run --battery standard --class <c>` with no
  design (N16a-15, N16a-20, N16b-03, N16b-31, BD-213). The row admits it, running the class's reference robots from
  the asset tree and recording them, which is how N24b-01 already resolves the design list of `smoke`. Reason: the
  fidelity battery is a property of a class, and naming one design in five scripts would measure one robot where the
  gates speak of the class.
- DESIGN (`eval.promote --adjudicator` and `--law`). BD-263 chooses the adjudicator automatically and N16c-09 must
  promote 200 results under each adjudicator law. The default `auto` keeps BD-263's behaviour, and the two inputs
  make the comparison requestable without a second verb. BD-263 takes both in `frob.eval.promote.in/1`.
- DESIGN (`eval.qualify --mission`). The written session of §32.16 calls `frob eval qualify carrier-b --mission
  q:…`; N32b-16's script uses the explicit `--task`, `--claim` and `--campaign`, which stay valid. The mission form
  resolves to the same three, so the receipt and the commitment are the same.
- Inputs the rows do not have, with the reason: a design-less conformance-battery form of `eval.run` with `--rules`,
  `--backend` and `--impl` (KNOWN_DRIFTS 1: unit-level batteries are Gauntlet entries run with `cargo test`; BD-192,
  BD-197, N23a-07, N23a-10 and N36-08 already do so); an operator-only `--workers` on `eval.run` (N24a-24 states
  that `eval.run` has none and the worker count comes from the pool configuration; no script calls it); an option
  that runs part of a battery (a shortened battery is a different population with a new `BatteryId`, GATE-ARENA-07,
  INV-ARENA-03); `--seed` as a second spelling of `--run-seed`; a `test` value of `--stream` (N23b-09 defines `dev`
  only; held-out evaluation is `--split held-out`).
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M2, M3, M4. The
  per-milestone surface beads (BD-137, BD-222, BD-312, BD-376) re-run this script as verbs land; that is how
  GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`read` → `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` →
  `<cluster>:write` or a named write scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope),
  plus `export:*` when the row exports and `private:*` where privacy applies, never a scope named after the
  object read or the engine spent on; N32b-01's catalogue generates the forms. `qual:run` and `qual:admin` are the
  named scopes of the qualification lane, and `fidelity:admin` the named scope of `--freeze-envelopes`.
  `<cluster>:run` and `<cluster>:write` imply `<cluster>:read`, closed when a set is stored (at minting and at
  derivation, `Scopes::close_implied`), so checks stay plain set inclusion.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §24.8, §24.12, §24.13,
§24.21, §16.8, §22.3, §23.16, §28.6, §28.10, §28.20, §35.3, §32.4, §32.6, §32.12.3, §32.16, §32.18, §32.21;
GATE-TETHER-02; INV-TETHER-01, -02; KA-755; N28-REF (layer rule, `verbs_lists_agree`); N35x-02 (neighbour pairs,
pending slugs); KNOWN_DRIFTS 1, 5, 6, 8, 12.
