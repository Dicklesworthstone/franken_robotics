---
key: "N16-REF"
title: "registries/refusals.toml: the nine Kinesis refusal codes and the uncoded refusals of the late subsections"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "kinesis", "frob-dyn", "solid", "registry", "test", "review-notes-open"]
parent: "WP-10"
deps: ["BD-005", "BD-022", "N32a-03", "N35-33", "STD-E2E"]
needs: []
related: ["N21-REF", "N21b-23", "N33a-13", "N33a-14", "N16b-03", "N16b-04", "N16b-23", "N16b-24", "N16b-25", "N16b-27", "N16b-35", "N16bx-02", "N16bx-04", "N16c-01", "N16c-05", "N16c-15", "N16c-20", "N16c-26", "N16c-30", "N16c-31", "N16c-32", "N16c-38", "N16c-40", "N16c-43", "N16c-44", "N17-REF"]
gates: []
plan_refs: ["§16.14", "Appendix O", "§16.3", "§16.4", "§16.5", "§16.6", "§16.8", "§16.19", "§16.21", "§16.22", "§16.23", "§16.24", "§32.4"]
---
## Background

A refusal is the system declining to return a number, by name. Every refusal has a stable code `FROB-REF-nnn`, a message and ranked repairs; the machine-readable registry is `registries/refusals.toml`, an unknown code is a load-time error, and codes are never reused. The hundreds digit is the block: `2xx` numerics and integrator admission, `7xx` budget and capability, `8xx` evidence and protocol. The chapter that defines a code governs its meaning and repairs.

Kinesis (dynamics on three tiers: the referee, `batch-cpu` and the GPU tiers) defines nine codes. They are the visible form of several rules: a transactional step either commits a checked interval or leaves no trace; capacity overflow is flagged or refused and never a silent truncation; a result is cited only under the label it earned. This bead registers the nine codes with triggers, repairs, exit codes and a fixture each, so that the mechanism beads raise codes that already exist and the surface tests can enumerate them. It also records the refusals that the six late subsections of the chapter name in words without assigning a code, so that they are not lost.

