---
key: "N17-REF"
title: "Phylum refusal registry: FROB-REF-160, 161, 162, 540, 541, 740, 741 with triggers, repairs and exit codes"
type: "task"
priority: 1
milestone: "M2"
labels: ["m2", "phylum", "frob-phylum", "solid", "registry", "test", "review-notes-open"]
parent: "WP-44"
deps: ["BD-005", "BD-022", "STD-E2E"]
needs: []
related: ["N16-REF", "N16c-26", "N17b-05", "N17b-11", "N21b-16", "N23a-25", "N23a-31", "N24ax-04", "N32a-03", "N35-33", "ROBOT-apollo", "VERBS-phylum"]
gates: []
plan_refs: ["§17.16", "Appendix O", "§32.4", "§35.11"]
---
## Background

Phylum is the subsystem (crate `frob-phylum`, layer 3) that makes a population of different robot designs, and not
one model, the unit of simulation work. Every way in which Phylum declines to answer is a typed refusal: a stable
code, a message, the field at fault, the observed and required values with their unit, and repairs ranked by how
likely they are to help. The project's standing rules are: refuse, never truncate; a budget stop publishes nothing
partial as a result; a misspelt parameter is a refusal and not a default. Refusal codes live in one machine-readable
registry, `registries/refusals.toml`, loaded by `frob-core`; an unknown code is a load-time error and a code is never
reused. Code families: `1xx` design and model, `5xx` learning and evaluation protocol, `7xx` budget and capability.

Phylum defines seven codes. This bead is the registry of those seven: it puts the rows in the registry with their
full content, fixes the exit code of each, and owns the test that every code is actually raised by its trigger. The
mechanisms that raise the codes are other beads; each is named in the table so that no code is left without a
raising site. Why a separate bead: the plan's rule is that a refusal which exists only as a row of a table and is
never raised is worse than no refusal, because agents plan around repairs that the system never offers.

## Scope

In: the seven registry rows with meaning, trigger, sub-reasons, envelope fields and ranked repairs; the exit-code
mapping; the registry tests; one end-to-end script that provokes each code from the command line.
Out: the mechanisms that detect each condition (named per row below), including the raising sites owned by other
chapters, which the rows name (the device tiers' slot admission for 160, N21b-16; the learning subsystem's
equivariant layer for 161, N23a-31; the Kinesis adjoint for 540, N16c-26).

## Technical approach

