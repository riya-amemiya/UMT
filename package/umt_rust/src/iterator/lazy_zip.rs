/// Lazily combines values from two iterables at corresponding positions
///
/// Stops as soon as either iterable is exhausted.
///
/// # Arguments
///
/// * `first` - The first source iterable
/// * `second` - The second source iterable
///
/// # Returns
///
/// An iterator yielding one pair per position
///
/// # Example
///
/// ```
/// use umt_rust::iterator::lazy_zip;
/// let pairs: Vec<(i32, char)> = lazy_zip(vec![1, 2, 3], vec!['a', 'b']).collect();
/// assert_eq!(pairs, vec![(1, 'a'), (2, 'b')]);
/// ```
pub fn lazy_zip<A, B>(first: A, second: B) -> impl Iterator<Item = (A::Item, B::Item)>
where
    A: IntoIterator,
    B: IntoIterator,
{
    first.into_iter().zip(second)
}

/// Lazily combines values from several iterables of the same type at corresponding positions
///
/// Stops as soon as any iterable is exhausted and yields nothing for no iterables.
///
/// # Arguments
///
/// * `iterables` - The source iterables
///
/// # Returns
///
/// An iterator yielding one vector per position
///
/// # Example
///
/// ```
/// use umt_rust::iterator::lazy_zip_many;
/// let rows: Vec<Vec<i32>> = lazy_zip_many(vec![vec![1, 2, 3], vec![4, 5], vec![6, 7]]).collect();
/// assert_eq!(rows, vec![vec![1, 4, 6], vec![2, 5, 7]]);
/// ```
pub fn lazy_zip_many<I>(iterables: Vec<I>) -> impl Iterator<Item = Vec<I::Item>>
where
    I: IntoIterator,
{
    let mut sources: Vec<I::IntoIter> =
        iterables.into_iter().map(IntoIterator::into_iter).collect();
    std::iter::from_fn(move || {
        if sources.is_empty() {
            return None;
        }
        sources.iter_mut().map(Iterator::next).collect()
    })
    .fuse()
}
