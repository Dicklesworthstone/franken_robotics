## Appendix O. Refusal code registry

Every refusal has a stable code, a message and ranked repairs (§35). This registry is compiled from the chapters
that define the codes; the chapter governs the meaning and the repairs. The machine-readable form is
`registries/refusals.toml`, and an unknown code is a load-time error. Codes are never reused.

### O.1 `1xx` Design and model

| Code | Meaning | Defined in |
|---|---|---|
| FROB-REF-101 | Port type mismatch; joint class and screw basis disagree | §14.13 |
| FROB-REF-102 | Unit or dimension error | §14.13 |
| FROB-REF-103 | Unknown, misspelt or duplicate field | §14.13 |
| FROB-REF-104 | Inertia not physically consistent (the smallest eigenvalue of the pseudo-inertia is reported) | §14.13 |
| FROB-REF-105 | A body without a fill; a solid written as a shell | §14.13 |
| FROB-REF-106 | Declared mass properties violate the mass-support bound | §14.13 |
| FROB-REF-107 | Empty joint range | §14.13 |
| FROB-REF-108 | Power sign mismatch, copied power port or algebraic loop | §14.13 |
| FROB-REF-109 | Declared symmetry differs from computed symmetry | §14.13 |
| FROB-REF-110 | A closure that over-constrains the mechanism (mobility below one) | §14.13 |
| FROB-REF-111 | A lever outside its bounds; an edit to a locked field | §14.13 |
| FROB-REF-112 | Stale parent in a patch | §14.13 |
| FROB-REF-113 | An actuator card with a required field of class `assumed` when the caller asked for `--strict-data` | §14.13 |
| FROB-REF-114 | Module digest or port signature mismatch on import or substitution | §14.13 |
| FROB-REF-115 | A design budget exceeded: mass, cost, peak power, energy or envelope | §14.13 |
| FROB-REF-116 | Co-design does not close: loop gain at or above one | §14.13 |
| FROB-REF-118 | The base cannot produce a motion the task class requires | §14.13 |
| FROB-REF-119 | No admissible route or reach in the scene | §14.13 |
| FROB-REF-120 | Observation alias under every admitted probe | §14.13 |
| FROB-REF-121 | Contact law missing or incompatible with its end effector | §14.13 |
| FROB-REF-122 | An unsupported element in an import under `--strict` | §14.13 |
| FROB-REF-123 | Self-collision or interpenetration at the reference pose or over declared ranges | §14.13 |
| FROB-REF-124 | A static margin the task requires is not met | §14.13 |
| FROB-REF-125 | Unsupported schema version | §14.13 |
| FROB-REF-126 | A sensor without a parent frame, a rate or a clock | §14.13 |
| FROB-REF-130 | A card or transmission that is not passive: loss negative in some quadrant, efficiency above one, or torque and back-EMF constants inconsistent beyond the stated uncertainty | §15.13 |
| FROB-REF-131 | A required operating point outside the torque, speed and voltage envelope | §15.13 |
| FROB-REF-132 | No stable thermal equilibrium at a required operating point, or an equilibrium above a limit | §15.13 |
| FROB-REF-133 | Peak demand above the bus ceiling E²/(4·R) | §15.13 |
| FROB-REF-134 | A card without a current convention, or with conventions that disagree | §15.13 |
| FROB-REF-135 | Power requested from an exhausted battery, or state of charge driven outside its bounds | §15.13 |
| FROB-REF-136 | A row option the requested tier does not carry | §15.13 |
| FROB-REF-160 | No shape class: A design exceeds the largest admitted bucket and the tree-operator route is off | §17.16 |
| FROB-REF-161 | Group not observation-preserving: A quotient or a weight tie is requested under a group that moves a sensor, the payload or the task | §17.16 |
| FROB-REF-162 | Reuse without a dependency digest: A cached result has no recorded dependency digest, or the shadow recompute disagreed | §17.16 |
| FROB-REF-163 | A check whose material card lacks the field or provenance it needs | §31.16 |
| FROB-REF-164 | An empty design region, or ports with no load path inside the envelope | §31.16 |
| FROB-REF-165 | An allocation with a zero or unknown sensitivity, or a failed band-extreme check | §31.16 |
| FROB-REF-166 | Optimization or packet export for a design with no qualification verdict | §31.16 |
| FROB-REF-167 | A load case that is not one instant, or a campaign failing the free-body check | §31.16 |
| FROB-REF-168 | A solid body exceeds the mass plausibility bound of its part class and no fill is declared (§11.5). | §12.7 |
| FROB-REF-169 | Telemetry without the traces or samples a load metric needs | §31.16 |

