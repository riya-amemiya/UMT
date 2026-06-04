use umt_rust::map::umt_zip_to_map;

#[test]
fn test_should_pair_keys_and_values_at_the_same_index() {
    let result = umt_zip_to_map(&["a", "b", "c"], &[1, 2, 3]);
    assert_eq!(result, vec![("a", 1), ("b", 2), ("c", 3)]);
}

#[test]
fn test_should_stop_at_the_shorter_length_when_keys_is_longer() {
    let result = umt_zip_to_map(&["a", "b", "c"], &[1, 2]);
    assert_eq!(result, vec![("a", 1), ("b", 2)]);
}

#[test]
fn test_should_stop_at_the_shorter_length_when_values_is_longer() {
    let result = umt_zip_to_map(&["a", "b"], &[1, 2, 3]);
    assert_eq!(result, vec![("a", 1), ("b", 2)]);
}

#[test]
fn test_should_keep_the_latest_value_for_duplicate_keys() {
    let result = umt_zip_to_map(&["a", "b", "a"], &[1, 2, 3]);
    let a = result
        .iter()
        .find(|(key, _)| *key == "a")
        .map(|(_, value)| *value);
    let b = result
        .iter()
        .find(|(key, _)| *key == "b")
        .map(|(_, value)| *value);
    assert_eq!(a, Some(3));
    assert_eq!(b, Some(2));
    assert_eq!(result.len(), 2);
}

#[test]
fn test_should_accept_object_keys() {
    #[derive(Clone, PartialEq, Debug)]
    struct Key {
        id: i32,
    }
    let k1 = Key { id: 1 };
    let k2 = Key { id: 2 };
    let result = umt_zip_to_map(&[k1.clone(), k2.clone()], &["x", "y"]);
    let v1 = result
        .iter()
        .find(|(key, _)| *key == k1)
        .map(|(_, value)| *value);
    let v2 = result
        .iter()
        .find(|(key, _)| *key == k2)
        .map(|(_, value)| *value);
    assert_eq!(v1, Some("x"));
    assert_eq!(v2, Some("y"));
}

#[test]
fn test_should_return_an_empty_map_when_either_input_is_empty() {
    let empty_keys: Vec<&str> = vec![];
    let empty_values: Vec<i32> = vec![];
    assert_eq!(umt_zip_to_map(&empty_keys, &[1, 2]).len(), 0);
    assert_eq!(umt_zip_to_map(&["a"], &empty_values).len(), 0);
}

#[test]
fn test_should_preserve_insertion_order_of_keys() {
    let result = umt_zip_to_map(&["c", "a", "b"], &[1, 2, 3]);
    let keys: Vec<&str> = result.iter().map(|(key, _)| *key).collect();
    assert_eq!(keys, vec!["c", "a", "b"]);
}
