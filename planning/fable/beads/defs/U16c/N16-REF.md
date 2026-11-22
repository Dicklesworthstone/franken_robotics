---
key: "N16-REF"
title: "registries/refusals.toml: the nine Kinesis refusal codes and the uncoded refusals of the late subsections"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "kinesis", "frob-dyn", "solid", "registry", "test"]
parent: "WP-10"
deps: ["BD-005", "BD-022"]
needs: ["§32.4: the exit-code table and the envelope's `refusal` block (`code`, `message`, `pointer`, `observed`, `required`, `unit`, `affected_claims`, `retry`, `repairs`), and the test that every refusal code maps to exactly one exit code (GATE-TETHER-04)"]
related: ["N16-CX", "BD-083", "N16c-01", "BD-081", "BD-082", "BD-159", "BD-263", "BD-299"]
gates: []
plan_refs: ["§16.14", "Appendix O", "§16.3", "§16.4", "§16.5", "§16.6", "§16.8", "§16.19", "§16.21", "§16.22", "§16.23", "§16.24", "§32.4"]
---
## Background

A refusal is the system declining to return a number, by name. Every refusal has a stable code `FROB-REF-nnn`, a message and ranked repairs; the machine-readable registry is `registries/refusals.toml`, an unknown code is a load-time error, and codes are never reused. The hundreds digit is the block: `2xx` numerics and integrator admission, `7xx` budget and capability, `8xx` evidence and protocol. The chapter that defines a code governs its meaning and repairs.

Kinesis (dynamics on three tiers: the referee, `batch-cpu` and the GPU tiers) defines nine codes. They are the visible form of several rules: a transactional step either commits a checked interval or leaves no trace; capacity overflow is flagged or refused and never a silent truncation; a result is cited only under the label it earned. This bead registers the nine codes with triggers, repairs, exit codes and a fixture each, so that the mechanism beads raise codes that already exist and the surface tests can enumerate them. It also records the refusals that the six late subsections of the chapter name in words without assigning a code, so that they are not lost.

