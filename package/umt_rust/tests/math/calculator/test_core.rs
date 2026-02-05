use umt_rust::math::calculator::umt_calculator_core;

#[test]
fn test_simple_addition() {
    assert_eq!(umt_calculator_core("1+2", None), "3");
}

#[test]
fn test_simple_subtraction() {
    assert_eq!(umt_calculator_core("5-3", None), "2");
}

#[test]
fn test_simple_multiplication() {
    assert_eq!(umt_calculator_core("2*3", None), "6");
}

#[test]
fn test_simple_division() {
    assert_eq!(umt_calculator_core("10/2", None), "5");
}

#[test]
fn test_parentheses() {
    assert_eq!(umt_calculator_core("(2+3)*4", None), "20");
}

#[test]
fn test_exponentiation() {
    assert_eq!(umt_calculator_core("2^3", None), "8");
}

#[test]
fn test_empty_string() {
    assert_eq!(umt_calculator_core("", None), "");
}

#[test]
fn test_double_negative() {
    assert_eq!(umt_calculator_core("5--3", None), "8");
}

#[test]
fn test_double_positive() {
    assert_eq!(umt_calculator_core("5++3", None), "8");
}

#[test]
fn test_negative_number() {
    assert_eq!(umt_calculator_core("-5", None), "-5");
}

#[test]
fn test_decimal_result() {
    assert_eq!(umt_calculator_core("5/2", None), "2.5");
}

#[test]
fn test_nested_parentheses() {
    assert_eq!(umt_calculator_core("((2+3))", None), "5");
}

#[test]
fn test_mixed_operations() {
    assert_eq!(umt_calculator_core("2+3*4", None), "14");
}

#[test]
fn test_division_before_addition() {
    assert_eq!(umt_calculator_core("10/2+3", None), "8");
}

#[test]
fn test_negative_result() {
    assert_eq!(umt_calculator_core("3-5", None), "-2");
}

#[test]
fn test_floating_point_precision() {
    assert_eq!(umt_calculator_core("0.1+0.2", None), "0.3");
}
