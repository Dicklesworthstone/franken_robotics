#![forbid(unsafe_code)]

//! Airlock feature-group, `BUILT_WITH`, `FROB-REF-790`, stub, `conformance`
//! isolation, and public-API hygiene tests (`N33a-11`).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use frob_core::refusal::{RefusalCode, RetryPolicy};
use frob_core::registry::refusals::RefusalRegistry;
use frob_core::registry::toml::{self, Table, Value};
use frob_fsim::{
    BUILT_WITH, EXIT_CLASS_GROUP_ABSENT, GROUP_REGISTRY, Group, GroupAbsent,
    REFUSAL_CODE_GROUP_ABSENT, built_with_names, group_descriptor, require,
};

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn workspace_root() -> PathBuf {
    crate_dir()
        .parent()
        .and_then(Path::parent)
        .expect("crates/frob-fsim must sit two levels below workspace root")
        .to_path_buf()
}

fn parse_toml_file(path: &Path) -> Table {
    let bytes = fs::read(path).unwrap_or_else(|e| panic!("failed reading {}: {e}", path.display()));
    toml::parse(&bytes).unwrap_or_else(|e| panic!("failed parsing {}: {e}", path.display()))
}

fn str_list(tbl: &Table, key: &str) -> Vec<String> {
    tbl.get(key)
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("missing array `{key}`"))
        .iter()
        .map(|sp| {
            sp.value
                .as_str()
                .unwrap_or_else(|| panic!("non-string entry in `{key}`"))
                .to_owned()
        })
        .collect()
}

#[derive(Debug, Default)]
struct CargoDepSpec {
    optional: bool,
    features: Vec<String>,
}

#[derive(Debug, Default)]
struct ParsedCargoManifest {
    package_name: String,
    normal_frob_fsim: Option<CargoDepSpec>,
    any_frob_fsim_features: BTreeSet<String>,
    features: BTreeMap<String, Vec<String>>,
}

fn strip_toml_comment(line: &str) -> &str {
    let (mut in_dq, mut in_sq, mut esc) = (false, false, false);
    for (i, b) in line.bytes().enumerate() {
        if esc {
            esc = false;
            continue;
        }
        if in_dq {
            if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_dq = false;
            }
        } else if in_sq {
            if b == b'\'' {
                in_sq = false;
            }
        } else if b == b'"' {
            in_dq = true;
        } else if b == b'\'' {
            in_sq = true;
        } else if b == b'#' {
            return &line[..i];
        }
    }
    line
}

fn parse_string_array(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = raw.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'"' || bytes[i] == b'\'' {
            let q = bytes[i];
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != q {
                if bytes[i] == b'\\' && q == b'"' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                i += 1;
            }
            out.push(raw[start..i].to_owned());
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        i += 1;
    }
    out
}

fn parse_inline_dep_spec(val: &str) -> CargoDepSpec {
    let mut spec = CargoDepSpec::default();
    let inner = val
        .trim()
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .unwrap_or(val);
    if let Some(pos) = inner.find("features")
        && let Some(eq) = inner[pos..].find('=')
        && let Some(lb) = inner[pos + eq..].find('[')
        && let Some(rb) = inner[pos + eq + lb..].find(']')
    {
        spec.features = parse_string_array(&inner[pos + eq + lb..=pos + eq + lb + rb]);
    }
    if let Some(pos) = inner.find("optional")
        && let Some(eq) = inner[pos..].find('=')
    {
        let after = inner[pos + eq + 1..].trim_start();
        if after.starts_with("true") {
            spec.optional = true;
        }
    }
    spec
}

