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

fn parse_defined_lib_helpers(lib_sh: &str) -> std::collections::BTreeSet<String> {
    let mut out = std::collections::BTreeSet::new();
    for line in lib_sh.lines() {
        if let Some(rest) = line.strip_prefix("e2e_")
            && let Some(idx) = rest.find("()")
        {
            let name = &rest[..idx];
            if !name.is_empty()
                && name
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
            {
                out.insert(format!("e2e_{name}"));
            }
        }
    }
    out
}

fn shell_split_simple(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut in_tok = false;
    for ch in line.chars() {
        if escaped {
            cur.push(ch);
            escaped = false;
            in_tok = true;
            continue;
        }
        match quote {
            Some('\'') => {
                if ch == '\'' {
                    quote = None;
                } else {
                    cur.push(ch);
                }
                in_tok = true;
            }
            Some('"') => {
                if ch == '\\' {
                    escaped = true;
                } else if ch == '"' {
                    quote = None;
                } else {
                    cur.push(ch);
                }
                in_tok = true;
            }
            None => match ch {
                '\\' => {
                    escaped = true;
                    in_tok = true;
                }
                '\'' | '"' => {
                    quote = Some(ch);
                    in_tok = true;
                }
                '#' if !in_tok => break,
                c if c.is_ascii_whitespace() => {
                    if in_tok {
                        tokens.push(std::mem::take(&mut cur));
                        in_tok = false;
                    }
                }
                _ => {
                    cur.push(ch);
                    in_tok = true;
                }
            },
            Some(_) => unreachable!(),
        }
    }
    if in_tok {
        tokens.push(cur);
    }
    tokens
}

fn detect_heredoc_end(line: &str) -> Option<String> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'<' && bytes[i + 1] == b'<' {
            // Skip `<<<` (here-string).
            if i + 2 < bytes.len() && bytes[i + 2] == b'<' {
                i += 3;
                continue;
            }
            let mut j = i + 2;
            if j < bytes.len() && bytes[j] == b'-' {
                j += 1;
            }
            while j < bytes.len() && (bytes[j] == b' ' || bytes[j] == b'\t') {
                j += 1;
            }
            let rest = &line[j..];
            let delim: String = rest
                .chars()
                .take_while(|c| !c.is_ascii_whitespace() && *c != ';' && *c != ')')
                .filter(|c| *c != '\'' && *c != '"' && *c != '\\')
                .collect();
            if !delim.is_empty() {
                return Some(delim);
            }
        }
        i += 1;
    }
    None
}

fn audit_e2e_script(
    script_name: &str,
    content: &str,
    defined: &std::collections::BTreeSet<String>,
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut heredoc_end: Option<String> = None;
    let mut logical_lines: Vec<(usize, String)> = Vec::new();
    let mut cur_line = String::new();
    let mut cur_start = 1usize;

    for (idx, raw) in content.lines().enumerate() {
        let lineno = idx + 1;
        if let Some(ref delim) = heredoc_end {
            if raw.trim() == delim {
                heredoc_end = None;
            }
            continue;
        }
        if cur_line.is_empty() {
            cur_start = lineno;
        }
        let trimmed_end = raw.trim_end();
        if let Some(stripped) = trimmed_end.strip_suffix('\\') {
            cur_line.push_str(stripped);
            cur_line.push(' ');
            continue;
        }
        cur_line.push_str(raw);
        let completed = std::mem::take(&mut cur_line);
        if let Some(delim) = detect_heredoc_end(&completed) {
            heredoc_end = Some(delim);
        }
        logical_lines.push((cur_start, completed));
    }
    if !cur_line.is_empty() {
        logical_lines.push((cur_start, cur_line));
    }

    // Header `plants=...` declarations must be checked by the script.
    for line in content.lines().take(5) {
        if let Some(pos) = line.find("plants=") {
            let val: String = line[pos + "plants=".len()..]
                .chars()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect();
            if val != "none" && !val.is_empty() {
                for plant in val.split(',').filter(|p| !p.is_empty()) {
                    let needle_dq = format!("e2e_plant \"{plant}\"");
                    let needle_sq = format!("e2e_plant '{plant}'");
                    let needle_bare = format!("e2e_plant {plant}");
                    if !content.contains(&needle_dq)
                        && !content.contains(&needle_sq)
                        && !content.contains(&needle_bare)
                        && !content.contains("E2E_PLANT_FAILURE")
                    {
                        errors.push(format!(
                            "{script_name}: header declares plant `{plant}` but script never checks e2e_plant/E2E_PLANT_FAILURE"
                        ));
                    }
                }
            }
        }
    }

    for (lineno, line) in logical_lines {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let toks = shell_split_simple(trimmed);
        if toks.is_empty() {
            continue;
        }
        // Handle simple `if e2e_...` or `if ! e2e_...` prefixes.
        let mut cmd_idx = 0usize;
        if toks[cmd_idx] == "if" || toks[cmd_idx] == "elif" {
            cmd_idx += 1;
            if toks.get(cmd_idx).map(String::as_str) == Some("!") {
                cmd_idx += 1;
            }
        }
        let Some(cmd) = toks.get(cmd_idx).map(String::as_str) else {
            continue;
        };
        if !cmd.starts_with("e2e_") {
            continue;
        }
        if !defined.contains(cmd) {
            errors.push(format!(
                "{script_name}:{lineno}: calls undefined STD-E2E helper `{cmd}`"
            ));
            continue;
        }
        let args = &toks[cmd_idx + 1..];
        match cmd {
            "e2e_cmd" => {
                let Some(sep) = args.iter().position(|t| t == "--") else {
                    errors.push(format!(
                        "{script_name}:{lineno}: e2e_cmd missing `--` separator before command"
                    ));
                    continue;
                };
                let opts = &args[..sep];
                let mut i = 0;
                while i < opts.len() {
                    if (opts[i] == "--name" || opts[i] == "--expect-exit") && i + 1 < opts.len() {
                        i += 2;
                    } else {
                        errors.push(format!(
                            "{script_name}:{lineno}: e2e_cmd unexpected option `{}` (allowed: --name <v>, --expect-exit <v>)",
                            opts[i]
                        ));
                        break;
                    }
                }
            }
            "e2e_step" => {
                if args.len() < 2 {
                    errors.push(format!(
                        "{script_name}:{lineno}: e2e_step requires `<id> <title>`, got {} arg(s)",
                        args.len()
                    ));
                }
            }
            "e2e_assert_json" => {
                if args.len() < 3 || !args[1].starts_with('/') {
                    errors.push(format!(
                        "{script_name}:{lineno}: e2e_assert_json requires `<name> <json-pointer> <expected>` (arg 2 must start with `/`)"
                    ));
                } else {
                    let tags = &args[3..];
                    let mut i = 0;
                    while i < tags.len() {
                        if (matches!(
                            tags[i].as_str(),
                            "--gate" | "--ka" | "--cx" | "--nc" | "--mr"
                        ) || tags[i].starts_with('$'))
                            && i + 1 < tags.len()
                        {
                            i += 2;
                        } else {
                            errors.push(format!(
                                "{script_name}:{lineno}: e2e_assert_json unexpected trailing arg `{}` (signature is `<name> <json-pointer> <expected> [tags...]`, not `<name> <file> <pointer> <expected>`)",
                                tags[i]
                            ));
                            break;
                        }
                    }
                }
            }
            "e2e_expect_caught" | "e2e_expect_refusal" => {
                if !args.iter().any(|t| t == "--") {
                    errors.push(format!(
                        "{script_name}:{lineno}: {cmd} missing `--` separator before command"
                    ));
                }
            }
            _ => {}
        }
    }
    errors
}

