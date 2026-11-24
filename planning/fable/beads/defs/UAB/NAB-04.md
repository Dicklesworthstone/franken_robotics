---
key: "NAB-04"
title: "registries/glossary.toml: term records D to F (D0 to D4 to the five binaries), 59 rows"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "workspace", "delivery", "solid", "registry"]
parent: "WP-01"
deps: ["NAB-01"]
needs: []
related: ["NAB-05", "NAB-06", "NAB-07"]
gates: []
plan_refs: ["Appendix A"]
---
## Background

The glossary of FrankenRobotics is a registry, `registries/glossary.toml`, from which `docs/GLOSSARY.md` is
generated (NAB-01) and against which every `CONTRACT.md` is linted (NAB-02). It is filled in five beads by
letter range. This bead carries every term of art whose sort key begins with D, E or F: 59 rows. Backticks are
ignored by the sort key, so the code literals `Degraded`, `energy-created` and `frob` belong here.

A row gives the term, what it means in this project, and in parentheses the chapter subsection, construction
(`C-nn`), doctrine rule (`D-n`) or other identifier that owns the definition. The owning text governs where a
row abbreviates. Numbers quoted in a row are DESIGN values of the owning section unless the row says otherwise.
Tier: Solid.

## Scope

In: the 59 `[[term]]` records of this range. Out: the schema, loader and generator (NAB-01); the lint (NAB-02);
the other ranges (NAB-03, NAB-05 to NAB-07).

## Technical approach

**Transcription rule.** One `[[term]]` record per row, in table order. `term` is the first column without
backticks (`code = true` if it was printed in backticks); a trailing parenthetical in the term column is the
`sense`; `meaning` is the second column up to its final parenthetical, which is split into `defined_in` (items
beginning `§` or `Appendix`) and `constructs` (items such as `C-25`, `D-11`); `kind` is `subsystem`, `label`,
`binary`, `class`, `record`, `quantity` or `concept` by the rule of NAB-03; `aliases` hold plurals and the
further names of a multi-name row; `design_values = true` when the meaning contains a number; `id` is the
lower-cased term with hyphens, plus `-<sense>`.

**Special cases in this range.**

