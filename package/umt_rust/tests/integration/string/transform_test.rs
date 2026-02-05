use umt_rust::string::{
    umt_camel_case, umt_format_string_indexed, umt_from_base64, umt_kebab_case, umt_to_base64,
    umt_trim_characters,
};
use umt_rust::object::Value;

#[test]
fn test_base64_roundtrip() {
    let original = "Hello, World!";
    let encoded = umt_to_base64(original);
    let decoded = umt_from_base64(&encoded).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn test_format_string() {
    let template = "{0}@{1}.{2}";
    let args_str = vec!["user", "example", "com"];
    let args: Vec<Value> = args_str.iter().map(|&s| Value::String(s.to_string())).collect();
    let generated = umt_format_string_indexed(template, &args);
    assert_eq!(generated, "user@example.com");
}

#[test]
fn test_trim_chars() {
    let messy = "!!!Hello World!!!";
    let cleaned = umt_trim_characters(messy, "!");
    assert_eq!(cleaned, "Hello World");
}

#[test]
fn test_case_transform() {
    let original = "user-profile-settings";
    let camel = umt_camel_case(original);
    let kebab = umt_kebab_case(&camel);
    assert_eq!(camel, "userProfileSettings");
    assert_eq!(kebab, original);
}
