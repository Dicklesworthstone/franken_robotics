---
key: "ART-L2"
title: "golden fixture L.2: the `.fmodule` leg `leg-3r-qdd`, its port contract read against quad-tray, and CLI script"
type: "task"
priority: 1
milestone: "M1"
labels: ["m1", "skeleton", "frob-design", "frob-conformance", "solid", "test", "e2e"]
parent: "WP-38"
deps: ["BD-054", "BD-060", "BD-075", "BD-077", "ART-L1", "NJL-07", "STD-E2E"]
needs: []
related: ["N15b-02", "N14a-30", "BD-076", "BD-108", "BD-141", "BD-058", "ART-L9"]
gates: []
plan_refs: ["Appendix L.2", "§14.6", "§14.9", "§15.10", "§15.5"]
---
## Background

FrankenRobotics (a robotics simulation and design system reached through the command line `frob`) lets a design
reuse a **module**: a design subterm with open ports, parameters and a contract, stored as canonical JSON with
schema `frob.module/1` (extension `.fmodule`) and referenced by content hash, for example
`"std/leg-3r-qdd.fmodule@b3:9a4e…"`. The owner crate is `frob-design`; the verbs are `module.list`,
`module.show`, `module.check` and `module.substitute`.

This bead turns the second worked artifact of the specification into a golden fixture: the three-joint leg of
the quadruped `quad-tray` (artifact L.1), extracted by a typed edit, with its port contract. It carries the
artifact completely, its schema validation, its round trip, the checks made when the module is joined to the
robot, and the end-to-end script that produces it from the command line. The numbers are DESIGN specimens of
the format, not results; the arithmetic in the contract readings is exact.

Why this fixture matters (DESIGN). The module contract is where two tempting errors live, and the fixture pins
both with numbers. First, a contract that carries only peak demands can admit and cannot exclude. Second, the
summary a limb presents at its mount port does not describe a leg whose foot is in contact. A fixture that
shows the right behaviour on a real leg is the regression test for both. Rejected: testing the contract only
on synthetic two-port toys (they have no contact port and no shared thermal sink).

Terms. *Port*: a typed interface of a module, with the sign convention that power is positive into the
module. *Guarantees*: what the rest of the robot may rely on without opening the module. *Assumptions*: what
the module relies on from the rest. *Leaf-local substitution*: a replacement after which the products of the
sibling modules are reused.

## Scope

In: the specimen and resolved files; the port contract and its two readings; the junction checks against
`quad-tray`; extraction of the module from the design; a leaf-local and a global substitution as controls; the
script. Out: the module format and its verbs (BD-060), substitution and its invalidation contract (BD-077),
the bus and battery model (BD-075); the layers and the identity map (NJL-07).

## Technical approach

**Files** under `fixtures/golden/appendix-l/L2/`: `leg-3r-qdd.specimen.fmodule.json` (the text below);
`leg-3r-qdd.resolved.fmodule.json`; `port-contract.expect.json`; `substitution.expect.json`; `fields.toml`.

**The artifact.**

