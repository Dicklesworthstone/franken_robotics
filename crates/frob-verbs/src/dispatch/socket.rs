//! Local Unix-domain socket hygiene (`0600`, owner verification) and the [`Socket`]
//! back end (`DaemonClient`, `Socket`, `SocketConfig`) (bead N32a-04, ADR-TETHER-03).
//!
//! Available under BOTH `--no-default-features --features registry-only` (KILL-MCP
//! Fallback 1, ADR-TETHER-02) AND the default `handlers` build. Zero `unsafe`, zero
//! external dependencies, zero file deletion (COMMON.md 9b).

use std::fmt;
use std::fs;
use std::io::{BufReader, Write};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use frob_core::registry::refusals::RefusalRegistry;

use super::pipeline::Pipeline;
use super::rpc::{
    MAX_FRAME_BYTES, ReadLineResult, ServerFrame, decode_server_frame, encode_attach,
    encode_request, read_line_bounded,
};
use super::{Dispatch, DispatchCx, DispatchError, Frame, JobRef, JobStream, Request};

/// Errors binding, validating, or connecting to a local `frobd` Unix-domain socket.
#[derive(Debug)]
pub enum SocketError {
    /// Socket path already exists and is owned by another user.
    WrongOwner {
        path: PathBuf,
        expected_uid: u32,
        observed_uid: u32,
    },
    /// Socket path exists with permissions other than `0600`.
    WrongPermissions { path: PathBuf, observed_mode: u32 },
    /// Path exists and is not a Unix-domain socket.
    NotASocket { path: PathBuf },
    /// Refusing to overwrite an existing socket file on bind.
    AlreadyExists { path: PathBuf },
    /// Could not determine current effective user ID.
    UidLookup(String),
    /// Underlying I/O error.
    Io(std::io::Error),
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongOwner {
                path,
                expected_uid,
                observed_uid,
            } => write!(
                f,
                "socket `{}` owned by uid {observed_uid}, expected uid {expected_uid}",
                path.display()
            ),
            Self::WrongPermissions {
                path,
                observed_mode,
            } => write!(
                f,
                "socket `{}` has mode {:04o}, required 0600",
                path.display(),
                observed_mode
            ),
            Self::NotASocket { path } => {
                write!(
                    f,
                    "path `{}` exists and is not a Unix-domain socket",
                    path.display()
                )
            }
            Self::AlreadyExists { path } => {
                write!(f, "socket path `{}` already exists", path.display())
            }
            Self::UidLookup(m) => write!(f, "failed to determine current effective uid: {m}"),
            Self::Io(e) => write!(f, "socket I/O error: {e}"),
        }
    }
}

impl std::error::Error for SocketError {}

impl From<std::io::Error> for SocketError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

static CACHED_UID: OnceLock<Result<u32, String>> = OnceLock::new();

/// Effective user ID of the current process (zero `unsafe`, zero external crates).
///
/// Uses `/proc/self` metadata on Linux; falls back once to `id -u` on macOS/BSD.
pub fn current_uid() -> Result<u32, SocketError> {
    CACHED_UID
        .get_or_init(|| {
            if let Ok(meta) = fs::metadata("/proc/self") {
                return Ok(meta.uid());
            }
            let out = Command::new("id")
                .arg("-u")
                .output()
                .map_err(|e| format!("failed to run `id -u`: {e}"))?;
            if !out.status.success() {
                return Err(format!("`id -u` exited with {}", out.status));
            }
            let s = std::str::from_utf8(&out.stdout)
                .map_err(|e| format!("non-UTF-8 `id -u` output: {e}"))?
                .trim();
            s.parse::<u32>()
                .map_err(|e| format!("invalid uid `{s}`: {e}"))
        })
        .clone()
        .map_err(SocketError::UidLookup)
}

