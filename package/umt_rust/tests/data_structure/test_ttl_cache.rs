use std::cell::Cell;
use std::rc::Rc;
use umt_rust::data_structure::TTLCache;

/// Helper to create a TTLCache with a controllable mock clock.
fn make_cache_with_mock_clock(
    default_ttl_ms: u64,
    max_size: Option<usize>,
) -> (TTLCache<String, i32>, Rc<Cell<u64>>) {
    let time = Rc::new(Cell::new(1_000_000u64)); // start at 1_000_000ms
    let time_clone = Rc::clone(&time);
    let clock = Box::new(move || time_clone.get());
    let cache = TTLCache::with_clock(default_ttl_ms, max_size, clock);
    (cache, time)
}

#[test]
fn test_constructor() {
    let cache = TTLCache::<String, i32>::new(5000, None);
    assert_eq!(cache.size(), 0);
}

#[test]
fn test_set_and_get() {
    let mut cache = TTLCache::<String, i32>::new(5000, None);
    cache.set("a".to_string(), 1, None);
    cache.set("b".to_string(), 2, None);

    assert_eq!(cache.get(&"a".to_string()), Some(&1));
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
}

#[test]
fn test_get_missing_key() {
    let mut cache = TTLCache::<String, i32>::new(5000, None);
    assert_eq!(cache.get(&"missing".to_string()), None);
}

#[test]
fn test_update_existing_key() {
    let mut cache = TTLCache::<String, i32>::new(5000, None);
    cache.set("a".to_string(), 1, None);
    cache.set("a".to_string(), 10, None);
    assert_eq!(cache.get(&"a".to_string()), Some(&10));
    assert_eq!(cache.size(), 1);
}

#[test]
fn test_ttl_expiration() {
    let (mut cache, time) = make_cache_with_mock_clock(5000, None);
    cache.set("a".to_string(), 1, None);

    assert_eq!(cache.get(&"a".to_string()), Some(&1));

    // Advance time by 5000ms
    time.set(time.get() + 5000);

    assert_eq!(cache.get(&"a".to_string()), None);
}

#[test]
fn test_not_expire_before_ttl() {
    let (mut cache, time) = make_cache_with_mock_clock(5000, None);
    cache.set("a".to_string(), 1, None);

    // Advance time by 4999ms
    time.set(time.get() + 4999);

    assert_eq!(cache.get(&"a".to_string()), Some(&1));
}

#[test]
fn test_per_entry_ttl_override() {
    let (mut cache, time) = make_cache_with_mock_clock(5000, None);
    cache.set("short".to_string(), 1, Some(1000));
    cache.set("long".to_string(), 2, Some(10000));

    // Advance 1000ms
    time.set(time.get() + 1000);

    assert_eq!(cache.get(&"short".to_string()), None);
    assert_eq!(cache.get(&"long".to_string()), Some(&2));

    // Advance another 9000ms
    time.set(time.get() + 9000);

    assert_eq!(cache.get(&"long".to_string()), None);
}

#[test]
fn test_expire_entries_checked_via_has() {
    let (mut cache, time) = make_cache_with_mock_clock(5000, None);
    cache.set("a".to_string(), 1, None);

    assert!(cache.has(&"a".to_string()));

    // Advance 5000ms
    time.set(time.get() + 5000);

    assert!(!cache.has(&"a".to_string()));
}

#[test]
fn test_cleanup_expired_entries_on_get() {
    let (mut cache, time) = make_cache_with_mock_clock(5000, None);
    cache.set("a".to_string(), 1, None);

    // Advance 5000ms
    time.set(time.get() + 5000);

    assert_eq!(cache.size(), 1);
    cache.get(&"a".to_string());
    assert_eq!(cache.size(), 0);
}

#[test]
fn test_cleanup_expired_entries_on_has() {
    let (mut cache, time) = make_cache_with_mock_clock(5000, None);
    cache.set("a".to_string(), 1, None);

    // Advance 5000ms
    time.set(time.get() + 5000);

    assert_eq!(cache.size(), 1);
    cache.has(&"a".to_string());
    assert_eq!(cache.size(), 0);
}

#[test]
fn test_max_size_eviction() {
    let mut cache = TTLCache::<String, i32>::new(5000, Some(2));
    cache.set("a".to_string(), 1, None);
    cache.set("b".to_string(), 2, None);
    cache.set("c".to_string(), 3, None);

    assert_eq!(cache.size(), 2);
    assert_eq!(cache.get(&"a".to_string()), None);
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
    assert_eq!(cache.get(&"c".to_string()), Some(&3));
}

#[test]
fn test_no_eviction_when_updating_existing_key() {
    let mut cache = TTLCache::<String, i32>::new(5000, Some(2));
    cache.set("a".to_string(), 1, None);
    cache.set("b".to_string(), 2, None);
    cache.set("a".to_string(), 10, None);

    assert_eq!(cache.size(), 2);
    assert_eq!(cache.get(&"a".to_string()), Some(&10));
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
}

#[test]
fn test_has() {
    let mut cache = TTLCache::<String, i32>::new(5000, None);
    cache.set("a".to_string(), 1, None);
    assert!(cache.has(&"a".to_string()));
    assert!(!cache.has(&"missing".to_string()));
}

#[test]
fn test_delete() {
    let mut cache = TTLCache::<String, i32>::new(5000, None);
    cache.set("a".to_string(), 1, None);
    assert!(cache.delete(&"a".to_string()));
    assert!(!cache.has(&"a".to_string()));
    assert_eq!(cache.size(), 0);
}

#[test]
fn test_delete_missing_key() {
    let mut cache = TTLCache::<String, i32>::new(5000, None);
    assert!(!cache.delete(&"missing".to_string()));
}

#[test]
fn test_clear() {
    let mut cache = TTLCache::<String, i32>::new(5000, None);
    cache.set("a".to_string(), 1, None);
    cache.set("b".to_string(), 2, None);
    cache.set("c".to_string(), 3, None);

    cache.clear();

    assert_eq!(cache.size(), 0);
    assert_eq!(cache.get(&"a".to_string()), None);
}

#[test]
fn test_max_size_1() {
    let mut cache = TTLCache::<String, i32>::new(5000, Some(1));
    cache.set("a".to_string(), 1, None);
    cache.set("b".to_string(), 2, None);

    assert_eq!(cache.size(), 1);
    assert_eq!(cache.get(&"a".to_string()), None);
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
}

#[test]
fn test_ttl_zero() {
    let (mut cache, _time) = make_cache_with_mock_clock(0, None);
    cache.set("a".to_string(), 1, None);

    assert_eq!(cache.get(&"a".to_string()), None);
}

#[test]
fn test_numeric_keys() {
    let mut cache = TTLCache::<i32, String>::new(5000, None);
    cache.set(1, "one".to_string(), None);
    cache.set(2, "two".to_string(), None);

    assert_eq!(cache.get(&1), Some(&"one".to_string()));
    assert_eq!(cache.get(&2), Some(&"two".to_string()));
}
