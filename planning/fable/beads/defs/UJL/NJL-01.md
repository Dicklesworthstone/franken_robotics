---
key: "NJL-01"
title: "frob-verbs: the derivation of a verb entry by kind: common inputs, appended refusals, actions, tool form"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-005", "N32-REF", "N32a-02", "N32a-03", "N34-REF", "N35-REF", "STD-E2E", "STD-TEST"]
needs: []
related: ["BD-022", "BD-023", "BD-137", "BD-138", "NJL-02", "NJL-03", "NJL-06", "N00-02", "N32a-09", "N36-20", "N33c-32", "N34-24", "N37-REF", "N28a-02", "N28c-14", "N41-12", "N32ax-02", "N32ax-01", "BD-111", "BD-139", "N21c-07", "N21c-35"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (preamble, J.1)", "§32.2", "§32.3 (ADR-TETHER-01, -02, -05, -09, -11)", "§32.6", "§32.7", "§32.8.2", "§32.12.3", "§32.12.5", "§32.12.6", "§32.19 (INV-TETHER-01, -07)", "§34.6"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through one command-line binary `frob`, a
daemon `frobd` and an MCP server `frob-mcp` (MCP: the Model Context Protocol, by which an agent harness lists and
calls tools). Every capability is a **verb**, written `cluster.verb` (`design.check`, `telemetry.inspect-pixel`).
All verbs are declared once, in `registries/verbs.toml`, owned by crate `frob-verbs` (layer 6, tier Solid). The CLI
subcommands, the daemon protocol, the MCP tool definitions, the input and output schemas, the refusal table and
the agent guide (`frob robot-docs`) are generated from that file, so the surfaces cannot drift. Gate
GATE-TETHER-02 (TARGET, claim Known answer, M1) states it: both surfaces are generated from the registry and
their parity diff is empty; every verb has a schema, an example and a refusal list; the served MCP `tools/list`
equals the registry under the name map.

The registry's data model has two halves, in two beads. N32a-02 defines the file, the fields of one entry, the
name rules, the build script, the emitted table and the lints that make a malformed registry fail the build. This
bead defines everything that is **derived** from an entry and seen by a caller: the input properties the
generator adds to a verb of each of the four kinds and by some of its fields, the refusals it appends, how a verb
with two kinds is selected and served, the tool's annotations, tags and description, and the rule by which a
registered verb that is not implemented yet answers. Together they are the contract every later row is written
against; every `VERBS-<cluster>` registry bead depends on this bead and, through it, on N32a-02.

DESIGN, the order of work. This bead comes after N32a-02 and **before** the two surfaces: the CLI of BD-022 builds
its flags, and the MCP server of BD-023 its tool definitions, by calling the members implemented here, so that
both read one derivation (ADR-TETHER-01). Reason (the direction rule for two beads that each seem to need the
other: the one that defines the interface comes first): if the CLI computed the common flags of a kind itself and
this bead computed them again for MCP, the program would have two lists of the same rule, which is the drift the
registry exists to prevent. The checks that need the running binaries are therefore stages of this bead's script
that later beads run (below), not acceptance criteria of this bead.

Decisions that govern it, with their reasons:

- **ADR-TETHER-01.** The registry is compiled into a static table in `frob-verbs` by its build script. Neither
  `frob-cli` nor `frob-mcp` contains per-verb code, with one exception (the MCP half of `session.focus`, which
  edits the visible tool set). Reason: parity by construction; the gate then checks the served surfaces, not the
  generator's intentions. Rejected: hand-written clap commands and hand-written `#[tool]` handlers per verb (two
  lists of every registered name, NJL-03's count, that drift); a registry read at run time (a deployment could then serve a surface the tests
  never saw).
- **ADR-TETHER-02.** `frob-verbs` has a feature `registry-only` with no dependency on any physics crate or on the
  FrankenSim airlock crate `frob-fsim`. Reason: it is what the first fallback of kill gate KILL-MCP links (the MCP
  server built as its own workspace must still share the table, the schemas and the envelope types). The
  derivation of this bead is part of `registry-only`.
- **ADR-TETHER-05.** A cancelled wait returns nothing and leaves the job running unless the call set
  `on_cancel: "cancel_job"`. Reason: a dropped connection must never be mistaken for a decision to stop work.
- **ADR-TETHER-09.** Integers that can exceed 2⁵³ − 1 travel as decimal strings; counts are range-checked numbers.
  Reason: a client that decodes JSON numbers as IEEE doubles silently changes larger values (2⁵³ + 1 =
  9,007,199,254,740,993 decodes to 9,007,199,254,740,992).
- **ADR-TETHER-11.** Mutating and job-submitting verbs bind immutable identifiers; a mutable name is resolved once,
  before the request is digested, and the resolution is echoed in the envelope's `bound` block. Reason: otherwise a
  retry after a name has moved would mean a different request under the same idempotency key.

Terms. *Envelope*: the single JSON object (`frob.envelope/1`) every verb writes with `--json`. *Refusal*: a typed
failure with a stable code `FROB-REF-nnn`, ranked repairs and an exit code. *Common property*: an input property
that the generator adds to an entry's input schema because of its kind, one of its fields or a registry table,
and that the row does not write. *Chapter list*: the `refusals` field of a row, the codes specific to the verb.
*Full list*: the chapter list plus every code the generator appends. *Task cluster*: a set of at most seven tools
an agent sees at a time (bead NJL-05); it differs from a registry cluster, which is a namespace. *Tier*: S
(Solid), F (Frontier) or M (Moonshot), the maturity class of the mechanism behind the verb.

## Scope