/// Validate an existing Unix-domain socket path: must be a socket, owned by `expected_uid`,
/// with exact permissions `0600`.
pub fn validate_existing_socket(path: &Path, expected_uid: u32) -> Result<(), SocketError> {
    let meta = fs::symlink_metadata(path)?;
    if !meta.file_type().is_socket() {
        return Err(SocketError::NotASocket {
            path: path.to_path_buf(),
        });
    }
    let observed_uid = meta.uid();
    if observed_uid != expected_uid {
        return Err(SocketError::WrongOwner {
            path: path.to_path_buf(),
            expected_uid,
            observed_uid,
        });
    }
    let mode = meta.mode() & 0o777;
    if mode != 0o600 {
        return Err(SocketError::WrongPermissions {
            path: path.to_path_buf(),
            observed_mode: mode,
        });
    }
    Ok(())
}

/// Bind a fresh Unix-domain socket at `path` with mode `0600`.
///
/// If `path` already exists, refuses cleanly: reports [`SocketError::WrongOwner`] if owned
/// by another user, [`SocketError::WrongPermissions`] if mode differs from `0600`,
/// [`SocketError::NotASocket`] if not a socket, or [`SocketError::AlreadyExists`] otherwise.
pub fn bind_socket_0600(path: &Path, expected_uid: u32) -> Result<UnixListener, SocketError> {
    if let Ok(meta) = fs::symlink_metadata(path) {
        let observed_uid = meta.uid();
        if observed_uid != expected_uid {
            return Err(SocketError::WrongOwner {
                path: path.to_path_buf(),
                expected_uid,
                observed_uid,
            });
        }
        let mode = meta.mode() & 0o777;
        if mode != 0o600 {
            return Err(SocketError::WrongPermissions {
                path: path.to_path_buf(),
                observed_mode: mode,
            });
        }
        if !meta.file_type().is_socket() {
            return Err(SocketError::NotASocket {
                path: path.to_path_buf(),
            });
        }
        return Err(SocketError::AlreadyExists {
            path: path.to_path_buf(),
        });
    }

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            fs::create_dir_all(parent)?;
            let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o700));
        }
    }

    let listener = UnixListener::bind(path)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    validate_existing_socket(path, expected_uid)?;
    Ok(listener)
}

/// Connect to an existing `frobd` Unix-domain socket after verifying owner and mode `0600`.
pub fn connect_socket_checked(path: &Path, expected_uid: u32) -> Result<UnixStream, SocketError> {
    validate_existing_socket(path, expected_uid)?;
    let stream = UnixStream::connect(path)?;
    Ok(stream)
}

/// Configuration for [`DaemonClient`].
#[derive(Debug, Clone)]
pub struct SocketConfig {
    pub path: PathBuf,
    pub expected_uid: u32,
    pub io_timeout: Duration,
}

impl SocketConfig {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, SocketError> {
        Ok(Self {
            path: path.into(),
            expected_uid: current_uid()?,
            io_timeout: Duration::from_secs(30),
        })
    }

    #[must_use]
    pub const fn with_expected_uid(mut self, uid: u32) -> Self {
        self.expected_uid = uid;
        self
    }

    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.io_timeout = timeout;
        self
    }
}

/// Client for the local `frobd` Unix-domain socket speaking `frob.rpc/1`.
pub struct DaemonClient {
    config: SocketConfig,
    refusals: RefusalRegistry,
    next_id: AtomicU64,
}

impl fmt::Debug for DaemonClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DaemonClient")
            .field("path", &self.config.path)
            .field("expected_uid", &self.config.expected_uid)
            .finish_non_exhaustive()
    }
}

impl DaemonClient {
    /// Construct a client targeting `path` owned by the current effective UID.
    pub fn new(path: impl Into<PathBuf>, refusals: RefusalRegistry) -> Result<Self, SocketError> {
        Ok(Self::with_config(SocketConfig::new(path)?, refusals))
    }

