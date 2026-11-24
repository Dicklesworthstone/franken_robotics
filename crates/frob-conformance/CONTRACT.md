# `frob-conformance` Contract

This crate belongs to the Gauntlet (layer 7, tier Solid). `BD-025` completes the full crate contract; `STD-E2E` declares the `frob-e2e` binary target and its table row below.

## Binaries

| Binary | Path | Owning bead | Reads | Writes | Registries |
|---|---|---|---|---|---|
| `frob-e2e` | `src/bin/frob-e2e.rs` | `STD-E2E` | Script headers (`# e2e:`), `frob.e2elog/1` NDJSON logs, and the captured output of the commands it runs | `frob.e2elog/1` NDJSON logs, transcripts, capture artifacts, mode-`0600` session token files, the gate report (`gates.json`, `gates.md`), and the `frob.e2elog/1` schema print | Reads and writes no registry |
