# Coverage of drafting unit U31b

Chapter §31 (Foundry: structure, manufacture, lifecycle), subsections §31.17 to §31.23, and the ten seed beads
of the chapter. Output: 56 bead files: 10 seed beads (BD-400, BD-402 to BD-408, BD-423, BD-438), 44 new beads
(N31b-01 to N31b-44) and 2 registry beads (N31-CX, N31-REF). The known-answer registry `N31-KA` is written by
unit U31a; this unit's derivations bead N31b-04 and battery N31b-05 carry and wire its rows.

Reading note for the lead. Unit U31a owns §31.1 to §31.16 and has no seed bead; this unit owns every seed bead
of the chapter, and those seeds draw on U31a's subsections (§31.6 to §31.14). Each seed bead here is written
completely from those subsections, as the brief requires, so that it stands alone. U31a's directory was empty
while this unit worked, so any new bead U31a writes for the same mechanisms could not be cross-referenced by
key; the two epics (BD-423, BD-438) name U31a's expected mechanism beads in `needs` and carry the full
specification themselves in case none exists. The seed bead BD-401 (beam rung, `foundry.size`, GATE-FOUNDRY-01)
is written by unit U33b; its `needs` for "the structure-loop pilot" and for "§31.23" resolve to N31b-06 and to
N31b-38 to N31b-43 here.

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §31.17 | Invariants and proof obligations | N31b-01 (INV-FOUNDRY-01 to -07 as property tests, each with a planted violation); N31b-02 (PO-FOUNDRY-01 to -04: derivations, bounds, discharge ledger); N31b-03 (the `xtask` firewall of INV-FOUNDRY-07 and the feature layout of ADR-FOUNDRY-12); N31b-44 (the auditor side of INV-FOUNDRY-01). Individually also in the mechanism beads: INV-01, -03, -04, -05, -07 in BD-408; INV-02 and PO-01, PO-02 in BD-400; INV-03 in BD-402; INV-04 and PO-03 in BD-404; INV-06 in BD-400, BD-402, BD-408, BD-423; PO-04 in BD-438 and N31b-20 |
| §31.18 | Known answers | N31b-04 (one derivation file per row, proposed tolerances for the two `to set` rows and controls for the four `none stated` rows, exact recomputation test); N31b-05 (the battery: KA-737 to KA-748 and KA-754 wired into the conformance registry at tiers G0 to G5 with caught controls); Moonshot rows KA-749 in N31b-10 and N31b-20, KA-750 to KA-753 in N31b-11 and N31b-22; each row also in its mechanism bead (BD-400, BD-402, BD-403, BD-404, BD-406, BD-407, BD-408; KA-737 and KA-738 in BD-401 of U33b). Registry `N31-KA`: U31a |
| §31.19 | Pilots, gates and kill gates | Pilots: N31b-06 (structure loop, stage 1), N31b-07 (structure loop, stage 2), N31b-08 (tolerance allocation), N31b-09 (build packet), N31b-10 (firmware export), N31b-11 (lifecycle). Gate rows: N31b-12 (all fifteen gates and seven kill gates in the gate registry, with the register's milestone rulings). Gate measurements: table (b). Kill-gate decisions: N31b-15 (KILL-TOPOPT), N31b-16 (KILL-LOADS), N31b-17 (KILL-FAB), N31b-18 (KILL-TOLERANCE), N31b-19 (KILL-PACKET), N31b-21 (KILL-FIRMWARE), N31b-23 (KILL-LIFECYCLE). Pipeline demonstration: N31b-14 (NS-20) |
| §31.20 | Open questions | N31b-24 (OPEN-FOUNDRY-1, experiment and decision), N31b-25 (OPEN-FOUNDRY-2, owner decision), N31b-26 (OPEN-FOUNDRY-3), N31b-27 (OPEN-FOUNDRY-4), N31b-28 (OPEN-FOUNDRY-5), N31b-29 (OPEN-FOUNDRY-6, experiment and decision); each also in the Considerations of the bead it concerns |
| §31.21 | Packet review as an imported record (late) | N31b-30 (hidden briefs: twelve, sealed, digest committed before the freeze, used once, retired); N31b-31 (unattended production; typed refusal naming the binding constraint; the ceiling "no time claim"); N31b-32 (mechanical pre-review M1 to M6 in `foundry.check <packet>`; the bracket counterexample; the three known answers); N31b-33 (review record `frob.review/1`, import, manifest view, wrong-digest refusal, the recorded 11 of 12 count, the wording rule, the release-rubric hook); N31b-34 (GATE-FOUNDRY-13 measured on the real briefs); N31b-44 (auditor check used by item M1) |
| §31.22 | Referee re-extraction of load cases that enter a build packet (late) | N31b-35 (rule 7: replay window, the referee's own maximum, three labels, neighbour re-search, strict-profile adjudication, the record, the cost, what `foundry.loads` returns); N31b-36 (spectra checked on a keyed sample of windows and rescaled; the packet-side refusal; exploratory runs); N31b-37 (GATE-FOUNDRY-14 with the planted stiffness spike); summary in BD-400 and BD-408 |
| §31.23 | Member sizing as a validity check in screening (late) | N31b-38 (the closed-form sizing kernel delivered at M3; the tube known answer); N31b-39 (the preflight stage-6 check under the bounded statics load set; finding `under-built`; the trait inversion; 50 ms); N31b-40 (the check under measured peak loads; label `structure-unchecked`; promotion refused with a typed repair); N31b-41 (the fill `{shell: sized}`; the template-default switch); N31b-42 (the corpus comparison: 10 quadruped and 5 mobile-manipulator campaigns; the decision rule); N31b-43 (GATE-FOUNDRY-15) |

Seed beads and the subsections of U31a they are written from: BD-400 (§31.6), BD-402 (§31.8), BD-403 (§31.7),
BD-404 (§31.9), BD-405 (§31.10), BD-406 (§31.11), BD-407 and BD-408 (§31.12), BD-438 (§31.13), BD-423 (§31.14);
verbs of §31.15 and refusals of §31.16 in each seed bead and in N31-REF; decisions ADR-FOUNDRY-01 to -12 of
§31.3 in the Background of the beads they govern; the sibling facts of §31.4 in each seed bead; the upstream
work items FD-FSIM-26 to -30 with interim posture and switch-over rule in BD-403, BD-405 and BD-407.

## (b) Gates, kill gates and who measures or decides them

| Gate | Beads of this unit | Note |
|---|---|---|
| GATE-FOUNDRY-01 | N31b-05 (battery row); N31b-12 (registry row) | Measured by BD-401 (U33b), which owns the beam rung |
| GATE-FOUNDRY-02 | BD-400; N31b-02 (the superposition clause, PO-FOUNDRY-01); N31b-05 | |
| GATE-FOUNDRY-03 | BD-402; N31b-05 | |
| GATE-FOUNDRY-04 | BD-403; N31b-24 (the meshing clause, blocked by OPEN-FOUNDRY-1); N31b-05 | |
| GATE-FOUNDRY-05 | N31b-13 (measurement); BD-403 (mechanism); N31b-15 (reads it); N31b-27 (fatigue leg for printed fixtures) | Design comparison; six-part form registered first |
| GATE-FOUNDRY-06 | BD-404; N31b-02 (PO-FOUNDRY-03); N31b-05 | |
| GATE-FOUNDRY-07 | BD-405; N31b-17 (reads it) | |
| GATE-FOUNDRY-08 | BD-406; N31b-18 (reads it); N31b-05 | |
| GATE-FOUNDRY-09 | BD-407; N31b-19 (reads it); N31b-05 | |
| GATE-FOUNDRY-10 | BD-408; N31b-44 (auditor clause); N31b-14 (pipeline); N31b-19 (reads it); N31b-05 | |
| GATE-FOUNDRY-11 | N31b-20 (measurement); BD-438; N31b-21 (reads it); N31b-29 (budget) | M8 |
| GATE-FOUNDRY-12 | N31b-22 (measurement); BD-423; N31b-23 (reads it); N31b-28 (data) | M6 per the register |
| GATE-FOUNDRY-13 | N31b-34 (measurement); N31b-30 to N31b-33 (parts) | M5; listed in the register's index of late gates |
| GATE-FOUNDRY-14 | N31b-37 (measurement); N31b-35, N31b-36 (parts) | M5; late gate |
| GATE-FOUNDRY-15 | N31b-43 (measurement); N31b-38 to N31b-42 (parts) | M3; late gate |
| KILL-TOPOPT | N31b-15 (decision); N31b-06, N31b-07 (first readings); BD-403 | M5 |
| KILL-LOADS | N31b-16 (decision); BD-400 | M5 per the register (chapter: M4) |
| KILL-FAB | N31b-17 (decision); BD-405 | M5 |
| KILL-TOLERANCE | N31b-18 (decision); N31b-08 (first reading); BD-406 | M5 |
| KILL-PACKET | N31b-19 (decision); N31b-09 (first reading); BD-408 | M5 |
| KILL-FIRMWARE | N31b-21 (decision); N31b-10 (first reading); BD-438 | M8 |
| KILL-LIFECYCLE | N31b-23 (decision); N31b-11 (first reading); BD-423 | M6 per the register (chapter: M8) |

Gates this unit expects another unit to carry: GATE-FOUNDRY-01's measurement (BD-401, U33b); INT-FSIM-15 (bead
N33b-14, U33b); KILL-SINEW-AGE, named in BD-423's seed condition, is owned and decided by the actuator chapter's
unit and is a consumer row here (BD-423, N31b-12, N31b-23, N31b-28 cite it).

Seed done-conditions of the form "KILL-x decided": the decision bead depends on the measuring seed bead, so the
seed bead's acceptance hands its receipt to the decision bead (BD-400 → N31b-16; BD-403 → N31b-13 and N31b-15;
BD-405 → N31b-17; BD-406 → N31b-18; BD-408 → N31b-19 and, for "NS-20 demonstrated", N31b-14).

