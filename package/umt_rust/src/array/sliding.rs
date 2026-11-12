/// Returns overlapping windows of a fixed size from an array.
/// Incomplete trailing windows are omitted.
///
/// # Arguments
///
/// * `array` - The array to window
/// * `size` - Window size
/// * `step` - Step between window starts
///
/// # Returns
///
/// Vector of windows, or an empty vector when `step` is 0
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_sliding;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// assert_eq!(umt_sliding(&arr, 3, 1), vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
/// assert_eq!(umt_sliding(&arr, 3, 2), vec![vec![1, 2, 3], vec![3, 4, 5]]);
/// ```
pub fn umt_sliding<T: Clone>(array: &[T], size: usize, step: usize) -> Vec<Vec<T>> {
    if step == 0 || size > array.len() {
        return vec![];
    }

    (0..=array.len() - size)
        .step_by(step)
        .map(|index| array[index..index + size].to_vec())
        .collect()
}
