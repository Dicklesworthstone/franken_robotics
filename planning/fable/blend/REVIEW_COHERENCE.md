# Coherence review of the Phase 1 part files

Reviewed: `parts/P00.md` to `parts/P12.md` (state of 2026-11-21, 21:21), against `INTEGRATION_CHARTER.md`,
`PHASE2_BRIEF.md` (R1 to R10 and the milestone skeleton), `LEDGERS.md`, `GATES_INVENTORY.md`, `COLLISIONS.md`,
`VERBS_INVENTORY.md`, `APPENDIX_O_REFUSALS.md`, the audit file `AUD02-findings.md`, and the sibling code under
`/Users/jemanuel/projects/`.

Method. The charter, the brief, both inventories and `P00.md` were read in full. Every other part was searched
with targeted patterns and read in the ranges the searches pointed to; no claim below rests on an unread
passage. Line numbers are those of the part files. Text inside a part's `<!-- LEDGER -->` block is ignored unless
a finding says otherwise, because the lead strips it. Every ORIGINAL span in section 2 was checked by script to
occur in the named file; it occurs exactly once unless the entry says "replace all".

Severity. **S1**: two binding texts cannot both be implemented, or a gate cannot run as written. **S2**: a number,
name, milestone or statement disagrees and one correction settles it. **S3**: minor.

Counts: 9 at S1, 23 at S2, 12 at S3. 44 findings.

## 1. Findings, ordered by severity

