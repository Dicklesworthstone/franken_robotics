#![deny(unsafe_code)]

//! Confinement, text-hint, and generator-row tests for `N33c-03` (`frob_core::qty`).
//!
//! Never deletes any file or directory (COMMON.md 9b).

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use xtask::check_deps::{self, RULE_L2_FS_QTY_DIRECT, check_deps, find_fs_qty_path_hints};
use xtask::check_generated::run_check;
use xtask::cli::{self, CheckFn, EXIT_OK};
use xtask::generate::{BUILTIN_GENERATORS, GeneratorRow, RULE_DIFFERS};
use xtask::metadata::WorkspaceRoot;
use xtask::report::FactValue;

static COUNTER: AtomicU64 = AtomicU64::new(0);
const SAMPLE_RECEIPT: &str = "d3b0885b1d5bfbe09039a84ae190c40fce4ad3b0b81b5c28bf946201987462d4";
const SAMPLE_GIT_REV: &str = "1111111111111111111111111111111111111111";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn fresh_dir(tag: &str) -> PathBuf {
    let base = env::var_os("BIG_SCRATCH")
        .or_else(|| env::var_os("TMPDIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let dir = base.join(format!(
        "frob_qty_confinement_{tag}_{}_{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&dir).expect("mkdir scratch");
    dir
}

fn write_file(path: &Path, content: &str) {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).expect("mkdir parent");
    }
    fs::write(path, content).expect("write file");
}

fn copy_file(src: &Path, dst: &Path) {
    if let Some(p) = dst.parent() {
        fs::create_dir_all(p).expect("mkdir parent");
    }
    fs::copy(src, dst).expect("copy file");
}

