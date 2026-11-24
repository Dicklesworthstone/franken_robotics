---
key: "N31-REF"
title: "Refusal registry of Foundry (§31): fourteen codes with triggers, payloads, ranked repairs and exit codes"
type: "task"
priority: 3
milestone: "M5"
labels: ["m5", "foundry", "frob-foundry", "frontier", "registry"]
parent: "WP-70"
deps: ["BD-005", "BD-022", "N31b-45", "N32a-03", "STD-E2E", "STD-TEST", "VERBS-foundry"]
needs: []
related: ["N31a-01", "N31a-03", "N31a-06", "N31a-07", "N31a-12", "N31a-13", "N31a-14", "N31a-19", "N31a-20", "N31a-21", "N31a-22", "N31a-24", "N31a-33", "N31a-35", "N31a-36", "N31a-37", "N31a-39", "N31a-43", "N31a-44", "N31a-45", "N31a-46", "N31a-47", "N31a-48", "N31a-49", "N31a-50", "N31a-51", "N31a-53", "N31a-54", "N31a-55", "N31a-56", "N31a-57", "N31a-58", "N31a-59", "N31a-62", "N31ax-01", "N31b-05", "N31b-25", "N31b-27", "N31b-28", "N31b-29", "N31b-30", "N31b-33", "N31b-35", "N31b-36", "N31b-40", "N31b-41", "N31b-44", "N35x-01", "N35x-02"]
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

