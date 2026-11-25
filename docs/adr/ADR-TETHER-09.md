# ADR-TETHER-09: Safe JSON Integer Representation (`int53`, `u64str`, `i64str`)

- **ID:** `ADR-TETHER-09`
- **Status:** `accepted`
- **Governing bead:** `N32a-07`
- **First milestone:** `M1`
- **Plan references:** §32.3, §32.8.2, GATE-TETHER-07, CX-348

## Decision

Every integer field in a verb input or output schema uses one of three closed types:

- `int53`: JSON number with no fraction and no exponent, range-checked to `-(2^53 - 1)..=(2^53 - 1)` (`±9,007,199,254,740,991`), used for bounded counts (episodes, steps, bytes, worlds, pages, progress).
- `u64str`: canonical decimal JSON string matching `^(0|[1-9][0-9]{0,19})$` (`0..=u64::MAX`), used for world addresses, seeds, stream counters, tick indices, commit sequence numbers, and nanosecond instants.
- `i64str`: canonical decimal JSON string matching `^(0|-?[1-9][0-9]{0,19})$` (`i64::MIN..=i64::MAX`, refusing `"-0"`), used for signed offsets and differences.

Bare `"type": "integer"` schemas outside the shared `int53` definition are forbidden.

## Reason

`FACT`: standard JSON decoders in JavaScript and many agent harnesses decode every JSON number as an IEEE-754 binary64 float, which loses consecutive-integer precision above `2^53 - 1` (`CX-348`: `9,007,199,254,740,993` silently decodes to `9,007,199,254,740,992`, corrupting seeds and world addresses without error).

## Alternative excluded

Emitting all 64-bit integers as raw JSON numbers (silently corrupted by double-decoding clients), emitting every small count as a string (unnecessary friction for arithmetic counts), or adopting a vendor-specific BigInt JSON extension unsupported by standard clients.

## Enforced by

- `RL-INT` schema lint (`integer-type`, reserved in `N32a-02` and wired by `N32a-07`, pending at M1): rejects bare `"type": "integer"` nodes and untyped numeric identifier fields across all registered schemas.
- `GATE-TETHER-07` (pending at M1, `N32a-07`): verifies double-decoding client round-trip fidelity and `FROB-REF-808` range refusals.

## Revisit when

- A protocol layer switches away from RFC 8259 JSON to a binary framing with native 64-bit integer types (canonical JSON artifacts and MCP tool payloads remain bound by RFC 8259 interoperability).
