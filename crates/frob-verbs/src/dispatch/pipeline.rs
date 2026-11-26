//! Request pipeline steps 1–5: Locate, Validate (including `forbidden_keys` before
//! unknown-parameter rejection), Authorize, Resolve (mutable names + `@fixture:` hook +
//! defaults), and Digest (canonical BLAKE3 covering resolved args and fixture digests)
//! (bead N32a-04, ADR-TETHER-11).
//!
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::{Arc, Mutex};

use frob_core::canon;
use frob_core::id::{Blake3, SchemaName, identity};
use frob_core::refusal::{Refusal, RefusalCode, Repair, RetryPolicy};
use frob_core::registry::refusals::RefusalRegistry;

use crate::envelope::{
    Bound, Cost, Envelope, FixtureBinding, NextAction, RefusalFields, Schema, Tree, escape_ptr,
};
use crate::registry::{
    Kind, MAX_MCP_WAIT_MS, ParamKind, VerbEntry, lookup_in, lookup_retired_verb,
};

use super::{
    DispatchError, FnCanonical, Request, ResolvedRequest, RunTier, SessionBudget, WireEnvelope,
};

/// Errors building a [`Pipeline`] or compiling a verb input schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    InvalidSchema {
        verb: String,
        reason: String,
    },
    InvalidForbiddenKeyCode {
        verb: String,
        code: String,
        reason: String,
    },
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema { verb, reason } => {
                write!(f, "invalid input schema for verb `{verb}`: {reason}")
            }
            Self::InvalidForbiddenKeyCode { verb, code, reason } => {
                write!(
                    f,
                    "invalid forbidden_keys code `{code}` for verb `{verb}`: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for PipelineError {}

/// Compiled verb input schema: optional root `forbidden_keys` rule + draft 2020-12 validator
/// + declared property names & defaults.
#[derive(Debug, Clone)]
pub struct InputSchema {
    schema_id: String,
    validator: Schema,
    /// Map from forbidden key name -> declared refusal code string (in chapter `refusals`).
    forbidden_keys: BTreeMap<String, String>,
    /// Declared top-level property names (for nearest-parameter edit-distance repairs).
    property_names: Vec<String>,
    /// Declared top-level default values (`properties.<name>.default`).
    defaults: Vec<(String, Tree)>,
}

impl InputSchema {
    /// Compile an input schema for `entry` from JSON text.
    ///
    /// Extracts the optional root member `forbidden_keys` (a map from refusal code string to
    /// list of key names), verifies every declared refusal code appears in `entry.refusals`
    /// and in `refusals`, strips `forbidden_keys` before passing the draft 2020-12 document
    /// to `envelope::Schema::compile`, and records top-level properties and defaults.
    pub fn compile_for(
        entry: &VerbEntry,
        schema_json: &str,
        refusals: &RefusalRegistry,
    ) -> Result<Self, PipelineError> {
        let mut doc =
            Tree::parse(schema_json.as_bytes()).map_err(|e| PipelineError::InvalidSchema {
                verb: entry.name.to_owned(),
                reason: format!("{} at `{}`", e.message, e.pointer),
            })?;
        let Tree::Obj(ref mut members) = doc else {
            return Err(PipelineError::InvalidSchema {
                verb: entry.name.to_owned(),
                reason: "input schema root must be an object".to_owned(),
            });
        };

        let mut forbidden_keys = BTreeMap::new();
        if let Some(pos) = members.iter().position(|(k, _)| k == "forbidden_keys") {
            let (_, fk_tree) = members.remove(pos);
            let Some(fk_entries) = fk_tree.as_object() else {
                return Err(PipelineError::InvalidSchema {
                    verb: entry.name.to_owned(),
                    reason: "`forbidden_keys` must be an object mapping refusal code -> [key]"
                        .to_owned(),
                });
            };
            for (code_str, keys_tree) in fk_entries {
                let Some(parsed_code) = RefusalCode::parse(code_str) else {
                    // Also allow synthetic refusal registries whose code isn't yet in RefusalCode
                    // only if it exists in `refusals` AND matches a chapter refusal; otherwise
                    // validate strictly against both `entry.refusals` and `refusals`.
                    if refusals.lookup(code_str, "forbidden_keys").is_err() {
                        return Err(PipelineError::InvalidForbiddenKeyCode {
                            verb: entry.name.to_owned(),
                            code: code_str.clone(),
                            reason: "code is not registered in refusal registry".to_owned(),
                        });
                    }
                    return Err(PipelineError::InvalidForbiddenKeyCode {
                        verb: entry.name.to_owned(),
                        code: code_str.clone(),
                        reason: "code is not in the verb entry's chapter `refusals` list"
                            .to_owned(),
                    });
                };
                if !entry.refusals.contains(&parsed_code) {
                    return Err(PipelineError::InvalidForbiddenKeyCode {
                        verb: entry.name.to_owned(),
                        code: code_str.clone(),
                        reason: "declared `forbidden_keys` code must be listed in the verb entry's `refusals`".to_owned(),
                    });
                }
                if refusals.lookup(code_str, "forbidden_keys").is_err() {
                    return Err(PipelineError::InvalidForbiddenKeyCode {
                        verb: entry.name.to_owned(),
                        code: code_str.clone(),
                        reason: "code missing from active refusal registry".to_owned(),
                    });
                }
                let Some(key_arr) = keys_tree.as_array() else {
                    return Err(PipelineError::InvalidSchema {
                        verb: entry.name.to_owned(),
                        reason: format!(
                            "`forbidden_keys.{code_str}` must be an array of key strings"
                        ),
                    });
                };
                for item in key_arr {
                    let Some(k) = item.as_str() else {
                        return Err(PipelineError::InvalidSchema {
                            verb: entry.name.to_owned(),
                            reason: format!("`forbidden_keys.{code_str}` entries must be strings"),
                        });
                    };
                    if k.is_empty() {
                        return Err(PipelineError::InvalidSchema {
                            verb: entry.name.to_owned(),
                            reason: format!("`forbidden_keys.{code_str}` contains an empty key"),
                        });
                    }
                    if let Some(prev) = forbidden_keys.insert(k.to_owned(), code_str.clone()) {
                        if prev != *code_str {
                            return Err(PipelineError::InvalidSchema {
                                verb: entry.name.to_owned(),
                                reason: format!(
                                    "forbidden key `{k}` mapped to both `{prev}` and `{code_str}`"
                                ),
                            });
                        }
                    }
                }
            }
        }

        let mut property_names = Vec::new();
        let mut defaults = Vec::new();
        if let Some(props) = doc.get("properties").and_then(Tree::as_object) {
            for (name, prop_schema) in props {
                property_names.push(name.clone());
                if let Some(def) = prop_schema.get("default") {
                    defaults.push((name.clone(), def.clone()));
                }
            }
        }

        let validator = Schema::compile(doc).map_err(|e| PipelineError::InvalidSchema {
            verb: entry.name.to_owned(),
            reason: e.to_string(),
        })?;

        Ok(Self {
            schema_id: entry.input_schema.as_str().to_owned(),
            validator,
            forbidden_keys,
            property_names,
            defaults,
        })
    }

    #[must_use]
    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    /// Walk `args` at every depth in document order (objects and arrays, free-form objects
    /// included). Returns `(code, json_pointer, canonical_observed)` on the first key
    /// present in `forbidden_keys`.
    #[must_use]
    pub fn first_forbidden_key(&self, args: &Tree) -> Option<(&str, String, String)> {
        if self.forbidden_keys.is_empty() {
            return None;
        }
        walk_forbidden("", args, &self.forbidden_keys)
    }

    /// Declared top-level property names.
    #[must_use]
    pub fn property_names(&self) -> &[String] {
        &self.property_names
    }
}

fn walk_forbidden<'a>(
    ptr: &str,
    node: &Tree,
    forbidden: &'a BTreeMap<String, String>,
) -> Option<(&'a str, String, String)> {
    match node {
        Tree::Obj(members) => {
            for (k, v) in members {
                let child_ptr = format!("{ptr}/{}", escape_ptr(k));
                if let Some(code) = forbidden.get(k) {
                    let observed = v
                        .to_canonical()
                        .map_or_else(|_| format!("{v:?}"), |c| c.as_str().to_owned());
                    return Some((code.as_str(), child_ptr, observed));
                }
                if let Some(found) = walk_forbidden(&child_ptr, v, forbidden) {
                    return Some(found);
                }
            }
            None
        }
        Tree::Arr(items) => {
            for (i, item) in items.iter().enumerate() {
                let child_ptr = format!("{ptr}/{i}");
                if let Some(found) = walk_forbidden(&child_ptr, item, forbidden) {
                    return Some(found);
                }
            }
            None
        }
        _ => None,
    }
}

/// Pluggable mutable-name resolver invoked during step 4 (*Resolve*, ADR-TETHER-11).
pub trait NameResolver: Send + Sync {
    /// Resolve a mutable name argument `(verb, param, raw_value)` into an immutable
    /// identifier (e.g. `quad-a` -> `d:41aa...`). Returning `Ok(None)` leaves the argument
    /// unchanged.
    fn resolve_name(&self, verb: &str, param: &str, value: &str)
    -> Result<Option<String>, Refusal>;
}

/// Default no-op name resolver.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopNameResolver;

impl NameResolver for NoopNameResolver {
    fn resolve_name(
        &self,
        _verb: &str,
        _param: &str,
        _value: &str,
    ) -> Result<Option<String>, Refusal> {
        Ok(None)
    }
}

/// Pluggable `@fixture:<name>` resolver hook registered by N34p3-01 at step 4 (*Resolve*).
pub trait FixtureResolver: Send + Sync {
    /// Resolve a `@fixture:<name>` reference for a parameter of path-taking kind
    /// (`artifact-input<F>`, `artifact-identity<F>`, or `foreign-file`).
    fn resolve_fixture(
        &self,
        param: &str,
        kind: ParamKind,
        reference: &str,
    ) -> Result<FixtureBinding, Refusal>;
}

/// Default no-op fixture resolver used before N34p3-01 registers its workspace resolver:
/// `@fixture:` arguments remain ordinary strings of their parameter.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopFixtureResolver;

impl FixtureResolver for NoopFixtureResolver {
    fn resolve_fixture(
        &self,
        param: &str,
        _kind: ParamKind,
        reference: &str,
    ) -> Result<FixtureBinding, Refusal> {
        Err(Refusal::new(
            RefusalCode::FrobRef802,
            format!("fixture resolver is not registered for `{reference}`"),
            format!("/{}", escape_ptr(param)),
            reference,
            "registered N34p3-01 fixture resolver",
            "",
            RetryPolicy::AfterChange,
            vec![Repair::new(
                "Register a workspace FixtureResolver (N34p3-01)",
            )],
        ))
    }
}

/// Steps 1–5 pipeline configuration shared by `InProcess` and `Socket`.
pub struct Pipeline {
    verbs: Vec<VerbEntry>,
    schemas: BTreeMap<String, InputSchema>,
    refusals: RefusalRegistry,
    name_resolver: Arc<dyn NameResolver>,
    fixture_resolver: Option<Arc<dyn FixtureResolver>>,
    run_tiers: BTreeMap<String, RunTier>,
    budget: Option<Arc<Mutex<SessionBudget>>>,
}

impl fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pipeline")
            .field("verb_count", &self.verbs.len())
            .field("schema_count", &self.schemas.len())
            .field("has_fixture_resolver", &self.fixture_resolver.is_some())
            .finish_non_exhaustive()
    }
}

