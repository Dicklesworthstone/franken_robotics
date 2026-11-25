# ADR-NUM-refusal-8xx: A full refusal family continues at four digits

- **ID:** `ADR-NUM-refusal-8xx`
- **Status:** `accepted` (owner ruling, 2026-11-25; the owner delegated owner decisions to the orchestrator, GentleEagle)
- **Governing bead:** `N35x-01`
- **First milestone:** `M0`
- **Plan references:** §35 (refusal registry), N35-33, N35x-02, N32a-03, BD-005

## Decision

**Option A, then option B.** Every refusal request is first classified by what is refused, and so placed in its family
(A). Requests that remain in a family with no free three-digit number continue at four digits within that family: the
evidence family continues at `FROB-REF-8100` (B). The family is still the first digit, a retired number stays retired,
and nothing already printed changes.

Consequences, each owned by the bead that owns the artifact:

- The code pattern is `FROB-REF-[1-9][0-9]{2,3}` in the registry schema, in the generated code type (BD-005), and in
  `frob.envelope/1` (N32a-03).
- The exit rule gains one row per four-digit hundred in use, with the same exit as its family. For example, `81xx` maps
  to 3, with the family's `conflict` (7) and `integrity` (9) lists applied as before (N35-33).
- Codes are compared as whole tokens everywhere. `FROB-REF-810` is a prefix of `FROB-REF-8100`, so a substring or
  prefix match is a defect. The registry test plants one.
- The printout sorts by family, then numerically.
- The four free numbers `836` to `839` are assigned by the allocation rule like any other free number. The hold
  described in N35x-01 is not needed for `8xx`. The `[[hold]]` mechanism itself is still built, because it is generic.
- The second allocation round (N35x-02) is run under this ruling. It is a pure function of the ledger and the registry.

## Reason

B is the only option that keeps the rule the whole surface rests on: one condition has one stable code, one message,
ranked repairs and one exit. Agents repair by code, so a condition that hides behind another code's `detail` value
(option D) gives them the wrong repairs. At M0 no code has been published in a result, a log or a schema snapshot, so
widening the pattern now costs one generator change and one schema pattern. After M1 publishes codes, the same change
would mean retirements. Three-digit families stay exactly as printed.

## Alternatives excluded

- **A alone:** 33 requests (22 from the second round, 11 from the first) remain for 4 free numbers.
- **C, a continuation inside `9xx`:** `9xx` is the internal family with exit 9. Rows with exit 3 there would fail the exit
  rule and take the numbers that two integrity faults need. The proposed `901` to `907` are not entered.
- **D, permanent sub-reasons:** two identifiers for one condition, per-verb refusal lists lose resolution, and repairs
  differ per sub-reason. A `detail` value remains only as the interim form for a request whose number is not yet
  allocated.

## Enforced by

- The N35-33 registry loader and its tests: a planted `9xx` row with exit 3 fails the load, and a planted row
  `FROB-REF-8100` loads with exit 3 (N35x-01 acceptance, the "if B is ruled" branch).
- The N35-33 registry test: a substring match of `FROB-REF-810` against `FROB-REF-8100` is planted and must be caught.
- The generated capacity table in `docs/generated/refusals.md` (N35x-01): a family whose requests exceed its free
  three-digit numbers is still marked `over capacity`, and its row also names the four-digit range in use.

## Revisit when

- A four-digit range of one family passes 50 numbers in use. That would suggest the family boundaries, not the number
  width, are wrong.
- Any consumer outside this workspace is found to parse refusal codes as exactly three digits.
