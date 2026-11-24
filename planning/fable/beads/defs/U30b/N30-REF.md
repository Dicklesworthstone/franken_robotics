---
key: "N30-REF"
title: "Ground refusal-code registry: FROB-REF-812, -814, -873, -876, -879, -883, -886, -889, -895, -897, -899"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "ground", "frob-ground", "frob-core", "solid", "registry"]
parent: "WP-68"
deps: ["BD-005", "BD-022", "N35x-02"]
needs: []
related: ["N30b-01", "N30b-06", "N30b-29", "N30b-36", "N32bx-02", "N35x-01", "N37-18"]
gates: []
plan_refs: ["§30.19", "§30.16", "§30.12", "Appendix O.8", "§32.4", "§35.11"]
---
## Background

Every refusal in FrankenRobotics has a stable code, a message and ranked repairs. A refusal is not an error: it is
the system declining to produce a result that would mislead, together with the smallest changes that would make
the question answerable. The machine-readable registry is `registries/refusals.toml`; an unknown code is a
load-time error and codes are never reused. With `--json` a refused verb returns the standard envelope with
`ok: false` and a `refusal` object: `code`, `message`, `pointer` (the field or part at fault), `observed`,
`required`, `unit`, `affected_claims`, `retry` (`never`, `after-change` or `backoff`) and ranked `repairs`. Exit
code 3 means a refused input or question; exit code 1 is never emitted on purpose.

Ground (crate `frob-ground`, layer 5) is the subsystem that compares simulation with measured reality. Its eleven
codes sit in the `8xx` family, evidence and protocol. Each one stops a specific way in which a simulated number
could be read as a measured one: a dataset that cannot name what it rests on; a clock fitted with too little
information; a fit that returns its prior under another name; a comparison that was looked at before it was
predicted; a holdout used twice; a band outside the one cell it belongs to; a calibration that has expired; a
private scene leaking into a public kit; a scan registered below its own noise; pools that hide a failing regime.

DESIGN reason for refusals instead of warnings: a warning travels with the number and is dropped at the first
copy. Each condition below changes what a statistic means, so the number is not produced.

**A numbering correction this bead makes binding.** The chapter's verb table lists refusal numbers 840 to 850 for
the seven `ground` verbs, and one rule of the band section cites "-846, -850". Those are positions in an earlier
numbering. The registry assigns 840 to 850 to the qualification lane and to mission certificates. The chapter's
own refusal table and the registry agree with each other and govern. The map, in the order of the refusal table:
840 → 873, 841 → 876, 842 → 879, 843 → 883, 844 → 886, 845 → 889, 846 → 895, 847 → 897, 848 → 899, 849 → 812,
850 → 814. The per-verb lists below are the verb table's lists after that map.

## Scope

