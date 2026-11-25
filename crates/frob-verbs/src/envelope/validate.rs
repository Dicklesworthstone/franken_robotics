//! Fail-closed JSON Schema (draft 2020-12 subset) validator for `frob.envelope/1`,
//! `frob.progress/1` and the per-verb specialization (N32a-03).
//!
//! Supported keywords: `type`, `enum`, `const`, `properties`, `required`,
//! `additionalProperties`, `patternProperties`, `propertyNames`, `dependentRequired`,
//! `minProperties`, `maxProperties`, `items`, `minItems`, `maxItems`, `uniqueItems`,
//! `pattern`, `minLength`, `maxLength`, `minimum`, `maximum`, `exclusiveMinimum`,
//! `exclusiveMaximum`, `allOf`, `anyOf`, `oneOf`, `not`, `if`/`then`/`else`, and `$ref` to
//! `#` or `#/$defs/<name>` of the same document. Annotations (`$schema`, `$id`, `$comment`,
//! `title`, `description`, `examples`, `default`, `deprecated`, `readOnly`, `writeOnly`, any
//! `x-` key) are ignored. Every other keyword is a compile error: a schema this validator
//! cannot fully enforce is refused instead of being checked partially.

use std::collections::BTreeMap;
use std::fmt;

use super::regex::{Pattern, PatternError};
use super::tree::Tree;

const ANNOTATIONS: &[&str] = &[
    "$schema",
    "$id",
    "$comment",
    "title",
    "description",
    "examples",
    "default",
    "deprecated",
    "readOnly",
    "writeOnly",
];

/// A schema document could not be compiled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaError {
    /// JSON pointer into the schema document.
    pub at: String,
    pub reason: String,
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "schema error at `{}`: {}", self.at, self.reason)
    }
}

impl std::error::Error for SchemaError {}

impl From<PatternError> for SchemaError {
    fn from(e: PatternError) -> Self {
        Self {
            at: String::new(),
            reason: e.to_string(),
        }
    }
}

