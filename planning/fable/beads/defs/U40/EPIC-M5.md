---
key: "EPIC-M5"
title: "Milestone M5, proofs, reality, manufacture: what can be trusted and what can be built, off the critical path"
type: "epic"
priority: 3
milestone: "M5"
labels: ["m5", "delivery", "frontier", "gate"]
parent: "EPIC-PROGRAM"
deps: []
needs: []
related: ["EPIC-M6", "EPIC-M7", "EPIC-PROGRAM", "N40-02", "N40-04", "N40-05", "N40-06", "N40-23", "N40-27"]
gates: ["GATE-LEMMA-01", "GATE-LEMMA-02", "GATE-LEMMA-05", "GATE-QUAL-03", "GATE-TRAC-09", "GATE-HAB-03", "GATE-GROUND-06", "GATE-GROUND-10", "GATE-FOUNDRY-10", "GATE-WS-02", "GATE-GROUND-01", "GATE-GAUNT-01", "KILL-LEMMA", "KILL-REFLECT", "KILL-BAND", "KILL-GROUND", "KILL-INGEST", "KILL-TOPOPT", "KILL-PACKET", "KILL-TREEOP", "KILL-HARDCONTACT", "KILL-VARIATIONAL"]
plan_refs: ["§43.8", "§43.1", "§43.2", "§40.6", "§40.3", "§40.4", "§43.12"]
---
## Background

**Purpose.** M5 strengthens what can be trusted and what can be built: certificates for derived kernels and Lean
capsules (proofs), calibration against measurements of real hardware (reality), and a build packet from a
qualified design (manufacture). None of it shortens the time to a design decision, so none of it gates the
ten-fold tournament of M7.

**Why here, and why off the critical path.** These lanes turn a simulated answer into something a third party can
check, an engineer can build, and a bench can contradict. They depend on a stable engine and a qualified design
(M2 to M4), and they are independent of one another: each has its own kill gates and a fallback that ships under
its own name. A milestone whose every lane can fall back cannot stall the program, which is why its exit line is
the weakest of the nine: *the Reality lane has a record or a recorded `no-claim`*. `no-claim` is a legitimate
terminal state: a gate that needs a physical board, a bench measurement or a building scan is an import lane with a
recorded `no-claim` until the data exists. Planning estimate: 6 weeks (DESIGN). Critical path: no; may overlap M4.
Tier: mostly Frontier [F].

**Demonstrates.** NS-11 *A kernel with a certificate*: `frob prove kernel K-cay-step` returns a certificate that
the update stays on the group and conserves spatial momentum in free flight, checked by the symbolic engine's
proof kernel, with a Lean capsule re-checked where the theorem's dependencies allow; the certificate says nothing
about floating-point results, and its text says so. NS-12 *Your own kitchen*: a walkthrough video becomes a private
scene pack; scale is `Estimated` unless anchored; unseen surfaces are treated conservatively and listed as a
re-scan request; the scan never leaves the machine. NS-20 *A build packet*: a `.fbuild` packet with a bill of
materials against a pinned catalogue, faceted solid geometry, a tolerance stack that closes, fatigue margins from
simulated load histories, one link re-optimized to be 18% lighter at equal margin; claim product Estimated; four
flags all false. NS-21 *A calibrated reality band*: a cost of transport of 0.62 with a band of [0.55, 0.71] at 90%
nominal coverage from 20 calibration episodes, coverage varying by about six points at that sample size; a design
never built returns a refusal naming the missing calibrated family.

## Scope

In: certificate coverage; Lean theorem capsules; the reflection pilot; replay of a qualification campaign by the
offline auditor; the articulated variational integrator; the hard-contact batch law on `batch-cpu`; Ground
(ingest, excite, identify, predict, score, band, kit) on the synthetic rehearsal and the first real cell if data
is admitted; real-place ingestion; asset-populated interiors if their pilot funds them; the tree-operator route as
a pilot; Foundry (loads, size, optimize, check, tolerance, export).

Explicitly not in M5: the hard-contact law on GPUs, living-world physics, lifecycle compression (M6); firmware
export (M8); any bench hardware (optional import packages tied to no milestone).

What later milestones need from M5: nothing on the critical path. M6 reads the KILL-TREEOP and KILL-HARDCONTACT
decisions; M7 counts certificates that changed a decision.

## Technical approach

**1. The artifact.**

