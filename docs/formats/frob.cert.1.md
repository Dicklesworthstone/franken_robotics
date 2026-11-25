# Certificate Bundle Format `frob.cert/1` (`FMT-FROBCERT`)

Normative specification for FrankenRobotics certificate bundles (`.frobcert`, schema `frob.cert/1`, format identifier `FMT-FROBCERT`, bead `N12-03`, `ADR-LEMMA-03`).

- **Owner (schema and reader):** `crates/frob-cert` (`crates/frob-cert/src/schema.rs`, `frob-verify`).
- **Writer (independent implementation sharing zero code with reader):** `frob-lemma` (`BD-027`, `BD-133`).
- **Encoding:** Strict canonical UTF-8 JSON (no BOM, keys sorted in ascending Unicode code-point / UTF-8 byte order, zero insignificant whitespace including trailing newline, minimal escapes, no duplicate keys, no floating-point literals, no JSON integer outside `±(2^53 - 1)`, no `-0`; `BD-026`).
- **Schema evolution:** Unknown fields are rejected (`schema.unknown-field`). New fields require a new schema version string; `frob.cert/1` bundles remain permanently readable.

---

## 1. Top-Level Fields (Closed Set of 13 Keys)

| # | Field | JSON Type | Requiredness | Summary & Validation Rules |
|---|---|---|---|---|
| 1 | `schema` | string | Required | Must equal the literal `"frob.cert/1"` (`schema.unknown`, `schema.missing-field`, `schema.field-type`). |
| 2 | `claim` | object | Required | Closed keys: `kind` (non-empty string), `class` (`Proved \| Certified \| Tested \| Estimated`), `quantifiers` (ordered array of non-empty strings), `horizon` (non-empty string), optional `cost_bounds` (R3), optional `unresolved_mass` (R4). |
| 3 | `query` | object | Required | Closed keys: `blake3` (64 lowercase hex chars), `bytes` (non-empty UTF-8 canonical JSON string of the query). `BLAKE3(query.bytes) == query.blake3` (`hash.mismatch`). |
| 4 | `definitions` | object | Required | Closed keys: `units` (non-empty string, e.g. `"SI"`), `frame` (non-empty string declaring origin, axes, handedness, sign conventions), `clocks` (array), `ports_ordered` (array of distinct non-empty strings — order is normative data), optional `origin`, `handedness`, `sign_convention`. |
| 5 | `assumptions` | array | Required | Array of objects with distinct `id` (non-empty string), `survives` (`"grow" \| "shrink"`), at least one of non-null `value` or 64-hex `blake3`, and optional `class` (`Proved \| Certified \| Tested \| Estimated`). |
| 6 | `plant` | object | Required | Closed keys: `model` (non-empty string), `policy_class` (64-hex BLAKE3 of the canonical `policy_class` section of the query), optional `model_class`, optional `inputs` (`[{id, class, blake3?}]`). |
| 7 | `domain` | object | Required | Closed keys: `chart` (non-empty string with units), `cell` (object with `lo`, `hi` as 1D `["p","q"]` or N-D `[["p","q"], ...]` reduced positive-denominator rationals with `lo <= hi`, optional `closed` booleans, optional `id`, `topology`, `lo_closed`, `hi_closed`, `open_ends`), optional `closed`, `unit`. |
| 8 | `proof` | object | Required | Closed keys: `kind` (non-empty string in `frob.cert.kinds/1`), `version` (integer >= 1, required), `primitive_equations` (non-empty array of non-empty Unicode strings, R2), `payload` (object), optional `side_conditions` (array), optional `required_side_conditions` (array of strings), optional `equations_dag`, optional `receipt`. |
| 9 | `checker` | object | Required | Closed keys: `name` (string), `build` (string), optional `version`, `registry`. Both `name` and `build` must be non-empty when `claim.class` is `Proved` or `Certified` (`class.no-receipt`). |
| 10 | `expected_verdict` | string | Required | Literal `"accept"` or `"reject"`. |
| 11 | `lineage` | object | Required | Closed keys: `merkle_root` (`""` or 64 lowercase hex chars; non-empty required for `Proved`/`Certified` via `class.no-receipt`), `alpha_spend` (array), optional `parent_hashes`, `receipt_hash`. |
| 12 | `authorization` | object | Required | Rust field name `flags: NoAuthorization`. Closed set of **exactly three** boolean fields: `execution_authorized`, `hardware_safety_signoff`, `sim_to_real_certified`. Every field must be present and literal `false` (`flags.missing`, `flags.true`; `FROB-REF-833`). Absence is never read as `false`. |
| 13 | `review` | object | Required for mission claims (`family-inclusion`, `family-exclusion`, `partition`); optional for `kernel-identity`, `qualification-receipt`, and analysis claims; validated whenever present | OPEN-AUGUR-8 marker, separate from `authorization`. Closed keys: `equations_reviewed` (boolean, required); when `true`, also requires `equations_digest` (64-hex BLAKE3 of `serialize_canonical(proof.primitive_equations)`), `review_record` (non-empty ledger artifact digest), and `scope` (non-empty text). Missing on mission claims -> `schema.review-missing`; digest mismatch -> `hash.equations-digest`. |

