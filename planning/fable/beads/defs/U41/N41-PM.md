---
key: "N41-PM"
title: "registries/premortem.toml: the 27 failure stories and the six reasons every gate could pass while the claim fails"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "tenfold", "registry", "solid"]
parent: "EPIC-M0"
deps: ["N40-05", "N41-01", "N41-REF", "N41-RISK1", "N41-RISK2", "N41-RISK3", "N41-RISK4", "STD-E2E"]
needs: []
related: ["N41-04", "N41-KA", "N41-CX"]
gates: []
plan_refs: ["§41.1", "§41.2", "§41.4"]
---
## Background

A risk register lists hazards one at a time. A pre-mortem asks a different question: it is late 2029 and the
program failed; what is the story? Each story is told from the failure backwards. Its **earliest signal** is a
quantity the program already records, chosen so that it moves while the failure is still cheap; its
**countermeasure** names the mechanism that answers it; its **residual** is what remains after the countermeasure
works as designed. A second, shorter pre-mortem covers the case that is easiest to miss: every mechanism gate is
green and the ten-fold claim still fails, because a gate measures a mechanism and the claim is about an outcome.

This bead keeps both as data (`registries/premortem.toml`) read at every milestone exit together with the risk
register, so that the stories are not prose that nobody re-reads. Nothing in either table is a result: every
threshold is a TARGET repeated from its gate, and incumbent figures are FACTs to be re-read from their sources.

## Scope

In: the two tables as registry records, each linked to the risk rows that carry its mitigation; the reading of the
six outcome-level signals at each exit. Out: the risk rows (N41-RISK1 to N41-RISK4); the review procedure and its
record (N41-01); the measurements themselves.

## Technical approach

Records: `[[story]]` with `n`, `story`, `signal`, `countermeasure`, `residual`, `risks` (register rows), `cluster`
(`thesis`, `evidence`, `program` or `carried`); `[[outcome_reason]]` with `rank`, `reason`, `signal`, `answer`,
`first_readable`. The loader is the one of N41-01; a story whose `risks` list is empty, or names a row that does
not exist, is a load error.

**Table 1. It is late 2029 and the program failed. Why?** Stories 1 to 15 were carried from the previous version
of the plan with countermeasures corrected to the present mechanisms; 16 to 27 are newer: five about the thesis
(16 to 20), three about the evidence (22 to 24), four about the program (21, 25 to 27).

