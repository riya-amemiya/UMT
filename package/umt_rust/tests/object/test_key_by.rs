use std::collections::HashMap;
use umt_rust::object::{Value, umt_key_by};

#[test]
fn test_key_by_string() {
    let mut map1 = HashMap::new();
    map1.insert("id".to_string(), Value::String("a".to_string()));
    map1.insert("val".to_string(), Value::Int(1));

    let mut map2 = HashMap::new();
    map2.insert("id".to_string(), Value::String("b".to_string()));
    map2.insert("val".to_string(), Value::Int(2));

    let items = Value::Array(vec![Value::Object(map1), Value::Object(map2)]);

    let result = umt_key_by(&items, "id");

    assert!(result.get("a").is_some());
    assert!(result.get("b").is_some());
}
