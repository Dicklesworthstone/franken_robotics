# Gap packet gap-A

14 reported gaps. Several groups often reported the same gap: treat those as one.

## Gap 4 (reported by G12)
- plan section: §33.4
- what is missing: no bead admits fs-checker as a re-check oracle: N33b-09 forbids any cargo edge to it and N25a-32 only records its pin facts; an out-of-process admission record is what the second-checker experiment would need
- needed by: N12-29

## Gap 6 (reported by G14)
- plan section: §33.3
- what is missing: no bead admits `fs-mbd::tri_wheel_cluster` as an oracle (first use at M3); until then the tri-wheel-stair row is `blocked: admission`
- needed by: N14b-04

## Gap 7 (reported by G14)
- plan section: §33.3
- what is missing: the airlock's mesh exchange and signed-distance queries are first admitted at M2 (N33a-30), but mesh Regions and mesh shells are M1 work; no M1 bead delivers them
- needed by: N14a-14, N14a-03

## Gap 36 (reported by G20)
- plan section: §33
- what is missing: no bead admits FrankenSim's tracer feature group through the airlock for offline hero renders; N20a-28 depends on the airlock skeleton N33a-11 and would have to do the admission itself
- needed by: N20a-28

## Gap 40 (reported by G22)
- plan section: Appendix F
- what is missing: an upstream work-item identifier for the input-affine (bilinear) EDMD fit in fs-surrogate of §22.20; N22c-22 carries the work under a local key and no FD-FSIM row exists
- needed by: N22c-22

## Gap 54 (reported by G25cde)
- plan section: §33.9
- what is missing: the implicit-function module of fs-adjoint is exposed only by the airlock group `structure` at M5 for frob-foundry (N33b-14); frob-synth needs it under `optimize` at M4
- needed by: BD-362, N25d-13

## Gap 68 (reported by G28)
- plan section: §33.3
- what is missing: the first-use admission record and airlock entry of fs-tropical (max_cycle_mean, critical_circuit): N33a-18 calls it a thin entry owned by the bead that admits it, and no bead does. BD-332 uses its local exact routine until then
- needed by: BD-332

## Gap 86 (reported by G31)
- plan section: §33.9
- what is missing: `fs-mesh` (tetrahedral meshing with its audits) is on no airlock group's crate list or consumer list; Foundry's extraction and independent check need it
- needed by: N31a-17, N31a-19, BD-402, BD-403, N31b-24

## Gap 87 (reported by G31)
- plan section: §33.9
- what is missing: `fs-mbd-material` is admitted only in the airlock group `referee` (N33a-15), whose consumer list (N33a-11, N33b-09) omits `frob-foundry`; Foundry computes mass properties of checked and exported geometry with it
- needed by: N31a-23, N31a-40, BD-404, BD-407

## Gap 88 (reported by G31)
- plan section: §33.9
- what is missing: `fs-uq::product_epistemic` is in the airlock group `stats` (N33b-13), whose consumer list omits `frob-foundry`; the worst-case boxes of tolerance allocation need it
- needed by: N31a-33, BD-406

## Gap 89 (reported by G31)
- plan section: §33.9, Appendix F
- what is missing: no upstream work item and no airlock probe for an external differentiable filter stage in the sibling optimizer's filter stack; without it the overhang filter runs as a post-check only
- needed by: N31a-29

## Gap 107 (reported by G33)
- plan section: §33.5 / §34
- what is missing: franken-stack.lock gains fields and tables from several units (required_at, optional_at, uses; [freeze]; [json_library]; [auditor]; worker_rev) and no one bead owned the list; N33a-03 now lists them, and the workspace beads N34-04, N34-13 and N34-25 must agree with that list
- needed by: N33a-03, N33b-01, N33b-03

## Gap 128 (reported by GEG)
- plan section: §33.3
- what is missing: no bead admits fs-mbd::tri_wheel_cluster as an oracle (first use at M3); the tri-wheel-stair oracle row stays `blocked: admission` (the same gap G14 reports for N14b-04)
- needed by: ZOO-tri-wheel-stair

## Gap 132 (reported by GF)
- plan section: Appendix F.1, §33.18
- what is missing: BD-037 tests for exactly 53 register rows; with the six items added by description (NF-07 to NF-12, registered by N33c-47) the count is 59, and 60 or 61 once N22c-22 and the conditional NF-13 have rows; BD-037's count should follow the registry's list
- needed by: NF-07, NF-08, NF-09, NF-10, NF-11, NF-12, NF-13

