//! Static lint for the Spark shared C/CUDA/Metal kernel dialect (BD-013).
//!
//! Enforces dialect rules 1, 2, 3, 4, and 6 before any GPU compiler sees a
//! kernel unit, refusing any violation with [`RefusalCode::FrobRef251`].
//! Determinism class `D0`; zero `unsafe`; prints nothing (L-7).

use std::collections::BTreeSet;
use std::fmt;
use std::ops::Range;

use frob_core::id::{Blake3, hash_blob};
use frob_core::refusal::{Refusal, RefusalCode, Repair, RetryPolicy};

use crate::DIALECT_LINT_VERSION;

/// Canonical marker inserted between the backend prelude and shared kernel text
/// when assembling a compilable unit. Bytes strictly after this marker are
/// backend-independent and hashed to assert one shared source across backends.
pub const PRELUDE_BOUNDARY_MARKER: &str = "/* === FROB_PRELUDE_END === */";

/// Dialect rules enforced by BD-013 (rules 5, 7, 8, 9 arrive in N21a-10).
pub const ACTIVE_RULES: &[u8] = &[1, 2, 3, 4, 6];

/// Metal math mode passed to the Metal shader compiler (`ft-kernel-metal`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MetalMathMode {
    /// `MathMode::Safe` (`fastMathEnabled = false`): the only mode admitted by rule 4.
    Safe,
    /// `MathMode::Fast` (`fastMathEnabled = true`): rejected by rule 4.
    Fast,
}

impl MetalMathMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Fast => "fast",
        }
    }
}

/// Backend compile-option set checked alongside dialect source text under rule 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileOptions {
    /// CUDA (NVRTC / nvcc) option list, e.g. `["-arch=sm_89", "--fmad=false"]`.
    Cuda { options: Vec<String> },
    /// Metal Shading Language compiler mode (MSL 3.1).
    Metal { math_mode: MetalMathMode },
}

impl CompileOptions {
    /// Production CUDA option set for a target SM architecture (e.g. `"sm_89"`).
    #[must_use]
    pub fn cuda_safe(arch: &str) -> Self {
        Self::Cuda {
            options: vec![format!("-arch={arch}"), "--fmad=false".to_owned()],
        }
    }

    /// Production Metal option set (`MathMode::Safe`).
    #[must_use]
    pub const fn metal_safe() -> Self {
        Self::Metal {
            math_mode: MetalMathMode::Safe,
        }
    }
}

/// One named text file within a [`DialectUnit`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    pub name: String,
    pub contents: String,
}

impl SourceFile {
    #[must_use]
    pub fn new(name: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contents: contents.into(),
        }
    }
}

/// Shared-dialect compilation unit partitioned into shape constants, prelude,
/// and kernel source files (generated bodies and hand-written skeletons).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DialectUnit {
    /// Optional shape-class capacity `#define` block (only `#define IDENT <int>`).
    pub shape_constants: Option<SourceFile>,
    /// Optional backend prelude. Only the checked-in target prelude is exempt from rule 1.
    pub prelude: Option<SourceFile>,
    /// Shared-dialect kernel sources (generated bodies and skeletons); all rules apply.
    pub sources: Vec<SourceFile>,
}

impl DialectUnit {
    /// Construct a unit containing a single kernel source file (no constants, no prelude).
    #[must_use]
    pub fn single_source(name: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            shape_constants: None,
            prelude: None,
            sources: vec![SourceFile::new(name, contents)],
        }
    }

    /// Attach a shape-class constant block.
    #[must_use]
    pub fn with_shape_constants(mut self, file: SourceFile) -> Self {
        self.shape_constants = Some(file);
        self
    }

    /// Attach a backend prelude file.
    #[must_use]
    pub fn with_prelude(mut self, file: SourceFile) -> Self {
        self.prelude = Some(file);
        self
    }

    /// Append a kernel source file (generated body or skeleton).
    #[must_use]
    pub fn with_source(mut self, file: SourceFile) -> Self {
        self.sources.push(file);
        self
    }

    /// Assemble the complete unit text fed to a backend compiler:
    /// shape constants, prelude, [`PRELUDE_BOUNDARY_MARKER`], then all sources.
    /// Source includes naming the inlined prelude are removed before compilation.
    #[must_use]
    pub fn expand(&self) -> String {
        self.expand_with_boundaries().0
    }

    fn expand_with_boundaries(&self) -> (String, usize, usize) {
        let mut out = String::new();
        if let Some(sc) = &self.shape_constants {
            out.push_str(&sc.contents);
            if !out.ends_with('\n') {
                out.push('\n');
            }
        }
        let prelude_start = out.len();
        if let Some(pre) = &self.prelude {
            out.push_str(&pre.contents);
            if !out.ends_with('\n') {
                out.push('\n');
            }
        }
        out.push_str(PRELUDE_BOUNDARY_MARKER);
        out.push('\n');
        let source_start = out.len();
        self.append_sources(&mut out);
        (out, prelude_start, source_start)
    }

    fn append_sources(&self, out: &mut String) {
        for src in &self.sources {
            let mut start = 0;
            if let Some(pre) = &self.prelude {
                for tok in tokenize(&src.contents, true) {
                    if let TokenKind::Directive { name, body, span } = tok.kind
                        && name == "include"
                        && is_prelude_include(&body, Some(&pre.name))
                    {
                        out.push_str(&src.contents[start..span.start]);
                        // Retain physical line endings so compiler line numbers stay stable.
                        let removed = &src.contents[span.clone()];
                        out.extend(removed.chars().filter(|c| matches!(c, '\r' | '\n')));
                        start = span.end;
                    }
                }
            }
            out.push_str(&src.contents[start..]);
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
        }
    }

    /// Extract the kernel text strictly after [`PRELUDE_BOUNDARY_MARKER`].
    #[must_use]
    pub fn body_after_prelude(expanded: &str) -> Option<&str> {
        let idx = expanded.find(PRELUDE_BOUNDARY_MARKER)?;
        let rest = &expanded[idx + PRELUDE_BOUNDARY_MARKER.len()..];
        Some(rest.strip_prefix('\n').unwrap_or(rest))
    }

    /// BLAKE3 digest of the shared kernel bytes strictly after the prelude marker.
    #[must_use]
    pub fn body_digest(&self) -> Blake3 {
        let expanded = self.expand();
        let body = Self::body_after_prelude(&expanded).unwrap_or("");
        hash_blob(body.as_bytes())
    }
}

/// One deterministic lint finding (`{rule, file, line, column, token, message}`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LintFinding {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub rule: u8,
    pub token: String,
    pub message: String,
}

impl LintFinding {
    #[must_use]
    pub fn new(
        rule: u8,
        file: impl Into<String>,
        line: usize,
        column: usize,
        token: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            rule,
            token: token.into(),
            message: message.into(),
        }
    }

    /// Render this finding as a canonical sorted-key JSON object.
    #[must_use]
    pub fn to_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\"column\":");
        out.push_str(&self.column.to_string());
        out.push_str(",\"file\":");
        write_json_str(&self.file, &mut out);
        out.push_str(",\"line\":");
        out.push_str(&self.line.to_string());
        out.push_str(",\"message\":");
        write_json_str(&self.message, &mut out);
        out.push_str(",\"rule\":");
        out.push_str(&self.rule.to_string());
        out.push_str(",\"token\":");
        write_json_str(&self.token, &mut out);
        out.push('}');
        out
    }
}

impl fmt::Display for LintFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rule {} at {}:{}:{}: `{}`: {}",
            self.rule, self.file, self.line, self.column, self.token, self.message
        )
    }
}

/// Clean lint report (determinism class `D0`, evidence class `Tested`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintReport {
    pub lint_version: u32,
    pub files_linted: usize,
    pub rules_checked: Vec<u8>,
    pub findings: Vec<LintFinding>,
}

/// Lint a [`DialectUnit`] and [`CompileOptions`] under rules 1, 2, 3, 4, and 6.
///
/// Returns `Ok(LintReport)` with zero findings when clean, or `Err(Refusal)` with
/// [`RefusalCode::FrobRef251`] carrying every structured finding in `detail`.
pub fn lint(unit: &DialectUnit, options: &CompileOptions) -> Result<LintReport, Refusal> {
    let findings = collect_findings(unit, options);
    let files_linted = usize::from(unit.shape_constants.is_some())
        + usize::from(unit.prelude.is_some())
        + unit.sources.len();

    if findings.is_empty() {
        return Ok(LintReport {
            lint_version: DIALECT_LINT_VERSION,
            files_linted,
            rules_checked: ACTIVE_RULES.to_vec(),
            findings,
        });
    }

    Err(build_refusal(&findings))
}

/// Collect all deterministic lint findings without wrapping in a [`Refusal`].
#[must_use]
pub fn collect_findings(unit: &DialectUnit, options: &CompileOptions) -> Vec<LintFinding> {
    let mut findings = Vec::new();

    check_compile_options(options, &mut findings);

    if let Some(sc) = &unit.shape_constants {
        scan_shape_constant_tokens(&sc.name, &tokenize(&sc.contents, true), &mut findings);
    }
    let (expected_prelude, checked_in) = match options {
        CompileOptions::Cuda { .. } => (
            "prelude_cuda.h",
            include_str!("../../frob-kernels/dialect/prelude_cuda.h"),
        ),
        CompileOptions::Metal { .. } => (
            "prelude_metal.h",
            include_str!("../../frob-kernels/dialect/prelude_metal.h"),
        ),
    };
    let trusted = unit
        .prelude
        .as_ref()
        .is_some_and(|pre| pre.name == expected_prelude && pre.contents == checked_in);
    if let Some(pre) = &unit.prelude {
        scan_prelude_zone(
            &pre.name,
            &tokenize(&pre.contents, true),
            trusted,
            &mut findings,
        );
    }
    let inlined_prelude = unit
        .prelude
        .as_ref()
        .and_then(|pre| (pre.name == expected_prelude).then_some(expected_prelude));
    for src in &unit.sources {
        scan_kernel_tokens(
            &src.name,
            &tokenize(&src.contents, true),
            inlined_prelude,
            &mut findings,
        );
    }
    // Every otherwise admissible unit must pass with the compiler's lexical
    // state, including across the prelude boundary and after include removal.
    let (expanded, prelude_start, source_start) = unit.expand_with_boundaries();
    let tokens = tokenize(&expanded, true);
    let name = "<assembled source>";
    if findings.is_empty() {
        let prelude = tokens.partition_point(|tok| tok.offset < prelude_start);
        let sources = tokens.partition_point(|tok| tok.offset < source_start);
        scan_shape_constant_tokens(name, &tokens[..prelude], &mut findings);
        scan_prelude_zone(name, &tokens[prelude..sources], trusted, &mut findings);
        scan_kernel_tokens(name, &tokens[sources..], None, &mut findings);
    }
    // A raw literal may close in a later file. Report it only in the assembled input.
    for tok in tokens
        .iter()
        .filter(|tok| tok.kind == TokenKind::InvalidRawString)
    {
        findings.push(LintFinding::new(
            1,
            name,
            tok.line,
            tok.column,
            "raw string literal",
            "invalid or unterminated raw string literal in compiler input",
        ));
    }

    findings.sort();
    findings.dedup();
    findings
}

