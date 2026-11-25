// N33c-03 compile-fail test (expect E0308: mismatched types)
use frob_core::qty::{Torque, Energy};
fn main() {
    let _ = Torque::new(1.0) + Energy::new(1.0);
}
