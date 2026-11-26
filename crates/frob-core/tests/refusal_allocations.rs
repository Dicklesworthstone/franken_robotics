#![deny(unsafe_code)]

//! Cross-chapter refusal allocation ledger, pure allocator (Rules 1–8 + Review Note 6),
//! exit rules, interim map, neighbour pairs, printout, and ADR-NUM-refusal-8xx tests (`N35-33`).

use std::collections::{BTreeMap, BTreeSet};

use frob_core::registry::refusal_allocations::{
    AllocationError, AllocationInput, AllocationLedger, AllocationRequest, AllocationStatus,
    EMBEDDED_REFUSAL_ALLOCATIONS_TOML, ExistingCode, HoldEntry, InterimSpec, NeighbourPair,
    PLAN_RESERVED_BLOCKS, PRINTED_ACTIVE_CODE_COUNT, allocate_round, format_refusal_code,
    printed_plan_codes, refusal_code_token_eq, render_refusals_markdown, run_cross_chapter_checks,
    text_contains_refusal_token, validate_ledger_and_interims,
};
use frob_core::registry::refusals::{
    EMBEDDED_REFUSALS_TOML, Milestone, RefusalRegistry, RefusalRegistryError,
};

const EMBEDDED_REFUSALS_MD: &str = include_str!("../../../docs/generated/refusals.md");

fn req(
    slug: &str,
    owner: &str,
    defined_in: &str,
    family: u8,
    subfamily: Option<&str>,
    proposed: Option<u16>,
    condition: &str,
) -> AllocationRequest {
    AllocationRequest {
        request: slug.into(),
        requested_by: owner.into(),
        defined_in: defined_in.into(),
        order_hint: 0,
        condition: condition.into(),
        family,
        subfamily: subfamily.map(str::to_owned),
        proposed,
        interim: InterimSpec::None,
        interim_detail: None,
        interim_pointer: None,
        assigned: None,
        merged_into: Vec::new(),
        status: AllocationStatus::Requested,
        reason: None,
    }
}

fn synthetic_registry(extra_exit_rules: &str, rows_toml: &str) -> String {
    format!(
        r#"schema = "frob.refusals/1"

[exit_rule]
"1xx" = 3
"2xx" = 3
"3xx" = 3
"4xx" = 3
"5xx" = 3
"6xx" = 6
"70x" = 4
"72x" = 5
"73x" = 5
"74x" = 4
"76x" = 4
"79x" = 5
"8xx" = 3
"9xx" = 9
{extra_exit_rules}

{rows_toml}
"#
    )
}

fn reg_row(code: &str, exit: u8, defined_in: &str, related: &str) -> String {
    let retry = if code.starts_with("FROB-REF-9") {
        "never"
    } else {
        "after-change"
    };
    format!(
        r#"[[refusal]]
code = "{code}"
meaning = "Synthetic test refusal {code}"
defined_in = "{defined_in}"
exit = {exit}
retry = "{retry}"
repairs = ["First repair for {code}", "Second repair for {code}"]
raised_by = ["N35-33"]
{related}
"#
    )
}

fn lcg_shuffle<T>(items: &mut [T], mut seed: u64) {
    let n = items.len();
    for i in (1..n).rev() {
        seed = seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let j = ((seed >> 33) as usize) % (i + 1);
        items.swap(i, j);
    }
}

#[test]
fn alloc_order_independent() {
    let reg = RefusalRegistry::load().expect("live refusals.toml loads");
    let ledger = AllocationLedger::load().expect("live refusal_allocations.toml loads");
    assert_eq!(printed_plan_codes().len(), PRINTED_ACTIVE_CODE_COUNT);

    let input = AllocationInput::from_plan_and_ledger(&reg, &ledger);
    let settled = allocate_round(&input).expect("existing round remains intact");
    for request in &ledger.requests {
        assert_eq!(settled.by_slug(&request.request), Some(request));
    }

    // A run on previously unallocated requests must also be order independent.
    let mut fresh_input = input;
    fresh_input
        .existing
        .retain(|code| printed_plan_codes().contains_key(&code.number));
    for r in &mut fresh_input.requests {
        if r.status != AllocationStatus::Merged {
            r.assigned = None;
            r.status = AllocationStatus::Requested;
            r.reason = None;
        }
    }
    let baseline = allocate_round(&fresh_input).expect("baseline allocation succeeds");

    // 100 seeded shuffles produce the exact same assignment every time.
    for seed in 1u64..=100 {
        let mut shuffled = fresh_input.clone();
        lcg_shuffle(&mut shuffled.requests, seed);
        let out = allocate_round(&shuffled).expect("shuffled allocation succeeds");
        assert_eq!(
            out.results, baseline.results,
            "shuffle seed={seed} changed allocation outcome"
        );
    }

    // Acceptance item 1: Marrow 128 assigned, no first-round row takes 127.
    assert_eq!(
        settled
            .by_slug("lifted-law-without-turn-state")
            .unwrap()
            .assigned
            .as_deref(),
        Some("FROB-REF-128")
    );
    assert!(
        settled
            .results
            .iter()
            .all(|r| r.assigned.as_deref() != Some("FROB-REF-127")),
        "no first-round row may take 127 (reserved for N14-REF second round)"
    );
    assert_eq!(
        settled
            .by_slug("stream-entity-hash-collision")
            .unwrap()
            .assigned
            .as_deref(),
        Some("FROB-REF-253")
    );
}

