//! Unit tests for registered `frob-e2e` script checks (`STD-E2E`).

use frob_conformance::e2e::checks::single_envelope::check_str;
use frob_conformance::e2e::checks::{CheckOutput, CheckRegistry};

#[test]
fn check_single_envelope() {
    // Exactly one JSON object with surrounding whitespace passes.
    let ok = check_str("  {\"schema\":\"frob.envelope/1\",\"ok\":true,\"exit\":0}\n  ");
    assert!(ok.ok, "expected single object to pass, got {ok:?}");
    assert!(ok.first_diff.is_none());

    // Two JSON objects fail with first_diff naming the exact byte offset of the second object.
    let two = "{\"ok\":true}\n{\"ok\":false}";
    let res_two = check_str(two);
    assert!(!res_two.ok, "expected two JSON objects to fail");
    let (path, obs, exp) = res_two.first_diff.expect("first_diff must be populated");
    assert_eq!(path, "byte:12", "second object starts at byte 12");
    assert!(obs.contains("12"), "observed diff should name byte offset 12: {obs}");
    assert_eq!(exp, "end-of-input");

    // One JSON object followed by trailing non-whitespace text fails with exact byte offset.
    let trailing = "{\"ok\":true}   WARNING: trailing text";
    let res_trail = check_str(trailing);
    assert!(!res_trail.ok, "expected trailing text to fail");
    let (tpath, tobs, texp) = res_trail.first_diff.expect("first_diff must be populated");
    assert_eq!(tpath, "byte:14", "trailing text starts at byte 14");
    assert!(tobs.contains("14"), "observed diff should name byte offset 14: {tobs}");
    assert_eq!(texp, "end-of-input");

    // Non-object (JSON array) fails at byte 0.
    let arr = check_str("[1, 2, 3]");
    assert!(!arr.ok);
    let (apath, aobs, aexp) = arr.first_diff.expect("first_diff for array");
    assert_eq!(apath, "byte:0");
    assert_eq!(aobs, "array");
    assert_eq!(aexp, "object");

    // Empty input fails at byte 0.
    let empty = check_str("   \n\t ");
    assert!(!empty.ok);
    let (epath, _, _) = empty.first_diff.expect("first_diff for empty input");
    assert_eq!(epath, "byte:0");
}

fn dummy_check(_args: &[String]) -> Result<CheckOutput, String> {
    Ok(CheckOutput {
        ok: true,
        observed: "ok".into(),
        expected: "ok".into(),
        first_diff: None,
    })
}

#[test]
fn check_registry_names_unique() {
    let mut reg = CheckRegistry::new();
    reg.register("alpha", dummy_check)
        .expect("first registration succeeds");
    let dup = reg.register("alpha", dummy_check);
    assert!(dup.is_err(), "registering duplicate check name must fail");
    assert!(
        dup.unwrap_err().contains("alpha"),
        "error message should name duplicate check"
    );

    // Default registry also rejects re-registering built-in single-envelope.
    let mut def = CheckRegistry::default_registry().expect("default registry valid");
    assert!(def.register("single-envelope", dummy_check).is_err());
}