impl Pipeline {
    /// Construct a pipeline over `verbs` (sorted by name), compiled `schemas`, and `refusals`.
    pub fn new(
        mut verbs: Vec<VerbEntry>,
        schemas: BTreeMap<String, InputSchema>,
        refusals: RefusalRegistry,
    ) -> Self {
        verbs.sort_by_key(|v| v.name);
        Self {
            verbs,
            schemas,
            refusals,
            name_resolver: Arc::new(NoopNameResolver),
            fixture_resolver: None,
            run_tiers: BTreeMap::new(),
            budget: None,
        }
    }

    #[must_use]
    pub fn with_name_resolver(mut self, resolver: Arc<dyn NameResolver>) -> Self {
        self.name_resolver = resolver;
        self
    }

    #[must_use]
    pub fn with_fixture_resolver(mut self, resolver: Arc<dyn FixtureResolver>) -> Self {
        self.fixture_resolver = Some(resolver);
        self
    }

    #[must_use]
    pub fn with_run_tier(mut self, verb: impl Into<String>, tier: RunTier) -> Self {
        self.run_tiers.insert(verb.into(), tier);
        self
    }

    #[must_use]
    pub fn with_budget(mut self, budget: Arc<Mutex<SessionBudget>>) -> Self {
        self.budget = Some(budget);
        self
    }