Exit codes (from the verb surface's table): 3 = refused input or question (`1xx` to `5xx`, and `8xx` other than conflicts); 5 = capability not implemented, not admitted or outside the token's scopes (`72x`, `73x`); 6 = device (`6xx`); 4 = budget; 2 = completed with findings or flags. Exit 1 is never emitted on purpose.

## Scope

In: the nine registry rows; the cross-reference to `FROB-REF-212` (owned by the algebra chapter, raised by Kinesis's integrators); proposals for the uncoded refusals with the block each belongs to; a registry test and an end-to-end script that triggers each code that exists in the build. Out: implementing the triggers (each row names the bead that raises it); codes of other chapters.

## Technical approach

**The nine codes.**

| Code | Name | Raised when | Ranked repairs | Retry | Exit | Raised by |
|---|---|---|---|---|---|---|
| `FROB-REF-220` | Event refinement limit | Under a transactional profile an event cannot be bracketed within L_max = 16 dyadic levels of its base interval (31 ns at a 2 ms interval), or more than E_max = 64 events fall in one base interval (an accumulation point of impacts). The transaction ends; no approximation is returned | 1. Report the accumulation (the event sequence and bracket are in the reply). 2. Switch the contact law. 3. Shorten h | `after-change` | 3 | BD-082 |
| `FROB-REF-221` | Transaction exhausted | Every numerical plan of the retry ladder (attempts 0 to 3) failed its checks. The world is `Faulted` with cause `numerical-fault`; nothing from the interval is published; the world is excluded from physical failure classes and counted as excluded | 1. Teleport the world to `batch-cpu` and inspect. 2. Accept the exclusion (the summary states it) | `never` | 3 | BD-083 |
| `FROB-REF-222` | Profile not available on tier | `--profile strict` or `--profile referee` requested on `gpu-cuda` or `gpu-metal`. A profile is a promise about what the step did, and a GPU cannot keep this one without giving up fixed shapes | 1. Run on `batch-cpu` | `after-change` | 3 | the profile-by-tier admission matrix of plan admission (N16a-02, M1), used by every verb that creates an environment; `sim.run`'s tier and profile options (N16b-25, M2) |
| `FROB-REF-223` | No refinement law | A stochastic driver that acts over an interval has no registered refinement law under a transactional profile | 1. Register the law. 2. Move the draw to a sample instant | `after-change` | 3 | N16c-01 |
| `FROB-REF-224` | Indeterminate contact | An event configuration has no solution, or several distinct post-event motions (the Painlevé rod is the fixture). Redundant contacts with one motion are not refused: the transaction reports observable motion, total impulse and work, never a pressure distribution | 1. Report the configuration. 2. Do not return a number | `never` | 3 | BD-082 (our coupled solve); N33a-13 at M0 (conversion of FrankenSim's no-solution or several-solutions error) |
| `FROB-REF-721` | Referee capacity | An island exceeds the referee's contact cap (21 contacts in FrankenSim's sustained solve at the pinned commit), or FrankenSim's fixed-witness gate refuses. Never a truncated contact set | 1. A documented patch reduction with its qualification fixture. 2. `--profile strict` as the named adjudicator (payload `kind = "fixed-witness"`: 1. a shorter interval, so that witnesses are fixed inside it by witness migration between calls; 2. `--profile strict`) | `after-change` | 5 | N33a-14 at M0 (the static and run-time capacity checks), N33a-13 at M0 (conversion of the sibling's capacity and fixed-witness refusals); surfaced through `sim.run` by BD-080, BD-081 |
| `FROB-REF-722` | Capacity overflow, refused | On `batch-cpu` (the host lockstep tier) a world exceeds its contact capacity C or row capacity R and the caller's overflow policy is `refuse` (the other policies are `reset` and `continue-flagged`, which mark the world `Degraded`). On `gpu-cuda` and `gpu-metal` the same trigger raises `FROB-REF-610` instead (below) | 1. Raise the capacity (a new `StepPlanId`). 2. Reset. 3. Continue flagged | `after-change` | 5 | BD-159 (N16b-25 for `sim.run`) |
| `FROB-REF-820` | Label misuse | A result whose label is not `established` (`provisional`, `provisional: tier-sensitive`, `model-conditional`, `energy-created`, `capacity-biased`) is cited as established, or a `model-conditional` ranking is used to authorize a build packet or prototype spend | 1. Promote the result. 2. Run the settling experiment named in `next_actions` | `after-change` | 3 | BD-263, BD-266 |
| `FROB-REF-821` | Envelope not frozen | Promotion is requested for a quantity whose fidelity envelope is not hashed in the ledger | 1. Freeze the envelope first | `after-change` | 3 | BD-263 |

**Pointer and payload of 721 and 224 (DESIGN; the conversion of N33a-13 fills exactly these).** 721: `pointer`
`referee/island/<island index>`; payload `kind` = `capacity` (with `observed` the island's contact count,
`required` the cap read from `ContactCapacities.max_contacts_per_solve`, `unit` = `contacts`) or `fixed-witness`
(no `observed`/`required`); the repair list is the one for its `kind` as in the row above; a refusal converted from
FrankenSim also carries the `upstream` object (crate, variant, message verbatim, pin). 224: `pointer`
`referee/event/<event index>`; payload `configuration` (the bodies and contacts of the event and the count of
distinct candidate motions, 0 for no solution); no `observed`/`required`.

**Capacity overflow on device tiers: `FROB-REF-610` (cross-reference).** `FROB-REF-610` "Capacity overflow under
the `refuse` policy" is a code of the device chapter (N21-REF, §21.21; exit 6, the device family; repairs: 1. a
larger capacity; 2. `continue-flagged`). It and 722 describe one trigger split by tier: one raising function with
one `pointer` (the capacity) and one needed count (N21b-23, test `overflow_code_by_tier`) raises 722 on
`batch-cpu` and 610 on `gpu-cuda` and `gpu-metal`. The two exit codes differ (5 against 6) because the families
differ; the registry rows cross-reference each other (`see_also`) so that an agent sees one repair list, and
`sim.run`'s refusal list carries both. KA-349's CUDA and Metal rows (every forced overflow is flagged `Degraded` or
refused, never dropped) are met by 610; its `batch-cpu` row by 722. DESIGN: both codes stay registered as the plan
defines them (§16.14 and §21.21); retiring one is a registry-owner decision that would change both rows and
`sim.run`'s list in one change.