### O.2 `2xx` Numerics and integrator admission

| Code | Meaning | Defined in |
|---|---|---|
| FROB-REF-212 | A Cayley update with ‖ω‖·h above 0.2 in one substep | §13.9 |
| FROB-REF-213 | A logarithm or its derivative inside the stated margin of a chart cut, with no second chart supplied | §13.9 |
| FROB-REF-214 | A representation with a constituent of quaternionic type, or a symmetry group outside the supported families | §13.9 |
| FROB-REF-215 | An exact evaluation whose bit budget exceeds its cap: a guarded predicate over 256 bits at build time, or a host computation over its declared rational size | §13.9 |
| FROB-REF-220 | Event refinement limit: An event cannot be bracketed within L_max, or more than E_max events fall in one physics interval | §16.14 |
| FROB-REF-221 | Transaction exhausted: Every numerical plan of the retry ladder failed its checks | §16.14 |
| FROB-REF-222 | Profile not available on tier: `strict` or `referee` requested on a GPU tier | §16.14 |
| FROB-REF-223 | No refinement law: A stochastic driver without a registered refinement law under a transactional profile | §16.14 |
| FROB-REF-224 | Indeterminate contact: No solution, or several distinct post-event motions | §16.14 |
| FROB-REF-240 | Invalid cone: μ < 0, r_eff < 0 or a non-positive anisotropy | §18.14 |
| FROB-REF-241 | No rotor state: Backlash rows requested at a card level without a rotor degree of freedom | §18.14 |
| FROB-REF-242 | Film law out of domain: Π or h/σ outside the card's calibrated range under `--strict-data` | §18.14 |
| FROB-REF-243 | Soil or tyre law out of range: Pressure, slip or sinkage outside the card's validity range | §18.14 |
| FROB-REF-244 | Stiffness beyond single precision: Estimated scaled κ·u above the admission bound on an `f32` tier | §18.14 |
| FROB-REF-245 | Half precision: A half-precision solve or state buffer is requested | §18.14 |
| FROB-REF-246 | Law not admitted: A law requested on a tier or profile where its gate has not passed | §18.14 |
| FROB-REF-247 | The determinism class a verb requires is not attainable on the requested placement | §35.11 |
| FROB-REF-248 | A solve is ill-conditioned beyond its precision profile, under the `refuse` policy | §35.11 |
| FROB-REF-249 | An anchored offset left its range in one step, or a cell index left the scene bounds | §35.11 |
| FROB-REF-250 | A kernel is not dimensionally homogeneous (build time) | §35.11 |
| FROB-REF-251 | A banned operation appears in kernel source (build time) | §35.11 |
| FROB-REF-252 | A strict predicate in floating point without an exact form or a certified guard (build time) | §35.11 |
| FROB-REF-260 | A loads–mass loop that diverges, cycles or exceeds its iteration budget | §31.16 |
| FROB-REF-261 | An optimized part that fails the independent check | §31.16 |
| FROB-REF-262 | Quasi-static superposition where the frequency separation fails | §31.16 |
| FROB-REF-263 | A derivative is requested near grazing, at an ambiguous event order, across a change of active set or at frictional degeneracy (C-16). The reply carries the offending state and directions. | §12.7 |

### O.3 `3xx` Scene

