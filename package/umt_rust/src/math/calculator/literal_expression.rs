use crate::math::{umt_division, umt_gcd};
use regex::Regex;

use super::core::umt_calculator_core;

/// Solves literal equations with variables.
///
/// # Arguments
///
/// * `x` - Equation string
///
/// # Returns
///
/// Solution result as a string.
///
/// # Examples
///
/// ```
/// use umt_rust::math::calculator::umt_literal_expression;
///
/// assert_eq!(umt_literal_expression("x+1=2"), "1");
/// assert_eq!(umt_literal_expression("2x=6"), "3");
/// assert_eq!(umt_literal_expression("3x+2=8"), "2");
/// ```
pub fn umt_literal_expression(x: &str) -> String {
    // Handle invalid equations like x=x
    let sides: Vec<&str> = x.split('=').collect();
    if sides.len() == 2 && sides[0] == sides[1] {
        return String::new();
    }

    // Store numerical and variable parts of the equation
    let mut numerical_part = String::new();
    let mut variable_part: Vec<String> = Vec::new();

    // Split by equals sign and identify numerical and variable parts
    for part in &sides {
        let letter_regex = Regex::new(r"[A-Za-z]+").unwrap();
        if letter_regex.is_match(part) {
            let split_regex = Regex::new(r"([-+]?\d*[A-Za-z]+)|([-+]?\d+)").unwrap();
            variable_part = split_regex
                .find_iter(part)
                .map(|m| m.as_str().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        } else {
            numerical_part = part.to_string();
        }
    }

    // Calculate the variable part (and invert signs for moving to other side of equation)
    if variable_part.len() > 1 {
        // Invert signs before calculating
        let inverted_part = variable_part[1]
            .replace('+', "PLUS")
            .replace('-', "MINUS")
            .replace("PLUS", "-")
            .replace("MINUS", "+");
        variable_part[1] = umt_calculator_core(&inverted_part, None);
    }

    // Calculate the numerical part
    if variable_part.len() > 1 {
        // Ensure proper sign handling when combining
        let sign = if variable_part[1].starts_with('-') {
            ""
        } else {
            "+"
        };
        numerical_part =
            umt_calculator_core(&format!("{}{}{}", numerical_part, sign, variable_part[1]), None);
    } else {
        numerical_part = umt_calculator_core(&numerical_part, None);
    }

    // Split the variable part again to separate coefficient and variable
    if !variable_part.is_empty() {
        let split_regex = Regex::new(r"(\d+)|([A-Za-z]+)").unwrap();
        let new_parts: Vec<String> = split_regex
            .find_iter(&variable_part[0])
            .map(|m| m.as_str().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        variable_part = new_parts;
    }

    // If there's no coefficient, return the numerical result
    if variable_part.is_empty() || variable_part[0].parse::<f64>().is_err() {
        return numerical_part;
    }

    let coefficient: i32 = variable_part[0].parse().unwrap_or(1);
    let numerical: i32 = numerical_part.parse().unwrap_or(0);

    // Simplify using greatest common divisor
    let common_gcd = umt_gcd(coefficient, numerical.abs());
    if common_gcd != 1 {
        let simplified_num = umt_division(numerical as f64, common_gcd as f64) as i32;
        let simplified_denom = umt_division(coefficient as f64, common_gcd as f64) as i32;

        numerical_part = format!("{}/{}", simplified_num, simplified_denom);

        let fraction_regex = Regex::new(r"(-?)\d+/1").unwrap();
        if fraction_regex.is_match(&numerical_part) {
            return numerical_part.replace("/1", "");
        }
    } else if coefficient != 1 {
        numerical_part = format!("{}/{}", numerical, coefficient);
    }

    numerical_part
}