/// Minimal Cargo manifest reader that tolerates Cargo's full TOML syntax
/// (such as `version.workspace = true` dotted keys) while extracting `[package].name`,
/// `frob-fsim` dependency feature lists, and `[features]` tables.
fn parse_cargo_manifest(path: &Path) -> ParsedCargoManifest {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed reading {}: {e}", path.display()));
    let mut out = ParsedCargoManifest::default();
    let mut section = String::new();
    let mut open_feature: Option<(String, String)> = None;

    let flush_feature =
        |feats: &mut BTreeMap<String, Vec<String>>, open: &mut Option<(String, String)>| {
            if let Some((k, buf)) = open.take() {
                feats.insert(k, parse_string_array(&buf));
            }
        };

    for raw_line in content.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if let Some((_, ref mut buf)) = open_feature {
            buf.push(' ');
            buf.push_str(line);
            if line.contains(']') {
                flush_feature(&mut out.features, &mut open_feature);
            }
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line.to_owned();
            continue;
        }
        let Some((k_raw, v_raw)) = line.split_once('=') else {
            continue;
        };
        let key = k_raw.trim().trim_matches('"').trim_matches('\'');
        let val = v_raw.trim();

        match section.as_str() {
            "[package]" => {
                if key == "name" {
                    out.package_name = val.trim_matches('"').trim_matches('\'').to_owned();
                }
            }
            "[features]" => {
                if val.contains('[') && !val.contains(']') {
                    open_feature = Some((key.to_owned(), val.to_owned()));
                } else {
                    out.features.insert(key.to_owned(), parse_string_array(val));
                }
            }
            "[dependencies]" | "[dev-dependencies]" | "[build-dependencies]" => {
                if key == "frob-fsim"
                    || (val.contains("package") && val.contains("\"frob-fsim\""))
                {
                    let spec = parse_inline_dep_spec(val);
                    for f in &spec.features {
                        out.any_frob_fsim_features.insert(f.clone());
                    }
                    if section == "[dependencies]" {
                        out.normal_frob_fsim = Some(spec);
                    }
                }
            }
            "[dependencies.frob-fsim]"
            | "[dev-dependencies.frob-fsim]"
            | "[build-dependencies.frob-fsim]" => {
                let is_normal = section == "[dependencies.frob-fsim]";
                if key == "features" {
                    let fs = parse_string_array(val);
                    for f in &fs {
                        out.any_frob_fsim_features.insert(f.clone());
                    }
                    if is_normal {
                        out.normal_frob_fsim
                            .get_or_insert_with(CargoDepSpec::default)
                            .features
                            .extend(fs);
                    }
                } else if key == "optional" && is_normal {
                    out.normal_frob_fsim
                        .get_or_insert_with(CargoDepSpec::default)
                        .optional = val == "true";
                }
            }
            _ => {
                if section.ends_with(".dependencies]")
                    && (key == "frob-fsim"
                        || (val.contains("package") && val.contains("\"frob-fsim\"")))
                {
                    let spec = parse_inline_dep_spec(val);
                    for f in &spec.features {
                        out.any_frob_fsim_features.insert(f.clone());
                    }
                }
            }
        }
    }
    flush_feature(&mut out.features, &mut open_feature);
    out
}

fn manifest_enables_frob_fsim_feature(
    manifest: &ParsedCargoManifest,
    target_feature: &str,
) -> bool {
    if manifest.any_frob_fsim_features.contains(target_feature) {
        return true;
    }
    let needle_direct = format!("frob-fsim/{target_feature}");
    let needle_weak = format!("frob-fsim?/{target_feature}");
    manifest.features.values().any(|items| {
        items
            .iter()
            .any(|x| x == &needle_direct || x == &needle_weak)
    })
}

fn manifest_enables_feature_in_default(
    manifest: &ParsedCargoManifest,
    target_feature: &str,
) -> bool {
    if let Some(spec) = &manifest.normal_frob_fsim
        && !spec.optional
        && spec.features.iter().any(|f| f == target_feature)
    {
        return true;
    }

    let needle_direct = format!("frob-fsim/{target_feature}");
    let needle_weak = format!("frob-fsim?/{target_feature}");
    let mut visited = BTreeSet::new();
    let mut stack: Vec<String> = manifest
        .features
        .get("default")
        .cloned()
        .unwrap_or_default();
    while let Some(item) = stack.pop() {
        if item == needle_direct
            || item == needle_weak
            || (manifest.package_name == "frob-fsim" && item == target_feature)
        {
            return true;
        }
        if visited.insert(item.clone())
            && let Some(sub) = manifest.features.get(&item)
        {
            stack.extend(sub.iter().cloned());
        }
    }
    false
}

#[test]
#[cfg(all(
    feature = "referee",
    feature = "geometry",
    feature = "ledger",
    not(feature = "optimize"),
    not(feature = "stats"),
    not(feature = "spectral"),
    not(feature = "actuation"),
    not(feature = "structure"),
    not(feature = "multiphysics")
))]
fn built_with_default_is_three() {
    assert_eq!(
        BUILT_WITH,
        &[Group::Referee, Group::Geometry, Group::Ledger],
        "default build must compile exactly [Referee, Geometry, Ledger]"
    );
}

