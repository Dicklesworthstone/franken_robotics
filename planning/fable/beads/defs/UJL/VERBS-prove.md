---
key: "VERBS-prove"
title: "frob-verbs registry: cluster `prove`, five rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "tether", "lemma", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "N12-REF", "N37-REF", "NJL-01", "NJL-02", "NJL-04", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§12.3", "§12.4", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)", "§22.5 (control-program certificates)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `prove`. The cluster covers certificates: the instance
certificate of a kernel, the list of certificates and theorem cards, the in-tree check of a bundle, assembling a
`.frobcert` bundle for a claim, and promotion of an instance certificate through a reflected checker. It has five
verbs (3 `read`, 2 `run`); the handlers live in `frob-cert`, `frob-lemma`. The bead carries every row completely,
with the three checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb
has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the five registry entries of cluster `prove`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `prove.bundle` | run | M1 | S | Assemble a `.frobcert` bundle for a claim (mission slice at M1, full at M5) | §12.4 |
| `prove.kernel` | read | M0 | S | The instance certificate of a kernel and what it states | §12.4 |
| `prove.list` | read | M0 | S | Certificates and theorem cards with their classes and checkers | §12.3 |
| `prove.reflect` | run | M5 | F | Promote an instance certificate through a reflected checker [F] | §12.4 |
| `prove.verify` | read | M0 | S | Re-check certificates in tree; `frob-verify` repeats the check offline | §12.4 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`prove.bundle`** (MCP tool `prove_bundle`; owner `frob-lemma`)
- Inputs: one claim source: a mission answer `a:…` (positional; the mission slice, M1, BD-133); or `--design
  <id>` with `--receipt <hash>` (a qualification receipt, whose campaign section the bundle gains, before or after
  the campaign's close) and optionally `--mission q:…` (the receipt form, M3, N24b-19); or `--run <id> --kind
  control-program` (M3, BD-251: every stored program-instance record of the run with its certificate; a run with
  none is the appended 802, naming the run); or `--card <T-nn> --capsule <path>` (the card form, M5, BD-381: reads
  the theorem card's registry row and registered statement, recomputes the capsule file's BLAKE3 and writes one
  `.frobcert` entry of kind `lean-capsule`); or an energy budget (positional, the `EnergyBudgetId` that
  `shield.budget` prints, told from an answer address by its identifier type; the budget form, M4, N22b-14:
  `frob-lemma`'s writer of the proof kind `energy-budget` writes one entry of that kind with the budget's payload
  and `run` null), optionally with `--run <id>` naming a run that ran under that budget (`sim.run
  --shield-budget`), whose entry then carries the `run` member that the auditor's run-replay step recomputes
  (DESIGN, the run-replay form: one claim source, the budget, and the run it is replayed on; a run that did not
  run under the budget is 802 with `pointer: /run`); the remaining claim kinds of the full form at M5. Two claim
  sources in one call are 802; `--run` with neither `--kind control-program` nor a budget is 802. `--out <path>`
  (a `.frobcert` file or a directory for a set of bundles); idempotency key; budget.
- Outputs: `.frobcert` bundle(s) (`frob.cert/1`) under one Merkle root, the entry count and the in-tree check
  verdict; with `--receipt`, the campaign section (no per-cluster datum before close, the reveal after it; a
  re-bundle after close has a new hash); the three authorization flags, all false; `not_claimed`, the array of
  the six fixed strings of `lemma::not_claimed()`, once per call (N12-08). Evidence: *certificate*.
- Chapter refusals: `FROB-REF-826` (the quantifier prefix, domain or assumption set of a certificate does not match
  the query it is offered for; in the card form, a statement that differs from the card's registered one);
  `FROB-REF-833` (a no-authorization flag is missing or true); `FROB-REF-834` (a result is presented above the
  class of its weakest input, or as `Proved` or `Certified` without a certificate hash and a checker identity; in
  the card form, a card whose Lean route has no sealed capsule, or whose capsule hash differs from the file
  named); `FROB-REF-845` (a result bundled as qualified without a promoting receipt; raised by the receipt form,
  N24b-19). DESIGN, when 845 enters the list: the lint RL-06 (N32a-02) refuses a listed code that
  `registries/refusals.toml` does not hold, this bead closes at M0, and 845's row is entered by N24-REF at M1; so
  the M0 entry lists the other nine codes, and 845 enters the TOML list in the change that lands the receipt
  form (N24b-19, M3), as a worded refusal enters a row with its number. The bundle is written outside the store at `--out`, and every writer of bytes outside
  the store holds a grant of the export door (N37-18), so the row also lists the door's codes, as N37-REF's list
  test requires of an exporting verb with a path argument: `FROB-REF-816` (the artifact's no-authorization flags
  missing or not `false` in its manifest), `FROB-REF-731` (a private input without the private scope and flag),
  `FROB-REF-733` (a licence that forbids the export) and `FROB-REF-734` (`--out` outside the session's roots).
  The stored results a bundle cites as evidence (a numeric input, a cited result) are offered to the evidence
  boundary in its use `Certificate` (N37-26's `consumes_evidence` declaration on those arguments; the
  certificate path of N37-28's five), so the row also lists the boundary's two codes, as N37-REF's list test requires of
  every verb with that declaration: `FROB-REF-825` (a cited result of determinism class D4 or unrecorded,
  first in the boundary's precedence; §37.11, INV-SAFE-07) and `FROB-REF-819` (a cited result below the
  Certificate requirement in another member, where no more specific code applies). Both rows are N37-REF's
  from M0, so they are in the M0 entry.
- Implemented by: BD-133.
- Extended by: N24b-19 (M3: `--design`, `--mission` and `--receipt`, the campaign section of `.frobcert`, and
  845), BD-251 (M3: `--run --kind control-program`), N22b-14 (M4: the budget form, with or without `--run`),
  BD-381 (M5: `--card --capsule`, the `lean-capsule` entry), N37-26 and N37-28 (M3: 819 and 825 through the
  evidence boundary's admission in the dispatcher).

**`prove.kernel`** (MCP tool `prove_kernel`; owner `frob-lemma`)
- Inputs: kernel name.
- Outputs: the instance certificate of the kernel and the statement it certifies (it says nothing about
  floating-point results). A kernel with no certificate returns class absent and exit 2 with the finding
  `no-certificate`, never a default class. From M1 the output also carries `not_claimed`, the array of the six
  fixed strings of `lemma::not_claimed()` (N12-08). Evidence: *certificate*.
- Chapter refusals: none of its own.
- Implemented by: BD-027. Extended by: N12-08 (M1, `not_claimed`).

**`prove.list`** (MCP tool `prove_list`; owner `frob-lemma`)
- Inputs: optional bundle (positional; one row per entry, BD-133); filters `--class <c>`, `--checker <name>`
  (BD-027); `--withdrawn` (withdrawn bundles and answers with their report identifiers and successors, N25b-03,
  M1); `--effects` (the decision effects of certificates per milestone window and kind, N12-12, M1);
  `--coverage` (one row per kernel: certificate coverage, BD-379, M5).
- Outputs: certificates and theorem cards with their classes and checkers (each card row with id, statement,
  tier and, per route, `kind`, `route`, `class` for a `checked` route with receipt hash and checker identity,
  `status` and `receipts`; T-35's row with one `sub_entries` row per reflector), `certificates[]`, and
  `prooftools_revision`; for a bundle, one row per entry. `--effects` returns `frob.prove.effects.out/1`
  (`consumed`, `changed`, `unchanged`, `unknown`, the changed decisions with their `avoided` quantities;
  class Tested, supporting no Proof claim) and `--coverage` returns `frob.prove.coverage.out/1`; the row's
  output schema `frob.prove.list.out/1` is the union of the three, selected by the option (DESIGN). From M1 a
  listing also carries `not_claimed`, the array of the six fixed strings of `lemma::not_claimed()`, once per
  listing at the top of the result and never inside a row (N12-08). Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-027 (the listing over N12-06's card rows), BD-133 (the per-bundle form), N12-06.
  Extended by: N12-08 (M1, `not_claimed`), N12-12 (`--effects`), N25b-03 (`--withdrawn`), BD-379 (`--coverage`).

**`prove.reflect`** (MCP tool `prove_reflect`; owner `frob-lemma`)
- Inputs: `--certificate <bundle hash>` (an instance certificate the auditor accepts, of kind `lp-farkas`,
  `rational-ldlt`, `poly-identity` or `rewrite-trace`); `--reflector` with the kebab-case values `farkas`,
  `ldlt-pos` (BD-382) and `poly-eq`, `replay` (N12-28), which map to the Lean definitions `farkas`, `ldltPos`,
  `polyEq` and `replay`, and to the certificate kinds `lp-farkas` → `farkas`, `rational-ldlt` → `ldlt-pos`,
  `poly-identity` → `poly-eq`, `rewrite-trace` → `replay`; idempotency key (`--key`, the generated property
  `idempotency_key`); budget.
- Outputs: `frob.prove.reflect.out/1`: the soundness capsule hash, the instance capsule hash and the evaluation
  time, with evidence of class `Proved` whose receipts are both capsule hashes and both checker identities (stock
  Lean and `fln`). A job runs with a wall limit of twice the budget table's time; a timeout returns `unknown` for
  the promotion and never a failure of the certificate. Evidence: *certificate*.
- Chapter refusals: `FROB-REF-823` (unknown proof kind, or a kind with no enabled reflector, or a payload the
  named reflector's input type cannot express). A certificate the auditor does not `accept` is refused before
  any instance is generated with the auditor's own code for its first failing rule, the code `prove.verify`
  prints for it (BD-382, N12-28): `FROB-REF-826` (prefix, domain or assumption set does not match the query),
  `FROB-REF-829` (a side condition missing, or a field with the wrong frame, origin, sign or unit),
  `FROB-REF-833` (a no-authorization flag missing or true) or `FROB-REF-834` (a class above the weakest input, or
  `Proved`/`Certified` without a certificate hash and a checker identity); 823 is also the auditor's code for an
  unknown kind. The row lists all five, so the verb never returns a code its row lacks.
- Appended codes this verb raises (by the rule of its kind, not chapter codes): `FROB-REF-701` (an instance
  outside the reflection budget table for the pinned checker, refused before any Lean process starts; the
  certificate stays `Certified` with the reason `over reflection budget` and the cell it missed; exit 4);
  `FROB-REF-724` (the verb, or one reflector, not in this build: before M5, after KILL-REFLECT, or a reflector
  whose sub-entry is not `checked`; exit 5).
- Implemented by: BD-382 (the pilot reflectors `farkas`, `ldlt-pos`, and the verb), N12-28 (`poly-eq`, `replay`
  and budgeted admission).

**`prove.verify`** (MCP tool `prove_verify`; owner `frob-cert`)
- Inputs: one or more bundle or certificate paths (positional), or `--all` (every certificate in tree); a capsule
  file (`.flncap`) may be named positionally after its `lean-capsule` bundle, and the auditor then recomputes its
  BLAKE3 against the bundle's `capsule_blake3` (BD-381; without the capsule file the verdict for that bundle is
  `unsupported`, never `accept`); optional `--query q:…` (the query the bundle is offered for; a prefix, domain
  or assumption-set mismatch is 826, N25a-22).
- Outputs: one row per bundle: digest, kind, verdict (`accept`, `reject` with the first failing rule, or
  `unsupported`), and the refusal code where one applies; the offline binary `frob-verify` repeats the check with
  the standard library alone and returns identical verdicts, first failing rules and codes. Evidence:
  *certificate*.
- Chapter refusals: `FROB-REF-823` (unknown proof kind); `FROB-REF-826` (the quantifier prefix, domain or assumption
  set of a certificate does not match the query it is offered for); `FROB-REF-829` (a side condition of a proof rule
  is missing, or a field has the wrong frame, origin, sign or unit); `FROB-REF-833` (a no-authorization flag is
  missing or true); `FROB-REF-834` (a result is presented above the class of its weakest input, or as `Proved` or
  `Certified` without a certificate hash and a checker identity).
- Correction (round 1): the row formerly also listed `FROB-REF-848` to `-851`. By the auditor's mapping table
  (BD-133, BD-132; restated by BD-027) `prove.verify` and `frob-verify` report a rejected bundle under the Lemma
  codes 823, 826, 829, 833 and 834 only; the mission codes 848 to 854 and 881 appear only on a mission verb that
  runs the same checker before its answer returns (they are on the `mission.*` rows).
- Implemented by: BD-027 (in-tree check), BD-133 (mission rows of the check). Extended by: BD-381 (M5: the
  `lean-capsule` kind and the positional capsule file; an axiom list wider than the pinned whitelist is 829).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `prove.bundle` | `cert:write` | `frob.prove.bundle.out/1` | `qualify`, `make`, `proofs` | `frob prove bundle a:93f0 --out stance-bundle/ --key cb-1 --json` |
| `prove.kernel` | `prove:read` | `frob.prove.kernel.out/1` | `proofs` | `frob prove kernel K-cay-step --json` |
| `prove.list` | `prove:read` | `frob.prove.list.out/1` | `qualify`, `proofs` | `frob prove list quad-tray.frobcert --json` |
| `prove.reflect` | `prove:run` | `frob.prove.reflect.out/1` | `proofs` | `frob prove reflect --certificate b3:77ac --reflector farkas --key rf-1 --json` |
| `prove.verify` | `prove:read` | `frob.prove.verify.out/1` | `qualify`, `proofs` | `frob prove verify quad-tray.frobcert --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "prove.bundle"
cluster = "prove"
kind = "run"
tier = "S"
capabilities = ["cert:write"]
input_schema = "frob.prove.bundle.in/1"
output_schema = "frob.prove.bundle.out/1"
refusals = ["FROB-REF-731", "FROB-REF-733", "FROB-REF-734", "FROB-REF-816", "FROB-REF-819", "FROB-REF-825", "FROB-REF-826", "FROB-REF-833", "FROB-REF-834"]   # "FROB-REF-845" appended by N24b-19's change (M3)
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M1"
task_clusters = ["qualify", "make", "proofs"]
examples = ["frob prove bundle a:93f0 --out stance-bundle/ --key cb-1 --json"]
owner = "frob-lemma"
purpose = "Assemble a `.frobcert` bundle for a claim (mission slice at M1, full at M5)"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
exports = true
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
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725,
   803, 804 (stale parent) and 805 for `mutate`. What fields and registry tables add is NJL-01's variation table,
   the one list of it: 806 and 807, with `cursor`, `limit` and `follow`, for a paginated read (variation 1);
   `estimate_only` on a non-`run` entry with `estimable = true` (2); `x-frob-key: "optional"` and no 805 on a
   `key_optional` entry (3); 808 for an entry with an `int53` property at any depth of its input schema, whatever
   its kind, so every `run` row through `sync_wait_ms` (4); `action` for an `actions` list (5); 790 for a
   non-empty `needs_groups` (6); 891 to 893 for artifact `params` (7, 8); 734 and 828 for a parameter that
   accepts a path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`) (9). Variations 10 to
   13 are read from NJL-01's table with the beads that land them.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M0), each with the `task_clusters` of the
   machine-fields table, which is NJL-05's target assignment. No `[[task_cluster]]` table exists at M0: NJL-05
   declares the tables at M1 and seats these rows in them (its staging rule, step 1), and from then
   `task-cluster-mismatch` holds for them. A verb whose handler is not in the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- `prove.verify` runs the in-tree check; the independent binary `frob-verify`, built from the Rust standard library
  alone, repeats it offline on a machine with neither the agent, a GPU nor the simulator. The two share a format and
  nothing else.
- `prove.bundle` first answers at M1 for the mission slice and in full at M5. Every bundle carries the three
  no-authorization flags, all false; a missing or true flag is refused (`FROB-REF-833`). The registry example is
  the M1 form (a mission answer), so the conformance script runs it from the first milestone; the receipt form
  (a design with `--receipt`, N24b-19) and the run form (`--run --kind control-program`, BD-251) are inputs of
  the same row and answer from M3, the budget form (an energy budget, alone or with `--run`, N22b-14) from M4,
  the card form (`--card --capsule`, BD-381) from M5, and the claim kinds of the full form that have not landed
  answer `FROB-REF-724`, naming the form, until M5. `prove.bundle` has `exports = true`.
- Scope of `prove.bundle` (DESIGN): the row cites `cert:write`, the plan's named scope for writing a certificate
  bundle, and no other. It authorizes both the work of this `run` row and the export of the bundle through the
  door: the plan's worked session runs `frob prove bundle … --out carrier-b.frobcert` holding `cert:write` and
  no `export:*` scope (N32b-16 grants it for exactly that call; BD-409's M5 table), and none of the three
  publish scopes (`export:report`, `export:packet`, `export:policy`) names a certificate bundle. BD-258's act
  lint and the scope step of N37-18's export door therefore take `cert:write` as this row's spending scope and
  its publish scope for the export kind `certificate`. Rejected: `prove:run` with an `export:*` scope, which
  would make the worked session fail with 720 and grant a report or packet scope to publish a certificate.
- One spelling of the idempotency key on every surface: `--key` on the CLI, `idempotency_key` on MCP (N12-38's
  script now writes `--key`).
- `prove.reflect` is Frontier (M5): it relabels an instance certificate from Certified to Proved by kernel
  evaluation and changes no design decision.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly five entries with `cluster = "prove"` (3 `read`, 2 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster prove` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 5, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_prove.sh` runs each verb of the cluster (five), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/prove/<verb>.help.json` byte for byte for each of
      the five, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The script's run leaves a receipt with the measured counts (registry, CLI and MCP each 5; every difference
      list 0) and the controls caught. This bead records no gate value: the per-milestone surface beads (BD-022 at
      M0, then BD-137 and BD-409) record GATE-TETHER-02's part for their milestone from such a receipt with `cargo
      run -p frob-conformance --example record_gate -- --gate GATE-TETHER-02 --receipt <hash> --part <M>`, in a
      session that did not write the rows (N38b-19).

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_prove` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 5 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `prove.verify` is `read` at M0).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`prove.bundle`: 826, 833, 834, 816, 731, 733, 734,
  819, 825, with `exports = true`, and 845 in addition once N24b-19's change has added it; a planted removal of
  825 or 819 fails, as N37-REF's `evidence_consumers_list_819_825` requires of an evidence consumer;
  `prove.kernel`: none; `prove.list`: none; `prove.reflect`: 823, 826, 829, 833, 834; `prove.verify`: 823, 826, 829,
  833, 834); the
  appended codes equal the rule for its kind and fields; a planted 848 on `prove.verify` fails (mission codes
  belong to mission verbs); a planted 845 on `prove.bundle` in a registry without N24-REF's row fails RL-06.
