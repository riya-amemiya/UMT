use umt_rust::obj;
use umt_rust::object::Value;
use umt_rust::string::format_string::get_value;

// Simple property access tests
#[test]
fn test_retrieve_simple_string_properties() {
    let obj = obj!("name" => "Alice", "age" => 30);

    assert_eq!(get_value(&obj, "name"), Some(&Value::String("Alice".to_string())));
    assert_eq!(get_value(&obj, "age"), Some(&Value::Int(30)));
}

#[test]
fn test_return_none_for_non_existent_properties() {
    let obj = obj!("name" => "Alice");

    assert_eq!(get_value(&obj, "nonexistent"), None);
    assert_eq!(get_value(&obj, "missing"), None);
}

#[test]
fn test_handle_various_data_types() {
    let obj = obj!(
        "string" => "text",
        "number" => 42,
        "boolean" => true,
        "nullValue" => Value::Null,
        "zero" => 0,
        "emptyString" => ""
    );

    assert_eq!(get_value(&obj, "string"), Some(&Value::String("text".to_string())));
    assert_eq!(get_value(&obj, "number"), Some(&Value::Int(42)));
    assert_eq!(get_value(&obj, "boolean"), Some(&Value::Bool(true)));
    assert_eq!(get_value(&obj, "nullValue"), Some(&Value::Null));
    assert_eq!(get_value(&obj, "zero"), Some(&Value::Int(0)));
    assert_eq!(get_value(&obj, "emptyString"), Some(&Value::String("".to_string())));
}

// Nested property access tests
#[test]
fn test_retrieve_nested_properties() {
    let obj = obj!(
        "user" => obj!(
            "name" => "Bob",
            "profile" => obj!(
                "age" => 25,
                "location" => "Tokyo"
            )
        )
    );

    assert_eq!(get_value(&obj, "user.name"), Some(&Value::String("Bob".to_string())));
    assert_eq!(get_value(&obj, "user.profile.age"), Some(&Value::Int(25)));
    assert_eq!(
        get_value(&obj, "user.profile.location"),
        Some(&Value::String("Tokyo".to_string()))
    );
}

#[test]
fn test_return_none_for_broken_nested_paths() {
    let obj = obj!(
        "user" => obj!(
            "name" => "Bob"
        )
    );

    assert_eq!(get_value(&obj, "user.nonexistent"), None);
    assert_eq!(get_value(&obj, "user.profile.age"), None);
    assert_eq!(get_value(&obj, "nonexistent.property"), None);
}

#[test]
fn test_handle_deep_nesting() {
    let obj = obj!(
        "level1" => obj!(
            "level2" => obj!(
                "level3" => obj!(
                    "level4" => obj!(
                        "value" => "deep"
                    )
                )
            )
        )
    );

    assert_eq!(
        get_value(&obj, "level1.level2.level3.level4.value"),
        Some(&Value::String("deep".to_string()))
    );
}

// Array access tests
#[test]
fn test_access_array_elements_with_positive_indices() {
    let obj = obj!(
        "items" => vec!["A", "B", "C", "D"]
    );

    assert_eq!(get_value(&obj, "items[0]"), Some(&Value::String("A".to_string())));
    assert_eq!(get_value(&obj, "items[1]"), Some(&Value::String("B".to_string())));
    assert_eq!(get_value(&obj, "items[2]"), Some(&Value::String("C".to_string())));
    assert_eq!(get_value(&obj, "items[3]"), Some(&Value::String("D".to_string())));
}

#[test]
fn test_access_array_elements_with_negative_indices() {
    let obj = obj!(
        "items" => vec!["A", "B", "C", "D"]
    );

    assert_eq!(get_value(&obj, "items[-1]"), Some(&Value::String("D".to_string())));
    assert_eq!(get_value(&obj, "items[-2]"), Some(&Value::String("C".to_string())));
    assert_eq!(get_value(&obj, "items[-3]"), Some(&Value::String("B".to_string())));
    assert_eq!(get_value(&obj, "items[-4]"), Some(&Value::String("A".to_string())));
}

#[test]
fn test_return_none_for_out_of_bounds_indices() {
    let obj = obj!(
        "items" => vec!["A", "B"]
    );

    assert_eq!(get_value(&obj, "items[5]"), None);
    assert_eq!(get_value(&obj, "items[-5]"), None);
}

#[test]
fn test_handle_empty_arrays() {
    let obj = obj!(
        "items" => Vec::<String>::new()
    );

    assert_eq!(get_value(&obj, "items[0]"), None);
    assert_eq!(get_value(&obj, "items[-1]"), None);
}

