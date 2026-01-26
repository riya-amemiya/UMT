use serde_json::json;
use umt_rust::string::format_string::get_value;

// Simple property access tests
#[test]
fn test_retrieve_simple_string_properties() {
    let obj = json!({"name": "Alice", "age": 30});

    assert_eq!(get_value(&obj, "name"), Some(&json!("Alice")));
    assert_eq!(get_value(&obj, "age"), Some(&json!(30)));
}

#[test]
fn test_return_none_for_non_existent_properties() {
    let obj = json!({"name": "Alice"});

    assert_eq!(get_value(&obj, "nonexistent"), None);
    assert_eq!(get_value(&obj, "missing"), None);
}

#[test]
fn test_handle_various_data_types() {
    let obj = json!({
        "string": "text",
        "number": 42,
        "boolean": true,
        "nullValue": null,
        "zero": 0,
        "emptyString": ""
    });

    assert_eq!(get_value(&obj, "string"), Some(&json!("text")));
    assert_eq!(get_value(&obj, "number"), Some(&json!(42)));
    assert_eq!(get_value(&obj, "boolean"), Some(&json!(true)));
    assert_eq!(get_value(&obj, "nullValue"), Some(&json!(null)));
    assert_eq!(get_value(&obj, "zero"), Some(&json!(0)));
    assert_eq!(get_value(&obj, "emptyString"), Some(&json!("")));
}

// Nested property access tests
#[test]
fn test_retrieve_nested_properties() {
    let obj = json!({
        "user": {
            "name": "Bob",
            "profile": {
                "age": 25,
                "location": "Tokyo"
            }
        }
    });

    assert_eq!(get_value(&obj, "user.name"), Some(&json!("Bob")));
    assert_eq!(get_value(&obj, "user.profile.age"), Some(&json!(25)));
    assert_eq!(
        get_value(&obj, "user.profile.location"),
        Some(&json!("Tokyo"))
    );
}

#[test]
fn test_return_none_for_broken_nested_paths() {
    let obj = json!({
        "user": {
            "name": "Bob"
        }
    });

    assert_eq!(get_value(&obj, "user.nonexistent"), None);
    assert_eq!(get_value(&obj, "user.profile.age"), None);
    assert_eq!(get_value(&obj, "nonexistent.property"), None);
}

#[test]
fn test_handle_deep_nesting() {
    let obj = json!({
        "level1": {
            "level2": {
                "level3": {
                    "level4": {
                        "value": "deep"
                    }
                }
            }
        }
    });

    assert_eq!(
        get_value(&obj, "level1.level2.level3.level4.value"),
        Some(&json!("deep"))
    );
}

// Array access tests
#[test]
fn test_access_array_elements_with_positive_indices() {
    let obj = json!({
        "items": ["A", "B", "C", "D"]
    });

    assert_eq!(get_value(&obj, "items[0]"), Some(&json!("A")));
    assert_eq!(get_value(&obj, "items[1]"), Some(&json!("B")));
    assert_eq!(get_value(&obj, "items[2]"), Some(&json!("C")));
    assert_eq!(get_value(&obj, "items[3]"), Some(&json!("D")));
}

#[test]
fn test_access_array_elements_with_negative_indices() {
    let obj = json!({
        "items": ["A", "B", "C", "D"]
    });

    assert_eq!(get_value(&obj, "items[-1]"), Some(&json!("D")));
    assert_eq!(get_value(&obj, "items[-2]"), Some(&json!("C")));
    assert_eq!(get_value(&obj, "items[-3]"), Some(&json!("B")));
    assert_eq!(get_value(&obj, "items[-4]"), Some(&json!("A")));
}

#[test]
fn test_return_none_for_out_of_bounds_indices() {
    let obj = json!({
        "items": ["A", "B"]
    });

    assert_eq!(get_value(&obj, "items[5]"), None);
    assert_eq!(get_value(&obj, "items[-5]"), None);
}

#[test]
fn test_handle_empty_arrays() {
    let obj = json!({
        "items": []
    });

    assert_eq!(get_value(&obj, "items[0]"), None);
    assert_eq!(get_value(&obj, "items[-1]"), None);
}