#[test]
#[cfg(all(
    feature = "mode-inproc",
    not(feature = "referee"),
    not(feature = "geometry"),
    not(feature = "ledger"),
    not(feature = "optimize"),
    not(feature = "stats"),
    not(feature = "spectral"),
    not(feature = "actuation"),
    not(feature = "structure"),
    not(feature = "multiphysics")
))]
fn built_with_none_is_empty() {
    assert!(
        BUILT_WITH.is_empty(),
        "--no-default-features --features mode-inproc must have empty BUILT_WITH, got {BUILT_WITH:?}"
    );
}

#[test]
fn built_with_matches_enabled_features() {
    let mut expected = Vec::new();
    if cfg!(feature = "referee") {
        expected.push(Group::Referee);
    }
    if cfg!(feature = "geometry") {
        expected.push(Group::Geometry);
    }
    if cfg!(feature = "ledger") {
        expected.push(Group::Ledger);
    }
    if cfg!(feature = "optimize") {
        expected.push(Group::Optimize);
    }
    if cfg!(feature = "stats") {
        expected.push(Group::Stats);
    }
    if cfg!(feature = "spectral") {
        expected.push(Group::Spectral);
    }
    if cfg!(feature = "actuation") {
        expected.push(Group::Actuation);
    }
    if cfg!(feature = "structure") {
        expected.push(Group::Structure);
    }
    if cfg!(feature = "multiphysics") {
        expected.push(Group::Multiphysics);
    }
    assert_eq!(
        BUILT_WITH,
        expected.as_slice(),
        "BUILT_WITH must equal the exact enabled runtime feature groups"
    );
    for &g in Group::ALL {
        assert_eq!(g.is_compiled(), expected.contains(&g));
    }
}

#[test]
fn every_group_feature_implies_inproc() {
    let manifest = parse_toml_file(&crate_dir().join("Cargo.toml"));
    let features = manifest
        .get("features")
        .and_then(Value::as_table)
        .expect("Cargo.toml must declare [features]");

    let default_items = str_list(features, "default");
    assert_eq!(
        default_items,
        vec!["referee", "geometry", "ledger", "mode-inproc"],
        "default feature set must be [referee, geometry, ledger, mode-inproc]"
    );

    let mode_remote_items = str_list(features, "mode-remote");
    let all_group_names: BTreeSet<&str> = GROUP_REGISTRY.iter().map(|d| d.name).collect();
    for item in &mode_remote_items {
        assert!(
            !all_group_names.contains(item.as_str()),
            "mode-remote must enable no group feature, found `{item}`"
        );
    }

    for desc in GROUP_REGISTRY {
        let items = str_list(features, desc.name);
        assert!(
            items.iter().any(|x| x == "mode-inproc"),
            "group feature `{}` must imply `mode-inproc`, got {items:?}",
            desc.name
        );
        for &krate in desc.crates {
            let expected_dep = format!("dep:{krate}");
            assert!(
                items.iter().any(|x| x == &expected_dep),
                "group feature `{}` must enable `{expected_dep}`, got {items:?}",
                desc.name
            );
        }
    }
}

#[test]
fn require_present_ok() {
    assert!(require(&[]).is_ok());
    assert!(require(BUILT_WITH).is_ok());
    for &group in BUILT_WITH {
        assert!(
            require(&[group]).is_ok(),
            "require(&[{group:?}]) must succeed when group is compiled in"
        );
    }
}

#[test]
fn require_absent_refuses_790() {
    let reg = RefusalRegistry::load().expect("registries/refusals.toml must load cleanly");
    assert_eq!(
        reg.exit_for(REFUSAL_CODE_GROUP_ABSENT),
        Ok(EXIT_CLASS_GROUP_ABSENT),
        "FROB-REF-790 must map to exit class 5 in registries/refusals.toml"
    );

    let missing = if !Group::Stats.is_compiled() {
        Group::Stats
    } else if let Some(&g) = Group::ALL.iter().find(|g| !g.is_compiled()) {
        g
    } else {
        // Every group enabled (--all-features): still verify GroupAbsent::into_refusal directly.
        let r = GroupAbsent::new(Group::Stats).into_refusal();
        assert_eq!(r.code, RefusalCode::FrobRef790);
        assert_eq!(r.pointer, "foundation/stats/compiled");
        assert_eq!(r.required, "stats");
        assert_eq!(r.observed, built_with_names());
        assert_eq!(r.retry, RetryPolicy::AfterChange);
        return;
    };

    let err = require(&[missing]).expect_err("require on absent group must refuse with FROB-REF-790");
    assert_eq!(err.code, RefusalCode::FrobRef790);
    assert_eq!(err.pointer, format!("foundation/{}/compiled", missing.as_str()));
    assert_eq!(err.required, missing.as_str());
    assert_eq!(err.observed, built_with_names());
    assert_eq!(err.retry, RetryPolicy::AfterChange);
    assert_eq!(EXIT_CLASS_GROUP_ABSENT, 5);
    assert_eq!(err, GroupAbsent::new(missing).into_refusal());
}

