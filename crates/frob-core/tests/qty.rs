#![deny(unsafe_code)]

//! Closed SI quantity surface tests (`N33c-03`, `KA-779`, `CX-354`, `OPEN-SUITE-4`).

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use frob_core::qty::{
    ALIAS_COUNT, ALIAS_NAMES, Checked, Length, MAX_ALIASES, QuantityAlias, Velocity,
};
use frob_core::registry::toml::{self, Value};

const ALIASES_TOML: &str = include_str!("../../../registries/qty_aliases.toml");
const PRODUCTS_TOML: &str = include_str!("../../../registries/qty_products.toml");
const QTY_RS: &str = include_str!("../src/qty.rs");
const QTY_GEN_RS: &str = include_str!("../src/qty_gen.rs");

#[derive(Debug, Clone)]
struct AliasRow {
    name: String,
    dim: [i8; 6],
    kind: String,
}

#[derive(Debug, Clone)]
struct ProductRow {
    lhs: String,
    op: String,
    rhs: String,
    out: String,
}

fn parse_aliases(src: &str) -> Vec<AliasRow> {
    let root = toml::parse(src.as_bytes()).expect("qty_aliases.toml parses");
    assert_eq!(
        root.get("schema").and_then(Value::as_str),
        Some("frob.qty_aliases/1")
    );
    let arr = root
        .get("alias")
        .and_then(Value::as_array)
        .expect("[[alias]]");
    arr.iter()
        .map(|sp| {
            let t = sp.value.as_table().expect("alias table");
            let name = t.get("name").and_then(Value::as_str).unwrap().to_owned();
            let kind = t.get("kind").and_then(Value::as_str).unwrap().to_owned();
            let d_arr = t.get("dimension").and_then(Value::as_array).unwrap();
            assert_eq!(d_arr.len(), 6);
            let mut dim = [0i8; 6];
            for (i, d) in d_arr.iter().enumerate() {
                dim[i] = i8::try_from(d.value.as_int().unwrap()).unwrap();
            }
            AliasRow { name, dim, kind }
        })
        .collect()
}

fn parse_products(src: &str) -> Vec<ProductRow> {
    let root = toml::parse(src.as_bytes()).expect("qty_products.toml parses");
    assert_eq!(
        root.get("schema").and_then(Value::as_str),
        Some("frob.qty_products/1")
    );
    let arr = root
        .get("product")
        .and_then(Value::as_array)
        .expect("[[product]]");
    arr.iter()
        .map(|sp| {
            let t = sp.value.as_table().expect("product table");
            ProductRow {
                lhs: t.get("lhs").and_then(Value::as_str).unwrap().to_owned(),
                op: t.get("op").and_then(Value::as_str).unwrap().to_owned(),
                rhs: t.get("rhs").and_then(Value::as_str).unwrap().to_owned(),
                out: t.get("out").and_then(Value::as_str).unwrap().to_owned(),
            }
        })
        .collect()
}

fn camel_to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn check_products_dims(aliases: &[AliasRow], products: &[ProductRow]) -> Result<(), String> {
    let by_name: BTreeMap<&str, [i8; 6]> =
        aliases.iter().map(|a| (a.name.as_str(), a.dim)).collect();
    for p in products {
        let dl = *by_name
            .get(p.lhs.as_str())
            .ok_or_else(|| format!("unknown lhs `{}`", p.lhs))?;
        let dr = *by_name
            .get(p.rhs.as_str())
            .ok_or_else(|| format!("unknown rhs `{}`", p.rhs))?;
        let dout = *by_name
            .get(p.out.as_str())
            .ok_or_else(|| format!("unknown out `{}`", p.out))?;
        let mut expected = [0i8; 6];
        for i in 0..6 {
            expected[i] = match p.op.as_str() {
                "mul" => dl[i] + dr[i],
                "div" => dl[i] - dr[i],
                other => return Err(format!("invalid op `{other}`")),
            };
        }
        if dout != expected {
            return Err(format!(
                "{} {} {} => {}: expected dim {:?}, got {:?}",
                p.lhs, p.op, p.rhs, p.out, expected, dout
            ));
        }
    }
    Ok(())
}