| Code | Meaning | Defined in |
|---|---|---|
| FROB-REF-301 | Source geometry is not watertight or not consistently oriented | §19.16 |
| FROB-REF-302 | The transform class of an import is not established: a reflection or affine map is not excluded, or winding disagrees with the determinant | §19.16 |
| FROB-REF-303 | A decode limit is exceeded or a reference does not resolve | §19.16 |
| FROB-REF-304 | Scale authority is below what the task requires | §19.16 |
| FROB-REF-305 | A required route, spawn or contact lies in an unknown region | §19.16 |
| FROB-REF-306 | A feature below voxel scale has no overlay, or an overlay index cell is over capacity | §19.16 |
| FROB-REF-307 | A film is outside the validity of the film model | §19.16 |
| FROB-REF-308 | A clearance query is outside the domain of the complex: the footprint changes shape along the route, a height or step capability is missing, or a kinematic label is requested for a base that cannot reverse | §19.16 |
| FROB-REF-309 | A run is requested on a route that has a standing impossibility cut for this footprint | §19.16 |
| FROB-REF-310 | A pack or report would export a research-only or private asset | §19.16 |
| FROB-REF-311 | A scene group straddles a split: Move the whole group | §19.16 |
| FROB-REF-312 | A realism claim cites a scene with `assumed` dimensions | §19.16 |
| FROB-REF-313 | A retrofit violates a constraint or an install class the mission does not permit | §19.16 |
| FROB-REF-314 | World-bank admission would exceed the device cap | §19.16 |
| FROB-REF-315 | A movable object has no declared behaviour | §19.16 |
| FROB-REF-316 | A surface region has no optical card | §19.16 |
| FROB-REF-317 | Collision-to-visual deviation exceeds the pack threshold with no waiver | §19.16 |
| FROB-REF-320 | Thin feature: Scene geometry thinner than two fine voxels with no thin route | §18.14 |
| FROB-REF-321 | No enclosure: A clearance certificate requested with no registered error bounds for a body or scene | §18.14 |
| FROB-REF-360 | A world or edit lies outside the legality profile; the violated bound is named | §26.12 |
| FROB-REF-361 | The request demands `cited` bounds and the profile has an `assumed` bound on a touched quantity | §26.12 |
| FROB-REF-362 | An edit does not declare the causal paths it changes | §26.12 |

### O.4 `4xx` Control and estimation

| Code | Meaning | Defined in |
|---|---|---|
| FROB-REF-421 | No synthesis route exists for the detected morphology class and no fallback is admitted | §22.14 |
| FROB-REF-424 | A control program is infeasible and the controller has no admissible fallback | §22.14 |
| FROB-REF-431 | A gait derivative was requested at grazing or at a fold | §22.14 |
| FROB-REF-440 | The clock lattice is not admissible: a period is zero or negative, or the common denominator exceeds the limit | §20.18 |
| FROB-REF-441 | History buffers cannot cover the declared latency and exposure window | §20.18 |
| FROB-REF-442 | An observation is requested from presentation state, or an actor schema names a privileged channel | §20.18 |
| FROB-REF-443 | A symmetry consumer names a group element outside 𝔾_obs | §20.18 |
| FROB-REF-444 | A transported skill does not refine its specification | §22.14 |
| FROB-REF-445 | A training or optimization run is requested on a mission with a standing alias witness for its interface | §20.18 |
| FROB-REF-446 | A calibration request exceeds what the graph supports: no anchor, or fault localization on a graph without the needed cycles | §20.18 |
| FROB-REF-447 | A Reality claim rests on a sensor or optical card field of class `assumed` | §20.18 |
| FROB-REF-448 | A skill's task wrench lies outside the body's attainable set | §22.14 |
| FROB-REF-451 | An energy budget omits a source in the port graph, or a class above `Estimated` is requested on an `assumed` absorber envelope | §22.14 |
| FROB-REF-454 | A shield's rows have no common admissible command on the calibration set, or more than four are coupled | §22.14 |
| FROB-REF-457 | A shield margin is used outside its scope key | §22.14 |
| FROB-REF-461 | A symmetric reduction was requested under a group not contained in Γ_use | §22.14 |
| FROB-REF-464 | An estimator was scored with a fitted scale, or an estimated-state run lacks a sensor the filter needs | §22.14 |