    #[must_use]
    pub fn verbs(&self) -> &[VerbEntry] {
        &self.verbs
    }

    #[must_use]
    pub const fn refusals(&self) -> &RefusalRegistry {
        &self.refusals
    }

    #[must_use]
    pub fn budget(&self) -> Option<&Arc<Mutex<SessionBudget>>> {
        self.budget.as_ref()
    }

    /// Run Steps 1 (*Locate*) and 2 (*Validate*) only — charging zero session budget.
    ///
    /// Used by the `Socket` back end before forwarding a call over `frob.rpc/1`, and as the
    /// front half of [`Self::prepare`].
    pub fn validate_request(
        &self,
        req: &Request,
    ) -> Result<Result<&VerbEntry, WireEnvelope>, DispatchError> {
        // Step 1: Locate.
        let Some(entry) = lookup_in(&self.verbs, &req.verb) else {
            return Ok(Err(self.refusal_envelope(
                &req.verb,
                "FROB-REF-801",
                RefusalFields {
                    message: format!("unknown verb `{}`", sanitize_token(&req.verb)),
                    pointer: "/verb".to_owned(),
                    observed: sanitize_token(&req.verb),
                    required: "a registered verb name".to_owned(),
                    unit: String::new(),
                    detail: None,
                    affected_claims: Vec::new(),
                    retry: RetryPolicy::AfterChange,
                    repairs: unknown_verb_repairs(&self.verbs, &req.verb),
                },
            )?));
        };

        // Step 2a: `args` must be a JSON object.
        if req.args.as_object().is_none() {
            return Ok(Err(self.refusal_envelope(
                entry.name,
                "FROB-REF-802",
                RefusalFields {
                    message: "verb arguments must be a JSON object".to_owned(),
                    pointer: "/args".to_owned(),
                    observed: req.args.type_name().to_owned(),
                    required: "object".to_owned(),
                    unit: String::new(),
                    detail: None,
                    affected_claims: Vec::new(),
                    retry: RetryPolicy::AfterChange,
                    repairs: vec![Repair::with_detail(
                        format!("frob schema {}", entry.input_schema.as_str()),
                        "Inspect the verb input schema for a minimal valid argument object.",
                    )],
                },
            )?));
        }

        // Step 2b: `forbidden_keys` at every depth in document order, BEFORE unknown-parameter
        // validation (N32a-04 / N31a-37 ordering rule).
        let Some(input_schema) = self.schemas.get(entry.name) else {
            return Ok(Err(self.refusal_envelope(
                entry.name,
                "FROB-REF-802",
                RefusalFields {
                    message: format!(
                        "input schema `{}` is not loaded",
                        entry.input_schema.as_str()
                    ),
                    pointer: "/args".to_owned(),
                    observed: entry.input_schema.as_str().to_owned(),
                    required: "compiled input schema".to_owned(),
                    unit: String::new(),
                    detail: None,
                    affected_claims: Vec::new(),
                    retry: RetryPolicy::AfterChange,
                    repairs: vec![Repair::new("frob doctor")],
                },
            )?));
        };

        if let Some((code, pointer, observed)) = input_schema.first_forbidden_key(&req.args) {
            return Ok(Err(self.refusal_envelope(
                entry.name,
                code,
                RefusalFields {
                    message: format!(
                        "forbidden key at `{pointer}` refused with `{code}` before argument schema validation"
                    ),
                    pointer,
                    observed,
                    required: "key omitted (cannot be set via request arguments)".to_owned(),
                    unit: String::new(),
                    detail: None,
                    affected_claims: Vec::new(),
                    retry: RetryPolicy::AfterChange,
                    repairs: vec![Repair::with_detail(
                        format!("frob schema {}", entry.input_schema.as_str()),
                        "Remove the forbidden key from the request arguments.",
                    )],
                },
            )?));
        }

        // Step 2c: `sync_wait_ms` bounds (`0..=30_000`, pointer `/sync_wait_ms`).
        if let Some(ms) = req.common.sync_wait_ms {
            if !(0..=MAX_MCP_WAIT_MS).contains(&ms) {
                return Ok(Err(self.refusal_envelope(
                    entry.name,
                    "FROB-REF-802",
                    RefusalFields {
                        message: format!(
                            "`sync_wait_ms` {ms} is outside the permitted range 0..={MAX_MCP_WAIT_MS}"
                        ),
                        pointer: "/sync_wait_ms".to_owned(),
                        observed: ms.to_string(),
                        required: format!("0..={MAX_MCP_WAIT_MS}"),
                        unit: "ms".to_owned(),
                        detail: None,
                        affected_claims: Vec::new(),
                        retry: RetryPolicy::AfterChange,
                        repairs: vec![
                            Repair::with_detail(
                                format!("--sync-wait-ms {MAX_MCP_WAIT_MS}"),
                                "Supply a sync_wait_ms within 0..=30000.",
                            ),
                            Repair::new(format!("frob schema {}", entry.input_schema.as_str())),
                        ],
                    },
                )?));
            }
        }

        // Step 2d: Schema validation (unknown/misspelt parameters, types, bounds).
        if let Err(violations) = input_schema.validator.validate(&req.args) {
            let first = violations
                .iter()
                .find(|v| v.keyword.ends_with("/additionalProperties"))
                .unwrap_or(&violations[0]);
            let pointer = if first.instance.is_empty() {
                "/".to_owned()
            } else {
                first.instance.clone()
            };
            let repairs = schema_violation_repairs(input_schema, &pointer);
            let observed = value_at_pointer(&req.args, &pointer)
                .and_then(|t| t.to_canonical().ok())
                .map_or_else(|| pointer.clone(), |c| c.as_str().to_owned());
            return Ok(Err(self.refusal_envelope(
                entry.name,
                "FROB-REF-802",
                RefusalFields {
                    message: format!(
                        "arguments fail schema `{}`: {}",
                        input_schema.schema_id, first.message
                    ),
                    pointer,
                    observed,
                    required: format!("arguments matching {}", input_schema.schema_id),
                    unit: String::new(),
                    detail: None,
                    affected_claims: Vec::new(),
                    retry: RetryPolicy::AfterChange,
                    repairs,
                },
            )?));
        }

        Ok(Ok(entry))
    }