fn make_synth_repo(tag: &str, frob_lie_deps: &str, frob_lie_src: &str) -> PathBuf {
    let base = fresh_dir(tag);
    let repo = base.join("repo");
    let fs_qty = base.join("frankensim/crates/fs-qty");
    write_file(
        &fs_qty.join("Cargo.toml"),
        "[package]\nname = \"fs-qty\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    );
    write_file(&fs_qty.join("src/lib.rs"), "pub struct Length;\n");

    write_file(
        &repo.join("Cargo.toml"),
        "[workspace]\nresolver = \"3\"\nmembers = [\"crates/frob-core\", \"crates/frob-fsim\", \"crates/frob-lie\"]\n",
    );
    for (dir, name, layer, rank, deps, src) in [
        (
            "crates/frob-core",
            "frob-core",
            "0",
            1,
            "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
            "// frob-core allowed\n",
        ),
        (
            "crates/frob-fsim",
            "frob-fsim",
            "0",
            2,
            "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
            "// frob-fsim allowed\n",
        ),
        (
            "crates/frob-lie",
            "frob-lie",
            "1",
            1,
            frob_lie_deps,
            frob_lie_src,
        ),
    ] {
        write_file(
            &repo.join(dir).join("Cargo.toml"),
            &format!(
                "[package]\nname = \"{name}\"\nversion = \"0.0.1\"\nedition = \"2024\"\n\n[package.metadata.frob]\nlayer = {layer}\nrank = {rank}\n\n{deps}\n"
            ),
        );
        write_file(&repo.join(dir).join("src/lib.rs"), src);
    }

    let lock_toml = format!(
        "toolchain = \"nightly-2026-07-06\"\n\n[[foundation]]\nid = \"FD-FSIM\"\nname = \"frankensim\"\nposture = \"consume-path\"\npath = \"../frankensim\"\ngit_rev = \"{SAMPLE_GIT_REV}\"\ntoolchain = \"nightly-2026-07-06\"\nfeatures = [\"referee\"]\nconfined_to = [\"frob-fsim\", \"bedrock-direct\"]\nadmission_gate = \"INT-FSIM-01\"\nreceipt = \"{SAMPLE_RECEIPT}\"\nfallback = \"synthetic\"\n"
    );
    write_file(&repo.join("franken-stack.lock"), &lock_toml);

    let out = Command::new("cargo")
        .current_dir(&repo)
        .args(["generate-lockfile", "--offline", "--manifest-path"])
        .arg(repo.join("Cargo.toml"))
        .output()
        .expect("generate lockfile");
    assert!(
        out.status.success(),
        "generate-lockfile failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    repo
}

#[test]
fn check_deps_rejects_fs_qty_outside_allowed() {
    let repo = make_synth_repo(
        "reject",
        "[dependencies]\nfs-qty = { path = \"../../../frankensim/crates/fs-qty\", default-features = false }\n",
        "// planted forbidden direct fs-qty edge\n",
    );
    let outcome = check_deps(&repo, &[WorkspaceRoot::core(&repo)]).expect("check_deps runs");
    assert!(
        outcome.findings.iter().any(|f| {
            f.rule == RULE_L2_FS_QTY_DIRECT
                && f.subject == "frob-lie -> fs-qty"
                && f.path == "crates/frob-lie/Cargo.toml"
        }),
        "expected L-2.fs-qty-direct for frob-lie -> fs-qty, got {:?}",
        outcome.findings
    );
}

#[test]
fn fs_qty_path_hint_reported() {
    let repo = make_synth_repo(
        "hint",
        "",
        "#[allow(unused)]\nfn hint_sample() { let _ = fs_qty::Length; }\n",
    );
    let hints = find_fs_qty_path_hints(&repo);
    assert!(
        hints
            .iter()
            .any(|h| h.contains("crates/frob-lie/src/lib.rs:2: fs_qty::")),
        "expected fs_qty:: hint in frob-lie, got {hints:?}"
    );
    // Reported as a `facts` entry, NOT a finding when manifest has no forbidden dep.
    const CMDS: &[(&str, CheckFn)] = &[("check-deps", check_deps::run_command)];
    let args = vec![
        "check-deps".to_owned(),
        "--root".to_owned(),
        repo.to_string_lossy().into_owned(),
        "--json".to_owned(),
    ];
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let exit = cli::run(CMDS, &args, &mut out, &mut err);
    assert_eq!(
        exit,
        std::process::ExitCode::from(EXIT_OK),
        "hint must not become a finding: {}",
        String::from_utf8_lossy(&out)
    );
    let json = String::from_utf8(out).expect("utf8");
    assert!(json.contains("\"fs_qty_path_hints\""));
    assert!(json.contains("crates/frob-lie/src/lib.rs:2: fs_qty::"));
}

fn qty_generator_slice() -> &'static [GeneratorRow] {
    let qty_row = BUILTIN_GENERATORS
        .iter()
        .find(|row| row.name == "quantity-aliases-and-products")
        .expect("quantity-aliases-and-products generator row registered");
    std::slice::from_ref(qty_row)
}

fn make_qty_gen_scratch(tag: &str) -> PathBuf {
    let real = repo_root();
    let dir = fresh_dir(tag);
    for rel in [
        "registries/qty_aliases.toml",
        "registries/qty_products.toml",
        "crates/frob-core/src/qty_gen.rs",
    ] {
        copy_file(&real.join(rel), &dir.join(rel));
    }
    dir
}

fn find_alias_count_span(src: &str) -> (usize, usize, usize) {
    const PREFIX: &str = "pub const ALIAS_COUNT: usize = ";
    let mut matches = src.match_indices(PREFIX);
    let (start, _) = matches
        .next()
        .expect("qty_gen.rs must declare pub const ALIAS_COUNT");
    assert!(
        matches.next().is_none(),
        "qty_gen.rs must declare ALIAS_COUNT exactly once"
    );
    let digits_start = start + PREFIX.len();
    let tail = &src[digits_start..];
    let semi = tail
        .find(';')
        .expect("ALIAS_COUNT declaration must end with `;`");
    let count: usize = tail[..semi]
        .parse()
        .expect("ALIAS_COUNT initializer must be a decimal usize");
    (digits_start, digits_start + semi, count)
}

fn parse_qty_gen_alias_count(src: &str) -> usize {
    find_alias_count_span(src).2
}

fn mutate_qty_gen_alias_count(orig: &str) -> String {
    let (digits_start, digits_end, count) = find_alias_count_span(orig);
    let bumped = count
        .checked_add(1)
        .expect("ALIAS_COUNT + 1 must not overflow");
    let mut mutated = String::with_capacity(orig.len() + 2);
    mutated.push_str(&orig[..digits_start]);
    mutated.push_str(&bumped.to_string());
    mutated.push_str(&orig[digits_end..]);
    assert_ne!(
        mutated, orig,
        "mutated qty_gen.rs must differ from original"
    );
    mutated
}

fn assert_qty_row_refusal(
    dir: &Path,
    rows: &[GeneratorRow],
    expected_path: &str,
    expected_reason_parts: &[&str],
) {
    let gen_path = dir.join("crates/frob-core/src/qty_gen.rs");
    let before_bytes = fs::read(&gen_path).expect("read existing qty_gen.rs");

    for write_mode in [false, true] {
        let outcome = run_check(dir, rows, write_mode).expect("run_check returns findings");
        assert_eq!(
            outcome.findings.len(),
            1,
            "write_mode={write_mode}: expected exactly one gen.differs finding, got {:?}",
            outcome.findings
        );
        let finding = &outcome.findings[0];
        assert_eq!(finding.rule, RULE_DIFFERS, "write_mode={write_mode}");
        assert_eq!(
            finding.subject, "quantity-aliases-and-products",
            "write_mode={write_mode}"
        );
        assert_eq!(finding.path, expected_path, "write_mode={write_mode}");
        for needle in expected_reason_parts {
            assert!(
                finding.message.contains(needle),
                "write_mode={write_mode}: finding message `{}` missing `{needle}`",
                finding.message
            );
        }
        assert_eq!(
            outcome.facts.get("written_files"),
            Some(&FactValue::Array(Vec::new())),
            "write_mode={write_mode}: refusal must not record any written files"
        );
        let after_bytes = fs::read(&gen_path).expect("re-read qty_gen.rs after check");
        assert_eq!(
            after_bytes, before_bytes,
            "write_mode={write_mode}: existing qty_gen.rs must remain byte-identical on refusal"
        );
    }
}

#[test]
fn qty_generator_row() {
    let dir = make_qty_gen_scratch("gen");
    let rows = qty_generator_slice();

    // Clean tree passes check-generated on the quantity generator row.
    let clean = run_check(&dir, rows, false).expect("clean check");
    assert!(
        clean.findings.is_empty(),
        "clean findings: {:?}",
        clean.findings
    );

    // Plant 1: derive ALIAS_COUNT from qty_gen.rs and bump by 1 -> exact single gen.differs finding.
    let gen_path = dir.join("crates/frob-core/src/qty_gen.rs");
    let orig = fs::read_to_string(&gen_path).expect("read qty_gen.rs");
    let mutated = mutate_qty_gen_alias_count(&orig);
    fs::write(&gen_path, &mutated).expect("write mutated qty_gen.rs");
    let bad_byte = run_check(&dir, rows, false).expect("check mutated byte");
    assert_eq!(bad_byte.findings.len(), 1, "{:?}", bad_byte.findings);
    assert_eq!(bad_byte.findings[0].rule, RULE_DIFFERS);
    assert_eq!(
        bad_byte.findings[0].subject,
        "quantity-aliases-and-products"
    );
    assert_eq!(bad_byte.findings[0].path, "crates/frob-core/src/qty_gen.rs");

    // Restore qty_gen.rs, then Plant 2: append an alias to qty_aliases.toml without regeneration -> gen.differs.
    fs::write(&gen_path, &orig).expect("restore qty_gen.rs");
    let aliases_path = dir.join("registries/qty_aliases.toml");
    let orig_aliases = fs::read_to_string(&aliases_path).expect("read aliases");
    let mut aliases = orig_aliases.clone();
    aliases.push_str(
        "\n[[alias]]\nname = \"Jerk\"\ndimension = [1, 0, -3, 0, 0, 0]\nkind = \"jerk\"\nunit = \"m·s^-3\"\n",
    );
    fs::write(&aliases_path, &aliases).expect("append alias");
    let bad_reg = run_check(&dir, rows, false).expect("check unregenerated registry");
    assert!(
        bad_reg
            .findings
            .iter()
            .any(|f| f.rule == RULE_DIFFERS && f.path == "crates/frob-core/src/qty_gen.rs"),
        "expected gen.differs on unregenerated alias, got {:?}",
        bad_reg.findings
    );

    // Restore qty_aliases.toml, then Plant 3: wrong product dimension -> generator rejects.
    fs::write(&aliases_path, &orig_aliases).expect("restore aliases");
    let products_path = dir.join("registries/qty_products.toml");
    let mut products = fs::read_to_string(&products_path).expect("read products");
    products.push_str(
        "\n[[product]]\nlhs = \"Length\"\nop = \"mul\"\nrhs = \"Time\"\nout = \"Area\"\n",
    );
    fs::write(&products_path, &products).expect("append bad product");
    let bad_dim = run_check(&dir, rows, false).expect("check wrong product dimension");
    assert!(
        bad_dim.findings.iter().any(|f| {
            f.rule == RULE_DIFFERS
                && f.path == "registries/qty_products.toml"
                && f.message.contains("dimension mismatch")
        }),
        "expected dimension mismatch finding on bad product, got {:?}",
        bad_dim.findings
    );
}

#[test]
fn qty_byte_plant_survives_regenerated_37th_alias() {
    let dir = make_qty_gen_scratch("gen-37th");
    let rows = qty_generator_slice();
    let gen_path = dir.join("crates/frob-core/src/qty_gen.rs");
    let aliases_path = dir.join("registries/qty_aliases.toml");

    let base_count = parse_qty_gen_alias_count(&fs::read_to_string(&gen_path).expect("read gen"));
    let mut aliases = fs::read_to_string(&aliases_path).expect("read aliases");
    aliases.push_str(
        "\n[[alias]]\nname = \"Jerk\"\ndimension = [1, 0, -3, 0, 0, 0]\nkind = \"jerk\"\nunit = \"m·s^-3\"\n",
    );
    fs::write(&aliases_path, &aliases).expect("write 37th alias");

    // Legitimately regenerate qty_gen.rs for the new alias count, then verify clean.
    let wrote = run_check(&dir, rows, true).expect("regenerate for added alias");
    assert!(
        wrote.findings.is_empty(),
        "write findings: {:?}",
        wrote.findings
    );
    let clean = run_check(&dir, rows, false).expect("clean check after regeneration");
    assert!(
        clean.findings.is_empty(),
        "clean findings: {:?}",
        clean.findings
    );

    let regen = fs::read_to_string(&gen_path).expect("read regenerated qty_gen.rs");
    assert_eq!(parse_qty_gen_alias_count(&regen), base_count + 1);

    // Derived-count plant must remain usable after regeneration and produce one gen.differs.
    let mutated = mutate_qty_gen_alias_count(&regen);
    fs::write(&gen_path, &mutated).expect("write derived mutation");
    let planted = run_check(&dir, rows, false).expect("check derived byte plant");
    assert_eq!(planted.findings.len(), 1, "{:?}", planted.findings);
    assert_eq!(planted.findings[0].rule, RULE_DIFFERS);
    assert_eq!(planted.findings[0].subject, "quantity-aliases-and-products");
    assert_eq!(planted.findings[0].path, "crates/frob-core/src/qty_gen.rs");
}

#[test]
fn qty_generator_alias_cap_64_accepted_and_65_refused() {
    let dir = make_qty_gen_scratch("cap-64-65");
    let rows = qty_generator_slice();
    let gen_path = dir.join("crates/frob-core/src/qty_gen.rs");
    let aliases_path = dir.join("registries/qty_aliases.toml");

    let base_count = parse_qty_gen_alias_count(&fs::read_to_string(&gen_path).expect("read gen"));
    assert!(
        base_count <= 64,
        "starter registry exceeds cap: {base_count}"
    );

    let mut aliases = fs::read_to_string(&aliases_path).expect("read aliases");
    for idx in (base_count + 1)..=64 {
        aliases.push_str(&format!(
            "\n[[alias]]\nname = \"SyntheticQty{idx:02}\"\ndimension = [0, 0, 0, 0, 0, 1]\nkind = \"synthetic-qty-{idx:02}\"\nunit = \"mol\"\n"
        ));
    }
    fs::write(&aliases_path, &aliases).expect("write 64 aliases");

    // Positive control: 64 otherwise valid aliases are accepted and regenerate cleanly.
    let wrote_64 = run_check(&dir, rows, true).expect("write 64 aliases");
    assert!(
        wrote_64.findings.is_empty(),
        "64 aliases must be accepted under --write, got {:?}",
        wrote_64.findings
    );
    let clean_64 = run_check(&dir, rows, false).expect("check 64 aliases");
    assert!(
        clean_64.findings.is_empty(),
        "64 aliases must pass clean check, got {:?}",
        clean_64.findings
    );
    assert_eq!(
        parse_qty_gen_alias_count(&fs::read_to_string(&gen_path).expect("read 64 gen")),
        64
    );

    // Negative control: 65th otherwise valid alias is refused with one gen.differs and --write leaves qty_gen.rs unchanged.
    aliases.push_str(
        "\n[[alias]]\nname = \"SyntheticQty65\"\ndimension = [0, 0, 0, 0, 0, 1]\nkind = \"synthetic-qty-65\"\nunit = \"mol\"\n",
    );
    fs::write(&aliases_path, &aliases).expect("write 65 aliases");
    assert_qty_row_refusal(
        &dir,
        rows,
        "registries/qty_aliases.toml",
        &["65", "exceeds cap 64", "OPEN-SUITE-4"],
    );
}

#[test]
fn qty_generator_refuses_repeated_ordered_triple() {
    let rows = qty_generator_slice();

    // Exact duplicate of an existing ordered triple.
    let dir_exact = make_qty_gen_scratch("dup-exact");
    let products_path = dir_exact.join("registries/qty_products.toml");
    let mut products = fs::read_to_string(&products_path).expect("read products");
    products.push_str(
        "\n[[product]]\nlhs = \"Length\"\nop = \"mul\"\nrhs = \"Length\"\nout = \"Area\"\n",
    );
    fs::write(&products_path, &products).expect("write duplicate product");
    assert_qty_row_refusal(
        &dir_exact,
        rows,
        "registries/qty_products.toml",
        &["duplicate", "(Length, mul, Length)"],
    );

    // Same ordered triple targeting a different equal-dimension output alias (Force * Length -> Torque vs Energy).
    let dir_alt = make_qty_gen_scratch("dup-alt-out");
    let products_path_alt = dir_alt.join("registries/qty_products.toml");
    let mut products_alt = fs::read_to_string(&products_path_alt).expect("read products");
    products_alt.push_str(
        "\n[[product]]\nlhs = \"Force\"\nop = \"mul\"\nrhs = \"Length\"\nout = \"Torque\"\n",
    );
    fs::write(&products_path_alt, &products_alt).expect("write conflicting triple");
    assert_qty_row_refusal(
        &dir_alt,
        rows,
        "registries/qty_products.toml",
        &["duplicate", "(Force, mul, Length)"],
    );
}

#[test]
fn qty_generator_refuses_self_division_to_ratio() {
    let dir = make_qty_gen_scratch("self-div");
    let rows = qty_generator_slice();
    let products_path = dir.join("registries/qty_products.toml");
    let mut products = fs::read_to_string(&products_path).expect("read products");
    products.push_str(
        "\n[[product]]\nlhs = \"Length\"\nop = \"div\"\nrhs = \"Length\"\nout = \"Ratio\"\n",
    );
    fs::write(&products_path, &products).expect("write self-division product");
    assert_qty_row_refusal(
        &dir,
        rows,
        "registries/qty_products.toml",
        &["Length / Length", "self-division", "Ratio"],
    );
}

#[test]
fn qty_generator_refuses_i8_exponent_overflow_high_and_low() {
    let rows = qty_generator_slice();

    // Positive overflow: 127 + 1 via mul.
    let dir_hi = make_qty_gen_scratch("overflow-hi");
    let aliases_hi = dir_hi.join("registries/qty_aliases.toml");
    let mut a_hi = fs::read_to_string(&aliases_hi).expect("read aliases");
    a_hi.push_str(
        "\n[[alias]]\nname = \"MaxLengthPow\"\ndimension = [127, 0, 0, 0, 0, 0]\nkind = \"max-length-pow\"\nunit = \"m^127\"\n",
    );
    fs::write(&aliases_hi, &a_hi).expect("write MaxLengthPow alias");
    let products_hi = dir_hi.join("registries/qty_products.toml");
    let mut p_hi = fs::read_to_string(&products_hi).expect("read products");
    p_hi.push_str(
        "\n[[product]]\nlhs = \"MaxLengthPow\"\nop = \"mul\"\nrhs = \"Length\"\nout = \"MaxLengthPow\"\n",
    );
    fs::write(&products_hi, &p_hi).expect("write 127+1 product");
    assert_qty_row_refusal(
        &dir_hi,
        rows,
        "registries/qty_products.toml",
        &["MaxLengthPow mul Length", "overflow"],
    );

    // Negative overflow: -128 - 1 via div.
    let dir_lo = make_qty_gen_scratch("overflow-lo");
    let aliases_lo = dir_lo.join("registries/qty_aliases.toml");
    let mut a_lo = fs::read_to_string(&aliases_lo).expect("read aliases");
    a_lo.push_str(
        "\n[[alias]]\nname = \"MinLengthPow\"\ndimension = [-128, 0, 0, 0, 0, 0]\nkind = \"min-length-pow\"\nunit = \"m^-128\"\n",
    );
    fs::write(&aliases_lo, &a_lo).expect("write MinLengthPow alias");
    let products_lo = dir_lo.join("registries/qty_products.toml");
    let mut p_lo = fs::read_to_string(&products_lo).expect("read products");
    p_lo.push_str(
        "\n[[product]]\nlhs = \"MinLengthPow\"\nop = \"div\"\nrhs = \"Length\"\nout = \"MinLengthPow\"\n",
    );
    fs::write(&products_lo, &p_lo).expect("write -128-1 product");
    assert_qty_row_refusal(
        &dir_lo,
        rows,
        "registries/qty_products.toml",
        &["MinLengthPow div Length", "overflow"],
    );
}
