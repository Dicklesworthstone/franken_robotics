---
key: "N31-REF"
title: "Refusal registry of Foundry (§31): fourteen codes with triggers, payloads, ranked repairs and exit codes"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "foundry", "frob-foundry", "frontier", "registry"]
parent: "WP-70"
deps: ["BD-005", "BD-022", "STD-TEST", "STD-E2E"]
needs: ["§32.4: the envelope's `refusal` object and the exit-code table (Tether unit)", "Appendix O: allocation of codes for the three refusals of §31.21 to §31.23 that the plan states without a number (registry owner of `registries/refusals.toml`)", "§32: registration of the Foundry verbs with their refusal lists (BD-409)"]
related: ["BD-400", "BD-401", "BD-402", "BD-403", "BD-404", "BD-405", "BD-406", "BD-407", "BD-408", "BD-423", "BD-438", "N31b-33", "N31b-36", "N31b-40", "N31-CX"]
gates: []
plan_refs: ["§31.16", "§31.15", "§31.6", "§31.8", "§31.9", "§31.11", "§31.12", "§31.13", "§31.14", "§31.21", "§31.22", "§31.23", "§32.4", "Appendix O"]
---
## Background

The program's rule is that an invalid request is refused with a stable code, a message and ranked repairs,
never repaired in silence and never answered with a number that looks valid. Every refusal code lives in
`registries/refusals.toml`; an unknown code is a load-time error; codes are never reused. With `--json` a
refusal is an envelope with `ok: false` and a `refusal` object: `code`, `message`, `pointer` (the field or part
at fault), `observed`, `required`, `unit`, `affected_claims`, `retry` (`never`, `after-change` or `backoff`)
and ranked `repairs`.

Foundry (structure, manufacture, lifecycle) defines fourteen codes. They sit in three blocks of the registry by
kind and not by chapter: 163 to 167 and 169 in the design-and-model block, 260 to 262 in the numerics block,
and 818, 824, 827, 831 and 860 in the evidence-and-protocol block. All fourteen map to process exit code 3
("refused input or question"): the exit table sends the 1xx to 5xx blocks and the 8xx block other than
conflicts to 3. None is a budget stop (4), a missing capability (5), a device fault (6), a conflict (7) or an
integrity fault (9).

This bead owns the registry rows, the per-code payload contract and the test that each code is raised by at
least one fixture and carries its repair. The mechanisms that raise the codes are built in the beads named per
row.

## Scope

In: the fourteen rows; payloads; ranked repairs; the map from verbs to codes; three refusals that the chapter
states without a number; the codes of other chapters that Foundry verbs also raise (listed, not owned).
Out: the checks themselves; the verb registry rows (the M5 verbs bead).

## Technical approach

**Table 1: the fourteen codes.** Exit code 3 for every row. `retry` is `after-change` unless stated.

