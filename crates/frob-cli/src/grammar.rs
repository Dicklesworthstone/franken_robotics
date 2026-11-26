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
    let positionals = schema
        .get("x-frob-cli")
        .and_then(|cli| cli.get("positional"))
        .and_then(Tree::as_array)
        .unwrap_or_default()
        .iter()
        .filter_map(Tree::as_str);
    parse_arguments(schema, &entry.common_properties(), positionals, args)
}
