//! Runs an operation with a timeout, failing if it does not finish in time.
//!
//! The TypeScript `timeout` wraps a promise and rejects with
//! `Timed out after <ms>ms` when it does not settle within the limit. The std
//! equivalent runs the operation on a worker thread and waits on a channel with
//! [`Receiver::recv_timeout`](std::sync::mpsc::Receiver::recv_timeout).

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// The error returned by [`umt_timeout`] when the operation does not finish.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeoutError<E> {
    /// The operation exceeded the allotted time.
    TimedOut { ms: u64 },
    /// The operation finished but produced its own error.
    Operation(E),
}

impl<E: std::fmt::Display> std::fmt::Display for TimeoutError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeoutError::TimedOut { ms } => write!(f, "Timed out after {}ms", ms),
            TimeoutError::Operation(error) => write!(f, "{}", error),
        }
    }
}

impl<E: std::fmt::Debug + std::fmt::Display> std::error::Error for TimeoutError<E> {}

/// Runs an operation on a worker thread, failing if it does not finish within
/// the specified number of milliseconds.
///
/// # Arguments
///
/// * `operation` - The work to run; it returns `Result<T, E>`
/// * `ms` - The timeout in milliseconds
///
/// # Returns
///
/// `Ok(value)` when the operation resolves in time, `Err(TimeoutError::Operation)`
/// when it fails in time, and `Err(TimeoutError::TimedOut)` when it exceeds the
/// limit.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_timeout;
///
/// let result = umt_timeout(|| Ok::<i32, String>(42), 1000);
/// assert_eq!(result, Ok(42));
/// ```
///
/// ```
/// use umt_rust::async_util::{umt_timeout, TimeoutError};
///
/// let result = umt_timeout(
///     || {
///         std::thread::sleep(std::time::Duration::from_millis(50));
///         Ok::<i32, String>(1)
///     },
///     5,
/// );
/// assert_eq!(result, Err(TimeoutError::TimedOut { ms: 5 }));
/// ```
#[inline]
pub fn umt_timeout<T, E, O>(operation: O, ms: u64) -> Result<T, TimeoutError<E>>
where
    T: Send + 'static,
    E: Send + 'static,
    O: FnOnce() -> Result<T, E> + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let _ = sender.send(operation());
    });

    match receiver.recv_timeout(Duration::from_millis(ms)) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => Err(TimeoutError::Operation(error)),
        Err(_) => Err(TimeoutError::TimedOut { ms }),
    }
}
