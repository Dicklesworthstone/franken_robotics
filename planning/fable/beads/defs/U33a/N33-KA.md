---
key: "N33-KA"
title: "frob-conformance: known-answer registry of suite consumption (KA-767 to KA-780) with derivation files and controls"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "suite", "gauntlet", "frob-conformance", "solid", "registry", "test", "review-notes-open"]
parent: "WP-37"
deps: ["BD-025", "N38a-12", "N38b-05", "STD-E2E", "STD-TEST"]
needs: []
related: ["N33a-04", "N33a-14", "N33a-17", "N33a-27", "N33a-29", "N33a-30", "N38a-13", "N38b-27", "N38b-34"]
gates: []
plan_refs: ["Appendix M.32", "§33.5", "§33.9", "§33.11", "§33.12", "§33.14", "§38.6", "§38.11"]
---
## Background

FrankenRobotics consumes sibling projects (FrankenSim, asupersync, FrankenSQLite, FrankenTorch, franken_numpy and
others) instead of copying them. Every statement the project makes about a sibling is a fact that can go stale, so
each one a gate leans on is turned into a **known answer**: a small case whose truth is computed by hand or by an
independent formula, with a stated tolerance, the verification tier it runs on, and a **negative control** (a
planted defect the test must catch; the rule is that a gate nothing can fail validates nothing).

This bead is the registry of the fourteen known answers that belong to the suite-consumption chapter. It exists so
that none of them is lost between the beads that implement the mechanisms: each mechanism bead cites its KA numbers
and repeats the values it needs, and this bead is the single list a reviewer can check them against. The registry
is data in `frob-conformance` (the Gauntlet harness crate, layer 7), not prose: a row here becomes a record in the
known-answer registry, and its stated negative control becomes, through the seeding tool of N38b-05, a record in
`crates/frob-conformance/registry/controls.toml`.

Verification tiers used below: G0 is static and algebraic checks on the host; G1 is known answers against closed
forms; G3 is metamorphic and randomized equivalence; G5 is determinism and replay (twice on one machine, across
worker counts, across platforms). "host" means the CPU with no device; "referee" means the CPU referee tier that
runs FrankenSim's own dynamics through the airlock crate `frob-fsim`.

Why the rows carry the reported wrong values too: several rows describe defects reported in sibling code (a
reversed clip branch in a PPO update, a planar-drive step that returns zero translation, a stride convention
trap). The wrong value is what the pinned sibling commit is reported to compute. It is the negative control's
expected observation, and it is FACT to recheck: a reported number stays a report until its admission test records
it at the pinned commit.

## Scope

In: the fourteen rows KA-767 to KA-780 as `[[ka]]` records of the suite's one known-answer file
`crates/frob-conformance/registry/known_answers.toml` (schema and loader: N38a-12), each with its derivation file;
the control text of every row, including the one this bead sets (KA-776), with the kind each seeded record
carries; KA-770's tolerance; the completeness test that joins rows to controls, gates and the beads that run
them; a registry-only end-to-end script. Out: the tests themselves, which live with their mechanisms (keys in the
last column); the `NC-` records, issued by the seeding tool of N38b-05 from the control texts (this bead writes
no control record); the runner entries that bind the rows (the carrying beads, N38a-13); the counterexample
registry of the chapter (`N33-CX`, written by another unit); the refusal registry (`N33-REF`).

## Technical approach

**Record format.** One `[[ka]]` record per row in the suite's one file
`crates/frob-conformance/registry/known_answers.toml`, N38a-12's schema field for field. This is a DESIGN
correction, request S12 of round 2. An earlier draft kept a chapter file
`crates/frob-conformance/registry/known_answers/suite.toml` with keys of its own: `control` as an `NC-`
identifier or `"none"`, `gate` as a list, `runs_in`, and derivations under
`crates/frob-conformance/derivations/suite/`. The loader refuses such a file with `ka-second-file` and refuses
unknown keys. The mapping:
- **Keys kept.** `id`, `case`, `truth` and `tolerance` as before.
- **`section`.** Appendix M.32's "Where stated":
  - §33.5 for KA-767;
  - §33.9 for KA-768 to KA-772;
  - §33.11 rows 1 to 4 for KA-773 to KA-778 (row 1: KA-773, KA-774; row 2: KA-775, KA-776; row 3: KA-777;
    row 4: KA-778);
  - §33.12 for KA-779;
  - §33.14 for KA-780.