    /// Construct a client with an explicit [`SocketConfig`].
    #[must_use]
    pub const fn with_config(config: SocketConfig, refusals: RefusalRegistry) -> Self {
        Self {
            config,
            refusals,
            next_id: AtomicU64::new(1),
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.config.path
    }

    #[must_use]
    pub const fn refusals(&self) -> &RefusalRegistry {
        &self.refusals
    }

    fn alloc_id(&self) -> String {
        self.next_id.fetch_add(1, Ordering::Relaxed).to_string()
    }

    /// Return `true` iff the socket exists with valid owner/mode and accepts a connection.
    #[must_use]
    pub fn is_reachable(&self) -> bool {
        connect_socket_checked(&self.config.path, self.config.expected_uid).is_ok()
    }

    /// Send one `frob.rpc/1` request frame and collect all progress frames + terminal envelope.
    pub fn call_request(&self, cx: &DispatchCx, req: &Request) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        let id = self.alloc_id();
        let line = encode_request(&id, req)?;
        self.exchange(cx, &id, line.as_bytes())
    }

    /// Send one `frob.rpc/1` attach frame and collect all progress frames + terminal envelope.
    pub fn call_attach(
        &self,
        cx: &DispatchCx,
        job: &JobRef,
        session: Option<&str>,
    ) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        let id = self.alloc_id();
        let line = encode_attach(&id, job, session)?;
        self.exchange(cx, &id, line.as_bytes())
    }

    fn exchange(
        &self,
        cx: &DispatchCx,
        expected_id: &str,
        request_line: &[u8],
    ) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        let mut stream = connect_socket_checked(&self.config.path, self.config.expected_uid)?;
        stream.set_read_timeout(Some(self.config.io_timeout))?;
        stream.set_write_timeout(Some(self.config.io_timeout))?;

        stream.write_all(request_line)?;
        stream.write_all(b"\n")?;
        stream.flush()?;

        let mut reader = BufReader::new(stream);
        let mut frames = Vec::new();
        loop {
            cx.checkpoint()?;
            match read_line_bounded(&mut reader, MAX_FRAME_BYTES)? {
                ReadLineResult::Line(bytes) => {
                    let sf = decode_server_frame(&bytes, &self.refusals)
                        .map_err(|e| DispatchError::Protocol(e.to_string()))?;
                    match sf {
                        ServerFrame::Progress { id, progress } => {
                            if id != expected_id {
                                return Err(DispatchError::Protocol(format!(
                                    "server echoed id `{id}`, expected `{expected_id}`"
                                )));
                            }
                            frames.push(Frame::Progress(progress));
                        }
                        ServerFrame::Envelope { id, envelope } => {
                            if id != expected_id && id != "0" {
                                return Err(DispatchError::Protocol(format!(
                                    "server echoed id `{id}`, expected `{expected_id}`"
                                )));
                            }
                            frames.push(Frame::Envelope(envelope));
                            return JobStream::from_frames(frames, None);
                        }
                    }
                }
                ReadLineResult::Oversize => {
                    return Err(DispatchError::Protocol(format!(
                        "server response frame exceeded {MAX_FRAME_BYTES}-byte cap"
                    )));
                }
                ReadLineResult::Eof | ReadLineResult::TruncatedEof => {
                    return Err(DispatchError::Protocol(
                        "server closed connection before sending terminal envelope".to_owned(),
                    ));
                }
            }
        }
    }
}

/// Socket back end (`ADR-TETHER-03` back end 2): validates steps 1–2 locally when a
/// [`Pipeline`] is attached, then forwards requests over `frob.rpc/1` to `frobd`.
///
/// Exposed under BOTH `registry-only` and `handlers` builds.
#[derive(Debug)]
pub struct Socket {
    client: DaemonClient,
    local_pipeline: Option<Pipeline>,
}

impl Socket {
    /// Construct a `Socket` back end forwarding directly over `client`.
    #[must_use]
    pub const fn new(client: DaemonClient) -> Self {
        Self {
            client,
            local_pipeline: None,
        }
    }

    /// Attach a local [`Pipeline`] so steps 1 and 2 (Locate + Validate, including
    /// `forbidden_keys`) run locally before forwarding over the socket (N32a-04).
    #[must_use]
    pub fn with_local_pipeline(mut self, pipeline: Pipeline) -> Self {
        self.local_pipeline = Some(pipeline);
        self
    }

    #[must_use]
    pub const fn client(&self) -> &DaemonClient {
        &self.client
    }
}

