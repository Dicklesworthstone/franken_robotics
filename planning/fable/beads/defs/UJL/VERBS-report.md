---
key: "VERBS-report"
title: "frob-verbs registry: cluster `report`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "atelier", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-26"
deps: ["BD-022", "N16-REF", "N18-REF", "N19-REF", "N20-REF", "N24-REF", "N27-REF", "N28-REF", "N29-REF", "N35-REF", "N37-REF", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§28.18", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `report`. The cluster covers evidence packages: of a design, a
run, a mission answer and a campaign, each with its claims, classes, weakest inputs and permitted wording. It has
four verbs (4 `read`); the handlers live in `frob-opt`. The bead carries every row completely, with the three checks
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

In: the four registry entries of cluster `report`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `report.campaign` | read | M4 | S | A campaign's modes, fallbacks, audits and phase ledger | §28.18 |
| `report.design` | read | M3 | S | The evidence package of a design | §28.18 |
| `report.run` | read | M3 | S | The evidence package of a run | §28.18, §32.6 |
| `report.mission` | read | M3 | S | The evidence package of a mission answer | §28.18, §32.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`report.campaign`** (MCP tool `report_campaign`; owner `frob-opt`)
- Inputs: a campaign or a registered comparison (positional); optional `--cite <id>` (a parameter marked
  `accepts = "verdict"`), `--compact`, `--claims-realism` (N19a-39); `--arms B,W,F` with `--stratum`, `--regime
  cold|warm`, `--ablate information|controller|stepping`, and `--register-comparison <manifest.json>` (N39b-46;
  registration writes one content-addressed graph node and moves no named object, as `phylum.map --merge` does).
- Outputs: the campaign's modes (every rung and stratum), fallbacks fired, the screening audit, the planted-winner
  outcome, stage controls and the phase ledger; the three named reuse columns where recorded; finalists chosen
  under a mode other than `selective` described as "k survivors"; a losing campaign reported as completely as a
  winning one. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-312` (a realism claim cites a scene with `assumed` dimensions); `FROB-REF-557` (a
  `model-conditional` ranking shown as a design comparison without its condition, for example `--compact`
  dropping the condition columns; a `model-conditional` result used for a build packet is `FROB-REF-830`,
  raised on the build-packet and promotion path through N16b-04's check, by N28-REF's one-code precedence, and
  is not on this row); `FROB-REF-819` (a statement or citation that would state a result above its claim product
  where no more specific code applies: the report generator's call to the evidence boundary, N37-26 step 6;
  under `--claims-realism`, a result whose weakest input is an `assumed` interface, film or soil card field);
  `FROB-REF-820` (a non-`established` result cited
  as established); `FROB-REF-822` (a `capacity-biased` comparison cited as a verdict, through `--cite` or a
  statement: N16b-04's citation check, the capacity-bias rule of N18b-22); `FROB-REF-845` (a result cited as
  qualified without a promoting receipt); `FROB-REF-862` (a
  performance figure without a complete disclosure block); `FROB-REF-863` (a performance ratio without a paired
  lower bound); `FROB-REF-865` (a summary statistic that omits what it excluded); `FROB-REF-869` (citing a claim
  whose evidence closure is dangling or tombstoned); `FROB-REF-870` (a verdict presented after an edit without a
  grade and a licence: BD-347's consumer check, wired on this verb by BD-352); `FROB-REF-872` (a
  value-of-information output cited as a verdict through `--cite` or a statement); `FROB-REF-880` (an Autopsy
  attribution cited as an established cause or as a cause in the physical robot: N27b-15's citation guard).
- Held, uncoded: `reuse-figure-without-column` (N28c-44: a reuse figure printed without its column name), a
  request of the second allocation round (N35x-02) in the family `8xx`, whose requests are `held` (N35x-01). The
  raising bead uses the ledger row's interim form, and the slug is on no row's list until the row is `assigned`
  (N35-33 rule 7).
- Implemented by: BD-352, BD-302.
- Extended by: N28c-44 (the three named reuse columns), N39b-46 (`--arms`, `--ablate`, `--register-comparison`;
  862, 863, 865), N28c-12 (the advisory type wall, 872), N19a-39 (`--claims-realism`, 312), BD-347 and BD-352
  (M4: 870), N27b-15 (880).

**`report.design`** (MCP tool `report_design`; owner `frob-opt`)
- Inputs: design (positional); optional `--cite <id>`, `--claims-realism`.
- Outputs: the evidence package of a design: the design and its lineage, claims with their classes, weakest
  inputs and permitted wording; a `model-conditional` result printed with its condition and both orders of the
  ranking; a statement whose closure does not resolve printed as `evidence: unavailable` with no number.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-312` (a realism claim cites a scene with `assumed` dimensions); `FROB-REF-557` (a
  `model-conditional` result offered as a design comparison without its condition); `FROB-REF-816` (the
  package's manifest lacks the three no-authorization flags: the export door's first check); `FROB-REF-731` (a private
  object, or anything derived from one, without the private scope and flag); `FROB-REF-733` (the licence set of
  the artifact forbids this export); `FROB-REF-819` (a result cited above its claim product); `FROB-REF-820` (a
  non-`established` result cited as established); `FROB-REF-822` (a `capacity-biased` comparison cited as a
  verdict; N18b-22's script asks `report.design` to cite one and expects 822); `FROB-REF-845` (a result cited as
  qualified without a promoting receipt); `FROB-REF-869` (citing a claim whose evidence closure is dangling or
  tombstoned); `FROB-REF-870` (a verdict presented after an edit without a grade and a licence: BD-347's
  consumer check, `report_design_unlicensed_870`); `FROB-REF-872` (a value-of-information output cited as a
  verdict); `FROB-REF-880` (an Autopsy attribution cited as an established cause or as a cause in the physical
  robot: N27b-15's citation guard).
- Implemented by: BD-302; the export refusals are raised by the export door (N37-18, BD-305). Extended by:
  N28c-12 (872), N19a-39 (312), BD-347 (M4: 870), N27b-15 (880).

**`report.run`** (MCP tool `report_run`; owner `frob-opt`)
- Inputs: run (positional); optional `--cite <claim id>`, `--claim reality|performance` (the claim the report is
  asked to make; a Reality claim is routed through the sensing guard of N20b-18), `--claims-realism`.
- Outputs: the evidence package of a run: identity and model, tier and profile, and its claims with their wording.
  Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-312` (a realism claim cites a scene with `assumed` dimensions); `FROB-REF-447` (a
  Reality claim rests on a sensor or optical card field of class `assumed`); `FROB-REF-816` (the package's
  manifest lacks the three no-authorization flags: the export door's first check); `FROB-REF-731` (a private object, or
  anything derived from one, without the private scope and flag); `FROB-REF-733` (the licence set of the artifact
  forbids this export); `FROB-REF-819` (a result cited above its claim product); `FROB-REF-820` (a
  non-`established` result cited as established); `FROB-REF-822` (a `capacity-biased` comparison cited as a
  verdict); `FROB-REF-845` (a result cited as qualified without a promoting receipt); `FROB-REF-869` (citing a
  claim whose evidence closure is dangling or tombstoned); `FROB-REF-872` (a value-of-information output cited as
  a verdict); `FROB-REF-880` (an Autopsy attribution cited as an established cause, or, under `--claim
  reality`, as a cause in the physical robot: N27b-15's citation guard).
- Implemented by: BD-302; the export refusals are raised by the export door (N37-18, BD-305). Extended by:
  N20b-18 (`--claim reality`, 447), N28c-12 (872), N19a-39 (312), N27b-15 (880).

**`report.mission`** (MCP tool `report_mission`; owner `frob-opt`)
- Inputs: mission answer (positional); optional `--cite <id>`, `--claims-realism`, `--publish` (the
  published-report path, N19b-43).
- Outputs: the evidence package of a mission answer: the canonical query and the answer with its proofs and
  explanations; no unresolved cell printed as a verdict. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-312` (a realism claim cites a scene with `assumed` dimensions); `FROB-REF-816` (the
  package's manifest lacks the three no-authorization flags: the export door's first check); `FROB-REF-731` (a
  private object, or anything derived from one, without the private scope and flag); `FROB-REF-733` (the licence
  set of the artifact forbids this export); `FROB-REF-819` (a result cited above its claim product);
  `FROB-REF-820` (a non-`established` result cited as established); `FROB-REF-822` (a `capacity-biased`
  comparison cited as a verdict); `FROB-REF-845` (a result cited as qualified without a promoting receipt);
  `FROB-REF-869` (citing a claim whose evidence closure is dangling or tombstoned); `FROB-REF-870` (a verdict
  presented after an edit without a grade and a licence, an amended mission's answer shown without
  revalidation: BD-347's consumer check, `report_mission_unlicensed_870`); `FROB-REF-872` (a
  value-of-information output cited as a verdict); `FROB-REF-880` (an Autopsy attribution cited as an
  established cause or as a cause in the physical robot: N27b-15's citation guard); and the worded refusal
  `retrofit-cost-on-assumed-prices` (row 9 of N19c-42: family `3xx`, exit 3, retry `after-change`, raised through
  the constant `RETROFIT_COST_ASSUMED_PRICES`), raised under `--publish` by N19b-43 when a cost comparison in the
  answer's lineage rests on an `assumed` price (publishing counts as strict data for cost claims). The row
  cites it by its constant; its number is the one the refusal registry allocates for row 9, entered in the
  row's `refusals` when that registry row lands, as other worded refusals are.
- Implemented by: BD-302; the export refusals are raised by the export door (N37-18, BD-305). Extended by:
  N19b-43 (`--publish`), N28c-12 (872), N19a-39 (312), BD-347 (M4: 870), N27b-15 (880).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `report.campaign` | `report:read` | `frob.report.campaign.out/1` | `ladder`, `explain` | `frob report campaign camp:12 --json` |
| `report.design` | `report:read`, `export:report` | `frob.report.design.out/1` | `qualify`, `make`, `explain`, `structure` | `frob report design quad-tray --json` |
| `report.run` | `report:read`, `export:report` | `frob.report.run.out/1` | `reality`, `explain` | `frob report run r:19e2 --json` |
| `report.mission` | `report:read`, `export:report` | `frob.report.mission.out/1` | `explain` | `frob report mission a:8b27 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "report.campaign"
cluster = "report"
kind = "read"
tier = "S"
capabilities = ["report:read"]
input_schema = "frob.report.campaign.in/1"
output_schema = "frob.report.campaign.out/1"
refusals = ["FROB-REF-312", "FROB-REF-557", "FROB-REF-819", "FROB-REF-820", "FROB-REF-822", "FROB-REF-845", "FROB-REF-862", "FROB-REF-863", "FROB-REF-865", "FROB-REF-869", "FROB-REF-870", "FROB-REF-872", "FROB-REF-880"]
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M4"
task_clusters = ["ladder", "explain"]
examples = ["frob report campaign camp:12 --json"]
owner = "frob-opt"
purpose = "A campaign's modes, fallbacks, audits and phase ledger"
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them. `run`:
   `sync_wait_ms` (from 0 to the row's `wait.mcp_max_ms`, at most 30,000; MCP default 5,000; CLI default to
   completion), `idempotency_key` (CLI `--key`), `budget`, `estimate_only` (CLI `--estimate`), `on_cancel`
   (`leave`, the default, or `cancel_job`) and `envelope_scope` (CLI `--envelope-scope
   <gate:…|campaign:…|pretrain:…>`, the approved compute-envelope scope the run belongs to); `mutate`:
   `idempotency_key` and `expected_parent`; `read` and `admin`: none. `idempotency_key` is in no JSON-schema
   `required` array: the annotation `x-frob-key: "required"` carries the rule, and the admission transaction
   refuses a missing key with 805 on both surfaces. The generator appends the surface-wide refusals by kind:
   `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force; N36-20's `admit_run`), 803 (key reused with different content) and 805 (missing
   key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. What fields and registry tables add is
   NJL-01's variation table, the one list of it: 806 and 807, with `cursor`, `limit` and `follow`, for a
   paginated read (variation 1); `estimate_only` on a non-`run` entry with `estimable = true` (2);
   `x-frob-key: "optional"` and no 805 on a `key_optional` entry (3); 808 for an entry with an `int53` property
   at any depth of its input schema, whatever its kind, so every `run` row through `sync_wait_ms` (4); `action`
   for an `actions` list (5); 790 for a non-empty `needs_groups` (6); 891 to 893 for artifact `params` (7, 8);
   734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`, `artifact-identity<F>`,
   `foreign-file`) (9). Variations 10 to 13 are read from NJL-01's table with the beads that land them. In this
   cluster the path parameter is `report.campaign`'s `--register-comparison <manifest.json>`, so that row gets
   891, 734 and 828; the other inputs are identifiers.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M3); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Refusal lists (round 1, requested by U28c and checked against BD-302, N28-REF, N28c-12, N16-REF, N24-REF,
  N19-REF, N20-REF and N39b-46): the chapter registries' "raised by" columns name the report verbs for 557
  (`report.design`, `report.campaign`), 872 (`report.*`, `--cite` and statements), 845 ("report verbs"), 820
  ("wherever a citation is made: reports"), 312 (N19a-39's realism check, which BD-302's report verbs carry) and
  447 (`report.run --claim reality`, N20b-18); BD-302 raises 869; N39b-46 raises 862, 863 and 865 on
  `report.campaign`. The registry cross-check is bidirectional, so every one is on its row. Round 3: 822 is on
  all four rows, because each takes `--cite` through N16b-04's citation check, which returns 822 (and neither 820
  nor 557) for a `capacity-biased` comparison cited as a verdict (N18b-22: "any verb asked to cite the comparison
  as a verdict, a report included"); 830 stays on none (no report spends budget). 880 is on all four rows: N27-REF
  names the four report verbs as its `raised_by` (N27b-15's citation guard on `--cite` and on statements). 819 is
  on all four rows: every report sentence and citation passes the evidence boundary (N37-26 step 6; N00-25's
  rule 3), whose code is 819 where no more specific one applies. 870 is on `report.design`, `report.mission`
  (BD-347's consumer check) and `report.campaign` (wired by BD-352), and not on `report.run`, which presents no
  verdict carried across an edit.
- Citation roles on the command line (DESIGN; N27b-15 leaves the spelling to the report owner). No `--role`
  option exists: `--cite <id>` cites in the role `Established` (BD-302's `report::resolve_cited`, whose
  non-established branch is N16b-04's `cite_as_established`); on `report.run`, `--claim reality` with `--cite
  <id>` places the citation under the Reality claim, the role `Reality`; a result the report reaches through its
  lineage without `--cite` is printed in its permitted wording, the role `Described`, and is never refused for
  its standing. For a claim an Autopsy verb produced, `resolve_cited` calls N27b-15's guard `offer(claim, role)`
  after the closure check (869) and before `cite_as_established`, so such a citation in the role `Established`
  or `Reality` answers 880 and not 820 (N27b-15: where 880 and a general citation code could both fire, 880 is
  returned, because its repairs are the useful ones).
- Realism claims (`--claims-realism`, every report verb). One request gets one code, the most specific
  registered for what fails (N37-26's precedence): a scene with `assumed` dimensions is 312 (N19a-39's
  `realism_claim_admissible`); a result whose weakest input is an `assumed` interface, film or soil card field
  is 819 from the evidence boundary, with `observed` the class `Estimated` and `weakest` naming the card and its
  field (OPEN-TRAC-3's fail-closed default: every result that depends on such a card stays `Estimated`, and
  claims of realism over it are refused); under `report.run --claim reality`, an `assumed` sensor or optical
  card field is 447 (N20b-18). DESIGN: no contact-card code is added, because 819's repairs (measure the
  assumed field; cite the result for what it supports) are the right ones and its fields carry the card field.

- A report uses only the permitted wording of each claim kind, prints the `reality:` line, and refuses to cite a
  result above its claim product (`FROB-REF-819`).
- Reports are exporting verbs: each carries the three no-authorization flags, all false.
- A report that would carry a research-only, proprietary or unknown-licence asset out of the store is refused with
  `FROB-REF-733`, and one derived from a private scene, without the private scope and flag, with `FROB-REF-731`; both
  are raised at the one export door every exporting verb passes (N37-18; the private rule is BD-305) and both exit 5.
  The three rows that hold the scope `export:report` (`report.design`, `report.run`, `report.mission`) carry
  `exports = true` (N37-18's lint then requires their handlers to route through `frob_verbs::export::admit`)
  and list both codes and the door's first check, `FROB-REF-816` (the no-authorization flags absent from the
  manifest), which every exporting row lists beside them. None takes an output path, so none lists 734.
  `report.campaign` holds no publish scope and is no export. No row of this cluster lists `FROB-REF-310`. DESIGN: the plan words that scene-chapter code "a pack or report
  would export a research-only or private asset" and also defines 731 and 733 in the safety chapter, without dividing
  them; the division used by every bead is that 310 is raised only by the scene pack's own export check when the
  exported artifact is itself a scene pack (N19b-02), and every other export, a report included, is decided at the
  door. Reason: a report holds no pack manifest, so only the join over its lineage can find the asset, and one request
  must get exactly one code.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "report"` (4 `read`), each with every field
      of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster report` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_report.sh` runs each verb of the cluster (four), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/report/<verb>.help.json` byte for byte for each of
      the four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`; `report.mission` cites
      `RETROFIT_COST_ASSUMED_PRICES` (N19c-42 row 9) and lists its number once the refusal registry holds it.
- [ ] The script's run leaves a receipt with the measured counts (registry, CLI and MCP each 4; every difference
      list 0) and the controls caught. This bead records no gate value: the per-milestone surface beads (BD-312 at
      M3, then BD-376) record GATE-TETHER-02's part for their milestone from such a receipt with `cargo run -p
      frob-conformance --example record_gate -- --gate GATE-TETHER-02 --receipt <hash> --part <M>`, in a session
      that did not write the rows (N38b-19).

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_report` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `report.mission` is `read` at M3).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`report.campaign`: 312, 557, 819, 820, 822, 845,
  862, 863, 865, 869, 870, 872, 880; `report.design`: 312, 557, 816, 731, 733, 819, 820, 822, 845, 869, 870, 872,
  880; `report.run`: 312, 447, 816, 731, 733, 819, 820, 822, 845, 869, 872, 880; `report.mission`: 312, 816,
  731, 733, 819, 820, 822, 845, 869, 870, 872, 880 and the number of
  `RETROFIT_COST_ASSUMED_PRICES` once allocated); no row lists 310 or 830; the appended codes equal the rule for
  its kind and fields (NJL-01): {720, 723, 724, 802, 808} for each row, plus 891, 734 and 828 for
  `report.campaign` (its `--register-comparison` file).
