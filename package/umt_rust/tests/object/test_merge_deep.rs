use std::collections::HashMap;
use umt_rust::object::{Value, umt_merge_deep};

#[test]
fn test_should_merge_deep_objects() {
    // { a: { b: 1 } }
    let mut inner1 = HashMap::new();
    inner1.insert("b".to_string(), Value::Int(1));
    let mut map1 = HashMap::new();
    map1.insert("a".to_string(), Value::Object(inner1));
    let obj1 = Value::Object(map1);

    // { a: { c: 2 } }
    let mut inner2 = HashMap::new();
    inner2.insert("c".to_string(), Value::Int(2));
    let mut map2 = HashMap::new();
    map2.insert("a".to_string(), Value::Object(inner2));
    let obj2 = Value::Object(map2);

    let result = umt_merge_deep(&obj1, &[obj2]);

    // Expected: { a: { b: 1, c: 2 } }
    if let Value::Object(res_map) = result {
        if let Some(Value::Object(a)) = res_map.get("a") {
            assert_eq!(a.get("b"), Some(&Value::Int(1)));
            assert_eq!(a.get("c"), Some(&Value::Int(2)));
        } else {
            panic!("Expected nested object 'a'");
        }
    } else {
        panic!("Expected object");
    }
}
