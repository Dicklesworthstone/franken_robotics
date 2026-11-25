//! Exact pooled ratio reduction and decimal rendering (`N39a-01` skeleton;
//! cells, pairs, pooled ratio, and censoring rule arrive in `N39a-27`).
//!
//! Lives inside the scorer boundary (`BOUNDARY_FILES`). Zero floats, zero
//! `unsafe`, determinism class D0.

use frob_cert::bigint::BigInt;
use frob_cert::rational::Rational;

use crate::tenfold::canon::{RoundingMode, render_rational_decimal};
use crate::tenfold::refusal::TenfoldRefusal;

/// Construct an exact reduced rational `numerator / denominator` using
/// `frob_cert::rational::Rational`.
pub fn exact_ratio(numerator: i64, denominator: i64) -> Result<Rational, TenfoldRefusal> {
    Rational::new(BigInt::from_i64(numerator), BigInt::from_i64(denominator)).map_err(|e| {
        TenfoldRefusal::noncanonical_number(
            "",
            format!("{numerator}/{denominator}"),
            "exact rational with non-zero denominator",
            vec![format!("Provide a valid non-zero denominator ({e})")],
        )
    })
}

/// Render an exact reduced ratio as `(exact_slash_string, decimal_string)`.
pub fn render_ratio(
    ratio: &Rational,
    places: usize,
    mode: RoundingMode,
) -> Result<(String, String), TenfoldRefusal> {
    let dec = render_rational_decimal(ratio, places, mode)?;
    Ok((ratio.to_slash_string(), dec))
}
