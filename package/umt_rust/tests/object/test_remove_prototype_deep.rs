use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_remove_prototype_deep};

fn as_map(value: Value) -> HashMap<String, Value> {
    value.into_object().unwrap()
}

#[test]
fn test_should_remove_prototype_polluting_properties_recursively() {
    let object = as_map(obj! {
        "safe" => obj!{"nested" => obj!{"__proto__" => obj!{"polluted" => true}, "value" => 1}},
        "list" => Value::Array(vec![
            obj!{"constructor" => obj!{"prototype" => obj!{"polluted" => true}}, "ok" => 1},
            obj!{"prototype" => obj!{"polluted" => true}, "safe" => 2},
        ])
    });

    let result = umt_remove_prototype_deep(&object);

    let safe = result.get("safe").and_then(Value::as_object).unwrap();
    let nested = safe.get("nested").and_then(Value::as_object).unwrap();
    assert_eq!(nested.get("value"), Some(&Value::Int(1)));
    assert_eq!(nested.get("__proto__"), None);

    if let Some(Value::Array(list)) = result.get("list") {
        let item0 = list[0].as_object().unwrap();
        let item1 = list[1].as_object().unwrap();
        assert_eq!(item0.get("ok"), Some(&Value::Int(1)));
        assert_eq!(item0.get("constructor"), None);
        assert_eq!(item1.get("safe"), Some(&Value::Int(2)));
        assert_eq!(item1.get("prototype"), None);
    } else {
        panic!("list should be an array");
    }
}

#[test]
fn test_should_handle_arrays_containing_arrays_and_primitives() {
    let object = as_map(obj! {
        "matrix" => Value::Array(vec![
            Value::Array(vec![obj!{"__proto__" => obj!{"polluted" => true}, "a" => 1}]),
            Value::Array(vec![
                Value::String("text".to_string()),
                Value::Int(42),
                Value::Bool(true),
                Value::Null,
            ]),
        ])
    });

    let result = umt_remove_prototype_deep(&object);

    if let Some(Value::Array(matrix)) = result.get("matrix") {
        if let Value::Array(inner) = &matrix[0] {
            let item = inner[0].as_object().unwrap();
            assert_eq!(item.get("a"), Some(&Value::Int(1)));
            assert_eq!(item.get("__proto__"), None);
        } else {
            panic!("matrix[0] should be an array");
        }
        assert_eq!(
            matrix[1],
            Value::Array(vec![
                Value::String("text".to_string()),
                Value::Int(42),
                Value::Bool(true),
                Value::Null,
            ])
        );
    } else {
        panic!("matrix should be an array");
    }
}

#[test]
fn test_should_not_modify_the_original_object() {
    let object = as_map(obj! {
        "safe" => obj!{"__proto__" => obj!{"polluted" => true}, "value" => 1}
    });
    let snapshot = object.clone();

    let _ = umt_remove_prototype_deep(&object);

    assert_eq!(object, snapshot);
}

#[test]
fn test_should_handle_an_empty_object() {
    let object: HashMap<String, Value> = HashMap::new();
    let result = umt_remove_prototype_deep(&object);
    assert!(result.is_empty());
}

#[test]
fn test_should_preserve_primitive_values_verbatim() {
    let object = as_map(obj! {
        "a" => 1,
        "b" => "text",
        "c" => true,
        "d" => Value::Null
    });

    let result = umt_remove_prototype_deep(&object);
    assert_eq!(result, object);
}

#[test]
fn test_should_sanitize_dangerous_keys_at_multiple_depths() {
    let object = as_map(obj! {
        "__proto__" => 1,
        "a" => obj!{"__proto__" => 2, "b" => obj!{"__proto__" => 3, "c" => 4}}
    });

    let result = umt_remove_prototype_deep(&object);

    let a = result.get("a").and_then(Value::as_object).unwrap();
    let b = a.get("b").and_then(Value::as_object).unwrap();
    assert_eq!(result.get("__proto__"), None);
    assert_eq!(a.get("__proto__"), None);
    assert_eq!(b.get("__proto__"), None);
    assert_eq!(b.get("c"), Some(&Value::Int(4)));
}

#[test]
fn test_should_recurse_through_arrays_that_contain_arrays() {
    let object = as_map(obj! {
        "matrix" => Value::Array(vec![Value::Array(vec![Value::Array(vec![
            obj!{"__proto__" => obj!{"polluted" => true}, "deep" => 1}
        ])])])
    });

    let result = umt_remove_prototype_deep(&object);

    if let Some(Value::Array(level0)) = result.get("matrix") {
        if let Value::Array(level1) = &level0[0] {
            if let Value::Array(level2) = &level1[0] {
                let item = level2[0].as_object().unwrap();
                assert_eq!(item.get("deep"), Some(&Value::Int(1)));
                assert_eq!(item.get("__proto__"), None);
                return;
            }
        }
    }
    panic!("matrix structure mismatch");
}
