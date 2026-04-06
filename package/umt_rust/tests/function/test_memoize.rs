use std::cell::Cell;
use std::rc::Rc;
use umt_rust::function::{umt_memoize, umt_memoize_with_resolver};

#[test]
fn test_memoize_caches_result_for_same_argument() {
    let call_count = Rc::new(Cell::new(0));
    let cc = call_count.clone();
    let memoized = umt_memoize(
        move |n: i32| {
            cc.set(cc.get() + 1);
            n * 2
        },
        None,
    );

    assert_eq!(memoized.call(5), 10);
    assert_eq!(memoized.call(5), 10);
    assert_eq!(call_count.get(), 1);
}

#[test]
fn test_memoize_computes_for_different_arguments() {
    let call_count = Rc::new(Cell::new(0));
    let cc = call_count.clone();
    let memoized = umt_memoize(
        move |n: i32| {
            cc.set(cc.get() + 1);
            n * 2
        },
        None,
    );

    assert_eq!(memoized.call(5), 10);
    assert_eq!(memoized.call(10), 20);
    assert_eq!(call_count.get(), 2);
}

#[test]
fn test_memoize_exposes_cache_size() {
    let memoized = umt_memoize(|n: i32| n * 2, None);

    memoized.call(5);
    assert_eq!(memoized.cache_size(), 1);
    assert_eq!(memoized.cache_get(&5), Some(10));
}

#[test]
fn test_memoize_evicts_oldest_entry_when_max_size_exceeded() {
    let call_count = Rc::new(Cell::new(0));
    let cc = call_count.clone();
    let memoized = umt_memoize(
        move |n: i32| {
            cc.set(cc.get() + 1);
            n * 2
        },
        Some(2),
    );

    memoized.call(1);
    memoized.call(2);
    memoized.call(3);

    assert_eq!(memoized.cache_size(), 2);
    assert!(!memoized.cache_has(&1));
    assert!(memoized.cache_has(&2));
    assert!(memoized.cache_has(&3));
}

#[test]
fn test_memoize_with_resolver_uses_custom_key() {
    let call_count = Rc::new(Cell::new(0));
    let cc = call_count.clone();
    let memoized = umt_memoize_with_resolver(
        move |(a, b): (i32, i32)| {
            cc.set(cc.get() + 1);
            a + b
        },
        |&(a, b): &(i32, i32)| format!("{}-{}", a, b),
        None,
    );

    assert_eq!(memoized.call((1, 2)), 3);
    assert_eq!(memoized.call((1, 2)), 3);
    assert_eq!(call_count.get(), 1);

    assert_eq!(memoized.call((2, 1)), 3);
    assert_eq!(call_count.get(), 2);
}

#[test]
fn test_memoize_with_string_keys() {
    let call_count = Rc::new(Cell::new(0));
    let cc = call_count.clone();
    let memoized = umt_memoize(
        move |s: String| {
            cc.set(cc.get() + 1);
            s.to_uppercase()
        },
        None,
    );

    assert_eq!(memoized.call("hello".to_string()), "HELLO");
    assert_eq!(memoized.call("hello".to_string()), "HELLO");
    assert_eq!(call_count.get(), 1);
}

#[test]
fn test_memoize_with_resolver_and_max_size() {
    let call_count = Rc::new(Cell::new(0));
    let cc = call_count.clone();
    let memoized = umt_memoize_with_resolver(
        move |(a, b): (i32, i32)| {
            cc.set(cc.get() + 1);
            a + b
        },
        |&(a, b): &(i32, i32)| format!("{}-{}", a, b),
        Some(2),
    );

    memoized.call((1, 2));
    memoized.call((3, 4));
    memoized.call((5, 6));

    assert_eq!(memoized.cache_size(), 2);
    assert!(!memoized.cache_has(&"1-2".to_string()));
    assert!(memoized.cache_has(&"3-4".to_string()));
    assert!(memoized.cache_has(&"5-6".to_string()));
}

#[test]
fn test_memoize_cache_clear() {
    let memoized = umt_memoize(|n: i32| n * 2, None);

    memoized.call(1);
    memoized.call(2);
    assert_eq!(memoized.cache_size(), 2);

    memoized.cache_clear();
    assert_eq!(memoized.cache_size(), 0);
}
