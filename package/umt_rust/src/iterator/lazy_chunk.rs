/// Lazily groups values from an iterable into vectors of the given size
///
/// The last chunk holds the remaining values and may be shorter than `size`.
///
/// # Arguments
///
/// * `iterable` - The source iterable
/// * `size` - The number of values in each chunk
///
/// # Returns
///
/// An iterator yielding chunks
///
/// # Example
///
/// ```
/// use umt_rust::iterator::lazy_chunk;
/// let chunks: Vec<Vec<i32>> = lazy_chunk(vec![1, 2, 3, 4, 5], 2).collect();
/// assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
/// ```
pub fn lazy_chunk<I>(iterable: I, size: usize) -> impl Iterator<Item = Vec<I::Item>>
where
    I: IntoIterator,
{
    let mut source = iterable.into_iter();
    std::iter::from_fn(move || {
        let mut chunk = Vec::new();
        for value in source.by_ref() {
            chunk.push(value);
            if chunk.len() == size {
                return Some(chunk);
            }
        }
        if chunk.is_empty() { None } else { Some(chunk) }
    })
    .fuse()
}