- **`owner`.** The first gate of the "Gate" column: the integration gate (`INT-…`) the row admits, or GATE-WS-01
  for KA-767, whose `check-stack` is one of the workspace checks. KA-778's two gates stay in the cell.
- **`fixture`.** One identity per row, named in its derivation file, with `same_as` empty. KA-775 and KA-776 test
  one mechanism with different cases, so they are two fixtures.
- **`tier` and `sites`.** As the "Tier" column gives them: `host`, `referee`, or both for KA-775.
- **`control`.**
  - **Text.** The cell's text, or the text set here for KA-776. There is never an `NC-` identifier typed by hand:
    the seeding tool fills `nc`.
  - **Gates.** A record seeded from a KA row attaches to the row's owner gate and to every gate the cell names
    (N38b-05, step 3). The old `gate` list is therefore the owner plus the gates in the control text.
  - **Expected observation.** The reported wrong value quoted in the cell is the control's expected observation.
    It is restated in the derivation file with the kind of the seeded record (below).
- **`derivation`.** `docs/derivations/<fixture>.md` (rule 1 of N38a-12). The file is a short program or a text
  derivation that reproduces the truth with the standard library alone, and its front matter lists the row, the
  test path (the old `runs_in`) and the carrying bead.
- **`label`.** The label describes the truth.
  - `DESIGN` for every row whose truth is a closed form, a construction or a rule of this design.
  - `FACT` for KA-778, whose truth is what the pinned sibling's contact step does at the pin, with N38a-12's note
    "to be re-run or re-read before a gate cites it".
  - The reported wrong values in the controls of KA-773 to KA-775 and KA-777 are FACT to recheck as well
    (Background). They are marked so in the derivation files, since a control text carries no label.
- **`due`.** The milestone at which every crate the row exercises has been admitted: the latest admitting
  carrier. A consumer that must not inherit a defect, or that uses the admitted group later (N23a-10, N23a-11 at
  M2; BD-333 at M4), does not delay the row.
  - M0 for KA-767, KA-773 to KA-777, KA-779 and KA-780;
  - M1 for KA-771 (N33b-15, BD-111) and KA-778 (whose witness-migration half is BD-093 at M1);
  - M3 for KA-768 (the `spectral` group gate, N33b-11);
  - M3 for KA-769. Its ranking flip is `fs-voi` and its multilevel estimate is `fs-uq`, both in the `stats`
    group of N33b-13. The M1 admission of `fs-eproc` (N33b-12, BD-124) covers neither.
  - M5 for KA-770 (the variance split through the `structure` group, N33b-14, and BD-401; the mesh half,
    N33a-30, is at M1);
  - M6 for KA-772.
- **`printed`.** Empty.

Identifiers are append-only and never renumbered. A row whose tolerance is `to set`, or whose control is `none
stated`, is not counted toward a milestone exit (the loader computes it `obligation: tolerance` or `obligation:
control`). The two such rows below therefore state what this bead sets.