## (c) Counts

- Known answers: 18 registry rows carried (KA-737 to KA-754), each with a derivation file in N31b-04 and its
  values stated inline in the beads that test it; 13 wired by the M5 battery (N31b-05), 5 by the Moonshot gate
  beads. Plus 5 late rows without a registry number (member sizing, damage against load bias, referee cost,
  unit error, cross-export tube), entered in N31b-04 with a request for numbers. Construction-level rows
  KA-168 to KA-171 are cited where used.
- Counterexamples: 9 catalogue entries carried (CX-336 to CX-344) in N31-CX, with numeric cases supplied for
  the two the catalogue prints without numbers (CX-337, CX-339); plus 6 late cases without a catalogue number.
- Refusal codes: 14 carried in N31-REF (163, 164, 165, 166, 167, 169, 260, 261, 262, 818, 824, 827, 831, 860),
  all exit code 3; plus 3 refusals the chapter states without a code.
- Invariants 7 of 7; proof obligations 4 of 4; architecture decisions 12 of 12 (in Backgrounds); open
  questions 6 of 6 (a bead each); pilots 5 of 5 (6 beads: the structure-loop pilot has two stages); gates 15
  of 15; kill gates 7 of 7; verbs 7 of 7 (six in this unit's beads, `foundry.size` in BD-401).

## (d) Needs from other slices (no seed key could be named)

- The airlock's `structure` feature group and its gate INT-FSIM-15 (bead N33b-14 of U33b; not a key this unit
  may put in `deps`).
