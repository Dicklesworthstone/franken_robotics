---
key: "NFx-02"
title: "upstream frankensim (conditional): an external differentiable stage in `fs-topopt`'s design map; airlock probe"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "suite", "foundry", "upstream", "frankensim", "frob-fsim", "frontier"]
parent: "WP-71"
deps: ["BD-403", "N31a-29", "N33b-14", "NFx-01", "STD-E2E"]
needs: []
related: []
gates: []
plan_refs: ["§31.10", "§31.4", "§31.7", "§33.8", "§33.9", "Appendix F", "Appendix F.3"]
---
## Background

Foundry, the structure and manufacture subsystem (crate `frob-foundry`, milestone M5, tier Frontier), optimizes
the topology of a link inside a FrankenSim job reached through the airlock crate `frob-fsim` (feature group
`structure`, trait `StructuralOpt`). The rule for manufacturability is: inside the optimization only constraints
with usable derivatives are active, namely length scale, envelope, symmetry and, for additive parts, an
**overhang filter**; everything else is checked after extraction, and a violation triggers a re-run with a
tightened length scale. An overhang filter maps a density field to the field that could actually be printed
layer by layer (a cell cannot be denser than what supports it), and it has an exact derivative by one reverse
sweep. Foundry owns that filter (bead N31a-29). To act *inside* the optimization it has to sit in the sibling's
design map, between the length-scale filter and the projection.

FACT (read 2026-11-22 at FrankenSim head `95a7fba8e8a7`; re-read at the pin before anything cites it): in
`frankensim/crates/fs-topopt/src/pipeline.rs` the design map is `DesignPipeline { filter: DensityFilter,
params: SimpParams }`, a struct with public fields. `try_forward` computes ρ̃ = F·ρ (a Helmholtz filter),
ρ̄ = H(ρ̃) (Heaviside projection with parameters β, η) and the moduli E = e_min + (1 − e_min)·ρ̄ᵖ; `try_pullback`
maps dc/dE back through SIMP′, H′ and Fᵀ. The chain is fixed: there is no slot for a further stage. The
three-dimensional cut-cell lane (`sdf3`, behind the sibling feature `cutfem-marquee`) applies the same
projection functions in its own design map (`sdf3.rs`, `sdf3/design.rs`); whether it has a slot is part of the
re-read. The sibling states "no process constraints beyond length scale".

So at the pin the filter cannot run in the job, and the plan's "for additive parts an overhang filter" inside
the optimization is unmet. No upstream work item covered this and no probe reported it. Every sibling is the
owner's project, so the missing slot is work this program does in FrankenSim's repository, as an upstream work
item with an interim posture and a switch-over rule; no milestone waits on it. Register row: `FD-FSIM-41`,
priority P3 (written by NFx-01).

Rejected: copying the filter into FrankenSim as a process-specific feature (the sibling keeps process knowledge
out of its optimizer, and the evaluators for overhang are a separate upstream item, FD-FSIM-29); wrapping the
optimizer's outer loop here and re-implementing its update (a fork of the optimizer by another name); a local
patch of the sibling (nothing in this repository patches a sibling).

## Scope

In: the capability probe through the airlock; the description of the upstream change and its tests in the
sibling; the mirrored bead; the stage-2 fixture; the interim posture; the switch-over and its script.
Out: the overhang filter, its derivative and the post-extraction re-run loop (N31a-29); the topology rung
(N31a-15); the overhang evaluator (N31a-28); the `structure` group adapter and INT-FSIM-15 (N33b-14); the
register row (NFx-01); geometric evaluators upstream (FD-FSIM-29).

## Technical approach

**1. The probe (here, first).** `StructuralOpt` gains a capability report (DESIGN, module
`frob_fsim::structure`):

```rust
pub struct StructuralCapabilities { pub external_design_stage: Capability /* further rows as needed */ }
pub enum Capability { Present, Absent { reason: &'static str } }
fn capabilities(&self) -> StructuralCapabilities;
```

The adapter is written against the pinned API, so the value is a fact of the pin: `Absent { reason: "no stage
slot in the sibling's design map" }` today. In the `remote` build mode of the airlock (the worker process) the
value is `Absent { reason: "remote mode: a stage is code and does not cross the job protocol" }` whatever the
pin has. `frob capabilities --json` prints the row `structure.external_design_stage`. Foundry reads it to decide
where its filter runs. If the M5 pin already has a slot, this item is closed as not needed with the probe
record, and only step 5 is done.

