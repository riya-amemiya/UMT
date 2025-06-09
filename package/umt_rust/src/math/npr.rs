use super::umt_factorial;

/// Calculates the number of permutations (nPr) of choosing r items from a set of n items.
///
/// This function takes two integers, n and r, as input and returns the number of permutations.
///
/// # Arguments
///
/// * `n` - The total number of items in the set.
/// * `r` - The number of items to choose.
///
/// # Returns
///
/// The number of permutations (nPr).
pub fn umt_npr(n: i32, r: i32) -> i32 {
    if r < 0 || r > n {
        return 0;
    }
    if n > 12 {
        return 0;
    }
    umt_factorial(n) / umt_factorial(n - r)
}
