use serde_json::json;
use umt_rust::string::format_string::detect_mode::detect_mode;

// Named mode detection tests

#[test]
fn test_detect_named_mode_with_object_data_only() {
    let data = json!({"name": "Alice", "age": 30});
    let result = detect_mode(Some(&data), None, &[]);

    assert_eq!(result.data, json!({"name": "Alice", "age": 30}));
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_detect_named_mode_with_object_data_and_options() {
    let data = json!({"name": "Bob", "score": 95});
    let options = json!({"formatters": {"custom": "test"}});
    let result = detect_mode(Some(&data), Some(&options), &[]);

    assert_eq!(result.data, data);
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_handle_empty_object_as_named_mode() {
    let data = json!({});
    let result = detect_mode(Some(&data), None, &[]);

    assert_eq!(result.data, json!({}));
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_handle_complex_nested_objects() {
    let complex_data = json!({
        "user": {"name": "Alice", "profile": {"age": 25}},
        "items": ["item1", "item2"]
    });
    let result = detect_mode(Some(&complex_data), None, &[]);

    assert_eq!(result.data, complex_data);
    assert!(result.options.formatters.is_empty());
}

// Indexed mode detection tests

#[test]
fn test_detect_indexed_mode_with_multiple_primitive_values() {
    let first = json!("first");
    let second = json!("second");
    let rest = vec![json!("third"), json!("fourth")];
    let result = detect_mode(Some(&first), Some(&second), &rest);

    assert_eq!(result.data, json!(["first", "second", "third", "fourth"]));
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_detect_indexed_mode_with_single_value() {
    let first = json!("single");
    let result = detect_mode(Some(&first), None, &[]);

    assert_eq!(result.data, json!(["single"]));
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_handle_mixed_types_in_indexed_mode() {
    let first = json!("string");
    let second = json!(42);
    let rest = vec![json!(true), json!(null), json!({"key": "value"})];
    let result = detect_mode(Some(&first), Some(&second), &rest);

    assert_eq!(
        result.data,
        json!(["string", 42, true, null, {"key": "value"}])
    );
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_handle_none_as_first_value() {
    let second = json!("second");
    let rest = vec![json!("third")];
    let result = detect_mode(None, Some(&second), &rest);

    assert_eq!(result.data, json!(["second", "third"]));
    assert!(result.options.formatters.is_empty());
}

// Object type differentiation tests

#[test]
fn test_treat_arrays_as_indexed_mode() {
    let array_val = json!(["array", "values"]);
    let second = json!("second");
    let result = detect_mode(Some(&array_val), Some(&second), &[]);

    assert_eq!(result.data, json!([["array", "values"], "second"]));
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_treat_null_as_indexed_mode() {
    let null_val = json!(null);
    let second = json!("second");
    let result = detect_mode(Some(&null_val), Some(&second), &[]);

    assert_eq!(result.data, json!([null, "second"]));
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_differentiate_between_data_object_and_options_object() {
    let data_object = json!({"name": "Alice"});
    let options_object = json!({"formatters": {"upper": "test"}});

    let result = detect_mode(Some(&data_object), Some(&options_object), &[]);

    assert_eq!(result.data, data_object);
}

// Edge cases tests

#[test]
fn test_handle_all_none_values() {
    let result = detect_mode(None, None, &[]);

    assert_eq!(result.data, json!([]));
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_handle_object_with_additional_values_as_indexed_mode() {
    let obj = json!({"name": "Alice"});
    let extra = json!("extra");
    let rest = vec![json!("more")];
    let result = detect_mode(Some(&obj), Some(&extra), &rest);

    assert_eq!(result.data, json!([{"name": "Alice"}, "extra", "more"]));
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_handle_non_options_object_as_second_parameter() {
    let data = json!({"name": "Alice"});
    let not_options = json!({"notFormatters": true});
    let result = detect_mode(Some(&data), Some(&not_options), &[]);

    assert_eq!(
        result.data,
        json!([{"name": "Alice"}, {"notFormatters": true}])
    );
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_handle_primitive_values_as_first_argument() {
    let string_val = json!("string");
    assert_eq!(
        detect_mode(Some(&string_val), None, &[]).data,
        json!(["string"])
    );

    let num_val = json!(42);
    assert_eq!(detect_mode(Some(&num_val), None, &[]).data, json!([42]));

    let bool_val = json!(true);
    assert_eq!(detect_mode(Some(&bool_val), None, &[]).data, json!([true]));

    let bool_val_false = json!(false);
    assert_eq!(
        detect_mode(Some(&bool_val_false), None, &[]).data,
        json!([false])
    );
}

// Options object validation tests

#[test]
fn test_recognize_valid_options_object() {
    let data = json!({"name": "test"});
    let options = json!({
        "formatters": {
            "custom": "value",
            "upper": "toUpperCase"
        }
    });
    let result = detect_mode(Some(&data), Some(&options), &[]);

    assert_eq!(result.data, json!({"name": "test"}));
}

#[test]
fn test_reject_object_without_formatters_property() {
    let data = json!({"name": "test"});
    let not_options = json!({"someOtherProperty": "value"});
    let result = detect_mode(Some(&data), Some(&not_options), &[]);

    assert_eq!(
        result.data,
        json!([{"name": "test"}, {"someOtherProperty": "value"}])
    );
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_reject_array_as_options() {
    let data = json!({"name": "test"});
    let array = json!(["not", "options"]);
    let result = detect_mode(Some(&data), Some(&array), &[]);

    assert_eq!(result.data, json!([{"name": "test"}, ["not", "options"]]));
    assert!(result.options.formatters.is_empty());
}

// Complex scenarios tests

#[test]
fn test_handle_deeply_nested_data_structures() {
    let complex_data = json!({
        "level1": {
            "level2": {
                "level3": {
                    "value": "deep"
                }
            }
        },
        "array": [1, 2, {"nested": "object"}]
    });

    let result = detect_mode(Some(&complex_data), None, &[]);

    assert_eq!(result.data, complex_data);
    assert!(result.options.formatters.is_empty());
}

#[test]
fn test_handle_mixed_scenarios_correctly() {
    // When first arg is object but has extra values, it goes to indexed mode
    let obj = json!({"key": "value"});
    let string_val = json!("string");
    let rest = vec![json!(42), json!(true)];

    let result = detect_mode(Some(&obj), Some(&string_val), &rest);

    assert_eq!(result.data, json!([{"key": "value"}, "string", 42, true]));
    assert!(result.options.formatters.is_empty());
}
