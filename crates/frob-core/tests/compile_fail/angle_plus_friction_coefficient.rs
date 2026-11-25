// N33c-03 compile-fail test (expect E0308: mismatched types)
use frob_core::qty::{Angle, FrictionCoefficient};
fn main() {
    let _ = Angle::new(1.0) + FrictionCoefficient::new(1.0);
}