- Certificate coverage across derived kernels (`frob prove kernel`, `frob prove bundle`); Lean theorem capsules
  re-checked by the Lean checker sibling; the reflection pilot (`frob prove reflect`); replay of a qualification
  campaign by `frob-verify`.
- The articulated variational integrator on the referee and the `strict` profile; the hard-contact batch law on
  `batch-cpu`.
- Ground: `frob ground ingest`, `excite`, `identify`, `predict`, `score`, `band` and `kit`, exercised first on the
  synthetic rehearsal; the first real cell if data is admitted.
- Real-place ingestion: `frob scene import`, `frob scene fit`, `frob scene probe-request`; asset-populated
  interiors, if their pilot funds them; the tree-operator route as a pilot.
- Foundry: `frob foundry loads`, `size`, `optimize`, `check`, `tolerance` and `export`, ending in a `.fbuild`
  packet whose four flags are false.

**2. Gates with a part due at this exit (compiled register tables: 4 admission, 26 convergence).** Four of these
are tagged M4 by their chapters and may pass there: GATE-FOUNDRY-01 to -03 and GATE-GROUND-03. GATE-TRAC-09 is due
on `batch-cpu` (its rows are admitted from M3; the GPU rows are M6). GATE-WS-02 completes for all thirteen formats.
One row carries an M5 part that the plan's printed load table (25) does not count, with its recording bead (N40-02,
step 5): GATE-LEMMA-03 over every proof kind enabled by M5, those of M1 to M4 and `lean-capsule` (BD-381), recorded
by this exit's review (BD-412).

