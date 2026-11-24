---
key: "VERBS-contact"
title: "frob-verbs registry: cluster `contact`, seven rows, with parity, e2e and schema-snapshot checks"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "tether", "traction", "frob-verbs", "solid", "registry", "verb", "test", "review-notes-open"]
parent: "WP-06"
deps: ["BD-022", "NJL-01", "NJL-02", "NJL-04", "NJL-05", "STD-E2E"]
needs: []
related: []
gates: ["GATE-TETHER-02"]
plan_refs: ["Appendix J (J.1, J.2, J.3, J.5)", "§18.13", "§18.14", "§32.6", "§32.12.3", "§32.21 (GATE-TETHER-02)"]
---
## Background

FrankenRobotics (a robotics simulation and design system) is reached through the command line `frob`, the daemon
`frobd` and the MCP server `frob-mcp` (the Model Context Protocol, by which an agent harness lists and calls tools).
Every capability is a verb, `cluster.verb`, declared once in `registries/verbs.toml` (crate `frob-verbs`, tier
Solid). The CLI, the daemon protocol, the MCP tools, the schemas, the refusal table and the agent guide are
generated from that file, and gate GATE-TETHER-02 compares the served surfaces with it. The registry holds 244 verbs
in 44 clusters (namespaces): the 243 of the specification and `artifact.import` (VERBS-artifact); the
count of record is NJL-03's.

This bead is the registry of record for the cluster `contact`. The cluster covers contact laws and audits: the
admitted laws per tier and profile, a law, cone or film evaluated at a contact state, static contact maxima against
capacities, the tunnelling audit, clearance with a certificate, soil sinkage and thrust, and the contact response
batch. It has seven verbs (4 `read`, 3 `run`); the handlers live in `frob-contact`. The bead carries every row
completely, with the three checks that keep the rows true: the generated CLI commands and MCP tools match the list;
every verb has an end-to-end script; the schema printed by `frob <verb> --help --json` is stable.

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

In: the seven registry entries of cluster `contact`; the MCP tool names; the cluster's conformance script; the
help-schema snapshots; each verb's answer before its milestone. Out: behaviour (the implementing beads named below);
the entry schema and lints (NJL-01); the conformance harness (NJL-02); the alias lint (NJL-04); task clusters
(NJL-05, NJL-06); generation of the surfaces (the Tether seed beads).

## Technical approach

**The rows.**

| Verb | Kind | First milestone | Tier | Purpose | Defined in |
|---|---|---|---|---|---|
| `contact.laws` | read | M1 | S | The admitted contact laws per tier and profile | §18.13 |
| `contact.probe` | read | M1 | S | A law, cone or film evaluated at a contact state | §18.13 |
| `contact.capacity` | read | M2 | S | Static contact maxima against capacities | §18.13 |
| `contact.audit` | run | M2 | S | The tunnelling audit | §18.13 |
| `contact.clearance` | run | M3 | S | `clear` with a certificate, `penetrates` with a witness, or `unknown` | §18.13 |
| `contact.soil` | read | M4 | S | Sinkage and thrust on a soil card | §18.13 |
| `contact.response` | run | M4 | F | The contact response batch | §18.13 |

**Contract of each verb.** Evidence labels used below (the `evidence` block of the envelope):
- *pass-through*: passes through the claim product of the stored objects it reads and adds no claim of its own; the
  read itself is `exact`.
- *static*: class Tested, tier `static`, determinism `exact` (class D0); `claims` empty unless a certificate is
  attached; `reality: no-claim`.
- *known answer*: a Known answer where a registered row is recovered within its tolerance; class Tested.
- *certificate*: a Proof claim of class Certified where the checker accepted a certificate, otherwise the class of
  the weakest input (the claim product takes the minimum); tier `static`, determinism `exact`; `reality: no-claim`.
- *law evaluation*: tier `static`, determinism D2 on the CPU reference; class `Tested` for a dry law on `measured`
  or `datasheet` card fields, and `Estimated` for a semi-empirical law (the film and soil laws) and wherever a
  consumed card field is `assumed` (the weakest-input pointer then names that field); a realism claim over an
  `Estimated` result is refused by the claim guard, not by the verb.

**`contact.laws`** (MCP tool `contact_laws`; owner `frob-contact`)
- Inputs: optional `tier`, `profile`, `law`, and `design` (to show the laws and cards a design's end effectors
  bind); from M3 (N18b-10) `scene` with `design`: the cards per end effector and per region of that scene the
  design can reach.
- Outputs: per law its kind, tier label and parameters by class (physical and numerical) with units; per (tier,
  profile) the status, the gate and the receipt of its PASS, and the default law; for a design, per end effector
  the bound law, the r_eff source and (from M2) the interface cards with provenance, each row's certification
  status (`not-certified` for a rolling or torsional row without its convexity certificate, BD-167); the
  stiffness-admission figure where one exists; from M2 (N18bx-01) each card's `pulse` block or `pulse: null`;
  from M3 with `scene` (N18b-10), per end effector and region the resolved card, its provenance and the covering
  parameters; from M4 each card's group `attachment` where it has one (N15ax-02) and the law `CL-1R` with
  its parameter classes (q, τ, a₀ and the rate, all physical; N18ax-01). Evidence: *pass-through*.