fn check_equal_dim_coverage(
    aliases: &[AliasRow],
    compile_fail_files: &BTreeSet<String>,
) -> Result<(usize, usize), Vec<String>> {
    let mut groups: BTreeMap<[i8; 6], Vec<&AliasRow>> = BTreeMap::new();
    for a in aliases {
        groups.entry(a.dim).or_default().push(a);
    }
    let mut missing = Vec::new();
    let mut group_count = 0usize;
    let mut pair_count = 0usize;
    for members in groups.values() {
        if members.len() < 2 {
            continue;
        }
        group_count += 1;
        let kinds: BTreeSet<&str> = members.iter().map(|m| m.kind.as_str()).collect();
        if kinds.len() != members.len() {
            missing.push(format!("duplicate kind tag in group {:?}", members[0].dim));
        }
        for i in 0..members.len() {
            for j in (i + 1)..members.len() {
                pair_count += 1;
                let a = camel_to_snake(&members[i].name);
                let b = camel_to_snake(&members[j].name);
                let f1 = format!("{a}_plus_{b}.rs");
                let f2 = format!("{b}_plus_{a}.rs");
                if !compile_fail_files.contains(&f1) && !compile_fail_files.contains(&f2) {
                    missing.push(f1);
                }
            }
        }
    }
    if missing.is_empty() {
        Ok((group_count, pair_count))
    } else {
        Err(missing)
    }
}

#[test]
fn alias_registry_matches_exports() {
    let aliases = parse_aliases(ALIASES_TOML);
    let reg_names: Vec<&str> = aliases.iter().map(|a| a.name.as_str()).collect();
    assert_eq!(reg_names.as_slice(), ALIAS_NAMES);
    assert_eq!(ALIAS_NAMES.len(), ALIAS_COUNT);
    assert!(!QTY_RS.contains("pub use fs_qty::"));
    assert!(!QTY_GEN_RS.contains("pub struct Qty<"));

    // Negative control: an alias exported outside the registry is caught.
    let mut extra = ALIAS_NAMES.to_vec();
    extra.push("UndeclaredGhostQty");
    assert_ne!(reg_names.as_slice(), extra.as_slice());
}

#[test]
fn product_dimensions_add() {
    let aliases = parse_aliases(ALIASES_TOML);
    let products = parse_products(PRODUCTS_TOML);
    check_products_dims(&aliases, &products).expect("all registered products add/subtract dims");

    // Negative control: wrong output dimension must fail.
    let mut bad = products;
    bad.push(ProductRow {
        lhs: "Length".into(),
        op: "mul".into(),
        rhs: "Length".into(),
        out: "Time".into(),
    });
    assert!(check_products_dims(&aliases, &bad).is_err());
}

#[test]
fn no_duplicate_product() {
    let products = parse_products(PRODUCTS_TOML);
    let mut seen: BTreeSet<(&str, &str, &str)> = BTreeSet::new();
    for p in &products {
        assert!(
            seen.insert((p.lhs.as_str(), p.op.as_str(), p.rhs.as_str())),
            "duplicate product ({}, {}, {})",
            p.lhs,
            p.op,
            p.rhs
        );
    }
}

#[test]
fn checked_roundtrip_bits() {
    let edges = [
        -0.0_f64,
        0.0,
        f64::from_bits(1),
        f64::from_bits(0x8000_0000_0000_0001),
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        1.0,
        -1.0,
        f64::MAX,
        -f64::MAX,
    ];
    let mut state = 0x5EED_C0DE_1234_5678_u64;
    for i in 0..10_000usize {
        let raw = if i < edges.len() {
            edges[i]
        } else {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let candidate = f64::from_bits(state);
            if candidate.is_finite() {
                candidate
            } else {
                (i as f64) * 0.125
            }
        };
        let alias: Length = Checked::new(raw, Length::DIMENSION)
            .into_alias()
            .expect("matching dimension converts");
        assert_eq!(alias.to_checked().value().to_bits(), raw.to_bits());
    }
}

#[test]
fn checked_mismatch_refuses() {
    let err = Checked::new(3.0, Velocity::DIMENSION)
        .into_alias::<Length>()
        .expect_err("velocity into length must fail");
    assert_eq!(err.refusal_code(), "FROB-REF-102");
    assert_eq!(err.observed, [1, 0, -1, 0, 0, 0]);
    assert_eq!(err.required, [1, 0, 0, 0, 0, 0]);
    assert!(err.to_string().contains("FROB-REF-102"));
}

