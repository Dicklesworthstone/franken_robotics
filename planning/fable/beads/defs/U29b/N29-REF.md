---
key: "N29-REF"
title: "registries/refusals.toml: eleven Chronicle refusal codes, the fork-graft cross-reference, three uncoded refusals"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "chronicle", "frob-chronicle", "solid", "registry", "test", "review-notes-open"]
parent: "WP-13"
deps: ["BD-005", "BD-022", "N32a-03", "N35x-02", "STD-E2E"]
needs: []
related: ["N24a-38", "N29a-01", "N29b-23", "N29bx-01", "N32-REF", "N35-REF", "N35x-01", "N38-REF"]
gates: []
plan_refs: ["§29.17", "Appendix O", "§29.4", "§29.5", "§29.7", "§29.8", "§29.10", "§29.11", "§29.13", "§29.14", "§29.16", "§29.22", "§32.4", "§32.18"]
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
chapter's codes that the plan leaves unresolved, and three refusals that have no code: one that a late
subsection names in words, and two that Chronicle's mechanism beads state where the plan gives the rule and no
refusal (the abort of the retention audit, and the admission of a scene whose failure monitor this build lacks).

Exit codes, from the verb surface's table: 0 completed; 2 completed with findings; 3 refused input or question
(`1xx` to `5xx`, and `8xx` other than conflicts); 4 budget; 5 capability or scope; 6 backend unavailable; 7
conflict; 8 accepted and still running; 9 indeterminate, also integrity faults. Exit 1 is never emitted on
purpose. The chapter's own table gives no exit codes; the assignments below follow the surface's rule and are
labelled DESIGN where a judgement was needed.

## Scope

In: the eleven registry rows; the cross-reference to FROB-REF-582 and the tombstone of FROB-REF-882; the overlap
table; the three uncoded refusals as requests of the allocation ledger; a registry test; an end-to-end script that triggers every code whose
mechanism exists in the build.

Out: implementing the triggers (each row names the bead that raises it); codes of other chapters.

## Technical approach

**The eleven codes.**