| # | Failure story | Earliest signal | Countermeasure | Residual | Register rows |
|---|---|---|---|---|---|
| 1 | The mathematics became the product: elegant algebra crates grew and no robot walked | Layer-1 crates grow while the M1 exit slips; a construction with no verb or no failing test | Doctrine rules D-2 and D-6; milestone exits are robots moving and answers accepted; a construction missing any of its four obligations (type, verb, failing-when-wrong test, measured payoff) moves to the killed-idea register | The temptation is constant | RISK-33, RISK-18 |
| 2 | FrankenSim moved weekly and adapter churn consumed the effort | Upstream breakage above 20% of working days in a four-week window (FACT context: 409 commits touched `fs-mbd` in thirty days) | Pinned commit, the airlock, the compatibility train; KILL-FSIM with the worker process first and the lift second | The lift is priced against a closure of about 1.5 million lines, not against two crates | RISK-01, RISK-22 |
| 3 | GPU kernels passed tests and diverged in the field, so nobody trusted fast results | Divergence-ledger entries without explanations; a world whose digest depends on the batch it ran in | The CPU-versus-device check on every driver or compiler change; the fingerprint gate; teleport of a world to the CPU; batch-composition invariance (GATE-SPARK-15); decision-level equivalence (GATE-SPARK-16); the promotion rule | Class D3 comparisons are statistical | RISK-04, RISK-48 |
| 4 | Throughput landed far under the parity band and agents stopped waiting | The M2 profile; a paired lower bound under 0.67 of the incumbent measured on `trj` | Profile first; KILL-PARITY; KILL-KIR; the ladder reduces how many steps a decision needs | Every figure is a TARGET until M2 | RISK-30, RISK-06 |
| 5 | Policies exploited contact compliance and promotions kept failing | Promotion failure rate; `energy-created` flags; the share of finalists typed `model-conditional` | The trust ladder, the energy audit and the model-form rank check; the exploitation detectors; KILL-CONTACT with the second compliant law | Convex compliant models have real artifacts | RISK-05, RISK-47 |
| 6 | Baseline controllers failed on new morphologies, so every design needed training after all | The trend of GATE-REFLEX-02 on the corpus; the share of designs reported `not-synthesizable` | Three-outcome reporting; KILL-BASELINE; the sampling controller as fallback (BET-27); behaviour-cloning warm starts | Novel bodies are hard | RISK-17 |
| 7 | The agent surface was too large and agents got lost | Failures of GATE-TETHER-01 and GATE-TETHER-13; serial turns per accepted answer | Clusters of at most seven tools; `session.focus`; `next_actions`; `robot-docs`; the surface A/B of BET-10 | The registry is large even in clusters | RISK-49 (surface), RISK-33 |
| 8 | The swarm closed beads that proved plumbing | Shared close reasons; a gate without a caught negative control; a PASS recorded by the session that implemented the capability | The rules against false progress; GATE-GAUNT-01 at every exit; typed close reasons | Needs discipline at every exit review | RISK-54; tracker hygiene (N41-16, N41-17) |
| 9 | Interiors were toy boxes and results did not transfer to real places | Reviewer inspection of generated scenes; the share of dimensions labelled `assumed` | Cited norms (OPEN-HAB-1); hazard registries; scene acceptance; real-place ingestion | Realism is unbounded work | RISK-41 |
| 10 | Sim-to-real was never tested and claims overreached | Any report without a `reality:` line | The seven claims kept apart; GATE-GROUND-01 at every exit; `reality: no-claim`; card provenance; the no-authorization flags | No hardware is in scope | RISK-32, RISK-43 |
| 11 | "Certified" became a marketing word while the proof tools stayed immature | Certificates without replay hashes; the word "verified" in any output | The wording rule; `frob-verify`; KILL-LEMMA; the record of whether a certificate changed a decision | Depends on the proof siblings maturing, which is upstream work of this program | RISK-35, RISK-07, RISK-16 |
| 12 | Single precision failed in large scenes and long runs | GATE-SPARK-05; the share of worlds marked `Degraded: ill-conditioned`; the guarded-predicate fallback rate | Anchored positions, diagonal scaling, guarded predicates with an exact fallback; half precision is not built | Near-rigid cards stay outside `f32` | RISK-04, RISK-40 |
| 13 | The GPU mandate and the suite's no-FFI doctrine were never reconciled, and work stalled on policy | OPEN-SPARK-1 still open at M2 | The annex design; the question is put to the owner at M0; M2 has a variant with no GPU | Owner decision | RISK-53, RISK-10 |
| 14 | The Metal gateway's limits throttled the Mac tier | Step-plan launch counts and blocked host time on Metal; GATE-SPARK-04 | Packed slabs and one batch per chunk; upstream work item FD-FT-01; KILL-METAL, after which the Mac keeps the world bank, proof, replay and `batch-cpu` roles | A Metal binding of our own needs an owner decision (OPEN-SPARK-5) | RISK-08 |
| 15 | The plan itself became the delivery log and stopped being readable | The plan file growing after conversion to beads | The hygiene rule that the delivery log lives in `docs/delivery/` | See story 27 | RISK-54 |
| 16 | The ten-fold claim failed on quadruped missions because incumbents already train in minutes | The M0 phase ledger: time to threshold near the published 410 s on one RTX 4090; a ceiling under ten for the class | Ceilings stated before anything is built; the class is given a range of 3 to 10; GATE-SPARK-21 measures the denominator first; BET-13 fixes the wording for a ratio under 3 and for a ratio between 3 and 10; the partial gate at M3 | The quadruped ceiling cannot be engineered away, and the class may leave the claim | RISK-25 |
| 17 | A composed incumbent layer tied | Ties already on record from the plan-time pilots (a lifted linear program, a dependency graph); the composed arm within a factor of 1.5 at the M3 partial gate | BET-14; doctrine rule D-17; KILL-AUGUR-TIE; KILL-FAMILYCERT; the layer ships over that engine for the stratum, and engine work continues only for gaps internal to an engine | The engine's share of the product shrinks | RISK-26 |
| 18 | Rank agreement between rungs was low, so most designs were trained anyway | Verdict flips after finalist training; top-weighted τ under 0.5 in the pilot on incumbent tools | The rank-agreement experiment runs first; GATE-REFLEX-04, GATE-CORTEX-12, GATE-ATEL-07; `discard-only` mode; the audit lane and the planted winner; KILL-RANK; training at parity (GATE-CORTEX-05) | With most designs trained, the compute ratio is near 1 | RISK-27, RISK-37 |
| 19 | Family policies did not transfer | The pre-funding pilot; return with no adaptation under 70% of specialists; OPEN-CORTEX-1 at its default when M4 opens | KILL-FAMILY; inheritance for parameter edits; the baseline and sampling rungs; the variant of M4 with no family policy | The class is decided by specialists, and the humanoid range narrows | RISK-28 |
| 20 | Mixed batches lost to per-family templates | The M1 pilot of padding overhead and bucket count; the template arm expressing at least 95% of the mutation corpus | The head-to-head with outcomes declared in advance; KILL-PHYLUM and KILL-BUCKET; parameter variants within a topology are Solid and unaffected | Two performance differentiators are withdrawn | RISK-29 |
| 21 | The plan's breadth starved the spine: forty-three crates, nine milestones and more than four hundred gates spread a swarm thin | Crates that hold only Frontier or Moonshot content gain lines while an M1 or M2 exit slips; more than four Moonshot items active; a work package funded without its pilot | A tier on every mechanism; features that select a tier are off by default; rule D-6 and the cap of four; pilots before funding; M1 needs three foundations; the program gate; exits defined by demonstrations and not by gate counts | Breadth is the brief, and the cap is enforced by people | RISK-33 |
| 22 | Qualification was gamed | A campaign with many requests and no close; a receipt field outside the listed set; a denominator that shrank | The blinded lane: sealed clusters, pre-committed requests, α_i = α/[i·(i + 1)]; GATE-QUAL-01 under 500 adaptive retries; the firewall gates GATE-QUAL-02 and GATE-SAFE-03; scorecard accounting (GATE-ARENA-07) | Whoever holds `qual:admin` (the qualification administrator scope) is trusted | RISK-34, RISK-15, RISK-44 |
| 23 | Certificates were issued for the wrong statement | A bundle whose embedded statement differs from the query; a changed quantifier accepted with consistent hashes | Rule D-12; the bundle embeds the canonical bytes of the statement; the mission identity changes with any quantifier; the mutation suites of GATE-LEMMA-03 and GATE-AUGUR-01, -06; KILL-AUGUR-FALSECERT; one false certificate disqualifies an arm of the ten-fold test | A true certificate of an uninteresting statement still reads as evidence (OPEN-AUGUR-8) | RISK-35, RISK-36 |
| 24 | A foundation defect surfaced late | A known answer or a baseline that cites a module before its admission test has passed | Four reported defects recorded as facts to recheck; the admission gates INT-FSIM-05, -06, -07 and INT-FNP-02; an admission is never inherited; the file-by-file tests run at M0 | Unreported defects exist | RISK-24, RISK-58 |
| 25 | The toolchain train stalled: the siblings could not be brought onto one toolchain, and a merge was blocked | A failing cell in the matrix of INT-TOOLCHAIN-01; two asupersync packages in `cargo tree -d` | One toolchain T chosen at INT-TOOLCHAIN-01, with candidate (b) in reserve; FD-FMCP-01 and FD-ASUP-01 done at M0; KILL-MCP's fallbacks; the `frob-units` escape hatch; `check-deps` | Four nightlies and one stable release are the starting state (FACT) | RISK-02, RISK-03, RISK-21, RISK-57 |
| 26 | Calibration data never arrived | No dataset admitted when M5 opens; OPEN-GROUND-1 and OPEN-GROUND-2 at their defaults | Reality is an import lane: a synthetic rehearsal, frozen predictions, a published-anchor comparison at M3, bench packages that may be sourced from M0; the resting state is `no-claim` and blocks nothing | The Reality claim stays empty | RISK-43 |
| 27 | The document itself became unreadable | A threshold stated two ways; an identifier defined twice; a reader who cannot tell the spine from the frontier | Reading paths; tier tags; the gate register as the only authority for a threshold; registries generated from tables; the plan stops growing at conversion to beads | The plan is long by design | RISK-54 |

