# ADR-TETHER-07: Persistent Multi-Slot Device Environments in `frobd`

- **ID:** `ADR-TETHER-07`
- **Status:** `accepted`
- **Governing bead:** `BD-295`
- **First milestone:** `M3`
- **Plan references:** §32.3, §32.11.1, KILL-SLOT, GATE-TETHER-09

## Decision

The daemon `frobd` dispatches simulation episodes into slots of persistent, shape-class-matched batched environments per device rather than allocating a fresh environment per submitted job.

## Reason

`FACT`: a single paired design comparison fills roughly 2.3% of a full GPU batch (`§32.11.1`). Allocating one dedicated device environment per job leaves more than 97% of the accelerator idle and multiplies setup overhead across concurrent studio queries. Persistent slotted environments share device capacity without leaking state across jobs.

## Alternative excluded

Allocating one standalone environment per job (retained explicitly as the fallback path if `KILL-SLOT` fires).

## Enforced by

- `GATE-TETHER-09` (pending at M3, `BD-295`): verifies slot isolation (bit-identical results whether a job runs alone or co-scheduled in a shared slot pool), throughput utilization, and clean slot reclamation on cancellation or fault.

## Revisit when

- `KILL-SLOT` fires (`BD-295`): this record transitions to `superseded` and is replaced by a successor ADR adopting the per-job environment fallback.