#[test]
fn group_absent_stub_returns_typed_error() {
    #[cfg(not(feature = "stats"))]
    {
        assert_eq!(
            frob_fsim::stats::available(),
            Err(GroupAbsent { group: Group::Stats })
        );
        assert_eq!(
            frob_fsim::stats::new_uncertainty(),
            Err(GroupAbsent { group: Group::Stats })
        );
        assert_eq!(
            frob_fsim::stats::new_value_of_information(),
            Err(GroupAbsent { group: Group::Stats })
        );
        assert_eq!(
            frob_fsim::stats::new_surrogate(),
            Err(GroupAbsent { group: Group::Stats })
        );
        assert_eq!(
            frob_fsim::stats::new_assimilation(),
            Err(GroupAbsent { group: Group::Stats })
        );
    }

    #[cfg(not(feature = "spectral"))]
    {
        assert_eq!(
            frob_fsim::spectral::new_spectral(),
            Err(GroupAbsent { group: Group::Spectral })
        );
    }

    #[cfg(not(feature = "actuation"))]
    {
        assert_eq!(
            frob_fsim::actuation::new_magnetics(),
            Err(GroupAbsent { group: Group::Actuation })
        );
        assert_eq!(
            frob_fsim::actuation::new_thermal_network(),
            Err(GroupAbsent { group: Group::Actuation })
        );
        assert_eq!(
            frob_fsim::actuation::new_current_loop(),
            Err(GroupAbsent { group: Group::Actuation })
        );
        assert_eq!(
            frob_fsim::actuation::new_port_hamiltonian(),
            Err(GroupAbsent { group: Group::Actuation })
        );
    }

    #[cfg(not(feature = "structure"))]
    {
        assert_eq!(
            frob_fsim::structure::new_structural_opt(),
            Err(GroupAbsent { group: Group::Structure })
        );
        assert_eq!(
            frob_fsim::structure::new_tolerance_allocation(),
            Err(GroupAbsent { group: Group::Structure })
        );
        assert_eq!(
            frob_fsim::structure::new_as_built(),
            Err(GroupAbsent { group: Group::Structure })
        );
        assert_eq!(
            frob_fsim::structure::new_fabrication_rules(),
            Err(GroupAbsent { group: Group::Structure })
        );
    }

    #[cfg(not(feature = "multiphysics"))]
    {
        assert_eq!(
            frob_fsim::multiphysics::new_fluid_island(),
            Err(GroupAbsent { group: Group::Multiphysics })
        );
        assert_eq!(
            frob_fsim::multiphysics::new_coupled_island(),
            Err(GroupAbsent { group: Group::Multiphysics })
        );
    }

    #[cfg(not(feature = "optimize"))]
    {
        assert_eq!(
            frob_fsim::optimize::new_optimizer(),
            Err(GroupAbsent { group: Group::Optimize })
        );
        assert_eq!(
            frob_fsim::optimize::new_archive(),
            Err(GroupAbsent { group: Group::Optimize })
        );
    }

    #[cfg(not(any(feature = "optimize", feature = "optimize-race")))]
    {
        assert_eq!(
            frob_fsim::optimize::new_racing(),
            Err(GroupAbsent { group: Group::Optimize })
        );
    }

    #[cfg(not(feature = "referee"))]
    {
        assert_eq!(
            frob_fsim::referee::new_referee_dynamics(),
            Err(GroupAbsent { group: Group::Referee })
        );
        assert_eq!(
            frob_fsim::referee::new_referee_contact(),
            Err(GroupAbsent { group: Group::Referee })
        );
        assert_eq!(
            frob_fsim::referee::new_integrator(),
            Err(GroupAbsent { group: Group::Referee })
        );
        assert_eq!(
            frob_fsim::referee::new_port_system(),
            Err(GroupAbsent { group: Group::Referee })
        );
    }

    #[cfg(not(feature = "geometry"))]
    {
        assert_eq!(
            frob_fsim::geometry::new_geometry_kernel(),
            Err(GroupAbsent { group: Group::Geometry })
        );
        assert_eq!(
            frob_fsim::geometry::new_scene_query(),
            Err(GroupAbsent { group: Group::Geometry })
        );
        assert_eq!(
            frob_fsim::geometry::new_mesh_exchange(),
            Err(GroupAbsent { group: Group::Geometry })
        );
    }

    #[cfg(not(feature = "ledger"))]
    {
        assert_eq!(
            frob_fsim::ledger::new_ledger(),
            Err(GroupAbsent { group: Group::Ledger })
        );
        assert_eq!(
            frob_fsim::ledger::new_session(),
            Err(GroupAbsent { group: Group::Ledger })
        );
        assert_eq!(
            frob_fsim::ledger::new_recompute(),
            Err(GroupAbsent { group: Group::Ledger })
        );
    }

    // Verify no group stub module source names any `fs_*` / `fs-*` FrankenSim crate.
    for group in Group::ALL {
        let mod_path = crate_dir().join("src").join(group.as_str()).join("mod.rs");
        let src = fs::read_to_string(&mod_path).expect("group mod.rs must exist");
        for desc in GROUP_REGISTRY {
            for &krate in desc.crates {
                let rust_ident = krate.replace('-', "_");
                assert!(
                    !src.contains(&format!("{rust_ident}::")) && !src.contains(krate),
                    "stub/skeleton `{}` must not name FrankenSim crate `{krate}`",
                    mod_path.display()
                );
            }
        }
    }
}

