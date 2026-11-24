---
key: "VERBS-module"
title: "frob-verbs registry: cluster `module`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "skeleton", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: ["N14-REF", "N14b-28", "N15b-02", "N34-24"]
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§14.6", "§14.16", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `module`. The cluster covers reusable modules with port
contracts: the catalog, a module's ports, parameters, guarantees and assumptions, the check of its local
certificates, substitution at a path, and the capability table used by the composed quasi-static screen. It has five
verbs (3 `read`, 1 `run`, 1 `mutate`); the handlers live in `frob-analyze`, `frob-design`. The bead carries every
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

In: the five registry entries of cluster `module`, which this bead alone writes into `registries/verbs.toml`
(the implementing beads write no entry: N15b-02, for `module.check`, `module.list` and `module.show`, builds the
handlers and the contents of the schemas the rows name, and depends on this bead); the MCP tool names; the
cluster's conformance script; the help-schema snapshots; each verb's answer before its milestone. Out: behaviour
(the implementing beads named below: handlers, the fields of each input and output schema, the envelope each verb
returns, which refusal each condition raises);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `module.check` | read | M1 | S | Check a module's ports, parameter schema, local certificates and contract digest | §14.6 |
| `module.list` | read | M1 | S | Modules in the catalog, by content hash | §14.6 |
| `module.show` | read | M1 | S | A module's ports, parameters, guarantees and assumptions | §14.6 |
| `module.substitute` | mutate | M1 | S/F | Replace a module at a path; reports leaf-local or global and what is reused | §14.6 |
| `module.characterize` | run | M3 | F | Characterize one module's capability table for the composed quasi-static screen | §14.16 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`. As everywhere, the class is that of the weakest input: for `module.check` a
  guarantee that reads a card field marked `assumed` makes the result Estimated and names that field (N15b-02).
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`module.check`** (MCP tool `module_check`; owner `frob-design`)
- Inputs (N15b-02): a module reference (a name, a `name@b3:hash`, or a file path inside the caller's namespace);
  optional `--bind id=value` (check at a parameter point); optional `--in-design <design> --at <port>` (also run the
  junction checks of N15b-01 against that mount: the only way 101 for a join, a copied port, a loop or an unmet
  assumption can arise here).
- Outputs: ports, parameter schema, local certificates and the contract digest, each checked (`ports_ok`,
  `params_ok`, `certificates[]`, `digest {stored, recomputed, match}`, `joins[]` with `--in-design`,
  `findings[]`). Evidence: *static*.
- Chapter refusals: `FROB-REF-101` (port type mismatch; joint class and screw basis disagree); `FROB-REF-103`
  (unknown, misspelt or duplicate field); `FROB-REF-108` (power sign mismatch, copied power port or algebraic
  loop); `FROB-REF-111` (a lever outside its bounds, raised for a `--bind` value outside a parameter's bounds);
  `FROB-REF-114` (module digest or port signature mismatch on import or substitution); `FROB-REF-121` (contact
  law missing or incompatible with its end effector); `FROB-REF-125` (unsupported schema version).
- Implemented by: BD-060, N15b-02.

**`module.list`** (MCP tool `module_list`; owner `frob-design`; `paginated = true`: the generator adds `cursor`,
`limit`, `follow` and appends 806 and 807)
- Inputs (N15b-02): optional name prefix; optional port-kind filter; the generated page cursor.
- Outputs: modules in the catalog, by content hash: rows of `{name, ref (name@b3:hash), port_signature, tier,
  card_families}`, paged against a committed frontier and never cut silently. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-060, N15b-02.

**`module.show`** (MCP tool `module_show`; owner `frob-design`)
- Inputs (N15b-02): a module reference: a name, a `name@b3:hash`, or a file path inside the caller's namespace; a
  name without a hash resolves to the catalog entry and `bound` reports the hash it resolved to.
- Outputs: ports, parameters (unit, default, bounds), guarantees and assumptions; the contract digest; whether
  `bus_demand` can exclude (false when it has no time-resolved profile). Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-114` (the referenced hash does not match the file's bytes); `FROB-REF-125`
  (unsupported schema version).
- Implemented by: BD-060, N15b-02.

Refusal lists of the three read verbs (DESIGN). The chapter's text names two codes for the module contract
(108 and 114); the handlers of N15b-02 also load and validate a module file, so they can raise the format's own
codes. The registry lint fails a verb that raises a code missing from its row, so the rows list every code a
handler can raise: seven for `module.check`, two for `module.show`, none for `module.list`. An earlier form of
these rows, with 108 and 114 for `module.check` and none for `module.show`, is replaced.

**`module.substitute`** (MCP tool `module_substitute`; owner `frob-design`)
- Inputs: design; `--at <path>`; `--with <module@hash>` (the replacement module pinned by content hash); optional
  `--bind id=value` (repeatable, a parameter of the replacement); the expected parent (the generated `mutate`
  property `expected_parent`, `--expected-parent` on the CLI); idempotency key.
- Outputs: the new `DesignId`; `leaf-local` or `global` with the failed condition; the table of products reused,
  recomputed and invalidated. Evidence: *static*.
- Chapter refusals: `FROB-REF-108` (power sign mismatch, copied power port or algebraic loop); `FROB-REF-111` (the
  path lies under a lock, or a `--bind` value outside a parameter's bounds); `FROB-REF-114` (module digest or port
  signature mismatch on import or substitution). A stale expected parent is the appended `FROB-REF-804` (exit 7).
- Correction (round 1): the row formerly listed `FROB-REF-112`, which the design-language refusal registry (N14-REF)
  reserves to the patch reader of `design.patch` (a stale `parent` named inside a patch document); the
  substitution's lineage write reports a stale parent as 804, and its lock and bounds checks raise 111, which the
  implementing bead BD-077 lists and the row lacked.
- Tier: `S/F` (round 3; formerly S): Solid for the format and port contract, Frontier for the reuse of
  certificates and analyses under substitution (BD-077's statement; the plan's Frontier item "Certificate and
  analysis reuse under substitution [F]", §14.6), with the kill gate KILL-SKEL-SUBST below.
- Implemented by: BD-077.

**`module.characterize`** (MCP tool `module_characterize`; owner `frob-analyze`)
- Inputs (N14b-28): module; optional `--box <file>` (the operating box; default the module's declared box),
  `--directions <n>` (sampled wrench directions, default 26), `--strict-data` (refuse rather than return an
  `Estimated` table when a card field the torque envelope needs is `assumed`); idempotency key `--key`.
- Outputs: the module's capability table: its identifier, the box, counts of cells and directions, a per-cell
  summary (largest and smallest support value), outer and inner wrench sets (labelled `outer` and `inner`) and the
  validity record; cached by content, so an edit inside one module re-characterizes that table only. Evidence:
  *static* (Estimated, with the field named, when a field the table rests on is `assumed`).
- Chapter refusals: `FROB-REF-114` (the module's digest or port signature does not match the reference given;
  repair: name the digest the catalog holds, shown by `frob module show`); `FROB-REF-113` (under `--strict-data`, a
  card field the torque envelope needs is `assumed`; repairs: a card version in which the field is measured or
  stated, or the same call without the flag). Both are the design-language registry's codes (N14-REF).
- Tier: Frontier (the plan marks capability tables [F], §14.16; N14b-28 carries the pilot and the kill gate
  KILL-SKEL-CELL, decided at M4, whose fallback keeps the tables as a sizing report, so the row is unchanged
  either way). The row formerly said S.
- Implemented by: N14b-28.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `module.check` | `module:read` | `frob.module.check.out/1` | `modules` | `frob module check std/leg-3r-qdd --json` |
| `module.list` | `module:read` | `frob.module.list.out/1` | `modules` | `frob module list --json` |
| `module.show` | `module:read` | `frob.module.show.out/1` | `modules` | `frob module show std/leg-3r-qdd --json` |
| `module.substitute` | `module:write` | `frob.module.substitute.out/1` | `modules` | `frob module substitute quad-a --at /limbs/of/of --with std/leg-3r-qdd@b3:9a4e --expected-parent d:7a3e --key sub-1 --json` |
| `module.characterize` | `module:run` | `frob.module.characterize.out/1` | `audit` | `frob module characterize std/leg-3r-qdd --key mc-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "module.characterize"
cluster = "module"
kind = "run"
tier = "F"
capabilities = ["module:run"]
input_schema = "frob.module.characterize.in/1"
output_schema = "frob.module.characterize.out/1"
refusals = ["FROB-REF-113", "FROB-REF-114"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M3"
task_clusters = ["audit"]
examples = ["frob module characterize std/leg-3r-qdd --key mc-1 --json"]
owner = "frob-analyze"
purpose = "Characterize one module's capability table for the composed quasi-static screen"
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
   In this cluster the module reference of `module.check`, `module.show` and `module.characterize` may be a file
   path inside the caller's namespace (N15b-02), and `module.characterize`'s `--box` takes a file: these are
   declared in `params` (N34-24), so the generator appends 891, 734 and 828 to those three rows.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `module.substitute` reports `leaf-local` exactly when four conditions hold (same port signature; the replaced
  module is a leaf with no closure to another branch; shared resources reached only through declared electrical and
  thermal ports with assumptions met; each reused product read only fields outside the replaced subterm); otherwise
  `global`, which invalidates everything downstream of the model identity.
- A contract that carries only peak demands can admit and cannot exclude; the bus analysis runs on time-resolved
  profiles before any design is excluded on electrical grounds.
- Round 3: `module.characterize` gained `--strict-data` and the codes 113 and 114 of N14b-28 and its tier F;
  `module.list` is paginated (N15b-02 pages it against a committed frontier), so it carries 806 and 807;
  `module.check` names `--bind` (already the cause of its 111) and `--in-design … --at` (the cause of its 101).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly five entries with `cluster = "module"` (3 `read`, 1 `run`, 1 `mutate`),
      each with every field of the entry schema, an example and a task-cluster list; the alias lint finds no second
      spelling.
- [ ] `frob-surface-diff parity --cluster module` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 5, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_module.sh` runs each verb of the cluster (five), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/module/<verb>.help.json` byte for byte for each of
      the five, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_module` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 5 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `module.characterize` is `run` at M3, tier F; `module.substitute` is `mutate` at M1, tier `S/F`).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`module.check`: 101, 103, 108, 111, 114, 121, 125;
  `module.list`: none; `module.show`: 114, 125; `module.substitute`: 108, 111, 114; `module.characterize`:
  113, 114); the appended codes equal the rule for its kind and fields (`module.list` is paginated and also gets
  806 and 807; the rows with file-path `params` get 891, 734 and 828); a planted 112 on `module.substitute` fails
  against the raising sites of N14-REF (112 belongs to `design.patch`).
- `options_by_row`: the generated input schemas accept the options of each block (`module.check`: `--bind`,
  `--in-design`, `--at`; `module.list`: the prefix and port-kind filters and the generated `cursor`, `limit`,
  `follow`; `module.substitute`: `--at`, `--with`, `--bind`, `--expected-parent`; `module.characterize`: `--box`,
  `--directions`, `--strict-data`); `module.check --in-design d:… ` without `--at` is 802.
- `characterize_strict_data`: on the synthetic module whose card has an `assumed` torque-envelope field,
  `--strict-data` refuses with 113 naming the field and the same call without it returns a table of class
  `Estimated` naming it (the case N14b-28 tests, run here through the generated surface).
- `mcp_names_one_to_one`: the tool names are the mapped names (`module_check`, `module_list`, `module_show`,
  `module_substitute`, `module_characterize`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of convention 2 (`envelope_scope` among them) and refuses
  `sync_wait_ms = 30001` with code 802; a
  `mutate` row has `idempotency_key` and `expected_parent` and refuses a call without the key with code 805; a verb
  whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_module.sh` (header: `bead=VERBS-module milestone=M1 labels=tether,verbs,verb:module.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field,dropped-option`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance module`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the five verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
module` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster module`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1; (8) for each verb, every option named in its block
appears in `frob <verb> --help --json` (logged as `option_present` per option with the block's bead).

Log: `target/e2e-logs/verbs_module/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 5 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 5 verbs (3 `read`, 1 `run`, 1 `mutate`); the tables above. KA-755 (the exit-code
  table on malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the
  surface's tables; exact) is exercised for every verb by steps (6) and (7).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`);
  `dropped-option` (`--strict-data` removed from the `module.characterize` entry: step (8) fails naming the verb,
  the option and N14b-28). A verb answering with two envelopes or exit code 1 fails step (7).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Kill gate KILL-SKEL-SUBST (reuse under substitution does not hold its gate): fallback, every substitution is
  reported `global`; the row and schema are unchanged.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M3. The per-milestone
  surface beads (BD-137, BD-312) re-run this script as verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names follow the naming rule decided in BD-258 (no longer proposals): a row cites scopes of its own
  cluster by kind (`read` → `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` →
  `<cluster>:write` or a named write scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope),
  plus `export:*` when the row exports and `private:*` where privacy applies, never a scope named after the
  object read or the engine spent on; N32b-01's catalogue generates the forms. `<cluster>:run` and
  `<cluster>:write` imply `<cluster>:read`, closed when a set is stored (at minting and at derivation,
  `Scopes::close_implied`), so checks stay plain set inclusion.
  For the three read verbs the scope is `module:read` (DESIGN): the one rule above applied to the cluster, as
  the plan's specimen entry applies it to `design.check` with `design:read`. The implementing bead N15b-02 first
  wrote `design:read` for them; `module:read` is kept because a module is read from the catalog with no design
  of the caller's involved, and because one rule for all 44 clusters is what the surface-parity check compares.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows and J.3 added rows; J.5 task clusters), §14.6, §14.16, §32.4, §32.6,
§32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
