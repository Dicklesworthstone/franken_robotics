---
key: "VERBS-shield"
title: "frob-verbs registry: cluster `shield`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "reflex", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-26"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§22.11", "§22.12", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `shield`. The cluster covers the energy-budget safety tier and
runtime shields: building and admitting an energy budget for a boundary, replaying a run against a budget or shield,
fitting a shield with a backup controller, and the shield report. It has four verbs (1 `read`, 3 `run`); the
handlers live in `frob-shield`. The bead carries every row completely, with the three checks that keep the rows
true: the generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema
printed by `frob <verb> --help --json` is stable.

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

In: the four registry entries of cluster `shield`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `shield.budget` | run | M3 | F | Build and admit an energy budget for a design and a boundary | §22.11 |
| `shield.check` | run | M3 | F | Replay a run against a budget or a shield | §22.11, §22.12 |
| `shield.fit` | run | M4 | F | Rows, measured margins, backup and monitors for a design, a controller and a task family | §22.12 |
| `shield.report` | read | M4 | F | Interventions, alarms, the violation bound and the return kept | §22.12 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *run*: class Tested at best, by the claim product of the run; the tier of the run, `provisional` on a fast tier
  until promoted to the referee; the determinism class of the placement (D2 unless stated); `reality: no-claim`.
- *screening*: class Estimated: a screening or planning signal that may discard under the ladder's rules and may not
  establish a comparison.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`shield.budget`** (MCP tool `shield_budget`; owner `frob-shield`)
- Inputs (BD-337, which fixes the spellings): design (positional); `--boundary <id>` (a boundary the design
  declares; its absorber card, stroke, approach direction and guide or clearance basis come with it);
  `--absorber <card>` (an absorber card in place of the boundary's own, N22bx-01); `--cutoff-delay <s>` (a
  declared cutoff delay in place of the derived chain of N22b-11, with provenance `declared`); `--load-path
  <file>` (optional; derived by the port-graph walk when absent); `--energy-step <J>` and `--time-step <s>` (the
  unit scales, default 10⁻⁶ J and 10⁻⁶ s); `--reserve <J>` (the declared reserve); `--class <class>` (the
  requested class; default what the inputs support); `--reading-max <m/s>` (the upper end of the speed reading
  domain; the speed bound adds the declared reading error to it); `--key <k>`.
- Outputs: an admitted `.fenergy` budget (`frob.energy/1`) with the integer predicate, the margin and the longest
  admissible cutoff delay. Evidence: *certificate*.
- Chapter refusals: `FROB-REF-451` (an energy budget omits a source in the port graph, or a class above `Estimated`
  is requested on an `assumed` absorber envelope).
- Implemented by: BD-337. A late row of NJL-02's `LATE_ROWS` (M3; BD-337, M4): in an M3 build it answers
  `FROB-REF-724`, which the harness accepts for it only because the row is in that closed list.

**`shield.check`** (MCP tool `shield_check`; owner `frob-shield`)
- Inputs: a run (positional) and exactly one of `--energy-budget <EnergyBudgetId | .fenergy>` (a stored energy
  budget or its file: the budget half, BD-337; the verb's own energy budget, not the generated compute `budget`,
  see Considerations), `--shield <ShieldId>` (the shield half, BD-338) or `--instance <file>` (one projection
  instance checked through the verb surface: the applied command and multiplier against the independent
  reference, N22b-32); `--key <k>`.
- Outputs: the replay of the run against the budget or shield: `holds`, or the first offending tick with recomputed
  and recorded values; exceedances (none allowed), interventions, the margin trace; from N22c-28 a `late-alarm`
  finding with its distance to the backup's domain. Evidence: *run* (class `Tested` or weaker, INV-SHIELD-02).
- Chapter refusals: `FROB-REF-451` (with `--energy-budget <file>`: a `.fenergy` document read against its absorber
  card has a `w_abs_j` above the card's recomputed W_abs; `pointer` `absorber.w_abs_j`, `observed` the copy,
  `required` the recomputed value; first repair: rebuild the budget with `shield.budget`; N22bx-01, as N22-REF's
  row lists `shield.check` among the raising verbs); `FROB-REF-457` (a shield margin is used outside its scope
  key).
