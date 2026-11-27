//! Script header parser (`frob-e2e list`) and gate report generator (`frob-e2e report`).
//! Reads headers and NDJSON logs only; never reads or writes any registry.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::e2e::json::{self, JsonValue, obj};
use crate::e2e::line::{ParsedFlags, format_utc_compact, now_utc_ms};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptHeader {
    pub script: String,
    pub path: PathBuf,
    pub bead: String,
    pub milestone: String,
    pub labels: Vec<String>,
    pub gates: Vec<String>,
    pub seed: u64,
    pub timeout_s: u64,
    pub needs: Vec<String>,
    pub plants: Vec<String>,
}

impl ScriptHeader {
    pub fn to_json(&self) -> JsonValue {
        obj(vec![
            ("script", JsonValue::String(self.script.clone())),
            (
                "path",
                JsonValue::String(self.path.to_string_lossy().into_owned()),
            ),
            ("bead", JsonValue::String(self.bead.clone())),
            ("milestone", JsonValue::String(self.milestone.clone())),
            ("labels", str_array(&self.labels)),
            ("gates", str_array(&self.gates)),
            ("seed", JsonValue::Number(self.seed.to_string())),
            ("timeout_s", JsonValue::Number(self.timeout_s.to_string())),
            ("needs", str_array(&self.needs)),
            ("plants", str_array(&self.plants)),
        ])
    }
}

fn str_array(items: &[String]) -> JsonValue {
    JsonValue::Array(items.iter().cloned().map(JsonValue::String).collect())
}
fn split_csv(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

/// Parses `# e2e:` header lines in the leading comment block of a script.
pub fn parse_script_header(path: &Path) -> Result<Option<ScriptHeader>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let (mut found, mut bead, mut milestone) = (false, String::new(), String::new());
    let (mut labels, mut gates, mut needs, mut plants) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let (mut seed, mut timeout_s) = (0u64, 600u64);

    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            break;
        }
        let Some(rest) = trimmed.strip_prefix("# e2e:") else {
            continue;
        };
        found = true;
        for tok in rest.split_whitespace() {
            let (k, v) = tok
                .split_once('=')
                .ok_or_else(|| format!("{}:{}: malformed '{tok}'", path.display(), idx + 1))?;
            match k {
                "bead" => bead = v.into(),
                "milestone" => milestone = v.into(),
                "labels" => labels.extend(split_csv(v)),
                "gates" => gates.extend(split_csv(v)),
                "seed" => {
                    seed = v.parse().map_err(|_| {
                        format!("{}:{}: invalid seed '{v}'", path.display(), idx + 1)
                    })?
                }
                "timeout_s" => {
                    timeout_s = v.parse().map_err(|_| {
                        format!("{}:{}: invalid timeout_s '{v}'", path.display(), idx + 1)
                    })?
                }
                "needs" => needs.extend(split_csv(v)),
                "plants" => plants.extend(split_csv(v)),
                other => {
                    return Err(format!(
                        "{}:{}: unknown header key '{other}'",
                        path.display(),
                        idx + 1
                    ));
                }
            }
        }
    }
    if !found {
        return Ok(None);
    }
    let script = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    Ok(Some(ScriptHeader {
        script,
        path: path.to_path_buf(),
        bead,
        milestone,
        labels,
        gates,
        seed,
        timeout_s,
        needs,
        plants,
    }))
}

pub fn list_scripts(scripts_dir: &Path) -> Result<Vec<ScriptHeader>, String> {
    if !scripts_dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths: Vec<PathBuf> = fs::read_dir(scripts_dir)
        .map_err(|e| format!("read {}: {e}", scripts_dir.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("sh"))
        .collect();
    paths.sort();
    let mut out = Vec::new();
    for p in paths {
        if let Some(h) = parse_script_header(&p)? {
            out.push(h);
        }
    }
    Ok(out)
}

fn milestone_num(m: &str) -> Option<u32> {
    m.strip_prefix('M')
        .filter(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))?
        .parse()
        .ok()
}

