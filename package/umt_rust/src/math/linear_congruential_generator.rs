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

    (multiplier * seed + increment) % max
}
