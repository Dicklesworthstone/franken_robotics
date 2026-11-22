---
key: "N29-REF"
title: "registries/refusals.toml: eleven Chronicle refusal codes, the fork-graft cross-reference, one uncoded refusal"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "chronicle", "frob-chronicle", "solid", "registry", "test"]
parent: "WP-13"
deps: ["BD-005", "BD-022", "STD-E2E"]
needs: ["§32.4: the exit-code table and the envelope's `refusal` block (`code`, `message`, `pointer`, `observed`, `required`, `unit`, `affected_claims`, `retry`, `repairs`), and the test that every refusal code maps to exactly one exit code (GATE-TETHER-04)", "§32.18: the surface chapter's codes FROB-REF-806, -807 and -810, which overlap three codes of this chapter; the registry's owner must rule on the overlap (see the table of overlaps)", "§27.14: FROB-REF-582, owned by the counterfactual-debugging chapter and raised by Chronicle's fork mechanism"]
related: ["N29-CX", "BD-091", "BD-095", "BD-203", "BD-205", "BD-207", "BD-209", "BD-210", "BD-211", "BD-264", "N29b-08", "N29b-10", "N29b-13", "N29b-22", "N24a-36", "N29a-01", "N29a-04", "N29a-07", "N29a-09", "N29a-18", "N38-REF"]
gates: []
plan_refs: ["§29.17", "Appendix O", "§29.4", "§29.5", "§29.7", "§29.8", "§29.11", "§29.13", "§29.14", "§29.16", "§29.22", "§32.4", "§32.18"]
---
## Background

A refusal is the system declining to answer, by name. Every refusal has a stable code `FROB-REF-nnn`, a message
and ranked repairs; the machine-readable registry is `registries/refusals.toml`; an unknown code is a load-time
error; codes are never reused. The hundreds digit is the block (`8xx`: evidence and protocol). The chapter that
defines a code governs its meaning and repairs.

The record subsystem (crate `frob-chronicle`) defines eleven codes, all in the `8xx` block. Its two governing
sentences explain why they exist: Chronicle refuses to let a claim be printed that it cannot trace, and it
refuses to compute a metric from samples it does not have. The other codes protect the record itself: one
content hash per chunk key, a contiguous frontier, pinned evidence, deterministic aggregates.

This bead registers the eleven codes with triggers, ranked repairs, retry class, exit code and a fixture each,
so that the mechanism beads raise codes that already exist and the surface tests can enumerate them. It also
records the cross-reference to the fork-graft code owned by another chapter, three overlaps with the surface
chapter's codes that the plan leaves unresolved, and one refusal that a late subsection names in words without
assigning a code.

Exit codes, from the verb surface's table: 0 completed; 2 completed with findings; 3 refused input or question
(`1xx` to `5xx`, and `8xx` other than conflicts); 4 budget; 5 capability or scope; 6 backend unavailable; 7
conflict; 8 accepted and still running; 9 indeterminate, also integrity faults. Exit 1 is never emitted on
purpose. The chapter's own table gives no exit codes; the assignments below follow the surface's rule and are
labelled DESIGN where a judgement was needed.

## Scope

In: the eleven registry rows; the cross-reference to FROB-REF-582 and the tombstone of FROB-REF-882; the overlap
table; the uncoded refusal with a proposal; a registry test; an end-to-end script that triggers every code whose
mechanism exists in the build.

Out: implementing the triggers (each row names the bead that raises it); codes of other chapters.

## Technical approach

**The eleven codes.**

