//! `frob.rpc/1` wire framing and local Unix-socket server loop (bead N32a-04, §32.3).
//!
//! Newline-delimited canonical JSON, UTF-8, strictly capped at [`MAX_FRAME_BYTES`] (4 MiB).
//! Per call: one request (or attach) frame from client to server, then zero or more progress
//! frames and exactly one terminal envelope frame from server to client.
//!
//! A malformed or oversize frame closes the connection after emitting one `frob.rpc/1`
//! envelope frame carrying `FROB-REF-802` (exit 3). A client that disconnects at any byte
//! offset leaves any admitted job running and leaves the server loop healthy for subsequent
//! connections.
//!
//! Determinism D0; prints nothing (L-7); zero `unsafe`.

use std::fmt;
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use frob_core::canon::{self, CanonicalBytes};
use frob_core::refusal::{Repair, RetryPolicy};
use frob_core::registry::refusals::RefusalRegistry;

use crate::envelope::{Cost, Envelope, Progress, ProgressReport, RefusalFields, Tree};

use super::socket::{SocketError, bind_socket_0600, current_uid};
use super::{
    CommonArgs, Dispatch, DispatchCx, DispatchError, FnCanonical, Frame, JobRef, OnCancel,
    ProgressFrame, Request, SessionAuth, WireEnvelope,
};

/// Wire protocol schema identifier carried in every `frob.rpc/1` frame's `rpc` member.
pub const RPC_SCHEMA_ID: &str = "frob.rpc/1";

/// Maximum permitted byte length of one NDJSON frame on `frob.rpc/1` (4 MiB, TARGET).
pub const MAX_FRAME_BYTES: usize = 4 * 1024 * 1024;

/// A decoded client-to-server `frob.rpc/1` frame.
#[derive(Debug, Clone, PartialEq)]
pub enum ClientFrame {
    Request {
        id: String,
        request: Request,
    },
    Attach {
        id: String,
        job: JobRef,
        session: Option<String>,
    },
}

/// A decoded server-to-client `frob.rpc/1` frame.
#[derive(Debug, Clone, PartialEq)]
pub enum ServerFrame {
    Progress { id: String, progress: ProgressFrame },
    Envelope { id: String, envelope: WireEnvelope },
}

/// Errors decoding a `frob.rpc/1` wire frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpcFrameError {
    Oversize { limit: usize },
    NotUtf8,
    Malformed { pointer: String, reason: String },
}

impl fmt::Display for RpcFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Oversize { limit } => {
                write!(f, "frob.rpc/1 frame exceeds {limit}-byte cap")
            }
            Self::NotUtf8 => f.write_str("frob.rpc/1 frame is not valid UTF-8"),
            Self::Malformed { pointer, reason } => {
                write!(f, "malformed frob.rpc/1 frame at `{pointer}`: {reason}")
            }
        }
    }
}

impl std::error::Error for RpcFrameError {}

/// Outcome of reading a bounded NDJSON line from a transport stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadLineResult {
    /// Clean EOF with zero bytes read before disconnect.
    Eof,
    /// Client disconnected mid-line before `\n`.
    TruncatedEof,
    /// Line exceeded `max_bytes` (the line was NOT buffered beyond `max_bytes`).
    Oversize,
    /// Complete line (excluding trailing `\n` / `\r\n`).
    Line(Vec<u8>),
}

/// Read one `\n`-terminated line up to `max_bytes` without unbounded allocation.
pub fn read_line_bounded<R: BufRead>(
    reader: &mut R,
    max_bytes: usize,
) -> io::Result<ReadLineResult> {
    let mut buf = Vec::new();
    let mut oversize = false;
    loop {
        let avail = match reader.fill_buf() {
            Ok(slice) => slice,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        if avail.is_empty() {
            if oversize {
                return Ok(ReadLineResult::Oversize);
            }
            if buf.is_empty() {
                return Ok(ReadLineResult::Eof);
            }
            return Ok(ReadLineResult::TruncatedEof);
        }
        if let Some(nl) = avail.iter().position(|&b| b == b'\n') {
            if !oversize {
                if buf.len().saturating_add(nl) > max_bytes {
                    oversize = true;
                } else {
                    buf.extend_from_slice(&avail[..nl]);
                }
            }
            reader.consume(nl + 1);
            if oversize {
                return Ok(ReadLineResult::Oversize);
            }
            if buf.last() == Some(&b'\r') {
                buf.pop();
            }
            return Ok(ReadLineResult::Line(buf));
        }
        let chunk_len = avail.len();
        if !oversize {
            if buf.len().saturating_add(chunk_len) > max_bytes {
                oversize = true;
                buf.clear();
            } else {
                buf.extend_from_slice(avail);
            }
        }
        reader.consume(chunk_len);
        if oversize {
            // Do not read gigabytes from a hostile peer: return Oversize immediately and
            // let the caller send the 802 envelope and close the connection.
            return Ok(ReadLineResult::Oversize);
        }
    }
}

/// Validate a `frob.rpc/1` request `id`: non-empty decimal string (`^[0-9]+$`).
fn check_rpc_id(id: &str) -> Result<(), RpcFrameError> {
    if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
        return Err(RpcFrameError::Malformed {
            pointer: "/id".to_owned(),
            reason: format!("`{id}` must be a non-empty decimal string"),
        });
    }
    Ok(())
}

