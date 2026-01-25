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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nhr_basic() {
        assert_eq!(umt_nhr(5, 2), 15);
    }

    #[test]
    fn test_nhr_equal() {
        assert_eq!(umt_nhr(3, 3), 10);
    }

    #[test]
    fn test_nhr_invalid() {
        assert_eq!(umt_nhr(0, 2), -1);
        assert_eq!(umt_nhr(5, 0), -1);
        assert_eq!(umt_nhr(-1, 2), -1);
        assert_eq!(umt_nhr(5, -1), -1);
    }

    #[test]
    fn test_nhr_one() {
        assert_eq!(umt_nhr(5, 1), 5);
    }
}
