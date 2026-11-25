// N33c-03 compile-fail test (expect E0308: mismatched types)
use frob_core::qty::{Pressure, Time};
fn main() {
    let _ = Pressure::new(1.0) + Time::new(1.0);
}
