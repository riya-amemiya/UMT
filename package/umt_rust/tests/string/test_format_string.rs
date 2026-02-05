use umt_rust::obj;
use umt_rust::object::Value;
use umt_rust::string::{umt_format_string, umt_format_string_indexed};

// Original functionality tests (Indexed)
#[test]
fn test_replace_placeholders_with_specified_values() {
    let template = "Hello, {0}! It's {1} today.";
    let result = umt_format_string_indexed(template, &[Value::String("World".to_string()), Value::String("sunny".to_string())]);
    assert_eq!(result, "Hello, World! It's sunny today.");
}

#[test]
fn test_not_replace_placeholders_with_undefined_values() {
    let template = "Hello, {0}! How's {1}?";
    let result = umt_format_string_indexed(template, &[Value::String("World".to_string())]);
    assert_eq!(result, "Hello, World! How's {1}?");
}

#[test]
fn test_not_replace_placeholders_with_non_existent_indices() {
    let template = "Hello, {0}! {2} is not available.";
    let result = umt_format_string_indexed(template, &[Value::String("World".to_string()), Value::String("sunny".to_string())]);
    assert_eq!(result, "Hello, World! {2} is not available.");
}

#[test]
fn test_handle_empty_strings_as_values() {
    let template = "Start{0}Middle{1}End";
    let result = umt_format_string_indexed(template, &[Value::String("".to_string()), Value::String("".to_string())]);
    assert_eq!(result, "StartMiddleEnd");
}

#[test]
fn test_handle_repeated_placeholders() {
    let template = "{0} {1} {0} {1} {0}";
    let result = umt_format_string_indexed(template, &[Value::String("A".to_string()), Value::String("B".to_string())]);
    assert_eq!(result, "A B A B A");
}

// Named placeholders tests
#[test]
fn test_replace_named_placeholders_with_object_values() {
    let template = "Hello, {name}! You are {age} years old.";
    let data = obj!("name" => "Alice", "age" => 25);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Hello, Alice! You are 25 years old.");
}

#[test]
fn test_handle_missing_named_placeholders() {
    let template = "Hello, {name}! How's {mood}?";
    let data = obj!("name" => "Bob");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Hello, Bob! How's {mood}?");
}

// Nested object access tests
#[test]
fn test_access_nested_object_properties() {
    let template = "User: {user.name}, Email: {user.email}";
    let data = obj!(
        "user" => obj!(
            "name" => "Charlie",
            "email" => "charlie@example.com"
        )
    );
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "User: Charlie, Email: charlie@example.com");
}

#[test]
fn test_handle_deep_nesting() {
    let template = "Address: {user.address.city}, {user.address.country}";
    let data = obj!(
        "user" => obj!(
            "address" => obj!(
                "city" => "Tokyo",
                "country" => "Japan"
            )
        )
    );
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Address: Tokyo, Japan");
}

#[test]
fn test_handle_missing_nested_properties() {
    let template = "User: {user.name}, Phone: {user.phone}";
    let data = obj!(
        "user" => obj!("name" => "Dave")
    );
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "User: Dave, Phone: {user.phone}");
}

#[test]
fn test_handle_null_values_in_nested_access() {
    let template = "Value: {data.nested}";
    let data = obj!("data" => Value::Null);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Value: {data.nested}");
}

// Array access tests
#[test]
fn test_access_array_elements_by_positive_index() {
    let template = "First: {items[0]}, Second: {items[1]}";
    let data = obj!("items" => vec!["A", "B", "C"]);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "First: A, Second: B");
}

#[test]
fn test_access_array_elements_by_negative_index() {
    let template = "Last: {items[-1]}, Second last: {items[-2]}";
    let data = obj!("items" => vec!["A", "B", "C"]);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Last: C, Second last: B");
}

#[test]
fn test_handle_out_of_bounds_array_access() {
    let template = "Item: {items[10]}";
    let data = obj!("items" => vec!["A", "B"]);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Item: {items[10]}");
}

#[test]
fn test_handle_nested_array_access() {
    let template = "First user: {users[0].name}";
    let data = obj!(
        "users" => vec![
            obj!("name" => "Alice"),
            obj!("name" => "Bob")
        ]
    );
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "First user: Alice");
}

// Default values tests
#[test]
fn test_use_default_values_for_missing_placeholders() {
    let template = "Name: {name|Unknown}, Age: {age|N/A}";
    let data = obj!("age" => 25);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Name: Unknown, Age: 25");
}

#[test]
fn test_use_actual_values_when_available() {
    let template = "Name: {name|Unknown}, Age: {age|N/A}";
    let data = obj!("name" => "Eve", "age" => 30);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Name: Eve, Age: 30");
}