/// One failed assertion: where in the instance, which keyword, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    /// JSON pointer into the validated instance (the first differing field).
    pub instance: String,
    /// Keyword path inside the schema.
    pub keyword: String,
    pub message: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` fails `{}`: {}",
            self.instance, self.keyword, self.message
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JsonType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    Integer,
    String,
}

impl JsonType {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "null" => Self::Null,
            "boolean" => Self::Boolean,
            "object" => Self::Object,
            "array" => Self::Array,
            "number" => Self::Number,
            "integer" => Self::Integer,
            "string" => Self::String,
            _ => return None,
        })
    }

    fn admits(self, v: &Tree) -> bool {
        match (self, v) {
            (Self::Null, Tree::Null)
            | (Self::Boolean, Tree::Bool(_))
            | (Self::Object, Tree::Obj(_))
            | (Self::Array, Tree::Arr(_))
            | (Self::Number, Tree::Num(_))
            | (Self::String, Tree::Str(_)) => true,
            (Self::Integer, Tree::Num(_)) => v.as_f64().is_some_and(|x| x.fract() == 0.0),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Bool(bool),
    Rules(Box<Rules>),
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Rules {
    types: Option<Vec<JsonType>>,
    enum_values: Option<Vec<Tree>>,
    const_value: Option<Tree>,
    properties: Vec<(String, Node)>,
    required: Vec<String>,
    additional: Option<Node>,
    pattern_properties: Vec<(Pattern, Node)>,
    property_names: Option<Node>,
    dependent_required: Vec<(String, Vec<String>)>,
    min_properties: Option<usize>,
    max_properties: Option<usize>,
    items: Option<Node>,
    min_items: Option<usize>,
    max_items: Option<usize>,
    unique_items: bool,
    pattern: Option<Pattern>,
    min_length: Option<usize>,
    max_length: Option<usize>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    exclusive_minimum: Option<f64>,
    exclusive_maximum: Option<f64>,
    all_of: Vec<Node>,
    any_of: Vec<Node>,
    one_of: Vec<Node>,
    not: Option<Node>,
    if_then_else: Option<(Node, Option<Node>, Option<Node>)>,
    reference: Option<String>,
}

/// A compiled schema document.
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    root: Node,
    defs: BTreeMap<String, Node>,
    document: Tree,
}

impl Schema {
    /// Compile a schema from JSON text.
    pub fn compile_json(text: &str) -> Result<Self, SchemaError> {
        let doc = Tree::parse(text.as_bytes()).map_err(|e| SchemaError {
            at: e.pointer.clone(),
            reason: e.message.clone(),
        })?;
        Self::compile(doc)
    }

    /// Compile a schema document.
    pub fn compile(document: Tree) -> Result<Self, SchemaError> {
        let mut defs = BTreeMap::new();
        if let Some(d) = document.get("$defs") {
            let members = d
                .as_object()
                .ok_or_else(|| err("/$defs", "`$defs` must be an object"))?;
            for (name, sub) in members {
                defs.insert(
                    name.clone(),
                    compile_node(sub, &format!("/$defs/{}", escape_ptr(name)))?,
                );
            }
        }
        let root = compile_node(&document, "")?;
        let schema = Self {
            root,
            defs,
            document,
        };
        schema.check_refs(&schema.root)?;
        for node in schema.defs.values() {
            schema.check_refs(node)?;
        }
        Ok(schema)
    }

    /// The schema document this was compiled from.
    #[must_use]
    pub const fn document(&self) -> &Tree {
        &self.document
    }

    /// Validate `instance`; on failure return every violation (the first is the first
    /// differing field in document order).
    pub fn validate(&self, instance: &Tree) -> Result<(), Vec<Violation>> {
        let mut out = Vec::new();
        self.eval(&self.root, instance, "", "", &mut out, 0);
        if out.is_empty() { Ok(()) } else { Err(out) }
    }

    /// Validate `instance` against the named `$defs` entry.
    pub fn validate_def(&self, def: &str, instance: &Tree) -> Result<(), Vec<Violation>> {
        let node = self.defs.get(def).ok_or_else(|| {
            vec![Violation {
                instance: String::new(),
                keyword: format!("$defs/{def}"),
                message: "no such definition".into(),
            }]
        })?;
        let mut out = Vec::new();
        self.eval(node, instance, "", &format!("/$defs/{def}"), &mut out, 0);
        if out.is_empty() { Ok(()) } else { Err(out) }
    }

    fn check_refs(&self, node: &Node) -> Result<(), SchemaError> {
        let Node::Rules(r) = node else { return Ok(()) };
        if let Some(target) = &r.reference {
            self.resolve(target)
                .ok_or_else(|| err(target, "unresolvable `$ref`"))?;
        }
        let mut children: Vec<&Node> = Vec::new();
        children.extend(r.properties.iter().map(|(_, n)| n));
        children.extend(r.additional.iter());
        children.extend(r.pattern_properties.iter().map(|(_, n)| n));
        children.extend(r.property_names.iter());
        children.extend(r.items.iter());
        children.extend(r.all_of.iter().chain(&r.any_of).chain(&r.one_of));
        children.extend(r.not.iter());
        if let Some((i, t, e)) = &r.if_then_else {
            children.push(i);
            children.extend(t.iter());
            children.extend(e.iter());
        }
        for c in children {
            self.check_refs(c)?;
        }
        Ok(())
    }

    fn resolve(&self, target: &str) -> Option<&Node> {
        if target == "#" {
            return Some(&self.root);
        }
        let name = target.strip_prefix("#/$defs/")?;
        self.defs.get(&unescape_ptr(name))
    }

    #[allow(clippy::too_many_lines)]
    fn eval(
        &self,
        node: &Node,
        v: &Tree,
        ip: &str,
        kp: &str,
        out: &mut Vec<Violation>,
        depth: usize,
    ) {
        let fail = |out: &mut Vec<Violation>, kw: &str, msg: String| {
            out.push(Violation {
                instance: ip.to_owned(),
                keyword: format!("{kp}/{kw}"),
                message: msg,
            });
        };
        if depth > 128 {
            fail(out, "$ref", "schema recursion deeper than 128".into());
            return;
        }
        let r = match node {
            Node::Bool(true) => return,
            Node::Bool(false) => {
                fail(out, "false", "no value is allowed here".into());
                return;
            }
            Node::Rules(r) => r,
        };
        if let Some(target) = &r.reference {
            match self.resolve(target) {
                Some(n) => self.eval(n, v, ip, target.trim_start_matches('#'), out, depth + 1),
                None => fail(out, "$ref", format!("unresolvable `{target}`")),
            }
        }
        if let Some(types) = &r.types
            && !types.iter().any(|t| t.admits(v))
        {
            fail(out, "type", format!("found {}", v.type_name()));
        }
        if let Some(values) = &r.enum_values
            && !values.iter().any(|e| e.same_value(v))
        {
            fail(
                out,
                "enum",
                format!("value {} is not one of the allowed values", short(v)),
            );
        }
        if let Some(c) = &r.const_value
            && !c.same_value(v)
        {
            fail(
                out,
                "const",
                format!("expected {}, found {}", short(c), short(v)),
            );
        }
        match v {
            Tree::Obj(members) => {
                for req in &r.required {
                    if !members.iter().any(|(k, _)| k == req) {
                        out.push(Violation {
                            instance: format!("{ip}/{}", escape_ptr(req)),
                            keyword: format!("{kp}/required"),
                            message: "required member is missing".into(),
                        });
                    }
                }
                for (key, deps) in &r.dependent_required {
                    if members.iter().any(|(k, _)| k == key) {
                        for d in deps {
                            if !members.iter().any(|(k, _)| k == d) {
                                out.push(Violation {
                                    instance: format!("{ip}/{}", escape_ptr(d)),
                                    keyword: format!("{kp}/dependentRequired/{}", escape_ptr(key)),
                                    message: format!("required because `{key}` is present"),
                                });
                            }
                        }
                    }
                }
                if r.min_properties.is_some_and(|n| members.len() < n) {
                    fail(out, "minProperties", format!("{} members", members.len()));
                }
                if r.max_properties.is_some_and(|n| members.len() > n) {
                    fail(out, "maxProperties", format!("{} members", members.len()));
                }
                for (k, mv) in members {
                    let child_ip = format!("{ip}/{}", escape_ptr(k));
                    let mut matched = false;
                    if let Some((_, sub)) = r.properties.iter().find(|(name, _)| name == k) {
                        matched = true;
                        self.eval(
                            sub,
                            mv,
                            &child_ip,
                            &format!("{kp}/properties/{}", escape_ptr(k)),
                            out,
                            depth + 1,
                        );
                    }
                    for (pat, sub) in &r.pattern_properties {
                        if pat.is_match(k) {
                            matched = true;
                            self.eval(
                                sub,
                                mv,
                                &child_ip,
                                &format!("{kp}/patternProperties/{}", escape_ptr(pat.source())),
                                out,
                                depth + 1,
                            );
                        }
                    }
                    if !matched && let Some(add) = &r.additional {
                        if matches!(add, Node::Bool(false)) {
                            out.push(Violation {
                                instance: child_ip.clone(),
                                keyword: format!("{kp}/additionalProperties"),
                                message: "member is not allowed".into(),
                            });
                        } else {
                            self.eval(
                                add,
                                mv,
                                &child_ip,
                                &format!("{kp}/additionalProperties"),
                                out,
                                depth + 1,
                            );
                        }
                    }
                    if let Some(names) = &r.property_names {
                        self.eval(
                            names,
                            &Tree::Str(k.clone()),
                            &child_ip,
                            &format!("{kp}/propertyNames"),
                            out,
                            depth + 1,
                        );
                    }
                }
            }
            Tree::Arr(items) => {
                if r.min_items.is_some_and(|n| items.len() < n) {
                    fail(out, "minItems", format!("{} items", items.len()));
                }
                if r.max_items.is_some_and(|n| items.len() > n) {
                    fail(out, "maxItems", format!("{} items", items.len()));
                }
                if r.unique_items {
                    for (i, a) in items.iter().enumerate() {
                        if items[..i].iter().any(|b| b.same_value(a)) {
                            out.push(Violation {
                                instance: format!("{ip}/{i}"),
                                keyword: format!("{kp}/uniqueItems"),
                                message: "duplicate item".into(),
                            });
                        }
                    }
                }
                if let Some(sub) = &r.items {
                    for (i, item) in items.iter().enumerate() {
                        self.eval(
                            sub,
                            item,
                            &format!("{ip}/{i}"),
                            &format!("{kp}/items"),
                            out,
                            depth + 1,
                        );
                    }
                }
            }
            Tree::Str(s) => {
                let len = s.chars().count();
                if r.min_length.is_some_and(|n| len < n) {
                    fail(out, "minLength", format!("length {len}"));
                }
                if r.max_length.is_some_and(|n| len > n) {
                    fail(out, "maxLength", format!("length {len}"));
                }
                if let Some(p) = &r.pattern
                    && !p.is_match(s)
                {
                    fail(
                        out,
                        "pattern",
                        format!("{} does not match `{}`", short(v), p.source()),
                    );
                }
            }
            Tree::Num(_) => {
                if let Some(x) = v.as_f64() {
                    if r.minimum.is_some_and(|m| x < m) {
                        fail(out, "minimum", format!("{x} is below the minimum"));
                    }
                    if r.maximum.is_some_and(|m| x > m) {
                        fail(out, "maximum", format!("{x} is above the maximum"));
                    }
                    if r.exclusive_minimum.is_some_and(|m| x <= m) {
                        fail(
                            out,
                            "exclusiveMinimum",
                            format!("{x} is not above the bound"),
                        );
                    }
                    if r.exclusive_maximum.is_some_and(|m| x >= m) {
                        fail(
                            out,
                            "exclusiveMaximum",
                            format!("{x} is not below the bound"),
                        );
                    }
                } else {
                    fail(out, "type", "number is not a finite binary64".into());
                }
            }
            Tree::Null | Tree::Bool(_) => {}
        }
        for (i, sub) in r.all_of.iter().enumerate() {
            self.eval(sub, v, ip, &format!("{kp}/allOf/{i}"), out, depth + 1);
        }
        if !r.any_of.is_empty() {
            let passing = r.any_of.iter().filter(|s| self.passes(s, v, depth)).count();
            if passing == 0 {
                fail(out, "anyOf", "no alternative matches".into());
            }
        }
        if !r.one_of.is_empty() {
            let passing = r.one_of.iter().filter(|s| self.passes(s, v, depth)).count();
            if passing != 1 {
                fail(
                    out,
                    "oneOf",
                    format!("{passing} alternatives match, exactly one must"),
                );
            }
        }
        if let Some(n) = &r.not
            && self.passes(n, v, depth)
        {
            fail(out, "not", "value matches a forbidden schema".into());
        }
        if let Some((cond, then, other)) = &r.if_then_else {
            if self.passes(cond, v, depth) {
                if let Some(t) = then {
                    self.eval(t, v, ip, &format!("{kp}/then"), out, depth + 1);
                }
            } else if let Some(e) = other {
                self.eval(e, v, ip, &format!("{kp}/else"), out, depth + 1);
            }
        }
    }

    fn passes(&self, node: &Node, v: &Tree, depth: usize) -> bool {
        let mut scratch = Vec::new();
        self.eval(node, v, "", "", &mut scratch, depth + 1);
        scratch.is_empty()
    }
}

fn err(at: &str, reason: &str) -> SchemaError {
    SchemaError {
        at: at.to_owned(),
        reason: reason.to_owned(),
    }
}

fn short(v: &Tree) -> String {
    let text = v
        .to_canonical()
        .map(|b| b.as_str().to_owned())
        .unwrap_or_else(|_| v.type_name().to_owned());
    if text.chars().count() > 80 {
        let cut: String = text.chars().take(77).collect();
        format!("{cut}...")
    } else {
        text
    }
}

/// Escape one JSON-pointer reference token (`~` → `~0`, `/` → `~1`).
#[must_use]
pub fn escape_ptr(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

fn unescape_ptr(token: &str) -> String {
    token.replace("~1", "/").replace("~0", "~")
}

fn as_count(v: &Tree, at: &str) -> Result<usize, SchemaError> {
    v.as_int53()
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| err(at, "expected a non-negative integer"))
}

fn as_number(v: &Tree, at: &str) -> Result<f64, SchemaError> {
    v.as_f64().ok_or_else(|| err(at, "expected a number"))
}

fn as_string_list(v: &Tree, at: &str) -> Result<Vec<String>, SchemaError> {
    let items = v
        .as_array()
        .ok_or_else(|| err(at, "expected an array of strings"))?;
    items
        .iter()
        .map(|i| {
            i.as_str()
                .map(str::to_owned)
                .ok_or_else(|| err(at, "expected an array of strings"))
        })
        .collect()
}

fn compile_list(v: &Tree, at: &str) -> Result<Vec<Node>, SchemaError> {
    let items = v
        .as_array()
        .ok_or_else(|| err(at, "expected an array of schemas"))?;
    if items.is_empty() {
        return Err(err(at, "empty schema array"));
    }
    items
        .iter()
        .enumerate()
        .map(|(i, s)| compile_node(s, &format!("{at}/{i}")))
        .collect()
}

#[allow(clippy::too_many_lines)]
fn compile_node(v: &Tree, at: &str) -> Result<Node, SchemaError> {
    let members = match v {
        Tree::Bool(b) => return Ok(Node::Bool(*b)),
        Tree::Obj(m) => m,
        _ => return Err(err(at, "a schema must be an object or a boolean")),
    };
    let mut r = Rules::default();
    let (mut if_s, mut then_s, mut else_s) = (None, None, None);
    for (k, val) in members {
        let here = format!("{at}/{}", escape_ptr(k));
        match k.as_str() {
            "type" => {
                let names = match val {
                    Tree::Str(s) => vec![s.clone()],
                    _ => as_string_list(val, &here)?,
                };
                let types = names
                    .iter()
                    .map(|n| JsonType::parse(n).ok_or_else(|| err(&here, "unknown type name")))
                    .collect::<Result<Vec<_>, _>>()?;
                r.types = Some(types);
            }
            "enum" => {
                let items = val
                    .as_array()
                    .ok_or_else(|| err(&here, "`enum` must be an array"))?;
                r.enum_values = Some(items.to_vec());
            }
            "const" => r.const_value = Some(val.clone()),
            "properties" => {
                let props = val
                    .as_object()
                    .ok_or_else(|| err(&here, "`properties` must be an object"))?;
                for (name, sub) in props {
                    r.properties.push((
                        name.clone(),
                        compile_node(sub, &format!("{here}/{}", escape_ptr(name)))?,
                    ));
                }
            }
            "required" => r.required = as_string_list(val, &here)?,
            "additionalProperties" => r.additional = Some(compile_node(val, &here)?),
            "patternProperties" => {
                let props = val
                    .as_object()
                    .ok_or_else(|| err(&here, "`patternProperties` must be an object"))?;
                for (pat, sub) in props {
                    let compiled = Pattern::compile(pat).map_err(|e| err(&here, &e.to_string()))?;
                    r.pattern_properties.push((
                        compiled,
                        compile_node(sub, &format!("{here}/{}", escape_ptr(pat)))?,
                    ));
                }
            }
            "propertyNames" => r.property_names = Some(compile_node(val, &here)?),
            "dependentRequired" => {
                let deps = val
                    .as_object()
                    .ok_or_else(|| err(&here, "`dependentRequired` must be an object"))?;
                for (key, list) in deps {
                    r.dependent_required
                        .push((key.clone(), as_string_list(list, &here)?));
                }
            }
            "minProperties" => r.min_properties = Some(as_count(val, &here)?),
            "maxProperties" => r.max_properties = Some(as_count(val, &here)?),
            "items" => r.items = Some(compile_node(val, &here)?),
            "minItems" => r.min_items = Some(as_count(val, &here)?),
            "maxItems" => r.max_items = Some(as_count(val, &here)?),
            "uniqueItems" => {
                r.unique_items = val
                    .as_bool()
                    .ok_or_else(|| err(&here, "`uniqueItems` must be a boolean"))?;
            }
            "pattern" => {
                let p = val
                    .as_str()
                    .ok_or_else(|| err(&here, "`pattern` must be a string"))?;
                r.pattern = Some(Pattern::compile(p).map_err(|e| err(&here, &e.to_string()))?);
            }
            "minLength" => r.min_length = Some(as_count(val, &here)?),
            "maxLength" => r.max_length = Some(as_count(val, &here)?),
            "minimum" => r.minimum = Some(as_number(val, &here)?),
            "maximum" => r.maximum = Some(as_number(val, &here)?),
            "exclusiveMinimum" => r.exclusive_minimum = Some(as_number(val, &here)?),
            "exclusiveMaximum" => r.exclusive_maximum = Some(as_number(val, &here)?),
            "allOf" => r.all_of = compile_list(val, &here)?,
            "anyOf" => r.any_of = compile_list(val, &here)?,
            "oneOf" => r.one_of = compile_list(val, &here)?,
            "not" => r.not = Some(compile_node(val, &here)?),
            "if" => if_s = Some(compile_node(val, &here)?),
            "then" => then_s = Some(compile_node(val, &here)?),
            "else" => else_s = Some(compile_node(val, &here)?),
            "$ref" => {
                let target = val
                    .as_str()
                    .ok_or_else(|| err(&here, "`$ref` must be a string"))?;
                if target != "#" && !target.starts_with("#/$defs/") {
                    return Err(err(
                        &here,
                        "only `#` and `#/$defs/<name>` references are supported",
                    ));
                }
                r.reference = Some(target.to_owned());
            }
            "$defs" => {
                if !at.is_empty() {
                    return Err(err(&here, "`$defs` is supported only at the document root"));
                }
            }
            other if ANNOTATIONS.contains(&other) || other.starts_with("x-") => {}
            other => return Err(err(&here, &format!("unsupported keyword `{other}`"))),
        }
    }
    match (if_s, then_s, else_s) {
        (Some(i), t, e) => r.if_then_else = Some((i, t, e)),
        (None, None, None) => {}
        (None, _, _) => return Err(err(at, "`then`/`else` without `if`")),
    }
    Ok(Node::Rules(Box::new(r)))
}