#[test]
fn require_reports_first_absent_in_declared_order() {
    let absent: Vec<Group> = Group::ALL
        .iter()
        .copied()
        .filter(|g| !g.is_compiled())
        .collect();
    if absent.len() >= 2 {
        let first = absent[0];
        let second = absent[1];
        let err_ab = require(&[first, second]).expect_err("expected refusal");
        assert_eq!(err_ab.code, RefusalCode::FrobRef790);
        assert_eq!(err_ab.required, first.as_str());
        assert_eq!(err_ab.pointer, first.compiled_pointer());

        let err_ba = require(&[second, first]).expect_err("expected refusal");
        assert_eq!(err_ba.code, RefusalCode::FrobRef790);
        assert_eq!(err_ba.required, second.as_str());
        assert_eq!(err_ba.pointer, second.compiled_pointer());
    }

    #[cfg(all(not(feature = "stats"), not(feature = "spectral")))]
    {
        let err = require(&[Group::Stats, Group::Spectral]).expect_err("expected refusal");
        assert_eq!(err.code, RefusalCode::FrobRef790);
        assert_eq!(err.required, "stats");
        assert_eq!(err.pointer, "foundation/stats/compiled");
    }
}

#[test]
fn group_records_match_manifest() {
    let manifest = parse_toml_file(&crate_dir().join("Cargo.toml"));
    let contract = parse_toml_file(&crate_dir().join("contract.toml"));

    let m_feats = manifest
        .get("features")
        .and_then(Value::as_table)
        .expect("Cargo.toml must have [features]");
    let c_feats = contract
        .get("features")
        .and_then(Value::as_table)
        .expect("contract.toml must have [features]");

    let non_group: BTreeSet<&str> = ["default", "mode-inproc", "mode-remote", "conformance"]
        .into_iter()
        .collect();
    let manifest_group_keys: BTreeSet<String> = m_feats
        .iter()
        .map(|(k, _)| k.clone())
        .filter(|k| !non_group.contains(k.as_str()))
        .collect();
    let contract_group_keys: BTreeSet<String> = c_feats.iter().map(|(k, _)| k.clone()).collect();
    let registry_keys: BTreeSet<String> =
        GROUP_REGISTRY.iter().map(|d| d.name.to_owned()).collect();

    assert_eq!(
        manifest_group_keys, contract_group_keys,
        "Cargo.toml group features (less mode/conformance) must equal contract.toml [features]"
    );
    assert_eq!(
        contract_group_keys, registry_keys,
        "contract.toml [features] must equal GROUP_REGISTRY"
    );

    // Ten primary groups (excluding sub-feature `optimize-race`).
    let primary_count = GROUP_REGISTRY
        .iter()
        .filter(|d| d.parent_group.is_none())
        .count();
    assert_eq!(primary_count, 10, "expected ten primary airlock groups");

    // Cross-check every record's fields between contract.toml and GROUP_REGISTRY.
    for desc in GROUP_REGISTRY {
        let rec = c_feats
            .get(desc.name)
            .and_then(Value::as_table)
            .unwrap_or_else(|| panic!("contract.toml missing [features.{}]", desc.name));
        assert_eq!(
            rec.get("default").and_then(Value::as_bool),
            Some(desc.default_on),
            "default mismatch for {}",
            desc.name
        );
        assert_eq!(
            rec.get("dev_only").and_then(Value::as_bool),
            Some(desc.dev_only),
            "dev_only mismatch for {}",
            desc.name
        );
        assert_eq!(
            str_list(rec, "crates"),
            desc.crates.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
            "crates mismatch for {}",
            desc.name
        );
        assert_eq!(
            str_list(rec, "traits"),
            desc.traits.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
            "traits mismatch for {}",
            desc.name
        );
        assert_eq!(
            str_list(rec, "consumers"),
            desc.consumers
                .iter()
                .map(|s| (*s).to_owned())
                .collect::<Vec<_>>(),
            "consumers mismatch for {}",
            desc.name
        );
        assert_eq!(
            str_list(rec, "test_consumers"),
            desc.test_consumers
                .iter()
                .map(|s| (*s).to_owned())
                .collect::<Vec<_>>(),
            "test_consumers mismatch for {}",
            desc.name
        );
        assert_eq!(
            rec.get("gate").and_then(Value::as_str),
            Some(desc.gate),
            "gate mismatch for {}",
            desc.name
        );
    }
}