| ID | Sev | Check | Files and lines | The conflicting texts | Which is right, and why | Fix |
|---|---|---|---|---|---|---|
| F01 | S1 | 1 | P09:41, 481, 484–493, 1181, 1195 against P00:918 (BET-17), P03:410–412, P06:802, P06:1231–1233, brief R6 | §28.10: "`selective` … τ on the top band ≥ 0.5", "`discard-only` … 0.2 ≤ τ < 0.5; or the stratum is unmeasured", "`uninformative` … τ < 0.2". §16.9, §22.18 and §23.11: at least 0.7 selects, "Between 0.5 and 0.7 it discards only", "Below 0.5 it is not a rung (FROB-REF-537)"; GATE-CORTEX-12: "a rung without a record is refused". | The 0.7 and 0.5 pair. R6 fixes it, BET-17 states it, and three chapters use it. §28 lets a rung select at 0.5, discard down to 0.2 and discard with no measurement at all, which R6 forbids ("below this the cheap rung may not discard"). BET-17 is one of the three bets the thesis turns on. | Nine substitutions in P09 (section 2, F01). KILL-RANK's fallback in §28 changes with it, since "`discard-only` everywhere" after a failed measurement is what R6 excludes. Close OPEN-ATEL-2. |
| F02 | S1 | 2 | P01:2161, 2177–2196 against P07:630–648 | Two registries, each called closed. §12.4: `farkas-dual`, `conic-dual-strict`, `witness-replay`, `partition-cover`, `alias-witness`, `product-unreachability`, `statistical-receipt`, `interval-sign` and twelve more. §25.5.2 (`frob.cert.kinds/1`): `lp-farkas`, `conic-farkas`, `lp-primal`, `partition`, `alias-pair`, `product-unreachable`, `sample-coverage`, `interval-box` and seven more. Both say the auditor rejects an unknown kind. The stance witness is `witness-replay` in §12.4's bundle and `lp-primal` in §25's known answers. | One registry, in §12.4, since §12 owns the `.frobcert` container (charter §5, P12 §34.9) and §25 itself says the container is fixed there. Use §25's spellings for the eight kinds both list: §25 uses them in fixtures, gates and known answers; §12 uses its own only in one table and one example. | Section 2, F02: eight renames in P01, one heading change in P07, and four rows to add to §12.4 (`taylor-box`, `sos-gram`, `symmetry-transport`, `composition`). §12.4 keeps its kernel-level kinds (`poly-identity`, `rational-ldlt`, `interval-root`, `complex-connectivity`, `tube-inclusion`, `rewrite-trace`, `orbit-count`, `group-enumeration`, `energy-budget`, `alpha-root`, `lean-capsule`). |
| F03 | S1 | 2 | P11:144–153, 753 against P08:1173–1179, 1642–1648; P10:504–510, 1069–1075; P07:142–148; P01:850, 1967; P06:1380; P03:544–545; P09:35, 41, 230, 1251–1252; P08:682 | The registry of §32.6 and the owner chapters name different verbs for the same actions. Autopsy: `interventions`, `run`, `diff` against `intervene`, `attribute`, `diverge`. Nemesis: `rare`, `adopt`, `separate` against `bound`, `hardset`, `oracle`, `audit`, `cover`. Ground: `import`, `calibrate`, `compare` against `ingest`, `identify`, `score`, `predict`, `excite`, `kit`. Foundry: `structure`, `fatigue`, `packet`, `firmware` against `loads`, `size`, `optimize`, `check`, `export`, `lifecycle`. Bundles: `cert.bundle`, `cert.check` against `prove.bundle`, `prove.verify`. One action, three names: `world.separate` (§10), `mission.separate` (§25), `nemesis.separate` (§32). Model-form rank check: `eval.rankcheck` (§23), `fidelity.modelform` (§16). Rank-agreement record: `fidelity.rank` (§16), `eval.rank-agreement` (§28). Fixed-n pre-analysis: `eval.preanalyze` (§24), `eval.compare --plan` (§28), `study.plan` (§32). Forks and first divergence: `autopsy.fork`, `autopsy.diverge` (§27) against `run.fork`, `telemetry.divergence` (§29). `experiment.suggest` has two owners and two kill gates with different fallbacks (KILL-VOI, P09:1197; KILL-AUGUR-PROBE, P07:2086). | The owner chapter's spelling, by the rule of R1: its gates, refusal tables and known answers already use it, and §32's own ledger says the semantics are the owner's. Where two owner chapters collide: `mission.separate` (§25 owns the query and KILL-AUGUR-SEPARATE); `fidelity.modelform` (§16.9 defines the variants); `eval.rank-agreement` (§28 owns KILL-RANK under R1); `eval.preanalyze` (§24 owns the study types; `study.plan` calls it); `run.fork` and `telemetry.divergence` (§29 owns fork classification). | Section 2, F03: seven row edits in P11 §32.6, one transcript line, and four renames in P01, P06 and P09. Then remove `autopsy.fork` and `autopsy.diverge` from the table of §27 (Autopsy starts from a `run.fork` handle), make `fidelity.rank` in §16 a pointer to `eval.rank-agreement`, and give `experiment.suggest` one owner (§25, with §28 as consumer) and one kill gate. |
| F04 | S1 | 1 | P03:262–279 against P05:1296–1306 and 1503–1513; P03:845 | §16.6 "Capacities per reference workload … Workload (§36)": quadruped C = 24, R = 128, 27 kB; humanoid C = 40, R = 224, 72 kB. §36.3 "Reference workloads": Q has C = 32, R = 160, about 35 kB; H has C = 64, R = 320, about 97 kB. §17.7 uses "C = 32, R = 160" for the 18-DoF bucket. `Degraded` budget: "at most 10⁻⁴ of all world-steps" (§16.6, GATE-KIN-07) against "fewer than one world-step in 10⁵" (§36.3). Two open questions own the numbers (OPEN-KIN-3, OPEN-SPARK-3). | §36.3 for C and R: §16.6 says its table is §36's workloads, §17.7 inside the same part already uses §36's values, and every throughput row and the byte model of §36.10 are computed on them. GATE-KIN-07 for the `Degraded` budget, since a gate holds the threshold (§0.3). | Section 2, F04: two table rows in P03 and one sentence in P05. Name one open question as owner. |
| F05 | S1 | 4 | P06:799, P02:1078 against brief §3 (M1, M2, M3, M4) and P03:267–272 | "GATE-REFLEX-01 … Every template stands for 10 s and traverses 5 m of flat ground under its baseline controller on `referee` and `batch-cpu`. … M1"; GATE-SKEL-03: "Each of the eighteen archetypes … stands or rolls under its baseline controller (with GATE-REFLEX-01)". The brief: M1 is CPU only with "first archetypes"; the batched step on `batch-cpu` lands at M2; "baseline controllers for every archetype" at M3; tracks and hybrids at M4. §16.6: a tracked robot does not fit the referee's 21-contact cap "in normal driving". | The brief. As written the M1 gate needs a tier that arrives at M2, controllers that arrive at M3, contact laws that arrive at M4, and a referee run the referee refuses. | Section 2, F05. The set of "first archetypes" must also be named somewhere (section 5, item 2). |
| F06 | S1 | 2 | P04:3, 125; P10:945, 963; P07:68; P12:1068–1077 | Schema names: `frob.fscene/1` and `frob.fclear/1` (§19), `frob.fbuild/1` (§31) against `frob.scene/1`, `frob.clearance/1`, `frob.build/1` (§34.9). Owner of `.fmission`: "`frob-mission` … The `.fmission` schema and admission" (§25) against `frob-spec` (§34.9). Card kinds: "Three kinds: actuator, interface, material" (§34.9) against transmission and battery cards (§15), optical, sensor and payload cards (§19, §20), soil cards (§18). | §34.9 for names and owner: it is the format registry (ADR-WS-03), its names follow the pattern R8 fixed (`frob.design/1` for `.fbot`), and it gives the reason one crate must parse both requirement formats. The chapters for card kinds: the registry row repeats the charter's short list and the chapters use more. | Section 2, F06: replace all in P04 and P10, one cell in P07, one cell in P12. |
| F07 | S1 | 1 | P00:919 (BET-18); P03:1180, 1189; P05:444, 796, 1319 | One experiment, five thresholds. BET-18: "mixed throughput per degree of freedom is below 0.33 of the single-design rate; or 1,024 variants … cost more than 1.3× the homogeneous step". GATE-PHYLUM-03: "at least 0.6 of homogeneous world-steps/s". GATE-SPARK-10 and §36.3 row 10: "at least 0.70 of the harmonic mean of its homogeneous rates". KILL-PHYLUM and KILL-BUCKET: below 0.5. No gate tests 1,024 variants (GATE-PHYLUM-02 uses 512, GATE-KIN-06 uses 256). | 0.70 as the tolerance and 0.5 as the kill. §36.3 is the table of binding tolerances, both kill gates agree on 0.5, and the published comparator is 0.86 of a harmonic mean, so that is the unit. BET-18's 0.33 and 1.3× have no gate and no source. | Section 2, F07: one clause in P00, one in P03. |
| F08 | S1 | 2 | P03:302 against P01:1564–1576, 1603–1608, 2433 | §16.7 step 2: "the momentum that the right-trivialized derivative of τ at −h·ξₖ assigns to 𝕄·ξₖ equals μ⁺". C-36: "Let Dcay_x be the left-trivialized differential … (Dcay_{−hξ_k})⁻ᵀ·𝕄ξ_k = μ⁺", with the negative control "evaluating step 2 with the differential at +hξ_k … destroys the energy bound", and the catalogue entry "right-trivialized differential in the variational solve". | C-36. For a retraction with τ(−x) = τ(x)⁻¹, the right-trivialized differential at −hξ equals the left-trivialized one at +hξ, so §16.7 specifies exactly the form that C-36's negative control exists to catch. C-36 carries the derivation and the test; §16.7 says it follows §11. | Section 2, F08. |
| F09 | S1 | 4 | P07:2048, 2049, 2053; P00:1137–1148 (NS-13, "M1 on the CPU") against P11:147 and P09:1200–1204 | GATE-AUGUR-02 (M1): "the passive-spring amendment withdraws the cell"; GATE-AUGUR-03 (M1): "the three-gain amendment withdraws every positive cell"; GATE-AUGUR-07 (M1): "10 scripted docking and stance amendments"; NS-13: "`result.revalidate` withdraws the earlier inner set". Registry: "`result` … `revalidate` … M4". §28.23 milestone line: dependency grades of GATE-ATEL-11 at M3, the rest at M4. | The M1 gates, because the brief puts the first mission slice with `frob-cert` and `frob-verify` at M1 and the stance and docking amendments are its known answers. The registry and §28 must supply dependency-keyed revalidation of mission cells at M1. | Section 2, F09: one cell in P11; add "the dependency grades of GATE-ATEL-11 on mission cells at M1" to the milestone line of §28.23. |
| F10 | S2 | 2 | P03:1469–1472, 1682 against P04:766–768, 1219; brief R1; P00:937 (BET-36) | Two different kill gates share the name KILL-FILM. §18.8 kills the film-friction law [S]: "the law changes fewer than 10% of episode outcomes … or adds more than 5% to step time", fallback "static condition cards … on the lockstep tiers". §19.10.7 kills the advected film [F]: "fails its mass gate …, or costs more than 15% of the step, or … rank correlation of 0.95 or above", fallback "static film-friction patches". R1 lists them as one gate with an owner and a consumer. BET-36 gives a third set: "fewer than 20% of kitchen rankings; or … more than 25% to training time". | They are two gates. Merging them under R1 would delete one trigger and one fallback. §18 keeps the name (R1 names it owner); §19's gate needs its own. BET-36 should cite both. | Section 2, F10: rename in P04 (two places), rewrite BET-36's falsifier. Correct the R1 entry in the gate register. |
| F11 | S2 | 5 | P11:877 against P11:431–444, P12 INT-FMCP-01 (GATES_INVENTORY row), brief R3 | KILL-MCP: "Trigger: INT-FMCP-01 fails (the MCP stack cannot share a build graph with the core). Fallback: `frob-mcp` is a separate workspace that talks to `frobd` over a local socket". §32.12.2: "The socket form is expected at M0 and is not a degraded mode". INT-FMCP-01: "builds on its annex toolchain … passes a protocol smoke test against `frobd` over the local socket". | R3, §32.12.2 and INT-FMCP-01. Sibling code confirms the premise: `fastmcp_rust/Cargo.toml` has `rust-version = "1.100"` and `asupersync = "=0.6.2"`, its toolchain file pins `nightly-2026-08-25`, and `frankensim/rust-toolchain.toml` pins `nightly-2026-07-06` (rustc 1.99.0-nightly). The kill gate's fallback is the default design, and its trigger is a condition that already holds. | Section 2, F11. |
| F12 | S2 | 1 | P00:916 (BET-15) against P06:1218–1219, 1483, 1492 | BET-15: "pretraining … is not repaid within 200 decisions (illustrative: 48 GPU-hours repay after 192 decisions at 900 GPU-seconds saved each)". §23.11: "about 590 quadruped designs (s = 410 s) and about 70 to 130 humanoid designs" at 60 GPU-hours and f = 0.1. No gate measures payback. | §23.11. A saving of 900 GPU-seconds per decision exceeds the 410 s the same document gives as the cost of a quadruped training run, so the illustration contradicts §1.1. As written the bet is falsified for quadrupeds by the plan's own arithmetic. | Section 2, F12. |
| F13 | S2 | 1 | P00:928 (BET-27), P00:305 (H-01) against P06:788, 804 | BET-27: "more than 1 GPU-minute for a quadruped or 5 for a humanoid"; H-01: "1–5 per humanoid episode". GATE-REFLEX-06: "at most 1 GPU-minute per 20 s quadruped episode and 6 per humanoid episode". | 5. The verdict register binds (G13) and gives 1 to 5 as the surviving range. | Section 2, F13 (gate row and pilot row in P06). |
| F14 | S2 | 1 | P00:930 (BET-29), P01:1555 (C-35) against P10:1157, 1168 | BET-29 and C-35: falsified by "less than 7% lighter" and "a mass saving below 7%". GATE-FOUNDRY-05 and KILL-TOPOPT: "saving at least 5% (kill threshold)", "saving under 5%". | 7%. Two parts agree, and 7% is the bottom of the published range the plan cites (H-12: 7 to 30%). | Section 2, F14. |
| F15 | S2 | 1, 4 | P00:935 (BET-34) against P03:512, 618, 622; brief M2 | BET-34: "At M2 exit the referee completes fewer than 90% of standard-battery scenarios … or sampled coverage is below 0.1%". §16.11 and GATE-FID-03: an adjudicator-naming rule at "fewer than 80%", and the gate passes on the report alone. GATE-FID-07 (the sampled referee) is at M3; the brief lands "the sampled referee" at M2. | BET-34's 90% and M2: the bet needs a gate that can fail, and the brief fixes the milestone. | Section 2, F15. |
| F16 | S2 | 1 | P00:936 (BET-35), P05:647 (KILL-FLEET) against P05:799 | BET-35: "less than 1.3× faster"; KILL-FLEET: "still below 1.3". GATE-SPARK-13: "at least 1.5". A result of 1.4 fails the gate and fires no kill. | 1.3, stated twice. | Section 2, F16. |
| F17 | S2 | 1 | P06:1155, 1518 against P05:1314, 1425–1433, GATE-SPARK-20 | GATE-CORTEX-08: "scaling of at least 1.6 times at constant global workload". §36.3 row 5: strong scaling tolerance "At least 1.25", ambition "At least 1.6 (the funding gate of §36.6)"; FACT in §36.6: the incumbent's own two-card gain is 1.26. | 1.25. §36.3 is the binding tolerance, GATE-SPARK-20 repeats it, and 1.6 is the ambition that KILL-SHARD uses to decide further funding. | Section 2, F17. |
| F18 | S2 | 1 | P00:927 (BET-26) against P06:789, 810 | BET-26: "The median orbit solve costs more than 25% of a training run for its class". GATE-GAIT-03: "median solve at most 10 minutes of CPU". Ten minutes is about 1.5 quadruped training runs at 410 s. | The gate, which owns the threshold (§0.3). The bet's 25% is in other units (GPU training time) and would be falsified by a result the gate passes. | Section 2, F18. |
| F19 | S2 | 1 | P00:932 (BET-31) against P03:1193 | BET-31: "less than 2× of campaign work is removed". KILL-QUOTIENT: "digest classes save less than 5% of dispatches". The bet's other clauses (planted winner at rank 1, six of the top eight) have no gate. | The kill gate's 5%: §2.4 of P00 itself gives 2× only for mirror symmetry. | Section 2, F19. The register needs a gate row for the planted-winner and six-of-eight clauses. |
| F20 | S2 | 1 | P00:637 against P05:1487–1488, P05:653–655, P03:908 | §1.8: "an incumbent step trace with 53% of its time in the solver". §36.9: "of 262 ns per world-step, the solve is 47%"; §21.18: "the solve is 124 ns". 53 is the nanosecond count of the recursion stages ("53 of 262 ns, 20% of the step"). | 47% (124 of 262). | Section 2, F20. |
| F21 | S2 | 1, 6 | P03:491–499 against P05:1493–1499 | §16.11: "arithmetic on the targets of §36 … a humanoid batch at 0.5 M world-steps/s … produces 1,000 … about 0.5 to 2.4% for the humanoid … the CPU could shadow most humanoid worlds". §36.9: "T·h = 1.7 M × 2 ms = 3,400 … 0.14 to 0.7% … 16% of 3,400". | §36.9. 0.5 M is the humanoid target of version 0.2, which the audit withdrew (item 12 of its list); §36.3 row 7 now reads 1.7 M. | Section 2, F21 (four spans in P03). |
| F22 | S2 | 4 | P05:1473, 1680 against brief M0, P00:27, P00:823 | "GATE-SPARK-21 … Parity baseline … M1" and "(GATE-SPARK-21, M1)". Brief M0: "the incumbent phase ledger measured on `trj` (parity baseline)", exit "the parity-baseline gate". | M0, by the brief. | Section 2, F22. |
| F23 | S2 | 4 | P04:1211; P07:2055, 2057; P03:1676; P06:811; P08:756; P10:620, 1153–1155 against brief §3 | Gate milestones against the skeleton: GATE-HAB-11 retrofit M3 (skeleton and BET-37: M4); GATE-AUGUR-09 region certificates M3 (skeleton M4; BET-20 "M4, M5"); GATE-AUGUR-11 probe design M3 (skeleton "experiment suggestion" M4; registry `experiment` M4); GATE-TRAC-09 hard-contact law "M3; GPU at M6" (skeleton M5); GATE-SHIELD-01 energy tier M3 (skeleton, BET-25 and NS-23: M4); GATE-ARENA-04 at M1 asks for "the fidelity envelope on the batch tiers" (batch tiers land at M2); GATE-FOUNDRY-01 to -03 and GATE-GROUND-03 at M4 (skeleton: Foundry and Ground at M5; §30 says nothing of its own is on a path before M5). | The brief's skeleton is binding for §43 and §40. | Section 2, F23: six cells. For the four Foundry and Ground rows, change M4 to M5, or have §43 list "load campaign, beam sizing, identifiability report" at M4. |
| F24 | S2 | 2 | 24 gate rows in P03:1185; P06 (9); P08 (4); P09 (6); P10 (2); P11 (2); P07:2053 against P00:1254–1256, 1331 | Gate tables use the claim "Performance of the method", which is not one of the seven. §8.1: "The 'method' gates of version 0.2 … name this claim [Design comparison]"; §8.3 rule 1: "A gate names exactly one claim." Seven gates name two (GATE-ATEL-12, GATE-CHRON-09, GATE-CORTEX-08, GATE-CORTEX-14, GATE-PHYLUM-06, GATE-TETHER-09; GATE-SPARK-21 says "Parity baseline"). §25 files agent-level gates under "Performance" (GATE-AUGUR-07). The gate tables of §12, §13, §14 and §15 have no claim cell. | P00 §8, which owns the claims. | Replace all "Performance of the method" with "Design comparison" in the six parts; split or re-label the seven two-claim rows; give GATE-AUGUR-07 the same claim as GATE-TETHER-01 and -13; add a claim cell to the tables of §12 to §15. |
| F25 | S2 | 2 | P01:2167; P08:212, 638, 988; P10:964; P11:90; P00:1297, 1194 | The flags object has three keys: `"flags"` (`.frobcert`, §12.4), `"no_authorization"` (§24, §26), `"authorization"` (`.fbuild`, the envelope, the type of §8.2). NS-20 says a `.fbuild` packet "carries the three no-authorization flags"; R7, §31 and §37.9 give build packets a fourth, `fabrication_released`. | `authorization`: it is the field of the type in §8.2 and of the envelope, and two formats already use it. Four flags on build packets, by R7. | Section 2, F25. |
| F26 | S2 | 3 | P07:3–5, 70, 609 against brief R2, P12:1077, P07:2117–2125 | §25 opens "Crates: `frob-mission` …, `frob-synth` … and `frob-cert`", and says "payload that `frob-cert` reads and writes". R2: "`frob-lemma` writes `.frobcert` bundles … and shares no code with the reader". The chapter corrects itself only inside an open-question entry: "Where this chapter says that `frob-cert` 'reads and writes' the family payload, read: `frob-cert` defines and reads it". | R2. A correction placed in §25.28 does not reach a reader of §25.5. `frob-cert` belongs to Lemma (P00 §0.5, P12 crate map), not to Augur. | Section 2, F26; reword the opening sentence of §25 so that `frob-cert` is named as Lemma's crate that Augur's payload rules live in. |
| F27 | S2 | 7 | P00:357, 288–289 against P05:1322, 1334, P05:234 | §1: "Nothing was run on an incumbent simulator". §36.3 row 13: "CPU MuJoCo in `f64` on this machine under load … 24 k world-steps/s on one thread, 550 k on 24, 576 k on 36", "was measured on a loaded machine". §0.8: "exact, CPU-scale pilots were run"; §21.8: "An exact unsigned-integer kernel returned every label correctly on all three GPUs". | §36.3 and §21.8 report what was done; §1 and §0.8 state more than is true. The honesty statement must match. | Section 2, F27. |
| F28 | S2 | 2 | P08:755, 689–702; P09:1180, 1092–1096; `APPENDIX_O_REFUSALS.md` rows 533, 542, 561, 552, 553, 532, 555 | Renumbering broke two gates that cite refusal ranges. GATE-ARENA-03: "every refusal 520 to 528", but the requirement-language codes are now 520, 535, 536, 523, 538, 525, 526, 539, 528, and 521, 522, 524 and 527 belong to §23. GATE-ATEL-06: "FROB-REF-553 to -544". The registry also holds one refusal under several codes: paired statistics under an uncontrolled class (533, 542, 561); mixed study types (552, 553); a fixed-n study with an unreachable verdict (532, 555). | List the codes; a range has no meaning after renumbering. Keep one code per refusal: 561 (§35 owns determinism classes), 552 and 532 (§24 owns study types). | Section 2, F28; retire 533, 542, 553 and 555 in Appendix O and point their rows at the kept codes. |
| F29 | S2 | 7 | P00:321, 940 against P01 C-34 and D-6, P07:76–77, 2060 (GATE-AUGUR-22 at M8), charter G9 and §10, P12:905 | H-17: "continuation is [F] on the CPU"; §4 closing list: "Moonshot items (kernel reflection, continuation-based synthesis, …)". §10, §25 and the charter tier continuation [M]. §12 and the crate map tier kernel reflection [F] (G9: "kernel reflection as a Frontier route"), with its gate at M5. | Continuation is [M]; kernel reflection is [F]. P00 has each one wrong once. | Section 2, F29. |
| F30 | S2 | 3 | P04:3, 918–936; P12:874, 965–978 (L-1, L-11) | §19 gives `frob-scene` (layer 2) the retrofit catalog and the verb `scene.retrofit`, whose step 3 reruns "the same procedure that produced the failing verdict" with the policy frozen and whose step 4 requests a redesign "from the design search (§28, §25)". Those procedures live in layers 3 to 5 (`frob-dyn`, `frob-arena`, `frob-opt`, `frob-mission`), and L-11 puts decisions about what to run at layer 4 and above. | P12's rules. The catalog, the typed scene edits and kit enumeration belong in `frob-scene`; the search cannot. | Add to §19.12: "`frob-scene` owns the catalog, the typed scene edits and the enumeration of kits. Steps 3 and 4 are executed by the verb implementation in `frob-verbs` through the owner of the failing verdict; `frob-scene` links no crate above layer 2." |
| F31 | S2 | 1 | P00:827, P05:1630–1631 against P01:1093, 1196; P03:402, 619; P06:1227; P09:472, 1181 | "About thirty per-design trainings, 5 to 7.5 GPU-hours" against a calibration set of 40 designs that "costs about 45 GPU-hours, or about 5 if the specialists … are reused". | 40 designs: four parts and three gates fix it. The cost needs both figures, since the owner is asked to approve the envelope. | Section 2, F31. |
| F32 | S2 | 1, 2 | P04:1183, 1206 against P03:1672; P07:2058 against P09:1185; P03:620, P06:1525, P09:1184; P08 GATE-AUTOPSY-01, -02, -07 against P09 GATE-CHRON-05, -06; P03 GATE-FID-06, -08 against P09 GATE-CHRON-08, -09 | Mechanisms with a gate in two or three chapters, which R1 resolves only for kill gates. Two pairs differ in threshold: film law "three squeeze times within 1%" (GATE-HAB-06) against "Stefan times within 2%" (GATE-TRAC-05); revalidation on the same 30-amendment campaign, "unnecessary invalidations at most 20%" (GATE-AUGUR-12) against "counted and reported" (GATE-ATEL-11), while BET-33 asks for a "declared share". The model-form check has three gates (GATE-FID-05, GATE-CORTEX-15, GATE-ATEL-10) and two spellings of its result (`ModelConditional`, `model-conditional`). | Apply R1's rule to convergence gates: the owner chapter's row governs and the others cite it. Film law: §18 (2%). Revalidation: §28 (R8, one engine), with the 20% share written into it. Model-form check: §16.9. Forks, divergence, the energy audit and tier correction: §29. | Section 2, F32 (two substitutions); the rest is a rule for the gate register. |
| F33 | S3 | 2 | P00:1129 against P03:211, P05:123, 786, P12:304 | NS-11: "`frob prove kernel K-se3-cay-step`"; the kernel is `K-cay-step` everywhere else. | `K-cay-step`. | Section 2, F33. |
| F34 | S3 | 2 | P00:1119, 1134; P08:211, 987 | Labels `ASSUMED` and `ESTIMATED` in NS-07 and NS-12, against `assumed` and `Estimated` in every chapter; `"class": "tested"` in two JSON examples against `"class": "Certified"` in §12.4. | Lower-case provenance labels; capitalised class names. | Section 2, F34. |
| F35 | S3 | 1 | P00:633 against P05:1518 | "about 1 to 1.5% of a 24 GB card" against "0.28 GB and … 0.79 GB: 1 to 3% of a 24 GB card". | §36.10, which shows the arithmetic. | Section 2, F35. |
| F36 | S3 | 1 | P04:1046–1050 against P05:1537–1541 | Band storage at 2 cm for the same three scenes: 45–84 MB, 1.6–3.9 GB, 2.9–7.3 GB (§19.15, "band bricks with labels") against 40 to 75 MB, 1.4 to 3.5 GB, 2.6 to 6.5 GB (§36.10). | Both may be right if the difference is the label bytes, but neither says so. | State in §36.10 that its band column excludes labels, or copy §19.15's figures. |
| F37 | S3 | 7 | P11:284, 950, 1014, 1025 against P00:1364 | "verified payload hashes", "verified immutable inputs", "kernel caches verified by content hash" (twice), in a plan whose rule 14 reads "The word 'verified' is not used." These are hash checks, not results. | Reword; the rule is stated without exception. | Section 2, F37. |
| F38 | S3 | 7 | P07:1850–1862 | §25.22 cites inner paths of the planning archive (`mor/repro/P01-*`, `followups/docking_support.*`). The chapter itself says the bundle's name is excluded by the style rules. | Process provenance belongs in Appendix H and K only (charter rule 1). | Replace the path column with source-register keys. |
| F39 | S3 | 2 | P03:540 against P05:895–898 | `sim.run --profile` takes a step profile (`lockstep`, `strict`, `referee`) in §16; §35.2 names determinism profiles (`reference-replay`, `device-replay`, `cross-backend`, `fast-training`) and §35 adds precision profiles; P05's ledger assigns them to the same flag. | Step profiles keep `--profile` (R8). | Appendix J: `--determinism` and `--precision` for the other two. |
| F40 | S3 | 1 | P00:908 (BET-07) against P06 GATE-CORTEX-03 | "across five seeds" against "ten paired seeds with intervals". | The gate. | Section 2, F40. |
| F41 | S3 | 2 | P02:903 against P07:70, P12:900 | "Parallel delta arm; parallel ankles … bounded exact synthesis lane … §28"; the lane and its kill gate (KILL-SYNTH-LINKAGE) are in §25. | §25. | Section 2, F41. |
| F42 | S3 | 1 | P12:923–925, 904 | "Eight crates hold only Frontier or Moonshot content … their caps sum to 74. The remaining 487 …"; `derive/frob-derive` is also tiered F alone (cap 14). All other sums in the table are correct (re-added: 561). | Nine crates, 88 and 473; or say "eight core crates". | Section 2, F42. |
| F43 | S3 | 2 | P11:156–157 against P03:1755–1758 and the chapters | §32.6 reserves the cluster `family` for §17, whose verbs are `phylum.*`. Clusters `sense`, `fidelity`, `contact`, `phylum`, `actuator`, `module`, `algebra`, `task`, `fleet`, `policy` are used by chapters and absent from the table, which says a chapter's verb "joins the cluster named here". | The chapters. | Replace `family` (§17) by `phylum` (§17) in the reserved list and add the ten clusters. |
| F44 | S3 | 1 | P05:1795–1796 (LEDGER), `LEDGERS.md`:702–703; P09:1881–1882, P07:2152, P10:1204–1205 (LEDGER) | P05's ledger still asks for "tolerance 1.5 times the incumbent …, planning value 615 s"; the reconciled body and R5 say 1.25 and 515 s. Several ledgers carry refusal ranges garbled by renumbering ("FROB-REF-868 to FROB-REF-859"). | The bodies. The ledgers are stripped, but `LEDGERS.md` is an input to Phase 2. | Regenerate `LEDGERS.md` after the fixes, or mark these lines superseded. |

