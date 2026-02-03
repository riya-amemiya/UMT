use regex::Regex;
use crate::object::Value;
use std::collections::HashMap;
use crate::math::{umt_addition, umt_subtract, umt_multiplication, umt_division};
use crate::validate::umt_is_number_str;
use super::convert_currency::umt_convert_currency;
use std::sync::OnceLock;

pub fn umt_calculator_core(
    expression: &str,
    currency_exchange: Option<&HashMap<String, Value>>,
) -> String {
    if expression.is_empty() {
        return "".to_string();
    }

    let mut sanitized_expression = sanitize_signs(expression);

    // Main calculation loop
    loop {
        // Currency conversion
        if let Some(exchange) = currency_exchange {
            sanitized_expression = apply_currency_exchange(&sanitized_expression, exchange);
        }

        // Handle parentheses
        if contains_parentheses(&sanitized_expression) {
            let temporary = resolve_parentheses(&sanitized_expression);
            if temporary == "NaN" {
                return sanitized_expression;
            }
            sanitized_expression = temporary;
        }
        // Handle exponentiation (highest precedence)
        else if contains_exp(&sanitized_expression) {
            let temporary = resolve_exp(&sanitized_expression);
            if temporary == "NaN" {
                return sanitized_expression;
            }
            sanitized_expression = temporary;
        }
        // Handle multiplication and division (mixed, left-to-right)
        else if contains_mul_div(&sanitized_expression) {
            let temporary = resolve_mul_div(&sanitized_expression);
            if temporary == "NaN" {
                return sanitized_expression;
            }
            sanitized_expression = temporary;
        }
        // Handle addition and subtraction
        else if contains_add_sub(&sanitized_expression) && !umt_is_number_str(&sanitized_expression) {
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

fn apply_currency_exchange(expr: &str, rates: &HashMap<String, Value>) -> String {
    let mut return_expr = expr.to_string();
    for (currency_symbol, _) in rates {
        if return_expr.contains(currency_symbol) {
             let pattern = format!(r"{}(\d+)", regex::escape(currency_symbol));
             let re = Regex::new(&pattern).unwrap();

             if let Some(caps) = re.captures(&return_expr) {
                 let match_str = caps.get(0).unwrap().as_str();
                 let converted = umt_convert_currency(match_str, Some(rates));
                 return_expr = return_expr.replacen(match_str, &converted, 1);
             }
        }
    }
    return_expr
}

fn sanitize_signs(expr: &str) -> String {
    expr.replace("--", "+")
        .replace("++", "+")
        .replace("+-", "+0-")
        .replace("-+", "+0-")
}

fn contains_parentheses(expr: &str) -> bool {
    expr.contains('(') || expr.contains(')')
}

fn resolve_parentheses(expr: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"\((-?\d+(?:\.\d+)?)([*+/-])(-?\d+(?:\.\d+)?)\)").unwrap());

    if let Some(caps) = re.captures(expr) {
        let match_str = caps.get(0).unwrap().as_str();
        let inner_expr = match_str.replace(['(', ')'], "");
        let result = umt_calculator_core(&inner_expr, None);
        return expr.replace(match_str, &result);
    }
    "NaN".to_string()
}

fn contains_exp(expr: &str) -> bool {
    expr.contains('^')
}

fn resolve_exp(expr: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"(-?\d+(?:\.\d+)?)([\^])(-?\d+(?:\.\d+)?)").unwrap());

    if let Some(caps) = re.captures(expr) {
        let op1: f64 = caps.get(1).unwrap().as_str().parse().unwrap_or(f64::NAN);
        let op2: f64 = caps.get(3).unwrap().as_str().parse().unwrap_or(f64::NAN);

        let result = op1.powf(op2);

        let match_str = caps.get(0).unwrap().as_str();
        return expr.replacen(match_str, &result.to_string(), 1);
    }
    "NaN".to_string()
}

fn contains_mul_div(expr: &str) -> bool {
    expr.contains('*') || expr.contains('/')
}

fn resolve_mul_div(expr: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"(-?\d+(?:\.\d+)?)([*\/])(-?\d+(?:\.\d+)?)").unwrap());

    if let Some(caps) = re.captures(expr) {
        let op1: f64 = caps.get(1).unwrap().as_str().parse().unwrap_or(f64::NAN);
        let operator = caps.get(2).unwrap().as_str();
        let op2: f64 = caps.get(3).unwrap().as_str().parse().unwrap_or(f64::NAN);

        let result = if operator == "*" {
            umt_multiplication(&[op1, op2])
        } else {
            umt_division(op1, op2)
        };

        let match_str = caps.get(0).unwrap().as_str();
        return expr.replacen(match_str, &result.to_string(), 1);
    }
    "NaN".to_string()
}

// contains_div/resolve_div removed as separate steps, merged into mul_div

fn contains_add_sub(expr: &str) -> bool {
    expr.contains('+') || expr.contains('-')
}

fn resolve_add_sub(expr: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"(-?\d+(?:\.\d+)?)(\+|-)(-?\d+(?:\.\d+)?)").unwrap());

    if let Some(caps) = re.captures(expr) {
        let op1: f64 = caps.get(1).unwrap().as_str().parse().unwrap_or(f64::NAN);
        let operator = caps.get(2).unwrap().as_str();
        let op2: f64 = caps.get(3).unwrap().as_str().parse().unwrap_or(f64::NAN);

        let result = if operator == "+" {
            umt_addition(&[op1, op2])
        } else {
            umt_subtract(&[op1, op2])
        };

        let match_str = caps.get(0).unwrap().as_str();
        return expr.replacen(match_str, &result.to_string(), 1);
    }
    "NaN".to_string()
}