```json
{ "schema": "frob.module/1", "name": "leg-3r-qdd",
  "ports": [ {"id": "mount", "kind": "mech",    "frame": "hip_mount", "power": "into-module"},
             {"id": "bus",   "kind": "elec",    "nominal": "22.2 V",  "power": "into-module"},
             {"id": "sink",  "kind": "thermal", "power": "into-module"},
             {"id": "state", "kind": "signal",  "msg": "joint-state/3", "rate": "1000 Hz",
              "latency_max": "1 ms"},
             {"id": "sole",  "kind": "contact", "law": "foot-patch"} ],
  "params": [ {"id": "thigh", "unit": "m", "default": "0.21 m", "bounds": ["0.15 m", "0.30 m"]},
              {"id": "knee_card", "kind": "card:actuator", "default": "qdd-l"} ],
  "body": { "chain": [
      { "joint": {"class": "R", "axis": [1,0,0], "limits": ["-45 deg", "45 deg"]},  "actuator": "qdd-m",
        "body": {"region": {"capsule": ["0.03 m", "0.06 m"]}, "fill": "solid", "material": "al-6061-t6"} },
      { "joint": {"class": "R", "axis": [0,1,0], "limits": ["-90 deg", "150 deg"]}, "actuator": "qdd-m",
        "body": {"region": {"capsule": ["0.025 m", "0.21 m"]}, "fill": {"shell": "2 mm"},
                 "material": "al-6061-t6"} },
      { "joint": {"class": "R", "axis": [0,1,0], "limits": ["-160 deg", "-30 deg"]}, "actuator": "qdd-l",
        "transmission": {"belt": {"ratio": "2.35"}},
        "body": {"region": {"capsule": ["0.02 m", "0.21 m"]}, "fill": {"shell": "1.5 mm"},
                 "material": "cfrp-ud"} },
      { "foot": {"region": {"sphere": "0.022 m"}, "fill": "solid", "material": "rubber-60a",
                 "contact": {"law": "foot-patch", "interface": "rubber-60a/default",
                             "torsion_radius": "8 mm"}} } ] },
  "contract": {
    "guarantees": { "mass": ["1.6 kg", "2.4 kg"], "port_inertia": "sym-psd-6",
                    "bus_demand": {"peak": "38 A", "profile": "per-task, time-resolved"},
                    "heat_to_sink": {"continuous_max": "60 W"} },
    "assumes": { "bus_voltage": ["18 V", "25.2 V"], "sink_conductance_min": "0.4 W/K" },
    "digest": "b3:9a4e…" } }
```

**The port contract, read against `quad-tray`.**

| Port | Kind | Effort and flow | What joining it to `quad-tray` checks |
|---|---|---|---|
| `mount` | `mech` | Wrench and twist at `hip_mount` | Twists identified in the common frame, wrenches balance; power positive into the module |
| `bus` | `elec` | Voltage and current | The bus nominal of 22.2 V lies in the assumed [18, 25.2] V; currents balance at the node |
| `sink` | `thermal` | Temperature and heat rate | Four modules each assume 0.4 W/K; the shared sink offers 1.6 W/K, so the assumption is met with nothing to spare |
| `state` | `signal` | A typed message, 1,000 Hz, latency at most 1 ms | One producer, any number of typed consumers; carries no power |
| `sole` | `contact` | Meets the world under the `foot-patch` law | Listed because the module's effect on the robot cannot be summarized at its mount port alone |

**Port kinds and junction rules** (what `module.check` and the join enforce). Five kinds of power or signal
port: `mech` (wrench and twist at a named frame; twists identified in a common frame, wrenches balance);
`shaft` (torque and angular velocity; speeds identified, torques balance); `elec` (voltage and current;
voltages identified, currents balance at the node); `thermal` (temperature and heat rate; temperatures
identified, heat rates balance); `signal` (a typed message with a rate and a latency bound; one producer, any
number of typed consumers; no power). A `contact` entry names an end effector that meets the world. Joining
ports checks kinds, frames and signs, sums the power over the joined ports to zero and resolves causality; an
algebraic loop, a reversed sign or a copied power port refuses (`FROB-REF-108`).

**The contract.** Its digest covers the port signature, the parameter schema, the hash of the body term, the
versions of the cards it names and its local certificates: inertia consistency of every body over the
parameter box (structural for geometry-derived inertia) and passivity of every element. A module whose digest
or port signature does not match on import or substitution refuses (`FROB-REF-114`).

**Two readings that are easy to get wrong, with their numbers.**
1. *Peaks admit; they do not exclude.* Four legs with a peak demand of 38 A each add to 152 A against a bus
   limit of 60 A, and that sum excludes nothing. The bus analysis runs on the time-resolved demand profiles
   before any design is excluded on electrical grounds. The smallest case that shows why: two consumers share
   a bus limited to one unit of current, and each has a peak of one unit; one draws at the first tick and the
   other at the second, so a schedule exists although the peaks sum to two.
2. *The mount port does not summarize a leg in contact.* The packet a free tree limb presents to its parent is
   27 scalars: a symmetric positive-semidefinite 6 × 6 inertia (21 independent entries) and a bias wrench (6).
   It is semidefinite, never guaranteed definite: eliminating a joint with axis s from an inertia I gives
   I − I·s·(sᵀ·I·s)⁻¹·sᵀ·I, which sends s to zero (with I the identity and s the first basis vector the
   result is diag(0, 1, 1, 1, 1, 1)). It cannot be inverted at the port, and it does not describe a limb
   whose end effector is in contact, because contact couples every limb through the base.

