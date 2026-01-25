use std::collections::HashMap;
use umt_rust::object::Value;

/// Helper function to deep merge HashMaps
fn merge_deep(
    target: &HashMap<String, Value>,
    sources: &[&HashMap<String, Value>],
) -> HashMap<String, Value> {
    let mut result = target.clone();
    for source in sources {
        for (key, value) in source.iter() {
            if let (Some(Value::Object(existing)), Value::Object(new_obj)) =
                (result.get(key), value)
            {
                // Both are objects, merge deeply
                result.insert(key.clone(), Value::Object(merge_deep(existing, &[new_obj])));
            } else {
                // Otherwise, just override
                result.insert(key.clone(), value.clone());
            }
        }
    }
    result
}

#[test]
fn test_should_deeply_merge_nested_objects() {
    let mut target_b = HashMap::new();
    target_b.insert("c".to_string(), Value::Int(2));
    target_b.insert("d".to_string(), Value::Int(3));

    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));
    target.insert("b".to_string(), Value::Object(target_b));

    let mut source_b = HashMap::new();
    source_b.insert("d".to_string(), Value::Int(4));
    source_b.insert("e".to_string(), Value::Int(5));

    let mut source = HashMap::new();
    source.insert("b".to_string(), Value::Object(source_b));
    source.insert("f".to_string(), Value::Int(6));

    let result = merge_deep(&target, &[&source]);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("f"), Some(&Value::Int(6)));

    if let Some(Value::Object(b)) = result.get("b") {
        assert_eq!(b.get("c"), Some(&Value::Int(2)));
        assert_eq!(b.get("d"), Some(&Value::Int(4)));
        assert_eq!(b.get("e"), Some(&Value::Int(5)));
    } else {
        panic!("Expected b to be an object");
    }
}

#[test]
fn test_should_not_modify_original_objects() {
    let mut target_a = HashMap::new();
    target_a.insert("b".to_string(), Value::Int(1));

    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Object(target_a));

    let mut source_a = HashMap::new();
    source_a.insert("c".to_string(), Value::Int(2));

    let mut source = HashMap::new();
    source.insert("a".to_string(), Value::Object(source_a));

    let target_clone = target.clone();
    let source_clone = source.clone();

    let result = merge_deep(&target, &[&source]);

    assert_eq!(target, target_clone);
    assert_eq!(source, source_clone);

    if let Some(Value::Object(a)) = result.get("a") {
        assert_eq!(a.get("b"), Some(&Value::Int(1)));
        assert_eq!(a.get("c"), Some(&Value::Int(2)));
    } else {
        panic!("Expected a to be an object");
    }
}

#[test]
fn test_should_handle_multiple_levels_of_nesting() {
    // Create target: { level1: { level2: { level3: { a: 1 } } } }
    let mut level3_target = HashMap::new();
    level3_target.insert("a".to_string(), Value::Int(1));

    let mut level2_target = HashMap::new();
    level2_target.insert("level3".to_string(), Value::Object(level3_target));

    let mut level1_target = HashMap::new();
    level1_target.insert("level2".to_string(), Value::Object(level2_target));

    let mut target = HashMap::new();
    target.insert("level1".to_string(), Value::Object(level1_target));

    // Create source: { level1: { level2: { level3: { b: 2 }, c: 3 } } }
    let mut level3_source = HashMap::new();
    level3_source.insert("b".to_string(), Value::Int(2));

    let mut level2_source = HashMap::new();
    level2_source.insert("level3".to_string(), Value::Object(level3_source));
    level2_source.insert("c".to_string(), Value::Int(3));

    let mut level1_source = HashMap::new();
    level1_source.insert("level2".to_string(), Value::Object(level2_source));

    let mut source = HashMap::new();
    source.insert("level1".to_string(), Value::Object(level1_source));

    let result = merge_deep(&target, &[&source]);

    // Verify the deep structure
    if let Some(Value::Object(l1)) = result.get("level1") {
        if let Some(Value::Object(l2)) = l1.get("level2") {
            assert_eq!(l2.get("c"), Some(&Value::Int(3)));
            if let Some(Value::Object(l3)) = l2.get("level3") {
                assert_eq!(l3.get("a"), Some(&Value::Int(1)));
                assert_eq!(l3.get("b"), Some(&Value::Int(2)));
            } else {
                panic!("Expected level3 to be an object");
            }
        } else {
            panic!("Expected level2 to be an object");
        }
    } else {
        panic!("Expected level1 to be an object");
    }
}

