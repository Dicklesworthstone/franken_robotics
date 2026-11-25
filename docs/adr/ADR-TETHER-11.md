# ADR-TETHER-11: Immutable Identifier Binding Before Request Digesting

- **ID:** `ADR-TETHER-11`
- **Status:** `accepted`
- **Governing bead:** `N32a-09`
- **First milestone:** `M1`
- **Plan references:** §32.3, §32.4, §32.6

## Decision

Every mutating (`mutate`) and job-submitting (`run`) verb resolves mutable human-friendly names (such as a design name, lineage head, or `@fixture:` reference) into immutable content-addressed identifiers (`d:...`, `m:...`, `s:...`, etc.) once during admission, *before* computing the canonical request digest for idempotency checking. Every resolved binding is echoed verbatim in the envelope's `bound` block.

## Reason

If the idempotency digest hashed unresolved mutable names, retrying a request with the same idempotency key after a name was repointed to a different artifact would silently treat two different physical inputs as the same request (or conversely trigger spurious `FROB-REF-803` conflicts without showing what changed). Binding immutable IDs first makes idempotent replay sound and auditable.

## Alternative excluded

Digesting the raw request text as typed with mutable names unexpanded, or resolving mutable names lazily inside each subsystem handler after idempotency admission.

## Enforced by

- `idempotency_moved_name_keeps_bound_id` (pending at M1, `N32a-09`): repoints a mutable name between the initial call and a retry with the same idempotency key, verifying the resolved content digest detects the change (`FROB-REF-803`) and that the envelope `bound` block records exact immutable identifiers.

## Revisit when

- A verb class introduces non-content-addressed external mutable state (must declare explicit `expected_parent` / `FROB-REF-804` concurrency guards and still record resolved immutable snapshots in `bound`).
