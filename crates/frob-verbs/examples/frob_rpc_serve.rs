//! Thin companion `frob.rpc/1` server binary built over `frob_verbs::dispatch::rpc::serve`
//! for `scripts/e2e/tether_dispatch.sh` (bead N32a-04).
//!
//! Usage: `frob_rpc_serve <socket-path>`
//! Binds `<socket-path>` with mode `0600`, prints `READY <socket-path>` to stdout, and
//! serves the synthetic dispatch fixture until terminated.

use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64};

use frob_verbs::dispatch::fixture::{synthetic_dispatch_service, synthetic_refusals};
use frob_verbs::dispatch::rpc::serve;
use frob_verbs::dispatch::socket::{bind_socket_0600, current_uid};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(sock_arg) = args.next() else {
        let _ = writeln!(io::stderr(), "usage: frob_rpc_serve <socket-path>");
        return ExitCode::from(3);
    };
    let sock_path = PathBuf::from(sock_arg);
    let uid = match current_uid() {
        Ok(u) => u,
        Err(e) => {
            let _ = writeln!(io::stderr(), "uid lookup failed: {e}");
            return ExitCode::from(9);
        }
    };
    let listener = match bind_socket_0600(&sock_path, uid) {
        Ok(l) => l,
        Err(e) => {
            let _ = writeln!(io::stderr(), "bind {} failed: {e}", sock_path.display());
            return ExitCode::from(6);
        }
    };

    if writeln!(io::stdout(), "READY {}", sock_path.display()).is_err()
        || io::stdout().flush().is_err()
    {
        return ExitCode::from(9);
    }

    let backend = synthetic_dispatch_service();
    let refusals = synthetic_refusals();
    let stop = Arc::new(AtomicBool::new(false));
    let counter = Arc::new(AtomicU64::new(0));
    match serve(listener, backend, refusals, stop, Some(counter)) {
        Ok(_) => ExitCode::from(0),
        Err(e) => {
            let _ = writeln!(io::stderr(), "rpc serve error: {e}");
            ExitCode::from(9)
        }
    }
}
