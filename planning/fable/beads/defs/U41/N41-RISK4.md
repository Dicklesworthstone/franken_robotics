---
key: "N41-RISK4"
title: "registries/risks.toml, part 4: evidence integrity, evaluation hygiene, privacy and security risks (RISK-11, -12, -14, -15, -17, -34 to -39, -44, -49, -50)"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "safety", "arena", "lemma", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N41-01"]
needs: ["§24 to §29, §37: the keys named in the Beads column are owned by the chapter units; they are named for traceability, not as blocking dependencies"]
related: ["N41-RISK1", "N41-RISK2", "N41-RISK3", "N41-PM", "N41-32", "BD-260", "BD-261", "BD-132", "BD-345", "BD-292", "N34-25"]
gates: []
plan_refs: ["§41.3", "§41.4", "§12", "§24", "§28", "§29", "§37"]
---
## Background

Fourth part of the risk register (conventions in N41-RISK1; schema and checks in N41-01). Its fourteen rows are the
ways the **evidence** can fail while every number looks fine: a qualification lane that can be retried until it
passes, a certificate for a statement nobody asked about, a checker that shares a defect with the thing it checks,
a screen that discards the winner, a cache that serves a stale verdict, an evaluation contaminated by training, a
scorecard whose denominator shrank, a private scan that leaks through a derived artifact, and text in an imported
file that addresses the agent. They share one remedy, stated once: **the party that proposes a result never checks
it**, and every check has a planted failure it must catch.

Terms. The *blinded qualification lane* evaluates a committed request on sealed world clusters and returns a
receipt and nothing per world; the i-th request of a campaign is tested at level α_i = α/[i·(i + 1)], so the total
error stays under α however many times an agent retries. A *bundle* (`.frobcert`) carries a certificate together
with the canonical bytes of the statement it certifies; `frob-verify` is the offline auditor built from `frob-cert`
with no dependencies beyond the standard library. The *audit lane* re-evaluates a sample of what a screen
discarded. *Shadow recompute* recomputes a sampled share of memo hits and compares.

## Scope

In: the fourteen rows below as records of `registries/risks.toml`. Out: the schema, loader and exit review
(N41-01); the mitigations (the beads named per row).

## Technical approach

Record fields as in N41-01.