## 2. Substitutions

Each ORIGINAL is verbatim and unique in the named file unless marked "replace all".

### F01 (P09.md)

```
ORIGINAL:    | `selective` | τ on the top band ≥ 0.5;
REPLACEMENT: | `selective` | τ on the top band ≥ 0.7;

ORIGINAL:    | `discard-only` | 0.2 ≤ τ < 0.5; or the stratum is unmeasured; or a fallback trigger of DC-02 or DC-03 fired |
REPLACEMENT: | `discard-only` | 0.5 ≤ τ < 0.7; or a fallback trigger of DC-02 or DC-03 fired |

ORIGINAL:    | `uninformative` | τ < 0.2, or the planted winner was lost in this stratum |
REPLACEMENT: | `uninformative` | τ < 0.5; or the stratum is unmeasured; or the planted winner was lost in this stratum |

ORIGINAL:    0.2 is a DESIGN default that the experiment resets (OPEN-ATEL-2)
REPLACEMENT: they are the thresholds of §16.9, §22.18 and §23.11, fixed before data

ORIGINAL:    An unmeasured stratum is `discard-only`; the default fails closed, and the default is the fixed table.
REPLACEMENT: An unmeasured stratum is `uninformative`; the default fails closed, and a rung without a rank-agreement record is refused (`FROB-REF-537`).

ORIGINAL:    default is that R1 discards.
REPLACEMENT: default, until a stratum is measured, is that R1 neither selects nor discards.

ORIGINAL:    Kendall τ on the top band at least 0.5 and ρ_ep at least 0.7 at the lower bootstrap bound, on 40 designs. A failing class stays `discard-only`.
REPLACEMENT: Kendall τ on the top band at least 0.7 for `selective` and at least 0.5 for `discard-only`, and ρ_ep at least 0.7, all at the lower bootstrap bound, on 40 designs. A class below 0.5 is `uninformative`.

ORIGINAL:    R1 is `discard-only` everywhere; the ladder's claim shrinks to stopping and elimination
REPLACEMENT: R1 is `uninformative` everywhere and every candidate that survives static analysis is scored on the higher rung; the ladder's claim shrinks to stopping and elimination among trained candidates

ORIGINAL:    KILL-RANK: `discard-only` everywhere
REPLACEMENT: KILL-RANK: `uninformative` everywhere
```

