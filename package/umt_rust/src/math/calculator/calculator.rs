use std::collections::HashMap;

use super::core::umt_calculator_core;
use super::literal_expression::umt_literal_expression;

/// Calculator function that handles mathematical expressions and simple equations.
/// Supports parentheses, signs, and currency conversion.
/// Handles simple equations with single-character variables.
///
/// # Arguments
///
/// * `expression` - Mathematical expression or equation
/// * `exchange` - Optional exchange rates for currency conversion
///
/// # Returns
///
/// Calculation result as a string.
///
/// # Examples
///
/// ```
/// use umt_rust::math::calculator::umt_calculator;
///
/// assert_eq!(umt_calculator("1+2", None), "3");
/// assert_eq!(umt_calculator("(2+3)*4", None), "20");
/// assert_eq!(umt_calculator("x=5", None), "5");
/// ```
pub fn umt_calculator(
    expression: &str,
    exchange: Option<&HashMap<String, f64>>,
) -> String {
    let clean_expression: String = expression.chars().filter(|c| !c.is_whitespace()).collect();

    if clean_expression.contains('=') {
        umt_literal_expression(&clean_expression)
    } else {
        umt_calculator_core(&clean_expression, exchange)
    }
}

/// Calculator function using string-based exchange rates.
///
/// # Arguments
///
/// * `expression` - Mathematical expression or equation
/// * `exchange` - Optional exchange rates as string values
///
/// # Returns
///
/// Calculation result as a string.
pub fn umt_calculator_string_rates(
    expression: &str,
    exchange: Option<&HashMap<String, String>>,
) -> String {
    let rates: Option<HashMap<String, f64>> = exchange.map(|e| {
        e.iter()
            .filter_map(|(k, v)| v.parse::<f64>().ok().map(|rate| (k.clone(), rate)))
            .collect()
    });

    umt_calculator(expression, rates.as_ref())
}