/// Encode a client request frame as canonical `frob.rpc/1` JSON (without trailing `\n`).
pub fn encode_request(id: &str, req: &Request) -> Result<CanonicalBytes, DispatchError> {
    check_rpc_id(id).map_err(|e| DispatchError::Protocol(e.to_string()))?;
    let bytes = canon::to_canonical(&FnCanonical(|w: canon::Value<'_>| {
        w.object(|o| {
            o.member_term("args", &req.args)?;
            o.object("common", |co| req.common.write_canonical(co))?;
            o.str("id", id)?;
            o.str("rpc", RPC_SCHEMA_ID)?;
            match &req.session {
                Some(s) => o.str("session", s)?,
                None => o.null("session")?,
            }
            o.str("verb", &req.verb)
        })
    }))
    .map_err(DispatchError::Canon)?;
    if bytes.len() > MAX_FRAME_BYTES {
        return Err(DispatchError::Protocol(format!(
            "encoded request frame ({} bytes) exceeds {MAX_FRAME_BYTES}-byte cap",
            bytes.len()
        )));
    }
    Ok(bytes)
}

/// Encode a client `attach` frame as canonical `frob.rpc/1` JSON (without trailing `\n`).
pub fn encode_attach(
    id: &str,
    job: &JobRef,
    session: Option<&str>,
) -> Result<CanonicalBytes, DispatchError> {
    check_rpc_id(id).map_err(|e| DispatchError::Protocol(e.to_string()))?;
    let bytes = canon::to_canonical(&FnCanonical(|w: canon::Value<'_>| {
        w.object(|o| {
            o.object("attach", |ao| match job {
                JobRef::Id(jid) => ao.str("job_id", jid),
                JobRef::Key {
                    verb,
                    idempotency_key,
                } => {
                    ao.str("idempotency_key", idempotency_key)?;
                    ao.str("verb", verb)
                }
            })?;
            o.str("id", id)?;
            o.str("rpc", RPC_SCHEMA_ID)?;
            match session {
                Some(s) => o.str("session", s),
                None => o.null("session"),
            }
        })
    }))
    .map_err(DispatchError::Canon)?;
    Ok(bytes)
}

/// Encode a server progress frame as canonical `frob.rpc/1` JSON (without trailing `\n`).
pub fn encode_progress(
    id: &str,
    progress: &ProgressFrame,
) -> Result<CanonicalBytes, DispatchError> {
    check_rpc_id(id).map_err(|e| DispatchError::Protocol(e.to_string()))?;
    canon::to_canonical(&FnCanonical(|w: canon::Value<'_>| {
        w.object(|o| {
            o.str("id", id)?;
            o.object("progress", |po| progress.write_canonical(po))?;
            o.str("rpc", RPC_SCHEMA_ID)
        })
    }))
    .map_err(DispatchError::Canon)
}

/// Encode a server terminal envelope frame as canonical `frob.rpc/1` JSON (without trailing `\n`).
pub fn encode_envelope(id: &str, env: &WireEnvelope) -> Result<CanonicalBytes, DispatchError> {
    check_rpc_id(id).map_err(|e| DispatchError::Protocol(e.to_string()))?;
    canon::to_canonical(&FnCanonical(|w: canon::Value<'_>| {
        w.object(|o| {
            o.member_term("envelope", env.tree())?;
            o.str("id", id)?;
            o.str("rpc", RPC_SCHEMA_ID)
        })
    }))
    .map_err(DispatchError::Canon)
}