**2. The upstream change (in `frankensim`, crate `fs-topopt`, layer L4, under FrankenSim's `AGENTS.md`).**
Additive: `DesignPipeline` keeps its two public fields, because it is built by struct literal and a third field
would break existing callers. Beside it (DESIGN; the final shape is decided under the sibling's rules):

```rust
pub trait DesignStage {
    fn apply(&self, filtered: &[f64], control: &mut SolveControl<'_>) -> Result<Vec<f64>, EvaluationStop>;
    /// Transposed-Jacobian action at `filtered`: cotangent on the stage's output → cotangent on its input.
    fn apply_transpose(&self, filtered: &[f64], cotangent: &[f64], control: &mut SolveControl<'_>)
        -> Result<Vec<f64>, EvaluationStop>;
    fn identity(&self) -> StageIdentity;      // name, version, parameter digest; enters the run's evidence
}
pub struct StagedDesignPipeline<'a> { pub base: &'a DesignPipeline, pub stage: &'a dyn DesignStage }
```

- Forward: ρ̃ = F·ρ, ρ̂ = S(ρ̃), ρ̄ = H(ρ̂), E = e_min + (1 − e_min)·ρ̄ᵖ.
- Pullback: dc/dρ = Fᵀ·J_S(ρ̃)ᵀ·(H′(ρ̂) ⊙ (1 − e_min)·p·ρ̄ᵖ⁻¹ ⊙ dc/dE).
- Contract of a stage: output of the same length, finite, in [0, 1]; a pure function of its input; the
  transpose consistent with `apply`. A violation is `EvaluationStop::Breakdown { stage: "external-stage" }`.
- The drivers that take a `DesignPipeline` (continuation, the optimality-criteria update, the volume and mass
  gradients, the three-dimensional lane's design map) accept the staged form through the same entry points or
  through twins beside them; volume and mass are computed on ρ̄ after the stage.
- Self-description: a result produced with a stage records `StageIdentity` in its evidence, and the sibling's
  own `gradient_check` is run on the composed map before the first iteration; a stage that fails it is refused.
- With no stage the present functions are untouched, so every existing result is bit-identical.

Rules that bind the work: FrankenSim's layer checks, contracts (`CONTRACT.md` of `fs-topopt` gains the stage
contract), unsafe-capsule discipline (no unsafe is needed), no branches and no worktrees; the files are reserved
through Agent Mail before editing, because other agents work in the same repository; the change lands on the
sibling's `main` through its own gates. The bead is mirrored in FrankenSim's tracker, and the mirror's
identifier is stored in the register row.

**3. Interim posture here.** The optimization runs without the filter. The result is labelled `overhang:
post-check only`, overhang is measured after extraction, and a violation goes to the re-run loop of N31a-29
(best alternative build direction first, then a tightened length scale, at most two re-runs). No gate depends
on the in-job form. Cost of the posture: mass, since a tightened length scale is a blunt substitute; the
mass-cost report of N31a-29 makes it visible.

**4. Fixture** (`tests/upstream/NFx-02/`, `frob-conformance` test `upstream_nfx_02`, `#[ignore]`d). Stage 1 is
written by NFx-01 (source-level absence). Stage 2, written here once the `structure` group is admitted: assert
`capabilities().external_design_stage == Present`, then that the identity stage reproduces the unstaged
forward and pullback bit for bit. At the pin the first assertion fails.

**5. Switch-over (the compatibility train).** When the change is on FrankenSim's `main`: move the pin to a
clean commit that contains it, write a new resolution receipt, re-run INT-FSIM-15 and the group's parity
gates, relock. Then wire the adapter (`StructuralOpt` accepts a stage in `inproc` mode), flip the probe, and
re-run the shelf fixture of N31a-29 for `slm` with the filter in the job: violating overhang area zero or
within the bridge allowance, checked by the evaluator rows of GATE-FOUNDRY-07; record the mass against the
post-check result. The row's `status` becomes `adopted` only after that.

Determinism: the stage is a fixed-order `f64` sweep; class as the sibling job's. Evidence class of an optimized
part is unchanged (Estimated).

## Acceptance criteria

- [ ] At the M5 pin the probe reports `Absent` with its reason in `inproc` mode and in `remote` mode; `frob capabilities --json` shows the row; the facts of the Background are re-read at the pin and recorded with file and line.
- [ ] In the interim posture `frob foundry optimize` for an additive process returns `overhang: post-check only` and never claims an in-job filter.
- [ ] The mirrored bead exists in FrankenSim's tracker and its identifier is in the row `FD-FSIM-41`; the stage-2 fixture fails at the pin with `observed_at` recorded.
- [ ] If opened, in FrankenSim: with no stage every existing test of `fs-topopt` passes with no expectation edited; the identity stage is bit-identical to no stage; the scaling stage matches the hand case; a planted inconsistent transpose is refused by the gradient check; an out-of-range or non-finite output stops with `external-stage`.
- [ ] After switch-over, here: the probe reports `Present` in `inproc` mode; the shelf fixture's in-job result has zero violating overhang area (or within the bridge allowance); the mass of the in-job result and of the post-check result are both recorded; INT-FSIM-15 was re-run on the new receipt.
- [ ] No file of the sibling was edited from this repository's tools, and no patched copy of the sibling stands in for the change.

## Test plan

### Unit tests

In FrankenSim (`fs-topopt`, under its conventions):
- `staged_identity_is_bit_identical`: seeded field of 4,096 cells, forward and pullback equal the unstaged ones bit for bit.
- `staged_scaling_hand_case`: one cell, ρ̃ = 0.8, S(x) = x/2, p = 3, e_min = 0, dc/dE = −10, in the linear-projection limit (β → 0, H(r) = r): ρ̂ = 0.4, E = 0.064, dE/dρ̄ = 0.48, dc/dρ̃ = ½·(−4.8) = −2.4; without the stage E = 0.512 and dc/dρ̃ = −19.2. The test evaluates H and H′ at a small β and compares to 10⁻³.
- `staged_gradient_matches_finite_differences`: a smooth nonlinear stage (S(x) = x²) on a 16 × 16 × 16 field, central differences to 10⁻⁵ relative.
- `inconsistent_transpose_refused`; `out_of_range_stage_output_stops`; `stage_identity_in_evidence`.
Here (`cargo test -p frob-fsim --features structure structure::capabilities`):
- `external_stage_absent_at_pin` (flips to `external_stage_present_at_pin` at switch-over); `remote_mode_reports_absent`.
- `capability_row_printed`: the registry row of `frob capabilities`.
In `frob-foundry`: `fallback_label_when_stage_not_admitted` (N31a-29) reads this probe once the capability row
exists; before that N31a-29 runs on its fail-closed default, the post-check posture, and does not wait for
this bead.

### End-to-end script and logging

`scripts/e2e/upstream_fs_topopt_stage.sh`, two phases, logging through the shared helper to
`target/e2e-logs/upstream_fs_topopt_stage/<utc>.ndjson`. Phase `probe` (every run): `frob capabilities --json`
(asserts the row and its reason), `frob foundry optimize <shelf fixture> --process slm --exploratory --key
"$(foundry_key probe-shelf)" --json` (a `run` verb: the key comes from the cluster preamble, N31a-51; `--process
<card>` per part is in `foundry.optimize`'s row of VERBS-foundry) (asserts `filter_placement =
"post-check"` and the label). Phase `switchover` (only when the register row is `landed`): the same command
under a new key `switchover-shelf` (the probe's key would return the stored post-check envelope) asserts
`filter_placement = "in-job"` and zero violating area. Logged: commands,
exit codes, wall time, the evidence block, the sibling commit before and after, the receipt hash, metrics
`violating_overhang_area_mm2` (threshold: 0 or the bridge allowance), `mass_in_job_kg`, `mass_post_check_kg`,
and a summary line.

### Known answers and negative controls

Known answers: the identity stage (bit equality); the scaling hand case (−2.4 against −19.2); the uniform field
of density 0.5 reproduced by the overhang filter within 10⁻³ (N31a-29's value) so that a job with the filter
on a fully supported part equals the job without it within the optimizer's tolerance. Negative controls: the
planted inconsistent transpose; a planted adapter that reports `Present` at a pin without the slot (the
stage-2 fixture then fails to build the job and the probe test fails); a planted placement that runs the
stage after the projection, caught by a recording stage whose captured input must equal F·ρ bit for bit (at the
β of the continuation's later steps the projected field differs from F·ρ in almost every cell). The uniform-field
case cannot detect a wrong placement: a uniform field is reproduced by the overhang filter at any density, and a
monotone projection nearly commutes with the filter's support minimum, so the two orders give nearly the same
output. Metamorphic: composing the identity
stage with any stage changes nothing. Lanes: bit-identical twice on one machine and across worker counts.

## Considerations

- This item is conditional and P3. If it never lands the program ships on the interim posture; the cost is
  mass on additive parts.
- A stage is consumer code called inside the sibling's solve. It is ordinary Rust on the CPU and is not an
  accelerator kernel, so the sibling's ban on in-process foreign kernels does not apply; the sibling's
  maintainers' reading of that rule is recorded in the mirrored bead before work starts.
- The worker-process route for Foundry jobs cannot carry a stage. A deployment that runs Foundry through the
  worker keeps the post-check posture; this is stated in the capability row.
- RISK-55: `fs-topopt` is under active change by other agents; reserve files, land small.
- OPEN (owner: Foundry lead; default: density lane first): whether the three-dimensional lane gets the slot
  in the same change or in a second one.

## References

§31.10 (constraints inside the optimization), §31.4 (what `fs-topopt` provides), §31.7, §33.8, §33.9,
Appendix F (the rule; F.3); `FD-FSIM-41`; GATE-FOUNDRY-07; KILL-FAB; RISK-55.
