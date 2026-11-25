//! Integration checks for `frob_core::perf::units` (`N36-03`, KA-806, CX-374).

use frob_core::perf::units::{
    Decimation, HardwareKey, ORDERING_DETAIL, ORDERING_POINTER, RateSchemaErrorKind, Rational,
    Settings, check_group_ordering, parse_rates_json,
};

#[test]
fn ka806_both_planted_reports_rejected() {
    // Planted report 1: 400k env-steps/s training (d=10) beside 2.0M world-steps/s physics (d=10).
    let report_valid_schema = br#"{
        "rates": [
            {
                "value": "400000",
                "unit": "env_steps_per_s",
                "decimation": "10/1",
                "h_s": "1/500",
                "scope": "training",
                "inclusion": {
                    "observe_reward_reset": true,
                    "policy_inference": true,
                    "learning": true
                },
                "settings": {
                    "kind": "matched",
                    "h_s": "1/500",
                    "d": "10/1",
                    "newton_iterations": 4
                },
                "counts": { "valid": 400000, "degraded": 0, "faulted": 0 },
                "wall_s": "1"
            },
            {
                "value": "2000000",
                "unit": "world_steps_per_s",
                "decimation": "10/1",
                "h_s": "1/500",
                "scope": "physics_only",
                "inclusion": {
                    "observe_reward_reset": false,
                    "policy_inference": false,
                    "learning": false
                },
                "settings": {
                    "kind": "matched",
                    "h_s": "1/500",
                    "d": "10/1",
                    "newton_iterations": 4
                },
                "counts": { "valid": 2000000, "degraded": 0, "faulted": 0 },
                "wall_s": "1"
            }
        ]
    }"#;
    let parsed = parse_rates_json(report_valid_schema).expect("well-formed KA-806 JSON parses");
    assert_eq!(parsed.len(), 2);

    let key = HardwareKey {
        machine: "trj".to_owned(),
        device: "rtx-4090-0".to_owned(),
        world_count: 4096,
        workload: "Q".to_owned(),
        settings: Settings::Matched {
            h: Rational::new(1, 500).unwrap(),
            d: Decimation::integer(10).unwrap(),
            newton_iterations: 4,
        },
    };
    let err = check_group_ordering(&key, &parsed).expect_err("KA-806 must violate INV-PERF-01");
    assert_eq!(err.detail, ORDERING_DETAIL);
    assert_eq!(err.pointer, ORDERING_POINTER);
    assert_eq!(err.slower_env_steps_per_s, 400_000.0);
    assert_eq!(err.faster_env_steps_per_s, 200_000.0);

    // Planted report 2 (KA-806 negative control): same report with decimation omitted -> rejected at parse time.
    let report_no_decimation = br#"{
        "rates": [
            {
                "value": "400000",
                "unit": "env_steps_per_s",
                "h_s": "1/500",
                "scope": "training",
                "inclusion": {
                    "observe_reward_reset": true,
                    "policy_inference": true,
                    "learning": true
                },
                "settings": {
                    "kind": "matched",
                    "h_s": "1/500",
                    "d": "10/1",
                    "newton_iterations": 4
                },
                "counts": { "valid": 400000, "degraded": 0, "faulted": 0 },
                "wall_s": "1"
            },
            {
                "value": "2000000",
                "unit": "world_steps_per_s",
                "h_s": "1/500",
                "scope": "physics_only",
                "inclusion": {
                    "observe_reward_reset": false,
                    "policy_inference": false,
                    "learning": false
                },
                "settings": {
                    "kind": "matched",
                    "h_s": "1/500",
                    "d": "10/1",
                    "newton_iterations": 4
                },
                "counts": { "valid": 2000000, "degraded": 0, "faulted": 0 },
                "wall_s": "1"
            }
        ]
    }"#;
    let schema_err =
        parse_rates_json(report_no_decimation).expect_err("omitted decimation must fail at parse");
    assert_eq!(schema_err.kind, RateSchemaErrorKind::MissingDecimation);
    assert_eq!(schema_err.pointer, "rates[0].decimation");
}

#[test]
fn contract_md_documents_perf_units_and_compile_fail_guards() {
    let contract = include_str!("../CONTRACT.md");
    for needle in [
        "perf::units",
        "Decimation",
        "INV-PERF-01",
        "FROB-REF-862",
        "rates-violate-unit-ordering",
        "rates.ordering",
    ] {
        assert!(
            contract.contains(needle),
            "crates/frob-core/CONTRACT.md must document `{needle}`"
        );
    }
    let units_src = include_str!("../src/perf/units.rs");
    assert!(units_src.contains("compile_fail,E0308"));
    assert!(units_src.contains("compile_fail,E0451"));
}