**Substitution, as the fixture's control.** A leaf-local substitution of this module (same port signature; the
leg is a leaf with no closure to another branch; shared resources reached only through its `elec` and
`thermal` ports with assumptions met; each reused product read only fields outside the replaced subterm)
reuses the sibling legs' expanded subtrees, kernels and local analyses, recomputes mass roll-up, both symmetry
groups, budgets, closure and the self-collision pairs with the new limb, and invalidates static stability, the
baseline controller, every rollout and every whole-robot certificate. An edit to the bus, its battery or the
shared sink, a limb in a closure, or a changed port signature is global and invalidates everything downstream
of the model identity.

**One spelling the format does not fix.** How an entry of `params` names its target inside `body` is not
specified: the specimen writes the default values in place, while a lever in a `.fbot` names its target by a
JSON pointer `path`. DESIGN proposal for the resolved layer, confined to that content: each parameter gains a
`path` (`thigh`: `/body/chain/1/body/region/capsule/1`, whose value is `"0.21 m"`; `knee_card`:
`/body/chain/2/actuator`, whose value is `"qdd-l"`). The specimen layer stays as printed. If the format's
owner bead chooses another spelling, the resolved layer follows it.

**Junction checks against `quad-tray`** (`port-contract.expect.json`): the bus nominal 22.2 V lies in the
assumed [18, 25.2] V; four modules each assume a sink conductance of at least 0.4 W/K and the shared sink
offers 1.6 W/K, so the assumption is met with nothing to spare (a sink of 1.5 W/K must fail); the `state`
port has one producer; the `sole` entry names the law `foot-patch`, the same law as the foot in the body.

## Acceptance criteria

- [ ] The specimen equals the text above byte for byte (manifest SHA-256); the resolved file parses strictly,
      validates against `frob.module/1` and round-trips canonical → value → canonical with zero byte
      differences.
- [ ] `frob module check` accepts the fixture: five entries in `ports` with the kinds `mech`, `elec`,
      `thermal`, `signal`, `contact`; both parameters resolve to their targets; the mass guarantee encloses the
      computed leg mass at both ends of the thigh range; the digest recomputes.
- [ ] Extracting the leg from `quad-tray` by a typed edit yields a module whose `body` equals the fixture's
      link for link and a design whose compiled model identity is unchanged.
- [ ] Joined to `quad-tray`, the four junction checks hold; with the shared sink lowered to 1.5 W/K the thermal
      assumption fails and names the sink; with the bus nominal at 26 V the electrical assumption fails.
- [ ] A reversed power sign on the `bus` port refuses with `FROB-REF-108`; a tampered digest refuses with
      `FROB-REF-114`; each is exit 3 with one envelope.
- [ ] The peak sum is reported as a number and excludes nothing: the bus analysis on the profiles is invoked
      before any electrical exclusion, and a design is not refused on 152 A against 60 A alone.
- [ ] `frob module substitute` with the same module and `knee_card` bound to `qdd-m` reports `leaf-local`
      with the reuse table above; the same substitution combined with a bus edit reports `global`.
- [ ] All outputs are identical on two runs, in debug and release, on Linux and macOS (class D0).

## Test plan

### Unit tests

`cargo test -p frob-design --test golden_l2` (the real reader, writer and checker on the fixture; no mocks):
- `l2_specimen_parses_and_validates`; `l2_manifest_hash`; `l2_roundtrip_value`.
- `l2_port_kinds`: the five kinds in order; every power port has `power: "into-module"`.
- `l2_body_equals_l1_chain`: the module's chain equals `quad-tray`'s limb chain element for element.
- `l2_params_resolve`: `thigh` → `"0.21 m"` within [0.15, 0.30] m; `knee_card` → `"qdd-l"`.
- `l2_bus_assumption_met`: 22.2 ∈ [18, 25.2]; 26 V fails.
- `l2_sink_exactly_met`: 4 × 0.4 = 1.6 W/K passes with margin zero; 1.5 W/K fails (exact decimals).
- `l2_peak_sum_does_not_exclude`: 4 × 38 = 152 A > 60 A yields a finding of class "admission only", never a
  refusal; the two-tick two-consumer case is feasible while its peak sum is two units.