### F02 (P01.md, P07.md)

```
P01  ORIGINAL:    | `farkas-dual`, `conic-dual-strict` |
     REPLACEMENT: | `lp-farkas`, `conic-farkas` |

P01  ORIGINAL:    | `interval-sign`, `interval-root` |
     REPLACEMENT: | `interval-box`, `interval-root` |

P01  ORIGINAL:    | `witness-replay` |
     REPLACEMENT: | `lp-primal` (static witnesses), `finite-game` (policy tables), `trace-witness` (a replayed execution; class at most `Tested`) |

P01  ORIGINAL:    | `partition-cover` |
     REPLACEMENT: | `partition` |

P01  ORIGINAL:    | `alias-witness` |
     REPLACEMENT: | `alias-pair` |

P01  ORIGINAL:    | `product-unreachability` |
     REPLACEMENT: | `product-unreachable` |

P01  ORIGINAL:    | `statistical-receipt` |
     REPLACEMENT: | `sample-coverage` |

P01  ORIGINAL:    "proof": { "kind": "witness-replay",
     REPLACEMENT: "proof": { "kind": "lp-primal",

P07  ORIGINAL:    #### 25.5.2 Proof kinds (registry `frob.cert.kinds/1`, closed)
     REPLACEMENT: #### 25.5.2 Proof kinds used by mission answers (rows of the one closed registry `frob.cert.kinds/1`, §12.4)
```

