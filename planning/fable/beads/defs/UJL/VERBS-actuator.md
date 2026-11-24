---
key: "VERBS-actuator"
title: "frob-verbs registry: cluster `actuator`, six rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "sinew", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§15.3", "§15.7", "§15.8", "§15.10", "§15.13", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `actuator`. The cluster covers actuator, battery and
transmission cards (and, from M4, the absorber cards of the safety shield, N22bx-01): the catalog, a card with its
per-field provenance, admission checks, the torque-speed envelope, the thermal steady state and the shared bus
ceiling. It has six verbs (6 `read`); the handlers live in
`frob-actuate`. The bead carries every row completely, with the three checks that keep the rows true: the generated
CLI commands and MCP tools match the list; every verb has an end-to-end script; the schema printed by `frob <verb>
--help --json` is stable.

Kinds: `read` returns stored or cheaply computed facts and changes nothing; `run` spends compute, takes an
idempotency key and a budget, and after a bounded synchronous wait returns a job handle with exit code 8; `mutate`
changes a named object in the caller's namespace and takes an idempotency key and an expected parent; `admin`
cancels, stops, evicts or deletes and needs a capability scope. Tiers: S Solid, F Frontier (with a pilot and a kill
gate), M Moonshot; `S/F` is a Solid core with Frontier extensions.

DESIGN: a registry bead per cluster exists beside the implementing beads because a row missing from the registry,
spelled twice, or served with different refusal lists on the CLI and over MCP is a surface defect that no mechanism
test catches. Rejected: rows left to the implementing beads alone (nothing checks their union against the
inventory); one bead for all the rows (too large to review).

## Scope

In: the six registry entries of cluster `actuator`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `actuator.list` | read | M1 | S | Actuator, battery and transmission cards in the catalog | §15 |
| `actuator.show` | read | M1 | S | A card and the provenance of every field | §15 |
| `actuator.check` | read | M1 | S | Admission checks of a card | §15 |
| `actuator.envelope` | read | M1 | S | The torque-speed envelope | §15.8 |
| `actuator.thermal` | read | M1 | S | The thermal steady state and runaway | §15.7 |
| `actuator.bus` | read | M1 | S | The shared bus ceiling | §15.10 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *card-static*: a static computation on card values. Class `Estimated` unless every card field the verb reads
  has provenance `measured`, then `Tested` (a `datasheet` or `generic` field is still `Estimated`);
  `evidence.weakest` names the field that set the class; tier `static`; determinism D2 unless the block says
  otherwise; `claims` empty; `reality: no-claim`. (An earlier draft of this bead labelled these verbs Tested and
  exact; the implementing beads BD-069, BD-072, BD-073 and BD-075 return the provenance-driven class, and they
  govern.)

**`actuator.list`** (MCP tool `actuator_list`; owner `frob-actuate`; paginated)
- Inputs: optional `--kind` (`actuator`, `battery`, `transmission`; from M4 also `absorber`, N22bx-01) and
  `--class` filters.
- Outputs: per card its name, kind, class, `CardId` (`name.fcard@b3:<hash>`) and the share of its fields in each
  provenance class (`measured`, `datasheet`, `generic`, `assumed`). Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-069; extended by N22bx-01 (the `absorber` kind and its four catalog cards, M4).

