---
key: "SPEC-C-19"
title: "spec C-19 (frob-sense, frob-mission): seven instants, causal policies, indistinguishability witnesses"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "solid", "frob-sense", "frob-mission", "frob-conformance", "test"]
parent: "WP-40"
deps: ["BD-025", "N10b-08", "STD-TEST", "STD-E2E"]
needs: ["§10 preamble: the construction register `frob-conformance/registry/constructions.toml`, the ten-section contract template and the shared law-suite convention (bead N10a-01 of unit U10a); this bead fills row C-19", "§10 registries, Appendix M.9 and Appendix N.7: the chapter registry beads N10-KA (unit U10a) and N10-CX (unit U10c) that load rows KA-112, KA-113 and CX-052, CX-053; this bead binds them to tests", "§20.4 and §25.4: the crates `frob-sense` and `frob-mission` are created by beads that implement this construction (BD-078, BD-128) and are expected to depend on this bead; they are deliberately not in `deps`, and those beads bind their real types to the law suite"]
related: ["N10b-09", "N10b-10", "BD-078", "BD-098", "BD-119", "BD-134", "BD-215", "BD-240", "BD-290", "CARD-T-30", "CARD-T-23", "SPEC-C-17", "SPEC-C-20", "N10a-02"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-01"]
plan_refs: ["§10 C-19", "§11.7", "§11.8", "§12.3 T-30", "§20.12", "§20.15", "§25.10", "Appendix D.6", "Appendix I.6", "Appendix M.9 KA-112 KA-113", "Appendix N.7 CX-052 CX-053", "Appendix N.8 CX-105"]
---
## Background

Sensor models are usually a noise level and a delay, and it is commonly believed that more training can
overcome any sensing limit. Construction C-19 replaces both. Every datum that crosses from physics to a
controller passes seven distinct instants; a policy is *causal* when each command depends only on packets that
have already arrived; and two worlds that deliver identical packet histories up to a commitment deadline while
needing different commands form an **indistinguishability witness**: a proof that no policy on that interface
can succeed in both, whatever its training. The payoff is a studio that can say that training cannot help and
what to change instead, that prices delay (a late packet has zero value for a committed decision, and every
held value exposes its age), and that can return "change the body so the information is unnecessary" as an
answer to a sensing problem.

Tier: Solid [S] for the clock contract, causal typing and witness checking (this bead); Frontier [F] for
witness search, the label count and schedule exploration (N10b-09, N10b-10). Owning chapters: Sensorium (crate
`frob-sense`) and Augur (crate `frob-mission`). C-19 defines the observation channel Y and the policy class Π_Y
that the mission query of construction C-17 quantifies over.

Terms. *Packet*: one delivered sensor datum. *Exposure window*: the half-open interval (a, b] over which a
sensor integrates; an event at instant b belongs to the window ending at b. *Commitment deadline*: the last
instant at which a decision can still change. *Total variation* TV(P, Q): half the sum of |P − Q| over
outcomes. *Admitted world*: a member of the declared world set.

Rejected alternatives: a sensor as (noise, delay) pair (it cannot express that the timestamp is a label that
may be wrong, or that a decision may only use what has arrived); ordering same-instant work by arrival (it makes
digests depend on a scheduler); "train longer" as a response to a sensing limit (the witness below shows it
cannot work).

## Scope

In: `docs/math/C-19.md` (core statements S1 to S7; the docking section is supplied by N10b-08 and the label and
schedule section by N10b-09); the law suite `crates/frob-conformance/tests/laws_c19.rs` with reference model and
mutants; registry rows KA-112 and KA-113; controls from CX-052, CX-053 and CX-105; row C-19 of the construction
register.

Out: rational clocks, phase accumulators and packet queues as implemented (BD-078, BD-098); the causal-policy
test as a verb (BD-119, BD-215); witness search (BD-240); `mission.refute` (BD-135, BD-290); the checker for
proof kind `alias-pair` (BD-132); the docking fixture (N10b-08).

Home of the law suite (DESIGN, with reason). The shared convention places a law suite in the owning crate's
test directory. `frob-sense` and `frob-mission` are created by the beads that implement this construction, and
those beads depend on this specification, so the suite cannot wait for them. It lives in `frob-conformance`,
which exists from M0 and may depend on every crate; the implementing beads add their `impl C19Subject` there.

## Technical approach

### 1. The seven instants (statement C-19.S1; class `Tested`)

| # | Instant | Definition |
|---|---|---|
| 1 | sample | The physical quantity begins to be read: emission of a ray, start of an exposure, latch of an encoder |
| 2 | exposure | The end of the integration window that began at the sample instant; equal to it for an instantaneous sensor |
| 3 | timestamp | The time the sensor writes on the packet, on its own clock and by its own convention (start, middle or end of exposure). A label, which may be wrong |
| 4 | arrival | The packet becomes available to its consumer, after transport delay and jitter. It may be out of order or never happen |
| 5 | decision | The tick at which an estimator or controller reads its inputs |
| 6 | command | The command is latched on the bus, after computation delay |
| 7 | actuation | The actuator's effect begins, after driver delay, and holds over an open interval |

Ordering. For one packet: sample ≤ exposure ≤ arrival; the timestamp is unconstrained by causality. A decision
may use only packets whose arrival is not later than the decision. For one command: decision ≤ command ≤
actuation.

Same-instant order (statement C-19.S2; class `Tested`). When several of these coincide at one instant t they
are processed in this fixed order: (1) commit the physical state at t; (2) close the exposures that end at t;
(3) enqueue the samples taken at t; (4) deliver the packets whose arrival is at or before t; (5) evaluate
estimators and controllers; (6) latch commands; a zero-delay command takes effect on the open interval that
follows t.

Rules that follow. A sensor's reading is a functional of the physical history over its window (a, b]. An
out-of-order arrival may change a buffered estimator's future outputs; it never changes a past decision or the
observation that was delivered at that decision. Truth, measurements, belief and policy observations live in
separate namespaces with declared access. A packet is never visible before its arrival, and private truth
behind a disconnected sensor cannot change an action.

### 2. Causal policies (statement C-19.S3; class `Tested`)

A causal policy is a map from the history of arrived packets to commands: the command decided at an instant
depends only on packets whose arrival is not later than that instant, and the content of a packet is the
sensor's functional of the physical history over its exposure window. Π_Y is the set of such maps for channel
Y. Test of causality (noninterference): changing any packet whose arrival is later than a decision, or any
truth the channel does not expose, leaves that decision's command unchanged.

### 3. Indistinguishability witness (statement C-19.S4)

A witness is a pair of admitted worlds w₁, w₂ and a commitment deadline such that (a) under every admitted
command prefix up to the deadline the two worlds deliver identical packet histories, and (b) the sets of
commands that succeed from the deadline on are disjoint. **A witness proves that no policy in Π_Y succeeds in
both worlds.** Argument (on file in the contract): by induction over decisions up to the deadline, identical
histories give identical commands, so the prefix and the command committed at the deadline are the same in
both worlds; by (b) that command fails in at least one. Class: an instance witness is `Certified` when both
worlds are replayed exactly in an integer or rational kernel class (theorem card T-30, proof kind `alias-pair`:
two exact replays with identical packet histories and disjoint success sets), and `Tested` if the worlds run in
floating point.

Stochastic form (statement C-19.S5; class `Certified` on finite distributions by exact enumeration). With
stochastic packets and equal priors, a fixed experiment whose reading has law P in one world and Q in the other
caps success at (1 + TV(P, Q))/2 (the Bayes rule picks the likelier world per reading; its success is
½·Σ max(P, Q) = (1 + TV)/2).

Redundancy (statement C-19.S6; class `Certified`, exact). n independent reads, each correct with probability
q, give a correct majority with probability Σ_{k > n/2} C(n, k)·qᵏ·(1 − q)ⁿ⁻ᵏ. For q = ¾: 27/32 with three
reads, 459/512 with five, 3807/4096 with seven. Reads that share one fault bit stay at ¾ after any number of
reads. Error dependence, not per-read accuracy, sets the value of redundancy.

### 4. The three engineering answers (statement C-19.S7; a reporting rule)

A witness has three engineering answers, compared in one budget: **extract** the information (a better
sensor), **move** to acquire it (a probing action or a mount change), or **change the mechanics** so that the
information is unnecessary. A refutation returned to an agent lists all three with their conditions.

### 5. Type sketch (DESIGN: fixes intent, not signatures)

```rust
pub struct Quanta(pub u64);    // integer count of 1/Q s; Q = lcm of the denominators of every period
pub enum Instant { Sample, Exposure, Timestamp, Arrival, Decision, Command, Actuation }
pub struct Packet { sensor: SensorId, seq: u32, t_sample: Quanta, t_exposure: Quanta,
                    t_stamp: DeviceTime, t_arrival: Quanta, lateness: Quanta, mask: MaskRef, payload: BufRef }
pub struct Command { t_decision: Quanta, t_command: Quanta, t_actuation: Quanta, payload: BufRef }
pub struct Age(pub Quanta);    // decision minus sample; exposed on every observation
```

Integer addition and comparison only: determinism class D0. The timestamp is deliberately unconstrained: it is
what the device believes.

### 6. Honest limits and non-claims (contract sections 7 and 8)

- A witness ends a futile policy search; it does not speed a search that can succeed.
- With bounded probes and positive read error, a nondegenerate interval of hidden gains always contains an
  indistinguishable pair, so an exact guarantee fails and a finite tolerance must be declared.
- A witness is conditional on the admitted world set and the admitted probes; a permitted probe that separates
  the two worlds contradicts it. Fallback when search is unreliable: witnesses are checked when supplied and
  not searched for.

### 7. Foundations (FACT, observed 2026-11-21)

`asupersync` has a deterministic lab runtime (`src/lab/runtime.rs`, `LabRuntime`) on which schedules can be
enumerated. FrankenSim (at `483d09c682f2`) has no packet model. Neither is admitted for this use today.

### 8. Law suite and registry (DESIGN)

- `crates/frob-conformance/tests/laws_c19.rs`, generic over `C19Subject { fn deliver(&self, world, prefix) ->
  Vec<Packet>; fn decide(&self, history: &[Packet], t: Quanta) -> Cmd; fn check_witness(&self, w) -> Verdict }`.
- `tests/support/c19_ref.rs`: a discrete-time packet model in integers (worlds, packets with arrival ticks,
  policies as functions of arrived history); the late-packet docking worlds come from `docking_ref.rs`
  (N10b-08). `tests/support/c19_mutants.rs`: mutants under the cargo feature `controls`.
- Register row C-19: `verb = "sense.causal-test"` (and `sense.alias`, `mission.refute`), `failing_test` =
  `frob-conformance::laws_c19::s3_timestamp_leak_caught`, `payoff.gate = "GATE-SENS-09"`, `cards = ["T-23",
  "T-30"]`, `known_answers = KA-108 … KA-115`, `pilot = "PILOT-C-19"` (for the Frontier parts),
  `generic_comparator = "a scenario-tree linear program; a satisfiability solver for label counts"`.
- Gates that measure the construction (recorded by mechanism beads): GATE-SENS-04 (clock contract, exact
  schedules, zero drift over 10⁹ ticks), GATE-SENS-05 (checkpoint restore at adversarial instants),
  GATE-SENS-12 (causal-policy test: every baseline passes, each of five planted leaks fails), GATE-SENS-09
  (alias witnesses found and checked; no false witness on 64 planted layouts), GATE-AUGUR-03 and GATE-AUGUR-10;
  kill gates KILL-ALIAS (one false witness from the searched lane; fallback: structural, enumerated and
  declared witnesses only), KILL-AUGUR-ALIAS, KILL-SKEL-ALIAS.

## Acceptance criteria

- [ ] `docs/math/C-19.md` has the ten sections, statements C-19.S1 to S7 with classes, the instants table, the
      same-instant order and the witness argument.
- [ ] `cargo test -p frob-conformance --test laws_c19` passes; with `--features controls` every mutant fails.
- [ ] KA-113: 27/32, 459/512, 3807/4096 exactly; the shared-fault-bit model returns ¾ for n = 3, 5, 7, 101.
- [ ] KA-112: for ten seeded pairs of finite distributions the enumerated Bayes success equals (1 + TV)/2 in ℚ.
- [ ] The late-packet docking pair is accepted as a witness; a pair whose packet histories differ at one tick
      is rejected; a pair whose success sets intersect is rejected.
- [ ] The timestamp-leak policy (decides on a packet whose timestamp precedes the decision and whose arrival
      follows it) fails the causality law.
- [ ] `scripts/e2e/spec_c19.sh` exits 0.

## Test plan

### Unit tests

- `s1_ordering_constraints`: 1,000 seeded packets and commands from the reference scheduler satisfy
  sample ≤ exposure ≤ arrival and decision ≤ command ≤ actuation; timestamps are unconstrained (some precede
  the sample, by construction).
- `s2_same_instant_order`: one instant with a state commit, an exposure end, a sample, an arrival, a decision
  and a command: the trace lists them in the fixed order; swapping two independent same-instant events leaves
  the delivered observations unchanged.
- `s3_noninterference`: changing a packet that arrives after decision k leaves command k unchanged, for every
  k; `s3_timestamp_leak_caught`: the leaking policy changes command k.
- `s3_out_of_order_arrival`: a late out-of-order packet changes later outputs and no past decision.
- `s4_witness_docking_late`: the pair g ∈ {1, 2} with the packet late is a witness (identical histories for
  every admitted prefix; success sets {u₁ = 1 − 2p} and {u₁ = ½ − 2p} disjoint).
- `s4_non_witness_rejected`: on-time packet at ε = 1/8, p = 5/8: histories differ, so no witness.
- `s5_tv_cap`, `s6_majority`, `s6_shared_fault_bit`: the acceptance values.
- `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c19.sh` (idempotent; seed 1900 printed): `frob --json doctor`; the law suite with and without
`--features controls`; `cargo test -p frob-conformance --test g1 -- KA-112 KA-113`; `frob --json prove list` and
assert cards T-23 and T-30 are listed with classes no stronger than their routes support; existence of
`docs/math/C-19.md`. Sources the STD-E2E helper; writes `target/e2e-logs/spec_c19/<utc>.ndjson` and a
transcript; each line has `ts`, `script`, `bead` (`SPEC-C-19`), `step`, `event` (`start`, `cmd`, `assert`,
`metric`, `refusal`, `end`), command line, exit code, wall time, evidence block (claim Known answer, class,
tier G1 host, determinism D0, seed), measured and expected rationals, and on failure the first decision whose
command changed or the first tick at which two histories differ. Final `summary`: assertion counts and
GATE-GAUNT-02.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-112 | Equal priors, reading law P or Q: success at most (1 + TV(P, Q))/2 | exact | G0 · host | Registered here (the appendix states none): a cap of ½ + TV, which returns 1 where ¾ is true for the single binary read |
| KA-113 | 27/32, 459/512, 3807/4096; ¾ with a shared fault bit | exact | G1 · host | CX-052: reads that share a fault bit treated as independent |
| KA-111 (via N10b-08) | Late packet: no robust policy | exact (verdict) | G1 · host | CX-053: more training expected to overcome the limit; a pair with different transcripts reported as an alias |
| KA-205 | The same packet is on time when its arrival precedes the second decision and worthless when it follows the third command | exact (verdict) | G1 · host | CX-105: a decision that uses any packet whose timestamp is earlier than the decision |

Metamorphic relations: swapping independent events that share an instant leaves digests unchanged (the control
is an ordering by arrival); re-phasing every clock by a common offset shifts every instant and changes no
decision. Determinism: integer arithmetic, class D0; twice on one machine, debug and release, identical output.

## Considerations

- The witness is a statement about the declared interface. Its strength is exactly its scope: world set,
  channel, deadline. An amendment to any of them re-opens it (construction C-18).
- Kill gates named in advance: KILL-ALIAS and KILL-AUGUR-ALIAS (a generated witness contradicted by a
  permitted probe in more than 5% of audited cases; fallback: witnesses are labelled experiment-specific only
  and universal observability claims are refused).
- Privacy: packet payloads of private scenes never appear in logs; the law suite uses synthetic worlds only.
- Open point carried from the source: the class of the soundness argument of S4 is an argument on file, not a
  Lean card; if a card is added it joins the theorem-card list and this contract's table is updated.

## References

Plan §10 C-19; §11.7, §11.8; §12.3 T-23, T-30; §20.12, §20.15, §25.10; Appendix D.6 (two-world success cap,
majority of repeated reads); Appendix I.6; KA-112, KA-113, KA-205; CX-052, CX-053, CX-105; GATE-SENS-04, -05,
-09, -12; GATE-AUGUR-10; KILL-ALIAS.