### O.5 `5xx` Learning and evaluation protocol

| Code | Meaning | Defined in |
|---|---|---|
| FROB-REF-502 | An evaluation reuses a scene from the policy's training manifest (carried from §19) | §24.14 |
| FROB-REF-520 | An atom has no scale and is not declared exact | §24.14 |
| FROB-REF-521 | `step` on a world that needs reset, in explicit mode | §23.19 |
| FROB-REF-522 | A non-finite action, before any state changes | §23.19 |
| FROB-REF-523 | A future-time operator appears in an online signal | §24.14 |
| FROB-REF-524 | The actor schema reads a `critic-only` or `evaluation-only` channel | §23.19 |
| FROB-REF-525 | A signal is read across clocks without a declared reduction, or its channel kind does not admit the reduction | §24.14 |
| FROB-REF-526 | The episode horizon is shorter than the reach of the verdict | §24.14 |
| FROB-REF-527 | The action law at learning differs from the law at collection | §23.19 |
| FROB-REF-528 | An online window exceeds its capacity | §24.14 |
| FROB-REF-529 | No settled-pose cache for this model and task | §24.14 |
| FROB-REF-530 | A run offered as a fixture run does not match the fixture's recorded hash | §24.14 |
| FROB-REF-531 | A policy's mapping table does not match the design, or cannot be built for an edit | §23.19 |
| FROB-REF-532 | A fixed-n study has no pre-analysis, or names a verdict it cannot reach | §24.14 |
| FROB-REF-533 | Paired statistics requested under an uncontrolled determinism class | §24.14 |
| FROB-REF-534 | A family policy applied outside its class or its pretraining distribution | §23.19 |
| FROB-REF-535 | Dimension mismatch inside an atom | §24.14 |
| FROB-REF-536 | A temporal operator has no finite window | §24.14 |
| FROB-REF-537 | A rung used to select or discard without a rank-agreement record that meets its threshold | §23.19 |
| FROB-REF-538 | A window bound is not a multiple of the clock period and no rounding is declared | §24.14 |
| FROB-REF-539 | A requirement has no labelled examples, or the monitor disagrees with a label | §24.14 |
| FROB-REF-540 | Gradient unreliable: Non-smooth metric, horizon above the admitted cap, pairs diverged early, or an uncontrolled determinism class | §17.16 |
| FROB-REF-541 | Outside generator support: A family-policy score is requested for a design outside its training generator's support | §17.16 |
| FROB-REF-542 | A paired study under an uncontrolled determinism class, or with arms in different classes | §28.20 |
| FROB-REF-543 | An `anytime` study on a metric with no declared span | §28.20 |
| FROB-REF-544 | A training gate record without its negative controls | §23.19 |
| FROB-REF-545 | A rung asked to select in a stratum whose mode is not `selective` | §28.20 |
| FROB-REF-546 | Calibration from survivors, or an audit episode cancelled or re-weighted | §28.20 |
| FROB-REF-547 | A tensor view the receiver cannot represent, under a zero-copy contract | §23.19 |
| FROB-REF-548 | Pooling evidence or calibration across two model identities | §28.20 |
| FROB-REF-549 | A rare-failure bound over battery-selected or search-selected episodes | §28.20 |
| FROB-REF-550 | `first-change` asked of bisect without a prefix scan or a declared monotone predicate | §28.20 |
| FROB-REF-551 | An exclusion from an analysis whose model does not enclose a declared state or energy source | §28.20 |
| FROB-REF-552 | A request mixes study types | §24.14 |
| FROB-REF-553 | A comparison that mixes study types, or re-reads fixed-n data as a sequential study | §28.20 |
| FROB-REF-554 | A future-time requirement term offered as an online reward | §23.19 |
| FROB-REF-555 | A fixed-n study whose declared verdict set contains an unreachable verdict | §28.20 |
| FROB-REF-556 | A pairing keyed by event order or by a training counter | §28.20 |
| FROB-REF-557 | A `model-conditional` result offered as a design comparison without its condition, or as input to a build packet | §28.20 |
| FROB-REF-560 | A bound was offered episodes that were not drawn nominally or with recorded bounded weights | §26.12 |
| FROB-REF-561 | A paired statistic was requested with an arm of class D4 or of unrecorded class, or as paired across device families without class D1 | §35.11 |
| FROB-REF-562 | Attribution or a counterfactual fork was requested below class D2 | §35.11 |
| FROB-REF-563 | Importance sampling outside its admitted law, or with a proposal fitted on the run it is applied to | §26.12 |
| FROB-REF-564 | Shrinking requested for a failure that does not replay, or that the guard labelled an artifact candidate | §26.12 |
| FROB-REF-565 | Coverage selection on a stale incidence | §26.12 |
| FROB-REF-566 | A planted winner's rank is not on record from a full evaluation | §26.12 |
| FROB-REF-567 | A hard-world or search-selected world offered as a confirmation sample | §26.12 |
| FROB-REF-568 | The failure predicate changed after sampling began | §26.12 |
| FROB-REF-569 | Splitting requested where no randomness enters after the fork | §26.12 |
| FROB-REF-580 | A continuation whose physical, clock, sensor or policy contract does not match the checkpoint | §27.14 |
| FROB-REF-581 | A structural or design-parameter intervention requested at a mid-episode tick; the earliest valid tick and the forcing fields are returned | §27.14 |
| FROB-REF-582 | State from one model grafted onto another | §27.14 |
| FROB-REF-583 | The null fork does not reproduce its parent | §27.14 |
| FROB-REF-584 | Attribution requested for a failure that is not reproduced in double precision, or under an uncontrolled determinism class | §27.14 |
| FROB-REF-586 | An intervention changes more than one declared mechanism | §27.14 |
| FROB-REF-587 | A design-parameter re-run with a policy that cannot act on the edited body without retraining | §27.14 |

