# Coverage: drafting unit U37 (§37 Safety, security, privacy)

43 bead files: 7 seed beads (BD-258, BD-303, BD-304, BD-305, BD-306, BD-307, BD-308) and 36 new beads
(N37-01 to N37-31, N37-KA, N37-CX, N37-REF, N37x-01, N37p1-01). N37-31 was added in the integration pass;
N37p1-01 was split from BD-303 in polishing round 1 (the M2 exporters' carriers, the generated banner, the
later-kind contract rows and the M2 record of GATE-SAFE-01).

## (a) Subsections and the beads that carry them

| Subsection | Title | Beads |
|---|---|---|
| §37 (preamble) | Where the mechanisms live; none on by request; no off switch below an owner-held configuration | N37-02 (owner-held configuration, no-off-switch sweep), N37-01 |
| §37.1 | The threat model | N37-01 (five assets, in-scope and out-of-scope threats, traced to mechanisms, gates, invariants, refusals, controls) |
| §37.2 | Decisions | N37-01 (all five in one table); ADR-SAFE-01 → N37-23, BD-303; ADR-SAFE-02 → N37-16; ADR-SAFE-03 → N37-17, BD-305; ADR-SAFE-04 → N37-11, BD-258; ADR-SAFE-05 → N37-04, BD-304 |
| §37.3 | Agents supply data, not code | N37-03 (expressions are data; patches; kernels; what bounds an MCP client; code payloads), N37-07 and N37-08 (weights are hostile data), N37-05, N37-06, BD-307 (imports are hostile input: limits, fuzz targets, loss report), N37-04, BD-304, N37-10 (text from imports is data), N37-09 (sandboxed custom reward terms, Frontier, not admitted) |
| §37.4 | Resource governance | BD-258 (the three acts; scopes; tokens), N37-13 (sizes before device work, ceilings, budgets in every unit, MCP rate and listener bounds, trace bytes, admission guard, preflight), N37-14 (the remote worker's manifest), N37-15 (script), N37-02 (configuration keys) |
| §37.5 | Capabilities, and why discovery cannot read confirmation worlds | BD-258 (the capability layer), N37-16 (three independent layers, audit scope, commit before draw, CX-384); built by BD-261 of unit U24b |
| §37.6 | Private places stay private | N37-17 (inheritance is computed), BD-305 (private root, views, scopes and flag, seeds not voxels), N37-11 and N37-12 (path jail), N37-31 (admission of the handle-relative filesystem library the jail needs), N37-19 (purge), N37-20 (weights inherit), N37-21 (script), N37-14 (what crosses to a worker) |
| §37.7 | Licences | N37-17 (the join), N37-18 (classes, export refusal, notices, dependencies as assets), BD-305 (gate record), N37-21 (script) |
| §37.8 | Supply chain | N37-22 (GATE-SAFE-06 at M2: one runtime package, lockfiles, cache flip, std-only auditor), BD-306 (exact pins, `--locked`, admission records, verified kernel loads, kernel provenance, vendored viewer, no local patches), N37-18 (third-party notices) |
| §37.9 | No safety certification, and the no-authorization flags | N37-23 (schemas, locator registry, reader refusal, PO-SAFE-03, M1), N37p1-01 (M2 exporters' carriers, generated banner, later-kind contract rows, M2 record), BD-303 (M3 full surface, report banner, table of where the flags sit), N37-24 (script, recurring gate) |
| §37.10 | The claim product at the boundary | N37-25 (M1 gate: serializer, random sets, API scan), BD-308 (full surface: propagation, claims and non-claims, bundles), N37-26 (citation above the product, FROB-REF-819), N37-27 (script) |
| §37.11 | The labeled nondeterministic training profile | N37-28 (labels, five paths, weights as data, bans hold), N37-29 (script); the profile itself is BD-269 of unit U35 |
| §37.12 | Refusals | N37-REF (eight rows with triggers and repairs; 840 and the retired 730); raised by N37-03, N37-05, N37-07, N37-11, N37-18, N37-23, N37-26, N37-28, BD-305 |
| §37.13 | Invariants and proof obligations | See the table below |
| §37.14 | Known answers and gates | N37-KA, N37-CX; gates in table (b) |
| §37.15 | Open questions | N37-08 (OPEN-SAFE-1), N37-20 (OPEN-SAFE-2), N37-09 (OPEN-SAFE-3), N37-30 (OPEN-SAFE-4) |

Invariants, proof obligations, decisions and open questions:

| Identifier | Statement (short) | Carried by (test) |
|---|---|---|
| INV-SAFE-01 | No verb accepts a kernel, a program or an expression outside the closed language | N37-03 (leaf lint; property over random arguments) |
| INV-SAFE-02 | An imported string reaches only fields marked untrusted | N37-04 (type; compile-fail), BD-304 and N37-10 (sweep) |
| INV-SAFE-03 | No discovery session can obtain any per-world datum of a confirmation run | N37-16 (random verb sequences with the sealed-value dictionary) |
| INV-SAFE-04 | Privacy mark and licence set are joins over lineage; neither decreases | N37-17 (random derivation graphs), BD-305 (kind sweep) |
| INV-SAFE-05 | Every exported artifact carries the three flags, each false | N37-23 (property over exports), N37p1-01, BD-303, N37-24 |
| INV-SAFE-06 | The evidence in an envelope equals the claim product of its inputs | N37-25 (random sets), BD-308 (taint lint) |
| INV-SAFE-07 | No D4 result is consumed by the five consumers | N37-28 (property over random D4 products), N37-29 |
| INV-SAFE-08 | One `asupersync` package per workspace; no kernel loads without a matching content hash | N37-22, BD-306 (verified-kernel type) |
| PO-SAFE-01 | The join of marks is monotone along lineage | N37-17 (property test; argument for `CONTRACT.md`) |
| PO-SAFE-02 | Discovery and confirmation address namespaces are disjoint | N37-16 (exact integer comparison; 10⁶ and 10⁵ draws) |
| PO-SAFE-03 | The authorization type has one inhabitant | N37-23 (compile-time assertion; golden schema fragment; source scan) |
| ADR-SAFE-01 to -05 | The five decisions with reason and rejected alternative | N37-01 table; each in the Background of the bead it governs (row §37.2 above) |
| OPEN-SAFE-1 | Weight layout and architecture allowlist at M2 | N37-08 (decision), default applied in N37-07 |
| OPEN-SAFE-2 | Whether weights inherit mark and licence set | N37-20 (decision), default applied in N37-17, BD-305 |
| OPEN-SAFE-3 | Sandboxed custom reward terms | N37-09 (decision and the pilot owed first), default applied in N37-03 |
| OPEN-SAFE-4 | Retention periods | N37-30 (decision), keys in N37-02 |
| RISK-11, RISK-12, RISK-34, RISK-49 | Licences; private scan leak; qualification gamed; injected text | N37-18 and BD-305; N37-17, BD-305, N37-19; N37-16; N37-04 and BD-304 |

## (b) Gates and the beads that measure or decide them

The chapter defines ten gates and no kill gate and no admission gate.

| Gate | Earliest (§40) | Measured by | Notes |
|---|---|---|---|
| GATE-SAFE-01 | M1, then every exit | N37-23 (M1 mechanism and record), N37p1-01 (M2 rows and record), N37-24 (script, recurring), BD-303 (M3 full surface) | Also touched at M0 and M1 by beads of U12 and U00 (flag sweep, claim-rules battery) |
| GATE-SAFE-02 | M3 | BD-304 (mechanism and scanner), N37-10 (script and record) | Partial M1 profile in N37-04 records no gate |
| GATE-SAFE-03 | M3 | N37-16 (layer independence, PO-SAFE-02, INV-SAFE-03) | Firewall and enumerated paths are BD-261 and its script, unit U24b |
| GATE-SAFE-04 | M3 | BD-305 (record), N37-19 (purge clause), N37-21 (script) | |
| GATE-SAFE-05 | M3 | N37-18 (battery), BD-305 (record), N37-21 (script) | |
| GATE-SAFE-06 | M2 | N37-22 (M2 record), BD-306 (M3 rules and re-record) | |
| GATE-SAFE-07 | M2 | N37-06 (M2 record), BD-307 (M3 complete surface) | Limits and targets built in N37-05, N37-07 |
| GATE-SAFE-08 | M1 | N37-25 (M1 record), BD-308 (M3 full surface), N37-27 (script) | Also named by beads of U00 and U10a at M0 and M1 |
| GATE-SAFE-09 | M3 | N37-28 (five paths), N37-29 (script and record) | Seed BD-269 of unit U35 also names it |
| GATE-SAFE-10 | M1 | N37-11 (M1 mechanism), N37-12 (script, M1 record), BD-258 (M3 full surface, per-session roots) | |

## (c) Counts

- Known answers: 9 appendix rows carried (KA-827 to KA-835), all in N37-KA; 14 bead-level answers listed
  there under local identifiers (SAFE-L01 to SAFE-L14).
- Counterexamples: 1 appendix entry carried (CX-384), in N37-CX; 10 unnumbered smallest cases of the chapter
  recorded there under local identifiers (SAFE-X01 to SAFE-X10), each with a control.
- Refusal codes: 8 carried in N37-REF (FROB-REF-731, -733, -734, -816, -819, -825, -828, -832), plus the
  chapter's ninth table row (FROB-REF-840, owned by §24.13) and the retired FROB-REF-730.
- Subsections: 15 (§37.1 to §37.15) and the chapter preamble. Gates: 10. Invariants: 8. Proof obligations: 3.
  Decisions: 5. Open questions: 4.

## (d) Needs from other slices (no key known, or a key in another unit's new-key space)

| Need | From | Used by |
|---|---|---|
| §32.11: the daemon's state root and configuration digest | U32a | N37-02 |
| §32.12.3, §32.12.7, §32.12.8, §32.12.9: generated tool descriptions, resources, prompts, transports and the authentication provider | U32a (BD-298) | BD-304, N37-10, N37-15, BD-258 |
| §32.21: the poisoned-import arm of GATE-TETHER-13 | U32a | BD-304 |
| §34.3 rule L-5 and §33.6: an admission record and a named exception for `cap-std` in `frob-verbs` | U34, U33 | N37-11 |
| §19.3, §19.4, §19.9, §19.13: the scene provenance block, the scene import profile registry and decode targets, the asset licence manifests, real-place ingestion | U19a, U19b, U19c (N19a-05, N19a-10, N19b-02) | N37-05, N37-06, BD-307, N37-17, N37-18, BD-305 |
| §23.7, §23.8: the checkpoint manifest, parameter layout and the container bridge | U23a (N23a-12, N23a-13, N23a-15) | N37-07, N37-08, N37-20, BD-303 |
| §24.13: the campaign simulator, the address layout and cluster derivation, the firewall's path registry | U24b (BD-261, N24b-17, N24b-18, N24b-24) | N37-16 |
| §29.13, §29.16, §28.14: `derived-from` edges for every stored kind, pins and tombstones, the memo store returning products and marks | U29a, U28 | N37-17, N37-19, BD-308, N37-30 |
| §35.2, §35.3: the `fast-training` profile, the licence table and FROB-REF-561, -562, -861 | U35 (BD-269) | N37-28, N37-29 |
| §16.8, §16.14: labels, promotion and FROB-REF-820 | U16b (BD-263) | N37-26, N37-27 |
| §12.4, §12.7: the bundle's stored product, the auditor's recomputation and FROB-REF-833, -834 | U12, U25b (N12-03, N12-11) | BD-308, N37-23, N37-24 |
| §21.10, §21.17, §21.21: the kernel cache key and loader, transfer classes, FROB-REF-612 and -618 | U21a, U21c | N37-22, BD-306, N37-14 |
| §33.5, §33.6, §33.8: the stack lock, receipts, admission records, upstream work items | U33a, U33b, U33c | N37-22, BD-306, N37-18 |
| §36.10, §36.12, §36.14: the byte model and the compute-envelope classes | U36 | N37-13 |
| §38.6, §38.11, §38.13: the known-answer runner, the control registry, `registries/gates.toml` | U38a, U38b | N37-KA, N37-CX, every script |
| §28.18, §32.24, §31.12: report verbs, the replay packet, the build packet | U25e, U32a, U31b | BD-303, N37-18, N37-28 |

Handoffs the lead should wire (other units' beads that should depend on a U37 bead): the URDF and MJCF
importers (BD-065, BD-066) use the `Untrusted` type owned by N37-04; `artifact.export` (BD-097) and every
path-taking verb resolve through N37-11; the build packet (BD-408) re-runs the fifth D4 path of N37-28 and the
flags contract of BD-303; the M3 conformance bead (BD-318) consumes the corpora packaged by BD-307.

## (e) Contradictions, gaps and what was done

1. **Seed milestones against the gate register.** The seeds BD-303, BD-308 and BD-258 sit at M3 while §40
   dates GATE-SAFE-01, -08 and -10 to M1; BD-306 and BD-307 sit at M3 while GATE-SAFE-06 and -07 are dated M2
   (§43 lists them in the M1 and M2 acceptance). Seeds keep their milestone and dependencies as the charter
   requires; new beads at the register's milestone build the mechanism for the surface that exists then and
   take the first record (N37-23, N37-24, N37-25, N37-11, N37-12 at M1; N37-22, N37-06 at M2), and each seed
   closes the gate over the full surface at M3.
2. **Handle-relative path resolution against dependency rule L-5.** ADR-SAFE-04 requires opening beneath
   retained directory handles; the standard library cannot open relative to a handle, and L-5 admits only the
   standard library and suite foundations in the core. N37-11 names `cap-std` at the exact pin the MCP library
   already uses, to be admitted for `frob-verbs` by ADR, with a fail-closed interim posture. The decision,
   the admission test and the exception row are bead N37-31 (written in the integration pass), on which
   N37-11 depends.
3. **Overlapping refusal codes.** 819 against 820 and 834; 825 against 561 and 861; 733 against 310; 832
   against 303; 816 against 833 and 860. Each pair has two owners. N37-26, N37-28 and N37-REF state one
   precedence so that every request gets exactly one code (819: requirement side; 820: assertion side; 825: a
   stored D4 result offered to a consumer; 561: a D4 arm planned; 861: a training number cited).
4. **FROB-REF-730 is retired in favour of 840**; the chapter's table still shows the row. N37-REF records 730
   as retired and reserved; 840 stays with §24.
5. **Known-answer gaps.** Appendix M has no row for GATE-SAFE-07, and three rows (KA-830, KA-832, KA-835)
   state no negative control. N37-KA lists bead-level answers for GATE-SAFE-07 and the controls the measuring
   beads supply for the three rows.
6. **The refusal table has no repairs.** Ranked repairs for all eight codes are DESIGN, written in the
   raising beads and gathered in N37-REF.
7. **Numbers the plan does not give.** Import limits, text caps, expression bounds, the weight-file caps, the
   MCP rate, the disk reserve and the session cap are DESIGN defaults, labelled, held in registries or the
   owner-held configuration.
8. **Surface gaps.** Appendix J has no verb for importing policy weights (admission is attached to the
   arguments that take a policy file, N37-07); no scope is named for `scene.purge` or other admin verbs
   (`<cluster>:admin`, DESIGN, in BD-258 and N37-19); no listing of private objects is specified (an explicit
   private listing, DESIGN, in BD-305); the mesh importer's location is not stated (`frob-io::mesh` for STL
   and OBJ, DESIGN, in N37-05).
9. **Sandboxed custom reward terms are tier Frontier with no pilot named.** No mechanism bead is created
   (the default is "not admitted"); N37-09 carries the decision and defines the paper-scale pilot and
   falsifier that must exist before any mechanism bead is funded.
10. **§37.4 has no gate.** Resource governance is evidenced by the exhaustion battery of N37-13 and the
    script N37-15, whose controls are registered against the surface chapter's gates.
11. **Purge against pins and "no digest remains".** §29 refuses a deletion of a pinned object, and tombstones
    and the purge receipt necessarily list digests. N37-19 withdraws claims and releases pins inside the
    purge transaction, and reads the gate clause as "no object with a digest of the closure remains".
12. **D4 and the weakest-input rule.** "The same policy re-evaluated under D2 is accepted" is consistent with
    "weakest input wins" only if weights enter an evaluation as an identity and not as evidence; N37-28 states
    that reading explicitly. Qualification has no natural D4 input; the planted cases are a supporting result
    and a tier plan naming the profile.
13. **Gates shared with other units.** GATE-SAFE-03 is also measured by BD-261 (U24b); GATE-SAFE-09 is also
    named by seed BD-269 (U35); GATE-SAFE-01 and -08 are touched at M0 and M1 by beads of U00, U10a and U12.
    The U37 beads state what each adds and do not repeat the others' mechanisms.
14. **Fuzzing engine.** A coverage-guided engine is a third-party development dependency; N37-05 specifies an
    in-house seeded driver that meets the gate and treats the guided engine as optional, subject to admission.
15. **Parents.** M1 beads for flags and the claim product are under WP-36 (its row lists GATE-SAFE-01 and
    -08); the other M1 and M2 safety beads are under WP-58 although its row names M2 and M3, because WP-58
    owns their gates; N37-16 is under WP-54, which owns GATE-SAFE-03.
