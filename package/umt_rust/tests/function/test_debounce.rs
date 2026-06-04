use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use umt_rust::function::{DebounceOptions, umt_debounce};

const WAIT: Duration = Duration::from_millis(50);

// Mirrors: "should delay invocation until after the wait period"
#[test]
fn test_debounce_delays_invocation_until_after_wait() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let debounced = umt_debounce(
        move |_: ()| sink.set(sink.get() + 1),
        WAIT,
        DebounceOptions::default(),
    );

    debounced.call(());
    assert_eq!(count.get(), 0);

    thread::sleep(WAIT + Duration::from_millis(20));
    assert!(debounced.flush());
    assert_eq!(count.get(), 1);
}

// Mirrors: "should reset the timer on subsequent calls"
#[test]
fn test_debounce_resets_timer_on_subsequent_calls() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let debounced = umt_debounce(
        move |_: ()| sink.set(sink.get() + 1),
        WAIT,
        DebounceOptions::default(),
    );

    debounced.call(());
    thread::sleep(WAIT / 2);
    debounced.call(());
    thread::sleep(WAIT / 2);
    // Wait window restarted by the second call: not yet elapsed.
    assert!(!debounced.flush());
    assert_eq!(count.get(), 0);

    thread::sleep(WAIT / 2 + Duration::from_millis(20));
    assert!(debounced.flush());
    assert_eq!(count.get(), 1);
}

// Mirrors: "should pass the latest arguments"
#[test]
fn test_debounce_passes_latest_arguments() {
    let last = Rc::new(Cell::new(0));
    let sink = Rc::clone(&last);
    let debounced = umt_debounce(move |n: i32| sink.set(n), WAIT, DebounceOptions::default());

    debounced.call(1);
    debounced.call(2);
    debounced.call(3);

    thread::sleep(WAIT + Duration::from_millis(20));
    assert!(debounced.flush());
    assert_eq!(last.get(), 3);
}

// Mirrors: "should invoke on the leading edge when leading is true"
#[test]
fn test_debounce_leading_only() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let debounced = umt_debounce(
        move |_: ()| sink.set(sink.get() + 1),
        WAIT,
        DebounceOptions {
            leading: true,
            trailing: false,
        },
    );

    debounced.call(());
    assert_eq!(count.get(), 1);

    debounced.call(());
    assert_eq!(count.get(), 1);

    thread::sleep(WAIT + Duration::from_millis(20));
    // trailing disabled: flush does not fire.
    assert!(!debounced.flush());
    assert_eq!(count.get(), 1);
}

// Mirrors: "should invoke on both leading and trailing edges"
#[test]
fn test_debounce_leading_and_trailing() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let sink = Rc::clone(&calls);
    let debounced = umt_debounce(
        move |n: i32| sink.borrow_mut().push(n),
        WAIT,
        DebounceOptions {
            leading: true,
            trailing: true,
        },
    );

    debounced.call(1);
    assert_eq!(calls.borrow().len(), 1);
    assert_eq!(*calls.borrow().last().unwrap(), 1);

    debounced.call(2);
    thread::sleep(WAIT + Duration::from_millis(20));
    assert!(debounced.flush());
    assert_eq!(calls.borrow().len(), 2);
    assert_eq!(*calls.borrow().last().unwrap(), 2);
}

// Mirrors: "should cancel pending invocation"
#[test]
fn test_debounce_cancel_pending() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let debounced = umt_debounce(
        move |_: ()| sink.set(sink.get() + 1),
        WAIT,
        DebounceOptions::default(),
    );

    debounced.call(());
    debounced.cancel();

    thread::sleep(WAIT + Duration::from_millis(20));
    assert!(!debounced.flush());
    assert_eq!(count.get(), 0);
}

// Mirrors: "should handle cancel when no timer is pending"
#[test]
fn test_debounce_cancel_when_no_timer_pending() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let debounced = umt_debounce(
        move |_: ()| sink.set(sink.get() + 1),
        WAIT,
        DebounceOptions::default(),
    );

    debounced.cancel();

    thread::sleep(WAIT + Duration::from_millis(20));
    assert!(!debounced.flush());
    assert_eq!(count.get(), 0);
}