### O.6 `6xx` Backend and device

| Code | Meaning | Defined in |
|---|---|---|
| FROB-REF-601 | Device unavailable or lost: Rerun on another device, or on `batch-cpu` under `mixed-allowed` | §21.21 |
| FROB-REF-602 | The memory estimate exceeds the device budget at admission | §21.21 |
| FROB-REF-603 | Kernel compile failure; compiler diagnostics verbatim | §21.21 |
| FROB-REF-604 | Strict backend: a stage has no kernel on the requested backend | §21.21 |
| FROB-REF-605 | The CPU-versus-device check is not current for a kernel in the plan | §21.21 |
| FROB-REF-606 | Fingerprint missing, or changed since admission | §21.21 |
| FROB-REF-607 | A launch or command buffer failed after bounded retries | §21.21 |
| FROB-REF-608 | Worlds marked `Faulted` exceed the caller's threshold | §21.21 |
| FROB-REF-609 | A double-float canary failed: Run without double-float kernels, labeled; file the compiler evidence | §21.21 |
| FROB-REF-610 | Capacity overflow under the `refuse` policy | §21.21 |
| FROB-REF-611 | A shape change was requested inside a running environment | §21.21 |
| FROB-REF-612 | Kernel cache entry corrupt and recompilation unavailable | §21.21 |
| FROB-REF-613 | A specialized kernel is not bit-identical to its generic kernel | §21.21 |
| FROB-REF-614 | Host access to a slab outside its lease | §21.21 |
| FROB-REF-615 | Device stalled (watchdog): Restart the device worker | §21.21 |
| FROB-REF-616 | The device lacks an arithmetic feature the kernel declares (for example 64-bit integers) | §21.21 |
| FROB-REF-617 | A view cannot cross a stride convention without a copy, under a zero-copy contract | §21.21 |
| FROB-REF-618 | A transfer class that may not cross machines | §21.21 |

### O.7 `7xx` Budget and capability

