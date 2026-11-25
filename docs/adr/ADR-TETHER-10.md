# ADR-TETHER-10: Dashboard and Viewer Are Ordinary Verb Clients

- **ID:** `ADR-TETHER-10`
- **Status:** `accepted`
- **Governing bead:** `N32b-06, N32b-10`
- **First milestone:** `M3`
- **Plan references:** §32.3, §32.19 (INV-TETHER-12), GATE-TETHER-14

## Decision

Both the terminal dashboard (`frob-tui`) and the browser viewer (`viewer/`) are ordinary read-only clients of the public verb surface and the daemon's typed event stream. Every number, status, pose, overlay, or diagnostic displayed in either UI is obtainable by calling a named verb from `registries/verbs.toml` with the same evidence and claim-product labels.

## Reason

Forbidding private side-channels eliminates unaudited state paths, guarantees that human UIs cannot display unlabelled or uncertified figures that bypass the envelope contract, and ensures headless AI agents without visual interfaces lose zero information compared to human operators.

## Alternative excluded

Allowing `frob-tui` or `viewer/` to open the SQLite ledger, inspect daemon memory, or read raw simulator state files directly through private internal APIs.

## Enforced by

- `INV-TETHER-12` client-boundary tests (pending at M2 for `viewer/` in `N32b-10`, M3 for `frob-tui` in `N32b-06` and `N32b-22`): verify neither UI links private daemon/ledger internals and every displayed field traces to a public verb response.
- `GATE-TETHER-14` (pending at M3, measured by `N32b-09` and `N32b-14`): asserts every number displayed by the dashboard and viewer is reproduced identically by its named source verb.

## Revisit when

- Real-time local telemetry rendering demonstrates that verb-streamed immutable snapshots cannot meet frame-rate targets (must be addressed by an explicit registered streaming/snapshot verb carrying provenance, never an unregistered private file read).
