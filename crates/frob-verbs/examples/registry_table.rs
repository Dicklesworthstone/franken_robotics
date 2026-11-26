//! Emit the actual build-script output for the tether registry E2E check.

fn main() {
    print!(
        "{}",
        include_str!(concat!(env!("OUT_DIR"), "/verbs_table.rs"))
    );
}