    /// Run Steps 1 through 5: Locate, Validate, Authorize, Resolve, and Digest.
    ///
    /// Returns `Ok(Ok(resolved))` when ready for steps 6–7, or `Ok(Err(refusal_envelope))`
    /// for any domain refusal raised in steps 1–4.
    pub fn prepare(
        &self,
        req: &Request,
    ) -> Result<Result<ResolvedRequest, WireEnvelope>, DispatchError> {
        let entry = match self.validate_request(req)? {
            Ok(e) => e,
            Err(env) => return Ok(Err(env)),
        };

        let selected = if entry.actions.is_empty() {
            Ok(None)
        } else {
            req.args
                .get("action")
                .map(|value| value.as_str().ok_or(RefusalCode::FrobRef802))
                .transpose()
        };
        let requested_action = selected.as_ref().ok().copied().flatten();
        let action = match selected.and_then(|name| entry.resolve_action(name)) {
            Ok(action) => action,
            Err(code) => {
                let fields = RefusalFields {
                    message: "unknown named action".to_owned(),
                    pointer: "/action".to_owned(),
                    observed: sanitize_token(requested_action.unwrap_or_default()),
                    required: "an action declared on the verb".to_owned(),
                    unit: String::new(),
                    detail: None,
                    affected_claims: Vec::new(),
                    retry: RetryPolicy::AfterChange,
                    repairs: vec![Repair::new(format!(
                        "frob {} --help",
                        entry.name.replace('.', " ")
                    ))],
                };
                return Ok(Err(self.refusal_envelope(
                    entry.name,
                    code.as_str(),
                    fields,
                )?));
            }
        };

        // Step 3: Authorize against the selected action's scopes.
        for cap in action.capabilities {
            if !req.auth.has_scope(cap.as_str()) {
                return Ok(Err(self.refusal_envelope(
                    entry.name,
                    "FROB-REF-720",
                    RefusalFields {
                        message: format!(
                            "session principal `{}` lacks required scope `{}`",
                            req.auth.principal(),
                            cap.as_str()
                        ),
                        pointer: "/session/scopes".to_owned(),
                        observed: req.auth.principal().to_owned(),
                        required: cap.as_str().to_owned(),
                        unit: "scope".to_owned(),
                        detail: None,
                        affected_claims: Vec::new(),
                        retry: RetryPolicy::AfterChange,
                        repairs: vec![
                            Repair::with_detail(
                                format!("frob session open --scope {}", cap.as_str()),
                                "Open a session holding the required capability scope.",
                            ),
                            Repair::new("Ask the parent session to derive a token including the required scope"),
                        ],
                    },
                )?));
            }
        }

        // Step 4: Resolve (fill defaults, resolve mutable names -> `bound`, resolve `@fixture:`
        // on path-taking params when a fixture resolver is registered).
        let input_schema = self
            .schemas
            .get(entry.name)
            .expect("validate_request verified schema presence");

        let mut resolved_members: Vec<(String, Tree)> = req
            .args
            .as_object()
            .expect("validate_request verified object args")
            .to_vec();

        // Fill defaults only for properties admitted on the selected action.
        let common = entry.common_properties();
        for (def_name, def_val) in &input_schema.defaults {
            if let Some(property) = common.iter().find(|p| p.name == def_name)
                && !action.name.map_or(property.on_default, |name| {
                    property.actions.iter().any(|action| action == name)
                })
            {
                continue;
            }
            if !resolved_members.iter().any(|(k, _)| k == def_name) {
                resolved_members.push((def_name.clone(), def_val.clone()));
            }
        }

        let mut bound = Bound::new();
        let mut fixtures: Vec<FixtureBinding> = Vec::new();
        let param_kinds: BTreeMap<&str, ParamKind> = entry.params.iter().copied().collect();

        for (param_name, val) in &mut resolved_members {
            if let Tree::Str(raw_s) = val {
                let is_path_param = param_kinds.get(param_name.as_str()).is_some_and(|pk| {
                    matches!(
                        pk,
                        ParamKind::ArtifactInput(_)
                            | ParamKind::ArtifactIdentity(_)
                            | ParamKind::ForeignFile
                    )
                });
                if is_path_param && raw_s.starts_with("@fixture:") {
                    if let Some(fx_resolver) = &self.fixture_resolver {
                        let pk = param_kinds[param_name.as_str()];
                        match fx_resolver.resolve_fixture(param_name, pk, raw_s) {
                            Ok(binding) => {
                                fixtures.push(binding);
                                continue;
                            }
                            Err(refusal) => {
                                return Ok(Err(
                                    self.refusal_envelope_from_typed(entry.name, &refusal)?
                                ));
                            }
                        }
                    }
                    // Before N34p3-01 registers its fixture resolver, `@fixture:` stays an
                    // ordinary string of its parameter (N32a-04 step 4).
                    continue;
                }

                match self
                    .name_resolver
                    .resolve_name(entry.name, param_name, raw_s)
                {
                    Ok(Some(immutable_id)) => {
                        bound = bound
                            .bind(param_name, &immutable_id)
                            .map_err(DispatchError::Envelope)?;
                        *raw_s = immutable_id;
                    }
                    Ok(None) => {}
                    Err(refusal) => {
                        return Ok(Err(self.refusal_envelope_from_typed(entry.name, &refusal)?));
                    }
                }
            }
        }

        if !fixtures.is_empty() {
            bound = bound.with_fixtures(fixtures.clone());
        }

        let resolved_args = Tree::Obj(resolved_members);

        // Step 5: Digest (canonical serialization covering verb, resolved args, `bound`
        // including every resolved fixture file digest, and semantic common options).
        let digest = compute_request_digest(entry.name, &resolved_args, &bound, &req.common)?;

        let run_tier = determine_run_tier(entry, action.kind, &resolved_args, &self.run_tiers);

        Ok(Ok(ResolvedRequest {
            verb: entry.name.to_owned(),
            kind: action.kind,
            run_tier,
            args: resolved_args,
            bound,
            fixtures,
            common: req.common.clone(),
            principal: req.auth.principal().to_owned(),
            session: req.session.clone(),
            auth: req.auth.clone(),
            digest,
        }))
    }