#[test]
fn conformance_feature_is_not_a_group() {
    let manifest = parse_toml_file(&crate_dir().join("Cargo.toml"));
    let features = manifest
        .get("features")
        .and_then(Value::as_table)
        .expect("Cargo.toml [features]");

    assert_eq!(
        str_list(features, "conformance"),
        vec!["mode-inproc"],
        "`conformance` feature must list only `mode-inproc`"
    );
    assert!(
        !str_list(features, "default").iter().any(|f| f == "conformance"),
        "`conformance` must be off by default"
    );
    assert!(
        group_descriptor("conformance").is_none(),
        "`conformance` must be absent from GROUP_REGISTRY"
    );
    assert!(
        Group::parse("conformance").is_none(),
        "`conformance` must be absent from Group"
    );

    let contract = parse_toml_file(&crate_dir().join("contract.toml"));
    let c_feats = contract
        .get("features")
        .and_then(Value::as_table)
        .expect("contract.toml [features]");
    for non_group in ["conformance", "mode-inproc", "mode-remote"] {
        assert!(
            c_feats.get(non_group).is_none(),
            "non-group feature `{non_group}` must be absent from contract.toml [features]"
        );
    }

    // Verify `pub mod conformance;` in src/lib.rs is gated on `#[cfg(any(test, feature = "conformance"))]`.
    let lib_src = fs::read_to_string(crate_dir().join("src/lib.rs")).expect("read src/lib.rs");
    let lines: Vec<&str> = lib_src.lines().collect();
    let conf_idx = lines
        .iter()
        .position(|l| l.trim() == "pub mod conformance;")
        .expect("src/lib.rs must declare `pub mod conformance;`");
    assert!(
        conf_idx > 0
            && lines[conf_idx - 1].trim() == "#[cfg(any(test, feature = \"conformance\"))]",
        "pub mod conformance must be gated by #[cfg(any(test, feature = \"conformance\"))]"
    );

    // When `conformance` is enabled with default groups, BUILT_WITH still matches default groups only.
    #[cfg(all(
        feature = "conformance",
        feature = "referee",
        feature = "geometry",
        feature = "ledger",
        not(feature = "optimize"),
        not(feature = "stats"),
        not(feature = "spectral"),
        not(feature = "actuation"),
        not(feature = "structure"),
        not(feature = "multiphysics")
    ))]
    assert_eq!(
        BUILT_WITH,
        &[Group::Referee, Group::Geometry, Group::Ledger],
        "`conformance` must not add any entry to BUILT_WITH"
    );
}