Three clusters, one sentence each. Stories 16 to 20 are the thesis failing lever by lever; no single lever delivers
the result, and what the design adds is that each lever has its own early experiment and a fallback that ships
under its own name. Stories 22 to 24 are the evidence failing, and they share a remedy: the party that proposes a
result never checks it. Stories 21 and 25 to 27 are the program failing under its own weight, and their remedy is
structural: M1 is small, the upper tiers are off by default, and progress is delegated to beads and registries.

**Table 2. Every gate passed and the claim still failed.** Each reason could hold with every convergence gate
green, so each is tied to a measurement that is not a mechanism gate. They are ranked by how early the signal is
available.

| Rank | Reason | Earliest signal | Where it is answered | First readable |
|---|---|---|---|---|
| 1 | Nothing confirmable existed: no metric, no incumbent arm, no neutral check | No incumbent phase measurement on record at the M0 exit | The metric is defined before anything is built; the ten-fold test defines the arms and the adjudicator; GATE-SPARK-21 is an M0 exit condition | M0 |
| 2 | Amdahl: incumbent training takes minutes, and the agent's latency is common to both arms | Training under about 90% of a measured incumbent cycle in the M0 ledger | Ceilings per mission class; many designs per turn; the phase ledger of the ten-fold test | M0 |
| 3 | The cheap rung reorders designs, so most are trained after all, on a training rung that is only at parity; worst on stairs, rubble and bipeds, where the multiplier would be largest | Verdict flips after finalist training; the trained fraction above about 1% | BET-17; the rank-agreement gates; `discard-only`; classes expected to be hard are sampled on purpose | M3 with the pilot on incumbent tools; M1 if the rank-agreement experiment is approved then |
| 4 | The referee could not run interior or tracked scenarios, so results stayed `provisional` | The share of standard-battery scenarios the referee completes at M2 (GATE-FID-03) | The transactional step with witness migration; the `strict` profile as adjudicator; BET-34 | M2 |
| 5 | Common-mode error: every tier shared `generic` and `assumed` cards, so tiers agreed with each other and accepted decisions failed neutral adjudication | The share of card fields that are `assumed` or `generic` in benchmark designs | Card provenance on every report; the published-anchor comparison at M3; an evaluator neither arm trained in | M3, when the benchmark corpus exists; per design from M1 |
| 6 | Structure helped agents only slightly | The ratio of restricted mean times in the surface A/B (GATE-ATEL-03), which needs an effect size and not merely a win | The threshold of 1.5 on at least ten briefs (BET-10); the verbs that showed no effect lose their speed claim | M4 |