- `N31-KA`, the known-answer registry bead (U31a).
- Mechanism beads for §31.13 (firmware export) and §31.14 (lifecycle compression), if U31a writes them.
- A fidelity-ledger stratum for the quantity "port-wrench projection", frozen before re-extraction (§16.8,
  §29.11).
- A device-side, event-level capture of one time-coherent peak sample per checked link per episode (§29.5),
  needed by the post-rollout sizing check at M3.
- A test-only contact-stiffness override on a fast tier (§18), needed to plant the artifact of
  GATE-FOUNDRY-14.
- Re-running a campaign with contact stiffness halved and doubled and under the alternative contact law
  (§18), needed by load extraction's rule 3.
- An architecture decision admitting a sixth standing trait inversion (§34.3, rule L-10): the sizing trait
  defined in `frob-analyze`, implemented in `frob-foundry`.
- Refusal codes from the registry owner for: `review-record-packet-mismatch`; a packet request holding a
  provisional load case (interim FROB-REF-820); `under-built-promotion`; a sized fill no stock wall satisfies
  (interim FROB-REF-105); the four hidden-brief protocol refusals.
- A registry revision admitting the record kind `packet-manifest` among the proof kinds (§12.4).
- A format-registry row for the review sidecar `frob.review/1` (§34.9) and the release rubric's hook for the
  review count (§47).
