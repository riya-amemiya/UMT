use std::cell::Cell;
use std::rc::Rc;
use umt_rust::data_structure::TTLCache;

fn make_cache(default_ttl_ms: u64, max_size: Option<usize>) -> (TTLCache<u32, u32>, Rc<Cell<u64>>) {
    let now = Rc::new(Cell::new(1000u64));
    let now_clone = now.clone();
    let cache = TTLCache::with_clock(default_ttl_ms, max_size, Box::new(move || now_clone.get()));
    (cache, now)
}

#[test]
fn test_ttl_eviction_skips_deleted_keys() {
    let (mut cache, _now) = make_cache(5000, Some(3));
    cache.set(1, 10, None);
    cache.set(2, 20, None);
    cache.set(3, 30, None);
    assert!(cache.delete(&1));
    cache.set(4, 40, None);
    assert_eq!(cache.size(), 3);
    cache.set(5, 50, None);
    assert_eq!(cache.size(), 3);
    assert!(!cache.has(&2));
    assert_eq!(cache.get(&3), Some(&30));
    assert_eq!(cache.get(&4), Some(&40));
    assert_eq!(cache.get(&5), Some(&50));
}

#[test]
fn test_ttl_eviction_treats_reinserted_key_as_newest() {
    let (mut cache, _now) = make_cache(5000, Some(2));
    cache.set(1, 10, None);
    cache.set(2, 20, None);
    assert!(cache.delete(&1));
    cache.set(1, 11, None);
    cache.set(3, 30, None);
    assert!(!cache.has(&2));
    assert_eq!(cache.get(&1), Some(&11));
    assert_eq!(cache.get(&3), Some(&30));
}

#[test]
fn test_ttl_eviction_skips_keys_removed_by_expiry() {
    let (mut cache, now) = make_cache(5000, Some(2));
    cache.set(1, 10, Some(100));
    cache.set(2, 20, None);
    now.set(1200);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.size(), 1);
    cache.set(3, 30, None);
    assert_eq!(cache.size(), 2);
    cache.set(4, 40, None);
    assert!(!cache.has(&2));
    assert!(cache.has(&3));
    assert!(cache.has(&4));
}

#[test]
fn test_ttl_eviction_update_keeps_insertion_position() {
    let (mut cache, _now) = make_cache(5000, Some(2));
    cache.set(1, 10, None);
    cache.set(2, 20, None);
    cache.set(1, 11, None);
    cache.set(3, 30, None);
    assert!(!cache.has(&1));
    assert_eq!(cache.get(&2), Some(&20));
    assert_eq!(cache.get(&3), Some(&30));
}

#[test]
fn test_ttl_eviction_max_size_zero_keeps_latest_entry() {
    let (mut cache, _now) = make_cache(5000, Some(0));
    cache.set(1, 10, None);
    assert_eq!(cache.size(), 1);
    cache.set(2, 20, None);
    assert_eq!(cache.size(), 1);
    assert!(!cache.has(&1));
    assert_eq!(cache.get(&2), Some(&20));
}

#[test]
fn test_ttl_eviction_survives_many_set_delete_cycles() {
    let (mut cache, _now) = make_cache(5000, Some(4));
    for round in 0..1000u32 {
        cache.set(round, round, None);
        assert!(cache.delete(&round));
    }
    assert_eq!(cache.size(), 0);
    for key in 2000..2006u32 {
        cache.set(key, key, None);
    }
    assert_eq!(cache.size(), 4);
    assert!(!cache.has(&2000));
    assert!(!cache.has(&2001));
    for key in 2002..2006u32 {
        assert_eq!(cache.get(&key), Some(&key));
    }
}

#[test]
fn test_ttl_eviction_unbounded_cache_never_evicts() {
    let (mut cache, _now) = make_cache(5000, None);
    for key in 0..100u32 {
        cache.set(key, key, None);
    }
    assert_eq!(cache.size(), 100);
    assert!(cache.delete(&0));
    assert!(!cache.delete(&0));
    assert_eq!(cache.get(&99), Some(&99));
}

#[test]
fn test_ttl_eviction_clear_resets_order() {
    let (mut cache, _now) = make_cache(5000, Some(2));
    cache.set(1, 10, None);
    cache.set(2, 20, None);
    cache.clear();
    cache.set(3, 30, None);
    cache.set(4, 40, None);
    cache.set(5, 50, None);
    assert!(!cache.has(&3));
    assert!(cache.has(&4));
    assert!(cache.has(&5));
}

#[test]
fn test_ttl_eviction_system_clock_constructor() {
    let mut cache: TTLCache<u32, u32> = TTLCache::new(60_000, Some(1));
    cache.set(1, 10, None);
    cache.set(2, 20, None);
    assert!(!cache.has(&1));
    assert_eq!(cache.get(&2), Some(&20));
}