pub fn cmd_list(args: &[String]) -> i32 {
    const VALUED: &[&str] = &[
        "--scripts-dir",
        "--milestone",
        "--through",
        "--label",
        "--gate",
        "--bead",
        "--script",
    ];
    let f = match ParsedFlags::parse(args, VALUED, &[], &["--json"]) {
        Ok(f) if f.positionals.is_empty() => f,
        Ok(_) => {
            eprintln!("frob-e2e list: unexpected positional argument");
            return 2;
        }
        Err(e) => {
            eprintln!("frob-e2e list: {e}");
            return 2;
        }
    };
    let through = match f.get("--through") {
        Some(v) => match milestone_num(v) {
            Some(n) => Some(n),
            None => {
                eprintln!("frob-e2e list: invalid --through: {v}");
                return 2;
            }
        },
        None => None,
    };
    let req_labels = f.get("--label").map(split_csv).unwrap_or_default();
    let all = match list_scripts(Path::new(f.get_or("--scripts-dir", "scripts/e2e"))) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("frob-e2e list: {e}");
            return 2;
        }
    };
    let filtered: Vec<ScriptHeader> = all
        .into_iter()
        .filter(|h| {
            if let Some(m) = f.get("--milestone")
                && h.milestone != m
            {
                return false;
            }
            if let Some(max_m) = through
                && !milestone_num(&h.milestone).is_some_and(|n| n <= max_m)
            {
                return false;
            }
            if !req_labels
                .iter()
                .all(|req| h.labels.iter().any(|l| l == req))
            {
                return false;
            }
            if let Some(g) = f.get("--gate")
                && !h.gates.iter().any(|x| x == g)
            {
                return false;
            }
            if let Some(b) = f.get("--bead")
                && h.bead != b
            {
                return false;
            }
            if let Some(s) = f.get("--script") {
                let stem = s.strip_suffix(".sh").unwrap_or(s);
                if h.script != stem && h.path.to_string_lossy() != s {
                    return false;
                }
            }
            true
        })
        .collect();

    if f.has("--json") {
        println!(
            "{}",
            JsonValue::Array(filtered.iter().map(ScriptHeader::to_json).collect()).to_json_string()
        );
    } else {
        for h in &filtered {
            println!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                h.path.display(),
                h.bead,
                h.milestone,
                h.seed,
                h.timeout_s,
                h.labels.join(","),
                h.gates.join(","),
                h.plants.join(",")
            );
        }
    }
    0
}

struct ParsedLogSummary {
    scale: String,
    registries: String,
    exit: i64,
    ts: String,
    receipts: Vec<String>,
    gates: BTreeMap<String, (bool, u64)>,
    metrics: Vec<(String, JsonValue)>,
}