// Complex nested access tests
#[test]
fn test_handle_arrays_of_objects() {
    let obj = obj!(
        "users" => vec![
            obj!("name" => "Alice", "age" => 30),
            obj!("name" => "Bob", "age" => 25),
            obj!("name" => "Charlie", "age" => 35)
        ]
    );

    assert_eq!(get_value(&obj, "users[0].name"), Some(&Value::String("Alice".to_string())));
    assert_eq!(get_value(&obj, "users[1].age"), Some(&Value::Int(25)));
    assert_eq!(get_value(&obj, "users[-1].name"), Some(&Value::String("Charlie".to_string())));
}

#[test]
fn test_handle_nested_arrays() {
    let obj = obj!(
        "matrix" => vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]
    );

    match get_value(&obj, "matrix[0]") {
        Some(Value::Array(arr)) => {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Value::Int(1));
        },
        _ => panic!("Expected array"),
    }
}

#[test]
fn test_handle_objects_containing_arrays_containing_objects() {
    let obj = obj!(
        "data" => obj!(
            "categories" => vec![
                obj!(
                    "name" => "Technology",
                    "items" => vec![
                        obj!("title" => "Laptop", "price" => 1000),
                        obj!("title" => "Phone", "price" => 500)
                    ]
                ),
                obj!(
                    "name" => "Books",
                    "items" => vec![
                        obj!("title" => "Novel", "price" => 20)
                    ]
                )
            ]
        )
    );

    assert_eq!(
        get_value(&obj, "data.categories[0].name"),
        Some(&Value::String("Technology".to_string()))
    );
    assert_eq!(
        get_value(&obj, "data.categories[0].items[1].title"),
        Some(&Value::String("Phone".to_string()))
    );
    assert_eq!(
        get_value(&obj, "data.categories[-1].items[0].price"),
        Some(&Value::Int(20))
    );
}

// Edge cases tests
#[test]
fn test_handle_null_and_undefined_objects() {
    assert_eq!(get_value(&Value::Null, "property"), None);
}

#[test]
fn test_handle_primitive_values_as_objects() {
    assert_eq!(get_value(&Value::Int(42), "property"), None);
    assert_eq!(get_value(&Value::String("string".to_string()), "property"), None);
    assert_eq!(get_value(&Value::Bool(true), "property"), None);
}

#[test]
fn test_handle_empty_path() {
    let obj = obj!("name" => "Alice");
    assert_eq!(get_value(&obj, ""), None);
}

#[test]
fn test_handle_array_access_on_non_arrays() {
    let obj = obj!(
        "notAnArray" => "string"
    );

    assert_eq!(get_value(&obj, "notAnArray[0]"), None);
}

// Special path formats tests
#[test]
fn test_handle_zero_indices() {
    let obj = obj!(
        "items" => vec!["zero", "one", "two"]
    );

    assert_eq!(get_value(&obj, "items[0]"), Some(&Value::String("zero".to_string())));
    assert_eq!(get_value(&obj, "items[-0]"), Some(&Value::String("zero".to_string())));
}

#[test]
fn test_handle_array_indices_in_the_middle_of_paths() {
    let obj = obj!(
        "groups" => vec![
            obj!(
                "name" => "Group1",
                "subGroups" => vec![
                    obj!("name" => "SubGroup1", "value" => "test1"),
                    obj!("name" => "SubGroup2", "value" => "test2")
                ]
            )
        ]
    );

    assert_eq!(
        get_value(&obj, "groups[0].subGroups[1].value"),
        Some(&Value::String("test2".to_string()))
    );
}

// Data type preservation tests
#[test]
fn test_preserve_original_data_types() {
    let obj = obj!(
        "numbers" => vec![Value::Int(0), Value::Int(1), Value::Int(-1), Value::Float(3.14)],
        "booleans" => vec![true, false],
        "objects" => vec![obj!("key" => "value")],
        "mixed" => vec![Value::Null, Value::String("".to_string()), Value::Int(0), Value::Bool(false)]
    );

    assert_eq!(get_value(&obj, "numbers[0]"), Some(&Value::Int(0)));
    assert_eq!(get_value(&obj, "numbers[2]"), Some(&Value::Int(-1)));
    assert_eq!(get_value(&obj, "numbers[3]"), Some(&Value::Float(3.14)));
    assert_eq!(get_value(&obj, "booleans[0]"), Some(&Value::Bool(true)));
    assert_eq!(get_value(&obj, "booleans[1]"), Some(&Value::Bool(false)));

    // For objects, we check by matching
    let obj0 = get_value(&obj, "objects[0]").unwrap();
    if let Value::Object(map) = obj0 {
        assert_eq!(map.get("key"), Some(&Value::String("value".to_string())));
    } else {
        panic!("Expected object");
    }

    assert_eq!(get_value(&obj, "mixed[0]"), Some(&Value::Null));
    assert_eq!(get_value(&obj, "mixed[1]"), Some(&Value::String("".to_string())));
    assert_eq!(get_value(&obj, "mixed[2]"), Some(&Value::Int(0)));
    assert_eq!(get_value(&obj, "mixed[3]"), Some(&Value::Bool(false)));
}
