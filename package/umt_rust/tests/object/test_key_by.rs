use std::collections::HashMap;
use umt_rust::object::{Value, umt_key_by};

#[test]
fn test_should_key_by_property() {
    let mut obj1 = HashMap::new();
    obj1.insert("id".to_string(), Value::String("a".to_string()));
    obj1.insert("code".to_string(), Value::Int(97));

    let mut obj2 = HashMap::new();
    obj2.insert("id".to_string(), Value::String("b".to_string()));
    obj2.insert("code".to_string(), Value::Int(98));

    let collection = Value::Array(vec![Value::Object(obj1), Value::Object(obj2)]);

    let result = umt_key_by(&collection, "id");

    if let Value::Object(res_map) = result {
        assert!(res_map.contains_key("a"));
        assert!(res_map.contains_key("b"));

        if let Some(Value::Object(a)) = res_map.get("a") {
             assert_eq!(a.get("code"), Some(&Value::Int(97)));
        } else {
            panic!("Expected object at key 'a'");
        }
    } else {
        panic!("Expected object");
    }
}