**`actuator.show`** (MCP tool `actuator_show`; owner `frob-actuate`)
- Inputs: card, of any kind, by name (the kind is in the card's header; an absorber card from M4, N22bx-01);
  `--capabilities` prints the row-option capability table (which tiers carry stiction, hard backlash, the exact
  voltage hexagon, the Stribeck shape; N15a-04).
- Outputs: every field with its provenance and uncertainty, the derived power-invariant modulus, the modulation
  limit rule, flags (`l1-reactive-drop-not-negligible`). Additive fields (same schema version): for a measured
  version (N15ax-03) the identified fields with value, σ, dataset hash and calibration cell, the correlation matrix
  with its field order (|ρ| above 0.9 flagged in the human form), the `delay_set` in seconds, the parent chain, the
  scope, and the status (`candidate` or `admitted`) with its record identifier, read from the ledger's calibration
  record keyed by `CardId` (`unknown (no record)` when absent; status is never a card field); for a class card,
  per hierarchical field the class mean and standard deviation, the unit spread with its degrees of freedom, the
  units pooled and the predictive σ; for an actuator card with a `rotor` block (N15b-36, M6, Moonshot) the block
  with each field's provenance; for an absorber card (N22bx-01, M4) its kind, stroke and fields with provenance.
  Evidence: *pass-through*.
- Chapter refusals: none of its own.
- Implemented by: BD-069; extended by N15ax-03 (measured-version and class-card blocks, M1), N22bx-01 (absorber
  cards, M4) and N15b-36 (the rotor block, M6).

**`actuator.check`** (MCP tool `actuator_check`; owner `frob-actuate`)
- Inputs: card (any kind by name; an absorber card from M4, N22bx-01), or `design:joint` (the card the design
  binds at that joint); `--bind <design>:<joint>` with a card (N15b-12: the card is bound in memory to the named
  joint in place of the card of its kind there, an actuator card keeping the joint's transmission and a
  transmission card keeping the joint's actuator; the bind-time checks and the witness run with that joint's
  context; nothing is written and the design keeps its `DesignId`); `--strict-data`; `--at τ,ω` (the evaluation
  point of the power-created figure, joint side; default the card's continuous torque at half its no-load speed).
- Outputs: the three checks, run at admission and again at binding: (1) torque and back-EMF constants agree in
  power-invariant coordinates within the stated uncertainty; (2) every loss term is nonnegative in all four
  quadrants (a `single-multiplier` loss law fails); (3) the stated peak and continuous torques are reachable
  inside the card's own voltage, current and thermal limits (the continuous rating under torque hold). At binding
  (`design:joint` or `--bind`) also the signed motion witness: a unit positive effort on the driving port must
  produce a rate of the authored sign at the driven port, per nonzero entry of a coupling matrix; the output
  carries `direction_witness` (`pass`, `fail`, or `not-bound` for a bare card) and `witness_entries` (one per
  driving and driven pair, with the authored and the observed sign). At the binding of a coupling transmission
  the output also carries `reflected_inertia` (BD-074): `{joints, matrix}`, the rotor inertias of the bound
  actuators pulled into joint space, T⁻ᵀ·diag(J_r)·T⁻¹ in kg·m² for a coupling q̇ = T·θ̇, exact in the cards'
  rationals, with `joints` in the order in which `actuator.envelope` lists a coupled group (the matrix is not
  diagonal in general: T = [[½, ½], [½, −½]] with rotors of 3·10⁻⁴ and 10⁻⁴ kg·m² gives [[4, 2], [2, 4]]·10⁻⁴,
  so unequal rotors couple the two joints); the field is absent for a
  bare card and for a transmission that drives one joint. The verb also runs the exact structural checks of
  N15b-09, at admission per element and at binding on the composite (J + Jᵀ = 0 and the lossless two-port
  identities as polynomial identities, R positive semidefinite by exact rational LDLᵀ, on the card's
  rationals); a failure is `FROB-REF-130` naming the element and the entry, and for an admitted card the output
  lists `certificates`, one `{kind, hash}` per instance certificate emitted (DESIGN name; `kind` is
  `poly-identity` or `rational-ldlt`, `hash` the content address of the certificate stored with the card
  version, which `frob-verify` checks offline). A measured version runs the same three
  checks on its values plus the validations of its covariance, delay-set, lineage and class blocks (N15ax-03); a
  rotor block is admitted only with 0 < FM ≤ 1 (N15b-36, M6). An absorber card is checked by the container's form
  rules (codes 102, 103; 113 under `--strict-data`) and the field rules of its kind (N22bx-01); the three
  electromechanical checks and the witness apply to actuator and transmission cards. Exit 0 clean, 2 with
  findings, 3 refused. Evidence: *card-static*, determinism D0 for check 2 (exact rationals), for
  `reflected_inertia` and for the certificates, and D2 for checks 1 and 3.
- Chapter refusals: `FROB-REF-102` and `FROB-REF-103` (form errors of the card: a unit error; a field without
  provenance or uncertainty, an unknown field or an unresolved card name); `FROB-REF-108` (power sign mismatch: a
  reversed signed motion witness at binding); `FROB-REF-113` (an actuator card with a required field of class
  `assumed` when the caller asked for `--strict-data`); `FROB-REF-130` (a card or transmission that is not passive:
  loss negative in some quadrant, efficiency above one, or torque and back-EMF constants inconsistent beyond the
  stated uncertainty; returned with the quadrant and the power created); `FROB-REF-131` (check 3: a peak rating
  outside the card's own envelope); `FROB-REF-132` (check 3: a continuous rating with no admissible thermal
  equilibrium, or one above `t_max_winding`); `FROB-REF-134` (a card without a current convention, or with
  conventions that disagree).
- Implemented by: BD-069; extended by N15a-03 (the re-run at binding, stage-1 refusals), N15b-12 (the signed
  motion witness, `--bind`, `direction_witness`, `witness_entries`), BD-074 (`reflected_inertia` at the binding
  of a coupling transmission), N15b-09 (the exact structural checks and `certificates`), N15ax-03 (measured
  versions), N22bx-01 (absorber cards, M4) and N15b-36 (rotor cards, M6).

**`actuator.envelope`** (MCP tool `actuator_envelope`; owner `frob-actuate`)
- Inputs: card or `design:joint` (a coupled group is addressed by naming any joint of the group; never a bare
  design); `--t-winding`, `--t-magnet`, `--v-bus`; `--speeds a:b:n`; `--at joint|motor`; `--shape
  circle|hexagon`; `--point τ,ω` with `--require`; for a coupled group `--torque τ₁,…,τₙ` (N15b-10: a joint-torque
  command for the group's joints in the order of `joints`, at the condition's joint rates, rest by default; the
  vector counterpart of `--point`), with `--require`.
- Outputs: the boundary of the feasible set as torque against speed for both signs of torque, each point with
  (i_d, i_q) and the binding limit; base, no-load and top speed (or `unbounded`); V_lim and the limit shape; the
  stated winding temperature, magnet temperature and bus voltage; with `--at joint` the boundary reflected through
  the bound transmission with its four-quadrant efficiency; for a coupled transmission the object
  `joint_torque_set` (`kind: "zonotope"`, `joints` in the column order of C whichever joint was named,
  `generators`, `intervals` [λ⁻, λ⁺], `condition` {winding_temperature, bus_voltage, joint_rates}) instead of
  per-joint limits; flags (`l1-reactive-drop-not-negligible`; `six-step: hexagon only` when a `six-step` card is
  answered without an explicit shape); the claim-product condition `iron-loss-not-modelled` for a card without
  iron-loss coefficients (N15b-27). Evidence: *card-static* (L2 laws whatever the simulation level); constant-C
  coupled results D0.
- Chapter refusals: `FROB-REF-131` (a required operating point outside the torque, speed and voltage envelope,
  or a `--torque` command outside the coupled joint-torque set; returned with the point, the boundary or
  separator, the nearest point of the set, and the gear-ratio or card edit that admits it); `FROB-REF-136` (an
  explicit `--shape circle` on a card with `modulation: six-step`: no inscribed limit radius is on file, the
  fail-closed rule of N15a-02 raised by the static envelope library N15a-16; returned with the tiers that carry
  it, `referee`, whose exact hexagon serves the card; repair `--shape hexagon` or the card option `svm-linear`).
  Without `--require` an outside point or command is a finding, exit 2, carrying the separator.
- Implemented by: BD-073, on the static envelope library N15a-16; extended by N15b-10 (coupling maps, the
  joint-torque zonotope, `--torque`) and N15b-27 (the iron-loss condition).

**`actuator.thermal`** (MCP tool `actuator_thermal`; owner `frob-actuate`)
- Inputs: card or `design:joint`; `--current I` or `--torque τ` (with `--hold torque` for the torque-hold curve);
  `--rms-of <run|gait>`; `--speed ω` (BD-072: the motor shaft speed at which the iron loss P_fe = k_h·abs(ω_e) +
  k_e·ω_e², ω_e = `pole_pairs`·ω, is added to the loss curve; default 0, the holding duty; not accepted with
  `--rms-of`, which uses the run's or gait's own speeds, the pair failing the input schema with `FROB-REF-802`);
  `--ambient T`; `--start cold|soaked:T`; `--require`.
- Outputs: `stable`; the equilibrium temperature or its absence; time constant or growth rate; time to limit;
  I_crit and I_continuous; under torque hold the tangency and the margin to it; the largest sustainable load; the
  thermal network used; the constant-resistance comparison, labelled as the wrong model; the claim-product
  condition `iron-loss-not-modelled` for a card without iron-loss coefficients (N15b-27). Evidence:
  *card-static*.
- Chapter refusals: `FROB-REF-132` (no stable thermal equilibrium at a required operating point, or an equilibrium
  above a limit), raised with `--require`; without it the same facts are a finding, exit 2.
- Implemented by: BD-072, on the static thermal library N15a-14 and the duty-history start states of N15a-15;
  extended by N15b-27 (iron loss at `--speed`).

**`actuator.bus`** (MCP tool `actuator_bus`; owner `frob-actuate`)
- Inputs: design and bus id; optional `--demand <power>`, `--task`, `--soc`, `--cell-temp`, `--condition`
  (default `end-of-discharge-cold`); `--source-voltage <V>` (BD-075, DESIGN: pins E to the stated value in place
  of its evaluation from the battery state at the condition, R read as before; the answer reports `condition:
  source-voltage-pinned` and is never the design's stage-6 verdict, which always evaluates E); from M6
  `--battery-age f_q=<ratio>,f_r=<ratio>` (N15b-14: either member may be omitted and defaults to 0; the ceiling is
  evaluated at Q = Q₀·(1 − f_Q) and R₀ = R₀₀·(1 + f_R), at end-of-discharge voltage and cold resistance; the
  report states the age it assumed).
- Outputs (`frob.actuator.bus.out/1`): `source_voltage_v`, `series_resistance_ohm`, `ceiling_w` (E²/(4·R)),
  `demand_w`, `voltage_at_demand_v`, `current_at_demand_a`, `margin_w`, `current_max_a`, `feasible`,
  `condition`, `members`, `regen_policy`. The verb is static: brownout events are world events of a run
  (`sim.run`), not outputs of this verb. Evidence: the weakest provenance among the battery and harness fields
  read; tier `static`; determinism exact for the rational parts and double precision for the square root.
- Chapter refusals: `FROB-REF-133` (peak demand above the bus ceiling E²/(4·R)); `FROB-REF-135` (power requested
  from an exhausted battery, or state of charge driven outside its bounds); from M6 `FROB-REF-111` (an ageing
  value outside its bounds: f_Q outside [0, 1) or f_R negative, N15b-14). An aged capacity that makes the
  demand infeasible surfaces through 133 and 135, never a new code.
- Implemented by: BD-075; extended by N15b-14 (`--battery-age`, M6).

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `actuator.list` | `actuator:read` | `frob.actuator.list.out/1` | `actuators` | `frob actuator list --json` |
| `actuator.show` | `actuator:read` | `frob.actuator.show.out/1` | `actuators` | `frob actuator show qdd-m --json` |
| `actuator.check` | `actuator:read` | `frob.actuator.check.out/1` | `actuators` | `frob actuator check qdd-m --json` |
| `actuator.envelope` | `actuator:read` | `frob.actuator.envelope.out/1` | `actuators` | `frob actuator envelope qdd-m --v-bus 22.2V --json` |
| `actuator.thermal` | `actuator:read` | `frob.actuator.thermal.out/1` | `actuators` | `frob actuator thermal qdd-m --torque 6N*m --hold torque --json` |
| `actuator.bus` | `actuator:read` | `frob.actuator.bus.out/1` | `actuators` | `frob actuator bus quad-a --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column; `paginated = true` where the block says paginated.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "actuator.list"
cluster = "actuator"
kind = "read"
tier = "S"
capabilities = ["actuator:read"]
input_schema = "frob.actuator.list.in/1"
output_schema = "frob.actuator.list.out/1"
refusals = []
idempotent = true
cost_model = "static: answered from the store or by a small CPU computation; no job"
first_milestone = "M1"
task_clusters = ["actuators"]
examples = ["frob actuator list --json"]
owner = "frob-actuate"
purpose = "Actuator, battery and transmission cards in the catalog"
paginated = true
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind (`run`: `sync_wait_ms` from 0 to 30,000 with MCP
   default 5,000, `idempotency_key`, `budget`, `estimate_only`, `on_cancel` (`leave`, the default, or
   `cancel_job`) and `envelope_scope` (`--envelope-scope <gate:…|campaign:…|pretrain:…>`: the approved compute
   envelope scope the job cites; its schema, its place in the scope order and its refusals are NJL-01's);
   `mutate`: `idempotency_key`, `expected_parent`). `idempotency_key` is in no JSON-schema `required` array: the
   annotation `x-frob-key` (`"required"`, or `"optional"` on a `key_optional` entry) carries the rule, and a
   missing key is refused with 805 by the admission transaction on both surfaces. The generator appends the
   surface-wide refusals: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this
   build) and 802 (schema failure, including an unknown parameter) for every verb; 701 to 704 (budget and
   admission), 761 (above the compute envelope in force), 803 (key reused with different content) and 805
   (missing key) for `run`; 725, 803, 804 (stale parent) and 805 for `mutate`. The other properties and codes are
   added by field, as the variation table of NJL-01 lists them (that table is the one list; its variations 10
   to 16, which later beads land, are read there and not copied here): 806 and 807 for paginated reads; 808 (an
   integer outside ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property and for no
   entry by kind (every `run` entry has one in `sync_wait_ms`, every paginated entry in `limit`); a
   `key_optional` entry is not given 805; a non-`run` entry with `estimable = true` gets `estimate_only`; 790
   for a non-empty `needs_groups`; 891 to 893 for artifact `params`; 734 and 828 for a parameter that accepts a
   path to a file (`artifact-input<F>`, `artifact-identity<F>`, `foreign-file`).
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- All six verbs are `read`; `actuator.list` is paginated (`paginated = true`: `cursor`, `limit`, `follow`, codes
  806 and 807). A card field of provenance `assumed` makes every result built on it Estimated; under
  `--strict-data` it is a refusal (`FROB-REF-113`).
- Output schemas are the registry's `frob.actuator.<verb>.out/1` (a verb that returns no named artifact has
  `.out/1`); BD-072, BD-073 and BD-075 name them so.
- Options added by extending beads after M1 (`absorber` at M4, `--battery-age` and code 111 at M6) are listed in
  the blocks now, labelled with their milestone. The generated input schema gains each when its extending bead
  lands (an additive change); before that, a call that passes it is an unknown parameter, `FROB-REF-802`, exit 3.

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly six entries with `cluster = "actuator"` (6 `read`), each with every
      field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster actuator` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 6, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_actuator.sh` runs each verb of the cluster (six), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/actuator/<verb>.help.json` byte for byte for each of
      the six, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_actuator` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 6 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `actuator.bus` is `read` at M1).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`actuator.list`: none; `actuator.show`: none;
  `actuator.check`: 102, 103, 108, 113, 130, 131, 132, 134; `actuator.envelope`: 131, 136; `actuator.thermal`:
  132; `actuator.bus`: 111, 133, 135); the appended codes equal the rule for its kind and fields (`actuator.list`
  also 806 and 807, and 808 through `limit`, the cluster's only `int53` input, so no other row has 808). A code
  whose raiser lands after the row's first milestone (111 on `actuator.bus`, M6) is in
  the list from the start: the entry has no per-code milestone, and a listed code that is not yet raised is legal.
- `input_properties`: each row's generated input schema names the M1 options of its block, including the M1
  extensions (`actuator.check` has `bind`; `actuator.envelope` has `torque` and `shape` with values `circle` and
  `hexagon`; `actuator.thermal` has `speed`; `actuator.bus` has `source_voltage`); a planted row without `bind`
  fails. The later extensions (`absorber` in `actuator.list`'s `kind` at M4, `battery_age` on `actuator.bus` at
  M6) are additive changes that keep the schema's major version; their beads (N22bx-01, N15b-14) re-pin the
  snapshot when they land.
- `check_output_fields`: the generated `actuator.check` output schema has `direction_witness` and
  `witness_entries` (N15b-12), `reflected_inertia` with `joints` and `matrix` (BD-074) and `certificates`, a list
  of objects with `kind` (`poly-identity` | `rational-ldlt`) and `hash` (N15b-09); a planted schema without
  `reflected_inertia` fails.
- `mcp_names_one_to_one`: the tool names are the mapped names (`actuator_list`, `actuator_show`, `actuator_check`,
  `actuator_envelope`, `actuator_thermal`, `actuator_bus`), with no collision against any other cluster.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_actuator.sh` (header: `bead=VERBS-actuator milestone=M1 labels=tether,verbs,verb:actuator.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance actuator`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the six verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
actuator` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster actuator`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`); (7) one envelope per call and no exit code 1.

Log: `target/e2e-logs/verbs_actuator/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 6
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 6 verbs (6 `read`); the tables above. KA-755 (the exit-code table on malformed,
  unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's tables;
  exact) is exercised for every verb by steps (6) and (7).
- Negative controls the script must catch when planted: `alias` (a second spelling of one verb: the build fails with
  the alias lint); `missing-row` (a row removed from the served MCP list: `missing_mcp` is not empty);
  `renamed-field` (an output property renamed without a version change: the snapshot check reports `breaking`). A
  verb answering with two envelopes or exit code 1 fails step (7).
- Invariants carried: INV-TETHER-01 (every surface is generated from the registry; nothing exists on one surface and
  not the other, apart from declared per-surface defaults and the unknown-verb case) as step (2); INV-TETHER-02
  (exactly one envelope per invocation with `--json`) as step (7). Determinism lanes: entries, tool definitions and
  snapshots byte-identical twice on one machine, across debug and release and across Linux and macOS (class D0); the
  verbs' own lanes belong to their implementing beads.

## Considerations

- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. The per-milestone surface beads (BD-137) re-run this script as
  verbs land; that is how GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §15.3, §15.7, §15.8, §15.10, §15.13, §32.4, §32.6,
§32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755.
