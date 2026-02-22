/// Lazily maps values from an iterable using a generator
///
/// # Arguments
///
/// * `iterable` - The source iterable
/// * `f` - The mapping function, which receives the value and the index
///
/// # Returns
///
/// An iterator yielding mapped values
///
/// # Example
///
/// ```
/// use umt_rust::iterator::lazy_map;
/// let doubled: Vec<i32> = lazy_map(vec![1, 2, 3], |n, _| n * 2).collect();
/// assert_eq!(doubled, vec![2, 4, 6]);
/// ```
pub fn lazy_map<I, F, U>(iterable: I, mut f: F) -> impl Iterator<Item = U>
where
    I: IntoIterator,
    F: FnMut(I::Item, usize) -> U,
{
    iterable.into_iter().enumerate().map(move |(i, v)| f(v, i))
}
