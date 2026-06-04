//! A debounced wrapper around an operation.
//!
//! Mirrors the TypeScript `debounceAsync`: calls made within the `wait` window
//! reset the timer and share a single underlying invocation, the latest
//! arguments win, and every caller receives the same result. `cancel` rejects
//! every pending caller with `debounceAsync cancelled`.
//!
//! Each caller receives a [`Receiver`] that delivers the settled
//! `Result<R, String>` once the debounced invocation completes (or is
//! cancelled). Block on it with `recv` to obtain the value, matching the
//! `await` of the TypeScript version.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Pending<R> = Vec<Sender<Result<R, String>>>;

struct DebounceState<A, R> {
    generation: AtomicU64,
    wait: u64,
    pending: Mutex<Pending<R>>,
    latest_args: Mutex<Option<A>>,
}

/// A debounced wrapper produced by [`umt_debounce_async`].
pub struct DebouncedAsync<A, R, F> {
    state: Arc<DebounceState<A, R>>,
    function: Arc<F>,
}

impl<A, R, F> DebouncedAsync<A, R, F>
where
    A: Clone + Send + 'static,
    R: Clone + Send + 'static,
    F: Fn(A) -> Result<R, String> + Send + Sync + 'static,
{
    /// Schedules an invocation with the given arguments, resetting the debounce
    /// timer. Returns a receiver that yields the shared result once the window
    /// elapses, or the cancellation error if `cancel` is called first.
    pub fn call(&self, args: A) -> Receiver<Result<R, String>> {
        let (sender, receiver) = mpsc::channel();
        {
            let mut pending = self.state.pending.lock().unwrap();
            pending.push(sender);
        }
        *self.state.latest_args.lock().unwrap() = Some(args);

        let generation = self.state.generation.fetch_add(1, Ordering::SeqCst) + 1;
        let state = Arc::clone(&self.state);
        let function = Arc::clone(&self.function);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(state.wait));
            if state.generation.load(Ordering::SeqCst) != generation {
                return;
            }
            let resolvers: Pending<R> = std::mem::take(&mut *state.pending.lock().unwrap());
            let args = state.latest_args.lock().unwrap().clone();
            let Some(args) = args else {
                return;
            };
            let outcome = function(args);
            for resolver in resolvers {
                let _ = resolver.send(outcome.clone());
            }
        });

        receiver
    }

    /// Cancels any pending invocation, rejecting every waiting caller with
    /// `debounceAsync cancelled`. A no-op when nothing is pending.
    pub fn cancel(&self) {
        self.state.generation.fetch_add(1, Ordering::SeqCst);
        let rejecters: Pending<R> = std::mem::take(&mut *self.state.pending.lock().unwrap());
        for rejecter in rejecters {
            let _ = rejecter.send(Err("debounceAsync cancelled".to_string()));
        }
    }
}

/// Creates a debounced wrapper around `function_`.
///
/// # Arguments
///
/// * `function_` - The operation to debounce; receives the latest arguments and
///   returns `Result<R, String>`
/// * `wait` - The debounce window in milliseconds
///
/// # Returns
///
/// A [`DebouncedAsync`] exposing `call` and `cancel`.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_debounce_async;
///
/// let debounced = umt_debounce_async(|value: i32| Ok::<i32, String>(value * 2), 5);
/// let a = debounced.call(1);
/// let b = debounced.call(2);
/// let c = debounced.call(3);
/// assert_eq!(a.recv().unwrap(), Ok(6));
/// assert_eq!(b.recv().unwrap(), Ok(6));
/// assert_eq!(c.recv().unwrap(), Ok(6));
/// ```
pub fn umt_debounce_async<A, R, F>(function_: F, wait: u64) -> DebouncedAsync<A, R, F>
where
    A: Clone + Send + 'static,
    R: Clone + Send + 'static,
    F: Fn(A) -> Result<R, String> + Send + Sync + 'static,
{
    DebouncedAsync {
        state: Arc::new(DebounceState {
            generation: AtomicU64::new(0),
            wait,
            pending: Mutex::new(Vec::new()),
            latest_args: Mutex::new(None),
        }),
        function: Arc::new(function_),
    }
}
