//! A throttled wrapper around an operation.
//!
//! Mirrors the TypeScript `throttleAsync`: concurrent calls while an invocation
//! is in flight all receive that single invocation's result; a call made within
//! the lockout window after completion is rejected with
//! `throttleAsync window not elapsed`; and `cancel` clears both the in-flight
//! state and the lockout so the next call runs immediately.
//!
//! Each accepted call returns a [`Receiver`] delivering the settled
//! `Result<R, String>`; a call rejected by the lockout returns a receiver that
//! immediately yields the lockout error.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

type Subscribers<R> = Vec<Sender<Result<R, String>>>;

struct Inflight<R> {
    id: u64,
    subscribers: Subscribers<R>,
}

struct ThrottleState<R> {
    wait: u64,
    inflight: Mutex<Option<Inflight<R>>>,
    locked_until: Mutex<Option<Instant>>,
    counter: AtomicU64,
}

/// A throttled wrapper produced by [`umt_throttle_async`].
pub struct ThrottledAsync<A, R, F> {
    state: Arc<ThrottleState<R>>,
    function: Arc<F>,
    _marker: std::marker::PhantomData<fn(A)>,
}

impl<A, R, F> ThrottledAsync<A, R, F>
where
    A: Send + 'static,
    R: Clone + Send + 'static,
    F: Fn(A) -> Result<R, String> + Send + Sync + 'static,
{
    /// Invokes the throttled operation. Concurrent callers during an in-flight
    /// invocation share its result; a call within the lockout window is
    /// rejected; otherwise a fresh invocation starts.
    pub fn call(&self, args: A) -> Receiver<Result<R, String>> {
        let (sender, receiver) = mpsc::channel();

        {
            let mut inflight = self.state.inflight.lock().unwrap();
            if let Some(ref mut current) = *inflight {
                current.subscribers.push(sender);
                return receiver;
            }

            if let Some(locked_until) = *self.state.locked_until.lock().unwrap()
                && Instant::now() < locked_until
            {
                let _ = sender.send(Err("throttleAsync window not elapsed".to_string()));
                return receiver;
            }

            let id = self.state.counter.fetch_add(1, Ordering::SeqCst) + 1;
            *inflight = Some(Inflight {
                id,
                subscribers: vec![sender],
            });

            let state = Arc::clone(&self.state);
            let function = Arc::clone(&self.function);
            thread::spawn(move || {
                let outcome = function(args);
                let mut inflight = state.inflight.lock().unwrap();
                let subscribers = match inflight.as_ref() {
                    Some(current) if current.id == id => inflight.take().map(|c| {
                        *state.locked_until.lock().unwrap() =
                            Some(Instant::now() + Duration::from_millis(state.wait));
                        c.subscribers
                    }),
                    _ => None,
                };
                drop(inflight);
                if let Some(subscribers) = subscribers {
                    for subscriber in subscribers {
                        let _ = subscriber.send(outcome.clone());
                    }
                }
            });
        }

        receiver
    }

    /// Clears the in-flight state and the lockout window, allowing the next call
    /// to run immediately.
    pub fn cancel(&self) {
        *self.state.inflight.lock().unwrap() = None;
        *self.state.locked_until.lock().unwrap() = None;
    }
}

/// Creates a throttled wrapper around `function_`.
///
/// # Arguments
///
/// * `function_` - The operation to throttle; returns `Result<R, String>`
/// * `wait` - The lockout window in milliseconds applied after completion
///
/// # Returns
///
/// A [`ThrottledAsync`] exposing `call` and `cancel`.
///
/// # Examples
///
/// ```
/// use umt_rust::async_util::umt_throttle_async;
///
/// let throttled = umt_throttle_async(|value: i32| Ok::<i32, String>(value), 1000);
/// assert_eq!(throttled.call(1).recv().unwrap(), Ok(1));
/// assert_eq!(
///     throttled.call(2).recv().unwrap(),
///     Err("throttleAsync window not elapsed".to_string())
/// );
/// ```
pub fn umt_throttle_async<A, R, F>(function_: F, wait: u64) -> ThrottledAsync<A, R, F>
where
    A: Send + 'static,
    R: Clone + Send + 'static,
    F: Fn(A) -> Result<R, String> + Send + Sync + 'static,
{
    ThrottledAsync {
        state: Arc::new(ThrottleState {
            wait,
            inflight: Mutex::new(None),
            locked_until: Mutex::new(None),
            counter: AtomicU64::new(0),
        }),
        function: Arc::new(function_),
        _marker: std::marker::PhantomData,
    }
}