    pub(crate) fn refusal_envelope(
        &self,
        verb: &str,
        code: &str,
        fields: RefusalFields,
    ) -> Result<WireEnvelope, DispatchError> {
        let mut env = Envelope::from_code_in(&self.refusals, verb, code, fields, Cost::ZERO)
            .map_err(DispatchError::Envelope)?;
        if code == "FROB-REF-801" && verb.is_empty() {
            let action = NextAction::new("frob robot-docs", "List the available commands.")
                .map_err(DispatchError::Envelope)?;
            env = env.with_next_action(action);
        }
        WireEnvelope::from_envelope(&env, &self.refusals)
    }

    pub(crate) fn refusal_envelope_from_typed(
        &self,
        verb: &str,
        refusal: &Refusal,
    ) -> Result<WireEnvelope, DispatchError> {
        self.refusal_envelope(verb, refusal.code.as_str(), RefusalFields::of(refusal))
    }
}

/// Compute the step-5 canonical BLAKE3 request digest (`frob.request/1`).
///
/// Covers `verb`, resolved `args`, `bound` (including every resolved fixture's file
/// `digest`), and semantic `common` modifiers (`budget`, `envelope_scope`,
/// `estimate_only`, `on_cancel`). Deliberately excludes `sync_wait_ms` (client wait
/// window) and `idempotency_key` (the lookup key itself).
pub fn compute_request_digest(
    verb: &str,
    resolved_args: &Tree,
    bound: &Bound,
    common: &super::CommonArgs,
) -> Result<Blake3, DispatchError> {
    let schema = SchemaName::new("request", 1).expect("valid static schema name");
    let canonical = canon::to_canonical(&FnCanonical(|w: canon::Value<'_>| {
        w.object(|o| {
            o.member_term("args", resolved_args)?;
            o.object("bound", |bo| bound.write(bo))?;
            o.object("common", |co| {
                match &common.budget {
                    Some(b) => co.member_term("budget", b)?,
                    None => co.null("budget")?,
                }
                match &common.envelope_scope {
                    Some(s) => co.str("envelope_scope", s)?,
                    None => co.null("envelope_scope")?,
                }
                co.bool("estimate_only", common.estimate_only)?;
                co.str("on_cancel", common.on_cancel.as_str())
            })?;
            o.str("verb", verb)
        })
    }))
    .map_err(DispatchError::Canon)?;
    Ok(identity(&schema, &canonical))
}