Also name the table of §12.4 `frob.cert.kinds/1` and add the rows `taylor-box`, `sos-gram`, `symmetry-transport`
and `composition` from §25.5.2.

### F03 (P11.md, P01.md, P06.md, P09.md)

```
P11  ORIGINAL:    `interventions` (R: what may be held or changed, and from which instant), `run` (U: interventional attribution with a placebo control and a stability report), `show` (R), `diff` (R: first divergent event between a run and its fork)
     REPLACEMENT: `intervene` (U: interventions, placebo forks, replicates and coalitions from a fork made by `run.fork`), `attribute` (R: contrasts, flip rates, pairs, labels), `repair` (U), `rerun` (U: paired re-run from the episode start), `show` (R)

P11  ORIGINAL:    `separate` (U: a world pair on which a design ordering reverses), `rare` (U: bounded-weight failure probability), `show` (R), `adopt` (M: add a counterexample to a regression battery)
     REPLACEMENT: `oracle` (U: classify one world for one candidate), `bound` (U: `certify`, `refute` or `estimate` a failure probability), `audit` (U), `cover` (R), `hardset` (R, U: show, add, retire and run the hard-world set)

P11  ORIGINAL:    | §26 | M3; `separate` M4 |
     REPLACEMENT: | §26 | M3; `audit` M4 |

P11  ORIGINAL:    `distinguish` (U: the smallest admissible intervention or instrument that separates alternatives), `show` (R)
     REPLACEMENT: `distinguish` (U: the smallest admissible intervention or instrument that separates alternatives), `separate` (U: an admissible world pair on which a design ordering or a diagnosis reverses), `transport` (U) [M], `revalidate` (U: the same engine as `result.revalidate`), `show` (R)

P11  ORIGINAL:    | `cert` | `bundle` (U: assemble a `.frobcert`), `check` (R: in-tree check; the independent `frob-verify` binary repeats it offline), `show` (R), `list` (R) | §12 | M1 for the mission slice; full M5 |
     REPLACEMENT: (delete the row)

P11  ORIGINAL:    | `prove` | `kernel` (R), `verify` (R), `list` (R) | §12 | M0, M5 |
     REPLACEMENT: | `prove` | `kernel` (R), `verify` (R: in-tree check of a bundle; the independent `frob-verify` binary repeats it offline), `list` (R), `bundle` (U: assemble a `.frobcert`), `reflect` (U) [F] | §12 | M0; `bundle` M1 for the mission slice; full M5 |

P11  ORIGINAL:    `import` (M: logs and measurements, as data), `calibrate` (U), `band` (R: the reality band for a predicted metric, or `no-claim`), `compare` (R: residuals and the exchangeability alarm), `show` (R)
     REPLACEMENT: `ingest` (M: logs and measurements, as data), `excite` (R), `identify` (U), `predict` (M), `score` (U), `band` (R: the reality band for a predicted metric, or `no-claim`), `kit` (U)

P11  ORIGINAL:    `structure` (U), `fatigue` (U), `tolerance` (U), `packet` (U: a `.fbuild` build packet), `show` (R); `firmware` (U) [M]
     REPLACEMENT: `loads` (U), `size` (R), `optimize` (U), `check` (U), `tolerance` (R), `export` (U: geometry and the `.fbuild` build packet; `--with-firmware` [M]), `lifecycle` (U) [M]

P11  ORIGINAL:    $ frob autopsy run r:5c11…
     REPLACEMENT: $ frob autopsy intervene r:5c11…

P01  ORIGINAL:    and `world.separate` returns the admitted world in S
     REPLACEMENT: and `mission.separate` returns the admitted world in S

P06  ORIGINAL:    `eval.rankcheck` (rank robustness of a finalist set)
     REPLACEMENT: `fidelity.modelform` (rank robustness of a finalist set; defined in §16.9)

P09  ORIGINAL (replace all, lines 35 and 230):  `eval.compare --plan`
     REPLACEMENT:                               `eval.preanalyze`
```

