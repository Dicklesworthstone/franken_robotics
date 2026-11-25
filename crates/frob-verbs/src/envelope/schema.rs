//! The compiled `frob.envelope/1` and `frob.progress/1` schemas and the per-verb
//! specialization (N32a-03): `verb` fixed to the entry's name, `result` set to the verb's
//! output schema, `authorization` required on an exporting verb's result and forbidden
//! otherwise, `cursor`/`frontier` forbidden on a verb that is not paginated.

use std::sync::OnceLock;

use crate::registry::{VerbEntry, lookup_schema};

use super::tree::Tree;
use super::validate::{Schema, SchemaError};
use super::{ENVELOPE_SCHEMA_JSON, EnvelopeError, PROGRESS_SCHEMA_JSON};

static BASE: OnceLock<Result<Schema, SchemaError>> = OnceLock::new();
static PROGRESS: OnceLock<Result<Schema, SchemaError>> = OnceLock::new();

fn schema_err(e: &SchemaError) -> EnvelopeError {
    EnvelopeError::Schema(e.to_string())
}

/// The base `frob.envelope/1` schema (`result` unconstrained, `verb` any string).
pub fn base_schema() -> Result<&'static Schema, EnvelopeError> {
    BASE.get_or_init(|| Schema::compile_json(ENVELOPE_SCHEMA_JSON))
        .as_ref()
        .map_err(schema_err)
}

/// The `frob.progress/1` schema.
pub fn progress_schema() -> Result<&'static Schema, EnvelopeError> {
    PROGRESS
        .get_or_init(|| Schema::compile_json(PROGRESS_SCHEMA_JSON))
        .as_ref()
        .map_err(schema_err)
}

/// The envelope schema specialized to a registered verb, its output schema read from the
/// registry's embedded schema table. A verb whose output schema is not embedded is refused.
pub fn envelope_schema(entry: &VerbEntry) -> Result<Schema, EnvelopeError> {
    let id = entry.output_schema.as_str();
    let text = lookup_schema(id).ok_or_else(|| {
        EnvelopeError::Schema(format!(
            "output schema `{id}` of `{}` is not embedded",
            entry.name
        ))
    })?;
    envelope_schema_for(entry, text)
}

/// The envelope schema specialized to `entry` with `output_schema` (JSON Schema text) as
/// the schema of `result`.
pub fn envelope_schema_for(
    entry: &VerbEntry,
    output_schema: &str,
) -> Result<Schema, EnvelopeError> {
    let output = Tree::parse(output_schema.as_bytes()).map_err(|e| {
        EnvelopeError::Schema(format!("output schema: {} at `{}`", e.message, e.pointer))
    })?;
    let doc = specialized_document(entry, output)?;
    Schema::compile(doc).map_err(|e| schema_err(&e))
}

fn members_mut(t: &mut Tree) -> Option<&mut Vec<(String, Tree)>> {
    match t {
        Tree::Obj(m) => Some(m),
        _ => None,
    }
}

fn set(members: &mut Vec<(String, Tree)>, key: &str, value: Tree) {
    if let Some(slot) = members.iter_mut().find(|(k, _)| k == key) {
        slot.1 = value;
    } else {
        members.push((key.to_owned(), value));
    }
}

fn take(members: &mut Vec<(String, Tree)>, key: &str) -> Option<Tree> {
    let i = members.iter().position(|(k, _)| k == key)?;
    Some(members.remove(i).1)
}

fn child_mut<'a>(members: &'a mut [(String, Tree)], key: &str) -> Option<&'a mut Tree> {
    members.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
}