| ID, severity | Risk | Earliest signal (detector) | Mitigation | Owner | Detected or decided by | Beads that carry the mitigation | Planted instance; first readable |
|---|---|---|---|---|---|---|---|
| RISK-11, Medium | Asset licences restrict what can ship | A pack whose licence join is not redistributable | Procedural content first; licence manifests enforced at pack time; GATE-SAFE-05 | Habitat lead | GATE-SAFE-05 | BD-305 | A pack containing one asset under a no-redistribution licence is refused at export with the asset named. M2 |
| RISK-12, High | A scan of a private place leaks through a still, a fixture or policy weights | A derived artifact without the private mark | Inheritance of the private mark by every derived artifact; the export flag; `scene.purge`; GATE-SAFE-04; weights inherit mark and licence by default (OPEN-SAFE-2) | Chronicle lead | GATE-SAFE-04 | BD-305, BD-398, N41-11 | A still rendered from a private scene and stripped of its mark by a scratch tool is blocked at export; weights trained in that scene carry the mark. M2 |
| RISK-14, Medium | The design corpus is tuned to the analyses | Corpus labels edited after an analysis changed | Labels frozen before tuning; a held-out third; potency controls in GATE-ATEL-01 | Arena lead | GATE-ATEL-01; GATE-GAUNT-09 | BD-248, BD-250, N38a-36 | A label edit recorded after an analysis version change is rejected by the freeze check. M3 |
| RISK-15, Medium | Agents optimize the scorecard instead of the robot, including by moving failures out of the denominator | Scorecard columns that do not sum to the battery size; a battery whose identity changed | Promotion, exploitation detectors, held-out scenes, tail metrics; GATE-ARENA-07; numerical faults kept apart from physical failures (GATE-CHRON-08) | Atelier lead | GATE-ARENA-07; GATE-CHRON-08; GATE-FID-02 | BD-254, N24a-22, BD-208, BD-263 | A scorecard whose outcome columns sum to less than the battery size is refused; a result on a battery with a changed identity is not comparable. M3 |
| RISK-17, Medium | Baseline controllers mask design flaws or create false ones | Disagreement between estimated-state and privileged-state runs; `controller-sensitive` flags | Three-outcome reporting (never a bare failure); the comparator of GATE-REFLEX-03; rank agreement before a rung may eliminate | Reflex lead | GATE-REFLEX-03; GATE-REFLEX-04; KILL-BASELINE | BD-114, N22a-05, BD-249, N22a-17, BD-328 | A planted IMU bias makes the estimator's `overconfident` flag fire; a design that fails only under the baseline controller is reported `controller-sensitive`. M1 |
| RISK-34, High | Qualification is gamed by adaptive retries or by a leak from sealed worlds | A receipt field outside the listed set; requests without a close; a denominator that shrank | The blinded lane: sealed clusters, pre-committed requests, the level schedule α_i; GATE-QUAL-01 (error rate held under 500 adaptive retries); the firewall gates GATE-QUAL-02 and GATE-SAFE-03; replay of a campaign by `frob-verify` (GATE-QUAL-03) | Arena lead | GATE-QUAL-01; GATE-QUAL-02; GATE-SAFE-03; GATE-QUAL-03 | BD-259, BD-260, BD-261, BD-383, N24c-13 | A null design retried 500 times adaptively qualifies with frequency under α; a receipt with one extra field is refused. M3 |
| RISK-35, High | A certificate is issued for a statement other than the one asked | A mutated quantifier, authority or comparator accepted by the checker; a bundle whose embedded statement differs from the query | Doctrine rule D-12; the bundle embeds the canonical bytes of the statement; the mission identity changes with any quantifier; the mutation suites of GATE-LEMMA-03 and GATE-AUGUR-01, -06; KILL-AUGUR-FALSECERT; unreviewed bundles say so (OPEN-AUGUR-8) | Lemma lead | GATE-LEMMA-03; GATE-AUGUR-01; GATE-AUGUR-06; KILL-AUGUR-FALSECERT | BD-132, N12-34, BD-126, BD-136, N25b-04, N25b-02 | A bundle whose quantifier is changed from "for all" to "exists" with recomputed, consistent hashes is rejected by the auditor. M1 |
| RISK-36, High | The proposer and the checker share a defect | A source file shared between `frob-cert` and a proposer; a checker dependency beyond the standard library | The architecture decision that `frob-cert` defines and reads bundles while the proposer writes them; layering rule L-9; GATE-WS-03; GATE-AUGUR-06; a second independent checker if one is not enough (OPEN-LEMMA-4) | Lemma lead | GATE-WS-03; GATE-AUGUR-06; GATE-LEMMA-04 | BD-026, N34-25, N25b-05, N12-29, BD-294 | A source file placed in both `frob-cert` and a proposer crate fails the shared-source scan; a dependency added to `frob-cert` makes `cargo tree -p frob-cert` print more than one line and fails the gate. M0 |
| RISK-37, High | A screen discards the winner | The audit lane finds a discarded design in the true top band; a planted winner is missing from the finalists | The screening audit (GATE-NEMESIS-07) and GATE-ATEL-09: no rung is trusted without a passed audit; a rung is `uninformative` where rank agreement is unmeasured; KILL-STATIC | Atelier lead | GATE-ATEL-09; GATE-NEMESIS-07; KILL-STATIC | BD-345, N26b-07, N28b-30, N28a-22 | The planted winner of the design corpus: a screen that discards it fails the audit. M3 |
| RISK-38, High | An incomplete memo key serves a stale verdict | A shadow-recompute mismatch | The shadow recompute and its gates (GATE-ATEL-11, GATE-ATEL-12); the memo-store admission INT-FSIM-16; polarity rules in revalidation; KILL-REUSE | Atelier lead | GATE-ATEL-11; GATE-ATEL-12; INT-FSIM-16; KILL-REUSE | BD-292, BD-347, BD-111, N28b-40, N28b-36, N28a-24 | The four-state camera fixture: a memo key without delivery time keeps a stale promotion, and the shadow recompute reports the mismatch. M1 |
| RISK-39, Medium | Retention deletes the interval that explains a decision | A citation of a collected object is refused; an `evidence.trace` closure that does not resolve | Decision-relevant chunks are pinned; tombstones; GATE-CHRON-12; the retention question OPEN-CHRON-4 | Chronicle lead | GATE-CHRON-12; GATE-CHRON-10 | BD-209, BD-095, N41-11 | A retention pass over a store holding one pinned and one unpinned cited chunk keeps the first and leaves a tombstone for the second, which the trace reports. M2 |
| RISK-44, High | Adaptation or tuning leaks into the final evaluation | A policy or controller identity that changed between discovery and confirmation; a training scene in an evaluation | The scene-split and design-split checks (GATE-ARENA-02, GATE-HAB-14); identities frozen in the qualification request | Arena lead | GATE-ARENA-02; GATE-HAB-14 | BD-235, N19c-15, N24a-31, BD-260 | An evaluation set containing one training scene is refused; a qualification request whose policy hash differs at confirmation is refused. M3 |
| RISK-49, Medium | Imported text carries instructions to an agent | A hostile string outside an untrusted-marked field | The untrusted annotation on every imported string; GATE-SAFE-02; the poisoned-import arm of GATE-TETHER-13 | Tether lead | GATE-SAFE-02; GATE-TETHER-13 | BD-304, BD-372, BD-373 | An imported model whose name field contains an instruction to the reader: every output carries the string only inside an untrusted-marked field. M3 |
| RISK-50, Medium | Attribution names a cause where none is stable | The placebo flip rate on reference failures | Abstention (GATE-AUTOPSY-05); double precision for forks; KILL-AUTOPSY | Autopsy lead | GATE-AUTOPSY-05; KILL-AUTOPSY | BD-282, N27a-14, N27b-11, BD-280 | A planted chaotic failure is given no cause: the attribution abstains and reports the placebo flip rate. M3 |