| Code | Refused | Raised when (trigger) | Ranked repairs | Retry | Exit | Raised by |
|---|---|---|---|---|---|---|
| `FROB-REF-811` | A tier-corrected estimate across identities, with no audited address, or below the effective-sample floor | An estimate is requested whose pairs span two model, kernel or battery identities; or whose round has no audited address; or whose effective sample size (Σ wᵢ)²/Σ wᵢ² (wᵢ = 1/pᵢ over audited addresses) is under 30% of the audited count. Write-side form (DESIGN, N29b-08): a round opened with an audit probability below 1/20, after a referee outcome of its schedule exists, or with a prediction map from a later frontier, or a referee outcome inserted for an address its round does not audit; `detail` names the trigger (`map-from-future`, `below-floor`, `outcome-before-assignment`, `unaudited-outcome`). A round replayed with different content is FROB-REF-803, not this code | 1. Referee episodes alone | `after-change` | 3 | N29b-08 (store guard); BD-264 (estimator) |
| `FROB-REF-813` | A deletion that would remove a pinned object | Any delete path (a collection sweep, an explicit removal, a purge) reaches an object with at least one pin row; the refusal lists the holders | 1. Withdraw the claim first | `after-change` | 3 | BD-209; N29b-13 |
| `FROB-REF-815` | Closing a bead without a resolving log line from another session | The bead's log has no line with verdict `pass` whose claim identifiers resolve in the artifact graph and whose recording session differs from the implementing session | 1. Record the gate from a fresh session. 2. If the claim does not resolve, re-run the evidence | `after-change` | 3 | Bead N29a-09 of unit U29a (the closing-rule evaluator) |
| `FROB-REF-869` | Citing a claim whose evidence closure is dangling or tombstoned | A gate row, bead log line, verdict, report or promotion cites a claim for which the closure walk finds a missing node or a tombstoned object | 1. Re-run the evidence; the tombstone names what is missing. 2. Withdraw or restate the claim | `after-change` | 3 | BD-095 (the rule); N29b-10 (every citing surface) |
| `FROB-REF-875` | A publish under a late lease | A chunk publish presents a `lease_gen` that is not the run's current lease generation (the writer was fenced out by a retry, a shard split or a device loss); nothing is written | 1. Re-acquire; replay from the last committed tick | `after-change` | 7 (DESIGN: a conflict with a newer assignment) | BD-091, BD-210 |
| `FROB-REF-878` | Different bytes for a committed chunk key | A publish finds its key already committed with a different content hash. The run is marked `determinism-violated`; serving from the memo graph stops for the node kinds involved; the publish is refused | 1. Stop the run; file the determinism violation with both hashes | `never` | 9 (DESIGN: an integrity fault, like the surface's chunk-conflict code) | BD-091, BD-210 |
| `FROB-REF-885` | A chunk that is not a partition extent, or whose predecessor is not the frontier | The chunk does not cover exactly [i·L, (i + 1)·L) or end at the episode's terminal tick; or its index is not the frontier plus one; or its `prev` is not the chain digest at the frontier | 1. Publish in order on the fixed partition | `after-change` | 3 | BD-091 |
| `FROB-REF-888` | A metric computed over missing samples | A metric, monitor or aggregate is requested over an interval that contains a recorded gap (dropped telemetry, a collected trace, an untraced world), including audit flags marked `unavailable` | 1. Re-run with the channel retained. 2. Accept `unavailable` | `after-change` | 3 | BD-090 (the gap record and the check); every consumer of traces |
| `FROB-REF-894` | A snapshot restored under another numerical profile and labelled a continuation | A replay or fork restores a snapshot under a numerical profile other than the one it was bound to and the request asks for a continuation (bit-level reuse) | 1. Request a comparison replay | `after-change` | 3 | BD-203, BD-204 (the teleport of `sim.replay --tier`), BD-205; the snapshot container bead N29a-18 of unit U29a |
| `FROB-REF-896` | A pagination cursor used with a different pin, filter or order | A page request to the store's paging interface presents a cursor whose pinned commit counter, filters or total order differ from the query's. Under a verb the caller sees the surface code 806 with this code as its cause (`surfaces_as = "FROB-REF-806"`; see the overlap table) | 1. Restart the query | `after-change` | 3 | Bead N29a-04 of unit U29a; BD-211 |
| `FROB-REF-898` | An order-dependent aggregate requested on the evidence path | A channel declaration, metric definition or query asks for an aggregate whose result depends on insertion or merge order (a compactor or digest quantile sketch, a floating-point cross-world sum) where the result would be evidence | 1. Use the integer histogram | `after-change` | 3 | Bead N29a-07 of unit U29a (the aggregate name registry and its check, M1; reached by agents through `telemetry.query --aggregate`, N24a-36); BD-207 (device histograms, M2) |

**Cross-reference and tombstone.** `FROB-REF-582` ("state from one model grafted onto another: a fork that asks
to reuse state its change set invalidates"; ranked repairs: 1. fork at the earliest valid tick returned; 2.
restart and settle) is owned by the counterfactual-debugging chapter and raised by Chronicle's fork mechanism
(seed BD-205). The chapter's table lists it for the reader and does not define it; it is not re-registered here.
`FROB-REF-882` is retired in its favour: the registry keeps a tombstone row for 882 (`status = "retired"`,
`superseded_by = "FROB-REF-582"`), because codes are never reused.

**Neighbour of 894.** The registry row of 894 carries `related = ["FROB-REF-247"]` with one sentence per side,
the same two sentences as the numerics registry N35-REF carries on 247: 247 is admission's code (the determinism
class a verb requires is not attainable on the placement; N35-04, N35-07); 894 is the snapshot container's code
(a snapshot restored under another numerical profile and labelled a continuation; N29a-18), raised by
`sim.replay` (BD-203), its teleport (BD-204) and `run.fork` (BD-205). A restore across profiles is never 247.

**Overlaps the plan leaves open, and their division (DESIGN).** The plan defines each pair in two chapters and
does not divide them. Each is divided by raising site, for one reason: one event must reach a caller under one
code, or agents handle it two ways. The surface registry N32-REF (owner of 806, 807 and 810) states the same
division; a change to one is a change to both. The registry's owner may still merge a pair; until then no code
is retired.

