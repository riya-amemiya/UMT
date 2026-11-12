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
/// The number of combinations (nCr), or 0 for invalid arguments and for results that do not fit in an `i32`.
pub fn umt_ncr(n: i32, r: i32) -> i32 {
    if r < 0 || r > n || n < 0 {
        return 0;
    }
    if r == 0 || n == r {
        return 1;
    }
    let n = i64::from(n);
    let effective_r = i64::from(r).min(n - i64::from(r));
    (0..effective_r)
        .try_fold(1_i64, |combinations, index| {
            combinations
                .checked_mul(n - index)
                .map(|product| product / (index + 1))
        })
        .and_then(|combinations| i32::try_from(combinations).ok())
        .unwrap_or(0)
}
