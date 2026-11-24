---
key: "NEG-01"
title: "frob-conformance: worked-card goldens: the `frob.card/1` file, the loader, the two lanes and the coverage lint"
type: "task"
priority: 0
milestone: "M0"
labels: ["m0", "gauntlet", "frob-conformance", "solid", "test", "card", "registry", "review-notes-open"]
parent: "WP-34"
deps: ["BD-005", "BD-025", "BD-026", "NAB-09", "STD-TEST"]
needs: []
related: []
gates: []
plan_refs: ["Appendix E (preamble)", "Appendix E.7", "§34.1", "Appendix H.1", "§38"]
---
## Background

A worked card takes one relation the project relies on, substitutes numbers, and says what the result means and
what would overturn it. There are 41 (GB-01 to GB-41). Cards fix intuition and supply hand-checkable arithmetic
for the known-answer suite. They are not results and none is a measurement of FrankenRobotics code: every
substituted number is DESIGN arithmetic recomputed from the equation printed beside it. Each card has five
fields: **Equation** (the relation, with the sections, constructions, gates and refusal codes it supports),
**Substituted** (the numbers; every value is illustrative unless tagged FACT, and "exact arithmetic" marks a
substitution with no free parameter), **Intuition** (what the arithmetic shows), **Assumptions** (what the
relation needs in order to hold) and **Flip** (the change that would reverse the conclusion).

Why make them goldens. When the cards were recomputed, seven printed values in chapters disagreed with their own
formulas (a rounding rule mixed with an exact rule, a stale planning rate, a ratio attributed to the wrong share)
and three chapters disagreed with each other. Prose arithmetic drifts. A card stored as data with a test that
recomputes it cannot drift silently, and a mechanism that later claims to implement the relation can be checked
against the same numbers. Rejected alternatives: keeping cards as documentation only (the drift already
happened once); folding them into the per-chapter known-answer registries (a card spans several chapters and
carries a decision and a flip condition, which a known-answer row does not).

This bead builds the shared machinery. The 41 card beads (keys `GB-01` to `GB-41`) each add one file and one
test and depend on this bead.

## Scope

In: the file format, its loader and validator in `frob-conformance`; the two test lanes and their contract; the
tolerance kinds; the FACT re-read rule; the coverage lint; the status report. Out: the cards themselves (GB-01
to GB-41); the end-to-end script (NEG-02); the discrepancy register and stale-value scan (NEG-03).

## Technical approach

**Location (DESIGN).** `registries/cards/GB-nn.toml`. The repository layout places `cards/` under `registries/`
and has no top-level `cards/` directory, so "`cards/GB-nn.toml`" means this path. Tests live in
`crates/frob-conformance/tests/cards/gb_nn.rs` (one file per card, one `[[test]]` target `cards`). The
computation a card checks lives in the crate that owns the relation (for example the Cayley retraction in
`frob-lie`); `frob-conformance` already depends on every core crate, so no low-layer crate gains a dependency.

**File format `frob.card/1` (DESIGN).** The sketch below is abbreviated, several keys to a line; the real
file has one key per line, as TOML requires.

```toml
schema = "frob.card/1"
id = "GB-11"                       # must equal the file stem
title = "When does a winding run away?"
owner_crate = "frob-actuate"       # where lane B's computation lives
first_milestone = "M1"             # before it, lane B reports `not run`
arithmetic = "illustrative"        # "illustrative" | "exact" | "mixed"
supports = ["§15.7", "C-38", "GATE-SINEW-05", "FROB-REF-132"]
intuition = "<prose>"  assumptions = ["<prose>"]  flip = ["<prose>"]
[[input]]  name = "R0"  value = "0.20"  unit = "ohm"  tag = "illustrative"   # or tag = "FACT", source = "H.3 …", reread = "pending"
[[output]] name = "rise" value = "164.74464579901152" unit = "K" printed = "164.7" tol_kind = "rel" tol = "1e-9"
[[control]] name = "constant-resistance" defect = "alpha = 0" expect = "rise == 100.0 (must differ from golden)"
[[mechanism]] name = "integrated_400s"  first_milestone = "M1"      # optional; one entry per lane B check
[[surface]] verb = "actuator.thermal"  first_milestone = "M1"  args = ["<card>", "--current", "20A"]  map = { rise = "equilibrium.rise_K" }
[[surface]] verb = "actuator.thermal"  first_milestone = "M1"  args = ["<card>", "--current", "35A", "--require"]  expect = { exit = 3, refusal = "FROB-REF-132" }
```

Rules of the format:

