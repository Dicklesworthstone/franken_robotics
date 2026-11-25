// N33c-03 compile-fail test (expect E0308: mismatched types)
use frob_core::qty::{AngularMomentum, AngularDamping};
fn main() {
    let _ = AngularMomentum::new(1.0) + AngularDamping::new(1.0);
}
