#![no_main]

use std::fmt::Write as _;

use frob_core::json::{Json, Limits, Spanned, admit};
use libfuzzer_sys::fuzz_target;

/// Plain re-serializer for the fuzz round-trip check (bead N34-17 AC7).
///
/// Writes object members in source order and number lexemes verbatim; this is a
/// test helper only, never the canonical writer (which lands in N34-18).
fn plain_reserialize(node: &Spanned<'_>, out: &mut String) {
    match &node.value {
        Json::Null => out.push_str("null"),
        Json::Bool(true) => out.push_str("true"),
        Json::Bool(false) => out.push_str("false"),
        Json::Num(lex) => out.push_str(lex),
        Json::Str(s) => write_json_string(s, out),
        Json::Arr(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                plain_reserialize(item, out);
            }
            out.push(']');
        }
        Json::Obj(members) => {
            out.push('{');
            for (i, m) in members.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_json_string(&m.name, out);
                out.push(':');
                plain_reserialize(&m.value, out);
            }
            out.push('}');
        }
    }
}

fn write_json_string(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fuzz_target!(|data: &[u8]| {
    let limits = Limits::default();
    if let Ok(admitted) = admit(data, &limits) {
        let mut reserialized = String::new();
        plain_reserialize(admitted.root(), &mut reserialized);
        let re_limits = Limits {
            max_bytes: reserialized.len().max(limits.max_bytes),
            max_depth: limits.max_depth,
            max_string: reserialized.len().max(limits.max_string),
            max_number_len: limits.max_number_len,
        };
        let re_admitted = admit(reserialized.as_bytes(), &re_limits)
            .expect("plain re-serialization of an admitted value must re-admit");
        assert!(
            admitted.value().eq_ordered(re_admitted.value()),
            "re-admitted tree must equal original admitted tree in source order"
        );
    }
});