1. Numbers are strings: a decimal (`"0.20"`) or a rational (`"5/16"`). The loader parses both exactly
   (`frob-cert`'s rational type); a float literal in the file is a validation error, so that no golden is
   silently rounded by a TOML reader.
2. `tol_kind` is one of `exact` (rational or integer equality), `count` (integer equality of an enumeration),
   `bits` (IEEE-754 bit equality in the stated width), `rel`, `abs`. Closed forms evaluated in `f64` use
   `rel = 1e-9` unless the card bead states otherwise; iterative solves state their own.
3. `printed` is the figure the card prints. The validator checks that `value`, rounded to the number of
   significant digits of `printed`, equals `printed` (round half to even on the decimal string). This is the
   check that would have caught every arithmetic row of the discrepancy list. Conventions (DESIGN, so the check
   is decidable): `printed` is a bare number in the unit of `value` (no thousands separators, no "about", no
   percent sign: 0.33% is written `"0.0033"`); trailing zeros of an integer without a decimal point are not
   significant (`"2000"` has one significant digit, so 2,000.6 passes), unless the output states
   `printed_sig = n`; a range in the card's prose is two outputs. Where the card's printed figure was computed
   from an input rounded first (GB-17 prints 14.8 ms from Ω rounded to 3.5, where the exact Ω gives 14.75 ms),
   the card carries both readings as separate outputs, each passing the check on its own value.
4. An input tagged `FACT` carries its source-register row and a `reread` field (`pending` or a date with the
   reader's mark). A card with a pending FACT input reports `citable = false`: a gate may not cite it until the
   primary source has been re-read. Its arithmetic still runs.
5. Every card has at least one `[[control]]`: a planted defect (the tempting wrong formula) whose result must
   differ from the golden by more than the tolerance. A card with no control fails validation.
6. `supports` is provenance; `flip` entries are prose, and where a flip has numbers the card bead adds them as
   outputs so that they are tested too.
7. Value kinds. Besides a number string, an output's `value` may be: a token (a verdict or label such as
   `"rejected"` or `"but-for"`, compared by string equality, `tol_kind = "exact"`); a bit pattern `"0x3F800000"`
   with `width = 32` or `64` (`tol_kind = "bits"`); or a vector or matrix written as an array of number strings
   (`["160", "130", "110", "0"]`, `[["-0.8", "0"], ["8.829", "-0.8"]]`), compared elementwise under the output's
   tolerance. An output the card does not print (a full-precision intermediate, a bit pattern, a token) sets
   `unprinted = true` in place of `printed`; every other output must carry `printed`.
8. `[[surface]]` entries (zero or more) name a verb, its arguments, the map from output names to fields of the
   verb's JSON, its own `first_milestone`, and optionally `expect = { exit, refusal }` for a call that must be
   refused; NEG-02 runs them. A card with no verb has no entry and says why in its bead.

**Two lanes (the contract every card test follows).**

- Lane A, `gb_nn_reference`: the test evaluates the card's equation itself, from the file's inputs, in exact
  rationals where the card is exact and in `f64` otherwise, and compares every output with its `value` under its
  tolerance. It uses no mechanism of the owning crate. A failure means the card file is wrong.
- Lane B, `gb_nn_mechanism`: the test calls the owning crate's public API (and, where a `[[surface]]` entry
  names a verb, the verb's in-process handler) on the same inputs and compares the same outputs. A failure means the mechanism
  disagrees with the card. Before `first_milestone` the lane is compiled out and reported `not run`, never
  `pass`. Where lane B checks need mechanisms that arrive at different milestones (GB-25: the rewrite table at
  M2 and the trace checker at M4, the card itself at M0), each such check is a `[[mechanism]]` entry with its
  own `first_milestone` and is gated, compiled out and reported separately; the card's `first_milestone` is
  the earliest of them. The surface step likewise carries the `first_milestone` of its verb (or of the
  verb's field the card reads) and reports `not run` before it.
- `gb_nn_controls`: each planted defect is evaluated in lane A's arithmetic and must miss the golden.

**Loader API (DESIGN).**

```rust
pub struct Card { pub id: CardId, pub inputs: Vec<Quantity>, pub outputs: Vec<Golden>, pub controls: Vec<Control>, /* … */ }
pub fn load(id: &str) -> Result<Card, CardError>;          // validates rules 1 to 6
pub fn check(card: &Card, name: &str, got: Exact) -> CheckResult;   // records measured, golden, tolerance
pub fn status() -> Vec<CardStatus>;   // id, lane A, lane B and each [[mechanism]] check and the surface
                                      // (pass | fail | not run, with the milestone it waits for), citable,
                                      // controls caught
```

**Card index (DESIGN).** `registries/cards/INDEX.toml` lists the 41 ids with the milestone of each card bead
(M0: GB-01, -08, -09, -18, -25; M1: GB-02, -03, -05, -06, -10, -11, -12, -13, -19, -23, -24, -30, -31; M2:
GB-04, -07, -16, -17, -20, -22, -40, -41; M3: GB-14, -15, -21, -29, -32, -33, -34, -39; M4: GB-26, -27, -28,
-35, -36; M5: GB-37, -38; cumulatively 5, 18, 26, 34, 39, 41). A card file's `first_milestone` may not be
earlier than its index milestone.

**Coverage lint** (`cards_registry_complete`): no file outside the index (`GB-42.toml` fails); `id` equals the
stem; `owner_crate` is a workspace member; every card file has at least one test of each of the three kinds,
recognized by the prefixes `gb_nn_reference`, `gb_nn_mechanism` and `gb_nn_controls` (a card's further tests,
such as `gb_24_authority_limits`, are allowed); no test names a missing file. Missing cards are reported by id
with their index milestone. With the environment variable `FROB_CARDS_EXPECT_THROUGH=Mn` the lint also fails
when a card whose index milestone is Mn or earlier is missing; without it, missing cards are reported and do
not fail (this bead closes before any card bead).

**Evidence.** Card results are claim kind Known answer ("recovers GB-nn within its tolerance"), class `Tested`
(the four classes are Proved, Certified, Tested, Estimated, and only a Proof claim can be above Tested), tier G0
(integer and rational cards) or G1 · host, determinism D0. They support no Reality, Parity or Performance claim.

## Acceptance criteria

- [ ] `cargo test -p frob-conformance --test cards` runs the loader's own tests and passes with zero cards on
      file, the lint reporting the 41 missing cards by id; with `FROB_CARDS_EXPECT_THROUGH=M0` and zero cards it
      fails naming the five M0 cards.
- [ ] A file with a float literal, a missing `printed` on an output not marked `unprinted`, a `printed` that
      `value` does not round to, a FACT input without a source, or no control is rejected, each with a distinct
      error naming the field; tokens, bit patterns and vectors load and compare as rule 7 states.
- [ ] `status()` distinguishes `not run` from `pass` for the whole lane, for each `[[mechanism]]` check and for
      the surface, and a card with a pending FACT reports `citable = false`.
- [ ] Rational parsing is exact: `"5/16"`, `"0.3125"` and `"3125e-4"` compare equal.
- [ ] `crates/frob-conformance/CONTRACT.md` documents the format, the two lanes and the lint.

## Test plan

### Unit tests

- `card_loader_parses_minimal`: a `synthetic` card with one input and one output loads.
- `card_rejects_float_literal`, `card_rejects_missing_control`, `card_rejects_fact_without_source`.
- `card_printed_roundtrip`: value `"0.5536688302318253"`, printed `"0.55"` passes; printed `"0.57"` fails.
- `card_printed_half_even`: value `"2.5"` with printed `"2"` passes and `"3"` fails; value `"7.69230769"` with
  printed `"7.7"` passes and `"7.8"` fails (the case of the discrepancy list).
- `card_tolerance_kinds`: `rel 1e-9` accepts 1 + 5·10⁻¹⁰ and rejects 1 + 2·10⁻⁹; `count` rejects 615 against 616;
  `bits` distinguishes +0.0 from −0.0.
- `card_status_not_run_is_not_pass`: a card whose `first_milestone` is later than the build's reports `not run`.
- `card_mechanism_milestones`: a `synthetic` card at M0 with a `[[mechanism]]` check at M4 and a surface at M2,
  built as M0, reports lane B `pass` for its M0 checks, the M4 check `not run (M4)` and the surface
  `not run (M2)`; the summary counts the two `not run` separately from passes.
- `card_printed_conventions`: value `"2000.6"` with printed `"2000"` passes; the same with `printed_sig = 4`
  fails; printed `"2,000"` and printed `"0.33%"` are rejected as malformed.
- `card_value_kinds`: a token output compares by string; `"0x3F800000"` at width 32 equals the bits of 1.0f32
  and `"0x00000000"` differs from `"0x80000000"`; a four-element vector fails if one element is off; an
  `unprinted` output with a `printed` field, or neither, is rejected.
- `cards_registry_complete`: the lint, with a planted extra file `GB-42.toml`, a planted missing test kind, and
  `FROB_CARDS_EXPECT_THROUGH=M1` on a scratch registry holding only the five M0 cards (fails naming the
  thirteen M1 cards).

### End-to-end script and logging

`scripts/e2e/worked_cards.sh` (NEG-02) drives the suite and logs per card. This bead supplies `status()` in the
JSON form that script reads.

### Known answers and negative controls

The loader's own negative controls are the rejected files above. Metamorphic relation: scaling every input and
output of a `synthetic` card by a power of two leaves an `exact` verdict unchanged. Determinism: the loader's
output is a pure function of the file bytes; `status()` is byte-identical twice on one machine and across
platforms.

## Considerations

- Cards print rounded figures; goldens carry the full recomputed value. If an implementer's recomputation
  disagrees with a golden beyond tolerance, the card bead is reopened; the golden is never edited to match a
  mechanism.
- A card is not a gate. A gate that cites a card names the card's test output and its `citable` flag.
- The `registries/cards/` directory is shared with nothing else; theorem cards live in their own registry.
- Size: the loader and validator are a few hundred lines; the 41 tests are owned by the card beads.

## References

Appendix E (preamble and E.7), §34.1 (layout), Appendix H.1 (marks and the re-read rule), §38 (known answers
and negative controls).