### F04 (P03.md, P05.md)

```
P03  ORIGINAL:    | Quadruped, 12 joints, 16 foot samples and 8 body samples | 18 | 13 | 24 | 24 | 128 | 27 kB |
     REPLACEMENT: | Quadruped, 12 joints, 16 foot samples and 8 body samples | 18 | 13 | 24 | 32 | 160 | 35 kB |

P03  ORIGINAL:    | Humanoid, 29 joints, 8 samples per sole, hands and body | 35 | 30 | 40 | 40 | 224 | 72 kB |
     REPLACEMENT: | Humanoid, 29 joints, 8 samples per sole, hands and body | 35 | 30 | 40 | 64 | 320 | 97 kB |

P05  ORIGINAL:    world-step in 10⁵ is marked `Degraded` for capacity on the standard battery; otherwise the capacity is raised
     REPLACEMENT: world-step in 10⁴ is marked `Degraded` for capacity on the standard battery (GATE-KIN-07); otherwise the capacity is raised
```

### F05 (P06.md, P02.md)

```
P06  ORIGINAL:    | GATE-REFLEX-01 | Known answer | Every template stands for 10 s and traverses 5 m of flat ground under its baseline controller on `referee` and `batch-cpu`. | M1 |
     REPLACEMENT: | GATE-REFLEX-01 | Known answer | Each archetype stands for 10 s and traverses 5 m of flat ground under its baseline controller: at M1 the first archetypes, on `referee` only, restricted to those whose contact law exists at M1 and whose contact count fits the referee's 21-contact cap (§16.6); at M2 the same set on `batch-cpu`; at M3 all eighteen on `batch-cpu`, except tracked and hybrid archetypes, which follow at M4 with their contact laws. | M1, M2, M3, M4 |

P02  ORIGINAL:    and stands or rolls under its baseline controller (with GATE-REFLEX-01). |
     REPLACEMENT: and stands or rolls under its baseline controller at the milestone GATE-REFLEX-01 gives for it. |
```

### F06 (P04.md, P10.md, P07.md, P12.md)

```
P04  ORIGINAL (replace all):  frob.fscene/1      REPLACEMENT: frob.scene/1
P04  ORIGINAL (replace all):  frob.fclear/1      REPLACEMENT: frob.clearance/1
P10  ORIGINAL (replace all):  frob.fbuild/1      REPLACEMENT: frob.build/1

P07  ORIGINAL:    | `frob-mission` | 5 | The `.fmission` schema and admission;
     REPLACEMENT: | `frob-mission` | 5 | Admission of `.fmission` queries (the schema, the parser and the canonical form are owned by `frob-spec`, §34.9);

P12  ORIGINAL:    Canonical. Three kinds: actuator, interface, material
     REPLACEMENT: Canonical. Kinds, a closed list in `registries/formats.toml`: actuator, transmission, battery, interface, material, soil, optical, sensor, payload
```

### F07 (P00.md, P03.md)

```
P00  ORIGINAL:    or mixed throughput per degree of freedom is below 0.33 of the single-design rate; or 1,024 variants in one batch fail to reproduce the digests of 1,024 single-design runs, or cost more than 1.3× the homogeneous step.
     REPLACEMENT: or the mixed batch runs below 0.5 of the harmonic mean of its homogeneous rates (the tolerance is 0.70, §36.3 row 10); or any of the 512 designs of a mixed cohort fails to reproduce the digests of its single-design run.

P03  ORIGINAL:    For the cohort to remain the engine: at least 0.6 of homogeneous world-steps/s
     REPLACEMENT: For the cohort to remain the engine: at least 0.70 of the harmonic mean of the homogeneous rates (§36.3 row 10)
```

### F08 (P03.md)

```
ORIGINAL:    the momentum that the right-trivialized derivative of τ at −h·ξₖ
REPLACEMENT: the momentum that the inverse transpose of the left-trivialized differential of τ at −h·ξₖ (the matrix (Dcay)⁻ᵀ of C-36)
```

### F09 (P11.md)

```
ORIGINAL:    which reruns are needed and their cost) | §28, §12 | M4 |
REPLACEMENT: which reruns are needed and their cost) | §28, §12 | M1 for the mission slice (dependency grades 1 and 2 on mission cells); full M4 |
```

### F10 (P04.md, P00.md)

```
P04  ORIGINAL:    ✖ **KILL-FILM.** Trigger: the advected film fails its mass gate
     REPLACEMENT: ✖ **KILL-ADVECT.** Trigger: the advected film fails its mass gate

P04  ORIGINAL:    | ✖ KILL-FILM | §19.10.7 | Static film-friction patches |
     REPLACEMENT: | ✖ KILL-ADVECT | §19.10.7 | Static film-friction patches |

P00  ORIGINAL:    The film law misses its known answers; or advected films change fewer than 20% of kitchen rankings; or they add more than 25% to training time.
     REPLACEMENT: The film law misses its known answers, changes fewer than 10% of episode outcomes on the wet-floor battery or adds more than 5% to step time (KILL-FILM, §18.8); or advected films leave design rankings on the spill battery at a rank correlation of 0.95 or above with static patches, or cost more than 15% of the step (KILL-ADVECT, §19.10.7).
```

### F11 (P11.md)

```
ORIGINAL:    | KILL-MCP | — | Trigger: INT-FMCP-01 fails (the MCP stack cannot share a build graph with the core). Fallback: `frob-mcp` is a separate workspace that talks to `frobd` over a local socket in newline-delimited JSON; if that also fails, agents use the CLI, which is complete on its own. |
REPLACEMENT: | KILL-MCP | — | Trigger: INT-FMCP-01 fails (`frob-mcp` cannot be built on its annex toolchain with a single asupersync package, or fails the protocol smoke test against `frobd` over the local socket). Fallback: agents use the CLI, which is complete on its own. Linking handlers in process is an upstream-dependent improvement (§33.13), not a fallback. |
```

### F12 (P00.md)

```
ORIGINAL:    is not repaid within 200 decisions (illustrative: 48 GPU-hours repay after 192 decisions at 900 GPU-seconds saved each)
REPLACEMENT: is not repaid within the break-even count N = P/(s·(1 − f)) of §23.11 at the measured P, s and f (illustrative: 60 GPU-hours at f = 0.1 repay after about 590 quadruped designs at s = 410 s, or 70 to 130 humanoid designs)
```

### F13 (P06.md)

```
ORIGINAL:    and 6 per humanoid episode
REPLACEMENT: and 5 per humanoid episode

ORIGINAL:    or 6 (humanoid) per episode
REPLACEMENT: or 5 (humanoid) per episode
```

### F14 (P10.md)

```
ORIGINAL:    saving at least 5% (kill threshold) with 15% the target
REPLACEMENT: saving at least 7% (kill threshold, as in BET-29 and C-35) with 15% the target

ORIGINAL:    saving under 5%, no convergence, or over budget
REPLACEMENT: saving under 7%, no convergence, or over budget
```