#[test]
fn conformance_enabled_only_by_frob_conformance() {
    let root = workspace_root();
    let crates_dir = root.join("crates");
    for entry in fs::read_dir(&crates_dir).expect("read crates/") {
        let entry = entry.expect("dir entry");
        let manifest_path = entry.path().join("Cargo.toml");
        if !manifest_path.is_file() {
            continue;
        }
        let manifest = parse_cargo_manifest(&manifest_path);
        let pkg = manifest.package_name.as_str();
        if pkg != "frob-conformance" {
            assert!(
                !manifest_enables_frob_fsim_feature(&manifest, "conformance"),
                "workspace crate `{pkg}` ({}) must not enable `frob-fsim/conformance`",
                manifest_path.display()
            );
        }
    }

    // Negative control fixture: planted `frob-verbs` manifest enabling `frob-fsim/conformance` is caught.
    let planted_path = crate_dir().join("tests/fixtures/frob_verbs_enables_conformance/Cargo.toml");
    let planted = parse_cargo_manifest(&planted_path);
    assert!(
        manifest_enables_frob_fsim_feature(&planted, "conformance"),
        "planted fixture `{}` enabling frob-fsim/conformance must be reported",
        planted_path.display()
    );
}

#[test]
fn public_api_names_no_volatile_type() {
    // Collect every volatile crate Rust identifier across all airlock groups.
    let volatile_idents: Vec<String> = GROUP_REGISTRY
        .iter()
        .flat_map(|d| d.crates.iter().copied())
        .map(|c| c.replace('-', "_"))
        .collect();

    let mut rs_files = Vec::new();
    fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).expect("read src dir") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                collect(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }
    collect(&crate_dir().join("src"), &mut rs_files);
    rs_files.sort();

    for path in rs_files {
        let src = fs::read_to_string(&path).expect("read rs");
        let mut in_pub_sig = false;
        let mut sig_buf = String::new();
        for (idx, line) in src.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            if trimmed.starts_with("pub ") || trimmed.starts_with("pub(") {
                in_pub_sig = true;
                sig_buf.clear();
            }
            if in_pub_sig {
                let head = match trimmed.find('{') {
                    Some(brace) => {
                        in_pub_sig = false;
                        &trimmed[..brace]
                    }
                    None => {
                        if trimmed.ends_with(';') {
                            in_pub_sig = false;
                        }
                        trimmed
                    }
                };
                sig_buf.push(' ');
                sig_buf.push_str(head);

                if !in_pub_sig {
                    for ident in &volatile_idents {
                        let path_prefix = format!("{ident}::");
                        assert!(
                            !sig_buf.contains(&path_prefix),
                            "public signature at {}:{} names volatile crate `{ident}`: `{sig_buf}`",
                            path.display(),
                            idx + 1
                        );
                        assert!(
                            !sig_buf.contains(&format!("pub use {ident}"))
                                && !sig_buf.contains(&format!("pub extern crate {ident}")),
                            "public re-export of `{ident}` at {}:{}",
                            path.display(),
                            idx + 1
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn multiphysics_not_in_any_default() {
    let root = workspace_root();
    let crates_dir = root.join("crates");
    for entry in fs::read_dir(&crates_dir).expect("read crates/") {
        let entry = entry.expect("entry");
        let manifest_path = entry.path().join("Cargo.toml");
        if !manifest_path.is_file() {
            continue;
        }
        let manifest = parse_cargo_manifest(&manifest_path);
        assert!(
            !manifest_enables_feature_in_default(&manifest, "multiphysics"),
            "manifest `{}` must not enable `multiphysics` in default features",
            manifest_path.display()
        );
    }
}

#[test]
fn planted_frob_dyn_structure_fixture_exists() {
    // Shipped for N33c-29's `airlock_fixture_frob_dyn_structure_refused` test (Review Note 3).
    let path = crate_dir().join("tests/fixtures/frob_dyn_enables_structure/Cargo.toml");
    let manifest = parse_cargo_manifest(&path);
    assert_eq!(manifest.package_name, "frob-dyn");
    assert!(
        manifest_enables_frob_fsim_feature(&manifest, "structure"),
        "planted fixture must enable `frob-fsim` feature `structure`"
    );
}