| Code | Meaning | Raised when (trigger) | Payload beyond the common fields | Ranked repairs | Raised by | Mechanism bead |
|---|---|---|---|---|---|---|
| FROB-REF-163 | A check whose material card lacks the field or provenance it needs | A structural check needs yield strength, modulus, density, a stress-life curve, a scatter factor or a knock-down factor, and the card lacks the field, or the field has no provenance, or the curve is not valid for the part's material state, process and surface finish | The card identity, the field, the check that needed it | 1. Supply the missing field with provenance. 2. Accept `fatigue: no-claim` for the fatigue criterion alone (the other criteria still answer) | `foundry.size`, `foundry.check` | BD-401, BD-402 |
| FROB-REF-164 | An empty design region, or ports with no load path inside the envelope | The keep-in envelope minus swept volumes of neighbours minus keep-outs is empty, or the frozen port bosses are not connected inside it | The blocking keep-outs, the disconnected port pair, the remaining volume | 1. Shrink or move the named keep-out. 2. Enlarge the keep-in envelope. 3. Fall back to the beam rung for the link | `foundry.optimize` | BD-403 |
| FROB-REF-165 | An allocation with a zero or unknown sensitivity, or a failed band-extreme check | A feature's first-order sensitivity is zero or absent; or the true quantity evaluated at the tolerance-band extremes leaves the specification | The feature, its sensitivity and class, the band-extreme value against the limit | 1. Worst-case boxes for the named features (interval evaluation). 2. Supply a second-order sensitivity or a simulated one. 3. Tighten the named feature | `foundry.tolerance` | BD-406 |
| FROB-REF-166 | Optimization or packet export for a design with no qualification verdict | `foundry.optimize` or `foundry.export` is asked for a design whose `DesignId` and `ModelId` have no qualification verdict, without `--exploratory` | The design and model identities; the verdict state | 1. Qualify the design. 2. Re-run with `--exploratory`: the result is labelled and cannot enter a packet | `foundry.optimize`, `foundry.export` | BD-403, BD-408 |
| FROB-REF-167 | A load case that is not one instant, or a campaign failing the free-body check | A case mixes samples of different instants, episodes or world addresses; or more than 0.1% of campaign samples exceed the free-body residual tolerance | The offending samples with world address, tick and residual; the dropped fraction | 1. Rebuild the case from one sample. 2. Inspect the offending samples (port map, frame, sign conventions). 3. Re-run the campaign on the referee for the named worlds | `foundry.loads` | BD-400 |
| FROB-REF-169 | Telemetry without the traces or samples a load metric needs | The campaign lacks control-rate traces for a world, a port or a body the metric names, or has a gap inside a requested window | The telemetry level required and present, the worlds and intervals missing | 1. Re-run the campaign with control-rate traces on the named worlds. 2. Restrict the metric to covered worlds (stated in the result) | `foundry.loads` | BD-400 |
| FROB-REF-260 | A loads–mass loop that diverges, cycles or exceeds its iteration budget | Estimated contraction at least 1 on two successive iterations; or alternating increments with contraction near 1 after one under-relaxed step; or five iterations without meeting the convergence test | The contraction estimate per iteration, the last iterates, the reason (`diverged`, `cycling`, `budget`) | 1. Use the beam rung for mass and inertia feedback. 2. Hold the named stock size fixed. 3. Raise the stress cap margin | `foundry.optimize` | BD-404 |
| FROB-REF-261 | An optimized part that fails the independent check | The extracted geometry fails stress, buckling, modal or fatigue utilization on the body-fitted mesh | The governing case, location, criterion and utilization | 1. Re-run with a tighter stress cap. 2. Re-run with a larger minimum length scale. 3. Add the stated fillet radius | `foundry.check`, `foundry.optimize` | BD-402, BD-403 |
| FROB-REF-262 | Quasi-static superposition where the frequency separation fails | The link's first natural frequency is below five times the highest frequency carrying 99% of load power, and the transient path is not enabled | Both frequencies and their ratio; the windows concerned | 1. Enable the transient path for impact windows. 2. Stiffen the link (the beam rung reports the wall needed) | `foundry.loads` | BD-400 |
| FROB-REF-818 | A priced parts list without a pinned, licensed catalogue | A request asks for prices, or a parts card carries a price, and no catalogue with a pinned revision and a licence is registered | The part, the catalogue state | 1. Omit prices (card references and quantities only) | `foundry.export` | BD-408 |
| FROB-REF-824 | A STEP export above its bounds or failing bit-exact replay | The faceted solid would exceed the instance limit (10⁶ by default, about 285,000 triangles), or the decode and re-emit does not reproduce every coordinate bit and triangle index | The instance count and the bound, or the first differing entity | 1. Decimate under a stated Hausdorff tolerance and re-check. 2. Export the mesh formats only | `foundry.export` | BD-407 |
| FROB-REF-827 | A fixed-point obligation refuted or undecided within budget | A bit-vector overflow query is satisfiable (a counterexample exists) or times out (2 s by default) | The intermediate, its format, the counterexample input or `undecided` | 1. Use the counterexample to add a clamp or narrow the declared range. 2. Choose a wider format | `foundry.export --with-firmware` | BD-438 |
| FROB-REF-831 | A lifecycle law outside its validity domain, or cited above class Estimated | A slow-state law is evaluated outside the temperature, load or throughput range of its card; or a lifecycle output is offered where a stronger class is required | The law, the domain, the operating point | 1. Supply a card valid for the point. 2. Restrict the run to the domain (stated in the result) | `foundry.lifecycle` | BD-423 |
| FROB-REF-860 | An export without the flags, or any attempt to produce a true flag | A packet or exported file lacks any of the four no-authorization flags, or carries one that is not `false`, at write time, at read time or in the offline auditor | The artifact, the flag, the value found | None. `retry: never` | `foundry.export`, the packet reader, `frob-verify` | BD-408 |

