//! Shared-dialect preludes v0, checked-in skeletons, and assembly helpers (BD-013).
//!
//! Dialect sources live as plain data under `crates/frob-kernels/dialect/` so
//! the core workspace never depends on a GPU driver or annex crate.

use frob_core::id::Blake3;
use frob_core::refusal::Refusal;
use frob_kir::FROB_PRELUDE_VERSION;
use frob_kir::dialect_lint::{
    CompileOptions, DialectUnit, LintReport, MetalMathMode, PRELUDE_BOUNDARY_MARKER, SourceFile,
    lint,
};

/// Version of the checked-in CUDA and Metal preludes (`FROB_PRELUDE_VERSION`).
pub const PRELUDE_VERSION: u32 = FROB_PRELUDE_VERSION;

/// Metal gateway buffer argument limit (`ft-kernel-metal::MAX_ARGUMENTS`).
pub const METAL_MAX_ARGUMENTS: usize = 31;

/// Checked-in CUDA prelude v0 (`dialect/prelude_cuda.h`).
pub const PRELUDE_CUDA: &str = include_str!("../dialect/prelude_cuda.h");

/// Checked-in Metal Shading Language 3.1 prelude v0 (`dialect/prelude_metal.h`).
pub const PRELUDE_METAL: &str = include_str!("../dialect/prelude_metal.h");

/// Checked-in reference skeleton `K-selftest` (`dialect/skeletons/k_selftest.h`).
pub const SKELETON_K_SELFTEST: &str = include_str!("../dialect/skeletons/k_selftest.h");

/// Checked-in contraction & reassociation detector `K-probe-contract`
/// (`dialect/skeletons/k_probe_contract.h`, KA-526 / GB-25).
pub const SKELETON_K_PROBE_CONTRACT: &str = include_str!("../dialect/skeletons/k_probe_contract.h");

/// Device backend family for shared-dialect prelude selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackendTarget {
    Cuda,
    Metal,
}

impl BackendTarget {
    /// Prelude header filename for this backend.
    #[must_use]
    pub const fn prelude_name(self) -> &'static str {
        match self {
            Self::Cuda => "prelude_cuda.h",
            Self::Metal => "prelude_metal.h",
        }
    }

    /// Checked-in prelude header text for this backend.
    #[must_use]
    pub const fn prelude_source(self) -> &'static str {
        match self {
            Self::Cuda => PRELUDE_CUDA,
            Self::Metal => PRELUDE_METAL,
        }
    }

    /// Production safe compile options for this backend.
    #[must_use]
    pub fn production_options(self) -> CompileOptions {
        match self {
            Self::Cuda => CompileOptions::cuda_safe("sm_89"),
            Self::Metal => CompileOptions::metal_safe(),
        }
    }
}

/// Exact Rust bit-reinterpretation twin of the dialect macro `FROB_F32_BITS(b)`.
#[must_use]
pub const fn frob_f32_bits(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// Assemble a [`DialectUnit`] for `target` from optional shape-class `#define`
/// lines, the checked-in version-0 prelude, and one or more shared kernel files.
#[must_use]
pub fn assemble_unit(
    target: BackendTarget,
    shape_constants: Option<SourceFile>,
    sources: Vec<SourceFile>,
) -> DialectUnit {
    DialectUnit {
        shape_constants,
        prelude: Some(SourceFile::new(
            target.prelude_name(),
            target.prelude_source(),
        )),
        sources,
    }
}

/// Expand a dialect unit for `target` and verify it passes the dialect lint
/// under `target`'s production compile options.
pub fn assemble_and_lint(
    target: BackendTarget,
    shape_constants: Option<SourceFile>,
    sources: Vec<SourceFile>,
) -> Result<(String, Blake3, LintReport), Refusal> {
    let unit = assemble_unit(target, shape_constants, sources);
    let report = lint(&unit, &target.production_options())?;
    let expanded = unit.expand();
    let digest = unit.body_digest();
    Ok((expanded, digest, report))
}

/// Extract the backend-independent body text strictly after [`PRELUDE_BOUNDARY_MARKER`].
#[must_use]
pub fn body_after_prelude(expanded: &str) -> Option<&str> {
    DialectUnit::body_after_prelude(expanded)
}

/// Planted contraction-permitting compile-option fixtures (KA-526 / KA-525 negative controls).
///
/// Every entry must be rejected by `frob_kir::dialect_lint::lint` under rule 4 (`FROB-REF-251`).
#[must_use]
pub fn planted_contraction_options() -> Vec<(&'static str, CompileOptions)> {
    vec![
        (
            "cuda-fmad-true",
            CompileOptions::Cuda {
                options: vec!["-arch=sm_89".to_owned(), "--fmad=true".to_owned()],
            },
        ),
        (
            "cuda-fmad-absent",
            CompileOptions::Cuda {
                options: vec!["-arch=sm_89".to_owned()],
            },
        ),
        (
            "cuda-use-fast-math",
            CompileOptions::Cuda {
                options: vec![
                    "-arch=sm_89".to_owned(),
                    "--fmad=false".to_owned(),
                    "--use_fast_math".to_owned(),
                ],
            },
        ),
        (
            "metal-math-fast",
            CompileOptions::Metal {
                math_mode: MetalMathMode::Fast,
            },
        ),
    ]
}

/// Expand `FROB_F32_BITS(<arg>)` directly from a prelude header's `#define FROB_F32_BITS(b) ...`
/// line, returning the backend expression text (for unit-test verification of both preludes).
#[must_use]
pub fn expand_f32_bits_macro(prelude: &str, u32_literal: &str) -> Option<String> {
    for line in prelude.lines() {
        let trimmed = line.trim();
        if let Some(body) = trimmed.strip_prefix("#define FROB_F32_BITS(b)") {
            let rhs = body.trim();
            return Some(rhs.replace("(b)", &format!("({u32_literal})")));
        }
    }
    None
}

/// Parse an unsigned 32-bit hex/decimal literal with optional `u`/`U` suffix.
#[must_use]
pub fn parse_c_u32_literal(lit: &str) -> Option<u32> {
    let trimmed = lit.trim();
    let core = trimmed
        .strip_suffix('u')
        .or_else(|| trimmed.strip_suffix('U'))
        .unwrap_or(trimmed);
    if let Some(hex) = core.strip_prefix("0x").or_else(|| core.strip_prefix("0X")) {
        if hex.is_empty() || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        u32::from_str_radix(hex, 16).ok()
    } else {
        if core.is_empty() || !core.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        core.parse::<u32>().ok()
    }
}

/// Boundary marker re-export for downstream callers (`BD-020`).
pub const BOUNDARY_MARKER: &str = PRELUDE_BOUNDARY_MARKER;