/// Decode one client-to-server `frob.rpc/1` line (strict RFC 8259 JSON, closed keys).
pub fn decode_client_frame(line: &[u8]) -> Result<ClientFrame, RpcFrameError> {
    if line.len() > MAX_FRAME_BYTES {
        return Err(RpcFrameError::Oversize {
            limit: MAX_FRAME_BYTES,
        });
    }
    if std::str::from_utf8(line).is_err() {
        return Err(RpcFrameError::NotUtf8);
    }
    let tree = Tree::parse(line).map_err(|e| RpcFrameError::Malformed {
        pointer: e.pointer,
        reason: e.message,
    })?;
    let Some(members) = tree.as_object() else {
        return Err(RpcFrameError::Malformed {
            pointer: "/".to_owned(),
            reason: "frame must be a JSON object".to_owned(),
        });
    };

    let rpc = tree
        .get("rpc")
        .and_then(Tree::as_str)
        .ok_or_else(|| RpcFrameError::Malformed {
            pointer: "/rpc".to_owned(),
            reason: "missing string `rpc`".to_owned(),
        })?;
    if rpc != RPC_SCHEMA_ID {
        return Err(RpcFrameError::Malformed {
            pointer: "/rpc".to_owned(),
            reason: format!("unsupported protocol `{rpc}` (expected `{RPC_SCHEMA_ID}`)"),
        });
    }

    let id = tree
        .get("id")
        .and_then(Tree::as_str)
        .ok_or_else(|| RpcFrameError::Malformed {
            pointer: "/id".to_owned(),
            reason: "missing string `id`".to_owned(),
        })?;
    check_rpc_id(id)?;

    let session = match tree.get("session") {
        None | Some(Tree::Null) => None,
        Some(Tree::Str(s)) => Some(s.clone()),
        Some(_) => {
            return Err(RpcFrameError::Malformed {
                pointer: "/session".to_owned(),
                reason: "`session` must be string or null".to_owned(),
            });
        }
    };

    if let Some(attach) = tree.get("attach") {
        for (k, _) in members {
            if !matches!(k.as_str(), "rpc" | "id" | "session" | "attach") {
                return Err(RpcFrameError::Malformed {
                    pointer: format!("/{k}"),
                    reason: format!("unknown attach frame key `{k}`"),
                });
            }
        }
        let Some(am) = attach.as_object() else {
            return Err(RpcFrameError::Malformed {
                pointer: "/attach".to_owned(),
                reason: "`attach` must be an object".to_owned(),
            });
        };
        let job = if let Some(jid) = attach.get("job_id").and_then(Tree::as_str) {
            if am.len() != 1 || jid.is_empty() {
                return Err(RpcFrameError::Malformed {
                    pointer: "/attach/job_id".to_owned(),
                    reason: "invalid `job_id` attach form".to_owned(),
                });
            }
            JobRef::Id(jid.to_owned())
        } else {
            let verb = attach.get("verb").and_then(Tree::as_str).ok_or_else(|| {
                RpcFrameError::Malformed {
                    pointer: "/attach".to_owned(),
                    reason: "attach requires `job_id` or (`verb`, `idempotency_key`)".to_owned(),
                }
            })?;
            let key = attach
                .get("idempotency_key")
                .and_then(Tree::as_str)
                .ok_or_else(|| RpcFrameError::Malformed {
                    pointer: "/attach/idempotency_key".to_owned(),
                    reason: "missing string `idempotency_key`".to_owned(),
                })?;
            if am.len() != 2 || verb.is_empty() || key.is_empty() {
                return Err(RpcFrameError::Malformed {
                    pointer: "/attach".to_owned(),
                    reason: "invalid key attach form".to_owned(),
                });
            }
            JobRef::Key {
                verb: verb.to_owned(),
                idempotency_key: key.to_owned(),
            }
        };
        return Ok(ClientFrame::Attach {
            id: id.to_owned(),
            job,
            session,
        });
    }

    // Request frame: closed member set {rpc, id, session, verb, args, common}.
    for (k, _) in members {
        if !matches!(
            k.as_str(),
            "rpc" | "id" | "session" | "verb" | "args" | "common"
        ) {
            return Err(RpcFrameError::Malformed {
                pointer: format!("/{k}"),
                reason: format!("unknown request frame key `{k}`"),
            });
        }
    }
    let verb = tree
        .get("verb")
        .and_then(Tree::as_str)
        .ok_or_else(|| RpcFrameError::Malformed {
            pointer: "/verb".to_owned(),
            reason: "missing string `verb`".to_owned(),
        })?;
    let args = tree
        .get("args")
        .ok_or_else(|| RpcFrameError::Malformed {
            pointer: "/args".to_owned(),
            reason: "missing `args`".to_owned(),
        })?
        .clone();
    let common = match tree.get("common") {
        None => CommonArgs::default(),
        Some(c) => parse_common(c)?,
    };

    Ok(ClientFrame::Request {
        id: id.to_owned(),
        request: Request {
            verb: verb.to_owned(),
            args,
            common,
            session,
            auth: SessionAuth::local_full(),
        },
    })
}

