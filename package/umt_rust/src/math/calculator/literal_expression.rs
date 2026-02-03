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
                if let Some(m) = cap.get(0) {
                    if !m.as_str().is_empty() {
                        variable_part_tokens.push(m.as_str().to_string());
                    }
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