Exit codes (from the verb surface's table): 3 = refused input or question (`1xx` to `5xx`, and `8xx` other than conflicts); 5 = capability not implemented, not admitted or outside the token's scopes (`72x`, `73x`); 4 = budget; 2 = completed with findings or flags. Exit 1 is never emitted on purpose.

## Scope

In: the nine registry rows; the cross-reference to `FROB-REF-212` (owned by the algebra chapter, raised by Kinesis's integrators); proposals for the uncoded refusals with the block each belongs to; a registry test and an end-to-end script that triggers each code that exists in the build. Out: implementing the triggers (each row names the bead that raises it); codes of other chapters.

## Technical approach

**The nine codes.**

| Code | Name | Raised when | Ranked repairs | Retry | Exit | Raised by |
|---|---|---|---|---|---|---|
| `FROB-REF-220` | Event refinement limit | Under a transactional profile an event cannot be bracketed within L_max = 16 dyadic levels of its base interval (31 ns at a 2 ms interval), or more than E_max = 64 events fall in one base interval (an accumulation point of impacts). The transaction ends; no approximation is returned | 1. Report the accumulation (the event sequence and bracket are in the reply). 2. Switch the contact law. 3. Shorten h | `after-change` | 3 | BD-082 |
| `FROB-REF-221` | Transaction exhausted | Every numerical plan of the retry ladder (attempts 0 to 3) failed its checks. The world is `Faulted` with cause `numerical-fault`; nothing from the interval is published; the world is excluded from physical failure classes and counted as excluded | 1. Teleport the world to `batch-cpu` and inspect. 2. Accept the exclusion (the summary states it) | `never` | 3 | BD-083 |
| `FROB-REF-222` | Profile not available on tier | `--profile strict` or `--profile referee` requested on `gpu-cuda` or `gpu-metal`. A profile is a promise about what the step did, and a GPU cannot keep this one without giving up fixed shapes | 1. Run on `batch-cpu` | `after-change` | 3 | the step-profile admission of `sim.run` |
| `FROB-REF-223` | No refinement law | A stochastic driver that acts over an interval has no registered refinement law under a transactional profile | 1. Register the law. 2. Move the draw to a sample instant | `after-change` | 3 | N16c-01 |
| `FROB-REF-224` | Indeterminate contact | An event configuration has no solution, or several distinct post-event motions (the Painlevé rod is the fixture). Redundant contacts with one motion are not refused: the transaction reports observable motion, total impulse and work, never a pressure distribution | 1. Report the configuration. 2. Do not return a number | `never` | 3 | BD-082 |
| `FROB-REF-721` | Referee capacity | An island exceeds the referee's contact cap (21 contacts in FrankenSim's sustained solve at the pinned commit), or FrankenSim's fixed-witness gate refuses. Never a truncated contact set | 1. A documented patch reduction with its qualification fixture. 2. `--profile strict` as the named adjudicator | `after-change` | 5 | BD-080, BD-081 |
| `FROB-REF-722` | Capacity overflow, refused | On a lockstep tier a world exceeds its contact capacity C or row capacity R and the caller's overflow policy is `refuse` (the other policies are `reset` and `continue-flagged`, which mark the world `Degraded`) | 1. Raise the capacity (a new `StepPlanId`). 2. Reset. 3. Continue flagged | `after-change` | 5 | BD-159 |
| `FROB-REF-820` | Label misuse | A result whose label is not `established` (`provisional`, `provisional: tier-sensitive`, `model-conditional`, `energy-created`, `capacity-biased`) is cited as established, or a `model-conditional` ranking is used to authorize a build packet or prototype spend | 1. Promote the result. 2. Run the settling experiment named in `next_actions` | `after-change` | 3 | BD-263, BD-266 |
| `FROB-REF-821` | Envelope not frozen | Promotion is requested for a quantity whose fidelity envelope is not hashed in the ledger | 1. Freeze the envelope first | `after-change` | 3 | BD-263 |

Cross-reference: `FROB-REF-212` (Cayley bound: ‖ω‖·h > 0.2 in a substep; repairs: substep, use the exponential) is an existing code of the algebra chapter that Kinesis's integrators raise; it is not re-registered here.

**Per-world codes inside a multi-world run (DESIGN).** Codes 220, 221, 224 and 722 can concern one world of thousands. The per-world outcome record carries the code; the envelope of the run exits 2 (completed with flags) and lists counts and addresses per code. The envelope itself is `ok: false` with exit 3 or 5 only when the request as a whole is refused (admission-time 222, 223, 821; a citation refused by 820; a single-world run; or a caller policy of `refuse` on first overflow). This keeps "every refusal code maps to exactly one exit code" true for envelope refusals.

**Refusals named without a code in the late subsections.** Each needs a code from the registry owner before its mechanism bead closes; until then the mechanism raises the nearest existing code listed here, with the message below.

| Named in | Refusal (words of the chapter) | Trigger | Repairs | Block; interim code |
|---|---|---|---|---|
| Row-class solve queues | "execution option not yet qualified on this backend" | A run made with `--solve-queues` carries the label `experimental-execution` until GATE-KIN-08 has passed on that backend, and is offered as promotion evidence | Re-run on the masked path; qualify the backend | `8xx`; interim `FROB-REF-820` (label misuse) |
| Step adjoints | The adjoint job "is refused when it does not fit" | The step-buffer store for K control intervals exceeds the device budget (160 world-steps × bytes per world × worlds) | Fewer worlds; shorter horizon; the paired stencil | `6xx`/`7xx`; interim `FROB-REF-602` (memory estimate exceeds the device budget at admission) |
| Step adjoints | A binary outcome as the metric | Non-smooth metric | Change the metric; rank-based search | existing `FROB-REF-540` (gradient unreliable) |
| Loop elimination | "a compile that cannot tell the branches apart at the reference pose refuses" | Both roots of the four-bar loop equation coincide within tolerance at the reference pose | Move the reference pose off the change point; declare the branch | `1xx` (design and model); no existing code fits, so the mechanism bead stays open until one is assigned |
| Ancestor-sparse assembly | A study that would mix the two solve paths "is refused with the two step-plan identities" | A paired study, promotion or comparison whose arms ran under `--solve-path dense` and `--solve-path ancestor` | Re-run one arm; or declare the comparison unpaired | `5xx` (evaluation protocol); interim `FROB-REF-561` (a paired statistic requested with arms that cannot be paired) |
| Held command | "refinement study that changes a clock" | A step-refinement study whose arms differ in any clock of the clock contract (control period, sensor period, latency) as well as in h | Hold every clock fixed in seconds and change only h; or submit it as a comparison of two controllers | `5xx`; no existing code fits, so the mechanism bead stays open until one is assigned |
| Decoupled stepping | A fleet scene beyond the coupled capacity | The fallback after KILL-INTERACT: one coupled world under `strict` exceeds its capacity | Patch reduction; fewer robots | existing `FROB-REF-721`, `FROB-REF-722` |

**Registry row format.** `code`, `name`, `block`, `defined_in`, `message_template`, `pointer_kind`, `required_fields`, `repairs[]` (ordered), `retry`, `exit`, `fixture`, `raised_by`.

## Acceptance criteria

- [ ] The nine rows load from `registries/refusals.toml`; a duplicate or unknown code is a load-time error (planted).
- [ ] Each of the nine has a fixture that triggers it through `frob … --json` once its mechanism exists, and the envelope carries the code, the repairs in the stated order, the `retry` class and the stated exit code.
- [ ] `FROB-REF-222` is returned for `--profile strict` and `--profile referee` on each GPU tier before any buffer is allocated.
- [ ] `FROB-REF-721` and `FROB-REF-722` never coincide with a truncated contact set: the fixture's contact count in telemetry equals the detected count.
- [ ] The seven uncoded refusals are filed with the registry owner; each has either a new code recorded here or the interim code in force, and no mechanism bead closes on an interim code without that record.
- [ ] The generated refusal table of the verb surface contains the nine rows with identical text.

## Test plan

### Unit tests

- `kinesis_refusals_registered`: nine rows, fields complete, exits {220..224: 3, 721: 5, 722: 5, 820: 3, 821: 3}.
- `refusal_repairs_ordered`: the repairs of each row in the order of the table.
- `duplicate_code_is_load_error`, `unknown_code_is_load_error` (negative controls).
- `per_world_code_does_not_change_envelope_exit`: a 64-world run with one world raising 722 under `continue-flagged` exits 2; with policy `refuse` exits 5.

### End-to-end script and logging

`scripts/e2e/kinesis_refusals.sh` triggers each code with the smallest fixture: 220, a ball with restitution bouncing to rest inside one base interval; 221, an island with inconsistent ties; 222, `frob sim run --tier gpu-cuda --profile strict`; 223, an unregistered Brownian driver; 224, the Painlevé rod; 721, a 24-contact hexapod stance on the referee; 722, a tracked fixture at C = 16 with `--on-overflow refuse`; 820, `frob report … --cite-as established` on a provisional result; 821, `frob eval promote` on an unfrozen quantity. For each it asserts code, repairs, retry class and exit code. NDJSON at `target/e2e-logs/kinesis_refusals/<utc-timestamp>.ndjson` with a transcript: per code a `cmd` line (command, exit code, wall time), a `refusal` line (the whole refusal block, the evidence block), an `assert` line per field; codes whose mechanism is not built are `not-run`; a `summary` line with counts.

### Known answers and negative controls

The planted duplicate and unknown codes; a planted silent truncation at overflow (the contact set clipped to capacity without a flag) must be caught by the 721/722 fixtures (INV-KIN-05); CX-151 (the referee cannot adjudicate everything: 16 fits, 24 does not).

## Considerations

- The contact cap 21 is a FACT about FrankenSim at the pinned commit and is re-checked by an admission test before a gate cites it; the upstream work item for a larger admitted solve changes the trigger's number, not the code.
- `FROB-REF-820` is raised wherever a citation is made (reports, build packets, promotion); Kinesis defines it, and the evidence chapter's claim product enforces it.
- Assigning real codes to the seven uncoded refusals is an owner-of-registry action; leaving them on interim codes forever would blur what `FROB-REF-820` means.

## References

Plan §16.14 (the chapter's refusal table), Appendix O.2, O.7, O.8, §16.3 (222), §16.4 (721), §16.5 (220, 221, 223, 224), §16.6 (722), §16.8 (820, 821), §32.4 (exit codes); INV-KIN-05.
