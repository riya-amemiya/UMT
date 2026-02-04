use std::collections::HashMap;
use umt_rust::math::calculator::umt_calculator;
use umt_rust::object::Value;

#[test]
fn test_should_calculate_simple_addition() {
    assert_eq!(umt_calculator("1+2", None), "3");
}

#[test]
fn test_should_calculate_simple_subtraction() {
    assert_eq!(umt_calculator("5-3", None), "2");
}

#[test]
fn test_should_calculate_simple_multiplication() {
    assert_eq!(umt_calculator("2*3", None), "6");
}

#[test]
fn test_should_calculate_simple_division() {
    assert_eq!(umt_calculator("10/2", None), "5");
}

#[test]
fn test_should_handle_parentheses() {
    assert_eq!(umt_calculator("(2+3)*4", None), "20");
}

#[test]
fn test_should_handle_nested_parentheses() {
    assert_eq!(umt_calculator("((1+2)*3)", None), "9");
}

#[test]
fn test_should_handle_simple_equation() {
    assert_eq!(umt_calculator("x=5", None), "5");
}

#[test]
fn test_should_handle_whitespace() {
    assert_eq!(umt_calculator("1 + 2", None), "3");
    assert_eq!(umt_calculator("  3 * 4  ", None), "12");
}

#[test]
fn test_should_handle_negative_numbers() {
    assert_eq!(umt_calculator("-5+3", None), "-2");
}

#[test]
fn test_should_handle_currency_conversion() {
    let mut rates = HashMap::new();
    rates.insert("$".to_string(), Value::Float(100.0));
    assert_eq!(umt_calculator("$1", Some(&rates)), "100");
}

#[test]
fn test_should_handle_currency_with_operations() {
    let mut rates = HashMap::new();
    rates.insert("$".to_string(), Value::Float(100.0));
    assert_eq!(umt_calculator("$1+50", Some(&rates)), "150");
}

#[test]
fn test_should_handle_exponentiation() {
    assert_eq!(umt_calculator("2^3", None), "8");
}

#[test]
fn test_should_handle_decimal_numbers() {
    assert_eq!(umt_calculator("1.5+2.5", None), "4");
}

#[test]
fn test_should_return_empty_for_empty_input() {
    assert_eq!(umt_calculator("", None), "");
}

#[test]
fn test_should_handle_double_negative() {
    assert_eq!(umt_calculator("5--3", None), "8");
}

#[test]
fn test_should_handle_complex_expression() {
    assert_eq!(umt_calculator("2+3*4", None), "14");
}
