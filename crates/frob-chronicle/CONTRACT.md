# frob-chronicle contract

## Purpose

Identify M0 referee runs and encode durable records for the BD-009 ledger.

## Inputs

Catalog model identity, stack receipt, foundation pin, integer step count, seed and optional conformance perturbation.

## Outputs

Canonical run manifest and BLAKE3 run identity, canonical run rows, and independent execution records.

## Invariants

Wall-clock execution times, traces and idempotency keys stay out of content identities. The caller publishes the run, episode, execution and key binding in one adapter transaction.

## Refusals

The adapter reports typed ledger errors. The verb owns surface refusals.

## Determinism class

Manifest bytes are D0 for equal inputs. The referee's state digests are scoped to the same backend, binary and toolchain (D2).

## Evidence class of outputs

Chronicle records caller evidence and issues no evidence claims.

## Validity domain

The M0 G1 referee lane and the BD-009 ledger schema.
The CLI uses the file named by FROB_STORE or defaults to chronicle/ledger.db in its working directory.

## Oracles

The manifest tests check stable canonical identity and seed separation. The caller compares physical digests against the BD-007 in-tree fixture.

## Known gaps

BD-090 and BD-091 supply the full Chronicle schema, episode files, and chunk publishing.
