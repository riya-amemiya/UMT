use serde_json::json;
use umt_rust::string::format_string::*;

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

#[test]
fn test_only_first_default_value_is_used() {
    assert_eq!(umt_format_string("{a|b|c}", &json!({}), None), "b");
    assert_eq!(
        umt_format_string("{a|x:upper}", &json!({}), None),
        "x:upper"
    );
    assert_eq!(umt_format_string("{a:upper|x}", &json!({}), None), "X");
    assert_eq!(umt_format_string("{a}", &json!({"a": null}), None), "{a}");
}

#[test]
fn test_path_is_not_trimmed_around_formatter_separator() {
    assert_eq!(
        umt_format_string("{name : upper}", &json!({"name": "x"}), None),
        "{name : upper}"
    );
    assert_eq!(
        umt_format_string("{ name:upper }", &json!({"name": "x"}), None),
        "X"
    );
    assert_eq!(
        umt_format_string("{a:unknown} {a:upper(}", &json!({"a": "x"}), None),
        "x x"
    );
}

#[test]
fn test_index_follows_javascript_number_rules() {
    assert_eq!(
        umt_format_string("{ 0 } {} { }", &json!(["a"]), None),
        "a {} a"
    );
    assert_eq!(
        umt_format_string("{0x1} {1e0} {1.5} {-1} {+1}", &json!(["a", "b"]), None),
        "b b {1.5} {-1} b"
    );
    assert_eq!(
        umt_format_string("{name} {Infinity} {-Infinity} {5}", &json!(["a"]), None),
        "{name} {Infinity} {-Infinity} {5}"
    );
}

#[test]
fn test_values_use_javascript_string_conversion() {
    assert_eq!(
        umt_format_string(
            "{0}",
            &json!([[1, [2, 3], null, {"a": 1}, "x", true]]),
            None
        ),
        "1,2,3,,[object Object],x,true"
    );
    assert_eq!(
        umt_format_string("{0} {1} {2} {3}", &json!([1.0, 1e21, 0.1, 1.5e-7]), None),
        "1 1e+21 0.1 1.5e-7"
    );
    assert_eq!(
        umt_format_string("{0} {1} {2} {3}", &json!([0.0, -0.5, 12, 1234.5]), None),
        "0 -0.5 12 1234.5"
    );
    assert_eq!(
        umt_format_string(
            "{a} {b} {c}",
            &json!({"a": {"b": 1}, "b": false, "c": ""}),
            None
        ),
        "[object Object] false "
    );
}

#[test]
fn test_array_property_paths() {
    assert_eq!(
        umt_format_string("{items.0} {items.01}", &json!({"items": ["A", "B"]}), None),
        "A {items.01}"
    );
}

#[test]
fn test_array_length_paths() {
    let data = json!({
        "items": ["A", "B", [1, 2, 3]],
        "user": {"tags": ["x"], "length": 9, "name": "bob"},
        "length": "top"
    });
    assert_eq!(
        umt_format_string("{items.length} {items[2].length}", &data, None),
        "3 3"
    );
    assert_eq!(
        umt_format_string("{user.tags.length:pad(3)}", &data, None),
        "001"
    );
    assert_eq!(
        umt_format_string("{user.length} {length}", &data, None),
        "9 top"
    );
    assert_eq!(
        umt_format_string("{user.name.length} {missing.length|none}", &data, None),
        "{user.name.length} none"
    );
    assert_eq!(
        umt_format_string("{items.length.x} {items.lengths}", &data, None),
        "{items.length.x} {items.lengths}"
    );
    assert_eq!(
        umt_format_string("{0.length} {length}", &json!([[1, 2]]), None),
        "{0.length} {length}"
    );
}

#[test]
fn test_built_in_number_currency_and_pad_formatters() {
    assert_eq!(
        umt_format_string("{0:number}", &json!([1234.5]), None),
        "1,234.5"
    );
    assert_eq!(
        umt_format_string("{0:number(en-US,2,2)}", &json!([1234.5]), None),
        "1,234.50"
    );
    assert_eq!(
        umt_format_string("{0:currency}", &json!([1234.5]), None),
        "$1,234.50"
    );
    assert_eq!(umt_format_string("{0:pad(3, )}", &json!([5]), None), "  5");
    assert_eq!(
        umt_format_string("{0:pad(4,ab)}", &json!([5]), None),
        "aba5"
    );
    assert_eq!(
        umt_format_string("{0:plural(item,items)}", &json!([" 1 "]), None),
        "item"
    );
}
