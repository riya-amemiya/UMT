//! A throttle wrapper that limits a function to at most one invocation per wait
//! period, with a trailing call for activity that happened during the cooldown.
//!
//! Because Rust has no built-in single-threaded event loop like JavaScript's
//! `setTimeout`, the trailing edge runs when [`Throttled::flush`] is called, as
//! the TypeScript `flush` does. Timing decisions use real wall-clock time via
//! [`std::time::Instant`], mirroring `Date.now()` in the TypeScript reference.

use std::cell::RefCell;
use std::time::{Duration, Instant};

struct ThrottleState<A> {
    /// The instant of the most recent invocation (`lastCallTime` in TS).
    /// `None` represents the initial `lastCallTime = 0` so the first call fires.
    last_call_time: Option<Instant>,
    /// Whether a trailing timer is pending (`timerId !== undefined` in TS).
    timer_active: bool,
    /// The latest arguments captured for a pending trailing invocation
    /// (`lastContext` in TS).
    last_arguments: Option<A>,
}

/// A throttled function with a trailing call and cancellation.
///
/// Call [`Throttled::call`] to attempt an invocation, [`Throttled::flush`] to
/// run the pending trailing invocation immediately, [`Throttled::pending`] to
/// check for a pending trailing invocation, and [`Throttled::cancel`] to
/// discard any pending invocation.
pub struct Throttled<A, R> {
    func: Box<dyn Fn(A) -> R>,
    wait: Duration,
    state: RefCell<ThrottleState<A>>,
}

impl<A, R> Throttled<A, R>
where
    A: Clone,
{
    fn invoke(&self, argument: A) {
        self.state.borrow_mut().last_call_time = Some(Instant::now());
        (self.func)(argument);
    }

    /// Invokes the throttled function with the given argument.
    ///
    /// If the wait period has elapsed since the last invocation (or this is the
    /// first call), any pending trailing timer is cleared and the function is
    /// invoked immediately. Otherwise the latest arguments are stored and a single
    /// trailing timer is armed for the remaining cooldown.
    pub fn call(&self, argument: A) {
        let remaining = {
            let mut state = self.state.borrow_mut();
            state.last_arguments = Some(argument);
            match state.last_call_time {
                Some(last_call_time) => self.wait.checked_sub(last_call_time.elapsed()),
                None => None,
            }
        };

        match remaining {
            // `remaining <= 0` in TS: cooldown elapsed, fire immediately.
            None => {
                let argument = {
                    let mut state = self.state.borrow_mut();
                    state.timer_active = false;
                    state.last_arguments.take()
                };
                if let Some(argument) = argument {
                    self.invoke(argument);
                }
            }
            // `remaining > 0` in TS: arm a single trailing timer.
            Some(_) => {
                self.state.borrow_mut().timer_active = true;
            }
        }
    }

    /// Immediately runs the pending trailing invocation, like the TypeScript
    /// `flush`.
    ///
    /// When a trailing timer is pending it is cleared and the underlying
    /// function is invoked with the latest stored arguments without waiting for
    /// the rest of the cooldown. Does nothing when no trailing invocation is
    /// pending. Returns `true` when the trailing invocation fired.
    pub fn flush(&self) -> bool {
        let argument = {
            let mut state = self.state.borrow_mut();
            if !state.timer_active {
                return false;
            }
            state.timer_active = false;
            state.last_arguments.take()
        };

        match argument {
            Some(argument) => {
                self.invoke(argument);
                true
            }
            None => false,
        }
    }

    /// Cancels any pending throttled invocation and resets all internal state.
    pub fn cancel(&self) {
        let mut state = self.state.borrow_mut();
        state.timer_active = false;
        state.last_arguments = None;
        state.last_call_time = None;
    }

    /// Returns whether a trailing throttled invocation is currently pending,
    /// like the TypeScript `pending`.
    pub fn pending(&self) -> bool {
        self.state.borrow().timer_active
    }
}

/// Creates a throttled version of the provided single-argument function that
/// invokes it at most once per the specified wait period, with a trailing call
/// for activity that occurred during the cooldown.
///
/// # Arguments
///
/// * `f` - The function to throttle
/// * `wait` - The minimum time between invocations
///
/// # Returns
///
/// A [`Throttled`] value. Use `.call(arg)` to attempt an invocation, `.flush()`
/// to run the pending trailing invocation immediately, `.pending()` to check for
/// a pending trailing invocation, and `.cancel()` to discard pending work.
///
/// # Examples
///
/// ```
/// use std::cell::Cell;
/// use std::rc::Rc;
/// use std::thread;
/// use std::time::Duration;
/// use umt_rust::function::umt_throttle;
///
/// let count = Rc::new(Cell::new(0));
/// let sink = Rc::clone(&count);
/// let throttled = umt_throttle(
///     move |n: i32| sink.set(sink.get() + n),
///     Duration::from_millis(20),
/// );
///
/// throttled.call(1);
/// throttled.call(2);
/// assert_eq!(count.get(), 1);
/// assert!(throttled.pending());
///
/// assert!(throttled.flush());
/// assert_eq!(count.get(), 3);
/// assert!(!throttled.pending());
///
/// thread::sleep(Duration::from_millis(30));
/// throttled.call(4);
/// assert_eq!(count.get(), 7);
/// ```
pub fn umt_throttle<F, A, R>(f: F, wait: Duration) -> Throttled<A, R>
where
    F: Fn(A) -> R + 'static,
    A: Clone,
{
    Throttled {
        func: Box::new(f),
        wait,
        state: RefCell::new(ThrottleState {
            last_call_time: None,
            timer_active: false,
            last_arguments: None,
        }),
    }
}
