use serde_json::json;
use umt_rust::string::{umt_format_string, umt_format_string_indexed};

// Original functionality tests
#[test]
fn test_replace_placeholders_with_specified_values() {
    let template = "Hello, {0}! It's {1} today.";
    let result = umt_format_string_indexed(template, &["World", "sunny"]);
    assert_eq!(result, "Hello, World! It's sunny today.");
}

#[test]
fn test_not_replace_placeholders_with_undefined_values() {
    let template = "Hello, {0}! How's {1}?";
    let result = umt_format_string_indexed(template, &["World"]);
    assert_eq!(result, "Hello, World! How's {1}?");
}

#[test]
fn test_not_replace_placeholders_with_non_existent_indices() {
    let template = "Hello, {0}! {2} is not available.";
    let result = umt_format_string_indexed(template, &["World", "sunny"]);
    assert_eq!(result, "Hello, World! {2} is not available.");
}

#[test]
fn test_handle_empty_strings_as_values() {
    let template = "Start{0}Middle{1}End";
    let result = umt_format_string_indexed(template, &["", ""]);
    assert_eq!(result, "StartMiddleEnd");
}

#[test]
fn test_handle_repeated_placeholders() {
    let template = "{0} {1} {0} {1} {0}";
    let result = umt_format_string_indexed(template, &["A", "B"]);
    assert_eq!(result, "A B A B A");
}

// Named placeholders tests
#[test]
fn test_replace_named_placeholders_with_object_values() {
    let template = "Hello, {name}! You are {age} years old.";
    let result = umt_format_string(template, &json!({"name": "Alice", "age": 25}), None);
    assert_eq!(result, "Hello, Alice! You are 25 years old.");
}

#[test]
fn test_handle_missing_named_placeholders() {
    let template = "Hello, {name}! How's {mood}?";
    let result = umt_format_string(template, &json!({"name": "Bob"}), None);
    assert_eq!(result, "Hello, Bob! How's {mood}?");
}

// Nested object access tests
#[test]
fn test_access_nested_object_properties() {
    let template = "User: {user.name}, Email: {user.email}";
    let result = umt_format_string(
        template,
        &json!({
            "user": { "name": "Charlie", "email": "charlie@example.com" }
        }),
        None,
    );
    assert_eq!(result, "User: Charlie, Email: charlie@example.com");
}

#[test]
fn test_handle_deep_nesting() {
    let template = "Address: {user.address.city}, {user.address.country}";
    let result = umt_format_string(
        template,
        &json!({
            "user": {
                "address": {
                    "city": "Tokyo",
                    "country": "Japan"
                }
            }
        }),
        None,
    );
    assert_eq!(result, "Address: Tokyo, Japan");
}

#[test]
fn test_handle_missing_nested_properties() {
    let template = "User: {user.name}, Phone: {user.phone}";
    let result = umt_format_string(template, &json!({"user": {"name": "Dave"}}), None);
    assert_eq!(result, "User: Dave, Phone: {user.phone}");
}

#[test]
fn test_handle_null_values_in_nested_access() {
    let template = "Value: {data.nested}";
    let result = umt_format_string(template, &json!({"data": null}), None);
    assert_eq!(result, "Value: {data.nested}");
}

// Array access tests
#[test]
fn test_access_array_elements_by_positive_index() {
    let template = "First: {items[0]}, Second: {items[1]}";
    let result = umt_format_string(template, &json!({"items": ["A", "B", "C"]}), None);
    assert_eq!(result, "First: A, Second: B");
}

#[test]
fn test_access_array_elements_by_negative_index() {
    let template = "Last: {items[-1]}, Second last: {items[-2]}";
    let result = umt_format_string(template, &json!({"items": ["A", "B", "C"]}), None);
    assert_eq!(result, "Last: C, Second last: B");
}

#[test]
fn test_handle_out_of_bounds_array_access() {
    let template = "Item: {items[10]}";
    let result = umt_format_string(template, &json!({"items": ["A", "B"]}), None);
    assert_eq!(result, "Item: {items[10]}");
}

#[test]
fn test_handle_nested_array_access() {
    let template = "First user: {users[0].name}";
    let result = umt_format_string(
        template,
        &json!({
            "users": [{"name": "Alice"}, {"name": "Bob"}]
        }),
        None,
    );
    assert_eq!(result, "First user: Alice");
}

