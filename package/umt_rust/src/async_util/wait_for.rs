//! Polls a condition until it produces a value or a timeout elapses.
//!
//! The TypeScript `waitFor` repeatedly evaluates a condition until it returns a
//! truthy value, resolving with that value, or rejecting once the timeout
//! elapses. The std equivalent maps "truthy" to `Some(value)` and lets the
//! condition surface its own error via the inner `Result`. The AbortSignal path
//! of the TypeScript version has no std equivalent and is omitted.

use std::thread;
use std::time::{Duration, Instant};

/// The error returned by [`umt_wait_for`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaitForError<E> {
    /// The condition did not produce a value before the timeout elapsed.
    TimedOut { ms: u64 },
    /// The condition itself produced an error.
    Condition(E),
}

impl<E: std::fmt::Display> std::fmt::Display for WaitForError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaitForError::TimedOut { ms } => write!(f, "waitFor timed out after {}ms", ms),
            WaitForError::Condition(error) => write!(f, "{}", error),
        }
    }
}

impl<E: std::fmt::Debug + std::fmt::Display> std::error::Error for WaitForError<E> {}

/// Options for [`umt_wait_for`], mirroring the TypeScript `WaitForOptions`.
#[derive(Debug, Clone, Copy)]
pub struct WaitForOptions {
    /// Maximum total time to poll, in milliseconds (default: 5000).
    pub timeout: u64,
    /// Delay between polls, in milliseconds (default: 100).
    pub interval: u64,
}

impl Default for WaitForOptions {
    fn default() -> Self {
        WaitForOptions {
            timeout: 5000,
            interval: 100,
        }
    }
}

/// Repeatedly evaluates a condition until it yields `Some(value)` or the timeout
/// elapses.
///
/// The condition is checked once before the first sleep, so a condition that is
/// immediately satisfied returns without any delay (matching the TypeScript
/// behavior of resolving immediately when the condition is initially truthy).
///
/// # Arguments
///
/// * `condition` - The predicate; returns `Ok(Some(value))` when satisfied,
///   `Ok(None)` to keep waiting, or `Err` to fail
/// * `options` - Polling configuration
///
/// # Returns
///
/// `Ok(value)` for the first satisfied poll, `Err(WaitForError::Condition)` when
/// the condition errors, or `Err(WaitForError::TimedOut)` when the timeout
/// elapses.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::{umt_wait_for, WaitForOptions};
///
/// let result = umt_wait_for(|| Ok::<Option<i32>, String>(Some(42)), WaitForOptions::default());
/// assert_eq!(result, Ok(42));
/// ```
///
/// ```
/// use umt_rust::async_util::{umt_wait_for, WaitForOptions};
///
/// let mut counter = 0;
/// let result = umt_wait_for(
///     || {
///         counter += 1;
///         Ok::<Option<&str>, String>(if counter >= 3 { Some("done") } else { None })
///     },
///     WaitForOptions { timeout: 500, interval: 1 },
/// );
/// assert_eq!(result, Ok("done"));
/// ```
pub fn umt_wait_for<T, E, C>(
    mut condition: C,
    options: WaitForOptions,
) -> Result<T, WaitForError<E>>
where
    C: FnMut() -> Result<Option<T>, E>,
{
    let WaitForOptions { timeout, interval } = options;
    let start = Instant::now();

    loop {
        match condition() {
            Ok(Some(value)) => return Ok(value),
            Ok(None) => {}
            Err(error) => return Err(WaitForError::Condition(error)),
        }
        if start.elapsed() >= Duration::from_millis(timeout) {
            return Err(WaitForError::TimedOut { ms: timeout });
        }
        thread::sleep(Duration::from_millis(interval));
    }
}
