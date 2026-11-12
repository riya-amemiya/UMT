use umt_rust::data_structure::LRUCache;

#[test]
fn test_lru_order_delete_head_then_evict_next_oldest() {
    let mut cache = LRUCache::new(3);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("c", 3);
    assert!(cache.delete(&"a"));
    cache.set("d", 4);
    cache.set("e", 5);
    assert!(!cache.has(&"b"));
    assert_eq!(cache.get(&"c"), Some(&3));
    assert_eq!(cache.get(&"d"), Some(&4));
    assert_eq!(cache.get(&"e"), Some(&5));
    assert_eq!(cache.size(), 3);
}

#[test]
fn test_lru_order_delete_middle_keeps_neighbors_linked() {
    let mut cache = LRUCache::new(3);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("c", 3);
    assert!(cache.delete(&"b"));
    cache.set("d", 4);
    cache.set("e", 5);
    assert!(!cache.has(&"a"));
    assert!(cache.has(&"c"));
    assert!(cache.has(&"d"));
    assert!(cache.has(&"e"));
}

#[test]
fn test_lru_order_delete_tail_then_insert() {
    let mut cache = LRUCache::new(3);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("c", 3);
    assert!(cache.delete(&"c"));
    cache.set("d", 4);
    cache.set("e", 5);
    assert!(!cache.has(&"a"));
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"d"), Some(&4));
    assert_eq!(cache.get(&"e"), Some(&5));
}

#[test]
fn test_lru_order_delete_only_entry_resets_list() {
    let mut cache = LRUCache::new(2);
    cache.set("a", 1);
    assert!(cache.delete(&"a"));
    assert_eq!(cache.size(), 0);
    cache.set("b", 2);
    cache.set("c", 3);
    cache.set("d", 4);
    assert!(!cache.has(&"b"));
    assert_eq!(cache.get(&"c"), Some(&3));
    assert_eq!(cache.get(&"d"), Some(&4));
}

#[test]
fn test_lru_order_get_refreshes_head_middle_and_tail() {
    let mut cache = LRUCache::new(3);
    cache.set(1, "one");
    cache.set(2, "two");
    cache.set(3, "three");
    assert_eq!(cache.get(&1), Some(&"one"));
    assert_eq!(cache.get(&3), Some(&"three"));
    assert_eq!(cache.get(&3), Some(&"three"));
    cache.set(4, "four");
    assert!(!cache.has(&2));
    cache.set(5, "five");
    assert!(!cache.has(&1));
    assert!(cache.has(&3));
    assert!(cache.has(&4));
    assert!(cache.has(&5));
}

#[test]
fn test_lru_order_update_refreshes_recency_and_value() {
    let mut cache = LRUCache::new(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.set("a", 10);
    cache.set("c", 3);
    assert!(!cache.has(&"b"));
    assert_eq!(cache.get(&"a"), Some(&10));
    assert_eq!(cache.size(), 2);
}

#[test]
fn test_lru_order_zero_capacity_keeps_latest_entry() {
    let mut cache = LRUCache::new(0);
    cache.set("a", 1);
    assert_eq!(cache.size(), 1);
    cache.set("b", 2);
    assert_eq!(cache.size(), 1);
    assert!(!cache.has(&"a"));
    assert_eq!(cache.get(&"b"), Some(&2));
}

#[test]
fn test_lru_order_clear_then_reuse() {
    let mut cache = LRUCache::new(2);
    cache.set("a", 1);
    cache.set("b", 2);
    cache.clear();
    assert_eq!(cache.size(), 0);
    assert_eq!(cache.get(&"a"), None);
    assert!(!cache.delete(&"a"));
    cache.set("c", 3);
    cache.set("d", 4);
    cache.set("e", 5);
    assert!(!cache.has(&"c"));
    assert!(cache.has(&"d"));
    assert!(cache.has(&"e"));
}

#[test]
fn test_lru_order_matches_reference_model_under_mixed_operations() {
    let capacity = 8;
    let mut cache = LRUCache::new(capacity);
    let mut model: Vec<(u32, u32)> = Vec::new();
    let mut seed: u32 = 12345;
    for step in 0..5000u32 {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let key = (seed >> 16) % 20;
        match (seed >> 8) % 4 {
            0 | 1 => {
                if let Some(pos) = model.iter().position(|(k, _)| *k == key) {
                    model.remove(pos);
                } else if model.len() >= capacity {
                    model.remove(0);
                }
                model.push((key, step));
                cache.set(key, step);
            }
            2 => {
                let expected = model.iter().position(|(k, _)| *k == key).map(|pos| {
                    let entry = model.remove(pos);
                    model.push(entry);
                    entry.1
                });
                assert_eq!(cache.get(&key).copied(), expected);
            }
            _ => {
                let expected = model
                    .iter()
                    .position(|(k, _)| *k == key)
                    .map(|pos| model.remove(pos))
                    .is_some();
                assert_eq!(cache.delete(&key), expected);
            }
        }
        assert_eq!(cache.size(), model.len());
    }
    for (key, value) in &model {
        assert!(cache.has(key));
        assert_eq!(cache.get(key), Some(value));
    }
}