// Complex nested access tests
#[test]
fn test_handle_arrays_of_objects() {
    let obj = json!({
        "users": [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25},
            {"name": "Charlie", "age": 35}
        ]
    });

    assert_eq!(get_value(&obj, "users[0].name"), Some(&json!("Alice")));
    assert_eq!(get_value(&obj, "users[1].age"), Some(&json!(25)));
    assert_eq!(get_value(&obj, "users[-1].name"), Some(&json!("Charlie")));
}

#[test]
fn test_handle_nested_arrays() {
    let obj = json!({
        "matrix": [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]
    });

    // Note: get_value doesn't support nested array notation like matrix[0][1]
    assert_eq!(get_value(&obj, "matrix[0]"), Some(&json!([1, 2, 3])));
    assert_eq!(get_value(&obj, "matrix[1]"), Some(&json!([4, 5, 6])));
    assert_eq!(get_value(&obj, "matrix[-1]"), Some(&json!([7, 8, 9])));
}

#[test]
fn test_handle_objects_containing_arrays_containing_objects() {
    let obj = json!({
        "data": {
            "categories": [
                {
                    "name": "Technology",
                    "items": [
                        {"title": "Laptop", "price": 1000},
                        {"title": "Phone", "price": 500}
                    ]
                },
                {
                    "name": "Books",
                    "items": [{"title": "Novel", "price": 20}]
                }
            ]
        }
    });

    assert_eq!(
        get_value(&obj, "data.categories[0].name"),
        Some(&json!("Technology"))
    );
    assert_eq!(
        get_value(&obj, "data.categories[0].items[1].title"),
        Some(&json!("Phone"))
    );
    assert_eq!(
        get_value(&obj, "data.categories[-1].items[0].price"),
        Some(&json!(20))
    );
}

// Edge cases tests
#[test]
fn test_handle_null_and_undefined_objects() {
    assert_eq!(get_value(&json!(null), "property"), None);
}

#[test]
fn test_handle_primitive_values_as_objects() {
    assert_eq!(get_value(&json!(42), "property"), None);
    assert_eq!(get_value(&json!("string"), "property"), None);
    assert_eq!(get_value(&json!(true), "property"), None);
}

#[test]
fn test_handle_empty_path() {
    let obj = json!({"name": "Alice"});
    assert_eq!(get_value(&obj, ""), None);
}

#[test]
fn test_handle_array_access_on_non_arrays() {
    let obj = json!({
        "notAnArray": "string"
    });

    assert_eq!(get_value(&obj, "notAnArray[0]"), None);
}

// Special path formats tests
#[test]
fn test_handle_zero_indices() {
    let obj = json!({
        "items": ["zero", "one", "two"]
    });

    assert_eq!(get_value(&obj, "items[0]"), Some(&json!("zero")));
    assert_eq!(get_value(&obj, "items[-0]"), Some(&json!("zero")));
}

#[test]
fn test_handle_array_indices_in_the_middle_of_paths() {
    let obj = json!({
        "groups": [
            {
                "name": "Group1",
                "subGroups": [
                    {"name": "SubGroup1", "value": "test1"},
                    {"name": "SubGroup2", "value": "test2"}
                ]
            }
        ]
    });

    assert_eq!(
        get_value(&obj, "groups[0].subGroups[1].value"),
        Some(&json!("test2"))
    );
}

// Data type preservation tests
#[test]
fn test_preserve_original_data_types() {
    let obj = json!({
        "numbers": [0, 1, -1, 3.14],
        "booleans": [true, false],
        "objects": [{"key": "value"}],
        "mixed": [null, "", 0, false]
    });

    assert_eq!(get_value(&obj, "numbers[0]"), Some(&json!(0)));
    assert_eq!(get_value(&obj, "numbers[2]"), Some(&json!(-1)));
    assert_eq!(get_value(&obj, "numbers[3]"), Some(&json!(3.14)));
    assert_eq!(get_value(&obj, "booleans[0]"), Some(&json!(true)));
    assert_eq!(get_value(&obj, "booleans[1]"), Some(&json!(false)));
    assert_eq!(
        get_value(&obj, "objects[0]"),
        Some(&json!({"key": "value"}))
    );
    assert_eq!(get_value(&obj, "mixed[0]"), Some(&json!(null)));
    assert_eq!(get_value(&obj, "mixed[1]"), Some(&json!("")));
    assert_eq!(get_value(&obj, "mixed[2]"), Some(&json!(0)));
    assert_eq!(get_value(&obj, "mixed[3]"), Some(&json!(false)));
}

use umt_rust::string::format_string::*;

