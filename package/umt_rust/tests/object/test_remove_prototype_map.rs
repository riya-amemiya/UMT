use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_remove_prototype_map};

fn as_map(value: Value) -> HashMap<String, Value> {
    value.into_object().unwrap()
}

#[test]
fn test_should_remove_prototype_polluting_properties_from_each_object() {
    let objects = vec![
        as_map(obj! {"__proto__" => obj!{"polluted" => true}, "a" => 1}),
        as_map(obj! {
            "constructor" => obj!{"prototype" => obj!{"polluted" => true}},
            "b" => 2,
            "nested" => obj!{"keep" => true}
        }),
        as_map(obj! {"prototype" => obj!{"polluted" => true}, "c" => 3}),
    ];

    let result = umt_remove_prototype_map(&objects);

    assert_eq!(result[0].get("a"), Some(&Value::Int(1)));
    assert_eq!(result[0].len(), 1);
    assert_eq!(result[1].get("b"), Some(&Value::Int(2)));
    assert_eq!(result[1].get("nested"), Some(&obj! {"keep" => true}));
    assert_eq!(result[1].len(), 2);
    assert_eq!(result[2].get("c"), Some(&Value::Int(3)));
    assert_eq!(result[2].len(), 1);
}

#[test]
fn test_should_return_an_empty_vector_when_given_an_empty_vector() {
    let objects: Vec<HashMap<String, Value>> = vec![];
    let result = umt_remove_prototype_map(&objects);
    assert!(result.is_empty());
}

#[test]
fn test_should_preserve_element_order() {
    let objects = vec![
        as_map(obj! {"a" => 1}),
        as_map(obj! {"a" => 2}),
        as_map(obj! {"a" => 3}),
        as_map(obj! {"a" => 4}),
    ];

    let result = umt_remove_prototype_map(&objects);

    let order: Vec<&Value> = result.iter().map(|item| item.get("a").unwrap()).collect();
    assert_eq!(
        order,
        vec![
            &Value::Int(1),
            &Value::Int(2),
            &Value::Int(3),
            &Value::Int(4)
        ]
    );
}

#[test]
fn test_should_only_sanitize_the_top_level_of_each_element() {
    let objects = vec![as_map(obj! {
        "nested" => obj!{"__proto__" => obj!{"polluted" => true}, "deep" => 1}
    })];

    let result = umt_remove_prototype_map(&objects);

    let nested = result[0].get("nested").and_then(Value::as_object).unwrap();
    assert!(nested.contains_key("__proto__"));
    assert_eq!(nested.get("deep"), Some(&Value::Int(1)));
}

#[test]
fn test_should_not_mutate_any_of_the_input_objects() {
    let objects = vec![
        as_map(obj! {"__proto__" => obj!{"polluted" => true}, "a" => 1}),
        as_map(obj! {"constructor" => obj!{"polluted" => true}, "b" => 2}),
    ];
    let snapshot = objects.clone();

    let _ = umt_remove_prototype_map(&objects);

    assert_eq!(objects, snapshot);
}
