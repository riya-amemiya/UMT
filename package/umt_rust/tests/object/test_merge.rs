use std::collections::HashMap;
use umt_rust::object::Value;

/// Helper function to merge HashMaps (shallow merge)
fn merge(
    target: &HashMap<String, Value>,
    sources: &[&HashMap<String, Value>],
) -> HashMap<String, Value> {
    let mut result = target.clone();
    for source in sources {
        for (key, value) in source.iter() {
            result.insert(key.clone(), value.clone());
        }
    }
    result
}

#[test]
fn test_should_merge_multiple_objects() {
    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));
    target.insert("b".to_string(), Value::Int(2));

    let mut source1 = HashMap::new();
    source1.insert("b".to_string(), Value::Int(3));
    source1.insert("c".to_string(), Value::Int(4));

    let mut source2 = HashMap::new();
    source2.insert("c".to_string(), Value::Int(5));
    source2.insert("d".to_string(), Value::Int(6));

    let result = merge(&target, &[&source1, &source2]);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(3)));
    assert_eq!(result.get("c"), Some(&Value::Int(5)));
    assert_eq!(result.get("d"), Some(&Value::Int(6)));
}

#[test]
fn test_should_not_modify_original_objects() {
    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));
    target.insert("b".to_string(), Value::Int(2));

    let mut source = HashMap::new();
    source.insert("b".to_string(), Value::Int(3));
    source.insert("c".to_string(), Value::Int(4));

    let target_clone = target.clone();
    let source_clone = source.clone();

    let result = merge(&target, &[&source]);

    // Original objects should not be modified
    assert_eq!(target, target_clone);
    assert_eq!(source, source_clone);

    // Result should have merged values
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(3)));
    assert_eq!(result.get("c"), Some(&Value::Int(4)));
}

#[test]
fn test_should_handle_empty_target() {
    let target: HashMap<String, Value> = HashMap::new();

    let mut source = HashMap::new();
    source.insert("a".to_string(), Value::Int(1));
    source.insert("b".to_string(), Value::Int(2));

    let result = merge(&target, &[&source]);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(2)));
}

#[test]
fn test_should_handle_no_sources() {
    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));
    target.insert("b".to_string(), Value::Int(2));

    let result = merge(&target, &[]);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(2)));
}

#[test]
fn test_should_handle_empty_sources() {
    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));
    target.insert("b".to_string(), Value::Int(2));

    let source: HashMap<String, Value> = HashMap::new();

    let result = merge(&target, &[&source]);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(2)));
}

#[test]
fn test_should_override_properties_with_later_sources() {
    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));

    let mut source1 = HashMap::new();
    source1.insert("a".to_string(), Value::Int(2));

    let mut source2 = HashMap::new();
    source2.insert("a".to_string(), Value::Int(3));

    let result = merge(&target, &[&source1, &source2]);

    assert_eq!(result.get("a"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_null_values() {
    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));
    target.insert("b".to_string(), Value::Null);

    let mut source = HashMap::new();
    source.insert("c".to_string(), Value::Null);

    let result = merge(&target, &[&source]);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Null));
    assert_eq!(result.get("c"), Some(&Value::Null));
}
