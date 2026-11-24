---
key: "N23-CX"
title: "Cortex counterexample registry: CX-246 to CX-258 and five late cases, seeding the negative controls"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "cortex", "gauntlet", "frob-conformance", "frob-learn", "solid", "registry", "test"]
parent: "WP-48"
deps: ["BD-025", "N38a-40", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N23-KA", "N23-REF", "N23b-07", "N23b-08", "N23b-15", "N23b-17", "N23b-29", "N23b-30", "N23b-32", "N23b-33", "N23b-34", "N23b-35", "N23b-38", "N23b-39", "N33c-37", "N38b-04"]
gates: []
plan_refs: ["Appendix N (N.20)", "§23.3", "§23.4", "§23.5", "§23.6", "§23.9", "§23.16", "§23.21", "§23.25", "§23.26", "§23.27", "§23.28", "§23.29", "§38.11"]
---
## Background

Cortex is the learning subsystem of FrankenRobotics (crate `frob-learn`, layer 4, with the optional annex
`bindings/frob-gymnasium`): the vectorized environment `VecEnv`, the learning algorithms, networks, checkpoints,
family policies, inheritance and the detectors that keep a learned result from resting on a simulator artifact.

A *counterexample* in this project is the smallest worked case that breaks a tempting shortcut. Each one protects
a rule, and each one becomes at least one *negative control*: a planted defect (a mutated build, a planted input or
a planted policy) that a gate must catch every time it runs. The standing rule is that a gate which no deficient
implementation can fail validates nothing; a gate run in which a registered control was not caught is a FAIL of the
gate whatever the passing test says; and a control that has never been caught in any recorded run is itself suspect
and is reviewed at each milestone exit.

This bead is the registry of the counterexamples that belong to Cortex: the thirteen numbered entries CX-246 to
CX-258 and five late cases that were written after the numbered catalogue closed and have no number yet. It carries
every case with its numbers, so that no mechanism bead has to rediscover them, and it gives each entry what the seeding tool needs to turn it into a control
record. Why a separate bead: the cases are owned by many mechanism beads, and a control that lives only in the test
file of one mechanism is not visible to the gate runner, which reads the registry and nothing else.

## Scope

In: the eighteen cases below as entries of the counterexample catalogue
`crates/frob-conformance/registry/counterexamples.toml`, each with the substance it needs to become a control
(the rule identifiers and gates it protects, the kind of control, and the identifier of its planted defect); the
block request for the numbers of the five late cases; a host test that recomputes every number; the script that
shows each planted defect is caught.
Out, with the bead that has it: the catalogue's record schema and loader, and the seeding tool that turns
catalogue entries into rows of `crates/frob-conformance/registry/controls.toml`, allocates their `NC-`
identifiers, keeps the seed map and proves completeness by the join (N38b-05, a dependency). This bead writes no
row of `controls.toml` by hand: its controls appear there when the seeding is re-run over its entries, which is
append-only and idempotent (DESIGN, correcting an earlier draft that wrote control records directly: one writer
of that file means control identifiers are allocated once and never rewritten). The control runner and the rule
that a gate with no control cannot pass (N38b-04). The number blocks (N38a-40). The mechanisms themselves, the
planted variants (built by the bead named in each row) and their gate measurements. The planted-exploit corpus
for the exploitation detectors is N23b-07; the chapter's fixture battery is N23b-18.

## Technical approach

**Data.** Each row below is one entry of the catalogue in its record schema (N38b-05): `id` (the CX number),
`shortcut`, `case`, `numeric` (true for all eighteen), `plan_case` (`"numbers"` for all eighteen), `rule`,
`rule_ids` (the gates of the last column with the invariants, obligations, rules and refusal codes of the rule
column), `defined_in`, `same_as`, `label` (DESIGN, or FACT with its source for CX-250 to CX-252 and the first
late case, whose figures were read in a sibling's source; they are registered with `rerun = false` and re-run by
the beads named under **FACT entries** below), `rerun` and `rerun_receipt` on those four,
`control_kind` and `plant`, and for a late entry `slug`. The fifth column gives the control's kind, target,
defect and expected observation: its first word is `control_kind`; the planted variant it describes is the
entry's `plant` (for a `mutation`, the Cargo feature that puts the shortcut into the real component, built by
the bead that owns the component; for the other kinds, the fixture); and the defect and the expected
observation are the texts the seeding tool copies into the control's `defect` and `expect`. Every gate among an
entry's `rule_ids` must fail with the plant enabled.

