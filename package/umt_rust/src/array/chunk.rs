/// Split an array into smaller chunks of specified size.
///
/// # Arguments
///
/// * `array` - The array to split
/// * `chunk_size` - The size of each chunk
///
/// # Returns
///
/// Vector of chunks
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_chunk;
///
/// let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
/// assert_eq!(umt_chunk(&arr, 3), vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
/// ```
pub fn umt_chunk<T: Clone>(array: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    if chunk_size == 0 {
        return vec![];
    }

    array.chunks(chunk_size).map(|c| c.to_vec()).collect()
}