#[test]
fn later_round_reserves_settled_and_live_codes() {
    let reg = RefusalRegistry::load().unwrap();
    let mut ledger = AllocationLedger::load().unwrap();
    let settled = ledger
        .requests
        .iter()
        .find(|r| r.request == "device-receipt-incomplete")
        .unwrap()
        .clone();
    // 990 is already live, independently of what requests the caller submits.
    ledger.requests.retain(|r| r.request != "internal-fault");
    let mut input = AllocationInput::from_plan_and_ledger(&reg, &ledger);
    input.requests.extend([
        req(
            "new-before-spark",
            "N12-REF",
            "§12.7",
            9,
            None,
            Some(901),
            "new fault",
        ),
        req(
            "new-before-tether",
            "N12-REF",
            "§12.7",
            9,
            None,
            Some(990),
            "another fault",
        ),
        req(
            "wrong-chapter-proposal",
            "N14-REF",
            "§14.13",
            1,
            None,
            Some(137),
            "new design refusal",
        ),
    ]);
    let out = allocate_round(&input).unwrap();
    assert_eq!(out.by_slug(&settled.request), Some(&settled));
    for (slug, occupied) in [
        ("new-before-spark", 901),
        ("new-before-tether", 990),
        ("wrong-chapter-proposal", 137),
    ] {
        assert_ne!(
            out.by_slug(slug).unwrap().assigned_number().unwrap(),
            occupied
        );
    }
    assert_eq!(
        input
            .existing
            .iter()
            .find(|code| code.number == 540)
            .unwrap()
            .chapter,
        17
    );
}

