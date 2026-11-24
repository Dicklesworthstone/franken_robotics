//! Registry of `std`-only script checks run by `frob-e2e check <name> <args...>`.

use std::collections::BTreeMap;

use crate::e2e::json::{JsonValue, obj};

pub mod single_envelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckOutput {
    pub ok: bool,
    pub observed: String,
    pub expected: String,
    pub first_diff: Option<(String, String, String)>,
}

impl CheckOutput {
    pub fn to_json(&self) -> JsonValue {
        let fd = match &self.first_diff {
            Some((p, o, e)) => obj(vec![
                ("path", JsonValue::String(p.clone())),
                ("observed", JsonValue::String(o.clone())),
                ("expected", JsonValue::String(e.clone())),
            ]),
            None => JsonValue::Null,
        };
        obj(vec![
            ("ok", JsonValue::Bool(self.ok)),
            ("observed", JsonValue::String(self.observed.clone())),
            ("expected", JsonValue::String(self.expected.clone())),
            ("first_diff", fd),
        ])
    }
}

pub type CheckFn = fn(&[String]) -> Result<CheckOutput, String>;

#[derive(Default)]
pub struct CheckRegistry {
    entries: BTreeMap<&'static str, CheckFn>,
}

impl CheckRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a check by name; fails if `name` is already registered.
    pub fn register(&mut self, name: &'static str, func: CheckFn) -> Result<(), String> {
        if self.entries.contains_key(name) {
            return Err(format!("duplicate script check name: {name}"));
        }
        self.entries.insert(name, func);
        Ok(())
    }

    pub fn default_registry() -> Result<Self, String> {
        let mut reg = Self::new();
        reg.register("single-envelope", single_envelope::run)?;
        Ok(reg)
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.entries.keys().copied().collect()
    }

    pub fn run(&self, name: &str, args: &[String]) -> Result<CheckOutput, String> {
        let func = self
            .entries
            .get(name)
            .ok_or_else(|| format!("unknown script check: {name}"))?;
        func(args)
    }
}

pub fn cmd_check(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("usage: frob-e2e check <name> [args...]");
        return 2;
    }
    let reg = match CheckRegistry::default_registry() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("frob-e2e check: registry init failed: {e}");
            return 2;
        }
    };
    match reg.run(&args[0], &args[1..]) {
        Ok(out) => {
            println!("{}", out.to_json().to_json_string());
            if out.ok { 0 } else { 1 }
        }
        Err(e) => {
            eprintln!("frob-e2e check: {e}");
            2
        }
    }
}
