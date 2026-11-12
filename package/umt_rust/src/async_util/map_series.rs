//! Maps an operation over a slice of items strictly one at a time.
//!
//! Mirrors the TypeScript `mapSeries`: each item is processed only after the
//! previous one has finished, results are returned in input order, and the
//! first error stops the iteration with that error.

/// Runs `function_` over each item sequentially, returning the results in input
/// order.
///
/// # Arguments
///
/// * `items` - The items to process
/// * `function_` - The operation applied to each `(item, index)`; returns
///   `Result<U, E>`
///
/// # Returns
///
/// `Ok(results)` in the same order as the input, or `Err` with the first error
/// encountered. Items after a failing one are not processed.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_map_series;
///
/// let items = vec![1, 2, 3];
/// let results = umt_map_series(&items, |n, _| Ok::<i32, String>(n * 2));
/// assert_eq!(results, Ok(vec![2, 4, 6]));
/// ```
///
/// ```
/// use umt_rust::async_util::umt_map_series;
///
/// let items = vec!["a", "b"];
/// let results = umt_map_series(&items, |item, index| {
///     Ok::<String, String>(format!("{}-{}", item, index))
/// });
/// assert_eq!(results, Ok(vec!["a-0".to_string(), "b-1".to_string()]));
/// ```
pub fn umt_map_series<T, U, E, F>(items: &[T], mut function_: F) -> Result<Vec<U>, E>
where
    F: FnMut(&T, usize) -> Result<U, E>,
{
    let mut results = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        results.push(function_(item, index)?);
    }
    Ok(results)
}
