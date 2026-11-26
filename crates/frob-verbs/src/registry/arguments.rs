//! Shared conversion of CLI tokens to the argument object used for schema validation.

use crate::envelope::Tree;

use super::derive::CommonProperty;

/// Parse verb arguments after the command path and process options have been consumed.
pub fn parse_arguments<'a>(
    schema: &Tree,
    common: &[CommonProperty],
    mut positionals: impl Iterator<Item = &'a str>,
    args: &[String],
) -> Result<Tree, (String, &'static str)> {
    let props = schema.get("properties");
    let mut args = args.iter().peekable();
    let mut members: Vec<(String, Tree)> = Vec::new();
    let mut flags = true;
    while let Some(arg) = args.next() {
        if flags && arg == "--" {
            flags = false;
            continue;
        }
        let flag = flags.then(|| arg.strip_prefix("--")).flatten();
        let (name, property, inline) = if let Some(flag) = flag {
            let (flag, inline) = flag
                .split_once('=')
                .map_or((flag, None), |(k, v)| (k, Some(v)));
            if let Some(property) = common
                .iter()
                .find(|p| p.cli_flag.strip_prefix("--") == Some(flag))
            {
                let shape = Tree::parse(property.schema_json.as_bytes())
                    .map_err(|_| (flag.to_owned(), "invalid registry property"))?;
                (property.name.to_owned(), shape, inline)
            } else {
                let (name, shape) = props
                    .and_then(Tree::as_object)
                    .unwrap_or_default()
                    .iter()
                    .find(|(name, _)| name.replace('_', "-") == flag)
                    .ok_or_else(|| (flag.to_owned(), "unknown flag"))?;
                (name.clone(), shape.clone(), inline)
            }
        } else {
            let name = positionals
                .next()
                .ok_or_else(|| ("args".to_owned(), "unexpected positional argument"))?;
            let shape = props
                .and_then(|p| p.get(name))
                .ok_or_else(|| (name.to_owned(), "invalid registry positional"))?;
            (name.to_owned(), shape.clone(), Some(arg.as_str()))
        };

        let kind = property.get("type").and_then(Tree::as_str);
        let value = if flag.is_some() && kind == Some("boolean") && inline.is_none() {
            Some(Tree::Bool(true))
        } else {
            let raw = inline.or_else(|| {
                args.next_if(|next| !next.starts_with("--"))
                    .map(String::as_str)
            });
            raw.and_then(|s| value(s, &property))
        }
        .ok_or_else(|| (name.clone(), "missing or invalid argument value"))?;
        if let Some((_, previous)) = members.iter_mut().find(|(key, _)| key == &name) {
            match (previous, value) {
                (Tree::Arr(prior), Tree::Arr(items)) => prior.extend(items),
                _ => return Err((name, "argument appears twice")),
            }
        } else {
            members.push((name, value));
        }
    }
    Ok(Tree::Obj(members))
}

fn value(raw: &str, schema: &Tree) -> Option<Tree> {
    match schema.get("type").and_then(Tree::as_str) {
        Some("boolean") => raw.parse().ok().map(Tree::Bool),
        Some("number" | "integer") => match Tree::parse(raw.as_bytes()).ok()? {
            n @ Tree::Num(_) => Some(n),
            _ => None,
        },
        Some("array") => {
            let default = Tree::Null;
            raw.split(',')
                .map(|item| value(item, schema.get("items").unwrap_or(&default)))
                .collect::<Option<Vec<_>>>()
                .map(Tree::Arr)
        }
        Some("object") => {
            let default = Tree::Null;
            let mut fields: Vec<(String, Tree)> = Vec::new();
            for pair in raw.split(',') {
                let (key, val) = pair.split_once('=')?;
                if key.is_empty() || fields.iter().any(|(seen, _)| seen == key) {
                    return None;
                }
                let shape = schema
                    .get("properties")
                    .and_then(|p| p.get(key))
                    .or_else(|| schema.get("additionalProperties"))
                    .unwrap_or(&default);
                fields.push((key.to_owned(), value(val, shape)?));
            }
            Some(Tree::Obj(fields))
        }
        _ => Some(Tree::Str(raw.to_owned())),
    }
}
