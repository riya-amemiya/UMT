use rand::RngExt;

/// Shuffles all elements in a 2D array while maintaining the row lengths.
///
/// # Arguments
///
/// * `array` - The 2D array to shuffle
///
/// # Returns
///
/// A new 2D vector with shuffled elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_shuffle_2d_array;
///
/// let arr = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
/// let shuffled = umt_shuffle_2d_array(&arr);
/// // Row lengths are preserved, but elements are shuffled across the entire array
/// assert_eq!(shuffled[0].len(), 2);
/// assert_eq!(shuffled[1].len(), 2);
/// assert_eq!(shuffled[2].len(), 2);
/// ```
pub fn umt_shuffle_2d_array<T: Clone>(array: &[Vec<T>]) -> Vec<Vec<T>> {
    if array.is_empty() {
        return vec![];
    }

    // Flatten the 2D array into 1D
    let mut flat_array: Vec<T> = Vec::new();
    for sub_array in array {
        flat_array.extend(sub_array.iter().cloned());
    }

    // Shuffle the flat array using Fisher-Yates
    let mut rng = rand::rng();
    for i in (1..flat_array.len()).rev() {
        let j = rng.random_range(0..=i);
        flat_array.swap(i, j);
    }

    // Reconstruct the 2D array with the same row lengths
    let mut result = Vec::new();
    let mut row_index = 0;

    for sub_array in array {
        let row_len = sub_array.len();
        let new_row = flat_array[row_index..row_index + row_len].to_vec();
        result.push(new_row);
        row_index += row_len;
    }

    result
}
