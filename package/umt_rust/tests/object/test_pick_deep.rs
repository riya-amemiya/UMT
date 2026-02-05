use std::collections::HashMap;
use umt_rust::object::{Value, umt_pick_deep};

#[test]
fn test_should_select_nested_keys() {
    // { a: { b: { c: 1, d: 2 } }, e: 3 }
    let mut level3 = HashMap::new();
    level3.insert("c".to_string(), Value::Int(1));
    level3.insert("d".to_string(), Value::Int(2));

    let mut level2 = HashMap::new();
    level2.insert("b".to_string(), Value::Object(level3));

    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Object(level2));
    map.insert("e".to_string(), Value::Int(3));

    let obj = Value::Object(map);

    let result = umt_pick_deep(&obj, &["a.b.c", "e"]);

    if let Value::Object(res_map) = result {
        assert_eq!(res_map.get("e"), Some(&Value::Int(3)));

        if let Some(Value::Object(a)) = res_map.get("a") {
            if let Some(Value::Object(b)) = a.get("b") {
                assert_eq!(b.get("c"), Some(&Value::Int(1)));
                assert_eq!(b.get("d"), None);
            } else {
                panic!("Expected 'b'");
            }
        } else {
             panic!("Expected 'a'");
        }
    } else {
        panic!("Expected object");
    }
}
