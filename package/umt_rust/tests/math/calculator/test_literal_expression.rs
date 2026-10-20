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

#[test]
fn test_basic_equations() {
    assert_eq!(umt_literal_expression("x=1"), "1");
    assert_eq!(umt_literal_expression("x=0"), "0");
    assert_eq!(umt_literal_expression("x=-1"), "-1");
}

#[test]
fn test_basic_equations_right_side() {
    assert_eq!(umt_literal_expression("1=x"), "1");
    assert_eq!(umt_literal_expression("0=x"), "0");
    assert_eq!(umt_literal_expression("-1=x"), "-1");
}

#[test]
fn test_mixed_numerical_and_variable_parts() {
    assert_eq!(umt_literal_expression("2x=4"), "2");
    assert_eq!(umt_literal_expression("1x=1"), "1");
    assert_eq!(umt_literal_expression("3x=-6"), "-2");
    assert_eq!(umt_literal_expression("2x+2=4"), "1");
    assert_eq!(umt_literal_expression("3x-2=-6"), "-4/3");
}

#[test]
fn test_mixed_numerical_and_variable_parts_right_side() {
    assert_eq!(umt_literal_expression("4=2x"), "2");
    assert_eq!(umt_literal_expression("1=1x"), "1");
    assert_eq!(umt_literal_expression("-6=3x"), "-2");
    assert_eq!(umt_literal_expression("4=2x+2"), "1");
    assert_eq!(umt_literal_expression("-6=3x-2"), "-4/3");
}

#[test]
fn test_simplify_when_gcd_greater_than_one() {
    assert_eq!(umt_literal_expression("2x=8"), "4");
}

#[test]
fn test_simplify_when_gcd_greater_than_one_right_side() {
    assert_eq!(umt_literal_expression("8=2x"), "4");
}

#[test]
fn test_fractions_that_cannot_be_simplified() {
    assert_eq!(umt_literal_expression("6x=4"), "2/3");
}

#[test]
fn test_coefficient_one_stays_integer() {
    assert_eq!(umt_literal_expression("1x=7"), "7");
}

#[test]
fn test_empty_and_no_equals() {
    assert_eq!(umt_literal_expression(""), "");
    assert_eq!(umt_literal_expression("42"), "42");
}
