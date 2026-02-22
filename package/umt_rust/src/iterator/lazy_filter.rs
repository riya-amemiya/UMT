/// Lazily filters values from an iterable using a generator
///
/// # Arguments
///
/// * `iterable` - The source iterable
/// * `predicate` - The filter predicate, which receives a reference to the value and the index
///
/// # Returns
///
/// An iterator yielding filtered values
///
/// # Example
///
/// ```
/// use umt_rust::iterator::lazy_filter;
/// let evens: Vec<i32> = lazy_filter(vec![1, 2, 3, 4], |n, _| n % 2 == 0).collect();
/// assert_eq!(evens, vec![2, 4]);
/// ```
pub fn lazy_filter<I, F>(iterable: I, mut predicate: F) -> impl Iterator<Item = I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item, usize) -> bool,
{
    iterable.into_iter().enumerate().filter_map(
        move |(i, v)| {
            if predicate(&v, i) { Some(v) } else { None }
        },
    )
}