| Code | Meaning | Defined in |
|---|---|---|
| FROB-REF-701 | The estimate's upper end exceeds the remaining budget; the estimate is attached | §32.18 |
| FROB-REF-702 | Budget exhausted during a run; stopped at a safe boundary; nothing partial published | §32.18 |
| FROB-REF-703 | Admission guard: device utilization or the per-environment job cap is reached; `retry: backoff` | §32.18 |
| FROB-REF-704 | A campaign-class job without an approved compute envelope | §32.18 |
| FROB-REF-720 | The session token lacks a required scope | §32.18 |
| FROB-REF-721 | Referee capacity: An island exceeds the referee's contact cap, or FrankenSim's fixed-witness gate refuses | §16.14 |
| FROB-REF-722 | Capacity overflow, refused: A world exceeds its contact or row capacity and the caller chose refuse | §16.14 |
| FROB-REF-723 | Token invalid or expired, or an attenuation that would widen scopes or budget | §32.18 |
| FROB-REF-724 | The verb is not implemented in the §0.2 sense in this build | §32.18 |
| FROB-REF-725 | A write outside the caller's namespace or lineage | §32.18 |
| FROB-REF-730 | A confirmation world or run is not readable in a discovery session | §37.12 |
| FROB-REF-731 | A private object, or anything derived from one, without the private scope and flag | §37.12 |
| FROB-REF-733 | The licence set of the artifact forbids this export | §37.12 |
| FROB-REF-734 | A path outside the session's roots | §37.12 |
| FROB-REF-740 | Population exceeds device: The population's bytes exceed every admitted placement | §17.16 |
| FROB-REF-741 | Bucket fragmentation: More than eight buckets are needed to hold padding waste at or below the limit | §17.16 |
| FROB-REF-760 | The episodes a claim needs at its level exceed the budget; the bound reachable within budget is attached | §26.12 |
| FROB-REF-761 | A job exceeds the compute envelope in force (§36.14) | §35.11 |
| FROB-REF-790 | `foundation-group-absent`: A verb needs an airlock feature group that this binary was not built with | §33.16 |
| FROB-REF-791 | `referee-worker-mismatch`: The worker is absent, or was built under a different stack receipt than the job names | §33.16 |

### O.8 `8xx` Evidence and protocol

