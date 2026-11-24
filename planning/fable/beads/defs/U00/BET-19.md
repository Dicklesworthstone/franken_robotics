---
key: "BET-19"
title: "Decide BET-19: mission queries over a design family beat point-by-point evaluation and survive amendment"
type: "task"
priority: 1
milestone: "M4"
labels: ["m4", "augur", "frob-mission", "frontier", "bet", "gate"]
parent: "WP-66"
deps: ["BD-136", "BD-140", "BD-147", "BD-317", "BD-319", "BD-355", "N00-28", "N25a-18", "N25a-28", "N25b-03", "N25b-04", "N25c-13", "N25c-20", "N38a-09", "STD-E2E"]
needs: []
related: ["BD-354", "BET-20", "N00-27", "N39b-39", "N40-31", "N40-32", "NS-13", "NS-22"]
gates: ["GATE-AUGUR-01", "GATE-AUGUR-02", "GATE-AUGUR-03", "GATE-AUGUR-04", "GATE-AUGUR-05", "GATE-AUGUR-06", "GATE-AUGUR-07", "GATE-AUGUR-15", "GATE-TENFOLD-06", "KILL-AUGUR-TIE", "KILL-AUGUR-FALSECERT"]
plan_refs: ["§4 (BET-19)", "§40.5", "§2.4 (lever 5)", "§2.6", "§25.27", "§43.4", "§43.12"]
---
## Background

**The bet.** Today an agent evaluates candidate bodies one at a time and reports the best score; nothing says
whether an untried body would also work, or why a failed one cannot. A mission query asks the question over a
family: given a mission, a design family, a controller class and what the robot may observe and when, return an
*inner set* (designs that provably can do it, each with a witness), an *outer set* (everything not excluded,
with impossibility witnesses for the excluded part), the unresolved cells between, and the cheapest next
experiment. The bet is that this reaches an accepted answer sooner than point-by-point evaluation and that the
answer survives amendment (when the mission changes, the system withdraws exactly what it must). Tier:
Frontier.

**What the evidence supports.** On small plants, standard exact methods tie a specialized solver: in the
plan-time pilots a lifted robust linear program, a projection, a satisfiability solver and quantifier
elimination each returned the same complete family answer on every fixture, and a specialized solver measured
only 1.345× warm and 1.088× cold against a composed one under a ceiling of about 1.46×. So the gain, if any, is
in scope and reuse across amendments, not in solve speed; expected ratio 1 to 3 per query warm, possibly below
1 cold.

## Scope

In: pre-registration; readings at M1 (the CPU slice), M3 (the envelope stratum of the partial gate) and M4
(scale) of one question: do mission queries over a family reach a witnessed answer that a matched reducer does
not reach at comparable cost, and are the accepted answers sound. Also in: this bet's verdict and its decision
records.

Out, and where it is:

- What a tie on the family strata means for the engine thesis: BET-14. Both bets read the one decision
  KILL-AUGUR-TIE; this bead decides only the consequence for mission queries (the conventional program ships
  behind the same verbs, schema and checker).
- Region certificates on continuous families (BET-20).
- The scenarios seen from the user's side (NS-13 for the stance query, NS-22 for the docking fixture).
- The mission engine, checkers, bundles and fixtures (BD-129 to BD-136 at M1; BD-355 at M4); the matched
  reducers used as competing arms (N25c-20, N25c-13).
- Applying the triggers of KILL-AUGUR-TIE (N25a-18) and KILL-AUGUR-FALSECERT (N25b-04) and writing their ADRs.
  This bead reads those decisions.

## Technical approach

1. **Falsifier clauses** (either falsifies):
   a. On the family strata of the sealed suite, a matched reducer with the same grammar and information reaches
      the same witnessed answer at comparable cost.
   b. An answer accepted by the offline auditor `frob-verify` is wrong on any locked case.
2. **Competing arm.** An expert with standard solvers. **Potency controls.** The quantifier-order fixture (every
   design has a command that works and no single command works for all) must give different answers to the two
   queries "for every build some command" and "one command for every build"; registered certificate mutations
   must be rejected.
3. **Deciding gates (TARGETs).** At M1: GATE-AUGUR-01 (admission and checker reject 100% of a mutation corpus of
   at least 60 mutated queries and certificates, among them a swapped quantifier prefix and a clairvoyant
   witness, with the expected refusal code; zero false accepts); -02 (the stance family through the public
   verb: boundary exactly 3/14 m; four vertex witnesses; the dual vector (0, 1/2, 1/2) for the excluded cell;
   11,520 of 11,520 grid states agree with an independent zonotope test; lifted and projected forms agree; the
   passive-spring amendment withdraws the cell at b = 1/4 m); -03 (the docking family: 561 of 561 cells, nine
   boundary cells excluded, all late-packet cells excluded, the equality reading 15/16 exported, the three-gain
   amendment withdraws every positive cell and reports 1/65 m); -04 (the pusher battery: 20 of 20 intervals with
   exact ends, and the per-build and shared queries return [1/2, 2] and ∅ for the same plant); -05 (every
   Certified cell of -02 to -04 accepted by the auditor built from an empty environment on a host with no GPU,
   in under 10 s in total, bundle digests identical across two hosts); -06 (checker independence: dependency
   closure is `std`, zero shared source files with proposers, a seeded-fault campaign of at least 200 mutated
   witnesses per proof kind yields zero false accepts; repeated at M3); -07 (a fresh agent with only the
   generated documentation completes 10 scripted docking and stance amendments, names the binding hardware
   restriction in at least 9, never reports an unresolved cell as a verdict; serial turns recorded against an
   arm restricted to the lower verbs). At M3:
   the envelope cell of the partial ten-fold gate. At M4: GATE-AUGUR-15 (labels for a 65,536-row screen and one
   1,024-design family batch identical on `batch-cpu`, CUDA and Metal under the exact-integer or guarded class,
   zero final errors against exact CPU arithmetic).
