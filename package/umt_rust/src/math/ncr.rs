use super::{umt_factorial, umt_npr};

/// Calculates the number of combinations (nCr) of choosing r items from a set of n items.
///
/// This function takes two integers, n and r, as input and returns the number of combinations.
///
/// # Arguments
///
/// * `n` - The total number of items in the set.
/// * `r` - The number of items to choose.
///
/// # Returns
///
/// The number of combinations (nCr).
pub fn umt_ncr(n: i32, r: i32) -> i32 {
    if r < 0 || r > n {
        return 0;
    }
    if n > 12 && r > 12 {
        return 0;
    }
    umt_npr(n, r) / umt_factorial(r)
}