| Code | Meaning | Defined in |
|---|---|---|
| FROB-REF-801 | Unknown verb (CLI) | §32.18 |
| FROB-REF-802 | Arguments fail the schema, including an unknown parameter or an out-of-range `sync_wait_ms` | §32.18 |
| FROB-REF-803 | Idempotency key reused with different resolved content | §32.18 |
| FROB-REF-804 | Stale parent: the expected parent is not the head of the lineage | §32.18 |
| FROB-REF-805 | A mutating or job-submitting verb without an idempotency key | §32.18 |
| FROB-REF-806 | Cursor does not match the query | §32.18 |
| FROB-REF-807 | Cursor expired: its frontier is no longer retained | §32.18 |
| FROB-REF-808 | An integer outside ±(2⁵³ − 1) in a number-typed field | §32.18 |
| FROB-REF-809 | Plan not found, or bound to an identity that has since been superseded | §32.18 |
| FROB-REF-810 | Chunk conflict: different bytes for a committed extent | §32.18 |
| FROB-REF-811 | A tier-corrected estimate across identities, with no audited address, or below the effective-sample floor | §29.17 |
| FROB-REF-812 | An as-built registration that is ill-posed or below its noise screen | §30.19 |
| FROB-REF-813 | A deletion that would remove a pinned object | §29.17 |
| FROB-REF-814 | Pooling across cells, across `ModelId`s, or of synthetic with real data | §30.19 |
| FROB-REF-815 | Closing a bead without a resolving log line from another session | §29.17 |
| FROB-REF-816 | An artifact whose authorization flags are missing or not `false` | §37.12 |
| FROB-REF-817 | Sampling-controller episodes were offered as the scoring source of acceptance, promotion or qualification | §22.14 |
| FROB-REF-818 | A priced parts list without a pinned, licensed catalogue | §31.16 |
| FROB-REF-819 | A result cited above its claim product | §37.12 |
| FROB-REF-820 | Label misuse: A result that is not `established` is cited as established, or a `model-conditional` ranking is used to authorize spend | §16.14 |
| FROB-REF-821 | Envelope not frozen: Promotion requested for a quantity whose envelope is not hashed in the ledger | §16.14 |
| FROB-REF-822 | Capacity-biased verdict: A `capacity-biased` comparison is cited as a verdict | §18.14 |
| FROB-REF-823 | Unknown proof kind. | §12.7 |
| FROB-REF-824 | A STEP export above its bounds or failing bit-exact replay | §31.16 |
| FROB-REF-825 | A result of class D4 offered as evidence | §37.12 |
| FROB-REF-826 | The quantifier prefix, domain or assumption set of a certificate does not match the query it is offered for. | §12.7 |
| FROB-REF-827 | A fixed-point obligation refuted or undecided within budget | §31.16 |
| FROB-REF-828 | A payload that carries code, or a weight format outside the allowlist | §37.12 |
| FROB-REF-829 | A side condition of a proof rule is missing, or a field has the wrong frame, origin, sign or unit. | §12.7 |
| FROB-REF-830 | A `model-conditional` result offered for promotion, build or prototype budget | §23.19 |
| FROB-REF-831 | A lifecycle law outside its validity domain, or cited above class Estimated | §31.16 |
| FROB-REF-832 | An import over its size, depth or polygon limit | §37.12 |
| FROB-REF-833 | A no-authorization flag is missing or true. | §12.7 |
| FROB-REF-834 | A result is presented above the class of its weakest input, or as `Proved` or `Certified` without a certificate hash and a checker identity. | §12.7 |
| FROB-REF-835 | A verdict is reused after a change without a recorded implication (D-11). | §12.7 |
| FROB-REF-840 | A session without `qual:admin` asks for sealed confirmation data before close | §24.14 |
| FROB-REF-841 | A confirmation request was not committed first, or differs from its commitment | §24.14 |
| FROB-REF-842 | The units the request needs at its level exceed its budget; the reachable verdicts are attached | §24.14 |
| FROB-REF-843 | The candidate's lineage contains data from the sealed family | §24.14 |
| FROB-REF-844 | A cluster would be used twice | §24.14 |
| FROB-REF-845 | A result is cited as qualified without a promoting receipt | §24.14 |
| FROB-REF-846 | A chance constraint lacks its population, event or confidence, or marginal stage rates are multiplied without a dependence assumption | §25.24 |
| FROB-REF-847 | An observation has no arrival instant, or a policy uses information before it arrives | §25.24 |
| FROB-REF-848 | A proof kind is unknown, at an unknown version, disabled, or used outside its scope | §25.24 |
| FROB-REF-849 | Certificate arithmetic is malformed: a non-reduced rational, a size limit exceeded, an interval without a declared rounding | §25.24 |
| FROB-REF-850 | A frame, unit, sign or port order in a certificate does not match its definitions (a first moment passed as a torque; reordered feet) | §25.24 |
| FROB-REF-851 | A certificate assumes an authority the design does not declare, or omits a force floor or other active bound | §25.24 |
| FROB-REF-852 | A monotone or symmetry transport lacks a side condition | §25.24 |
| FROB-REF-853 | A minimum is claimed without a matching lower bound, or an unattained infimum is reported as attained | §25.24 |
| FROB-REF-854 | A classifier, screen or certificate is reused outside its eligibility: support, deadline, error units or tolerance changed | §25.24 |
| FROB-REF-855 | Evidence profiles are mixed inside one cell, or a probability mass is requested without a supplied weighting | §25.24 |
| FROB-REF-856 | A region certificate is requested over more parameters than the admitted limit, or a joint paving is requested | §25.24 |
| FROB-REF-857 | A synthesis system is outside the symbolic caps or is not zero-dimensional | §25.24 |
| FROB-REF-858 | Stage composition fails: an exit set not inside the next entry set, a clock mismatch, or a resource spent twice | §25.24 |
| FROB-REF-859 | An experiment or world edit cannot be realized through the declared interfaces, or does not declare the causal path it changes | §25.24 |
| FROB-REF-860 | An export without the flags, or any attempt to produce a true flag | §31.16 |
| FROB-REF-861 | A number from a `fast-training` run was cited as evidence, or a policy trained under it lacks a stricter re-evaluation | §35.11 |
| FROB-REF-862 | A performance figure without a complete disclosure block (§36.7) | §35.11 |
| FROB-REF-863 | A performance ratio without a paired lower bound (§36.6) | §35.11 |
| FROB-REF-864 | A device result cited without a current CPU-versus-device check | §35.11 |
| FROB-REF-865 | A summary statistic that omits what it excluded | §35.11 |
| FROB-REF-866 | A speed experiment requested without its computed ceiling, or with a ceiling below its goal (§36.5) | §35.11 |
| FROB-REF-867 | A D4 training result cited without re-evaluation on a stricter profile | §23.19 |
| FROB-REF-868 | The `.fmission` file is not valid: unknown field, missing unit, misspelt parameter, number not an exact string | §25.24 |
| FROB-REF-869 | Citing a claim whose evidence closure is dangling or tombstoned | §29.17 |
| FROB-REF-870 | Reuse of a verdict with no licence, or under a rule the registry does not know | §28.20 |
| FROB-REF-871 | A decision-core table that is missing, stale against its contract, or not the registry version | §28.20 |
| FROB-REF-872 | A value-of-information output cited as a verdict | §28.20 |
| FROB-REF-873 | A dataset lacking licence, provenance, rights or measurement uncertainty | §30.19 |
| FROB-REF-874 | The world family is empty, or its scope changed after candidates were tested | §25.24 |
| FROB-REF-875 | A publish under a late lease: Re-acquire; replay from the last committed tick | §29.17 |
| FROB-REF-876 | Clock alignment under-determined, or a scored instant outside the bracket | §30.19 |
| FROB-REF-877 | The predicates conflict with the world family for every design | §25.24 |
| FROB-REF-878 | Different bytes for a committed chunk key | §29.17 |
| FROB-REF-879 | A fit on a parameter set that fails identifiability | §30.19 |
| FROB-REF-880 | An attribution cited as an established cause, or as a cause in the physical robot | §27.14 |
| FROB-REF-881 | Evidence or a certificate establishes a different quantifier prefix than the query's, including clairvoyant evidence offered for a robust claim | §25.24 |
| FROB-REF-882 | A fork that asks to reuse state its change set invalidates | §29.17 |
| FROB-REF-883 | A read of sealed data with no frozen prediction, or a prediction changed after freezing | §30.19 |
| FROB-REF-884 | No quantifier prefix, or a hidden parameter with no persistence declaration | §25.24 |
| FROB-REF-885 | A chunk that is not a partition extent, or whose predecessor is not the frontier | §29.17 |
| FROB-REF-886 | Reuse of a spent holdout: Acquire fresh sealed data | §30.19 |
| FROB-REF-887 | A policy is indexed by a parameter the controller cannot observe | §25.24 |
| FROB-REF-888 | A metric computed over missing samples | §29.17 |
| FROB-REF-889 | A band for an uncalibrated cell, or with too few calibration points | §30.19 |
| FROB-REF-890 | `stack-mismatch`: Evidence recorded under another stack receipt is offered as same-stack evidence without revalidation | §33.16 |
| FROB-REF-891 | `format-version-unsupported`: Unknown major version, or an unknown required field | §34.9 |
| FROB-REF-892 | `view-is-not-identity`: A view is offered where an identity is required: to hash, to sign, to cite as evidence, or as a cache key | §34.9 |
| FROB-REF-893 | `format-owner-bypass`: A file with a registered extension reaches a verb through a path that did not go through its owner's reader | §34.9 |
| FROB-REF-894 | A snapshot restored under another numerical profile and labelled a continuation | §29.17 |
| FROB-REF-895 | A band for a design that was never built | §30.19 |
| FROB-REF-896 | A pagination cursor used with a different pin, filter or order | §29.17 |
| FROB-REF-897 | Use of a retired calibration: The retirement reason; recalibrate | §30.19 |
| FROB-REF-898 | An order-dependent aggregate requested on the evidence path | §29.17 |
| FROB-REF-899 | A private or non-redistributable input to a kit | §30.19 |