| Gate | Claim | Part due | Pass criterion in brief (TARGET) | Defined in |
|---|---|---|---|---|
| INT-FSIM-15 | Admission | M5 | Foundation: frankensim group `structure`. A tolerance allocation on a two-term stack reproduces the hand-computed variance split; a mesh written through `fs-io` reads back with the same hash; a STEP file above the stated bound is quarantined with a receipt, not parsed | §33.9 |
| INT-FLN-01 | Admission | M5 | Foundation: franken_lean. `fln` builds on a Linux host, re-checks and seals one trivial theorem from `lean/`, and the capsule checks from an empty environment | §33.6 |
| INT-FLN-02 | Admission | M5 | Foundation: franken_lean checker. The checker admits a true port identity and rejects its sign-mutated twin; axioms and coverage are listed | §33.10 |
| INT-FDGR-01 | Admission | M3 (schema fixtures); M5 (scan import) | Foundation: drone geometry reconstruction (pattern; subprocess) [F]. The schema fixtures validate; a geometry generation whose scale status is not metric is refused; coverage masks become the scene's unknown-space label; privacy inheritance applies | §33.15 |
| GATE-FOUNDRY-01 | Known answer | M5 | Cantilever deflection within 2% and stress within 5% after refinement; the planted overstress located with utilization 1.25 ± 5%. | §31.19 |
| GATE-FOUNDRY-02 | Known answer | M5 | Free-body residual at most 10⁻⁶ relative on the referee and 10⁻³ on single-precision tiers; superposition equals direct solves (PO-FOUNDRY-01); the assembled-peak and equal-limb mutations refused. | §31.19 |
| GATE-FOUNDRY-03 | Known answer | M5 | Fatigue block and rainflow sequence exact to 10⁻¹² relative; the window-split mutation reproduces its 10.7% and is caught. | §31.19 |
| GATE-FOUNDRY-04 | Known answer | M5 | Two-bar compliance within 10% and bar angle within 5°; the extracted surface meshes and passes the mesh audit on 20 of 20 fixtures. | §31.19 |
| GATE-FOUNDRY-05 | Design comparison | M5 | The stage-2 pilot: saving at least 7% (kill threshold, as in BET-29 and C-35) with 15% the target; the convergence test passes within five iterations; coarse solve at most 120 s per link class. | §31.19 |
| GATE-FOUNDRY-06 | Known answer | M5 | Planted-gain loop: fixed point within 10⁻⁶, λ̂ within 0.01; divergence and cycling refused. | §31.19 |
| GATE-FOUNDRY-07 | Known answer | M5 | Planted violations of wall, corner radius, tool access, overhang and undercut each detected with location and repair; clean parts pass; 20 of 20. | §31.19 |
| GATE-FOUNDRY-08 | Known answer | M5 | The worked allocation to 10⁻⁹; the zero-sensitivity and quadratic cases handled as specified; a correlated stack equals its hand value. | §31.19 |
| GATE-FOUNDRY-09 | Known answer | M5 | Tetrahedron round trip bit-exact; open shell and over-bound parts refused; mass properties from exported geometry within 10⁻⁶ of the model. | §31.19 |
| GATE-FOUNDRY-10 | Known answer | M5 | Packet schema-valid, every hash matching, flags present and false, cross-consistency of parts, geometry, nets and schedule; a mutated flag rejected by `frob-verify`; a writer killed at any file boundary leaves no packet. | §31.19 |
| GATE-GROUND-02 | Known answer | M5 | Clock recovery as in §30.21 in at least 99% of 1,000 resamples; both negative controls caught. | §30.22 |
| GATE-GROUND-03 | Known answer | M5 | The eight structural aliases are flagged, 8 of 8, and no fit is issued on an aliased set. | §30.22 |
| GATE-GROUND-04 | Known answer | M5 | Planted actuator parameters within 10% on at least 95 of 100 synthetic rigs; identified cards cut held-out trajectory error by at least 30% against prior cards; the omitted-law candidate is rejected in at least 90 of 100. | §30.22 |
| GATE-GROUND-05 | Known answer | M5 | The freeze-before-read order holds for every trial; the three protocol mutations are refused. | §30.22 |
| GATE-GROUND-06 | Known answer | M5 | Mean in-cell coverage within 0.01 of 19/21; fraction below 0.80 within 0.03 of 0.069; zero bands for the shifted cell, the short cell or the unbuilt design. | §30.22 |
| GATE-GROUND-07 | Known answer | M5 | The planted drift retires the calibration within 20 episodes in at least 90% of resamples; false retirement at most 5% over 10⁴ null runs; every scope change fires. | §30.22 |
| GATE-GROUND-08 | Known answer | M5 | As-built known answers of §30.21; the as-built variant has a new `ModelId`. | §30.22 |
| GATE-GROUND-09 | Known answer | M5 | A kit rebuilds the rehearsal on a clean machine inside its class; a private input is refused. | §30.22 |
| GATE-HAB-03 | Reality | M5 | One real place ingested end to end, privately, with its scale label and unknown regions respected, and at least one probe request emitted for it. A place for which no capture exists is recorded as `no-claim`, not as a pass. [F]. | §19.19 |
| GATE-LEMMA-01 | Proof | M5 | At least 80% of derived kernels carry a replayable instance certificate for each identity their manifest declares (BET-11). | §12.6 |
| GATE-LEMMA-02 | Proof | M5 | At least three Lean theorem cards compiled by stock Lean, re-checked by `fln`, and sealed; the capsule verifies from an empty environment. | §12.6 |
| GATE-LEMMA-03 | Known answer | M1; M2, M3, M4, M5, M8 (every kind enabled by that exit) | Every proof kind in use has one specimen that `frob-verify` accepts and at least three mutations it rejects; the four stance mutations, an unknown kind with consistent hashes and a changed quantifier are all rejected; the binary has no dependency outside the standard library and runs in an empty environment. | §12.6 |
| GATE-LEMMA-05 | Proof | M5 | The Farkas and rational LDLᵀ reflectors have soundness theorems compiled by stock Lean and re-checked by `fln`, and each proves its specimen by kernel evaluation within 60 s. | §12.6 |
| GATE-QUAL-03 | Known answer | M5 | `frob-verify` replays the levels, the commitments and, after close, the cluster derivation of a campaign; a reordered index, a changed request and a reused cluster are each rejected. | §24.17 |
| GATE-TRAC-09 | Fidelity | M5; GPU at M6 | CL-H on `batch-cpu`: residual at most 10⁻⁴ within eight passes on at least 95% of battery contact states and 10⁻⁶ under `strict`; zero creep on the incline below threshold; no separation velocity on a sliding block; agreement with the referee on the Solid rows of §18.16. | §18.17 |
| GATE-WS-02 | Known answer | M1, then at each new format; complete at M5 | The format registry round trips of §34.9. | §34.13 |

GATE-GROUND-10 (Reality: the first real cell, a frozen prediction scored on sealed data and a band issued) is tied
to no milestone. It is an import lane: it blocks nothing, its resting state is `no-claim`, and it is recorded when
data is admitted.

**3. Gates of later subsections due at this exit.**

