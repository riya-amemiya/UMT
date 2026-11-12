use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::Duration;
use umt_rust::async_util::{umt_memoize_async, umt_memoize_async_with_resolver};

#[test]
fn test_memoize_async_caches_ok_values() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let memoized = umt_memoize_async(
        move |n: i32| {
            counter.fetch_add(1, Ordering::SeqCst);
            Ok::<i32, String>(n * 2)
        },
        None,
    );
    assert_eq!(memoized.call(5), Ok(10));
    assert_eq!(memoized.call(5), Ok(10));
    assert_eq!(memoized.call(6), Ok(12));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(memoized.cache_get(&5), Some(10));
    assert_eq!(memoized.cache_get(&7), None);
    assert_eq!(memoized.cache_size(), 2);
}

#[test]
fn test_memoize_async_shares_one_in_flight_call() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let (started_tx, started_rx) = mpsc::channel::<()>();
    let (release_tx, release_rx) = mpsc::channel::<()>();
    let release_rx = std::sync::Mutex::new(release_rx);
    let started_tx = std::sync::Mutex::new(started_tx);
    let memoized = Arc::new(umt_memoize_async(
        move |key: &'static str| {
            counter.fetch_add(1, Ordering::SeqCst);
            started_tx.lock().unwrap().send(()).unwrap();
            release_rx.lock().unwrap().recv().unwrap();
            Ok::<String, String>(format!("{}-done", key))
        },
        None,
    ));

    let leader = {
        let memoized = Arc::clone(&memoized);
        thread::spawn(move || memoized.call("a"))
    };
    started_rx.recv().unwrap();

    let followers: Vec<_> = (0..3)
        .map(|_| {
            let memoized = Arc::clone(&memoized);
            thread::spawn(move || memoized.call("a"))
        })
        .collect();
    thread::sleep(Duration::from_millis(50));
    assert_eq!(memoized.cache_size(), 0);
    release_tx.send(()).unwrap();

    assert_eq!(leader.join().unwrap(), Ok("a-done".to_string()));
    for follower in followers {
        assert_eq!(follower.join().unwrap(), Ok("a-done".to_string()));
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(memoized.cache_has(&"a"));
}

#[test]
fn test_memoize_async_shares_errors_and_does_not_cache_them() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let barrier = Arc::new(Barrier::new(2));
    let gate = Arc::clone(&barrier);
    let memoized = Arc::new(umt_memoize_async(
        move |n: i32| {
            if counter.fetch_add(1, Ordering::SeqCst) == 0 {
                gate.wait();
                thread::sleep(Duration::from_millis(50));
                Err("fail".to_string())
            } else {
                Ok(n)
            }
        },
        None,
    ));

    let leader = {
        let memoized = Arc::clone(&memoized);
        thread::spawn(move || memoized.call(1))
    };
    barrier.wait();
    let follower = memoized.call(1);
    assert_eq!(leader.join().unwrap(), Err("fail".to_string()));
    assert_eq!(follower, Err("fail".to_string()));
    assert!(!memoized.cache_has(&1));
    assert_eq!(memoized.call(1), Ok(1));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn test_memoize_async_invokes_again_after_error() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let memoized = umt_memoize_async(
        move |key: &'static str| {
            if counter.fetch_add(1, Ordering::SeqCst) == 0 {
                Err("fail".to_string())
            } else {
                Ok(key.len())
            }
        },
        None,
    );
    assert_eq!(memoized.call("k"), Err("fail".to_string()));
    assert_eq!(memoized.call("k"), Ok(1));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn test_memoize_async_with_resolver() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let memoized = umt_memoize_async_with_resolver(
        move |(a, b): (i32, i32)| {
            counter.fetch_add(1, Ordering::SeqCst);
            Ok::<i32, String>(a + b)
        },
        |&(a, b): &(i32, i32)| format!("{},{}", a, b),
        None,
    );
    assert_eq!(memoized.call((1, 2)), Ok(3));
    assert_eq!(memoized.call((1, 2)), Ok(3));
    assert_eq!(memoized.call((1, 3)), Ok(4));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert!(memoized.cache_has(&"1,2".to_string()));
    assert!(memoized.cache_has(&"1,3".to_string()));
}

#[test]
fn test_memoize_async_evicts_oldest_at_max_size() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let memoized = umt_memoize_async(
        move |n: i32| {
            counter.fetch_add(1, Ordering::SeqCst);
            Ok::<i32, String>(n)
        },
        Some(2),
    );
    memoized.call(1).unwrap();
    memoized.call(2).unwrap();
    memoized.call(3).unwrap();
    assert_eq!(memoized.cache_size(), 2);
    assert!(!memoized.cache_has(&1));
    assert!(memoized.cache_has(&2));
    assert!(memoized.cache_has(&3));
    memoized.call(1).unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 4);
}

#[test]
fn test_memoize_async_max_size_zero_keeps_latest() {
    let memoized = umt_memoize_async(|n: i32| Ok::<i32, String>(n), Some(0));
    memoized.call(1).unwrap();
    memoized.call(2).unwrap();
    assert_eq!(memoized.cache_size(), 1);
    assert!(memoized.cache_has(&2));
}

#[test]
fn test_memoize_async_cache_clear() {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let memoized = umt_memoize_async(
        move |n: i32| {
            counter.fetch_add(1, Ordering::SeqCst);
            Ok::<i32, String>(n)
        },
        Some(1),
    );
    memoized.call(1).unwrap();
    memoized.cache_clear();
    assert_eq!(memoized.cache_size(), 0);
    memoized.call(1).unwrap();
    memoized.call(2).unwrap();
    assert_eq!(memoized.cache_size(), 1);
    assert_eq!(calls.load(Ordering::SeqCst), 3);
}

#[test]
fn test_memoize_async_panic_propagates_to_leader_and_waiters() {
    let barrier = Arc::new(Barrier::new(2));
    let gate = Arc::clone(&barrier);
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let memoized = Arc::new(umt_memoize_async(
        move |n: i32| -> Result<i32, String> {
            if counter.fetch_add(1, Ordering::SeqCst) == 0 {
                gate.wait();
                thread::sleep(Duration::from_millis(50));
                panic!("boom");
            }
            Ok(n)
        },
        None,
    ));

    let leader = {
        let memoized = Arc::clone(&memoized);
        thread::spawn(move || memoized.call(1))
    };
    barrier.wait();
    let follower = catch_unwind(AssertUnwindSafe(|| memoized.call(1)));
    assert!(leader.join().is_err());
    assert!(follower.is_err());
    assert!(!memoized.cache_has(&1));
    assert_eq!(memoized.call(1), Ok(1));
}