Cross-references, not re-registered here: `FROB-REF-212` (Cayley bound: ‖ω‖·h > 0.2 in a substep; repairs: substep, use the exponential) is an existing code of the algebra chapter that Kinesis's integrators raise. `FROB-REF-263` (a derivative requested near grazing, at an ambiguous event order, across a change of active set or at frictional degeneracy, construction C-16; the reply carries the offending state and directions; exit 3) is the code Kinesis raises when a derivative through a step is requested where none exists (the events-and-limits rule of the chapter: fixed-mode derivatives on smooth kernels only) and when an adjoint request cannot be served under any label (N16c-25, N16c-26).

**Per-world codes inside a multi-world run (DESIGN).** Codes 220, 221, 224 and 722 can concern one world of thousands. The per-world outcome record carries the code; the envelope of the run exits 2 (completed with flags) and lists counts and addresses per code. The envelope itself is `ok: false` with exit 3 or 5 only when the request as a whole is refused (admission-time 222, 223, 821; a citation refused by 820; a single-world run; or a caller policy of `refuse` on first overflow). This keeps "every refusal code maps to exactly one exit code" true for envelope refusals.

**Refusals named without a code in the late subsections.** Each is a row of the refusal allocation ledger
`registries/refusal_allocations.toml` (the Kinesis rows of N35-33's first round), under the slug below (DESIGN
slugs, stable keys for tests). A request whose condition an existing code already states is `merged` (the
ledger's rule 1): it gets no number, and its raising bead joins that code's `raised_by`. The three requests outside
`8xx` are numbered by the first-round run at M0, before their raising beads land (M2 to M6): this registry enters
each as a full row under its assigned number, and the raising bead raises that number behind one named
constant, with the message below. The two `8xx` requests (`solve-queues-unqualified`, `ancestor-path-after-kill`)
are `held` (N35-33 rule 8 and the hold of N35x-01): their raising beads raise the interim code listed, with
`detail` equal to the slug, behind one named constant. For the three assigned rows the last column records the
ledger's `interim` field, the earlier stand-in that lapses at the run.

| Named in | Ledger slug; raising bead | Refusal (words of the chapter) | Trigger | Repairs | Block; interim code |
|---|---|---|---|---|---|
| Row-class solve queues | `solve-queues-unqualified`; N16c-15 | "execution option not yet qualified on this backend" | A run made with `--solve-queues` carries the label `experimental-execution` until GATE-KIN-08 has passed on that backend, and is offered as promotion evidence | Re-run on the masked path; qualify the backend | `8xx`; interim `FROB-REF-820` (label misuse) |
| Step adjoints | `adjoint-store-over-budget` (merged into 602); N16c-26 | The adjoint job "is refused when it does not fit" | The step-buffer store for K control intervals exceeds the device budget at admission (K·d world-steps × bytes per world × worlds, d being the world-steps per control interval; 160 world-steps at K = 16, d = 10, so 4.32 MB per quadruped world at 27 kB) | Fewer worlds; shorter horizon; the paired stencil | existing `FROB-REF-602` (merged; `pointer` the slab class `adjoint`, N21-REF), raised by N16c-26 |
| Step adjoints | `adjoint-metric-not-smooth` (merged into 540); N16c-26 | A binary outcome as the metric | Non-smooth metric | Change the metric; rank-based search | existing `FROB-REF-540` (gradient unreliable, `pointer: metric`; Phylum's row, N17-REF, lists N16c-26 among its raisers) |
| Loop elimination | `loop-elimination-inadmissible`; N16c-30 (compile), N16c-32 (admission and variants; the four-bar kernels of N16c-31 rely on the variant guard and raise nothing) | "a compile that cannot tell the branches apart at the reference pose refuses"; the same refusal at admission and for a variant | Three triggers, told apart by `stage` and `pointer`: (a) at compile (N16c-30, `stage: compile`): both roots of the four-bar loop equation coincide within tolerance at the reference pose (it is at a change point), or neither root equals the reference pose's φ₀ (the design does not close); `observed` both roots and φ₀. (b) At environment admission (N16c-32, `stage: admission`): the position kernel evaluated at the reference pose's θ₀ on the tier that will run it returns dependent coordinates that differ from the compiler's reference pose by more than 10⁻⁹ rad in `f64` or 10⁻⁴ rad in `f32` (the kernel and the model disagree about the branch); `observed` both values; refused before any step, with no world-step counter moved. (c) At admission of a per-world parameter variant (N16c-32, `stage: variant`): the variant's link lengths, re-tested by the compiler's toggle test and ratio bound (N16c-30) over the declared joint range, bring the sine of the transmission angle μ below 0.17 (|cos μ| above 0.98544) or break the ratio bound; message "variant moves an eliminated loop inside its toggle guard", `pointer` the variant and the closure, `observed` the smallest sin μ over the range, `required` 0.17; refused before any world exists | (a) 1. Move the reference pose off the change point; 2. declare the branch. (b) 1. Recompile the model; 2. if it persists, keep the loop as a row (`--closures rows`). (c) 1. Change the variant's link lengths or narrow the joint range; 2. run the family with `--closures rows` | `1xx` (design and model), exit 3, retry `after-change`; interim none (no existing code fits; N16c-30 and N16c-32 each raise the assigned number behind one named constant) |
| Ancestor-sparse assembly | `solve-path-mixed-study`; N16c-38 | A study that would mix the two solve paths "is refused with the two step-plan identities" | A paired study, promotion or comparison whose arms ran under `--solve-path dense` and `--solve-path ancestor` | Re-run one arm; or declare the comparison unpaired | `5xx` (evaluation protocol); interim `FROB-REF-561` (a paired statistic requested with arms that cannot be paired) |
| Ancestor-sparse assembly, after KILL-ANCESTOR (DESIGN, from the kill bead N16c-40) | `ancestor-path-after-kill`; N16c-38 (the verb), N16c-40 (the decision that withdraws it) | The ancestor path requested where the kill left only the dense path | `--solve-path ancestor` on a model of 18 to about 60 coordinates after KILL-ANCESTOR has fired (the path stays admitted above that range, where the tree-operator route's own gate decides) | `--solve-path dense`, or omit the option | `8xx` (argument not admitted for this model); interim `FROB-REF-802` (arguments fail the schema) with the coordinate count and the admitted range in `observed` |
| Held command | `refinement-study-changes-clock`; N16c-43 | "refinement study that changes a clock" | A step-refinement study whose arms differ in any clock of the clock contract (control period, sensor period, latency) as well as in h, or in any other physical parameter of ADR-KIN-05 (clocks are physical parameters, so it is one rule and one code; `observed` names what differs) | Hold every clock fixed in seconds and change only h; or submit it as a comparison of two controllers | `5xx`; interim none (no existing code fits) |
| Decoupled stepping | `decoupled-fleet-over-capacity` (merged into 721 and 722); N16c-20 | A fleet scene beyond the coupled capacity | The fallback after KILL-INTERACT: one coupled world under `strict` exceeds its capacity | Patch reduction; fewer robots | existing `FROB-REF-721` (referee), `FROB-REF-722` (`batch-cpu`); N16c-20 joins both rows' `raised_by` |

**Registry row format.** `code`, `name`, `block`, `defined_in`, `message_template`, `pointer_kind`, `required_fields`, `repairs[]` (ordered; per payload `kind` where the row has kinds), `retry`, `exit`, `fixture`, `raised_by` (the bead keys of the "Raised by" column, the field N35-33's registry-wide check reads), `see_also` (codes that describe the same trigger on another tier).

**Why M0.** The registry is data and is entered before the mechanisms: the conversion of FrankenSim refusals
(N33a-13) and the capacity checks of `RefereeContact` (N33a-14) raise 721 and 224 at M0, and an unknown code is a
load-time error, so these rows must load at M0. The other seven rows are entered at the same time; their raisers
arrive at M1 to M3, and their fixtures are `not-run` until then.

## Acceptance criteria

- [ ] The nine rows load from `registries/refusals.toml` at M0 (721 and 224 with the pointer and payload above, as N33a-13's conversion needs); a duplicate or unknown code is a load-time error (planted).
- [ ] Each of the nine has a fixture that triggers it through `frob … --json` once its mechanism exists, and the envelope carries the code, the repairs in the stated order, the `retry` class and the stated exit code.
- [ ] `FROB-REF-222` is returned for `--profile strict` and `--profile referee` on each GPU tier before any buffer is allocated.
- [ ] `FROB-REF-721` and `FROB-REF-722` never coincide with a truncated contact set: the fixture's contact count in telemetry equals the detected count.
- [ ] The eight uncoded refusals are rows of the allocation ledger under the slugs of the table: the three whose condition an existing code states (`adjoint-store-over-budget` into 602, `adjoint-metric-not-smooth` into 540, `decoupled-fleet-over-capacity` into 721 and 722) are `merged`, with their beads added to those rows' `raised_by`; the three outside `8xx` are `assigned` and entered here as full rows under their numbers; the two `8xx` rows are `held` and raised on their interim code (820, 802) with `detail` equal to the slug.
- [ ] The loop-elimination row carries its three stages (`compile`, `admission`, `variant`) with the payload and repairs of each, and N16c-30 and N16c-32 raise the same ledger row.
- [ ] The generated refusal table of the verb surface contains the nine rows with identical text.

## Test plan

### Unit tests

- `kinesis_refusals_registered`: nine rows, fields complete, exits {220..224: 3, 721: 5, 722: 5, 820: 3, 821: 3}.
- `refusal_repairs_ordered`: the repairs of each row in the order of the table.
- `duplicate_code_is_load_error`, `unknown_code_is_load_error` (negative controls).
- `per_world_code_does_not_change_envelope_exit`: a 64-world run with one world raising 722 under `continue-flagged` exits 2; with policy `refuse` exits 5.
- `overflow_rows_cross_reference`: the 722 row's `see_also` names `FROB-REF-610` and the 610 row (N21-REF) names 722; the exits are 5 and 6.
- `uncoded_rows_in_ledger`: each of the eight slugs is a ledger row with `requested_by = "N16-REF"`; the three merged slugs have no number and their beads appear in the `raised_by` of 602 and of 540 (N16c-26) and of 721 and 722 (N16c-20), and 602's row (N21-REF) lists the slab class `adjoint` in its `pointer`; the three outside `8xx` have an assigned number with a registry row of that number and slug; the two `8xx` slugs are `held` with interims 820 and 802.
- `loop_elimination_stages`: the row admits `stage` values `compile`, `admission` and `variant`, each with its own ordered repairs; the `variant` stage has `observed` (smallest sin μ) and `required` = 0.17. Run with N16c-30 and N16c-32 (M2): the parallelogram with range 20° to 160° and a variant that shortens b from 2 to 1.2 is refused at `stage: variant` before any world exists.
- `referee_721_payload_by_kind`: the 721 row admits payload kinds `capacity` (with `observed`, `required`, unit `contacts`) and `fixed-witness` (without them), each with its own ordered repairs; the 224 row has no `observed`/`required`.

### End-to-end script and logging

`scripts/e2e/kinesis_refusals.sh` triggers each code with the smallest fixture: 220, a ball with restitution bouncing to rest inside one base interval; 221, the tied-slider island with a declared impulse tolerance of 10⁻²⁰ N·s that no rung can meet in `f64` (inconsistent ties would be 224, not 221); 222, `frob sim run --tier gpu-cuda --profile strict`; 223, an unregistered Brownian driver; 224, the Painlevé rod; 721, a 24-contact hexapod stance on the referee; 722, a tracked fixture at C = 16 with `--on-overflow refuse`; 820, `frob report … --cite-as established` on a provisional result; 821, `frob eval promote` on an unfrozen quantity. For each it asserts code, repairs, retry class and exit code. NDJSON at `target/e2e-logs/kinesis_refusals/<utc-timestamp>.ndjson` with a transcript: per code a `cmd` line (command, exit code, wall time), a `refusal` line (the whole refusal block, the evidence block), an `assert` line per field; codes whose mechanism is not built are `not-run`; a `summary` line with counts.

### Known answers and negative controls

The planted duplicate and unknown codes; a planted silent truncation at overflow (the contact set clipped to capacity without a flag) must be caught by the 721/722 fixtures (INV-KIN-05); CX-151 (the referee cannot adjudicate everything: 16 fits, 24 does not).

## Considerations

- The contact cap 21 is a FACT about FrankenSim at the pinned commit and is re-checked by an admission test before a gate cites it; the upstream work item for a larger admitted solve changes the trigger's number, not the code.
- `FROB-REF-820` is raised wherever a citation is made (reports, build packets, promotion); Kinesis defines it, and the evidence chapter's claim product enforces it.
- Numbers for the five uncoded refusals that need one are an owner-of-registry action (N35-33's first round), never typed in a chapter bead. The two `8xx` ones stay on 820 and 802 while the owner decision N35x-01 holds the family; the `detail` equal to the slug, and the interim list that `frob --json schema` prints, keep those uses from blurring what `FROB-REF-820` and `FROB-REF-802` mean.

## References

Plan §16.14 (the chapter's refusal table), Appendix O.2, O.7, O.8, §16.3 (222), §16.4 (721), §16.5 (220, 221, 223, 224), §16.6 (722), §16.8 (820, 821), §32.4 (exit codes); INV-KIN-05.
