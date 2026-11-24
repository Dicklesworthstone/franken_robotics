---
key: "NJL-05"
title: "frob-verbs registry: task clusters of at most seven tools: 45-cluster assignment, lints, the 12 kB list budget"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "frob-verbs", "solid", "registry", "verb", "test", "gate"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJLx-02", "STD-E2E"]
needs: []
related: ["NJL-09", "VERBS-actuator", "VERBS-algebra", "VERBS-analyze", "VERBS-artifact", "VERBS-autopsy", "VERBS-contact", "VERBS-control", "VERBS-cores", "VERBS-design", "VERBS-eval", "VERBS-evidence", "VERBS-experiment", "VERBS-explain", "VERBS-fidelity", "VERBS-fleet", "VERBS-foundry", "VERBS-gait", "VERBS-job", "VERBS-mission", "VERBS-module", "VERBS-operator", "VERBS-opt", "VERBS-phylum", "VERBS-policy", "VERBS-predict", "VERBS-prove", "VERBS-report", "VERBS-result", "VERBS-run", "VERBS-scene", "VERBS-session", "VERBS-shield", "VERBS-sim", "VERBS-skill", "VERBS-solve", "VERBS-spark", "VERBS-store", "VERBS-study", "VERBS-task", "VERBS-telemetry", "VERBS-train"]
gates: ["GATE-TETHER-12", "GATE-TETHER-02"]
plan_refs: ["Appendix J (J.5)", "§32.12.4", "§32.12.3", "§32.13", "§32.21 (GATE-TETHER-12)", "§32.22 (OPEN-TETHER-2)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) declares every capability as a verb in one registry,
`registries/verbs.toml`, from which the MCP server `frob-mcp` generates one tool per verb (MCP: the Model
Context Protocol, by which an agent harness lists and calls tools). The registry holds 244 verbs (the 243 of the
specification and `artifact.import`, registered by VERBS-artifact; the count of record is NJL-03's). A **registry
cluster** is a namespace and can be large: `scene` has thirteen verbs, `design` twelve, `analyze` twenty-six.
What an agent sees is a **task cluster**: at most seven tools chosen for one activity, declared in the registry
beside the verbs. A verb may sit in several task clusters, and a lint requires every verb to sit in at least
one. Three **base tools** are always listed and are not counted: `session_focus`, `job_wait`, `job_cancel`. An
agent switches with `session_focus {cluster}`; focus changes context size, never authority, which is the
session's capability token.

Why seven, and why focus (DESIGN). Agents choose badly among long tool lists, and every tool definition an
agent does not need costs context on every turn. TARGET: the definitions of the base tools and one task cluster
occupy at most 12 kB of the `tools/list` response. Rejected: listing all 244 tools (about a megabyte of
definitions, and no agent chooses well among them); one generic `invoke` tool as the primary surface (the
schemas, refusal lists and examples would no longer be visible where the agent decides; it is kept only as a
compatibility fallback); making focus a permission (authority would then depend on what a client chose to
list, and a client that never re-lists would be locked out of permitted work).

This bead owns the assignment of verbs to task clusters as registry data, the lints that keep it valid and the
measurement of the list budget on the generated definitions. The switching mechanism (enabling and disabling
tools per session) and its tests on a running server belong to the MCP beads BD-138 and BD-298, which depend on
this bead; the resource that serves the cluster directory is bead NJL-06.

## Scope

This bead is registry data and the checks that need no running server. It is the only place where a verb is
given a seat in a task cluster.

In: the registry form of a task cluster; the full assignment (seventeen principal clusters, eleven further
clusters the registry appendix names, and seventeen this bead adds so that every verb has a seat), which
includes every cluster that is served at M1; the lints; the tool tags; the list-budget measurement on the
generated tool definitions; the rules of focus, stated as the contract the assignment relies on.

Out, and which bead has it:

- The per-session enable and disable mechanism as code, the served default cluster and base tools, and the tests
  of the rules of focus on a running server: BD-138 at M1, for the clusters whose verbs are registered (script
  `mcp_m1_clusters.sh`), and BD-298 at M3, for the whole registry on the daemon, where GATE-TETHER-12 is
  recorded. Neither declares a cluster or seats a verb: both read the rows this bead writes.
- The fallbacks as code and their tests (the tag filter, the launch flag, `verb_invoke`): N32a-12.
- Capability tokens: BD-258. The cluster directory resource: NJL-06. The verbs' rows: the `VERBS-<cluster>`
  beads.

## Technical approach

**Registry form.** Task clusters are declared in `registries/verbs.toml` as their own tables, and each verb
entry repeats its memberships in `task_clusters`; the lint `task-cluster-mismatch` fails when the two disagree.

```toml
[[task_cluster]]
name = "author"
activity = "Make and change a design"
default = false
tools = ["design.new", "design.show", "design.edit", "design.diff", "design.check", "design.import", "control.synth"]
```

Each generated tool carries the tags `task:<cluster>` for every task cluster it sits in, beside `cluster:<name>`,
`kind:<kind>` and `tier:<S|F|M>`. The three base tools sit in no `[[task_cluster]]` table and have
`task_clusters = ["base"]` in the registry; their generated definitions carry `task:base` and, in addition, the
tag `task:<cluster>` of every declared cluster (DESIGN, as the fallback bead N32a-12 requires: a list filtered by
one cluster's tag must still hold the three tools that are always listed). The default cluster, listed before
any focus, is `orient`.

**Staging (DESIGN; the rule BD-138 already reads: "at M1 a cluster lists only the tools whose verbs are
registered").** Rows enter the registry with their cluster's `VERBS-<cluster>` bead, from M0 to M4, so the table
below is the **target** assignment and the registry reaches it at M4. Steps:
1. This bead commits the target as `fixtures/golden/verbs/task_clusters_target.toml` (45 clusters, tool lists as
   below) and declares all 45 `[[task_cluster]]` tables in `registries/verbs.toml` at M1, each `tools` list
   holding the target's tools whose rows are registered (a cluster may be short, or empty, until its rows land;
   focusing it lists what is registered plus the base three). Rows registered at M0 (the `operator`, `session`,
   `sim`, `spark` and `prove` clusters, before any table existed) get their memberships from this bead.
2. A `VERBS-<cluster>` bead that registers a row writes its `task_clusters` from the target and adds the tool to
   each of those tables in the same change, so `task-cluster-mismatch` holds at every commit.
3. The test `assignment_is_target_restricted` checks at every milestone that each declared table equals the
   target restricted to registered rows, and that every registered row's memberships equal the target's.
4. The counts (45 clusters, sizes, 241 seated plus three base tools) are asserted on the target file from M1 and
   on the registry itself once every row is registered (M4); `verb-without-task-cluster` applies to every
   registered row from M1. A regrouping edits the target and the tables in one change.

**The assignment** (tool names are the MCP names: the verb name with `.` and `-` replaced by `_`).

| Task cluster | Origin | Tools |
|---|---|---|
| `orient` | principal; list fixed by the surface chapter; the default | `capabilities`, `doctor`, `robot_docs`, `schema`, `design_templates`, `scene_list`, `session_show` |
| `author` | principal; list fixed by the surface chapter | `design_new`, `design_show`, `design_edit`, `design_diff`, `design_check`, `design_import`, `control_synth` |
| `analyze-body` | principal; list assigned here | `analyze_mobility`, `analyze_mass`, `analyze_symmetry`, `analyze_workspace`, `analyze_singularities`, `analyze_modes`, `analyze_self_collision` |
| `analyze-motion` | principal; list assigned here | `analyze_gait`, `analyze_maneuverability`, `analyze_stability`, `analyze_curvature`, `analyze_route`, `analyze_wrench`, `analyze_loop_stability` |
| `analyze-budget` | principal; list assigned here | `analyze_closure`, `analyze_envelope`, `analyze_structure`, `analyze_energy_budget`, `analyze_deadlines`, `analyze_bus_schedule`, `analyze_sensitivity` |
| `simulate` | principal; list fixed by the surface chapter | `sim_estimate`, `sim_run`, `sim_replay`, `run_fork`, `telemetry_summary`, `telemetry_failures`, `telemetry_stills` |
| `diagnose` | principal; list fixed by the surface chapter | `explain_run`, `autopsy_intervene`, `autopsy_attribute`, `autopsy_show`, `telemetry_query`, `telemetry_inspect_pixel`, `telemetry_energy` |
| `compare` | principal; list fixed by the surface chapter | `study_plan`, `study_submit`, `study_status`, `study_compare`, `eval_promote`, `experiment_suggest`, `predict_register` |
| `mission` | principal; list fixed by the surface chapter | `mission_new`, `mission_check`, `mission_synthesize`, `mission_refute`, `mission_distinguish`, `mission_show`, `result_revalidate` |
| `attack` | principal; list fixed by the surface chapter | `nemesis_search`, `nemesis_shrink`, `mission_separate`, `nemesis_bound`, `nemesis_oracle`, `nemesis_hardset`, `scene_show` |
| `qualify` | principal; list fixed by the surface chapter | `eval_qualify`, `eval_scorecard`, `eval_ledger`, `prove_bundle`, `prove_verify`, `report_design`, `prove_list` |
| `learn` | principal; list fixed by the surface chapter | `train_start`, `train_status`, `train_stop`, `train_export`, `eval_run`, `sim_estimate`, `telemetry_summary` |
| `places` | principal; list fixed by the surface chapter | `scene_gen`, `scene_import`, `scene_compile`, `scene_list`, `scene_show`, `telemetry_stills`, `design_check` |
| `search` | principal; list fixed by the surface chapter | `opt_start`, `opt_status`, `opt_archive`, `design_bisect`, `design_blame`, `solve_run`, `experiment_suggest` |
| `reality` | principal; list fixed by the surface chapter | `ground_ingest`, `ground_identify`, `ground_predict`, `ground_score`, `ground_band`, `predict_report`, `report_run` |
| `make` | principal; list fixed by the surface chapter | `foundry_size`, `foundry_optimize`, `foundry_check`, `foundry_tolerance`, `foundry_export`, `prove_bundle`, `report_design` |
| `operate` | principal; list assigned here | `job_list`, `job_status`, `job_find`, `artifact_list`, `artifact_read`, `artifact_export`, `artifact_import` |
| `families` | named by the registry appendix | `phylum_generate`, `phylum_gradient`, `phylum_map`, `phylum_plan`, `phylum_quotient`, `phylum_reuse`, `phylum_sweep` |
| `fidelity` | named by the registry appendix | `fidelity_ledger`, `fidelity_energy`, `fidelity_coverage`, `fidelity_modelform`, `fidelity_shadow`, `eval_promote`, `eval_rank_agreement` |
| `audit` | named by the registry appendix | `task_audit`, `eval_items`, `module_characterize`, `sense_visibility`, `control_lift` |
| `contact` | named by the registry appendix | `contact_laws`, `contact_probe`, `contact_capacity`, `contact_audit`, `contact_clearance`, `contact_soil`, `contact_response` |
| `sensing` | named by the registry appendix | `sense_alias`, `sense_calibrate`, `sense_causal_test`, `sense_observability`, `sense_profile`, `sense_schedule`, `sense_symmetry` |
| `gaits` | named by the registry appendix | `gait_atlas`, `gait_continue`, `gait_gradient`, `gait_orbit`, `control_ladder`, `control_mpc`, `control_symmetry` |
| `shield` | named by the registry appendix | `shield_budget`, `shield_check`, `shield_fit`, `shield_report`, `skill_compile`, `skill_check`, `skill_show` |
| `tasks` | named by the registry appendix | `task_check`, `task_fixture`, `task_list`, `task_new`, `task_settle`, `task_show`, `task_verdict` |
| `devices` | named by the registry appendix | `spark_cache`, `spark_check`, `spark_devices`, `spark_fingerprint`, `spark_place`, `spark_precompile`, `spark_probe` |
| `inherit` | named by the registry appendix | `train_family`, `train_inherit`, `train_distill`, `policy_apply`, `eval_detect`, `fidelity_modelform`, `eval_rank_agreement` |
| `ladder` | named by the registry appendix | `opt_cascade`, `nemesis_audit`, `eval_rank_agreement`, `cores_show`, `report_campaign`, `explain_compare`, `predict_report` |
| `analyze-evidence` | added by this bead | `analyze_coverage`, `analyze_observability`, `analyze_identifiability`, `analyze_rank_robustness`, `analyze_regimes` |
| `control` | added by this bead | `control_synth`, `control_show`, `control_estimate`, `control_ladder`, `control_mpc`, `control_symmetry`, `control_lift` |
| `studies` | added by this bead | `eval_compare`, `eval_preanalyze`, `study_cancel`, `study_plan`, `study_submit`, `study_status`, `study_compare` |
| `explain` | added by this bead | `explain_design`, `explain_run`, `explain_compare`, `report_design`, `report_run`, `report_mission`, `report_campaign` |
| `runs` | added by this bead | `run_show`, `run_list`, `run_export`, `sim_replay`, `telemetry_divergence`, `telemetry_histogram`, `telemetry_export_html` |
| `repair` | added by this bead | `autopsy_repair`, `autopsy_rerun`, `autopsy_show`, `run_fork`, `telemetry_divergence`, `design_edit`, `result_revalidate` |
| `modules` | added by this bead | `module_list`, `module_show`, `module_check`, `module_substitute`, `design_patch`, `design_view`, `design_export` |
| `places-check` | added by this bead | `scene_accept`, `scene_fit`, `scene_retrofit`, `scene_probe_request`, `scene_hazards`, `scene_split`, `contact_clearance` |
| `batteries` | added by this bead | `nemesis_cover`, `nemesis_hardset`, `eval_run`, `eval_scorecard`, `eval_items`, `task_list`, `scene_list` |
| `proofs` | added by this bead | `prove_kernel`, `prove_reflect`, `prove_list`, `prove_verify`, `prove_bundle`, `evidence_trace`, `result_show` |
| `transfer` | added by this bead | `mission_transport`, `skill_transport`, `mission_revalidate`, `result_revalidate`, `skill_show`, `skill_check`, `mission_show` |
| `housekeeping` | added by this bead | `session_open`, `session_close`, `store_status`, `store_collect`, `scene_bank`, `scene_purge`, `spark_cache` |
| `structure` | added by this bead | `foundry_loads`, `foundry_lifecycle`, `foundry_size`, `foundry_check`, `foundry_tolerance`, `analyze_structure`, `report_design` |
| `calibrate` | added by this bead | `ground_excite`, `ground_kit`, `ground_ingest`, `ground_identify`, `sense_calibrate`, `analyze_identifiability`, `ground_band` |
| `fleet` | added by this bead | `fleet_status`, `fleet_plan`, `spark_devices`, `spark_place`, `sim_estimate`, `phylum_plan`, `job_list` |
| `symmetry` | added by this bead | `algebra_irreps`, `algebra_orbits`, `analyze_symmetry`, `sense_symmetry`, `control_symmetry`, `phylum_quotient`, `gait_atlas` |
| `actuators` | added by this bead | `actuator_list`, `actuator_show`, `actuator_check`, `actuator_envelope`, `actuator_thermal`, `actuator_bus`, `analyze_envelope` |
| (base) | always listed, not counted | `session_focus`, `job_wait`, `job_cancel` |

How the three groups came about.
- *Principal* (17). The surface chapter fixes fourteen named rows, of which one row names the three analysis
  clusters; with `operate` that makes seventeen. Their tool lists are fixed, except the analysis clusters and
  `operate`, for which the chapter says only "the analyses, at most seven per cluster" and "the job, artifact,
  device and session verbs; the registry holds the assignment". The lists shown for those four are this bead's.
- *Named by the registry appendix* (11): `families`, `fidelity`, `audit`, `contact`, `sensing`, `gaits`,
  `shield`, `tasks`, `devices`, `inherit`, `ladder`. A DESIGN starting assignment that the lint checks.
- *Added by this bead* (17), because the lint "every verb sits in at least one task cluster" cannot be met by
  the 28 above: 61 verbs would have no seat.

Two places where the specification's own text cannot be satisfied as written, and what this bead does:
1. Twenty-six analyses do not fit three analysis clusters of seven (21 seats). A fourth, `analyze-evidence`,
   takes the five analyses of sensing and of recorded runs (A-14, A-17, A-25, A-19, A-26).
2. "The job, artifact, device and session verbs" are eleven verbs once the base tools are set aside (`job.list`,
   `job.status`, `job.find`; four `artifact` verbs, `artifact.import` among them; `spark.devices`;
   `session.open`, `session.show`, `session.close`). `operate` takes the three job verbs and the four artifact
   verbs, so that an agent with this one cluster focused can write a transfer bundle on one machine and admit it
   on the other (VERBS-artifact's reason for the seat); `spark.devices`, which held the seventh seat before
   `artifact.import` was registered, keeps its seats in `devices` and `fleet`; `session.show` is already in
   `orient`; `session.open` and `session.close` sit in `housekeeping`. Rejected: dropping a job verb instead
   (`job.status` and `job.find` have no other seat, and watching jobs is the cluster's first use).
The `fidelity` task cluster is written for the default of the open owner decision on `fidelity.rank` (bead
N16b-27): the five registered `fidelity` verbs with `eval_promote` and `eval_rank_agreement`. If the owner
registers `fidelity.rank`, it takes the seat of `fidelity_modelform`, which keeps its seat in `inherit`.

**Lints (each fails the build of `frob-verbs` with a named error).** `task-cluster-size` (more than seven
tools); `verb-without-task-cluster`; `base-tool-in-cluster` (a base tool listed inside a cluster: base tools
are implicit); `unknown-verb-in-cluster`; `duplicate-in-cluster`; `default-cluster-unique` (exactly one
`default = true`); `task-cluster-mismatch`; `task-cluster-name` (grammar `[a-z][a-z0-9]*(-[a-z0-9]+)*`, and no
collision with a base pseudo-cluster name `base`).

**Rules of focus** (the contract the assignment relies on. The mechanism is BD-138's and BD-298's, and so are the
tests of rules 1 to 4 on a running server: BD-138's script at M1 and BD-298's at M3; rule 5 is built and tested
by N32a-12. This bead tests only what follows from the registry: by the lint `task-cluster-size` no focused list
can exceed ten, and the tags of rule 5 are those of the generated definitions):
1. After `session_focus {cluster}` the `tools/list` holds exactly the tools of that cluster plus the three base
   tools: at most ten.
2. The result of `session_focus` lists the names and one-line purposes of the now-visible tools, for clients
   that do not act on the catalog-changed notification.
3. Focus changes what is listed, not what is permitted: `session_show` reports the same scopes and budgets
   before and after any focus change.
4. A direct call to a tool outside the focused cluster is refused by the MCP library as a disabled tool; after
   focusing its cluster the same call succeeds under the same token.
5. Fallbacks, in order: a connection without session state filters the list by the tag `task:<cluster>`
   (`includeTags`), which returns the cluster's tools and the three base tools; the launch flag `--clusters`
   fixes the listed set when the server is built; the compatibility flag exposes
   `verb_invoke {verb, arguments}`, validated against the verb's schema.

**The list budget.** For each task cluster, serialize compactly the `tools` array that `tools/list` returns
when that cluster is focused (base tools included) and take its UTF-8 length. TARGET: at most 12,000 bytes for
every cluster. A generated tool description is at most 800 characters (TARGET); the long form is the resource
`frob://docs/verb/{name}`. The measurement is a test of the generated definitions, so it runs without a server.

## Acceptance criteria

- [ ] The target file holds the 45 task clusters of the table, each with at most seven tools, and seats every
      one of the 244 verbs or names it a base tool; the registry declares the same 45 clusters from M1, each
      equal to the target restricted to registered rows; `orient` is the only default; every registered verb
      sits in at least one cluster or is a base tool, and at M4 the registry equals the target.
- [ ] The eight lints each fire on their planted defect and name it (8 of 8); the clean registry builds.
- [ ] The registry side of GATE-TETHER-12's focus clause (TARGET; M3) holds: no declared task cluster lists more
      than seven tools, so a focused list is at most seven tools plus the base three for every task cluster.
      The measurement on a running server is BD-138's at M1, for the clusters whose verbs exist, and BD-298's
      at M3, where the gate is recorded; so are the checks that `session_show` returns identical scopes and
      budgets before and after each focus change and that a tool outside the focus is refused until its
      cluster is focused.
- [ ] The list budget holds on the generated definitions: the largest focused list is at most 12,000 bytes; the
      measured maximum and the cluster that attains it are logged against the threshold.
- [ ] Every generated tool has one `task:<cluster>` tag per membership, each base tool has the tag of every
      cluster, and filtering the generated list by a cluster's tag returns exactly that cluster plus the three
      base tools.
- [ ] `frob robot-docs --cluster <name> --json` prints exactly the cluster's verbs, for all 45.
- [ ] The parity harness shows the same task-cluster memberships on the CLI documentation and the MCP tags.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test task_clusters` (the real registry; planted registries are TOML strings labelled
`synthetic`):
- `clusters_45_sizes` (on the target file): 45 clusters; none above seven tools; the five-tool clusters are
  `audit` and `analyze-evidence`; all others have exactly seven.
- `assignment_is_target_restricted` (on the registry): every declared table equals the target restricted to
  registered rows; a planted row whose `task_clusters` differs from the target fails, and so does a table that
  lists a registered tool the target does not put there.
- `every_verb_seated`: on the target, 241 verbs have at least one membership and the remaining three are the
  base tools (`session.focus`, `job.wait`, `job.cancel`), with `task_clusters = ["base"]`; on the registry,
  every registered row has a membership.
- `principal_lists_fixed`: the thirteen principal clusters whose lists the surface chapter fixes (`orient`,
  `author`, `simulate`, `diagnose`, `compare`, `mission`, `attack`, `qualify`, `learn`, `places`, `search`,
  `reality`, `make`) equal the table tool for tool.
- `default_is_orient`; `tags_match_memberships`; `memberships_bidirectional`; `base_tools_carry_every_task_tag`
  (filtering the generated definitions by `task:author` yields the seven tools of `author` and the three base
  tools).
- `lint_<name>` for the eight lints, one planted registry each.
- `list_budget_per_cluster`: the compact length of base tools plus each cluster is at most 12,000 bytes; the
  test prints the maximum.
- `shared_seats`: a verb in several clusters (`sim.estimate` in `simulate`, `learn` and `fleet`;
  `eval.rank-agreement` in `fidelity`, `inherit` and `ladder`) yields one tool with several `task:` tags, not
  several tools; `spark.devices` is in `devices` and `fleet` and not in `operate`, whose seven tools are the
  three job verbs and the four artifact verbs.

### End-to-end script and logging

`scripts/e2e/verbs_task_clusters.sh` (header: `bead=NJL-05 milestone=M1 labels=tether,verbs
gates=GATE-TETHER-12,GATE-TETHER-02 seed=1 needs=cpu plants=eight-tools,unseated-verb`). It needs no server
(DESIGN: a registry bead must be finishable before the mechanism that reads it, so the steps that drive
`frob-mcp` and `frobd --mcp` are in the scripts of BD-138, BD-298 and N32a-12). (1) Run the unit target above
on the real registry. (2) For each of the 45 clusters: `frob robot-docs --cluster <name> --json`, assert that
the verbs printed equal the registry's row (the `task:` tags of the generated definitions are checked by the
unit test `tags_match_memberships` of step 1). (3) For each cluster, take the generated `tools` array of the base tools
plus the cluster, assert a count of at most ten, measure the compact UTF-8 length and emit it as a metric
against 12,000. (4) Filter the generated definitions by `task:author` and assert the seven tools of `author`
plus the three base tools. (5) The plants: a registry with an eighth tool in `author` and one with an unseated
verb must each fail to build with the named lint. Logged through the shared helper to
`target/e2e-logs/verbs_task_clusters/<utc>.ndjson`: each command with its exact line, exit code and wall time;
per cluster the metrics `tools_listed` (<= 10) and `list_bytes` (<= 12000) tagged `--gate GATE-TETHER-12`;
assertions with observed and expected sets and the first differing tool name on failure; the `summary` with
the gates touched.

The steps this script no longer runs, and where each is now: the default list of a fresh connection, the list
after `session_focus` for each cluster with its size and bytes on a running server, `session_show` unchanged
across focus changes, and the disabled-tool refusal followed by success after focusing (or `FROB-REF-724`):
`scripts/e2e/mcp_m1_clusters.sh` of BD-138 at M1 and `scripts/e2e/mcp_full_surface.sh` of BD-298 at M3, on
stdio and loopback HTTP, the latter with the daemon; `includeTags` on a fresh connection of a running server:
N32a-12.

### Known answers and negative controls

- Known answers (exact): 45 task clusters; 17 principal, 11 named by the appendix, 17 added; 241 seated verbs
  and three base tools (240 and three for the specification's 243 verbs, row JL-05 of NJLx-02); the tool lists
  of the table.
- Negative controls: the eight planted registries. The two patched servers (one that lists eight tools after a
  focus; one that drops a scope on focus) and the metamorphic relation over the order of focus changes are
  controls of the mechanism and are in BD-138.
- Determinism: the generated tool definitions and their byte lengths are identical across runs, build
  profiles and platforms (class D0).

## Considerations

- OPEN-TETHER-2 (owner: Tether lead; blocks GATE-TETHER-12): do the agent harnesses in use speak one of the
  two protocol eras the MCP library serves, and act on catalog-changed notifications? Fail-closed default:
  static clusters by launch flag, plus `verb_invoke`. The check that the default works on a running server is
  N32a-12's; this bead's step (4) checks the tags it relies on.
- The seventeen added clusters are a starting assignment. The owner or a subsystem lead may regroup them; the
  lints, not this table, are the contract. A regrouping changes the help snapshots of the affected verbs
  (their `task_clusters` field), which is an additive change.
- Forty-five clusters is itself a list an agent must choose from; the directory resource (NJL-06) is the
  place where that choice is made cheap, with one line per cluster.
- If kill gate KILL-SOLVE fires (clause 1, before the verb is built: the pilot's plumbing-turn fraction is below
  30%, so the ceiling is under 1.4-fold; or clause 2 after the A/B), the `solve.run` row is withdrawn from the
  registry (BD-376 records the withdrawal; the name moves to the retired-name table of NJL-04 with `use =
  ["opt.cascade"]`), so no verb is left without a seat, and `opt_cascade` takes the vacated seat in `search`,
  which keeps seven tools. DESIGN, decided here at the request of the surface beads: `solve.run` is the
  agent-facing form of the co-design cascade, so without it the cascade itself is the tool an agent searching a
  design space reaches for; leaving it only in `ladder` would force a focus change for the activity's main
  verb. Rejected: leaving `search` at six tools (the seat would be wasted on the activity's most expensive gap).
  The swap is one edit of the `search` table and of `opt.cascade`'s `task_clusters`; the lints hold.
- From M3 NJLx-51 seats three late analysis verbs: `audit` then has seven tools and `analyze-evidence` six, and
  that bead edits `clusters_45_sizes` and `every_verb_seated` (244 seated) in the same commit.
- Risk: the 12,000-byte budget is tight for clusters of `run` verbs, whose input schemas carry six common
  properties each (NJL-01's kinds table, `envelope_scope` included). If a cluster exceeds it, the remedy is shorter descriptions (the long form is a resource),
  never an eighth tool removed silently.

## References

Appendix J (J.5), §32.12.4, §32.12.3, §32.13, §32.21, §32.22; GATE-TETHER-12, GATE-TETHER-02; OPEN-TETHER-2;
ADR-TETHER-01; KILL-SOLVE.
