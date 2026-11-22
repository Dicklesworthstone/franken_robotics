---
key: "SPEC-C-18"
title: "spec C-18 (frob-mission): the assumption lattice and verdict polarity: the six-row survival table"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "lemma", "spine", "spec", "frontier", "frob-mission", "frob-conformance", "test"]
parent: "WP-42"
deps: ["SPEC-C-17", "N10b-08", "BD-025", "STD-TEST", "STD-E2E"]
needs: ["§10 preamble: the construction register `frob-conformance/registry/constructions.toml`, the ten-section contract template and the shared law-suite convention (bead N10a-01 of unit U10a); this bead fills row C-18", "§10 registries, Appendix M.9, M.11 and Appendix N.7: the registry beads that load rows KA-103 to KA-106, KA-224 and CX-048 to CX-050 (N10-KA of unit U10a, N10-CX of unit U10c, and the Lemma chapter's registry bead for KA-224); this bead binds them to tests", "§10 C-32: the stance reference model (four feet at (±1/3, ±b) m, 400 N, cap 160 N) from the specification bead SPEC-C-32 of unit U10c, for the stance half of the twelve-move check", "§25.4: the crate `frob-mission` is created by a bead that implements this construction (BD-128) and is expected to depend on this bead; it is deliberately not in `deps`"]
related: ["N10b-06", "N10b-07", "SPEC-C-32", "BD-128", "BD-288", "BD-292", "BD-286", "BD-111", "BD-347"]
gates: ["GATE-GAUNT-02", "GATE-GAUNT-01"]
plan_refs: ["§10 C-18", "§9 D-11", "§9 D-19", "§12.7 PO-LEMMA-03", "§25.8", "§28.13", "§38.7 MR-10 MR-11", "Appendix D.6", "Appendix M.9 KA-103 to KA-106", "Appendix M KA-224", "Appendix N.7 CX-048 to CX-050"]
---
## Background

Requirements, motors and tolerances change during a design study. The two usual responses are both wrong:
recomputing everything after every amendment wastes evidence, and trusting a cache key to mean "the verdict
still holds" produces stale verdicts. Construction C-18 gives the rule in between. The robust design family
F(W, Π, A) of a mission (construction C-17: the designs for which one causal policy in the class Π satisfies
the acceptance predicate A in every world of the set W) moves in a known direction when W, Π or A change. So
every certificate has a **polarity** on each axis, and an amendment is answered by reading a six-row table:
which certificates survive, which regions are invalidated, which reruns are required and what they cost
(`result.revalidate`). A motor, a tolerance or a requirement changes without redoing the part of the answer
that an implication already covers.

