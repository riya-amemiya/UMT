use umt_rust::obj;
use umt_rust::object::Value;
use umt_rust::string::umt_format_string;
use std::collections::HashMap;

#[test]
fn test_indexed_mode_basic() {
    let template = "Hello, {0}!";
    let result = umt_format_string(template, &[Value::String("World".to_string())]);
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_indexed_mode_multiple() {
    let template = "{0} + {1} = {2}";
    let result = umt_format_string(
        template,
        &[Value::Int(1), Value::Int(2), Value::Int(3)]
    );
    assert_eq!(result, "1 + 2 = 3");
}

#[test]
fn test_named_mode_basic() {
    let template = "Hello, {name}!";
    let data = obj!("name" => "Alice");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Hello, Alice!");
}

#[test]
fn test_named_mode_nested() {
    let template = "{user.name} is {user.age} years old.";
    let data = obj!(
        "user" => obj!(
            "name" => "Bob",
            "age" => 25
        )
    );
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Bob is 25 years old.");
}

#[test]
fn test_named_mode_with_array_access() {
    let template = "First: {items[0]}, Last: {items[-1]}";
    let data = obj!(
        "items" => vec!["A", "B", "C"]
    );
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "First: A, Last: C");
}

#[test]
fn test_default_values() {
    let template = "Hello, {name|Stranger}!";
    let data = obj!("age" => 20); // name missing
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Hello, Stranger!");
}

#[test]
fn test_formatters_upper() {
    let template = "Hello, {name:upper}!";
    let data = obj!("name" => "alice");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Hello, ALICE!");
}

#[test]
fn test_formatters_pad() {
    let template = "ID: {id:pad(4,0)}";
    let data = obj!("id" => 5);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "ID: 0005");
}

#[test]
fn test_detect_mode_indexed_implicit() {
    // If multiple args are passed and first is not strictly an object-only data source (or options are missing)
    // Actually, detect_mode logic:
    // Case 3: Indexed mode if >2 args OR second arg is not options OR first arg is not object

    // "First", "Second" -> Indexed
    let result = umt_format_string("{0} {1}", &[
        Value::String("Hello".to_string()),
        Value::String("World".to_string())
    ]);
    assert_eq!(result, "Hello World");
}

#[test]
fn test_detect_mode_named_explicit_options() {
    let template = "{name}";
    let data = obj!("name" => "Alice");
    let options = obj!("formatters" => obj!()); // Dummy options

    // detect_mode(data, options, []) -> Named
    let result = umt_format_string(template, &[data, options]);
    assert_eq!(result, "Alice");
}

#[test]
fn test_mixed_types_display() {
    let template = "String: {s}, Int: {i}, Float: {f}, Bool: {b}, Null: {n}";
    let data = obj!(
        "s" => "text",
        "i" => 42,
        "f" => 3.14,
        "b" => true,
        "n" => Value::Null
    );
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "String: text, Int: 42, Float: 3.14, Bool: true, Null: null");
}

#[test]
fn test_escaped_braces() {
    let template = "Use {{braces}} for placeholders like {placeholder}.";
    let data = obj!("placeholder" => "value");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Use {braces} for placeholders like value.");
}
