use super::core::umt_calculator_core;
use super::literal_expression::umt_literal_expression;
use crate::object::Value;
use std::collections::HashMap;

/// Calculator function that handles mathematical expressions and simple equations
/// Supports parentheses, signs, and currency conversion
/// Handles simple equations with single-character variables
///
/// # Arguments
///
/// * `expression` - Mathematical expression or equation
/// * `exchange` - Exchange rates for currency conversion
///
/// # Returns
///
/// Calculation result
pub fn umt_calculator(
    expression: &str,
    exchange: Option<&HashMap<String, Value>>,
) -> String {
    let clean_expression = expression.replace(char::is_whitespace, "");
    if clean_expression.contains('=') {
        umt_literal_expression(&clean_expression)
    } else {
        umt_calculator_core(&clean_expression, exchange)
    }
}
