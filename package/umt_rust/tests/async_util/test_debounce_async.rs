use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use umt_rust::async_util::{umt_debounce_async, umt_sleep};

#[test]
fn test_debounce_invokes_once_for_rapid_calls_with_latest_args() {
    let calls = Arc::new(AtomicUsize::new(0));
    let last_arg = Arc::new(AtomicUsize::new(0));
    let calls_clone = Arc::clone(&calls);
    let last_arg_clone = Arc::clone(&last_arg);
    let debounced = umt_debounce_async(
        move |value: i32| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
            last_arg_clone.store(value as usize, Ordering::SeqCst);
            Ok::<i32, String>(value * 2)
        },
        10,
    );
    let a = debounced.call(1);
    let b = debounced.call(2);
    let c = debounced.call(3);
    assert_eq!(a.recv().unwrap(), Ok(6));
    assert_eq!(b.recv().unwrap(), Ok(6));
    assert_eq!(c.recv().unwrap(), Ok(6));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(last_arg.load(Ordering::SeqCst), 3);
}

#[test]
fn test_debounce_invokes_again_after_wait_elapsed() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_clone = Arc::clone(&calls);
    let debounced = umt_debounce_async(
        move |value: i32| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
            Ok::<i32, String>(value)
        },
        5,
    );
    debounced.call(1).recv().unwrap().unwrap();
    umt_sleep(20);
    debounced.call(2).recv().unwrap().unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn test_debounce_rejects_pending_callers_on_cancel() {
    let debounced = umt_debounce_async(|value: i32| Ok::<i32, String>(value), 50);
    let pending = debounced.call(1);
    debounced.cancel();
    assert_eq!(
        pending.recv().unwrap(),
        Err("debounceAsync cancelled".to_string())
    );
}

#[test]
fn test_debounce_propagates_rejection() {
    let debounced = umt_debounce_async(|_: i32| Err::<i32, String>("boom".to_string()), 5);
    assert_eq!(debounced.call(0).recv().unwrap(), Err("boom".to_string()));
}

#[test]
fn test_debounce_cancel_is_noop_when_nothing_pending() {
    let debounced = umt_debounce_async(|_: i32| Ok::<i32, String>(1), 10);
    debounced.cancel();
}
