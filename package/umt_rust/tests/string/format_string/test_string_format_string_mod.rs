use umt_rust::string::format_string::*;
use serde_json::json;

#[test]
fn test_indexed_mode_basic() {
    let result = umt_format_string_indexed("Hello, {0}! It's {1} today.", &["World", "sunny"]);
    assert_eq!(result, "Hello, World! It's sunny today.");
}

#[test]
fn test_indexed_mode_undefined_values() {
    let result = umt_format_string_indexed("Hello, {0}! How's {1}?", &["World"]);
    assert_eq!(result, "Hello, World! How's {1}?");
}

#[test]
fn test_indexed_mode_repeated_placeholders() {
    let result = umt_format_string_indexed("{0} {1} {0} {1} {0}", &["A", "B"]);
    assert_eq!(result, "A B A B A");
}

#[test]
fn test_named_placeholders() {
    let result = umt_format_string(
        "Hello, {name}! You are {age} years old.",
        &json!({"name": "Alice", "age": 25}),
        None,
    );
    assert_eq!(result, "Hello, Alice! You are 25 years old.");
}

#[test]
fn test_missing_named_placeholders() {
    let result = umt_format_string(
        "Hello, {name}! How's {mood}?",
        &json!({"name": "Bob"}),
        None,
    );
    assert_eq!(result, "Hello, Bob! How's {mood}?");
}

#[test]
fn test_nested_object_access() {
    let result = umt_format_string(
        "User: {user.name}, Email: {user.email}",
        &json!({
            "user": { "name": "Charlie", "email": "charlie@example.com" }
        }),
        None,
    );
    assert_eq!(result, "User: Charlie, Email: charlie@example.com");
}

#[test]
fn test_deep_nesting() {
    let result = umt_format_string(
        "Address: {user.address.city}, {user.address.country}",
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
fn test_array_access_positive_index() {
    let result = umt_format_string(
        "First: {items[0]}, Second: {items[1]}",
        &json!({"items": ["A", "B", "C"]}),
        None,
    );
    assert_eq!(result, "First: A, Second: B");
}

#[test]
fn test_array_access_negative_index() {
    let result = umt_format_string(
        "Last: {items[-1]}, Second last: {items[-2]}",
        &json!({"items": ["A", "B", "C"]}),
        None,
    );
    assert_eq!(result, "Last: C, Second last: B");
}

#[test]
fn test_default_values() {
    let result = umt_format_string(
        "Name: {name|Unknown}, Age: {age|N/A}",
        &json!({"age": 25}),
        None,
    );
    assert_eq!(result, "Name: Unknown, Age: 25");
}

#[test]
fn test_escaped_braces() {
    let result = umt_format_string_indexed("Literal {{0}} and value {0}", &["test"]);
    assert_eq!(result, "Literal {0} and value test");
}

#[test]
fn test_upper_formatter() {
    let result = umt_format_string("Name: {name:upper}", &json!({"name": "alice"}), None);
    assert_eq!(result, "Name: ALICE");
}

#[test]
fn test_lower_formatter() {
    let result = umt_format_string("Name: {name:lower}", &json!({"name": "ALICE"}), None);
    assert_eq!(result, "Name: alice");
}

#[test]
fn test_plural_formatter() {
    let result1 = umt_format_string(
        "You have {count:plural(item,items)}",
        &json!({"count": 1}),
        None,
    );
    let result2 = umt_format_string(
        "{count} {count:plural(item,items)}",
        &json!({"count": 1}),
        None,
    );
    let result3 = umt_format_string(
        "{count} {count:plural(item,items)}",
        &json!({"count": 5}),
        None,
    );

    assert_eq!(result1, "You have item");
    assert_eq!(result2, "1 item");
    assert_eq!(result3, "5 items");
}

#[test]
fn test_pad_formatter() {
    let result = umt_format_string("ID: {id:pad(4,0)}", &json!({"id": 42}), None);
    assert_eq!(result, "ID: 0042");
}

#[test]
fn test_empty_template() {
    let result = umt_format_string("", &json!({"name": "test"}), None);
    assert_eq!(result, "");
}

#[test]
fn test_only_escaped_braces() {
    let result = umt_format_string("{{}} {{test}}", &json!({"test": "value"}), None);
    assert_eq!(result, "{} {test}");
}

#[test]
fn test_invalid_formatter_syntax() {
    let result = umt_format_string(
        "Value: {text:invalid-formatter-name!@#}",
        &json!({"text": "hello"}),
        None,
    );
    assert_eq!(result, "Value: hello");
}

#[test]
fn test_unknown_formatter() {
    let result = umt_format_string(
        "Value: {text:nonExistentFormatter}",
        &json!({"text": "hello"}),
        None,
    );
    assert_eq!(result, "Value: hello");
}