#[test]
fn e2e_scripts_conform_to_lib_sh_contract() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("locate repo root from CARGO_MANIFEST_DIR");
    let e2e_dir = repo_root.join("scripts/e2e");
    let lib_sh = std::fs::read_to_string(e2e_dir.join("lib.sh")).expect("read scripts/e2e/lib.sh");
    let defined = parse_defined_lib_helpers(&lib_sh);
    assert!(
        defined.contains("e2e_cmd") && defined.contains("e2e_assert_json"),
        "lib.sh helper parser must find core helpers"
    );

    // Negative regressions: confirm each bug class found in conformance_v0.sh,
    // std_test_lint.sh, and tenfold_records.sh is caught before passing clean scripts.
    let planted_bad = "\
#!/usr/bin/env bash
# e2e: bead=TEST milestone=M0 plants=missing-plant
e2e_step \"only_one_arg\"
e2e_cmd \"bad_positional_name\" -- echo hi
e2e_cmd --allow-fail echo hi
e2e_assert_json \"claim\" \"/tmp/ev.json\" \"/claim\" \"known-answer\"
e2e_assert_contains \"out\" \"needle\"
";
    let planted_errs = audit_e2e_script("planted.sh", planted_bad, &defined);
    assert!(
        planted_errs.iter().any(|e| e.contains("missing-plant")),
        "must catch undeclared plant check: {planted_errs:?}"
    );
    assert!(
        planted_errs.iter().any(|e| e.contains("e2e_step")),
        "must catch 1-arg e2e_step: {planted_errs:?}"
    );
    assert!(
        planted_errs.iter().any(|e| e.contains("bad_positional_name")),
        "must catch e2e_cmd missing --name: {planted_errs:?}"
    );
    assert!(
        planted_errs.iter().any(|e| e.contains("missing `--`")),
        "must catch e2e_cmd missing `--`: {planted_errs:?}"
    );
    assert!(
        planted_errs.iter().any(|e| e.contains("e2e_assert_json")),
        "must catch e2e_assert_json non-pointer arg2: {planted_errs:?}"
    );
    assert!(
        planted_errs.iter().any(|e| e.contains("e2e_assert_contains")),
        "must catch undefined helper e2e_assert_contains: {planted_errs:?}"
    );

    // Every real script under scripts/e2e/*.sh must conform.
    let mut all_errors = Vec::new();
    let mut scripts_checked = 0usize;
    let mut entries: Vec<_> = std::fs::read_dir(&e2e_dir)
        .expect("read scripts/e2e")
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sh") {
            continue;
        }
        let fname = entry.file_name().to_string_lossy().into_owned();
        if matches!(fname.as_str(), "lib.sh" | "run-all.sh" | "selftest.sh") {
            continue;
        }
        let body = std::fs::read_to_string(&path).expect("read e2e script");
        scripts_checked += 1;
        all_errors.extend(audit_e2e_script(&fname, &body, &defined));
    }
    assert!(
        scripts_checked >= 5,
        "expected at least 5 scripts in scripts/e2e, found {scripts_checked}"
    );
    assert!(
        all_errors.is_empty(),
        "scripts/e2e lib.sh contract violations:\n{}",
        all_errors.join("\n")
    );
}