- An author for the twelve hidden briefs, and independent reviewers (owner).
- The KILL-SINEW-AGE decision and the battery-ageing pilot (§15.14).
- Rates of rare overload events from adversarial search (§26) for lifecycle compression.
- Registration of the Foundry verbs' packet mode and flags in the verb registry (BD-409 covers the M5 verbs;
  the M3 use of sizing inside `design.check` needs no new verb).

## (e) Contradictions, gaps and what was done

1. **Milestones.** The chapter tags GATE-FOUNDRY-01 to -03 and KILL-LOADS at M4 and GATE-FOUNDRY-12 and
   KILL-LIFECYCLE at M8; the gate register sets M5 (recordable at M4) and M6. The register governs; all beads
   follow it and N31b-12 records the rulings. The verb registry dates `foundry.loads` and `foundry.size` at M4
   and `foundry.lifecycle` at M8, while their seed beads are at M5 and M6; beads follow the seeds and say so.
2. **GATE-FOUNDRY-15 is due at M3, but the beam rung's seed bead (BD-401, U33b) is at M5 and depends on
   BD-400 (M5).** N31b-38 delivers the closed-form kernel at M3 in the modules BD-401 names. The lead should
   add N31b-38 to BD-401's dependencies so that BD-401 extends it instead of duplicating it.
