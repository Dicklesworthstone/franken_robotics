//! M0 operator handlers and registry-backed help shared by the surfaces.

use frob_core::{
    canon,
    evidence::{
        ClaimKind, ClaimProduct, DetClass, FlagSet, InputRef, RealityScope, Scope, TierMeet,
    },
    id::hash_blob,
    refusal::{Repair, RetryPolicy},
    registry::refusals::{RefusalRegistry, RefusalRow},
    stack::BUILT_UNDER,
};

use crate::{
    dispatch::{
        DispatchCx, HandlerContext, HandlerTable, ProgressEmit, ResolvedRequest,
        in_process::FnHandler, pipeline::edit_distance,
    },
    envelope::{
        Cost, ENVELOPE_SCHEMA_ID, ENVELOPE_SCHEMA_JSON, Envelope, EvidenceBlock, ExitCode,
        HandlerFault, JsonBody, PROGRESS_SCHEMA_ID, PROGRESS_SCHEMA_JSON, RefusalFields, Schema,
        Tree, escape_ptr, specialized_document,
    },
    handlers,
    registry::{
        REGISTRY_DIGEST_HEX, SCHEMAS, VERBS, VerbEntry, entry_schema_json, lookup,
        lookup_entry_json, lookup_input_schema, lookup_schema,
    },
};

fn object<const N: usize>(members: [(&str, Tree); N]) -> Tree {
    Tree::Obj(
        members
            .into_iter()
            .map(|(key, val)| (key.into(), val))
            .collect(),
    )
}

fn text(s: impl ToString) -> Tree {
    Tree::Str(s.to_string())
}

fn array<I: IntoIterator<Item = Tree>>(items: I) -> Tree {
    Tree::Arr(items.into_iter().collect())
}

fn unmapped(error: impl ToString) -> HandlerFault {
    HandlerFault::Unmapped(error.to_string())
}

pub fn argument_fields(parameter: &str, reason: &str) -> RefusalFields {
    RefusalFields {
        message: reason.into(),
        pointer: format!("/{}", escape_ptr(parameter)),
        observed: parameter.into(),
        required: "an argument allowed by the verb input schema".into(),
        unit: String::new(),
        detail: None,
        affected_claims: vec![],
        retry: RetryPolicy::AfterChange,
        repairs: vec![Repair::new("Pass --help to see the accepted arguments.")],
    }
}

fn bad_argument(parameter: &str, reason: &str) -> HandlerFault {
    HandlerFault::Refused {
        code: "FROB-REF-802".into(),
        fields: argument_fields(parameter, reason),
    }
}

fn unknown_schema_name(parameter: &str, name: &str) -> HandlerFault {
    let mut candidates: Vec<_> = VERBS
        .iter()
        .map(|entry| entry.name)
        .chain(SCHEMAS.iter().map(|(id, _)| id.as_str()))
        .chain([ENVELOPE_SCHEMA_ID, PROGRESS_SCHEMA_ID, "frob.verbs.entry/1"])
        .map(|candidate| (edit_distance(name, candidate), candidate))
        .collect();
    candidates.sort_unstable();
    let mut fields = argument_fields(parameter, "unknown schema identifier or verb");
    fields.observed = candidates
        .into_iter()
        .take(3)
        .map(|(_, name)| name)
        .collect::<Vec<_>>()
        .join(", ");
    HandlerFault::Refused {
        code: "FROB-REF-802".into(),
        fields,
    }
}

fn schema_tree(raw: &str) -> Result<Tree, HandlerFault> {
    Tree::parse(raw.as_bytes()).map_err(|e| unmapped(e.message))
}

fn raw_input(entry: &VerbEntry) -> Result<&'static str, HandlerFault> {
    lookup_input_schema(entry.name).ok_or_else(|| unmapped("input schema missing"))
}

fn completed(verb: &str, body: &Tree) -> Result<Envelope, HandlerFault> {
    complete(verb, body, false)
}

