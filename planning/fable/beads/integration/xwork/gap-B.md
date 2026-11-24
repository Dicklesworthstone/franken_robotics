# Gap packet gap-B

17 reported gaps. Several groups often reported the same gap: treat those as one.

## Gap 26 (reported by G19)
- plan section: §32.9
- what is missing: a hook in run admission (sim.run, eval.run, train.start, opt.start, study.submit) through which a subsystem's standing refusal can refuse a run; N19b-32 needs it for FROB-REF-309, BD-228 for an unaccepted pack, and no Tether bead names it
- needed by: N19b-32, BD-228

## Gap 34 (reported by G20)
- plan section: §32.6
- what is missing: the sense cluster has eight verbs (sense.visibility appears only in the verb registry appendix), one more than an MCP task cluster allows; the surface owner must assign the split that N20b-17 proposes
- needed by: N20b-17

## Gap 43 (reported by G23)
- plan section: §32.4
- what is missing: no bead assigns Appendix O refusal codes to refusals that late subsections state in words: Cortex has three (candidates 558, 559, 570 in N23-REF) and §24.14 one (randomizing a numerical parameter)
- needed by: N23-REF, N23b-29, N23b-31, N23b-35, N23b-03

## Gap 59 (reported by G25cde)
- plan section: §32.6
- what is missing: the M4 verb-set bead BD-376 does not register the static-core form of mission.transport (it defers the verb to a later verb set), while §25.11 delivers that form at M4 (N25c-02) and VERBS-mission records `static core at M4`
- needed by: N25c-02, N25e-02, N25e-05

## Gap 60 (reported by G25cde)
- plan section: §32.6 and Appendix J
- what is missing: registry rows for three analysis verbs that the late Augur rules introduce (analyze.sweep-monotone, analyze.constraint-graph, analyze.regressor-identity); VERBS-analyze has the 26 rows of A-01 to A-26 only
- needed by: N25e-08, N25e-27, N25e-19

## Gap 65 (reported by G27)
- plan section: §32.23
- what is missing: capture of a session's verb calls (resolved request, idempotency key, result digest) in frob-verbs exists only inside seed BD-313, which waits on the whole M3 surface (BD-298, BD-299, BD-312). BD-284 (a lower-numbered seed) builds its transcript on that capture and cannot depend on BD-313. Needed: a small frob-verbs bead split out of BD-313 that BD-284, N32b-38 and BD-313 depend on
- needed by: BD-284

## Gap 75 (reported by G29)
- plan section: Appendix J
- what is missing: no verb or option withdraws a claim, although the repair of FROB-REF-813 is `withdraw the claim first`; N29b-13 proposes `store.collect --withdraw-claim`, which the store.collect row of VERBS-store does not carry
- needed by: N29b-13, N37-19

## Gap 81 (reported by G30)
- plan section: §32.13
- what is missing: no bead defines the session capability scope ground:retire (BD-258 does not name it)
- needed by: N30a-43, N30b-13

## Gap 82 (reported by G30)
- plan section: §32.12 and Appendix J.5
- what is missing: a task-cluster assignment for ground_excite and ground_kit, which are outside the reality cluster of seven tools (N30b-13 proposes reality-plan)
- needed by: N30b-13

## Gap 97 (reported by G32)
- plan section: Appendix O, §32.4
- what is missing: no bead owns the refusal catalogue as a whole (allocation of new numbers, uniqueness across chapters, one exit code per code). N32-REF registers three provisional numbers (836 packet over the byte cap, 837 frames from a chunk neither stored nor pinned, 838 resource read over the 4 MiB cap), records four pairs of codes for one event (804 and 112, 806 and 896, 810 and 878, 704 and 761), and §32.4 names no exit for the families 71x and 74x to 79x, which N32a-03 requires each registry row to state
- needed by: N32-REF, N32a-03, N32a-25, BD-299, N32b-41

## Gap 104 (reported by G33)
- plan section: §32.4
- what is missing: the exit-code table names the 70x, 72x and 73x ranges and is silent on 79x; N33-REF assigns exit 5 to FROB-REF-790, -791 and -792 and needs the table's owner to add the range
- needed by: N33-REF, N33b-21, N33c-32

## Gap 106 (reported by G33)
- plan section: Appendix J / §32.6
- what is missing: the entry schema of registries/verbs.toml (NJL-01, N32a-02) has no field for the airlock feature groups a verb needs; N33c-32 adds needs_groups and the schema owner must admit it
- needed by: N33c-32, N33a-11

## Gap 112 (reported by G35)
- plan section: Appendix J
- what is missing: registry rows lack what the numerics beads add: spark.check has no --decisions, --invariance, --conditions or --sweeps (VERBS-spark); eval.compare has no --pairing or pair_correlation (VERBS-eval)
- needed by: BD-177, BD-181, N35-14, N35-07, N35-08

## Gap 117 (reported by G39)
- plan section: §32
- what is missing: a backend dispatch in the verb registry through which a layer over an incumbent engine could sit behind the same verbs, schemas and checker: the fallback branch of KILL-TENFOLD-TIE (§39.10); no bead carries it
- needed by: N39b-07

## Gap 126 (reported by G44)
- plan section: §32.4, §36.14
- what is missing: no bead names the option or environment variable by which frob and frobd read registries/ from another root. The refusal and verb registries are embedded at build time; the envelope registry must be read at run time. N36-20, N21c-35 and N44-02 all test with a scratch registry
- needed by: N44-02

## Gap 130 (reported by GEG)
- plan section: Appendix J
- what is missing: the registry row of design.import (VERBS-design) lists URDF and MJCF inputs only; the new bead NEG-32 adds the format `fsim-catalog` with `--builder` (sibling catalog builders lowered to a design term), so the row and its schema snapshot need that value
- needed by: NEG-32

## Gap 134 (reported by GJL)
- plan section: Appendix J.3, J.4
- what is missing: verb names used by beads of another unit have no registry row: analyze.sweep-monotone, analyze.constraint-graph, analyze.regressor-identity. The registry is closed at 243 rows with count tests in NJL-03; the bead that introduces a verb adds its row and the counts, or the name is withdrawn (also reported by G25cde)
- needed by: N25e-08, N25e-27, N25e-19

