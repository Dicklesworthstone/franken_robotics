//! Thin companion dispatch client binary over `frob_verbs::dispatch` for
//! `scripts/e2e/tether_dispatch.sh` (bead N32a-04).
//!
//! Usage:
//!   frob_rpc_call --backend <in-process|socket> [--socket <path>] --verb <verb> --args <json> [--mask]
//!   frob_rpc_call --drop-at-byte <N> --socket <path> --verb <verb> --args <json>

use std::env;
use std::io::{self, Write};
use std::net::Shutdown;
use std::path::PathBuf;
use std::process::ExitCode;

use frob_verbs::dispatch::fixture::{synthetic_pipeline, synthetic_refusals};
use frob_verbs::dispatch::rpc::encode_request;
use frob_verbs::dispatch::socket::{connect_socket_checked, current_uid};
use frob_verbs::dispatch::{
    DaemonClient, Dispatch, DispatchCx, Request, Socket, mask_envelope_for_parity,
};
use frob_verbs::envelope::Tree;

fn main() -> ExitCode {
    let mut backend = String::from("socket");
    let mut socket_path: Option<PathBuf> = None;
    let mut verb: Option<String> = None;
    let mut args_json = String::from("{}");
    let mut mask = false;
    let mut drop_at_byte: Option<usize> = None;

    let raw_args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0;
    while i < raw_args.len() {
        match raw_args[i].as_str() {
            "--backend" => {
                i += 1;
                if let Some(v) = raw_args.get(i) {
                    backend = v.clone();
                }
            }
            "--socket" => {
                i += 1;
                if let Some(v) = raw_args.get(i) {
                    socket_path = Some(PathBuf::from(v));
                }
            }
            "--verb" => {
                i += 1;
                if let Some(v) = raw_args.get(i) {
                    verb = Some(v.clone());
                }
            }
            "--args" => {
                i += 1;
                if let Some(v) = raw_args.get(i) {
                    args_json = v.clone();
                }
            }
            "--mask" => {
                mask = true;
            }
            "--drop-at-byte" => {
                i += 1;
                if let Some(v) = raw_args.get(i) {
                    match v.parse::<usize>() {
                        Ok(n) => drop_at_byte = Some(n),
                        Err(_) => {
                            let _ = writeln!(io::stderr(), "invalid --drop-at-byte `{v}`");
                            return ExitCode::from(3);
                        }
                    }
                }
            }
            other => {
                let _ = writeln!(io::stderr(), "unknown flag `{other}`");
                return ExitCode::from(3);
            }
        }
        i += 1;
    }

    let Some(verb_name) = verb else {
        let _ = writeln!(io::stderr(), "missing required --verb");
        return ExitCode::from(3);
    };
    let parsed_args = match Tree::parse(args_json.as_bytes()) {
        Ok(t) => t,
        Err(e) => {
            let _ = writeln!(io::stderr(), "invalid --args JSON: {}", e.message);
            return ExitCode::from(3);
        }
    };
    let req = Request::new(verb_name, parsed_args);

    // Mid-call drop simulation: write only the first `cut` bytes of the request frame and
    // drop the connection immediately.
    if let Some(cut) = drop_at_byte {
        let Some(path) = socket_path else {
            let _ = writeln!(io::stderr(), "--drop-at-byte requires --socket <path>");
            return ExitCode::from(3);
        };
        let Ok(uid) = current_uid() else {
            return ExitCode::from(9);
        };
        let Ok(encoded) = encode_request("99", &req) else {
            return ExitCode::from(9);
        };
        let mut frame = encoded.as_bytes().to_vec();
        frame.push(b'\n');
        let slice = &frame[..cut.min(frame.len())];
        match connect_socket_checked(&path, uid) {
            Ok(mut stream) => {
                if !slice.is_empty() {
                    let _ = stream.write_all(slice);
                    let _ = stream.flush();
                }
                let _ = stream.shutdown(Shutdown::Both);
                return ExitCode::from(0);
            }
            Err(e) => {
                let _ = writeln!(io::stderr(), "connect failed: {e}");
                return ExitCode::from(6);
            }
        }
    }

    let refusals = synthetic_refusals();
    let cx = DispatchCx::new();
    let stream_res = match backend.as_str() {
        "socket" => {
            let Some(path) = socket_path else {
                let _ = writeln!(io::stderr(), "--backend socket requires --socket <path>");
                return ExitCode::from(3);
            };
            let client = match DaemonClient::new(&path, refusals) {
                Ok(c) => c,
                Err(e) => {
                    let _ = writeln!(io::stderr(), "socket client error: {e}");
                    return ExitCode::from(6);
                }
            };
            let sock = Socket::new(client).with_local_pipeline(synthetic_pipeline());
            sock.submit(&cx, &req)
        }
        "in-process" => run_in_process(&cx, &req, socket_path, refusals),
        other => {
            let _ = writeln!(io::stderr(), "unknown --backend `{other}`");
            return ExitCode::from(3);
        }
    };

    let stream = match stream_res {
        Ok(s) => s,
        Err(e) => {
            let _ = writeln!(io::stderr(), "dispatch error: {e}");
            return ExitCode::from(9);
        }
    };
    let (_progress, env) = match stream.into_parts() {
        Ok(parts) => parts,
        Err(e) => {
            let _ = writeln!(io::stderr(), "stream error: {e}");
            return ExitCode::from(9);
        }
    };
    let out_str = if mask {
        match mask_envelope_for_parity(&env) {
            Ok(b) => b.as_str().to_owned(),
            Err(e) => {
                let _ = writeln!(io::stderr(), "mask error: {e}");
                return ExitCode::from(9);
            }
        }
    } else {
        env.as_str().to_owned()
    };
    if writeln!(io::stdout(), "{out_str}").is_err() {
        return ExitCode::from(9);
    }
    ExitCode::from(env.exit().code())
}

#[cfg(feature = "handlers")]
fn run_in_process(
    cx: &DispatchCx,
    req: &Request,
    socket_path: Option<PathBuf>,
    refusals: frob_core::registry::refusals::RefusalRegistry,
) -> Result<frob_verbs::dispatch::JobStream, frob_verbs::dispatch::DispatchError> {
    let mut inproc = frob_verbs::dispatch::fixture::synthetic_in_process();
    if let Some(path) = socket_path {
        let client = DaemonClient::new(path, refusals)?;
        inproc = inproc.with_daemon(client);
    }
    inproc.submit(cx, req)
}

#[cfg(not(feature = "handlers"))]
fn run_in_process(
    _cx: &DispatchCx,
    _req: &Request,
    _socket_path: Option<PathBuf>,
    _refusals: frob_core::registry::refusals::RefusalRegistry,
) -> Result<frob_verbs::dispatch::JobStream, frob_verbs::dispatch::DispatchError> {
    Err(frob_verbs::dispatch::DispatchError::Protocol(
        "in-process backend is not compiled under --features registry-only".to_owned(),
    ))
}