**KA-779 is conditional.** Its carriers N33c-05 and N33c-06 are built only if the `frob-units` escape hatch opens
(INT-TOOLCHAIN-01's decision, BD-031). While the hatch is closed the row has no runner entry, and an ordinary due
row without one is `unbound`, which fails the gate. The plan's intent is a row that is not a pass and does not
block an exit, so the row's derivation front matter carries `conditional_on = "BD-031: escape-hatch"`, the key
N38a-13 reads. While the key is present the runner reports the row on every site as `not-run: conditional
(BD-031: escape-hatch)`, whether or not a function is bound: the row stays in the count of rows due, enters no
other count, and the gate line does not depend on it. The commit that records the hatch as open (BD-031's
decision) removes the key; from that commit KA-779 is an ordinary due row, `unbound` until N33c-05 and N33c-06
bind it.

**The rows.**

| KA | Case | Truth (formula or value) | Tolerance | Tier | Negative control | Gate; carried by |
|---|---|---|---|---|---|---|
| KA-767 | The stack resolution receipt (`frob.stack-receipt/1`) computed twice on an unchanged tree | The two files are byte-identical; so are their BLAKE3 hashes | exact | G5 · host | Touch one tracked file in a sibling copy: `dirty` flips to true, `tree_hash` changes, `check-stack` fails | the stack-receipt rules (GATE-WS-01 through `check-stack`); N33a-04, N33a-05, N33a-27; N33b-04 (the pin-advance train) |
| KA-768 | Floquet multipliers of the monodromy of ẍ + x = 0 over one period T = 2π, through the airlock trait `Spectral` | The monodromy is the 2×2 identity, so both multipliers equal 1 | 10⁻¹² | G1 · host | A monodromy with a multiplier of modulus 1.01 is reported unstable | INT-FSIM-13; N33b-11 (airlock group `spectral` and its group gate), BD-333 |
| KA-769 | Two designs with mean gap Δ and standard deviations s₁, s₂; and a multilevel estimate of E[x²] for x standard normal | Ranking-flip probability Φ(−Δ/√(s₁² + s₂²)), Φ the standard normal distribution function (for Δ = 1, s₁ = s₂ = 1: Φ(−1/√2) = 0.23975006109347673, the value 0.2397500610934767311… rounded to 17 digits); the estimate of E[x²] is 1 inside its own stated interval | 10⁻¹² for the closed form | G1 · host | Importance weights from a proposal that misses the failure region trip the effective-sample-size floor and the estimate is refused | INT-FSIM-14; N33b-12 (fs-eproc at M1), N33b-13 (group `stats`), N33b-21, BD-124 |
| KA-770 | A tolerance allocation on a two-term stack y = a₁·x₁ + a₂·x₂; a mesh written through `fs-io` | First-order variance split σ_y² = a₁²·σ₁² + a₂²·σ₂². Fixture set here: a = (1, 2), σ = (0.03, 0.02) mm gives contributions 0.0009 and 0.0016 mm², σ_y = 0.05 mm, shares 36% and 64%. The mesh reads back with the same content hash | exact for the hash; 10⁻¹² relative for the split (set by this bead; the plan left it `to set`) | G1 · host | A STEP file above the stated size bound is quarantined with a receipt and is not parsed | INT-FSIM-15; N33a-30 (the mesh through `fs-io`), N33b-14 (group `structure`), BD-401 |
| KA-771 | Incremental recomputation through `fs-recompute` on 10⁴ random edit sequences | Equal to full recomputation, digest for digest | exact | G3 · host | An input left out of a node key is caught by the shadow recompute within 64 hits | INT-FSIM-16; N33b-15 (the `Recompute` adapter), N33a-17, BD-111 |
| KA-772 | Poiseuille flow between parallel plates; film mass over 10⁴ steps, across the multiphysics bridge | The velocity profile is the parabola u(y) = (G/(2·μ))·y·(H − y) for pressure gradient G, viscosity μ and gap H, to the crate's own stated tolerance; total film mass is conserved | 10⁻¹² relative for mass | G1 · referee (evaluation tier) | The bridge refuses when the `multiphysics` feature is off (`FROB-REF-790`) and refuses a throughput-tier request | INT-FSIM-17; N33b-16 (the gated bridge), BD-413 |
| KA-773 | PPO clipped surrogate L = min(ρ·Â, clip(ρ, 1 − ε, 1 + ε)·Â) with advantage Â = −1 and ε = 0.2 | At ratio ρ = 0.5 the terms are −0.5 and −0.8: the clipped term is the minimum and the gradient is zero. At ρ = 2 they are −2 and −1.2: the unclipped term is the minimum. Finite differences of the loss in the log standard deviation at ρ = 0.5, 1, 2: 0, −0.5, −1 | 10⁻³ relative in single precision | G1 · host | The reported sibling code path gives −0.25, −0.5 and 0 | INT-FSIM-06; BD-034; N23a-10 (the PPO update, which must not inherit the defect) |
| KA-774 | Critic output 2 with stored value 0 and return 0; two independently reset one-step episodes, packed | The squared-error objective gives gradient 2 into the critic bias; a frozen policy's likelihood ratio is 1 on both episodes | ratio 1 to 10⁻⁶ | G1 · host | The reported sibling code path gives gradient 0 and a second ratio of 1.582459 | INT-FSIM-06; BD-034; N23a-10, N23a-11 |
| KA-775 | Planar drive near zero curvature: heading θ = 32 rad, wheel speeds 1 and 1 + 6·10⁻¹³ m/s, track 0.5 m, Δt = 1/400 s, 400 steps | ω = (6·10⁻¹³)/0.5 = 1.2·10⁻¹² rad/s and ω·Δt = 3·10⁻¹⁵ rad, below half the spacing of doubles at 32 (2⁻⁴⁸ ≈ 3.55·10⁻¹⁵). The true displacement is one metre along the heading: (cos 32, sin 32) ≈ (0.83422336, 0.55142668) m | 10⁻¹² | G1 · host, referee | The reported branch returns exactly zero translation (θ + ω·Δt rounds to θ, so both trigonometric differences are zero) | INT-FSIM-05; BD-033 |
| KA-776 | The independent chord formula Δp = v·Δt·sinc(δ/2)·(cos(θ + δ/2), sin(θ + δ/2)) with δ = ω·Δt, sinc(x) = sin(x)/x, sinc(0) = 1: unit speed through a half turn from θ = 0 | With v·Δt = 1 m of path and δ = π (an arc of radius 1/π m): sinc(π/2) = 2/π and the direction is (cos π/2, sin π/2) = (0, 1), so the chord is (0, 2/π) m = (0, 0.6366197723675814) m | 10⁻¹² | G1 · host | Set here (the plan states none): replacing sinc(δ/2) by 1 (the straight-line step) returns (0, 1) m and must fail | INT-FSIM-05; BD-033 |
| KA-777 | A contiguous 2×2 single-precision view: strides (8, 4) in bytes and (2, 1) in elements; a second view with byte origin 4 and byte strides (8, −4) | Index [1, 1] is element 1·2 + 1·1 = 3. The negative-stride view addresses elements (1, 0, 3, 2) and is materialized with the copy charged to the caller, or refused under a zero-copy contract (`FROB-REF-617`) | exact | G1 · host | Copying (8, 4) verbatim as element strides over a 16-element buffer addresses element 8 + 4 = 12, in bounds, with no fault | INT-FNP-02; BD-036; N33a-28 (`admission_m0.sh`) |
| KA-778 | A wheel rolling flat, onto an edge, then onto the top of a riser | The sustained contact step returns its typed refusal through the airlock at the pin; the same transition completes under witness migration with the energy ledger closed to its stated residual | the ledger's residual bound | G1 · referee | A set of 22 contacts (capacity 21) returns the capacity refusal `FROB-REF-721` | INT-FSIM-07, INT-FSIM-04; BD-035, N33a-13, N33a-14 (M0 refusal half), BD-093 (M1 witness migration); N33a-29, N33ap1-01 (scripts) |
| KA-779 | `frob-units` aliases against `fs-qty` (only if the escape hatch opens) | Every alias product agrees in a differential test while both crates exist; a round trip through the airlock conversion preserves the value bits; adding a pressure to a time fails to compile | exact | G0 · host | Adding a torque to an energy must fail to compile although both have the dimension vector (kilogram 1, metre 2, second −2) | INT-TOOLCHAIN-01; BD-031, N33c-03; N33c-05, N33c-06 (conditional: built only if the hatch opens) |
| KA-780 | A 2,000-step G1 referee rollout submitted through the worker process `frob-fsim-worker` | Chunk digests equal, bit for bit, to the same job run in process; the same after killing the worker mid-job and resubmitting; fixed overhead at most 50 ms per job and at most 5% of wall time on jobs of one second or more (TARGET) | exact for digests; overhead as stated | G5 · referee | A worker built at a different commit refuses the job (`FROB-REF-791`) | INT-FSIM-18; BD-038, N33c-10 to N33c-13; N33b-06 (if fallback (a) is adopted) |

**Derivation files.** For each row, a short program or a text derivation that needs no FrankenRobotics code:
KA-769 evaluates Φ by the complementary error function of the deterministic elementary-function set; KA-775
prints the three numbers 1.2·10⁻¹², 3·10⁻¹⁵ and 2⁻⁴⁸ and shows the rounding; KA-777 enumerates the four indices
of both views. A derivation file is never regenerated from the candidate implementation.

**Controls.** Each `Negative control` cell is the row's `control.text`. The seeding tool of N38b-05 makes it a
record of `controls.toml` with a stable `NC-` identifier and `seed = "KA-nnn"`, or reuses the record of a
counterexample with the same plant and target. The record carries the gates that must catch it (the row's owner
gate and the gates the cell names), its `kind` and its `expect`, which is the wrong value quoted above. The kinds,
stated with each control in the derivation file so that the seeding and the review read one source:
- `mutation`, a planted defect in code, for KA-771, the sibling-defect rows KA-773 to KA-775, KA-776 and KA-777;
- `planted-input`, a planted file, tree, program or request, for KA-767 to KA-770, KA-772, KA-778, KA-779 and
  KA-780.

Every one of the fourteen rows has exactly one kind. N38b-34 registers a second record for KA-776 (the same
mutation, sinc(δ/2) replaced by 1). This bead's text is first.

## Acceptance criteria

- [ ] `crates/frob-conformance/registry/known_answers.toml` holds exactly fourteen `[[ka]]` records of this chapter, KA-767 to KA-780, each with every field of N38a-12 (section, owner, due and label as stated above), and the loader of N38a-12 validates the file; no `registry/known_answers/suite.toml` and no derivation under `crates/frob-conformance/derivations/suite/` exists.
- [ ] Each record has a derivation file at `docs/derivations/<fixture>.md` that reproduces its truth with the standard library alone and names in its front matter the test path, which exists once the carrying bead lands; until then the runner reports the row `unbound` or `not due` (N38a-13), never `pass`.
- [ ] After N38b-05's seeding is re-run, each of the fourteen rows has its control in `control.nc` (KA-776's from the text set here), each seeded record with one `kind` as listed above and at least one gate.
- [ ] KA-770's tolerance is recorded as 10⁻¹² relative with the fixture a = (1, 2), σ = (0.03, 0.02) mm, so that no row of this chapter remains `to set`.
- [ ] The completeness test (below) passes: no row without a carrying test, no control without a gate.
- [ ] Each row appears with its tier, site and outcome in the receipts of the tier runners (N38a-13) and in the generated printout of N38a-12 (`tests/known_answers_printout.rs`). No `frob` command reports the harness: it is never driven through the surface it judges (ADR-GAUNT-01).
- [ ] KA-779's derivation file records the decision it is conditional on (`conditional_on = "BD-031: escape-hatch"`), and the row is never reported `pass` while the escape hatch is closed.

