# ADR-<GATE-NAME>-<MILESTONE>[-<READING>]: Kill-Gate Decision for `<GATE-NAME>`

> **Template (`N40-03` step 6).**
> File path rules (enforced by `decision-adr-path`):
> - Primary decision at `<MILESTONE>`: `docs/adr/ADR-<GATE-NAME>-<MILESTONE>.md`
> - Second reading for one scope at one exit (`reading = "final"`): `docs/adr/ADR-<GATE-NAME>-<MILESTONE>-final.md`
> - Re-opened decision at an exit already decided (`reading = "reopened"`): `docs/adr/ADR-<GATE-NAME>-<MILESTONE>-reopened.md`
> - Per-scope decisions taken at the same exit are sections of the single exit file (`## Scope: <scope_kind>:<value>`).
> - Never store decision ADRs under the retired `docs/delivery/adr/` directory; never renumber committed decision ADRs.

## Metadata

- **Kill gate:** `<GATE-NAME>`
- **Milestone:** `<M0..M8>`
- **Reading:** `primary` | `final` | `reopened`
- **Scope (if `scope_kind` set):** `<scope_kind>:<value>` (or `whole-row`)
- **Date (ISO-8601 UTC):** `YYYY-MM-DDTHH:MM:SSZ`
- **Recording session:** `<fresh-context session ID — must NOT be in the mechanism's implementer set>`
- **Implementer sessions excluded:** `<comma-separated implementer session IDs>`

## Trigger and Amendments Applied

- **Trigger as registered:** `<verbatim trigger text from registries/gates.toml>`
- **Amendments applied (`[[amendment]]`):** `<none, or list AMD-xxxx IDs with date, commit, old -> new, and reason>`

## Measured Values and Receipts

| Read Gate / Pilot | Trigger Bound | Measured Value | Receipt Hash (`receipts/gates/` or `receipts/pilots/`) |
|---|---|---|---|
| `<GATE-ID or PILOT-ID>` | `<trigger bound>` | `<actual measured value>` | `<64-hex BLAKE3 digest>` |

*(For qualitative triggers, cite the written argument evaluated by the exit review `N40-05`.)*

## Outcome

- **Outcome:** `not-fired` | `fired` | `not-reached` | `deferred`
- **If `deferred`:** `deferred_to = <M..>`, `reason = <reason>`
- **Fallback in force (if `fired`):** `<verbatim fallback or fallback_form>`
- **Capability name carried from now on:** `<original capability name if not-fired; fallback name if fired; withdrawn if not-reached>`

## Beads Re-Read

List every affected bead with exactly one disposition label (`N40-03` step 6, `N41-17` rule 5):

- `<BEAD-KEY>`: `stands on the fallback` | `superseded by this ADR` | `withdrawn before this decision` (`<reason / pilot or envelope ruling>`)

## Downstream Changes

- **Gates becoming `killed` or `superseded`:** `<list gates and detail strings>`
- **Release cells withdrawn:** `<list withdrawn cells or none>`
- **Wording removed from README and reports:** `<describe exact capability renaming>`
- **Rolling re-read line (if `mode = "rolling"` and exit > `decided`):** `<re-read confirmation at this exit>`