| Gate | Part due at this exit | Statement (from the defining subsection) | Defined in |
|---|---|---|---|
| GATE-FOUNDRY-13 | whole | GATE-FOUNDRY-13 (Known answer, M5). On the 12 hidden briefs every outcome is a packet that passes M1 to M6 or a typed refusal naming a binding constraint (12 of 12); no cross-export difference exceeds 10⁻⁶; the planted unit error and the planted stale verdict are each caught in 100 of 100 runs; importing a record leaves the packet digest unchanged; a record with a wrong packet digest is refused. The fail-closed default without a reviewer is `review: no-claim`, and KILL-PACKET (§31.19) is unchanged. | §31.21 |
| GATE-FOUNDRY-14 | whole | Known answer ¦ Every envelope case in a packet has a re-extraction record. A planted fast-tier artifact (a stiffness spike that doubles one peak on the fast tier only) is relabelled `tier-sensitive` and replaced in 100 of 100 runs; a packet request holding a provisional case is refused in 100 of 100; the referee cost per packet is reported ¦ M5 | §31.22 |
| GATE-GROUND-11 | whole | Known answer. In the synthetic rehearsal of §30.21, the two explanations produce bit-equal samples on the +2 m/s pass and the fit reports "delay times speed plus offset = 40 mm" with no card update. With the reversed pass added, both the 20 ms delay and a planted 40 mm offset are recovered within their intervals, and the predicted separation is 80 mm to 10⁻⁹. A card delay update from the single pass is refused. M5. | §30.24 |

**4. Recurring gates.** GATE-GROUND-01, GATE-SAFE-01, GATE-TENFOLD-09, GATE-GAUNT-01, -02, -03, -05, -10;
GATE-GAUNT-08 at each funding decision (every package of this milestone holds Frontier work: none of WP-31, WP-32,
WP-68 and WP-70 to WP-72 is funded without a pilot record that has a falsifier, a computed ceiling, a fallback and a
result).

**5. Kill gates decided here.** In brief: KILL-LEMMA → certificates become exact-rational randomized tests recorded
as `Tested`. KILL-REFLECT → instance certificates stay `Certified` and reflection is parked. KILL-VARIATIONAL → the
free-body step as oracle and the default integrator everywhere. KILL-HARDCONTACT → fixed-iteration island solvers
with a residual flag, and the law kept as a specification and a model-form arm. KILL-GROUND, KILL-BAND, KILL-CLOCK,
KILL-ASBUILT, KILL-KIT → Ground keeps admission, frozen predictions, direct measurements and time-free metrics,
withdraws bands, and reports `reality: no-claim`. KILL-INGEST → a surveyed proxy authored in the interior grammar.
KILL-DRESSING → structural proxies and static clutter only. KILL-LOADS, KILL-TOPOPT, KILL-FAB, KILL-TOLERANCE,
KILL-PACKET → load cases declared by hand, beam sizing, advisory lints, worst-case boxes, and a packet reduced to a
parts list, mass properties and STL. KILL-TREEOP and KILL-SKEL-PORT → the dense factorization and a cap of 64
degrees of freedom.

