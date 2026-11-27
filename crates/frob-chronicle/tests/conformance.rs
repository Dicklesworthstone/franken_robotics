use frob_chronicle::m0::{DEFAULT_STEPS, Manifest, execution_row, new_trace, request_key};
use frob_core::{canon::to_canonical, id::hash_blob};

#[test]
fn manifest_canonical_and_stable() {
    let mut manifest = Manifest {
        model_id: "m:fixture",
        foundation_pin: "fixture",
        steps: DEFAULT_STEPS,
        seed: u64::MAX,
        perturbed: false,
    };
    let bytes = to_canonical(&manifest).unwrap();
    let digest = manifest.digest().unwrap();
    assert_eq!(to_canonical(&manifest).unwrap(), bytes);
    assert_eq!(hash_blob(bytes.as_bytes()), digest);
    assert!(bytes.as_str().contains(r#""seed":"18446744073709551615""#));
    assert!(!bytes.as_str().contains("perturb_ulp"));
    let first = new_trace(1_700_000_000_000).unwrap();
    let second = new_trace(1_700_000_000_000).unwrap();
    assert_ne!(first, second);
    assert_eq!(first.timestamp_ms(), second.timestamp_ms());
    assert_ne!(
        execution_row("r:fixture", first, 10).unwrap(),
        execution_row("r:fixture", second, 10).unwrap()
    );
    assert_eq!(manifest.digest().unwrap(), digest);
    manifest.perturbed = true;
    assert_ne!(manifest.digest().unwrap(), digest);
    manifest.perturbed = false;
    manifest.seed -= 1;
    assert_ne!(manifest.digest().unwrap(), digest);
    assert_ne!(
        request_key("ab", "c").unwrap(),
        request_key("a", "bc").unwrap()
    );
}
