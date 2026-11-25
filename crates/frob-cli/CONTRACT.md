# frob-cli contract

## Purpose

Serve the FrankenRobotics command line using the static verb registry (ADR-TETHER-01).

## Inputs

Command paths, positionals and flags derived from each registered verb and its input schema. Process options carry rendering and local dispatch choices.

## Outputs

Human-readable output or NDJSON progress followed by one `frob.envelope/1` (INV-TETHER-02). The process exit matches the envelope exit.

## Invariants

The CLI uses the shared dispatch service (ADR-TETHER-03). All verb bindings live in `frob-verbs`.

## Refusals

`FROB-REF-801` for unknown verbs, `FROB-REF-802` for invalid arguments, and `FROB-REF-990` for unmapped faults. Dispatch and handlers supply their registered domain refusals.

## Determinism class

D0 for canonical registry and schema output. Runtime observations depend on the installation.

## Evidence class of outputs

Answers carry the evidence emitted by the handler. Operator answers carry an Estimated claim product with an empty claims list while the Tested doorway is reserved for qualified harness and study outputs.

## Validity domain

The compiled registry, schemas, and local or Unix socket dispatch on supported hosts.

## Oracles

`frob doctor --section stack_receipt --json` reports the digests embedded from `receipts/HEAD` and `receipts/LOCK_DIGEST`. `tests/cli.rs` checks the actual process status and reads envelopes with the shared validator.
Unit-test conventions (STD-TEST) are in [docs/testing/CONVENTIONS.md](../../docs/testing/CONVENTIONS.md).

## Known gaps

Foundation admission, remaining doctor sections, the refusal allocation ledger, and the synthetic budget and deferred test verbs still require their backing integrations.