## Test plan

### Unit tests (`cargo test -p frob-conformance --test ka_suite`, `crates/frob-conformance/tests/ka_suite.rs`)
- `suite_ka_one_file`: a scratch copy that moves a row into `registry/known_answers/suite.toml`, or adds a key `gate` or `runs_in` to a row, is refused (`ka-second-file`; unknown key).
- `suite_ka_sections_owners_due`: each row's section, owner, label and due equal the lists above.
- `suite_ka_rows_are_fourteen_and_contiguous`: identifiers 767 to 780, no gap, no duplicate.
- `suite_ka_derivations_reproduce`: each derivation program runs and prints the truth: Φ(−1/√2) = 0.23975006109347673 to 10⁻¹⁵; 2/π = 0.6366197723675814; element index 3 and the tuple (1, 0, 3, 2); (cos 32, sin 32) = (0.83422336, 0.55142668) to eight digits; 0.0009 + 0.0016 = 0.0025 and √0.0025 = 0.05.
- `suite_ka_rounding_argument`: in `f64`, 32.0 + 3e-15 == 32.0 is true, and 2⁻⁴⁸ > 3e-15 is true.
- `suite_ka_every_row_has_control_and_gate`: a join of the three registries (known answers, controls, gates) with no unmatched row (the registry-seed completeness obligation PO-GAUNT-03, restricted to this chapter).
- `suite_ka_ids_append_only`: a planted registry diff that renumbers KA-770 fails.

