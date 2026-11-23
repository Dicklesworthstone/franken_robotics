---
key: "N41-CX"
title: "registry: counterexamples and planted defects of the delivery registers, each a control the tooling must catch"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "EPIC-M0"
deps: ["BD-025", "STD-TEST"]
needs: ["Appendix N: the printed counterexample catalogue has no section for the risk, open-question, bead, first-session or definition-of-done chapters; if these rows are to receive CX and NC numbers, the unit that owns the catalogue's numbering assigns them (numbers are append-only)", "§38.11: the control runner that applies each registered control and records whether it was caught (N38b-04)"]
related: ["N41-KA", "N41-REF", "N41-01", "N41-02", "N41-03", "N41-04", "N41-16", "N41-17", "N41-18", "N41-19", "N41-20", "N41-23", "N41-24", "N41-25", "N41-26", "N41-27", "N41-28", "N41-29", "N41-30", "N41-31", "N41-32", "N41-37", "N41-PM", "N40-CX"]
gates: ["GATE-GAUNT-01", "GATE-WS-07", "GATE-WS-08"]
plan_refs: ["§41.4", "§41.5", "§42.1", "§45.11", "§46", "§47.1", "§47.2", "§47.4", "§38.11", "Appendix N"]
---
## Background

The program's rule is that a gate which no deficient model can fail validates nothing: every gate has a
registered **negative control**, a planted defect its test must catch. The rule applies with full force to the
tooling that keeps the program honest about itself. A risk review that nothing can block, an open-question
register that lets silence decide, a tracker check that accepts a shared close reason, a release checker that
accepts a device claim backed by a CPU run: each is the failure the delivery chapters exist to prevent.

