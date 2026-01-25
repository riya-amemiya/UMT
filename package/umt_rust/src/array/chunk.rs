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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_splits_array() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let expected = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7]];
        assert_eq!(umt_chunk(&input, 3), expected);
    }

    #[test]
    fn test_chunk_empty_array() {
        let input: Vec<i32> = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(umt_chunk(&input, 3), expected);
    }

    #[test]
    fn test_chunk_larger_than_array() {
        let input = vec![0, 1, 2];
        let expected = vec![vec![0, 1, 2]];
        assert_eq!(umt_chunk(&input, 5), expected);
    }

    #[test]
    fn test_chunk_size_one() {
        let input = vec![1, 2, 3, 4, 5];
        let expected = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
        assert_eq!(umt_chunk(&input, 1), expected);
    }

    #[test]
    fn test_chunk_strings() {
        let input = vec!["a", "b", "c", "d", "e", "f", "g"];
        let expected = vec![vec!["a", "b", "c"], vec!["d", "e", "f"], vec!["g"]];
        assert_eq!(umt_chunk(&input, 3), expected);
    }

    #[test]
    fn test_chunk_perfect_division() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(umt_chunk(&input, 3), expected);
    }

    #[test]
    fn test_chunk_equal_to_size() {
        let input = vec![1, 2, 3];
        let expected = vec![vec![1, 2, 3]];
        assert_eq!(umt_chunk(&input, 3), expected);
    }

    #[test]
    fn test_chunk_single_element() {
        let input = vec![42];
        let expected = vec![vec![42]];
        assert_eq!(umt_chunk(&input, 1), expected);
    }

    #[test]
    fn test_chunk_zero_size() {
        let input = vec![1, 2, 3];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(umt_chunk(&input, 0), expected);
    }

    #[test]
    fn test_chunk_does_not_mutate_input() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let _ = umt_chunk(&input, 3);
        assert_eq!(input, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }
}