| Kill gate | Trigger | Fallback, named in advance | Decided | Owner (consumer) |
|---|---|---|---|---|
| KILL-LEMMA | GATE-LEMMA-01 below 50% after one repair cycle | Certificates become exact-rational randomized identity tests recorded as `Tested`; the word certificate leaves kernel manifests | M5 | §12.6 |
| KILL-REFLECT | GATE-LEMMA-05 fails after one repair cycle, or no reflector's closure avoids the blocked modules | Instance certificates stay `Certified`; reflection is parked in Appendix C | M5 | §12.6 |
| KILL-SKEL-PORT | PO-SKEL-04 fails, or the tree-operator route of §16 is not funded (KILL-TREEOP) | No limb port packets; all dynamics go through the whole-model path | M5 | §14.15 |
| KILL-TREEOP | Whole-step gain below 1.15 at 64 degrees of freedom on the M2 profile | The dense factorization everywhere and a cap of 64 degrees of freedom | M5 | §16.6 |
| KILL-VARIATIONAL | The implicit solve costs more than twice a semi-implicit step on `batch-cpu`, or the articulated form fails its momentum row after one repair cycle | The free-body step is kept as a known-answer oracle; every rollout uses the default integrator | M5 | §16.7 |
| KILL-HARDCONTACT | GATE-TRAC-09 fails after one repair cycle | Fixed-iteration island solvers on `batch-cpu` with a residual flag; CL-H remains the referee's specification and a model-form arm under `strict` | M5 | §18.5 |
| KILL-DRESSING | A dressing item breaks the fixed launch shape or the determinism class of a batch, adds more than 25% to step cost, or exposes no failure class beyond the proxies on the M5 battery | Structural proxies and static clutter only | M5 | §19.7 |
| KILL-INGEST | GATE-HAB-03 cannot pass with a scale label of `witnessed` or better after one repair cycle | The place is authored in the interior grammar from a tape-measure survey, and the pack says it is a proxy | M5 | §19.13 |
| KILL-ASBUILT | GATE-GROUND-08 fails | Scans are not ingested; as-built data enters as scalar bench measurements, each a direct measurement of one card field | M5 | §30.22 |
| KILL-BAND | GATE-GROUND-06 or -07 fails | Bands are withdrawn; reports print `reality: no-claim` with the raw residual table | M5 | §30.22 |
| KILL-CLOCK | GATE-GROUND-02 or -05 fails | Real logs are scored on time-free metrics only; no crossing time, latency or delay is reported | M5 | §30.22 |
| KILL-GROUND | GATE-GROUND-04 fails after one repair cycle | No identification; Ground keeps admission, clocks, frozen predictions and bands on direct measurements with prior cards | M5 | §30.22 |
| KILL-KIT | GATE-GROUND-09 fails | The kit ships as an archive of inputs, records and hashes, with no reproduction claim | M5 | §30.22 |
| KILL-FAB | GATE-FOUNDRY-07 fails | Manufacturability findings become advisory lints; a packet records no manufacturability pass | M5 | §31.19 |
| KILL-LOADS | GATE-FOUNDRY-02 fails after one repair cycle | Load cases are declared by hand from statics with a stated dynamic factor; spectra and fatigue read `no-claim` | M5 | §31.19 |
| KILL-PACKET | GATE-FOUNDRY-09 or -10 fails after one repair cycle | Parts list, mass properties and STL only; STEP and harness deferred | M5 | §31.19 |
| KILL-TOLERANCE | GATE-FOUNDRY-08 fails, or band-extreme checks fail on more than a fifth of features in the pilot | Worst-case boxes per interface | M5 | §31.19 |
| KILL-TOPOPT | GATE-FOUNDRY-05 fails: saving under 7%, no convergence, or over budget | Beam-rung sizing supplies mass and inertia feedback; three-dimensional optimization becomes an offline option or is dropped; A-15 is unaffected | M5 | §31.19 |

**6. Exit rule.** Recorded by a fresh-context session (seed BD-412, procedure N40-05). The exit line: *the Reality
lane has a record or a recorded no-claim (GATE-GROUND-01, GATE-GROUND-10).*

**7. Dependency firewall.** Needs FrankenSim (structural, variational, as-built, manufacture), FrankenSymPy
(coverage), franken_lean (capsules). Uses if admitted: drone reconstruction (scan import). If missing: KILL-LEMMA;
KILL-REFLECT; KILL-INGEST; the Foundry kill gates.

**8. Owner input wanted.** OPEN-GAUNT-1 (which real-robot dataset, if any, enters the Reality lane first, with what
licence and provenance; default: `reality: no-claim` everywhere, and the second-simulator stand-in labelled
`synthetic`). OPEN-GROUND-2 (whether any bench hardware is acquired; default: none). OPEN-HAB-5 (which
reconstruction package and version is the import contract; default: the walkthrough twin package, and the drone
route when it publishes geometry). OPEN-FOUNDRY-2 (which parts catalogue and price data, under what licence;
default: card references and no prices) and OPEN-SKEL-2 (where card prices come from; default: prices labelled
`assumed`, the cost budget advisory, and a cost verdict refused under `--strict-data`).

**9. Stop or re-plan.** Nothing here stops the program: every lane has a fallback that ships under its own name,
and the exit is a record or a recorded no-claim. Lanes stop singly. Work on Lemma beyond the coverage gate is not
funded further before M7 if no consumed certificate has changed a decision by this exit (the count is taken in the
phase ledger of the ten-fold test; if it is zero, Lemma is evidence quality only and stays off the critical path).
Ground work beyond the rehearsal waits if no dataset has been admitted.