The printed counterexample catalogue is organized by subsystem and has **no section** for the five delivery
chapters this unit carries. Two of those chapters state counterexamples in so many words (the build that never
enables the oracle's feature; the step plan in which one stage fell back to the host), and each of the others
states rules whose tempting violation is easy to name. This registry supplies the rows: for each rule, the
shortcut, the smallest planted case that takes it, and the rule the case protects. Each row seeds a control in
the conformance registry.

A row here is a **defect**, not a boundary. "The release is refused" is not a control. "A release checker built
with the per-stage comparison reduced to any-stage accepts the planted cell" is: it names what was removed and
what then goes wrong. Where a row's case is an input rather than a mutant build, the mutant that would let it
through is named in the bead of the last column.

## Scope

In: the rows; their home in the conformance registry; the binding check that each plant is caught. Out: known
answers (N41-KA); the reason a tool prints when it catches a row (N41-REF); the planted instances of the risk
register's own rows, which are controls of the hazards and not of the tooling (N41-32 to N41-37); physics
counterexamples (the chapter registries).

## Technical approach

**File.** `crates/frob-conformance/registry/counterexamples/delivery.toml` (DESIGN path): `id`, `shortcut`, `case`,
`rule`, `mutant` (a scratch-copy edit script or a test-only fixture; never applied to the real registers),
`caught_by` (test symbol), `gate`, `bead`. Each row also becomes a `[[control]]` record in the control registry,
of kind `planted-input` for a scratch record or manifest and `mutation` for a mutant tool, attached to
GATE-GAUNT-01 and, for rows X41-01 to X41-08, to GATE-WS-07 or GATE-WS-08.

**Rows.**

| Row | The tempting shortcut | The planted case | The rule it protects | Caught by |
|---|---|---|---|---|
| X41-01 | A passing build means the pins are compatible | A second exact pin on one package, reachable only through an optional feature group that is off by default: the default build resolves, and the conflict appears when the group is first enabled at the oracle's milestone | Dependency resolution is checked with every optional feature group enabled | N41-02 |
| X41-02 | The run gave correct results and a receipt, so it shows the device path | A step plan in which one of eight stages fell back to the host; a release cell cites it for device throughput | A device cell needs requested backend equal to actual backend on every stage it claims | N41-29 |
| X41-03 | A successful CPU run on the large Mac, or in a shell on the training machine, establishes the path | A device cell backed only by a CPU receipt | A CPU run establishes the CPU path and nothing about a device path | N41-29 |
| X41-04 | The published incumbent figure will do for a comparison | A comparison cell whose incumbent figure is published and not re-measured on the same hardware | A pinned external comparison is a re-measurement under the disclosure block | N41-29 |
| X41-05 | An inconclusive study is a capability in progress | A cell whose study ended `inconclusive`, listed as a capability; a release note that rewords `tied` as a win | Tied, infeasible, incomparable and inconclusive are complete dispositions, printed under their own names | N41-29, N41-28 |
| X41-06 | Canonical bytes can come from the library's serializer | A dummy dependency that enables insertion-ordered maps and another number representation; a writer that serializes through the library's map has digests that move | Canonical bytes come only from the project's own writer; the stack lock records the unified feature set | N41-02 |
| X41-07 | The marginals are right, so the sampler is right | The linear-phase sampler: correct marginals, pairwise sign agreement 0.6655 where ½ ± 0.005 is required | The joint law of sampled actions is tested, and draws are keyed per channel | N41-31 |
| X41-08 | A resampling interval on the sealed sample certifies the ratio | The rare-tail arm: a percentile bootstrap reports 20 where the population ratio is 1.8215, because 24 families miss the slow 1% with probability 0.78568 | A stratum below its necessary family count reports a finite-suite result, never a population claim | N41-31 |
| X41-09 | "We will keep an eye on it" | A risk row whose mitigation is an intention ("monitor throughput") with no gate, fallback or default | No risk is mitigated by a hope | N41-01 |
| X41-10 | The upstream fix will land | A risk row whose only mitigation is an upstream work item, with no interim posture | An upstream work item is never a mitigation by itself; its interim posture is | N41-01 |
| X41-11 | The signal fired, but everybody knows | A failing cell in the toolchain matrix at the M0 exit (the signal of RISK-02) with no recorded outcome | A fired signal ends in a kill gate decided, an ADR, or a written reason; otherwise the exit review is defective | N41-01, N41-04 |
| X41-12 | Not readable yet means it did not fire | A row first readable at M3 recorded as `no` at the M0 exit | `not-readable` is a different reading from `no` | N41-01 |
| X41-13 | Renumber the list to keep it tidy | A retired row deleted and the later rows renumbered; a new risk entered under a retired number | A retired risk is tombstoned with the gate that retired it and is not renumbered | N41-01 |
| X41-14 | The implementer knows the risks best | A risk review recorded by a session that implemented beads of the milestone | The register is read by the fresh-context session that records the exit | N41-01, N41-04 |
| X41-15 | A plant fired against a stand-in is enough | A planted instance run against a stubbed detector | A plant is fired against the real detector; a stub proves nothing | N41-32 |
| X41-16 | It was refused, so it is a control | A control record whose defect only restates a limit of the product, with no guard named | A boundary is not a control: the record names what was removed | N41-32 |
| X41-17 | The detectors were shown to fire once | A detector disabled by a later change, after its plant last ran | From M5 the full sweep fires every row at every exit | N41-37 |
| X41-18 | Assume the catalogue price | An open question whose default adds a claim | A default removes a claim or a capability and never adds one | N41-03 |
| X41-19 | Nobody answered, so carry on | A question past its deadline with no decision and no default record, still `open` | Nothing is decided by silence: at its deadline a question takes its default, recorded as an ADR | N41-03, N41-04 |
| X41-20 | The owner said it is fine | A question whose row names a gate, closed by an owner's ADR with no measurement on record | Evidence decides where evidence exists | N41-03 |
| X41-21 | Reuse the retired identifier | A second question entered under OPEN-SKEL-1 | A closed question is tombstoned, not reused | N41-03, N41-OPEN1 |
| X41-22 | The default is only a label | A device Performance claim dated after the exploratory default was taken | A default in force is printed on the results it touches, and the claim it removed is refused | N41-04 |
| X41-23 | "Independently accepted" reads better | That phrase in a report while no neutral party is on record | A ratio measured without a neutral party is labelled `self-adjudicated` | N41-06, N41-35 |
| X41-24 | Every default lets every exit through | The M3 exit attempted with the ten-fold compute envelope at its default | One default blocks an exit, and the question is asked two milestones early for that reason | N41-15, N41-04 |
| X41-25 | Lower numbers first means no cycles, for ever | An edge from BD-005 to BD-014 added to the export, closing a cycle; after conversion a discovered bead may point to a higher number | After conversion the guarantee against cycles is the tracker's cycle check, not the numbering | N41-16 |
| X41-26 | A closed bead is acceptance | A status table generated from bead states | Acceptance is the gate's row in the gate register | N41-17 |
| X41-27 | One good close reason fits them all | 87 beads closed in 12 seconds with one reason text | A close reason shared by several beads is a failure signal, and the exit review rejects it | N41-17 |
| X41-28 | The gate has a bead somewhere | A gate added to the register with no bead; a gate that closes in parts with a bead for one part only | Every gate, and every part of a gate, is carried by a bead that can record it | N41-16 |
| X41-29 | The kill gate fired; its dependents carry on | A dependent of a bead closed `fallback`, with no re-read note | Dependents are re-read: `superseded` with the ADR, or kept on the fallback | N41-17 |
| X41-30 | Start the frontier work; the pilot will follow | A Frontier bead in progress with no recorded pilot result | A bead for a Frontier or Moonshot mechanism is not ready until its pilot has a recorded result | N41-17 |
| X41-31 | It is only a slightly bigger run | A run above the default envelope recorded with no approval cited | A bead whose run exceeds the envelope names its budget and waits for the owner's approval | N41-17 |
| X41-32 | Repeat the step until it passes | A first-session step with a failed attempt, a passing second attempt and no recorded change between them | A failing result is recorded as failing, and no step is repeated until it happens to pass | N41-25 |
| X41-33 | Scaffold first, observe later | A session record with crates created and an observation missing | Each step produces an observation, not a scaffold; a missing step is written as `not run` | N41-25 |
| X41-34 | A non-strict comparison is close enough | The docking classifier with ε ≤ (U + 1/2)/4 in place of ε < (U + 1/2)/4: exactly nine cells change | A strict decision is computed exactly | N41-24 |
| X41-35 | The digests matched, so the dependency is live | The forward-dynamics digest compared without perturbing an input | Perturb one input and require the digest to change | N41-19 |
| X41-36 | Patch the sibling locally to get the build through | An edit in a sibling's checkout outside an upstream work item during the session | A sibling is changed only as an upstream work item, in its own repository, under its own rules | N41-20, N41-25 |
| X41-37 | Put our number beside the rival's already | A figure for this product's speed in the M0 record beside the incumbent's phases | The denominator is measured before anything is built against it; M0 says nothing about our speed | N41-23, N41-28 |
| X41-38 | Twelve-fold, not yet published | A done report with an unpublished ratio of 12; another with a published ratio of 2 and a complete ledger | No particular ratio is required; publication is | N41-26 |
| X41-39 | The kill gate fired, so the clause is waived | A fired kill gate with no ADR and no fallback name | A clause is true by fallback only when the fallback is in force, recorded as an ADR and described by its own name | N41-26 |
| X41-40 | "Mission synthesis" is a fair name for the cell | A release cell named by a cluster where its qualifying fixture covers one exact static family | No capability is described more broadly than the fixture that qualified it | N41-27 |
| X41-41 | The demo shows the numbers | A demo citing a claim absent from the gate register; a demo that takes 61 s | A demo shows that a path runs in under 60 s; the numbers come from the gate | N41-27 |
| X41-42 | Only the wins need to reproduce | A release with no reproducible result that went against the plan | A losing experiment reproduces as readily as a winning one | N41-27 |
| X41-43 | Fix the status table by hand | A committed status table that differs from the one generated from the gate register; a failing gate missing from it | The status table is generated; a failing gate appears as failing | N41-27 |
| X41-44 | A withdrawn capability can be dropped quietly | A release table that omits a capability withdrawn by a fired kill gate | It stays visible as withdrawn, with the ADR | N41-28 |
| X41-45 | Say it a milestone early | A throughput figure or the word "qualified" in an M1 note; "ten-fold" in an M3 bundle with no permitting record | Each milestone has statements that must not be made until their evidence exists | N41-28 |
| X41-46 | Done needs the Moonshots | A done clause edited to require a gate whose only bead is a Moonshot | No Moonshot item is required for done | N41-26 |
| X41-47 | A target is as good as a measurement | A gate recorded `pass` with a target in place of a measured value, or with no caught control | A clause counts a gate only with a measured value and a caught negative control | N41-26 |
| X41-48 | Every gate is green, so the claim holds | An M0 exit with no incumbent phase measurement on record | Outcome-level reasons are read at every exit beside the gates, and an adverse one is answered in writing | N41-PM, N41-04 |
| X41-49 | It is finished; nothing needs to wait for it | A bead that is not an ancestor of its milestone's exit review | Every bead of a milestone is an ancestor of that milestone's exit review | N41-16 |
| X41-50 | Close to the Rust loop is close enough | The kernel comparison run with a tolerance: a one-ulp difference in one backend passes | One algebraic kernel is compared bit for bit on the three backends | N41-21, N41-22 |
| X41-51 | Tune a fixture constant later | A constant of the cart, sill and dock fixture changed after the session with the fixture's identity unchanged | A constant changed after this point changes the fixture's identity | N41-24 |

**Binding check.** For every row: the control exists in the control registry, names the test in `caught_by`, and
that test fails when the tool is replaced by the row's mutant (or passes the planted input through). A row whose
control was never caught in recorded history is listed by the control runner's suspect review at each exit.

## Acceptance criteria

- [ ] `delivery.toml` holds the 51 rows with non-empty shortcut, case, rule and `caught_by`; each has a
      `[[control]]` record whose `gate` list is non-empty.
- [ ] Each planted case is caught by the named test: the tool fails or refuses with the reason registered for it
      (N41-REF).
- [ ] For each row whose case is an input, the mutant tool that would let it through exists as a test-only build
      and is shown to miss it (so that the control is known to be able to fail).
- [ ] No row's plant is applied to a real register, the real tracker export, a sibling repository or a real
      release manifest; the digests of those files are unchanged by a run of all rows.
- [ ] Rows X41-01 to X41-08 agree with the planted instances and planted cells recorded for GATE-WS-07 and
      GATE-WS-08.
- [ ] The all-controls run of the verification harness includes these rows and reports each as `caught`.

## Test plan

### Unit tests
`crates/frob-conformance/tests/counterexamples_delivery.rs`: `fifty_one_rows_load`; `every_row_has_control`;
`every_row_names_existing_test`; `plants_never_touch_real_files` (digests before and after); one parametrized
test `row_is_caught(row)` that applies each plant on a scratch root and asserts the registered reason;
`mutant_misses(row)` for the rows with a mutant tool (the per-stage comparison reduced to any-stage for X41-02;
the default-mode resolution for X41-01; the runner that treats a fired signal as a warning for X41-11; the runner
that lets a due question pass for X41-19; the tolerance comparison for X41-50).

### End-to-end script and logging
No script of its own. Every end-to-end script of this unit applies the rows that concern it and logs one `assert`
line per row with the row identifier, the expected reason and the observed reason: `registers_exit.sh` (X41-09 to
X41-14, X41-18 to X41-24, X41-48), `ws07_build_plants.sh` and the late-row script (X41-01, X41-06 to X41-08),
`risk_plants.sh` (X41-15 to X41-17), `tracker_hygiene.sh` and `bead_lifecycle.sh` (X41-25 to X41-31, X41-49), the
first-session scripts (X41-32 to X41-37, X41-50, X41-51), `done_check.sh` and `release_check.sh` (X41-02 to
X41-05, X41-38 to X41-47).

### Known answers and negative controls
This bead is the negative-control registry of the unit's tooling. Known answers used by rows: nine cells for
X41-34; 0.6655 and ½ ± 0.005 for X41-07; 20 against 1.8215 and 0.78568 for X41-08; 87 closes in 12 s for X41-27;
61 s against 60 s for X41-41; 12 unpublished against 2 published for X41-38. Determinism: every row is exact;
identical outcomes on Linux and macOS.

## Considerations

- The rows are small on purpose: the smallest case that takes the shortcut. A larger case hides which rule
  caught it.
- A row whose tool changes its message is reported `caught-unexpected` by the control runner, which counts as
  missed until the expectation is corrected in the same change.
- Rows added later take the next number; identifiers are never reused, and a row that no longer applies is
  withdrawn with a reason and stays in the file.
- These rows overlap in intent with the verification chapter's rules against false progress (typed close reasons,
  no self-certification). Where the same planted case is already registered there, the row names that control
  and no second record is written.

## References

§41.4 (rules of the risk register), §41.5 (the counterexample to "a passing build means the pins are compatible";
GATE-WS-07), §42.1 (the decision rule), §45.11 (conversion rules), §46 (working rules of the first session),
§47.1, §47.2, §47.4 (the counterexample of the stage that fell back; GATE-WS-08), §38.11; Appendix N (the printed
catalogue, which has no section for these chapters).