**The numbered cases.** "Gates" lists every gate whose run must catch the control.

| CX | The tempting shortcut | The smallest case that breaks it, with numbers | The rule it protects | Control (kind; target; defect; expected observation) | Gates |
|---|---|---|---|---|---|
| CX-246 | One `done` flag is enough for bootstrap targets | A true 8 s mission deadline with terminal reward 0 needs bootstrap target 0. An external collection cut at the same instant on a continuing task with V = 10 and γ = 0.99 needs γ·V = 9.9. One flag gives 0 for both | Termination and truncation are different facts (environment rule E-3); the value bootstrap is cut by termination only and the advantage trace at every reset (PPO rule P-1) | mutation; advantage recursion in `frob-learn`; `terminated` and `truncated` merged into one flag; the truncation row of the fixture returns 0 where 9.9 is true | GATE-CORTEX-06, GATE-CORTEX-07 |
| CX-247 | A per-step reward is the same reward at any control rate | A penalty of 1 per control step over a 10 s episode totals 500 at 50 Hz and 1,000 at 100 Hz for the same motion | Time-aware rewards: a running term is a rate integrated over simulated time across the physics substeps | planted-input; a task whose penalty term is declared per step; totals 500 and 1,000 differ, where the rate form gives one total | GATE-CORTEX-06 |
| CX-248 | A per-step discount is the same discount at any control rate | With γ = 0.99 per step a constant reward rate r is worth r·Δt/(1 − γ): 2.0·r at Δt = 20 ms and 1.0·r at 10 ms. With γ(Δt) = exp(−β·Δt), β = 0.5025 /s, it is r·Δt/(1 − e^(−β·Δt)): 2.000·r and 1.995·r, both near r/β = 1.990·r | Physical discounting (P-7) | mutation; discount computation; one per-step γ = 0.99 at both rates; the 10 ms row reads 1.0·r | GATE-CORTEX-07 |
| CX-249 | The density of the sampled Gaussian is the likelihood of the applied clipped action | Unit-variance Gaussians clipped at +1 with means 0 and 1. The clipped policy has a point mass at the bound: P(a = 1) is 1 − Φ(1) = 0.158655 under mean 0 and 0.5 under mean 1, ratio 0.5/0.158655 = 3.15149. The latent densities at +1 are φ(0)/φ(1) = e^0.5 = 1.64872 | The likelihood ratio is for the variable that was sampled (P-3, INV-CORTEX-03); a mixture of the two laws is refused (FROB-REF-527) | mutation; likelihood evaluation; collected under `clipped-with-endpoint-mass`, scored under `latent-then-limit`; the ratio reads 1.64872 where 3.15149 is registered, and no refusal fires | GATE-CORTEX-07 |
| CX-250 | The clipped surrogate's branch test is the same for both signs of the advantage | L = min(ρ·A, clip(ρ, 1 − ε, 1 + ε)·A), ε = 0.2, A < 0, ratios 0.5, 1, 2: finite differences of the fixture loss in log σ are 0, −0.5, −1. The reversed branch gives −0.25, −0.5, 0 | P-4 | mutation; surrogate gradient; branch test reversed for A < 0 (follow the ratio below 0.8, clip above 1.2); values −0.25, −0.5, 0 | GATE-CORTEX-07 |
| CX-251 | The value loss may use the value stored at collection | Critic output 2, stored value 0, return 0, loss c_V·(V − R)²: the gradient into the value head is 2·c_V·(2 − 0) = 4·c_V. The stored-value form gives 0 | P-5 | mutation; value-loss gradient; differentiates the stored value; gradient 0 | GATE-CORTEX-07 |
| CX-252 | Concatenated episodes may be evaluated as one sequence | Two independently reset one-step episodes packed together, frozen policy: likelihood ratios 1 and 1. Reading across the reset gives a second ratio of 1.582459 | P-6; PO-CORTEX-02 | mutation; sequence evaluation; hidden state carried across the reset; second ratio 1.582459 | GATE-CORTEX-07 |
| CX-253 | Average the shard means | One sample with gradient 1 and three with gradient 0, shards of sizes 1 and 3: global mean 0.25; mean of shard means (1 + 0)/2 = 0.5; with an empty shard it divides by zero or halves the answer | Sums and global counts across shards (multi-device rule; P-10) | mutation; shard reduction; averages per-shard means; 0.5 | GATE-CORTEX-08 |
| CX-254 | Normalize with each shard's own variance | Shards {0, 0} and {4}: global mean 4/3, variance 32/9 (population form: ((4/3)² + (4/3)² + (8/3)²)/3 = 32/9); each shard's own variance is 0 | Exact pairwise merge of (n, μ, S): n = n_a + n_b, d = μ_b − μ_a, μ = μ_a + d·n_b/n, S = S_a + S_b + d²·n_a·n_b/n | mutation; advantage normalization; per-shard statistics; variance 0, division by zero or unnormalized output | GATE-CORTEX-08 |
| CX-255 | Copy stride numbers between array libraries | A contiguous 2 × 2 `f32` view has strides (8, 4) in signed bytes and (2, 1) in unsigned elements. Copied verbatim onto a 16-element buffer, index [1, 1] reads element 8 + 4 = 12 where element 2 + 1 = 3 belongs, and the read is in bounds. Byte origin 4 with strides (8, −4) indexes elements (1, 0, 3, 2) and has no unsigned form | One conversion boundary; unrepresentable views are materialized with a charged copy or refused (FROB-REF-547); PO-CORTEX-05 | mutation; stride converter; verbatim copy; element 12 | GATE-CORTEX-09 |
| CX-256 | Take the minimum of raw margins in mixed units | min(0.05 m, 12 K) is 0.05 in metres and 12 with the clearance written as 50 mm. With scales 0.10 m and 10 K the margins are 0.5 and 1.2 and the clearance binds in every unit system | Unit-normalized predicates ρ = ψ/s before any min or max | planted-input; a requirement with an unscaled atom; the binding atom changes with the unit | GATE-CORTEX-10 |
| CX-257 | A learning gate passes when the pipeline runs | An exported checkpoint with an all-zero policy head passes a test that training runs, exports and reloads. Zero-weight, random and shuffled-label policies must fail the same return threshold | Behaviour, not plumbing (INV-CORTEX-05; FROB-REF-544) | planted-design; a gate record whose "trained" policy is the zero-weight policy; the record must fail its threshold and a record without controls must be refused | GATE-CORTEX-01, GATE-CORTEX-14 |
| CX-258 | A ranking found under a pyramidal (box) friction cone is a ranking | A friction cone linearized as a box grants √2·μ along its diagonals, 41% more traction. A planted policy that drives diagonally flips a planted ranking; the result is typed `model-conditional`, and an honest policy on the circular cone is not flagged | Rank robustness of finalists; FROB-REF-830 | planted-design; a finalist whose advantage is diagonal traction; flagged `law-sensitive`, ranking `model-conditional` | GATE-CORTEX-04, GATE-CORTEX-15 |