#[cfg(test)]
mod tests {
    use super::{Schema, Tree};

    fn tree(s: &str) -> Tree {
        Tree::parse(s.as_bytes()).expect("test JSON parses")
    }

    #[test]
    fn validator_refuses_unknown_keywords_and_remote_refs() {
        for bad in [
            r#"{"format":"date-time"}"#,
            r#"{"$ref":"https://example.org/s.json"}"#,
            r#"{"properties":{"a":{"contains":{}}}}"#,
            r#"{"then":{}}"#,
            r##"{"$ref":"#/$defs/missing"}"##,
            r#"{"pattern":"(?=x)"}"#,
            r#"{"properties":{"a":{"$defs":{}}}}"#,
        ] {
            assert!(Schema::compile_json(bad).is_err(), "{bad} must not compile");
        }
        assert!(
            Schema::compile_json(r#"{"x-frob-identity":"r:","title":"t","type":"object"}"#).is_ok()
        );
    }

    #[test]
    fn validator_core_keywords() {
        let s = Schema::compile_json(
            r#"{"type":"object","required":["a"],"additionalProperties":false,
                "properties":{"a":{"type":"integer","minimum":1},"b":{"enum":["x","y"]},
                              "c":{"type":"array","items":{"type":"string","pattern":"^[a-z]+$"},"uniqueItems":true}},
                "dependentRequired":{"b":["c"]},
                "if":{"properties":{"a":{"const":2}}},"then":{"required":["b"]}}"#,
        )
        .expect("compiles");
        assert!(s.validate(&tree(r#"{"a":1}"#)).is_ok());
        assert!(
            s.validate(&tree(r#"{"a":1.0}"#)).is_ok(),
            "1.0 is an integer in draft 2020-12"
        );
        assert!(s.validate(&tree(r#"{"a":1.5}"#)).is_err());
        assert!(s.validate(&tree(r#"{"a":0}"#)).is_err());
        assert!(s.validate(&tree(r#"{}"#)).is_err());
        assert!(s.validate(&tree(r#"{"a":1,"z":0}"#)).is_err());
        assert!(
            s.validate(&tree(r#"{"a":1,"b":"x"}"#)).is_err(),
            "dependentRequired"
        );
        assert!(s.validate(&tree(r#"{"a":1,"b":"x","c":["p"]}"#)).is_ok());
        assert!(s.validate(&tree(r#"{"a":1,"b":"q","c":["p"]}"#)).is_err());
        assert!(
            s.validate(&tree(r#"{"a":1,"b":"x","c":["p","p"]}"#))
                .is_err()
        );
        assert!(s.validate(&tree(r#"{"a":1,"b":"x","c":["P"]}"#)).is_err());
        assert!(s.validate(&tree(r#"{"a":2}"#)).is_err(), "if/then");
        let v = s.validate(&tree(r#"{"a":1,"z":0}"#)).unwrap_err();
        assert_eq!(v[0].instance, "/z");
    }

    #[test]
    fn validator_combinators_and_refs() {
        let s = Schema::compile_json(
            r##"{"$defs":{"pos":{"type":"number","exclusiveMinimum":0}},
                 "oneOf":[{"$ref":"#/$defs/pos"},{"type":"string"}],
                 "not":{"const":"forbidden"}}"##,
        )
        .expect("compiles");
        assert!(s.validate(&tree("3")).is_ok());
        assert!(s.validate(&tree("0")).is_err());
        assert!(s.validate(&tree(r#""ok""#)).is_ok());
        assert!(s.validate(&tree(r#""forbidden""#)).is_err());
        assert!(s.validate(&tree("null")).is_err());
        assert!(s.validate_def("pos", &tree("2")).is_ok());
        assert!(s.validate_def("pos", &tree("-2")).is_err());
    }
}