## Acceptance criteria

- [ ] `registries/risks.toml` contains the fourteen records above with no empty field; every `beads` and `detects`
      entry resolves.
- [ ] Every row names a planted instance and its first-readable milestone.
- [ ] With parts 1 to 3 the register holds exactly 59 records, RISK-01 to RISK-59 with no gap and no duplicate.
- [ ] The loader's rule test passes on these rows.

## Test plan

### Unit tests
`crates/frob-conformance/tests/risks_part4.rs`: `fourteen_rows_load`; `ids_match_expected_set`; `severity_counts`
(7 High: RISK-12, -34, -35, -36, -37, -38, -44; 7 Medium); `register_is_complete_01_to_59` (the union of the four
parts, with the totals 28 High, 29 Medium, 2 Low); `every_bead_key_resolves`; `thresholds_agree_with_gate_register`
(500 adaptive retries; α_i = α/[i·(i + 1)]).

### End-to-end script and logging
Covered by `scripts/e2e/registers_exit.sh` (N41-04).

### Known answers and negative controls
Known answers: 14 rows here; 59 in total; the severity totals. Negative controls: a duplicate identifier across
parts; a gap in the series; a row whose planted instance field is empty.

## Considerations

- The rows of this part are the ones a hurried swarm is most likely to violate without noticing, because each
  violation makes results look better. Their signals are therefore all produced by checks that run without anyone
  asking: the firewall gates, the mutation suites, the shadow recompute, the scorecard accounting.
- Whoever holds the qualification administrator scope is trusted (the residual of RISK-34); the owner decision on
  who that is belongs to the task chapter's decision bead.
- A true certificate of an uninteresting statement still reads as evidence (the residual of RISK-35); the review of
  primitive equations in a bundle is an open question with a fail-closed "unreviewed" marker.

## References

§41.3 (rows RISK-11, -12, -14, -15, -17, -34 to -39, -44, -49, -50), §41.4, §24.13, §12, §34.9, §34.11, §28.12 to
§28.14, §29, §37.
