/// Generates a vector containing a range of numbers.
///
/// This function takes a minimum and maximum value as input and returns a vector
/// containing all numbers within that range (inclusive of the minimum, exclusive of the maximum).
///
/// # Arguments
///
/// * `min` - The minimum value of the range (inclusive).
/// * `max` - The maximum value of the range (exclusive).
///
/// # Returns
///
/// A vector containing the numbers in the specified range.
pub fn umt_range(min: i32, max: i32) -> Vec<i32> {
    (min..max).collect()
}
