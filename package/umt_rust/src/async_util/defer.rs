//! A deferred value whose resolution can be triggered externally.
//!
//! The TypeScript `defer` returns an object exposing `promise`, `resolve`, and
//! `reject`. The std equivalent here is a [`Deferred`] backed by a one-shot
//! channel: `resolve`/`reject` send the settled value (possibly from another
//! thread) and `wait` blocks until it arrives.

use std::sync::mpsc::{self, Receiver, Sender};

/// A deferred value with externally accessible resolve and reject.
///
/// `resolve` and `reject` settle the deferred exactly once; the first call
/// wins. `wait` blocks the current thread until the deferred is settled and
/// returns `Ok` for a resolve or `Err` for a reject.
pub struct Deferred<T, E> {
    sender: Sender<Result<T, E>>,
    receiver: Receiver<Result<T, E>>,
}

impl<T, E> Deferred<T, E> {
    /// Resolves the deferred with the given value.
    ///
    /// Sending after the deferred has already been settled is a no-op.
    pub fn resolve(&self, value: T) {
        let _ = self.sender.send(Ok(value));
    }

    /// Rejects the deferred with the given reason.
    ///
    /// Sending after the deferred has already been settled is a no-op.
    pub fn reject(&self, reason: E) {
        let _ = self.sender.send(Err(reason));
    }

    /// Returns a cloneable handle to settle this deferred from elsewhere
    /// (for example, another thread).
    pub fn settler(&self) -> Sender<Result<T, E>> {
        self.sender.clone()
    }

    /// Blocks the current thread until the deferred is settled and returns the
    /// settled result.
    pub fn wait(self) -> Result<T, E> {
        match self.receiver.recv() {
            Ok(result) => result,
            Err(_) => unreachable!("deferred sender is held by the Deferred itself"),
        }
    }
}

/// Creates a deferred value whose resolve and reject can be called externally.
///
/// # Returns
///
/// A [`Deferred`] exposing `resolve`, `reject`, and `wait`.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_defer;
///
/// let d = umt_defer::<i32, String>();
/// d.resolve(42);
/// assert_eq!(d.wait(), Ok(42));
/// ```
///
/// ```
/// use umt_rust::async_util::umt_defer;
///
/// let d = umt_defer::<i32, String>();
/// d.reject("deferred error".to_string());
/// assert_eq!(d.wait(), Err("deferred error".to_string()));
/// ```
#[inline]
pub fn umt_defer<T, E>() -> Deferred<T, E> {
    let (sender, receiver) = mpsc::channel();
    Deferred { sender, receiver }
}
