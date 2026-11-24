---
key: "VERBS-design"
title: "frob-verbs registry: cluster `design`, twelve rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "skeleton", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§14.4", "§14.7", "§14.8", "§14.10", "§14.11", "§14.12", "§28.7", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `design`. The cluster covers the design language: creating a
design from a template, a file or a mission cell, typed edits and patches with an expected parent, the eight-stage
preflight `design.check`, lineage (diff, blame, bisect), import and export with loss reports, and the two views of
the canonical term. It has twelve verbs (7 `read`, 1 `run`, 4 `mutate`); the handlers live in `frob-analyze`,
`frob-design`, `frob-io`, `frob-opt`. The bead carries every row completely, with the three checks that keep the
rows true: the generated CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema
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

In: the twelve registry entries of cluster `design`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `design.bisect` | run | M4 | S | Find the edit in a lineage that changed a verdict, with an anytime-valid comparison at each step | §28.7 |
| `design.blame` | read | M1 | S | The edit that last changed a field or a derived quantity | §28.7 |
| `design.check` | read | M1 | S | The eight preflight stages and the analyses marked for `check`; findings with ranked repairs | §14.11, §28.4 |
| `design.diff` | read | M1 | S | Structural and parameter difference of two versions, with the symmetry effect | §28.7 |
| `design.edit` | mutate | M1 | S | One typed edit; returns the new `DesignId`, the symmetry effect and the invalidation set | §14.7 |
| `design.patch` | mutate | M1 | S | Several typed edits against a named parent, with locks and required analyses | §14.8 |
| `design.templates` | read | M1 | S | The eighteen archetypes and their parameters | §14.10 |
| `design.new` | mutate | M1 | S | Create a design from a template, a file or a mission cell | §14, §32.6 |
| `design.show` | read | M1 | S | Show a design and its compiled sections | §14, §32.6 |
| `design.import` | mutate | M1 | S | Import a robot description with a loss report | §14.12, §32.6 |
| `design.export` | read | M1 | S | Export a design with the reverse loss report | §14.12, §32.6 |
| `design.view` | read | M1 | S | Convert between the JSON and S-expression views | §14.4, §32.6 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *study*: a Design comparison of class Tested (Estimated if any input is `assumed`); tier and determinism the
  weakest of the arms; `provisional` until promoted to the referee.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.

