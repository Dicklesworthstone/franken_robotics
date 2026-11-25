// N33c-03 compile-fail test (expect E0308: mismatched types)
use frob_core::qty::{Energy, AngularStiffness};
fn main() {
    let _ = Energy::new(1.0) + AngularStiffness::new(1.0);
}
