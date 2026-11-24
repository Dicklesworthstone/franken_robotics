---
key: "N41-CX"
title: "registry: counterexamples and planted defects of the delivery registers, each a control the tooling must catch"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "delivery", "gauntlet", "frob-conformance", "solid", "registry", "test"]
parent: "EPIC-M0"
deps: ["BD-025", "N38a-40", "N38b-05", "N40-CX", "STD-E2E", "STD-TEST"]
needs: []
related: ["N00-CX", "N33-CX", "N34-CX", "N38-CX", "N38b-04", "N39-CX", "N41-01", "N41-02", "N41-03", "N41-04", "N41-06", "N41-15", "N41-16", "N41-17", "N41-19", "N41-20", "N41-21", "N41-22", "N41-23", "N41-24", "N41-25", "N41-26", "N41-27", "N41-28", "N41-29", "N41-31", "N41-32", "N41-35", "N41-37", "N41-38", "N41-KA", "N41-OPEN1", "N41-PM", "N41-REF"]
gates: ["GATE-GAUNT-01", "GATE-WS-07", "GATE-WS-08"]
plan_refs: ["§41.3", "§41.4", "§41.5", "§42.1", "§45.10", "§45.11", "§46", "§47.1", "§47.2", "§47.4", "§38.11", "Appendix N"]
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
states rules whose tempting violation is easy to name. This registry supplies the entries: for each rule, the
shortcut, the smallest planted case that takes it, and the rule the case protects. Each entry seeds a control in
the conformance registry.

A row here is a **defect**, not a boundary. "The release is refused" is not a control. "A release checker built
with the per-stage comparison reduced to any-stage accepts the planted cell" is: it names what was removed and
what then goes wrong. Where a row's case is an input rather than a mutant build, the mutant that would let it
through is named in the bead of the last column.

## Scope

In: the 58 entries, appended to the counterexample catalogue through one block of numbers; the planted inputs
(scratch-copy edit scripts and fixtures) that need no mechanism to exist; the compiled-in table of catching tests
(each entry carries its own `due`); the binding check and its script. Out, with the bead that has it: the catalogue's record schema
and the seeding tool that writes `crates/frob-conformance/registry/controls.toml`, allocates `NC-` numbers and
proves completeness by the join (N38b-05; this bead writes no row of `controls.toml`); the number blocks (N38a-40);
the control runner (N38b-04); the catching tests and the mutant tools that show a control can fail (the beads of
the last column); known answers (N41-KA); the reason a tool prints when it catches a row (N41-REF); the planted
instances of the risk register's own rows, which are controls of the hazards and not of the tooling (N41-32 to
N41-37); physics counterexamples (the chapter registries).

## Technical approach

**Entries.** Each row is one entry of `crates/frob-conformance/registry/counterexamples.toml` in the record schema
of N38b-05 and no other field: `id` (the tag until the block is issued, then the CX number), `shortcut`, `case`
(the planted case), `numeric = true`, `plan_case = "numbers"`, `case_supplied_by` (empty), `rule`, `rule_ids`
(GATE-GAUNT-01 for every row; GATE-WS-07 for X41-01 and X41-06 to X41-08; GATE-WS-08 for X41-02 to X41-05; the
sections and refusal codes the rule names), `defined_in`, `same_as` (as stated below), `label` (DESIGN),
`control_kind` (`planted-input` for a scratch record or manifest, `planted-design` for a scratch dependency graph
or the planted sampler, `mutation` for a mutant component), `plant`, `due` (Staging, below) and `slug` (the `X41-`
tag). A planted input or design is the edit script or
fixture `crates/frob-conformance/fixtures/delivery_controls/<plant>` applied only to a scratch root; a mutation is
built by the catching bead under the plant's feature reachable only under `gauntlet-mutations`. The entry's `plant`
is one plant record of N38b-05's form whose `id` is `<planting bead>:<plant>`: `N41-CX:` for a planted input or
design of this bead's fixture directory, `N40-CX:` for the four plants shared with that registry and `N33c-37:` for
its sampler fixture (both below), and for a mutation the catching bead named first in the last column, which
declares the feature of that name. The plant record's `target` is the rule column (the rule's check; X41-52 and
X41-53 state none, below), its `defect`
the planted case and its `expect` the catching test's refusal with the reason registered in N41-REF, and where the
expectation is a refusal its `removed_check` names the guard whose mutant would let the case through; the seeding
tool copies them to the control.
DESIGN, correcting an earlier draft of this bead that kept the rows in a private file
`registry/counterexamples/delivery.toml` with fields `mutant`, `caught_by`, `gate`, `bead` and `due` and wrote
`[[control]]` records itself: the registries named in the conventions are the only registry files, and only the
seeding tool writes `controls.toml`. The catching test and its bead are kept in one compiled-in table keyed
by CX number and slug in `crates/frob-conformance/tests/counterexamples_delivery.rs`; `due` is the entry's own
field of N38b-05's schema, which the seeding tool copies to the control's optional `due` (BD-025), so the table
holds no milestone.