**Envelope shape (FACT about the project's contract, schema `frob.envelope/1`).** When `ok` is false the envelope
carries `refusal = { code, message, pointer, observed, required, unit, affected_claims, retry, repairs[] }`, where
`retry` is one of `never`, `after-change`, `backoff`, and each repair is a typed suggestion (a verb with arguments or
a typed design edit) with a one-line reason. Exit codes: 3 = refused input or question (codes `1xx` to `5xx`);
4 = budget, refused at admission or stopped; 5 = capability; 6 = backend or device. Exit 1 is never emitted on
purpose.

**The seven rows.**

| Code | Name | Raised when | `pointer` and values | Ranked repairs | `retry` | Exit | Raising bead |
|---|---|---|---|---|---|---|---|
| `FROB-REF-160` | No shape class | A design exceeds the largest admitted size bucket (any capacity: degrees of freedom, links, tree depth, collision samples, contacts, rows, actuators) and the tree-operator route for large mechanisms is off | The `DesignId`, the capacity exceeded, `observed` = what the design needs (for example 72 degrees of freedom), `required` = the largest admitted capacity (64 by default), `unit` = `dof`, `links`, … | 1. Enable the tree-operator route (offered only in builds where that route exists). 2. Run the design alone on tier `batch-cpu`, which needs no cohort and no padding. 3. Simplify the design (typed edits: remove a limb, fold fixed joints) | `after-change` | 3 | N17a-02 (the fit, the one raising site in the crate), reached through BD-170 (lowering), N17b-04 (`phylum.sweep`) and BD-171 (`phylum.plan`), and on the device tiers at variant-slot admission (N21b-16) |
| `FROB-REF-161` | Group not observation-preserving | A quotient of a design sweep, or a tie of policy weights, is requested under a symmetry group that moves a sensor pose or field of view, the payload, or the task's goal orientation | The generator of the requested group that fails and the element that breaks it (sensor id, payload id or task term); `observed` = the requested group and its order (for example the dihedral group of order 12); `required` = the observation-preserving subgroup Γ_obs and its order (for example the mirror group of order 2) | 1. Use Γ_obs. 2. Declare the invariance explicitly on the design (the `assert-symmetry` edit) and let the metamorphic equivariance test check it | `after-change` | 3 | N17a-23 (orbit representatives), surfaced by BD-285 (`phylum.quotient`); the weight-tie site is the equivariant policy layer of the learning subsystem (N23a-31) |
| `FROB-REF-162` | Reuse without a dependency digest | A cached result is offered for reuse and has no recorded dependency digest, or the one-in-64 shadow recomputation disagreed with the cached bytes | The memo node kind and key; `observed` = `dep_digest: absent`, or the two differing digests | 1. Recompute the result. 2. Quarantine the node kind (stop serving every node of that kind) until the mismatch is explained | `after-change` | 3 | N17b-13 (calibration and quarantine), BD-286 (the contract), N17b-14 (`phylum.reuse`), N17a-26 (memo entries of the mission digest) |
| `FROB-REF-540` | Gradient unreliable | A paired finite-difference design gradient is requested and one of four conditions holds; `pointer` names which: `metric` (the metric is not smooth: a binary outcome, a mode switch, a count), `horizon` (the horizon exceeds the admitted cap of 50 control intervals for a class with no recorded variance ratio of at least 2 at that horizon), `divergence` (more than half the pairs diverged before a quarter of the horizon), `determinism` (the placement cannot deliver a controlled determinism class) | The metric id, or the requested and admitted horizon in control intervals, or the share of diverged pairs and the step, or the determinism class observed and required | 1. Shorten the horizon. 2. Change the metric to a smooth one (energy, tracking error, a margin). 3. Use rank-based search (CMA-ES or racing through `opt.start`) | `after-change` | 3 | N17b-22 (estimator), BD-353 (`phylum.gradient`); the adjoint method `phylum.gradient --method adjoint` refuses a non-smooth metric with the same code and `pointer: metric` (N16c-26, M6; the Kinesis ledger slug `adjoint-metric-not-smooth` is merged into this row) |
| `FROB-REF-541` | Outside generator support | A family-policy score is requested for a design that lies outside the support of the generator the policy was trained on; such a score is labelled out-of-distribution and is not a rung of the evaluation ladder | The `DesignId`, the `GeneratorId`, and the first derivation step at which the design leaves the grammar | 1. Fine-tune the policy on the design. 2. Train a specialist. 3. Extend the generator and retrain | `after-change` | 3 | N17a-20 (the support test `in_support` and `phylum.generate --locate`, on the generator BD-287; VERBS-phylum lists 541 on `phylum.generate`). The family-policy scoring verb `policy.apply` (N23a-25) calls the same support test and does not raise 541: by the division of the learning registry (N23-REF) it raises `FROB-REF-534` (a family policy applied outside its class or pretraining distribution) and cites the 541 finding (the `GeneratorId` and the first failing derivation step) in that refusal's payload, so VERBS-policy lists 541 as cited, not raised |
| `FROB-REF-740` | Population exceeds device | The bytes a population needs (physics state per padded world, model tables, rollout storage when training) exceed every admitted placement | `observed` = bytes needed in total and per shard; `required` = the budget of each candidate placement; `unit` = `bytes` | 1. Fewer worlds per design. 2. Split across the two cards (each shard must fit on its own; the cards have no link). 3. Switch to the path-table Jacobian (it replaces R·n_v by about R·(6 + depth) per world). 4. Run on the 256 GB Apple machine | `after-change` | 4 | N17a-07 (the memory model and placement verdicts), surfaced by BD-171 (`phylum.plan`) and N17b-04 (`phylum.sweep` admission), and by the admission controller's memory preflight when a population is admitted to a device (N17b-05); BD-219 refines the estimate the verdict reads (exact state bytes) and raises nothing of its own |
| `FROB-REF-741` | Bucket fragmentation | More than eight size buckets are needed to hold padding waste at or below the limit (default 25%), or contact capacity forces a second bucket axis that fragments the population | `observed` = buckets needed; `required` = 8; the waste limit in force; the histogram of sizes | 1. Stratify the generator by shape class so the population lands in fewer buckets. 2. Raise the waste limit knowingly (an explicit argument, echoed in the envelope). 3. Split the campaign | `after-change` | 4 | N17a-05 (the bucket planner), surfaced by BD-171 (`phylum.plan`) and N17b-04; N17b-11 freezes the bucket set and limit it reads |

**Planning limits behind 740 (FACT, each to be re-read before a gate cites it).** 16 to 19 GB usable of 24 GB on one
RTX 4090; on the M5 Ultra a measured Metal working-set limit of about 222 GiB and a largest single buffer of about
167 GiB. Bytes per padded world, physics only: about 35 kB in the 18-degree-of-freedom bucket (contact capacity 32,
row capacity 160), 85 kB at 36 (48, 256) and 223 kB at 64 (96, 448) with a dense Jacobian; about 125 kB at 64 with
the path-table Jacobian. Model tables add about 192 bytes per link. Training adds 23 to 100 kB of rollout storage
per world (an assumption).

**Exit code of the `74x` codes (DESIGN).** The project's exit table names `70x` for exit 4 (budget: refused at
admission or stopped) and `72x`, `73x` for exit 5 (capability). It does not name `74x`. Both Phylum codes are
refusals at admission for lack of memory or of an admissible plan, so they map to exit 4. The mapping is written in
the registry row, because the surface gate requires every refusal code to map to exactly one exit code (N32a-03's
lint: a `74x` row without an explicit `exit` fails), and it is the `74x` → 4 entry of the registry's `[exit_rule]`
that N35-33 confirms against this row.