fn determine_run_tier(
    entry: &VerbEntry,
    kind: Kind,
    resolved_args: &Tree,
    overrides: &BTreeMap<String, RunTier>,
) -> RunTier {
    if let Some(tier_s) = resolved_args.get("tier").and_then(Tree::as_str) {
        if matches!(tier_s, "cuda" | "metal" | "gpu" | "device" | "fast") {
            return RunTier::Device;
        }
        if matches!(tier_s, "cpu" | "referee" | "static") {
            return RunTier::Cpu;
        }
    }
    if let Some(&t) = overrides.get(entry.name) {
        return t;
    }
    if kind == Kind::Run && entry.cost_model.contains("device") {
        RunTier::Device
    } else {
        RunTier::Cpu
    }
}

fn sanitize_token(raw: &str) -> String {
    raw.chars()
        .map(|c| if c.is_control() { '\u{fffd}' } else { c })
        .collect()
}

fn unknown_verb_repairs(verbs: &[VerbEntry], input: &str) -> Vec<Repair> {
    if let Some(retired) = lookup_retired_verb(input) {
        let suffix = match retired.name {
            "autopsy.fork" => {
                " <run> --world <world> --before <event> --dt <duration> --classify-only --key <key>"
            }
            "fidelity.rank" => " --designs <set> --pair tier:<tier-a>,<tier-b> --key <key>",
            "nemesis.show" => " --class <class>",
            _ => " ...",
        };
        return retired
            .use_instead
            .iter()
            .map(|target| {
                Repair::with_detail(
                    format!("frob {}{suffix} --json", target.replace('.', " ")),
                    retired.reason,
                )
            })
            .collect();
    }
    let mut scored: Vec<(usize, &str)> = verbs
        .iter()
        .map(|v| (edit_distance(input, v.name), v.name))
        .filter(|&(distance, _)| distance <= 3)
        .collect();
    scored.sort_by_key(|&(d, name)| (d, name));
    scored
        .into_iter()
        .take(3)
        .map(|(dist, name)| {
            Repair::with_detail(
                format!("frob {}", name.replace('.', " ")),
                format!("Nearest registered verb (edit distance {dist})."),
            )
        })
        .collect()
}

fn schema_violation_repairs(schema: &InputSchema, pointer: &str) -> Vec<Repair> {
    let mut repairs = Vec::new();
    if let Some(bad_param) = pointer.strip_prefix('/') {
        if !bad_param.is_empty() && !bad_param.contains('/') && !schema.property_names.is_empty() {
            if let Some((_, best)) = schema
                .property_names
                .iter()
                .map(|p| (edit_distance(bad_param, p), p.as_str()))
                .min_by_key(|&(d, p)| (d, p))
            {
                if best != bad_param {
                    repairs.push(Repair::with_detail(
                        format!("--{best}"),
                        format!("Nearest valid parameter name for `{bad_param}`."),
                    ));
                }
            }
        }
    }
    repairs.push(Repair::with_detail(
        format!("frob schema {}", schema.schema_id),
        "Inspect the input schema for a minimal valid example.",
    ));
    repairs
}

fn value_at_pointer<'a>(root: &'a Tree, ptr: &str) -> Option<&'a Tree> {
    if ptr.is_empty() || ptr == "/" {
        return Some(root);
    }
    let mut cur = root;
    for seg in ptr.strip_prefix('/')?.split('/') {
        let unescaped = seg.replace("~1", "/").replace("~0", "~");
        match cur {
            Tree::Obj(_) => cur = cur.get(&unescaped)?,
            Tree::Arr(items) => {
                let idx: usize = unescaped.parse().ok()?;
                cur = items.get(idx)?;
            }
            _ => return None,
        }
    }
    Some(cur)
}