impl Dispatch for Socket {
    fn submit(&self, cx: &DispatchCx, req: &Request) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        if let Some(pipe) = &self.local_pipeline {
            if let Err(refusal_env) = pipe.validate_request(req)? {
                return Ok(JobStream::single_envelope(refusal_env));
            }
        }
        self.client.call_request(cx, req)
    }

    fn attach(&self, cx: &DispatchCx, job: &JobRef) -> Result<JobStream, DispatchError> {
        cx.checkpoint()?;
        self.client.call_attach(cx, job, None)
    }
}

/// Generate a unique short scratch socket path guaranteed well below Unix `SUN_LEN` (104 B).
/// Never deletes files or directories (COMMON.md 9b).
#[cfg(test)]
pub(crate) fn test_socket_path(tag: &str) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    let base = match std::env::var_os("BIG_SCRATCH").or_else(|| std::env::var_os("TMPDIR")) {
        Some(dir) if dir.len() <= 48 => PathBuf::from(dir),
        _ => PathBuf::from("/tmp"),
    };
    let dir = base.join(format!("fbh-{pid}-{nanos:09}-{n}"));
    let _ = fs::create_dir_all(&dir);
    let _ = fs::set_permissions(&dir, fs::Permissions::from_mode(0o700));
    dir.join(format!("{tag}.sock"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_mode_0600() {
        let uid = current_uid().expect("current uid");
        let sock = test_socket_path("mode");
        let listener = bind_socket_0600(&sock, uid).expect("bind socket 0600");
        let meta = fs::symlink_metadata(&sock).expect("socket metadata");
        assert!(
            meta.file_type().is_socket(),
            "bound path must be a Unix socket"
        );
        assert_eq!(
            meta.mode() & 0o777,
            0o600,
            "socket must be created with mode 0600"
        );
        assert_eq!(meta.uid(), uid);

        // Re-binding an existing 0600 socket is refused (`AlreadyExists`).
        assert!(matches!(
            bind_socket_0600(&sock, uid),
            Err(SocketError::AlreadyExists { .. })
        ));

        // Loosening permissions to 0644 causes BOTH validate_existing_socket and
        // bind_socket_0600 / connect_socket_checked to refuse with WrongPermissions.
        fs::set_permissions(&sock, fs::Permissions::from_mode(0o644)).expect("chmod 0644");
        assert!(matches!(
            validate_existing_socket(&sock, uid),
            Err(SocketError::WrongPermissions {
                observed_mode: 0o644,
                ..
            })
        ));
        assert!(matches!(
            bind_socket_0600(&sock, uid),
            Err(SocketError::WrongPermissions {
                observed_mode: 0o644,
                ..
            })
        ));
        assert!(matches!(
            connect_socket_checked(&sock, uid),
            Err(SocketError::WrongPermissions {
                observed_mode: 0o644,
                ..
            })
        ));
        drop(listener);
    }

    #[test]
    fn socket_wrong_owner_refused() {
        let uid = current_uid().expect("current uid");
        let wrong_uid = uid.wrapping_add(1);
        let sock = test_socket_path("owner");
        let listener = bind_socket_0600(&sock, uid).expect("bind socket 0600");

        // Existing socket owned by `uid` must be refused when `expected_uid` differs.
        assert!(matches!(
            validate_existing_socket(&sock, wrong_uid),
            Err(SocketError::WrongOwner {
                expected_uid,
                observed_uid,
                ..
            }) if expected_uid == wrong_uid && observed_uid == uid
        ));
        assert!(matches!(
            bind_socket_0600(&sock, wrong_uid),
            Err(SocketError::WrongOwner {
                expected_uid,
                observed_uid,
                ..
            }) if expected_uid == wrong_uid && observed_uid == uid
        ));
        assert!(matches!(
            connect_socket_checked(&sock, wrong_uid),
            Err(SocketError::WrongOwner {
                expected_uid,
                observed_uid,
                ..
            }) if expected_uid == wrong_uid && observed_uid == uid
        ));
        drop(listener);
    }
}