**The late cases (no number yet).** They are appended to the catalogue, which is append-only. A bead never
invents a number above the plan-time maximum (CX-384): the numbers come from the allocation ledger
`crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`, N38a-40). This bead files
one `[[block]]` request there with `registry = "cx"`, `count = 5`, `owner = "N23-CX"` and
`section = "§23.25 to §23.29"`; the allocator fills `first` and `last`; the five tags `CX-23-L1` … `CX-23-L5`
are then mapped to the block's numbers in table order in one commit, and each entry keeps its tag in `slug`.
Until the block is issued an entry is cited by its tag; a tag still used as an identifier in a registry, a test
name or a derivation file at this bead's milestone (M2) fails the ledger check `provisional-tag-past-due`.

| Provisional | The tempting shortcut | The smallest case that breaks it | The rule it protects | Control | Gates |
|---|---|---|---|---|---|
| CX-23-L1 | A sampler with correct marginals samples the law the likelihood assumes | A Box–Muller cosine whose phase uniform is a linear function of a counter advanced by a constant: consecutive draws differ in phase by the fixed fraction 0.167245 of a turn, so two adjacent channels agree in sign with probability 1 − 2·0.167245 = 0.665510 where independence gives 0.5; counts of 66,552, 66,552 and 66,551 in 100,000 two-channel calls on three seeds; each channel alone is positive half the time | The joint law of sampled actions (P-12) | planted-design; the linear-phase sampler re-implemented, which is the one fixture `frob_conformance::plants::linear_phase_sampler` of N33c-37 (this entry's `plant`; that bead builds the fixture and registers no second control for it); adjacent-channel agreement 0.6655 ± 0.005, more than 100 standard deviations from ½ at N = 10⁵ | GATE-CORTEX-18, GATE-WS-06 (sampler row), GATE-WS-07 (row `m2-sampler`) |
| CX-23-L2 | A reward handed to the actor is not an observation | Three one-second ticks, hidden gain g ∈ {1, 2}, acceleration g·u, first command u₀ = ¼: x₁ = g/8, so the shaped reward −abs(1 − x₁) is −7/8 under g = 1 and −3/4 under g = 2. With the velocity packet late no causal policy docks in both worlds; with that reward delivered before the second command the commands (¼, ½, −¾) and (¼, 0, −¼) dock both | The actor's information contract lists every readable field (E-10) | planted-input (four of them, the entry's four plants `leak.cortex.1` to `leak.cortex.4` of N23b-32, one control each): previous reward wired to the actor; an `info` field carrying a hidden parameter; an action mask computed from hidden state; a termination flag delivered early. Each fails the causal test in 100 of 100 seeds | GATE-CORTEX-19 |
| CX-23-L3 | The first evaluation look that clears the target is the time of attainment | True success rate 0.89, target 0.90, 30 episodes per look. 30 of 30 has an exact one-sided 95% lower bound of 0.05^(1/30) = 0.90497. One look does this with probability 0.89³⁰ = 0.0303; over 50 looks, 1 − (1 − 0.0303)⁵⁰ = 0.78548 | A threshold crossing is not attainment | a-a; the attainment procedure replaced by the unadjusted first-crossing rule; reports attainment in about 78.5% of 2,000 simulated trainings of a policy that never attains | GATE-CORTEX-20, GATE-CORTEX-05 |
| CX-23-L4 | A rung with Kendall τ above 0.7 near the top keeps the winner | Forty designs; the cheap rung reproduces the true order except that the true best is ranked third. Among the true top ten, 2 of 45 pairs are inverted: τ = (43 − 2)/45 = 41/45 ≈ 0.911. A cut keeping the top 5% (two designs) discards the winner | The winner check: containment rank and regret | planted-design; the forty-design fixture with a request to select k = 2; must be refused, and accepted at k = 3 | GATE-CORTEX-21 |
| CX-23-L5 | A gradient through a long differentiable rollout tells the optimizer where to go | At a closed-loop exponent of 0.16 per control step a perturbation grows by e^(0.16·K): 13 at K = 16, 167 at K = 32, 7.8 × 10⁸ at K = 128. A gradient through 128 steps is dominated by its variance | Short windows, a cosine monitor and step labels in short-horizon gradient training | planted-input; a run with the window forced to K = 128 and the monitor off; the median cosine against paired central differences falls below 0.5 and the run must switch to the stencil or stop | GATE-CORTEX-22 |

