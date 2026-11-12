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
/// The number of permutations (nPr), or 0 for invalid arguments and for results that do not fit in an `i32`.
pub fn umt_npr(n: i32, r: i32) -> i32 {
    if r < 0 || r > n {
        return 0;
    }
    (0..r)
        .try_fold(1_i32, |product, index| product.checked_mul(n - index))
        .unwrap_or(0)
}
