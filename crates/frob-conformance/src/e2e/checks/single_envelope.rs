//! Built-in script check `single-envelope`: verifies that a file (or stdin)
//! holds exactly one JSON object followed only by optional RFC 8259 whitespace.

use crate::e2e::checks::CheckOutput;
use crate::e2e::json::{self, JsonValue};
use crate::e2e::line::read_input;

pub fn check_str(input: &str) -> CheckOutput {
    let trimmed_start = json::skip_ws(input.as_bytes(), 0);
    if trimmed_start == input.len() {
        return CheckOutput {
            ok: false,
            observed: "empty input".into(),
            expected: "single JSON object".into(),
            first_diff: Some(("byte:0".into(), "empty".into(), "object".into())),
        };
    }

    let (val, end) = match json::parse_prefix_with_offset(input) {
        Ok(pair) => pair,
        Err(e) => {
            return CheckOutput {
                ok: false,
                observed: e.to_string(),
                expected: "single JSON object".into(),
                first_diff: Some((
                    format!("byte:{}", e.offset),
                    e.message,
                    "valid JSON object".into(),
                )),
            };
        }
    };

    if !matches!(val, JsonValue::Object(_)) {
        let kind = match val {
            JsonValue::Null => "null",
            JsonValue::Bool(_) => "boolean",
            JsonValue::Number(_) => "number",
            JsonValue::String(_) => "string",
            JsonValue::Array(_) => "array",
            JsonValue::Object(_) => "object",
        };
        return CheckOutput {
            ok: false,
            observed: format!("JSON {kind}"),
            expected: "single JSON object".into(),
            first_diff: Some((
                format!("byte:{trimmed_start}"),
                kind.into(),
                "object".into(),
            )),
        };
    }

    let trailing = json::skip_ws(input.as_bytes(), end);
    if trailing < input.len() {
        let snippet: String = input[trailing..].chars().take(32).collect();
        return CheckOutput {
            ok: false,
            observed: format!("trailing content at byte {trailing}"),
            expected: "end-of-input".into(),
            first_diff: Some((
                format!("byte:{trailing}"),
                format!("byte {trailing}: {snippet}"),
                "end-of-input".into(),
            )),
        };
    }

    CheckOutput {
        ok: true,
        observed: "1 JSON object".into(),
        expected: "1 JSON object".into(),
        first_diff: None,
    }
}

pub fn run(args: &[String]) -> Result<CheckOutput, String> {
    if args.len() > 1 {
        return Err("usage: frob-e2e check single-envelope [<file>]".into());
    }
    let text = read_input(args.first().map(String::as_str))?;
    Ok(check_str(&text))
}
