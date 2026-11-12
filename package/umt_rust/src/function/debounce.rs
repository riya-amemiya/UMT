//! A debounce wrapper that delays invoking a function until after a wait period
//! has elapsed since the last call.
//!
//! Because Rust has no built-in single-threaded event loop like JavaScript's
//! `setTimeout`, the trailing edge runs when [`Debounced::flush`] is called, as
//! the TypeScript `flush` does, or lazily at the next [`Debounced::call`] once
//! the wait has elapsed, which is when the JavaScript timer would already have
//! fired. Timing decisions use real wall-clock time via [`std::time::Instant`],
//! mirroring `Date.now()` in the TypeScript reference.

use std::cell::RefCell;
use std::time::{Duration, Instant};

/// Configuration for leading/trailing edge invocation of a debounced function.
///
/// Mirrors the `DebounceOptions` interface of the TypeScript reference.
#[derive(Debug, Clone, Copy)]
pub struct DebounceOptions {
    /// Whether to invoke on the leading edge of the timeout.
    pub leading: bool,
    /// Whether to invoke on the trailing edge of the timeout.
    pub trailing: bool,
}

impl Default for DebounceOptions {
    /// Returns the TypeScript default: `leading = false`, `trailing = true`.
    fn default() -> Self {
        DebounceOptions {
            leading: false,
            trailing: true,
        }
    }
}

struct DebounceState<A> {
    /// Whether a timer is currently pending (`timerId !== undefined` in TS).
    timer_active: bool,
    /// The latest arguments captured for a pending trailing invocation.
    last_arguments: Option<A>,
    /// The instant of the most recent call (`lastCallTime` in TS).
    last_call_time: Option<Instant>,
}

/// A debounced function with leading/trailing edge control and cancellation.
///
/// Call [`Debounced::call`] to schedule, [`Debounced::flush`] to run the pending
/// trailing invocation immediately, [`Debounced::pending`] to check for a
/// pending timer, and [`Debounced::cancel`] to discard any pending invocation.
pub struct Debounced<A, R> {
    func: Box<dyn Fn(A) -> R>,
    wait: Duration,
    leading: bool,
    trailing: bool,
    state: RefCell<DebounceState<A>>,
}

impl<A, R> Debounced<A, R>
where
    A: Clone,
{
    /// Invokes the debounced function with the given argument.
    ///
    /// If the wait has elapsed since the previous call of a burst, the trailing
    /// edge of that burst runs first, as the JavaScript timer would already have
    /// fired. On the leading edge (when `leading` is enabled and no timer is
    /// pending) the underlying function is invoked immediately. A timer is
    /// started on the first call of a burst; subsequent calls within the wait
    /// window only update the stored arguments and the last-call time.
    pub fn call(&self, argument: A) {
        let is_expired = {
            let state = self.state.borrow();
            state.timer_active
                && state
                    .last_call_time
                    .is_some_and(|last_call_time| last_call_time.elapsed() >= self.wait)
        };
        if is_expired {
            self.flush();
        }

        let is_first_call = {
            let mut state = self.state.borrow_mut();
            state.last_arguments = Some(argument);
            state.last_call_time = Some(Instant::now());
            !state.timer_active
        };

        if self.leading && is_first_call {
            let argument = {
                let mut state = self.state.borrow_mut();
                state.last_arguments.take()
            };
            if let Some(argument) = argument {
                (self.func)(argument);
            }
        }

        if is_first_call {
            self.state.borrow_mut().timer_active = true;
        }
    }

    /// Immediately runs the pending trailing invocation, like the TypeScript
    /// `flush`.
    ///
    /// When a timer is pending it is cleared and, when `trailing` is enabled and
    /// arguments are pending, the underlying function is invoked with the latest
    /// arguments without waiting for the rest of the wait period. Does nothing
    /// when no timer is pending.
    ///
    /// Returns `true` when the trailing invocation fired, `false` otherwise.
    pub fn flush(&self) -> bool {
        let argument = {
            let mut state = self.state.borrow_mut();
            if !state.timer_active {
                return false;
            }
            state.timer_active = false;
            if self.trailing {
                state.last_arguments.take()
            } else {
                None
            }
        };

        match argument {
            Some(argument) => {
                (self.func)(argument);
                true
            }
            None => false,
        }
    }

    /// Cancels any pending debounced invocation and resets all internal state.
    pub fn cancel(&self) {
        let mut state = self.state.borrow_mut();
        state.timer_active = false;
        state.last_arguments = None;
        state.last_call_time = None;
    }

    /// Returns whether a debounce timer is currently pending, like the
    /// TypeScript `pending`.
    ///
    /// The timer stays pending until [`Debounced::flush`], [`Debounced::cancel`]
    /// or a [`Debounced::call`] after the wait has elapsed resolves it.
    pub fn pending(&self) -> bool {
        self.state.borrow().timer_active
    }
}

/// Creates a debounced version of the provided single-argument function that
/// delays invoking it until after the specified wait time has elapsed since the
/// last call.
///
/// # Arguments
///
/// * `f` - The function to debounce
/// * `wait` - The debounce delay
/// * `options` - Configuration for leading/trailing invocation
///
/// # Returns
///
/// A [`Debounced`] value. Use `.call(arg)` to schedule, `.flush()` to run the
/// pending trailing invocation immediately, `.pending()` to check for a pending
/// timer, and `.cancel()` to discard pending work.
///
/// # Examples
///
/// ```
/// use std::cell::Cell;
/// use std::rc::Rc;
/// use std::thread;
/// use std::time::Duration;
/// use umt_rust::function::{umt_debounce, DebounceOptions};
///
/// let count = Rc::new(Cell::new(0));
/// let sink = Rc::clone(&count);
/// let debounced = umt_debounce(
///     move |n: i32| sink.set(sink.get() + n),
///     Duration::from_millis(20),
///     DebounceOptions::default(),
/// );
///
/// debounced.call(1);
/// debounced.call(2);
/// debounced.call(3);
/// assert_eq!(count.get(), 0);
/// assert!(debounced.pending());
///
/// assert!(debounced.flush());
/// assert_eq!(count.get(), 3);
/// assert!(!debounced.pending());
///
/// debounced.call(4);
/// thread::sleep(Duration::from_millis(30));
/// debounced.call(5);
/// assert_eq!(count.get(), 7);
/// ```
pub fn umt_debounce<F, A, R>(f: F, wait: Duration, options: DebounceOptions) -> Debounced<A, R>
where
    F: Fn(A) -> R + 'static,
    A: Clone,
{
    Debounced {
        func: Box::new(f),
        wait,
        leading: options.leading,
        trailing: options.trailing,
        state: RefCell::new(DebounceState {
            timer_active: false,
            last_arguments: None,
            last_call_time: None,
        }),
    }
}
