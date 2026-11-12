/// Flexible function to convert various inputs to numbers whenever possible.
///
/// This function has the following features:
/// 1. Returns numbers as-is if already a number type
/// 2. Properly handles infinity (Infinity, -Infinity)
/// 3. Supports string representations of hexadecimal (0x), octal (0o), and binary (0b)
/// 4. Properly parses floating-point number strings
/// 5. Extracts numbers from strings that start with numbers when possible
/// 6. Returns NaN if none of the above conditions are met
///
/// # Arguments
///
/// * `value` - String value to convert.
///
/// # Returns
///
/// Converted number, or NaN if conversion is not possible.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_flexible_number_conversion;
///
/// assert_eq!(umt_flexible_number_conversion("123"), 123.0);
/// assert_eq!(umt_flexible_number_conversion("78.9"), 78.9);
/// assert_eq!(umt_flexible_number_conversion("0xFF"), 255.0);
/// assert_eq!(umt_flexible_number_conversion("42px"), 42.0);
/// assert!(umt_flexible_number_conversion("not a number").is_nan());
/// ```
pub fn umt_flexible_number_conversion(value: &str) -> f64 {
    // Handle empty string
    if value.is_empty() {
        return 0.0;
    }

    let trimmed = value.trim_matches(is_js_whitespace).to_lowercase();

    // Handle infinity
    if trimmed == "infinity" || trimmed == "+infinity" {
        return f64::INFINITY;
    }
    if trimmed == "-infinity" {
        return f64::NEG_INFINITY;
    }

    // Handle special base notations (hex, octal, binary)
    if let Some(hex) = trimmed.strip_prefix("0x") {
        return parse_radix_digits(hex, 16);
    }
    if let Some(oct) = trimmed.strip_prefix("0o") {
        return parse_radix_digits(oct, 8);
    }
    if let Some(bin) = trimmed.strip_prefix("0b") {
        return parse_radix_digits(bin, 2);
    }

    // Parse the longest leading decimal literal
    leading_decimal_literal(&trimmed)
        .parse::<f64>()
        .unwrap_or(f64::NAN)
}

fn is_js_whitespace(character: char) -> bool {
    matches!(
        character,
        '\u{9}'..='\u{d}'
            | ' '
            | '\u{a0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200a}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202f}'
            | '\u{205f}'
            | '\u{3000}'
            | '\u{feff}'
    )
}

fn parse_radix_digits(digits: &str, radix: u32) -> f64 {
    if digits.is_empty() || !digits.chars().all(|digit| digit.is_digit(radix)) {
        return f64::NAN;
    }
    match u128::from_str_radix(digits, radix) {
        Ok(n) => n as f64,
        Err(_) => digits.chars().fold(0.0, |total, digit| {
            total * f64::from(radix) + f64::from(digit.to_digit(radix).unwrap_or(0))
        }),
    }
}

fn leading_decimal_literal(value: &str) -> &str {
    let bytes = value.as_bytes();
    let count_digits = |start: usize| {
        bytes[start..]
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count()
    };

    let sign_length = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let integer_digits = count_digits(sign_length);
    let mut end = sign_length + integer_digits;

    let fraction_digits = if bytes.get(end) == Some(&b'.') {
        count_digits(end + 1)
    } else {
        0
    };
    if integer_digits == 0 && fraction_digits == 0 {
        return "";
    }
    if bytes.get(end) == Some(&b'.') {
        end += 1 + fraction_digits;
    }

    if bytes.get(end) == Some(&b'e') {
        let exponent_sign_length = usize::from(matches!(bytes.get(end + 1), Some(b'+' | b'-')));
        let exponent_digits = count_digits(end + 1 + exponent_sign_length);
        if exponent_digits > 0 {
            end += 1 + exponent_sign_length + exponent_digits;
        }
    }

    &value[..end]
}

/// Converts an Option value to a number.
///
/// Returns 0.0 for None, and the conversion result for Some.
pub fn umt_flexible_number_conversion_opt(value: Option<&str>) -> f64 {
    match value {
        None => 0.0,
        Some(s) => umt_flexible_number_conversion(s),
    }
}