#[test]
fn alloc_same_proposal_twice() {
    let input = AllocationInput {
        existing: vec![],
        reserved_blocks: vec![],
        retired: BTreeSet::new(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![
            req("later-ch", "N14-REF", "§14.5", 1, None, Some(127), "c2"),
            req("earlier-ch", "N13-REF", "§13.11", 1, None, Some(127), "c1"),
        ],
    };
    let out = allocate_round(&input).expect("allocation succeeds");
    assert_eq!(
        out.by_slug("earlier-ch").unwrap().assigned.as_deref(),
        Some("FROB-REF-127")
    );
    let later = out.by_slug("later-ch").unwrap();
    assert_eq!(later.status, AllocationStatus::Assigned);
    assert_ne!(later.assigned.as_deref(), Some("FROB-REF-127"));
    assert!(later.assigned.is_some(), "neither request is lost");
}

#[test]
fn alloc_merges_duplicate_condition() {
    let input = AllocationInput {
        existing: vec![ExistingCode {
            number: 540,
            chapter: 23,
            condition: "Objective metric is not smooth".into(),
        }],
        reserved_blocks: vec![],
        retired: BTreeSet::new(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![req(
            "dup-metric",
            "N16-REF",
            "§16.12",
            5,
            None,
            None,
            "  objective  metric is NOT smooth ",
        )],
    };
    let out = allocate_round(&input).expect("allocation succeeds");
    let r = out.by_slug("dup-metric").unwrap();
    assert_eq!(r.status, AllocationStatus::Merged);
    assert_eq!(r.assigned, None);
    assert_eq!(r.merged_into, vec!["FROB-REF-540".to_owned()]);
}

#[test]
fn alloc_merges_duplicate_condition_in_same_round() {
    let input = AllocationInput {
        existing: vec![],
        reserved_blocks: vec![],
        retired: BTreeSet::new(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![
            req(
                "later",
                "N23-REF",
                "§23.19",
                5,
                None,
                Some(551),
                " metric   NOT smooth ",
            ),
            req(
                "earlier",
                "N16-REF",
                "§16.12",
                5,
                None,
                Some(550),
                "Metric not smooth",
            ),
        ],
    };

    let out = allocate_round(&input).unwrap();
    let earlier = out.by_slug("earlier").unwrap();
    let later = out.by_slug("later").unwrap();
    assert_eq!(earlier.status, AllocationStatus::Assigned);
    assert_eq!(earlier.assigned.as_deref(), Some("FROB-REF-550"));
    assert_eq!(later.status, AllocationStatus::Merged);
    assert_eq!(later.assigned, None);
    assert_eq!(later.merged_into, ["FROB-REF-550"]);
}

#[test]
fn alloc_never_reuses_retired() {
    let input = AllocationInput {
        existing: vec![ExistingCode {
            number: 252,
            chapter: 35,
            condition: "existing 252".into(),
        }],
        reserved_blocks: vec![(35, 240, 252)],
        retired: BTreeSet::from([253]),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![req(
            "next-2xx", "N35-REF", "§35.11", 2, None, None, "new 2xx",
        )],
    };
    let out = allocate_round(&input).expect("allocation succeeds");
    assert_eq!(
        out.by_slug("next-2xx").unwrap().assigned.as_deref(),
        Some("FROB-REF-254")
    );
}

#[test]
fn alloc_block_rule() {
    // 1. Chapter 35 ends at 252 with 253..259 free -> receives 253.
    let base = AllocationInput {
        existing: vec![
            ExistingCode {
                number: 247,
                chapter: 35,
                condition: "c247".into(),
            },
            ExistingCode {
                number: 252,
                chapter: 35,
                condition: "c252".into(),
            },
        ],
        reserved_blocks: vec![(35, 247, 252)],
        retired: BTreeSet::new(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![req("r35", "N35-REF", "§35.11", 2, None, None, "new")],
    };
    let out1 = allocate_round(&base).expect("base succeeds");
    assert_eq!(
        out1.by_slug("r35").unwrap().assigned.as_deref(),
        Some("FROB-REF-253")
    );

    // 2. With 253 taken by another chapter (26), receives lowest free outside every block (201).
    let mut blocked = base;
    blocked.existing.push(ExistingCode {
        number: 253,
        chapter: 26,
        condition: "c253".into(),
    });
    blocked.reserved_blocks.push((26, 253, 259));
    let out2 = allocate_round(&blocked).expect("blocked succeeds");
    assert_eq!(
        out2.by_slug("r35").unwrap().assigned.as_deref(),
        Some("FROB-REF-201")
    );

    // 3. Review Note 6: §14 1xx overflow must NEVER steal 137..139 from Sinew (§15 130..=139).
    let note6 = AllocationInput {
        existing: (101u16..=129)
            .map(|n| ExistingCode {
                number: n,
                chapter: 14,
                condition: format!("c{n}"),
            })
            .collect(),
        reserved_blocks: PLAN_RESERVED_BLOCKS.to_vec(),
        retired: BTreeSet::new(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![req(
            "region-not-admissible",
            "N14-REF",
            "§14.8",
            1,
            None,
            None,
            "region not admissible",
        )],
    };
    let out3 = allocate_round(&note6).expect("note 6 succeeds");
    assert_eq!(
        out3.by_slug("region-not-admissible")
            .unwrap()
            .assigned
            .as_deref(),
        Some("FROB-REF-140"),
        "§14 overflow must skip Sinew reserved block 130..=139"
    );
}

#[test]
fn alloc_proposal_not_offered_to_others() {
    let base_existing: Vec<ExistingCode> = (720u16..=725)
        .map(|n| ExistingCode {
            number: n,
            chapter: 32,
            condition: format!("c{n}"),
        })
        .collect();

    // Case 1: §18 unproposed + §32 proposing 726 -> §32 gets 726, §18 gets 727.
    let r18 = req(
        "rut-capacity-overflow",
        "N18-REF",
        "§18.4",
        7,
        Some("72x"),
        None,
        "rut",
    );
    let r32 = req(
        "mechanism-not-admitted",
        "N32-REF",
        "§32.4",
        7,
        Some("72x"),
        Some(726),
        "mech",
    );
    let out1 = allocate_round(&AllocationInput {
        existing: base_existing.clone(),
        reserved_blocks: vec![(32, 720, 725)],
        retired: BTreeSet::new(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![r18.clone(), r32.clone()],
    })
    .expect("case 1");
    assert_eq!(
        out1.by_slug("mechanism-not-admitted")
            .unwrap()
            .assigned
            .as_deref(),
        Some("FROB-REF-726")
    );
    assert_eq!(
        out1.by_slug("rut-capacity-overflow")
            .unwrap()
            .assigned
            .as_deref(),
        Some("FROB-REF-727")
    );

    // Case 2: with 727..729 planted taken -> §32 still gets 726, §18 stays requested/range-full.
    let mut full_existing = base_existing.clone();
    for n in 727u16..=729 {
        full_existing.push(ExistingCode {
            number: n,
            chapter: 32,
            condition: format!("c{n}"),
        });
    }
    let out2 = allocate_round(&AllocationInput {
        existing: full_existing,
        reserved_blocks: vec![(32, 720, 725)],
        retired: BTreeSet::new(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![r18.clone(), r32],
    })
    .expect("case 2");
    assert_eq!(
        out2.by_slug("mechanism-not-admitted")
            .unwrap()
            .assigned
            .as_deref(),
        Some("FROB-REF-726")
    );
    let o18 = out2.by_slug("rut-capacity-overflow").unwrap();
    assert_eq!(o18.status, AllocationStatus::Requested);
    assert_eq!(o18.reason.as_deref(), Some("range-full"));

    // Case 3: with §32 removed, §18 receives 726.
    let out3 = allocate_round(&AllocationInput {
        existing: base_existing,
        reserved_blocks: vec![(32, 720, 725)],
        retired: BTreeSet::new(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![r18],
    })
    .expect("case 3");
    assert_eq!(
        out3.by_slug("rut-capacity-overflow")
            .unwrap()
            .assigned
            .as_deref(),
        Some("FROB-REF-726")
    );
}

#[test]
fn alloc_respects_subfamily() {
    let reg = RefusalRegistry::load().expect("live reg");
    let ledger = AllocationLedger::load().expect("live ledger");
    let mut input = AllocationInput::from_plan_and_ledger(&reg, &ledger);
    input.requests = vec![req(
        "adv-76x-req",
        "N26-REF",
        "§26.4",
        7,
        Some("76x"),
        Some(726), // proposal outside 76x decade must be ignored!
        "budget refusal in 76x",
    )];
    let out = allocate_round(&input).expect("76x allocation succeeds");
    assert_eq!(
        out.by_slug("adv-76x-req").unwrap().assigned.as_deref(),
        Some("FROB-REF-762"),
        "76x request must receive 762 (not 726 outside its decade)"
    );

    // 7xx without subfamily is a ledger error.
    let mut bad = input.clone();
    bad.requests = vec![req("missing-sf", "N26-REF", "§26.4", 7, None, None, "bad")];
    assert!(matches!(
        allocate_round(&bad),
        Err(AllocationError::MissingSubfamily { .. })
    ));

    // Full 76x decade leaves request requested with range-full.
    let mut full = input;
    for n in 760u16..=769 {
        full.existing.push(ExistingCode {
            number: n,
            chapter: 26,
            condition: format!("c{n}"),
        });
    }
    full.requests = vec![req(
        "full-76x",
        "N26-REF",
        "§26.4",
        7,
        Some("76x"),
        None,
        "full",
    )];
    let out_full = allocate_round(&full).expect("full decade returns range-full");
    let rf = out_full.by_slug("full-76x").unwrap();
    assert_eq!(rf.status, AllocationStatus::Requested);
    assert_eq!(rf.reason.as_deref(), Some("range-full"));
}

#[test]
fn first_round_assigns_no_8xx() {
    let reg = RefusalRegistry::load().expect("live reg");
    let ledger = AllocationLedger::load().expect("live ledger");

    let mut no_hold = AllocationInput::from_plan_and_ledger(&reg, &ledger);
    for r in &mut no_hold.requests {
        if r.status != AllocationStatus::Merged {
            r.assigned = None;
            r.status = AllocationStatus::Requested;
            r.reason = None;
        }
    }
    let out_no_hold = allocate_round(&no_hold).expect("no-hold run");
    let mut count_8xx = 0usize;
    for r in &out_no_hold.results {
        if r.family == 8 {
            count_8xx += 1;
            assert_eq!(r.status, AllocationStatus::Requested);
            assert_eq!(r.reason.as_deref(), Some("owner-decision"));
            assert_eq!(r.assigned, None);
        }
        assert!(
            !matches!(
                r.assigned.as_deref(),
                Some("FROB-REF-836" | "FROB-REF-837" | "FROB-REF-838" | "FROB-REF-839")
            ),
            "836..839 must stay free in first round"
        );
    }
    assert!(count_8xx >= 7, "expected first-round 8xx requests");

    // Adding N35x-01 hold flips 8xx requests to `held`, leaving non-8xx identical.
    let mut with_hold = no_hold;
    with_hold.holds.insert(8);
    let out_hold = allocate_round(&with_hold).expect("hold run");
    for (a, b) in out_no_hold.results.iter().zip(&out_hold.results) {
        if a.family == 8 {
            assert_eq!(b.status, AllocationStatus::Held);
            assert_eq!(b.assigned, None);
        } else {
            assert_eq!(a, b, "non-8xx assignment must be identical under 8xx hold");
        }
    }
}

#[test]
fn interim_forms_admitted() {
    let reg_toml = synthetic_registry(
        "conflict = []\nintegrity = []\ncapability = []",
        &reg_row("FROB-REF-602", 6, "§21.21", ""),
    );
    let reg = RefusalRegistry::from_bytes(reg_toml.as_bytes()).expect("reg parses");

    // 1. Bare held proposal number `interim = 838` fails parse.
    let bad_838 = r#"schema = "frob.refalloc/1"
[[request]]
request = "bad-num"
requested_by = "N32-REF"
defined_in = "§32.18"
condition = "c"
family = 8
interim = 838
status = "requested"
"#;
    assert!(matches!(
        AllocationLedger::from_bytes(bad_838.as_bytes()),
        Err(AllocationError::InvalidInterim { .. })
    ));
    let bad_array = bad_838.replace("interim = 838", "interim = [838]");
    assert!(matches!(
        AllocationLedger::from_bytes(bad_array.as_bytes()),
        Err(AllocationError::InvalidField { context, .. }) if context == "request `bad-num`.interim[0]"
    ));

    // 2. Unregistered code `interim = "FROB-REF-726"` fails validation against `reg`.
    let bad_726 = r#"schema = "frob.refalloc/1"
[[request]]
request = "bad-absent"
requested_by = "N32-REF"
defined_in = "§32.18"
condition = "c"
family = 8
interim = "FROB-REF-726"
status = "requested"
"#;
    let l726 = AllocationLedger::from_bytes(bad_726.as_bytes()).expect("parses syntactically");
    assert!(matches!(
        validate_ledger_and_interims(&reg, &l726),
        Err(AllocationError::InvalidInterim { .. })
    ));

    // 3. Bare slug `interim = "device-count-wrap"` fails parse.
    let bad_slug = r#"schema = "frob.refalloc/1"
[[request]]
request = "bad-slug"
requested_by = "N21-REF"
defined_in = "§21.21"
condition = "c"
family = 6
interim = "device-count-wrap"
status = "requested"
"#;
    assert!(matches!(
        AllocationLedger::from_bytes(bad_slug.as_bytes()),
        Err(AllocationError::InvalidInterim { .. })
    ));

    // 4. Registered code `FROB-REF-602` with detail and `interim = "none"` both pass.
    let good = r#"schema = "frob.refalloc/1"
[[request]]
request = "ok-code"
requested_by = "N16-REF"
defined_in = "§16.12"
condition = "c1"
family = 6
interim = "FROB-REF-602"
interim_detail = "adjoint-store-over-budget"
status = "requested"

[[request]]
request = "ok-none"
requested_by = "N16-REF"
defined_in = "§16.12"
condition = "c2"
family = 6
interim = "none"
status = "requested"
"#;
    let lgood = AllocationLedger::from_bytes(good.as_bytes()).expect("good parses");
    let interims = validate_ledger_and_interims(&reg, &lgood).expect("good interims validate");
    assert_eq!(interims.len(), 2);
}

#[test]
fn retired_plan_time() {
    let ledger = AllocationLedger::load().expect("live ledger");
    let retired_map: BTreeMap<u16, &str> = ledger
        .retired
        .iter()
        .map(|r| (r.number, r.replaced_by.as_str()))
        .collect();
    assert_eq!(retired_map.get(&730).copied(), Some("FROB-REF-840"));
    assert_eq!(retired_map.get(&882).copied(), Some("FROB-REF-582"));

    let out = allocate_round(&AllocationInput {
        existing: vec![],
        reserved_blocks: vec![],
        retired: ledger.retired_numbers(),
        holds: BTreeSet::new(),
        allow_8xx_allocation: false,
        requests: vec![req(
            "proposes-730",
            "N37-REF",
            "§37.12",
            7,
            Some("73x"),
            Some(730),
            "retired proposal",
        )],
    })
    .expect("retired proposal gets another number");
    let assigned = out.by_slug("proposes-730").unwrap().assigned.as_deref();
    assert!(assigned.is_some());
    assert_ne!(assigned, Some("FROB-REF-730"));
}

#[test]
fn exit_rule_covers_all_decades() {
    let reg = RefusalRegistry::load().expect("live refusals.toml loads");
    assert_eq!(reg.exit_rule().expected_exit(701).unwrap().0, 4);
    assert_eq!(reg.exit_rule().expected_exit(720).unwrap().0, 5);
    assert_eq!(reg.exit_rule().expected_exit(731).unwrap().0, 5);
    assert_eq!(reg.exit_rule().expected_exit(740).unwrap().0, 4);
    assert_eq!(reg.exit_rule().expected_exit(761).unwrap().0, 4);
    assert_eq!(reg.exit_rule().expected_exit(790).unwrap().0, 5);
}

#[test]
fn exit_rule_761_is_4() {
    let reg = RefusalRegistry::load().expect("live refusals.toml loads");
    let row = reg
        .lookup("FROB-REF-761", "exit_rule_761_is_4")
        .expect("761 registered");
    assert_eq!(row.exit, 4);
}

#[test]
fn exit_rule_missing_decade_fails() {
    let bad = synthetic_registry(
        "conflict = []\nintegrity = []\ncapability = []",
        &reg_row("FROB-REF-771", 4, "§35.11", ""),
    );
    assert!(matches!(
        RefusalRegistry::from_bytes(bad.as_bytes()),
        Err(RefusalRegistryError::ExitMismatch { .. })
    ));
}

#[test]
fn exit_rule_exception_lists() {
    let rows = format!(
        "{}{}{}{}{}",
        reg_row("FROB-REF-112", 7, "§14.8", ""),
        reg_row("FROB-REF-804", 7, "§32.18", ""),
        reg_row("FROB-REF-810", 9, "§32.18", ""),
        reg_row("FROB-REF-820", 3, "§35.11", ""),
        reg_row("FROB-REF-840", 5, "§24.14", ""),
    );
    let exc = r#"conflict = ["FROB-REF-112", "FROB-REF-804"]
integrity = ["FROB-REF-810"]
capability = ["FROB-REF-840"]"#;
    let valid = synthetic_registry(exc, &rows);
    let reg = RefusalRegistry::from_bytes(valid.as_bytes()).expect("exception lists load");
    assert_eq!(reg.lookup("FROB-REF-112", "t").unwrap().exit, 7);
    assert_eq!(reg.lookup("FROB-REF-804", "t").unwrap().exit, 7);
    assert_eq!(reg.lookup("FROB-REF-810", "t").unwrap().exit, 9);
    assert_eq!(reg.lookup("FROB-REF-840", "t").unwrap().exit, 5);
    assert_eq!(reg.lookup("FROB-REF-820", "t").unwrap().exit, 3);

    // Planted copy of 112 with exit 3 fails.
    let bad_112 = valid.replacen("code = \"FROB-REF-112\"\nmeaning = \"Synthetic test refusal FROB-REF-112\"\ndefined_in = \"§14.8\"\nexit = 7", "code = \"FROB-REF-112\"\nmeaning = \"Synthetic test refusal FROB-REF-112\"\ndefined_in = \"§14.8\"\nexit = 3", 1);
    assert!(matches!(
        RefusalRegistry::from_bytes(bad_112.as_bytes()),
        Err(RefusalRegistryError::ExitMismatch { .. })
    ));

    // Planted copy of 840 with exit 3 fails.
    let bad_840 = valid.replacen("code = \"FROB-REF-840\"\nmeaning = \"Synthetic test refusal FROB-REF-840\"\ndefined_in = \"§24.14\"\nexit = 5", "code = \"FROB-REF-840\"\nmeaning = \"Synthetic test refusal FROB-REF-840\"\ndefined_in = \"§24.14\"\nexit = 3", 1);
    assert!(matches!(
        RefusalRegistry::from_bytes(bad_840.as_bytes()),
        Err(RefusalRegistryError::ExitMismatch { .. })
    ));

    // 840 planted in both conflict and capability fails load.
    let overlap = synthetic_registry(
        r#"conflict = ["FROB-REF-112", "FROB-REF-804", "FROB-REF-840"]
integrity = ["FROB-REF-810"]
capability = ["FROB-REF-840"]"#,
        &rows,
    );
    assert!(matches!(
        RefusalRegistry::from_bytes(overlap.as_bytes()),
        Err(RefusalRegistryError::OverlappingExceptionList { .. })
    ));

    // Absent code 899 in exception list fails load.
    let absent = synthetic_registry(
        r#"conflict = ["FROB-REF-112", "FROB-REF-804", "FROB-REF-899"]
integrity = ["FROB-REF-810"]
capability = ["FROB-REF-840"]"#,
        &rows,
    );
    assert!(matches!(
        RefusalRegistry::from_bytes(absent.as_bytes()),
        Err(RefusalRegistryError::ExceptionCodeMissing { .. })
    ));
}

#[test]
fn interim_detail_and_pairs() {
    let rows = format!(
        "{}{}{}{}",
        reg_row("FROB-REF-103", 3, "§14.8", ""),
        reg_row("FROB-REF-802", 3, "§32.18", ""),
        reg_row("FROB-REF-883", 3, "§28.4", ""),
        reg_row("FROB-REF-891", 3, "§34.9", ""),
    );
    let reg = RefusalRegistry::from_bytes(
        synthetic_registry("conflict = []\nintegrity = []\ncapability = []", &rows).as_bytes(),
    )
    .expect("reg loads");

    let good_toml = r#"schema = "frob.refalloc/1"
[[hold]]
family = 8
reason = "owner-decision"

[[request]]
request = "prediction-after-outcome"
requested_by = "N28cx-01"
defined_in = "§28.4"
condition = "prediction recorded after outcome"
family = 8
interim = "FROB-REF-883"
interim_detail = "registration-after-outcome"
interim_pointer = "/outcome"
status = "held"

[[request]]
request = "duplicate-member-name"
requested_by = "N35x-02"
defined_in = "§34.9"
condition = "duplicate JSON member name"
family = 8
interim = ["FROB-REF-802", "FROB-REF-103", "FROB-REF-891"]
interim_pointer = "/member"
status = "held"
"#;
    let ledger = AllocationLedger::from_bytes(good_toml.as_bytes()).expect("ledger parses");
    assert!(ledger.is_family_held(8));
    let interims = validate_ledger_and_interims(&reg, &ledger).expect("interims valid");
    assert_eq!(interims.len(), 2);
    let line0 = interims[0].format_line();
    assert!(line0.contains("FROB-REF-883") && line0.contains("detail=registration-after-outcome"));
    let line1 = interims[1].format_line();
    assert!(
        line1.contains("FROB-REF-802, FROB-REF-103, FROB-REF-891")
            && line1.contains("detail=duplicate-member-name")
    );

    // Second unsettled row sharing (FROB-REF-883, registration-after-outcome) fails.
    let dup_toml = format!(
        r#"{good_toml}
[[request]]
request = "colliding-interim"
requested_by = "N28cx-02"
defined_in = "§28.5"
condition = "another condition"
family = 8
interim = "FROB-REF-883"
interim_detail = "registration-after-outcome"
status = "held"
"#
    );
    let dup_ledger = AllocationLedger::from_bytes(dup_toml.as_bytes()).expect("parses");
    assert!(matches!(
        validate_ledger_and_interims(&reg, &dup_ledger),
        Err(AllocationError::DuplicateInterimPair { .. })
    ));
}

#[test]
fn neighbour_pairs_symmetric() {
    let reg = RefusalRegistry::load().expect("live reg");
    let ledger = AllocationLedger::load().expect("live ledger");
    let report =
        run_cross_chapter_checks(&reg, &ledger, None).expect("live cross-chapter checks pass");
    assert_eq!(report.printed_target, 294);

    // All 5 named pairs from N35-33 Background are present with distinct sites.
    for (l, r) in [
        ("FROB-REF-244", "FROB-REF-248"),
        ("FROB-REF-561", "FROB-REF-825"),
        ("FROB-REF-562", "FROB-REF-584"),
        ("FROB-REF-605", "FROB-REF-864"),
        ("FROB-REF-861", "FROB-REF-867"),
    ] {
        assert!(
            ledger
                .neighbour_pairs
                .iter()
                .any(|p| p.left == l && p.right == r),
            "missing named neighbour pair ({l}, {r})"
        );
    }

    // Planted asymmetric `related` between two present codes fails check 5.
    let asym_rows = format!(
        "{}{}",
        reg_row("FROB-REF-605", 6, "§21.21", r#"related = ["FROB-REF-864"]"#),
        reg_row("FROB-REF-864", 3, "§35.11", ""),
    );
    let asym_reg = RefusalRegistry::from_bytes(
        synthetic_registry("conflict = []\nintegrity = []\ncapability = []", &asym_rows).as_bytes(),
    )
    .expect("asym reg loads");
    let empty_ledger = AllocationLedger {
        schema: "frob.refalloc/1".into(),
        holds: vec![],
        retired: vec![],
        neighbour_pairs: vec![],
        requests: vec![],
    };
    assert!(matches!(
        run_cross_chapter_checks(&asym_reg, &empty_ledger, None),
        Err(AllocationError::NeighbourPairError { .. })
    ));
    let no_edges = RefusalRegistry::from_bytes(
        synthetic_registry(
            "conflict = []\nintegrity = []\ncapability = []",
            &(reg_row("FROB-REF-605", 6, "§21.21", "") + &reg_row("FROB-REF-864", 3, "§35.11", "")),
        )
        .as_bytes(),
    )
    .unwrap();
    let declared = AllocationLedger {
        neighbour_pairs: vec![
            ledger
                .neighbour_pairs
                .iter()
                .find(|p| p.left == "FROB-REF-605")
                .unwrap()
                .clone(),
        ],
        ..empty_ledger
    };
    assert!(matches!(
        run_cross_chapter_checks(&no_edges, &declared, None),
        Err(AllocationError::NeighbourPairError { .. })
    ));
}

#[test]
fn neighbour_pair_same_site_fails() {
    let reg = RefusalRegistry::load().expect("live reg");
    let mut ledger = AllocationLedger::load().expect("live ledger");
    ledger.neighbour_pairs.push(NeighbourPair {
        left: "FROB-REF-605".into(),
        right: "FROB-REF-864".into(),
        left_site: "Same raising site".into(),
        right_site: "same raising site".into(),
    });
    assert!(matches!(
        run_cross_chapter_checks(&reg, &ledger, None),
        Err(AllocationError::NeighbourPairError { .. })
    ));
}

#[test]
fn ledger_matches_registry() {
    // 1. The same assignment is pending before the gate and fatal at its owner's exit.
    let reg = RefusalRegistry::from_bytes(
        synthetic_registry(
            "conflict = []\nintegrity = []\ncapability = []",
            &reg_row("FROB-REF-802", 3, "§32.18", ""),
        )
        .as_bytes(),
    )
    .expect("reg");
    let mut missing_row_ledger = AllocationLedger {
        schema: "frob.refalloc/1".into(),
        holds: vec![],
        retired: vec![],
        neighbour_pairs: vec![],
        requests: vec![AllocationRequest {
            assigned: Some("FROB-REF-726".into()),
            status: AllocationStatus::Assigned,
            ..req("mech", "N32-REF", "§32.4", 7, Some("72x"), Some(726), "m")
        }],
    };
    let report = run_cross_chapter_checks(&reg, &missing_row_ledger, None).unwrap();
    assert_eq!(
        report.pending_assigned,
        [("FROB-REF-726".into(), "N32-REF".into())]
    );
    assert!(matches!(
        run_cross_chapter_checks(&reg, &missing_row_ledger, Some(Milestone::M0)),
        Err(AllocationError::LedgerRegistryMismatch { code, .. }) if code == "FROB-REF-726"
    ));

    // 2. Planted registry row beyond the printed 294 with no ledger row fails Check 4.
    missing_row_ledger.requests.clear();
    let extra_reg = RefusalRegistry::from_bytes(
        synthetic_registry(
            "conflict = []\nintegrity = []\ncapability = []",
            &reg_row("FROB-REF-995", 9, "§32.4", ""),
        )
        .as_bytes(),
    )
    .expect("extra reg");
    assert!(matches!(
        run_cross_chapter_checks(&extra_reg, &missing_row_ledger, None),
        Err(AllocationError::LedgerRegistryMismatch { code, .. }) if code == "FROB-REF-995"
    ));

    // 3. Planted retired code reused in registry fails Check 1.
    let retired_reg = RefusalRegistry::from_bytes(
        synthetic_registry(
            "conflict = []\nintegrity = []\ncapability = []",
            &reg_row("FROB-REF-730", 5, "§37.12", ""),
        )
        .as_bytes(),
    )
    .expect("retired reg");
    let retired_ledger = AllocationLedger {
        retired: AllocationLedger::load().expect("live ledger").retired,
        ..missing_row_ledger
    };
    assert!(matches!(
        run_cross_chapter_checks(&retired_reg, &retired_ledger, None),
        Err(AllocationError::RetiredCodeReused { code, context })
            if code == "FROB-REF-730" && context == "registries/refusals.toml"
    ));
}

#[test]
fn milestone_gate_checks_printed_membership() {
    let header = "conflict = []\nintegrity = []\ncapability = []";
    let empty = RefusalRegistry::from_bytes(synthetic_registry(header, "").as_bytes()).unwrap();
    let ledger = AllocationLedger::from_bytes(b"schema = \"frob.refalloc/1\"\n").unwrap();
    let rows = |omit| {
        let rows: String = printed_plan_codes()
            .iter()
            .filter(|&(&number, &chapter)| {
                number != omit && matches!(chapter, 12 | 13 | 16 | 21 | 32..=35 | 37)
            })
            .map(|(&number, &chapter)| {
                reg_row(
                    &format_refusal_code(number),
                    empty.exit_rule().expected_exit(number).unwrap().0,
                    &format!("§{chapter}"),
                    "",
                )
            })
            .collect();
        RefusalRegistry::from_bytes(synthetic_registry(header, &rows).as_bytes()).unwrap()
    };

    let clean = run_cross_chapter_checks(&rows(0), &ledger, Some(Milestone::M0)).unwrap();
    assert!(
        clean
            .pending_chapters
            .contains(&("N17-REF".into(), Milestone::M2, 7))
    );
    assert!(
        !clean
            .pending_chapters
            .iter()
            .any(|(bead, _, _)| bead == "N12-REF")
    );
    assert!(matches!(
        run_cross_chapter_checks(&rows(168), &ledger, Some(Milestone::M0)),
        Err(AllocationError::MilestoneCountShortfall { bead, detail })
            if bead == "N12-REF" && detail.contains("FROB-REF-168")
    ));
}

#[test]
fn printout_is_generated() {
    let reg = RefusalRegistry::load().expect("live refusals.toml");
    let ledger = AllocationLedger::load().expect("live refusal_allocations.toml");
    let run1 = render_refusals_markdown(&reg, &ledger);
    let run2 = render_refusals_markdown(&reg, &ledger);
    assert_eq!(run1, run2, "printout must be byte-stable across runs");
    assert_eq!(
        EMBEDDED_REFUSALS_MD, run1,
        "docs/generated/refusals.md must match render_refusals_markdown byte-for-byte"
    );
    assert!(
        run1.contains("over capacity") && run1.contains("FROB-REF-8100"),
        "capacity table must mark 8xx over capacity and name FROB-REF-8100 (ADR-NUM-refusal-8xx)"
    );
    let _ = (
        EMBEDDED_REFUSALS_TOML,
        EMBEDDED_REFUSAL_ALLOCATIONS_TOML,
        format_refusal_code(8100),
    );
    let _ = HoldEntry {
        family: 8,
        reason: String::new(),
        ruling: None,
    };
}

#[test]
fn printout_capacity_counts_each_occupied_three_digit_code_once() {
    let rows = reg_row("FROB-REF-837", 3, "§35.11", "")
        + &reg_row("FROB-REF-838", 3, "§35.11", "")
        + &reg_row("FROB-REF-8100", 3, "§35.11", "");
    let reg = RefusalRegistry::from_bytes(
        synthetic_registry(
            "\"81xx\" = 3\nconflict = []\nintegrity = []\ncapability = []",
            &rows,
        )
        .as_bytes(),
    )
    .unwrap();
    let mut ledger = AllocationLedger::from_bytes(
        br#"schema = "frob.refalloc/1"
[[retired]]
code = "FROB-REF-801"
replaced_by = "FROB-REF-838"
reason = "Printed and subsequently retired"
[[retired]]
code = "FROB-REF-882"
replaced_by = "FROB-REF-582"
reason = "Retired before Appendix O"
"#,
    )
    .unwrap();
    ledger.requests = [836, 837, 8100]
        .into_iter()
        .map(|code| AllocationRequest {
            status: AllocationStatus::Assigned,
            assigned: Some(format_refusal_code(code)),
            ..req(
                &format!("request-{code}"),
                "N35-REF",
                "§35.11",
                8,
                None,
                None,
                "example",
            )
        })
        .collect();

    let rendered = render_refusals_markdown(&reg, &ledger);
    let family = rendered
        .lines()
        .find(|line| line.starts_with("| `8xx`"))
        .unwrap();
    // 801 is printed and retired; 837 is assigned and live. The four-digit code
    // consumes no three-digit slot. Of the four 8xx vacancies only 839 remains.
    assert!(
        family.contains("over capacity (1 free 3-digit;"),
        "{family}"
    );
}

#[test]
fn adr_num_refusal_8xx_four_digit_and_whole_token() {
    // 1. FROB-REF-8100 loads with exit 3 under [exit_rule] "81xx" = 3.
    let four_digit = synthetic_registry(
        "\"81xx\" = 3\nconflict = []\nintegrity = []\ncapability = []",
        &format!(
            "{}{}",
            reg_row("FROB-REF-810", 3, "§32.18", ""),
            reg_row("FROB-REF-8100", 3, "§35.11", ""),
        ),
    );
    let reg = RefusalRegistry::from_bytes(four_digit.as_bytes()).expect("8100 loads with exit 3");
    assert_eq!(reg.lookup("FROB-REF-8100", "t").unwrap().exit, 3);

    // 2. Planted 9xx row with exit 3 fails load (option C excluded by ADR-NUM-refusal-8xx).
    let bad_9xx = synthetic_registry(
        "conflict = []\nintegrity = []\ncapability = []",
        &reg_row("FROB-REF-901", 3, "§35.11", ""),
    );
    assert!(matches!(
        RefusalRegistry::from_bytes(bad_9xx.as_bytes()),
        Err(RefusalRegistryError::ExitMismatch { .. })
    ));

    // 3. Whole-token comparison: FROB-REF-810 must NOT match FROB-REF-8100.
    assert!(!refusal_code_token_eq("FROB-REF-810", "FROB-REF-8100"));
    assert!(refusal_code_token_eq("FROB-REF-8100", "FROB-REF-8100"));
    assert!(!text_contains_refusal_token(
        "refused with FROB-REF-8100.",
        "FROB-REF-810"
    ));
    assert!(text_contains_refusal_token(
        "refused with FROB-REF-8100.",
        "FROB-REF-8100"
    ));
    assert!(text_contains_refusal_token(
        "refused with FROB-REF-810.",
        "FROB-REF-810"
    ));
}