#[test]
fn test_handle_default_values_with_null() {
    let template = "Value: {value|Default}";
    let data = obj!("value" => Value::Null);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Value: Default");
}

// Escape sequences tests
#[test]
fn test_handle_escaped_braces() {
    let template = "Literal {{0}} and value {0}";
    let result = umt_format_string_indexed(template, &[Value::String("test".to_string())]);
    assert_eq!(result, "Literal {0} and value test");
}

#[test]
fn test_handle_multiple_escaped_braces() {
    let template = "{{name}} is not {name}, but {{age}} is not {age}";
    let data = obj!("name" => "Alice", "age" => 25);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "{name} is not Alice, but {age} is not 25");
}

// Formatters tests
#[test]
fn test_format_with_upper_formatter() {
    let template = "Name: {name:upper}";
    let data = obj!("name" => "alice");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Name: ALICE");
}

#[test]
fn test_format_with_lower_formatter() {
    let template = "Name: {name:lower}";
    let data = obj!("name" => "ALICE");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Name: alice");
}

#[test]
fn test_handle_plural_formatting() {
    let template1 = "You have {count:plural(item,items)}";
    let template2 = "{count} {count:plural(item,items)}";

    let data1 = obj!("count" => 1);
    let data5 = obj!("count" => 5);

    let result1 = umt_format_string(template1, &[data1.clone()]);
    let result2 = umt_format_string(template2, &[data1]);
    let result3 = umt_format_string(template2, &[data5]);

    assert_eq!(result1, "You have item");
    assert_eq!(result2, "1 item");
    assert_eq!(result3, "5 items");
}

#[test]
fn test_pad_numbers() {
    let template = "ID: {id:pad(4,0)}";
    let data = obj!("id" => 42);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "ID: 0042");
}

#[test]
fn test_handle_invalid_formatter_syntax() {
    let template = "Value: {text:invalid-formatter-name!@#}";
    let data = obj!("text" => "hello");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Value: hello");
}

#[test]
fn test_handle_unknown_formatter_names() {
    let template = "Value: {text:nonExistentFormatter}";
    let data = obj!("text" => "hello");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Value: hello");
}

// Edge cases tests
#[test]
fn test_handle_empty_template() {
    let data = obj!("name" => "test");
    let result = umt_format_string("", &[data]);
    assert_eq!(result, "");
}

#[test]
fn test_handle_template_with_only_escaped_braces() {
    let data = obj!("test" => "value");
    let result = umt_format_string("{{}} {{test}}", &[data]);
    assert_eq!(result, "{} {test}");
}

#[test]
fn test_handle_complex_nesting_in_arrays() {
    let template = "Nested: {data[0].items[1].name}";
    let data = obj!(
        "data" => vec![
            obj!(
                "items" => vec![
                    obj!("name" => "first"),
                    obj!("name" => "second")
                ]
            )
        ]
    );
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Nested: second");
}

#[test]
fn test_work_with_single_object_argument() {
    let template = "{name}";
    let data = obj!("name" => "test");
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "test");
}

#[test]
fn test_work_with_exactly_two_arguments() {
    let template = "{0} and {1}";
    let result = umt_format_string_indexed(template, &[Value::String("first".to_string()), Value::String("second".to_string())]);
    assert_eq!(result, "first and second");
}

#[test]
fn test_custom_formatter_options() {
    // Custom formatters are not yet supported in Rust Value enum
    // So this test is omitted/modified to check default behavior or commented out
    // let template = "Reversed: {name:reverse}";
    // ...
}

#[test]
fn test_format_array_value() {
    let template = "Items: {items}";
    let data = obj!("items" => vec!["a", "b", "c"]);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Items: a,b,c");
}

#[test]
fn test_format_array_with_numbers() {
    let template = "Numbers: {nums}";
    let data = obj!("nums" => vec![1, 2, 3]);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Numbers: 1,2,3");
}

#[test]
fn test_format_object_value() {
    let template = "Object: {obj}";
    let data = obj!("obj" => obj!("key" => "value"));
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Object: [object Object]");
}

#[test]
fn test_format_boolean_value() {
    let template = "Is active: {active}";
    let data = obj!("active" => true);
    let result = umt_format_string(template, &[data]);
    assert_eq!(result, "Is active: true");
}

#[test]
fn test_format_null_value_direct() {
    let template = "Value: {val}";
    let data = obj!("val" => Value::Null);
    let result = umt_format_string(template, &[data]);
    // Should keep the placeholder since value is null
    assert_eq!(result, "Value: {val}");
}
