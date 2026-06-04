use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_remove_prototype_map_deep};

fn as_map(value: Value) -> HashMap<String, Value> {
    value.into_object().unwrap()
}

#[test]
fn test_should_remove_prototype_polluting_properties_recursively_from_each_object() {
    let objects = vec![
        as_map(obj! {"safe" => obj!{"__proto__" => obj!{"polluted" => true}, "value" => 1}}),
        as_map(obj! {
            "list" => Value::Array(vec![obj!{"prototype" => obj!{"polluted" => true}, "ok" => 1}]),
            "meta" => obj!{"constructor" => obj!{"prototype" => obj!{"polluted" => true}}, "keep" => 2}
        }),
    ];

    let result = umt_remove_prototype_map_deep(&objects);

    let safe = result[0].get("safe").and_then(Value::as_object).unwrap();
    assert_eq!(safe.get("value"), Some(&Value::Int(1)));
    assert_eq!(safe.get("__proto__"), None);

    if let Some(Value::Array(list)) = result[1].get("list") {
        let item = list[0].as_object().unwrap();
        assert_eq!(item.get("ok"), Some(&Value::Int(1)));
        assert_eq!(item.get("prototype"), None);
    } else {
        panic!("list should be an array");
    }
    let meta = result[1].get("meta").and_then(Value::as_object).unwrap();
    assert_eq!(meta.get("keep"), Some(&Value::Int(2)));
    assert_eq!(meta.get("constructor"), None);
}

#[test]
fn test_should_return_an_empty_vector_when_given_an_empty_vector() {
    let objects: Vec<HashMap<String, Value>> = vec![];
    let result = umt_remove_prototype_map_deep(&objects);
    assert!(result.is_empty());
}

#[test]
fn test_should_preserve_element_order() {
    let objects = vec![
        as_map(obj! {"id" => 1, "value" => obj!{"x" => 1}}),
        as_map(obj! {"id" => 2, "value" => obj!{"x" => 2}}),
        as_map(obj! {"id" => 3, "value" => obj!{"x" => 3}}),
    ];

    let result = umt_remove_prototype_map_deep(&objects);

    let order: Vec<&Value> = result.iter().map(|item| item.get("id").unwrap()).collect();
    assert_eq!(order, vec![&Value::Int(1), &Value::Int(2), &Value::Int(3)]);
}

#[test]
fn test_should_sanitize_dangerous_keys_nested_inside_arrays() {
    let objects = vec![as_map(obj! {
        "items" => Value::Array(vec![
            obj!{"__proto__" => obj!{"polluted" => true}, "a" => 1},
            obj!{"constructor" => obj!{"polluted" => true}, "b" => 2},
        ])
    })];

    let result = umt_remove_prototype_map_deep(&objects);

    if let Some(Value::Array(items)) = result[0].get("items") {
        let item0 = items[0].as_object().unwrap();
        let item1 = items[1].as_object().unwrap();
        assert_eq!(item0.get("a"), Some(&Value::Int(1)));
        assert_eq!(item0.get("__proto__"), None);
        assert_eq!(item1.get("b"), Some(&Value::Int(2)));
        assert_eq!(item1.get("constructor"), None);
    } else {
        panic!("items should be an array");
    }
}

#[test]
fn test_should_handle_arrays_that_mix_sanitized_and_already_safe_objects() {
    let objects = vec![
        as_map(obj! {"clean" => obj!{"a" => 1}}),
        as_map(obj! {"dirty" => obj!{"__proto__" => obj!{"polluted" => true}, "b" => 2}}),
    ];

    let result = umt_remove_prototype_map_deep(&objects);

    assert_eq!(result[0].get("clean"), Some(&obj! {"a" => 1}));
    let dirty = result[1].get("dirty").and_then(Value::as_object).unwrap();
    assert_eq!(dirty.get("b"), Some(&Value::Int(2)));
    assert_eq!(dirty.get("__proto__"), None);
}
