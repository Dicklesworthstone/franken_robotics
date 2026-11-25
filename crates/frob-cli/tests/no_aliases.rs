//! NJL-04: the schema-driven CLI exposes a single spelling per verb.

use std::{env, fs, path::Path};

const ALIASES: &[&str] = &["alias", "visible_alias", "aliases"];

fn alias_attribute(source: &str) -> Option<&str> {
    source
        .split("#[")
        .skip(1)
        .filter_map(|attribute| attribute.split_once(']').map(|(body, _)| body))
        .flat_map(|body| body.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_'))
        .find(|token| ALIASES.contains(token))
}

fn check_source(path: &Path) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            check_source(&entry.unwrap().path());
        }
    } else if path.extension().is_some_and(|ext| ext == "rs") {
        let source = fs::read_to_string(path).unwrap();
        if let Some(attribute) = alias_attribute(&source) {
            panic!("surface-alias: {attribute} in {}", path.display());
        }
    }
}

#[test]
fn no_alias_attributes() {
    check_source(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"));
    if let Some(plant) = env::var_os("FROB_CLI_ALIAS_PLANT") {
        check_source(Path::new(&plant));
    }
}

#[test]
fn visible_alias_is_caught() {
    assert_eq!(
        alias_attribute("#[command(\n    visible_alias = \"second\"\n)]\nfn generated() {}"),
        Some("visible_alias")
    );
}