---

## 2. Format Rules (R1–R5)

- **R1 Typed fields (`field.type`, `FROB-REF-829`):** Every physical field entering the argument is a typed object declaring non-empty `quantity`, `frame` (matching `definitions.frame`), `origin`, `sign`, `unit` (compatible with `definitions.units`), and `value`.
  - `proof.payload.torque` must declare `quantity: "torque"`, unit `"N*m"` or `"N·m"`, and a 3-component exact rational vector. When `proof.payload.first_moment` `(P_x, P_y)` is also present under a `+z up` frame, `torque` must satisfy `(τ_x, τ_y, τ_z) = (P_y, -P_x, 0)` (CX-077, CX-108). Passing a 2-component first moment `(60, 30)` where torque `(30, -60, 0)` N·m is expected is rejected at `proof.payload.torque`.
  - `proof.payload.forces_newton` (when present) must match the length of `definitions.ports_ordered`.
- **R2 Primitive equations (`side.equations-missing`, `FROB-REF-829`):** `proof.primitive_equations` must be present and contain at least one non-empty Unicode equation string.
- **R3 Minimum and open threshold (`claim.minimum-unbounded`):** When `cost_bounds` is present:
  - `attained: true` requires `strict` absent/false, exact rational `incumbent`, exact rational `lower_bound == incumbent`, and `gap` absent or `0`.
  - `attained: false` (open threshold, e.g. docking `ε < 5/16` at `U = 3/4`, CX-114) requires `strict: true`, exact rational `bound`, feasible `candidate`, and strictly positive `gap == |bound - candidate|`.
- **R4 Unresolved probability mass (`claim.mass-without-weighting`):** When `unresolved_mass` is present, it must be an object supplying a non-empty `weighting` (string or non-empty object) alongside `value` (in `[0, 1]`).
- **R5 Exact numbers (`num.rational-form`, `num.interval-decl`, `num.int-range`, `FROB-REF-849`):**
  - Rationals travel as reduced positive-denominator pairs `["p", "q"]` (`gcd(|p|, q) == 1`, `q > 0`, canonical decimal digits) or strict reduced slash strings `"p/q"`.
  - Outward-rounded intervals travel as objects declaring non-empty `"impl"`, positive integer `"precision"`, and valid exact rational `"lo" <= "hi"`.
  - Integers with magnitude `>= 2^53` travel as decimal strings; JSON numbers outside `±(2^53 - 1)` are rejected at parse time (`num.int-range`).

---

## 3. Generic Rejection Rules (Strict Precedence Order 1 → 6)

Every `frob.cert/1` bundle passes canonical JSON parsing (`BD-026`) and then the six generic rules in exact order before any kind-specific checker runs. A bundle whose kind has status `not-implemented` in `frob.cert.kinds/1` still executes Rules 4–6 so every generic defect is reported as `reject`; `unsupported` is returned only when all six generic rules pass.