In: the four kinds as generator rules (the common input properties each kind adds, with their CLI spellings, and
the surface-wide refusals appended to each verb's list); the table of variations driven by fields and registry
tables, naming the bead that lands each arm; the derived members of an entry (`common_properties`,
`input_schema_with_common_fields`, `envelope_schema`, `all_refusals` with the source of each code, `short_doc`,
`tags`, `annotations`); the `action` property of an entry with an `actions` list, with the scope, annotation and
schema that follow; the per-surface wait default as served; the rule that whether a verb answers is decided by
its handler being in the build; the lint `common-property-shadowed`; the generated schema of one entry; the
script `scripts/e2e/verbs_registry_schema.sh` and the beads that run its stages.

Out, with the bead that has each: the registry file and its format, the TOML schema of one `[[verb]]` entry with
its field table, the name grammar, the CLI path and the MCP tool-name map, schema identifiers, the build script,
the emitted `VerbEntry` type and static table, every build-time lint other than `common-property-shadowed`, and
the `registry-only` feature (N32a-02); the envelope types and the function `envelope_schema(entry)` (N32a-03); the
CLI parser and its process options (BD-022), the MCP `VerbTool` (BD-023), which call this bead's members; the
answer `FROB-REF-724` itself, produced by dispatch (BD-022's fail-closed first form, then N00-02's registry-driven
rule); the refusal `FROB-REF-805`, raised by the admission transaction (N32a-09); the rows themselves (the
`VERBS-<cluster>` beads and NJL-03); the conformance harness that compares served surfaces with the registry
(NJL-02, with the parity diff of N32a-11); the alias rule (NJL-04); task-cluster assignment (NJL-05); idempotency
transactions, pagination and the bounded wait as mechanisms (N32a-09, BD-211, N32a-17, BD-298); the arms of the
variations whose field or table a later bead adds (the bead named in the variation table writes the arm and its
test).

## Technical approach

**One entry.** The form of a `[[verb]]` entry (its fields, their types and rules, the specimen `design.check`),
the name grammar and the tool-name map, the schema identifiers, the type the build script emits and the
build-time lints are defined in one place, N32a-02, and are not repeated here. A bead that needs a new field adds
it to the field table of N32a-02 with its lint; the entry schema is closed.

**Where the derivation lives (DESIGN).** `crates/frob-verbs/src/registry/derive.rs` holds pure functions over the
entry types of N32a-02's `registry/entry.rs`; it imports only the standard library, `frob-core` and those types,
never a handler, a surface crate or `frob-fsim`. N32a-02's build script compiles it as it compiles its own lint
modules, so the build-time checks that depend on the derivation (`RL-11`, the description length, and
`common-property-shadowed`) and the served surfaces use the same code. The order of every generated list is
fixed (kind properties in the order of the kinds table, then the variations in the order of their table;
action names sorted by name; refusal codes ascending), so the generated schemas are byte-stable (class D0).

**The four kinds.**

| Kind | Meaning | Common input properties the generator adds | Surface-wide refusals appended |
|---|---|---|---|
| `read` | Returns stored or cheaply computed facts and changes nothing | none (a paginated `read`: variation 1) | 720, 723, 724, 802 |
| `run` | Spends compute; takes an idempotency key and a budget; accepts the bounded synchronous wait and otherwise returns a job handle with exit code 8 | `sync_wait_ms`, `idempotency_key`, `budget`, `estimate_only`, `on_cancel`, `envelope_scope` | 701, 702, 703, 704, 720, 723, 724, 761, 802, 803, 805 |
| `mutate` | Creates or changes a named object in the caller's namespace; takes an idempotency key and an expected parent | `idempotency_key`, `expected_parent` | 720, 723, 724, 725, 802, 803, 804, 805 |
| `admin` | Cancels, stops, evicts or deletes; needs a capability scope | none | 720, 723, 724, 802 |

Code 808 is appended by variation 4 (an `int53` input), whatever the kind; every `run` entry has it through
`sync_wait_ms`.

The common properties, with their types and CLI spellings. The CLI flag of a property is its name in kebab case
(`expected_parent` → `--expected-parent`), with two exceptions: `idempotency_key` is `--key` and `estimate_only` is
`--estimate` (BD-022's flag table; the parity harness N32a-11 maps the two).
- `sync_wait_ms` (`int53`): from 0 to the entry's `wait.mcp_max_ms` (at most 30,000 by `RL-09`); MCP default
  `wait.mcp_ms` (5,000 in every row today); CLI default: wait to completion (`wait.cli = "to-completion"`), the one
  per-surface default the registry declares. A value below 0 or above the maximum is `FROB-REF-802`, exit 3.
- `idempotency_key` (string): present in the schema of every `run` and `mutate` entry and **never listed in the
  JSON-schema `required` array**; the annotation `x-frob-key: "required"` (or `"optional"` on a `key_optional`
  entry) carries the rule. DESIGN reason: with strict input validation (BD-023), a missing required property is an
  input-validation failure, `FROB-REF-802`, on MCP, so the plan's `FROB-REF-805` would be unreachable there and the
  two surfaces would answer a missing key differently; the admission transaction (N32a-09) refuses a missing key
  with 805 on both surfaces before anything is charged. Before that binding exists (M0) a missing key is not
  refused (BD-022).
- `budget` (object from unit to amount: `gpu_s`, `cpu_core_s`, …; CLI `--budget k=v,…`).
- `estimate_only` (boolean; CLI `--estimate`): returns CPU core-seconds, GPU seconds, memory and disk with an
  interval, and runs nothing. The estimate block a `run` verb returns also carries the optional top-level members
  `role` and `preferred_machines` of `frob.sim.estimate.out/1` (BD-219), filled by N21c-07's `role_of` from M2;
  in a build without it they are absent, never empty or guessed.
- `on_cancel` (string, `leave` or `cancel_job`, default `leave`): what an interrupted wait does to the job.
  `leave` leaves it running (ADR-TETHER-05); `cancel_job` makes the handler ask the daemon to cancel it (served by
  N32a-17). DESIGN spelling: the plan names only `cancel_job`; `leave` is the value BD-022's CLI table and N32a-17
  serve, and the earlier spelling `leave_running` is retired (it is a schema failure, 802).
- `envelope_scope` (string, optional; CLI `--envelope-scope <scope>`): the approved compute-envelope scope the job
  belongs to, of the form `gate:<id>`, `campaign:<id>` or `pretrain:<run>` (the scope forms of `admits` in
  `registries/compute_envelope.toml`, N21c-35); `adhoc` and any other form fail the schema (802), since no
  citation already means `adhoc`. The scope of the job's demand (N36-20) is, in this order: the token of a
  launcher it runs under (`frob-launch`, the campaign runner), this citation, a scope the verb derives from its
  own inputs (`train.family`), otherwise `adhoc`, whose class in force is E0. A cited scope that no approved,
  unexpired request row admits has the class in force E0 (N21c-35's `class_in_force`), so a job above E0 under
  it is 704 or 761 as below; a citation that contradicts the scope of the launcher's token is `FROB-REF-761`
  with `pointer: /envelope_scope`, `observed` the cited scope and `required` the token's (N35-REF). The citation
  grants nothing: no capability scope is added, the session's run scope and budget apply, and the envelope meter
  bounds what the scope consumes. Its codes, 704 and 761, are already in every `run` list. DESIGN (convention 14):
  the repair of 704 and 761 is "resubmit citing the approved scope", and an agent over MCP has no launcher, so
  without this input no agent could run above E0.
- `expected_parent` (identifier, optional): the lineage head the caller believes is current; given and stale is
  `FROB-REF-804` (N32a-09).

Meanings of the appended codes: 701 the estimate's upper end exceeds the remaining budget (exit 4); 702 budget
exhausted during a run, stopped at a safe boundary, nothing partial published (4); 703 admission guard, `retry:
backoff` (4); 704 a campaign-class job whose scope has no approved compute envelope, raised by the daemon's size
classifier when the job is submitted and again by the scheduler at dispatch and resume when the approval has
lapsed (4); 720 the token lacks a scope (5); 723 token invalid or expired (5); 724 the verb, or an option it
declares, is not implemented in this build (5); 725 a write outside the caller's namespace (5); 761 every other
envelope case: the job exceeds the compute envelope in force for its scope or an approved scope's per-run or
cumulative limit, or a cited `envelope_scope` contradicts the launcher's token (4); 802 arguments fail the schema,
including an unknown parameter or an out-of-range `sync_wait_ms` (3); 803 idempotency key reused with different
resolved content (7); 804 stale parent (7); 805 a mutating or job-submitting verb without an idempotency key (3);
806 cursor does not match the query (3); 807 cursor expired (3); 808 an integer outside ±(2⁵³ − 1) in a
number-typed field (3).

DESIGN, 761 on the `run` kind: the surface chapter's kind list names 701 to 704 only, but the compute-envelope
admission of N36-20 (`admit_run`) refuses any job whose needed class exceeds the class in force with
`FROB-REF-761`, before any queue, for every `run` verb on every surface; and the conformance check "a verb raises
only codes of its full list" (N32a-02, `RL-06`, which exempts only `FROB-REF-990`) would fail every `run` verb
the first time an over-envelope job is submitted. The code is therefore appended by kind, as 701 to 704 are,
and not written in every `run` row's chapter list. The two envelope codes are divided by raising site, so that
one oversized job gets one code (convention 14; N32-REF row 704, N35-REF row 761): 704 is the daemon's size
classifier's code when a `campaign`-class job whose scope has no approved envelope is submitted, and the
scheduler's again at dispatch and at resume when the approval it was queued under has lapsed (BD-296); every
other envelope case is 761, raised before the job starts. Both come before the session-budget check 701.

DESIGN, the internal fault: `FROB-REF-990` (N32a-03: a handler panicked or returned an unhandled error, exit 9) is
never appended to any generated list, by kind or by field, as N32a-02's `RL-06` never admits it in a chapter
list. Any handler can raise it, it describes a defect and not the question, and listing it on every row would add
the same words to every tool description; the envelope's exit table and `robot-docs` state it once, and the
conformance check "a verb raises only codes of its full list" exempts exactly this code.

**Variations driven by fields and tables** (DESIGN: one generator rule each, instead of 44 row beads repeating
properties and codes by hand). Each variation is one arm of `derive.rs`. An arm whose field or table a later bead
adds is written by that bead, in the change that adds the field, together with its test; until then no entry has
the field and no verb has the property, so no surface offers an option that does nothing. This table is the one
list of what the generator adds.

| # | Driven by | Adds to the input schema | Appends | Lands with |
|---|---|---|---|---|
| 1 | `paginated = true` on an entry whose default action is `read`, or on a `read` action (N32a-02's action member `paginated`, lint `RL-23`) | `cursor` (string), `limit` (`int53`), `follow` (boolean), belonging to that action | 806, 807, with that action as their source | this bead |
| 2 | `estimable` | `estimate_only` on every `run` entry whatever the flag (a `run` verb spends compute, and every verb that can cost more than a second must accept `--estimate`); on another kind only with `estimable = true`, without `budget` and without any 70x code, because it submits no job | none | this bead |
| 3 | `key_optional = true` (lint `RL-21`: only `session.focus`, and a `mutate` whose result identifier is the content address of its resolved request, such as `study.plan`) | `idempotency_key` stays, with `x-frob-key: "optional"` | 805 is not appended | this bead |
| 4 | an `int53` property at any depth of the generated input schema | none | 808 | this bead |
| 5 | an `actions` list (below) | `action` | the codes of each action's kind and fields | this bead |
| 6 | `needs_groups` non-empty (the airlock feature groups a verb requires) | none | 790 (a required foundation group is absent from this build; exit 5); an empty list never gets it | N33c-32, which adds the field, its member, its lint `needs-groups-known` and the test `registry_refusal_lists_generated` with a real group (`stats`) |
| 7 | `params`: a parameter of kind `artifact-input<F>`, `artifact-ref<F>` or `artifact-identity<F>` | none | 891 (`format-version-unsupported`: unknown major version or unknown required field, for an owner with no version code of its own); an `artifact-identity<F>` parameter also 892 (`view-is-not-identity`) | this bead (the `params` field is in N32a-02's closed table from M0); the served check is N34-24's `artifact_verbs_list_the_three_codes` |
| 8 | `params`: a parameter of kind `foreign-file` | none | 893 (`format-owner-bypass`) | this bead; served check N34-24's |
| 9 | `params`: a parameter that accepts a path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`; an `artifact-ref<F>` is an identity or a name, never a path: by N34-24's decision a parameter that accepts a name or an identity as well as a path is `artifact-input<F>`, and `artifact-ref<F>` is used only where no path can arrive) | none | 734 (`path-outside-roots`, the path jail, exit 5) and 828 (`code-payload-or-weight-format`, the dispatcher's sniff of N37-03, exit 3) | this bead; served checks N37-REF's `path_verbs_list_734` and `file_args_list_828` |
| 10 | a `requires` row for the verb in `registries/loop.toml` (the loop order of N28a-02) | `override` (array of stage names; CLI `--override <stage>[,<stage>]`) and `override_reason` (string of at most 500 characters; CLI `--override-reason`) | none: `override` without `override_reason` is 802 with `pointer: /override_reason`, already in every list | N28a-02 (M1), whose test `override_properties_generated` checks `sim.run` and `control.synth` have both and `artifact.export` neither; the build script reads the loop table for this arm |
| 11 | every `run` entry, from N28c-14 (M4) | `from_proposal` (string, the identifier of a proposal returned by `experiment.suggest`; CLI `--from-proposal <id>`): the prediction of the bought proposal is frozen in the admission transaction of whatever run verb the proposal's command names | 804, raised with `pointer: /from_proposal` when the proposal's inputs have changed since it was issued (DESIGN: N28c-14 names the condition a stale parent with exit 7 and gives no code; 804 is the registered code for exactly that) | N28c-14 |
| 12 | every `run` entry, from N41-12 (M4) | `machine` (string, `mac` or `trj`; CLI `--machine`): under `fleet.peer_link = false` a job addressed to the other machine is refused with the repair "submit on that machine" | the refusal `machine-not-local` (execution-site family 6xx, exit 6, N35x-02), cited through N41-12's constant `MACHINE_NOT_LOCAL`, which holds the number the allocation ledger N35-33 assigns, read from `docs/generated/refusal_round2.md`, the ledger run's output (N35x-02); this bead writes no number | N41-12, in the change that enters the code's row |
| 13 | an `engines` list naming more than `native` (N32a-02's later field `engines`, added by N32ax-02; values `native`, `reference`, `external`; default `["native"]`) | `engine` (string, a name in `registries/engines.toml`; CLI `--engine`) | the codes held by N32ax-02's named constants for `engine-environment-mismatch` (`pointer: /engine`) and `engine-model-unrepresentable` (`pointer: /engine/loss`), both execution-site family 6xx, exit 6 (N35x-02's rows); 724 is already in every list | N32ax-02 (M4, only if KILL-TENFOLD-TIE fires) |
| 14 | `memo = true`: the entry's executor is fronted by the memo graph (a field BD-111 adds to N32a-02's later-field table) | `no_memo` (boolean, default false; CLI `--no-memo`): nothing is served from the memo graph for this call in any store mode, every node is computed and still stored, and the envelope's memo disposition is `NoMemo` (BD-111, N28b-38); outside every identity, since a served product is bit-identical to a computed one | none | BD-111 (M1), which sets `memo = true` on the entries whose executors its wrapper fronts |
| 15 | `reports = true` (a report surface: BD-139's field in N32a-02's later-field table), or a `params` parameter of kind `artifact-input<F>`, `artifact-ref<F>` or `artifact-identity<F>` (an artifact reader) | none | 814, 816, 819, 873, 889, 895, 897: the refusals of the reality guard's `check` (BD-139, all exit 3), which every report renderer and artifact reader calls | BD-139 (M1) |
| 16 | the `[[admission_check]]` rows of `registries/verbs.toml` that name the verb (N32ax-01's table) | the derived member `admission_checks`; and, when at least one guarding check declares an override, `diagnostic_override` (array of those override names; CLI `--diagnostic-override a,b`) | each guarding check's `refusals` | N32ax-01 (M2), with its test `derive_admission_check_arm` |

A chapter list may still name a code that a variation appends (734 or 828 where the verb's own handler raises it
on another argument, such as an export's `out` or an importer's reader); the full list is a set. The export door's
codes (731, 733, 816, and 734 for `out`) stay in the chapter lists of the exporting verbs, which N37-18 and
N37-REF name as their place. DESIGN, variation 9: N37-REF's list test requires every verb with a path argument to
list 734, and N37-03 sniffs every file or blob argument before its reader; without the rule several dozen rows
(every verb that reads a patch, sweep, regime, mission or interface file) would each carry the two codes by hand,
and most did not. DESIGN, variation 4: a code is listed where it can be raised; 808 is a schema-validation
refusal of an `int53` field, so an entry without one cannot raise it (the earlier form listed it on every `read`,
`run` and `mutate` and on no `admin`, so an `admin` verb with a count could raise an unlisted code). DESIGN,
variation 1: it applies to the `read` kind only. A second call of a `run`, `mutate` or `admin` verb with a cursor
would be a second job or a second change, not the next page; a `run` verb's long result is read in pages through a
`read` verb. N32a-02's `RL-23` refuses `paginated = true` on an action that is not `read`, and an entry whose
default action is not `read` gets nothing from the field. DESIGN, variation 14: the memo bypass exists only where
the memo graph fronts the executor, so a verb the wrapper does not front offers no `--no-memo` that would do
nothing; a row that declared `no_memo` itself drops it when it sets `memo = true`, or `common-property-shadowed`
fails it. DESIGN, variation 15: the guard's `check` runs wherever an artifact that carries a reality status
arrives, so a verb that renders a report or reads an artifact can raise its seven codes and lists them.

**Codes whose registry lands later (DESIGN, the staging rule).** A list names a code only from the change in which
the code's row is in `registries/refusals.toml`: `RL-06` (N32a-02) makes an unknown code a load-time error, and a
code with no row cannot be raised. (1) At registration a row's chapter list holds only codes already entered. A
code of a later registry is appended to the row in the change of the bead that raises it, or of the registry bead
that enters it when that bead lands second, and the refusal-table and coverage checks count it from that change
(for example the M0 `sim.run` row's 103, which N14-REF enters at M1; VERBS-prove's 845; VERBS-run's 580 to 583
and 322). (2) The kinds table and variations 1 to 5, 7 and 8 append only codes that this bead's dependencies enter
(N32-REF, N35-REF, N34-REF); the arm a later bead writes appends codes that bead or its dependencies enter.
(3) Variation 9's 734 and 828 are N37-REF's, which depends on this bead: `derive.rs` appends each of the two only
when the refusal table holds its row, and N37-REF's checks `path_verbs_list_734` and `file_args_list_828` hold
from N37-REF's change.

**Verbs with actions (DESIGN).** Three inventory rows print two kinds: `nemesis.hardset` (read, run), `scene.bank`
(read, admin), `spark.cache` (read, admin); the cluster rows add nine that use the same form, `session.show`
(`audit`), `sense.symmetry` (`test`), `design.diff` (`run-control`, from M3), `ground.band` (`retire`),
`nemesis.audit` (`prepare`), `spark.check` (`list`, a paginated `read` action), `task.check` (`coupling`, from
M2), `eval.ledger` (`open`, `close`, `seal`) and `scene.split` (default `create`, a `mutate`; actions `add`,
`move-group`, `show`, `check`, `part-of`, `audit`). The list of record is the registry: the file's `[counts]`
counts every row with an `actions` list under `two_kinds` (NJLx-51, rule 4), while NJL-03's baseline states the
kinds as the specification prints them. How such an entry is written is N32a-02's (one `kind`, the kind of its default
action, and an `actions` list for the others: `actions = [{ name = "evict", kind = "admin", capabilities =
["spark:admin"] }]`). How it is served:
1. The invoked action is the input property `action`, a string whose values are the names in `actions` (sorted);
   absent, the default action runs. CLI `--action <name>`, by the kebab rule, so both surfaces have one spelling:
   no per-action flag (`--retire`, `--coupling`, `--open`) and no action word in a positional place (`frob scene
   bank evict`). DESIGN reason: BD-022 derives every flag from an input property, and the parity harness N32a-11
   maps `--action` to `action` with no exception; a per-action flag or word would be a second spelling on one
   surface only. A value a per-action flag carried (`--evict <key>`, `--seal <name>`) is an ordinary input
   property of that action, named by its row and described as belonging to the action (item 3).
2. The common properties, the scope and the appended refusals follow the kind and fields of the selected action.
3. The MCP input schema is the flat union of the properties of every action, each common property's description
   naming the actions it belongs to (DESIGN: flat schemas are what MCP clients render reliably; a `oneOf` keyed on
   `action` is not). A property given with an action it does not belong to is `FROB-REF-802` with `pointer` at it.
4. The full list is the union over the actions; each code's source names its action.
5. An action whose row says that dispatch chooses it from the resolved request (the `audit` action of
   `session.show`, chosen when the transcript asked for belongs to another session, N32bx-01) is still listed in
   `action`, and the scope check follows the action dispatch chose.

**Annotations, tags and description** (the MCP `Tool` fields that BD-023 reads from here).
- `read_only`: true only when every action, the default included, is `read`.
- `idempotent`: the entry's `idempotent` field, and false when any action is `run` or `mutate`.
- `destructive`: true for an `admin` entry with `destructive = true`, and for an entry with an `admin` action whose
  `destructive` member (N32a-02's action table, lint `RL-08`) is true, its default (DESIGN: the annotation is a
  hint to a client to confirm, and the safe default is to ask).
- `open_world_hint`: false, since every verb acts on the local store.
- `tags`: `cluster:<c>`, `kind:<k>` once for each distinct kind among the default action and the actions,
  `task:<t>` for each task cluster, `tier:S`, `F` or `M`.
- `short_doc`, the generated description, at most 800 characters (TARGET, checked at build time by N32a-02's
  `RL-11` on this function), in this order: the `purpose` sentence; the `use_when` sentence; `Cost: ` followed by
  the `static:` bound of `cost_model` or `estimated (<estimator>)`; `Refusals (FROB-REF-): ` followed by the full
  list as three-digit numbers in ascending order, comma separated; `Example: ` followed by the first example. This
  is the form the parity harness N32a-11 parses the served list from; the long form is the resource
  `frob://docs/verb/{name}`. `RL-11` is wired in this bead's change, because `short_doc` does not exist before
  it (N32a-02 reserves the lint's identifier): its planted registry and the test
  `rl11_description_801_chars_caught` are this bead's.

**Derived members (DESIGN sketch).** The emitted `VerbEntry` and the static table are N32a-02's.

```rust
pub struct CommonProperty {
    pub name: &'static str, pub cli_flag: &'static str, pub schema: Schema,
    pub key_rule: Option<KeyRule>,              // x-frob-key: Required | Optional, on idempotency_key only
    pub source: Source,                         // Kind(Kind) | Field(&'static str) | Table(&'static str)
    pub actions: Vec<&'static str>,             // the actions it belongs to; empty = the default action
}
pub struct ListedRefusal { pub code: RefusalCode, pub source: RefusalSource } // Chapter | Kind(Kind) | Field(&str), with the action
impl VerbEntry {
    pub fn common_properties(&self) -> Vec<CommonProperty>;  // the two tables above, in their fixed order
    pub fn input_schema_with_common_fields(&self) -> Schema; // the row's schema plus common_properties()
    pub fn envelope_schema(&self) -> Schema;           // N32a-03's envelope_schema(self): result specialized
    pub fn all_refusals(&self) -> Vec<ListedRefusal>;  // the full list as a set, ascending, each code's source
    pub fn short_doc(&self) -> String;                 // the description above, at most 800 characters
    pub fn tags(&self) -> Vec<String>;
    pub fn annotations(&self) -> Annotations;          // read_only, idempotent, destructive, open_world_hint
}
pub const PROCESS_OPTIONS: &[&str];  // "json", "session", "registries-root", "session-token-file"
```

`envelope_schema` is N32a-03's function exposed as a member, so that both surfaces reach it through the entry;
its specialization rule is N32a-03's and is not restated. `PROCESS_OPTIONS` lists the options every `frob`
invocation accepts whatever the verb (BD-022's `--json` and `--session`, N32ax-03's `--registries-root`, N32b-01's
`--session-token-file`); a bead that adds a process option adds it to this list.

**Lint `common-property-shadowed` (DESIGN).** Run by N32a-02's build script as a
lint of a later bead. It fails when a row's own input property, or one of an action's, has the name of a property
the generator adds to that entry or action, or a CLI flag equal to a generated flag, to a process option or to
`--help` (BD-022's parser built-in, which is not an input property). The message names the entry, the property
and what it shadows. It reads the embedded input schema after N32a-02's build script has removed the
`conformance_only` properties, so it judges the schema the build serves. Reason: `shield.check` and `shield.fit`
had an energy budget spelled `--budget` beside the generated compute budget (now `--energy-budget` in
VERBS-shield); an agent reading `budget` on one tool and `budget` on another must meet one meaning. The same rule
keeps a row from declaring `session` (the process option `--session` carries the caller's own session identifier,
and N32b-01 refuses another with 802), and from writing its own `limit`, `cursor` or `follow` while it is
`paginated`: a session other than the caller's is read with N32bx-01's `of_session` input, as `session.show`
does.

**A registered verb that is not implemented yet.** Rows are entered when their cluster is first registered,
whatever their `first_milestone`. While its handler is not in the build, a verb answers one envelope with `ok:
false`, `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json` lists it as absent; the answer
is produced by dispatch (BD-022's fail-closed first form, then N00-02's rule for a capability that is
`not-in-build` or `killed`). Reason: the agent guide and the tool list are generated once and stay stable across
milestones. Whether a verb answers is decided by its handler being in the build, never by comparing the build's
milestone with `first_milestone` (DESIGN, the rule NJL-03 and BD-137 share), and the derivation of this bead never
reads `first_milestone`: a reduced form that lands before the row's milestone (for example the M3 expert-grid
form of `experiment.suggest`, whose row says M4) answers, and `frob capabilities --json` lists it as implemented
with its restriction named by the handler (`form: "expert-grid"`, as BD-137 prints `form: "cpu"` for the M1 forms
of the study verbs). The field keeps the appendix's milestone, so the milestone histogram stays a known answer
about the appendix. The converse case, a row whose implementing bead lands after the appendix's milestone
(BD-137's rule keeps the field at the appendix value), answers `FROB-REF-724` until its handler is in the build.
NJL-02's capability-level coverage check treats the two cases differently (DESIGN): a verb past its
`first_milestone` with no handler fails that check, unless it is one of the declared late rows of NJL-02's closed
list `LATE_ROWS` (thirteen rows, from the four `session` verbs at M0 to `ground.excite` at M4, each with its
implementing bead and that bead's milestone), for which the expected `FROB-REF-724` answer satisfies the check
until the implementing bead's milestone and no longer.

The same rule holds for an option (DESIGN: an agent told 724 learns that the option exists and is absent from this
build, where 802 would read as a misspelling, and the help snapshot of a row stays the same across milestones). An
option, or an option value, that a row declares is in the generated input schema from the change that declares it;
while its implementing bead is not in the build, a call that uses it answers one envelope `FROB-REF-724` (`reason:
feature-not-built`, naming the option and its bead, exit 5, `retry: never`), never 802 and never a partial run.
A value outside the option's declared domain is 802. A `conformance_only` property is absent from the schema of
every build without the feature `conformance` (N32a-02's build script removes it), so such a build answers it with
802 as an unknown parameter. An option the row does not declare yet is an unknown parameter, 802, until the
change that declares it. This is one rule for every cluster: 724 or 802 is decided by whether the row declares
the option, never by the option's milestone. When a row declares an option that a later bead implements is a
choice its registry bead states once, in its notes: at the row's registration, so that the option answers 724
until that bead is in the build (preferred, for the reason above), or in the change that lands that bead, an
additive change (NJL-02) before which the option is 802. Either way no build accepts a declared option and
ignores it.

**Files.** `crates/frob-verbs/src/registry/derive.rs` (the members, the two tables, `PROCESS_OPTIONS` and the lint
`common-property-shadowed`) and the generated `schemas/frob.verbs.entry.1.json`, the JSON Schema of one entry
written from N32a-02's closed field table with a minimal valid example; `frob schema frob.verbs.entry/1 --json`
prints it once BD-022's `schema` verb exists. The registry file, the build script and the other lint modules are
N32a-02's.

**This bead's script and who runs it.** `scripts/e2e/verbs_registry_schema.sh` is owned here and drives `frob` and
`frob-mcp`, which this bead's dependencies do not build. By the program's rule a check that needs a later bead
belongs to that bead, so the stages are run green by: NJL-02 (M0, stages 1 to 3, 5 and 6, against the M0
binaries, as a stage of its registry-wide script); BD-023 (stage 4, already in its acceptance); BD-138 (M1,
stage 7, after the admission transaction of N32a-09 exists). A stage whose prerequisite is absent is logged with
`e2e_skip` naming the missing binary, verb or kind, never as a pass; with no `frob` binary the script exits 77.

## Acceptance criteria

- [ ] `derive.rs` implements the seven members, the two tables, `PROCESS_OPTIONS` and `common-property-shadowed`;
      a source scan shows it imports only the standard library, `frob-core` and N32a-02's entry types; it builds
      under `--no-default-features --features registry-only`; N32a-02's build script compiles it.
- [ ] The generator adds exactly the common properties of the kinds table and of variations 1 to 5, in the fixed
      order and with the CLI flags listed, and appends exactly the codes of those rows; `FROB-REF-990` is in no
      generated list.
- [ ] `idempotency_key` is in no JSON-schema `required` array; `x-frob-key` is `"required"` on a `run` and a
      `mutate` entry and `"optional"` on a `key_optional` one; the `mutate` set of a `key_optional` entry lacks 805.
- [ ] Under the generated schema of a `run` entry, `sync_wait_ms` 0, 5,000 and 30,000 validate and −1 and 30,001
      fail as 802; with `wait.mcp_max_ms = 10000`, 10,001 fails; `on_cancel` admits exactly `leave` (the default)
      and `cancel_job`, and `leave_running` fails as 802; `envelope_scope` admits `gate:…`, `campaign:…` and
      `pretrain:…` and refuses `adhoc` and an unprefixed name as 802, and is in no `required` array.
- [ ] 734 and 828 are appended by variation 9 only when the refusal table holds their rows; `RL-11` fails the
      build on a planted 801-character description.
- [ ] An entry with `actions` exposes `action`; the selected action's kind governs its properties, scope and codes;
      a property of another action is 802 with `pointer` at it; `read_only` is false when any action is not `read`.
- [ ] `common-property-shadowed` fires on each of its planted rows (below) and passes on the committed registry.
- [ ] `crates/frob-verbs/CONTRACT.md` states the kinds table with CLI spellings, the variation table with the bead
      that lands each arm, the action rule, the annotation rule, the description format and the not-implemented
      rule, beside the entry schema and the lints that N32a-02 documents there.
- [ ] The generated schemas (the entry schema, the input schemas with common fields, the envelope schemas) are
      byte-identical across two builds, debug and release, Linux and macOS (determinism class D0).
- [ ] `scripts/e2e/verbs_registry_schema.sh` parses (`bash -n`); under `E2E_DRY_RUN=1` it prints every command and
      exits 77 with a log that `frob-e2e lint` accepts; with no `frob` binary it exits 77, never 0.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test registry_schema` (no mocks; fixture registries are TOML strings labelled
`synthetic`; the derive functions are called on entries parsed with N32a-02's strict parser, so appended codes
are compared as numbers, and that every code resolves in `registries/refusals.toml` is `RL-06` on the committed
registry):
- Tests that moved with their subject to N32a-02: `entry_specimen_parses`, `name_map_examples`,
  `name_map_injective_on_registry`, `lint_<name>` for each of its lints, `prop_entry_roundtrip`.
- `appended_refusals_by_kind`: a `read` with string inputs only gets {720, 723, 724, 802}; a `run` gets {701, 702,
  703, 704, 720, 723, 724, 761, 802, 803, 805, 808} (808 through `sync_wait_ms`); a `mutate` with string inputs
  gets {720, 723, 724, 725, 802, 803, 804, 805}; an `admin` gets {720, 723, 724, 802}, and 808 as well when it has
  an `int53` input; a `mutate` with `key_optional = true` (`study.plan`, `session.focus`) gets the `mutate` set
  without 805.
- `common_properties_by_kind`: a `run` has exactly `sync_wait_ms`, `idempotency_key`, `budget`, `estimate_only`,
  `on_cancel`, `envelope_scope` in that order with flags `--sync-wait-ms`, `--key`, `--budget`, `--estimate`,
  `--on-cancel`, `--envelope-scope`; a `mutate` exactly `idempotency_key`, `expected_parent` (`--key`,
  `--expected-parent`); a plain `read` none; a `read` with `estimable = true` only `estimate_only`; a paginated
  `read` exactly `cursor`, `limit`, `follow`; a `run` entry with `paginated = true` gets no `cursor`, `limit` or
  `follow` and no 806 or 807.
- `envelope_scope_schema`: `gate:GATE-SPARK-11`, `campaign:rank-agreement` and `pretrain:r-1` validate; `adhoc`,
  `rank-agreement` and `gate:` fail with 802; the property is optional and adds no code beyond the `run` set.
- `paginated_read_action`: a `synthetic` `spark.check`-shaped entry (default `run`, action `list` of kind `read`
  with `paginated = true`) has `cursor`, `limit` and `follow` described as belonging to `list`, and 806 and 807
  with source action `list`; the default action gets none of them.
- `key_never_schema_required`: for a `run`, a `mutate` and a `key_optional` entry, `required` lacks
  `idempotency_key`, and `x-frob-key` is `required`, `required`, `optional`.
- `on_cancel_values`: enum exactly `["leave", "cancel_job"]`, default `leave`; `leave_running` fails validation.
- `sync_wait_bounds`: 0, 5,000 and 30,000 validate; −1 and 30,001 fail with code 802; an entry with
  `wait.mcp_max_ms = 10000` refuses 10,001 and has default `wait.mcp_ms`.
- `int53_drives_808`: a `read` with an `int53` property `count` gets 808; one whose only integer is nested in an
  object gets 808; one with strings and a `u64str` only does not.
- `field_appended_codes`: a parameter of kind `artifact-identity<design>` gets 891, 892, 734 and 828; one of kind
  `foreign-file` gets 893, 734 and 828; `artifact-input<task>` gets 891, 734 and 828; `artifact-ref<run>` gets
  891 only; a chapter list that already names 828 yields one 828; each appended code's source is `field:params`
  (on a `synthetic` refusal table that holds 734 and 828). `appended_only_when_entered`: on a refusal table
  without the rows of 734 and 828, the same parameters get the same lists less those two.
  (Variation 6 is tested by N33c-32 with the real group `stats`, since its field does not exist before it; the
  group name `gpu-cuda` is not one of the ten airlock groups and fails its lint.)
- `internal_fault_never_appended`: over the synthetic matrix of the four kinds and variations 1 to 5 and 7 to 9,
  no full list contains 990.
- `two_kind_actions`: `spark.cache` with `actions = [{ name = "evict", kind = "admin", capabilities =
  ["spark:admin"] }]` has `action` with values `["evict"]`; `evict` requires `spark:admin`, the default does not;
  `read_only` false, `destructive` true, tags `kind:admin` and `kind:read`. A `synthetic` `nemesis.hardset`-shaped
  entry (default `read`, action `run`) has the six `run` properties in its flat schema, each described as
  belonging to the `run` action; `sync_wait_ms` given with the default action is 802 with `pointer:
  /sync_wait_ms`; its full list holds 805 with source `kind:run` and action `run`.
- `annotations_rule`: `read` → `read_only` true, `idempotent` true, `destructive` false; `admin` with
  `destructive = true` → `destructive` true; `mutate` → `read_only` false, `idempotent` false;
  `open_world_hint` false on every entry.
- `shadow_lint_catches`: a `run` row with its own input `budget` fails naming `budget` and the kind; a `mutate` row
  with an input `key` fails naming the flag `--key` of `idempotency_key`; a `read` row with an input `json`, and a
  `session.show`-shaped `read` row with its own input `session` (the known case, before that row took
  `of_session`), each fail naming the process option; a `read` row with an input `help` fails naming `--help`; a
  paginated `read` with its own `limit` fails; an entry with `actions` and its own input `action` fails; a `run` row with its own input
  `envelope_scope` fails; a property named in the row's `conformance_only` is judged in a conformance build only;
  the same rows with the input renamed (`energy_budget`) pass.
- `short_doc_format`: for the specimen `design.check` (a `read`, chapter list 101, 104, 107, 110, string inputs) the
  description begins with its `purpose`, contains `Refusals (FROB-REF-): 101, 104, 107, 110, 720, 723, 724, 802`
  and ends with `Example: frob design check quad-a --json`, and is at most 800 characters; a `synthetic` entry
  whose description would be 801 characters is reported by the function's length check.
  `rl11_description_801_chars_caught`: N32a-02's build script, run on a planted registry holding that entry, fails
  with `description-too-long` naming the row.
- `derived_members_complete`: for the specimen entry, `all_refusals` is ascending, a set, and each code carries
  its source; `tags` is exactly `cluster:design`, `kind:read`, `task:author`, `task:places`, `tier:S`.
- `envelope_schema_delegates`: `entry.envelope_schema()` equals N32a-03's `envelope_schema(entry)` byte for byte.
- `derivation_ignores_first_milestone`: two entries differing only in `first_milestone` give identical derived
  outputs.
- `entry_schema_file`: `schemas/frob.verbs.entry.1.json` validates the specimen and its own minimal example and is
  regenerated byte-identically.
- `prop_derivation_closed` (property test through `testkit::prop`, 256 cases, `synthetic` entries drawn over kind,
  `paginated`, `estimable`, `key_optional`, up to three actions, parameter kinds and the presence of an `int53`
  field): the generated input schema has no duplicate property name and no duplicate CLI flag; the full list is a
  set that contains the chapter list and never 990; the derivation is a pure function (the same entry gives the
  same bytes); permuting the `actions` list leaves every output unchanged.
- The integer lint (`schema_integer_types_only`, KA-756's control) is N32a-07's test.

### End-to-end script and logging

`scripts/e2e/verbs_registry_schema.sh` (header: `bead=NJL-01 milestone=M0 labels=tether,verbs,registry
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp plants=omit-805,unimplemented-as-unknown`). Stages: (1) `frob --json
schema frob.verbs.entry/1`, assert the example validates; (2) `frob --json capabilities`, assert every listed verb
has kind, tier, milestone and at least one example; (3) for one registered verb of each kind, chosen from `frob
--json robot-docs`, `frob --json <cli path> --help`, and assert that its input schema has exactly the common
properties of its kind and variations, with `x-frob-key` where expected, and that its refusal list equals the full
list with sources (a kind with no registered verb at the milestone, such as `admin` at M0, is a logged skip
naming the kind); (4) over MCP (in-memory transport and stdio), `tools/list`, and assert each tool name is the
mapped name of exactly one registry verb and its `inputSchema` and annotations equal stage 3's; (5) a verb that
`frob --json capabilities` lists as not in the build (at M0 `session show`, declared by BD-022) →
`e2e_expect_refusal FROB-REF-724 5`, a skip when no verb is absent; (6) on the test-lane binary built with
BD-022's `synthetic-verbs`, `frob --json synthetic deferred --sync-wait-ms 30001 --key "$(e2e_key wait-over)"` →
`e2e_expect_refusal FROB-REF-802 3`, and the same with `--on-cancel leave_running` → `e2e_expect_refusal
FROB-REF-802 3`; (7) `frob --json session focus mission` with no key succeeds, and `frob --json session open
--scopes design:read` with no key is `e2e_expect_refusal FROB-REF-805 3`. Run by: NJL-02 for stages 1 to 3, 5 and
6; BD-023 for stage 4; BD-138 for stage 7. The negative-control lane that builds `frob-verbs` on planted
registries and expects the named lint is N32a-02's script `scripts/e2e/tether_registry.sh` (its plants
`collision`, `unknown-field` and `bare-int`). This script's own plants: `omit-805`, a test-lane build whose
generator omits 805 from `run` verbs, which stage 3 must catch; `unimplemented-as-unknown`, a test-lane build
whose dispatch answers a not-in-build verb with 801, which stage 5 must catch (the dispatch mutant is N00-02's).
Logs go to `target/e2e-logs/verbs_registry_schema/<utc>.ndjson` through the shared helper: every command with exit
code and wall time, the envelope's evidence block, each assertion with observed and expected values, each
refusal with its code, each skipped stage with its reason, and the final `summary`.

### Known answers and negative controls

- KA-756 (2⁵³ + 1 as `int53` is refused with `FROB-REF-808`, preserved exactly as `u64str`; exact; control: a
  schema with a bare integer type fails the schema lint). CX-348 (send 64-bit integers as JSON numbers) is the
  counterexample behind the `integer-type` lint.
- Known answer of this bead: the appended codes and common properties of each kind and variation, as the two
  tables list them, and the description of the specimen `design.check` above. Held by N32a-02 with the name rules:
  the name map of its three examples, and the collision of `robot-docs` with `robot.docs`.
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; no verb, flag, schema or
  refusal exists on one surface and not the other, except per-surface defaults declared in the registry and the
  unknown-verb case) as the property `prop_derivation_closed`, the property `name_map_injective_on_registry` of
  N32a-02 and the parity harness of NJL-02; INV-TETHER-07 (no integer travels as a JSON number unless its schema
  type is `int53` and its value is within ±(2⁵³ − 1)) as `schema_integer_types_only` and `int53_drives_808`.
- Negative controls of this bead, each a mutant of `derive.rs` built with `gauntlet-mutations` (the mutation guard
  of BD-025 forbids them in a release build) and registered for GATE-TETHER-02 with the next identifiers of the
  control registry: omitting 805 from `run` (caught by `appended_refusals_by_kind` and by stage 3); writing
  `idempotency_key` into `required` (caught by `key_never_schema_required`); appending 990 by kind (caught by
  `internal_fault_never_appended`); a shadowing row (caught by `shadow_lint_catches`); an unimplemented verb
  answering "unknown verb" (caught by stage 5). The planted registries of N32a-02's lints, and the rule that a
  build with the override variable set fails the generated-file check, are N32a-02's.
- Metamorphic relation: permuting the entries of the file, or the `actions` of an entry, leaves every derived output
  unchanged.
- Determinism lanes: twice on one machine; debug and release; Linux and macOS; class D0 (byte equality of the
  emitted schemas and descriptions).

## Considerations

- The fields `purpose`, `use_when`, `owner`, `wait` and `actions` are DESIGN additions to the entry printed in the
  appendix; they are confined to what the generated surfaces already need. Their one definition is N32a-02's
  field table, where the spellings of the two drafts were reconciled to these.
- Capability scope names in rows are proposals until the scope catalogue `registries/scopes.toml` exists (the
  token bead N32b-01, M2); `unknown-scope` (lint `RL-15`, N32a-02's) is therefore a warning until M2 and an error
  after, and the capability bead BD-258 adds the `<cluster>:admin` scopes to the same file at M3.
- The registry is a build input of every binary; the derivation runs inside the build script, so it must be cheap.
  Budget: the whole build script, derivation included, under 1 s for 300 entries.
- The variations of later beads (6 and 10 to 16) change the input schema or the lists of many verbs at once.
  Each adds an optional property or a listed code, which the schema-snapshot classification of NJL-02 treats as
  additive (a new optional input property, a new appended refusal code); a property that became required would be breaking and need a new
  major schema version.
- Variation 4 lists 808 only where an `int53` field exists. If a later schema type made another field
  range-checked as a number, the arm grows with it; fail-closed default: a field whose type the arm does not
  recognize is treated as `int53` for this rule (listing a code that cannot be raised is harmless; omitting one
  that can is a conformance failure).
- OPEN-TETHER-3 (owner: Tether lead; blocks GATE-TETHER-13): the default and maximum of `sync_wait_ms` against
  measured agent turn time. Fail-closed default 5,000 and 30,000; they are registry values, so a change is one
  edit.
- If kill gate KILL-MCP fires (INT-FMCP-01 cannot pass within the M0 budget), the registry and this bead are
  unchanged: the socket fallback links `frob-verbs` with `registry-only`, which includes the derivation.

## References

Appendix J (preamble, J.1), §32.2, §32.3, §32.6, §32.7, §32.8.2, §32.12.3, §32.12.5, §32.12.6, §32.18, §32.19,
§34.6; GATE-TETHER-02; ADR-TETHER-01, -02, -05, -09, -11; INV-TETHER-01, -07; KA-756; CX-348; OPEN-TETHER-3.
