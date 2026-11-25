// N33c-03 compile-fail test (expect E0308: mismatched types)
use frob_core::qty::{Frequency, AngularVelocity};
fn main() {
    let _ = Frequency::new(1.0) + AngularVelocity::new(1.0);
}