fn parse_common(tree: &Tree) -> Result<CommonArgs, RpcFrameError> {
    let Some(members) = tree.as_object() else {
        return Err(RpcFrameError::Malformed {
            pointer: "/common".to_owned(),
            reason: "`common` must be an object".to_owned(),
        });
    };
    for (k, _) in members {
        if !matches!(
            k.as_str(),
            "idempotency_key"
                | "budget"
                | "sync_wait_ms"
                | "estimate_only"
                | "on_cancel"
                | "envelope_scope"
        ) {
            return Err(RpcFrameError::Malformed {
                pointer: format!("/common/{k}"),
                reason: format!("unknown `common` field `{k}`"),
            });
        }
    }
    let idempotency_key = match tree.get("idempotency_key") {
        None | Some(Tree::Null) => None,
        Some(Tree::Str(s)) => Some(s.clone()),
        Some(_) => {
            return Err(RpcFrameError::Malformed {
                pointer: "/common/idempotency_key".to_owned(),
                reason: "expected string or null".to_owned(),
            });
        }
    };
    let budget = match tree.get("budget") {
        None | Some(Tree::Null) => None,
        Some(b @ Tree::Obj(_)) => Some(b.clone()),
        Some(_) => {
            return Err(RpcFrameError::Malformed {
                pointer: "/common/budget".to_owned(),
                reason: "expected object or null".to_owned(),
            });
        }
    };
    let sync_wait_ms = match tree.get("sync_wait_ms") {
        None | Some(Tree::Null) => None,
        Some(v) => {
            let ms = v.as_int53().ok_or_else(|| RpcFrameError::Malformed {
                pointer: "/common/sync_wait_ms".to_owned(),
                reason: "expected exact integer or null".to_owned(),
            })?;
            Some(ms)
        }
    };
    let estimate_only = match tree.get("estimate_only") {
        None => false,
        Some(Tree::Bool(b)) => *b,
        Some(_) => {
            return Err(RpcFrameError::Malformed {
                pointer: "/common/estimate_only".to_owned(),
                reason: "expected boolean".to_owned(),
            });
        }
    };
    let on_cancel = match tree.get("on_cancel") {
        None => OnCancel::Leave,
        Some(Tree::Str(s)) => OnCancel::parse(s).ok_or_else(|| RpcFrameError::Malformed {
            pointer: "/common/on_cancel".to_owned(),
            reason: format!("invalid `on_cancel` value `{s}`"),
        })?,
        Some(_) => {
            return Err(RpcFrameError::Malformed {
                pointer: "/common/on_cancel".to_owned(),
                reason: "expected string".to_owned(),
            });
        }
    };
    let envelope_scope = match tree.get("envelope_scope") {
        None | Some(Tree::Null) => None,
        Some(Tree::Str(s)) => Some(s.clone()),
        Some(_) => {
            return Err(RpcFrameError::Malformed {
                pointer: "/common/envelope_scope".to_owned(),
                reason: "expected string or null".to_owned(),
            });
        }
    };
    Ok(CommonArgs {
        idempotency_key,
        budget,
        sync_wait_ms,
        estimate_only,
        on_cancel,
        envelope_scope,
    })
}

/// Decode one server-to-client `frob.rpc/1` line into a progress or terminal envelope frame.
pub fn decode_server_frame(
    line: &[u8],
    refusals: &RefusalRegistry,
) -> Result<ServerFrame, RpcFrameError> {
    if line.len() > MAX_FRAME_BYTES {
        return Err(RpcFrameError::Oversize {
            limit: MAX_FRAME_BYTES,
        });
    }
    if std::str::from_utf8(line).is_err() {
        return Err(RpcFrameError::NotUtf8);
    }
    let tree = Tree::parse(line).map_err(|e| RpcFrameError::Malformed {
        pointer: e.pointer,
        reason: e.message,
    })?;
    let Some(members) = tree.as_object() else {
        return Err(RpcFrameError::Malformed {
            pointer: "/".to_owned(),
            reason: "server frame must be a JSON object".to_owned(),
        });
    };
    let rpc = tree
        .get("rpc")
        .and_then(Tree::as_str)
        .ok_or_else(|| RpcFrameError::Malformed {
            pointer: "/rpc".to_owned(),
            reason: "missing string `rpc`".to_owned(),
        })?;
    if rpc != RPC_SCHEMA_ID {
        return Err(RpcFrameError::Malformed {
            pointer: "/rpc".to_owned(),
            reason: format!("unsupported protocol `{rpc}`"),
        });
    }
    let id = tree
        .get("id")
        .and_then(Tree::as_str)
        .ok_or_else(|| RpcFrameError::Malformed {
            pointer: "/id".to_owned(),
            reason: "missing string `id`".to_owned(),
        })?;
    check_rpc_id(id)?;

    match (tree.get("progress"), tree.get("envelope")) {
        (Some(p), None) => {
            for (k, _) in members {
                if !matches!(k.as_str(), "rpc" | "id" | "progress") {
                    return Err(RpcFrameError::Malformed {
                        pointer: format!("/{k}"),
                        reason: format!("unknown progress frame key `{k}`"),
                    });
                }
            }
            let job =
                p.get("job")
                    .and_then(Tree::as_str)
                    .ok_or_else(|| RpcFrameError::Malformed {
                        pointer: "/progress/job".to_owned(),
                        reason: "missing string `job`".to_owned(),
                    })?;
            let state =
                p.get("state")
                    .and_then(Tree::as_str)
                    .ok_or_else(|| RpcFrameError::Malformed {
                        pointer: "/progress/state".to_owned(),
                        reason: "missing string `state`".to_owned(),
                    })?;
            let done_i =
                p.get("done")
                    .and_then(Tree::as_int53)
                    .ok_or_else(|| RpcFrameError::Malformed {
                        pointer: "/progress/done".to_owned(),
                        reason: "missing integer `done`".to_owned(),
                    })?;
            let total_i = p.get("total").and_then(Tree::as_int53).ok_or_else(|| {
                RpcFrameError::Malformed {
                    pointer: "/progress/total".to_owned(),
                    reason: "missing integer `total`".to_owned(),
                }
            })?;
            let unit =
                p.get("unit")
                    .and_then(Tree::as_str)
                    .ok_or_else(|| RpcFrameError::Malformed {
                        pointer: "/progress/unit".to_owned(),
                        reason: "missing string `unit`".to_owned(),
                    })?;
            let done = u64::try_from(done_i).map_err(|_| RpcFrameError::Malformed {
                pointer: "/progress/done".to_owned(),
                reason: "negative `done`".to_owned(),
            })?;
            let total = u64::try_from(total_i).map_err(|_| RpcFrameError::Malformed {
                pointer: "/progress/total".to_owned(),
                reason: "negative `total`".to_owned(),
            })?;
            let progress = Progress::from_report(&ProgressReport {
                done,
                total: Some(total),
                unit: unit.to_owned(),
            })
            .map_err(|e| RpcFrameError::Malformed {
                pointer: "/progress".to_owned(),
                reason: e.to_string(),
            })?;
            let pf =
                ProgressFrame::new(job, state, progress).map_err(|e| RpcFrameError::Malformed {
                    pointer: "/progress".to_owned(),
                    reason: e.to_string(),
                })?;
            Ok(ServerFrame::Progress {
                id: id.to_owned(),
                progress: pf,
            })
        }
        (None, Some(e)) => {
            for (k, _) in members {
                if !matches!(k.as_str(), "rpc" | "id" | "envelope") {
                    return Err(RpcFrameError::Malformed {
                        pointer: format!("/{k}"),
                        reason: format!("unknown envelope frame key `{k}`"),
                    });
                }
            }
            let raw = e.to_canonical().map_err(|err| RpcFrameError::Malformed {
                pointer: "/envelope".to_owned(),
                reason: err.to_string(),
            })?;
            let env = WireEnvelope::from_bytes(raw.as_bytes(), refusals).map_err(|err| {
                RpcFrameError::Malformed {
                    pointer: "/envelope".to_owned(),
                    reason: err.to_string(),
                }
            })?;
            Ok(ServerFrame::Envelope {
                id: id.to_owned(),
                envelope: env,
            })
        }
        _ => Err(RpcFrameError::Malformed {
            pointer: "/".to_owned(),
            reason: "server frame must carry exactly one of `progress` or `envelope`".to_owned(),
        }),
    }
}

