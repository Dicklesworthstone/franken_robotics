use frob_verbs::envelope::Tree;
use frob_verbs::registry::{VerbEntry, cli_path, parse_arguments};

type ArgumentError = (String, &'static str);

pub fn locate<'a>(entries: &'a [VerbEntry], args: &[String]) -> Option<(&'a VerbEntry, usize)> {
    entries.iter().find_map(|entry| {
        let path = cli_path(entry.name);
        args.get(..path.len())
            .is_some_and(|prefix| prefix.iter().map(String::as_str).eq(path.iter().copied()))
            .then_some((entry, path.len()))
    })
}

pub fn parse(entry: &VerbEntry, schema: &Tree, args: &[String]) -> Result<Tree, ArgumentError> {
    parse_arguments(schema, &entry.common_properties(), args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use frob_verbs::registry::lookup;

    #[test]
    fn cli_parses_all_registered_positional_forms() {
        let entry = lookup("doctor").unwrap();
        for schema in [
            r#"{"x-frob-cli":{"positional":["section"]},"properties":{"section":{"type":"string","x-frob-positional":true}}}"#,
            r#"{"x-frob-positionals":["section"],"properties":{"section":{"type":"string"}}}"#,
            r#"{"properties":{"section":{"type":"string","x-frob-positional":true}}}"#,
        ] {
            let doc = Tree::parse(schema.as_bytes()).unwrap();
            let parsed = parse(entry, &doc, &["stack".into()]).unwrap();
            assert_eq!(parsed.get("section").and_then(Tree::as_str), Some("stack"));
        }
    }
}