#[test]
fn test_array_access_negative_indices() {
    let obj = json!({
        "items": ["A", "B", "C", "D"]
    });

    assert_eq!(get_value(&obj, "items[-1]"), Some(&json!("D")));
    assert_eq!(get_value(&obj, "items[-2]"), Some(&json!("C")));
    assert_eq!(get_value(&obj, "items[-3]"), Some(&json!("B")));
    assert_eq!(get_value(&obj, "items[-4]"), Some(&json!("A")));
}

#[test]
fn test_array_access_on_non_arrays() {
    let obj = json!({
        "notAnArray": "string"
    });

    assert_eq!(get_value(&obj, "notAnArray[0]"), None);
}

#[test]
fn test_array_access_positive_indices() {
    let obj = json!({
        "items": ["A", "B", "C", "D"]
    });

    assert_eq!(get_value(&obj, "items[0]"), Some(&json!("A")));
    assert_eq!(get_value(&obj, "items[1]"), Some(&json!("B")));
    assert_eq!(get_value(&obj, "items[2]"), Some(&json!("C")));
    assert_eq!(get_value(&obj, "items[3]"), Some(&json!("D")));
}

#[test]
fn test_arrays_of_objects() {
    let obj = json!({
        "users": [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25},
            {"name": "Charlie", "age": 35}
        ]
    });

    assert_eq!(get_value(&obj, "users[0].name"), Some(&json!("Alice")));
    assert_eq!(get_value(&obj, "users[1].age"), Some(&json!(25)));
    assert_eq!(get_value(&obj, "users[-1].name"), Some(&json!("Charlie")));
}

#[test]
fn test_broken_nested_paths() {
    let obj = json!({
        "user": {
            "name": "Bob"
        }
    });

    assert_eq!(get_value(&obj, "user.nonexistent"), None);
    assert_eq!(get_value(&obj, "user.profile.age"), None);
    assert_eq!(get_value(&obj, "nonexistent.property"), None);
}

#[test]
fn test_empty_path() {
    let obj = json!({"name": "Alice"});
    assert_eq!(get_value(&obj, ""), None);
}

#[test]
fn test_nested_property_access() {
    let obj = json!({
        "user": {
            "name": "Bob",
            "profile": {
                "age": 25,
                "location": "Tokyo"
            }
        }
    });

    assert_eq!(get_value(&obj, "user.name"), Some(&json!("Bob")));
    assert_eq!(get_value(&obj, "user.profile.age"), Some(&json!(25)));
    assert_eq!(
        get_value(&obj, "user.profile.location"),
        Some(&json!("Tokyo"))
    );
}

#[test]
fn test_non_existent_properties() {
    let obj = json!({"name": "Alice"});

    assert_eq!(get_value(&obj, "nonexistent"), None);
    assert_eq!(get_value(&obj, "missing"), None);
}

#[test]
fn test_null_and_primitives() {
    assert_eq!(get_value(&json!(null), "property"), None);
    assert_eq!(get_value(&json!(42), "property"), None);
    assert_eq!(get_value(&json!("string"), "property"), None);
    assert_eq!(get_value(&json!(true), "property"), None);
}

#[test]
fn test_out_of_bounds_indices() {
    let obj = json!({
        "items": ["A", "B"]
    });

    assert_eq!(get_value(&obj, "items[5]"), None);
    assert_eq!(get_value(&obj, "items[-5]"), None);
}

#[test]
fn test_simple_property_access() {
    let obj = json!({"name": "Alice", "age": 30});

    assert_eq!(get_value(&obj, "name"), Some(&json!("Alice")));
    assert_eq!(get_value(&obj, "age"), Some(&json!(30)));
}

#[test]
fn test_various_data_types() {
    let obj = json!({
        "string": "text",
        "number": 42,
        "boolean": true,
        "nullValue": null,
        "zero": 0,
        "emptyString": ""
    });

    assert_eq!(get_value(&obj, "string"), Some(&json!("text")));
    assert_eq!(get_value(&obj, "number"), Some(&json!(42)));
    assert_eq!(get_value(&obj, "boolean"), Some(&json!(true)));
    assert_eq!(get_value(&obj, "nullValue"), Some(&json!(null)));
    assert_eq!(get_value(&obj, "zero"), Some(&json!(0)));
    assert_eq!(get_value(&obj, "emptyString"), Some(&json!("")));
}
