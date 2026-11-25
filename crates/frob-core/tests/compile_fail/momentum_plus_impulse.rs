// N33c-03 compile-fail test (expect E0308: mismatched types)
use frob_core::qty::{Momentum, Impulse};
fn main() {
    let _ = Momentum::new(1.0) + Impulse::new(1.0);
}