fn complete(verb: &str, body: &Tree, findings: bool) -> Result<Envelope, HandlerFault> {
    let body = JsonBody::from_term(body).map_err(unmapped)?;
    let product = ClaimProduct::estimated(
        ClaimKind::KnownAnswer,
        InputRef::new(hash_blob(REGISTRY_DIGEST_HEX.as_bytes()), "/result"),
        Scope::default(),
        TierMeet::Static,
        DetClass::D0Exact,
        RealityScope::NoClaim,
        FlagSet::new(),
    );
    let evidence = EvidenceBlock::from_claim_product(&product, &format!("m:{REGISTRY_DIGEST_HEX}"))
        .map_err(unmapped)?;
    let build = if findings {
        Envelope::completed_with_findings
    } else {
        Envelope::completed
    };
    build(verb, body, evidence, Cost::ZERO).map_err(unmapped)
}

type Operation = fn(&ResolvedRequest, &RefusalRegistry) -> Result<Tree, HandlerFault>;
const BINDINGS: &[(&str, Operation)] = &[
    ("capabilities", capabilities),
    ("robot-docs", robot_docs),
    ("schema", schema),
];

pub(crate) fn register(table: &mut HandlerTable) {
    doctor::register(table, &[]);
    for &(name, operation) in BINDINGS {
        table.register(
            name,
            std::sync::Arc::new(FnHandler(
                move |cx: &DispatchCx,
                      request: &ResolvedRequest,
                      context: &HandlerContext<'_>,
                      _: &mut dyn ProgressEmit| {
                    cx.checkpoint().map_err(unmapped)?;
                    let body = operation(request, context.refusals)?;
                    completed(&request.verb, &body)
                },
            )),
        );
    }
}

fn argument<'a>(req: &'a ResolvedRequest, key: &str) -> Option<&'a str> {
    req.args.get(key).and_then(Tree::as_str)
}

fn enabled(req: &ResolvedRequest, key: &str) -> bool {
    req.args.get(key).and_then(Tree::as_bool) == Some(true)
}

pub mod doctor {
    use super::*;

    /// The result of a read-only installation check.
    #[derive(Default)]
    pub struct SectionReport {
        pub rows: Vec<(String, Tree)>,
        pub findings: Vec<Tree>,
        pub skipped: Option<String>,
    }

    pub trait DoctorSection: Send + Sync {
        fn name(&self) -> &'static str;
        fn run(&self, cx: &DispatchCx) -> Result<SectionReport, HandlerFault>;
    }

    struct StackReceipt;

    impl DoctorSection for StackReceipt {
        fn name(&self) -> &'static str {
            "stack_receipt"
        }

        fn run(&self, _: &DispatchCx) -> Result<SectionReport, HandlerFault> {
            let bytes = canon::to_canonical(&BUILT_UNDER).map_err(unmapped)?;
            let Tree::Obj(rows) = schema_tree(bytes.as_str())? else {
                return Err(unmapped("stack identity is not an object"));
            };
            Ok(SectionReport {
                rows,
                ..SectionReport::default()
            })
        }
    }

    /// Bind doctor with the built-in checks and the caller's extra sections.
    pub fn register(table: &mut HandlerTable, extra: &'static [&'static dyn DoctorSection]) {
        table.register(
            "doctor",
            std::sync::Arc::new(FnHandler(
                move |cx: &DispatchCx,
                      request: &ResolvedRequest,
                      _: &HandlerContext<'_>,
                      _: &mut dyn ProgressEmit| {
                    let mut sections: Vec<&dyn DoctorSection> = vec![&StackReceipt];
                    sections.extend(extra.iter().copied());
                    sections.sort_unstable_by_key(|section| section.name());
                    let selected = request
                        .args
                        .get("section")
                        .and_then(Tree::as_array)
                        .unwrap_or_default();
                    if selected.iter().any(|name| {
                        !sections
                            .iter()
                            .any(|section| name.as_str() == Some(section.name()))
                    }) {
                        return Err(bad_argument("section", "unknown doctor section"));
                    }
                    let mut rows = Vec::new();
                    let mut has_findings = false;
                    for section in sections {
                        let name = section.name();
                        if !selected.is_empty()
                            && !selected.iter().any(|item| item.as_str() == Some(name))
                        {
                            continue;
                        }
                        cx.checkpoint().map_err(unmapped)?;
                        let SectionReport {
                            rows: mut report,
                            findings,
                            skipped,
                        } = section.run(cx)?;
                        has_findings |= findings.iter().any(|finding| {
                            matches!(
                                finding.get("severity").and_then(Tree::as_str),
                                Some("low" | "high")
                            )
                        });
                        let status = match skipped {
                            Some(reason) => {
                                report.push(("reason".into(), text(reason)));
                                "skipped"
                            }
                            None if findings.is_empty() => "ok",
                            None => "findings",
                        };
                        report.extend([
                            ("status".into(), text(status)),
                            ("findings".into(), array(findings)),
                        ]);
                        rows.push((name.to_owned(), Tree::Obj(report)));
                    }
                    complete(&request.verb, &Tree::Obj(rows), has_findings)
                },
            )),
        );
    }
}