**Reading at an exit.** The exit runner prints, for each story, whether any of its register rows has a fired signal
(from the risk review), and for each outcome reason whose first-readable milestone has been reached, the measured
value of its signal with the artifact that holds it. A reason whose signal is adverse (for example, no incumbent
phase measurement at the M0 exit) is listed under "outcome-level warnings" in the exit record and must be answered
there in writing. An exit record with an adverse reason and no written answer fails the exit: the runner exits 3
with `reason=premortem-warning-unanswered` (harness reason of N41-REF), naming the reason's rank and its signal.
This bead adds the two readings and that check to the exit runner of N40-05 as one attached check, by a record
of `scripts/exit/checks.toml`: `id = "premortem"`, `owner = "N41-PM"`, `step = 9`, `since = "M0"`, `after =
"risk-review"` (it reads the fired readings of `risk-review.json`), `command = "scripts/exit/premortem.sh"`. The
crate `frob-conformance` has no binary of its own name (BD-025, ADR-GAUNT-01), so the reading is an example
target that this bead declares, `crates/frob-conformance/examples/premortem.rs`, and the wrapper, run by the
runner as `scripts/exit/premortem.sh <Mn> --root <dir> --json`, calls `cargo run -p frob-conformance --example
premortem -- read <Mn> --root <dir> --json`. Under the runner's check contract that command prints one object
`{check, milestone, section, blocks, warnings}` and exits 0 whenever it ran. Its section holds the two readings;
an adverse reason with a written answer is a warning, and one with no answer is a block whose reason is
`premortem-warning-unanswered`, which is how the runner comes to fail the exit as stated above (a check that
exited non-zero would be read as `not-run`). It does not change the runner's other steps.

## Acceptance criteria

- [ ] `registries/premortem.toml` holds 27 stories and 6 outcome reasons; every story lists at least one register
      row that exists; every outcome reason names a measurable signal and a first-readable milestone.