/// Outcome of handling one `frob.rpc/1` connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionOutcome {
    /// One valid request or attach was answered with progress frames and one envelope.
    Completed,
    /// One malformed or oversize frame was answered with a `FROB-REF-802` envelope and closed.
    RefusedMalformed,
    /// Client disconnected before completing the frame or while receiving the response.
    ClientDisconnected,
}

fn build_frame_error_envelope(
    refusals: &RefusalRegistry,
    err: &RpcFrameError,
) -> Result<WireEnvelope, DispatchError> {
    let (pointer, observed, required, repairs) = match err {
        RpcFrameError::Oversize { limit } => (
            "/frame".to_owned(),
            format!(">{limit} bytes"),
            format!("<= {limit} bytes"),
            vec![
                Repair::with_detail(
                    "frob artifact read <digest> --range <start>-<end>",
                    "Transfer payloads larger than 4 MiB through artifact.read by byte range.",
                ),
                Repair::new("Reduce the request frame size below 4 MiB"),
            ],
        ),
        RpcFrameError::NotUtf8 => (
            "/frame".to_owned(),
            "non-utf8".to_owned(),
            "UTF-8 NDJSON frame".to_owned(),
            vec![Repair::new("Encode frob.rpc/1 frames as valid UTF-8 JSON")],
        ),
        RpcFrameError::Malformed { pointer, reason } => (
            pointer.clone(),
            reason.clone(),
            format!("valid {RPC_SCHEMA_ID} frame"),
            vec![Repair::new(format!(
                "Send a valid {RPC_SCHEMA_ID} newline-delimited JSON request frame"
            ))],
        ),
    };
    let env = Envelope::from_code_in(
        refusals,
        "rpc.frame",
        "FROB-REF-802",
        RefusalFields {
            message: err.to_string(),
            pointer,
            observed,
            required,
            unit: "bytes".to_owned(),
            detail: None,
            affected_claims: Vec::new(),
            retry: RetryPolicy::AfterChange,
            repairs,
        },
        Cost::ZERO,
    )
    .map_err(DispatchError::Envelope)?;
    WireEnvelope::from_envelope(&env, refusals)
}

fn write_line_ignore_disconnect<W: Write>(writer: &mut W, bytes: &[u8]) -> io::Result<bool> {
    match writer
        .write_all(bytes)
        .and_then(|()| writer.write_all(b"\n"))
        .and_then(|()| writer.flush())
    {
        Ok(()) => Ok(true),
        Err(e)
            if matches!(
                e.kind(),
                io::ErrorKind::BrokenPipe
                    | io::ErrorKind::ConnectionReset
                    | io::ErrorKind::ConnectionAborted
                    | io::ErrorKind::WriteZero
            ) =>
        {
            Ok(false)
        }
        Err(e) => Err(e),
    }
}

