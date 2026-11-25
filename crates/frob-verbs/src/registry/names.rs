//! Verb name grammar, CLI path splitting, MCP tool-name mapping, and base tools
//! (bead N32a-02, rules RL-01 and RL-02).
//!
//! Grammar:
//! - `segment ::= [a-z][a-z0-9]*("-"[a-z0-9]+)*`
//! - Cluster verb: `segment "." segment` (`cluster` equals the first segment)
//! - Operator verb: `segment` (`cluster` equals `"operator"`)
//! - No registry name contains an underscore (`_`).
//! - MCP tool name replaces `.` and `-` with `_`.

use std::fmt;

/// The three always-listed base MCP tools, excluded from the seven-tool task-cluster cap.
pub const BASE_TOOLS: &[&str] = &["session_focus", "job_wait", "job_cancel"];

/// Canonical verb names that own each base MCP tool.
///
/// An entry whose tool name matches one of [`BASE_TOOLS`] must carry the paired canonical
/// verb name; any other verb name mapping to the same tool name (such as `session-focus`)
/// is a `RL-02` `tool-name-collision`.
pub const BASE_VERB_NAMES: &[(&str, &str)] = &[
    ("session.focus", "session_focus"),
    ("job.wait", "job_wait"),
    ("job.cancel", "job_cancel"),
];

/// Error returned when validating a verb name against its cluster (RL-01).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameError {
    /// Verb name violates segment grammar or contains underscores / extra dots.
    Grammar { name: String, reason: &'static str },
    /// Verb `cluster` field disagrees with the prefix of `name` (or `"operator"` for undotted verbs).
    ClusterMismatch {
        name: String,
        expected_cluster: String,
        observed_cluster: String,
    },
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Grammar { name, reason } => {
                write!(f, "invalid verb name {name:?}: {reason}")
            }
            Self::ClusterMismatch {
                name,
                expected_cluster,
                observed_cluster,
            } => write!(
                f,
                "verb {name:?} has cluster {observed_cluster:?}, expected {expected_cluster:?}"
            ),
        }
    }
}

impl std::error::Error for NameError {}

/// Return `true` iff `seg` matches `^[a-z][a-z0-9]*(-[a-z0-9]+)*$`.
#[must_use]
pub fn is_valid_segment(seg: &str) -> bool {
    let bytes = seg.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_lowercase() {
        return false;
    }
    let mut prev_hyphen = false;
    for &b in &bytes[1..] {
        if b == b'-' {
            if prev_hyphen {
                return false;
            }
            prev_hyphen = true;
        } else if b.is_ascii_lowercase() || b.is_ascii_digit() {
            prev_hyphen = false;
        } else {
            return false;
        }
    }
    !prev_hyphen
}

/// Validate a verb name and its `cluster` field against RL-01.
pub fn validate_verb_name(name: &str, cluster: &str) -> Result<(), NameError> {
    if name.is_empty() {
        return Err(NameError::Grammar {
            name: name.to_owned(),
            reason: "verb name must not be empty",
        });
    }
    if name.contains('_') {
        return Err(NameError::Grammar {
            name: name.to_owned(),
            reason: "underscores are forbidden in registry verb names",
        });
    }

    let mut parts = name.split('.');
    let first = parts.next().unwrap_or("");
    let second = parts.next();
    if parts.next().is_some() {
        return Err(NameError::Grammar {
            name: name.to_owned(),
            reason: "verb name may contain at most one '.' separating cluster and verb",
        });
    }

    if !is_valid_segment(first) {
        return Err(NameError::Grammar {
            name: name.to_owned(),
            reason: "first segment must match ^[a-z][a-z0-9]*(-[a-z0-9]+)*$",
        });
    }

    let expected_cluster = if let Some(verb_seg) = second {
        if !is_valid_segment(verb_seg) {
            return Err(NameError::Grammar {
                name: name.to_owned(),
                reason: "verb segment must match ^[a-z][a-z0-9]*(-[a-z0-9]+)*$",
            });
        }
        first
    } else {
        "operator"
    };

    if cluster != expected_cluster {
        return Err(NameError::ClusterMismatch {
            name: name.to_owned(),
            expected_cluster: expected_cluster.to_owned(),
            observed_cluster: cluster.to_owned(),
        });
    }
    Ok(())
}

/// Map a registry verb name to its MCP tool name by replacing `.` and `-` with `_`.
#[must_use]
pub fn tool_name(verb_name: &str) -> String {
    verb_name
        .chars()
        .map(|c| match c {
            '.' | '-' => '_',
            other => other,
        })
        .collect()
}

/// Split a registry verb name into its CLI command path segments.
#[must_use]
pub fn cli_path(verb_name: &str) -> Vec<&str> {
    verb_name.split('.').collect()
}

/// Return `true` iff `verb_name` is one of the three always-listed base verbs.
#[must_use]
pub fn is_base_verb(verb_name: &str) -> bool {
    BASE_VERB_NAMES
        .iter()
        .any(|&(canonical, _)| canonical == verb_name)
}

/// If `tool` is one of the three reserved base tool names, return its canonical owning verb name.
#[must_use]
pub fn base_tool_owner(tool: &str) -> Option<&'static str> {
    BASE_VERB_NAMES
        .iter()
        .find(|&&(_, base_tool)| base_tool == tool)
        .map(|&(verb, _)| verb)
}