3. **Layering.** §31.23 says `frob-analyze` (layer 4) calls `frob-foundry` (layer 5); a crate may depend only
   on lower crates. N31b-39 uses a trait defined in `frob-analyze` and implemented in `frob-foundry` (the
   workspace's inversion rule), which is a sixth inversion where the workspace lists five: an ADR is needed.
   The same constraint shapes `{shell: sized}` (N31b-41): neither `frob-design` nor `frob-model` can call the
   sizer, so resolution runs in the verb layer.
4. **The verb table's refusal column** (§31.15) prints an older numbering (160 to 162, 861 to 864) that the
   registry assigns to other chapters. The chapter's refusal table (§31.16) is followed; N31-REF gives the
   corrected verb-to-code map.
5. **Three flag codes.** FROB-REF-860 (§31, packets), FROB-REF-816 (§37, artifacts in general) and
   FROB-REF-833 (§12, bundles in the auditor) overlap. Packets report 860; N31b-44 maps each code to its
   artifact kind; folding them is left to the registry owner.
6. **The auditor reads only certificate bundles, yet the packet gate requires it to reject a mutated packet
   flag.** Resolved by a bundle that embeds the manifest's canonical bytes and a checker for it in `frob-cert`
   (N31b-44), consistent with the packet work package naming `frob-cert` among its crates.
7. **Refusals without codes.** §31.21 (wrong packet digest), §31.22 (provisional case in a packet request)
   and §31.23 (promotion of an under-built design) state refusals and give no code. Carried by name in
   N31-REF table 3 with an interim where an existing code fits; allocation requested.
8. **Decimation against the 10⁻⁶ cross-export identity.** A decimated STEP beside full-resolution meshes
   would break the identity among the four exports. BD-407 treats a decimated mesh as a new geometry that
   re-enters the check and the mass feedback, and the export refuses instead of decimating in place.
9. **`exploratory` in the manifest.** An exploratory run "cannot enter a build packet", yet the manifest has
   an `exploratory` field and `qualification` may be null. Reading (BD-408): `foundry.export --exploratory`
   writes a labelled exploratory artifact that readers and the pre-review do not treat as a build packet; a
   non-exploratory packet rejects exploratory inputs.
10. **Pilot threshold.** The pilot row uses a 5% saving; construction C-35's pilot line and the gate use 7%.
    Pilot beads report against both; funding follows the chapter's row; the gate applies 7%.
11. **GATE-FOUNDRY-06 asks for the fixed point within 10⁻⁶** on the planted-gain loop; plain iteration at gain
    0.15 reaches that at iteration seven, beyond the five-iteration budget. BD-404 notes that the Aitken
    proposal (exact for the linear fixture) meets it inside the budget and tests both facts.
12. **KILL-PACKET's fallback** ("parts list, mass properties and STL only; STEP and harness deferred") is read
    strictly in N31b-19: the reduced packet holds the manifest and those three; other evidence is cited by
    identity. A safety-property failure (flags, atomicity, hashes) blocks every kind of packet.
13. **`provisional: referee-unavailable`.** The text refuses any packet request with a provisional case, and
    also says the strict CPU profile adjudicates when the referee is unavailable, with "no packet" only when
    both are unavailable. Reading (N31b-35, N31b-36): a case adjudicated by the strict profile is admissible
    with its label printed; an unadjudicated one is not.
14. **"Peak port wrenches of T1 telemetry."** T1 is event-level and holds no traces. N31b-40 requires an
    event that captures one time-coherent sample per link per episode at the running maximum of a registered
    scalar; this is a requirement on Chronicle listed in (d).
15. **Load factor for manipulator ports.** "The robot's weight times … 1.5 for manipulator ports with the
    rated payload added" would put the base's weight on the gripper. N31b-39 reads the reference load as rated
    payload plus the arm distal to the link, times 1.5, as a DESIGN interpretation recorded for the owner.
16. **GATE-FOUNDRY-15 names two claims** (Known answer; Design comparison). By the register's rule it takes
    the claim of its governing part; N31b-12 and N31b-43 read the known-answer clauses as governing and the
    corpus share as a recorded figure deciding a template default.
17. **OPEN-FOUNDRY-5 "blocks GATE-FOUNDRY-12".** The gate is a Fidelity claim about the method and can be
    measured on synthetic cards; what the missing data blocks is any output about a real design without the
    `assumed` label. N31b-28 records this reading for the owner (the Sinew lead) to confirm or override.
18. **`model-conditional` load cases.** The trust-ladder chapter says a `model-conditional` result cannot
    authorize a build packet; §31.6 makes the envelope over contact variants the load set and has the margin
    inherit the label. Reading (BD-400, BD-408): a labelled case may enter a packet with the label printed
    and supports nothing above Estimated.
19. **Review of a refusal outcome.** "A correct refusal … is reviewed as one", but `frob.review/1` addresses a
    packet's manifest digest. N31b-33 records such a review against the run record's digest as a DESIGN
    extension, listed separately and not counted among packets.
20. **Feature names.** ADR-FOUNDRY-12 names features `firmware-export` and `lifecycle`; the workspace rule
    requires a `moonshot-` prefix for Cargo features that gate Moonshots. N31b-03 uses the prefixed Cargo names
    and keeps the chapter's names at the user surface.
21. **§31.10 says `fs-geocon` measures two of the features; its table attributes three rows to it.** BD-405
    follows the table. Manufacturability [F] has no pilot row of its own; the build-packet pilot (N31b-09)
    carries one part through the fab report and serves as its pilot.
22. **NS-20** speaks of "a bill of materials against a pinned catalogue" and of 18% lighter; no catalogue is
    chosen (OPEN-FOUNDRY-2) and 18% is an illustration. N31b-14 asserts card references without prices under
    the default and records the measured saving without a threshold.
23. **Schema name.** The format registry names the packet schema `frob.build/1`; a note in the type-sketch
    appendix mentions `frob.fbuild/1`. The registry's name is used everywhere.
24. **Hidden-brief author.** "Someone who has not seen the development fixtures" is not a role the program
    has; N31b-30 defaults to a fresh-context session jailed away from the fixtures and lets the owner name a
    person.
25. **The window-split known answer** (7.816 × 10⁻⁶) is reproduced only if the fifth point closes the first
    window and opens the second; N31b-04 and N31-CX state the convention.
