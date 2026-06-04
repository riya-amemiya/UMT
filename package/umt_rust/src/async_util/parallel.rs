//! Runs an operation over a slice of items with a bounded number of worker
//! threads, preserving input order in the results.
//!
//! Mirrors the TypeScript `parallel`: at most `limit` operations run at once,
//! results are returned in input order, and the first error aborts with that
//! error.

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// Runs `function_` over each item with at most `limit` concurrent worker
/// threads, returning the results in input order.
///
/// # Arguments
///
/// * `limit` - Maximum number of concurrent worker threads (clamped to at least
///   1 when items are present)
/// * `items` - The items to process
/// * `function_` - The operation applied to each `(item, index)`; returns
///   `Result<U, E>`
///
/// # Returns
///
/// `Ok(results)` in the same order as the input, or `Err` with the first error
/// encountered.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_parallel;
///
/// let items = vec![1, 2, 3, 4, 5];
/// let results = umt_parallel(2, &items, |n, _| Ok::<i32, String>(n * 10));
/// assert_eq!(results, Ok(vec![10, 20, 30, 40, 50]));
/// ```
///
/// ```
/// use umt_rust::async_util::umt_parallel;
///
/// let items: Vec<i32> = vec![];
/// let results = umt_parallel(2, &items, |n, _| Ok::<i32, String>(n * 2));
/// assert_eq!(results, Ok(vec![]));
/// ```
pub fn umt_parallel<T, U, E, F>(limit: usize, items: &[T], function_: F) -> Result<Vec<U>, E>
where
    T: Sync,
    U: Send,
    E: Send,
    F: Fn(&T, usize) -> Result<U, E> + Sync,
{
    let len = items.len();
    if len == 0 {
        return Ok(Vec::new());
    }

    let results: Vec<Mutex<Option<U>>> = (0..len).map(|_| Mutex::new(None)).collect();
    let error: Mutex<Option<E>> = Mutex::new(None);
    let next_index = AtomicUsize::new(0);
    let worker_count = limit.min(len).max(1);

    thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| {
                loop {
                    if error.lock().unwrap().is_some() {
                        return;
                    }
                    let current = next_index.fetch_add(1, Ordering::SeqCst);
                    if current >= len {
                        return;
                    }
                    match function_(&items[current], current) {
                        Ok(value) => {
                            *results[current].lock().unwrap() = Some(value);
                        }
                        Err(error_value) => {
                            let mut guard = error.lock().unwrap();
                            if guard.is_none() {
                                *guard = Some(error_value);
                            }
                            return;
                        }
                    }
                }
            });
        }
    });

    if let Some(error_value) = error.into_inner().unwrap() {
        return Err(error_value);
    }

    Ok(results
        .into_iter()
        .map(|cell| {
            cell.into_inner()
                .unwrap()
                .expect("every slot is filled when no error occurs")
        })
        .collect())
}