#[test]
fn alias_count_at_most_64() {
    let aliases = parse_aliases(ALIASES_TOML);
    assert!(
        aliases.len() <= MAX_ALIASES,
        "alias count {} exceeds cap {}",
        aliases.len(),
        MAX_ALIASES
    );
    assert_eq!(aliases.len(), 36);
    // Negative control: 65 aliases breaches the gate.
    assert!(65 > MAX_ALIASES);
}

fn collect_artifact_dep_dirs(exe_dir: &Path) -> (PathBuf, Vec<PathBuf>) {
    // Locate `<target>/<profile>` from either `.../deps/<exe>` (classic Cargo layout)
    // or `.../build/<crate>/<unit>/out/<exe>` (nightly Cargo build-dir layout).
    let mut profile_dir = exe_dir.to_path_buf();
    for anc in exe_dir.ancestors() {
        let name = anc.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if name == "deps" || name == "build" {
            if let Some(parent) = anc.parent() {
                profile_dir = parent.to_path_buf();
            }
            break;
        }
    }

    let mut dep_dirs: Vec<PathBuf> = Vec::new();
    let mut push_if_has_artifacts = |dir: PathBuf| {
        if let Ok(rd) = fs::read_dir(&dir) {
            let has_lib = rd.flatten().any(|e| {
                let p = e.path();
                matches!(
                    p.extension().and_then(|s| s.to_str()),
                    Some("rlib" | "rmeta" | "so" | "dylib")
                )
            });
            if has_lib {
                dep_dirs.push(dir);
            }
        }
    };

    push_if_has_artifacts(exe_dir.to_path_buf());
    push_if_has_artifacts(profile_dir.clone());
    push_if_has_artifacts(profile_dir.join("deps"));

    let build_root = profile_dir.join("build");
    if build_root.is_dir()
        && let Ok(entries) = fs::read_dir(&build_root)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let direct_out = path.join("out");
            if direct_out.is_dir() {
                push_if_has_artifacts(direct_out);
            }
            if let Ok(units) = fs::read_dir(&path) {
                for unit in units.flatten() {
                    let unit_path = unit.path();
                    if !unit_path.is_dir() {
                        continue;
                    }
                    let out_dir = unit_path.join("out");
                    if out_dir.is_dir() {
                        push_if_has_artifacts(out_dir);
                    }
                    push_if_has_artifacts(unit_path);
                }
            }
        }
    }

    dep_dirs.sort();
    dep_dirs.dedup();
    (profile_dir, dep_dirs)
}

fn collect_frob_core_candidates(dep_dirs: &[PathBuf]) -> Vec<PathBuf> {
    // Prefer `.rmeta` first (nightly Cargo pipelined builds emit full metadata in
    // `.rmeta` and only a metadata stub in `.rlib`), falling back to `.rlib`.
    let mut rmeta_candidates: Vec<PathBuf> = Vec::new();
    let mut rlib_candidates: Vec<PathBuf> = Vec::new();
    for dir in dep_dirs {
        if let Ok(rd) = fs::read_dir(dir) {
            for entry in rd.flatten() {
                let p = entry.path();
                let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if (fname.starts_with("libfrob_core-") || fname == "libfrob_core.rmeta")
                    && fname.ends_with(".rmeta")
                {
                    rmeta_candidates.push(p);
                } else if (fname.starts_with("libfrob_core-") || fname == "libfrob_core.rlib")
                    && fname.ends_with(".rlib")
                {
                    rlib_candidates.push(p);
                }
            }
        }
    }
    let sort_newest_first = |list: &mut Vec<PathBuf>| {
        list.sort_by(|a, b| {
            let ma = fs::metadata(a).and_then(|m| m.modified()).ok();
            let mb = fs::metadata(b).and_then(|m| m.modified()).ok();
            Reverse(ma.unwrap_or(SystemTime::UNIX_EPOCH))
                .cmp(&Reverse(mb.unwrap_or(SystemTime::UNIX_EPOCH)))
                .then_with(|| a.cmp(b))
        });
        list.dedup();
    };
    sort_newest_first(&mut rmeta_candidates);
    sort_newest_first(&mut rlib_candidates);
    let mut candidates = rmeta_candidates;
    candidates.extend(rlib_candidates);
    candidates
}