- Senses: `Envelope` has two records: `fidelity` (the frozen tolerance within which a faster tier must agree
  with a slower one) and `output` (the wrapper `frob.envelope/1` of every verb's output). A contract that says
  "envelope" without a qualifier fails the lint outside the two owning crates. The compute envelope (classes E0
  to E3) is a separate term in the A to C range.
- Multi-name rows: `Degraded`, `Faulted` (two world states; alias `Faulted`); the row of five binaries `frob`,
  `frobd`, `frob-mcp`, `frob-verify`, `frob-tui` (one record of kind `binary`, the other four names as aliases).
- Code literals (`code = true`, kind `label`): `Degraded`, `Faulted`, `energy-created`. The binaries row is also
  `code = true` (kind `binary`), so this range holds three of the eleven code-literal records of NAB-01's
  known answers.
- Classes: `D0 to D4` (the five determinism classes: exact; bitwise across backends; bitwise per backend and
  independent of the batch; tolerance with decision agreement; unspecified), `Evidence class` (Proved,
  Certified, Tested, Estimated, strongest to weakest), `Determinism profile` (`reference-replay`,
  `device-replay`, `cross-backend`, and the labelled class-D4 `fast-training`), `Derivative-validity label` (nine
  labels: `smooth`, `saltation`, `discrete-step`, `fixed-mode`, `one-sided`, `generalized`, `smoothed`,
  `finite-difference`, `refused`), `Fork classification` (five: continuation, exogenous, contact-affecting,
  information, structural).
- Subsystem row: `Foundry`.
- Formulas and numbers to transcribe exactly: decimation d = control period ÷ physics step h, and an env-step is
  d world-steps; defensive mixture q = β·p + (1 − β)·q′ with weights at most 1/β; the docking boundary
  ε < (U + ½)/4, equality excluded; the docking fixture: a unit mass, hidden gain g ∈ {1, 2}, three ticks, one
  noisy velocity packet; the doubly robust correction Z = m + A·(Y − m)/p; the dual witness Mᵀy ≥ 0 and rᵀy < 0;
  the e-process Eₙ = ∏ (1 + λᵢ·d̃ᵢ); `energy-created`: created energy above 1% of the absolute work exchanged in
  any 0.5 s window on a fast tier, or 10⁻⁶ relative on the referee; the design corpus of about 200 designs; the
  eleven decision cores DC-01 to DC-11; the thirteen differentiators N1 to N13; the doctrine rules D-1 to D-19;
  the Five Explicits (units, seeds, budgets, versions, capabilities); the fleet (the M5 Ultra holds the world
  bank, proofs, replay and large-memory jobs; the two RTX 4090 train; only seeds, weights and receipts cross).

**The rows.**

| Term | Meaning, with the section or construction that defines it |
|---|---|
| D0 to D4 | Determinism classes: exact; bitwise across backends; bitwise per backend and independent of the batch; tolerance with decision agreement; unspecified (§35.1) |
| Decimation | d, the control period divided by the physics step h; an env-step is d world-steps (§36.2) |
| Decision core | One of eleven recurring decisions (DC-01 to DC-11), each with a compiled table, a loss model, proof obligations, a deterministic fallback and a budgeted mode (§28.19) |
| Decision-level equivalence | The test under class D3 that task decisions agree between two runs, not only that state errors are small (§35.4) |
| Defensive mixture | A proposal q = β·p + (1 − β)·q′ whose importance weights are at most 1/β (C-27, §26.9) |
| `Degraded`, `Faulted` | World states. `Degraded`: a world that exceeded a contact or row capacity or did not converge, marked with its reason and never silent. `Faulted`: a world whose transaction failed every numerical plan, with cause `numerical-fault` kept apart from physical failure (§16.6, §16.14) |
| Dependency firewall | The checks that keep sibling and third-party dependencies out of crates that may not have them (§33.7) |
| Derivative-validity label | One of nine labels a derivative carries: `smooth`, `saltation`, `discrete-step`, `fixed-mode`, `one-sided`, `generalized`, `smoothed`, `finite-difference`, `refused` (C-16, D-15) |
| Design | A typed term in the design language, stored as `.fbot` with schema `frob.design/1` (§14.3, §14.4) |
| Design corpus | About 200 designs (templates, parameter perturbations, designs with planted defects, planted winners and planted causes) with labels fixed before any analysis is tuned (§24.9) |
| Design family | A set of designs over typed holes and parameter charts on which a mission query or a family batch ranges (§25.3, §17) |
| Design-family bucket | A packing of a family's worlds in which constants shared by its members are stored once; the device form of a family lane (§21.11) |
| Determinism profile | A named set of pins a run declares: `reference-replay`, `device-replay`, `cross-backend`, or the labelled class-D4 `fast-training` (§35.2) |
| Device-second | One second of one device's wall-clock occupancy, including compilation, admission, transfer and synchronization (§36.2) |
| Differentiator | One of the thirteen statements N1 to N13 of what the project adds, worded as the audit permits (§1.6) |
| Disclosure block | The fields every Performance claim carries: hardware, versions, world count, step and decimation, solver settings, contacts, sensors, class, timings and memory (§36.7) |
| Discovery lane | Everything an agent may read while it designs: training scenes, development splits, batteries, the hard-world set, its own runs (§24.13) |
| Divergence ledger | Recorded CPU-versus-device differences per kernel (§21.13) |
| Docking boundary | The closed-form condition ε < (U + ½)/4 of the docking fixture, equality excluded (C-19, §20.16) |
| Docking fixture | A unit mass with hidden gain g ∈ {1, 2}, three ticks and one noisy velocity packet; the known answer for information timing (C-19, §20.16) |
| Doctrine rule | One of D-1 to D-19, the rules every construction obeys (§9) |
| Doubly robust tier correction | The estimator Z = m + A·(Y − m)/p that stays unbiased whatever the cheap prediction m is (C-25, §29.11) |
| Dual witness | A vector y that proves infeasibility of a relaxation: Mᵀy ≥ 0 and rᵀy < 0 (C-32) |
| E-process | A nonnegative supermartingale under the null, here the betting product Eₙ = ∏ (1 + λᵢ·d̃ᵢ) (C-15) |
| Edit | A typed change to a design that declares its correspondences and returns an invalidation set (§14.7) |
| Edit-to-first-step latency | The wall time from an accepted `design.edit` to the first batched step of the new design (§17.8) |
| Energy budget | The `.fenergy` inequality that bounds every source of energy that can reach an absorber by the absorber's minimum work (C-31, §22.11) |
| Energy ledger | The per-episode attribution of every joule to a store, a loss or an export, with the numerical residual on its own line (§29.9) |
| `energy-created` | The flag of an episode in which created energy exceeded 1% of the absolute work exchanged in any 0.5 s window on a fast tier, or 10⁻⁶ relative on the referee (§29.9; applied by §16.10) |
| Env-step | One control step of one environment: d world-steps (§16.1, §36.2) |
| Envelope (fidelity) | The frozen tolerance within which a faster tier must agree with a slower one for a result to be promoted (§16.8) |
| Envelope (output) | The common wrapper `frob.envelope/1` of every verb's output, with claims, reality status, budget and refusal fields (§32.4) |
| Established | The label of a result re-evaluated on a higher tier and found inside its envelope; names its adjudicator (§16.8) |
| Estimated | The evidence class of a number from a model or a statistical procedure, with its uncertainty (§12.2) |
| Evidence class | Proved, Certified, Tested or Estimated, ordered strongest to weakest (§12.2) |
| Exact-integer kernel class | Kernels in integer arithmetic for strict predicates, digests and histograms; class D0 on every backend (§21.8) |
| Exclusion | A cell removed from the outer set with a refutation of matching scope (C-17) |
| Explicit reset | The default gym semantics: an ended world keeps its final observation until the caller resets it (§23.3) |
| Exploitation detector | A re-evaluation under a perturbation that should not matter, which flags a policy that depends on it (§23.16) |
| Facility co-design | Designing the robot, the building amendments and the handoff between robots in one query [M] (§25.17) |
| Family certificate | `Certified` evidence about a set of designs: a cell of the inner set or a cell removed from the outer set (§12.2) |
| Family lane | Worlds that share a tree, a sensor graph and, for a stretch of time, a contact mode, so that structure is constant across them (§17.6) |
| Family policy | One set of weights that acts on the tokens of any design of one morphology class (C-23, §23.11) |
| Feasibility witness | A face-connected path of inner cells from start to goal in a clearance complex (§19.11) |
| Fidelity ledger | Recorded differences between a faster tier and a slower one, per stratum (§16.8) |
| Film-friction law | The contact-level law in which friction on a wetted patch depends on dwell time, load and pad size (§18.8) |
| Firmware export | Export of a fixed-point controller with its Q formats and test vectors against caller-supplied interfaces [M] (§31.13) |
| First-divergence search | The search over the chained digests of two runs for the first tick and stage at which they differ (§27.10) |
| Five Explicits | Units, seeds, budgets, versions and capabilities are never implicit at the agent surface (§32.2) |
| Fixed-n exact study | A study with an exact interval or test at a sample size fixed in advance, admitted only with a reachability pre-analysis (§24.12) |
| Fixture | A small registered task with a closed-form or enumerated answer, for example `fixture:cart-sill-dock` (§24.10, §24.11) |
| Fleet | The two machines under one scheduler: the M5 Ultra holds the world bank, proofs, replay and large-memory jobs, the two RTX 4090 train, and only seeds, weights and receipts cross (§21.17) |
| Flip rate | ρ₀, the fraction of placebo forks whose verdict differs from the parent's (§27.5) |
| Floquet multiplier | An eigenvalue of the derivative of the return map of a periodic gait (C-16) |
| Fork | A child run started from a snapshot, with its own world address and fresh keyed streams (§27.3, §29.8) |
| Fork classification | The class the service assigns a requested change before restoring state: continuation, exogenous, contact-affecting, information, or structural (§27.4) |
| Foundation | A sibling project with an `FD-*` row: posture, pin, admission gate and fallback (§33.4) |
| Foundry | The subsystem that owns structure, manufacture and lifecycle (§31) |
| `frob`, `frobd`, `frob-mcp`, `frob-verify`, `frob-tui` | The CLI, the daemon, the MCP server, the offline certificate auditor and the terminal dashboard (§32, §12.4) |

Rows in this range: 59.

## Acceptance criteria

- [ ] The registry contains 59 records whose sort key begins with D, E or F, in the order above, and loads.
- [ ] `Envelope` has two records with senses `fidelity` and `output`.
- [ ] `lookup("Faulted")`, `lookup("frobd")` and `lookup("frob-verify")` each resolve to one record.
- [ ] The nine derivative-validity labels and the four determinism profiles are present verbatim in their rows.
- [ ] The generated document prints the 59 meanings character for character as in the table.

## Test plan

### Unit tests

`crates/frob-conformance/tests/glossary_content.rs`, module `d_to_f`:

- `count_is_59`; `order_matches_frozen_list`.
- `envelope_has_two_senses`; `binaries_row_has_four_aliases`.
- `pinned_values`: `Docking boundary` contains "ε < (U + ½)/4"; `Docking fixture` contains "g ∈ {1, 2}";
  `Doubly robust tier correction` contains "Z = m + A·(Y − m)/p"; `Defensive mixture` contains "1/β";
  `energy-created` contains "1%" and "0.5 s" and "10⁻⁶"; `Decision core` contains "DC-01 to DC-11";
  `Differentiator` contains "N1 to N13"; `Doctrine rule` contains "D-1 to D-19".
- `derivative_validity_has_nine_labels` (splits the row and counts nine code spans).
- `meanings_match_frozen_table` against `tests/fixtures/glossary/d_to_f.tsv`.

### End-to-end script and logging

`scripts/e2e/glossary.sh` (NAB-08) asserts the count of this range (59) and logs it as a `metric` per STD-E2E.

### Known answers and negative controls

Known answers: 59 rows (23 D, 16 E, 20 F). Negative controls: giving both `Envelope` records the sense
`output` must fail `gloss.duplicate-term-sense`; removing `refused` from the derivative-validity row must fail
the nine-label test; writing the docking boundary with "≤" must fail `pinned_values` (equality is excluded).

## Considerations

- The determinism classes and evidence classes appear in every output envelope; their glossary rows must stay
  word-for-word consistent with the enumerations in `frob-core`. When `frob-core`'s contract lands it becomes
  the authority (`owner_crate = "frob-core"`).
- `Differentiator` wordings are owned by the claims-wording registry (N00-15); the glossary row only says what a
  differentiator is.

## References

Appendix A, rows D to F.