/// Render a list of findings as a canonical JSON array string.
#[must_use]
pub fn findings_to_json(findings: &[LintFinding]) -> String {
    let mut out = String::from("[");
    for (idx, f) in findings.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push_str(&f.to_json());
    }
    out.push(']');
    out
}

fn build_refusal(findings: &[LintFinding]) -> Refusal {
    let first = &findings[0];
    let pointer = if first.file == "<options>" {
        "/options".to_owned()
    } else {
        format!("{}:{}:{}", first.file, first.line, first.column)
    };
    let observed = findings
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("; ");

    let triggered: BTreeSet<u8> = findings.iter().map(|f| f.rule).collect();
    let mut repairs = Vec::new();
    if triggered.contains(&3) {
        repairs.push(Repair::with_detail(
            "Replace built-in transcendental or rsqrt with a det_* function in the shared dialect",
            "Elementary functions use det_* ported from fs-math; square root uses FROB_SQRT(x).",
        ));
    }
    if triggered.contains(&2) {
        repairs.push(Repair::with_detail(
            "Replace floating-point or non-admitted atomic with count, exclusive scan, and deterministic write",
            "Only FROB_ATOMIC_ADDW_U32, FROB_ATOMIC_MAX_U32, FROB_ATOMIC_MIN_U32, and FROB_ATOMIC_OR_U32 are admitted.",
        ));
    }
    if triggered.contains(&4) {
        repairs.push(Repair::with_detail(
            "Disable floating-point contraction and fast-math and replace explicit fma/mad with separate statements",
            "CUDA requires --fmad=false and no --use_fast_math; Metal requires MathMode::Safe and no contraction-on pragma.",
        ));
    }
    if triggered.contains(&6) {
        repairs.push(Repair::with_detail(
            "Replace spin wait or grid-wide barrier with two separate kernel launches",
            "Intra-threadgroup sync uses FROB_GROUP_BARRIER(); while/do spin waits are forbidden (CX-221).",
        ));
    }
    if triggered.contains(&1) {
        repairs.push(Repair::with_detail(
            "Move backend qualifiers, intrinsics, and macros into prelude_cuda.h / prelude_metal.h",
            "Kernel sources call FROB_* macros only; shape-class constants belong in the shape_constants block.",
        ));
    }
    repairs.push(Repair::new(
        "Move the computation to the host if it does not belong in a batched kernel",
    ));

    Refusal::new(
        RefusalCode::FrobRef251,
        "a banned operation appears in kernel source (build time)",
        pointer,
        observed,
        "zero dialect lint findings across rules [1, 2, 3, 4, 6]",
        "findings",
        RetryPolicy::AfterChange,
        repairs,
    )
    .with_detail(findings_to_json(findings))
}

// ============================================================================
// Rule 4: Compile-option checks
// ============================================================================

fn check_compile_options(options: &CompileOptions, findings: &mut Vec<LintFinding>) {
    match options {
        CompileOptions::Cuda { options: flags } => {
            let mut saw_fmad_false = false;
            let mut saw_fmad_true = false;

            let mut idx = 0usize;
            while idx < flags.len() {
                let flag = flags[idx].trim();
                let next_val = flags.get(idx + 1).map(|s| s.trim());

                if flag == "--use_fast_math"
                    || flag == "-use_fast_math"
                    || flag.starts_with("--use_fast_math=")
                    || flag.starts_with("-use_fast_math=")
                    || matches!(
                        flag,
                        "-ffast-math" | "--fast-math" | "-fapprox-func" | "-Ofast"
                    )
                {
                    findings.push(LintFinding::new(
                        4,
                        "<options>",
                        1,
                        1,
                        flag,
                        "CUDA fast-math option is forbidden by dialect rule 4",
                    ));
                } else if let Some(val) = flag
                    .strip_prefix("-ffp-contract=")
                    .or_else(|| flag.strip_prefix("--fp-contract="))
                {
                    if val != "off" {
                        findings.push(LintFinding::new(
                            4,
                            "<options>",
                            1,
                            1,
                            flag,
                            "floating-point contraction option is forbidden by dialect rule 4",
                        ));
                    }
                } else if let Some(val) = flag
                    .strip_prefix("--fmad=")
                    .or_else(|| flag.strip_prefix("-fmad="))
                {
                    if matches!(val, "false" | "0") {
                        saw_fmad_false = true;
                    } else {
                        saw_fmad_true = true;
                        findings.push(LintFinding::new(
                            4,
                            "<options>",
                            1,
                            1,
                            flag,
                            "CUDA FMA contraction enabled; dialect rule 4 requires --fmad=false",
                        ));
                    }
                } else if matches!(flag, "--fmad" | "-fmad") {
                    if let Some(val) = next_val {
                        if matches!(val, "false" | "0") {
                            saw_fmad_false = true;
                        } else {
                            saw_fmad_true = true;
                            findings.push(LintFinding::new(
                                4,
                                "<options>",
                                1,
                                1,
                                format!("{flag} {val}"),
                                "CUDA FMA contraction enabled; dialect rule 4 requires --fmad=false",
                            ));
                        }
                        idx += 2;
                        continue;
                    }
                    saw_fmad_true = true;
                    findings.push(LintFinding::new(
                        4,
                        "<options>",
                        1,
                        1,
                        flag,
                        "CUDA --fmad flag missing `false` argument under dialect rule 4",
                    ));
                } else if let Some(val) = flag
                    .strip_prefix("--ftz=")
                    .or_else(|| flag.strip_prefix("-ftz="))
                {
                    if !matches!(val, "false" | "0") {
                        findings.push(LintFinding::new(
                            4,
                            "<options>",
                            1,
                            1,
                            flag,
                            "CUDA floating-point relaxation flag is forbidden by dialect rule 4",
                        ));
                    }
                } else if let Some(val) = flag
                    .strip_prefix("--prec-div=")
                    .or_else(|| flag.strip_prefix("-prec-div="))
                    .or_else(|| flag.strip_prefix("--prec-sqrt="))
                    .or_else(|| flag.strip_prefix("-prec-sqrt="))
                {
                    if !matches!(val, "true" | "1") {
                        findings.push(LintFinding::new(
                            4,
                            "<options>",
                            1,
                            1,
                            flag,
                            "CUDA floating-point relaxation flag is forbidden by dialect rule 4",
                        ));
                    }
                } else if matches!(flag, "--ftz" | "-ftz") {
                    if let Some(val) = next_val {
                        if !matches!(val, "false" | "0") {
                            findings.push(LintFinding::new(
                                4,
                                "<options>",
                                1,
                                1,
                                format!("{flag} {val}"),
                                "CUDA floating-point relaxation flag is forbidden by dialect rule 4",
                            ));
                        }
                        idx += 2;
                        continue;
                    }
                    findings.push(LintFinding::new(
                        4,
                        "<options>",
                        1,
                        1,
                        flag,
                        "CUDA --ftz flag missing `false` argument under dialect rule 4",
                    ));
                } else if matches!(
                    flag,
                    "--prec-div" | "-prec-div" | "--prec-sqrt" | "-prec-sqrt"
                ) {
                    if let Some(val) = next_val {
                        if !matches!(val, "true" | "1") {
                            findings.push(LintFinding::new(
                                4,
                                "<options>",
                                1,
                                1,
                                format!("{flag} {val}"),
                                "CUDA floating-point relaxation flag is forbidden by dialect rule 4",
                            ));
                        }
                        idx += 2;
                        continue;
                    }
                    findings.push(LintFinding::new(
                        4,
                        "<options>",
                        1,
                        1,
                        flag,
                        "CUDA precision flag missing `true` argument under dialect rule 4",
                    ));
                } else if matches!(
                    flag,
                    "-D" | "--define-macro"
                        | "-U"
                        | "--undefine-macro"
                        | "-include"
                        | "--pre-include"
                ) {
                    let tok = match next_val {
                        Some(val) => format!("{flag} {val}"),
                        None => flag.to_owned(),
                    };
                    findings.push(LintFinding::new(
                        1,
                        "<options>",
                        1,
                        1,
                        tok,
                        "CUDA macro or pre-include compile option is forbidden outside the prelude (rule 1)",
                    ));
                    idx += if next_val.is_some() { 2 } else { 1 };
                    continue;
                } else if flag.starts_with("-D")
                    || flag.starts_with("--define-macro=")
                    || flag.starts_with("-U")
                    || flag.starts_with("--undefine-macro=")
                    || flag.starts_with("-include=")
                    || flag.starts_with("--pre-include=")
                {
                    findings.push(LintFinding::new(
                        1,
                        "<options>",
                        1,
                        1,
                        flag,
                        "CUDA macro or pre-include compile option is forbidden outside the prelude (rule 1)",
                    ));
                } else if !flag.split_once('=').is_some_and(|(name, arch)| {
                    matches!(name, "-arch" | "--gpu-architecture")
                        && !arch.is_empty()
                        && arch.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                }) {
                    findings.push(LintFinding::new(
                        1,
                        "<options>",
                        1,
                        1,
                        flag,
                        "CUDA compile option is outside the dialect allowlist (rule 1)",
                    ));
                }
                idx += 1;
            }

            if !saw_fmad_false && !saw_fmad_true {
                findings.push(LintFinding::new(
                    4,
                    "<options>",
                    1,
                    1,
                    "--fmad=false",
                    "CUDA compile options omit --fmad=false; contraction defaults to enabled",
                ));
            }
        }
        CompileOptions::Metal { math_mode } => {
            if *math_mode != MetalMathMode::Safe {
                findings.push(LintFinding::new(
                    4,
                    "<options>",
                    1,
                    1,
                    format!(
                        "MathMode::{}",
                        match math_mode {
                            MetalMathMode::Safe => "Safe",
                            MetalMathMode::Fast => "Fast",
                        }
                    ),
                    "Metal math mode must be Safe (fastMathEnabled = false) under dialect rule 4",
                ));
            }
        }
    }
}

// ============================================================================
// C / CUDA / MSL Lexer (comment stripping + 1-based line/column preservation)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Ident(String),
    Number(String),
    StringLit(String),
    InvalidRawString,
    CharLit,
    DoubleColon,
    DoubleLBracket,
    TokenPaste,
    Punct(char),
    Directive {
        name: String,
        body: String,
        span: Range<usize>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    line: usize,
    column: usize,
    offset: usize,
}

#[derive(Debug, Clone, Copy)]
struct SplicedChar {
    ch: char,
    line: usize,
    col: usize,
    offset: usize,
}