**Registry row format.** One TOML table per code: `code`, `name`, `family`, `message_template`, `subreasons`
(for 540), `exit`, `retry`, `repairs` (ordered), `defined_in = "§17.16"` (the plan subsection, as N35-33's
registry-wide check requires), `raised_by` (the bead keys of the "Raising bead" column), `first_milestone` (160, 740, 741: M2;
161, 162, 541: M3; 540: M4). A row may exist before its mechanism: the verb that would raise it then answers the
capability refusal `FROB-REF-724` (not implemented in this build).

## Acceptance criteria

- [ ] `registries/refusals.toml` holds the seven rows above, schema-valid, with ordered repairs and one exit code each.
- [ ] `cargo test -p frob-core` loads the registry; a duplicated code, an unknown code and a row with two exit codes are each load-time errors.
- [ ] Each of the seven codes has a raising test in its raising bead that asserts code, `pointer`, `observed`, `required`, `unit`, `retry` and the repair order.
- [ ] The crate `CONTRACT.md` of `frob-phylum` lists the seven codes in its refusals section with the same wording.
- [ ] `frob schema` and the generated refusal table print the seven rows; the served MCP tool list shows the same refusal lists per verb.
- [ ] The end-to-end script passes: every code whose mechanism is present in the build is provoked and returns the registered exit code; a code whose mechanism is absent is logged as `skipped: capability absent` and never as passed.

## Test plan

### Unit tests

- `refusals_phylum_rows_present`: loads the registry and asserts the seven codes, names and families (three `1xx`, two `5xx`, two `7xx`).
- `refusals_phylum_exit_codes`: 160, 161, 162, 540, 541 → 3; 740, 741 → 4.
- `refusals_repairs_ordered`: the first repair of 740 is "fewer worlds per design" and the last is the Apple machine; the first repair of 161 is "use Γ_obs".
- `refusal_540_subreasons_exhaustive`: the `pointer` of a 540 refusal is one of `metric`, `horizon`, `divergence`, `determinism`; any other string fails to serialize.
- `refusal_row_before_mechanism`: with the gradient capability absent, `phylum.gradient` answers `FROB-REF-724`, exit 5, and not 540.

### End-to-end script and logging

`scripts/e2e/phylum_refusals.sh` sources the shared helper of `STD-E2E` and logs newline-delimited JSON to
`target/e2e-logs/phylum_refusals/<utc>.ndjson` with a transcript beside it. Steps, each a `cmd` event with the exact
command line, exit code, wall time and the envelope's `evidence` block, then a `refusal` event carrying the
envelope's whole `refusal` block (code, pointer, observed, required, unit, retry, repairs), followed by `assert`
events:

1. `frob phylum plan --designs @fixture:too-big-72dof --json` → exit 3, `FROB-REF-160`, `observed` 72, `required` 64.
2. `frob phylum plan --designs @fixture:nowhere --worlds-per-design 2048 --json` (4,096 designs in the 64 bucket) → exit 4, `FROB-REF-740`, bytes observed about 1.87 TB, above every placement (one RTX 4090, the two cards split, the M5 Ultra). A shape that fits the M5 Ultra, such as 256 designs × 2,048 worlds in the 36 bucket (44.6 GB), is not refused: 740 means no admitted placement fits, and the M5 Ultra is always among the placements the verdict evaluates.
3. `frob phylum plan --designs @fixture:nine-bucket-population --waste-limit 0.10 --json` → exit 4, `FROB-REF-741`, `observed` 9, `required` 8, repairs in the registered order.
4. `frob phylum quotient --designs @fixture:hexapod-front-lidar --group mech --json` → exit 3, `FROB-REF-161`.
5. `frob phylum reuse --design @fixture:quad-undigested --edit @fixture:edit-on-undigested-result --json` → exit 3, `FROB-REF-162` (the edit form of `phylum.reuse` takes `--design` and `--edit`).
6. `frob phylum gradient --design @fixture:ballistic --metric landed-beyond-10m --key ref-540 --json` → exit 3, `FROB-REF-540`, `pointer: metric` (a `run` verb, so the call carries `--key`).
7. `frob policy apply @fixture:quad-family --design @fixture:hexapod --key ref-541 --json` (the family policy is the positional input of `policy.apply`, a `run` verb) → exit 3, `FROB-REF-534`, never 541, with the 541 finding (the `GeneratorId` and the first failing derivation step) cited in the payload: the division of N23-REF. The code 541 itself is asserted by its raising bead's test (N17a-20, `refusal_541_shape`).

The final `summary` line gives assertions passed and failed, steps skipped with the reason, and `gates: []`.

### Known answers and negative controls

- Known values: the 1.87 TB of step 2 is 4,096 × 2,048 = 8,388,608 worlds × about 223 kB, against a largest budget of about 189 GiB (85% of the M5 Ultra's 222 GiB working set); the 9 of step 3 comes from a `synthetic` population of nine sizes {8, 10, 13, 17, 22, 28, 36, 47, 61} degrees of freedom with equal design counts. Padding waste inside a bucket is Ω₁ − 1 = c·N/Σ s − 1, so two adjacent sizes a < b in one bucket waste (b − a)/(a + b): 0.111 for 8 and 10, 0.120 to 0.133 for every other pair, all above the 10% limit, so each size needs its own bucket. (Under the default 25% limit the same population pairs up into five buckets and is not refused; a ratio of 1.3 between sizes with equal counts never forces nine buckets at 25%, and every size stays at or below 64 so that `FROB-REF-160` cannot fire first.)
- Negative controls: a planted registry row without repairs must fail the registry lint; a planted raising site that returns `ok: true` with a warning in place of 740 must fail step 2; a planted second exit code for 741 must fail `refusals_phylum_exit_codes`.
- Counterexamples behind the codes: CX-153 (161), CX-158 (162), CX-165 (540), CX-160 (741).
- Determinism: the script is run twice on one machine and the two logs must agree on every asserted field.

## Considerations

- The exit code of `74x` is a reading of an exit table that does not name the family; if the surface owners register a different mapping the row changes and nothing else does.
- Repair 1 of 160 names a route that is a Frontier pilot at a later milestone; until it exists the repair is omitted from the envelope, not printed as unavailable advice.
- Repair 2 of 741 lets a caller waste memory knowingly. It must be an explicit argument and must be echoed in the result; it never becomes a default.
- 541 is defined here and raised by the generator's support test; the learning subsystem's `policy.apply` cites its finding inside 534 and never returns it. If the family-policy work falls back at its kill gate, the code stays registered and is raised only by the generator's support test.

## References

Plan §17.16, Appendix O (O.1, O.5, O.7), §32.4, §35.11. Counterexamples CX-153, CX-158, CX-160, CX-165.
