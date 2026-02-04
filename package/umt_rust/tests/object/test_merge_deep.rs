use std::collections::HashMap;
use umt_rust::object::{Value, umt_merge_deep};

#[test]
fn test_should_deeply_merge_objects() {
    let mut inner1 = HashMap::new();
    inner1.insert("a".to_string(), Value::Int(1));
    let mut map1 = HashMap::new();
    map1.insert("nested".to_string(), Value::Object(inner1));
    let obj1 = Value::Object(map1);

    let mut inner2 = HashMap::new();
    inner2.insert("b".to_string(), Value::Int(2));
    let mut map2 = HashMap::new();
    map2.insert("nested".to_string(), Value::Object(inner2));
    let obj2 = Value::Object(map2);

    let result = umt_merge_deep(&obj1, &[obj2]);

    if let Some(Value::Object(nested)) = result.get("nested") {
        assert_eq!(nested.get("a"), Some(&Value::Int(1)));
        assert_eq!(nested.get("b"), Some(&Value::Int(2)));
    } else {
        panic!("Nested object missing");
    }
}