- `l2_port_packet_27_scalars`: 21 + 6; the eliminated inertia of the identity along the first axis is
  diag(0, 1, 1, 1, 1, 1) and is refused as an invertible quantity.
- `l2_reversed_sign_refuses_108`; `l2_tampered_digest_refuses_114`.
- `l2_substitution_leaf_local` and `l2_substitution_global`: the reuse and invalidation sets equal
  `substitution.expect.json`.

### End-to-end script and logging

`scripts/e2e/art_l2_fmodule.sh` (header: `bead=ART-L2 milestone=M1 labels=golden,appendix-l,skeleton,
verb:module.show,verb:module.check,verb:module.list,verb:module.substitute seed=1 needs=cpu
plants=sink-1.5,peaks-exclude`). Steps: (1) create `quad-tray` from artifact L.1's fixture; (2) extract the
leg with the typed edit that makes a module from a subterm, capture the module hash; (3) `frob module show`
and `frob module check` on it, assert kinds, parameters, guarantees and digest; (4) compare its canonical
bytes with the resolved fixture (`e2e_assert_digest_eq`); (5) `frob module list --json` contains it by content
hash; (6) `frob design check` on the design that uses the module, assert the four junction facts; (7) `frob
module substitute … --bind knee_card=qdd-m --key l2-sub --json`, assert `leaf-local` and the reuse table;
(8) the same with a bus edit in one patch, assert `global`; (9) `e2e_expect_refusal FROB-REF-108 3` and
`FROB-REF-114 3` on the two planted modules; (10) `frob-golden check --artifact L2`. Plants: `sink-1.5` must
fail step (6); `peaks-exclude` (a checker patched to refuse on the peak sum) must be caught by the assertion
that the design is not refused. The log (`target/e2e-logs/art_l2_fmodule/<utc>.ndjson`) has every command with
exit code and wall time, the evidence block, digests, each assertion with observed and expected values, each
refusal with code and first repair, and the `summary`.

### Known answers and negative controls

- Known answers (exact): 4 × 38 A = 152 A; 4 × 0.4 W/K = 1.6 W/K; 22.2 V in [18, 25.2] V; 27 = 21 + 6 scalars;
  diag(0, 1, 1, 1, 1, 1). They have no number in the known-answer suite; the unit's coverage file lists them.
- Counterexample kept alive: adding peak currents on a shared bus. Two consumers each need one powered tick
  and the bus powers one per tick: with a one-tick deadline no joint schedule exists; with two ticks exactly
  two exist; a rule that adds both peaks rejects the two-tick design, which is feasible.
- Negative controls: the two plants; the reversed sign; the tampered digest; the 1.5 W/K sink; the 26 V bus.
- Metamorphic relation: renaming the module (its `name` field) changes its content hash and leaves the
  junction checks and the reuse table unchanged.
- Determinism lanes: twice on one machine; debug and release; Linux and macOS; class D0.

## Considerations

- The guarantee interval for mass ([1.6, 2.4] kg) is illustrative; with the real card catalog the computed
  mass may fall outside it. The interval is therefore a `structural` field: the test asserts that the
  guarantee encloses the computed masses, and a blessing may re-derive the interval, which changes the digest.
- The specimen prints one abbreviation (`b3:9a4e…`) for both the hash that references the module and the
  contract digest inside it. A file cannot contain its own hash, so they cannot be the same value unless the
  reference hash is defined over the file with the `digest` field removed. Fail-closed default until the
  owner bead of the format decides: a reference `@b3:…` is the content hash of the canonical file, and
  `contract.digest` is a separate value; the identity map holds both.
- Kill gate KILL-SKEL-SUBST (reuse under substitution does not hold its gate): fallback, every substitution
  is reported `global`; steps (7) and (8) then assert `global` twice and the rest stands.
- The limb port interface serves the tree-operator route for large mechanisms; it is Frontier, and this
  fixture only pins its size and its semidefiniteness, not any solve through it.

## References

Appendix L.2; §14.6 (format, port contract, substitution contract, the limb port interface), §14.9, §15.5,
§15.10; INV-SKEL-06 (the invalidation set of an edit contains every product that differs after a full
recompile); GATE-SKEL-06 is measured by the substitution beads.
