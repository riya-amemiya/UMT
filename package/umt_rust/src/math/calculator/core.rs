use std::collections::HashMap;

use crate::math::{umt_addition, umt_division, umt_multiplication, umt_subtract};

use super::convert_currency::umt_convert_currency;
use regex::Regex;

/// Core calculator function that handles mathematical expressions.
/// Supports parentheses, signs, and currency conversion.
///
/// # Arguments
///
/// * `expression` - Mathematical expression string
/// * `currency_exchange` - Optional HashMap of currency exchange rates
///
/// # Returns
///
/// Calculation result as a string.
///
/// # Examples
///
/// ```
/// use umt_rust::math::calculator::umt_calculator_core;
///
/// assert_eq!(umt_calculator_core("1+2", None), "3");
/// assert_eq!(umt_calculator_core("2*3", None), "6");
/// assert_eq!(umt_calculator_core("10/2", None), "5");
/// assert_eq!(umt_calculator_core("(2+3)*4", None), "20");
/// ```
pub fn umt_calculator_core(
    expression: &str,
    currency_exchange: Option<&HashMap<String, f64>>,
) -> String {
    // Handle empty string
    if expression.is_empty() {
        return String::new();
    }

    let mut sanitized_expression = expression.to_string();

    // Handle signs
    sanitized_expression = sanitize_signs(&sanitized_expression);

    // Main calculation loop
    loop {
        // Currency conversion
        if let Some(rates) = currency_exchange {
            sanitized_expression = apply_currency_exchange(&sanitized_expression, rates);
        }

        // Handle parentheses
        if contains_parentheses(&sanitized_expression) {
            let temporary = resolve_parentheses(&sanitized_expression);
            if temporary == "NaN" {
                return sanitized_expression;
            }
            sanitized_expression = temporary;
        }
        // Handle multiplication and exponentiation
        else if contains_mul_exp(&sanitized_expression) {
            let temporary = resolve_mul_exp(&sanitized_expression);
            if temporary == "NaN" {
                return sanitized_expression;
            }
            sanitized_expression = temporary;
        }
        // Handle division
        else if contains_div(&sanitized_expression) {
            let temporary = resolve_div(&sanitized_expression);
            if temporary == "NaN" {
                return sanitized_expression;
            }
            sanitized_expression = temporary;
        }
        // Handle addition and subtraction
        else if contains_add_sub(&sanitized_expression) && !is_number_string(&sanitized_expression)
        {
            let temporary = resolve_add_sub(&sanitized_expression);
            if temporary == "NaN" {
                return sanitized_expression;
            }
            sanitized_expression = temporary;
        }
        // Return result if no more calculations needed
        else {
            if let Ok(number) = sanitized_expression.parse::<f64>() {
                if !number.is_nan() {
                    // Handle floating point precision issues
                    let rounded = (number * 1e10).round() / 1e10;
                    return rounded.to_string();
                }
            }
            return sanitized_expression;
        }
    }
}

fn sanitize_signs(expr: &str) -> String {
    expr.replace("--", "+")
        .replace("++", "+")
        .replace("+-", "+0-")
        .replace("-+", "+0-")
}

fn apply_currency_exchange(expr: &str, rates: &HashMap<String, f64>) -> String {
    let mut return_expr = expr.to_string();

    for currency_symbol in rates.keys() {
        if return_expr.contains(currency_symbol) {
            let pattern = format!(r"{}([0-9]+(?:\.[0-9]+)?)", regex::escape(currency_symbol));
            if let Ok(regex) = Regex::new(&pattern) {
                if let Some(captures) = regex.captures(&return_expr) {
                    let full_match = captures.get(0).unwrap().as_str();
                    let converted = umt_convert_currency(full_match, Some(rates));
                    return_expr = return_expr.replacen(full_match, &converted, 1);
                }
            }
        }
    }

    return_expr
}

fn contains_parentheses(expr: &str) -> bool {
    expr.contains('(') || expr.contains(')')
}

fn resolve_parentheses(expr: &str) -> String {
    // First, try to match parentheses with an operation inside
    let regex_with_op = Regex::new(r"\((-?\d+(?:\.\d+)?)([*+/\-^])(-?\d+(?:\.\d+)?)\)").unwrap();

    if let Some(captures) = regex_with_op.captures(expr) {
        let full_match = captures.get(0).unwrap().as_str();
        let inner = full_match.replace(['(', ')'], "");
        let result = umt_calculator_core(&inner, None);
        return expr.replacen(full_match, &result, 1);
    }

    // Then, try to match simple parentheses containing just a number like (5) or ((5))
    let regex_simple = Regex::new(r"\((-?\d+(?:\.\d+)?)\)").unwrap();

    if let Some(captures) = regex_simple.captures(expr) {
        let full_match = captures.get(0).unwrap().as_str();
        let inner = captures.get(1).unwrap().as_str();
        return expr.replacen(full_match, inner, 1);
    }

    "NaN".to_string()
}

fn contains_mul_exp(expr: &str) -> bool {
    expr.contains('^') || expr.contains('*')
}

fn contains_div(expr: &str) -> bool {
    expr.contains('/')
}

fn resolve_mul_exp(expr: &str) -> String {
    let regex = Regex::new(r"(.*?)(-?\d+(?:\.\d+)?)([*^])(-?\d+(?:\.\d+)?)$").unwrap();

    if let Some(captures) = regex.captures(expr) {
        let prefix = captures.get(1).map_or("", |m| m.as_str());
        let left: f64 = captures.get(2).unwrap().as_str().parse().unwrap_or(f64::NAN);
        let operator = captures.get(3).unwrap().as_str();
        let right: f64 = captures.get(4).unwrap().as_str().parse().unwrap_or(f64::NAN);

        let result = if operator == "^" {
            left.powf(right)
        } else {
            umt_multiplication(&[left, right])
        };

        return format!("{}{}", prefix, result);
    }

    "NaN".to_string()
}

fn resolve_div(expr: &str) -> String {
    let regex = Regex::new(r"(-?\d+(?:\.\d+)?)/(-?\d+(?:\.\d+)?)").unwrap();

    if let Some(captures) = regex.captures(expr) {
        let full_match = captures.get(0).unwrap().as_str();
        let left: f64 = captures.get(1).unwrap().as_str().parse().unwrap_or(f64::NAN);
        let right: f64 = captures.get(2).unwrap().as_str().parse().unwrap_or(f64::NAN);

        let result = umt_division(left, right);
        return expr.replacen(full_match, &result.to_string(), 1);
    }

    "NaN".to_string()
}

fn contains_add_sub(expr: &str) -> bool {
    expr.contains('+') || expr.contains('-')
}

fn resolve_add_sub(expr: &str) -> String {
    let regex = Regex::new(r"(-?\d+(?:\.\d+)?)(\+|-)(-?\d+(?:\.\d+)?)").unwrap();

    if let Some(captures) = regex.captures(expr) {
        let full_match = captures.get(0).unwrap().as_str();
        let left: f64 = captures.get(1).unwrap().as_str().parse().unwrap_or(f64::NAN);
        let operator = captures.get(2).unwrap().as_str();
        let right: f64 = captures.get(3).unwrap().as_str().parse().unwrap_or(f64::NAN);

        let result = if operator == "+" {
            umt_addition(&[left, right])
        } else {
            umt_subtract(&[left, right])
        };

        return expr.replacen(full_match, &result.to_string(), 1);
    }

    "NaN".to_string()
}

fn is_number_string(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}
