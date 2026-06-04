//! Runs a collection of tasks and reports each one's settled outcome, with an
//! optional concurrency limit.
//!
//! Mirrors the TypeScript `pSettled`: every task runs to completion, results are
//! returned in input order, and each entry records whether the task fulfilled or
//! rejected. A limit of `0` (or `None`) means unlimited concurrency.

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// The settled outcome of a single task, mirroring the TypeScript
/// `SettledResult<T>` discriminated union.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettledResult<T, E> {
    /// The task fulfilled with a value.
    Fulfilled { value: T },
    /// The task rejected with a reason.
    Rejected { reason: E },
}

/// A single boxed task accepted by [`umt_p_settled`]. Using boxed trait objects
/// lets the input mix heterogeneous closures and closures that capture shared
/// references.
pub type SettledTask<'a, T, E> = Box<dyn FnOnce() -> Result<T, E> + Send + 'a>;

/// Runs each task to completion and returns their settled outcomes in input
/// order.
///
/// # Arguments
///
/// * `tasks` - The task closures; each returns `Result<T, E>`
/// * `limit` - Maximum concurrent in-flight tasks; `None` or `Some(0)` means
///   unlimited
///
/// # Returns
///
/// A vector of [`SettledResult`] in the same order as the input tasks.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::{umt_p_settled, SettledResult, SettledTask};
///
/// let tasks: Vec<SettledTask<i32, String>> = vec![
///     Box::new(|| Ok(1)),
///     Box::new(|| Err("nope".to_string())),
/// ];
/// let results = umt_p_settled(tasks, None);
/// assert_eq!(results[0], SettledResult::Fulfilled { value: 1 });
/// assert_eq!(results[1], SettledResult::Rejected { reason: "nope".to_string() });
/// ```
///
/// ```
/// use umt_rust::async_util::{umt_p_settled, SettledTask};
///
/// let tasks: Vec<SettledTask<i32, String>> = vec![];
/// let results = umt_p_settled(tasks, None);
/// assert!(results.is_empty());
/// ```
pub fn umt_p_settled<T, E>(
    tasks: Vec<SettledTask<T, E>>,
    limit: Option<usize>,
) -> Vec<SettledResult<T, E>>
where
    T: Send,
    E: Send,
{
    let len = tasks.len();
    if len == 0 {
        return Vec::new();
    }

    let effective_limit = match limit {
        Some(value) if value > 0 => value,
        _ => len,
    };
    let worker_count = effective_limit.min(len).max(1);

    let results: Vec<Mutex<Option<SettledResult<T, E>>>> =
        (0..len).map(|_| Mutex::new(None)).collect();
    let tasks: Vec<Mutex<Option<SettledTask<T, E>>>> = tasks
        .into_iter()
        .map(|task| Mutex::new(Some(task)))
        .collect();
    let next_index = AtomicUsize::new(0);

    thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| {
                loop {
                    let current = next_index.fetch_add(1, Ordering::SeqCst);
                    if current >= len {
                        return;
                    }
                    let task = tasks[current].lock().unwrap().take();
                    if let Some(task) = task {
                        let settled = match task() {
                            Ok(value) => SettledResult::Fulfilled { value },
                            Err(reason) => SettledResult::Rejected { reason },
                        };
                        *results[current].lock().unwrap() = Some(settled);
                    }
                }
            });
        }
    });

    results
        .into_iter()
        .map(|cell| {
            cell.into_inner()
                .unwrap()
                .expect("every task produces a settled result")
        })
        .collect()
}