Tier: Frontier [F]. Crate: `frob-mission` (the revalidation engine also serves the studio's memo graph). It
serves BET-33 (revalidation) and doctrine rules D-11 (a hash identifies; only an implication licenses skipping
a recheck) and D-19 (a model assumption is an input with a name).

Terms. *Inner certificate*: evidence that a cell of designs lies in F. *Exclusion*: evidence that a cell lies
outside F. *Amendment*: a change to W, Π or A. *Survives*: the certificate remains valid without any
recomputation. *Recheck*: the certificate must be re-evaluated; the result may confirm or reverse it.
*Antitone*: order-reversing. *A ordered by implication*: A is stronger than A′ when every execution accepted
by A is accepted by A′.

Rejected alternatives: recompute from scratch on every amendment (safe, kept as the fallback); reuse keyed by
content hash alone (a hash identifies bytes and carries no verdict); intersecting per-world answers to get the
joint answer (wrong for an existential controller, shown below).

## Scope

In: `docs/math/C-18.md` (statements S1 to S5; the duality section comes from N10b-06); the law suite
`crates/frob-conformance/tests/laws_c18.rs`; registry rows KA-103 to KA-106 and the binding of KA-224;
controls from CX-048, CX-049, CX-050; row C-18 of the construction register.

Out: the Galois connection and the separating world (N10b-06); the revalidation engine and verbs (BD-292,
BD-347); monotone transport between cells (BD-288); the memo graph and its shadow recompute (BD-111); the
pilot (N10b-07).

Home of the law suite (DESIGN, with reason): `frob-conformance`, because `frob-mission` is created by the bead
that implements this construction and that bead depends on this specification.

## Technical approach

### 1. Statements (status HYPOTHESIS; class once the route has run)

| Id | Statement | Class and route |
|---|---|---|
| C-18.S1 | F is antitone in the world set W, monotone in the policy class Π (more information, earlier packets, more actuator authority), and monotone in the acceptance predicate A ordered by implication | `Tested` as a G0 law on finite models; the three one-line arguments are on file in the contract |
| C-18.S2 | The polarity table below gives the survival of each certificate kind under each amendment | `Tested`: fresh evaluation of all twelve moves per fixture (proof obligation PO-LEMMA-03) |
| C-18.S3 | A certificate records the assumption set it used and its polarity on each axis; an amendment is answered by reading the table and returns surviving certificates, invalidated regions, required reruns and their cost | `Tested` (schema and behaviour) |
| C-18.S4 | For designs with an existential controller, per-world answers do not intersect to the joint answer: F(V) ⊆ ⋂_{w ∈ V} F({w}), and the gap is exactly what the controller cannot observe in time | `Certified` on the docking fixture (exact); `Tested` in general |
| C-18.S5 | A verdict is reused after a change only under a recorded implication: polarity, a monotonicity argument, a symmetry with its stabilizer check, or equality of every dependency. A hash match is not a verdict transfer | `Tested`; refusal FROB-REF-835 (a verdict is reused after a change without a recorded implication; exit code 3) |

Arguments for S1. If W′ ⊆ W, a policy that succeeds in every world of W succeeds in every world of W′, so
F(W) ⊆ F(W′). If Π ⊆ Π′, a witness policy in Π is in Π′, so F(Π) ⊆ F(Π′). If A implies A′, every execution
accepted under A is accepted under A′, so F(A) ⊆ F(A′).

**The polarity table (normative).**

| Amendment | Inner certificate (d ∈ I) | Exclusion (d ∉ O) |
|---|---|---|
| W shrinks | survives | must be rechecked |
| W grows | must be rechecked | survives |
| Π grows | survives | must be rechecked |
| Π shrinks | must be rechecked | survives |
| A weakens | survives | must be rechecked |
| A strengthens | must be rechecked | survives |

Priors over worlds stay distinct from admissibility of worlds: a change of weighting is not an amendment of W.

### 2. Record on a certificate (DESIGN, as carried in the bundle format)

Each assumption of a certificate carries the direction in which the certificate survives it:

```json
"assumptions": [ { "id": "authority", "value": "independent normal forces", "survives": "grow" },
                 { "id": "world-set", "blake3": "…", "survives": "shrink" } ]
```

For an inner certificate the world set survives `shrink` and the policy class and authority survive `grow`;
for an exclusion the directions are reversed. The acceptance predicate is recorded with `weaken` or
`strengthen` in the same way. An amendment that cannot be ordered against the recorded assumption (neither
subset nor superset, neither implied nor implying) is treated as "must be rechecked".

### 3. The twelve moves on the docking fixture (all exact; reference model of N10b-08)

The docking family: unit mass, hidden gain g ∈ {1, 2}, three ticks, command bound U, read error ε; with the
packet on time a robust policy exists exactly when ε < (U + ½)/4. Axes: W is the gain set and the error
interval [−ε, ε]; Π is the authority U and the packet timing; A is exact docking (x₃, v₃) = (1, 0).

| # | Certificate | Amendment | Table says | Fresh evaluation |
|---|---|---|---|---|
| 1 | Inclusion at (U, ε) = (¾, ¼) | W shrinks: ε falls to 1/8 | survives | inner (1/8 < 5/16) |
| 2 | same | W grows: ε rises to 5/16 | recheck | reversed (boundary; strict) |
| 3 | same | Π grows: U rises to 1 | survives | inner (¼ < 3/8) |
| 4 | same | Π shrinks: U falls to ½ | recheck | reversed (¼ is not below ¼) |
| 5 | same | A weakens: only v₃ = 0 is required | survives | inner |
| 6 | same | A strengthens: also x_k ≤ 1 at every tick | recheck | reversed: any admissible probe has p > 2ε = ½, and for g = 2 the position after two ticks is p + ½ > 1 |
| 7 | Exclusion at (¾, 3/8) | W grows: ε rises to ½ | survives | excluded |
| 8 | same | W shrinks: ε falls to ¼ | recheck | reversed (becomes inner) |
| 9 | same | Π shrinks: U falls to ½ | survives | excluded |
| 10 | same | Π grows: U rises to 1 | recheck | confirmed (3/8 is not below 3/8) |
| 11 | same | A strengthens: also x_k ≤ 1 | survives | excluded |
| 12 | same | A weakens: only v₃ = 0 is required | recheck | reversed (the zero command succeeds) |

Position after two ticks, for move 6: x₂ = g·p/2 + ½, from x₁ = g·p/2, v₁ = g·p and u₁ = 1/g − 2p. A recheck
may confirm (move 10) or reverse; the table never claims which.

**World set enlarged by one hidden gain (KA-104).** Adding the gain 33/32 keeps every exclusion and forces a
recheck of every inclusion. The inclusion at (U, ε) = (¾, 1/64) reverses, because the reading (65/64)·p is
possible under gains 1 and 33/32 for every first command p, and the least worst-case terminal error becomes
1/65 m.

**Per-world answers against the joint answer (KA-105).** With the packet late, each of the worlds g = 1 and
g = 2 is solvable alone and the joint query has no solution: the inclusion F(V) ⊆ ⋂ F({w}) is strict.

**Stale verdict (KA-106).** A dependency graph with one edge omitted (the delivery time left out of a key)
returns a promotion that fresh replay contradicts; the check (a shadow recompute) must catch it.

**The stance fixture's twelve moves** use the same harness with the stance reference model (400 N on four feet
at (±1/3, ±b) m, centre of mass in a rectangle of half-sizes 150 mm and 75 mm, each foot in [0, 160] N; the
binding corner requires f₊₊ − f₋₋ = 90 + 15/b N). Certificates: the inclusion at b = ¼ m (forces
(155, 135, 105, 5) N) and the exclusion at b = 1/5 m (165 N against the 160 N cap). Moves: W = the
centre-of-mass rectangle (shrink to 100 × 50 mm, grow to 200 × 100 mm); Π = the per-foot cap and the authority
model (grow to 170 N; shrink to 150 N, or to equal passive springs, which give (175, 115, 85, 25) N and
withdraw the inclusion at b = ¼ m); A = a force floor per foot (strengthen: every foot at least 10 N; weaken:
drop the lower bound on one foot). Expected table entries are those of the polarity table; fresh evaluations
come from the stance reference model.

