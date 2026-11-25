#![forbid(unsafe_code)]

//! Airlock admission-probe entry points and planted mutants (`N33a-11`).
//!
//! Compiled only under `#[cfg(any(test, feature = "conformance"))]`; absent
//! from every default production binary. Not an airlock feature group.
//! Populated by `N33c-36`, `N33c-43`, `N33c-44`, and `N14a-19`.
