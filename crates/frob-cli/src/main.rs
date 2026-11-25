mod grammar;

use std::{
    env,
    io::{self, Write},
    panic::{self, AssertUnwindSafe},
    process::ExitCode,
};

use frob_verbs::{
    dispatch::{
        CommonArgs, DaemonClient, Dispatch, DispatchCx, Frame, InProcess, InputSchema, OnCancel,
        Pipeline, Request, Socket, WireEnvelope,
    },
    envelope::{Cost, Envelope, FaultContext, Tree, embedded_registry},
    handlers::handler_table,
    operator,
    registry::{VERBS, VerbEntry, lookup_schema},
};

type Error = Box<dyn std::error::Error>;

fn main() -> ExitCode {
    panic::set_hook(Box::new(|_| ()));
    let mut args = env::args_os()
        .skip(1)
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let mut json = false;
    let mut help = false;
    let mut no_daemon = false;
    let mut session = env::var("FROB_SESSION").ok();
    let end_options = args.iter().position(|a| a == "--").unwrap_or(args.len());
    let trailing = args.split_off(end_options);
    let mut command = Vec::new();
    let mut options = args.into_iter().peekable();
    while let Some(arg) = options.next() {
        match arg.as_str() {
            "--json" => json = true,
            "--help" | "-h" => help = true,
            "--no-daemon" => no_daemon = true,
            "--session" if options.peek().is_some_and(|v| !v.starts_with("--")) => {
                session = options.next()
            }
            _ => command.push(arg),
        }
    }
    command.extend(trailing);
    let mut stdout = io::stdout().lock();
    if command == ["--version"] {
        return if writeln!(stdout, "frob {}", env!("CARGO_PKG_VERSION")).is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(9)
        };
    }
    let entry = grammar::locate(&VERBS, &command);
    let verb = entry.map_or_else(
        || {
            command
                .iter()
                .take_while(|arg| !arg.starts_with('-'))
                .take(2)
                .cloned()
                .collect::<Vec<_>>()
                .join(".")
        },
        |(entry, _)| entry.name.to_owned(),
    );
    let mut fault = FaultContext {
        verb: verb
            .chars()
            .map(|c| if c.is_control() { '\u{fffd}' } else { c })
            .collect(),
        idempotency_key: None,
        diagnostic_path: String::new(),
    };
    let answer = panic::catch_unwind(AssertUnwindSafe(|| {
        run(
            entry,
            &command,
            &mut fault,
            session,
            help,
            no_daemon,
            json,
            &mut stdout,
        )
    }));
    let exit = match answer {
        Ok(Ok(answer)) if help && !json && answer.exit().code() == 0 => {
            if let Some((entry, _)) = entry {
                render_help(&mut stdout, entry, &answer)
            } else {
                render(&mut stdout, json, &answer)
            }
        }
        Ok(Ok(answer)) => render(&mut stdout, json, &answer),
        _ => fault_response(&mut stdout, json, &fault),
    };
    ExitCode::from(exit.unwrap_or(9))
}

