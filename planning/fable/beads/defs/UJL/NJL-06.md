---
key: "NJL-06"
title: "frob-mcp: the resource frob://docs/clusters, the task-cluster directory generated from the registry, its test"
type: "feature"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "frob-mcp", "frob-verbs", "solid", "verb", "test", "e2e"]
parent: "WP-06"
deps: ["BD-138", "N32b-17", "NJL-05", "STD-E2E"]
needs: []
related: ["NJL-08", "NJLx-51"]
gates: ["GATE-TETHER-12"]
plan_refs: ["Appendix J (J.5)", "§32.12.4", "§32.12.7", "§32.12.1", "§32.17", "§32.19 (INV-TETHER-13)", "§32.21 (GATE-TETHER-12)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) serves its 244 verbs to agents as MCP tools (MCP:
the Model Context Protocol) through the server `frob-mcp`, which is generated from the verb registry
`registries/verbs.toml`. An agent never sees all the tools. It sees a **task cluster**: at most seven tools
chosen for one activity, plus three base tools (`session_focus`, `job_wait`, `job_cancel`), and it changes
cluster with `session_focus {cluster}`. The registry declares 45 task clusters (bead NJL-05).

To choose a cluster the agent must know which clusters exist and what each is for, without loading 244 tool
definitions. That is the job of one MCP **resource**, `frob://docs/clusters`: the task-cluster directory. It is
one of the three documentation resources of the server (`frob://docs/guide` is the agent guide and
`frob://docs/verb/{name}` the long form of one verb's documentation). This bead builds the directory and the
test that keeps it equal to the registry and to the command-line guide.

FACT (the suite's precedent): the mail server of the same software suite maps its tools to named clusters and
serves a cluster directory as a resource (`resource://tooling/directory`), filtered when the server is built.
DESIGN decisions: (1) the directory is generated from the registry by the same generator that writes the
`clusters` section of `frob robot-docs`, so the guide, the resource and the tool tags cannot drift (rejected: a
hand-written directory file, which would be a second copy of the assignment); (2) it is a resource and not a
tool, because it is static per build, costs nothing to serve and should not occupy one of a cluster's seven
seats (rejected: a `clusters_list` tool in `orient`, which would push `session_show` or another tool out);
(3) it lists tool names and one activity line per cluster, not tool schemas: the schemas arrive when the
cluster is focused.

Terms. *Resource*: an MCP object addressed by a URI and read with `resources/read`; resource URIs of this
server use the scheme `frob://` and name a registry row or a content-addressed object, never a file-system
path. *Focus*: the session's current task cluster; it changes what is listed and never what is permitted.

## Scope

This bead is the only place where the task-cluster directory is specified: its one generator, its text, and
the handler that serves it from M1, two milestones before the rest of the server's resources.

In: the generator `cluster_directory()` in `frob-verbs`, which is the single renderer of the directory for
every door; the resource handler for `frob://docs/clusters` in `surface/frob-mcp`; the directory's text format
and size budget; its equality with the registry and with `frob robot-docs`; its listing and reading in process
and through `frobd --mcp`; the end-to-end script.

Out, with the bead that has it:
- the documentation model, the verb `robot-docs` and its other sections and layers (N32b-17, a dependency: it
  builds the model and the verb, with a `clusters` section of name, activity and tool names per cluster; this
  bead fixes the exact text of that section and routes it through `cluster_directory()`, so that from this bead
  on the section and the resource are one string; the verb's `--cluster <name>` layer, with one-line purposes
  per tool, is a different text and is not touched).
- the other two documentation resources' texts (`frob://docs/guide`, `frob://docs/verb/{name}`), the skill
  file, the prompt texts, and the drift check across the three doors: N32b-18, which depends on this bead. For
  `frob://docs/clusters` its resource renderer returns the string of `cluster_directory()` unchanged; it does
  not generate the directory a second time.
- the resource scheme as a whole (the URI grammar and parser, the other seven groups, templates, the size cap
  as a refusal, subscriptions, private-scene handling) and the registration of the other documentation
  resources at M3: N32a-25, which depends on this bead, keeps this handler, and brings it under the common
  parser without changing a byte of what it returns.
- the assignment itself and its lints (NJL-05); the focus mechanism (BD-138, BD-298); the prompts (N32a-26,
  BD-298); the agent guide as a whole (BD-313).

## Technical approach

**Generator (shared by both surfaces).** `frob_verbs::docs::cluster_directory(reg: &Registry) -> String`,
pure, with no clock and no environment input. It lives in the module of the documentation model (N32b-17) and
is the renderer of that model's cluster list: this bead makes the guide's renderer print the `clusters`
section by calling it, and the resource renderer (N32b-18) returns its string, so there is one directory text. `frob robot-docs --json` returns the same string as
`result.sections.clusters`, and plain `frob robot-docs` prints it as the section `clusters` after the guide
pages. The verb has no option of its own for the directory: `--cluster <name>` prints one cluster in depth,
and `--clusters` is not a `robot-docs` option (it is the server's launch flag, below).

**Format** (UTF-8 text, media type `text/markdown`; one block per cluster; the order is the registry's
declaration order, with the default cluster first):

```text
# Task clusters (45). Call session_focus {"cluster": "<name>"} to switch. Focus changes what is listed, not
# what is permitted. Always listed: session_focus, job_wait, job_cancel.

orient (default): Find out what exists
  capabilities, doctor, robot_docs, schema, design_templates, scene_list, session_show
author: Make and change a design
  design_new, design_show, design_edit, design_diff, design_check, design_import, control_synth
…
```

Rules of the text:
1. One header of at most three lines, giving the number of clusters, the switching call and the base tools.
2. Per cluster: the name, `(default)` on exactly one, the activity line (at most 80 characters) from the
   registry's `activity` field, then the tool names in registry order under the name map (`.` and `-` to `_`).
3. A tool whose verb is not implemented in this build is followed by `(absent)`; a Frontier or Moonshot tool by
   `[F]` or `[M]`. Both marks come from the same capability table that `frob capabilities` prints. A verb whose
   row is not registered yet does not appear: the 45 blocks are declared from M1 and fill as the cluster beads
   register their rows (the staging rule of NJL-05), so a block may be short before M4.
4. No text in the directory originates in an import (design names, asset metadata, scan annotations): every
   character comes from the registry (INV-TETHER-13: no tool description, prompt, server instruction or
   `next_actions.command` contains text that originated in an import).
5. Size budget (DESIGN TARGET): at most 16,000 bytes, so that reading the directory costs about as much as one
   focused tool list (whose own budget is 12,000 bytes). The server-wide cap on any resource read is 4 MiB.

**Resource handler.** A hand-written handler declared with the resource attribute macro of `fastmcp_rust` (the
suite's MCP library, on which the server is built) in `surface/frob-mcp/src/resources/docs.rs` (resources,
prompts and the MCP half of `session.focus` are the only hand-written handlers of the server; every tool is
generated). It returns one content item `{ uri: "frob://docs/clusters", mime_type: "text/markdown", text:
<directory> }`. It needs a session and no scope beyond it, because the directory holds no object of any
namespace. It is listed by `resources/list` on every transport the server offers (stdio; streamable HTTP on
loopback, also when the daemon hosts the server as `frobd --mcp`; the in-memory transport of the tests).

**Consistency the test enforces.** For every cluster c in the directory: the names under c equal the tools
tagged `task:c` in `tools/list`, equal the registry's `[[task_cluster]]` table for c, and equal the tools that
`session_focus {cluster: c}` reports as now visible. The directory text equals `result.sections.clusters` of
`frob robot-docs --json` byte for byte.

**Compatibility fallbacks.** When the server is launched with `--clusters a,b` (a static listed set), the
directory still lists all clusters and marks those not served as `(not served by this launch)`; an agent can
then see why a focus call is refused. That mark is the only text a launch can change, so the byte equality with
`robot-docs` above is asserted on a launch that serves every cluster, and on a static launch the directory
with the marks removed equals it (the CLI has no launch flag and prints no such mark).

## Acceptance criteria

- [ ] `resources/list` contains `frob://docs/clusters`, and `resources/read` returns one text item whose
      header states 45 clusters, in process (stdio and in-memory) and through `frobd --mcp` (loopback HTTP).
- [ ] The directory equals the registry: 45 blocks, the same names, activity lines and tool lists as the
      `[[task_cluster]]` tables; exactly one block is marked default and it is `orient`.
- [ ] The directory equals `result.sections.clusters` of `frob robot-docs --json` byte for byte.
- [ ] For every cluster, the tool names in its block equal the result of `session_focus` for that cluster and
      the `tools/list` filtered by the tag `task:<cluster>`.
- [ ] GATE-TETHER-12, the resource clause for this row (TARGET; M3; first measured at M1): the documentation
      resource kind is listed and readable, and it matches `robot-docs`.
- [ ] The text is at most 16,000 bytes; the measured size is logged against the threshold.
- [ ] A design whose name contains instruction-like text, created before the read, changes no byte of the
      directory (the poisoned-import control).
- [ ] The directory is byte-identical across two reads, two server starts, debug and release builds, Linux and
      macOS.
- [ ] `surface/frob-mcp/CONTRACT.md` lists the resource with its URI, form, source and size budget.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_directory` and `cargo test -p frob-mcp --test resource_clusters` (the
real registry and the real server over the in-memory transport; no mocks):
- `directory_lists_45`: 45 blocks; the header's count equals the block count.
- `directory_matches_registry`: each block's name, activity and tools equal the registry table.
- `default_first_and_unique`: the first block is `orient (default)`; no other block carries the mark.
- `base_tools_in_header_only`: `session_focus`, `job_wait`, `job_cancel` appear in the header and in no block.
- `absent_and_tier_marks`: with a `synthetic` registry in which `solve.run` is registered in `search` and a
  capability table in which it is absent and Frontier, the `search` block shows `solve_run [F] (absent)` (the
  real `solve.run` row is registered only at M4).
- `staged_blocks`: on the real M1 registry, a cluster whose rows are not all registered yet (for example
  `gaits`, whose rows land at M4) shows exactly its registered tools, and its block is still present.
- `size_budget`: the length is at most 16,000 bytes; the test prints it.
- `pure_generator`: two calls return equal strings; a change of environment variables, locale or time zone
  changes nothing.
- `equals_robot_docs_section`: the string equals the guide's `clusters` section.
- `resource_listed_and_read`: over the in-memory transport, list then read; the item's URI and media type are
  as specified.
- `static_launch_marks`: a server built with `--clusters orient,author` marks the other 43 blocks, and the text
  with the 43 marks removed equals the directory of a full launch byte for byte.
- `no_imported_text`: after importing a design named with an instruction-like string, the directory is
  unchanged.

### End-to-end script and logging

`scripts/e2e/mcp_cluster_directory.sh` (header: `bead=NJL-06 milestone=M1 labels=tether,mcp,verbs
gates=GATE-TETHER-12 seed=1 needs=cpu,mcp,frobd plants=stale-directory,extra-tool`). It runs the same steps
against `frob-mcp` over stdio and against `frobd --mcp` over loopback HTTP: (1) `resources/list`, assert the
URI is present; (2) `resources/read`, save the text, assert the header and count the blocks (45); (3) `frob
robot-docs --json`, assert equality of the saved text with the `clusters` section (`e2e_assert_digest_eq` on
SHA-256 of both); (4) for each cluster in the saved text, `session_focus` and `tools/list`, assert the three
sets (directory block, focus result, listed tools minus base) are equal; (5) `tools/list` with `includeTags`
for three clusters chosen by the seed, assert equality with the directory; (6) the poisoned-import control:
`frob design import` of a file whose name field is an instruction, then a second read, assert the digest is
unchanged; (7) plants: `stale-directory` serves a directory generated from an older registry and must fail
step (4); `extra-tool` adds a tool to one block and must fail the same step. The log
(`target/e2e-logs/mcp_cluster_directory/<utc>.ndjson`, written through the shared helper) has each MCP request
verbatim, its wall time and result or error code; the metrics `clusters` (== 45) and `directory_bytes` (<=
16000) tagged `--gate GATE-TETHER-12`; each assertion with observed and expected sets and, on failure, the
first cluster and tool that differ and the paths of the saved texts; and the `summary`.

### Known answers and negative controls

- Known answers (exact): 45 blocks; the tool lists of the task-cluster table of bead NJL-05; the header's three
  base tools.
- Negative controls: the two plants; the poisoned import; a generator patched to read the system clock (must
  fail `pure_generator`).
- Invariant carried: INV-TETHER-13, as `no_imported_text` and step (6).
- Metamorphic relation: reading the directory before and after any sequence of `session_focus` calls returns
  the same bytes (the directory does not depend on focus).
- Determinism lanes: byte equality twice on one machine, across server restarts, across debug and release and
  across Linux and macOS (class D0).

## Considerations

- OPEN-TETHER-2 (owner: Tether lead): whether agent harnesses act on catalog-changed notifications. The
  directory is useful in both cases: a client that never re-lists reads the directory once and uses
  `verb_invoke` or tag filters.
- If kill gate KILL-MCP fires and the server becomes its own workspace talking to the daemon over a socket, the
  handler is unchanged, because it links `frob-verbs` with the `registry-only` feature and needs no physics.
  Under the second fallback (CLI only) the directory is the `clusters` section of `frob robot-docs` and this
  bead's MCP half is recorded as not run.
- The handler written here is the first resource of the server. When the full resource scheme lands at M3
  (N32a-25) the handler moves under the common URI parser and size rule; the equality tests of this bead are
  re-run there and must pass unchanged. The documentation resources carry no evidence footer (their source
  verb's answer is operational and has no claims), which is what keeps them byte-equal to `robot-docs`.
- The resource is public within a session. It must never list an object of a namespace or a private scene;
  it lists registry names only, which is why it needs no private-scope check.
- The subscription mechanism of the MCP library (at most 64 streams server-wide) is not used: the directory
  cannot change while a server runs.

## References

Appendix J (J.5), §32.12.4, §32.12.7, §32.12.1, §32.17, §32.19, §32.21; GATE-TETHER-12; INV-TETHER-13;
OPEN-TETHER-2; KILL-MCP.