**10. No single failed bet blocks the exit.** BET-11 (certificate coverage is unaffordable) → exact-rational
randomized tests recorded as `Tested`. BET-28, or no data arrives → `reality: no-claim`, recorded as the state.
BET-29 (generative structure saves too little) → beam sizing and the structural margin analysis.

**11. Work packages.** (WP-69, the optional bench packages, is tied to no milestone; its sourcing may start at M0.)

| Package | Delivers | Crates | Milestones | Key gates |
|---|---|---|---|---|
| WP-31 | Lemma: certificate coverage and Lean capsules [F] | `frob-lemma`, `lean/`, `derive/frob-derive` | M5 | INT-FLN-01, -02; GATE-LEMMA-01, -02; GATE-QUAL-03 |
| WP-32 | Real-place ingestion [F] | `frob-scene`, `frob-conformance`, `frob-analyze` | M5 | GATE-HAB-03 |
| WP-68 | Ground: admission, clocks, identification, frozen predictions, bands, the kit [F] | `frob-ground` | M1 (the reality line), M5 | GATE-GROUND-01 to -09; GATE-GROUND-10 when data is admitted |
| WP-69 | Bench packages A to F of §30.17, optional imports | — | None; sourcing may start at M0 | GATE-GROUND-10 |
| WP-70 | Foundry I: loads, sizing and the structure loop [F] | `frob-foundry` | M5 | GATE-FOUNDRY-01 to -06; INT-FSIM-15 |
| WP-71 | Foundry II: manufacturability, tolerance and the build packet [F] | `frob-foundry`, `frob-cert` | M5 | GATE-FOUNDRY-07 to -10; GATE-WS-02 |
| WP-72 | Reflection pilot, variational integrator, hard-contact batch law, the tree-operator pilot [F] | `frob-lemma`, `lean/`, `frob-dyn`, `frob-contact` | M5 | GATE-LEMMA-05; GATE-TRAC-09 |

## Acceptance criteria

- [ ] Every part in the gate tables has a recorded value and a caught negative control, a kill decision with its
      fallback adopted, or (Reality gates only) a recorded `no-claim`.
- [ ] GATE-HAB-03 is `pass` for a real place ingested privately, or `no-claim` where no capture exists; it is never
      a pass by default.
- [ ] The 18 kill gates each have an ADR with an outcome.
- [ ] The synthetic rehearsal exercises every Ground verb and its evidence is labelled `synthetic`.
- [ ] A `.fbuild` packet is schema-valid with four flags false, and a mutated flag is rejected by `frob-verify`.
- [ ] NS-11, NS-12, NS-20, NS-21 run from a clean checkout, or are recorded as not demonstrated with the reason.
- [ ] The exit report for M5 is `exit-ready`; the exit record is written by a fresh-context session.

## Test plan

### Unit tests

Carried by member beads. Epic check: due counts for M5 are 4 admission, 26 convergence, 18 kill decisions, plus 3
late parts; GATE-GROUND-10 is listed outside the blocking set.

### End-to-end script and logging

`scripts/exit/review.sh M5`: `frob prove kernel K-cay-step --json` and `frob-verify` on its bundle on a host with
no GPU; the Ground rehearsal from ingest to band; `frob foundry export` and the packet audit; the private-scene
import with its privacy marks checked on every derived artifact. Logs carry evidence blocks, the `reality:` line
of every report, coverage figures beside 19/21, bundle digests, and the gates touched; no private-scene data is
logged.

### Known answers and negative controls

Mean in-cell coverage within 0.01 of 19/21; cantilever deflection within 2%; tetrahedron round trip bit-exact; at
least three Lean cards sealed; 80% certificate coverage (TARGET). Controls: a sign-mutated port identity rejected
by the checker; three protocol mutations of the freeze-before-read order refused; a writer killed at any file
boundary leaves no packet; a private input to a kit refused.

## Considerations

- The temptation at this exit is to read `no-claim` as failure and to fill it. The rule is the opposite: a place
  for which no capture exists is recorded as `no-claim`, not as a pass, and reports print the line.
- Privacy: real-place scans are private scenes; logs, kits and exports refuse them.
- INT-FLN-01 needs a Linux host; the Lean lane does not run on the Mac.

## References

Plan §43.8, §43.1, §43.2, §40.3 to §40.6, §43.12. Bets BET-11, BET-28, BET-29. Scenarios NS-11, NS-12, NS-20,
NS-21. Work packages WP-31, WP-32, WP-68 to WP-72.
