use std::collections::HashMap;
use umt_rust::object::{Value, umt_unflatten_object};

fn nested_get<'a>(map: &'a HashMap<String, Value>, path: &[&str]) -> Option<&'a Value> {
    let mut current = map;
    for (index, key) in path.iter().enumerate() {
        let value = current.get(*key)?;
        if index == path.len() - 1 {
            return Some(value);
        }
        current = value.as_object()?;
    }
    None
}

#[test]
fn test_rebuilds_a_nested_object_from_dotted_paths() {
    let mut flat = HashMap::new();
    flat.insert("a.b.c".to_string(), Value::Int(1));

    let result = umt_unflatten_object(&flat, ".");
    assert_eq!(nested_get(&result, &["a", "b", "c"]), Some(&Value::Int(1)));
}

#[test]
fn test_merges_multiple_paths_into_shared_parents() {
    let mut flat = HashMap::new();
    flat.insert("a.b".to_string(), Value::Int(1));
    flat.insert("a.c".to_string(), Value::Int(2));

    let result = umt_unflatten_object(&flat, ".");
    assert_eq!(nested_get(&result, &["a", "b"]), Some(&Value::Int(1)));
    assert_eq!(nested_get(&result, &["a", "c"]), Some(&Value::Int(2)));
}

#[test]
fn test_supports_custom_separator() {
    let mut flat = HashMap::new();
    flat.insert("a/b".to_string(), Value::Int(1));

    let result = umt_unflatten_object(&flat, "/");
    assert_eq!(nested_get(&result, &["a", "b"]), Some(&Value::Int(1)));
}

#[test]
fn test_preserves_top_level_scalars() {
    let mut flat = HashMap::new();
    flat.insert("a".to_string(), Value::Int(1));
    flat.insert("b".to_string(), Value::Int(2));

    let result = umt_unflatten_object(&flat, ".");
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(2)));
}

#[test]
fn test_returns_empty_for_empty_input() {
    let flat: HashMap<String, Value> = HashMap::new();
    let result = umt_unflatten_object(&flat, ".");
    assert!(result.is_empty());
}
