/// Generates a pseudo-random number using a linear congruential generator (LCG).
///
/// This function takes a seed value, a maximum value, a multiplier, and an increment as input and returns a pseudo-random number.
///
/// # Arguments
///
/// * `seed` - The seed value.
/// * `max` - The maximum value (optional).
/// * `multiplier` - The multiplier (optional).
/// * `increment` - The increment (optional).
///
/// # Returns
///
/// A pseudo-random number.
pub fn umt_linear_congruential_generator<M, L, I>(
    seed: i32,
    max: M,
    multiplier: L,
    increment: I,
) -> i32
where
    M: Into<Option<i32>>,
    L: Into<Option<i32>>,
    I: Into<Option<i32>>,
{
    let max = max.into().unwrap_or(2_147_483_647);
    let multiplier = multiplier.into().unwrap_or(1_664_525);
    let increment = increment.into().unwrap_or(1_013_904_223);

    (multiplier.wrapping_mul(seed).wrapping_add(increment)) % max
}