In: the fourteen rows as data in `registries/refusals.toml` (code, name, message, trigger, payload fields, ranked
repairs, exit, retry); the map from verbs to codes; the refusals that the chapter states without a number,
listed by name with their interim; the codes of other chapters that Foundry verbs also raise (listed, not
owned); the registry-level checks (the loader rejects a duplicate or a reused code; the closure test; the
equality of Table 2 with the verb registry's refusal lists).
Out: the checks that raise a refusal (the mechanism beads); the typed payload types and their rendering
(N31a-52); the trigger battery through the real verbs (N31ap1-51, split from N31a-52, which depends on this
bead: the per-code tests `ref_<code>_<name>` and the repair-replay script `scripts/e2e/foundry_refusals.sh` of
the Test plan below state the binding each row needs and are implemented there, once, beside the battery; the
two Moonshot rows' tests `ref_827` and `ref_831` are written by the feature lanes N31a-58 and N31a-59 to the
same naming); the allocation of numbers for the uncoded refusals (the
decision bead N31b-45); the verb registry rows (entered with their refusal lists by VERBS-foundry at M4;
N31a-51 repeats them and enters none; completed for M5 by BD-409).

## Technical approach

**Table 1: the fourteen codes.** Exit code 3 for every row. `retry` is `after-change` unless stated. The
ranked repairs of a row are its whole list: the chapter's own first, then the further repairs the mechanism
beads name. The rendering (N31a-52) prints the row's list in this order and leaves out a repair that does not
apply to the payload's reason, so the first repair rendered is the first applicable one of the row.

| Code | Meaning | Raised when (trigger) | Payload beyond the common fields | Ranked repairs | Raised by | Mechanism bead |
|---|---|---|---|---|---|---|
| FROB-REF-163 | A check whose material card lacks the field or provenance it needs | A structural check needs yield strength, modulus, density, a stress-life curve, a scatter factor or a knock-down factor, and the card lacks the field, or the field has no provenance, or the curve is not valid for the part's material state, process and surface finish; with `--require fatigue` (or a packet that would record a fatigue pass), a fatigue criterion that reads `no-claim`. A field of provenance `assumed` under `--strict-data` is not this code: it is FROB-REF-113, the program's one strict-data code (N31a-22, N31b-27; Table 2) | The card identity, the field, the check that needed it | 1. Supply the missing field with provenance. 2. Accept `fatigue: no-claim` for the fatigue criterion alone (the other criteria still answer). 3. Use a card that has the field | `foundry.size`, `foundry.check` | BD-401, BD-402 |
| FROB-REF-164 | An empty design region, or ports with no load path inside the envelope | The keep-in envelope minus swept volumes of neighbours minus keep-outs is empty, or the frozen port bosses are not connected inside it | The blocking keep-outs, the disconnected port pair, the remaining volume | 1. Shrink or move the named keep-out. 2. Enlarge the keep-in envelope. 3. Fall back to the beam rung for the link. 4. Narrow the named joint range by the stated angle, where a neighbour's swept volume is what blocks the path (N31a-14) | `foundry.optimize` | BD-403 |
| FROB-REF-165 | An allocation with a zero or unknown sensitivity, or a failed band-extreme check | A feature's first-order sensitivity is zero or absent; or the true quantity evaluated at the tolerance-band extremes leaves the specification | The feature, its sensitivity and class, the band-extreme value against the limit | 1. Worst-case boxes for the named features (interval evaluation). 2. Supply a second-order sensitivity or a simulated one. 3. Tighten the named feature. 4. A wider margin or another process (N31a-33) | `foundry.tolerance` | BD-406 |
| FROB-REF-166 | Optimization or packet export for a design with no qualification verdict | `foundry.optimize` or `foundry.export` is asked for a design whose `DesignId` and `ModelId` have no qualification verdict, without `--exploratory` | The design and model identities; the verdict state | 1. Qualify the design. 2. Re-run with `--exploratory`: the result is labelled and cannot enter a packet. 3. For an optimized design not yet re-qualified: export the packet of the pre-optimization design, whose links are then the beam-rung sections (N31a-03) | `foundry.optimize`, `foundry.export` | BD-403, BD-408 |
| FROB-REF-167 | A load case that is not one instant, or a campaign failing the free-body check | A case mixes samples of different instants, episodes or world addresses; or more than 0.1% of campaign samples exceed the free-body residual tolerance | The offending samples with world address, tick and residual; the dropped fraction | 1. Rebuild the case from one sample. 2. Inspect the offending samples (port map, frame, sign conventions); when off-port samples are what exceed the budget this repair is rendered as the change to the port map it points at, "declare a contact port on link L" (N31a-07). 3. Re-run the campaign on the referee for the named worlds. 4. Fix the frame or sign named by the median residual wrench; for off-port samples instead, review the gait, since the task makes unplanned contact (N31a-07) | `foundry.loads` | BD-400 |
| FROB-REF-169 | Telemetry without the traces or samples a load metric needs | The campaign lacks control-rate traces for a world, a port or a body the metric names, or has a gap inside a requested window | The telemetry level required and present, the worlds and intervals missing | 1. Re-run the campaign with control-rate traces on the named worlds. 2. Restrict the metric to covered worlds (stated in the result; not offered for a symmetric class's coverage rule). 3. For a symmetric class whose limbs are not all covered: read the missing limbs' traces (N31a-11) | `foundry.loads` | BD-400 |
| FROB-REF-260 | A loads–mass loop that diverges, cycles or exceeds its iteration budget | Estimated contraction at least 1 on two successive iterations; or alternating increments with contraction near 1 after one under-relaxed step; or five iterations without meeting the convergence test | The contraction estimate per iteration, the last three iterates, the governing link class and component, the reason (`diverged`, `cycling`, `budget-exhausted`; for `cycling` the two stock sizes) | 1. Accept the beam rung's section for the governing class (mass and inertia feedback from the beam rung) and re-run. 2. For cycling, lock the heavier of the two named stock sizes and re-run. 3. Reduce the governing load by a design change named by the governing envelope case (as N31a-24 and N31a-52 state them; a larger stress-cap margin is not offered, since it raises the gain λ) | `foundry.optimize` | BD-404 |
| FROB-REF-261 | An optimized part that fails the independent check | The extracted geometry fails stress, buckling, modal or fatigue utilization on the body-fitted mesh | The governing case, location, criterion and utilization | 1. Re-run with a tighter stress cap. 2. Re-run with a larger minimum length scale. 3. Add the stated fillet radius. For a buckling or a modal failure repairs 1 and 3 do not apply and repair 2 is the first rendered (the length scale raised by one stock step); the further repairs are N31a-20's. Buckling: 4. re-run with the sibling's buckling-constrained allocation enabled; 5. for a local wall mode, add a wall-thickness floor at the mode's location. Modal: 6. re-run with the first-mode constraint tightened to the floor × 1.05; 7. a stiffer material; 8. enable the transient path, when only the 5·f₉₉ term fails (the label `quasi-static: not separated` is attached) | `foundry.check`, `foundry.optimize` | BD-402, BD-403 |
| FROB-REF-262 | Quasi-static superposition where the frequency separation fails | The link's first natural frequency is below five times the highest frequency carrying 99% of load power, and the transient path is not enabled | Both frequencies and their ratio; the windows concerned | 1. Enable the transient path for impact windows. 2. Stiffen the link (the beam rung reports the wall needed). 3. A higher trace rate, when the class is `under-sampled` (f₉₉ above 0.4 × the control rate; N31a-12) | `foundry.loads` | BD-400 |
| FROB-REF-818 | A priced parts list without a pinned, licensed catalogue | A request asks for prices (`--priced`), or a parts card carries a price, and no catalogue with a pinned revision and a licence is registered; under `--strict-data`, a priced list whose catalogue is marked `assumed` (N31a-39) | The part, the catalogue state | 1. Omit prices (card references and quantities only) | `foundry.export` | BD-408 |
| FROB-REF-824 | A STEP export above its bounds or failing bit-exact replay | The faceted solid would exceed the instance limit (10⁶ by default: 285,712 triangles of a closed shell); or the decode and re-emit does not reproduce every coordinate bit and triangle index; or the mesh offered is not a closed shell of triangles (an open shell, a non-triangle face), which the STEP writer of the format sibling refuses and this code carries with the sibling's reason | The instance count and the bound, or the first differing entity, or the sibling's reason (`open-shell`, `non-triangle-face`, `over-limit`, `replay`) | 1. Decimate under a stated Hausdorff tolerance and re-check. 2. Close the shell (a repair outside Foundry). 3. Export the mesh formats only | `foundry.export` | BD-407 |
| FROB-REF-827 | A fixed-point obligation refuted or undecided within budget | A bit-vector overflow query is satisfiable (a counterexample exists) or times out (2 s by default) | The intermediate, its format, the counterexample input or `undecided` | 1. Use the counterexample to add a clamp or narrow the declared range. 2. Choose a wider format. 3. A larger budget, for `undecided` only (N31a-45) | `foundry.export --with-firmware` | BD-438 |
| FROB-REF-831 | A lifecycle law outside its validity domain, or cited above class Estimated | A slow-state law is evaluated outside the temperature, load or throughput range of its card; or a lifecycle output is offered where a stronger class is required | The law, the domain, the operating point | 1. Supply a card valid for the point (a card whose domain covers the observed range). 2. Restrict the run to the domain (stated in the result). 3. A design change that brings the quantity inside the domain (for example a lower duty). 4. Run without the law, its state reported `no-claim` (N31a-48) | `foundry.lifecycle` | BD-423 |
| FROB-REF-860 | An export without the flags, or any attempt to produce a true flag | A packet or exported file lacks any of the four no-authorization flags, or carries one that is not `false`, at write time, at read time or in the offline auditor | The artifact, the flag, the value found | None. `retry: never` | `foundry.export`, the packet reader, `frob-verify` | BD-408 |

**Table 2: verbs and the codes they raise.** The chapter's verb table prints an older numbering in its
refusal column (160, 161, 162 and 861 to 864, which the registry assigns to other chapters). The chapter's
refusal table governs. Corrected map (DESIGN reading: the two lists are in the same order):

| Verb | Codes |
|---|---|
| `foundry.loads` | 167, 169, 262; and, from the trust-ladder steps of §31.22: 821 (re-extraction against a wrench envelope not frozen, N31b-35) and 820 (the interim of `provisional-load-case-in-packet` under `--admit packet`, N31b-36) |
| `foundry.size` | 163 (and 167 for a load file that is not one instant; 169 for a class load set that does not cover every instance; 113 under `--strict-data` for a required material-card field of provenance `assumed`, N31a-22) |
| `foundry.optimize` | 166, 164, 260, 261 (261 when the loop's final check fails; and 167, 169, 262 from the campaigns it runs; 113 under `--strict-data`, N31a-22) |
| `foundry.check` | 163, 261 (and 860 when the argument is a packet; 169 for a class load set that does not cover every instance; 113 under `--strict-data` for a required card field or a stress-life curve of provenance `assumed`, N31a-22, N31b-27; with `--review-record`, the uncoded `review-record-packet-mismatch` (N31b-33); with `--firmware-receipt` under the firmware feature, `firmware-receipt-mismatch` (N31a-62), raised meanwhile under its interim 890 with `detail: "firmware-receipt-mismatch"`, which the verb row lists) |
| `foundry.tolerance` | 165 |
| `foundry.export` | 166, 860, 818, 824, 261 (an input geometry whose independent check failed), and 827 with `--with-firmware` (and 169 for a class load set that does not cover every instance); and, from the packet preconditions other beads register, 820 (the interim of `provisional-load-case-in-packet`, N31b-36), 830 and 819 (prototype spend from a `model-conditional` or `tier-sensitive` selection, N31ax-01) and the uncoded `packet-inconsistent` (N31a-43); with `--with-firmware`, the uncoded `firmware-controller-not-exportable` (N31a-44, exit 5) |
| `foundry.lifecycle` | 831 (and 169 for incomplete window traces; 113 under `--strict-data`) |

The parenthesized codes are those a verb passes on from a stage it runs; the list is the same, row for row, as
the refusal lists of the verb rows (VERBS-foundry, which enters them at M4), and a test compares the two. The verb
rows now carry every code above; two kinds of entry are excepted from the comparison: the uncoded names
(`packet-inconsistent`, `review-record-packet-mismatch`, `firmware-controller-not-exportable`, and
`firmware-receipt-mismatch` as a name) while their numbers are held or not yet assigned, and the export door's
codes 731, 733, 734 and 816 on `foundry.export`, which are another chapter's (Table 4) and which the verb row
lists with `exports = true`. An interim code a verb row lists (890 on `foundry.check`, 820 on `foundry.loads`
and `foundry.export`) is compared like any code; when its row's number is assigned, Table 2 and the verb row
change in the same edit as the named constant.

**Table 3: refusals the chapter states without a code.** Eleven, all rows of the decision bead N31b-45: nine
stated by the chapter or by the beads that specify it, a tenth raised by the packet writer N31a-43 (its row
10) and an eleventh, `firmware-controller-not-exportable`, raised by the firmware exporter N31a-44 (its row 11).
Numbers are never typed by a chapter: each is a request in the allocation ledger
`registries/refusal_allocations.toml` (rows 1 to 10 filed by N35x-02 under the rule of N35-33; row 11 filed by
N31b-45 itself). The two design-and-model rows
receive their numbers from the ledger; the evidence-and-protocol rows are `held` under the owner decision on the
full `8xx` family (N35x-01) and are raised meanwhile under an interim code with a `detail` discriminator behind
one named constant, or, where no code fits, the raising bead keeps that point open while its mechanism still
refuses; row 11 is a capability request in the decade `72x` with no interim. No row is ever placed in `9xx`,
which is the internal family with exit 9.

| Name | Refused | Family; ledger status | Interim while held (N31b-45) | Mechanism bead |
|---|---|---|---|---|
| `sized-fill-unsatisfiable` | A body with fill `{shell: sized}` that no wall of its stock card carries | `1xx`; assigned (proposal 170) | FROB-REF-105 with the pointer at the body, until the row loads | N31b-41 |
| `under-built-promotion` | Promotion of a design (or a comparison arm) with a link `under-built` under measured loads | `1xx`; assigned (proposal 171) | none needed once the row loads | N31b-40 |
| `provisional-load-case-in-packet` | A build-packet request that holds an envelope case still labelled `provisional` | `8xx`; held | FROB-REF-820 with the pointer at the case | N31b-36 |
| `review-record-packet-mismatch` | Import of a review record whose `packet` digest is not the manifest digest of the packet named | `8xx`; held | none fits: the point stays open in its bead | N31b-33 |
| `brief-set-not-committed` | A hidden brief opened before the commit record of its set exists | `8xx`; held | FROB-REF-841 with `detail` | N31b-30 |
| `pipeline-not-frozen-or-different` | A hidden brief requested by a binary that is not the frozen one | `8xx`; held | FROB-REF-883 with `detail` | N31b-30 |
| `brief-already-used` | A hidden brief opened a second time | `8xx`; held | FROB-REF-844 with `detail` | N31b-30 |
| `brief-in-development-corpus` | A brief whose digest is already in the development corpus offered for a hidden set | `8xx`; held | FROB-REF-843 with `detail` | N31b-30 |
| `firmware-receipt-mismatch` | An imported timing receipt whose firmware digest, vectors or unit do not match the export | `8xx`; held | FROB-REF-890 with `detail` | N31a-62 |
| `packet-inconsistent` | A packet whose cross-consistency or cross-export identity check fails | `8xx`; held | none fits: the point stays open in its bead | N31a-43 |
| `firmware-controller-not-exportable` | `foundry.export --with-firmware` for a controller whose kernel-IR program leaves the exportable set (a data-dependent loop bound, a division by a non-constant, a transcendental function, a shield with more than 8 rows); payload: the first construct outside the set with its IR name and source position, and the three exportable kinds | `7xx`, decade `72x` (capability, exit 5: the not-implemented class, as N31a-44 states); requested, with no proposal (row 11 of N31b-45, which files the ledger row) | none (`interim = "none"`): N31a-44 refuses with exit 5 and `detail: "firmware-controller-not-exportable"` behind the named constant `FIRMWARE_CONTROLLER_NOT_EXPORTABLE`, the point open in its bead | N31a-44 |

**Table 4: codes of other chapters that Foundry verbs raise** (listed so a caller's handler is complete; not
owned here): FROB-REF-790 (the airlock's structural feature group is absent from this binary; exit 5);
FROB-REF-888 (a metric whose telemetry samples are missing, the general form of 169); the export door's codes
on `foundry.export`, defined in N37-REF and raised by N37-18, with the exit codes N37-REF gives them:
FROB-REF-731 (a private design or an asset it derives from, without the private scope and flag), FROB-REF-733
(the licence set of the artifact forbids the export), FROB-REF-734 (the `--packet` directory or a geometry
output path outside the session's roots) and FROB-REF-816 (the general form of 860 for artifacts other than
packets); FROB-REF-833 (the offline auditor's code for a certificate
bundle whose flags are missing or true); FROB-REF-820 and -830 (label misuse; a `model-conditional`
result offered for promotion, build or prototype budget); FROB-REF-819 (a result cited above its claim product:
on `foundry.export`, a packet requested from a `tier-sensitive` selection); FROB-REF-891 to -893 (format version,
view offered as identity, owner bypass) on `.fbuild`; FROB-REF-734 and -828 (a path outside the session's
roots; a file whose leading bytes are an executable, script, pickle or archive signature, the dispatcher's
sniff) on the rows whose inputs accept a path to a file, where the generator appends them with 891 by its rule
for file parameters and Table 2 does not repeat them: `foundry.check` (`--loads`, `--review-record`,
`--firmware-receipt`, a packet directory), `foundry.optimize` and `foundry.size` (`--loads` given as a file),
`foundry.loads` (`--campaign` given as a specification file) and `foundry.lifecycle` (`--rare-rates`,
`--wear-states`); FROB-REF-113 (a card field of class `assumed` under
`--strict-data`); FROB-REF-869 (a claim whose evidence closure dangles); the interim codes of Table 3.

**Registry rows.** Each of the fourteen codes gets a row in `registries/refusals.toml` with `code`, `name`
(kebab-case: `material-card-incomplete`, `design-region-empty`, `sensitivity-zero-or-band-failed`,
`not-qualified`, `load-case-not-coherent`, `load-telemetry-missing`, `loads-mass-loop-not-converged`,
`independent-check-failed`, `frequency-separation-failed`, `priced-without-catalogue`, `step-bound-or-replay`,
`fixed-point-obligation`, `lifecycle-law-domain`, `flags-missing-or-true`; names are DESIGN), `message`,
`exit = 3`, `retry`, `repairs` (ordered) and `defined_in`.

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the fourteen rows with the names, messages, payload fields, ranked repairs, `exit = 3` and `retry` of Table 1 (860 has no repair and `retry: never`); the loader rejects a duplicate or a reused code.
- [ ] Each row names its binding: the test `ref_<code>_<name>` and the repair-replay step of `scripts/e2e/foundry_refusals.sh`, implemented by N31ap1-51 with the battery. The binding check reports a row `pending: N31ap1-51` until that test exists, and fails at the M5 exit if any of the twelve rows of the default build is still pending; FROB-REF-827 and FROB-REF-831 need a Moonshot feature compiled in (`moonshot-firmware-export`, M8; `moonshot-lifecycle`, M6), so at the M5 exit they are reported `pending: N31a-58` and `pending: N31a-59` on their feature lanes, not failed, and the check fails only if one is still pending at its own lane's milestone exit. Once bound, each code is raised through `frob --json` with exit 3, its payload fields and its first repair.
- [ ] The comparison test of Table 2 against the refusal lists of the seven Foundry verb rows exists; it runs against the rows VERBS-foundry enters (M4, a dependency, so the lists exist and no row is pending) and reports any difference by verb and code, with the two exceptions stated under Table 2 (uncoded names while held or not yet assigned; the export door's 731, 733, 734 and 816 on `foundry.export`, listed in Table 4); the codes the generator appends by kind and by file parameter (891, 734 and 828 among them) are outside the comparison. VERBS-foundry's own `table2_equality` runs the same comparison from the verb side.
- [ ] The eleven refusals of Table 3 each have a ledger request and a registry row with the number the ledger assigned, or the `held` status with the interim of N31b-45 in force (or the point open in its raising bead where no interim fits); `firmware-controller-not-exportable` is row 11 of N31b-45 (a dependency), which files its ledger row in `72x` with `interim = "none"`: the check reads that row like the others and fails if it is absent.
- [ ] No Foundry refusal is emitted with an unregistered code or a bare name (registry-closure test).

## Test plan

### Unit tests
This bead's own tests, `cargo test -p frob-core refusal_registry_foundry` on the real registry (plants are
scratch copies labelled `synthetic`): `fourteen_rows_load` (fields of Table 1, names as listed);
`exit_code_is_3_for_all`; `duplicate_or_reused_code_rejected` (163 duplicated; a retired number re-entered);
`verb_lists_equal_table_2` (against the verb rows VERBS-foundry enters; a planted
extra code on one verb must be reported);
`uncoded_rows_have_ledger_requests` (the eleven of Table 3, each `assigned` or `held` with its interim, and
`firmware-controller-not-exportable` `requested` or `assigned` in `72x` with `interim = "none"`);
`table_1_repairs_complete` (each row's `repairs` holds every repair of Table 1 in its order: 163 three, 164
four, 165 four, 166 three, 167 four, 169 three, 260 three, 261 eight, 262 three, 818 one, 824 three, 827 three,
831 four, 860 none; a planted row of 831 with two repairs fails);
`strict_data_is_113_not_163` (Table 2 lists 113 on `foundry.size`, `foundry.check`, `foundry.optimize` and
`foundry.lifecycle`, and the 163 row's trigger excludes an `assumed` field under `--strict-data`);
`closure_no_unregistered_code_or_bare_name` (a scan of `frob-foundry` and `frob-lifecycle` emit sites);
`every_row_has_a_binding` (pending until N31ap1-51 lands; 827 and 831 pending on N31a-58 and N31a-59 until
their milestones); `export_door_codes_listed_in_table_4` (731, 733, 734 and 816 present in Table 4 and
excepted, not missing, in `verb_lists_equal_table_2`).

The binding tests, specified here and implemented by N31ap1-51 (`cargo test -p frob-foundry refusals`; the
twelve of the default build there, `ref_827` and `ref_831` in N31a-58 and N31a-59 under their features): one test
per code named `ref_<code>_<name>`, each constructing the
smallest input that triggers it and asserting code, exit, pointer and repair order: `ref_163` (card without
`structure.yield_Pa`, the yield field of N31a-22's card section); `ref_164` (keep-out covering the whole envelope); `ref_165` (sensitivity 0); `ref_166`
(design without a verdict; and `--exploratory` accepted and labelled); `ref_167` (a case built from ticks 10
and 11; and a campaign with 0.2% dropped samples); `ref_169` (campaign without control-rate traces);
`ref_260` (planted gain 1.1); `ref_261` (planted overstress at utilization 1.25); `ref_262` (f₁ = 4 × the
loaded frequency); `ref_818`; `ref_824` (300,000 triangles; and a replay with one flipped coordinate bit);
`ref_827` (Q15 product of −1 and −1); `ref_831` (temperature above the card's range); `ref_860` (a manifest
with `fabrication_released: true`, and one with the field absent). `every_code_has_a_fixture`;
`exit_code_is_3_for_all`.

### End-to-end script and logging
This bead's script, `scripts/e2e/foundry_refusal_registry.sh` (sources the STD-E2E helper): (1) `cargo test -p
frob-core refusal_registry_foundry`; (2) `frob schema --refusals --json`, asserting the fourteen rows with exit 3
and the Table 3 requests with their status and interim; (3) `frob schema foundry.<verb> --json` for the seven
verbs, comparing each refusal list with Table 2; (4) negative controls on scratch registries: 163 duplicated
(the load must fail), a reused retired number (must fail), a row in `9xx` with exit 3 (must fail). Logs to
`target/e2e-logs/foundry_refusal_registry/<utc-timestamp>.ndjson` with a transcript: every line has `ts`,
`script`, `bead` (`N31-REF`), `step`, `event`, the exact command line, exit code and wall time; per row the code,
name, exit, binding status (`bound`, `pending: <bead>`); per verb the expected and observed lists and the first
differing code on a mismatch, with the path of the saved schema output. The `summary` counts rows loaded (14),
rows bound, rows pending, uncoded requests by status, and controls caught (3 of 3).

The repair-replay script, specified here and implemented by N31ap1-51, `scripts/e2e/foundry_refusals.sh`: for
each code runs the `frob foundry …` command of its fixture with
`--json`, asserts `ok: false`, the code, exit 3 and the first repair, then applies the first repair where one
exists and is a single command, and asserts the command now succeeds (for 163: adding the field; for 824:
decimation). For 166 it applies the row's second repair, `--exploratory`, and asserts the label is present:
the first, qualification, is a study and not one replayable command. Logs per STD-E2E (`event: refusal` lines with code, pointer, observed,
required, repairs; `event: assert` for the repair replay) to `target/e2e-logs/foundry_refusals/`; the summary
counts codes exercised out of fourteen.

### Known answers and negative controls
Known answers: the trigger numbers of each fixture (0.1% drop limit; ratio 5; 10⁶ instances and 285,712
triangles, the largest even face count under that limit; tick 82; 2 s). Negative controls: a build in which a refusal is downgraded to a warning (the script
must fail); a registry file with code 163 duplicated (the loader must fail); a fixture whose repair does not
clear the refusal (reported as a defect of the repair list).

## Considerations

- 860 overlaps two other flag codes: 816, defined by the safety chapter for artifacts in general, and 833, defined by the certificates chapter for bundles in the offline auditor. Packets report 860 (the chapter governs its own artifact); a reader must treat all three as "foreign or tampered: refuse". Folding them is the registry owner's call.
- 166 has two repairs of very different weight. `--exploratory` is not a way around qualification: its output is labelled and the packet writer rejects it.
- The validity clause of 163 (a curve not valid for the material state, process and finish) is the reason printed parts read `fatigue: no-claim` until stress-life data exists for them (open question OPEN-FOUNDRY-4).
- Codes 827 and 831 belong to Moonshot features and are registered at M5 so the numbers are reserved; their fixtures run only when the features are built.

## References

§31.16, §31.15, Appendix O (O.1, O.2, O.8), §32.4; FROB-REF-163 to -167, -169, -260 to -262, -818, -824, -827, -831, -860; §31.21 to §31.23 for the unnumbered refusals; §35.11 (families; `9xx` internal);
the allocation ledger and its rule (N35-33, N35x-02) and the `8xx` capacity decision (N35x-01).
