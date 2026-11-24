---
key: "NJL-09"
title: "golden fixture L.9, part B: the simulated half of the MCP transcript (attack, diagnose, edit, qualify, bundle)"
type: "task"
priority: 2
milestone: "M3"
labels: ["m3", "tether", "nemesis", "autopsy", "arena", "frob-mcp", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-26"
deps: ["ART-L3", "ART-L6", "ART-L7", "BD-058", "BD-133", "BD-205", "BD-210", "BD-259", "BD-273", "BD-274", "BD-279", "BD-280", "BD-292", "BD-298", "N24b-15", "N24b-19", "NJL-08", "NJLx-02", "STD-E2E"]
needs: []
related: []
gates: []
plan_refs: ["Appendix L.9 (attack, diagnose, author then mission, attack again, qualify, offline; rules 4 to 11)", "§32.7", "§32.10", "§32.12.5", "§32.12.6", "§26.4", "§26.7", "§27.4", "§27.5", "§27.6", "§28.13", "§24.13", "§12.4"]
---
## Background

FrankenRobotics (a robotics simulation and design system) serves its verbs to agents as MCP tools through the
server `frob-mcp`; a tool response's `structuredContent` is the envelope `frob.envelope/1`. The specification's
ninth worked artifact is one agent session over MCP in the story of a quadruped that carries a tray through a
commercial-kitchen galley. Bead NJL-08 carries the first, exact half (the mission answer and the design
created from it) and the tool that drives a scripted session. This bead carries the second half: the agent
attacks the design with adversarial worlds, shrinks the worst failure, diagnoses it with interventions, edits
the design, revalidates, attacks again, qualifies the design in the blinded lane, bundles the certificates and
verifies the bundle offline.

The second half differs in kind from the first. Its responses are outcomes of simulated episodes: how many
worlds fail, by what margin, which interventions rescue the run. On the real simulator those values will not
be the specimen's. What can be pinned is the **shape** of every response, the **accounting** that ties them
together (budgets, counts, identities), the **exact arithmetic** inside them (the qualification levels and
bounds, the placebo rate), and the **rules** the session demonstrates. The fixture therefore classes most
fields as `structural`, and says which are exact.

Why keep a specimen whose numbers will not reproduce (DESIGN): it fixes what each envelope must contain, in a
sequence no single verb's test exercises, and its arithmetic is a known answer. Rejected: asserting specimen
values on produced output (it would never pass, or would be made to pass by blessing whatever came out).

## Scope

In: the transcript of the second half; the expectation of each step with its field classes; the accounting
invariants; the exact arithmetic of the qualification steps; rules 4 to 11 as assertions. Out: the driver and
the first half (NJL-08); the two hosting modes and the session-wide checks (ART-L9); the mechanisms (search,
shrinking, forks, attribution, revalidation, qualification, bundles: their own beads).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L9/`: `transcript.b.specimen.txt` (the text below);
`session.b.steps.json`; `expect/B1.json` … `expect/B14.json`.

**The transcript, second half** (`→` is a request, `←` a response; the last block is a shell command on a
machine with neither the agent, a GPU nor the simulator).

```text
── attack ──────────────────────────────────────────────────────────────────────────────────────────────
→ session_focus { "cluster": "attack" }
→ nemesis_search { "design": "quad-tray", "task": "t:kitchen-tray@88c1…", "legality": "kitchen-commercial/default",
                   "budget": { "gpu_s": 240 }, "idempotency_key": "nem-1", "sync_wait_ms": 5000 }
← { "verb": "nemesis.search", "ok": true, "exit": 8,
    "job": { "id": "j:77c0…", "state": "running", "progress": { "done": 512, "total": 4096, "unit": "worlds" },
             "eta_s": 190, "poll_after_ms": 5000, "last_committed": "tick 10000" },
    "next_actions": [ { "command": "job_wait j:77c0…", "why": "Accepted and still running." } ] }

→ job_wait { "job": "j:77c0…", "sync_wait_ms": 30000 }        (repeated until the job completes)
← { "verb": "nemesis.search", "ok": true, "exit": 2,
    "result": { "worlds_searched": 4096, "failures_promoted": 3, "classes": { "slip": 2, "timeout": 1 },
                "baseline_arm": { "method": "random within the profile", "failures_at_equal_budget": 1 },
                "worst": { "world": "w:9d04…", "run": "r:19e2…", "requirement": "delivered",
                           "binding_atom": "upright", "margin": -0.42, "edits": 7 },
                "artifact_guard": "all three reproduce on the referee" },
    "evidence": { "class": "Tested", "tier": "referee", "determinism": "D2", "claims": [], "reality": "no-claim" },
    "cost": { "gpu_s": 212 }, "budget": { "gpu_s_left": 1588, "cpu_core_s_left": 7140 } }

→ nemesis_shrink { "world": "w:9d04…", "idempotency_key": "shr-1", "sync_wait_ms": 30000 }
← { "verb": "nemesis.shrink", "ok": true, "exit": 2,
    "result": { "mcw": "w:3fa9…", "edits": 3, "minimality": "1-minimal; minimum among pairs", "tests": 42,
                "seeds_failing": "55/64", "oracle": "solvable-causal",
                "neighbours_passing": ["w:51b0…", "w:e7c4…", "w:0a9d…"] },              (the .fmcw of L.6)
    "cost": { "gpu_s": 61 }, "budget": { "gpu_s_left": 1527, "cpu_core_s_left": 7140 } }

── diagnose ────────────────────────────────────────────────────────────────────────────────────────────
→ session_focus { "cluster": "simulate" }
→ run_fork { "run": "r:19e2…", "world": "w:3fa9…", "before": "failure", "dt": "1.0 s", "classify_only": true }
← { "verb": "run.fork", "ok": true, "exit": 0,
    "result": { "fork": "f:a41c…", "tick": "10.30 s", "null_replay": "bitwise",
                "double_precision": "decision-equivalent",
                "classified": [
                  { "do": "scene./floor/patch[galley-2]/film := dry", "class": "contact-affecting",
                    "earliest_valid_tick": "10.30 s" },
                  { "do": "scene./fixtures/door[pass]/closer_torque := 14 N·m", "class": "exogenous",
                    "earliest_valid_tick": "10.30 s" },
                  { "do": "payload.tray.fill := 0.50", "class": "structural", "earliest_valid_tick": "0 s",
                    "forced_by": ["payload mass and slosh state", "policy history", "contact memory"] } ] } }

→ session_focus { "cluster": "diagnose" }
→ autopsy_intervene { "fork": "f:a41c…", "placebo": 64, "replicates": 8, "coalitions": "pairs",
                "do": [ "scene./floor/patch[galley-2]/film := dry",
                        "scene./fixtures/door[pass]/closer_torque := 14 N·m",
                        "payload.tray.fill := 0.50" ],
                "idempotency_key": "aut-1", "sync_wait_ms": 30000 }
← { "verb": "autopsy.intervene", "ok": true, "exit": 2, "result": { "fork": "f:a41c…", "forks_run": 113 } }
→ autopsy_attribute { "fork": "f:a41c…" }
← { "verb": "autopsy.attribute", "ok": true, "exit": 2,
    "result": {
      "schema": "frob.autopsy.attribution/1", "fork": "f:a41c…", "tier": "batch-cpu", "precision": "f64",
      "horizon_s": "2", "forks_run": 113, "null_replay": "bitwise",
      "placebo": { "n": 64, "flips": 2, "rate": 0.031, "separation_growth": "1.8e4" },
      "contrasts": [
        { "do": "scene./floor/patch[galley-2]/film := dry", "class": "contact-affecting", "rescued": "8/8",
          "contrast": 0.97, "flip_rate": { "1e-12": 0.0, "1e-7": 0.0 }, "label": "alternative" },
        { "do": "scene./fixtures/door[pass]/closer_torque := 14 N·m", "class": "exogenous", "rescued": "7/8",
          "contrast": 0.84, "flip_rate": { "1e-12": 0.0, "1e-7": 0.125 }, "label": "alternative" } ],
      "pairs": [ { "do": ["film := dry", "closer_torque := 14 N·m"], "rescued": "8/8", "interaction": -0.84 } ],
      "refused": [ { "do": "payload.tray.fill := 0.50", "code": "FROB-REF-581",
                     "earliest_valid_fork_tick": 0 } ],
      "evidence": { "class": "Estimated", "scope": "cause in the model at this tier, among the listed factors" },
      "next_actions": [
        { "command": "frob autopsy rerun r:19e2… --edit 'set-param foot.radius 0.030 m' --worlds failing,sample:64",
          "why": "Neither rescuing factor is the robot's to change; a design parameter is tested from the start." } ] },
    "cost": { "cpu_core_s": 840 }, "budget": { "gpu_s_left": 1527, "cpu_core_s_left": 6300 } }

── author, then mission ────────────────────────────────────────────────────────────────────────────────
→ session_focus { "cluster": "author" }
→ design_edit { "design": "quad-tray", "edit": "set-param",
                "path": "/limbs/of/of/chain/3/foot/region/sphere", "value": "0.030 m",
                "expected_parent": "d:7a3e…", "idempotency_key": "ed-1" }
← { "verb": "design.edit", "ok": true, "exit": 0,
    "result": { "design": "d:c05b…", "parent": "d:7a3e…", "symmetry": "preserved",
                "invalidated": ["settled poses", "baseline controller", "every rollout", "contact binding"] } }

→ session_focus { "cluster": "mission" }
→ result_revalidate { "design": "quad-tray", "since": "d:7a3e…", "idempotency_key": "rev-1" }
← { "verb": "result.revalidate", "ok": true, "exit": 2,
    "result": { "inherited": [ { "result": "a:8b27…",
                                 "why": "reads half-width, knee ratio and ratings; the foot is outside its read set" },
                               { "count": 8, "kind": "static analyses that do not read the foot region" } ],
                "invalidated": [ { "count": 4, "kind": "dynamic verdicts and the counterexample replay",
                                   "field": "foot region" } ],
                "reruns": { "count": 4, "estimate_gpu_s": 37 } } }

── attack again ────────────────────────────────────────────────────────────────────────────────────────
→ session_focus { "cluster": "attack" }
→ nemesis_search { "design": "quad-tray", "task": "t:kitchen-tray@88c1…", "legality": "kitchen-commercial/default",
                   "budget": { "gpu_s": 240 }, "idempotency_key": "nem-2", "sync_wait_ms": 30000 }
← { "verb": "nemesis.search", "ok": true, "exit": 0,
    "result": { "worlds_searched": 4096, "failures_promoted": 0,
                "note": "none found inside the profile; this is a search result and not a certificate" },
    "cost": { "gpu_s": 240 }, "budget": { "gpu_s_left": 1287, "cpu_core_s_left": 6300 } }

── qualify ─────────────────────────────────────────────────────────────────────────────────────────────
→ session_focus { "cluster": "qualify" }
→ eval_qualify { "design": "quad-tray", "task": "t:kitchen-tray@88c1…",
                 "claim": "failure probability of `delivered` at most 0.05", "dry": true }
← { "verb": "eval.qualify", "ok": true, "exit": 0,
    "result": { "campaign": "q:kitchen-tray/1", "index_if_committed": 1, "alpha_total": "0.05",
                "alpha_i": "0.05/2", "clusters": 200,
                "preanalysis": { "rule": "one-sided exact binomial bound", "qualified_reachable_with_failures_at_most": 3,
                                 "bounds": { "3": 0.0432, "4": 0.0504 } },
                "note": "nothing was committed and no level was spent" } }

→ eval_qualify { "design": "quad-tray", "task": "t:kitchen-tray@88c1…",
                 "claim": "failure probability of `delivered` at most 0.05",
                 "idempotency_key": "qual-1", "sync_wait_ms": 30000 }
← { "verb": "eval.qualify", "ok": true, "exit": 0,
    "result": { "schema": "frob.qualify.receipt/1", "campaign": "q:kitchen-tray/1", "index": 1,
                "alpha_i": "0.05/2", "alpha_spent": "0.05*1/2", "decision": "promoted", "units_used": 200,
                "sealed_until_close": ["world addresses", "per-unit outcomes", "telemetry", "stills"],
                "receipt_hash": "b3:e2a0…",
                "authorization": { "execution_authorized": false, "hardware_safety_signoff": false,
                                      "sim_to_real_certified": false } },
    "evidence": { "class": "Tested", "tier": "referee", "determinism": "D2",
                  "claims": ["Design comparison"], "reality": "no-claim" } }

→ prove_bundle { "design": "quad-tray", "mission": "q:3c9d…", "receipt": "b3:e2a0…",
                "out": "quad-tray.frobcert", "idempotency_key": "cb-1" }
← { "verb": "prove.bundle", "ok": true, "exit": 0,
    "result": { "bundle": "b3:d4a9…", "entries": 6, "in_tree_check": "accept" },                (the listing of L.7)
    "authorization": { "execution_authorized": false, "hardware_safety_signoff": false,
                       "sim_to_real_certified": false } }

── offline, on a machine with neither the agent, a GPU nor the simulator ───────────────────────────────
$ frob-verify quad-tray.frobcert
accept   6 of 6 entries; class of the bundle: Tested (weakest entry e6); all three authorization flags false
```

**Steps and expectations.**

| Step | Call | What is asserted (structural unless marked exact) |
|---|---|---|
| B1 | `session_focus {cluster: "attack"}` | Exact: the seven tools `nemesis_search`, `nemesis_shrink`, `mission_separate`, `nemesis_bound`, `nemesis_oracle`, `nemesis_hardset`, `scene_show` |
| B2 | `nemesis_search`, key `nem-1`, budget 240 GPU-seconds, `sync_wait_ms` 5,000 | If the job outlives the wait: `ok: true`, exit 8, a `job` with `id`, `state: "running"`, `progress {done, total, unit}` with a denominator, `eta_s`, `poll_after_ms`, `last_committed`; `next_actions` names `job_wait`. The call returns within 5,500 ms |
| B3 | `job_wait {job, sync_wait_ms: 30000}`, repeated until completion | The final envelope of `nemesis.search`: `worlds_searched`; `failures_promoted` with a count per class that sums to it; the baseline arm's failures at equal budget; the worst world with its run, requirement, binding atom, a negative margin and its edit count; the artifact guard's line; evidence class Tested with tier and determinism; cost within the 240 GPU-second budget; budget left = budget before − cost |
| B4 | `nemesis_shrink {world}`, key `shr-1` | A `.fmcw` identity; an edit count; a minimality label; `tests`; `seeds_failing` as a/64 with a ≥ 48; an oracle class that is solvable; one passing neighbour per edit (the invariants of artifact L.6) |
| B5 | `session_focus {cluster: "simulate"}`, then `run_fork {run, world, before: "failure", dt: "1.0 s", classify_only: true}`, key `fk-1` (the step file adds the key the specimen abridges: `run.fork` is a `run` verb and its registry example carries `--key fk-1`) | A fork handle; the fork tick; `null_replay: "bitwise"`; one classified entry per proposed change, each with one of the five fork classes and an earliest valid tick; a change to a property the payload carried from the first tick is `structural` with earliest valid tick 0 and a non-empty `forced_by`; with `classify_only` nothing is restored |
| B6 | `session_focus {cluster: "diagnose"}`, then `autopsy_intervene {fork, placebo: 64, replicates: 8, coalitions: "pairs", do: [three interventions]}`, key `aut-1` | `forks_run` equals the number of forks the request implies for the admitted interventions (placebo forks, replicates per single intervention and per pair, and the null replay) |
| B7 | `autopsy_attribute {fork}` | Schema `frob.autopsy.attribution/1`; tier and precision; `null_replay: "bitwise"`; placebo `{n: 64, flips, rate}` with rate = flips/64 (exact arithmetic: 2/64 = 0.03125); a contrast per admitted intervention with `rescued` as k/8, flip rates at the two perturbation scales 10⁻¹² and 10⁻⁷, and a label; pairs with an interaction; the intervention on the tray fill refused with `FROB-REF-581` and earliest valid fork tick 0; evidence class Estimated with scope "cause in the model at this tier, among the listed factors"; a `next_actions` command that is a registered CLI path; budget left = budget before − cost |
| B8 | `session_focus {cluster: "author"}`, then `design_edit {edit: "set-param", path: …/foot/region/sphere, value: "0.030 m", expected_parent}`, key `ed-1` | Exact: a new design identity whose parent is the design of the first half; `symmetry: "preserved"`; an invalidation set that contains settled poses, the baseline controller, every rollout and the contact binding. A repeat with a stale `expected_parent` is refused (`FROB-REF-804`, exit 7) |
| B9 | `session_focus {cluster: "mission"}`, then `result_revalidate {design, since}`, key `rev-1` | Exact: the refined mission answer is inherited, with the reason that its read set (half-width, knee ratio, ratings) does not contain the foot region. Structural: counts of inherited static analyses and of invalidated dynamic results, each with the forcing field; a rerun count with a cost estimate |
| B10 | `session_focus {cluster: "attack"}`, then `nemesis_search`, key `nem-2` | The result's shape; when `failures_promoted` is 0 the result carries the statement that none was found inside the profile and that this is a search result and not a certificate, and `evidence.claims` contains no Proof |
| B11 | `session_focus {cluster: "qualify"}`, then `eval_qualify {…, dry: true}` | Exact: `index_if_committed: 1`; `alpha_total: "0.05"`; `alpha_i: "0.05/2"`; with 200 clusters, `qualified_reachable_with_failures_at_most: 3` and bounds 0.0432 at 3 and 0.0504 at 4; nothing committed, no level spent, the campaign index unchanged |
| B12 | `eval_qualify`, key `qual-1` | Schema `frob.qualify.receipt/1`; exact: `index: 1`, `alpha_i: "0.05/2"`, `alpha_spent: "0.05*1/2"`; `units_used: 200`; `sealed_until_close` lists world addresses, per-unit outcomes, telemetry and stills; a receipt hash; the three flags false; evidence class Tested at the referee tier with claims [Design comparison]. Structural: the decision (`promoted` or `not-promoted`), which must equal the rule applied to the sealed counts |
| B13 | `prove_bundle {design, mission, receipt, out}`, key `cb-1` | `entries: 6`; `in_tree_check: "accept"`; a bundle root; the three flags false |
| B14 | `frob-verify quad-tray.frobcert` (offline) | `accept`, six of six entries; class of the bundle Tested, with the receipt as the weakest entry; all three authorization flags false |

**Accounting invariants across steps.** (1) For every resource, budget left after a step equals budget left
before it minus the step's cost, as exact decimals (in the specimen: 1,800 − 212 = 1,588; 1,588 − 61 = 1,527;
1,527 − 240 = 1,287 GPU-seconds; 7,140 − 840 = 6,300 CPU core-seconds). (2) The worst world of B3 is the input
of B4; the world of B4 is the world of B5; the fork of B5 is the fork of B6 and B7; the parent identity of the
edit of B8 is the `since` of B9. (3) The interventions of B6 are the inverses of the edits of the `.fmcw` of B4,
one per edit. (4) The receipt hash of B12 is the `receipt` of B13 and the sixth entry of the bundle. (5) The
bundle root of B13 is the root of the first half's refined answer extended by the receipt.

**Exact arithmetic of the qualification steps.** The campaign's total error budget is α = 0.05 and request i
receives α_i = α / [i·(i + 1)]: 0.025 for the first, 0.05/6 for the second. With 200 sealed clusters and the
claim "failure probability at most 0.05", the one-sided exact binomial upper bounds at level 0.025 are 0.0183,
0.0275, 0.0357, 0.0432 and 0.0504 for 0 to 4 failures, so a first request is promoted with at most 3 failures.
A second request at level 0.05/6 has bounds 0.0425 at 2 failures and 0.0505 at 3, so it admits at most 2: a
retry costs power. In the story the committed request observes 2 failures, bound 0.0357, and is promoted.

**Rules the second half demonstrates** (asserted).
4. *A long tool returns a handle.* The first search returns exit 8 with a job, a denominator and a poll
   interval; `job_wait` holds it with bounded waits. A dropped connection is recovered with `job.find` and the
   key `nem-1`: one job and one charge exist, or none do.
5. *A certified static answer does not make the robot walk.* The search may find failing worlds for a design
   that lies inside a certified cell. The two facts are about different claims, and the envelope of each says
   which (`claims: ["Proof"]` there; none here).
6. *A counterexample is shrunk before it is argued about*, with passing neighbours and a solvability witness.
7. *Attribution is interventional and labelled.* Two single interventions that each rescue the run are both
   labelled `alternative`; no factor is named when the placebo flip rate exceeds 0.2 (in the story 2 of 64). A
   property the payload carried from the first tick cannot be intervened on at the fork; the request is
   refused with the earliest valid tick, zero. The attribution is Estimated and is a cause in the model.
8. *An edit puts evidence in question, by read set.* The mission answer survives the foot edit because it never
   read the foot region; the dynamic results do not.
9. *"None found" is not a certificate*, and the result says so.
10. *Qualification spends error budget.* The dry run shows the level and what is reachable; the agent receives
    a receipt and nothing per world, and no scope in its token can read the sealed clusters before the
    campaign closes (`FROB-REF-840`).
11. *The weakest input sets the class.* Five Certified entries and one Tested receipt give a Tested bundle, and
    every exported object carries the three flags, all false: the bundle authorizes nothing.

## Acceptance criteria

- [ ] `transcript.b.specimen.txt` equals the text above byte for byte (manifest SHA-256).
- [ ] The fourteen steps run in order after the first half, against `frob-mcp` in process, and every assertion
      of the table holds on the recorded responses.
- [ ] The five accounting invariants hold on the produced session; in particular the budget identity holds at
      every step for both resources, as exact decimals.
- [ ] The exact arithmetic holds: `alpha_i` of the first request is 0.05/2; the dry run reports at most 3
      failures reachable with bounds 0.0432 and 0.0504; the decision of B12 equals the bound computed from the
      sealed counts by the auditor; a second dry run reports level 0.05/6 and at most 2 failures (0.0425,
      0.0505).
- [ ] Rule 4: with a search that outlives the wait, B2 returns exit 8 within 5,500 ms; killing the client
      connection during B3 and calling `job_find` with the key `nem-1` returns the same job, and the budget
      shows one charge.
- [ ] Rule 7: the intervention on the tray fill is refused with `FROB-REF-581` and earliest valid tick 0; with
      a placebo rate above 0.2 (a planted noisy fork) no factor is named.
- [ ] Rule 10: a read of any sealed field (world addresses, per-unit outcomes, telemetry, stills) from the
      session is refused with `FROB-REF-840`, and the receipt contains only the listed fields.
- [ ] Rule 11: the bundle of B13 has class Tested and the offline auditor accepts six of six entries.
- [ ] The test campaign is opened and closed by the administrative session through `eval.ledger`'s `open` and
      `close` actions, which take no key; the agent's session is refused `--action open` (FROB-REF-720, exit 5); while
      OPEN-ARENA-3 is undecided a non-synthetic claim family is refused `qualification-undecided` (exit 3).
- [ ] If a search finds no failure for the unedited design, steps B4 to B7 are logged as skips with that
      reason, never as passes, and the remaining steps run.

## Test plan

### Unit tests

`cargo test -p frob-conformance --test golden_l9_b` (the expectation files and the accounting checker on
recorded fixture responses labelled `synthetic`; the mechanisms are tested in their own crates):
- `b_transcript_manifest_hash`.
- `b_budget_identity_specimen`: 1,800 − 212 = 1,588; 1,588 − 61 = 1,527; 1,527 − 240 = 1,287; 7,140 − 840 =
  6,300.
- `b_placebo_rate`: 2/64 = 0.03125 < 0.2; 13/64 > 0.2 names no factor.
- `b_alpha_sequence`: α_1 = 1/40, α_2 = 1/120, α_3 = 1/240 as exact rationals; their partial sums are
  α·m/(m + 1).
- `b_binomial_bounds_200`: at 0.025: 0.0183, 0.0275, 0.0357, 0.0432, 0.0504; at 0.05/6: 0.0425 at 2 and 0.0505
  at 3.
- `b_reachable_failures`: at most 3 at the first level, at most 2 at the second.
- `b_class_join`: five Certified and one Tested → Tested.
- `b_interventions_invert_edits`: each `do` of B6 sets the path of one `.fmcw` edit back to its `from` value.
- `b_step_file_schema`; `b_expectations_reference_existing_pointers`.

### End-to-end script and logging

`scripts/e2e/art_l9_session_b.sh` (header: `bead=NJL-09 milestone=M3 labels=golden,appendix-l,mcp,
verb:nemesis.search,verb:nemesis.shrink,verb:run.fork,verb:autopsy.intervene,verb:autopsy.attribute,
verb:design.edit,verb:result.revalidate,verb:eval.qualify,verb:prove.bundle,verb:job.wait,verb:job.find
seed=901 needs=cpu,cuda,mcp,frobd plants=double-charge,sealed-read`). It runs the first half's step file, then
`session.b.steps.json`, through `frob-mcp-drive` over stdio (after opening the test campaign under the
administrative session, as Considerations specifies), and asserts each row of the table and each
accounting invariant on the recorded responses (`e2e_assert_json`, `e2e_assert_num` for budget identities and
bounds, `e2e_assert_digest_eq` for identities that flow between steps). It then runs the connection-kill
check of rule 4, the sealed-read refusals of rule 10 (`e2e_expect_refusal FROB-REF-840 5`: the one Arena code with exit 5,
N24-REF and the firewall BD-261) and the stale-parent
refusal of B8, then closes the test campaign. Finally it copies the bundle to a scratch directory with an empty
environment and runs `frob-verify`. Plants: `double-charge` (a server patched to charge a retried key twice) must fail the budget
identity; `sealed-read` (a planted leak path) must be caught by the sealed-field assertion. The log
(`target/e2e-logs/art_l9_session_b/<utc>.ndjson`) has one `cmd` line per MCP request with the request
verbatim, wall time, the envelope's exit code and its `evidence` block (claims, class, tier, determinism,
identities, seed); metrics per step for cost and budget left with their identities as thresholds, and for
`wait_overrun_ms` (<= 500); `assert` lines with observed and expected values and the step identifier; skips
with reasons; refusals with code and first repair; and the `summary`. No sealed field is ever written to the
log.

### Known answers and negative controls

- KA-762 (exact binomial bound, 200 worlds, α = 0.025: 0.0183, 0.0275, 0.0357, 0.0432, 0.0504 for 0 to 4
  failures; to the printed digits) and KA-760 (Σ over i = 1…n of α/[i·(i + 1)] = α·n/(n + 1) < α; exact) are
  the arithmetic of B11 and B12. KA-763 (one job and one charge per key; the retry returns the stored receipt)
  is rule 4.
- Known answers of this bead (recomputed for it): the second-request level 0.05/6 with bounds 0.0425 and
  0.0505; the budget identities of the specimen. They are row JL-18 of NJLx-02's known-answer block (a
  dependency), which this bead's tests cite by number once the block is issued.
- Counterexample kept alive: retrying a held-out test until it passes. With independent attempts at level 5%
  each, fourteen null submissions give a 51% chance of at least one false promotion; the level sequence
  α/[i·(i + 1)] keeps the campaign-wide chance at most α for any number of retries.
- Negative controls: the two plants; the noisy placebo; the stale parent; a search result with zero failures
  that carries a Proof claim must fail B10.
- Determinism lanes: the simulated steps at class D2 twice on one backend (identical digests); the exact
  arithmetic in debug and release, on Linux and macOS (class D0).

## Considerations

- `forks_run` in the specimen (113) is a story value: 64 placebo forks and the null replay leave 48 for
  replicated single and pair interventions, which is the count for three admitted interventions, while the
  transcript refuses one of the three. The step's assertion is therefore the count implied by the admitted
  interventions of the produced session, not the specimen's number. Recorded in the unit's coverage file.
- The specimen's `forced_by` list names a slosh state of the payload; modelling slosh is a later epic. The
  assertion is on the class, the earliest tick and a non-empty list that names a payload field.
- The session costs about 500 GPU-seconds and 900 CPU core-seconds in the story; in small mode the script
  runs the exact arithmetic, the dry run and the format checks only, and the log says nothing is claimable.
- Kill gate KILL-AUTOPSY (planted-cause accuracy below 90%, or the top factor changes in more than 30% of
  cases at 10⁻⁷): fallback, single-factor paired re-runs from the episode start; steps B6 and B7 then assert
  the fallback's shape, without coalitions.
- B13 calls the receipt form of `prove.bundle` (`design`, `mission`, `receipt`), which N24b-19 adds at M3
  together with the campaign section of the bundle; BD-133's mission-slice form takes an answer only. N24b-19
  is a dependency for that reason.
- The qualification of B12 consumes sealed clusters of a real campaign; the script uses a campaign opened for
  the test in the scratch root, so that no production campaign's index moves. It is opened and closed with
  the admin actions of `eval.ledger` (N24b-15), under a second, administrative session holding `qual:admin`
  that the agent's session never sees: before the first half, `frob eval ledger --action open --task
  t:kitchen-tray@88c1… --world-family <the fixture's synthetic family> --json` (the record is the
  campaign `q:kitchen-tray/1` of the transcript); after the sealed-read refusals of rule 10 and before the
  offline step, `frob eval ledger <campaign> --action close --json` (admin actions take no key: `--key` on
  `eval.ledger` is FROB-REF-802). While OPEN-ARENA-3 is undecided
  (`decided: false` in `config/qualification.toml`), opening on a non-synthetic claim family is refused with
  `qualification-undecided` (exit 3), so the fixture's task family carries the `synthetic` label; the script
  asserts that refusal once on a non-synthetic family (`e2e_expect_refusal` by name), and logs which posture
  held. The agent's session calling `--action open` is refused with FROB-REF-720, exit 5 (asserted once).

## References

Appendix L.9 (second half; rules 4 to 11), §32.7, §32.10, §32.12.5, §32.12.6, §26.4, §26.7, §27.4 to §27.6,
§28.13, §24.13, §12.4; INV-TETHER-05, INV-TETHER-11 (a cancelled wait is not a cancelled job), INV-QUAL-01
(discovery cannot read confirmation worlds), INV-QUAL-03 (the ledger of levels is append-only and the total
spent never exceeds α); KA-760, KA-762, KA-763; KILL-AUTOPSY.