/// Handle a single `frob.rpc/1` connection over any buffered reader + writer.
///
/// One request frame -> zero or more progress frames -> one terminal envelope frame.
/// Malformed or oversize frames emit one `FROB-REF-802` envelope frame and close.
pub fn serve_connection<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    backend: &dyn Dispatch,
    refusals: &RefusalRegistry,
) -> io::Result<ConnectionOutcome> {
    let line = match read_line_bounded(reader, MAX_FRAME_BYTES) {
        Ok(ReadLineResult::Eof | ReadLineResult::TruncatedEof) => {
            return Ok(ConnectionOutcome::ClientDisconnected);
        }
        Ok(ReadLineResult::Oversize) => {
            let err = RpcFrameError::Oversize {
                limit: MAX_FRAME_BYTES,
            };
            if let Ok(env) = build_frame_error_envelope(refusals, &err) {
                if let Ok(frame) = encode_envelope("0", &env) {
                    let _ = write_line_ignore_disconnect(writer, frame.as_bytes());
                }
            }
            return Ok(ConnectionOutcome::RefusedMalformed);
        }
        Ok(ReadLineResult::Line(bytes)) => bytes,
        Err(e)
            if matches!(
                e.kind(),
                io::ErrorKind::ConnectionReset
                    | io::ErrorKind::ConnectionAborted
                    | io::ErrorKind::UnexpectedEof
                    | io::ErrorKind::WouldBlock
                    | io::ErrorKind::TimedOut
            ) =>
        {
            return Ok(ConnectionOutcome::ClientDisconnected);
        }
        Err(e) => return Err(e),
    };

    let client_frame = match decode_client_frame(&line) {
        Ok(f) => f,
        Err(err) => {
            if let Ok(env) = build_frame_error_envelope(refusals, &err) {
                if let Ok(frame) = encode_envelope("0", &env) {
                    let _ = write_line_ignore_disconnect(writer, frame.as_bytes());
                }
            }
            return Ok(ConnectionOutcome::RefusedMalformed);
        }
    };

    let cx = DispatchCx::new();
    let (id, stream_res) = match client_frame {
        ClientFrame::Request { id, request } => (id, backend.submit(&cx, &request)),
        ClientFrame::Attach { id, job, .. } => (id, backend.attach(&cx, &job)),
    };

    let mut stream = match stream_res {
        Ok(s) => s,
        Err(err) => {
            let fault = Envelope::internal_fault(
                &crate::envelope::FaultContext {
                    verb: "rpc.dispatch".to_owned(),
                    idempotency_key: None,
                    diagnostic_path: err.to_string(),
                },
                Cost::ZERO,
            );
            if let Ok(wire) = WireEnvelope::from_envelope(&fault, refusals) {
                if let Ok(frame) = encode_envelope(&id, &wire) {
                    let _ = write_line_ignore_disconnect(writer, frame.as_bytes());
                }
            }
            return Ok(ConnectionOutcome::Completed);
        }
    };

    while let Ok(Some(frame)) = stream.next_frame(&cx) {
        let encoded = match frame {
            Frame::Progress(p) => encode_progress(&id, &p),
            Frame::Envelope(e) => encode_envelope(&id, &e),
        };
        let Ok(bytes) = encoded else {
            return Ok(ConnectionOutcome::ClientDisconnected);
        };
        if !write_line_ignore_disconnect(writer, bytes.as_bytes())? {
            return Ok(ConnectionOutcome::ClientDisconnected);
        }
    }

    Ok(ConnectionOutcome::Completed)
}

/// Statistics collected by [`serve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RpcServeStats {
    pub completed: u64,
    pub malformed: u64,
    pub disconnected: u64,
}