- [ ] The exit runner prints the two readings in every exit record from M0 onward, through the attached check
      `premortem`, which runs after `risk-review`.
- [ ] At the M0 exit, reason 1 is evaluated from the gate register: with no recorded incumbent phase measurement
      the warning is present; with the measurement recorded it is absent.
- [ ] An exit record carrying an adverse reason with no written answer fails with
      `premortem-warning-unanswered`; the same record with the answer passes.
- [ ] The three cluster statements are in the registry header and are printed with the stories of each cluster.
- [ ] `scripts/e2e/premortem_exit.sh` passes, and fails under its plant `answer-not-required`.

## Test plan

### Unit tests
`crates/frob-conformance/tests/premortem.rs`: `counts_27_and_6`; `every_story_links_existing_risks`;
`clusters_partition_stories` (carried 1 to 15; thesis 16 to 20; evidence 22 to 24; program 21, 25, 26, 27);
`outcome_reason_1_warns_without_ledger` and `..._clears_with_ledger` on a synthetic gate register;
`outcome_reasons_have_rank_and_milestone` (ranks are exactly 1 to 6 and each record names the first exit at which
its signal can be read); `unanswered_warning_fails_exit` (an M0 exit record with reason 1 adverse and an empty
answer gives `premortem-warning-unanswered`; with a written answer it passes); `story_rows_resolve` (story 25
names RISK-02, RISK-03, RISK-21 and RISK-57, all present in the loaded register); `premortem_as_exit_check`
(through the runner of N40-05 on a scratch repository: the check runs after `risk-review`, a story whose row
RISK-02 fired is printed as fired, an answered adverse reason is a warning and an unanswered one a block).

### End-to-end script and logging
`scripts/e2e/premortem_exit.sh` (this bead; header `bead=N41-PM milestone=M0 labels=delivery,registry
plants=answer-not-required`) sources the shared helper of STD-E2E and uses only this bead and its dependencies: a
scratch repository root holding copies of the registries and a scratch M0 exit directory (no registry of record
is changed; the script claims no gate). Steps: (1) `cargo run -p frob-conformance --example premortem -- read M0
--root <scratch> --json` with no incumbent phase measurement in the scratch gate register and no written answer:
exit 0, reason 1 listed as adverse, one block with `premortem-warning-unanswered` naming rank 1 and its signal;
(2) the same with a written answer in the scratch exit record: a warning and no block; (3) the same with the
measurement recorded in the scratch gate register: neither; (4) `scripts/exit/review.sh M0` of N40-05 on the
scratch root of step 1: the check `premortem` runs after `risk-review`, its section is in the exit record, and
the verdict is `not exited` naming the check; (5) a scratch register in which RISK-02's reading is `fired`: story
25 is printed as fired. With the plant `answer-not-required` (a reading built to treat an unanswered adverse
reason as a warning) step 1 has no block and the script must fail. It writes
`target/e2e-logs/premortem_exit/<utc>.ndjson` and a transcript; every line has `ts`, `script`, `bead`, `step` and
`event`; `cmd` lines carry the exact command line, exit code and wall time; one `metric` line per readable
outcome reason gives its measured value, unit and source artifact; one `assert` line per warning gives the
expected and observed state; a `refusal` line carries `premortem-warning-unanswered` with the rank and the first
repair; the digests of the real registries before and after are logged (equal); on failure the log names the
first reading that differed and the scratch path; a final `summary` counts assertions passed and failed.

`scripts/e2e/registers_exit.sh` (N41-04) later walks whole exits over the three registers together and reads the
same two readings there.

### Known answers and negative controls
Known answers (K41-01, K41-02): the counts; the cluster partition; the thresholds quoted (20%, 0.67, 0.5, 70%, 95%,
1.5, 500, 90%, 1%), compared with the gate register where a gate owns them. Negative controls: a story with an
empty `risks` list; an outcome reason with no signal; an exit record that omits a readable reason; X41-48, an M0
exit with no incumbent phase measurement on record and no written answer.

## Considerations

- The stories overlap the register on purpose: a story is a causal chain, a risk row is one link. The link table
  keeps them from drifting apart.
- Residuals are not defects to fix; they are what an exit review should expect to remain true.
- New stories are appended with the next number; none is renumbered or deleted, and a story that can no longer
  happen is marked retired with the reason.

## References

§41.1 (the 27 stories and the three clusters), §41.2 (the six reasons), §41.4.
