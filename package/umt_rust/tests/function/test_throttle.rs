use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use umt_rust::function::umt_throttle;

const WAIT: Duration = Duration::from_millis(50);

// Mirrors: "should invoke immediately on the first call"
#[test]
fn test_throttle_invokes_immediately_on_first_call() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let throttled = umt_throttle(move |_: ()| sink.set(sink.get() + 1), WAIT);

    throttled.call(());
    assert_eq!(count.get(), 1);
}

// Mirrors: "should throttle subsequent calls within the wait period"
#[test]
fn test_throttle_throttles_subsequent_calls_within_wait() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let throttled = umt_throttle(move |_: ()| sink.set(sink.get() + 1), WAIT);

    throttled.call(());
    throttled.call(());
    throttled.call(());

    assert_eq!(count.get(), 1);
}

// Mirrors: "should invoke trailing call after the wait period"
#[test]
fn test_throttle_invokes_trailing_call_after_wait() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let sink = Rc::clone(&calls);
    let throttled = umt_throttle(move |n: i32| sink.borrow_mut().push(n), WAIT);

    throttled.call(1);
    throttled.call(2);

    thread::sleep(WAIT + Duration::from_millis(20));
    assert!(throttled.flush());
    assert_eq!(calls.borrow().len(), 2);
    assert_eq!(*calls.borrow().last().unwrap(), 2);
}

// Mirrors: "should pass the latest arguments to the trailing call"
#[test]
fn test_throttle_passes_latest_arguments_to_trailing_call() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let sink = Rc::clone(&calls);
    let throttled = umt_throttle(move |s: &'static str| sink.borrow_mut().push(s), WAIT);

    throttled.call("a");
    throttled.call("b");
    throttled.call("c");

    thread::sleep(WAIT + Duration::from_millis(20));
    assert!(throttled.flush());
    assert_eq!(*calls.borrow().last().unwrap(), "c");
}

// Mirrors: "should allow invocation again after wait period has passed"
#[test]
fn test_throttle_allows_invocation_again_after_wait() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let throttled = umt_throttle(move |_: ()| sink.set(sink.get() + 1), WAIT);

    throttled.call(());
    thread::sleep(WAIT + Duration::from_millis(20));

    throttled.call(());
    assert_eq!(count.get(), 2);
}

// Mirrors: "should cancel pending invocation"
#[test]
fn test_throttle_cancel_pending() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let throttled = umt_throttle(move |_: ()| sink.set(sink.get() + 1), WAIT);

    throttled.call(());
    throttled.call(());
    throttled.cancel();

    thread::sleep(WAIT + Duration::from_millis(20));
    assert!(!throttled.flush());
    assert_eq!(count.get(), 1);
}

// Mirrors: "should clear pending timer when called after wait period elapses"
#[test]
fn test_throttle_clears_pending_timer_when_called_after_wait() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let throttled = umt_throttle(move |_: ()| sink.set(sink.get() + 1), WAIT);

    throttled.call(());
    throttled.call(());

    thread::sleep(WAIT + Duration::from_millis(20));

    throttled.call(());
    assert_eq!(count.get(), 2);
}

// Mirrors: "should handle cancel when no timer is pending"
#[test]
fn test_throttle_cancel_when_no_timer_pending() {
    let count = Rc::new(Cell::new(0));
    let sink = Rc::clone(&count);
    let throttled = umt_throttle(move |_: ()| sink.set(sink.get() + 1), WAIT);

    throttled.cancel();
    assert_eq!(count.get(), 0);
}