#[test]
fn equal_dimension_pairs_covered() {
    let aliases = parse_aliases(ALIASES_TOML);
    let cf_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/compile_fail");
    let files: BTreeSet<String> = fs::read_dir(&cf_dir)
        .expect("compile_fail dir exists")
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.ends_with(".rs"))
        .collect();

    assert!(
        files.contains("pressure_plus_time.rs"),
        "KA-779 pressure_plus_time.rs required"
    );
    let (groups, pairs) =
        check_equal_dim_coverage(&aliases, &files).expect("all equal-dim pairs covered");
    assert_eq!((groups, pairs), (5, 9));

    // Negative control: adding WorkDone with Energy's dimension without pair files fails.
    let mut with_work = aliases;
    with_work.push(AliasRow {
        name: "WorkDone".into(),
        dim: [2, 1, -2, 0, 0, 0],
        kind: "work-done".into(),
    });
    let missing = check_equal_dim_coverage(&with_work, &files)
        .expect_err("37th alias WorkDone without pair tests must fail");
    assert!(missing.iter().any(|m| m.contains("work_done")));

    // Real-compiler check on all 10 compile_fail/*.rs files + positive control Length + Length.
    let exe = env::current_exe().expect("current_exe");
    let exe_dir = exe.parent().expect("exe parent dir");
    let (profile_dir, dep_dirs) = collect_artifact_dep_dirs(exe_dir);
    let candidates = collect_frob_core_candidates(&dep_dirs);

    let scratch_base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    let out_dir = scratch_base.join(format!("frob_qty_cf_{}", std::process::id()));
    fs::create_dir_all(&out_dir).expect("mkdir cf temp");

    // Positive control: Length + Length MUST compile cleanly with these exact rustc flags.
    let pos_src = out_dir.join("positive_length_add.rs");
    fs::write(
        &pos_src,
        "use frob_core::qty::Length;\nfn main() { let _ = Length::new(1.0) + Length::new(2.0); }\n",
    )
    .expect("write positive control");

    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let mut matched_lib: Option<PathBuf> = None;
    let mut probe_diags = Vec::new();
    for (idx, candidate) in candidates.iter().enumerate() {
        let mut cmd = Command::new(&rustc);
        cmd.current_dir(env!("CARGO_MANIFEST_DIR"))
            .args(["--edition=2024", "--crate-type=bin", "--emit=metadata"])
            .arg(format!("--extern=frob_core={}", candidate.display()));
        for d in &dep_dirs {
            cmd.arg(format!("-Ldependency={}", d.display()));
        }
        let pos = cmd
            .arg("-o")
            .arg(out_dir.join(format!("positive_{idx}.rmeta")))
            .arg(&pos_src)
            .output()
            .expect("run rustc on positive control");
        if pos.status.success() {
            matched_lib = Some(candidate.clone());
            break;
        }
        probe_diags.push(format!(
            "{} (exit {:?}): {}",
            candidate.display(),
            pos.status.code(),
            String::from_utf8_lossy(&pos.stderr).trim()
        ));
    }
    let frob_core_lib = matched_lib.unwrap_or_else(|| {
        panic!(
            "must find compiled libfrob_core-*.rmeta/.rlib under {} (exe={}; rustc={rustc}; dep_dirs={}; candidates={}; tried={probe_diags:?})",
            profile_dir.display(),
            exe.display(),
            dep_dirs.len(),
            candidates.len()
        )
    });

    for name in &files {
        let src = cf_dir.join(name);
        let mut cmd = Command::new(&rustc);
        cmd.current_dir(env!("CARGO_MANIFEST_DIR"))
            .args(["--edition=2024", "--crate-type=bin", "--emit=metadata"])
            .arg(format!("--extern=frob_core={}", frob_core_lib.display()));
        for d in &dep_dirs {
            cmd.arg(format!("-Ldependency={}", d.display()));
        }
        let out = cmd
            .arg("-o")
            .arg(out_dir.join(format!("{name}.rmeta")))
            .arg(&src)
            .output()
            .expect("run rustc on compile_fail fixture");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "compile_fail fixture `{name}` unexpectedly compiled!"
        );
        assert!(
            stderr.contains("E0308"),
            "compile_fail fixture `{name}` failed without E0308:\n{stderr}"
        );
    }
}
