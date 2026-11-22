---
key: "CARD-T-14"
title: "Theorem card T-14: an exported fixed-point controller has no overflow on its stated input range"
type: "task"
priority: 4
milestone: "M8"
labels: ["m8", "lemma", "frob-lemma", "frob-foundry", "moonshot"]
parent: "WP-80"
deps: ["BD-380", "N12-06", "N12-02"]
needs: ["§31.13: the firmware exporter, its range analysis and Q-format assignment, which emit the overflow queries (BD-438)", "§21.7: the reserved fixed-point types q16_16 and q32_32 of the kernel IR (BD-018)"]
related: ["BD-438", "N12-07", "CARD-T-09"]
gates: []
plan_refs: ["§12.3 (T-14)", "§12.1 (bit-vector decisions)", "§31.13", "§31.16 (PO-FOUNDRY-04)", "§42 OPEN-FOUNDRY-6", "Appendix C (firmware export)"]
---
## Background

Firmware export is a Moonshot: joint-level controllers and monitors (a proportional–derivative or
proportional–integral loop, the energy-budget monitor, a shield projection with a few rows) are emitted as
source for a heap-free crate in fixed-point arithmetic, so that the controller evaluated in simulation is,
integer for integer, the arithmetic that would be shipped. Fixed-point arithmetic overflows silently. The
exporter's checks, in order, are: range analysis (declared input ranges propagated through the kernel IR by
interval arithmetic, fixing a Q format for every intermediate); bit-vector queries where intervals are too
coarse; inductive range invariants for state; quantization bounds; test vectors.

Statement (T-14). An exported fixed-point controller has no overflow on its stated input range: for each
exported function and each intermediate value, no input within the declared ranges makes that intermediate
exceed its Q format. For stateful controllers (integrators, filters) the statement is inductive: the state range
invariant holds initially (base case) and is preserved by one step (step case).

Route: FrankenLean's bit-vector decision with checked proofs (`Engine::decide_bv`, exposed as
`fln-verdict::bv_decide`). Where intervals are too coarse (correlated terms such as a·x − a·y), the question "is
there an input in range for which this intermediate overflows?" is bit-blasted and decided; an unsatisfiable
answer with a replayed proof stream is an instance certificate, class `Certified`. It is not `Proved`, because
the proof-term step is not built upstream. Tier: Moonshot. A full equivalence of the firmware lowering with its
kernel is not decidable here (it needs a verified compiler and a model of the target): what stands in for it is
this overflow bound per function, and otherwise parity tests.

## Scope

In: the card's statement and registry row; the certificate format for an overflow query and its replay; the
time-budget rule; the counterexample fixtures. Out: the exporter, range analysis and quantization (the firmware
epic); the 10⁶ test vectors (a test, not called equivalence); execution time, stack depth and behaviour on a
board (not claimed: `timing: no-claim` until a receipt from real hardware is imported).

## Technical approach

1. Query per intermediate v with Q format (word length w, fractional bits): variables are the inputs as
   bit-vectors constrained to their declared ranges; the formula asserts that the exact two's-complement
   evaluation of v at extended width differs from its w-bit truncation (overflow). `bv_decide` returns
   unsatisfiable with a proof stream, or a model (a concrete overflowing input).
2. Inductive invariant for state s with claimed range [lo, hi]: base query (the initial state is in range) and
   step query (s in range and inputs in range imply the next state in range and no intermediate overflows).
3. Certificate (DESIGN): per query, the bit-vector formula's digest, the function and intermediate it is about
   (bound to the exported kernel's content hash), the declared ranges, the verdict, and the proof stream's
   digest; the proof stream is replayed by `fln` (a subprocess on the Linux host). In a bundle it travels as a
   `lean-capsule`-style reference to the replayable artifact with claim class `Certified`; the auditor checks the
   hashes and the statement match and names the FrankenLean kernel in the trusted base. (If the registry later
   gains a dedicated bit-vector kind, the certificate moves to it by a version bump.)
4. Time budget: OPEN-FOUNDRY-6 sets the default at 2 s per obligation (TARGET); a timeout is undecided and is
   therefore refused: the function is not exported with a certificate, and the exporter must widen a format or
   add a clamp.
5. A satisfiable answer is a counterexample: the model is stored as a test vector and the export is refused for
   that function until the format or the range is changed.
6. Class and limits: `Certified` per function and range; never `Proved`. It bounds overflow, nothing else.
7. Relies on it: the firmware export epic; proof obligation PO-FOUNDRY-04 (for each exported controller, no
   intermediate overflows on the declared input ranges; replayed bit-vector certificates).
8. Artifacts: `docs/math/cards/T-14.md`; registry row (status `parked` until the epic is funded); fixtures.

## Acceptance criteria

- [ ] While the firmware epic is unfunded the registry row is `parked` and prints no class.
- [ ] On the funded branch: for a proportional–integral loop with declared input ranges, every intermediate has
      an unsatisfiable overflow query with a proof stream that `fln` replays; the certificates are bound to the
      exported kernel's hash.
- [ ] The three seeded counterexamples are found as satisfiable queries: in Q15, (−1)·(−1) is
      0x8000·0x8000 = 0x40000000, which shifts to 0x8000 and reads as −1; the negation of −32768 overflows; an
      integrator adding 400 counts per tick from zero overflows sixteen bits at tick 82, and without a clamp the
      one-step query returns the state 32,400 at once.
- [ ] A query that exceeds the time budget is reported undecided and the export is refused.
- [ ] Negative controls: a proof stream with one step altered fails replay; a certificate bound to another
      kernel hash is rejected; a range silently widened after certification invalidates the certificate.

## Test plan

### Unit tests

`crates/frob-lemma/tests/cards/t14.rs`: `q15_minus_one_squared_overflows`; `q15_negate_min_overflows`;
`integrator_overflows_at_tick_82` (400·82 = 32,800 > 32,767; 400·81 = 32,400 fits);
`integrator_one_step_query_returns_32400`; `clamped_integrator_invariant_holds` (base and step queries
unsatisfiable with a clamp at ±32,000); `correlated_terms_need_bitvectors` (a·x − a·y with x, y in a narrow
common range: interval analysis reports a possible overflow that the bit-vector query refutes);
`timeout_is_undecided_refused`; `altered_proof_stream_fails_replay`.

### End-to-end script and logging

The firmware epic's export script drives this card: it logs, per function, the number of queries, the slowest
query against the 2 s budget, undecided counts (threshold 0 for export), and the certificate digests.

### Known answers and negative controls

The three counterexamples above are the known answers and the seeds of the negative-control registry for this
card. Determinism: bit-vector decisions are exact; proof streams replay identically on the pinned checker.

## Considerations

- Kill gate of the owning epic: KILL-FIRMWARE, whose fallback is to export parameters, Q formats and the
  test-vector manifest only (no certificates, no generated code claims).
- The exported artifact carries the no-authorization flags; the generated code never addresses a register, a bus
  socket or an emergency-stop relay.
- No fixed-point type is used by any kernel through M5; the reserved words exist so that this target lowers the
  same programs without a second IR.

## References

§12.1, §12.3, §31.13; PO-FOUNDRY-04; OPEN-FOUNDRY-6; GATE-FOUNDRY-11; KILL-FIRMWARE.