In: the eleven registry rows with message templates, triggers, pointer and payload conventions, retry class,
ranked repairs, exit code and the verbs each is listed under; registry tests.
Out: raising each code (the mechanism beads named in the table); the planted-violation battery through the real
verbs (N30b-37); codes Ground only consumes: FROB-REF-532 (unreachable verdict at registration), FROB-REF-552
(mixed study types), FROB-REF-102 (a missing or mixed unit, used for a unitless tolerance), FROB-REF-731 and
-733 (private scope, licence; 731 is raised at two Ground sites: by `ground.kit` as an exporting verb, and by
the handler of `ground.ingest` for a private-marked bundle admitted without the private scope and flag, exit 5,
BD-387), FROB-REF-734 (an output path of `ground.kit` outside the session's export root), FROB-REF-828 and -832
(the importer's file sniff and import-profile limits on the file arguments of `ground.ingest`), FROB-REF-816
and -819 (authorization flags, citation above the claim product),
FROB-REF-724 (verb not implemented in this build), FROB-REF-802 (schema).

## Technical approach

Rows to enter in `registries/refusals.toml`. All exit 3; all `retry = "after-change"` (one site excepted: 883 at
`predict.register` for a frozen record's own target, below); `block = "8xx"`; `defined_in = "ground"`. "The
reality guard" in the last column is `frob_ground::reality::guard` (BD-139, M1), called by the envelope
serializer, by every artifact reader and by every report renderer on any input that arrives carrying a Reality
status or a `measured` card field; its check order is flags, parse, origin, design built, cell record,
retirement, counts, so a payload is stable.

| Code | Refused | Trigger (the condition the check tests) | Payload (`observed` / `required`) | Ranked repairs | Raised by; verbs | First due |
|---|---|---|---|---|---|---|
| FROB-REF-873 | A dataset lacking licence, provenance, rights or measurement uncertainty | Manifest `frob.ground.dataset/1` without `license`, `acquired.by`, `acquired.on` or `acquired.rights`; an instrument without `sigma`, `bias_bound`, `unit`, `instant`, `clock` or `calibration`; a cell component that cannot be determined (units, controller, firmware, surface, envelope, design, task family); a recomputed dataset id that differs from the declared one; `attempts` not `"all"` or disagreeing with the operator's trial counter; a split by rows, a split whose parts overlap, or an episode assigned to no part or to two (pointer `split`); a card field marked `measured`, or a status line, that names no admitted dataset | The missing or failing field paths / the required field list | 1. Supply the missing fields. 2. Re-export the bundle so its id recomputes. 3. If rights or uncertainty cannot be established, admit nothing: the state stays `reality: no-claim` | Admission and the reality guard; `ground.ingest` | M1 (guard), M5 (admission) |
| FROB-REF-876 | Clock alignment under-determined, or a scored instant outside the bracket | Fewer independent marks than the clock model needs (affine: 2; piecewise affine with k segments: k + 1); marks that do not bracket the trial; an instant to be scored outside [first mark, last mark]; a time-based metric on a clock with no shared hardware mark; a time-based metric on an instrument whose timestamp instant is undeclared | Marks present, their times, the instant / marks needed and where | 1. Record the marks needed (one before and one after the trial; k + 1 for piecewise). 2. Score time-free metrics only (success, energy, peak values). 3. Report "delay plus offset" as one quantity | Clock fitting, scoring; `ground.ingest`, `ground.score` | M5 |
| FROB-REF-879 | A fit on a parameter set that fails identifiability | A parameter requested from `ground.identify`, or a card field update, that the identifiability analysis of the recorded experiment marks structurally unidentified (σ_min/σ_max of the scaled Jacobian below 10⁻⁶), imprecise (Cramér–Rao bound above half the prior standard deviation), aliased (correlation magnitude at least 0.95) or flat in profile; includes delay without a shared mark, Coulomb friction from one direction of motion, friction coefficient from torque-limited data, exposure delay against mount offset at one axial velocity. Also the stage-order case (DESIGN, pointer `stage`): a later identification stage requested when an earlier stage of the fixed order (clocks, actuators, contact and surface, inertia and geometry, sensors) has neither a record in the cell nor an explicit `--keep-prior <stage>` (for the clocks stage: a clock whose time-stamped traces the request needs has no fit); each stage's error aliases into the next, so a stage fitted on an unfitted predecessor is a fit on a set that is not identified | The aliased combination with its value and interval / the separating experiment; for the stage-order case the missing stage / the stage record or `--keep-prior` | 1. Accept the identified combination. 2. Run the separating experiment named. 3. Hold the parameter at its prior, recorded as prior. For the stage-order case: 1. run the missing stage first; 2. pass `--keep-prior <stage>`, which records that stage's parameters as held at their prior | Identifiability and identification; `ground.excite`, `ground.identify` | M5 |
| FROB-REF-883 | A read of sealed data with no frozen prediction, or a prediction changed after freezing | Any read path to a sealed chunk without a score token; `ground.score` on a record whose content hash differs from the hash frozen in the ledger; a `frozen_at` that does not precede the read; a posterior refitted after validation data was seen, scored under the old version; a new prediction version offered against the sealed data of the old one; a later dataset that declares unsealed (as `calibration`, or in a part missing from `sealed`) a chunk hash that an earlier admission committed as sealed and that has not been scored: admitting it would be a read path with no frozen prediction | The read path, or the fields that differ / a frozen record that precedes the read | 1. Register a new prediction version, which needs fresh sealed data. 2. For a plain read: freeze a prediction first, then score | The sealed store and its read guard (BD-387: the only read path takes a score token, which `ground.score` alone mints), admission (BD-387), the prediction record (N30a-31) and scoring (N30b-06); `ground.predict`, `ground.score`, `ground.ingest`, and the read verbs BD-387 names as paths that could reach a sealed chunk: `artifact.read`, `run.export`, `telemetry.query` (the viewer and a direct store call are refused the same way). Second raising site, merged into this code by the allocation rule (N28cx-01, `prediction-changed-after-freeze`): the studio's prediction ledger (BD-344) on `predict.register`, for a registration by the same predictor for the same target whose content differs from a record in the target's freeze set; told apart by `pointer` (`prediction` for the studio's record, `ground.prediction` for this chapter's); its repair is 1. register a new version, which applies to targets not yet dispatched (none for this target: the record stands and will be scored), with `retry: never` for that target (DESIGN of N28cx-01). Interim form: the held request `prediction-after-outcome` (a registration after an outcome for its target exists in the ledger) is raised as 883 with `detail = "registration-after-outcome"` by `predict.register` (and `experiment.suggest` when it registers its own outcome interval) until its `8xx` number is released | M4 (`predict.register`), M5 |
| FROB-REF-886 | Reuse of a spent holdout | A holdout chunk already scored for this cell and model lineage named again in `ground.score`; a spent chunk hash declared as `holdout` or `validation` in a later dataset | The chunk hash, the scoring event that spent it / unspent sealed data | 1. Acquire fresh sealed data. 2. Use the spent chunk as calibration or regression data | Scoring, admission; `ground.score`, `ground.ingest` | M5 |
| FROB-REF-889 | A band for an uncalibrated cell, or with too few calibration points | A band, or a status `candidate` or `calibrated`, requested or cited for a cell with no band record; n below the minimum count (19 episodes for a per-unit cell; ⌈(1 − α)/α⌉ units for a band across units); a finite band cited at a level whose quantile index ⌈(n + 1)·(1 − α)⌉ exceeds n; a status line that does not parse | Samples present, level / samples needed | 1. Acquire the k more calibration episodes (or units) stated. 2. Lower the level to one that is finite at this n. 3. Report `reality: no-claim` with the simulation's own interval | Bands and the reality guard; `ground.band` | M1 (guard), M5 (bands) |
| FROB-REF-895 | A band for a design that was never built | A band or a Reality status for a `DesignId` with no admitted dataset of origin `real` naming a physical unit of that design: an unbuilt design, an edited child of a built one, a generated family, a design with synthetic data only, a robot of which one part was scanned | The design identity, datasets found / a built and measured unit | 1. Report `reality: no-claim` with the simulation's own interval. 2. Build and measure a unit, then calibrate its cell | Bands and the reality guard; `ground.band` | M1 (guard), M5 |
| FROB-REF-897 | Use of a retired calibration | A band, an admitted card or a status whose cell is `retired(cell, reason)`, used in a report, a prediction, a randomization width or a verdict | The retirement reason (scope component that changed, rank e-value, miss e-value, or expiry limit) and its ledger sequence / a calibration in the present regime | 1. Recalibrate inside the new regime. 2. Report `reality: no-claim` | Drift and retirement; `ground.band`, and every consumer of a band | M1 (guard), M5 |
| FROB-REF-899 | A private or non-redistributable input to a kit | A kit input that carries or inherits the private mark; any non-dataset input whose joined licence set forbids redistribution (`research-only`, `proprietary`, `unknown`) | The offending inputs with the lineage step that marks each / inputs that may be redistributed | 1. Remove or replace the offending inputs. 2. For a dataset under a restrictive licence, ship its hash and acquisition protocol in place of its bytes. 3. Build the kit on the synthetic rehearsal only. There is no override flag | The kit builder; `ground.kit` | M5 |
| FROB-REF-812 | An as-built registration that is ill-posed or below its noise screen | Fewer than three fiducial correspondences on a part, or fiducials collinear within tolerance; a scan with neither fiducials nor an identified external registration tool; a registration residual at or above the deviation under review; a scale estimate that raises unit suspicion, when a deviation or posterior is requested from it | Fiducials found, residual, deviation under review, scale / at least three non-collinear fiducials; residual below the deviation | 1. Supply the fiducials named. 2. Accept the smallest deviation resolvable at this residual, stated. 3. Register with an identified external tool and record its identity. 4. Re-export in the declared unit | As-built ingestion; `ground.ingest` | M5 |
| FROB-REF-814 | Pooling across cells, across `ModelId`s, or of synthetic with real data | A calibration set, an identification batch or a monitor stream whose rows come from more than one cell key, more than one model identity, or both origins; a chunk hash once admitted as `synthetic` appearing in a dataset declared `real` | The partition of the offered rows by cell, model identity and origin / one pool per cell, identity and origin | 1. Separate the pools as listed and repeat per pool | Admission, identification, bands; `ground.identify`, `ground.ingest`, `ground.band` | M1 (guard), M5 |

**How 876 appears at `ground.ingest`.** Admission and clock fitting are two steps of one call, and a refused
clock fit does not undo admission (time-free metrics stay scorable on the dataset). The call then completes
with exit code 2 and carries the full refusal object of FROB-REF-876 in `clocks[i].refused`, with time-based
metrics reported unavailable (the integrated behaviour BD-389 fixes). The row's `exit = 3` is the code of a
refused call, which is how 876 appears at `ground.score`.

**Per-verb refusal lists** (entered in `registries/verbs.toml`; the generated-artifact check compares both
directions): `ground.ingest`: 873, 876, 812, 814, 886, 883; `ground.excite`: 879; `ground.identify`: 879, 814;
`ground.predict`: 883 (plus the consumed 532, 552 and 102, and the schema refusal 802 that every verb carries,
which the prediction registrar raises for a record with no cell, an unknown card, an interval with lo > hi or a
level outside (0, 1)); `ground.score`: 883, 886, 876; `ground.band`: 889, 895, 897, 814; `ground.kit`: 899.
Outside the `ground` cluster, 883 is in the rows of `predict.register` (M4), `artifact.read`, `run.export` and
`telemetry.query` (M5, BD-387's read guard); the generated-artifact check holds those rows to this row too.
The two-direction comparison covers the eleven codes of this registry and no others. The codes of other
chapters that the `ground` rows list are outside it and are held to their own registries' rows: 828, 832 and 731
on `ground.ingest` (an importing verb, whose handler also refuses a private-marked bundle without the private
scope, BD-387), and 816, 731, 733 and 734 on `ground.kit` (an exporting verb with a path argument).

**Message templates (DESIGN).** One line each, naming what was refused and the observed and required values.
Examples: 889: "No band: 8 calibration points in cell c:41aa…, 19 needed", `pointer` "cell.n", `observed` "8",
`required` "19", `unit` "episodes". 876: "Clock `mcu` has 1 mark; an affine fit needs 2 that bracket the trial",
`observed` "marks: [pulse-0 @ 0.000 s]", `required` "a mark at or after 8.000 s". 895: "Design d:7c03… has no
measured unit; no reality band exists for it". `affected_claims` lists the claim identifiers that would have
rested on the refused result.

**Registry mechanics.** Each row carries `code`, `block`, `message`, `retry`, ordered `repairs`, `exit = 3`,
`defined_in`, `raised_by` (crate and module, for example `frob-ground::band`) and `verbs`.

## Acceptance criteria

- [ ] The eleven rows are present in `registries/refusals.toml`, load without error, and each has a message, a
      retry class, at least one ranked repair and exit 3.
- [ ] For each code, every verb in its list names the code in `registries/verbs.toml` and no `ground` verb names
      one of these codes without the row naming it (`xtask check-generated`); the rows outside the cluster that
      the 883 row names (`predict.register`, `artifact.read`, `run.export`, `telemetry.query`) list 883.
- [ ] The 883 row documents both raising sites with their `pointer` values (the merge of
      `prediction-changed-after-freeze`, N28cx-01) and the interim form of `prediction-after-outcome`.
- [ ] No `ground` verb lists a code in 840 to 850; a planted registry in which `ground.band` lists 846 fails the
      check with a message that names the map above.
- [ ] No two rows share a code; none of the eleven collides with a code defined elsewhere.
- [ ] The refusal envelope for each code validates against the envelope schema with `pointer`, `observed`,
      `required` and `repairs` populated (one fixture refusal per code, built with the constructor the mechanisms
      use).
- [ ] The agent-facing reference generated from the registry lists all eleven with their repairs.

## Test plan

### Unit tests

`crates/frob-core/tests/refusals_ground.rs`:
- `eleven_codes_present`: the exact set {812, 814, 873, 876, 879, 883, 886, 889, 895, 897, 899};
- `all_exit_3`, `all_have_repairs`, `retry_class_after_change`;
- `verbs_lists_agree`: the per-verb lists above, both directions, over the eleven codes of this registry; the
  codes of other chapters on the `ground` rows (828, 832 and 731 on `ground.ingest`; 816, 731, 733 and 734 on
  `ground.kit`) are outside the comparison and do not fail it, while a `ground` row that lists one of the eleven
  without this registry's row naming the verb still does;
- `no_stale_840_block_on_ground_verbs`;
- `fixture_envelopes_validate`: one `synthetic` refusal per code;
- `stage_order_fixture_is_879`: the fixture refusal for a `--stage contact` request with no actuator-stage record
  carries pointer `stage`, `observed` "actuators: no record", and the repairs "run the missing stage first" then
  "`--keep-prior actuators`", in that order;
- `ingest_lists_883`: the row of `ground.ingest` names 883, and the 883 row names `ground.ingest`;
- `sites_of_883_by_pointer`: the 883 row documents two raising sites, and the fixture refusal of each carries
  its own `pointer` (`ground.prediction` for a Ground record changed after freezing; `prediction` for the
  studio's record on `predict.register`, with `retry: never`), with one code and one exit (3); the interim form
  with `detail = "registration-after-outcome"` resolves to 883 while the allocation ledger's row for
  `prediction-after-outcome` is `held`, and a scratch ledger in which that row is `assigned` makes the interim
  constant fail the check until it is switched;
- `repair_order_stable`: first repair of 889 is "acquire k more", of 895 "report no-claim", of 883 "register a
  new version", of 899 "remove or replace";
- `kit_has_no_override`: the row for 899 has no repair naming a flag, and the schema of `ground.kit` has no
  parameter whose name contains `force`, `override` or `allow-private`;
- `unknown_code_is_load_error`: a scratch copy of the registry with the `FROB-REF-899` row deleted, while
  `ground.kit` still lists `FROB-REF-899`, fails to load and names the code (one of this chapter's own codes on a
  scratch copy, so the test never depends on which numbers other chapters hold).

### End-to-end script and logging

`scripts/e2e/ground_refusals.sh` (bead N30b-37) fires each of the eleven on a planted violation through the real
verbs and logs, per code, a `refusal` line with code, exit code, pointer, observed, required and the ranked
repairs, compared with this registry.

### Known answers and negative controls

- Values that appear in payloads: minimum count 19; ⌈(1 − α)/α⌉ = 9, 19 and 99 units at levels 0.90, 0.95 and
  0.99; at n = 20 the largest finite level is 20/21 = 0.952.
- Negative controls: the registry with FROB-REF-889 removed (the `ground.band` verb fails to load); a row with
  exit 4; a `ground.kit` schema with an added `--allow-private` flag (`kit_has_no_override` fails).

## Considerations

- The stale 840-block numbers appear in two places of the source text. Any document, schema or test that
  repeats them is a defect; the acceptance check above exists to catch that.
- Three refusals of the chapter have no code of their own: a unitless tolerance in a prediction, a timestamp
  with no declared instant, and a dataset with no declared cell. DESIGN: they are raised under FROB-REF-102, -873
  (at admission; -876 at scoring) and -873. If the registry owner assigns narrower codes, only the `raised_by`
  tables change.
- Repairs are advice to an agent: each is an action it can take with a verb or by acquiring data, and the first
  is the cheapest one that makes the original question answerable or states the honest fallback.
- Numbering has gaps; the loader does not assume contiguity.

## References

§30.19, §30.16, §30.12 (rule 1); Appendix O (8xx); §32.4 (envelope and exit codes); GATE-GROUND-01, -05, -06, -09.