- Chapter refusals: `FROB-REF-246` (law not admitted: A law requested on a tier or profile where its gate has not
  passed).
- Implemented by: BD-167, N18b-28. Extended by: N18bx-01 (the pulse fields of the output, M2), N18b-10 (`scene`,
  M3), N15ax-02 (the attachment group, M4), N18ax-01 (`CL-1R`, M4).

**`contact.probe`** (MCP tool `contact_probe`; owner `frob-contact`)
- Inputs, common to the state branches: `law` (a row of the law table, spelled as the table spells it, N18b-28:
  CL-1, CL-2, CL-M, CL-H; default the tier's admitted default; from M4 also `tyre`, N18b-04, and CL-1R, N18ax-01;
  a lowercase spelling of a CL row, such as `cl-1r` or `cl-h`, is `FROB-REF-802`, as on `sim.run`), `tier`,
  `profile`, `card` (an interface card by name or hash, an interface pair as in the example (`--interface
  <mover>/<substrate>`, resolved by the binding's lookup with the key (mover, substrate, `dry`) in key order,
  BD-167; a value that does not parse as two non-empty material names joined by one `/` is `FROB-REF-802`,
  pointer `/card`), a `.fcard` file, or inline parameters labelled `inline`; a file form is a path parameter, so
  the field rule applies), signed gap or prescribed `normal_load`, normal and tangential relative velocity, spin,
  optional `r_eff`, step length `h`, `strict_data`; optional `sweep` over one field (at most 1,024 points,
  paginated). From M3 (N18b-10) a scene position in place of `card`: `scene` (`--scene <scene>`, a compiled scene
  by identity or name), `point` (`--point <x>,<y>`, metres in the scene frame; the height is the floor's at that
  point, so none is given) and `mover` (`--mover <material>`, the probe sample's material, required with
  `scene`); the region under the point gives (substrate material, condition) and the triple (mover, substrate,
  condition) is resolved by the same lookup. A point outside the scene's floor or not two numbers, `scene`
  without `mover`, a scene position together with `card` (`--card` or `--interface`), and `--at` with `--scene`
  are `FROB-REF-802`.
  The branches, each enabled when the bead that wires it lands (before that the branch answers `FROB-REF-724`
  naming the branch and the bead, never a default):
  - *law* (M1 on the referee laws, M2 batched laws): the state above.
  - *cone* (M2, BD-154; selected with `--cone`, the spelling of GB-20 and N18a-36): N, the generalized sliding
    velocity and r_eff, with inline `mu`.
  - *film* (M3, BD-230; selected with `--film`): the film state, spelled `--h0 <thickness>` (the film thickness
    h₀ at the start of the dwell; `0` is the dry limit, which returns μ_d bit for bit), `--dwell <t>` (the dwell
    time t: the film is squeezed to h_f with 1/h_f² − 1/h₀² = 4·N·t/(3π·η·a⁴), and `0s` keeps h_f = h₀) and
    `--radius <a>` (the patch radius a), with `--normal` (N), `--slip` (the sliding speed) and the card's
    viscosity η, roughness and coefficients (`--card`, or the viscosity inline as `--eta`); for example N19b-06's
    squeezed state, `frob contact probe --film --card <card> --eta 1mPa.s --h0 1mm --dwell 0.2s --radius 20mm
    --normal 100N --slip 0.10m/s --json`. The squeeze form of GB-21 and N18a-37 asks instead for the time to a
    target thickness `until_h` (`--until-h`), with `eta`, `h0`, `radius` and `normal`: `frob contact probe
    --film --eta 1mPa.s --radius 20mm --normal 100N --h0 1mm --until-h 10um --json`.
  - *model* (M1, N18b-29; the reader of the records N18a-02, N18a-10 and N18a-11 store in the compiled model):
    `design` with exactly one of `samples`, `pairs` or `gap_budget`; `gap_budget` requires `scene` (in
    `sim.run`'s form) and `pose` (a fixture name, or a file of base pose and joint positions) and takes an
    optional `sample` (`<body/primitive/index>`). Two selectors at once, `gap_budget` without both `scene` and
    `pose`, or a model selector mixed with a contact-state field is `FROB-REF-802`.
  - *pulse* (M2, N18bx-01): `impulse` and `m_eff`, both required for a pulse probe (one without the other is
    `FROB-REF-802`). A contact-state field (gap, normal or tangential velocity, spin, normal load) beside the
    pair, or the pair with a selector (`--cone`, `--film`, `--design`) or with the attachment pair, is also
    `FROB-REF-802` (DESIGN, N18b-29: no state question silently becomes a pulse question).
  - *tyre* (M4, N18b-04; a form of the law branch): `--law tyre --card <tyre card>` with `normal`, `slip_ratio`
    (κ) and `tan_slip_angle` (tan α), both dimensionless; `sweep` over one of them.
  - *attachment* (M4, N15ax-02): `card` (an interface card with the group `attachment`), `force` and `moment` (the
    norms of the wrench at the grasp origin, N and N·m), `conservative` (flag: the lower interval ends). It reads
    no law, so `law`, `tier` and the state fields are ignored and not required; `force` without `moment`, or
    the reverse, is `FROB-REF-802`.
  - *held displacement* (M4, N18ax-01; a form of the law branch): `--law CL-1R --card <c>` with `radius` (the
    declared patch radius a, m), `hold` (the held indentation δ, m) and `at` (time since contact, s; a list for
    a history); `hold` without `at`, or the reverse, is `FROB-REF-802`.
  Soil is not a probe branch: it is `contact.soil`.
- Outputs: impulse components in N·s and forces in N as separate typed fields, regime (`open`, `stick`, `slip`,
  `spin-coupled`), μ_eff with the limit coefficient beside it, Hersey number and h/σ where a film is present,
  validity flags (`extrapolated`, `assumed-card`, `regularized`, `law-not-admitted`, and in the film branch
  `viscous-capped`), the kernel identities evaluated and the card identity; per branch:
  - cone: the tangential force, the spin moment and r_eff with its provenance; the projection of a trial impulse
    with the three projection identities;
  - film: the film regime (boundary, mixed, full film or dry; BD-230); the squeeze form returns the squeeze time
    from h0 to until_h (class `Estimated`, GB-21);
  - model: with `samples`, every sample sphere `{centre_local, radius, patch_radius, material}` with its
    provenance (`authored` or `generated`) and placement rule and the patch radius's source, then `static_max`
    beside the referee cap of 21 and each tier's contact capacity with the verdict; with `pairs`, the
    self-collision filter record (tested pairs, excluded pairs each with its reason `adjacent`, `same-link`,
    `clearance` with the smallest clearance and its table, or `authored` with the author's reason; counts by
    reason; the sweep axis or grid; the filter digest that enters `ModelId`); with `gap_budget`, per sample
    within the margin (or the named `sample`) the gap in N18a-02's form `{g_m, eps_g_m, terms_m{field, fit_a,
    fit_b, routine, registration}}`, the normal, the witnesses and `active_samples`. The model records are
    `exact` reads carried with the `ModelId`; a gap is always given with ε_g, never bare;
  - pulse: τ_c with its interval, F_peak and `extrapolated` when the impulse lies outside the card's
    `valid_impulse`;
  - tyre: F_x and F_y in N as separate typed fields, ζ, the regime `build-up` or `saturated`;
  - attachment: `holds`, the force and moment utilizations, the margin, the interaction rule used, the nominal
    or conservative limits and the evidence class;
  - held displacement: Fₙ(t)/Fₙ(0), the regime (the schedule segment) and validity;
  - from M6 (N18a-31), for a card with Stribeck fields: `stribeck_regime` (`static-dominated` when
    σ(v/v_str) ≥ 0.9, `kinetic` when σ ≤ 0.1, `transition` between) beside `regime`, which keeps its four
    values, with μ(v) as `μ_eff` and μ_s and μ_k beside it.
  Evidence: *law evaluation* (the film, pulse, tyre, Stribeck and assumed-field branches are `Estimated` unless
  every consumed field is `measured` or `datasheet`).
- Chapter refusals: `FROB-REF-240` (invalid cone: μ < 0, r_eff < 0 or a non-positive anisotropy; from M6 also a
  Stribeck card with μ_s < μ_k or v_str ≤ 0, N18a-31); `FROB-REF-242` (film law out of domain: Π or h/σ outside the
  card's calibrated range under `--strict-data`); `FROB-REF-243` (soil or tyre law out of range: Pressure, slip or
  sinkage outside the card's validity range; from M4 through the tyre form, N18b-04); from M2 `FROB-REF-102` and
  `FROB-REF-103` (a card loaded by the probe fails its container's form check: a unit or dimension error, or an
  unknown value or key, such as a pulse block with a non-positive duration; N18bx-01); from M4 `FROB-REF-121`
  (an attachment probe on a card without the attachment group, the pair named, N15ax-02; a `CL-1R` probe on a
  fixture without a patch radius, N18ax-01); from M4 `FROB-REF-246` (`--law CL-1R` on a card without a
  `relaxation` block, reason `card-invalid`, N18ax-01). A design that does not compile in the model branch
  returns the model compiler's own refusal unchanged. The missing-pair refusal has its code pending: ledger slug
  `interface-pair-missing` (family `2xx`, filed by N35x-02, numbered by N35-33's rule, entered by N18-REF; exit
  3, retry `after-change`, `observed` the triple, repairs "add a card for the pair and condition" and "choose a
  material that has one", `next_actions` the cards that exist for that mover material), raised from M2 for a
  well-formed `--interface <mover>/<substrate>` naming a pair with no card (key (mover, substrate, `dry`);
  pointer `/card`; BD-167) and from M3 for a scene position whose triple has no card (pointer `/point`;
  N18b-10), never with a default card and never as 802; it joins this list in the change that assigns its
  number.
- Implemented by: BD-167, N18b-29. Extended by: BD-154 (cone, M2), N18bx-01 (pulse, M2), BD-230 and GB-21
  (film and squeeze, M3), N18b-10 (the scene position, M3), N18b-36 (the generic `assumed` cards and their
  weakest-input pointer, M4), N18b-04 (tyre, M4), N15ax-02 (attachment, M4), N18ax-01 (`CL-1R`, M4), N18a-31
  (`stribeck_regime`, M6).

**`contact.capacity`** (MCP tool `contact_capacity`; owner `frob-contact`)
- Inputs: `design`; optional `step_plan` or shape class; optional `scenario_classes`; `with_reductions` (default
  true).
- Outputs: `model_id`; `referee_revision` and `referee_cap`; per scenario class the maxima, per-solver fit, headroom
  and response; the reductions; `next_actions`. Overflow is a finding, never a refusal: exit 2 when any reported
  class does not fit a solver it is compared against, exit 0 only when every class fits. Evidence: *static*.
- Chapter refusals: none of its own.
- Implemented by: BD-168, N18b-20.

**`contact.audit`** (MCP tool `contact_audit`; owner `frob-contact`)
- Inputs: `run` (a run or job identity) or `battery` with `tier` and `profile`; `p` (inclusion probability);
  `window`; `detector` (`referee` or `strict`); `scene_classes`; `seed`.
- Outputs: missed contacts per 10⁶ world-steps, by scene class, from certified collision detection; the misses
  (world address, step, pair, cause, entry and exit times, penetration depth, relative speed; paginated) with, for
  the ledger's refinement (N16b-21), the step at which the fast tier first registered the pair; per class the
  one-step-late certified contacts and the windows undecided on both routes; the detector and its revision; the
  share of windows that fell to `strict`. A window with no snapshot at its start is reached by replay and costed,
  never refused. Evidence: *known answer* (claim kind Fidelity, class Tested; D2 on the auditing back end).
- Chapter refusals: none of the Traction chapter's own. Raised by the handler from another registry, and so listed:
  `FROB-REF-222` (`detector: referee` or `strict` requested on a GPU tier; repair: run the detector on `batch-cpu`).
- Implemented by: BD-168, N18b-23.

**`contact.clearance`** (MCP tool `contact_clearance`; owner `frob-contact`)
- Inputs: a body (`design`, its collision primitives with their fitting errors, or explicit shapes); a `sweep`
  (poses at slab boundaries with the motion class inside each slab); obstacles (`scene` and `region`, or other
  bodies); whether a certificate is requested, `certificate` (`--certificate required|none`, default
  `required`; N19b-33's DESIGN spelling): with `none` no bundle is written, a `clear` answer carries class
  `Tested` and never `Certified`, and a request with no registered error bounds answers `unknown` naming the
  missing bounds instead of `FROB-REF-321`.
- Outputs: `clear` with a certificate (per slab and pair the separating data, written to a `.frobcert` bundle that
  the auditor re-checks offline; no bundle with `--certificate none`), `penetrates` with a witness, or `unknown`
  with the undecided slabs and their dominant error terms. Exit 0 for `clear`, 2 for `penetrates` or `unknown`.
  Evidence: *certificate* (`Certified` once the verifier has accepted the bundle, `Tested` before and always
  with `--certificate none`; D0).
- Chapter refusals: `FROB-REF-320` (thin feature: Scene geometry thinner than two fine voxels with no thin route);
  `FROB-REF-321` (no enclosure: A clearance certificate requested with no registered error bounds for a body or
  scene).
- Implemented by: BD-233, N19b-33.

**`contact.soil`** (MCP tool `contact_soil`; owner `frob-contact`)
- Inputs: `soil` (a soil card by name or hash); `load` (N); one running-gear form: `plate {width, pressure}`,
  `wheel {part | radius, width; slip}`, `track {length, width, count, slip}`, or `design {design, end_effector,
  load_case}` (patch geometry and static load share read from the compiled model); optional `slip_sweep` (at most
  1,024 points, paginated); optional `clearance` (a belly clearance to compare the sinkage with). Spellings
  (BD-366's table): `--soil <card>` (required); `--plate <width>@<pressure>` (`0.1m@20kPa`; no `--load` and no
  slip); `--wheel <part>` or `--wheel <radius>x<width>` (`w-75`, or `0.15x0.1` in metres); `--track
  <length>x<width>x<count>` (`0.5x0.1x2`); `--design <design> --end-effector <id> [--load-case <name>]` (load
  case default `static`); `--load <force>` (required with `--wheel` and `--track`; with `--design` it overrides
  the load share); `--slip <i>` (a fraction in [0, 1]; without `--slip` or `--slip-sweep` the answer is sinkage,
  pressure and the thrust limit, with no thrust at a slip); `--slip-sweep <from>:<to>:<points>`; `--clearance
  <length>`. Two running-gear forms, or none, is `FROB-REF-802` with the pointer to the second form.
- Outputs: sinkage (m), ground pressure (Pa), thrust (N) and its limit, the thrust curve for a sweep, drawbar pull
  where a rolling-resistance arm is declared, the finding "sinkage exceeds clearance", the regime report, the
  quadrature used, the card identity with provenance per field, `next_actions`. No value is clipped to the card's
  range. Evidence: *law evaluation*, always `Estimated` (a semi-empirical law), with tier label Frontier: the
  soft-soil model is Frontier although the row's tier is the appendix's S.
- Chapter refusals: `FROB-REF-243` (soil or tyre law out of range: Pressure, slip or sinkage outside the card's
  validity range).
- Implemented by: BD-366 (the spellings above are its DESIGN).

**`contact.response`** (MCP tool `contact_response`; owner `frob-contact`)
- Inputs (BD-367's spellings and defaults): `<run>` and `--world <address>` (required unless `--anchor` is
  given); `--step <k>` (default the last committed step of that world that converged and is not `Degraded`,
  N18b-39's capture rule); `--anchor <id>` (an anchor digest: the fixed active set; default captured from (run,
  world, step) and returned); `--columns` (`<file.json>` of explicit vectors, `base-impulses`, `contact-rows` or
  `stencil:<file>`; default `base-impulses`: one unit generalized impulse per coordinate of the floating base,
  three of 1 N·s and three of 1 N·m·s, or one per actuated joint on a fixed base); `--precision <p>` (default
  the anchoring step's); `--on-stale refuse|refactor` (default `refactor`, N18b-39); `--lambda standard|tight`
  (default `standard`, the certificate on Â); `--describe` (print the anchor digest and its covered components
  without solving). An unknown column family or `on_stale` value is `FROB-REF-802`. The column files are path
  parameters, so the field rule applies.
- Outputs: the anchor digest; per column the solution, the attached bound, the cone slack, status `accepted` or
  `rerouted` (the mode guard) and the class; the rerouted share; λ_lo with its certificate reference; flop counts.
  Evidence: claim Known answer for the gate rows; a bound is `Certified` when λ_lo carries its certificate and
  `Estimated` otherwise; determinism D2.
- Chapter refusals: the stale-anchor refusal (a batch requested against an anchor other than the one it was
  factored for, with `on_stale = refuse`; exit 3; `observed` the requested anchor and the first differing
  component). Its code is pending: ledger slug `response-batch-stale-anchor` (family `2xx`, filed by N35x-02,
  numbered by N35-33's rule, entered by N18-REF), and it joins this list in the change that assigns it; BD-367
  does not ship before then. The capture refusal (an anchor requested at a step that did not converge or is
  flagged `Degraded`; exit 3; `observed` the step's final scaled gradient norm against the plan's tolerance, or
  the `Degraded` flag with its cause; repairs: anchor the last committed step of that world that converged and
  is not `Degraded`, the default of `--step`; re-run the step to convergence under a larger iteration budget)
  has its code pending the same way: ledger slug `response-anchor-unconverged` (family `2xx`, N18b-39; filed by
  N35x-02 and entered by N18-REF as the stale-anchor row is), and it joins this list in the change that assigns
  it.
- Implemented by: BD-367 (the verb), on N18b-39 (the anchor and its refusal), N18b-40 and N18b-41.

**Machine fields of each row.**

| Verb | Capabilities | Output schema | Task clusters | Example |
|---|---|---|---|---|
| `contact.laws` | `contact:read` | `frob.contact.laws.out/1` | `contact` | `frob contact laws --tier referee --json` |
| `contact.probe` | `contact:read` | `frob.contact.probe.out/1` | `contact` | `frob contact probe --law CL-1 --interface rubber-60a/tile --normal 80N --slip 0.02m/s --json` |
| `contact.capacity` | `contact:read` | `frob.contact.capacity.out/1` | `contact` | `frob contact capacity quad-a --json` |
| `contact.audit` | `contact:run` | `frob.contact.audit.out/1` | `contact` | `frob contact audit r:19e2 --key ca-1 --json` |
| `contact.clearance` | `contact:run` | `frob.contact.clearance.out/1` | `contact`, `places-check` | `frob contact clearance --design carrier-a --sweep sweep.json --scene kitchen-17 --region door-pass --key cc-1 --json` |
| `contact.soil` | `contact:read` | `frob.contact.soil.out/1` | `contact` | `frob contact soil --wheel w-75 --soil sand-dry --load 40N --json` |
| `contact.response` | `contact:run` | `frob.contact.response.out/1` | `contact` | `frob contact response r:19e2 --world 311 --key cr-1 --json` |

The remaining fields follow by rule: `input_schema` is `frob.<verb name>.in/1`; `refusals` is the chapter list of
the verb's block; `idempotent` is `true` for `read` and `admin` and `false` for `run` and `mutate` (which take an
idempotency key); `cost_model` is `static: …` for `read`, `mutate` and `admin` and the `--estimate` cost model for
`run`; a `run` row carries `wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }`; `owner` is the
crate of the block; `purpose` is the Purpose column; `paginated = true` on `contact.probe` (its sweep) and
`contact.soil` (its slip sweep). `paginated` is a `read` field (NJL-01), so `contact.audit`, a `run`, does not set
it: its list of misses is paged inside its stored result.

One entry in full (the others follow field for field):

```toml
[[verb]]
name = "contact.audit"
cluster = "contact"
kind = "run"
tier = "S"
capabilities = ["contact:run"]
input_schema = "frob.contact.audit.in/1"
output_schema = "frob.contact.audit.out/1"
refusals = ["FROB-REF-222"]
idempotent = false
cost_model = "estimated: `--estimate` returns CPU core-seconds, GPU seconds, memory and disk with a conformal interval"
first_milestone = "M2"
task_clusters = ["contact"]
examples = ["frob contact audit r:19e2 --key ca-1 --json"]
owner = "frob-contact"
purpose = "The tunnelling audit"
wait = { cli = "to-completion", mcp_ms = 5000, mcp_max_ms = 30000 }
```

**Conventions the rows follow.**

1. The MCP tool name is the verb name with `.` and `-` replaced by `_`. The output on the wire is one envelope
   `frob.envelope/1` with `result` specialized to the output schema; its `evidence` block is a claim product (the
   weakest class, tier and determinism class among the inputs).
2. The generator adds the common input properties of the kind, as NJL-01's kinds table lists them (`run`:
   `sync_wait_ms` from 0 to the row's `wait.mcp_max_ms`, 30,000, with MCP default 5,000; `idempotency_key`, CLI
   `--key`; `budget`; `estimate_only`, CLI `--estimate`; `on_cancel`, `leave` (the default: the job keeps
   running) or `cancel_job`; `envelope_scope`, CLI `--envelope-scope <gate:…|campaign:…|pretrain:…>`, the
   approved compute-envelope scope the job cites (program convention 14, owned by NJL-01); `mutate`:
   `idempotency_key` and `expected_parent`). `idempotency_key` is never in the JSON-schema `required` array: the
   annotation `x-frob-key: "required"` (`"optional"` on a `key_optional` entry) carries the rule, and the
   admission transaction refuses a missing key with 805 on both surfaces. The surface-wide refusals appended by
   kind: `FROB-REF-720` (scope missing), 723 (token invalid), 724 (not implemented in this build) and 802 (schema
   failure, including an unknown parameter) for every verb; 701 to 704 (budget and admission), 761 (above the
   compute envelope in force), 803 (key reused with different content) and 805 (missing key) for `run`; 725, 803,
   804 (stale parent) and 805 for `mutate`. The properties and codes driven by fields are those of NJL-01's
   variation table: `cursor`, `limit` and `follow` with 806 and 807 for a paginated entry; 808 (an integer outside
   ±(2⁵³ − 1)) for an entry whose generated input schema has an `int53` property at any depth, whatever its kind
   (every `run` entry has one, `sync_wait_ms`); `estimate_only` on a non-`run` entry with `estimable = true`; no
   805 for a `key_optional` entry; the `action` property, with each action's codes, for an entry with `actions`;
   790 for a non-empty `needs_groups`; 891 for an `artifact-input<F>`, `artifact-ref<F>` or
   `artifact-identity<F>` parameter, and 892 as well for `artifact-identity<F>`; 893 for a `foreign-file`
   parameter; 734 and 828 for a parameter that accepts a path to a file (`artifact-input<F>`,
   `artifact-identity<F>`, `foreign-file`). The later variations of that table (10 to 13) land with their beads
   and are read there, not copied here.
3. Exit codes: 0 done; 2 done with findings; 3 refused input (1xx to 5xx, and 8xx other than conflicts); 4 budget
   (70x and 761); 5 capability (72x, 73x); 6 device (6xx); 7 conflict (803, 804, 809); 8 accepted and still running; 9
   indeterminate; never 1 on purpose.
4. Rows are entered when the cluster is first registered (M1); each row's tool is added to the task-cluster
   tables its `task_clusters` names in the same change (NJL-05's staging rule). A verb whose handler is not in
   the build answers one envelope with `FROB-REF-724`, exit 5, on both surfaces, and `frob capabilities --json`
   lists it as absent; a reduced form that lands before the row's `first_milestone` answers and is listed as
   implemented with its form named (NJL-01, NJL-03: the field keeps the appendix's milestone).

**Notes for this cluster.**

- Seven verbs. A law requested on a tier or profile where its gate has not passed is refused (`FROB-REF-246`).
- Branches, forms and codes that land after a row's first milestone are listed in the blocks with their
  milestone. Each option enters the generated input schema when its extending bead lands (an additive change that
  keeps the major version); before that a call that passes one is an unknown parameter, `FROB-REF-802`, except a
  branch whose fields `contact.probe` declares from M1 (law, cone, film and model, N18b-29), which answers
  `FROB-REF-724` naming the branch until its wiring bead lands. A listed code that is not yet raised is legal.
- Path parameters: `card` of `contact.probe` (its `.fcard` file form) and `pose` (its file form), and
  `--columns` of `contact.response` (its file and stencil forms), accept a path to a file, and so does `sweep`
  of `contact.clearance` (`sweep.json` in its example); the generator therefore appends 734 and 828 to those
  three rows (NJL-01's field rule) and to no other row of the cluster.
- `contact.clearance` with `--certificate required` (the default) returns `clear` only with a certificate;
  without registered error bounds it refuses a certificate and offers `unknown` (`FROB-REF-321`, whose second
  repair is the same call with `--certificate none`, which answers `unknown` naming the missing bounds, exit 2).

## Acceptance criteria

- [ ] `registries/verbs.toml` holds exactly seven entries with `cluster = "contact"` (4 `read`, 3 `run`), each with
      every field of the entry schema, an example and a task-cluster list; the alias lint finds no second spelling.
- [ ] `frob-surface-diff parity --cluster contact` exits 0 in process (`frob-mcp` over stdio) and through `frobd
      --mcp`: registry, CLI and MCP each count 7, and every difference list is empty apart from the harness's closed
      exemptions.
- [ ] Every verb has an end-to-end script: `scripts/e2e/verbs_contact.sh` runs each verb of the cluster (seven), and
      from a verb's first milestone a capability script of its implementing bead exists (`frob-surface-diff
      e2e-coverage`).
- [ ] `frob <verb> --help --json` equals `fixtures/golden/verbs/contact/<verb>.help.json` byte for byte for each of
      the seven, on two runs, debug and release, Linux and macOS; a planted renamed output property is classified
      `breaking` and refused.
- [ ] A verb not yet implemented answers `FROB-REF-724`, exit 5, with one envelope on both surfaces; an implemented
      verb runs its registry example with the result its implementing bead specifies; a misspelt parameter is
      `FROB-REF-802`, exit 3; a `run` or `mutate` call without a key is `FROB-REF-805`, exit 3.
- [ ] The generated refusal table lists for each verb exactly its chapter codes plus the codes appended for its
      kind and fields (NJL-01), and every code exists in `registries/refusals.toml`.
- [ ] The gate row is recorded with the measured counts by a session that did not write the rows.

## Test plan

### Unit tests

`cargo test -p frob-verbs --test cluster_contact` (the real generated table; no mocks; planted registries are TOML
strings labelled `synthetic`):
- `rows_present`: exactly 7 rows in the cluster, with the names of the rows table; a planted extra row or a removed
  row fails.
- `kinds_milestones_tiers`: each row's kind, first milestone and tier equal the rows table (for example
  `contact.response` is `run` at M4).
- `schemas_and_scopes`: input schemas are `frob.<name>.in/1`; output schemas and capabilities equal the
  machine-fields table.
- `refusal_lists`: each row's chapter codes equal its block (`contact.laws`: 246; `contact.probe`: 102, 103, 121,
  240, 242, 243, 246, and the `interface-pair-missing` code once assigned; `contact.capacity`: none;
  `contact.audit`: 222; `contact.clearance`: 320, 321; `contact.soil`: 243; `contact.response`: none until the
  stale-anchor and capture codes are assigned, then those codes); the appended codes equal the rule for its kind
  and fields, with 806 and 807 on `contact.probe` and `contact.soil` only, 808 on every row whose generated
  input schema has an `int53` property (the three `run` rows through `sync_wait_ms`, `contact.probe` and
  `contact.soil` through `limit`), and 734 and 828 on `contact.probe`, `contact.clearance` and
  `contact.response` only.
- `mcp_names_one_to_one`: the tool names are the mapped names (`contact_laws`, `contact_probe`, `contact_capacity`,
  `contact_audit`, `contact_clearance`, `contact_soil`, `contact_response`), with no collision against any other
  cluster.
- `probe_and_soil_inputs`: the generated `contact.probe` input schema has `law` with the CL rows spelled as the
  law table spells them (`--law cl-1` and `--law cl-h` are 802), `sweep`, and the pair form of `card`
  (`--interface rubber-60a` and `--interface rubber-60a/` are 802); the model branch's `design` with exactly
  one of `samples`, `pairs`, `gap_budget` (a schema `oneOf`), and `scene`, `pose` and the optional `sample`
  beside `gap_budget`; the planted calls
  `--design quad-a --samples --pairs`, `--design quad-a --gap-budget --scene s` (no pose) and `--design quad-a
  --samples --normal 80N` are each 802; the output schema has the sample list with provenance and rule,
  `static_max`, the filter record (tested and excluded pairs with reasons, smallest clearance, filter digest) and
  the five-term gap form. The generated `contact.soil` input schema has `soil`, `load` and exactly one of `plate`,
  `wheel`, `track`, `design` (a schema `oneOf`), so the registry example with `--wheel` and the implementing
  bead's example with `--track` both validate; every spelling of BD-366's table parses. The `contact.probe`
  schema has from M1 the selector `film` and the film state N18b-29 declares (`h0`, `dwell` and the patch
  `radius`), and `--film --h0 1mm --dwell 0.2s --radius 20mm --normal 100N --slip 0.10m/s` answers 724 naming
  BD-230 until that bead wires the branch. The generated `contact.clearance` input schema has `certificate` with
  the values `required` (the default) and `none`.
- `late_branches`: once the extending beads land, the generated `contact.probe` schema gains the pulse inputs
  (`impulse`, `m_eff`; N18bx-01), the squeeze inputs (`eta`, `until_h`; GB-21), the scene position (`scene`,
  `point`, `mover`; N18b-10), the tyre inputs (`slip_ratio`, `tan_slip_angle`; N18b-04), the attachment inputs
  (`force`, `moment`, `conservative`; N15ax-02), the held-displacement inputs (`hold`, `at`, with the film
  state's `radius`; N18ax-01) and the output field
  `stribeck_regime` (N18a-31), each classified additive by the snapshot check; the planted calls `--impulse 1N.s`
  without `--m-eff`, `--impulse 1N.s --m-eff 2kg --normal 80N`, `--scene s --point 1,2` without `--mover`,
  `--scene s --point 1,2 --mover rubber-60a --card c`, `--scene s --point 1,2 --mover rubber-60a --at 0.1s`
  and `--law cl-1r` are each 802; `contact.laws` gains `scene`; `contact.response` has the defaults of BD-367's
  table, so its registry example parses with `columns = base-impulses`.
- `task_clusters_and_examples`: every row sits in a task cluster of at most seven tools or is a base tool; each
  example parses against the generated CLI and its command path is the row's own.
- `kind_behaviour`: a `run` row has the six common properties of its kind (`sync_wait_ms`, `idempotency_key`
  with `x-frob-key: "required"` and absent from the schema's `required` array, `budget`, `estimate_only`,
  `on_cancel` with the values `leave` and `cancel_job`, `envelope_scope`) and refuses `sync_wait_ms = 30001`
  with code 802; a verb whose capability is absent returns code 724, exit 5, one envelope.

### End-to-end script and logging

`scripts/e2e/verbs_contact.sh` (header: `bead=VERBS-contact milestone=M1 labels=tether,verbs,verb:contact.*
gates=GATE-TETHER-02 seed=1 needs=cpu,mcp,frobd plants=alias,missing-row,renamed-field`) sources the shared helper
and `scripts/e2e/verbs_lib.sh` and calls `verbs_cluster_conformance contact`. Steps: (1) `frob capabilities --json`
and `frob robot-docs --json` list the seven verbs with kind and milestone; (2) `frob-surface-diff parity --cluster
contact` over stdio and through `frobd --mcp`, every difference list empty; (3) `frob-surface-diff snapshot --check
--cluster contact`; (4) each verb's registry example verbatim, expecting a result (exit 0 or 2), a job handle (exit
8, then `frob job wait`) or `FROB-REF-724` exactly when the capability report says the verb is absent; (5) for each
implemented verb the same call as an MCP tool, the two envelopes agreeing on `verb`, `ok`, `exit`, `result`,
`refusal`, `evidence`, `bound`, `warnings` and `next_actions`; (6) per verb one call with a misspelt parameter
(`e2e_expect_refusal FROB-REF-802 3`) and, for `run` and `mutate` verbs, one without a key (`e2e_expect_refusal
FROB-REF-805 3`); (7) one envelope per call and no exit code 1. Examples that name a file (`sweep.json` of
`contact.clearance`) run in `fixtures/verbs/contact/`, which holds those files.

Log: `target/e2e-logs/verbs_contact/<utc>.ndjson` with a transcript beside it. Each `cmd` line has the exact
command, exit code, wall time and the envelope's `evidence` block; each `assert` has observed and expected values
and the tag `--gate GATE-TETHER-02`; the metrics `registry_verbs`, `cli_verbs` and `mcp_tools` are logged against 7
and the difference counts against 0; refusals are logged with code and first repair; a failure logs the first
differing verb and field; the `summary` line gives assertions passed and failed and the gate touched.

### Known answers and negative controls

- Known answers: the cluster has 7 verbs (4 `read`, 3 `run`); the tables above. KA-755 (the exit-code table on
  malformed, unknown, over-budget, unscoped and conflicting inputs gives the exit and refusal code of the surface's
  tables; exact) is exercised for every verb by steps (6) and (7).
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

- Evidence labels follow the implementing beads (DESIGN, with the reason): an earlier draft labelled `contact.probe`
  and `contact.soil` *static* (Tested, exact) and `contact.response` *screening*. The probe evaluates semi-empirical
  laws and `assumed` cards, which are `Estimated` (N18b-29, N18b-36); the soil model is a semi-empirical Frontier
  model, always `Estimated` (BD-366); the response batch carries certified or estimated bounds and known answers
  (BD-367). The registry's evidence text must not promise a class the handler does not give.
- The registry example of `contact.clearance` names its sweep and region: a clearance query without a sweep has
  no body motion to certify, and the earlier example (`--design quad-a --scene s:kitchen-17`) would have been
  refused by the schema. The example is N19b-33's own, with the key every `run` verb takes.
- Not taken into the rows: a contact state read from a file (`frob contact probe --state <file>`). No
  implementing bead defines such an input; a film state is given with the film branch's own fields (`--h0`,
  `--dwell`, `--radius`, `--normal`, `--slip`, `--card`, `--eta`), as N19b-06's film-patch script gives its
  dry, squeezed and thick-film states, and the squeeze time with GB-21's `--until-h`.
- Tier of the late forms: the tyre, attachment, held-displacement and Stribeck forms belong to the Solid probe;
  the laws they evaluate carry their own tiers and kill gates in their beads (KILL-SOIL, KILL-FILM and the
  admission table), which `contact.laws` reports, and a law not admitted on a tier is evaluated with the flag
  `law-not-admitted`, never refused by the probe.
- Closing rule: the bead closes at M1, when the rows are registered, the surfaces match and every verb passes its
  example or answers the not-implemented refusal. Verbs of this cluster first answer at M1, M2, M3, M4. The
  per-milestone surface beads (BD-137, BD-222, BD-312, BD-376) re-run this script as verbs land; that is how
  GATE-TETHER-02 is re-measured.
- Scope names are proposals (`<cluster>:<read|run|write|admin>` plus the named scopes of the surface) until the
  capability bead BD-258 fixes the scope registry; a `run` or `write` scope of a cluster implies its `read` scope.
  Inputs and outputs above name contents, not flag spellings, except where a flag is quoted; the implementing bead
  fixes the spellings and the snapshot then pins them (additive changes keep the schema's major version; anything
  else raises it).

## References

Appendix J (J.1 entry form; J.3 added rows; J.5 task clusters), §18.4, §18.13, §18.14, §18.19, §32.4, §32.6,
§32.12.3, §32.18, §32.21; GATE-TETHER-02; INV-TETHER-01, -02; KA-755; the inputs' sources N18b-29, N18a-02,
N18a-10, N18a-11, BD-154, BD-167, BD-230, GB-21, N18a-37, N18b-10, N18bx-01, N18b-04, N15ax-02, N18ax-01,
N18a-31, BD-366, BD-367, N18b-39, N19b-06, N19b-33.
