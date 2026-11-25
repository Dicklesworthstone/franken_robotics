use frob_kir::ir::Program;
use std::fmt::Write as _;
use std::{env, fs, process::Command, time::SystemTime};

pub fn audit(source: &str) -> Result<(), String> {
    for line in source.lines().map(str::trim) {
        if ["let v", "const f32 v", "const u32 v"]
            .iter()
            .any(|p| line.starts_with(p))
            && let Some((_, rhs)) = line.split_once('=')
            && rhs.bytes().filter(|c| b"+-*/".contains(c)).count() > 1
        {
            return Err(format!("multiple arithmetic operations: {line}"));
        }
        if line.starts_with("//") || line.starts_with("/*") {
            continue;
        }
        for token in line.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.')) {
            let number = token.trim_start_matches('.');
            if number.starts_with(|c: char| c.is_ascii_digit())
                && !number.starts_with("0x")
                && (token.contains(['.', 'e', 'E'])
                    || token.ends_with("f32")
                    || token.ends_with("f64"))
            {
                return Err(format!("decimal float literal: {token}"));
            }
        }
        if line.contains("mul_add(") {
            return Err(format!("fused arithmetic: {line}"));
        }
    }
    Ok(())
}

// Host C++ runs the actual emitted body with the scalar prelude operations
// used here. Device execution and compiler flags are qualified separately.
pub fn assert_dialect_outputs(program: &Program, dialect: &str, cases: &[[f32; 2]]) {
    let mut cpp = String::from(
        r#"
#include <bit>
#include <cstdint>
#include <cstdio>
using f32 = float;
using u32 = std::uint32_t;
#define FROB_FN static inline
#define FROB_PTR(T) T*
#define FROB_SELECT(p, a, b) ((p) ? (a) : (b))
#define FROB_F32_BITS(b) std::bit_cast<f32>(u32(b))
"#,
    );
    cpp.push_str(dialect);
    cpp.push_str("int main() { f32 mn, mx, mag;\n");
    let mut expected = String::new();
    for inputs in cases {
        let [a, b] = inputs.map(f32::to_bits);
        writeln!(cpp, "minmax_body(FROB_F32_BITS(0x{a:08x}u), FROB_F32_BITS(0x{b:08x}u), &mn, &mx, &mag); std::printf(\"%08x %08x %08x\\n\", std::bit_cast<u32>(mn), std::bit_cast<u32>(mx), std::bit_cast<u32>(mag));").unwrap();
        let got = program.eval_f32(inputs).unwrap();
        writeln!(
            expected,
            "{:08x} {:08x} {:08x}",
            got[0].to_bits(),
            got[1].to_bits(),
            got[2].to_bits()
        )
        .unwrap();
    }
    cpp.push_str("}\n");
    let stamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let scratch = env::temp_dir().join(format!("frob-kir-dialect-{}-{stamp}", std::process::id()));
    fs::create_dir(&scratch).unwrap();
    let source = scratch.join("kernel.cpp");
    let binary = scratch.join("kernel");
    fs::write(&source, cpp).unwrap();
    let child = Command::new("c++")
        .args(["-std=c++20", "-fno-fast-math", "-ffp-contract=off"])
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .output()
        .expect("compile host dialect body");
    assert!(
        child.status.success(),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    // The compiler just created this executable in the test's exclusive directory.
    let child = Command::new(binary) // ubs:ignore — test-owned compiler output
        .output()
        .expect("run host dialect body");
    assert!(
        child.status.success(),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    assert_eq!(String::from_utf8(child.stdout).unwrap(), expected);
}