- `reflector_values`: `prove.reflect`'s `reflector` enum is exactly `farkas`, `ldlt-pos`, `poly-eq`, `replay`; a
  camel-case `ldltPos` is 802.
- `inputs_by_row`: `prove.list` accepts `--class`, `--checker`, `--withdrawn`, `--effects`, `--coverage`;
  `prove.verify` accepts positional bundles, a positional capsule file after its bundle, `--all` and `--query`;
  `prove.bundle` accepts each claim source alone (a positional answer; `--design` with `--receipt`; `--run` with
  `--kind control-program`; a positional energy budget, alone or with `--run`; `--card` with `--capsule`) and
  refuses two together (an answer and a budget among them), `--kind` without `--run`, `--run` with neither
  `--kind` nor a budget and `--card` without `--capsule` (802).
- `scopes_cited`: `prove.bundle` cites exactly `["cert:write"]` with `exports = true`; `prove.reflect` cites
  `prove:run`; the three reads cite `prove:read`.
- `mcp_names_one_to_one`: the tool names are the mapped names (`prove_bundle`, `prove_kernel`, `prove_list`,
  `prove_reflect`, `prove_verify`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of conventions item 2, `idempotency_key` in no
  `required` array and annotated `x-frob-key: "required"`, `on_cancel` admitting exactly `leave` and
  `cancel_job`, and refuses `sync_wait_ms = 30001` with code 802; a
  verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_prove.sh` (header: `bead=VERBS-prove milestone=M0 labels=tether,verbs,verb:prove.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance prove`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the five verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
prove` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster prove`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit 8,
then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_prove/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact command,
exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values and the tag
`--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 5 and the
difference counts against 0; refusals are logged with code and first repair; a failure logs the first differing verb
and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 5 verbs (3 `read`, 2 `run`); the tables above. KA-755 (the exit-code table on
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

- Kill gate KILL-REFLECT: fallback, instance certificates stay Certified and `prove.reflect` answers the
  not-implemented refusal.
- Closing rule: the bead closes at M0, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M0, M1, M5. The
  per-milestone surface beads (BD-022, BD-137, BD-409) re-run this script as verbs land; that is how GATE-TETHER-02
  is re-measured.
- Scope names follow the naming rule decided in BD-258: a row cites scopes of its own cluster by kind (`read` →
  `<cluster>:read`; `run` → `<cluster>:run` or a named spending scope; `mutate` → `<cluster>:write` or a named write
  scope; `admin` → `<cluster>:admin`, `job:cancel` or a named admin scope), plus `export:*` when the row exports and
  `private:*` where privacy applies, never a scope named after the object read or the engine spent on; N32b-01's
  catalogue generates the forms; `prove.bundle`'s `cert:write` is the named scope of the catalogue that this cluster's
  Notes give as its spending and publish scope. `<cluster>:run` and `<cluster>:write` imply `<cluster>:read`, closed
  when a set is stored (at minting and at derivation, `Scopes::close_implied`), so checks stay plain set inclusion.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything else
  raises it).

## References

Appendix J (J.1 entry form; J.2 inventory rows; J.5 task clusters), §12.3, §12.4, §32.4, §32.6, §32.12.3, §32.18,
§32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
