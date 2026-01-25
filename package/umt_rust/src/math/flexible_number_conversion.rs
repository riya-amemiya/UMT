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
    let trimmed = value.trim().to_lowercase();

    // Handle empty string
    if trimmed.is_empty() {
        return 0.0;
    }

    // Handle infinity
    if trimmed == "infinity" || trimmed == "+infinity" {
        return f64::INFINITY;
    }
    if trimmed == "-infinity" {
        return f64::NEG_INFINITY;
    }

    // Handle special base notations (hex, octal, binary)
    if let Some(hex) = trimmed.strip_prefix("0x") {
        return match i64::from_str_radix(hex, 16) {
            Ok(n) => n as f64,
            Err(_) => f64::NAN,
        };
    }
    if let Some(oct) = trimmed.strip_prefix("0o") {
        return match i64::from_str_radix(oct, 8) {
            Ok(n) => n as f64,
            Err(_) => f64::NAN,
        };
    }
    if let Some(bin) = trimmed.strip_prefix("0b") {
        return match i64::from_str_radix(bin, 2) {
            Ok(n) => n as f64,
            Err(_) => f64::NAN,
        };
    }

    // Try to parse as float
    if let Ok(n) = trimmed.parse::<f64>() {
        return n;
    }

    // Try to extract number from the beginning of the string
    let mut num_str = String::new();
    let mut has_dot = false;
    let mut has_e = false;
    let mut chars = trimmed.chars().peekable();

    // Handle optional sign
    if let Some(&c) = chars.peek()
        && (c == '+' || c == '-')
    {
        num_str.push(c);
        chars.next();
    }

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            num_str.push(c);
            chars.next();
        } else if c == '.' && !has_dot && !has_e {
            has_dot = true;
            num_str.push(c);
            chars.next();
        } else if (c == 'e' || c == 'E') && !has_e && !num_str.is_empty() {
            has_e = true;
            num_str.push(c);
            chars.next();
            // Handle optional sign after e
            if let Some(&next) = chars.peek()
                && (next == '+' || next == '-')
            {
                num_str.push(next);
                chars.next();
            }
        } else {
            break;
        }
    }

    if !num_str.is_empty()
        && let Ok(n) = num_str.parse::<f64>()
    {
        return n;
    }

    f64::NAN
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_string() {
        assert_eq!(umt_flexible_number_conversion("123"), 123.0);
    }

    #[test]
    fn test_float_string() {
        assert_eq!(umt_flexible_number_conversion("78.9"), 78.9);
    }

    #[test]
    fn test_scientific_notation() {
        assert_eq!(umt_flexible_number_conversion("3.14e2"), 314.0);
    }

    #[test]
    fn test_hex() {
        assert_eq!(umt_flexible_number_conversion("0xFF"), 255.0);
    }

    #[test]
    fn test_octal() {
        assert_eq!(umt_flexible_number_conversion("0o10"), 8.0);
    }

    #[test]
    fn test_binary() {
        assert_eq!(umt_flexible_number_conversion("0b1010"), 10.0);
    }

    #[test]
    fn test_number_with_unit() {
        assert_eq!(umt_flexible_number_conversion("42px"), 42.0);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_flexible_number_conversion(""), 0.0);
    }

    #[test]
    fn test_infinity() {
        assert_eq!(umt_flexible_number_conversion("infinity"), f64::INFINITY);
        assert_eq!(
            umt_flexible_number_conversion("-infinity"),
            f64::NEG_INFINITY
        );
    }

    #[test]
    fn test_not_a_number() {
        assert!(umt_flexible_number_conversion("not a number").is_nan());
    }

    #[test]
    fn test_option_none() {
        assert_eq!(umt_flexible_number_conversion_opt(None), 0.0);
    }

    #[test]
    fn test_option_some() {
        assert_eq!(umt_flexible_number_conversion_opt(Some("42")), 42.0);
    }
}