### F15 (P03.md)

```
ORIGINAL:    where the referee completes fewer than 80% of battery scenarios (TARGET)
REPLACEMENT: where the referee completes fewer than 90% of battery scenarios (TARGET; the threshold of BET-34)

ORIGINAL:    the adjudicator named wherever coverage is below 80%.
REPLACEMENT: the adjudicator named wherever coverage is below 90%; coverage below 90% in a reference class falsifies BET-34 for that class, and its fallback applies.

ORIGINAL:    detected within twice the time §16.11 predicts. | M3 |
REPLACEMENT: detected within twice the time §16.11 predicts. | M2 |
```

### F16 (P05.md)

```
ORIGINAL:    over the better single machine is at least 1.5;
REPLACEMENT: over the better single machine is at least 1.3;
```

### F17 (P06.md)

```
ORIGINAL:    scaling of at least 1.6 times at constant global workload and equal task quality.
REPLACEMENT: strong scaling of at least 1.25 times at constant global workload and equal task quality (§36.3 row 5; 1.6 is the ambition and the funding threshold of KILL-SHARD).

ORIGINAL:    Scaling TARGET: at least 1.6 times from one RTX 4090 to two at
REPLACEMENT: Scaling TARGET: at least 1.25 times (tolerance; the ambition is 1.6, §36.3 row 5) from one RTX 4090 to two at
```

### F18, F19 (P00.md)

```
ORIGINAL:    The median orbit solve costs more than 25% of a training run for its class (published solves span 2% to over 100%)
REPLACEMENT: The median orbit solve takes more than 10 minutes of CPU (published solves span 4 to 260 s, which is 2% to over 100% of a training run)

ORIGINAL:    or less than 2× of campaign work is removed on the reference campaign
REPLACEMENT: or digest classes save less than 5% of dispatches on the campaign corpus
```

### F20 (P00.md)

```
ORIGINAL:    an incumbent step trace with 53% of its time in the solver
REPLACEMENT: an incumbent step trace with 47% of its time in the solver
```

### F21 (P03.md)

```
ORIGINAL:    so a humanoid batch at 0.5 M world-steps/s and h = 2 ms produces 1,000
REPLACEMENT: so a humanoid batch at 1.7 M world-steps/s (the tolerance of §36.3 row 7) and h = 2 ms produces 3,400

ORIGINAL:    The share is therefore about 0.5
REPLACEMENT: The share is therefore about 0.14

ORIGINAL:    to 2.4% for the humanoid and 0.1 to 0.6% for the quadruped
REPLACEMENT: to 0.7% for the humanoid and 0.12 to 0.6% for the quadruped

ORIGINAL:    produce 0.4 to 1.7 times the humanoid GPU target, so under the `lockstep` profile the CPU could shadow most
REPLACEMENT: produce 0.1 to 0.65 times the humanoid GPU tolerance, so under the `lockstep` profile the CPU could shadow a tenth or more of
```

### F22 (P05.md)

```
ORIGINAL:    is reproduced within 20% or the difference is explained in writing | M1 |
REPLACEMENT: is reproduced within 20% or the difference is explained in writing | M0 |

ORIGINAL:    (GATE-SPARK-21, M1)
REPLACEMENT: (GATE-SPARK-21, M0)
```

### F23 (P04.md, P07.md, P03.md, P06.md, P08.md)

```
P04  ORIGINAL:    the wheel-and-ramp closed form is reproduced by both arms. | M3 |
     REPLACEMENT: the wheel-and-ramp closed form is reproduced by both arms. | M4 |

P07  ORIGINAL:    GATE-AUGUR-09 | Known answer | M3 |
     REPLACEMENT: GATE-AUGUR-09 | Known answer | M4 |

P07  ORIGINAL:    GATE-AUGUR-11 | Design comparison | M3 |
     REPLACEMENT: GATE-AUGUR-11 | Design comparison | M4 |

P03  ORIGINAL:    agreement with the referee on the Solid rows of §18.16. | M3; GPU at M6 |
     REPLACEMENT: agreement with the referee on the Solid rows of §18.16. | M5; GPU at M6 |

P06  ORIGINAL:    a payload-only budget shows an exceedance. | M3 |
     REPLACEMENT: a payload-only budget shows an exceedance. | M4 |

P08  ORIGINAL:    both negative controls are caught | M1 |
     REPLACEMENT: both negative controls are caught | M1 on the referee; M2 on the batch tiers |
```

### F25 (P01.md, P08.md, P00.md)

```
P01  ORIGINAL:    "flags": { "execution_authorized": false,
     REPLACEMENT: "authorization": { "execution_authorized": false,

P08  ORIGINAL (replace all, lines 212, 638, 988):  "no_authorization"
     REPLACEMENT:                                  "authorization"

P00  ORIGINAL:    carries the three no-authorization flags of §8, all false
     REPLACEMENT: carries the three no-authorization flags of §8 and the packet flag `fabrication_released`, all false
```

### F26 (P07.md)

```
ORIGINAL:    payload that `frob-cert` reads and writes.
REPLACEMENT: payload that `frob-cert` defines and reads and that `frob-lemma` writes (§34.9).
```

### F27 (P00.md)

```
ORIGINAL:    Nothing was run on an incumbent simulator:
REPLACEMENT: No incumbent GPU simulator was run (the one executed incumbent measurement is CPU MuJoCo on this Mac, §36.3 row 13):

ORIGINAL:    exact, CPU-scale pilots were run.
REPLACEMENT: exact, small pilots were run, on CPUs and, for one boundary-classifier kernel, on the three GPUs (§21.8).
```

### F28 (P08.md, P09.md)

```
P08  ORIGINAL:    every refusal 520 to 528 is triggered by its planted task file
     REPLACEMENT: every requirement-language refusal of §24.14 (FROB-REF-520, -535, -536, -523, -538, -525, -526, -539 and -528) is triggered by its planted task file

P09  ORIGINAL:    FROB-REF-553 to -544 each fire on their planted violation
     REPLACEMENT: FROB-REF-553, -555, -542, -543 and -556 each fire on their planted violation
```

### F29 (P00.md)

```
ORIGINAL:    continuation is [F] on the CPU
REPLACEMENT: continuation is [M], on the CPU

ORIGINAL:    Moonshot items (kernel reflection, continuation-based synthesis,
REPLACEMENT: Moonshot items (continuation-based synthesis,
```

### F31 (P00.md, P05.md)

```
P00  ORIGINAL:    About thirty per-design trainings, 5 to 7.5 GPU-hours, need the
     REPLACEMENT: About forty per-design trainings (about 5 GPU-hours where the specialists of §23 are reused, about 45 otherwise; §28.10) need the

P05  ORIGINAL:    needs about thirty per-design trainings,
     REPLACEMENT: needs about forty per-design trainings: about 45 device-hours, or, where specialists are reused,
```

### F32 (P04.md, P09.md)

```
P04  ORIGINAL:    Film-friction law: the three squeeze times within 1%;
     REPLACEMENT: Film-friction law as scenes consume it (the law's own gate is GATE-TRAC-05): the three squeeze times within 2%;

P09  ORIGINAL:    unnecessary invalidations are counted and reported
     REPLACEMENT: unnecessary invalidations are at most 20% of unaffected verdicts, counted and reported
```

The known-answer row at P04:1183 ("within 1%") changes with the gate.

### F33, F34, F35, F40 (P00.md), F34 (P08.md)

```
P00  ORIGINAL:    `frob prove kernel K-se3-cay-step`
     REPLACEMENT: `frob prove kernel K-cay-step`

P00  ORIGINAL:    filled from a card marked `ASSUMED`.
     REPLACEMENT: filled from a card marked `assumed`.

P00  ORIGINAL:    Scale is labelled `ESTIMATED` unless
     REPLACEMENT: Scale is labelled `Estimated` unless

P00  ORIGINAL:    worlds use about 1 to 1.5% of a 24 GB card
     REPLACEMENT: worlds use about 1 to 3% of a 24 GB card

P00  ORIGINAL:    unconstrained policies to reach the same return, across five seeds;
     REPLACEMENT: unconstrained policies to reach the same return, across ten paired seeds;

P08  ORIGINAL (replace all, lines 211 and 987):  "class": "tested"
     REPLACEMENT:                                "class": "Tested"
```

