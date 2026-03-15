use umt_rust::data_structure::LRUCache;

#[test]
fn test_constructor() {
    let cache = LRUCache::<String, i32>::new(3);
    assert_eq!(cache.size(), 0);
}

#[test]
fn test_set_and_get() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("b".to_string(), 2);
    cache.set("c".to_string(), 3);

    assert_eq!(cache.get(&"a".to_string()), Some(&1));
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
    assert_eq!(cache.get(&"c".to_string()), Some(&3));
}

#[test]
fn test_missing_keys() {
    let mut cache = LRUCache::<String, i32>::new(3);
    assert_eq!(cache.get(&"missing".to_string()), None);
}

#[test]
fn test_update_existing_key_value() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("a".to_string(), 10);
    assert_eq!(cache.get(&"a".to_string()), Some(&10));
    assert_eq!(cache.size(), 1);
}

#[test]
fn test_eviction() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("b".to_string(), 2);
    cache.set("c".to_string(), 3);
    cache.set("d".to_string(), 4);

    assert_eq!(cache.has(&"a".to_string()), false);
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
    assert_eq!(cache.get(&"c".to_string()), Some(&3));
    assert_eq!(cache.get(&"d".to_string()), Some(&4));
    assert_eq!(cache.size(), 3);
}

#[test]
fn test_promote_accessed_entries() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("b".to_string(), 2);
    cache.set("c".to_string(), 3);

    cache.get(&"a".to_string());
    cache.set("d".to_string(), 4);

    assert_eq!(cache.has(&"a".to_string()), true);
    assert_eq!(cache.has(&"b".to_string()), false);
    assert_eq!(cache.has(&"c".to_string()), true);
    assert_eq!(cache.has(&"d".to_string()), true);
}

#[test]
fn test_promote_updated_entries() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("b".to_string(), 2);
    cache.set("c".to_string(), 3);

    cache.set("a".to_string(), 10);
    cache.set("d".to_string(), 4);

    assert_eq!(cache.has(&"a".to_string()), true);
    assert_eq!(cache.get(&"a".to_string()), Some(&10));
    assert_eq!(cache.has(&"b".to_string()), false);
}

#[test]
fn test_has() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    assert_eq!(cache.has(&"a".to_string()), true);
    assert_eq!(cache.has(&"missing".to_string()), false);
}

#[test]
fn test_delete() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("b".to_string(), 2);

    assert_eq!(cache.delete(&"a".to_string()), true);
    assert_eq!(cache.has(&"a".to_string()), false);
    assert_eq!(cache.size(), 1);
}

#[test]
fn test_delete_missing_key() {
    let mut cache = LRUCache::<String, i32>::new(3);
    assert_eq!(cache.delete(&"missing".to_string()), false);
}

#[test]
fn test_delete_head_node() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("b".to_string(), 2);
    cache.set("c".to_string(), 3);

    assert_eq!(cache.delete(&"c".to_string()), true);
    assert_eq!(cache.size(), 2);
    assert_eq!(cache.get(&"a".to_string()), Some(&1));
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
}

#[test]
fn test_delete_tail_node() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("b".to_string(), 2);
    cache.set("c".to_string(), 3);

    assert_eq!(cache.delete(&"a".to_string()), true);
    assert_eq!(cache.size(), 2);
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
    assert_eq!(cache.get(&"c".to_string()), Some(&3));
}

#[test]
fn test_delete_only_entry() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    assert_eq!(cache.delete(&"a".to_string()), true);
    assert_eq!(cache.size(), 0);
}

#[test]
fn test_clear() {
    let mut cache = LRUCache::<String, i32>::new(3);
    cache.set("a".to_string(), 1);
    cache.set("b".to_string(), 2);
    cache.set("c".to_string(), 3);

    cache.clear();

    assert_eq!(cache.size(), 0);
    assert_eq!(cache.get(&"a".to_string()), None);
    assert_eq!(cache.get(&"b".to_string()), None);
    assert_eq!(cache.get(&"c".to_string()), None);
}

#[test]
fn test_size() {
    let mut cache = LRUCache::<String, i32>::new(5);
    assert_eq!(cache.size(), 0);

    cache.set("a".to_string(), 1);
    assert_eq!(cache.size(), 1);

    cache.set("b".to_string(), 2);
    assert_eq!(cache.size(), 2);

    cache.delete(&"a".to_string());
    assert_eq!(cache.size(), 1);
}

#[test]
fn test_capacity_of_1() {
    let mut cache = LRUCache::<String, i32>::new(1);
    cache.set("a".to_string(), 1);
    assert_eq!(cache.get(&"a".to_string()), Some(&1));

    cache.set("b".to_string(), 2);
    assert_eq!(cache.get(&"a".to_string()), None);
    assert_eq!(cache.get(&"b".to_string()), Some(&2));
    assert_eq!(cache.size(), 1);
}

#[test]
fn test_numeric_keys() {
    let mut cache = LRUCache::<i32, String>::new(3);
    cache.set(1, "one".to_string());
    cache.set(2, "two".to_string());
    cache.set(3, "three".to_string());

    assert_eq!(cache.get(&1), Some(&"one".to_string()));
    assert_eq!(cache.get(&2), Some(&"two".to_string()));
    assert_eq!(cache.get(&3), Some(&"three".to_string()));
}

#[test]
fn test_handle_many_operations() {
    let mut cache = LRUCache::<i32, i32>::new(100);
    for i in 0..1000 {
        cache.set(i, i * 2);
    }
    assert_eq!(cache.size(), 100);

    for i in 900..1000 {
        assert_eq!(cache.get(&i), Some(&(i * 2)));
    }

    assert_eq!(cache.get(&0), None);
}
