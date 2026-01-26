use super::umt_ncr;

/// Calculates combinations with repetition (nHr).
///
/// This calculates the number of ways to choose r items from n items with repetition allowed.
/// Also known as "stars and bars" problem or "multichoose".
/// Uses the formula nHr = (n+r-1)Cr
///
/// # Arguments
///
/// * `n` - Total number of items.
/// * `r` - Number of items to choose.
///
/// # Returns
///
/// Number of combinations with repetition, or -1 for invalid arguments.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_nhr;
///
/// assert_eq!(umt_nhr(5, 2), 15);
/// ```
pub fn umt_nhr(n: i32, r: i32) -> i32 {
    if n == 0 || r == 0 || n < 0 || r < 0 {
        return -1;
    }

    umt_ncr(n + r - 1, r)
}