**`design.bisect`** (MCP tool `design_bisect`; owner `frob-opt`)
- Inputs (BD-343's spellings, used by N28b-02, N28b-03, N28b-06, N28c-21, N28c-24 and NS-06): `--good
  <version>` and `--bad <version>` (the lineage path from the good to the bad version; the bad version must be a
  descendant); `--predicate <expr>` (`succeeds`, the task's requirement verdict, or another probe predicate of
  BD-343); `--task T`; `--margin δ` (the predicate's margin, same meaning and default as `eval.compare`'s);
  `--level α` (the total level, default 0.05, as `eval.compare`); `--via <version>` (names one path when several
  exist; else the path with the fewest edits, ties by canonical bytes, reported as `path_rule`); `--scan-prefix`;
  `--assume-monotone`; `--require first-change` (an `a-change` is not an acceptable answer); the common budget and
  key of a `run` verb. No positional design and no `--from`/`--to`: an earlier example of this row had them.
- Outputs (N28b-02's `frob.bisect/1`): the path and `path_rule`, the predicate, α, the probes planned and run with
  each probe's record and e-value, and the label: `a-change` (a pair whose verdicts differ, always a true
  statement), `first-change` (only with `--scan-prefix`, or with `--assume-monotone`, which is recorded as the
  assumption `monotone-predicate`), `bracket` (an inconclusive probe stopped the search: the bracket reached and
  what each step would still need) or `no-change`; further changes found by a scan listed after the first.
  Exit 0 for a found change (`a-change` or `first-change`: the verb answered its question with a true
  statement), 2 for `bracket` or `no-change` (BD-343's DESIGN). Evidence: *study* (each probe's comparison
  `provisional` on a fast tier; `next_actions` offer its promotion on the referee and `explain.compare`).
- Chapter refusals: `FROB-REF-550` (`first-change` asked of bisect without a prefix scan or a declared monotone
  predicate; exit 3, zero probes). Every probe is a paired comparison, so the probe plan passes N28a-18's
  `pairing::admit` and then N35-07's determinism licence, as on `eval.compare`, before the error budget is fixed
  and before the first probe; one failing admission refuses the whole search with zero probes: `FROB-REF-542` (a
  D4 arm, arms of different classes, or D3 without decision-level equivalence recorded for the task);
  `FROB-REF-543` (the predicate `metric M worse-by m` on a metric with no declared span; `succeeds` has the span
  [−1, 1]); `FROB-REF-561` (the licence-only cases). A `--bad` that is not a descendant of `--good` is the
  surface's `FROB-REF-802` (exit 3, pointer `/bad`, naming both versions and the relation found,
  `not-descendant`), before any probe; 802 is appended to every row, so no chapter code is added for it.
- Implemented by: BD-343, with N28b-02 (the result record, the scan and the assumption).

**`design.blame`** (MCP tool `design_blame`; owner `frob-opt`)
- Inputs: design; field path or derived quantity; `max_versions` (the versions walked, default 64; the verb
  reports where it stopped; BD-113).
- Outputs: the edit that last changed it, with author, parent and patch identifier. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-113.

**`design.check`** (MCP tool `design_check`; owner `frob-analyze`)
- Inputs: design (a name, a `DesignId` or a design file in either view); optional `task`, `scene`, `budget` and
  `skill`, which bind the analyses marked "when bound" (A-18
  with a scene, A-20 with a budget or a task segment with a rise, A-24 with a skill or task wrench); `stages` (a
  range; the rest are `not-run: excluded`); `all` (collect every row's findings instead of stopping a stage at its
  first refusal); `strict_data` (a card field `assumed` refuses, 113); `time_budget` (stage 8's budget, default
  10 s); `no_memo` (`--no-memo`, generated from the row's `memo = true` by NJL-01's variation 14 and not declared
  by the row: the memo graph serves nothing for this call, in any store mode; every stage and analysis is
  computed and its result still stored, memo disposition `NoMemo`; N28b-38, BD-111); from M6 `battery_age` (`--battery-age
  f_q=<ratio>,f_r=<ratio>`, either member defaulting to 0: the bus feasibility of stage 6 at the aged values, with the age assumed stated in the report; N15b-14).
- Outputs: per stage its status in the wire forms of N14b-10 (`passed`, `refused`, `undecided`, `not-run`; a
  stage whose findings refuse nothing is `passed` with its `findings[]`, and `not-run` carries its reason), its
  evidence class and its timing against its budget; per analysis row `run` or `not-run` with its reason
  (`unbound` with the flag that binds, `stopped`, `excluded`, `not-landed`); findings with code, severity and
  ranked repairs as typed edits; `closure`, `symmetry`.
  Evidence: the claim product of the analyses run, each labelled as in VERBS-analyze (*analysis*).
- Exit policy (BD-110's policy rows in `registries/preflight.toml`, applied by N14bx-01's `exit_for`): exit 3 when
  a stage refuses, that is when a finding's policy row says refusing, with the stage's owned code, the pointer and
  a first repair; exit 2 for reported findings (a High finding with no registered code included), for an
  `undecided` stage (an analysis that exhausted its budget) and for a stage over its budget; exit 0 with nothing
  to report. A verb that is not `design.check` never refuses on a finding; refusing on findings is this verb's
  policy (§32.4; §32.16 shows exit 2 for an A-11 finding, which BD-110's table classes as reported).
- Chapter refusals: the codes owned by the eight stages (N14b-10's stage table, 28 codes, with 168 that BD-110
  adds at stage 2) and by the analyses of the "when bound" rows (N14bx-01): stage 1 `FROB-REF-101` to -103,
  -105, -107, -108 (raised at binding: a power port whose signed motion witness disagrees with its sign, N15b-13,
  and an actuator on no bus or on two, N14a-09; N14b-10's row 1), -111, -114, -121, -125, -126, -130, -134;
  stage 2 -104, -106, -168 (N13-04's solid-fill plausibility, through A-02's kind `A-02.<body>.solid-fill`,
  BD-110); stage 3 -101, -107, -110, -123; stage 4 -118; stage 5 -108, -109; stage 6 -113, -115, -116, -124,
  -131, -132, -133, and from the bound A-20 and A-24 -551 and -448; stage 7 -119 only (plan §14.11): a
  `FROB-REF-308` from the bound A-18's route query is not a code of stage 7's row, and the stage reports it as
  status `undecided` with 308's repairs as `next_actions` (N14b-12), so 308 is not in this list. In order: 101,
  102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 113, 114, 115, 116, 118, 119, 120, 121, 123, 124, 125, 126,
  130, 131, 132, 133, 134, 168, 448, 551 (31 codes). With `battery_age`, a value outside its
  bounds is 111 and an aged bus that cannot supply the demand is 133 (N15b-14: existing codes, never a new one).
  Pending: the stage-7 lead refusal (a target whose lead is shorter than the declared latency chain, computed by
  N20b-33), raised under its reserved slot `sens-lead-too-short` (family `4xx`, ledger row in N35x-02) with exit
  3 until N20b-45 applies the number the ledger assigns; it joins this list then, as VERBS-sense notes.
- Implemented by: BD-110 (stages 1 to 6 and the policy rows), BD-244 (stages 7 and 8); the runner, budgets and
  options are N28ap1-01, N14b-10 and N14bx-01's.

**`design.diff`** (MCP tool `design_diff`; owner `frob-opt`)
- Inputs: two design versions; `as` (`edits` or `summary`; BD-113).
- Outputs: structural and parameter differences as typed edits (verified: applying them to A gives exactly B's
  `DesignId`, else `edits: unavailable` with the first mismatching path), a structural summary, a parameter table,
  with the symmetry effect; from M3 `correspondence`, the composed correspondence record (`frob.design.corr/1`)
  along the lineage path from A to B, or `no lineage path` (N14ax-02). Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-113.
- Extended by: N14ax-02, at M3: the composed `correspondence` output. N28c-44, at M3: `reuse` (A and B a parent
  and child joined by typed edits in one lineage; the result gains the three never-summed reuse columns, exact reuse, evidence carried forward and reacquisition
  avoided, per regime and resource), `regime` (`cold`, `warm`, `both`), `detail` (breakdowns by node kind), and
  the sub-mode `run-control`, which submits the from-scratch controls of the edit as a job. That sub-mode is an
  action of kind `run` (capability `design:run`; it takes an idempotency key and the budget, waits boundedly and
  otherwise returns a job handle with exit 8; `estimate_only` prices the control without running it), carried as
  `actions = [{ name = "run-control", kind = "run", capabilities = ["design:run"] }]` (N32a-02's two-kind form,
  served by NJL-01's `action` property). It is selected only by that property: on the command line `--action
  run-control` with `--key` (`frob design diff quad-a@prev quad-a --reuse --action run-control --key rc-1 --json`),
  over MCP `"action": "run-control"`; there is no `--run-control` flag (NJL-01 action rule 1). The default action
  stays `read`. Evidence of the reuse figures: claim
  kind Performance, class `Tested` where a control measured the total and `Estimated` where it is reconstructed,
  with the performance disclosure block.

**`design.edit`** (MCP tool `design_edit`; owner `frob-design`)
- Inputs: design (a name, resolved to its head and echoed in `bound`, or a `DesignId`); the edit kind; the target,
  a JSON pointer into the term or a selector `<instance pattern>.<field>` (for example `foot.radius`), with
  `joints <pattern>` for the kinds that act on joints and `instance <id>` to name one image under a constructor;
  the value or values; expected parent. Kinds whose inputs later beads add:
  - `extract-module` (M1, N41p2-01; catalogue row 23, symmetry effect `none`; its inverse is the same kind with
    `inline: true` at the `use` path): the target is the limb subterm, with `--name <module-name>` and
    `--args @<file>` (the file supplies `op.args`: `ports`, `params`, `contract`; a path parameter, so the field
    rule applies), as in `frob design edit quad-a extract-module /limbs/of/of --name leg-quad-a --args
    @extract.json --key x-3 --json`.
  - `resolve-sized` (M3, N31b-41): `--loads statics | measured:<batch>` re-resolves every `{shell: sized}` fill
    against the bounded statics load set, or against its union with the measured batch's cases, to the
    thinnest passing stock wall (no inverse: the authored version stays the parent; symmetry effect
    `preserved`, walls resolved per symmetry class); and from the same bead `set-fill` accepts the value
    `{shell: sized}` on a Region the shell rule supports with a uniform wall (elsewhere `set-fill`'s 105).
  - `correspond` (M3, N14ax-02; any kind): `--correspond <kind>:<from>=<to>[,…]` replaces computed
    correspondence entries by caller-supplied ones (`Replaced` or `Same`, origin `supplied`), carried in the
    operation's arguments so that they enter the `edit_id`; both paths must exist in parent and child, kinds and
    an actuator role's joint class must agree, and no role may be claimed twice, else 101 (type mismatch) or 103
    (unknown path), with the pointer.
- Outputs: the new `DesignId`, its parent, `edit_id`, the inverse, the structural and parameter diff, the resolved
  target as a term path, the declared symmetry effect (`preserved`, `reported`, `grows` or `none`, with the
  catalogue row's qualifier, and `symmetry.split`, the constructors an `Instance` edit split), the predicted scope
  of invalidation; the computed symmetry groups and the computed invalidation set when their functors are in the
  build (the invalidation set over stored verdicts and derived assets is N28b-01's); with `extract-module`,
  `result.module` (`ref`, `contract_digest`, the derived port signature) and `result.model_id_unchanged: true`
  (N41p2-01); from M3 `correspondence`, the record `frob.design.corr/1` (parent-to-child correspondence of
  frames, actuator, sensor and contact roles) with its digest (N14ax-02). Evidence: *static*.
- Chapter refusals (BD-058's precondition table): `FROB-REF-101` (port type mismatch); `FROB-REF-102` (unit or
  dimension error); `FROB-REF-103` (unknown, misspelt or duplicate field, or a target that does not resolve);
  `FROB-REF-105` (a body without a fill; a solid written as a shell); `FROB-REF-106` (declared mass properties
  violate the mass-support bound); `FROB-REF-107` (empty joint range); `FROB-REF-108` (power sign mismatch, copied
  power port or algebraic loop); `FROB-REF-111` (a lever outside its bounds; an edit to a locked field);
  `FROB-REF-114` (module digest or port signature mismatch); `FROB-REF-121` (contact law missing or incompatible
  with its end effector); `FROB-REF-123` (a `mirror` or `radial` wrap of a limb mounted on its plane or axis);
  `FROB-REF-125` (unsupported schema version, `migrate`); `FROB-REF-126` (a sensor without a parent frame, a rate
  or a clock). `FROB-REF-109` is not in this row: an `assert-symmetry` edit is checked at stage 5 by
  `design.check`, which raises 109. The refusals of `extract-module` (103, 108, 111, 114, 121) and of
  `--correspond` (101, 103) are in this list. A `resolve-sized` whose link no stock wall carries is refused, exit
  3, under the ledger slug `sized-fill-unsatisfiable` (row 1 of N31b-45, family `1xx`, filed by N35x-02;
  N31b-41's resolution), with the pointer at the fill and the thickest entry's governing case and utilization;
  it joins this list under its assigned number in the change that loads it, as it joins `design.check`'s.
- Implemented by: BD-058, N28b-01. Extended by: N41p2-01 (`extract-module`, M1), N14ax-02 (`correspond` and
  `correspondence`, M3), N31b-41 (`resolve-sized` and `set-fill {shell: sized}`, M3).

**`design.patch`** (MCP tool `design_patch`; owner `frob-design`)
- Inputs: `frob.design.patch/1` (a file, `--file <patch.json>`, or standard input, in either view): several
  typed edits against a named parent (`parent`), with locks (`constraints.locked`) and required analyses
  (`constraints.require`). The key may be written in the document (`idempotency_key`), given as `--key`, or both;
  both present and unequal is `FROB-REF-802` naming both values. The document's `parent` is the expected parent:
  a call's `--expected-parent`, when given, must equal it, else `FROB-REF-802` likewise; so a stale parent is
  always 112 from the patch reader and never also 804 (BD-059, N14-REF's division).
- Outputs: the new `DesignId`, per-edit results, the invalidation set; all or nothing. Evidence: *static*.
- Chapter refusals: `FROB-REF-112` (stale parent in a patch, exit 7, with the current head and the diff from
  `parent` to it); `FROB-REF-103` (unknown, misspelt or duplicate field of the document, or an operation
  addressing a derived quantity); `FROB-REF-111` (a lever outside its bounds; an edit to a locked path); the
  precondition code of a refused operation, with its index (the codes of `design.edit`'s list: 101, 102, 103,
  105, 106, 107, 108, 111, 114, 121, 123, 125, 126); the code of a required analysis that refuses, which is a
  code of `design.check`'s list. A refusal of any operation or required analysis refuses the patch as a whole,
  and nothing is stored. The row's `refusals` is therefore 112 with the 31 codes of `design.check`, which
  contain every code of `design.edit`'s list: 32 codes.
- Implemented by: BD-059.

**`design.templates`** (MCP tool `design_templates`; owner `frob-design`)
- Inputs: optional archetype filter (`--name <archetype>`; a registered alias is accepted and returns the
  canonical entry with the alias marked, N14bx-03); from M6 `include_variants` (N14b-05).
- Outputs: the eighteen archetypes with their parameters, bounds and defaults, each entry with its `aliases: [{
  name, preset, reason }]` (N14bx-03, additive); with `include_variants` and a
  variant feature compiled in, each variant row with its tier, flag, physics owner, physics bead and computed
  status (with no variant feature compiled in, the eighteen and the note `no variant features in this build`,
  never a refusal; a Moonshot variant is never offered as a suggested next action). Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-109. Extended by: N14bx-03 (`aliases`, M1), N14b-05 (`include_variants`, M6).

**`design.new`** (MCP tool `design_new`; owner `frob-design`)
- Inputs (N14a-21): name; exactly one source: a file (`--file <path|->`, either view), a template
  (`--template <archetype>` with `--preset <ref>` and `--set id=value`…; `--template` also accepts a registered
  alias, which instantiates its target with the alias's preset, `--set` still applying on top, N14bx-03), a
  mission cell (`--from-cell <mission>/<cell>`), or from M3 a design content address readable by the caller
  (`--from <address>`, N32b-03: the fork into the caller's namespace whose root records the source address and
  its lineage, the first repair of `FROB-REF-725`; distinct from `--from-cell`).
- Outputs: `DesignId`, the name, the source, the parameter point (the template's parameters after `--set`, the
  cell centre for a mission cell, absent for a file), the computed symmetry groups, the stage-1 report; for a
  template, `template` (the canonical archetype), `preset` and `alias_used` (the alias given, if any), the
  recorded family staying canonical (`template:<archetype>@1`; N14bx-03); with
  `--from`, the fork root's source address and lineage. Exit 0 with no findings, 2 with stage-1 findings (the
  design is stored; a stage-1 refusal leaves `symmetry` absent with `reason: "stage-1-refused"`). Evidence:
  *static*.
- Chapter refusals: `FROB-REF-102` (unit or dimension error); `FROB-REF-103` (unknown, misspelt or duplicate field);
  `FROB-REF-111` (a `--set` value outside a template parameter's bounds); `FROB-REF-125` (unsupported schema
  version); `FROB-REF-808` (a JSON integer outside ±(2⁵³ − 1) in a design file, N14a-01's rule 5). An existing
  name is the appended stale-parent conflict 804, exit 7 (a create is a `mutate` whose expected parent is "no
  lineage"); a write outside the caller's namespace is the appended 725.
- Implemented by: N14a-21. Extended by: N14bx-03 (template aliases, M1), N32b-03 (`--from`, M3).

**`design.show`** (MCP tool `design_show`; owner `frob-design`)
- Inputs (N14a-21): a design or a design file; `section` (repeatable; each section also has a shorthand flag of
  its own name, such as `--mass` or `--frames`; default `summary`); `at` (`home`, a joint map, or `@<path>`, a file
  of at most 10⁵ configurations, one joint map per line, answered from one compile); `tier` (`host` or
  `referee`); `no_memo` (generated from the row's `memo = true`, as on `design.check`; not declared by the row).
- Outputs: the design's canonical term and compiled sections (model identity, mass roll-up, symmetry, levers,
  locks; with a file of configurations, one frame set per configuration in file order). Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-102`, `FROB-REF-103`, `FROB-REF-125` and `FROB-REF-808` (a JSON integer outside
  ±(2⁵³ − 1), N14a-01's rule 5), a design file that does not parse (the reader's codes). More than 10⁵
  configurations is 802.
- Implemented by: N14a-21.

**`design.import`** (MCP tool `design_import`; owner `frob-io`)
- Inputs (BD-065's spellings): the document (a file); `--format` (`urdf` or `mjcf`, inferred from the root element
  when absent, `mujoco` routing to BD-066's MJCF mapper; `sdf` and `usd`, values of the schema from M1, which
  BD-065's format dispatch recognises (with an `sdf` root element when `--format` is absent) and dispatches to
  the SDF importer of N14b-22 or the OpenUSD importer of N14bp1-03 once it is linked; from NEG-32 also
  `fsim-catalog` with `--builder <name>`);
  `--name <design name>` (required); `--strict`; `--expanded-by <tool>` (the macro expander that produced the
  document, recorded in `source`); `--fold-fixed` (write the folded tree of N14b-33's fold, each folded link
  listed in the report as `folded` with its mass); `--reduced <profile>` (a declared reduction, N14b-34; every
  mass-dependent result then carries `evidence.reduced = { profile, omitted_mass }`); the idempotency key.
- Outputs: `design` (the new `DesignId`), `source` (hash of the bytes, `expanded_by`), `loss_report` (artifact
  identifier: every element dropped, approximated, defaulted or folded), `counts` (mapped, default-filled,
  dropped, unsupported, repaired-proposed), `identifier_map`. Evidence: *static*.
- Chapter refusals: `FROB-REF-103` (unknown, misspelt or duplicate field: a bug guard on the produced term);
  `FROB-REF-122` (an unsupported element under `--strict`; an unexpanded macro or an unknown root at any time);
  `FROB-REF-125` (unsupported schema version); `FROB-REF-734` (a file reference or include inside the document
  that resolves outside the import root: the path jail of N37-11, exit 5, nothing read); `FROB-REF-828` (a
  document whose leading bytes are an executable, script, pickle or archive signature, refused by the
  dispatcher's sniff before the reader, N37-03; N37-03's own end-to-end step drives eight such files through this
  verb); `FROB-REF-832` (an import over its size, depth or polygon limit). N37-REF's list test requires every
  importing verb to list 828 and 832. `FROB-REF-724`, appended to every row, is what `--format sdf` or `--format
  usd` (or an `sdf` root without `--format`) answers until its importer is linked: one envelope, `reason:
  feature-not-built`, `mechanism: import.sdf` or `import.usd`, exit 5, `retry: never`, before any byte of the
  document is read and with no design created, the message naming the condition under which the importer is
  built (BD-065). Pending: the mass-dropped refusal (an import drops a body of nonzero mass
  without a declared reduction; payload each dropped body with its mass and the total; exit 3; N14b-34), ledger
  slug `import-mass-dropped` (family `1xx`, N35x-02, which has the proposal 127 on file and no assignment); it
  joins this list under the number the ledger assigns, and until then is raised under its slug.
- Implemented by: BD-065, BD-066.
- Extended by: N14b-33 (the fold behind `--fold-fixed`), N14b-34 (`--reduced` and the mass-dropped refusal),
  NEG-32 (the input format `fsim-catalog` with `--builder`), N14b-22 (the SDF importer behind `--format sdf`),
  N14bp1-03 (the OpenUSD importer behind `--format usd`).

**`design.export`** (MCP tool `design_export`; owner `frob-io`)
- Inputs: design; `format` (`urdf` or `mjcf`; from M2 also `glb`, N14b-21: a standalone GLB of the design's
  visuals for the viewer and stills, `frob design export <design> --format glb [--frames] [--chord <length>]
  [--out <dir>] --json`); `out` (a directory under the session's export root); `visuals` (`glb` or `none`; `glb`
  from N14b-21); with `--format glb` only, `frames` (flag: sensor mounts and port frames exported as named
  empties) and `chord` (the chord tolerance of a collision proxy tessellated where a body has no visual mesh,
  default 0.5 mm, recorded).
- Outputs: the exported file's path and hash, the GLB files' hashes when visuals were written, the reverse loss
  report's identifier and counts, the identifier map, and the `authorization` block with the three
  no-authorization flags, all `false` (`exports = true`). Evidence: *pass-through* (class `Tested`, tier `static`,
  determinism `exact` for the report; the exported text is a view and has no identity of its own).
- Chapter refusals: a design that stage 1 refuses is not exported, with its stage-1 code (`FROB-REF-101` to -103,
  -105, -107, -108, -111, -114, -121, -125, -126, -130, -134); `FROB-REF-816` (the artifact's no-authorization flags
  missing or not `false`, the door's first check), `FROB-REF-731` (a private asset) and `FROB-REF-733` (a licence
  that forbids export), raised by the export door N37-18 through which the verb is routed; `FROB-REF-734` (`out`
  outside the session's roots, by the path jail). N37-REF's list test requires every exporting verb to list 731,
  733 and 816. `FROB-REF-122` is not in this row: it is the import's refusal under `--strict`.
- Implemented by: N14a-25, BD-066. Extended by: N14b-21 (`--format glb` with `frames` and `chord`, and `--visuals
  glb`, M2).

**`design.view`** (MCP tool `design_view`; owner `frob-design`)
- Inputs (BD-057: `frob design view <design-or-file> --as sexpr|json [--pretty] --json`): design or document (a
  file, a path parameter); `--as sexpr` or `--as json`; `pretty` (`--pretty`: reflows the view with indentation
  for reading; pretty text is accepted on input and is not canonical).
- Outputs: the other view of the same canonical term, `result.text` with `result.view`, `result.canonical` (true
  unless `--pretty`) and `bound.design`; the round trip is the identity on canonical forms. Evidence:
  *pass-through*.
- Chapter refusals: `FROB-REF-102` (unit or dimension error, a parsing refusal of BD-057); `FROB-REF-103` (unknown,
  misspelt or duplicate field); `FROB-REF-125` (unsupported schema version); `FROB-REF-808` (a JSON integer outside
  ±(2⁵³ − 1) in the document, N14a-01's rule 5); `FROB-REF-892` (`view-is-not-identity`: A view is offered where an identity is required: to hash, to
  sign, to cite as evidence, or as a cache key).
- Implemented by: BD-057.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `design.bisect` | `design:run` | `frob.bisect/1` | `search` | `frob design bisect --good d:41aa --bad d:7c03 --predicate succeeds --task stairs-0.4 --key bis-1 --json` |
| `design.blame` | `design:read` | `frob.design.blame.out/1` | `search` | `frob design blame quad-a /limbs/of/of/chain/2/transmission/belt/ratio --json` |
| `design.check` | `design:read` | `frob.design.check.out/1` | `author`, `places` | `frob design check quad-a --json` |
| `design.diff` | `design:read` | `frob.design.diff.out/1` | `author` | `frob design diff quad-a@prev quad-a --json` |
| `design.edit` | `design:write` | `frob.design.edit.out/1` | `author`, `repair` | `frob design edit quad-a set-param foot.radius 0.030 m --expected-parent d:7a3e --key ed-1 --json` |
| `design.patch` | `design:write` | `frob.design.patch.out/1` | `modules` | `frob design patch quad-a --file patch.json --key pat-1 --json` |
| `design.templates` | `design:read` | `frob.design.templates.out/1` | `orient` | `frob design templates --json` |
| `design.new` | `design:write` | `frob.design.new.out/1` | `author` | `frob design new quad-a --template quadruped-12dof --key des-1 --json` |
| `design.show` | `design:read` | `frob.design.show.out/1` | `author` | `frob design show quad-a --json` |
| `design.import` | `design:write` | `frob.design.import.out/1` | `author` | `frob design import arm.urdf --name arm-a --key imp-1 --json` |
| `design.export` | `design:read` | `frob.design.export.out/1` | `modules` | `frob design export quad-a --format urdf --json` |
| `design.view` | `design:read` | `frob.design.view.out/1` | `modules` | `frob design view quad-a --as sexpr --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column. `design.check` is the exception to the `static:` rule for
reads: it has `estimable = true` and `cost_model = "estimated: --estimate returns the planned stages and rows with
their budgets (stages 1 to 6 within 2 s at the 95th percentile for a template-sized design, TARGET; stage 7 about
2 s per bound scene; stage 8 the call's time budget, default 10 s)"`, because its cost exceeds one second when a
scene or the information stage is bound and N32a-15's lint `estimate-required` warns on a `static:` bound above
one second; the estimate is N14bx-01's `plan_check` priced by the stage budgets. `design.export` has `exports =
true`. `design.check` and `design.show` have `memo = true` (BD-111's field, set in its change at M1): NJL-01's
variation 14 generates their `no_memo`, and neither row declares it as an input of its own (a row that does fails
the lint `common-property-shadowed`).

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "design.bisect"
cluster = "design"
kind = "run"
tier = "S"
capabilities = ["design:run"]
input_schema = "frob.design.bisect.in/1"
output_schema = "frob.bisect/1"
refusals = ["FROB-REF-542", "FROB-REF-543", "FROB-REF-550", "FROB-REF-561"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M4"
task_clusters = ["search"]
examples = ["frob design bisect --good d:41aa --bad d:7c03 --predicate succeeds --task stairs-0.4 --key bis-1 --json"]
owner = "frob-opt"
purpose = "Find the edit in a lineage that changed a verdict, with an anytime-valid comparison at each step"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them (`run`:
   `sync_wait_ms` from 0 to the row's `wait.mcp_max_ms`, 30,000, with MCP default 5,000; `idempotency_key`, CLI
   `--key`; `budget`; `estimate_only`, CLI `--estimate`; `on_cancel`, `leave` (the default: the job keeps
   running) or `cancel_job`; `envelope_scope`, CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`, the
   approved compute-envelope scope the job cites (program convention 14, owned by NJL-01); `mutate`:
   `idempotency_key` and `expected_parent`). `idempotency_key` is never in the JSON-schema `required` array: the
   annotation `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the rule, and the
   admission transaction refuses a missing key with 805 on both surfaces. The surface-wide refusals appended by
   kind: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. The properties and codes driven by fields are those of NJL-01's
   variation table: `cursor`, `limit` and `follow` with 806 and 807 for a paginated entry; 808 (an integer outside
   ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property at any depth, whatever its kind
   (every `run` entry has one, `sync_wait_ms`); `estimate_only` on a non-`run` entry with `estimable = true`; no
   805 for a `key_optional` entry; the `action` property, with each action's codes, for an entry with `actions`;
   790 for a non-empty `needs_groups`; 891 for an `artifact-input<F>`, `artifact-ref<F>` or
   `artifact-identity<F>` parameter, and 892 as well for `artifact-identity<F>`; 893 for a `foreign-file`
   parameter; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The later variations of that table (10 to 16) land with their beads
   and are read there, not copied here.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- The cluster has twelve verbs, more than an agent should see at once; the task clusters `author`, `modules`,
  `search` and `repair` each list at most seven tools (bead NJL-05).
- Handlers live in four crates: `frob-design` (the term, edits, views), `frob-analyze` (`design.check`), `frob-opt`
  (lineage) and `frob-io` (import, export). The registry field `owner` records which.
- `design.edit` and `design.patch` are `mutate` verbs: they take an idempotency key and an expected parent; a stale
  parent is refused (`FROB-REF-804` at the surface, `FROB-REF-112` inside a patch). The envelope's `bound` block
  echoes the immutable `DesignId` every name resolved to.
- `design.check` returns exit 0 with nothing to report, exit 2 for reported findings and `undecided` stages, and
  exit 3 when a stage refuses with one of its owned codes (the exit policy of its block). An earlier draft reserved
  exit 3 for a design the compiler cannot read; BD-110, BD-244 and N14b-10 refuse at every stage. TARGET: stages 1
  to 6 under 2 s for a template-sized design.
- The appendix's example entry (J.1, repeated in N32a-02 as the illustration of the entry form) shows
  `design.check` with four refusal codes (101, 104, 107, 110). The row lists every code a call can answer, which
  is the 31 of its block; the example illustrates the form and is not the row.
- Options and values added after a row's first milestone (`--format glb` of `design.export`, with `frames` and
  `chord`, at M2; `--from` of `design.new`, and `--correspond`, the kind `resolve-sized` and `set-fill`'s value
  `{shell: sized}` of `design.edit`, at M3; `battery_age` of `design.check` and `include_variants` of
  `design.templates` at M6) are listed in the blocks with their milestone. Each enters the generated input schema
  when its extending bead lands (an additive change that keeps the major version); before that a call that
  passes one is an unknown parameter, `FROB-REF-802`. The exception is `design.import`'s `sdf` and `usd`: they are
  values of `format` from M1 and answer `FROB-REF-724` until their importer is linked (BD-065), as `--visuals
  glb` of `design.export` answers 724 before N14b-21 (N14a-25); so a caller learns that the mechanism exists
  behind a condition, and the importer's landing changes no schema. A listed code that is not yet raised is
  legal; a pending slug joins a list only under its assigned number.
- `design.bisect` returns the result record N28b-02 defines, so its output schema is that record's name,
  `frob.bisect/1` (DESIGN, the rule VERBS-autopsy states for result objects stored and cited under their own
  names: a second name `frob.design.bisect.out/1` would give one schema two identifiers).
- Path parameters: the document of `design.import`, the file form of `design.new` (`--file`), of `design.show`
  (a design file, and `--at @<path>`), of `design.check` (a design file, as NS-01 runs it) and of `design.view`
  (`<design-or-file>`), `--file` of `design.patch`, and `--args @<file>` of `design.edit` (`extract-module`)
  accept a path to a file, so the generator appends 734 and 828 to those seven rows (NJL-01's field rule);
  `design.export`'s `out` is a write path whose 734 is a chapter code of its block.
- From M3, `design.diff` is a verb with two kinds (read by default; the action `run-control` is `run`), added by
  N28c-44 with its option. The registry's count of rows with two kinds becomes 4 and of `read` rows one lower;
  N28c-44 updates `[counts]` in the same change (NJLx-51's count-mismatch lint). The MCP annotation `read_only`
  of `design_diff` is then false (NJL-01: set only when every action is `read`).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly twelve entries with `cluster = "design"` (7 `read`, 1 `run`, 4
      `mutate`), each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no
      second spelling.
- [ ] `frob-surface-diff parity --cluster design` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 12, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_design.sh` runs each verb of the cluster (twelve), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/design/<verb>.help.json` byte for byte for each of
      the twelve, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_design` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 12 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `design.view` is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`design.bisect`: 542, 543, 550, 561;
  `design.blame`: none; `design.check`: the 31 codes of its block, 168 among them and not 308; `design.edit`:
  101, 102, 103, 105, 106, 107, 108, 111, 114, 121, 123, 125, 126, and not 109, plus the
  `sized-fill-unsatisfiable` code once assigned; `design.patch`: 112 and the 31 codes of `design.check` (32
  codes); `design.new`: 102, 103, 111, 125, 808; `design.show`: 102, 103, 125, 808; `design.view`: 102, 103, 125,
  808, 892; `design.import`: 103, 122, 125, 734, 828, 832, plus the mass-dropped code once assigned;
  `design.export`: its stage-1 codes (108 among them), 731, 733, 734, 816, and not 122); the appended codes equal
  the rule for its kind and fields, with `estimate_only` on `design.check` (`estimable = true`), 808 on
  `design.bisect` through `sync_wait_ms`, and 734 and 828 on the seven rows with a file parameter.
- `bisect_and_import_inputs`: the generated `design.bisect` input schema has `good`, `bad`, `predicate`, `task`,
  `margin`, `level` (default 0.05), `via`, `scan_prefix`, `assume_monotone` and `require` and no positional design
  and no `from` or `to` (`frob design bisect quad-a --from d:41aa --to d:7c03 …` is 802); its output schema is
  `frob.bisect/1`; the generated `design.import` input schema has `format` (`urdf`, `mjcf`, `sdf`, `usd`, and
  `fsim-catalog` once NEG-32 lands), `name` (required), `strict`, `expanded_by`, `fold_fixed`, `reduced` and
  `builder`, and in a build without the SDF and OpenUSD importers `--format sdf` and `--format usd` each answer
  724, exit 5, `reason: feature-not-built`, never 802; the generated `design.check` input schema has `no_memo`
  (from `memo = true`, as `design.show`'s has), and its output schema's stage status enumerates exactly `passed`, `refused`, `undecided` and `not-run`; the
  generated `design.view` input schema has `pretty`; the generated `design.edit` input schema has, once their
  beads land, `name` and `args` (N41p2-01), `correspond` (N14ax-02) and `loads` (N31b-41), and its output schema
  `correspondence`; `design.export`'s has `frames` and `chord` and the `format` value `glb` once N14b-21 lands;
  the generated `design.templates` output schema has `aliases` per entry and `design.new`'s output has
  `template`, `preset` and `alias_used`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`design_bisect`, `design_blame`, `design_check`,
  `design_diff`, `design_edit`, `design_patch`, `design_templates`, `design_new`, `design_show`, `design_import`,
  `design_export`, `design_view`), with no collision against any other cluster.
- `check_policy`: against the real binary, the parallel-roller mecanum fixture returns exit 3 with 118 at stage 4,
  the point-foot biped template exit 2 with `A-08.no-static-stability`, and `quad-a` exit 0 (BD-110's legs);
  `frob design check quad-a --estimate --json` returns the planned stages with budgets and runs nothing.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`
  with `x-frob-key: "required"` and absent from the schema's `required` array, `budget`, `estimate_only`,
  `on_cancel` with the values `leave` and `cancel_job`, `envelope_scope`) and refuses `sync_wait_ms = 30001` with
  code 802; a `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with
  code 805; a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_design.sh` (header: `bead=VERBS-design milestone=M1 labels=tether,verbs,verb:design.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance design`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the twelve verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
design` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster design`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_design/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 12 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 12 verbs (7 `read`, 1 `run`, 4 `mutate`); the tables above. KA-755 (the exit-code
  table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the
  surface's tables; exact) is exercised for every verb by steps (6) and (7).
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

- Kill gate KILL-SKEL-SEXPR (the S-expression view cannot be kept lossless): fallback, JSON is the only syntax and
  `design.view` keeps its row with `--as json` only.
- `design.edit`'s symmetry effect follows BD-058 (`preserved`, `reported`, `grows`, `none`, with `symmetry.split`):
  an earlier draft printed `preserved`, `broken`, `restored`, which are not values of the declared effect; "broken
  if applied to one leg" in §14.7 is the `reported` case with the split constructors listed.
- `design.bisect` first answers at M4; `first-change` is refused without a prefix scan or a declared monotone
  predicate (`FROB-REF-550`), because verdicts along a lineage need not be monotone. Its spelling is BD-343's
  `--good`/`--bad` with `--predicate`; NS-06 already uses it. An earlier example of this row used a positional
  design with `--from`/`--to`, which no implementing bead accepts.
- `design.check`'s stage statuses follow N14b-10's wire forms; an earlier draft listed a fifth status
  `findings`, which is not a variant (a stage with reported findings is `passed` with `findings[]`).
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M4. The per-milestone
  surface beads (BD-137, BD-376) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §14.4, §14.7, §14.8, §14.10,
§14.11, §14.12, §28.7, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