- `exporting_rows`: `report.design`, `report.run` and `report.mission` have `exports = true` and
  `export:report` among their capabilities; `report.campaign` has neither.
- `raised_by_cross_check`: for 557 the rows holding it are exactly `report.design` and `report.campaign` among the
  report verbs, and every report row holds 872, as the "raised by" columns of N28-REF require, 822, as the
  citation check of N16b-04 returns it on every citing verb, 880, as N27-REF's `raised_by` names the four report
  verbs, and 819, as every report passes the evidence boundary; the rows holding 870 are exactly
  `report.design`, `report.mission` and `report.campaign` (BD-347's consumers); a planted removal of 872, 822,
  880 or 819 from `report.run`, or of 870 from `report.campaign`, fails.
- `inputs_by_row`: every report verb accepts `--cite` and `--claims-realism`; `report.run` accepts `--claim
  reality|performance` (with `--cite`, the role `Reality`); `report.mission` accepts `--publish`;
  `report.campaign` accepts `--compact`, `--arms`, `--stratum`, `--regime`, `--ablate`,
  `--register-comparison`; a `--role` on any report verb is 802.
- `mcp_names_one_to_one`: the tool names are the mapped names (`report_campaign`, `report_design`, `report_run`,
  `report_mission`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_report.sh` (header: `bead=VERBS-report milestone=M3 labels=tether,verbs,verb:report.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance report`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
report` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster report`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_report/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 4 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 4 verbs (4 `read`); the tables above. KA-755 (the exit-code table on malformed,
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

- Closing rule: the bead closes at M3, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M3, M4. The per-milestone
  surface beads (BD-312, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258: a row cites scopes of its own cluster by kind (`read` →
  `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` → `<cluster>:write` or a named write
  scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope), plus `export:*` when the row exports and
  `private:*` where privacy applies, never a scope named after the object read or the engine spent on; N32b-01's
  catalogue generates the forms. `<cluster>:run` and `<cluster>:write` imply `<cluster>:read`, closed when a set is
  stored (at minting and at derivation, `Scopes::close_implied`), so checks stay plain set inclusion. Inputs and
  outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead fixes the
  spellings and the snapshot then pins them (additive changes keep the schema's major version; anything else raises
  it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §28.18, §32.4, §32.6,
§32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755; N28-REF (557 and 830 precedence); N37-18
(the export door, 816); N19c-42 row 9 and N19b-43 (`retrofit-cost-on-assumed-prices`); NJL-01, NJL-04, NJL-05.
