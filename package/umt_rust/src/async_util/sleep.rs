//! Blocks the current thread for the specified number of milliseconds.
//!
//! This is the synchronous std equivalent of the TypeScript `sleep`, which
//! returns a promise that resolves after the given delay. Here the current
//! thread is parked for the duration instead.

use std::thread;
use std::time::Duration;

/// Blocks the current thread for the specified number of milliseconds.
///
/// # Arguments
///
/// * `ms` - The number of milliseconds to sleep
///
/// # Returns
///
/// Returns after the delay has elapsed.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_sleep;
///
/// umt_sleep(0);
/// ```
#[inline]
pub fn umt_sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
