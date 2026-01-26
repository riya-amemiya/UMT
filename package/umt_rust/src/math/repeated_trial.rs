use super::{umt_gcd, umt_ncr};

/// Probability fraction for repeated trials.
#[derive(Debug, Clone, Copy)]
pub struct Probability {
    pub x: f64,
    pub y: f64,
}

/// Calculates probability in repeated trials.
///
/// # Arguments
///
/// * `n` - Number of trials.
/// * `r` - Number of successes.
/// * `p` - Probability fraction (x/y).
///
/// # Returns
///
/// A tuple containing (numerator, denominator) of the result.
///
/// # Examples
///
/// ```
/// use umt_rust::math::{umt_repeated_trial, Probability};
///
/// let (num, den) = umt_repeated_trial(5, 2, Probability { x: 1.0, y: 3.0 });
/// // The probability of getting exactly 2 successes in 5 trials
/// // where P(success) = 1/3
/// ```
pub fn umt_repeated_trial(n: i32, r: i32, p: Probability) -> (i64, i64) {
    let x = umt_ncr(n, r) as f64;
    let answer1 = x * p.x.powi(r) * (p.y - p.x).powi(n - r);
    let answer2 = p.y.powi(r) * p.y.powi(n - r);

    let a1 = answer1.round() as i64;
    let a2 = answer2.round() as i64;

    if a2 == 0 {
        return (a1, a2);
    }

    let gcd = umt_gcd(a1.abs() as i32, a2.abs() as i32) as i64;
    (a1 / gcd, a2 / gcd)
}