#[allow(clippy::too_many_arguments)]
fn run(
    located: Option<(&VerbEntry, usize)>,
    command: &[String],
    fault: &mut FaultContext,
    session: Option<String>,
    help: bool,
    no_daemon: bool,
    json: bool,
    out: &mut impl Write,
) -> Result<WireEnvelope, Error> {
    let registry = embedded_registry()?;
    let mut schemas = std::collections::BTreeMap::new();
    let mut request = Request::new(&fault.verb, Tree::Obj(vec![])).with_session(session);
    if let Some((entry, count)) = located {
        let raw = lookup_schema(entry.input_schema.as_str()).ok_or("missing input schema")?;
        if help {
            let answer = operator::help(entry, registry).map_err(|_| "help generation failed")?;
            return Ok(WireEnvelope::from_envelope(&answer, registry)?);
        }
        let doc = Tree::parse(raw.as_bytes()).map_err(|_| "invalid input schema")?;
        request.args = match grammar::parse(entry, &doc, &command[count..]) {
            Ok(args) => args,
            Err((parameter, reason)) => return argument_refusal(entry.name, &parameter, reason),
        };
        let string = |key| {
            request
                .args
                .get(key)
                .and_then(Tree::as_str)
                .map(str::to_owned)
        };
        fault.idempotency_key = string("idempotency_key");
        request.common = CommonArgs {
            idempotency_key: fault.idempotency_key.clone(),
            budget: request.args.get("budget").cloned(),
            sync_wait_ms: request.args.get("sync_wait_ms").and_then(Tree::as_int53),
            estimate_only: request
                .args
                .get("estimate_only")
                .and_then(Tree::as_bool)
                .unwrap_or(false),
            on_cancel: match string("on_cancel").as_deref() {
                Some("cancel_job") => OnCancel::Cancel,
                _ => OnCancel::Leave,
            },
            envelope_scope: string("envelope_scope"),
        };
        let merged = entry.input_schema_with_common_fields_json(raw)?;
        schemas.insert(
            entry.name.to_owned(),
            InputSchema::compile_for(entry, &merged, registry)?,
        );
    }
    let pipeline = Pipeline::new(VERBS.to_vec(), schemas, registry.clone());
    if let Err(answer) = pipeline.validate_request(&request)? {
        return Ok(answer);
    }
    let cx = DispatchCx::new();
    let mut stream = if !no_daemon && env::var("FROB_DISPATCH").as_deref() == Ok("socket") {
        let path =
            env::var_os("FROB_SOCKET").ok_or("FROB_SOCKET is required for socket dispatch")?;
        Socket::new(DaemonClient::new(path, registry.clone())?).submit(&cx, &request)?
    } else {
        InProcess::new(pipeline, handler_table()).submit(&cx, &request)?
    };
    while let Some(frame) = stream.next_frame(&cx)? {
        match frame {
            Frame::Envelope(answer) => return Ok(answer),
            Frame::Progress(progress) if json => {
                let event = Tree::Obj(vec![
                    (
                        "schema".into(),
                        Tree::Str(frob_verbs::envelope::PROGRESS_SCHEMA_ID.into()),
                    ),
                    ("job".into(), Tree::Str(progress.job)),
                    ("state".into(), Tree::Str(progress.state)),
                    (
                        "done".into(),
                        Tree::Num(progress.progress.done().to_string()),
                    ),
                    (
                        "total".into(),
                        Tree::Num(progress.progress.total().to_string()),
                    ),
                    ("unit".into(), Tree::Str(progress.progress.unit().into())),
                ]);
                writeln!(out, "{}", event.to_canonical()?.as_str())?;
                out.flush()?;
            }
            Frame::Progress(_) => {}
        }
    }
    Err("dispatch omitted its terminal envelope".into())
}

fn argument_refusal(verb: &str, parameter: &str, reason: &str) -> Result<WireEnvelope, Error> {
    Ok(WireEnvelope::from_envelope(
        &Envelope::from_code(
            verb,
            "FROB-REF-802",
            operator::argument_fields(parameter, reason),
            Cost::ZERO,
        )?,
        embedded_registry()?,
    )?)
}

fn fault_response(out: &mut impl Write, json: bool, fault: &FaultContext) -> Result<u8, Error> {
    let envelope = Envelope::internal_fault(fault, Cost::ZERO);
    if json {
        writeln!(out, "{}", envelope.to_canonical()?.as_str())?;
    } else if let Some(refusal) = envelope.refusal() {
        writeln!(out, "{} {}", refusal.code(), refusal.fields().message)?;
    }
    Ok(envelope.exit().code())
}

fn render(out: &mut impl Write, json: bool, answer: &WireEnvelope) -> Result<u8, Error> {
    if json {
        writeln!(out, "{}", answer.as_str())?;
    } else if let Some(refusal) = answer.tree().get("refusal") {
        writeln!(
            out,
            "{} {}",
            answer.refusal_code().unwrap_or_default(),
            refusal
                .get("message")
                .and_then(Tree::as_str)
                .unwrap_or_default()
        )?;
    } else if let Some(result) = answer
        .tree()
        .get("result")
        .or_else(|| answer.tree().get("job"))
    {
        writeln!(out, "{}", result.to_canonical()?.as_str())?;
    }
    Ok(answer.exit().code())
}

fn render_help(
    out: &mut impl Write,
    entry: &VerbEntry,
    answer: &WireEnvelope,
) -> Result<u8, Error> {
    writeln!(out, "frob {}", entry.cli_path().join(" "))?;
    writeln!(out, "{}", entry.purpose)?;
    let result = answer.tree().get("result").ok_or("help result missing")?;
    let common = entry.common_properties();
    if let Some(properties) = result
        .get("input_schema")
        .and_then(|schema| schema.get("properties"))
        .and_then(Tree::as_object)
    {
        for (name, property) in properties {
            let flag = common
                .iter()
                .find(|common| name == common.name)
                .map_or_else(
                    || format!("--{}", name.replace('_', "-")),
                    |common| common.cli_flag.to_owned(),
                );
            writeln!(
                out,
                "  {flag}  {}",
                property
                    .get("description")
                    .and_then(Tree::as_str)
                    .unwrap_or_default()
            )?;
        }
    }
    for refusal in result
        .get("refusals")
        .and_then(Tree::as_array)
        .unwrap_or_default()
    {
        let field = |key| refusal.get(key).and_then(Tree::as_str).unwrap_or_default();
        writeln!(out, "  {}  {}", field("code"), field("meaning"))?;
    }
    for example in entry.examples {
        writeln!(out, "  {example}")?;
    }
    Ok(answer.exit().code())
}