| This chapter | Surface chapter | Same event? | Division |
|---|---|---|---|
| 878, different bytes for a committed chunk key | 810, chunk conflict: different bytes for a committed extent (exit 9; the job fails as indeterminate) | Yes, seen at two layers | Keep both with distinct roles: 878 is the store's refusal to the publishing worker and its record of the conflicting chunk key; 810 is what the job and its caller see, with the store code as its cause (the mechanism beads of unit U29a already do this). The registry row of 878 carries `surfaces_as = "FROB-REF-810"`; both exit 9 with `retry: never` |
| 896, cursor used with a different pin, filter or order | 806, cursor does not match the query (exit 3) | Yes, seen at two layers | Through any verb the envelope's refusal is 806: the surface decodes the cursor before the store is read and refuses a query digest (object, filters, ordering) or schema version that differs from the request's, so a changed filter with an old cursor is 806 for every caller of a verb (the surface's pagination text and its pagination gate name 806 for exactly that case). 896 is the store reader's own check of pin, filter and order: it is what a direct caller of the store's paging interface receives (a library consumer, the conformance worker), and when it fires under a verb the envelope carries 806 with 896 as its cause. The registry row of 896 carries `surfaces_as = "FROB-REF-806"`; both exit 3 |
| (none) | 807, cursor expired: its frontier is no longer retained (exit 3) | Related | Chronicle raises 807 when a cursor's pinned frontier was collected; no new code |

**The uncoded refusals.** Three refusals of the chapter have no code. Each takes its number from the refusal
allocation ledger `registries/refusal_allocations.toml` (N35-33's rule), never from this bead, and this bead
enters each registry row under the assigned number with `request = "<slug>"`, so that the registry check finds
its ledger row. No interim code is used for any of them: none of the eleven codes means any of the three.

*1. `non-physical-energy-bound`.* The late subsection on the solver's kinetic-energy correction names a refusal in words:
*non-physical energy bound to a physical state*. Trigger: a channel binding routes the `algorithmic exchange`
line, the `numerical residual` line, a store or work line, or a stage-level solve energy into a thermal state
(winding, housing, tyre, film) or a wear state. Repairs: 1. bind the state to the constitutive-dissipation or
electrical-loss line of the same element; 2. remove the binding. Retry `after-change`; exit 3; raised by
N29b-22 at run admission. Its number comes from the refusal allocation ledger
`registries/refusal_allocations.toml`, never from this bead: the second allocation round (N35x-02) files it
under the slug `non-physical-energy-bound` in family `1xx` (DESIGN of that bead, accepted here: what is refused
is a binding of the model at run admission, which is the design-and-model family; this chapter's first proposal
of "the next free `8xx`" is withdrawn, and `8xx` has almost no free numbers under the hold of N35x-01). The
allocation function gives it an assigned number (outside `8xx`, so not held). This bead enters the registry row
under that assigned number with the message, the two repairs, retry, exit 3, the fixture and the raiser above,
and `request = "non-physical-energy-bound"` so the registry check finds its ledger row. No interim code is used
(none of the eleven means this); until the row is entered N29b-22 does not close.

*2. `unpinned-claim-closure`* (`defined_in = "§29.16"`; DESIGN of N29b-13: the plan states that a verdict's
evidence closure is pinned in the transaction that writes the verdict, and names no refusal for a store in which
that does not hold). Trigger: the pre-mark audit of `store.collect` (`audit::pins_cover_live_claims`) is not
empty, that is, a live claim's evidence closure holds a stored object that the claim does not pin. A real
collection and a dry run both raise it, before any orphan pin is released, so an aborted cycle changes nothing.
Message: "A live claim's evidence closure holds a stored object that the claim does not pin; nothing was
collected." Payload: the count of (claim, object) pairs and the first 20 pairs, with a handle for the rest.
Ranked repairs: 1. withdraw each listed claim (`frob store collect --withdraw-claim <claim_id> --reason <text>`;
the withdrawal form runs no cycle and is not blocked by the audit) and re-run its evidence, so that the claim is
issued again through the pinning transaction; 2. report the writer that inserted the claim, and leave the store
uncollected until it is fixed. Retry `never`; exit 9; family `9xx` (an integrity fault of the store, not an input
a caller got wrong; exit 9 and retry `never` are that family's rule). Raised by N29b-13. This bead appends its
ledger row, as it does for `failure-monitor-unavailable` below (`requested_by = "N29-REF"`; the second
allocation round, N35x-02, does not file it): `9xx`, no proposal, `interim = none`. Fixture: the test build of N29b-13 in which a claim row is inserted without its pins; until
the registry row is entered N29b-13 does not close.

*3. `failure-monitor-unavailable`* (`defined_in = "§29.10"`; DESIGN of N24a-38: the plan states that a failure
class of the world-and-perception group is reported only when the feature that can produce it is on, and names
no refusal for a build that cannot report it). Trigger: at run admission the monitor set is computed from the
scene pack and the compiled model, and a feature is on whose failure class is `unavailable` in the failure-class
registry (its monitor was delivered and is missing from this build). A class whose row is `pending` (entered
before its monitor exists) does not refuse: the run is admitted and stamped `unmonitored: <class> (pending
<bead>)`. Message: "The scene enables a feature whose failure class {class} has no monitor in this build
({reason}); the run would be a silent world." Payload: `pointer` the feature in the scene or model; `observed`
the class and the reason it is unavailable; `required` a build with the monitor, naming the bead that delivers
it. Ranked repairs (DESIGN, from N24a-38's rule): 1. run on a build that contains the monitor; 2. waive with the
run's explicit waiver flag (N24a-38's), which stamps every result of the run `unmonitored: <class>` and bars it
from scorecards. Retry `after-change`; exit 3; raised by N24a-38 (M3). This bead appends its ledger row
(`requested_by = "N29-REF"`; the ledger is append-only and later requests follow the same rule): family `3xx`
(DESIGN: what is refused is a run on a scene with a feature this build cannot monitor, the family of the
ledger's `scene-pack-not-accepted`; the ledger decides the family by what is refused, and an `8xx` row would be
held under N35x-01 with no code to stand in for it), no proposal, `interim = none`. Fixture: the wet-strip scene
under a scratch registries root in which the `film-slip` row is planted `unavailable` (N24a-38's step
`admission-unavailable`).

**Per-world and worker-facing codes (DESIGN).** 875, 878 and 885 are returned to publishing workers, not to
agents; 896 is returned to direct callers of the store's paging interface, and reaches an agent only as the
cause of the surface's 806. When they occur inside a job they are recorded in the job's log with the world addresses concerned; the
job's own envelope exits by the job protocol's rules (878 ends the run as indeterminate, exit 9; 875 and 885 are
recovered by the worker and do not change the job's exit). The single exit code in the table is what a direct
caller of the publishing interface receives, which keeps "every refusal code maps to exactly one exit code"
true.

**Registry row format.** `code`, `name`, `block`, `defined_in`, `message_template`, `pointer_kind`,
`required_fields`, `repairs[]` (ordered), `retry`, `exit`, `fixture`, `raised_by`, and where applicable
`surfaces_as`, `status`, `superseded_by`, `related` (a neighbouring code with the sentence for each side) and
`request` (the ledger slug of a row beyond the printed codes).

## Acceptance criteria

- [ ] The eleven rows load from `registries/refusals.toml` with complete fields; a duplicate or an unknown code is
      a load-time error (planted).
- [ ] 882 is present as a retired tombstone pointing to 582, and reusing 882 for a new meaning is a load-time
      error.
- [ ] Each of the eleven has a fixture that triggers it through the real binary once its mechanism exists, and the
      envelope (or the worker's reply) carries the code, the repairs in the stated order, the retry class and the
      stated exit code.
- [ ] The division of the three overlaps is recorded in the rows (`surfaces_as` on 878 and on 896) with the
      same text as the surface registry's, and is filed with the registry's owner, who may still merge a pair
      (a merge is recorded as a retirement).
- [ ] A changed filter with an old cursor gives 806 in the envelope of `frob telemetry query` with 896 as its
      cause, and 896 itself from the store's paging interface: one code per caller.
- [ ] The first uncoded refusal is a registry row under the number the allocation ledger assigned to
      `non-physical-energy-bound` (family `1xx`), with message, repairs, retry `after-change`, exit 3, fixture
      and raiser N29b-22; the registry check that every row beyond the printed codes has an assigned ledger row
      passes; this is done before N29b-22 closes.
- [ ] `unpinned-claim-closure` has a ledger row appended by this bead (`requested_by = "N29-REF"`, `9xx`, no
      proposal, `interim = none`) and is a registry row under the number the ledger assigned (family `9xx`), with
      message, the payload of the count and the first 20 (claim, object) pairs, the two repairs in order, retry
      `never`, exit 9, fixture and raiser N29b-13; this is done before N29b-13 closes.
- [ ] `failure-monitor-unavailable` has a ledger row appended by this bead (`requested_by = "N29-REF"`, `3xx`, no
      proposal, `interim = none`) and a registry row under the assigned number with message, payload, the two
      repairs in order, retry `after-change`, exit 3, fixture and raiser N24a-38.
- [ ] The row of 894 carries `related = ["FROB-REF-247"]` and the row of 247 names 894, with one sentence per
      side naming different raising sites (the symmetric-pair check of N35-33).
- [ ] The generated refusal table of the verb surface contains the eleven rows with identical text, and the
      exit-code uniqueness test of the surface passes with them.

## Test plan

### Unit tests

- `chronicle_refusals_registered`: eleven rows; exits {811: 3, 813: 3, 815: 3, 869: 3, 875: 7, 878: 9, 885: 3,
  888: 3, 894: 3, 896: 3, 898: 3}; retry classes as in the table.
- `refusal_repairs_ordered`: the repairs of each row in the order of the table.
- `duplicate_code_is_load_error`, `unknown_code_is_load_error`, `retired_code_cannot_be_reused` (negative
  controls).
- `surfaces_as_resolves`: 878's `surfaces_as` names an existing code with the same exit (810, exit 9), and
  896's does too (806, exit 3).
- `worker_code_does_not_change_job_exit`: a job in which one publish is refused with 885 and then succeeds exits
  0; a job with an 878 exits 9.
- `non_physical_energy_bound_row`: the row's code equals the ledger's assigned number for the slug, its family
  is `1xx`, exit 3; a planted copy of the row under a hand-typed `8xx` number with no ledger row is a load-time
  error (negative control).
- `unpinned_claim_closure_row`: the ledger holds the slug with `requested_by = "N29-REF"`; the row's code
  equals the ledger's assigned number for the slug, its family is
  `9xx`, exit 9, retry `never`, two repairs with "withdraw each listed claim" first; a planted copy with exit 3
  fails the exit rule of its family (negative control).
- `failure_monitor_unavailable_row`: the ledger holds the slug with `requested_by = "N29-REF"`; the row's code
  equals the assigned number, exit 3, retry `after-change`, two repairs with "run on a build that contains the
  monitor" first.
- `related_894_247_symmetric`: 894's `related` names 247 and 247's names 894; the two sentences name different
  raising sites (admission; the snapshot container); a planted copy of the 894 row without `related` fails the
  registry's symmetric-pair check.

### End-to-end script and logging

`scripts/e2e/chronicle_refusals.sh` triggers each code with the smallest fixture: 811, `frob eval promote <r>
--method dr --key <k> --json` on a round with no audited address; 813, `frob store collect --class <c> --force
--key <k> --json` on a scratch store whose class holds a pinned chunk (the whole call is refused and names the
pins); 815, a bead close recorded by the implementing session (the closing rule's dry run, N29a-09); 869, a
report citing a claim whose evidence chunk was tombstoned in a scratch store (`frob report run <r> --cite
<claim> --json`); 875, the conformance worker publishing with a stale lease generation; 878, the same worker
publishing a committed key with one flipped payload byte; 885, a publish at index frontier + 2 and one
misaligned by a tick; 888, `frob telemetry query <run> --aggregate sum` over a window with a recorded gap; 894,
`frob sim replay --run <run> --world <w> --from-snapshot <hash> --tier batch-cpu --precision f64 --key <k>
--json` without `--comparison` (a snapshot of the recorded profile restored under another profile as a
continuation: exit 3), then the same with `--comparison` (admitted: a tolerance report), the repair the row
names; 896, the
conformance worker asking the store's paging interface for the second page of a query with a changed filter
(896 itself), and the same request through `frob telemetry query` (the envelope's refusal is 806 with 896 as
its cause); 898, `frob telemetry query <run> --aggregate tdigest` (the admitted alternative `histogram` named in
the refusal) and a run manifest that declares a compactor quantile sketch on an evidence channel; `non-physical-energy-bound`, `frob sim run --tier referee` on the resting-block fixture
whose thermal test node is bound to the algorithmic-exchange line (exit 3 before any buffer is allocated);
`unpinned-claim-closure`, which no input to a release build produces (the raw claim writer is private to the
retention module): the step reads the result of N29b-13's test `collect_aborts_on_unpinned_closure` (the test
build with the handle exposed) and asserts from it exit 9, retry `never`, count 1 and the pair, for a real
collection and for a dry run; `failure-monitor-unavailable`, `frob --registries-root <scratch dir> sim run
<wet-strip design> --scene <wet-strip scene> --tier referee --key <k> --json`, the scratch root a copy of the
registries in which the `film-slip` row is planted `unavailable` (N24a-38's step: exit 3 naming `film-slip`). For
each it asserts code, repairs, retry class and exit code. NDJSON at
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
  start; fixtures for 811 (third milestone) and for 813, 894 and the device paths of 875, 878, 885 and 898
  (second milestone) are `not-run` until then. 898 is raised at the first milestone by the aggregate registry's
  check (N29a-07) on the query and manifest paths, so its fixture runs from M1.
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
§29.8 (894, 582), §29.10 (`failure-monitor-unavailable`), §29.11 (811), §29.13 (869), §29.14 (815), §29.16 (813,
`unpinned-claim-closure`), §29.22 (`non-physical-energy-bound`), §32.4
and §32.18 (exit codes; 806, 807, 810).