pub trait StatusSource {
    fn status(&self, verb: &VerbEntry) -> VerbStatus;
}

pub struct VerbStatus {
    pub linked: bool,
    pub state: &'static str,
    pub conditions: Vec<(u8, &'static str)>,
}

impl StatusSource for HandlerTable {
    fn status(&self, verb: &VerbEntry) -> VerbStatus {
        if self.contains(verb.name) {
            VerbStatus {
                linked: true,
                state: "partial",
                conditions: (2..=8).map(|number| (number, "not-evaluated")).collect(),
            }
        } else {
            VerbStatus {
                linked: false,
                state: "not-in-build",
                conditions: vec![(1, "missing")],
            }
        }
    }
}

fn capabilities(req: &ResolvedRequest, _: &RefusalRegistry) -> Result<Tree, HandlerFault> {
    let filter = argument(req, "verb");
    if filter.is_some_and(|name| lookup(name).is_none()) {
        return Err(bad_argument("verb", "unknown verb filter"));
    }
    Ok(capabilities_report(&handlers::handler_table(), filter))
}

pub fn capabilities_report(source: &dyn StatusSource, filter: Option<&str>) -> Tree {
    let rows = VERBS
        .iter()
        .filter(|entry| filter.is_none_or(|name| entry.name == name))
        .map(|entry| {
            let status = source.status(entry);
            object([
                ("verb", text(entry.name)),
                ("linked", Tree::Bool(status.linked)),
                ("kind", text(entry.kind)),
                ("tier", text(entry.tier)),
                ("examples", array(entry.examples.iter().map(text))),
                ("first_milestone", text(entry.first_milestone)),
                ("status", text(status.state)),
                (
                    "conditions",
                    array(status.conditions.into_iter().map(|(number, state)| {
                        object([
                            ("condition", Tree::Num(number.to_string())),
                            ("status", text(state)),
                        ])
                    })),
                ),
            ])
        });
    object([
        ("registry_digest", text(REGISTRY_DIGEST_HEX)),
        ("capabilities", array(rows)),
    ])
}

fn refusal_row(row: &RefusalRow) -> [(&'static str, Tree); 6] {
    [
        ("code", text(&row.code)),
        ("meaning", text(&row.meaning)),
        ("exit", Tree::Num(row.exit.to_string())),
        ("retry", text(row.retry.as_str())),
        ("repairs", array(row.repairs.iter().map(text))),
        ("defined_in", text(&row.defined_in)),
    ]
}

fn full_refusals(entry: &VerbEntry, reg: &RefusalRegistry) -> Result<Tree, HandlerFault> {
    let rows = entry
        .all_refusals_with(raw_input(entry)?, reg)
        .into_iter()
        .map(|item| {
            let row = item
                .typed()
                .and_then(|code| reg.row(code))
                .ok_or_else(|| unmapped("refusal row missing"))?;
            let mut members = Vec::from(refusal_row(row));
            members.push(("source", text(item.source)));
            Ok(Tree::Obj(
                members
                    .into_iter()
                    .map(|(name, val)| (name.into(), val))
                    .collect(),
            ))
        })
        .collect::<Result<Vec<_>, HandlerFault>>()?;
    Ok(array(rows))
}

fn entry_document(entry: &VerbEntry) -> Result<Tree, HandlerFault> {
    let raw = lookup_entry_json(entry.name).ok_or_else(|| unmapped("registry entry missing"))?;
    schema_tree(raw)
}

pub fn help(entry: &VerbEntry, reg: &RefusalRegistry) -> Result<Envelope, HandlerFault> {
    let raw = raw_input(entry)?;
    let merged = entry
        .input_schema_with_common_fields_json(raw)
        .map_err(unmapped)?;
    let output = lookup_schema(entry.output_schema.as_str())
        .ok_or_else(|| unmapped("output schema missing"))?;
    let body = object([
        ("entry", entry_document(entry)?),
        ("input_schema", schema_tree(&merged)?),
        (
            "output_schema",
            specialized_document(entry, schema_tree(output)?).map_err(unmapped)?,
        ),
        ("refusals", full_refusals(entry, reg)?),
        ("examples", array(entry.examples.iter().map(text))),
    ]);
    completed(entry.name, &body)
}

fn robot_docs(req: &ResolvedRequest, reg: &RefusalRegistry) -> Result<Tree, HandlerFault> {
    let filter = argument(req, "cluster");
    let verbs = VERBS
        .iter()
        .filter(|entry| filter.is_none_or(|name| entry.task_clusters.contains(&name)))
        .map(|entry| {
            Ok(object([
                ("entry", entry_document(entry)?),
                ("refusals", full_refusals(entry, reg)?),
            ]))
        })
        .collect::<Result<Vec<_>, HandlerFault>>()?;
    Ok(object([
        ("verbs", array(verbs)),
        ("envelope", schema_tree(ENVELOPE_SCHEMA_JSON)?),
        (
            "exits",
            array(ExitCode::ALL.iter().map(|exit| {
                object([
                    ("exit", Tree::Num(exit.code().to_string())),
                    ("meaning", text(exit.meaning())),
                ])
            })),
        ),
        (
            "surface_claim",
            text(
                "every verb carries a compute budget, provenance, and a typed refusal, and both CLI and MCP surfaces are generated from one registry.",
            ),
        ),
    ]))
}

fn schema(req: &ResolvedRequest, reg: &RefusalRegistry) -> Result<Tree, HandlerFault> {
    let id = argument(req, "id");
    let verb = argument(req, "verb");
    let select_in = enabled(req, "in");
    let select_out = enabled(req, "out");
    let list = ["refusals", "verbs", "path_args"]
        .into_iter()
        .filter(|key| enabled(req, key))
        .collect::<Vec<_>>();
    if usize::from(id.is_some()) + usize::from(verb.is_some()) + list.len() > 1
        || select_in && select_out
        || (select_in || select_out)
            && (id.or(verb).is_none() || id.is_some_and(|id| id.contains('/')))
    {
        return Err(bad_argument("args", "choose one schema form"));
    }
    if let Some(name) = verb.or(id) {
        if verb.is_none() && name.contains('/') {
            if name == "frob.verbs.entry/1" {
                return schema_with_example(schema_tree(&entry_schema_json())?);
            }
            let raw = match name {
                ENVELOPE_SCHEMA_ID => Some(ENVELOPE_SCHEMA_JSON),
                PROGRESS_SCHEMA_ID => Some(PROGRESS_SCHEMA_JSON),
                _ => lookup_schema(name),
            };
            if raw.is_none() {
                let repairs: Vec<_> = VERBS
                    .iter()
                    .filter(|entry| entry.input_schema.as_str() == name)
                    .map(|entry| Repair::new(format!("frob schema --verb {} --in", entry.name)))
                    .collect();
                if !repairs.is_empty() {
                    let mut fields = argument_fields("id", "ambiguous input schema identifier");
                    fields.observed = name.to_owned();
                    fields.required = "a verb-qualified input schema selection".into();
                    fields.repairs = repairs;
                    return Err(HandlerFault::Refused {
                        code: "FROB-REF-802".into(),
                        fields,
                    });
                }
            }
            return schema_with_example(schema_tree(
                raw.ok_or_else(|| unknown_schema_name("id", name))?,
            )?);
        }
        let entry = lookup(name)
            .ok_or_else(|| unknown_schema_name(if verb.is_some() { "verb" } else { "id" }, name))?;
        let input = || {
            schema_tree(
                &entry
                    .input_schema_with_common_fields_json(raw_input(entry)?)
                    .map_err(unmapped)?,
            )
        };
        let output = || {
            schema_tree(
                lookup_schema(entry.output_schema.as_str())
                    .ok_or_else(|| unmapped("output schema missing"))?,
            )
        };
        return match (select_in, select_out) {
            (true, _) => schema_with_example(input()?),
            (_, true) => schema_with_example(output()?),
            _ => Ok(object([
                ("in", schema_with_example(input()?)?),
                ("out", schema_with_example(output()?)?),
            ])),
        };
    }
    match list.first().copied() {
        Some("verbs") => {
            let rows = VERBS
                .iter()
                .map(|entry| {
                    let appended = full_refusals(entry, reg)?
                        .as_array()
                        .unwrap_or_default()
                        .iter()
                        .filter(|row| row.get("source").and_then(Tree::as_str) != Some("chapter"))
                        .map(|row| {
                            object([
                                ("code", row.get("code").cloned().unwrap_or(Tree::Null)),
                                ("source", row.get("source").cloned().unwrap_or(Tree::Null)),
                            ])
                        })
                        .collect::<Vec<_>>();
                    Ok(object([
                        ("verb", text(entry.name)),
                        ("kind", text(entry.kind)),
                        ("capabilities", array(entry.capabilities.iter().map(text))),
                        (
                            "actions",
                            array(entry.actions.iter().map(|a| {
                                object([
                                    ("name", text(a.name)),
                                    ("kind", text(a.kind)),
                                    ("capabilities", array(a.capabilities.iter().map(text))),
                                ])
                            })),
                        ),
                        ("chapter_refusals", array(entry.refusals.iter().map(text))),
                        ("appended_refusals", array(appended)),
                    ]))
                })
                .collect::<Result<Vec<_>, HandlerFault>>()?;
            Ok(object([("verbs", array(rows))]))
        }
        Some("path_args") => {
            let mut paths = Vec::new();
            for entry in &VERBS {
                collect_paths(entry, &schema_tree(raw_input(entry)?)?, "", &mut paths);
            }
            Ok(object([("path_args", array(paths))]))
        }
        _ => Ok(object([(
            "refusals",
            array(reg.rows().iter().map(|row| {
                let mut members = Vec::from(refusal_row(row));
                members.extend([
                    ("message", text(&row.meaning)),
                    ("name", text(&row.code)),
                    ("raised_by", array(row.raised_by.iter().map(text))),
                ]);
                Tree::Obj(
                    members
                        .into_iter()
                        .map(|(key, value)| (key.into(), value))
                        .collect(),
                )
            })),
        )])),
    }
}

fn collect_paths(entry: &VerbEntry, schema: &Tree, parent: &str, rows: &mut Vec<Tree>) {
    if let Some(items) = schema.get("items") {
        collect_paths(entry, items, &format!("{parent}/*"), rows);
    }
    for (name, property) in schema
        .get("properties")
        .and_then(Tree::as_object)
        .unwrap_or_default()
    {
        let argument = format!("{parent}/{}", escape_ptr(name));
        if let Some(annotation) = property.get("x-frob-path").and_then(Tree::as_object) {
            let mut row = vec![
                ("verb".into(), text(entry.name)),
                ("argument".into(), text(&argument)),
            ];
            row.extend(
                annotation
                    .iter()
                    .filter(|(key, _)| matches!(key.as_str(), "root" | "access" | "kind"))
                    .cloned(),
            );
            if parent.is_empty()
                && let Some((_, param)) = entry.params.iter().find(|(key, _)| *key == name)
            {
                row.push(("param_kind".into(), text(param.to_wire_string())));
            }
            rows.push(Tree::Obj(row));
        }
        collect_paths(entry, property, &argument, rows);
    }
}

fn schema_with_example(doc: Tree) -> Result<Tree, HandlerFault> {
    let example = match doc.get("$id").and_then(Tree::as_str) {
        Some(ENVELOPE_SCHEMA_ID) => {
            let bytes = completed("schema", &object([("path_args", array([]))]))?
                .to_canonical()
                .map_err(unmapped)?;
            schema_tree(bytes.as_str())?
        }
        Some(PROGRESS_SCHEMA_ID) => schema_tree(
            r#"{"schema":"frob.progress/1","job":"j:example","state":"running","unit":"episodes","done":0,"total":1}"#,
        )?,
        _ => doc
            .get("examples")
            .and_then(Tree::as_array)
            .and_then(|values| values.first())
            .cloned()
            .ok_or_else(|| unmapped("schema does not declare a minimal example"))?,
    };
    Schema::compile(doc.clone())
        .map_err(unmapped)?
        .validate(&example)
        .map_err(|_| unmapped("schema example is invalid"))?;
    Ok(object([("schema", doc), ("example", example)]))
}