### 4. Honest limits, foundations

- Polarity is an evidence rule, not a speed lever. FACT (planning pilot, to be re-run): a semantic receipt and
  a correctly keyed dependency graph did identical work on a four-state edit sequence (three pair evaluations
  and two reductions each; every verdict matched fresh replay; omitting one dependency inherited a stale
  promotion). FACT (source register, to be re-read): exact reuse of rollouts after a mass or geometry edit is
  at most about 10% of compute. The benefit is fewer wrong reuses.
- Foundations (FACT, observed 2026-11-21 at FrankenSim `483d09c682f2`): `fs-recompute` and `fs-ledger` exist as
  crates; whether their dependency records can carry polarity is an admission question and is not answered.

### 5. Law suite and registry (DESIGN)

- `crates/frob-conformance/tests/laws_c18.rs`, generic over `C18Subject { fn revalidate(&self, cert: &Cert,
  amendment: &Amendment) -> Revalidation; fn fresh(&self, query: &FiniteQuery) -> Verdict }` where
  `Revalidation` is `Survives | Recheck`. `tests/support/c18_ref.rs`: the table as data, finite models for S1
  and S4, the twelve docking moves over `docking_ref.rs`, a three-node dependency graph for KA-106.
  `tests/support/c18_mutants.rs`: mutants under the cargo feature `controls`.
- Register row C-18: `verb = "result.revalidate"` (with `mission.revalidate`), `failing_test` =
  `frob-conformance::laws_c18::s2_twelve_moves_docking`, `payoff.gate = "GATE-AUGUR-12"`, `known_answers =
  KA-103 … KA-107`, `pilot = "PILOT-C-18"`, `generic_comparator = "recomputation from the dependency closure of
  a correctly keyed dependency graph"`.
- Gates that measure the construction (recorded by mechanism beads): GATE-AUGUR-03 at M1 (the three-gain
  amendment withdraws every positive cell and reports 1/65 m); GATE-ATEL-11 (revalidation on the frozen
  30-amendment campaign: every decision equals full recomputation; zero unsafe reuse; the stale-key mutant
  caught; unnecessary invalidations at most 20% of unaffected verdicts); GATE-AUGUR-12 (zero incorrectly
  inherited verdicts on the same campaign); GATE-PHYLUM-05. Kill gates: KILL-AUGUR-REUSE (standing from M3: one
  incorrectly inherited verdict from an implication rule removes that rule; reuse falls back to the
  dependency-keyed memo graph) and KILL-REUSE.

## Acceptance criteria

- [ ] `docs/math/C-18.md` has the ten sections, the polarity table verbatim, statements C-18.S1 to S5 with
      classes, and the twelve docking moves with their fresh evaluations.
- [ ] `cargo test -p frob-conformance --test laws_c18` passes; with `--features controls` every mutant fails.
- [ ] KA-103 and PO-LEMMA-03 (docking half): for all twelve moves the table entry equals the expected entry,
      and every `survives` entry is confirmed by fresh evaluation.
- [ ] KA-104: with the gain 33/32 added, all 217 exclusions of the grid survive, all 344 inclusions are marked
      for recheck, the inclusion at (¾, 1/64) reverses and the worst error is exactly 1/65.
- [ ] KA-105: joint F is empty and both single-world answers are nonempty on the late-packet query.
- [ ] KA-106: the omitted-edge graph returns the stale promotion and the shadow recompute flags it.
- [ ] `scripts/e2e/spec_c18.sh` exits 0.