**Numbers.** A bead never invents a number above the plan-time maximum (CX-384): this bead files one `[[block]]`
request in `crates/frob-conformance/registry/allocations.toml` (schema `frob.gauntlet.alloc/1`, N38a-40) with
`registry = "cx"`, `count = 58`, `owner = "N41-CX"`,
`section = "§41.3, §41.4, §41.5, §42.1, §45.10, §45.11, §46, §47"`; the
allocator fills `first` and `last`; the tags X41-01 to X41-58 are mapped to the block's numbers in table order in
one commit and kept in `slug`; the beads that name a row (N41-01, N41-03, N41-04, N41-31, N41-32, N41-38 and the
others) may go on citing the `X41-` alias in prose. A tag still used as an identifier in a registry, a test name or
a derivation file at M0 fails the ledger check `provisional-tag-past-due`; test names use the `cx_<nnn>_` prefix
(STD-TEST).

**Staging.** This registry is written at M0, but several of the beads that catch its rows land later. Each row's
`due` is the milestone of the earliest bead in its last column; until that bead lands, the row's control is
`stale` (its plant or catching test is not built) and is not run, and the binding check at an exit covers exactly
the rows due by that exit. A row past its `due` milestone whose control is still stale, or whose last outcome is not
`caught`, fails the exit (`suspect-undisposed`, the control review's reason, reused through N41-REF); that is the
exit's rule (GATE-GAUNT-01), not this bead's acceptance. With the milestones of the beads as they stand: 33 rows due
at M0 (X41-01, -06, -09 to -16, -18 to -22, -24, -25, -28, -32 to -37, -48 to -55, -58); 21 at M1 (X41-02 to -05, -26,
-27, -29 to -31, -38 to -47, -56, -57); 2 at M2 (X41-07, -23); 1 at M3 (X41-08); 1 at M5 (X41-17). X41-24 is due at M0
because the exit simulation of N41-04 attempts a synthetic M3 exit; N41-15 adds the real check at M2.

**Rows.**

| Tag | The tempting shortcut | The planted case | The rule it protects | Kind; plant | Caught by |
|---|---|---|---|---|---|
| X41-01 | A passing build means the pins are compatible | A second exact pin on one package, reachable only through an optional feature group that is off by default: the default build resolves, and the conflict appears when the group is first enabled at the oracle's milestone | Dependency resolution is checked with every optional feature group enabled | planted-design; `second-pin-behind-feature` | N41-02 |
| X41-02 | The run gave correct results and a receipt, so it shows the device path | A step plan in which one of eight stages fell back to the host; a release cell cites it for device throughput | A device cell needs requested backend equal to actual backend on every stage it claims | planted-input; `stage-fell-back-to-host` | N41-29 |
| X41-03 | A successful CPU run on the large Mac, or in a shell on the training machine, establishes the path | A device cell backed only by a CPU receipt | A CPU run establishes the CPU path and nothing about a device path | planted-input; `device-cell-cpu-receipt` | N41-29 |
| X41-04 | The published incumbent figure will do for a comparison | A comparison cell whose incumbent figure is published and not re-measured on the same hardware | A pinned external comparison is a re-measurement under the disclosure block | planted-input; `published-incumbent-figure` | N41-29 |
| X41-05 | An inconclusive study is a capability in progress | A cell whose study ended `inconclusive`, listed as a capability; a release note that rewords `tied` as a win | Tied, infeasible, incomparable and inconclusive are complete dispositions, printed under their own names | planted-input; `inconclusive-as-capability` | N41-29, N41-28 |
| X41-06 | Canonical bytes can come from the library's serializer | A dummy dependency that enables insertion-ordered maps and another number representation; a writer that serializes through the library's map has digests that move | Canonical bytes come only from the project's own writer; the stack lock records the unified feature set | planted-design; `dummy-dep-insertion-order` | N41-02 |
| X41-07 | The marginals are right, so the sampler is right | The linear-phase sampler: correct marginals, pairwise sign agreement 0.6655 where ½ ± 0.005 is required | The joint law of sampled actions is tested, and draws are keyed per channel | planted-design; `linear-phase-sampler` (the one fixture `frob_conformance::plants::linear_phase_sampler` of N33c-37) | N41-38 |
| X41-08 | A resampling interval on the sealed sample certifies the ratio | The rare-tail arm: a percentile bootstrap reports 20 where the population ratio is 1.8215, because 24 families miss the slow 1% with probability 0.99²⁴ = 0.78568 | A stratum below its necessary family count reports a finite-suite result, never a population claim | planted-input; `rare-tail-arm-bootstrap` | N41-31 |
| X41-09 | "We will keep an eye on it" | A risk row whose mitigation is an intention ("monitor throughput") with no gate, fallback or default | No risk is mitigated by a hope | planted-input; `hope-mitigation` | N41-01 |
| X41-10 | The upstream fix will land | A risk row whose only mitigation is an upstream work item, with no interim posture | An upstream work item is never a mitigation by itself; its interim posture is | planted-input; `bare-upstream-mitigation` | N41-01 |
| X41-11 | The signal fired, but everybody knows | A failing cell in the toolchain matrix at the M0 exit (the signal of RISK-02) with no recorded outcome | A fired signal ends in a kill gate decided, an ADR, or a written reason; otherwise the exit review is defective | planted-input; `fired-risk-without-outcome` | N41-01, N41-04 |
| X41-12 | Not readable yet means it did not fire | A row first readable at M3 recorded as `no` at the M0 exit | `not-readable` is a different reading from `no` | planted-input; `not-readable-as-no` | N41-01 |
| X41-13 | Renumber the list to keep it tidy | A retired row deleted and the later rows renumbered; a new risk entered under a retired number | A retired risk is tombstoned with the gate that retired it and is not renumbered | planted-input; `retired-row-renumbered` | N41-01 |
| X41-14 | The implementer knows the risks best | A risk review recorded by a session that implemented beads of the milestone | The register is read by the fresh-context session that records the exit | planted-input; `implementer-reviews-risks` | N41-01, N41-04 |
| X41-15 | A plant fired against a stand-in is enough | A planted instance run against a stubbed detector | A plant is fired against the real detector; a stub proves nothing | planted-input; `plant-stub-detector` | N41-32 |
| X41-16 | It was refused, so it is a control | A control record whose defect only restates a limit of the product, with no guard named | A boundary is not a control: the record names what was removed | planted-input; `boundary-only-control` | N41-32 |
| X41-17 | The detectors were shown to fire once | A detector disabled by a later change, after its plant last ran | From M5 the full sweep fires every row at every exit | mutation; `detector-disabled-after-plant` | N41-37 |
| X41-18 | Assume the catalogue price | An open question whose default adds a claim | A default removes a claim or a capability and never adds one | planted-input; `default-adds-claim` | N41-03 |
| X41-19 | Nobody answered, so carry on | A question past its deadline with no decision and no default record, still `open` | Nothing is decided by silence: at its deadline a question takes its default, recorded as an ADR | planted-input; `decided-by-silence` | N41-03, N41-04 |
| X41-20 | The owner said it is fine | A question whose row names a gate, closed by an owner's ADR with no measurement on record | Evidence decides where evidence exists | planted-input; `owner-adr-over-measurement` | N41-03 |
| X41-21 | Reuse the retired identifier | A second question entered under OPEN-SKEL-1 | A closed question is tombstoned, not reused | planted-input; `tombstone-reused` | N41-03, N41-OPEN1 |
| X41-22 | The default is only a label | A device Performance claim dated after the exploratory default was taken | A default in force is printed on the results it touches, and the claim it removed is refused | planted-input; `claim-under-default` | N41-04 |
| X41-23 | "Independently accepted" reads better | That phrase in a report while no neutral party is on record | A ratio measured without a neutral party is labelled `self-adjudicated` | planted-input; `independently-accepted-wording` | N41-06, N41-35 |
| X41-24 | Every default lets every exit through | The M3 exit attempted with the ten-fold compute envelope at its default and no stratum of the partial gate recorded | One default can hold an exit (the gate it leaves `not run` blocks, and the report names the question), and the question is asked two milestones early for that reason | planted-input; `m3-exit-on-default-envelope` | N41-15, N41-04 (through the naming check of N40-35) |
| X41-25 | Lower numbers first means no cycles, for ever | An edge from BD-005 to BD-014 added to the export, closing a cycle; after conversion a discovered bead may point to a higher number | After conversion the guarantee against cycles is the tracker's cycle check, not the numbering | planted-input; `cycle-edge-in-export` | N41-16 |
| X41-26 | A closed bead is acceptance | A status table generated from bead states | Acceptance is the gate's row in the gate register | planted-input; `status-from-bead-states` | N41-17 |
| X41-27 | One good close reason fits them all | 87 beads closed in 12 seconds with one reason text | A close reason shared by several beads is a failure signal, and the exit review rejects it | planted-input; `eighty-seven-shared-closes` | N41-17 |
| X41-28 | The gate has a bead somewhere | A gate added to the register with no bead; a gate that closes in parts with a bead for one part only | Every gate, and every part of a gate, is carried by a bead that can record it | planted-input; `gate-without-bead` | N41-16 |
| X41-29 | The kill gate fired; its dependents carry on | A dependent of a bead closed `fallback`, with no re-read note | Dependents are re-read: `superseded` with the ADR, or kept on the fallback | planted-input; `fallback-dependent-not-reread` | N41-17 |
| X41-30 | Start the frontier work; the pilot will follow | A Frontier bead in progress with no recorded pilot result | A bead for a Frontier or Moonshot mechanism is not ready until its pilot has a recorded result | planted-input; `frontier-without-pilot` | N41-17 |
| X41-31 | It is only a slightly bigger run | A run above the default envelope recorded with no approval cited | A bead whose run exceeds the envelope names its budget and waits for the owner's approval | planted-input; `envelope-exceeded-unapproved` | N41-17 |
| X41-32 | Repeat the step until it passes | A first-session step with a failed attempt, a passing second attempt and no recorded change between them | A failing result is recorded as failing, and no step is repeated until it happens to pass | planted-input; `repeat-until-pass` | N41-25 |
| X41-33 | Scaffold first, observe later | A session record with crates created and an observation missing | Each step produces an observation, not a scaffold; a missing step is written as `not run` | planted-input; `scaffold-without-observation` | N41-25 |
| X41-34 | A non-strict comparison is close enough | The docking classifier with ε ≤ (U + 1/2)/4 in place of ε < (U + 1/2)/4: exactly nine cells change | A strict decision is computed exactly | mutation; `non-strict-docking-classifier` | N41-24 |
| X41-35 | The digests matched, so the dependency is live | The forward-dynamics digest compared without perturbing an input | Perturb one input and require the digest to change | planted-input; `digest-without-perturbation` | N41-19 |
| X41-36 | Patch the sibling locally to get the build through | An edit in a sibling's checkout outside an upstream work item during the session | A sibling is changed only as an upstream work item, in its own repository, under its own rules | planted-input; `sibling-patched-locally` | N41-20 (`siblings_unchanged_by_step`), N41-25 |
| X41-37 | Put our number beside the rival's already | A figure for this product's speed in the M0 record beside the incumbent's phases | The denominator is measured before anything is built against it; M0 says nothing about our speed | planted-input; `own-speed-in-m0-record` | N41-23, N41-28 |
| X41-38 | Twelve-fold, not yet published | A done report with an unpublished ratio of 12; another with a published ratio of 2 and a complete ledger | No particular ratio is required; publication is | planted-input; `unpublished-twelvefold` | N41-26 |
| X41-39 | The kill gate fired, so the clause is waived | A fired kill gate with no ADR and no fallback name | A clause is true by fallback only when the fallback is in force, recorded as an ADR and described by its own name | planted-input; `fired-kill-without-adr` | N41-26 |
| X41-40 | "Mission synthesis" is a fair name for the cell | A release cell named by a cluster where its qualifying fixture covers one exact static family | No capability is described more broadly than the fixture that qualified it | planted-input; `cell-named-by-cluster` | N41-27 |
| X41-41 | The demo shows the numbers | A demo citing a claim absent from the gate register; a demo that takes 61 s | A demo shows that a path runs in under 60 s; the numbers come from the gate | planted-input; `demo-claim-or-61s` | N41-27 |
| X41-42 | Only the wins need to reproduce | A release with no reproducible result that went against the plan | A losing experiment reproduces as readily as a winning one | planted-input; `no-losing-result` | N41-27 |
| X41-43 | Fix the status table by hand | A committed status table that differs from the one generated from the gate register; a failing gate missing from it | The status table is generated; a failing gate appears as failing | planted-input; `hand-status-table` | N41-27 |
| X41-44 | A withdrawn capability can be dropped quietly | A release table that omits a capability withdrawn by a fired kill gate | It stays visible as withdrawn, with the ADR | planted-input; `withdrawn-capability-omitted` | N41-28 |
| X41-45 | Say it a milestone early | A throughput figure or the word "qualified" in an M1 note; "ten-fold" in an M3 bundle with no permitting record | Each milestone has statements that must not be made until their evidence exists | planted-input; `statement-a-milestone-early` | N41-28 |
| X41-46 | Done needs the Moonshots | A done clause edited to require a gate whose only bead is a Moonshot | No Moonshot item is required for done | planted-input; `done-needs-moonshot` | N41-26 |
| X41-47 | A target is as good as a measurement | A gate recorded `pass` with a target in place of a measured value, or with no caught control | A clause counts a gate only with a measured value and a caught negative control | planted-input; `target-as-measurement` | N41-26 |
| X41-48 | Every gate is green, so the claim holds | An M0 exit with no incumbent phase measurement on record | Outcome-level reasons are read at every exit beside the gates, and an adverse one is answered in writing | planted-input; `m0-without-incumbent-phase` | N41-PM, N41-04 |
| X41-49 | It is finished; nothing needs to wait for it | A bead that is not an ancestor of its milestone's exit review | Every bead of a milestone is an ancestor of that milestone's exit review | planted-input; `bead-not-ancestor-of-exit` | N41-16 |
| X41-50 | Close to the Rust loop is close enough | The kernel comparison run with an absolute tolerance of 10⁻⁵: a one-ulp difference in one backend passes | One algebraic kernel is compared bit for bit on the three backends | mutation; `kernel-tolerance-comparison` | N41-21 (`bit_comparison_reports_one_ulp`), N41-22 (the one-ulp plant on CUDA) |
| X41-51 | Tune a fixture constant later | A constant of the cart, sill and dock fixture changed after the session with the fixture's identity unchanged | A constant changed after this point changes the fixture's identity | planted-input; `fixture-constant-changed` | N41-24 |
| X41-52 | The printed table is the register | The severity tally taken over the printed rows RISK-01 to RISK-55 only: 25 High, 28 Medium, 2 Low, where the 59 rows give 28, 29 and 2 | The register is 59 rows: the four late rows RISK-56 to RISK-59 (3 High, 1 Medium) are counted with the rest in every tally | mutation; `severity-tally-printed-rows-only` | N41-01 (`severity_tally_from_rows`) |
| X41-53 | Edges have no direction when counting | The widest beads of the seed graph computed on the reversed edge direction (dependents in place of dependencies), which names other beads than BD-312 and BD-376 (24 dependencies each) | A bead's width is the number of its dependencies; a graph fact is computed on the direction the tracker stores | mutation; `widest-by-dependents` | N41-16 (`seed_widest_24`) |
| X41-54 | List the orphan as related and the exemption covers it | A bead in BD-378's `related` that is no item of the upstream register, has no item among its ancestors and is not an ancestor of the exit review | The exemption from the ancestor rule needs both the exit review's `related` and an upstream item (the bead or one of its ancestors; never FD-ASUP-01 or FD-FMCP-01); `related` alone hides no orphan (`tracker-orphan`) | planted-input; `orphan-hidden-in-related` | N41-16 (`related_does_not_hide_orphan`) |
| X41-55 | No unlabelled bead depends on a sibling item, so no exit waits on one | An edge from BD-224 to N33b-04 and from N33b-04 to the M1 item FD-FSIM-11: the exit review reaches the item through a labelled in-repository bead, which the direct-edge lint does not see | No milestone waits on a sibling change landing: an exit review with a non-P0 upstream item among its ancestors is listed with one path to it (the warning `tracker-exit-waits-on-upstream`) | planted-input; `exit-waits-through-labelled-bead` | N41-16 (`exit_waiting_on_upstream_listed`) |
| X41-56 | The gate failed and was recorded, so the bead closes `recorded` | A `recorded` close on a gate no kill row reads; and one on a read gate whose trigger names a repair cycle, with no `note` line opening the cycle before the cited `fail` line | `recorded` closes only the measurement of a gate that a kill gate's trigger reads, on a `fail` line recorded by another session with every control caught and, where the trigger names a cycle, after the line that opened it (`recorded-close-invalid`) | planted-input; `recorded-close-without-cycle` | N41-17 (`recorded_close_after_repair_cycle`) |
| X41-57 | Any decision record will do behind a `withdrawn` close | A fallback-work bead closed `withdrawn: not needed: KILL-X not fired, docs/adr/ADR-KILL-X-M2.md` while the ADR's entry in force is `fired` | A `withdrawn` close that cites a decision cites one that exists and says what the close says (`withdrawn-without-decision`) | planted-input; `withdrawn-on-contrary-adr` | N41-17 (`untaken_branch_withdrawn_needs_adr`) |
| X41-58 | A decision taken after its exit review needs no later exit to wait on it | An M1 bead depending on BD-147 that is no ancestor of BD-224 | A bead with its milestone's exit review among its ancestors is an ancestor of the next exit review, and after the M8 review (BD-442) there is none (`tracker-orphan`) | planted-input; `post-exit-bead-not-held` | N41-16 (`post_exit_bead_held_to_next_exit`) |

**Rows whose case is stated elsewhere (DESIGN, following N38b-05: one control per entry; the same case stated in
two places keeps two entries and two controls, each caught by its own rule's check and gates).** An earlier draft
of this bead let four rows name a control of the gate-register registry and write no record of their own; that is
replaced by one shared plant and two entries, so the case is planted from one fixture while each detector keeps its
own history of outcomes:

- X41-11, X41-18, X41-19 and X41-21 state the cases of N40-CX's RC-17, RC-53, RC-52 and RC-54. The rule lives in
  this unit (N41-01 and N41-03 test it where it lives; N40-05 and N40-35 replay it through the exit runner), so each
  keeps its entry here against GATE-GAUNT-01 and uses the plant of the same name from N40-CX
  (`fired-risk-without-outcome`, `default-adds-claim`, `decided-by-silence`, `tombstone-reused`); the entry numbered
  later carries `same_as` naming the earlier, and since this bead depends on N40-CX (it uses that bead's four
  fixtures) and files its block request after it, the four entries here carry `same_as` the numbers issued to RC-17,
  RC-53, RC-52 and RC-54.
- X41-27 states CX-002's case (87 beads closed within 12 seconds with one shared reason) and carries
  `same_as = "CX-002"`; X41-16 states CX-399's (a boundary registered as a control) and carries `same_as =
  "CX-399"`. Both targets are numbered already, and an appended entry's `same_as` does not touch the plan-time count
  of thirteen.
- X41-01, X41-06, X41-07 and X41-08 are the risk register's planted instances for GATE-WS-07; their cases are also
  stated by the suite (N33-CX `suite-pins-compatible`, GATE-WS-06), the workspace (N34-CX `ws-json-library-map`,
  GATE-WS-05), the learning chapter and the suite (N23-CX `CX-23-L1` and N33-CX `suite-sampler-marginals`, with the
  one linear-phase sampler of N33c-37) and the ten-fold harness (N39-CX TF-CX-16, GATE-TENFOLD-10). Each keeps its
  own entry; the one numbered later carries `same_as` the earliest; N41-32's binding rule (reuse first) attaches the
  risk row to these controls rather than registering a third.
- X41-14 and RC-03, X41-29 and RC-45, X41-43 and RC-06, X41-47 and RC-01, RC-02, RC-05 protect related rules at
  different tools (the risk review against the gate record; the tracker against the kill ledger; the release table
  against the README table; the done check against the status register) with different cases; they are not ≡.
- X41-52 and X41-53 are the controls of two known answers of N41-KA, K41-04 (the severity tally) and K41-23 (the
  widest beads). Each row's control cell there names the entry and its plant (`N41-01:severity-tally-printed-rows-only`,
  `N41-16:widest-by-dependents`), and the seeding tool links the control seeded from the entry to the row
  (`control.nc`), writing no second control for the same plant. The two plant records state no `target`, so the
  control's target is the plant's `id`, which is the target a known-answer cell gives the plant it names: N38b-05's
  step 3 then finds a control with the same plant and target and appends its NC instead of allocating another.

**Binding check.** For every row: the entry exists and seeds one control; once the catching bead is closed, the
test named in the table exists and fails (refuses with the registered reason) on the planted case, and where the
row names a mutant tool, that tool built under its feature misses the case, so the control is known to be able to
fail. A row whose catching bead has not landed is reported `stale` with the bead. A row whose control was never
caught in recorded history is listed by the control runner's suspect review at each exit.

## Acceptance criteria

- [ ] `counterexamples.toml` holds the 58 entries X41-01 to X41-58 in the schema of N38b-05 and no other field, each
      with non-empty shortcut, case, rule, plant, gates and `due`, and the test file's table names a catching test
      and its bead for each; the due counts are 33 at M0, 21 at M1, 2 at M2, 1 at M3 and 1 at M5; no private file
      `registry/counterexamples/delivery.toml` exists.
- [ ] The block request for 58 numbers is committed; after issue the entries carry the block's numbers in table
      order with their `X41-` tags in `slug`; `registry::alloc::validate` passes and no tag remains as an
      identifier at M0; X41-16 and X41-27 carry `same_as` CX-399 and CX-002, and X41-11, X41-18, X41-19 and X41-21
      the numbers of RC-17, RC-53, RC-52 and RC-54.
- [ ] After the seeding tool of N38b-05 is re-run, each entry has exactly one control with a non-empty `gate` list;
      the second run changes no byte; the join passes; this bead writes no record of `controls.toml`.
- [ ] The binding check passes for every row whose catching bead is closed and reports the others `stale` with
      their beads; the planted inputs this bead writes load, and the mutant tools named are checked once their
      beads land.
- [ ] No row's plant is applied to a real register, the real tracker export, a sibling repository or a real
      release manifest; the digests of those files are unchanged by a run of all rows.
- [ ] Rows X41-01 to X41-08 agree with the planted instances and planted cells recorded for GATE-WS-07 and
      GATE-WS-08 (same plant, same expected signal).
- [ ] `scripts/e2e/registry_n41_cx.sh` passes and rejects each of its planted registry defects.

## Test plan

### Unit tests

`crates/frob-conformance/tests/counterexamples_delivery.rs`, run as `cargo test -p frob-conformance --test
counterexamples_delivery` (real registry files and scratch copies, no mocks): `fifty_eight_entries_load`;
`every_entry_seeds_a_control` (on a scratch seeding run); `every_row_names_existing_test` (for rows whose
catching bead is closed; a later row's symbol is checked when its bead lands); `due_counts_33_21_2_1_1` (each row's
`due` equals the earliest milestone among its catching beads, read from the bead index); `stale_past_due_fails_exit`
(X41-07 left stale on a scratch M2 exit gives `suspect-undisposed`); `plants_never_touch_real_files` (digests
before and after); `schema_fields_only` (a scratch entry carrying `caught_by` is rejected by the loader);
`block_request_of_58`; `shared_plants_resolve` (X41-11, -18, -19, -21 name plants that exist in N40-CX's fixture
directory; X41-07 names the linear-phase sampler of N33c-37); `same_as_targets_exist` (CX-002, CX-399, and the four
entries of N40-CX); one
parametrized test `row_is_caught(row)`, named `cx_<nnn>_<plant>`, that applies each plant on a scratch root and
asserts the registered reason once the catching bead has landed; `mutant_misses(row)` for the rows with a mutant
tool (the per-stage comparison reduced to any-stage for X41-02; the default-mode resolution for X41-01; the runner
that treats a fired signal as a warning for X41-11; the runner that lets a due question pass for X41-19; the
tolerance comparison for X41-50; the tally over the printed rows for X41-52; the width by dependents for X41-53),
run once those beads land.

### End-to-end script and logging

`scripts/e2e/registry_n41_cx.sh` (this bead; header `bead=N41-CX milestone=M0 labels=delivery,registry
plants=drop-plant,private-field,missing-same-as-target,stale-tag`): runs the test file; re-runs the seeding tool
(`cargo test -p frob-conformance --test seed_controls -- --ignored seed`) and the join (`cargo test -p
frob-conformance --test seed_join`) on a scratch copy of `registry/` and asserts by digest that a second seeding
run is a no-op; plants, one at a time on scratch copies, an entry with its plant removed, an entry carrying a
private field, X41-27 with `same_as = "CX-999"`, and a tag left as an identifier at M0, and asserts each is
rejected; then, for each row whose catching bead has landed, applies the plant on a scratch root and runs the
catching test in the form of ADR-GAUNT-01 (`FROB_TEST_SEED=0 cargo test -p frob-conformance --test <tier from the
gate record> -- <row>`), asserting the registered reason. It calls no `frob` verb. It sources the STD-E2E helper
and writes `target/e2e-logs/registry_n41_cx/<utc-timestamp>.ndjson` with a transcript: every line has `ts`,
`script`, `bead`, `step`, `event`; `cmd` lines with the exact command line, exit code and wall time; the evidence
block (claim Known answer, class Tested, determinism D0); one `assert` line per row with the CX number, the `X41-`
slug, the expected reason, the observed reason, the exit code and the scratch path; `metric` lines with caught,
stale and due by milestone; the digests of the real registers before and after (equal); a `summary` line naming
GATE-GAUNT-01, GATE-WS-07 and GATE-WS-08. The rows are also applied by every end-to-end script of this unit that
concerns them, each logging one `assert` line per row through the same helper: `scripts/e2e/registers_exit.sh`
(X41-09 to X41-14, X41-18 to X41-22, X41-24, X41-48), `owner_prog2.sh` (X41-23), `owner_tenfold_inputs.sh` (X41-24
at the real M3-exit check), `ws07_build_plants.sh` (X41-01, X41-06) and `ws07_late_plants.sh` (X41-07, X41-08),
`risk_plants.sh` (X41-15 to X41-17), `tracker_hygiene.sh` and `bead_lifecycle.sh` (X41-25 to X41-31, X41-49,
X41-54 to X41-58), `ka_delivery_registry.sh` (N41-KA: X41-52 and X41-53, run as the controls of K41-04 and K41-23), the
first-session step scripts `scripts/session/first/step-2.sh` to `step-8.sh` and `first_session_record.sh` (X41-32 to
X41-37, X41-50, X41-51), `done_check.sh` and `release_check.sh` (X41-02 to X41-05, X41-38 to X41-47; the gate
stage of `release_check.sh`, N41-30, runs the controls seeded from X41-03, X41-02, X41-04 and X41-05, in the order
of its planted cells W1 to W4, as GATE-WS-08's registered controls through the tier entry with
`--features gauntlet-mutations`).

### Known answers and negative controls

This bead is the negative-control registry of the unit's tooling. Known answers used by rows: nine cells for
X41-34; 0.6655 and ½ ± 0.005 for X41-07; 20 against 1.8215 and 0.99²⁴ = 0.78568 for X41-08; 87 closes in 12 s for
X41-27; 61 s against 60 s for X41-41; 12 unpublished against 2 published for X41-38; 25, 28 and 2 against 28, 29
and 2 for X41-52; BD-312 and BD-376 with 24 dependencies each for X41-53. Controls of the registry
itself: the four planted defects of the script. Determinism: every row is exact; identical outcomes on Linux and
macOS.

## Considerations

- The rows are small on purpose: the smallest case that takes the shortcut. A larger case hides which rule
  caught it.
- A row whose tool changes its message is reported `caught-unexpected` by the control runner, which counts as
  missed until the expectation is corrected in the same change.
- Rows added later take the next tag and are numbered through a new block; identifiers are never reused, and a row
  that no longer applies is withdrawn with a reason and stays in the file.
- These rows overlap in intent with the verification chapter's rules against false progress (typed close reasons,
  no self-certification). Where the same planted case is stated there, the row shares its plant and carries
  `same_as`, keeping its own control (see above).

## References

§41.3 (the register's rows and severities), §41.4 (rules of the risk register), §41.5 (the counterexample to "a
passing build means the pins are compatible"; GATE-WS-07), §42.1 (the decision rule), §45.10 (the seed graph's
facts), §45.11 (conversion rules), §46 (working rules of the first session),
§47.1, §47.2, §47.4 (the counterexample of the stage that fell back; GATE-WS-08), §38.11; Appendix N (the printed
catalogue, which has no section for these chapters); CX-002, CX-399.