- Implemented by: BD-338 (shield half), BD-337 (budget half). Extended by: N22c-28 (`late-alarm`). A late row of
  NJL-02's `LATE_ROWS` (M3; BD-338, M4): in an M3 build it answers `FROB-REF-724`, accepted for it only through
  that list.

**`shield.fit`** (MCP tool `shield_fit`; owner `frob-shield`)
- Inputs (BD-338, which fixes the spellings): design (positional); `--controller <id>` (a controller or policy
  identity); `--tasks <family>` (the task family, whose declaration supplies the hard limits to enforce:
  keep-out regions, speed limits, door-pinch zones, balance region); `--tier <tier>`; `--energy-budget
  <EnergyBudgetId>` (optional; the verb's own energy budget, see Considerations); `--calibration <N>` (the
  calibration count, N22b-29); `--reuse-margin <MarginId>` (a stored calibration offered for reuse);
  `--listing <file|id>` (the monitored-assumption listing, N22c-28);
  `--backup <kind>` with repeated `--param <name>=<value>` (for example `--backup damping --param kd=300 --param
  h=0.010`, N22c-44); `--allow-reversal` (admit gains above the no-reversal bound up to the passivity bound,
  recorded in the shield record, N22c-44); `--table <subsystem description>` (N22b-35, M8: the `params` entry
  `table = "artifact-input<subsystem>"`, a file of format `frob.shield.subsystem/1`, the canonical-JSON
  description members of N12-19's `tube-inclusion` payload; it builds the reachable-set table of N22b-34 from that
  file and attaches it as a payload of the shield record, whose `ShieldId` then covers its digest; after KILL-TUBE
  fires with `killed: fallback adopted`, N22b-36, the option is refused with `FROB-REF-724`, exit 5, and the rest
  of the verb is unchanged); `--key <k>`. α is not an input: it, the violation budget and the confidence level are
  read only from the OPEN-SHIELD-2 entry of `registries/shield_levels.toml` (N22b-27), so a
  second definition cannot arise, and the output echoes them with their source.
- Outputs: `ShieldId`; shield rows with their three-part margins and sources; the measured margin with n and α;
  the backup with its domain and sampled bound; the monitors; findings; the levels in force (α, the violation
  budget, the confidence level and their source, read from `registries/shield_levels.toml` of N22b-27).
  Evidence: *run* (class `Tested`: a statistically measured margin, never `Proved`; INV-SHIELD-02). Corrected in
  round 1 from *screening*, which is class `Estimated` and disagreed with BD-338 and N22b-32.