#[test]
fn test_should_handle_non_object_values() {
    let mut target_a = HashMap::new();
    target_a.insert("b".to_string(), Value::Int(1));

    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Object(target_a));

    let mut source = HashMap::new();
    source.insert("a".to_string(), Value::String("string".to_string()));

    let result = merge_deep(&target, &[&source]);

    assert_eq!(result.get("a"), Some(&Value::String("string".to_string())));
}

#[test]
fn test_should_handle_arrays_as_values() {
    let mut target_b = HashMap::new();
    target_b.insert("c".to_string(), Value::Array(vec![Value::Int(3)]));

    let mut target = HashMap::new();
    target.insert(
        "a".to_string(),
        Value::Array(vec![Value::Int(1), Value::Int(2)]),
    );
    target.insert("b".to_string(), Value::Object(target_b));

    let mut source_b = HashMap::new();
    source_b.insert("c".to_string(), Value::Array(vec![Value::Int(6)]));

    let mut source = HashMap::new();
    source.insert(
        "a".to_string(),
        Value::Array(vec![Value::Int(4), Value::Int(5)]),
    );
    source.insert("b".to_string(), Value::Object(source_b));

    let result = merge_deep(&target, &[&source]);

    // Arrays should be replaced, not merged
    assert_eq!(
        result.get("a"),
        Some(&Value::Array(vec![Value::Int(4), Value::Int(5)]))
    );

    if let Some(Value::Object(b)) = result.get("b") {
        assert_eq!(b.get("c"), Some(&Value::Array(vec![Value::Int(6)])));
    } else {
        panic!("Expected b to be an object");
    }
}

#[test]
fn test_should_handle_empty_objects() {
    let target: HashMap<String, Value> = HashMap::new();

    let mut source_a = HashMap::new();
    source_a.insert("b".to_string(), Value::Int(1));

    let mut source = HashMap::new();
    source.insert("a".to_string(), Value::Object(source_a));

    let result = merge_deep(&target, &[&source]);

    if let Some(Value::Object(a)) = result.get("a") {
        assert_eq!(a.get("b"), Some(&Value::Int(1)));
    } else {
        panic!("Expected a to be an object");
    }
}

#[test]
fn test_should_handle_multiple_sources() {
    let mut target_a = HashMap::new();
    target_a.insert("b".to_string(), Value::Int(1));

    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Object(target_a));

    let mut source1_a = HashMap::new();
    source1_a.insert("c".to_string(), Value::Int(2));

    let mut source1 = HashMap::new();
    source1.insert("a".to_string(), Value::Object(source1_a));

    let mut source2_a = HashMap::new();
    source2_a.insert("d".to_string(), Value::Int(3));

    let mut source2 = HashMap::new();
    source2.insert("a".to_string(), Value::Object(source2_a));

    let result = merge_deep(&target, &[&source1, &source2]);

    if let Some(Value::Object(a)) = result.get("a") {
        assert_eq!(a.get("b"), Some(&Value::Int(1)));
        assert_eq!(a.get("c"), Some(&Value::Int(2)));
        assert_eq!(a.get("d"), Some(&Value::Int(3)));
    } else {
        panic!("Expected a to be an object");
    }
}

#[test]
fn test_should_handle_null_values() {
    let mut target_a = HashMap::new();
    target_a.insert("b".to_string(), Value::Int(1));

    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Object(target_a));

    let mut source = HashMap::new();
    source.insert("a".to_string(), Value::Null);

    let result = merge_deep(&target, &[&source]);

    assert_eq!(result.get("a"), Some(&Value::Null));
}

#[test]
fn test_should_handle_no_sources_provided() {
    let mut target_b = HashMap::new();
    target_b.insert("c".to_string(), Value::Int(2));

    let mut target = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));
    target.insert("b".to_string(), Value::Object(target_b));

    let result = merge_deep(&target, &[]);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    if let Some(Value::Object(b)) = result.get("b") {
        assert_eq!(b.get("c"), Some(&Value::Int(2)));
    } else {
        panic!("Expected b to be an object");
    }
}
