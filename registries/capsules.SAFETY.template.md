# Capsule Safety Document (`<capsule.id>`)

<!-- Copy this template beside each annex unsafe capsule (for example,
     `spark/frob-spark-cuda/src/capsule/driver_load.SAFETY.md`) and register the
     capsule in `registries/capsules.toml`. Every one of the four sections below
     must be present and non-empty to pass `cargo xtask check-unsafe` (N34-07). -->

## What is unsafe here

<!-- Describe every `unsafe` block, `unsafe fn`, `unsafe trait`, `unsafe impl`,
     or `unsafe extern` item in this capsule. State explicitly what the foreign
     side (the vendor driver, device runtime, or OS loader) may do and what this
     capsule assumes it does. -->

## Invariants the safe API maintains

<!-- List the exact preconditions, postconditions, ownership, lifetime,
     alignment, aliasing, thread-safety, and error-handling invariants that each
     public item listed in `safe_api` upholds so callers in safe Rust cannot
     trigger undefined behavior. -->

## Why safe Rust cannot do this

<!-- Explain why this boundary cannot be expressed in safe Rust or through a
     safe facade crate (for example, `dlopen`/FFI symbol resolution or raw device
     driver ABI calls). Note that `unsafe` is never used to work around
     lifetime, aliasing, cancellation, or synchronization problems. -->

## How it is tested

<!-- Name the exact unit, integration, fault-injection, and device tests that
     exercise this unsafe boundary and verify the invariants above. -->