- Chapter refusals: `FROB-REF-454` (a shield's rows have no common admissible command on the calibration set, or
  more than four are coupled); `FROB-REF-457` (a margin offered for reuse, or found stored, under a scope key that
  differs from the request's in any component: BD-338 step 4; N22b-32 step 7 exercises it through `frob shield
  fit --reuse-margin`).
- Pending, uncoded (raised with exit 3 under their slugs and on no row's list until the allocation ledger gives
  them numbers): `monitored-assumption-not-timely` (N22c-28), `sampled-damping-above-passivity-bound` (N22c-44),
  and `insufficient-calibration` (the calibration count is below (1 − α)/α, with the needed count; N22b-29,
  N22b-32 step 14). N22-REF states the three rows (meaning, payload, repairs, exit 3, `shield.fit` as the raising
  verb), and the allocation ledger (N35x-02) holds them with no proposal and no interim code.
- Implemented by: BD-338. Extended by: N22c-28 (the timeliness rule), N22c-44 (the sampled-damping bound and
  `--allow-reversal`), N22b-35 (`--table`, M8).

**`shield.report`** (MCP tool `shield_report`; owner `frob-shield`)
- Inputs (BD-340): a `ShieldId`; optionally a run (`--run`), a batch of runs or a campaign identity to restrict
  the account.
- Outputs: interventions, alarms, the violation bound and the return kept; for a shield whose record carries a
  reachable-set table (N22b-35, M8), the table block: kept-cell count per mode, coverage, the fraction of ticks
  answered `uncovered`, and the reference of the `tube-inclusion` certificate. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-457` (a shield margin is used outside its scope key).
- Implemented by: BD-340. Extended by: N22b-35 (the table block).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `shield.budget` | `shield:run` | `frob.shield.budget.out/1` | `shield` | `frob shield budget quad-tray --boundary pass_counter_stop --key sb-1 --json` |
| `shield.check` | `shield:run` | `frob.shield.check.out/1` | `shield` | `frob shield check r:19e2 --energy-budget pass_counter_stop.fenergy --key shc-1 --json` |
| `shield.fit` | `shield:run` | `frob.shield.fit.out/1` | `shield` | `frob shield fit quad-tray --controller c:2be8 --tasks kitchen --key sf-1 --json` |
| `shield.report` | `shield:read` | `frob.shield.report.out/1` | `shield` | `frob shield report sh:4 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "shield.budget"
cluster = "shield"
kind = "run"
tier = "F"
capabilities = ["shield:run"]
input_schema = "frob.shield.budget.in/1"
output_schema = "frob.shield.budget.out/1"
refusals = ["FROB-REF-451"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["shield"]
examples = ["frob shield budget quad-tray --boundary pass_counter_stop --key sb-1 --json"]
owner = "frob-shield"
purpose = "Build and admit an energy budget for a design and a boundary"
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
   compute envelope in force; N36-20's `admit_run`), 803 (key reused with different content) and 805 (missing
   key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. What a field or a registry table adds
   follows NJL-01's variation table, which is the one list of what the generator adds: 806 and 807 for
   `paginated = true`; 808 for an entry with an `int53` input at any depth, whatever its kind (every `run` entry
   has one, `sync_wait_ms`); a `key_optional` entry is not given 805; a non-`run` entry with `estimable = true`
   gets `estimate_only`; 790 for a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a
   parameter that accepts a path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`). Its
   later variations, 10 to 16 (`override` with `override_reason`, `from_proposal`, `machine`, `engine`,
   `no_memo`, the reality guard's seven codes, and `diagnostic_override` with the admission checks' codes), are
   read from that table and not copied here. In this cluster the path parameters are `shield.budget`'s `--load-path`,
   `shield.check`'s `--energy-budget <.fenergy>` and `--instance`, and `shield.fit`'s `--listing` given as a file
   and `--table` (`table = "artifact-input<subsystem>"`, format `frob.shield.subsystem/1`, N22b-35), `params`
   entries of kind `artifact-input<F>` for the format each holds, so those three rows get 891, 734 and 828;
   `shield.report` takes identifiers only.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M3); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `shield.budget` walks the design's port graph and refuses a budget with an unlisted energy source
  (`FROB-REF-451`); the admission predicate is an exact-integer inequality that the offline auditor recomputes.
- The budget answers an energy question. It supports no statement about peak force, clamping after the stop or
  contacts that bypass the absorber, and every artifact of the cluster carries the no-authorization flags.