// Default values tests
#[test]
fn test_use_default_values_for_missing_placeholders() {
    let template = "Name: {name|Unknown}, Age: {age|N/A}";
    let result = umt_format_string(template, &json!({"age": 25}), None);
    assert_eq!(result, "Name: Unknown, Age: 25");
}

#[test]
fn test_use_actual_values_when_available() {
    let template = "Name: {name|Unknown}, Age: {age|N/A}";
    let result = umt_format_string(template, &json!({"name": "Eve", "age": 30}), None);
    assert_eq!(result, "Name: Eve, Age: 30");
}

#[test]
fn test_handle_default_values_with_null() {
    let template = "Value: {value|Default}";
    let result = umt_format_string(template, &json!({"value": null}), None);
    assert_eq!(result, "Value: Default");
}

// Escape sequences tests
#[test]
fn test_handle_escaped_braces() {
    let template = "Literal {{0}} and value {0}";
    let result = umt_format_string_indexed(template, &["test"]);
    assert_eq!(result, "Literal {0} and value test");
}

#[test]
fn test_handle_multiple_escaped_braces() {
    let template = "{{name}} is not {name}, but {{age}} is not {age}";
    let result = umt_format_string(template, &json!({"name": "Alice", "age": 25}), None);
    assert_eq!(result, "{name} is not Alice, but {age} is not 25");
}

// Formatters tests
#[test]
fn test_format_with_upper_formatter() {
    let template = "Name: {name:upper}";
    let result = umt_format_string(template, &json!({"name": "alice"}), None);
    assert_eq!(result, "Name: ALICE");
}

#[test]
fn test_format_with_lower_formatter() {
    let template = "Name: {name:lower}";
    let result = umt_format_string(template, &json!({"name": "ALICE"}), None);
    assert_eq!(result, "Name: alice");
}

#[test]
fn test_handle_plural_formatting() {
    let template1 = "You have {count:plural(item,items)}";
    let template2 = "{count} {count:plural(item,items)}";

    let result1 = umt_format_string(template1, &json!({"count": 1}), None);
    let result2 = umt_format_string(template2, &json!({"count": 1}), None);
    let result3 = umt_format_string(template2, &json!({"count": 5}), None);

    assert_eq!(result1, "You have item");
    assert_eq!(result2, "1 item");
    assert_eq!(result3, "5 items");
}

#[test]
fn test_pad_numbers() {
    let template = "ID: {id:pad(4,0)}";
    let result = umt_format_string(template, &json!({"id": 42}), None);
    assert_eq!(result, "ID: 0042");
}

#[test]
fn test_handle_invalid_formatter_syntax() {
    let template = "Value: {text:invalid-formatter-name!@#}";
    let result = umt_format_string(template, &json!({"text": "hello"}), None);
    assert_eq!(result, "Value: hello");
}

#[test]
fn test_handle_unknown_formatter_names() {
    let template = "Value: {text:nonExistentFormatter}";
    let result = umt_format_string(template, &json!({"text": "hello"}), None);
    assert_eq!(result, "Value: hello");
}

// Edge cases tests
#[test]
fn test_handle_empty_template() {
    let result = umt_format_string("", &json!({"name": "test"}), None);
    assert_eq!(result, "");
}

#[test]
fn test_handle_template_with_only_escaped_braces() {
    let result = umt_format_string("{{}} {{test}}", &json!({"test": "value"}), None);
    assert_eq!(result, "{} {test}");
}

#[test]
fn test_handle_complex_nesting_in_arrays() {
    let template = "Nested: {data[0].items[1].name}";
    let data = json!({
        "data": [
            {
                "items": [{"name": "first"}, {"name": "second"}]
            }
        ]
    });
    let result = umt_format_string(template, &data, None);
    assert_eq!(result, "Nested: second");
}

#[test]
fn test_work_with_single_object_argument() {
    let template = "{name}";
    let result = umt_format_string(template, &json!({"name": "test"}), None);
    assert_eq!(result, "test");
}

#[test]
fn test_work_with_exactly_two_arguments() {
    let template = "{0} and {1}";
    let result = umt_format_string_indexed(template, &["first", "second"]);
    assert_eq!(result, "first and second");
}
