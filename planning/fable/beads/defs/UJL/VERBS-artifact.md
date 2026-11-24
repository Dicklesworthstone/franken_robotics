---
key: "VERBS-artifact"
title: "frob-verbs registry: cluster `artifact`, four rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "chronicle", "frob-verbs", "solid", "registry", "verb", "test"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§32.6", "§29.3", "§21.17", "§19.16", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import`, which this bead registers (DESIGN
below); the count of record is NJL-03's.

This bead is the registry of record for the cluster `artifact`. The cluster covers content-addressed artifacts:
listing, reading by digest and byte range, export under the session's export root (single files, or from M2 one
cross-machine transfer bundle), and admission of such a bundle on the receiving machine. It has four verbs (2
`read`, 2 `run`); the handlers live in `frob-chronicle`. The bead carries every row completely, with the three
checks that keep the rows true: the generated CLI commands and MCP tools match the list; every verb has an
end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

DESIGN, the fourth row `artifact.import` (decided here because the plan gives the transfer rule and no command
that admits a bundle): the transfer rule between the
two machines (§21.17) must hold when they run standalone, which is the default, so a person or an agent moves the
allowed classes by hand: `artifact.export --to-host` writes one `frob.xfer/1` bundle (N21c-08), and without a verb
that admits it on the other machine the bundle can only be read in process or by a test entry, which no user can
call. N21c-08 specifies the verb (kind `run`, owner `frob-chronicle`, M2). Registering it is better for users than
the alternatives: an import mode on `artifact.export` would make one verb both write and admit, with two refusal
sets; a `store` verb would separate admission from the transfer class rules that `XferReader` enforces. The cost
is a departure from the count Appendix J states ("the registry therefore holds 243 verbs", the sum over its own
tables, not a rule that forbids a row): 244 verbs, 84 `run`, 42 first registered for M2. The row is recorded as
an addition: this bead writes its `[[addition]]` record in the change that enters the row (below), NJL-03's
count rows keep the specification's 243 as the baseline and give the complete registry as 244, NJL-05's
`operate` cluster seats `artifact_import`, and the verb-count sentence of every `VERBS-<cluster>` bead reads
244; the count tests read the registry, so no test is edited by hand.

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

In: the four registry entries of cluster `artifact`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `artifact.read` | read | M1 | S | Read an artifact by digest and byte range | §32.6, §29 |
| `artifact.list` | read | M1 | S | List artifacts | §32.6, §29 |
| `artifact.export` | run | M1 | S | Export an artifact under the session's export root | §32.6, §29 |
| `artifact.import` | run | M2 | S | Admit a cross-machine transfer bundle into the local store, every digest and class verified | §21.17, §29 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.

**`artifact.read`** (MCP tool `artifact_read`; owner `frob-chronicle`)
- Inputs: `digest` (`b3:` and 64 hex digits of BLAKE3); `offset` (default 0) and `length` (default to the end, at
  most 4 MiB), written on the CLI as `--range <start>-<end>` (inclusive offsets; `<start>-` reads to the end). It
  takes no path.
- Outputs: `{digest, kind, total_bytes, offset, length, range_blake3, bytes_b64, next_offset}`; `range_blake3` is the
  hash of the returned bytes, and the concatenation of all ranges hashes to `digest`; a request above 4 MiB is cut
  to the limit and says so in `next_offset`. Integrity is verified once per process per object: a stored object
  whose bytes do not hash to its name is an integrity fault (exit 9), quarantined and never served. An unknown
  digest is `FROB-REF-802` with the pointer `digest`. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-731` (a private object, or anything derived from one, without the private scope and
  flag; exit 5); `FROB-REF-816` (the object is an artifact of this system whose carried authorization flags are
  missing or not `false`: the reader-side check `frob_core::auth::check_carried` of N37-23, run before any other
  field of the artifact is used, on a bundle's manifest and, from M2, on a checkpoint container, a still's
  sidecar and a replay page's header, N37p1-01; returned with `missing:<field>`, `true:<field>`,
  `not-false:<field>` or `extra:<field>`; exit 3); `FROB-REF-840` (the object belongs to a sealed confirmation
  run and the session lacks `qual:admin` before close; exit 5, the exit of the code's row in N24-REF, the sealed
  data being outside the session's scopes); `FROB-REF-869` (the object is tombstoned: its evidence closure is
  dangling or tombstoned; returned with the tombstone's class, time and reason; exit 3); from M5 `FROB-REF-883`
  (the object is a chunk of a sealed validation or holdout part of a calibration dataset: the sealed store's
  only read path takes a score token, which `ground.score` alone mints, so this read is refused and the attempt
  logged; the read guard of BD-387; exit 3).
- Implemented by: BD-097; extended by N37-23 and N37p1-01 (the flags check and 816) and BD-387 (the sealed
  store's read guard and 883, M5).

**`artifact.list`** (MCP tool `artifact_list`; owner `frob-chronicle`)
- Inputs: `--run <run>` or `--kind <kind>`, where a kind is any row of the table `ObjectKind` of
  `frob-chronicle` (BD-097: the input's enumeration is generated from the table, which holds seven rows at M1,
  `column`, `snapshot`, `certificate`, `receipt`, `report`, `still` and `bundle`; a bead that stores objects of a
  new kind adds its row in the same change, an additive change of the schema: the transfer bundle of N21c-08,
  the scene packs of N19b-02, policy weights and scene recipes as their owners store them; a kind outside the
  table is `FROB-REF-802` with the pointer `kind` and the table's kinds listed); `limit`; `cursor`; from M3
  `--private` (BD-305: the explicit private listing, the only way a session sees its private objects; the row
  then carries `option_capabilities = { private = ["private:read"] }`, so the option without that scope is
  `FROB-REF-720` while a call without it needs only `artifact:read`). Paginated against the committed frontier.
- Outputs: rows of digest, kind, bytes, the ledger row that references the object (for a column file its chunk
  key) and whether it is pinned, ordered by (kind, digest), with the `frontier`; private objects are absent
  from the plain listing whatever the session's scopes, and no count of withheld rows is printed; the envelope
  of a private listing is itself marked private. Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-097; extended by BD-305 (`--private` and the visibility filter every listing calls, M3).

**`artifact.export`** (MCP tool `artifact_export`; owner `frob-chronicle`)
- Inputs: `digest`; `out`, a path under the session's export root (a path ending in `/` is a directory and the
  file is named `<hex>.<kind extension>`); `idempotency_key`. A private object also needs the scope
  `private:export`, the per-call flag `--allow-private-export` and the owner-held key `private.export_allowed`
  set to true (BD-305, M3); without one of them the call is `FROB-REF-731` with `observed` `missing-scope`,
  `missing-flag` or `export-disabled-by-owner`. From M2 (N21c-08), the cross-machine form: `--to-host <mac|trj>`
  with one or more digests (`<digest> [<digest> …]`) and an optional `--campaign <id>` writes one `frob.xfer/1`
  bundle at `out` in place of single files (an `out` ending in `/` names a directory, and the bundle is written
  there as `<hex>.frobxfer`, `<hex>` being the bundle's digest); `--to-host` names the destination recorded in
  the manifest and metered against and opens no connection. From M2 (N21c-09), `--replay-closure
  <run>:<address>` names the replay closure of a failed world (its world address, design, policy weights and
  scene recipe) in place of a digest list; it is valid only with `--to-host`, and given without it, or together
  with digests, it is a schema failure (`FROB-REF-802`).
- Outputs: the relative path, bytes, the digest (the copy is hashed while written and compared before the rename
  into place), and the `authorization` block with the three no-authorization flags, all `false`; a repeated key
  with the same resolved request returns the stored receipt and writes nothing. With `--to-host`, the digest is
  the bundle's, the result lists each item (`id`, `kind`, `class`, `bytes`, `blake3`), and the envelope carries the
  `transfer` block (class, items, bytes, the campaign's running total, bytes per campaign-hour, `over_target`;
  exact integers, `Tested`). Exceeding 1 GB per campaign-hour (TARGET) sets `over_target: true` and is never a
  refusal. Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-731` (a private object, or anything derived from one, without the private scope and
  flag); `FROB-REF-733` (the licence set of the artifact forbids this export: `research-only`, `proprietary` or
  `unknown`); `FROB-REF-734` (a path outside the session's roots: `..`, an absolute path, or a link that leaves the
  root); `FROB-REF-816` (an artifact whose authorization flags are missing or not `false`, naming
  `missing:<field>`, `true:<field>`, `not-false:<field>` or `extra:<field>`); from M2 `FROB-REF-618` (a transfer
  class that may not cross machines: an item whose kind maps to a compiled scene field, world state or a rollout
  buffer, or has no row in `registries/transfer_classes.toml`; the class is derived from the recorded kind and
  never declared by the caller; the whole call is refused and no file is written; exit 6, `pointer` the item id;
  N21c-08); from M3 `FROB-REF-310` (the artifact is a scene pack, a scene variant, or a kit or starter pack
  containing scene packs, and the pack holds a `research-only` or private asset: raised through this verb's
  handler by `ScenePack::export_check`, the code's only raiser, and returned unchanged by the export door; exit
  3; N19b-02, N37-18). Every other derived artifact is refused at this door with 731 or 733, never 310.
- Implemented by: BD-097; extended by N21c-08 (`--to-host`, `--campaign`, the bundle and 618, M2), N21c-09
  (`--replay-closure`, M2), BD-305 (`--allow-private-export` and the three conditions of a private export, M3)
  and N19b-02 (the scene-pack check and 310, M3), with N37-18 owning the export door.

**`artifact.import`** (MCP tool `artifact_import`; owner `frob-chronicle`; first milestone M2)
- Inputs: `bundle`, the path of one `frob.xfer/1` bundle under the session's roots (a file parameter, so the path
  and payload refusals of the field rule apply); `idempotency_key`. It takes no digest and no class: classes are
  re-derived from the kinds in the manifest.
- Outputs: the identities admitted (per item `id`, `kind`, `class`, `blake3`) and the `transfer` block of the
  receiving side (the same fields as `artifact.export --to-host`); one ledger row `xfer` per admission (direction
  `in`). A repeated key with the same resolved request returns the stored receipt and admits nothing again. A scene
  is never admitted as a field: its recipe is admitted and the scene is regenerated by `scene.gen --recipe`
  (N21c-09). Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-618` (the bundle holds an item whose class may not cross machines, or a kind with no
  row, even if the sender let it through; nothing is admitted; exit 6); `FROB-REF-816` (the bundle's manifest
  lacks the three no-authorization flags, or holds one that is not the literal `false`: a bundle is an exported
  artifact of this system and this verb is its reader, so the reader-side check of N37-23 runs before any other
  field of the manifest is used; nothing is admitted; exit 3); and the integrity refusal
  `xfer-item-digest-mismatch` (N21c-08: an item whose bytes do not hash to its manifest digest; family `9xx`,
  exit 9, `retry: never`, `pointer` the item id, `observed` and `required` the two digests; nothing is
  admitted). That refusal is filed through N21-REF under its slug and numbered by the allocation ledger
  (N35-33); the row lists the ledger's number for the slug, and this bead writes none. The appended codes 734
  and 828 come from the field rule for `bundle`.
- Implemented by: N21c-08 (`XferReader`, the admission and the meter); BD-097 for the store it admits into.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `artifact.read` | `artifact:read` | `frob.artifact.read.out/1` | `operate` | `frob artifact read b3:61f0 --range 0-4095 --json` |
| `artifact.list` | `artifact:read` | `frob.artifact.list.out/1` | `operate` | `frob artifact list --kind certificate --json` |
| `artifact.export` | `artifact:run`, `export:report` | `frob.artifact.export.out/1` | `operate` | `frob artifact export b3:61f0 --out exports/ --key ax-1 --json` |
| `artifact.import` | `artifact:run` | `frob.artifact.import.out/1` | `operate` | `frob artifact import exports/xfer-ax-2 --key ai-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column; `paginated = true` where the block says paginated.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "artifact.export"
cluster = "artifact"
kind = "run"
tier = "S"
capabilities = ["artifact:run", "export:report"]
input_schema = "frob.artifact.export.in/1"
output_schema = "frob.artifact.export.out/1"
refusals = ["FROB-REF-310", "FROB-REF-618", "FROB-REF-731", "FROB-REF-733", "FROB-REF-734", "FROB-REF-816"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M1"
task_clusters = ["operate"]
examples = ["frob artifact export b3:61f0 --out exports/ --key ax-1 --json"]
owner = "frob-chronicle"
purpose = "Export an artifact under the session's export root"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
exports = true
```

The addition record of the fourth row, written in the same change as the row (NJL-03's rule for a row beyond the
specification's 243; `[[addition]]` is a top-level table that N32a-02's strict reader admits from M0). Without it
NJL-03's staged check of the registry against the baseline list fails with `not-in-baseline`:

```toml
[[addition]]
name = "artifact.import"
added_by = "VERBS-artifact"
defined_in = "§21.17"
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
   admission), 761 (above the compute envelope in force), 803 (key reused with different content) and 805
   (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. The other properties and codes are
   added by field, as the variation table of NJL-01 lists them (that table is the one list; its variations 10
   to 16, which later beads land, are read there and not copied here): 806 and 807 for paginated reads; 808 (an
   integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property and for no
   entry by kind (every `run` entry has one in `sync_wait_ms`, every paginated entry in `limit`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790
   for a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a
   path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`).
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose. A code whose registry row states another exit keeps its row's exit:
   `FROB-REF-840` on `artifact.read` is exit 5 (N24-REF).
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- A resource read over MCP returns at most 4 MiB (TARGET); larger artifacts are read with `artifact.read` by byte
  range or exported.
- A path outside the session's roots is refused (`FROB-REF-734`: a chapter code of `artifact.export`, whose `out`
  is a write path, and appended by the field rule to `artifact.import`, whose `bundle` is a file parameter;
  `artifact.read` and `artifact.list` take no path); a private object without the private scope is refused
  (`FROB-REF-731`); an export the licence set forbids is refused (`FROB-REF-733`).
- `artifact.list` is paginated (`paginated = true`: `cursor`, `limit`, `follow`, codes 806 and 807).
  `artifact.export` sets `exports = true` (its envelope carries the three no-authorization flags, also on the
  bundle form, and the bundle's manifest carries them as every exported artifact does, which is what
  `artifact.import` checks before it admits anything).
- Exit codes of the chapter refusals (from the owning registries): 5 for 731, 733, 734 and 840; 3 for 310, 816,
  869 and 883; 6 for 618; 9 for `xfer-item-digest-mismatch`.
- The transfer rule is one path in each direction: `artifact.export --to-host` writes through `XferWriter` and
  `artifact.import` admits through `XferReader`, the same writer and reader the peer link calls from M4
  (N21c-08); there is no second path, and no other verb admits a bundle.
- Options and codes added after M1 (`--to-host`, `--campaign`, `--replay-closure` and 618 at M2; `--private`,
  `--allow-private-export` and 310 at M3; 883 at M5) are listed in the blocks with their milestone. Each option enters the generated input schema when its extending
  bead lands (an additive change that keeps the major version); before that a call that passes one is an unknown
  parameter, `FROB-REF-802`. A listed code that is not yet raised is legal in a `refusals` list.
- DESIGN, the task cluster of `artifact.import`: `operate` (the surface chapter's "job, artifact, device and
  session verbs"), so that an agent with one cluster enabled can write a bundle on one machine and admit it on
  the other. NJL-05, which assigns `operate`, seats it there: the cluster's seven tools are `job_list`,
  `job_status`, `job_find`, `artifact_list`, `artifact_read`, `artifact_export` and `artifact_import`, and
  `spark_devices` sits in `devices` and `fleet`. Rejected: `fleet` (also full, and its verbs land at M4, two
  milestones after this one).
- If the owner prefers the specification's count of 243, this row is removed and admission stays in process
  (the peer link from M4) and in N21c-08's integration entry; N21c-08's and N21c-09's scripts already run
  either way, and the count tests read the registry (NJL-03), so the reversal is the row, its `[[addition]]`
  record and its task-cluster seat.
- The examples follow BD-097's input forms: `--kind` takes a row of its kind table (`certificate`, not the file
  suffix `frobcert`, which an earlier draft had and which the schema refuses with 802), and `b3:61f0` is the
  abbreviation of the certificate bundle root of the first mission answer of Appendix L (L.9), replaced by its full digest when the
  cluster script runs the example (NJL-02's example identities). The `artifact.import` example reads the bundle
  `exports/xfer-ax-2`, which the cluster script writes just before it (from M2) with `frob artifact export b3:61f0
  --to-host mac --out exports/xfer-ax-2 --key ax-2 --json`; admitting objects already in the store re-verifies
  them and changes nothing. The certificate bundle may cross: N21c-08's table `registries/transfer_classes.toml`
  maps the kind `certificate` to the class `Receipt` (a certificate is a checkable record of a result, as a
  receipt is), so the example's export and import are admitted.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly four entries with `cluster = "artifact"` (2 `read`, 2 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster artifact` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 4, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_artifact.sh` runs each verb of the cluster (four),
      and from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`; for `artifact.import`, N21c-08's `scripts/e2e/fleet_transfer.sh`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/artifact/<verb>.help.json` byte for byte for each of
      the four, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused; the snapshot of `artifact.export` is re-pinned, as an additive change, when N21c-08,
      N21c-09 and N19b-02 land.
- [ ] `registries/verbs.toml` holds the `[[addition]]` record of `artifact.import` (`added_by = "VERBS-artifact"`,
      `defined_in = "§21.17"`), entered in the change that enters the row, and no other record names this bead.
- [ ] From M2, an export of a forbidden class with `--to-host` answers `FROB-REF-618`, exit 6, and writes no file;
      `--replay-closure` without `--to-host` answers `FROB-REF-802`, exit 3; the import example admits the bundle
      the script wrote and reports the `transfer` block.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_artifact` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 4 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `artifact.export` is `run` at M1, `artifact.import` is `run` at M2).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`artifact.read`: 731, 816, 840, 869, 883, and not
  734, since it takes no path; `artifact.list`: none; `artifact.export`: 310, 618, 731, 733, 734, 816;
  `artifact.import`: 618, 816 and the code the allocation ledger holds for `xfer-item-digest-mismatch`, read
  from the ledger by the test and written nowhere in it, with 734 and 828 appended by the field rule for
  `bundle`); the appended codes equal the rule for its kind and fields, with 806 and 807 on `artifact.list`
  (paginated) and on no other row.
- `addition_recorded`: the registry file parses with exactly one `[[addition]]` record whose `added_by` is
  `VERBS-artifact`; its `name` is `artifact.import`, a row of the registry, and its `defined_in` is `§21.17`; a
  `synthetic` registry with the row and without the record is the planted case NJL-03's staged check refuses
  (`not-in-baseline`), which that bead's `additions_recorded` asserts.
- `entry_flags`: `artifact.list` has `paginated = true`; `artifact.export` has `exports = true` and
  `artifact.import` does not; the generated `artifact.read` input schema has `digest`, `offset` and `length` and
  no path property; the generated `artifact.import` input schema has `bundle` as a file parameter and no digest
  or class property.
- `late_options`: once N21c-08 and N21c-09 land, the generated `artifact.export` input schema has `to_host`,
  `campaign` and `replay_closure` and the change is classified additive; before that the snapshot has none of
  them (a call that passes one is `FROB-REF-802`). Once BD-305 lands, `artifact.export` has
  `allow_private_export` and `artifact.list` has `private` together with `option_capabilities = { private =
  ["private:read"] }` (lint `RL-22` requires the key to be an input property, so the two enter in one change);
  before that neither row has the property or the field.
- `mcp_names_one_to_one`: the tool names are the mapped names (`artifact_read`, `artifact_list`, `artifact_export`,
  `artifact_import`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties (`sync_wait_ms`, `idempotency_key`, `budget`,
  `estimate_only`, `on_cancel`, `envelope_scope`), with `idempotency_key` outside the schema's `required` array
  and annotated `x-frob-key: "required"`, and refuses `sync_wait_ms = 30001` with code 802; a verb whose
  capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_artifact.sh` (header: `bead=VERBS-artifact milestone=M1
labels=tether,verbs,verb:artifact.read,verb:artifact.list,verb:artifact.export,verb:artifact.import gates=GATE-TETHER-02 seed=1
needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper and
`scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance artifact`. Steps: (1) `frob capabilities --json` and
`frob robot-docs --json` list the four verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
artifact` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster artifact`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; from M2,
before the `artifact.import` example, the bundle it reads is written with the `--to-host` form named in the notes
(4a), and one export of a planted rollout buffer with `--to-host` expects `e2e_expect_refusal FROB-REF-618 6`
with no file written; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_artifact/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 4
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

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

- An earlier draft listed `FROB-REF-734` on `artifact.read`, which takes no path, and omitted the tombstone and
  sealed-run refusals its implementing bead BD-097 raises (869, 840); the block now follows BD-097.
- The bundle form of `artifact.export` and the verb `artifact.import` are N21c-08's DESIGN (the plan gives the
  transfer rule and no command); this bead registers what N21c-08 and N21c-09 specify and adds the fourth row by
  the DESIGN in the background. The meter never refuses: `over_target` is a flag on the campaign's transfer
  summary and a failed clause of GATE-SPARK-13, not a code.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-137) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §19.16 (`FROB-REF-310`), §21.17, §21.21
(`FROB-REF-618`), §21.22 (INV-SPARK-10), §29.3, §32.4, §32.6, §32.12.3, §32.18, §32.21; GATE-TETHER-02;
INV-TETHER-01, -02; KA-755; N21c-08, N21c-09, N19b-02, N37-18.
