---
key: "EPIC-M7"
title: "Milestone M7, the ten-fold tournament: the thesis measured against the strongest rival, published whatever it says"
type: "epic"
priority: 2
milestone: "M7"
labels: ["m7", "delivery", "tenfold", "solid", "gate"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["BD-434", "N40-14", "N40-16", "N40-17", "N40-18", "N40-20", "N40-21", "N40-22", "N40-28", "N40-07", "EPIC-M4", "EPIC-M8"]
gates: ["GATE-TENFOLD-03", "GATE-TENFOLD-04", "GATE-TENFOLD-05", "GATE-TENFOLD-07", "GATE-TENFOLD-08", "GATE-TENFOLD-11", "GATE-TENFOLD-12", "GATE-TENFOLD-13", "GATE-TENFOLD-09", "GATE-GAUNT-01", "KILL-TENFOLD-SPEED", "KILL-TENFOLD-TIE", "KILL-TENFOLD-CERT", "KILL-TENFOLD-LEAK", "KILL-TENFOLD-ADJ", "KILL-TENFOLD-REACH"]
plan_refs: ["§43.10", "§43.1", "§43.2", "§40.5", "§40.6", "§39.10", "§39.13", "§43.12"]
---
## Background

**Purpose.** M7 measures the thesis against the strongest composed rival, and publishes the measured result
whatever it is.

**Why this is a milestone and why it has no subsystem gate.** The program's central bet (BET-13) is that an agent
with this system reaches an accepted engineering answer in one tenth of the time the strongest incumbent workflow
needs, on sealed missions. Everything before M7 measured parts of that claim on development fixtures or on a
partial suite. M7 is the one test whose result is the claim: sealed mission families that no earlier gate used,
every arm frozen and published before any sealed mission is opened, neutral adjudication, and a report that is
published with its phase ledgers and a replication kit. The exit is the *publication of the measured ratio with
its phase ledger*, so M7 cannot fail by producing a low number; it can only fail by not being run honestly. M7
needs M4 and neither M5 nor M6: a sealed stratum that needs a mechanism of M5 or M6 which has not landed is
declared `not run` in the report, with the reason. Planning estimate: 6 weeks, including a two-week freeze and
rehearsal (DESIGN). Critical path: yes.

**Demonstrates.** NS-24 *The sealed ten-fold tournament*: missions in named strata are sealed, and some worlds come
from an evaluator independent of both arms. The rival is the strongest composed incumbent workflow, improved by a
competent baseline engineer on development fixtures and then frozen. Both arms get the same agent model, token
budget, hardware envelope and prior information, in fresh contexts. A neutral adjudicator releases paired receipts
after both arms close. The report gives, per stratum, the ratio of restricted mean times with its interval, cold
and warm separately, the crossover reuse count, and a phase ledger for each arm that sums to wall-clock. Every
submitted certificate is replayed by `frob-verify`, and one false certificate disqualifies that arm from the scoped
claim.

## Scope

In: the sealed suite in named strata; every pin frozen; the rival arms; neutral adjudication; the report; the
replication kit; suite hygiene; the recovery ratios on the sealed suite.

Explicitly not in M7: any mechanism work (no mechanism is killed or built at M7); any stopping on an interim
number; any comparative claim before publication.

What follows from M7: positioning. The product is described by its measured ratio per stratum, and by auditability
where the ratio is under 3. The table of expected ranges in the thesis is replaced by the measured table.

## Technical approach

**1. The artifact.** The sealed suite in named strata (`legged-quad`, `legged-biped`, `wheeled`, `tracked`,
`hybrid`, `envelope`, `sensing`, `rebody`, `failure`, `throughput`, and `facility` only if its Moonshot gate has
passed). Every pin frozen. The rival arms: the strongest composed incumbent workflow, tuned on development
fixtures and then frozen, and our own layer over the incumbent engine where the verbs allow it. Neutral
adjudication with receipts released after both arms close. The report: per stratum, the ratio of restricted mean
times with its interval, cold and warm runs separately, the crossover reuse count, a phase ledger for each arm
that sums to wall-clock, the audit of every submitted certificate by `frob-verify`, the tie statement, and the
count of certificates that changed a decision.

**2. Order of work.** Refresh the rival audit; publish the enlarged development pool (which now holds the families
the partial gate retired); commit the suite and protocol hashes; tune; freeze and publish every arm; record the
pre-analysis and name at most four confirmatory cells; run with the arms interleaved; close and reveal; publish
the report and the kit; have an external party re-run the kit. Adjudication at M7 is external, or the word is not
used until the external replication has passed.

**3. Gates at this exit.** No admission gate and no subsystem convergence gate is first due at M7. The exit is
defined by program gates (beads in parentheses):

| Gate | Part due | Pass criterion in brief (TARGET) |
|---|---|---|
| GATE-TENFOLD-03 (N40-14) | Repeated before the sealed suite is opened | The harness reproduces every known answer of the ten-fold harness exactly and catches every negative control |
| GATE-TENFOLD-04 (N40-14) | Repeated | Freeze and seal: the six steps in order; a published manifest and advocate statement for every arm; the frozen rival reproduces its home-task training time within 20% on `trj` and wins at least one easy-win development family per stratum; the pre-analysis recorded, with every confirmatory cell meeting the reachability conditions |
| GATE-TENFOLD-05 (N40-14) | Repeated | Adjudicator acceptance: the adjudicating simulator passes the known answers it can express inside its own tolerances; classifies 20 of 20 planted answers per stratum; appears in no arm's training or tuning logs; discordance with each home referee reported on a 5% sample |
| GATE-TENFOLD-11 (N40-20) | Repeated before sealing | The rival audit is complete: each arm's frozen configuration reproduces its development-fixture figure within 10%; the sign-off and the pack digest are recorded; a changed digest voids the arm |
| GATE-TENFOLD-07 (N40-16) | The tournament | The ratio, interval, verdict, completion, coverage, tie test and phase ledgers for every stratum run, on fresh sealed families with every pin frozen, published with its ledgers and its kit. Met when the report is published, whatever it says |
| GATE-TENFOLD-08 (N40-17) | The replication kit | Level 1 recomputes every published number bit for bit on a clean machine; level 2 re-runs one sampled pair per stratum with the same acceptance outcome and a time within a factor of 1.5 in at least 90% of pairs; a kit with a private input is refused |
| GATE-TENFOLD-12 (N40-21) | Suite hygiene | An easy win and an unresolved answer in every stratum; every planted mutation rejected; zero missed withdrawals for a scoped claim |
| GATE-TENFOLD-13 (N40-22) | On the sealed suite | The recovery ratios r_T and r_C with 95% intervals for every stratum that has a ratio; the three phase ablations inside their bounds |
| GATE-TENFOLD-09 (N40-18) | Recurring | Ceiling before speed, program-wide |
| GATE-GROUND-01, GATE-SAFE-01, GATE-GAUNT-01, -02, -03, -05, -10 | Recurring | As at every exit |

If GATE-TENFOLD-07 is not met, M7 does not exit and no comparative claim is made. If GATE-TENFOLD-08 is not met,
the result is labelled `unreplicated` and the word "ten-fold" is not used. If GATE-TENFOLD-12 is not met, the
affected stratum is `descriptive`.

No admission or convergence gate of the compiled register tables has a part due at this exit.

**4. Kill gates decided here.** No mechanism is killed at M7. Three written rules are applied: the decision rule
of BET-13 and its wording for each range of the ratio (below 3: `withdrawn`; from 3 up to 10, or 10 and above
without the guard: `measured`, and the word "ten-fold" is not used for the class; `ten-fold` only with the guard);
the tie rule of BET-14 (arm C within a factor of 1.5, or a recovery ratio of at least 0.8 on time and on compute:
say so and ship the layer over that engine for that stratum); and disqualification of an arm from the scoped claim
by one false certificate. Every KILL-TENFOLD rule is read (ledger bead N40-28): KILL-TENFOLD-SPEED, -TIE, -CERT,
-LEAK, -ADJ, -REACH.

No kill gate of the compiled register tables is decided at this exit.

**5. Exit rule.** Recorded by a fresh-context session (seed BD-434, procedure N40-05). The exit line:
*GATE-TENFOLD-07: the measured ratio published with its phase ledger.*

**6. Dependency firewall.** Every pin and the toolchain T frozen for the length of the tournament. Uses if
admitted: fastmcp_rust for rival access. A pin move or a toolchain train voids the runs it spans (check N40-07).

**7. Owner input wanted.** OPEN-PROG-2, the neutral party and the adjudicating simulator, confirmed before the
suite is sealed (default: a pinned incumbent CPU engine that neither arm trains in, run by a fresh-context session;
every ratio labelled `self-adjudicated`; the words "independently accepted" are not used). OPEN-PROG-1 (repository
visibility, and who outside the program can replicate the result; default: private; the kit and `frob-verify` are
the artifacts prepared for a third party). The compute envelope for the tournament itself: 24 mission families per
cell by default and at least 32 for a cell that declares a population result, on both machines, a campaign class
per run and a pretraining class where an arm pretrains (default if not approved: fewer strata, each declared
`not run` with the reason).

**8. Stop or re-plan.** A tournament is not stopped by an interim number. It is voided and re-run only for a breach
of protocol: a pin moved during the freeze, sealed material read by an arm, or a rival arm changed after its
freeze. After publication the positioning follows BET-13.

**9. No single failed bet blocks the exit.** BET-13 (the ratio is under ten) → the measured ratio, published per
stratum with the wording BET-13 fixes. BET-14 (the composed incumbent ties) → the layer over that engine for the
stratum, and a report that says so.

**10. Work packages.**

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-76 | Sealed suite, rival arms and the freeze | `frob-conformance` | M7 | GATE-TENFOLD-03, -04, -05, -11, -12 |
| WP-77 | Tournament, adjudication and publication | `frob-conformance`, `docs/` | M7 | GATE-TENFOLD-07, -08, -13; NS-24 |

## Acceptance criteria

- [ ] GATE-TENFOLD-03, -04, -05 and -11 are recorded again, on the M7 suite, before the sealed suite is opened.
- [ ] The report is published with one table per cell and one phase ledger per arm per cell, the ceilings
      evaluated on the rival's ledger, and the replication kit (GATE-TENFOLD-07).
- [ ] GATE-TENFOLD-08, -12 and -13 are recorded; a failed -08 prints `unreplicated` on every figure.
- [ ] No family used at the M3 partial gate is in the sealed suite; no pin moved during the freeze.
- [ ] Each of the six KILL-TENFOLD rules that apply has its outcome per cell recorded as an ADR.
- [ ] The words used for each stratum are the ones the outcome rules fix for its measured range.
- [ ] The exit report for M7 is `exit-ready`; the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by the ten-fold harness beads. Epic check: due counts for M7 are 0 admission, 0 convergence, 0 kill
decisions of the compiled subsystem tables; eight ten-fold program parts and the recurring gates.

### End-to-end script and logging

`scripts/exit/review.sh M7`: the scorer of the replication kit, which depends on nothing outside the standard
library, recomputes every published ratio, interval, count and verdict from ledgers and receipts on a clean
machine and compares bit for bit; `frob-verify` replays every submitted certificate; the lock digest is compared
at both ends of the freeze window. Logs: each recomputed value beside the published one, digests, the evidence
block of each receipt, and the gates touched. Logs carry no sealed content before the reveal.

### Known answers and negative controls

The harness's own known answers (a six-family card with R̂ = 2.647 and completion ceiling 3.75; critical counts
17 of 24 at 0.05 and 18 of 24 at 0.0125; the zero-count bound 1 − 0.05^(1/24) = 0.1174). Controls: a planted false
certificate; a canary value from a sealed world in a transcript; a ledger with 3% unattributed time; a manifest
whose freeze time follows the suite opening; a confirmatory cell of four families.

## Considerations

- The tournament exceeds every default compute envelope and needs an external party for the word "replicated".
- A low ratio is a result, not an incident. The claims that never depended on speed remain: single-world replay
  on the CPU, error-controlled comparisons, independently checked family answers, minimal counterexample worlds
  with interventional explanations, typed evidence, a closing energy ledger.
- A private repository cannot be independently confirmed; that limit is printed with the result.

## References

Plan §43.10, §39.10, §39.13, §40.5, §40.6, §43.12. Bets BET-13, BET-14. Scenario NS-24. Risk RISK-52. Work
packages WP-76, WP-77.