/// Splice `\` + newline (LF, CRLF, or CR) before
/// tokenization while keeping the exact 1-based physical `(line, col)` of every
/// surviving character. Prevents `rsq\\\nrt`, `fm\\\naf`, `whi\\\nle` evasion.
fn splice_backslash_newlines(src: &str) -> Vec<SplicedChar> {
    let mut out = Vec::with_capacity(src.len());
    let mut chars = src.char_indices().peekable();
    if matches!(chars.peek(), Some((0, '\u{feff}'))) {
        chars.next();
    }
    let mut line = 1usize;
    let mut col = 1usize;

    while let Some((offset, ch)) = chars.next() {
        if ch == '\\' {
            // Clang also splices when horizontal whitespace precedes the newline.
            let mut clone = chars.clone();
            while matches!(clone.peek(), Some((_, ' ' | '\t' | '\x0b' | '\x0c'))) {
                clone.next();
            }
            if matches!(clone.peek(), Some((_, '\n' | '\r'))) {
                if matches!(clone.next(), Some((_, '\r')))
                    && matches!(clone.peek(), Some((_, '\n')))
                {
                    clone.next();
                }
                chars = clone;
                line += 1;
                col = 1;
                continue;
            }
        }
        if ch == '\r' && matches!(chars.peek(), Some((_, '\n'))) {
            chars.next();
        }
        let ch = if ch == '\r' { '\n' } else { ch };
        out.push(SplicedChar {
            ch,
            line,
            col,
            offset,
        });
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    out
}

fn raw_string_end(src: &str, quote: usize) -> Option<usize> {
    // Raw string contents retain their original bytes, including escaped newlines.
    let (delimiter, contents) = src[quote + 1..].split_once('(')?;
    let valid = |b: u8| b.is_ascii_graphic() && !matches!(b, b')' | b'\\');
    if delimiter.len() > 16 || !delimiter.bytes().all(valid) {
        return None;
    }
    let close = format!("){delimiter}\"");
    Some(src.len() - contents.len() + contents.find(&close)? + close.len())
}

fn raw_string_prefix(ident: &str) -> bool {
    matches!(ident, "R" | "u8R" | "uR" | "UR" | "LR")
}

fn tokenize(src: &str, mut at_line_start: bool) -> Vec<Token> {
    let sc = splice_backslash_newlines(src);
    let n = sc.len();
    let mut tokens = Vec::new();
    let mut i = 0usize;

    while i < n {
        let cur = sc[i];
        let ch = cur.ch;

        // Newline
        if ch == '\n' {
            i += 1;
            at_line_start = true;
            continue;
        }

        // Horizontal whitespace
        if matches!(ch, ' ' | '\t' | '\x0c' | '\x0b') {
            i += 1;
            continue;
        }

        // Line comment `//...`
        if ch == '/' && i + 1 < n && sc[i + 1].ch == '/' {
            i += 2;
            while i < n && sc[i].ch != '\n' {
                i += 1;
            }
            continue;
        }

        // Block comment `/* ... */`
        if ch == '/' && i + 1 < n && sc[i + 1].ch == '*' {
            i += 2;
            while i < n {
                if sc[i].ch == '*' && i + 1 < n && sc[i + 1].ch == '/' {
                    i += 2;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Token pasting uses the longest match, including the `%:%:` digraph.
        let starts_with = |text: &str| {
            let remaining = sc[i..].iter().map(|c| c.ch);
            remaining.take(text.len()).eq(text.chars())
        };
        let paste_width = match ch {
            '#' if starts_with("##") => 2,
            '%' if starts_with("%:%:") => 4,
            _ => 0,
        };
        if paste_width != 0 {
            tokens.push(Token {
                kind: TokenKind::TokenPaste,
                line: cur.line,
                column: cur.col,
                offset: cur.offset,
            });
            i += paste_width;
            at_line_start = false;
            continue;
        }

        // Preprocessor directive at the start of a logical line (`#` or `%:`).
        let directive_width = match ch {
            '#' => 1,
            '%' if sc.get(i + 1).is_some_and(|next| next.ch == ':') => 2,
            _ => 0,
        };
        if at_line_start && directive_width != 0 {
            i += directive_width;
            let clean = match consume_preprocessor_rest(src, &sc, &mut i) {
                Ok(clean) => clean,
                Err(open) => {
                    tokens.push(Token {
                        kind: TokenKind::InvalidRawString,
                        line: open.line,
                        column: open.col,
                        offset: open.offset,
                    });
                    break;
                }
            };
            let trimmed = clean.trim();
            let (name, body) = split_directive_head(trimmed);
            tokens.push(Token {
                kind: TokenKind::Directive {
                    name: name.to_owned(),
                    body: body.to_owned(),
                    span: cur.offset..sc.get(i).map_or(src.len(), |next| next.offset),
                },
                line: cur.line,
                column: cur.col,
                offset: cur.offset,
            });
            at_line_start = true;
            continue;
        }

        at_line_start = false;

        // String literal (retain decoded content for `_Pragma("...")` inspection)
        if ch == '"' {
            let start_line = cur.line;
            let start_col = cur.col;
            let mut decoded = String::new();
            i += 1;
            while i < n {
                if sc[i].ch == '\\' && i + 1 < n {
                    let esc = sc[i + 1].ch;
                    match esc {
                        '"' => decoded.push('"'),
                        '\\' => decoded.push('\\'),
                        'n' => decoded.push('\n'),
                        't' => decoded.push('\t'),
                        other => {
                            decoded.push('\\');
                            decoded.push(other);
                        }
                    }
                    i += 2;
                    continue;
                }
                if sc[i].ch == '"' {
                    i += 1;
                    break;
                }
                if sc[i].ch == '\n' {
                    break;
                }
                decoded.push(sc[i].ch);
                i += 1;
            }
            tokens.push(Token {
                kind: TokenKind::StringLit(decoded),
                line: start_line,
                column: start_col,
                offset: cur.offset,
            });
            continue;
        }

        // Character literal
        if ch == '\'' {
            let start_line = cur.line;
            let start_col = cur.col;
            i += 1;
            while i < n {
                if sc[i].ch == '\\' && i + 1 < n {
                    i += 2;
                    continue;
                }
                if sc[i].ch == '\'' {
                    i += 1;
                    break;
                }
                if sc[i].ch == '\n' {
                    break;
                }
                i += 1;
            }
            tokens.push(Token {
                kind: TokenKind::CharLit,
                line: start_line,
                column: start_col,
                offset: cur.offset,
            });
            continue;
        }

        // Identifier
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start_line = cur.line;
            let start_col = cur.col;
            let mut ident = String::new();
            while i < n && (sc[i].ch.is_ascii_alphanumeric() || sc[i].ch == '_') {
                ident.push(sc[i].ch);
                i += 1;
            }
            let kind = if raw_string_prefix(&ident)
                && let Some(quote) = sc.get(i).filter(|next| next.ch == '"')
            {
                if let Some(end) = raw_string_end(src, quote.offset) {
                    i += sc[i..].partition_point(|next| next.offset < end);
                    TokenKind::Ident(ident)
                } else {
                    i = n;
                    TokenKind::InvalidRawString
                }
            } else {
                TokenKind::Ident(ident)
            };
            tokens.push(Token {
                kind,
                line: start_line,
                column: start_col,
                offset: cur.offset,
            });
            continue;
        }

        // Numeric literal
        if ch.is_ascii_digit() {
            let start_line = cur.line;
            let start_col = cur.col;
            let mut num = String::new();
            while i < n
                && (sc[i].ch.is_ascii_alphanumeric()
                    || matches!(sc[i].ch, '.' | '_')
                    || (sc[i].ch == '\''
                        && sc
                            .get(i + 1)
                            .is_some_and(|next| next.ch.is_ascii_alphanumeric() || next.ch == '_')))
            {
                if matches!(sc[i].ch, 'e' | 'E' | 'p' | 'P')
                    && i + 1 < n
                    && matches!(sc[i + 1].ch, '+' | '-')
                {
                    num.push(sc[i].ch);
                    num.push(sc[i + 1].ch);
                    i += 2;
                    continue;
                }
                num.push(sc[i].ch);
                i += 1;
            }
            tokens.push(Token {
                kind: TokenKind::Number(num),
                line: start_line,
                column: start_col,
                offset: cur.offset,
            });
            continue;
        }

        // Multi-char punctuation: `::` and `[[`
        if ch == ':' && i + 1 < n && sc[i + 1].ch == ':' {
            tokens.push(Token {
                kind: TokenKind::DoubleColon,
                line: cur.line,
                column: cur.col,
                offset: cur.offset,
            });
            i += 2;
            continue;
        }
        if ch == '[' && i + 1 < n && sc[i + 1].ch == '[' {
            tokens.push(Token {
                kind: TokenKind::DoubleLBracket,
                line: cur.line,
                column: cur.col,
                offset: cur.offset,
            });
            i += 2;
            continue;
        }

        tokens.push(Token {
            kind: TokenKind::Punct(ch),
            line: cur.line,
            column: cur.col,
            offset: cur.offset,
        });
        i += 1;
    }

    tokens
}

/// Consume the rest of a logical preprocessor directive, keeping multi-line
/// `/* ... */` block comments inside the directive instead of truncating it,
/// while ending immediately at `//` line comments so `// /*` cannot swallow
/// subsequent source lines into the directive.
fn consume_preprocessor_rest(
    src: &str,
    sc: &[SplicedChar],
    i: &mut usize,
) -> Result<String, SplicedChar> {
    let n = sc.len();
    let mut out = String::new();
    let mut quote = None;

    while *i < n {
        let ch = sc[*i].ch;
        if let Some(delimiter) = quote {
            if ch == '\\' && *i + 1 < n {
                out.push('\\');
                out.push(sc[*i + 1].ch);
                *i += 2;
                continue;
            }
            if ch == delimiter {
                quote = None;
            }
            if ch == '\n' {
                *i += 1;
                break;
            }
            out.push(ch);
            *i += 1;
            continue;
        }
        if matches!(ch, '"' | '\'') {
            quote = Some(ch);
            out.push(ch);
            *i += 1;
            continue;
        }
        if ch == '/' && *i + 1 < n && sc[*i + 1].ch == '/' {
            *i += 2;
            while *i < n && sc[*i].ch != '\n' {
                *i += 1;
            }
            if *i < n && sc[*i].ch == '\n' {
                *i += 1;
            }
            break;
        }
        if ch == '/' && *i + 1 < n && sc[*i + 1].ch == '*' {
            out.push(' ');
            *i += 2;
            while *i < n {
                if sc[*i].ch == '*' && sc.get(*i + 1).is_some_and(|next| next.ch == '/') {
                    *i += 2;
                    break;
                }
                *i += 1;
            }
            continue;
        }
        if ch == '\n' {
            *i += 1;
            break;
        }
        if ch.is_ascii_alphanumeric() || ch == '_' {
            let start = *i;
            let start_out = out.len();
            while *i < n && (sc[*i].ch.is_ascii_alphanumeric() || sc[*i].ch == '_') {
                out.push(sc[*i].ch);
                *i += 1;
            }
            if raw_string_prefix(&out[start_out..])
                && let Some(open) = sc.get(*i).filter(|next| next.ch == '"')
            {
                let end = raw_string_end(src, open.offset).ok_or(sc[start])?;
                out.push_str(&src[open.offset..end]);
                *i += sc[*i..].partition_point(|next| next.offset < end);
            }
            continue;
        }
        out.push(ch);
        *i += 1;
    }
    Ok(out)
}

fn next_char(s: &str, idx: usize) -> char {
    s[idx..].chars().next().unwrap_or('\0')
}

fn strip_comments_inline(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let n = bytes.len();
    let mut out = String::with_capacity(n);
    let mut i = 0usize;
    let mut in_str = false;
    let mut in_char = false;
    while i < n {
        if in_str {
            if bytes[i] == b'\\' && i + 1 < n {
                out.push('\\');
                let ch = next_char(raw, i + 1);
                out.push(ch);
                i += 1 + ch.len_utf8();
                continue;
            }
            if bytes[i] == b'"' {
                in_str = false;
            }
            let ch = next_char(raw, i);
            out.push(ch);
            i += ch.len_utf8();
            continue;
        }
        if in_char {
            if bytes[i] == b'\\' && i + 1 < n {
                out.push('\\');
                let ch = next_char(raw, i + 1);
                out.push(ch);
                i += 1 + ch.len_utf8();
                continue;
            }
            if bytes[i] == b'\'' {
                in_char = false;
            }
            let ch = next_char(raw, i);
            out.push(ch);
            i += ch.len_utf8();
            continue;
        }
        if bytes[i] == b'"' {
            in_str = true;
            out.push('"');
            i += 1;
            continue;
        }
        if bytes[i] == b'\'' {
            in_char = true;
            out.push('\'');
            i += 1;
            continue;
        }
        if bytes[i] == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
            break;
        }
        if bytes[i] == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
            i += 2;
            while i < n {
                if bytes[i] == b'*' && i + 1 < n && bytes[i + 1] == b'/' {
                    i += 2;
                    break;
                }
                i += 1;
            }
            out.push(' ');
            continue;
        }
        let ch = next_char(raw, i);
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn split_directive_head(trimmed: &str) -> (&str, &str) {
    let end = trimmed
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(trimmed.len());
    (&trimmed[..end], trimmed[end..].trim())
}

// ============================================================================
// Zone-specific lint passes
// ============================================================================

/// Shape-class constant block: only simple scalar `#define NAME <int-literal>` is allowed.
fn scan_shape_constant_tokens(file: &str, tokens: &[Token], findings: &mut Vec<LintFinding>) {
    for tok in tokens {
        match &tok.kind {
            TokenKind::Directive { name, body, .. } if name == "define" => {
                if !is_valid_shape_constant_define(body) {
                    findings.push(LintFinding::new(
                        1,
                        file,
                        tok.line,
                        tok.column,
                        format!("#define {}", body.split_whitespace().next().unwrap_or("")),
                        "shape-class constant block admits only `#define <IDENT> <integer-literal>`",
                    ));
                }
            }
            TokenKind::Directive { name, .. } => {
                findings.push(LintFinding::new(
                    1,
                    file,
                    tok.line,
                    tok.column,
                    format!("#{name}"),
                    "non-#define preprocessor directive is forbidden in shape-class constants",
                ));
            }
            other => {
                findings.push(LintFinding::new(
                    1,
                    file,
                    tok.line,
                    tok.column,
                    token_label(other),
                    "shape-class constant block may contain only `#define` capacity constants",
                ));
            }
        }
    }
}

fn is_valid_shape_constant_define(body: &str) -> bool {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return false;
    }
    let id_end = trimmed
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(trimmed.len());
    let ident = &trimmed[..id_end];
    if ident.is_empty()
        || !ident
            .as_bytes()
            .first()
            .is_some_and(|b| b.is_ascii_alphabetic() || *b == b'_')
    {
        return false;
    }
    let after_ident = &trimmed[id_end..];
    // Reject function-like macro `#define F(x) ...`
    if after_ident.starts_with('(') {
        return false;
    }
    let rhs = after_ident.trim();
    let rhs = rhs
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .map(str::trim)
        .unwrap_or(rhs);
    is_unsigned_or_int_literal(rhs)
}

fn is_unsigned_or_int_literal(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let core = s
        .strip_suffix("u")
        .or_else(|| s.strip_suffix("U"))
        .unwrap_or(s);
    if core.is_empty() {
        return false;
    }
    if let Some(hex) = core.strip_prefix("0x").or_else(|| core.strip_prefix("0X")) {
        !hex.is_empty() && hex.bytes().all(|b| b.is_ascii_hexdigit())
    } else {
        core.bytes().all(|b| b.is_ascii_digit())
    }
}

fn scan_prelude_zone(file: &str, tokens: &[Token], trusted: bool, findings: &mut Vec<LintFinding>) {
    if trusted {
        scan_prelude_tokens(file, tokens, None, findings);
    } else {
        scan_kernel_tokens(file, tokens, None, findings);
    }
}

fn scan_prelude_tokens(
    file: &str,
    tokens: &[Token],
    override_pos: Option<(usize, usize)>,
    findings: &mut Vec<LintFinding>,
) {
    let len = tokens.len();
    let mut i = 0usize;
    while i < len {
        let tok = &tokens[i];
        let (line, col) = override_pos.unwrap_or((tok.line, tok.column));
        match &tok.kind {
            TokenKind::Directive { name, body, .. } if name == "pragma" => {
                if is_forbidden_fp_pragma(body) {
                    findings.push(LintFinding::new(
                        4,
                        file,
                        line,
                        col,
                        format!("#pragma {body}"),
                        "contraction-on or fast-math #pragma is forbidden by dialect rule 4",
                    ));
                }
                i += 1;
            }
            TokenKind::Directive { name, body, .. } if name == "define" => {
                let rhs = skip_define_head(body);
                let inner = tokenize(rhs, false);
                scan_prelude_tokens(file, &inner, Some((line, col)), findings);
                i += 1;
            }
            TokenKind::TokenPaste => {
                findings.push(LintFinding::new(
                    4,
                    file,
                    line,
                    col,
                    "##",
                    "token pasting can form uninspected pragmas or intrinsics (rule 4)",
                ));
                i += 1;
            }
            TokenKind::Ident(id) => {
                if id == "_Pragma" {
                    if let Some((pragma_body, next_i)) = extract_underscore_pragma_arg(tokens, i) {
                        if is_forbidden_fp_pragma(&pragma_body) {
                            findings.push(LintFinding::new(
                                4,
                                file,
                                line,
                                col,
                                format!("_Pragma(\"{pragma_body}\")"),
                                "contraction-on or fast-math `_Pragma` in prelude is forbidden by dialect rule 4",
                            ));
                        }
                        i = next_i;
                        continue;
                    }
                    findings.push(LintFinding::new(
                        4,
                        file,
                        line,
                        col,
                        "_Pragma",
                        "uninspectable `_Pragma` in prelude is forbidden by dialect rule 4",
                    ));
                    i += 1;
                    continue;
                }
                if id == "fast" && i + 1 < len && tokens[i + 1].kind == TokenKind::DoubleColon {
                    let member = if i + 2 < len {
                        match &tokens[i + 2].kind {
                            TokenKind::Ident(m) => Some(m.as_str()),
                            _ => None,
                        }
                    } else {
                        None
                    };
                    let full = match member {
                        Some(m) => format!("fast::{m}"),
                        None => "fast::".to_owned(),
                    };
                    if let Some(m) = member
                        && is_transcendental_or_rsqrt_ident(m)
                    {
                        findings.push(LintFinding::new(
                            3,
                            file,
                            line,
                            col,
                            &full,
                            "built-in transcendental or rsqrt in prelude is forbidden by dialect rule 3",
                        ));
                    }
                    findings.push(LintFinding::new(
                        4,
                        file,
                        line,
                        col,
                        full,
                        "`fast::` math relaxation in prelude is forbidden by dialect rule 4",
                    ));
                    i += if member.is_some() { 3 } else { 2 };
                    continue;
                }
                if is_explicit_fma_ident(id) {
                    findings.push(LintFinding::new(
                        4,
                        file,
                        line,
                        col,
                        id,
                        "explicit fused multiply-add in prelude is forbidden by dialect rule 4",
                    ));
                }
                if is_transcendental_or_rsqrt_ident(id) {
                    findings.push(LintFinding::new(
                        3,
                        file,
                        line,
                        col,
                        id,
                        "built-in transcendental or rsqrt in prelude is forbidden by dialect rule 3",
                    ));
                }
                if is_spin_or_grid_barrier_ident(id) {
                    findings.push(LintFinding::new(
                        6,
                        file,
                        line,
                        col,
                        id,
                        "spin loop or grid-wide barrier in prelude is forbidden by dialect rule 6",
                    ));
                }
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
}

fn extract_underscore_pragma_arg(tokens: &[Token], idx: usize) -> Option<(String, usize)> {
    if idx + 2 >= tokens.len() || tokens[idx + 1].kind != TokenKind::Punct('(') {
        return None;
    }
    let mut j = idx + 2;
    let mut combined = String::new();
    let mut saw_str = false;
    while j < tokens.len() {
        if let TokenKind::StringLit(ref s) = tokens[j].kind {
            combined.push_str(s);
            saw_str = true;
            j += 1;
        } else if let TokenKind::Ident(ref prefix) = tokens[j].kind
            && matches!(prefix.as_str(), "L" | "u8" | "u" | "U")
            && j + 1 < tokens.len()
            && let TokenKind::StringLit(ref s) = tokens[j + 1].kind
        {
            combined.push_str(s);
            saw_str = true;
            j += 2;
        } else {
            break;
        }
    }
    if saw_str && j < tokens.len() && tokens[j].kind == TokenKind::Punct(')') {
        Some((combined, j + 1))
    } else {
        None
    }
}

fn skip_define_head(body: &str) -> &str {
    let trimmed = body.trim();
    let id_end = trimmed
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(trimmed.len());
    let rest = &trimmed[id_end..];
    if let Some(after_lparen) = rest.strip_prefix('(') {
        if let Some(close) = after_lparen.find(')') {
            return after_lparen[close + 1..].trim();
        }
    }
    rest.trim()
}

/// Kernel source file (generated body or hand-written skeleton): enforce rules 1, 2, 3, 4, 6.
fn scan_kernel_tokens(
    file: &str,
    tokens: &[Token],
    inlined_prelude: Option<&str>,
    findings: &mut Vec<LintFinding>,
) {
    let len = tokens.len();
    let mut i = 0usize;

    while i < len {
        let tok = &tokens[i];
        match &tok.kind {
            TokenKind::Directive { name, body, .. } => {
                if name != "include" || !is_prelude_include(body, inlined_prelude) {
                    check_source_directive(file, tok.line, tok.column, name, body, findings);
                }
                i += 1;
            }
            TokenKind::DoubleLBracket => {
                let (attr, next_i) = collect_attribute_text_and_end(tokens, i);
                findings.push(LintFinding::new(
                    1,
                    file,
                    tok.line,
                    tok.column,
                    attr,
                    "backend attribute `[[...]]` is forbidden outside the prelude (rule 1)",
                ));
                i = next_i;
            }
            TokenKind::Ident(id) => {
                // Check namespace-qualified `ns::member` first.
                let next_is_dc = i + 1 < len && tokens[i + 1].kind == TokenKind::DoubleColon;
                if next_is_dc {
                    let member = if i + 2 < len {
                        match &tokens[i + 2].kind {
                            TokenKind::Ident(m) => Some(m.as_str()),
                            _ => None,
                        }
                    } else {
                        None
                    };
                    if handle_qualified_ident(file, tok.line, tok.column, id, member, findings) {
                        // Preserve nested namespace heads like `metal::fast::divide` so
                        // `fast::` / `precise::` is also inspected on the next iteration.
                        let nested_ns = matches!(member, Some("fast" | "precise"))
                            && i + 3 < len
                            && tokens[i + 3].kind == TokenKind::DoubleColon;
                        i += if member.is_some() && !nested_ns { 3 } else { 2 };
                        continue;
                    }
                }

                // `_Pragma(...)` operator outside prelude (also rule 4 if contraction/fast-math)
                if id == "_Pragma" {
                    if let Some((pragma_body, next_i)) = extract_underscore_pragma_arg(tokens, i) {
                        findings.push(LintFinding::new(
                            1,
                            file,
                            tok.line,
                            tok.column,
                            format!("_Pragma(\"{pragma_body}\")"),
                            "`_Pragma` is forbidden outside the prelude (rule 1)",
                        ));
                        if is_forbidden_fp_pragma(&pragma_body) {
                            findings.push(LintFinding::new(
                                4,
                                file,
                                tok.line,
                                tok.column,
                                format!("_Pragma(\"{pragma_body}\")"),
                                "contraction-on or fast-math `_Pragma` is forbidden (rule 4)",
                            ));
                        }
                        i = next_i;
                    } else {
                        findings.push(LintFinding::new(
                            1,
                            file,
                            tok.line,
                            tok.column,
                            "_Pragma",
                            "`_Pragma` is forbidden outside the prelude (rule 1)",
                        ));
                        i += 1;
                    }
                    continue;
                }

                check_source_ident(file, tok.line, tok.column, id, findings);
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
}

fn check_source_directive(
    file: &str,
    line: usize,
    column: usize,
    name: &str,
    body: &str,
    findings: &mut Vec<LintFinding>,
) {
    match name {
        "include" => {
            findings.push(LintFinding::new(
                1,
                file,
                line,
                column,
                format!("#include {body}"),
                "only the bare name of the inlined backend prelude may be included (rule 1)",
            ));
        }
        "define" => {
            findings.push(LintFinding::new(
                1,
                file,
                line,
                column,
                "#define",
                "`#define` is forbidden outside the prelude and shape-class constant block (rule 1)",
            ));
        }
        "pragma" => {
            findings.push(LintFinding::new(
                1,
                file,
                line,
                column,
                format!("#pragma {body}"),
                "`#pragma` is forbidden outside the prelude (rule 1)",
            ));
            if is_forbidden_fp_pragma(body) {
                findings.push(LintFinding::new(
                    4,
                    file,
                    line,
                    column,
                    format!("#pragma {body}"),
                    "contraction-on or fast-math `#pragma` is forbidden (rule 4)",
                ));
            }
        }
        other => {
            findings.push(LintFinding::new(
                1,
                file,
                line,
                column,
                format!("#{other}"),
                "preprocessor directive is forbidden outside the prelude (rule 1)",
            ));
        }
    }
}

fn is_prelude_include(body: &str, inlined_prelude: Option<&str>) -> bool {
    inlined_prelude
        .is_some_and(|name| body.strip_prefix('"').and_then(|s| s.strip_suffix('"')) == Some(name))
}

fn collect_attribute_text_and_end(tokens: &[Token], start: usize) -> (String, usize) {
    let mut out = String::from("[[");
    let mut j = start + 1;
    while j < tokens.len() {
        if tokens[j].kind == TokenKind::Punct(']')
            && j + 1 < tokens.len()
            && tokens[j + 1].kind == TokenKind::Punct(']')
        {
            out.push_str("]]");
            return (out, j + 2);
        }
        if out.len() < 48 {
            out.push_str(&token_label(&tokens[j].kind));
        }
        j += 1;
    }
    // Unclosed `[[`: do not swallow subsequent source tokens.
    (out, start + 1)
}

fn handle_qualified_ident(
    file: &str,
    line: usize,
    column: usize,
    ns: &str,
    member: Option<&str>,
    findings: &mut Vec<LintFinding>,
) -> bool {
    match ns {
        "fast" => {
            let full = match member {
                Some(m) => format!("fast::{m}"),
                None => "fast::".to_owned(),
            };
            if let Some(m) = member
                && is_transcendental_or_rsqrt_ident(m)
            {
                findings.push(LintFinding::new(
                    3,
                    file,
                    line,
                    column,
                    &full,
                    "built-in transcendental or rsqrt under `fast::` is forbidden (rule 3)",
                ));
            }
            findings.push(LintFinding::new(
                4,
                file,
                line,
                column,
                full,
                "`fast::` math relaxation namespace is forbidden (rule 4)",
            ));
            true
        }
        "precise" => {
            let full = match member {
                Some(m) => format!("precise::{m}"),
                None => "precise::".to_owned(),
            };
            if let Some(m) = member {
                if is_transcendental_or_rsqrt_ident(m) {
                    findings.push(LintFinding::new(
                        3,
                        file,
                        line,
                        column,
                        &full,
                        "built-in transcendental or rsqrt under `precise::` is forbidden (rule 3)",
                    ));
                } else if is_explicit_fma_ident(m) {
                    findings.push(LintFinding::new(
                        4,
                        file,
                        line,
                        column,
                        &full,
                        "explicit fused multiply-add under `precise::` is forbidden (rule 4)",
                    ));
                } else {
                    findings.push(LintFinding::new(
                        1,
                        file,
                        line,
                        column,
                        &full,
                        "backend `precise::` namespace is forbidden outside the prelude (rule 1)",
                    ));
                }
            } else {
                findings.push(LintFinding::new(
                    1,
                    file,
                    line,
                    column,
                    full,
                    "backend `precise::` namespace is forbidden outside the prelude (rule 1)",
                ));
            }
            true
        }
        "metal" | "cuda" | "cooperative_groups" | "std" => {
            let full = match member {
                Some(m) => format!("{ns}::{m}"),
                None => format!("{ns}::"),
            };
            findings.push(LintFinding::new(
                1,
                file,
                line,
                column,
                &full,
                "backend namespace is forbidden outside the prelude (rule 1)",
            ));
            if let Some(m) = member {
                if is_transcendental_or_rsqrt_ident(m) {
                    findings.push(LintFinding::new(
                        3,
                        file,
                        line,
                        column,
                        &full,
                        "built-in transcendental or rsqrt is forbidden (rule 3)",
                    ));
                }
                if is_explicit_fma_ident(m) {
                    findings.push(LintFinding::new(
                        4,
                        file,
                        line,
                        column,
                        &full,
                        "explicit fused multiply-add is forbidden (rule 4)",
                    ));
                }
                if is_grid_barrier_ident(m) {
                    findings.push(LintFinding::new(
                        6,
                        file,
                        line,
                        column,
                        &full,
                        "grid-wide barrier is forbidden (rule 6)",
                    ));
                }
                if is_banned_atomic_ident(m) {
                    findings.push(LintFinding::new(
                        2,
                        file,
                        line,
                        column,
                        &full,
                        "only the four FROB_ATOMIC_*_U32 integer macros are admitted (rule 2)",
                    ));
                }
            }
            true
        }
        _ => false,
    }
}

fn check_source_ident(
    file: &str,
    line: usize,
    column: usize,
    id: &str,
    findings: &mut Vec<LintFinding>,
) {
    // Deterministic elementary functions (`det_*`) are explicitly admitted by rule 3.
    if id.starts_with("det_") {
        return;
    }

    // Rule 1: backend intrinsics & qualifiers outside prelude
    if is_rule1_backend_intrinsic(id) {
        findings.push(LintFinding::new(
            1,
            file,
            line,
            column,
            id,
            "backend intrinsic or qualifier is forbidden outside the prelude (rule 1)",
        ));
    }

    // Rule 2: atomics other than the four FROB_ATOMIC_*_U32 macros
    if is_banned_atomic_ident(id) {
        findings.push(LintFinding::new(
            2,
            file,
            line,
            column,
            id,
            "atomic outside {FROB_ATOMIC_ADDW_U32, FROB_ATOMIC_MAX_U32, FROB_ATOMIC_MIN_U32, FROB_ATOMIC_OR_U32} is forbidden (rule 2)",
        ));
    }

    // Rule 3: built-in transcendentals and rsqrt/inversesqrt
    if is_transcendental_or_rsqrt_ident(id) {
        findings.push(LintFinding::new(
            3,
            file,
            line,
            column,
            id,
            "built-in transcendental or reciprocal square root is forbidden; use a det_* function (rule 3)",
        ));
    }

    // Rule 4: explicit FMA / MAD
    if is_explicit_fma_ident(id) {
        findings.push(LintFinding::new(
            4,
            file,
            line,
            column,
            id,
            "explicit fused multiply-add is forbidden by dialect rule 4",
        ));
    }

    // Rule 6: while / do spin loops and grid-wide barriers
    if is_spin_or_grid_barrier_ident(id) {
        findings.push(LintFinding::new(
            6,
            file,
            line,
            column,
            id,
            "while/do spin wait or grid-wide barrier is forbidden; use two launches (rule 6)",
        ));
    }
}

// ============================================================================
// Token classification tables
// ============================================================================

fn is_forbidden_fp_pragma(body: &str) -> bool {
    let lower = strip_comments_inline(body).to_ascii_lowercase();
    let compact: String = lower.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    // Explicit contraction-on / fast / default pragmas
    if compact.contains("fp_contracton") || compact.contains("fp_contractdefault") {
        return true;
    }
    if compact.contains("contract(on)")
        || compact.contains("contract(fast")
        || compact.contains("reassociate(on)")
        || compact.contains("fast_math")
        || compact.contains("fast-math")
        || compact.contains("unsafe_math")
    {
        return true;
    }
    // Check whitespace-separated `stdc fp_contract on` / `default`
    let words: Vec<&str> = lower.split_whitespace().collect();
    for win in words.windows(2) {
        if win[0] == "fp_contract" && matches!(win[1], "on" | "default") {
            return true;
        }
    }
    false
}

fn is_rule1_backend_intrinsic(id: &str) -> bool {
    const EXACT: &[&str] = &[
        // CUDA qualifiers and grid/thread builtins
        "__global__",
        "__device__",
        "__host__",
        "__shared__",
        "__constant__",
        "__managed__",
        "__restrict__",
        "__forceinline__",
        "__noinline__",
        "__launch_bounds__",
        "__syncthreads",
        "__syncthreads_count",
        "__syncthreads_and",
        "__syncthreads_or",
        "__syncwarp",
        "__threadfence",
        "__threadfence_block",
        "__threadfence_system",
        "blockIdx",
        "threadIdx",
        "blockDim",
        "gridDim",
        "warpSize",
        "this_grid",
        "this_thread_block",
        // CUDA reinterpretation and device-only math intrinsics
        "__uint_as_float",
        "__float_as_uint",
        "__int_as_float",
        "__float_as_int",
        "__longlong_as_double",
        "__double_as_longlong",
        "__half_as_short",
        "__short_as_half",
        "__fadd_rn",
        "__fadd_rz",
        "__fadd_ru",
        "__fadd_rd",
        "__fsub_rn",
        "__fmul_rn",
        "__fdiv_rn",
        "__frcp_rn",
        "__fsqrt_rn",
        "__fdividef",
        "__saturatef",
        // Metal qualifiers, barriers, address spaces, and reinterpretation
        "kernel",
        "vertex",
        "fragment",
        "device",
        "constant",
        "threadgroup",
        "threadgroup_imageblock",
        "thread",
        "threadgroup_barrier",
        "simdgroup_barrier",
        "mem_flags",
        "memory_order_relaxed",
        "memory_order_seq_cst",
        "thread_scope_device",
        "as_type",
        "simd_shuffle",
        "simd_shuffle_xor",
        "simd_sum",
        "simd_broadcast",
        "simd_prefix_exclusive_sum",
        "simd_prefix_inclusive_sum",
        // MSL > 3.1 constructs (BD-013 AC6 / N21a-10)
        "bfloat",
        "bfloat16_t",
        "cooperative_tensor",
        "mesh_grid_properties",
        // Inline assembly
        "asm",
        "__asm",
        "__asm__",
    ];

    if EXACT.contains(&id) {
        return true;
    }
    if id.starts_with("atomic_")
        || id.starts_with("__builtin_")
        || id.starts_with("__nv_")
        || id.starts_with("__ptx_")
    {
        return true;
    }
    // CUDA camelCase atomics: `atomicAdd`, `atomicMax`, `atomicCAS`, etc.
    if let Some(rest) = id.strip_prefix("atomic")
        && rest.as_bytes().first().is_some_and(u8::is_ascii_uppercase)
    {
        return true;
    }
    false
}

fn is_banned_atomic_ident(id: &str) -> bool {
    const ALLOWED_FROB_ATOMICS: &[&str] = &[
        "FROB_ATOMIC_ADDW_U32",
        "FROB_ATOMIC_MAX_U32",
        "FROB_ATOMIC_MIN_U32",
        "FROB_ATOMIC_OR_U32",
    ];
    if ALLOWED_FROB_ATOMICS.contains(&id) || id == "frob_atomic_u32" {
        return false;
    }
    if id.starts_with("FROB_ATOMIC_") || id == "frob_atomic_u32_ptr" {
        return true;
    }
    // `atomic_thread_fence` is a barrier (rules 1 & 6), not an arithmetic atomic.
    if id == "atomic_thread_fence" {
        return false;
    }
    if id.starts_with("atomic_") || id.starts_with("nv_atomic_") {
        return true;
    }
    if let Some(rest) = id.strip_prefix("atomic")
        && rest.as_bytes().first().is_some_and(u8::is_ascii_uppercase)
    {
        return true;
    }
    false
}

fn is_transcendental_or_rsqrt_ident(id: &str) -> bool {
    const BASES: &[&str] = &[
        "sin",
        "cos",
        "tan",
        "asin",
        "acos",
        "atan",
        "atan2",
        "sinh",
        "cosh",
        "tanh",
        "asinh",
        "acosh",
        "atanh",
        "sincos",
        "sinpi",
        "cospi",
        "tanpi",
        "exp",
        "exp2",
        "exp10",
        "expm1",
        "log",
        "log2",
        "log10",
        "log1p",
        "pow",
        "powr",
        "cbrt",
        "rcbrt",
        "hypot",
        "rsqrt",
        "inversesqrt",
        "erf",
        "erfc",
        "tgamma",
        "lgamma",
    ];
    const CUDA_FAST: &[&str] = &[
        "__sinf",
        "__cosf",
        "__tanf",
        "__sincosf",
        "__expf",
        "__exp10f",
        "__logf",
        "__log2f",
        "__log10f",
        "__powf",
        "__frsqrt_rn",
        "frsqrt_rn",
    ];

    if CUDA_FAST.contains(&id) {
        return true;
    }
    if BASES.contains(&id) {
        return true;
    }
    if let Some(stem) = id.strip_suffix('f').or_else(|| id.strip_suffix('l'))
        && BASES.contains(&stem)
    {
        return true;
    }
    false
}

fn is_explicit_fma_ident(id: &str) -> bool {
    if matches!(
        id,
        "fma" | "fmaf" | "fmal" | "mad" | "madf" | "dfma" | "ffma"
    ) {
        return true;
    }
    id.starts_with("__fma")
}

fn is_grid_barrier_ident(id: &str) -> bool {
    matches!(
        id,
        "__threadfence"
            | "__threadfence_system"
            | "atomic_thread_fence"
            | "this_grid"
            | "grid_sync"
            | "GRID_SYNC"
            | "grid_barrier"
            | "FROB_GRID_BARRIER"
    )
}

fn is_spin_or_grid_barrier_ident(id: &str) -> bool {
    matches!(id, "while" | "do") || is_grid_barrier_ident(id)
}

fn token_label(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Ident(s) | TokenKind::Number(s) => s.clone(),
        TokenKind::StringLit(_) => "\"...\"".to_owned(),
        TokenKind::InvalidRawString => "raw string literal".to_owned(),
        TokenKind::CharLit => "'...'".to_owned(),
        TokenKind::DoubleColon => "::".to_owned(),
        TokenKind::DoubleLBracket => "[[".to_owned(),
        TokenKind::TokenPaste => "##".to_owned(),
        TokenKind::Punct(c) => c.to_string(),
        TokenKind::Directive { name, .. } => format!("#{name}"),
    }
}

fn write_json_str(s: &str, out: &mut String) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => {
                let b = c as u8;
                out.push_str("\\u00");
                out.push(HEX[usize::from(b >> 4)] as char);
                out.push(HEX[usize::from(b & 0x0f)] as char);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

// ============================================================================
// Unit tests (`cargo test -p frob-kir dialect_lint`)
// ============================================================================

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    fn kernels_dialect_path(rel: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../frob-kernels/dialect")
            .join(rel)
    }

    fn read_dialect(rel: &str) -> String {
        let p = kernels_dialect_path(rel);
        fs::read_to_string(&p).unwrap_or_else(|e| panic!("failed to read {}: {e}", p.display()))
    }

    #[test]
    fn lint_rejects_transcendental() {
        let unit = DialectUnit::single_source("test.h", "f32 y = sinf(x);\n");
        let opts = CompileOptions::cuda_safe("sm_89");
        let findings = collect_findings(&unit, &opts);
        assert_eq!(findings.len(), 1, "expected 1 finding, got {findings:?}");
        assert_eq!(findings[0].rule, 3);
        assert_eq!(findings[0].token, "sinf");
        assert_eq!(findings[0].line, 1);
        assert_eq!(findings[0].column, 9);

        let err = lint(&unit, &opts).unwrap_err();
        assert_eq!(err.code, RefusalCode::FrobRef251);
        assert_eq!(err.retry, RetryPolicy::AfterChange);
        assert!(err.detail.as_deref().unwrap_or("").contains("\"rule\":3"));

        // Qualified `fast::sin` and `precise::exp` are also rejected under rule 3.
        let fast_unit =
            DialectUnit::single_source("q.h", "f32 a = fast::sin(x); f32 b = precise::exp(x);\n");
        let fast_findings = collect_findings(&fast_unit, &opts);
        assert!(
            fast_findings
                .iter()
                .any(|f| f.rule == 3 && f.token == "fast::sin")
        );
        assert!(
            fast_findings
                .iter()
                .any(|f| f.rule == 4 && f.token == "fast::sin")
        );
        assert!(
            fast_findings
                .iter()
                .any(|f| f.rule == 3 && f.token == "precise::exp")
        );

        // `det_*` identifiers are admitted.
        let det_unit = DialectUnit::single_source(
            "det.h",
            "f32 y = det_sin(x) + det_cos(x) + det_rsqrt(x);\n",
        );
        assert!(lint(&det_unit, &opts).is_ok());
    }

    #[test]
    fn lint_rejects_rsqrt() {
        let opts = CompileOptions::cuda_safe("sm_89");
        for (src, tok) in [
            ("f32 a = rsqrt(x);\n", "rsqrt"),
            ("f32 b = inversesqrt(x);\n", "inversesqrt"),
            ("f32 c = rsqrtf(x);\n", "rsqrtf"),
        ] {
            let unit = DialectUnit::single_source("rsqrt.h", src);
            let findings = collect_findings(&unit, &opts);
            assert_eq!(findings.len(), 1, "src={src} findings={findings:?}");
            assert_eq!(findings[0].rule, 3);
            assert_eq!(findings[0].token, tok);
            assert_eq!(
                lint(&unit, &opts).unwrap_err().code,
                RefusalCode::FrobRef251
            );
        }
    }

    #[test]
    fn lint_rejects_float_atomic() {
        let unit = DialectUnit::single_source("atom.h", "atomicAdd(&acc_f32, v);\n");
        let opts = CompileOptions::cuda_safe("sm_89");
        let findings = collect_findings(&unit, &opts);
        let rules: BTreeSet<u8> = findings.iter().map(|f| f.rule).collect();
        assert!(rules.contains(&1), "expected rule 1 in {findings:?}");
        assert!(rules.contains(&2), "expected rule 2 in {findings:?}");
        assert!(findings.iter().all(|f| f.token == "atomicAdd"));
        let err = lint(&unit, &opts).unwrap_err();
        assert_eq!(err.code, RefusalCode::FrobRef251);
    }

    #[test]
    fn lint_rejects_fma_explicit() {
        let opts = CompileOptions::cuda_safe("sm_89");
        for (src, tok) in [
            ("f32 y = fmaf(a, b, c);\n", "fmaf"),
            ("f32 y = fma(a, b, c);\n", "fma"),
            ("f32 y = mad(a, b, c);\n", "mad"),
            ("f32 y = __fmaf_rn(a, b, c);\n", "__fmaf_rn"),
        ] {
            let unit = DialectUnit::single_source("fma.h", src);
            let findings = collect_findings(&unit, &opts);
            assert!(
                findings.iter().any(|f| f.rule == 4 && f.token == tok),
                "expected rule 4 for `{tok}`, got {findings:?}"
            );
            assert_eq!(
                lint(&unit, &opts).unwrap_err().code,
                RefusalCode::FrobRef251
            );
        }
    }

    #[test]
    fn lint_rejects_contraction_option() {
        let unit = DialectUnit::single_source("clean.h", "f32 y = a * b;\n");

        let fmad_true = CompileOptions::Cuda {
            options: vec!["-arch=sm_89".into(), "--fmad=true".into()],
        };
        let f1 = collect_findings(&unit, &fmad_true);
        assert_eq!(f1.len(), 1);
        assert_eq!(f1[0].rule, 4);
        assert_eq!(
            lint(&unit, &fmad_true).unwrap_err().code,
            RefusalCode::FrobRef251
        );

        let fmad_absent = CompileOptions::Cuda {
            options: vec!["-arch=sm_89".into()],
        };
        let f2 = collect_findings(&unit, &fmad_absent);
        assert_eq!(f2.len(), 1);
        assert_eq!(f2[0].rule, 4);
        assert_eq!(
            lint(&unit, &fmad_absent).unwrap_err().code,
            RefusalCode::FrobRef251
        );

        let fmad_false = CompileOptions::Cuda {
            options: vec!["-arch=sm_89".into(), "--fmad=false".into()],
        };
        let report = lint(&unit, &fmad_false).expect("--fmad=false must be accepted");
        assert!(report.findings.is_empty());
    }

    #[test]
    fn lint_rejects_fast_math_option() {
        let unit = DialectUnit::single_source("clean.h", "f32 y = a * b;\n");

        let cuda_fast = CompileOptions::Cuda {
            options: vec![
                "-arch=sm_89".into(),
                "--fmad=false".into(),
                "--use_fast_math".into(),
            ],
        };
        let cf = collect_findings(&unit, &cuda_fast);
        assert_eq!(cf.len(), 1);
        assert_eq!(cf[0].rule, 4);
        assert_eq!(cf[0].token, "--use_fast_math");
        assert_eq!(
            lint(&unit, &cuda_fast).unwrap_err().code,
            RefusalCode::FrobRef251
        );

        let metal_fast = CompileOptions::Metal {
            math_mode: MetalMathMode::Fast,
        };
        let mf = collect_findings(&unit, &metal_fast);
        assert_eq!(mf.len(), 1);
        assert_eq!(mf[0].rule, 4);
        assert_eq!(
            lint(&unit, &metal_fast).unwrap_err().code,
            RefusalCode::FrobRef251
        );
    }

    #[test]
    fn lint_rejects_intrinsic_outside_prelude() {
        let opts = CompileOptions::cuda_safe("sm_89");
        let bad = DialectUnit::single_source("skel.h", "__syncthreads();\n");
        let findings = collect_findings(&bad, &opts);
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert_eq!(findings[0].rule, 1);
        assert_eq!(findings[0].token, "__syncthreads");
        assert_eq!(lint(&bad, &opts).unwrap_err().code, RefusalCode::FrobRef251);

        // The checked-in prelude provides the wrapper around this intrinsic.
        let good = DialectUnit::default()
            .with_prelude(SourceFile::new(
                "prelude_cuda.h",
                read_dialect("prelude_cuda.h"),
            ))
            .with_source(SourceFile::new("skel.h", "FROB_GROUP_BARRIER();\n"));
        assert!(lint(&good, &opts).is_ok());
    }

    #[test]
    fn lint_rejects_define_outside_prelude() {
        let opts = CompileOptions::cuda_safe("sm_89");
        for marker in ["#", "%:"] {
            let shape_define = format!("{marker}define NV_CAP 64u\n");
            let bad = DialectUnit::single_source("skel.h", shape_define.clone());
            let findings = collect_findings(&bad, &opts);
            assert_eq!(findings.len(), 1, "{marker}: got {findings:?}");
            assert_eq!(findings[0].rule, 1);
            assert_eq!(findings[0].token, "#define");
            assert_eq!(lint(&bad, &opts).unwrap_err().code, RefusalCode::FrobRef251);

            let good = DialectUnit::default()
                .with_shape_constants(SourceFile::new("shape.h", shape_define))
                .with_source(SourceFile::new("skel.h", "const u32 n = NV_CAP;\n"));
            assert!(lint(&good, &opts).is_ok());

            let smuggle = DialectUnit::default().with_shape_constants(SourceFile::new(
                "shape.h",
                format!("{marker}define F fmaf\n"),
            ));
            let sf = collect_findings(&smuggle, &opts);
            assert_eq!(sf.len(), 1);
            assert_eq!(sf[0].rule, 1);
        }
    }

    #[test]
    fn lint_rejects_spin_loop() {
        // Plant `BD-013:spin-loop-source` (CX-221: Metal grid-barrier threadgroup stall).
        let opts = CompileOptions::metal_safe();
        let unit =
            DialectUnit::single_source("spin.h", "while (FROB_ATOMIC_MAX_U32(p, 0u) < n) {}\n");
        let findings = collect_findings(&unit, &opts);
        assert_eq!(findings.len(), 1, "got {findings:?}");
        assert_eq!(findings[0].rule, 6);
        assert_eq!(findings[0].token, "while");
        assert_eq!(
            lint(&unit, &opts).unwrap_err().code,
            RefusalCode::FrobRef251
        );

        // `do { ... } while (...);` is also rejected under rule 6.
        let do_unit =
            DialectUnit::single_source("do_spin.h", "do { x = x + 1u; } while (x < n);\n");
        let df = collect_findings(&do_unit, &opts);
        assert!(df.iter().any(|f| f.rule == 6 && f.token == "do"));
        assert!(df.iter().any(|f| f.rule == 6 && f.token == "while"));
    }

    #[test]
    fn lint_accepts_reference_skeleton() {
        let skel = read_dialect("skeletons/k_selftest.h");
        let cuda_pre = read_dialect("prelude_cuda.h");
        let metal_pre = read_dialect("prelude_metal.h");

        let cuda_unit = DialectUnit::default()
            .with_shape_constants(SourceFile::new("shape.h", "#define NV_CAP 64u\n"))
            .with_prelude(SourceFile::new("prelude_cuda.h", cuda_pre))
            .with_source(SourceFile::new("skeletons/k_selftest.h", skel.clone()));
        let r_cuda = lint(&cuda_unit, &CompileOptions::cuda_safe("sm_89"))
            .expect("K-selftest with CUDA prelude must have zero findings");
        assert!(r_cuda.findings.is_empty());

        let metal_unit = DialectUnit::default()
            .with_shape_constants(SourceFile::new("shape.h", "#define NV_CAP 64u\n"))
            .with_prelude(SourceFile::new("prelude_metal.h", metal_pre))
            .with_source(SourceFile::new("skeletons/k_selftest.h", skel));
        let r_metal = lint(&metal_unit, &CompileOptions::metal_safe())
            .expect("K-selftest with Metal prelude must have zero findings");
        assert!(r_metal.findings.is_empty());
    }

    #[test]
    fn lint_reports_positions() {
        let src = concat!(
            "// sinf(x) inside line comment must be ignored\n",
            "/* fmaf(a, b, c)\n",
            "   inside block comment must be ignored */\n",
            "    f32 a = sinf(x);\n",
            "  f32 b = fmaf(a, x, y);\n",
        );
        let unit = DialectUnit::single_source("two_defects.h", src);
        let findings = collect_findings(&unit, &CompileOptions::cuda_safe("sm_89"));
        assert_eq!(findings.len(), 2, "expected 2 findings, got {findings:?}");

        assert_eq!(findings[0].file, "two_defects.h");
        assert_eq!(findings[0].line, 4);
        assert_eq!(findings[0].column, 13);
        assert_eq!(findings[0].rule, 3);
        assert_eq!(findings[0].token, "sinf");

        assert_eq!(findings[1].file, "two_defects.h");
        assert_eq!(findings[1].line, 5);
        assert_eq!(findings[1].column, 11);
        assert_eq!(findings[1].rule, 4);
        assert_eq!(findings[1].token, "fmaf");
    }

    #[test]
    fn prelude_expansions_differ_only_in_prelude() {
        let skel = read_dialect("skeletons/k_selftest.h");
        let cuda_pre = read_dialect("prelude_cuda.h");
        let metal_pre = read_dialect("prelude_metal.h");

        let cuda_unit = DialectUnit::default()
            .with_prelude(SourceFile::new("prelude_cuda.h", cuda_pre))
            .with_source(SourceFile::new("skeletons/k_selftest.h", skel.clone()));
        let metal_unit = DialectUnit::default()
            .with_prelude(SourceFile::new("prelude_metal.h", metal_pre))
            .with_source(SourceFile::new("skeletons/k_selftest.h", skel));

        let exp_cuda = cuda_unit.expand();
        let exp_metal = metal_unit.expand();
        assert_ne!(exp_cuda, exp_metal, "preludes themselves must differ");

        let body_cuda = DialectUnit::body_after_prelude(&exp_cuda).expect("marker present");
        let body_metal = DialectUnit::body_after_prelude(&exp_metal).expect("marker present");
        assert_eq!(body_cuda, body_metal);
        assert_eq!(cuda_unit.body_digest(), metal_unit.body_digest());
        assert_eq!(hash_blob(body_cuda.as_bytes()), cuda_unit.body_digest());
    }

    #[test]
    fn prelude_contraction_pragma_is_rejected_under_rule_4() {
        let opts = CompileOptions::metal_safe();
        let arithmetic = "f32 probe(f32 a, f32 b, f32 c) { return a*b+c; }\n";
        let unit = DialectUnit::single_source("ok.h", arithmetic);
        assert!(lint(&unit, &opts).is_ok());
        for prelude in [
            "#pragma clang fp contract(fast)\n",
            "\u{feff}#pragma clang fp contract(on)\n",
            "_Pragma(\"clang fp contract(/* separator */ on)\")\n",
            "#define DO_PRAGMA _Pra ## gma\nDO_PRAGMA(\"clang fp contract(on)\")\n",
            "#define DO_PRAGMA _Pra %:%: gma\nDO_PRAGMA(\"clang fp contract(on)\")\n",
        ] {
            let bad = unit
                .clone()
                .with_prelude(SourceFile::new("prelude_bad.h", prelude));
            let findings = trusted_prelude_findings(&bad);
            assert_eq!(findings.len(), 1, "{prelude:?}: {findings:?}");
            assert_eq!(findings[0].rule, 4);
            assert_eq!(lint(&bad, &opts).unwrap_err().code, RefusalCode::FrobRef251);
        }
    }

    #[test]
    fn lint_catches_backslash_newline_line_splicing_evasion() {
        let opts = CompileOptions::cuda_safe("sm_89");
        let cases: &[(&str, &[u8], &str)] = &[
            ("f32 y = rsq\\\nrt(x);\n", &[3], "rsqrt"),
            ("f32 y = fm\\\r\naf(a, b, c);\n", &[4], "fmaf"),
            ("f32 y = fm\\\raf(a, b, c);\n", &[4], "fmaf"),
            ("f32 y = fm\\ \t\naf(a, b, c);\n", &[4], "fmaf"),
            (
                "whi\\\nle (FROB_ATOMIC_MAX_U32(p, 0u) < n) {}\n",
                &[6],
                "while",
            ),
            ("atomic\\\nAdd(&acc, v);\n", &[1, 2], "atomicAdd"),
        ];
        for &(src, expected_rules, expected_tok) in cases {
            let unit = DialectUnit::single_source("splice.h", src);
            let findings = collect_findings(&unit, &opts);
            let obs_rules: BTreeSet<u8> = findings.iter().map(|f| f.rule).collect();
            let exp_rules: BTreeSet<u8> = expected_rules.iter().copied().collect();
            assert_eq!(obs_rules, exp_rules, "src={src:?} findings={findings:?}");
            assert!(
                findings
                    .iter()
                    .all(|f| f.token == expected_tok && f.line == 1),
                "src={src:?} findings={findings:?}"
            );
            assert_eq!(
                lint(&unit, &opts).unwrap_err().code,
                RefusalCode::FrobRef251
            );
        }
    }

    #[test]
    fn lint_keeps_findings_after_cpp_digit_separators() {
        let opts = CompileOptions::cuda_safe("sm_89");
        for (src, rule, token) in [
            (
                "f32 probe(f32 a, f32 b, f32 c) { return 1'000.0f + __builtin_fmaf(a,b,c); }",
                1,
                "__builtin_fmaf",
            ),
            (
                "void probe() { u32 n = 1'000u; while (n) { --n; } }",
                6,
                "while",
            ),
        ] {
            let unit = DialectUnit::single_source("digits.h", src);
            let findings = collect_findings(&unit, &opts);
            assert!(
                findings.iter().any(|f| f.rule == rule && f.token == token),
                "{findings:?}"
            );
            assert_eq!(
                lint(&unit, &opts).unwrap_err().code,
                RefusalCode::FrobRef251
            );
        }
    }

    #[test]
    fn lint_catches_multiline_comment_in_pragma_and_underscore_pragma() {
        let opts = CompileOptions::metal_safe();

        // Multi-line block comment inside `#pragma` must not truncate the directive.
        let split_pragma = DialectUnit::single_source(
            "split_pragma.h",
            "#pragma /*\n  comment */ clang fp contract(fast)\nf32 y = a * b;\n",
        );
        let sf = collect_findings(&split_pragma, &opts);
        assert!(sf.iter().any(|f| f.rule == 1), "expected rule 1 in {sf:?}");
        assert!(sf.iter().any(|f| f.rule == 4), "expected rule 4 in {sf:?}");

        // `_Pragma("clang fp contract(fast)")` inside a prelude must still trigger rule 4.
        let bad_pre_pragma = DialectUnit::default()
            .with_prelude(SourceFile::new(
                "prelude_bad.h",
                "_Pragma(\"clang fp \" \"contract(fast)\")\n",
            ))
            .with_source(SourceFile::new("ok.h", "f32 y = a * b;\n"));
        let pf = trusted_prelude_findings(&bad_pre_pragma);
        assert_eq!(pf.len(), 1, "got {pf:?}");
        assert_eq!(pf[0].rule, 4);

        // `_Pragma("STDC FP_CONTRACT ON")` in kernel source triggers both rule 1 and rule 4.
        let src_pragma = DialectUnit::single_source(
            "src_pragma.h",
            "_Pragma(\"STDC FP_CONTRACT ON\")\nf32 y = a * b;\n",
        );
        let spf = collect_findings(&src_pragma, &opts);
        assert!(spf.iter().any(|f| f.rule == 1), "got {spf:?}");
        assert!(spf.iter().any(|f| f.rule == 4), "got {spf:?}");

        // `fast::divide` hidden inside a prelude macro triggers rule 4.
        let bad_pre_fast = DialectUnit::default()
            .with_prelude(SourceFile::new(
                "prelude_fast.h",
                "#define MY_DIV(a, b) fast::divide((a), (b))\n",
            ))
            .with_source(SourceFile::new("ok.h", "f32 y = a * b;\n"));
        let ff = trusted_prelude_findings(&bad_pre_fast);
        assert_eq!(ff.len(), 1, "got {ff:?}");
        assert_eq!(ff[0].rule, 4);
    }

    #[test]
    fn lint_catches_std_namespace_and_single_attribute_finding() {
        let opts = CompileOptions::cuda_safe("sm_89");

        let std_sqrt = DialectUnit::single_source("std.h", "f32 y = std::sqrt(x);\n");
        let f1 = collect_findings(&std_sqrt, &opts);
        assert_eq!(f1.len(), 1, "got {f1:?}");
        assert_eq!(f1[0].rule, 1);
        assert_eq!(f1[0].token, "std::sqrt");

        let std_fma = DialectUnit::single_source("std_fma.h", "f32 y = std::fma(a, b, c);\n");
        let f2 = collect_findings(&std_fma, &opts);
        let rules: BTreeSet<u8> = f2.iter().map(|f| f.rule).collect();
        assert_eq!(rules, BTreeSet::from([1, 4]), "got {f2:?}");

        // `[[threadgroup(0)]]` yields one clean rule-1 finding (not a duplicate on `threadgroup`).
        let attr_unit =
            DialectUnit::single_source("attr.h", "void f(u32 x [[threadgroup(0)]]) {}\n");
        let f3 = collect_findings(&attr_unit, &opts);
        assert_eq!(f3.len(), 1, "expected single attribute finding, got {f3:?}");
        assert_eq!(f3[0].rule, 1);
        assert_eq!(f3[0].token, "[[threadgroup(0)]]");
    }

    #[test]
    fn lint_catches_nested_metal_fast_numeric_cuda_flags_and_include_injection() {
        let safe_cuda = CompileOptions::cuda_safe("sm_89");

        // Nested MSL namespace `metal::fast::divide` must trigger BOTH rule 1 (`metal::`)
        // and rule 4 (`fast::divide`).
        let nested_fast = DialectUnit::single_source("m.h", "f32 y = metal::fast::divide(a, b);\n");
        let nf = collect_findings(&nested_fast, &CompileOptions::metal_safe());
        let nrules: BTreeSet<u8> = nf.iter().map(|f| f.rule).collect();
        assert_eq!(nrules, BTreeSet::from([1, 4]), "got {nf:?}");
        assert!(nf.iter().any(|f| f.rule == 4 && f.token == "fast::divide"));

        // Numeric and two-arg CUDA relaxation flags must trigger rule 4.
        let clean = DialectUnit::single_source("ok.h", "f32 y = a * b;\n");
        for bad_opts in [
            vec!["--fmad=false", "--ftz=1"],
            vec!["--fmad=false", "--prec-div=0"],
            vec!["--fmad=false", "--prec-sqrt=0"],
            vec!["--fmad=false", "--fmad", "true"],
            vec!["--fmad=false", "--ftz", "1"],
            vec!["--fmad=false", "--prec-div", "0"],
        ] {
            let opts = CompileOptions::Cuda {
                options: bad_opts.into_iter().map(str::to_owned).collect(),
            };
            let fs = collect_findings(&clean, &opts);
            assert!(
                fs.iter().any(|f| f.rule == 4),
                "expected rule 4 for {opts:?}, got {fs:?}"
            );
        }

        // Quote-injected and parent-traversal `#include` must fail rule 1.
        for bad_inc in [
            "#include \"evil.h\" \"/prelude_cuda.h\"\n",
            "#include \"../outside/prelude_cuda.h\"\n",
            "#include \"/etc/prelude_metal.h\"\n",
        ] {
            let unit = DialectUnit::single_source("inc.h", bad_inc);
            let fs = collect_findings(&unit, &safe_cuda);
            assert!(
                fs.iter().any(|f| f.rule == 1),
                "expected rule 1 for {bad_inc:?}, got {fs:?}"
            );
        }
    }

    #[test]
    fn lint_catches_line_comment_block_start_wide_pragma_and_cuda_option_injections() {
        let safe_cuda = CompileOptions::cuda_safe("sm_89");
        let safe_metal = CompileOptions::metal_safe();

        // A `// /*` line comment on a `#include` must end at `\n` rather than
        // opening a block comment that swallows the rest of the kernel file.
        let bypass_unit = DialectUnit::single_source(
            "bypass.h",
            "#include \"prelude_cuda.h\" // /*\n\
             void k(f32 x) { f32 y = fmaf(sinf(x), x, x); while (1) {} }\n",
        )
        .with_prelude(SourceFile::new("prelude_cuda.h", ""));
        let bf = collect_findings(&bypass_unit, &safe_cuda);
        let brules: BTreeSet<u8> = bf.iter().map(|f| f.rule).collect();
        assert_eq!(brules, BTreeSet::from([3, 4, 6]), "got {bf:?}");

        // Character literal containing `/*` in a directive must not swallow following lines.
        let char_dir = DialectUnit::single_source(
            "char_dir.h",
            "#define C '/*'\nvoid k(f32 x) { f32 y = sinf(x); }\n",
        );
        let cf = collect_findings(&char_dir, &safe_cuda);
        let crules: BTreeSet<u8> = cf.iter().map(|f| f.rule).collect();
        assert_eq!(crules, BTreeSet::from([1, 3]), "got {cf:?}");

        // ISO C99 wide/UTF `_Pragma(L"...")` and uninspectable `_Pragma(MACRO)` in preludes
        // must both fail rule 4, while benign `_Pragma(L"STDC FP_CONTRACT OFF")` passes.
        let wide_pragma = DialectUnit::default()
            .with_prelude(SourceFile::new(
                "pre_wide.h",
                "_Pragma(L\"clang fp contract(fast)\")\n",
            ))
            .with_source(SourceFile::new("ok.h", "f32 y = a * b;\n"));
        let wf = trusted_prelude_findings(&wide_pragma);
        assert_eq!(wf.len(), 1, "got {wf:?}");
        assert_eq!(wf[0].rule, 4);

        let macro_pragma = DialectUnit::default()
            .with_prelude(SourceFile::new("pre_macro.h", "_Pragma(EVIL_PRAGMA)\n"))
            .with_source(SourceFile::new("ok.h", "f32 y = a * b;\n"));
        let mf = trusted_prelude_findings(&macro_pragma);
        assert_eq!(mf.len(), 1, "got {mf:?}");
        assert_eq!(mf[0].rule, 4);

        let benign_pragma = DialectUnit::default()
            .with_prelude(SourceFile::new(
                "pre_ok.h",
                "_Pragma(L\"STDC FP_CONTRACT OFF\")\n",
            ))
            .with_source(SourceFile::new("ok.h", "f32 y = a * b;\n"));
        assert!(trusted_prelude_findings(&benign_pragma).is_empty());
        assert_eq!(
            lint(&benign_pragma, &safe_metal).unwrap_err().code,
            RefusalCode::FrobRef251
        );

        // CUDA compile-option edge cases: non-false `--fmad=...`, Clang/NVCC fast-math flags,
        // and macro/pre-include injection flags must be refused.
        let clean = DialectUnit::single_source("ok.h", "f32 y = a * b;\n");
        for (bad_opts, expected_rule) in [
            (vec!["--fmad=false", "--fmad=True"], 4),
            (vec!["--fmad=false", "-ffast-math"], 4),
            (vec!["--fmad=false", "-Ofast"], 4),
            (vec!["--fmad=false", "-fapprox-func"], 4),
            (vec!["--fmad=false", "-ffp-contract=fast"], 4),
            (vec!["--fmad=false", "--ftz=True"], 4),
            (vec!["--fmad=false", "--prec-div=no"], 4),
            (vec!["--fmad=false", "-DFROB_SELECT=fmaf"], 1),
            (vec!["--fmad=false", "-D", "EVIL=1"], 1),
            (vec!["--fmad=false", "-UFROB_SQRT"], 1),
            (vec!["--fmad=false", "-include", "evil.h"], 1),
            (vec!["--fmad=false", "--pre-include=evil.h"], 1),
            (vec!["--fmad=false", "--include-path=/unreviewed"], 1),
            (vec!["--fmad=false", "-I/unreviewed"], 1),
        ] {
            let opts = CompileOptions::Cuda {
                options: bad_opts.into_iter().map(str::to_owned).collect(),
            };
            let fs = collect_findings(&clean, &opts);
            assert!(
                fs.iter().any(|f| f.rule == expected_rule),
                "expected rule {expected_rule} for {opts:?}, got {fs:?}"
            );
        }

        // Benign explicit safe CUDA flags remain admitted.
        let explicit_safe = CompileOptions::Cuda {
            options: vec![
                "--fmad=false".to_owned(),
                "--ftz=false".to_owned(),
                "--prec-div=true".to_owned(),
                "--prec-sqrt=true".to_owned(),
                "-ffp-contract=off".to_owned(),
            ],
        };
        assert!(collect_findings(&clean, &explicit_safe).is_empty());
    }

    fn trusted_prelude_findings(unit: &DialectUnit) -> Vec<LintFinding> {
        let pre = unit.prelude.as_ref().expect("prelude fixture");
        let mut findings = Vec::new();
        scan_prelude_tokens(
            &pre.name,
            &tokenize(&pre.contents, true),
            None,
            &mut findings,
        );
        findings
    }
}