**Table 2: verbs and the codes they raise.** The chapter's verb table prints an older numbering in its
refusal column (160, 161, 162 and 861 to 864, which the registry assigns to other chapters). The chapter's
refusal table governs. Corrected map (DESIGN reading: the two lists are in the same order):

| Verb | Codes |
|---|---|
| `foundry.loads` | 167, 169, 262 |
| `foundry.size` | 163 |
| `foundry.optimize` | 166, 164, 260 (and 261 when the loop's final check fails) |
| `foundry.check` | 163, 261 |
| `foundry.tolerance` | 165 |
| `foundry.export` | 166, 860, 818, 824, and 827 with `--with-firmware` |
| `foundry.lifecycle` | 831 |

**Table 3: refusals the chapter states without a code.** Each needs a registry row before its mechanism
ships. Until the registry owner allocates one, the mechanism bead carries the refusal under the name below and
the stated interim.

| Name | Refused | Repair | Interim (DESIGN) | Mechanism bead |
|---|---|---|---|---|
| `review-record-packet-mismatch` | Import of a review record whose `packet` digest is not the manifest digest of the packet named | Re-issue the record against the right packet | A new code in the evidence-and-protocol block; no existing code fits | N31b-33 |
| `provisional-load-case-in-packet` | A build-packet request that contains an envelope case still labelled `provisional` | Re-extract on the referee; or run exploratory (labelled; cannot enter a packet) | The existing trust-ladder code FROB-REF-820 ("a result that is not `established` is cited as established"), with the pointer at the case | N31b-36 |
| `under-built-promotion` | Promotion of a design with a link that is `under-built` under measured loads | A typed edit that sets the sized section; the link, the ratio and the section are in the payload | A new code in the design-and-model block | N31b-40 |

**Table 4: codes of other chapters that Foundry verbs raise** (listed so a caller's handler is complete; not
owned here): FROB-REF-790 (the airlock's structural feature group is absent from this binary; exit 5);
FROB-REF-888 (a metric whose telemetry samples are missing, the general form of 169); FROB-REF-816 (the general
form of 860 for artifacts other than packets) and FROB-REF-833 (the offline auditor's code for a certificate
bundle whose flags are missing or true); FROB-REF-820 and -830 (label misuse; a `model-conditional`
result offered for promotion or build); FROB-REF-891 to -893 (format version, view offered as identity, owner
bypass) on `.fbuild`; FROB-REF-113 (a card field of class `assumed` under `--strict-data`); FROB-REF-869 (a
claim whose evidence closure dangles).

**Registry rows.** Each of the fourteen codes gets a row in `registries/refusals.toml` with `code`, `name`
(kebab-case: `material-card-incomplete`, `design-region-empty`, `sensitivity-zero-or-band-failed`,
`not-qualified`, `load-case-not-coherent`, `load-telemetry-missing`, `loads-mass-loop-not-converged`,
`independent-check-failed`, `frequency-separation-failed`, `priced-without-catalogue`, `step-bound-or-replay`,
`fixed-point-obligation`, `lifecycle-law-domain`, `flags-missing-or-true`; names are DESIGN), `message`,
`exit = 3`, `retry`, `repairs` (ordered) and `defined_in`.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the fourteen rows; the loader rejects a duplicate or a reused code.
- [ ] Each code is raised by at least one fixture through `frob --json`, with exit 3, the payload fields of Table 1 and at least one repair (860 has none and says `retry: never`).
- [ ] The verb registry's refusal lists for the seven Foundry verbs equal Table 2.
- [ ] The three refusals of Table 3 have registry rows, or a recorded request and the interim in force.
- [ ] No Foundry refusal is emitted with an unregistered code (registry-closure test).

## Test plan

### Unit tests
`cargo test -p frob-foundry refusals`: one test per code named `ref_<code>_<name>`, each constructing the
smallest input that triggers it and asserting code, exit, pointer and repair order: `ref_163` (card without
`yield_strength`); `ref_164` (keep-out covering the whole envelope); `ref_165` (sensitivity 0); `ref_166`
(design without a verdict; and `--exploratory` accepted and labelled); `ref_167` (a case built from ticks 10
and 11; and a campaign with 0.2% dropped samples); `ref_169` (campaign without control-rate traces);
`ref_260` (planted gain 1.1); `ref_261` (planted overstress at utilization 1.25); `ref_262` (f₁ = 4 × the
loaded frequency); `ref_818`; `ref_824` (300,000 triangles; and a replay with one flipped coordinate bit);
`ref_827` (Q15 product of −1 and −1); `ref_831` (temperature above the card's range); `ref_860` (a manifest
with `fabrication_released: true`, and one with the field absent). `every_code_has_a_fixture`;
`exit_code_is_3_for_all`.

### End-to-end script and logging
`scripts/e2e/foundry_refusals.sh`: for each code runs the `frob foundry …` command of its fixture with
`--json`, asserts `ok: false`, the code, exit 3 and the first repair, then applies the first repair where one
exists and asserts the command now succeeds (for 163: adding the field; for 166: `--exploratory` with the
label present; for 824: decimation). Logs per STD-E2E (`event: refusal` lines with code, pointer, observed,
required, repairs; `event: assert` for the repair replay) to `target/e2e-logs/foundry_refusals/`; the summary
counts codes exercised out of fourteen.

### Known answers and negative controls
Known answers: the trigger numbers of each fixture (0.1% drop limit; ratio 5; 10⁶ instances and 285,713
triangles; tick 82; 2 s). Negative controls: a build in which a refusal is downgraded to a warning (the script
must fail); a registry file with code 163 duplicated (the loader must fail); a fixture whose repair does not
clear the refusal (reported as a defect of the repair list).

## Considerations

- 860 overlaps two other flag codes: 816, defined by the safety chapter for artifacts in general, and 833, defined by the certificates chapter for bundles in the offline auditor. Packets report 860 (the chapter governs its own artifact); a reader must treat all three as "foreign or tampered: refuse". Folding them is the registry owner's call.
- 166 has two repairs of very different weight. `--exploratory` is not a way around qualification: its output is labelled and the packet writer rejects it.
- The validity clause of 163 (a curve not valid for the material state, process and finish) is the reason printed parts read `fatigue: no-claim` until stress-life data exists for them (open question OPEN-FOUNDRY-4).
- Codes 827 and 831 belong to Moonshot features and are registered at M5 so the numbers are reserved; their fixtures run only when the features are built.

## References

§31.16, §31.15, Appendix O (O.1, O.2, O.8), §32.4; FROB-REF-163 to -167, -169, -260 to -262, -818, -824, -827, -831, -860; §31.21 to §31.23 for the unnumbered refusals.