### End-to-end script and logging
`scripts/e2e/suite_ka_registry.sh` checks the registry and nothing a later bead delivers.
- **Header and log.** Its header is `bead=N33-KA milestone=M0 labels=suite,registry gates=`. It sources the
  STD-E2E helper and writes NDJSON to `target/e2e-logs/suite_ka_registry/<utc>.ndjson` with a transcript.
- **Steps.** It runs `cargo test -p frob-conformance --test registry_check` and `--test ka_suite`.
- **Metric lines.** It logs:
  - the row count (14);
  - each row's computed eligibility, owner and due;
  - the `control.nc` count per row after seeding;
  - KA-770's tolerance;
  - KA-779's conditional state;
  - the digest of `known_answers.toml`.
- **Planted scratch defects.** On scratch copies it plants KA-776's control text deleted, KA-770's tolerance put
  back to `to-set`, and a row moved into a chapter file, and requires each to be caught.
- **Line fields.** Every line carries `ts`, `script`, `bead`, `step`, `event`, the exact command, the exit code
  and the wall time. On failure it also logs the first differing value. The `summary` gives the assertions
  passed and failed and `gates: []`.

The rows themselves are exercised by `scripts/e2e/stack_receipt.sh` (KA-767, bead N33a-27),
`scripts/e2e/admission_m0.sh` (rows due at M0, bead N33a-28) and the scripts of the carrying beads. Each of those
scripts logs, per row, an `assert` line with the KA number, the measured value with its unit, the tolerance, the
tier and the control's `NC-` identifier with `caught: true|false`, in the shared NDJSON format of `STD-E2E`.