## Test plan

### Unit tests

- `s1_monotonicity_finite`: 500 seeded finite queries; for random W′ ⊆ W, Π ⊆ Π′ and A ⇒ A′ the three
  inclusions of S1 hold (brute force).
- `s2_twelve_moves_docking`: the table above, entry and fresh evaluation.
- `s2_survivor_never_contradicted`: over the whole 561-cell grid and 200 seeded amendments per polarity, no
  certificate marked `survives` is contradicted by fresh evaluation.
- `s2_recheck_can_confirm`: move 10 is `Recheck` and its fresh evaluation is unchanged.
- `s2_unordered_amendment_is_recheck`: an amendment that both removes one gain and adds another is `Recheck`
  for both certificate kinds.
- `s3_certificate_records_polarity`: every reference certificate carries its assumption set with a direction;
  a certificate without it is rejected.
- `s4_per_world_not_joint`, `ka104_third_gain`, `ka106_stale_edge`: the acceptance values.
- `s5_hash_is_not_verdict`: two queries with equal design hash and different delivery time do not share a
  verdict; the attempt is refused with FROB-REF-835.
- `controls_are_caught`.

### End-to-end script and logging

`scripts/e2e/spec_c18.sh` (idempotent; seed 1800 printed): `frob --json doctor`; the law suite with and without
`--features controls`; `cargo test -p frob-conformance --test g1 -- KA-103 KA-104 KA-105`;
`cargo test -p frob-conformance --test g0 -- KA-106`; existence of `docs/math/C-18.md`. Sources the STD-E2E
helper; writes `target/e2e-logs/spec_c18/<utc>.ndjson` and a transcript; each line has `ts`, `script`, `bead`
(`SPEC-C-18`), `step`, `event` (`start`, `cmd`, `assert`, `metric`, `refusal`, `end`), command line, exit code,
wall time, evidence block (claim Known answer, class, tier G0 or G1 host, determinism D0, seed), per move the
table entry, the expected entry and the fresh verdict, and on failure the first (certificate, amendment) pair
that disagrees. Final `summary`: assertion counts and GATE-GAUNT-02.

### Known answers and negative controls

| Row | Truth | Tolerance | Tier | Control |
|---|---|---|---|---|
| KA-103 | Inclusion at (¾, ¼) survives ε falling to 1/8 and U rising to 1; exclusion at (¾, 3/8) survives ε rising and U falling; the four opposite moves demand a recheck | exact | G1 · host | Registered here (the appendix states none): a table with the two columns exchanged, so that an inclusion "survives" W growing |
| KA-104 | One hidden gain 33/32 added: exclusions kept, inclusions rechecked, (¾, 1/64) reverses, 1/65 m | exact | G1 · host | CX-049: an inclusion carried across the enlargement without a recheck |
| KA-105 | F(V) ⊆ ⋂ F({w}), strict on the late packet | exact | G1 · host | CX-048: intersecting per-world answers |
| KA-106 | An omitted dependency edge returns a promotion that fresh replay contradicts, and the check catches it | exact (verdict) | G0 · host | CX-050: the omitted edge |
| KA-224 | Each row of the polarity table gives the stated survival on the docking and stance fixtures, by fresh evaluation of all twelve moves per fixture | exact (verdict) | G1 · auditor (bound by the bundle bead) | Registered here: hash match taken as verdict transfer |

Metamorphic relations: an edit followed by its inverse restores every verdict with zero evaluations, and an
edit to a field the mission does not read keeps every result (the control is a memo key without delivery
time); an inclusion survives the world set shrinking, the policy class growing and the predicate weakening, and
an exclusion survives the three opposite moves. Determinism: exact arithmetic, class D0; twice on one machine,
debug and release.

## Considerations

- One false survivor is the failure that matters (it is the pilot's falsifier and the kill-gate trigger). The
  suite therefore tests survivors against fresh evaluation exhaustively on the grid, not by sample.
- The table is conservative by design: "recheck" is always safe. An implementation may add finer implication
  rules; each needs its own recorded argument and its own falsification test, and each can be removed alone.
- Widening a hypothesis set by a hair invalidates inclusions wholesale (the 33/32 gain). This is the intended
  behaviour and is surfaced to the agent as a cost, not hidden.
- Fallback named in advance: every amendment recomputes from its dependency closure.

## References

Plan §10 C-18; §9 D-11, D-19; §12.7 (PO-LEMMA-03, FROB-REF-835); §25.8; §28.13; §38.7 (MR-10, MR-11); Appendix
D.6 (monotonicity and polarity); KA-103 to KA-106, KA-224, KA-036; CX-048 to CX-050; BET-33; GATE-ATEL-11,
GATE-AUGUR-03, GATE-AUGUR-12; KILL-AUGUR-REUSE.