fn newest_log_for_script(logs_dir: &Path, script: &str) -> Option<PathBuf> {
    let dir = logs_dir.join(script);
    if !dir.is_dir() {
        return None;
    }
    if let Ok(name) = fs::read_to_string(dir.join("LATEST")) {
        let stem = name.trim();
        if !stem.is_empty() {
            let cand = dir.join(if stem.ends_with(".ndjson") {
                stem.into()
            } else {
                format!("{stem}.ndjson")
            });
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    let mut ndjsons: Vec<PathBuf> = fs::read_dir(&dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("ndjson"))
        .collect();
    ndjsons.sort();
    ndjsons.pop()
}

fn parse_log_for_report(path: &Path) -> Option<ParsedLogSummary> {
    let text = fs::read_to_string(path).ok()?;
    let (mut scale, mut registries, mut exit, mut ts, mut summary_count, mut corrupt) = (
        "full".into(),
        "of-record".into(),
        2i64,
        String::new(),
        0u32,
        false,
    );
    let (mut receipts, mut gates, mut metrics) = (Vec::new(), BTreeMap::new(), Vec::new());
    for line in text.lines().filter(|l| !l.is_empty()) {
        let Ok(val) = json::parse_strict(line) else {
            corrupt = true;
            break;
        };
        let (ev, step) = (
            val.get("event").and_then(JsonValue::as_str).unwrap_or(""),
            val.get("step").and_then(JsonValue::as_str).unwrap_or(""),
        );
        match ev {
            "start" if step.is_empty() => {
                if let Some(s) = val.get("scale").and_then(JsonValue::as_str) {
                    scale = s.into();
                }
            }
            "metric" => {
                if let Some(g) = val
                    .get("gate")
                    .and_then(JsonValue::as_str)
                    .filter(|s| !s.is_empty())
                {
                    let mut m = vec![
                        (
                            "name",
                            JsonValue::String(
                                val.get("name")
                                    .and_then(JsonValue::as_str)
                                    .unwrap_or("")
                                    .into(),
                            ),
                        ),
                        (
                            "value",
                            val.get("value").cloned().unwrap_or(JsonValue::Null),
                        ),
                        (
                            "unit",
                            JsonValue::String(
                                val.get("unit")
                                    .and_then(JsonValue::as_str)
                                    .unwrap_or("")
                                    .into(),
                            ),
                        ),
                    ];
                    if let Some(th) = val.get("threshold") {
                        m.push(("threshold", th.clone()));
                    }
                    if let Some(v) = val.get("verdict") {
                        m.push(("verdict", v.clone()));
                    }
                    metrics.push((g.into(), obj(m)));
                }
            }
            "summary" => {
                summary_count += 1;
                if let Some(r) = val.get("registries").and_then(JsonValue::as_str) {
                    registries = r.into();
                }
                exit = val.get("exit").and_then(JsonValue::as_i64).unwrap_or(2);
                ts = val
                    .get("ts")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("")
                    .into();
                if let Some(arr) = val.get("receipts").and_then(JsonValue::as_array) {
                    receipts.extend(
                        arr.iter()
                            .filter_map(JsonValue::as_str)
                            .filter(|s| !s.is_empty())
                            .map(str::to_string),
                    );
                }
                if let Some(arr) = val.get("gates").and_then(JsonValue::as_array) {
                    for g in arr {
                        if let Some(id) = g.get("id").and_then(JsonValue::as_str) {
                            gates.insert(
                                id.into(),
                                (
                                    g.get("measured_ok")
                                        .and_then(JsonValue::as_bool)
                                        .unwrap_or(false),
                                    g.get("controls_caught")
                                        .and_then(JsonValue::as_u64)
                                        .unwrap_or(0),
                                ),
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if corrupt || summary_count != 1 {
        exit = 2;
    }
    Some(ParsedLogSummary {
        scale,
        registries,
        exit,
        ts,
        receipts,
        gates,
        metrics,
    })
}

pub fn generate_gate_report(
    scripts_dir: &Path,
    logs_dir: &Path,
    out_dir: &Path,
) -> Result<JsonValue, String> {
    let (mut script_names, mut hdr_map) = (BTreeSet::new(), BTreeMap::new());
    for h in list_scripts(scripts_dir)? {
        script_names.insert(h.script.clone());
        hdr_map.insert(h.script.clone(), h);
    }
    if logs_dir.is_dir()
        && let Ok(rd) = fs::read_dir(logs_dir)
    {
        for e in rd.flatten().filter(|e| e.path().is_dir()) {
            let n = e.file_name().to_string_lossy().into_owned();
            if !n.starts_with('_') {
                script_names.insert(n);
            }
        }
    }
    let logs: BTreeMap<String, ParsedLogSummary> = script_names
        .iter()
        .filter_map(|n| {
            newest_log_for_script(logs_dir, n)
                .and_then(|p| parse_log_for_report(&p))
                .map(|l| (n.clone(), l))
        })
        .collect();

    #[derive(Default)]
    struct Agg {
        milestone: String,
        scripts: BTreeSet<String>,
        last_run: Option<String>,
        any_fail: bool,
        any_missing: bool,
        any_non_claimable: bool,
        cc: u64,
        receipts: BTreeSet<String>,
        metrics: Vec<JsonValue>,
    }
    let mut gates = BTreeMap::<String, Agg>::new();
    let mut touch = |gid: &str, sname: &str, ms: &str| {
        let e = gates.entry(gid.into()).or_default();
        if e.milestone.is_empty() && !ms.is_empty() {
            e.milestone = ms.into();
        }
        e.scripts.insert(sname.into());
    };
    for (s, h) in &hdr_map {
        for g in &h.gates {
            touch(g, s, &h.milestone);
        }
    }
    for (s, l) in &logs {
        let ms = hdr_map.get(s).map_or("", |h| h.milestone.as_str());
        for g in l.gates.keys() {
            touch(g, s, ms);
        }
    }

    let mut rows = Vec::new();
    let mut md = String::from(
        "# Gate Report (View Only — No Registry Written)\n\n| Gate | Milestone | Result | Controls Caught | Scripts | Receipts | Last Run |\n|---|---|---|---|---|---|---|\n",
    );
    for (gid, mut a) in gates {
        for s in &a.scripts {
            let Some(log) = logs.get(s) else {
                a.any_missing = true;
                continue;
            };
            if log.exit == 77 {
                a.any_missing = true;
                continue;
            }
            if !log.ts.is_empty() && a.last_run.as_ref().is_none_or(|p| &log.ts > p) {
                a.last_run = Some(log.ts.clone());
            }
            if log.scale != "full" || log.registries != "of-record" {
                a.any_non_claimable = true;
            }
            match log.gates.get(&gid) {
                Some(&(ok, cc)) => {
                    if !ok || log.exit != 0 {
                        a.any_fail = true;
                    }
                    a.cc += cc;
                }
                None if log.exit != 0 => a.any_fail = true,
                None => a.any_missing = true,
            }
            a.receipts.extend(log.receipts.iter().cloned());
            a.metrics.extend(
                log.metrics
                    .iter()
                    .filter(|(mg, _)| mg == &gid)
                    .map(|(_, m)| m.clone()),
            );
        }
        let res = if a.any_fail {
            "fail"
        } else if a.any_missing {
            "not-run"
        } else if a.any_non_claimable {
            "small-only"
        } else if a.cc == 0 {
            "no-control"
        } else {
            "pass"
        };
        let (sv, rv): (Vec<String>, Vec<String>) = (
            a.scripts.into_iter().collect(),
            a.receipts.into_iter().collect(),
        );
        md.push_str(&format!(
            "| {gid} | {} | {res} | {} | {} | {} | {} |\n",
            if a.milestone.is_empty() {
                "-"
            } else {
                &a.milestone
            },
            a.cc,
            sv.join(", "),
            if rv.is_empty() {
                "-".into()
            } else {
                rv.join(", ")
            },
            a.last_run.as_deref().unwrap_or("-")
        ));
        rows.push(obj(vec![
            ("id", JsonValue::String(gid)),
            ("milestone", JsonValue::String(a.milestone)),
            ("result", JsonValue::String(res.into())),
            ("scripts", str_array(&sv)),
            (
                "last_run",
                a.last_run.map_or(JsonValue::Null, JsonValue::String),
            ),
            ("controls_caught", JsonValue::Number(a.cc.to_string())),
            ("metrics", JsonValue::Array(a.metrics)),
            ("receipts", str_array(&rv)),
        ]));
    }
    let report = obj(vec![
        ("schema", JsonValue::String("frob.e2e.report/1".into())),
        ("gates", JsonValue::Array(rows)),
    ]);
    fs::create_dir_all(out_dir).map_err(|e| format!("create {}: {e}", out_dir.display()))?;
    fs::write(
        out_dir.join("gates.json"),
        format!("{}\n", report.to_json_pretty()),
    )
    .map_err(|e| format!("write gates.json: {e}"))?;
    fs::write(out_dir.join("gates.md"), md).map_err(|e| format!("write gates.md: {e}"))?;
    Ok(report)
}

pub fn cmd_report(args: &[String]) -> i32 {
    let f = match ParsedFlags::parse(
        args,
        &["--scripts-dir", "--logs-dir", "--out-dir"],
        &[],
        &[],
    ) {
        Ok(f) if f.positionals.is_empty() => f,
        Ok(_) => {
            eprintln!("frob-e2e report: unexpected positional argument");
            return 2;
        }
        Err(e) => {
            eprintln!("frob-e2e report: {e}");
            return 2;
        }
    };
    let out = f.get("--out-dir").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("target/e2e-report").join(format_utc_compact(now_utc_ms()))
    });
    match generate_gate_report(
        Path::new(f.get_or("--scripts-dir", "scripts/e2e")),
        Path::new(f.get_or("--logs-dir", "target/e2e-logs")),
        &out,
    ) {
        Ok(_) => {
            println!("{}", out.display());
            0
        }
        Err(e) => {
            eprintln!("frob-e2e report: {e}");
            2
        }
    }
}