4. **Kill gates and fallback.** KILL-AUGUR-TIE (first read on the docking slice at M1; decided at M3): a short
   robust program over a conventional solver reaches the same checked scope at comparable turnaround. Fallback:
   a tie retires any claim of exclusive expressivity on that fixture, and the query interface ships as a front
   end that compiles to standard solvers, behind the same verbs, schema and checker. KILL-AUGUR-FALSECERT
   (standing from M1): one certificate accepted by the auditor is shown false within its scope; if the cause is
   the rule and not an implementation defect, the proof kind is removed and its cells become class Tested. A
   false answer withdraws the proof rule that produced it.
5. **Downstream if falsified.** Under (a): the product keeps the query interface, schema and auditor and drops
   specialized projections; the research budget goes to the first mission the conventional program cannot
   resolve. Under (b): the affected proof kind's certificates are withdrawn everywhere, a permanent negative
   control is added, and the arm that submitted the false certificate is disqualified from the scoped claim.
6. **Procedure.** Register clauses, the reducers and "comparable cost" (within a factor of 1.5 at equal checked
   scope) before the M1 slice is timed; read at M1, M3 and M4; `docs/adr/bet-19-m1.md`, `-m3.md`, `-m4.md`. The
   M1 and M3 readings are taken at those exits by the exit reviews (BD-147 at M1, BD-319 at M3), from the gate
   rows through the bet register's verdict function (N00-28) and the exit state of N40-31. This bead closes at
   M4: it writes `bet-19-m1.md` and `bet-19-m3.md` from those recorded readings (it does not recompute them from
   later data) and `bet-19-m4.md` from the M4 reading.

## Acceptance criteria

- [ ] Pre-registered with the matched reducers named and given the same grammar and information.
- [ ] At M1: the seven gates recorded; the docking slice's first tie reading recorded with both arms' times.
- [ ] At M3 and M4: the envelope stratum's ratio against the reducer and GATE-AUGUR-15 recorded (GATE-AUGUR-15's
      row is entered by its gate script N25a-28; this bead reads it).
- [ ] Zero accepted false certificates on the locked cases, or KILL-AUGUR-FALSECERT applied.
- [ ] Verdicts recorded as `docs/adr/bet-19-m1.md`, `-m3.md` and `-m4.md` by a session that did not implement
      the mission engine, each citing by identifier and hash the kill-gate records on file at that milestone
      (KILL-AUGUR-TIE by N25a-18, `docs/adr/ADR-KILL-AUGUR-TIE-M3.md` from M3; KILL-AUGUR-FALSECERT by N25b-04);
      this bead writes no kill ADR; a tie is stated in the report.

## Test plan

### Unit tests
`crates/frob-conformance/tests/bets.rs`: `bet19_m1_gates_pass_first_reading_held`; `bet19_reducer_within_1_5_is_tie_falsifies_a`;
`bet19_one_false_certificate_falsifies_b_and_removes_proof_kind`; `bet19_quantifier_fixture_same_answer_is_untested`
(if the two queries return the same answer the control failed); `bet19_tie_keeps_verbs_schema_checker`.

### End-to-end script and logging
`scripts/e2e/bet_19_decide.sh` re-runs the stance and docking fixtures through `frob mission synthesize
<fixture> --key bet19-<fixture>-<seed> --json`, writes their bundles with `frob prove bundle a:… --out <dir> --key
bet19-bundle-<fixture> --json` and replays them with `frob-verify` on a host with no GPU, runs the reducer arm's
recorded program on the same fixtures, reads the recorded gate rows and the bet register's row of BET-19 (`cargo
run -p frob-conformance --example bets -- status --json`, example `bets` of N00-28), recomputes the verdict and checks that the ADR of the
milestone being read (`docs/adr/bet-19-m4.md` at M4) exists and matches. Logged per STD-E2E: fixture, cells by label, witness digests,
auditor verdicts and total check time against 10 s, both arms' times and their ratio against 1.5, mutations
rejected; on a mismatch between the recomputed verdict and the ADR, the first differing field and the paths of the
gate rows and the ADR; `summary` naming the gates and kill gates.

### Known answers and negative controls
KA-019 to KA-026 (stance boundary 3/14 m; allocations (160, 130, 110, 0) and (155, 135, 105, 5) N; 165 N at
b = 0.20 m; docking ε < (U + 1/2)/4), KA-032. CX-015 (quantifier order), CX-010 (midpoint). Negative controls:
the mutation corpus; a clairvoyant witness offered for a robust claim; a swapped quantifier prefix with
consistent hashes.

## Considerations

- The gain claimed is a kind of answer and its reuse, not solve speed; the mission subsystem's benchmarks are
  refused a speed comparison whose ceiling is below its claim.
- Certificates are statements about the declared model under listed assumptions, never about the world.

## References

§4 (BET-19), §40.5, §2.4, §2.6, §25.27, §43.4, §43.12; GATE-AUGUR-01 to -07, -15; GATE-TENFOLD-06;
KILL-AUGUR-TIE; KILL-AUGUR-FALSECERT.
