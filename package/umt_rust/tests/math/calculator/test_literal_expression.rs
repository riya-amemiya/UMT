use umt_rust::math::calculator::umt_literal_expression;

#[test]
fn test_simple_equation_x_equals_value() {
    assert_eq!(umt_literal_expression("x=5"), "5");
}

#[test]
fn test_equation_with_addition() {
    assert_eq!(umt_literal_expression("x+1=2"), "1");
}

#[test]
fn test_equation_with_coefficient() {
    assert_eq!(umt_literal_expression("2x=6"), "3");
}

#[test]
fn test_equation_with_coefficient_and_constant() {
    assert_eq!(umt_literal_expression("3x+2=8"), "2");
}

#[test]
fn test_equation_x_equals_x_returns_empty() {
    assert_eq!(umt_literal_expression("x=x"), "");
}

#[test]
fn test_equation_with_subtraction() {
    assert_eq!(umt_literal_expression("x-1=4"), "5");
}

#[test]
fn test_equation_value_on_left_side() {
    assert_eq!(umt_literal_expression("5=x"), "5");
}

#[test]
fn test_equation_with_negative_result() {
    assert_eq!(umt_literal_expression("x+5=3"), "-2");
}

#[test]
fn test_equation_with_larger_coefficient() {
    assert_eq!(umt_literal_expression("4x=12"), "3");
}

#[test]
fn test_equation_needs_simplification() {
    assert_eq!(umt_literal_expression("6x=9"), "3/2");
}
