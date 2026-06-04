use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use umt_rust::async_util::{umt_sleep, umt_throttle_async};

#[test]
fn test_throttle_coalesces_concurrent_calls() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_clone = Arc::clone(&calls);
    let throttled = umt_throttle_async(
        move |value: i32| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
            umt_sleep(10);
            Ok::<i32, String>(value)
        },
        0,
    );
    let a = throttled.call(1);
    let b = throttled.call(2);
    assert_eq!(a.recv().unwrap(), Ok(1));
    assert_eq!(b.recv().unwrap(), Ok(1));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn test_throttle_rejects_within_lockout_window() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_clone = Arc::clone(&calls);
    let throttled = umt_throttle_async(
        move |value: i32| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
            Ok::<i32, String>(value)
        },
        1000,
    );
    assert_eq!(throttled.call(1).recv().unwrap(), Ok(1));
    assert_eq!(
        throttled.call(2).recv().unwrap(),
        Err("throttleAsync window not elapsed".to_string())
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn test_throttle_allows_new_invocation_after_window() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_clone = Arc::clone(&calls);
    let throttled = umt_throttle_async(
        move |value: i32| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
            Ok::<i32, String>(value)
        },
        5,
    );
    throttled.call(1).recv().unwrap().unwrap();
    umt_sleep(20);
    throttled.call(2).recv().unwrap().unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn test_throttle_cancel_clears_lockout() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_clone = Arc::clone(&calls);
    let throttled = umt_throttle_async(
        move |value: i32| {
            calls_clone.fetch_add(1, Ordering::SeqCst);
            Ok::<i32, String>(value)
        },
        1000,
    );
    throttled.call(1).recv().unwrap().unwrap();
    throttled.cancel();
    throttled.call(2).recv().unwrap().unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn test_throttle_propagates_rejection() {
    let throttled = umt_throttle_async(|_: i32| Err::<i32, String>("boom".to_string()), 5);
    assert_eq!(throttled.call(0).recv().unwrap(), Err("boom".to_string()));
}
