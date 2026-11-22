---
key: "NJL-05"
title: "frob-verbs registry: task clusters of at most seven tools: 45-cluster assignment, lints and the 12 kB list budget"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "frob-verbs", "solid", "registry", "verb", "test", "gate"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "STD-E2E"]
needs: []
related: ["BD-138", "BD-298", "BD-258", "BD-313", "NJL-03", "NJL-06", "N28a-05", "N16b-27"]
gates: ["GATE-TETHER-12", "GATE-TETHER-02"]
plan_refs: ["Appendix J (J.5)", "§32.12.4", "§32.12.3", "§32.13", "§32.21 (GATE-TETHER-12)", "§32.22 (OPEN-TETHER-2)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) declares every capability as a verb in one registry,
`registries/verbs.toml`, from which the MCP server `frob-mcp` generates one tool per verb (MCP: the Model
Context Protocol, by which an agent harness lists and calls tools). The registry holds 243 verbs. A **registry
cluster** is a namespace and can be large: `scene` has thirteen verbs, `design` twelve, `analyze` twenty-six.
What an agent sees is a **task cluster**: at most seven tools chosen for one activity, declared in the registry
beside the verbs. A verb may sit in several task clusters, and a lint requires every verb to sit in at least
one. Three **base tools** are always listed and are not counted: `session_focus`, `job_wait`, `job_cancel`. An
agent switches with `session_focus {cluster}`; focus changes context size, never authority, which is the
session's capability token.

Why seven, and why focus (DESIGN). Agents choose badly among long tool lists, and every tool definition an
agent does not need costs context on every turn. TARGET: the definitions of the base tools and one task cluster
occupy at most 12 kB of the `tools/list` response. Rejected: listing all 243 tools (about a megabyte of
definitions, and no agent chooses well among them); one generic `invoke` tool as the primary surface (the
schemas, refusal lists and examples would no longer be visible where the agent decides; it is kept only as a
compatibility fallback); making focus a permission (authority would then depend on what a client chose to
list, and a client that never re-lists would be locked out of permitted work).

This bead owns the assignment of verbs to task clusters as registry data, the lints that keep it valid and the
measurement of the list budget. The switching mechanism (enabling and disabling tools per session) belongs to
the MCP beads BD-138 and BD-298; the resource that serves the cluster directory is bead NJL-06.

## Scope

In: the registry form of a task cluster; the full assignment (seventeen principal clusters, eleven further
clusters the registry appendix names, and seventeen this bead adds so that every verb has a seat); the lints;
the tool tags; the list-budget measurement; the rules of focus as checks. Out: the per-session enable and
disable mechanism and its fallbacks as code (BD-138, BD-298); capability tokens (BD-258); the cluster directory
resource (NJL-06); the verbs' rows (the `VERBS-<cluster>` beads).

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
`kind:<kind>` and `tier:<S|F|M>`. The default cluster, listed before any focus, is `orient`.

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
| `operate` | principal; list assigned here | `job_list`, `job_status`, `job_find`, `artifact_list`, `artifact_read`, `artifact_export`, `spark_devices` |
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
2. "The job, artifact, device and session verbs" are ten verbs once the base tools are set aside (`job.list`,
   `job.status`, `job.find`; three `artifact` verbs; `spark.devices`; `session.open`, `session.show`,
   `session.close`). `operate` takes the first seven; `session.show` is already in `orient`; `session.open` and
   `session.close` sit in `housekeeping`.
The `fidelity` task cluster is written for the default of the open owner decision on `fidelity.rank` (bead
N16b-27): the five registered `fidelity` verbs with `eval_promote` and `eval_rank_agreement`. If the owner
registers `fidelity.rank`, it takes the seat of `fidelity_modelform`, which keeps its seat in `inherit`.

**Lints (each fails the build of `frob-verbs` with a named error).** `task-cluster-size` (more than seven
tools); `verb-without-task-cluster`; `base-tool-in-cluster` (a base tool listed inside a cluster: base tools
are implicit); `unknown-verb-in-cluster`; `duplicate-in-cluster`; `default-cluster-unique` (exactly one
`default = true`); `task-cluster-mismatch`; `task-cluster-name` (grammar `[a-z][a-z0-9]*(-[a-z0-9]+)*`, and no
collision with a base pseudo-cluster name `base`).

**Rules of focus, as checks** (the mechanism is BD-138's and BD-298's; these are the properties this bead's
script asserts on the served surface):
1. After `session_focus {cluster}` the `tools/list` holds exactly the tools of that cluster plus the three base
   tools: at most ten.
2. The result of `session_focus` lists the names and one-line purposes of the now-visible tools, for clients
   that do not act on the catalog-changed notification.
3. Focus changes what is listed, not what is permitted: `session_show` reports the same scopes and budgets
   before and after any focus change.
4. A direct call to a tool outside the focused cluster is refused by the MCP library as a disabled tool; after
   focusing its cluster the same call succeeds under the same token.
5. Fallbacks, in order: a connection without session state filters the list by the tag `task:<cluster>`
   (`includeTags`); the launch flag `--clusters` fixes the listed set when the server is built; the
   compatibility flag exposes `verb_invoke {verb, arguments}`, validated against the verb's schema.

**The list budget.** For each task cluster, serialize compactly the `tools` array that `tools/list` returns
when that cluster is focused (base tools included) and take its UTF-8 length. TARGET: at most 12,000 bytes for
every cluster. A generated tool description is at most 800 characters (TARGET); the long form is the resource
`frob://docs/verb/{name}`. The measurement is a test of the generated definitions, so it runs without a server.

## Acceptance criteria

- [ ] The registry declares the 45 task clusters of the table, each with at most seven tools; `orient` is the
      only default; every one of the 243 verbs sits in at least one cluster or is one of the three base tools.
- [ ] The eight lints each fire on their planted defect and name it (8 of 8); the clean registry builds.
- [ ] GATE-TETHER-12, the focus clause (TARGET; M3; first measured at M1 for the clusters whose verbs exist):
      after `session_focus` the list holds at most seven tools plus the base three, for every task cluster.
- [ ] `session_show` returns identical scopes and budgets before and after each focus change (focus is not
      authority), and a tool outside the focus is refused until its cluster is focused.
- [ ] The list budget holds: the largest focused list is at most 12,000 bytes; the measured maximum and the
      cluster that attains it are logged against the threshold.
- [ ] Every generated tool has one `task:<cluster>` tag per membership, and filtering `tools/list` by that tag
      returns exactly the cluster.
- [ ] `frob robot-docs --cluster <name> --json` prints exactly the cluster's verbs, for all 45.
- [ ] The parity harness shows the same task-cluster memberships on the CLI documentation and the MCP tags.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test task_clusters` (the real registry; planted registries are TOML strings labelled
`synthetic`):
- `clusters_45_sizes`: 45 clusters; none above seven tools; the five-tool clusters are `audit` and
  `analyze-evidence`; all others have exactly seven.
- `every_verb_seated`: 240 verbs have at least one membership; the remaining three are the base tools
  (`session.focus`, `job.wait`, `job.cancel`), with `task_clusters = ["base"]`.
- `principal_lists_fixed`: the thirteen principal clusters whose lists the surface chapter fixes (`orient`,
  `author`, `simulate`, `diagnose`, `compare`, `mission`, `attack`, `qualify`, `learn`, `places`, `search`,
  `reality`, `make`) equal the table tool for tool.
- `default_is_orient`; `tags_match_memberships`; `memberships_bidirectional`.
- `lint_<name>` for the eight lints, one planted registry each.
- `list_budget_per_cluster`: the compact length of base tools plus each cluster is at most 12,000 bytes; the
  test prints the maximum.
- `shared_seats`: a verb in several clusters (`sim.estimate` in `simulate`, `learn` and `fleet`;
  `eval.rank-agreement` in `fidelity`, `inherit` and `ladder`) yields one tool with several `task:` tags, not
  several tools.

### End-to-end script and logging

`scripts/e2e/verbs_task_clusters.sh` (header: `bead=NJL-05 milestone=M1 labels=tether,verbs,mcp
gates=GATE-TETHER-12,GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=eight-tools,unseated-verb`). Against
`frob-mcp` over stdio and against `frobd --mcp` over loopback HTTP: (1) `tools/list` before any focus, assert
the ten tools of `orient` plus base; (2) `session_show`, record scopes and budgets; (3) for each of the 45
clusters: `session_focus {cluster}`, assert the result's tool names equal the registry's; `tools/list`, assert
the same set plus the three base tools and a count of at most ten; measure the byte length of the list and
emit it as a metric against 12,000; (4) `session_show` again, assert scopes and budgets unchanged
(`e2e_assert_json`); (5) with `mission` focused, call `design_check` and assert the library's disabled-tool
error, then focus `author` and assert the call succeeds or answers `FROB-REF-724`; (6) list with `includeTags:
["task:author"]` on a fresh connection and assert the seven tools; (7) the plants: a registry with an eighth
tool in `author` and one with an unseated verb must each fail to build with the named lint. Logged through the
shared helper to `target/e2e-logs/verbs_task_clusters/<utc>.ndjson`: each call with its exact request, exit or
error code and wall time; per cluster the metrics `tools_listed` (<= 10) and `list_bytes` (<= 12000) tagged
`--gate GATE-TETHER-12`; assertions with observed and expected sets and the first differing tool name on
failure; the `summary` with the gates touched.

### Known answers and negative controls

- Known answers (exact): 45 task clusters; 17 principal, 11 named by the appendix, 17 added; 240 seated verbs
  and three base tools; the tool lists of the table.
- Negative controls: the eight planted registries; a server patched to list eight tools after a focus; a
  server patched to drop a scope on focus (must fail step 4).
- Metamorphic relation: focusing clusters in any order leaves `session_show` and every later tool result
  unchanged (focus is not state that enters a result).
- Determinism: the generated tool definitions and their byte lengths are identical across runs, build
  profiles and platforms (class D0).

## Considerations

- OPEN-TETHER-2 (owner: Tether lead; blocks GATE-TETHER-12): do the agent harnesses in use speak one of the
  two protocol eras the MCP library serves, and act on catalog-changed notifications? Fail-closed default:
  static clusters by launch flag, plus `verb_invoke`. The script's step (6) is the check that the default works.
- The seventeen added clusters are a starting assignment. The owner or a subsystem lead may regroup them; the
  lints, not this table, are the contract. A regrouping changes the help snapshots of the affected verbs
  (their `task_clusters` field), which is an additive change.
- Forty-five clusters is itself a list an agent must choose from; the directory resource (NJL-06) is the
  place where that choice is made cheap, with one line per cluster.
- If kill gate KILL-SOLVE fires, `solve_run` leaves `search`, which then has six tools; no lint is affected.
- Risk: the 12,000-byte budget is tight for clusters of `run` verbs, whose input schemas carry five common
  properties each. If a cluster exceeds it, the remedy is shorter descriptions (the long form is a resource),
  never an eighth tool removed silently.

## References

Appendix J (J.5), §32.12.4, §32.12.3, §32.13, §32.21, §32.22; GATE-TETHER-12, GATE-TETHER-02; OPEN-TETHER-2;
ADR-TETHER-01; KILL-SOLVE.