/// Shared library server loop (`frob_verbs::rpc::serve`, N32a-04): accepts Unix-domain
/// connections on `listener` until `stop` is signalled, handling each connection cleanly so
/// dropped or malformed clients never disrupt subsequent requests.
pub fn serve(
    listener: UnixListener,
    backend: Arc<dyn Dispatch>,
    refusals: RefusalRegistry,
    stop: Arc<AtomicBool>,
    request_counter: Option<Arc<AtomicU64>>,
) -> io::Result<RpcServeStats> {
    listener.set_nonblocking(true)?;
    let mut stats = RpcServeStats::default();
    let mut workers: Vec<JoinHandle<ConnectionOutcome>> = Vec::new();

    while !stop.load(Ordering::SeqCst) {
        // Reap finished connection workers opportunistically so thread count stays bounded.
        let mut idx = 0;
        while idx < workers.len() {
            if workers[idx].is_finished() {
                let h = workers.swap_remove(idx);
                if let Ok(outcome) = h.join() {
                    record_outcome(&mut stats, outcome);
                }
            } else {
                idx += 1;
            }
        }

        match listener.accept() {
            Ok((stream, _addr)) => {
                if stop.load(Ordering::SeqCst) {
                    break;
                }
                let _ = stream.set_nonblocking(false);
                let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
                let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));
                let backend_c = Arc::clone(&backend);
                let refusals_c = refusals.clone();
                let counter_c = request_counter.clone();
                workers.push(thread::spawn(move || {
                    let Ok(mut writer) = stream.try_clone() else {
                        return ConnectionOutcome::ClientDisconnected;
                    };
                    let mut reader = BufReader::new(stream);
                    let outcome =
                        serve_connection(&mut reader, &mut writer, backend_c.as_ref(), &refusals_c)
                            .unwrap_or(ConnectionOutcome::ClientDisconnected);
                    if outcome == ConnectionOutcome::Completed {
                        if let Some(c) = counter_c {
                            c.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                    outcome
                }));
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(5));
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }

    for h in workers {
        if let Ok(outcome) = h.join() {
            record_outcome(&mut stats, outcome);
        }
    }
    Ok(stats)
}

fn record_outcome(stats: &mut RpcServeStats, outcome: ConnectionOutcome) {
    match outcome {
        ConnectionOutcome::Completed => stats.completed += 1,
        ConnectionOutcome::RefusedMalformed => stats.malformed += 1,
        ConnectionOutcome::ClientDisconnected => stats.disconnected += 1,
    }
}

/// Running background `frob.rpc/1` server handle (for tests and companion binaries).
pub struct RpcServerHandle {
    socket_path: PathBuf,
    stop: Arc<AtomicBool>,
    requests: Arc<AtomicU64>,
    thread: Option<JoinHandle<io::Result<RpcServeStats>>>,
}

impl RpcServerHandle {
    /// Bind `socket_path` with mode `0600` (refusing pre-existing sockets with wrong owner
    /// or permissions) and spawn the [`serve`] loop.
    pub fn spawn(
        socket_path: impl AsRef<Path>,
        expected_uid: u32,
        backend: Arc<dyn Dispatch>,
        refusals: RefusalRegistry,
    ) -> Result<Self, SocketError> {
        let path = socket_path.as_ref().to_path_buf();
        let listener = bind_socket_0600(&path, expected_uid)?;
        let stop = Arc::new(AtomicBool::new(false));
        let requests = Arc::new(AtomicU64::new(0));
        let stop_c = Arc::clone(&stop);
        let req_c = Arc::clone(&requests);
        let thread = thread::spawn(move || serve(listener, backend, refusals, stop_c, Some(req_c)));
        Ok(Self {
            socket_path: path,
            stop,
            requests,
            thread: Some(thread),
        })
    }

    /// Spawn bound to the current process's effective UID.
    pub fn spawn_current_uid(
        socket_path: impl AsRef<Path>,
        backend: Arc<dyn Dispatch>,
        refusals: RefusalRegistry,
    ) -> Result<Self, SocketError> {
        let uid = current_uid()?;
        Self::spawn(socket_path, uid, backend, refusals)
    }

    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Number of completed `frob.rpc/1` calls served so far.
    #[must_use]
    pub fn request_count(&self) -> u64 {
        self.requests.load(Ordering::SeqCst)
    }

    /// Stop the server loop cleanly and return its collected stats (never deletes the socket
    /// file; caller's scratch directory holds it per COMMON.md 9b).
    pub fn shutdown(mut self) -> io::Result<RpcServeStats> {
        self.stop.store(true, Ordering::SeqCst);
        let _ = UnixStream::connect(&self.socket_path);
        match self.thread.take() {
            Some(handle) => handle
                .join()
                .unwrap_or_else(|_| Err(io::Error::other("rpc server thread panicked"))),
            None => Ok(RpcServeStats::default()),
        }
    }
}

impl Drop for RpcServerHandle {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = UnixStream::connect(&self.socket_path);
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::net::Shutdown;

    use crate::dispatch::fixture;
    use crate::dispatch::socket::test_socket_path;
    use crate::dispatch::{DaemonClient, Socket};

    use super::*;

    #[test]
    fn frame_roundtrip() {
        let refusals = fixture::synthetic_refusals();

        // 1. Request frame round-trip + id echo.
        let req = Request::new(
            "design.check",
            Tree::parse(br#"{"design":"quad-a"}"#).unwrap(),
        )
        .with_common(CommonArgs {
            idempotency_key: Some("k-42".to_owned()),
            sync_wait_ms: Some(2500),
            estimate_only: false,
            on_cancel: OnCancel::Leave,
            budget: None,
            envelope_scope: None,
        })
        .with_session(Some("tok-abc".to_owned()));
        let enc1 = encode_request("7", &req).expect("encode request");
        let dec = decode_client_frame(enc1.as_bytes()).expect("decode request");
        let ClientFrame::Request { id, request: req2 } = dec else {
            panic!("expected request frame");
        };
        assert_eq!(id, "7");
        let enc2 = encode_request(&id, &req2).expect("re-encode request");
        assert_eq!(enc1.as_str(), enc2.as_str());

        // 2. Progress frame round-trip + id echo.
        let pf = ProgressFrame::new(
            "j:77c0",
            "running",
            Progress::from_report(&ProgressReport {
                done: 512,
                total: Some(4096),
                unit: "worlds".to_owned(),
            })
            .unwrap(),
        )
        .unwrap();
        let penc1 = encode_progress("7", &pf).expect("encode progress");
        let pdec = decode_server_frame(penc1.as_bytes(), &refusals).expect("decode progress");
        let ServerFrame::Progress {
            id: pid,
            progress: pf2,
        } = pdec
        else {
            panic!("expected progress frame");
        };
        assert_eq!(pid, "7");
        assert_eq!(pf, pf2);
        let penc2 = encode_progress(&pid, &pf2).expect("re-encode progress");
        assert_eq!(penc1.as_str(), penc2.as_str());

        // 3. Envelope frame round-trip + id echo.
        let wire_env = WireEnvelope::from_envelope(
            &Envelope::internal_fault(
                &crate::envelope::FaultContext {
                    verb: "design.check".to_owned(),
                    idempotency_key: None,
                    diagnostic_path: "d.txt".to_owned(),
                },
                Cost::ZERO,
            ),
            &refusals,
        )
        .unwrap();
        let eenc1 = encode_envelope("7", &wire_env).expect("encode envelope");
        let edec = decode_server_frame(eenc1.as_bytes(), &refusals).expect("decode envelope");
        let ServerFrame::Envelope {
            id: eid,
            envelope: wire_env2,
        } = edec
        else {
            panic!("expected envelope frame");
        };
        assert_eq!(eid, "7");
        assert_eq!(wire_env.as_str(), wire_env2.as_str());
        let eenc2 = encode_envelope(&eid, &wire_env2).expect("re-encode envelope");
        assert_eq!(eenc1.as_str(), eenc2.as_str());
    }

    #[test]
    fn oversize_frame_refused() {
        let refusals = fixture::synthetic_refusals();
        let backend = fixture::synthetic_dispatch_service();
        let mut big = Vec::with_capacity(5 * 1024 * 1024 + 32);
        big.extend_from_slice(
            br#"{"rpc":"frob.rpc/1","id":"1","verb":"design.check","args":{"pad":""#,
        );
        big.resize(5 * 1024 * 1024, b'a');
        big.extend_from_slice(b"\"}}\n");

        let mut reader = Cursor::new(big);
        let mut writer = Vec::new();
        let outcome = serve_connection(&mut reader, &mut writer, backend.as_ref(), &refusals)
            .expect("serve_connection");
        assert_eq!(outcome, ConnectionOutcome::RefusedMalformed);

        let lines: Vec<&[u8]> = writer
            .split(|&b| b == b'\n')
            .filter(|l| !l.is_empty())
            .collect();
        assert_eq!(lines.len(), 1, "must emit exactly one 802 envelope frame");
        let ServerFrame::Envelope { envelope, .. } =
            decode_server_frame(lines[0], &refusals).expect("valid server envelope frame")
        else {
            panic!("expected envelope frame");
        };
        assert_eq!(envelope.refusal_code(), Some("FROB-REF-802"));
        assert_eq!(envelope.exit().code(), 3);
    }

    #[test]
    fn malformed_frame_gets_envelope() {
        let refusals = fixture::synthetic_refusals();
        let backend = fixture::synthetic_dispatch_service();
        let mut reader = Cursor::new(b"{\"rpc\":\"frob.rpc/2\"}\n".to_vec());
        let mut writer = Vec::new();
        let outcome = serve_connection(&mut reader, &mut writer, backend.as_ref(), &refusals)
            .expect("serve_connection");
        assert_eq!(outcome, ConnectionOutcome::RefusedMalformed);

        let lines: Vec<&[u8]> = writer
            .split(|&b| b == b'\n')
            .filter(|l| !l.is_empty())
            .collect();
        assert_eq!(lines.len(), 1);
        let ServerFrame::Envelope { envelope, .. } =
            decode_server_frame(lines[0], &refusals).expect("valid server envelope frame")
        else {
            panic!("expected envelope frame");
        };
        assert_eq!(envelope.refusal_code(), Some("FROB-REF-802"));
        assert_eq!(envelope.exit().code(), 3);
    }

    #[test]
    fn client_drop_leaves_server_alive() {
        let refusals = fixture::synthetic_refusals();
        let backend = fixture::synthetic_dispatch_service();
        let sock_path = test_socket_path("drop100");
        let server = RpcServerHandle::spawn_current_uid(&sock_path, backend, refusals.clone())
            .expect("spawn server");

        let valid_req = Request::new(
            "design.check",
            Tree::parse(br#"{"design":"quad-a"}"#).unwrap(),
        );
        let full_line = format!("{}\n", encode_request("1", &valid_req).unwrap().as_str());
        let bytes = full_line.as_bytes();

        // Drop 100 client connections at deterministic pseudo-random byte offsets.
        let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
        for i in 0..100usize {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let cut = usize::try_from(state % (bytes.len() as u64)).unwrap_or(0);
            let mut stream = UnixStream::connect(&sock_path).expect("connect drop client");
            if i % 5 == 0 {
                // Send full request, then drop immediately before reading response.
                let _ = stream.write_all(bytes);
            } else if cut > 0 {
                // Send partial frame prefix, then close cleanly without newline.
                let _ = stream.write_all(&bytes[..cut]);
            }
            let _ = stream.shutdown(Shutdown::Both);
            drop(stream);
        }

        // The 101st call over Socket MUST succeed cleanly!
        let client = DaemonClient::new(&sock_path, refusals).expect("daemon client");
        let socket_backend = Socket::new(client);
        let cx = DispatchCx::new();
        let stream = socket_backend
            .submit(&cx, &valid_req)
            .expect("101st call must succeed after 100 dropped connections");
        let (_progress, env) = stream.into_parts().expect("terminal envelope");
        assert_eq!(env.verb(), "design.check");
        assert_eq!(env.exit().code(), 0);

        let _stats = server.shutdown().expect("clean shutdown");
    }
}