### F37 (P11.md)

```
ORIGINAL:    verified payload hashes                              REPLACEMENT: checked payload hashes
ORIGINAL:    verified immutable inputs                            REPLACEMENT: hash-checked immutable inputs
ORIGINAL:    kernel caches verified by content hash before load   REPLACEMENT: kernel caches checked by content hash before load
ORIGINAL:    is verified by content hash before load              REPLACEMENT: is checked by content hash before load
```

### F41 (P02.md), F42 (P12.md)

```
P02  ORIGINAL:    | Parallel delta arm; parallel ankles | Closures produced by the bounded exact synthesis lane | [F] | §28 |
     REPLACEMENT: | Parallel delta arm; parallel ankles | Closures produced by the bounded exact synthesis lane | [F] | §25 |

P12  ORIGINAL:    Reading the total. Eight crates hold only Frontier or Moonshot content
     REPLACEMENT: Reading the total. Eight core crates hold only Frontier or Moonshot content
```

## 3. Checked and found consistent

- Incumbent figures: 410 s on one RTX 4090 and about 325 s on two; 2.55 M and 1.68 M humanoid world-steps/s at
  8,192 worlds on an RTX 6000 Ada; 4.8 M physics steps/s with 71% overhead; 0.86 and 86 k samples/s for the mixed
  batch. No part states a different value.
- The qualification budget α_i = α/[i·(i + 1)] and the 500-retry gate (P00, P01, P08, P09, P10, P11).
- Step h = 2 ms with decimation 10; the 400 Hz clock example is a clock fixture, not a workload.
- The sampled-referee range of 0.1 to 2% and the 0.1% floor; the Metal limits of about 222 GiB and 167 GiB.
- Family-policy and inheritance figures (70 to 85%, 10 to 40%, 1 to 20%, 33 to 100%); rare-event savings (10 to
  100 times at 10⁻⁴ to 10⁻⁵); conformal coverage (about six points at n = 20; 19/21).
- The Amdahl ceilings (7.8, 6.7, 1.9, 1.4, 3.0 to 3.5, 15, 10.8, 13.5), recomputed.
- Crate scope: 561, with every layer subtotal re-added; crate names against charter §6 (no stray name);
  chapter headings against charter §5.
- Counts: eighteen robot archetypes, seven interior archetypes, A-01 to A-26, C-01 to C-40, thirteen formats.
- Step profile names, determinism classes D0 to D4, the four evidence classes, anchored positions (R4), exit
  code 8, the stance and docking known answers (3/14 m, 1/64, 1/65 m, 344, 217, 9 of 561).
- Sibling facts, read in the code on 2026-11-21: `MAX_CONTACTS = 21` in
  `frankensim/crates/fs-contact/src/sustained/solve.rs` and a default budget of 21 in
  `frictional_multicontact.rs`; `MAX_ARGUMENTS = 31` in `frankentorch/crates/ft-kernel-metal/src/compute.rs`;
  FrankenSymPy's caps of 8 variables, degree 256 and 4,096 bits in `fsym-solvers/src/real_rur.rs`;
  `fln-anvil` at 1,423 lines; every `fs-*` crate a chapter cites exists. No two chapters disagree on a sibling
  fact; the only foundation defect is F11.
- Withdrawn claims: none of the thirteen has crept back. Determinism, mixed topologies, actuator physics, Metal
  and the agent interface are each worded as parity where they appear, and no sentence gives per-step throughput
  as the source of the ten-fold.
- Style: no plan name, model name, duel or score; no LaTeX; no exclamation mark (two factorials); no Moonshot on
  a critical path once F29 is fixed.

## 4. Reader test

FrankenRobotics is a planned Rust system, with no code yet, that simulates legged, wheeled, tracked and hybrid
robots in terrain and interiors on FrankenSim's physics, in thousands of GPU worlds on CUDA and Metal, and gives
AI agents a command line and an MCP server for designing a robot, testing it, reading why it failed and
improving it. It is six products on one model and one evidence ledger: a proving ground, a design studio, a
mission-query engine over families of designs, an adversary with a counterfactual debugger, a path to
calibration and build packets, and a Lie-algebraic core from which kernels and certificates are derived. Its
measure of success is the ratio of restricted mean times to an independently accepted answer against the
strongest composed rival on a sealed suite, with a partial gate at M3 and a tournament at M7. It bets that
ten-fold comes from no single mechanism and not from a faster step, but from a conjunction: deciding many
designs per agent turn, training few of them because cheap rungs rank designs as trained policies do, spending
evaluation where it matters, finding failures by search, answering over families, never recomputing or
re-deciding, explaining by intervention, and short setup. The three bets it turns on are the ten-fold on at
least one stratum (BET-13), that a short program over an incumbent engine cannot tie it (BET-14), and rank
agreement between cheap and trained rungs (BET-17), and it says in advance what it ships if each is lost.

## 5. What an implementing agent still cannot tell

1. **Which threshold governs a bet.** §0.3 makes the gate register the only authority, §0.4 sends the reader to
   §40 first, and §40 is not in Phase 1. Until it exists, BET-15, -18, -26, -27, -29, -31, -34, -35 and -36 and
   their gates give different numbers (F07, F12 to F19, F10).
2. **Which archetypes are "the first archetypes" of M1.** The brief uses the phrase; no chapter lists them, and
   the only M1 gate that could (GATE-REFLEX-01) says "every template" (F05).
3. **Whether the `strict` profile is an M1 deliverable.** GATE-KIN-04, GATE-TRAC-01 and GATE-TRAC-06 require it
   at M1. It is our own dynamics in `f64` on the `batch-cpu` tier, and the brief lists only the transactional
   referee step at M1 and the batched step at M2.
4. **What the M3 partial gate contains.** P00 and the brief say legged and wheeled briefs. KILL-AUGUR-TIE and
   KILL-FAMILYCERT are triggered "at the partial ten-fold gate" on mission-query strata.
5. **What a cheap rung may do before the rank-agreement experiment has run.** The experiment exceeds the default
   compute envelope and waits for the owner. §28 lets an unmeasured rung discard; §23 refuses it (F01). This
   decides the behaviour of every campaign until M3.
6. **The verb list.** Appendix J does not exist, and §32.6 and the owner chapters disagree on six clusters (F03,
   F43).
7. **The proof-kind names and the JSON key of the authorization object** (F02, F25): both are wire formats that
   `frob-verify` rejects on mismatch.
8. **The reference capacities, and who owns them** (F04; OPEN-KIN-3 and OPEN-SPARK-3 both claim the numbers).
9. **Who owns `frob-kernels`.** P00 §0.5 and §16 give it to Kinesis; §21 and the crate map give it to Spark. The
   contract, the kernel manifests and the tests have one owner.
10. **Where three orchestrations live:** the retrofit search (F30), `experiment.suggest` (§25 or §28, with two
    kill gates), and analysis A-19, which reads runs but is listed with the static analyses of `frob-analyze`.
11. **Who the neutral adjudicator and the independent evaluator are.** §2 and NS-24 depend on both; neither is
    identified, and the M0 audit needs the rival's configuration frozen by someone.
12. **When five mechanisms land** (F23): the sampled referee, the hard-contact law, the energy tier, region
    certificates and the retrofit search each have a gate one or two milestones away from the skeleton.
13. **§0.6 is a placeholder** (`<<LEAD-PROVENANCE>>`), by design; it must be filled before the document is read
    by anyone outside the drafting process.