fn obj(pairs: Vec<(&str, Tree)>) -> Tree {
    Tree::Obj(pairs.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
}

fn ints(values: &[i64]) -> Tree {
    Tree::Arr(values.iter().map(|v| Tree::Num(v.to_string())).collect())
}

/// Rewrite every `$ref` of a hoisted output schema into the envelope document's `$defs`.
/// Values of `const`, `enum`, `examples` and `default` are data and are left alone.
fn rewrite_refs(t: &mut Tree) {
    match t {
        Tree::Obj(members) => {
            for (k, v) in members.iter_mut() {
                match k.as_str() {
                    "$ref" => {
                        if let Tree::Str(target) = v {
                            let new = if target.as_str() == "#" {
                                Some("#/$defs/result".to_owned())
                            } else {
                                target
                                    .strip_prefix("#/$defs/")
                                    .map(|name| format!("#/$defs/result.{name}"))
                            };
                            if let Some(n) = new {
                                *target = n;
                            }
                        }
                    }
                    "const" | "enum" | "examples" | "default" => {}
                    _ => rewrite_refs(v),
                }
            }
        }
        Tree::Arr(items) => items.iter_mut().for_each(rewrite_refs),
        _ => {}
    }
}

/// The specialized schema document (JSON), for `frob schema`-style output and for tests.
pub fn specialized_document(entry: &VerbEntry, output_schema: Tree) -> Result<Tree, EnvelopeError> {
    let mut doc = Tree::parse(ENVELOPE_SCHEMA_JSON.as_bytes())
        .map_err(|e| EnvelopeError::Schema(format!("{} at `{}`", e.message, e.pointer)))?;
    let mut output = output_schema;
    let Some(out_members) = members_mut(&mut output) else {
        return Err(EnvelopeError::Schema(format!(
            "output schema of `{}` is not an object",
            entry.name
        )));
    };
    let out_defs = take(out_members, "$defs");
    take(out_members, "$schema");
    take(out_members, "$id");
    rewrite_refs(&mut output);

    let top = members_mut(&mut doc)
        .ok_or_else(|| EnvelopeError::Schema("envelope schema is not an object".into()))?;
    set(
        top,
        "$id",
        Tree::Str(format!("frob.envelope/1;verb={}", entry.name)),
    );
    set(
        top,
        "x-frob-output-schema",
        Tree::Str(entry.output_schema.as_str().to_owned()),
    );

    let defs = child_mut(top, "$defs")
        .and_then(members_mut)
        .ok_or_else(|| EnvelopeError::Schema("no `$defs`".into()))?;
    if defs
        .iter()
        .any(|(k, _)| k == "result" || k.starts_with("result."))
    {
        return Err(EnvelopeError::Schema(
            "envelope `$defs` already names `result`".into(),
        ));
    }
    set(defs, "result", output);
    if let Some(d) = out_defs {
        let Tree::Obj(entries) = d else {
            return Err(EnvelopeError::Schema(
                "output schema `$defs` is not an object".into(),
            ));
        };
        for (name, mut sub) in entries {
            rewrite_refs(&mut sub);
            defs.push((format!("result.{name}"), sub));
        }
    }

    let props = child_mut(top, "properties")
        .and_then(members_mut)
        .ok_or_else(|| EnvelopeError::Schema("no `properties`".into()))?;
    set(
        props,
        "verb",
        obj(vec![("const", Tree::Str(entry.name.to_owned()))]),
    );
    set(
        props,
        "result",
        obj(vec![("$ref", Tree::Str("#/$defs/result".into()))]),
    );
    if !entry.exports {
        set(props, "authorization", Tree::Bool(false));
    }
    if !entry.paginated {
        set(props, "cursor", Tree::Bool(false));
        set(props, "frontier", Tree::Bool(false));
    }
    if entry.exports {
        let all_of =
            child_mut(top, "allOf").ok_or_else(|| EnvelopeError::Schema("no `allOf`".into()))?;
        let Tree::Arr(items) = all_of else {
            return Err(EnvelopeError::Schema("`allOf` is not an array".into()));
        };
        items.push(obj(vec![
            (
                "if",
                obj(vec![(
                    "properties",
                    obj(vec![("exit", obj(vec![("enum", ints(&[0, 2]))]))]),
                )]),
            ),
            (
                "then",
                obj(vec![(
                    "required",
                    Tree::Arr(vec![Tree::Str("authorization".into())]),
                )]),
            ),
        ]));
    }
    Ok(doc)
}