- The inventory gives `shield.budget` and `shield.check` M3 and `shield.fit` and `shield.report` M4; the seed beads
  of all four are at M4 (BD-337, BD-338, BD-340), and GATE-SHIELD-01 is due at M4 in the gate register. The rows
  keep the appendix's milestones; `shield.budget` and `shield.check` answer `FROB-REF-724` in an M3 build and
  answer from the build that carries BD-337 and BD-338. They are the two shield entries of NJL-02's closed list
  `LATE_ROWS` (each with its implementing bead, `lands = "M4"` and the reason), which is the only ground on which
  the harness accepts their 724 past the rows' first milestone; a 724 from any other shield row past its
  milestone fails GATE-TETHER-02.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "shield"` (1 `read`, 3 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster shield` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_shield.sh` runs each verb of the cluster (four), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/shield/<verb>.help.json` byte for byte for each of
      the four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The generated input schemas carry the spellings of BD-337, BD-338 and N22c-44 (`shield.budget`: `boundary`,
      `absorber`, `cutoff_delay`, `load_path`, `energy_step`, `time_step`, `reserve`, `class`, `reading_max`;
      `shield.check`: exactly one of `energy_budget`, `shield`, `instance`; `shield.fit`: `controller`, `tasks`,
      `tier`, `energy_budget`, `calibration`, `reuse_margin`, `listing`, `backup`, `param`, `allow_reversal`,
      `table`), no
      `alpha` input, and no row input that shadows its kind's common properties (`budget` stays the compute
      budget); `shield.budget` and `shield.check` are in `LATE_ROWS` with `lands = "M4"`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_shield` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `shield.report` is `read` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`shield.budget`: 451; `shield.check`: 451, 457;
  `shield.fit`: 454, 457; `shield.report`: 457); the appended codes equal the rule for its kind and fields
  (NJL-01): 761 on the three `run` rows, and 891, 734 and 828 on `shield.budget`, `shield.check` and `shield.fit`
  (their file inputs), none on `shield.report`; the three pending slugs of `shield.fit` are on no row's list.
- `input_spellings`: the input schemas hold the spellings of the acceptance criterion; `shield.fit`
  has no `alpha` property (α is read from `registries/shield_levels.toml`); `shield.check` with two of
  `--energy-budget`, `--shield`, `--instance`, or none, is 802; on `shield.check` and `shield.fit` the property
  `budget` is the generated compute budget and `energy_budget` the verb's own, and a planted row that names its
  own input `budget` fails a shadowing check over the cluster; `shield.fit --table` is a `params` entry (a path:
  734 and 828 on the row once), and with KILL-TUBE's fallback recorded it is 724, exit 5, while `shield.fit`
  without it still answers.
- `late_rows_listed`: NJL-02's `LATE_ROWS` holds `shield.budget` (BD-337, M4) and `shield.check` (BD-338, M4)
  and no other shield row.
- `evidence_words`: `shield.fit`, `shield.check` are *run* (class `Tested` or weaker); no row of the cluster is
  labelled with a class above `Tested` except `shield.budget`'s *certificate*.
- `mcp_names_one_to_one`: the tool names are the mapped names (`shield_budget`, `shield_check`, `shield_fit`,
  `shield_report`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of convention 2 and refuses `sync_wait_ms = 30001`
  with code 802; a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_shield.sh` (header: `bead=VERBS-shield milestone=M3 labels=tether,verbs,verb:shield.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance shield`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
shield` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster shield`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_shield/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 4 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 4 verbs (1 `read`, 3 `run`); the tables above. KA-755 (the exit-code table on
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

- The energy-budget input is `energy_budget` (`--energy-budget`), not `budget` (DESIGN). Every `run` row gets the
  generated common property `budget`, the compute budget (`--budget gpu_s=…,cpu_core_s=…` on the CLI, NJL-01),
  and BD-337 and BD-338 spelled the energy budget of `shield.check` and `shield.fit` `--budget` too. One input
  schema cannot hold two properties of one name, and on the CLI the two would be told apart only by the grammar
  of their values, so the row names the verb's own input `energy_budget`; every other spelling of BD-337, BD-338
  and N22c-44 is recorded unchanged. BD-337, BD-338 and the scripts that call `frob shield check <run>
  --energy-budget <id>` (N22b-16, N22b-18, ART-L5, NS-23, N34p1-03) use that spelling, and NJL-01's lint
  `common-property-shadowed` fails a row whose own input is named `budget`. Rejected: keeping `--budget` with a
  type union (an agent reading the schema could not tell which budget it sets).
- Kill gate KILL-ENERGY (an exceedance under the premises survives one repair cycle at the energy-tier gate):
  fallback, the claim is withdrawn and the monitor remains as an alarm of class Estimated; the rows stay registered.
- Closing rule: the bead closes at M3, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M3, M4. The per-milestone
  surface beads (BD-312, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`read` → `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` →
  `<cluster>:write`; `admin` → `<cluster>:admin` or a named admin scope), never a scope named after the object
  read; N32b-01's catalogue generates the forms, and `<cluster>:run` and `<cluster>:write` imply `<cluster>:read`
  (`Scopes::close_implied`).
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §22.11, §22.12, §32.4, §32.6, §32.12.3, §32.18,
§32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755; BD-337, BD-338, N22c-44 (spellings); N22b-27 (α from
OPEN-SHIELD-2); NJL-01 (appended codes), NJL-02 (`LATE_ROWS`), NJL-04, NJL-05.
