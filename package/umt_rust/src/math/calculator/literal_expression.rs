<<<<<<< HEAD
use super::core::umt_calculator_core;
use crate::math::umt_gcd;
use regex::Regex;
use std::sync::OnceLock;

/// Solves literal equations with variables
///
/// # Arguments
///
/// * `x` - Equation string
///
/// # Returns
///
/// Solution result
pub fn umt_literal_expression(x: &str) -> String {
    // Handle invalid equations like x=x
    let sides: Vec<&str> = x.split('=').collect();
    if sides.len() == 2 && sides[0] == sides[1] {
        return "".to_string();
    }
    if sides.len() != 2 {
        return "".to_string(); // Invalid format
    }

    let mut numerical_part = "".to_string();
    let mut variable_part_tokens: Vec<String> = Vec::new();

    // Regex to identify if a part contains letters
    static RE_LETTERS: OnceLock<Regex> = OnceLock::new();
    let re_letters = RE_LETTERS.get_or_init(|| Regex::new(r"[A-Za-z]+").unwrap());

    static RE_TOKENS: OnceLock<Regex> = OnceLock::new();
    let re_tokens =
        RE_TOKENS.get_or_init(|| Regex::new(r"([-+]?\d*[A-Za-z]+)|([-+]?\d+)").unwrap());

    // Split by equals sign and identify numerical and variable parts
    for part in &sides {
        if re_letters.is_match(part) {
            // This side has the variable
            // We parse it into tokens.
            for cap in re_tokens.captures_iter(part) {
                if let Some(m) = cap.get(0).filter(|m| !m.as_str().is_empty()) {
                    variable_part_tokens.push(m.as_str().to_string());
                }
            }
        } else {
            numerical_part = part.to_string();
        }
    }

    // Check if we found variable part
    if variable_part_tokens.is_empty() {
        return "".to_string();
    }

    // Now variable_part_tokens has [variable_term, number_term, ...]
    // We iterate over all number terms (index 1 and beyond)
    if variable_part_tokens.len() > 1 {
        for token in variable_part_tokens.iter().skip(1) {
            let mut inverted_part = token.clone();
            // Invert sign
            if !inverted_part.starts_with('+') && !inverted_part.starts_with('-') {
                inverted_part = format!("-{}", inverted_part);
            } else {
                inverted_part = inverted_part
                    .replace('+', "plus")
                    .replace('-', "minus")
                    .replace("plus", "-")
                    .replace("minus", "+");
            }

            // Combine with numerical part
            // Ensure sign
            let sign = if inverted_part.starts_with('-') {
                ""
            } else {
                "+"
            };
            numerical_part = format!("{}{}{}", numerical_part, sign, inverted_part);
        }
        // Calculate the accumulated numerical part once
        numerical_part = umt_calculator_core(&numerical_part, None);
    } else {
        numerical_part = umt_calculator_core(&numerical_part, None);
    }

    // Split variable term into coeff and var
    // variablePart[0] is e.g. "2x" or "-x" or "x"
    let var_term = &variable_part_tokens[0];

    let coeff;

    // Check sign
    let (sign_mult, rest) = if let Some(stripped) = var_term.strip_prefix('-') {
        (-1.0, stripped)
    } else if let Some(stripped) = var_term.strip_prefix('+') {
        (1.0, stripped)
    } else {
        (1.0, &var_term[0..])
    };

    // Extract number part
    static RE_NUM: OnceLock<Regex> = OnceLock::new();
    let re_num = RE_NUM.get_or_init(|| Regex::new(r"^(\d+)").unwrap());

    if let Some(caps) = re_num.captures(rest) {
        let num_str = caps.get(1).unwrap().as_str();
        coeff = num_str.parse::<f64>().unwrap_or(1.0) * sign_mult;
    } else {
        coeff = sign_mult;
    }

    // Simplify using GCD
    let num_val = numerical_part.parse::<f64>().unwrap_or(f64::NAN);

    if num_val.is_nan() {
        return numerical_part;
    }

    // If coeff is 1, result is numerical_part.
    if (coeff - 1.0).abs() < 1e-10 {
        return numerical_part;
    }

    // Need to cast to integers for GCD check
    let coeff_int = coeff as i32;
    let num_int = num_val as i32;

    // Check if they are effectively integers
    if (coeff - coeff_int as f64).abs() < 1e-10 && (num_val - num_int as f64).abs() < 1e-10 {
        let common = umt_gcd(coeff_int.abs(), num_int.abs());
        if common != 1 {
            let new_num = num_int / common;
            let new_den = coeff_int / common;

            // Handle signs: if denominator is negative, move sign to numerator
            let (final_num, final_den) = if new_den < 0 {
                (-new_num, -new_den)
            } else {
                (new_num, new_den)
            };

            if final_den == 1 {
                return final_num.to_string();
            }
            return format!("{}/{}", final_num, final_den);
        }
    }

    // Fallback normal division if not simplifiable by GCD integer logic
    if (coeff - 1.0).abs() > 1e-10 {
        return format!("{}/{}", numerical_part, coeff);
    }

    numerical_part
}
||||||| 55b8153
=======
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
    let letter_regex = Regex::new(r"[A-Za-z]+").unwrap();
    let split_regex = Regex::new(r"([-+]?\d*[A-Za-z]+)|([-+]?\d+)").unwrap();
    for part in &sides {
        if letter_regex.is_match(part) {
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
        numerical_part = umt_calculator_core(
            &format!("{}{}{}", numerical_part, sign, variable_part[1]),
            None,
        );
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
>>>>>>> d916e13f83cf5d012a5cda2956ae6890fbe99788