| Code | Refused | Raised when (trigger) | Ranked repairs | Retry | Exit | Raised by |
|---|---|---|---|---|---|---|
| `FROB-REF-811` | A tier-corrected estimate across identities, with no audited address, or below the effective-sample floor | An estimate is requested whose pairs span two model, kernel or battery identities; or whose round has no audited address; or whose effective sample size (Σ wᵢ)²/Σ wᵢ² (wᵢ = 1/pᵢ over audited addresses) is under 30% of the audited count | 1. Referee episodes alone | `after-change` | 3 | N29b-08 (store guard); BD-264 (estimator) |
| `FROB-REF-813` | A deletion that would remove a pinned object | Any delete path (a collection sweep, an explicit removal, a purge) reaches an object with at least one pin row; the refusal lists the holders | 1. Withdraw the claim first | `after-change` | 3 | BD-209; N29b-13 |
| `FROB-REF-815` | Closing a bead without a resolving log line from another session | The bead's log has no line with verdict `pass` whose claim identifiers resolve in the artifact graph and whose recording session differs from the implementing session | 1. Record the gate from a fresh session. 2. If the claim does not resolve, re-run the evidence | `after-change` | 3 | Bead N29a-09 of unit U29a (the closing-rule evaluator) |
| `FROB-REF-869` | Citing a claim whose evidence closure is dangling or tombstoned | A gate row, bead log line, verdict, report or promotion cites a claim for which the closure walk finds a missing node or a tombstoned object | 1. Re-run the evidence; the tombstone names what is missing. 2. Withdraw or restate the claim | `after-change` | 3 | BD-095 (the rule); N29b-10 (every citing surface) |
| `FROB-REF-875` | A publish under a late lease | A chunk publish presents a `lease_gen` that is not the run's current lease generation (the writer was fenced out by a retry, a shard split or a device loss); nothing is written | 1. Re-acquire; replay from the last committed tick | `after-change` | 7 (DESIGN: a conflict with a newer assignment) | BD-091, BD-210 |
| `FROB-REF-878` | Different bytes for a committed chunk key | A publish finds its key already committed with a different content hash. The run is marked `determinism-violated`; serving from the memo graph stops for the node kinds involved; the publish is refused | 1. Stop the run; file the determinism violation with both hashes | `never` | 9 (DESIGN: an integrity fault, like the surface's chunk-conflict code) | BD-091, BD-210 |
| `FROB-REF-885` | A chunk that is not a partition extent, or whose predecessor is not the frontier | The chunk does not cover exactly [i·L, (i + 1)·L) or end at the episode's terminal tick; or its index is not the frontier plus one; or its `prev` is not the chain digest at the frontier | 1. Publish in order on the fixed partition | `after-change` | 3 | BD-091 |
| `FROB-REF-888` | A metric computed over missing samples | A metric, monitor or aggregate is requested over an interval that contains a recorded gap (dropped telemetry, a collected trace, an untraced world), including audit flags marked `unavailable` | 1. Re-run with the channel retained. 2. Accept `unavailable` | `after-change` | 3 | BD-090 (the gap record and the check); every consumer of traces |
| `FROB-REF-894` | A snapshot restored under another numerical profile and labelled a continuation | A replay or fork restores a snapshot under a numerical profile other than the one it was bound to and the request asks for a continuation (bit-level reuse) | 1. Request a comparison replay | `after-change` | 3 | BD-203, BD-205; the snapshot container bead N29a-18 of unit U29a |
| `FROB-REF-896` | A pagination cursor used with a different pin, filter or order | A page request presents a cursor whose pinned commit counter, filters or total order differ from the query's | 1. Restart the query | `after-change` | 3 | Bead N29a-04 of unit U29a; BD-211 |
| `FROB-REF-898` | An order-dependent aggregate requested on the evidence path | A channel declaration, metric definition or query asks for an aggregate whose result depends on insertion or merge order (a compactor or digest quantile sketch, a floating-point cross-world sum) where the result would be evidence | 1. Use the integer histogram | `after-change` | 3 | Bead N29a-07 of unit U29a; BD-207 |

**Cross-reference and tombstone.** `FROB-REF-582` ("state from one model grafted onto another: a fork that asks
to reuse state its change set invalidates"; ranked repairs: 1. fork at the earliest valid tick returned; 2.
restart and settle) is owned by the counterfactual-debugging chapter and raised by Chronicle's fork mechanism
(seed BD-205). The chapter's table lists it for the reader and does not define it; it is not re-registered here.
`FROB-REF-882` is retired in its favour: the registry keeps a tombstone row for 882 (`status = "retired"`,
`superseded_by = "FROB-REF-582"`), because codes are never reused.

**Overlaps the plan leaves open (to be ruled on by the registry's owner; proposals are DESIGN).**

| This chapter | Surface chapter | Same event? | Proposal |
|---|---|---|---|
| 878, different bytes for a committed chunk key | 810, chunk conflict: different bytes for a committed extent (exit 9; the job fails as indeterminate) | Yes, seen at two layers | Keep both with distinct roles: 878 is the store's refusal to the publishing worker; 810 is what the job's client sees, with the store code as its cause (the mechanism beads of unit U29a already do this). The registry row of 878 carries `surfaces_as = "FROB-REF-810"`; both exit 9 |
| 896, cursor used with a different pin, filter or order | 806, cursor does not match the query (exit 3) | Nearly | 806 for a cursor that does not belong to the verb or query at all (schema-level, before the store is read); 896 for the store's check of pin, filter and order. If the owner prefers one code, 896 is kept and 806 retired with a tombstone |
| (none) | 807, cursor expired: its frontier is no longer retained (exit 3) | Related | Chronicle raises 807 when a cursor's pinned frontier was collected; no new code |

**The uncoded refusal.** The late subsection on the solver's kinetic-energy correction names a refusal in words:
*non-physical energy bound to a physical state*. Trigger: a channel binding routes the `algorithmic exchange`
line, the `numerical residual` line, a store or work line, or a stage-level solve energy into a thermal state
(winding, housing, tyre, film) or a wear state. Repairs: 1. bind the state to the constitutive-dissipation or
electrical-loss line of the same element; 2. remove the binding. Retry `after-change`; exit 3; raised by
N29b-22 at run admission. Block: it is a model-admission refusal raised by Chronicle; the proposal is the next
free code of the `8xx` block. Until the registry's owner assigns a code the mechanism bead stays open on this
point, and no interim code is used (none of the eleven means this).

**Per-world and worker-facing codes (DESIGN).** 875, 878 and 885 are returned to publishing workers, not to
agents. When they occur inside a job they are recorded in the job's log with the world addresses concerned; the
job's own envelope exits by the job protocol's rules (878 ends the run as indeterminate, exit 9; 875 and 885 are
recovered by the worker and do not change the job's exit). The single exit code in the table is what a direct
caller of the publishing interface receives, which keeps "every refusal code maps to exactly one exit code"
true.

**Registry row format.** `code`, `name`, `block`, `defined_in`, `message_template`, `pointer_kind`,
`required_fields`, `repairs[]` (ordered), `retry`, `exit`, `fixture`, `raised_by`, and where applicable
`surfaces_as`, `status`, `superseded_by`.

## Acceptance criteria

- [ ] The eleven rows load from `registries/refusals.toml` with complete fields; a duplicate or an unknown code is
      a load-time error (planted).
- [ ] 882 is present as a retired tombstone pointing to 582, and reusing 882 for a new meaning is a load-time
      error.
- [ ] Each of the eleven has a fixture that triggers it through the real binary once its mechanism exists, and the
      envelope (or the worker's reply) carries the code, the repairs in the stated order, the retry class and the
      stated exit code.
- [ ] The three overlaps are filed with the registry's owner and the ruling is recorded in the rows
      (`surfaces_as`, or a retirement).
- [ ] The uncoded refusal has a code recorded here before N29b-22 closes.
- [ ] The generated refusal table of the verb surface contains the eleven rows with identical text, and the
      exit-code uniqueness test of the surface passes with them.

## Test plan

### Unit tests

- `chronicle_refusals_registered`: eleven rows; exits {811: 3, 813: 3, 815: 3, 869: 3, 875: 7, 878: 9, 885: 3,
  888: 3, 894: 3, 896: 3, 898: 3}; retry classes as in the table.
- `refusal_repairs_ordered`: the repairs of each row in the order of the table.
- `duplicate_code_is_load_error`, `unknown_code_is_load_error`, `retired_code_cannot_be_reused` (negative
  controls).
- `surfaces_as_resolves`: 878's `surfaces_as` names an existing code with the same exit.
- `worker_code_does_not_change_job_exit`: a job in which one publish is refused with 885 and then succeeds exits
  0; a job with an 878 exits 9.

### End-to-end script and logging

`scripts/e2e/chronicle_refusals.sh` triggers each code with the smallest fixture: 811, `frob eval promote
--method dr` on a round with no audited address; 813, `frob store collect` asked to remove a pinned chunk; 815, a
bead close recorded by the implementing session; 869, a report citing a claim whose evidence chunk was
tombstoned in a scratch store; 875, the conformance worker publishing with a stale lease generation; 878, the
same worker publishing a committed key with one flipped payload byte; 885, a publish at index frontier + 2 and
one misaligned by a tick; 888, `frob telemetry query` asking for a mean over an interval with a recorded gap;
894, `frob sim replay` of a snapshot under another numerical profile with `--as continuation`; 896, the second
page of a query requested with a changed filter; 898, a run manifest that declares a compactor quantile sketch
on an evidence channel. For each it asserts code, repairs, retry class and exit code. NDJSON at
`target/e2e-logs/chronicle_refusals/<utc-timestamp>.ndjson` with a transcript: per code a `cmd` line (command,
exit code, wall time), a `refusal` line (the whole refusal block and the evidence block), an `assert` line per
field; codes whose mechanism is not built at the current milestone are `not-run` with the milestone at which
they fall due; a `summary` line with counts.

### Known answers and negative controls

The planted duplicate, unknown and reused codes. For each code the control is the build or input in which the
limit was removed: a late lease that writes (875), a conflicting publish that overwrites (878), overlapping
ranges that commit (885), a metric interpolated across a gap (888), a claim printed after its evidence was
collected (869), a deletion of pinned evidence (813), audit bits chosen after the outcomes (811), a query
ordered by timestamp alone that accepts any cursor (896), an order-dependent sketch on the evidence path (898),
a bead closed by its implementer (815), a continuation across numerical profiles (894). Each is registered
against the gate that measures its mechanism.

## Considerations

- Milestones: the codes are registered at the first milestone so that every mechanism has its code from the
  start; fixtures for 811 (third milestone) and for 813, 894, 898 and the device paths of 875, 878, 885 (second
  milestone) are `not-run` until then.
- The exit codes of 875 and 878 are judgements; the surface's rule ("`8xx` other than conflicts exit 3") leaves
  conflicts and integrity faults to exits 7 and 9, and a stale lease and a hash conflict are those two things.
- Leaving 878 and 810 as unexplained twins would make agents handle one event two ways; the `surfaces_as` field
  is the cheapest fix that keeps both chapters' text true.
- 888 is the most widely raised code of the chapter: every trace consumer in other subsystems raises it rather
  than estimate from neighbours.
- RISK-54 (the plan and the registries drift apart; early warnings: an identifier defined twice, a threshold
  stated two ways) has three instances in this chapter's refusals: the two overlapping pairs and the refusal
  named without a code. The countermeasure is that the registry file is the authority and the printed tables are
  generated from it; this bead is where the three are filed so that the generated table has one answer.

## References

Plan §29.17 (the chapter's refusal table), Appendix O.8, §29.4 (875, 878, 885, 896), §29.5 (888), §29.7 (898),
§29.8 (894, 582), §29.11 (811), §29.13 (869), §29.14 (815), §29.16 (813), §29.22 (the uncoded refusal), §32.4
and §32.18 (exit codes; 806, 807, 810).
