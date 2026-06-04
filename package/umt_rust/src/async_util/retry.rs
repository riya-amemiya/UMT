//! Retries an operation until it succeeds or the retry budget is exhausted.
//!
//! Faithfully mirrors the TypeScript `retry`: `retries` defaults to 3, `delay`
//! to 1000ms, with fixed/linear/exponential backoff, optional jitter, a
//! `should_retry` predicate, and an `on_retry` callback. The AbortSignal path of
//! the TypeScript version has no std equivalent and is omitted.

use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::math::umt_xoshiro256_01;

/// Backoff strategy for [`umt_retry`], matching the TypeScript options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backoff {
    /// Constant delay between attempts.
    Fixed,
    /// Delay grows linearly with the attempt number.
    Linear,
    /// Delay doubles with each attempt.
    Exponential,
}

/// Configuration options for [`umt_retry`].
pub struct RetryAsyncOptions<S, R> {
    /// Maximum number of retries after the first attempt (default: 3).
    pub retries: u32,
    /// Base delay between attempts in milliseconds (default: 1000).
    pub delay: u64,
    /// Backoff strategy (default: [`Backoff::Fixed`]).
    pub backoff: Backoff,
    /// Whether to multiply the delay by a random factor (default: false).
    pub jitter: bool,
    /// Predicate deciding whether an error should trigger a retry. Receives the
    /// error and the zero-based attempt number (default: always retry).
    pub should_retry: S,
    /// Optional callback invoked before each retry with the error and the
    /// zero-based attempt number.
    pub on_retry: Option<R>,
}

impl<E> Default for RetryAsyncOptions<fn(&E, u32) -> bool, fn(&E, u32)> {
    fn default() -> Self {
        RetryAsyncOptions {
            retries: 3,
            delay: 1000,
            backoff: Backoff::Fixed,
            jitter: false,
            should_retry: |_, _| true,
            on_retry: None,
        }
    }
}

fn compute_delay(base_delay: u64, attempt_number: u32, backoff: Backoff, jitter: bool) -> u64 {
    let mut wait_ms = base_delay as f64;
    match backoff {
        Backoff::Linear => wait_ms = base_delay as f64 * attempt_number as f64,
        Backoff::Exponential => {
            wait_ms = base_delay as f64 * 2f64.powi((attempt_number - 1) as i32)
        }
        Backoff::Fixed => {}
    }
    if jitter {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1);
        let mut state = [
            (nanos as u32) | 1,
            ((nanos >> 16) as u32) | 1,
            ((nanos >> 32) as u32) | 1,
            ((nanos >> 48) as u32)
                .wrapping_add(attempt_number)
                .wrapping_add(1),
        ];
        wait_ms *= umt_xoshiro256_01(&mut state);
    }
    wait_ms as u64
}

/// Retries an operation until it succeeds or the retry budget is exhausted.
///
/// # Arguments
///
/// * `operation` - The operation to invoke; returns `Result<T, E>`
/// * `options` - Retry configuration
///
/// # Returns
///
/// `Ok(value)` from the first successful invocation, or the last `Err` when the
/// budget is exhausted or `should_retry` returns false.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::{umt_retry, Backoff, RetryAsyncOptions};
///
/// let mut attempts = 0;
/// let result = umt_retry(
///     || {
///         attempts += 1;
///         if attempts < 3 {
///             Err("nope")
///         } else {
///             Ok("ok")
///         }
///     },
///     RetryAsyncOptions {
///         retries: 3,
///         delay: 1,
///         backoff: Backoff::Fixed,
///         jitter: false,
///         should_retry: |_: &&str, _| true,
///         on_retry: None::<fn(&&str, u32)>,
///     },
/// );
/// assert_eq!(result, Ok("ok"));
/// assert_eq!(attempts, 3);
/// ```
pub fn umt_retry<T, E, O, S, R>(mut operation: O, options: RetryAsyncOptions<S, R>) -> Result<T, E>
where
    O: FnMut() -> Result<T, E>,
    S: Fn(&E, u32) -> bool,
    R: Fn(&E, u32),
{
    let RetryAsyncOptions {
        retries,
        delay,
        backoff,
        jitter,
        should_retry,
        on_retry,
    } = options;

    let mut attempt_number: u32 = 0;
    loop {
        match operation() {
            Ok(value) => return Ok(value),
            Err(error) => {
                let remaining = retries as i64 - attempt_number as i64;
                if remaining <= 0 || !should_retry(&error, attempt_number) {
                    return Err(error);
                }
                if let Some(ref callback) = on_retry {
                    callback(&error, attempt_number);
                }
                thread::sleep(Duration::from_millis(compute_delay(
                    delay,
                    attempt_number + 1,
                    backoff,
                    jitter,
                )));
                attempt_number += 1;
            }
        }
    }
}