### Known answers and negative controls
The table above is the content. Metamorphic relation that applies to the registry itself: reordering the records
in the file changes no test outcome. Determinism lanes: the derivation programs print identical bytes twice on one
machine, in debug and release, and on Linux and macOS.

## Considerations

- KA-773 to KA-775 and KA-777 quote values reported from a source reading on 2026-11-21 that was not re-executed. The admission tests record what the pinned commit really does; if a sibling has since been fixed, the control is still a valid planted mutation and the record notes "fixed upstream at <rev>".
- KA-779 only runs if the `frob-units` escape hatch opens (the units crate fails on the chosen toolchain and the upstream change cannot land within a toolchain train). Until then the runner reports it `not-run: conditional (BD-031: escape-hatch)` (N38a-13's outcome for a row whose derivation carries `conditional_on`), which is not a pass and does not block an exit.
- KA-772 is an evaluation-tier row at M6 and is off every critical path.
- The alias count gate of the units escape hatch (at most 64 aliases, TARGET) is OPEN-SUITE-4; it is carried with BD-031, not here.

## References

Appendix M.32 (KA-767 to KA-780); §33.5, §33.9, §33.11, §33.12, §33.14; INT-FSIM-05, -06, -07, -13 to -18,
INT-FNP-02, INT-TOOLCHAIN-01; PO-GAUNT-03; FROB-REF-617, -721, -790, -791.
