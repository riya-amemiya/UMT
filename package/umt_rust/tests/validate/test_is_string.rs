//! Tests for is_string function

use umt_rust::validate::umt_is_string;

#[test]
fn test_is_string_returns_true_for_empty_string() {
    assert!(umt_is_string(""));
}

#[test]
fn test_is_string_returns_true_for_regular_strings() {
    assert!(umt_is_string("test"));
    assert!(umt_is_string("hello world"));
    assert!(umt_is_string("123"));
}

#[test]
fn test_is_string_returns_true_for_string_from_constructor() {
    let s = String::from("test");
    assert!(umt_is_string(&s));

    let s2 = 123.to_string();
    assert!(umt_is_string(&s2));
}

#[test]
fn test_is_string_returns_true_for_formatted_strings() {
    let value = "world";
    let formatted = format!("hello {}", value);
    assert!(umt_is_string(&formatted));
}

#[test]
fn test_is_string_with_special_characters() {
    assert!(umt_is_string("hello\nworld"));
    assert!(umt_is_string("tab\there"));
    assert!(umt_is_string("unicode: \u{1F600}"));
}

#[test]
fn test_is_string_with_whitespace() {
    assert!(umt_is_string(" "));
    assert!(umt_is_string("   "));
    assert!(umt_is_string("\t"));
    assert!(umt_is_string("\n"));
}

#[test]
fn test_is_string_with_numbers_as_strings() {
    assert!(umt_is_string("0"));
    assert!(umt_is_string("-1"));
    assert!(umt_is_string("3.14"));
}

use umt_rust::validate::*;

#[test]
fn test_is_string() {
    assert!(umt_is_string("test"));
    assert!(umt_is_string(&String::from("hello")));
    assert!(umt_is_string(""));
}
