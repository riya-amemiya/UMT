/// Splits an array into two vectors based on a predicate.
///
/// # Arguments
///
/// * `array` - The array to partition
/// * `predicate` - Function returning true for elements that go into the first vector
///
/// # Returns
///
/// A tuple of (pass, fail) vectors
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_partition;
///
/// let (pass, fail) = umt_partition(&[1, 2, 3, 4], |n| n % 2 == 0);
/// assert_eq!(pass, vec![2, 4]);
/// assert_eq!(fail, vec![1, 3]);
/// ```
pub fn umt_partition<T, F>(array: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    umt_partition_indexed(array, |item, _| predicate(item))
}

/// Splits an array into two vectors with index access in the predicate.
///
/// # Arguments
///
/// * `array` - The array to partition
/// * `predicate` - Function returning true for elements that go into the first vector
///   (receives value and index)
///
/// # Returns
///
/// A tuple of (pass, fail) vectors
pub fn umt_partition_indexed<T, F>(array: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T, usize) -> bool,
{
    let mut pass = Vec::new();
    let mut fail = Vec::new();

    for (index, item) in array.iter().enumerate() {
        if predicate(item, index) {
            pass.push(item.clone());
        } else {
            fail.push(item.clone());
        }
    }

    (pass, fail)
}