/// Exact Levenshtein edit distance on Unicode scalar values (small strings, zero deps).
pub(crate) fn edit_distance(a: &str, b: &str) -> usize {
    let av: Vec<char> = a.chars().collect();
    let bv: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=bv.len()).collect();
    let mut cur = vec![0usize; bv.len() + 1];
    for (i, &ca) in av.iter().enumerate() {
        cur[0] = i + 1;
        for (j, &cb) in bv.iter().enumerate() {
            let cost = usize::from(ca != cb);
            cur[j + 1] = (prev[j + 1] + 1).min(cur[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[bv.len()]
}

/// Set of top-level keys already checked as unknown parameters when needed by tests.
#[allow(dead_code)]
pub(crate) fn distinct_keys(args: &Tree) -> BTreeSet<String> {
    args.as_object()
        .map(|ms| ms.iter().map(|(k, _)| k.clone()).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use frob_core::id::hash_blob;

    use crate::dispatch::fixture;
    use crate::dispatch::{CommonArgs, SessionAuth};
    use crate::envelope::ExitCode;

    use super::*;

    struct QuadAResolver;
    impl NameResolver for QuadAResolver {
        fn resolve_name(
            &self,
            _verb: &str,
            param: &str,
            value: &str,
        ) -> Result<Option<String>, Refusal> {
            if param == "design" && value == "quad-a" {
                Ok(Some(
                    "d:41aa000000000000000000000000000000000000000000000000000000000001".to_owned(),
                ))
            } else {
                Ok(None)
            }
        }
    }

    struct MemFixtureResolver {
        content: Vec<u8>,
    }
    impl FixtureResolver for MemFixtureResolver {
        fn resolve_fixture(
            &self,
            param: &str,
            _kind: ParamKind,
            reference: &str,
        ) -> Result<FixtureBinding, Refusal> {
            let binding = FixtureBinding::new(
                param,
                reference,
                "fixtures/terrain.toml",
                hash_blob(&self.content),
            )
            .expect("valid binding");
            Ok(binding)
        }
    }

    #[test]
    fn pipeline_order() {
        let pipe = fixture::synthetic_pipeline();

        // Unknown verb with malformed arguments MUST report 801 (step 1), NOT 802 (step 2).
        let req = Request::new("design.chekc", Tree::parse(br#"{"wrolds": -99}"#).unwrap());
        let env = pipe
            .prepare(&req)
            .expect("no transport error")
            .expect_err("unknown verb refused");
        assert_eq!(env.refusal_code(), Some("FROB-REF-801"));
        assert_eq!(env.exit(), ExitCode::Refused);

        // Known verb with misspelt parameter reports 802 with pointer naming the parameter.
        let req2 = Request::new(
            "design.check",
            Tree::parse(br#"{"desgin": "quad-a"}"#).unwrap(),
        );
        let env2 = pipe
            .prepare(&req2)
            .expect("no transport error")
            .expect_err("misspelt param refused");
        assert_eq!(env2.refusal_code(), Some("FROB-REF-802"));
        assert_eq!(env2.exit(), ExitCode::Refused);
        let ptr = env2
            .tree()
            .get("refusal")
            .and_then(|r| r.get("pointer"))
            .and_then(Tree::as_str);
        assert_eq!(ptr, Some("/desgin"));
    }

    #[test]
    fn misspelt_parameter_is_refusal() {
        let pipe = fixture::synthetic_pipeline();
        let req = Request::new(
            "sim.run",
            Tree::parse(br#"{"design":"quad-a","duration_s":2.0,"wrolds":16}"#).unwrap(),
        );
        let env = pipe
            .prepare(&req)
            .expect("no transport error")
            .expect_err("misspelt --wrolds must be refused, never defaulted");
        assert_eq!(env.refusal_code(), Some("FROB-REF-802"));
        assert_eq!(env.exit(), ExitCode::Refused);
        let ptr = env
            .tree()
            .get("refusal")
            .and_then(|r| r.get("pointer"))
            .and_then(Tree::as_str);
        assert_eq!(ptr, Some("/wrolds"));
        let first_repair = env
            .tree()
            .get("refusal")
            .and_then(|r| r.get("repairs"))
            .and_then(Tree::as_array)
            .and_then(|a| a.first())
            .and_then(|r| r.get("action"))
            .and_then(Tree::as_str);
        assert_eq!(first_repair, Some("--worlds"));
    }

    #[test]
    fn request_digest_refuses_inexact_numbers() {
        let digest = |number| {
            let args = Tree::parse(format!(r#"{{"nested":[{number}]}}"#).as_bytes()).unwrap();
            compute_request_digest("design.check", &args, &Bound::new(), &CommonArgs::default())
        };
        assert_eq!(
            digest("9007199254740992").unwrap(),
            digest("9007199254740992.0").unwrap()
        );
        for inexact in ["9007199254740993", "0.10000000000000000000000001"] {
            assert!(matches!(
                digest(inexact),
                Err(DispatchError::Canon(
                    canon::CanonError::InexactUntypedNumber
                ))
            ));
        }
    }

    #[test]
    fn resolution_echoed_and_fixture_digest_enters_request_digest() {
        let pipe1 = fixture::synthetic_pipeline()
            .with_name_resolver(Arc::new(QuadAResolver))
            .with_fixture_resolver(Arc::new(MemFixtureResolver {
                content: b"terrain v1".to_vec(),
            }));
        let req = Request::new(
            "design.check",
            Tree::parse(br#"{"design":"quad-a","scene":"@fixture:flat"}"#).unwrap(),
        );
        let resolved1 = pipe1
            .prepare(&req)
            .expect("ok")
            .expect("valid request resolves");
        assert_eq!(
            resolved1.args.get("design").and_then(Tree::as_str),
            Some("d:41aa000000000000000000000000000000000000000000000000000000000001")
        );
        assert_eq!(resolved1.fixtures.len(), 1);

        // Changing the fixture file content changes the step-5 request digest.
        let pipe2 = fixture::synthetic_pipeline()
            .with_name_resolver(Arc::new(QuadAResolver))
            .with_fixture_resolver(Arc::new(MemFixtureResolver {
                content: b"terrain v2".to_vec(),
            }));
        let resolved2 = pipe2
            .prepare(&req)
            .expect("ok")
            .expect("valid request resolves");
        assert_ne!(resolved1.digest, resolved2.digest);
    }

    #[test]
    fn forbidden_key_declared_code_first() {
        let pipe = fixture::synthetic_pipeline();

        // Top-level forbidden key -> declared code (FROB-REF-801 in synthetic schema),
        // pointer `/released`, observed `true`.
        let top = Request::new(
            "export.packet",
            Tree::parse(br#"{"artifact":"a:1","released":true}"#).unwrap(),
        );
        let env_top = pipe.prepare(&top).unwrap().unwrap_err();
        assert_eq!(env_top.refusal_code(), Some("FROB-REF-801"));
        assert_eq!(
            env_top
                .tree()
                .get("refusal")
                .and_then(|r| r.get("pointer"))
                .and_then(Tree::as_str),
            Some("/released")
        );
        assert_eq!(
            env_top
                .tree()
                .get("refusal")
                .and_then(|r| r.get("observed"))
                .and_then(Tree::as_str),
            Some("true")
        );

        // Nested two levels deep inside a free-form object -> still refused with declared code!
        let nested = Request::new(
            "export.packet",
            Tree::parse(br#"{"artifact":"a:1","meta":{"inner":{"released":true}}}"#).unwrap(),
        );
        let env_nested = pipe.prepare(&nested).unwrap().unwrap_err();
        assert_eq!(env_nested.refusal_code(), Some("FROB-REF-801"));
        assert_eq!(
            env_nested
                .tree()
                .get("refusal")
                .and_then(|r| r.get("pointer"))
                .and_then(Tree::as_str),
            Some("/meta/inner/released")
        );
        assert_eq!(
            env_nested
                .tree()
                .get("refusal")
                .and_then(|r| r.get("observed"))
                .and_then(Tree::as_str),
            Some("true")
        );

        // Renamed `relesed` (not forbidden, but unknown parameter) -> 802.
        let renamed = Request::new(
            "export.packet",
            Tree::parse(br#"{"artifact":"a:1","relesed":true}"#).unwrap(),
        );
        let env_renamed = pipe.prepare(&renamed).unwrap().unwrap_err();
        assert_eq!(env_renamed.refusal_code(), Some("FROB-REF-802"));

        // Both a forbidden key AND a misspelt parameter -> reports the forbidden key's code!
        let both = Request::new(
            "export.packet",
            Tree::parse(br#"{"artifact":"a:1","relesed":false,"meta":{"released":true}}"#).unwrap(),
        );
        let env_both = pipe.prepare(&both).unwrap().unwrap_err();
        assert_eq!(env_both.refusal_code(), Some("FROB-REF-801"));
        assert_eq!(
            env_both
                .tree()
                .get("refusal")
                .and_then(|r| r.get("pointer"))
                .and_then(Tree::as_str),
            Some("/meta/released")
        );
    }

    #[test]
    fn steps_one_and_two_charge_nothing_and_step_three_enforces_scopes() {
        let budget = Arc::new(Mutex::new(
            SessionBudget::new()
                .with_unit("gpu_s", 600)
                .with_unit("cpu_core_s", 7200),
        ));
        let pipe = fixture::synthetic_pipeline().with_budget(Arc::clone(&budget));

        for i in 0..1_000 {
            let req = if i % 2 == 0 {
                Request::new("no.such.verb", Tree::parse(b"{}").unwrap())
            } else {
                Request::new(
                    "design.check",
                    Tree::parse(br#"{"unknown_arg":123}"#).unwrap(),
                )
                .with_common(CommonArgs {
                    sync_wait_ms: Some(99_999),
                    ..CommonArgs::default()
                })
            };
            let env = pipe.prepare(&req).unwrap().unwrap_err();
            assert!(matches!(
                env.refusal_code(),
                Some("FROB-REF-801" | "FROB-REF-802")
            ));
        }

        let b = budget.lock().unwrap();
        assert_eq!(b.remaining("gpu_s"), 600);
        assert_eq!(b.remaining("cpu_core_s"), 7200);
        drop(b);

        // Step 3: synthetic restricted token without `design:read` gets FROB-REF-720 (exit 5).
        let restricted = Request::new(
            "design.check",
            Tree::parse(br#"{"design":"quad-a"}"#).unwrap(),
        )
        .with_auth(SessionAuth::scoped("agent-test", ["sim:read"]));
        let env = pipe.prepare(&restricted).unwrap().unwrap_err();
        assert_eq!(env.refusal_code(), Some("FROB-REF-720"));
        assert_eq!(env.exit(), ExitCode::Capability);
    }
}