1. **Structure, schema, numeric form, and recomputable hashes** (`schema.unknown`, `schema.missing-field`, `schema.unknown-field`, `schema.field-type`, `schema.review-missing`, `num.rational-form`, `num.interval-decl`, `hash.mismatch`, `hash.equations-digest`).
2. **Flags** (`flags.missing`, `flags.true`; **`FROB-REF-833`**, retry: `never`, no repair). Runs strictly before Kind lookup so a bundle with a true flag and an unknown kind reports `flags.true` first.
3. **Kind lookup** (`kind.unknown`, `kind.version-unknown`, `kind.out-of-scope`, `kind.disabled`; **`FROB-REF-823`**) against the closed compiled registry `frob.cert.kinds/1` (`N12-02`).
4. **Query match** (`query.prefix`, `query.domain`, `query.assumptions`, `query.policy-class`; **`FROB-REF-826`**). Compares field-by-field against the caller-offered query (`--query <file>`) or else `query.bytes`:
   - `query.prefix`: exact ordered equality of `claim.quantifiers` vs offered query `quantifiers`.
   - `query.domain`: exact field-by-field equality of `domain.chart`, exact rational `domain.cell.{lo,hi}`, and `closed` markers.
   - `query.assumptions`: order-independent set equality of `(id, survives, value, blake3)`.
   - `query.policy-class`: `plant.policy_class == BLAKE3(serialize_canonical(offered_query.policy_class))` covering channel, timing with packet arrival instants relative to commands, indexing, authority, and randomization.
5. **Side conditions, typed fields, and claim rules** (`side.equations-missing`, `side.missing`, `field.type` -> **`FROB-REF-829`**; `claim.minimum-unbounded`, `claim.mass-without-weighting`).
6. **Class rule** (`class.above-kind-cap`, `class.above-input`, `class.no-receipt`; **`FROB-REF-834`**). Enforces `claim.class <= kind.cap`, `claim.class <= min(input/assumption classes)`, and non-empty `checker.{name,build}` + `lineage.merkle_root` for `Proved` and `Certified`.

---

## 4. Merkle Root Rule and Multi-Bundle Sets

An answer comprising multiple claims ships as one `.frobcert` bundle per claim sharing a single `lineage.merkle_root`:

1. **Leaf digest:** Clone each bundle, set `lineage.merkle_root` to `""`, serialize to canonical JSON, and compute the 32-byte `BLAKE3` digest.
2. **Ordering & uniqueness:** Sort leaf digests in ascending 32-byte lexicographic order. Any duplicate leaf digest is rejected (`hash.merkle-duplicate`).
3. **Inner nodes:** `BLAKE3(0x01 || left_32 || right_32)`. The `0x01` domain prefix prevents second-preimage confusion with leaves (whose canonical JSON bytes always start with `0x7b` `'{'`).
4. **Odd-width carry:** When a tree level has an odd number of nodes, its last node is carried up unchanged to the next level (e.g., 6 leaves produce level widths `6 -> 3 -> 2 -> 1`).
5. **Single-claim set:** A 1-leaf set's root is its sole leaf digest.
6. **Audit behavior:** Given a full bundle set, the auditor recomputes the root and rejects any mismatch with `hash.merkle-root`; given a single bundle alone, the auditor reports `recomputed: false`. The class of a set is the minimum (`Estimated < Tested < Certified < Proved`) over its members.

---

## 5. Golden Specimens and Generic Mutants

- `crates/frob-cert/tests/golden/stance_inclusion.frobcert` — stance family inclusion at `b = 3/14 m` (`lp-primal`, `Certified`, attained minimum with matching lower bound, typed `first_moment` `(60, 30)` and `torque` `(30, -60, 0)` N·m, `review: {equations_reviewed: false}`).
- `crates/frob-cert/tests/golden/docking_open_threshold.frobcert` — docking open threshold at `U = 3/4`, `ε < 5/16` (`finite-game`, `Certified`, `attained: false`, `strict: true`, candidate `1/4`, gap `1/16`).
- `crates/frob-cert/tests/golden/six_entry/` — Appendix L.7 six-entry listing (`e1`..`e5` `Certified`, `e6` `Tested` qualification receipt) under one shared Merkle root (`6 -> 3 -> 2 -> 1`, set class `Tested`).
- `crates/frob-cert/tests/golden/mutants/` + `expected.toml` — one canonical mutant per generic rule plus the rule-order control (`flags.true` + `kind.unknown` -> `flags.true`).
