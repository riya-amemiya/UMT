/// Factorizes a number into its prime factors.
///
/// This function takes an integer as input and returns a vector containing its prime factors.
///
/// # Arguments
///
/// * `n` - The integer to factorize.
///
/// # Returns
///
/// A vector containing the prime factors of the absolute value of the input integer.
/// The result is empty for 0, 1 and -1.
pub fn umt_factorize(n: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut remaining = n.unsigned_abs();

    let mut factor = 2_u32;
    while factor
        .checked_mul(factor)
        .is_some_and(|square| square <= remaining)
    {
        while remaining.is_multiple_of(factor) {
            result.push(factor as i32);
            remaining /= factor;
        }
        factor += 1;
    }

    if remaining > 1 {
        result.push(remaining as i32);
    }

    result
}
