//! A debounce wrapper that delays invoking a function until after a wait period
//! has elapsed since the last call.
//!
//! Because Rust has no built-in single-threaded event loop like JavaScript's
//! `setTimeout`, the trailing edge is driven explicitly by [`Debounced::flush`]
//! (which reproduces exactly what the JavaScript timer callback would do once the
//! wait has elapsed). Timing decisions use real wall-clock time via
//! [`std::time::Instant`], mirroring `Date.now()` in the TypeScript reference.

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
/// Call [`Debounced::call`] to schedule, [`Debounced::flush`] to drive the
/// trailing edge once the wait has elapsed, and [`Debounced::cancel`] to discard
/// any pending invocation.
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
    /// On the leading edge (when `leading` is enabled and no timer is pending)
    /// the underlying function is invoked immediately. A timer is started on the
    /// first call of a burst; subsequent calls within the wait window only update
    /// the stored arguments and the last-call time.
    pub fn call(&self, argument: A) {
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

    /// Drives the trailing edge, reproducing the JavaScript timer callback.
    ///
    /// If the wait period has elapsed since the last call, the timer is cleared
    /// and, when `trailing` is enabled and arguments are pending, the underlying
    /// function is invoked with the latest arguments. Otherwise the timer remains
    /// active (the JavaScript code reschedules itself for the remaining time).
    ///
    /// Returns `true` when the trailing invocation fired, `false` otherwise.
    pub fn flush(&self) -> bool {
        let elapsed = {
            let state = self.state.borrow();
            if !state.timer_active {
                return false;
            }
            match state.last_call_time {
                Some(last_call_time) => last_call_time.elapsed(),
                None => return false,
            }
        };

        if elapsed < self.wait {
            return false;
        }

        let argument = {
            let mut state = self.state.borrow_mut();
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

    /// Returns whether a debounced invocation is currently pending.
    pub fn is_pending(&self) -> bool {
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
/// A [`Debounced`] value. Use `.call(arg)` to schedule, `.flush()` to drive the
/// trailing edge once `wait` has elapsed, and `.cancel()` to discard pending work.
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
///
/// thread::sleep(Duration::from_millis(30));
/// assert!(debounced.flush());
/// assert_eq!(count.get(), 3);
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