**Who builds each plant (DESIGN).** `plant` is written `<building bead>:cx-<nnn>` (or `<building bead>:<tag>`, or
the fixture path the row names), the bead that owns the component and cites the entry: CX-246 N23a-07 (the
single-flag recursion; BD-197 reproduces it in the PPO battery); CX-247 BD-192 (the per-step penalty, registered
against GATE-CORTEX-06); CX-248 N23a-07 (the per-step γ); CX-249 N23a-08; CX-250, CX-251 and CX-252 BD-197 (the
three planted defects that reproduce the sibling's errors); CX-253 and CX-254 N23a-19; CX-255 BD-193 (N23b-12 for
the adapter's views); CX-256 N23a-05; CX-257 BD-198; CX-258 N23b-08 (GATE-CORTEX-15), with the planted-exploit
corpus N23b-07 for GATE-CORTEX-04; CX-23-L1 N33c-37; CX-23-L2 N23b-32; CX-23-L3 N23b-34; CX-23-L4 N23b-35; CX-23-L5
N23b-39 (the cosine monitor and step labels of GATE-CORTEX-22).

**FACT entries.** CX-250, CX-251, CX-252 and CX-23-L1 carry `label = "FACT"` with their source (the sibling's
training code, or its sampler, at a pinned commit) and `rerun = false` until the bead that reproduces them here
records a receipt: BD-197 for CX-250 to CX-252, N33c-37 for CX-23-L1. Until then the seeding tool writes their
controls as `obligation` (not runnable; no gate made eligible).

**What the seeding tool writes from an entry (the schema of the negative-control registry, owned by BD-025).**
One `[[control]]` record per control: `id` (`NC-` plus four digits, stable, append-only, never reused, assigned by the registry
owner), `seed` (the CX it came from; for a late case its issued number), `gate` (every gate that must catch it; each
must exist in `registries/gates.toml`), `kind` (`mutation`, `planted-input`, `planted-design`, `planted-world` or
`a-a`), `target` (the code path, verb, fixture or procedure the defect is planted in), `defect`, `expect` (an
observable: a value, a refusal code, a failed row), `last_seen = { caught, session, date }` (written by the
control runner, never by hand), and the optional `feature` (for a `mutation`) or `fixture` (for the other
kinds). Rules that bind this bead: a gate with no control cannot record PASS; a control is run every time its
gate is run; a control is a defect and not a boundary (the control for a refusal is the build or input in which
the refusal's check was removed, recorded in `removed_check`).

**Mutation points (DESIGN, following BD-025).** Each `mutation` control is a Cargo feature of the owning crate
(named by its plant, as N38b-05's seeding writes it and BD-025 records it: `feature = "cx-<nnn>"`, or the tag,
for the plant `<building bead>:cx-<nnn>`; declared in that crate's manifest by the building bead), enabled only under the conformance configuration's
`gauntlet-mutations` umbrella, which does not compile under the release profile, so no planted defect can reach a
release binary. `planted-input` and `planted-design` controls are fixtures under
`crates/frob-conformance/fixtures/cortex/` with a `synthetic` kind header and a recorded hash.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/counterexamples.toml` holds the thirteen entries CX-246 to CX-258 and the five late entries, each with `control_kind`, a `plant` and its gates among `rule_ids`. After the seeding tool of N38b-05 is re-run, each entry has at least one control in `controls.toml` whose `seed` names it (the second late entry has four), each naming its gates; the tool's second run changes no byte and the join passes. No record of `controls.toml` is written by this bead.
- [ ] Every entry's `plant` names the bead that builds it (the list above); the four FACT entries carry `rerun = false` and seed as obligations until their reproducing bead records a receipt.
- [ ] `cx_values_recomputed` reproduces every expected observation from first principles (for example 0.5 for CX-253, element 12 for CX-255, 1.582459 for CX-252).
- [ ] The five late entries carry numbers from a block issued to N23-CX in `allocations.toml`, in table order, with their tags in `slug`; `registry::alloc::validate` passes, no tag remains as an identifier at M2, and the registry test fails if any CX of this chapter has no control.
- [ ] `scripts/e2e/cortex_controls.sh` exits 0.

## Test plan

### Unit tests

`cargo test -p frob-conformance cortex_controls`:
- `cx_cortex_all_present`: the catalogue holds CX-246 … CX-258 (13) and the five late entries (by tag before the block is issued, by number and `slug` after), and after seeding the control registry holds at least one control per entry.
- `cx_cortex_block_request_of_five`: the ledger holds the request; after issue the five numbers are contiguous and in table order.
- `cx_values_recomputed`: recomputes from first principles and asserts 9.9 (CX-246); 500 and 1,000 (CX-247); 2.000·r, 1.995·r, 1.990·r and the per-step 2.0·r, 1.0·r (CX-248, to four significant digits); 3.15149 and 1.64872 (CX-249); 0.25 and 0.5 (CX-253); 4/3 and 32/9 as exact rationals (CX-254); elements 3 and 12 and the order (1, 0, 3, 2) (CX-255); margins 0.5 and 1.2 (CX-256); 0.665510 from the two constants 0x9E3779B97F4A7C15 and 0x94D049BB133111EB (CX-23-L1); −7/8 and −3/4 (L2); 0.90497, 0.0303, 0.78548 (L3); 41/45 (L4); 12.9, 167.3, 7.84 × 10⁸ (L5).
- `cx_cortex_plants_and_labels`: every `plant` names its building bead; CX-250, CX-251, CX-252 and CX-23-L1 are FACT with `rerun = false`; every other entry is DESIGN.
- `control_is_defect_not_boundary`: each record's `kind` is one of the five kinds and its `defect` names a removed or altered rule.
- `gate_without_control_cannot_pass`: a synthetic gate row with an empty `controls` list is refused PASS.

### End-to-end script and logging

`scripts/e2e/cortex_controls.sh` (sources the shared helper of `STD-E2E`; newline-delimited JSON to
`target/e2e-logs/cortex_controls/<utc>.ndjson` with a transcript beside it; it calls only cargo targets of its
dependencies, so no `frob` command). It runs the unit tests above; re-runs N38b-05's seeding tool, `cargo test -p
frob-conformance --test seed_controls -- --ignored seed`, twice, asserting that the second run leaves the digest
of the committed `controls.toml` unchanged; runs the join, `cargo test -p frob-conformance --test seed_join`; and
reads the control runner's history `registry/control_history.ndjson` (N38b-04) to report each control's latest
outcome. Running a gate's conformance entries with the plant off and with `--features gauntlet-mutations` is the
control runner's, invoked by the building beads' own scripts, and is not repeated here. Each line carries `ts`,
`script`, `bead`, `step`, `event`, the exact command, exit code, wall time, the control identity, its building
bead, its state (`runnable`, `obligation`, `stale` while the plant is unbuilt) and latest outcome; the digest of
`controls.toml` before and after the second seeding; on failure the first missing entry or unresolved gate.
Controls whose plant is not yet built are logged `stale: capability absent`, never as caught. The `summary` line
gives controls caught, missed, never run and stale, and the gates touched.

### Known answers and negative controls

- Known answers behind the cases: KA-576 (CX-246), KA-578 (CX-247), KA-579 (CX-248), KA-582 (CX-249), KA-583 (CX-250), KA-584 (CX-251), KA-585 (CX-252), KA-587 (CX-253), KA-588 (CX-254), KA-580 (CX-255), KA-581 (CX-256), KA-590 (CX-257), KA-591 (CX-258).
- Control of the controls: a planted registry record whose `expect` cannot occur (CX-253 with expected 0.75) must be reported as "never caught" by the review query.
- Determinism: the script is run twice on one machine and across worker counts; the two logs agree on every asserted field (the fixtures are exact or class D2).

## Considerations

- CX-256 is stated with a thermal margin in kelvin: a temperature margin is a difference, so its scale is written in kelvin even where a source text writes degrees Celsius.
- The 1.582459 of CX-252 and the finite differences of CX-250 are FACTs read from a sibling's training code at a pinned commit and re-run at admission; the fixture reproduces them from the fixture's own definition, not from that code, and the sibling is never linked.
- Owed by other beads, not by this registry: every gate named in the tables has at least one of these controls in its `controls` list in `registries/gates.toml` (the Cortex rows, N23b-16); each `mutation` feature is declared in the owning crate's manifest, reachable only through `gauntlet-mutations`, and does not compile under the release profile (the building beads); each control's expected observation is reproduced by a test that plants the defect (the building beads); `last_seen.caught` is true for every control whose gate has run, and a control never caught is listed for the milestone exit review (N38b-04).
- The five late cases have no catalogue number until their block is issued. Until then gates and beads cite the tags; when the numbers arrive the tags move to `slug` in one commit and nothing else changes.
- CX-258 is shared with the fidelity chapter's model-form check and the optimization chapter's finalist stage; one control record lists both gates of its row (GATE-CORTEX-04, GATE-CORTEX-15), so the planted finalist is one fixture.

## References

Appendix N section N.20 (CX-246 to CX-258); §23.3, §23.4, §23.5, §23.6, §23.9, §23.16, §23.21, §23.25 to §23.29; §38.11.
