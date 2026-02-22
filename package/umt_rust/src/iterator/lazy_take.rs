/// Lazily takes the first n values from an iterable
///
/// # Arguments
///
/// * `iterable` - The source iterable
/// * `n` - The number of values to take
///
/// # Returns
///
/// An iterator yielding at most n values
///
/// # Example
///
/// ```
/// use umt_rust::iterator::lazy_take;
/// let first3: Vec<i32> = lazy_take(vec![1, 2, 3, 4, 5], 3).collect();
/// assert_eq!(first3, vec![1, 2, 3]);
/// ```
pub fn lazy_take<I>(iterable: I, n: usize) -> impl Iterator<Item = I::Item>
where
    I: IntoIterator,
{
    iterable.into_iter().take(n)
}
