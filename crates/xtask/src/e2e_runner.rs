//! Run shell tests in the source tree Cargo compiled. The outer RCH invocation pins the tree.

use std::ffi::OsStr;
use std::path::{Component, Path};
use std::process::Command;

pub use std::process::Output as ScriptRun;

/// Run a workspace-relative `.sh` file with the given arguments and environment overrides.
/// A nonzero script exit is an ordinary result. Path and launch errors are returned separately.
pub fn run_script(
    rel_path: &str,
    args: &[&str],
    env: &[(&str, &str)],
) -> Result<ScriptRun, String> {
    let path = Path::new(rel_path);
    if path.is_absolute() || path.components().any(|part| part == Component::ParentDir) {
        return Err(format!("script outside workspace: {rel_path}"));
    }
    if path.extension() != Some(OsStr::new("sh")) {
        return Err(format!("script path must end in .sh: {rel_path}"));
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .map_err(|err| format!("resolve Cargo workspace: {err}"))?;
    let script = root
        .join(path)
        .canonicalize()
        .map_err(|err| format!("resolve workspace script {rel_path}: {err}"))?;
    if !script.starts_with(&root) {
        return Err(format!("script outside workspace: {rel_path}"));
    }
    if !script.is_file() || script.extension() != Some(OsStr::new("sh")) {
        return Err(format!("script must resolve to a .sh file: {rel_path}"));
    }
    let run = Command::new("bash")
        .arg(script)
        .args(args)
        .envs(env.iter().copied())
        .current_dir(root)
        .output()
        .map_err(|err| format!("run workspace script {rel_path}: {err}"))?;
    print!("{}", String::from_utf8_lossy(&run.stdout));
    eprint!("{}", String::from_utf8_lossy(&run.stderr));
    Ok(run)
}

#[cfg(test)]
mod tests {
    use super::run_script;

    #[test]
    fn refuses_escape_and_preserves_script_exit() {
        let err = run_script("../outside.sh", &[], &[]).unwrap_err();
        assert!(err.contains("outside workspace"), "{err}");
        let run = run_script(
            "crates/xtask/tests/fixtures/e2e_runner_exit_3.sh",
            &["fixture argument"],
            &[("FROB_RUNNER_FIXTURE", "fixture environment")],
        )
        .expect("launch runner fixture");
        assert_eq!(run.status.code(), Some(3));
        assert_eq!(run.stdout, b"fixture argument\n");
        assert_eq!(run.stderr, b"fixture environment\n");
    }
}
